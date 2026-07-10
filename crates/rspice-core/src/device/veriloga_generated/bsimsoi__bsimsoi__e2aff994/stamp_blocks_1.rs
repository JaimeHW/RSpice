#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.b[1545] = (s.v[452] <= p.p1347);s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((!s.b[1542]) && s.b[1545]) {s.store_scalar(452, p.p1347);}
        s.b[1546] = (s.v[453] <= p.p1347);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if ((!s.b[1542]) && s.b[1546]) {s.store_scalar(453, p.p1347);}
        s.b[1547] = (p.p33 == 1.0);s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });s.b[1548] = (s.v[773] <= 0.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1548]) {s.store_scalar(773, 0.0);}
        s.b[1549] = (s.v[772] <= 0.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1549]) {s.store_scalar(772, 0.0);}
        s.b[1550] = (s.v[775] <= 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1550]) {s.store_scalar(775, 0.0);}
        s.b[1551] = (s.v[774] <= 0.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if (s.b[1547] && s.b[1551]) {s.store_scalar(774, 0.0);}
        s.b[1552] = (s.v[777] <= 0.0);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if ((!s.b[1547]) && s.b[1552]) {s.store_scalar(777, 0.0);}
        s.b[1553] = (s.v[776] <= 0.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((!s.b[1547]) && s.b[1553]) {s.store_scalar(776, 0.0);}
        s.store_scalar(465, ((p.p900 * (p.p21 + ((s.v[189] / 3.0) / p.p22))) / ((p.p22 * p.p2) * (s.v[261] - p.p899))));s.b[1554] = (s.v[465] > 0.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if s.b[1554] {s.store_scalar(465, (1.0 / s.v[465]));}
        if (!s.b[1554]) {s.store_scalar(465, 1000.0);}
        s.store_scalar(167, (p.p76 * p.p76));s.store_scale(168, 822, p.p76);s.store_square(169, 168);s.store_scaled_limited_exp_scaled_input(492, 826, ((((p.p722 / p.p76)).max(1e-38)) as f64).ln(), 1.0 / (s.v[167]));s.store_div_mixed_ai(493, A::limited_exp(A::mul(s.ad_value(826), A::ln(A::max_with_scalar(A::div_from_scalar(p.p722, s.ad_value(168)), 1e-38)))), 169);s.store_scalar(487, (if (p.p30 == 1.0) { p.p703 } else { p.p702 }));s.store_scalar(488, (if (p.p30 == 1.0) { p.p705 } else { p.p704 }));s.store_scale(489, 493, (s.v[487] * ((s.v[183] / p.p1373) + p.p1378)));s.store_scale(490, 493, (s.v[487] * ((s.v[183] / p.p1373) + p.p1377)));s.store_primal_scale(491, 822, ((-s.v[488]) * p.p76));s.store_scale(487, 492, (s.v[487] * (((s.v[183] / p.p1373) * s.v[184]) + (p.p1381 / p.p2))));s.store_scalar(488, ((-s.v[488]) * p.p76));s.store_scalar(191, (p.p1101 + s.v[183]));s.b[1556] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });s.b[1559] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if s.b[1559] {s.store_scalar(1015, ((s.v[191] * p.p2) / p.p1099));s.store_scalar(1016, ((p.p1100 * s.v[191]) * p.p2));}
        if (!s.b[1559]) {s.store_scalar(1015, 1.0);s.store_scalar(1016, 0.0);}
        s.b[1560] = (p.p1028 <= (-273.15));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if s.b[1560] {s.store_scalar(167, (300.15 - 273.15));s.store_scalar(636, 300.15);}
        if (!s.b[1560]) {s.store_scalar(636, (p.p1028 + 273.15));}
        s.store_scalar(635, (ctx_temp + p.p23));s.b[1561] = ((p.p41 != 0.0) && (p.p1099 > 0.0));s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = ((p.p40 != 0.0) && (!true));s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = true;s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if ((s.b[1561] && s.b[1562]) && s.b[1563]) {s.store_voltage(634, ctx, nodes, Some(4), None);}
        if ((s.b[1561] && s.b[1562]) && (!s.b[1563])) {s.store_voltage(634, ctx, nodes, Some(5), None);}
        if (s.b[1561] && (!s.b[1562])) {s.store_voltage(634, ctx, nodes, Some(5), None);}
        if (!s.b[1561]) {s.store_scalar(634, 0.0);}
        s.store_offset(635, 634, s.v[635]);s.store_scale(271, 635, s.v[1048]);s.store_div_from_scalar(272, 1.0, 271);s.store_div(639, 635, 636);s.store_sub(640, 635, 636);s.store_scale(637, 635, s.v[1048]);s.store_primal_scale(638, 636, s.v[1048]);s.store_sub_from_scalar_ad(190, p.p108, A::div_scaled_product_offset_denominator(s.ad_value(635), s.ad_value(635), p.p1029, s.ad_value(635), p.p1030, 1.0));s.store_mul_div_scaled_inputs_mixed_aii(168, A::sqrt(A::div(s.ad_value(635), s.ad_value(636))), 635, 1.0, 636, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_scaled_limited_exp_ad_rhs(182, 168, p.p107, A::sub(A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(638), 2.0), A::div_scaled_inputs(s.ad_value(190), 1.0, s.ad_value(637), 2.0)));s.b[1564] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if s.b[1564] {s.store_ln_ad(167, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));s.store_sqrt_square_offset(251, 167, 1e-6);}
        if (!s.b[1564]) {s.store_ln_ad(251, A::max_with_scalar(A::div(s.ad_value(706), s.ad_value(182)), 1e-38));}
        s.b[1565] = (((p.p41 != 0.0) && (p.p1099 > 0.0)) && (s.v[191] > 0.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if s.b[1565] {s.store_ln_ad(167, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));s.store_sqrt_square_offset(942, 167, 1e-6);}
        if (!s.b[1565]) {s.store_ln_ad(942, A::max_with_scalar(A::div_scaled_product(s.ad_value(953), s.ad_value(705), 1.0, A::square(s.ad_value(182)), 1.0), 1e-38));}
        s.b[1566] = (s.v[704] > 0.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if s.b[1566] {s.store_offset_product3(219, s.ad_value(379), s.ad_value(271), A::ln(A::max_with_scalar(A::div(s.ad_value(704), s.ad_value(705)), 1e-38)), -1.0, p.p5);}
        if (!s.b[1566]) {s.store_scalar(219, 0.0);}
        s.store_max_with_scalar_ad(298, A::add(A::offset(A::mul(s.ad_value(271), s.ad_value(251)), 0.4), s.ad_value(729)), 0.4);s.store_sqrt(299, 298);s.store_sqrt_div_from_scalar_ad(277, (2.0 * s.v[180]), A::scale(s.ad_value(706), 1.602176462e-19));s.store_primal_sqrt_scaled_input(300, 782, ((s.v[180] / s.v[181]) * p.p76));s.store_mul_add_scaled_inputs_rhs(665, 720, A::scale_offset(s.ad_value(639), p.p1031, (((((-1.0)) * (p.p1031))) + (1.0))), 0.5, A::sqrt_square_offset(A::scale_offset(s.ad_value(639), p.p1031, (((((-1.0)) * (p.p1031))) + (1.0))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_scale_offset_rhs(663, 730, 639, p.p1059, (((((-1.0)) * (p.p1059))) + (1.0)));s.b[1577] = (p.p35 != 0.0);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if s.b[1577] {s.store_mul_scale_offset_rhs(664, 731, 639, p.p1059, (((((-1.0)) * (p.p1059))) + (1.0)));}
        s.store_scalar(338, (if (p.p30 != 1.0) { (0.3333333333333333 * p.p347) } else { (0.5 * p.p347) }));s.store_mul_pow_indices(641, 738, 639, 796);s.store_mul_add_scaled_inputs_rhs(643, 748, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_add_scaled_inputs_rhs(645, 758, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(647, 752, 639, 799);s.store_mul_pow_indices(649, 755, 639, 800);s.store_mul_add_scaled_inputs_rhs(651, 751, A::offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0), 0.5, A::sqrt_square_offset(A::offset(A::mul_offset_rhs(s.ad_value(805), s.ad_value(639), (-1.0)), 1.0), ((4.0 * 0.001) * 0.001)), 0.5);s.b[1578] = (p.p35 != 0.0);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if s.b[1578] {s.store_mul_pow_indices(642, 739, 639, 796);s.store_mul_add_scaled_inputs_rhs(644, 749, A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(797), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(646, 759, A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(798), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(648, 753, 639, 799);s.store_mul_pow_indices(650, 756, 639, 800);}
        s.store_pow_indices(652, 639, 801);s.store_mul_pow_mixed_iia(653, 741, 639, A::neg(s.ad_value(802)));s.b[1579] = (s.v[653] < 100.0);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if s.b[1579] {s.store_scalar(653, 100.0);}
        s.b[1580] = (p.p35 != 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if s.b[1580] {s.store_mul_pow_mixed_iia(654, 745, 639, A::neg(s.ad_value(802)));}
        s.b[1581] = (s.v[654] < 100.0);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1580] && s.b[1581]) {s.store_scalar(654, 100.0);}
        s.store_mul_pow_mixed_iia(655, 747, 639, A::neg(s.ad_value(802)));s.b[1582] = (s.v[655] < 100.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.store_scalar(655, 100.0);}
        s.store_div_from_scalar_offset_ad(656, 1.0, A::add_scaled_inputs(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p.p1069, 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(A::div_from_scalar(1.0, s.ad_value(737)), A::scale_offset(s.ad_value(640), p.p1069, 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5), 2.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_add_scaled_inputs_rhs(657, 778, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5);s.b[1583] = (p.p35 != 0.0);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if s.b[1583] {s.store_mul_add_scaled_inputs_rhs(658, 779, A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(1.0, A::mul(s.ad_value(803), s.ad_value(640))), (-1e-6)), ((4.0 * 0.001) * 0.001)), 0.5);}
        s.store_mul_add_scaled_inputs_rhs(330, 328, A::offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(329), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(333, 331, A::offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(332), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_pow_indices(659, 866, 639, 804);s.store_add_scaled_offset_product_rhs(660, 893, 1.0, 900, 639, (-1.0), 1.0);s.store_add_scaled_offset_product_rhs(661, 897, 1.0, 901, 639, (-1.0), 1.0);s.store_limited_exp_ad(662, A::mul(s.ad_value(827), A::ln(A::max_with_scalar(s.ad_value(639), 1e-38))));s.store_mul_add_scaled_inputs_rhs(832, 828, A::offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(830), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(833, 829, A::offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(831), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_mul_add_scaled_inputs_rhs(858, 856, A::offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(857), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(861, 859, A::offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(860), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_mul_add_scaled_inputs_rhs(864, 862, A::offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(863), s.ad_value(640)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5);s.store_scaled_add_sqrt_square_offset_ad(666, A::scale_offset(s.ad_value(640), p.p1093, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p901));s.store_scaled_add_sqrt_square_offset_ad(669, A::scale_offset(s.ad_value(640), p.p1093, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p902));s.store_scaled_add_sqrt_square_offset_ad(667, A::scale_offset(s.ad_value(640), p.p1094, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p903));s.store_scaled_add_sqrt_square_offset_ad(670, A::scale_offset(s.ad_value(640), p.p1094, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p904));s.store_scaled_add_sqrt_square_offset_ad(671, A::scale_offset(s.ad_value(640), p.p1095, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p905));s.store_scaled_add_sqrt_square_offset_ad(668, A::scale_offset(s.ad_value(640), p.p1095, ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), (0.5 * p.p906));s.store_offset_add_scaled_inputs(672, A::offset(A::sub_from_scalar(p.p907, A::scale(s.ad_value(640), p.p1096)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p907, A::scale(s.ad_value(640), p.p1096)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_add_scaled_inputs(675, A::offset(A::sub_from_scalar(p.p908, A::scale(s.ad_value(640), p.p1096)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p908, A::scale(s.ad_value(640), p.p1096)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);s.store_offset_add_scaled_inputs(673, A::offset(A::sub_from_scalar(p.p909, A::scale(s.ad_value(640), p.p1097)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p909, A::scale(s.ad_value(640), p.p1097)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);s.store_offset_add_scaled_inputs(676, A::offset(A::sub_from_scalar(p.p910, A::scale(s.ad_value(640), p.p1097)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p910, A::scale(s.ad_value(640), p.p1097)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);s.store_offset_add_scaled_inputs(674, A::offset(A::sub_from_scalar(p.p911, A::scale(s.ad_value(640), p.p1098)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p911, A::scale(s.ad_value(640), p.p1098)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);s.store_offset_add_scaled_inputs(677, A::offset(A::sub_from_scalar(p.p912, A::scale(s.ad_value(640), p.p1098)), (-0.01)), 0.5, A::sqrt_square_offset(A::offset(A::sub_from_scalar(p.p912, A::scale(s.ad_value(640), p.p1098)), (-0.01)), ((4.0 * 0.001) * 0.001)), 0.5, 0.01);s.b[1584] = (p.p8 < 9.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });s.b[1585] = ((p.p2 % 2.0) != 0.0);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if (s.b[1584] && s.b[1585]) {s.store_scalar(969, 1.0);s.store_scalar(971, 1.0);s.store_scalar(968, (2.0 * (((p.p2 - 1.0) / 2.0)).max(0.0)));s.copy_ad(970, 968);}
        s.b[1586] = (p.p6 == 1.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if ((s.b[1584] && (!s.b[1585])) && s.b[1586]) {s.store_scalar(969, 2.0);s.store_scalar(968, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));s.store_scalar(971, 0.0);s.store_scalar(970, p.p2);}
        if ((s.b[1584] && (!s.b[1585])) && (!s.b[1586])) {s.store_scalar(969, 0.0);s.store_scalar(968, p.p2);s.store_scalar(971, 2.0);s.store_scalar(970, (2.0 * (((p.p2 / 2.0) - 1.0)).max(0.0)));}
        s.store_scalar(167, (s.v[449] + s.v[451]));s.store_scalar(168, (s.v[449] + s.v[449]));s.store_scalar(169, (s.v[450] + s.v[450]));s.store_scalar(155, ((s.v[167] + s.v[167]) + s.v[189]));s.store_scalar(156, ((s.v[167] + s.v[167]) + s.v[189]));s.store_scalar(157, s.v[168]);s.store_scalar(158, s.v[168]);s.store_scalar(159, s.v[169]);s.store_scalar(160, s.v[169]);s.store_scalar(161, (s.v[167] * s.v[189]));s.store_scalar(162, (s.v[167] * s.v[189]));s.store_scalar(163, (s.v[449] * s.v[189]));s.store_scalar(164, (s.v[449] * s.v[189]));s.store_scalar(165, (s.v[450] * s.v[189]));s.store_scalar(166, (s.v[450] * s.v[189]));s.b[1587] = (p.p8 == 0.0);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1588] = (p.p8 == 1.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });s.b[1589] = (p.p8 == 2.0);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });s.b[1590] = (p.p8 == 3.0);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });s.b[1591] = (p.p8 == 4.0);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });s.b[1592] = (p.p8 == 5.0);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });s.b[1593] = (p.p8 == 6.0);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });s.b[1594] = (p.p8 == 7.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });s.b[1595] = (p.p8 == 8.0);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (p.p8 == 9.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });s.b[1597] = (p.p8 == 10.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if s.b[1587] {s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);}
        if (s.b[1588] && (!s.b[1587])) {s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);s.store_scaled_add(462, 969, 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);s.store_scaled_add(460, 969, 968, s.v[164]);}
        if (s.b[1589] && (!(s.b[1587] || s.b[1588]))) {s.store_scaled_add(461, 971, 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);s.store_scaled_add(459, 971, 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);}
        if (s.b[1590] && (!((s.b[1587] || s.b[1588]) || s.b[1589]))) {s.store_scaled_add(461, 971, 970, s.v[157]);s.store_scaled_add(462, 969, 968, s.v[158]);s.store_scaled_add(459, 971, 970, s.v[163]);s.store_scaled_add(460, 969, 968, s.v[164]);}
        if (s.b[1591] && (!(((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]))) {s.store_add_scaled_inputs(461, 971, s.v[155], 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[161], 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);}
        if (s.b[1592] && (!((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]))) {s.store_scaled_add(461, 971, 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);s.store_scaled_add(459, 971, 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);}
        if (s.b[1593] && (!(((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]))) {s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[156], 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[162], 968, s.v[164]);}
        if (s.b[1594] && (!((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]))) {s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);s.store_scaled_add(462, 969, 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);s.store_scaled_add(460, 969, 968, s.v[164]);}
        if (s.b[1595] && (!(((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]))) {s.store_add_scaled_inputs(461, 971, s.v[159], 970, s.v[157]);s.store_add_scaled_inputs(462, 969, s.v[160], 968, s.v[158]);s.store_add_scaled_inputs(459, 971, s.v[165], 970, s.v[163]);s.store_add_scaled_inputs(460, 969, s.v[166], 968, s.v[164]);}
        if (s.b[1596] && (!((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]))) {s.store_scalar(461, (s.v[155] + ((p.p2 - 1.0) * s.v[157])));s.store_scalar(462, (p.p2 * s.v[158]));s.store_scalar(459, (s.v[161] + ((p.p2 - 1.0) * s.v[163])));s.store_scalar(460, (p.p2 * s.v[164]));}
        if (s.b[1597] && (!(((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]) || s.b[1596]))) {s.store_scalar(461, (p.p2 * s.v[157]));s.store_scalar(462, (s.v[156] + ((p.p2 - 1.0) * s.v[158])));s.store_scalar(459, (p.p2 * s.v[163]));s.store_scalar(460, (s.v[162] + ((p.p2 - 1.0) * s.v[164])));}
        if (!((((((((((s.b[1587] || s.b[1588]) || s.b[1589]) || s.b[1590]) || s.b[1591]) || s.b[1592]) || s.b[1593]) || s.b[1594]) || s.b[1595]) || s.b[1596]) || s.b[1597])) {s.store_scalar(461, 0.0);s.store_scalar(462, 0.0);s.store_scalar(459, 0.0);s.store_scalar(460, 0.0);}
        s.b[1598] = param_given[17];s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if s.b[1598] {s.store_scalar(463, ((p.p17 * p.p50) * p.p49));}
        if (!s.b[1598]) {s.copy_ad(463, 459);}
        s.b[1599] = (s.v[463] < 0.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if s.b[1599] {s.store_scalar(463, 0.0);}
        s.b[1600] = param_given[18];s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if s.b[1600] {s.store_scalar(464, ((p.p18 * p.p50) * p.p49));}
        if (!s.b[1600]) {s.copy_ad(464, 460);}
        s.b[1601] = (s.v[464] < 0.0);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if s.b[1601] {s.store_scalar(464, 0.0);}
        s.b[1602] = param_given[19];s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });s.b[1603] = (p.p926 == 0.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (s.b[1602] && s.b[1603]) {s.store_scalar(494, (p.p19 * p.p50));}
        if (s.b[1602] && (!s.b[1603])) {s.store_scalar(494, (((p.p19 * p.p50) - (s.v[189] * p.p2))).max(0.0));}
        if (!s.b[1602]) {s.copy_ad(494, 461);}
        s.b[1604] = (s.v[494] < 0.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((!s.b[1602]) && s.b[1604]) {s.store_scalar(494, 0.0);}
        s.b[1605] = param_given[20];s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });s.b[1606] = (p.p926 == 0.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1605] && s.b[1606]) {s.store_scalar(495, (p.p20 * p.p50));}
        if (s.b[1605] && (!s.b[1606])) {s.store_scalar(495, (((p.p20 * p.p50) - (s.v[189] * p.p2))).max(0.0));}
        if (!s.b[1605]) {s.copy_ad(495, 462);}
        s.b[1607] = (s.v[495] < 0.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((!s.b[1605]) && s.b[1607]) {s.store_scalar(495, 0.0);}
        s.b[1608] = (((p.p10 > 0.0) && (p.p11 > 0.0)) && ((p.p2 == 1.0) || ((p.p2 > 1.0) && (p.p12 > 0.0))));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if s.b[1608] {s.store_scalar(167, ((s.v[261]) as f64).powf(p.p1111));s.store_scalar(910, (s.v[263] + p.p1104));s.store_powf(168, 910, p.p1112);s.store_add_scaled_inputs3(911, A::div_from_scalar(p.p1108, s.ad_value(167)), 1.0, A::div_from_scalar(p.p1109, s.ad_value(168)), 1.0, A::div_from_scalar(p.p1110, A::mul(s.ad_value(167), s.ad_value(168))), 1.0);s.store_offset(912, 911, 1.0);s.store_scalar(167, ((s.v[261]) as f64).powf(p.p1117));s.store_powf(168, 910, p.p1118);s.store_add_scaled_inputs3(913, A::div_from_scalar(p.p1114, s.ad_value(167)), 1.0, A::div_from_scalar(p.p1115, s.ad_value(168)), 1.0, A::div_from_scalar(p.p1116, A::mul(s.ad_value(167), s.ad_value(168))), 1.0);s.store_offset(914, 913, 1.0);s.store_offset(167, 639, (-1.0));s.store_offset_mul_ad(915, s.ad_value(912), A::scale_offset(s.ad_value(167), p.p1107, 1.0), 1e-9);s.store_scalar(929, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[1608] && (s.v[929] < p.p2)) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[1608] {s.store_div_from_scalar_offset_scaled_input(167, (1.0 / p.p2), 929, (p.p12 + s.v[262]), (p.p10 + (0.5 * s.v[262])));s.store_div_from_scalar_offset_scaled_input(168, (1.0 / p.p2), 929, (p.p12 + s.v[262]), (p.p11 + (0.5 * s.v[262])));s.store_offset(916, 167, s.v[916]);s.store_offset(917, 168, s.v[917]);s.store_primal_offset(929, 929, 1.0);}
        }
        if s.b[1608] {s.store_scalar(918, (1.0 / (p.p1102 + (0.5 * s.v[262]))));s.store_scalar(919, (1.0 / (p.p1103 + (0.5 * s.v[262]))));s.store_primal_add(920, 918, 919);s.store_mul_div_from_scalar_lhs_ad_indices(921, p.p1105, 915, 920);s.store_add(922, 916, 917);s.store_mul_div_from_scalar_lhs_ad_indices(923, p.p1105, 915, 922);s.store_div_scaled_offset_numerator_mixed_ia(924, 923, 1.0, 1.0, A::offset(s.ad_value(921), 1.0), 1.0);s.store_div_scaled_offset_numerator_mixed_ia(925, 923, p.p1106, 1.0, A::scale_offset(s.ad_value(921), p.p1106, 1.0), 1.0);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(926, p.p1113, 914, A::sub(s.ad_value(922), s.ad_value(920)));s.store_mul_div_from_scalar_lhs_ad(927, p.p1119, A::powf(s.ad_value(914), p.p1120), A::sub(s.ad_value(922), s.ad_value(920)));s.store_mul_div_from_scalar_lhs_ad(928, p.p1121, A::powf(s.ad_value(914), p.p1122), A::sub(s.ad_value(922), s.ad_value(920)));s.store_mul(641, 641, 924);s.store_mul(653, 653, 925);s.store_add(734, 734, 927);s.store_add(663, 663, 928);}
        s.b[1609] = (p.p27 == 1.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (s.b[1608] && s.b[1609]) {s.store_mul_div_scaled_inputs_mixed_aii(956, A::sub(s.ad_value(922), s.ad_value(920)), 850, 1.0, 914, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1608] && s.b[1609]) {s.store_mul_div_scaled_inputs_mixed_aia(957, A::sub(s.ad_value(922), s.ad_value(920)), 851, 1.0, A::powf(s.ad_value(914), p.p1120), 1.0);s.store_mul_div_scaled_inputs_mixed_aia(958, A::sub(s.ad_value(922), s.ad_value(920)), 852, 1.0, A::powf(s.ad_value(914), p.p1122), 1.0);}
        if s.b[1608] {s.store_add(849, 849, 957);s.store_add(841, 841, 958);}
        if (!s.b[1608]) {s.store_scalar(926, 0.0);s.store_scalar(956, 0.0);}
        s.b[1610] = (p.p34 == 1.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if s.b[1610] {s.store_scalar(935, (p.p1 / p.p2));s.store_scalar(936, p.p13);s.store_scalar(937, p.p14);s.store_scalar(938, p.p15);}
        s.b[1611] = (((!param_given[13]) && (!param_given[14])) && (!param_given[15]));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });s.b[1612] = (param_given[16] && (p.p16 > 0.0));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if ((s.b[1610] && s.b[1611]) && s.b[1612]) {s.store_offset(168, 935, p.p16);s.store_scalar(169, (1.0 / p.p1137));s.store_div_from_scalar_scaled_input(936, (p.p1137 * p.p1137), 168, p.p16);s.store_div_scaled_add_product_mixed_aaai(937, A::limited_exp_scaled_input(s.ad_value(169), ((-10.0) * p.p16)), ((0.1 * p.p16) + (0.01 * p.p1137)), A::scale_offset(s.ad_value(168), 0.1, (0.01 * p.p1137)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(168), (-10.0), s.ad_value(169))), (-1.0), 935, 1.0);s.store_div_scaled_add_product_mixed_aaai(938, A::limited_exp_scaled_input(s.ad_value(169), ((-20.0) * p.p16)), ((0.05 * p.p16) + (0.0025 * p.p1137)), A::scale_offset(s.ad_value(168), 0.05, (0.0025 * p.p1137)), A::limited_exp(A::mul_scaled_lhs(s.ad_value(168), (-20.0), s.ad_value(169))), (-1.0), 935, 1.0);}
        s.store_mul_add_scaled_inputs3_offset_rhs_indices(930, 806, 936, 1.0, 937, p.p1123, 938, p.p1124, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(931, 807, 936, 1.0, 937, p.p1123, 938, p.p1124, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(932, 855, 936, 1.0, 937, p.p1123, 938, p.p1124, 0.0);s.store_mul_add_scaled_inputs3_offset_rhs_indices(933, 854, 936, 1.0, 937, p.p1123, 938, p.p1124, 0.0);s.store_offset_mul_ad(934, s.ad_value(808), A::add_scaled_inputs3(s.ad_value(936), 1.0, s.ad_value(937), p.p1123, s.ad_value(938), p.p1124), 1.0);s.store_mul(641, 641, 934);s.store_add(734, 734, 931);s.store_add(849, 849, 933);s.store_mul_voltage_ad(221, s.ad_value(379), ctx, nodes, Some(8), Some(10));s.store_mul_voltage_ad(230, s.ad_value(379), ctx, nodes, Some(8), Some(11));s.store_mul_voltage_ad(223, s.ad_value(379), ctx, nodes, Some(6), Some(10));s.store_mul_voltage_ad(224, s.ad_value(379), ctx, nodes, Some(7), Some(10));s.store_mul_voltage_ad(232, s.ad_value(379), ctx, nodes, Some(7), Some(11));s.store_sub(226, 223, 224);s.copy_ad(205, 226);s.copy_ad(202, 224);s.copy_ad(201, 223);s.store_mul_voltage_ad(498, s.ad_value(379), ctx, nodes, Some(10), Some(7));s.store_mul_voltage_ad(499, s.ad_value(379), ctx, nodes, Some(10), Some(6));s.store_sub(204, 221, 223);s.store_sub(203, 221, 224);s.store_mul_voltage_ad(430, s.ad_value(379), ctx, nodes, Some(9), Some(6));s.store_mul_voltage_ad(431, s.ad_value(379), ctx, nodes, Some(9), Some(7));s.store_mul_voltage_ad(234, s.ad_value(379), ctx, nodes, Some(3), Some(10));s.store_mul_voltage_ad(239, s.ad_value(379), ctx, nodes, Some(3), Some(11));s.store_sub(235, 234, 224);s.store_sub(236, 234, 224);s.store_sub(237, 234, 223);s.store_mul_scaled_voltage(1033, 379, -1.0, ctx, nodes, Some(7), Some(10));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scalar(211, 1.0);s.b[1613] = (s.v[226] < 0.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if s.b[1613] {s.store_scalar(211, (-1.0));s.store_mul_voltage_ad(223, s.ad_value(379), ctx, nodes, Some(7), Some(10));s.store_mul_voltage_ad(224, s.ad_value(379), ctx, nodes, Some(6), Some(10));s.store_mul_voltage_ad(232, s.ad_value(379), ctx, nodes, Some(6), Some(11));s.store_sub(235, 234, 224);s.store_mul_scaled_voltage(1033, 379, -1.0, ctx, nodes, Some(6), Some(10));}
        s.store_sub(226, 223, 224);s.store_scale(167, 226, p.p1146);s.b[1614] = (s.v[167] > 80.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if s.b[1614] {s.copy_ad(168, 167);}
        if (!s.b[1614]) {s.store_ln_ad(168, A::offset(A::limited_exp(s.ad_value(167)), 1.0));}
        s.store_offset_sub_scaled_inputs_indices(227, 168, (2.0 / p.p1146), 226, 1.0, (-((2.0 / p.p1146) * ((2.0) as f64).ln())));s.store_add_scaled_inputs3_indices(218, 224, (-1.0), 226, (-0.5), 227, (-(-0.5)));s.store_add_scaled_inputs3_indices(241, 232, (-1.0), 226, (-0.5), 227, (-(-0.5)));s.store_add_scaled_inputs3_indices(220, 234, 1.0, 227, 0.5, 226, (-0.5));s.store_tanh_ad(167, A::div_scaled_inputs(s.ad_value(205), 0.6, s.ad_value(637), 1.0));s.store_offset_scaled(265, 167, 0.5, 0.5);s.store_sub_from_scalar(266, 1.0, 265);s.b[1615] = (p.p35 != 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if s.b[1615] {s.store_add_scaled_products_indices(838, 839, 266, 1.0, 837, 265, 1.0);s.store_add_scaled_products_indices(718, 717, 266, 1.0, 710, 265, 1.0);s.store_add_scaled_products_indices(732, 664, 266, 1.0, 663, 265, 1.0);s.store_add_scaled_products_indices(763, 762, 266, 1.0, 761, 265, 1.0);s.store_add_scaled_products_indices(785, 784, 266, 1.0, 783, 265, 1.0);s.store_add_scaled_products_indices(348, 347, 266, 1.0, 346, 265, 1.0);s.store_add_scaled_products_indices(746, 654, 266, 1.0, 653, 265, 1.0);s.store_add_scaled_products_indices(780, 658, 266, 1.0, 657, 265, 1.0);s.store_add_scaled_products_indices(740, 642, 266, 1.0, 641, 265, 1.0);s.store_add_scaled_products_indices(750, 644, 266, 1.0, 643, 265, 1.0);s.store_add_scaled_products_indices(760, 646, 266, 1.0, 645, 265, 1.0);s.store_add_scaled_products_indices(754, 648, 266, 1.0, 647, 265, 1.0);s.store_add_scaled_products_indices(757, 650, 266, 1.0, 649, 265, 1.0);}
        if (!s.b[1615]) {s.copy_ad(838, 837);s.copy_ad(718, 710);s.copy_ad(732, 663);s.copy_ad(763, 761);s.copy_ad(785, 783);s.copy_ad(348, 346);s.copy_ad(746, 653);s.copy_ad(780, 657);s.copy_ad(740, 641);s.copy_ad(750, 643);s.copy_ad(760, 645);s.copy_ad(754, 647);s.copy_ad(757, 649);}
        s.store_scalar(301, ((((s.v[200] * p.p74) * p.p76)) as f64).sqrt());s.store_scalar(303, (((p.p74 * ((s.v[200] * p.p76) + (0.375 * p.p74)))) as f64).sqrt());s.store_scalar(304, (p.p74 + (s.v[200] * (p.p76 + p.p75))));s.store_add_scaled_inputs4_indices(167, 221, ((p.p75 * s.v[200]) * 1.0 / (s.v[304])), 707, ((-(p.p75 * s.v[200])) * 1.0 / (s.v[304])), 234, (((p.p76 * s.v[200]) + p.p74) * 1.0 / (s.v[304])), 708, ((-((p.p76 * s.v[200]) + p.p74)) * 1.0 / (s.v[304])));s.store_offset_scaled_ad(168, A::atan(A::add_scaled_product(s.ad_value(721), 1.0, s.ad_value(722), s.ad_value(167), 1.0)), 0.3183098861837907, 0.5);s.store_offset_scaled(302, 168, (s.v[301] - s.v[303]), s.v[303]);s.store_offset_div_scaled_inputs_indices(305, 723, s.v[184], 302, 1.0, 1e-6);s.b[1616] = (s.v[305] < 40.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if s.b[1616] {s.store_div_from_scalar_offset_ad(955, 0.5, A::cosh(s.ad_value(305)), (-1.0));}
        if (!s.b[1616]) {s.store_limited_exp_neg_input(955, 305);}
        s.store_scalar(5, (s.v[180] / p.p74));s.store_scalar(7, (s.v[181] / p.p75));s.store_add_scaled_inputs3_offset_mixed_iia(273, 298, 0.5, 218, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(298), s.ad_value(218)), (-0.05)), ((0.25 * 0.1) * 0.1)), 0.5, (0.05 * 0.5));s.store_sqrt(274, 273);s.store_mul(275, 277, 274);s.store_div_from_scalar(260, s.v[180], 275);s.store_add_scaled_inputs_products_indices(276, 709, 1.0, 665, 1.0, 718, 227, 1.0, 719, 218, (-1.0));s.store_offset_scaled(168, 276, 1.0 / (s.v[199]), 1.0);s.b[1617] = (p.p29 == 1.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if s.b[1617] {s.store_scalar(169, ((s.v[5] * s.v[7]) / (s.v[5] + s.v[7])));s.store_mul_add_scaled_inputs3_offset_rhs_indices(170, 227, 718, 1.0, 220, p.p266, 218, (-p.p267), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1617] {s.store_add_scaled_inputs_products_mixed_aiiiia(171, A::add_scaled_product(s.ad_value(220), p.p268, s.ad_value(220), s.ad_value(220), p.p269), 1.0, 218, (-p.p280), 218, 218, (-p.p281), 955, A::add(A::add_scaled_product(A::add_scaled_value_products3(s.ad_value(714), 1.0, s.ad_value(711), s.ad_value(220), 1.0, s.ad_value(220), s.ad_value(220), p.p274, s.ad_value(713), s.ad_value(218), 1.0), 1.0, s.ad_value(218), s.ad_value(218), p.p279), s.ad_value(170)), 1.0);s.store_div_ad(168, A::add_scaled_inputs4_offset(s.ad_value(169), 1.0, s.ad_value(709), 1.0, s.ad_value(665), 1.0, s.ad_value(171), 1.0, s.v[199]), A::offset(s.ad_value(169), s.v[199]));}
        s.store_scaled_add_offset_sqrt_square_offset(267, 168, 1.0, (-1.0), ((0.25 * 0.05) * 0.05), 0.5);s.store_mul(269, 267, 271);s.store_div_from_scalar(270, 1.0, 269);s.store_mul_scale_offset_mixed_ia(292, 227, A::add_scaled_product(s.ad_value(732), 1.0, s.ad_value(733), s.ad_value(218), 1.0), -1.0, 0.0);s.store_offset_sub_scaled_inputs_mixed_ia(292, 292, 0.5, A::sqrt_square_offset(s.ad_value(292), ((0.25 * 5e-5) * 5e-5)), 0.5, (0.25 * 5e-5));s.store_mul_scale_offset(293, A::add_scaled_product(A::offset(s.ad_value(679), (p.p1077 / s.v[184])), 1.0, s.ad_value(680), s.ad_value(218), 1.0), A::powf(s.ad_value(639), p.p1076), 1.0, (-1.0));s.b[1618] = (s.v[279] > 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if s.b[1618] {s.store_mul_scale_offset_indices(167, 227, 281, -1.0, 0.0);}
        s.b[1619] = (s.v[167] < (-80.0));s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (s.b[1618] && s.b[1619]) {s.store_scalar(169, 1.804851387e-35);}
        if (s.b[1618] && (!s.b[1619])) {s.store_limited_exp(169, 167);}
        if s.b[1618] {s.store_offset_mul_offset_rhs(170, 279, 169, 1.0, s.v[184]);s.store_mul_scaled_ln_ad_rhs(278, 269, -1.0, A::max_with_scalar(A::div_from_scalar(s.v[184], s.ad_value(170)), 1e-38));}
        if (!s.b[1618]) {s.store_scalar(278, 0.0);}
        s.store_add_div_rhs_mixed_ia(171, 289, 283, A::pow_from_scalar(s.v[184], s.ad_value(285)));s.store_add_scaled_product_mixed_iia(278, 278, 1.0, 171, A::tanh(A::mul(s.ad_value(287), s.ad_value(227))), (-1.0));s.store_offset(707, 707, p.p25);s.store_mul(222, 221, 270);s.store_mul(225, 224, 270);s.store_mul(212, 707, 270);s.store_mul(215, 708, 270);s.store_mul(238, 234, 270);s.store_add_scaled_products_mixed_iaii(291, 735, A::sub(s.ad_value(274), s.ad_value(299)), 1.0, 734, 218, (-1.0));s.store_add_mixed_ai(242, A::add(A::add_scaled_inputs4(s.ad_value(292), 1.0, s.ad_value(278), 1.0, s.ad_value(291), 1.0, s.ad_value(293), -1.0), s.ad_value(926)), 930);s.store_add_scaled_inputs_product_indices(213, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs_product_mixed_aiii(367, A::add_scaled_product(s.ad_value(222), 1.0, s.ad_value(218), s.ad_value(270), (-1.0)), 1.0, 212, (-1.0), 242, 270, (-1.0));s.store_add_scaled_inputs_product_indices(214, 222, 1.0, 212, (-1.0), 242, 270, (-1.0));s.store_sub(216, 238, 215);s.store_scaled_sqrt_mul_scaled_lhs(294, 706, ((2.0 * 1.602176462e-19) * s.v[180]), 272, 1.0 / (s.v[199]));s.store_scalar(978, 0.5);s.store_scaled_add_sqrt_square_offset_ad(167, A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), ((4.0 * 0.001) * 0.001), 0.5);s.store_offset_div_scaled_inputs_sqrt_rhs(253, 294, 1.0, 167, 2.0, 1.0);let t2: A = A::sqrt(s.ad_value(167));let t3: A = A::ln(A::max_with_scalar(A::mul(A::div_scaled_inputs(s.ad_value(253), 2.0, s.ad_value(294), 1.0), A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(253), (2.0 * s.v[978]), s.ad_value(294), 1.0), 1.0, t2, 2.0)), 1e-38));s.store_add_scaled_inputs3_offset(979, A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), 0.5, t3, 0.5, A::sqrt_square_offset(A::add(A::offset(A::add_scaled_product(s.ad_value(251), 2.0, s.ad_value(224), s.ad_value(272), 1.0), (((((s.v[978]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[978])))), t3), ((4.0 * 0.001) * 0.001)), 0.5, ((((((s.v[978]).max(1e-38)) as f64).ln()) + ((2.0 * s.v[978]))) * 0.5));s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aai(1128, 379, A::add_scaled_product(s.ad_value(707), 1.0, A::add_scaled_product(s.ad_value(979), 1.0, s.ad_value(224), s.ad_value(272), (-1.0)), s.ad_value(271), 1.0), 1.0, A::mul3(s.ad_value(271), s.ad_value(294), A::sqrt(s.ad_value(979))), 1.0, 242, 1.0, 0.0);s.store_div_from_scalar(295, 1.0, 294);s.b[1620] = (p.p29 == 1.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if s.b[1620] {s.store_scaled_sqrt_mul_scaled_lhs(294, 706, ((2.0 * 1.602176462e-19) * s.v[180]), 270, 1.0 / (s.v[199]));s.store_div_from_scalar(295, 1.0, 294);s.store_square(296, 294);s.store_div_from_scalar(297, 1.0, 296);s.store_scalar(5, (s.v[180] / p.p74));s.store_scalar(7, (s.v[181] / p.p75));s.store_primal_div_scaled_inputs2_indices(3, 7, 1.0, 728, 1.0, 5, 1.0);s.store_scalar(2, (p.p76 / p.p75));s.store_div(124, 294, 2);s.store_offset_scaled(125, 124, 0.7071067811865475, 1.0);}
        let (t5,) = {
    if s.b[1620] {
        let t4: f64 = (1e-7 * s.v[125]);
        (t4,)
    } else {
        (s.v[126],)
    }
};
        s.store_scalar(126, t5);
        if s.b[1620] {s.store_scalar(127, (5.0 / 4.0));s.store_div_from_scalar(128, 1.0, 124);s.store_square(129, 124);s.store_div_from_scalar_ad(130, 1.0, A::add_scaled_inputs(s.ad_value(127), 1.0, s.ad_value(124), 0.7324648775608221));}
        s.b[1621] = (((s.v[216]) as f64).abs() <= s.v[126]);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        if (s.b[1620] && s.b[1621]) {s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        s.b[1622] = (s.v[216] < (-s.v[126]));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((s.b[1620] && (!s.b[1621])) && s.b[1622]) {s.store_neg(132, 216);s.store_mul3_lhs(133, 127, 132, 128);s.store_scaled_sub_offset_sqrt_square_offset(134, 133, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(135, A::sub(s.ad_value(132), s.ad_value(134)), 1.0, 129, A::offset(s.ad_value(134), 1.0), 1.0);s.store_add_scaled_inputs3_indices(137, 132, 2.0, 134, (-2.0), 129, -1.0);s.store_sub_mixed_ai(138, A::ln(A::max_with_scalar(A::div(s.ad_value(135), s.ad_value(129)), 1e-38)), 134);s.store_add(0, 135, 137);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 138, A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.5, s.ad_value(135), 1.0), 1.0);s.store_add_mixed_ia(140, 134, A::div_scaled_product3(s.ad_value(135), s.ad_value(0), s.ad_value(138), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(138), s.ad_value(138)), s.ad_value(137), A::sub_scaled_inputs(A::square(s.ad_value(137)), 0.3333333333333333, s.ad_value(135), 1.0))), 1.0));s.store_limited_exp(141, 140);s.store_sub(142, 132, 140);s.store_add_scaled_offset_product_rhs(143, 142, 2.0, 129, 141, (-1.0), 1.0);s.store_add_scaled_square_product_mixed_iia(136, 142, 1.0, 129, A::sub(A::offset(s.ad_value(140), 1.0), s.ad_value(141)), 1.0);s.store_sub_from_scalar_scaled_mul(144, 1.0, 129, 141, 0.5);s.store_add_scaled_square_product_indices(142, 143, 1.0, 144, 136, (-4.0));s.store_scaled_div_mixed_ia(145, 136, A::add(s.ad_value(143), A::sqrt(s.ad_value(142))), 2.0);s.store_neg_add(131, 140, 145);}
        if ((s.b[1620] && (!s.b[1621])) && (!s.b[1622])) {s.store_mul_scale_offset_mixed_ia(146, 130, A::mul3(s.ad_value(125), s.ad_value(127), s.ad_value(130)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(147, 216, 128, A::offset(A::mul(s.ad_value(146), s.ad_value(216)), 1.0));s.store_limited_exp_neg_input(150, 147);s.store_sub_from_scalar(149, 1.0, 150);s.store_add_scaled_inputs_product_mixed_iiia(148, 216, 1.0, 129, 0.5, 124, A::sqrt(A::add_scaled_inputs3(s.ad_value(216), 1.0, s.ad_value(129), 0.25, s.ad_value(149), -1.0)), (-1.0));s.store_limited_exp_neg_input(151, 148);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if ((s.b[1620] && (!s.b[1621])) && (!s.b[1622])) {s.store_add_scaled_inputs3_mixed_iia(152, 216, 2.0, 148, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(129), 1.0, s.ad_value(151)), 1.0);s.store_add_scaled_square_product_mixed_aia(153, A::sub(s.ad_value(216), s.ad_value(148)), 1.0, 129, A::add(A::offset(s.ad_value(148), (-1.0)), s.ad_value(151)), (-1.0));s.store_sub_from_scalar_scaled_mul(154, 1.0, 129, 151, 0.5);s.store_add_scaled_square_product_indices(150, 152, 1.0, 154, 153, (-4.0));s.store_scaled_div_mixed_ia(139, 153, A::add(s.ad_value(152), A::sqrt(s.ad_value(150))), 2.0);s.store_add(131, 148, 139);}
        s.b[1623] = (((s.v[216]) as f64).abs() < s.v[126]);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if (s.b[1620] && s.b[1623]) {s.store_mul_ad_affine_product_rhs(46, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);s.store_mul_ad_affine_product_rhs(131, 216, s.ad_value(128), A::offset(A::mul(s.ad_value(124), A::div_scaled_inputs(s.ad_value(216), -1.0, A::mul_scaled_lhs(s.ad_value(125), (6.0 * ((2.0) as f64).sqrt()), s.ad_value(125)), 1.0)), 1.0), -1.0, 0.0);}
        if (s.b[1620] && (!s.b[1623])) {s.store_add_scaled_inputs3_offset_mixed_aai(19, A::mul3(A::mul3(A::square(s.ad_value(2)), A::sub(s.ad_value(216), s.ad_value(131)), A::sub(s.ad_value(216), s.ad_value(131))), A::div_from_scalar(1.0, s.ad_value(294)), A::div_from_scalar(1.0, s.ad_value(294))), 1.0, A::limited_exp_scaled_input(s.ad_value(131), -1.0), -1.0, 131, -1.0, (-(-1.0)));s.store_offset_add_ad(20, A::limited_exp_scaled_input(s.ad_value(131), -1.0), A::div_scaled_product(A::square(s.ad_value(2)), A::sub_scaled_inputs(s.ad_value(131), 2.0, s.ad_value(216), 2.0), 1.0, A::square(s.ad_value(294)), 1.0), (-1.0));s.store_sub_div_rhs_indices(46, 131, 19, 20);}
        if s.b[1620] {s.store_mul(46, 46, 269);s.store_offset_scaled(95, 294, 0.7071067811865475, 1.0);s.store_div_from_scalar(96, 1.0, 95);s.store_add_mixed_ai(97, A::div_scaled_inputs(s.ad_value(251), 2.0, s.ad_value(267), 1.0), 225);s.store_limited_exp_neg_input(99, 97);}
        let (t7,) = {
    if s.b[1620] {
        let t6: f64 = (0.001 * s.v[95]);
        (t6,)
    } else {
        (s.v[101],)
    }
};
        s.store_scalar(101, t7);
        if s.b[1620] {s.store_div_scaled_inputs_indices(167, 726, (-s.v[184]), 300, 1.0);s.store_mul_ad_product_lhs_mixed_ia(168, 725, A::add_scaled_inputs(A::limited_exp_scaled_input(s.ad_value(167), 0.5), 1.0, A::limited_exp(s.ad_value(167)), 2.0), 724);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1620] {s.store_add_scaled_inputs_product_mixed_aaii(4, A::div_scaled_product(A::scale_offset(s.ad_value(743), 1.0 / (s.v[184]), 1.0), s.ad_value(706), ((1.602176462e-19 * p.p74) * p.p74), s.ad_value(269), (2.0 * s.v[180])), 1.0, A::div_from_scalar(p.p294, s.ad_value(269)), 1.0, 3, 216, (-1.0));}
        let (tf,) = {
    if s.b[1620] {
        let t8: f64 = (-s.v[4]);let t9: f64 = { let limited_exp_arg = t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };let ta: f64 = (t9 + s.v[4]);let tb: f64 = (ta - 1.0);let tc: f64 = (tb).sqrt();let td: f64 = (s.v[294] * tc);let te: f64 = (s.v[4] + td);
        (te,)
    } else {
        (s.v[104],)
    }
};
        s.store_scalar(104, tf);s.b[1624] = (s.v[4] < s.v[97]);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });s.b[1625] = (s.v[214] < s.v[104]);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });s.b[1626] = (((s.v[214]) as f64).abs() <= s.v[101]);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((s.b[1620] && s.b[1624]) && s.b[1625]) && s.b[1626]) {s.store_scaled_square(167, 96, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(9, 214, 96, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(214), 1.0, s.ad_value(99)), s.ad_value(294), s.ad_value(167)), 1.0));}
        s.b[1627] = (s.v[214] < (-s.v[101]));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((s.b[1620] && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {s.store_neg(10, 214);s.store_scaled_mul(11, 10, 96, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(12, 11, 10.0, (-6.0), 64.0, 0.5);s.store_sub(13, 10, 12);s.store_add_scaled_square_product_mixed_iia(14, 13, 1.0, 296, A::offset(s.ad_value(12), 1.0), 1.0);s.store_sub_scaled_inputs(16, 13, 2.0, 296, 1.0);s.store_sub_mixed_ai(17, A::ln(A::max_with_scalar(A::mul(s.ad_value(14), s.ad_value(297)), 1e-38)), 12);s.store_add(0, 14, 16);s.store_add_scaled_square_product_mixed_iia(1, 0, 1.0, 17, A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.5, s.ad_value(14), 1.0), 1.0);s.store_add_mixed_ia(18, 12, A::div_scaled_product3(s.ad_value(14), s.ad_value(0), s.ad_value(17), 1.0, A::add(s.ad_value(1), A::mul3(A::mul3(A::div(s.ad_value(0), s.ad_value(1)), s.ad_value(17), s.ad_value(17)), s.ad_value(16), A::sub_scaled_inputs(A::square(s.ad_value(16)), 0.3333333333333333, s.ad_value(14), 1.0))), 1.0));s.store_limited_exp(28, 18);s.store_div_from_scalar(29, 1.0, 28);s.store_div_from_scalar_offset_square(13, 1.0, 18, 2.0);s.store_mul_square_lhs(30, 18, 13);s.store_mul3_affine_lhs(31, 18, 13, 4.0, 0.0, 13);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(13), 8.0, s.ad_value(30), 12.0), 13, 13);s.store_sub(13, 10, 18);s.store_mul(33, 99, 29);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        if ((((s.b[1620] && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {s.store_add_scaled_product_mixed_iia(36, 13, 2.0, 296, A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(33), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(99), 1.0, s.ad_value(31)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(37, 13, 1.0, 296, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(28), 1.0, s.ad_value(18), (-1.0), s.ad_value(33), 1.0, (-1.0)), 1.0, s.ad_value(99), A::sub(A::offset(s.ad_value(18), (-1.0)), s.ad_value(30)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(13, 2.0, 296, A::add_scaled_inputs_product(s.ad_value(28), 1.0, s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(13, 36, 1.0, 37, 13, (-2.0));s.store_sub_scaled_inputs_mixed_ia(9, 18, -1.0, A::div(s.ad_value(37), A::add(s.ad_value(36), A::sqrt(s.ad_value(13)))), 2.0);}
        if ((((s.b[1620] && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) {s.store_div_from_scalar_offset_scaled_input(38, 1.0, 294, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(39, 38, A::mul_scaled_lhs(s.ad_value(95), 1.25, s.ad_value(38)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(40, 214, 96, A::offset(A::mul(s.ad_value(39), s.ad_value(214)), 1.0));s.store_limited_exp_neg_input(13, 40);s.store_sub_from_scalar(41, 1.0, 13);s.store_add_scaled_inputs_product_mixed_iiia(42, 214, 1.0, 296, 0.5, 294, A::sqrt(A::add_scaled_inputs3(s.ad_value(214), 1.0, s.ad_value(296), 0.25, s.ad_value(41), -1.0)), (-1.0));s.store_offset(43, 97, 3.0);s.store_sub_ad(12, A::add_scaled_inputs3(s.ad_value(42), 0.5, s.ad_value(43), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(42), s.ad_value(43)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(43), 0.5, A::sqrt_square_offset(s.ad_value(43), 5.0), 0.5));s.store_sub(13, 214, 12);s.store_limited_exp_neg_input(33, 12);s.store_div_from_scalar_offset_square(34, 1.0, 12, 2.0);s.store_mul_square_lhs(30, 12, 34);s.store_mul3_affine_lhs(31, 12, 34, 4.0, 0.0, 34);s.store_mul_ad_product_lhs_mixed_ai(32, A::sub_scaled_inputs(s.ad_value(34), 8.0, s.ad_value(30), 12.0), 34, 34);s.store_max_from_scalar_ad(14, 1e-40, A::add_scaled_square_product(s.ad_value(13), 1.0, s.ad_value(296), A::add_scaled_product(A::offset(A::add(s.ad_value(33), s.ad_value(12)), (-1.0)), 1.0, s.ad_value(99), A::add(A::offset(s.ad_value(12), 1.0), s.ad_value(30)), (-1.0)), (-1.0)));s.store_sub_from_scalar_scaled_mul_mixed_ia(15, 1.0, 296, A::add_scaled_product(s.ad_value(33), 1.0, s.ad_value(99), s.ad_value(32), (-1.0)), 0.5);}
    }
}
