#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[990] = (p.p37 >= 0.0);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
        if s.b[990] {s.store_scalar(0, 1.0);}
        if (!s.b[990]) {s.store_scalar(0, (-1.0));}
        s.store_scalar(767, (8.8541878176e-12 * 11.8));s.b[991] = (p.p51 < 0.5);s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
        let (t9,) = {
    if s.b[991] {
        (0.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t9);s.b[992] = (p.p51 < 1.5);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        let (tc,) = {
    if ((!s.b[991]) && s.b[992]) {
        (1.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, tc);s.b[993] = (p.p51 < 2.5);s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });
        let (td,) = {
    if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
        (2.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, td);s.b[994] = (p.p51 < 4.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        let (t0,) = {
    if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
        (3.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t0);s.b[995] = (p.p51 < 7.0);s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
        (5.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t2);
        let (t3,) = {
    if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
        (9.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t3);s.store_scalar(2, 1000.0);s.store_scalar(3, 10.0);s.store_scalar(4, (1.0 / s.v[3]));s.store_scalar(350, (273.15 + p.p38));s.store_scalar(474, 0.0);s.b[996] = (p.p927 > 0.5);s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        let (t7,) = {
    if s.b[996] {
        (1.0,)
    } else {
        (s.v[474],)
    }
};
        s.store_scalar(474, t7);
        let (t8,) = {
    if (!s.b[996]) {
        (0.0,)
    } else {
        (s.v[474],)
    }
};
        s.store_scalar(474, t8);s.store_scalar(364, (273.15 + p.p823));s.store_scalar(367, (1.3806505e-23 / 1.6021918e-19));s.store_scalar(368, (s.v[367] * s.v[364]));s.store_scalar(369, (1.0 / s.v[368]));s.store_scalar(375, ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364])));s.store_scalar(378, (p.p834 + s.v[375]));s.store_scalar(379, (p.p835 + s.v[375]));s.store_scalar(380, (p.p836 + s.v[375]));s.store_scalar(408, (1.0 - p.p831));s.store_scalar(409, (1.0 - p.p832));s.store_scalar(410, (1.0 - p.p833));s.store_scalar(411, (1.0 / s.v[408]));s.store_scalar(412, (1.0 / s.v[409]));s.store_scalar(413, (1.0 / s.v[410]));s.store_scalar(423, (s.v[767] / p.p825));s.store_scalar(424, ((p.p843 * s.v[767]) / p.p826));s.store_scalar(425, ((p.p844 * s.v[767]) / p.p827));s.store_scalar(426, (1.0 / s.v[423]));s.store_scalar(427, (1.0 / s.v[424]));s.store_scalar(428, (1.0 / s.v[425]));s.store_scalar(429, (1.0 / p.p828));s.store_scalar(430, (1.0 / p.p829));s.store_scalar(431, (1.0 / p.p830));s.store_scalar(372, (1.772453850905516 * 0.29214664));s.store_scalar(373, (((((-5.0) * 0.29214664) + 6.0) - ((s.v[372]) as f64).powi(((-2.0) as i32))) / 3.0));s.store_scalar(374, ((1.0 - 0.29214664) - s.v[373]));s.store_scalar(444, (1.0 - (1.0 / p.p824)));s.store_scalar(445, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p863))));s.store_scalar(446, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p864))));s.store_scalar(447, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p.p865))));s.store_scalar(448, (1.0 / p.p860));s.store_scalar(449, (1.0 / p.p861));s.store_scalar(450, (1.0 / p.p862));s.store_scalar(451, (((-((s.v[445] * s.v[445]) * ((s.v[444]) as f64).powf((p.p863 - 1.0)))) * p.p863) * s.v[448]));s.store_scalar(452, (((-((s.v[446] * s.v[446]) * ((s.v[444]) as f64).powf((p.p864 - 1.0)))) * p.p864) * s.v[449]));s.store_scalar(453, (((-((s.v[447] * s.v[447]) * ((s.v[444]) as f64).powf((p.p865 - 1.0)))) * p.p865) * s.v[450]));s.b[997] = ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0));s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        let (ta,) = {
    if s.b[997] {
        (1.0,)
    } else {
        (s.v[473],)
    }
};
        s.store_scalar(473, ta);
        let (tb,) = {
    if (!s.b[997]) {
        (0.0,)
    } else {
        (s.v[473],)
    }
};
        s.store_scalar(473, tb);s.b[998] = (s.v[473] == 1.0);s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });
        if s.b[998] {s.store_scalar(457, (if ((p.p827 * p.p866) > 1e-18) { (p.p827 * p.p866) } else { 1e-18 }));}
        if s.b[998] {s.store_scalar(458, (if ((p.p830 * p.p867) > 0.05) { (p.p830 * p.p867) } else { 0.05 }));}
        if s.b[998] {s.store_scalar(459, (if ((if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) < 0.95) { (if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) } else { 0.95 }));}
        if s.b[998] {s.store_scalar(460, (p.p836 * p.p869));s.store_primal_offset(462, 460, s.v[375]);s.store_primal_sub_from_scalar(467, 1.0, 459);s.store_primal_div_from_scalar(468, 1.0, 467);}
        s.b[999] = (p.p44 == 0.0);s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if s.b[999] {s.store_scalar(505, p.p825);s.store_scalar(506, p.p826);s.store_scalar(507, p.p827);s.store_scalar(508, p.p828);s.store_scalar(509, p.p829);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[999] {s.store_scalar(510, p.p830);s.store_scalar(511, p.p831);s.store_scalar(512, p.p832);s.store_scalar(513, p.p833);s.store_scalar(514, p.p834);s.store_scalar(515, p.p835);s.store_scalar(516, p.p836);s.store_scalar(517, p.p837);s.store_scalar(518, p.p838);s.store_scalar(519, p.p839);s.store_scalar(522, p.p840);s.store_scalar(523, p.p841);s.store_scalar(524, p.p842);s.store_scalar(520, p.p843);s.store_scalar(521, p.p844);s.store_scalar(525, p.p845);s.store_scalar(526, p.p846);s.store_scalar(527, p.p847);s.store_scalar(528, p.p848);s.store_scalar(529, p.p849);s.store_scalar(530, p.p850);s.store_scalar(531, p.p851);s.store_scalar(532, p.p852);s.store_scalar(533, p.p853);s.store_scalar(534, p.p854);s.store_scalar(535, p.p855);s.store_scalar(536, p.p856);s.store_scalar(537, p.p857);s.store_scalar(538, p.p858);s.store_scalar(539, p.p859);s.store_scalar(540, p.p860);s.store_scalar(541, p.p861);s.store_scalar(542, p.p862);s.store_scalar(543, p.p863);s.store_scalar(544, p.p864);s.store_scalar(545, p.p865);s.store_scalar(552, p.p928);}
        let (t1,) = {
    if s.b[999] {
        (p.p929,)
    } else {
        (s.v[553],)
    }
};
        s.store_scalar(553, t1);
        if s.b[999] {s.store_scalar(636, p.p872);s.store_scalar(637, p.p873);s.store_scalar(638, p.p874);s.store_scalar(639, p.p875);s.store_scalar(546, p.p866);s.store_scalar(547, p.p867);s.store_scalar(548, p.p868);s.store_scalar(549, p.p869);s.store_scalar(550, p.p870);s.store_scalar(551, p.p871);}
        if (!s.b[999]) {s.store_scalar(505, p.p876);s.store_scalar(506, p.p877);s.store_scalar(507, p.p878);s.store_scalar(508, p.p879);s.store_scalar(509, p.p880);s.store_scalar(510, p.p881);s.store_scalar(511, p.p882);s.store_scalar(512, p.p883);s.store_scalar(513, p.p884);s.store_scalar(514, p.p885);s.store_scalar(515, p.p886);s.store_scalar(516, p.p887);s.store_scalar(517, p.p888);s.store_scalar(518, p.p889);s.store_scalar(519, p.p890);s.store_scalar(522, p.p891);s.store_scalar(523, p.p892);s.store_scalar(524, p.p893);s.store_scalar(520, p.p894);s.store_scalar(521, p.p895);s.store_scalar(525, p.p896);s.store_scalar(526, p.p897);s.store_scalar(527, p.p898);s.store_scalar(528, p.p899);s.store_scalar(529, p.p900);s.store_scalar(530, p.p901);s.store_scalar(531, p.p902);s.store_scalar(532, p.p903);s.store_scalar(533, p.p904);s.store_scalar(534, p.p905);s.store_scalar(535, p.p906);s.store_scalar(536, p.p907);s.store_scalar(537, p.p908);s.store_scalar(538, p.p909);s.store_scalar(539, p.p910);s.store_scalar(540, p.p911);s.store_scalar(541, p.p912);s.store_scalar(542, p.p913);s.store_scalar(543, p.p914);s.store_scalar(544, p.p915);s.store_scalar(545, p.p916);s.store_scalar(552, p.p930);}
        let (t4,) = {
    if (!s.b[999]) {
        (p.p931,)
    } else {
        (s.v[553],)
    }
};
        s.store_scalar(553, t4);
        if (!s.b[999]) {s.store_scalar(636, p.p923);s.store_scalar(637, p.p924);s.store_scalar(638, p.p925);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[999]) {s.store_scalar(639, p.p926);s.store_scalar(546, p.p917);s.store_scalar(547, p.p918);s.store_scalar(548, p.p919);s.store_scalar(549, p.p920);s.store_scalar(550, p.p921);s.store_scalar(551, p.p922);}
        s.store_primal_offset(554, 514, s.v[375]);s.store_primal_offset(555, 515, s.v[375]);s.store_primal_offset(556, 516, s.v[375]);s.store_primal_sub_from_scalar(575, 1.0, 511);s.store_primal_sub_from_scalar(576, 1.0, 512);s.store_primal_sub_from_scalar(577, 1.0, 513);s.store_primal_div_from_scalar(578, 1.0, 575);s.store_primal_div_from_scalar(579, 1.0, 576);s.store_primal_div_from_scalar(580, 1.0, 577);s.store_primal_div_from_scalar(590, s.v[767], 505);s.store_primal_div_scaled_inputs_indices(591, 520, s.v[767], 506, 1.0);s.store_primal_div_scaled_inputs_indices(592, 521, s.v[767], 507, 1.0);s.store_primal_div_from_scalar(593, 1.0, 590);s.store_primal_div_from_scalar(594, 1.0, 591);s.store_primal_div_from_scalar(595, 1.0, 592);s.store_primal_div_from_scalar(596, 1.0, 508);s.store_primal_div_from_scalar(597, 1.0, 509);s.store_primal_div_from_scalar(598, 1.0, 510);s.store_primal_div_from_scalar_sub_from_scalar_ad(611, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(543)));s.store_primal_div_from_scalar_sub_from_scalar_ad(612, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(544)));s.store_primal_div_from_scalar_sub_from_scalar_ad(613, 1.0, 1.0, A::pow_from_scalar(s.v[444], s.ad_value(545)));s.store_primal_div_from_scalar(614, 1.0, 540);s.store_primal_div_from_scalar(615, 1.0, 541);s.store_div_from_scalar(616, 1.0, 542);s.store_primal_mul_product3_mixed_iaai(617, 614, A::square(s.ad_value(611)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(543), (-1.0))), 543, -1.0);s.store_primal_mul_product3_mixed_iaai(618, 615, A::square(s.ad_value(612)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(544), (-1.0))), 544, -1.0);s.store_mul_product3_mixed_iaai(619, 616, A::square(s.ad_value(613)), A::pow_from_scalar(s.v[444], A::offset(s.ad_value(545), (-1.0))), 545, -1.0);s.b[1000] = ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0));s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });
        let (t5,) = {
    if s.b[1000] {
        (1.0,)
    } else {
        (s.v[635],)
    }
};
        s.store_scalar(635, t5);
        let (t6,) = {
    if (!s.b[1000]) {
        (0.0,)
    } else {
        (s.v[635],)
    }
};
        s.store_scalar(635, t6);s.b[1001] = (s.v[635] == 1.0);s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });
        if s.b[1001] {
            if ((s.v[507] * s.v[546]) > 1e-18) {
                s.store_primal_mul(620, 507, 546);
            } else {
                s.store_scalar(620, 1e-18);
            }
        }
        if s.b[1001] {
            if ((s.v[510] * s.v[547]) > 0.05) {
                s.store_primal_mul(621, 510, 547);
            } else {
                s.store_scalar(621, 0.05);
            }
        }
        if s.b[1001] {
            if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                if ((s.v[513] * s.v[548]) > 0.05) {
                    s.store_primal_mul(622, 513, 548);
                } else {
                    s.store_scalar(622, 0.05);
                }
            } else {
                s.store_scalar(622, 0.95);
            }
        }
        if s.b[1001] {s.store_primal_mul(623, 516, 549);s.store_primal_offset(625, 623, s.v[375]);s.store_primal_sub_from_scalar(630, 1.0, 622);s.store_primal_div_from_scalar(631, 1.0, 630);}
        s.store_scalar(878, 0.0);s.store_scalar(351, ((ctx_temp + p.p56) + p.p35));s.store_scalar(352, (s.v[351] / s.v[350]));s.store_scalar(353, (s.v[351] - s.v[350]));s.store_scalar(354, ((s.v[351] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(355, (1.0 / s.v[354]));s.store_scalar(356, s.v[351]);s.store_scalar(357, (s.v[356] * s.v[356]));s.store_scalar(358, (s.v[356] - s.v[350]));s.store_scalar(359, (s.v[350] / s.v[356]));s.store_scalar(360, ((s.v[359]) as f64).ln());s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(361, (1.0 / s.v[715]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));
        if (!(s.v[363] > 0.001)) {s.store_scalar(363, 0.001);}
        s.store_scalar(718, ((4.0 * 1.3806505e-23) * s.v[356]));s.store_scalar(365, (((ctx_temp + p.p56) + p.p35)).max((273.15 + (-250.0))));s.store_scalar(366, (s.v[365] / s.v[364]));s.store_scalar(370, (s.v[367] * s.v[365]));s.store_scalar(371, (1.0 / s.v[370]));s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));s.store_scalar(381, (p.p834 + s.v[376]));s.store_scalar(382, (p.p835 + s.v[376]));s.store_scalar(383, (p.p836 + s.v[376]));s.store_scalar(384, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp()));s.store_scalar(385, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp()));s.store_scalar(386, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp()));s.store_scalar(387, ((p.p837 * s.v[384]) * s.v[384]));s.store_scalar(388, ((p.p838 * s.v[385]) * s.v[385]));s.store_scalar(389, ((p.p839 * s.v[386]) * s.v[386]));s.store_scalar(390, ((p.p828 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln())));s.store_scalar(391, ((p.p829 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln())));s.store_scalar(392, ((p.p830 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln())));s.store_scalar(393, (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(394, (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(395, (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(405, (1.0 / s.v[393]));s.store_scalar(406, (1.0 / s.v[394]));s.store_scalar(407, (1.0 / s.v[395]));s.store_scalar(414, (p.p825 * (((p.p828 * s.v[405])) as f64).powf(p.p831)));s.store_scalar(415, (p.p826 * (((p.p829 * s.v[406])) as f64).powf(p.p832)));s.store_scalar(416, (p.p827 * (((p.p830 * s.v[407])) as f64).powf(p.p833)));s.store_scalar(417, ((s.v[414] * s.v[393]) * s.v[411]));s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));s.store_scalar(420, (2.0 * s.v[414]));s.store_scalar(421, (2.0 * s.v[415]));s.store_scalar(422, (2.0 * s.v[416]));s.store_scalar(432, ((0.5 * s.v[381])).max(s.v[370]));s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[370]));s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[370]));s.store_scalar(435, (s.v[432] * s.v[371]));s.store_scalar(436, (s.v[433] * s.v[371]));s.store_scalar(437, (s.v[434] * s.v[371]));s.store_scalar(438, (((((((32.0 * p.p848) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(439, (((((((32.0 * p.p849) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(440, (((((((32.0 * p.p850) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(441, (p.p854 * (1.0 + (p.p857 * (s.v[365] - s.v[364])))));s.store_scalar(442, (p.p855 * (1.0 + (p.p858 * (s.v[365] - s.v[364])))));s.store_scalar(443, (p.p856 * (1.0 + (p.p859 * (s.v[365] - s.v[364])))));
        if (!(s.v[441] > 0.0)) {s.store_scalar(441, 0.0);}
        if (!(s.v[442] > 0.0)) {s.store_scalar(442, 0.0);}
        if (!(s.v[443] > 0.0)) {s.store_scalar(443, 0.0);}
        s.b[1021] = (s.v[473] == 1.0);s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if s.b[1021] {s.store_primal_offset(461, 460, s.v[376]);s.store_primal_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(464, 458, s.v[366], 463, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(466, 1.0, 465);s.store_primal_mul_pow_mixed_iai(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), 459);s.store_primal_mul3_lhs(470, 469, 465, 468);s.store_primal_scale(471, 469, 2.0);}
        s.store_primal_offset(557, 514, s.v[376]);s.store_primal_offset(558, 515, s.v[376]);s.store_primal_offset(559, 516, s.v[376]);s.store_primal_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_mul3_lhs(563, 517, 560, 560);s.store_primal_mul3_lhs(564, 518, 561, 561);s.store_primal_mul3_lhs(565, 519, 562, 562);s.store_primal_sub_scaled_inputs_ln_rhs(566, 508, s.v[366], 560, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(567, 509, s.v[366], 561, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(568, 510, s.v[366], 562, (2.0 * s.v[370]));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs_mixed_ia(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(572, 1.0, 569);s.store_primal_div_from_scalar(573, 1.0, 570);s.store_primal_div_from_scalar(574, 1.0, 571);s.store_primal_mul_pow_mixed_iai(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), 511);s.store_primal_mul_pow_mixed_iai(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), 512);s.store_primal_mul_pow_mixed_iai(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), 513);s.store_primal_mul3_lhs(584, 581, 569, 578);s.store_primal_mul3_lhs(585, 582, 570, 579);s.store_primal_mul3_lhs(586, 583, 571, 580);s.store_primal_scale(587, 581, 2.0);s.store_primal_scale(588, 582, 2.0);s.store_primal_scale(589, 583, 2.0);s.store_primal_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);s.store_primal_scale(602, 599, s.v[371]);s.store_primal_scale(603, 600, s.v[371]);s.store_primal_scale(604, 601, s.v[371]);s.store_primal_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);s.store_primal_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);
        if (!(s.v[608] > 0.0)) {s.store_scalar(608, 0.0);}
        if (!(s.v[609] > 0.0)) {s.store_scalar(609, 0.0);}
        if (!(s.v[610] > 0.0)) {s.store_scalar(610, 0.0);}
        s.b[1022] = (s.v[635] == 1.0);s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if s.b[1022] {s.store_primal_offset(624, 623, s.v[376]);s.store_primal_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(627, 621, s.v[366], 626, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(629, 1.0, 628);s.store_primal_mul_pow_mixed_iai(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), 622);s.store_primal_mul3_lhs(633, 632, 628, 631);s.store_primal_scale(634, 632, 2.0);}
        s.store_scalar(5, 1.0);s.store_scalar(6, 1.0);s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);s.store_scalar(7, p.p0);s.store_scalar(8, p.p1);s.store_scalar(9, p.p2);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(10, p.p3);s.store_scalar(11, p.p4);s.store_scalar(12, p.p8);s.store_scalar(13, p.p11);s.store_scalar(646, p.p19);s.store_scalar(647, p.p20);s.store_scalar(648, p.p21);s.store_scalar(673, p.p22);s.store_scalar(674, p.p23);s.store_scalar(675, p.p24);s.store_scalar(649, p.p25);s.store_scalar(650, p.p26);s.store_scalar(676, p.p27);s.store_scalar(677, p.p28);s.store_scalar(14, p.p14);s.b[1023] = (p.p39 > 0.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if s.b[1023] {s.store_scalar(5, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));}
        if s.b[1023] {s.store_primal_floor_ad(5, A::offset(s.ad_value(5), 0.5));s.store_primal_div_from_scalar(6, 1.0, 5);}
        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_primal_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }
        s.store_scalar(15, p.p5);s.store_scalar(16, p.p6);s.store_scalar(17, p.p7);s.store_scalar(18, (if (p.p10 < 1.5) { 1.0 } else { 2.0 }));s.store_scalar(308, (1e-6 / s.v[7]));s.store_primal_div_from_scalar(309, 1e-6, 8);s.store_primal_offset_scaled(310, 309, ((p.p190) * ((p.p188 * (1.0 + (p.p189 * s.v[308]))))), (p.p188 * (1.0 + (p.p189 * s.v[308]))));s.store_primal_offset_scaled(311, 309, ((p.p194) * ((p.p192 * (1.0 + (p.p193 * s.v[308]))))), (p.p192 * (1.0 + (p.p193 * s.v[308]))));
        if (((s.v[7] + s.v[310]) - (2.0 * p.p191)) > 1e-9) {
            s.store_primal_offset(312, 310, ((s.v[7]) + ((-(2.0 * p.p191)))));
        } else {
            s.store_scalar(312, 1e-9);
        }
        if (((s.v[8] + s.v[311]) - (2.0 * p.p195)) > 1e-9) {
            s.store_primal_offset_add(313, 8, 311, (-(2.0 * p.p195)));
        } else {
            s.store_scalar(313, 1e-9);
        }
        s.store_primal_div_from_scalar(314, 1e-6, 312);s.store_primal_square(315, 314);s.store_primal_div_from_scalar(316, 1e-6, 313);s.store_primal_div_from_scalar(317, 1.0, 316);s.store_primal_mul(318, 314, 316);s.store_primal_div_from_scalar(319, 1.0, 318);
        if ((((s.v[7] + s.v[310]) - (2.0 * p.p191)) + p.p196) > 1e-9) {
            s.store_primal_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p.p191))))) + (p.p196)));
        } else {
            s.store_scalar(320, 1e-9);
        }
        if ((((s.v[8] + s.v[311]) - (2.0 * p.p195)) + p.p197) > 1e-9) {
            s.store_primal_offset_add(321, 8, 311, (((-(2.0 * p.p195))) + (p.p197)));
        } else {
            s.store_scalar(321, 1e-9);
        }
        s.store_primal_scale(322, 321, 1000000.0);
        if (((s.v[7] + s.v[310]) + p.p196) > 1e-9) {
            s.store_primal_offset(323, 310, ((s.v[7]) + (p.p196)));
        } else {
            s.store_scalar(323, 1e-9);
        }
        if (((s.v[8] + s.v[311]) + p.p197) > 1e-9) {
            s.store_primal_offset_add(324, 8, 311, p.p197);
        } else {
            s.store_scalar(324, 1e-9);
        }
        s.store_primal_scale(325, 323, 1000000.0);s.store_primal_scale(326, 324, 1000000.0);
        if ((s.v[7] + s.v[310]) > 1e-9) {
            s.store_primal_offset(327, 310, s.v[7]);
        } else {
            s.store_scalar(327, 1e-9);
        }
        if ((s.v[327] + p.p443) > 1e-9) {
            s.store_primal_offset(328, 327, p.p443);
        } else {
            s.store_scalar(328, 1e-9);
        }
        if ((s.v[8] + s.v[311]) > 1e-9) {
            s.store_primal_add(329, 8, 311);
        } else {
            s.store_scalar(329, 1e-9);
        }
        if ((s.v[13] - (0.5 * s.v[311])) > 1e-9) {
            s.store_primal_sub_from_scalar_scaled_input(330, s.v[13], 311, 0.5);
        } else {
            s.store_scalar(330, 1e-9);
        }
        s.store_scalar(44, p.p57);s.store_scalar(45, p.p58);s.store_scalar(46, p.p59);s.store_scalar(47, p.p60);s.store_scalar(48, p.p61);s.store_scalar(49, p.p62);s.store_scalar(50, p.p63);s.store_scalar(51, p.p64);s.store_scalar(52, p.p65);s.store_scalar(53, p.p66);s.store_scalar(54, p.p67);s.store_scalar(59, p.p68);s.store_scalar(60, p.p69);s.store_scalar(61, p.p70);s.store_scalar(62, p.p71);s.store_scalar(55, p.p72);s.store_scalar(56, p.p74);s.store_scalar(57, p.p73);s.store_scalar(58, p.p75);s.store_scalar(63, p.p79);s.store_scalar(64, p.p81);s.store_scalar(65, p.p80);s.store_scalar(66, p.p76);s.store_scalar(67, p.p78);s.store_scalar(68, p.p77);s.store_scalar(69, p.p82);s.store_scalar(70, p.p83);s.store_scalar(71, p.p84);s.store_scalar(72, p.p85);s.store_scalar(73, p.p86);s.store_scalar(74, p.p87);s.store_scalar(75, p.p88);s.store_scalar(76, p.p89);s.store_scalar(77, p.p90);s.store_scalar(78, p.p91);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(79, p.p92);s.store_scalar(80, p.p93);s.store_scalar(81, p.p94);s.store_scalar(82, p.p95);s.store_scalar(83, p.p96);s.store_scalar(84, p.p97);s.store_scalar(85, p.p98);s.store_scalar(86, p.p99);s.store_scalar(87, p.p100);s.store_scalar(88, p.p101);s.store_scalar(89, p.p102);s.store_scalar(90, p.p103);s.store_scalar(91, p.p104);s.store_scalar(92, p.p105);s.store_scalar(93, p.p106);s.store_scalar(94, p.p107);s.store_scalar(95, p.p108);s.store_scalar(96, p.p109);s.store_scalar(97, p.p110);s.store_scalar(98, p.p111);s.store_scalar(99, p.p112);s.store_scalar(100, p.p113);s.store_scalar(101, p.p114);s.store_scalar(102, p.p115);s.store_scalar(103, p.p116);s.store_scalar(104, p.p117);s.store_scalar(105, p.p118);s.store_scalar(106, p.p119);s.store_scalar(107, p.p120);s.store_scalar(108, p.p121);s.store_scalar(109, p.p120);s.b[1024] = param_given[122];s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if s.b[1024] {s.store_scalar(109, p.p122);}
        s.store_scalar(110, p.p121);s.b[1025] = param_given[123];s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if s.b[1025] {s.store_scalar(110, p.p123);}
        s.copy_ad(111, 109);s.b[1026] = param_given[124];s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if s.b[1026] {s.store_scalar(111, p.p124);}
        s.copy_ad(112, 110);s.b[1027] = param_given[125];s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if s.b[1027] {s.store_scalar(112, p.p125);}
        s.store_scalar(113, p.p126);s.store_scalar(114, p.p127);s.store_scalar(115, p.p128);s.store_scalar(116, p.p129);s.store_scalar(117, p.p130);s.store_scalar(118, p.p131);s.store_scalar(119, p.p132);s.store_scalar(120, p.p133);s.store_scalar(121, p.p134);s.store_scalar(122, p.p135);s.store_scalar(123, p.p136);s.store_scalar(124, p.p137);s.store_scalar(125, p.p99);s.b[1028] = param_given[138];s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if s.b[1028] {s.store_scalar(125, p.p138);}
        s.store_scalar(126, p.p104);s.b[1029] = param_given[139];s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if s.b[1029] {s.store_scalar(126, p.p139);}
        s.store_scalar(127, p.p140);s.store_scalar(128, p.p141);s.store_scalar(129, p.p142);s.store_scalar(130, p.p143);s.store_scalar(131, p.p144);s.store_scalar(132, p.p145);s.store_scalar(133, p.p146);s.store_scalar(134, p.p147);s.store_scalar(135, p.p148);s.store_scalar(136, p.p149);s.store_scalar(137, p.p150);s.store_scalar(138, p.p151);s.store_scalar(139, p.p152);s.store_scalar(140, p.p153);s.store_scalar(141, p.p154);s.store_scalar(142, p.p155);s.store_scalar(143, p.p156);s.store_scalar(144, p.p157);s.store_scalar(149, p.p162);s.store_scalar(150, p.p163);s.store_scalar(151, p.p164);s.store_scalar(152, p.p165);s.store_scalar(153, p.p166);s.store_scalar(154, p.p167);s.store_scalar(155, p.p168);s.store_scalar(156, p.p169);s.store_scalar(157, p.p170);s.store_scalar(158, p.p171);s.store_scalar(159, p.p172);s.store_scalar(160, p.p174);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(161, p.p173);s.store_scalar(167, p.p180);s.store_scalar(170, p.p181);s.store_scalar(171, p.p182);s.store_scalar(172, p.p184);s.store_scalar(173, p.p183);s.store_scalar(174, p.p185);s.store_scalar(175, p.p186);s.store_scalar(176, p.p187);s.b[1030] = (p.p39 > 0.0);s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if s.b[1030] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(44, A::powf(s.ad_value(314), p.p200), p.p199, 316, p.p201, 318, p.p202, p.p198);s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p.p204, 316, p.p205, 318, p.p206, p.p203);s.store_scalar(46, p.p207);s.store_scalar(47, p.p208);s.store_scalar(48, p.p209);}
        if s.b[1030] {
            s.store_primal_scale_ad(331, {
                if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p211, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p212), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p210);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(332, {
                if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p214, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p213);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(333, {
                if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p217, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p216);
        }
        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1031]) {s.store_scalar(334, 75000000000.0);s.store_primal_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));s.store_primal_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);s.store_primal_square(336, 336);}
        s.b[1032] = (s.v[312] >= s.v[333]);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {s.store_primal_add_mixed_ia(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));}
        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {s.store_primal_add_mixed_ia(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));}
        if s.b[1030] {s.store_primal_mul_sub_scaled_inputs_rhs_mixed_ai(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p218)), 1.0, 315, p.p219);s.store_primal_add_scaled_inputs3_offset_mixed_aii(50, A::powf(s.ad_value(314), p.p222), p.p221, 316, p.p223, 318, p.p224, p.p220);s.store_scalar(51, p.p225);s.store_scalar(52, p.p226);s.store_primal_add_scaled_inputs3_offset_mixed_aii(53, A::powf(s.ad_value(314), p.p229), p.p228, 316, p.p230, 318, p.p231, p.p227);}
        if s.b[1030] {
            s.store_primal_scale_ad(54, {
                if (1e-6 > (1.0 + (p.p233 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p233, 1.0)
                }
            }, p.p232);
        }
        if s.b[1030] {s.store_scalar(59, p.p234);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1030] {s.store_scalar(60, p.p235);s.store_scalar(61, p.p238);s.store_scalar(62, p.p239);s.store_primal_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p.p242), p.p241, p.p240), A::scale_offset(s.ad_value(316), p.p243, 1.0), A::scale_offset(s.ad_value(318), p.p244, 1.0));s.store_scalar(56, p.p246);s.store_scalar(57, p.p245);s.store_scalar(58, p.p247);s.store_primal_mul_powf_scale_offset_lhs(66, 314, 316, p.p249, (p.p250) * (p.p248), (1.0) * (p.p248));s.store_scalar(67, p.p252);s.store_scalar(68, p.p251);s.store_primal_mul_powf_scale_offset_lhs(63, 314, 316, p.p254, (p.p255) * (p.p253), (1.0) * (p.p253));s.store_scalar(64, p.p257);s.store_scalar(65, p.p256);s.store_primal_offset_scaled(337, 316, ((p.p260) * (p.p259)), p.p259);}
        if s.b[1030] {
            s.store_primal_scale_ad(338, {
                if ((1.0 + (p.p262 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p262, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p261);
        }
        if s.b[1030] {s.store_primal_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p264)))));}
        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p265, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p267), 1.0)), p.p266);s.store_primal_mul_div_scaled_inputs_mixed_iia(69, 340, 313, p.p258, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p.p269, 316, p.p270, 318, p.p271, p.p268);s.store_primal_offset_scaled(71, 316, ((p.p273) * (p.p272)), p.p272);s.store_scalar(72, p.p274);s.store_scalar(73, p.p275);s.store_scalar(74, p.p276);s.store_primal_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p.p279), p.p278, p.p277), A::scale_offset(s.ad_value(316), p.p280, 1.0), A::scale_offset(s.ad_value(318), p.p281, 1.0));s.store_scalar(76, p.p282);s.store_scalar(77, p.p283);s.store_scalar(78, p.p284);s.store_primal_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p.p286, 1.0), A::scale_offset(s.ad_value(316), p.p287, 1.0), A::scale_offset(s.ad_value(318), p.p288, 1.0), p.p285);s.store_scalar(80, p.p289);s.store_scalar(81, p.p290);s.store_primal_mul_scale_offset_rhs(82, 316, 316, ((p.p292) * (p.p291)), p.p291);s.store_scalar(83, p.p293);s.store_scalar(84, p.p294);s.store_scalar(85, p.p295);s.store_primal_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p297, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p298)), p.p296), A::scale_offset(s.ad_value(316), p.p299, 1.0), A::scale_offset(s.ad_value(318), p.p300, 1.0));s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p.p302, 316, p.p303, 318, p.p304, p.p301);s.store_scalar(88, p.p305);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_scalar(89, p.p306);s.store_scalar(90, p.p307);s.store_primal_div_from_scalar_offset_scaled_input(91, p.p308, 314, p.p309, 1.0);s.store_primal_mul_powf_scale_offset_lhs(92, 314, 316, p.p311, (p.p312) * (p.p310), (1.0) * (p.p310));s.store_primal_powf(341, 314, p.p314);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(93, 341, A::scale_offset(s.ad_value(316), p.p316, 1.0), p.p313, A::mul_scaled_lhs(s.ad_value(314), p.p315, s.ad_value(341)), 1.0, 1.0);s.store_primal_powf(341, 314, p.p318);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(94, 341, A::scale_offset(s.ad_value(316), p.p320, 1.0), p.p317, A::mul_scaled_lhs(s.ad_value(314), p.p319, s.ad_value(341)), 1.0, 1.0);s.store_scalar(95, p.p321);s.store_primal_scaled_mul_scale_offset_inputs(96, 314, p.p323, 1.0, 316, p.p324, 1.0, p.p322);s.store_scalar(97, p.p325);s.store_scalar(98, p.p326);s.store_primal_scaled_mul_scale_offset_inputs(99, 314, p.p328, 1.0, 316, p.p329, 1.0, p.p327);s.store_primal_scaled_mul_scale_offset_inputs(100, 314, p.p331, 1.0, 316, p.p332, 1.0, p.p330);s.store_scalar(101, p.p333);s.store_scalar(102, p.p334);s.store_primal_div_from_scalar(103, p.p335, 318);s.store_primal_div_from_scalar_scaled_input(104, (p.p336 * p.p236), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(105, (p.p337 * p.p237), 316, 1e-6);s.store_scalar(106, p.p338);s.store_scalar(107, p.p339);s.store_scalar(108, p.p340);s.store_scalar(109, p.p339);}
        s.b[1033] = param_given[341];s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1033]) {s.store_scalar(109, p.p341);}
        if s.b[1030] {s.store_scalar(110, p.p340);}
        s.b[1034] = param_given[342];s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1034]) {s.store_scalar(110, p.p342);}
        if s.b[1030] {s.copy_ad(111, 109);}
        s.b[1035] = param_given[343];s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1035]) {s.store_scalar(111, p.p343);}
        if s.b[1030] {s.copy_ad(112, 110);}
        s.b[1036] = param_given[344];s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1036]) {s.store_scalar(112, p.p344);}
        if s.b[1030] {s.store_scalar(113, p.p345);s.store_primal_div_from_scalar_scaled_input(114, (p.p346 * p.p236), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(115, (p.p347 * p.p237), 316, 1e-6);s.store_scalar(116, p.p348);s.store_scalar(117, p.p349);s.store_scalar(118, p.p350);s.store_scalar(119, p.p351);s.store_scalar(120, p.p352);s.store_scalar(121, p.p353);s.store_primal_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p.p209) * 1.0 / (p.p208)));s.store_primal_scale(129, 321, ((8.8541878176e-12 * p.p209) * (p.p236 * 1.0 / (p.p234))));s.store_primal_scale(130, 321, ((8.8541878176e-12 * p.p209) * (p.p237 * 1.0 / (p.p235))));s.store_primal_add_scaled_inputs3_offset_mixed_aii(123, A::powf(s.ad_value(314), p.p356), p.p355, 316, p.p357, 318, p.p358, p.p354);s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p.p360, 316, p.p361, 318, p.p362, p.p359);s.store_scalar(36, p.p296);}
        s.b[1037] = param_given[363];s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1037]) {s.store_scalar(36, p.p363);}
        if s.b[1030] {s.store_scalar(37, p.p297);}
        s.b[1038] = param_given[364];s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1038]) {s.store_scalar(37, p.p364);}
        if s.b[1030] {s.store_scalar(38, p.p298);}
        s.b[1039] = param_given[365];s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1039]) {s.store_scalar(38, p.p365);}
        if s.b[1030] {s.store_scalar(39, p.p299);}
        s.b[1040] = param_given[366];s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1040]) {s.store_scalar(39, p.p366);}
        if s.b[1030] {s.store_scalar(40, p.p300);}
        s.b[1041] = param_given[367];s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1041]) {s.store_scalar(40, p.p367);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_primal_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));s.store_scalar(41, p.p308);}
        s.b[1042] = param_given[368];s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1042]) {s.store_scalar(41, p.p368);}
        if s.b[1030] {s.store_scalar(42, p.p309);}
        s.b[1043] = param_given[369];s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1043]) {s.store_scalar(42, p.p369);}
        if s.b[1030] {s.store_primal_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);s.store_primal_mul_powf_scale_offset_lhs(127, 314, 316, p.p371, (p.p372) * (p.p370), (1.0) * (p.p370));s.store_primal_powf(341, 314, p.p374);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(128, 341, A::scale_offset(s.ad_value(316), p.p376, 1.0), p.p373, A::mul_scaled_lhs(s.ad_value(314), p.p375, s.ad_value(341)), 1.0, 1.0);s.store_scalar(131, p.p377);s.store_scalar(132, p.p378);s.store_scalar(133, p.p379);s.store_primal_scale(134, 325, p.p380);s.store_primal_scale(135, 322, p.p381);s.store_primal_scale(136, 322, p.p382);s.store_scalar(137, p.p383);s.store_scalar(138, p.p384);s.store_scalar(139, p.p385);s.store_scalar(140, p.p386);s.store_primal_scale(141, 326, p.p387);s.store_primal_scale(142, 326, p.p388);s.store_primal_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p.p395), s.ad_value(312)));s.store_scalar(143, p.p389);s.store_primal_mul_product3_indices(144, 316, 69, 69, 316, p.p390);s.store_primal_offset_scaled(344, 313, p.p398, (2.0 * p.p397));s.store_scalar(149, p.p399);s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p.p401, 316, p.p402, 318, p.p403, p.p400);s.store_primal_add_scaled_inputs3_offset_mixed_aii(151, A::powf(s.ad_value(314), p.p406), p.p405, 316, p.p407, 318, p.p408, p.p404);s.store_primal_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p.p411), p.p410, 1.0), A::scale_offset(s.ad_value(316), p.p412, 1.0), A::scale_offset(s.ad_value(318), p.p413, 1.0), p.p409);s.store_primal_offset_scaled_ad(153, A::powf(s.ad_value(314), p.p416), p.p415, p.p414);s.store_primal_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p417 * p.p418), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p418)))), 1.0);}
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_mul_div_scaled_inputs_mixed_aia(154, A::scale_offset(s.ad_value(316), p.p419, 1.0), 344, p.p258, A::mul(s.ad_value(347), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p.p421, 316, p.p422, 318, p.p423, p.p420);s.store_primal_mul_powf_scale_offset_lhs(156, 314, 316, p.p425, (p.p426) * (p.p424), (1.0) * (p.p424));s.store_scalar(157, p.p427);s.store_scalar(158, p.p428);s.store_primal_mul_powf_scale_offset_lhs(159, 314, 316, p.p430, (p.p431) * (p.p429), (1.0) * (p.p429));s.store_scalar(160, p.p433);s.store_scalar(161, p.p432);s.store_primal_add_scaled_inputs3_offset_indices(348, 314, p.p815, 316, p.p816, 318, p.p817, p.p814);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_primal_add_scaled_inputs3_offset_indices(349, 314, p.p819, 316, p.p820, 318, p.p821, p.p818);s.store_primal_add_scaled_inputs3_mixed_aai(167, A::div_scaled_inputs2(s.ad_value(329), ((0.3333333333333333 * 1.0 / (s.v[18])) * p.p442), s.ad_value(330), p.p442, s.ad_value(328), s.v[18]), 1.0, A::div_from_scalar((p.p440 + p.p441), A::mul(s.ad_value(329), s.ad_value(327))), 1.0, 5, p.p439);}
        if s.b[1030] {s.store_scalar(168, (if (p.p444 > 0.0) { p.p444 } else { 0.0 }));}
        if s.b[1030] {s.store_scalar(169, (if (p.p445 > 0.0) { p.p445 } else { 0.0 }));}
        s.b[1044] = (p.p44 == 0.0);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1044]) {s.copy_ad(169, 168);}
        if s.b[1030] {s.store_primal_scaled_mul(170, 5, 168, p.p12);s.store_primal_scaled_mul(171, 5, 169, p.p13);s.store_primal_scale(172, 5, p.p447);s.store_primal_scale(173, 5, p.p446);s.store_primal_scale(174, 5, p.p448);s.store_primal_scale(175, 5, p.p449);s.store_scalar(176, p.p450);}
        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1045]) {s.store_primal_add_scaled_inputs3_offset_indices(44, 314, p.p452, 316, p.p453, 318, p.p454, p.p451);}
        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1046]) {s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p.p456, 316, p.p457, 318, p.p458, p.p455);}
        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1047]) {s.store_primal_add_scaled_inputs3_offset_indices(49, 314, p.p460, 316, p.p461, 318, p.p462, p.p459);}
        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1048]) {s.store_primal_add_scaled_inputs3_offset_indices(50, 314, p.p464, 316, p.p465, 318, p.p466, p.p463);}
        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1049]) {s.store_primal_add_scaled_inputs3_offset_indices(51, 314, p.p468, 316, p.p469, 318, p.p470, p.p467);}
        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1050]) {s.store_primal_add_scaled_inputs3_offset_indices(53, 314, p.p472, 316, p.p473, 318, p.p474, p.p471);}
        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1051]) {s.store_primal_add_scaled_inputs3_offset_indices(54, 314, p.p476, 316, p.p477, 318, p.p478, p.p475);}
        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1052]) {s.store_primal_add_scaled_inputs3_offset_indices(61, 314, p.p480, 316, p.p481, 318, p.p482, p.p479);}
        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1053]) {s.store_primal_add_scaled_inputs3_offset_indices(62, 314, p.p484, 316, p.p485, 318, p.p486, p.p483);}
        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1054]) {s.store_primal_add_scaled_inputs3_offset_indices(55, 314, p.p488, 316, p.p489, 318, p.p490, p.p487);}
        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1055]) {s.store_primal_add_scaled_inputs3_offset_indices(56, 314, p.p496, 316, p.p497, 318, p.p498, p.p495);}
        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1056]) {s.store_primal_add_scaled_inputs3_offset_indices(57, 314, p.p492, 316, p.p493, 318, p.p494, p.p491);}
        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1057]) {s.store_primal_add_scaled_inputs3_offset_indices(58, 314, p.p500, 316, p.p501, 318, p.p502, p.p499);}
        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1058]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(66, 315, 314, p.p504, 316, p.p505, 318, p.p506, p.p503);}
        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1059]) {s.store_primal_add_scaled_inputs3_offset_indices(67, 314, p.p512, 316, p.p513, 318, p.p514, p.p511);}
        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1060]) {s.store_primal_add_scaled_inputs3_offset_indices(68, 314, p.p508, 316, p.p509, 318, p.p510, p.p507);}
        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1061]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(63, 315, 314, p.p516, 316, p.p517, 318, p.p518, p.p515);}
        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1062]) {s.store_primal_add_scaled_inputs3_offset_indices(64, 314, p.p524, 316, p.p525, 318, p.p526, p.p523);}
        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1063]) {s.store_primal_add_scaled_inputs3_offset_indices(65, 314, p.p520, 316, p.p521, 318, p.p522, p.p519);}
        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1064]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(69, A::add_scaled_inputs3_offset(s.ad_value(314), p.p528, s.ad_value(316), p.p529, s.ad_value(318), p.p530, p.p527), 313, 1.0, 312, 1.0);}
        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1065]) {s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p.p532, 316, p.p533, 318, p.p534, p.p531);}
        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1066]) {s.store_primal_add_scaled_inputs3_offset_indices(71, 314, p.p536, 316, p.p537, 318, p.p538, p.p535);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1067]) {s.store_primal_add_scaled_inputs3_offset_indices(73, 314, p.p540, 316, p.p541, 318, p.p542, p.p539);}
        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1068]) {s.store_primal_add_scaled_inputs3_offset_indices(75, 314, p.p544, 316, p.p545, 318, p.p546, p.p543);}
        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1069]) {s.store_primal_add_scaled_inputs3_offset_indices(77, 314, p.p548, 316, p.p549, 318, p.p550, p.p547);}
        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1070]) {s.store_primal_add_scaled_inputs3_offset_indices(79, 314, p.p552, 316, p.p553, 318, p.p554, p.p551);}
        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1071]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(82, 316, 314, p.p556, 316, p.p557, 318, p.p558, p.p555);}
        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1072]) {s.store_primal_add_scaled_inputs3_offset_indices(83, 314, p.p560, 316, p.p561, 318, p.p562, p.p559);}
        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1073]) {s.store_primal_add_scaled_inputs3_offset_indices(84, 314, p.p564, 316, p.p565, 318, p.p566, p.p563);}
        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1074]) {s.store_primal_add_scaled_inputs3_offset_indices(85, 314, p.p568, 316, p.p569, 318, p.p570, p.p567);}
        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1075]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(86, 314, 314, p.p572, 316, p.p573, 318, p.p574, p.p571);}
        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1076]) {s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p.p576, 316, p.p577, 318, p.p578, p.p575);}
        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1077]) {s.store_primal_add_scaled_inputs3_offset_indices(88, 314, p.p580, 316, p.p581, 318, p.p582, p.p579);}
        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1078]) {s.store_primal_add_scaled_inputs3_offset_indices(89, 314, p.p584, 316, p.p585, 318, p.p586, p.p583);}
        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1079]) {s.store_primal_add_scaled_inputs3_offset_indices(91, 314, p.p588, 316, p.p589, 318, p.p590, p.p587);}
        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1080]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(92, 314, 314, p.p592, 316, p.p593, 318, p.p594, p.p591);}
        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1081]) {s.store_primal_add_scaled_inputs3_offset_indices(93, 314, p.p596, 316, p.p597, 318, p.p598, p.p595);}
        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1082]) {s.store_primal_add_scaled_inputs3_offset_indices(94, 314, p.p600, 316, p.p601, 318, p.p602, p.p599);}
        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1083]) {s.store_primal_add_scaled_inputs3_offset_indices(96, 314, p.p604, 316, p.p605, 318, p.p606, p.p603);}
        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1084]) {s.store_primal_add_scaled_inputs3_offset_indices(98, 314, p.p608, 316, p.p609, 318, p.p610, p.p607);}
        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1085]) {s.store_primal_add_scaled_inputs3_offset_indices(99, 314, p.p612, 316, p.p613, 318, p.p614, p.p611);}
        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1086]) {s.store_primal_add_scaled_inputs3_offset_indices(100, 314, p.p616, 316, p.p617, 318, p.p618, p.p615);}
        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1087]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(103, 319, 314, p.p620, 316, p.p621, 318, p.p622, p.p619);}
        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1088]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(104, 317, 314, p.p624, 316, p.p625, 318, p.p626, p.p623);}
        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1089]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(105, 317, 314, p.p628, 316, p.p629, 318, p.p630, p.p627);}
        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1090]) {s.store_primal_add_scaled_inputs3_offset_indices(106, 314, p.p632, 316, p.p633, 318, p.p634, p.p631);}
        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1091]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(114, 317, 314, p.p636, 316, p.p637, 318, p.p638, p.p635);}
        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1092]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(115, 317, 314, p.p640, 316, p.p641, 318, p.p642, p.p639);}
        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1093]) {s.store_primal_add_scaled_inputs3_offset_indices(118, 314, p.p644, 316, p.p645, 318, p.p646, p.p643);}
        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1094]) {s.store_primal_add_scaled_inputs3_offset_indices(119, 314, p.p648, 316, p.p649, 318, p.p650, p.p647);}
        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1095]) {s.store_primal_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p652, s.ad_value(316), p.p653, s.ad_value(318), p.p654, p.p651), 1.0 / (1e-6), 0.0);}
        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1096]) {s.store_primal_add_scaled_inputs3_offset_indices(123, 314, p.p656, 316, p.p657, 318, p.p658, p.p655);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1097]) {s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p.p660, 316, p.p661, 318, p.p662, p.p659);}
        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1098]) {s.store_scalar(32, p.p571);}
        s.b[1099] = param_given[663];s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {s.store_scalar(32, p.p663);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(33, p.p572);}
        s.b[1100] = param_given[664];s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {s.store_scalar(33, p.p664);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(34, p.p573);}
        s.b[1101] = param_given[665];s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {s.store_scalar(34, p.p665);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(35, p.p574);}
        s.b[1102] = param_given[666];s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {s.store_scalar(35, p.p666);}
        if (s.b[1030] && s.b[1098]) {s.store_primal_mul_mixed_ia(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));}
        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1103]) {s.store_scalar(32, p.p587);}
        s.b[1104] = param_given[667];s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {s.store_scalar(32, p.p667);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(33, p.p588);}
        s.b[1105] = param_given[668];s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {s.store_scalar(33, p.p668);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(34, p.p589);}
        s.b[1106] = param_given[669];s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {s.store_scalar(34, p.p669);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(35, p.p590);}
        s.b[1107] = param_given[670];s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {s.store_scalar(35, p.p670);}
        if (s.b[1030] && s.b[1103]) {s.store_primal_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);}
        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1108]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(127, 314, 314, p.p672, 316, p.p673, 318, p.p674, p.p671);}
        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1109]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(128, 314, 314, p.p676, 316, p.p677, 318, p.p678, p.p675);}
        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1110]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(129, 322, 314, p.p680, 316, p.p681, 318, p.p682, p.p679);}
        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1111]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(130, 322, 314, p.p684, 316, p.p685, 318, p.p686, p.p683);}
        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1112]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(134, 325, 314, p.p688, 316, p.p689, 318, p.p690, p.p687);}
        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1113]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(135, 322, 314, p.p692, 316, p.p693, 318, p.p694, p.p691);}
        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1114]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(136, 322, 314, p.p696, 316, p.p697, 318, p.p698, p.p695);}
        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1115]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(141, 326, 314, p.p700, 316, p.p701, 318, p.p702, p.p699);}
        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1116]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(142, 326, 314, p.p704, 316, p.p705, 318, p.p706, p.p703);}
        s.b[1117] = (((param_given[707] || param_given[708]) || param_given[709]) || param_given[710]);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1117]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(144, 315, 314, p.p708, 316, p.p709, 318, p.p710, p.p707);}
        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1121]) {s.store_primal_add_scaled_inputs3_offset_indices(149, 314, p.p724, 316, p.p725, 318, p.p726, p.p723);}
        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1122]) {s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p.p728, 316, p.p729, 318, p.p730, p.p727);}
        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1123]) {s.store_primal_add_scaled_inputs3_offset_indices(151, 314, p.p732, 316, p.p733, 318, p.p734, p.p731);}
        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1124]) {s.store_primal_add_scaled_inputs3_offset_indices(152, 314, p.p736, 316, p.p737, 318, p.p738, p.p735);}
        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1125]) {s.store_primal_add_scaled_inputs3_offset_indices(153, 314, p.p740, 316, p.p741, 318, p.p742, p.p739);}
        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1126]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(154, A::add_scaled_inputs3_offset(s.ad_value(314), p.p744, s.ad_value(316), p.p745, s.ad_value(318), p.p746, p.p743), 344, 1.0, 312, 1.0);}
        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1127]) {s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p.p748, 316, p.p749, 318, p.p750, p.p747);}
        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1128]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(156, 315, 314, p.p752, 316, p.p753, 318, p.p754, p.p751);}
        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1129]) {s.store_primal_add_scaled_inputs3_offset_indices(157, 314, p.p756, 316, p.p757, 318, p.p758, p.p755);}
        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1130]) {s.store_primal_add_scaled_inputs3_offset_indices(158, 314, p.p760, 316, p.p761, 318, p.p762, p.p759);}
        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1131]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(159, 315, 314, p.p764, 316, p.p765, 318, p.p766, p.p763);}
        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1132]) {s.store_primal_add_scaled_inputs3_offset_indices(160, 314, p.p772, 316, p.p773, 318, p.p774, p.p771);}
        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1133]) {s.store_primal_add_scaled_inputs3_offset_indices(161, 314, p.p768, 316, p.p769, 318, p.p770, p.p767);}
        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1137]) {s.store_primal_add_scaled_inputs3_offset_indices(176, 314, p.p788, 316, p.p789, 318, p.p790, p.p787);}
        if s.b[1030] {s.store_scalar(1019, 0.0);s.store_scalar(1020, 0.0);s.store_scalar(1018, 0.0);s.store_scalar(43, p.p795);}
        s.b[1138] = param_given[796];s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1138]) {s.store_scalar(43, p.p796);}
        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });let mut t10: usize = 0;
        while {
            let te: f64 = (s.v[5] - 0.5);let tf: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < te)) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1030] && s.b[1139]) {s.store_primal_add_mixed_ia(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));s.store_primal_add_mixed_ia(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));s.store_primal_offset(1018, 1018, 1.0);}
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_mul(1003, 1019, 6);s.store_primal_mul(1004, 1020, 6);s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));}
        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_primal_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                s.store_primal_offset_add(1017, 8, 311, p.p793);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);s.store_primal_add_scaled_inputs_product_mixed_aiii(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (s.v[352] - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (s.v[352] - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (s.v[352] - 1.0)))));s.store_primal_div_scaled_inputs2_indices(1008, 1003, p.p794, 1004, p.p794, 1007, 1.0);s.store_primal_div_scaled_inputs2_indices(1009, 1005, p.p794, 1006, p.p794, 1007, 1.0);s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);s.store_primal_add_scaled_inputs_product_mixed_aiii(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);s.store_primal_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);s.store_primal_div_scaled_offset_numerator_mixed_ia(1013, 1008, 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);s.store_primal_mul(69, 69, 1013);s.store_primal_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);s.store_primal_mul(154, 154, 1013);s.store_primal_div_scaled_inputs_indices(1013, 1012, p.p803, 1010, 1.0);s.store_primal_add(44, 44, 1013);s.store_primal_add(149, 149, 1013);s.store_primal_div_scaled_inputs_mixed_ia(1013, 1012, p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);s.store_primal_add(66, 66, 1013);s.store_primal_add(159, 159, 1013);}
        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {s.store_primal_offset(1012, 8, s.v[12]);s.store_scalar(1013, (1.0 / p.p811));s.store_primal_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);s.store_primal_div_scaled_add_product_mixed_aaai(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), 8, 1.0);s.store_primal_div_scaled_add_product_mixed_aaai(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), 8, 1.0);}
        if (s.b[1030] && s.b[1140]) {s.store_primal_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p.p812, 17, p.p813);s.store_primal_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);}
        s.copy_ad(177, 44);s.copy_ad(178, 45);s.copy_ad(179, 46);s.copy_ad(181, 47);s.copy_ad(182, 48);
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
        s.copy_ad(186, 52);s.copy_ad(187, 53);
        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }
        s.copy_ad(192, 59);s.copy_ad(193, 60);
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
    }
}
