#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1238] = (s.v[485] < s.v[654]);s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });s.b[1239] = (((((-0.5) * (s.v[485] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && s.b[1239]) {s.store_primal_exp_scaled_input(1200, 485, (s.v[371] * (-0.5)));}
        s.b[1240] = (((-0.5) * (s.v[485] * s.v[371])) < 0.0);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && s.b[1240]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(485), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) && (!s.b[1239])) && (!s.b[1240])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(485), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(485), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1238]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1238])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(485), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1241] = (s.v[485] > 0.0);s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && s.b[1241]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[1171] && s.b[1188]) && s.b[1237]) && (!s.b[1241])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 485);}
        if ((s.b[1171] && s.b[1188]) && s.b[1237]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 485, 0.5, 1203, 0.5, 485, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 485, 0.5, 659, 0.5, 485, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 485, A::sqrt_square_offset(s.ad_value(485), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1242] = (s.v[646] == 0.0);s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1242]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {s.store_primal_scale(1208, 1198, s.v[387]);}
        s.b[1243] = ((p[840] == 0.0) && (p[845] == 0.0));s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1243]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1244] = (p[831] == 0.5);s.store_scalar(1244, if s.b[1244] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1244]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1244])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[831])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1245] = (p[831] == 0.5);s.store_scalar(1245, if s.b[1245] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && s.b[1245]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) && (!s.b[1245])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p[831]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1243])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p[840]);}
        s.b[1246] = (p[845] == 0.0);s.store_scalar(1246, if s.b[1246] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1246]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1247] = (((-p[831]) * s.v[411]) == (-1.0));s.store_scalar(1247, if s.b[1247] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1247]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1247])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[831]) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1248] = (s.v[1228] > 0.0);s.store_scalar(1248, if s.b[1248] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1248]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1248])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1249] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1249, if s.b[1249] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1249]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1249])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1250] = (s.v[1228] > 0.0);s.store_scalar(1250, if s.b[1250] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && s.b[1250]) {s.copy_ad(1229, 1191);}
        s.b[1251] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && s.b[1251]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) && (!s.b[1251])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) && (!s.b[1250])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1246])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[845], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1252] = (p[851] == 0.0);s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1252]) {s.store_scalar(1231, 0.0);}
        s.b[1253] = (p[831] == 0.5);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1253]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[828], s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1253])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p[828]) * (s.v[429])), p[831]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p[828]) * (s.v[426])) * s.v[411]), 1207, 1.0);}
        s.b[1254] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && s.b[1254]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1255] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && s.b[1255]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) && (!s.b[1254])) && (!s.b[1255])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1252])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p[851], 0.0);}
        s.b[1256] = (p[860] > 1000.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1242])) && s.b[1256]) {s.store_scalar(1233, 1.0);}
        s.b[1257] = (s.v[1206] > ((-s.v[444]) * p[860]));s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });s.b[1258] = (p[863] == 4.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && s.b[1258]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) && (!s.b[1258])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p[863]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && s.b[1257]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1242])) && (!s.b[1256])) && (!s.b[1257])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p[860])) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1242])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1259] = (s.v[647] == 0.0);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1259]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1260] = ((p[841] == 0.0) && (p[846] == 0.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1260]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1261] = (p[832] == 0.5);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1261]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1261])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[832])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1262] = (p[832] == 0.5);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && s.b[1262]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) && (!s.b[1262])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p[832]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1260])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p[841]);}
        s.b[1263] = (p[846] == 0.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1263]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1264] = (((-p[832]) * s.v[412]) == (-1.0));s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1264]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1264])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[832]) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1265] = (s.v[1228] > 0.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1265]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1265])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1266] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1266]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1266])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1267] = (s.v[1228] > 0.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && s.b[1267]) {s.copy_ad(1229, 1191);}
        s.b[1268] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && s.b[1268]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) && (!s.b[1268])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) && (!s.b[1267])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1263])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[846], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1269] = (p[852] == 0.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1269]) {s.store_scalar(1231, 0.0);}
        s.b[1270] = (p[832] == 0.5);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1270]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[829], s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1270])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p[829]) * (s.v[430])), p[832]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p[829]) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1271] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && s.b[1271]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1272] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && s.b[1272]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) && (!s.b[1271])) && (!s.b[1272])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1269])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p[852], 0.0);}
        s.b[1273] = (p[861] > 1000.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1259])) && s.b[1273]) {s.store_scalar(1233, 1.0);}
        s.b[1274] = (s.v[1206] > ((-s.v[444]) * p[861]));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });s.b[1275] = (p[864] == 4.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && s.b[1275]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) && (!s.b[1275])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p[864]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && s.b[1274]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1259])) && (!s.b[1273])) && (!s.b[1274])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p[861])) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1259])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1276] = (s.v[648] == 0.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1276]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1277] = ((p[842] == 0.0) && (p[847] == 0.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1277]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1278] = (p[833] == 0.5);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1278]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1278])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[833])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1279] = (p[833] == 0.5);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && s.b[1279]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) && (!s.b[1279])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p[833]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1277])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p[842]);}
        s.b[1280] = (p[847] == 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1280]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1281] = (((-p[833]) * s.v[413]) == (-1.0));s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1281]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1281])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[833]) * s.v[413]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[437]), 1218, 1221, s.v[437], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1282] = (s.v[1228] > 0.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1282]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1282])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1283] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1283]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1283])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1284] = (s.v[1228] > 0.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && s.b[1284]) {s.copy_ad(1229, 1191);}
        s.b[1285] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && s.b[1285]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) && (!s.b[1285])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) && (!s.b[1284])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1280])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[847], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1286] = (p[853] == 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1286]) {s.store_scalar(1231, 0.0);}
        s.b[1287] = (p[833] == 0.5);s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1287]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[830], s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1287])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p[830]) * (s.v[431])), p[833]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p[830]) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1288] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && s.b[1288]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1289] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && s.b[1289]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) && (!s.b[1288])) && (!s.b[1289])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1286])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(485), s.ad_value(1232), s.ad_value(1232)), 1207, p[853], 0.0);}
        s.b[1290] = (p[862] > 1000.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1276])) && s.b[1290]) {s.store_scalar(1233, 1.0);}
        s.b[1291] = (s.v[1206] > ((-s.v[444]) * p[862]));s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });s.b[1292] = (p[865] == 4.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && s.b[1292]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) && (!s.b[1292])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p[865]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && s.b[1291]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1276])) && (!s.b[1290])) && (!s.b[1291])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p[862])) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1276])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_add_scaled_products3_indices(475, 646, 1234, 1.0, 647, 1235, 1.0, 648, 1236, 1.0);s.store_scalar(1205, 0.0);s.store_scalar(1202, 0.0);}
        s.b[1293] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });s.b[1294] = (s.v[486] < s.v[654]);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });s.b[1295] = (((((-0.5) * (s.v[486] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && s.b[1295]) {s.store_primal_exp_scaled_input(1200, 486, (s.v[371] * (-0.5)));}
        s.b[1296] = (((-0.5) * (s.v[486] * s.v[371])) < 0.0);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && s.b[1296]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1200, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) && (!s.b[1295])) && (!s.b[1296])) {s.store_primal_scaled_offset_ad(1200, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1294]) {s.store_primal_div_from_scalar(1201, 1.0, 1200);s.store_primal_square(1198, 1201);}
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1294])) {s.store_primal_mul_scale_offset_mixed_ia(1198, 655, A::sub_scaled_inputs(s.ad_value(486), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_primal_sqrt(1201, 1198);s.store_primal_div_from_scalar(1200, 1.0, 1201);}
        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {s.store_primal_offset(1198, 1198, (-1.0));}
        s.b[1297] = (s.v[486] > 0.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && s.b[1297]) {s.store_primal_scaled_ln_ad(1202, A::add(A::offset(s.ad_value(1200), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1200), 1.0, A::offset(s.ad_value(1200), 3.0)))), (s.v[370] * 2.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && s.b[1293]) && (!s.b[1297])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 486);}
        if ((s.b[1171] && s.b[1188]) && s.b[1293]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 486, 0.5, 1203, 0.5, 486, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 486, 0.5, 659, 0.5, 486, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 486, A::sqrt_square_offset(s.ad_value(486), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1298] = (s.v[646] == 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1298]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {s.store_primal_scale(1208, 1198, s.v[387]);}
        s.b[1299] = ((p[840] == 0.0) && (p[845] == 0.0));s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1299]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1300] = (p[831] == 0.5);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1300]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1300])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[831])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1301] = (p[831] == 0.5);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && s.b[1301]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) && (!s.b[1301])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p[831]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1299])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p[840]);}
        s.b[1302] = (p[845] == 0.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1302]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1303] = (((-p[831]) * s.v[411]) == (-1.0));s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1303]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1303])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[831]) * s.v[411]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[435]), 1218, 1221, s.v[435], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1304] = (s.v[1228] > 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1304]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1304])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1305] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1305]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1305])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1306] = (s.v[1228] > 0.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && s.b[1306]) {s.copy_ad(1229, 1191);}
        s.b[1307] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && s.b[1307]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) && (!s.b[1307])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) && (!s.b[1306])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1302])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[435] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[845], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1308] = (p[851] == 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1308]) {s.store_scalar(1231, 0.0);}
        s.b[1309] = (p[831] == 0.5);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1309]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[828], s.ad_value(1205)), s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1309])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[429]), ((p[828]) * (s.v[429])), p[831]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[426]) * s.v[411]), (((p[828]) * (s.v[426])) * s.v[411]), 1207, 1.0);}
        s.b[1310] = (((((-s.v[441]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && s.b[1310]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(1232), 1.0));}
        s.b[1311] = (((-s.v[441]) / s.v[1232]) < 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && s.b[1311]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 441, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) && (!s.b[1310])) && (!s.b[1311])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 441, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1308])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p[851], 0.0);}
        s.b[1312] = (p[860] > 1000.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1298])) && s.b[1312]) {s.store_scalar(1233, 1.0);}
        s.b[1313] = (s.v[1206] > ((-s.v[444]) * p[860]));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });s.b[1314] = (p[863] == 4.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && s.b[1314]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[448] * s.v[448]) * s.v[448])), 1206, s.v[448], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) && (!s.b[1314])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[448]), p[863]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && s.b[1313]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1298])) && (!s.b[1312])) && (!s.b[1313])) {s.store_offset_scaled(1233, 1206, s.v[451], (((((s.v[444] * p[860])) * (s.v[451]))) + (s.v[445])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1298])) {s.store_mul_scale_offset_mixed_ia(1234, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1315] = (s.v[647] == 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1315]) {s.store_scalar(1235, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {s.store_primal_scale(1208, 1198, s.v[388]);}
        s.b[1316] = ((p[841] == 0.0) && (p[846] == 0.0));s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1316]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_primal_sub_from_scalar(1210, s.v[394], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1317] = (p[832] == 0.5);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[832])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1318] = (p[832] == 0.5);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1318]) {s.store_sqrt_scaled_input(1207, 1210, s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1318])) {s.store_powf_scaled_input(1207, 1210, s.v[430], p[832]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1316])) {s.store_scale(1214, 1207, s.v[424]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1209, 1215, 1213, p[841]);}
        s.b[1319] = (p[846] == 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1319]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[409] * s.v[439]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[436]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1320] = (((-p[832]) * s.v[412]) == (-1.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1320]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1320])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[832]) * s.v[412]));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_product_add_scaled_denominator_indices(1224, 1213, 1223, 1.0, 1213, 1.0, 1223, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1225, A::div(s.ad_value(1217), s.ad_value(1221)), 0.375);s.store_add_scaled_product_indices(1226, 1220, (-1.0), 1218, 1221, 2.0);s.store_add_scaled_value_products_indices(1227, 1220, (-s.v[436]), 1218, 1221, s.v[436], 1217, 1222, 0.5);s.store_mul_scale_offset_indices(1228, 1225, 1226, 1.0, (-1.0));s.store_square(1189, 1228);}
        s.b[1321] = (s.v[1228] > 0.0);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1321]) {s.store_div_from_scalar_offset_scaled_input(1190, 1.0, 1228, s.v[372], 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1321])) {s.store_div_from_scalar_sub_from_scalar_ad(1190, 1.0, 1.0, A::scale(s.ad_value(1228), s.v[372]));}
        s.b[1322] = (((-s.v[1189]) + s.v[1227]) > (-230.25850929940458));s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1322]) {s.store_exp_sub(1207, 1227, 1189);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1322])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1207, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1227), s.ad_value(1189)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_mul_mixed_ai(1191, A::add_scaled_inputs_product(s.ad_value(1190), 0.29214664, A::square(s.ad_value(1190)), s.v[373], A::square(s.ad_value(1190)), s.ad_value(1190), s.v[374]), 1207);}
        s.b[1323] = (s.v[1228] > 0.0);s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && s.b[1323]) {s.copy_ad(1229, 1191);}
        s.b[1324] = (s.v[1227] > (-230.25850929940458));s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && s.b[1324]) {s.store_exp(1207, 1227);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) && (!s.b[1324])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 1227, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) && (!s.b[1323])) {s.store_sub_scaled_inputs(1229, 1207, 2.0, 1191, 1.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1319])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[436] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[846], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1325] = (p[852] == 0.0);s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1325]) {s.store_scalar(1231, 0.0);}
        s.b[1326] = (p[832] == 0.5);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1326]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[829], s.ad_value(1205)), s.v[430]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1326])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[430]), ((p[829]) * (s.v[430])), p[832]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[427]) * s.v[412]), (((p[829]) * (s.v[427])) * s.v[412]), 1207, 1.0);}
        s.b[1327] = (((((-s.v[442]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && s.b[1327]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1232), 1.0));}
        s.b[1328] = (((-s.v[442]) / s.v[1232]) < 0.0);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && s.b[1328]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 442, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) && (!s.b[1327])) && (!s.b[1328])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 442, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1325])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p[852], 0.0);}
        s.b[1329] = (p[861] > 1000.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1315])) && s.b[1329]) {s.store_scalar(1233, 1.0);}
        s.b[1330] = (s.v[1206] > ((-s.v[444]) * p[861]));s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });s.b[1331] = (p[864] == 4.0);s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && s.b[1331]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[449] * s.v[449]) * s.v[449])), 1206, s.v[449], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) && (!s.b[1331])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[449]), p[864]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && s.b[1330]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1315])) && (!s.b[1329])) && (!s.b[1330])) {s.store_offset_scaled(1233, 1206, s.v[452], (((((s.v[444] * p[861])) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1315])) {s.store_mul_scale_offset_mixed_ia(1235, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
        s.b[1332] = (s.v[648] == 0.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1332]) {s.store_scalar(1236, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {s.store_primal_scale(1208, 1198, s.v[389]);}
        s.b[1333] = ((p[842] == 0.0) && (p[847] == 0.0));s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1333]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_primal_sub_from_scalar(1210, s.v[395], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1334] = (p[833] == 0.5);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1334]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1334])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[833])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1335] = (p[833] == 0.5);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && s.b[1335]) {s.store_sqrt_scaled_input(1207, 1210, s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) && (!s.b[1335])) {s.store_powf_scaled_input(1207, 1210, s.v[431], p[833]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1333])) {s.store_scale(1214, 1207, s.v[425]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1209, 1215, 1213, p[842]);}
        s.b[1336] = (p[847] == 0.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1336]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[410] * s.v[440]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[437]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1337] = (((-p[833]) * s.v[413]) == (-1.0));s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && s.b[1337]) {s.store_div_from_scalar_offset_product(1223, 1.0, 1217, 1222, 1.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) && (!s.b[1337])) {s.store_powf_ad(1223, A::offset(A::mul(s.ad_value(1217), s.ad_value(1222)), 1.0), ((-p[833]) * s.v[413]));}
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
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1336])) {s.store_div_scaled_inputs_indices(1230, 1229, (s.v[437] * (1.772453850905516 * 0.5)), 1225, 1.0);s.store_mul3_affine_lhs(1216, 1215, 1230, p[847], 0.0, 1224);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1342] = (p[853] == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1342]) {s.store_scalar(1231, 0.0);}
        s.b[1343] = (p[833] == 0.5);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1343]) {s.store_sqrt_scaled_input_ad(1207, A::sub_from_scalar(p[830], s.ad_value(1205)), s.v[431]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1343])) {s.store_powf_scale_offset_input(1207, 1205, (-s.v[431]), ((p[830]) * (s.v[431])), p[833]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {s.store_div_scaled_offset_numerator_indices(1232, 1205, ((-s.v[428]) * s.v[413]), (((p[830]) * (s.v[428])) * s.v[413]), 1207, 1.0);}
        s.b[1344] = (((((-s.v[443]) / s.v[1232])) as f64).abs() < 230.25850929940458);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && s.b[1344]) {s.store_ad_value(1207, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1232), 1.0));}
        s.b[1345] = (((-s.v[443]) / s.v[1232]) < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && s.b[1345]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1207, 1e-100, (-230.25850929940458), 443, -1.0, 1232, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) && (!s.b[1344])) && (!s.b[1345])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1207, 443, -1.0, 1232, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1342])) {s.store_mul_scale_offset_mixed_ai(1231, A::mul3(s.ad_value(486), s.ad_value(1232), s.ad_value(1232)), 1207, p[853], 0.0);}
        s.b[1346] = (p[862] > 1000.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1332])) && s.b[1346]) {s.store_scalar(1233, 1.0);}
        s.b[1347] = (s.v[1206] > ((-s.v[444]) * p[862]));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });s.b[1348] = (p[865] == 4.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && s.b[1348]) {s.store_mul_scale_offset_mixed_ai(1207, A::mul3_scaled_output(s.ad_value(1206), s.ad_value(1206), s.ad_value(1206), ((s.v[450] * s.v[450]) * s.v[450])), 1206, s.v[450], 0.0);}
        if (((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) && (!s.b[1348])) {s.store_powf_ad(1207, A::abs_scaled_input(s.ad_value(1206), s.v[450]), p[865]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && s.b[1347]) {s.store_div_from_scalar_sub_from_scalar_ad(1233, 1.0, 1.0, s.ad_value(1207));}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1332])) && (!s.b[1346])) && (!s.b[1347])) {s.store_offset_scaled(1233, 1206, s.v[453], (((((s.v[444] * p[862])) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1332])) {s.store_mul_scale_offset_mixed_ia(1236, 1233, A::add_scaled_inputs4(s.ad_value(1208), 1.0, s.ad_value(1209), 1.0, s.ad_value(1216), 1.0, s.ad_value(1231), 1.0), p[29], 0.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1171] && s.b[1188]) && s.b[1349]) && (!s.b[1353])) {s.store_primal_sub_mixed_ai(1202, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1201), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1201), 1.0, A::scale_offset(s.ad_value(1201), 3.0, 1.0))))), (s.v[370] * 2.0)), 487);}
        if ((s.b[1171] && s.b[1188]) && s.b[1349]) {s.store_primal_sub(1203, 656, 1202);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1204, 487, 0.5, 1203, 0.5, 487, 1203, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1205, 487, 0.5, 659, 0.5, 487, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1206, 487, A::sqrt_square_offset(s.ad_value(487), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1354] = (s.v[646] == 0.0);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1354]) {s.store_scalar(1234, 0.0);}
        if ((s.b[1171] && s.b[1188]) && (!s.b[1354])) {s.store_primal_scale(1208, 1198, s.v[387]);}
        s.b[1355] = ((p[840] == 0.0) && (p[845] == 0.0));s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1355]) {s.store_scalar(1209, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_primal_sub_from_scalar(1210, s.v[393], 1204);s.store_primal_sub_from_scalar_ad(1211, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1202), s.ad_value(1210)))));}
        s.b[1356] = (p[831] == 0.5);s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1356]) {s.store_scalar(1212, 0.0);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1356])) {s.store_primal_scaled_add_mixed_ai(1212, A::div_scaled_product(A::square(s.ad_value(1211)), A::ln(s.ad_value(1211)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1211)), 1.0), 1211, (1.0 - (2.0 * p[831])));}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_primal_add(1213, 1211, 1212);}
        s.b[1357] = (p[831] == 0.5);s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && s.b[1357]) {s.store_sqrt_scaled_input(1207, 1210, s.v[429]);}
        if ((((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) && (!s.b[1357])) {s.store_powf_scaled_input(1207, 1210, s.v[429], p[831]);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1355])) {s.store_scale(1214, 1207, s.v[423]);s.store_mul_scale_offset_indices(1215, 1214, 1201, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(1209, 1215, 1213, p[840]);}
        s.b[1358] = (p[845] == 0.0);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && s.b[1358]) {s.store_scalar(1216, 0.0);}
        if (((s.b[1171] && s.b[1188]) && (!s.b[1354])) && (!s.b[1358])) {s.store_div_scaled_inputs_indices(1217, 1214, (s.v[408] * s.v[438]), 1210, 1.0);s.store_div_from_scalar(1218, (0.666666666666667 * s.v[435]), 1217);s.store_square(1219, 1218);s.store_sqrt_div_scaled_square_offset_denominator(1220, 1219, 1.0, 1.0, 1.0);s.store_sqrt(1221, 1220);s.store_mul(1222, 1220, 1221);}
        s.b[1359] = (((-p[831]) * s.v[411]) == (-1.0));s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
    }
}
