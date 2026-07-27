#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(100, (s.v[99] * 1000000.0));s.store_scalar(101, (p[1] / p[9]));s.store_scalar(102, p[60]);s.store_scalar(103, (if (s.v[56] < 1.0) { 0.0 } else { p[295] }));s.store_scalar(104, (if (s.v[56] < 1.0) { p[60] } else { p[61] }));s.b[634] = (p[43] == 0.0);s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if s.b[634] {s.store_scalar(105, (s.v[101] - (2.0 * s.v[102])));s.store_scalar(106, (s.v[101] - (2.0 * s.v[104])));}
        if (!s.b[634]) {s.store_scalar(105, ((s.v[101] - (p[18] * s.v[103])) - ((2.0 - p[18]) * s.v[102])));s.store_scalar(106, ((s.v[101] - (p[18] * s.v[103])) - ((2.0 - p[18]) * s.v[104])));}
        s.store_primal_scale(107, 105, p[9]);s.store_primal_scale(108, 106, p[9]);s.store_scalar(109, (s.v[101] * 1000000.0));s.store_scalar(110, (s.v[109] * s.v[100]));s.store_scalar(111, ((p[107] * (1.0 + (p[108] / ((s.v[100]) as f64).powf(p[111])))) * (1.0 + (p[109] / ((s.v[109]) as f64).powf(p[110])))));s.b[635] = (((s.v[56] > 3.0) && (s.v[59] < s.v[65])) && (p[72] > 0.0));s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if s.b[635] {s.store_scalar(59, s.v[65]);}
        s.store_primal_scale(112, 59, (1.0 + (p[74] / ((s.v[109]) as f64).powf(p[75]))));s.store_scalar(113, (2.0 / ((1.0 / (p[62] + (0.5 * s.v[96]))) + (1.0 / (p[63] + (0.5 * s.v[96]))))));s.store_scalar(114, (1.6021918e-19 / (1.3806226e-23 * s.v[81])));s.store_scalar(115, ((1.6021918e-19 * s.v[66]) * 1.034943e-10));s.store_scalar(116, (p[244] * ((s.v[100]) as f64).powf((-p[247]))));s.store_scalar(117, (p[251] * ((s.v[100]) as f64).powf((-p[252]))));s.store_scalar(118, (p[248] * (((s.v[100] + s.v[79])) as f64).powf((-p[249]))));s.store_scalar(119, (((((2.0 * 1.6021918e-19) * s.v[71]) * 1.034943e-10)) as f64).sqrt());s.store_scalar(120, (1.0 / (s.v[71] * s.v[71])));s.store_scalar(121, ((((1.0 + (1.0 / s.v[100]))) as f64).powf(p[91]) * p[89]));s.store_scalar(122, s.v[115]);s.store_scalar(123, p[68]);s.store_scalar(124, (s.v[99] + (p[76] / ((s.v[110]) as f64).powf(p[77]))));s.store_scalar(125, (p[78] / ((s.v[110]) as f64).powf(p[79])));s.store_scalar(126, ((p[149] * (1.0 + (p[150] / (((s.v[124] * 1000000.0)) as f64).powf(p[151])))) + (p[152] / ((s.v[109]) as f64).powf(p[153]))));s.store_scalar(127, (1.0 + (((s.v[100]) as f64).powf(p[192]) * p[193])));s.b[636] = (p[44] <= 0.0);s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if s.b[636] {s.store_scalar(129, (1.0 + (p[130] / ((s.v[109]) as f64).powf(p[131]))));s.store_scalar(130, (p[124] * (1.0 + (p[125] / ((s.v[100]) as f64).powf(p[126])))));s.store_scalar(131, (s.v[100] / (s.v[100] + p[123])));s.store_scalar(132, (p[117] * (1.0 + (p[119] / ((s.v[100]) as f64).powf(p[120])))));s.store_scalar(133, (p[118] * (1.0 + (p[121] / s.v[100]))));}
        if (!s.b[636]) {s.store_scalar(329, ((s.v[109]) as f64).powf(p[131]));s.store_div_scaled_value_offset_denominator(134, s.ad_value(329), (p[127] * (1.0 + (p[128] / ((s.v[100]) as f64).powf(p[129])))), s.ad_value(329), p[130], 1.0);s.store_scalar(130, (p[124] * (1.0 + (p[125] / ((s.v[100]) as f64).powf(p[126])))));s.store_scalar(131, (p[123] * (1.0 + (p[132] / ((s.v[100]) as f64).powf(p[133])))));s.store_scalar(132, (p[117] * (1.0 + (p[119] / ((s.v[100]) as f64).powf(p[120])))));s.store_scalar(133, (p[118] * (1.0 + (p[121] / s.v[100]))));}
        s.store_primal_scale(135, 108, (1000000.0 * (p[65] * 1.0 / (((s.v[100]) as f64).powf(p[66])))));s.store_scalar(136, (p[134] * (1.0 + (p[135] / ((s.v[100]) as f64).powf(p[136])))));s.b[637] = (p[44] <= 0.0);s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if s.b[637] {s.store_scalar(137, (p[127] * (1.0 + (p[128] / ((s.v[100]) as f64).powf(p[129])))));}
        s.store_scalar(138, (((((p[115] * s.v[100]) * p[114]) / ((p[115] * s.v[100]) + p[114])) + p[116]) + 1e-50));s.b[638] = (s.v[138] < 3.0);s.store_scalar(638, if s.b[638] { 1.0 } else { 0.0 });
        if s.b[638] {s.store_scalar(138, 3.0);}
        s.store_scalar(139, (p[50] * p[253]));s.b[564] = param_given[168];s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });s.b[565] = param_given[169];s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });s.b[566] = param_given[170];s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });s.b[525] = param_given[294];s.store_scalar(525, if s.b[525] { 1.0 } else { 0.0 });s.b[524] = param_given[293];s.store_scalar(524, if s.b[524] { 1.0 } else { 0.0 });s.b[529] = param_given[13];s.store_scalar(529, if s.b[529] { 1.0 } else { 0.0 });s.b[530] = param_given[14];s.store_scalar(530, if s.b[530] { 1.0 } else { 0.0 });s.b[527] = param_given[23];s.store_scalar(527, if s.b[527] { 1.0 } else { 0.0 });s.b[526] = param_given[22];s.store_scalar(526, if s.b[526] { 1.0 } else { 0.0 });s.b[539] = param_given[16];s.store_scalar(539, if s.b[539] { 1.0 } else { 0.0 });s.b[540] = (p[17] != 0.0);s.store_scalar(540, if s.b[540] { 1.0 } else { 0.0 });s.store_scalar(451, 1.0);s.store_scalar(142, 0.0);s.store_scalar(518, p[13]);s.store_scalar(519, p[14]);s.store_scalar(520, (p[16] + 273.15));s.store_primal_scale(542, 108, (s.v[451] * s.v[68]));s.b[639] = (((p[10] > 0.0) && (p[11] > 0.0)) && ((p[9] == 1.0) || ((p[9] > 1.0) && (p[12] > 0.0))));s.store_scalar(639, if s.b[639] { 1.0 } else { 0.0 });
        if s.b[639] {s.store_scalar(328, 0.0);s.store_scalar(562, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[639] && (s.v[562] < p[9])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[639] {s.store_add_scaled_inputs3_mixed_iaa(328, 328, 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p[12] + s.v[96]), (p[10] + (0.5 * s.v[96])))), 1.0, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(562), (p[12] + s.v[96]), (p[11] + (0.5 * s.v[96])))), 1.0);s.store_primal_offset(562, 562, 1.0);}
        }
        if s.b[639] {s.store_div_from_scalar(537, (2.0 * p[9]), 328);}
        if (!s.b[639]) {s.store_scalar(537, 0.0);}
        s.b[640] = (s.v[537] > 0.0);s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if s.b[640] {s.store_scalar(328, (1.0 / (1.0 + p[162])));s.store_powf_ad(329, A::div_from_scalar(p[161], s.ad_value(537)), p[163]);s.store_scalar(330, (((p[161] / s.v[113])) as f64).powf(p[163]));s.store_div_scaled_product_offset_denominator_mixed_iaa(538, 112, A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);}
        if (!s.b[640]) {s.copy_ad(538, 112);}
        s.store_scalar(329, ((1.0 + (p[199] / ((s.v[109]) as f64).powf(p[200]))) * (1.0 + (p[202] / ((s.v[100]) as f64).powf(p[203])))));s.store_scalar(330, (s.v[61] / s.v[65]));s.store_scalar(44, ((s.v[330] - s.v[329]) - 0.01));s.store_scalar(45, ((4.0 * s.v[330]) * 0.01));
        if (!(s.v[45] > 0.0)) {s.store_scalar(45, (-s.v[45]));}
        s.store_sqrt_offset_input(45, 45, (s.v[44] * s.v[44]));s.store_sub_from_scalar_ad(328, s.v[330], A::scaled_offset(s.ad_value(45), s.v[44], 0.5));s.store_scale(544, 328, s.v[65]);s.b[641] = (s.v[537] > 0.0);s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if s.b[641] {s.store_scalar(328, (1.0 / (1.0 + p[165])));s.store_powf_ad(329, A::div_from_scalar(p[164], s.ad_value(537)), p[166]);s.store_scalar(330, (((p[164] / s.v[113])) as f64).powf(p[166]));s.store_div_scaled_product_offset_denominator_mixed_iaa(544, 544, A::offset(A::mul(s.ad_value(328), s.ad_value(329)), 1.0), 1.0, A::mul(s.ad_value(328), s.ad_value(330)), 1.0, 1.0);}
        s.b[642] = ((s.v[99] > p[72]) || (p[72] <= 0.0));s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_add_scaled_inputs(536, 544, ((s.v[99] - p[72]) * 1.0 / (s.v[99])), 538, (p[72] * 1.0 / (s.v[99])));}
        if (!s.b[642]) {s.store_add_scaled_inputs3_indices(536, 538, 1.0, 538, ((p[72] - s.v[99]) * 1.0 / (p[72])), 544, (-((p[72] - s.v[99]) * 1.0 / (p[72]))));}
        s.store_scale(229, 536, 1.6021918e-19);s.store_scale(545, 229, 1.034943e-10);s.store_scale(546, 545, 2.0);s.b[643] = ((s.v[99] <= (2.0 * p[72])) && (p[72] > 0.0));s.store_scalar(643, if s.b[643] { 1.0 } else { 0.0 });
        if s.b[643] {s.store_add_scaled_inputs4_indices(593, 538, 2.0, 538, (-(s.v[99] * 1.0 / (p[72]))), 544, (-(-(s.v[99] * 1.0 / (p[72])))), 544, -1.0);s.store_ln_div(548, 593, 544);}
        if (!s.b[643]) {s.store_scalar(548, 0.0);}
        s.store_scaled_ln_scaled_input(232, 536, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));s.store_scaled_ln_scaled_input(236, 544, 1.0 / ((10400000000.0 / 1e-6)), (2.0 / 38.68283));s.store_sqrt_div_from_scalar_ad(549, ((2.0 * 1.034943e-10) / 1.6021918e-19), s.ad_value(536));s.store_scalar(328, ((1.0 + (p[194] / ((s.v[100]) as f64).powf(p[195]))) * (1.0 + (p[196] / ((s.v[110]) as f64).powf(p[197])))));s.store_scalar(44, ((((s.v[328] * s.v[328]) + ((4.0 * 0.001) * 0.001))) as f64).sqrt());s.store_scalar(550, ((0.5 * (s.v[328] + s.v[44])) + (1e-10 * 0.001)));s.b[644] = (s.v[550] < 0.0);s.store_scalar(644, if s.b[644] { 1.0 } else { 0.0 });
        if s.b[644] {s.store_scalar(550, 0.0);}
        s.b[647] = (p[261] == 1.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if s.b[647] {s.store_offset_scaled(327, 107, p[289], p[288]);}
        s.b[652] = (p[43] == 1.0);s.store_scalar(652, if s.b[652] { 1.0 } else { 0.0 });
        if (s.b[652] && (p[24] != 0.0)) {s.store_scalar(533, (if s.b[527] { p[23] } else { ((p[20] * p[9]) * p[19]) }));}
        if (s.b[652] && (p[24] != 0.0)) {s.store_scalar(534, (if s.b[526] { p[22] } else { ((p[21] * p[9]) * p[19]) }));}
        if (s.b[652] && (p[24] != 0.0)) {s.store_scalar(531, 0.0);s.store_scalar(532, 0.0);}
        s.b[653] = ((s.v[533] > 0.0) && s.b[525]);s.store_scalar(653, if s.b[653] { 1.0 } else { 0.0 });
        if ((s.b[652] && (p[24] != 0.0)) && s.b[653]) {s.store_primal_scale(531, 533, (-p[294]));}
        if ((s.b[652] && (p[24] != 0.0)) && (!s.b[653])) {s.store_scalar(531, 0.0);}
        s.b[654] = ((s.v[534] > 0.0) && s.b[524]);s.store_scalar(654, if s.b[654] { 1.0 } else { 0.0 });
        if ((s.b[652] && (p[24] != 0.0)) && s.b[654]) {s.store_primal_scale(532, 534, (-p[293]));s.store_scalar(534, 0.0);}
        if (s.b[652] && (p[24] == 0.0)) {s.store_scalar(534, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();let nv10 = ctx.node_voltage(nodes[10]);
        if (s.b[652] && (p[24] == 0.0)) {s.store_scalar(532, 0.0);s.store_scalar(533, 0.0);s.store_scalar(531, 0.0);}
        if s.b[652] {s.store_scalar(535, (if (p[19] > s.v[96]) { (0.5 * (p[19] - s.v[96])) } else { 0.0 }));}
        s.b[655] = (!s.b[529]);s.store_scalar(655, if s.b[655] { 1.0 } else { 0.0 });
        if (s.b[652] && s.b[655]) {s.copy_ad(518, 535);}
        s.b[656] = (!s.b[530]);s.store_scalar(656, if s.b[656] { 1.0 } else { 0.0 });
        if (s.b[652] && s.b[656]) {s.copy_ad(519, 535);}
        if s.b[652] {s.store_primal_add_scaled_inputs(286, 107, 1.0, 518, p[9]);s.store_primal_add_scaled_inputs(285, 107, 1.0, 519, p[9]);s.store_primal_add_scaled_inputs(288, 108, 1.0, 518, p[9]);s.store_primal_add_scaled_inputs(287, 108, 1.0, 519, p[9]);}
        if (!s.b[652]) {s.store_scalar(534, 0.0);s.store_scalar(532, 0.0);s.store_scalar(533, 0.0);s.store_scalar(531, 0.0);s.store_scalar(286, 0.0);s.store_scalar(285, 0.0);s.store_scalar(288, 0.0);s.store_scalar(287, 0.0);}
        s.store_scaled_voltage(571, ctx, nodes, Some(6), Some(7), p[50]);s.store_scaled_voltage(572, ctx, nodes, Some(11), Some(7), p[50]);s.store_scaled_voltage(570, ctx, nodes, Some(12), Some(7), p[50]);s.b[657] = (p[43] == 1.0);s.store_scalar(657, if s.b[657] { 1.0 } else { 0.0 });
        if s.b[657] {s.store_scaled_voltage(590, ctx, nodes, Some(12), Some(6), p[50]);s.store_scaled_voltage(591, ctx, nodes, Some(12), Some(7), p[50]);}
        if (s.b[657] && (s.v[85] != 0.0)) {s.store_scaled_voltage(580, ctx, nodes, Some(18), None, (1e-9 / 0.0001));s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));}
        if (s.b[657] && (s.v[85] == 0.0)) {s.store_scalar(580, 0.0);s.store_scalar(581, 0.0);}
        if (!s.b[657]) {s.store_scalar(590, 0.0);s.store_scalar(591, 0.0);}
        if ((!s.b[657]) && (s.v[85] != 0.0)) {s.store_scaled_voltage(584, ctx, nodes, Some(15), None, (1e-9 / 0.0001));s.store_scaled_voltage(585, ctx, nodes, Some(16), None, (1e-9 / 0.0001));s.store_scaled_voltage(581, ctx, nodes, Some(13), None, (1e-9 / 0.0001));}
        if ((!s.b[657]) && (s.v[85] == 0.0)) {s.store_scalar(584, 0.0);s.store_scalar(585, 0.0);s.store_scalar(581, 0.0);}
        if ((p[38] > 0.0) && (s.v[67] > 0.0)) {
            if (nv10 > 0.0) {
                s.store_voltage(20, ctx, nodes, Some(10), None);
            } else {
                s.store_scalar(20, 0.0);
            }
        } else {
            s.store_scalar(20, 0.0);
        }
        s.b[658] = (s.v[571] >= 0.0);s.store_scalar(658, if s.b[658] { 1.0 } else { 0.0 });
        if s.b[658] {s.store_scalar(613, 1.0);s.store_scalar(461, 1.0);s.store_scalar(462, 0.0);s.copy_ad(157, 571);s.copy_ad(158, 572);s.copy_ad(156, 570);}
        if (!s.b[658]) {s.store_scalar(613, (-1.0));s.store_scalar(461, 0.0);s.store_scalar(462, 1.0);s.store_neg(157, 571);s.store_sub(158, 572, 571);s.store_sub(156, 570, 571);}
        s.store_scalar(429, ctx_temp);
        if s.b[539] {s.store_scalar(429, s.v[520]);}
        if s.b[540] {s.store_offset(429, 429, p[17]);}
        s.store_add(429, 429, 20);s.store_offset(328, 429, (-s.v[81]));s.store_mul_scale_offset_indices(329, 328, 429, 1.0, s.v[81]);s.store_sub_scaled_inputs_mixed_ai(237, A::sub_from_scalar(s.v[87], A::scale(s.ad_value(328), p[53])), 1.0, 329, p[54]);s.store_div_from_scalar_scaled_input(225, 1.6021918e-19, 429, 1.3806226e-23);s.store_square(226, 225);s.store_div_from_scalar(227, 1.0, 225);s.store_scalar(661, (((p[254] * (1.0 + (p[98] / ((s.v[109]) as f64).powf(p[99])))) * (1.0 + (p[100] / ((s.v[100]) as f64).powf(p[101])))) * (1.0 + (p[102] / ((s.v[110]) as f64).powf(p[103])))));s.store_scalar(664, (1.0 / (1.0 + p[159])));s.store_scalar(665, (if (((p[158] / s.v[83]) == 0.0) && (p[160] == 0.0)) { 1.0 } else { (((p[158] / s.v[83])) as f64).powf(p[160]) }));s.store_scalar(662, (s.v[661] * (1.0 + (s.v[664] * s.v[665]))));s.store_powf_scaled_input(663, 429, 1.0 / (s.v[81]), p[112]);s.store_scale(543, 663, 1.0 / (s.v[662]));s.store_mul(433, 548, 227);s.store_scale(328, 429, 1.0 / (s.v[81]));s.store_div_scaled_inputs_mixed_ia(253, 550, s.v[73], A::sub(A::add_scaled_product(A::scale_offset(s.ad_value(328), 0.4, 1.8), 1.0, s.ad_value(328), s.ad_value(328), 0.1), A::scale_offset(s.ad_value(328), (-s.v[60]), s.v[60])), 1.0);s.store_sqrt(302, 237);s.store_mul(303, 237, 302);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul_ad(230, A::powf(A::scale(s.ad_value(429), 1.0 / (s.v[81])), 1.5), A::exp(A::offset(A::mul_scaled_lhs(s.ad_value(237), (-1.0 / (2.0)), s.ad_value(225)), ((s.v[87] / 2.0) * s.v[114]))), (10400000000.0 / 1e-6));s.store_scaled_sqrt(208, 227, s.v[119]);s.store_square(205, 208);s.store_scaled_square(209, 230, s.v[120]);s.store_scalar(441, (s.v[96] - (2.0 * p[56])));s.b[666] = (s.v[56] > 3.0);s.store_scalar(666, if s.b[666] { 1.0 } else { 0.0 });
        if s.b[666] {s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(536), s.ad_value(230)));}
        if (!s.b[666]) {s.store_mul_scaled_ln_ad_rhs(231, 227, 2.0, A::div(s.ad_value(544), s.ad_value(230)));}
        s.store_sqrt_mul_ad(228, A::div_from_scalar(1.034943e-10, s.ad_value(229)), s.ad_value(227));s.store_scaled_mul(238, 229, 228, 1.414213562373095);s.b[667] = (p[43] == 1.0);s.store_scalar(667, if s.b[667] { 1.0 } else { 0.0 });
        if s.b[667] {s.store_scalar(474, 0.0);s.store_scalar(239, 0.0);s.store_div(328, 230, 536);}
        if (!s.b[667]) {s.store_sqrt_scaled_input(474, 227, (2.0 * s.v[122]));s.store_scale(328, 230, 1.0 / (s.v[66]));s.store_square(239, 328);s.store_div(328, 230, 544);}
        s.store_square(379, 328);s.store_sqrt_scaled_input_ad(444, A::div_scalar_by_product(1.034943e-10, s.ad_value(229), s.ad_value(225), 1.0), 2.0);s.store_div_from_scalar(547, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544);s.store_sqrt_div_scaled_inputs(416, 231, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);s.b[672] = (p[43] == 1.0);s.store_scalar(672, if s.b[672] { 1.0 } else { 0.0 });
        if s.b[672] {s.store_scalar(141, 0.4);s.store_scalar(140, 0.8);}
        if (!s.b[672]) {s.store_scalar(141, 0.8);s.store_scalar(140, 1.2);}
        s.b[673] = (s.v[141] > (s.v[140] * 0.5));s.store_scalar(673, if s.b[673] { 1.0 } else { 0.0 });
        if s.b[673] {s.store_primal_scale(141, 140, 0.5);}
        s.b[674] = (s.v[156] > s.v[141]);s.store_scalar(674, if s.b[674] { 1.0 } else { 0.0 });
        if s.b[674] {s.store_sub(329, 156, 141);s.store_sub(330, 140, 141);s.store_square(49, 329);s.store_square(50, 330);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[675] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(675, if s.b[675] { 1.0 } else { 0.0 });s.b[676] = (4.0 == 1.0);s.store_scalar(676, if s.b[676] { 1.0 } else { 0.0 });
        if ((s.b[674] && s.b[675]) && s.b[676]) {s.store_scalar(55, 1.0);}
        s.b[677] = (4.0 == 2.0);s.store_scalar(677, if s.b[677] { 1.0 } else { 0.0 });
        if (((s.b[674] && s.b[675]) && (!s.b[676])) && s.b[677]) {s.store_scalar(55, 2.0);}
        s.b[678] = (4.0 == 4.0);s.store_scalar(678, if s.b[678] { 1.0 } else { 0.0 });
        if ((((s.b[674] && s.b[675]) && (!s.b[676])) && (!s.b[677])) && s.b[678]) {s.store_scalar(55, 3.0);}
        s.b[679] = (4.0 == 8.0);s.store_scalar(679, if s.b[679] { 1.0 } else { 0.0 });
        if (((((s.b[674] && s.b[675]) && (!s.b[676])) && (!s.b[677])) && (!s.b[678])) && s.b[679]) {s.store_scalar(55, 4.0);}
        if (s.b[674] && s.b[675]) {s.store_scalar(54, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((s.b[674] && s.b[675]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[674] && s.b[675]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
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
        s.copy_ad(157, 152);s.copy_ad(158, 153);s.copy_ad(156, 154);s.store_scalar(144, 0.0);s.store_scalar(619, 0.0);s.store_scalar(620, 0.0);s.store_scalar(621, 0.0);s.store_scalar(622, 0.0);s.store_scalar(623, 0.0);s.store_scalar(624, 0.0);s.store_scalar(425, 0.0);s.store_scalar(426, 0.0);s.store_scalar(427, 0.0);s.store_scalar(428, 0.0);s.store_scalar(167, 0.0);s.store_scalar(168, 0.0);s.store_scaled_mul(680, 155, 157, 0.5);s.store_scale(44, 680, (2.0 * 1.0 / (p[226])));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(175, p[226], 45);s.b[681] = (s.v[175] < 5e-12);s.store_scalar(681, if s.b[681] { 1.0 } else { 0.0 });
        if s.b[681] {s.store_scalar(175, 5e-12);}
        s.store_add(172, 156, 175);s.store_add_scaled_inputs(173, 157, 1.0, 175, 2.0);s.store_add(174, 158, 175);s.b[682] = (p[43] == 1.0);s.store_scalar(682, if s.b[682] { 1.0 } else { 0.0 });
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
        s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(194, 44, 0.5, 45, 0.5, 0.1);s.store_div(683, 157, 194);s.copy_ad(44, 683);s.store_square(45, 44);s.store_mul(46, 45, 44);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_square(47, 45);s.store_div_from_scalar_ad(686, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(327, A::add_scaled_inputs3_offset(s.ad_value(44), 2.0, s.ad_value(45), 3.0, s.ad_value(46), 4.0, 1.0), s.ad_value(686), -1.0, 0.0, 686);s.store_sub_from_scalar(686, 1.0, 686);s.store_neg(327, 327);s.store_square(326, 686);s.b[694] = (((p[204] == 0.0) && (p[206] == 0.0)) || (p[205] == 0.0));s.store_scalar(694, if s.b[694] { 1.0 } else { 0.0 });
        if s.b[694] {s.store_scalar(148, 0.0);}
        if (!s.b[694]) {s.store_scalar(148, 1.0);}
        s.store_sqrt_mul_scaled_lhs(688, 229, (2.0 * 1.034943e-10), 232);s.store_add_scaled_inputs_mixed_ai(325, A::offset(s.ad_value(232), s.v[123]), 1.0, 688, 1.0 / (s.v[91]));s.b[695] = (s.v[148] == 0.0);s.store_scalar(695, if s.b[695] { 1.0 } else { 0.0 });
        if s.b[695] {s.store_scalar(321, s.v[88]);s.store_scalar(323, s.v[91]);s.store_scalar(324, s.v[92]);s.store_scaled_mul(434, 238, 238, (s.v[92] * s.v[92]));}
        if (!s.b[695]) {s.store_add_scaled_inputs3_offset_indices(692, 158, 1.0, 513, (-1.0), 325, -1.0, p[205]);s.store_sqrt_square_offset(44, 692, ((4.0 * 0.0001) * 0.0001));s.store_offset_add_scaled_inputs_indices(688, 692, 0.5, 44, 0.5, (1e-10 * 0.0001));}
        s.b[696] = (s.v[688] < 0.0);s.store_scalar(696, if s.b[696] { 1.0 } else { 0.0 });
        if ((!s.b[695]) && s.b[696]) {s.store_scalar(688, 0.0);}
        if (!s.b[695]) {s.store_div_from_scalar(689, 1.0, 688);s.store_scaled_abs(691, 325, 2.0);s.store_offset_sub_from_scalar_ad(693, s.v[123], s.ad_value(325), p[205]);}
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
        if (!s.b[695]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_mixed_aii(688, A::div_from_scalar(1.0, s.ad_value(690)), 1.0, 44, (-0.5), 45, (-0.5));s.store_offset_scaled(322, 688, p[204], p[206]);}
        s.b[697] = ((s.v[322] * 1000000000000.0) < s.v[88]);s.store_scalar(697, if s.b[697] { 1.0 } else { 0.0 });
        if ((!s.b[695]) && s.b[697]) {s.store_scalar(322, 0.0);s.store_scalar(148, 0.0);}
        if (!s.b[695]) {s.store_offset(321, 322, s.v[88]);s.store_div_from_scalar(323, 3.453133e-11, 321);s.store_scale(324, 321, 28959208927.08158);s.store_mul_ad_product_lhs_mixed_ai(434, A::square(s.ad_value(238)), 324, 324);}
        s.b[698] = ((p[43] == 1.0) || (s.v[56] < 3.0));s.store_scalar(698, if s.b[698] { 1.0 } else { 0.0 });
        if s.b[698] {s.store_offset_sub_from_scalar_ad(44, 0.5, s.ad_value(514), (-0.001));s.store_scalar(45, ((4.0 * 0.5) * 0.001));}
        if s.b[698] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[698] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(435, 44, (-0.5), 45, (-0.5), 0.5);s.store_add_scaled_inputs3_indices(440, 229, (((-p[237]) * p[237]) * 1.0 / ((2.0 * 1.034943e-10))), 231, 1.0, 227, -1.0);s.store_offset_sub(44, 435, 440, (-0.001));s.store_scale(45, 440, (4.0 * 0.001));}
        if s.b[698] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[698] {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(435, 440, 1.0, 44, 0.5, 45, 0.5);}
        s.b[699] = (s.v[56] > 2.0);s.store_scalar(699, if s.b[699] { 1.0 } else { 0.0 });
        if (s.b[698] && s.b[699]) {s.store_offset_sub(44, 232, 435, (-0.001));s.store_scale(45, 232, (4.0 * 0.001));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[698] && s.b[699]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[698] && s.b[699]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(435, 232, 1.0, 44, (-0.5), 45, (-0.5));}
        if (!s.b[698]) {s.store_scalar(435, 0.0);}
        s.b[700] = (s.v[56] < 3.0);s.store_scalar(700, if s.b[700] { 1.0 } else { 0.0 });
        if s.b[700] {s.store_scalar(184, p[237]);}
        if (!s.b[700]) {s.store_div_from_scalar(328, (2.0 * 1.034943e-10), 229);s.store_sqrt_mul_sub_rhs(184, 328, 232, 435);}
        if (s.v[56] < 3.0) {
            s.store_sqrt_mul(245, 546, 232);
        } else {
            s.store_sqrt_mul_sub_rhs(245, 546, 232, 435);
        }
        s.store_add_mixed_ai(318, A::add_scaled_product(A::offset(s.ad_value(232), s.v[123]), 1.0, s.ad_value(245), s.ad_value(324), 1.0), 433);s.copy_ad(233, 232);s.store_scalar(702, 0.95);s.store_offset_sub_scaled_inputs_indices(701, 233, s.v[702], 435, 1.0, (-0.001));s.store_sqrt_add_scaled_square_input(703, 701, 1.0, 233, ((4.0 * s.v[702]) * 0.001));s.store_add_scaled_inputs3_indices(704, 233, s.v[702], 701, (-0.5), 703, (-0.5));s.store_sub(234, 233, 704);s.store_sqrt(235, 234);s.b[712] = (p[72] != 0.0);s.store_scalar(712, if s.b[712] { 1.0 } else { 0.0 });
        if s.b[712] {s.store_scale(706, 544, ((2.0 * 1.6021918e-19) * 1.034943e-10));}
        if s.b[712] {
            if (s.v[56] < 3.0) {
                s.store_sqrt_mul(707, 706, 236);
            } else {
                s.store_sqrt_mul_sub_rhs(707, 706, 236, 435);
            }
        }
        if s.b[712] {s.store_add_scaled_product_mixed_aii(183, A::offset(s.ad_value(236), s.v[123]), 1.0, 707, 324, 1.0);s.store_scale(706, 324, 1.034943e-10);s.store_scalar(709, (1.0 / (p[72] * p[72])));s.store_scaled_mul(708, 184, 709, 2.0);s.store_mul_ad_product_rhs_mixed_ia(710, 706, 708, A::sub_from_scalar(p[69], s.ad_value(233)));s.copy_ad(711, 710);s.store_sub(706, 318, 183);s.store_scalar(705, (s.v[78] / p[72]));s.store_offset_mul(707, 705, 234, p[80]);s.store_scalar(710, s.v[77]);s.store_add_scaled_product_indices(708, 707, 1.0, 710, 173, 1.0);s.store_mul3_lhs(319, 706, 711, 708);}
        if (!s.b[712]) {s.store_scalar(319, 0.0);}
        s.store_scale(713, 184, (1.034943e-10 * 2.0));s.store_mul(714, 324, 713);s.store_sub_from_scalar(715, p[69], 233);s.store_scalar(716, (s.v[99] - p[71]));s.store_scalar(717, (1.0 / (s.v[716] * s.v[716])));s.store_scaled_mul(719, 714, 715, s.v[717]);s.store_scalar(714, (s.v[76] / s.v[99]));s.store_offset_scaled(717, 234, s.v[714], p[83]);s.store_add_scaled_inputs(718, 717, 1.0, 173, s.v[75]);s.store_mul(187, 719, 718);s.b[723] = (p[86] > 0.0);s.store_scalar(723, if s.b[723] { 1.0 } else { 0.0 });
        if s.b[723] {s.store_add_scaled_inputs3_offset_indices(720, 237, 1.0, 231, 1.0, 173, p[87], (-(2.0 * p[88])));s.store_scalar(721, ((s.v[99] * 0.5) + s.v[74]));s.store_primal_div_from_scalar(722, (p[86] * p[237]), 721);s.store_mul(188, 720, 722);}
        if (!s.b[723]) {s.store_scalar(188, 0.0);}
        s.copy_ad(724, 324);s.store_div_from_scalar_add_ad(725, 1.0, s.ad_value(323), A::div_from_scalar(s.v[72], s.ad_value(105)));s.store_sub(726, 724, 725);s.store_offset_mul(189, 245, 726, (p[105] / s.v[109]));s.store_add_scaled_inputs4_offset_indices(185, 187, 1.0, 319, 1.0, 189, 1.0, 188, 1.0, s.v[125]);s.store_sub(182, 318, 185);s.b[730] = (p[89] == 0.0);s.store_scalar(730, if s.b[730] { 1.0 } else { 0.0 });
        if s.b[730] {s.store_scalar(147, 0.0);}
        if (!s.b[730]) {s.store_scalar(147, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[731] = (s.v[147] == 0.0);s.store_scalar(731, if s.b[731] { 1.0 } else { 0.0 });
        if s.b[731] {s.store_scalar(320, 0.0);}
        if (!s.b[731]) {s.copy_ad(727, 174);s.store_scalar(728, s.v[121]);s.store_offset(729, 727, (-p[90]));}
        s.b[732] = (s.v[729] < (-3.0));s.store_scalar(732, if s.b[732] { 1.0 } else { 0.0 });
        if ((!s.b[731]) && s.b[732]) {s.store_scalar(320, 0.0);}
        s.b[733] = (s.v[729] < 0.0);s.store_scalar(733, if s.b[733] { 1.0 } else { 0.0 });
        if (((!s.b[731]) && (!s.b[732])) && s.b[733]) {s.store_offset_mul_offset_rhs_mixed_ia(320, 729, A::mul(s.ad_value(729), A::scale_offset(s.ad_value(729), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if (((!s.b[731]) && (!s.b[732])) && (!s.b[733])) {s.store_offset_mul_offset_rhs_mixed_ia(320, 729, A::mul_offset_rhs(s.ad_value(729), A::mul(s.ad_value(729), A::scale_offset(s.ad_value(729), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if (!s.b[731]) {s.store_sqrt_offset_square_offset(44, 320, (-1.0), ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_mixed_ai(320, A::offset(s.ad_value(320), (-1.0)), 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[734] = (s.v[320] < 0.0);s.store_scalar(734, if s.b[734] { 1.0 } else { 0.0 });
        if ((!s.b[731]) && s.b[734]) {s.store_scalar(320, 0.0);}
        if (!s.b[731]) {s.store_mul(320, 320, 728);s.store_offset_sub_from_scalar_ad(44, 1.0, s.ad_value(320), (-0.05));s.store_scalar(45, (4.0 * 0.05));}
        if (!s.b[731]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (!s.b[731]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(320, 44, (-0.5), 45, (-0.5), 1.0);}
        s.store_add_scaled_inputs3_offset_indices(159, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.copy_ad(178, 159);s.store_ln_scaled_input(328, 544, 1.0 / (s.v[66]));s.store_mul(342, 227, 328);s.store_add_mixed_ai(160, A::sub_from_scalar(s.v[123], s.ad_value(185)), 320);s.store_mul(240, 238, 324);s.store_square(241, 240);s.b[735] = (p[43] == 0.0);s.store_scalar(735, if s.b[735] { 1.0 } else { 0.0 });
        if s.b[735] {s.store_scalar(740, 7.0);s.store_offset(399, 231, 1.0);s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::offset(s.ad_value(399), (-s.v[383])), A::offset(s.ad_value(399), (-s.v[383])));s.store_add_mixed_ia(330, 225, A::div_scalar_offset_denominator(2.0, s.ad_value(399), (-s.v[383]), 1.0));s.store_div_ln_lhs(180, 329, 330);s.store_sqrt_mul(403, 547, 180);}
        if s.b[735] {
            if (s.v[403] > p[237]) {
                s.store_scalar(403, p[237]);
            } else {
            }
        }
        if s.b[735] {s.store_scaled_mul(406, 544, 403, (-1.6021918e-19));s.store_scalar(738, p[237]);s.store_scaled_mul(341, 544, 738, (-1.6021918e-19));s.store_scalar(739, 1.5);s.store_primal_div_from_scalar(736, 1.034943e-10, 738);s.store_primal_div_from_scalar(737, 1.0, 736);s.store_scale(741, 341, (-0.001));s.store_scale(742, 341, (-1e-5));}
        if (s.b[735] && (p[39] != 0.0)) {s.store_add(475, 172, 342);}
        if (s.b[735] && (p[39] == 0.0)) {s.store_add(475, 156, 342);}
        if s.b[735] {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(382, 2.0, 225, A::ln(A::div_from_scalar(s.v[66], s.ad_value(230))));s.store_scaled_square(743, 474, (s.v[95] * s.v[95]));s.store_neg(744, 475);s.store_add_scaled_inputs3_mixed_aai(745, A::square(A::add_scaled_product(s.ad_value(744), 2.0, s.ad_value(743), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(744)), (-4.0), 743, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
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
    pub(super) fn stamp_reactive_block_11(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (s.b[735] && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
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
            if s.b[735] {s.copy_ad(387, 756);s.copy_ad(386, 754);s.store_primal_offset(167, 167, 1.0);}
        }
        if s.b[735] {s.copy_ad(388, 390);s.store_sqrt_div_scaled_inputs(763, 388, ((2.0 * 1.034943e-10) / 1.6021918e-19), 544, 1.0);}
        s.b[768] = (s.v[763] > (0.99 * s.v[738]));s.store_scalar(768, if s.b[768] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[768]) {s.store_div_from_scalar(762, 1.0, 323);s.store_scale(763, 738, 9662367879.197212);s.store_scalar(764, (1.0 / s.v[93]));s.store_div_from_scalar_ad(765, 1.0, A::add_scaled_inputs3(s.ad_value(762), 1.0, s.ad_value(763), 1.0, s.ad_value(764), 1.0));s.store_sub_from_scalar_scaled_mul(766, 1.0, 765, 762, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (s.b[735] && s.b[768]) {s.store_mul_ad_product_rhs_mixed_ia(767, 762, 765, A::sub(A::mul_scaled_rhs(A::add_scaled_inputs(s.ad_value(764), 1.0, s.ad_value(763), 0.5), s.ad_value(341), -1.0), s.ad_value(475)));s.store_div(383, 767, 766);s.store_add(160, 160, 383);}
        if s.b[735] {s.store_scaled_mul(769, 155, 157, 0.5);s.store_scale(44, 769, (2.0 * 10.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(770, 0.1, 45);}
        s.b[771] = (s.v[770] < 5e-12);s.store_scalar(771, if s.b[771] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[771]) {s.store_scalar(770, 5e-12);}
        if s.b[735] {s.copy_ad(330, 770);s.store_add_scaled_inputs4_offset_indices(179, 158, 1.0, 330, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]));s.store_mul_div_mixed_iia(404, 179, 403, A::mul(s.ad_value(739), s.ad_value(231)));}
        s.b[772] = ((s.v[404] < (s.v[738] * 7.0)) && ((s.v[738] * 7.0) >= 0.0));s.store_scalar(772, if s.b[772] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[772]) {s.store_sub_scaled_inputs(44, 738, 7.0, 404, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 738, 738, (7.0 * 7.0));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[773] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(773, if s.b[773] { 1.0 } else { 0.0 });s.b[774] = (2.0 == 1.0);s.store_scalar(774, if s.b[774] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[772]) && s.b[773]) && s.b[774]) {s.store_scalar(55, 1.0);}
        s.b[775] = (2.0 == 2.0);s.store_scalar(775, if s.b[775] { 1.0 } else { 0.0 });
        if ((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && s.b[775]) {s.store_scalar(55, 2.0);}
        s.b[776] = (2.0 == 4.0);s.store_scalar(776, if s.b[776] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && (!s.b[775])) && s.b[776]) {s.store_scalar(55, 3.0);}
        s.b[777] = (2.0 == 8.0);s.store_scalar(777, if s.b[777] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && s.b[772]) && s.b[773]) && (!s.b[774])) && (!s.b[775])) && (!s.b[776])) && s.b[777]) {s.store_scalar(55, 4.0);}
        if ((s.b[735] && s.b[772]) && s.b[773]) {s.store_scalar(54, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((s.b[735] && s.b[772]) && s.b[773]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[735] && s.b[772]) && s.b[773]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[735] && s.b[772]) && (!s.b[773])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[735] && s.b[772]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 738, 7.0, 0.0, 53);s.store_sub_scaled_inputs(405, 738, 7.0, 43, 1.0);}
        if (s.b[735] && (!s.b[772])) {s.copy_ad(405, 404);}
        s.b[778] = ((s.v[405] > (s.v[403] - s.v[738])) && (s.v[738] >= 0.0));s.store_scalar(778, if s.b[778] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[778]) {s.store_add_scaled_inputs3_indices(44, 405, 1.0, 403, (-1.0), 738, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 738, 738, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[779] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(779, if s.b[779] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[780] = (2.0 == 1.0);s.store_scalar(780, if s.b[780] { 1.0 } else { 0.0 });
        if (((s.b[735] && s.b[778]) && s.b[779]) && s.b[780]) {s.store_scalar(55, 1.0);}
        s.b[781] = (2.0 == 2.0);s.store_scalar(781, if s.b[781] { 1.0 } else { 0.0 });
        if ((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && s.b[781]) {s.store_scalar(55, 2.0);}
        s.b[782] = (2.0 == 4.0);s.store_scalar(782, if s.b[782] { 1.0 } else { 0.0 });
        if (((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && s.b[782]) {s.store_scalar(55, 3.0);}
        s.b[783] = (2.0 == 8.0);s.store_scalar(783, if s.b[783] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && s.b[778]) && s.b[779]) && (!s.b[780])) && (!s.b[781])) && (!s.b[782])) && s.b[783]) {s.store_scalar(55, 4.0);}
        if ((s.b[735] && s.b[778]) && s.b[779]) {s.store_scalar(54, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if (((s.b[735] && s.b[778]) && s.b[779]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[735] && s.b[778]) && s.b[779]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[735] && s.b[778]) && (!s.b[779])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (s.b[735] && s.b[778]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 738, 53);s.store_add_scaled_inputs3_indices(405, 403, 1.0, 738, (-1.0), 43, 1.0);}
        if (s.b[735] && (!s.b[778])) {
        }
        if s.b[735] {s.store_mul_scale_offset_indices(369, 229, 405, -1.0, 0.0);s.store_add_scaled_product_indices(384, 227, 1.0, 341, 738, ((-0.5) * 9662367879.197212));s.store_add_scaled_product_indices(385, 384, 1.0, 386, 738, (-9662367879.197212));}
        s.b[784] = (s.v[144] >= 1.0);s.store_scalar(784, if s.b[784] { 1.0 } else { 0.0 });
        if (s.b[735] && s.b[784]) {s.store_scalar(349, s.v[619]);s.store_scalar(350, s.v[620]);s.store_scalar(351, s.v[621]);}
        if (s.b[735] && s.b[784]) {s.store_scalar(339, (if (s.v[349] < s.v[385]) { 1.0 } else { 2.0 }));}
        if (s.b[735] && (!s.b[784])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), s.ad_value(178)), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if (s.b[735] && (!s.b[784])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (s.b[735] && (!s.b[784])) {s.store_add_product3_rhs_mixed_iia(376, 178, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul(181, 225, 376);}
        s.b[785] = (s.v[181] < 3.0);s.store_scalar(785, if s.b[785] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[785]) {s.store_mul_sub_rhs(337, 225, 178, 156);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 156, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[786] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(786, if s.b[786] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && s.b[786]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[735] && (!s.b[784])) && (!s.b[785])) && s.b[786]) {s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
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
        if ((s.b[735] && (!s.b[784])) && s.b[787]) {s.store_scalar(339, 1.0);}
        if ((s.b[735] && (!s.b[784])) && (!s.b[787])) {s.store_scalar(339, 2.0);}
        s.b[788] = ((s.v[158] - s.v[383]) <= s.v[182]);s.store_scalar(788, if s.b[788] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[788]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        if ((s.b[735] && (!s.b[784])) && (!s.b[788])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 738, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(178), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 178, 331, 323);s.copy_ad(378, 376);}
        s.b[789] = ((s.v[178] - s.v[383]) > 0.0);s.store_scalar(789, if s.b[789] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(178), s.ad_value(383)), A::sub(s.ad_value(178), s.ad_value(383)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) {s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(178), s.ad_value(383))));s.store_div_ln_lhs(377, 329, 330);}
        s.b[790] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(790, if s.b[790] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[791] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(791, if s.b[791] { 1.0 } else { 0.0 });s.b[792] = (2.0 == 1.0);s.store_scalar(792, if s.b[792] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && s.b[792]) {s.store_scalar(55, 1.0);}
        s.b[793] = (2.0 == 2.0);s.store_scalar(793, if s.b[793] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && s.b[793]) {s.store_scalar(55, 2.0);}
        s.b[794] = (2.0 == 4.0);s.store_scalar(794, if s.b[794] { 1.0 } else { 0.0 });
        if ((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && s.b[794]) {s.store_scalar(55, 3.0);}
        s.b[795] = (2.0 == 8.0);s.store_scalar(795, if s.b[795] { 1.0 } else { 0.0 });
        if (((((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (!s.b[792])) && (!s.b[793])) && (!s.b[794])) && s.b[795]) {s.store_scalar(55, 4.0);}
        if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {s.store_scalar(54, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && s.b[791]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) && (!s.b[791])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && s.b[790]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if ((((s.b[735] && (!s.b[784])) && (!s.b[788])) && s.b[789]) && (!s.b[790])) {s.copy_ad(378, 376);}
        if (s.b[735] && (!s.b[784])) {s.copy_ad(349, 378);s.copy_ad(163, 376);s.store_sub_mixed_ai(328, A::add_scaled_product(s.ad_value(349), 1.0, s.ad_value(341), s.ad_value(737), 0.5), 475);}
        s.b[796] = (s.v[328] < 0.0);s.store_scalar(796, if s.b[796] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {s.store_mul_scale_offset_indices(329, 474, 737, 1.0, s.v[94]);s.store_square(329, 329);s.store_offset_scaled(332, 328, (-1.6), 0.6);s.store_scalar(331, 0.5);s.store_add_scaled_inputs3_indices(44, 332, 1.0, 331, (-1.0), 332, (-0.001));s.store_scaled_mul(45, 332, 332, (4.0 * 0.001));}
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((s.b[735] && (!s.b[784])) && s.b[796]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(331, 332, 1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(330, 329, 331, 226);s.store_div_ad(351, A::mul_sub_from_scalar_rhs(s.ad_value(328), 1.0, A::sqrt(s.ad_value(330))), A::sub_from_scalar(1.0, s.ad_value(330)));}
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_scaled_square(327, 474, (s.v[95] * s.v[95]));s.store_neg_ad(328, A::add_scaled_inputs_product(s.ad_value(475), 1.0, s.ad_value(349), (-1.0), s.ad_value(341), s.ad_value(738), (-(1.0 / (2.0) * 9662367879.197212))));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_add_scaled_inputs3_mixed_aai(329, A::square(A::add_scaled_product(s.ad_value(328), 2.0, s.ad_value(327), s.ad_value(225), 1.0)), 1.0, A::square(s.ad_value(328)), (-4.0), 327, (-4.0));}
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {
            if (s.v[329] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(329, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((s.b[735] && (!s.b[784])) && (!s.b[796])) {s.store_sqrt(329, 329);s.store_add_scaled_product_indices(330, 328, 2.0, 327, 225, 1.0);s.store_scaled_sub(380, 330, 329, 0.5);s.store_div_ad(381, A::ln(A::div_scaled_product_by_product(s.ad_value(328), s.ad_value(328), 1.0, s.ad_value(327), s.ad_value(239), 1.0)), A::add(s.ad_value(225), A::div_from_scalar(2.0, s.ad_value(328))));}
        s.b[797] = (s.v[380] < s.v[382]);s.store_scalar(797, if s.b[797] { 1.0 } else { 0.0 });
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && s.b[797]) {s.copy_ad(351, 380);}
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {s.store_offset_sub(44, 381, 380, (-0.0008));s.store_scale(45, 381, (4.0 * 0.0008));}
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[735] && (!s.b[784])) && (!s.b[796])) && (!s.b[797])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(351, 381, 1.0, 44, (-0.5), 45, (-0.5));}
        if (s.b[735] && (!s.b[784])) {s.store_scalar(167, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut td: usize = 0;
        while {
            let tc: f64 = if ((s.b[735] && (!s.b[784])) && (s.v[167] < s.v[57])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[735] && (!s.b[784])) {s.copy_ad(328, 474);s.store_mul(329, 225, 351);s.store_exp_neg_input(330, 329);}
            s.b[798] = (s.v[351] > 1e-9);s.store_scalar(798, if s.b[798] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[798]) {s.store_exp_mul(327, 225, 351);s.store_mul_scaled_sqrt_ad_rhs(331, 328, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)), 1.0, s.ad_value(239), s.ad_value(327), (-1.0), 1.0));s.store_mul_div_from_scalar_lhs_ad_mixed_ia(332, s.v[122], 331, A::add_scaled_sub_value_product(1.0, s.ad_value(330), 1.0, s.ad_value(239), s.ad_value(327), 1.0));}
            s.b[799] = (s.v[351] < (-1e-9));s.store_scalar(799, if s.b[799] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[784])) && (!s.b[798])) && s.b[799]) {s.store_mul_sqrt_mixed_ia(331, 328, A::offset(A::add(s.ad_value(330), s.ad_value(329)), (-1.0)));s.store_mul_scale_offset_mixed_ai(332, A::div_from_scalar(s.v[122], s.ad_value(331)), 330, -1.0, 1.0);}
            if (((s.b[735] && (!s.b[784])) && (!s.b[798])) && (!s.b[799])) {s.store_mul_ad_affine_product_lhs(331, A::sqrt(A::div_from_scalar(s.v[122], s.ad_value(225))), s.ad_value(225), -1.0, 0.0, 351);s.store_scaled_sqrt_scaled_input(332, 225, s.v[122], -1.0);}
            if (s.b[735] && (!s.b[784])) {s.store_sqrt_add_scaled_square_product(45, 331, 1.0, 741, 741, 4.0);s.store_offset_scaled_div(334, 331, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 331, 0.5, 45, 0.5, 741, 1e-10);}
            s.b[800] = (s.v[333] < 0.0);s.store_scalar(800, if s.b[800] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[800]) {s.store_scalar(333, 0.0);s.store_scalar(334, 0.0);}
            if (s.b[735] && (!s.b[784])) {s.store_add_scaled_inputs3_indices(44, 341, -1.0, 333, (-1.0), 742, -1.0);s.store_scaled_mul(45, 341, 742, (-4.0));}
            if (s.b[735] && (!s.b[784])) {
                if (s.v[45] > 0.0) {
                } else {
                    s.store_neg(45, 45);
                }
            }
            if (s.b[735] && (!s.b[784])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(335, 44, 45, 0.5, 0.5);s.store_add_scaled_inputs3_indices(333, 341, -1.0, 44, (-0.5), 45, (-0.5));s.store_mul3_lhs(334, 334, 332, 335);s.store_div_scaled_inputs_mixed_ai(388, A::square(s.ad_value(333)), ((0.5 * 9662367879.197212) * 6.241449993689894e18), 544, 1.0);s.store_div_scaled_product_indices(389, 388, 334, 2.0, 333, 1.0);s.store_sub_mixed_ia(333, 351, A::div_scaled_inputs3(A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(349), 1.0, s.ad_value(351), (-1.0), s.ad_value(331), 1.0 / (s.v[93])), 1.0, A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(341), 0.5), s.ad_value(738), 9662367879.197212), 1.0, s.ad_value(475), (-1.0), s.ad_value(388), 1.0, A::add(A::add_scaled_product(A::scale_offset(s.ad_value(332), 1.0 / (s.v[93]), (-1.0)), 1.0, s.ad_value(332), s.ad_value(738), 9662367879.197212), s.ad_value(389)), 1.0));s.copy_ad(334, 167);}
            s.b[801] = ((((s.v[333] - s.v[351])) as f64).abs() < 0.001);s.store_scalar(801, if s.b[801] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[784])) && s.b[801]) {s.store_scalar(167, s.v[57]);}
            if (s.b[735] && (!s.b[784])) {s.copy_ad(351, 333);s.copy_ad(357, 331);s.store_primal_offset(167, 167, 1.0);}
        }
        if (s.b[735] && (!s.b[784])) {s.store_add(351, 475, 351);s.store_add_scaled_product_mixed_iia(350, 349, 1.0, 737, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(357), 1.0), 1.0);}
    }
}
