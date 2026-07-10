#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && s.b[1785]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0));}
        s.b[1786] = (((-s.v[609]) / s.v[1232]) < 0.0);s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && (!s.b[1785])) && s.b[1786]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 609, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && (!s.b[1785])) && (!s.b[1786])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 609, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 532, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1787] = (s.v[541] > 1000.0);s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && s.b[1787]) {s.store_scalar(1233, 1.0);}
        s.b[1788] = (s.v[1206] > ((-s.v[444]) * s.v[541]));s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });s.b[1789] = (s.v[544] == 4.0);s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) && s.b[1789]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), 1206, 615);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) && (!s.b[1789])) {s.store_pow_abs_mul_base_indices(1207, 1206, 615, 544);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && (!s.b[1788])) {s.store_add_scaled_product_mixed_iai(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1773])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1790] = (s.v[675] == 0.0);s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1790]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1790])) {s.store_primal_mul(1208, 565, 1198);}
        s.b[1791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1791]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {s.store_primal_sub(1210, 571, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1792] = (s.v[513] == 0.5);s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1792]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1792])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1793] = (s.v[513] == 0.5);s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1793]) {s.store_sqrt_mul(1207, 1210, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1793])) {s.store_pow_mul_base_indices(1207, 1210, 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {s.store_mul(1214, 592, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 562, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 524, 1215, 1213);}
        s.b[1794] = (s.v[527] == 0.0);s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1794]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {s.store_mul_div_scaled_product_indices(1217, 607, 1214, 577, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1795] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1795]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1795])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, 604, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1796] = (s.v[1228] > 0.0);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1796]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1796])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1797] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1797]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1797])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1798] = (s.v[1228] > 0.0);s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1798]) {s.copy_ad(1229, 1191);}
        s.b[1799] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) && s.b[1799]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) && (!s.b[1799])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 527, 1215, 1230, 1224, 1.0);}
        s.b[1800] = (s.v[533] == 0.0);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1800]) {s.store_scalar(1231, 0.0);}
        s.b[1801] = (s.v[513] == 0.5);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && s.b[1801]) {s.store_sqrt_mul_sub_lhs(1207, 510, 1205, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1801])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(510), s.ad_value(1205)), 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), 595, 1.0, 1207, 1.0);}
        s.b[1802] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && s.b[1802]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0));}
        s.b[1803] = (((-s.v[610]) / s.v[1232]) < 0.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1802])) && s.b[1803]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 610, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1802])) && (!s.b[1803])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 610, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 533, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1804] = (s.v[542] > 1000.0);s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1804]) {s.store_scalar(1233, 1.0);}
        s.b[1805] = (s.v[1206] > ((-s.v[444]) * s.v[542]));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });s.b[1806] = (s.v[545] == 4.0);s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) && s.b[1806]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), 1206, 616);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) && (!s.b[1806])) {s.store_pow_abs_mul_base_indices(1207, 1206, 616, 545);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && (!s.b[1805])) {s.store_add_scaled_product_mixed_iai(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1790])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(479, 673, 1234, 1.0, 674, 1235, 1.0, 675, 1236, 1.0);s.store_primal_add_scaled_products3_indices(694, 673, 563, 1.0, 674, 564, 1.0, 675, 565, 1.0);s.store_add_scaled_offset_product_rhs_mixed_iia(483, 478, 1.0, 694, A::exp_scaled_input(s.ad_value(488), (s.v[371] * s.v[695])), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 694, A::exp_scaled_input(s.ad_value(489), (s.v[371] * s.v[695])), (-1.0), (-1.0));}
        s.b[1807] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });s.b[1808] = ((s.v[478] > 0.0) && (s.v[479] > 0.0));s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });s.b[1809] = ((((((s.v[483] / s.v[478]) > 0.001) || ((s.v[484] / s.v[479]) > 0.001)) && (s.v[483] > 0.0)) && (s.v[484] > 0.0)) && (s.v[484] > s.v[483]));s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {s.store_div(490, 483, 484);s.store_div_scaled_inputs(697, A::ln(s.ad_value(490)), s.v[370], A::sub(s.ad_value(488), s.ad_value(489)), 1.0);s.store_div_scaled_value_offset_denominator(696, s.ad_value(483), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[371], s.ad_value(697))), (-1.0), 1.0);}
        if ((s.b[1171] && s.b[1188]) && s.b[1807]) {s.store_add_scaled_offset_product_rhs_mixed_aia(480, A::add_scaled_offset_product_rhs(s.ad_value(475), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(485), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(485), s.v[371], s.ad_value(697))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(486), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[371], s.ad_value(697))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(487), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[371], s.ad_value(697))), (-1.0), (-1.0));}
        s.b[1810] = (((s.v[475] < 0.0) && (s.v[476] < 0.0)) && (s.v[477] < 0.0));s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });s.b[1811] = (((((((s.v[480] / s.v[475]) > 0.001) || ((s.v[481] / s.v[476]) > 0.001)) || ((s.v[482] / s.v[477]) > 0.001)) && (s.v[480] < 0.0)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0));s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) {s.store_div(490, 480, 481);s.store_div_scaled_inputs(491, A::ln(s.ad_value(490)), (-s.v[370]), A::sub(s.ad_value(485), s.ad_value(486)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(493, 486, 486, 1.0, 485, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_83(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) {s.store_scaled_mul_ad(494, A::offset(s.ad_value(490), (-1.0)), A::offset(A::pow(s.ad_value(490), s.ad_value(493)), (-1.0)), s.v[370]);s.store_primal_div_add_scaled_inputs_rhs_indices(493, 485, 485, 1.0, 486, -1.0);s.store_sub_mixed_ai(495, A::add_scaled_products(A::pow(s.ad_value(490), s.ad_value(493)), A::sub(s.ad_value(486), s.ad_value(485)), 1.0, s.ad_value(490), s.ad_value(485), 1.0), 486);s.store_div(492, 494, 495);s.store_add(699, 491, 492);}
        s.b[1812] = (((((s.v[487] * s.v[371]) * s.v[699])) as f64).abs() < 1e-6);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && s.b[1812]) {
        (1.0,)
    } else {
        (s.v[693],)
    }
};
        s.store_scalar(693, t0);
        if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && s.b[1812]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(698, 482, A::div_from_scalar(1.0, s.ad_value(487)), 1.0, 699, (0.5 * s.v[371]));s.store_div_scaled_product_indices(699, 482, 699, ((-0.5) * s.v[371]), 487, 1.0);}
        let (t1,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && (!s.b[1812])) {
        (0.0,)
    } else {
        (s.v[693],)
    }
};
        s.store_scalar(693, t1);
        if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && (!s.b[1812])) {s.store_div_scaled_value_offset_denominator(698, s.ad_value(482), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(487), (-s.v[371]), s.ad_value(699))), (-1.0), 1.0);}
        let (t8,) = {
    if (s.b[1171] && s.b[1188]) {
        let t2: f64 = (s.v[673] * s.v[581]);let t3: f64 = (s.v[674] * s.v[582]);let t4: f64 = (t2 + t3);let t5: f64 = (s.v[675] * s.v[583]);let t6: f64 = (t4 + t5);let t7: f64 = (s.v[553] * t6);
        (t7,)
    } else {
        (s.v[501],)
    }
};
        s.store_scalar(501, t8);s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1813]) {
        (0.0,)
    } else {
        (s.v[678],)
    }
};
        s.store_scalar(678, t9);s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1814]) {
        (0.0,)
    } else {
        (s.v[679],)
    }
};
        s.store_scalar(679, ta);s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[680],)
    }
};
        s.store_scalar(680, tb);s.b[1816] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1816]) {s.store_primal_ln_ad(687, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(694), 1e-21, 1.0));s.store_ln_ad(689, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(696), 1e-21, 1.0));s.store_ln_ad(691, A::div_scalar_offset_denominator((0.5 * p.p822), A::abs(s.ad_value(698)), 1e-21, 1.0));}
        if (s.b[1171] && s.b[1188]) {s.store_primal_min_with_scalar(687, 687, 230.25850929940458);s.store_primal_exp(688, 687);s.store_min_with_scalar(689, 689, 230.25850929940458);s.store_exp(690, 689);s.store_min_with_scalar(691, 691, 230.25850929940458);s.store_exp(692, 691);}
        s.store_scalar(2027, 0.0);s.store_scalar(2028, 0.0);s.store_scalar(2029, 0.0);s.store_scalar(1937, 1.0);s.store_scalar(1936, 0.0);s.b[2102] = (s.v[0] == 1.0);s.store_scalar(2102, if s.b[2102] { 1.0 } else { 0.0 });
        if s.b[2102] {s.store_voltage(825, ctx, nodes, Some(5), Some(6));s.store_voltage(826, ctx, nodes, Some(7), Some(6));s.store_voltage(827, ctx, nodes, Some(6), Some(8));s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);}
        if (!s.b[2102]) {s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);s.store_voltage(832, ctx, nodes, Some(6), Some(10));s.store_voltage(833, ctx, nodes, Some(7), Some(11));}
        s.store_add(829, 825, 827);s.copy_ad(834, 825);s.copy_ad(835, 827);s.store_add(836, 826, 827);s.store_sub(837, 825, 826);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scale(1817, 834, (-s.v[355]));s.store_scale(1818, 837, (-s.v[355]));s.store_scaled_sub(1819, 829, 700, (-s.v[355]));s.store_scalar(831, 1.0);s.b[2103] = (s.v[826] < 0.0);s.store_scalar(2103, if s.b[2103] { 1.0 } else { 0.0 });
        if s.b[2103] {s.store_scalar(831, (-1.0));s.store_sub(825, 825, 826);s.store_add(827, 827, 826);s.store_neg(826, 826);}
        s.store_add(828, 826, 827);s.store_div_scaled_product_offset_denominator_mixed_iia(830, 826, 826, 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739))), (-0.5), 737, 1.0);s.copy_ad(1820, 2107);s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2107)), s.ad_value(738))), (-(-0.5)), 741, 1.0);s.copy_ad(1821, 2030);s.store_scalar(2031, 0.0);s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
        if s.b[2263] {s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));s.store_sub_mixed_ai(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));s.store_sub(2031, 1821, 2030);}
        s.copy_ad(2104, 728);s.copy_ad(2105, 738);s.copy_ad(2106, 729);s.copy_ad(2108, 2030);s.copy_ad(2112, 2031);s.copy_ad(2109, 720);s.copy_ad(2110, 777);s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));s.store_scalar(2125, 1.0);s.b[2264] = (s.v[190] > 0.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
        if s.b[2264] {s.store_primal_scale(2116, 2104, s.v[361]);s.store_scale(2117, 2113, s.v[361]);s.store_scale(2118, 2111, s.v[361]);s.store_offset_div_scaled_inputs_sqrt_rhs(2028, 2106, 0.5, 2116, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));s.store_primal_offset_scaled(2120, 2116, 0.5, 2.0);s.store_add(2121, 2116, 2117);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2264] {s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2119, 0.5, 2122, 0.5, 2119, 2122, 20.0, 0.5);s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2123, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2123, 0.5, 2120, 0.5, 2123, 2120, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0, 1.0);}
        s.b[2265] = (s.v[2029] > (-230.25850929940458));s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
        if (s.b[2264] && s.b[2265]) {s.store_exp(2125, 2029);}
        if (s.b[2264] && (!s.b[2265])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_offset_mul(2126, 701, 2125, 1.0);s.store_scale(2127, 2126, s.v[715]);s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));s.store_mul_scale_offset_indices(2129, 2127, 2128, 1.0, 1.0);s.store_div_from_scalar(2130, 1.0, 2129);s.store_mul_mixed_ia(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));s.store_square(2115, 2114);s.store_div_from_scalar(2131, 1.0, 2115);s.store_mul(2132, 2108, 2130);s.store_mul(2133, 2111, 2130);s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2135, 196, 2134, A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));s.store_mul(2136, 2104, 2130);s.store_sqrt_square_add(2028, 2107, 2105);s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2137, 2130, 2135, 0.5, 2028, 0.5, 2029, ((-1.0) * (0.5)), 0.0);s.store_add(2138, 2136, 2132);s.store_sub(2139, 2138, 2137);s.b[2266] = (p.p45 > 0.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
        if (s.b[2266] && s.b[2267]) {s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);}
        s.b[2268] = (s.v[2139] < 460.51701859880916);s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {s.store_exp_neg_input(2154, 2139);}
        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2266] && (!s.b[2267])) {s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
    ) {
        if (s.b[2266] && (!s.b[2267])) {s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);}
        if (!s.b[2266]) {s.store_offset_div_scaled_inputs_sqrt_rhs(2140, 2114, 0.5, 2139, 1.0, 1.0);}
        s.store_add_scaled_value_products_mixed_iiaia(2141, 2139, 1.0, 2114, A::sqrt(s.ad_value(2139)), 1.0, 2140, A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));s.store_scalar(2147, 0.0);s.store_scalar(2149, 1.0);s.b[2269] = (s.v[2142] > (-30.0));s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
        if s.b[2269] {s.store_offset_mul(2143, 2140, 2142, (-1.0));s.store_scaled_add_mixed_ia(2027, 2143, A::sqrt_square_offset(s.ad_value(2143), 10.0), 0.5);s.store_sub_mixed_ia(2144, 2142, A::ln(s.ad_value(2027)));s.store_scaled_add_mixed_ia(2145, 2144, A::sqrt_square_offset(s.ad_value(2144), 2.0), 0.5);}
        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2270]) {s.store_exp_sub(2027, 2142, 2145);}
        if (s.b[2269] && (!s.b[2270])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if s.b[2269] {s.store_div(2146, 2027, 2140);s.store_sub_mixed_ai(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);}
        s.b[2271] = (s.v[2146] > 1e-6);s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2271]) {s.store_mul_scale_offset_mixed_ia(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0, 1.0);}
        if (s.b[2269] && (!s.b[2271])) {s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);}
        if s.b[2269] {s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));}
        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);let tc: f64 = (1e-5 * s.v[2150]);s.store_scalar(2151, tc);s.store_div_from_scalar(2152, 1.0, 2150);s.store_scalar(2259, 0.0);s.store_scalar(2153, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
    ) {
        s.b[2272] = (s.v[2139] < 460.51701859880916);s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });
        if s.b[2272] {s.store_exp_neg_input(2154, 2139);}
        if (!s.b[2272]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });
        if s.b[2273] {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2153, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        s.b[2274] = (s.v[2133] < (-s.v[2151]));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((!s.b[2273]) && s.b[2274]) {s.store_neg(2241, 2133);s.store_scaled_mul(2242, 2241, 2152, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2238, 2241, 2243);s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);s.store_sub_ln_mul_lhs(2246, 2244, 2131, 2243);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);s.store_add_mixed_ia(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));}
        s.b[2275] = (s.v[2247] < 230.25850929940458);s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_exp(2248, 2247);}
        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((!s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(2249, 1.0, 2248);s.store_div_from_scalar_offset_square(2238, 1.0, 2247, 2.0);s.store_mul_square_lhs(2250, 2247, 2238);s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2241, 2247);s.store_mul(2239, 2154, 2249);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
    ) {
        if ((!s.b[2273]) && s.b[2274]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2256, 2255, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2257, 2133, 2152, A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));}
        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {s.store_exp_neg_input(2238, 2257);}
        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar(2258, 1.0, 2238);s.store_add_scaled_inputs_product_mixed_iiia(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));s.store_offset(2260, 2139, 3.0);s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2139, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
    ) {
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2277] = (s.v[2262] < 230.25850929940458);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2154, 2248);}
        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {s.store_exp_sub(2248, 2262, 2139);s.store_div(2249, 2154, 2248);}
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        s.store_scalar(2156, 0.0);s.store_scalar(2157, 0.0);s.store_scalar(2158, 0.0);s.store_scalar(2159, 0.0);s.store_scalar(2160, 0.0);s.store_scalar(2161, 0.0);s.store_scalar(2162, 0.0);s.store_scalar(2163, 1.0);s.store_scalar(2164, 1.0);s.store_sub(2165, 2133, 2153);s.store_scalar(2166, 0.0);s.store_mul(2167, 2129, 2165);s.store_scalar(2168, 1.0);s.store_scalar(2169, 1.0);s.store_scalar(2173, 1.0);s.store_scalar(2174, 1.0);s.store_scalar(2176, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
    ) {
        s.b[2279] = (s.v[2133] > 0.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        if s.b[2279] {s.store_div_from_scalar_offset_square(2027, 1.0, 2153, 2.0);s.store_mul_square_lhs(2155, 2153, 2027);s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);s.store_mul_ad_product_lhs_mixed_ai(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), 2027, 2027);s.store_scalar(2158, 0.0);}
        s.b[2280] = (s.v[2153] < 230.25850929940458);s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2280]) {s.store_exp(2158, 2153);s.store_div_from_scalar(2159, 1.0, 2158);s.store_mul(2158, 2154, 2158);}
        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });
        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {s.store_exp_sub(2158, 2153, 2139);s.store_div(2159, 2154, 2158);}
        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2279] {s.store_add_scaled_product_mixed_iia(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));}
        s.b[2282] = (s.v[2153] < 1e-5);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2282]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2163, 2114, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, 2027, 1.0, 1.0);}
        if (s.b[2279] && (!s.b[2282])) {s.store_add_offset_lhs(2161, 2153, (-1.0), 2159);s.store_sqrt(2162, 2161);s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);}
        if s.b[2279] {s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);}
        s.b[2283] = (s.v[2160] > 1e-100);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2283]) {s.store_mul_sqrt_mixed_ia(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);s.store_mul3_lhs(2167, 2162, 2114, 2129);}
        s.b[2284] = (s.v[217] < 0.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {s.store_offset_mul(2168, 217, 2113, 1.0);}
        s.b[2285] = (s.v[218] < 0.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2166, 1.0);}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2166, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul_product3_indices(2170, 2166, 757, 2168, 2169, 1.0);s.store_mul_add_scaled_product_rhs_indices(2171, 774, 2167, 1.0, 775, 2166, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
    ) {
        if (s.b[2279] && s.b[2283]) {s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2173, 2164, A::offset(s.ad_value(2172), 1.0), 2170);}
        s.b[2286] = (s.v[221] < 0.0);s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {s.store_offset_mul(2174, 221, 2113, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul(2029, 2166, 2174);s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2287] = (s.v[222] < 0.0);s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {s.store_offset_mul(2176, 222, 2175, 1.0);}
        s.copy_ad(1822, 2111);s.copy_ad(1823, 2113);s.copy_ad(1824, 2129);s.copy_ad(1825, 2130);s.copy_ad(1826, 2114);s.copy_ad(1827, 2115);s.copy_ad(1828, 2131);s.copy_ad(1829, 2133);s.copy_ad(1830, 2138);s.copy_ad(1831, 2139);s.copy_ad(1832, 2150);s.store_scalar(1833, s.v[2151]);s.copy_ad(1834, 2152);s.copy_ad(1835, 2259);s.copy_ad(1836, 2154);s.copy_ad(1837, 2153);s.copy_ad(1838, 2156);s.copy_ad(1839, 2157);s.copy_ad(1840, 2158);s.copy_ad(1841, 2159);s.copy_ad(1842, 2161);s.copy_ad(1843, 2160);s.copy_ad(1844, 2162);s.copy_ad(1845, 2163);s.copy_ad(1846, 2164);s.copy_ad(1847, 2165);s.copy_ad(1848, 2166);s.copy_ad(1849, 2167);s.copy_ad(1850, 2168);s.copy_ad(1851, 2169);s.copy_ad(1852, 2173);s.copy_ad(1853, 2174);s.copy_ad(1854, 2176);s.store_scalar(2178, 0.0);s.store_scale(2177, 2129, 4.60517018598809);s.copy_ad(2194, 2177);s.copy_ad(2195, 826);s.store_mul(2196, 826, 2130);s.copy_ad(2200, 2153);s.store_scalar(2201, 0.0);s.store_scalar(2204, 0.0);s.copy_ad(2206, 2159);s.copy_ad(2207, 2161);s.copy_ad(2209, 2160);s.copy_ad(2210, 2167);s.copy_ad(2211, 2153);s.copy_ad(2212, 2159);s.copy_ad(2214, 2160);s.copy_ad(2215, 2161);s.store_sub(2216, 2133, 2153);s.store_scalar(2217, 1.0);s.store_scalar(2219, 1.0);s.store_scalar(2218, 0.0);s.copy_ad(2228, 2166);s.store_mul(2232, 2216, 2129);s.store_scalar(2229, 0.0);s.copy_ad(2230, 2167);s.store_scalar(2235, 0.0);s.store_scalar(2234, 1.0);s.copy_ad(2237, 2109);s.copy_ad(2236, 2232);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
    ) {
        s.b[2288] = (s.v[2133] > 0.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });s.b[2289] = (s.v[2160] > 1e-100);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2289]) {s.store_mul(2237, 2109, 2176);s.store_div(2178, 2237, 2173);s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);s.store_div_scaled_product_by_product_indices(2027, 2115, 2158, 1.0, 2179, 2179, 1.0);}
        s.b[2290] = (s.v[2027] > 0.0001);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {s.store_sub_from_scalar(2028, 1.0, 2027);}
        s.b[2291] = (s.v[2028] < 1e-10);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {s.store_scalar(2029, 1.0);}
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {s.store_scale(2029, 2027, 0.5);}
        if (s.b[2288] && s.b[2289]) {s.store_mul(2180, 2029, 2179);}
        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_scaled_mul(2181, 2129, 2180, 0.475);s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));s.store_scaled_add_mixed_ia(2182, 2027, A::sqrt_square_offset(s.ad_value(2027), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2183, 2166, (-1.0), 2129, 2165, 1.0, A::offset(s.ad_value(2163), (-1.0)), 2181, 1.0);s.store_offset_div_scaled_product_indices(2184, 2115, 2129, 0.5, 2183, 1.0, 1.0);s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));s.store_mul_mixed_ai(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);s.store_div(2027, 2182, 2183);s.store_mul_pow_mixed_iaa(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));s.store_mul_div_scaled_product_mixed_iiai(2029, 2186, 707, A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2183, 1.0);s.store_mul_product3_indices(2187, 2182, 757, 2168, 2169, 1.0);s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);}
        s.b[2293] = (s.v[2027] < 230.25850929940458);s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);}
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {s.copy_ad(2028, 2027);}
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);s.store_mul_scale_offset_mixed_ia(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt_square_offset(s.ad_value(2188), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {s.copy_ad(2189, 2180);}
        if (s.b[2288] && s.b[2289]) {s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);}
        s.b[2294] = (s.v[0] == (-1.0));s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {s.store_div_mixed_ia(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));}
        if (s.b[2288] && s.b[2289]) {s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
    ) {
        if (s.b[2288] && s.b[2289]) {s.store_mul(2027, 2191, 2190);s.store_mul_ad_product_rhs_mixed_ia(2192, 2189, 2191, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));s.store_scale(2193, 2192, 0.99);s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);}
        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_mixed_iia(2194, 2129, 2193, A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if (s.b[2288] && (!s.b[2289])) {s.copy_ad(2194, 2177);}
        if s.b[2288] {s.store_offset(2027, 2110, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);s.store_mul(2196, 2195, 2130);s.store_add(2197, 2139, 2196);}
        s.b[2295] = (s.v[2196] < 460.51701859880916);s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2295]) {s.store_exp_neg_input(2198, 2196);}
        if (s.b[2288] && (!s.b[2295])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2288] {s.store_mul(2199, 2154, 2198);}
        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2296]) {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2200, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        if (s.b[2288] && (!s.b[2296])) {s.store_offset(2260, 2197, 3.0);s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
        if (s.b[2288] && (!s.b[2296])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2197, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2297] = (s.v[2262] < 230.25850929940458);s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2199, 2248);}
        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {s.store_exp_sub(2248, 2262, 2197);s.store_div(2249, 2199, 2248);}
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2288] && (!s.b[2296])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if s.b[2288] {s.store_sub(2201, 2200, 2153);}
        s.b[2299] = (s.v[2201] < 1e-10);s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2299]) {s.store_add_scaled_inputs_product_mixed_iiia(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2115, A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));s.store_scaled_div_mixed_ia(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2200, 2153, 2201);}
        if s.b[2288] {s.store_mul(2204, 2201, 2129);s.store_div_scaled_product_offset_denominator_mixed_iia(2205, 2200, 2200, 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);}
        s.b[2300] = (s.v[2200] < 230.25850929940458);s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2300]) {s.store_exp_neg_input(2206, 2200);}
        s.b[2301] = (s.v[2200] < 1e-5);s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));}
        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);s.store_sqrt(2208, 2207);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, 2200, (-1.0), 2205, -1.0, (-1.0));}
        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {s.store_exp_sub(2027, 2200, 2197);s.store_div(2206, 2199, 2027);s.store_add_scaled_product_mixed_iia(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));}
        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
    }
}
