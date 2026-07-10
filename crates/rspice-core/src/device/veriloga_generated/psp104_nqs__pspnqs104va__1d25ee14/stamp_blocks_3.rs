#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1489] = (s.v[1228] > 0.0);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1489]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1489])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1490] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1490]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1490])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1491] = (s.v[1228] > 0.0);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1491]) {s.copy_ad(1229, 1191);}
        s.b[1492] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) && s.b[1492]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) && (!s.b[1492])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);}
        s.b[1493] = (p.p852 == 0.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1493]) {s.store_scalar(1231, 0.0);}
        s.b[1494] = (p.p832 == 0.5);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && s.b[1494]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1494])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1495] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && s.b[1495]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1496] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1495])) && s.b[1496]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1495])) && (!s.b[1496])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852, 0.0);}
        s.b[1497] = (p.p861 > 1000.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1497]) {s.store_scalar(1233, 1.0);}
        s.b[1498] = (s.v[1206] > ((-s.v[444]) * p.p861));s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });s.b[1499] = (p.p864 == 4.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) && s.b[1499]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) && (!s.b[1499])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && (!s.b[1498])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1483])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1500] = (s.v[648] == 0.0);s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1500]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1500])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1501] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1501]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1502] = (p.p833 == 0.5);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && s.b[1502]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && (!s.b[1502])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1503] = (p.p833 == 0.5);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && s.b[1503]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && (!s.b[1503])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p.p842);}
        s.b[1504] = (p.p847 == 0.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1504]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1505] = (((-p.p833) * s.v[413]) == (-1.0));s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1505]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1505])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1506] = (s.v[1228] > 0.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1506]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1506])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1507] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1507]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1507])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1508] = (s.v[1228] > 0.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1508]) {s.copy_ad(1229, 1191);}
        s.b[1509] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) && s.b[1509]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) && (!s.b[1509])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);}
        s.b[1510] = (p.p853 == 0.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1510]) {s.store_scalar(1231, 0.0);}
        s.b[1511] = (p.p833 == 0.5);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && s.b[1511]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1511])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1512] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && s.b[1512]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1513] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1512])) && s.b[1513]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1512])) && (!s.b[1513])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853, 0.0);}
        s.b[1514] = (p.p862 > 1000.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1514]) {s.store_scalar(1233, 1.0);}
        s.b[1515] = (s.v[1206] > ((-s.v[444]) * p.p862));s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });s.b[1516] = (p.p865 == 4.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) && s.b[1516]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) && (!s.b[1516])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && (!s.b[1515])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1500])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(479, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_primal_add_scaled_inputs3_indices(667, 646, s.v[387], 647, s.v[388], 648, s.v[389]);s.store_add_scaled_offset_product_rhs_mixed_iia(483, 478, 1.0, 667, A::exp_scaled_input(s.ad_value(488), (s.v[371] * s.v[668])), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 667, A::exp_scaled_input(s.ad_value(489), (s.v[371] * s.v[668])), (-1.0), (-1.0));}
        s.b[1517] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });s.b[1518] = ((s.v[478] > 0.0) && (s.v[479] > 0.0));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });s.b[1519] = ((((((s.v[483] / s.v[478]) > 0.001) || ((s.v[484] / s.v[479]) > 0.001)) && (s.v[483] > 0.0)) && (s.v[484] > 0.0)) && (s.v[484] > s.v[483]));s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1518]) && s.b[1519]) {s.store_div(490, 483, 484);s.store_div_scaled_inputs(670, A::ln(s.ad_value(490)), s.v[370], A::sub(s.ad_value(488), s.ad_value(489)), 1.0);s.store_div_scaled_value_offset_denominator(669, s.ad_value(483), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[371], s.ad_value(670))), (-1.0), 1.0);}
        if ((s.b[1171] && s.b[1188]) && s.b[1517]) {s.store_add_scaled_offset_product_rhs_mixed_aia(480, A::add_scaled_offset_product_rhs(s.ad_value(475), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(485), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(485), s.v[371], s.ad_value(670))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(486), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[371], s.ad_value(670))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(487), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[371], s.ad_value(670))), (-1.0), (-1.0));}
        s.b[1520] = (((s.v[475] < 0.0) && (s.v[476] < 0.0)) && (s.v[477] < 0.0));s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });s.b[1521] = (((((((s.v[480] / s.v[475]) > 0.001) || ((s.v[481] / s.v[476]) > 0.001)) || ((s.v[482] / s.v[477]) > 0.001)) && (s.v[480] < 0.0)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) {s.store_div(490, 480, 481);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) {s.store_div_scaled_inputs(491, A::ln(s.ad_value(490)), (-s.v[370]), A::sub(s.ad_value(485), s.ad_value(486)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(493, 486, 486, 1.0, 485, -1.0);s.store_scaled_mul_ad(494, A::offset(s.ad_value(490), (-1.0)), A::offset(A::pow(s.ad_value(490), s.ad_value(493)), (-1.0)), s.v[370]);s.store_primal_div_add_scaled_inputs_rhs_indices(493, 485, 485, 1.0, 486, -1.0);s.store_sub_mixed_ai(495, A::add_scaled_products(A::pow(s.ad_value(490), s.ad_value(493)), A::sub(s.ad_value(486), s.ad_value(485)), 1.0, s.ad_value(490), s.ad_value(485), 1.0), 486);s.store_div(492, 494, 495);s.store_add(672, 491, 492);}
        s.b[1522] = (((((s.v[487] * s.v[371]) * s.v[672])) as f64).abs() < 1e-6);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && s.b[1522]) {
        (1.0,)
    } else {
        (s.v[666],)
    }
};
        s.store_scalar(666, t0);
        if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && s.b[1522]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(671, 482, A::div_from_scalar(1.0, s.ad_value(487)), 1.0, 672, (0.5 * s.v[371]));s.store_div_scaled_product_indices(672, 482, 672, ((-0.5) * s.v[371]), 487, 1.0);}
        let (t1,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && (!s.b[1522])) {
        (0.0,)
    } else {
        (s.v[666],)
    }
};
        s.store_scalar(666, t1);
        if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && (!s.b[1522])) {s.store_div_scaled_value_offset_denominator(671, s.ad_value(482), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(487), (-s.v[371]), s.ad_value(672))), (-1.0), 1.0);}
        let (t8,) = {
    if (s.b[1171] && s.b[1188]) {
        let t2: f64 = (s.v[646] * s.v[414]);let t3: f64 = (s.v[647] * s.v[415]);let t4: f64 = (t2 + t3);let t5: f64 = (s.v[648] * s.v[416]);let t6: f64 = (t4 + t5);let t7: f64 = (p.p929 * t6);
        (t7,)
    } else {
        (s.v[501],)
    }
};
        s.store_scalar(501, t8);s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1523]) {
        (0.0,)
    } else {
        (s.v[651],)
    }
};
        s.store_scalar(651, t9);s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1524]) {
        (0.0,)
    } else {
        (s.v[652],)
    }
};
        s.store_scalar(652, ta);s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1525]) {
        (0.0,)
    } else {
        (s.v[653],)
    }
};
        s.store_scalar(653, tb);s.b[1526] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1526]) {s.store_primal_ln_ad(660, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(667), 1e-21, 1.0));s.store_ln_ad(662, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(669), 1e-21, 1.0));s.store_ln_ad(664, A::div_scalar_offset_denominator((0.5 * p.p822), A::abs(s.ad_value(671)), 1e-21, 1.0));}
        if (s.b[1171] && s.b[1188]) {s.store_primal_min_with_scalar(660, 660, 230.25850929940458);s.store_primal_exp(661, 660);s.store_min_with_scalar(662, 662, 230.25850929940458);s.store_exp(663, 662);s.store_min_with_scalar(664, 664, 230.25850929940458);s.store_exp(665, 664);s.store_scalar(498, 0.4);s.store_scalar(499, 0.65);s.store_scalar(500, 0.8);s.store_primal_mul_scale_offset_indices(485, 552, 498, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(486, 552, 499, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(487, 552, 500, -1.0, 0.0);s.store_scalar(488, 0.1);s.store_scalar(489, 0.2);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1527] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });s.b[1528] = (s.v[485] < s.v[681]);s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        s.b[1529] = (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && s.b[1529]) {s.store_primal_exp_scaled_input(1200, 485, (s.v[371] * (-0.5)));}
        s.b[1530] = (((-0.5) * (s.v[485] * s.v[371])) < 0.0);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && (!s.b[1529])) && s.b[1530]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && (!s.b[1529])) && (!s.b[1530])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(485), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && (!s.b[1528])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 682, A::sub_scaled_inputs(s.ad_value(485), s.v[371], s.ad_value(681), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1527]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1531] = (s.v[485] > 0.0);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1531]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && (!s.b[1531])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 485);}
        if ((s.b[1171] && s.b[1188]) && s.b[1527]) {s.store_primal_sub(1203, 683, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 485, 0.5, 1203, 0.5, 485, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 485, 0.5, 686, 0.5, 485, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 485, A::sqrt_square_offset(s.ad_value(485), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1532] = (s.v[673] == 0.0);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1532]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1532])) {s.store_primal_mul(1208, 563, 1198);}
        s.b[1533] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1533]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {s.store_primal_sub(1210, 569, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1534] = (s.v[511] == 0.5);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && s.b[1534]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && (!s.b[1534])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(511), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1535] = (s.v[511] == 0.5);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && s.b[1535]) {s.store_sqrt_mul(1207, 1210, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && (!s.b[1535])) {s.store_pow_mul_base_indices(1207, 1210, 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {s.store_mul(1214, 590, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 560, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 522, 1215, 1213);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        s.b[1536] = (s.v[525] == 0.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1536]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {s.store_mul_div_scaled_product_indices(1217, 605, 1214, 575, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1537] = (((-s.v[511]) * s.v[578]) == (-1.0));s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1537]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1537])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, 602, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1538] = (s.v[1228] > 0.0);s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1538]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1538])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1539] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1539]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1539])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1540] = (s.v[1228] > 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1540]) {s.copy_ad(1229, 1191);}
        s.b[1541] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) && s.b[1541]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) && (!s.b[1541])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 525, 1215, 1230, 1224, 1.0);}
        s.b[1542] = (s.v[531] == 0.0);s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1542]) {s.store_scalar(1231, 0.0);}
        s.b[1543] = (s.v[511] == 0.5);s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && s.b[1543]) {s.store_sqrt_mul_sub_lhs(1207, 508, 1205, 596);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1543])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(508), s.ad_value(1205)), 596, 511);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), 593, 1.0, 1207, 1.0);}
        s.b[1544] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && s.b[1544]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0));}
        s.b[1545] = (((-s.v[608]) / s.v[1232]) < 0.0);s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1544])) && s.b[1545]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 608, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1544])) && (!s.b[1545])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 608, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 531, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1546] = (s.v[540] > 1000.0);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1546]) {s.store_scalar(1233, 1.0);}
        s.b[1547] = (s.v[1206] > ((-s.v[444]) * s.v[540]));s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });s.b[1548] = (s.v[543] == 4.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) && s.b[1548]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), 1206, 614);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) && (!s.b[1548])) {s.store_pow_abs_mul_base_indices(1207, 1206, 614, 543);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && (!s.b[1547])) {s.store_add_scaled_product_mixed_iai(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1532])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1549] = (s.v[674] == 0.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1549]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1549])) {s.store_primal_mul(1208, 564, 1198);}
        s.b[1550] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1550]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {s.store_primal_sub(1210, 570, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1551] = (s.v[512] == 0.5);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && s.b[1551]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && (!s.b[1551])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1552] = (s.v[512] == 0.5);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && s.b[1552]) {s.store_sqrt_mul(1207, 1210, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && (!s.b[1552])) {s.store_pow_mul_base_indices(1207, 1210, 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {s.store_mul(1214, 591, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 561, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 523, 1215, 1213);}
        s.b[1553] = (s.v[526] == 0.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1553]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {s.store_mul_div_scaled_product_indices(1217, 606, 1214, 576, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1554] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1554]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1554])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, 603, 1220, (-1.0), 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1555] = (s.v[1228] > 0.0);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1555]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1555])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1556] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1556]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1556])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1557] = (s.v[1228] > 0.0);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1557]) {s.copy_ad(1229, 1191);}
        s.b[1558] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) && s.b[1558]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) && (!s.b[1558])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);s.store_mul_product3_indices(1216, 526, 1215, 1230, 1224, 1.0);}
        s.b[1559] = (s.v[532] == 0.0);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1559]) {s.store_scalar(1231, 0.0);}
        s.b[1560] = (s.v[512] == 0.5);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && s.b[1560]) {s.store_sqrt_mul_sub_lhs(1207, 509, 1205, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1560])) {s.store_pow_mul_base_mixed_ai(1207, A::sub(s.ad_value(509), s.ad_value(1205)), 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) {s.store_mul_div_scaled_product_mixed_iaii(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), 594, 1.0, 1207, 1.0);}
        s.b[1561] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && s.b[1561]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0));}
        s.b[1562] = (((-s.v[609]) / s.v[1232]) < 0.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1561])) && s.b[1562]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 609, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1561])) && (!s.b[1562])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 609, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 532, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1563] = (s.v[541] > 1000.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1563]) {s.store_scalar(1233, 1.0);}
        s.b[1564] = (s.v[1206] > ((-s.v[444]) * s.v[541]));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });s.b[1565] = (s.v[544] == 4.0);s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) && s.b[1565]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), 1206, 615);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) && (!s.b[1565])) {s.store_pow_abs_mul_base_indices(1207, 1206, 615, 544);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && (!s.b[1564])) {s.store_add_scaled_product_mixed_iai(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1549])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1566] = (s.v[675] == 0.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1566]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1566])) {s.store_primal_mul(1208, 565, 1198);}
        s.b[1567] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1567]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {s.store_primal_sub(1210, 571, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1568] = (s.v[513] == 0.5);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) {s.store_primal_mul_scale_offset(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1569] = (s.v[513] == 0.5);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1569]) {s.store_sqrt_mul(1207, 1210, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1569])) {s.store_pow_mul_base_indices(1207, 1210, 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {s.store_mul(1214, 592, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 562, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 524, 1215, 1213);}
        s.b[1570] = (s.v[527] == 0.0);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1570]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_mul_div_scaled_product_indices(1217, 607, 1214, 577, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1571] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1571]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1571])) {s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, 604, 1220, (-1.0), 1217, 1222, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1171] && s.b[1188]) && (!s.b[1566])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(475, 673, 1234, 1.0, 674, 1235, 1.0, 675, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1583] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });s.b[1584] = (s.v[486] < s.v[681]);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });s.b[1585] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && s.b[1585]) {s.store_primal_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));}
        s.b[1586] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && s.b[1586]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && (!s.b[1586])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1584])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 682, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(681), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1587] = (s.v[486] > 0.0);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1587]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1587])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);}
        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {s.store_primal_sub(1203, 683, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 486, 0.5, 1203, 0.5, 486, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 486, 0.5, 686, 0.5, 486, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 486, A::sqrt_square_offset(s.ad_value(486), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1588] = (s.v[673] == 0.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1588]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {s.store_primal_mul(1208, 563, 1198);}
        s.b[1589] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1589]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {s.store_primal_sub(1210, 569, 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1590] = (s.v[511] == 0.5);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && s.b[1590]) {s.store_scalar(1212, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
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
        s.b[1598] = (s.v[531] == 0.0);s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1598]) {s.store_scalar(1231, 0.0);}
        s.b[1599] = (s.v[511] == 0.5);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && s.b[1599]) {s.store_sqrt_mul_sub_lhs(1207, 508, 1205, 596);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
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
        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
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
        s.b[1608] = (s.v[512] == 0.5);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1608]) {s.store_sqrt_mul(1207, 1210, 597);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1608])) {s.store_pow_mul_base_indices(1207, 1210, 597, 512);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {s.store_mul(1214, 591, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 561, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 523, 1215, 1213);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
    ) {
        s.b[1609] = (s.v[526] == 0.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1609]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {s.store_mul_div_scaled_product_indices(1217, 606, 1214, 576, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1610] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) {s.store_mul_ad_product_lhs_mixed_ia(1231, 532, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);}
        s.b[1619] = (s.v[541] > 1000.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1619]) {s.store_scalar(1233, 1.0);}
        s.b[1620] = (s.v[1206] > ((-s.v[444]) * s.v[541]));s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });s.b[1621] = (s.v[544] == 4.0);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && s.b[1621]) {s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::square(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), 1206, 615);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && (!s.b[1621])) {s.store_pow_abs_mul_base_indices(1207, 1206, 615, 544);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && (!s.b[1620])) {s.store_add_scaled_product_mixed_iai(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1605])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
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
        s.b[1625] = (s.v[513] == 0.5);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1625]) {s.store_sqrt_mul(1207, 1210, 598);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1625])) {s.store_pow_mul_base_indices(1207, 1210, 598, 513);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {s.store_mul(1214, 592, 1207);s.store_mul_ad_product_lhs_mixed_ia(1215, 562, A::offset(s.ad_value(1201), (-1.0)), 1214);s.store_mul3_lhs(1209, 524, 1215, 1213);}
        s.b[1626] = (s.v[527] == 0.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1626]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {s.store_mul_div_scaled_product_indices(1217, 607, 1214, 577, 1.0, 1210, 1.0);s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1627] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1627]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
    ) {
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
    }
}
