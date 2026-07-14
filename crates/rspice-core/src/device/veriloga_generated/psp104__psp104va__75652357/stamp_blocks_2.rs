#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1305] = (p.p825 == 0.5);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && s.b[1305]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && (!s.b[1305])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1306] = (p.p825 == 0.5);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && s.b[1306]) {s.store_sqrt_scaled_input(1195, 1198, s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) && (!s.b[1306])) {s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1304])) {s.store_scale(1202, 1195, s.v[418]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[379], ((-1.0)) * (s.v[379]));s.store_scaled_mul(1197, 1203, 1201, p.p834);}
        s.b[1307] = (p.p839 == 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1307]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1308] = (((-p.p825) * s.v[406]) == (-1.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1308]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1308])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[430]), 1206, 1209, s.v[430], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1309] = (s.v[1216] > 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1309]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1309])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1310] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1310]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1310])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1311] = (s.v[1216] > 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && s.b[1311]) {s.copy_ad(1217, 1179);}
        s.b[1312] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) && s.b[1312]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) && (!s.b[1312])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) && (!s.b[1311])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1307])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);}
        s.b[1313] = (p.p845 == 0.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1313]) {s.store_scalar(1219, 0.0);}
        s.b[1314] = (p.p825 == 0.5);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && s.b[1314]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1314])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 1195, 1.0);}
        s.b[1315] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && s.b[1315]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));}
        s.b[1316] = (((-s.v[436]) / s.v[1220]) < 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1315])) && s.b[1316]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) && (!s.b[1315])) && (!s.b[1316])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1313])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845, 0.0);}
        s.b[1317] = (p.p854 > 1000.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1303])) && s.b[1317]) {s.store_scalar(1221, 1.0);}
        s.b[1318] = (s.v[1194] > ((-s.v[438]) * p.p854));s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });s.b[1319] = (p.p857 == 4.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) && s.b[1319]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) && (!s.b[1319])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && s.b[1318]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1303])) && (!s.b[1317])) && (!s.b[1318])) {s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1303])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1320] = (s.v[642] == 0.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1320]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1320])) {s.store_primal_scale(1196, 1186, s.v[383]);}
        s.b[1321] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1321]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {s.store_primal_sub_from_scalar(1198, s.v[389], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1322] = (p.p826 == 0.5);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && s.b[1322]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && (!s.b[1322])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1323] = (p.p826 == 0.5);s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && s.b[1323]) {s.store_sqrt_scaled_input(1195, 1198, s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) && (!s.b[1323])) {s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1321])) {s.store_scale(1202, 1195, s.v[419]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(1197, 1203, 1201, p.p835);}
        s.b[1324] = (p.p840 == 0.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1324]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1325] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1325]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1325])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[431]), 1206, 1209, s.v[431], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1326] = (s.v[1216] > 0.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1326]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1326])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1327] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1327]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1327])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1328] = (s.v[1216] > 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && s.b[1328]) {s.copy_ad(1217, 1179);}
        s.b[1329] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) && s.b[1329]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) && (!s.b[1329])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1328])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1324])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);}
        s.b[1330] = (p.p846 == 0.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1330]) {s.store_scalar(1219, 0.0);}
        s.b[1331] = (p.p826 == 0.5);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && s.b[1331]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1331])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 1195, 1.0);}
        s.b[1332] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && s.b[1332]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));}
        s.b[1333] = (((-s.v[437]) / s.v[1220]) < 0.0);s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1332])) && s.b[1333]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) && (!s.b[1332])) && (!s.b[1333])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1330])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(480), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846, 0.0);}
        s.b[1334] = (p.p855 > 1000.0);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1320])) && s.b[1334]) {s.store_scalar(1221, 1.0);}
        s.b[1335] = (s.v[1194] > ((-s.v[438]) * p.p855));s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });s.b[1336] = (p.p858 == 4.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) && s.b[1336]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) && (!s.b[1336])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && s.b[1335]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1320])) && (!s.b[1334])) && (!s.b[1335])) {s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1320])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(470, 640, 1222, 1.0, 641, 1223, 1.0, 642, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1337] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });s.b[1338] = (s.v[481] < s.v[648]);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });s.b[1339] = (((((-0.5) * (s.v[481] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && s.b[1339]) {s.store_primal_exp_scaled_input(1188, 481, (s.v[365] * (-0.5)));}
        s.b[1340] = (((-0.5) * (s.v[481] * s.v[365])) < 0.0);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && (!s.b[1339])) && s.b[1340]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(481), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) && (!s.b[1339])) && (!s.b[1340])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(481), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(481), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1338]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && (!s.b[1338])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 649, A::sub_scaled_inputs(s.ad_value(481), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1337]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1341] = (s.v[481] > 0.0);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && s.b[1341]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1337]) && (!s.b[1341])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 481);}
        if ((s.b[1159] && s.b[1176]) && s.b[1337]) {s.store_primal_sub(1191, 650, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 481, 0.5, 1191, 0.5, 481, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 481, 0.5, 653, 0.5, 481, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 481, A::sqrt_square_offset(s.ad_value(481), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1342] = (s.v[640] == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1342]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1342])) {s.store_primal_scale(1196, 1186, s.v[381]);}
        s.b[1343] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1343]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {s.store_primal_sub_from_scalar(1198, s.v[387], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1344] = (p.p824 == 0.5);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && s.b[1344]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && (!s.b[1344])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1345] = (p.p824 == 0.5);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && s.b[1345]) {s.store_sqrt_scaled_input(1195, 1198, s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) && (!s.b[1345])) {s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1343])) {s.store_scale(1202, 1195, s.v[417]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[378], ((-1.0)) * (s.v[378]));s.store_scaled_mul(1197, 1203, 1201, p.p833);}
        s.b[1346] = (p.p838 == 0.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1346]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1347] = (((-p.p824) * s.v[405]) == (-1.0));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1347]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1347])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[429]), 1206, 1209, s.v[429], 1205, 1210, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1348] = (s.v[1216] > 0.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1348]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1348])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1349] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1349]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1349])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1350] = (s.v[1216] > 0.0);s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && s.b[1350]) {s.copy_ad(1217, 1179);}
        s.b[1351] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) && s.b[1351]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) && (!s.b[1351])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1350])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1346])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);}
        s.b[1352] = (p.p844 == 0.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1352]) {s.store_scalar(1219, 0.0);}
        s.b[1353] = (p.p824 == 0.5);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && s.b[1353]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1353])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 1195, 1.0);}
        s.b[1354] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && s.b[1354]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));}
        s.b[1355] = (((-s.v[435]) / s.v[1220]) < 0.0);s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1354])) && s.b[1355]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) && (!s.b[1354])) && (!s.b[1355])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1352])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844, 0.0);}
        s.b[1356] = (p.p853 > 1000.0);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1342])) && s.b[1356]) {s.store_scalar(1221, 1.0);}
        s.b[1357] = (s.v[1194] > ((-s.v[438]) * p.p853));s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });s.b[1358] = (p.p856 == 4.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) && s.b[1358]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) && (!s.b[1358])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && s.b[1357]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1342])) && (!s.b[1356])) && (!s.b[1357])) {s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1342])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1359] = (s.v[641] == 0.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1359]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1359])) {s.store_primal_scale(1196, 1186, s.v[382]);}
        s.b[1360] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1360]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {s.store_primal_sub_from_scalar(1198, s.v[388], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1361] = (p.p825 == 0.5);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && s.b[1361]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && (!s.b[1361])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1362] = (p.p825 == 0.5);s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && s.b[1362]) {s.store_sqrt_scaled_input(1195, 1198, s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) && (!s.b[1362])) {s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1360])) {s.store_scale(1202, 1195, s.v[418]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[379], ((-1.0)) * (s.v[379]));s.store_scaled_mul(1197, 1203, 1201, p.p834);}
        s.b[1363] = (p.p839 == 0.0);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1363]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1364] = (((-p.p825) * s.v[406]) == (-1.0));s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1364]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1364])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[430]), 1206, 1209, s.v[430], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1365] = (s.v[1216] > 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1365]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1365])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1366] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1366]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1366])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1367] = (s.v[1216] > 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && s.b[1367]) {s.copy_ad(1217, 1179);}
        s.b[1368] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) && s.b[1368]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) && (!s.b[1368])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1367])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1363])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);}
        s.b[1369] = (p.p845 == 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1369]) {s.store_scalar(1219, 0.0);}
        s.b[1370] = (p.p825 == 0.5);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && s.b[1370]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1370])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 1195, 1.0);}
        s.b[1371] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && s.b[1371]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));}
        s.b[1372] = (((-s.v[436]) / s.v[1220]) < 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1371])) && s.b[1372]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) && (!s.b[1371])) && (!s.b[1372])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1369])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845, 0.0);}
        s.b[1373] = (p.p854 > 1000.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1359])) && s.b[1373]) {s.store_scalar(1221, 1.0);}
        s.b[1374] = (s.v[1194] > ((-s.v[438]) * p.p854));s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });s.b[1375] = (p.p857 == 4.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) && s.b[1375]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) && (!s.b[1375])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && s.b[1374]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1359])) && (!s.b[1373])) && (!s.b[1374])) {s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1359])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1376] = (s.v[642] == 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1376]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1376])) {s.store_primal_scale(1196, 1186, s.v[383]);}
        s.b[1377] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1377]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {s.store_primal_sub_from_scalar(1198, s.v[389], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1378] = (p.p826 == 0.5);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && s.b[1378]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && (!s.b[1378])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1379] = (p.p826 == 0.5);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && s.b[1379]) {s.store_sqrt_scaled_input(1195, 1198, s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) && (!s.b[1379])) {s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1377])) {s.store_scale(1202, 1195, s.v[419]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(1197, 1203, 1201, p.p835);}
        s.b[1380] = (p.p840 == 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1380]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1381] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1381]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1381])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[431]), 1206, 1209, s.v[431], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1382] = (s.v[1216] > 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1382]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1382])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1383] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1383]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1383])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1384] = (s.v[1216] > 0.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && s.b[1384]) {s.copy_ad(1217, 1179);}
        s.b[1385] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) && s.b[1385]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) && (!s.b[1385])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1384])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1380])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);}
        s.b[1386] = (p.p846 == 0.0);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1386]) {s.store_scalar(1219, 0.0);}
        s.b[1387] = (p.p826 == 0.5);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && s.b[1387]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1387])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 1195, 1.0);}
        s.b[1388] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && s.b[1388]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));}
        s.b[1389] = (((-s.v[437]) / s.v[1220]) < 0.0);s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1388])) && s.b[1389]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) && (!s.b[1388])) && (!s.b[1389])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1386])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(481), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846, 0.0);}
        s.b[1390] = (p.p855 > 1000.0);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1376])) && s.b[1390]) {s.store_scalar(1221, 1.0);}
        s.b[1391] = (s.v[1194] > ((-s.v[438]) * p.p855));s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });s.b[1392] = (p.p858 == 4.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) && s.b[1392]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) && (!s.b[1392])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && s.b[1391]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1376])) && (!s.b[1390])) && (!s.b[1391])) {s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1376])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(471, 640, 1222, 1.0, 641, 1223, 1.0, 642, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1393] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });s.b[1394] = (s.v[482] < s.v[648]);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });s.b[1395] = (((((-0.5) * (s.v[482] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && s.b[1395]) {s.store_primal_exp_scaled_input(1188, 482, (s.v[365] * (-0.5)));}
        s.b[1396] = (((-0.5) * (s.v[482] * s.v[365])) < 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && (!s.b[1395])) && s.b[1396]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(482), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) && (!s.b[1395])) && (!s.b[1396])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(482), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(482), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1394]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && (!s.b[1394])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 649, A::sub_scaled_inputs(s.ad_value(482), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1393]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1397] = (s.v[482] > 0.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && s.b[1397]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1393]) && (!s.b[1397])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 482);}
        if ((s.b[1159] && s.b[1176]) && s.b[1393]) {s.store_primal_sub(1191, 650, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 482, 0.5, 1191, 0.5, 482, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 482, 0.5, 653, 0.5, 482, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 482, A::sqrt_square_offset(s.ad_value(482), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1398] = (s.v[640] == 0.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1398]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1398])) {s.store_primal_scale(1196, 1186, s.v[381]);}
        s.b[1399] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1399]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {s.store_primal_sub_from_scalar(1198, s.v[387], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1400] = (p.p824 == 0.5);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && s.b[1400]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && (!s.b[1400])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1401] = (p.p824 == 0.5);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && s.b[1401]) {s.store_sqrt_scaled_input(1195, 1198, s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) && (!s.b[1401])) {s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1399])) {s.store_scale(1202, 1195, s.v[417]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[378], ((-1.0)) * (s.v[378]));s.store_scaled_mul(1197, 1203, 1201, p.p833);}
        s.b[1402] = (p.p838 == 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1402]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1403] = (((-p.p824) * s.v[405]) == (-1.0));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1403]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1403])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[429]), 1206, 1209, s.v[429], 1205, 1210, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1404] = (s.v[1216] > 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1404]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1404])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1405] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1405]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1405])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1406] = (s.v[1216] > 0.0);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && s.b[1406]) {s.copy_ad(1217, 1179);}
        s.b[1407] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) && s.b[1407]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) && (!s.b[1407])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1406])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1402])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);}
        s.b[1408] = (p.p844 == 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1408]) {s.store_scalar(1219, 0.0);}
        s.b[1409] = (p.p824 == 0.5);s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && s.b[1409]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1409])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 1195, 1.0);}
        s.b[1410] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && s.b[1410]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));}
        s.b[1411] = (((-s.v[435]) / s.v[1220]) < 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1410])) && s.b[1411]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) && (!s.b[1410])) && (!s.b[1411])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1408])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844, 0.0);}
        s.b[1412] = (p.p853 > 1000.0);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1398])) && s.b[1412]) {s.store_scalar(1221, 1.0);}
        s.b[1413] = (s.v[1194] > ((-s.v[438]) * p.p853));s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });s.b[1414] = (p.p856 == 4.0);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) && s.b[1414]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) && (!s.b[1414])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && s.b[1413]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1398])) && (!s.b[1412])) && (!s.b[1413])) {s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1398])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1415] = (s.v[641] == 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1415]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1415])) {s.store_primal_scale(1196, 1186, s.v[382]);}
        s.b[1416] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1416]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {s.store_primal_sub_from_scalar(1198, s.v[388], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1417] = (p.p825 == 0.5);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && s.b[1417]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && (!s.b[1417])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1418] = (p.p825 == 0.5);s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && s.b[1418]) {s.store_sqrt_scaled_input(1195, 1198, s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) && (!s.b[1418])) {s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1416])) {s.store_scale(1202, 1195, s.v[418]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[379], ((-1.0)) * (s.v[379]));s.store_scaled_mul(1197, 1203, 1201, p.p834);}
        s.b[1419] = (p.p839 == 0.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1419]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1420] = (((-p.p825) * s.v[406]) == (-1.0));s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1420]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1420])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[430]), 1206, 1209, s.v[430], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1421] = (s.v[1216] > 0.0);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1421]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1421])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1422] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1422]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1422])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1423] = (s.v[1216] > 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && s.b[1423]) {s.copy_ad(1217, 1179);}
        s.b[1424] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) && s.b[1424]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) && (!s.b[1424])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1423])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1419])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);}
        s.b[1425] = (p.p845 == 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1425]) {s.store_scalar(1219, 0.0);}
        s.b[1426] = (p.p825 == 0.5);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && s.b[1426]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1426])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 1195, 1.0);}
        s.b[1427] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && s.b[1427]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));}
        s.b[1428] = (((-s.v[436]) / s.v[1220]) < 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1427])) && s.b[1428]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) && (!s.b[1427])) && (!s.b[1428])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1425])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p845, 0.0);}
        s.b[1429] = (p.p854 > 1000.0);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1415])) && s.b[1429]) {s.store_scalar(1221, 1.0);}
        s.b[1430] = (s.v[1194] > ((-s.v[438]) * p.p854));s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });s.b[1431] = (p.p857 == 4.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) && s.b[1431]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[443] * s.v[443]) * s.v[443])), 1194, s.v[443], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) && (!s.b[1431])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[443]), p.p857);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && s.b[1430]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1415])) && (!s.b[1429])) && (!s.b[1430])) {s.store_offset_scaled(1221, 1194, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1415])) {s.store_mul_scale_offset_mixed_ia(1223, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1432] = (s.v[642] == 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1432]) {s.store_scalar(1224, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1432])) {s.store_primal_scale(1196, 1186, s.v[383]);}
        s.b[1433] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1433]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {s.store_primal_sub_from_scalar(1198, s.v[389], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1434] = (p.p826 == 0.5);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && s.b[1434]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && (!s.b[1434])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p826)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1435] = (p.p826 == 0.5);s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && s.b[1435]) {s.store_sqrt_scaled_input(1195, 1198, s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) && (!s.b[1435])) {s.store_powf_scaled_input(1195, 1198, s.v[425], p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1433])) {s.store_scale(1202, 1195, s.v[419]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(1197, 1203, 1201, p.p835);}
        s.b[1436] = (p.p840 == 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1436]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[404] * s.v[434]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[431]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1437] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1437]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1437])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[431]), 1206, 1209, s.v[431], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1438] = (s.v[1216] > 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1438]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1438])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1439] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1439]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1439])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1440] = (s.v[1216] > 0.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && s.b[1440]) {s.copy_ad(1217, 1179);}
        s.b[1441] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) && s.b[1441]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) && (!s.b[1441])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1440])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1436])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[431] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p840, 0.0, 1212);}
        s.b[1442] = (p.p846 == 0.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1442]) {s.store_scalar(1219, 0.0);}
        s.b[1443] = (p.p826 == 0.5);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && s.b[1443]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p823, s.ad_value(1193)), s.v[425]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1443])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 1195, 1.0);}
        s.b[1444] = (((((-s.v[437]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && s.b[1444]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(1220), 1.0));}
        s.b[1445] = (((-s.v[437]) / s.v[1220]) < 0.0);s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1444])) && s.b[1445]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 437, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) && (!s.b[1444])) && (!s.b[1445])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 437, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1442])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(482), s.ad_value(1220), s.ad_value(1220)), 1195, p.p846, 0.0);}
        s.b[1446] = (p.p855 > 1000.0);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1432])) && s.b[1446]) {s.store_scalar(1221, 1.0);}
        s.b[1447] = (s.v[1194] > ((-s.v[438]) * p.p855));s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });s.b[1448] = (p.p858 == 4.0);s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) && s.b[1448]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[444] * s.v[444]) * s.v[444])), 1194, s.v[444], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) && (!s.b[1448])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[444]), p.p858);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && s.b[1447]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1432])) && (!s.b[1446])) && (!s.b[1447])) {s.store_offset_scaled(1221, 1194, s.v[447], (((((s.v[438] * p.p855)) * (s.v[447]))) + (s.v[441])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1432])) {s.store_mul_scale_offset_mixed_ia(1224, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        if (s.b[1159] && s.b[1176]) {s.store_add_scaled_products3_indices(472, 640, 1222, 1.0, 641, 1223, 1.0, 642, 1224, 1.0);s.store_scalar(1193, 0.0);s.store_scalar(1190, 0.0);}
        s.b[1449] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });s.b[1450] = (s.v[483] < s.v[648]);s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });s.b[1451] = (((((-0.5) * (s.v[483] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && s.b[1451]) {s.store_primal_exp_scaled_input(1188, 483, (s.v[365] * (-0.5)));}
        s.b[1452] = (((-0.5) * (s.v[483] * s.v[365])) < 0.0);s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && (!s.b[1451])) && s.b[1452]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1188, 1e-100, (-230.25850929940458), A::scale(s.ad_value(483), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) && (!s.b[1451])) && (!s.b[1452])) {s.store_primal_scaled_offset_ad(1188, A::mul_offset_rhs(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(483), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(483), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1450]) {s.store_primal_div_from_scalar(1189, 1.0, 1188);s.store_primal_square(1186, 1189);}
        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && (!s.b[1450])) {s.store_primal_mul_scale_offset_mixed_ia(1186, 649, A::sub_scaled_inputs(s.ad_value(483), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_primal_sqrt(1189, 1186);s.store_primal_div_from_scalar(1188, 1.0, 1189);}
        if ((s.b[1159] && s.b[1176]) && s.b[1449]) {s.store_primal_offset(1186, 1186, (-1.0));}
        s.b[1453] = (s.v[483] > 0.0);s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && s.b[1453]) {s.store_primal_scaled_ln_ad(1190, A::add(A::offset(s.ad_value(1188), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1188), 1.0, A::offset(s.ad_value(1188), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[1159] && s.b[1176]) && s.b[1449]) && (!s.b[1453])) {s.store_primal_sub_mixed_ai(1190, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1189), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1189), 1.0, A::scale_offset(s.ad_value(1189), 3.0, 1.0))))), (s.v[364] * 2.0)), 483);}
        if ((s.b[1159] && s.b[1176]) && s.b[1449]) {s.store_primal_sub(1191, 650, 1190);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1192, 483, 0.5, 1191, 0.5, 483, 1191, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1193, 483, 0.5, 653, 0.5, 483, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1194, 483, A::sqrt_square_offset(s.ad_value(483), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1454] = (s.v[640] == 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1454]) {s.store_scalar(1222, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1454])) {s.store_primal_scale(1196, 1186, s.v[381]);}
        s.b[1455] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1455]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {s.store_primal_sub_from_scalar(1198, s.v[387], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
        s.b[1456] = (p.p824 == 0.5);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && s.b[1456]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && (!s.b[1456])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p824)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1457] = (p.p824 == 0.5);s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && s.b[1457]) {s.store_sqrt_scaled_input(1195, 1198, s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) && (!s.b[1457])) {s.store_powf_scaled_input(1195, 1198, s.v[423], p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1455])) {s.store_scale(1202, 1195, s.v[417]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[378], ((-1.0)) * (s.v[378]));s.store_scaled_mul(1197, 1203, 1201, p.p833);}
        s.b[1458] = (p.p838 == 0.0);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1458]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[402] * s.v[432]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[429]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1459] = (((-p.p824) * s.v[405]) == (-1.0));s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1459]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1459])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p824) * s.v[405]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[429]), 1206, 1209, s.v[429], 1205, 1210, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1460] = (s.v[1216] > 0.0);s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1460]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1460])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1461] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1461]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1461])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1462] = (s.v[1216] > 0.0);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && s.b[1462]) {s.copy_ad(1217, 1179);}
        s.b[1463] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) && s.b[1463]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) && (!s.b[1463])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1462])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1458])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[429] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p838, 0.0, 1212);}
        s.b[1464] = (p.p844 == 0.0);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1464]) {s.store_scalar(1219, 0.0);}
        s.b[1465] = (p.p824 == 0.5);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && s.b[1465]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p821, s.ad_value(1193)), s.v[423]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1465])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 1195, 1.0);}
        s.b[1466] = (((((-s.v[435]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && s.b[1466]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(1220), 1.0));}
        s.b[1467] = (((-s.v[435]) / s.v[1220]) < 0.0);s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1466])) && s.b[1467]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 435, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) && (!s.b[1466])) && (!s.b[1467])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 435, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1464])) {s.store_mul_scale_offset_mixed_ai(1219, A::mul3(s.ad_value(483), s.ad_value(1220), s.ad_value(1220)), 1195, p.p844, 0.0);}
        s.b[1468] = (p.p853 > 1000.0);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1454])) && s.b[1468]) {s.store_scalar(1221, 1.0);}
        s.b[1469] = (s.v[1194] > ((-s.v[438]) * p.p853));s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });s.b[1470] = (p.p856 == 4.0);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) && s.b[1470]) {s.store_mul_scale_offset_mixed_ai(1195, A::mul3_scaled_output(s.ad_value(1194), s.ad_value(1194), s.ad_value(1194), ((s.v[442] * s.v[442]) * s.v[442])), 1194, s.v[442], 0.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) && (!s.b[1470])) {s.store_powf_ad(1195, A::abs_scaled_input(s.ad_value(1194), s.v[442]), p.p856);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && s.b[1469]) {s.store_div_from_scalar_sub_from_scalar_ad(1221, 1.0, 1.0, s.ad_value(1195));}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1454])) && (!s.b[1468])) && (!s.b[1469])) {s.store_offset_scaled(1221, 1194, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1454])) {s.store_mul_scale_offset_mixed_ia(1222, 1221, A::add_scaled_inputs4(s.ad_value(1196), 1.0, s.ad_value(1197), 1.0, s.ad_value(1204), 1.0, s.ad_value(1219), 1.0), p.p29, 0.0);}
        s.b[1471] = (s.v[641] == 0.0);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1176]) && s.b[1471]) {s.store_scalar(1223, 0.0);}
        if ((s.b[1159] && s.b[1176]) && (!s.b[1471])) {s.store_primal_scale(1196, 1186, s.v[382]);}
        s.b[1472] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1472]) {s.store_scalar(1197, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {s.store_primal_sub_from_scalar(1198, s.v[388], 1192);s.store_primal_sub_from_scalar_ad(1199, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1190), s.ad_value(1198)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1473] = (p.p825 == 0.5);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {s.store_scalar(1200, 0.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {s.store_primal_scaled_add_mixed_ai(1200, A::div_scaled_product(A::square(s.ad_value(1199)), A::ln(s.ad_value(1199)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1199)), 1.0), 1199, (1.0 - (2.0 * p.p825)));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {s.store_primal_add(1201, 1199, 1200);}
        s.b[1474] = (p.p825 == 0.5);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && s.b[1474]) {s.store_sqrt_scaled_input(1195, 1198, s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1474])) {s.store_powf_scaled_input(1195, 1198, s.v[424], p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1472])) {s.store_scale(1202, 1195, s.v[418]);s.store_mul_scale_offset_indices(1203, 1202, 1189, s.v[379], ((-1.0)) * (s.v[379]));s.store_scaled_mul(1197, 1203, 1201, p.p834);}
        s.b[1475] = (p.p839 == 0.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1475]) {s.store_scalar(1204, 0.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {s.store_div_scaled_inputs_indices(1205, 1202, (s.v[403] * s.v[433]), 1198, 1.0);s.store_div_from_scalar(1206, (0.666666666666667 * s.v[430]), 1205);s.store_square(1207, 1206);s.store_sqrt_div_scaled_square_offset_denominator(1208, 1207, 1.0, 1.0, 1.0);s.store_sqrt(1209, 1208);s.store_mul(1210, 1208, 1209);}
        s.b[1476] = (((-p.p825) * s.v[406]) == (-1.0));s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1476]) {s.store_div_from_scalar_offset_product(1211, 1.0, 1205, 1210, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1476])) {s.store_powf_ad(1211, A::offset(A::mul(s.ad_value(1205), s.ad_value(1210)), 1.0), ((-p.p825) * s.v[406]));}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {s.store_div_scaled_product_add_scaled_denominator_indices(1212, 1201, 1211, 1.0, 1201, 1.0, 1211, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1213, A::div(s.ad_value(1205), s.ad_value(1209)), 0.375);s.store_add_scaled_product_indices(1214, 1208, (-1.0), 1206, 1209, 2.0);s.store_add_scaled_value_products_indices(1215, 1208, (-s.v[430]), 1206, 1209, s.v[430], 1205, 1210, 0.5);s.store_mul_scale_offset_indices(1216, 1213, 1214, 1.0, (-1.0));s.store_square(1177, 1216);}
        s.b[1477] = (s.v[1216] > 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1477]) {s.store_div_from_scalar_offset_scaled_input(1178, 1.0, 1216, s.v[366], 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1477])) {s.store_div_from_scalar_sub_from_scalar_ad(1178, 1.0, 1.0, A::scale(s.ad_value(1216), s.v[366]));}
        s.b[1478] = (((-s.v[1177]) + s.v[1215]) > (-230.25850929940458));s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1478]) {s.store_exp_sub(1195, 1215, 1177);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1478])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1195, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1215), s.ad_value(1177)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {s.store_mul_mixed_ai(1179, A::add_scaled_inputs_product(s.ad_value(1178), 0.29214664, A::square(s.ad_value(1178)), s.v[367], A::square(s.ad_value(1178)), s.ad_value(1178), s.v[368]), 1195);}
        s.b[1479] = (s.v[1216] > 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && s.b[1479]) {s.copy_ad(1217, 1179);}
        s.b[1480] = (s.v[1215] > (-230.25850929940458));s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) && s.b[1480]) {s.store_exp(1195, 1215);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) && (!s.b[1480])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 1215, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1479])) {s.store_sub_scaled_inputs(1217, 1195, 2.0, 1179, 1.0);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1475])) {s.store_div_scaled_inputs_indices(1218, 1217, (s.v[430] * (1.772453850905516 * 0.5)), 1213, 1.0);s.store_mul3_affine_lhs(1204, 1203, 1218, p.p839, 0.0, 1212);}
        s.b[1481] = (p.p845 == 0.0);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && s.b[1481]) {s.store_scalar(1219, 0.0);}
        s.b[1482] = (p.p825 == 0.5);s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && s.b[1482]) {s.store_sqrt_scaled_input_ad(1195, A::sub_from_scalar(p.p822, s.ad_value(1193)), s.v[424]);}
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1482])) {s.store_powf_scale_offset_input(1195, 1193, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) {s.store_div_scaled_offset_numerator_indices(1220, 1193, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 1195, 1.0);}
        s.b[1483] = (((((-s.v[436]) / s.v[1220])) as f64).abs() < 230.25850929940458);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && s.b[1483]) {s.store_ad_value(1195, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(1220), 1.0));}
        s.b[1484] = (((-s.v[436]) / s.v[1220]) < 0.0);s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1483])) && s.b[1484]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1195, 1e-100, (-230.25850929940458), 436, -1.0, 1220, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1159] && s.b[1176]) && (!s.b[1471])) && (!s.b[1481])) && (!s.b[1483])) && (!s.b[1484])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1195, 436, -1.0, 1220, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
}
