#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1318] = (p.p850 == 0.5);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && (!s.b[1318])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1319] = (p.p850 == 0.5);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && s.b[1319]) {s.store_sqrt_scaled_input(1191, 1194, s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) && (!s.b[1319])) {s.store_powf_scaled_input(1191, 1194, s.v[432], p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1317])) {s.store_scale(1198, 1191, s.v[426]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(1193, 1199, 1197, p.p859);}
        s.b[1320] = (p.p864 == 0.0);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1320]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[411] * s.v[441]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1321] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1321]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1321])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[438]), 1202, 1205, s.v[438], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1322] = (s.v[1212] > 0.0);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1322]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1322])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1323] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1323]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1323])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1324] = (s.v[1212] > 0.0);s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && s.b[1324]) {s.copy_ad(1213, 1175);}
        s.b[1325] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) && s.b[1325]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) && (!s.b[1325])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) && (!s.b[1324])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1320])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[438] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);}
        s.b[1326] = (p.p870 == 0.0);s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1326]) {s.store_scalar(1215, 0.0);}
        s.b[1327] = (p.p850 == 0.5);s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && s.b[1327]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1327])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 1191, 1.0);}
        s.b[1328] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && s.b[1328]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));}
        s.b[1329] = (((-s.v[444]) / s.v[1216]) < 0.0);s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1328])) && s.b[1329]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 444, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) && (!s.b[1328])) && (!s.b[1329])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 444, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1326])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870, 0.0);}
        s.b[1330] = (p.p879 > 1000.0);s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1316])) && s.b[1330]) {s.store_scalar(1217, 1.0);}
        s.b[1331] = (s.v[1190] > ((-s.v[445]) * p.p879));s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });s.b[1332] = (p.p882 == 4.0);s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) && s.b[1332]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) && (!s.b[1332])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && s.b[1331]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1316])) && (!s.b[1330])) && (!s.b[1331])) {s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1316])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(477, 647, 1218, 1.0, 648, 1219, 1.0, 649, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1333] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });s.b[1334] = (s.v[488] < s.v[655]);s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });s.b[1335] = (((((-0.5) * (s.v[488] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && s.b[1335]) {s.store_primal_exp_scaled_input(1184, 488, (s.v[372] * (-0.5)));}
        s.b[1336] = (((-0.5) * (s.v[488] * s.v[372])) < 0.0);s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && (!s.b[1335])) && s.b[1336]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) && (!s.b[1335])) && (!s.b[1336])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1334]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && (!s.b[1334])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 656, A::sub_scaled_inputs(s.ad_value(488), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1333]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1337] = (s.v[488] > 0.0);s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && s.b[1337]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1333]) && (!s.b[1337])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 488);}
        if ((s.b[1155] && s.b[1172]) && s.b[1333]) {s.store_primal_sub(1187, 657, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 488, 0.5, 1187, 0.5, 488, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 488, 0.5, 660, 0.5, 488, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 488, A::sqrt_square_offset(s.ad_value(488), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1338] = (s.v[647] == 0.0);s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1338]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1338])) {s.store_primal_scale(1192, 1182, s.v[388]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1339] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1339]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {s.store_primal_sub_from_scalar(1194, s.v[394], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1340] = (p.p848 == 0.5);s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && s.b[1340]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && (!s.b[1340])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1341] = (p.p848 == 0.5);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && s.b[1341]) {s.store_sqrt_scaled_input(1191, 1194, s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) && (!s.b[1341])) {s.store_powf_scaled_input(1191, 1194, s.v[430], p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1339])) {s.store_scale(1198, 1191, s.v[424]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1193, 1199, 1197, p.p857);}
        s.b[1342] = (p.p862 == 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1342]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[409] * s.v[439]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1343] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1343]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1343])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[436]), 1202, 1205, s.v[436], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1344] = (s.v[1212] > 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1344]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1344])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1345] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1345]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1345])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1346] = (s.v[1212] > 0.0);s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && s.b[1346]) {s.copy_ad(1213, 1175);}
        s.b[1347] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) && s.b[1347]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) && (!s.b[1347])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) && (!s.b[1346])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1342])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[436] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);}
        s.b[1348] = (p.p868 == 0.0);s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1348]) {s.store_scalar(1215, 0.0);}
        s.b[1349] = (p.p848 == 0.5);s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && s.b[1349]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1349])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 1191, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1350] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1350, if s.b[1350] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && s.b[1350]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));}
        s.b[1351] = (((-s.v[442]) / s.v[1216]) < 0.0);s.store_scalar(1351, if s.b[1351] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1350])) && s.b[1351]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 442, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) && (!s.b[1350])) && (!s.b[1351])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 442, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1348])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868, 0.0);}
        s.b[1352] = (p.p877 > 1000.0);s.store_scalar(1352, if s.b[1352] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1338])) && s.b[1352]) {s.store_scalar(1217, 1.0);}
        s.b[1353] = (s.v[1190] > ((-s.v[445]) * p.p877));s.store_scalar(1353, if s.b[1353] { 1.0 } else { 0.0 });s.b[1354] = (p.p880 == 4.0);s.store_scalar(1354, if s.b[1354] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) && s.b[1354]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) && (!s.b[1354])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && s.b[1353]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1338])) && (!s.b[1352])) && (!s.b[1353])) {s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1338])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1355] = (s.v[648] == 0.0);s.store_scalar(1355, if s.b[1355] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1355]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1355])) {s.store_primal_scale(1192, 1182, s.v[389]);}
        s.b[1356] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(1356, if s.b[1356] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1356]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {s.store_primal_sub_from_scalar(1194, s.v[395], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1357] = (p.p849 == 0.5);s.store_scalar(1357, if s.b[1357] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && s.b[1357]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && (!s.b[1357])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1358] = (p.p849 == 0.5);s.store_scalar(1358, if s.b[1358] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && s.b[1358]) {s.store_sqrt_scaled_input(1191, 1194, s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) && (!s.b[1358])) {s.store_powf_scaled_input(1191, 1194, s.v[431], p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1356])) {s.store_scale(1198, 1191, s.v[425]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1193, 1199, 1197, p.p858);}
        s.b[1359] = (p.p863 == 0.0);s.store_scalar(1359, if s.b[1359] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1359]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[410] * s.v[440]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1360] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(1360, if s.b[1360] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1360]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1360])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[437]), 1202, 1205, s.v[437], 1201, 1206, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1361] = (s.v[1212] > 0.0);s.store_scalar(1361, if s.b[1361] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1361]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1361])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1362] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1362, if s.b[1362] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1362]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1362])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1363] = (s.v[1212] > 0.0);s.store_scalar(1363, if s.b[1363] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && s.b[1363]) {s.copy_ad(1213, 1175);}
        s.b[1364] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) && s.b[1364]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) && (!s.b[1364])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) && (!s.b[1363])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1359])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[437] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);}
        s.b[1365] = (p.p869 == 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1365]) {s.store_scalar(1215, 0.0);}
        s.b[1366] = (p.p849 == 0.5);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && s.b[1366]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1366])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 1191, 1.0);}
        s.b[1367] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && s.b[1367]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));}
        s.b[1368] = (((-s.v[443]) / s.v[1216]) < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1367])) && s.b[1368]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 443, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) && (!s.b[1367])) && (!s.b[1368])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 443, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1365])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869, 0.0);}
        s.b[1369] = (p.p878 > 1000.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1355])) && s.b[1369]) {s.store_scalar(1217, 1.0);}
        s.b[1370] = (s.v[1190] > ((-s.v[445]) * p.p878));s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });s.b[1371] = (p.p881 == 4.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) && s.b[1371]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) && (!s.b[1371])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && s.b[1370]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1355])) && (!s.b[1369])) && (!s.b[1370])) {s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1355])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1372] = (s.v[649] == 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1372]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1372])) {s.store_primal_scale(1192, 1182, s.v[390]);}
        s.b[1373] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1373]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {s.store_primal_sub_from_scalar(1194, s.v[396], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1374] = (p.p850 == 0.5);s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && s.b[1374]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && (!s.b[1374])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1375] = (p.p850 == 0.5);s.store_scalar(1375, if s.b[1375] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && s.b[1375]) {s.store_sqrt_scaled_input(1191, 1194, s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) && (!s.b[1375])) {s.store_powf_scaled_input(1191, 1194, s.v[432], p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1373])) {s.store_scale(1198, 1191, s.v[426]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(1193, 1199, 1197, p.p859);}
        s.b[1376] = (p.p864 == 0.0);s.store_scalar(1376, if s.b[1376] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1376]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[411] * s.v[441]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1377] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(1377, if s.b[1377] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1377]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1377])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[438]), 1202, 1205, s.v[438], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1378] = (s.v[1212] > 0.0);s.store_scalar(1378, if s.b[1378] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1378]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1378])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1379] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1379, if s.b[1379] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1379]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1379])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1380] = (s.v[1212] > 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && s.b[1380]) {s.copy_ad(1213, 1175);}
        s.b[1381] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) && s.b[1381]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) && (!s.b[1381])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) && (!s.b[1380])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1376])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[438] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);}
        s.b[1382] = (p.p870 == 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1382]) {s.store_scalar(1215, 0.0);}
        s.b[1383] = (p.p850 == 0.5);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && s.b[1383]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1383])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 1191, 1.0);}
        s.b[1384] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && s.b[1384]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));}
        s.b[1385] = (((-s.v[444]) / s.v[1216]) < 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1384])) && s.b[1385]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 444, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) && (!s.b[1384])) && (!s.b[1385])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 444, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1382])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(488), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870, 0.0);}
        s.b[1386] = (p.p879 > 1000.0);s.store_scalar(1386, if s.b[1386] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1372])) && s.b[1386]) {s.store_scalar(1217, 1.0);}
        s.b[1387] = (s.v[1190] > ((-s.v[445]) * p.p879));s.store_scalar(1387, if s.b[1387] { 1.0 } else { 0.0 });s.b[1388] = (p.p882 == 4.0);s.store_scalar(1388, if s.b[1388] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) && s.b[1388]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) && (!s.b[1388])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && s.b[1387]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1372])) && (!s.b[1386])) && (!s.b[1387])) {s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1372])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(478, 647, 1218, 1.0, 648, 1219, 1.0, 649, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1389] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1389, if s.b[1389] { 1.0 } else { 0.0 });s.b[1390] = (s.v[489] < s.v[655]);s.store_scalar(1390, if s.b[1390] { 1.0 } else { 0.0 });s.b[1391] = (((((-0.5) * (s.v[489] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && s.b[1391]) {s.store_primal_exp_scaled_input(1184, 489, (s.v[372] * (-0.5)));}
        s.b[1392] = (((-0.5) * (s.v[489] * s.v[372])) < 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && (!s.b[1391])) && s.b[1392]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(489), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) && (!s.b[1391])) && (!s.b[1392])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(489), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(489), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1390]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && (!s.b[1390])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 656, A::sub_scaled_inputs(s.ad_value(489), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1389]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1393] = (s.v[489] > 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && s.b[1393]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1389]) && (!s.b[1393])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 489);}
        if ((s.b[1155] && s.b[1172]) && s.b[1389]) {s.store_primal_sub(1187, 657, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 489, 0.5, 1187, 0.5, 489, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 489, 0.5, 660, 0.5, 489, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 489, A::sqrt_square_offset(s.ad_value(489), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1394] = (s.v[647] == 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1394]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1394])) {s.store_primal_scale(1192, 1182, s.v[388]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1395] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1395]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {s.store_primal_sub_from_scalar(1194, s.v[394], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1396] = (p.p848 == 0.5);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && s.b[1396]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && (!s.b[1396])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1397] = (p.p848 == 0.5);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && s.b[1397]) {s.store_sqrt_scaled_input(1191, 1194, s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) && (!s.b[1397])) {s.store_powf_scaled_input(1191, 1194, s.v[430], p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1395])) {s.store_scale(1198, 1191, s.v[424]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1193, 1199, 1197, p.p857);}
        s.b[1398] = (p.p862 == 0.0);s.store_scalar(1398, if s.b[1398] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1398]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[409] * s.v[439]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1399] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(1399, if s.b[1399] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1399]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1399])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[436]), 1202, 1205, s.v[436], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1400] = (s.v[1212] > 0.0);s.store_scalar(1400, if s.b[1400] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1400]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1400])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1401] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1401, if s.b[1401] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1401]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1401])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1402] = (s.v[1212] > 0.0);s.store_scalar(1402, if s.b[1402] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && s.b[1402]) {s.copy_ad(1213, 1175);}
        s.b[1403] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1403, if s.b[1403] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) && s.b[1403]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) && (!s.b[1403])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) && (!s.b[1402])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1398])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[436] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);}
        s.b[1404] = (p.p868 == 0.0);s.store_scalar(1404, if s.b[1404] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1404]) {s.store_scalar(1215, 0.0);}
        s.b[1405] = (p.p848 == 0.5);s.store_scalar(1405, if s.b[1405] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && s.b[1405]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1405])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 1191, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1406] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1406, if s.b[1406] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && s.b[1406]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));}
        s.b[1407] = (((-s.v[442]) / s.v[1216]) < 0.0);s.store_scalar(1407, if s.b[1407] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1406])) && s.b[1407]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 442, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) && (!s.b[1406])) && (!s.b[1407])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 442, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1404])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868, 0.0);}
        s.b[1408] = (p.p877 > 1000.0);s.store_scalar(1408, if s.b[1408] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1394])) && s.b[1408]) {s.store_scalar(1217, 1.0);}
        s.b[1409] = (s.v[1190] > ((-s.v[445]) * p.p877));s.store_scalar(1409, if s.b[1409] { 1.0 } else { 0.0 });s.b[1410] = (p.p880 == 4.0);s.store_scalar(1410, if s.b[1410] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) && s.b[1410]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) && (!s.b[1410])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && s.b[1409]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1394])) && (!s.b[1408])) && (!s.b[1409])) {s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1394])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1411] = (s.v[648] == 0.0);s.store_scalar(1411, if s.b[1411] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1411]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1411])) {s.store_primal_scale(1192, 1182, s.v[389]);}
        s.b[1412] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(1412, if s.b[1412] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1412]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {s.store_primal_sub_from_scalar(1194, s.v[395], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1413] = (p.p849 == 0.5);s.store_scalar(1413, if s.b[1413] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && s.b[1413]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && (!s.b[1413])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1414] = (p.p849 == 0.5);s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && s.b[1414]) {s.store_sqrt_scaled_input(1191, 1194, s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) && (!s.b[1414])) {s.store_powf_scaled_input(1191, 1194, s.v[431], p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1412])) {s.store_scale(1198, 1191, s.v[425]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1193, 1199, 1197, p.p858);}
        s.b[1415] = (p.p863 == 0.0);s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1415]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[410] * s.v[440]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1416] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1416]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1416])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[437]), 1202, 1205, s.v[437], 1201, 1206, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1417] = (s.v[1212] > 0.0);s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1417]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1417])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1418] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1418]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1418])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1419] = (s.v[1212] > 0.0);s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && s.b[1419]) {s.copy_ad(1213, 1175);}
        s.b[1420] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) && s.b[1420]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) && (!s.b[1420])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) && (!s.b[1419])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1415])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[437] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);}
        s.b[1421] = (p.p869 == 0.0);s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1421]) {s.store_scalar(1215, 0.0);}
        s.b[1422] = (p.p849 == 0.5);s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && s.b[1422]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1422])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 1191, 1.0);}
        s.b[1423] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && s.b[1423]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));}
        s.b[1424] = (((-s.v[443]) / s.v[1216]) < 0.0);s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1423])) && s.b[1424]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 443, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) && (!s.b[1423])) && (!s.b[1424])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 443, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1421])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869, 0.0);}
        s.b[1425] = (p.p878 > 1000.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1411])) && s.b[1425]) {s.store_scalar(1217, 1.0);}
        s.b[1426] = (s.v[1190] > ((-s.v[445]) * p.p878));s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });s.b[1427] = (p.p881 == 4.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) && s.b[1427]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) && (!s.b[1427])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && s.b[1426]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1411])) && (!s.b[1425])) && (!s.b[1426])) {s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1411])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1428] = (s.v[649] == 0.0);s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1428]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1428])) {s.store_primal_scale(1192, 1182, s.v[390]);}
        s.b[1429] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1429]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {s.store_primal_sub_from_scalar(1194, s.v[396], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1430] = (p.p850 == 0.5);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && s.b[1430]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && (!s.b[1430])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1431] = (p.p850 == 0.5);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && s.b[1431]) {s.store_sqrt_scaled_input(1191, 1194, s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) && (!s.b[1431])) {s.store_powf_scaled_input(1191, 1194, s.v[432], p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1429])) {s.store_scale(1198, 1191, s.v[426]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(1193, 1199, 1197, p.p859);}
        s.b[1432] = (p.p864 == 0.0);s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1432]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[411] * s.v[441]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1433] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1433]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1433])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[438]), 1202, 1205, s.v[438], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1434] = (s.v[1212] > 0.0);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1434]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1434])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1435] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1435]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1435])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1436] = (s.v[1212] > 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && s.b[1436]) {s.copy_ad(1213, 1175);}
        s.b[1437] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) && s.b[1437]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) && (!s.b[1437])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) && (!s.b[1436])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1432])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[438] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);}
        s.b[1438] = (p.p870 == 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1438]) {s.store_scalar(1215, 0.0);}
        s.b[1439] = (p.p850 == 0.5);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && s.b[1439]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1439])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 1191, 1.0);}
        s.b[1440] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && s.b[1440]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));}
        s.b[1441] = (((-s.v[444]) / s.v[1216]) < 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1440])) && s.b[1441]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 444, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) && (!s.b[1440])) && (!s.b[1441])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 444, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1438])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(489), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870, 0.0);}
        s.b[1442] = (p.p879 > 1000.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1428])) && s.b[1442]) {s.store_scalar(1217, 1.0);}
        s.b[1443] = (s.v[1190] > ((-s.v[445]) * p.p879));s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });s.b[1444] = (p.p882 == 4.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) && s.b[1444]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) && (!s.b[1444])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && s.b[1443]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1428])) && (!s.b[1442])) && (!s.b[1443])) {s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1428])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(479, 647, 1218, 1.0, 648, 1219, 1.0, 649, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1445] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });s.b[1446] = (s.v[490] < s.v[655]);s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });s.b[1447] = (((((-0.5) * (s.v[490] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && s.b[1447]) {s.store_primal_exp_scaled_input(1184, 490, (s.v[372] * (-0.5)));}
        s.b[1448] = (((-0.5) * (s.v[490] * s.v[372])) < 0.0);s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && (!s.b[1447])) && s.b[1448]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(490), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) && (!s.b[1447])) && (!s.b[1448])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(490), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(490), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1446]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && (!s.b[1446])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 656, A::sub_scaled_inputs(s.ad_value(490), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1445]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1449] = (s.v[490] > 0.0);s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && s.b[1449]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1445]) && (!s.b[1449])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 490);}
        if ((s.b[1155] && s.b[1172]) && s.b[1445]) {s.store_primal_sub(1187, 657, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 490, 0.5, 1187, 0.5, 490, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 490, 0.5, 660, 0.5, 490, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 490, A::sqrt_square_offset(s.ad_value(490), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1450] = (s.v[647] == 0.0);s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1450]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1450])) {s.store_primal_scale(1192, 1182, s.v[388]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1451] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1451]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {s.store_primal_sub_from_scalar(1194, s.v[394], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1452] = (p.p848 == 0.5);s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && s.b[1452]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && (!s.b[1452])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p848)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1453] = (p.p848 == 0.5);s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && s.b[1453]) {s.store_sqrt_scaled_input(1191, 1194, s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) && (!s.b[1453])) {s.store_powf_scaled_input(1191, 1194, s.v[430], p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1451])) {s.store_scale(1198, 1191, s.v[424]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(1193, 1199, 1197, p.p857);}
        s.b[1454] = (p.p862 == 0.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1454]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[409] * s.v[439]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[436]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1455] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1455]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1455])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[436]), 1202, 1205, s.v[436], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1456] = (s.v[1212] > 0.0);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1456]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1456])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1457] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1457]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1457])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1458] = (s.v[1212] > 0.0);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && s.b[1458]) {s.copy_ad(1213, 1175);}
        s.b[1459] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) && s.b[1459]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) && (!s.b[1459])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) && (!s.b[1458])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1454])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[436] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p862, 0.0, 1208);}
        s.b[1460] = (p.p868 == 0.0);s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1460]) {s.store_scalar(1215, 0.0);}
        s.b[1461] = (p.p848 == 0.5);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && s.b[1461]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p845, s.ad_value(1189)), s.v[430]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1461])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 1191, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1462] = (((((-s.v[442]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && s.b[1462]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(1216), 1.0));}
        s.b[1463] = (((-s.v[442]) / s.v[1216]) < 0.0);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1462])) && s.b[1463]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 442, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) && (!s.b[1462])) && (!s.b[1463])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 442, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1460])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p868, 0.0);}
        s.b[1464] = (p.p877 > 1000.0);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1450])) && s.b[1464]) {s.store_scalar(1217, 1.0);}
        s.b[1465] = (s.v[1190] > ((-s.v[445]) * p.p877));s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });s.b[1466] = (p.p880 == 4.0);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) && s.b[1466]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[449] * s.v[449]) * s.v[449])), 1190, s.v[449], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) && (!s.b[1466])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[449]), p.p880);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && s.b[1465]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1450])) && (!s.b[1464])) && (!s.b[1465])) {s.store_offset_scaled(1217, 1190, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1450])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1467] = (s.v[648] == 0.0);s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1467]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1467])) {s.store_primal_scale(1192, 1182, s.v[389]);}
        s.b[1468] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1468]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {s.store_primal_sub_from_scalar(1194, s.v[395], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1469] = (p.p849 == 0.5);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && s.b[1469]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && (!s.b[1469])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p849)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1470] = (p.p849 == 0.5);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && s.b[1470]) {s.store_sqrt_scaled_input(1191, 1194, s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) && (!s.b[1470])) {s.store_powf_scaled_input(1191, 1194, s.v[431], p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1468])) {s.store_scale(1198, 1191, s.v[425]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(1193, 1199, 1197, p.p858);}
        s.b[1471] = (p.p863 == 0.0);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1471]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[410] * s.v[440]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[437]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1472] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[437]), 1202, 1205, s.v[437], 1201, 1206, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1473] = (s.v[1212] > 0.0);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1473]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1473])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1474] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1474]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1474])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1475] = (s.v[1212] > 0.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && s.b[1475]) {s.copy_ad(1213, 1175);}
        s.b[1476] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) && s.b[1476]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) && (!s.b[1476])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1475])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1471])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[437] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p863, 0.0, 1208);}
        s.b[1477] = (p.p869 == 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1477]) {s.store_scalar(1215, 0.0);}
        s.b[1478] = (p.p849 == 0.5);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && s.b[1478]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p846, s.ad_value(1189)), s.v[431]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1478])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 1191, 1.0);}
        s.b[1479] = (((((-s.v[443]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && s.b[1479]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(1216), 1.0));}
        s.b[1480] = (((-s.v[443]) / s.v[1216]) < 0.0);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1479])) && s.b[1480]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 443, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) && (!s.b[1479])) && (!s.b[1480])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 443, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1477])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p869, 0.0);}
        s.b[1481] = (p.p878 > 1000.0);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1467])) && s.b[1481]) {s.store_scalar(1217, 1.0);}
        s.b[1482] = (s.v[1190] > ((-s.v[445]) * p.p878));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });s.b[1483] = (p.p881 == 4.0);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) && s.b[1483]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[450] * s.v[450]) * s.v[450])), 1190, s.v[450], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) && (!s.b[1483])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[450]), p.p881);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && s.b[1482]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1467])) && (!s.b[1481])) && (!s.b[1482])) {s.store_offset_scaled(1217, 1190, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1467])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1484] = (s.v[649] == 0.0);s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1484]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1484])) {s.store_primal_scale(1192, 1182, s.v[390]);}
        s.b[1485] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1485]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {s.store_primal_sub_from_scalar(1194, s.v[396], 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1486] = (p.p850 == 0.5);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && s.b[1486]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && (!s.b[1486])) {s.store_primal_scaled_add_mixed_ai(1196, A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), 1195, (1.0 - (2.0 * p.p850)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1487] = (p.p850 == 0.5);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && s.b[1487]) {s.store_sqrt_scaled_input(1191, 1194, s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) && (!s.b[1487])) {s.store_powf_scaled_input(1191, 1194, s.v[432], p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1485])) {s.store_scale(1198, 1191, s.v[426]);s.store_mul_scale_offset_indices(1199, 1198, 1185, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(1193, 1199, 1197, p.p859);}
        s.b[1488] = (p.p864 == 0.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1488]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {s.store_div_scaled_inputs_indices(1201, 1198, (s.v[411] * s.v[441]), 1194, 1.0);s.store_div_from_scalar(1202, (0.666666666666667 * s.v[438]), 1201);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1489] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1489]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1489])) {s.store_powf_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_indices(1211, 1204, (-s.v[438]), 1202, 1205, s.v[438], 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1490] = (s.v[1212] > 0.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1490]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1490])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1491] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1491]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1491])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1492] = (s.v[1212] > 0.0);s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && s.b[1492]) {s.copy_ad(1213, 1175);}
        s.b[1493] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) && s.b[1493]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) && (!s.b[1493])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) && (!s.b[1492])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1488])) {s.store_div_scaled_inputs_indices(1214, 1213, (s.v[438] * (1.772453850905516 * 0.5)), 1209, 1.0);s.store_mul3_affine_lhs(1200, 1199, 1214, p.p864, 0.0, 1208);}
        s.b[1494] = (p.p870 == 0.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1494]) {s.store_scalar(1215, 0.0);}
        s.b[1495] = (p.p850 == 0.5);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && s.b[1495]) {s.store_sqrt_scaled_input_ad(1191, A::sub_from_scalar(p.p847, s.ad_value(1189)), s.v[432]);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1495])) {s.store_powf_scale_offset_input(1191, 1189, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) {s.store_div_scaled_offset_numerator_indices(1216, 1189, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 1191, 1.0);}
        s.b[1496] = (((((-s.v[444]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && s.b[1496]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(1216), 1.0));}
        s.b[1497] = (((-s.v[444]) / s.v[1216]) < 0.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1496])) && s.b[1497]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 444, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) && (!s.b[1496])) && (!s.b[1497])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 444, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
}
