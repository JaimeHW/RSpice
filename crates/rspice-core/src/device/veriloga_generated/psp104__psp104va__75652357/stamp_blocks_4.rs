#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && (!s.b[1631])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 481);}
        if ((s.b[1159] && s.b[1176]) && s.b[1627]) {s.store_primal_sub(1191, 677, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 481, 0.5, 1191, 0.5, 481, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 481, 0.5, 680, 0.5, 481, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 481, A::sqrt_square_offset(s.ad_value(481), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1632] = (s.v[667] == 0.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1632]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1632])) {s.store_primal_mul(1196, 557, 1186);}
        s.b[1633] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1633]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {s.store_primal_sub(1198, 563, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1634] = (s.v[505] == 0.5);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1634])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1635] = (s.v[505] == 0.5);s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && s.b[1635]) {s.store_sqrt_mul(1195, 1198, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1635])) {s.store_pow_mul_base_indices(1195, 1198, 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {s.store_mul(1202, 584, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 516, 1203, 1201);}
        s.b[1636] = (s.v[519] == 0.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1636]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1637] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1637]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1637])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, 596, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1638] = (s.v[1216] > 0.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1638]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1638])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1639] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1639]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1639])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1640] = (s.v[1216] > 0.0);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1640]) {s.copy_ad(1217, 1179);}
        s.b[1641] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) && s.b[1641]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) && (!s.b[1641])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);}
        s.b[1642] = (s.v[525] == 0.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1642]) {s.store_scalar(1219, 0.0);}
        s.b[1643] = (s.v[505] == 0.5);s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && s.b[1643]) {s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1643])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);}
        s.b[1644] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && s.b[1644]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));}
        s.b[1645] = (((-s.v[602]) / s.v[1220]) < 0.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1644])) && s.b[1645]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1644])) && (!s.b[1645])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1646] = (s.v[534] > 1000.0);s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1646]) {s.store_scalar(1221, 1.0);}
        s.b[1647] = (s.v[1194] > ((-s.v[438]) * s.v[534]));s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });s.b[1648] = (s.v[537] == 4.0);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) && s.b[1648]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) && (!s.b[1648])) {s.store_pow_abs_mul_base_indices(1195, 1194, 608, 537);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && (!s.b[1647])) {s.store_add_scaled_product_mixed_iai(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1632])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1649] = (s.v[668] == 0.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
    ) {
        if ((s.b[1159] && s.b[1176]) && s.b[1649]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1649])) {s.store_primal_mul(1196, 558, 1186);}
        s.b[1650] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1650]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {s.store_primal_sub(1198, 564, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1651] = (s.v[506] == 0.5);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1652] = (s.v[506] == 0.5);s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1652]) {s.store_sqrt_mul(1195, 1198, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1652])) {s.store_pow_mul_base_indices(1195, 1198, 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {s.store_mul(1202, 585, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 517, 1203, 1201);}
        s.b[1653] = (s.v[520] == 0.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1653]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1654] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1654]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1654])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, 597, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1655] = (s.v[1216] > 0.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1655]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1655])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1656] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1656]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1656])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1657] = (s.v[1216] > 0.0);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1657]) {s.copy_ad(1217, 1179);}
        s.b[1658] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) && s.b[1658]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) && (!s.b[1658])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);}
        s.b[1659] = (s.v[526] == 0.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1659]) {s.store_scalar(1219, 0.0);}
        s.b[1660] = (s.v[506] == 0.5);s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && s.b[1660]) {s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1660])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);}
        s.b[1661] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && s.b[1661]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));}
        s.b[1662] = (((-s.v[603]) / s.v[1220]) < 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1661])) && s.b[1662]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1661])) && (!s.b[1662])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1663] = (s.v[535] > 1000.0);s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1663]) {s.store_scalar(1221, 1.0);}
        s.b[1664] = (s.v[1194] > ((-s.v[438]) * s.v[535]));s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });s.b[1665] = (s.v[538] == 4.0);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) && s.b[1665]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) && (!s.b[1665])) {s.store_pow_abs_mul_base_indices(1195, 1194, 609, 538);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && (!s.b[1664])) {s.store_add_scaled_product_mixed_iai(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1649])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1666] = (s.v[669] == 0.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1666]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1666])) {s.store_primal_mul(1196, 559, 1186);}
        s.b[1667] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1667]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {s.store_primal_sub(1198, 565, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1668] = (s.v[507] == 0.5);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1668]) {s.store_scalar(1200, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1668])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(507), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1669] = (s.v[507] == 0.5);s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1669]) {s.store_sqrt_mul(1195, 1198, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1669])) {s.store_pow_mul_base_indices(1195, 1198, 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {s.store_mul(1202, 586, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 518, 1203, 1201);}
        s.b[1670] = (s.v[521] == 0.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1670]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1671] = (((-s.v[507]) * s.v[574]) == (-1.0));s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1671]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1671])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, 598, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1672] = (s.v[1216] > 0.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1672]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1672])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1673] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1673]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1673])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1674] = (s.v[1216] > 0.0);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1674]) {s.copy_ad(1217, 1179);}
        s.b[1675] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) && s.b[1675]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) && (!s.b[1675])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);}
        s.b[1676] = (s.v[527] == 0.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1676]) {s.store_scalar(1219, 0.0);}
        s.b[1677] = (s.v[507] == 0.5);s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && s.b[1677]) {s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1677])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);}
        s.b[1678] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && s.b[1678]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));}
        s.b[1679] = (((-s.v[604]) / s.v[1220]) < 0.0);s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1678])) && s.b[1679]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1678])) && (!s.b[1679])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1680] = (s.v[536] > 1000.0);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1680]) {s.store_scalar(1221, 1.0);}
        s.b[1681] = (s.v[1194] > ((-s.v[438]) * s.v[536]));s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });s.b[1682] = (s.v[539] == 4.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) && s.b[1682]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) && (!s.b[1682])) {s.store_pow_abs_mul_base_indices(1195, 1194, 610, 539);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && (!s.b[1681])) {s.store_add_scaled_product_mixed_iai(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1666])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(471, 667, 1222, 1.0, 668, 1223, 1.0, 669, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1683] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });s.b[1684] = (s.v[482] < s.v[675]);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });s.b[1685] = (((((-0.5) * (s.v[482] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && s.b[1685]) {s.store_primal_exp_scaled_input(1188, 482, (s.v[365] * (-0.5)));}
        s.b[1686] = (((-0.5) * (s.v[482] * s.v[365])) < 0.0);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && s.b[1686]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(482), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && (!s.b[1684])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 676, A::sub_scaled_inputs(s.ad_value(482), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1683]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1687] = (s.v[482] > 0.0);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1687]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && (!s.b[1687])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 482);}
        if ((s.b[1159] && s.b[1176]) && s.b[1683]) {s.store_primal_sub(1191, 677, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 482, 0.5, 1191, 0.5, 482, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 482, 0.5, 680, 0.5, 482, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 482, A::sqrt_square_offset(s.ad_value(482), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1688] = (s.v[667] == 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1688]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1688])) {s.store_primal_mul(1196, 557, 1186);}
        s.b[1689] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1689]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {s.store_primal_sub(1198, 563, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1690] = (s.v[505] == 0.5);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && (!s.b[1690])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1691] = (s.v[505] == 0.5);s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && s.b[1691]) {s.store_sqrt_mul(1195, 1198, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && (!s.b[1691])) {s.store_pow_mul_base_indices(1195, 1198, 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {s.store_mul(1202, 584, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 516, 1203, 1201);}
        s.b[1692] = (s.v[519] == 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1692]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1693] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1693]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1693])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, 596, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1694] = (s.v[1216] > 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1694]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1694])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1695] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1695]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1695])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1696] = (s.v[1216] > 0.0);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1696]) {s.copy_ad(1217, 1179);}
        s.b[1697] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) && s.b[1697]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) && (!s.b[1697])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);}
        s.b[1698] = (s.v[525] == 0.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1698]) {s.store_scalar(1219, 0.0);}
        s.b[1699] = (s.v[505] == 0.5);s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && s.b[1699]) {s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1699])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);}
        s.b[1700] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && s.b[1700]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));}
        s.b[1701] = (((-s.v[602]) / s.v[1220]) < 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1700])) && s.b[1701]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1700])) && (!s.b[1701])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1702] = (s.v[534] > 1000.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1702]) {s.store_scalar(1221, 1.0);}
        s.b[1703] = (s.v[1194] > ((-s.v[438]) * s.v[534]));s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });s.b[1704] = (s.v[537] == 4.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) && s.b[1704]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) && (!s.b[1704])) {s.store_pow_abs_mul_base_indices(1195, 1194, 608, 537);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && (!s.b[1703])) {s.store_add_scaled_product_mixed_iai(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1688])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1705] = (s.v[668] == 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1705]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1705])) {s.store_primal_mul(1196, 558, 1186);}
        s.b[1706] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1706]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {s.store_primal_sub(1198, 564, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1707] = (s.v[506] == 0.5);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1707]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1707])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1708] = (s.v[506] == 0.5);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1708]) {s.store_sqrt_mul(1195, 1198, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1708])) {s.store_pow_mul_base_indices(1195, 1198, 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {s.store_mul(1202, 585, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 517, 1203, 1201);}
        s.b[1709] = (s.v[520] == 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1709]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1710] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1710]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1710])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, 597, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1711] = (s.v[1216] > 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1711]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1711])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1712] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1712]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1712])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1713] = (s.v[1216] > 0.0);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1713]) {s.copy_ad(1217, 1179);}
        s.b[1714] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) && s.b[1714]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) && (!s.b[1714])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);}
        s.b[1715] = (s.v[526] == 0.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1715]) {s.store_scalar(1219, 0.0);}
        s.b[1716] = (s.v[506] == 0.5);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && s.b[1716]) {s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1716])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);}
        s.b[1717] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && s.b[1717]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));}
        s.b[1718] = (((-s.v[603]) / s.v[1220]) < 0.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1717])) && s.b[1718]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1717])) && (!s.b[1718])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1719] = (s.v[535] > 1000.0);s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1719]) {s.store_scalar(1221, 1.0);}
        s.b[1720] = (s.v[1194] > ((-s.v[438]) * s.v[535]));s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });s.b[1721] = (s.v[538] == 4.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) && s.b[1721]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) && (!s.b[1721])) {s.store_pow_abs_mul_base_indices(1195, 1194, 609, 538);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && (!s.b[1720])) {s.store_add_scaled_product_mixed_iai(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1705])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1722] = (s.v[669] == 0.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1722]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1722])) {s.store_primal_mul(1196, 559, 1186);}
        s.b[1723] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1723]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {s.store_primal_sub(1198, 565, 1192);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1724] = (s.v[507] == 0.5);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && (!s.b[1724])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(507), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1725] = (s.v[507] == 0.5);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && s.b[1725]) {s.store_sqrt_mul(1195, 1198, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && (!s.b[1725])) {s.store_pow_mul_base_indices(1195, 1198, 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {s.store_mul(1202, 586, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 518, 1203, 1201);}
        s.b[1726] = (s.v[521] == 0.0);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1726]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1727] = (((-s.v[507]) * s.v[574]) == (-1.0));s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1727]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1727])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, 598, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1728] = (s.v[1216] > 0.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1728]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1728])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1729] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1729]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1729])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1730] = (s.v[1216] > 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1730]) {s.copy_ad(1217, 1179);}
        s.b[1731] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) && s.b[1731]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) && (!s.b[1731])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1732] = (s.v[527] == 0.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1732]) {s.store_scalar(1219, 0.0);}
        s.b[1733] = (s.v[507] == 0.5);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && s.b[1733]) {s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1733])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);}
        s.b[1734] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && s.b[1734]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));}
        s.b[1735] = (((-s.v[604]) / s.v[1220]) < 0.0);s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1734])) && s.b[1735]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1734])) && (!s.b[1735])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1736] = (s.v[536] > 1000.0);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1736]) {s.store_scalar(1221, 1.0);}
        s.b[1737] = (s.v[1194] > ((-s.v[438]) * s.v[536]));s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });s.b[1738] = (s.v[539] == 4.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) && s.b[1738]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) && (!s.b[1738])) {s.store_pow_abs_mul_base_indices(1195, 1194, 610, 539);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && (!s.b[1737])) {s.store_add_scaled_product_mixed_iai(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1722])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(472, 667, 1222, 1.0, 668, 1223, 1.0, 669, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1739] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });s.b[1740] = (s.v[483] < s.v[675]);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });s.b[1741] = (((((-0.5) * (s.v[483] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && s.b[1741]) {s.store_primal_exp_scaled_input(1188, 483, (s.v[365] * (-0.5)));}
        s.b[1742] = (((-0.5) * (s.v[483] * s.v[365])) < 0.0);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && s.b[1742]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(483), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(483), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && (!s.b[1740])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 676, A::sub_scaled_inputs(s.ad_value(483), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && (!s.b[1740])) {s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1739]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1743] = (s.v[483] > 0.0);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1743]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && (!s.b[1743])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 483);}
        if ((s.b[1159] && s.b[1176]) && s.b[1739]) {s.store_primal_sub(1191, 677, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 483, 0.5, 1191, 0.5, 483, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 483, 0.5, 680, 0.5, 483, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 483, A::sqrt_square_offset(s.ad_value(483), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1744] = (s.v[667] == 0.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1744]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1744])) {s.store_primal_mul(1196, 557, 1186);}
        s.b[1745] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1745]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {s.store_primal_sub(1198, 563, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1746] = (s.v[505] == 0.5);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && s.b[1746]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && (!s.b[1746])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1747] = (s.v[505] == 0.5);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && s.b[1747]) {s.store_sqrt_mul(1195, 1198, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && (!s.b[1747])) {s.store_pow_mul_base_indices(1195, 1198, 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {s.store_mul(1202, 584, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 516, 1203, 1201);}
        s.b[1748] = (s.v[519] == 0.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1748]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1749] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1749]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1749])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, 596, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1750] = (s.v[1216] > 0.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1750]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1750])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1751] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1751]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1751])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1752] = (s.v[1216] > 0.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1752]) {s.copy_ad(1217, 1179);}
        s.b[1753] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) && s.b[1753]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) && (!s.b[1753])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);}
        s.b[1754] = (s.v[525] == 0.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1754]) {s.store_scalar(1219, 0.0);}
        s.b[1755] = (s.v[505] == 0.5);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && s.b[1755]) {s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1755])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);}
        s.b[1756] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && s.b[1756]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));}
        s.b[1757] = (((-s.v[602]) / s.v[1220]) < 0.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1756])) && s.b[1757]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1756])) && (!s.b[1757])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1758] = (s.v[534] > 1000.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1758]) {s.store_scalar(1221, 1.0);}
        s.b[1759] = (s.v[1194] > ((-s.v[438]) * s.v[534]));s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });s.b[1760] = (s.v[537] == 4.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) && s.b[1760]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) && (!s.b[1760])) {s.store_pow_abs_mul_base_indices(1195, 1194, 608, 537);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && (!s.b[1759])) {s.store_add_scaled_product_mixed_iai(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1744])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1761] = (s.v[668] == 0.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1761]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1761])) {s.store_primal_mul(1196, 558, 1186);}
        s.b[1762] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1762]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {s.store_primal_sub(1198, 564, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1763] = (s.v[506] == 0.5);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && s.b[1763]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && (!s.b[1763])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1764] = (s.v[506] == 0.5);s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && s.b[1764]) {s.store_sqrt_mul(1195, 1198, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && (!s.b[1764])) {s.store_pow_mul_base_indices(1195, 1198, 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {s.store_mul(1202, 585, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 517, 1203, 1201);}
        s.b[1765] = (s.v[520] == 0.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1765]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1766] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1766]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1766])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, 597, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1767] = (s.v[1216] > 0.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1767]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1767])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1768] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1768]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1768])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1769] = (s.v[1216] > 0.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1769]) {s.copy_ad(1217, 1179);}
        s.b[1770] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) && s.b[1770]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) && (!s.b[1770])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);}
        s.b[1771] = (s.v[526] == 0.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1771]) {s.store_scalar(1219, 0.0);}
        s.b[1772] = (s.v[506] == 0.5);s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && s.b[1772]) {s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1772])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);}
        s.b[1773] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && s.b[1773]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));}
        s.b[1774] = (((-s.v[603]) / s.v[1220]) < 0.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1773])) && s.b[1774]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1773])) && (!s.b[1774])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1775] = (s.v[535] > 1000.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1775]) {s.store_scalar(1221, 1.0);}
        s.b[1776] = (s.v[1194] > ((-s.v[438]) * s.v[535]));s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });s.b[1777] = (s.v[538] == 4.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) && s.b[1777]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) && (!s.b[1777])) {s.store_pow_abs_mul_base_indices(1195, 1194, 609, 538);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && (!s.b[1776])) {s.store_add_scaled_product_mixed_iai(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1761])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1778] = (s.v[669] == 0.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1778]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1778])) {s.store_primal_mul(1196, 559, 1186);}
    }
}
