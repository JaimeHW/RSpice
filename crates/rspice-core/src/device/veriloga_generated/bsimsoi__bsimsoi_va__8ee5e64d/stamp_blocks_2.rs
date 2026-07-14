#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
    ) {
        if s.b[1597] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1408), 200.0));s.store_mul_scale_offset_indices(1408, 1188, 1408, -1.0, 0.02);}
        if (s.v[68] != 0.0) {s.store_scaled_offset_ad(1300, A::sub_from_scalar((s.v[79] - s.v[80]), A::scale(s.ad_value(1247), 0.5)), 0.45, (2.0 * s.v[36]));s.store_scalar(1442, ((s.v[72] * s.v[74]) / 3.9));}
        if (s.v[68] == 0.0) {s.store_scalar(1300, 0.0);s.store_scalar(1442, s.v[91]);}
        s.b[1598] = (s.v[89] == 1.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if s.b[1598] {s.store_add_scaled_inputs4_indices(1179, 1210, 1.0, 1165, 1.0, 1165, 1.0, 1300, -1.0);s.store_add_scaled_product_indices(1181, 1291, 1.0, 1293, 1177, 1.0);s.store_div(1182, 1179, 1442);s.store_mul_add_scaled_product_rhs_indices(1184, 1182, 1181, 1.0, 1292, 1182, 1.0);}
        s.b[1599] = (s.v[89] == 2.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((!s.b[1598]) && s.b[1599]) {s.store_mul_add(1184, A::div_scaled_inputs2(s.ad_value(1210), 1.0, s.ad_value(1300), (-1.0), s.ad_value(776), 1.0), A::add_scaled_product(s.ad_value(1291), 1.0, s.ad_value(1293), s.ad_value(1177), 1.0), A::div_scaled_product(s.ad_value(1292), A::sub(s.ad_value(1210), s.ad_value(1300)), 1.0, s.ad_value(776), 1.0));}
        s.b[1600] = (s.v[89] == 3.0);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if (((!s.b[1598]) && (!s.b[1599])) && s.b[1600]) {s.store_add_scaled_inputs4_indices(1179, 1210, 1.0, 1165, 1.0, 1165, 1.0, 1300, -1.0);s.store_offset_mul(1181, 1293, 1177, 1.0);s.store_div(1182, 1179, 1442);s.store_mul_add_scaled_product_rhs_indices(1183, 1182, 1291, 1.0, 1292, 1182, 1.0);s.store_mul(1184, 1183, 1181);}
        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {s.store_scale_ad(1179, A::div_scaled_inputs2(s.ad_value(1210), 1e-8, s.ad_value(425), 1e-8, s.ad_value(776), 1.0), 0.16666666666666666);}
        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1180, A::mul(s.ad_value(518), {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }
        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {s.store_add_scaled_product_indices(1181, 1291, 1.0, 1293, 1177, 1.0);s.store_mul_pow_indices(1490, 519, 771, 520);s.store_mul_pow_indices(1491, 516, 771, 517);s.copy_ad(1441, 426);}
        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_exp_ad(1189, A::mul(s.ad_value(1490), {
                if ((1.0 + (s.v[1210] / s.v[1441])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(1210), s.ad_value(1441)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }
        if (((!s.b[1598]) && (!s.b[1599])) && (!s.b[1600])) {s.store_div(1190, 1491, 1189);s.store_add_scaled_product_indices(1184, 1190, 1.0, 1180, 1181, 1.0);}
        s.b[1601] = (s.v[1184] >= (-0.8));s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if s.b[1601] {s.store_offset(1271, 1184, 1.0);}
        if (!s.b[1601]) {s.store_div_from_scalar_offset_scaled_input(1188, 1.0, 1184, 10.0, 7.0);s.store_mul_scale_offset_indices(1271, 1188, 1184, 1.0, 0.6);}
        s.store_div(1171, 1280, 1271);s.copy_ad(410, 1171);s.store_mul3_lhs(1223, 1228, 1281, 757);s.store_mul(1224, 1223, 1222);s.store_div_scaled_inputs_indices(1172, 1281, 2.0, 1171, 1.0);s.store_scale(1174, 1172, s.v[1227]);s.b[1602] = (s.v[475] == 0.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if s.b[1602] {s.copy_ad(1209, 476);}
        s.b[1603] = (s.v[475] > 0.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if ((!s.b[1602]) && s.b[1603]) {s.store_sub_from_scalar(1179, 1.0, 476);s.store_offset_add_scaled_product_indices(1180, 1179, 1.0, 475, 1210, (-1.0), (-0.0001));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.0004);s.store_add_scaled_inputs4_indices(1209, 476, 1.0, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));}
        if ((!s.b[1602]) && (!s.b[1603])) {s.store_offset_add_scaled_product_indices(1180, 476, 1.0, 475, 1210, 1.0, (-0.0001));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 476, 0.0004);s.store_scaled_add(1209, 1180, 1181, 0.5);}
        s.b[1604] = ((s.v[1222] == 0.0) && (s.v[1209] == 1.0));s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
    ) {
        if s.b[1604] {s.store_div_from_scalar_ad(1179, 1.0, A::add_scaled_product(s.ad_value(1225), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0));s.store_mul(1182, 1174, 1225);s.store_mul(1173, 1182, 1179);}
        if (!s.b[1604]) {s.store_mul(1188, 1195, 1224);s.store_mul(1186, 1225, 1188);s.store_mul(1185, 1225, 1224);s.store_mul_add_scaled_inputs_rhs(1179, 1195, A::offset(s.ad_value(1188), (-1.0)), 2.0, A::div_from_scalar(1.0, s.ad_value(1209)), 2.0);s.store_add_scaled_inputs_mixed_ai(1180, A::add_scaled_products(s.ad_value(1225), A::offset(A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0)), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0), 1.0, 1186, 3.0);s.store_mul_add_scaled_inputs_rhs_indices(1181, 1225, 1174, 1.0, 1185, 2.0);s.store_sqrt_add_scaled_square_product(1182, 1180, 1.0, 1179, 1181, (-2.0));s.store_div_scaled_inputs2_indices(1173, 1180, 1.0, 1182, (-1.0), 1179, 1.0);}
        s.store_add_scaled_inputs3_indices(1180, 1173, 1.0, 1158, (-1.0), 550, -1.0);s.store_sqrt_add_scaled_square_product(1181, 1180, 1.0, 550, 1173, 4.0);s.store_add_scaled_inputs3_indices(1211, 1173, 1.0, 1180, (-0.5), 1181, (-0.5));s.b[1605] = (s.v[1211] > s.v[1158]);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if s.b[1605] {s.copy_ad(1211, 1158);}
        s.store_sub(1213, 1158, 1211);s.store_sub_from_scalar_ad(1207, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1173), 0.5, s.ad_value(1225), 1.0));s.store_mul(1188, 1224, 1210);s.store_add_scaled_inputs_product_indices(1179, 1174, 1.0, 1173, 1.0, 1188, 1207, 2.0);s.store_mul(1188, 1224, 1195);s.store_add_offset_lhs_mixed_ai(1180, A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0), 1188);s.store_div(1176, 1179, 1180);s.b[1606] = ((s.v[560] > 0.0) && (s.v[1213] > 1e-10));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if s.b[1606] {s.store_div_from_scalar_ad(1179, 1.0, A::mul3(s.ad_value(560), s.ad_value(1195), s.ad_value(489)));s.store_div(1181, 1210, 1174);s.store_scaled_add(1180, 1195, 1181, s.v[1227]);s.store_mul(1188, 1179, 1180);s.store_mul(1197, 1188, 1213);}
        if (!s.b[1606]) {s.store_scalar(1197, 2.688117142e43);}
        s.b[1607] = (s.v[1475] > 0.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if s.b[1607] {s.store_mul(1187, 1195, 1173);s.store_mul(1179, 1225, 1187);s.store_add(1180, 1225, 1187);s.copy_ad(1181, 1475);s.store_div_scaled_inputs2_mixed_iai(1198, 1225, 1.0, A::div(s.ad_value(1179), s.ad_value(1180)), (-1.0), 1181, 1.0);s.store_mul(1186, 563, 1177);}
        s.b[1608] = (s.v[1186] >= (-0.9));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if (s.b[1607] && s.b[1608]) {s.store_div_from_scalar_offset_input(1182, 1.0, 1186, 1.0);s.store_mul(1198, 1198, 1182);}
        if (s.b[1607] && (!s.b[1608])) {s.store_div_from_scalar_offset_input(1183, 1.0, 1186, 0.8);s.store_mul_scale_offset_rhs(1182, 1183, 1186, 20.0, 17.0);s.store_mul(1198, 1198, 1182);}
        if (!s.b[1607]) {s.store_scalar(1198, 2.688117142e43);}
        s.store_mul(1179, 748, 1158);s.b[1609] = (s.v[1179] > 100.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if s.b[1609] {s.store_scalar(1180, 2.688117142e43);}
        if (!s.b[1609]) {s.store_exp(1180, 1179);}
        s.b[1610] = (s.v[747] > 3.720075976e-44);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if s.b[1610] {s.store_scalar(1181, (1.0 + (s.v[273] * s.v[1227])));s.store_div_scaled_offset_numerator_mixed_ai(1427, A::mul(s.ad_value(1181), s.ad_value(1180)), 1.0, 1.0, 747, 1.0);s.store_mul(1427, 1427, 1426);}
        if (!s.b[1610]) {s.store_scalar(1427, 2.688117142e43);}
        s.store_div(1187, 564, 1174);s.store_mul(1188, 1187, 1210);s.b[1611] = (s.v[1188] > (-0.9));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if s.b[1611] {s.store_offset(1179, 1188, 1.0);}
        if (!s.b[1611]) {s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1188, 20.0, 17.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        if (!s.b[1611]) {s.store_mul_scale_offset_indices(1179, 1180, 1188, 1.0, 0.8);}
        s.store_add(1206, 1197, 1198);s.store_div_scaled_product_indices(1180, 1197, 1198, 1.0, 1206, 1.0);s.store_add(1206, 1180, 1427);s.store_div_scaled_product_indices(1181, 1180, 1427, 1.0, 1206, 1.0);s.store_add_scaled_product_indices(1175, 1176, 1.0, 1179, 1181, 1.0);s.store_scaled_mul(1221, 757, 1228, 1.0 / (s.v[1227]));s.store_mul(1215, 1171, 1221);s.store_sub_from_scalar_ad(1179, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0));s.store_mul(1217, 1210, 1179);s.store_div(1188, 1211, 1174);s.store_offset(1218, 1188, 1.0);s.store_div_scaled_product_indices(1216, 1215, 1217, 1.0, 1218, 1.0);s.store_offset_mul(1179, 1216, 1222, 1.0);s.store_div(1188, 1211, 1179);s.store_mul(1219, 1216, 1188);s.store_div(1419, 1216, 1179);s.store_div(1188, 1213, 1175);s.store_offset(1179, 1188, 1.0);s.store_scaled_mul(1220, 1219, 1179, 1.0 / (s.v[59]));s.store_scaled_mul(454, 1419, 1179, 1.0 / (s.v[59]));s.b[1612] = (s.v[454] < 1e-9);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if s.b[1612] {s.store_scalar(454, 1e-9);}
        s.store_scaled_mul(1420, 1419, 1179, 1.0 / (s.v[59]));s.b[1613] = (s.v[57] != 2.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });s.b[1614] = (s.v[68] == 0.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1614]) {s.store_mul_div_from_scalar_lhs_ad_indices(1179, (3.0 * 3.9), 777, 776);}
        if (s.b[1613] && (!s.b[1614])) {s.store_div_scaled_inputs_indices(1179, 776, s.v[74], 777, 1.0);}
        s.b[1615] = (s.v[70] == 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });s.b[1616] = (s.v[68] == 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1616]) {s.store_div_scaled_inputs3_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 1179, 1.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1616])) {s.store_div_scaled_inputs4_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 736, 1.0, 1179, 1.0);}
        s.b[1617] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1617]) {s.store_scalar(1241, 0.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1617])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_mul_product3_mixed_aiii(1241, A::exp_scaled_input(s.ad_value(1181), -1.0), 1330, 1483, 1180, 1.0);s.store_square(1183, 1160);s.store_mul_scale_offset_indices(1184, 1183, 1160, -1.0, 0.0);s.store_offset_add_ad(1185, s.ad_value(1485), A::abs(s.ad_value(1184)), 1e-9);s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));s.store_mul(1241, 1241, 1186);}
        s.b[1618] = (s.v[68] == 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1618]) {s.store_div_scaled_inputs3_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 1179, 1.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1618])) {s.store_div_scaled_inputs4_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 736, 1.0, 1179, 1.0);}
        s.b[1619] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1619]) {s.store_scalar(1240, 0.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1619])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        if ((s.b[1613] && s.b[1615]) && (!s.b[1619])) {s.store_mul_product3_mixed_aiii(1240, A::exp_scaled_input(s.ad_value(1181), -1.0), 1331, 1476, 1180, 1.0);s.store_square(1183, 1235);s.store_mul_scale_offset_indices(1184, 1183, 1235, -1.0, 0.0);s.store_offset_add_ad(1185, s.ad_value(1478), A::abs(s.ad_value(1184)), 1e-9);s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));s.store_mul(1240, 1240, 1186);}
        s.b[1620] = (s.v[68] == 0.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1620]) {s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 1179, 1.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1620])) {s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 736, 1.0, 1179, 1.0);}
        s.b[1621] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1621]) {s.store_scalar(1241, 0.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_mul_product3_mixed_aiii(1241, A::exp_scaled_input(s.ad_value(1181), -1.0), 1330, 1483, 1180, 1.0);s.store_sub(1183, 1160, 1489);}
        s.b[1622] = (s.v[1183] >= ((-1.0) / 100.0));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && s.b[1622]) {s.store_scale(1184, 1488, (-100.0));}
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && (!s.b[1622])) {s.store_div(1184, 1488, 1183);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {s.store_exp(1185, 1184);s.store_mul(1241, 1241, 1185);}
        s.b[1623] = (s.v[68] == 0.0);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1623]) {s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 1179, 1.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1623])) {s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 736, 1.0, 1179, 1.0);}
        s.b[1624] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1624]) {s.store_scalar(1240, 0.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_mul_product3_mixed_aiii(1240, A::exp_scaled_input(s.ad_value(1181), -1.0), 1331, 1476, 1180, 1.0);s.store_sub(1183, 1235, 1482);}
        s.b[1625] = (s.v[1183] >= ((-1.0) / 100.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && s.b[1625]) {s.store_scale(1184, 1481, (-100.0));}
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && (!s.b[1625])) {s.store_div(1184, 1481, 1183);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {s.store_exp(1185, 1184);s.store_mul(1240, 1240, 1185);}
        if s.b[1613] {s.store_scalar(1309, (s.v[708] * s.v[174]));s.store_scalar(1310, (s.v[709] * s.v[174]));s.store_mul(1266, 1168, 661);s.store_div(1179, 1421, 1266);}
        s.b[1626] = (s.v[1179] > 100.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1626]) {s.store_scaled_offset(1318, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1627] = (s.v[1179] < (-100.0));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1626])) && s.b[1627]) {s.store_scalar(1318, 3.720075976e-44);}
        if ((s.b[1613] && (!s.b[1626])) && (!s.b[1627])) {s.store_exp(1318, 1179);}
        if s.b[1613] {s.store_mul(1266, 1168, 662);s.store_div(1179, 1422, 1266);}
        s.b[1628] = (s.v[1179] > 100.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1628]) {s.store_scaled_offset(1319, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1629] = (s.v[1179] < (-100.0));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1628])) && s.b[1629]) {s.store_scalar(1319, 3.720075976e-44);}
        if ((s.b[1613] && (!s.b[1628])) && (!s.b[1629])) {s.store_exp(1319, 1179);}
        s.b[1630] = (s.v[1282] == 0.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1630]) {s.store_scalar(1261, 0.0);}
        if (s.b[1613] && (!s.b[1630])) {s.store_mul(1179, 1309, 1282);s.store_mul_scale_offset_indices(1261, 1179, 1318, 1.0, (-1.0));}
        s.b[1631] = (s.v[1283] == 0.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1631]) {s.store_scalar(1257, 0.0);}
        if (s.b[1613] && (!s.b[1631])) {s.store_mul(1179, 1310, 1283);s.store_mul_scale_offset_indices(1257, 1179, 1319, 1.0, (-1.0));}
        s.b[1632] = (s.v[1286] == 0.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1632]) {s.store_scalar(1262, 0.0);}
        if (s.b[1613] && (!s.b[1632])) {s.store_mul_scaled_offset_ad_rhs(1305, 663, s.v[783], A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(1306, 665, s.v[783], A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0);s.store_div(1179, 1421, 1305);}
        s.b[1633] = (s.v[1179] > 100.0);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1632])) && s.b[1633]) {s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1634] = (s.v[1179] < (-100.0));s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {s.store_scalar(1189, 3.720075976e-44);}
        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1634])) {s.store_exp(1189, 1179);}
        s.b[1635] = ((s.v[675] - s.v[1421]) < 0.001);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1306, 1.0, 675, 1180);}
        s.b[1636] = (s.v[1179] > 100.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1632])) && s.b[1635]) && s.b[1636]) {s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1637] = (s.v[1179] < (-100.0));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && s.b[1637]) {s.store_scalar(1190, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1632])) && s.b[1635]) && (!s.b[1636])) && (!s.b[1637])) {s.store_exp(1190, 1179);}
        if ((s.b[1613] && (!s.b[1632])) && s.b[1635]) {s.store_neg(1190, 1190);}
        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(675), s.ad_value(1421));s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1306, 1.0, 675, 1180);}
        s.b[1638] = (s.v[1179] > 100.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && s.b[1638]) {s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1639] = (s.v[1179] < (-100.0));s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && s.b[1639]) {s.store_scalar(1190, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) && (!s.b[1638])) && (!s.b[1639])) {s.store_exp(1190, 1179);}
        if ((s.b[1613] && (!s.b[1632])) && (!s.b[1635])) {s.store_neg(1190, 1190);}
        if (s.b[1613] && (!s.b[1632])) {s.store_mul(1182, 1309, 1286);s.store_mul_add_rhs(1262, 1182, 1189, 1190);}
        s.b[1640] = (s.v[1287] == 0.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1640]) {s.store_scalar(1258, 0.0);}
        if (s.b[1613] && (!s.b[1640])) {s.store_mul_scaled_offset_ad_rhs(1305, 664, s.v[783], A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(1306, 666, s.v[783], A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0);s.store_div(1179, 1422, 1305);}
        s.b[1641] = (s.v[1179] > 100.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1640])) && s.b[1641]) {s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1642] = (s.v[1179] < (-100.0));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {s.store_scalar(1189, 3.720075976e-44);}
        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && (!s.b[1642])) {s.store_exp(1189, 1179);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
    ) {
        s.b[1643] = ((s.v[676] - s.v[1422]) < 0.001);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1306, 1.0, 676, 1180);}
        s.b[1644] = (s.v[1179] > 100.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1640])) && s.b[1643]) && s.b[1644]) {s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1645] = (s.v[1179] < (-100.0));s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && s.b[1645]) {s.store_scalar(1190, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1640])) && s.b[1643]) && (!s.b[1644])) && (!s.b[1645])) {s.store_exp(1190, 1179);}
        if ((s.b[1613] && (!s.b[1640])) && s.b[1643]) {s.store_neg(1190, 1190);}
        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(676), s.ad_value(1422));s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1306, 1.0, 676, 1180);}
        s.b[1646] = (s.v[1179] > 100.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && s.b[1646]) {s.store_scaled_offset(1190, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1647] = (s.v[1179] < (-100.0));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && s.b[1647]) {s.store_scalar(1190, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) && (!s.b[1646])) && (!s.b[1647])) {s.store_exp(1190, 1179);}
        if ((s.b[1613] && (!s.b[1640])) && (!s.b[1643])) {s.store_neg(1190, 1190);}
        if (s.b[1613] && (!s.b[1640])) {s.store_mul(1182, 1310, 1287);s.store_mul_add_rhs(1258, 1182, 1189, 1190);}
        if s.b[1613] {s.store_scalar(1265, ((s.v[689] / s.v[59]) * s.v[174]));}
        s.b[1648] = ((s.v[1284] == 0.0) && (s.v[1285] == 0.0));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1648]) {s.store_scalar(1263, 0.0);s.store_scalar(1259, 0.0);s.store_scalar(1322, 0.0);s.store_scalar(1323, 0.0);s.store_scalar(1268, 0.0);}
        if (s.b[1613] && (!s.b[1648])) {s.store_mul_scale_offset_indices(1324, 1307, 1318, 1.0, (-1.0));}
        s.b[1649] = (s.v[1324] < 1e-5);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1649]) {s.store_scalar(1324, 0.0);s.store_scalar(1326, 1.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1649])) {s.store_div_from_scalar_sqrt_ad(1326, 1.0, A::offset(s.ad_value(1324), 1.0));}
        if (s.b[1613] && (!s.b[1648])) {s.store_mul_scale_offset_indices(1325, 1308, 1319, 1.0, (-1.0));}
        s.b[1650] = (s.v[1325] < 1e-5);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1650]) {s.store_scalar(1325, 0.0);s.store_scalar(1327, 1.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1650])) {s.store_div_from_scalar_sqrt_ad(1327, 1.0, A::offset(s.ad_value(1325), 1.0));}
        if (s.b[1613] && (!s.b[1648])) {s.store_sub_from_scalar(1179, 1.0, 712);s.store_mul3_lhs(1320, 1265, 1284, 713);s.store_mul(1180, 1179, 1320);s.store_mul_ad_product_lhs_mixed_ia(1263, 1180, A::offset(s.ad_value(1318), (-1.0)), 1326);s.store_mul3_lhs(1320, 1265, 1285, 713);s.store_mul(1180, 1179, 1320);s.store_mul_ad_product_lhs_mixed_ia(1259, 1180, A::offset(s.ad_value(1319), (-1.0)), 1327);s.store_mul3_lhs(1321, 1265, 1284, 714);s.store_mul_ad_product_lhs_mixed_ia(1322, 1321, A::offset(s.ad_value(1318), (-1.0)), 1326);s.store_mul3_lhs(1321, 1265, 1285, 714);s.store_mul_ad_product_lhs_mixed_ia(1323, 1321, A::offset(s.ad_value(1319), (-1.0)), 1327);}
        s.b[1651] = (s.v[49] == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1651]) {s.store_scalar(1268, 0.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {s.store_offset_div_scaled_inputs2_indices(1179, 1421, 1.0, 1422, 1.0, 715, 1.0, 1.0);s.store_add(1180, 1324, 1325);s.store_sqrt_add_scaled_square_input(1182, 1179, 1.0, 1180, 4.0);s.store_scaled_add(1181, 1179, 1182, 0.5);}
        s.b[1652] = (s.v[1181] < 0.1);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && s.b[1652]) {s.store_scalar(1328, 10.0);}
        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && (!s.b[1652])) {s.store_div_from_scalar(1328, 1.0, 1181);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {s.store_mul(1179, 712, 1320);s.store_mul_ad_product_lhs_mixed_ia(1268, 1179, A::sub(s.ad_value(1318), s.ad_value(1319)), 1328);}
        s.b[1653] = ((s.v[1288] == 0.0) && (s.v[1289] == 0.0));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1653]) {s.store_scalar(1260, 0.0);s.store_scalar(1264, 0.0);}
        if (s.b[1613] && (!s.b[1653])) {s.store_scale(1267, 659, s.v[783]);}
        s.b[1654] = ((s.v[677] - s.v[1421]) < 0.001);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);}
        s.b[1655] = (s.v[1179] > 100.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
    ) {
        if (((s.b[1613] && (!s.b[1653])) && s.b[1654]) && s.b[1655]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1656] = (s.v[1179] < (-100.0));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {s.store_mul(1182, 1309, 1288);s.store_mul_scale_offset_indices(1264, 1182, 1180, -1.0, 1.0);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(677), s.ad_value(1421));s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);}
        s.b[1657] = (s.v[1179] > 100.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && s.b[1657]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1658] = (s.v[1179] < (-100.0));s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && s.b[1658]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && (!s.b[1658])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {s.store_mul(1182, 1309, 1288);s.store_mul_scale_offset_indices(1264, 1182, 1180, -1.0, 1.0);}
        if (s.b[1613] && (!s.b[1653])) {s.store_scale(1267, 660, s.v[783]);}
        s.b[1659] = ((s.v[678] - s.v[1422]) < 0.001);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);}
        s.b[1660] = (s.v[1179] > 100.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && s.b[1659]) && s.b[1660]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1661] = (s.v[1179] < (-100.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && s.b[1661]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && (!s.b[1661])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {s.store_mul(1182, 1310, 1289);s.store_mul_scale_offset_indices(1260, 1182, 1180, -1.0, 1.0);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(678), s.ad_value(1422));s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);}
        s.b[1662] = (s.v[1179] > 100.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && s.b[1662]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1663] = (s.v[1179] < (-100.0));s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && s.b[1663]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && (!s.b[1663])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {s.store_mul(1182, 1310, 1289);s.store_mul_scale_offset_indices(1260, 1182, 1180, -1.0, 1.0);}
        if s.b[1613] {s.store_add_scaled_inputs4_indices(1269, 1261, 1.0, 1262, 1.0, 1263, 1.0, 1264, 1.0);s.store_add_scaled_inputs4_indices(1270, 1257, 1.0, 1258, 1.0, 1259, 1.0, 1260, 1.0);}
        if (!s.b[1613]) {s.store_scalar(1240, 0.0);s.store_scalar(1241, 0.0);s.store_scalar(1269, 0.0);s.store_scalar(1270, 0.0);s.store_scalar(1322, 0.0);s.store_scalar(1323, 0.0);s.store_scalar(1268, 0.0);}
        s.b[1664] = ((s.v[355] != 0.0) || (s.v[356] != 0.0));s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if s.b[1664] {s.store_sub(1409, 1161, 1160);s.store_add_scaled_inputs_product_indices(1162, 768, s.v[36], 1277, (-1.0), 707, 1278, (-1.0));s.store_add_scaled_inputs3_offset_indices(1182, 1162, 1.0, 1161, (-1.0), 1160, 1.0, (-0.02));}
        s.b[1665] = (s.v[1162] <= 0.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if (s.b[1664] && s.b[1665]) {s.store_sqrt_add_scaled_square_input(1179, 1182, 1.0, 1162, (-(4.0 * 0.02)));}
        if (s.b[1664] && (!s.b[1665])) {s.store_sqrt_add_scaled_square_input(1179, 1182, 1.0, 1162, (4.0 * 0.02));}
        if s.b[1664] {s.store_add_scaled_inputs3_indices(1148, 1162, 1.0, 1182, (-0.5), 1179, (-0.5));s.store_sub(1415, 1162, 1148);}
        s.b[1666] = (s.v[1415] < 0.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (s.b[1664] && s.b[1666]) {s.store_scalar(1415, 0.0);}
        s.b[1667] = (s.v[737] == 0.0);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (s.b[1664] && s.b[1667]) {s.store_scalar(1416, 0.0);}
        if (s.b[1664] && (!s.b[1667])) {s.store_add_scaled_inputs4_indices(1179, 1161, 1.0, 1210, (-1.0), 1148, -1.0, 1177, -1.0);}
        s.b[1668] = (s.v[1179] < 0.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((s.b[1664] && (!s.b[1667])) && s.b[1668]) {s.store_div(1180, 1179, 737);}
        if ((s.b[1664] && (!s.b[1667])) && (!s.b[1668])) {s.store_mul_scaled_offset_ad_rhs(1180, 737, 1.0 / (2.0), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1179), 4.0, s.ad_value(737), s.ad_value(737), 1.0), 1.0)), (-1.0));}
        if (s.b[1664] && (!s.b[1667])) {s.store_add_scaled_inputs4_mixed_iaii(1416, 1161, 1.0, A::square(s.ad_value(1180)), -1.0, 1160, -1.0, 1162, -1.0);}
        if (!s.b[1664]) {s.store_scalar(1162, 0.0);s.store_scalar(1409, 0.0);s.store_scalar(1415, 0.0);s.store_scalar(1416, 0.0);}
        if (s.v[356] != 0.0) {s.store_mul(1179, 1168, 578);s.store_div_scaled_inputs2_indices(1362, 1161, 1.0, 768, (-s.v[36]), 1179, 1.0);}
        s.b[1669] = (s.v[1362] > 100.0);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1669]) {s.store_sub_scaled_inputs(1412, 1161, 1.0, 768, s.v[36]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
    ) {
        s.b[1670] = (s.v[1362] < (-100.0));s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1669])) && s.b[1670]) {s.store_scale(1412, 1179, (((1.0 + 3.720075976e-44)) as f64).ln());}
        if (((s.v[356] != 0.0) && (!s.b[1669])) && (!s.b[1670])) {s.store_exp(1363, 1362);s.store_mul_ln_mixed_ia(1412, 1179, A::offset(s.ad_value(1363), 1.0));}
        if (s.v[356] != 0.0) {s.store_mul(1181, 1161, 1412);s.copy_ad(1190, 730);s.copy_ad(1191, 731);s.store_add_scaled_product_indices(1182, 573, (-1.0), 572, 574, 1.0);s.store_mul(1183, 573, 574);s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(572), 1.0, s.ad_value(1182), s.ad_value(1416), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1416), s.ad_value(1416)));}
        s.b[1671] = (s.v[1184] > 100.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1671]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1672] = (s.v[1184] < (-100.0));s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1671])) && s.b[1672]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1671])) && (!s.b[1672])) {s.store_exp(1185, 1184);}
        if (s.v[356] != 0.0) {s.store_mul3_lhs(1355, 1190, 1181, 1185);s.store_mul_scale_offset_indices(1186, 1158, 579, -1.0, 0.0);s.store_offset_square(1187, 1186, 0.0002);}
        s.b[1673] = (s.v[1186] > 100.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1673]) {s.store_scalar(1188, 2.688117142e43);}
        s.b[1674] = (s.v[1186] < (-100.0));s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1673])) && s.b[1674]) {s.store_scalar(1188, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1673])) && (!s.b[1674])) {s.store_exp(1188, 1186);}
        if (s.v[356] != 0.0) {s.store_offset(1180, 1188, (((-1.0)) + (0.0001)));s.store_div_scaled_inputs2_indices(1189, 1180, 1.0, 1186, (-1.0), 1187, 1.0);s.store_mul(1358, 1355, 1189);s.store_offset(1180, 1188, (((-1.0)) + ((-0.0001))));s.store_div_scaled_add_product_indices(1189, 1180, (-1.0), 1186, 1188, 1.0, 1187, 1.0);s.store_mul(1359, 1355, 1189);s.store_sub(1179, 1157, 736);s.store_sqrt_square_offset(1360, 1179, 0.0001);s.store_mul(1181, 1157, 1360);s.copy_ad(1299, 733);s.copy_ad(1300, 734);s.copy_ad(1191, 735);s.store_add_scaled_product_indices(1182, 576, (-1.0), 575, 577, 1.0);s.store_mul(1183, 576, 577);s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1360), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1360), s.ad_value(1360)));}
        s.b[1675] = (s.v[1184] > 100.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1675]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1676] = (s.v[1184] < (-100.0));s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1675])) && s.b[1676]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1675])) && (!s.b[1676])) {s.store_exp(1185, 1184);}
        if (s.v[356] != 0.0) {s.store_mul3_lhs(1356, 1299, 1181, 1185);s.store_sub(1179, 1156, 736);s.store_sqrt_square_offset(1361, 1179, 0.0001);s.store_mul(1181, 1156, 1361);s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1361), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1361), s.ad_value(1361)));}
        s.b[1677] = (s.v[1184] > 100.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1677]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1678] = (s.v[1184] < (-100.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1677])) && s.b[1678]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1677])) && (!s.b[1678])) {s.store_exp(1185, 1184);}
        if (s.v[356] != 0.0) {s.store_mul3_lhs(1357, 1300, 1181, 1185);}
        if (s.v[356] == 0.0) {s.store_scalar(1357, 0.0);s.store_scalar(1356, 0.0);s.store_scalar(1359, 0.0);s.store_scalar(1358, 0.0);}
        s.b[1679] = ((s.v[355] != 0.0) && (s.v[57] != 2.0));s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if s.b[1679] {s.store_scalar(1411, s.v[706]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
    ) {
        if s.b[1679] {s.copy_ad(1410, 1416);s.store_scalar(1179, s.v[374]);s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));s.copy_ad(1410, 1414);s.store_scaled_offset(1179, 1410, (-s.v[362]), 1.0 / (s.v[363]));}
        s.b[1680] = (s.v[1179] > 100.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1680]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1681] = (s.v[1179] < (-100.0));s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1680])) && s.b[1681]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1680])) && (!s.b[1681])) {s.store_exp(1180, 1179);}
        if s.b[1679] {s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[363]);}
        s.b[1682] = (s.v[366] != 0.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1682]) {s.store_sub_from_scalar_scaled_input(1179, 1.0, 1410, 1.0 / (s.v[366]));}
        if (s.b[1679] && (!s.b[1682])) {s.store_scalar(1179, 1.0);}
        s.b[1683] = (s.v[1179] < 0.01);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1683]) {s.store_scalar(1179, 0.01);}
        if s.b[1679] {s.store_mul_ad_product_lhs_mixed_ai(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), 784, 1411);s.store_scale(1181, 785, s.v[357]);s.copy_ad(1182, 609);s.copy_ad(1183, 610);s.store_div_scaled_product_mixed_iai(1185, 1181, A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, 1179, 1.0);}
        s.b[1684] = (s.v[1185] > 100.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1684]) {s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1685] = (s.v[1185] < (-100.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1684])) && s.b[1685]) {s.store_scalar(1184, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1684])) && (!s.b[1685])) {s.store_exp(1184, 1185);}
        if s.b[1679] {s.store_mul_product3_indices(1417, 1184, 1180, 1409, 1412, 1.0);s.copy_ad(1410, 1415);s.store_scalar(1179, s.v[374]);s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));s.copy_ad(1410, 1414);s.store_scaled_sub(1179, 1162, 1409, 1.0 / (s.v[367]));}
        s.b[1686] = (s.v[1179] > 100.0);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1686]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1687] = (s.v[1179] < (-100.0));s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1686])) && s.b[1687]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1686])) && (!s.b[1687])) {s.store_exp(1180, 1179);}
        if s.b[1679] {s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[367]);}
        s.b[1688] = (s.v[370] != 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1688]) {s.store_sub_from_scalar_scaled_input(1179, 1.0, 1410, 1.0 / (s.v[370]));}
        if (s.b[1679] && (!s.b[1688])) {s.store_scalar(1179, 1.0);}
        s.b[1689] = (s.v[1179] < 0.01);s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1689]) {s.store_scalar(1179, 0.01);}
        if s.b[1679] {s.store_mul_ad_product_lhs_mixed_ai(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), 786, 1411);s.store_scale(1181, 787, s.v[357]);s.copy_ad(1182, 611);s.copy_ad(1183, 612);s.store_div_scaled_product_mixed_iai(1185, 1181, A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, 1179, 1.0);}
        s.b[1690] = (s.v[1185] > 100.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1690]) {s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1691] = (s.v[1185] < (-100.0));s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1690])) && s.b[1691]) {s.store_scalar(1184, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1690])) && (!s.b[1691])) {s.store_exp(1184, 1185);}
        if s.b[1679] {s.store_mul_product3_indices(1418, 1184, 1180, 1409, 1412, 1.0);}
        s.b[1692] = (s.v[1409] >= 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1692]) {s.copy_ad(1413, 1417);}
        if (s.b[1679] && (!s.b[1692])) {s.copy_ad(1413, 1418);}
        if s.b[1679] {s.store_add(1460, 1162, 781);}
        if (!s.b[1679]) {s.store_scalar(1413, 0.0);}
        s.store_scale(412, 1413, s.v[36]);s.b[1693] = (((((s.v[355] != 0.0) && (s.v[57] != 2.0)) && (s.v[760] != 0.0)) && (s.v[63] > 0.0)) && (s.v[1447] < s.v[1460]));s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if s.b[1693] {s.store_sub(1179, 1447, 1460);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
    ) {
        if s.b[1693] {s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_offset_scaled_sub(1446, 1180, 1179, 0.5, (((-0.01)) * (0.5)));}
        if s.b[1693] {
            if (s.v[36] == 1.0) {
                s.copy_ad(1190, 788);
            } else {
                s.copy_ad(1190, 789);
            }
        }
        if s.b[1693] {
            if (s.v[36] == 1.0) {
                s.copy_ad(1191, 790);
            } else {
                s.copy_ad(1191, 791);
            }
        }
        if s.b[1693] {s.store_mul(1181, 1447, 1446);s.store_add_scaled_product_indices(1182, 614, (-1.0), 613, 615, 1.0);s.store_mul(1183, 614, 615);s.store_mul_sub_scaled_inputs_rhs(1184, 1191, A::add_scaled_product(s.ad_value(613), 1.0, s.ad_value(1182), s.ad_value(1446), 1.0), (-s.v[357]), A::mul3(s.ad_value(1183), s.ad_value(1446), s.ad_value(1446)), (-s.v[357]));}
        s.b[1694] = (s.v[1184] > 100.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if (s.b[1693] && s.b[1694]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1695] = (s.v[1184] < (-100.0));s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((s.b[1693] && (!s.b[1694])) && s.b[1695]) {s.store_scalar(1185, 3.720075976e-44);}
        if ((s.b[1693] && (!s.b[1694])) && (!s.b[1695])) {s.store_exp(1185, 1184);}
        if s.b[1693] {s.store_scale(1190, 1190, (s.v[63] * s.v[706]));s.store_mul3_lhs(1445, 1190, 1181, 1185);}
        if (!s.b[1693]) {s.store_scalar(1445, 0.0);}
        s.store_scale(417, 1445, s.v[36]);s.b[1696] = (s.v[57] != 2.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });s.b[1697] = (s.v[71] == 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });s.b[1698] = (s.v[570] <= 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((s.b[1696] && s.b[1697]) && s.b[1698]) {s.store_scalar(1243, 0.0);}
        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {s.store_add_scaled_product_mixed_iia(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);s.store_scale(1179, 640, s.v[1227]);s.store_div_scaled_product_offset_denominator_indices(1180, 641, 1179, 1.0, 1179, 1.0, 1.0);s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);s.store_add(1182, 1179, 643);s.store_mul(1181, 1166, 1182);s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);s.store_mul3_lhs(1302, 1180, 1181, 1182);s.store_add(1256, 1301, 1302);s.store_sub(1304, 1158, 1256);s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));}
        s.b[1699] = (s.v[1179] < 1e-5);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1699]) {s.store_scalar(1179, 1e-5);}
        s.b[1700] = ((s.v[1179] < (s.v[1304] / 100.0)) && (s.v[1304] > 0.0));s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1700]) {s.store_scale(1303, 570, 2.688117142e43);}
        s.b[1701] = ((s.v[1179] < ((-s.v[1304]) / 100.0)) && (s.v[1304] < 0.0));s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if ((((s.b[1696] && s.b[1697]) && (!s.b[1698])) && (!s.b[1700])) && s.b[1701]) {s.store_scale(1303, 570, 3.720075976e-44);}
        if ((((s.b[1696] && s.b[1697]) && (!s.b[1698])) && (!s.b[1700])) && (!s.b[1701])) {s.store_mul_exp_mixed_ia(1303, 570, A::div(s.ad_value(1304), s.ad_value(1179)));}
        s.b[1702] = (s.v[1303] > 10.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1702]) {s.store_scalar(1303, 10.0);}
        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {s.store_add_product3_rhs_indices(1179, 1220, 630, 759, 1268, 1.0);s.store_mul(1243, 1303, 1179);}
        s.b[1703] = (s.v[570] <= 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if ((s.b[1696] && (!s.b[1697])) && s.b[1703]) {s.store_scalar(1439, 0.0);}
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {s.store_add_scaled_product_mixed_iia(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);s.store_scale(1179, 640, s.v[1227]);s.store_div_scaled_product_offset_denominator_indices(1180, 641, 1179, 1.0, 1179, 1.0, 1.0);s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);s.store_add(1182, 1179, 643);s.store_mul(1181, 1166, 1182);s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);s.store_mul3_lhs(1302, 1180, 1181, 1182);s.store_add(1256, 1301, 1302);s.store_sub(1304, 1158, 1256);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
    ) {
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));}
        s.b[1704] = (s.v[1179] < 1e-5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1704]) {s.store_scalar(1179, 1e-5);}
        s.b[1705] = ((s.v[1179] < (s.v[1304] / 100.0)) && (s.v[1304] > 0.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1705]) {s.store_scale(1303, 570, 2.688117142e43);}
        s.b[1706] = ((s.v[1179] < ((-s.v[1304]) / 100.0)) && (s.v[1304] < 0.0));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && (!s.b[1705])) && s.b[1706]) {s.store_scale(1303, 570, 3.720075976e-44);}
        if ((((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && (!s.b[1705])) && (!s.b[1706])) {s.store_mul_exp_mixed_ia(1303, 570, A::div(s.ad_value(1304), s.ad_value(1179)));}
        s.b[1707] = (s.v[1303] > 10.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1707]) {s.store_scalar(1303, 10.0);}
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {s.copy_ad(1179, 1220);s.store_mul(1439, 1303, 1179);}
        if (s.b[1696] && (!s.b[1697])) {s.store_add_scaled_inputs(1179, 632, 1.0 / (s.v[1227]), 631, (s.v[1227] * 1.0 / (s.v[1227])));s.store_mul_scale_offset_rhs(1438, 633, 771, s.v[301], (((((-1.0)) * (s.v[301]))) + (1.0)));}
        s.b[1708] = (s.v[759] > 0.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((s.b[1696] && (!s.b[1697])) && s.b[1708]) {s.store_sub(1180, 1438, 1422);}
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1708])) {s.store_sub(1180, 1438, 1421);}
        if (s.b[1696] && (!s.b[1697])) {s.store_offset(1181, 635, (-1.0));}
        s.b[1709] = (s.v[1180] <= 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((s.b[1696] && (!s.b[1697])) && s.b[1709]) {s.store_scalar(1182, 0.0);}
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1709])) {s.store_mul_scaled_pow_ad_rhs(1182, 634, -1.0, s.ad_value(1180), s.ad_value(1181));}
        s.b[1710] = (s.v[1182] > 100.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if ((s.b[1696] && (!s.b[1697])) && s.b[1710]) {s.store_scalar(1183, 2.688117142e43);}
        s.b[1711] = (s.v[1182] < (-100.0));s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && s.b[1711]) {s.store_scalar(1183, 3.720075976e-44);}
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1710])) && (!s.b[1711])) {s.store_exp(1183, 1182);}
        if (s.b[1696] && (!s.b[1697])) {s.store_mul_ad_product_lhs_mixed_ai(1440, A::mul3(s.ad_value(1179), s.ad_value(759), s.ad_value(1268)), 1180, 1183);s.store_add(1243, 1439, 1440);}
        s.b[1712] = ((s.v[760] == 0.0) || (s.v[760] == 2.0));s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (s.b[1696] && s.b[1712]) {s.store_scalar(1242, 0.0);}
        s.b[1713] = (s.v[526] < 0.001);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });s.b[1714] = (s.v[427] <= 0.001);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && s.b[1714]) {s.store_scalar(1179, (1.0 / 0.001));}
        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && (!s.b[1714])) {s.store_scalar(1179, (1.0 / s.v[427]));}
        if ((s.b[1696] && (!s.b[1712])) && s.b[1713]) {s.store_mul(1242, 1234, 1179);}
        if ((s.b[1696] && (!s.b[1712])) && (!s.b[1713])) {s.store_div_scaled_value_offset_denominator(1242, s.ad_value(1234), 1.0, s.ad_value(526), s.v[427], 1.0);}
        if (!s.b[1696]) {s.store_scalar(1243, 0.0);s.store_scalar(1242, 0.0);}
        s.b[1715] = (s.v[66] > 1.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if s.b[1715] {s.store_mul(1188, 596, 409);s.store_mul(1179, 1188, 1215);s.store_mul_add_rhs(413, 595, 1179, 1420);}
        s.b[1716] = (s.v[39] != 1.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if (s.b[1715] && s.b[1716]) {s.store_scale(413, 413, s.v[39]);}
        s.b[1717] = (s.v[66] == 2.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (s.b[1715] && s.b[1717]) {s.store_add(1190, 421, 413);s.store_div_scaled_product_indices(413, 421, 413, 1.0, 1190, 1.0);}
        if (!s.b[1715]) {s.store_scalar(413, 0.0);}
        s.b[1718] = (s.v[403] == 1.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if s.b[1718] {s.store_scalar(1222, 0.0);s.store_sub(1179, 1157, 736);s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_scaled_add(1360, 1179, 1180, 0.5);s.store_offset_mul(1179, 553, 1360, 1.0);s.store_mul_scale_offset_indices(1180, 1154, 554, -1.0, 0.0);s.store_add_mixed_ai(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
    ) {
        if s.b[1718] {s.store_add_mixed_ia(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));s.store_scale(1183, 1430, 0.5);s.store_add_mixed_ai(1434, A::add_scaled_product(s.ad_value(1432), 1.0, s.ad_value(1182), s.ad_value(1183), 1.0), 422);s.store_sub(1179, 1156, 736);s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_scaled_add(1361, 1179, 1180, 0.5);s.store_offset_mul(1179, 553, 1361, 1.0);s.store_mul_scale_offset_indices(1180, 1153, 554, -1.0, 0.0);s.store_add_mixed_ai(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);s.store_add_mixed_ia(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));s.store_scale(1183, 1429, 0.5);s.store_add_mixed_ai(1433, A::add_scaled_product(s.ad_value(1431), 1.0, s.ad_value(1182), s.ad_value(1183), 1.0), 423);}
        if (!s.b[1718]) {s.copy_ad(1434, 422);s.copy_ad(1433, 423);}
        s.b[1719] = (s.v[403] == 2.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if s.b[1719] {s.store_scalar(1434, 0.0);s.store_scalar(1433, 0.0);}
        s.store_mul_scale_offset_mixed_ia(1180, 1210, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0), -1.0, 1.0);s.b[1720] = (s.v[39] != 1.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if s.b[1720] {s.store_scale(1220, 1220, s.v[39]);s.store_scale(1268, 1268, s.v[39]);s.store_scale(454, 454, s.v[39]);s.store_scale(1269, 1269, s.v[39]);s.store_scale(1270, 1270, s.v[39]);s.store_scale(1358, 1358, s.v[39]);s.store_scale(1359, 1359, s.v[39]);s.store_scale(1356, 1356, s.v[39]);s.store_scale(1357, 1357, s.v[39]);s.store_scale(1243, 1243, s.v[39]);s.store_scale(412, 412, s.v[39]);s.store_scale(1240, 1240, s.v[39]);s.store_scale(1241, 1241, s.v[39]);}
        s.store_scalar(439, (A::ddx_projection(&s.ad_value(1220), Some(9), None) * s.v[36]));s.b[1721] = (s.v[759] > 0.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if s.b[1721] {s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(7), None) * s.v[36]));}
        if (!s.b[1721]) {s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(8), None) * s.v[36]));}
        s.store_scalar(441, (A::ddx_projection(&s.ad_value(1220), Some(5), None) * s.v[36]));s.store_scale(1178, 757, ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[692]) + s.v[62]));s.store_scale(1316, 757, (s.v[342] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[726]) + s.v[62])));s.store_scale(1448, 757, s.v[63]);s.store_scale(1449, 757, (s.v[342] * s.v[63]));s.store_sub(1166, 1161, 1407);s.store_mul(1189, 1393, 1168);s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);s.store_mul3_lhs(1351, 1393, 724, 1168);s.store_mul3_lhs(1352, 1393, 725, 1168);s.b[1722] = (s.v[69] == 0.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });s.b[1723] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if (s.b[1722] && s.b[1723]) {let t0: A = A::exp(s.ad_value(1145));s.store_square_ad(1146, t0);s.store_mul_mixed_ia(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
    ) {
        if (s.b[1722] && s.b[1723]) {
            s.store_mul_mixed_ia(1210, 1351, {
                            if ((1.0 + s.v[1146]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1146), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1724] = (s.v[63] > 0.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {s.store_mul_exp_mixed_ia(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));}
        if ((s.b[1722] && s.b[1723]) && s.b[1724]) {
            s.store_mul_mixed_ia(1451, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1725] = (s.v[69] == 1.0);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });s.b[1726] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {s.store_exp_ad(1146, A::div(s.ad_value(1145), A::mul(s.ad_value(745), s.ad_value(724))));s.store_mul_mixed_ia(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));}
        if (((!s.b[1722]) && s.b[1725]) && s.b[1726]) {
            s.store_mul_mixed_ia(1210, 1351, {
                            if ((1.0 + s.v[1146]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1146), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1727] = (s.v[63] > 0.0);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {s.store_mul_exp_mixed_ia(1450, 1146, A::div_scaled_value_by_product(s.ad_value(781), -1.0, s.ad_value(1352), A::square(s.ad_value(1168)), 1.0));}
        if ((((!s.b[1722]) && s.b[1725]) && s.b[1726]) && s.b[1727]) {
            s.store_mul_mixed_ia(1451, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1722]) && (!s.b[1725])) {s.store_div_scaled_product_mixed_iai(1145, 749, A::sub(s.ad_value(1166), s.ad_value(685)), 1.0, 1351, 1.0);s.store_div_scaled_inputs2_mixed_iai(1169, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::sub(s.ad_value(1166), s.ad_value(685))), (-1.0), 1351, 1.0);}
        s.b[1728] = (s.v[1145] > 100.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1728]) {s.store_sub(1210, 1166, 685);}
        s.b[1729] = (s.v[1169] > 100.0);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && s.b[1729]) {s.store_div_scaled_inputs3_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 1351, 1.0);s.store_exp(1146, 1179);s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);}
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {s.store_exp(1146, 1145);}
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_mul_mixed_ia(1180, 1351, {
                            if ((1.0 + s.v[1146]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1146), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(749)));s.store_sub_mixed_ia(1181, 749, A::div_scaled_product(s.ad_value(1351), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
    ) {
        if ((((!s.b[1722]) && (!s.b[1725])) && (!s.b[1728])) && (!s.b[1729])) {s.store_div(1210, 1180, 1181);}
        s.b[1730] = (s.v[63] > 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if (((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) {s.store_div_scaled_product_mixed_iai(1452, 749, A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0), 1.0, 1352, 1.0);s.store_div_scaled_inputs2_mixed_iai(1453, 751, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(749), A::add_scaled_inputs3(s.ad_value(1166), 1.0, s.ad_value(685), (-1.0), s.ad_value(781), -1.0)), (-1.0), 1352, 1.0);}
        s.b[1731] = (s.v[1452] > 100.0);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if ((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && s.b[1731]) {s.store_add_scaled_inputs3_indices(1451, 1166, 1.0, 685, (-1.0), 781, -1.0);}
        s.b[1732] = (s.v[1453] > 100.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && s.b[1732]) {s.store_div_scaled_inputs4_indices(1179, 1166, 1.0, 685, (-1.0), 751, -1.0, 781, -1.0, 1352, 1.0);s.store_exp(1450, 1179);s.store_mul_div_scaled_product_indices(1451, 1450, 1168, 1473, 1.0, 757, 1.0);}
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {s.store_exp(1450, 1452);}
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_mul_mixed_ia(1180, 1352, {
                            if ((1.0 + s.v[1450]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(1450), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((((!s.b[1722]) && (!s.b[1725])) && s.b[1730]) && (!s.b[1731])) && (!s.b[1732])) {s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1453)), A::sub_from_scalar(1.0, s.ad_value(749)));s.store_sub_mixed_ia(1181, 749, A::div_scaled_product(s.ad_value(1352), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(749)), 1.0));s.store_div(1451, 1180, 1181);}
        s.copy_ad(1165, 1407);s.copy_ad(1164, 1388);s.copy_ad(1177, 1378);s.b[1733] = (s.v[88] == 2.0);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });s.b[1734] = (s.v[57] == 2.0);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1734]) {s.store_scalar(1273, 0.0);s.store_scalar(1272, 0.0);}
        if (s.b[1733] && (!s.b[1734])) {s.store_add_mixed_ai(1162, A::add_scaled_inputs_product(s.ad_value(1165), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1164), (-1.0)), 685);s.store_add_scaled_inputs3_offset_indices(1149, 1162, 1.0, 1161, (-1.0), 1177, 1.0, (-0.08));}
        s.b[1735] = (s.v[1162] <= 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1735]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (-(4.0 * 0.08)));}
        if ((s.b[1733] && (!s.b[1734])) && (!s.b[1735])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1162, (4.0 * 0.08));}
        if (s.b[1733] && (!s.b[1734])) {s.store_add_scaled_inputs3_indices(1148, 1162, 1.0, 1149, (-0.5), 1179, (-0.5));s.store_mul_sub_rhs(1273, 1316, 1148, 1162);}
        s.b[1736] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {s.store_add(1460, 1162, 781);s.store_scalar(1472, 0.08);s.store_add_scaled_inputs4_indices(1149, 1460, 1.0, 1458, (-1.0), 1177, 1.0, 1472, -1.0);}
        s.b[1737] = (s.v[1460] <= 0.0);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && s.b[1737]) {s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, (-100.0));}
        if (((s.b[1733] && (!s.b[1734])) && s.b[1736]) && (!s.b[1737])) {s.store_sqrt_add_scaled_square_product(1179, 1149, 1.0, 1472, 1460, 100.0);}
        if ((s.b[1733] && (!s.b[1734])) && s.b[1736]) {s.store_add_scaled_inputs3_indices(1461, 1460, 1.0, 1149, (-0.5), 1179, (-0.5));s.store_add_scaled_product_right_sub(1273, 1273, 1.0, 1449, 1461, 1460, 1.0);}
        if (s.b[1733] && (!s.b[1734])) {s.store_scale(1179, 737, 0.5);s.store_add_scaled_inputs4_indices(1182, 1161, 1.0, 1148, (-1.0), 1177, -1.0, 1210, -1.0);}
        s.b[1738] = (s.v[737] == 0.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1738]) {s.store_scalar(1180, 0.0);}
        s.b[1739] = (s.v[1182] < 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && s.b[1739]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if (((s.b[1733] && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1739])) {s.store_sqrt_square_add(1180, 1179, 1182);}
        if (s.b[1733] && (!s.b[1734])) {s.store_mul_ad_product_rhs_mixed_ia(1272, 1316, 737, A::sub(s.ad_value(1180), s.ad_value(1179)));}
        s.b[1740] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {s.store_add_scaled_inputs4_indices(1182, 1458, 1.0, 1461, (-1.0), 1177, -1.0, 1451, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
    ) {
        s.b[1741] = (s.v[1182] < 0.0);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && s.b[1741]) {s.store_add_div_rhs_indices(1180, 1179, 1182, 737);}
        if (((s.b[1733] && (!s.b[1734])) && s.b[1740]) && (!s.b[1741])) {s.store_sqrt_square_add(1180, 1179, 1182);}
        if ((s.b[1733] && (!s.b[1734])) && s.b[1740]) {s.store_add_product3_rhs_mixed_iia(1272, 1272, 1449, 737, A::sub(s.ad_value(1180), s.ad_value(1179)), 1.0);}
        if s.b[1733] {s.store_scale(1229, 1196, s.v[694]);s.store_div(1226, 1210, 1229);s.store_offset_sub(1150, 1226, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1226, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1212, 1226, 1.0, 1150, (-0.5), 1179, (-0.5));}
        s.b[1742] = (s.v[63] > 0.0);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1742]) {s.store_div(1462, 1451, 1229);s.store_offset_sub(1150, 1462, 1158, (-0.02));s.store_sqrt_add_scaled_square_input(1179, 1150, 1.0, 1462, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1463, 1462, 1.0, 1150, (-0.5), 1179, (-0.5));}
        s.b[1743] = (s.v[57] == 2.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1743]) {s.store_scalar(1341, 0.0);}
        if (s.b[1733] && (!s.b[1743])) {s.store_mul(1179, 1229, 1212);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1212, 1180);s.store_mul(1182, 1179, 1181);s.store_sub_from_scalar(1186, 1.0, 1229);s.store_mul_ad_product_rhs_mixed_ia(1341, 1316, 1186, A::sub_scaled_inputs(s.ad_value(1212), 0.5, s.ad_value(1182), 1.0));}
        s.b[1744] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1743])) && s.b[1744]) {s.store_mul(1179, 1229, 1463);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1463, 1180);s.store_mul(1182, 1179, 1181);s.store_sub_from_scalar(1186, 1.0, 1229);s.store_add_product3_rhs_mixed_iia(1341, 1341, 1449, 1186, A::sub_scaled_inputs(s.ad_value(1463), 0.5, s.ad_value(1182), 1.0), 1.0);}
        if s.b[1733] {s.store_mul(1179, 1229, 1212);s.store_scaled_offset_ad(1180, A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), 0.5), 1e-20, 12.0);s.store_div(1181, 1179, 1180);s.store_mul(1182, 1179, 1181);s.store_mul_add_scaled_inputs3_offset_rhs_indices(1250, 1178, 1210, 1.0, 1179, (-0.5), 1182, 1.0, 0.0);}
        s.b[1745] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1745]) {s.store_mul(1454, 1229, 1463);s.store_scaled_offset_ad(1191, A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), 0.5), 1e-20, 12.0);s.store_div(1181, 1454, 1191);s.store_mul(1182, 1454, 1181);s.store_add_scaled_product_mixed_iia(1250, 1250, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 1.0, s.ad_value(1454), (-0.5), s.ad_value(1182), 1.0), 1.0);}
        s.b[1746] = (s.v[153] > 0.5);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1746]) {s.store_scale(1180, 1180, 2.0);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(1254, 1178, 1210, ((0.5) * (-1.0)), 1179, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1179), s.ad_value(1179), 1.0, s.ad_value(1180), 1.0), ((-1.0) * (-1.0)), 0.0);}
        s.b[1747] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((s.b[1733] && s.b[1746]) && s.b[1747]) {s.store_scale(1191, 1191, 2.0);s.store_add_scaled_product_mixed_iia(1254, 1254, 1.0, 1448, A::add_scaled_inputs3(s.ad_value(1451), 0.5, s.ad_value(1454), 0.25, A::div_scaled_product(s.ad_value(1454), s.ad_value(1454), 1.0, s.ad_value(1191), 1.0), -1.0), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1748] = (s.v[153] < 0.5);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if ((s.b[1733] && (!s.b[1746])) && s.b[1748]) {s.store_scale(1180, 1180, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1181, 1178, 0.5, 1180, 1.0);s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1179), s.ad_value(1179), s.ad_value(1179), (2.0 * 0.06666666666666667)), (-1.0), 1210, A::add_scaled_products(s.ad_value(1179), s.ad_value(1179), (2.0 * 0.3333333333333333), s.ad_value(1210), A::sub_scaled_inputs(s.ad_value(1210), 1.0, s.ad_value(1179), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1254, 1182, 1181, -1.0, 0.0);}
        s.b[1749] = (((s.v[57] != 2.0) && (s.v[760] != 0.0)) && (s.v[63] > 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (((s.b[1733] && (!s.b[1746])) && s.b[1748]) && s.b[1749]) {s.store_scale(1191, 1191, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(1181, 1448, 0.5, 1191, 1.0);s.store_add_scaled_product_mixed_aia(1182, A::mul3_scaled_output(s.ad_value(1454), s.ad_value(1454), s.ad_value(1454), (2.0 * 0.06666666666666667)), (-1.0), 1451, A::add_scaled_products(s.ad_value(1454), s.ad_value(1454), (2.0 * 0.3333333333333333), s.ad_value(1451), A::sub_scaled_inputs(s.ad_value(1451), 1.0, s.ad_value(1454), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1470, 1182, 1181, -1.0, 0.0);s.store_add(1254, 1254, 1470);}
        if ((s.b[1733] && (!s.b[1746])) && (!s.b[1748])) {s.store_scaled_add(1254, 1250, 1341, (-0.5));}
        s.b[1750] = (s.v[57] == 2.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (s.b[1733] && s.b[1750]) {s.store_scalar(1274, 0.0);}
        if (s.b[1733] && (!s.b[1750])) {s.store_scale(1249, 626, (s.v[342] * (s.v[1248] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[727]) + s.v[65]))));s.store_mul_sub_rhs(1274, 1249, 1237, 1160);}
        if s.b[1733] {s.store_add_scaled_inputs3_indices(1251, 1250, 1.0, 1273, 1.0, 1272, 1.0);s.store_add_scaled_inputs4_indices(1252, 1341, 1.0, 1273, (-1.0), 1272, -1.0, 1274, -1.0);s.copy_ad(1255, 1274);s.store_add_scaled_inputs4_indices(1253, 1251, (-1.0), 1254, (-1.0), 1252, (-1.0), 1255, (-1.0));}
        s.b[1751] = (s.v[88] == 3.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });s.b[1752] = (s.v[68] == 0.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1752]) {s.store_div_from_scalar(1332, 3.453133e-11, 92);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1752])) {s.store_div_scaled_inputs_indices(1332, 777, 8.85418e-12, 92, 1.0);}
        if ((!s.b[1733]) && s.b[1751]) {s.store_div_scaled_product_indices(1178, 1178, 776, 1.0, 92, 1.0);s.store_div_scaled_inputs_indices(1316, 1316, s.v[91], 92, 1.0);s.store_scale(1333, 92, 100000000.0);}
        s.b[1753] = (s.v[63] > 0.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1753]) {s.store_div_scaled_inputs_indices(1448, 1448, s.v[91], 92, 1.0);s.store_div_scaled_inputs_indices(1449, 1449, s.v[91], 92, 1.0);}
        s.b[1754] = (s.v[57] == 2.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (((!s.b[1733]) && s.b[1751]) && s.b[1754]) {s.store_scalar(1273, 0.0);s.store_scalar(1272, 0.0);s.store_scalar(1350, 0.0);}
        s.b[1755] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1755]) {s.store_add_mixed_ai(1350, A::add_scaled_inputs_product(s.ad_value(1349), 1.0, s.ad_value(1277), (-1.0), s.ad_value(707), s.ad_value(1278), (-1.0)), 685);}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1755])) {s.store_add(1350, 424, 685);}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_add_scaled_inputs3_offset_indices(1149, 1350, 1.0, 1161, (-1.0), 1177, 1.0, (-0.02));}
        s.b[1756] = (s.v[1350] <= 0.0);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1756]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (-(4.0 * 0.02)));}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1756])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1350, (4.0 * 0.02));}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_add_scaled_inputs3_indices(1148, 1350, 1.0, 1149, (-0.5), 1179, (-0.5));}
        s.b[1757] = (s.v[63] > 0.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {s.store_add(1459, 1350, 781);s.store_add_scaled_inputs3_offset_indices(1149, 1459, 1.0, 1458, (-1.0), 1177, 1.0, (-0.02));}
        s.b[1758] = (s.v[1459] <= 0.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && s.b[1758]) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (-(100.0 * 0.02)));}
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) && (!s.b[1758])) {s.store_sqrt_add_scaled_square_input(1179, 1149, 1.0, 1459, (100.0 * 0.02));}
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1757]) {s.store_add_scaled_inputs3_indices(1461, 1459, 1.0, 1149, (-0.5), 1179, (-0.5));}
        if (((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) {s.store_div_scaled_inputs3_indices(1179, 1161, 1.0, 1177, (-1.0), 1350, -1.0, 1333, 1.0);s.store_mul(1194, 1179, 722);}
        s.b[1759] = (((-100.0) < s.v[1194]) && (s.v[1194] < 100.0));s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && s.b[1759]) {s.store_mul_exp_rhs(1334, 721, 1194);}
        s.b[1760] = (s.v[1194] <= (-100.0));s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && s.b[1760]) {s.store_scale(1334, 721, 3.720075976e-44);}
        if (((((!s.b[1733]) && s.b[1751]) && (!s.b[1754])) && (!s.b[1759])) && (!s.b[1760])) {s.store_scale(1334, 721, 2.688117142e43);}
    }
}
