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
            let t1: f64 = (2.0 * 20.0);let t2: f64 = (t1 + 1.0);let t3: f64 = if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (s.v[167] <= t2)) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;
            if t4 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t4, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_scalar(1464, 0.0);s.store_mul_add_rhs(1440, 225, 1442, 1417);}
            s.b[1486] = (s.v[1440] < 5.0);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1486]) {s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1462, 1468, 1460, 1460);s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);s.store_mul_scale_offset_mixed_ia(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);}
            s.b[1487] = (s.v[1440] < 80.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && s.b[1487]) {s.store_exp(243, 1440);s.store_mul_scale_offset_indices(1462, 1468, 243, 1.0, (-1.0));s.store_mul3_lhs(1463, 1468, 225, 243);}
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && (!s.b[1487])) {s.store_exp_mul(1469, 225, 1442);s.store_mul_sub_rhs(1462, 1453, 1469, 1459);s.store_mul3_lhs(1463, 1453, 225, 1469);}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) {s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);}
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);}
            s.b[1488] = (s.v[1413] == 1.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1488]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1489] = (((s.v[494]) as f64).abs() > s.v[1472]);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1489]) {s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {s.store_add(1442, 1442, 494);}
            s.b[1490] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
            let (t0,) = {
    if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1490]) {
        (1.0,)
    } else {
        (s.v[1413],)
    }
};
            s.store_scalar(1413, t0);
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1492] = (s.v[1440] < 5.0);s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1492]) {s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1492])) {s.store_offset(1473, 1440, (-1.0));s.store_sqrt(1474, 1473);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_mul(458, 1425, 1474);s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));s.store_mul3_lhs(460, 1425, 1462, 1397);s.store_add(459, 458, 460);}
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_sub(460, 459, 458);}
        s.b[1494] = (1.0 == 1.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });s.b[1495] = (1.0 == 2.0);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_scalar(1407, ((1.0 - 1.0) / 2.0));s.store_scalar(1408, ((1.0 + 1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1422, 1419, 1418);s.store_neg(1423, 1418);s.store_primal_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);s.store_primal_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1396, 1416);}
        s.b[1496] = (s.v[1396] > s.v[141]);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1496]) {s.store_sub(1397, 1396, 141);s.store_sub(1398, 140, 141);s.store_div(44, 1397, 1398);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1404, 1398, 1404, -1.0, 1.0);s.store_add(1401, 141, 1404);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1496])) {s.copy_ad(1401, 1396);}
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));s.store_mul(1426, 1425, 1406);s.store_square(1427, 1426);s.store_sub(1428, 1424, 523);s.store_div(1396, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));}
        let (t6,) = {
    if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {
        let t5: f64 = (-s.v[1417]);
        (t5,)
    } else {
        (s.v[1430],)
    }
};
        s.store_scalar(1430, t6);s.b[1497] = (s.v[1428] < s.v[1430]);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1497]) {s.store_div_scalar_by_product_indices(1397, 1.0, 225, 1425, 1.0);s.store_mul(1404, 1397, 1405);s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);s.store_sub(1433, 237, 1429);s.store_mul_add_rhs(1403, 225, 1428, 1417);s.store_sub_from_scalar_scaled_mul_mixed_ia(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);s.store_square(1435, 1434);}
        s.b[1498] = (s.v[1432] < (s.v[1435] * 1e-8));s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1497]) && s.b[1498]) {s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1497]) && (!s.b[1498])) {s.store_sqrt_add(1436, 1432, 1435);s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1497]) {s.store_powf(1438, 1437, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);s.store_div(1440, 1439, 1438);s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);s.store_add(1397, 1441, 1417);s.store_div(1398, 1397, 1433);s.store_sqrt_square_offset(1399, 1398, 1.0);s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);s.store_sub(1398, 1428, 1442);s.store_mul(459, 1405, 1398);s.copy_ad(458, 459);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_scalar(1440, 3.0);s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1499] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1499]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1500] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1500]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        s.b[1501] = (s.v[1440] < 3.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {s.store_div_scaled_value_by_product_mixed_aii(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), 1.0, 1444, 1444, 9.0);s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_offset_add(1452, 1428, 1417, 0.1);s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1454, 1453, 1459);s.store_mul(1396, 226, 1427);s.store_mul(1455, 225, 1452);s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.store_offset_sub(44, 1455, 1456, (-1.0));s.store_scale(45, 1455, 4.0);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1455, 1455, 1456);s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.copy_ad(1458, 1440);s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);s.store_add_offset_lhs_mixed_ia(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));}
        s.b[1502] = (s.v[1397] < (10.0 * 2.220446049250313e-16));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1502]) {s.store_scalar(1397, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt(1398, 1397);s.store_mul(458, 1425, 1398);s.store_mul_sub_rhs(459, 1405, 1428, 1442);}
        s.b[1503] = (p[42] == 1.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1468, 1453, 1459);}
        let (t7,) = {
    if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
        (0.0,)
    } else {
        (s.v[1413],)
    }
};
        s.store_scalar(1413, t7);
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut tc: usize = 0;
        while {
            let t9: f64 = (2.0 * 20.0);let ta: f64 = (t9 + 1.0);let tb: f64 = if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (s.v[167] <= ta)) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;
            if tc > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", tc, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_scalar(1464, 0.0);s.store_mul_add_rhs(1440, 225, 1442, 1417);}
            s.b[1504] = (s.v[1440] < 5.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1504]) {s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1462, 1468, 1460, 1460);s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);s.store_mul_scale_offset_mixed_ia(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);}
            s.b[1505] = (s.v[1440] < 80.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && s.b[1505]) {s.store_exp(243, 1440);s.store_mul_scale_offset_indices(1462, 1468, 243, 1.0, (-1.0));s.store_mul3_lhs(1463, 1468, 225, 243);}
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && (!s.b[1505])) {s.store_exp_mul(1469, 225, 1442);s.store_mul_sub_rhs(1462, 1453, 1469, 1459);s.store_mul3_lhs(1463, 1453, 225, 1469);}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) {s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);}
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);}
            s.b[1506] = (s.v[1413] == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1506]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1507] = (((s.v[494]) as f64).abs() > s.v[1472]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1507]) {s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_add(1442, 1442, 494);}
            s.b[1508] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
            let (t8,) = {
    if ((((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1508]) {
        (1.0,)
    } else {
        (s.v[1413],)
    }
};
            s.store_scalar(1413, t8);
            if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1510] = (s.v[1440] < 5.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1510]) {s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1510])) {s.store_offset(1473, 1440, (-1.0));s.store_sqrt(1474, 1473);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_mul(458, 1425, 1474);s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));s.store_mul3_lhs(460, 1425, 1462, 1397);s.store_add(459, 458, 460);}
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_sub(460, 459, 458);}
        s.b[1512] = (1.0 == 1.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });s.b[1513] = (1.0 == 2.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p[189]);s.b[1516] = (s.v[145] != 0.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if s.b[1516] {s.store_add(1515, 157, 161);s.store_add_scaled_inputs(314, 1515, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1517] = (p[64] != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_scalar(315, 0.0);}
        s.b[1518] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1518]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1519] = (p[64] != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });s.b[1520] = (s.v[246] < 1e-15);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (((!s.b[1516]) && s.b[1519]) && s.b[1520]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1516]) && s.b[1519]) && (!s.b[1520])) {s.store_scale(1514, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1515, 1.0, 244);s.store_mul3_lhs(315, 246, 1514, 1515);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(1532, s.v[91]);s.store_scalar(1533, (1.0 / s.v[1532]));s.store_scalar(1553, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.b[1604] = ((p[29] >= 1.0) && (p[188] > 0.0));s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((p[24] != 0.0) && s.b[1604]) {s.store_scalar(1535, p[171]);s.store_scalar(1536, p[172]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((p[24] != 0.0) && s.b[1604]) {s.copy_ad(1537, 158);s.store_scalar(1534, p[188]);}
        s.b[1605] = ((s.v[69] == 0.0) && (p[188] > 0.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((p[24] != 0.0) && s.b[1604]) && s.b[1605]) {
            if (p[43] == 1.0) {
                s.store_scale(1522, 287, s.v[1532]);
            } else {
                s.store_scale(1522, 108, s.v[1532]);
            }
        }
        if (((p[24] != 0.0) && s.b[1604]) && s.b[1605]) {s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add(s.ad_value(1536), s.ad_value(1537)));s.store_mul(1526, 1534, 1522);s.copy_ad(1530, 161);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_indices(267, 158, 1526, 1.0, 1527, 1525, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add_scaled_inputs3(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(157), -1.0));s.store_sub(1530, 162, 157);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1526, 1.0, 1525, 1527, (-1.0));}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_mul_sqrt_mixed_ia(1554, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1538, ((1.0 - -1.0) / 2.0));s.store_scalar(1539, ((1.0 + -1.0) / 2.0));}
        s.b[1606] = (p[43] == 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1538] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) && (s.v[1539] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1606])) {s.store_scalar(1546, 0.0);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_neg(1521, 1546);}
        s.b[1607] = (s.v[1521] > s.v[141]);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1607]) {s.store_sub(1522, 1521, 141);s.store_sub(1523, 140, 141);s.store_div(44, 1522, 1523);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1531, 1523, 1531, -1.0, 1.0);s.store_add(1528, 141, 1531);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1607])) {s.copy_ad(1528, 1521);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));s.store_scale(1555, 1554, s.v[1533]);s.store_square(1556, 1555);s.store_sub_from_scalar(1557, s.v[82], 1553);s.store_div_from_scalar(1521, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (te,) = {
    if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {
        let td: f64 = (-s.v[1547]);
        (td,)
    } else {
        (s.v[1559],)
    }
};
        s.store_scalar(1559, te);s.b[1608] = (s.v[1557] < s.v[1559]);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {s.store_div_scalar_by_product_indices(1522, 1.0, 225, 1554, 1.0);s.store_scale(1531, 1522, s.v[1532]);s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);s.store_sub(1562, 237, 1558);s.store_mul_add_rhs(1530, 225, 1557, 1547);s.store_sub_from_scalar_scaled_mul_mixed_ia(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);s.store_square(1564, 1563);}
        s.b[1609] = (s.v[1561] < (s.v[1564] * 1e-8));s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && s.b[1609]) {s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) && (!s.b[1609])) {s.store_sqrt_add(1565, 1561, 1564);s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1608]) {s.store_powf(1567, 1566, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);s.store_div(1569, 1568, 1567);s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);s.store_add(1522, 1570, 1547);s.store_div(1523, 1522, 1562);s.store_sqrt_square_offset(1524, 1523, 1.0);s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);s.store_sub(1523, 1557, 1571);s.store_scale(459, 1523, s.v[1532]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_scalar(1569, 3.0);s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1610] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1610]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1611] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1611]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1612] = (s.v[1569] < 3.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1612]) {s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), 1.0, 1573, 1573, 9.0);s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1613] = (p[41] > 0.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_offset_add(1581, 1557, 1547, 0.1);s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1583, 1582, 1588);s.store_mul(1521, 226, 1556);s.store_mul(1584, 225, 1581);s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.store_offset_sub(44, 1584, 1585, (-1.0));s.store_scale(45, 1584, 4.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1584, 1584, 1585);s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.copy_ad(1587, 1569);s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1613]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_add_offset_lhs_mixed_ia(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));}
        s.b[1614] = (s.v[1522] < (10.0 * 2.220446049250313e-16));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1614]) {s.store_scalar(1522, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) {s.store_sqrt(1523, 1522);s.store_mul(458, 1554, 1523);s.store_scaled_sub(459, 1557, 1571, s.v[1532]);}
        s.b[1615] = (p[41] == 1.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1597, 1582, 1588);}
        let (tf,) = {
    if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {
        (0.0,)
    } else {
        (s.v[1544],)
    }
};
        s.store_scalar(1544, tf);
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t14: usize = 0;
        while {
            let t11: f64 = (2.0 * 20.0);let t12: f64 = (t11 + 1.0);let t13: f64 = if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (s.v[167] <= t12)) { 1.0 } else { 0.0 };
            t13 != 0.0
        } {
            t14 += 1;
            if t14 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t14, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_scalar(1593, 0.0);s.store_mul_add_rhs(1569, 225, 1571, 1547);}
            s.b[1616] = (s.v[1569] < 5.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1616]) {s.store_mul3_ad_middle(1589, A::square(s.ad_value(1569)), 1569, A::offset(A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1590, A::square(s.ad_value(1569)), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1591, 1597, 1589, 1589);s.store_mul_product3_indices(1592, 1590, 1597, 225, 1589, 2.0);s.store_mul_scale_offset_mixed_ia(1593, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1594, 1569, A::mul_offset_rhs(s.ad_value(1569), A::mul(s.ad_value(1569), A::scale_offset(s.ad_value(1569), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1595, A::add(A::square(s.ad_value(1593)), s.ad_value(1591)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1596, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1594), s.ad_value(1593), 2.0), 1.0, 1592, 1.0, 1595, 2.0);}
            s.b[1617] = (s.v[1569] < 80.0);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && s.b[1617]) {s.store_exp(243, 1569);s.store_mul_scale_offset_indices(1591, 1597, 243, 1.0, (-1.0));s.store_mul3_lhs(1592, 1597, 225, 243);}
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) && (!s.b[1617])) {s.store_exp_mul(1598, 225, 1571);s.store_mul_sub_rhs(1591, 1582, 1598, 1588);s.store_mul3_lhs(1592, 1582, 225, 1598);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1616])) {s.store_sqrt_add_ad(1595, A::offset(s.ad_value(1569), (-1.0)), s.ad_value(1591));s.store_scale_ad(1596, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1592), 1.0, s.ad_value(1595), 1.0), 0.5);}
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_add_scaled_inputs_product_indices(1599, 1557, 1.0, 1571, (-1.0), 1555, 1595, (-1.0));s.store_sub_from_scalar_scaled_mul(1600, (-1.0), 1555, 1596, 1.0);}
            s.b[1618] = (s.v[1544] == 1.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1618]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_div_scaled_inputs_indices(494, 1599, -1.0, 1600, 1.0);}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {
                s.store_scaled_offset_ad(1601, {
                    if (1.0 >= ((s.v[1571]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1571))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1619] = (((s.v[494]) as f64).abs() > s.v[1601]);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
            if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1619]) {s.store_scale(494, 1601, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) {s.store_add(1571, 1571, 494);}
            s.b[1620] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1599]) as f64).abs() <= 1e-8));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
            let (t10,) = {
    if (((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1618])) && s.b[1620]) {
        (1.0,)
    } else {
        (s.v[1544],)
    }
};
            s.store_scalar(1544, t10);
            if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1622] = (s.v[1569] < 5.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && s.b[1622]) {s.store_offset_square(1602, 1593, (10.0 * 2.220446049250313e-16));s.store_offset(1603, 1593, (10.0 * 2.220446049250313e-16));}
        if ((((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) && (!s.b[1622])) {s.store_offset(1602, 1569, (-1.0));s.store_sqrt(1603, 1602);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1608])) && s.b[1615]) {s.store_mul(458, 1554, 1603);s.store_div_from_scalar_add_ad(1522, 1.0, s.ad_value(1595), s.ad_value(1603));s.store_mul3_lhs(460, 1554, 1591, 1522);s.store_add(459, 458, 460);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_sub(460, 459, 458);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {
            if (p[43] == 1.0) {
                s.store_mul(1525, 287, 1534);
            } else {
                s.store_mul(1525, 108, 1534);
            }
        }
        s.b[1624] = (((s.v[1540] != 0.0) && (p[43] == 0.0)) || ((s.v[1538] != 0.0) && (p[43] == 1.0)));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1624]) {s.store_mul(455, 1525, 459);s.store_mul(457, 1525, 458);}
        s.b[1625] = (((s.v[1541] != 0.0) && (p[43] == 0.0)) || ((s.v[1539] != 0.0) && (p[43] == 1.0)));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1625]) {s.store_mul(454, 1525, 459);s.store_mul(456, 1525, 458);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_scalar(1538, ((1.0 - 1.0) / 2.0));s.store_scalar(1539, ((1.0 + 1.0) / 2.0));}
        s.b[1626] = (p[43] == 1.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1626]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);s.store_offset_ad(1546, A::add_scaled_products(s.ad_value(1540), s.ad_value(1552), 1.0, s.ad_value(1541), s.ad_value(1551), 1.0), (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1538] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) && (s.v[1539] != 0.0)) {s.store_add_scaled_products_mixed_iiia(1553, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1626])) {s.store_scalar(1546, 0.0);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_neg(1521, 1546);}
        s.b[1627] = (s.v[1521] > s.v[141]);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1627]) {s.store_sub(1522, 1521, 141);s.store_sub(1523, 140, 141);s.store_div(44, 1522, 1523);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1531, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1531, 1523, 1531, -1.0, 1.0);s.store_add(1528, 141, 1531);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1627])) {s.copy_ad(1528, 1521);}
        if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_offset_scaled(1547, 1528, -1.0, (-1e-12));s.store_scale(1555, 1554, s.v[1533]);s.store_square(1556, 1555);s.store_sub_from_scalar(1557, s.v[82], 1553);s.store_div_from_scalar(1521, s.v[69], 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1558, 2.0, 225, A::ln(s.ad_value(1521)));}
        let (t16,) = {
    if (((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) {
        let t15: f64 = (-s.v[1547]);
        (t15,)
    } else {
        (s.v[1559],)
    }
};
        s.store_scalar(1559, t16);s.b[1628] = (s.v[1557] < s.v[1559]);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_div_scalar_by_product_indices(1522, 1.0, 225, 1554, 1.0);s.store_scale(1531, 1522, s.v[1532]);s.store_offset_scaled(1560, 1531, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1561, 1560, 1560, 8.0, 0.0, 1560);s.store_sub(1562, 237, 1558);s.store_mul_add_rhs(1530, 225, 1557, 1547);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(1563, (7.0 * 1.414213562373095), 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);s.store_square(1564, 1563);}
        s.b[1629] = (s.v[1561] < (s.v[1564] * 1e-8));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && s.b[1629]) {s.store_add_scaled_inputs_product_mixed_aaia(1566, A::offset(s.ad_value(1563), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1561), 0.5, s.ad_value(1563), 1.0), 1.0, 1531, A::offset(s.ad_value(1530), (-2.0)), 9.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) && (!s.b[1629])) {s.store_sqrt_add(1565, 1561, 1564);s.store_add_scaled_offset_product_rhs_mixed_aii(1566, A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), 1.0, 1531, 1530, (-2.0), 9.0);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1628]) {s.store_powf(1567, 1566, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1568, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1531), 12.0)), 1.0, 1567, 2.0, 1567, 1567, 1.414213562373095);s.store_div(1569, 1568, 1567);s.store_add_scaled_product_indices(1570, 1547, (-1.0), 1569, 227, 1.0);s.store_add(1522, 1570, 1547);s.store_div(1523, 1522, 1562);s.store_sqrt_square_offset(1524, 1523, 1.0);s.store_sub_div_lhs_indices(1571, 1522, 1524, 1547);s.store_sub(1523, 1557, 1571);s.store_scale(459, 1523, s.v[1532]);s.copy_ad(458, 459);}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_scalar(1569, 3.0);s.store_sub_div_lhs_indices(1572, 1569, 225, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1630] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1630]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);s.store_exp_neg_input(1531, 1569);s.store_offset_div_scaled_inputs2_mixed_aia(1530, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1557), s.ad_value(1547))), (-1.0)), 4.0, 1531, 4.0, A::mul(s.ad_value(1556), s.ad_value(226)), 1.0, 1.0);}
        s.b[1631] = (s.v[1530] < (10.0 * 2.220446049250313e-16));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1631]) {s.store_scalar(1530, (10.0 * 2.220446049250313e-16));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_add_product3_rhs_mixed_iia(1572, 1557, 1556, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1530))), 1.0 / (2.0));s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1632] = (s.v[1569] < 3.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_scalar(1573, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1574, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1575, 1.0, A::mul(s.ad_value(225), s.ad_value(1555)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1576, 1557, -1.0, 1547, -1.0, 1555, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1632]) {s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1577, A::div_scaled_product(A::square(s.ad_value(1574)), s.ad_value(1574), 1.0, A::mul3_scaled_output(s.ad_value(1573), s.ad_value(1573), s.ad_value(1573), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1574), s.ad_value(1575), 1.0, s.ad_value(1573), s.ad_value(1573), 6.0), (-1.0), 1576, 1.0, 1573, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1578, A::add_scaled_square_product(s.ad_value(1574), (-1.0), s.ad_value(1573), s.ad_value(1575), 3.0), 1.0, 1573, 1573, 9.0);s.store_sqrt_add_scaled_square_cube_product(1526, 1577, 1.0, 1578, 1.0);s.store_powf_ad(1579, A::sub(s.ad_value(1526), s.ad_value(1577)), 0.3333333333333333);s.store_neg_powf_add_input(1580, 1577, 1526, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1530, 1579, 1.0, 1580, 1.0, 1574, 1.0, 1573, 3.0, -1.0);s.store_add_scaled_product_indices(1572, 1547, (-1.0), 1530, 227, 1.0);s.store_mul_add_rhs(1569, 225, 1572, 1547);}
        s.b[1633] = (p[41] > 0.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_offset_add(1581, 1557, 1547, 0.1);s.store_offset_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0), 1e-50);s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1583, 1582, 1588);s.store_mul(1521, 226, 1556);s.store_mul(1584, 225, 1581);s.store_add_scaled_inputs_product_mixed_aaii(1585, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.store_offset_sub(44, 1584, 1585, (-1.0));s.store_scale(45, 1584, 4.0);}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1585, 1584, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1584, 1584, 1585);s.store_add_scaled_inputs(1584, 1584, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1586, A::ln(A::add_scaled_square_product(s.ad_value(1584), 1.0, s.ad_value(1583), s.ad_value(1521), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1582), s.ad_value(1521))), (-1.0), 225, 1547, 1.0);s.copy_ad(1587, 1569);s.store_offset_sub(44, 1586, 1587, (-(0.0008 * 75.0)));s.store_scale(45, 1586, (4.0 * (0.0008 * 75.0)));}
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1633]) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1522, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1523, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1569, 1586, 1.0, 44, (-0.5), 45, (-0.5));}
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sub_div_lhs_indices(1571, 1569, 225, 1547);s.store_add_offset_lhs_mixed_ia(1522, 1569, (-1.0), A::exp_scaled_input(s.ad_value(1569), -1.0));}
        s.b[1634] = (s.v[1522] < (10.0 * 2.220446049250313e-16));s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1634]) {s.store_scalar(1522, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) {s.store_sqrt(1523, 1522);s.store_mul(458, 1554, 1523);s.store_scaled_sub(459, 1557, 1571, s.v[1532]);}
        s.b[1635] = (p[41] == 1.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_exp_ad(1588, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1547), -1.0));s.store_scale(1521, 230, 1.0 / (s.v[69]));s.store_square(1582, 1521);s.store_mul(1597, 1582, 1588);}
        let (t17,) = {
    if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {
        (0.0,)
    } else {
        (s.v[1544],)
    }
};
        s.store_scalar(1544, t17);
        if (((((p[24] != 0.0) && s.b[1604]) && (!s.b[1605])) && (!s.b[1628])) && s.b[1635]) {s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(167, 1.0);}
    }
}
