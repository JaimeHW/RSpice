#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1421]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1421])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1422] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1422]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1422])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1423] = (s.v[1216] > 0.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1423]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1424] = (s.v[1215] > (-230.25850929940458));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) && s.b[1424]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) && (!s.b[1424])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);
        }

        s.b[1425] = (p.p845 == 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1425]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1426] = (p.p825 == 0.5);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && s.b[1426]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1426])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(1195), 1.0);
        }

        s.b[1427] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && s.b[1427]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1428] = (((-s.v[436]) / s.v[1220]) < 0.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1427])) && s.b[1428]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1427])) && (!s.b[1428])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845);
        }

        s.b[1429] = (p.p854 > 1000.0);
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1429]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1430] = (s.v[1194] > ((-s.v[438]) * p.p854));
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        s.b[1431] = (p.p857 == 4.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) && s.b[1431]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) && (!s.b[1431])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && (!s.b[1430])) {
            s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1415])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1432] = (s.v[642] == 0.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1432]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1432])) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.b[1433] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1433]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1434] = (p.p826 == 0.5);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && s.b[1434]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && (!s.b[1434])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1435] = (p.p826 == 0.5);
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && s.b[1435]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && (!s.b[1435])) {
            s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_scale(1202, 1195, s.v[419]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[380]);
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.b[1436] = (p.p840 == 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1436]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1437] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1437]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1437])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[431]), s.ad_value(1206), s.ad_value(1209), s.v[431], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1438] = (s.v[1216] > 0.0);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1438]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1438])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1439] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1439]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1439])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1440] = (s.v[1216] > 0.0);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1440]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1441] = (s.v[1215] > (-230.25850929940458));
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) && s.b[1441]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) && (!s.b[1441])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);
        }

        s.b[1442] = (p.p846 == 0.0);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1442]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1443] = (p.p826 == 0.5);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && s.b[1443]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1443])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(1195), 1.0);
        }

        s.b[1444] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && s.b[1444]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1445] = (((-s.v[437]) / s.v[1220]) < 0.0);
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1444])) && s.b[1445]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1444])) && (!s.b[1445])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846);
        }

        s.b[1446] = (p.p855 > 1000.0);
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1446]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1447] = (s.v[1194] > ((-s.v[438]) * p.p855));
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        s.b[1448] = (p.p858 == 4.0);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) && s.b[1448]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) && (!s.b[1448])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && (!s.b[1447])) {
            s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1432])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(472, s.ad_value(640), s.ad_value(1222), 1.0, s.ad_value(641), s.ad_value(1223), 1.0, s.ad_value(642), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1449] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        s.b[1450] = (s.v[483] < s.v[648]);
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        s.b[1451] = (((((-0.5) * (s.v[483] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && s.b[1451]) {
            s.store_exp_scaled_input(1188, 483, (s.v[365] * (-0.5)));
        }

        s.b[1452] = (((-0.5) * (s.v[483] * s.v[365])) < 0.0);
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && (!s.b[1451])) && s.b[1452]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(483), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && (!s.b[1451])) && (!s.b[1452])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(483), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && (!s.b[1450])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(483), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1449]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1453] = (s.v[483] > 0.0);
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1453]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && (!s.b[1453])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 483);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1449]) {
            s.store_sub(1191, 650, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 483, 0.5, 1191, 0.5, 483, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 483, 0.5, 653, 0.5, 483, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 483, 483, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1454] = (s.v[640] == 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1454]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1454])) {
            s.store_scale(1196, 1186, s.v[381]);
        }

        s.b[1455] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1455]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {
            s.store_sub_from_scalar(1198, s.v[387], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1456] = (p.p824 == 0.5);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && s.b[1456]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && (!s.b[1456])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1457] = (p.p824 == 0.5);
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && s.b[1457]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && (!s.b[1457])) {
            s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {
            s.store_scale(1202, 1195, s.v[417]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[378]);
            s.store_scaled_mul(1197, 1203, 1201, p.p833);
        }

        s.b[1458] = (p.p838 == 0.0);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1458]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1459] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1459]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1459])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[429]), s.ad_value(1206), s.ad_value(1209), s.v[429], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1460] = (s.v[1216] > 0.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1460]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1460])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1461] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1461]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1461])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1462] = (s.v[1216] > 0.0);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1462]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1463] = (s.v[1215] > (-230.25850929940458));
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) && s.b[1463]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) && (!s.b[1463])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);
        }

        s.b[1464] = (p.p844 == 0.0);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1464]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1465] = (p.p824 == 0.5);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && s.b[1465]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1465])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(1195), 1.0);
        }

        s.b[1466] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && s.b[1466]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1467] = (((-s.v[435]) / s.v[1220]) < 0.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1466])) && s.b[1467]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1466])) && (!s.b[1467])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844);
        }

        s.b[1468] = (p.p853 > 1000.0);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1468]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1469] = (s.v[1194] > ((-s.v[438]) * p.p853));
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        s.b[1470] = (p.p856 == 4.0);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) && s.b[1470]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) && (!s.b[1470])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && (!s.b[1469])) {
            s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1454])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1471] = (s.v[641] == 0.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1471]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1471])) {
            s.store_scale(1196, 1186, s.v[382]);
        }

        s.b[1472] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1472]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {
            s.store_sub_from_scalar(1198, s.v[388], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1473] = (p.p825 == 0.5);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1474] = (p.p825 == 0.5);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && s.b[1474]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1474])) {
            s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {
            s.store_scale(1202, 1195, s.v[418]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[379]);
            s.store_scaled_mul(1197, 1203, 1201, p.p834);
        }

        s.b[1475] = (p.p839 == 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1475]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1476] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1476]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1476])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[430]), s.ad_value(1206), s.ad_value(1209), s.v[430], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1477] = (s.v[1216] > 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1477]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1477])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1478] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1478]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1478])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1479] = (s.v[1216] > 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1479]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1480] = (s.v[1215] > (-230.25850929940458));
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) && s.b[1480]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) && (!s.b[1480])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);
        }

        s.b[1481] = (p.p845 == 0.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1481]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1482] = (p.p825 == 0.5);
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && s.b[1482]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1482])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(1195), 1.0);
        }

        s.b[1483] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && s.b[1483]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1484] = (((-s.v[436]) / s.v[1220]) < 0.0);
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1483])) && s.b[1484]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1483])) && (!s.b[1484])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845);
        }

        s.b[1485] = (p.p854 > 1000.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1485]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1486] = (s.v[1194] > ((-s.v[438]) * p.p854));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        s.b[1487] = (p.p857 == 4.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) && s.b[1487]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) && (!s.b[1487])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && (!s.b[1486])) {
            s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1471])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1488] = (s.v[642] == 0.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1488]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1488])) {
            s.store_scale(1196, 1186, s.v[383]);
        }

        s.b[1489] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1489]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_sub_from_scalar(1198, s.v[389], 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1490] = (p.p826 == 0.5);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && s.b[1490]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && (!s.b[1490])) {
            s.store_scaled_add_ad_lhs(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1491] = (p.p826 == 0.5);
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && s.b[1491]) {
            s.store_sqrt_scaled_input(1195, 1198, s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && (!s.b[1491])) {
            s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_scale(1202, 1195, s.v[419]);
            s.store_mul_offset_lhs_scaled_output(1203, 1189, (-1.0), 1202, s.v[380]);
            s.store_scaled_mul(1197, 1203, 1201, p.p835);
        }

        s.b[1492] = (p.p840 == 0.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1492]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {
            s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);
            s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1493] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1493]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1493])) {
            s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, s.ad_value(1208), (-s.v[431]), s.ad_value(1206), s.ad_value(1209), s.v[431], s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1494] = (s.v[1216] > 0.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1494]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1494])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1495] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1495]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1495])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1496] = (s.v[1216] > 0.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1496]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1497] = (s.v[1215] > (-230.25850929940458));
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) && s.b[1497]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) && (!s.b[1497])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {
            s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);
            s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);
        }

        s.b[1498] = (p.p846 == 0.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1498]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1499] = (p.p826 == 0.5);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && s.b[1499]) {
            s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1499])) {
            s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) {
            s.store_div_scaled_offset_numerator(1220, s.ad_value(1193), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(1195), 1.0);
        }

        s.b[1500] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && s.b[1500]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1501] = (((-s.v[437]) / s.v[1220]) < 0.0);
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1500])) && s.b[1501]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1500])) && (!s.b[1501])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) {
            s.store_mul_scaled_ad_lhs(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846);
        }

        s.b[1502] = (p.p855 > 1000.0);
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1502]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1503] = (s.v[1194] > ((-s.v[438]) * p.p855));
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        s.b[1504] = (p.p858 == 4.0);
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) && s.b[1504]) {
            s.store_mul_scaled_ad_lhs(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444]);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) && (!s.b[1504])) {
            s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && (!s.b[1503])) {
            s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1488])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(473, s.ad_value(640), s.ad_value(1222), 1.0, s.ad_value(641), s.ad_value(1223), 1.0, s.ad_value(642), s.ad_value(1224), 1.0);
            s.store_add_scaled_inputs3_indices(661, 640, s.v[381], 641, s.v[382], 642, s.v[383]);
            s.store_add_scaled_offset_product_rhs_mixed_iia(477, 472, 1.0, 661, A::exp_scaled_input(s.ad_value(482), (s.v[365] * s.v[662])), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_iia(478, 473, 1.0, 661, A::exp_scaled_input(s.ad_value(483), (s.v[365] * s.v[662])), (-1.0), (-1.0));
        }

        s.b[1505] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = ((s.v[472] > 0.0) && (s.v[473] > 0.0));
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        s.b[1507] = ((((((s.v[477] / s.v[472]) > 0.001) || ((s.v[478] / s.v[473]) > 0.001)) && (s.v[477] > 0.0)) && (s.v[478] > 0.0)) && (s.v[478] > s.v[477]));
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1506]) && s.b[1507]) {
            s.store_div(484, 477, 478);
            s.store_div_scaled_inputs(664, A::ln(s.ad_value(484)), s.v[364], A::sub(s.ad_value(482), s.ad_value(483)), 1.0);
            s.store_div_scaled_value_offset_denominator(663, s.ad_value(477), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(482), s.v[365], s.ad_value(664))), (-1.0), 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1505]) {
            s.store_add_scaled_offset_product_rhs_mixed_aia(474, A::add_scaled_offset_product_rhs(s.ad_value(469), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(479), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(479), s.v[365], s.ad_value(664))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(475, A::add_scaled_offset_product_rhs(s.ad_value(470), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(480), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(480), s.v[365], s.ad_value(664))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(476, A::add_scaled_offset_product_rhs(s.ad_value(471), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(481), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(481), s.v[365], s.ad_value(664))), (-1.0), (-1.0));
        }

        s.b[1508] = (((s.v[469] < 0.0) && (s.v[470] < 0.0)) && (s.v[471] < 0.0));
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        s.b[1509] = (((((((s.v[474] / s.v[469]) > 0.001) || ((s.v[475] / s.v[470]) > 0.001)) || ((s.v[476] / s.v[471]) > 0.001)) && (s.v[474] < 0.0)) && (s.v[475] < 0.0)) && (s.v[476] < 0.0));
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) {
            s.store_div(484, 474, 475);
            s.store_div_scaled_inputs(485, A::ln(s.ad_value(484)), (-s.v[364]), A::sub(s.ad_value(479), s.ad_value(480)), 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(487, 480, 480, 1.0, 479, -1.0);
            s.store_scaled_mul_ad(488, A::offset(s.ad_value(484), (-1.0)), A::offset(A::pow(s.ad_value(484), s.ad_value(487)), (-1.0)), s.v[364]);
            s.store_div_add_scaled_inputs_rhs_indices(487, 479, 479, 1.0, 480, -1.0);
            s.store_sub_ad_lhs(489, A::add_scaled_products(A::pow(s.ad_value(484), s.ad_value(487)), A::sub(s.ad_value(480), s.ad_value(479)), 1.0, s.ad_value(484), s.ad_value(479), 1.0), 480);
            s.store_div(486, 488, 489);
            s.store_add(666, 485, 486);
        }

        s.b[1510] = (((((s.v[481] * s.v[365]) * s.v[666])) as f64).abs() < 1e-6);
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        let (assign27540_e32808,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && s.b[1510]) {
        (1.0,)
    } else {
        (s.v[660],)
    }
};
        s.v[660] = assign27540_e32808;

        if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && s.b[1510]) {
            s.store_mul_add_scaled_inputs_rhs(665, 476, A::div_from_scalar(1.0, s.ad_value(481)), 1.0, s.ad_value(666), (0.5 * s.v[365]));
            s.store_div_scaled_product_indices(666, 476, 666, ((-0.5) * s.v[365]), 481, 1.0);
        }

        let (assign27570_e32870,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {
        (0.0,)
    } else {
        (s.v[660],)
    }
};
        s.v[660] = assign27570_e32870;

        if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {
            s.store_div_scaled_value_offset_denominator(665, s.ad_value(476), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(481), (-s.v[365]), s.ad_value(666))), (-1.0), 1.0);
        }

        let (assign27590_e32914,) = {
    if (s.b[1159] && s.b[1176]) {
        let assign27590_e32903: f64 = (s.v[640] * s.v[408]);
        let assign27590_e32906: f64 = (s.v[641] * s.v[409]);
        let assign27590_e32907: f64 = (assign27590_e32903 + assign27590_e32906);
        let assign27590_e32910: f64 = (s.v[642] * s.v[410]);
        let assign27590_e32911: f64 = (assign27590_e32907 + assign27590_e32910);
        let assign27590_e32912: f64 = (p.p922 * assign27590_e32911);
        (assign27590_e32912,)
    } else {
        (s.v[495],)
    }
};
        s.v[495] = assign27590_e32914;

        s.b[1511] = ((s.v[640] * s.v[408]) <= s.v[495]);
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        let (assign27610_e32927,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1511]) {
        (0.0,)
    } else {
        (s.v[645],)
    }
};
        s.v[645] = assign27610_e32927;

        s.b[1512] = ((s.v[641] * s.v[409]) <= s.v[495]);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        let (assign27630_e32940,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1512]) {
        (0.0,)
    } else {
        (s.v[646],)
    }
};
        s.v[646] = assign27630_e32940;

        s.b[1513] = ((s.v[642] * s.v[410]) <= s.v[495]);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        let (assign27650_e32953,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1513]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.v[647] = assign27650_e32953;

        s.b[1514] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1514]) {
            s.store_ln_ad(654, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(661), 1e-21, 1.0));
            s.store_ln_ad(656, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(663), 1e-21, 1.0));
            s.store_ln_ad(658, A::div_scalar_offset_denominator((0.5 * p.p815), A::abs(s.ad_value(665)), 1e-21, 1.0));
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_min_with_scalar(654, 654, 230.25850929940458);
            s.store_exp(655, 654);
            s.store_min_with_scalar(656, 656, 230.25850929940458);
            s.store_exp(657, 656);
            s.store_min_with_scalar(658, 658, 230.25850929940458);
            s.store_exp(659, 658);
            s.store_scalar(492, 0.4);
            s.store_scalar(493, 0.65);
            s.store_scalar(494, 0.8);
            s.store_mul_neg_lhs(479, 492, 546);
            s.store_mul_neg_lhs(480, 493, 546);
            s.store_mul_neg_lhs(481, 494, 546);
            s.store_scalar(482, 0.1);
            s.store_scalar(483, 0.2);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1515] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        s.b[1516] = (s.v[479] < s.v[675]);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        s.b[1517] = (((((-0.5) * (s.v[479] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && s.b[1517]) {
            s.store_exp_scaled_input(1188, 479, (s.v[365] * (-0.5)));
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1518] = (((-0.5) * (s.v[479] * s.v[365])) < 0.0);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && (!s.b[1517])) && s.b[1518]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && (!s.b[1517])) && (!s.b[1518])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(479), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && (!s.b[1516])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(479), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1515]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1519] = (s.v[479] > 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1519]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && (!s.b[1519])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 479);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1515]) {
            s.store_sub(1191, 677, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 479, 0.5, 1191, 0.5, 479, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 479, 0.5, 680, 0.5, 479, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 479, 479, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1520] = (s.v[667] == 0.0);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1520]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1520])) {
            s.store_mul(1196, 557, 1186);
        }

        s.b[1521] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1521]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {
            s.store_sub(1198, 563, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1522] = (s.v[505] == 0.5);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && s.b[1522]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && (!s.b[1522])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(505), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1523] = (s.v[505] == 0.5);
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && s.b[1523]) {
            s.store_sqrt_mul(1195, 1198, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && (!s.b[1523])) {
            s.store_pow_mul_base_indices(1195, 1198, 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {
            s.store_mul(1202, 584, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 516, 1203, 1201);
        }

        s.b[1524] = (s.v[519] == 0.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1524]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {
            s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1525] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1525]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1525])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(596), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1526] = (s.v[1216] > 0.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1526]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1526])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1527] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1527]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1527])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1528] = (s.v[1216] > 0.0);
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1528]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1529] = (s.v[1215] > (-230.25850929940458));
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) && s.b[1529]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) && (!s.b[1529])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {
            s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);
        }

        s.b[1530] = (s.v[525] == 0.0);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1530]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1531] = (s.v[505] == 0.5);
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && s.b[1531]) {
            s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1531])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);
        }

        s.b[1532] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && s.b[1532]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1533] = (((-s.v[602]) / s.v[1220]) < 0.0);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1532])) && s.b[1533]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1532])) && (!s.b[1533])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1534] = (s.v[534] > 1000.0);
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1534]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1535] = (s.v[1194] > ((-s.v[438]) * s.v[534]));
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        s.b[1536] = (s.v[537] == 4.0);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) && s.b[1536]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) && (!s.b[1536])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && (!s.b[1535])) {
            s.store_add_scaled_product_left_ad(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1520])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1537] = (s.v[668] == 0.0);
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1537]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1537])) {
            s.store_mul(1196, 558, 1186);
        }

        s.b[1538] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1538]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {
            s.store_sub(1198, 564, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1539] = (s.v[506] == 0.5);
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && s.b[1539]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && (!s.b[1539])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(506), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1540] = (s.v[506] == 0.5);
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && s.b[1540]) {
            s.store_sqrt_mul(1195, 1198, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && (!s.b[1540])) {
            s.store_pow_mul_base_indices(1195, 1198, 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {
            s.store_mul(1202, 585, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 517, 1203, 1201);
        }

        s.b[1541] = (s.v[520] == 0.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1541]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {
            s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1542] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1542]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1542])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(597), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1543] = (s.v[1216] > 0.0);
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1543]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1543])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1544] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1544]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1544])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1545] = (s.v[1216] > 0.0);
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1545]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1546] = (s.v[1215] > (-230.25850929940458));
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) && s.b[1546]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) && (!s.b[1546])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {
            s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1547] = (s.v[526] == 0.0);
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1547]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1548] = (s.v[506] == 0.5);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && s.b[1548]) {
            s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1548])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);
        }

        s.b[1549] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && s.b[1549]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1550] = (((-s.v[603]) / s.v[1220]) < 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1549])) && s.b[1550]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1549])) && (!s.b[1550])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1551] = (s.v[535] > 1000.0);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1551]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1552] = (s.v[1194] > ((-s.v[438]) * s.v[535]));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        s.b[1553] = (s.v[538] == 4.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) && s.b[1553]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) && (!s.b[1553])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && (!s.b[1552])) {
            s.store_add_scaled_product_left_ad(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1537])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1554] = (s.v[669] == 0.0);
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1554]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1554])) {
            s.store_mul(1196, 559, 1186);
        }

        s.b[1555] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1555]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {
            s.store_sub(1198, 565, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1556] = (s.v[507] == 0.5);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && s.b[1556]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && (!s.b[1556])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(507), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1557] = (s.v[507] == 0.5);
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && s.b[1557]) {
            s.store_sqrt_mul(1195, 1198, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && (!s.b[1557])) {
            s.store_pow_mul_base_indices(1195, 1198, 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {
            s.store_mul(1202, 586, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 518, 1203, 1201);
        }

        s.b[1558] = (s.v[521] == 0.0);
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1558]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {
            s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1559] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1559]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1559])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(598), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1560] = (s.v[1216] > 0.0);
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1560]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1560])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1561] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1561]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1561])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1562] = (s.v[1216] > 0.0);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1562]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1563] = (s.v[1215] > (-230.25850929940458));
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) && s.b[1563]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) && (!s.b[1563])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {
            s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);
        }

        s.b[1564] = (s.v[527] == 0.0);
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1564]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1565] = (s.v[507] == 0.5);
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && s.b[1565]) {
            s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1565])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);
        }

        s.b[1566] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && s.b[1566]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1567] = (((-s.v[604]) / s.v[1220]) < 0.0);
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1566])) && s.b[1567]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1566])) && (!s.b[1567])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1568] = (s.v[536] > 1000.0);
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1568]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1569] = (s.v[1194] > ((-s.v[438]) * s.v[536]));
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        s.b[1570] = (s.v[539] == 4.0);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) && s.b[1570]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) && (!s.b[1570])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && (!s.b[1569])) {
            s.store_add_scaled_product_left_ad(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1554])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(469, s.ad_value(667), s.ad_value(1222), 1.0, s.ad_value(668), s.ad_value(1223), 1.0, s.ad_value(669), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1571] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        s.b[1572] = (s.v[480] < s.v[675]);
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        s.b[1573] = (((((-0.5) * (s.v[480] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && s.b[1573]) {
            s.store_exp_scaled_input(1188, 480, (s.v[365] * (-0.5)));
        }

        s.b[1574] = (((-0.5) * (s.v[480] * s.v[365])) < 0.0);
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && (!s.b[1573])) && s.b[1574]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && (!s.b[1573])) && (!s.b[1574])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(480), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && (!s.b[1572])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(480), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1571]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1575] = (s.v[480] > 0.0);
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1575]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && (!s.b[1575])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 480);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1571]) {
            s.store_sub(1191, 677, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 480, 0.5, 1191, 0.5, 480, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 480, 0.5, 680, 0.5, 480, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 480, 480, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1576] = (s.v[667] == 0.0);
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1576]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1576])) {
            s.store_mul(1196, 557, 1186);
        }

        s.b[1577] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1577]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_sub(1198, 563, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1578] = (s.v[505] == 0.5);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && s.b[1578]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && (!s.b[1578])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(505), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1579] = (s.v[505] == 0.5);
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && s.b[1579]) {
            s.store_sqrt_mul(1195, 1198, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && (!s.b[1579])) {
            s.store_pow_mul_base_indices(1195, 1198, 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_mul(1202, 584, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 516, 1203, 1201);
        }

        s.b[1580] = (s.v[519] == 0.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1580]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1581] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1581]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1581])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(596), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1582] = (s.v[1216] > 0.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1582]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1582])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1583] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1583]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1583])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1584] = (s.v[1216] > 0.0);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1584]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1585] = (s.v[1215] > (-230.25850929940458));
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) && s.b[1585]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) && (!s.b[1585])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);
        }

        s.b[1586] = (s.v[525] == 0.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1586]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1587] = (s.v[505] == 0.5);
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && s.b[1587]) {
            s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1587])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);
        }

        s.b[1588] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && s.b[1588]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1589] = (((-s.v[602]) / s.v[1220]) < 0.0);
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1588])) && s.b[1589]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1588])) && (!s.b[1589])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1590] = (s.v[534] > 1000.0);
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1590]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1591] = (s.v[1194] > ((-s.v[438]) * s.v[534]));
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        s.b[1592] = (s.v[537] == 4.0);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) && s.b[1592]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) && (!s.b[1592])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && (!s.b[1591])) {
            s.store_add_scaled_product_left_ad(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1576])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1593] = (s.v[668] == 0.0);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1593]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1593])) {
            s.store_mul(1196, 558, 1186);
        }

        s.b[1594] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1594]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {
            s.store_sub(1198, 564, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1595] = (s.v[506] == 0.5);
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && s.b[1595]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && (!s.b[1595])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(506), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1596] = (s.v[506] == 0.5);
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && s.b[1596]) {
            s.store_sqrt_mul(1195, 1198, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && (!s.b[1596])) {
            s.store_pow_mul_base_indices(1195, 1198, 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {
            s.store_mul(1202, 585, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 517, 1203, 1201);
        }

        s.b[1597] = (s.v[520] == 0.0);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1597]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {
            s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1598] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1598]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1598])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(597), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1599] = (s.v[1216] > 0.0);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1599]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1599])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1600] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1600]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1600])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1601] = (s.v[1216] > 0.0);
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1601]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1602] = (s.v[1215] > (-230.25850929940458));
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) && s.b[1602]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) && (!s.b[1602])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {
            s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);
        }

        s.b[1603] = (s.v[526] == 0.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1603]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1604] = (s.v[506] == 0.5);
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && s.b[1604]) {
            s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);
        }

        s.b[1605] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && s.b[1605]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1606] = (((-s.v[603]) / s.v[1220]) < 0.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1605])) && s.b[1606]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1605])) && (!s.b[1606])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1607] = (s.v[535] > 1000.0);
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1607]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1608] = (s.v[1194] > ((-s.v[438]) * s.v[535]));
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        s.b[1609] = (s.v[538] == 4.0);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) && s.b[1609]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) && (!s.b[1609])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_add_scaled_product_left_ad(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1593])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1610] = (s.v[669] == 0.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1610]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1610])) {
            s.store_mul(1196, 559, 1186);
        }

        s.b[1611] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1611]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {
            s.store_sub(1198, 565, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1612] = (s.v[507] == 0.5);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && (!s.b[1612])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(507), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1613] = (s.v[507] == 0.5);
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && s.b[1613]) {
            s.store_sqrt_mul(1195, 1198, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && (!s.b[1613])) {
            s.store_pow_mul_base_indices(1195, 1198, 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {
            s.store_mul(1202, 586, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 518, 1203, 1201);
        }

        s.b[1614] = (s.v[521] == 0.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1614]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {
            s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1615] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1615]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1615])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(598), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1616] = (s.v[1216] > 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1616]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1616])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1617] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1617]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1617])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1618] = (s.v[1216] > 0.0);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1618]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1619] = (s.v[1215] > (-230.25850929940458));
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) && s.b[1619]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) && (!s.b[1619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {
            s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);
        }

        s.b[1620] = (s.v[527] == 0.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1620]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1621] = (s.v[507] == 0.5);
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && s.b[1621]) {
            s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1621])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);
        }

        s.b[1622] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && s.b[1622]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1623] = (((-s.v[604]) / s.v[1220]) < 0.0);
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1622])) && s.b[1623]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1622])) && (!s.b[1623])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1624] = (s.v[536] > 1000.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1624]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1625] = (s.v[1194] > ((-s.v[438]) * s.v[536]));
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        s.b[1626] = (s.v[539] == 4.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) && s.b[1626]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) && (!s.b[1626])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && (!s.b[1625])) {
            s.store_add_scaled_product_left_ad(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1610])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(470, s.ad_value(667), s.ad_value(1222), 1.0, s.ad_value(668), s.ad_value(1223), 1.0, s.ad_value(669), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1627] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        s.b[1628] = (s.v[481] < s.v[675]);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        s.b[1629] = (((((-0.5) * (s.v[481] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && s.b[1629]) {
            s.store_exp_scaled_input(1188, 481, (s.v[365] * (-0.5)));
        }

        s.b[1630] = (((-0.5) * (s.v[481] * s.v[365])) < 0.0);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && s.b[1630]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(481), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && (!s.b[1628])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(481), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1627]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1631] = (s.v[481] > 0.0);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1631]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && (!s.b[1631])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 481);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1627]) {
            s.store_sub(1191, 677, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 481, 0.5, 1191, 0.5, 481, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 481, 0.5, 680, 0.5, 481, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 481, 481, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1632] = (s.v[667] == 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1632]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1632])) {
            s.store_mul(1196, 557, 1186);
        }

        s.b[1633] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1633]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {
            s.store_sub(1198, 563, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1634] = (s.v[505] == 0.5);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && s.b[1634]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1634])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(505), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1635] = (s.v[505] == 0.5);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && s.b[1635]) {
            s.store_sqrt_mul(1195, 1198, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) && (!s.b[1635])) {
            s.store_pow_mul_base_indices(1195, 1198, 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1633])) {
            s.store_mul(1202, 584, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 516, 1203, 1201);
        }

        s.b[1636] = (s.v[519] == 0.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1636]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {
            s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1637] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1637]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1637])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(596), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1638] = (s.v[1216] > 0.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1638]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1638])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1639] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1639]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1639])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1640] = (s.v[1216] > 0.0);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && s.b[1640]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1641] = (s.v[1215] > (-230.25850929940458));
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) && s.b[1641]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) && (!s.b[1641])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1640])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1636])) {
            s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);
        }

        s.b[1642] = (s.v[525] == 0.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1642]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1643] = (s.v[505] == 0.5);
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && s.b[1643]) {
            s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1643])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);
        }

        s.b[1644] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && s.b[1644]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1645] = (((-s.v[602]) / s.v[1220]) < 0.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1644])) && s.b[1645]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) && (!s.b[1644])) && (!s.b[1645])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1642])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1646] = (s.v[534] > 1000.0);
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1632])) && s.b[1646]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1647] = (s.v[1194] > ((-s.v[438]) * s.v[534]));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        s.b[1648] = (s.v[537] == 4.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) && s.b[1648]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) && (!s.b[1648])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && s.b[1647]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1632])) && (!s.b[1646])) && (!s.b[1647])) {
            s.store_add_scaled_product_left_ad(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1632])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1649] = (s.v[668] == 0.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1649]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1649])) {
            s.store_mul(1196, 558, 1186);
        }

        s.b[1650] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1650]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {
            s.store_sub(1198, 564, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1651] = (s.v[506] == 0.5);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1651]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1651])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(506), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1652] = (s.v[506] == 0.5);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && s.b[1652]) {
            s.store_sqrt_mul(1195, 1198, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) && (!s.b[1652])) {
            s.store_pow_mul_base_indices(1195, 1198, 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1650])) {
            s.store_mul(1202, 585, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 517, 1203, 1201);
        }

        s.b[1653] = (s.v[520] == 0.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1653]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {
            s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1654] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1654]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1654])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(597), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1655] = (s.v[1216] > 0.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1655]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1655])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1656] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1656]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1656])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1657] = (s.v[1216] > 0.0);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && s.b[1657]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1658] = (s.v[1215] > (-230.25850929940458));
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) && s.b[1658]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) && (!s.b[1658])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1657])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1653])) {
            s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);
        }

        s.b[1659] = (s.v[526] == 0.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1659]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1660] = (s.v[506] == 0.5);
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && s.b[1660]) {
            s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1660])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);
        }

        s.b[1661] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && s.b[1661]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1662] = (((-s.v[603]) / s.v[1220]) < 0.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1661])) && s.b[1662]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) && (!s.b[1661])) && (!s.b[1662])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1659])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1663] = (s.v[535] > 1000.0);
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1649])) && s.b[1663]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1664] = (s.v[1194] > ((-s.v[438]) * s.v[535]));
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        s.b[1665] = (s.v[538] == 4.0);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) && s.b[1665]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) && (!s.b[1665])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && s.b[1664]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1649])) && (!s.b[1663])) && (!s.b[1664])) {
            s.store_add_scaled_product_left_ad(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1649])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1666] = (s.v[669] == 0.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1666]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1666])) {
            s.store_mul(1196, 559, 1186);
        }

        s.b[1667] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1667]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {
            s.store_sub(1198, 565, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1668] = (s.v[507] == 0.5);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1668]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1668])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(507), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1669] = (s.v[507] == 0.5);
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && s.b[1669]) {
            s.store_sqrt_mul(1195, 1198, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) && (!s.b[1669])) {
            s.store_pow_mul_base_indices(1195, 1198, 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1667])) {
            s.store_mul(1202, 586, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 518, 1203, 1201);
        }

        s.b[1670] = (s.v[521] == 0.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1670]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {
            s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1671] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1671]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1671])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(598), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1672] = (s.v[1216] > 0.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1672]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1672])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1673] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1673]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1673])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1674] = (s.v[1216] > 0.0);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && s.b[1674]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1675] = (s.v[1215] > (-230.25850929940458));
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) && s.b[1675]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) && (!s.b[1675])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1674])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1670])) {
            s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);
        }

        s.b[1676] = (s.v[527] == 0.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1676]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1677] = (s.v[507] == 0.5);
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && s.b[1677]) {
            s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1677])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);
        }

        s.b[1678] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && s.b[1678]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1679] = (((-s.v[604]) / s.v[1220]) < 0.0);
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1678])) && s.b[1679]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) && (!s.b[1678])) && (!s.b[1679])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1676])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1680] = (s.v[536] > 1000.0);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1666])) && s.b[1680]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1681] = (s.v[1194] > ((-s.v[438]) * s.v[536]));
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        s.b[1682] = (s.v[539] == 4.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) && s.b[1682]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) && (!s.b[1682])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && s.b[1681]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1666])) && (!s.b[1680])) && (!s.b[1681])) {
            s.store_add_scaled_product_left_ad(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1666])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(471, s.ad_value(667), s.ad_value(1222), 1.0, s.ad_value(668), s.ad_value(1223), 1.0, s.ad_value(669), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1683] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        s.b[1684] = (s.v[482] < s.v[675]);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        s.b[1685] = (((((-0.5) * (s.v[482] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && s.b[1685]) {
            s.store_exp_scaled_input(1188, 482, (s.v[365] * (-0.5)));
        }

        s.b[1686] = (((-0.5) * (s.v[482] * s.v[365])) < 0.0);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && s.b[1686]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) && (!s.b[1685])) && (!s.b[1686])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(482), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1684]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && (!s.b[1684])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(482), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1683]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1687] = (s.v[482] > 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && s.b[1687]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1683]) && (!s.b[1687])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 482);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1683]) {
            s.store_sub(1191, 677, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 482, 0.5, 1191, 0.5, 482, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 482, 0.5, 680, 0.5, 482, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 482, 482, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1688] = (s.v[667] == 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1688]) {
            s.store_scalar(1222, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1688])) {
            s.store_mul(1196, 557, 1186);
        }

        s.b[1689] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1689]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_sub(1198, 563, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1690] = (s.v[505] == 0.5);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && s.b[1690]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && (!s.b[1690])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(505), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1691] = (s.v[505] == 0.5);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && s.b[1691]) {
            s.store_sqrt_mul(1195, 1198, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) && (!s.b[1691])) {
            s.store_pow_mul_base_indices(1195, 1198, 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_mul(1202, 584, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 516, 1203, 1201);
        }

        s.b[1692] = (s.v[519] == 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1692]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {
            s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1693] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1693]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1693])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(596), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1694] = (s.v[1216] > 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1694]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1694])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1695] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1695]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1695])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1696] = (s.v[1216] > 0.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && s.b[1696]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1697] = (s.v[1215] > (-230.25850929940458));
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) && s.b[1697]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) && (!s.b[1697])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1696])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1692])) {
            s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);
        }

        s.b[1698] = (s.v[525] == 0.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1698]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1699] = (s.v[505] == 0.5);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && s.b[1699]) {
            s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1699])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);
        }

        s.b[1700] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && s.b[1700]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1701] = (((-s.v[602]) / s.v[1220]) < 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1700])) && s.b[1701]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) && (!s.b[1700])) && (!s.b[1701])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1698])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1702] = (s.v[534] > 1000.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1688])) && s.b[1702]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1703] = (s.v[1194] > ((-s.v[438]) * s.v[534]));
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        s.b[1704] = (s.v[537] == 4.0);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) && s.b[1704]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) && (!s.b[1704])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && s.b[1703]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1688])) && (!s.b[1702])) && (!s.b[1703])) {
            s.store_add_scaled_product_left_ad(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1688])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1705] = (s.v[668] == 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1705]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1705])) {
            s.store_mul(1196, 558, 1186);
        }

        s.b[1706] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1706]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {
            s.store_sub(1198, 564, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1707] = (s.v[506] == 0.5);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1707]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1707])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(506), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1708] = (s.v[506] == 0.5);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && s.b[1708]) {
            s.store_sqrt_mul(1195, 1198, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) && (!s.b[1708])) {
            s.store_pow_mul_base_indices(1195, 1198, 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1706])) {
            s.store_mul(1202, 585, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 517, 1203, 1201);
        }

        s.b[1709] = (s.v[520] == 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1709]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1710] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1710]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1710])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(597), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1711] = (s.v[1216] > 0.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1711]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1711])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1712] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1712]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1712])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1713] = (s.v[1216] > 0.0);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && s.b[1713]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1714] = (s.v[1215] > (-230.25850929940458));
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) && s.b[1714]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) && (!s.b[1714])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1713])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);
        }

        s.b[1715] = (s.v[526] == 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1715]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1716] = (s.v[506] == 0.5);
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && s.b[1716]) {
            s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1716])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);
        }

        s.b[1717] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && s.b[1717]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1718] = (((-s.v[603]) / s.v[1220]) < 0.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1717])) && s.b[1718]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) && (!s.b[1717])) && (!s.b[1718])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1715])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1719] = (s.v[535] > 1000.0);
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1705])) && s.b[1719]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1720] = (s.v[1194] > ((-s.v[438]) * s.v[535]));
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        s.b[1721] = (s.v[538] == 4.0);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) && s.b[1721]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) && (!s.b[1721])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && s.b[1720]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1705])) && (!s.b[1719])) && (!s.b[1720])) {
            s.store_add_scaled_product_left_ad(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1705])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1722] = (s.v[669] == 0.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1722]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1722])) {
            s.store_mul(1196, 559, 1186);
        }

        s.b[1723] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1723]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {
            s.store_sub(1198, 565, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1724] = (s.v[507] == 0.5);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && (!s.b[1724])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(507), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1725] = (s.v[507] == 0.5);
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && s.b[1725]) {
            s.store_sqrt_mul(1195, 1198, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) && (!s.b[1725])) {
            s.store_pow_mul_base_indices(1195, 1198, 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1723])) {
            s.store_mul(1202, 586, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 518, 1203, 1201);
        }

        s.b[1726] = (s.v[521] == 0.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1726]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {
            s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1727] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1727]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1727])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(598), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1728] = (s.v[1216] > 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1728]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1728])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1729] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1729]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1729])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1730] = (s.v[1216] > 0.0);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && s.b[1730]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1731] = (s.v[1215] > (-230.25850929940458));
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) && s.b[1731]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) && (!s.b[1731])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1730])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1726])) {
            s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);
        }

        s.b[1732] = (s.v[527] == 0.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1732]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1733] = (s.v[507] == 0.5);
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && s.b[1733]) {
            s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1733])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);
        }

        s.b[1734] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && s.b[1734]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1735] = (((-s.v[604]) / s.v[1220]) < 0.0);
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1734])) && s.b[1735]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) && (!s.b[1734])) && (!s.b[1735])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1732])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1736] = (s.v[536] > 1000.0);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1722])) && s.b[1736]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1737] = (s.v[1194] > ((-s.v[438]) * s.v[536]));
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        s.b[1738] = (s.v[539] == 4.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) && s.b[1738]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) && (!s.b[1738])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && s.b[1737]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1722])) && (!s.b[1736])) && (!s.b[1737])) {
            s.store_add_scaled_product_left_ad(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1722])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(472, s.ad_value(667), s.ad_value(1222), 1.0, s.ad_value(668), s.ad_value(1223), 1.0, s.ad_value(669), s.ad_value(1224), 1.0);
            s.store_scalar(1193, 0.0);
            s.store_scalar(1190, 0.0);
        }

        s.b[1739] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        s.b[1740] = (s.v[483] < s.v[675]);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        s.b[1741] = (((((-0.5) * (s.v[483] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && s.b[1741]) {
            s.store_exp_scaled_input(1188, 483, (s.v[365] * (-0.5)));
        }

        s.b[1742] = (((-0.5) * (s.v[483] * s.v[365])) < 0.0);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && s.b[1742]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(483), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) && (!s.b[1741])) && (!s.b[1742])) {
            s.store_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(483), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1740]) {
            s.store_div_from_scalar(1189, 1.0, 1188);
            s.store_square(1186, 1189);
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && (!s.b[1740])) {
            s.store_mul_offset_ad_lhs(1186, A::sub_scaled_inputs(s.ad_value(483), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(1189, 1186);
            s.store_div_from_scalar(1188, 1.0, 1189);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1739]) {
            s.store_offset(1186, 1186, (-1.0));
        }

        s.b[1743] = (s.v[483] > 0.0);
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && s.b[1743]) {
            s.store_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && s.b[1739]) && (!s.b[1743])) {
            s.store_sub_ad_lhs(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 483);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1739]) {
            s.store_sub(1191, 677, 1190);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 483, 0.5, 1191, 0.5, 483, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 483, 0.5, 680, 0.5, 483, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(1194, 483, 483, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[1744] = (s.v[667] == 0.0);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1744]) {
            s.store_scalar(1222, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1159] && s.b[1176]) && (!s.b[1744])) {
            s.store_mul(1196, 557, 1186);
        }

        s.b[1745] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1745]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {
            s.store_sub(1198, 563, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1746] = (s.v[505] == 0.5);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && s.b[1746]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && (!s.b[1746])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(505), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1747] = (s.v[505] == 0.5);
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && s.b[1747]) {
            s.store_sqrt_mul(1195, 1198, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) && (!s.b[1747])) {
            s.store_pow_mul_base_indices(1195, 1198, 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1745])) {
            s.store_mul(1202, 584, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 516, 1203, 1201);
        }

        s.b[1748] = (s.v[519] == 0.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1748]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {
            s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1749] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1749]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1749])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(596), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1750] = (s.v[1216] > 0.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1750]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1750])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1751] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1751]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1751])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1752] = (s.v[1216] > 0.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && s.b[1752]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1753] = (s.v[1215] > (-230.25850929940458));
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) && s.b[1753]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) && (!s.b[1753])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1752])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1748])) {
            s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);
        }

        s.b[1754] = (s.v[525] == 0.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1754]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1755] = (s.v[505] == 0.5);
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && s.b[1755]) {
            s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1755])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);
        }

        s.b[1756] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && s.b[1756]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1757] = (((-s.v[602]) / s.v[1220]) < 0.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1756])) && s.b[1757]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) && (!s.b[1756])) && (!s.b[1757])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1754])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1758] = (s.v[534] > 1000.0);
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1744])) && s.b[1758]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1759] = (s.v[1194] > ((-s.v[438]) * s.v[534]));
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        s.b[1760] = (s.v[537] == 4.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) && s.b[1760]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) && (!s.b[1760])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && s.b[1759]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1744])) && (!s.b[1758])) && (!s.b[1759])) {
            s.store_add_scaled_product_left_ad(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1744])) {
            s.store_mul_scale_ad_lhs(1222, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1761] = (s.v[668] == 0.0);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1761]) {
            s.store_scalar(1223, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1761])) {
            s.store_mul(1196, 558, 1186);
        }

        s.b[1762] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1762]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {
            s.store_sub(1198, 564, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1763] = (s.v[506] == 0.5);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && s.b[1763]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && (!s.b[1763])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(506), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1764] = (s.v[506] == 0.5);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && s.b[1764]) {
            s.store_sqrt_mul(1195, 1198, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) && (!s.b[1764])) {
            s.store_pow_mul_base_indices(1195, 1198, 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1762])) {
            s.store_mul(1202, 585, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 517, 1203, 1201);
        }

        s.b[1765] = (s.v[520] == 0.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1765]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {
            s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1766] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1766]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1766])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(597), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1767] = (s.v[1216] > 0.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1767]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1767])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1768] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1768]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1768])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1769] = (s.v[1216] > 0.0);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && s.b[1769]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1770] = (s.v[1215] > (-230.25850929940458));
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) && s.b[1770]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) && (!s.b[1770])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1769])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1765])) {
            s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);
        }

        s.b[1771] = (s.v[526] == 0.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1771]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1772] = (s.v[506] == 0.5);
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && s.b[1772]) {
            s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1772])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);
        }

        s.b[1773] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && s.b[1773]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1774] = (((-s.v[603]) / s.v[1220]) < 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1773])) && s.b[1774]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) && (!s.b[1773])) && (!s.b[1774])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1771])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1775] = (s.v[535] > 1000.0);
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1761])) && s.b[1775]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1776] = (s.v[1194] > ((-s.v[438]) * s.v[535]));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        s.b[1777] = (s.v[538] == 4.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) && s.b[1777]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) && (!s.b[1777])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && s.b[1776]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1761])) && (!s.b[1775])) && (!s.b[1776])) {
            s.store_add_scaled_product_left_ad(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1761])) {
            s.store_mul_scale_ad_lhs(1223, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        s.b[1778] = (s.v[669] == 0.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1778]) {
            s.store_scalar(1224, 0.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1778])) {
            s.store_mul(1196, 559, 1186);
        }

        s.b[1779] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && s.b[1779]) {
            s.store_scalar(1197, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) {
            s.store_sub(1198, 565, 1192);
            s.store_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));
        }

        s.b[1780] = (s.v[507] == 0.5);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) && s.b[1780]) {
            s.store_scalar(1200, 0.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) && (!s.b[1780])) {
            s.store_mul_sub_from_scalar_rhs_ad(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), 1.0, A::scale(s.ad_value(507), 2.0));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) {
            s.store_add(1201, 1199, 1200);
        }

        s.b[1781] = (s.v[507] == 0.5);
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) && s.b[1781]) {
            s.store_sqrt_mul(1195, 1198, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) && (!s.b[1781])) {
            s.store_pow_mul_base_indices(1195, 1198, 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1779])) {
            s.store_mul(1202, 586, 1195);
            s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);
            s.store_mul3_lhs(1197, 518, 1203, 1201);
        }

        s.b[1782] = (s.v[521] == 0.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && s.b[1782]) {
            s.store_scalar(1204, 0.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) {
            s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);
            s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);
            s.store_square(1207, 1206);
            s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);
            s.store_sqrt(1209, 1208);
            s.store_mul(1210, 1208, 1209);
        }

        s.b[1783] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && s.b[1783]) {
            s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1783])) {
            s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);
            s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);
            s.store_add_scaled_value_products(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, s.ad_value(598), s.ad_value(1208), (-1.0), s.ad_value(1205), s.ad_value(1210), 0.5);
            s.store_mul_offset_lhs(1216, 1214, (-1.0), 1213);
            s.store_square(1177, 1216);
        }

        s.b[1784] = (s.v[1216] > 0.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && s.b[1784]) {
            s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1784])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));
        }

        s.b[1785] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && s.b[1785]) {
            s.store_exp_sub(1195, 1215, 1177);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1785])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) {
            s.store_mul_ad_lhs(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);
        }

        s.b[1786] = (s.v[1216] > 0.0);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && s.b[1786]) {
            s.copy_ad(1217, 1179);
        }

        s.b[1787] = (s.v[1215] > (-230.25850929940458));
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1786])) && s.b[1787]) {
            s.store_exp(1195, 1215);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1786])) && (!s.b[1787])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1786])) {
            s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1782])) {
            s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);
            s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);
        }

        s.b[1788] = (s.v[527] == 0.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && s.b[1788]) {
            s.store_scalar(1219, 0.0);
        }

        s.b[1789] = (s.v[507] == 0.5);
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) && s.b[1789]) {
            s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) && (!s.b[1789])) {
            s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) {
            s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);
        }

        s.b[1790] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) && s.b[1790]) {
            s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));
        }

        s.b[1791] = (((-s.v[604]) / s.v[1220]) < 0.0);
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) && (!s.b[1790])) && s.b[1791]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) && (!s.b[1790])) && (!s.b[1791])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1788])) {
            s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195);
        }

        s.b[1792] = (s.v[536] > 1000.0);
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        if (((s.b[1159] && s.b[1176]) && (!s.b[1778])) && s.b[1792]) {
            s.store_scalar(1221, 1.0);
        }

        s.b[1793] = (s.v[1194] > ((-s.v[438]) * s.v[536]));
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        s.b[1794] = (s.v[539] == 4.0);
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1792])) && s.b[1793]) && s.b[1794]) {
            s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);
        }

        if (((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1792])) && s.b[1793]) && (!s.b[1794])) {
            s.store_pow_ad(1195, A::abs(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1792])) && s.b[1793]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));
        }

        if ((((s.b[1159] && s.b[1176]) && (!s.b[1778])) && (!s.b[1792])) && (!s.b[1793])) {
            s.store_add_scaled_product_left_ad(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && (!s.b[1778])) {
            s.store_mul_scale_ad_lhs(1224, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 1221);
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_add_scaled_products3(473, s.ad_value(667), s.ad_value(1222), 1.0, s.ad_value(668), s.ad_value(1223), 1.0, s.ad_value(669), s.ad_value(1224), 1.0);
            s.store_add_scaled_products3(688, s.ad_value(667), s.ad_value(557), 1.0, s.ad_value(668), s.ad_value(558), 1.0, s.ad_value(669), s.ad_value(559), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(477, 472, 1.0, 688, A::exp_scaled_input(s.ad_value(482), (s.v[365] * s.v[689])), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_iia(478, 473, 1.0, 688, A::exp_scaled_input(s.ad_value(483), (s.v[365] * s.v[689])), (-1.0), (-1.0));
        }

        s.b[1795] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        s.b[1796] = ((s.v[472] > 0.0) && (s.v[473] > 0.0));
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        s.b[1797] = ((((((s.v[477] / s.v[472]) > 0.001) || ((s.v[478] / s.v[473]) > 0.001)) && (s.v[477] > 0.0)) && (s.v[478] > 0.0)) && (s.v[478] > s.v[477]));
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1796]) && s.b[1797]) {
            s.store_div(484, 477, 478);
            s.store_div_scaled_inputs(691, A::ln(s.ad_value(484)), s.v[364], A::sub(s.ad_value(482), s.ad_value(483)), 1.0);
            s.store_div_scaled_value_offset_denominator(690, s.ad_value(477), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(482), s.v[365], s.ad_value(691))), (-1.0), 1.0);
        }

        if ((s.b[1159] && s.b[1176]) && s.b[1795]) {
            s.store_add_scaled_offset_product_rhs_mixed_aia(474, A::add_scaled_offset_product_rhs(s.ad_value(469), 1.0, s.ad_value(688), A::exp_scaled_input(s.ad_value(479), (s.v[365] * s.v[689])), (-1.0), (-1.0)), 1.0, 690, A::exp(A::mul_scaled_lhs(s.ad_value(479), s.v[365], s.ad_value(691))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(475, A::add_scaled_offset_product_rhs(s.ad_value(470), 1.0, s.ad_value(688), A::exp_scaled_input(s.ad_value(480), (s.v[365] * s.v[689])), (-1.0), (-1.0)), 1.0, 690, A::exp(A::mul_scaled_lhs(s.ad_value(480), s.v[365], s.ad_value(691))), (-1.0), (-1.0));
            s.store_add_scaled_offset_product_rhs_mixed_aia(476, A::add_scaled_offset_product_rhs(s.ad_value(471), 1.0, s.ad_value(688), A::exp_scaled_input(s.ad_value(481), (s.v[365] * s.v[689])), (-1.0), (-1.0)), 1.0, 690, A::exp(A::mul_scaled_lhs(s.ad_value(481), s.v[365], s.ad_value(691))), (-1.0), (-1.0));
        }

        s.b[1798] = (((s.v[469] < 0.0) && (s.v[470] < 0.0)) && (s.v[471] < 0.0));
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        s.b[1799] = (((((((s.v[474] / s.v[469]) > 0.001) || ((s.v[475] / s.v[470]) > 0.001)) || ((s.v[476] / s.v[471]) > 0.001)) && (s.v[474] < 0.0)) && (s.v[475] < 0.0)) && (s.v[476] < 0.0));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if ((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1798]) && s.b[1799]) {
            s.store_div(484, 474, 475);
            s.store_div_scaled_inputs(485, A::ln(s.ad_value(484)), (-s.v[364]), A::sub(s.ad_value(479), s.ad_value(480)), 1.0);
            s.store_div_add_scaled_inputs_rhs_indices(487, 480, 480, 1.0, 479, -1.0);
            s.store_scaled_mul_ad(488, A::offset(s.ad_value(484), (-1.0)), A::offset(A::pow(s.ad_value(484), s.ad_value(487)), (-1.0)), s.v[364]);
            s.store_div_add_scaled_inputs_rhs_indices(487, 479, 479, 1.0, 480, -1.0);
            s.store_sub_ad_lhs(489, A::add_scaled_products(A::pow(s.ad_value(484), s.ad_value(487)), A::sub(s.ad_value(480), s.ad_value(479)), 1.0, s.ad_value(484), s.ad_value(479), 1.0), 480);
            s.store_div(486, 488, 489);
            s.store_add(693, 485, 486);
        }

        s.b[1800] = (((((s.v[481] * s.v[365]) * s.v[693])) as f64).abs() < 1e-6);
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        let (assign40070_e53201,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1798]) && s.b[1799]) && s.b[1800]) {
        (1.0,)
    } else {
        (s.v[687],)
    }
};
        s.v[687] = assign40070_e53201;

        if (((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1798]) && s.b[1799]) && s.b[1800]) {
            s.store_mul_add_scaled_inputs_rhs(692, 476, A::div_from_scalar(1.0, s.ad_value(481)), 1.0, s.ad_value(693), (0.5 * s.v[365]));
            s.store_div_scaled_product_indices(693, 476, 693, ((-0.5) * s.v[365]), 481, 1.0);
        }

        let (assign40100_e53263,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) {
        (0.0,)
    } else {
        (s.v[687],)
    }
};
        s.v[687] = assign40100_e53263;

        if (((((s.b[1159] && s.b[1176]) && s.b[1795]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) {
            s.store_div_scaled_value_offset_denominator(692, s.ad_value(476), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(481), (-s.v[365]), s.ad_value(693))), (-1.0), 1.0);
        }

        let (assign40120_e53307,) = {
    if (s.b[1159] && s.b[1176]) {
        let assign40120_e53296: f64 = (s.v[667] * s.v[575]);
        let assign40120_e53299: f64 = (s.v[668] * s.v[576]);
        let assign40120_e53300: f64 = (assign40120_e53296 + assign40120_e53299);
        let assign40120_e53303: f64 = (s.v[669] * s.v[577]);
        let assign40120_e53304: f64 = (assign40120_e53300 + assign40120_e53303);
        let assign40120_e53305: f64 = (s.v[547] * assign40120_e53304);
        (assign40120_e53305,)
    } else {
        (s.v[495],)
    }
};
        s.v[495] = assign40120_e53307;

        s.b[1801] = ((s.v[667] * s.v[575]) <= s.v[495]);
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        let (assign40140_e53320,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1801]) {
        (0.0,)
    } else {
        (s.v[672],)
    }
};
        s.v[672] = assign40140_e53320;

        s.b[1802] = ((s.v[668] * s.v[576]) <= s.v[495]);
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        let (assign40160_e53333,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1802]) {
        (0.0,)
    } else {
        (s.v[673],)
    }
};
        s.v[673] = assign40160_e53333;

        s.b[1803] = ((s.v[669] * s.v[577]) <= s.v[495]);
        s.v[1803] = if s.b[1803] { 1.0 } else { 0.0 };

        let (assign40180_e53346,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1803]) {
        (0.0,)
    } else {
        (s.v[674],)
    }
};
        s.v[674] = assign40180_e53346;

        s.b[1804] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[1804] = if s.b[1804] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1176]) && s.b[1804]) {
            s.store_ln_ad(681, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(688), 1e-21, 1.0));
            s.store_ln_ad(683, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(690), 1e-21, 1.0));
            s.store_ln_ad(685, A::div_scalar_offset_denominator((0.5 * p.p815), A::abs(s.ad_value(692)), 1e-21, 1.0));
        }

        if (s.b[1159] && s.b[1176]) {
            s.store_min_with_scalar(681, 681, 230.25850929940458);
            s.store_exp(682, 681);
            s.store_min_with_scalar(683, 683, 230.25850929940458);
            s.store_exp(684, 683);
            s.store_min_with_scalar(685, 685, 230.25850929940458);
            s.store_exp(686, 685);
        }

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[1921] = 0.0;

        s.b[1994] = (s.v[0] == 1.0);
        s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };

        if s.b[1994] {
            s.store_voltage(819, ctx, nodes, Some(5), Some(6));
            s.store_voltage(820, ctx, nodes, Some(7), Some(6));
            s.store_voltage(821, ctx, nodes, Some(6), Some(8));
            s.store_scaled_voltage(826, ctx, nodes, Some(6), Some(10), -1.0);
            s.store_scaled_voltage(827, ctx, nodes, Some(7), Some(11), -1.0);
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (!s.b[1994]) {
            s.store_scaled_voltage(819, ctx, nodes, Some(5), Some(6), -1.0);
            s.store_scaled_voltage(820, ctx, nodes, Some(7), Some(6), -1.0);
            s.store_scaled_voltage(821, ctx, nodes, Some(6), Some(8), -1.0);
            s.store_voltage(826, ctx, nodes, Some(6), Some(10));
            s.store_voltage(827, ctx, nodes, Some(7), Some(11));
        }

        s.store_add(823, 819, 821);

        s.copy_ad(828, 819);

        s.copy_ad(829, 821);

        s.store_add(830, 820, 821);

        s.store_sub(831, 819, 820);

        s.store_scale(1805, 828, (-s.v[349]));

        s.store_scale(1806, 831, (-s.v[349]));

        s.store_scaled_sub(1807, 823, 694, (-s.v[349]));

        s.v[825] = 1.0;

        s.b[1995] = (s.v[820] < 0.0);
        s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };

        if s.b[1995] {
            s.store_scalar(825, (-1.0));
            s.store_sub(819, 819, 820);
            s.store_add(821, 821, 820);
            s.store_neg(820, 820);
        }

        s.store_add(822, 820, 821);

        s.store_div_scaled_product_offset_denominator(824, s.ad_value(820), s.ad_value(820), 1.0, A::sqrt_square_offset(s.ad_value(820), 0.01), 0.1, 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(1999, 822, 0.5, 821, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(822), s.ad_value(821))), s.ad_value(733))), (-0.5), 731, 1.0);

        s.copy_ad(1808, 1999);

        s.store_add_scaled_inputs4_mixed_iiai(1922, 821, 1.0, 1999, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1999)), s.ad_value(732))), (-(-0.5)), 735, 1.0);

        s.copy_ad(1809, 1922);

        s.v[1923] = 0.0;

        s.b[2155] = ((p.p45 != 0.0) && (s.v[179] != 1.0));
        s.v[2155] = if s.b[2155] { 1.0 } else { 0.0 };

        if s.b[2155] {
            s.store_add_scaled_inputs3_indices(1924, 1922, 1.0, 820, 0.5, 824, (-0.5));
            s.store_sub_ad_lhs(1925, A::sqrt(A::add(s.ad_value(1924), s.ad_value(722))), 730);
            s.store_offset_div_scaled_inputs2_indices(1919, 1925, 2.0, 737, (-2.0), 738, 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(1926, 1925, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(179), s.ad_value(738), 0.25), A::add(s.ad_value(1919), A::sqrt_square_offset(s.ad_value(1919), 0.4804530139182)), (-1.0));
            s.store_add_scaled_square_product_indices(1927, 1926, 1.0, 730, 1926, 2.0);
            s.store_add_scaled_inputs3_indices(1922, 1927, 1.0, 820, (-0.5), 824, (-(-0.5)));
            s.store_sub(1923, 1809, 1922);
        }

        s.copy_ad(1996, 722);

        s.copy_ad(1997, 732);

        s.copy_ad(1998, 723);

        s.copy_ad(2000, 1922);

        s.copy_ad(2004, 1923);

        s.copy_ad(2001, 714);

        s.copy_ad(2002, 771);

        s.store_add_scaled_inputs3_indices(2003, 823, 1.0, 2004, (-1.0), 694, -1.0);

        s.store_add_scaled_inputs3_indices(2005, 2000, 1.0, 820, 0.5, 824, (-0.5));

        s.v[2017] = 1.0;

        s.b[2156] = (s.v[185] > 0.0);
        s.v[2156] = if s.b[2156] { 1.0 } else { 0.0 };

        if s.b[2156] {
            s.store_scale(2008, 1996, s.v[355]);
            s.store_scale(2009, 2005, s.v[355]);
            s.store_scale(2010, 2003, s.v[355]);
            s.store_offset_div_scaled_inputs_mixed_ia(1920, 1998, 0.5, A::sqrt(s.ad_value(2008)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1921, 2008, 1.0, 1998, A::sqrt(s.ad_value(2008)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2011, A::div_scaled_inputs2(s.ad_value(2010), 1.0, s.ad_value(1921), (-1.0), s.ad_value(1920), 1.0), 1.0, 2008, 0.5, A::offset(s.ad_value(186), 1.0), 2009, (-1.0));
            s.store_offset_scaled(2012, 2008, 0.5, 2.0);
            s.store_add(2013, 2008, 2009);
            s.store_sub_scaled_inputs_ad(1920, A::add_scaled_inputs_product(s.ad_value(2010), 1.0, s.ad_value(2013), (-1.0), s.ad_value(1998), A::sqrt(s.ad_value(2013)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2008), s.ad_value(1998)), A::sqrt(s.ad_value(2008)))), 2.0);
            s.store_add_scaled_inputs(2014, 1920, 2.0, 2012, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2011, 0.5, 2014, 0.5, 2011, 2014, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(1921, 2010, 2.0, 2009, (-2.0), 2012, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2015, 1920, 0.5, 1921, 0.5, 1920, 1921, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1920, 2015, 0.5, 2012, 0.5, 2015, 2012, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2016, 1920, 0.5, 2012, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2012), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2016), s.ad_value(2012)), 1.0);
        }

        s.b[2157] = (s.v[1921] > (-230.25850929940458));
        s.v[2157] = if s.b[2157] { 1.0 } else { 0.0 };

        if (s.b[2156] && s.b[2157]) {
            s.store_exp(2017, 1921);
        }

        if (s.b[2156] && (!s.b[2157])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2017, 1e-100, (-230.25850929940458), 1921, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2018, 695, 2017, 1.0);

        s.store_scale(2019, 2018, s.v[709]);

        s.store_mul_ad_product_rhs(2020, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2005)), 1.0));

        s.store_mul_offset_rhs(2021, 2019, 2020, 1.0);

        s.store_div_from_scalar(2022, 1.0, 2021);

        s.store_mul_ad_rhs(2006, 1998, A::sqrt_scaled_input(s.ad_value(2022), s.v[709]));

        s.store_square(2007, 2006);

        s.store_div_from_scalar(2023, 1.0, 2007);

        s.store_mul(2024, 2000, 2022);

        s.store_mul(2025, 2003, 2022);

        s.store_div_scaled_value_offset_denominator(2026, s.ad_value(824), 2.0, A::sqrt_product_offset(s.ad_value(192), s.ad_value(824), 1.0), 1.0, 1.0);

        s.store_mul_ad_product_rhs_mixed_ia(2027, 191, 2026, A::offset(A::mul(s.ad_value(193), s.ad_value(2005)), 1.0));

        s.store_mul(2028, 1996, 2022);

        s.store_sqrt_square_add(1920, 1999, 1997);

        s.store_sqrt_add_ad(1921, A::square(A::sub(s.ad_value(1999), s.ad_value(2027))), s.ad_value(1997));

        s.store_mul_add_scaled_inputs3_offset_rhs(2029, 2022, s.ad_value(2027), 0.5, s.ad_value(1920), 0.5, s.ad_value(1921), ((-1.0) * (0.5)), 0.0);

        s.store_add(2030, 2028, 2024);

        s.store_sub(2031, 2030, 2029);

        s.b[2158] = (p.p45 > 0.0);
        s.v[2158] = if s.b[2158] { 1.0 } else { 0.0 };

        s.b[2159] = (((s.v[2031]) as f64).abs() < 1e-5);
        s.v[2159] = if s.b[2159] { 1.0 } else { 0.0 };

        if (s.b[2158] && s.b[2159]) {
            s.store_offset_ad(2032, A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2031), 1.0, A::scale(s.ad_value(2031), 0.3125), 0.5)), 1.0);
        }

        s.b[2160] = (s.v[2031] < 460.51701859880916);
        s.v[2160] = if s.b[2160] { 1.0 } else { 0.0 };

        if ((s.b[2158] && (!s.b[2159])) && s.b[2160]) {
            s.store_exp_neg_input(2046, 2031);
        }

        if ((s.b[2158] && (!s.b[2159])) && (!s.b[2160])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_scalar(1919, (if (s.v[2031] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2158] && (!s.b[2159])) {
            s.store_offset_ad(2032, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2006), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2031))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2031), 1.0, s.ad_value(2046))), 2.0), 1.0);
        }

        if (!s.b[2158]) {
            s.store_offset_div_scaled_inputs_mixed_ia(2032, 2006, 0.5, A::sqrt(s.ad_value(2031)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2033, s.ad_value(2031), 1.0, s.ad_value(2006), A::sqrt(s.ad_value(2031)), 1.0, s.ad_value(2032), A::ln(A::offset(s.ad_value(2032), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2_indices(2034, 2025, 1.0, 2033, (-1.0), 2032, 1.0);

        s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2007)), 1.0)), (-1.0));

        s.v[2039] = 0.0;

        s.v[2041] = 1.0;

        s.b[2161] = (s.v[2034] > (-30.0));
        s.v[2161] = if s.b[2161] { 1.0 } else { 0.0 };

        if s.b[2161] {
            s.store_offset_mul(2035, 2032, 2034, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1919, 2035, 2035, 10.0, 0.5);
            s.store_sub_ad_rhs(2036, 2034, A::ln(s.ad_value(1919)));
            s.store_scaled_add_sqrt_square_offset_rhs(2037, 2036, 2036, 2.0, 0.5);
        }

        s.b[2162] = ((s.v[2034] - s.v[2037]) < 230.25850929940458);
        s.v[2162] = if s.b[2162] { 1.0 } else { 0.0 };

        if (s.b[2161] && s.b[2162]) {
            s.store_exp_sub(1919, 2034, 2037);
        }

        if (s.b[2161] && (!s.b[2162])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1919, A::sub(s.ad_value(2034), s.ad_value(2037)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if s.b[2161] {
            s.store_div(2038, 1919, 2032);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2037), 1.0, 2.0), 2038);
        }

        s.b[2163] = (s.v[2038] > 1e-6);
        s.v[2163] = if s.b[2163] { 1.0 } else { 0.0 };

        if (s.b[2161] && s.b[2163]) {
            s.store_mul_offset_ad_rhs(2039, 2032, A::sub(s.ad_value(2037), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2038), s.ad_value(1919), 1.0), 1.0, (-1.0), s.ad_value(2038), 1.0)), 1.0);
        }

        if (s.b[2161] && (!s.b[2163])) {
            s.store_mul_ad_affine_product_rhs(2039, 2032, s.ad_value(2038), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if s.b[2161] {
            s.store_add_scaled_inputs3_offset_mixed_iia(1919, 2025, 0.5, 2039, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2025), s.ad_value(2039)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2040, 2007, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2007)), s.ad_value(1919), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2041, 2040, 2040, 1.0, 2039, 1.0);
            s.store_add_scaled_product_indices(2031, 2030, 1.0, 2041, 2029, (-1.0));
        }

        s.store_offset_scaled(2042, 2006, 0.7071067811865475, 1.0);

        let assign41520_e54443: f64 = (1e-5 * s.v[2042]);
        s.v[2043] = assign41520_e54443;

        s.store_div_from_scalar(2044, 1.0, 2042);

        s.v[2151] = 0.0;

        s.v[2045] = 0.0;

        s.b[2164] = (s.v[2031] < 460.51701859880916);
        s.v[2164] = if s.b[2164] { 1.0 } else { 0.0 };

        if s.b[2164] {
            s.store_exp_neg_input(2046, 2031);
        }

        if (!s.b[2164]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2046, 1e-200, 2031, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2165] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.v[2165] = if s.b[2165] { 1.0 } else { 0.0 };

        if s.b[2165] {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2045, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2046)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        s.b[2166] = (s.v[2025] < (-s.v[2043]));
        s.v[2166] = if s.b[2166] { 1.0 } else { 0.0 };

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_neg(2133, 2025);
            s.store_scaled_mul(2134, 2133, 2044, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2135, 2134, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2130, 2133, 2135);
            s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::offset(s.ad_value(2135), 1.0), 1.0);
            s.store_sub_scaled_inputs(2137, 2130, 2.0, 2007, 1.0);
            s.store_sub_ln_mul_lhs(2138, 2136, 2023, 2135);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.5, s.ad_value(2136), 1.0), 1.0);
            s.store_add_ad_rhs(2139, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::sub_scaled_inputs(A::square(s.ad_value(2137)), 0.3333333333333333, s.ad_value(2136), 1.0))), 1.0));
        }

        s.b[2167] = (s.v[2139] < 230.25850929940458);
        s.v[2167] = if s.b[2167] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && s.b[2166]) && s.b[2167]) {
            s.store_exp(2140, 2139);
        }

        if (((!s.b[2165]) && s.b[2166]) && (!s.b[2167])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2140, 2139, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((!s.b[2165]) && s.b[2166]) {
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_div_from_scalar_offset_square(2130, 1.0, 2139, 2.0);
            s.store_mul_square_lhs(2142, 2139, 2130);
            s.store_mul3_affine_lhs(2143, 2139, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2133, 2139);
            s.store_mul(2131, 2046, 2141);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2131), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2046), 1.0, s.ad_value(2143)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2140), 1.0, s.ad_value(2139), (-1.0), s.ad_value(2131), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::sub(A::offset(s.ad_value(2139), (-1.0)), s.ad_value(2142)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2140), 1.0, s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2045, 2139, -1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_scaled_input(2147, 1.0, 2006, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2148, A::mul_scaled_lhs(s.ad_value(2042), 1.25, s.ad_value(2147)), (-1.0), 2147);
            s.store_mul_ad_product_rhs_mixed_ia(2149, 2025, 2044, A::offset(A::mul(s.ad_value(2148), s.ad_value(2025)), 1.0));
        }

        s.b[2168] = ((-s.v[2149]) > (-230.25850929940458));
        s.v[2168] = if s.b[2168] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2168]) {
            s.store_exp_neg_input(2130, 2149);
        }

        if (((!s.b[2165]) && (!s.b[2166])) && (!s.b[2168])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2130, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2149)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar(2150, 1.0, 2130);
            s.store_add_scaled_inputs_product_right_ad(2151, 2025, 1.0, 2007, 0.5, 2006, A::sqrt(A::add_scaled_inputs3(s.ad_value(2025), 1.0, s.ad_value(2007), 0.25, s.ad_value(2150), -1.0)), (-1.0));
        }

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_offset(2152, 2031, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2046] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2138, 2031, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2169] = (s.v[2154] < 230.25850929940458);
        s.v[2169] = if s.b[2169] { 1.0 } else { 0.0 };

        if (((!s.b[2165]) && (!s.b[2166])) && s.b[2169]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2046, 2140);
        }

        s.b[2170] = (s.v[2154] > (s.v[2031] - 230.25850929940458));
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && s.b[2170]) {
            s.store_exp_sub(2140, 2154, 2031);
            s.store_div(2141, 2046, 2140);
        }

        if ((((!s.b[2165]) && (!s.b[2166])) && (!s.b[2169])) && (!s.b[2170])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((!s.b[2165]) && (!s.b[2166])) {
            s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2046), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2046), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2045, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        s.v[2048] = 0.0;

        s.v[2049] = 0.0;

        s.v[2050] = 0.0;

        s.v[2051] = 0.0;

        s.v[2052] = 0.0;

        s.v[2053] = 0.0;

        s.v[2054] = 0.0;

        s.v[2055] = 1.0;

        s.v[2056] = 1.0;

        s.store_sub(2057, 2025, 2045);

        s.v[2058] = 0.0;

        s.store_mul(2059, 2021, 2057);

        s.v[2060] = 1.0;

        s.v[2061] = 1.0;

        s.v[2065] = 1.0;

        s.v[2066] = 1.0;

        s.v[2068] = 1.0;

        s.b[2171] = (s.v[2025] > 0.0);
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        if s.b[2171] {
            s.store_div_from_scalar_offset_square(1919, 1.0, 2045, 2.0);
            s.store_mul_square_lhs(2047, 2045, 1919);
            s.store_mul3_affine_lhs(2048, 2045, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs_mixed_ai(2049, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2047), 12.0), 1919, 1919);
            s.store_scalar(2050, 0.0);
        }

        s.b[2172] = (s.v[2045] < 230.25850929940458);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2172]) {
            s.store_exp(2050, 2045);
            s.store_div_from_scalar(2051, 1.0, 2050);
            s.store_mul(2050, 2046, 2050);
        }

        s.b[2173] = (s.v[2045] > (s.v[2031] - 230.25850929940458));
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if ((s.b[2171] && (!s.b[2172])) && s.b[2173]) {
            s.store_exp_sub(2050, 2045, 2031);
            s.store_div(2051, 2046, 2050);
        }

        if ((s.b[2171] && (!s.b[2172])) && (!s.b[2173])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2050, 1e-100, A::sub(s.ad_value(2031), s.ad_value(2045)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2051, 1e-100, 2045, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2171] {
            s.store_add_scaled_product_right_ad(2052, 2050, 1.0, 2046, A::add(A::offset(s.ad_value(2045), 1.0), s.ad_value(2047)), (-1.0));
        }

        s.b[2174] = (s.v[2045] < 1e-5);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2174]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2053, 2045, 1.0, 2045, 1.0, 2045, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2052, A::mul3(s.ad_value(2046), s.ad_value(2045), s.ad_value(2045)), 2045, A::scale_offset(s.ad_value(2045), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2054, 2045, 1919, 0.7071067811865475);
            s.store_offset_div_scaled_product(2055, s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2045), 0.5)), 1.0, A::square(s.ad_value(2045)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0, 1.0);
        }

        if (s.b[2171] && (!s.b[2174])) {
            s.store_add_offset_lhs(2053, 2045, (-1.0), 2051);
            s.store_sqrt(2054, 2053);
            s.store_offset_scaled_ad(2055, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2051)), s.ad_value(2054)), 0.5, 1.0);
        }

        if s.b[2171] {
            s.store_div_scaled_offset_numerator(2056, A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2005)), 1.0, 1.0, A::offset(A::mul(s.ad_value(702), s.ad_value(2005)), 1.0), 1.0);
        }

        s.b[2175] = (s.v[2052] > 1e-100);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_sqrt_ad_rhs(2057, 2006, A::add(s.ad_value(2053), s.ad_value(2052)));
            s.store_div_scaled_product3_mixed_iiia(2058, 2007, 2052, 2021, 1.0, A::add_scaled_product(s.ad_value(2057), 1.0, s.ad_value(2006), s.ad_value(2054), 1.0), 1.0);
            s.store_mul3_lhs(2059, 2054, 2006, 2021);
        }

        s.b[2176] = (s.v[212] < 0.0);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2176]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2060, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2176])) {
            s.store_offset_mul(2060, 212, 2005, 1.0);
        }

        s.b[2177] = (s.v[213] < 0.0);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2177]) {
            s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2058, 1.0);
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2177])) {
            s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2058, 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul_product3_indices(2062, 2058, 751, 2060, 2061, 1.0);
            s.store_mul_add_scaled_product_rhs(2063, 768, s.ad_value(2059), 1.0, s.ad_value(769), s.ad_value(2058), 1.0);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2053), 1.0, A::add(s.ad_value(2053), s.ad_value(2052)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2063), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2065, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
        }

        s.b[2178] = (s.v[216] < 0.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2178]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2066, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2005)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2178])) {
            s.store_offset_mul(2066, 216, 2005, 1.0);
        }

        if (s.b[2171] && s.b[2175]) {
            s.store_mul(1921, 2058, 2066);
            s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2179] = (s.v[217] < 0.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if ((s.b[2171] && s.b[2175]) && s.b[2179]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if ((s.b[2171] && s.b[2175]) && (!s.b[2179])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        s.copy_ad(1810, 2003);

        s.copy_ad(1811, 2005);

        s.copy_ad(1812, 2021);

        s.copy_ad(1813, 2022);

        s.copy_ad(1814, 2006);

        s.copy_ad(1815, 2007);

        s.copy_ad(1816, 2023);

        s.copy_ad(1817, 2025);

        s.copy_ad(1818, 2030);

        s.copy_ad(1819, 2031);

        s.copy_ad(1820, 2042);

        s.v[1821] = s.v[2043];

        s.copy_ad(1822, 2044);

        s.copy_ad(1823, 2151);

        s.copy_ad(1824, 2046);

        s.copy_ad(1825, 2045);

        s.copy_ad(1826, 2048);

        s.copy_ad(1827, 2049);

        s.copy_ad(1828, 2050);

        s.copy_ad(1829, 2051);

        s.copy_ad(1830, 2053);

        s.copy_ad(1831, 2052);

        s.copy_ad(1832, 2054);

        s.copy_ad(1833, 2055);

        s.copy_ad(1834, 2056);

        s.copy_ad(1835, 2057);

        s.copy_ad(1836, 2058);

        s.copy_ad(1837, 2059);

        s.copy_ad(1838, 2060);

        s.copy_ad(1839, 2061);

        s.copy_ad(1840, 2065);

        s.copy_ad(1841, 2066);

        s.copy_ad(1842, 2068);

        s.v[2070] = 0.0;

        s.store_scale(2069, 2021, 4.60517018598809);

        s.copy_ad(2086, 2069);

        s.copy_ad(2087, 820);

        s.store_mul(2088, 820, 2022);

        s.copy_ad(2092, 2045);

        s.v[2093] = 0.0;

        s.v[2096] = 0.0;

        s.copy_ad(2098, 2051);

        s.copy_ad(2099, 2053);

        s.copy_ad(2101, 2052);

        s.copy_ad(2102, 2059);

        s.copy_ad(2103, 2045);

        s.copy_ad(2104, 2051);

        s.copy_ad(2106, 2052);

        s.copy_ad(2107, 2053);

        s.store_sub(2108, 2025, 2045);

        s.v[2109] = 1.0;

        s.v[2111] = 1.0;

        s.v[2110] = 0.0;

        s.copy_ad(2120, 2058);

        s.store_mul(2124, 2108, 2021);

        s.v[2121] = 0.0;

        s.copy_ad(2122, 2059);

        s.v[2127] = 0.0;

        s.v[2126] = 1.0;

        s.copy_ad(2129, 2001);

        s.copy_ad(2128, 2124);

        s.b[2180] = (s.v[2025] > 0.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        s.b[2181] = (s.v[2052] > 1e-100);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2129, 2001, 2068);
            s.store_div(2070, 2129, 2065);
            s.store_add_scaled_inputs(2071, 2057, 1.0, 2007, 0.5);
            s.store_div_scaled_product_by_product(1919, s.ad_value(2007), s.ad_value(2050), 1.0, s.ad_value(2071), s.ad_value(2071), 1.0);
        }

        s.b[2182] = (s.v[1919] > 0.0001);
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2182]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        s.b[2183] = (s.v[1920] < 1e-10);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && s.b[2183]) {
            s.store_scalar(1921, 1.0);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2182]) && (!s.b[2183])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2182])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul(2072, 1921, 2071);
        }

        s.b[2184] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_scaled_mul(2073, 2021, 2072, 0.475);
            s.store_add_scaled_product_indices(1919, 2058, 1.0, 2055, 2073, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2074, 1919, 1919, 1e-12, 0.5);
            s.store_add_scaled_value_products(2075, s.ad_value(2058), (-1.0), s.ad_value(2021), s.ad_value(2057), 1.0, A::offset(s.ad_value(2055), (-1.0)), s.ad_value(2073), 1.0);
            s.store_offset_div_scaled_product(2076, s.ad_value(2007), s.ad_value(2021), 0.5, s.ad_value(2075), 1.0, 1.0);
            s.store_add_scaled_product_indices(1919, 2075, 1.0, 769, 2074, 1.0);
            s.store_pow_ad(2077, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2076), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2077);
            s.store_div(1919, 2074, 2075);
            s.store_mul_pow_ad_rhs(2078, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_div_scaled_product_mixed_iiai(1921, 2078, 701, A::add(A::offset(s.ad_value(2076), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, 2075, 1.0);
            s.store_mul_product3_indices(2079, 2074, 751, 2060, 2061, 1.0);
            s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2060), s.ad_value(2061)), s.ad_value(2076), (-1.0), s.ad_value(1921), 1.0), 1.0);
        }

        s.b[2185] = (s.v[1919] < 230.25850929940458);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && s.b[2185]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if (((s.b[2180] && s.b[2181]) && s.b[2184]) && (!s.b[2185])) {
            s.copy_ad(1920, 1919);
        }

        if ((s.b[2180] && s.b[2181]) && s.b[2184]) {
            s.store_div_scaled_product3_mixed_iiia(2080, 2073, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2077), 1.0, s.ad_value(2078), 1.0, s.ad_value(2079), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2081, 2072, A::div_scaled_value_offset_denominator(s.ad_value(2080), 1.0, A::sqrt_square_offset(s.ad_value(2080), 1.0), 1.0, 1.0), 1.0);
        }

        if ((s.b[2180] && s.b[2181]) && (!s.b[2184])) {
            s.copy_ad(2081, 2072);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul3_affine_lhs(2082, 2021, 2070, 0.7071067811865475, 0.0, 2081);
        }

        s.b[2186] = (s.v[0] == (-1.0));
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2181]) && s.b[2186]) {
            s.store_div_ad_rhs(2082, 2082, A::sqrt(A::offset(s.ad_value(2082), 1.0)));
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_div_from_scalar_offset_ad(2083, 2.0, A::sqrt(A::scale_offset(s.ad_value(2082), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2083, 2082);
            s.store_mul_ad_product_rhs_mixed_ia(2084, 2081, 2083, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2083)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2083), 4.0), 1.0)), 1.0));
            s.store_scale(2085, 2084, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1919, 2085, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2071), 2.0), 2023, 1.0, 2052, 1.0);
        }

        if (s.b[2180] && s.b[2181]) {
            s.store_mul_sub_ad_rhs(2086, 2021, s.ad_value(2085), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2180] && (!s.b[2181])) {
            s.copy_ad(2086, 2069);
        }

        if s.b[2180] {
            s.store_offset(1919, 2002, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2086, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2087, 2086, 1919, 1.0, A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), 1.0, A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919))), 1.0, 1.0);
            s.store_mul(2088, 2087, 2022);
            s.store_add(2089, 2031, 2088);
        }

        s.b[2187] = (s.v[2088] < 460.51701859880916);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2187]) {
            s.store_exp_neg_input(2090, 2088);
        }

        if (s.b[2180] && (!s.b[2187])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2090, 1e-200, 2088, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2180] {
            s.store_mul(2091, 2046, 2090);
        }

        s.b[2188] = (((s.v[2025]) as f64).abs() <= s.v[2043]);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2188]) {
            s.store_scaled_square(2131, 2044, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2092, 2025, 2044, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2025), 1.0, s.ad_value(2091)), s.ad_value(2006), s.ad_value(2131)), 1.0));
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_offset(2152, 2089, 3.0);
            s.store_sub_ad(2135, A::add_scaled_inputs3(s.ad_value(2151), 0.5, s.ad_value(2152), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2151), s.ad_value(2152)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2152), 0.5, A::sqrt_square_offset(s.ad_value(2152), 5.0), 0.5));
            s.store_sub(2130, 2025, 2135);
            s.store_exp_neg_input(2131, 2135);
            s.store_div_from_scalar_offset_square(2132, 1.0, 2135, 2.0);
            s.store_mul_square_lhs(2142, 2135, 2132);
            s.store_mul3_affine_lhs(2143, 2135, 2132, 4.0, 0.0, 2132);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2132), 8.0, s.ad_value(2142), 12.0), 2132, 2132);
        }

        if (s.b[2180] && (!s.b[2188])) {
            if (1e-40 > ((s.v[2130] * s.v[2130]) - (s.v[2007] * (((s.v[2131] + s.v[2135]) - 1.0) - (s.v[2091] * ((s.v[2135] + 1.0) + s.v[2142])))))) {
                s.store_scalar(2136, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2136, 2130, 1.0, 2007, A::add_scaled_product(A::offset(A::add(s.ad_value(2131), s.ad_value(2135)), (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2135), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2153, 1.0, 2007, A::add_scaled_product(s.ad_value(2131), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2137, 2130, 2.0, 2007, A::add_scaled_sub_value_product(1.0, s.ad_value(2131), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2138, 2089, 1.0, 2135, (-1.0), A::ln(A::div(s.ad_value(2136), s.ad_value(2007))), 1.0);
            s.store_add(818, 2136, 2137);
            s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2138, A::add_scaled_square_product(s.ad_value(2137), 0.5, s.ad_value(2136), s.ad_value(2153), (-1.0)), 1.0);
            s.store_add_ad_rhs(2154, 2135, A::div_scaled_product3(s.ad_value(2136), s.ad_value(818), s.ad_value(2138), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2138), s.ad_value(2138)), s.ad_value(2137), A::add_scaled_square_product(s.ad_value(2137), 0.3333333333333333, s.ad_value(2136), s.ad_value(2153), (-1.0)))), 1.0));
        }

        s.b[2189] = (s.v[2154] < 230.25850929940458);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2188])) && s.b[2189]) {
            s.store_exp(2140, 2154);
            s.store_div_from_scalar(2141, 1.0, 2140);
            s.store_mul(2140, 2091, 2140);
        }

        s.b[2190] = (s.v[2154] > (s.v[2089] - 230.25850929940458));
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && s.b[2190]) {
            s.store_exp_sub(2140, 2154, 2089);
            s.store_div(2141, 2091, 2140);
        }

        if (((s.b[2180] && (!s.b[2188])) && (!s.b[2189])) && (!s.b[2190])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2140, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2154)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2141, 1e-100, 2154, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2180] && (!s.b[2188])) {
            s.store_div_from_scalar_offset_square(2130, 1.0, 2154, 2.0);
            s.store_mul_square_lhs(2142, 2154, 2130);
            s.store_mul3_affine_lhs(2143, 2154, 2130, 4.0, 0.0, 2130);
            s.store_mul_ad_product_lhs_mixed_ai(2144, A::sub_scaled_inputs(s.ad_value(2130), 8.0, s.ad_value(2142), 12.0), 2130, 2130);
            s.store_sub(2130, 2025, 2154);
            s.store_add_scaled_product_right_ad(2145, 2130, 2.0, 2007, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2141)), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), A::offset(s.ad_value(2143), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2146, 2130, 1.0, 2007, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2141), 1.0, s.ad_value(2154), 1.0, s.ad_value(2140), 1.0, (-1.0)), 1.0, s.ad_value(2091), A::add(A::offset(s.ad_value(2154), 1.0), s.ad_value(2142)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 2.0, 2007, A::add_scaled_inputs_product(s.ad_value(2141), 1.0, s.ad_value(2140), 1.0, s.ad_value(2091), s.ad_value(2144), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2130, 2145, 1.0, 2146, 2130, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2092, 2154, 1.0, A::div(s.ad_value(2146), A::add(s.ad_value(2145), A::sqrt(s.ad_value(2130)))), 2.0);
        }

        if s.b[2180] {
            s.store_sub(2093, 2092, 2045);
        }

        s.b[2191] = (s.v[2093] < 1e-10);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2191]) {
            s.store_add_scaled_inputs_product_right_ad(2094, 2025, 2.0, 2045, (-2.0), 2007, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0), 1.0, s.ad_value(2091), s.ad_value(2048), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2095, A::mul_sub_from_scalar_rhs(s.ad_value(2007), 1.0, s.ad_value(2090)), 2052);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1919, 2.0, 2007, A::add_scaled_value_products(s.ad_value(2051), 1.0, s.ad_value(2050), s.ad_value(2090), 1.0, s.ad_value(2091), s.ad_value(2049), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1919, 2094, 1.0, 1919, 2095, (-2.0));
            s.store_scaled_div_ad_rhs(2093, 2095, A::add(s.ad_value(2094), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2092, 2045, 2093);
        }

        if s.b[2180] {
            s.store_mul(2096, 2093, 2021);
            s.store_div_scaled_product_offset_denominator(2097, s.ad_value(2092), s.ad_value(2092), 1.0, A::square(s.ad_value(2092)), 2.0, 1.0);
        }

        s.b[2192] = (s.v[2092] < 230.25850929940458);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2192]) {
            s.store_exp_neg_input(2098, 2092);
        }

        s.b[2193] = (s.v[2092] < 1e-5);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2192]) && s.b[2193]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2099, 2092, 1.0, 2092, 1.0, 2092, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2092), 1.0, A::scale(s.ad_value(2092), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2100, 2092, 1919, 0.7071067811865475);
            s.store_mul3_ad_middle(2101, A::mul3_scaled_output(s.ad_value(2091), s.ad_value(2092), s.ad_value(2092), 0.16666666666666666), 2092, A::scale_offset(s.ad_value(2092), 1.75, 1.0));
        }

        if ((s.b[2180] && s.b[2192]) && (!s.b[2193])) {
            s.store_add_offset_lhs(2099, 2092, (-1.0), 2098);
            s.store_sqrt(2100, 2099);
            s.store_mul_add_scaled_inputs3_offset_rhs(2101, 2091, A::div_from_scalar(1.0, s.ad_value(2098)), 1.0, s.ad_value(2092), (-1.0), s.ad_value(2097), -1.0, (-1.0));
        }

        s.b[2194] = (s.v[2092] > (s.v[2089] - 230.25850929940458));
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2192])) && s.b[2194]) {
            s.store_exp_sub(1919, 2092, 2089);
            s.store_div(2098, 2091, 1919);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if ((s.b[2180] && (!s.b[2192])) && (!s.b[2194])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2098, 1e-100, 2092, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1919, 1e-100, A::sub(s.ad_value(2089), s.ad_value(2092)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2101, 1919, 1.0, 2091, A::add(A::offset(s.ad_value(2092), 1.0), s.ad_value(2097)), (-1.0));
        }

        if (s.b[2180] && (!s.b[2192])) {
            s.store_add_offset_lhs(2099, 2092, (-1.0), 2098);
            s.store_sqrt(2100, 2099);
        }

        if s.b[2180] {
            s.store_mul3_lhs(2102, 2100, 2006, 2021);
            s.store_scaled_add(2103, 2045, 2092, 0.5);
            s.store_scalar(2104, 0.0);
            s.store_mul(1919, 2098, 2051);
        }

        s.b[2195] = (s.v[1919] > 0.0);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2195]) {
            s.store_sqrt(2104, 1919);
        }

        if s.b[2180] {
            s.store_scaled_add(2105, 2052, 2101, 0.5);
            s.store_add_scaled_product_mixed_iaa(2106, 2105, 1.0, A::square(s.ad_value(2093)), A::sub_scaled_inputs(s.ad_value(2104), 1.0, s.ad_value(2023), 2.0), 0.125);
        }

        s.b[2196] = (s.v[2103] < 1e-5);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2196]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2107, 2103, 1.0, 2103, 1.0, 2103, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2197] = (s.v[724] > 0.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((s.b[2180] && s.b[2196]) && s.b[2197]) {
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
        }

        if (s.b[2180] && s.b[2196]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2103), 1.0, A::scale(s.ad_value(2103), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2110, 2103, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2111, 2109, A::div_scaled_product(s.ad_value(2006), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2103), 0.5)), 1.0, A::square(s.ad_value(2103)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2180] && (!s.b[2196])) {
            s.store_add_offset_lhs(2107, 2103, (-1.0), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
        }

        s.b[2198] = (s.v[724] > 0.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if ((s.b[2180] && (!s.b[2196])) && s.b[2198]) {
            s.store_add_scaled_sub_value_product_indices(2112, 1.0, 2104, 1.0, 2108, 2023, 2.0);
            s.store_div_from_scalar_sqrt_ad(2109, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2108)), 1.0));
            s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2109), 1.0, s.ad_value(2109), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2113, 724, A::square(s.ad_value(1919)), 2007, 2106, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2114, 2108, 2.0, 2113, (-2.0), 2007, A::add(A::sub_from_scalar(1.0, s.ad_value(2104)), s.ad_value(2106)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2115, 2113, s.ad_value(2113), 1.0, s.ad_value(2108), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2116, 1.0, 2007, A::add(s.ad_value(2104), s.ad_value(2106)), 0.5);
            s.store_div_scaled_product_denominator_ad(2117, 2115, 2114, 1.0, A::add_scaled_square_product(s.ad_value(2114), 1.0, s.ad_value(2116), s.ad_value(2115), (-1.0)), 1.0);
            s.store_add(2103, 2103, 2117);
            s.store_exp(2118, 2117);
            s.store_div(2104, 2104, 2118);
            s.store_mul(2106, 2106, 2118);
            s.store_add_offset_lhs(2107, 2103, (-1.0), 2104);
            s.store_mul_sqrt_ad_rhs(2108, 2006, A::add(s.ad_value(2106), s.ad_value(2107)));
            s.store_add_ad(2119, A::sub_from_scalar(1.0, s.ad_value(2104)), A::mul3_scaled_output(s.ad_value(2108), s.ad_value(2109), s.ad_value(2023), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2093, 2093, 2118, A::add(s.ad_value(2112), s.ad_value(2105)), 1.0, A::add_scaled_product(s.ad_value(2119), 1.0, s.ad_value(2118), s.ad_value(2105), 1.0), 1.0);
            s.store_mul(2096, 2093, 2021);
        }

        if (s.b[2180] && (!s.b[2196])) {
            s.store_sqrt(2110, 2107);
            s.store_add_scaled_inputs_ad_rhs(2111, 2109, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2006), 1.0, s.ad_value(2104)), s.ad_value(2110)), 0.5);
        }

        if s.b[2180] {
            s.store_mul_div_scaled_product_mixed_iiia(2120, 2021, 2007, 2106, 1.0, A::add_scaled_product(s.ad_value(2108), 1.0, s.ad_value(2006), s.ad_value(2110), 1.0), 1.0);
            s.store_add_scaled_product_indices(2121, 2120, 1.0, 2021, 2111, 1.0);
            s.store_mul3_lhs(2122, 2110, 2006, 2021);
        }

        s.b[2199] = (s.v[213] < 0.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2199]) {
            s.store_sub_from_scalar_scaled_mul(2061, 1.0, 213, 2120, 1.0);
        }

        if (s.b[2180] && (!s.b[2199])) {
            s.store_div_from_scalar_offset_product(2061, 1.0, 213, 2120, 1.0);
        }

        if s.b[2180] {
            s.store_mul_product3_indices(2062, 2120, 751, 2060, 2061, 1.0);
            s.store_add_scaled_product_indices(2123, 2122, 1.0, 769, 2120, 1.0);
            s.store_add_scaled_product_indices(2124, 2122, 1.0, 770, 2120, 1.0);
            s.store_mul(2125, 768, 2123);
            s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2107), 1.0, A::add(s.ad_value(2107), s.ad_value(2106)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2064, A::pow(A::mul(s.ad_value(2125), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);
            s.store_mul_add_ad_lhs(2126, A::offset(s.ad_value(2064), 1.0), s.ad_value(2062), 2056);
            s.store_ln_ad(2127, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2096)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2087), s.ad_value(2096)), s.ad_value(773)), 1.0), 1.0));
            s.store_mul(1921, 2120, 2066);
            s.store_div_add_scaled_inputs_rhs_indices(2067, 1921, 218, 1.0, 1921, 1.0);
        }

        s.b[2200] = (s.v[217] < 0.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if (s.b[2180] && s.b[2200]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2068, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2067)));
        }

        if (s.b[2180] && (!s.b[2200])) {
            s.store_offset_mul(2068, 217, 2067, 1.0);
        }

        if s.b[2180] {
            s.store_mul(2129, 2001, 2068);
            s.store_mul(2128, 2108, 2021);
        }

        s.copy_ad(1843, 2069);

        s.copy_ad(1845, 2087);

        s.copy_ad(1846, 2088);

        s.copy_ad(1847, 2093);

        s.copy_ad(1848, 2096);

        s.copy_ad(1850, 2103);

        s.copy_ad(1849, 2102);

        s.copy_ad(1851, 2109);

        s.copy_ad(1852, 2111);

        s.copy_ad(1853, 2120);

        s.copy_ad(1854, 2121);

        s.copy_ad(1855, 2122);

        s.copy_ad(1856, 2124);

        s.copy_ad(1857, 2126);

        s.copy_ad(1859, 2127);

        s.copy_ad(1858, 2129);

        s.copy_ad(1860, 2128);

        s.v[1861] = 1.0;

        s.v[1862] = 1.0;

        s.v[1864] = 1.0;

        s.v[1865] = 1.0;

        s.v[832] = 0.0;

        s.b[2201] = (s.v[1817] > 0.0);
        s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };

        if s.b[2201] {
            s.store_ln_ad(1929, A::offset(A::mul(s.ad_value(824), s.ad_value(773)), 1.0));
            s.store_div_scaled_product_indices(1919, 1812, 1852, 1.0, 1854, 1.0);
            s.store_add_scaled_product_mixed_aai(1928, A::mul3(A::mul3(s.ad_value(222), s.ad_value(1855), s.ad_value(1919)), s.ad_value(1919), s.ad_value(1929)), 1.0, A::div_scaled_product(A::add(s.ad_value(220), A::div(s.ad_value(221), s.ad_value(1854))), s.ad_value(1853), 1.0, s.ad_value(1854), 1.0), 1859, 1.0);
            s.store_div_from_scalar_add_ad(1861, 1.0, A::offset(s.ad_value(1928), 1.0), A::square(s.ad_value(1928)));
            s.store_mul(1862, 1857, 1861);
            s.store_div(1863, 1858, 1862);
            s.store_mul_ad_product_lhs_mixed_ai(1930, A::square(s.ad_value(1863)), 1848, 1848);
        }

        s.b[2202] = (s.v[0] == (-1.0));
        s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };

        if (s.b[2201] && s.b[2202]) {
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(1930), 1.0, A::mul(s.ad_value(1863), s.ad_value(1848)), 1.0, 1.0);
        }

        if s.b[2201] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1931, 1862, A::sqrt(A::scale_offset(s.ad_value(1930), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1864, 1.0, 1931);
            s.store_mul(1919, 1862, 1864);
            s.store_mul_offset_ad_rhs(1932, 1852, A::mul3_scaled_output(s.ad_value(1930), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_div_scaled_product_indices(1865, 1919, 1854, 1.0, 1932, 1.0);
            s.store_mul_product3_indices(832, 1864, 710, 1854, 1848, 1.0);
        }

        s.v[1934] = 0.0;

        s.v[1935] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 0.0;

        s.b[2203] = (((((p.p40 != 0.0) && ((s.v[232] > 0.0) || (s.v[233] > 0.0))) || ((p.p42 != 0.0) && ((s.v[242] > 0.0) || (s.v[243] > 0.0)))) || (s.v[257] > 0.0)) || (s.v[258] > 0.0));
        s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };

        if s.b[2203] {
            s.store_scaled_add_ad_rhs(1933, 1805, A::sqrt(A::add(A::square(s.ad_value(1805)), s.ad_value(783))), 0.5);
            s.store_add_ad_lhs(1934, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(778), (-0.5), s.ad_value(776), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(778), 0.25, s.ad_value(784), 1.0)), 1.0), 785);
            s.store_scaled_add_ad_rhs(1933, 1806, A::sqrt(A::add(A::square(s.ad_value(1806)), s.ad_value(786))), 0.5);
            s.store_add_ad_lhs(1935, A::add_scaled_inputs_product(s.ad_value(1933), -1.0, s.ad_value(779), (-0.5), s.ad_value(777), A::sqrt(A::add_scaled_inputs3(s.ad_value(1933), 1.0, s.ad_value(779), 0.25, s.ad_value(787), 1.0)), 1.0), 788);
            s.store_scaled_add(1866, 1805, 1934, (-s.v[348]));
            s.store_scaled_add(1867, 1806, 1935, (-s.v[348]));
        }

        s.v[833] = 0.0;

        s.v[834] = 0.0;

        s.v[1962] = 0.0;

        s.v[837] = 0.0;

        s.v[835] = 0.0;

        s.v[836] = 0.0;

        s.b[2204] = (p.p40 != 0.0);
        s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };

        s.b[2205] = (s.v[232] > 0.0);
        s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1866)), 1e-6), 789);
        }

        s.b[2206] = (s.v[238] < 0.0);
        s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2205]) && s.b[2206]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 795, 0.5, 1936, 795, 1e-6, (-0.5));
        }

        if (s.b[2204] && s.b[2205]) {
            s.store_mul_offset_ad_rhs(1919, 792, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(237), 1.0, s.ad_value(238), s.ad_value(1936), 1.0)), (-1.5));
        }

        s.b[2207] = (s.v[1919] > 0.0);
        s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2205]) && s.b[2207]) {
            s.store_offset_mul_offset_rhs_ad_rhs(1937, 1919, A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);
        }

        s.b[2208] = (s.v[1919] > (-230.25850929940458));
        s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2205]) && (!s.b[2207])) && s.b[2208]) {
            s.store_exp(1937, 1919);
        }

        if (((s.b[2204] && s.b[2205]) && (!s.b[2207])) && (!s.b[2208])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1937, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2204] && s.b[2205]) {
            s.store_offset(1938, 1934, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 828, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1941, 2.0, 812, A::add(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1939), s.ad_value(1919))))));
            s.store_mul3_lhs(833, 232, 1937, 1941);
        }

        s.b[2209] = (s.v[233] > 0.0);
        s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1867)), 1e-6), 789);
        }

        s.b[2210] = (s.v[240] < 0.0);
        s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2209]) && s.b[2210]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1936, 1936, 0.5, 796, 0.5, 1936, 796, 1e-6, (-0.5));
        }

        if (s.b[2204] && s.b[2209]) {
            s.store_mul_offset_ad_rhs(1919, 793, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(239), 1.0, s.ad_value(240), s.ad_value(1936), 1.0)), (-1.5));
        }

        s.b[2211] = (s.v[1919] > 0.0);
        s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2209]) && s.b[2211]) {
            s.store_offset_mul_offset_rhs_ad_rhs(1937, 1919, A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);
        }

        s.b[2212] = (s.v[1919] > (-230.25850929940458));
        s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && s.b[2212]) {
            s.store_exp(1937, 1919);
        }

        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && (!s.b[2212])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1937, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2204] && s.b[2209]) {
            s.store_offset(1938, 1935, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 831, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1919, 2.0, 812, A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1941, 2.0, 812, A::add(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1939), s.ad_value(1919))))));
            s.store_mul3_lhs(834, 233, 1937, 1941);
        }

        s.b[2213] = (s.v[231] > 0.0);
        s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };

        s.b[2214] = (s.v[1817] <= 0.0);
        s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {
            s.store_offset(1919, 771, 1.0);
            s.store_div_scaled_product_left_ad(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 1843, 1.0);
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1846, 1843, 1813, 1919, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0);
        }

        s.b[2215] = ((s.v[1847] - s.v[1846]) > (-230.25850929940458));
        s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2215]) {
            s.store_exp_sub(1919, 1847, 1846);
        }

    }
}
