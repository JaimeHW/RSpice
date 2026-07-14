#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        s.b[1561] = (s.v[404] == 0.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1561]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1385, 1384, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1381, 1181, 1.0, 1179, 1385, 1.0);}
        if ((!s.b[1540]) && (!s.b[1561])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1384), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1385, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1381, 1385, 1186);}
        s.b[1562] = (s.v[57] == 2.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1562]) {s.store_offset(1380, 1381, 0.02);s.store_offset(1160, 1381, 0.02);}
        if ((!s.b[1540]) && (!s.b[1562])) {s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1381), 0.02), (-0.01));s.store_sqrt_square_offset(1181, 1180, 0.0001);s.store_add_scaled_inputs3_offset_indices(1380, 1381, 1.0, 1180, 0.5, 1181, 0.5, 0.02);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1385, 1380, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1382, 1380, 1.0, 1182, 1183, (-0.5));}
        s.store_offset(1179, 1367, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(1181, 1179, 0.5, 1180, 0.5, (-5.0));s.store_scalar(1179, 1.5);s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));s.store_offset_add_scaled_inputs_indices(1297, 1180, (-0.5), 1182, (-0.5), s.v[1179]);s.store_scale(1179, 1277, 0.95);s.store_offset_sub(1180, 1179, 1297, (-0.002));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.008);s.store_add_scaled_inputs3_indices(1177, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));s.store_offset(1179, 1382, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(1181, 1179, 0.5, 1180, 0.5, (-5.0));s.store_scalar(1179, 1.5);s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));s.store_offset_add_scaled_inputs_indices(1379, 1180, (-0.5), 1182, (-0.5), s.v[1179]);s.store_scale(1179, 1277, 0.95);s.store_offset_sub(1180, 1179, 1379, (-0.002));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.008);s.store_add_scaled_inputs3_indices(1378, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));s.store_sub(1163, 1277, 1177);s.store_sqrt(1164, 1163);s.store_div_scaled_product_indices(1199, 1279, 1164, 1.0, 1278, 1.0);s.store_sqrt(1182, 1199);s.store_mul(1179, 501, 1177);s.b[1563] = (s.v[1179] >= (-0.5));s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if s.b[1563] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1563]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1200, 758, 1182, 1180);s.store_mul(1179, 504, 1177);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        s.b[1564] = (s.v[1179] >= (-0.5));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if s.b[1564] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1564]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1201, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1200, 1.0);s.b[1565] = (s.v[1179] > (-100.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if s.b[1565] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1565]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1199, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1177, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, 469, 1.0, 757, 1.0);s.b[1566] = (s.v[1183] >= (-0.5));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if s.b[1566] {s.store_offset(1167, 1183, 1.0);}
        if (!s.b[1566]) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1167, 1179, 1183, 3.0, 1.0);}
        s.b[1567] = (s.v[739] > 0.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if s.b[1567] {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
        s.b[1568] = (s.v[1179] < (-100.0));s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (s.b[1567] && s.b[1568]) {s.store_scalar(1181, 3.720075976e-44);}
        if (s.b[1567] && (!s.b[1568])) {s.store_exp(1181, 1179);}
        if s.b[1567] {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if s.b[1567] {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1567] {s.store_mul(1424, 1167, 1183);}
        if (!s.b[1567]) {s.store_scalar(1424, 0.0);}
        s.store_mul(411, 499, 1203);s.store_mul(1202, 411, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1201, 1.0);s.b[1569] = (s.v[1179] > (-100.0));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if s.b[1569] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1569]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        s.store_mul(1179, 502, 1181);s.store_mul(1239, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1177, 1.0);s.store_add_scaled_product_mixed_aii(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_div_scaled_product_offset_denominator_indices(1205, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 761, 1.0, 557, 1177, 1.0);s.b[1570] = (s.v[1182] < 0.0001);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if s.b[1570] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        s.store_mul3_lhs(1208, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_div_from_scalar(1188, 2.2361, 1278);s.store_add_scaled_product_right_sub(1298, 1164, 1.0, 1188, 1297, 1177, (-1.0));s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1165, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1298), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1177), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1177), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1208, -1.0, 1424, -1.0, 1425);s.store_sub(1387, 1277, 1378);s.store_sqrt(1388, 1387);s.store_div_scaled_product_indices(1389, 1279, 1388, 1.0, 1278, 1.0);s.store_sqrt(1182, 1389);s.store_mul(1179, 501, 1378);s.b[1571] = (s.v[1179] >= (-0.5));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if s.b[1571] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1571]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1390, 758, 1182, 1180);s.store_mul(1179, 504, 1378);s.b[1572] = (s.v[1179] >= (-0.5));s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if s.b[1572] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1572]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1391, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1390, 1.0);s.b[1573] = (s.v[1179] > (-100.0));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if s.b[1573] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1573]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);}
        s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1389, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1378, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1392), 1.0), 1.0, 469, 1.0, 757, 1.0);s.b[1574] = (s.v[1183] >= (-0.5));s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if s.b[1574] {s.store_offset(1393, 1183, 1.0);}
        if (!s.b[1574]) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1393, 1179, 1183, 3.0, 1.0);}
        s.b[1575] = (s.v[739] > 0.0);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if s.b[1575] {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
        s.b[1576] = (s.v[1179] < (-100.0));s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (s.b[1575] && s.b[1576]) {s.store_scalar(1181, 3.720075976e-44);}
        if (s.b[1575] && (!s.b[1576])) {s.store_exp(1181, 1179);}
        if s.b[1575] {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if s.b[1575] {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1575] {s.store_mul(1405, 1393, 1183);}
        if (!s.b[1575]) {s.store_scalar(1405, 0.0);}
        s.store_mul(411, 499, 1392);s.store_mul(1401, 411, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1391, 1.0);s.b[1577] = (s.v[1179] > (-100.0));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if s.b[1577] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1577]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        s.store_mul(1179, 502, 1181);s.store_mul(1402, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1378, 1.0);s.store_add_scaled_product_mixed_aii(1403, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_div_scaled_product_offset_denominator_indices(1400, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 762, 1.0, 559, 1378, 1.0);s.b[1578] = (s.v[1182] < 0.0001);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if s.b[1578] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        s.store_mul3_lhs(1404, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_div_from_scalar(1188, 2.2361, 1278);s.store_add_scaled_product_right_sub(1406, 1388, 1.0, 1188, 1379, 1378, (-1.0));s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1407, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1406), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1378), (-1.0)), 1.0, s.ad_value(1401), (-1.0), s.ad_value(1402), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1378), 1.0), s.ad_value(1400), 1.0), 1.0, 1403, 1.0, 1404, -1.0, 1405, -1.0, 1425);s.b[1579] = (((s.v[88] == 3.0) && (p.p33 == 1.0)) && (p.p16 != 0.0));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if s.b[1579] {s.store_sqrt(1342, 1279);s.store_mul(1343, 758, 1342);s.store_mul(1344, 758, 1342);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1343, 1.0);}
        s.b[1580] = (s.v[1179] > (-100.0));s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (s.b[1579] && s.b[1580]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);}
        if (s.b[1579] && (!s.b[1580])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);}
        if s.b[1579] {s.store_mul3_lhs(1346, 499, 1345, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1344, 1.0);}
        s.b[1581] = (s.v[1179] > (-100.0));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1579] && s.b[1581]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (s.b[1579] && (!s.b[1581])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if s.b[1579] {s.store_mul(1179, 502, 1181);s.store_mul(1347, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs(1180, 491, 1.0, 492, 1.0 / (s.v[1227]));s.store_add_scaled_product_mixed_aii(1348, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_add_mixed_ai(1349, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(768), s.v[36], s.ad_value(1346), (-1.0), s.ad_value(1347), -1.0), 1.0, s.ad_value(495), s.ad_value(1400), 1.0), 1348);}
        if (!s.b[1579]) {s.store_scalar(1349, 0.0);}
        s.store_sub(1166, 1161, 1165);s.store_mul(1189, 1167, 1168);s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);s.store_div_scaled_inputs2_mixed_iai(1169, 521, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(1166)), (-1.0), 1189, 1.0);s.b[1582] = (s.v[1145] > 100.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.copy_ad(1210, 1166);s.store_scalar(1146, 0.0);}
        s.b[1583] = (s.v[1169] > 100.0);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if ((!s.b[1582]) && s.b[1583]) {s.store_div_scaled_inputs2_by_product_indices(1179, 1166, 1.0, 521, (-1.0), 1167, 1168, 1.0);s.store_exp(1146, 1179);s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);}
        if ((!s.b[1582]) && (!s.b[1583])) {s.store_exp(1146, 1145);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1582]) && (!s.b[1583])) {s.store_mul_ln_mixed_ia(1180, 1189, A::offset(s.ad_value(1146), 1.0));s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(745)));s.store_sub_mixed_ia(1181, 745, A::div_scaled_product(s.ad_value(1189), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));s.store_div(1210, 1180, 1181);}
        s.store_add_scaled_inputs(1225, 1210, 1.0, 1168, 2.0);s.copy_ad(451, 1210);s.b[1584] = (s.v[746] <= 0.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if s.b[1584] {s.store_scalar(1426, 1.0);}
        if (!s.b[1584]) {s.store_div_scaled_inputs_indices(1188, 746, ((s.v[1227]) as f64).sqrt(), 1225, 1.0);s.store_div_from_scalar_offset_input(1426, 1.0, 1188, 1.0);}
        s.store_sub(1188, 1164, 1278);s.store_sub_from_scalar_ad(1228, s.v[689], A::add_scaled_products(s.ad_value(566), s.ad_value(1210), (2.0 - s.v[58]), s.ad_value(567), s.ad_value(1188), (2.0 - s.v[58])));s.b[1585] = (s.v[1228] < 2e-8);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if s.b[1585] {s.store_div_from_scalar_sub_from_scalar_ad(1179, 1.0, 6e-8, A::scale(s.ad_value(1228), 2.0));s.store_mul_scale_offset_indices(1228, 1179, 1228, -(2e-8), (4e-8) * (2e-8));}
        s.b[1586] = (s.v[403] == 1.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if s.b[1586] {s.store_scalar(1222, 0.0);}
        if (!s.b[1586]) {s.store_add_scaled_products_indices(1179, 553, 1210, 1.0, 554, 1188, 1.0);}
        s.b[1587] = (s.v[1179] >= (-0.9));s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if ((!s.b[1586]) && s.b[1587]) {s.store_mul_scale_offset_indices(1222, 1290, 1179, 1.0, 1.0);}
        if ((!s.b[1586]) && (!s.b[1587])) {s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1179, 20.0, 17.0);s.store_mul_ad_product_lhs_mixed_ia(1222, 1290, A::offset(s.ad_value(1179), 0.8), 1180);}
        s.b[1588] = (s.v[403] == 2.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if s.b[1588] {s.store_add_scaled_inputs3_indices(1222, 423, 1.0, 1222, 1.0, 422, 1.0);}
        s.b[1589] = (s.v[473] == 0.0);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if s.b[1589] {s.store_scalar(1195, 1.0);s.store_scalar(1196, 1.0);}
        if (!s.b[1589]) {s.store_mul(1189, 477, 1297);}
        s.b[1590] = (s.v[1189] >= (-0.5));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if ((!s.b[1589]) && s.b[1590]) {s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);}
        if ((!s.b[1589]) && (!s.b[1590])) {s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);}
        if (!s.b[1589]) {s.store_add(1189, 1277, 629);s.store_div_scaled_product_indices(1299, 1297, 1190, 1.0, 1189, 1.0);}
        s.b[1591] = (s.v[1299] < 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
        if ((!s.b[1589]) && s.b[1591]) {s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));}
        if ((!s.b[1589]) && (!s.b[1591])) {s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);}
        if (!s.b[1589]) {s.store_div_scaled_product_mixed_iia(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);s.store_mul(1180, 1189, 1300);s.store_sqrt_mul(1188, 608, 1199);s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);s.store_div_from_scalar(1184, s.v[1227], 1204);s.store_mul(1205, 473, 1184);s.store_offset(1206, 569, s.v[689]);s.store_div(1207, 568, 1206);s.store_add(1181, 1205, 1207);s.store_square(1185, 1184);s.store_mul(1186, 1184, 1185);s.store_offset_mul(1196, 1180, 1181, 1.0);s.store_mul3_lhs(1187, 474, 473, 1186);s.store_mul_scale_offset_indices(1214, 1187, 1180, -1.0, 0.0);s.store_add_scaled_product_indices(1195, 1196, 1.0, 1214, 1210, 1.0);}
        s.b[1592] = (s.v[1196] < 0.01);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1592] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1196), 200.0));s.store_mul_scale_offset_indices(1196, 1188, 1196, -1.0, 0.02);}
        s.b[1593] = (s.v[1195] < 0.01);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
        if s.b[1593] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1195), 200.0));s.store_mul_scale_offset_indices(1195, 1188, 1195, -1.0, 0.02);}
        s.b[1594] = (s.v[473] == 0.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if s.b[1594] {s.store_scalar(1408, 1.0);}
        if (!s.b[1594]) {s.store_mul(1189, 477, 1379);}
        s.b[1595] = (s.v[1189] >= (-0.5));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((!s.b[1594]) && s.b[1595]) {s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);}
        if ((!s.b[1594]) && (!s.b[1595])) {s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);}
        if (!s.b[1594]) {s.store_add(1189, 1277, 629);s.store_div_scaled_product_indices(1299, 1379, 1190, 1.0, 1189, 1.0);}
        s.b[1596] = (s.v[1299] < 0.5);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if ((!s.b[1594]) && s.b[1596]) {s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));}
        if ((!s.b[1594]) && (!s.b[1596])) {s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);}
        if (!s.b[1594]) {s.store_div_scaled_product_mixed_iia(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);s.store_mul(1180, 1189, 1300);s.store_sqrt_mul(1188, 608, 1389);s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);s.store_div_from_scalar(1184, s.v[1227], 1204);s.store_mul(1205, 473, 1184);s.store_offset(1206, 569, s.v[689]);s.store_div(1207, 568, 1206);s.store_add(1181, 1205, 1207);s.store_square(1185, 1184);s.store_mul(1186, 1184, 1185);s.store_offset_mul(1408, 1180, 1181, 1.0);}
        s.b[1597] = (s.v[1408] < 0.01);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
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
        if s.b[1604] {s.store_div_from_scalar_ad(1179, 1.0, A::add_scaled_product(s.ad_value(1225), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0));s.store_mul(1182, 1174, 1225);s.store_mul(1173, 1182, 1179);}
        if (!s.b[1604]) {s.store_mul(1188, 1195, 1224);s.store_mul(1186, 1225, 1188);s.store_mul(1185, 1225, 1224);s.store_mul_add_scaled_inputs_rhs(1179, 1195, A::offset(s.ad_value(1188), (-1.0)), 2.0, A::div_from_scalar(1.0, s.ad_value(1209)), 2.0);s.store_add_scaled_inputs_mixed_ai(1180, A::add_scaled_products(s.ad_value(1225), A::offset(A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0)), 1.0, s.ad_value(1195), s.ad_value(1174), 1.0), 1.0, 1186, 3.0);s.store_mul_add_scaled_inputs_rhs_indices(1181, 1225, 1174, 1.0, 1185, 2.0);s.store_sqrt_add_scaled_square_product(1182, 1180, 1.0, 1179, 1181, (-2.0));s.store_div_scaled_inputs2_indices(1173, 1180, 1.0, 1182, (-1.0), 1179, 1.0);}
        s.store_add_scaled_inputs3_indices(1180, 1173, 1.0, 1158, (-1.0), 550, -1.0);s.store_sqrt_add_scaled_square_product(1181, 1180, 1.0, 550, 1173, 4.0);s.store_add_scaled_inputs3_indices(1211, 1173, 1.0, 1180, (-0.5), 1181, (-0.5));s.b[1605] = (s.v[1211] > s.v[1158]);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if s.b[1605] {s.copy_ad(1211, 1158);}
        s.store_sub(1213, 1158, 1211);s.store_sub_from_scalar_ad(1207, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1173), 0.5, s.ad_value(1225), 1.0));s.store_mul(1188, 1224, 1210);s.store_add_scaled_inputs_product_indices(1179, 1174, 1.0, 1173, 1.0, 1188, 1207, 2.0);s.store_mul(1188, 1224, 1195);s.store_add_offset_lhs_mixed_ai(1180, A::div_from_scalar(2.0, s.ad_value(1209)), (-1.0), 1188);s.store_div(1176, 1179, 1180);s.b[1606] = ((s.v[560] > 0.0) && (s.v[1213] > 1e-10));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
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
        if (!s.b[1611]) {s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1188, 20.0, 17.0);s.store_mul_scale_offset_indices(1179, 1180, 1188, 1.0, 0.8);}
        s.store_add(1206, 1197, 1198);s.store_div_scaled_product_indices(1180, 1197, 1198, 1.0, 1206, 1.0);s.store_add(1206, 1180, 1427);s.store_div_scaled_product_indices(1181, 1180, 1427, 1.0, 1206, 1.0);s.store_add_scaled_product_indices(1175, 1176, 1.0, 1179, 1181, 1.0);s.store_scaled_mul(1221, 757, 1228, 1.0 / (s.v[1227]));s.store_mul(1215, 1171, 1221);s.store_sub_from_scalar_ad(1179, 1.0, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0));s.store_mul(1217, 1210, 1179);s.store_div(1188, 1211, 1174);s.store_offset(1218, 1188, 1.0);s.store_div_scaled_product_indices(1216, 1215, 1217, 1.0, 1218, 1.0);s.store_offset_mul(1179, 1216, 1222, 1.0);s.store_div(1188, 1211, 1179);s.store_mul(1219, 1216, 1188);s.store_div(1419, 1216, 1179);s.store_div(1188, 1213, 1175);s.store_offset(1179, 1188, 1.0);s.store_scaled_mul(1220, 1219, 1179, 1.0 / (s.v[59]));s.store_scaled_mul(454, 1419, 1179, 1.0 / (s.v[59]));s.b[1612] = (s.v[454] < 1e-9);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if s.b[1612] {s.store_scalar(454, 1e-9);}
        s.store_scaled_mul(1420, 1419, 1179, 1.0 / (s.v[59]));s.b[1613] = (s.v[57] != 2.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });s.b[1614] = (s.v[68] == 0.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1614]) {s.store_mul_div_from_scalar_lhs_ad_indices(1179, (3.0 * 3.9), 777, 776);}
        if (s.b[1613] && (!s.b[1614])) {s.store_div_scaled_inputs_indices(1179, 776, s.v[74], 777, 1.0);}
        s.b[1615] = (s.v[70] == 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });s.b[1616] = (s.v[68] == 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1616]) {s.store_div_scaled_inputs3_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 1179, 1.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1616])) {s.store_div_scaled_inputs4_indices(1180, 1158, -1.0, 1444, (-1.0), 1486, -1.0, 736, 1.0, 1179, 1.0);}
        s.b[1617] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1613] && s.b[1615]) && (!s.b[1617])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_square(1183, 1160);s.store_mul_scale_offset_indices(1184, 1183, 1160, -1.0, 0.0);s.store_offset_add_ad(1185, s.ad_value(1485), A::abs(s.ad_value(1184)), 1e-9);s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        s.b[1618] = (s.v[68] == 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && s.b[1618]) {s.store_div_scaled_inputs3_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 1179, 1.0);}
        if ((s.b[1613] && s.b[1615]) && (!s.b[1618])) {s.store_div_scaled_inputs4_indices(1180, 1158, 1.0, 1161, (-1.0), 1479, -1.0, 736, 1.0, 1179, 1.0);}
        s.b[1619] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if ((s.b[1613] && s.b[1615]) && (!s.b[1619])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_square(1183, 1235);s.store_mul_scale_offset_indices(1184, 1183, 1235, -1.0, 0.0);s.store_offset_add_ad(1185, s.ad_value(1478), A::abs(s.ad_value(1184)), 1e-9);s.store_offset_add_scaled_inputs(1186, A::div(s.ad_value(1184), s.ad_value(1185)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(1184), s.ad_value(1185)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));}
        s.b[1620] = (s.v[68] == 0.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1620]) {s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 1179, 1.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1620])) {s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), -1.0, s.ad_value(1487), s.ad_value(1444), (-1.0)), 1.0, 1486, (-1.0), 736, 1.0, 1179, 1.0);}
        s.b[1621] = (((s.v[1483] <= 0.0) || (s.v[1484] <= 0.0)) || (s.v[1485] < 0.0));s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1484), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_sub(1183, 1160, 1489);}
        s.b[1622] = (s.v[1183] >= ((-1.0) / 100.0));s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && s.b[1622]) {s.store_scale(1184, 1488, (-100.0));}
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) && (!s.b[1622])) {s.store_div(1184, 1488, 1183);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1621])) {s.store_exp(1185, 1184);}
        s.b[1623] = (s.v[68] == 0.0);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1615])) && s.b[1623]) {s.store_div_scaled_inputs2_mixed_aii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 1179, 1.0);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1623])) {s.store_div_scaled_inputs3_mixed_aiii(1180, A::add_scaled_product(s.ad_value(1158), 1.0, s.ad_value(1480), s.ad_value(1161), (-1.0)), 1.0, 1479, (-1.0), 736, 1.0, 1179, 1.0);}
        s.b[1624] = (((s.v[1476] <= 0.0) || (s.v[1477] <= 0.0)) || (s.v[1478] < 0.0));s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {s.store_scaled_add_mixed_ia(1180, 1180, A::sqrt_square_offset(s.ad_value(1180), ((4.0 * 0.01) * 0.01)), 0.5);s.store_div_scaled_value_offset_denominator(1181, s.ad_value(1477), 1.0, s.ad_value(1180), 0.001, 1.0);s.store_sub(1183, 1235, 1482);}
        s.b[1625] = (s.v[1183] >= ((-1.0) / 100.0));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && s.b[1625]) {s.store_scale(1184, 1481, (-100.0));}
        if (((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) && (!s.b[1625])) {s.store_div(1184, 1481, 1183);}
        if ((s.b[1613] && (!s.b[1615])) && (!s.b[1624])) {s.store_exp(1185, 1184);}
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
        if (s.b[1613] && (!s.b[1630])) {s.store_mul(1179, 1309, 1282);}
        s.b[1631] = (s.v[1283] == 0.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (s.b[1613] && (!s.b[1631])) {s.store_mul(1179, 1310, 1283);}
        s.b[1632] = (s.v[1286] == 0.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
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
        if (s.b[1613] && (!s.b[1632])) {s.store_mul(1182, 1309, 1286);}
        s.b[1640] = (s.v[1287] == 0.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if (s.b[1613] && (!s.b[1640])) {s.store_mul_scaled_offset_ad_rhs(1305, 664, s.v[783], A::mul_offset_rhs(s.ad_value(617), s.ad_value(771), (-1.0)), 1.0);s.store_mul_scaled_offset_ad_rhs(1306, 666, s.v[783], A::mul_offset_rhs(s.ad_value(618), s.ad_value(771), (-1.0)), 1.0);s.store_div(1179, 1422, 1305);}
        s.b[1641] = (s.v[1179] > 100.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1613] && (!s.b[1640])) && s.b[1641]) {s.store_scaled_offset(1189, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1642] = (s.v[1179] < (-100.0));s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && s.b[1642]) {s.store_scalar(1189, 3.720075976e-44);}
        if (((s.b[1613] && (!s.b[1640])) && (!s.b[1641])) && (!s.b[1642])) {s.store_exp(1189, 1179);}
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
        if (s.b[1613] && (!s.b[1640])) {s.store_mul(1182, 1310, 1287);}
        if s.b[1613] {s.store_scalar(1265, ((s.v[689] / s.v[59]) * s.v[174]));}
        s.b[1648] = ((s.v[1284] == 0.0) && (s.v[1285] == 0.0));s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (s.b[1613] && s.b[1648]) {s.store_scalar(1322, 0.0);s.store_scalar(1323, 0.0);s.store_scalar(1268, 0.0);}
        if (s.b[1613] && (!s.b[1648])) {s.store_mul_scale_offset_indices(1324, 1307, 1318, 1.0, (-1.0));}
        s.b[1649] = (s.v[1324] < 1e-5);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1649]) {s.store_scalar(1324, 0.0);s.store_scalar(1326, 1.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1649])) {s.store_div_from_scalar_sqrt_ad(1326, 1.0, A::offset(s.ad_value(1324), 1.0));}
        if (s.b[1613] && (!s.b[1648])) {s.store_mul_scale_offset_indices(1325, 1308, 1319, 1.0, (-1.0));}
        s.b[1650] = (s.v[1325] < 1e-5);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1650]) {s.store_scalar(1325, 0.0);s.store_scalar(1327, 1.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1650])) {s.store_div_from_scalar_sqrt_ad(1327, 1.0, A::offset(s.ad_value(1325), 1.0));}
        if (s.b[1613] && (!s.b[1648])) {s.store_sub_from_scalar(1179, 1.0, 712);s.store_mul3_lhs(1320, 1265, 1284, 713);s.store_mul(1180, 1179, 1320);s.store_mul3_lhs(1320, 1265, 1285, 713);s.store_mul(1180, 1179, 1320);s.store_mul3_lhs(1321, 1265, 1284, 714);s.store_mul_ad_product_lhs_mixed_ia(1322, 1321, A::offset(s.ad_value(1318), (-1.0)), 1326);s.store_mul3_lhs(1321, 1265, 1285, 714);s.store_mul_ad_product_lhs_mixed_ia(1323, 1321, A::offset(s.ad_value(1319), (-1.0)), 1327);}
        s.b[1651] = (s.v[49] == 1.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1648])) && s.b[1651]) {s.store_scalar(1268, 0.0);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {s.store_offset_div_scaled_inputs2_indices(1179, 1421, 1.0, 1422, 1.0, 715, 1.0, 1.0);s.store_add(1180, 1324, 1325);s.store_sqrt_add_scaled_square_input(1182, 1179, 1.0, 1180, 4.0);s.store_scaled_add(1181, 1179, 1182, 0.5);}
        s.b[1652] = (s.v[1181] < 0.1);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && s.b[1652]) {s.store_scalar(1328, 10.0);}
        if (((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) && (!s.b[1652])) {s.store_div_from_scalar(1328, 1.0, 1181);}
        if ((s.b[1613] && (!s.b[1648])) && (!s.b[1651])) {s.store_mul(1179, 712, 1320);s.store_mul_ad_product_lhs_mixed_ia(1268, 1179, A::sub(s.ad_value(1318), s.ad_value(1319)), 1328);}
        s.b[1653] = ((s.v[1288] == 0.0) && (s.v[1289] == 0.0));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if (s.b[1613] && (!s.b[1653])) {s.store_scale(1267, 659, s.v[783]);}
        s.b[1654] = ((s.v[677] - s.v[1421]) < 0.001);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);}
        s.b[1655] = (s.v[1179] > 100.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && s.b[1654]) && s.b[1655]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1656] = (s.v[1179] < (-100.0));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && s.b[1656]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1654]) && (!s.b[1655])) && (!s.b[1656])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && s.b[1654]) {s.store_mul(1182, 1309, 1288);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(677), s.ad_value(1421));s.store_mul_div_scaled_inputs_product_lhs(1179, 1421, -1.0, 1267, 1.0, 677, 1180);}
        s.b[1657] = (s.v[1179] > 100.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && s.b[1657]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1658] = (s.v[1179] < (-100.0));s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && s.b[1658]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) && (!s.b[1657])) && (!s.b[1658])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1654])) {s.store_mul(1182, 1309, 1288);}
        if (s.b[1613] && (!s.b[1653])) {s.store_scale(1267, 660, s.v[783]);}
        s.b[1659] = ((s.v[678] - s.v[1422]) < 0.001);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {s.store_scalar(1180, 1000.0);s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);}
        s.b[1660] = (s.v[1179] > 100.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && s.b[1659]) && s.b[1660]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1661] = (s.v[1179] < (-100.0));s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && s.b[1661]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && s.b[1659]) && (!s.b[1660])) && (!s.b[1661])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && s.b[1659]) {s.store_mul(1182, 1310, 1289);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {s.store_div_from_scalar_sub_ad(1180, 1.0, s.ad_value(678), s.ad_value(1422));s.store_mul_div_scaled_inputs_product_lhs(1179, 1422, -1.0, 1267, 1.0, 678, 1180);}
        s.b[1662] = (s.v[1179] > 100.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && s.b[1662]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1663] = (s.v[1179] < (-100.0));s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && s.b[1663]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) && (!s.b[1662])) && (!s.b[1663])) {s.store_exp(1180, 1179);}
        if ((s.b[1613] && (!s.b[1653])) && (!s.b[1659])) {s.store_mul(1182, 1310, 1289);}
        if (!s.b[1613]) {s.store_scalar(1322, 0.0);s.store_scalar(1323, 0.0);s.store_scalar(1268, 0.0);}
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
        s.b[1670] = (s.v[1362] < (-100.0));s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1669])) && s.b[1670]) {s.store_scale(1412, 1179, (((1.0 + 3.720075976e-44)) as f64).ln());}
        if (((s.v[356] != 0.0) && (!s.b[1669])) && (!s.b[1670])) {s.store_exp(1363, 1362);s.store_mul_ln_mixed_ia(1412, 1179, A::offset(s.ad_value(1363), 1.0));}
        if (s.v[356] != 0.0) {s.store_mul(1181, 1161, 1412);s.copy_ad(1190, 730);s.copy_ad(1191, 731);s.store_add_scaled_product_indices(1182, 573, (-1.0), 572, 574, 1.0);s.store_mul(1183, 573, 574);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if (s.v[356] != 0.0) {s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(572), 1.0, s.ad_value(1182), s.ad_value(1416), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1416), s.ad_value(1416)));}
        s.b[1671] = (s.v[1184] > 100.0);s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1671]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1672] = (s.v[1184] < (-100.0));s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1671])) && s.b[1672]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1671])) && (!s.b[1672])) {s.store_exp(1185, 1184);}
        if (s.v[356] != 0.0) {s.store_mul_scale_offset_indices(1186, 1158, 579, -1.0, 0.0);s.store_offset_square(1187, 1186, 0.0002);}
        s.b[1673] = (s.v[1186] > 100.0);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1673]) {s.store_scalar(1188, 2.688117142e43);}
        s.b[1674] = (s.v[1186] < (-100.0));s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1673])) && s.b[1674]) {s.store_scalar(1188, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1673])) && (!s.b[1674])) {s.store_exp(1188, 1186);}
        if (s.v[356] != 0.0) {s.store_offset(1180, 1188, (((-1.0)) + (0.0001)));s.store_div_scaled_inputs2_indices(1189, 1180, 1.0, 1186, (-1.0), 1187, 1.0);s.store_offset(1180, 1188, (((-1.0)) + ((-0.0001))));s.store_div_scaled_add_product_indices(1189, 1180, (-1.0), 1186, 1188, 1.0, 1187, 1.0);s.store_sub(1179, 1157, 736);s.store_sqrt_square_offset(1360, 1179, 0.0001);s.store_mul(1181, 1157, 1360);s.copy_ad(1299, 733);s.copy_ad(1300, 734);s.copy_ad(1191, 735);s.store_add_scaled_product_indices(1182, 576, (-1.0), 575, 577, 1.0);s.store_mul(1183, 576, 577);s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1360), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1360), s.ad_value(1360)));}
        s.b[1675] = (s.v[1184] > 100.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1675]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1676] = (s.v[1184] < (-100.0));s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1675])) && s.b[1676]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1675])) && (!s.b[1676])) {s.store_exp(1185, 1184);}
        if (s.v[356] != 0.0) {s.store_sub(1179, 1156, 736);s.store_sqrt_square_offset(1361, 1179, 0.0001);s.store_mul(1181, 1156, 1361);s.store_mul_sub_mixed_iaa(1184, 1191, A::add_scaled_product(s.ad_value(575), 1.0, s.ad_value(1182), s.ad_value(1361), 1.0), A::mul3(s.ad_value(1183), s.ad_value(1361), s.ad_value(1361)));}
        s.b[1677] = (s.v[1184] > 100.0);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((s.v[356] != 0.0) && s.b[1677]) {s.store_scalar(1185, 2.688117142e43);}
        s.b[1678] = (s.v[1184] < (-100.0));s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if (((s.v[356] != 0.0) && (!s.b[1677])) && s.b[1678]) {s.store_scalar(1185, 3.720075976e-44);}
        if (((s.v[356] != 0.0) && (!s.b[1677])) && (!s.b[1678])) {s.store_exp(1185, 1184);}
        s.b[1679] = ((s.v[355] != 0.0) && (s.v[57] != 2.0));s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if s.b[1679] {s.store_scalar(1411, s.v[706]);s.copy_ad(1410, 1416);s.store_scalar(1179, s.v[374]);s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));s.copy_ad(1410, 1414);s.store_scaled_offset(1179, 1410, (-s.v[362]), 1.0 / (s.v[363]));}
        s.b[1680] = (s.v[1179] > 100.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1680]) {s.store_scaled_offset(1180, 1179, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1681] = (s.v[1179] < (-100.0));s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1680])) && s.b[1681]) {s.store_scalar(1180, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1680])) && (!s.b[1681])) {s.store_exp(1180, 1179);}
        if s.b[1679] {s.store_scaled_ln_ad(1412, A::offset(s.ad_value(1180), 1.0), s.v[363]);}
        s.b[1682] = (s.v[366] != 0.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1682]) {s.store_sub_from_scalar_scaled_input(1179, 1.0, 1410, 1.0 / (s.v[366]));}
        if (s.b[1679] && (!s.b[1682])) {s.store_scalar(1179, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        s.b[1683] = (s.v[1179] < 0.01);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1683]) {s.store_scalar(1179, 0.01);}
        if s.b[1679] {s.store_mul_ad_product_lhs_mixed_ai(1180, A::scale_offset(s.ad_value(1228), (s.v[1227] * 1.0 / (s.v[59])), (s.v[64] / s.v[39])), 784, 1411);s.store_scale(1181, 785, s.v[357]);s.copy_ad(1182, 609);s.copy_ad(1183, 610);s.store_div_scaled_product_mixed_iai(1185, 1181, A::add_scaled_product(s.ad_value(1182), 1.0, s.ad_value(1183), s.ad_value(1410), (-1.0)), 1.0, 1179, 1.0);}
        s.b[1684] = (s.v[1185] > 100.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if (s.b[1679] && s.b[1684]) {s.store_scaled_offset(1184, 1185, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1685] = (s.v[1185] < (-100.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if ((s.b[1679] && (!s.b[1684])) && s.b[1685]) {s.store_scalar(1184, 3.720075976e-44);}
        if ((s.b[1679] && (!s.b[1684])) && (!s.b[1685])) {s.store_exp(1184, 1185);}
        if s.b[1679] {s.copy_ad(1410, 1415);s.store_scalar(1179, s.v[374]);s.store_offset_sub(1180, 1179, 1410, (-s.v[375]));s.store_sqrt_add_scaled_square_input(1182, 1180, 1.0, 1179, (4.0 * s.v[375]));s.store_add_scaled_inputs3_indices(1414, 1179, 1.0, 1180, (-0.5), 1182, (-0.5));s.copy_ad(1410, 1414);s.store_scaled_sub(1179, 1162, 1409, 1.0 / (s.v[367]));}
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
        if s.b[1679] {s.store_add(1460, 1162, 781);}
        s.b[1693] = (((((s.v[355] != 0.0) && (s.v[57] != 2.0)) && (s.v[760] != 0.0)) && (s.v[63] > 0.0)) && (s.v[1447] < s.v[1460]));s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if s.b[1693] {s.store_sub(1179, 1447, 1460);s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_offset_scaled_sub(1446, 1180, 1179, 0.5, (((-0.01)) * (0.5)));}
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
        if s.b[1693] {s.store_scale(1190, 1190, (s.v[63] * s.v[706]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
    ) {
        s.b[1696] = (s.v[57] != 2.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });s.b[1697] = (s.v[71] == 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });s.b[1698] = (s.v[570] <= 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {s.store_add_scaled_product_mixed_iia(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);s.store_scale(1179, 640, s.v[1227]);s.store_div_scaled_product_offset_denominator_indices(1180, 641, 1179, 1.0, 1179, 1.0, 1.0);s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);s.store_add(1182, 1179, 643);s.store_mul(1181, 1166, 1182);s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);s.store_mul3_lhs(1302, 1180, 1181, 1182);s.store_add(1256, 1301, 1302);s.store_sub(1304, 1158, 1256);s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));}
        s.b[1699] = (s.v[1179] < 1e-5);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1696] && s.b[1697]) && (!s.b[1698])) && s.b[1699]) {s.store_scalar(1179, 1e-5);}
        if ((s.b[1696] && s.b[1697]) && (!s.b[1698])) {s.store_add_product3_rhs_indices(1179, 1220, 630, 759, 1268, 1.0);}
        s.b[1703] = (s.v[570] <= 0.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {s.store_add_scaled_product_mixed_iia(1301, 639, (-1.0 / (s.v[1227])), 638, A::scale_offset(s.ad_value(771), s.v[289], (((((-1.0)) * (s.v[289]))) + (1.0))), 1.0);s.store_scale(1179, 640, s.v[1227]);s.store_div_scaled_product_offset_denominator_indices(1180, 641, 1179, 1.0, 1179, 1.0, 1.0);s.store_div_from_scalar_offset_product(1179, 1.0, 642, 1210, 1.0);s.store_add(1182, 1179, 643);s.store_mul(1181, 1166, 1182);s.store_div_from_scalar_offset_product(1182, 1.0, 644, 1158, 1.0);s.store_mul3_lhs(1302, 1180, 1181, 1182);s.store_add(1256, 1301, 1302);s.store_sub(1304, 1158, 1256);s.store_add_ad(1179, A::add_scaled_product(s.ad_value(637), 1.0, s.ad_value(636), s.ad_value(1304), 1.0), A::mul3(s.ad_value(571), s.ad_value(1304), s.ad_value(1304)));}
        s.b[1704] = (s.v[1179] < 1e-5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) && s.b[1704]) {s.store_scalar(1179, 1e-5);}
        if ((s.b[1696] && (!s.b[1697])) && (!s.b[1703])) {s.copy_ad(1179, 1220);}
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
        s.b[1712] = ((s.v[760] == 0.0) || (s.v[760] == 2.0));s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });s.b[1713] = (s.v[526] < 0.001);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });s.b[1714] = (s.v[427] <= 0.001);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && s.b[1714]) {s.store_scalar(1179, (1.0 / 0.001));}
        if (((s.b[1696] && (!s.b[1712])) && s.b[1713]) && (!s.b[1714])) {s.store_scalar(1179, (1.0 / s.v[427]));}
        s.b[1715] = (s.v[66] > 1.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if s.b[1715] {s.store_mul(1188, 596, 409);s.store_mul(1179, 1188, 1215);s.store_mul_add_rhs(413, 595, 1179, 1420);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
    ) {
        s.b[1716] = (s.v[39] != 1.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if (s.b[1715] && s.b[1716]) {s.store_scale(413, 413, s.v[39]);}
        s.b[1717] = (s.v[66] == 2.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (s.b[1715] && s.b[1717]) {s.store_add(1190, 421, 413);s.store_div_scaled_product_indices(413, 421, 413, 1.0, 1190, 1.0);}
        if (!s.b[1715]) {s.store_scalar(413, 0.0);}
        s.b[1718] = (s.v[403] == 1.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if s.b[1718] {s.store_scalar(1222, 0.0);s.store_sub(1179, 1157, 736);s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_scaled_add(1360, 1179, 1180, 0.5);s.store_offset_mul(1179, 553, 1360, 1.0);s.store_mul_scale_offset_indices(1180, 1154, 554, -1.0, 0.0);s.store_add_mixed_ai(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);s.store_add_mixed_ia(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));s.store_scale(1183, 1430, 0.5);s.store_sub(1179, 1156, 736);s.store_sqrt_square_offset(1180, 1179, 0.0001);s.store_scaled_add(1361, 1179, 1180, 0.5);s.store_offset_mul(1179, 553, 1361, 1.0);s.store_mul_scale_offset_indices(1180, 1153, 554, -1.0, 0.0);s.store_add_mixed_ai(1181, A::div_from_scalar(1.0, s.ad_value(1179)), 1180);s.store_add_mixed_ia(1182, 1181, A::sqrt_square_offset(s.ad_value(1181), 0.01));s.store_scale(1183, 1429, 0.5);}
        s.store_mul_scale_offset_mixed_ia(1180, 1210, A::div_scaled_product(s.ad_value(1195), s.ad_value(1211), 0.5, s.ad_value(1225), 1.0), -1.0, 1.0);s.b[1720] = (s.v[39] != 1.0);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if s.b[1720] {s.store_scale(1220, 1220, s.v[39]);s.store_scale(1268, 1268, s.v[39]);s.store_scale(454, 454, s.v[39]);}
        s.store_scalar(439, (A::ddx_projection(&s.ad_value(1220), Some(9), None) * s.v[36]));s.b[1721] = (s.v[759] > 0.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if s.b[1721] {s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(7), None) * s.v[36]));}
        if (!s.b[1721]) {s.store_scalar(440, (A::ddx_projection(&s.ad_value(1220), Some(8), None) * s.v[36]));}
        s.store_scalar(441, (A::ddx_projection(&s.ad_value(1220), Some(5), None) * s.v[36]));s.store_scale(1178, 757, ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[692]) + s.v[62]));s.store_scale(1316, 757, (s.v[342] * ((((s.v[693] / s.v[59]) * s.v[39]) * s.v[726]) + s.v[62])));s.store_scale(1448, 757, s.v[63]);s.store_scale(1449, 757, (s.v[342] * s.v[63]));s.store_sub(1166, 1161, 1407);s.store_mul(1189, 1393, 1168);s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);s.store_mul3_lhs(1351, 1393, 724, 1168);s.store_mul3_lhs(1352, 1393, 725, 1168);s.b[1722] = (s.v[69] == 0.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });s.b[1723] = ((s.v[1145] > (-100.0)) && (s.v[1145] < 100.0));s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if (s.b[1722] && s.b[1723]) {let t0: A = A::exp(s.ad_value(1145));s.store_square_ad(1146, t0);s.store_mul_mixed_ia(1146, 1146, A::exp_scaled_input(A::div(s.ad_value(685), s.ad_value(1351)), -1.0));}
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
    }
}
