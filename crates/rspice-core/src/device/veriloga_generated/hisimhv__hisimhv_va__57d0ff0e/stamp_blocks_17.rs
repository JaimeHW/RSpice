#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1426]) {s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p[236]);s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);s.store_scaled_add(336, 339, 782, 0.5);}
        s.b[1427] = (s.v[336] < 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1427]) {s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[1426]) {s.store_offset(336, 336, 1e-25);s.store_div_from_scalar(337, 1.0, 336);s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));s.store_scaled_abs(338, 190, 2.0);s.store_offset_sub(340, 339, 791, s.v[160]);}
        s.b[1428] = (s.v[340] > s.v[338]);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1428]) {s.copy_ad(338, 340);}
        if (!s.b[1426]) {s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));}
        if (!s.b[1426]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (!s.b[1426]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));s.store_offset_scaled(184, 336, p[235], p[237]);s.store_scalar(341, p[235]);}
        s.b[1429] = ((s.v[184] * 1000000000000.0) < s.v[187]);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if ((!s.b[1426]) && s.b[1429]) {s.store_scalar(184, 0.0);s.store_scalar(80, 0.0);}
        if (!s.b[1426]) {s.store_offset(183, 184, s.v[187]);s.store_div_from_scalar(185, s.v[161], 183);s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));s.store_scale(186, 183, 1.0 / (s.v[161]));s.store_scalar(335, (1.0 / s.v[161]));s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        s.copy_ad(364, 105);s.copy_ad(335, 637);s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);s.store_add_mixed_ai(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);s.copy_ad(123, 158);s.store_scalar(334, 0.95);s.b[338] = (!(s.v[963] > 1.0));s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));
        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p[366]);
        } else {
            s.store_scalar(339, 0.0);
        }
        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);s.store_sqrt(181, 180);s.b[1430] = (p[140] != 0.0);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if s.b[1430] {s.copy_ad(335, 637);s.store_sub_from_scalar(336, p[224], 364);s.store_offset(337, 336, 1e-25);s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));s.store_scaled_add(339, 337, 338, 0.5);s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);s.store_div_from_scalar(341, 1.0, 339);s.store_scale(175, 341, p[223]);s.store_mul_scale_offset_indices(342, 341, 175, -1.0, 0.0);s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));}
        if s.b[1430] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1430] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1430] {s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);s.store_div(343, 334, 176);s.store_mul_sub_lhs(177, 239, 176, 186);s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);s.store_add_scaled_product_mixed_aii(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 685);s.store_scalar(338, (1.0 / (p[140] * p[140])));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_sub(335, 173, 119);s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p[140])), s.v[465]);s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);s.store_offset(178, 106, p[221]);s.store_square(179, 178);s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);}
        if (!s.b[1430]) {s.store_scalar(174, 0.0);}
        s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[582] - p[139]));s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));s.store_mul_scale_offset_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338], 0.0);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_scale_offset_mixed_ia(341, 181, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 0.0);s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);s.store_scalar(335, (s.v[470] / s.v[582]));s.store_offset_scaled(338, 180, s.v[335], s.v[468]);s.store_add_scaled_product_mixed_iia(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p[150], 1.0), s.v[469]);s.store_mul(122, 121, 339);s.store_div_from_scalar(335, 1.0, 185);s.store_square(336, 335);s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));s.store_square(338, 337);s.store_sub(339, 335, 337);s.store_mul_sub_rhs(340, 239, 336, 338);s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);s.store_add_scaled_inputs3_offset_indices(118, 157, 1.0, 336, s.v[189], 120, -1.0, s.v[160]);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_scalar(182, 0.0);s.b[1431] = (s.v[615] == 1.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if s.b[1431] {s.copy_ad(341, 107);s.copy_ad(334, 642);s.store_offset(337, 341, (-p[152]));}
        s.b[1432] = (s.v[337] < (-3.0));s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1432]) {s.store_scalar(340, 0.0);s.store_scalar(182, 0.0);}
        s.b[1433] = (s.v[337] < 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1431] && (!s.b[1432])) && s.b[1433]) {s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if ((s.b[1431] && (!s.b[1432])) && (!s.b[1433])) {s.store_offset_mul_offset_rhs_mixed_ia(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if s.b[1431] {s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);}
        s.b[1434] = (s.v[182] < 0.0);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (s.b[1431] && s.b[1434]) {s.store_scalar(182, 0.0);s.store_scalar(340, 0.0);}
        if s.b[1431] {s.store_mul(182, 182, 334);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1431] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1431] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);}
        s.b[1441] = (s.v[792] > s.v[73]);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if ((p[37] != 0.0) && s.b[1441]) {s.store_sub(335, 792, 73);s.store_sub(336, 72, 73);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(1436, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1436, 1.0);s.store_neg(1436, 1436);s.store_add(1435, 73, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(1436, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if ((p[37] != 0.0) && (!s.b[1441])) {s.copy_ad(1435, 792);s.store_scalar(1436, 1.0);}
        if (p[37] == 0.0) {s.copy_ad(1435, 792);s.store_scalar(1436, 1.0);}
        s.store_scaled_mul(335, 1436, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[262])));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(1437, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1442] = (s.v[1437] < 1e-12);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if s.b[1442] {s.store_scalar(1437, 1e-12);}
        s.store_add(1438, 1435, 1437);s.store_add_scaled_inputs(1439, 790, 1.0, 1437, 2.0);s.store_add(1440, 791, 1437);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1435, 1.0, s.v[160]);s.b[1443] = (s.v[963] != 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });s.b[1444] = (p[42] == 1.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });s.b[1445] = (p[42] == 2.0);s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });s.b[1446] = (p[42] == 3.0);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1463, 960);s.store_scale(1546, 964, 1.6021918e-19);s.store_square(1545, 964);s.store_scale(1502, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1544, 622, 1.6021918e-19);s.store_scalar(1541, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1542, (1.034943e-10 * 1.034943e-10));s.store_square(1543, 965);s.store_div_from_scalar(1547, (2.0 * 1.034943e-10), 1546);s.store_scale(1548, 1546, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1549, 1546, (2.0 * 1.034943e-10));s.store_div_from_scalar(1550, (2.0 * 1.034943e-10), 1544);s.store_scale(1551, 1544, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1536, 964, 622);s.store_div_from_scalar_offset_input(1535, 1.0, 1536, 1.0);s.store_scalar(1552, (1e-12 * 1000.0));s.store_scalar(1553, (1e-10 * 1000.0));s.store_scalar(1461, 0.0);s.store_scalar(1462, 0.0);s.store_scalar(1475, 0.0);s.store_scalar(1476, 0.0);s.store_scalar(1517, 0.0);s.store_scalar(1518, 0.0);s.store_scalar(1497, 0.0);s.store_scalar(1499, 0.0);s.store_scalar(1498, 0.0);s.store_scalar(1500, 0.0);s.store_scalar(1520, 0.0);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1456, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1459, A::div_scaled_value_by_product(s.ad_value(1456), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1545);s.store_sqrt_mul_ad(1453, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1463), s.ad_value(1435)));}
        s.b[1559] = (s.v[1453] > s.v[965]);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_scalar(1466, 0.0);s.copy_ad(1447, 965);s.store_scalar(1483, 0.0);s.store_sub_mixed_ia(1464, 1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)));s.store_scalar(1511, 0.0);s.copy_ad(1510, 1466);s.copy_ad(1472, 1464);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (150.0 + 1.0);let t5: f64 = if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (s.v[97] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;
            if t6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1560] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1561] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = (2.0 == 1.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && s.b[1562]) {s.store_scalar(720, 1.0);}
            s.b[1563] = (2.0 == 2.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && s.b[1563]) {s.store_scalar(720, 2.0);}
            s.b[1564] = (2.0 == 4.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && s.b[1564]) {s.store_scalar(720, 3.0);}
            s.b[1565] = (2.0 == 8.0);s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (!s.b[1562])) && (!s.b[1563])) && (!s.b[1564])) && s.b[1565]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {s.store_scalar(719, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;
                if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && s.b[1561]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) && (!s.b[1561])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1560]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1560])) {s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_add_scaled_inputs3_indices(335, 1464, 1.0, 1435, (-1.0), 1463, 1.0);}
            s.b[1566] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1567] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });s.b[1568] = (2.0 == 1.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && s.b[1568]) {s.store_scalar(720, 1.0);}
            s.b[1569] = (2.0 == 2.0);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && s.b[1569]) {s.store_scalar(720, 2.0);}
            s.b[1570] = (2.0 == 4.0);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && s.b[1570]) {s.store_scalar(720, 3.0);}
            s.b[1571] = (2.0 == 8.0);s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            if ((((((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (!s.b[1568])) && (!s.b[1569])) && (!s.b[1570])) && s.b[1571]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;
                if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && s.b[1567]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) && (!s.b[1567])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1566]) {
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1566])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.store_sqrt_mul(1451, 1550, 336);s.store_mul(1497, 1447, 1546);s.store_mul_div_from_scalar_lhs_ad_indices(1529, (-1.034943e-10), 1447, 334);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1531, (-1.034943e-10), 1451, 341);s.store_add_mixed_ai(1485, A::add_scaled_product(s.ad_value(1497), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1483)), 1.0), 1498);s.copy_ad(1487, 185);s.store_add(1488, 1529, 1531);s.store_add_scaled_product_mixed_iia(1486, 1464, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), s.ad_value(1463)), (-1.0));s.store_scalar(1489, 0.0);s.store_scalar(1490, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1572] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1572]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1572])) {s.store_sub_mixed_ia(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1573] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1464] - s.v[1472])) as f64).abs() <= 1e-12));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1573]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.copy_ad(1510, 1466);s.copy_ad(1472, 1464);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1559]) {s.copy_ad(1513, 1464);s.store_mul(1451, 965, 1536);s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_scaled_product_indices(1483, 1464, 1.0, 1548, 1543, 1.0);s.copy_ad(1461, 1483);s.copy_ad(1467, 1483);s.copy_ad(1509, 1483);}
        s.b[1574] = (s.v[85] > s.v[1466]);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1559]) && s.b[1574]) {s.store_scalar(1479, 1.0);}
        s.b[1575] = (s.v[85] > s.v[1509]);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && s.b[1575]) {s.store_scalar(1479, 3.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1559]) && (!s.b[1574])) && (!s.b[1575])) {s.store_scalar(1479, 2.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1559])) {s.store_scalar(1466, 0.0);s.copy_ad(1509, 1466);s.store_scalar(1467, 0.0);s.copy_ad(1511, 1466);s.copy_ad(1447, 1453);s.store_mul(1451, 1447, 1536);s.store_add_scaled_inputs3_mixed_aii(1464, A::mul3(s.ad_value(1551), s.ad_value(1451), s.ad_value(1451)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_mixed_ai(1483, A::mul3(s.ad_value(1548), s.ad_value(1447), s.ad_value(1447)), 1464);s.copy_ad(1513, 1464);}
        s.b[1576] = (s.v[85] > s.v[1466]);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && s.b[1576]) {s.store_scalar(1479, 1.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1559])) && (!s.b[1576])) {s.store_scalar(1479, 2.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1549, 1467, 1.0, 1435, -1.0, 961, 1.0, 0.0);}
        s.b[1577] = (s.v[335] > 0.0);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1577]) {s.store_add_scaled_inputs3_mixed_iia(1455, 1435, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1577])) {s.store_sub(1455, 1435, 961);}
        s.b[1578] = (s.v[85] > s.v[1466]);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1578]) {s.copy_ad(1464, 1513);s.store_scalar(1483, 0.0);s.store_add_div_lhs(1480, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1483);}
        s.b[1579] = (s.v[1480] < (s.v[1511] + s.v[1553]));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1578]) && s.b[1579]) {s.store_add(1480, 1511, 1553);}
        s.b[1580] = (s.v[85] > s.v[1509]);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1578])) && s.b[1580]) {s.copy_ad(1480, 1461);}
        s.b[1581] = (s.v[85] > s.v[1455]);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));s.copy_ad(1470, 1483);s.store_div_scaled_inputs2_mixed_aii(1480, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);}
        s.b[1582] = (s.v[1480] > (s.v[1467] - s.v[1552]));s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1582]) {s.store_sub(1480, 1467, 1552);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1583] = ((s.v[1449] + s.v[1447]) > s.v[965]);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_scalar(97, 1.0);}
        let mut t9: usize = 0;
        while {
            let t7: f64 = (150.0 + 1.0);let t8: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (s.v[97] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));}
            s.b[1584] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1584]) {s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1584])) {s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);}
            s.b[1585] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1585]) {s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1483, (-1.0));s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));}
            s.b[1586] = (s.v[335] > 0.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1586]) {s.store_div_scaled_inputs2_sqrt_first(1480, 335, 0.5, 1457, (-0.5), 1456, 1.0);}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && (!s.b[1586])) {s.store_div_scaled_inputs_indices(1480, 1457, (-0.5), 1456, 1.0);}
            s.b[1587] = (s.v[1480] > s.v[1467]);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1587]) {s.copy_ad(1480, 1467);}
            s.b[1588] = (s.v[1480] > s.v[1483]);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1588]) {s.store_sub(1480, 1483, 1553);s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1589] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-8);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) && s.b[1589]) {s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && s.b[1581]) && s.b[1583]) {s.copy_ad(1470, 1483);s.store_primal_offset(97, 97, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) {s.store_div_mixed_ia(1460, 1459, A::exp(A::mul(s.ad_value(154), s.ad_value(1435))));s.copy_ad(1470, 1483);s.store_div_ad(1480, A::ln(A::mul3(s.ad_value(1460), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
        s.b[1590] = ((s.v[1449] + s.v[1447]) > s.v[965]);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_scalar(97, 1.0);}
        let mut tc: usize = 0;
        while {
            let ta: f64 = (s.v[421] + 1.0);let tb: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (s.v[97] <= ta)) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_add_scaled_inputs3_indices(1468, 1449, 1.0, 1447, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1449), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1447)));}
            s.b[1591] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1591]) {s.store_offset(1483, 1483, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && (!s.b[1591])) {s.store_sub_div_rhs_indices(1483, 1483, 1468, 1508);}
            s.b[1592] = (((s.v[1483] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1592]) {s.store_offset_sub(1483, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.store_sqrt_mul_sub_rhs(1449, 1547, 1483, 1480);s.store_div_scaled_inputs2_mixed_aia(1464, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
            s.b[1593] = ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-5);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) && s.b[1593]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1578])) && (!s.b[1580])) && (!s.b[1581])) && s.b[1590]) {s.copy_ad(1470, 1483);s.store_primal_offset(97, 97, 1.0);}
        }
        if (s.b[1443] && s.b[1444]) {s.copy_ad(1482, 1483);s.store_scalar(1519, 0.12);s.store_scalar(79, 0.0);s.copy_ad(1461, 1480);s.copy_ad(1483, 1482);s.copy_ad(1469, 1461);s.copy_ad(1470, 1483);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t15: usize = 0;
        while {
            let t13: f64 = (150.0 + 1.0);let t14: f64 = if ((s.b[1443] && s.b[1444]) && (s.v[97] <= t13)) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1443] && s.b[1444]) {s.store_mul_sub_mixed_iai(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1463);s.store_mul(1533, 1535, 1536);s.store_sub(335, 1483, 1464);}
            s.b[1594] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1595] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });s.b[1596] = (2.0 == 1.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && s.b[1596]) {s.store_scalar(720, 1.0);}
            s.b[1597] = (2.0 == 2.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && s.b[1597]) {s.store_scalar(720, 2.0);}
            s.b[1598] = (2.0 == 4.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && s.b[1598]) {s.store_scalar(720, 3.0);}
            s.b[1599] = (2.0 == 8.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (!s.b[1596])) && (!s.b[1597])) && (!s.b[1598])) && s.b[1599]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {s.store_scalar(719, 0.0);}
            let mut t10: usize = 0;
            while {
                let tf: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tf != 0.0
            } {
                t10 += 1;
                if t10 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t10, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && s.b[1444]) && s.b[1594]) && s.b[1595]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1594]) && (!s.b[1595])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1443] && s.b[1444]) && s.b[1594]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1594])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1447, 1547, 336);}
            s.b[1600] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1601] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });s.b[1602] = (2.0 == 1.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && s.b[1602]) {s.store_scalar(720, 1.0);}
            s.b[1603] = (2.0 == 2.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && s.b[1603]) {s.store_scalar(720, 2.0);}
            s.b[1604] = (2.0 == 4.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && s.b[1604]) {s.store_scalar(720, 3.0);}
            s.b[1605] = (2.0 == 8.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (!s.b[1602])) && (!s.b[1603])) && (!s.b[1604])) && s.b[1605]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {s.store_scalar(719, 0.0);}
            let mut t12: usize = 0;
            while {
                let t11: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t11 != 0.0
            } {
                t12 += 1;
                if t12 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t12, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && s.b[1444]) && s.b[1600]) && s.b[1601]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1600]) && (!s.b[1601])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
            if ((s.b[1443] && s.b[1444]) && s.b[1600]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1600])) {s.store_scalar(337, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar(1.034943e-10, s.ad_value(1447)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1529, A::div_from_scalar((-1.034943e-10), s.ad_value(1447)), 334, 337);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);s.store_div_from_scalar(1531, (-1.034943e-10), 1451);s.store_scaled_mul(335, 1502, 1543, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1520, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1464), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1542), s.ad_value(1461), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1461), s.ad_value(1461), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1464), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1461), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), 1543, 1.0, 335, 1.0);s.store_div_mixed_ai(1521, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1461), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_div_mixed_ai(1522, A::add_scaled_products3(s.ad_value(1464), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1461), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1461, 1483);s.store_exp(336, 335);}
            s.b[1606] = (s.v[1461] >= s.v[1483]);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1606]) {s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1523, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1475, 1.0);s.store_neg(1525, 1523);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1606])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1475, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1475, 1.0);s.store_mul_add_mixed_iaa(1523, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1525, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1607] = ((s.v[1520] > (s.v[1511] - s.v[1519])) && (s.v[1519] >= 0.0));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1511, (-1.0), 1519, 1.0);s.store_square(722, 781);s.store_square(723, 1519);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1608] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });s.b[1609] = (4.0 == 1.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && s.b[1609]) {s.store_scalar(720, 1.0);}
            s.b[1610] = (4.0 == 2.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            if (((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && s.b[1610]) {s.store_scalar(720, 2.0);}
            s.b[1611] = (4.0 == 4.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && s.b[1611]) {s.store_scalar(720, 3.0);}
            s.b[1612] = (4.0 == 8.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (!s.b[1609])) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {s.store_scalar(720, 4.0);}
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {s.store_scalar(719, 0.0);}
            let mut te: usize = 0;
            while {
                let td: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                td != 0.0
            } {
                te += 1;
                if te > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", te, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1443] && s.b[1444]) && s.b[1607]) && s.b[1608]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1443] && s.b[1444]) && s.b[1607]) && (!s.b[1608])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1519, 726);s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1511, 1.0, 1519, (-1.0), 780, 1.0);}
            if ((s.b[1443] && s.b[1444]) && s.b[1607]) {
            }
            if ((s.b[1443] && s.b[1444]) && (!s.b[1607])) {s.copy_ad(335, 1520);s.store_scalar(334, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.store_sub(1485, 1483, 335);s.store_mul_scale_offset_indices(1487, 334, 1521, -1.0, 0.0);s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1533), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1475), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1461)), 1.0), 1.0, 1497, 1.0, 1498, 1.0);s.store_sub(1489, 1523, 185);s.store_add_scaled_inputs_products_indices(1490, 1525, 1.0, 1527, 1.0, 1529, 1533, 1.0, 1531, 1533, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1613] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {s.store_offset(1461, 1461, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1443] && s.b[1444]) && s.b[1613]) {s.store_offset(1483, 1483, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1613])) {s.store_sub_mixed_ia(1461, 1461, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1483, 1483, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1614] = (((((s.v[1461] - s.v[1469])) as f64).abs() <= 1e-12) && ((((s.v[1483] - s.v[1470])) as f64).abs() <= 1e-12));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
            if ((s.b[1443] && s.b[1444]) && s.b[1614]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if (s.b[1443] && s.b[1444]) {s.copy_ad(1469, 1461);s.copy_ad(1470, 1483);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1616] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });s.b[1617] = ((s.v[1483] > (s.v[1461] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_offset_sub(781, 1483, 1461, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1618] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });s.b[1619] = (2.0 == 1.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && s.b[1619]) {s.store_scalar(720, 1.0);}
        s.b[1620] = (2.0 == 2.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && s.b[1620]) {s.store_scalar(720, 2.0);}
        s.b[1621] = (2.0 == 4.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && s.b[1621]) {s.store_scalar(720, 3.0);}
        s.b[1622] = (2.0 == 8.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (!s.b[1619])) && (!s.b[1620])) && (!s.b[1621])) && s.b[1622]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {s.store_scalar(719, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;
            if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && s.b[1618]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) && (!s.b[1618])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1483, 1461, (-0.02), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && s.b[1617]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1616]) && (!s.b[1617])) {s.store_scalar(335, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_sub_mixed_iai(1464, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1483), 1.0), 1463);s.store_mul_sub_rhs(335, 154, 1461, 1483);s.store_exp(336, 335);}
        s.b[1623] = (s.v[1461] >= s.v[1483]);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {s.store_mul_scaled_sqrt_ad_rhs(1475, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1538, 1475);s.store_scalar(1517, 0.0);s.store_scalar(1477, 0.0);s.store_sqrt_mul_sub_rhs(1447, 1547, 1483, 1464);}
        s.b[1624] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1625] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });s.b[1626] = (2.0 == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && s.b[1626]) {s.store_scalar(720, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1627] = (2.0 == 2.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && s.b[1627]) {s.store_scalar(720, 2.0);}
        s.b[1628] = (2.0 == 4.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && s.b[1628]) {s.store_scalar(720, 3.0);}
        s.b[1629] = (2.0 == 8.0);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (!s.b[1626])) && (!s.b[1627])) && (!s.b[1628])) && s.b[1629]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;
            if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && s.b[1624]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1623]) && (!s.b[1624])) {s.store_scalar(337, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1623]) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1475, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1630] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1630]) {s.store_scalar(1477, 0.0);s.store_scalar(1517, 0.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1630])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1461), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1483), s.ad_value(1435)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1477, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1517, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_scalar(1538, 0.0);s.store_sub(335, 1483, 1464);}
        s.b[1631] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1632] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });s.b[1633] = (2.0 == 1.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && s.b[1633]) {s.store_scalar(720, 1.0);}
        s.b[1634] = (2.0 == 2.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && s.b[1634]) {s.store_scalar(720, 2.0);}
        s.b[1635] = (2.0 == 4.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && s.b[1635]) {s.store_scalar(720, 3.0);}
        s.b[1636] = (2.0 == 8.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (!s.b[1633])) && (!s.b[1634])) && (!s.b[1635])) && s.b[1636]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {s.store_scalar(719, 0.0);}
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && s.b[1632]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) && (!s.b[1632])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1631]) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1631])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_sqrt_mul(1447, 1547, 336);}
        s.b[1637] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1638] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });s.b[1639] = (2.0 == 1.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && s.b[1639]) {s.store_scalar(720, 1.0);}
        s.b[1640] = (2.0 == 2.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && s.b[1640]) {s.store_scalar(720, 2.0);}
        s.b[1641] = (2.0 == 4.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && s.b[1641]) {s.store_scalar(720, 3.0);}
        s.b[1642] = (2.0 == 8.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (!s.b[1639])) && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && s.b[1638]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) && (!s.b[1638])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && s.b[1637]) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1623])) && (!s.b[1637])) {s.store_scalar(337, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1623])) {s.store_sqrt_mul_ad(1451, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1464), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1497, 1447, 1546);s.store_mul_scale_offset_indices(1498, 1544, 1451, -1.0, 0.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sub(335, 1483, 1464);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1643] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1644] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });s.b[1645] = (2.0 == 1.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && s.b[1645]) {s.store_scalar(720, 1.0);}
        s.b[1646] = (2.0 == 2.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && s.b[1646]) {s.store_scalar(720, 2.0);}
        s.b[1647] = (2.0 == 4.0);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {s.store_scalar(720, 3.0);}
        s.b[1648] = (2.0 == 8.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) && s.b[1648]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {s.store_scalar(719, 0.0);}
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && s.b[1444]) && s.b[1643]) && s.b[1644]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1643]) && (!s.b[1644])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1643]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1643])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1447, 1547, 336);}
        s.b[1649] = ((s.v[1447] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_offset_sub(781, 1447, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1650] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });s.b[1651] = (2.0 == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && s.b[1651]) {s.store_scalar(720, 1.0);}
        s.b[1652] = (2.0 == 2.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && s.b[1652]) {s.store_scalar(720, 2.0);}
        s.b[1653] = (2.0 == 4.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && s.b[1653]) {s.store_scalar(720, 3.0);}
        s.b[1654] = (2.0 == 8.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (!s.b[1651])) && (!s.b[1652])) && (!s.b[1653])) && s.b[1654]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {s.store_scalar(719, 0.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;
            if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && s.b[1444]) && s.b[1649]) && s.b[1650]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1649]) && (!s.b[1650])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {s.store_add_offset_lhs(1447, 965, (-1e-8), 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1649]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1649])) {s.store_scalar(337, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sub(335, 1483, 1461);}
        s.b[1655] = ((s.v[335] < 0.05) && (0.05 >= 0.0));s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_sub_from_scalar(781, 0.05, 335);s.store_square(722, 781);s.store_scalar(723, (0.05 * 0.05));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1656] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });s.b[1657] = (2.0 == 1.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && s.b[1657]) {s.store_scalar(720, 1.0);}
        s.b[1658] = (2.0 == 2.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && s.b[1658]) {s.store_scalar(720, 2.0);}
        s.b[1659] = (2.0 == 4.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && s.b[1659]) {s.store_scalar(720, 3.0);}
        s.b[1660] = (2.0 == 8.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (!s.b[1657])) && (!s.b[1658])) && (!s.b[1659])) && s.b[1660]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;
            if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && s.b[1444]) && s.b[1655]) && s.b[1656]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1655]) && (!s.b[1656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.05);s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);s.store_sub_from_scalar(336, 0.05, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1655]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1655])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_sqrt_mul(1449, 1547, 336);s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1447, (-1.0), 1449, -1.0);}
        s.b[1661] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);s.store_square(722, 781);s.store_scalar(723, (1e-18 * 1e-18));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1662] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });s.b[1663] = (2.0 == 1.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && s.b[1663]) {s.store_scalar(720, 1.0);}
        s.b[1664] = (2.0 == 2.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && s.b[1664]) {s.store_scalar(720, 2.0);}
        s.b[1665] = (2.0 == 4.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && s.b[1665]) {s.store_scalar(720, 3.0);}
        s.b[1666] = (2.0 == 8.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (!s.b[1663])) && (!s.b[1664])) && (!s.b[1665])) && s.b[1666]) {s.store_scalar(720, 4.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {s.store_scalar(719, 0.0);}
        let mut t25: usize = 0;
        while {
            let t24: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1443] && s.b[1444]) && s.b[1661]) && s.b[1662]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1661]) && (!s.b[1662])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-18);s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);s.store_sub_from_scalar(1501, (1e-25 + 1e-18), 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1661]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1661])) {s.copy_ad(1501, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul_scale_offset_indices(1496, 1546, 1501, -1.0, 0.0);}
        s.b[1667] = ((s.v[1453] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });s.b[1668] = ((s.v[1461] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_offset_sub(781, 1461, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });s.b[1670] = (2.0 == 1.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && s.b[1670]) {s.store_scalar(720, 1.0);}
        s.b[1671] = (2.0 == 2.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && s.b[1671]) {s.store_scalar(720, 2.0);}
        s.b[1672] = (2.0 == 4.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && s.b[1672]) {s.store_scalar(720, 3.0);}
        s.b[1673] = (2.0 == 8.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (!s.b[1670])) && (!s.b[1671])) && (!s.b[1672])) && s.b[1673]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;
            if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && s.b[1669]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) && (!s.b[1669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1511, (-0.8), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && s.b[1668]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1667]) && (!s.b[1668])) {s.copy_ad(336, 1461);s.store_scalar(335, 1.0);}
        s.b[1674] = ((s.v[1520] > (s.v[1511] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) {s.store_offset_sub(781, 1520, 1511, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1675] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });s.b[1676] = (2.0 == 1.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && s.b[1676]) {s.store_scalar(720, 1.0);}
        s.b[1677] = (2.0 == 2.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && s.b[1677]) {s.store_scalar(720, 2.0);}
        s.b[1678] = (2.0 == 4.0);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && s.b[1678]) {s.store_scalar(720, 3.0);}
        s.b[1679] = (2.0 == 8.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) && (!s.b[1676])) && (!s.b[1677])) && (!s.b[1678])) && s.b[1679]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1667])) && s.b[1674]) && s.b[1675]) {s.store_scalar(719, 0.0);}
    }
}
