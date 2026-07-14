#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845, 0.0);}
        s.b[1485] = (p.p854 > 1000.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1485]) {s.store_scalar(1221, 1.0);}
        s.b[1486] = (s.v[1194] > ((-s.v[438]) * p.p854));s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });s.b[1487] = (p.p857 == 4.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) && s.b[1487]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) && (!s.b[1487])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && s.b[1486]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1485])) && (!s.b[1486])) {s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1471])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1488] = (s.v[642] == 0.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1488]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1488])) {s.store_primal_scale(1196, 1186, s.v[383]);}
        s.b[1489] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1489]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {s.store_primal_sub_from_scalar(1198, s.v[389], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1490] = (p.p826 == 0.5);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && s.b[1490]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && (!s.b[1490])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1491] = (p.p826 == 0.5);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && s.b[1491]) {s.store_sqrt_scaled_input(1195, 1198, s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) && (!s.b[1491])) {s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1489])) {s.store_scale(1202, 1195, s.v[419]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(1197, 1203, 1201, p.p835);}
        s.b[1492] = (p.p840 == 0.0);s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1492]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1493] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1493]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1493])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[431]), 1206, 1209, s.v[431], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1494] = (s.v[1216] > 0.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1494]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1494])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1495] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1495]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1495])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1496] = (s.v[1216] > 0.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && s.b[1496]) {s.copy_ad(1217, 1179);}
        s.b[1497] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) && s.b[1497]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) && (!s.b[1497])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1496])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1492])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);}
        s.b[1498] = (p.p846 == 0.0);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1498]) {s.store_scalar(1219, 0.0);}
        s.b[1499] = (p.p826 == 0.5);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && s.b[1499]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1499])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 1195, 1.0);}
        s.b[1500] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && s.b[1500]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));}
        s.b[1501] = (((-s.v[437]) / s.v[1220]) < 0.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1500])) && s.b[1501]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) && (!s.b[1500])) && (!s.b[1501])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1498])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846, 0.0);}
        s.b[1502] = (p.p855 > 1000.0);s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1488])) && s.b[1502]) {s.store_scalar(1221, 1.0);}
        s.b[1503] = (s.v[1194] > ((-s.v[438]) * p.p855));s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });s.b[1504] = (p.p858 == 4.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) && s.b[1504]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) && (!s.b[1504])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && s.b[1503]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1488])) && (!s.b[1502])) && (!s.b[1503])) {s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1488])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(473, 640, 1222, 1.0, 641, 1223, 1.0, 642, 1224, 1.0);s.store_primal_add_scaled_inputs3_indices(661, 640, s.v[381], 641, s.v[382], 642, s.v[383]);s.store_add_scaled_offset_product_rhs_mixed_iia(477, 472, 1.0, 661, A::exp_scaled_input(s.ad_value(482), (s.v[365] * s.v[662])), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(478, 473, 1.0, 661, A::exp_scaled_input(s.ad_value(483), (s.v[365] * s.v[662])), (-1.0), (-1.0));}
        s.b[1505] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = ((s.v[472] > 0.0) && (s.v[473] > 0.0));s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });s.b[1507] = ((((((s.v[477] / s.v[472]) > 0.001) || ((s.v[478] / s.v[473]) > 0.001)) && (s.v[477] > 0.0)) && (s.v[478] > 0.0)) && (s.v[478] > s.v[477]));s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1506]) && s.b[1507]) {s.store_div(484, 477, 478);s.store_div_scaled_inputs(664, A::ln(s.ad_value(484)), s.v[364], A::sub(s.ad_value(482), s.ad_value(483)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1506]) && s.b[1507]) {s.store_div_scaled_value_offset_denominator(663, s.ad_value(477), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(482), s.v[365], s.ad_value(664))), (-1.0), 1.0);}
        if ((s.b[1159] && s.b[1176]) && s.b[1505]) {s.store_add_scaled_offset_product_rhs_mixed_aia(474, A::add_scaled_offset_product_rhs(s.ad_value(469), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(479), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(479), s.v[365], s.ad_value(664))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(475, A::add_scaled_offset_product_rhs(s.ad_value(470), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(480), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(480), s.v[365], s.ad_value(664))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(476, A::add_scaled_offset_product_rhs(s.ad_value(471), 1.0, s.ad_value(661), A::exp_scaled_input(s.ad_value(481), (s.v[365] * s.v[662])), (-1.0), (-1.0)), 1.0, 663, A::exp(A::mul_scaled_lhs(s.ad_value(481), s.v[365], s.ad_value(664))), (-1.0), (-1.0));}
        s.b[1508] = (((s.v[469] < 0.0) && (s.v[470] < 0.0)) && (s.v[471] < 0.0));s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });s.b[1509] = (((((((s.v[474] / s.v[469]) > 0.001) || ((s.v[475] / s.v[470]) > 0.001)) || ((s.v[476] / s.v[471]) > 0.001)) && (s.v[474] < 0.0)) && (s.v[475] < 0.0)) && (s.v[476] < 0.0));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) {s.store_div(484, 474, 475);s.store_div_scaled_inputs(485, A::ln(s.ad_value(484)), (-s.v[364]), A::sub(s.ad_value(479), s.ad_value(480)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(487, 480, 480, 1.0, 479, -1.0);s.store_scaled_mul_ad(488, A::offset(s.ad_value(484), (-1.0)), A::offset(A::pow(s.ad_value(484), s.ad_value(487)), (-1.0)), s.v[364]);s.store_primal_div_add_scaled_inputs_rhs_indices(487, 479, 479, 1.0, 480, -1.0);s.store_sub_mixed_ai(489, A::add_scaled_products(A::pow(s.ad_value(484), s.ad_value(487)), A::sub(s.ad_value(480), s.ad_value(479)), 1.0, s.ad_value(484), s.ad_value(479), 1.0), 480);s.store_div(486, 488, 489);s.store_add(666, 485, 486);}
        s.b[1510] = (((((s.v[481] * s.v[365]) * s.v[666])) as f64).abs() < 1e-6);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && s.b[1510]) {
        (1.0,)
    } else {
        (s.v[660],)
    }
};
        s.store_scalar(660, t0);
        if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && s.b[1510]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(665, 476, A::div_from_scalar(1.0, s.ad_value(481)), 1.0, 666, (0.5 * s.v[365]));s.store_div_scaled_product_indices(666, 476, 666, ((-0.5) * s.v[365]), 481, 1.0);}
        let (t1,) = {
    if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {
        (0.0,)
    } else {
        (s.v[660],)
    }
};
        s.store_scalar(660, t1);
        if (((((s.b[1159] && s.b[1176]) && s.b[1505]) && s.b[1508]) && s.b[1509]) && (!s.b[1510])) {s.store_div_scaled_value_offset_denominator(665, s.ad_value(476), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(481), (-s.v[365]), s.ad_value(666))), (-1.0), 1.0);}
        let (t8,) = {
    if (s.b[1159] && s.b[1176]) {
        let t2: f64 = (s.v[640] * s.v[408]);let t3: f64 = (s.v[641] * s.v[409]);let t4: f64 = (t2 + t3);let t5: f64 = (s.v[642] * s.v[410]);let t6: f64 = (t4 + t5);let t7: f64 = (p.p922 * t6);
        (t7,)
    } else {
        (s.v[495],)
    }
};
        s.store_scalar(495, t8);s.b[1511] = ((s.v[640] * s.v[408]) <= s.v[495]);s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1511]) {
        (0.0,)
    } else {
        (s.v[645],)
    }
};
        s.store_scalar(645, t9);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1512] = ((s.v[641] * s.v[409]) <= s.v[495]);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1512]) {
        (0.0,)
    } else {
        (s.v[646],)
    }
};
        s.store_scalar(646, ta);s.b[1513] = ((s.v[642] * s.v[410]) <= s.v[495]);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1159] && s.b[1176]) && s.b[1513]) {
        (0.0,)
    } else {
        (s.v[647],)
    }
};
        s.store_scalar(647, tb);s.b[1514] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1514]) {s.store_primal_ln_ad(654, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(661), 1e-21, 1.0));s.store_ln_ad(656, A::div_scalar_offset_denominator((0.5 * p.p815), s.ad_value(663), 1e-21, 1.0));s.store_ln_ad(658, A::div_scalar_offset_denominator((0.5 * p.p815), A::abs(s.ad_value(665)), 1e-21, 1.0));}
        if (s.b[1159] && s.b[1176]) {s.store_primal_min_with_scalar(654, 654, 230.25850929940458);s.store_primal_exp(655, 654);s.store_min_with_scalar(656, 656, 230.25850929940458);s.store_exp(657, 656);s.store_min_with_scalar(658, 658, 230.25850929940458);s.store_exp(659, 658);s.store_scalar(492, 0.4);s.store_scalar(493, 0.65);s.store_scalar(494, 0.8);s.store_primal_mul_scale_offset_indices(479, 546, 492, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(480, 546, 493, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(481, 546, 494, -1.0, 0.0);s.store_scalar(482, 0.1);s.store_scalar(483, 0.2);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1515] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });s.b[1516] = (s.v[479] < s.v[675]);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });s.b[1517] = (((((-0.5) * (s.v[479] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && s.b[1517]) {s.store_primal_exp_scaled_input(1188, 479, (s.v[365] * (-0.5)));}
        s.b[1518] = (((-0.5) * (s.v[479] * s.v[365])) < 0.0);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && (!s.b[1517])) && s.b[1518]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(479), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) && (!s.b[1517])) && (!s.b[1518])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(479), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(479), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1516]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && (!s.b[1516])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 676, A::sub_scaled_inputs(s.ad_value(479), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1515]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1519] = (s.v[479] > 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && s.b[1519]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1515]) && (!s.b[1519])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 479);}
        if ((s.b[1159] && s.b[1176]) && s.b[1515]) {s.store_primal_sub(1191, 677, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 479, 0.5, 1191, 0.5, 479, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 479, 0.5, 680, 0.5, 479, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 479, A::sqrt_square_offset(s.ad_value(479), ((4.0 * 1e-6) * 1e-6)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        s.b[1520] = (s.v[667] == 0.0);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1520]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1520])) {s.store_primal_mul(1196, 557, 1186);}
        s.b[1521] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1521]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {s.store_primal_sub(1198, 563, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1522] = (s.v[505] == 0.5);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && s.b[1522]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && (!s.b[1522])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1523] = (s.v[505] == 0.5);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && s.b[1523]) {s.store_sqrt_mul(1195, 1198, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) && (!s.b[1523])) {s.store_pow_mul_base_indices(1195, 1198, 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1521])) {s.store_mul(1202, 584, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 516, 1203, 1201);}
        s.b[1524] = (s.v[519] == 0.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1524]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1525] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1525]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1525])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, 596, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1526] = (s.v[1216] > 0.0);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1526]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1526])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1527] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1527]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1527])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1528] = (s.v[1216] > 0.0);s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && s.b[1528]) {s.copy_ad(1217, 1179);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1529] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) && s.b[1529]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) && (!s.b[1529])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1528])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1524])) {s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);}
        s.b[1530] = (s.v[525] == 0.0);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1530]) {s.store_scalar(1219, 0.0);}
        s.b[1531] = (s.v[505] == 0.5);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && s.b[1531]) {s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1531])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);}
        s.b[1532] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && s.b[1532]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));}
        s.b[1533] = (((-s.v[602]) / s.v[1220]) < 0.0);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1532])) && s.b[1533]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) && (!s.b[1532])) && (!s.b[1533])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1530])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1534] = (s.v[534] > 1000.0);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1520])) && s.b[1534]) {s.store_scalar(1221, 1.0);}
        s.b[1535] = (s.v[1194] > ((-s.v[438]) * s.v[534]));s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });s.b[1536] = (s.v[537] == 4.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) && s.b[1536]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) && (!s.b[1536])) {s.store_pow_abs_mul_base_indices(1195, 1194, 608, 537);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && s.b[1535]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1520])) && (!s.b[1534])) && (!s.b[1535])) {s.store_add_scaled_product_mixed_iai(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1520])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1537] = (s.v[668] == 0.0);s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1537]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1537])) {s.store_primal_mul(1196, 558, 1186);}
        s.b[1538] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1538]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {s.store_primal_sub(1198, 564, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1539] = (s.v[506] == 0.5);s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && s.b[1539]) {s.store_scalar(1200, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && (!s.b[1539])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1540] = (s.v[506] == 0.5);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && s.b[1540]) {s.store_sqrt_mul(1195, 1198, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) && (!s.b[1540])) {s.store_pow_mul_base_indices(1195, 1198, 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1538])) {s.store_mul(1202, 585, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 517, 1203, 1201);}
        s.b[1541] = (s.v[520] == 0.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1541]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1542] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1542]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1542])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, 597, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1543] = (s.v[1216] > 0.0);s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1543]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1543])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1544] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1544]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1544])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1545] = (s.v[1216] > 0.0);s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && s.b[1545]) {s.copy_ad(1217, 1179);}
        s.b[1546] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) && s.b[1546]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) && (!s.b[1546])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1545])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1541])) {s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);}
        s.b[1547] = (s.v[526] == 0.0);s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1547]) {s.store_scalar(1219, 0.0);}
        s.b[1548] = (s.v[506] == 0.5);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && s.b[1548]) {s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1548])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);}
        s.b[1549] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && s.b[1549]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));}
        s.b[1550] = (((-s.v[603]) / s.v[1220]) < 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1549])) && s.b[1550]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) && (!s.b[1549])) && (!s.b[1550])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1547])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1551] = (s.v[535] > 1000.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1537])) && s.b[1551]) {s.store_scalar(1221, 1.0);}
        s.b[1552] = (s.v[1194] > ((-s.v[438]) * s.v[535]));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = (s.v[538] == 4.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) && s.b[1553]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) && (!s.b[1553])) {s.store_pow_abs_mul_base_indices(1195, 1194, 609, 538);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && s.b[1552]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1537])) && (!s.b[1551])) && (!s.b[1552])) {s.store_add_scaled_product_mixed_iai(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1537])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1554] = (s.v[669] == 0.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1554]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1554])) {s.store_primal_mul(1196, 559, 1186);}
        s.b[1555] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1555]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {s.store_primal_sub(1198, 565, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1556] = (s.v[507] == 0.5);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && s.b[1556]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && (!s.b[1556])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(507), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1557] = (s.v[507] == 0.5);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && s.b[1557]) {s.store_sqrt_mul(1195, 1198, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) && (!s.b[1557])) {s.store_pow_mul_base_indices(1195, 1198, 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1555])) {s.store_mul(1202, 586, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 518, 1203, 1201);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
    ) {
        s.b[1558] = (s.v[521] == 0.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1558]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1559] = (((-s.v[507]) * s.v[574]) == (-1.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1559]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1559])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, 598, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1560] = (s.v[1216] > 0.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1560]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1560])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1561] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1561]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1561])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1562] = (s.v[1216] > 0.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && s.b[1562]) {s.copy_ad(1217, 1179);}
        s.b[1563] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) && s.b[1563]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) && (!s.b[1563])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1562])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1558])) {s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);}
        s.b[1564] = (s.v[527] == 0.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1564]) {s.store_scalar(1219, 0.0);}
        s.b[1565] = (s.v[507] == 0.5);s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && s.b[1565]) {s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1565])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);}
        s.b[1566] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && s.b[1566]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));}
        s.b[1567] = (((-s.v[604]) / s.v[1220]) < 0.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1566])) && s.b[1567]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) && (!s.b[1566])) && (!s.b[1567])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1564])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(479), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1568] = (s.v[536] > 1000.0);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1554])) && s.b[1568]) {s.store_scalar(1221, 1.0);}
        s.b[1569] = (s.v[1194] > ((-s.v[438]) * s.v[536]));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });s.b[1570] = (s.v[539] == 4.0);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) && s.b[1570]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) && (!s.b[1570])) {s.store_pow_abs_mul_base_indices(1195, 1194, 610, 539);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && s.b[1569]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1554])) && (!s.b[1568])) && (!s.b[1569])) {s.store_add_scaled_product_mixed_iai(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1554])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(469, 667, 1222, 1.0, 668, 1223, 1.0, 669, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1571] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });s.b[1572] = (s.v[480] < s.v[675]);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });s.b[1573] = (((((-0.5) * (s.v[480] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && s.b[1573]) {s.store_primal_exp_scaled_input(1188, 480, (s.v[365] * (-0.5)));}
        s.b[1574] = (((-0.5) * (s.v[480] * s.v[365])) < 0.0);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && (!s.b[1573])) && s.b[1574]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(480), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) && (!s.b[1573])) && (!s.b[1574])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(480), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(480), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1572]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && (!s.b[1572])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 676, A::sub_scaled_inputs(s.ad_value(480), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1571]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1575] = (s.v[480] > 0.0);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && s.b[1575]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1571]) && (!s.b[1575])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 480);}
        if ((s.b[1159] && s.b[1176]) && s.b[1571]) {s.store_primal_sub(1191, 677, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 480, 0.5, 1191, 0.5, 480, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 480, 0.5, 680, 0.5, 480, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
    ) {
        if ((s.b[1159] && s.b[1176]) && s.b[1571]) {s.store_primal_scaled_sub_mixed_ia(1194, 480, A::sqrt_square_offset(s.ad_value(480), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1576] = (s.v[667] == 0.0);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1576]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1576])) {s.store_primal_mul(1196, 557, 1186);}
        s.b[1577] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1577]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {s.store_primal_sub(1198, 563, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1578] = (s.v[505] == 0.5);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && s.b[1578]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && (!s.b[1578])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1579] = (s.v[505] == 0.5);s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && s.b[1579]) {s.store_sqrt_mul(1195, 1198, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) && (!s.b[1579])) {s.store_pow_mul_base_indices(1195, 1198, 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1577])) {s.store_mul(1202, 584, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 554, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 516, 1203, 1201);}
        s.b[1580] = (s.v[519] == 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1580]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {s.store_mul_div_scaled_product_indices(1205, 599, 1202, 569, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 596, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1581] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1581]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1581])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(596), s.ad_value(1206), s.ad_value(1209)), 1.0, 596, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1582] = (s.v[1216] > 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1582]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1582])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1583] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1583]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1583])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1584] = (s.v[1216] > 0.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && s.b[1584]) {s.copy_ad(1217, 1179);}
        s.b[1585] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) && s.b[1585]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) && (!s.b[1585])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1584])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1580])) {s.store_div_scaled_product_indices(1218, 596, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 519, 1203, 1218, 1212, 1.0);}
        s.b[1586] = (s.v[525] == 0.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1586]) {s.store_scalar(1219, 0.0);}
        s.b[1587] = (s.v[505] == 0.5);s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && s.b[1587]) {s.store_sqrt_mul_sub_lhs(1195, 502, 1193, 590);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1587])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(502), s.ad_value(1193)), 590, 505);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 572, A::sub(s.ad_value(502), s.ad_value(1193)), 587, 1.0, 1195, 1.0);}
        s.b[1588] = (((((-s.v[602]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && s.b[1588]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(1220), 1.0));}
        s.b[1589] = (((-s.v[602]) / s.v[1220]) < 0.0);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1588])) && s.b[1589]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 602, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) && (!s.b[1588])) && (!s.b[1589])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 602, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1586])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 525, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1590] = (s.v[534] > 1000.0);s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1576])) && s.b[1590]) {s.store_scalar(1221, 1.0);}
        s.b[1591] = (s.v[1194] > ((-s.v[438]) * s.v[534]));s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });s.b[1592] = (s.v[537] == 4.0);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) && s.b[1592]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(608))), s.ad_value(1194), s.ad_value(608)), 1194, 608);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) && (!s.b[1592])) {s.store_pow_abs_mul_base_indices(1195, 1194, 608, 537);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && s.b[1591]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1576])) && (!s.b[1590])) && (!s.b[1591])) {s.store_add_scaled_product_mixed_iai(1221, 605, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1576])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1593] = (s.v[668] == 0.0);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1593]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1593])) {s.store_primal_mul(1196, 558, 1186);}
        s.b[1594] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1594]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {s.store_primal_sub(1198, 564, 1192);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1595] = (s.v[506] == 0.5);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && s.b[1595]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && (!s.b[1595])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1596] = (s.v[506] == 0.5);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && s.b[1596]) {s.store_sqrt_mul(1195, 1198, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) && (!s.b[1596])) {s.store_pow_mul_base_indices(1195, 1198, 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1594])) {s.store_mul(1202, 585, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 555, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 517, 1203, 1201);}
        s.b[1597] = (s.v[520] == 0.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1597]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {s.store_mul_div_scaled_product_indices(1205, 600, 1202, 570, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 597, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1598] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1598]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1598])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(597), s.ad_value(1206), s.ad_value(1209)), 1.0, 597, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1599] = (s.v[1216] > 0.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1599]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1599])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1600] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1600]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1600])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1601] = (s.v[1216] > 0.0);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && s.b[1601]) {s.copy_ad(1217, 1179);}
        s.b[1602] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) && s.b[1602]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) && (!s.b[1602])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1601])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1597])) {s.store_div_scaled_product_indices(1218, 597, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 520, 1203, 1218, 1212, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1603] = (s.v[526] == 0.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1603]) {s.store_scalar(1219, 0.0);}
        s.b[1604] = (s.v[506] == 0.5);s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && s.b[1604]) {s.store_sqrt_mul_sub_lhs(1195, 503, 1193, 591);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1604])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(503), s.ad_value(1193)), 591, 506);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 573, A::sub(s.ad_value(503), s.ad_value(1193)), 588, 1.0, 1195, 1.0);}
        s.b[1605] = (((((-s.v[603]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && s.b[1605]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(1220), 1.0));}
        s.b[1606] = (((-s.v[603]) / s.v[1220]) < 0.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1605])) && s.b[1606]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 603, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) && (!s.b[1605])) && (!s.b[1606])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 603, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1603])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 526, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1607] = (s.v[535] > 1000.0);s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1593])) && s.b[1607]) {s.store_scalar(1221, 1.0);}
        s.b[1608] = (s.v[1194] > ((-s.v[438]) * s.v[535]));s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });s.b[1609] = (s.v[538] == 4.0);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) && s.b[1609]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(609))), s.ad_value(1194), s.ad_value(609)), 1194, 609);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) && (!s.b[1609])) {s.store_pow_abs_mul_base_indices(1195, 1194, 609, 538);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && s.b[1608]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1593])) && (!s.b[1607])) && (!s.b[1608])) {s.store_add_scaled_product_mixed_iai(1221, 606, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1593])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1610] = (s.v[669] == 0.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1610]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1610])) {s.store_primal_mul(1196, 559, 1186);}
        s.b[1611] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1611]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {s.store_primal_sub(1198, 565, 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1612] = (s.v[507] == 0.5);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && s.b[1612]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && (!s.b[1612])) {s.store_primal_mul_scale_offset(1200, A::add(A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), s.ad_value(1199)), A::scale(s.ad_value(507), 2.0), -1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {s.store_primal_add(1201, 1199, 1200);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
    ) {
        s.b[1613] = (s.v[507] == 0.5);s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && s.b[1613]) {s.store_sqrt_mul(1195, 1198, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) && (!s.b[1613])) {s.store_pow_mul_base_indices(1195, 1198, 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1611])) {s.store_mul(1202, 586, 1195);s.store_mul_ad_product_lhs_mixed_ia(1203, 556, A::offset(s.ad_value(1189), (-1.0)), 1202);s.store_mul3_lhs(1197, 518, 1203, 1201);}
        s.b[1614] = (s.v[521] == 0.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1614]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {s.store_mul_div_scaled_product_indices(1205, 601, 1202, 571, 1.0, 1198, 1.0);s.store_div_scaled_inputs_indices(1206, 598, 0.666666666666667, 1205, 1.0);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1615] = (((-s.v[507]) * s.v[574]) == (-1.0));s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1615]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1615])) {s.store_pow_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1215, A::mul3(s.ad_value(598), s.ad_value(1206), s.ad_value(1209)), 1.0, 598, 1208, (-1.0), 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1616] = (s.v[1216] > 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1616]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1616])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1617] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1617]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1617])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1618] = (s.v[1216] > 0.0);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && s.b[1618]) {s.copy_ad(1217, 1179);}
        s.b[1619] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) && s.b[1619]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) && (!s.b[1619])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1618])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1614])) {s.store_div_scaled_product_indices(1218, 598, 1217, (1.772453850905516 * 0.5), 1213, 1.0);s.store_mul_product3_indices(1204, 521, 1203, 1218, 1212, 1.0);}
        s.b[1620] = (s.v[527] == 0.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1620]) {s.store_scalar(1219, 0.0);}
        s.b[1621] = (s.v[507] == 0.5);s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && s.b[1621]) {s.store_sqrt_mul_sub_lhs(1195, 504, 1193, 592);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1621])) {s.store_pow_mul_base_mixed_ai(1195, A::sub(s.ad_value(504), s.ad_value(1193)), 592, 507);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) {s.store_mul_div_scaled_product_mixed_iaii(1220, 574, A::sub(s.ad_value(504), s.ad_value(1193)), 589, 1.0, 1195, 1.0);}
        s.b[1622] = (((((-s.v[604]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && s.b[1622]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(1220), 1.0));}
        s.b[1623] = (((-s.v[604]) / s.v[1220]) < 0.0);s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1622])) && s.b[1623]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 604, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) && (!s.b[1622])) && (!s.b[1623])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 604, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1620])) {s.store_mul_ad_product_lhs_mixed_ia(1219, 527, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195);}
        s.b[1624] = (s.v[536] > 1000.0);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1610])) && s.b[1624]) {s.store_scalar(1221, 1.0);}
        s.b[1625] = (s.v[1194] > ((-s.v[438]) * s.v[536]));s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });s.b[1626] = (s.v[539] == 4.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) && s.b[1626]) {s.store_mul_ad_product_lhs_mixed_ai(1195, A::mul3(A::square(A::mul(s.ad_value(1194), s.ad_value(610))), s.ad_value(1194), s.ad_value(610)), 1194, 610);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) && (!s.b[1626])) {s.store_pow_abs_mul_base_indices(1195, 1194, 610, 539);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && s.b[1625]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1610])) && (!s.b[1624])) && (!s.b[1625])) {s.store_add_scaled_product_mixed_iai(1221, 607, 1.0, A::add_scaled_inputs(s.ad_value(1194), 1.0, s.ad_value(536), s.v[438]), 613, 1.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1610])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(470, 667, 1222, 1.0, 668, 1223, 1.0, 669, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1627] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });s.b[1628] = (s.v[481] < s.v[675]);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });s.b[1629] = (((((-0.5) * (s.v[481] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && s.b[1629]) {s.store_primal_exp_scaled_input(1188, 481, (s.v[365] * (-0.5)));}
        s.b[1630] = (((-0.5) * (s.v[481] * s.v[365])) < 0.0);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && s.b[1630]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) && (!s.b[1629])) && (!s.b[1630])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(481), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1628]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && (!s.b[1628])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 676, A::sub_scaled_inputs(s.ad_value(481), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1627]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1631] = (s.v[481] > 0.0);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1627]) && s.b[1631]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
    }
}
