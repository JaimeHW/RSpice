#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1571]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1571])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, 604, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1572] = (s.v[1228] > 0.0);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1572]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1572])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1573] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1573]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1573])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1574] = (s.v[1228] > 0.0);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1574]) {s.copy_ad(1229, 1191);}
        s.b[1575] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) && s.b[1575]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) && (!s.b[1575])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 527, 1215, 1230, 1224, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1576] = (s.v[533] == 0.0);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1576]) {s.store_scalar(1231, 0.0);}
        s.b[1577] = (s.v[513] == 0.5);s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && s.b[1577]) {s.store_sqrt_mul_sub_lhs(1207, 510, 1205, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1577])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(510), s.ad_value(1205)), 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), 595, 1.0, 1207, 1.0);}
        s.b[1578] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && s.b[1578]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0));}
        s.b[1579] = (((-s.v[610]) / s.v[1232]) < 0.0);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 610, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 610, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 533, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1580] = (s.v[542] > 1000.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1580]) {s.store_scalar(1233, 1.0);}
        s.b[1581] = (s.v[1206] > ((-s.v[444]) * s.v[542]));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });s.b[1582] = (s.v[545] == 4.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) && s.b[1582]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), 1206, 616);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) && (!s.b[1582])) {s.store_pow_abs_mul_base_indices(1207, 1206, 616, 545);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && (!s.b[1581])) {s.store_add_scaled_product_mixed_iai(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1566])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(475, 673, 1234, 1.0, 674, 1235, 1.0, 675, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1583] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });s.b[1584] = (s.v[486] < s.v[681]);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });s.b[1585] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && s.b[1585]) {s.store_primal_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));}
        s.b[1586] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && s.b[1586]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && (!s.b[1586])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1584])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 682, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(681), s.v[371]), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
    ) {
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1584])) {s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1587] = (s.v[486] > 0.0);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1587]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1587])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);}
        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {s.store_primal_sub(1203, 683, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 486, 0.5, 1203, 0.5, 486, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 486, 0.5, 686, 0.5, 486, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_sqrt_square_offset_rhs(1206, 486, 486, ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1588] = (s.v[673] == 0.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1588]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {s.store_primal_mul(1208, 563, 1198);}
        s.b[1589] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1589]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {s.store_primal_sub(1210, 569, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1590] = (s.v[511] == 0.5);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && s.b[1590]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && (!s.b[1590])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(511), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1591] = (s.v[511] == 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && s.b[1591]) {s.store_sqrt_mul(1207, 1210, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && (!s.b[1591])) {s.store_pow_mul_base_indices(1207, 1210, 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {s.store_mul(1214, 590, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 560, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 522, 1215, 1213);}
        s.b[1592] = (s.v[525] == 0.0);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1592]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {s.store_mul_div_scaled_product_indices(1217, 605, 1214, 575, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1593] = (((-s.v[511]) * s.v[578]) == (-1.0));s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1593]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1593])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, 602, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1594] = (s.v[1228] > 0.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1594]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1594])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1595] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1595]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1595])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1596] = (s.v[1228] > 0.0);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1596]) {s.copy_ad(1229, 1191);}
        s.b[1597] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) && s.b[1597]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) && (!s.b[1597])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 525, 1215, 1230, 1224, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1598] = (s.v[531] == 0.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1598]) {s.store_scalar(1231, 0.0);}
        s.b[1599] = (s.v[511] == 0.5);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && s.b[1599]) {s.store_sqrt_mul_sub_lhs(1207, 508, 1205, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1599])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(508), s.ad_value(1205)), 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), 593, 1.0, 1207, 1.0);}
        s.b[1600] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && s.b[1600]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0));}
        s.b[1601] = (((-s.v[608]) / s.v[1232]) < 0.0);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1600])) && s.b[1601]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 608, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1600])) && (!s.b[1601])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 608, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 531, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1602] = (s.v[540] > 1000.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1602]) {s.store_scalar(1233, 1.0);}
        s.b[1603] = (s.v[1206] > ((-s.v[444]) * s.v[540]));s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });s.b[1604] = (s.v[543] == 4.0);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) && s.b[1604]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), 1206, 614);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) && (!s.b[1604])) {s.store_pow_abs_mul_base_indices(1207, 1206, 614, 543);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && (!s.b[1603])) {s.store_add_scaled_product_mixed_iai(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1605] = (s.v[674] == 0.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1605]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1605])) {s.store_primal_mul(1208, 564, 1198);}
        s.b[1606] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1606]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {s.store_primal_sub(1210, 570, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1607] = (s.v[512] == 0.5);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1607]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1607])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {s.store_primal_add(1213, 1211, 1212);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        s: &mut Scratch,
    ) {
        s.b[1608] = (s.v[512] == 0.5);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1608]) {s.store_sqrt_mul(1207, 1210, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1608])) {s.store_pow_mul_base_indices(1207, 1210, 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {s.store_mul(1214, 591, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 561, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 523, 1215, 1213);}
        s.b[1609] = (s.v[526] == 0.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1609]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {s.store_mul_div_scaled_product_indices(1217, 606, 1214, 576, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1610] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1610]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1610])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, 603, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1611] = (s.v[1228] > 0.0);s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1611]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1611])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1612] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1612]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1612])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1613] = (s.v[1228] > 0.0);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1613]) {s.copy_ad(1229, 1191);}
        s.b[1614] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) && s.b[1614]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) && (!s.b[1614])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 526, 1215, 1230, 1224, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1615] = (s.v[532] == 0.0);s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1615]) {s.store_scalar(1231, 0.0);}
        s.b[1616] = (s.v[512] == 0.5);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && s.b[1616]) {s.store_sqrt_mul_sub_lhs(1207, 509, 1205, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1616])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(509), s.ad_value(1205)), 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), 594, 1.0, 1207, 1.0);}
        s.b[1617] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && s.b[1617]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0));}
        s.b[1618] = (((-s.v[609]) / s.v[1232]) < 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1617])) && s.b[1618]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 609, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1617])) && (!s.b[1618])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 609, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 532, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1619] = (s.v[541] > 1000.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1619]) {s.store_scalar(1233, 1.0);}
        s.b[1620] = (s.v[1206] > ((-s.v[444]) * s.v[541]));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });s.b[1621] = (s.v[544] == 4.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && s.b[1621]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), 1206, 615);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && (!s.b[1621])) {s.store_pow_abs_mul_base_indices(1207, 1206, 615, 544);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && (!s.b[1620])) {s.store_add_scaled_product_mixed_iai(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1605])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1622] = (s.v[675] == 0.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1622]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1622])) {s.store_primal_mul(1208, 565, 1198);}
        s.b[1623] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1623]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {s.store_primal_sub(1210, 571, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1624] = (s.v[513] == 0.5);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1624]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1624])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {s.store_primal_add(1213, 1211, 1212);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        s: &mut Scratch,
    ) {
        s.b[1625] = (s.v[513] == 0.5);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1625]) {s.store_sqrt_mul(1207, 1210, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1625])) {s.store_pow_mul_base_indices(1207, 1210, 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {s.store_mul(1214, 592, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 562, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 524, 1215, 1213);}
        s.b[1626] = (s.v[527] == 0.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1626]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {s.store_mul_div_scaled_product_indices(1217, 607, 1214, 577, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1627] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1627]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1627])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, 604, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1628] = (s.v[1228] > 0.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1628]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1628])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1629] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1629]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1629])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1630] = (s.v[1228] > 0.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1630]) {s.copy_ad(1229, 1191);}
        s.b[1631] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) && s.b[1631]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) && (!s.b[1631])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 527, 1215, 1230, 1224, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1632] = (s.v[533] == 0.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1632]) {s.store_scalar(1231, 0.0);}
        s.b[1633] = (s.v[513] == 0.5);s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && s.b[1633]) {s.store_sqrt_mul_sub_lhs(1207, 510, 1205, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1633])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(510), s.ad_value(1205)), 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), 595, 1.0, 1207, 1.0);}
        s.b[1634] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && s.b[1634]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0));}
        s.b[1635] = (((-s.v[610]) / s.v[1232]) < 0.0);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1634])) && s.b[1635]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 610, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1634])) && (!s.b[1635])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 610, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 533, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1636] = (s.v[542] > 1000.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1636]) {s.store_scalar(1233, 1.0);}
        s.b[1637] = (s.v[1206] > ((-s.v[444]) * s.v[542]));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });s.b[1638] = (s.v[545] == 4.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) && s.b[1638]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), 1206, 616);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) && (!s.b[1638])) {s.store_pow_abs_mul_base_indices(1207, 1206, 616, 545);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && (!s.b[1637])) {s.store_add_scaled_product_mixed_iai(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1622])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(476, 673, 1234, 1.0, 674, 1235, 1.0, 675, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1639] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });s.b[1640] = (s.v[487] < s.v[681]);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });s.b[1641] = (((((-0.5) * (s.v[487] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {s.store_primal_exp_scaled_input(1200, 487, (s.v[371] * (-0.5)));}
        s.b[1642] = (((-0.5) * (s.v[487] * s.v[371])) < 0.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && (!s.b[1640])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 682, A::sub_scaled_inputs(s.ad_value(487), s.v[371], s.ad_value(681), s.v[371]), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
    ) {
        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && (!s.b[1640])) {s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1639]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1643] = (s.v[487] > 0.0);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1643]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && (!s.b[1643])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 487);}
        if ((s.b[1171] && s.b[1188]) && s.b[1639]) {s.store_primal_sub(1203, 683, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 487, 0.5, 1203, 0.5, 487, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 487, 0.5, 686, 0.5, 487, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_sqrt_square_offset_rhs(1206, 487, 487, ((4.0 * 1e-6) * 1e-6), 0.5);}
        s.b[1644] = (s.v[673] == 0.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1644]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1644])) {s.store_primal_mul(1208, 563, 1198);}
        s.b[1645] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1645]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {s.store_primal_sub(1210, 569, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1646] = (s.v[511] == 0.5);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && (!s.b[1646])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(511), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1647] = (s.v[511] == 0.5);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && s.b[1647]) {s.store_sqrt_mul(1207, 1210, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && (!s.b[1647])) {s.store_pow_mul_base_indices(1207, 1210, 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {s.store_mul(1214, 590, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 560, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 522, 1215, 1213);}
        s.b[1648] = (s.v[525] == 0.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1648]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {s.store_mul_div_scaled_product_indices(1217, 605, 1214, 575, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1649] = (((-s.v[511]) * s.v[578]) == (-1.0));s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1649]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1649])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, 602, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1650] = (s.v[1228] > 0.0);s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1650]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1650])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1651] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1651]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1651])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1652] = (s.v[1228] > 0.0);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1652]) {s.copy_ad(1229, 1191);}
        s.b[1653] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) && s.b[1653]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) && (!s.b[1653])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 525, 1215, 1230, 1224, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1654] = (s.v[531] == 0.0);s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1654]) {s.store_scalar(1231, 0.0);}
        s.b[1655] = (s.v[511] == 0.5);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && s.b[1655]) {s.store_sqrt_mul_sub_lhs(1207, 508, 1205, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1655])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(508), s.ad_value(1205)), 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), 593, 1.0, 1207, 1.0);}
        s.b[1656] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && s.b[1656]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0));}
        s.b[1657] = (((-s.v[608]) / s.v[1232]) < 0.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1656])) && s.b[1657]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 608, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1656])) && (!s.b[1657])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 608, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 531, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1658] = (s.v[540] > 1000.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1658]) {s.store_scalar(1233, 1.0);}
        s.b[1659] = (s.v[1206] > ((-s.v[444]) * s.v[540]));s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });s.b[1660] = (s.v[543] == 4.0);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) && s.b[1660]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), 1206, 614);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) && (!s.b[1660])) {s.store_pow_abs_mul_base_indices(1207, 1206, 614, 543);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && (!s.b[1659])) {s.store_add_scaled_product_mixed_iai(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1644])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1661] = (s.v[674] == 0.0);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1661]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1661])) {s.store_primal_mul(1208, 564, 1198);}
        s.b[1662] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1662]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {s.store_primal_sub(1210, 570, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1663] = (s.v[512] == 0.5);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {s.store_primal_add(1213, 1211, 1212);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        s: &mut Scratch,
    ) {
        s.b[1664] = (s.v[512] == 0.5);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1664]) {s.store_sqrt_mul(1207, 1210, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1664])) {s.store_pow_mul_base_indices(1207, 1210, 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {s.store_mul(1214, 591, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 561, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 523, 1215, 1213);}
        s.b[1665] = (s.v[526] == 0.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1665]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {s.store_mul_div_scaled_product_indices(1217, 606, 1214, 576, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1666] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1666]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1666])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, 603, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1667] = (s.v[1228] > 0.0);s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1667]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1667])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1668] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1668]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1668])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1669] = (s.v[1228] > 0.0);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1669]) {s.copy_ad(1229, 1191);}
        s.b[1670] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) && s.b[1670]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) && (!s.b[1670])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 526, 1215, 1230, 1224, 1.0);}
    }
}
