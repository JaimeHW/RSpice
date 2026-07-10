#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_0(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[627] = (p.p43 == 0.0);s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });s.store_scalar(246, 0.0);s.store_scalar(300, 1e-12);s.store_scalar(25, 0.0);s.store_scalar(146, 0.0);s.store_scalar(612, 0.0);s.store_scalar(556, 0.0);s.store_scalar(145, 0.0);s.store_scalar(338, 0.0);s.store_scalar(162, 0.0);s.store_scalar(163, 0.0);s.store_scalar(164, 0.0);s.store_scalar(165, 0.0);s.store_scalar(176, 1.0);s.store_scalar(190, 0.0);s.store_scalar(192, 0.0);s.store_scalar(196, 0.0);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_scalar(199, 0.0);s.store_scalar(242, 0.0);s.store_scalar(244, 0.0);s.store_scalar(250, 0.0);s.store_scalar(251, 0.0);s.store_scalar(252, 0.0);s.store_scalar(263, 0.0);s.store_scalar(264, 1.0);s.store_scalar(265, 0.0);s.store_scalar(267, 0.0);s.store_scalar(268, 0.0);s.store_scalar(272, 0.0);s.store_scalar(454, 0.0);s.store_scalar(455, 0.0);s.store_scalar(456, 0.0);s.store_scalar(457, 0.0);s.store_scalar(282, 0.0);s.store_scalar(281, 0.0);s.store_scalar(284, 0.0);s.store_scalar(283, 0.0);s.store_scalar(478, 0.0);s.store_scalar(479, 0.0);s.store_scalar(402, p.p237);s.store_scalar(463, 0.0);s.store_scalar(464, 0.0);s.store_scalar(466, 0.0);s.store_scalar(465, 0.0);s.store_scalar(467, 0.0);s.store_scalar(468, 0.0);s.store_scalar(470, 0.0);s.store_scalar(469, 0.0);s.store_scalar(522, 0.0);s.store_scalar(523, 0.0);s.store_scalar(471, 0.0);s.store_scalar(473, 0.0);s.store_scalar(289, 0.0);s.store_scalar(290, 0.0);s.store_scalar(293, 0.0);s.store_scalar(294, 0.0);s.store_scalar(296, 0.0);s.store_scalar(297, 0.0);s.store_scalar(298, 0.0);s.store_scalar(299, 0.0);s.store_scalar(301, 0.0);s.store_scalar(314, 0.0);s.store_scalar(315, 0.0);s.store_scalar(316, 0.0);s.store_scalar(339, 0.0);s.store_scalar(346, 0.0);s.store_scalar(347, 0.0);s.store_scalar(348, 0.0);s.store_scalar(349, 0.0);s.store_scalar(350, 0.0);s.store_scalar(351, 0.0);s.store_scalar(352, 0.0);s.store_scalar(353, 0.0);s.store_scalar(354, 0.0);s.store_scalar(370, 0.0);s.store_scalar(355, 0.0);s.store_scalar(363, 0.0);s.store_scalar(366, 0.0);s.store_scalar(356, 0.0);s.store_scalar(357, 0.0);s.store_scalar(358, 0.0);s.store_scalar(359, 0.0);s.store_scalar(360, 0.0);s.store_scalar(383, 0.0);s.store_scalar(386, 0.0);s.store_scalar(574, 0.0);s.store_scalar(575, 0.0);s.store_scalar(582, 0.0);s.store_scalar(580, 0.0);s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(390, 0.0);s.store_scalar(392, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_1(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(393, 0.0);s.store_scalar(401, 0.0);s.store_scalar(376, 0.0);s.store_scalar(436, 0.0);s.store_scalar(437, 0.0);s.store_scalar(438, 0.5);s.store_scalar(439, 0.5);s.store_scalar(447, 0.0);s.store_scalar(476, 0.0);s.store_scalar(477, 0.0);s.store_scalar(592, 0.0);s.store_scalar(576, 0.0);s.store_scalar(577, 0.0);s.store_scalar(587, 0.0);s.store_scalar(588, 0.0);s.store_scalar(488, 0.0);s.store_scalar(490, 0.0);s.store_scalar(497, 0.0);s.store_scalar(499, 0.0);let t0: f64 = (p.p51 * 10.0);let t1: f64 = (t0 % 10.0);s.store_scalar(56, t1);s.store_scalar(57, 200.0);s.store_scalar(58, 200.0);s.store_scalar(86, 0.0);s.store_scalar(475, 0.0);s.store_scalar(378, 0.0);s.store_scalar(369, 0.0);s.store_scalar(203, 0.0);s.store_scalar(161, 0.0);s.store_scalar(515, 0.0);s.store_scalar(73, (p.p52 * 0.01));s.store_scalar(59, (p.p73 / 1e-6));s.store_scalar(60, (p.p104 * 0.01));s.store_scalar(61, (p.p201 / 1e-6));s.store_scalar(65, (p.p240 / 1e-6));s.store_scalar(66, (p.p241 / 1e-6));s.store_scalar(67, (p.p242 * 0.01));s.store_scalar(68, (p.p243 / 0.01));s.store_scalar(69, (p.p59 / 1e-6));s.store_scalar(70, (p.p284 / 1e-6));s.store_scalar(71, (p.p148 / 1e-6));s.store_scalar(72, (p.p198 / 0.0001));s.store_scalar(74, (p.p70 * 0.01));s.store_scalar(75, (if (p.p83 == 0.0) { 0.0 } else { p.p84 }));s.store_scalar(76, (if (p.p83 == 0.0) { 0.0 } else { p.p85 }));s.store_scalar(77, (if (p.p80 == 0.0) { 0.0 } else { p.p81 }));s.store_scalar(78, (if (p.p83 == 0.0) { 0.0 } else { p.p82 }));s.store_scalar(79, (p.p250 * 1000000.0));s.store_scalar(81, (p.p232 + 273.15));s.store_scalar(82, p.p58);s.store_scalar(83, (p.p15 * 100.0));s.store_scalar(84, p.p46);s.store_scalar(85, p.p34);s.store_scalar(80, (if param_given[190] { p.p190 } else { (5000000000.0 / (p.p237 * p.p240)) }));s.b[628] = ((s.v[80] < (2.0 + 0.1)) && (0.1 >= 0.0));s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(44, ((2.0 + 0.1) - s.v[80]));s.store_square(49, 44);s.store_scalar(50, (0.1 * 0.1));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2,) = {
    if s.b[628] {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2);
        let (t3,) = {
    if s.b[628] {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3);
        if s.b[628] {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[629] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });s.b[630] = (2.0 == 1.0);s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((s.b[628] && s.b[629]) && s.b[630]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4);s.b[631] = (2.0 == 2.0);s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (((s.b[628] && s.b[629]) && (!s.b[630])) && s.b[631]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5);s.b[632] = (2.0 == 4.0);s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        let (t6,) = {
    if ((((s.b[628] && s.b[629]) && (!s.b[630])) && (!s.b[631])) && s.b[632]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t6);s.b[633] = (2.0 == 8.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        let (t7,) = {
    if (((((s.b[628] && s.b[629]) && (!s.b[630])) && (!s.b[631])) && (!s.b[632])) && s.b[633]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t7);
        let (t8,) = {
    if (s.b[628] && s.b[629]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t8);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_2(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let mut tc: usize = 0;
        while {
            let tb: f64 = if ((s.b[628] && s.b[629]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;assert!(tc <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[628] && s.b[629]) {s.store_sqrt(53, 53);}
            let (ta,) = {
    if (s.b[628] && s.b[629]) {
        let t9: f64 = (s.v[54] + 1.0);
        (t9,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, ta);
        }
        if (s.b[628] && (!s.b[629])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if s.b[628] {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.1);s.store_sub_from_scalar(80, (2.0 + 0.1), 43);}
        if (!s.b[628]) {
        }
        s.store_scalar(87, (p.p55 - (s.v[81] * (9.025e-5 + (s.v[81] * 1e-7)))));s.store_scalar(88, p.p236);s.store_scalar(89, (1.034943e-10 / p.p237));s.store_scalar(90, (1.0 / s.v[89]));s.store_scalar(91, (3.453133e-11 / s.v[88]));s.store_scalar(92, (s.v[88] / 3.453133e-11));s.store_scalar(93, (3.453133e-11 / p.p239));s.store_scalar(94, (p.p239 / 3.453133e-11));s.store_scalar(95, (s.v[94] + s.v[90]));s.store_scalar(96, p.p0);s.store_scalar(97, (s.v[96] - (2.0 * p.p56)));s.store_scalar(98, (s.v[96] - (2.0 * p.p57)));s.store_scalar(99, (if (p.p40 == 0.0) { s.v[96] } else { s.v[97] }));s.store_scalar(100, (s.v[99] * 1000000.0));s.store_scalar(101, (p.p1 / p.p9));s.store_scalar(102, p.p60);s.store_scalar(103, (if (s.v[56] < 1.0) { 0.0 } else { p.p295 }));s.store_scalar(104, (if (s.v[56] < 1.0) { p.p60 } else { p.p61 }));s.b[634] = (p.p43 == 0.0);s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if s.b[634] {s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));}
        if (!s.b[634]) {s.store_scalar(105, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[102])));s.store_scalar(106, ((s.v[101] - (p.p18 * s.v[103])) - ((2.0 - p.p18) * s.v[104])));}
        s.store_primal_scale(107, 105, p.p9);s.store_primal_scale(108, 106, p.p9);s.store_scalar(109, (s.v[101] * 1000000.0));s.store_scalar(110, (s.v[109] * s.v[100]));s.store_scalar(111, ((p.p107 * (1.0 + (p.p108 / ((s.v[100]) as f64).powf(p.p111)))) * (1.0 + (p.p109 / ((s.v[109]) as f64).powf(p.p110)))));s.b[635] = (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p.p72 > 0.0));s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if s.b[635] {s.store_scalar(59, s.v[65]);}
        s.store_primal_scale(112, 59, (1.0 + (p.p74 / ((s.v[109]) as f64).powf(p.p75))));s.store_scalar(113, (2.0 / ((1.0 / (p.p62 + (0.5 * s.v[96]))) + (1.0 / (p.p63 + (0.5 * s.v[96]))))));s.store_scalar(114, (1.6021918e-19 / (1.3806226e-23 * s.v[81])));s.store_scalar(115, ((1.6021918e-19 * s.v[66]) * 1.034943e-10));s.store_scalar(116, (p.p244 * ((s.v[100]) as f64).powf((-p.p247))));s.store_scalar(117, (p.p251 * ((s.v[100]) as f64).powf((-p.p252))));s.store_scalar(118, (p.p248 * (((s.v[100] + s.v[79])) as f64).powf((-p.p249))));s.store_scalar(119, (((((2.0 * 1.6021918e-19) * s.v[71]) * 1.034943e-10)) as f64).sqrt());s.store_scalar(120, (1.0 / (s.v[71] * s.v[71])));s.store_scalar(121, ((((1.0 + (1.0 / s.v[100]))) as f64).powf(p.p91) * p.p89));s.store_scalar(122, s.v[115]);s.store_scalar(123, p.p68);s.store_scalar(124, (s.v[99] + (p.p76 / ((s.v[110]) as f64).powf(p.p77))));s.store_scalar(125, (p.p78 / ((s.v[110]) as f64).powf(p.p79)));s.store_scalar(126, ((p.p149 * (1.0 + (p.p150 / (((s.v[124] * 1000000.0)) as f64).powf(p.p151)))) + (p.p152 / ((s.v[109]) as f64).powf(p.p153))));s.store_scalar(127, (1.0 + (((s.v[100]) as f64).powf(p.p192) * p.p193)));s.store_primal_offset_scaled(128, 105, ((1.0 / ((3.0 * p.p6))) * ((p.p67 * 1.0 / (((p.p6 * (s.v[96] - p.p8)) * p.p9))))), ((p.p7) * ((p.p67 * 1.0 / (((p.p6 * (s.v[96] - p.p8)) * p.p9))))));s.b[636] = (p.p44 <= 0.0);s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if s.b[636] {s.store_scalar(129, (1.0 + (p.p130 / ((s.v[109]) as f64).powf(p.p131))));s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));s.store_scalar(131, (s.v[100] / (s.v[100] + p.p123)));s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));}
        if (!s.b[636]) {s.store_scalar(329, ((s.v[109]) as f64).powf(p.p131));s.store_div_scaled_value_offset_denominator(134, s.ad_value(329), (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))), s.ad_value(329), p.p130, 1.0);s.store_scalar(130, (p.p124 * (1.0 + (p.p125 / ((s.v[100]) as f64).powf(p.p126)))));s.store_scalar(131, (p.p123 * (1.0 + (p.p132 / ((s.v[100]) as f64).powf(p.p133)))));s.store_scalar(132, (p.p117 * (1.0 + (p.p119 / ((s.v[100]) as f64).powf(p.p120)))));s.store_scalar(133, (p.p118 * (1.0 + (p.p121 / s.v[100]))));}
        s.store_primal_scale(135, 108, (1000000.0 * (p.p65 * 1.0 / (((s.v[100]) as f64).powf(p.p66)))));s.store_scalar(136, (p.p134 * (1.0 + (p.p135 / ((s.v[100]) as f64).powf(p.p136)))));s.b[637] = (p.p44 <= 0.0);s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if s.b[637] {s.store_scalar(137, (p.p127 * (1.0 + (p.p128 / ((s.v[100]) as f64).powf(p.p129)))));}
        s.store_scalar(138, (((((p.p115 * s.v[100]) * p.p114) / ((p.p115 * s.v[100]) + p.p114)) + p.p116) + 1e-50));s.b[638] = (s.v[138] < 3.0);s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if s.b[638] {s.store_scalar(138, 3.0);}
        s.store_scalar(139, (p.p50 * p.p253));s.b[564] = param_given[168];s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });s.b[565] = param_given[169];s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });s.b[566] = param_given[170];s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });s.b[525] = param_given[294];s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });s.b[524] = param_given[293];s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_3(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[529] = param_given[13];s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });s.b[530] = param_given[14];s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });s.b[527] = param_given[23];s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });s.b[526] = param_given[22];s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });s.b[539] = param_given[16];s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });s.b[540] = (p.p17 != 0.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });s.store_scalar(451, 1.0);s.store_scalar(142, 0.0);s.store_scalar(518, p.p13);s.store_scalar(519, p.p14);s.store_scalar(520, (p.p16 + 273.15));s.store_primal_div_from_scalar_scaled_input(541, s.v[67], 107, s.v[451]);s.store_primal_scale(542, 108, (s.v[451] * s.v[68]));s.b[639] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p9 == 1.0) || ((p.p9 > 1.0) && (p.p12 > 0.0))));s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if s.b[639] {s.store_scalar(328, 0.0);s.store_scalar(562, 0.0);}
        let mut te: usize = 0;
        while {
            let td: f64 = if (s.b[639] && (s.v[562] < p.p9)) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;assert!(te <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[639] {s.store_add_scaled_inputs3_mixed_iaa(328, 328, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p10 + (0.5 * s.v[96])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p.p12 + s.v[96]), (p.p11 + (0.5 * s.v[96])))), 1.0);s.store_primal_offset(562, 562, 1.0);}
        }
        if s.b[639] {s.store_div_from_scalar(537, (2.0 * p.p9), 328);}
        if (!s.b[639]) {s.store_scalar(537, 0.0);}
        s.b[640] = (s.v[537] > 0.0);s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if s.b[640] {s.store_scalar(328, (1.0 / (1.0 + p.p162)));s.store_powf_ad(329, A::div_from_scalar(p.p161, s.ad_value(537)), p.p163);s.store_scalar(330, (((p.p161 / s.v[113])) as f64).powf(p.p163));s.store_div_scaled_product_offset_denominator_mixed_iaa(538, 112, A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);}
        if (!s.b[640]) {s.copy_ad(538, 112);}
        s.store_scalar(329, ((1.0 + (p.p199 / ((s.v[109]) as f64).powf(p.p200))) * (1.0 + (p.p202 / ((s.v[100]) as f64).powf(p.p203)))));s.store_scalar(330, (s.v[61] / s.v[65]));s.store_scalar(44, ((s.v[330] - s.v[329]) - 0.01));s.store_scalar(45, ((4.0 * s.v[330]) * 0.01));
        if (!(s.v[45] > 0.0)) {s.store_scalar(45, (-s.v[45]));}
        s.store_sqrt_offset_input(45, 45, (s.v[44] * s.v[44]));s.store_sub_from_scalar_ad(328, s.v[330], A::scaled_offset(s.ad_value(45), s.v[44], 0.5));s.store_scale(544, 328, s.v[65]);s.b[641] = (s.v[537] > 0.0);s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if s.b[641] {s.store_scalar(328, (1.0 / (1.0 + p.p165)));s.store_powf_ad(329, A::div_from_scalar(p.p164, s.ad_value(537)), p.p166);s.store_scalar(330, (((p.p164 / s.v[113])) as f64).powf(p.p166));s.store_div_scaled_product_offset_denominator_mixed_iaa(544, 544, A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);}
        s.b[642] = ((s.v[99] > p.p72) || (p.p72 <= 0.0));s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_add_scaled_inputs(536, 544, ((s.v[99] - p.p72) * 1.0 / (s.v[99])), 538, (p.p72 * 1.0 / (s.v[99])));}
        if (!s.b[642]) {s.store_add_scaled_inputs3_indices(536, 538, 1.0, 538, ((p.p72 - s.v[99]) * 1.0 / (p.p72)), 544, (-((p.p72 - s.v[99]) * 1.0 / (p.p72))));}
        s.store_scale(229, 536, 1.6021918e-19);s.store_scale(545, 229, 1.034943e-10);s.store_scale(546, 545, 2.0);s.b[643] = ((s.v[99] <= (2.0 * p.p72)) && (p.p72 > 0.0));s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if s.b[643] {s.store_add_scaled_inputs4_indices(593, 538, 2.0, 538, (-(s.v[99] * 1.0 / (p.p72))), 544, (-(-(s.v[99] * 1.0 / (p.p72)))), 544, -1.0);s.store_ln_div(548, 593, 544);}
        if (!s.b[643]) {s.store_scalar(548, 0.0);}
        s.store_scaled_ln_scaled_input(232, 536, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));s.store_scaled_ln_scaled_input(236, 544, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));s.store_sqrt_div_from_scalar_ad(549, ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536));s.store_scalar(328, ((1.0 + (p.p194 / ((s.v[100]) as f64).powf(p.p195))) * (1.0 + (p.p196 / ((s.v[110]) as f64).powf(p.p197)))));s.store_scalar(44, ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt());
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);s.store_scalar(550, ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001)));s.b[644] = (s.v[550] < 0.0);s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if s.b[644] {s.store_scalar(550, 0.0);}
        s.b[645] = (p.p35 == 1.0);s.store_scalar(645, if s.b[645] { 1.0 } else { 0.0 });s.b[646] = (s.v[128] > 0.001);s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if (s.b[645] && s.b[646]) {s.store_primal_div_from_scalar(551, s.v[451], 128);}
        if (s.b[645] && (!s.b[646])) {s.store_scalar(551, (s.v[451] * 1000.0));}
        if (!s.b[645]) {s.store_scalar(551, (s.v[451] * 1000.0));}
        s.b[647] = (p.p261 == 1.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if s.b[647] {s.store_offset_scaled(327, 107, p.p289, p.p288);s.store_scale(2, 327, 1.0 / (s.v[451]));}
        s.b[648] = (s.v[2] < 0.0001);s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[648]) {s.store_scalar(2, 0.0001);}
        if (!s.b[647]) {s.store_scalar(2, 0.0001);}
        s.store_scalar(552, 0.0);s.store_scalar(553, 0.0);s.b[649] = (p.p262 == 1.0);s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });s.b[650] = (p.p290 < 0.0001);s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (s.b[649] && s.b[650]) {s.store_scalar(552, (s.v[451] * 10000.0));}
        if (s.b[649] && (!s.b[650])) {s.store_scalar(552, (s.v[451] * (1e-6 + (1.0 / p.p290))));}
        s.b[651] = (p.p291 < 0.0001);s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if (s.b[649] && s.b[651]) {s.store_scalar(553, (s.v[451] * 10000.0));}
        if (s.b[649] && (!s.b[651])) {s.store_scalar(553, (s.v[451] * (1e-6 + (1.0 / p.p291))));}
        s.b[652] = (p.p43 == 1.0);s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (s.b[652] && (p.p24 != 0.0)) {s.store_scalar(533, (if s.b[527] { p.p23 } else { ((p.p20 * p.p9) * p.p19) }));}
        if (s.b[652] && (p.p24 != 0.0)) {s.store_scalar(534, (if s.b[526] { p.p22 } else { ((p.p21 * p.p9) * p.p19) }));}
        if (s.b[652] && (p.p24 != 0.0)) {s.store_scalar(531, 0.0);s.store_scalar(532, 0.0);}
        s.b[653] = ((s.v[533] > 0.0) && s.b[525]);s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if ((s.b[652] && (p.p24 != 0.0)) && s.b[653]) {s.store_primal_scale(531, 533, (-p.p294));}
        if ((s.b[652] && (p.p24 != 0.0)) && (!s.b[653])) {s.store_scalar(531, 0.0);}
        s.b[654] = ((s.v[534] > 0.0) && s.b[524]);s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if ((s.b[652] && (p.p24 != 0.0)) && s.b[654]) {s.store_primal_scale(532, 534, (-p.p293));s.store_scalar(534, 0.0);}
        if (s.b[652] && (p.p24 == 0.0)) {s.store_scalar(534, 0.0);s.store_scalar(532, 0.0);s.store_scalar(533, 0.0);s.store_scalar(531, 0.0);}
        if s.b[652] {s.store_scalar(535, (if (p.p19 > s.v[96]) { (0.5 * (p.p19 - s.v[96])) } else { 0.0 }));}
        s.b[655] = (!s.b[529]);s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if (s.b[652] && s.b[655]) {s.copy_ad(518, 535);}
        s.b[656] = (!s.b[530]);s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if (s.b[652] && s.b[656]) {s.copy_ad(519, 535);}
        if s.b[652] {s.store_primal_add_scaled_inputs(286, 107, 1.0, 518, p.p9);s.store_primal_add_scaled_inputs(285, 107, 1.0, 519, p.p9);s.store_primal_add_scaled_inputs(288, 108, 1.0, 518, p.p9);s.store_primal_add_scaled_inputs(287, 108, 1.0, 519, p.p9);}
        if (!s.b[652]) {s.store_scalar(534, 0.0);s.store_scalar(532, 0.0);s.store_scalar(533, 0.0);s.store_scalar(531, 0.0);s.store_scalar(286, 0.0);s.store_scalar(285, 0.0);s.store_scalar(288, 0.0);s.store_scalar(287, 0.0);}
        s.store_scaled_voltage(571, ctx, nodes, Some(6), Some(7), p.p50);s.store_scaled_voltage(572, ctx, nodes, Some(11), Some(7), p.p50);s.store_scaled_voltage(570, ctx, nodes, Some(12), Some(7), p.p50);s.b[657] = (p.p43 == 1.0);s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });
        if s.b[657] {s.store_scaled_voltage(590, ctx, nodes, Some(12), Some(6), p.p50);s.store_scaled_voltage(591, ctx, nodes, Some(12), Some(7), p.p50);}
        if (s.b[657] && (s.v[85] != 0.0)) {s.store_scaled_voltage(580, ctx, nodes, Some(18), None, (1e-9 / 0.0001));s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));}
        if (s.b[657] && (s.v[85] == 0.0)) {s.store_scalar(580, 0.0);s.store_scalar(581, 0.0);}
        if (!s.b[657]) {s.store_scalar(590, 0.0);s.store_scalar(591, 0.0);}
        if ((!s.b[657]) && (s.v[85] != 0.0)) {s.store_scaled_voltage(584, ctx, nodes, Some(15), None, (1e-9 / 0.0001));s.store_scaled_voltage(585, ctx, nodes, Some(16), None, (1e-9 / 0.0001));s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));}
        if ((!s.b[657]) && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(581, 0.0);}
        if ((p.p38 > 0.0) && (s.v[67] > 0.0)) {
            if (nv10 > 0.0) {
                s.store_voltage(20, ctx, nodes, Some(10), None);
            } else {
                s.store_scalar(20, 0.0);
            }
        } else {
            s.store_scalar(20, 0.0);
        }
        s.b[658] = (s.v[571] >= 0.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        let (tf,) = {
    if s.b[658] {
        (1.0,)
    } else {
        (s.v[613],)
    }
};
        s.store_scalar(613, tf);
        if s.b[658] {s.store_scalar(461, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[658] {s.store_scalar(462, 0.0);s.copy_ad(157, 571);s.copy_ad(158, 572);s.copy_ad(156, 570);}
        let (t11,) = {
    if (!s.b[658]) {
        let t10: f64 = (-1.0);
        (t10,)
    } else {
        (s.v[613],)
    }
};
        s.store_scalar(613, t11);
        if (!s.b[658]) {s.store_scalar(461, 0.0);s.store_scalar(462, 1.0);s.store_neg(157, 571);s.store_sub(158, 572, 571);s.store_sub(156, 570, 571);}
        s.store_scalar(429, ctx_temp);
        if s.b[539] {s.store_scalar(429, s.v[520]);}
        if s.b[540] {s.store_offset(429, 429, p.p17);}
        s.store_add(429, 429, 20);s.store_offset(328, 429, (-s.v[81]));s.store_mul_scale_offset_indices(329, 328, 429, 1.0, s.v[81]);s.store_sub_scaled_inputs_mixed_ai(237, A::sub_from_scalar(s.v[87], A::scale(s.ad_value(328), p.p53)), 1.0, 329, p.p54);s.store_div_from_scalar_scaled_input(225, 1.6021918e-19, 429, 1.3806226e-23);s.store_square(226, 225);s.store_div_from_scalar(227, 1.0, 225);s.store_scalar(661, (((p.p254 * (1.0 + (p.p98 / ((s.v[109]) as f64).powf(p.p99)))) * (1.0 + (p.p100 / ((s.v[100]) as f64).powf(p.p101)))) * (1.0 + (p.p102 / ((s.v[110]) as f64).powf(p.p103)))));s.store_scalar(664, (1.0 / (1.0 + p.p159)));s.store_scalar(665, (if (((p.p158 / s.v[83]) == 0.0) && (p.p160 == 0.0)) { 1.0 } else { (((p.p158 / s.v[83])) as f64).powf(p.p160) }));s.store_scalar(662, (s.v[661] * (1.0 + (s.v[664] * s.v[665]))));s.store_powf_scaled_input(663, 429, 1.0 / (s.v[81]), p.p112);s.store_scale(543, 663, 1.0 / (s.v[662]));s.store_mul(433, 548, 227);s.store_scale(328, 429, 1.0 / (s.v[81]));s.store_div_scaled_inputs_mixed_ia(253, 550, s.v[73], A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(328), 0.4, 1.8), 1.0, s.ad_value(328), s.ad_value(328), 0.1), A::scale_offset(s.ad_value(328), (-s.v[60]), s.v[60])), 1.0);s.store_sqrt(302, 237);s.store_mul(303, 237, 302);s.store_scaled_mul_ad(230, A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(237), (-1.0 / (2.0)), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))), (10400000000.0 / 1e-6));s.store_scaled_sqrt(208, 227, s.v[119]);s.store_square(205, 208);s.store_scaled_square(209, 230, s.v[120]);s.store_scalar(441, (s.v[96] - (2.0 * p.p56)));s.b[666] = (s.v[56] > 3.0);s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(536), s.ad_value(230)));}
        if (!s.b[666]) {s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(544), s.ad_value(230)));}
        s.store_sqrt_mul_ad(228, A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227));s.store_scaled_mul(238, 229, 228, 1.414213562373095);s.b[667] = (p.p43 == 1.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_scalar(474, 0.0);s.store_scalar(239, 0.0);s.store_div(328, 230, 536);}
        if (!s.b[667]) {s.store_sqrt_scaled_input(474, 227, (2.0 * s.v[122]));s.store_scale(328, 230, 1.0 / (s.v[66]));s.store_square(239, 328);s.store_div(328, 230, 544);}
        s.store_square(379, 328);s.store_sqrt_scaled_input_ad(444, A::div_scalar_by_product(1.034943e-10, s.ad_value(229), s.ad_value(225), 1.0), 2.0);s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);let t12: f64 = (2.0 * 1.034943e-10);let t13: f64 = (t12 / 1.6021918e-19);let t14: f64 = (t13 * s.v[231]);let t15: f64 = (t14 / s.v[544]);let t16: f64 = (t15).sqrt();s.store_scalar(416, t16);s.b[672] = (p.p43 == 1.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_scalar(141, 0.4);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_6(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[672] {s.store_scalar(140, 0.8);}
        if (!s.b[672]) {s.store_scalar(141, 0.8);s.store_scalar(140, 1.2);}
        s.b[673] = (s.v[141] > (s.v[140] * 0.5));s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_primal_scale(141, 140, 0.5);}
        s.b[674] = (s.v[156] > s.v[141]);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if s.b[674] {s.store_sub(329, 156, 141);s.store_sub(330, 140, 141);s.store_square(49, 329);s.store_square(50, 330);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t17,) = {
    if s.b[674] {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t17);
        let (t18,) = {
    if s.b[674] {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t18);
        if s.b[674] {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[675] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });s.b[676] = (4.0 == 1.0);s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });
        let (t19,) = {
    if ((s.b[674] && s.b[675]) && s.b[676]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t19);s.b[677] = (4.0 == 2.0);s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });
        let (t1a,) = {
    if (((s.b[674] && s.b[675]) && (!s.b[676])) && s.b[677]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t1a);s.b[678] = (4.0 == 4.0);s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });
        let (t1b,) = {
    if ((((s.b[674] && s.b[675]) && (!s.b[676])) && (!s.b[677])) && s.b[678]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t1b);s.b[679] = (4.0 == 8.0);s.store_scalar(679, if s.b[679] { 1.0 } else { 0.0 });
        let (t1c,) = {
    if (((((s.b[674] && s.b[675]) && (!s.b[676])) && (!s.b[677])) && (!s.b[678])) && s.b[679]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t1c);
        let (t1d,) = {
    if (s.b[674] && s.b[675]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t1d);let mut t21: usize = 0;
        while {
            let t20: f64 = if ((s.b[674] && s.b[675]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;assert!(t21 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[674] && s.b[675]) {s.store_sqrt(53, 53);}
            let (t1f,) = {
    if (s.b[674] && s.b[675]) {
        let t1e: f64 = (s.v[54] + 1.0);
        (t1e,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t1f);
        }
        if (s.b[674] && (!s.b[675])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if s.b[674] {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(331, 329, 330, 53);s.store_div_scaled_product3_indices(335, 330, 52, 53, 1.0, 48, 1.0);s.store_add(154, 141, 331);s.copy_ad(155, 335);}
        if (!s.b[674]) {s.copy_ad(154, 156);s.store_scalar(155, 1.0);}
        if (s.v[157] > 20.0) {
            s.store_scalar(152, 20.0);
        } else {
            s.copy_ad(152, 157);
        }
        if (s.v[158] > 20.0) {
            s.store_scalar(153, 20.0);
        } else {
            s.copy_ad(153, 158);
        }
        if (s.v[158] < (-20.0)) {s.store_scalar(153, (-20.0));}
        if (s.v[154] < (-20.0)) {s.store_scalar(154, (-20.0));}
        s.copy_ad(157, 152);s.copy_ad(158, 153);s.copy_ad(156, 154);s.store_scalar(144, 0.0);s.store_scalar(619, 0.0);s.store_scalar(620, 0.0);s.store_scalar(621, 0.0);s.store_scalar(622, 0.0);s.store_scalar(623, 0.0);s.store_scalar(624, 0.0);s.store_scalar(425, 0.0);s.store_scalar(426, 0.0);s.store_scalar(427, 0.0);s.store_scalar(428, 0.0);s.store_scalar(167, 0.0);s.store_scalar(168, 0.0);s.store_scaled_mul(680, 155, 157, 0.5);s.store_scale(44, 680, (2.0 * 1.0 / (p.p226)));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_7(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(175, p.p226, 45);s.b[681] = (s.v[175] < 5e-12);s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });
        if s.b[681] {s.store_scalar(175, 5e-12);}
        s.store_add(172, 156, 175);s.store_add_scaled_inputs(173, 157, 1.0, 175, 2.0);s.store_add(174, 158, 175);s.b[682] = (p.p43 == 1.0);s.store_scalar(682, if s.b[682] { 1.0 } else { 0.0 });
        if s.b[682] {s.copy_ad(513, 156);s.copy_ad(514, 172);}
        if (!s.b[682]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(513, 156);
            } else {
                s.store_scalar(513, 0.0);
            }
        }
        if (!s.b[682]) {
            if (s.v[56] < 3.0) {
                s.copy_ad(514, 172);
            } else {
                s.store_scalar(514, 0.0);
            }
        }
        s.store_scale(683, 229, (2.0 * (1.034943e-10 * (s.v[92] * s.v[92]))));s.store_offset(684, 158, (-s.v[123]));s.store_offset_mul_ad(685, A::div_from_scalar(2.0, s.ad_value(683)), A::add_scaled_inputs3(s.ad_value(684), 1.0, s.ad_value(227), (-1.0), s.ad_value(513), -1.0), 1.0);s.store_sqrt_square_offset(44, 685, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(331, 685, 0.5, 44, 0.5, (1e-10 * 0.001));s.b[687] = (s.v[331] < 0.0);s.store_scalar(687, if s.b[687] { 1.0 } else { 0.0 });
        if s.b[687] {s.store_scalar(331, 0.0);}
        s.store_sqrt_offset_input(686, 331, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(193, 684, 683, 1.0, 686);s.store_sub(194, 193, 231);s.store_offset(44, 194, (((-0.1)) + ((-0.05))));s.store_scalar(45, ((4.0 * 0.1) * 0.05));
        if (!(s.v[45] > 0.0)) {s.store_scalar(45, (-s.v[45]));}
        s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(194, 44, 0.5, 45, 0.5, 0.1);s.store_div(683, 157, 194);s.copy_ad(44, 683);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(686, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(327, A::add_scaled_inputs3_offset(s.ad_value(44), 2.0, s.ad_value(45), 3.0, s.ad_value(46), 4.0, 1.0), s.ad_value(686), -1.0, 0.0, 686);s.store_sub_from_scalar(686, 1.0, 686);s.store_neg(327, 327);s.store_square(326, 686);s.b[694] = (((p.p204 == 0.0) && (p.p206 == 0.0)) || (p.p205 == 0.0));s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });
        let (t22,) = {
    if s.b[694] {
        (0.0,)
    } else {
        (s.v[148],)
    }
};
        s.store_scalar(148, t22);
        let (t23,) = {
    if (!s.b[694]) {
        (1.0,)
    } else {
        (s.v[148],)
    }
};
        s.store_scalar(148, t23);s.store_sqrt_mul_scaled_lhs(688, 229, (2.0 * 1.034943e-10), 232);s.store_add_scaled_inputs_mixed_ai(325, A::offset(s.ad_value(232), s.v[123]), 1.0, 688, 1.0 / (s.v[91]));s.b[695] = (s.v[148] == 0.0);s.store_scalar(695, if s.b[695] { 1.0 } else { 0.0 });
        if s.b[695] {s.store_scalar(321, s.v[88]);s.store_scalar(323, s.v[91]);s.store_scalar(324, s.v[92]);s.store_scaled_mul(434, 238, 238, (s.v[92] * s.v[92]));}
        if (!s.b[695]) {s.store_add_scaled_inputs3_offset_indices(692, 158, 1.0, 513, (-1.0), 325, -1.0, p.p205);s.store_sqrt_square_offset(44, 692, ((4.0 * 0.0001) * 0.0001));s.store_offset_add_scaled_inputs_indices(688, 692, 0.5, 44, 0.5, (1e-10 * 0.0001));}
        s.b[696] = (s.v[688] < 0.0);s.store_scalar(696, if s.b[696] { 1.0 } else { 0.0 });
        if ((!s.b[695]) && s.b[696]) {s.store_scalar(688, 0.0);}
        if (!s.b[695]) {s.store_div_from_scalar(689, 1.0, 688);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_8(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[695]) {s.store_scaled_abs(691, 325, 2.0);s.store_offset_sub_from_scalar_ad(693, s.v[123], s.ad_value(325), p.p205);}
        if (!s.b[695]) {
            if (s.v[693] > s.v[691]) {
                s.copy_ad(690, 693);
            } else {
                s.copy_ad(690, 691);
            }
        }
        if (!s.b[695]) {s.store_offset_sub_ad(44, A::div_from_scalar(1.0, s.ad_value(690)), s.ad_value(689), (-0.0001));s.store_scale_ad(45, A::div_from_scalar(1.0, s.ad_value(690)), (4.0 * 0.0001));}
        if (!s.b[695]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (!s.b[695]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_mixed_aii(688, A::div_from_scalar(1.0, s.ad_value(690)), 1.0, 44, (-0.5), 45, (-0.5));s.store_offset_scaled(322, 688, p.p204, p.p206);}
        s.b[697] = ((s.v[322] * 1000000000000.0) < s.v[88]);s.store_scalar(697, if s.b[697] { 1.0 } else { 0.0 });
        if ((!s.b[695]) && s.b[697]) {s.store_scalar(322, 0.0);}
        let (t24,) = {
    if ((!s.b[695]) && s.b[697]) {
        (0.0,)
    } else {
        (s.v[148],)
    }
};
        s.store_scalar(148, t24);
        if (!s.b[695]) {s.store_offset(321, 322, s.v[88]);s.store_div_from_scalar(323, 3.453133e-11, 321);s.store_scale(324, 321, 28959208927.08158);s.store_mul_ad_product_lhs_mixed_ai(434, A::square(s.ad_value(238)), 324, 324);}
        s.b[698] = ((p.p43 == 1.0) || (s.v[56] < 3.0));s.store_scalar(698, if s.b[698] { 1.0 } else { 0.0 });
        if s.b[698] {s.store_offset_sub_from_scalar_ad(44, 0.5, s.ad_value(514), (-0.001));s.store_scalar(45, ((4.0 * 0.5) * 0.001));}
        if s.b[698] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[698] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(435, 44, (-0.5), 45, (-0.5), 0.5);s.store_add_scaled_inputs3_indices(440, 229, (((-p.p237) * p.p237) * 1.0 / ((2.0 * 1.034943e-10))), 231, 1.0, 227, -1.0);s.store_offset_sub(44, 435, 440, (-0.001));s.store_scale(45, 440, (4.0 * 0.001));}
        if s.b[698] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[698] {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(435, 440, 1.0, 44, 0.5, 45, 0.5);}
        s.b[699] = (s.v[56] > 2.0);s.store_scalar(699, if s.b[699] { 1.0 } else { 0.0 });
        if (s.b[698] && s.b[699]) {s.store_offset_sub(44, 232, 435, (-0.001));s.store_scale(45, 232, (4.0 * 0.001));}
        if (s.b[698] && s.b[699]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[698] && s.b[699]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(435, 232, 1.0, 44, (-0.5), 45, (-0.5));}
        if (!s.b[698]) {s.store_scalar(435, 0.0);}
        s.b[700] = (s.v[56] < 3.0);s.store_scalar(700, if s.b[700] { 1.0 } else { 0.0 });
        if s.b[700] {s.store_scalar(184, p.p237);}
        if (!s.b[700]) {s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);s.store_sqrt_mul_sub_rhs(184, 328, 232, 435);}
        if (s.v[56] < 3.0) {
            s.store_sqrt_mul(245, 546, 232);
        } else {
            s.store_sqrt_mul_sub_rhs(245, 546, 232, 435);
        }
        s.store_add_mixed_ai(318, A::add_scaled_product(A::offset(s.ad_value(232), s.v[123]), 1.0, s.ad_value(245), s.ad_value(324), 1.0), 433);s.copy_ad(233, 232);s.store_scalar(702, 0.95);s.store_offset_sub_scaled_inputs_indices(701, 233, s.v[702], 435, 1.0, (-0.001));s.store_sqrt_add_scaled_square_input(703, 701, 1.0, 233, ((4.0 * s.v[702]) * 0.001));s.store_add_scaled_inputs3_indices(704, 233, s.v[702], 701, (-0.5), 703, (-0.5));s.store_sub(234, 233, 704);s.store_sqrt(235, 234);s.b[712] = (p.p72 != 0.0);s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });
        if s.b[712] {s.store_scale(706, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_9(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[712] {
            if (s.v[56] < 3.0) {
                s.store_sqrt_mul(707, 706, 236);
            } else {
                s.store_sqrt_mul_sub_rhs(707, 706, 236, 435);
            }
        }
        if s.b[712] {s.store_add_scaled_product_mixed_aii(183, A::offset(s.ad_value(236), s.v[123]), 1.0, 707, 324, 1.0);s.store_scale(706, 324, 1.034943e-10);s.store_scalar(709, (1.0 / (p.p72 * p.p72)));s.store_scaled_mul(708, 184, 709, 2.0);s.store_mul_ad_product_rhs_mixed_ia(710, 706, 708, A::sub_from_scalar(p.p69, s.ad_value(233)));s.copy_ad(711, 710);s.store_sub(706, 318, 183);s.store_scalar(705, (s.v[78] / p.p72));s.store_offset_mul(707, 705, 234, p.p80);s.store_scalar(710, s.v[77]);s.store_add_scaled_product_indices(708, 707, 1.0, 710, 173, 1.0);s.store_mul3_lhs(319, 706, 711, 708);}
        if (!s.b[712]) {s.store_scalar(319, 0.0);}
        s.store_scale(713, 184, (1.034943e-10 * 2.0));s.store_mul(714, 324, 713);s.store_sub_from_scalar(715, p.p69, 233);s.store_scalar(716, (s.v[99] - p.p71));s.store_scalar(717, (1.0 / (s.v[716] * s.v[716])));s.store_scaled_mul(719, 714, 715, s.v[717]);s.store_scalar(714, (s.v[76] / s.v[99]));s.store_offset_scaled(717, 234, s.v[714], p.p83);s.store_add_scaled_inputs(718, 717, 1.0, 173, s.v[75]);s.store_mul(187, 719, 718);s.b[723] = (p.p86 > 0.0);s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
        if s.b[723] {s.store_add_scaled_inputs3_offset_indices(720, 237, 1.0, 231, 1.0, 173, p.p87, (-(2.0 * p.p88)));s.store_scalar(721, ((s.v[99] * 0.5) + s.v[74]));s.store_primal_div_from_scalar(722, (p.p86 * p.p237), 721);s.store_mul(188, 720, 722);}
        if (!s.b[723]) {s.store_scalar(188, 0.0);}
        s.copy_ad(724, 324);s.store_div_from_scalar_add_ad(725, 1.0, s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105)));s.store_sub(726, 724, 725);s.store_offset_mul(189, 245, 726, (p.p105 / s.v[109]));s.store_add_scaled_inputs4_offset_indices(185, 187, 1.0, 319, 1.0, 189, 1.0, 188, 1.0, s.v[125]);let t25: f64 = (s.v[318] - s.v[185]);s.store_scalar(182, t25);s.b[730] = (p.p89 == 0.0);s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });
        let (t26,) = {
    if s.b[730] {
        (0.0,)
    } else {
        (s.v[147],)
    }
};
        s.store_scalar(147, t26);
        let (t27,) = {
    if (!s.b[730]) {
        (1.0,)
    } else {
        (s.v[147],)
    }
};
        s.store_scalar(147, t27);s.b[731] = (s.v[147] == 0.0);s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });
        if s.b[731] {s.store_scalar(320, 0.0);}
        if (!s.b[731]) {s.copy_ad(727, 174);s.store_scalar(728, s.v[121]);s.store_offset(729, 727, (-p.p90));}
        s.b[732] = (s.v[729] < (-3.0));s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });
        if ((!s.b[731]) && s.b[732]) {s.store_scalar(320, 0.0);}
        s.b[733] = (s.v[729] < 0.0);s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });
        if (((!s.b[731]) && (!s.b[732])) && s.b[733]) {s.store_offset_mul_offset_rhs_mixed_ia(320, 729, A::mul(s.ad_value(729), A::scale_offset(s.ad_value(729), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if (((!s.b[731]) && (!s.b[732])) && (!s.b[733])) {s.store_offset_mul_offset_rhs_mixed_ia(320, 729, A::mul_offset_rhs(s.ad_value(729), A::mul(s.ad_value(729), A::scale_offset(s.ad_value(729), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if (!s.b[731]) {s.store_sqrt_offset_square_offset(44, 320, (-1.0), ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_mixed_ai(320, A::offset(s.ad_value(320), (-1.0)), 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[734] = (s.v[320] < 0.0);s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });
        if ((!s.b[731]) && s.b[734]) {s.store_scalar(320, 0.0);}
        if (!s.b[731]) {s.store_mul(320, 320, 728);s.store_offset_sub_from_scalar_ad(44, 1.0, s.ad_value(320), (-0.05));s.store_scalar(45, (4.0 * 0.05));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_10(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[731]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (!s.b[731]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(320, 44, (-0.5), 45, (-0.5), 1.0);}
        s.store_add_scaled_inputs3_offset_indices(159, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.copy_ad(178, 159);s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));s.store_mul(342, 227, 328);let t28: f64 = (s.v[123] - s.v[185]);let t29: f64 = (t28 + s.v[320]);s.store_scalar(160, t29);s.store_mul(240, 238, 324);s.store_square(241, 240);s.b[735] = (p.p43 == 0.0);s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
        if s.b[735] {s.store_scalar(740, 7.0);s.store_offset(399, 231, 1.0);s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));s.store_add_mixed_ia(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));s.store_div_ln_lhs(180, 329, 330);s.store_sqrt_mul(403, 547, 180);}
        if s.b[735] {
            if (s.v[403] > p.p237) {
                s.store_scalar(403, p.p237);
            } else {
            }
        }
        if s.b[735] {s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));s.store_scalar(738, p.p237);s.store_scaled_mul(341, 544, 738, (-1.6021918e-19));s.store_scalar(739, 1.5);s.store_primal_div_from_scalar(736, 1.034943e-10, 738);s.store_primal_div_from_scalar(737, 1.0, 736);s.store_scale(741, 341, (-0.001));s.store_scale(742, 341, (-1e-5));}
        if (s.b[735] && (p.p39 != 0.0)) {s.store_add(475, 172, 342);}
        if (s.b[735] && (p.p39 == 0.0)) {s.store_add(475, 156, 342);}
        let (t2e,) = {
    if s.b[735] {
        let t2a: f64 = (2.0 / s.v[225]);let t2b: f64 = (s.v[66] / s.v[230]);let t2c: f64 = (t2b).ln();let t2d: f64 = (t2a * t2c);
        (t2d,)
    } else {
        (s.v[382],)
    }
};
        s.store_scalar(382, t2e);
        if s.b[735] {s.store_scaled_square(743, 474, (s.v[95] * s.v[95]));s.store_neg(744, 475);s.store_add_scaled_inputs3_mixed_aai(745, A::square(A::add_scaled_product(s.ad_value(744), 2.0, s.ad_value(743), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(744)), (-4.0), 743, (-4.0));}
        if s.b[735] {
            if (s.v[745] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(745, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[735] {s.store_sqrt(745, 745);s.store_add_scaled_product_indices(746, 744, 2.0, 743, 225, 1.0);s.store_scaled_sub(747, 746, 745, 0.5);s.store_div_ad(748, A::ln(A::div_scaled_product_by_product(s.ad_value(744), s.ad_value(744), 1.0, s.ad_value(743), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(744))));}
        s.b[749] = (s.v[747] < s.v[382]);s.store_scalar(749, if s.b[749] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[749]) {s.copy_ad(387, 747);}
        if (s.b[735] && (!s.b[749])) {s.store_offset_sub(44, 748, 747, (-0.0008));s.store_scale(45, 748, (4.0 * 0.0008));}
        if (s.b[735] && (!s.b[749])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[735] && (!s.b[749])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(387, 748, 1.0, 44, (-0.5), 45, (-0.5));}
        if s.b[735] {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_11(
        s: &mut Scratch,
    ) {
        let mut t31: usize = 0;
        while {
            let t30: f64 = if (s.b[735] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t30 != 0.0
        } {
            t31 += 1;assert!(t31 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[735] {s.copy_ad(750, 474);s.store_mul(751, 225, 387);s.store_exp_neg_input(752, 751);}
            s.b[758] = (s.v[387] > 1e-9);s.store_scalar(758, if s.b[758] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[758]) {s.store_exp_mul(753, 225, 387);s.store_mul_scaled_sqrt_ad_rhs(754, 750, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(752), s.ad_value(751)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(753), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(755, s.v[122], 754, A::add_scaled_sub_value_product(1.0, s.ad_value(752), 1.0, s.ad_value(239), s.ad_value(753), 1.0));}
            s.b[759] = (s.v[387] < (-1e-9));s.store_scalar(759, if s.b[759] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[758])) && s.b[759]) {s.store_mul_sqrt_mixed_ia(754, 750, A::offset(A::add(s.ad_value(752), s.ad_value(751)), (-1.0)));s.store_mul_scale_offset_mixed_ai(755, A::div_from_scalar(s.v[122], s.ad_value(754)), 752, -1.0, 1.0);}
            if ((s.b[735] && (!s.b[758])) && (!s.b[759])) {s.store_mul_ad_affine_product_lhs(754, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 387);s.store_scaled_sqrt_scaled_input(755, 225, s.v[122], -1.0);}
            if s.b[735] {s.store_sqrt_add_scaled_square_product(45, 754, 1.0, 741, 741, 4.0);s.store_offset_scaled_div(757, 754, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(756, 754, 0.5, 45, 0.5, 741, 1e-10);}
            s.b[760] = (s.v[756] < 0.0);s.store_scalar(760, if s.b[760] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[760]) {s.store_scalar(756, 0.0);s.store_scalar(757, 0.0);}
            if s.b[735] {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 756, (-1.0), 742, -1.0);s.store_scaled_mul(45, 341, 742, (-4.0));}
            if s.b[735] {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if s.b[735] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(756, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(757, 757, 755, 335);s.store_div_scaled_inputs_mixed_ai(390, A::square(s.ad_value(756)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(391, 390, 757, 2.0, 756, 1.0);s.store_sub_mixed_ia(756, 387, A::div_scaled_inputs4(s.ad_value(754), 1.0 / (s.v[93]), s.ad_value(387), (-1.0), s.ad_value(475), -1.0, s.ad_value(390), 1.0, A::add(A::scale_offset(s.ad_value(755), 1.0 / (s.v[93]), (-1.0)), s.ad_value(391)), 1.0));}
            s.b[761] = ((((s.v[756] - s.v[387])) as f64).abs() < 5e-12);s.store_scalar(761, if s.b[761] { 1.0 } else { 0.0 });
            if (s.b[735] && s.b[761]) {s.store_scalar(167, s.v[57]);}
            if s.b[735] {s.copy_ad(387, 756);}
            let (t2f,) = {
    if s.b[735] {
        (s.v[754],)
    } else {
        (s.v[386],)
    }
};
            s.store_scalar(386, t2f);
            if s.b[735] {s.store_primal_offset(167, 167, 1.0);}
        }
        if s.b[735] {s.copy_ad(388, 390);s.store_sqrt_div_scaled_inputs(763, 388, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[768] = (s.v[763] > (0.99 * s.v[738]));s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[768]) {s.store_div_from_scalar(762, 1.0, 323);s.store_scale(763, 738, 9662367879.197212);s.store_scalar(764, (1.0 / s.v[93]));s.store_div_from_scalar_ad(765, 1.0, A::add_scaled_inputs3(s.ad_value(762), 1.0, s.ad_value(763), 1.0, s.ad_value(764), 1.0));s.store_sub_from_scalar_scaled_mul(766, 1.0, 765, 762, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_12(
        s: &mut Scratch,
    ) {
        if (s.b[735] && s.b[768]) {s.store_mul_ad_product_rhs_mixed_ia(767, 762, 765, A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(764), 1.0, s.ad_value(763), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));s.store_div(383, 767, 766);}
        let (t33,) = {
    if (s.b[735] && s.b[768]) {
        let t32: f64 = (s.v[160] + s.v[383]);
        (t32,)
    } else {
        (s.v[160],)
    }
};
        s.store_scalar(160, t33);
        if s.b[735] {s.store_scaled_mul(769, 155, 157, 0.5);s.store_scale(44, 769, (2.0 * 10.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(770, 0.1, 45);}
        s.b[771] = (s.v[770] < 5e-12);s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[771]) {s.store_scalar(770, 5e-12);}
        if s.b[735] {s.copy_ad(330, 770);s.store_add_scaled_inputs4_offset_indices(179, 158, 1.0, 330, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.store_mul_div_mixed_iia(404, 179, 403, A::mul(s.ad_value(739), s.ad_value(231)));}
        s.b[772] = ((s.v[404] < (s.v[738] * 7.0)) && ((s.v[738] * 7.0) >= 0.0));s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[772]) {s.store_sub_scaled_inputs(44, 738, 7.0, 404, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 738, 738, (7.0 * 7.0));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t34,) = {
    if (s.b[735] && s.b[772]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t34);
        let (t35,) = {
    if (s.b[735] && s.b[772]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t35);
        if (s.b[735] && s.b[772]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[773] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });s.b[774] = (2.0 == 1.0);s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        let (t36,) = {
    if (((s.b[735] && s.b[772]) && s.b[773]) && s.b[774]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t36);s.b[775] = (2.0 == 2.0);s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        let (t37,) = {
    if ((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && s.b[775]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t37);s.b[776] = (2.0 == 4.0);s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        let (t38,) = {
    if (((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && (!s.b[775])) && s.b[776]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t38);s.b[777] = (2.0 == 8.0);s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });
        let (t39,) = {
    if ((((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && (!s.b[775])) && (!s.b[776])) && s.b[777]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t39);
        let (t3a,) = {
    if ((s.b[735] && s.b[772]) && s.b[773]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3a);let mut t3e: usize = 0;
        while {
            let t3d: f64 = if (((s.b[735] && s.b[772]) && s.b[773]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t3d != 0.0
        } {
            t3e += 1;assert!(t3e <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[735] && s.b[772]) && s.b[773]) {s.store_sqrt(53, 53);}
            let (t3c,) = {
    if ((s.b[735] && s.b[772]) && s.b[773]) {
        let t3b: f64 = (s.v[54] + 1.0);
        (t3b,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t3c);
        }
        if ((s.b[735] && s.b[772]) && (!s.b[773])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[735] && s.b[772]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 738, 7.0, 0.0, 53);s.store_sub_scaled_inputs(405, 738, 7.0, 43, 1.0);}
        if (s.b[735] && (!s.b[772])) {s.copy_ad(405, 404);}
        s.b[778] = ((s.v[405] > (s.v[403] - s.v[738])) && (s.v[738] >= 0.0));s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[778]) {s.store_add_scaled_inputs3_indices(44, 405, 1.0, 403, (-1.0), 738, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 738, 738, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t3f,) = {
    if (s.b[735] && s.b[778]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3f);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_13(
        s: &mut Scratch,
    ) {
        let (t40,) = {
    if (s.b[735] && s.b[778]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t40);
        if (s.b[735] && s.b[778]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[779] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });s.b[780] = (2.0 == 1.0);s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });
        let (t41,) = {
    if (((s.b[735] && s.b[778]) && s.b[779]) && s.b[780]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t41);s.b[781] = (2.0 == 2.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        let (t42,) = {
    if ((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && s.b[781]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t42);s.b[782] = (2.0 == 4.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        let (t43,) = {
    if (((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && s.b[782]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t43);s.b[783] = (2.0 == 8.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        let (t44,) = {
    if ((((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && (!s.b[782])) && s.b[783]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t44);
        let (t45,) = {
    if ((s.b[735] && s.b[778]) && s.b[779]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t45);let mut t49: usize = 0;
        while {
            let t48: f64 = if (((s.b[735] && s.b[778]) && s.b[779]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t48 != 0.0
        } {
            t49 += 1;assert!(t49 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[735] && s.b[778]) && s.b[779]) {s.store_sqrt(53, 53);}
            let (t47,) = {
    if ((s.b[735] && s.b[778]) && s.b[779]) {
        let t46: f64 = (s.v[54] + 1.0);
        (t46,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t47);
        }
        if ((s.b[735] && s.b[778]) && (!s.b[779])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[735] && s.b[778]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 738, 53);s.store_add_scaled_inputs3_indices(405, 403, 1.0, 738, (-1.0), 43, 1.0);}
        if (s.b[735] && (!s.b[778])) {
        }
        if s.b[735] {s.store_mul_scale_offset_indices(369, 229, 405, -1.0, 0.0);}
        let (t4f,) = {
    if s.b[735] {
        let t4a: f64 = (-s.v[341]);let t4b: f64 = (t4a * s.v[738]);let t4c: f64 = (t4b / 2.0);let t4d: f64 = (t4c / 1.034943e-10);let t4e: f64 = (t4d + s.v[227]);
        (t4e,)
    } else {
        (s.v[384],)
    }
};
        s.store_scalar(384, t4f);
        let (t53,) = {
    if s.b[735] {
        let t50: f64 = (s.v[386] * s.v[738]);let t51: f64 = (t50 / 1.034943e-10);let t52: f64 = (s.v[384] - t51);
        (t52,)
    } else {
        (s.v[385],)
    }
};
        s.store_scalar(385, t53);s.b[784] = (s.v[144] >= 1.0);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[784]) {s.store_scalar(349, s.v[619]);s.store_scalar(350, s.v[620]);s.store_scalar(351, s.v[621]);}
        let (t55,) = {
    if (s.b[735] && s.b[784]) {
        let (t54,) = {
            if (s.v[349] < s.v[385]) {
                (1.0,)
            } else {
                (2.0,)
            }
        };
        (t54,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t55);
        if (s.b[735] && (!s.b[784])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if (s.b[735] && (!s.b[784])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (s.b[735] && (!s.b[784])) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul(181, 225, 376);}
        s.b[785] = (s.v[181] < 3.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[785]) {s.store_mul_sub_rhs(337, 225, 178, 156);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_14(
        s: &mut Scratch,
    ) {
        if ((s.b[735] && (!s.b[784])) && s.b[785]) {s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[786] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && s.b[786]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && (!s.b[786])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[735] && (!s.b[784])) {
            if (s.v[378] > 0.0) {
                s.store_sqrt_div_scaled_inputs(401, 378, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);
            } else {
                s.store_scalar(401, 0.0);
            }
        }
        s.b[787] = (s.v[401] < s.v[738]);s.store_scalar(787, if s.b[787] { 1.0 } else { 0.0 });
        let (t56,) = {
    if ((s.b[735] && (!s.b[784])) && s.b[787]) {
        (1.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t56);
        let (t57,) = {
    if ((s.b[735] && (!s.b[784])) && (!s.b[787])) {
        (2.0,)
    } else {
        (s.v[339],)
    }
};
        s.store_scalar(339, t57);s.b[788] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[788]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if ((s.b[735] && (!s.b[784])) && (!s.b[788])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_15(
        s: &mut Scratch,
    ) {
        if ((s.b[735] && (!s.b[784])) && (!s.b[788])) {s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        s.b[789] = ((s.v[178] - s.v[383]) > 0.0);s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);}
        s.b[790] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t58,) = {
    if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t58);
        let (t59,) = {
    if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t59);
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });s.b[792] = (2.0 == 1.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        let (t5a,) = {
    if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && s.b[792]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5a);s.b[793] = (2.0 == 2.0);s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });
        let (t5b,) = {
    if (((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && s.b[793]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5b);s.b[794] = (2.0 == 4.0);s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });
        let (t5c,) = {
    if ((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && s.b[794]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5c);s.b[795] = (2.0 == 8.0);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        let (t5d,) = {
    if (((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && (!s.b[794])) && s.b[795]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5d);
        let (t5e,) = {
    if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t5e);let mut t62: usize = 0;
        while {
            let t61: f64 = if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t61 != 0.0
        } {
            t62 += 1;assert!(t62 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {s.store_sqrt(53, 53);}
            let (t60,) = {
    if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {
        let t5f: f64 = (s.v[54] + 1.0);
        (t5f,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t60);
        }
        if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && (!s.b[791])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && (!s.b[790])) {s.copy_ad(378, 376);}
        if (s.b[735] && (!s.b[784])) {s.copy_ad(349, 378);s.copy_ad(163, 376);s.store_sub_mixed_ai(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(737), 0.5), 475);}
        s.b[796] = (s.v[328] < 0.0);s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
    }
}
