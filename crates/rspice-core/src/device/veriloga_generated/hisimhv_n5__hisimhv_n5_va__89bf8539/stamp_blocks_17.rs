#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1424]) {s.store_add_scaled_inputs3_offset_indices(339, 791, 1.0, 792, (-1.0), 190, -1.0, p[236]);s.store_sqrt_square_offset(782, 339, ((4.0 * (1e-9 * 0.01)) * (1e-9 * 0.01)));s.store_offset_scaled_div(337, 339, 782, 0.5, 0.5);s.store_scaled_add(336, 339, 782, 0.5);}
        s.b[1425] = (s.v[336] < 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1425]) {s.store_scalar(336, 0.0);s.store_scalar(337, 0.0);}
        if (!s.b[1424]) {s.store_offset(336, 336, 1e-25);s.store_div_from_scalar(337, 1.0, 336);s.store_div_from_scalar_square_ad(341, (-1.0), s.ad_value(336));s.store_scaled_abs(338, 190, 2.0);s.store_offset_sub(340, 339, 791, s.v[160]);}
        s.b[1426] = (s.v[340] > s.v[338]);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1426]) {s.copy_ad(338, 340);}
        if (!s.b[1424]) {s.store_offset_sub_ad(781, A::div_from_scalar(1.0, s.ad_value(338)), s.ad_value(337), (-(1e-9 * 0.01)));s.store_scale_ad(782, A::div_from_scalar(1.0, s.ad_value(338)), (4.0 * (1e-9 * 0.01)));}
        if (!s.b[1424]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (!s.b[1424]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(340, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_mixed_aii(336, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 781, (-0.5), 782, (-0.5));s.store_offset_scaled(184, 336, p[235], p[237]);s.store_scalar(341, p[235]);}
        s.b[1427] = ((s.v[184] * 1000000000000.0) < s.v[187]);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((!s.b[1424]) && s.b[1427]) {s.store_scalar(184, 0.0);s.store_scalar(80, 0.0);}
        if (!s.b[1424]) {s.store_offset(183, 184, s.v[187]);s.store_div_from_scalar(185, s.v[161], 183);s.store_div_from_scalar_square_ad(335, (-s.v[161]), s.ad_value(183));s.store_scale(186, 183, 1.0 / (s.v[161]));s.store_scalar(335, (1.0 / s.v[161]));s.store_mul_square_lhs(334, 209, 186);s.store_mul(211, 334, 186);}
        s.copy_ad(364, 105);s.copy_ad(335, 637);s.store_sqrt_mul_sub_rhs(239, 335, 158, 364);s.store_div_scaled_inputs_indices(336, 335, 0.5, 239, 1.0);s.store_add_mixed_ai(173, A::add_scaled_product(A::offset(s.ad_value(158), s.v[160]), 1.0, s.ad_value(239), s.ad_value(186), 1.0), 680);s.copy_ad(123, 158);s.store_scalar(334, 0.95);s.b[338] = (!(s.v[963] > 1.0));s.store_scalar(338, if s.b[338] { 1.0 } else { 0.0 });s.store_offset_sub_scaled_inputs_indices(335, 123, s.v[334], 364, s.v[338], (-0.001));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 123, ((4.0 * s.v[334]) * 0.001));s.store_add_scaled_inputs3_indices(337, 123, s.v[334], 335, (-0.5), 336, (-0.5));
        if (s.v[963] == 1.0) {
            s.store_scale(339, 106, p[366]);
        } else {
            s.store_scalar(339, 0.0);
        }
        s.store_add_scaled_inputs3_indices(180, 123, 1.0, 337, (-1.0), 339, 1.0);s.store_sqrt(181, 180);s.b[1428] = (p[140] != 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if s.b[1428] {s.copy_ad(335, 637);s.store_sub_from_scalar(336, p[224], 364);s.store_offset(337, 336, 1e-25);s.store_sqrt_square_offset(338, 337, (4.0 * 0.001));s.store_scaled_add(339, 337, 338, 0.5);s.store_offset_scaled_div(340, 337, 338, 0.5, 0.5);s.store_div_from_scalar(341, 1.0, 339);s.store_scale(175, 341, p[223]);s.store_mul_scale_offset_indices(342, 341, 175, -1.0, 0.0);s.store_add_scaled_inputs3_offset_indices(781, 158, 0.93, 364, -1.0, 175, -1.0, (-0.001));s.store_scale(782, 158, (0.93 * (4.0 * 0.001)));}
        if s.b[1428] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1428] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(344, 158, 0.93, 781, (-0.5), 782, (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1428] {s.store_sqrt_mul_sub_rhs(176, 335, 158, 344);s.store_div(343, 334, 176);s.store_mul_sub_lhs(177, 239, 176, 186);s.store_scale(335, 622, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_sqrt_mul_sub_rhs(336, 335, 159, 364);s.store_add_scaled_product_mixed_aii(119, A::offset(s.ad_value(159), s.v[160]), 1.0, 336, 186, 1.0);s.store_mul_div_scaled_inputs_indices(337, 186, 335, 0.5, 336, 1.0);s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 685);s.store_scalar(338, (1.0 / (p[140] * p[140])));s.store_mul_ad_product_lhs_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, 338);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_ad_product_lhs_mixed_ai(341, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), 338, 181);s.store_mul_product3_indices(342, 181, 335, 336, 338, (-2.0));s.store_sub(335, 173, 119);s.store_offset_scaled(336, 180, (s.v[467] * 1.0 / (p[140])), s.v[465]);s.store_add_scaled_inputs(337, 336, 1.0, 106, s.v[466]);s.store_offset(178, 106, p[221]);s.store_square(179, 178);s.store_add_scaled_inputs3_mixed_aia(174, A::mul3(s.ad_value(335), s.ad_value(121), s.ad_value(337)), 1.0, 177, 1.0, A::div(s.ad_value(618), s.ad_value(179)), -1.0);}
        if (!s.b[1428]) {s.store_scalar(174, 0.0);}
        s.store_scale(335, 186, 1.034943e-10);s.copy_ad(336, 684);s.store_scalar(337, (s.v[582] - p[139]));s.store_scalar(338, (1.0 / (s.v[337] * s.v[337])));s.store_mul_scale_offset_mixed_ai(339, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(335), 2.0), 336, s.v[338], 0.0);s.store_mul(121, 339, 181);s.store_div_scaled_inputs_indices(340, 339, 0.5, 181, 1.0);s.store_mul_scale_offset_mixed_ia(341, 181, A::mul_sub_from_scalar_lhs_scaled_output(p[137], s.ad_value(123), s.ad_value(336), (2.0 * 1.034943e-10)), s.v[338], 0.0);s.store_mul3_affine_lhs(342, 335, 336, ((-2.0) * s.v[338]), 0.0, 181);s.store_scalar(335, (s.v[470] / s.v[582]));s.store_offset_scaled(338, 180, s.v[335], s.v[468]);s.store_add_scaled_product_mixed_iia(339, 338, 1.0, 106, A::scale_offset(s.ad_value(180), p[150], 1.0), s.v[469]);s.store_mul(122, 121, 339);s.store_div_from_scalar(335, 1.0, 185);s.store_square(336, 335);s.store_div_from_scalar_offset_input(337, 1.0, 185, (s.v[510] / s.v[163]));s.store_square(338, 337);s.store_sub(339, 335, 337);s.store_mul_sub_rhs(340, 239, 336, 338);s.store_offset_mul(124, 239, 339, (s.v[478] / s.v[580]));s.store_add_scaled_inputs3_offset_indices(120, 122, 1.0, 174, 1.0, 124, 1.0, s.v[629]);s.store_sqrt_mul_sub_rhs(336, 637, 157, 105);s.store_add_scaled_inputs3_offset_indices(118, 157, 1.0, 336, s.v[189], 120, -1.0, s.v[160]);s.store_mul(212, 209, 186);s.store_square(213, 212);s.store_scalar(182, 0.0);s.b[1429] = (s.v[615] == 1.0);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if s.b[1429] {s.copy_ad(341, 107);s.copy_ad(334, 642);s.store_offset(337, 341, (-p[152]));}
        s.b[1430] = (s.v[337] < (-3.0));s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if (s.b[1429] && s.b[1430]) {s.store_scalar(340, 0.0);s.store_scalar(182, 0.0);}
        s.b[1431] = (s.v[337] < 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1429] && (!s.b[1430])) && s.b[1431]) {s.store_offset_mul_ad(340, s.ad_value(337), A::scale_offset(s.ad_value(337), (3.0 * (1.0 / 27.0)), (2.0 * (1.0 / 3.0))), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (1.0 / 27.0), (1.0 / 3.0))), 1.0, 1.0);}
        if ((s.b[1429] && (!s.b[1430])) && (!s.b[1431])) {s.store_offset_mul_offset_rhs_mixed_ia(340, 337, A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), (4.0 * 0.148148111111111), (3.0 * 0.0402052934513951))), (2.0 * (1.0 / 3.0)), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(182, 337, A::mul_offset_rhs(s.ad_value(337), A::mul(s.ad_value(337), A::scale_offset(s.ad_value(337), 0.148148111111111, 0.0402052934513951)), (1.0 / 3.0)), 1.0, 1.0);}
        if s.b[1429] {s.store_sqrt_offset_square_offset(782, 182, (-1.0), ((4.0 * 0.05) * 0.05));s.store_scaled_offset_ad(340, A::div_scaled_offset_numerator(s.ad_value(182), 1.0, (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(182, A::offset(s.ad_value(182), (-1.0)), 782, 0.5);}
        s.b[1432] = (s.v[182] < 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (s.b[1429] && s.b[1432]) {s.store_scalar(182, 0.0);s.store_scalar(340, 0.0);}
        if s.b[1429] {s.store_mul(182, 182, 334);s.store_offset_sub_from_scalar_ad(781, 1.0, s.ad_value(182), (-0.05));s.store_scalar(782, (4.0 * 0.05));}
        if s.b[1429] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[1429] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(343, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(182, 781, (-0.5), 782, (-0.5), 1.0);}
        s.b[1439] = (s.v[792] > s.v[73]);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if ((p[37] != 0.0) && s.b[1439]) {s.store_sub(335, 792, 73);s.store_sub(336, 72, 73);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(1434, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 1434, 1.0);s.store_neg(1434, 1434);s.store_add(1433, 73, 333);s.store_div_from_scalar(337, 1.0, 336);s.store_mul(338, 335, 337);s.store_square(339, 338);s.store_add_scaled_product_mixed_aia(341, A::offset(s.ad_value(338), 1.0), 1.0, 339, A::add(A::offset(s.ad_value(338), 1.0), s.ad_value(339)), 1.0);s.store_div_scaled_inputs_product_mixed_aiiia(1434, A::scale_offset(s.ad_value(338), 2.0, 1.0), 1.0, 339, 3.0, 338, 339, 4.0, A::square(s.ad_value(341)), 1.0);}
        if ((p[37] != 0.0) && (!s.b[1439])) {s.copy_ad(1433, 792);s.store_scalar(1434, 1.0);}
        if (p[37] == 0.0) {s.copy_ad(1433, 792);s.store_scalar(1434, 1.0);}
        s.store_scaled_mul(335, 1434, 790, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p[262])));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(1435, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[1440] = (s.v[1435] < 1e-12);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if s.b[1440] {s.store_scalar(1435, 1e-12);}
        s.store_add(1436, 1433, 1435);s.store_add_scaled_inputs(1437, 790, 1.0, 1435, 2.0);s.store_add(1438, 791, 1435);s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]));s.store_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 1433, 1.0, s.v[160]);s.b[1441] = (s.v[963] != 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });s.b[1442] = (p[42] == 1.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });s.b[1443] = (p[42] == 2.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });s.b[1444] = (p[42] == 3.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (s.b[1441] && s.b[1442]) {s.copy_ad(1461, 960);s.store_scale(1544, 964, 1.6021918e-19);s.store_square(1543, 964);s.store_scale(1500, 964, (1.6021918e-19 * 1.034943e-10));s.store_scale(1542, 622, 1.6021918e-19);s.store_scalar(1539, (1.6021918e-19 * 1.6021918e-19));s.store_scalar(1540, (1.034943e-10 * 1.034943e-10));s.store_square(1541, 965);s.store_div_from_scalar(1545, (2.0 * 1.034943e-10), 1544);s.store_scale(1546, 1544, 1.0 / ((2.0 * 1.034943e-10)));s.store_scale(1547, 1544, (2.0 * 1.034943e-10));s.store_div_from_scalar(1548, (2.0 * 1.034943e-10), 1542);s.store_scale(1549, 1542, 1.0 / ((2.0 * 1.034943e-10)));s.store_div(1534, 964, 622);s.store_div_from_scalar_offset_input(1533, 1.0, 1534, 1.0);s.store_scalar(1550, (1e-12 * 1000.0));s.store_scalar(1551, (1e-10 * 1000.0));s.store_scalar(1459, 0.0);s.store_scalar(1460, 0.0);s.store_scalar(1473, 0.0);s.store_scalar(1474, 0.0);s.store_scalar(1515, 0.0);s.store_scalar(1516, 0.0);s.store_scalar(1495, 0.0);s.store_scalar(1497, 0.0);s.store_scalar(1496, 0.0);s.store_scalar(1498, 0.0);s.store_scalar(1518, 0.0);s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 10000000.0));s.store_div_scaled_product_by_product_indices(1454, 185, 185, 1.0, 209, 209, 1.0);s.store_mul_mixed_ai(1457, A::div_scaled_value_by_product(s.ad_value(1454), 1.0, s.ad_value(394), s.ad_value(394), 1.0), 1543);s.store_sqrt_mul_ad(1451, A::div_scaled_product(s.ad_value(1545), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::sub(s.ad_value(1461), s.ad_value(1433)));}
        s.b[1557] = (s.v[1451] > s.v[965]);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_scalar(1464, 0.0);s.copy_ad(1445, 965);s.store_scalar(1481, 0.0);s.store_sub_mixed_ia(1462, 1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)));s.store_scalar(1509, 0.0);s.copy_ad(1508, 1464);s.copy_ad(1470, 1462);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (150.0 + 1.0);let t5: f64 = if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (s.v[97] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;
            if t6 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t6, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1558] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1559] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });s.b[1560] = (2.0 == 1.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && s.b[1560]) {s.store_scalar(720, 1.0);}
            s.b[1561] = (2.0 == 2.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && s.b[1561]) {s.store_scalar(720, 2.0);}
            s.b[1562] = (2.0 == 4.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && s.b[1562]) {s.store_scalar(720, 3.0);}
            s.b[1563] = (2.0 == 8.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (!s.b[1560])) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {s.store_scalar(719, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;
                if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && s.b[1559]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) && (!s.b[1559])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1558]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1558])) {s.store_scalar(334, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_add_scaled_inputs3_indices(335, 1462, 1.0, 1433, (-1.0), 1461, 1.0);}
            s.b[1564] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1565] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = (2.0 == 1.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && s.b[1566]) {s.store_scalar(720, 1.0);}
            s.b[1567] = (2.0 == 2.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && s.b[1567]) {s.store_scalar(720, 2.0);}
            s.b[1568] = (2.0 == 4.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {s.store_scalar(720, 3.0);}
            s.b[1569] = (2.0 == 8.0);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
            if ((((((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) && s.b[1569]) {s.store_scalar(720, 4.0);}
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {s.store_scalar(719, 0.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;
                if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && s.b[1565]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) && (!s.b[1565])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1564]) {
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1564])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.store_sqrt_mul(1449, 1548, 336);s.store_mul(1495, 1445, 1544);s.store_mul_div_from_scalar_lhs_ad_indices(1527, (-1.034943e-10), 1445, 334);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1529, (-1.034943e-10), 1449, 341);s.store_add_mixed_ai(1483, A::add_scaled_product(s.ad_value(1495), 1.0, s.ad_value(185), A::sub(s.ad_value(1464), s.ad_value(1481)), 1.0), 1496);s.copy_ad(1485, 185);s.store_add(1486, 1527, 1529);s.store_add_scaled_product_mixed_iia(1484, 1462, 1.0, 1533, A::sub(A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), s.ad_value(1461)), (-1.0));s.store_scalar(1487, 0.0);s.store_scalar(1488, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1570] = (((((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1464, 1464, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1570]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1570])) {s.store_sub_mixed_ia(1464, 1464, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1571] = (((((s.v[1464] - s.v[1508])) as f64).abs() <= 1e-12) && ((((s.v[1462] - s.v[1470])) as f64).abs() <= 1e-12));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
            if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1571]) {s.store_scalar(97, (150.0 + 1.0));}
            if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.copy_ad(1508, 1464);s.copy_ad(1470, 1462);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1557]) {s.copy_ad(1511, 1462);s.store_mul(1449, 965, 1534);s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_scaled_product_indices(1481, 1462, 1.0, 1546, 1541, 1.0);s.copy_ad(1459, 1481);s.copy_ad(1465, 1481);s.copy_ad(1507, 1481);}
        s.b[1572] = (s.v[85] > s.v[1464]);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1557]) && s.b[1572]) {s.store_scalar(1477, 1.0);}
        s.b[1573] = (s.v[85] > s.v[1507]);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && s.b[1573]) {s.store_scalar(1477, 3.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1557]) && (!s.b[1572])) && (!s.b[1573])) {s.store_scalar(1477, 2.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1557])) {s.store_scalar(1464, 0.0);s.copy_ad(1507, 1464);s.store_scalar(1465, 0.0);s.copy_ad(1509, 1464);s.copy_ad(1445, 1451);s.store_mul(1449, 1445, 1534);s.store_add_scaled_inputs3_mixed_aii(1462, A::mul3(s.ad_value(1549), s.ad_value(1449), s.ad_value(1449)), 1.0, 1433, 1.0, 1461, -1.0);s.store_add_mixed_ai(1481, A::mul3(s.ad_value(1546), s.ad_value(1445), s.ad_value(1445)), 1462);s.copy_ad(1511, 1462);}
        s.b[1574] = (s.v[85] > s.v[1464]);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && s.b[1574]) {s.store_scalar(1477, 1.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1557])) && (!s.b[1574])) {s.store_scalar(1477, 2.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1547, 1465, 1.0, 1433, -1.0, 961, 1.0, 0.0);}
        s.b[1575] = (s.v[335] > 0.0);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1575]) {s.store_add_scaled_inputs3_mixed_iia(1453, 1433, 1.0, 961, (-1.0), A::div(A::sqrt(s.ad_value(335)), s.ad_value(185)), -1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1575])) {s.store_sub(1453, 1433, 961);}
        s.b[1576] = (s.v[85] > s.v[1464]);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1576]) {s.copy_ad(1462, 1511);s.store_scalar(1481, 0.0);s.store_add_div_lhs(1478, A::ln(A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 1481);}
        s.b[1577] = (s.v[1478] < (s.v[1509] + s.v[1551]));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1576]) && s.b[1577]) {s.store_add(1478, 1509, 1551);}
        s.b[1578] = (s.v[85] > s.v[1507]);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1576])) && s.b[1578]) {s.copy_ad(1478, 1459);}
        s.b[1579] = (s.v[85] > s.v[1453]);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_add_scaled_product_indices(1455, 154, 1.0, 1454, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));s.copy_ad(1468, 1481);s.store_div_scaled_inputs2_mixed_aii(1478, A::sqrt(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1456), (-4.0))), 0.5, 1455, (-0.5), 1454, 1.0);}
        s.b[1580] = (s.v[1478] > (s.v[1465] - s.v[1550]));s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1580]) {s.store_sub(1478, 1465, 1550);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1581] = ((s.v[1447] + s.v[1445]) > s.v[965]);s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_scalar(97, 1.0);}
        let mut t9: usize = 0;
        while {
            let t7: f64 = (150.0 + 1.0);let t8: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (s.v[97] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));}
            s.b[1582] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1582]) {s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1582])) {s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);}
            s.b[1583] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1583]) {s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_add_scaled_product_mixed_aii(1456, A::mul3(s.ad_value(1454), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1481, (-1.0));s.store_add_scaled_square_product_indices(335, 1455, 1.0, 1454, 1456, (-4.0));}
            s.b[1584] = (s.v[335] > 0.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1584]) {s.store_div_scaled_inputs2_sqrt_first(1478, 335, 0.5, 1455, (-0.5), 1454, 1.0);}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && (!s.b[1584])) {s.store_div_scaled_inputs_indices(1478, 1455, (-0.5), 1454, 1.0);}
            s.b[1585] = (s.v[1478] > s.v[1465]);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1585]) {s.copy_ad(1478, 1465);}
            s.b[1586] = (s.v[1478] > s.v[1481]);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1586]) {s.store_sub(1478, 1481, 1551);s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1587] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-8);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) && s.b[1587]) {s.store_scalar(97, (150.0 + 1.0));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) && s.b[1581]) {s.copy_ad(1468, 1481);s.store_primal_offset(97, 97, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {s.store_div_mixed_ia(1458, 1457, A::exp(A::mul(s.ad_value(154), s.ad_value(1433))));s.copy_ad(1468, 1481);s.store_div_ad(1478, A::ln(A::mul3(s.ad_value(1458), s.ad_value(85), s.ad_value(85))), A::sub(A::div_from_scalar(2.0, s.ad_value(85)), s.ad_value(154)));s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
        s.b[1588] = ((s.v[1447] + s.v[1445]) > s.v[965]);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.store_scalar(97, 1.0);}
        let mut tc: usize = 0;
        while {
            let ta: f64 = (s.v[421] + 1.0);let tb: f64 = if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (s.v[97] <= ta)) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.store_add_scaled_inputs3_indices(1466, 1447, 1.0, 1445, 1.0, 965, -1.0);s.store_add_ad(1506, A::div_scalar_by_product(1.034943e-10, s.ad_value(1544), s.ad_value(1447), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1544)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1534), 1.0, s.ad_value(1534), 1.0, 1.0)), s.ad_value(1445)));}
            s.b[1589] = ((((s.v[1466] / s.v[1506])) as f64).abs() > 0.5);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1589]) {s.store_offset(1481, 1481, (-(0.5 * (if ((s.v[1466] / s.v[1506]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && (!s.b[1589])) {s.store_sub_div_rhs_indices(1481, 1481, 1466, 1506);}
            s.b[1590] = (((s.v[1481] - s.v[1433]) + s.v[1461]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1590]) {s.store_offset_sub(1481, 1433, 1461, (10.0 * 2.220446049250313e-16));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.store_sqrt_mul_sub_rhs(1447, 1545, 1481, 1478);s.store_div_scaled_inputs2_mixed_aia(1462, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1.0, 1461, (-1.0), A::offset(s.ad_value(1534), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
            s.b[1591] = ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) && s.b[1591]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((((s.b[1441] && s.b[1442]) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) && s.b[1588]) {s.copy_ad(1468, 1481);s.store_primal_offset(97, 97, 1.0);}
        }
        if (s.b[1441] && s.b[1442]) {s.copy_ad(1480, 1481);s.store_scalar(1517, 0.12);s.store_scalar(79, 0.0);s.copy_ad(1459, 1478);s.copy_ad(1481, 1480);s.copy_ad(1467, 1459);s.copy_ad(1468, 1481);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t15: usize = 0;
        while {
            let t13: f64 = (150.0 + 1.0);let t14: f64 = if ((s.b[1441] && s.b[1442]) && (s.v[97] <= t13)) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1441] && s.b[1442]) {s.store_mul_sub_mixed_iai(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1461);s.store_mul(1531, 1533, 1534);s.store_sub(335, 1481, 1462);}
            s.b[1592] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1593] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });s.b[1594] = (2.0 == 1.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && s.b[1594]) {s.store_scalar(720, 1.0);}
            s.b[1595] = (2.0 == 2.0);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && s.b[1595]) {s.store_scalar(720, 2.0);}
            s.b[1596] = (2.0 == 4.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && s.b[1596]) {s.store_scalar(720, 3.0);}
            s.b[1597] = (2.0 == 8.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (!s.b[1594])) && (!s.b[1595])) && (!s.b[1596])) && s.b[1597]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {s.store_scalar(719, 0.0);}
            let mut t10: usize = 0;
            while {
                let tf: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                tf != 0.0
            } {
                t10 += 1;
                if t10 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t10, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1592]) && s.b[1593]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1592]) && (!s.b[1593])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((s.b[1441] && s.b[1442]) && s.b[1592]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1592])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1445, 1545, 336);}
            s.b[1598] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1599] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });s.b[1600] = (2.0 == 1.0);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && s.b[1600]) {s.store_scalar(720, 1.0);}
            s.b[1601] = (2.0 == 2.0);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && s.b[1601]) {s.store_scalar(720, 2.0);}
            s.b[1602] = (2.0 == 4.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && s.b[1602]) {s.store_scalar(720, 3.0);}
            s.b[1603] = (2.0 == 8.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (!s.b[1600])) && (!s.b[1601])) && (!s.b[1602])) && s.b[1603]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {s.store_scalar(719, 0.0);}
            let mut t12: usize = 0;
            while {
                let t11: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t11 != 0.0
            } {
                t12 += 1;
                if t12 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t12, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1598]) && s.b[1599]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1598]) && (!s.b[1599])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
            if ((s.b[1441] && s.b[1442]) && s.b[1598]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1598])) {s.store_scalar(337, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_ad_product_lhs_mixed_ai(1525, A::div_from_scalar(1.034943e-10, s.ad_value(1445)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1527, A::div_from_scalar((-1.034943e-10), s.ad_value(1445)), 334, 337);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);s.store_div_from_scalar(1529, (-1.034943e-10), 1449);s.store_scaled_mul(335, 1500, 1541, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1518, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1462), s.ad_value(1540), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1540), s.ad_value(1459), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1540), s.ad_value(1459), s.ad_value(1459), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1459), s.ad_value(1500), s.ad_value(1541), 4.0), 1.0, A::mul3(s.ad_value(1543), s.ad_value(1539), s.ad_value(1541)), 1541, 1.0, 335, 1.0);s.store_div_mixed_ai(1519, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (-8.0), s.ad_value(1540), s.ad_value(1459), (4.0 * 2.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_div_mixed_ai(1520, A::add_scaled_products3(s.ad_value(1462), s.ad_value(1540), (4.0 * 2.0), s.ad_value(1540), s.ad_value(1459), (-8.0), s.ad_value(1500), s.ad_value(1541), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1459, 1481);s.store_exp(336, 335);}
            s.b[1604] = (s.v[1459] >= s.v[1481]);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1604]) {s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1521, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1473, 1.0);s.store_neg(1523, 1521);}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1604])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1473, 209, A::add_scaled_product(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1473, 1.0);s.store_mul_add_mixed_iaa(1521, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1523, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1605] = ((s.v[1518] > (s.v[1509] - s.v[1517])) && (s.v[1517] >= 0.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {s.store_add_scaled_inputs3_indices(781, 1518, 1.0, 1509, (-1.0), 1517, 1.0);s.store_square(722, 781);s.store_square(723, 1517);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1606] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });s.b[1607] = (4.0 == 1.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
            if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && s.b[1607]) {s.store_scalar(720, 1.0);}
            s.b[1608] = (4.0 == 2.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
            if (((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {s.store_scalar(720, 2.0);}
            s.b[1609] = (4.0 == 4.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
            if ((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && s.b[1609]) {s.store_scalar(720, 3.0);}
            s.b[1610] = (4.0 == 8.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
            if (((((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (!s.b[1609])) && s.b[1610]) {s.store_scalar(720, 4.0);}
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {s.store_scalar(719, 0.0);}
            let mut te: usize = 0;
            while {
                let td: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                td != 0.0
            } {
                te += 1;
                if te > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", te, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
                if (((s.b[1441] && s.b[1442]) && s.b[1605]) && s.b[1606]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
            }
            if (((s.b[1441] && s.b[1442]) && s.b[1605]) && (!s.b[1606])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1517, 726);s.store_div_scaled_product3_indices(334, 1517, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1509, 1.0, 1517, (-1.0), 780, 1.0);}
            if ((s.b[1441] && s.b[1442]) && s.b[1605]) {
            }
            if ((s.b[1441] && s.b[1442]) && (!s.b[1605])) {s.copy_ad(335, 1518);s.store_scalar(334, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.store_sub(1483, 1481, 335);s.store_mul_scale_offset_indices(1485, 334, 1519, -1.0, 0.0);s.store_sub_from_scalar_ad(1486, 1.0, A::mul3(s.ad_value(1520), s.ad_value(1531), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1484, A::add_scaled_product(s.ad_value(1473), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1459)), 1.0), 1.0, 1495, 1.0, 1496, 1.0);s.store_sub(1487, 1521, 185);s.store_add_scaled_inputs_products_indices(1488, 1523, 1.0, 1525, 1.0, 1527, 1531, 1.0, 1529, 1531, 1.0);s.store_add_scaled_products_indices(1489, 1485, 1488, 1.0, 1487, 1486, (-1.0));s.store_div(1490, 1488, 1489);s.store_div_scaled_inputs_indices(1491, 1486, -1.0, 1489, 1.0);s.store_div_scaled_inputs_indices(1492, 1487, -1.0, 1489, 1.0);s.store_div(1493, 1485, 1489);}
            s.b[1611] = (((((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484]))) as f64).abs() > 0.5);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {s.store_offset(1459, 1459, (-(0.5 * (if (((s.v[1490] * s.v[1483]) + (s.v[1491] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1441] && s.b[1442]) && s.b[1611]) {s.store_offset(1481, 1481, (-(0.5 * (if (((s.v[1492] * s.v[1483]) + (s.v[1493] * s.v[1484])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((s.b[1441] && s.b[1442]) && (!s.b[1611])) {s.store_sub_mixed_ia(1459, 1459, A::add_scaled_products(s.ad_value(1490), s.ad_value(1483), 1.0, s.ad_value(1491), s.ad_value(1484), 1.0));s.store_sub_mixed_ia(1481, 1481, A::add_scaled_products(s.ad_value(1492), s.ad_value(1483), 1.0, s.ad_value(1493), s.ad_value(1484), 1.0));}
            s.b[1612] = (((((s.v[1459] - s.v[1467])) as f64).abs() <= 1e-12) && ((((s.v[1481] - s.v[1468])) as f64).abs() <= 1e-12));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
            if ((s.b[1441] && s.b[1442]) && s.b[1612]) {s.store_scalar(97, (150.0 + 1.0));s.store_scalar(79, 1.0);}
            if (s.b[1441] && s.b[1442]) {s.copy_ad(1467, 1459);s.copy_ad(1468, 1481);s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1614] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });s.b[1615] = ((s.v[1481] > (s.v[1459] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {s.store_offset_sub(781, 1481, 1459, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1616] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });s.b[1617] = (2.0 == 1.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && s.b[1617]) {s.store_scalar(720, 1.0);}
        s.b[1618] = (2.0 == 2.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && s.b[1618]) {s.store_scalar(720, 2.0);}
        s.b[1619] = (2.0 == 4.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && s.b[1619]) {s.store_scalar(720, 3.0);}
        s.b[1620] = (2.0 == 8.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (!s.b[1617])) && (!s.b[1618])) && (!s.b[1619])) && s.b[1620]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {s.store_scalar(719, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;
            if t17 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t17, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && s.b[1616]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) && (!s.b[1616])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1481, 1459, (-0.02), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && s.b[1615]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1614]) && (!s.b[1615])) {s.store_scalar(335, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_sub_mixed_iai(1462, 1533, A::add_scaled_product(s.ad_value(1433), 1.0, s.ad_value(1534), s.ad_value(1481), 1.0), 1461);s.store_mul_sub_rhs(335, 154, 1459, 1481);s.store_exp(336, 335);}
        s.b[1621] = (s.v[1459] >= s.v[1481]);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {s.store_mul_scaled_sqrt_ad_rhs(1473, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1536, 1473);s.store_scalar(1515, 0.0);s.store_scalar(1475, 0.0);s.store_sqrt_mul_sub_rhs(1445, 1545, 1481, 1462);}
        s.b[1622] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1623] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });s.b[1624] = (2.0 == 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && s.b[1624]) {s.store_scalar(720, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1625] = (2.0 == 2.0);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && s.b[1625]) {s.store_scalar(720, 2.0);}
        s.b[1626] = (2.0 == 4.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && s.b[1626]) {s.store_scalar(720, 3.0);}
        s.b[1627] = (2.0 == 8.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (!s.b[1624])) && (!s.b[1625])) && (!s.b[1626])) && s.b[1627]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {s.store_scalar(719, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;
            if t19 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t19, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && s.b[1623]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) && (!s.b[1623])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && s.b[1622]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1621]) && (!s.b[1622])) {s.store_scalar(337, 1.0);}
        if ((s.b[1441] && s.b[1442]) && s.b[1621]) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)));s.store_mul_sqrt_mixed_ia(1473, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1628] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1628]) {s.store_scalar(1475, 0.0);s.store_scalar(1515, 0.0);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1628])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1459), s.ad_value(1433)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1481), s.ad_value(1433)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1475, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));s.store_mul_sqrt_mixed_ia(1515, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_scalar(1536, 0.0);s.store_sub(335, 1481, 1462);}
        s.b[1629] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1630] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });s.b[1631] = (2.0 == 1.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && s.b[1631]) {s.store_scalar(720, 1.0);}
        s.b[1632] = (2.0 == 2.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && s.b[1632]) {s.store_scalar(720, 2.0);}
        s.b[1633] = (2.0 == 4.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && s.b[1633]) {s.store_scalar(720, 3.0);}
        s.b[1634] = (2.0 == 8.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (!s.b[1631])) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {s.store_scalar(719, 0.0);}
        let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && s.b[1630]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) && (!s.b[1630])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1629]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1629])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_sqrt_mul(1445, 1545, 336);}
        s.b[1635] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1636] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });s.b[1637] = (2.0 == 1.0);s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && s.b[1637]) {s.store_scalar(720, 1.0);}
        s.b[1638] = (2.0 == 2.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && s.b[1638]) {s.store_scalar(720, 2.0);}
        s.b[1639] = (2.0 == 4.0);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && s.b[1639]) {s.store_scalar(720, 3.0);}
        s.b[1640] = (2.0 == 8.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (!s.b[1637])) && (!s.b[1638])) && (!s.b[1639])) && s.b[1640]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {s.store_scalar(719, 0.0);}
        let mut t1d: usize = 0;
        while {
            let t1c: f64 = if (((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1c != 0.0
        } {
            t1d += 1;
            if t1d > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1d, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && s.b[1636]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) && (!s.b[1636])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && s.b[1635]) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {
        }
        if (((s.b[1441] && s.b[1442]) && (!s.b[1621])) && (!s.b[1635])) {s.store_scalar(337, 1.0);}
        if ((s.b[1441] && s.b[1442]) && (!s.b[1621])) {s.store_sqrt_mul_ad(1449, s.ad_value(1548), A::add_scaled_inputs3(s.ad_value(1462), 1.0, s.ad_value(1433), (-1.0), s.ad_value(1461), 1.0));s.store_mul(1495, 1445, 1544);s.store_mul_scale_offset_indices(1496, 1542, 1449, -1.0, 0.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sub(335, 1481, 1462);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        s.b[1641] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1642] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });s.b[1643] = (2.0 == 1.0);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && s.b[1643]) {s.store_scalar(720, 1.0);}
        s.b[1644] = (2.0 == 2.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && s.b[1644]) {s.store_scalar(720, 2.0);}
        s.b[1645] = (2.0 == 4.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && s.b[1645]) {s.store_scalar(720, 3.0);}
        s.b[1646] = (2.0 == 8.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (!s.b[1643])) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {s.store_scalar(719, 0.0);}
        let mut t1f: usize = 0;
        while {
            let t1e: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1e != 0.0
        } {
            t1f += 1;
            if t1f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1641]) && s.b[1642]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1641]) && (!s.b[1642])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1641]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1641])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1445, 1545, 336);}
        s.b[1647] = ((s.v[1445] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_offset_sub(781, 1445, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1648] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });s.b[1649] = (2.0 == 1.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && s.b[1649]) {s.store_scalar(720, 1.0);}
        s.b[1650] = (2.0 == 2.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && s.b[1650]) {s.store_scalar(720, 2.0);}
        s.b[1651] = (2.0 == 4.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {s.store_scalar(720, 3.0);}
        s.b[1652] = (2.0 == 8.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) && s.b[1652]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {s.store_scalar(719, 0.0);}
        let mut t21: usize = 0;
        while {
            let t20: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t20 != 0.0
        } {
            t21 += 1;
            if t21 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t21, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1647]) && s.b[1648]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1647]) && (!s.b[1648])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {s.store_add_offset_lhs(1445, 965, (-1e-8), 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1647]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1647])) {s.store_scalar(337, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sub(335, 1481, 1459);}
        s.b[1653] = ((s.v[335] < 0.05) && (0.05 >= 0.0));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {s.store_sub_from_scalar(781, 0.05, 335);s.store_square(722, 781);s.store_scalar(723, (0.05 * 0.05));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1654] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });s.b[1655] = (2.0 == 1.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && s.b[1655]) {s.store_scalar(720, 1.0);}
        s.b[1656] = (2.0 == 2.0);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {s.store_scalar(720, 2.0);}
        s.b[1657] = (2.0 == 4.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && s.b[1657]) {s.store_scalar(720, 3.0);}
        s.b[1658] = (2.0 == 8.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) && (!s.b[1657])) && s.b[1658]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {s.store_scalar(719, 0.0);}
        let mut t23: usize = 0;
        while {
            let t22: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t22 != 0.0
        } {
            t23 += 1;
            if t23 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t23, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1653]) && s.b[1654]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1653]) && (!s.b[1654])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.05);s.store_div_scaled_product_indices(334, 725, 726, 0.05, 770, 1.0);s.store_sub_from_scalar(336, 0.05, 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1653]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1653])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_sqrt_mul(1447, 1545, 336);s.store_add_scaled_inputs3_indices(335, 965, 1.0, 1445, (-1.0), 1447, -1.0);}
        s.b[1659] = ((s.v[335] < (1e-25 + 1e-18)) && (1e-18 >= 0.0));s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {s.store_sub_from_scalar(781, (1e-25 + 1e-18), 335);s.store_square(722, 781);s.store_scalar(723, (1e-18 * 1e-18));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1660] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });s.b[1661] = (2.0 == 1.0);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && s.b[1661]) {s.store_scalar(720, 1.0);}
        s.b[1662] = (2.0 == 2.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && s.b[1662]) {s.store_scalar(720, 2.0);}
        s.b[1663] = (2.0 == 4.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {s.store_scalar(720, 3.0);}
        s.b[1664] = (2.0 == 8.0);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {s.store_scalar(720, 4.0);}
        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {s.store_scalar(719, 0.0);}
        let mut t25: usize = 0;
        while {
            let t24: f64 = if ((((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[1441] && s.b[1442]) && s.b[1659]) && s.b[1660]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[1441] && s.b[1442]) && s.b[1659]) && (!s.b[1660])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-18);s.store_div_scaled_product_indices(334, 725, 726, 1e-18, 770, 1.0);s.store_sub_from_scalar(1499, (1e-25 + 1e-18), 780);}
        if ((s.b[1441] && s.b[1442]) && s.b[1659]) {
        }
        if ((s.b[1441] && s.b[1442]) && (!s.b[1659])) {s.copy_ad(1499, 335);s.store_scalar(334, 1.0);}
        if (s.b[1441] && s.b[1442]) {s.store_mul_scale_offset_indices(1494, 1544, 1499, -1.0, 0.0);}
        s.b[1665] = ((s.v[1451] > s.v[965]) && (s.v[1477] != 2.0));s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });s.b[1666] = ((s.v[1459] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_offset_sub(781, 1459, 1509, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1667] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });s.b[1668] = (2.0 == 1.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && s.b[1668]) {s.store_scalar(720, 1.0);}
        s.b[1669] = (2.0 == 2.0);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && s.b[1669]) {s.store_scalar(720, 2.0);}
        s.b[1670] = (2.0 == 4.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && s.b[1670]) {s.store_scalar(720, 3.0);}
        s.b[1671] = (2.0 == 8.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (!s.b[1668])) && (!s.b[1669])) && (!s.b[1670])) && s.b[1671]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {s.store_scalar(719, 0.0);}
        let mut t27: usize = 0;
        while {
            let t26: f64 = if (((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;
            if t27 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t27, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && s.b[1667]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) && (!s.b[1667])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.8);s.store_div_scaled_product_indices(335, 725, 726, 0.8, 770, 1.0);s.store_add_offset_lhs(336, 1509, (-0.8), 780);}
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && s.b[1666]) {
        }
        if (((s.b[1441] && s.b[1442]) && s.b[1665]) && (!s.b[1666])) {s.copy_ad(336, 1459);s.store_scalar(335, 1.0);}
        s.b[1672] = ((s.v[1518] > (s.v[1509] - 0.8)) && (0.8 >= 0.0));s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) {s.store_offset_sub(781, 1518, 1509, 0.8);s.store_square(722, 781);s.store_scalar(723, (0.8 * 0.8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1673] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });s.b[1674] = (2.0 == 1.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && s.b[1674]) {s.store_scalar(720, 1.0);}
        s.b[1675] = (2.0 == 2.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && s.b[1675]) {s.store_scalar(720, 2.0);}
        s.b[1676] = (2.0 == 4.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && s.b[1676]) {s.store_scalar(720, 3.0);}
        s.b[1677] = (2.0 == 8.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((((((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) && (!s.b[1674])) && (!s.b[1675])) && (!s.b[1676])) && s.b[1677]) {s.store_scalar(720, 4.0);}
        if ((((s.b[1441] && s.b[1442]) && (!s.b[1665])) && s.b[1672]) && s.b[1673]) {s.store_scalar(719, 0.0);}
    }
}
