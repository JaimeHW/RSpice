#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1430] = (p.p832 == 0.5);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && s.b[1430]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && (!s.b[1430])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) {
            s.store_scale(1214, 1207, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[385]);
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.b[1431] = (p.p846 == 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1431]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {
            s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1432] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1432]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1432])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[436]), s.ad_value(1218), s.ad_value(1221), s.v[436], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1433] = (s.v[1228] > 0.0);
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1433]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1433])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1434] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1434]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1434])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1435] = (s.v[1228] > 0.0);
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1435]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1436] = (s.v[1227] > (-230.25850929940458));
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) && s.b[1436]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) && (!s.b[1436])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {
            s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);
        }

        s.b[1437] = (p.p852 == 0.0);
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1437]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1438] = (p.p832 == 0.5);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && s.b[1438]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1438])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[430]), ((p.p829) * (s.v[430]))), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(1207), 1.0);
        }

        s.b[1439] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && s.b[1439]) {
            s.store_exp_div_scaled_inputs_indices(1207, 442, -1.0, 1232, 1.0);
        }

        s.b[1440] = (((-s.v[442]) / s.v[1232]) < 0.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1439])) && s.b[1440]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1439])) && (!s.b[1440])) {
            let assign24280_ad_e27382: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign24280_ad_e27382, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852);
        }

        s.b[1441] = (p.p861 > 1000.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1441]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1442] = (s.v[1206] > ((-s.v[444]) * p.p861));
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        s.b[1443] = (p.p864 == 4.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) && s.b[1443]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) && (!s.b[1443])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && (!s.b[1442])) {
            s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1427])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1444] = (s.v[648] == 0.0);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1444]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1444])) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.b[1445] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1445]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1446] = (p.p833 == 0.5);
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && s.b[1446]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && (!s.b[1446])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1447] = (p.p833 == 0.5);
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && s.b[1447]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && (!s.b[1447])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {
            s.store_scale(1214, 1207, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[386]);
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.b[1448] = (p.p847 == 0.0);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1448]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {
            s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1449] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1449]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1449])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[437]), s.ad_value(1218), s.ad_value(1221), s.v[437], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1450] = (s.v[1228] > 0.0);
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1450]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1450])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1451] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1451]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1451])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1452] = (s.v[1228] > 0.0);
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1452]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1453] = (s.v[1227] > (-230.25850929940458));
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) && s.b[1453]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) && (!s.b[1453])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {
            s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);
        }

        s.b[1454] = (p.p853 == 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1454]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1455] = (p.p833 == 0.5);
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && s.b[1455]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1455])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[431]), ((p.p830) * (s.v[431]))), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(1207), 1.0);
        }

        s.b[1456] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && s.b[1456]) {
            s.store_exp_div_scaled_inputs_indices(1207, 443, -1.0, 1232, 1.0);
        }

        s.b[1457] = (((-s.v[443]) / s.v[1232]) < 0.0);
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1456])) && s.b[1457]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1456])) && (!s.b[1457])) {
            let assign24980_ad_e28525: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign24980_ad_e28525, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853);
        }

        s.b[1458] = (p.p862 > 1000.0);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1458]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1459] = (s.v[1206] > ((-s.v[444]) * p.p862));
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        s.b[1460] = (p.p865 == 4.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) && s.b[1460]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) && (!s.b[1460])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && (!s.b[1459])) {
            s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1444])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(478, s.ad_value(646), s.ad_value(1234), 1.0, s.ad_value(647), s.ad_value(1235), 1.0, s.ad_value(648), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1461] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1462] = (s.v[489] < s.v[654]);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        s.b[1463] = (((((-0.5) * (s.v[489] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && s.b[1463]) {
            s.store_exp_scaled_input(1200, 489, (s.v[371] * (-0.5)));
        }

        s.b[1464] = (((-0.5) * (s.v[489] * s.v[371])) < 0.0);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && (!s.b[1463])) && s.b[1464]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(489), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(489), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && (!s.b[1463])) && (!s.b[1464])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && (!s.b[1462])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(489), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1461]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1465] = (s.v[489] > 0.0);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1465]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && (!s.b[1465])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 489);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1461]) {
            s.store_sub(1203, 656, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 489, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(1203)), A::sub(s.ad_value(489), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 489, 0.5, 659, 0.5, A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(659)), A::sub(s.ad_value(489), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 489, 489, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1466] = (s.v[646] == 0.0);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1466]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1466])) {
            s.store_scale(1208, 1198, s.v[387]);
        }

        s.b[1467] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1467]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {
            s.store_sub_from_scalar(1210, s.v[393], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1468] = (p.p831 == 0.5);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && (!s.b[1468])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1469] = (p.p831 == 0.5);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && s.b[1469]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && (!s.b[1469])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[429]), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {
            s.store_scale(1214, 1207, s.v[423]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[384]);
            s.store_scaled_mul(1209, 1215, 1213, p.p840);
        }

        s.b[1470] = (p.p845 == 0.0);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1470]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {
            s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1471] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1471]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1471])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[435]), s.ad_value(1218), s.ad_value(1221), s.v[435], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1472] = (s.v[1228] > 0.0);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1472]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1472])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1473] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1473]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1473])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1474] = (s.v[1228] > 0.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1474]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1475] = (s.v[1227] > (-230.25850929940458));
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) && s.b[1475]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) && (!s.b[1475])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {
            s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);
        }

        s.b[1476] = (p.p851 == 0.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1476]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1477] = (p.p831 == 0.5);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && s.b[1477]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1477])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[429]), ((p.p828) * (s.v[429]))), p.p831);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), s.ad_value(1207), 1.0);
        }

        s.b[1478] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && s.b[1478]) {
            s.store_exp_div_scaled_inputs_indices(1207, 441, -1.0, 1232, 1.0);
        }

        s.b[1479] = (((-s.v[441]) / s.v[1232]) < 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1478])) && s.b[1479]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1478])) && (!s.b[1479])) {
            let assign25980_ad_e30169: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign25980_ad_e30169, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851);
        }

        s.b[1480] = (p.p860 > 1000.0);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1480]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1481] = (s.v[1206] > ((-s.v[444]) * p.p860));
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        s.b[1482] = (p.p863 == 4.0);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) && s.b[1482]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) && (!s.b[1482])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && (!s.b[1481])) {
            s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1466])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1483] = (s.v[647] == 0.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1483]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1483])) {
            s.store_scale(1208, 1198, s.v[388]);
        }

        s.b[1484] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1484]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {
            s.store_sub_from_scalar(1210, s.v[394], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1485] = (p.p832 == 0.5);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && s.b[1485]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && (!s.b[1485])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1486] = (p.p832 == 0.5);
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && s.b[1486]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && (!s.b[1486])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[430]), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {
            s.store_scale(1214, 1207, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[385]);
            s.store_scaled_mul(1209, 1215, 1213, p.p841);
        }

        s.b[1487] = (p.p846 == 0.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1487]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {
            s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1488] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1488]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1488])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[436]), s.ad_value(1218), s.ad_value(1221), s.v[436], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1489] = (s.v[1228] > 0.0);
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1489]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1489])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1490] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1490]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1490])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1491] = (s.v[1228] > 0.0);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1491]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1492] = (s.v[1227] > (-230.25850929940458));
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) && s.b[1492]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) && (!s.b[1492])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1491])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {
            s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);
        }

        s.b[1493] = (p.p852 == 0.0);
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1493]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1494] = (p.p832 == 0.5);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && s.b[1494]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1494])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[430]), ((p.p829) * (s.v[430]))), p.p832);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), s.ad_value(1207), 1.0);
        }

        s.b[1495] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && s.b[1495]) {
            s.store_exp_div_scaled_inputs_indices(1207, 442, -1.0, 1232, 1.0);
        }

        s.b[1496] = (((-s.v[442]) / s.v[1232]) < 0.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1495])) && s.b[1496]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) && (!s.b[1495])) && (!s.b[1496])) {
            let assign26680_ad_e31312: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign26680_ad_e31312, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1493])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852);
        }

        s.b[1497] = (p.p861 > 1000.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1497]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1498] = (s.v[1206] > ((-s.v[444]) * p.p861));
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        s.b[1499] = (p.p864 == 4.0);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) && s.b[1499]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) && (!s.b[1499])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && s.b[1498]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1497])) && (!s.b[1498])) {
            s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1483])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1500] = (s.v[648] == 0.0);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1500]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1500])) {
            s.store_scale(1208, 1198, s.v[389]);
        }

        s.b[1501] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1501]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {
            s.store_sub_from_scalar(1210, s.v[395], 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1502] = (p.p833 == 0.5);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && s.b[1502]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && (!s.b[1502])) {
            s.store_scaled_add_ad_lhs(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1503] = (p.p833 == 0.5);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && s.b[1503]) {
            s.store_sqrt_scaled_input(1207, 1210, s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) && (!s.b[1503])) {
            s.store_powf_ad(1207, A::scale(s.ad_value(1210), s.v[431]), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1501])) {
            s.store_scale(1214, 1207, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(1215, 1201, (-1.0), 1214, s.v[386]);
            s.store_scaled_mul(1209, 1215, 1213, p.p842);
        }

        s.b[1504] = (p.p847 == 0.0);
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1504]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {
            s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);
            s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1505] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1505]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1505])) {
            s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, s.ad_value(1220), (-s.v[437]), s.ad_value(1218), s.ad_value(1221), s.v[437], s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1506] = (s.v[1228] > 0.0);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1506]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1506])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1507] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1507]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1507])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1508] = (s.v[1228] > 0.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && s.b[1508]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1509] = (s.v[1227] > (-230.25850929940458));
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) && s.b[1509]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) && (!s.b[1509])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) && (!s.b[1508])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1504])) {
            s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);
            s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);
        }

        s.b[1510] = (p.p853 == 0.0);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1510]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1511] = (p.p833 == 0.5);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && s.b[1511]) {
            s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1511])) {
            s.store_powf_ad(1207, A::scale_offset(s.ad_value(1205), (-s.v[431]), ((p.p830) * (s.v[431]))), p.p833);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) {
            s.store_div_scaled_offset_numerator(1232, s.ad_value(1205), ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), s.ad_value(1207), 1.0);
        }

        s.b[1512] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && s.b[1512]) {
            s.store_exp_div_scaled_inputs_indices(1207, 443, -1.0, 1232, 1.0);
        }

        s.b[1513] = (((-s.v[443]) / s.v[1232]) < 0.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1512])) && s.b[1513]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) && (!s.b[1512])) && (!s.b[1513])) {
            let assign27380_ad_e32455: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign27380_ad_e32455, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1510])) {
            s.store_mul_scaled_ad_lhs(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853);
        }

        s.b[1514] = (p.p862 > 1000.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1500])) && s.b[1514]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1515] = (s.v[1206] > ((-s.v[444]) * p.p862));
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        s.b[1516] = (p.p865 == 4.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) && s.b[1516]) {
            s.store_mul_scaled_ad_lhs(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450]);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) && (!s.b[1516])) {
            s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && s.b[1515]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1500])) && (!s.b[1514])) && (!s.b[1515])) {
            s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1500])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(479, s.ad_value(646), s.ad_value(1234), 1.0, s.ad_value(647), s.ad_value(1235), 1.0, s.ad_value(648), s.ad_value(1236), 1.0);
            s.store_add_scaled_inputs3_indices(667, 646, s.v[387], 647, s.v[388], 648, s.v[389]);
            s.store_add_scaled_offset_product_rhs_mixed_iia(483, 478, 1.0, 667, A::exp_scaled_input(s.ad_value(488), (s.v[371] * s.v[668])), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 667, A::exp_scaled_input(s.ad_value(489), (s.v[371] * s.v[668])), (-1.0), (-1.0));
        }

        s.b[1517] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        s.b[1518] = ((s.v[478] > 0.0) && (s.v[479] > 0.0));
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        s.b[1519] = ((((((s.v[483] / s.v[478]) > 0.001) || ((s.v[484] / s.v[479]) > 0.001)) && (s.v[483] > 0.0)) && (s.v[484] > 0.0)) && (s.v[484] > s.v[483]));
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1518]) && s.b[1519]) {
            s.store_div(490, 483, 484);
            s.store_div_scaled_inputs(670, A::ln(s.ad_value(490)), s.v[370], A::sub(s.ad_value(488), s.ad_value(489)), 1.0);
            s.store_div_scaled_value_offset_denominator(669, s.ad_value(483), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[371], s.ad_value(670))), (-1.0), 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1517]) {
            s.store_add_scaled_offset_product_rhs_mixed_aia(480, A::add_scaled_offset_product_rhs(s.ad_value(475), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(485), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(485), s.v[371], s.ad_value(670))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(486), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[371], s.ad_value(670))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(667), A::exp_scaled_input(s.ad_value(487), (s.v[371] * s.v[668])), (-1.0), (-1.0)), 1.0, 669, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[371], s.ad_value(670))), (-1.0), (-1.0));
        }

        s.b[1520] = (((s.v[475] < 0.0) && (s.v[476] < 0.0)) && (s.v[477] < 0.0));
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        s.b[1521] = (((((((s.v[480] / s.v[475]) > 0.001) || ((s.v[481] / s.v[476]) > 0.001)) || ((s.v[482] / s.v[477]) > 0.001)) && (s.v[480] < 0.0)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) {
            s.store_div(490, 480, 481);
            s.store_div_scaled_inputs(491, A::ln(s.ad_value(490)), (-s.v[370]), A::sub(s.ad_value(485), s.ad_value(486)), 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(493, 486, 486, 1.0, 485, -1.0);
            s.store_scaled_mul_ad(494, A::offset(s.ad_value(490), (-1.0)), A::offset(A::pow(s.ad_value(490), s.ad_value(493)), (-1.0)), s.v[370]);
            s.store_div_add_scaled_inputs_rhs_indices(493, 485, 485, 1.0, 486, -1.0);
            s.store_sub_ad_lhs(495, A::add_scaled_products(A::pow(s.ad_value(490), s.ad_value(493)), A::sub(s.ad_value(486), s.ad_value(485)), 1.0, s.ad_value(490), s.ad_value(485), 1.0), 486);
            s.store_div(492, 494, 495);
            s.store_add(672, 491, 492);
        }

        s.b[1522] = (((((s.v[487] * s.v[371]) * s.v[672])) as f64).abs() < 1e-6);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        let (assign27730_e33067,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && s.b[1522]) {
        (1.0,)
    } else {
        (s.v[666],)
    }
};
        s.v[666] = assign27730_e33067;

        if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && s.b[1522]) {
            s.store_mul_add_scaled_inputs_rhs(671, 482, A::div_from_scalar(1.0, s.ad_value(487)), 1.0, s.ad_value(672), (0.5 * s.v[371]));
            s.store_div_scaled_product_indices(672, 482, 672, ((-0.5) * s.v[371]), 487, 1.0);
        }

        let (assign27760_e33129,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && (!s.b[1522])) {
        (0.0,)
    } else {
        (s.v[666],)
    }
};
        s.v[666] = assign27760_e33129;

        if (((((s.b[1171] && s.b[1188]) && s.b[1517]) && s.b[1520]) && s.b[1521]) && (!s.b[1522])) {
            s.store_div_scaled_value_offset_denominator(671, s.ad_value(482), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(487), (-s.v[371]), s.ad_value(672))), (-1.0), 1.0);
        }

        let (assign27780_e33173,) = {
    if (s.b[1171] && s.b[1188]) {
        let assign27780_e33162: f64 = (s.v[646] * s.v[414]);
        let assign27780_e33165: f64 = (s.v[647] * s.v[415]);
        let assign27780_e33166: f64 = (assign27780_e33162 + assign27780_e33165);
        let assign27780_e33169: f64 = (s.v[648] * s.v[416]);
        let assign27780_e33170: f64 = (assign27780_e33166 + assign27780_e33169);
        let assign27780_e33171: f64 = (p.p929 * assign27780_e33170);
        (assign27780_e33171,)
    } else {
        (s.v[501],)
    }
};
        s.v[501] = assign27780_e33173;

        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        let (assign27800_e33186,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1523]) {
        (0.0,)
    } else {
        (s.v[651],)
    }
};
        s.v[651] = assign27800_e33186;

        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign27820_e33199,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1524]) {
        (0.0,)
    } else {
        (s.v[652],)
    }
};
        s.v[652] = assign27820_e33199;

        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        let (assign27840_e33212,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1525]) {
        (0.0,)
    } else {
        (s.v[653],)
    }
};
        s.v[653] = assign27840_e33212;

        s.b[1526] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1526]) {
            s.store_ln_ad(660, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(667), 1e-21, 1.0));
            s.store_ln_ad(662, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(669), 1e-21, 1.0));
            s.store_ln_ad(664, A::div_scalar_offset_denominator((0.5 * p.p822), A::abs(s.ad_value(671)), 1e-21, 1.0));
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_min_with_scalar(660, 660, 230.25850929940458);
            s.store_exp(661, 660);
            s.store_min_with_scalar(662, 662, 230.25850929940458);
            s.store_exp(663, 662);
            s.store_min_with_scalar(664, 664, 230.25850929940458);
            s.store_exp(665, 664);
            s.store_scalar(498, 0.4);
            s.store_scalar(499, 0.65);
            s.store_scalar(500, 0.8);
            s.store_mul_neg_lhs(485, 498, 552);
            s.store_mul_neg_lhs(486, 499, 552);
            s.store_mul_neg_lhs(487, 500, 552);
            s.store_scalar(488, 0.1);
            s.store_scalar(489, 0.2);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1527] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        s.b[1528] = (s.v[485] < s.v[681]);
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        s.b[1529] = (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && s.b[1529]) {
            s.store_exp_scaled_input(1200, 485, (s.v[371] * (-0.5)));
        }

        s.b[1530] = (((-0.5) * (s.v[485] * s.v[371])) < 0.0);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && (!s.b[1529])) && s.b[1530]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) && (!s.b[1529])) && (!s.b[1530])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(485), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1528]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && (!s.b[1528])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(485), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1527]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1531] = (s.v[485] > 0.0);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && s.b[1531]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1527]) && (!s.b[1531])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 485);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1527]) {
            s.store_sub(1203, 683, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 485, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(1203)), A::sub(s.ad_value(485), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 485, 0.5, 686, 0.5, A::offset(A::mul(A::sub(s.ad_value(485), s.ad_value(686)), A::sub(s.ad_value(485), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 485, 485, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1532] = (s.v[673] == 0.0);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1532]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1532])) {
            s.store_mul(1208, 563, 1198);
        }

        s.b[1533] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1533]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {
            s.store_sub(1210, 569, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1534] = (s.v[511] == 0.5);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && s.b[1534]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && (!s.b[1534])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(511), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1535] = (s.v[511] == 0.5);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && s.b[1535]) {
            s.store_sqrt_mul(1207, 1210, 596);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) && (!s.b[1535])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1533])) {
            s.store_mul(1214, 590, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(560), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 522, 1215, 1213);
        }

        s.b[1536] = (s.v[525] == 0.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1536]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {
            s.store_mul_div_scaled_product_rhs(1217, 605, s.ad_value(1214), s.ad_value(575), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1537] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1537]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1537])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(602), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1538] = (s.v[1228] > 0.0);
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1538]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1538])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1539] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1539]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1539])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1540] = (s.v[1228] > 0.0);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && s.b[1540]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1541] = (s.v[1227] > (-230.25850929940458));
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) && s.b[1541]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) && (!s.b[1541])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) && (!s.b[1540])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1536])) {
            s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 525, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1542] = (s.v[531] == 0.0);
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1542]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1543] = (s.v[511] == 0.5);
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && s.b[1543]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1543])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) {
            s.store_mul_div_scaled_product_rhs(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(593), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1544] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && s.b[1544]) {
            s.store_exp_div_scaled_inputs_indices(1207, 608, -1.0, 1232, 1.0);
        }

        s.b[1545] = (((-s.v[608]) / s.v[1232]) < 0.0);
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1544])) && s.b[1545]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) && (!s.b[1544])) && (!s.b[1545])) {
            let assign28910_ad_e34842: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign28910_ad_e34842, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1542])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(531), A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1546] = (s.v[540] > 1000.0);
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1532])) && s.b[1546]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1547] = (s.v[1206] > ((-s.v[444]) * s.v[540]));
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        s.b[1548] = (s.v[543] == 4.0);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) && s.b[1548]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(614), A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), s.ad_value(1206), 614);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) && (!s.b[1548])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && s.b[1547]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1532])) && (!s.b[1546])) && (!s.b[1547])) {
            s.store_add_scaled_product_left_ad(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1532])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1549] = (s.v[674] == 0.0);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1549]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1549])) {
            s.store_mul(1208, 564, 1198);
        }

        s.b[1550] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1550]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {
            s.store_sub(1210, 570, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1551] = (s.v[512] == 0.5);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && s.b[1551]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && (!s.b[1551])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1552] = (s.v[512] == 0.5);
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && s.b[1552]) {
            s.store_sqrt_mul(1207, 1210, 597);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) && (!s.b[1552])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1550])) {
            s.store_mul(1214, 591, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(561), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 523, 1215, 1213);
        }

        s.b[1553] = (s.v[526] == 0.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1553]) {
            s.store_scalar(1216, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {
            s.store_mul_div_scaled_product_rhs(1217, 606, s.ad_value(1214), s.ad_value(576), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1554] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1554]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1554])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(603), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1555] = (s.v[1228] > 0.0);
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1555]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1555])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1556] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1556]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1556])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1557] = (s.v[1228] > 0.0);
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && s.b[1557]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1558] = (s.v[1227] > (-230.25850929940458));
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) && s.b[1558]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) && (!s.b[1558])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) && (!s.b[1557])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1553])) {
            s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 526, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1559] = (s.v[532] == 0.0);
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1559]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1560] = (s.v[512] == 0.5);
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && s.b[1560]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1560])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) {
            s.store_mul_div_scaled_product_rhs(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(594), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1561] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && s.b[1561]) {
            s.store_exp_div_scaled_inputs_indices(1207, 609, -1.0, 1232, 1.0);
        }

        s.b[1562] = (((-s.v[609]) / s.v[1232]) < 0.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1561])) && s.b[1562]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) && (!s.b[1561])) && (!s.b[1562])) {
            let assign29610_ad_e35985: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign29610_ad_e35985, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1559])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(532), A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1563] = (s.v[541] > 1000.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1549])) && s.b[1563]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1564] = (s.v[1206] > ((-s.v[444]) * s.v[541]));
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        s.b[1565] = (s.v[544] == 4.0);
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) && s.b[1565]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(615), A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), s.ad_value(1206), 615);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) && (!s.b[1565])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && s.b[1564]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1549])) && (!s.b[1563])) && (!s.b[1564])) {
            s.store_add_scaled_product_left_ad(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1549])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1566] = (s.v[675] == 0.0);
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1566]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1566])) {
            s.store_mul(1208, 565, 1198);
        }

        s.b[1567] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1567]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {
            s.store_sub(1210, 571, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1568] = (s.v[513] == 0.5);
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1568]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1568])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1569] = (s.v[513] == 0.5);
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && s.b[1569]) {
            s.store_sqrt_mul(1207, 1210, 598);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) && (!s.b[1569])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1567])) {
            s.store_mul(1214, 592, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(562), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 524, 1215, 1213);
        }

        s.b[1570] = (s.v[527] == 0.0);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1570]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {
            s.store_mul_div_scaled_product_rhs(1217, 607, s.ad_value(1214), s.ad_value(577), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1571] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1571]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1571])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(604), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1572] = (s.v[1228] > 0.0);
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1572]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1572])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1573] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1573]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1573])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1574] = (s.v[1228] > 0.0);
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && s.b[1574]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1575] = (s.v[1227] > (-230.25850929940458));
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) && s.b[1575]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) && (!s.b[1575])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) && (!s.b[1574])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1570])) {
            s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 527, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1576] = (s.v[533] == 0.0);
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1576]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1577] = (s.v[513] == 0.5);
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && s.b[1577]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) {
            s.store_mul_div_scaled_product_rhs(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(595), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1578] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && s.b[1578]) {
            s.store_exp_div_scaled_inputs_indices(1207, 610, -1.0, 1232, 1.0);
        }

        s.b[1579] = (((-s.v[610]) / s.v[1232]) < 0.0);
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1578])) && s.b[1579]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) && (!s.b[1578])) && (!s.b[1579])) {
            let assign30310_ad_e37128: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign30310_ad_e37128, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1576])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(533), A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1580] = (s.v[542] > 1000.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1566])) && s.b[1580]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1581] = (s.v[1206] > ((-s.v[444]) * s.v[542]));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        s.b[1582] = (s.v[545] == 4.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) && s.b[1582]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(616), A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), s.ad_value(1206), 616);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) && (!s.b[1582])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && s.b[1581]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1566])) && (!s.b[1580])) && (!s.b[1581])) {
            s.store_add_scaled_product_left_ad(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1566])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(475, s.ad_value(673), s.ad_value(1234), 1.0, s.ad_value(674), s.ad_value(1235), 1.0, s.ad_value(675), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1583] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        s.b[1584] = (s.v[486] < s.v[681]);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        s.b[1585] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && s.b[1585]) {
            s.store_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));
        }

        s.b[1586] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && s.b[1586]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) && (!s.b[1585])) && (!s.b[1586])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1584]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1584])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(1201, 1198);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1584])) {
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1587] = (s.v[486] > 0.0);
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && s.b[1587]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1583]) && (!s.b[1587])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1583]) {
            s.store_sub(1203, 683, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 486, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1203)), A::sub(s.ad_value(486), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 486, 0.5, 686, 0.5, A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(686)), A::sub(s.ad_value(486), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 486, 486, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1588] = (s.v[673] == 0.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1588]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {
            s.store_mul(1208, 563, 1198);
        }

        s.b[1589] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1589]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {
            s.store_sub(1210, 569, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1590] = (s.v[511] == 0.5);
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && s.b[1590]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && (!s.b[1590])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(511), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1591] = (s.v[511] == 0.5);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && s.b[1591]) {
            s.store_sqrt_mul(1207, 1210, 596);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) && (!s.b[1591])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1589])) {
            s.store_mul(1214, 590, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(560), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 522, 1215, 1213);
        }

        s.b[1592] = (s.v[525] == 0.0);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1592]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {
            s.store_mul_div_scaled_product_rhs(1217, 605, s.ad_value(1214), s.ad_value(575), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1593] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1593]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1593])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(602), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1594] = (s.v[1228] > 0.0);
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1594]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1594])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1595] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1595]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1595])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1596] = (s.v[1228] > 0.0);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && s.b[1596]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1597] = (s.v[1227] > (-230.25850929940458));
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) && s.b[1597]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) && (!s.b[1597])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) && (!s.b[1596])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1592])) {
            s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 525, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1598] = (s.v[531] == 0.0);
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1598]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1599] = (s.v[511] == 0.5);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && s.b[1599]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1599])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) {
            s.store_mul_div_scaled_product_rhs(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(593), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1600] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && s.b[1600]) {
            s.store_exp_div_scaled_inputs_indices(1207, 608, -1.0, 1232, 1.0);
        }

        s.b[1601] = (((-s.v[608]) / s.v[1232]) < 0.0);
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1600])) && s.b[1601]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) && (!s.b[1600])) && (!s.b[1601])) {
            let assign31310_ad_e38772: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign31310_ad_e38772, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1598])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(531), A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1602] = (s.v[540] > 1000.0);
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1588])) && s.b[1602]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1603] = (s.v[1206] > ((-s.v[444]) * s.v[540]));
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        s.b[1604] = (s.v[543] == 4.0);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) && s.b[1604]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(614), A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), s.ad_value(1206), 614);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) && (!s.b[1604])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && s.b[1603]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1588])) && (!s.b[1602])) && (!s.b[1603])) {
            s.store_add_scaled_product_left_ad(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1588])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1605] = (s.v[674] == 0.0);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1605]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1605])) {
            s.store_mul(1208, 564, 1198);
        }

        s.b[1606] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1606]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_sub(1210, 570, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1607] = (s.v[512] == 0.5);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1607]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1607])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1608] = (s.v[512] == 0.5);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && s.b[1608]) {
            s.store_sqrt_mul(1207, 1210, 597);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) && (!s.b[1608])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_mul(1214, 591, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(561), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 523, 1215, 1213);
        }

        s.b[1609] = (s.v[526] == 0.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1609]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {
            s.store_mul_div_scaled_product_rhs(1217, 606, s.ad_value(1214), s.ad_value(576), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1610] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1610]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1610])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(603), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1611] = (s.v[1228] > 0.0);
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1611]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1611])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1612] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1612]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1612])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1613] = (s.v[1228] > 0.0);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && s.b[1613]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1614] = (s.v[1227] > (-230.25850929940458));
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) && s.b[1614]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) && (!s.b[1614])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) && (!s.b[1613])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1609])) {
            s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 526, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1615] = (s.v[532] == 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1615]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1616] = (s.v[512] == 0.5);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && s.b[1616]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1616])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) {
            s.store_mul_div_scaled_product_rhs(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(594), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1617] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && s.b[1617]) {
            s.store_exp_div_scaled_inputs_indices(1207, 609, -1.0, 1232, 1.0);
        }

        s.b[1618] = (((-s.v[609]) / s.v[1232]) < 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1617])) && s.b[1618]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) && (!s.b[1617])) && (!s.b[1618])) {
            let assign32010_ad_e39915: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign32010_ad_e39915, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1615])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(532), A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1619] = (s.v[541] > 1000.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1605])) && s.b[1619]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1620] = (s.v[1206] > ((-s.v[444]) * s.v[541]));
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        s.b[1621] = (s.v[544] == 4.0);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && s.b[1621]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(615), A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), s.ad_value(1206), 615);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) && (!s.b[1621])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && s.b[1620]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1605])) && (!s.b[1619])) && (!s.b[1620])) {
            s.store_add_scaled_product_left_ad(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1605])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1622] = (s.v[675] == 0.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1622]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1622])) {
            s.store_mul(1208, 565, 1198);
        }

        s.b[1623] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1623]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {
            s.store_sub(1210, 571, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1624] = (s.v[513] == 0.5);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1624]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1624])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1625] = (s.v[513] == 0.5);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && s.b[1625]) {
            s.store_sqrt_mul(1207, 1210, 598);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) && (!s.b[1625])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1623])) {
            s.store_mul(1214, 592, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(562), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 524, 1215, 1213);
        }

        s.b[1626] = (s.v[527] == 0.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1626]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {
            s.store_mul_div_scaled_product_rhs(1217, 607, s.ad_value(1214), s.ad_value(577), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1627] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1627]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1627])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(604), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1628] = (s.v[1228] > 0.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1628]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1628])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1629] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1629]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1629])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1630] = (s.v[1228] > 0.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && s.b[1630]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1631] = (s.v[1227] > (-230.25850929940458));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) && s.b[1631]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) && (!s.b[1631])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) && (!s.b[1630])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1626])) {
            s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 527, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1632] = (s.v[533] == 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1632]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1633] = (s.v[513] == 0.5);
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && s.b[1633]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1633])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) {
            s.store_mul_div_scaled_product_rhs(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(595), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1634] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && s.b[1634]) {
            s.store_exp_div_scaled_inputs_indices(1207, 610, -1.0, 1232, 1.0);
        }

        s.b[1635] = (((-s.v[610]) / s.v[1232]) < 0.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1634])) && s.b[1635]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) && (!s.b[1634])) && (!s.b[1635])) {
            let assign32710_ad_e41058: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign32710_ad_e41058, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1632])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(533), A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1636] = (s.v[542] > 1000.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1622])) && s.b[1636]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1637] = (s.v[1206] > ((-s.v[444]) * s.v[542]));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        s.b[1638] = (s.v[545] == 4.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) && s.b[1638]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(616), A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), s.ad_value(1206), 616);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) && (!s.b[1638])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && s.b[1637]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1622])) && (!s.b[1636])) && (!s.b[1637])) {
            s.store_add_scaled_product_left_ad(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1622])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(476, s.ad_value(673), s.ad_value(1234), 1.0, s.ad_value(674), s.ad_value(1235), 1.0, s.ad_value(675), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1639] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        s.b[1640] = (s.v[487] < s.v[681]);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        s.b[1641] = (((((-0.5) * (s.v[487] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && s.b[1641]) {
            s.store_exp_scaled_input(1200, 487, (s.v[371] * (-0.5)));
        }

        s.b[1642] = (((-0.5) * (s.v[487] * s.v[371])) < 0.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && s.b[1642]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) && (!s.b[1641])) && (!s.b[1642])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1640]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && (!s.b[1640])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(487), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1639]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1643] = (s.v[487] > 0.0);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && s.b[1643]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1639]) && (!s.b[1643])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 487);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1639]) {
            s.store_sub(1203, 683, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 487, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1203)), A::sub(s.ad_value(487), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 487, 0.5, 686, 0.5, A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(686)), A::sub(s.ad_value(487), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 487, 487, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1644] = (s.v[673] == 0.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1644]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1644])) {
            s.store_mul(1208, 563, 1198);
        }

        s.b[1645] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1645]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_sub(1210, 569, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1646] = (s.v[511] == 0.5);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && (!s.b[1646])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(511), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1647] = (s.v[511] == 0.5);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && s.b[1647]) {
            s.store_sqrt_mul(1207, 1210, 596);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) && (!s.b[1647])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_mul(1214, 590, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(560), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 522, 1215, 1213);
        }

        s.b[1648] = (s.v[525] == 0.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1648]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {
            s.store_mul_div_scaled_product_rhs(1217, 605, s.ad_value(1214), s.ad_value(575), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1649] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1649]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1649])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(602), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1650] = (s.v[1228] > 0.0);
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1650]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1650])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1651] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1651]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1651])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1652] = (s.v[1228] > 0.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && s.b[1652]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1653] = (s.v[1227] > (-230.25850929940458));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) && s.b[1653]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) && (!s.b[1653])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) && (!s.b[1652])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1648])) {
            s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 525, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1654] = (s.v[531] == 0.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1654]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1655] = (s.v[511] == 0.5);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && s.b[1655]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1655])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) {
            s.store_mul_div_scaled_product_rhs(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(593), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1656] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && s.b[1656]) {
            s.store_exp_div_scaled_inputs_indices(1207, 608, -1.0, 1232, 1.0);
        }

        s.b[1657] = (((-s.v[608]) / s.v[1232]) < 0.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1656])) && s.b[1657]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) && (!s.b[1656])) && (!s.b[1657])) {
            let assign33710_ad_e42702: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign33710_ad_e42702, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1654])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(531), A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1658] = (s.v[540] > 1000.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1644])) && s.b[1658]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1659] = (s.v[1206] > ((-s.v[444]) * s.v[540]));
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        s.b[1660] = (s.v[543] == 4.0);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) && s.b[1660]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(614), A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), s.ad_value(1206), 614);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) && (!s.b[1660])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && s.b[1659]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1644])) && (!s.b[1658])) && (!s.b[1659])) {
            s.store_add_scaled_product_left_ad(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1644])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1661] = (s.v[674] == 0.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1661]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1661])) {
            s.store_mul(1208, 564, 1198);
        }

        s.b[1662] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1662]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {
            s.store_sub(1210, 570, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1663] = (s.v[512] == 0.5);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1664] = (s.v[512] == 0.5);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && s.b[1664]) {
            s.store_sqrt_mul(1207, 1210, 597);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) && (!s.b[1664])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1662])) {
            s.store_mul(1214, 591, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(561), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 523, 1215, 1213);
        }

        s.b[1665] = (s.v[526] == 0.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1665]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {
            s.store_mul_div_scaled_product_rhs(1217, 606, s.ad_value(1214), s.ad_value(576), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1666] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1666]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1666])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(603), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1667] = (s.v[1228] > 0.0);
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1667]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1667])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1668] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1668]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1668])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1669] = (s.v[1228] > 0.0);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && s.b[1669]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1670] = (s.v[1227] > (-230.25850929940458));
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) && s.b[1670]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) && (!s.b[1670])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) && (!s.b[1669])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1665])) {
            s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 526, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1671] = (s.v[532] == 0.0);
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1671]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1672] = (s.v[512] == 0.5);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) && s.b[1672]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) && (!s.b[1672])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) {
            s.store_mul_div_scaled_product_rhs(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(594), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1673] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) && s.b[1673]) {
            s.store_exp_div_scaled_inputs_indices(1207, 609, -1.0, 1232, 1.0);
        }

        s.b[1674] = (((-s.v[609]) / s.v[1232]) < 0.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) && (!s.b[1673])) && s.b[1674]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) && (!s.b[1673])) && (!s.b[1674])) {
            let assign34410_ad_e43845: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign34410_ad_e43845, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1671])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(532), A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1675] = (s.v[541] > 1000.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1661])) && s.b[1675]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1676] = (s.v[1206] > ((-s.v[444]) * s.v[541]));
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        s.b[1677] = (s.v[544] == 4.0);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1675])) && s.b[1676]) && s.b[1677]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(615), A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), s.ad_value(1206), 615);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1675])) && s.b[1676]) && (!s.b[1677])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1675])) && s.b[1676]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1661])) && (!s.b[1675])) && (!s.b[1676])) {
            s.store_add_scaled_product_left_ad(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1661])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1678] = (s.v[675] == 0.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1678]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1678])) {
            s.store_mul(1208, 565, 1198);
        }

        s.b[1679] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && s.b[1679]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) {
            s.store_sub(1210, 571, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1680] = (s.v[513] == 0.5);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) && s.b[1680]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) && (!s.b[1680])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1681] = (s.v[513] == 0.5);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) && s.b[1681]) {
            s.store_sqrt_mul(1207, 1210, 598);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) && (!s.b[1681])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1679])) {
            s.store_mul(1214, 592, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(562), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 524, 1215, 1213);
        }

        s.b[1682] = (s.v[527] == 0.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && s.b[1682]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) {
            s.store_mul_div_scaled_product_rhs(1217, 607, s.ad_value(1214), s.ad_value(577), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) {
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1683] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && s.b[1683]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1683])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(604), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1684] = (s.v[1228] > 0.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && s.b[1684]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1684])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1685] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && s.b[1685]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1685])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1686] = (s.v[1228] > 0.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && s.b[1686]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1687] = (s.v[1227] > (-230.25850929940458));
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1686])) && s.b[1687]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1686])) && (!s.b[1687])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) && (!s.b[1686])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1682])) {
            s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 527, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1688] = (s.v[533] == 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && s.b[1688]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1689] = (s.v[513] == 0.5);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) && s.b[1689]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) {
            s.store_mul_div_scaled_product_rhs(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(595), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1690] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) && s.b[1690]) {
            s.store_exp_div_scaled_inputs_indices(1207, 610, -1.0, 1232, 1.0);
        }

        s.b[1691] = (((-s.v[610]) / s.v[1232]) < 0.0);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) && (!s.b[1690])) && s.b[1691]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) && (!s.b[1690])) && (!s.b[1691])) {
            let assign35110_ad_e44988: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign35110_ad_e44988, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1688])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(533), A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1692] = (s.v[542] > 1000.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1678])) && s.b[1692]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1693] = (s.v[1206] > ((-s.v[444]) * s.v[542]));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        s.b[1694] = (s.v[545] == 4.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1692])) && s.b[1693]) && s.b[1694]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(616), A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), s.ad_value(1206), 616);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1692])) && s.b[1693]) && (!s.b[1694])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1692])) && s.b[1693]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1678])) && (!s.b[1692])) && (!s.b[1693])) {
            s.store_add_scaled_product_left_ad(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1678])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(477, s.ad_value(673), s.ad_value(1234), 1.0, s.ad_value(674), s.ad_value(1235), 1.0, s.ad_value(675), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1695] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        s.b[1696] = (s.v[488] < s.v[681]);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (((((-0.5) * (s.v[488] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1695]) && s.b[1696]) && s.b[1697]) {
            s.store_exp_scaled_input(1200, 488, (s.v[371] * (-0.5)));
        }

        s.b[1698] = (((-0.5) * (s.v[488] * s.v[371])) < 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && s.b[1698]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1695]) && s.b[1696]) && (!s.b[1697])) && (!s.b[1698])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1695]) && s.b[1696]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1695]) && (!s.b[1696])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(488), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1695]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1699] = (s.v[488] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1695]) && s.b[1699]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1695]) && (!s.b[1699])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 488);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1695]) {
            s.store_sub(1203, 683, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 488, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1203)), A::sub(s.ad_value(488), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 488, 0.5, 686, 0.5, A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(686)), A::sub(s.ad_value(488), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 488, 488, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1700] = (s.v[673] == 0.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1700]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1700])) {
            s.store_mul(1208, 563, 1198);
        }

        s.b[1701] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && s.b[1701]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) {
            s.store_sub(1210, 569, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1702] = (s.v[511] == 0.5);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) && s.b[1702]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) && (!s.b[1702])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(511), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1703] = (s.v[511] == 0.5);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) && s.b[1703]) {
            s.store_sqrt_mul(1207, 1210, 596);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) && (!s.b[1703])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1701])) {
            s.store_mul(1214, 590, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(560), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 522, 1215, 1213);
        }

        s.b[1704] = (s.v[525] == 0.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && s.b[1704]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) {
            s.store_mul_div_scaled_product_rhs(1217, 605, s.ad_value(1214), s.ad_value(575), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1705] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && s.b[1705]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1705])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(602), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1706] = (s.v[1228] > 0.0);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && s.b[1706]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1706])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1707] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && s.b[1707]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1707])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1708] = (s.v[1228] > 0.0);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && s.b[1708]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1709] = (s.v[1227] > (-230.25850929940458));
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1708])) && s.b[1709]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1708])) && (!s.b[1709])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) && (!s.b[1708])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1704])) {
            s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 525, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1710] = (s.v[531] == 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && s.b[1710]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1711] = (s.v[511] == 0.5);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) && s.b[1711]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) && (!s.b[1711])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) {
            s.store_mul_div_scaled_product_rhs(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(593), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1712] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) && s.b[1712]) {
            s.store_exp_div_scaled_inputs_indices(1207, 608, -1.0, 1232, 1.0);
        }

        s.b[1713] = (((-s.v[608]) / s.v[1232]) < 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) && (!s.b[1712])) && s.b[1713]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) && (!s.b[1712])) && (!s.b[1713])) {
            let assign36110_ad_e46632: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign36110_ad_e46632, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1710])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(531), A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1714] = (s.v[540] > 1000.0);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1700])) && s.b[1714]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1715] = (s.v[1206] > ((-s.v[444]) * s.v[540]));
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        s.b[1716] = (s.v[543] == 4.0);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1714])) && s.b[1715]) && s.b[1716]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(614), A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), s.ad_value(1206), 614);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1714])) && s.b[1715]) && (!s.b[1716])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1714])) && s.b[1715]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1700])) && (!s.b[1714])) && (!s.b[1715])) {
            s.store_add_scaled_product_left_ad(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1700])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1717] = (s.v[674] == 0.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1717]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1717])) {
            s.store_mul(1208, 564, 1198);
        }

        s.b[1718] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && s.b[1718]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) {
            s.store_sub(1210, 570, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1719] = (s.v[512] == 0.5);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) && s.b[1719]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) && (!s.b[1719])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1720] = (s.v[512] == 0.5);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) && s.b[1720]) {
            s.store_sqrt_mul(1207, 1210, 597);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) && (!s.b[1720])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1718])) {
            s.store_mul(1214, 591, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(561), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 523, 1215, 1213);
        }

        s.b[1721] = (s.v[526] == 0.0);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && s.b[1721]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) {
            s.store_mul_div_scaled_product_rhs(1217, 606, s.ad_value(1214), s.ad_value(576), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1722] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && s.b[1722]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1722])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(603), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1723] = (s.v[1228] > 0.0);
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && s.b[1723]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1723])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1724] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && s.b[1724]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1724])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1725] = (s.v[1228] > 0.0);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && s.b[1725]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1726] = (s.v[1227] > (-230.25850929940458));
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1725])) && s.b[1726]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1725])) && (!s.b[1726])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) && (!s.b[1725])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1721])) {
            s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 526, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1727] = (s.v[532] == 0.0);
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && s.b[1727]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1728] = (s.v[512] == 0.5);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) && s.b[1728]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) && (!s.b[1728])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) {
            s.store_mul_div_scaled_product_rhs(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(594), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1729] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) && s.b[1729]) {
            s.store_exp_div_scaled_inputs_indices(1207, 609, -1.0, 1232, 1.0);
        }

        s.b[1730] = (((-s.v[609]) / s.v[1232]) < 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) && (!s.b[1729])) && s.b[1730]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) && (!s.b[1729])) && (!s.b[1730])) {
            let assign36810_ad_e47775: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign36810_ad_e47775, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1727])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(532), A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1731] = (s.v[541] > 1000.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1717])) && s.b[1731]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1732] = (s.v[1206] > ((-s.v[444]) * s.v[541]));
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        s.b[1733] = (s.v[544] == 4.0);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1731])) && s.b[1732]) && s.b[1733]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(615), A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), s.ad_value(1206), 615);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1731])) && s.b[1732]) && (!s.b[1733])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1731])) && s.b[1732]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1717])) && (!s.b[1731])) && (!s.b[1732])) {
            s.store_add_scaled_product_left_ad(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1717])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1734] = (s.v[675] == 0.0);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1734]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1734])) {
            s.store_mul(1208, 565, 1198);
        }

        s.b[1735] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && s.b[1735]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) {
            s.store_sub(1210, 571, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1736] = (s.v[513] == 0.5);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) && s.b[1736]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1736])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1737] = (s.v[513] == 0.5);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) && s.b[1737]) {
            s.store_sqrt_mul(1207, 1210, 598);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) && (!s.b[1737])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1735])) {
            s.store_mul(1214, 592, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(562), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 524, 1215, 1213);
        }

        s.b[1738] = (s.v[527] == 0.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && s.b[1738]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) {
            s.store_mul_div_scaled_product_rhs(1217, 607, s.ad_value(1214), s.ad_value(577), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1739] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && s.b[1739]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1739])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(604), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1740] = (s.v[1228] > 0.0);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && s.b[1740]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1740])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1741] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && s.b[1741]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1741])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1742] = (s.v[1228] > 0.0);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && s.b[1742]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1743] = (s.v[1227] > (-230.25850929940458));
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1742])) && s.b[1743]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1742])) && (!s.b[1743])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) && (!s.b[1742])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1738])) {
            s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 527, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1744] = (s.v[533] == 0.0);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && s.b[1744]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1745] = (s.v[513] == 0.5);
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) && s.b[1745]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) && (!s.b[1745])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) {
            s.store_mul_div_scaled_product_rhs(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(595), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1746] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) && s.b[1746]) {
            s.store_exp_div_scaled_inputs_indices(1207, 610, -1.0, 1232, 1.0);
        }

        s.b[1747] = (((-s.v[610]) / s.v[1232]) < 0.0);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) && (!s.b[1746])) && s.b[1747]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) && (!s.b[1746])) && (!s.b[1747])) {
            let assign37510_ad_e48918: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign37510_ad_e48918, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1744])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(533), A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1748] = (s.v[542] > 1000.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1734])) && s.b[1748]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1749] = (s.v[1206] > ((-s.v[444]) * s.v[542]));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        s.b[1750] = (s.v[545] == 4.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1748])) && s.b[1749]) && s.b[1750]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(616), A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), s.ad_value(1206), 616);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1748])) && s.b[1749]) && (!s.b[1750])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1748])) && s.b[1749]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1734])) && (!s.b[1748])) && (!s.b[1749])) {
            s.store_add_scaled_product_left_ad(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1734])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(478, s.ad_value(673), s.ad_value(1234), 1.0, s.ad_value(674), s.ad_value(1235), 1.0, s.ad_value(675), s.ad_value(1236), 1.0);
            s.store_scalar(1205, 0.0);
            s.store_scalar(1202, 0.0);
        }

        s.b[1751] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        s.b[1752] = (s.v[489] < s.v[681]);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        s.b[1753] = (((((-0.5) * (s.v[489] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1751]) && s.b[1752]) && s.b[1753]) {
            s.store_exp_scaled_input(1200, 489, (s.v[371] * (-0.5)));
        }

        s.b[1754] = (((-0.5) * (s.v[489] * s.v[371])) < 0.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && s.b[1751]) && s.b[1752]) && (!s.b[1753])) && s.b[1754]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(489), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(489), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && s.b[1751]) && s.b[1752]) && (!s.b[1753])) && (!s.b[1754])) {
            s.store_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1751]) && s.b[1752]) {
            s.store_div_from_scalar(1201, 1.0, 1200);
            s.store_square(1198, 1201);
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1751]) && (!s.b[1752])) {
            s.store_mul_offset_ad_lhs(1198, A::sub_scaled_inputs(s.ad_value(489), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(1201, 1198);
            s.store_div_from_scalar(1200, 1.0, 1201);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1751]) {
            s.store_offset(1198, 1198, (-1.0));
        }

        s.b[1755] = (s.v[489] > 0.0);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && s.b[1751]) && s.b[1755]) {
            s.store_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && s.b[1751]) && (!s.b[1755])) {
            s.store_sub_ad_lhs(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 489);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1751]) {
            s.store_sub(1203, 683, 1202);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1204, 489, 0.5, 1203, 0.5, A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(1203)), A::sub(s.ad_value(489), s.ad_value(1203))), ((4.0 * s.v[370]) * s.v[370])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1205, 489, 0.5, 686, 0.5, A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(686)), A::sub(s.ad_value(489), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1206, 489, 489, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1756] = (s.v[673] == 0.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1756]) {
            s.store_scalar(1234, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1756])) {
            s.store_mul(1208, 563, 1198);
        }

        s.b[1757] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && s.b[1757]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) {
            s.store_sub(1210, 569, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1758] = (s.v[511] == 0.5);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) && s.b[1758]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) && (!s.b[1758])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(511), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1759] = (s.v[511] == 0.5);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) && s.b[1759]) {
            s.store_sqrt_mul(1207, 1210, 596);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) && (!s.b[1759])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1757])) {
            s.store_mul(1214, 590, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(560), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 522, 1215, 1213);
        }

        s.b[1760] = (s.v[525] == 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && s.b[1760]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) {
            s.store_mul_div_scaled_product_rhs(1217, 605, s.ad_value(1214), s.ad_value(575), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 602, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1761] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && s.b[1761]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1761])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(602), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(602), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1762] = (s.v[1228] > 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && s.b[1762]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1762])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1763] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && s.b[1763]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1763])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1764] = (s.v[1228] > 0.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && s.b[1764]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1765] = (s.v[1227] > (-230.25850929940458));
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1764])) && s.b[1765]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1764])) && (!s.b[1765])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) && (!s.b[1764])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1760])) {
            s.store_div_scaled_product_indices(1230, 602, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 525, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1766] = (s.v[531] == 0.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && s.b[1766]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1767] = (s.v[511] == 0.5);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) && s.b[1767]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) && (!s.b[1767])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) {
            s.store_mul_div_scaled_product_rhs(1232, 578, A::sub(s.ad_value(508), s.ad_value(1205)), s.ad_value(593), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1768] = (((((-s.v[608]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) && s.b[1768]) {
            s.store_exp_div_scaled_inputs_indices(1207, 608, -1.0, 1232, 1.0);
        }

        s.b[1769] = (((-s.v[608]) / s.v[1232]) < 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) && (!s.b[1768])) && s.b[1769]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) && (!s.b[1768])) && (!s.b[1769])) {
            let assign38510_ad_e50562: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign38510_ad_e50562, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1766])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(531), A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1770] = (s.v[540] > 1000.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1756])) && s.b[1770]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1771] = (s.v[1206] > ((-s.v[444]) * s.v[540]));
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        s.b[1772] = (s.v[543] == 4.0);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1770])) && s.b[1771]) && s.b[1772]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(614), A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(1206), s.ad_value(614)), s.ad_value(1206), 614);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1770])) && s.b[1771]) && (!s.b[1772])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1770])) && s.b[1771]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1756])) && (!s.b[1770])) && (!s.b[1771])) {
            s.store_add_scaled_product_left_ad(1233, 611, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(540), s.v[444]), 617, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1756])) {
            s.store_mul_scale_ad_lhs(1234, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1773] = (s.v[674] == 0.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1773]) {
            s.store_scalar(1235, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1773])) {
            s.store_mul(1208, 564, 1198);
        }

        s.b[1774] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && s.b[1774]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) {
            s.store_sub(1210, 570, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1775] = (s.v[512] == 0.5);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) && s.b[1775]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) && (!s.b[1775])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1776] = (s.v[512] == 0.5);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) && s.b[1776]) {
            s.store_sqrt_mul(1207, 1210, 597);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) && (!s.b[1776])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1774])) {
            s.store_mul(1214, 591, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(561), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 523, 1215, 1213);
        }

        s.b[1777] = (s.v[526] == 0.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && s.b[1777]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) {
            s.store_mul_div_scaled_product_rhs(1217, 606, s.ad_value(1214), s.ad_value(576), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 603, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1778] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && s.b[1778]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1778])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(603), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(603), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
        }

    }

    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) {
            s.store_square(1189, 1228);
        }

        s.b[1779] = (s.v[1228] > 0.0);
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && s.b[1779]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1779])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1780] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && s.b[1780]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1780])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1781] = (s.v[1228] > 0.0);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && s.b[1781]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1782] = (s.v[1227] > (-230.25850929940458));
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1781])) && s.b[1782]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1781])) && (!s.b[1782])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) && (!s.b[1781])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1777])) {
            s.store_div_scaled_product_indices(1230, 603, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 526, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1783] = (s.v[532] == 0.0);
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && s.b[1783]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1784] = (s.v[512] == 0.5);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && s.b[1784]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && (!s.b[1784])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) {
            s.store_mul_div_scaled_product_rhs(1232, 579, A::sub(s.ad_value(509), s.ad_value(1205)), s.ad_value(594), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1785] = (((((-s.v[609]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && s.b[1785]) {
            s.store_exp_div_scaled_inputs_indices(1207, 609, -1.0, 1232, 1.0);
        }

        s.b[1786] = (((-s.v[609]) / s.v[1232]) < 0.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && (!s.b[1785])) && s.b[1786]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) && (!s.b[1785])) && (!s.b[1786])) {
            let assign39210_ad_e51705: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign39210_ad_e51705, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1783])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(532), A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1787] = (s.v[541] > 1000.0);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1773])) && s.b[1787]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1788] = (s.v[1206] > ((-s.v[444]) * s.v[541]));
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        s.b[1789] = (s.v[544] == 4.0);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) && s.b[1789]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(615), A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(1206), s.ad_value(615)), s.ad_value(1206), 615);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) && (!s.b[1789])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && s.b[1788]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1773])) && (!s.b[1787])) && (!s.b[1788])) {
            s.store_add_scaled_product_left_ad(1233, 612, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1773])) {
            s.store_mul_scale_ad_lhs(1235, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        s.b[1790] = (s.v[675] == 0.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1790]) {
            s.store_scalar(1236, 0.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1790])) {
            s.store_mul(1208, 565, 1198);
        }

        s.b[1791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1791]) {
            s.store_scalar(1209, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {
            s.store_sub(1210, 571, 1204);
            s.store_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));
        }

        s.b[1792] = (s.v[513] == 0.5);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1792]) {
            s.store_scalar(1212, 0.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1792])) {
            s.store_mul_sub_from_scalar_rhs_ad(1212, A::add(A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), s.ad_value(1211)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {
            s.store_add(1213, 1211, 1212);
        }

        s.b[1793] = (s.v[513] == 0.5);
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && s.b[1793]) {
            s.store_sqrt_mul(1207, 1210, 598);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) && (!s.b[1793])) {
            s.store_pow_ad(1207, A::mul(s.ad_value(1210), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1791])) {
            s.store_mul(1214, 592, 1207);
            s.store_mul_ad_product_lhs(1215, s.ad_value(562), A::offset(s.ad_value(1201), (-1.0)), 1214);
            s.store_mul3_lhs(1209, 524, 1215, 1213);
        }

        s.b[1794] = (s.v[527] == 0.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1794]) {
            s.store_scalar(1216, 0.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {
            s.store_mul_div_scaled_product_rhs(1217, 607, s.ad_value(1214), s.ad_value(577), 1.0, s.ad_value(1210), 1.0);
            s.store_div_scaled_inputs_indices(1218, 604, 0.666666666666667, 1217, 1.0);
            s.store_square(1219, 1218);
            s.store_sqrt_ad(1220, A::div_scaled_product_offset_denominator(s.ad_value(1219), s.ad_value(1219), 1.0, A::square(s.ad_value(1219)), 1.0, 1.0));
            s.store_sqrt(1221, 1220);
            s.store_mul(1222, 1220, 1221);
        }

        s.b[1795] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1795]) {
            s.store_div_from_scalar_offset_ad(1223, 1.0, A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1795])) {
            s.store_pow_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);
            s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);
            s.store_add_scaled_value_products(1227, A::mul3(s.ad_value(604), s.ad_value(1218), s.ad_value(1221)), 1.0, s.ad_value(604), s.ad_value(1220), (-1.0), s.ad_value(1217), s.ad_value(1222), 0.5);
            s.store_mul_offset_lhs(1228, 1226, (-1.0), 1225);
            s.store_square(1189, 1228);
        }

        s.b[1796] = (s.v[1228] > 0.0);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1796]) {
            s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1796])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));
        }

        s.b[1797] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1797]) {
            s.store_exp_sub(1207, 1227, 1189);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1797])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {
            s.store_mul_ad_lhs(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);
        }

        s.b[1798] = (s.v[1228] > 0.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && s.b[1798]) {
            s.copy_ad(1229, 1191);
        }

        s.b[1799] = (s.v[1227] > (-230.25850929940458));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) && s.b[1799]) {
            s.store_exp(1207, 1227);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) && (!s.b[1799])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) && (!s.b[1798])) {
            s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1794])) {
            s.store_div_scaled_product_indices(1230, 604, 1229, (1.772453850905516 * 0.5), 1225, 1.0);
            s.store_mul_product3_rhs(1216, 527, s.ad_value(1215), s.ad_value(1230), s.ad_value(1224), 1.0);
        }

        s.b[1800] = (s.v[533] == 0.0);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1800]) {
            s.store_scalar(1231, 0.0);
        }

        s.b[1801] = (s.v[513] == 0.5);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && s.b[1801]) {
            s.store_sqrt_mul_ad(1207, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1801])) {
            s.store_pow_ad(1207, A::mul(A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) {
            s.store_mul_div_scaled_product_rhs(1232, 580, A::sub(s.ad_value(510), s.ad_value(1205)), s.ad_value(595), 1.0, s.ad_value(1207), 1.0);
        }

        s.b[1802] = (((((-s.v[610]) / s.v[1232])) as f64).abs() < 230.25850929940458);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && s.b[1802]) {
            s.store_exp_div_scaled_inputs_indices(1207, 610, -1.0, 1232, 1.0);
        }

        s.b[1803] = (((-s.v[610]) / s.v[1232]) < 0.0);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1802])) && s.b[1803]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1207, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) && (!s.b[1802])) && (!s.b[1803])) {
            let assign39910_ad_e52848: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1232), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1207, assign39910_ad_e52848, 1e100);
        }

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1800])) {
            s.store_mul_ad_product_lhs(1231, s.ad_value(533), A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207);
        }

        s.b[1804] = (s.v[542] > 1000.0);
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if (((s.b[1171] && s.b[1188]) && (!s.b[1790])) && s.b[1804]) {
            s.store_scalar(1233, 1.0);
        }

        s.b[1805] = (s.v[1206] > ((-s.v[444]) * s.v[542]));
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        s.b[1806] = (s.v[545] == 4.0);
        s.v[1806] = if s.b[1806] { 1.0 } else { 0.0 };

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) && s.b[1806]) {
            s.store_mul_ad_product_lhs(1207, A::mul3(A::mul3(s.ad_value(1206), s.ad_value(616), A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(1206), s.ad_value(616)), s.ad_value(1206), 616);
        }

        if (((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) && (!s.b[1806])) {
            s.store_pow_ad(1207, A::abs(A::mul(s.ad_value(1206), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && s.b[1805]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));
        }

        if ((((s.b[1171] && s.b[1188]) && (!s.b[1790])) && (!s.b[1804])) && (!s.b[1805])) {
            s.store_add_scaled_product_left_ad(1233, 613, 1.0, A::add_scaled_inputs(s.ad_value(1206), 1.0, s.ad_value(542), s.v[444]), 619, 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && (!s.b[1790])) {
            s.store_mul_scale_ad_lhs(1236, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 1233);
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_add_scaled_products3(479, s.ad_value(673), s.ad_value(1234), 1.0, s.ad_value(674), s.ad_value(1235), 1.0, s.ad_value(675), s.ad_value(1236), 1.0);
            s.store_add_scaled_products3(694, s.ad_value(673), s.ad_value(563), 1.0, s.ad_value(674), s.ad_value(564), 1.0, s.ad_value(675), s.ad_value(565), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(483, 478, 1.0, 694, A::exp_scaled_input(s.ad_value(488), (s.v[371] * s.v[695])), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 694, A::exp_scaled_input(s.ad_value(489), (s.v[371] * s.v[695])), (-1.0), (-1.0));
        }

        s.b[1807] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1807] = if s.b[1807] { 1.0 } else { 0.0 };

        s.b[1808] = ((s.v[478] > 0.0) && (s.v[479] > 0.0));
        s.v[1808] = if s.b[1808] { 1.0 } else { 0.0 };

        s.b[1809] = ((((((s.v[483] / s.v[478]) > 0.001) || ((s.v[484] / s.v[479]) > 0.001)) && (s.v[483] > 0.0)) && (s.v[484] > 0.0)) && (s.v[484] > s.v[483]));
        s.v[1809] = if s.b[1809] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1808]) && s.b[1809]) {
            s.store_div(490, 483, 484);
            s.store_div_scaled_inputs(697, A::ln(s.ad_value(490)), s.v[370], A::sub(s.ad_value(488), s.ad_value(489)), 1.0);
            s.store_div_scaled_value_offset_denominator(696, s.ad_value(483), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[371], s.ad_value(697))), (-1.0), 1.0);
        }

        if ((s.b[1171] && s.b[1188]) && s.b[1807]) {
            s.store_add_scaled_offset_product_rhs_mixed_aia(480, A::add_scaled_offset_product_rhs(s.ad_value(475), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(485), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(485), s.v[371], s.ad_value(697))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(486), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[371], s.ad_value(697))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(694), A::exp_scaled_input(s.ad_value(487), (s.v[371] * s.v[695])), (-1.0), (-1.0)), 1.0, 696, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[371], s.ad_value(697))), (-1.0), (-1.0));
        }

        s.b[1810] = (((s.v[475] < 0.0) && (s.v[476] < 0.0)) && (s.v[477] < 0.0));
        s.v[1810] = if s.b[1810] { 1.0 } else { 0.0 };

        s.b[1811] = (((((((s.v[480] / s.v[475]) > 0.001) || ((s.v[481] / s.v[476]) > 0.001)) || ((s.v[482] / s.v[477]) > 0.001)) && (s.v[480] < 0.0)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0));
        s.v[1811] = if s.b[1811] { 1.0 } else { 0.0 };

        if ((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) {
            s.store_div(490, 480, 481);
            s.store_div_scaled_inputs(491, A::ln(s.ad_value(490)), (-s.v[370]), A::sub(s.ad_value(485), s.ad_value(486)), 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(493, 486, 486, 1.0, 485, -1.0);
            s.store_scaled_mul_ad(494, A::offset(s.ad_value(490), (-1.0)), A::offset(A::pow(s.ad_value(490), s.ad_value(493)), (-1.0)), s.v[370]);
            s.store_div_add_scaled_inputs_rhs_indices(493, 485, 485, 1.0, 486, -1.0);
            s.store_sub_ad_lhs(495, A::add_scaled_products(A::pow(s.ad_value(490), s.ad_value(493)), A::sub(s.ad_value(486), s.ad_value(485)), 1.0, s.ad_value(490), s.ad_value(485), 1.0), 486);
            s.store_div(492, 494, 495);
            s.store_add(699, 491, 492);
        }

        s.b[1812] = (((((s.v[487] * s.v[371]) * s.v[699])) as f64).abs() < 1e-6);
        s.v[1812] = if s.b[1812] { 1.0 } else { 0.0 };

        let (assign40260_e53460,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && s.b[1812]) {
        (1.0,)
    } else {
        (s.v[693],)
    }
};
        s.v[693] = assign40260_e53460;

        if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && s.b[1812]) {
            s.store_mul_add_scaled_inputs_rhs(698, 482, A::div_from_scalar(1.0, s.ad_value(487)), 1.0, s.ad_value(699), (0.5 * s.v[371]));
            s.store_div_scaled_product_indices(699, 482, 699, ((-0.5) * s.v[371]), 487, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let (assign40290_e53522,) = {
    if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && (!s.b[1812])) {
        (0.0,)
    } else {
        (s.v[693],)
    }
};
        s.v[693] = assign40290_e53522;

        if (((((s.b[1171] && s.b[1188]) && s.b[1807]) && s.b[1810]) && s.b[1811]) && (!s.b[1812])) {
            s.store_div_scaled_value_offset_denominator(698, s.ad_value(482), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(487), (-s.v[371]), s.ad_value(699))), (-1.0), 1.0);
        }

        let (assign40310_e53566,) = {
    if (s.b[1171] && s.b[1188]) {
        let assign40310_e53555: f64 = (s.v[673] * s.v[581]);
        let assign40310_e53558: f64 = (s.v[674] * s.v[582]);
        let assign40310_e53559: f64 = (assign40310_e53555 + assign40310_e53558);
        let assign40310_e53562: f64 = (s.v[675] * s.v[583]);
        let assign40310_e53563: f64 = (assign40310_e53559 + assign40310_e53562);
        let assign40310_e53564: f64 = (s.v[553] * assign40310_e53563);
        (assign40310_e53564,)
    } else {
        (s.v[501],)
    }
};
        s.v[501] = assign40310_e53566;

        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);
        s.v[1813] = if s.b[1813] { 1.0 } else { 0.0 };

        let (assign40330_e53579,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1813]) {
        (0.0,)
    } else {
        (s.v[678],)
    }
};
        s.v[678] = assign40330_e53579;

        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);
        s.v[1814] = if s.b[1814] { 1.0 } else { 0.0 };

        let (assign40350_e53592,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1814]) {
        (0.0,)
    } else {
        (s.v[679],)
    }
};
        s.v[679] = assign40350_e53592;

        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);
        s.v[1815] = if s.b[1815] { 1.0 } else { 0.0 };

        let (assign40370_e53605,) = {
    if ((s.b[1171] && s.b[1188]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[680],)
    }
};
        s.v[680] = assign40370_e53605;

        s.b[1816] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[1816] = if s.b[1816] { 1.0 } else { 0.0 };

        if ((s.b[1171] && s.b[1188]) && s.b[1816]) {
            s.store_ln_ad(687, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(694), 1e-21, 1.0));
            s.store_ln_ad(689, A::div_scalar_offset_denominator((0.5 * p.p822), s.ad_value(696), 1e-21, 1.0));
            s.store_ln_ad(691, A::div_scalar_offset_denominator((0.5 * p.p822), A::abs(s.ad_value(698)), 1e-21, 1.0));
        }

        if (s.b[1171] && s.b[1188]) {
            s.store_min_with_scalar(687, 687, 230.25850929940458);
            s.store_exp(688, 687);
            s.store_min_with_scalar(689, 689, 230.25850929940458);
            s.store_exp(690, 689);
            s.store_min_with_scalar(691, 691, 230.25850929940458);
            s.store_exp(692, 691);
        }

        s.v[2027] = 0.0;

        s.v[2028] = 0.0;

        s.v[2029] = 0.0;

        s.v[1937] = 1.0;

        s.v[1936] = 0.0;

        s.b[2102] = (s.v[0] == 1.0);
        s.v[2102] = if s.b[2102] { 1.0 } else { 0.0 };

        if s.b[2102] {
            s.store_voltage(825, ctx, nodes, Some(5), Some(6));
            s.store_voltage(826, ctx, nodes, Some(7), Some(6));
            s.store_voltage(827, ctx, nodes, Some(6), Some(8));
            s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);
        }

        if (!s.b[2102]) {
            s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(832, ctx, nodes, Some(6), Some(10));
            s.store_voltage(833, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(829, 825, 827);

        s.copy_ad(834, 825);

        s.copy_ad(835, 827);

        s.store_add(836, 826, 827);

        s.store_sub(837, 825, 826);

        s.store_scale(1817, 834, (-s.v[355]));

        s.store_scale(1818, 837, (-s.v[355]));

        s.store_scaled_sub(1819, 829, 700, (-s.v[355]));

        s.v[831] = 1.0;

        s.b[2103] = (s.v[826] < 0.0);
        s.v[2103] = if s.b[2103] { 1.0 } else { 0.0 };

        if s.b[2103] {
            s.store_scalar(831, (-1.0));
            s.store_sub(825, 825, 826);
            s.store_add(827, 827, 826);
            s.store_neg(826, 826);
        }

        s.store_add(828, 826, 827);

        s.store_div_scaled_product_offset_denominator(830, s.ad_value(826), s.ad_value(826), 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add_scaled_product(s.ad_value(739), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5), 737, 1.0);

        s.copy_ad(1820, 2107);

        s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(738), 1.0, s.ad_value(2107), s.ad_value(2107), 1.0)), (-(-0.5)), 741, 1.0);

        s.copy_ad(1821, 2030);

        s.v[2031] = 0.0;

        s.b[2263] = ((p.p45 != 0.0) && (s.v[184] != 1.0));
        s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };

        if s.b[2263] {
            s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));
            s.store_sub_ad_lhs(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);
            s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);
            s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));
            s.store_sub(2031, 1821, 2030);
        }

        s.copy_ad(2104, 728);

        s.copy_ad(2105, 738);

        s.copy_ad(2106, 729);

        s.copy_ad(2108, 2030);

        s.copy_ad(2112, 2031);

        s.copy_ad(2109, 720);

        s.copy_ad(2110, 777);

        s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);

        s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));

        s.v[2125] = 1.0;

        s.b[2264] = (s.v[190] > 0.0);
        s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };

        if s.b[2264] {
            s.store_scale(2116, 2104, s.v[361]);
            s.store_scale(2117, 2113, s.v[361]);
            s.store_scale(2118, 2111, s.v[361]);
            s.store_offset_div_scaled_inputs_mixed_ia(2028, 2106, 0.5, A::sqrt(s.ad_value(2116)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));
            s.store_offset_scaled(2120, 2116, 0.5, 2.0);
            s.store_add(2121, 2116, 2117);
            s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);
            s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2119, 0.5, 2122, 0.5, A::offset(A::mul(A::sub(s.ad_value(2119), s.ad_value(2122)), A::sub(s.ad_value(2119), s.ad_value(2122))), 20.0), 0.5);
            s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2123, 2028, 0.5, 2029, 0.5, A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2028, 2123, 0.5, 2120, 0.5, A::offset(A::mul(A::sub(s.ad_value(2123), s.ad_value(2120)), A::sub(s.ad_value(2123), s.ad_value(2120))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0), A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0);
        }

        s.b[2265] = (s.v[2029] > (-230.25850929940458));
        s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };

        if (s.b[2264] && s.b[2265]) {
            s.store_exp(2125, 2029);
        }

        if (s.b[2264] && (!s.b[2265])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2126, 701, 2125, 1.0);

        s.store_scale(2127, 2126, s.v[715]);

        s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));

        s.store_mul_offset_rhs(2129, 2127, 2128, 1.0);

        s.store_div_from_scalar(2130, 1.0, 2129);

        s.store_mul_ad_rhs(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));

        s.store_square(2115, 2114);

        s.store_div_from_scalar(2131, 1.0, 2115);

        s.store_mul(2132, 2108, 2130);

        s.store_mul(2133, 2111, 2130);

        s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs(2135, 196, s.ad_value(2134), A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));

        s.store_mul(2136, 2104, 2130);

        s.store_sqrt_square_add(2028, 2107, 2105);

        s.store_sqrt_ad(2029, A::add_scaled_product(s.ad_value(2105), 1.0, A::sub(s.ad_value(2107), s.ad_value(2135)), A::sub(s.ad_value(2107), s.ad_value(2135)), 1.0));

        s.store_mul_add_scaled_inputs3_offset_rhs(2137, 2130, s.ad_value(2135), 0.5, s.ad_value(2028), 0.5, s.ad_value(2029), ((-1.0) * (0.5)), 0.0);

        s.store_add(2138, 2136, 2132);

        s.store_sub(2139, 2138, 2137);

        s.b[2266] = (p.p45 > 0.0);
        s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };

        s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);
        s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };

        if (s.b[2266] && s.b[2267]) {
            s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);
        }

        s.b[2268] = (s.v[2139] < 460.51701859880916);
        s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };

        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {
            s.store_exp_neg_input(2154, 2139);
        }

        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2139), (-460.51701859880916), A::scale_offset(s.ad_value(2139), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2266] && (!s.b[2267])) {
            s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);
        }

        if (!s.b[2266]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2140, 2114, 0.5, A::sqrt(s.ad_value(2139)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2141, s.ad_value(2139), 1.0, s.ad_value(2114), A::sqrt(s.ad_value(2139)), 1.0, s.ad_value(2140), A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));

        s.v[2147] = 0.0;

        s.v[2149] = 1.0;

        s.b[2269] = (s.v[2142] > (-30.0));
        s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };

        if s.b[2269] {
            s.store_offset_mul(2143, 2140, 2142, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2027, 2143, 2143, 10.0, 0.5);
            s.store_sub_ad_rhs(2144, 2142, A::ln(s.ad_value(2027)));
            s.store_scaled_add_sqrt_square_offset_rhs(2145, 2144, 2144, 2.0, 0.5);
        }

        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);
        s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };

        if (s.b[2269] && s.b[2270]) {
            s.store_exp_sub(2027, 2142, 2145);
        }

        if (s.b[2269] && (!s.b[2270])) {
            s.store_scaled_offset_mul_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2142), s.ad_value(2145)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[2269] {
            s.store_div(2146, 2027, 2140);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);
        }

        s.b[2271] = (s.v[2146] > 1e-6);
        s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };

        if (s.b[2269] && s.b[2271]) {
            s.store_mul_offset_ad_rhs(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0);
        }

        if (s.b[2269] && (!s.b[2271])) {
            s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if s.b[2269] {
            s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_offset_square_offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);
            s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));
        }

        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);

        let assign41730_e54704: f64 = (1e-5 * s.v[2150]);
        s.v[2151] = assign41730_e54704;

        s.store_div_from_scalar(2152, 1.0, 2150);

        s.v[2259] = 0.0;

        s.v[2153] = 0.0;

        s.b[2272] = (s.v[2139] < 460.51701859880916);
        s.v[2272] = if s.b[2272] { 1.0 } else { 0.0 };

        if s.b[2272] {
            s.store_exp_neg_input(2154, 2139);
        }

        if (!s.b[2272]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2139), (-460.51701859880916), A::scale_offset(s.ad_value(2139), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };

        if s.b[2273] {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if s.b[2273] {
            s.store_mul_ad_product_rhs(2153, 2133, s.ad_value(2152), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        s.b[2274] = (s.v[2133] < (-s.v[2151]));
        s.v[2274] = if s.b[2274] { 1.0 } else { 0.0 };

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_neg(2241, 2133);
            s.store_scaled_mul(2242, 2241, 2152, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2238, 2241, 2243);
            s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);
            s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);
            s.store_sub_ad_lhs(2246, A::ln(A::mul(s.ad_value(2244), s.ad_value(2131))), 2243);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);
            s.store_add_ad_rhs(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));
        }

        s.b[2275] = (s.v[2247] < 230.25850929940458);
        s.v[2275] = if s.b[2275] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {
            s.store_exp(2248, 2247);
        }

        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2247), (-230.25850929940458), A::scale_offset(s.ad_value(2247), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((!s.b[2273]) && s.b[2274]) {
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2247)), 2.0);
            s.store_mul_square_lhs(2250, 2247, 2238);
            s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2241, 2247);
            s.store_mul(2239, 2154, 2249);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2256, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), (-1.0), 2255);
            s.store_mul_ad_product_rhs(2257, 2133, s.ad_value(2152), A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));
        }

        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));
        s.v[2276] = if s.b[2276] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {
            s.store_exp_neg_input(2238, 2257);
        }

        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar(2258, 1.0, 2238);
            s.store_add_scaled_inputs_product_right_ad(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));
            s.store_offset(2260, 2139, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_ad(2240, 1.0, A::square(s.ad_value(2243)), 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), s.ad_value(2240), 2240);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2246, 2139, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2277] = (s.v[2262] < 230.25850929940458);
        s.v[2277] = if s.b[2277] { 1.0 } else { 0.0 };

        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2154, 2248);
        }

        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));
        s.v[2278] = if s.b[2278] { 1.0 } else { 0.0 };

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {
            s.store_exp_sub(2248, 2262, 2139);
            s.store_div(2249, 2154, 2248);
        }

        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2139), s.ad_value(2262)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2262), (-230.25850929940458), A::scale_offset(s.ad_value(2262), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((!s.b[2273]) && (!s.b[2274])) {
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2262)), 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        s.v[2156] = 0.0;

        s.v[2157] = 0.0;

        s.v[2158] = 0.0;

        s.v[2159] = 0.0;

        s.v[2160] = 0.0;

        s.v[2161] = 0.0;

        s.v[2162] = 0.0;

        s.v[2163] = 1.0;

        s.v[2164] = 1.0;

        s.store_sub(2165, 2133, 2153);

        s.v[2166] = 0.0;

        s.store_mul(2167, 2129, 2165);

        s.v[2168] = 1.0;

        s.v[2169] = 1.0;

        s.v[2173] = 1.0;

        s.v[2174] = 1.0;

        s.v[2176] = 1.0;

        s.b[2279] = (s.v[2133] > 0.0);
        s.v[2279] = if s.b[2279] { 1.0 } else { 0.0 };

        if s.b[2279] {
            s.store_div_from_scalar_offset_ad(2027, 1.0, A::square(s.ad_value(2153)), 2.0);
            s.store_mul_square_lhs(2155, 2153, 2027);
            s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), s.ad_value(2027), 2027);
            s.store_scalar(2158, 0.0);
        }

        s.b[2280] = (s.v[2153] < 230.25850929940458);
        s.v[2280] = if s.b[2280] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2280]) {
            s.store_exp(2158, 2153);
            s.store_div_from_scalar(2159, 1.0, 2158);
            s.store_mul(2158, 2154, 2158);
        }

        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));
        s.v[2281] = if s.b[2281] { 1.0 } else { 0.0 };

        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {
            s.store_exp_sub(2158, 2153, 2139);
            s.store_div(2159, 2154, 2158);
        }

        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2139), s.ad_value(2153)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2153), (-230.25850929940458), A::scale_offset(s.ad_value(2153), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2279] {
            s.store_add_scaled_product_right_ad(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));
        }

        s.b[2282] = (s.v[2153] < 1e-5);
        s.v[2282] = if s.b[2282] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2282]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);
            s.store_offset_div_scaled_product(2163, s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0, 1.0);
        }

        if (s.b[2279] && (!s.b[2282])) {
            s.store_add_offset_lhs(2161, 2153, (-1.0), 2159);
            s.store_sqrt(2162, 2161);
            s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);
        }

        if s.b[2279] {
            s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);
        }

        s.b[2283] = (s.v[2160] > 1e-100);
        s.v[2283] = if s.b[2283] { 1.0 } else { 0.0 };

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_sqrt_ad_rhs(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));
            s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);
            s.store_mul3_lhs(2167, 2162, 2114, 2129);
        }

        s.b[2284] = (s.v[217] < 0.0);
        s.v[2284] = if s.b[2284] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {
            s.store_offset_mul(2168, 217, 2113, 1.0);
        }

        s.b[2285] = (s.v[218] < 0.0);
        s.v[2285] = if s.b[2285] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {
            s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2166, 1.0);
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {
            s.store_div_from_scalar_offset_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2166)), 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul_product3_rhs(2170, 2166, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_mul_add_scaled_product_rhs(2171, 774, s.ad_value(2167), 1.0, s.ad_value(775), s.ad_value(2166), 1.0);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2173, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
        }

        s.b[2286] = (s.v[221] < 0.0);
        s.v[2286] = if s.b[2286] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {
            s.store_offset_mul(2174, 221, 2113, 1.0);
        }

        if (s.b[2279] && s.b[2283]) {
            s.store_mul(2029, 2166, 2174);
            s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2287] = (s.v[222] < 0.0);
        s.v[2287] = if s.b[2287] { 1.0 } else { 0.0 };

        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        s.copy_ad(1822, 2111);

        s.copy_ad(1823, 2113);

        s.copy_ad(1824, 2129);

        s.copy_ad(1825, 2130);

        s.copy_ad(1826, 2114);

        s.copy_ad(1827, 2115);

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        s.copy_ad(1828, 2131);

        s.copy_ad(1829, 2133);

        s.copy_ad(1830, 2138);

        s.copy_ad(1831, 2139);

        s.copy_ad(1832, 2150);

        s.v[1833] = s.v[2151];

        s.copy_ad(1834, 2152);

        s.copy_ad(1835, 2259);

        s.copy_ad(1836, 2154);

        s.copy_ad(1837, 2153);

        s.copy_ad(1838, 2156);

        s.copy_ad(1839, 2157);

        s.copy_ad(1840, 2158);

        s.copy_ad(1841, 2159);

        s.copy_ad(1842, 2161);

        s.copy_ad(1843, 2160);

        s.copy_ad(1844, 2162);

        s.copy_ad(1845, 2163);

        s.copy_ad(1846, 2164);

        s.copy_ad(1847, 2165);

        s.copy_ad(1848, 2166);

        s.copy_ad(1849, 2167);

        s.copy_ad(1850, 2168);

        s.copy_ad(1851, 2169);

        s.copy_ad(1852, 2173);

        s.copy_ad(1853, 2174);

        s.copy_ad(1854, 2176);

        s.v[2178] = 0.0;

        s.store_scale(2177, 2129, 4.60517018598809);

        s.copy_ad(2194, 2177);

        s.copy_ad(2195, 826);

        s.store_mul(2196, 826, 2130);

        s.copy_ad(2200, 2153);

        s.v[2201] = 0.0;

        s.v[2204] = 0.0;

        s.copy_ad(2206, 2159);

        s.copy_ad(2207, 2161);

        s.copy_ad(2209, 2160);

        s.copy_ad(2210, 2167);

        s.copy_ad(2211, 2153);

        s.copy_ad(2212, 2159);

        s.copy_ad(2214, 2160);

        s.copy_ad(2215, 2161);

        s.store_sub(2216, 2133, 2153);

        s.v[2217] = 1.0;

        s.v[2219] = 1.0;

        s.v[2218] = 0.0;

        s.copy_ad(2228, 2166);

        s.store_mul(2232, 2216, 2129);

        s.v[2229] = 0.0;

        s.copy_ad(2230, 2167);

        s.v[2235] = 0.0;

        s.v[2234] = 1.0;

        s.copy_ad(2237, 2109);

        s.copy_ad(2236, 2232);

        s.b[2288] = (s.v[2133] > 0.0);
        s.v[2288] = if s.b[2288] { 1.0 } else { 0.0 };

        s.b[2289] = (s.v[2160] > 1e-100);
        s.v[2289] = if s.b[2289] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2237, 2109, 2176);
            s.store_div(2178, 2237, 2173);
            s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);
            s.store_div_scaled_product_by_product(2027, s.ad_value(2115), s.ad_value(2158), 1.0, s.ad_value(2179), s.ad_value(2179), 1.0);
        }

        s.b[2290] = (s.v[2027] > 0.0001);
        s.v[2290] = if s.b[2290] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2291] = (s.v[2028] < 1e-10);
        s.v[2291] = if s.b[2291] { 1.0 } else { 0.0 };

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {
            s.store_scalar(2029, 1.0);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul(2180, 2029, 2179);
        }

        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.v[2292] = if s.b[2292] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_scaled_mul(2181, 2129, 2180, 0.475);
            s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2182, 2027, 2027, 1e-12, 0.5);
            s.store_add_scaled_value_products(2183, s.ad_value(2166), (-1.0), s.ad_value(2129), s.ad_value(2165), 1.0, A::offset(s.ad_value(2163), (-1.0)), s.ad_value(2181), 1.0);
            s.store_offset_div_scaled_product(2184, s.ad_value(2115), s.ad_value(2129), 0.5, s.ad_value(2183), 1.0, 1.0);
            s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);
            s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);
            s.store_div(2027, 2182, 2183);
            s.store_mul_pow_ad_rhs(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_div_scaled_product_rhs(2029, 2186, s.ad_value(707), A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, s.ad_value(2183), 1.0);
            s.store_mul_product3_rhs(2187, 2182, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);
        }

        s.b[2293] = (s.v[2027] < 230.25850929940458);
        s.v[2293] = if s.b[2293] { 1.0 } else { 0.0 };

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {
            s.copy_ad(2028, 2027);
        }

        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {
            s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt_square_offset(s.ad_value(2188), 1.0), 1.0, 1.0), 1.0);
        }

        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {
            s.copy_ad(2189, 2180);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);
        }

        s.b[2294] = (s.v[0] == (-1.0));
        s.v[2294] = if s.b[2294] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {
            s.store_div_ad_rhs(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2191, 2190);
            s.store_mul_ad_product_rhs(2192, 2189, s.ad_value(2191), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));
            s.store_scale(2193, 2192, 0.99);
            s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);
        }

        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_ad_rhs(2194, 2129, s.ad_value(2193), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2288] && (!s.b[2289])) {
            s.copy_ad(2194, 2177);
        }

        if s.b[2288] {
            s.store_offset(2027, 2110, 1.0);
            s.store_div_scaled_product_left_ad(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);
            s.store_mul(2196, 2195, 2130);
            s.store_add(2197, 2139, 2196);
        }

        s.b[2295] = (s.v[2196] < 460.51701859880916);
        s.v[2295] = if s.b[2295] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2295]) {
            s.store_exp_neg_input(2198, 2196);
        }

        if (s.b[2288] && (!s.b[2295])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2196), (-460.51701859880916), A::scale_offset(s.ad_value(2196), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2288] {
            s.store_mul(2199, 2154, 2198);
        }

        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);
        s.v[2296] = if s.b[2296] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2296]) {
            s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2200, 2133, s.ad_value(2152), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_offset(2260, 2197, 3.0);
            s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));
            s.store_sub(2238, 2133, 2243);
            s.store_exp_neg_input(2239, 2243);
            s.store_div_from_scalar_offset_ad(2240, 1.0, A::square(s.ad_value(2243)), 2.0);
            s.store_mul_square_lhs(2250, 2243, 2240);
            s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), s.ad_value(2240), 2240);
        }

        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2246, 2197, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);
            s.store_add(824, 2244, 2245);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);
            s.store_add_ad_rhs(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));
        }

        s.b[2297] = (s.v[2262] < 230.25850929940458);
        s.v[2297] = if s.b[2297] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {
            s.store_exp(2248, 2262);
            s.store_div_from_scalar(2249, 1.0, 2248);
            s.store_mul(2248, 2199, 2248);
        }

        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));
        s.v[2298] = if s.b[2298] { 1.0 } else { 0.0 };

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {
            s.store_exp_sub(2248, 2262, 2197);
            s.store_div(2249, 2199, 2248);
        }

        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2197), s.ad_value(2262)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2262), (-230.25850929940458), A::scale_offset(s.ad_value(2262), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2288] && (!s.b[2296])) {
            s.store_div_from_scalar_offset_ad(2238, 1.0, A::square(s.ad_value(2262)), 2.0);
            s.store_mul_square_lhs(2250, 2262, 2238);
            s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);
            s.store_mul_ad_product_lhs(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), s.ad_value(2238), 2238);
            s.store_sub(2238, 2133, 2262);
            s.store_add_scaled_product_right_ad(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);
        }

        if s.b[2288] {
            s.store_sub(2201, 2200, 2153);
        }

        s.b[2299] = (s.v[2201] < 1e-10);
        s.v[2299] = if s.b[2299] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2299]) {
            s.store_add_scaled_inputs_product_right_ad(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2027, 2.0, 2115, A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));
            s.store_scaled_div_ad_rhs(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2200, 2153, 2201);
        }

        if s.b[2288] {
            s.store_mul(2204, 2201, 2129);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2288] {
            s.store_div_scaled_product_offset_denominator(2205, s.ad_value(2200), s.ad_value(2200), 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);
        }

        s.b[2300] = (s.v[2200] < 230.25850929940458);
        s.v[2300] = if s.b[2300] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2300]) {
            s.store_exp_neg_input(2206, 2200);
        }

        s.b[2301] = (s.v[2200] < 1e-5);
        s.v[2301] = if s.b[2301] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);
            s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));
        }

        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {
            s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);
            s.store_sqrt(2208, 2207);
            s.store_mul_add_scaled_inputs3_offset_rhs(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, s.ad_value(2200), (-1.0), s.ad_value(2205), -1.0, (-1.0));
        }

        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));
        s.v[2302] = if s.b[2302] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {
            s.store_exp_sub(2027, 2200, 2197);
            s.store_div(2206, 2199, 2027);
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2200), (-230.25850929940458), A::scale_offset(s.ad_value(2200), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2197), s.ad_value(2200)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));
        }

        if (s.b[2288] && (!s.b[2300])) {
            s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);
            s.store_sqrt(2208, 2207);
        }

        if s.b[2288] {
            s.store_mul3_lhs(2210, 2208, 2114, 2129);
            s.store_scaled_add(2211, 2153, 2200, 0.5);
            s.store_scalar(2212, 0.0);
            s.store_mul(2027, 2206, 2159);
        }

        s.b[2303] = (s.v[2027] > 0.0);
        s.v[2303] = if s.b[2303] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2303]) {
            s.store_sqrt(2212, 2027);
        }

        if s.b[2288] {
            s.store_scaled_add(2213, 2160, 2209, 0.5);
            s.store_add_scaled_product_mixed_iaa(2214, 2213, 1.0, A::square(s.ad_value(2201)), A::sub_scaled_inputs(s.ad_value(2212), 1.0, s.ad_value(2131), 2.0), 0.125);
        }

        s.b[2304] = (s.v[2211] < 1e-5);
        s.v[2304] = if s.b[2304] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2304]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2215, 2211, 1.0, 2211, 1.0, 2211, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2305] = (s.v[730] > 0.0);
        s.v[2305] = if s.b[2305] { 1.0 } else { 0.0 };

        if ((s.b[2288] && s.b[2304]) && s.b[2305]) {
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
        }

        if (s.b[2288] && s.b[2304]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2211), 1.0, A::scale(s.ad_value(2211), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2219, 2217, A::div_scaled_product(s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), 1.0, A::square(s.ad_value(2211)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
        }

        s.b[2306] = (s.v[730] > 0.0);
        s.v[2306] = if s.b[2306] { 1.0 } else { 0.0 };

        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {
            s.store_add_scaled_sub_value_product_indices(2220, 1.0, 2212, 1.0, 2216, 2131, 2.0);
            s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));
            s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2217), 1.0, s.ad_value(2217), 1.0, 1.0);
            s.store_mul_product3_rhs(2221, 730, A::square(s.ad_value(2027)), s.ad_value(2115), s.ad_value(2214), 1.0);
            s.store_add_scaled_inputs_product_right_ad(2222, 2216, 2.0, 2221, (-2.0), 2115, A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2223, 2221, s.ad_value(2221), 1.0, s.ad_value(2216), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2224, 1.0, 2115, A::add(s.ad_value(2212), s.ad_value(2214)), 0.5);
            s.store_div_scaled_product_denominator_ad(2225, 2223, 2222, 1.0, A::add_scaled_square_product(s.ad_value(2222), 1.0, s.ad_value(2224), s.ad_value(2223), (-1.0)), 1.0);
            s.store_add(2211, 2211, 2225);
            s.store_exp(2226, 2225);
            s.store_div(2212, 2212, 2226);
            s.store_mul(2214, 2214, 2226);
            s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);
            s.store_mul_sqrt_ad_rhs(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));
            s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::mul3_scaled_output(s.ad_value(2216), s.ad_value(2217), s.ad_value(2131), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2201, 2201, 2226, A::add(s.ad_value(2220), s.ad_value(2213)), 1.0, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2226), s.ad_value(2213), 1.0), 1.0);
            s.store_mul(2204, 2201, 2129);
        }

        if (s.b[2288] && (!s.b[2304])) {
            s.store_sqrt(2218, 2215);
            s.store_add_scaled_inputs_ad_rhs(2219, 2217, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2212)), s.ad_value(2218)), 0.5);
        }

        if s.b[2288] {
            s.store_mul_div_scaled_product_rhs(2228, 2129, s.ad_value(2115), s.ad_value(2214), 1.0, A::add_scaled_product(s.ad_value(2216), 1.0, s.ad_value(2114), s.ad_value(2218), 1.0), 1.0);
            s.store_add_scaled_product_indices(2229, 2228, 1.0, 2129, 2219, 1.0);
            s.store_mul3_lhs(2230, 2218, 2114, 2129);
        }

        s.b[2307] = (s.v[218] < 0.0);
        s.v[2307] = if s.b[2307] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2307]) {
            s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2228, 1.0);
        }

        if (s.b[2288] && (!s.b[2307])) {
            s.store_div_from_scalar_offset_ad(2169, 1.0, A::mul(s.ad_value(218), s.ad_value(2228)), 1.0);
        }

        if s.b[2288] {
            s.store_mul_product3_rhs(2170, 2228, s.ad_value(757), s.ad_value(2168), s.ad_value(2169), 1.0);
            s.store_add_scaled_product_indices(2231, 2230, 1.0, 775, 2228, 1.0);
            s.store_add_scaled_product_indices(2232, 2230, 1.0, 776, 2228, 1.0);
            s.store_mul(2233, 774, 2231);
            s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2215), 1.0, A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);
            s.store_mul_add_ad_lhs(2234, A::offset(s.ad_value(2172), 1.0), s.ad_value(2170), 2164);
            s.store_ln_ad(2235, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0), 1.0));
            s.store_mul(2029, 2228, 2174);
            s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);
        }

        s.b[2308] = (s.v[222] < 0.0);
        s.v[2308] = if s.b[2308] { 1.0 } else { 0.0 };

        if (s.b[2288] && s.b[2308]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));
        }

        if (s.b[2288] && (!s.b[2308])) {
            s.store_offset_mul(2176, 222, 2175, 1.0);
        }

        if s.b[2288] {
            s.store_mul(2237, 2109, 2176);
            s.store_mul(2236, 2216, 2129);
        }

        s.copy_ad(1855, 2177);

        s.copy_ad(1857, 2195);

        s.copy_ad(1858, 2196);

        s.copy_ad(1859, 2201);

        s.copy_ad(1860, 2204);

        s.copy_ad(1862, 2211);

        s.copy_ad(1861, 2210);

        s.copy_ad(1863, 2217);

        s.copy_ad(1864, 2219);

        s.copy_ad(1865, 2228);

        s.copy_ad(1866, 2229);

        s.copy_ad(1867, 2230);

        s.copy_ad(1868, 2232);

        s.copy_ad(1869, 2234);

        s.copy_ad(1871, 2235);

        s.copy_ad(1870, 2237);

        s.copy_ad(1872, 2236);

        s.copy_ad(1931, 2216);

        s.v[1873] = 1.0;

        s.v[1874] = 1.0;

        s.v[1876] = 1.0;

        s.v[1877] = 1.0;

        s.v[838] = 0.0;

        s.b[2309] = (s.v[1829] > 0.0);
        s.v[2309] = if s.b[2309] { 1.0 } else { 0.0 };

        if s.b[2309] {
            s.store_ln_ad(2037, A::offset(A::mul(s.ad_value(830), s.ad_value(779)), 1.0));
            s.store_div_scaled_product_indices(2027, 1824, 1864, 1.0, 1866, 1.0);
            s.store_add_scaled_product_mixed_aai(2036, A::mul3(A::mul3(s.ad_value(227), s.ad_value(1867), s.ad_value(2027)), s.ad_value(2027), s.ad_value(2037)), 1.0, A::div_scaled_product(A::add(s.ad_value(225), A::div(s.ad_value(226), s.ad_value(1866))), s.ad_value(1865), 1.0, s.ad_value(1866), 1.0), 1871, 1.0);
            s.store_div_from_scalar_add_ad(1873, 1.0, A::offset(s.ad_value(2036), 1.0), A::square(s.ad_value(2036)));
            s.store_mul(1874, 1869, 1873);
            s.store_div(1875, 1870, 1874);
            s.store_mul_ad_product_lhs(2038, A::square(s.ad_value(1875)), s.ad_value(1860), 1860);
        }

        s.b[2310] = (s.v[0] == (-1.0));
        s.v[2310] = if s.b[2310] { 1.0 } else { 0.0 };

        if (s.b[2309] && s.b[2310]) {
            s.store_div_scaled_value_offset_denominator(2038, s.ad_value(2038), 1.0, A::mul(s.ad_value(1875), s.ad_value(1860)), 1.0, 1.0);
        }

        if s.b[2309] {
            s.store_mul_offset_rhs_scaled_ad_rhs(2039, 1874, A::sqrt(A::scale_offset(s.ad_value(2038), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1876, 1.0, 2039);
            s.store_mul(2027, 1874, 1876);
            s.store_mul_offset_ad_rhs(2040, 1864, A::mul3_scaled_output(s.ad_value(2038), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_div_scaled_product_indices(1877, 2027, 1866, 1.0, 2040, 1.0);
            s.store_mul_product3_rhs(838, 1876, s.ad_value(716), s.ad_value(1866), s.ad_value(1860), 1.0);
        }

        s.v[2042] = 0.0;

        s.v[2043] = 0.0;

        s.v[1878] = 0.0;

        s.v[1879] = 0.0;

        s.b[2311] = (((((p.p40 != 0.0) && ((s.v[237] > 0.0) || (s.v[238] > 0.0))) || ((p.p42 != 0.0) && ((s.v[247] > 0.0) || (s.v[248] > 0.0)))) || (s.v[262] > 0.0)) || (s.v[263] > 0.0));
        s.v[2311] = if s.b[2311] { 1.0 } else { 0.0 };

        if s.b[2311] {
            s.store_scaled_add_ad_rhs(2041, 1817, A::sqrt(A::add(A::square(s.ad_value(1817)), s.ad_value(789))), 0.5);
            s.store_add_ad_lhs(2042, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(784), (-0.5), s.ad_value(782), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), 1.0), 791);
            s.store_scaled_add_ad_rhs(2041, 1818, A::sqrt(A::add(A::square(s.ad_value(1818)), s.ad_value(792))), 0.5);
            s.store_add_ad_lhs(2043, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(785), (-0.5), s.ad_value(783), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), 1.0), 794);
            s.store_scaled_add(1878, 1817, 2042, (-s.v[354]));
            s.store_scaled_add(1879, 1818, 2043, (-s.v[354]));
        }

        s.v[839] = 0.0;

        s.v[840] = 0.0;

        s.v[2070] = 0.0;

        s.v[843] = 0.0;

        s.v[841] = 0.0;

        s.v[842] = 0.0;

        s.b[2312] = (p.p40 != 0.0);
        s.v[2312] = if s.b[2312] { 1.0 } else { 0.0 };

        s.b[2313] = (s.v[237] > 0.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1878)), 1e-6), 795);
        }

        s.b[2314] = (s.v[243] < 0.0);
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2313]) && s.b[2314]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2044, 2044, 0.5, 801, 0.5, A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(801)), A::sub(s.ad_value(2044), s.ad_value(801))), 1e-6), (-0.5));
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_offset_ad_rhs(2027, 798, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2315] = (s.v[2027] > 0.0);
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2313]) && s.b[2315]) {
            s.store_offset_mul_offset_rhs_ad_rhs(2045, 2027, A::mul_scaled_output(s.ad_value(2027), A::scale_offset(s.ad_value(2027), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);
        }

        s.b[2316] = (s.v[2027] > (-230.25850929940458));
        s.v[2316] = if s.b[2316] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2313]) && (!s.b[2315])) && s.b[2316]) {
            s.store_exp(2045, 2027);
        }

        if (((s.b[2312] && s.b[2313]) && (!s.b[2315])) && (!s.b[2316])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2045, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_offset(2046, 2042, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
        }

    }
}
