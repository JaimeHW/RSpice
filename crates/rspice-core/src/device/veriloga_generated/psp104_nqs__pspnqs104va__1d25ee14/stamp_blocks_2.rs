#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851, 0.0);}
        s.b[1312] = (p.p860 > 1000.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1312]) {s.store_scalar(1233, 1.0);}
        s.b[1313] = (s.v[1206] > ((-s.v[444]) * p.p860));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });s.b[1314] = (p.p863 == 4.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && s.b[1314]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && (!s.b[1314])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && (!s.b[1313])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1315] = (s.v[647] == 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1315]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1316] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1316]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1317] = (p.p832 == 0.5);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1318] = (p.p832 == 0.5);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1318]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1318])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p.p841);}
        s.b[1319] = (p.p846 == 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1319]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1320] = (((-p.p832) * s.v[412]) == (-1.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1320]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1320])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1321] = (s.v[1228] > 0.0);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1321]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1321])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1322] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1322]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1322])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1323] = (s.v[1228] > 0.0);s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1323]) {s.copy_ad(1229, 1191);}
        s.b[1324] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && s.b[1324]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && (!s.b[1324])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);}
        s.b[1325] = (p.p852 == 0.0);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1325]) {s.store_scalar(1231, 0.0);}
        s.b[1326] = (p.p832 == 0.5);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1326]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1326])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1327] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1327]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1328] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && s.b[1328]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && (!s.b[1328])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852, 0.0);}
        s.b[1329] = (p.p861 > 1000.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1329]) {s.store_scalar(1233, 1.0);}
        s.b[1330] = (s.v[1206] > ((-s.v[444]) * p.p861));s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });s.b[1331] = (p.p864 == 4.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && s.b[1331]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && (!s.b[1331])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && (!s.b[1330])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1332] = (s.v[648] == 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1332]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1333] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1333]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1334] = (p.p833 == 0.5);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1334]) {s.store_scalar(1212, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1334])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1335] = (p.p833 == 0.5);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1335]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1335])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p.p842);}
        s.b[1336] = (p.p847 == 0.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1336]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1337] = (((-p.p833) * s.v[413]) == (-1.0));s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1337]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1337])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1338] = (s.v[1228] > 0.0);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1338]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1338])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1339] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1339]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1339])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1340] = (s.v[1228] > 0.0);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1340]) {s.copy_ad(1229, 1191);}
        s.b[1341] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) && s.b[1341]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) && (!s.b[1341])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1340])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);}
        s.b[1342] = (p.p853 == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1342]) {s.store_scalar(1231, 0.0);}
        s.b[1343] = (p.p833 == 0.5);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1343]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1343])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1344] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1344]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1345] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && s.b[1345]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && (!s.b[1345])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853, 0.0);}
        s.b[1346] = (p.p862 > 1000.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1346]) {s.store_scalar(1233, 1.0);}
        s.b[1347] = (s.v[1206] > ((-s.v[444]) * p.p862));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });s.b[1348] = (p.p865 == 4.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && s.b[1348]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && (!s.b[1348])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && (!s.b[1347])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(476, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1349] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });s.b[1350] = (s.v[487] < s.v[654]);s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });s.b[1351] = (((((-0.5) * (s.v[487] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && s.b[1351]) {s.store_primal_exp_scaled_input(1200, 487, (s.v[371] * (-0.5)));}
        s.b[1352] = (((-0.5) * (s.v[487] * s.v[371])) < 0.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && (!s.b[1351])) && s.b[1352]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) && (!s.b[1351])) && (!s.b[1352])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1350]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && (!s.b[1350])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(487), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1349]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1353] = (s.v[487] > 0.0);s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && s.b[1353]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && (!s.b[1353])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 487);}
        if ((s.b[1171] && s.b[1188]) && s.b[1349]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 487, 0.5, 1203, 0.5, 487, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 487, 0.5, 659, 0.5, 487, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 487, A::sqrt_square_offset(s.ad_value(487), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1354] = (s.v[646] == 0.0);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1354]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1354])) {s.store_primal_scale(1208, 1198, s.v[387]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1355] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1355]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1356] = (p.p831 == 0.5);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1356]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1356])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1357] = (p.p831 == 0.5);s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1357]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1357])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p.p840);}
        s.b[1358] = (p.p845 == 0.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1358]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1359] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1359]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1359])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1360] = (s.v[1228] > 0.0);s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1360]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1360])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1361] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1361]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1361])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1362] = (s.v[1228] > 0.0);s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && s.b[1362]) {s.copy_ad(1229, 1191);}
        s.b[1363] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) && s.b[1363]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) && (!s.b[1363])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) && (!s.b[1362])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);}
        s.b[1364] = (p.p851 == 0.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1364]) {s.store_scalar(1231, 0.0);}
        s.b[1365] = (p.p831 == 0.5);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && s.b[1365]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1365])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 1207, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1366] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && s.b[1366]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1367] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1366])) && s.b[1367]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) && (!s.b[1366])) && (!s.b[1367])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1364])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851, 0.0);}
        s.b[1368] = (p.p860 > 1000.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1368]) {s.store_scalar(1233, 1.0);}
        s.b[1369] = (s.v[1206] > ((-s.v[444]) * p.p860));s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });s.b[1370] = (p.p863 == 4.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) && s.b[1370]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) && (!s.b[1370])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && s.b[1369]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1368])) && (!s.b[1369])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1354])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1371] = (s.v[647] == 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1371]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1371])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1372] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1372]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1373] = (p.p832 == 0.5);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && s.b[1373]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && (!s.b[1373])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1374] = (p.p832 == 0.5);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && s.b[1374]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) && (!s.b[1374])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1372])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p.p841);}
        s.b[1375] = (p.p846 == 0.0);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1375]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1376] = (((-p.p832) * s.v[412]) == (-1.0));s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1376]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1376])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1377] = (s.v[1228] > 0.0);s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1377]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1377])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1378] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1378]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1378])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1379] = (s.v[1228] > 0.0);s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && s.b[1379]) {s.copy_ad(1229, 1191);}
        s.b[1380] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) && s.b[1380]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) && (!s.b[1380])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) && (!s.b[1379])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1375])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);}
        s.b[1381] = (p.p852 == 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1381]) {s.store_scalar(1231, 0.0);}
        s.b[1382] = (p.p832 == 0.5);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && s.b[1382]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1382])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1383] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && s.b[1383]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1384] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1383])) && s.b[1384]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) && (!s.b[1383])) && (!s.b[1384])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1381])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852, 0.0);}
        s.b[1385] = (p.p861 > 1000.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1371])) && s.b[1385]) {s.store_scalar(1233, 1.0);}
        s.b[1386] = (s.v[1206] > ((-s.v[444]) * p.p861));s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });s.b[1387] = (p.p864 == 4.0);s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) && s.b[1387]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) && (!s.b[1387])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && s.b[1386]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1371])) && (!s.b[1385])) && (!s.b[1386])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1371])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1388] = (s.v[648] == 0.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1388]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1388])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1389] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1389]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1390] = (p.p833 == 0.5);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && s.b[1390]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && (!s.b[1390])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1391] = (p.p833 == 0.5);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && s.b[1391]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) && (!s.b[1391])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1389])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p.p842);}
        s.b[1392] = (p.p847 == 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1392]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1393] = (((-p.p833) * s.v[413]) == (-1.0));s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1393]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1393])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1394] = (s.v[1228] > 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1394]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1394])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1395] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1395]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1395])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1396] = (s.v[1228] > 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && s.b[1396]) {s.copy_ad(1229, 1191);}
        s.b[1397] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) && s.b[1397]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) && (!s.b[1397])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) && (!s.b[1396])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1392])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);}
        s.b[1398] = (p.p853 == 0.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1398]) {s.store_scalar(1231, 0.0);}
        s.b[1399] = (p.p833 == 0.5);s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && s.b[1399]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1399])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1400] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && s.b[1400]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1401] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1400])) && s.b[1401]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) && (!s.b[1400])) && (!s.b[1401])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1398])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(487), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853, 0.0);}
        s.b[1402] = (p.p862 > 1000.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1388])) && s.b[1402]) {s.store_scalar(1233, 1.0);}
        s.b[1403] = (s.v[1206] > ((-s.v[444]) * p.p862));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });s.b[1404] = (p.p865 == 4.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) && s.b[1404]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) && (!s.b[1404])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && s.b[1403]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1388])) && (!s.b[1402])) && (!s.b[1403])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1388])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(477, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1405] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });s.b[1406] = (s.v[488] < s.v[654]);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });s.b[1407] = (((((-0.5) * (s.v[488] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && s.b[1407]) {s.store_primal_exp_scaled_input(1200, 488, (s.v[371] * (-0.5)));}
        s.b[1408] = (((-0.5) * (s.v[488] * s.v[371])) < 0.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && (!s.b[1407])) && s.b[1408]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(488), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) && (!s.b[1407])) && (!s.b[1408])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1406]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && (!s.b[1406])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(488), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1405]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1409] = (s.v[488] > 0.0);s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && s.b[1409]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1405]) && (!s.b[1409])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 488);}
        if ((s.b[1171] && s.b[1188]) && s.b[1405]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 488, 0.5, 1203, 0.5, 488, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 488, 0.5, 659, 0.5, 488, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 488, A::sqrt_square_offset(s.ad_value(488), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1410] = (s.v[646] == 0.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1410]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1410])) {s.store_primal_scale(1208, 1198, s.v[387]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1411] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1411]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1412] = (p.p831 == 0.5);s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && s.b[1412]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && (!s.b[1412])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1413] = (p.p831 == 0.5);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && s.b[1413]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) && (!s.b[1413])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1411])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p.p840);}
        s.b[1414] = (p.p845 == 0.0);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1414]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1415] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1415]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1415])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1416] = (s.v[1228] > 0.0);s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1416]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1416])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1417] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1417]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1417])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1418] = (s.v[1228] > 0.0);s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && s.b[1418]) {s.copy_ad(1229, 1191);}
        s.b[1419] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1418])) && s.b[1419]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1418])) && (!s.b[1419])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) && (!s.b[1418])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1414])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);}
        s.b[1420] = (p.p851 == 0.0);s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1420]) {s.store_scalar(1231, 0.0);}
        s.b[1421] = (p.p831 == 0.5);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) && s.b[1421]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) && (!s.b[1421])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 1207, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1422] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) && s.b[1422]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1423] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) && (!s.b[1422])) && s.b[1423]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) && (!s.b[1422])) && (!s.b[1423])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1420])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851, 0.0);}
        s.b[1424] = (p.p860 > 1000.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1410])) && s.b[1424]) {s.store_scalar(1233, 1.0);}
        s.b[1425] = (s.v[1206] > ((-s.v[444]) * p.p860));s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });s.b[1426] = (p.p863 == 4.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1424])) && s.b[1425]) && s.b[1426]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1424])) && s.b[1425]) && (!s.b[1426])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1424])) && s.b[1425]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1410])) && (!s.b[1424])) && (!s.b[1425])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1410])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1427] = (s.v[647] == 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1427]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1427])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1428] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1428]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1429] = (p.p832 == 0.5);s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && s.b[1429]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && (!s.b[1429])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1430] = (p.p832 == 0.5);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && s.b[1430]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) && (!s.b[1430])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1428])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p.p841);}
        s.b[1431] = (p.p846 == 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1431]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1432] = (((-p.p832) * s.v[412]) == (-1.0));s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1432]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1432])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1433] = (s.v[1228] > 0.0);s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1433]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1433])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1434] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1434]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1434])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1435] = (s.v[1228] > 0.0);s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && s.b[1435]) {s.copy_ad(1229, 1191);}
        s.b[1436] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) && s.b[1436]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) && (!s.b[1436])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) && (!s.b[1435])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1431])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p846, 0.0, 1224);}
        s.b[1437] = (p.p852 == 0.0);s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1437]) {s.store_scalar(1231, 0.0);}
        s.b[1438] = (p.p832 == 0.5);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && s.b[1438]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p829, s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1438])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1439] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && s.b[1439]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1440] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1439])) && s.b[1440]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) && (!s.b[1439])) && (!s.b[1440])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1437])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207, p.p852, 0.0);}
        s.b[1441] = (p.p861 > 1000.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1427])) && s.b[1441]) {s.store_scalar(1233, 1.0);}
        s.b[1442] = (s.v[1206] > ((-s.v[444]) * p.p861));s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });s.b[1443] = (p.p864 == 4.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) && s.b[1443]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) && (!s.b[1443])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p.p864);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && s.b[1442]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1427])) && (!s.b[1441])) && (!s.b[1442])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1427])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1444] = (s.v[648] == 0.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1444]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1444])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1445] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1445]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1446] = (p.p833 == 0.5);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && s.b[1446]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && (!s.b[1446])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p833)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1447] = (p.p833 == 0.5);s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && s.b[1447]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) && (!s.b[1447])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1445])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p.p842);}
        s.b[1448] = (p.p847 == 0.0);s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1448]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1449] = (((-p.p833) * s.v[413]) == (-1.0));s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1449]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1449])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p833) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1450] = (s.v[1228] > 0.0);s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1450]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1450])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1451] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1451]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1451])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1452] = (s.v[1228] > 0.0);s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && s.b[1452]) {s.copy_ad(1229, 1191);}
        s.b[1453] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) && s.b[1453]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) && (!s.b[1453])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) && (!s.b[1452])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1448])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p847, 0.0, 1224);}
        s.b[1454] = (p.p853 == 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1454]) {s.store_scalar(1231, 0.0);}
        s.b[1455] = (p.p833 == 0.5);s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && s.b[1455]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p830, s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1455])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1456] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && s.b[1456]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1457] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1456])) && s.b[1457]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) && (!s.b[1456])) && (!s.b[1457])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1454])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(488), s.ad_value(1232), s.ad_value(1232)), 1207, p.p853, 0.0);}
        s.b[1458] = (p.p862 > 1000.0);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1444])) && s.b[1458]) {s.store_scalar(1233, 1.0);}
        s.b[1459] = (s.v[1206] > ((-s.v[444]) * p.p862));s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });s.b[1460] = (p.p865 == 4.0);s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) && s.b[1460]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) && (!s.b[1460])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p.p865);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && s.b[1459]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1444])) && (!s.b[1458])) && (!s.b[1459])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p.p862)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1444])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(478, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1461] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });s.b[1462] = (s.v[489] < s.v[654]);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });s.b[1463] = (((((-0.5) * (s.v[489] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && s.b[1463]) {s.store_primal_exp_scaled_input(1200, 489, (s.v[371] * (-0.5)));}
        s.b[1464] = (((-0.5) * (s.v[489] * s.v[371])) < 0.0);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && (!s.b[1463])) && s.b[1464]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(489), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) && (!s.b[1463])) && (!s.b[1464])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1462]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && (!s.b[1462])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(489), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1461]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1465] = (s.v[489] > 0.0);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && s.b[1465]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1461]) && (!s.b[1465])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 489);}
        if ((s.b[1171] && s.b[1188]) && s.b[1461]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 489, 0.5, 1203, 0.5, 489, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 489, 0.5, 659, 0.5, 489, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 489, A::sqrt_square_offset(s.ad_value(489), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1466] = (s.v[646] == 0.0);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1466]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1466])) {s.store_primal_scale(1208, 1198, s.v[387]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1467] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1467]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1468] = (p.p831 == 0.5);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && s.b[1468]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && (!s.b[1468])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p831)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1469] = (p.p831 == 0.5);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && s.b[1469]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) && (!s.b[1469])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1467])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p.p840);}
        s.b[1470] = (p.p845 == 0.0);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1470]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1471] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1471]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1471])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1472] = (s.v[1228] > 0.0);s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1472]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1472])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1473] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1473]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1473])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1474] = (s.v[1228] > 0.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && s.b[1474]) {s.copy_ad(1229, 1191);}
        s.b[1475] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) && s.b[1475]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) && (!s.b[1475])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) && (!s.b[1474])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1470])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p.p845, 0.0, 1224);}
        s.b[1476] = (p.p851 == 0.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1476]) {s.store_scalar(1231, 0.0);}
        s.b[1477] = (p.p831 == 0.5);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && s.b[1477]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p.p828, s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1477])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 1207, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1478] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && s.b[1478]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1479] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1478])) && s.b[1479]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) && (!s.b[1478])) && (!s.b[1479])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1476])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(489), s.ad_value(1232), s.ad_value(1232)), 1207, p.p851, 0.0);}
        s.b[1480] = (p.p860 > 1000.0);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1466])) && s.b[1480]) {s.store_scalar(1233, 1.0);}
        s.b[1481] = (s.v[1206] > ((-s.v[444]) * p.p860));s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });s.b[1482] = (p.p863 == 4.0);s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) && s.b[1482]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) && (!s.b[1482])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p.p863);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && s.b[1481]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1466])) && (!s.b[1480])) && (!s.b[1481])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1466])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p.p29, 0.0);}
        s.b[1483] = (s.v[647] == 0.0);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1483]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1483])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1484] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1484]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1485] = (p.p832 == 0.5);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && s.b[1485]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && (!s.b[1485])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p.p832)));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1486] = (p.p832 == 0.5);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && s.b[1486]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) && (!s.b[1486])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p.p832);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1484])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p.p841);}
        s.b[1487] = (p.p846 == 0.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && s.b[1487]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1488] = (((-p.p832) * s.v[412]) == (-1.0));s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && s.b[1488]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) && (!s.b[1488])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p.p832) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1483])) && (!s.b[1487])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);}
    }
}
