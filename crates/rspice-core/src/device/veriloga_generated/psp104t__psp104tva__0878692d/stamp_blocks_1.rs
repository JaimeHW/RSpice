#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && (!s.b[1426])) {
            s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1411])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1428] = (s.v[649] == 0.0);
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1428]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1428])) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.b[1429] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1429]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1430] = (p.p850 == 0.5);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && s.b[1430]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && (!s.b[1430])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1431] = (p.p850 == 0.5);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && s.b[1431]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && (!s.b[1431])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_scale(1198, 1191, s.v[426]);
            s.store_ad_value(1199, A::mul_offset_lhs_scaled_output(s.ad_value(1185), (-1.0), s.ad_value(1198), s.v[387]));
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.b[1432] = (p.p864 == 0.0);
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1432]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[411] * s.v[441]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1433] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1433]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(s.ad_value(1204), (-s.v[438]), s.ad_value(1202), s.ad_value(1205), s.v[438], s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1434] = (s.v[1212] > 0.0);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1434]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1434])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1435] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1435]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1435])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1436] = (s.v[1212] > 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1436]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1437] = (s.v[1211] > (-230.25850929940458));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) && s.b[1437]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1437])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[438] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);
        }

        s.b[1438] = (p.p870 == 0.0);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1438]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1439] = (p.p850 == 0.5);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && s.b[1439]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1439])) {
            s.store_powf_ad(1191, A::scale_offset(s.ad_value(1189), (-s.v[432]), ((p.p847) * (s.v[432]))), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) {
            s.store_scaled_div_ad_lhs(1216, A::scale_offset(s.ad_value(1189), (-s.v[429]), ((p.p847) * (s.v[429]))), 1191, s.v[414]);
        }

        s.b[1440] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && s.b[1440]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1441] = (((-s.v[444]) / s.v[1216]) < 0.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1440])) && s.b[1441]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1440])) && (!s.b[1441])) {
            let assign23900_ad_e27764: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign23900_ad_e27764, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870);
        }

        s.b[1442] = (p.p879 > 1000.0);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1442]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1443] = (s.v[1190] > ((-s.v[445]) * p.p879));
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        s.b[1444] = (p.p882 == 4.0);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) && s.b[1444]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) && (!s.b[1444])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && (!s.b[1443])) {
            s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1428])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(479, A::add_scaled_products3(s.ad_value(647), s.ad_value(1218), 1.0, s.ad_value(648), s.ad_value(1219), 1.0, s.ad_value(649), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1445] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        s.b[1446] = (s.v[490] < s.v[655]);
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        s.b[1447] = (((((-0.5) * (s.v[490] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && s.b[1447]) {
            s.store_exp_scaled_input(1184, 490, (s.v[372] * (-0.5)));
        }

        s.b[1448] = (((-0.5) * (s.v[490] * s.v[372])) < 0.0);
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && (!s.b[1447])) && s.b[1448]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && (!s.b[1447])) && (!s.b[1448])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(490), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && (!s.b[1446])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(490), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1445]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1449] = (s.v[490] > 0.0);
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1449]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && (!s.b[1449])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 490);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1445]) {
            s.store_sub(1187, 657, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(490), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(1187)), A::sub(s.ad_value(490), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(490), 0.5, s.ad_value(660), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(660)), A::sub(s.ad_value(490), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 490, A::sqrt(A::offset(A::mul(s.ad_value(490), s.ad_value(490)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1450] = (s.v[647] == 0.0);
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1450]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1450])) {
            s.store_scale(1192, 1182, s.v[388]);
        }

        s.b[1451] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1451]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {
            s.store_sub_from_scalar(1194, s.v[394], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1452] = (p.p848 == 0.5);
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && s.b[1452]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && (!s.b[1452])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1453] = (p.p848 == 0.5);
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && s.b[1453]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && (!s.b[1453])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[430]), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {
            s.store_scale(1198, 1191, s.v[424]);
            s.store_ad_value(1199, A::mul_offset_lhs_scaled_output(s.ad_value(1185), (-1.0), s.ad_value(1198), s.v[385]));
            s.store_scaled_mul(1193, 1199, 1197, p.p857);
        }

        s.b[1454] = (p.p862 == 0.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1454]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1455] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1455]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1455])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(s.ad_value(1204), (-s.v[436]), s.ad_value(1202), s.ad_value(1205), s.v[436], s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1456] = (s.v[1212] > 0.0);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1456]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1456])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1457] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1457]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1457])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1458] = (s.v[1212] > 0.0);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1458]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1459] = (s.v[1211] > (-230.25850929940458));
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) && s.b[1459]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1459])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);
        }

        s.b[1460] = (p.p868 == 0.0);
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1460]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1461] = (p.p848 == 0.5);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && s.b[1461]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1461])) {
            s.store_powf_ad(1191, A::scale_offset(s.ad_value(1189), (-s.v[430]), ((p.p845) * (s.v[430]))), p.p848);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) {
            s.store_scaled_div_ad_lhs(1216, A::scale_offset(s.ad_value(1189), (-s.v[427]), ((p.p845) * (s.v[427]))), 1191, s.v[412]);
        }

        s.b[1462] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && s.b[1462]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1463] = (((-s.v[442]) / s.v[1216]) < 0.0);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1462])) && s.b[1463]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1462])) && (!s.b[1463])) {
            let assign24900_ad_e29408: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign24900_ad_e29408, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868);
        }

        s.b[1464] = (p.p877 > 1000.0);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1464]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1465] = (s.v[1190] > ((-s.v[445]) * p.p877));
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        s.b[1466] = (p.p880 == 4.0);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) && s.b[1466]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) && (!s.b[1466])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && (!s.b[1465])) {
            s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1450])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1467] = (s.v[648] == 0.0);
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1467]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1467])) {
            s.store_scale(1192, 1182, s.v[389]);
        }

        s.b[1468] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {
            s.store_sub_from_scalar(1194, s.v[395], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1469] = (p.p849 == 0.5);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && s.b[1469]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && (!s.b[1469])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1470] = (p.p849 == 0.5);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && s.b[1470]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && (!s.b[1470])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[431]), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {
            s.store_scale(1198, 1191, s.v[425]);
            s.store_ad_value(1199, A::mul_offset_lhs_scaled_output(s.ad_value(1185), (-1.0), s.ad_value(1198), s.v[386]));
            s.store_scaled_mul(1193, 1199, 1197, p.p858);
        }

        s.b[1471] = (p.p863 == 0.0);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1471]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1472] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(s.ad_value(1204), (-s.v[437]), s.ad_value(1202), s.ad_value(1205), s.v[437], s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1473] = (s.v[1212] > 0.0);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1473]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1473])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1474] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1474]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1474])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1475] = (s.v[1212] > 0.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1475]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1476] = (s.v[1211] > (-230.25850929940458));
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) && s.b[1476]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1476])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);
        }

        s.b[1477] = (p.p869 == 0.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1477]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1478] = (p.p849 == 0.5);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && s.b[1478]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1478])) {
            s.store_powf_ad(1191, A::scale_offset(s.ad_value(1189), (-s.v[431]), ((p.p846) * (s.v[431]))), p.p849);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) {
            s.store_scaled_div_ad_lhs(1216, A::scale_offset(s.ad_value(1189), (-s.v[428]), ((p.p846) * (s.v[428]))), 1191, s.v[413]);
        }

        s.b[1479] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && s.b[1479]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1480] = (((-s.v[443]) / s.v[1216]) < 0.0);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1479])) && s.b[1480]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1479])) && (!s.b[1480])) {
            let assign25600_ad_e30551: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign25600_ad_e30551, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869);
        }

        s.b[1481] = (p.p878 > 1000.0);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1481]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1482] = (s.v[1190] > ((-s.v[445]) * p.p878));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        s.b[1483] = (p.p881 == 4.0);
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) && s.b[1483]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) && (!s.b[1483])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && (!s.b[1482])) {
            s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1467])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1484] = (s.v[649] == 0.0);
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1484]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1484])) {
            s.store_scale(1192, 1182, s.v[390]);
        }

        s.b[1485] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1485]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {
            s.store_sub_from_scalar(1194, s.v[396], 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1486] = (p.p850 == 0.5);
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && s.b[1486]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && (!s.b[1486])) {
            s.store_scaled_add_ad_lhs(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1487] = (p.p850 == 0.5);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && s.b[1487]) {
            s.store_sqrt_scaled_input(1191, 1194, s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && (!s.b[1487])) {
            s.store_powf_ad(1191, A::scale(s.ad_value(1194), s.v[432]), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {
            s.store_scale(1198, 1191, s.v[426]);
            s.store_ad_value(1199, A::mul_offset_lhs_scaled_output(s.ad_value(1185), (-1.0), s.ad_value(1198), s.v[387]));
            s.store_scaled_mul(1193, 1199, 1197, p.p859);
        }

        s.b[1488] = (p.p864 == 0.0);
        s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1488]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {
            s.store_scaled_div(1201, 1198, 1194, (s.v[411] * s.v[441]));
            s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1489] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1489]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(s.ad_value(1204), (-s.v[438]), s.ad_value(1202), s.ad_value(1205), s.v[438], s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1490] = (s.v[1212] > 0.0);
        s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1490]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1490])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1491] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1491]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1491])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1492] = (s.v[1212] > 0.0);
        s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1492]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1493] = (s.v[1211] > (-230.25850929940458));
        s.v[1493] = if s.b[1493] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) && s.b[1493]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1493])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {
            s.store_scaled_div(1214, 1213, 1209, (s.v[438] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);
        }

        s.b[1494] = (p.p870 == 0.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1494]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1495] = (p.p850 == 0.5);
        s.v[1495] = if s.b[1495] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && s.b[1495]) {
            s.store_sqrt_scaled_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1495])) {
            s.store_powf_ad(1191, A::scale_offset(s.ad_value(1189), (-s.v[432]), ((p.p847) * (s.v[432]))), p.p850);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) {
            s.store_scaled_div_ad_lhs(1216, A::scale_offset(s.ad_value(1189), (-s.v[429]), ((p.p847) * (s.v[429]))), 1191, s.v[414]);
        }

        s.b[1496] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && s.b[1496]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1497] = (((-s.v[444]) / s.v[1216]) < 0.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1496])) && s.b[1497]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1496])) && (!s.b[1497])) {
            let assign26300_ad_e31694: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign26300_ad_e31694, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) {
            s.store_mul_scaled_ad_lhs(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870);
        }

        s.b[1498] = (p.p879 > 1000.0);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1498]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1499] = (s.v[1190] > ((-s.v[445]) * p.p879));
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        s.b[1500] = (p.p882 == 4.0);
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) && s.b[1500]) {
            s.store_mul_scaled_ad_lhs(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451]);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) && (!s.b[1500])) {
            s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && (!s.b[1499])) {
            s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1484])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(480, A::add_scaled_products3(s.ad_value(647), s.ad_value(1218), 1.0, s.ad_value(648), s.ad_value(1219), 1.0, s.ad_value(649), s.ad_value(1220), 1.0));
            s.store_ad_value(668, A::add_scaled_inputs3(s.ad_value(647), s.v[388], s.ad_value(648), s.v[389], s.ad_value(649), s.v[390]));
            s.store_ad_value(484, A::add_scaled_offset_product_rhs(s.ad_value(479), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(489), (s.v[372] * s.v[669])), (-1.0), (-1.0)));
            s.store_ad_value(485, A::add_scaled_offset_product_rhs(s.ad_value(480), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(490), (s.v[372] * s.v[669])), (-1.0), (-1.0)));
        }

        s.b[1501] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        s.b[1502] = ((s.v[479] > 0.0) && (s.v[480] > 0.0));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        s.b[1503] = ((((((s.v[484] / s.v[479]) > 0.001) || ((s.v[485] / s.v[480]) > 0.001)) && (s.v[484] > 0.0)) && (s.v[485] > 0.0)) && (s.v[485] > s.v[484]));
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1502]) && s.b[1503]) {
            s.store_div(491, 484, 485);
            s.store_ad_value(671, A::div_scaled_inputs(A::ln(s.ad_value(491)), s.v[371], A::sub(s.ad_value(489), s.ad_value(490)), 1.0));
            s.store_div_ad_rhs(670, 484, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(489), s.v[372], s.ad_value(671))), (-1.0)));
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1501]) {
            s.store_ad_value(481, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(486), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, s.ad_value(670), A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[372], s.ad_value(671))), (-1.0), (-1.0)));
            s.store_ad_value(482, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(487), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, s.ad_value(670), A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[372], s.ad_value(671))), (-1.0), (-1.0)));
            s.store_ad_value(483, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(478), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(488), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, s.ad_value(670), A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[372], s.ad_value(671))), (-1.0), (-1.0)));
        }

        s.b[1504] = (((s.v[476] < 0.0) && (s.v[477] < 0.0)) && (s.v[478] < 0.0));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        s.b[1505] = (((((((s.v[481] / s.v[476]) > 0.001) || ((s.v[482] / s.v[477]) > 0.001)) || ((s.v[483] / s.v[478]) > 0.001)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0)) && (s.v[483] < 0.0));
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) {
            s.store_div(491, 481, 482);
            s.store_ad_value(492, A::div_scaled_inputs(A::ln(s.ad_value(491)), (-s.v[371]), A::sub(s.ad_value(486), s.ad_value(487)), 1.0));
            s.store_div_ad_rhs(494, 487, A::sub(s.ad_value(487), s.ad_value(486)));
            s.store_scaled_mul_ad(495, A::offset(s.ad_value(491), (-1.0)), A::offset(A::pow(s.ad_value(491), s.ad_value(494)), (-1.0)), s.v[371]);
            s.store_div_ad_rhs(494, 486, A::sub(s.ad_value(486), s.ad_value(487)));
            s.store_sub_ad_lhs(496, A::add_scaled_products(A::pow(s.ad_value(491), s.ad_value(494)), A::sub(s.ad_value(487), s.ad_value(486)), 1.0, s.ad_value(491), s.ad_value(486), 1.0), 487);
            s.store_div(493, 495, 496);
            s.store_add(673, 492, 493);
        }

        s.b[1506] = (((((s.v[488] * s.v[372]) * s.v[673])) as f64).abs() < 1e-6);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && s.b[1506]) {
            s.store_scalar(667, 1.0);
            s.store_mul_ad_rhs(672, 483, A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(488)), 1.0, s.ad_value(673), (0.5 * s.v[372])));
            s.store_ad_value(673, A::div_scaled_product(s.ad_value(483), s.ad_value(673), ((-0.5) * s.v[372]), s.ad_value(488), 1.0));
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {
            s.store_scalar(667, 0.0);
            s.store_ad_value(672, A::div_scaled_inputs(s.ad_value(483), -1.0, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(488), (-s.v[372]), s.ad_value(673))), (-1.0)), 1.0));
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(502, A::add_scaled_inputs3(s.ad_value(647), (s.v[415] * p.p946), s.ad_value(648), (s.v[416] * p.p946), s.ad_value(649), (s.v[417] * p.p946)));
        }

        s.b[1507] = ((s.v[647] * s.v[415]) <= s.v[502]);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1507]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1508] = ((s.v[648] * s.v[416]) <= s.v[502]);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1508]) {
            s.store_scalar(653, 0.0);
        }

        s.b[1509] = ((s.v[649] * s.v[417]) <= s.v[502]);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1509]) {
            s.store_scalar(654, 0.0);
        }

        s.b[1510] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1510]) {
            s.store_ln_ad(661, A::div_from_scalar((0.5 * p.p839), A::offset(s.ad_value(668), 1e-21)));
            s.store_ln_ad(663, A::div_from_scalar((0.5 * p.p839), A::offset(s.ad_value(670), 1e-21)));
            s.store_ln_ad(665, A::div_from_scalar((0.5 * p.p839), A::offset(A::abs(s.ad_value(672)), 1e-21)));
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_min_with_scalar(661, 661, 230.25850929940458);
            s.store_exp(662, 661);
            s.store_min_with_scalar(663, 663, 230.25850929940458);
            s.store_exp(664, 663);
            s.store_min_with_scalar(665, 665, 230.25850929940458);
            s.store_exp(666, 665);
            s.store_scalar(499, 0.4);
            s.store_scalar(500, 0.65);
            s.store_scalar(501, 0.8);
            s.store_mul_neg_lhs(486, 499, 553);
            s.store_mul_neg_lhs(487, 500, 553);
            s.store_mul_neg_lhs(488, 501, 553);
            s.store_scalar(489, 0.1);
            s.store_scalar(490, 0.2);
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1511] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1511] = if s.b[1511] { 1.0 } else { 0.0 };

        s.b[1512] = (s.v[486] < s.v[682]);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        s.b[1513] = (((((-0.5) * (s.v[486] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && s.b[1513]) {
            s.store_exp_scaled_input(1184, 486, (s.v[372] * (-0.5)));
        }

        s.b[1514] = (((-0.5) * (s.v[486] * s.v[372])) < 0.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && (!s.b[1513])) && s.b[1514]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && (!s.b[1513])) && (!s.b[1514])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && (!s.b[1512])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(486), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1511]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1515] = (s.v[486] > 0.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1515]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && (!s.b[1515])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 486);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1511]) {
            s.store_sub(1187, 684, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(486), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(1187)), A::sub(s.ad_value(486), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(486), 0.5, s.ad_value(687), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(486), s.ad_value(687)), A::sub(s.ad_value(486), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 486, A::sqrt(A::offset(A::mul(s.ad_value(486), s.ad_value(486)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1516] = (s.v[674] == 0.0);
        s.v[1516] = if s.b[1516] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1516]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1516])) {
            s.store_mul(1192, 564, 1182);
        }

        s.b[1517] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1517] = if s.b[1517] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1517]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {
            s.store_sub(1194, 570, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1518] = (s.v[512] == 0.5);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && s.b[1518]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && (!s.b[1518])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1519] = (s.v[512] == 0.5);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && s.b[1519]) {
            s.store_sqrt_mul(1191, 1194, 597);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && (!s.b[1519])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {
            s.store_mul(1198, 591, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(561), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 523, 1199, 1197);
        }

        s.b[1520] = (s.v[526] == 0.0);
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1520]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {
            s.store_mul_ad_rhs(1201, 606, A::div_scaled_product(s.ad_value(1198), s.ad_value(576), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 603, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
        }

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1521] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1521]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1521])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(603), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1522] = (s.v[1212] > 0.0);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1522]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1522])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1523] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1523] = if s.b[1523] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1523]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1523])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1524] = (s.v[1212] > 0.0);
        s.v[1524] = if s.b[1524] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1524]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1525] = (s.v[1211] > (-230.25850929940458));
        s.v[1525] = if s.b[1525] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) && s.b[1525]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1525])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(603), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 526, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1526] = (s.v[532] == 0.0);
        s.v[1526] = if s.b[1526] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1526]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1527] = (s.v[512] == 0.5);
        s.v[1527] = if s.b[1527] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && s.b[1527]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1527])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) {
            s.store_mul_ad_rhs(1216, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(594), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1528] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1528] = if s.b[1528] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && s.b[1528]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1529] = (((-s.v[609]) / s.v[1216]) < 0.0);
        s.v[1529] = if s.b[1529] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1528])) && s.b[1529]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1528])) && (!s.b[1529])) {
            let assign27830_ad_e34081: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign27830_ad_e34081, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(532), A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1530] = (s.v[541] > 1000.0);
        s.v[1530] = if s.b[1530] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1530]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1531] = (s.v[1190] > ((-s.v[445]) * s.v[541]));
        s.v[1531] = if s.b[1531] { 1.0 } else { 0.0 };

        s.b[1532] = (s.v[544] == 4.0);
        s.v[1532] = if s.b[1532] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) && s.b[1532]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(615), A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), s.ad_value(1190), 615);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) && (!s.b[1532])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && (!s.b[1531])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), s.ad_value(618), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1516])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1533] = (s.v[675] == 0.0);
        s.v[1533] = if s.b[1533] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1533]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1533])) {
            s.store_mul(1192, 565, 1182);
        }

        s.b[1534] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1534] = if s.b[1534] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1534]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {
            s.store_sub(1194, 571, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1535] = (s.v[513] == 0.5);
        s.v[1535] = if s.b[1535] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && s.b[1535]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && (!s.b[1535])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1536] = (s.v[513] == 0.5);
        s.v[1536] = if s.b[1536] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && s.b[1536]) {
            s.store_sqrt_mul(1191, 1194, 598);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && (!s.b[1536])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {
            s.store_mul(1198, 592, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(562), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 524, 1199, 1197);
        }

        s.b[1537] = (s.v[527] == 0.0);
        s.v[1537] = if s.b[1537] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1537]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {
            s.store_mul_ad_rhs(1201, 607, A::div_scaled_product(s.ad_value(1198), s.ad_value(577), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 604, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1538] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1538] = if s.b[1538] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1538]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1538])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(604), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1539] = (s.v[1212] > 0.0);
        s.v[1539] = if s.b[1539] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1539]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1539])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1540] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1540] = if s.b[1540] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1540]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1540])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1541] = (s.v[1212] > 0.0);
        s.v[1541] = if s.b[1541] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1541]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1542] = (s.v[1211] > (-230.25850929940458));
        s.v[1542] = if s.b[1542] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) && s.b[1542]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1542])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(604), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 527, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1543] = (s.v[533] == 0.0);
        s.v[1543] = if s.b[1543] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1543]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1544] = (s.v[513] == 0.5);
        s.v[1544] = if s.b[1544] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && s.b[1544]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1544])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) {
            s.store_mul_ad_rhs(1216, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(595), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1545] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1545] = if s.b[1545] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && s.b[1545]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1546] = (((-s.v[610]) / s.v[1216]) < 0.0);
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1545])) && s.b[1546]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1545])) && (!s.b[1546])) {
            let assign28530_ad_e35224: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign28530_ad_e35224, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(533), A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1547] = (s.v[542] > 1000.0);
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1547]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1548] = (s.v[1190] > ((-s.v[445]) * s.v[542]));
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        s.b[1549] = (s.v[545] == 4.0);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) && s.b[1549]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(616), A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), s.ad_value(1190), 616);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) && (!s.b[1549])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && (!s.b[1548])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), s.ad_value(619), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1533])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1550] = (s.v[676] == 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1550]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1550])) {
            s.store_mul(1192, 566, 1182);
        }

        s.b[1551] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1551]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {
            s.store_sub(1194, 572, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1552] = (s.v[514] == 0.5);
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && s.b[1552]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && (!s.b[1552])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1553] = (s.v[514] == 0.5);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && s.b[1553]) {
            s.store_sqrt_mul(1191, 1194, 599);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && (!s.b[1553])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {
            s.store_mul(1198, 593, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(563), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 525, 1199, 1197);
        }

        s.b[1554] = (s.v[528] == 0.0);
        s.v[1554] = if s.b[1554] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1554]) {
            s.store_scalar(1200, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {
            s.store_mul_ad_rhs(1201, 608, A::div_scaled_product(s.ad_value(1198), s.ad_value(578), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 605, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1555] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1555]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1555])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(605), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1556] = (s.v[1212] > 0.0);
        s.v[1556] = if s.b[1556] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1556]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1556])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1557] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1557] = if s.b[1557] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1557]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1557])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1558] = (s.v[1212] > 0.0);
        s.v[1558] = if s.b[1558] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1558]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1559] = (s.v[1211] > (-230.25850929940458));
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) && s.b[1559]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1559])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(605), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 528, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1560] = (s.v[534] == 0.0);
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1560]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1561] = (s.v[514] == 0.5);
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && s.b[1561]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1561])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) {
            s.store_mul_ad_rhs(1216, 581, A::div_scaled_product(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(596), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1562] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && s.b[1562]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1563] = (((-s.v[611]) / s.v[1216]) < 0.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1562])) && s.b[1563]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1562])) && (!s.b[1563])) {
            let assign29230_ad_e36367: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign29230_ad_e36367, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(534), A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1564] = (s.v[543] > 1000.0);
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1564]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1565] = (s.v[1190] > ((-s.v[445]) * s.v[543]));
        s.v[1565] = if s.b[1565] { 1.0 } else { 0.0 };

        s.b[1566] = (s.v[546] == 4.0);
        s.v[1566] = if s.b[1566] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) && s.b[1566]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(617), A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), s.ad_value(1190), 617);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) && (!s.b[1566])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && (!s.b[1565])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(614), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), s.ad_value(620), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1550])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(476, A::add_scaled_products3(s.ad_value(674), s.ad_value(1218), 1.0, s.ad_value(675), s.ad_value(1219), 1.0, s.ad_value(676), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1567] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1567] = if s.b[1567] { 1.0 } else { 0.0 };

        s.b[1568] = (s.v[487] < s.v[682]);
        s.v[1568] = if s.b[1568] { 1.0 } else { 0.0 };

        s.b[1569] = (((((-0.5) * (s.v[487] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1569] = if s.b[1569] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && s.b[1569]) {
            s.store_exp_scaled_input(1184, 487, (s.v[372] * (-0.5)));
        }

        s.b[1570] = (((-0.5) * (s.v[487] * s.v[372])) < 0.0);
        s.v[1570] = if s.b[1570] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && (!s.b[1569])) && s.b[1570]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && (!s.b[1569])) && (!s.b[1570])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && (!s.b[1568])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(487), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1567]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1571] = (s.v[487] > 0.0);
        s.v[1571] = if s.b[1571] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1571]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && (!s.b[1571])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 487);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1567]) {
            s.store_sub(1187, 684, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(487), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(1187)), A::sub(s.ad_value(487), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(487), 0.5, s.ad_value(687), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(487), s.ad_value(687)), A::sub(s.ad_value(487), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 487, A::sqrt(A::offset(A::mul(s.ad_value(487), s.ad_value(487)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1572] = (s.v[674] == 0.0);
        s.v[1572] = if s.b[1572] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1572]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1572])) {
            s.store_mul(1192, 564, 1182);
        }

        s.b[1573] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1573] = if s.b[1573] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1573]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {
            s.store_sub(1194, 570, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1574] = (s.v[512] == 0.5);
        s.v[1574] = if s.b[1574] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && s.b[1574]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && (!s.b[1574])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1575] = (s.v[512] == 0.5);
        s.v[1575] = if s.b[1575] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && s.b[1575]) {
            s.store_sqrt_mul(1191, 1194, 597);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && (!s.b[1575])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {
            s.store_mul(1198, 591, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(561), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 523, 1199, 1197);
        }

        s.b[1576] = (s.v[526] == 0.0);
        s.v[1576] = if s.b[1576] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1576]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {
            s.store_mul_ad_rhs(1201, 606, A::div_scaled_product(s.ad_value(1198), s.ad_value(576), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 603, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1577] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1577] = if s.b[1577] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1577]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1577])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(603), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1578] = (s.v[1212] > 0.0);
        s.v[1578] = if s.b[1578] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1578]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1578])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1579] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1579] = if s.b[1579] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1579]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1579])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1580] = (s.v[1212] > 0.0);
        s.v[1580] = if s.b[1580] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1580]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1581] = (s.v[1211] > (-230.25850929940458));
        s.v[1581] = if s.b[1581] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) && s.b[1581]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1581])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(603), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 526, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1582] = (s.v[532] == 0.0);
        s.v[1582] = if s.b[1582] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1582]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1583] = (s.v[512] == 0.5);
        s.v[1583] = if s.b[1583] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && s.b[1583]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1583])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) {
            s.store_mul_ad_rhs(1216, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(594), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1584] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1584] = if s.b[1584] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && s.b[1584]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1585] = (((-s.v[609]) / s.v[1216]) < 0.0);
        s.v[1585] = if s.b[1585] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1584])) && s.b[1585]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1584])) && (!s.b[1585])) {
            let assign30230_ad_e38011: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign30230_ad_e38011, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(532), A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

    }

    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1586] = (s.v[541] > 1000.0);
        s.v[1586] = if s.b[1586] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1586]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1587] = (s.v[1190] > ((-s.v[445]) * s.v[541]));
        s.v[1587] = if s.b[1587] { 1.0 } else { 0.0 };

        s.b[1588] = (s.v[544] == 4.0);
        s.v[1588] = if s.b[1588] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) && s.b[1588]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(615), A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), s.ad_value(1190), 615);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) && (!s.b[1588])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && (!s.b[1587])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), s.ad_value(618), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1572])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1589] = (s.v[675] == 0.0);
        s.v[1589] = if s.b[1589] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1589]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1589])) {
            s.store_mul(1192, 565, 1182);
        }

        s.b[1590] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1590] = if s.b[1590] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1590]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {
            s.store_sub(1194, 571, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1591] = (s.v[513] == 0.5);
        s.v[1591] = if s.b[1591] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && s.b[1591]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && (!s.b[1591])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1592] = (s.v[513] == 0.5);
        s.v[1592] = if s.b[1592] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && s.b[1592]) {
            s.store_sqrt_mul(1191, 1194, 598);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && (!s.b[1592])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {
            s.store_mul(1198, 592, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(562), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 524, 1199, 1197);
        }

        s.b[1593] = (s.v[527] == 0.0);
        s.v[1593] = if s.b[1593] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1593]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {
            s.store_mul_ad_rhs(1201, 607, A::div_scaled_product(s.ad_value(1198), s.ad_value(577), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 604, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1594] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1594] = if s.b[1594] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1594]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1594])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(604), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1595] = (s.v[1212] > 0.0);
        s.v[1595] = if s.b[1595] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1595]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1595])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1596] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1596] = if s.b[1596] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1596]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1596])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1597] = (s.v[1212] > 0.0);
        s.v[1597] = if s.b[1597] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1597]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1598] = (s.v[1211] > (-230.25850929940458));
        s.v[1598] = if s.b[1598] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) && s.b[1598]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1598])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(604), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 527, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1599] = (s.v[533] == 0.0);
        s.v[1599] = if s.b[1599] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1599]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1600] = (s.v[513] == 0.5);
        s.v[1600] = if s.b[1600] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && s.b[1600]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1600])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) {
            s.store_mul_ad_rhs(1216, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(595), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1601] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1601] = if s.b[1601] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && s.b[1601]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1602] = (((-s.v[610]) / s.v[1216]) < 0.0);
        s.v[1602] = if s.b[1602] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1601])) && s.b[1602]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1601])) && (!s.b[1602])) {
            let assign30930_ad_e39154: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign30930_ad_e39154, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(533), A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1603] = (s.v[542] > 1000.0);
        s.v[1603] = if s.b[1603] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1603]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1604] = (s.v[1190] > ((-s.v[445]) * s.v[542]));
        s.v[1604] = if s.b[1604] { 1.0 } else { 0.0 };

        s.b[1605] = (s.v[545] == 4.0);
        s.v[1605] = if s.b[1605] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) && s.b[1605]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(616), A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), s.ad_value(1190), 616);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) && (!s.b[1605])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && (!s.b[1604])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), s.ad_value(619), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1589])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1606] = (s.v[676] == 0.0);
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1606]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1606])) {
            s.store_mul(1192, 566, 1182);
        }

        s.b[1607] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1607]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {
            s.store_sub(1194, 572, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1608] = (s.v[514] == 0.5);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && s.b[1608]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1609] = (s.v[514] == 0.5);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && s.b[1609]) {
            s.store_sqrt_mul(1191, 1194, 599);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && (!s.b[1609])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {
            s.store_mul(1198, 593, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(563), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 525, 1199, 1197);
        }

        s.b[1610] = (s.v[528] == 0.0);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1610]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {
            s.store_mul_ad_rhs(1201, 608, A::div_scaled_product(s.ad_value(1198), s.ad_value(578), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 605, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1611] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1611]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1611])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(605), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1612] = (s.v[1212] > 0.0);
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1612]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1612])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1613] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1613]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1613])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1614] = (s.v[1212] > 0.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1614]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1615] = (s.v[1211] > (-230.25850929940458));
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) && s.b[1615]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1615])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(605), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 528, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1616] = (s.v[534] == 0.0);
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1616]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1617] = (s.v[514] == 0.5);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && s.b[1617]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1617])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) {
            s.store_mul_ad_rhs(1216, 581, A::div_scaled_product(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(596), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1618] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && s.b[1618]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1619] = (((-s.v[611]) / s.v[1216]) < 0.0);
        s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1618])) && s.b[1619]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1618])) && (!s.b[1619])) {
            let assign31630_ad_e40297: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign31630_ad_e40297, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(534), A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1620] = (s.v[543] > 1000.0);
        s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1620]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1621] = (s.v[1190] > ((-s.v[445]) * s.v[543]));
        s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };

        s.b[1622] = (s.v[546] == 4.0);
        s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) && s.b[1622]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(617), A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), s.ad_value(1190), 617);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) && (!s.b[1622])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && (!s.b[1621])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(614), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), s.ad_value(620), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1606])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(477, A::add_scaled_products3(s.ad_value(674), s.ad_value(1218), 1.0, s.ad_value(675), s.ad_value(1219), 1.0, s.ad_value(676), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1623] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1623] = if s.b[1623] { 1.0 } else { 0.0 };

        s.b[1624] = (s.v[488] < s.v[682]);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        s.b[1625] = (((((-0.5) * (s.v[488] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1625] = if s.b[1625] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {
            s.store_exp_scaled_input(1184, 488, (s.v[372] * (-0.5)));
        }

        s.b[1626] = (((-0.5) * (s.v[488] * s.v[372])) < 0.0);
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) && s.b[1626]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) && (!s.b[1626])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && (!s.b[1624])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(488), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1623]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1627] = (s.v[488] > 0.0);
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1627]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && (!s.b[1627])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 488);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1623]) {
            s.store_sub(1187, 684, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(488), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(1187)), A::sub(s.ad_value(488), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(488), 0.5, s.ad_value(687), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(488), s.ad_value(687)), A::sub(s.ad_value(488), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 488, A::sqrt(A::offset(A::mul(s.ad_value(488), s.ad_value(488)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1628] = (s.v[674] == 0.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1628]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1628])) {
            s.store_mul(1192, 564, 1182);
        }

        s.b[1629] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1629]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {
            s.store_sub(1194, 570, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1630] = (s.v[512] == 0.5);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && s.b[1630]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && (!s.b[1630])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1631] = (s.v[512] == 0.5);
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && s.b[1631]) {
            s.store_sqrt_mul(1191, 1194, 597);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && (!s.b[1631])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {
            s.store_mul(1198, 591, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(561), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 523, 1199, 1197);
        }

        s.b[1632] = (s.v[526] == 0.0);
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1632]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {
            s.store_mul_ad_rhs(1201, 606, A::div_scaled_product(s.ad_value(1198), s.ad_value(576), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 603, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1633] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1633]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1633])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(603), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1634] = (s.v[1212] > 0.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1634]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1634])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1635] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1635]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1635])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1636] = (s.v[1212] > 0.0);
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1636]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1637] = (s.v[1211] > (-230.25850929940458));
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) && s.b[1637]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1637])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(603), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 526, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1638] = (s.v[532] == 0.0);
        s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1638]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1639] = (s.v[512] == 0.5);
        s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && s.b[1639]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1639])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) {
            s.store_mul_ad_rhs(1216, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(594), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1640] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && s.b[1640]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1641] = (((-s.v[609]) / s.v[1216]) < 0.0);
        s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1640])) && s.b[1641]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) && (!s.b[1640])) && (!s.b[1641])) {
            let assign32630_ad_e41941: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign32630_ad_e41941, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1638])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(532), A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1642] = (s.v[541] > 1000.0);
        s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1642]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1643] = (s.v[1190] > ((-s.v[445]) * s.v[541]));
        s.v[1643] = if s.b[1643] { 1.0 } else { 0.0 };

        s.b[1644] = (s.v[544] == 4.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) && s.b[1644]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(615), A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), s.ad_value(1190), 615);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) && (!s.b[1644])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && s.b[1643]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1642])) && (!s.b[1643])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), s.ad_value(618), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1628])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1645] = (s.v[675] == 0.0);
        s.v[1645] = if s.b[1645] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1645]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1645])) {
            s.store_mul(1192, 565, 1182);
        }

        s.b[1646] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1646]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {
            s.store_sub(1194, 571, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1647] = (s.v[513] == 0.5);
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1647]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1647])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1648] = (s.v[513] == 0.5);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && s.b[1648]) {
            s.store_sqrt_mul(1191, 1194, 598);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) && (!s.b[1648])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1646])) {
            s.store_mul(1198, 592, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(562), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 524, 1199, 1197);
        }

        s.b[1649] = (s.v[527] == 0.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1649]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {
            s.store_mul_ad_rhs(1201, 607, A::div_scaled_product(s.ad_value(1198), s.ad_value(577), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 604, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1650] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1650]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1650])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(604), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1651] = (s.v[1212] > 0.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1651]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1651])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1652] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1652]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1652])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1653] = (s.v[1212] > 0.0);
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && s.b[1653]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1654] = (s.v[1211] > (-230.25850929940458));
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) && s.b[1654]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) && (!s.b[1654])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) && (!s.b[1653])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1649])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(604), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 527, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1655] = (s.v[533] == 0.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1655]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1656] = (s.v[513] == 0.5);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && s.b[1656]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1656])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) {
            s.store_mul_ad_rhs(1216, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(595), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1657] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1657] = if s.b[1657] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && s.b[1657]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1658] = (((-s.v[610]) / s.v[1216]) < 0.0);
        s.v[1658] = if s.b[1658] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1657])) && s.b[1658]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) && (!s.b[1657])) && (!s.b[1658])) {
            let assign33330_ad_e43084: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign33330_ad_e43084, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1655])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(533), A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1659] = (s.v[542] > 1000.0);
        s.v[1659] = if s.b[1659] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1645])) && s.b[1659]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1660] = (s.v[1190] > ((-s.v[445]) * s.v[542]));
        s.v[1660] = if s.b[1660] { 1.0 } else { 0.0 };

        s.b[1661] = (s.v[545] == 4.0);
        s.v[1661] = if s.b[1661] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) && s.b[1661]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(616), A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), s.ad_value(1190), 616);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) && (!s.b[1661])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && s.b[1660]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1645])) && (!s.b[1659])) && (!s.b[1660])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), s.ad_value(619), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1645])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1662] = (s.v[676] == 0.0);
        s.v[1662] = if s.b[1662] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1662]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1662])) {
            s.store_mul(1192, 566, 1182);
        }

        s.b[1663] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[1663] = if s.b[1663] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1663]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_sub(1194, 572, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1664] = (s.v[514] == 0.5);
        s.v[1664] = if s.b[1664] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && s.b[1664]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && (!s.b[1664])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1665] = (s.v[514] == 0.5);
        s.v[1665] = if s.b[1665] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && s.b[1665]) {
            s.store_sqrt_mul(1191, 1194, 599);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) && (!s.b[1665])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1663])) {
            s.store_mul(1198, 593, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(563), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 525, 1199, 1197);
        }

        s.b[1666] = (s.v[528] == 0.0);
        s.v[1666] = if s.b[1666] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1666]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {
            s.store_mul_ad_rhs(1201, 608, A::div_scaled_product(s.ad_value(1198), s.ad_value(578), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 605, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1667] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[1667] = if s.b[1667] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1667]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1667])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(605), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1668] = (s.v[1212] > 0.0);
        s.v[1668] = if s.b[1668] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1668]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1668])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1669] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1669] = if s.b[1669] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1669]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1669])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1670] = (s.v[1212] > 0.0);
        s.v[1670] = if s.b[1670] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && s.b[1670]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1671] = (s.v[1211] > (-230.25850929940458));
        s.v[1671] = if s.b[1671] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) && s.b[1671]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) && (!s.b[1671])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) && (!s.b[1670])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1666])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(605), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 528, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1672] = (s.v[534] == 0.0);
        s.v[1672] = if s.b[1672] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1672]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1673] = (s.v[514] == 0.5);
        s.v[1673] = if s.b[1673] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && s.b[1673]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1673])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) {
            s.store_mul_ad_rhs(1216, 581, A::div_scaled_product(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(596), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1674] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1674] = if s.b[1674] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && s.b[1674]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1675] = (((-s.v[611]) / s.v[1216]) < 0.0);
        s.v[1675] = if s.b[1675] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1674])) && s.b[1675]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) && (!s.b[1674])) && (!s.b[1675])) {
            let assign34030_ad_e44227: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign34030_ad_e44227, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1672])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(534), A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1676] = (s.v[543] > 1000.0);
        s.v[1676] = if s.b[1676] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1662])) && s.b[1676]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1677] = (s.v[1190] > ((-s.v[445]) * s.v[543]));
        s.v[1677] = if s.b[1677] { 1.0 } else { 0.0 };

        s.b[1678] = (s.v[546] == 4.0);
        s.v[1678] = if s.b[1678] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) && s.b[1678]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(617), A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), s.ad_value(1190), 617);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) && (!s.b[1678])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && s.b[1677]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1662])) && (!s.b[1676])) && (!s.b[1677])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(614), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), s.ad_value(620), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1662])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(478, A::add_scaled_products3(s.ad_value(674), s.ad_value(1218), 1.0, s.ad_value(675), s.ad_value(1219), 1.0, s.ad_value(676), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1679] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1679] = if s.b[1679] { 1.0 } else { 0.0 };

        s.b[1680] = (s.v[489] < s.v[682]);
        s.v[1680] = if s.b[1680] { 1.0 } else { 0.0 };

        s.b[1681] = (((((-0.5) * (s.v[489] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1681] = if s.b[1681] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && s.b[1681]) {
            s.store_exp_scaled_input(1184, 489, (s.v[372] * (-0.5)));
        }

        s.b[1682] = (((-0.5) * (s.v[489] * s.v[372])) < 0.0);
        s.v[1682] = if s.b[1682] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && (!s.b[1681])) && s.b[1682]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) && (!s.b[1681])) && (!s.b[1682])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1680]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && (!s.b[1680])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(489), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1679]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1683] = (s.v[489] > 0.0);
        s.v[1683] = if s.b[1683] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && s.b[1683]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1679]) && (!s.b[1683])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 489);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1679]) {
            s.store_sub(1187, 684, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(489), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(1187)), A::sub(s.ad_value(489), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(489), 0.5, s.ad_value(687), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(489), s.ad_value(687)), A::sub(s.ad_value(489), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 489, A::sqrt(A::offset(A::mul(s.ad_value(489), s.ad_value(489)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1684] = (s.v[674] == 0.0);
        s.v[1684] = if s.b[1684] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1684]) {
            s.store_scalar(1218, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1155] && s.b[1172]) && (!s.b[1684])) {
            s.store_mul(1192, 564, 1182);
        }

        s.b[1685] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1685]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {
            s.store_sub(1194, 570, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1686] = (s.v[512] == 0.5);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && s.b[1686]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && (!s.b[1686])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1687] = (s.v[512] == 0.5);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && s.b[1687]) {
            s.store_sqrt_mul(1191, 1194, 597);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) && (!s.b[1687])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1685])) {
            s.store_mul(1198, 591, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(561), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 523, 1199, 1197);
        }

        s.b[1688] = (s.v[526] == 0.0);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1688]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {
            s.store_mul_ad_rhs(1201, 606, A::div_scaled_product(s.ad_value(1198), s.ad_value(576), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 603, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1689] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1689]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1689])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(603), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1690] = (s.v[1212] > 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1690]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1690])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1691] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1691]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1691])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1692] = (s.v[1212] > 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && s.b[1692]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1693] = (s.v[1211] > (-230.25850929940458));
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) && s.b[1693]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) && (!s.b[1693])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) && (!s.b[1692])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1688])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(603), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 526, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1694] = (s.v[532] == 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1694]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1695] = (s.v[512] == 0.5);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && s.b[1695]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1695])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) {
            s.store_mul_ad_rhs(1216, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(594), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1696] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && s.b[1696]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1697] = (((-s.v[609]) / s.v[1216]) < 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1696])) && s.b[1697]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) && (!s.b[1696])) && (!s.b[1697])) {
            let assign35030_ad_e45871: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign35030_ad_e45871, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1694])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(532), A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1698] = (s.v[541] > 1000.0);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1684])) && s.b[1698]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1699] = (s.v[1190] > ((-s.v[445]) * s.v[541]));
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        s.b[1700] = (s.v[544] == 4.0);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) && s.b[1700]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(615), A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), s.ad_value(1190), 615);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) && (!s.b[1700])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && s.b[1699]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1684])) && (!s.b[1698])) && (!s.b[1699])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), s.ad_value(618), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1684])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1701] = (s.v[675] == 0.0);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1701]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1701])) {
            s.store_mul(1192, 565, 1182);
        }

        s.b[1702] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1702]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {
            s.store_sub(1194, 571, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1703] = (s.v[513] == 0.5);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1704] = (s.v[513] == 0.5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1704]) {
            s.store_sqrt_mul(1191, 1194, 598);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1704])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1702])) {
            s.store_mul(1198, 592, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(562), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 524, 1199, 1197);
        }

        s.b[1705] = (s.v[527] == 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1705]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {
            s.store_mul_ad_rhs(1201, 607, A::div_scaled_product(s.ad_value(1198), s.ad_value(577), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 604, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1706] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1706]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1706])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(604), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1707] = (s.v[1212] > 0.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1707]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1707])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1708] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1708]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1708])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1709] = (s.v[1212] > 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && s.b[1709]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1710] = (s.v[1211] > (-230.25850929940458));
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) && s.b[1710]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) && (!s.b[1710])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) && (!s.b[1709])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1705])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(604), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 527, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1711] = (s.v[533] == 0.0);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1711]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1712] = (s.v[513] == 0.5);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && s.b[1712]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1712])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) {
            s.store_mul_ad_rhs(1216, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(595), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1713] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && s.b[1713]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1714] = (((-s.v[610]) / s.v[1216]) < 0.0);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1713])) && s.b[1714]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) && (!s.b[1713])) && (!s.b[1714])) {
            let assign35730_ad_e47014: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign35730_ad_e47014, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1711])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(533), A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1715] = (s.v[542] > 1000.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1701])) && s.b[1715]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1716] = (s.v[1190] > ((-s.v[445]) * s.v[542]));
        s.v[1716] = if s.b[1716] { 1.0 } else { 0.0 };

        s.b[1717] = (s.v[545] == 4.0);
        s.v[1717] = if s.b[1717] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) && s.b[1717]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(616), A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), s.ad_value(1190), 616);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) && (!s.b[1717])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(545));
        }

    }

    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && s.b[1716]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1701])) && (!s.b[1715])) && (!s.b[1716])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), s.ad_value(619), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1701])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1718] = (s.v[676] == 0.0);
        s.v[1718] = if s.b[1718] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1718]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1718])) {
            s.store_mul(1192, 566, 1182);
        }

        s.b[1719] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[1719] = if s.b[1719] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1719]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {
            s.store_sub(1194, 572, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1720] = (s.v[514] == 0.5);
        s.v[1720] = if s.b[1720] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && s.b[1720]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && (!s.b[1720])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1721] = (s.v[514] == 0.5);
        s.v[1721] = if s.b[1721] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && s.b[1721]) {
            s.store_sqrt_mul(1191, 1194, 599);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) && (!s.b[1721])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1719])) {
            s.store_mul(1198, 593, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(563), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 525, 1199, 1197);
        }

        s.b[1722] = (s.v[528] == 0.0);
        s.v[1722] = if s.b[1722] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1722]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {
            s.store_mul_ad_rhs(1201, 608, A::div_scaled_product(s.ad_value(1198), s.ad_value(578), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 605, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1723] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[1723] = if s.b[1723] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1723]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1723])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(605), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1724] = (s.v[1212] > 0.0);
        s.v[1724] = if s.b[1724] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1724]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1724])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1725] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1725] = if s.b[1725] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1725]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1725])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1726] = (s.v[1212] > 0.0);
        s.v[1726] = if s.b[1726] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && s.b[1726]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1727] = (s.v[1211] > (-230.25850929940458));
        s.v[1727] = if s.b[1727] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) && s.b[1727]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) && (!s.b[1727])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) && (!s.b[1726])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1722])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(605), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 528, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1728] = (s.v[534] == 0.0);
        s.v[1728] = if s.b[1728] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1728]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1729] = (s.v[514] == 0.5);
        s.v[1729] = if s.b[1729] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && s.b[1729]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1729])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) {
            s.store_mul_ad_rhs(1216, 581, A::div_scaled_product(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(596), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1730] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1730] = if s.b[1730] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && s.b[1730]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1731] = (((-s.v[611]) / s.v[1216]) < 0.0);
        s.v[1731] = if s.b[1731] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1730])) && s.b[1731]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) && (!s.b[1730])) && (!s.b[1731])) {
            let assign36430_ad_e48157: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign36430_ad_e48157, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1728])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(534), A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1732] = (s.v[543] > 1000.0);
        s.v[1732] = if s.b[1732] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1718])) && s.b[1732]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1733] = (s.v[1190] > ((-s.v[445]) * s.v[543]));
        s.v[1733] = if s.b[1733] { 1.0 } else { 0.0 };

        s.b[1734] = (s.v[546] == 4.0);
        s.v[1734] = if s.b[1734] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) && s.b[1734]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(617), A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), s.ad_value(1190), 617);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) && (!s.b[1734])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && s.b[1733]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1718])) && (!s.b[1732])) && (!s.b[1733])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(614), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), s.ad_value(620), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1718])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(479, A::add_scaled_products3(s.ad_value(674), s.ad_value(1218), 1.0, s.ad_value(675), s.ad_value(1219), 1.0, s.ad_value(676), s.ad_value(1220), 1.0));
            s.store_scalar(1189, 0.0);
            s.store_scalar(1186, 0.0);
        }

        s.b[1735] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1735] = if s.b[1735] { 1.0 } else { 0.0 };

        s.b[1736] = (s.v[490] < s.v[682]);
        s.v[1736] = if s.b[1736] { 1.0 } else { 0.0 };

        s.b[1737] = (((((-0.5) * (s.v[490] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[1737] = if s.b[1737] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && s.b[1737]) {
            s.store_exp_scaled_input(1184, 490, (s.v[372] * (-0.5)));
        }

        s.b[1738] = (((-0.5) * (s.v[490] * s.v[372])) < 0.0);
        s.v[1738] = if s.b[1738] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && (!s.b[1737])) && s.b[1738]) {
            s.store_div_from_scalar_offset_ad(1184, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) && (!s.b[1737])) && (!s.b[1738])) {
            s.store_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(490), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1736]) {
            s.store_div_from_scalar(1185, 1.0, 1184);
            s.store_square(1182, 1185);
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && (!s.b[1736])) {
            s.store_mul_offset_ad_lhs(1182, A::sub_scaled_inputs(s.ad_value(490), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(1185, 1182);
            s.store_div_from_scalar(1184, 1.0, 1185);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1735]) {
            s.store_offset(1182, 1182, (-1.0));
        }

        s.b[1739] = (s.v[490] > 0.0);
        s.v[1739] = if s.b[1739] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && s.b[1739]) {
            s.store_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[1155] && s.b[1172]) && s.b[1735]) && (!s.b[1739])) {
            s.store_sub_ad_lhs(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 490);
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1735]) {
            s.store_sub(1187, 684, 1186);
            s.store_ad_value(1188, A::add_scaled_inputs3(s.ad_value(490), 0.5, s.ad_value(1187), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(1187)), A::sub(s.ad_value(490), s.ad_value(1187))), ((4.0 * s.v[371]) * s.v[371]))), (-0.5)));
            s.store_ad_value(1189, A::add_scaled_inputs3(s.ad_value(490), 0.5, s.ad_value(687), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(490), s.ad_value(687)), A::sub(s.ad_value(490), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(1190, 490, A::sqrt(A::offset(A::mul(s.ad_value(490), s.ad_value(490)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[1740] = (s.v[674] == 0.0);
        s.v[1740] = if s.b[1740] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1740]) {
            s.store_scalar(1218, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1740])) {
            s.store_mul(1192, 564, 1182);
        }

        s.b[1741] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[1741] = if s.b[1741] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1741]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {
            s.store_sub(1194, 570, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1742] = (s.v[512] == 0.5);
        s.v[1742] = if s.b[1742] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && s.b[1742]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && (!s.b[1742])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1743] = (s.v[512] == 0.5);
        s.v[1743] = if s.b[1743] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && s.b[1743]) {
            s.store_sqrt_mul(1191, 1194, 597);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) && (!s.b[1743])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1741])) {
            s.store_mul(1198, 591, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(561), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 523, 1199, 1197);
        }

        s.b[1744] = (s.v[526] == 0.0);
        s.v[1744] = if s.b[1744] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1744]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {
            s.store_mul_ad_rhs(1201, 606, A::div_scaled_product(s.ad_value(1198), s.ad_value(576), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 603, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1745] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[1745] = if s.b[1745] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1745]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1745])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(603), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1746] = (s.v[1212] > 0.0);
        s.v[1746] = if s.b[1746] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1746]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1746])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1747] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1747] = if s.b[1747] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1747]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

    }

    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1747])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1748] = (s.v[1212] > 0.0);
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && s.b[1748]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1749] = (s.v[1211] > (-230.25850929940458));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) && s.b[1749]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) && (!s.b[1749])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) && (!s.b[1748])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1744])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(603), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 526, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1750] = (s.v[532] == 0.0);
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1750]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1751] = (s.v[512] == 0.5);
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && s.b[1751]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1751])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) {
            s.store_mul_ad_rhs(1216, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(1189)), s.ad_value(594), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1752] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && s.b[1752]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1753] = (((-s.v[609]) / s.v[1216]) < 0.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1752])) && s.b[1753]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) && (!s.b[1752])) && (!s.b[1753])) {
            let assign37430_ad_e49801: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign37430_ad_e49801, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1750])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(532), A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1754] = (s.v[541] > 1000.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1740])) && s.b[1754]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1755] = (s.v[1190] > ((-s.v[445]) * s.v[541]));
        s.v[1755] = if s.b[1755] { 1.0 } else { 0.0 };

        s.b[1756] = (s.v[544] == 4.0);
        s.v[1756] = if s.b[1756] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) && s.b[1756]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(615), A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), s.ad_value(1190), 615);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) && (!s.b[1756])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && s.b[1755]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1740])) && (!s.b[1754])) && (!s.b[1755])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), s.ad_value(618), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1740])) {
            s.store_mul_scale_ad_lhs(1218, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1757] = (s.v[675] == 0.0);
        s.v[1757] = if s.b[1757] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1757]) {
            s.store_scalar(1219, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1757])) {
            s.store_mul(1192, 565, 1182);
        }

        s.b[1758] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[1758] = if s.b[1758] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1758]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {
            s.store_sub(1194, 571, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1759] = (s.v[513] == 0.5);
        s.v[1759] = if s.b[1759] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && s.b[1759]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && (!s.b[1759])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1760] = (s.v[513] == 0.5);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && s.b[1760]) {
            s.store_sqrt_mul(1191, 1194, 598);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) && (!s.b[1760])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1758])) {
            s.store_mul(1198, 592, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(562), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 524, 1199, 1197);
        }

        s.b[1761] = (s.v[527] == 0.0);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1761]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {
            s.store_mul_ad_rhs(1201, 607, A::div_scaled_product(s.ad_value(1198), s.ad_value(577), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 604, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1762] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1762]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1762])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(604), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

        s.b[1763] = (s.v[1212] > 0.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1763]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1763])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1764] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1764]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1764])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1765] = (s.v[1212] > 0.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && s.b[1765]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1766] = (s.v[1211] > (-230.25850929940458));
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) && s.b[1766]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) && (!s.b[1766])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) && (!s.b[1765])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1761])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(604), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 527, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1767] = (s.v[533] == 0.0);
        s.v[1767] = if s.b[1767] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1767]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1768] = (s.v[513] == 0.5);
        s.v[1768] = if s.b[1768] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && s.b[1768]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1768])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) {
            s.store_mul_ad_rhs(1216, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(1189)), s.ad_value(595), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1769] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1769] = if s.b[1769] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && s.b[1769]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1770] = (((-s.v[610]) / s.v[1216]) < 0.0);
        s.v[1770] = if s.b[1770] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1769])) && s.b[1770]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) && (!s.b[1769])) && (!s.b[1770])) {
            let assign38130_ad_e50944: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign38130_ad_e50944, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1767])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(533), A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1771] = (s.v[542] > 1000.0);
        s.v[1771] = if s.b[1771] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1757])) && s.b[1771]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1772] = (s.v[1190] > ((-s.v[445]) * s.v[542]));
        s.v[1772] = if s.b[1772] { 1.0 } else { 0.0 };

        s.b[1773] = (s.v[545] == 4.0);
        s.v[1773] = if s.b[1773] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) && s.b[1773]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(616), A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), s.ad_value(1190), 616);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) && (!s.b[1773])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && s.b[1772]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1757])) && (!s.b[1771])) && (!s.b[1772])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), s.ad_value(619), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1757])) {
            s.store_mul_scale_ad_lhs(1219, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        s.b[1774] = (s.v[676] == 0.0);
        s.v[1774] = if s.b[1774] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1774]) {
            s.store_scalar(1220, 0.0);
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1774])) {
            s.store_mul(1192, 566, 1182);
        }

        s.b[1775] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1775]) {
            s.store_scalar(1193, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {
            s.store_sub(1194, 572, 1188);
            s.store_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));
        }

        s.b[1776] = (s.v[514] == 0.5);
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && s.b[1776]) {
            s.store_scalar(1196, 0.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && (!s.b[1776])) {
            s.store_ad_value(1196, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), 1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {
            s.store_add(1197, 1195, 1196);
        }

        s.b[1777] = (s.v[514] == 0.5);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && s.b[1777]) {
            s.store_sqrt_mul(1191, 1194, 599);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) && (!s.b[1777])) {
            s.store_pow_ad(1191, A::mul(s.ad_value(1194), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1775])) {
            s.store_mul(1198, 593, 1191);
            s.store_mul_ad_product_lhs(1199, s.ad_value(563), A::offset(s.ad_value(1185), (-1.0)), 1198);
            s.store_mul3_lhs(1193, 525, 1199, 1197);
        }

        s.b[1778] = (s.v[528] == 0.0);
        s.v[1778] = if s.b[1778] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1778]) {
            s.store_scalar(1200, 0.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {
            s.store_mul_ad_rhs(1201, 608, A::div_scaled_product(s.ad_value(1198), s.ad_value(578), 1.0, s.ad_value(1194), 1.0));
            s.store_scaled_div(1202, 605, 1201, 0.666666666666667);
            s.store_square(1203, 1202);
            s.store_sqrt_ad(1204, A::div_scaled_product_offset_denominator(s.ad_value(1203), s.ad_value(1203), 1.0, A::square(s.ad_value(1203)), 1.0, 1.0));
            s.store_sqrt(1205, 1204);
            s.store_mul(1206, 1204, 1205);
        }

        s.b[1779] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[1779] = if s.b[1779] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1779]) {
            s.store_div_from_scalar_offset_ad(1207, 1.0, A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1779])) {
            s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {
            s.store_ad_value(1208, A::div_scaled_product(s.ad_value(1197), s.ad_value(1207), 1.0, A::add(s.ad_value(1197), s.ad_value(1207)), 1.0));
            s.store_sqrt_scaled_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);
            s.store_ad_value(1210, A::add_scaled_product(s.ad_value(1204), (-1.0), s.ad_value(1202), s.ad_value(1205), 2.0));
            s.store_ad_value(1211, A::add_scaled_value_products(A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, s.ad_value(605), s.ad_value(1204), (-1.0), s.ad_value(1201), s.ad_value(1206), 0.5));
            s.store_mul_offset_lhs(1212, 1210, (-1.0), 1209);
            s.store_square(1173, 1212);
        }

    }

    pub(super) fn stamp_transient_block_27(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1780] = (s.v[1212] > 0.0);
        s.v[1780] = if s.b[1780] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1780]) {
            s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1780])) {
            s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));
        }

        s.b[1781] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));
        s.v[1781] = if s.b[1781] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1781]) {
            s.store_exp_sub(1191, 1211, 1173);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1781])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {
            s.store_mul_ad_lhs(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);
        }

        s.b[1782] = (s.v[1212] > 0.0);
        s.v[1782] = if s.b[1782] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && s.b[1782]) {
            s.copy_ad(1213, 1175);
        }

        s.b[1783] = (s.v[1211] > (-230.25850929940458));
        s.v[1783] = if s.b[1783] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) && s.b[1783]) {
            s.store_exp(1191, 1211);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) && (!s.b[1783])) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1211), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1211), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) && (!s.b[1782])) {
            s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1778])) {
            s.store_ad_value(1214, A::div_scaled_product(s.ad_value(605), s.ad_value(1213), (1.772453850905516 * 0.5), s.ad_value(1209), 1.0));
            s.store_mul_ad_rhs(1200, 528, A::mul3(s.ad_value(1199), s.ad_value(1214), s.ad_value(1208)));
        }

        s.b[1784] = (s.v[534] == 0.0);
        s.v[1784] = if s.b[1784] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1784]) {
            s.store_scalar(1215, 0.0);
        }

        s.b[1785] = (s.v[514] == 0.5);
        s.v[1785] = if s.b[1785] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && s.b[1785]) {
            s.store_sqrt_mul_ad(1191, A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1785])) {
            s.store_pow_ad(1191, A::mul(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) {
            s.store_mul_ad_rhs(1216, 581, A::div_scaled_product(A::sub(s.ad_value(511), s.ad_value(1189)), s.ad_value(596), 1.0, s.ad_value(1191), 1.0));
        }

        s.b[1786] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);
        s.v[1786] = if s.b[1786] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && s.b[1786]) {
            s.store_exp_ad(1191, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));
        }

        s.b[1787] = (((-s.v[611]) / s.v[1216]) < 0.0);
        s.v[1787] = if s.b[1787] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1786])) && s.b[1787]) {
            s.store_div_from_scalar_offset_ad(1191, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) && (!s.b[1786])) && (!s.b[1787])) {
            let assign38830_ad_e52087: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(1191, assign38830_ad_e52087, 1e100);
        }

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1784])) {
            s.store_mul_ad_product_lhs(1215, s.ad_value(534), A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191);
        }

        s.b[1788] = (s.v[543] > 1000.0);
        s.v[1788] = if s.b[1788] { 1.0 } else { 0.0 };

        if (((s.b[1155] && s.b[1172]) && (!s.b[1774])) && s.b[1788]) {
            s.store_scalar(1217, 1.0);
        }

        s.b[1789] = (s.v[1190] > ((-s.v[445]) * s.v[543]));
        s.v[1789] = if s.b[1789] { 1.0 } else { 0.0 };

        s.b[1790] = (s.v[546] == 4.0);
        s.v[1790] = if s.b[1790] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) && s.b[1790]) {
            s.store_mul_ad_product_lhs(1191, A::mul3(A::mul3(s.ad_value(1190), s.ad_value(617), A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), s.ad_value(1190), 617);
        }

        if (((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) && (!s.b[1790])) {
            s.store_pow_ad(1191, A::abs(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && s.b[1789]) {
            s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));
        }

        if ((((s.b[1155] && s.b[1172]) && (!s.b[1774])) && (!s.b[1788])) && (!s.b[1789])) {
            s.store_ad_value(1217, A::add_scaled_product(s.ad_value(614), 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), s.ad_value(620), 1.0));
        }

        if ((s.b[1155] && s.b[1172]) && (!s.b[1774])) {
            s.store_mul_scale_ad_lhs(1220, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 1217);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_ad_value(480, A::add_scaled_products3(s.ad_value(674), s.ad_value(1218), 1.0, s.ad_value(675), s.ad_value(1219), 1.0, s.ad_value(676), s.ad_value(1220), 1.0));
            s.store_ad_value(695, A::add_scaled_products3(s.ad_value(674), s.ad_value(564), 1.0, s.ad_value(675), s.ad_value(565), 1.0, s.ad_value(676), s.ad_value(566), 1.0));
            s.store_ad_value(484, A::add_scaled_offset_product_rhs(s.ad_value(479), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(489), (s.v[372] * s.v[696])), (-1.0), (-1.0)));
            s.store_ad_value(485, A::add_scaled_offset_product_rhs(s.ad_value(480), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(490), (s.v[372] * s.v[696])), (-1.0), (-1.0)));
        }

        s.b[1791] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1791] = if s.b[1791] { 1.0 } else { 0.0 };

        s.b[1792] = ((s.v[479] > 0.0) && (s.v[480] > 0.0));
        s.v[1792] = if s.b[1792] { 1.0 } else { 0.0 };

        s.b[1793] = ((((((s.v[484] / s.v[479]) > 0.001) || ((s.v[485] / s.v[480]) > 0.001)) && (s.v[484] > 0.0)) && (s.v[485] > 0.0)) && (s.v[485] > s.v[484]));
        s.v[1793] = if s.b[1793] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1792]) && s.b[1793]) {
            s.store_div(491, 484, 485);
            s.store_ad_value(698, A::div_scaled_inputs(A::ln(s.ad_value(491)), s.v[371], A::sub(s.ad_value(489), s.ad_value(490)), 1.0));
            s.store_div_ad_rhs(697, 484, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(489), s.v[372], s.ad_value(698))), (-1.0)));
        }

        if ((s.b[1155] && s.b[1172]) && s.b[1791]) {
            s.store_ad_value(481, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(486), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, s.ad_value(697), A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[372], s.ad_value(698))), (-1.0), (-1.0)));
            s.store_ad_value(482, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(487), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, s.ad_value(697), A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[372], s.ad_value(698))), (-1.0), (-1.0)));
            s.store_ad_value(483, A::add_scaled_offset_product_rhs(A::add_scaled_offset_product_rhs(s.ad_value(478), 1.0, s.ad_value(695), A::exp_scaled_input(s.ad_value(488), (s.v[372] * s.v[696])), (-1.0), (-1.0)), 1.0, s.ad_value(697), A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[372], s.ad_value(698))), (-1.0), (-1.0)));
        }

        s.b[1794] = (((s.v[476] < 0.0) && (s.v[477] < 0.0)) && (s.v[478] < 0.0));
        s.v[1794] = if s.b[1794] { 1.0 } else { 0.0 };

        s.b[1795] = (((((((s.v[481] / s.v[476]) > 0.001) || ((s.v[482] / s.v[477]) > 0.001)) || ((s.v[483] / s.v[478]) > 0.001)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0)) && (s.v[483] < 0.0));
        s.v[1795] = if s.b[1795] { 1.0 } else { 0.0 };

        if ((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) {
            s.store_div(491, 481, 482);
            s.store_ad_value(492, A::div_scaled_inputs(A::ln(s.ad_value(491)), (-s.v[371]), A::sub(s.ad_value(486), s.ad_value(487)), 1.0));
            s.store_div_ad_rhs(494, 487, A::sub(s.ad_value(487), s.ad_value(486)));
            s.store_scaled_mul_ad(495, A::offset(s.ad_value(491), (-1.0)), A::offset(A::pow(s.ad_value(491), s.ad_value(494)), (-1.0)), s.v[371]);
            s.store_div_ad_rhs(494, 486, A::sub(s.ad_value(486), s.ad_value(487)));
            s.store_sub_ad_lhs(496, A::add_scaled_products(A::pow(s.ad_value(491), s.ad_value(494)), A::sub(s.ad_value(487), s.ad_value(486)), 1.0, s.ad_value(491), s.ad_value(486), 1.0), 487);
            s.store_div(493, 495, 496);
            s.store_add(700, 492, 493);
        }

        s.b[1796] = (((((s.v[488] * s.v[372]) * s.v[700])) as f64).abs() < 1e-6);
        s.v[1796] = if s.b[1796] { 1.0 } else { 0.0 };

        if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && s.b[1796]) {
            s.store_scalar(694, 1.0);
            s.store_mul_ad_rhs(699, 483, A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(488)), 1.0, s.ad_value(700), (0.5 * s.v[372])));
            s.store_ad_value(700, A::div_scaled_product(s.ad_value(483), s.ad_value(700), ((-0.5) * s.v[372]), s.ad_value(488), 1.0));
        }

        if (((((s.b[1155] && s.b[1172]) && s.b[1791]) && s.b[1794]) && s.b[1795]) && (!s.b[1796])) {
            s.store_scalar(694, 0.0);
            s.store_ad_value(699, A::div_scaled_inputs(s.ad_value(483), -1.0, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(488), (-s.v[372]), s.ad_value(700))), (-1.0)), 1.0));
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_mul_ad_rhs(502, 554, A::add_scaled_products3(s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0, s.ad_value(676), s.ad_value(584), 1.0));
        }

        s.b[1797] = ((s.v[674] * s.v[582]) <= s.v[502]);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1797]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1798] = ((s.v[675] * s.v[583]) <= s.v[502]);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1798]) {
            s.store_scalar(680, 0.0);
        }

        s.b[1799] = ((s.v[676] * s.v[584]) <= s.v[502]);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1799]) {
            s.store_scalar(681, 0.0);
        }

        s.b[1800] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1800]) {
            s.store_ln_ad(688, A::div_from_scalar((0.5 * p.p839), A::offset(s.ad_value(695), 1e-21)));
            s.store_ln_ad(690, A::div_from_scalar((0.5 * p.p839), A::offset(s.ad_value(697), 1e-21)));
            s.store_ln_ad(692, A::div_from_scalar((0.5 * p.p839), A::offset(A::abs(s.ad_value(699)), 1e-21)));
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_min_with_scalar(688, 688, 230.25850929940458);
            s.store_exp(689, 688);
            s.store_min_with_scalar(690, 690, 230.25850929940458);
            s.store_exp(691, 690);
            s.store_min_with_scalar(692, 692, 230.25850929940458);
            s.store_exp(693, 692);
        }

        s.v[1929] = 0.0;

        s.v[1930] = 0.0;

        s.v[1931] = 0.0;

        s.store_offset_voltage(357, ctx, nodes, Some(4), None, s.v[352]);

        s.store_square(358, 357);

        s.store_offset(359, 357, (-s.v[351]));

        s.store_div_from_scalar(360, s.v[351], 357);

        s.store_ln(361, 360);

        s.store_scale(1916, 357, (1.3806505e-23 * 6.241449993689894e18));

        s.store_div_from_scalar(362, 1.0, 1916);

        s.store_sub_scaled_ad_lhs(363, A::sub_from_scalar(1.179, A::scale(s.ad_value(357), 9.025e-5)), 358, 3.05e-7);

        s.store_mul_ad_affine_product_lhs(364, A::scale_offset(s.ad_value(357), 0.00045, 1.045), A::sub_scaled_inputs(A::scale_offset(s.ad_value(357), 0.0014, 0.523), 1.0, s.ad_value(358), 1.48e-6), 1.1111111111111112e-5, 0.0, 358);

        if (!(s.v[364] > 0.001)) {
            s.store_scalar(364, 0.001);
        }

        s.store_scale(1919, 357, (4.0 * 1.3806505e-23));

        s.store_ad_value(717, A::add_scaled_inputs_product(s.ad_value(363), 1.0, s.ad_value(185), 1.0, s.ad_value(1916), A::ln_scaled_input(A::mul(s.ad_value(181), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0));

        if (!(s.v[717] > 0.05)) {
            s.store_scalar(717, 0.05);
        }

        s.store_div_ad_lhs(718, A::sqrt(A::mul_scaled_lhs(s.ad_value(181), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.v[719] = 0.0;

        s.v[720] = 0.0;

        s.b[2004] = (s.v[186] > 0.0);
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if s.b[2004] {
            s.store_div_from_scalar(721, 80000000.0, 759);
        }

        if s.b[2004] {
            s.store_ad_value(720, {
                if (s.v[186] > s.v[721]) {
                    s.ad_value(186)
                } else {
                    s.ad_value(721)
                }
            });
        }

        if s.b[2004] {
            s.store_ad_value(720, {
                if (5e24 > s.v[720]) {
                    A::constant(5e24)
                } else {
                    s.ad_value(720)
                }
            });
        }

        if s.b[2004] {
            s.store_ad_value(719, A::div_scaled_product3(s.ad_value(758), s.ad_value(758), s.ad_value(1916), 2.0, s.ad_value(720), (1.6021918e-19 * s.v[756])));
        }

        s.store_scaled_mul(722, 1916, 1916, 100.0);

        s.b[2005] = (p.p51 > 0.0);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if s.b[2005] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(718), s.ad_value(718)), s.ad_value(717));
            s.store_mul_scaled_ad_rhs(724, 762, 0.75, A::powf(s.ad_value(723), 0.6666666666666666));
            s.store_add(717, 717, 724);
            s.store_mul_offset_ad_rhs(718, 718, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_sqrt(725, 717);

        s.store_scale(726, 717, 0.95);

        s.store_scaled_mul(727, 717, 717, 0.0025);

        s.copy_ad(728, 727);

        s.store_scaled_sqrt(729, 728, 0.5);

        s.store_ad_value(730, A::add_scaled_inputs3(s.ad_value(726), 0.5, s.ad_value(729), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(727), 1.0, A::sub(s.ad_value(726), s.ad_value(729)), A::sub(s.ad_value(726), s.ad_value(729)), 1.0)), (-0.5)));

        s.store_scaled_add(731, 717, 363, 0.5);

        s.store_sub_ad_lhs(732, A::sqrt(A::add(s.ad_value(183), s.ad_value(717))), 725);

        s.store_ad_value(733, A::add_scaled_inputs3(A::sqrt(A::add_scaled_inputs3(s.ad_value(183), 1.0, s.ad_value(184), 1.0, s.ad_value(717), 1.0)), 1.0, s.ad_value(725), (-1.0), s.ad_value(732), -1.0));

        s.store_ad_value(734, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(363), 1.0, s.ad_value(185), 1.0, s.ad_value(254), 1.0), 1.0, s.ad_value(1916), A::ln_scaled_input(A::mul(s.ad_value(761), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0));

        if (!(s.v[734] > 0.05)) {
            s.store_scalar(734, 0.05);
        }

        s.store_div_ad_lhs(735, A::sqrt(A::mul_scaled_lhs(s.ad_value(761), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.b[2006] = (p.p51 > 0.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if s.b[2006] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(735), s.ad_value(735)), s.ad_value(734));
            s.store_mul_scaled_ad_rhs(724, 762, 0.75, A::powf(s.ad_value(723), 0.6666666666666666));
            s.store_add(734, 734, 724);
            s.store_mul_offset_ad_rhs(735, 735, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_scale(736, 734, 0.95);

        s.store_scaled_mul(737, 734, 734, 0.0025);

        s.copy_ad(738, 737);

        s.store_scaled_sqrt(729, 738, 0.5);

        s.store_ad_value(739, A::add_scaled_inputs3(s.ad_value(736), 0.5, s.ad_value(729), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(737), 1.0, A::sub(s.ad_value(736), s.ad_value(729)), A::sub(s.ad_value(736), s.ad_value(729)), 1.0)), (-0.5)));

        s.store_offset_add_ad(701, s.ad_value(175), A::mul3(s.ad_value(176), s.ad_value(359), A::offset(A::mul(s.ad_value(177), s.ad_value(359)), 1.0)), s.v[17]);

        s.store_exp_mul(740, 178, 361);

        s.store_mul(702, 187, 740);

        s.store_div(703, 188, 360);

        s.store_exp_mul(741, 201, 361);

        s.store_mul(704, 200, 741);

        s.store_scaled_mul(1917, 704, 758, s.v[16]);

        s.store_mul_exp_ad_rhs(706, 204, A::mul(s.ad_value(205), s.ad_value(361)));

        s.store_exp_mul(742, 203, 361);

        s.store_mul(705, 202, 742);

        s.store_mul_exp_ad_rhs(708, 208, A::mul(s.ad_value(209), s.ad_value(361)));

        s.store_exp_mul(743, 207, 361);

        s.store_mul(707, 206, 743);

        s.store_exp_mul(744, 211, 361);

        s.store_mul(709, 210, 744);

        s.store_exp_mul(745, 214, 361);

        s.store_mul(710, 213, 745);

        s.store_scaled_mul(746, 1917, 710, 2.0);

        s.store_exp_mul(747, 218, 361);

        s.store_mul(1921, 217, 747);

        s.store_mul(1922, 256, 747);

        s.store_mul_exp_ad_rhs(713, 228, A::mul_scaled_lhs(s.ad_value(229), -1.0, s.ad_value(361)));

        s.store_scaled_mul(1920, 274, 357, (4.0 * 1.3806505e-23));

        s.store_ad_value(1923, A::div_scaled_product(A::square(s.ad_value(1916)), s.ad_value(1917), 1.0, s.ad_value(760), 1.0));

        s.b[2007] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if s.b[2007] {
            s.store_offset_ad(714, A::add_scaled_product(s.ad_value(280), 1.0, s.ad_value(281), s.ad_value(359), 1.0), s.v[19]);
        }

    }

    pub(super) fn stamp_transient_block_28(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[2007] {
            s.store_exp_mul(748, 286, 361);
            s.store_mul(715, 285, 748);
            s.store_scaled_mul(1918, 715, 758, s.v[18]);
            s.store_mul_offset_ad_rhs(1924, 1916, A::mul(s.ad_value(284), s.ad_value(360)), 1.0);
            s.store_ad_value(749, A::add_scaled_inputs_product(s.ad_value(363), 1.0, s.ad_value(282), 1.0, s.ad_value(1924), A::ln_scaled_input(A::mul(s.ad_value(283), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0));
        }

        if s.b[2007] {
            s.store_ad_value(749, {
                if (s.v[749] > 0.05) {
                    s.ad_value(749)
                } else {
                    A::constant(0.05)
                }
            });
        }

        if s.b[2007] {
            s.store_div_ad_lhs(750, A::sqrt(A::mul_scaled_lhs(s.ad_value(283), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);
            s.store_square(1925, 750);
            s.store_ln(1926, 1925);
            s.store_scale(751, 749, 0.95);
            s.store_scaled_mul(752, 749, 749, 0.0025);
            s.copy_ad(753, 752);
            s.store_scaled_sqrt(754, 753, 0.5);
            s.store_ad_value(755, A::add_scaled_inputs3(s.ad_value(751), 0.5, s.ad_value(754), ((-1.0) * 0.5), A::sqrt(A::add_scaled_product(s.ad_value(752), 1.0, A::sub(s.ad_value(751), s.ad_value(754)), A::sub(s.ad_value(751), s.ad_value(754)), 1.0)), (-0.5)));
            s.store_ad_value(1927, A::div_scaled_product(A::square(s.ad_value(1916)), s.ad_value(1918), 1.0, s.ad_value(760), 1.0));
            s.store_scaled_mul(1928, 293, 357, (4.0 * 1.3806505e-23));
        }

        if (!s.b[2007]) {
            s.store_scalar(714, 0.0);
            s.store_scalar(748, 1.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(1918, 0.0);
            s.copy_ad(1924, 1916);
            s.store_scalar(749, 0.0);
            s.store_scalar(750, 1.0);
            s.store_scalar(1925, 1.0);
            s.store_scalar(1926, 0.0);
            s.store_scalar(751, 0.0);
            s.store_scalar(752, 0.0);
            s.store_scalar(753, 0.0);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 0.0);
            s.store_scalar(1927, 0.0);
            s.store_scalar(1928, 1.0);
        }

        s.b[2008] = (s.v[0] == 1.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        if s.b[2008] {
            s.store_voltage(814, ctx, nodes, Some(6), Some(7));
            s.store_voltage(815, ctx, nodes, Some(8), Some(7));
            s.store_voltage(816, ctx, nodes, Some(7), Some(9));
            s.store_scaled_voltage(821, ctx, nodes, Some(7), Some(11), -1.0);
            s.store_scaled_voltage(822, ctx, nodes, Some(8), Some(12), -1.0);
        }

        if (!s.b[2008]) {
            s.store_scaled_voltage(814, ctx, nodes, Some(6), Some(7), -1.0);
            s.store_scaled_voltage(815, ctx, nodes, Some(8), Some(7), -1.0);
            s.store_scaled_voltage(816, ctx, nodes, Some(7), Some(9), -1.0);
            s.store_voltage(821, ctx, nodes, Some(7), Some(11));
            s.store_voltage(822, ctx, nodes, Some(8), Some(12));
        }

        s.store_add(818, 814, 816);

        s.copy_ad(823, 814);

        s.copy_ad(824, 816);

        s.store_add(825, 815, 816);

        s.store_sub(826, 814, 815);

        s.store_scale(1801, 823, (-s.v[356]));

        s.store_scale(1802, 826, (-s.v[356]));

        s.store_scaled_sub(1803, 818, 701, (-s.v[356]));

        s.v[820] = 1.0;

        s.b[2009] = (s.v[815] < 0.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if s.b[2009] {
            s.store_scalar(820, (-1.0));
            s.store_sub(814, 814, 815);
            s.store_add(816, 816, 815);
            s.store_neg(815, 815);
        }

        s.store_add(817, 815, 816);

        s.store_ad_value(819, A::div_scaled_product_offset_denominator(s.ad_value(815), s.ad_value(815), 1.0, A::sqrt(A::offset(A::square(s.ad_value(815)), 0.01)), 0.1, 1.0));

        s.store_ad_value(2013, A::add_scaled_inputs4(s.ad_value(817), 0.5, s.ad_value(816), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(728), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), s.ad_value(726), 1.0));

        s.copy_ad(1804, 2013);

        s.store_ad_value(1932, A::add_scaled_inputs4(s.ad_value(816), 1.0, s.ad_value(2013), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(2013), s.ad_value(2013), 1.0)), (-(-0.5)), s.ad_value(730), 1.0));

        s.copy_ad(1805, 1932);

        s.v[1933] = 0.0;

        s.b[2169] = ((p.p45 != 0.0) && (s.v[182] != 1.0));
        s.v[2169] = if s.b[2169] { 1.0 } else { 0.0 };

        if s.b[2169] {
            s.store_ad_value(1934, A::add_scaled_inputs3(s.ad_value(1932), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5)));
            s.store_sub_ad_lhs(1935, A::sqrt(A::add(s.ad_value(1934), s.ad_value(717))), 725);
            s.store_offset_div_ad(1929, A::sub_scaled_inputs(s.ad_value(1935), 2.0, s.ad_value(732), 2.0), s.ad_value(733), (-1.0));
            s.store_ad_value(1936, A::add_scaled_product(s.ad_value(1935), 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(182), s.ad_value(733), 0.25), A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 0.4804530139182))), (-1.0)));
            s.store_ad_value(1937, A::add_scaled_square_product(s.ad_value(1936), 1.0, s.ad_value(725), s.ad_value(1936), 2.0));
            s.store_ad_value(1932, A::add_scaled_inputs3(s.ad_value(1937), 1.0, s.ad_value(815), (-0.5), s.ad_value(819), (-(-0.5))));
            s.store_sub(1933, 1805, 1932);
        }

        s.copy_ad(2010, 717);

        s.copy_ad(2011, 727);

        s.copy_ad(2012, 718);

        s.copy_ad(2014, 1932);

        s.copy_ad(2018, 1933);

        s.copy_ad(2015, 1921);

        s.copy_ad(2016, 766);

        s.store_ad_value(2017, A::add_scaled_inputs3(s.ad_value(818), 1.0, s.ad_value(2018), (-1.0), s.ad_value(701), -1.0));

        s.store_ad_value(2019, A::add_scaled_inputs3(s.ad_value(2014), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5)));

        s.v[2031] = 1.0;

        s.b[2170] = (s.v[188] > 0.0);
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if s.b[2170] {
            s.store_mul(2022, 2010, 362);
            s.store_mul(2023, 2019, 362);
            s.store_mul(2024, 2017, 362);
            s.store_offset_ad(1930, A::div_scaled_inputs(s.ad_value(2012), 0.5, A::sqrt(s.ad_value(2022)), 1.0), 1.0);
            s.store_ad_value(1931, A::add_scaled_product(s.ad_value(2022), 1.0, s.ad_value(2012), A::sqrt(s.ad_value(2022)), 1.0));
            s.store_ad_value(2025, A::add_scaled_inputs_product(A::div(A::sub(s.ad_value(2024), s.ad_value(1931)), s.ad_value(1930)), 1.0, s.ad_value(2022), 0.5, A::offset(s.ad_value(189), 1.0), s.ad_value(2023), (-1.0)));
            s.store_offset_scaled(2026, 2022, 0.5, 2.0);
            s.store_add(2027, 2022, 2023);
            s.store_ad_value(1930, A::sub_scaled_inputs(A::add_scaled_inputs_product(s.ad_value(2024), 1.0, s.ad_value(2027), (-1.0), s.ad_value(2012), A::sqrt(s.ad_value(2027)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2022), s.ad_value(2012)), A::sqrt(s.ad_value(2022)))), 2.0));
            s.store_add_scaled_inputs(2028, 1930, 2.0, 2026, 1.0);
            s.store_ad_value(1930, A::add_scaled_inputs3(s.ad_value(2025), 0.5, s.ad_value(2028), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2025), s.ad_value(2028)), A::sub(s.ad_value(2025), s.ad_value(2028))), 20.0)), 0.5));
            s.store_ad_value(1931, A::add_scaled_inputs3(s.ad_value(2024), 2.0, s.ad_value(2023), (-2.0), s.ad_value(2026), -1.0));
            s.store_ad_value(2029, A::add_scaled_inputs3(s.ad_value(1930), 0.5, s.ad_value(1931), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0)), (-0.5)));
            s.store_ad_value(1930, A::add_scaled_inputs3(s.ad_value(2029), 0.5, s.ad_value(2026), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2029), s.ad_value(2026)), A::sub(s.ad_value(2029), s.ad_value(2026))), 5.0)), (-0.5)));
            s.store_ad_value(2030, A::add_scaled_inputs3(s.ad_value(1930), 0.5, s.ad_value(2026), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0), A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0)), 20.0)), 0.5));
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2030), s.ad_value(2026)), 1.0);
        }

        s.b[2171] = (s.v[1931] > (-230.25850929940458));
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        if (s.b[2170] && s.b[2171]) {
            s.store_exp(2031, 1931);
        }

        if (s.b[2170] && (!s.b[2171])) {
            s.store_div_from_scalar_offset_ad(2031, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1931), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1931), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.store_offset_mul(2032, 702, 2031, 1.0);

        s.store_mul(2033, 1916, 2032);

        s.store_mul_ad_product_rhs(2034, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2019)), 1.0));

        s.store_mul_offset_rhs(2035, 2033, 2034, 1.0);

        s.store_div_from_scalar(2036, 1.0, 2035);

        s.store_mul_sqrt_ad_rhs(2020, 2012, A::mul(s.ad_value(1916), s.ad_value(2036)));

        s.store_square(2021, 2020);

        s.store_div_from_scalar(2037, 1.0, 2021);

        s.store_mul(2038, 2014, 2036);

        s.store_mul(2039, 2017, 2036);

        s.store_ad_value(2040, A::div_scaled_inputs(s.ad_value(819), 2.0, A::offset(A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0), 1.0));

        s.store_mul_ad_product_rhs(2041, 194, s.ad_value(2040), A::offset(A::mul(s.ad_value(196), s.ad_value(2019)), 1.0));

        s.store_mul(2042, 2010, 2036);

        s.store_sqrt_square_add(1930, 2013, 2011);

        s.store_sqrt_ad(1931, A::add_scaled_product(s.ad_value(2011), 1.0, A::sub(s.ad_value(2013), s.ad_value(2041)), A::sub(s.ad_value(2013), s.ad_value(2041)), 1.0));

        s.store_mul_scaled_ad_rhs(2043, 2036, 0.5, A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(1930), 1.0, s.ad_value(1931), -1.0));

        s.store_add(2044, 2042, 2038);

        s.store_sub(2045, 2044, 2043);

        s.b[2172] = (p.p45 > 0.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        s.b[2173] = (((s.v[2045]) as f64).abs() < 1e-5);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (s.b[2172] && s.b[2173]) {
            s.store_offset_ad(2046, A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.3125), 0.5)), 1.0);
        }

        s.b[2174] = (s.v[2045] < 460.51701859880916);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if ((s.b[2172] && (!s.b[2173])) && s.b[2174]) {
            s.store_exp_neg_input(2060, 2045);
        }

        if ((s.b[2172] && (!s.b[2173])) && (!s.b[2174])) {
            s.store_div_from_scalar_offset_ad(2060, 1e-200, A::mul_offset_lhs(s.ad_value(2045), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2045), (-460.51701859880916), A::scale_offset(s.ad_value(2045), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_scalar(1929, (if (s.v[2045] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_offset_ad(2046, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2020), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2045))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2045), 1.0, s.ad_value(2060))), 2.0), 1.0);
        }

        if (!s.b[2172]) {
            s.store_offset_ad(2046, A::div_scaled_inputs(s.ad_value(2020), 0.5, A::sqrt(s.ad_value(2045)), 1.0), 1.0);
        }

        s.store_ad_value(2047, A::add_scaled_value_products(s.ad_value(2045), 1.0, s.ad_value(2020), A::sqrt(s.ad_value(2045)), 1.0, s.ad_value(2046), A::ln(A::offset(s.ad_value(2046), (-1.0))), (-1.0)));

        s.store_div_ad_lhs(2048, A::sub(s.ad_value(2039), s.ad_value(2047)), 2046);

        s.store_mul_scaled_ad_rhs(2054, 2021, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2021)), 1.0)), (-1.0)));

        s.v[2053] = 0.0;

        s.v[2055] = 1.0;

        s.b[2175] = (s.v[2048] > (-30.0));
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if s.b[2175] {
            s.store_offset_mul(2049, 2046, 2048, (-1.0));
            s.store_scaled_add_ad_rhs(1929, 2049, A::sqrt(A::offset(A::square(s.ad_value(2049)), 10.0)), 0.5);
            s.store_sub_ad_rhs(2050, 2048, A::ln(s.ad_value(1929)));
            s.store_scaled_add_ad_rhs(2051, 2050, A::sqrt(A::offset(A::square(s.ad_value(2050)), 2.0)), 0.5);
        }

        s.b[2176] = ((s.v[2048] - s.v[2051]) < 230.25850929940458);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if (s.b[2175] && s.b[2176]) {
            s.store_exp_sub(1929, 2048, 2051);
        }

        if (s.b[2175] && (!s.b[2176])) {
            s.store_scaled_offset_ad(1929, A::mul_offset_lhs(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2048), s.ad_value(2051)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if s.b[2175] {
            s.store_div(2052, 1929, 2046);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2051), 1.0, 2.0), 2052);
        }

        s.b[2177] = (s.v[2052] > 1e-6);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        if (s.b[2175] && s.b[2177]) {
            s.store_mul_offset_ad_rhs(2053, 2046, A::sub(s.ad_value(2051), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2052), s.ad_value(1929)), 1.0)), (-1.0)), s.ad_value(2052))), 1.0);
        }

        if (s.b[2175] && (!s.b[2177])) {
            s.store_mul_ad_affine_product_rhs(2053, 2046, s.ad_value(2052), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if s.b[2175] {
            s.store_ad_value(1929, A::add_scaled_inputs3_offset(s.ad_value(2039), 0.5, s.ad_value(2053), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0), A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5)));
            s.store_mul_scaled_ad_rhs(2054, 2021, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2021)), s.ad_value(1929)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2055, 2054, A::add(s.ad_value(2054), s.ad_value(2053)));
            s.store_ad_value(2045, A::add_scaled_product(s.ad_value(2044), 1.0, s.ad_value(2055), s.ad_value(2043), (-1.0)));
        }

        s.store_offset_scaled(2056, 2020, 0.7071067811865475, 1.0);

        s.store_scale(2057, 2056, 1e-5);

        s.store_div_from_scalar(2058, 1.0, 2056);

        s.v[2165] = 0.0;

        s.v[2059] = 0.0;

    }

    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        s.b[2178] = (s.v[2045] < 460.51701859880916);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if s.b[2178] {
            s.store_exp_neg_input(2060, 2045);
        }

        if (!s.b[2178]) {
            s.store_div_from_scalar_offset_ad(2060, 1e-200, A::mul_offset_lhs(s.ad_value(2045), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2045), (-460.51701859880916), A::scale_offset(s.ad_value(2045), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2179] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if s.b[2179] {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2059, 2039, s.ad_value(2058), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2060)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        s.b[2180] = (s.v[2039] < (-s.v[2057]));
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_neg(2147, 2039);
            s.store_scaled_mul(2148, 2147, 2058, 1.25);
            s.store_scaled_sub_ad(2149, A::offset(s.ad_value(2148), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2148), (-6.0), A::offset(s.ad_value(2148), (-6.0))), 64.0)), 0.5);
            s.store_sub(2144, 2147, 2149);
            s.store_ad_value(2150, A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::offset(s.ad_value(2149), 1.0), 1.0));
            s.store_sub_scaled_inputs(2151, 2144, 2.0, 2021, 1.0);
            s.store_sub_ad_lhs(2152, A::ln(A::mul(s.ad_value(2150), s.ad_value(2037))), 2149);
            s.store_add(813, 2150, 2151);
            s.store_ad_value(812, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(2152), A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.5, s.ad_value(2150), 1.0), 1.0));
            s.store_add_ad_rhs(2153, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.3333333333333333, s.ad_value(2150), 1.0))), 1.0));
        }

        s.b[2181] = (s.v[2153] < 230.25850929940458);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (((!s.b[2179]) && s.b[2180]) && s.b[2181]) {
            s.store_exp(2154, 2153);
        }

        if (((!s.b[2179]) && s.b[2180]) && (!s.b[2181])) {
            s.store_scaled_offset_ad(2154, A::mul_offset_lhs(s.ad_value(2153), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2153), (-230.25850929940458), A::scale_offset(s.ad_value(2153), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2153)), 2.0);
            s.store_mul_square_lhs(2156, 2153, 2144);
            s.store_mul3_affine_lhs(2157, 2153, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2147, 2153);
            s.store_mul(2145, 2060, 2155);
            s.store_ad_value(2159, A::add_scaled_product(s.ad_value(2144), 2.0, s.ad_value(2021), A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2145), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2157)), 1.0, (-1.0)), 1.0));
            s.store_ad_value(2160, A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2153), (-1.0), s.ad_value(2145), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::sub(A::offset(s.ad_value(2153), (-1.0)), s.ad_value(2156)), 1.0), (-1.0)));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2154), 1.0, s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0))));
            s.store_ad_value(2144, A::add_scaled_square_product(s.ad_value(2159), 1.0, s.ad_value(2160), s.ad_value(2144), (-2.0)));
            s.store_ad_value(2059, A::sub_scaled_inputs(s.ad_value(2153), -1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_scaled_input(2161, 1.0, 2020, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2162, A::mul_scaled_lhs(s.ad_value(2056), 1.25, s.ad_value(2161)), (-1.0), 2161);
            s.store_mul_ad_product_rhs(2163, 2039, s.ad_value(2058), A::offset(A::mul(s.ad_value(2162), s.ad_value(2039)), 1.0));
        }

        s.b[2182] = ((-s.v[2163]) > (-230.25850929940458));
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2182]) {
            s.store_exp_neg_input(2144, 2163);
        }

        if (((!s.b[2179]) && (!s.b[2180])) && (!s.b[2182])) {
            s.store_div_from_scalar_offset_ad(2144, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2163)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2163)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar(2164, 1.0, 2144);
            s.store_ad_value(2165, A::add_scaled_inputs_product(s.ad_value(2039), 1.0, s.ad_value(2021), 0.5, s.ad_value(2020), A::sqrt(A::add_scaled_inputs3(s.ad_value(2039), 1.0, s.ad_value(2021), 0.25, s.ad_value(2164), -1.0)), (-1.0)));
            s.store_offset(2166, 2045, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0)), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_ad(2146, 1.0, A::square(s.ad_value(2149)), 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), s.ad_value(2146), 2146);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            let assign42220_ad_e55425: A = {
                if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2060] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2150, assign42220_ad_e55425);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 0.5));
            s.store_ad_value(2151, A::add_scaled_product(s.ad_value(2144), 2.0, s.ad_value(2021), A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2152, A::add_scaled_inputs3(s.ad_value(2045), 1.0, s.ad_value(2149), (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0));
            s.store_add(813, 2150, 2151);
            s.store_ad_value(812, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(2152), A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0));
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2183] = (s.v[2168] < 230.25850929940458);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2183]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2060, 2154);
        }

        s.b[2184] = (s.v[2168] > (s.v[2045] - 230.25850929940458));
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && s.b[2184]) {
            s.store_exp_sub(2154, 2168, 2045);
            s.store_div(2155, 2060, 2154);
        }

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && (!s.b[2184])) {
            s.store_div_from_scalar_offset_ad(2154, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2045), s.ad_value(2168)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2155, 1e-100, A::mul_offset_lhs(s.ad_value(2168), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2168), (-230.25850929940458), A::scale_offset(s.ad_value(2168), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2168)), 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_ad_value(2159, A::add_scaled_product(s.ad_value(2144), 2.0, s.ad_value(2021), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2160, A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0))));
            s.store_ad_value(2144, A::add_scaled_square_product(s.ad_value(2159), 1.0, s.ad_value(2160), s.ad_value(2144), (-2.0)));
            s.store_ad_value(2059, A::add_scaled_inputs(s.ad_value(2168), 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        s.v[2062] = 0.0;

        s.v[2063] = 0.0;

        s.v[2064] = 0.0;

        s.v[2065] = 0.0;

        s.v[2066] = 0.0;

        s.v[2067] = 0.0;

        s.v[2068] = 0.0;

        s.v[2069] = 1.0;

        s.v[2070] = 1.0;

        s.store_sub(2071, 2039, 2059);

        s.v[2072] = 0.0;

        s.store_mul(2073, 2035, 2071);

        s.v[2074] = 1.0;

        s.v[2075] = 1.0;

        s.v[2079] = 1.0;

        s.v[2080] = 1.0;

        s.v[2082] = 1.0;

        s.b[2185] = (s.v[2039] > 0.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if s.b[2185] {
            s.store_div_from_scalar_offset_ad(1929, 1.0, A::square(s.ad_value(2059)), 2.0);
            s.store_mul_square_lhs(2061, 2059, 1929);
            s.store_mul3_affine_lhs(2062, 2059, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs(2063, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2061), 12.0), s.ad_value(1929), 1929);
            s.store_scalar(2064, 0.0);
        }

        s.b[2186] = (s.v[2059] < 230.25850929940458);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2186]) {
            s.store_exp(2064, 2059);
            s.store_div_from_scalar(2065, 1.0, 2064);
            s.store_mul(2064, 2060, 2064);
        }

        s.b[2187] = (s.v[2059] > (s.v[2045] - 230.25850929940458));
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if ((s.b[2185] && (!s.b[2186])) && s.b[2187]) {
            s.store_exp_sub(2064, 2059, 2045);
            s.store_div(2065, 2060, 2064);
        }

        if ((s.b[2185] && (!s.b[2186])) && (!s.b[2187])) {
            s.store_div_from_scalar_offset_ad(2064, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2045), s.ad_value(2059)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2065, 1e-100, A::mul_offset_lhs(s.ad_value(2059), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2059), (-230.25850929940458), A::scale_offset(s.ad_value(2059), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if s.b[2185] {
            s.store_ad_value(2066, A::add_scaled_product(s.ad_value(2064), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2059), 1.0), s.ad_value(2061)), (-1.0)));
        }

        s.b[2188] = (s.v[2059] < 1e-5);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2188]) {
            s.store_ad_value(2067, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2059)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2059), 1.0, A::scale(s.ad_value(2059), 0.25), 0.3333333333333333), 0.5));
            s.store_ad_value(2066, A::mul3_scaled_output(A::mul3(s.ad_value(2060), s.ad_value(2059), s.ad_value(2059)), s.ad_value(2059), A::scale_offset(s.ad_value(2059), 1.75, 1.0), 0.16666666666666666));
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2059), 1.0, A::scale(s.ad_value(2059), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2068, 2059, 1929, 0.7071067811865475);
            s.store_offset_ad(2069, A::div_scaled_product(s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.5)), 1.0, A::square(s.ad_value(2059)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0), 1.0);
        }

        if (s.b[2185] && (!s.b[2188])) {
            s.store_add_ad_lhs(2067, A::offset(s.ad_value(2059), (-1.0)), 2065);
            s.store_sqrt(2068, 2067);
            s.store_offset_scaled_ad(2069, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2065)), s.ad_value(2068)), 0.5, 1.0);
        }

        if s.b[2185] {
            s.store_div_ad(2070, A::offset(A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2019)), 1.0), A::offset(A::mul(s.ad_value(709), s.ad_value(2019)), 1.0));
        }

        s.b[2189] = (s.v[2066] > 1e-100);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_sqrt_ad_rhs(2071, 2020, A::add(s.ad_value(2067), s.ad_value(2066)));
            s.store_ad_value(2072, A::div_scaled_product3(s.ad_value(2021), s.ad_value(2066), s.ad_value(2035), 1.0, A::add_scaled_product(s.ad_value(2071), 1.0, s.ad_value(2020), s.ad_value(2068), 1.0), 1.0));
            s.store_mul3_lhs(2073, 2068, 2020, 2035);
        }

        s.b[2190] = (s.v[215] < 0.0);
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2190]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2074, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2190])) {
            s.store_offset_mul(2074, 215, 2019, 1.0);
        }

        s.b[2191] = (s.v[216] < 0.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2191]) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2072)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2191])) {
            s.store_div_from_scalar_offset_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2072)), 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_ad_lhs(2076, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), 2072);
            s.store_mul_ad_rhs(2077, 763, A::add_scaled_product(s.ad_value(2073), 1.0, s.ad_value(764), s.ad_value(2072), 1.0));
            s.store_ln_ad(1930, A::div(s.ad_value(2067), A::offset(A::add(s.ad_value(2067), s.ad_value(2066)), 1e-14)));
            s.store_ad_value(2078, A::add_scaled_product(A::pow(A::mul(s.ad_value(2077), s.ad_value(705)), s.ad_value(706)), 1.0, s.ad_value(707), A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0));
            s.store_mul_add_ad_lhs(2079, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
        }

        s.b[2192] = (s.v[219] < 0.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2192]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2080, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2192])) {
            s.store_offset_mul(2080, 219, 2019, 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul(1931, 2072, 2080);
            s.store_div_ad_rhs(2081, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.b[2193] = (s.v[220] < 0.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if ((s.b[2185] && s.b[2189]) && s.b[2193]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2193])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        s.copy_ad(1806, 2017);

        s.copy_ad(1807, 2019);

        s.copy_ad(1808, 2035);

        s.copy_ad(1809, 2036);

        s.copy_ad(1810, 2020);

        s.copy_ad(1811, 2021);

        s.copy_ad(1812, 2037);

        s.copy_ad(1813, 2039);

        s.copy_ad(1814, 2044);

        s.copy_ad(1815, 2045);

        s.copy_ad(1816, 2056);

        s.copy_ad(1817, 2057);

        s.copy_ad(1818, 2058);

        s.copy_ad(1819, 2165);

        s.copy_ad(1820, 2060);

        s.copy_ad(1821, 2059);

        s.copy_ad(1822, 2062);

        s.copy_ad(1823, 2063);

        s.copy_ad(1824, 2064);

        s.copy_ad(1825, 2065);

        s.copy_ad(1826, 2067);

        s.copy_ad(1827, 2066);

        s.copy_ad(1828, 2068);

        s.copy_ad(1829, 2069);

        s.copy_ad(1830, 2070);

        s.copy_ad(1831, 2071);

        s.copy_ad(1832, 2072);

        s.copy_ad(1833, 2073);

        s.copy_ad(1834, 2074);

        s.copy_ad(1835, 2075);

        s.copy_ad(1836, 2079);

        s.copy_ad(1837, 2080);

        s.copy_ad(1838, 2082);

        s.v[2084] = 0.0;

        s.store_scale(2083, 2035, 4.60517018598809);

        s.copy_ad(2100, 2083);

        s.copy_ad(2101, 815);

        s.store_mul(2102, 815, 2036);

        s.copy_ad(2106, 2059);

        s.v[2107] = 0.0;

        s.v[2110] = 0.0;

        s.copy_ad(2112, 2065);

        s.copy_ad(2113, 2067);

        s.copy_ad(2115, 2066);

        s.copy_ad(2116, 2073);

        s.copy_ad(2117, 2059);

        s.copy_ad(2118, 2065);

        s.copy_ad(2120, 2066);

        s.copy_ad(2121, 2067);

        s.store_sub(2122, 2039, 2059);

        s.v[2123] = 1.0;

        s.v[2125] = 1.0;

        s.v[2124] = 0.0;

        s.copy_ad(2134, 2072);

        s.store_mul(2138, 2122, 2035);

        s.v[2135] = 0.0;

        s.copy_ad(2136, 2073);

        s.v[2141] = 0.0;

        s.v[2140] = 1.0;

        s.copy_ad(2143, 2015);

        s.copy_ad(2142, 2138);

        s.b[2194] = (s.v[2039] > 0.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        s.b[2195] = (s.v[2066] > 1e-100);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2143, 2015, 2082);
            s.store_div(2084, 2143, 2079);
            s.store_add_scaled_inputs(2085, 2071, 1.0, 2021, 0.5);
            s.store_div_ad_lhs(1929, A::div_scaled_product(s.ad_value(2021), s.ad_value(2064), 1.0, s.ad_value(2085), 1.0), 2085);
        }

        s.b[2196] = (s.v[1929] > 0.0001);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2196]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2197] = (s.v[1930] < 1e-10);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(1931, 1.0);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && (!s.b[2197])) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2196])) {
            s.store_scale(1931, 1929, 0.5);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2086, 1931, 2085);
        }

        s.b[2198] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_scaled_mul(2087, 2035, 2086, 0.475);
            s.store_ad_value(1929, A::add_scaled_product(s.ad_value(2072), 1.0, s.ad_value(2069), s.ad_value(2087), (-1.0)));
            s.store_scaled_add_ad_rhs(2088, 1929, A::sqrt(A::offset(A::square(s.ad_value(1929)), 1e-12)), 0.5);
            s.store_ad_value(2089, A::add_scaled_value_products(s.ad_value(2072), (-1.0), s.ad_value(2035), s.ad_value(2071), 1.0, A::offset(s.ad_value(2069), (-1.0)), s.ad_value(2087), 1.0));
            s.store_offset_ad(2090, A::div_scaled_product(s.ad_value(2021), s.ad_value(2035), 0.5, s.ad_value(2089), 1.0), 1.0);
            s.store_ad_value(1929, A::add_scaled_product(s.ad_value(2089), 1.0, s.ad_value(764), s.ad_value(2088), 1.0));
            s.store_pow_ad(2091, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));
            s.store_mul_ad_lhs(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2090), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2091);
            s.store_div(1929, 2088, 2089);
            s.store_mul_pow_ad_rhs(2092, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));
            s.store_mul_ad_lhs(1931, A::div_scaled_product(s.ad_value(708), A::add(A::offset(s.ad_value(2090), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1929), 1.0))), 1.0, s.ad_value(2089), 1.0), 2092);
            s.store_mul_ad_lhs(2093, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), 2088);
            s.store_offset_div_ad(1929, A::add_scaled_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), s.ad_value(2090), (-1.0)), s.ad_value(1931), 1.0);
        }

        s.b[2199] = (s.v[1929] < 230.25850929940458);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && s.b[2199]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && (!s.b[2199])) {
            s.copy_ad(1930, 1929);
        }

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_ad_value(2094, A::div_scaled_product3(s.ad_value(2087), s.ad_value(1931), s.ad_value(1930), -1.0, A::add_scaled_inputs3_offset(s.ad_value(2091), 1.0, s.ad_value(2092), 1.0, s.ad_value(2093), 1.0, 1.0), 1.0));
            s.store_mul_offset_ad_rhs(2095, 2086, A::div(s.ad_value(2094), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2094)), 1.0)), 1.0)), 1.0);
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2198])) {
            s.copy_ad(2095, 2086);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul3_affine_lhs(2096, 2035, 2084, 0.7071067811865475, 0.0, 2095);
        }

        s.b[2200] = (s.v[0] == (-1.0));
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2200]) {
            s.store_div_ad_rhs(2096, 2096, A::sqrt(A::offset(s.ad_value(2096), 1.0)));
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_div_from_scalar_offset_ad(2097, 2.0, A::sqrt(A::scale_offset(s.ad_value(2096), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2097, 2096);
            s.store_mul_ad_product_rhs(2098, 2095, s.ad_value(2097), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2097)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2097), 4.0), 1.0)), 1.0));
            s.store_scale(2099, 2098, 0.99);
            s.store_ad_value(1929, A::div_scaled_product3(s.ad_value(2099), A::sub_scaled_inputs(s.ad_value(2099), 1.0, s.ad_value(2085), 2.0), s.ad_value(2037), 1.0, s.ad_value(2066), 1.0));
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul_sub_ad_rhs(2100, 2035, s.ad_value(2099), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2194] && (!s.b[2195])) {
            s.copy_ad(2100, 2083);
        }

        if s.b[2194] {
            s.store_offset(1929, 2016, 1.0);
            s.store_ad_value(1930, A::div_scaled_product(A::sqrt(s.ad_value(1929)), s.ad_value(815), 1.0, s.ad_value(2100), 1.0));
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_ad_value(2101, A::div_scaled_product(s.ad_value(2100), s.ad_value(1929), 1.0, A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))), 1.0));
            s.store_mul(2102, 2101, 2036);
            s.store_add(2103, 2045, 2102);
        }

        s.b[2201] = (s.v[2102] < 460.51701859880916);
        s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2201]) {
            s.store_exp_neg_input(2104, 2102);
        }

        if (s.b[2194] && (!s.b[2201])) {
            s.store_div_from_scalar_offset_ad(2104, 1e-200, A::mul_offset_lhs(s.ad_value(2102), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2102), (-460.51701859880916), A::scale_offset(s.ad_value(2102), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if s.b[2194] {
            s.store_mul(2105, 2060, 2104);
        }

        s.b[2202] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2202]) {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2106, 2039, s.ad_value(2058), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2105)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_offset(2166, 2103, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0)), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_ad(2146, 1.0, A::square(s.ad_value(2149)), 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), s.ad_value(2146), 2146);
        }

        if (s.b[2194] && (!s.b[2202])) {
            let assign44400_ad_e57369: A = {
                if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2105] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2150, assign44400_ad_e57369);
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 0.5));
            s.store_ad_value(2151, A::add_scaled_product(s.ad_value(2144), 2.0, s.ad_value(2021), A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2152, A::add_scaled_inputs3(s.ad_value(2103), 1.0, s.ad_value(2149), (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0));
            s.store_add(813, 2150, 2151);
            s.store_ad_value(812, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(2152), A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0));
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2203] = (s.v[2168] < 230.25850929940458);
        s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2202])) && s.b[2203]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2105, 2154);
        }

        s.b[2204] = (s.v[2168] > (s.v[2103] - 230.25850929940458));
        s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
            s.store_exp_sub(2154, 2168, 2103);
            s.store_div(2155, 2105, 2154);
        }

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && (!s.b[2204])) {
            s.store_div_from_scalar_offset_ad(2154, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2103), s.ad_value(2168)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2155, 1e-100, A::mul_offset_lhs(s.ad_value(2168), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2168), (-230.25850929940458), A::scale_offset(s.ad_value(2168), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2168)), 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_ad_value(2159, A::add_scaled_product(s.ad_value(2144), 2.0, s.ad_value(2021), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2160, A::add_scaled_square_product(s.ad_value(2144), 1.0, s.ad_value(2021), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0))));
            s.store_ad_value(2144, A::add_scaled_square_product(s.ad_value(2159), 1.0, s.ad_value(2160), s.ad_value(2144), (-2.0)));
            s.store_ad_value(2106, A::add_scaled_inputs(s.ad_value(2168), 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0));
        }

        if s.b[2194] {
            s.store_sub(2107, 2106, 2059);
        }

        s.b[2205] = (s.v[2107] < 1e-10);
        s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2205]) {
            s.store_ad_value(2108, A::add_scaled_inputs_product(s.ad_value(2039), 2.0, s.ad_value(2059), (-2.0), s.ad_value(2021), A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0), 1.0, s.ad_value(2105), s.ad_value(2062), 1.0, (-1.0)), 1.0));
            s.store_mul_ad_lhs(2109, A::mul_sub_from_scalar_rhs(s.ad_value(2021), 1.0, s.ad_value(2104)), 2066);
        }

    }

    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2194] && s.b[2205]) {
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2021), A::add_scaled_value_products(s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0, s.ad_value(2105), s.ad_value(2063), (-1.0))));
            s.store_ad_value(1929, A::add_scaled_square_product(s.ad_value(2108), 1.0, s.ad_value(1929), s.ad_value(2109), (-2.0)));
            s.store_scaled_div_ad_rhs(2107, 2109, A::add(s.ad_value(2108), A::sqrt(s.ad_value(1929))), 2.0);
            s.store_add(2106, 2059, 2107);
        }

        if s.b[2194] {
            s.store_mul(2110, 2107, 2035);
            s.store_ad_value(2111, A::div_scaled_product_offset_denominator(s.ad_value(2106), s.ad_value(2106), 1.0, A::square(s.ad_value(2106)), 2.0, 1.0));
        }

        s.b[2206] = (s.v[2106] < 230.25850929940458);
        s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2206]) {
            s.store_exp_neg_input(2112, 2106);
        }

        s.b[2207] = (s.v[2106] < 1e-5);
        s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2206]) && s.b[2207]) {
            s.store_ad_value(2113, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2106)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2106), 1.0, A::scale(s.ad_value(2106), 0.25), 0.3333333333333333), 0.5));
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2106), 1.0, A::scale(s.ad_value(2106), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2114, 2106, 1929, 0.7071067811865475);
            s.store_ad_value(2115, A::mul3(A::mul3_scaled_output(s.ad_value(2105), s.ad_value(2106), s.ad_value(2106), 0.16666666666666666), s.ad_value(2106), A::scale_offset(s.ad_value(2106), 1.75, 1.0)));
        }

        if ((s.b[2194] && s.b[2206]) && (!s.b[2207])) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
            s.store_sqrt(2114, 2113);
            s.store_mul_ad_rhs(2115, 2105, A::add_scaled_inputs3_offset(A::div_from_scalar(1.0, s.ad_value(2112)), 1.0, s.ad_value(2106), (-1.0), s.ad_value(2111), -1.0, (-1.0)));
        }

        s.b[2208] = (s.v[2106] > (s.v[2103] - 230.25850929940458));
        s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2206])) && s.b[2208]) {
            s.store_exp_sub(1929, 2106, 2103);
            s.store_div(2112, 2105, 1929);
            s.store_ad_value(2115, A::add_scaled_product(s.ad_value(1929), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0)));
        }

        if ((s.b[2194] && (!s.b[2206])) && (!s.b[2208])) {
            s.store_div_from_scalar_offset_ad(2112, 1e-100, A::mul_offset_lhs(s.ad_value(2106), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2106), (-230.25850929940458), A::scale_offset(s.ad_value(2106), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1929, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2103), s.ad_value(2106)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_ad_value(2115, A::add_scaled_product(s.ad_value(1929), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0)));
        }

        if (s.b[2194] && (!s.b[2206])) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
            s.store_sqrt(2114, 2113);
        }

        if s.b[2194] {
            s.store_mul3_lhs(2116, 2114, 2020, 2035);
            s.store_scaled_add(2117, 2059, 2106, 0.5);
            s.store_scalar(2118, 0.0);
            s.store_mul(1929, 2112, 2065);
        }

        s.b[2209] = (s.v[1929] > 0.0);
        s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2209]) {
            s.store_sqrt(2118, 1929);
        }

        if s.b[2194] {
            s.store_scaled_add(2119, 2066, 2115, 0.5);
            s.store_ad_value(2120, A::add_scaled_product(s.ad_value(2119), 1.0, A::square(s.ad_value(2107)), A::sub_scaled_inputs(s.ad_value(2118), 1.0, s.ad_value(2037), 2.0), 0.125));
        }

        s.b[2210] = (s.v[2117] < 1e-5);
        s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2210]) {
            s.store_ad_value(2121, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2117)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2117), 1.0, A::scale(s.ad_value(2117), 0.25), 0.3333333333333333), 0.5));
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

        s.b[2211] = (s.v[719] > 0.0);
        s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2210]) && s.b[2211]) {
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
        }

        if (s.b[2194] && s.b[2210]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2117), 1.0, A::scale(s.ad_value(2117), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2124, 2117, 1929, 0.7071067811865475);
            s.store_add_ad_rhs(2125, 2123, A::div_scaled_product(s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.5)), 1.0, A::square(s.ad_value(2117)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

        s.b[2212] = (s.v[719] > 0.0);
        s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2210])) && s.b[2212]) {
            s.store_ad_value(2126, A::add_scaled_sub_value_product(1.0, s.ad_value(2118), 1.0, s.ad_value(2122), s.ad_value(2037), 2.0));
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
            s.store_div_ad_rhs(1929, 2123, A::offset(s.ad_value(2123), 1.0));
            s.store_mul_ad_rhs(2127, 719, A::mul3(A::square(s.ad_value(1929)), s.ad_value(2021), s.ad_value(2120)));
            s.store_ad_value(2128, A::add_scaled_inputs_product(s.ad_value(2122), 2.0, s.ad_value(2127), (-2.0), s.ad_value(2021), A::add(A::sub_from_scalar(1.0, s.ad_value(2118)), s.ad_value(2120)), 1.0));
            s.store_mul_ad_rhs(2129, 2127, A::sub_scaled_inputs(s.ad_value(2127), 1.0, s.ad_value(2122), 2.0));
            s.store_sub_from_scalar_ad(2130, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add(s.ad_value(2118), s.ad_value(2120)), 0.5));
            s.store_ad_value(2131, A::div_scaled_product(s.ad_value(2129), s.ad_value(2128), 1.0, A::add_scaled_square_product(s.ad_value(2128), 1.0, s.ad_value(2130), s.ad_value(2129), (-1.0)), 1.0));
            s.store_add(2117, 2117, 2131);
            s.store_exp(2132, 2131);
            s.store_div(2118, 2118, 2132);
            s.store_mul(2120, 2120, 2132);
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
            s.store_add_ad(2133, A::sub_from_scalar(1.0, s.ad_value(2118)), A::mul3_scaled_output(s.ad_value(2122), s.ad_value(2123), s.ad_value(2037), 2.0));
            s.store_ad_value(2107, A::div_scaled_product3(s.ad_value(2107), s.ad_value(2132), A::add(s.ad_value(2126), s.ad_value(2119)), 1.0, A::add_scaled_product(s.ad_value(2133), 1.0, s.ad_value(2132), s.ad_value(2119), 1.0), 1.0));
            s.store_mul(2110, 2107, 2035);
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_sqrt(2124, 2121);
            s.store_ad_value(2125, A::add_scaled_inputs(s.ad_value(2123), 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2118)), s.ad_value(2124)), 0.5));
        }

        if s.b[2194] {
            s.store_mul_ad_rhs(2134, 2035, A::div_scaled_product(s.ad_value(2021), s.ad_value(2120), 1.0, A::add_scaled_product(s.ad_value(2122), 1.0, s.ad_value(2020), s.ad_value(2124), 1.0), 1.0));
            s.store_ad_value(2135, A::add_scaled_product(s.ad_value(2134), 1.0, s.ad_value(2035), s.ad_value(2125), 1.0));
            s.store_mul3_lhs(2136, 2124, 2020, 2035);
        }

        s.b[2213] = (s.v[216] < 0.0);
        s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2213]) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2134)));
        }

        if (s.b[2194] && (!s.b[2213])) {
            s.store_div_from_scalar_offset_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2134)), 1.0);
        }

        if s.b[2194] {
            s.store_mul_ad_lhs(2076, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), 2134);
            s.store_ad_value(2137, A::add_scaled_product(s.ad_value(2136), 1.0, s.ad_value(764), s.ad_value(2134), 1.0));
            s.store_ad_value(2138, A::add_scaled_product(s.ad_value(2136), 1.0, s.ad_value(765), s.ad_value(2134), 1.0));
            s.store_mul(2139, 763, 2137);
            s.store_ln_ad(1930, A::div(s.ad_value(2121), A::offset(A::add(s.ad_value(2121), s.ad_value(2120)), 1e-14)));
            s.store_ad_value(2078, A::add_scaled_product(A::pow(A::mul(s.ad_value(2139), s.ad_value(705)), s.ad_value(706)), 1.0, s.ad_value(707), A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0));
            s.store_mul_add_ad_lhs(2140, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
            s.store_ln_ad(2141, A::div(A::offset(A::mul(A::sub(s.ad_value(815), s.ad_value(2110)), s.ad_value(768)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2101), s.ad_value(2110)), s.ad_value(768)), 1.0)));
            s.store_mul(1931, 2134, 2080);
            s.store_div_ad_rhs(2081, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.b[2214] = (s.v[220] < 0.0);
        s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2214]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if (s.b[2194] && (!s.b[2214])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        if s.b[2194] {
            s.store_mul(2143, 2015, 2082);
            s.store_mul(2142, 2122, 2035);
        }

        s.copy_ad(1839, 2083);

        s.copy_ad(1841, 2101);

        s.copy_ad(1842, 2102);

        s.copy_ad(1843, 2107);

        s.copy_ad(1844, 2110);

        s.copy_ad(1846, 2117);

        s.copy_ad(1845, 2116);

        s.copy_ad(1847, 2123);

        s.copy_ad(1848, 2125);

        s.copy_ad(1849, 2134);

        s.copy_ad(1850, 2135);

        s.copy_ad(1851, 2136);

        s.copy_ad(1852, 2138);

        s.copy_ad(1853, 2140);

        s.copy_ad(1855, 2141);

        s.copy_ad(1854, 2143);

        s.copy_ad(1856, 2142);

        s.v[1857] = 1.0;

        s.v[1858] = 1.0;

        s.v[1860] = 1.0;

        s.v[1861] = 1.0;

        s.v[827] = 0.0;

        s.b[2215] = (s.v[1813] > 0.0);
        s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };

        if s.b[2215] {
            s.store_ln_ad(1939, A::offset(A::mul(s.ad_value(819), s.ad_value(768)), 1.0));
            s.store_ad_value(1929, A::div_scaled_product(s.ad_value(1808), s.ad_value(1848), 1.0, s.ad_value(1850), 1.0));
            s.store_ad_value(1938, A::add_scaled_product(A::mul3(A::mul3(s.ad_value(225), s.ad_value(1851), s.ad_value(1929)), s.ad_value(1929), s.ad_value(1939)), 1.0, A::div_scaled_product(A::add(s.ad_value(223), A::div(s.ad_value(224), s.ad_value(1850))), s.ad_value(1849), 1.0, s.ad_value(1850), 1.0), s.ad_value(1855), 1.0));
            s.store_div_from_scalar_add_ad(1857, 1.0, A::offset(s.ad_value(1938), 1.0), A::square(s.ad_value(1938)));
            s.store_mul(1858, 1853, 1857);
            s.store_div(1859, 1854, 1858);
            s.store_mul_ad_product_lhs(1940, A::square(s.ad_value(1859)), s.ad_value(1844), 1844);
        }

        s.b[2216] = (s.v[0] == (-1.0));
        s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };

        if (s.b[2215] && s.b[2216]) {
            s.store_div_ad_rhs(1940, 1940, A::offset(A::mul(s.ad_value(1859), s.ad_value(1844)), 1.0));
        }

        if s.b[2215] {
            s.store_ad_value(1941, A::mul_offset_rhs_scaled_output(s.ad_value(1858), A::sqrt(A::scale_offset(s.ad_value(1940), 2.0, 1.0)), 1.0, 0.5));
            s.store_div_from_scalar(1860, 1.0, 1941);
            s.store_mul(1929, 1858, 1860);
            s.store_mul_offset_ad_rhs(1942, 1848, A::mul3_scaled_output(s.ad_value(1940), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_ad_value(1861, A::div_scaled_product(s.ad_value(1929), s.ad_value(1850), 1.0, s.ad_value(1942), 1.0));
            s.store_mul_ad_lhs(827, A::mul3(s.ad_value(1917), s.ad_value(1850), s.ad_value(1844)), 1860);
        }

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.b[2217] = (((((p.p40 != 0.0) && ((s.v[235] > 0.0) || (s.v[236] > 0.0))) || ((p.p42 != 0.0) && ((s.v[245] > 0.0) || (s.v[246] > 0.0)))) || (s.v[260] > 0.0)) || (s.v[261] > 0.0));
        s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };

        if s.b[2217] {
            s.store_scaled_add_ad_rhs(1943, 1801, A::sqrt(A::add(A::square(s.ad_value(1801)), s.ad_value(778))), 0.5);
            s.store_add_ad_lhs(1944, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(773), (-0.5), s.ad_value(771), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), 1.0), 780);
            s.store_scaled_add_ad_rhs(1943, 1802, A::sqrt(A::add(A::square(s.ad_value(1802)), s.ad_value(781))), 0.5);
            s.store_add_ad_lhs(1945, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(774), (-0.5), s.ad_value(772), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), 1.0), 783);
            s.store_scaled_add(1862, 1801, 1944, (-s.v[355]));
            s.store_scaled_add(1863, 1802, 1945, (-s.v[355]));
        }

        s.v[828] = 0.0;

        s.v[829] = 0.0;

        s.v[1972] = 0.0;

        s.v[832] = 0.0;

        s.v[830] = 0.0;

        s.v[831] = 0.0;

        s.b[2218] = (p.p40 != 0.0);
        s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };

        s.b[2219] = (s.v[235] > 0.0);
        s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1862)), 1e-6), 784);
        }

        s.b[2220] = (s.v[241] < 0.0);
        s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2219]) && s.b[2220]) {
            s.store_ad_value(1946, A::add_scaled_inputs3(s.ad_value(1946), 0.5, s.ad_value(790), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(790)), A::sub(s.ad_value(1946), s.ad_value(790))), 1e-6)), (-0.5)));
        }

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_offset_ad_rhs(1929, 787, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(1946), 1.0)), (-1.5));
        }

        s.b[2221] = (s.v[1929] > 0.0);
        s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2219]) && s.b[2221]) {
            s.store_offset_ad(1947, A::mul_offset_rhs(s.ad_value(1929), A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2222] = (s.v[1929] > (-230.25850929940458));
        s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };

    }
}
