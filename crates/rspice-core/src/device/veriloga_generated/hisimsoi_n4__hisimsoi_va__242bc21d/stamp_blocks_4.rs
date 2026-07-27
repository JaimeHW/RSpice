#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t4: usize = 0;
        while {
            let t1: f64 = (2.0 * 20.0);let t2: f64 = (t1 + 1.0);let t3: f64 = if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (s.v[167] <= t2)) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;
            if t4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {s.store_scalar(1462, 0.0);s.store_mul_add_rhs(1438, 225, 1440, 1415);}
            s.b[1484] = (s.v[1438] < 5.0);s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1484]) {s.store_mul3_ad_middle(1458, A::square(s.ad_value(1438)), 1438, A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1459, A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1460, 1466, 1458, 1458);s.store_mul_product3_indices(1461, 1459, 1466, 225, 1458, 2.0);s.store_mul_scale_offset_mixed_ia(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1463, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, 1461, 1.0, 1464, 2.0);}
            s.b[1485] = (s.v[1438] < 80.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && s.b[1485]) {s.store_exp(243, 1438);s.store_mul_scale_offset_indices(1460, 1466, 243, 1.0, (-1.0));s.store_mul3_lhs(1461, 1466, 225, 243);}
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) && (!s.b[1485])) {s.store_exp_mul(1467, 225, 1440);s.store_mul_sub_rhs(1460, 1451, 1467, 1457);s.store_mul3_lhs(1461, 1451, 225, 1467);}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1484])) {s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);}
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));s.store_sub_from_scalar_scaled_mul(1469, (-1.0), 1424, 1465, 1.0);}
            s.b[1486] = (s.v[1411] == 1.0);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1486]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {s.store_div_scaled_inputs_indices(494, 1468, -1.0, 1469, 1.0);}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1487] = (((s.v[494]) as f64).abs() > s.v[1470]);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1487]) {s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) {s.store_add(1440, 1440, 494);}
            s.b[1488] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
            let (t0,) = {
    if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1486])) && s.b[1488]) {
        (1.0,)
    } else {
        (s.v[1411],)
    }
};
            s.store_scalar(1411, t0);
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1490] = (s.v[1438] < 5.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
        if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && s.b[1490]) {s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) && (!s.b[1490])) {s.store_offset(1471, 1438, (-1.0));s.store_sqrt(1472, 1471);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1477])) && s.b[1483]) {s.store_mul(458, 1423, 1472);s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));s.store_mul3_lhs(460, 1423, 1460, 1395);s.store_add(459, 458, 460);}
        if ((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) {s.store_sub(460, 459, 458);}
        s.b[1492] = (1.0 == 1.0);s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });s.b[1493] = (1.0 == 2.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1492]) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (s.b[1493] && (!s.b[1492]))) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        if ((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) {s.store_scalar(1405, ((1.0 - 1.0) / 2.0));s.store_scalar(1406, ((1.0 + 1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1416, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1417, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1418, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1419, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1420, 1417, 1416);s.store_neg(1421, 1416);s.store_primal_add_scaled_products_indices(1407, 1405, 461, 1.0, 1406, 462, 1.0);s.store_primal_add_scaled_products_indices(1408, 1405, 462, 1.0, 1406, 461, 1.0);s.store_add_scaled_products_indices(1422, 1407, 1418, 1.0, 1408, 1419, 1.0);s.store_offset_ad(1414, A::add_scaled_products(s.ad_value(1407), s.ad_value(1421), 1.0, s.ad_value(1408), s.ad_value(1420), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1394, 1414);}
        s.b[1494] = (s.v[1394] > s.v[141]);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1494]) {s.store_sub(1395, 1394, 141);s.store_sub(1396, 140, 141);s.store_div(44, 1395, 1396);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1402, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1402, 1396, 1402, -1.0, 1.0);s.store_add(1399, 141, 1402);}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1494])) {s.copy_ad(1399, 1394);}
        if ((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) {s.store_offset_scaled(1415, 1399, -1.0, (-1e-12));s.store_mul(1424, 1423, 1404);s.store_square(1425, 1424);s.store_sub(1426, 1422, 523);s.store_div(1394, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1427, 2.0, 225, A::ln(s.ad_value(1394)));}
        let (t6,) = {
    if ((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) {
        let t5: f64 = (-s.v[1415]);
        (t5,)
    } else {
        (s.v[1428],)
    }
};
        s.store_scalar(1428, t6);s.b[1495] = (s.v[1426] < s.v[1428]);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1495]) {s.store_div_scalar_by_product_indices(1395, 1.0, 225, 1423, 1.0);s.store_mul(1402, 1395, 1403);s.store_offset_scaled(1429, 1402, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1430, 1429, 1429, 8.0, 0.0, 1429);s.store_sub(1431, 237, 1427);s.store_mul_add_rhs(1401, 225, 1426, 1415);s.store_sub_from_scalar_scaled_mul_mixed_ia(1432, (7.0 * 1.414213562373095), 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);s.store_square(1433, 1432);}
        s.b[1496] = (s.v[1430] < (s.v[1433] * 1e-8));s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1495]) && s.b[1496]) {s.store_add_scaled_inputs_product_mixed_aaia(1435, A::offset(s.ad_value(1432), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1430), 0.5, s.ad_value(1432), 1.0), 1.0, 1402, A::offset(s.ad_value(1401), (-2.0)), 9.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1495]) && (!s.b[1496])) {s.store_sqrt_add(1434, 1430, 1433);s.store_add_scaled_offset_product_rhs_mixed_aii(1435, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, 1402, 1401, (-2.0), 9.0);}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1495]) {s.store_powf(1436, 1435, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1437, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1402), 12.0)), 1.0, 1436, 2.0, 1436, 1436, 1.414213562373095);s.store_div(1438, 1437, 1436);s.store_add_scaled_product_indices(1439, 1415, (-1.0), 1438, 227, 1.0);s.store_add(1395, 1439, 1415);s.store_div(1396, 1395, 1431);s.store_sqrt_square_offset(1397, 1396, 1.0);s.store_sub_div_lhs_indices(1440, 1395, 1397, 1415);s.store_sub(1396, 1426, 1440);s.store_mul(459, 1403, 1396);s.copy_ad(458, 459);}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_scalar(1438, 3.0);s.store_sub_div_lhs_indices(1441, 1438, 225, 1415);s.store_exp_neg_input(1402, 1438);s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);}
        s.b[1497] = (s.v[1401] < (10.0 * 2.220446049250313e-16));s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1497]) {s.store_scalar(1401, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_add_product3_rhs_mixed_iia(1441, 1426, 1425, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0));s.store_mul_add_rhs(1438, 225, 1441, 1415);s.store_exp_neg_input(1402, 1438);s.store_offset_div_scaled_inputs2_mixed_aia(1401, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1426), s.ad_value(1415))), (-1.0)), 4.0, 1402, 4.0, A::mul(s.ad_value(1425), s.ad_value(226)), 1.0, 1.0);}
        s.b[1498] = (s.v[1401] < (10.0 * 2.220446049250313e-16));s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1498]) {s.store_scalar(1401, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_add_product3_rhs_mixed_iia(1441, 1426, 1425, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1401))), 1.0 / (2.0));s.store_mul_add_rhs(1438, 225, 1441, 1415);}
        s.b[1499] = (s.v[1438] < 3.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1499]) {s.store_scalar(1442, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1443, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1444, 1.0, A::mul(s.ad_value(225), s.ad_value(1424)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1445, 1426, -1.0, 1415, -1.0, 1424, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1446, A::div_scaled_product(A::square(s.ad_value(1443)), s.ad_value(1443), 1.0, A::mul3_scaled_output(s.ad_value(1442), s.ad_value(1442), s.ad_value(1442), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1443), s.ad_value(1444), 1.0, s.ad_value(1442), s.ad_value(1442), 6.0), (-1.0), 1445, 1.0, 1442, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1499]) {s.store_div_scaled_value_by_product_mixed_aii(1447, A::add_scaled_square_product(s.ad_value(1443), (-1.0), s.ad_value(1442), s.ad_value(1444), 3.0), 1.0, 1442, 1442, 9.0);s.store_sqrt_add_scaled_square_cube_product(1398, 1446, 1.0, 1447, 1.0);s.store_powf_ad(1448, A::sub(s.ad_value(1398), s.ad_value(1446)), 0.3333333333333333);s.store_neg_powf_add_input(1449, 1446, 1398, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1401, 1448, 1.0, 1449, 1.0, 1443, 1.0, 1442, 3.0, -1.0);s.store_add_scaled_product_indices(1441, 1415, (-1.0), 1401, 227, 1.0);s.store_mul_add_rhs(1438, 225, 1441, 1415);}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_offset_add(1450, 1426, 1415, 0.1);s.store_offset_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0), 1e-50);s.store_div(1394, 230, 521);s.store_square(1451, 1394);s.store_mul(1452, 1451, 1457);s.store_mul(1394, 226, 1425);s.store_mul(1453, 225, 1450);s.store_add_scaled_inputs_product_mixed_aaii(1454, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);s.store_offset_sub(44, 1453, 1454, (-1.0));s.store_scale(45, 1453, 4.0);}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1454, 1453, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1453, 1453, 1454);s.store_add_scaled_inputs(1453, 1453, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1455, A::ln(A::add_scaled_square_product(s.ad_value(1453), 1.0, s.ad_value(1452), s.ad_value(1394), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1451), s.ad_value(1394))), (-1.0), 225, 1415, 1.0);s.copy_ad(1456, 1438);s.store_offset_sub(44, 1455, 1456, (-(0.0008 * 75.0)));s.store_scale(45, 1455, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1395, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1396, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1438, 1455, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1440, 1438, 225, 1415);s.store_add_offset_lhs_mixed_ia(1395, 1438, (-1.0), A::exp_scaled_input(s.ad_value(1438), -1.0));}
        s.b[1500] = (s.v[1395] < (10.0 * 2.220446049250313e-16));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1500]) {s.store_scalar(1395, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) {s.store_sqrt(1396, 1395);s.store_mul(458, 1423, 1396);s.store_mul_sub_rhs(459, 1403, 1426, 1440);}
        s.b[1501] = (p[42] == 1.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_exp_ad(1457, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1415), -1.0));s.store_div(1394, 230, 521);s.store_square(1451, 1394);s.store_mul(1466, 1451, 1457);}
        let (t7,) = {
    if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {
        (0.0,)
    } else {
        (s.v[1411],)
    }
};
        s.store_scalar(1411, t7);
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tc: usize = 0;
        while {
            let t9: f64 = (2.0 * 20.0);let ta: f64 = (t9 + 1.0);let tb: f64 = if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (s.v[167] <= ta)) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_scalar(1462, 0.0);s.store_mul_add_rhs(1438, 225, 1440, 1415);}
            s.b[1502] = (s.v[1438] < 5.0);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1502]) {s.store_mul3_ad_middle(1458, A::square(s.ad_value(1438)), 1438, A::offset(A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1459, A::square(s.ad_value(1438)), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1460, 1466, 1458, 1458);s.store_mul_product3_indices(1461, 1459, 1466, 225, 1458, 2.0);s.store_mul_scale_offset_mixed_ia(1462, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1463, 1438, A::mul_offset_rhs(s.ad_value(1438), A::mul(s.ad_value(1438), A::scale_offset(s.ad_value(1438), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1464, A::add(A::square(s.ad_value(1462)), s.ad_value(1460)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1465, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1463), s.ad_value(1462), 2.0), 1.0, 1461, 1.0, 1464, 2.0);}
            s.b[1503] = (s.v[1438] < 80.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && s.b[1503]) {s.store_exp(243, 1438);s.store_mul_scale_offset_indices(1460, 1466, 243, 1.0, (-1.0));s.store_mul3_lhs(1461, 1466, 225, 243);}
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) && (!s.b[1503])) {s.store_exp_mul(1467, 225, 1440);s.store_mul_sub_rhs(1460, 1451, 1467, 1457);s.store_mul3_lhs(1461, 1451, 225, 1467);}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1502])) {s.store_sqrt_add_ad(1464, A::offset(s.ad_value(1438), (-1.0)), s.ad_value(1460));s.store_scale_ad(1465, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1461), 1.0, s.ad_value(1464), 1.0), 0.5);}
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_add_scaled_inputs_product_indices(1468, 1426, 1.0, 1440, (-1.0), 1424, 1464, (-1.0));s.store_sub_from_scalar_scaled_mul(1469, (-1.0), 1424, 1465, 1.0);}
            s.b[1504] = (s.v[1411] == 1.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1504]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {s.store_div_scaled_inputs_indices(494, 1468, -1.0, 1469, 1.0);}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {
                s.store_scaled_offset_ad(1470, {
                    if (1.0 >= ((s.v[1440]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1440))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1505] = (((s.v[494]) as f64).abs() > s.v[1470]);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
            if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1505]) {s.store_scale(494, 1470, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) {s.store_add(1440, 1440, 494);}
            s.b[1506] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1468]) as f64).abs() <= 1e-8));s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            let (t8,) = {
    if ((((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1504])) && s.b[1506]) {
        (1.0,)
    } else {
        (s.v[1411],)
    }
};
            s.store_scalar(1411, t8);
            if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1508] = (s.v[1438] < 5.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && s.b[1508]) {s.store_offset_square(1471, 1462, (10.0 * 2.220446049250313e-16));s.store_offset(1472, 1462, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) && (!s.b[1508])) {s.store_offset(1471, 1438, (-1.0));s.store_sqrt(1472, 1471);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (!s.b[1495])) && s.b[1501]) {s.store_mul(458, 1423, 1472);s.store_div_from_scalar_add_ad(1395, 1.0, s.ad_value(1464), s.ad_value(1472));s.store_mul3_lhs(460, 1423, 1460, 1395);s.store_add(459, 458, 460);}
        if ((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) {s.store_sub(460, 459, 458);}
        s.b[1510] = (1.0 == 1.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });s.b[1511] = (1.0 == 2.0);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && s.b[1510]) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1405] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1393] && (p[24] != 0.0)) && s.b[1475]) && (s.b[1511] && (!s.b[1510]))) && (s.v[1406] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p[189]);s.b[1514] = (s.v[145] != 0.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if s.b[1514] {s.store_add(1513, 157, 161);s.store_add_scaled_inputs(314, 1513, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1515] = (p[64] != 0.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if (s.b[1514] && s.b[1515]) {s.store_scalar(315, 0.0);}
        s.b[1516] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (s.b[1514] && s.b[1516]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1517] = (p[64] != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });s.b[1518] = (s.v[246] < 1e-15);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (((!s.b[1514]) && s.b[1517]) && s.b[1518]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1514]) && s.b[1517]) && (!s.b[1518])) {s.store_scale(1512, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1513, 1.0, 244);s.store_mul3_lhs(315, 246, 1512, 1513);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(1530, s.v[91]);s.store_scalar(1531, (1.0 / s.v[1530]));s.store_scalar(1551, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.b[1602] = ((p[29] >= 1.0) && (p[188] > 0.0));s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if ((p[24] != 0.0) && s.b[1602]) {s.store_scalar(1533, p[171]);s.store_scalar(1534, p[172]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((p[24] != 0.0) && s.b[1602]) {s.copy_ad(1535, 158);s.store_scalar(1532, p[188]);}
        s.b[1603] = ((s.v[69] == 0.0) && (p[188] > 0.0));s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (((p[24] != 0.0) && s.b[1602]) && s.b[1603]) {
            if (p[43] == 1.0) {
                s.store_scale(1520, 287, s.v[1530]);
            } else {
                s.store_scale(1520, 108, s.v[1530]);
            }
        }
        if (((p[24] != 0.0) && s.b[1602]) && s.b[1603]) {s.store_mul_ad_product_rhs_mixed_ia(1523, 1533, 1520, A::add(s.ad_value(1534), s.ad_value(1535)));s.store_mul(1524, 1532, 1520);s.copy_ad(1528, 161);s.store_sub_from_scalar(1525, 1.2, 1528);s.store_add_scaled_products_indices(267, 158, 1524, 1.0, 1525, 1523, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1523, 1533, 1520, A::add_scaled_inputs3(s.ad_value(1534), 1.0, s.ad_value(1535), 1.0, s.ad_value(157), -1.0));s.store_sub(1528, 162, 157);s.store_sub_from_scalar(1525, 1.2, 1528);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1524, 1.0, 1523, 1525, (-1.0));}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_mul_sqrt_mixed_ia(1552, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1536, ((1.0 - -1.0) / 2.0));s.store_scalar(1537, ((1.0 + -1.0) / 2.0));}
        s.b[1604] = (p[43] == 1.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1604]) {s.store_add_scaled_products_mixed_iiia(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1549, 1547, 1546);s.store_sub(1551, 1548, 1546);s.store_neg(1550, 1546);s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1536] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) && (s.v[1537] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1604])) {s.store_scalar(1544, 0.0);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_neg(1519, 1544);}
        s.b[1605] = (s.v[1519] > s.v[141]);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1605]) {s.store_sub(1520, 1519, 141);s.store_sub(1521, 140, 141);s.store_div(44, 1520, 1521);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1529, 1521, 1529, -1.0, 1.0);s.store_add(1526, 141, 1529);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1605])) {s.copy_ad(1526, 1519);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));s.store_scale(1553, 1552, s.v[1531]);s.store_square(1554, 1553);s.store_sub_from_scalar(1555, s.v[82], 1551);s.store_div_from_scalar(1519, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (te,) = {
    if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {
        let td: f64 = (-s.v[1545]);
        (td,)
    } else {
        (s.v[1557],)
    }
};
        s.store_scalar(1557, te);s.b[1606] = (s.v[1555] < s.v[1557]);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {s.store_div_scalar_by_product_indices(1520, 1.0, 225, 1552, 1.0);s.store_scale(1529, 1520, s.v[1530]);s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);s.store_sub(1560, 237, 1556);s.store_mul_add_rhs(1528, 225, 1555, 1545);s.store_sub_from_scalar_scaled_mul_mixed_ia(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);s.store_square(1562, 1561);}
        s.b[1607] = (s.v[1559] < (s.v[1562] * 1e-8));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && s.b[1607]) {s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) && (!s.b[1607])) {s.store_sqrt_add(1563, 1559, 1562);s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1606]) {s.store_powf(1565, 1564, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);s.store_div(1567, 1566, 1565);s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);s.store_add(1520, 1568, 1545);s.store_div(1521, 1520, 1560);s.store_sqrt_square_offset(1522, 1521, 1.0);s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);s.store_sub(1521, 1555, 1569);s.store_scale(459, 1521, s.v[1530]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_scalar(1567, 3.0);s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1608] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1608]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1609] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1609]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1610] = (s.v[1567] < 3.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1610]) {s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), 1574, 1.0, 1571, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), 1.0, 1571, 1571, 9.0);s.store_sqrt_add_scaled_square_cube_product(1524, 1575, 1.0, 1576, 1.0);s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);s.store_neg_powf_add_input(1578, 1575, 1524, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1528, 1577, 1.0, 1578, 1.0, 1572, 1.0, 1571, 3.0, -1.0);s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1611] = (p[41] > 0.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_offset_add(1579, 1555, 1545, 0.1);s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1581, 1580, 1586);s.store_mul(1519, 226, 1554);s.store_mul(1582, 225, 1579);s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.store_offset_sub(44, 1582, 1583, (-1.0));s.store_scale(45, 1582, 4.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1582, 1582, 1583);s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.copy_ad(1585, 1567);s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1611]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_add_offset_lhs_mixed_ia(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));}
        s.b[1612] = (s.v[1520] < (10.0 * 2.220446049250313e-16));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1612]) {s.store_scalar(1520, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) {s.store_sqrt(1521, 1520);s.store_mul(458, 1552, 1521);s.store_scaled_sub(459, 1555, 1569, s.v[1530]);}
        s.b[1613] = (p[41] == 1.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1595, 1580, 1586);}
        let (tf,) = {
    if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {
        (0.0,)
    } else {
        (s.v[1542],)
    }
};
        s.store_scalar(1542, tf);
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t14: usize = 0;
        while {
            let t11: f64 = (2.0 * 20.0);let t12: f64 = (t11 + 1.0);let t13: f64 = if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (s.v[167] <= t12)) { 1.0 } else { 0.0 };
            t13 != 0.0
        } {
            t14 += 1;
            if t14 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t14, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_scalar(1591, 0.0);s.store_mul_add_rhs(1567, 225, 1569, 1545);}
            s.b[1614] = (s.v[1567] < 5.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1614]) {s.store_mul3_ad_middle(1587, A::square(s.ad_value(1567)), 1567, A::offset(A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1588, A::square(s.ad_value(1567)), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1589, 1595, 1587, 1587);s.store_mul_product3_indices(1590, 1588, 1595, 225, 1587, 2.0);s.store_mul_scale_offset_mixed_ia(1591, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1592, 1567, A::mul_offset_rhs(s.ad_value(1567), A::mul(s.ad_value(1567), A::scale_offset(s.ad_value(1567), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1593, A::add(A::square(s.ad_value(1591)), s.ad_value(1589)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1594, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1592), s.ad_value(1591), 2.0), 1.0, 1590, 1.0, 1593, 2.0);}
            s.b[1615] = (s.v[1567] < 80.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && s.b[1615]) {s.store_exp(243, 1567);s.store_mul_scale_offset_indices(1589, 1595, 243, 1.0, (-1.0));s.store_mul3_lhs(1590, 1595, 225, 243);}
            if (((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) && (!s.b[1615])) {s.store_exp_mul(1596, 225, 1569);s.store_mul_sub_rhs(1589, 1580, 1596, 1586);s.store_mul3_lhs(1590, 1580, 225, 1596);}
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1614])) {s.store_sqrt_add_ad(1593, A::offset(s.ad_value(1567), (-1.0)), s.ad_value(1589));s.store_scale_ad(1594, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1590), 1.0, s.ad_value(1593), 1.0), 0.5);}
            if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_add_scaled_inputs_product_indices(1597, 1555, 1.0, 1569, (-1.0), 1553, 1593, (-1.0));s.store_sub_from_scalar_scaled_mul(1598, (-1.0), 1553, 1594, 1.0);}
            s.b[1616] = (s.v[1542] == 1.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1616]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {s.store_div_scaled_inputs_indices(494, 1597, -1.0, 1598, 1.0);}
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {
                s.store_scaled_offset_ad(1599, {
                    if (1.0 >= ((s.v[1569]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1569))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1617] = (((s.v[494]) as f64).abs() > s.v[1599]);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1617]) {s.store_scale(494, 1599, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) {s.store_add(1569, 1569, 494);}
            s.b[1618] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1597]) as f64).abs() <= 1e-8));s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            let (t10,) = {
    if (((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1616])) && s.b[1618]) {
        (1.0,)
    } else {
        (s.v[1542],)
    }
};
            s.store_scalar(1542, t10);
            if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1620] = (s.v[1567] < 5.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && s.b[1620]) {s.store_offset_square(1600, 1591, (10.0 * 2.220446049250313e-16));s.store_offset(1601, 1591, (10.0 * 2.220446049250313e-16));}
        if ((((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) && (!s.b[1620])) {s.store_offset(1600, 1567, (-1.0));s.store_sqrt(1601, 1600);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1606])) && s.b[1613]) {s.store_mul(458, 1552, 1601);s.store_div_from_scalar_add_ad(1520, 1.0, s.ad_value(1593), s.ad_value(1601));s.store_mul3_lhs(460, 1552, 1589, 1520);s.store_add(459, 458, 460);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_sub(460, 459, 458);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {
            if (p[43] == 1.0) {
                s.store_mul(1523, 287, 1532);
            } else {
                s.store_mul(1523, 108, 1532);
            }
        }
        s.b[1622] = (((s.v[1538] != 0.0) && (p[43] == 0.0)) || ((s.v[1536] != 0.0) && (p[43] == 1.0)));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1622]) {s.store_mul(455, 1523, 459);s.store_mul(457, 1523, 458);}
        s.b[1623] = (((s.v[1539] != 0.0) && (p[43] == 0.0)) || ((s.v[1537] != 0.0) && (p[43] == 1.0)));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1623]) {s.store_mul(454, 1523, 459);s.store_mul(456, 1523, 458);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_scalar(1536, ((1.0 - 1.0) / 2.0));s.store_scalar(1537, ((1.0 + 1.0) / 2.0));}
        s.b[1624] = (p[43] == 1.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1624]) {s.store_add_scaled_products_mixed_iiia(1546, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1547, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1548, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1549, 1547, 1546);s.store_sub(1551, 1548, 1546);s.store_neg(1550, 1546);s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);s.store_offset_ad(1544, A::add_scaled_products(s.ad_value(1538), s.ad_value(1550), 1.0, s.ad_value(1539), s.ad_value(1549), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {s.store_primal_add_scaled_products_indices(1538, 1536, 461, 1.0, 1537, 462, 1.0);s.store_primal_add_scaled_products_indices(1539, 1536, 462, 1.0, 1537, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1536] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) && (s.v[1537] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1551, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1624])) {s.store_scalar(1544, 0.0);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_neg(1519, 1544);}
        s.b[1625] = (s.v[1519] > s.v[141]);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1625]) {s.store_sub(1520, 1519, 141);s.store_sub(1521, 140, 141);s.store_div(44, 1520, 1521);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1529, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1529, 1521, 1529, -1.0, 1.0);s.store_add(1526, 141, 1529);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1625])) {s.copy_ad(1526, 1519);}
        if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {s.store_offset_scaled(1545, 1526, -1.0, (-1e-12));s.store_scale(1553, 1552, s.v[1531]);s.store_square(1554, 1553);s.store_sub_from_scalar(1555, s.v[82], 1551);s.store_div_from_scalar(1519, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1556, 2.0, 225, A::ln(s.ad_value(1519)));}
        let (t16,) = {
    if (((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) {
        let t15: f64 = (-s.v[1545]);
        (t15,)
    } else {
        (s.v[1557],)
    }
};
        s.store_scalar(1557, t16);s.b[1626] = (s.v[1555] < s.v[1557]);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_div_scalar_by_product_indices(1520, 1.0, 225, 1552, 1.0);s.store_scale(1529, 1520, s.v[1530]);s.store_offset_scaled(1558, 1529, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1559, 1558, 1558, 8.0, 0.0, 1558);s.store_sub(1560, 237, 1556);s.store_mul_add_rhs(1528, 225, 1555, 1545);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(1561, (7.0 * 1.414213562373095), 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);s.store_square(1562, 1561);}
        s.b[1627] = (s.v[1559] < (s.v[1562] * 1e-8));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && s.b[1627]) {s.store_add_scaled_inputs_product_mixed_aaia(1564, A::offset(s.ad_value(1561), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1559), 0.5, s.ad_value(1561), 1.0), 1.0, 1529, A::offset(s.ad_value(1528), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) && (!s.b[1627])) {s.store_sqrt_add(1563, 1559, 1562);s.store_add_scaled_offset_product_rhs_mixed_aii(1564, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, 1529, 1528, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && s.b[1626]) {s.store_powf(1565, 1564, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1566, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1529), 12.0)), 1.0, 1565, 2.0, 1565, 1565, 1.414213562373095);s.store_div(1567, 1566, 1565);s.store_add_scaled_product_indices(1568, 1545, (-1.0), 1567, 227, 1.0);s.store_add(1520, 1568, 1545);s.store_div(1521, 1520, 1560);s.store_sqrt_square_offset(1522, 1521, 1.0);s.store_sub_div_lhs_indices(1569, 1520, 1522, 1545);s.store_sub(1521, 1555, 1569);s.store_scale(459, 1521, s.v[1530]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_scalar(1567, 3.0);s.store_sub_div_lhs_indices(1570, 1567, 225, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1628] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1628]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);s.store_exp_neg_input(1529, 1567);s.store_offset_div_scaled_inputs2_mixed_aia(1528, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1555), s.ad_value(1545))), (-1.0)), 4.0, 1529, 4.0, A::mul(s.ad_value(1554), s.ad_value(226)), 1.0, 1.0);}
        s.b[1629] = (s.v[1528] < (10.0 * 2.220446049250313e-16));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1629]) {s.store_scalar(1528, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_add_product3_rhs_mixed_iia(1570, 1555, 1554, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1528))), 1.0 / (2.0));s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1630] = (s.v[1567] < 3.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {s.store_scalar(1571, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1572, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1573, 1.0, A::mul(s.ad_value(225), s.ad_value(1553)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1574, 1555, -1.0, 1545, -1.0, 1553, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1630]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1575, A::div_scaled_product(A::square(s.ad_value(1572)), s.ad_value(1572), 1.0, A::mul3_scaled_output(s.ad_value(1571), s.ad_value(1571), s.ad_value(1571), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1572), s.ad_value(1573), 1.0, s.ad_value(1571), s.ad_value(1571), 6.0), (-1.0), 1574, 1.0, 1571, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1576, A::add_scaled_square_product(s.ad_value(1572), (-1.0), s.ad_value(1571), s.ad_value(1573), 3.0), 1.0, 1571, 1571, 9.0);s.store_sqrt_add_scaled_square_cube_product(1524, 1575, 1.0, 1576, 1.0);s.store_powf_ad(1577, A::sub(s.ad_value(1524), s.ad_value(1575)), 0.3333333333333333);s.store_neg_powf_add_input(1578, 1575, 1524, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1528, 1577, 1.0, 1578, 1.0, 1572, 1.0, 1571, 3.0, -1.0);s.store_add_scaled_product_indices(1570, 1545, (-1.0), 1528, 227, 1.0);s.store_mul_add_rhs(1567, 225, 1570, 1545);}
        s.b[1631] = (p[41] > 0.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_offset_add(1579, 1555, 1545, 0.1);s.store_offset_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0), 1e-50);s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1581, 1580, 1586);s.store_mul(1519, 226, 1554);s.store_mul(1582, 225, 1579);s.store_add_scaled_inputs_product_mixed_aaii(1583, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.store_offset_sub(44, 1582, 1583, (-1.0));s.store_scale(45, 1582, 4.0);}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1583, 1582, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1582, 1582, 1583);s.store_add_scaled_inputs(1582, 1582, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1584, A::ln(A::add_scaled_square_product(s.ad_value(1582), 1.0, s.ad_value(1581), s.ad_value(1519), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1580), s.ad_value(1519))), (-1.0), 225, 1545, 1.0);s.copy_ad(1585, 1567);s.store_offset_sub(44, 1584, 1585, (-(0.0008 * 75.0)));s.store_scale(45, 1584, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1631]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1520, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1521, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1567, 1584, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_sub_div_lhs_indices(1569, 1567, 225, 1545);s.store_add_offset_lhs_mixed_ia(1520, 1567, (-1.0), A::exp_scaled_input(s.ad_value(1567), -1.0));}
        s.b[1632] = (s.v[1520] < (10.0 * 2.220446049250313e-16));s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1632]) {s.store_scalar(1520, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) {s.store_sqrt(1521, 1520);s.store_mul(458, 1552, 1521);s.store_scaled_sub(459, 1555, 1569, s.v[1530]);}
        s.b[1633] = (p[41] == 1.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_exp_ad(1586, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1545), -1.0));s.store_scale(1519, 230, 1.0 / (s.v[69]));s.store_square(1580, 1519);s.store_mul(1595, 1580, 1586);}
        let (t17,) = {
    if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {
        (0.0,)
    } else {
        (s.v[1542],)
    }
};
        s.store_scalar(1542, t17);
        if (((((p[24] != 0.0) && s.b[1602]) && (!s.b[1603])) && (!s.b[1626])) && s.b[1633]) {s.store_scalar(1589, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(167, 1.0);}
    }
}
