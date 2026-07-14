#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_64(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1638] = (s.v[532] == 0.0);s.store_scalar(1638, if s.b[1638] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1638]) {s.store_scalar(1215, 0.0);}
        s.b[1639] = (s.v[512] == 0.5);s.store_scalar(1639, if s.b[1639] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && s.b[1639]) {s.store_sqrt_mul_sub_lhs(1191, 509, 1189, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1639])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(509), s.ad_value(1189)), 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 579, A::sub(s.ad_value(509), s.ad_value(1189)), 594, 1.0, 1191, 1.0);}
        s.b[1640] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1640, if s.b[1640] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && s.b[1640]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));}
        s.b[1641] = (((-s.v[609]) / s.v[1216]) < 0.0);s.store_scalar(1641, if s.b[1641] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1640])) && s.b[1641]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 609, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1640])) && (!s.b[1641])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 609, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 532, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1642] = (s.v[541] > 1000.0);s.store_scalar(1642, if s.b[1642] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1642]) {s.store_scalar(1217, 1.0);}
        s.b[1643] = (s.v[1190] > ((-s.v[445]) * s.v[541]));s.store_scalar(1643, if s.b[1643] { 1.0 } else { 0.0 });s.b[1644] = (s.v[544] == 4.0);s.store_scalar(1644, if s.b[1644] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) && s.b[1644]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), 1190, 615);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) && (!s.b[1644])) {s.store_pow_abs_mul_base_indices(1191, 1190, 615, 544);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && (!s.b[1643])) {s.store_add_scaled_product_mixed_iai(1217, 612, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1628])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1645] = (s.v[675] == 0.0);s.store_scalar(1645, if s.b[1645] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1645]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1645])) {s.store_primal_mul(1192, 565, 1182);}
        s.b[1646] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1646, if s.b[1646] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1646]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {s.store_primal_sub(1194, 571, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1647] = (s.v[513] == 0.5);s.store_scalar(1647, if s.b[1647] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {s.store_primal_add(1197, 1195, 1196);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_65(
        s: &mut Scratch,
    ) {
        s.b[1648] = (s.v[513] == 0.5);s.store_scalar(1648, if s.b[1648] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1648]) {s.store_sqrt_mul(1191, 1194, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1648])) {s.store_pow_mul_base_indices(1191, 1194, 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {s.store_mul(1198, 592, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 562, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 524, 1199, 1197);}
        s.b[1649] = (s.v[527] == 0.0);s.store_scalar(1649, if s.b[1649] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1649]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {s.store_mul_div_scaled_product_indices(1201, 607, 1198, 577, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 604, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1650] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1650, if s.b[1650] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1650]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1650])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, 604, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1651] = (s.v[1212] > 0.0);s.store_scalar(1651, if s.b[1651] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1651]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1651])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1652] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1652, if s.b[1652] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1652]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1652])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1653] = (s.v[1212] > 0.0);s.store_scalar(1653, if s.b[1653] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1653]) {s.copy_ad(1213, 1175);}
        s.b[1654] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1654, if s.b[1654] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) && s.b[1654]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1654])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {s.store_div_scaled_product_indices(1214, 604, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 527, 1199, 1214, 1208, 1.0);}
        s.b[1655] = (s.v[533] == 0.0);s.store_scalar(1655, if s.b[1655] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1655]) {s.store_scalar(1215, 0.0);}
        s.b[1656] = (s.v[513] == 0.5);s.store_scalar(1656, if s.b[1656] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && s.b[1656]) {s.store_sqrt_mul_sub_lhs(1191, 510, 1189, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1656])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(510), s.ad_value(1189)), 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 580, A::sub(s.ad_value(510), s.ad_value(1189)), 595, 1.0, 1191, 1.0);}
        s.b[1657] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1657, if s.b[1657] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_66(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && s.b[1657]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));}
        s.b[1658] = (((-s.v[610]) / s.v[1216]) < 0.0);s.store_scalar(1658, if s.b[1658] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1657])) && s.b[1658]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 610, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1657])) && (!s.b[1658])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 610, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 533, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1659] = (s.v[542] > 1000.0);s.store_scalar(1659, if s.b[1659] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1659]) {s.store_scalar(1217, 1.0);}
        s.b[1660] = (s.v[1190] > ((-s.v[445]) * s.v[542]));s.store_scalar(1660, if s.b[1660] { 1.0 } else { 0.0 });s.b[1661] = (s.v[545] == 4.0);s.store_scalar(1661, if s.b[1661] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) && s.b[1661]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), 1190, 616);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) && (!s.b[1661])) {s.store_pow_abs_mul_base_indices(1191, 1190, 616, 545);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && (!s.b[1660])) {s.store_add_scaled_product_mixed_iai(1217, 613, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1645])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1662] = (s.v[676] == 0.0);s.store_scalar(1662, if s.b[1662] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1662]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1662])) {s.store_primal_mul(1192, 566, 1182);}
        s.b[1663] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(1663, if s.b[1663] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1663]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {s.store_primal_sub(1194, 572, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1664] = (s.v[514] == 0.5);s.store_scalar(1664, if s.b[1664] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && (!s.b[1664])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1665] = (s.v[514] == 0.5);s.store_scalar(1665, if s.b[1665] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && s.b[1665]) {s.store_sqrt_mul(1191, 1194, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && (!s.b[1665])) {s.store_pow_mul_base_indices(1191, 1194, 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {s.store_mul(1198, 593, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 563, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 525, 1199, 1197);}
        s.b[1666] = (s.v[528] == 0.0);s.store_scalar(1666, if s.b[1666] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1666]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {s.store_mul_div_scaled_product_indices(1201, 608, 1198, 578, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 605, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1667] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(1667, if s.b[1667] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_67(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1667]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1667])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, 605, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1668] = (s.v[1212] > 0.0);s.store_scalar(1668, if s.b[1668] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1668]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1668])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1669] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1669, if s.b[1669] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1669]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1669])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1670] = (s.v[1212] > 0.0);s.store_scalar(1670, if s.b[1670] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1670]) {s.copy_ad(1213, 1175);}
        s.b[1671] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1671, if s.b[1671] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) && s.b[1671]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1671])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {s.store_div_scaled_product_indices(1214, 605, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 528, 1199, 1214, 1208, 1.0);}
        s.b[1672] = (s.v[534] == 0.0);s.store_scalar(1672, if s.b[1672] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1672]) {s.store_scalar(1215, 0.0);}
        s.b[1673] = (s.v[514] == 0.5);s.store_scalar(1673, if s.b[1673] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && s.b[1673]) {s.store_sqrt_mul_sub_lhs(1191, 511, 1189, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1673])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(511), s.ad_value(1189)), 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 581, A::sub(s.ad_value(511), s.ad_value(1189)), 596, 1.0, 1191, 1.0);}
        s.b[1674] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1674, if s.b[1674] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && s.b[1674]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));}
        s.b[1675] = (((-s.v[611]) / s.v[1216]) < 0.0);s.store_scalar(1675, if s.b[1675] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1674])) && s.b[1675]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 611, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1674])) && (!s.b[1675])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 611, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 534, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1676] = (s.v[543] > 1000.0);s.store_scalar(1676, if s.b[1676] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1676]) {s.store_scalar(1217, 1.0);}
        s.b[1677] = (s.v[1190] > ((-s.v[445]) * s.v[543]));s.store_scalar(1677, if s.b[1677] { 1.0 } else { 0.0 });s.b[1678] = (s.v[546] == 4.0);s.store_scalar(1678, if s.b[1678] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_68(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) && s.b[1678]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), 1190, 617);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) && (!s.b[1678])) {s.store_pow_abs_mul_base_indices(1191, 1190, 617, 546);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && (!s.b[1677])) {s.store_add_scaled_product_mixed_iai(1217, 614, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), 620, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1662])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(478, 674, 1218, 1.0, 675, 1219, 1.0, 676, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1679] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1679, if s.b[1679] { 1.0 } else { 0.0 });s.b[1680] = (s.v[489] < s.v[682]);s.store_scalar(1680, if s.b[1680] { 1.0 } else { 0.0 });s.b[1681] = (((((-0.5) * (s.v[489] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1681, if s.b[1681] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && s.b[1681]) {s.store_primal_exp_scaled_input(1184, 489, (s.v[372] * (-0.5)));}
        s.b[1682] = (((-0.5) * (s.v[489] * s.v[372])) < 0.0);s.store_scalar(1682, if s.b[1682] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && (!s.b[1681])) && s.b[1682]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && (!s.b[1681])) && (!s.b[1682])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && (!s.b[1680])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 683, A::sub_scaled_inputs(s.ad_value(489), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1679]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1683] = (s.v[489] > 0.0);s.store_scalar(1683, if s.b[1683] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1683]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && (!s.b[1683])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 489);}
        if ((s.b[1155] && s.b[1172]) && s.b[1679]) {s.store_primal_sub(1187, 684, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 489, 0.5, 1187, 0.5, 489, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 489, 0.5, 687, 0.5, 489, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 489, A::sqrt_square_offset(s.ad_value(489), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1684] = (s.v[674] == 0.0);s.store_scalar(1684, if s.b[1684] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1684]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1684])) {s.store_primal_mul(1192, 564, 1182);}
        s.b[1685] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1685, if s.b[1685] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1685]) {s.store_scalar(1193, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_69(
        s: &mut Scratch,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {s.store_primal_sub(1194, 570, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1686] = (s.v[512] == 0.5);s.store_scalar(1686, if s.b[1686] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && s.b[1686]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && (!s.b[1686])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1687] = (s.v[512] == 0.5);s.store_scalar(1687, if s.b[1687] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && s.b[1687]) {s.store_sqrt_mul(1191, 1194, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && (!s.b[1687])) {s.store_pow_mul_base_indices(1191, 1194, 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {s.store_mul(1198, 591, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 561, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 523, 1199, 1197);}
        s.b[1688] = (s.v[526] == 0.0);s.store_scalar(1688, if s.b[1688] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1688]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {s.store_mul_div_scaled_product_indices(1201, 606, 1198, 576, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 603, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1689] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1689, if s.b[1689] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1689]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1689])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, 603, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1690] = (s.v[1212] > 0.0);s.store_scalar(1690, if s.b[1690] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1690]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1690])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1691] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1691, if s.b[1691] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1691]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1691])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1692] = (s.v[1212] > 0.0);s.store_scalar(1692, if s.b[1692] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1692]) {s.copy_ad(1213, 1175);}
        s.b[1693] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1693, if s.b[1693] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) && s.b[1693]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1693])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {s.store_div_scaled_product_indices(1214, 603, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 526, 1199, 1214, 1208, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_70(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1694] = (s.v[532] == 0.0);s.store_scalar(1694, if s.b[1694] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1694]) {s.store_scalar(1215, 0.0);}
        s.b[1695] = (s.v[512] == 0.5);s.store_scalar(1695, if s.b[1695] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && s.b[1695]) {s.store_sqrt_mul_sub_lhs(1191, 509, 1189, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1695])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(509), s.ad_value(1189)), 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 579, A::sub(s.ad_value(509), s.ad_value(1189)), 594, 1.0, 1191, 1.0);}
        s.b[1696] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1696, if s.b[1696] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && s.b[1696]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));}
        s.b[1697] = (((-s.v[609]) / s.v[1216]) < 0.0);s.store_scalar(1697, if s.b[1697] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1696])) && s.b[1697]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 609, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1696])) && (!s.b[1697])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 609, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 532, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1698] = (s.v[541] > 1000.0);s.store_scalar(1698, if s.b[1698] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1698]) {s.store_scalar(1217, 1.0);}
        s.b[1699] = (s.v[1190] > ((-s.v[445]) * s.v[541]));s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });s.b[1700] = (s.v[544] == 4.0);s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) && s.b[1700]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), 1190, 615);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) && (!s.b[1700])) {s.store_pow_abs_mul_base_indices(1191, 1190, 615, 544);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && (!s.b[1699])) {s.store_add_scaled_product_mixed_iai(1217, 612, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1684])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1701] = (s.v[675] == 0.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1701]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1701])) {s.store_primal_mul(1192, 565, 1182);}
        s.b[1702] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1702]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {s.store_primal_sub(1194, 571, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1703] = (s.v[513] == 0.5);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {s.store_primal_add(1197, 1195, 1196);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_71(
        s: &mut Scratch,
    ) {
        s.b[1704] = (s.v[513] == 0.5);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1704]) {s.store_sqrt_mul(1191, 1194, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1704])) {s.store_pow_mul_base_indices(1191, 1194, 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {s.store_mul(1198, 592, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 562, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 524, 1199, 1197);}
        s.b[1705] = (s.v[527] == 0.0);s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1705]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {s.store_mul_div_scaled_product_indices(1201, 607, 1198, 577, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 604, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1706] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1706]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1706])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, 604, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1707] = (s.v[1212] > 0.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1707]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1707])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1708] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1708]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1708])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1709] = (s.v[1212] > 0.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1709]) {s.copy_ad(1213, 1175);}
        s.b[1710] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) && s.b[1710]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1710])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {s.store_div_scaled_product_indices(1214, 604, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 527, 1199, 1214, 1208, 1.0);}
        s.b[1711] = (s.v[533] == 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1711]) {s.store_scalar(1215, 0.0);}
        s.b[1712] = (s.v[513] == 0.5);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_mul_sub_lhs(1191, 510, 1189, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1712])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(510), s.ad_value(1189)), 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 580, A::sub(s.ad_value(510), s.ad_value(1189)), 595, 1.0, 1191, 1.0);}
        s.b[1713] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_72(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && s.b[1713]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));}
        s.b[1714] = (((-s.v[610]) / s.v[1216]) < 0.0);s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1713])) && s.b[1714]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 610, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1713])) && (!s.b[1714])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 610, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 533, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1715] = (s.v[542] > 1000.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1715]) {s.store_scalar(1217, 1.0);}
        s.b[1716] = (s.v[1190] > ((-s.v[445]) * s.v[542]));s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });s.b[1717] = (s.v[545] == 4.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) && s.b[1717]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), 1190, 616);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) && (!s.b[1717])) {s.store_pow_abs_mul_base_indices(1191, 1190, 616, 545);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && (!s.b[1716])) {s.store_add_scaled_product_mixed_iai(1217, 613, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1701])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1718] = (s.v[676] == 0.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1718]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1718])) {s.store_primal_mul(1192, 566, 1182);}
        s.b[1719] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1719]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {s.store_primal_sub(1194, 572, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1720] = (s.v[514] == 0.5);s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && s.b[1720]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && (!s.b[1720])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1721] = (s.v[514] == 0.5);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && s.b[1721]) {s.store_sqrt_mul(1191, 1194, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && (!s.b[1721])) {s.store_pow_mul_base_indices(1191, 1194, 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {s.store_mul(1198, 593, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 563, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 525, 1199, 1197);}
        s.b[1722] = (s.v[528] == 0.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1722]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {s.store_mul_div_scaled_product_indices(1201, 608, 1198, 578, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 605, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1723] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_73(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1723]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1723])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, 605, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1724] = (s.v[1212] > 0.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1724]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1724])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1725] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1725]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1725])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1726] = (s.v[1212] > 0.0);s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1726]) {s.copy_ad(1213, 1175);}
        s.b[1727] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) && s.b[1727]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1727])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {s.store_div_scaled_product_indices(1214, 605, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 528, 1199, 1214, 1208, 1.0);}
        s.b[1728] = (s.v[534] == 0.0);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1728]) {s.store_scalar(1215, 0.0);}
        s.b[1729] = (s.v[514] == 0.5);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && s.b[1729]) {s.store_sqrt_mul_sub_lhs(1191, 511, 1189, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1729])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(511), s.ad_value(1189)), 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 581, A::sub(s.ad_value(511), s.ad_value(1189)), 596, 1.0, 1191, 1.0);}
        s.b[1730] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && s.b[1730]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));}
        s.b[1731] = (((-s.v[611]) / s.v[1216]) < 0.0);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1730])) && s.b[1731]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 611, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1730])) && (!s.b[1731])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 611, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 534, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1732] = (s.v[543] > 1000.0);s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1732]) {s.store_scalar(1217, 1.0);}
        s.b[1733] = (s.v[1190] > ((-s.v[445]) * s.v[543]));s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });s.b[1734] = (s.v[546] == 4.0);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_74(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) && s.b[1734]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), 1190, 617);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) && (!s.b[1734])) {s.store_pow_abs_mul_base_indices(1191, 1190, 617, 546);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && (!s.b[1733])) {s.store_add_scaled_product_mixed_iai(1217, 614, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), 620, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1718])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(479, 674, 1218, 1.0, 675, 1219, 1.0, 676, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1735] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });s.b[1736] = (s.v[490] < s.v[682]);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });s.b[1737] = (((((-0.5) * (s.v[490] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && s.b[1737]) {s.store_primal_exp_scaled_input(1184, 490, (s.v[372] * (-0.5)));}
        s.b[1738] = (((-0.5) * (s.v[490] * s.v[372])) < 0.0);s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && (!s.b[1737])) && s.b[1738]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && (!s.b[1737])) && (!s.b[1738])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(490), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && (!s.b[1736])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 683, A::sub_scaled_inputs(s.ad_value(490), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1735]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1739] = (s.v[490] > 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1739]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && (!s.b[1739])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 490);}
        if ((s.b[1155] && s.b[1172]) && s.b[1735]) {s.store_primal_sub(1187, 684, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 490, 0.5, 1187, 0.5, 490, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 490, 0.5, 687, 0.5, 490, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 490, A::sqrt_square_offset(s.ad_value(490), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1740] = (s.v[674] == 0.0);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1740]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1740])) {s.store_primal_mul(1192, 564, 1182);}
        s.b[1741] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1741]) {s.store_scalar(1193, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_75(
        s: &mut Scratch,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {s.store_primal_sub(1194, 570, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1742] = (s.v[512] == 0.5);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && s.b[1742]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && (!s.b[1742])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1743] = (s.v[512] == 0.5);s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && s.b[1743]) {s.store_sqrt_mul(1191, 1194, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && (!s.b[1743])) {s.store_pow_mul_base_indices(1191, 1194, 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {s.store_mul(1198, 591, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 561, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 523, 1199, 1197);}
        s.b[1744] = (s.v[526] == 0.0);s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1744]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {s.store_mul_div_scaled_product_indices(1201, 606, 1198, 576, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 603, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1745] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1745]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1745])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, 603, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1746] = (s.v[1212] > 0.0);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1746]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1746])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1747] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1747]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1747])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1748] = (s.v[1212] > 0.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1748]) {s.copy_ad(1213, 1175);}
        s.b[1749] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) && s.b[1749]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1749])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {s.store_div_scaled_product_indices(1214, 603, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 526, 1199, 1214, 1208, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_76(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1750] = (s.v[532] == 0.0);s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1750]) {s.store_scalar(1215, 0.0);}
        s.b[1751] = (s.v[512] == 0.5);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && s.b[1751]) {s.store_sqrt_mul_sub_lhs(1191, 509, 1189, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1751])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(509), s.ad_value(1189)), 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 579, A::sub(s.ad_value(509), s.ad_value(1189)), 594, 1.0, 1191, 1.0);}
        s.b[1752] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && s.b[1752]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));}
        s.b[1753] = (((-s.v[609]) / s.v[1216]) < 0.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1752])) && s.b[1753]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 609, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1752])) && (!s.b[1753])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 609, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 532, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1754] = (s.v[541] > 1000.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1754]) {s.store_scalar(1217, 1.0);}
        s.b[1755] = (s.v[1190] > ((-s.v[445]) * s.v[541]));s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });s.b[1756] = (s.v[544] == 4.0);s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) && s.b[1756]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), 1190, 615);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) && (!s.b[1756])) {s.store_pow_abs_mul_base_indices(1191, 1190, 615, 544);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && (!s.b[1755])) {s.store_add_scaled_product_mixed_iai(1217, 612, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1740])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1757] = (s.v[675] == 0.0);s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1757]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1757])) {s.store_primal_mul(1192, 565, 1182);}
        s.b[1758] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1758]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {s.store_primal_sub(1194, 571, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1759] = (s.v[513] == 0.5);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && s.b[1759]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && (!s.b[1759])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {s.store_primal_add(1197, 1195, 1196);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_77(
        s: &mut Scratch,
    ) {
        s.b[1760] = (s.v[513] == 0.5);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && s.b[1760]) {s.store_sqrt_mul(1191, 1194, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && (!s.b[1760])) {s.store_pow_mul_base_indices(1191, 1194, 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {s.store_mul(1198, 592, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 562, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 524, 1199, 1197);}
        s.b[1761] = (s.v[527] == 0.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1761]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {s.store_mul_div_scaled_product_indices(1201, 607, 1198, 577, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 604, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1762] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1762]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1762])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, 604, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1763] = (s.v[1212] > 0.0);s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1763]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1763])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1764] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1764, if s.b[1764] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1764]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1764])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1765] = (s.v[1212] > 0.0);s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1765]) {s.copy_ad(1213, 1175);}
        s.b[1766] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) && s.b[1766]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1766])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {s.store_div_scaled_product_indices(1214, 604, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 527, 1199, 1214, 1208, 1.0);}
        s.b[1767] = (s.v[533] == 0.0);s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1767]) {s.store_scalar(1215, 0.0);}
        s.b[1768] = (s.v[513] == 0.5);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && s.b[1768]) {s.store_sqrt_mul_sub_lhs(1191, 510, 1189, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1768])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(510), s.ad_value(1189)), 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 580, A::sub(s.ad_value(510), s.ad_value(1189)), 595, 1.0, 1191, 1.0);}
        s.b[1769] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_78(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && s.b[1769]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));}
        s.b[1770] = (((-s.v[610]) / s.v[1216]) < 0.0);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1769])) && s.b[1770]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 610, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1769])) && (!s.b[1770])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 610, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 533, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1771] = (s.v[542] > 1000.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1771]) {s.store_scalar(1217, 1.0);}
        s.b[1772] = (s.v[1190] > ((-s.v[445]) * s.v[542]));s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });s.b[1773] = (s.v[545] == 4.0);s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) && s.b[1773]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), 1190, 616);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) && (!s.b[1773])) {s.store_pow_abs_mul_base_indices(1191, 1190, 616, 545);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && (!s.b[1772])) {s.store_add_scaled_product_mixed_iai(1217, 613, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1757])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1774] = (s.v[676] == 0.0);s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1774]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1774])) {s.store_primal_mul(1192, 566, 1182);}
        s.b[1775] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1775]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {s.store_primal_sub(1194, 572, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1776] = (s.v[514] == 0.5);s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && s.b[1776]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && (!s.b[1776])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1777] = (s.v[514] == 0.5);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && s.b[1777]) {s.store_sqrt_mul(1191, 1194, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && (!s.b[1777])) {s.store_pow_mul_base_indices(1191, 1194, 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {s.store_mul(1198, 593, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 563, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 525, 1199, 1197);}
        s.b[1778] = (s.v[528] == 0.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1778]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {s.store_mul_div_scaled_product_indices(1201, 608, 1198, 578, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 605, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1779] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_79(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1779]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1779])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, 605, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1780] = (s.v[1212] > 0.0);s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1780]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1780])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1781] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1781]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1781])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1782] = (s.v[1212] > 0.0);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1782]) {s.copy_ad(1213, 1175);}
        s.b[1783] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) && s.b[1783]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1783])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {s.store_div_scaled_product_indices(1214, 605, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 528, 1199, 1214, 1208, 1.0);}
        s.b[1784] = (s.v[534] == 0.0);s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1784]) {s.store_scalar(1215, 0.0);}
        s.b[1785] = (s.v[514] == 0.5);s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && s.b[1785]) {s.store_sqrt_mul_sub_lhs(1191, 511, 1189, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1785])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(511), s.ad_value(1189)), 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 581, A::sub(s.ad_value(511), s.ad_value(1189)), 596, 1.0, 1191, 1.0);}
        s.b[1786] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && s.b[1786]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));}
        s.b[1787] = (((-s.v[611]) / s.v[1216]) < 0.0);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1786])) && s.b[1787]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 611, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1786])) && (!s.b[1787])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 611, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 534, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1788] = (s.v[543] > 1000.0);s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1788]) {s.store_scalar(1217, 1.0);}
        s.b[1789] = (s.v[1190] > ((-s.v[445]) * s.v[543]));s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });s.b[1790] = (s.v[546] == 4.0);s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
    }
}
