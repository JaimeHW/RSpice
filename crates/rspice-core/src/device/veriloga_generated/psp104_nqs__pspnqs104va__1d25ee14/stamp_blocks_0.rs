#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[990] = (p[37] >= 0.0);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
        if s.b[990] {s.store_scalar(0, 1.0);}
        if (!s.b[990]) {s.store_scalar(0, (-1.0));}
        s.store_scalar(767, (8.8541878176e-12 * 11.8));s.b[991] = (p[51] < 0.5);s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
        let (t9,) = {
    if s.b[991] {
        (0.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t9);s.b[992] = (p[51] < 1.5);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        let (tc,) = {
    if ((!s.b[991]) && s.b[992]) {
        (1.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, tc);s.b[993] = (p[51] < 2.5);s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });
        let (td,) = {
    if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
        (2.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, td);s.b[994] = (p[51] < 4.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        let (t0,) = {
    if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
        (3.0,)
    } else {
        (s.v[1],)
    }
};
        s.store_scalar(1, t0);s.b[995] = (p[51] < 7.0);s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });
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
        s.store_scalar(1, t3);s.store_scalar(2, 1000.0);s.store_scalar(3, 10.0);s.store_scalar(4, (1.0 / s.v[3]));s.store_scalar(350, (273.15 + p[38]));s.store_scalar(474, 0.0);s.b[996] = (p[927] > 0.5);s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
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
        s.store_scalar(474, t8);s.store_scalar(364, (273.15 + p[823]));s.store_scalar(367, (1.3806505e-23 / 1.6021918e-19));s.store_scalar(368, (s.v[367] * s.v[364]));s.store_scalar(369, (1.0 / s.v[368]));s.store_scalar(375, ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364])));s.store_scalar(378, (p[834] + s.v[375]));s.store_scalar(379, (p[835] + s.v[375]));s.store_scalar(380, (p[836] + s.v[375]));s.store_scalar(408, (1.0 - p[831]));s.store_scalar(409, (1.0 - p[832]));s.store_scalar(410, (1.0 - p[833]));s.store_scalar(411, (1.0 / s.v[408]));s.store_scalar(412, (1.0 / s.v[409]));s.store_scalar(413, (1.0 / s.v[410]));s.store_scalar(423, (s.v[767] / p[825]));s.store_scalar(424, ((p[843] * s.v[767]) / p[826]));s.store_scalar(425, ((p[844] * s.v[767]) / p[827]));s.store_scalar(426, (1.0 / s.v[423]));s.store_scalar(427, (1.0 / s.v[424]));s.store_scalar(428, (1.0 / s.v[425]));s.store_scalar(429, (1.0 / p[828]));s.store_scalar(430, (1.0 / p[829]));s.store_scalar(431, (1.0 / p[830]));s.store_scalar(372, (1.772453850905516 * 0.29214664));s.store_scalar(373, (((((-5.0) * 0.29214664) + 6.0) - ((s.v[372]) as f64).powi(((-2.0) as i32))) / 3.0));s.store_scalar(374, ((1.0 - 0.29214664) - s.v[373]));s.store_scalar(444, (1.0 - (1.0 / p[824])));s.store_scalar(445, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p[863]))));s.store_scalar(446, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p[864]))));s.store_scalar(447, (1.0 / (1.0 - ((s.v[444]) as f64).powf(p[865]))));s.store_scalar(448, (1.0 / p[860]));s.store_scalar(449, (1.0 / p[861]));s.store_scalar(450, (1.0 / p[862]));s.store_scalar(451, (((-((s.v[445] * s.v[445]) * ((s.v[444]) as f64).powf((p[863] - 1.0)))) * p[863]) * s.v[448]));s.store_scalar(452, (((-((s.v[446] * s.v[446]) * ((s.v[444]) as f64).powf((p[864] - 1.0)))) * p[864]) * s.v[449]));s.store_scalar(453, (((-((s.v[447] * s.v[447]) * ((s.v[444]) as f64).powf((p[865] - 1.0)))) * p[865]) * s.v[450]));s.b[997] = ((((p[866] != 1.0) || (p[867] != 1.0)) || (p[868] != 1.0)) || (p[869] != 1.0));s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
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
        if s.b[998] {s.store_scalar(457, (if ((p[827] * p[866]) > 1e-18) { (p[827] * p[866]) } else { 1e-18 }));}
        if s.b[998] {s.store_scalar(458, (if ((p[830] * p[867]) > 0.05) { (p[830] * p[867]) } else { 0.05 }));}
        if s.b[998] {s.store_scalar(459, (if ((if ((p[833] * p[868]) > 0.05) { (p[833] * p[868]) } else { 0.05 }) < 0.95) { (if ((p[833] * p[868]) > 0.05) { (p[833] * p[868]) } else { 0.05 }) } else { 0.95 }));}
        if s.b[998] {s.store_scalar(460, (p[836] * p[869]));s.store_primal_offset(462, 460, s.v[375]);s.store_primal_sub_from_scalar(467, 1.0, 459);s.store_primal_div_from_scalar(468, 1.0, 467);}
        s.b[999] = (p[44] == 0.0);s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if s.b[999] {s.store_scalar(505, p[825]);s.store_scalar(506, p[826]);s.store_scalar(507, p[827]);s.store_scalar(508, p[828]);s.store_scalar(509, p[829]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[999] {s.store_scalar(510, p[830]);s.store_scalar(511, p[831]);s.store_scalar(512, p[832]);s.store_scalar(513, p[833]);s.store_scalar(514, p[834]);s.store_scalar(515, p[835]);s.store_scalar(516, p[836]);s.store_scalar(517, p[837]);s.store_scalar(518, p[838]);s.store_scalar(519, p[839]);s.store_scalar(522, p[840]);s.store_scalar(523, p[841]);s.store_scalar(524, p[842]);s.store_scalar(520, p[843]);s.store_scalar(521, p[844]);s.store_scalar(525, p[845]);s.store_scalar(526, p[846]);s.store_scalar(527, p[847]);s.store_scalar(528, p[848]);s.store_scalar(529, p[849]);s.store_scalar(530, p[850]);s.store_scalar(531, p[851]);s.store_scalar(532, p[852]);s.store_scalar(533, p[853]);s.store_scalar(534, p[854]);s.store_scalar(535, p[855]);s.store_scalar(536, p[856]);s.store_scalar(537, p[857]);s.store_scalar(538, p[858]);s.store_scalar(539, p[859]);s.store_scalar(540, p[860]);s.store_scalar(541, p[861]);s.store_scalar(542, p[862]);s.store_scalar(543, p[863]);s.store_scalar(544, p[864]);s.store_scalar(545, p[865]);s.store_scalar(552, p[928]);}
        let (t1,) = {
    if s.b[999] {
        (p[929],)
    } else {
        (s.v[553],)
    }
};
        s.store_scalar(553, t1);
        if s.b[999] {s.store_scalar(636, p[872]);s.store_scalar(637, p[873]);s.store_scalar(638, p[874]);s.store_scalar(639, p[875]);s.store_scalar(546, p[866]);s.store_scalar(547, p[867]);s.store_scalar(548, p[868]);s.store_scalar(549, p[869]);s.store_scalar(550, p[870]);s.store_scalar(551, p[871]);}
        if (!s.b[999]) {s.store_scalar(505, p[876]);s.store_scalar(506, p[877]);s.store_scalar(507, p[878]);s.store_scalar(508, p[879]);s.store_scalar(509, p[880]);s.store_scalar(510, p[881]);s.store_scalar(511, p[882]);s.store_scalar(512, p[883]);s.store_scalar(513, p[884]);s.store_scalar(514, p[885]);s.store_scalar(515, p[886]);s.store_scalar(516, p[887]);s.store_scalar(517, p[888]);s.store_scalar(518, p[889]);s.store_scalar(519, p[890]);s.store_scalar(522, p[891]);s.store_scalar(523, p[892]);s.store_scalar(524, p[893]);s.store_scalar(520, p[894]);s.store_scalar(521, p[895]);s.store_scalar(525, p[896]);s.store_scalar(526, p[897]);s.store_scalar(527, p[898]);s.store_scalar(528, p[899]);s.store_scalar(529, p[900]);s.store_scalar(530, p[901]);s.store_scalar(531, p[902]);s.store_scalar(532, p[903]);s.store_scalar(533, p[904]);s.store_scalar(534, p[905]);s.store_scalar(535, p[906]);s.store_scalar(536, p[907]);s.store_scalar(537, p[908]);s.store_scalar(538, p[909]);s.store_scalar(539, p[910]);s.store_scalar(540, p[911]);s.store_scalar(541, p[912]);s.store_scalar(542, p[913]);s.store_scalar(543, p[914]);s.store_scalar(544, p[915]);s.store_scalar(545, p[916]);s.store_scalar(552, p[930]);}
        let (t4,) = {
    if (!s.b[999]) {
        (p[931],)
    } else {
        (s.v[553],)
    }
};
        s.store_scalar(553, t4);
        if (!s.b[999]) {s.store_scalar(636, p[923]);s.store_scalar(637, p[924]);s.store_scalar(638, p[925]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[999]) {s.store_scalar(639, p[926]);s.store_scalar(546, p[917]);s.store_scalar(547, p[918]);s.store_scalar(548, p[919]);s.store_scalar(549, p[920]);s.store_scalar(550, p[921]);s.store_scalar(551, p[922]);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
    ) {
        s.store_scalar(878, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(351, ((ctx_temp + p[56]) + p[35]));s.store_scalar(352, (s.v[351] / s.v[350]));s.store_scalar(353, (s.v[351] - s.v[350]));s.store_scalar(354, ((s.v[351] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(355, (1.0 / s.v[354]));s.store_scalar(356, s.v[351]);s.store_scalar(357, (s.v[356] * s.v[356]));s.store_scalar(358, (s.v[356] - s.v[350]));s.store_scalar(359, (s.v[350] / s.v[356]));s.store_scalar(360, ((s.v[359]) as f64).ln());s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(361, (1.0 / s.v[715]));s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));
        if (!(s.v[363] > 0.001)) {s.store_scalar(363, 0.001);}
        s.store_scalar(718, ((4.0 * 1.3806505e-23) * s.v[356]));s.store_scalar(365, (((ctx_temp + p[56]) + p[35])).max((273.15 + (-250.0))));s.store_scalar(366, (s.v[365] / s.v[364]));s.store_scalar(370, (s.v[367] * s.v[365]));s.store_scalar(371, (1.0 / s.v[370]));s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));s.store_scalar(381, (p[834] + s.v[376]));s.store_scalar(382, (p[835] + s.v[376]));s.store_scalar(383, (p[836] + s.v[376]));s.store_scalar(384, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp()));s.store_scalar(385, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp()));s.store_scalar(386, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp()));s.store_scalar(387, ((p[837] * s.v[384]) * s.v[384]));s.store_scalar(388, ((p[838] * s.v[385]) * s.v[385]));s.store_scalar(389, ((p[839] * s.v[386]) * s.v[386]));s.store_scalar(390, ((p[828] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln())));s.store_scalar(391, ((p[829] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln())));s.store_scalar(392, ((p[830] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln())));s.store_scalar(393, (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(394, (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(395, (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(405, (1.0 / s.v[393]));s.store_scalar(406, (1.0 / s.v[394]));s.store_scalar(407, (1.0 / s.v[395]));s.store_scalar(414, (p[825] * (((p[828] * s.v[405])) as f64).powf(p[831])));s.store_scalar(415, (p[826] * (((p[829] * s.v[406])) as f64).powf(p[832])));s.store_scalar(416, (p[827] * (((p[830] * s.v[407])) as f64).powf(p[833])));s.store_scalar(417, ((s.v[414] * s.v[393]) * s.v[411]));s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));s.store_scalar(420, (2.0 * s.v[414]));s.store_scalar(421, (2.0 * s.v[415]));s.store_scalar(422, (2.0 * s.v[416]));s.store_scalar(432, ((0.5 * s.v[381])).max(s.v[370]));s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[370]));s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[370]));s.store_scalar(435, (s.v[432] * s.v[371]));s.store_scalar(436, (s.v[433] * s.v[371]));s.store_scalar(437, (s.v[434] * s.v[371]));s.store_scalar(438, (((((((32.0 * p[848]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(439, (((((((32.0 * p[849]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(440, (((((((32.0 * p[850]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(441, (p[854] * (1.0 + (p[857] * (s.v[365] - s.v[364])))));s.store_scalar(442, (p[855] * (1.0 + (p[858] * (s.v[365] - s.v[364])))));s.store_scalar(443, (p[856] * (1.0 + (p[859] * (s.v[365] - s.v[364])))));
        if (!(s.v[441] > 0.0)) {s.store_scalar(441, 0.0);}
        if (!(s.v[442] > 0.0)) {s.store_scalar(442, 0.0);}
        if (!(s.v[443] > 0.0)) {s.store_scalar(443, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        s: &mut Scratch,
    ) {
        s.b[1021] = (s.v[473] == 1.0);s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
    ) {
        if s.b[1021] {s.store_primal_offset(461, 460, s.v[376]);s.store_primal_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(464, 458, s.v[366], 463, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(466, 1.0, 465);s.store_primal_mul_pow_mixed_iai(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), 459);s.store_primal_mul3_lhs(470, 469, 465, 468);s.store_primal_scale(471, 469, 2.0);}
        s.store_primal_offset(557, 514, s.v[376]);s.store_primal_offset(558, 515, s.v[376]);s.store_primal_offset(559, 516, s.v[376]);s.store_primal_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_mul3_lhs(563, 517, 560, 560);s.store_primal_mul3_lhs(564, 518, 561, 561);s.store_primal_mul3_lhs(565, 519, 562, 562);s.store_primal_sub_scaled_inputs_ln_rhs(566, 508, s.v[366], 560, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(567, 509, s.v[366], 561, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(568, 510, s.v[366], 562, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(572, 1.0, 569);s.store_primal_div_from_scalar(573, 1.0, 570);s.store_primal_div_from_scalar(574, 1.0, 571);s.store_primal_mul_pow_mixed_iai(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), 511);s.store_primal_mul_pow_mixed_iai(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), 512);s.store_primal_mul_pow_mixed_iai(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), 513);s.store_primal_mul3_lhs(584, 581, 569, 578);s.store_primal_mul3_lhs(585, 582, 570, 579);s.store_primal_mul3_lhs(586, 583, 571, 580);s.store_primal_scale(587, 581, 2.0);s.store_primal_scale(588, 582, 2.0);s.store_primal_scale(589, 583, 2.0);s.store_primal_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);s.store_primal_scale(602, 599, s.v[371]);s.store_primal_scale(603, 600, s.v[371]);s.store_primal_scale(604, 601, s.v[371]);s.store_primal_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
    ) {
        s.store_primal_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);s.store_primal_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);
        if (!(s.v[608] > 0.0)) {s.store_scalar(608, 0.0);}
        if (!(s.v[609] > 0.0)) {s.store_scalar(609, 0.0);}
        if (!(s.v[610] > 0.0)) {s.store_scalar(610, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
    ) {
        s.b[1022] = (s.v[635] == 1.0);s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
    ) {
        if s.b[1022] {s.store_primal_offset(624, 623, s.v[376]);s.store_primal_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(627, 621, s.v[366], 626, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(629, 1.0, 628);s.store_primal_mul_pow_mixed_iai(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), 622);s.store_primal_mul3_lhs(633, 632, 628, 631);s.store_primal_scale(634, 632, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(5, 1.0);s.store_scalar(6, 1.0);s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);s.store_scalar(7, p[0]);s.store_scalar(8, p[1]);s.store_scalar(9, p[2]);s.store_scalar(10, p[3]);s.store_scalar(11, p[4]);s.store_scalar(12, p[8]);s.store_scalar(13, p[11]);s.store_scalar(646, p[19]);s.store_scalar(647, p[20]);s.store_scalar(648, p[21]);s.store_scalar(673, p[22]);s.store_scalar(674, p[23]);s.store_scalar(675, p[24]);s.store_scalar(649, p[25]);s.store_scalar(650, p[26]);s.store_scalar(676, p[27]);s.store_scalar(677, p[28]);s.store_scalar(14, p[14]);s.b[1023] = (p[39] > 0.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if s.b[1023] {s.store_scalar(5, (if (p[9] > 1.0) { p[9] } else { 1.0 }));}
        if s.b[1023] {s.store_primal_floor_ad(5, A::offset(s.ad_value(5), 0.5));s.store_primal_div_from_scalar(6, 1.0, 5);}
        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_primal_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }
        s.store_scalar(15, p[5]);s.store_scalar(16, p[6]);s.store_scalar(17, p[7]);s.store_scalar(18, (if (p[10] < 1.5) { 1.0 } else { 2.0 }));s.store_scalar(308, (1e-6 / s.v[7]));s.store_primal_div_from_scalar(309, 1e-6, 8);s.store_primal_offset_scaled(310, 309, ((p[190]) * ((p[188] * (1.0 + (p[189] * s.v[308]))))), (p[188] * (1.0 + (p[189] * s.v[308]))));s.store_primal_offset_scaled(311, 309, ((p[194]) * ((p[192] * (1.0 + (p[193] * s.v[308]))))), (p[192] * (1.0 + (p[193] * s.v[308]))));
        if (((s.v[7] + s.v[310]) - (2.0 * p[191])) > 1e-9) {
            s.store_primal_offset(312, 310, ((s.v[7]) + ((-(2.0 * p[191])))));
        } else {
            s.store_scalar(312, 1e-9);
        }
        if (((s.v[8] + s.v[311]) - (2.0 * p[195])) > 1e-9) {
            s.store_primal_offset_add(313, 8, 311, (-(2.0 * p[195])));
        } else {
            s.store_scalar(313, 1e-9);
        }
        s.store_primal_div_from_scalar(314, 1e-6, 312);s.store_primal_square(315, 314);s.store_primal_div_from_scalar(316, 1e-6, 313);s.store_primal_div_from_scalar(317, 1.0, 316);s.store_primal_mul(318, 314, 316);s.store_primal_div_from_scalar(319, 1.0, 318);
        if ((((s.v[7] + s.v[310]) - (2.0 * p[191])) + p[196]) > 1e-9) {
            s.store_primal_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p[191]))))) + (p[196])));
        } else {
            s.store_scalar(320, 1e-9);
        }
        if ((((s.v[8] + s.v[311]) - (2.0 * p[195])) + p[197]) > 1e-9) {
            s.store_primal_offset_add(321, 8, 311, (((-(2.0 * p[195]))) + (p[197])));
        } else {
            s.store_scalar(321, 1e-9);
        }
        s.store_primal_scale(322, 321, 1000000.0);
        if (((s.v[7] + s.v[310]) + p[196]) > 1e-9) {
            s.store_primal_offset(323, 310, ((s.v[7]) + (p[196])));
        } else {
            s.store_scalar(323, 1e-9);
        }
        if (((s.v[8] + s.v[311]) + p[197]) > 1e-9) {
            s.store_primal_offset_add(324, 8, 311, p[197]);
        } else {
            s.store_scalar(324, 1e-9);
        }
        s.store_primal_scale(325, 323, 1000000.0);s.store_primal_scale(326, 324, 1000000.0);
        if ((s.v[7] + s.v[310]) > 1e-9) {
            s.store_primal_offset(327, 310, s.v[7]);
        } else {
            s.store_scalar(327, 1e-9);
        }
        if ((s.v[327] + p[443]) > 1e-9) {
            s.store_primal_offset(328, 327, p[443]);
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
        s.store_scalar(44, p[57]);s.store_scalar(45, p[58]);s.store_scalar(46, p[59]);s.store_scalar(47, p[60]);s.store_scalar(48, p[61]);s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.store_scalar(51, p[64]);s.store_scalar(52, p[65]);s.store_scalar(53, p[66]);s.store_scalar(54, p[67]);s.store_scalar(59, p[68]);s.store_scalar(60, p[69]);s.store_scalar(61, p[70]);s.store_scalar(62, p[71]);s.store_scalar(55, p[72]);s.store_scalar(56, p[74]);s.store_scalar(57, p[73]);s.store_scalar(58, p[75]);s.store_scalar(63, p[79]);s.store_scalar(64, p[81]);s.store_scalar(65, p[80]);s.store_scalar(66, p[76]);s.store_scalar(67, p[78]);s.store_scalar(68, p[77]);s.store_scalar(69, p[82]);s.store_scalar(70, p[83]);s.store_scalar(71, p[84]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(72, p[85]);s.store_scalar(73, p[86]);s.store_scalar(74, p[87]);s.store_scalar(75, p[88]);s.store_scalar(76, p[89]);s.store_scalar(77, p[90]);s.store_scalar(78, p[91]);s.store_scalar(79, p[92]);s.store_scalar(80, p[93]);s.store_scalar(81, p[94]);s.store_scalar(82, p[95]);s.store_scalar(83, p[96]);s.store_scalar(84, p[97]);s.store_scalar(85, p[98]);s.store_scalar(86, p[99]);s.store_scalar(87, p[100]);s.store_scalar(88, p[101]);s.store_scalar(89, p[102]);s.store_scalar(90, p[103]);s.store_scalar(91, p[104]);s.store_scalar(92, p[105]);s.store_scalar(93, p[106]);s.store_scalar(94, p[107]);s.store_scalar(95, p[108]);s.store_scalar(96, p[109]);s.store_scalar(97, p[110]);s.store_scalar(98, p[111]);s.store_scalar(99, p[112]);s.store_scalar(100, p[113]);s.store_scalar(101, p[114]);s.store_scalar(102, p[115]);s.store_scalar(103, p[116]);s.store_scalar(104, p[117]);s.store_scalar(105, p[118]);s.store_scalar(106, p[119]);s.store_scalar(107, p[120]);s.store_scalar(108, p[121]);s.store_scalar(109, p[120]);s.b[1024] = param_given[122];s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if s.b[1024] {s.store_scalar(109, p[122]);}
        s.store_scalar(110, p[121]);s.b[1025] = param_given[123];s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if s.b[1025] {s.store_scalar(110, p[123]);}
        s.copy_ad(111, 109);s.b[1026] = param_given[124];s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if s.b[1026] {s.store_scalar(111, p[124]);}
        s.copy_ad(112, 110);s.b[1027] = param_given[125];s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if s.b[1027] {s.store_scalar(112, p[125]);}
        s.store_scalar(113, p[126]);s.store_scalar(114, p[127]);s.store_scalar(115, p[128]);s.store_scalar(116, p[129]);s.store_scalar(117, p[130]);s.store_scalar(118, p[131]);s.store_scalar(119, p[132]);s.store_scalar(120, p[133]);s.store_scalar(121, p[134]);s.store_scalar(122, p[135]);s.store_scalar(123, p[136]);s.store_scalar(124, p[137]);s.store_scalar(125, p[99]);s.b[1028] = param_given[138];s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
        if s.b[1028] {s.store_scalar(125, p[138]);}
        s.store_scalar(126, p[104]);s.b[1029] = param_given[139];s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if s.b[1029] {s.store_scalar(126, p[139]);}
        s.store_scalar(127, p[140]);s.store_scalar(128, p[141]);s.store_scalar(129, p[142]);s.store_scalar(130, p[143]);s.store_scalar(131, p[144]);s.store_scalar(132, p[145]);s.store_scalar(133, p[146]);s.store_scalar(134, p[147]);s.store_scalar(135, p[148]);s.store_scalar(136, p[149]);s.store_scalar(137, p[150]);s.store_scalar(138, p[151]);s.store_scalar(139, p[152]);s.store_scalar(140, p[153]);s.store_scalar(141, p[154]);s.store_scalar(142, p[155]);s.store_scalar(143, p[156]);s.store_scalar(144, p[157]);s.store_scalar(149, p[162]);s.store_scalar(150, p[163]);s.store_scalar(151, p[164]);s.store_scalar(152, p[165]);s.store_scalar(153, p[166]);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(154, p[167]);s.store_scalar(155, p[168]);s.store_scalar(156, p[169]);s.store_scalar(157, p[170]);s.store_scalar(158, p[171]);s.store_scalar(159, p[172]);s.store_scalar(160, p[174]);s.store_scalar(161, p[173]);s.store_scalar(167, p[180]);s.store_scalar(170, p[181]);s.store_scalar(171, p[182]);s.store_scalar(172, p[184]);s.store_scalar(173, p[183]);s.store_scalar(174, p[185]);s.store_scalar(175, p[186]);s.store_scalar(176, p[187]);s.b[1030] = (p[39] > 0.0);s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if s.b[1030] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(44, A::powf(s.ad_value(314), p[200]), p[199], 316, p[201], 318, p[202], p[198]);s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p[204], 316, p[205], 318, p[206], p[203]);s.store_scalar(46, p[207]);s.store_scalar(47, p[208]);s.store_scalar(48, p[209]);}
        if s.b[1030] {
            s.store_primal_scale_ad(331, {
                if ((1.0 + ((p[211] * s.v[316]) * (((1.0 + (s.v[313] / p[212]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[211], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[212]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[210]);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(332, {
                if ((1.0 + ((p[214] * s.v[316]) * (((1.0 + (s.v[313] / p[215]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[214], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[215]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[213]);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(333, {
                if ((1.0 + ((p[217] * s.v[316]) * (((1.0 + (s.v[313] / p[215]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[217], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[215]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[216]);
        }
        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1031]) {s.store_scalar(334, 75000000000.0);s.store_primal_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));s.store_primal_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);s.store_primal_square(336, 336);}
        s.b[1032] = (s.v[312] >= s.v[333]);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {s.store_primal_add_mixed_ia(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));}
        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {s.store_primal_add_mixed_ia(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));}
        if s.b[1030] {s.store_primal_mul_sub_scaled_inputs_rhs_mixed_ai(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p[218])), 1.0, 315, p[219]);s.store_primal_add_scaled_inputs3_offset_mixed_aii(50, A::powf(s.ad_value(314), p[222]), p[221], 316, p[223], 318, p[224], p[220]);s.store_scalar(51, p[225]);s.store_scalar(52, p[226]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1030] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(53, A::powf(s.ad_value(314), p[229]), p[228], 316, p[230], 318, p[231], p[227]);}
        if s.b[1030] {
            s.store_primal_scale_ad(54, {
                if (1e-6 > (1.0 + (p[233] * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p[233], 1.0)
                }
            }, p[232]);
        }
        if s.b[1030] {s.store_scalar(59, p[234]);s.store_scalar(60, p[235]);s.store_scalar(61, p[238]);s.store_scalar(62, p[239]);s.store_primal_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p[242]), p[241], p[240]), A::scale_offset(s.ad_value(316), p[243], 1.0), A::scale_offset(s.ad_value(318), p[244], 1.0));s.store_scalar(56, p[246]);s.store_scalar(57, p[245]);s.store_scalar(58, p[247]);s.store_primal_mul_powf_scale_offset_lhs(66, 314, 316, p[249], (p[250]) * (p[248]), (1.0) * (p[248]));s.store_scalar(67, p[252]);s.store_scalar(68, p[251]);s.store_primal_mul_powf_scale_offset_lhs(63, 314, 316, p[254], (p[255]) * (p[253]), (1.0) * (p[253]));s.store_scalar(64, p[257]);s.store_scalar(65, p[256]);s.store_primal_offset_scaled(337, 316, ((p[260]) * (p[259])), p[259]);}
        if s.b[1030] {
            s.store_primal_scale_ad(338, {
                if ((1.0 + (p[262] * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p[262], 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[261]);
        }
        if s.b[1030] {s.store_primal_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[263] * p[264]), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p[264])))));}
        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p[265], 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[267]), 1.0)), p[266]);s.store_primal_mul_div_scaled_inputs_mixed_iia(69, 340, 313, p[258], A::mul(s.ad_value(339), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p[269], 316, p[270], 318, p[271], p[268]);s.store_primal_offset_scaled(71, 316, ((p[273]) * (p[272])), p[272]);s.store_scalar(72, p[274]);s.store_scalar(73, p[275]);s.store_scalar(74, p[276]);s.store_primal_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p[279]), p[278], p[277]), A::scale_offset(s.ad_value(316), p[280], 1.0), A::scale_offset(s.ad_value(318), p[281], 1.0));s.store_scalar(76, p[282]);s.store_scalar(77, p[283]);s.store_scalar(78, p[284]);s.store_primal_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p[286], 1.0), A::scale_offset(s.ad_value(316), p[287], 1.0), A::scale_offset(s.ad_value(318), p[288], 1.0), p[285]);s.store_scalar(80, p[289]);s.store_scalar(81, p[290]);s.store_primal_mul_scale_offset_rhs(82, 316, 316, ((p[292]) * (p[291])), p[291]);s.store_scalar(83, p[293]);s.store_scalar(84, p[294]);s.store_scalar(85, p[295]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_primal_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p[297], s.ad_value(339), 1.0), A::powf(s.ad_value(314), p[298])), p[296]), A::scale_offset(s.ad_value(316), p[299], 1.0), A::scale_offset(s.ad_value(318), p[300], 1.0));s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p[302], 316, p[303], 318, p[304], p[301]);s.store_scalar(88, p[305]);s.store_scalar(89, p[306]);s.store_scalar(90, p[307]);s.store_primal_div_from_scalar_offset_scaled_input(91, p[308], 314, p[309], 1.0);s.store_primal_mul_powf_scale_offset_lhs(92, 314, 316, p[311], (p[312]) * (p[310]), (1.0) * (p[310]));s.store_primal_powf(341, 314, p[314]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(93, 341, A::scale_offset(s.ad_value(316), p[316], 1.0), p[313], A::mul_scaled_lhs(s.ad_value(314), p[315], s.ad_value(341)), 1.0, 1.0);s.store_primal_powf(341, 314, p[318]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(94, 341, A::scale_offset(s.ad_value(316), p[320], 1.0), p[317], A::mul_scaled_lhs(s.ad_value(314), p[319], s.ad_value(341)), 1.0, 1.0);s.store_scalar(95, p[321]);s.store_primal_scaled_mul_scale_offset_inputs(96, 314, p[323], 1.0, 316, p[324], 1.0, p[322]);s.store_scalar(97, p[325]);s.store_scalar(98, p[326]);s.store_primal_scaled_mul_scale_offset_inputs(99, 314, p[328], 1.0, 316, p[329], 1.0, p[327]);s.store_primal_scaled_mul_scale_offset_inputs(100, 314, p[331], 1.0, 316, p[332], 1.0, p[330]);s.store_scalar(101, p[333]);s.store_scalar(102, p[334]);s.store_primal_div_from_scalar(103, p[335], 318);s.store_primal_div_from_scalar_scaled_input(104, (p[336] * p[236]), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(105, (p[337] * p[237]), 316, 1e-6);s.store_scalar(106, p[338]);s.store_scalar(107, p[339]);s.store_scalar(108, p[340]);s.store_scalar(109, p[339]);}
        s.b[1033] = param_given[341];s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1033]) {s.store_scalar(109, p[341]);}
        if s.b[1030] {s.store_scalar(110, p[340]);}
        s.b[1034] = param_given[342];s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1034]) {s.store_scalar(110, p[342]);}
        if s.b[1030] {s.copy_ad(111, 109);}
        s.b[1035] = param_given[343];s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1035]) {s.store_scalar(111, p[343]);}
        if s.b[1030] {s.copy_ad(112, 110);}
        s.b[1036] = param_given[344];s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1036]) {s.store_scalar(112, p[344]);}
        if s.b[1030] {s.store_scalar(113, p[345]);s.store_primal_div_from_scalar_scaled_input(114, (p[346] * p[236]), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(115, (p[347] * p[237]), 316, 1e-6);s.store_scalar(116, p[348]);s.store_scalar(117, p[349]);s.store_scalar(118, p[350]);s.store_scalar(119, p[351]);s.store_scalar(120, p[352]);s.store_scalar(121, p[353]);s.store_primal_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p[209]) * 1.0 / (p[208])));s.store_primal_scale(129, 321, ((8.8541878176e-12 * p[209]) * (p[236] * 1.0 / (p[234]))));s.store_primal_scale(130, 321, ((8.8541878176e-12 * p[209]) * (p[237] * 1.0 / (p[235]))));s.store_primal_add_scaled_inputs3_offset_mixed_aii(123, A::powf(s.ad_value(314), p[356]), p[355], 316, p[357], 318, p[358], p[354]);s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p[360], 316, p[361], 318, p[362], p[359]);s.store_scalar(36, p[296]);}
        s.b[1037] = param_given[363];s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1037]) {s.store_scalar(36, p[363]);}
        if s.b[1030] {s.store_scalar(37, p[297]);}
        s.b[1038] = param_given[364];s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1038]) {s.store_scalar(37, p[364]);}
        if s.b[1030] {s.store_scalar(38, p[298]);}
        s.b[1039] = param_given[365];s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1039]) {s.store_scalar(38, p[365]);}
        if s.b[1030] {s.store_scalar(39, p[299]);}
        s.b[1040] = param_given[366];s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1040]) {s.store_scalar(39, p[366]);}
        if s.b[1030] {s.store_scalar(40, p[300]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1041] = param_given[367];s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1041]) {s.store_scalar(40, p[367]);}
        if s.b[1030] {s.store_primal_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));s.store_scalar(41, p[308]);}
        s.b[1042] = param_given[368];s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1042]) {s.store_scalar(41, p[368]);}
        if s.b[1030] {s.store_scalar(42, p[309]);}
        s.b[1043] = param_given[369];s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1043]) {s.store_scalar(42, p[369]);}
        if s.b[1030] {s.store_primal_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);s.store_primal_mul_powf_scale_offset_lhs(127, 314, 316, p[371], (p[372]) * (p[370]), (1.0) * (p[370]));s.store_primal_powf(341, 314, p[374]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(128, 341, A::scale_offset(s.ad_value(316), p[376], 1.0), p[373], A::mul_scaled_lhs(s.ad_value(314), p[375], s.ad_value(341)), 1.0, 1.0);s.store_scalar(131, p[377]);s.store_scalar(132, p[378]);s.store_scalar(133, p[379]);s.store_primal_scale(134, 325, p[380]);s.store_primal_scale(135, 322, p[381]);s.store_primal_scale(136, 322, p[382]);s.store_scalar(137, p[383]);s.store_scalar(138, p[384]);s.store_scalar(139, p[385]);s.store_scalar(140, p[386]);s.store_primal_scale(141, 326, p[387]);s.store_primal_scale(142, 326, p[388]);s.store_primal_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p[395]), s.ad_value(312)));s.store_scalar(143, p[389]);s.store_primal_mul_product3_indices(144, 316, 69, 69, 316, p[390]);s.store_primal_offset_scaled(344, 313, p[398], (2.0 * p[397]));s.store_scalar(149, p[399]);s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p[401], 316, p[402], 318, p[403], p[400]);s.store_primal_add_scaled_inputs3_offset_mixed_aii(151, A::powf(s.ad_value(314), p[406]), p[405], 316, p[407], 318, p[408], p[404]);s.store_primal_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p[411]), p[410], 1.0), A::scale_offset(s.ad_value(316), p[412], 1.0), A::scale_offset(s.ad_value(318), p[413], 1.0), p[409]);s.store_primal_offset_scaled_ad(153, A::powf(s.ad_value(314), p[416]), p[415], p[414]);s.store_primal_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[417] * p[418]), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p[418])))), 1.0);}
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_mul_div_scaled_inputs_mixed_aia(154, A::scale_offset(s.ad_value(316), p[419], 1.0), 344, p[258], A::mul(s.ad_value(347), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p[421], 316, p[422], 318, p[423], p[420]);s.store_primal_mul_powf_scale_offset_lhs(156, 314, 316, p[425], (p[426]) * (p[424]), (1.0) * (p[424]));s.store_scalar(157, p[427]);s.store_scalar(158, p[428]);s.store_primal_mul_powf_scale_offset_lhs(159, 314, 316, p[430], (p[431]) * (p[429]), (1.0) * (p[429]));}
    }
}
