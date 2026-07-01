#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1402] {
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 843, 844);
            s.store_mul(846, 843, 845);
            s.store_mul_add_scaled_inputs3_offset_rhs(915, 842, s.ad_value(875), 1.0, s.ad_value(843), (-0.5), s.ad_value(846), 1.0, 0.0);
        }

        s.b[1414] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1414, if s.b[1414] { 1.0 } else { 0.0 });

        if (s.b[1402] && s.b[1414]) {
            s.store_mul(1121, 894, 1130);
            s.store_scaled_offset_ad(855, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);
            s.store_div(845, 1121, 855);
            s.store_mul(846, 1121, 845);
            s.store_add_scaled_product_right_ad(915, 915, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 1.0, s.ad_value(1121), (-0.5), s.ad_value(846), 1.0), 1.0);
        }

        s.b[1415] = (p.p129 > 0.5);
        s.store_scalar(1415, if s.b[1415] { 1.0 } else { 0.0 });

        if (s.b[1402] && s.b[1415]) {
            s.store_scale(844, 844, 2.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 842, s.ad_value(875), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(844), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1416] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1416, if s.b[1416] { 1.0 } else { 0.0 });

        if ((s.b[1402] && s.b[1415]) && s.b[1416]) {
            s.store_scale(855, 855, 2.0);
            s.store_add_scaled_product_right_ad(919, 919, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 0.5, s.ad_value(1121), 0.25, A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 1.0, s.ad_value(855), 1.0), -1.0), (-1.0));
        }

        s.b[1417] = (p.p129 < 0.5);
        s.store_scalar(1417, if s.b[1417] { 1.0 } else { 0.0 });

        if ((s.b[1402] && (!s.b[1415])) && s.b[1417]) {
            s.store_scale(844, 844, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(845, 842, 0.5, 844, 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 875, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(875), A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 845, 846);
        }

        s.b[1418] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1418, if s.b[1418] { 1.0 } else { 0.0 });

        if (((s.b[1402] && (!s.b[1415])) && s.b[1417]) && s.b[1418]) {
            s.store_scale(855, 855, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(845, 1115, 0.5, 855, 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 1118, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(1118), A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 845, 846);
            s.store_add(919, 919, 1137);
        }

        if ((s.b[1402] && (!s.b[1415])) && (!s.b[1417])) {
            s.store_scaled_add(919, 915, 1006, (-0.5));
        }

        s.b[1419] = (s.v[37] == 2.0);
        s.store_scalar(1419, if s.b[1419] { 1.0 } else { 0.0 });

        if (s.b[1402] && s.b[1419]) {
            s.store_scalar(939, 0.0);
        }

        if (s.b[1402] && (!s.b[1419])) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
            s.store_mul_sub_rhs(939, 914, 902, 824);
        }

        if s.b[1402] {
            s.store_add_scaled_inputs3_indices(916, 915, 1.0, 938, 1.0, 937, 1.0);
            s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);
            s.copy_ad(920, 939);
            s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 919, (-1.0), 917, (-1.0), 920, (-1.0));
        }

        s.b[1420] = (p.p61 == 3.0);
        s.store_scalar(1420, if s.b[1420] { 1.0 } else { 0.0 });

        s.b[1421] = (p.p41 == 0.0);
        s.store_scalar(1421, if s.b[1421] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1421]) {
            s.store_div_from_scalar(997, 3.453133e-11, 62);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1421])) {
            s.store_div_scaled_inputs_indices(997, 416, 8.85418e-12, 62, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_div_scaled_product_indices(842, 842, 415, 1.0, 62, 1.0);
            s.store_div_scaled_inputs_indices(981, 981, p.p66, 62, 1.0);
            s.store_scale(998, 62, 100000000.0);
        }

        s.b[1422] = (p.p27 > 0.0);
        s.store_scalar(1422, if s.b[1422] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1422]) {
            s.store_div_scaled_inputs_indices(1115, 1115, p.p66, 62, 1.0);
            s.store_div_scaled_inputs_indices(1116, 1116, p.p66, 62, 1.0);
        }

        s.b[1423] = (s.v[37] == 2.0);
        s.store_scalar(1423, if s.b[1423] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1423]) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
            s.store_scalar(1015, 0.0);
        }

        s.b[1424] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1424]) {
            s.store_add_ad_lhs(1015, A::add_scaled_inputs_product(s.ad_value(1014), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(943), (-1.0)), 324);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1424])) {
            s.store_add(1015, 67, 324);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_add_scaled_inputs3_offset_indices(813, 1015, 1.0, 825, (-1.0), 841, 1.0, (-0.02));
        }

        s.b[1425] = (s.v[1015] <= 0.0);
        s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1425]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (-(4.0 * 0.02)));
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1425])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (4.0 * 0.02));
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_add_scaled_inputs3_indices(812, 1015, 1.0, 813, (-0.5), 843, (-0.5));
        }

        s.b[1426] = (p.p27 > 0.0);
        s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {
            s.store_offset(1126, 1015, p.p1033);
            s.store_add_scaled_inputs3_offset_indices(813, 1126, 1.0, 1125, (-1.0), 841, 1.0, (-0.02));
        }

        s.b[1427] = (s.v[1126] <= 0.0);
        s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && s.b[1427]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (-(100.0 * 0.02)));
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && (!s.b[1427])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (100.0 * 0.02));
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {
            s.store_add_scaled_inputs3_indices(1128, 1126, 1.0, 813, (-0.5), 843, (-0.5));
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div_scaled_inputs3_indices(843, 825, 1.0, 841, (-1.0), 1015, -1.0, 998, 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1428] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1428]) {
            s.store_mul_exp_rhs(999, 360, 859);
        }

        s.b[1429] = (s.v[859] <= (-100.0));
        s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && s.b[1429]) {
            s.store_scale(999, 360, 3.720075976e-44);
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_scale(999, 360, 2.688117142e43);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(1000, 62, 0.001);
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 999, (-1.0), 1000, -1.0);
            s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);
            s.store_add_scaled_inputs3_indices(999, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1430] = (s.v[999] < 1e-15);
        s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1430]) {
            s.store_scalar(999, 1e-15);
        }

        s.b[1431] = (p.p27 > 0.0);
        s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_div_scaled_inputs3_indices(843, 1125, 1.0, 841, (-1.0), 1126, -1.0, 998, 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1432] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1432]) {
            s.store_mul_exp_rhs(1131, 360, 859);
        }

        s.b[1433] = (s.v[859] <= (-100.0));
        s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && s.b[1433]) {
            s.store_scale(1131, 360, 3.720075976e-44);
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_scale(1131, 360, 2.688117142e43);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 1131, (-1.0), 1000, -1.0);
            s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);
            s.store_add_scaled_inputs3_indices(1131, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1434] = (s.v[1131] < 1e-15);
        s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1434]) {
            s.store_scalar(1131, 1e-15);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div(1001, 417, 999);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1001, 1.0);
            s.store_mul(1002, 845, 1001);
        }

        s.b[1435] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1435]) {
            s.store_div(1132, 417, 1131);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1132, 1.0);
            s.store_mul(1133, 845, 1132);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);
        }

        s.b[1436] = (p.p27 > 0.0);
        s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1436]) {
            s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_sub_rhs(938, 982, 812, 1015);
        }

        s.b[1437] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1437]) {
            s.store_mul_sub_rhs(1123, 1135, 1128, 1126);
            s.store_add(938, 938, 1123);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(843, 376, 0.5);
            s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);
        }

        s.b[1438] = (s.v[376] == 0.0);
        s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1438]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1439] = (s.v[846] < 0.0);
        s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && s.b[1439]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && (!s.b[1439])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_ad_product_rhs_mixed_ia(937, 982, 376, A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.b[1440] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);
        }

        s.b[1441] = (s.v[376] == 0.0);
        s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && s.b[1441]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1442] = (s.v[846] < 0.0);
        s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && s.b[1442]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && (!s.b[1442])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_mul_ad_product_rhs_mixed_ia(1124, 1135, 376, A::sub(s.ad_value(844), s.ad_value(843)));
            s.store_add(937, 937, 1124);
        }

        s.b[1443] = (s.v[376] <= 0.0);
        s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1443]) {
            s.store_scaled_mul(936, 362, 832, 0.25);
            s.store_scale(843, 339, 0.5);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1443])) {
            s.store_mul_product3_indices(936, 376, 362, 832, 376, 1.0);
            s.store_mul(843, 376, 339);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs(844, 843, 2.0, 875, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_mul_ad_rhs(1004, 832, {
                if ((1.0 + ((s.v[844] * s.v[875]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(875), 1.0, s.ad_value(936), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1444] = (p.p27 > 0.0);
        s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_add_scaled_inputs(844, 843, 2.0, 1118, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_mul_ad_rhs(1136, 832, {
                if ((1.0 + ((s.v[844] * s.v[1118]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(1118), 1.0, s.ad_value(936), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs3_indices(846, 829, 4.0, 1015, ((-1.0) * 4.0), 942, (-4.0));
            s.store_sqrt_square_offset(845, 846, 0.0001);
            s.store_scaled_add(847, 846, 845, 0.5);
            s.store_scale(998, 998, 2.0);
            s.store_div_scaled_inputs2_indices(843, 875, 1.0, 847, 1.0, 998, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_offset(844, 859, 1.0);
            s.store_div_from_scalar(999, (p.p58 * 1.9e-9), 844);
            s.store_div(1001, 417, 999);
            s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1001, 1.0);
            s.store_mul(1002, 843, 1001);
            s.store_div_scaled_product_indices(1003, 842, 1002, 1.0, 997, 1.0);
            s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);
        }

        s.b[1445] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_add_scaled_inputs3_offset_indices(846, 829, 4.0, 1126, ((-1.0) * 4.0), 942, (-4.0), (p.p1033 * 4.0));
            s.store_sqrt_square_offset(845, 846, 0.0001);
            s.store_scaled_add(847, 846, 845, 0.5);
            s.store_div_scaled_inputs2_indices(843, 1118, 1.0, 847, 1.0, 998, 1.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_offset(844, 859, 1.0);
            s.store_div_from_scalar(1131, (p.p58 * 1.9e-9), 844);
            s.store_div(1132, 417, 1131);
            s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1132, 1.0);
            s.store_mul(1133, 843, 1132);
            s.store_div_scaled_product_indices(1134, 1115, 1133, 1.0, 997, 1.0);
            s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_sub(844, 875, 1004);
            s.store_mul(894, 861, 333);
            s.store_div(891, 844, 894);
            s.store_offset_sub(814, 891, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(845, A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(846, 843, 845);
            s.store_mul_sub_ad_rhs(915, 1003, s.ad_value(844), A::mul_sub_from_scalar_rhs(s.ad_value(843), 0.5, s.ad_value(846)));
            s.copy_ad(916, 915);
        }

        s.b[1446] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {
            s.store_sub(855, 1118, 1136);
            s.store_div(1129, 855, 894);
            s.store_offset_sub(814, 1129, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(1121, 814, 1.0, 1129, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 1121, (-0.5));
            s.store_mul(1121, 894, 1130);
            s.store_scaled_offset_ad(1122, A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);
            s.store_div(846, 1121, 1122);
            s.store_mul_sub_ad_rhs(850, 1134, s.ad_value(855), A::mul_sub_from_scalar_rhs(s.ad_value(1121), 0.5, s.ad_value(846)));
            s.store_add(915, 915, 850);
            s.copy_ad(916, 915);
        }

        s.b[1447] = (s.v[37] == 2.0);
        s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1447]) {
            s.store_scalar(1006, 0.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) {
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_mul_ad_product_rhs_mixed_ia(1006, 982, 850, A::sub_scaled_inputs(s.ad_value(877), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(877), 1.0, s.ad_value(845), 1.0), 1.0));
        }

        s.b[1448] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) && s.b[1448]) {
            s.store_mul_ad_product_rhs_mixed_ia(1138, 1135, 850, A::sub_scaled_inputs(s.ad_value(1130), 0.5, A::div_scaled_product(s.ad_value(1121), s.ad_value(1130), 1.0, s.ad_value(1122), 1.0), 1.0));
            s.store_add(1006, 1006, 1138);
        }

        s.b[1449] = (p.p129 > 0.5);
        s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1449]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 1003, s.ad_value(844), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 0.5, s.ad_value(845), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1450] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && s.b[1449]) && s.b[1450]) {
            s.store_mul_add_scaled_inputs4_rhs(1137, 1134, s.ad_value(1118), ((0.5) * (-1.0)), s.ad_value(1136), (((-0.5)) * (-1.0)), s.ad_value(1121), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 0.5, s.ad_value(1122), 1.0), ((-1.0) * (-1.0)));
            s.store_add(919, 919, 1137);
        }

        s.b[1451] = (p.p129 < 0.5);
        s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) {
            s.store_scale(845, 845, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(846, 1003, 0.5, 845, 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 844, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(844), A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 846, 847);
        }

        s.b[1452] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) && s.b[1452]) {
            s.store_scale(1122, 1122, 0.08333333333333333);
            s.store_div_scaled_inputs_square_rhs(846, 1134, 0.5, 1122, 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 855, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(855), A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 846, 847);
            s.store_add(919, 919, 1137);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && (!s.b[1451])) {
            s.store_scale(919, 916, (-0.5));
        }

        s.b[1453] = (s.v[37] == 2.0);
        s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });

        if (((!s.b[1402]) && s.b[1420]) && s.b[1453]) {
            s.store_scalar(939, 0.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1453])) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
            s.store_mul_sub_rhs(939, 914, 902, 824);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs4_indices(916, 916, 1.0, 938, 1.0, 937, 1.0, 1006, -1.0);
            s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);
            s.copy_ad(920, 939);
            s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 917, (-1.0), 920, (-1.0), 919, (-1.0));
        }

        if ((!s.b[1402]) && (!s.b[1420])) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
            s.store_scalar(920, 0.0);
            s.store_scalar(917, 0.0);
            s.store_scalar(919, 0.0);
            s.store_scalar(918, 0.0);
            s.store_scalar(916, 0.0);
        }

        s.b[1454] = (s.v[37] == 2.0);
        s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });

        if s.b[1454] {
            s.store_scalar(909, 0.0);
            s.store_scalar(910, 0.0);
        }

        if (!s.b[1454]) {
            s.copy_ad(815, 48);
            s.store_scalar(980, (-p.p363));
            s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);
            s.store_scalar(816, p.p183);
            s.store_scalar(976, ((((p.p185 * s.v[350]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(979, 976, p.p362);
            s.store_add_scaled_product_right_sub(976, 976, 1.0, 979, 409, 429, 1.0);
            s.store_scalar(977, ((((p.p186 * s.v[349]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(978, 977, p.p364);
            s.store_add_scaled_product_right_sub(977, 977, 1.0, 978, 409, 429, 1.0);
            s.store_scale(994, 815, 0.9);
        }

        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1087] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1087)
                }
            }, 815);
        }

        s.b[1455] = (s.v[816] == 0.5);
        s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });

        if ((!s.b[1454]) && s.b[1455]) {
            s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));
        }

        if ((!s.b[1454]) && (!s.b[1455])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[1454]) {
            s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.b[1456] = (s.v[1087] > s.v[994]);
        s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });

        if ((!s.b[1454]) && s.b[1456]) {
            s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1087, 994, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(910, 987, (p.p351 * p.p3), 976, 846, 1.0);
            s.copy_ad(815, 41);
            s.store_scalar(980, (-p.p365));
            s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);
            s.store_scalar(816, p.p184);
            s.store_scale(994, 815, 0.9);
        }

        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1088] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1088)
                }
            }, 815);
        }

        s.b[1457] = (s.v[816] == 0.5);
        s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });

        if ((!s.b[1454]) && s.b[1457]) {
            s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));
        }

        if ((!s.b[1454]) && (!s.b[1457])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1454]) {
            s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.b[1458] = (s.v[1088] > s.v[994]);
        s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });

        if ((!s.b[1454]) && s.b[1458]) {
            s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1088, 994, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(909, 988, (p.p351 * p.p3), 977, 846, 1.0);
        }

        s.store_scale(853, 897, (-p.p37));

        s.store_scaled_sub(854, 819, 897, p.p37);

        s.b[1459] = (s.v[43] != 0.0);
        s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });

        s.b[1460] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });

        s.b[1461] = (s.v[853] < s.v[322]);
        s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });

        if ((s.b[1459] && s.b[1460]) && s.b[1461]) {
            s.store_scaled_sub(86, 853, 322, s.v[52]);
        }

        s.b[1462] = (s.v[853] < s.v[175]);
        s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });

        if (((s.b[1459] && s.b[1460]) && (!s.b[1461])) && s.b[1462]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(86, 843, s.v[52], A::mul_scaled_lhs(s.ad_value(176), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1463] = (s.v[853] < s.v[323]);
        s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && (!s.b[1463])) {
            s.store_add_scaled_product_indices(86, 56, 1.0, 53, 853, 1.0);
        }

        s.b[1464] = (s.v[853] < s.v[323]);
        s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });

        if ((s.b[1459] && (!s.b[1460])) && s.b[1464]) {
            s.store_mul_sub_rhs(86, 53, 853, 323);
        }

        s.b[1465] = (s.v[853] < s.v[175]);
        s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });

        if (((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && s.b[1465]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(86, 843, s.ad_value(53), 1.0, s.ad_value(176), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1466] = (s.v[853] < s.v[322]);
        s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && s.b[1466]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_add_scaled_inputs3_mixed_iia(86, 853, s.v[52], 56, 1.0, A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && (!s.b[1466])) {
            s.store_add_scaled_inputs(86, 853, s.v[52], 56, 1.0);
        }

        s.b[1467] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });

        s.b[1468] = (s.v[854] < s.v[322]);
        s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });

        if ((s.b[1459] && s.b[1467]) && s.b[1468]) {
            s.store_scaled_sub(87, 854, 322, s.v[54]);
        }

        s.b[1469] = (s.v[854] < s.v[175]);
        s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });

        if (((s.b[1459] && s.b[1467]) && (!s.b[1468])) && s.b[1469]) {
            s.store_sub(843, 854, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(87, 843, s.v[54], A::mul_scaled_lhs(s.ad_value(178), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1470] = (s.v[854] < s.v[323]);
        s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && s.b[1470]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_add_ad(87, A::add_scaled_product(s.ad_value(57), 1.0, s.ad_value(55), s.ad_value(854), 1.0), A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && (!s.b[1470])) {
            s.store_add_scaled_product_indices(87, 57, 1.0, 55, 854, 1.0);
        }

        s.b[1471] = (s.v[854] < s.v[323]);
        s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });

        if ((s.b[1459] && (!s.b[1467])) && s.b[1471]) {
            s.store_mul_sub_rhs(87, 55, 854, 323);
        }

        s.b[1472] = (s.v[854] < s.v[175]);
        s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });

        if (((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(87, 843, s.ad_value(55), 1.0, s.ad_value(178), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1473] = (s.v[854] < s.v[322]);
        s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });

        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {
            s.store_sub(843, 854, 322);
            s.store_square(844, 843);
            s.store_add_scaled_inputs3_mixed_iia(87, 854, s.v[54], 57, 1.0, A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {
            s.store_add_scaled_inputs(87, 854, s.v[54], 57, 1.0);
        }

        if (!s.b[1459]) {
            s.store_scale(86, 853, s.v[52]);
            s.store_scale(87, 854, s.v[54]);
        }

        s.store_add_scaled_product_indices(86, 86, 1.0, 58, 853, 1.0);

        s.store_add_scaled_product_indices(87, 87, 1.0, 59, 854, 1.0);

        s.b[1474] = (p.p39 == 3.0);
        s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });

        if s.b[1474] {
            s.store_offset(843, 1019, 0.02);
        }

        if (!s.b[1474]) {
            s.store_offset(843, 820, 0.02);
        }

        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 237, s.v[349]);

        s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));

        s.b[1475] = (p.p39 == 3.0);
        s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });

        if s.b[1475] {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1475]) {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1476] = (p.p39 == 3.0);
        s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });

        if s.b[1476] {
            s.store_offset(843, 1018, 0.02);
        }

        if (!s.b[1476]) {
            s.store_offset(843, 821, 0.02);
        }

        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 236, s.v[350]);

        s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));

        s.b[1477] = (p.p39 == 3.0);
        s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });

        if s.b[1477] {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1477]) {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1478] = (p.p3 != 1.0);
        s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });

        if s.b[1478] {
            s.store_scale(895, 895, p.p3);
            s.store_scale(896, 896, p.p3);
        }

        s.b[1505] = (p.p223 == 0.0);
        s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });

        s.b[1506] = (p.p223 == 1.0);
        s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });

        s.b[1507] = (p.p223 == 2.0);
        s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });

        s.b[1508] = (p.p223 == 3.0);
        s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });

        if (s.b[1506] && (!s.b[1505])) {
            s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);
            s.store_square(843, 843);
            s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);
            s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);
            s.store_square(848, 848);
            s.store_offset_scaled(1487, 848, (((p.p227 * s.v[327])) * (p.p229)), p.p229);
            s.store_add_scaled_product_right_ad(844, 84, 1.0, 1487, A::add(s.ad_value(83), s.ad_value(85)), 1.0);
            s.store_div_scaled_product_indices(845, 844, 844, 1.0, 78, 1.0);
        }

        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
            s.store_sub_from_scalar_scaled_mul(1491, 1.0, 77, 76, 1.0);
            s.store_sub_from_scalar(843, 1.0, 1491);
            s.store_offset(844, 1491, 1.0);
            s.store_add_ad_rhs(845, 844, A::div_scaled_product_offset_denominator(s.ad_value(74), s.ad_value(49), 2.0, s.ad_value(72), 1e-10, 1.0));
            s.store_offset_scaled_div(1495, 77, 838, s.v[892], s.v[892]);
            s.store_div_from_scalar(849, s.v[892], 1495);
            s.store_square(846, 845);
            s.store_square(847, 843);
            s.store_square(848, 846);
            s.store_div(850, 843, 845);
            s.store_div(851, 72, 838);
            s.store_square(851, 851);
            s.store_offset_scaled(1487, 851, (((p.p227 * s.v[892])) * (p.p229)), p.p229);
            s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));
        }

        s.b[1548] = (s.v[398] > 0.0);
        s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });

        if s.b[1548] {
            s.store_scale(92, 918, p.p37);
            s.store_scale(93, 919, p.p37);
        }

        if (!s.b[1548]) {
            s.store_scale(93, 918, p.p37);
            s.store_scale(92, 919, p.p37);
        }

        s.b[1553] = (p.p39 == 3.0);
        s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });

        s.b[1559] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });

        s.b[1560] = ((p.p35 != 0.0) && (!true));
        s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });

        s.b[1561] = true;
        s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });

        s.b[1562] = true;
        s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });

        s.b[1563] = (p.p430 == 2.0);
        s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });

        s.b[1564] = (p.p430 == 2.0);
        s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });

        s.copy_ad(426, 916);

        s.copy_ad(427, 918);

        s.copy_ad(428, 919);

        s.store_add(425, 896, 895);

        s.store_sub(918, 427, 895);

        s.store_sub(919, 428, 896);

        s.store_add(916, 426, 425);

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_b4soiigidl: f64,
        var_b4soiigidl_dn10: f64,
        var_b4soiigidl_dn11: f64,
        var_b4soiigidl_dn12: f64,
        var_b4soiigidl_dn3: f64,
        var_b4soiigidl_dn4: f64,
        var_b4soiigidl_dn5: f64,
        var_b4soiigidl_dn6: f64,
        var_b4soiigidl_dn7: f64,
        var_b4soiigidl_dn8: f64,
        var_b4soiigidl_dn9: f64,
        var_b4soiigisl: f64,
        var_b4soiigisl_dn10: f64,
        var_b4soiigisl_dn11: f64,
        var_b4soiigisl_dn12: f64,
        var_b4soiigisl_dn3: f64,
        var_b4soiigisl_dn4: f64,
        var_b4soiigisl_dn5: f64,
        var_b4soiigisl_dn6: f64,
        var_b4soiigisl_dn7: f64,
        var_b4soiigisl_dn8: f64,
        var_b4soiigisl_dn9: f64,
        var_c0: f64,
        var_c0_dn10: f64,
        var_c0_dn11: f64,
        var_c0_dn12: f64,
        var_c0_dn3: f64,
        var_c0_dn4: f64,
        var_c0_dn5: f64,
        var_c0_dn6: f64,
        var_c0_dn7: f64,
        var_c0_dn8: f64,
        var_c0_dn9: f64,
        var_ctnoi: f64,
        var_ctnoi_dn10: f64,
        var_ctnoi_dn11: f64,
        var_ctnoi_dn12: f64,
        var_ctnoi_dn3: f64,
        var_ctnoi_dn4: f64,
        var_ctnoi_dn5: f64,
        var_ctnoi_dn6: f64,
        var_ctnoi_dn7: f64,
        var_ctnoi_dn8: f64,
        var_ctnoi_dn9: f64,
        var_guard1470: f64,
        var_guard1471: f64,
        var_guard1472: f64,
        var_guard1473: f64,
        var_guard1511: f64,
        var_guard1512: f64,
        var_guard1513: f64,
        var_ic_1: f64,
        var_ic_1_dn10: f64,
        var_ic_1_dn11: f64,
        var_ic_1_dn12: f64,
        var_ic_1_dn3: f64,
        var_ic_1_dn4: f64,
        var_ic_1_dn5: f64,
        var_ic_1_dn6: f64,
        var_ic_1_dn7: f64,
        var_ic_1_dn8: f64,
        var_ic_1_dn9: f64,
        var_ids_1: f64,
        var_ids_1_dn10: f64,
        var_ids_1_dn11: f64,
        var_ids_1_dn12: f64,
        var_ids_1_dn3: f64,
        var_ids_1_dn4: f64,
        var_ids_1_dn5: f64,
        var_ids_1_dn6: f64,
        var_ids_1_dn7: f64,
        var_ids_1_dn8: f64,
        var_ids_1_dn9: f64,
        var_iii: f64,
        var_iii_dn10: f64,
        var_iii_dn11: f64,
        var_iii_dn12: f64,
        var_iii_dn3: f64,
        var_iii_dn4: f64,
        var_iii_dn5: f64,
        var_iii_dn6: f64,
        var_iii_dn7: f64,
        var_iii_dn8: f64,
        var_iii_dn9: f64,
        var_rd: f64,
        var_rd_dn10: f64,
        var_rd_dn11: f64,
        var_rd_dn12: f64,
        var_rd_dn3: f64,
        var_rd_dn4: f64,
        var_rd_dn5: f64,
        var_rd_dn6: f64,
        var_rd_dn7: f64,
        var_rd_dn8: f64,
        var_rd_dn9: f64,
        var_rs: f64,
        var_rs_dn10: f64,
        var_rs_dn11: f64,
        var_rs_dn12: f64,
        var_rs_dn3: f64,
        var_rs_dn4: f64,
        var_rs_dn5: f64,
        var_rs_dn6: f64,
        var_rs_dn7: f64,
        var_rs_dn8: f64,
        var_rs_dn9: f64,
        var_sf: f64,
        var_sf_dn10: f64,
        var_sf_dn11: f64,
        var_sf_dn12: f64,
        var_sf_dn3: f64,
        var_sf_dn4: f64,
        var_sf_dn5: f64,
        var_sf_dn6: f64,
        var_sf_dn7: f64,
        var_sf_dn8: f64,
        var_sf_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq11_e1314, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13,) = {
    if ((var_guard1473 != 0.0) && (!(((var_guard1470 != 0.0) || (var_guard1471 != 0.0)) || (var_guard1472 != 0.0)))) {
        let eq11_e1308: f64 = (p.p32 * (nv13 - 0.0));
        let eq11_e1310: f64 = (eq11_e1308 * var_sf);
        let eq11_e1310_d_n3: f64 = (eq11_e1308 * var_sf_dn3);
        let eq11_e1310_d_n4: f64 = (eq11_e1308 * var_sf_dn4);
        let eq11_e1310_d_n5: f64 = (eq11_e1308 * var_sf_dn5);
        let eq11_e1310_d_n6: f64 = (eq11_e1308 * var_sf_dn6);
        let eq11_e1310_d_n7: f64 = (eq11_e1308 * var_sf_dn7);
        let eq11_e1310_d_n8: f64 = (eq11_e1308 * var_sf_dn8);
        let eq11_e1310_d_n9: f64 = (eq11_e1308 * var_sf_dn9);
        let eq11_e1310_d_n10: f64 = (eq11_e1308 * var_sf_dn10);
        let eq11_e1310_d_n11: f64 = (eq11_e1308 * var_sf_dn11);
        let eq11_e1310_d_n12: f64 = (eq11_e1308 * var_sf_dn12);
        let eq11_e1310_d_n13: f64 = (p.p32 * var_sf);
        let eq11_e1312: f64 = (eq11_e1310 * p.p226);
        let eq11_e1312_d_n3: f64 = (eq11_e1310_d_n3 * p.p226);
        let eq11_e1312_d_n4: f64 = (eq11_e1310_d_n4 * p.p226);
        let eq11_e1312_d_n5: f64 = (eq11_e1310_d_n5 * p.p226);
        let eq11_e1312_d_n6: f64 = (eq11_e1310_d_n6 * p.p226);
        let eq11_e1312_d_n7: f64 = (eq11_e1310_d_n7 * p.p226);
        let eq11_e1312_d_n8: f64 = (eq11_e1310_d_n8 * p.p226);
        let eq11_e1312_d_n9: f64 = (eq11_e1310_d_n9 * p.p226);
        let eq11_e1312_d_n10: f64 = (eq11_e1310_d_n10 * p.p226);
        let eq11_e1312_d_n11: f64 = (eq11_e1310_d_n11 * p.p226);
        let eq11_e1312_d_n12: f64 = (eq11_e1310_d_n12 * p.p226);
        let eq11_e1312_d_n13: f64 = (eq11_e1310_d_n13 * p.p226);
        (eq11_e1312, eq11_e1312_d_n3, eq11_e1312_d_n4, eq11_e1312_d_n5, eq11_e1312_d_n6, eq11_e1312_d_n7, eq11_e1312_d_n8, eq11_e1312_d_n9, eq11_e1312_d_n10, eq11_e1312_d_n11, eq11_e1312_d_n12, eq11_e1312_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1314;
        let eq11_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq11_node_derivatives: [f64; 11] = [eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1356, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13,) = {
    if ((var_guard1473 != 0.0) && (!(((var_guard1470 != 0.0) || (var_guard1471 != 0.0)) || (var_guard1472 != 0.0)))) {
        let eq13_e1348: f64 = (p.p32 * var_ctnoi);
        let eq13_e1348_d_n3: f64 = (p.p32 * var_ctnoi_dn3);
        let eq13_e1348_d_n4: f64 = (p.p32 * var_ctnoi_dn4);
        let eq13_e1348_d_n5: f64 = (p.p32 * var_ctnoi_dn5);
        let eq13_e1348_d_n6: f64 = (p.p32 * var_ctnoi_dn6);
        let eq13_e1348_d_n7: f64 = (p.p32 * var_ctnoi_dn7);
        let eq13_e1348_d_n8: f64 = (p.p32 * var_ctnoi_dn8);
        let eq13_e1348_d_n9: f64 = (p.p32 * var_ctnoi_dn9);
        let eq13_e1348_d_n10: f64 = (p.p32 * var_ctnoi_dn10);
        let eq13_e1348_d_n11: f64 = (p.p32 * var_ctnoi_dn11);
        let eq13_e1348_d_n12: f64 = (p.p32 * var_ctnoi_dn12);
        let eq13_e1350: f64 = (eq13_e1348 * (nv13 - 0.0));
        let eq13_e1350_d_n3: f64 = (eq13_e1348_d_n3 * (nv13 - 0.0));
        let eq13_e1350_d_n4: f64 = (eq13_e1348_d_n4 * (nv13 - 0.0));
        let eq13_e1350_d_n5: f64 = (eq13_e1348_d_n5 * (nv13 - 0.0));
        let eq13_e1350_d_n6: f64 = (eq13_e1348_d_n6 * (nv13 - 0.0));
        let eq13_e1350_d_n7: f64 = (eq13_e1348_d_n7 * (nv13 - 0.0));
        let eq13_e1350_d_n8: f64 = (eq13_e1348_d_n8 * (nv13 - 0.0));
        let eq13_e1350_d_n9: f64 = (eq13_e1348_d_n9 * (nv13 - 0.0));
        let eq13_e1350_d_n10: f64 = (eq13_e1348_d_n10 * (nv13 - 0.0));
        let eq13_e1350_d_n11: f64 = (eq13_e1348_d_n11 * (nv13 - 0.0));
        let eq13_e1350_d_n12: f64 = (eq13_e1348_d_n12 * (nv13 - 0.0));
        let eq13_e1352: f64 = (eq13_e1350 * var_sf);
        let eq13_e1352_d_n3: f64 = ((eq13_e1350_d_n3 * var_sf) + (eq13_e1350 * var_sf_dn3));
        let eq13_e1352_d_n4: f64 = ((eq13_e1350_d_n4 * var_sf) + (eq13_e1350 * var_sf_dn4));
        let eq13_e1352_d_n5: f64 = ((eq13_e1350_d_n5 * var_sf) + (eq13_e1350 * var_sf_dn5));
        let eq13_e1352_d_n6: f64 = ((eq13_e1350_d_n6 * var_sf) + (eq13_e1350 * var_sf_dn6));
        let eq13_e1352_d_n7: f64 = ((eq13_e1350_d_n7 * var_sf) + (eq13_e1350 * var_sf_dn7));
        let eq13_e1352_d_n8: f64 = ((eq13_e1350_d_n8 * var_sf) + (eq13_e1350 * var_sf_dn8));
        let eq13_e1352_d_n9: f64 = ((eq13_e1350_d_n9 * var_sf) + (eq13_e1350 * var_sf_dn9));
        let eq13_e1352_d_n10: f64 = ((eq13_e1350_d_n10 * var_sf) + (eq13_e1350 * var_sf_dn10));
        let eq13_e1352_d_n11: f64 = ((eq13_e1350_d_n11 * var_sf) + (eq13_e1350 * var_sf_dn11));
        let eq13_e1352_d_n12: f64 = ((eq13_e1350_d_n12 * var_sf) + (eq13_e1350 * var_sf_dn12));
        let eq13_e1352_d_n13: f64 = (eq13_e1348 * var_sf);
        let eq13_e1354: f64 = (eq13_e1352 * p.p226);
        let eq13_e1354_d_n3: f64 = (eq13_e1352_d_n3 * p.p226);
        let eq13_e1354_d_n4: f64 = (eq13_e1352_d_n4 * p.p226);
        let eq13_e1354_d_n5: f64 = (eq13_e1352_d_n5 * p.p226);
        let eq13_e1354_d_n6: f64 = (eq13_e1352_d_n6 * p.p226);
        let eq13_e1354_d_n7: f64 = (eq13_e1352_d_n7 * p.p226);
        let eq13_e1354_d_n8: f64 = (eq13_e1352_d_n8 * p.p226);
        let eq13_e1354_d_n9: f64 = (eq13_e1352_d_n9 * p.p226);
        let eq13_e1354_d_n10: f64 = (eq13_e1352_d_n10 * p.p226);
        let eq13_e1354_d_n11: f64 = (eq13_e1352_d_n11 * p.p226);
        let eq13_e1354_d_n12: f64 = (eq13_e1352_d_n12 * p.p226);
        let eq13_e1354_d_n13: f64 = (eq13_e1352_d_n13 * p.p226);
        (eq13_e1354, eq13_e1354_d_n3, eq13_e1354_d_n4, eq13_e1354_d_n5, eq13_e1354_d_n6, eq13_e1354_d_n7, eq13_e1354_d_n8, eq13_e1354_d_n9, eq13_e1354_d_n10, eq13_e1354_d_n11, eq13_e1354_d_n12, eq13_e1354_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1356;
        let eq13_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq13_node_derivatives: [f64; 11] = [eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e1376, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13,) = {
    if ((var_guard1473 != 0.0) && (!(((var_guard1470 != 0.0) || (var_guard1471 != 0.0)) || (var_guard1472 != 0.0)))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * var_c0);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * var_c0_dn3);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * var_c0_dn4);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * var_c0_dn5);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * var_c0_dn6);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * var_c0_dn7);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * var_c0_dn8);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * var_c0_dn9);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * var_c0_dn10);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * var_c0_dn11);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * var_c0_dn12);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq14_e1373);
        (eq14_e1374, (eq14_e1373_d_n3 * ddt_scale), (eq14_e1373_d_n4 * ddt_scale), (eq14_e1373_d_n5 * ddt_scale), (eq14_e1373_d_n6 * ddt_scale), (eq14_e1373_d_n7 * ddt_scale), (eq14_e1373_d_n8 * ddt_scale), (eq14_e1373_d_n9 * ddt_scale), (eq14_e1373_d_n10 * ddt_scale), (eq14_e1373_d_n11 * ddt_scale), (eq14_e1373_d_n12 * ddt_scale), (eq14_e1371 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e1376;
        let eq14_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq14_node_derivatives: [f64; 11] = [eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13,) = {
    if ((var_guard1473 != 0.0) && (!(((var_guard1470 != 0.0) || (var_guard1471 != 0.0)) || (var_guard1472 != 0.0)))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * var_c0);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * var_c0_dn3);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * var_c0_dn4);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * var_c0_dn5);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * var_c0_dn6);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * var_c0_dn7);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * var_c0_dn8);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * var_c0_dn9);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * var_c0_dn10);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * var_c0_dn11);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * var_c0_dn12);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq15_e1393);
        (eq15_e1394, (eq15_e1393_d_n3 * ddt_scale), (eq15_e1393_d_n4 * ddt_scale), (eq15_e1393_d_n5 * ddt_scale), (eq15_e1393_d_n6 * ddt_scale), (eq15_e1393_d_n7 * ddt_scale), (eq15_e1393_d_n8 * ddt_scale), (eq15_e1393_d_n9 * ddt_scale), (eq15_e1393_d_n10 * ddt_scale), (eq15_e1393_d_n11 * ddt_scale), (eq15_e1393_d_n12 * ddt_scale), (eq15_e1391 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1396;
        let eq15_node_derivative_indices: [usize; 11] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let eq15_node_derivatives: [f64; 11] = [eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1414, eq18_e1414_d_n0, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12,) = {
    if (var_guard1511 != 0.0) {
        let eq18_e1410: f64 = (p.p32 * (nv0 - nv7));
        let __rspice_inv_cse_0: f64 = 1.0 / var_rd;
        let eq18_e1412: f64 = (eq18_e1410 * __rspice_inv_cse_0);
        let eq18_e1412_d_n0: f64 = (p.p32 * __rspice_inv_cse_0);
        let eq18_e1412_d_n3: f64 = (-((eq18_e1410 * var_rd_dn3) / (var_rd * var_rd)));
        let eq18_e1412_d_n4: f64 = (-((eq18_e1410 * var_rd_dn4) / (var_rd * var_rd)));
        let eq18_e1412_d_n5: f64 = (-((eq18_e1410 * var_rd_dn5) / (var_rd * var_rd)));
        let eq18_e1412_d_n6: f64 = (-((eq18_e1410 * var_rd_dn6) / (var_rd * var_rd)));
        let eq18_e1412_d_n7: f64 = ((((-p.p32) * var_rd) - (eq18_e1410 * var_rd_dn7)) / (var_rd * var_rd));
        let eq18_e1412_d_n8: f64 = (-((eq18_e1410 * var_rd_dn8) / (var_rd * var_rd)));
        let eq18_e1412_d_n9: f64 = (-((eq18_e1410 * var_rd_dn9) / (var_rd * var_rd)));
        let eq18_e1412_d_n10: f64 = (-((eq18_e1410 * var_rd_dn10) / (var_rd * var_rd)));
        let eq18_e1412_d_n11: f64 = (-((eq18_e1410 * var_rd_dn11) / (var_rd * var_rd)));
        let eq18_e1412_d_n12: f64 = (-((eq18_e1410 * var_rd_dn12) / (var_rd * var_rd)));
        (eq18_e1412, eq18_e1412_d_n0, eq18_e1412_d_n3, eq18_e1412_d_n4, eq18_e1412_d_n5, eq18_e1412_d_n6, eq18_e1412_d_n7, eq18_e1412_d_n8, eq18_e1412_d_n9, eq18_e1412_d_n10, eq18_e1412_d_n11, eq18_e1412_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1414;
        let eq18_node_derivative_indices: [usize; 11] = [0, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq18_node_derivatives: [f64; 11] = [eq18_e1414_d_n0, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1438, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12,) = {
    if (var_guard1512 != 0.0) {
        let eq21_e1434: f64 = (p.p32 * (nv2 - nv8));
        let __rspice_inv_cse_1: f64 = 1.0 / var_rs;
        let eq21_e1436: f64 = (eq21_e1434 * __rspice_inv_cse_1);
        let eq21_e1436_d_n2: f64 = (p.p32 * __rspice_inv_cse_1);
        let eq21_e1436_d_n3: f64 = (-((eq21_e1434 * var_rs_dn3) / (var_rs * var_rs)));
        let eq21_e1436_d_n4: f64 = (-((eq21_e1434 * var_rs_dn4) / (var_rs * var_rs)));
        let eq21_e1436_d_n5: f64 = (-((eq21_e1434 * var_rs_dn5) / (var_rs * var_rs)));
        let eq21_e1436_d_n6: f64 = (-((eq21_e1434 * var_rs_dn6) / (var_rs * var_rs)));
        let eq21_e1436_d_n7: f64 = (-((eq21_e1434 * var_rs_dn7) / (var_rs * var_rs)));
        let eq21_e1436_d_n8: f64 = ((((-p.p32) * var_rs) - (eq21_e1434 * var_rs_dn8)) / (var_rs * var_rs));
        let eq21_e1436_d_n9: f64 = (-((eq21_e1434 * var_rs_dn9) / (var_rs * var_rs)));
        let eq21_e1436_d_n10: f64 = (-((eq21_e1434 * var_rs_dn10) / (var_rs * var_rs)));
        let eq21_e1436_d_n11: f64 = (-((eq21_e1434 * var_rs_dn11) / (var_rs * var_rs)));
        let eq21_e1436_d_n12: f64 = (-((eq21_e1434 * var_rs_dn12) / (var_rs * var_rs)));
        (eq21_e1436, eq21_e1436_d_n2, eq21_e1436_d_n3, eq21_e1436_d_n4, eq21_e1436_d_n5, eq21_e1436_d_n6, eq21_e1436_d_n7, eq21_e1436_d_n8, eq21_e1436_d_n9, eq21_e1436_d_n10, eq21_e1436_d_n11, eq21_e1436_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1438;
        let eq21_node_derivative_indices: [usize; 11] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq21_node_derivatives: [f64; 11] = [eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1472, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12,) = {
    if (var_guard1513 != 0.0) {
        let eq24_e1458: f64 = (p.p37 * p.p32);
        let eq24_e1461: f64 = (var_ids_1 + var_ic_1);
        let eq24_e1461_d_n3: f64 = (var_ids_1_dn3 + var_ic_1_dn3);
        let eq24_e1461_d_n4: f64 = (var_ids_1_dn4 + var_ic_1_dn4);
        let eq24_e1461_d_n5: f64 = (var_ids_1_dn5 + var_ic_1_dn5);
        let eq24_e1461_d_n6: f64 = (var_ids_1_dn6 + var_ic_1_dn6);
        let eq24_e1461_d_n7: f64 = (var_ids_1_dn7 + var_ic_1_dn7);
        let eq24_e1461_d_n8: f64 = (var_ids_1_dn8 + var_ic_1_dn8);
        let eq24_e1461_d_n9: f64 = (var_ids_1_dn9 + var_ic_1_dn9);
        let eq24_e1461_d_n10: f64 = (var_ids_1_dn10 + var_ic_1_dn10);
        let eq24_e1461_d_n11: f64 = (var_ids_1_dn11 + var_ic_1_dn11);
        let eq24_e1461_d_n12: f64 = (var_ids_1_dn12 + var_ic_1_dn12);
        let eq24_e1462: f64 = (eq24_e1458 * eq24_e1461);
        let eq24_e1462_d_n3: f64 = (eq24_e1458 * eq24_e1461_d_n3);
        let eq24_e1462_d_n4: f64 = (eq24_e1458 * eq24_e1461_d_n4);
        let eq24_e1462_d_n5: f64 = (eq24_e1458 * eq24_e1461_d_n5);
        let eq24_e1462_d_n6: f64 = (eq24_e1458 * eq24_e1461_d_n6);
        let eq24_e1462_d_n7: f64 = (eq24_e1458 * eq24_e1461_d_n7);
        let eq24_e1462_d_n8: f64 = (eq24_e1458 * eq24_e1461_d_n8);
        let eq24_e1462_d_n9: f64 = (eq24_e1458 * eq24_e1461_d_n9);
        let eq24_e1462_d_n10: f64 = (eq24_e1458 * eq24_e1461_d_n10);
        let eq24_e1462_d_n11: f64 = (eq24_e1458 * eq24_e1461_d_n11);
        let eq24_e1462_d_n12: f64 = (eq24_e1458 * eq24_e1461_d_n12);
        let eq24_e1466: f64 = 0.0;
        let eq24_e1468: f64 = (eq24_e1466 * (nv7 - nv8));
        let eq24_e1469: f64 = (p.p32 * eq24_e1468);
        let eq24_e1469_d_n7: f64 = (p.p32 * eq24_e1466);
        let eq24_e1469_d_n8: f64 = (p.p32 * (-eq24_e1466));
        let eq24_e1470: f64 = (eq24_e1462 + eq24_e1469);
        let eq24_e1470_d_n7: f64 = (eq24_e1462_d_n7 + eq24_e1469_d_n7);
        let eq24_e1470_d_n8: f64 = (eq24_e1462_d_n8 + eq24_e1469_d_n8);
        (eq24_e1470, eq24_e1462_d_n3, eq24_e1462_d_n4, eq24_e1462_d_n5, eq24_e1462_d_n6, eq24_e1470_d_n7, eq24_e1470_d_n8, eq24_e1462_d_n9, eq24_e1462_d_n10, eq24_e1462_d_n11, eq24_e1462_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1472;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq24_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq24_e1472_d_n3), multiplicity * (eq24_e1472_d_n4), multiplicity * (eq24_e1472_d_n5), multiplicity * (eq24_e1472_d_n6), multiplicity * (eq24_e1472_d_n7), multiplicity * (eq24_e1472_d_n8), multiplicity * (eq24_e1472_d_n9), multiplicity * (eq24_e1472_d_n10), multiplicity * (eq24_e1472_d_n11), multiplicity * (eq24_e1472_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq25_e1480, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12,) = {
    if (var_guard1513 != 0.0) {
        let eq25_e1476: f64 = (p.p37 * p.p32);
        let eq25_e1478: f64 = (eq25_e1476 * var_iii);
        let eq25_e1478_d_n3: f64 = (eq25_e1476 * var_iii_dn3);
        let eq25_e1478_d_n4: f64 = (eq25_e1476 * var_iii_dn4);
        let eq25_e1478_d_n5: f64 = (eq25_e1476 * var_iii_dn5);
        let eq25_e1478_d_n6: f64 = (eq25_e1476 * var_iii_dn6);
        let eq25_e1478_d_n7: f64 = (eq25_e1476 * var_iii_dn7);
        let eq25_e1478_d_n8: f64 = (eq25_e1476 * var_iii_dn8);
        let eq25_e1478_d_n9: f64 = (eq25_e1476 * var_iii_dn9);
        let eq25_e1478_d_n10: f64 = (eq25_e1476 * var_iii_dn10);
        let eq25_e1478_d_n11: f64 = (eq25_e1476 * var_iii_dn11);
        let eq25_e1478_d_n12: f64 = (eq25_e1476 * var_iii_dn12);
        (eq25_e1478, eq25_e1478_d_n3, eq25_e1478_d_n4, eq25_e1478_d_n5, eq25_e1478_d_n6, eq25_e1478_d_n7, eq25_e1478_d_n8, eq25_e1478_d_n9, eq25_e1478_d_n10, eq25_e1478_d_n11, eq25_e1478_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1480;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq25_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq25_e1480_d_n3), multiplicity * (eq25_e1480_d_n4), multiplicity * (eq25_e1480_d_n5), multiplicity * (eq25_e1480_d_n6), multiplicity * (eq25_e1480_d_n7), multiplicity * (eq25_e1480_d_n8), multiplicity * (eq25_e1480_d_n9), multiplicity * (eq25_e1480_d_n10), multiplicity * (eq25_e1480_d_n11), multiplicity * (eq25_e1480_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq26_e1499, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12,) = {
    if (var_guard1513 == 0.0) {
        let eq26_e1485: f64 = (p.p37 * p.p32);
        let eq26_e1488: f64 = (var_ids_1 - var_ic_1);
        let eq26_e1488_d_n3: f64 = (var_ids_1_dn3 - var_ic_1_dn3);
        let eq26_e1488_d_n4: f64 = (var_ids_1_dn4 - var_ic_1_dn4);
        let eq26_e1488_d_n5: f64 = (var_ids_1_dn5 - var_ic_1_dn5);
        let eq26_e1488_d_n6: f64 = (var_ids_1_dn6 - var_ic_1_dn6);
        let eq26_e1488_d_n7: f64 = (var_ids_1_dn7 - var_ic_1_dn7);
        let eq26_e1488_d_n8: f64 = (var_ids_1_dn8 - var_ic_1_dn8);
        let eq26_e1488_d_n9: f64 = (var_ids_1_dn9 - var_ic_1_dn9);
        let eq26_e1488_d_n10: f64 = (var_ids_1_dn10 - var_ic_1_dn10);
        let eq26_e1488_d_n11: f64 = (var_ids_1_dn11 - var_ic_1_dn11);
        let eq26_e1488_d_n12: f64 = (var_ids_1_dn12 - var_ic_1_dn12);
        let eq26_e1489: f64 = (eq26_e1485 * eq26_e1488);
        let eq26_e1489_d_n3: f64 = (eq26_e1485 * eq26_e1488_d_n3);
        let eq26_e1489_d_n4: f64 = (eq26_e1485 * eq26_e1488_d_n4);
        let eq26_e1489_d_n5: f64 = (eq26_e1485 * eq26_e1488_d_n5);
        let eq26_e1489_d_n6: f64 = (eq26_e1485 * eq26_e1488_d_n6);
        let eq26_e1489_d_n7: f64 = (eq26_e1485 * eq26_e1488_d_n7);
        let eq26_e1489_d_n8: f64 = (eq26_e1485 * eq26_e1488_d_n8);
        let eq26_e1489_d_n9: f64 = (eq26_e1485 * eq26_e1488_d_n9);
        let eq26_e1489_d_n10: f64 = (eq26_e1485 * eq26_e1488_d_n10);
        let eq26_e1489_d_n11: f64 = (eq26_e1485 * eq26_e1488_d_n11);
        let eq26_e1489_d_n12: f64 = (eq26_e1485 * eq26_e1488_d_n12);
        let eq26_e1493: f64 = 0.0;
        let eq26_e1495: f64 = (eq26_e1493 * (nv8 - nv7));
        let eq26_e1496: f64 = (p.p32 * eq26_e1495);
        let eq26_e1496_d_n7: f64 = (p.p32 * (-eq26_e1493));
        let eq26_e1496_d_n8: f64 = (p.p32 * eq26_e1493);
        let eq26_e1497: f64 = (eq26_e1489 + eq26_e1496);
        let eq26_e1497_d_n7: f64 = (eq26_e1489_d_n7 + eq26_e1496_d_n7);
        let eq26_e1497_d_n8: f64 = (eq26_e1489_d_n8 + eq26_e1496_d_n8);
        (eq26_e1497, eq26_e1489_d_n3, eq26_e1489_d_n4, eq26_e1489_d_n5, eq26_e1489_d_n6, eq26_e1497_d_n7, eq26_e1497_d_n8, eq26_e1489_d_n9, eq26_e1489_d_n10, eq26_e1489_d_n11, eq26_e1489_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1499;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq26_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq26_e1499_d_n3), multiplicity * (eq26_e1499_d_n4), multiplicity * (eq26_e1499_d_n5), multiplicity * (eq26_e1499_d_n6), multiplicity * (eq26_e1499_d_n7), multiplicity * (eq26_e1499_d_n8), multiplicity * (eq26_e1499_d_n9), multiplicity * (eq26_e1499_d_n10), multiplicity * (eq26_e1499_d_n11), multiplicity * (eq26_e1499_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq27_e1508, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12,) = {
    if (var_guard1513 == 0.0) {
        let eq27_e1504: f64 = (p.p37 * p.p32);
        let eq27_e1506: f64 = (eq27_e1504 * var_iii);
        let eq27_e1506_d_n3: f64 = (eq27_e1504 * var_iii_dn3);
        let eq27_e1506_d_n4: f64 = (eq27_e1504 * var_iii_dn4);
        let eq27_e1506_d_n5: f64 = (eq27_e1504 * var_iii_dn5);
        let eq27_e1506_d_n6: f64 = (eq27_e1504 * var_iii_dn6);
        let eq27_e1506_d_n7: f64 = (eq27_e1504 * var_iii_dn7);
        let eq27_e1506_d_n8: f64 = (eq27_e1504 * var_iii_dn8);
        let eq27_e1506_d_n9: f64 = (eq27_e1504 * var_iii_dn9);
        let eq27_e1506_d_n10: f64 = (eq27_e1504 * var_iii_dn10);
        let eq27_e1506_d_n11: f64 = (eq27_e1504 * var_iii_dn11);
        let eq27_e1506_d_n12: f64 = (eq27_e1504 * var_iii_dn12);
        (eq27_e1506, eq27_e1506_d_n3, eq27_e1506_d_n4, eq27_e1506_d_n5, eq27_e1506_d_n6, eq27_e1506_d_n7, eq27_e1506_d_n8, eq27_e1506_d_n9, eq27_e1506_d_n10, eq27_e1506_d_n11, eq27_e1506_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1508;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq27_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq27_e1508_d_n3), multiplicity * (eq27_e1508_d_n4), multiplicity * (eq27_e1508_d_n5), multiplicity * (eq27_e1508_d_n6), multiplicity * (eq27_e1508_d_n7), multiplicity * (eq27_e1508_d_n8), multiplicity * (eq27_e1508_d_n9), multiplicity * (eq27_e1508_d_n10), multiplicity * (eq27_e1508_d_n11), multiplicity * (eq27_e1508_d_n12)],
            [],
            [],
            1.0,
        );
        let eq28_e1511: f64 = (p.p32 * var_b4soiigidl);
        let eq28_e1511_d_n3: f64 = (p.p32 * var_b4soiigidl_dn3);
        let eq28_e1511_d_n4: f64 = (p.p32 * var_b4soiigidl_dn4);
        let eq28_e1511_d_n5: f64 = (p.p32 * var_b4soiigidl_dn5);
        let eq28_e1511_d_n6: f64 = (p.p32 * var_b4soiigidl_dn6);
        let eq28_e1511_d_n7: f64 = (p.p32 * var_b4soiigidl_dn7);
        let eq28_e1511_d_n8: f64 = (p.p32 * var_b4soiigidl_dn8);
        let eq28_e1511_d_n9: f64 = (p.p32 * var_b4soiigidl_dn9);
        let eq28_e1511_d_n10: f64 = (p.p32 * var_b4soiigidl_dn10);
        let eq28_e1511_d_n11: f64 = (p.p32 * var_b4soiigidl_dn11);
        let eq28_e1511_d_n12: f64 = (p.p32 * var_b4soiigidl_dn12);
        let eq28_value: f64 = eq28_e1511;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq28_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq28_e1511_d_n3), multiplicity * (eq28_e1511_d_n4), multiplicity * (eq28_e1511_d_n5), multiplicity * (eq28_e1511_d_n6), multiplicity * (eq28_e1511_d_n7), multiplicity * (eq28_e1511_d_n8), multiplicity * (eq28_e1511_d_n9), multiplicity * (eq28_e1511_d_n10), multiplicity * (eq28_e1511_d_n11), multiplicity * (eq28_e1511_d_n12)],
            [],
            [],
            1.0,
        );
        let eq29_e1514: f64 = (p.p32 * var_b4soiigisl);
        let eq29_e1514_d_n3: f64 = (p.p32 * var_b4soiigisl_dn3);
        let eq29_e1514_d_n4: f64 = (p.p32 * var_b4soiigisl_dn4);
        let eq29_e1514_d_n5: f64 = (p.p32 * var_b4soiigisl_dn5);
        let eq29_e1514_d_n6: f64 = (p.p32 * var_b4soiigisl_dn6);
        let eq29_e1514_d_n7: f64 = (p.p32 * var_b4soiigisl_dn7);
        let eq29_e1514_d_n8: f64 = (p.p32 * var_b4soiigisl_dn8);
        let eq29_e1514_d_n9: f64 = (p.p32 * var_b4soiigisl_dn9);
        let eq29_e1514_d_n10: f64 = (p.p32 * var_b4soiigisl_dn10);
        let eq29_e1514_d_n11: f64 = (p.p32 * var_b4soiigisl_dn11);
        let eq29_e1514_d_n12: f64 = (p.p32 * var_b4soiigisl_dn12);
        let eq29_value: f64 = eq29_e1514;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq29_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq29_e1514_d_n3), multiplicity * (eq29_e1514_d_n4), multiplicity * (eq29_e1514_d_n5), multiplicity * (eq29_e1514_d_n6), multiplicity * (eq29_e1514_d_n7), multiplicity * (eq29_e1514_d_n8), multiplicity * (eq29_e1514_d_n9), multiplicity * (eq29_e1514_d_n10), multiplicity * (eq29_e1514_d_n11), multiplicity * (eq29_e1514_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_b4soiig: f64,
        var_b4soiig_dn10: f64,
        var_b4soiig_dn11: f64,
        var_b4soiig_dn12: f64,
        var_b4soiig_dn3: f64,
        var_b4soiig_dn4: f64,
        var_b4soiig_dn5: f64,
        var_b4soiig_dn6: f64,
        var_b4soiig_dn7: f64,
        var_b4soiig_dn8: f64,
        var_b4soiig_dn9: f64,
        var_b4soiigcd: f64,
        var_b4soiigcd_dn10: f64,
        var_b4soiigcd_dn11: f64,
        var_b4soiigcd_dn12: f64,
        var_b4soiigcd_dn3: f64,
        var_b4soiigcd_dn4: f64,
        var_b4soiigcd_dn5: f64,
        var_b4soiigcd_dn6: f64,
        var_b4soiigcd_dn7: f64,
        var_b4soiigcd_dn8: f64,
        var_b4soiigcd_dn9: f64,
        var_b4soiigcs: f64,
        var_b4soiigcs_dn10: f64,
        var_b4soiigcs_dn11: f64,
        var_b4soiigcs_dn12: f64,
        var_b4soiigcs_dn3: f64,
        var_b4soiigcs_dn4: f64,
        var_b4soiigcs_dn5: f64,
        var_b4soiigcs_dn6: f64,
        var_b4soiigcs_dn7: f64,
        var_b4soiigcs_dn8: f64,
        var_b4soiigcs_dn9: f64,
        var_b4soiigd: f64,
        var_b4soiigd_dn10: f64,
        var_b4soiigd_dn11: f64,
        var_b4soiigd_dn12: f64,
        var_b4soiigd_dn3: f64,
        var_b4soiigd_dn4: f64,
        var_b4soiigd_dn5: f64,
        var_b4soiigd_dn6: f64,
        var_b4soiigd_dn7: f64,
        var_b4soiigd_dn8: f64,
        var_b4soiigd_dn9: f64,
        var_b4soiigp: f64,
        var_b4soiigp_dn10: f64,
        var_b4soiigp_dn11: f64,
        var_b4soiigp_dn12: f64,
        var_b4soiigp_dn3: f64,
        var_b4soiigp_dn4: f64,
        var_b4soiigp_dn5: f64,
        var_b4soiigp_dn6: f64,
        var_b4soiigp_dn7: f64,
        var_b4soiigp_dn8: f64,
        var_b4soiigp_dn9: f64,
        var_b4soiigs: f64,
        var_b4soiigs_dn10: f64,
        var_b4soiigs_dn11: f64,
        var_b4soiigs_dn12: f64,
        var_b4soiigs_dn3: f64,
        var_b4soiigs_dn4: f64,
        var_b4soiigs_dn5: f64,
        var_b4soiigs_dn6: f64,
        var_b4soiigs_dn7: f64,
        var_b4soiigs_dn8: f64,
        var_b4soiigs_dn9: f64,
        var_b4soiqdrn: f64,
        var_b4soiqdrn_dn10: f64,
        var_b4soiqdrn_dn11: f64,
        var_b4soiqdrn_dn12: f64,
        var_b4soiqdrn_dn3: f64,
        var_b4soiqdrn_dn4: f64,
        var_b4soiqdrn_dn5: f64,
        var_b4soiqdrn_dn6: f64,
        var_b4soiqdrn_dn7: f64,
        var_b4soiqdrn_dn8: f64,
        var_b4soiqdrn_dn9: f64,
        var_b4soiqsrc: f64,
        var_b4soiqsrc_dn10: f64,
        var_b4soiqsrc_dn11: f64,
        var_b4soiqsrc_dn12: f64,
        var_b4soiqsrc_dn3: f64,
        var_b4soiqsrc_dn4: f64,
        var_b4soiqsrc_dn5: f64,
        var_b4soiqsrc_dn6: f64,
        var_b4soiqsrc_dn7: f64,
        var_b4soiqsrc_dn8: f64,
        var_b4soiqsrc_dn9: f64,
        var_guard1517: f64,
        var_guard1518: f64,
        var_ibd_1: f64,
        var_ibd_1_dn10: f64,
        var_ibd_1_dn11: f64,
        var_ibd_1_dn12: f64,
        var_ibd_1_dn3: f64,
        var_ibd_1_dn4: f64,
        var_ibd_1_dn5: f64,
        var_ibd_1_dn6: f64,
        var_ibd_1_dn7: f64,
        var_ibd_1_dn8: f64,
        var_ibd_1_dn9: f64,
        var_ibp: f64,
        var_ibp_dn10: f64,
        var_ibp_dn11: f64,
        var_ibp_dn12: f64,
        var_ibp_dn3: f64,
        var_ibp_dn4: f64,
        var_ibp_dn5: f64,
        var_ibp_dn6: f64,
        var_ibp_dn7: f64,
        var_ibp_dn8: f64,
        var_ibp_dn9: f64,
        var_ibs_1: f64,
        var_ibs_1_dn10: f64,
        var_ibs_1_dn11: f64,
        var_ibs_1_dn12: f64,
        var_ibs_1_dn3: f64,
        var_ibs_1_dn4: f64,
        var_ibs_1_dn5: f64,
        var_ibs_1_dn6: f64,
        var_ibs_1_dn7: f64,
        var_ibs_1_dn8: f64,
        var_ibs_1_dn9: f64,
        var_pparam_b4soicgeo: f64,
        var_pparam_b4soicgeo_dn10: f64,
        var_pparam_b4soicgeo_dn11: f64,
        var_pparam_b4soicgeo_dn12: f64,
        var_pparam_b4soicgeo_dn3: f64,
        var_pparam_b4soicgeo_dn4: f64,
        var_pparam_b4soicgeo_dn5: f64,
        var_pparam_b4soicgeo_dn6: f64,
        var_pparam_b4soicgeo_dn7: f64,
        var_pparam_b4soicgeo_dn8: f64,
        var_pparam_b4soicgeo_dn9: f64,
        var_qgate: f64,
        var_qgate_dn10: f64,
        var_qgate_dn11: f64,
        var_qgate_dn12: f64,
        var_qgate_dn3: f64,
        var_qgate_dn4: f64,
        var_qgate_dn5: f64,
        var_qgate_dn6: f64,
        var_qgate_dn7: f64,
        var_qgate_dn8: f64,
        var_qgate_dn9: f64,
        var_qgdo: f64,
        var_qgdo_dn10: f64,
        var_qgdo_dn11: f64,
        var_qgdo_dn12: f64,
        var_qgdo_dn3: f64,
        var_qgdo_dn4: f64,
        var_qgdo_dn5: f64,
        var_qgdo_dn6: f64,
        var_qgdo_dn7: f64,
        var_qgdo_dn8: f64,
        var_qgdo_dn9: f64,
        var_qgso: f64,
        var_qgso_dn10: f64,
        var_qgso_dn11: f64,
        var_qgso_dn12: f64,
        var_qgso_dn3: f64,
        var_qgso_dn4: f64,
        var_qgso_dn5: f64,
        var_qgso_dn6: f64,
        var_qgso_dn7: f64,
        var_qgso_dn8: f64,
        var_qgso_dn9: f64,
        var_qjd_1: f64,
        var_qjd_1_dn10: f64,
        var_qjd_1_dn11: f64,
        var_qjd_1_dn12: f64,
        var_qjd_1_dn3: f64,
        var_qjd_1_dn4: f64,
        var_qjd_1_dn5: f64,
        var_qjd_1_dn6: f64,
        var_qjd_1_dn7: f64,
        var_qjd_1_dn8: f64,
        var_qjd_1_dn9: f64,
        var_qjs_1: f64,
        var_qjs_1_dn10: f64,
        var_qjs_1_dn11: f64,
        var_qjs_1_dn12: f64,
        var_qjs_1_dn3: f64,
        var_qjs_1_dn4: f64,
        var_qjs_1_dn5: f64,
        var_qjs_1_dn6: f64,
        var_qjs_1_dn7: f64,
        var_qjs_1_dn8: f64,
        var_qjs_1_dn9: f64,
        var_qsub: f64,
        var_qsub_dn10: f64,
        var_qsub_dn11: f64,
        var_qsub_dn12: f64,
        var_qsub_dn3: f64,
        var_qsub_dn4: f64,
        var_qsub_dn5: f64,
        var_qsub_dn6: f64,
        var_qsub_dn7: f64,
        var_qsub_dn8: f64,
        var_qsub_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq30_e1517: f64 = (p.p37 * p.p32);
        let eq30_e1519: f64 = (eq30_e1517 * var_ibd_1);
        let eq30_e1519_d_n3: f64 = (eq30_e1517 * var_ibd_1_dn3);
        let eq30_e1519_d_n4: f64 = (eq30_e1517 * var_ibd_1_dn4);
        let eq30_e1519_d_n5: f64 = (eq30_e1517 * var_ibd_1_dn5);
        let eq30_e1519_d_n6: f64 = (eq30_e1517 * var_ibd_1_dn6);
        let eq30_e1519_d_n7: f64 = (eq30_e1517 * var_ibd_1_dn7);
        let eq30_e1519_d_n8: f64 = (eq30_e1517 * var_ibd_1_dn8);
        let eq30_e1519_d_n9: f64 = (eq30_e1517 * var_ibd_1_dn9);
        let eq30_e1519_d_n10: f64 = (eq30_e1517 * var_ibd_1_dn10);
        let eq30_e1519_d_n11: f64 = (eq30_e1517 * var_ibd_1_dn11);
        let eq30_e1519_d_n12: f64 = (eq30_e1517 * var_ibd_1_dn12);
        let eq30_value: f64 = eq30_e1519;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq30_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq30_e1519_d_n3), multiplicity * (eq30_e1519_d_n4), multiplicity * (eq30_e1519_d_n5), multiplicity * (eq30_e1519_d_n6), multiplicity * (eq30_e1519_d_n7), multiplicity * (eq30_e1519_d_n8), multiplicity * (eq30_e1519_d_n9), multiplicity * (eq30_e1519_d_n10), multiplicity * (eq30_e1519_d_n11), multiplicity * (eq30_e1519_d_n12)],
            [],
            [],
            1.0,
        );
        let eq31_e1522: f64 = (p.p37 * p.p32);
        let eq31_e1524: f64 = (eq31_e1522 * var_ibs_1);
        let eq31_e1524_d_n3: f64 = (eq31_e1522 * var_ibs_1_dn3);
        let eq31_e1524_d_n4: f64 = (eq31_e1522 * var_ibs_1_dn4);
        let eq31_e1524_d_n5: f64 = (eq31_e1522 * var_ibs_1_dn5);
        let eq31_e1524_d_n6: f64 = (eq31_e1522 * var_ibs_1_dn6);
        let eq31_e1524_d_n7: f64 = (eq31_e1522 * var_ibs_1_dn7);
        let eq31_e1524_d_n8: f64 = (eq31_e1522 * var_ibs_1_dn8);
        let eq31_e1524_d_n9: f64 = (eq31_e1522 * var_ibs_1_dn9);
        let eq31_e1524_d_n10: f64 = (eq31_e1522 * var_ibs_1_dn10);
        let eq31_e1524_d_n11: f64 = (eq31_e1522 * var_ibs_1_dn11);
        let eq31_e1524_d_n12: f64 = (eq31_e1522 * var_ibs_1_dn12);
        let eq31_value: f64 = eq31_e1524;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq31_e1524_d_n3), multiplicity * (eq31_e1524_d_n4), multiplicity * (eq31_e1524_d_n5), multiplicity * (eq31_e1524_d_n6), multiplicity * (eq31_e1524_d_n7), multiplicity * (eq31_e1524_d_n8), multiplicity * (eq31_e1524_d_n9), multiplicity * (eq31_e1524_d_n10), multiplicity * (eq31_e1524_d_n11), multiplicity * (eq31_e1524_d_n12)],
            [],
            [],
            1.0,
        );
        let eq32_e1528: f64 = (var_b4soiigd + var_b4soiigcd);
        let eq32_e1528_d_n3: f64 = (var_b4soiigd_dn3 + var_b4soiigcd_dn3);
        let eq32_e1528_d_n4: f64 = (var_b4soiigd_dn4 + var_b4soiigcd_dn4);
        let eq32_e1528_d_n5: f64 = (var_b4soiigd_dn5 + var_b4soiigcd_dn5);
        let eq32_e1528_d_n6: f64 = (var_b4soiigd_dn6 + var_b4soiigcd_dn6);
        let eq32_e1528_d_n7: f64 = (var_b4soiigd_dn7 + var_b4soiigcd_dn7);
        let eq32_e1528_d_n8: f64 = (var_b4soiigd_dn8 + var_b4soiigcd_dn8);
        let eq32_e1528_d_n9: f64 = (var_b4soiigd_dn9 + var_b4soiigcd_dn9);
        let eq32_e1528_d_n10: f64 = (var_b4soiigd_dn10 + var_b4soiigcd_dn10);
        let eq32_e1528_d_n11: f64 = (var_b4soiigd_dn11 + var_b4soiigcd_dn11);
        let eq32_e1528_d_n12: f64 = (var_b4soiigd_dn12 + var_b4soiigcd_dn12);
        let eq32_e1529: f64 = (p.p32 * eq32_e1528);
        let eq32_e1529_d_n3: f64 = (p.p32 * eq32_e1528_d_n3);
        let eq32_e1529_d_n4: f64 = (p.p32 * eq32_e1528_d_n4);
        let eq32_e1529_d_n5: f64 = (p.p32 * eq32_e1528_d_n5);
        let eq32_e1529_d_n6: f64 = (p.p32 * eq32_e1528_d_n6);
        let eq32_e1529_d_n7: f64 = (p.p32 * eq32_e1528_d_n7);
        let eq32_e1529_d_n8: f64 = (p.p32 * eq32_e1528_d_n8);
        let eq32_e1529_d_n9: f64 = (p.p32 * eq32_e1528_d_n9);
        let eq32_e1529_d_n10: f64 = (p.p32 * eq32_e1528_d_n10);
        let eq32_e1529_d_n11: f64 = (p.p32 * eq32_e1528_d_n11);
        let eq32_e1529_d_n12: f64 = (p.p32 * eq32_e1528_d_n12);
        let eq32_e1533: f64 = 0.0;
        let eq32_e1535: f64 = (eq32_e1533 * (nv9 - nv7));
        let eq32_e1536: f64 = (p.p32 * eq32_e1535);
        let eq32_e1536_d_n7: f64 = (p.p32 * (-eq32_e1533));
        let eq32_e1536_d_n9: f64 = (p.p32 * eq32_e1533);
        let eq32_e1537: f64 = (eq32_e1529 + eq32_e1536);
        let eq32_e1537_d_n7: f64 = (eq32_e1529_d_n7 + eq32_e1536_d_n7);
        let eq32_e1537_d_n9: f64 = (eq32_e1529_d_n9 + eq32_e1536_d_n9);
        let eq32_value: f64 = eq32_e1537;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq32_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq32_e1529_d_n3), multiplicity * (eq32_e1529_d_n4), multiplicity * (eq32_e1529_d_n5), multiplicity * (eq32_e1529_d_n6), multiplicity * (eq32_e1537_d_n7), multiplicity * (eq32_e1529_d_n8), multiplicity * (eq32_e1537_d_n9), multiplicity * (eq32_e1529_d_n10), multiplicity * (eq32_e1529_d_n11), multiplicity * (eq32_e1529_d_n12)],
            [],
            [],
            1.0,
        );
        let eq33_e1541: f64 = (var_b4soiigs + var_b4soiigcs);
        let eq33_e1541_d_n3: f64 = (var_b4soiigs_dn3 + var_b4soiigcs_dn3);
        let eq33_e1541_d_n4: f64 = (var_b4soiigs_dn4 + var_b4soiigcs_dn4);
        let eq33_e1541_d_n5: f64 = (var_b4soiigs_dn5 + var_b4soiigcs_dn5);
        let eq33_e1541_d_n6: f64 = (var_b4soiigs_dn6 + var_b4soiigcs_dn6);
        let eq33_e1541_d_n7: f64 = (var_b4soiigs_dn7 + var_b4soiigcs_dn7);
        let eq33_e1541_d_n8: f64 = (var_b4soiigs_dn8 + var_b4soiigcs_dn8);
        let eq33_e1541_d_n9: f64 = (var_b4soiigs_dn9 + var_b4soiigcs_dn9);
        let eq33_e1541_d_n10: f64 = (var_b4soiigs_dn10 + var_b4soiigcs_dn10);
        let eq33_e1541_d_n11: f64 = (var_b4soiigs_dn11 + var_b4soiigcs_dn11);
        let eq33_e1541_d_n12: f64 = (var_b4soiigs_dn12 + var_b4soiigcs_dn12);
        let eq33_e1542: f64 = (p.p32 * eq33_e1541);
        let eq33_e1542_d_n3: f64 = (p.p32 * eq33_e1541_d_n3);
        let eq33_e1542_d_n4: f64 = (p.p32 * eq33_e1541_d_n4);
        let eq33_e1542_d_n5: f64 = (p.p32 * eq33_e1541_d_n5);
        let eq33_e1542_d_n6: f64 = (p.p32 * eq33_e1541_d_n6);
        let eq33_e1542_d_n7: f64 = (p.p32 * eq33_e1541_d_n7);
        let eq33_e1542_d_n8: f64 = (p.p32 * eq33_e1541_d_n8);
        let eq33_e1542_d_n9: f64 = (p.p32 * eq33_e1541_d_n9);
        let eq33_e1542_d_n10: f64 = (p.p32 * eq33_e1541_d_n10);
        let eq33_e1542_d_n11: f64 = (p.p32 * eq33_e1541_d_n11);
        let eq33_e1542_d_n12: f64 = (p.p32 * eq33_e1541_d_n12);
        let eq33_e1546: f64 = 0.0;
        let eq33_e1548: f64 = (eq33_e1546 * (nv9 - nv8));
        let eq33_e1549: f64 = (p.p32 * eq33_e1548);
        let eq33_e1549_d_n8: f64 = (p.p32 * (-eq33_e1546));
        let eq33_e1549_d_n9: f64 = (p.p32 * eq33_e1546);
        let eq33_e1550: f64 = (eq33_e1542 + eq33_e1549);
        let eq33_e1550_d_n8: f64 = (eq33_e1542_d_n8 + eq33_e1549_d_n8);
        let eq33_e1550_d_n9: f64 = (eq33_e1542_d_n9 + eq33_e1549_d_n9);
        let eq33_value: f64 = eq33_e1550;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq33_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq33_e1542_d_n3), multiplicity * (eq33_e1542_d_n4), multiplicity * (eq33_e1542_d_n5), multiplicity * (eq33_e1542_d_n6), multiplicity * (eq33_e1542_d_n7), multiplicity * (eq33_e1550_d_n8), multiplicity * (eq33_e1550_d_n9), multiplicity * (eq33_e1542_d_n10), multiplicity * (eq33_e1542_d_n11), multiplicity * (eq33_e1542_d_n12)],
            [],
            [],
            1.0,
        );
        let eq34_e1553: f64 = (p.p32 * var_b4soiig);
        let eq34_e1553_d_n3: f64 = (p.p32 * var_b4soiig_dn3);
        let eq34_e1553_d_n4: f64 = (p.p32 * var_b4soiig_dn4);
        let eq34_e1553_d_n5: f64 = (p.p32 * var_b4soiig_dn5);
        let eq34_e1553_d_n6: f64 = (p.p32 * var_b4soiig_dn6);
        let eq34_e1553_d_n7: f64 = (p.p32 * var_b4soiig_dn7);
        let eq34_e1553_d_n8: f64 = (p.p32 * var_b4soiig_dn8);
        let eq34_e1553_d_n9: f64 = (p.p32 * var_b4soiig_dn9);
        let eq34_e1553_d_n10: f64 = (p.p32 * var_b4soiig_dn10);
        let eq34_e1553_d_n11: f64 = (p.p32 * var_b4soiig_dn11);
        let eq34_e1553_d_n12: f64 = (p.p32 * var_b4soiig_dn12);
        let eq34_value: f64 = eq34_e1553;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq34_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq34_e1553_d_n3), multiplicity * (eq34_e1553_d_n4), multiplicity * (eq34_e1553_d_n5), multiplicity * (eq34_e1553_d_n6), multiplicity * (eq34_e1553_d_n7), multiplicity * (eq34_e1553_d_n8), multiplicity * (eq34_e1553_d_n9), multiplicity * (eq34_e1553_d_n10), multiplicity * (eq34_e1553_d_n11), multiplicity * (eq34_e1553_d_n12)],
            [],
            [],
            1.0,
        );
        let eq35_e1556: f64 = (p.p32 * var_b4soiigp);
        let eq35_e1556_d_n3: f64 = (p.p32 * var_b4soiigp_dn3);
        let eq35_e1556_d_n4: f64 = (p.p32 * var_b4soiigp_dn4);
        let eq35_e1556_d_n5: f64 = (p.p32 * var_b4soiigp_dn5);
        let eq35_e1556_d_n6: f64 = (p.p32 * var_b4soiigp_dn6);
        let eq35_e1556_d_n7: f64 = (p.p32 * var_b4soiigp_dn7);
        let eq35_e1556_d_n8: f64 = (p.p32 * var_b4soiigp_dn8);
        let eq35_e1556_d_n9: f64 = (p.p32 * var_b4soiigp_dn9);
        let eq35_e1556_d_n10: f64 = (p.p32 * var_b4soiigp_dn10);
        let eq35_e1556_d_n11: f64 = (p.p32 * var_b4soiigp_dn11);
        let eq35_e1556_d_n12: f64 = (p.p32 * var_b4soiigp_dn12);
        let eq35_value: f64 = eq35_e1556;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(4),
            multiplicity * (eq35_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq35_e1556_d_n3), multiplicity * (eq35_e1556_d_n4), multiplicity * (eq35_e1556_d_n5), multiplicity * (eq35_e1556_d_n6), multiplicity * (eq35_e1556_d_n7), multiplicity * (eq35_e1556_d_n8), multiplicity * (eq35_e1556_d_n9), multiplicity * (eq35_e1556_d_n10), multiplicity * (eq35_e1556_d_n11), multiplicity * (eq35_e1556_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq37_e1569, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12,) = {
    if (var_guard1517 == 0.0) {
        let eq37_e1565: f64 = (p.p37 * p.p32);
        let eq37_e1567: f64 = (eq37_e1565 * var_ibp);
        let eq37_e1567_d_n3: f64 = (eq37_e1565 * var_ibp_dn3);
        let eq37_e1567_d_n4: f64 = (eq37_e1565 * var_ibp_dn4);
        let eq37_e1567_d_n5: f64 = (eq37_e1565 * var_ibp_dn5);
        let eq37_e1567_d_n6: f64 = (eq37_e1565 * var_ibp_dn6);
        let eq37_e1567_d_n7: f64 = (eq37_e1565 * var_ibp_dn7);
        let eq37_e1567_d_n8: f64 = (eq37_e1565 * var_ibp_dn8);
        let eq37_e1567_d_n9: f64 = (eq37_e1565 * var_ibp_dn9);
        let eq37_e1567_d_n10: f64 = (eq37_e1565 * var_ibp_dn10);
        let eq37_e1567_d_n11: f64 = (eq37_e1565 * var_ibp_dn11);
        let eq37_e1567_d_n12: f64 = (eq37_e1565 * var_ibp_dn12);
        (eq37_e1567, eq37_e1567_d_n3, eq37_e1567_d_n4, eq37_e1567_d_n5, eq37_e1567_d_n6, eq37_e1567_d_n7, eq37_e1567_d_n8, eq37_e1567_d_n9, eq37_e1567_d_n10, eq37_e1567_d_n11, eq37_e1567_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1569;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq37_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq37_e1569_d_n3), multiplicity * (eq37_e1569_d_n4), multiplicity * (eq37_e1569_d_n5), multiplicity * (eq37_e1569_d_n6), multiplicity * (eq37_e1569_d_n7), multiplicity * (eq37_e1569_d_n8), multiplicity * (eq37_e1569_d_n9), multiplicity * (eq37_e1569_d_n10), multiplicity * (eq37_e1569_d_n11), multiplicity * (eq37_e1569_d_n12)],
            [],
            [],
            1.0,
        );
        let eq44_e1647: f64 = (p.p33 * var_b4soiqdrn);
        let eq44_e1647_d_n3: f64 = (p.p33 * var_b4soiqdrn_dn3);
        let eq44_e1647_d_n4: f64 = (p.p33 * var_b4soiqdrn_dn4);
        let eq44_e1647_d_n5: f64 = (p.p33 * var_b4soiqdrn_dn5);
        let eq44_e1647_d_n6: f64 = (p.p33 * var_b4soiqdrn_dn6);
        let eq44_e1647_d_n7: f64 = (p.p33 * var_b4soiqdrn_dn7);
        let eq44_e1647_d_n8: f64 = (p.p33 * var_b4soiqdrn_dn8);
        let eq44_e1647_d_n9: f64 = (p.p33 * var_b4soiqdrn_dn9);
        let eq44_e1647_d_n10: f64 = (p.p33 * var_b4soiqdrn_dn10);
        let eq44_e1647_d_n11: f64 = (p.p33 * var_b4soiqdrn_dn11);
        let eq44_e1647_d_n12: f64 = (p.p33 * var_b4soiqdrn_dn12);
        let eq44_e1648: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq44_e1647);
        let eq44_value: f64 = eq44_e1648;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq44_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq44_e1647_d_n3 * ddt_scale)), multiplicity * ((eq44_e1647_d_n4 * ddt_scale)), multiplicity * ((eq44_e1647_d_n5 * ddt_scale)), multiplicity * ((eq44_e1647_d_n6 * ddt_scale)), multiplicity * ((eq44_e1647_d_n7 * ddt_scale)), multiplicity * ((eq44_e1647_d_n8 * ddt_scale)), multiplicity * ((eq44_e1647_d_n9 * ddt_scale)), multiplicity * ((eq44_e1647_d_n10 * ddt_scale)), multiplicity * ((eq44_e1647_d_n11 * ddt_scale)), multiplicity * ((eq44_e1647_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq45_e1651: f64 = (p.p33 * var_b4soiqsrc);
        let eq45_e1651_d_n3: f64 = (p.p33 * var_b4soiqsrc_dn3);
        let eq45_e1651_d_n4: f64 = (p.p33 * var_b4soiqsrc_dn4);
        let eq45_e1651_d_n5: f64 = (p.p33 * var_b4soiqsrc_dn5);
        let eq45_e1651_d_n6: f64 = (p.p33 * var_b4soiqsrc_dn6);
        let eq45_e1651_d_n7: f64 = (p.p33 * var_b4soiqsrc_dn7);
        let eq45_e1651_d_n8: f64 = (p.p33 * var_b4soiqsrc_dn8);
        let eq45_e1651_d_n9: f64 = (p.p33 * var_b4soiqsrc_dn9);
        let eq45_e1651_d_n10: f64 = (p.p33 * var_b4soiqsrc_dn10);
        let eq45_e1651_d_n11: f64 = (p.p33 * var_b4soiqsrc_dn11);
        let eq45_e1651_d_n12: f64 = (p.p33 * var_b4soiqsrc_dn12);
        let eq45_e1652: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq45_e1651);
        let eq45_value: f64 = eq45_e1652;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq45_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq45_e1651_d_n3 * ddt_scale)), multiplicity * ((eq45_e1651_d_n4 * ddt_scale)), multiplicity * ((eq45_e1651_d_n5 * ddt_scale)), multiplicity * ((eq45_e1651_d_n6 * ddt_scale)), multiplicity * ((eq45_e1651_d_n7 * ddt_scale)), multiplicity * ((eq45_e1651_d_n8 * ddt_scale)), multiplicity * ((eq45_e1651_d_n9 * ddt_scale)), multiplicity * ((eq45_e1651_d_n10 * ddt_scale)), multiplicity * ((eq45_e1651_d_n11 * ddt_scale)), multiplicity * ((eq45_e1651_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq46_e1656: f64 = (p.p33 * var_qgate);
        let eq46_e1656_d_n3: f64 = (p.p33 * var_qgate_dn3);
        let eq46_e1656_d_n4: f64 = (p.p33 * var_qgate_dn4);
        let eq46_e1656_d_n5: f64 = (p.p33 * var_qgate_dn5);
        let eq46_e1656_d_n6: f64 = (p.p33 * var_qgate_dn6);
        let eq46_e1656_d_n7: f64 = (p.p33 * var_qgate_dn7);
        let eq46_e1656_d_n8: f64 = (p.p33 * var_qgate_dn8);
        let eq46_e1656_d_n9: f64 = (p.p33 * var_qgate_dn9);
        let eq46_e1656_d_n10: f64 = (p.p33 * var_qgate_dn10);
        let eq46_e1656_d_n11: f64 = (p.p33 * var_qgate_dn11);
        let eq46_e1656_d_n12: f64 = (p.p33 * var_qgate_dn12);
        let eq46_e1657: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq46_e1656);
        let eq46_e1658: f64 = (p.p37 * eq46_e1657);
        let eq46_e1658_d_n3: f64 = (p.p37 * (eq46_e1656_d_n3 * ddt_scale));
        let eq46_e1658_d_n4: f64 = (p.p37 * (eq46_e1656_d_n4 * ddt_scale));
        let eq46_e1658_d_n5: f64 = (p.p37 * (eq46_e1656_d_n5 * ddt_scale));
        let eq46_e1658_d_n6: f64 = (p.p37 * (eq46_e1656_d_n6 * ddt_scale));
        let eq46_e1658_d_n7: f64 = (p.p37 * (eq46_e1656_d_n7 * ddt_scale));
        let eq46_e1658_d_n8: f64 = (p.p37 * (eq46_e1656_d_n8 * ddt_scale));
        let eq46_e1658_d_n9: f64 = (p.p37 * (eq46_e1656_d_n9 * ddt_scale));
        let eq46_e1658_d_n10: f64 = (p.p37 * (eq46_e1656_d_n10 * ddt_scale));
        let eq46_e1658_d_n11: f64 = (p.p37 * (eq46_e1656_d_n11 * ddt_scale));
        let eq46_e1658_d_n12: f64 = (p.p37 * (eq46_e1656_d_n12 * ddt_scale));
        let eq46_value: f64 = eq46_e1658;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(5),
            multiplicity * (eq46_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq46_e1658_d_n3), multiplicity * (eq46_e1658_d_n4), multiplicity * (eq46_e1658_d_n5), multiplicity * (eq46_e1658_d_n6), multiplicity * (eq46_e1658_d_n7), multiplicity * (eq46_e1658_d_n8), multiplicity * (eq46_e1658_d_n9), multiplicity * (eq46_e1658_d_n10), multiplicity * (eq46_e1658_d_n11), multiplicity * (eq46_e1658_d_n12)],
            [],
            [],
            1.0,
        );
        let eq47_e1662: f64 = (p.p33 * var_qsub);
        let eq47_e1662_d_n3: f64 = (p.p33 * var_qsub_dn3);
        let eq47_e1662_d_n4: f64 = (p.p33 * var_qsub_dn4);
        let eq47_e1662_d_n5: f64 = (p.p33 * var_qsub_dn5);
        let eq47_e1662_d_n6: f64 = (p.p33 * var_qsub_dn6);
        let eq47_e1662_d_n7: f64 = (p.p33 * var_qsub_dn7);
        let eq47_e1662_d_n8: f64 = (p.p33 * var_qsub_dn8);
        let eq47_e1662_d_n9: f64 = (p.p33 * var_qsub_dn9);
        let eq47_e1662_d_n10: f64 = (p.p33 * var_qsub_dn10);
        let eq47_e1662_d_n11: f64 = (p.p33 * var_qsub_dn11);
        let eq47_e1662_d_n12: f64 = (p.p33 * var_qsub_dn12);
        let eq47_e1663: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq47_e1662);
        let eq47_e1664: f64 = (p.p37 * eq47_e1663);
        let eq47_e1664_d_n3: f64 = (p.p37 * (eq47_e1662_d_n3 * ddt_scale));
        let eq47_e1664_d_n4: f64 = (p.p37 * (eq47_e1662_d_n4 * ddt_scale));
        let eq47_e1664_d_n5: f64 = (p.p37 * (eq47_e1662_d_n5 * ddt_scale));
        let eq47_e1664_d_n6: f64 = (p.p37 * (eq47_e1662_d_n6 * ddt_scale));
        let eq47_e1664_d_n7: f64 = (p.p37 * (eq47_e1662_d_n7 * ddt_scale));
        let eq47_e1664_d_n8: f64 = (p.p37 * (eq47_e1662_d_n8 * ddt_scale));
        let eq47_e1664_d_n9: f64 = (p.p37 * (eq47_e1662_d_n9 * ddt_scale));
        let eq47_e1664_d_n10: f64 = (p.p37 * (eq47_e1662_d_n10 * ddt_scale));
        let eq47_e1664_d_n11: f64 = (p.p37 * (eq47_e1662_d_n11 * ddt_scale));
        let eq47_e1664_d_n12: f64 = (p.p37 * (eq47_e1662_d_n12 * ddt_scale));
        let eq47_value: f64 = eq47_e1664;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(3),
            Some(5),
            multiplicity * (eq47_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq47_e1664_d_n3), multiplicity * (eq47_e1664_d_n4), multiplicity * (eq47_e1664_d_n5), multiplicity * (eq47_e1664_d_n6), multiplicity * (eq47_e1664_d_n7), multiplicity * (eq47_e1664_d_n8), multiplicity * (eq47_e1664_d_n9), multiplicity * (eq47_e1664_d_n10), multiplicity * (eq47_e1664_d_n11), multiplicity * (eq47_e1664_d_n12)],
            [],
            [],
            1.0,
        );
        let eq48_e1668: f64 = (p.p33 * var_qjd_1);
        let eq48_e1668_d_n3: f64 = (p.p33 * var_qjd_1_dn3);
        let eq48_e1668_d_n4: f64 = (p.p33 * var_qjd_1_dn4);
        let eq48_e1668_d_n5: f64 = (p.p33 * var_qjd_1_dn5);
        let eq48_e1668_d_n6: f64 = (p.p33 * var_qjd_1_dn6);
        let eq48_e1668_d_n7: f64 = (p.p33 * var_qjd_1_dn7);
        let eq48_e1668_d_n8: f64 = (p.p33 * var_qjd_1_dn8);
        let eq48_e1668_d_n9: f64 = (p.p33 * var_qjd_1_dn9);
        let eq48_e1668_d_n10: f64 = (p.p33 * var_qjd_1_dn10);
        let eq48_e1668_d_n11: f64 = (p.p33 * var_qjd_1_dn11);
        let eq48_e1668_d_n12: f64 = (p.p33 * var_qjd_1_dn12);
        let eq48_e1669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq48_e1668);
        let eq48_e1670: f64 = (p.p37 * eq48_e1669);
        let eq48_e1670_d_n3: f64 = (p.p37 * (eq48_e1668_d_n3 * ddt_scale));
        let eq48_e1670_d_n4: f64 = (p.p37 * (eq48_e1668_d_n4 * ddt_scale));
        let eq48_e1670_d_n5: f64 = (p.p37 * (eq48_e1668_d_n5 * ddt_scale));
        let eq48_e1670_d_n6: f64 = (p.p37 * (eq48_e1668_d_n6 * ddt_scale));
        let eq48_e1670_d_n7: f64 = (p.p37 * (eq48_e1668_d_n7 * ddt_scale));
        let eq48_e1670_d_n8: f64 = (p.p37 * (eq48_e1668_d_n8 * ddt_scale));
        let eq48_e1670_d_n9: f64 = (p.p37 * (eq48_e1668_d_n9 * ddt_scale));
        let eq48_e1670_d_n10: f64 = (p.p37 * (eq48_e1668_d_n10 * ddt_scale));
        let eq48_e1670_d_n11: f64 = (p.p37 * (eq48_e1668_d_n11 * ddt_scale));
        let eq48_e1670_d_n12: f64 = (p.p37 * (eq48_e1668_d_n12 * ddt_scale));
        let eq48_value: f64 = eq48_e1670;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(12),
            Some(7),
            multiplicity * (eq48_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq48_e1670_d_n3), multiplicity * (eq48_e1670_d_n4), multiplicity * (eq48_e1670_d_n5), multiplicity * (eq48_e1670_d_n6), multiplicity * (eq48_e1670_d_n7), multiplicity * (eq48_e1670_d_n8), multiplicity * (eq48_e1670_d_n9), multiplicity * (eq48_e1670_d_n10), multiplicity * (eq48_e1670_d_n11), multiplicity * (eq48_e1670_d_n12)],
            [],
            [],
            1.0,
        );
        let eq49_e1674: f64 = (p.p33 * var_qjs_1);
        let eq49_e1674_d_n3: f64 = (p.p33 * var_qjs_1_dn3);
        let eq49_e1674_d_n4: f64 = (p.p33 * var_qjs_1_dn4);
        let eq49_e1674_d_n5: f64 = (p.p33 * var_qjs_1_dn5);
        let eq49_e1674_d_n6: f64 = (p.p33 * var_qjs_1_dn6);
        let eq49_e1674_d_n7: f64 = (p.p33 * var_qjs_1_dn7);
        let eq49_e1674_d_n8: f64 = (p.p33 * var_qjs_1_dn8);
        let eq49_e1674_d_n9: f64 = (p.p33 * var_qjs_1_dn9);
        let eq49_e1674_d_n10: f64 = (p.p33 * var_qjs_1_dn10);
        let eq49_e1674_d_n11: f64 = (p.p33 * var_qjs_1_dn11);
        let eq49_e1674_d_n12: f64 = (p.p33 * var_qjs_1_dn12);
        let eq49_e1675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq49_e1674);
        let eq49_e1676: f64 = (p.p37 * eq49_e1675);
        let eq49_e1676_d_n3: f64 = (p.p37 * (eq49_e1674_d_n3 * ddt_scale));
        let eq49_e1676_d_n4: f64 = (p.p37 * (eq49_e1674_d_n4 * ddt_scale));
        let eq49_e1676_d_n5: f64 = (p.p37 * (eq49_e1674_d_n5 * ddt_scale));
        let eq49_e1676_d_n6: f64 = (p.p37 * (eq49_e1674_d_n6 * ddt_scale));
        let eq49_e1676_d_n7: f64 = (p.p37 * (eq49_e1674_d_n7 * ddt_scale));
        let eq49_e1676_d_n8: f64 = (p.p37 * (eq49_e1674_d_n8 * ddt_scale));
        let eq49_e1676_d_n9: f64 = (p.p37 * (eq49_e1674_d_n9 * ddt_scale));
        let eq49_e1676_d_n10: f64 = (p.p37 * (eq49_e1674_d_n10 * ddt_scale));
        let eq49_e1676_d_n11: f64 = (p.p37 * (eq49_e1674_d_n11 * ddt_scale));
        let eq49_e1676_d_n12: f64 = (p.p37 * (eq49_e1674_d_n12 * ddt_scale));
        let eq49_value: f64 = eq49_e1676;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(11),
            Some(8),
            multiplicity * (eq49_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq49_e1676_d_n3), multiplicity * (eq49_e1676_d_n4), multiplicity * (eq49_e1676_d_n5), multiplicity * (eq49_e1676_d_n6), multiplicity * (eq49_e1676_d_n7), multiplicity * (eq49_e1676_d_n8), multiplicity * (eq49_e1676_d_n9), multiplicity * (eq49_e1676_d_n10), multiplicity * (eq49_e1676_d_n11), multiplicity * (eq49_e1676_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq50_e1685, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12,) = {
    if (var_guard1518 != 0.0) {
        let eq50_e1681: f64 = (p.p33 * var_qgdo);
        let eq50_e1681_d_n3: f64 = (p.p33 * var_qgdo_dn3);
        let eq50_e1681_d_n4: f64 = (p.p33 * var_qgdo_dn4);
        let eq50_e1681_d_n5: f64 = (p.p33 * var_qgdo_dn5);
        let eq50_e1681_d_n6: f64 = (p.p33 * var_qgdo_dn6);
        let eq50_e1681_d_n7: f64 = (p.p33 * var_qgdo_dn7);
        let eq50_e1681_d_n8: f64 = (p.p33 * var_qgdo_dn8);
        let eq50_e1681_d_n9: f64 = (p.p33 * var_qgdo_dn9);
        let eq50_e1681_d_n10: f64 = (p.p33 * var_qgdo_dn10);
        let eq50_e1681_d_n11: f64 = (p.p33 * var_qgdo_dn11);
        let eq50_e1681_d_n12: f64 = (p.p33 * var_qgdo_dn12);
        let eq50_e1682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq50_e1681);
        let eq50_e1683: f64 = (p.p37 * eq50_e1682);
        let eq50_e1683_d_n3: f64 = (p.p37 * (eq50_e1681_d_n3 * ddt_scale));
        let eq50_e1683_d_n4: f64 = (p.p37 * (eq50_e1681_d_n4 * ddt_scale));
        let eq50_e1683_d_n5: f64 = (p.p37 * (eq50_e1681_d_n5 * ddt_scale));
        let eq50_e1683_d_n6: f64 = (p.p37 * (eq50_e1681_d_n6 * ddt_scale));
        let eq50_e1683_d_n7: f64 = (p.p37 * (eq50_e1681_d_n7 * ddt_scale));
        let eq50_e1683_d_n8: f64 = (p.p37 * (eq50_e1681_d_n8 * ddt_scale));
        let eq50_e1683_d_n9: f64 = (p.p37 * (eq50_e1681_d_n9 * ddt_scale));
        let eq50_e1683_d_n10: f64 = (p.p37 * (eq50_e1681_d_n10 * ddt_scale));
        let eq50_e1683_d_n11: f64 = (p.p37 * (eq50_e1681_d_n11 * ddt_scale));
        let eq50_e1683_d_n12: f64 = (p.p37 * (eq50_e1681_d_n12 * ddt_scale));
        (eq50_e1683, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1685;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(7),
            multiplicity * (eq50_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq50_e1685_d_n3), multiplicity * (eq50_e1685_d_n4), multiplicity * (eq50_e1685_d_n5), multiplicity * (eq50_e1685_d_n6), multiplicity * (eq50_e1685_d_n7), multiplicity * (eq50_e1685_d_n8), multiplicity * (eq50_e1685_d_n9), multiplicity * (eq50_e1685_d_n10), multiplicity * (eq50_e1685_d_n11), multiplicity * (eq50_e1685_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq51_e1694, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12,) = {
    if (var_guard1518 != 0.0) {
        let eq51_e1690: f64 = (p.p33 * var_qgso);
        let eq51_e1690_d_n3: f64 = (p.p33 * var_qgso_dn3);
        let eq51_e1690_d_n4: f64 = (p.p33 * var_qgso_dn4);
        let eq51_e1690_d_n5: f64 = (p.p33 * var_qgso_dn5);
        let eq51_e1690_d_n6: f64 = (p.p33 * var_qgso_dn6);
        let eq51_e1690_d_n7: f64 = (p.p33 * var_qgso_dn7);
        let eq51_e1690_d_n8: f64 = (p.p33 * var_qgso_dn8);
        let eq51_e1690_d_n9: f64 = (p.p33 * var_qgso_dn9);
        let eq51_e1690_d_n10: f64 = (p.p33 * var_qgso_dn10);
        let eq51_e1690_d_n11: f64 = (p.p33 * var_qgso_dn11);
        let eq51_e1690_d_n12: f64 = (p.p33 * var_qgso_dn12);
        let eq51_e1691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1690);
        let eq51_e1692: f64 = (p.p37 * eq51_e1691);
        let eq51_e1692_d_n3: f64 = (p.p37 * (eq51_e1690_d_n3 * ddt_scale));
        let eq51_e1692_d_n4: f64 = (p.p37 * (eq51_e1690_d_n4 * ddt_scale));
        let eq51_e1692_d_n5: f64 = (p.p37 * (eq51_e1690_d_n5 * ddt_scale));
        let eq51_e1692_d_n6: f64 = (p.p37 * (eq51_e1690_d_n6 * ddt_scale));
        let eq51_e1692_d_n7: f64 = (p.p37 * (eq51_e1690_d_n7 * ddt_scale));
        let eq51_e1692_d_n8: f64 = (p.p37 * (eq51_e1690_d_n8 * ddt_scale));
        let eq51_e1692_d_n9: f64 = (p.p37 * (eq51_e1690_d_n9 * ddt_scale));
        let eq51_e1692_d_n10: f64 = (p.p37 * (eq51_e1690_d_n10 * ddt_scale));
        let eq51_e1692_d_n11: f64 = (p.p37 * (eq51_e1690_d_n11 * ddt_scale));
        let eq51_e1692_d_n12: f64 = (p.p37 * (eq51_e1690_d_n12 * ddt_scale));
        (eq51_e1692, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1694;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(8),
            multiplicity * (eq51_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq51_e1694_d_n3), multiplicity * (eq51_e1694_d_n4), multiplicity * (eq51_e1694_d_n5), multiplicity * (eq51_e1694_d_n6), multiplicity * (eq51_e1694_d_n7), multiplicity * (eq51_e1694_d_n8), multiplicity * (eq51_e1694_d_n9), multiplicity * (eq51_e1694_d_n10), multiplicity * (eq51_e1694_d_n11), multiplicity * (eq51_e1694_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq52_e1703, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12,) = {
    if (var_guard1518 != 0.0) {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * var_pparam_b4soicgeo);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * var_pparam_b4soicgeo) + (eq52_e1698 * var_pparam_b4soicgeo_dn3));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn4);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn5);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn6);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn7);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn8);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn9);
        let eq52_e1700_d_n10: f64 = ((p.p33 * var_pparam_b4soicgeo) + (eq52_e1698 * var_pparam_b4soicgeo_dn10));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn11);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * var_pparam_b4soicgeo_dn12);
        let eq52_e1701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1700);
        (eq52_e1701, (eq52_e1700_d_n3 * ddt_scale), (eq52_e1700_d_n4 * ddt_scale), (eq52_e1700_d_n5 * ddt_scale), (eq52_e1700_d_n6 * ddt_scale), (eq52_e1700_d_n7 * ddt_scale), (eq52_e1700_d_n8 * ddt_scale), (eq52_e1700_d_n9 * ddt_scale), (eq52_e1700_d_n10 * ddt_scale), (eq52_e1700_d_n11 * ddt_scale), (eq52_e1700_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1703;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(3),
            multiplicity * (eq52_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq52_e1703_d_n3), multiplicity * (eq52_e1703_d_n4), multiplicity * (eq52_e1703_d_n5), multiplicity * (eq52_e1703_d_n6), multiplicity * (eq52_e1703_d_n7), multiplicity * (eq52_e1703_d_n8), multiplicity * (eq52_e1703_d_n9), multiplicity * (eq52_e1703_d_n10), multiplicity * (eq52_e1703_d_n11), multiplicity * (eq52_e1703_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_b4soigcrg: f64,
        var_b4soigcrg_dn10: f64,
        var_b4soigcrg_dn11: f64,
        var_b4soigcrg_dn12: f64,
        var_b4soigcrg_dn3: f64,
        var_b4soigcrg_dn4: f64,
        var_b4soigcrg_dn5: f64,
        var_b4soigcrg_dn6: f64,
        var_b4soigcrg_dn7: f64,
        var_b4soigcrg_dn8: f64,
        var_b4soigcrg_dn9: f64,
        var_b4soiqde: f64,
        var_b4soiqde_dn10: f64,
        var_b4soiqde_dn11: f64,
        var_b4soiqde_dn12: f64,
        var_b4soiqde_dn3: f64,
        var_b4soiqde_dn4: f64,
        var_b4soiqde_dn5: f64,
        var_b4soiqde_dn6: f64,
        var_b4soiqde_dn7: f64,
        var_b4soiqde_dn8: f64,
        var_b4soiqde_dn9: f64,
        var_b4soiqse: f64,
        var_b4soiqse_dn10: f64,
        var_b4soiqse_dn11: f64,
        var_b4soiqse_dn12: f64,
        var_b4soiqse_dn3: f64,
        var_b4soiqse_dn4: f64,
        var_b4soiqse_dn5: f64,
        var_b4soiqse_dn6: f64,
        var_b4soiqse_dn7: f64,
        var_b4soiqse_dn8: f64,
        var_b4soiqse_dn9: f64,
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_deltemp_dn6: f64,
        var_guard1518: f64,
        var_guard1520: f64,
        var_guard1524: f64,
        var_guard1525: f64,
        var_guard1526: f64,
        var_guard1527: f64,
        var_guard1528: f64,
        var_ids_1: f64,
        var_ids_1_dn10: f64,
        var_ids_1_dn11: f64,
        var_ids_1_dn12: f64,
        var_ids_1_dn3: f64,
        var_ids_1_dn4: f64,
        var_ids_1_dn5: f64,
        var_ids_1_dn6: f64,
        var_ids_1_dn7: f64,
        var_ids_1_dn8: f64,
        var_ids_1_dn9: f64,
        var_pparam_b4soicgeo: f64,
        var_pparam_b4soicgeo_dn10: f64,
        var_pparam_b4soicgeo_dn11: f64,
        var_pparam_b4soicgeo_dn12: f64,
        var_pparam_b4soicgeo_dn3: f64,
        var_pparam_b4soicgeo_dn4: f64,
        var_pparam_b4soicgeo_dn5: f64,
        var_pparam_b4soicgeo_dn6: f64,
        var_pparam_b4soicgeo_dn7: f64,
        var_pparam_b4soicgeo_dn8: f64,
        var_pparam_b4soicgeo_dn9: f64,
        var_pparam_b4soicth: f64,
        var_pparam_b4soicth_dn10: f64,
        var_pparam_b4soicth_dn11: f64,
        var_pparam_b4soicth_dn12: f64,
        var_pparam_b4soicth_dn3: f64,
        var_pparam_b4soicth_dn4: f64,
        var_pparam_b4soicth_dn5: f64,
        var_pparam_b4soicth_dn6: f64,
        var_pparam_b4soicth_dn7: f64,
        var_pparam_b4soicth_dn8: f64,
        var_pparam_b4soicth_dn9: f64,
        var_pparam_b4soirth: f64,
        var_pparam_b4soirth_dn10: f64,
        var_pparam_b4soirth_dn11: f64,
        var_pparam_b4soirth_dn12: f64,
        var_pparam_b4soirth_dn3: f64,
        var_pparam_b4soirth_dn4: f64,
        var_pparam_b4soirth_dn5: f64,
        var_pparam_b4soirth_dn6: f64,
        var_pparam_b4soirth_dn7: f64,
        var_pparam_b4soirth_dn8: f64,
        var_pparam_b4soirth_dn9: f64,
        var_qgdo: f64,
        var_qgdo_dn10: f64,
        var_qgdo_dn11: f64,
        var_qgdo_dn12: f64,
        var_qgdo_dn3: f64,
        var_qgdo_dn4: f64,
        var_qgdo_dn5: f64,
        var_qgdo_dn6: f64,
        var_qgdo_dn7: f64,
        var_qgdo_dn8: f64,
        var_qgdo_dn9: f64,
        var_qgso: f64,
        var_qgso_dn10: f64,
        var_qgso_dn11: f64,
        var_qgso_dn12: f64,
        var_qgso_dn3: f64,
        var_qgso_dn4: f64,
        var_qgso_dn5: f64,
        var_qgso_dn6: f64,
        var_qgso_dn7: f64,
        var_qgso_dn8: f64,
        var_qgso_dn9: f64,
        var_vds_1: f64,
        var_vds_1_dn7: f64,
        var_vds_1_dn8: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq53_e1713, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12,) = {
    if (var_guard1518 == 0.0) {
        let eq53_e1709: f64 = (p.p33 * var_qgdo);
        let eq53_e1709_d_n3: f64 = (p.p33 * var_qgdo_dn3);
        let eq53_e1709_d_n4: f64 = (p.p33 * var_qgdo_dn4);
        let eq53_e1709_d_n5: f64 = (p.p33 * var_qgdo_dn5);
        let eq53_e1709_d_n6: f64 = (p.p33 * var_qgdo_dn6);
        let eq53_e1709_d_n7: f64 = (p.p33 * var_qgdo_dn7);
        let eq53_e1709_d_n8: f64 = (p.p33 * var_qgdo_dn8);
        let eq53_e1709_d_n9: f64 = (p.p33 * var_qgdo_dn9);
        let eq53_e1709_d_n10: f64 = (p.p33 * var_qgdo_dn10);
        let eq53_e1709_d_n11: f64 = (p.p33 * var_qgdo_dn11);
        let eq53_e1709_d_n12: f64 = (p.p33 * var_qgdo_dn12);
        let eq53_e1710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1709);
        let eq53_e1711: f64 = (p.p37 * eq53_e1710);
        let eq53_e1711_d_n3: f64 = (p.p37 * (eq53_e1709_d_n3 * ddt_scale));
        let eq53_e1711_d_n4: f64 = (p.p37 * (eq53_e1709_d_n4 * ddt_scale));
        let eq53_e1711_d_n5: f64 = (p.p37 * (eq53_e1709_d_n5 * ddt_scale));
        let eq53_e1711_d_n6: f64 = (p.p37 * (eq53_e1709_d_n6 * ddt_scale));
        let eq53_e1711_d_n7: f64 = (p.p37 * (eq53_e1709_d_n7 * ddt_scale));
        let eq53_e1711_d_n8: f64 = (p.p37 * (eq53_e1709_d_n8 * ddt_scale));
        let eq53_e1711_d_n9: f64 = (p.p37 * (eq53_e1709_d_n9 * ddt_scale));
        let eq53_e1711_d_n10: f64 = (p.p37 * (eq53_e1709_d_n10 * ddt_scale));
        let eq53_e1711_d_n11: f64 = (p.p37 * (eq53_e1709_d_n11 * ddt_scale));
        let eq53_e1711_d_n12: f64 = (p.p37 * (eq53_e1709_d_n12 * ddt_scale));
        (eq53_e1711, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1713;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq53_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq53_e1713_d_n3), multiplicity * (eq53_e1713_d_n4), multiplicity * (eq53_e1713_d_n5), multiplicity * (eq53_e1713_d_n6), multiplicity * (eq53_e1713_d_n7), multiplicity * (eq53_e1713_d_n8), multiplicity * (eq53_e1713_d_n9), multiplicity * (eq53_e1713_d_n10), multiplicity * (eq53_e1713_d_n11), multiplicity * (eq53_e1713_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq54_e1723, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12,) = {
    if (var_guard1518 == 0.0) {
        let eq54_e1719: f64 = (p.p33 * var_qgso);
        let eq54_e1719_d_n3: f64 = (p.p33 * var_qgso_dn3);
        let eq54_e1719_d_n4: f64 = (p.p33 * var_qgso_dn4);
        let eq54_e1719_d_n5: f64 = (p.p33 * var_qgso_dn5);
        let eq54_e1719_d_n6: f64 = (p.p33 * var_qgso_dn6);
        let eq54_e1719_d_n7: f64 = (p.p33 * var_qgso_dn7);
        let eq54_e1719_d_n8: f64 = (p.p33 * var_qgso_dn8);
        let eq54_e1719_d_n9: f64 = (p.p33 * var_qgso_dn9);
        let eq54_e1719_d_n10: f64 = (p.p33 * var_qgso_dn10);
        let eq54_e1719_d_n11: f64 = (p.p33 * var_qgso_dn11);
        let eq54_e1719_d_n12: f64 = (p.p33 * var_qgso_dn12);
        let eq54_e1720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq54_e1719);
        let eq54_e1721: f64 = (p.p37 * eq54_e1720);
        let eq54_e1721_d_n3: f64 = (p.p37 * (eq54_e1719_d_n3 * ddt_scale));
        let eq54_e1721_d_n4: f64 = (p.p37 * (eq54_e1719_d_n4 * ddt_scale));
        let eq54_e1721_d_n5: f64 = (p.p37 * (eq54_e1719_d_n5 * ddt_scale));
        let eq54_e1721_d_n6: f64 = (p.p37 * (eq54_e1719_d_n6 * ddt_scale));
        let eq54_e1721_d_n7: f64 = (p.p37 * (eq54_e1719_d_n7 * ddt_scale));
        let eq54_e1721_d_n8: f64 = (p.p37 * (eq54_e1719_d_n8 * ddt_scale));
        let eq54_e1721_d_n9: f64 = (p.p37 * (eq54_e1719_d_n9 * ddt_scale));
        let eq54_e1721_d_n10: f64 = (p.p37 * (eq54_e1719_d_n10 * ddt_scale));
        let eq54_e1721_d_n11: f64 = (p.p37 * (eq54_e1719_d_n11 * ddt_scale));
        let eq54_e1721_d_n12: f64 = (p.p37 * (eq54_e1719_d_n12 * ddt_scale));
        (eq54_e1721, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e1723;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(8),
            multiplicity * (eq54_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq54_e1723_d_n3), multiplicity * (eq54_e1723_d_n4), multiplicity * (eq54_e1723_d_n5), multiplicity * (eq54_e1723_d_n6), multiplicity * (eq54_e1723_d_n7), multiplicity * (eq54_e1723_d_n8), multiplicity * (eq54_e1723_d_n9), multiplicity * (eq54_e1723_d_n10), multiplicity * (eq54_e1723_d_n11), multiplicity * (eq54_e1723_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq55_e1733, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12,) = {
    if (var_guard1518 == 0.0) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * var_pparam_b4soicgeo);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * var_pparam_b4soicgeo) + (eq55_e1728 * var_pparam_b4soicgeo_dn3));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn4);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn5);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn6);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn7);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn8);
        let eq55_e1730_d_n9: f64 = ((p.p33 * var_pparam_b4soicgeo) + (eq55_e1728 * var_pparam_b4soicgeo_dn9));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn10);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn11);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * var_pparam_b4soicgeo_dn12);
        let eq55_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq55_e1730);
        (eq55_e1731, (eq55_e1730_d_n3 * ddt_scale), (eq55_e1730_d_n4 * ddt_scale), (eq55_e1730_d_n5 * ddt_scale), (eq55_e1730_d_n6 * ddt_scale), (eq55_e1730_d_n7 * ddt_scale), (eq55_e1730_d_n8 * ddt_scale), (eq55_e1730_d_n9 * ddt_scale), (eq55_e1730_d_n10 * ddt_scale), (eq55_e1730_d_n11 * ddt_scale), (eq55_e1730_d_n12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1733;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(9),
            Some(3),
            multiplicity * (eq55_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq55_e1733_d_n3), multiplicity * (eq55_e1733_d_n4), multiplicity * (eq55_e1733_d_n5), multiplicity * (eq55_e1733_d_n6), multiplicity * (eq55_e1733_d_n7), multiplicity * (eq55_e1733_d_n8), multiplicity * (eq55_e1733_d_n9), multiplicity * (eq55_e1733_d_n10), multiplicity * (eq55_e1733_d_n11), multiplicity * (eq55_e1733_d_n12)],
            [],
            [],
            1.0,
        );
        let eq56_e1736: f64 = (p.p33 * var_b4soiqde);
        let eq56_e1736_d_n3: f64 = (p.p33 * var_b4soiqde_dn3);
        let eq56_e1736_d_n4: f64 = (p.p33 * var_b4soiqde_dn4);
        let eq56_e1736_d_n5: f64 = (p.p33 * var_b4soiqde_dn5);
        let eq56_e1736_d_n6: f64 = (p.p33 * var_b4soiqde_dn6);
        let eq56_e1736_d_n7: f64 = (p.p33 * var_b4soiqde_dn7);
        let eq56_e1736_d_n8: f64 = (p.p33 * var_b4soiqde_dn8);
        let eq56_e1736_d_n9: f64 = (p.p33 * var_b4soiqde_dn9);
        let eq56_e1736_d_n10: f64 = (p.p33 * var_b4soiqde_dn10);
        let eq56_e1736_d_n11: f64 = (p.p33 * var_b4soiqde_dn11);
        let eq56_e1736_d_n12: f64 = (p.p33 * var_b4soiqde_dn12);
        let eq56_e1737: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq56_e1736);
        let eq56_value: f64 = eq56_e1737;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq56_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq56_e1736_d_n3 * ddt_scale)), multiplicity * ((eq56_e1736_d_n4 * ddt_scale)), multiplicity * ((eq56_e1736_d_n5 * ddt_scale)), multiplicity * ((eq56_e1736_d_n6 * ddt_scale)), multiplicity * ((eq56_e1736_d_n7 * ddt_scale)), multiplicity * ((eq56_e1736_d_n8 * ddt_scale)), multiplicity * ((eq56_e1736_d_n9 * ddt_scale)), multiplicity * ((eq56_e1736_d_n10 * ddt_scale)), multiplicity * ((eq56_e1736_d_n11 * ddt_scale)), multiplicity * ((eq56_e1736_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq57_e1740: f64 = (p.p33 * var_b4soiqse);
        let eq57_e1740_d_n3: f64 = (p.p33 * var_b4soiqse_dn3);
        let eq57_e1740_d_n4: f64 = (p.p33 * var_b4soiqse_dn4);
        let eq57_e1740_d_n5: f64 = (p.p33 * var_b4soiqse_dn5);
        let eq57_e1740_d_n6: f64 = (p.p33 * var_b4soiqse_dn6);
        let eq57_e1740_d_n7: f64 = (p.p33 * var_b4soiqse_dn7);
        let eq57_e1740_d_n8: f64 = (p.p33 * var_b4soiqse_dn8);
        let eq57_e1740_d_n9: f64 = (p.p33 * var_b4soiqse_dn9);
        let eq57_e1740_d_n10: f64 = (p.p33 * var_b4soiqse_dn10);
        let eq57_e1740_d_n11: f64 = (p.p33 * var_b4soiqse_dn11);
        let eq57_e1740_d_n12: f64 = (p.p33 * var_b4soiqse_dn12);
        let eq57_e1741: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq57_e1740);
        let eq57_value: f64 = eq57_e1741;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(8),
            Some(3),
            multiplicity * (eq57_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * ((eq57_e1740_d_n3 * ddt_scale)), multiplicity * ((eq57_e1740_d_n4 * ddt_scale)), multiplicity * ((eq57_e1740_d_n5 * ddt_scale)), multiplicity * ((eq57_e1740_d_n6 * ddt_scale)), multiplicity * ((eq57_e1740_d_n7 * ddt_scale)), multiplicity * ((eq57_e1740_d_n8 * ddt_scale)), multiplicity * ((eq57_e1740_d_n9 * ddt_scale)), multiplicity * ((eq57_e1740_d_n10 * ddt_scale)), multiplicity * ((eq57_e1740_d_n11 * ddt_scale)), multiplicity * ((eq57_e1740_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq62_e1779, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12,) = {
    if (var_guard1520 == 0.0) {
        let eq62_e1775: f64 = (p.p32 * (nv10 - nv9));
        let eq62_e1777: f64 = (eq62_e1775 * var_b4soigcrg);
        let eq62_e1777_d_n3: f64 = (eq62_e1775 * var_b4soigcrg_dn3);
        let eq62_e1777_d_n4: f64 = (eq62_e1775 * var_b4soigcrg_dn4);
        let eq62_e1777_d_n5: f64 = (eq62_e1775 * var_b4soigcrg_dn5);
        let eq62_e1777_d_n6: f64 = (eq62_e1775 * var_b4soigcrg_dn6);
        let eq62_e1777_d_n7: f64 = (eq62_e1775 * var_b4soigcrg_dn7);
        let eq62_e1777_d_n8: f64 = (eq62_e1775 * var_b4soigcrg_dn8);
        let eq62_e1777_d_n9: f64 = (((-p.p32) * var_b4soigcrg) + (eq62_e1775 * var_b4soigcrg_dn9));
        let eq62_e1777_d_n10: f64 = ((p.p32 * var_b4soigcrg) + (eq62_e1775 * var_b4soigcrg_dn10));
        let eq62_e1777_d_n11: f64 = (eq62_e1775 * var_b4soigcrg_dn11);
        let eq62_e1777_d_n12: f64 = (eq62_e1775 * var_b4soigcrg_dn12);
        (eq62_e1777, eq62_e1777_d_n3, eq62_e1777_d_n4, eq62_e1777_d_n5, eq62_e1777_d_n6, eq62_e1777_d_n7, eq62_e1777_d_n8, eq62_e1777_d_n9, eq62_e1777_d_n10, eq62_e1777_d_n11, eq62_e1777_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1779;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(10),
            Some(9),
            multiplicity * (eq62_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq62_e1779_d_n3), multiplicity * (eq62_e1779_d_n4), multiplicity * (eq62_e1779_d_n5), multiplicity * (eq62_e1779_d_n6), multiplicity * (eq62_e1779_d_n7), multiplicity * (eq62_e1779_d_n8), multiplicity * (eq62_e1779_d_n9), multiplicity * (eq62_e1779_d_n10), multiplicity * (eq62_e1779_d_n11), multiplicity * (eq62_e1779_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq71_e1869, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12,) = {
    if (((var_guard1524 != 0.0) && (var_guard1525 != 0.0)) && (var_guard1526 != 0.0)) {
        let eq71_e1856: f64 = (-var_ids_1);
        let eq71_e1858: f64 = (eq71_e1856 * var_vds_1);
        let eq71_e1858_d_n3: f64 = ((-var_ids_1_dn3) * var_vds_1);
        let eq71_e1858_d_n4: f64 = ((-var_ids_1_dn4) * var_vds_1);
        let eq71_e1858_d_n5: f64 = ((-var_ids_1_dn5) * var_vds_1);
        let eq71_e1858_d_n6: f64 = ((-var_ids_1_dn6) * var_vds_1);
        let eq71_e1858_d_n7: f64 = (((-var_ids_1_dn7) * var_vds_1) + (eq71_e1856 * var_vds_1_dn7));
        let eq71_e1858_d_n8: f64 = (((-var_ids_1_dn8) * var_vds_1) + (eq71_e1856 * var_vds_1_dn8));
        let eq71_e1858_d_n9: f64 = ((-var_ids_1_dn9) * var_vds_1);
        let eq71_e1858_d_n10: f64 = ((-var_ids_1_dn10) * var_vds_1);
        let eq71_e1858_d_n11: f64 = ((-var_ids_1_dn11) * var_vds_1);
        let eq71_e1858_d_n12: f64 = ((-var_ids_1_dn12) * var_vds_1);
        let eq71_e1861: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq71_e1861_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq71_e1861_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq71_e1861_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq71_e1861_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq71_e1861_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq71_e1861_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq71_e1861_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq71_e1861_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq71_e1861_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq71_e1861_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq71_e1862: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq71_e1861);
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1862);
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + (eq71_e1861_d_n3 * ddt_scale));
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + (eq71_e1861_d_n4 * ddt_scale));
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + (eq71_e1861_d_n5 * ddt_scale));
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + (eq71_e1861_d_n6 * ddt_scale));
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + (eq71_e1861_d_n7 * ddt_scale));
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + (eq71_e1861_d_n8 * ddt_scale));
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + (eq71_e1861_d_n9 * ddt_scale));
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + (eq71_e1861_d_n10 * ddt_scale));
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + (eq71_e1861_d_n11 * ddt_scale));
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + (eq71_e1861_d_n12 * ddt_scale));
        let eq71_e1866: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq71_e1866_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_0: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq71_e1866_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1866_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1866_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1866_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1866_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1866_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        (eq71_e1867, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1869;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            None,
            multiplicity * (eq71_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq71_e1869_d_n3), multiplicity * (eq71_e1869_d_n4), multiplicity * (eq71_e1869_d_n5), multiplicity * (eq71_e1869_d_n6), multiplicity * (eq71_e1869_d_n7), multiplicity * (eq71_e1869_d_n8), multiplicity * (eq71_e1869_d_n9), multiplicity * (eq71_e1869_d_n10), multiplicity * (eq71_e1869_d_n11), multiplicity * (eq71_e1869_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq72_e1892, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12,) = {
    if ((((var_guard1524 != 0.0) && (var_guard1525 != 0.0)) && (var_guard1526 == 0.0)) && (var_guard1527 != 0.0)) {
        let eq72_e1879: f64 = (-var_ids_1);
        let eq72_e1881: f64 = (eq72_e1879 * var_vds_1);
        let eq72_e1881_d_n3: f64 = ((-var_ids_1_dn3) * var_vds_1);
        let eq72_e1881_d_n4: f64 = ((-var_ids_1_dn4) * var_vds_1);
        let eq72_e1881_d_n5: f64 = ((-var_ids_1_dn5) * var_vds_1);
        let eq72_e1881_d_n6: f64 = ((-var_ids_1_dn6) * var_vds_1);
        let eq72_e1881_d_n7: f64 = (((-var_ids_1_dn7) * var_vds_1) + (eq72_e1879 * var_vds_1_dn7));
        let eq72_e1881_d_n8: f64 = (((-var_ids_1_dn8) * var_vds_1) + (eq72_e1879 * var_vds_1_dn8));
        let eq72_e1881_d_n9: f64 = ((-var_ids_1_dn9) * var_vds_1);
        let eq72_e1881_d_n10: f64 = ((-var_ids_1_dn10) * var_vds_1);
        let eq72_e1881_d_n11: f64 = ((-var_ids_1_dn11) * var_vds_1);
        let eq72_e1881_d_n12: f64 = ((-var_ids_1_dn12) * var_vds_1);
        let eq72_e1884: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq72_e1884_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq72_e1884_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq72_e1884_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq72_e1884_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq72_e1884_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq72_e1884_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq72_e1884_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq72_e1884_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq72_e1884_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq72_e1884_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq72_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq72_e1884);
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1885);
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + (eq72_e1884_d_n3 * ddt_scale));
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + (eq72_e1884_d_n4 * ddt_scale));
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + (eq72_e1884_d_n5 * ddt_scale));
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + (eq72_e1884_d_n6 * ddt_scale));
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + (eq72_e1884_d_n7 * ddt_scale));
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + (eq72_e1884_d_n8 * ddt_scale));
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + (eq72_e1884_d_n9 * ddt_scale));
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + (eq72_e1884_d_n10 * ddt_scale));
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + (eq72_e1884_d_n11 * ddt_scale));
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + (eq72_e1884_d_n12 * ddt_scale));
        let eq72_e1889: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq72_e1889_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_1: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq72_e1889_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1889_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1889_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1889_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1889_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1889_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        (eq72_e1890, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1892;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(4),
            None,
            multiplicity * (eq72_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq72_e1892_d_n3), multiplicity * (eq72_e1892_d_n4), multiplicity * (eq72_e1892_d_n5), multiplicity * (eq72_e1892_d_n6), multiplicity * (eq72_e1892_d_n7), multiplicity * (eq72_e1892_d_n8), multiplicity * (eq72_e1892_d_n9), multiplicity * (eq72_e1892_d_n10), multiplicity * (eq72_e1892_d_n11), multiplicity * (eq72_e1892_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq73_e1920, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12,) = {
    if (((((var_guard1524 != 0.0) && (var_guard1525 != 0.0)) && (var_guard1526 == 0.0)) && (var_guard1527 == 0.0)) && (var_guard1528 != 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (var_ids_1 * __rspice_inv_cse_2);
        let eq73_e1906_d_n3: f64 = (var_ids_1_dn3 * __rspice_inv_cse_2);
        let eq73_e1906_d_n4: f64 = (var_ids_1_dn4 * __rspice_inv_cse_2);
        let eq73_e1906_d_n5: f64 = (var_ids_1_dn5 * __rspice_inv_cse_2);
        let eq73_e1906_d_n6: f64 = (var_ids_1_dn6 * __rspice_inv_cse_2);
        let eq73_e1906_d_n7: f64 = (var_ids_1_dn7 * __rspice_inv_cse_2);
        let eq73_e1906_d_n8: f64 = (var_ids_1_dn8 * __rspice_inv_cse_2);
        let eq73_e1906_d_n9: f64 = (var_ids_1_dn9 * __rspice_inv_cse_2);
        let eq73_e1906_d_n10: f64 = (var_ids_1_dn10 * __rspice_inv_cse_2);
        let eq73_e1906_d_n11: f64 = (var_ids_1_dn11 * __rspice_inv_cse_2);
        let eq73_e1906_d_n12: f64 = (var_ids_1_dn12 * __rspice_inv_cse_2);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * var_vds_1);
        let eq73_e1909_d_n3: f64 = ((-eq73_e1906_d_n3) * var_vds_1);
        let eq73_e1909_d_n4: f64 = ((-eq73_e1906_d_n4) * var_vds_1);
        let eq73_e1909_d_n5: f64 = ((-eq73_e1906_d_n5) * var_vds_1);
        let eq73_e1909_d_n6: f64 = ((-eq73_e1906_d_n6) * var_vds_1);
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * var_vds_1) + (eq73_e1907 * var_vds_1_dn7));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * var_vds_1) + (eq73_e1907 * var_vds_1_dn8));
        let eq73_e1909_d_n9: f64 = ((-eq73_e1906_d_n9) * var_vds_1);
        let eq73_e1909_d_n10: f64 = ((-eq73_e1906_d_n10) * var_vds_1);
        let eq73_e1909_d_n11: f64 = ((-eq73_e1906_d_n11) * var_vds_1);
        let eq73_e1909_d_n12: f64 = ((-eq73_e1906_d_n12) * var_vds_1);
        let eq73_e1912: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq73_e1912_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq73_e1912_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq73_e1912_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq73_e1912_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq73_e1912_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq73_e1912_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq73_e1912_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq73_e1912_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq73_e1912_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq73_e1912_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq73_e1913: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq73_e1912);
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1913);
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + (eq73_e1912_d_n3 * ddt_scale));
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + (eq73_e1912_d_n4 * ddt_scale));
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + (eq73_e1912_d_n5 * ddt_scale));
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + (eq73_e1912_d_n6 * ddt_scale));
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + (eq73_e1912_d_n7 * ddt_scale));
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + (eq73_e1912_d_n8 * ddt_scale));
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + (eq73_e1912_d_n9 * ddt_scale));
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + (eq73_e1912_d_n10 * ddt_scale));
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + (eq73_e1912_d_n11 * ddt_scale));
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + (eq73_e1912_d_n12 * ddt_scale));
        let eq73_e1917: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq73_e1917_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_3: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq73_e1917_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_3);
        let eq73_e1917_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1917_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1917_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1917_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1917_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1917_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        (eq73_e1918, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1920;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq73_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq73_e1920_d_n3), multiplicity * (eq73_e1920_d_n4), multiplicity * (eq73_e1920_d_n5), multiplicity * (eq73_e1920_d_n6), multiplicity * (eq73_e1920_d_n7), multiplicity * (eq73_e1920_d_n8), multiplicity * (eq73_e1920_d_n9), multiplicity * (eq73_e1920_d_n10), multiplicity * (eq73_e1920_d_n11), multiplicity * (eq73_e1920_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq74_e1947, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12,) = {
    if (((((var_guard1524 != 0.0) && (var_guard1525 != 0.0)) && (var_guard1526 == 0.0)) && (var_guard1527 == 0.0)) && (var_guard1528 == 0.0)) {
        let eq74_e1934: f64 = (-var_ids_1);
        let eq74_e1936: f64 = (eq74_e1934 * var_vds_1);
        let eq74_e1936_d_n3: f64 = ((-var_ids_1_dn3) * var_vds_1);
        let eq74_e1936_d_n4: f64 = ((-var_ids_1_dn4) * var_vds_1);
        let eq74_e1936_d_n5: f64 = ((-var_ids_1_dn5) * var_vds_1);
        let eq74_e1936_d_n6: f64 = ((-var_ids_1_dn6) * var_vds_1);
        let eq74_e1936_d_n7: f64 = (((-var_ids_1_dn7) * var_vds_1) + (eq74_e1934 * var_vds_1_dn7));
        let eq74_e1936_d_n8: f64 = (((-var_ids_1_dn8) * var_vds_1) + (eq74_e1934 * var_vds_1_dn8));
        let eq74_e1936_d_n9: f64 = ((-var_ids_1_dn9) * var_vds_1);
        let eq74_e1936_d_n10: f64 = ((-var_ids_1_dn10) * var_vds_1);
        let eq74_e1936_d_n11: f64 = ((-var_ids_1_dn11) * var_vds_1);
        let eq74_e1936_d_n12: f64 = ((-var_ids_1_dn12) * var_vds_1);
        let eq74_e1939: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq74_e1939_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq74_e1939_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq74_e1939_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq74_e1939_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq74_e1939_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq74_e1939_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq74_e1939_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq74_e1939_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq74_e1939_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq74_e1939_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq74_e1940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq74_e1939);
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1940);
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + (eq74_e1939_d_n3 * ddt_scale));
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + (eq74_e1939_d_n4 * ddt_scale));
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + (eq74_e1939_d_n5 * ddt_scale));
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + (eq74_e1939_d_n6 * ddt_scale));
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + (eq74_e1939_d_n7 * ddt_scale));
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + (eq74_e1939_d_n8 * ddt_scale));
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + (eq74_e1939_d_n9 * ddt_scale));
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + (eq74_e1939_d_n10 * ddt_scale));
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + (eq74_e1939_d_n11 * ddt_scale));
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + (eq74_e1939_d_n12 * ddt_scale));
        let eq74_e1944: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq74_e1944_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_4: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq74_e1944_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_4);
        let eq74_e1944_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1944_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1944_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1944_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1944_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1944_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        (eq74_e1945, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1947;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq74_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq74_e1947_d_n3), multiplicity * (eq74_e1947_d_n4), multiplicity * (eq74_e1947_d_n5), multiplicity * (eq74_e1947_d_n6), multiplicity * (eq74_e1947_d_n7), multiplicity * (eq74_e1947_d_n8), multiplicity * (eq74_e1947_d_n9), multiplicity * (eq74_e1947_d_n10), multiplicity * (eq74_e1947_d_n11), multiplicity * (eq74_e1947_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_deltemp: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_deltemp_dn6: f64,
        var_guard1524: f64,
        var_guard1525: f64,
        var_guard1529: f64,
        var_ids_1: f64,
        var_ids_1_dn10: f64,
        var_ids_1_dn11: f64,
        var_ids_1_dn12: f64,
        var_ids_1_dn3: f64,
        var_ids_1_dn4: f64,
        var_ids_1_dn5: f64,
        var_ids_1_dn6: f64,
        var_ids_1_dn7: f64,
        var_ids_1_dn8: f64,
        var_ids_1_dn9: f64,
        var_pparam_b4soicth: f64,
        var_pparam_b4soicth_dn10: f64,
        var_pparam_b4soicth_dn11: f64,
        var_pparam_b4soicth_dn12: f64,
        var_pparam_b4soicth_dn3: f64,
        var_pparam_b4soicth_dn4: f64,
        var_pparam_b4soicth_dn5: f64,
        var_pparam_b4soicth_dn6: f64,
        var_pparam_b4soicth_dn7: f64,
        var_pparam_b4soicth_dn8: f64,
        var_pparam_b4soicth_dn9: f64,
        var_pparam_b4soirth: f64,
        var_pparam_b4soirth_dn10: f64,
        var_pparam_b4soirth_dn11: f64,
        var_pparam_b4soirth_dn12: f64,
        var_pparam_b4soirth_dn3: f64,
        var_pparam_b4soirth_dn4: f64,
        var_pparam_b4soirth_dn5: f64,
        var_pparam_b4soirth_dn6: f64,
        var_pparam_b4soirth_dn7: f64,
        var_pparam_b4soirth_dn8: f64,
        var_pparam_b4soirth_dn9: f64,
        var_vds_1: f64,
        var_vds_1_dn7: f64,
        var_vds_1_dn8: f64,
    ) {
        let (eq75_e1970, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12,) = {
    if (((var_guard1524 != 0.0) && (var_guard1525 == 0.0)) && (var_guard1529 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (var_ids_1 * __rspice_inv_cse_0);
        let eq75_e1956_d_n3: f64 = (var_ids_1_dn3 * __rspice_inv_cse_0);
        let eq75_e1956_d_n4: f64 = (var_ids_1_dn4 * __rspice_inv_cse_0);
        let eq75_e1956_d_n5: f64 = (var_ids_1_dn5 * __rspice_inv_cse_0);
        let eq75_e1956_d_n6: f64 = (var_ids_1_dn6 * __rspice_inv_cse_0);
        let eq75_e1956_d_n7: f64 = (var_ids_1_dn7 * __rspice_inv_cse_0);
        let eq75_e1956_d_n8: f64 = (var_ids_1_dn8 * __rspice_inv_cse_0);
        let eq75_e1956_d_n9: f64 = (var_ids_1_dn9 * __rspice_inv_cse_0);
        let eq75_e1956_d_n10: f64 = (var_ids_1_dn10 * __rspice_inv_cse_0);
        let eq75_e1956_d_n11: f64 = (var_ids_1_dn11 * __rspice_inv_cse_0);
        let eq75_e1956_d_n12: f64 = (var_ids_1_dn12 * __rspice_inv_cse_0);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * var_vds_1);
        let eq75_e1959_d_n3: f64 = ((-eq75_e1956_d_n3) * var_vds_1);
        let eq75_e1959_d_n4: f64 = ((-eq75_e1956_d_n4) * var_vds_1);
        let eq75_e1959_d_n5: f64 = ((-eq75_e1956_d_n5) * var_vds_1);
        let eq75_e1959_d_n6: f64 = ((-eq75_e1956_d_n6) * var_vds_1);
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * var_vds_1) + (eq75_e1957 * var_vds_1_dn7));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * var_vds_1) + (eq75_e1957 * var_vds_1_dn8));
        let eq75_e1959_d_n9: f64 = ((-eq75_e1956_d_n9) * var_vds_1);
        let eq75_e1959_d_n10: f64 = ((-eq75_e1956_d_n10) * var_vds_1);
        let eq75_e1959_d_n11: f64 = ((-eq75_e1956_d_n11) * var_vds_1);
        let eq75_e1959_d_n12: f64 = ((-eq75_e1956_d_n12) * var_vds_1);
        let eq75_e1962: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq75_e1962_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq75_e1962_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq75_e1962_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq75_e1962_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq75_e1962_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq75_e1962_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq75_e1962_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq75_e1962_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq75_e1962_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq75_e1962_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq75_e1963: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, eq75_e1962);
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1963);
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + (eq75_e1962_d_n3 * ddt_scale));
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + (eq75_e1962_d_n4 * ddt_scale));
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + (eq75_e1962_d_n5 * ddt_scale));
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + (eq75_e1962_d_n6 * ddt_scale));
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + (eq75_e1962_d_n7 * ddt_scale));
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + (eq75_e1962_d_n8 * ddt_scale));
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + (eq75_e1962_d_n9 * ddt_scale));
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + (eq75_e1962_d_n10 * ddt_scale));
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + (eq75_e1962_d_n11 * ddt_scale));
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + (eq75_e1962_d_n12 * ddt_scale));
        let eq75_e1967: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq75_e1967_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_1: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq75_e1967_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_1);
        let eq75_e1967_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1967_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1967_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1967_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1967_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1967_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        (eq75_e1968, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1970;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq75_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq75_e1970_d_n3), multiplicity * (eq75_e1970_d_n4), multiplicity * (eq75_e1970_d_n5), multiplicity * (eq75_e1970_d_n6), multiplicity * (eq75_e1970_d_n7), multiplicity * (eq75_e1970_d_n8), multiplicity * (eq75_e1970_d_n9), multiplicity * (eq75_e1970_d_n10), multiplicity * (eq75_e1970_d_n11), multiplicity * (eq75_e1970_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq76_e1992, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12,) = {
    if (((var_guard1524 != 0.0) && (var_guard1525 == 0.0)) && (var_guard1529 == 0.0)) {
        let eq76_e1979: f64 = (-var_ids_1);
        let eq76_e1981: f64 = (eq76_e1979 * var_vds_1);
        let eq76_e1981_d_n3: f64 = ((-var_ids_1_dn3) * var_vds_1);
        let eq76_e1981_d_n4: f64 = ((-var_ids_1_dn4) * var_vds_1);
        let eq76_e1981_d_n5: f64 = ((-var_ids_1_dn5) * var_vds_1);
        let eq76_e1981_d_n6: f64 = ((-var_ids_1_dn6) * var_vds_1);
        let eq76_e1981_d_n7: f64 = (((-var_ids_1_dn7) * var_vds_1) + (eq76_e1979 * var_vds_1_dn7));
        let eq76_e1981_d_n8: f64 = (((-var_ids_1_dn8) * var_vds_1) + (eq76_e1979 * var_vds_1_dn8));
        let eq76_e1981_d_n9: f64 = ((-var_ids_1_dn9) * var_vds_1);
        let eq76_e1981_d_n10: f64 = ((-var_ids_1_dn10) * var_vds_1);
        let eq76_e1981_d_n11: f64 = ((-var_ids_1_dn11) * var_vds_1);
        let eq76_e1981_d_n12: f64 = ((-var_ids_1_dn12) * var_vds_1);
        let eq76_e1984: f64 = (var_deltemp * var_pparam_b4soicth);
        let eq76_e1984_d_n3: f64 = (var_deltemp * var_pparam_b4soicth_dn3);
        let eq76_e1984_d_n4: f64 = ((var_deltemp_dn4 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn4));
        let eq76_e1984_d_n5: f64 = ((var_deltemp_dn5 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn5));
        let eq76_e1984_d_n6: f64 = ((var_deltemp_dn6 * var_pparam_b4soicth) + (var_deltemp * var_pparam_b4soicth_dn6));
        let eq76_e1984_d_n7: f64 = (var_deltemp * var_pparam_b4soicth_dn7);
        let eq76_e1984_d_n8: f64 = (var_deltemp * var_pparam_b4soicth_dn8);
        let eq76_e1984_d_n9: f64 = (var_deltemp * var_pparam_b4soicth_dn9);
        let eq76_e1984_d_n10: f64 = (var_deltemp * var_pparam_b4soicth_dn10);
        let eq76_e1984_d_n11: f64 = (var_deltemp * var_pparam_b4soicth_dn11);
        let eq76_e1984_d_n12: f64 = (var_deltemp * var_pparam_b4soicth_dn12);
        let eq76_e1985: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq76_e1984);
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1985);
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + (eq76_e1984_d_n3 * ddt_scale));
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + (eq76_e1984_d_n4 * ddt_scale));
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + (eq76_e1984_d_n5 * ddt_scale));
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + (eq76_e1984_d_n6 * ddt_scale));
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + (eq76_e1984_d_n7 * ddt_scale));
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + (eq76_e1984_d_n8 * ddt_scale));
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + (eq76_e1984_d_n9 * ddt_scale));
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + (eq76_e1984_d_n10 * ddt_scale));
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + (eq76_e1984_d_n11 * ddt_scale));
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + (eq76_e1984_d_n12 * ddt_scale));
        let eq76_e1989: f64 = (var_deltemp / var_pparam_b4soirth);
        let eq76_e1989_d_n3: f64 = (-((var_deltemp * var_pparam_b4soirth_dn3) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let __rspice_inv_cse_2: f64 = 1.0 / (var_pparam_b4soirth * var_pparam_b4soirth);
        let eq76_e1989_d_n4: f64 = (((var_deltemp_dn4 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn4)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n5: f64 = (((var_deltemp_dn5 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn5)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n6: f64 = (((var_deltemp_dn6 * var_pparam_b4soirth) - (var_deltemp * var_pparam_b4soirth_dn6)) * __rspice_inv_cse_2);
        let eq76_e1989_d_n7: f64 = (-((var_deltemp * var_pparam_b4soirth_dn7) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1989_d_n8: f64 = (-((var_deltemp * var_pparam_b4soirth_dn8) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1989_d_n9: f64 = (-((var_deltemp * var_pparam_b4soirth_dn9) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1989_d_n10: f64 = (-((var_deltemp * var_pparam_b4soirth_dn10) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1989_d_n11: f64 = (-((var_deltemp * var_pparam_b4soirth_dn11) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1989_d_n12: f64 = (-((var_deltemp * var_pparam_b4soirth_dn12) / (var_pparam_b4soirth * var_pparam_b4soirth)));
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        (eq76_e1990, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1992;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            None,
            multiplicity * (eq76_value),
            [3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            [multiplicity * (eq76_e1992_d_n3), multiplicity * (eq76_e1992_d_n4), multiplicity * (eq76_e1992_d_n5), multiplicity * (eq76_e1992_d_n6), multiplicity * (eq76_e1992_d_n7), multiplicity * (eq76_e1992_d_n8), multiplicity * (eq76_e1992_d_n9), multiplicity * (eq76_e1992_d_n10), multiplicity * (eq76_e1992_d_n11), multiplicity * (eq76_e1992_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17, eq14_e1376_q,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * s.v[1501]);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n0: f64 = ((eq14_e1367 * s.dn[1501][0]) * p.p226);
        let eq14_e1371_d_n1: f64 = ((eq14_e1367 * s.dn[1501][1]) * p.p226);
        let eq14_e1371_d_n2: f64 = ((eq14_e1367 * s.dn[1501][2]) * p.p226);
        let eq14_e1371_d_n3: f64 = ((eq14_e1367 * s.dn[1501][3]) * p.p226);
        let eq14_e1371_d_n4: f64 = ((eq14_e1367 * s.dn[1501][4]) * p.p226);
        let eq14_e1371_d_n5: f64 = ((eq14_e1367 * s.dn[1501][5]) * p.p226);
        let eq14_e1371_d_n6: f64 = ((eq14_e1367 * s.dn[1501][6]) * p.p226);
        let eq14_e1371_d_n7: f64 = ((eq14_e1367 * s.dn[1501][7]) * p.p226);
        let eq14_e1371_d_n8: f64 = ((eq14_e1367 * s.dn[1501][8]) * p.p226);
        let eq14_e1371_d_n9: f64 = ((eq14_e1367 * s.dn[1501][9]) * p.p226);
        let eq14_e1371_d_n10: f64 = ((eq14_e1367 * s.dn[1501][10]) * p.p226);
        let eq14_e1371_d_n11: f64 = ((eq14_e1367 * s.dn[1501][11]) * p.p226);
        let eq14_e1371_d_n12: f64 = ((eq14_e1367 * s.dn[1501][12]) * p.p226);
        let eq14_e1371_d_n13: f64 = ((eq14_e1367 * s.dn[1501][13]) * p.p226);
        let eq14_e1371_d_b0: f64 = ((eq14_e1367 * s.db[1501][0]) * p.p226);
        let eq14_e1371_d_b1: f64 = ((eq14_e1367 * s.db[1501][1]) * p.p226);
        let eq14_e1371_d_b2: f64 = ((eq14_e1367 * s.db[1501][2]) * p.p226);
        let eq14_e1371_d_b3: f64 = ((eq14_e1367 * s.db[1501][3]) * p.p226);
        let eq14_e1371_d_b4: f64 = ((eq14_e1367 * s.db[1501][4]) * p.p226);
        let eq14_e1371_d_b5: f64 = ((eq14_e1367 * s.db[1501][5]) * p.p226);
        let eq14_e1371_d_b6: f64 = ((eq14_e1367 * s.db[1501][6]) * p.p226);
        let eq14_e1371_d_b7: f64 = ((eq14_e1367 * s.db[1501][7]) * p.p226);
        let eq14_e1371_d_b8: f64 = ((eq14_e1367 * s.db[1501][8]) * p.p226);
        let eq14_e1371_d_b9: f64 = ((eq14_e1367 * s.db[1501][9]) * p.p226);
        let eq14_e1371_d_b10: f64 = ((eq14_e1367 * s.db[1501][10]) * p.p226);
        let eq14_e1371_d_b11: f64 = ((eq14_e1367 * s.db[1501][11]) * p.p226);
        let eq14_e1371_d_b12: f64 = ((eq14_e1367 * s.db[1501][12]) * p.p226);
        let eq14_e1371_d_b13: f64 = ((eq14_e1367 * s.db[1501][13]) * p.p226);
        let eq14_e1371_d_b14: f64 = ((eq14_e1367 * s.db[1501][14]) * p.p226);
        let eq14_e1371_d_b15: f64 = ((eq14_e1367 * s.db[1501][15]) * p.p226);
        let eq14_e1371_d_b16: f64 = ((eq14_e1367 * s.db[1501][16]) * p.p226);
        let eq14_e1371_d_b17: f64 = ((eq14_e1367 * s.db[1501][17]) * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n0: f64 = (eq14_e1371_d_n0 * (nv13 - 0.0));
        let eq14_e1373_d_n1: f64 = (eq14_e1371_d_n1 * (nv13 - 0.0));
        let eq14_e1373_d_n2: f64 = (eq14_e1371_d_n2 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1373_d_n13: f64 = ((eq14_e1371_d_n13 * (nv13 - 0.0)) + eq14_e1371);
        let eq14_e1373_d_b0: f64 = (eq14_e1371_d_b0 * (nv13 - 0.0));
        let eq14_e1373_d_b1: f64 = (eq14_e1371_d_b1 * (nv13 - 0.0));
        let eq14_e1373_d_b2: f64 = (eq14_e1371_d_b2 * (nv13 - 0.0));
        let eq14_e1373_d_b3: f64 = (eq14_e1371_d_b3 * (nv13 - 0.0));
        let eq14_e1373_d_b4: f64 = (eq14_e1371_d_b4 * (nv13 - 0.0));
        let eq14_e1373_d_b5: f64 = (eq14_e1371_d_b5 * (nv13 - 0.0));
        let eq14_e1373_d_b6: f64 = (eq14_e1371_d_b6 * (nv13 - 0.0));
        let eq14_e1373_d_b7: f64 = (eq14_e1371_d_b7 * (nv13 - 0.0));
        let eq14_e1373_d_b8: f64 = (eq14_e1371_d_b8 * (nv13 - 0.0));
        let eq14_e1373_d_b9: f64 = (eq14_e1371_d_b9 * (nv13 - 0.0));
        let eq14_e1373_d_b10: f64 = (eq14_e1371_d_b10 * (nv13 - 0.0));
        let eq14_e1373_d_b11: f64 = (eq14_e1371_d_b11 * (nv13 - 0.0));
        let eq14_e1373_d_b12: f64 = (eq14_e1371_d_b12 * (nv13 - 0.0));
        let eq14_e1373_d_b13: f64 = (eq14_e1371_d_b13 * (nv13 - 0.0));
        let eq14_e1373_d_b14: f64 = (eq14_e1371_d_b14 * (nv13 - 0.0));
        let eq14_e1373_d_b15: f64 = (eq14_e1371_d_b15 * (nv13 - 0.0));
        let eq14_e1373_d_b16: f64 = (eq14_e1371_d_b16 * (nv13 - 0.0));
        let eq14_e1373_d_b17: f64 = (eq14_e1371_d_b17 * (nv13 - 0.0));
        let eq14_e1374_q: f64 = eq14_e1373;
        (eq14_e1373, eq14_e1373_d_n0, eq14_e1373_d_n1, eq14_e1373_d_n2, eq14_e1373_d_n3, eq14_e1373_d_n4, eq14_e1373_d_n5, eq14_e1373_d_n6, eq14_e1373_d_n7, eq14_e1373_d_n8, eq14_e1373_d_n9, eq14_e1373_d_n10, eq14_e1373_d_n11, eq14_e1373_d_n12, eq14_e1373_d_n13, eq14_e1373_d_b0, eq14_e1373_d_b1, eq14_e1373_d_b2, eq14_e1373_d_b3, eq14_e1373_d_b4, eq14_e1373_d_b5, eq14_e1373_d_b6, eq14_e1373_d_b7, eq14_e1373_d_b8, eq14_e1373_d_b9, eq14_e1373_d_b10, eq14_e1373_d_b11, eq14_e1373_d_b12, eq14_e1373_d_b13, eq14_e1373_d_b14, eq14_e1373_d_b15, eq14_e1373_d_b16, eq14_e1373_d_b17, eq14_e1374_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_reactive_node_derivatives: [f64; 14] = [eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_reactive_branch_derivatives: [f64; 18] = [eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17, eq15_e1396_q,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * s.v[1501]);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n0: f64 = ((eq15_e1387 * s.dn[1501][0]) * p.p226);
        let eq15_e1391_d_n1: f64 = ((eq15_e1387 * s.dn[1501][1]) * p.p226);
        let eq15_e1391_d_n2: f64 = ((eq15_e1387 * s.dn[1501][2]) * p.p226);
        let eq15_e1391_d_n3: f64 = ((eq15_e1387 * s.dn[1501][3]) * p.p226);
        let eq15_e1391_d_n4: f64 = ((eq15_e1387 * s.dn[1501][4]) * p.p226);
        let eq15_e1391_d_n5: f64 = ((eq15_e1387 * s.dn[1501][5]) * p.p226);
        let eq15_e1391_d_n6: f64 = ((eq15_e1387 * s.dn[1501][6]) * p.p226);
        let eq15_e1391_d_n7: f64 = ((eq15_e1387 * s.dn[1501][7]) * p.p226);
        let eq15_e1391_d_n8: f64 = ((eq15_e1387 * s.dn[1501][8]) * p.p226);
        let eq15_e1391_d_n9: f64 = ((eq15_e1387 * s.dn[1501][9]) * p.p226);
        let eq15_e1391_d_n10: f64 = ((eq15_e1387 * s.dn[1501][10]) * p.p226);
        let eq15_e1391_d_n11: f64 = ((eq15_e1387 * s.dn[1501][11]) * p.p226);
        let eq15_e1391_d_n12: f64 = ((eq15_e1387 * s.dn[1501][12]) * p.p226);
        let eq15_e1391_d_n13: f64 = ((eq15_e1387 * s.dn[1501][13]) * p.p226);
        let eq15_e1391_d_b0: f64 = ((eq15_e1387 * s.db[1501][0]) * p.p226);
        let eq15_e1391_d_b1: f64 = ((eq15_e1387 * s.db[1501][1]) * p.p226);
        let eq15_e1391_d_b2: f64 = ((eq15_e1387 * s.db[1501][2]) * p.p226);
        let eq15_e1391_d_b3: f64 = ((eq15_e1387 * s.db[1501][3]) * p.p226);
        let eq15_e1391_d_b4: f64 = ((eq15_e1387 * s.db[1501][4]) * p.p226);
        let eq15_e1391_d_b5: f64 = ((eq15_e1387 * s.db[1501][5]) * p.p226);
        let eq15_e1391_d_b6: f64 = ((eq15_e1387 * s.db[1501][6]) * p.p226);
        let eq15_e1391_d_b7: f64 = ((eq15_e1387 * s.db[1501][7]) * p.p226);
        let eq15_e1391_d_b8: f64 = ((eq15_e1387 * s.db[1501][8]) * p.p226);
        let eq15_e1391_d_b9: f64 = ((eq15_e1387 * s.db[1501][9]) * p.p226);
        let eq15_e1391_d_b10: f64 = ((eq15_e1387 * s.db[1501][10]) * p.p226);
        let eq15_e1391_d_b11: f64 = ((eq15_e1387 * s.db[1501][11]) * p.p226);
        let eq15_e1391_d_b12: f64 = ((eq15_e1387 * s.db[1501][12]) * p.p226);
        let eq15_e1391_d_b13: f64 = ((eq15_e1387 * s.db[1501][13]) * p.p226);
        let eq15_e1391_d_b14: f64 = ((eq15_e1387 * s.db[1501][14]) * p.p226);
        let eq15_e1391_d_b15: f64 = ((eq15_e1387 * s.db[1501][15]) * p.p226);
        let eq15_e1391_d_b16: f64 = ((eq15_e1387 * s.db[1501][16]) * p.p226);
        let eq15_e1391_d_b17: f64 = ((eq15_e1387 * s.db[1501][17]) * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n0: f64 = (eq15_e1391_d_n0 * (nv13 - 0.0));
        let eq15_e1393_d_n1: f64 = (eq15_e1391_d_n1 * (nv13 - 0.0));
        let eq15_e1393_d_n2: f64 = (eq15_e1391_d_n2 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1393_d_n13: f64 = ((eq15_e1391_d_n13 * (nv13 - 0.0)) + eq15_e1391);
        let eq15_e1393_d_b0: f64 = (eq15_e1391_d_b0 * (nv13 - 0.0));
        let eq15_e1393_d_b1: f64 = (eq15_e1391_d_b1 * (nv13 - 0.0));
        let eq15_e1393_d_b2: f64 = (eq15_e1391_d_b2 * (nv13 - 0.0));
        let eq15_e1393_d_b3: f64 = (eq15_e1391_d_b3 * (nv13 - 0.0));
        let eq15_e1393_d_b4: f64 = (eq15_e1391_d_b4 * (nv13 - 0.0));
        let eq15_e1393_d_b5: f64 = (eq15_e1391_d_b5 * (nv13 - 0.0));
        let eq15_e1393_d_b6: f64 = (eq15_e1391_d_b6 * (nv13 - 0.0));
        let eq15_e1393_d_b7: f64 = (eq15_e1391_d_b7 * (nv13 - 0.0));
        let eq15_e1393_d_b8: f64 = (eq15_e1391_d_b8 * (nv13 - 0.0));
        let eq15_e1393_d_b9: f64 = (eq15_e1391_d_b9 * (nv13 - 0.0));
        let eq15_e1393_d_b10: f64 = (eq15_e1391_d_b10 * (nv13 - 0.0));
        let eq15_e1393_d_b11: f64 = (eq15_e1391_d_b11 * (nv13 - 0.0));
        let eq15_e1393_d_b12: f64 = (eq15_e1391_d_b12 * (nv13 - 0.0));
        let eq15_e1393_d_b13: f64 = (eq15_e1391_d_b13 * (nv13 - 0.0));
        let eq15_e1393_d_b14: f64 = (eq15_e1391_d_b14 * (nv13 - 0.0));
        let eq15_e1393_d_b15: f64 = (eq15_e1391_d_b15 * (nv13 - 0.0));
        let eq15_e1393_d_b16: f64 = (eq15_e1391_d_b16 * (nv13 - 0.0));
        let eq15_e1393_d_b17: f64 = (eq15_e1391_d_b17 * (nv13 - 0.0));
        let eq15_e1394_q: f64 = eq15_e1393;
        (eq15_e1393, eq15_e1393_d_n0, eq15_e1393_d_n1, eq15_e1393_d_n2, eq15_e1393_d_n3, eq15_e1393_d_n4, eq15_e1393_d_n5, eq15_e1393_d_n6, eq15_e1393_d_n7, eq15_e1393_d_n8, eq15_e1393_d_n9, eq15_e1393_d_n10, eq15_e1393_d_n11, eq15_e1393_d_n12, eq15_e1393_d_n13, eq15_e1393_d_b0, eq15_e1393_d_b1, eq15_e1393_d_b2, eq15_e1393_d_b3, eq15_e1393_d_b4, eq15_e1393_d_b5, eq15_e1393_d_b6, eq15_e1393_d_b7, eq15_e1393_d_b8, eq15_e1393_d_b9, eq15_e1393_d_b10, eq15_e1393_d_b11, eq15_e1393_d_b12, eq15_e1393_d_b13, eq15_e1393_d_b14, eq15_e1393_d_b15, eq15_e1393_d_b16, eq15_e1393_d_b17, eq15_e1394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_reactive_node_derivatives: [f64; 14] = [eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_reactive_branch_derivatives: [f64; 18] = [eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1647: f64 = (p.p33 * s.v[92]);
        let eq44_e1648_q: f64 = eq44_e1647;
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &s.dn[92],
            branches,
            &s.db[92],
            (multiplicity) * (p.p33),
        );
        let eq45_e1651: f64 = (p.p33 * s.v[93]);
        let eq45_e1652_q: f64 = eq45_e1651;
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            nodes,
            &s.dn[93],
            branches,
            &s.db[93],
            (multiplicity) * (p.p33),
        );
        let eq46_e1656: f64 = (p.p33 * s.v[916]);
        let eq46_e1657_q: f64 = eq46_e1656;
        let eq46_e1658: f64 = (p.p37 * eq46_e1656);
        let eq46_e1658_d_n0: f64 = (p.p37 * (p.p33 * s.dn[916][0]));
        let eq46_e1658_d_n1: f64 = (p.p37 * (p.p33 * s.dn[916][1]));
        let eq46_e1658_d_n2: f64 = (p.p37 * (p.p33 * s.dn[916][2]));
        let eq46_e1658_d_n3: f64 = (p.p37 * (p.p33 * s.dn[916][3]));
        let eq46_e1658_d_n4: f64 = (p.p37 * (p.p33 * s.dn[916][4]));
        let eq46_e1658_d_n5: f64 = (p.p37 * (p.p33 * s.dn[916][5]));
        let eq46_e1658_d_n6: f64 = (p.p37 * (p.p33 * s.dn[916][6]));
        let eq46_e1658_d_n7: f64 = (p.p37 * (p.p33 * s.dn[916][7]));
        let eq46_e1658_d_n8: f64 = (p.p37 * (p.p33 * s.dn[916][8]));
        let eq46_e1658_d_n9: f64 = (p.p37 * (p.p33 * s.dn[916][9]));
        let eq46_e1658_d_n10: f64 = (p.p37 * (p.p33 * s.dn[916][10]));
        let eq46_e1658_d_n11: f64 = (p.p37 * (p.p33 * s.dn[916][11]));
        let eq46_e1658_d_n12: f64 = (p.p37 * (p.p33 * s.dn[916][12]));
        let eq46_e1658_d_n13: f64 = (p.p37 * (p.p33 * s.dn[916][13]));
        let eq46_e1658_d_b0: f64 = (p.p37 * (p.p33 * s.db[916][0]));
        let eq46_e1658_d_b1: f64 = (p.p37 * (p.p33 * s.db[916][1]));
        let eq46_e1658_d_b2: f64 = (p.p37 * (p.p33 * s.db[916][2]));
        let eq46_e1658_d_b3: f64 = (p.p37 * (p.p33 * s.db[916][3]));
        let eq46_e1658_d_b4: f64 = (p.p37 * (p.p33 * s.db[916][4]));
        let eq46_e1658_d_b5: f64 = (p.p37 * (p.p33 * s.db[916][5]));
        let eq46_e1658_d_b6: f64 = (p.p37 * (p.p33 * s.db[916][6]));
        let eq46_e1658_d_b7: f64 = (p.p37 * (p.p33 * s.db[916][7]));
        let eq46_e1658_d_b8: f64 = (p.p37 * (p.p33 * s.db[916][8]));
        let eq46_e1658_d_b9: f64 = (p.p37 * (p.p33 * s.db[916][9]));
        let eq46_e1658_d_b10: f64 = (p.p37 * (p.p33 * s.db[916][10]));
        let eq46_e1658_d_b11: f64 = (p.p37 * (p.p33 * s.db[916][11]));
        let eq46_e1658_d_b12: f64 = (p.p37 * (p.p33 * s.db[916][12]));
        let eq46_e1658_d_b13: f64 = (p.p37 * (p.p33 * s.db[916][13]));
        let eq46_e1658_d_b14: f64 = (p.p37 * (p.p33 * s.db[916][14]));
        let eq46_e1658_d_b15: f64 = (p.p37 * (p.p33 * s.db[916][15]));
        let eq46_e1658_d_b16: f64 = (p.p37 * (p.p33 * s.db[916][16]));
        let eq46_e1658_d_b17: f64 = (p.p37 * (p.p33 * s.db[916][17]));
        let eq46_e1658_q: f64 = (p.p37 * eq46_e1657_q);
        let eq46_reactive_node_derivatives: [f64; 14] = [eq46_e1658_d_n0, eq46_e1658_d_n1, eq46_e1658_d_n2, eq46_e1658_d_n3, eq46_e1658_d_n4, eq46_e1658_d_n5, eq46_e1658_d_n6, eq46_e1658_d_n7, eq46_e1658_d_n8, eq46_e1658_d_n9, eq46_e1658_d_n10, eq46_e1658_d_n11, eq46_e1658_d_n12, eq46_e1658_d_n13];
        let eq46_reactive_branch_derivatives: [f64; 18] = [eq46_e1658_d_b0, eq46_e1658_d_b1, eq46_e1658_d_b2, eq46_e1658_d_b3, eq46_e1658_d_b4, eq46_e1658_d_b5, eq46_e1658_d_b6, eq46_e1658_d_b7, eq46_e1658_d_b8, eq46_e1658_d_b9, eq46_e1658_d_b10, eq46_e1658_d_b11, eq46_e1658_d_b12, eq46_e1658_d_b13, eq46_e1658_d_b14, eq46_e1658_d_b15, eq46_e1658_d_b16, eq46_e1658_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let eq47_e1662: f64 = (p.p33 * s.v[920]);
        let eq47_e1663_q: f64 = eq47_e1662;
        let eq47_e1664: f64 = (p.p37 * eq47_e1662);
        let eq47_e1664_d_n0: f64 = (p.p37 * (p.p33 * s.dn[920][0]));
        let eq47_e1664_d_n1: f64 = (p.p37 * (p.p33 * s.dn[920][1]));
        let eq47_e1664_d_n2: f64 = (p.p37 * (p.p33 * s.dn[920][2]));
        let eq47_e1664_d_n3: f64 = (p.p37 * (p.p33 * s.dn[920][3]));
        let eq47_e1664_d_n4: f64 = (p.p37 * (p.p33 * s.dn[920][4]));
        let eq47_e1664_d_n5: f64 = (p.p37 * (p.p33 * s.dn[920][5]));
        let eq47_e1664_d_n6: f64 = (p.p37 * (p.p33 * s.dn[920][6]));
        let eq47_e1664_d_n7: f64 = (p.p37 * (p.p33 * s.dn[920][7]));
        let eq47_e1664_d_n8: f64 = (p.p37 * (p.p33 * s.dn[920][8]));
        let eq47_e1664_d_n9: f64 = (p.p37 * (p.p33 * s.dn[920][9]));
        let eq47_e1664_d_n10: f64 = (p.p37 * (p.p33 * s.dn[920][10]));
        let eq47_e1664_d_n11: f64 = (p.p37 * (p.p33 * s.dn[920][11]));
        let eq47_e1664_d_n12: f64 = (p.p37 * (p.p33 * s.dn[920][12]));
        let eq47_e1664_d_n13: f64 = (p.p37 * (p.p33 * s.dn[920][13]));
        let eq47_e1664_d_b0: f64 = (p.p37 * (p.p33 * s.db[920][0]));
        let eq47_e1664_d_b1: f64 = (p.p37 * (p.p33 * s.db[920][1]));
        let eq47_e1664_d_b2: f64 = (p.p37 * (p.p33 * s.db[920][2]));
        let eq47_e1664_d_b3: f64 = (p.p37 * (p.p33 * s.db[920][3]));
        let eq47_e1664_d_b4: f64 = (p.p37 * (p.p33 * s.db[920][4]));
        let eq47_e1664_d_b5: f64 = (p.p37 * (p.p33 * s.db[920][5]));
        let eq47_e1664_d_b6: f64 = (p.p37 * (p.p33 * s.db[920][6]));
        let eq47_e1664_d_b7: f64 = (p.p37 * (p.p33 * s.db[920][7]));
        let eq47_e1664_d_b8: f64 = (p.p37 * (p.p33 * s.db[920][8]));
        let eq47_e1664_d_b9: f64 = (p.p37 * (p.p33 * s.db[920][9]));
        let eq47_e1664_d_b10: f64 = (p.p37 * (p.p33 * s.db[920][10]));
        let eq47_e1664_d_b11: f64 = (p.p37 * (p.p33 * s.db[920][11]));
        let eq47_e1664_d_b12: f64 = (p.p37 * (p.p33 * s.db[920][12]));
        let eq47_e1664_d_b13: f64 = (p.p37 * (p.p33 * s.db[920][13]));
        let eq47_e1664_d_b14: f64 = (p.p37 * (p.p33 * s.db[920][14]));
        let eq47_e1664_d_b15: f64 = (p.p37 * (p.p33 * s.db[920][15]));
        let eq47_e1664_d_b16: f64 = (p.p37 * (p.p33 * s.db[920][16]));
        let eq47_e1664_d_b17: f64 = (p.p37 * (p.p33 * s.db[920][17]));
        let eq47_e1664_q: f64 = (p.p37 * eq47_e1663_q);
        let eq47_reactive_node_derivatives: [f64; 14] = [eq47_e1664_d_n0, eq47_e1664_d_n1, eq47_e1664_d_n2, eq47_e1664_d_n3, eq47_e1664_d_n4, eq47_e1664_d_n5, eq47_e1664_d_n6, eq47_e1664_d_n7, eq47_e1664_d_n8, eq47_e1664_d_n9, eq47_e1664_d_n10, eq47_e1664_d_n11, eq47_e1664_d_n12, eq47_e1664_d_n13];
        let eq47_reactive_branch_derivatives: [f64; 18] = [eq47_e1664_d_b0, eq47_e1664_d_b1, eq47_e1664_d_b2, eq47_e1664_d_b3, eq47_e1664_d_b4, eq47_e1664_d_b5, eq47_e1664_d_b6, eq47_e1664_d_b7, eq47_e1664_d_b8, eq47_e1664_d_b9, eq47_e1664_d_b10, eq47_e1664_d_b11, eq47_e1664_d_b12, eq47_e1664_d_b13, eq47_e1664_d_b14, eq47_e1664_d_b15, eq47_e1664_d_b16, eq47_e1664_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1668: f64 = (p.p33 * s.v[909]);
        let eq48_e1669_q: f64 = eq48_e1668;
        let eq48_e1670: f64 = (p.p37 * eq48_e1668);
        let eq48_e1670_d_n0: f64 = (p.p37 * (p.p33 * s.dn[909][0]));
        let eq48_e1670_d_n1: f64 = (p.p37 * (p.p33 * s.dn[909][1]));
        let eq48_e1670_d_n2: f64 = (p.p37 * (p.p33 * s.dn[909][2]));
        let eq48_e1670_d_n3: f64 = (p.p37 * (p.p33 * s.dn[909][3]));
        let eq48_e1670_d_n4: f64 = (p.p37 * (p.p33 * s.dn[909][4]));
        let eq48_e1670_d_n5: f64 = (p.p37 * (p.p33 * s.dn[909][5]));
        let eq48_e1670_d_n6: f64 = (p.p37 * (p.p33 * s.dn[909][6]));
        let eq48_e1670_d_n7: f64 = (p.p37 * (p.p33 * s.dn[909][7]));
        let eq48_e1670_d_n8: f64 = (p.p37 * (p.p33 * s.dn[909][8]));
        let eq48_e1670_d_n9: f64 = (p.p37 * (p.p33 * s.dn[909][9]));
        let eq48_e1670_d_n10: f64 = (p.p37 * (p.p33 * s.dn[909][10]));
        let eq48_e1670_d_n11: f64 = (p.p37 * (p.p33 * s.dn[909][11]));
        let eq48_e1670_d_n12: f64 = (p.p37 * (p.p33 * s.dn[909][12]));
        let eq48_e1670_d_n13: f64 = (p.p37 * (p.p33 * s.dn[909][13]));
        let eq48_e1670_d_b0: f64 = (p.p37 * (p.p33 * s.db[909][0]));
        let eq48_e1670_d_b1: f64 = (p.p37 * (p.p33 * s.db[909][1]));
        let eq48_e1670_d_b2: f64 = (p.p37 * (p.p33 * s.db[909][2]));
        let eq48_e1670_d_b3: f64 = (p.p37 * (p.p33 * s.db[909][3]));
        let eq48_e1670_d_b4: f64 = (p.p37 * (p.p33 * s.db[909][4]));
        let eq48_e1670_d_b5: f64 = (p.p37 * (p.p33 * s.db[909][5]));
        let eq48_e1670_d_b6: f64 = (p.p37 * (p.p33 * s.db[909][6]));
        let eq48_e1670_d_b7: f64 = (p.p37 * (p.p33 * s.db[909][7]));
        let eq48_e1670_d_b8: f64 = (p.p37 * (p.p33 * s.db[909][8]));
        let eq48_e1670_d_b9: f64 = (p.p37 * (p.p33 * s.db[909][9]));
        let eq48_e1670_d_b10: f64 = (p.p37 * (p.p33 * s.db[909][10]));
        let eq48_e1670_d_b11: f64 = (p.p37 * (p.p33 * s.db[909][11]));
        let eq48_e1670_d_b12: f64 = (p.p37 * (p.p33 * s.db[909][12]));
        let eq48_e1670_d_b13: f64 = (p.p37 * (p.p33 * s.db[909][13]));
        let eq48_e1670_d_b14: f64 = (p.p37 * (p.p33 * s.db[909][14]));
        let eq48_e1670_d_b15: f64 = (p.p37 * (p.p33 * s.db[909][15]));
        let eq48_e1670_d_b16: f64 = (p.p37 * (p.p33 * s.db[909][16]));
        let eq48_e1670_d_b17: f64 = (p.p37 * (p.p33 * s.db[909][17]));
        let eq48_e1670_q: f64 = (p.p37 * eq48_e1669_q);
        let eq48_reactive_node_derivatives: [f64; 14] = [eq48_e1670_d_n0, eq48_e1670_d_n1, eq48_e1670_d_n2, eq48_e1670_d_n3, eq48_e1670_d_n4, eq48_e1670_d_n5, eq48_e1670_d_n6, eq48_e1670_d_n7, eq48_e1670_d_n8, eq48_e1670_d_n9, eq48_e1670_d_n10, eq48_e1670_d_n11, eq48_e1670_d_n12, eq48_e1670_d_n13];
        let eq48_reactive_branch_derivatives: [f64; 18] = [eq48_e1670_d_b0, eq48_e1670_d_b1, eq48_e1670_d_b2, eq48_e1670_d_b3, eq48_e1670_d_b4, eq48_e1670_d_b5, eq48_e1670_d_b6, eq48_e1670_d_b7, eq48_e1670_d_b8, eq48_e1670_d_b9, eq48_e1670_d_b10, eq48_e1670_d_b11, eq48_e1670_d_b12, eq48_e1670_d_b13, eq48_e1670_d_b14, eq48_e1670_d_b15, eq48_e1670_d_b16, eq48_e1670_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1674: f64 = (p.p33 * s.v[910]);
        let eq49_e1675_q: f64 = eq49_e1674;
        let eq49_e1676: f64 = (p.p37 * eq49_e1674);
        let eq49_e1676_d_n0: f64 = (p.p37 * (p.p33 * s.dn[910][0]));
        let eq49_e1676_d_n1: f64 = (p.p37 * (p.p33 * s.dn[910][1]));
        let eq49_e1676_d_n2: f64 = (p.p37 * (p.p33 * s.dn[910][2]));
        let eq49_e1676_d_n3: f64 = (p.p37 * (p.p33 * s.dn[910][3]));
        let eq49_e1676_d_n4: f64 = (p.p37 * (p.p33 * s.dn[910][4]));
        let eq49_e1676_d_n5: f64 = (p.p37 * (p.p33 * s.dn[910][5]));
        let eq49_e1676_d_n6: f64 = (p.p37 * (p.p33 * s.dn[910][6]));
        let eq49_e1676_d_n7: f64 = (p.p37 * (p.p33 * s.dn[910][7]));
        let eq49_e1676_d_n8: f64 = (p.p37 * (p.p33 * s.dn[910][8]));
        let eq49_e1676_d_n9: f64 = (p.p37 * (p.p33 * s.dn[910][9]));
        let eq49_e1676_d_n10: f64 = (p.p37 * (p.p33 * s.dn[910][10]));
        let eq49_e1676_d_n11: f64 = (p.p37 * (p.p33 * s.dn[910][11]));
        let eq49_e1676_d_n12: f64 = (p.p37 * (p.p33 * s.dn[910][12]));
        let eq49_e1676_d_n13: f64 = (p.p37 * (p.p33 * s.dn[910][13]));
        let eq49_e1676_d_b0: f64 = (p.p37 * (p.p33 * s.db[910][0]));
        let eq49_e1676_d_b1: f64 = (p.p37 * (p.p33 * s.db[910][1]));
        let eq49_e1676_d_b2: f64 = (p.p37 * (p.p33 * s.db[910][2]));
        let eq49_e1676_d_b3: f64 = (p.p37 * (p.p33 * s.db[910][3]));
        let eq49_e1676_d_b4: f64 = (p.p37 * (p.p33 * s.db[910][4]));
        let eq49_e1676_d_b5: f64 = (p.p37 * (p.p33 * s.db[910][5]));
        let eq49_e1676_d_b6: f64 = (p.p37 * (p.p33 * s.db[910][6]));
        let eq49_e1676_d_b7: f64 = (p.p37 * (p.p33 * s.db[910][7]));
        let eq49_e1676_d_b8: f64 = (p.p37 * (p.p33 * s.db[910][8]));
        let eq49_e1676_d_b9: f64 = (p.p37 * (p.p33 * s.db[910][9]));
        let eq49_e1676_d_b10: f64 = (p.p37 * (p.p33 * s.db[910][10]));
        let eq49_e1676_d_b11: f64 = (p.p37 * (p.p33 * s.db[910][11]));
        let eq49_e1676_d_b12: f64 = (p.p37 * (p.p33 * s.db[910][12]));
        let eq49_e1676_d_b13: f64 = (p.p37 * (p.p33 * s.db[910][13]));
        let eq49_e1676_d_b14: f64 = (p.p37 * (p.p33 * s.db[910][14]));
        let eq49_e1676_d_b15: f64 = (p.p37 * (p.p33 * s.db[910][15]));
        let eq49_e1676_d_b16: f64 = (p.p37 * (p.p33 * s.db[910][16]));
        let eq49_e1676_d_b17: f64 = (p.p37 * (p.p33 * s.db[910][17]));
        let eq49_e1676_q: f64 = (p.p37 * eq49_e1675_q);
        let eq49_reactive_node_derivatives: [f64; 14] = [eq49_e1676_d_n0, eq49_e1676_d_n1, eq49_e1676_d_n2, eq49_e1676_d_n3, eq49_e1676_d_n4, eq49_e1676_d_n5, eq49_e1676_d_n6, eq49_e1676_d_n7, eq49_e1676_d_n8, eq49_e1676_d_n9, eq49_e1676_d_n10, eq49_e1676_d_n11, eq49_e1676_d_n12, eq49_e1676_d_n13];
        let eq49_reactive_branch_derivatives: [f64; 18] = [eq49_e1676_d_b0, eq49_e1676_d_b1, eq49_e1676_d_b2, eq49_e1676_d_b3, eq49_e1676_d_b4, eq49_e1676_d_b5, eq49_e1676_d_b6, eq49_e1676_d_b7, eq49_e1676_d_b8, eq49_e1676_d_b9, eq49_e1676_d_b10, eq49_e1676_d_b11, eq49_e1676_d_b12, eq49_e1676_d_b13, eq49_e1676_d_b14, eq49_e1676_d_b15, eq49_e1676_d_b16, eq49_e1676_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[8]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1685, eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13, eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17, eq50_e1685_q,) = {
    if s.b[1553] {
        let eq50_e1681: f64 = (p.p33 * s.v[895]);
        let eq50_e1682_q: f64 = eq50_e1681;
        let eq50_e1683: f64 = (p.p37 * eq50_e1681);
        let eq50_e1683_d_n0: f64 = (p.p37 * (p.p33 * s.dn[895][0]));
        let eq50_e1683_d_n1: f64 = (p.p37 * (p.p33 * s.dn[895][1]));
        let eq50_e1683_d_n2: f64 = (p.p37 * (p.p33 * s.dn[895][2]));
        let eq50_e1683_d_n3: f64 = (p.p37 * (p.p33 * s.dn[895][3]));
        let eq50_e1683_d_n4: f64 = (p.p37 * (p.p33 * s.dn[895][4]));
        let eq50_e1683_d_n5: f64 = (p.p37 * (p.p33 * s.dn[895][5]));
        let eq50_e1683_d_n6: f64 = (p.p37 * (p.p33 * s.dn[895][6]));
        let eq50_e1683_d_n7: f64 = (p.p37 * (p.p33 * s.dn[895][7]));
        let eq50_e1683_d_n8: f64 = (p.p37 * (p.p33 * s.dn[895][8]));
        let eq50_e1683_d_n9: f64 = (p.p37 * (p.p33 * s.dn[895][9]));
        let eq50_e1683_d_n10: f64 = (p.p37 * (p.p33 * s.dn[895][10]));
        let eq50_e1683_d_n11: f64 = (p.p37 * (p.p33 * s.dn[895][11]));
        let eq50_e1683_d_n12: f64 = (p.p37 * (p.p33 * s.dn[895][12]));
        let eq50_e1683_d_n13: f64 = (p.p37 * (p.p33 * s.dn[895][13]));
        let eq50_e1683_d_b0: f64 = (p.p37 * (p.p33 * s.db[895][0]));
        let eq50_e1683_d_b1: f64 = (p.p37 * (p.p33 * s.db[895][1]));
        let eq50_e1683_d_b2: f64 = (p.p37 * (p.p33 * s.db[895][2]));
        let eq50_e1683_d_b3: f64 = (p.p37 * (p.p33 * s.db[895][3]));
        let eq50_e1683_d_b4: f64 = (p.p37 * (p.p33 * s.db[895][4]));
        let eq50_e1683_d_b5: f64 = (p.p37 * (p.p33 * s.db[895][5]));
        let eq50_e1683_d_b6: f64 = (p.p37 * (p.p33 * s.db[895][6]));
        let eq50_e1683_d_b7: f64 = (p.p37 * (p.p33 * s.db[895][7]));
        let eq50_e1683_d_b8: f64 = (p.p37 * (p.p33 * s.db[895][8]));
        let eq50_e1683_d_b9: f64 = (p.p37 * (p.p33 * s.db[895][9]));
        let eq50_e1683_d_b10: f64 = (p.p37 * (p.p33 * s.db[895][10]));
        let eq50_e1683_d_b11: f64 = (p.p37 * (p.p33 * s.db[895][11]));
        let eq50_e1683_d_b12: f64 = (p.p37 * (p.p33 * s.db[895][12]));
        let eq50_e1683_d_b13: f64 = (p.p37 * (p.p33 * s.db[895][13]));
        let eq50_e1683_d_b14: f64 = (p.p37 * (p.p33 * s.db[895][14]));
        let eq50_e1683_d_b15: f64 = (p.p37 * (p.p33 * s.db[895][15]));
        let eq50_e1683_d_b16: f64 = (p.p37 * (p.p33 * s.db[895][16]));
        let eq50_e1683_d_b17: f64 = (p.p37 * (p.p33 * s.db[895][17]));
        let eq50_e1683_q: f64 = (p.p37 * eq50_e1682_q);
        (eq50_e1683, eq50_e1683_d_n0, eq50_e1683_d_n1, eq50_e1683_d_n2, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_d_n13, eq50_e1683_d_b0, eq50_e1683_d_b1, eq50_e1683_d_b2, eq50_e1683_d_b3, eq50_e1683_d_b4, eq50_e1683_d_b5, eq50_e1683_d_b6, eq50_e1683_d_b7, eq50_e1683_d_b8, eq50_e1683_d_b9, eq50_e1683_d_b10, eq50_e1683_d_b11, eq50_e1683_d_b12, eq50_e1683_d_b13, eq50_e1683_d_b14, eq50_e1683_d_b15, eq50_e1683_d_b16, eq50_e1683_d_b17, eq50_e1683_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 14] = [eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13];
        let eq50_reactive_branch_derivatives: [f64; 18] = [eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1694, eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13, eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17, eq51_e1694_q,) = {
    if s.b[1553] {
        let eq51_e1690: f64 = (p.p33 * s.v[896]);
        let eq51_e1691_q: f64 = eq51_e1690;
        let eq51_e1692: f64 = (p.p37 * eq51_e1690);
        let eq51_e1692_d_n0: f64 = (p.p37 * (p.p33 * s.dn[896][0]));
        let eq51_e1692_d_n1: f64 = (p.p37 * (p.p33 * s.dn[896][1]));
        let eq51_e1692_d_n2: f64 = (p.p37 * (p.p33 * s.dn[896][2]));
        let eq51_e1692_d_n3: f64 = (p.p37 * (p.p33 * s.dn[896][3]));
        let eq51_e1692_d_n4: f64 = (p.p37 * (p.p33 * s.dn[896][4]));
        let eq51_e1692_d_n5: f64 = (p.p37 * (p.p33 * s.dn[896][5]));
        let eq51_e1692_d_n6: f64 = (p.p37 * (p.p33 * s.dn[896][6]));
        let eq51_e1692_d_n7: f64 = (p.p37 * (p.p33 * s.dn[896][7]));
        let eq51_e1692_d_n8: f64 = (p.p37 * (p.p33 * s.dn[896][8]));
        let eq51_e1692_d_n9: f64 = (p.p37 * (p.p33 * s.dn[896][9]));
        let eq51_e1692_d_n10: f64 = (p.p37 * (p.p33 * s.dn[896][10]));
        let eq51_e1692_d_n11: f64 = (p.p37 * (p.p33 * s.dn[896][11]));
        let eq51_e1692_d_n12: f64 = (p.p37 * (p.p33 * s.dn[896][12]));
        let eq51_e1692_d_n13: f64 = (p.p37 * (p.p33 * s.dn[896][13]));
        let eq51_e1692_d_b0: f64 = (p.p37 * (p.p33 * s.db[896][0]));
        let eq51_e1692_d_b1: f64 = (p.p37 * (p.p33 * s.db[896][1]));
        let eq51_e1692_d_b2: f64 = (p.p37 * (p.p33 * s.db[896][2]));
        let eq51_e1692_d_b3: f64 = (p.p37 * (p.p33 * s.db[896][3]));
        let eq51_e1692_d_b4: f64 = (p.p37 * (p.p33 * s.db[896][4]));
        let eq51_e1692_d_b5: f64 = (p.p37 * (p.p33 * s.db[896][5]));
        let eq51_e1692_d_b6: f64 = (p.p37 * (p.p33 * s.db[896][6]));
        let eq51_e1692_d_b7: f64 = (p.p37 * (p.p33 * s.db[896][7]));
        let eq51_e1692_d_b8: f64 = (p.p37 * (p.p33 * s.db[896][8]));
        let eq51_e1692_d_b9: f64 = (p.p37 * (p.p33 * s.db[896][9]));
        let eq51_e1692_d_b10: f64 = (p.p37 * (p.p33 * s.db[896][10]));
        let eq51_e1692_d_b11: f64 = (p.p37 * (p.p33 * s.db[896][11]));
        let eq51_e1692_d_b12: f64 = (p.p37 * (p.p33 * s.db[896][12]));
        let eq51_e1692_d_b13: f64 = (p.p37 * (p.p33 * s.db[896][13]));
        let eq51_e1692_d_b14: f64 = (p.p37 * (p.p33 * s.db[896][14]));
        let eq51_e1692_d_b15: f64 = (p.p37 * (p.p33 * s.db[896][15]));
        let eq51_e1692_d_b16: f64 = (p.p37 * (p.p33 * s.db[896][16]));
        let eq51_e1692_d_b17: f64 = (p.p37 * (p.p33 * s.db[896][17]));
        let eq51_e1692_q: f64 = (p.p37 * eq51_e1691_q);
        (eq51_e1692, eq51_e1692_d_n0, eq51_e1692_d_n1, eq51_e1692_d_n2, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_d_n13, eq51_e1692_d_b0, eq51_e1692_d_b1, eq51_e1692_d_b2, eq51_e1692_d_b3, eq51_e1692_d_b4, eq51_e1692_d_b5, eq51_e1692_d_b6, eq51_e1692_d_b7, eq51_e1692_d_b8, eq51_e1692_d_b9, eq51_e1692_d_b10, eq51_e1692_d_b11, eq51_e1692_d_b12, eq51_e1692_d_b13, eq51_e1692_d_b14, eq51_e1692_d_b15, eq51_e1692_d_b16, eq51_e1692_d_b17, eq51_e1692_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 14] = [eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13];
        let eq51_reactive_branch_derivatives: [f64; 18] = [eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[8]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13, eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17, eq52_e1703_q,) = {
    if s.b[1553] {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n10: f64 = ((p.p33 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1701_q: f64 = eq52_e1700;
        (eq52_e1700, (eq52_e1698 * s.dn[336][0]), (eq52_e1698 * s.dn[336][1]), (eq52_e1698 * s.dn[336][2]), eq52_e1700_d_n3, (eq52_e1698 * s.dn[336][4]), (eq52_e1698 * s.dn[336][5]), (eq52_e1698 * s.dn[336][6]), (eq52_e1698 * s.dn[336][7]), (eq52_e1698 * s.dn[336][8]), (eq52_e1698 * s.dn[336][9]), eq52_e1700_d_n10, (eq52_e1698 * s.dn[336][11]), (eq52_e1698 * s.dn[336][12]), (eq52_e1698 * s.dn[336][13]), (eq52_e1698 * s.db[336][0]), (eq52_e1698 * s.db[336][1]), (eq52_e1698 * s.db[336][2]), (eq52_e1698 * s.db[336][3]), (eq52_e1698 * s.db[336][4]), (eq52_e1698 * s.db[336][5]), (eq52_e1698 * s.db[336][6]), (eq52_e1698 * s.db[336][7]), (eq52_e1698 * s.db[336][8]), (eq52_e1698 * s.db[336][9]), (eq52_e1698 * s.db[336][10]), (eq52_e1698 * s.db[336][11]), (eq52_e1698 * s.db[336][12]), (eq52_e1698 * s.db[336][13]), (eq52_e1698 * s.db[336][14]), (eq52_e1698 * s.db[336][15]), (eq52_e1698 * s.db[336][16]), (eq52_e1698 * s.db[336][17]), eq52_e1701_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_reactive_node_derivatives: [f64; 14] = [eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13];
        let eq52_reactive_branch_derivatives: [f64; 18] = [eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[3]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13, eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17, eq53_e1713_q,) = {
    if (!s.b[1553]) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1710_q: f64 = eq53_e1709;
        let eq53_e1711: f64 = (p.p37 * eq53_e1709);
        let eq53_e1711_d_n0: f64 = (p.p37 * (p.p33 * s.dn[895][0]));
        let eq53_e1711_d_n1: f64 = (p.p37 * (p.p33 * s.dn[895][1]));
        let eq53_e1711_d_n2: f64 = (p.p37 * (p.p33 * s.dn[895][2]));
        let eq53_e1711_d_n3: f64 = (p.p37 * (p.p33 * s.dn[895][3]));
        let eq53_e1711_d_n4: f64 = (p.p37 * (p.p33 * s.dn[895][4]));
        let eq53_e1711_d_n5: f64 = (p.p37 * (p.p33 * s.dn[895][5]));
        let eq53_e1711_d_n6: f64 = (p.p37 * (p.p33 * s.dn[895][6]));
        let eq53_e1711_d_n7: f64 = (p.p37 * (p.p33 * s.dn[895][7]));
        let eq53_e1711_d_n8: f64 = (p.p37 * (p.p33 * s.dn[895][8]));
        let eq53_e1711_d_n9: f64 = (p.p37 * (p.p33 * s.dn[895][9]));
        let eq53_e1711_d_n10: f64 = (p.p37 * (p.p33 * s.dn[895][10]));
        let eq53_e1711_d_n11: f64 = (p.p37 * (p.p33 * s.dn[895][11]));
        let eq53_e1711_d_n12: f64 = (p.p37 * (p.p33 * s.dn[895][12]));
        let eq53_e1711_d_n13: f64 = (p.p37 * (p.p33 * s.dn[895][13]));
        let eq53_e1711_d_b0: f64 = (p.p37 * (p.p33 * s.db[895][0]));
        let eq53_e1711_d_b1: f64 = (p.p37 * (p.p33 * s.db[895][1]));
        let eq53_e1711_d_b2: f64 = (p.p37 * (p.p33 * s.db[895][2]));
        let eq53_e1711_d_b3: f64 = (p.p37 * (p.p33 * s.db[895][3]));
        let eq53_e1711_d_b4: f64 = (p.p37 * (p.p33 * s.db[895][4]));
        let eq53_e1711_d_b5: f64 = (p.p37 * (p.p33 * s.db[895][5]));
        let eq53_e1711_d_b6: f64 = (p.p37 * (p.p33 * s.db[895][6]));
        let eq53_e1711_d_b7: f64 = (p.p37 * (p.p33 * s.db[895][7]));
        let eq53_e1711_d_b8: f64 = (p.p37 * (p.p33 * s.db[895][8]));
        let eq53_e1711_d_b9: f64 = (p.p37 * (p.p33 * s.db[895][9]));
        let eq53_e1711_d_b10: f64 = (p.p37 * (p.p33 * s.db[895][10]));
        let eq53_e1711_d_b11: f64 = (p.p37 * (p.p33 * s.db[895][11]));
        let eq53_e1711_d_b12: f64 = (p.p37 * (p.p33 * s.db[895][12]));
        let eq53_e1711_d_b13: f64 = (p.p37 * (p.p33 * s.db[895][13]));
        let eq53_e1711_d_b14: f64 = (p.p37 * (p.p33 * s.db[895][14]));
        let eq53_e1711_d_b15: f64 = (p.p37 * (p.p33 * s.db[895][15]));
        let eq53_e1711_d_b16: f64 = (p.p37 * (p.p33 * s.db[895][16]));
        let eq53_e1711_d_b17: f64 = (p.p37 * (p.p33 * s.db[895][17]));
        let eq53_e1711_q: f64 = (p.p37 * eq53_e1710_q);
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13, eq53_e1711_d_b0, eq53_e1711_d_b1, eq53_e1711_d_b2, eq53_e1711_d_b3, eq53_e1711_d_b4, eq53_e1711_d_b5, eq53_e1711_d_b6, eq53_e1711_d_b7, eq53_e1711_d_b8, eq53_e1711_d_b9, eq53_e1711_d_b10, eq53_e1711_d_b11, eq53_e1711_d_b12, eq53_e1711_d_b13, eq53_e1711_d_b14, eq53_e1711_d_b15, eq53_e1711_d_b16, eq53_e1711_d_b17, eq53_e1711_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 14] = [eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13];
        let eq53_reactive_branch_derivatives: [f64; 18] = [eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13, eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17, eq54_e1723_q,) = {
    if (!s.b[1553]) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1720_q: f64 = eq54_e1719;
        let eq54_e1721: f64 = (p.p37 * eq54_e1719);
        let eq54_e1721_d_n0: f64 = (p.p37 * (p.p33 * s.dn[896][0]));
        let eq54_e1721_d_n1: f64 = (p.p37 * (p.p33 * s.dn[896][1]));
        let eq54_e1721_d_n2: f64 = (p.p37 * (p.p33 * s.dn[896][2]));
        let eq54_e1721_d_n3: f64 = (p.p37 * (p.p33 * s.dn[896][3]));
        let eq54_e1721_d_n4: f64 = (p.p37 * (p.p33 * s.dn[896][4]));
        let eq54_e1721_d_n5: f64 = (p.p37 * (p.p33 * s.dn[896][5]));
        let eq54_e1721_d_n6: f64 = (p.p37 * (p.p33 * s.dn[896][6]));
        let eq54_e1721_d_n7: f64 = (p.p37 * (p.p33 * s.dn[896][7]));
        let eq54_e1721_d_n8: f64 = (p.p37 * (p.p33 * s.dn[896][8]));
        let eq54_e1721_d_n9: f64 = (p.p37 * (p.p33 * s.dn[896][9]));
        let eq54_e1721_d_n10: f64 = (p.p37 * (p.p33 * s.dn[896][10]));
        let eq54_e1721_d_n11: f64 = (p.p37 * (p.p33 * s.dn[896][11]));
        let eq54_e1721_d_n12: f64 = (p.p37 * (p.p33 * s.dn[896][12]));
        let eq54_e1721_d_n13: f64 = (p.p37 * (p.p33 * s.dn[896][13]));
        let eq54_e1721_d_b0: f64 = (p.p37 * (p.p33 * s.db[896][0]));
        let eq54_e1721_d_b1: f64 = (p.p37 * (p.p33 * s.db[896][1]));
        let eq54_e1721_d_b2: f64 = (p.p37 * (p.p33 * s.db[896][2]));
        let eq54_e1721_d_b3: f64 = (p.p37 * (p.p33 * s.db[896][3]));
        let eq54_e1721_d_b4: f64 = (p.p37 * (p.p33 * s.db[896][4]));
        let eq54_e1721_d_b5: f64 = (p.p37 * (p.p33 * s.db[896][5]));
        let eq54_e1721_d_b6: f64 = (p.p37 * (p.p33 * s.db[896][6]));
        let eq54_e1721_d_b7: f64 = (p.p37 * (p.p33 * s.db[896][7]));
        let eq54_e1721_d_b8: f64 = (p.p37 * (p.p33 * s.db[896][8]));
        let eq54_e1721_d_b9: f64 = (p.p37 * (p.p33 * s.db[896][9]));
        let eq54_e1721_d_b10: f64 = (p.p37 * (p.p33 * s.db[896][10]));
        let eq54_e1721_d_b11: f64 = (p.p37 * (p.p33 * s.db[896][11]));
        let eq54_e1721_d_b12: f64 = (p.p37 * (p.p33 * s.db[896][12]));
        let eq54_e1721_d_b13: f64 = (p.p37 * (p.p33 * s.db[896][13]));
        let eq54_e1721_d_b14: f64 = (p.p37 * (p.p33 * s.db[896][14]));
        let eq54_e1721_d_b15: f64 = (p.p37 * (p.p33 * s.db[896][15]));
        let eq54_e1721_d_b16: f64 = (p.p37 * (p.p33 * s.db[896][16]));
        let eq54_e1721_d_b17: f64 = (p.p37 * (p.p33 * s.db[896][17]));
        let eq54_e1721_q: f64 = (p.p37 * eq54_e1720_q);
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13, eq54_e1721_d_b0, eq54_e1721_d_b1, eq54_e1721_d_b2, eq54_e1721_d_b3, eq54_e1721_d_b4, eq54_e1721_d_b5, eq54_e1721_d_b6, eq54_e1721_d_b7, eq54_e1721_d_b8, eq54_e1721_d_b9, eq54_e1721_d_b10, eq54_e1721_d_b11, eq54_e1721_d_b12, eq54_e1721_d_b13, eq54_e1721_d_b14, eq54_e1721_d_b15, eq54_e1721_d_b16, eq54_e1721_d_b17, eq54_e1721_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 14] = [eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13];
        let eq54_reactive_branch_derivatives: [f64; 18] = [eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13, eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17, eq55_e1733_q,) = {
    if (!s.b[1553]) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n9: f64 = ((p.p33 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1731_q: f64 = eq55_e1730;
        (eq55_e1730, (eq55_e1728 * s.dn[336][0]), (eq55_e1728 * s.dn[336][1]), (eq55_e1728 * s.dn[336][2]), eq55_e1730_d_n3, (eq55_e1728 * s.dn[336][4]), (eq55_e1728 * s.dn[336][5]), (eq55_e1728 * s.dn[336][6]), (eq55_e1728 * s.dn[336][7]), (eq55_e1728 * s.dn[336][8]), eq55_e1730_d_n9, (eq55_e1728 * s.dn[336][10]), (eq55_e1728 * s.dn[336][11]), (eq55_e1728 * s.dn[336][12]), (eq55_e1728 * s.dn[336][13]), (eq55_e1728 * s.db[336][0]), (eq55_e1728 * s.db[336][1]), (eq55_e1728 * s.db[336][2]), (eq55_e1728 * s.db[336][3]), (eq55_e1728 * s.db[336][4]), (eq55_e1728 * s.db[336][5]), (eq55_e1728 * s.db[336][6]), (eq55_e1728 * s.db[336][7]), (eq55_e1728 * s.db[336][8]), (eq55_e1728 * s.db[336][9]), (eq55_e1728 * s.db[336][10]), (eq55_e1728 * s.db[336][11]), (eq55_e1728 * s.db[336][12]), (eq55_e1728 * s.db[336][13]), (eq55_e1728 * s.db[336][14]), (eq55_e1728 * s.db[336][15]), (eq55_e1728 * s.db[336][16]), (eq55_e1728 * s.db[336][17]), eq55_e1731_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 14] = [eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13];
        let eq55_reactive_branch_derivatives: [f64; 18] = [eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[3]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1737_q: f64 = eq56_e1736;
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[3]),
            nodes,
            &s.dn[87],
            branches,
            &s.db[87],
            (multiplicity) * (p.p33),
        );
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1741_q: f64 = eq57_e1740;
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[3]),
            nodes,
            &s.dn[86],
            branches,
            &s.db[86],
            (multiplicity) * (p.p33),
        );
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13, eq71_e1869_d_b0, eq71_e1869_d_b1, eq71_e1869_d_b2, eq71_e1869_d_b3, eq71_e1869_d_b4, eq71_e1869_d_b5, eq71_e1869_d_b6, eq71_e1869_d_b7, eq71_e1869_d_b8, eq71_e1869_d_b9, eq71_e1869_d_b10, eq71_e1869_d_b11, eq71_e1869_d_b12, eq71_e1869_d_b13, eq71_e1869_d_b14, eq71_e1869_d_b15, eq71_e1869_d_b16, eq71_e1869_d_b17, eq71_e1869_q, eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13, eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17,) = {
    if ((s.b[1559] && s.b[1560]) && s.b[1561]) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1858_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq71_e1856 * s.db[822][0]));
        let eq71_e1858_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq71_e1856 * s.db[822][1]));
        let eq71_e1858_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq71_e1856 * s.db[822][2]));
        let eq71_e1858_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq71_e1856 * s.db[822][3]));
        let eq71_e1858_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq71_e1856 * s.db[822][4]));
        let eq71_e1858_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq71_e1856 * s.db[822][5]));
        let eq71_e1858_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq71_e1856 * s.db[822][6]));
        let eq71_e1858_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq71_e1856 * s.db[822][7]));
        let eq71_e1858_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq71_e1856 * s.db[822][8]));
        let eq71_e1858_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq71_e1856 * s.db[822][9]));
        let eq71_e1858_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq71_e1856 * s.db[822][10]));
        let eq71_e1858_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq71_e1856 * s.db[822][11]));
        let eq71_e1858_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq71_e1856 * s.db[822][12]));
        let eq71_e1858_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq71_e1856 * s.db[822][13]));
        let eq71_e1858_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq71_e1856 * s.db[822][14]));
        let eq71_e1858_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq71_e1856 * s.db[822][15]));
        let eq71_e1858_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq71_e1856 * s.db[822][16]));
        let eq71_e1858_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq71_e1856 * s.db[822][17]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1862_q: f64 = eq71_e1861;
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1861);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq71_e1863_d_b0: f64 = (eq71_e1858_d_b0 + (s.db[410][0] * s.v[158]));
        let eq71_e1863_d_b1: f64 = (eq71_e1858_d_b1 + (s.db[410][1] * s.v[158]));
        let eq71_e1863_d_b2: f64 = (eq71_e1858_d_b2 + (s.db[410][2] * s.v[158]));
        let eq71_e1863_d_b3: f64 = (eq71_e1858_d_b3 + (s.db[410][3] * s.v[158]));
        let eq71_e1863_d_b4: f64 = (eq71_e1858_d_b4 + (s.db[410][4] * s.v[158]));
        let eq71_e1863_d_b5: f64 = (eq71_e1858_d_b5 + (s.db[410][5] * s.v[158]));
        let eq71_e1863_d_b6: f64 = (eq71_e1858_d_b6 + (s.db[410][6] * s.v[158]));
        let eq71_e1863_d_b7: f64 = (eq71_e1858_d_b7 + (s.db[410][7] * s.v[158]));
        let eq71_e1863_d_b8: f64 = (eq71_e1858_d_b8 + (s.db[410][8] * s.v[158]));
        let eq71_e1863_d_b9: f64 = (eq71_e1858_d_b9 + (s.db[410][9] * s.v[158]));
        let eq71_e1863_d_b10: f64 = (eq71_e1858_d_b10 + (s.db[410][10] * s.v[158]));
        let eq71_e1863_d_b11: f64 = (eq71_e1858_d_b11 + (s.db[410][11] * s.v[158]));
        let eq71_e1863_d_b12: f64 = (eq71_e1858_d_b12 + (s.db[410][12] * s.v[158]));
        let eq71_e1863_d_b13: f64 = (eq71_e1858_d_b13 + (s.db[410][13] * s.v[158]));
        let eq71_e1863_d_b14: f64 = (eq71_e1858_d_b14 + (s.db[410][14] * s.v[158]));
        let eq71_e1863_d_b15: f64 = (eq71_e1858_d_b15 + (s.db[410][15] * s.v[158]));
        let eq71_e1863_d_b16: f64 = (eq71_e1858_d_b16 + (s.db[410][16] * s.v[158]));
        let eq71_e1863_d_b17: f64 = (eq71_e1858_d_b17 + (s.db[410][17] * s.v[158]));
        let eq71_e1863_q: f64 = eq71_e1862_q;
        let __rspice_inv_cse_0: f64 = 1.0 / s.v[157];
        let eq71_e1866: f64 = (s.v[410] * __rspice_inv_cse_0);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_0);
        let eq71_e1866_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_0);
        let eq71_e1866_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_0);
        let eq71_e1866_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_0);
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n0: f64 = (eq71_e1863_d_n0 + eq71_e1866_d_n0);
        let eq71_e1867_d_n1: f64 = (eq71_e1863_d_n1 + eq71_e1866_d_n1);
        let eq71_e1867_d_n2: f64 = (eq71_e1863_d_n2 + eq71_e1866_d_n2);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_d_n13: f64 = (eq71_e1863_d_n13 + eq71_e1866_d_n13);
        let eq71_e1867_d_b0: f64 = (eq71_e1863_d_b0 + eq71_e1866_d_b0);
        let eq71_e1867_d_b1: f64 = (eq71_e1863_d_b1 + eq71_e1866_d_b1);
        let eq71_e1867_d_b2: f64 = (eq71_e1863_d_b2 + eq71_e1866_d_b2);
        let eq71_e1867_d_b3: f64 = (eq71_e1863_d_b3 + eq71_e1866_d_b3);
        let eq71_e1867_d_b4: f64 = (eq71_e1863_d_b4 + eq71_e1866_d_b4);
        let eq71_e1867_d_b5: f64 = (eq71_e1863_d_b5 + eq71_e1866_d_b5);
        let eq71_e1867_d_b6: f64 = (eq71_e1863_d_b6 + eq71_e1866_d_b6);
        let eq71_e1867_d_b7: f64 = (eq71_e1863_d_b7 + eq71_e1866_d_b7);
        let eq71_e1867_d_b8: f64 = (eq71_e1863_d_b8 + eq71_e1866_d_b8);
        let eq71_e1867_d_b9: f64 = (eq71_e1863_d_b9 + eq71_e1866_d_b9);
        let eq71_e1867_d_b10: f64 = (eq71_e1863_d_b10 + eq71_e1866_d_b10);
        let eq71_e1867_d_b11: f64 = (eq71_e1863_d_b11 + eq71_e1866_d_b11);
        let eq71_e1867_d_b12: f64 = (eq71_e1863_d_b12 + eq71_e1866_d_b12);
        let eq71_e1867_d_b13: f64 = (eq71_e1863_d_b13 + eq71_e1866_d_b13);
        let eq71_e1867_d_b14: f64 = (eq71_e1863_d_b14 + eq71_e1866_d_b14);
        let eq71_e1867_d_b15: f64 = (eq71_e1863_d_b15 + eq71_e1866_d_b15);
        let eq71_e1867_d_b16: f64 = (eq71_e1863_d_b16 + eq71_e1866_d_b16);
        let eq71_e1867_d_b17: f64 = (eq71_e1863_d_b17 + eq71_e1866_d_b17);
        let eq71_e1867_q: f64 = eq71_e1863_q;
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13, eq71_e1867_d_b0, eq71_e1867_d_b1, eq71_e1867_d_b2, eq71_e1867_d_b3, eq71_e1867_d_b4, eq71_e1867_d_b5, eq71_e1867_d_b6, eq71_e1867_d_b7, eq71_e1867_d_b8, eq71_e1867_d_b9, eq71_e1867_d_b10, eq71_e1867_d_b11, eq71_e1867_d_b12, eq71_e1867_d_b13, eq71_e1867_d_b14, eq71_e1867_d_b15, eq71_e1867_d_b16, eq71_e1867_d_b17, eq71_e1867_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_reactive_node_derivatives: [f64; 14] = [eq71_e1869_q_d_n0, eq71_e1869_q_d_n1, eq71_e1869_q_d_n2, eq71_e1869_q_d_n3, eq71_e1869_q_d_n4, eq71_e1869_q_d_n5, eq71_e1869_q_d_n6, eq71_e1869_q_d_n7, eq71_e1869_q_d_n8, eq71_e1869_q_d_n9, eq71_e1869_q_d_n10, eq71_e1869_q_d_n11, eq71_e1869_q_d_n12, eq71_e1869_q_d_n13];
        let eq71_reactive_branch_derivatives: [f64; 18] = [eq71_e1869_q_d_b0, eq71_e1869_q_d_b1, eq71_e1869_q_d_b2, eq71_e1869_q_d_b3, eq71_e1869_q_d_b4, eq71_e1869_q_d_b5, eq71_e1869_q_d_b6, eq71_e1869_q_d_b7, eq71_e1869_q_d_b8, eq71_e1869_q_d_b9, eq71_e1869_q_d_b10, eq71_e1869_q_d_b11, eq71_e1869_q_d_b12, eq71_e1869_q_d_b13, eq71_e1869_q_d_b14, eq71_e1869_q_d_b15, eq71_e1869_q_d_b16, eq71_e1869_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq71_reactive_node_derivatives,
            branches,
            &eq71_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13, eq72_e1892_d_b0, eq72_e1892_d_b1, eq72_e1892_d_b2, eq72_e1892_d_b3, eq72_e1892_d_b4, eq72_e1892_d_b5, eq72_e1892_d_b6, eq72_e1892_d_b7, eq72_e1892_d_b8, eq72_e1892_d_b9, eq72_e1892_d_b10, eq72_e1892_d_b11, eq72_e1892_d_b12, eq72_e1892_d_b13, eq72_e1892_d_b14, eq72_e1892_d_b15, eq72_e1892_d_b16, eq72_e1892_d_b17, eq72_e1892_q, eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13, eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17,) = {
    if (((s.b[1559] && s.b[1560]) && (!s.b[1561])) && s.b[1562]) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1881_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq72_e1879 * s.db[822][0]));
        let eq72_e1881_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq72_e1879 * s.db[822][1]));
        let eq72_e1881_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq72_e1879 * s.db[822][2]));
        let eq72_e1881_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq72_e1879 * s.db[822][3]));
        let eq72_e1881_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq72_e1879 * s.db[822][4]));
        let eq72_e1881_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq72_e1879 * s.db[822][5]));
        let eq72_e1881_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq72_e1879 * s.db[822][6]));
        let eq72_e1881_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq72_e1879 * s.db[822][7]));
        let eq72_e1881_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq72_e1879 * s.db[822][8]));
        let eq72_e1881_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq72_e1879 * s.db[822][9]));
        let eq72_e1881_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq72_e1879 * s.db[822][10]));
        let eq72_e1881_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq72_e1879 * s.db[822][11]));
        let eq72_e1881_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq72_e1879 * s.db[822][12]));
        let eq72_e1881_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq72_e1879 * s.db[822][13]));
        let eq72_e1881_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq72_e1879 * s.db[822][14]));
        let eq72_e1881_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq72_e1879 * s.db[822][15]));
        let eq72_e1881_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq72_e1879 * s.db[822][16]));
        let eq72_e1881_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq72_e1879 * s.db[822][17]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1885_q: f64 = eq72_e1884;
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1884);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq72_e1886_d_b0: f64 = (eq72_e1881_d_b0 + (s.db[410][0] * s.v[158]));
        let eq72_e1886_d_b1: f64 = (eq72_e1881_d_b1 + (s.db[410][1] * s.v[158]));
        let eq72_e1886_d_b2: f64 = (eq72_e1881_d_b2 + (s.db[410][2] * s.v[158]));
        let eq72_e1886_d_b3: f64 = (eq72_e1881_d_b3 + (s.db[410][3] * s.v[158]));
        let eq72_e1886_d_b4: f64 = (eq72_e1881_d_b4 + (s.db[410][4] * s.v[158]));
        let eq72_e1886_d_b5: f64 = (eq72_e1881_d_b5 + (s.db[410][5] * s.v[158]));
        let eq72_e1886_d_b6: f64 = (eq72_e1881_d_b6 + (s.db[410][6] * s.v[158]));
        let eq72_e1886_d_b7: f64 = (eq72_e1881_d_b7 + (s.db[410][7] * s.v[158]));
        let eq72_e1886_d_b8: f64 = (eq72_e1881_d_b8 + (s.db[410][8] * s.v[158]));
        let eq72_e1886_d_b9: f64 = (eq72_e1881_d_b9 + (s.db[410][9] * s.v[158]));
        let eq72_e1886_d_b10: f64 = (eq72_e1881_d_b10 + (s.db[410][10] * s.v[158]));
        let eq72_e1886_d_b11: f64 = (eq72_e1881_d_b11 + (s.db[410][11] * s.v[158]));
        let eq72_e1886_d_b12: f64 = (eq72_e1881_d_b12 + (s.db[410][12] * s.v[158]));
        let eq72_e1886_d_b13: f64 = (eq72_e1881_d_b13 + (s.db[410][13] * s.v[158]));
        let eq72_e1886_d_b14: f64 = (eq72_e1881_d_b14 + (s.db[410][14] * s.v[158]));
        let eq72_e1886_d_b15: f64 = (eq72_e1881_d_b15 + (s.db[410][15] * s.v[158]));
        let eq72_e1886_d_b16: f64 = (eq72_e1881_d_b16 + (s.db[410][16] * s.v[158]));
        let eq72_e1886_d_b17: f64 = (eq72_e1881_d_b17 + (s.db[410][17] * s.v[158]));
        let eq72_e1886_q: f64 = eq72_e1885_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq72_e1889: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq72_e1889_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq72_e1889_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq72_e1889_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n0: f64 = (eq72_e1886_d_n0 + eq72_e1889_d_n0);
        let eq72_e1890_d_n1: f64 = (eq72_e1886_d_n1 + eq72_e1889_d_n1);
        let eq72_e1890_d_n2: f64 = (eq72_e1886_d_n2 + eq72_e1889_d_n2);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_d_n13: f64 = (eq72_e1886_d_n13 + eq72_e1889_d_n13);
        let eq72_e1890_d_b0: f64 = (eq72_e1886_d_b0 + eq72_e1889_d_b0);
        let eq72_e1890_d_b1: f64 = (eq72_e1886_d_b1 + eq72_e1889_d_b1);
        let eq72_e1890_d_b2: f64 = (eq72_e1886_d_b2 + eq72_e1889_d_b2);
        let eq72_e1890_d_b3: f64 = (eq72_e1886_d_b3 + eq72_e1889_d_b3);
        let eq72_e1890_d_b4: f64 = (eq72_e1886_d_b4 + eq72_e1889_d_b4);
        let eq72_e1890_d_b5: f64 = (eq72_e1886_d_b5 + eq72_e1889_d_b5);
        let eq72_e1890_d_b6: f64 = (eq72_e1886_d_b6 + eq72_e1889_d_b6);
        let eq72_e1890_d_b7: f64 = (eq72_e1886_d_b7 + eq72_e1889_d_b7);
        let eq72_e1890_d_b8: f64 = (eq72_e1886_d_b8 + eq72_e1889_d_b8);
        let eq72_e1890_d_b9: f64 = (eq72_e1886_d_b9 + eq72_e1889_d_b9);
        let eq72_e1890_d_b10: f64 = (eq72_e1886_d_b10 + eq72_e1889_d_b10);
        let eq72_e1890_d_b11: f64 = (eq72_e1886_d_b11 + eq72_e1889_d_b11);
        let eq72_e1890_d_b12: f64 = (eq72_e1886_d_b12 + eq72_e1889_d_b12);
        let eq72_e1890_d_b13: f64 = (eq72_e1886_d_b13 + eq72_e1889_d_b13);
        let eq72_e1890_d_b14: f64 = (eq72_e1886_d_b14 + eq72_e1889_d_b14);
        let eq72_e1890_d_b15: f64 = (eq72_e1886_d_b15 + eq72_e1889_d_b15);
        let eq72_e1890_d_b16: f64 = (eq72_e1886_d_b16 + eq72_e1889_d_b16);
        let eq72_e1890_d_b17: f64 = (eq72_e1886_d_b17 + eq72_e1889_d_b17);
        let eq72_e1890_q: f64 = eq72_e1886_q;
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13, eq72_e1890_d_b0, eq72_e1890_d_b1, eq72_e1890_d_b2, eq72_e1890_d_b3, eq72_e1890_d_b4, eq72_e1890_d_b5, eq72_e1890_d_b6, eq72_e1890_d_b7, eq72_e1890_d_b8, eq72_e1890_d_b9, eq72_e1890_d_b10, eq72_e1890_d_b11, eq72_e1890_d_b12, eq72_e1890_d_b13, eq72_e1890_d_b14, eq72_e1890_d_b15, eq72_e1890_d_b16, eq72_e1890_d_b17, eq72_e1890_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_reactive_node_derivatives: [f64; 14] = [eq72_e1892_q_d_n0, eq72_e1892_q_d_n1, eq72_e1892_q_d_n2, eq72_e1892_q_d_n3, eq72_e1892_q_d_n4, eq72_e1892_q_d_n5, eq72_e1892_q_d_n6, eq72_e1892_q_d_n7, eq72_e1892_q_d_n8, eq72_e1892_q_d_n9, eq72_e1892_q_d_n10, eq72_e1892_q_d_n11, eq72_e1892_q_d_n12, eq72_e1892_q_d_n13];
        let eq72_reactive_branch_derivatives: [f64; 18] = [eq72_e1892_q_d_b0, eq72_e1892_q_d_b1, eq72_e1892_q_d_b2, eq72_e1892_q_d_b3, eq72_e1892_q_d_b4, eq72_e1892_q_d_b5, eq72_e1892_q_d_b6, eq72_e1892_q_d_b7, eq72_e1892_q_d_b8, eq72_e1892_q_d_b9, eq72_e1892_q_d_b10, eq72_e1892_q_d_b11, eq72_e1892_q_d_b12, eq72_e1892_q_d_b13, eq72_e1892_q_d_b14, eq72_e1892_q_d_b15, eq72_e1892_q_d_b16, eq72_e1892_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq72_reactive_node_derivatives,
            branches,
            &eq72_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_2(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13, eq73_e1920_d_b0, eq73_e1920_d_b1, eq73_e1920_d_b2, eq73_e1920_d_b3, eq73_e1920_d_b4, eq73_e1920_d_b5, eq73_e1920_d_b6, eq73_e1920_d_b7, eq73_e1920_d_b8, eq73_e1920_d_b9, eq73_e1920_d_b10, eq73_e1920_d_b11, eq73_e1920_d_b12, eq73_e1920_d_b13, eq73_e1920_d_b14, eq73_e1920_d_b15, eq73_e1920_d_b16, eq73_e1920_d_b17, eq73_e1920_q, eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13, eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq73_e1906_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq73_e1906_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq73_e1906_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = (((-eq73_e1906_d_n0) * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = (((-eq73_e1906_d_n1) * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = (((-eq73_e1906_d_n2) * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = (((-eq73_e1906_d_n3) * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = (((-eq73_e1906_d_n4) * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = (((-eq73_e1906_d_n5) * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = (((-eq73_e1906_d_n6) * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = (((-eq73_e1906_d_n9) * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = (((-eq73_e1906_d_n10) * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = (((-eq73_e1906_d_n11) * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = (((-eq73_e1906_d_n12) * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = (((-eq73_e1906_d_n13) * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1909_d_b0: f64 = (((-eq73_e1906_d_b0) * s.v[822]) + (eq73_e1907 * s.db[822][0]));
        let eq73_e1909_d_b1: f64 = (((-eq73_e1906_d_b1) * s.v[822]) + (eq73_e1907 * s.db[822][1]));
        let eq73_e1909_d_b2: f64 = (((-eq73_e1906_d_b2) * s.v[822]) + (eq73_e1907 * s.db[822][2]));
        let eq73_e1909_d_b3: f64 = (((-eq73_e1906_d_b3) * s.v[822]) + (eq73_e1907 * s.db[822][3]));
        let eq73_e1909_d_b4: f64 = (((-eq73_e1906_d_b4) * s.v[822]) + (eq73_e1907 * s.db[822][4]));
        let eq73_e1909_d_b5: f64 = (((-eq73_e1906_d_b5) * s.v[822]) + (eq73_e1907 * s.db[822][5]));
        let eq73_e1909_d_b6: f64 = (((-eq73_e1906_d_b6) * s.v[822]) + (eq73_e1907 * s.db[822][6]));
        let eq73_e1909_d_b7: f64 = (((-eq73_e1906_d_b7) * s.v[822]) + (eq73_e1907 * s.db[822][7]));
        let eq73_e1909_d_b8: f64 = (((-eq73_e1906_d_b8) * s.v[822]) + (eq73_e1907 * s.db[822][8]));
        let eq73_e1909_d_b9: f64 = (((-eq73_e1906_d_b9) * s.v[822]) + (eq73_e1907 * s.db[822][9]));
        let eq73_e1909_d_b10: f64 = (((-eq73_e1906_d_b10) * s.v[822]) + (eq73_e1907 * s.db[822][10]));
        let eq73_e1909_d_b11: f64 = (((-eq73_e1906_d_b11) * s.v[822]) + (eq73_e1907 * s.db[822][11]));
        let eq73_e1909_d_b12: f64 = (((-eq73_e1906_d_b12) * s.v[822]) + (eq73_e1907 * s.db[822][12]));
        let eq73_e1909_d_b13: f64 = (((-eq73_e1906_d_b13) * s.v[822]) + (eq73_e1907 * s.db[822][13]));
        let eq73_e1909_d_b14: f64 = (((-eq73_e1906_d_b14) * s.v[822]) + (eq73_e1907 * s.db[822][14]));
        let eq73_e1909_d_b15: f64 = (((-eq73_e1906_d_b15) * s.v[822]) + (eq73_e1907 * s.db[822][15]));
        let eq73_e1909_d_b16: f64 = (((-eq73_e1906_d_b16) * s.v[822]) + (eq73_e1907 * s.db[822][16]));
        let eq73_e1909_d_b17: f64 = (((-eq73_e1906_d_b17) * s.v[822]) + (eq73_e1907 * s.db[822][17]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1913_q: f64 = eq73_e1912;
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1912);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq73_e1914_d_b0: f64 = (eq73_e1909_d_b0 + (s.db[410][0] * s.v[158]));
        let eq73_e1914_d_b1: f64 = (eq73_e1909_d_b1 + (s.db[410][1] * s.v[158]));
        let eq73_e1914_d_b2: f64 = (eq73_e1909_d_b2 + (s.db[410][2] * s.v[158]));
        let eq73_e1914_d_b3: f64 = (eq73_e1909_d_b3 + (s.db[410][3] * s.v[158]));
        let eq73_e1914_d_b4: f64 = (eq73_e1909_d_b4 + (s.db[410][4] * s.v[158]));
        let eq73_e1914_d_b5: f64 = (eq73_e1909_d_b5 + (s.db[410][5] * s.v[158]));
        let eq73_e1914_d_b6: f64 = (eq73_e1909_d_b6 + (s.db[410][6] * s.v[158]));
        let eq73_e1914_d_b7: f64 = (eq73_e1909_d_b7 + (s.db[410][7] * s.v[158]));
        let eq73_e1914_d_b8: f64 = (eq73_e1909_d_b8 + (s.db[410][8] * s.v[158]));
        let eq73_e1914_d_b9: f64 = (eq73_e1909_d_b9 + (s.db[410][9] * s.v[158]));
        let eq73_e1914_d_b10: f64 = (eq73_e1909_d_b10 + (s.db[410][10] * s.v[158]));
        let eq73_e1914_d_b11: f64 = (eq73_e1909_d_b11 + (s.db[410][11] * s.v[158]));
        let eq73_e1914_d_b12: f64 = (eq73_e1909_d_b12 + (s.db[410][12] * s.v[158]));
        let eq73_e1914_d_b13: f64 = (eq73_e1909_d_b13 + (s.db[410][13] * s.v[158]));
        let eq73_e1914_d_b14: f64 = (eq73_e1909_d_b14 + (s.db[410][14] * s.v[158]));
        let eq73_e1914_d_b15: f64 = (eq73_e1909_d_b15 + (s.db[410][15] * s.v[158]));
        let eq73_e1914_d_b16: f64 = (eq73_e1909_d_b16 + (s.db[410][16] * s.v[158]));
        let eq73_e1914_d_b17: f64 = (eq73_e1909_d_b17 + (s.db[410][17] * s.v[158]));
        let eq73_e1914_q: f64 = eq73_e1913_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq73_e1917: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq73_e1917_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq73_e1917_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq73_e1917_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n0: f64 = (eq73_e1914_d_n0 + eq73_e1917_d_n0);
        let eq73_e1918_d_n1: f64 = (eq73_e1914_d_n1 + eq73_e1917_d_n1);
        let eq73_e1918_d_n2: f64 = (eq73_e1914_d_n2 + eq73_e1917_d_n2);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_d_n13: f64 = (eq73_e1914_d_n13 + eq73_e1917_d_n13);
        let eq73_e1918_d_b0: f64 = (eq73_e1914_d_b0 + eq73_e1917_d_b0);
        let eq73_e1918_d_b1: f64 = (eq73_e1914_d_b1 + eq73_e1917_d_b1);
        let eq73_e1918_d_b2: f64 = (eq73_e1914_d_b2 + eq73_e1917_d_b2);
        let eq73_e1918_d_b3: f64 = (eq73_e1914_d_b3 + eq73_e1917_d_b3);
        let eq73_e1918_d_b4: f64 = (eq73_e1914_d_b4 + eq73_e1917_d_b4);
        let eq73_e1918_d_b5: f64 = (eq73_e1914_d_b5 + eq73_e1917_d_b5);
        let eq73_e1918_d_b6: f64 = (eq73_e1914_d_b6 + eq73_e1917_d_b6);
        let eq73_e1918_d_b7: f64 = (eq73_e1914_d_b7 + eq73_e1917_d_b7);
        let eq73_e1918_d_b8: f64 = (eq73_e1914_d_b8 + eq73_e1917_d_b8);
        let eq73_e1918_d_b9: f64 = (eq73_e1914_d_b9 + eq73_e1917_d_b9);
        let eq73_e1918_d_b10: f64 = (eq73_e1914_d_b10 + eq73_e1917_d_b10);
        let eq73_e1918_d_b11: f64 = (eq73_e1914_d_b11 + eq73_e1917_d_b11);
        let eq73_e1918_d_b12: f64 = (eq73_e1914_d_b12 + eq73_e1917_d_b12);
        let eq73_e1918_d_b13: f64 = (eq73_e1914_d_b13 + eq73_e1917_d_b13);
        let eq73_e1918_d_b14: f64 = (eq73_e1914_d_b14 + eq73_e1917_d_b14);
        let eq73_e1918_d_b15: f64 = (eq73_e1914_d_b15 + eq73_e1917_d_b15);
        let eq73_e1918_d_b16: f64 = (eq73_e1914_d_b16 + eq73_e1917_d_b16);
        let eq73_e1918_d_b17: f64 = (eq73_e1914_d_b17 + eq73_e1917_d_b17);
        let eq73_e1918_q: f64 = eq73_e1914_q;
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13, eq73_e1918_d_b0, eq73_e1918_d_b1, eq73_e1918_d_b2, eq73_e1918_d_b3, eq73_e1918_d_b4, eq73_e1918_d_b5, eq73_e1918_d_b6, eq73_e1918_d_b7, eq73_e1918_d_b8, eq73_e1918_d_b9, eq73_e1918_d_b10, eq73_e1918_d_b11, eq73_e1918_d_b12, eq73_e1918_d_b13, eq73_e1918_d_b14, eq73_e1918_d_b15, eq73_e1918_d_b16, eq73_e1918_d_b17, eq73_e1918_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_reactive_node_derivatives: [f64; 14] = [eq73_e1920_q_d_n0, eq73_e1920_q_d_n1, eq73_e1920_q_d_n2, eq73_e1920_q_d_n3, eq73_e1920_q_d_n4, eq73_e1920_q_d_n5, eq73_e1920_q_d_n6, eq73_e1920_q_d_n7, eq73_e1920_q_d_n8, eq73_e1920_q_d_n9, eq73_e1920_q_d_n10, eq73_e1920_q_d_n11, eq73_e1920_q_d_n12, eq73_e1920_q_d_n13];
        let eq73_reactive_branch_derivatives: [f64; 18] = [eq73_e1920_q_d_b0, eq73_e1920_q_d_b1, eq73_e1920_q_d_b2, eq73_e1920_q_d_b3, eq73_e1920_q_d_b4, eq73_e1920_q_d_b5, eq73_e1920_q_d_b6, eq73_e1920_q_d_b7, eq73_e1920_q_d_b8, eq73_e1920_q_d_b9, eq73_e1920_q_d_b10, eq73_e1920_q_d_b11, eq73_e1920_q_d_b12, eq73_e1920_q_d_b13, eq73_e1920_q_d_b14, eq73_e1920_q_d_b15, eq73_e1920_q_d_b16, eq73_e1920_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq73_reactive_node_derivatives,
            branches,
            &eq73_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13, eq74_e1947_d_b0, eq74_e1947_d_b1, eq74_e1947_d_b2, eq74_e1947_d_b3, eq74_e1947_d_b4, eq74_e1947_d_b5, eq74_e1947_d_b6, eq74_e1947_d_b7, eq74_e1947_d_b8, eq74_e1947_d_b9, eq74_e1947_d_b10, eq74_e1947_d_b11, eq74_e1947_d_b12, eq74_e1947_d_b13, eq74_e1947_d_b14, eq74_e1947_d_b15, eq74_e1947_d_b16, eq74_e1947_d_b17, eq74_e1947_q, eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13, eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && (!s.b[1563])) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1936_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq74_e1934 * s.db[822][0]));
        let eq74_e1936_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq74_e1934 * s.db[822][1]));
        let eq74_e1936_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq74_e1934 * s.db[822][2]));
        let eq74_e1936_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq74_e1934 * s.db[822][3]));
        let eq74_e1936_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq74_e1934 * s.db[822][4]));
        let eq74_e1936_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq74_e1934 * s.db[822][5]));
        let eq74_e1936_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq74_e1934 * s.db[822][6]));
        let eq74_e1936_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq74_e1934 * s.db[822][7]));
        let eq74_e1936_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq74_e1934 * s.db[822][8]));
        let eq74_e1936_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq74_e1934 * s.db[822][9]));
        let eq74_e1936_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq74_e1934 * s.db[822][10]));
        let eq74_e1936_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq74_e1934 * s.db[822][11]));
        let eq74_e1936_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq74_e1934 * s.db[822][12]));
        let eq74_e1936_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq74_e1934 * s.db[822][13]));
        let eq74_e1936_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq74_e1934 * s.db[822][14]));
        let eq74_e1936_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq74_e1934 * s.db[822][15]));
        let eq74_e1936_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq74_e1934 * s.db[822][16]));
        let eq74_e1936_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq74_e1934 * s.db[822][17]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1940_q: f64 = eq74_e1939;
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1939);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq74_e1941_d_b0: f64 = (eq74_e1936_d_b0 + (s.db[410][0] * s.v[158]));
        let eq74_e1941_d_b1: f64 = (eq74_e1936_d_b1 + (s.db[410][1] * s.v[158]));
        let eq74_e1941_d_b2: f64 = (eq74_e1936_d_b2 + (s.db[410][2] * s.v[158]));
        let eq74_e1941_d_b3: f64 = (eq74_e1936_d_b3 + (s.db[410][3] * s.v[158]));
        let eq74_e1941_d_b4: f64 = (eq74_e1936_d_b4 + (s.db[410][4] * s.v[158]));
        let eq74_e1941_d_b5: f64 = (eq74_e1936_d_b5 + (s.db[410][5] * s.v[158]));
        let eq74_e1941_d_b6: f64 = (eq74_e1936_d_b6 + (s.db[410][6] * s.v[158]));
        let eq74_e1941_d_b7: f64 = (eq74_e1936_d_b7 + (s.db[410][7] * s.v[158]));
        let eq74_e1941_d_b8: f64 = (eq74_e1936_d_b8 + (s.db[410][8] * s.v[158]));
        let eq74_e1941_d_b9: f64 = (eq74_e1936_d_b9 + (s.db[410][9] * s.v[158]));
        let eq74_e1941_d_b10: f64 = (eq74_e1936_d_b10 + (s.db[410][10] * s.v[158]));
        let eq74_e1941_d_b11: f64 = (eq74_e1936_d_b11 + (s.db[410][11] * s.v[158]));
        let eq74_e1941_d_b12: f64 = (eq74_e1936_d_b12 + (s.db[410][12] * s.v[158]));
        let eq74_e1941_d_b13: f64 = (eq74_e1936_d_b13 + (s.db[410][13] * s.v[158]));
        let eq74_e1941_d_b14: f64 = (eq74_e1936_d_b14 + (s.db[410][14] * s.v[158]));
        let eq74_e1941_d_b15: f64 = (eq74_e1936_d_b15 + (s.db[410][15] * s.v[158]));
        let eq74_e1941_d_b16: f64 = (eq74_e1936_d_b16 + (s.db[410][16] * s.v[158]));
        let eq74_e1941_d_b17: f64 = (eq74_e1936_d_b17 + (s.db[410][17] * s.v[158]));
        let eq74_e1941_q: f64 = eq74_e1940_q;
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq74_e1944: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq74_e1944_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq74_e1944_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq74_e1944_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n0: f64 = (eq74_e1941_d_n0 + eq74_e1944_d_n0);
        let eq74_e1945_d_n1: f64 = (eq74_e1941_d_n1 + eq74_e1944_d_n1);
        let eq74_e1945_d_n2: f64 = (eq74_e1941_d_n2 + eq74_e1944_d_n2);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_d_n13: f64 = (eq74_e1941_d_n13 + eq74_e1944_d_n13);
        let eq74_e1945_d_b0: f64 = (eq74_e1941_d_b0 + eq74_e1944_d_b0);
        let eq74_e1945_d_b1: f64 = (eq74_e1941_d_b1 + eq74_e1944_d_b1);
        let eq74_e1945_d_b2: f64 = (eq74_e1941_d_b2 + eq74_e1944_d_b2);
        let eq74_e1945_d_b3: f64 = (eq74_e1941_d_b3 + eq74_e1944_d_b3);
        let eq74_e1945_d_b4: f64 = (eq74_e1941_d_b4 + eq74_e1944_d_b4);
        let eq74_e1945_d_b5: f64 = (eq74_e1941_d_b5 + eq74_e1944_d_b5);
        let eq74_e1945_d_b6: f64 = (eq74_e1941_d_b6 + eq74_e1944_d_b6);
        let eq74_e1945_d_b7: f64 = (eq74_e1941_d_b7 + eq74_e1944_d_b7);
        let eq74_e1945_d_b8: f64 = (eq74_e1941_d_b8 + eq74_e1944_d_b8);
        let eq74_e1945_d_b9: f64 = (eq74_e1941_d_b9 + eq74_e1944_d_b9);
        let eq74_e1945_d_b10: f64 = (eq74_e1941_d_b10 + eq74_e1944_d_b10);
        let eq74_e1945_d_b11: f64 = (eq74_e1941_d_b11 + eq74_e1944_d_b11);
        let eq74_e1945_d_b12: f64 = (eq74_e1941_d_b12 + eq74_e1944_d_b12);
        let eq74_e1945_d_b13: f64 = (eq74_e1941_d_b13 + eq74_e1944_d_b13);
        let eq74_e1945_d_b14: f64 = (eq74_e1941_d_b14 + eq74_e1944_d_b14);
        let eq74_e1945_d_b15: f64 = (eq74_e1941_d_b15 + eq74_e1944_d_b15);
        let eq74_e1945_d_b16: f64 = (eq74_e1941_d_b16 + eq74_e1944_d_b16);
        let eq74_e1945_d_b17: f64 = (eq74_e1941_d_b17 + eq74_e1944_d_b17);
        let eq74_e1945_q: f64 = eq74_e1941_q;
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13, eq74_e1945_d_b0, eq74_e1945_d_b1, eq74_e1945_d_b2, eq74_e1945_d_b3, eq74_e1945_d_b4, eq74_e1945_d_b5, eq74_e1945_d_b6, eq74_e1945_d_b7, eq74_e1945_d_b8, eq74_e1945_d_b9, eq74_e1945_d_b10, eq74_e1945_d_b11, eq74_e1945_d_b12, eq74_e1945_d_b13, eq74_e1945_d_b14, eq74_e1945_d_b15, eq74_e1945_d_b16, eq74_e1945_d_b17, eq74_e1945_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_reactive_node_derivatives: [f64; 14] = [eq74_e1947_q_d_n0, eq74_e1947_q_d_n1, eq74_e1947_q_d_n2, eq74_e1947_q_d_n3, eq74_e1947_q_d_n4, eq74_e1947_q_d_n5, eq74_e1947_q_d_n6, eq74_e1947_q_d_n7, eq74_e1947_q_d_n8, eq74_e1947_q_d_n9, eq74_e1947_q_d_n10, eq74_e1947_q_d_n11, eq74_e1947_q_d_n12, eq74_e1947_q_d_n13];
        let eq74_reactive_branch_derivatives: [f64; 18] = [eq74_e1947_q_d_b0, eq74_e1947_q_d_b1, eq74_e1947_q_d_b2, eq74_e1947_q_d_b3, eq74_e1947_q_d_b4, eq74_e1947_q_d_b5, eq74_e1947_q_d_b6, eq74_e1947_q_d_b7, eq74_e1947_q_d_b8, eq74_e1947_q_d_b9, eq74_e1947_q_d_b10, eq74_e1947_q_d_b11, eq74_e1947_q_d_b12, eq74_e1947_q_d_b13, eq74_e1947_q_d_b14, eq74_e1947_q_d_b15, eq74_e1947_q_d_b16, eq74_e1947_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq74_reactive_node_derivatives,
            branches,
            &eq74_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_3(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13, eq75_e1970_d_b0, eq75_e1970_d_b1, eq75_e1970_d_b2, eq75_e1970_d_b3, eq75_e1970_d_b4, eq75_e1970_d_b5, eq75_e1970_d_b6, eq75_e1970_d_b7, eq75_e1970_d_b8, eq75_e1970_d_b9, eq75_e1970_d_b10, eq75_e1970_d_b11, eq75_e1970_d_b12, eq75_e1970_d_b13, eq75_e1970_d_b14, eq75_e1970_d_b15, eq75_e1970_d_b16, eq75_e1970_d_b17, eq75_e1970_q, eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13, eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && s.b[1564]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq75_e1956_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq75_e1956_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq75_e1956_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = (((-eq75_e1956_d_n0) * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = (((-eq75_e1956_d_n1) * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = (((-eq75_e1956_d_n2) * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = (((-eq75_e1956_d_n3) * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = (((-eq75_e1956_d_n4) * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = (((-eq75_e1956_d_n5) * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = (((-eq75_e1956_d_n6) * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = (((-eq75_e1956_d_n9) * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = (((-eq75_e1956_d_n10) * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = (((-eq75_e1956_d_n11) * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = (((-eq75_e1956_d_n12) * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = (((-eq75_e1956_d_n13) * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1959_d_b0: f64 = (((-eq75_e1956_d_b0) * s.v[822]) + (eq75_e1957 * s.db[822][0]));
        let eq75_e1959_d_b1: f64 = (((-eq75_e1956_d_b1) * s.v[822]) + (eq75_e1957 * s.db[822][1]));
        let eq75_e1959_d_b2: f64 = (((-eq75_e1956_d_b2) * s.v[822]) + (eq75_e1957 * s.db[822][2]));
        let eq75_e1959_d_b3: f64 = (((-eq75_e1956_d_b3) * s.v[822]) + (eq75_e1957 * s.db[822][3]));
        let eq75_e1959_d_b4: f64 = (((-eq75_e1956_d_b4) * s.v[822]) + (eq75_e1957 * s.db[822][4]));
        let eq75_e1959_d_b5: f64 = (((-eq75_e1956_d_b5) * s.v[822]) + (eq75_e1957 * s.db[822][5]));
        let eq75_e1959_d_b6: f64 = (((-eq75_e1956_d_b6) * s.v[822]) + (eq75_e1957 * s.db[822][6]));
        let eq75_e1959_d_b7: f64 = (((-eq75_e1956_d_b7) * s.v[822]) + (eq75_e1957 * s.db[822][7]));
        let eq75_e1959_d_b8: f64 = (((-eq75_e1956_d_b8) * s.v[822]) + (eq75_e1957 * s.db[822][8]));
        let eq75_e1959_d_b9: f64 = (((-eq75_e1956_d_b9) * s.v[822]) + (eq75_e1957 * s.db[822][9]));
        let eq75_e1959_d_b10: f64 = (((-eq75_e1956_d_b10) * s.v[822]) + (eq75_e1957 * s.db[822][10]));
        let eq75_e1959_d_b11: f64 = (((-eq75_e1956_d_b11) * s.v[822]) + (eq75_e1957 * s.db[822][11]));
        let eq75_e1959_d_b12: f64 = (((-eq75_e1956_d_b12) * s.v[822]) + (eq75_e1957 * s.db[822][12]));
        let eq75_e1959_d_b13: f64 = (((-eq75_e1956_d_b13) * s.v[822]) + (eq75_e1957 * s.db[822][13]));
        let eq75_e1959_d_b14: f64 = (((-eq75_e1956_d_b14) * s.v[822]) + (eq75_e1957 * s.db[822][14]));
        let eq75_e1959_d_b15: f64 = (((-eq75_e1956_d_b15) * s.v[822]) + (eq75_e1957 * s.db[822][15]));
        let eq75_e1959_d_b16: f64 = (((-eq75_e1956_d_b16) * s.v[822]) + (eq75_e1957 * s.db[822][16]));
        let eq75_e1959_d_b17: f64 = (((-eq75_e1956_d_b17) * s.v[822]) + (eq75_e1957 * s.db[822][17]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1963_q: f64 = eq75_e1962;
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1962);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq75_e1964_d_b0: f64 = (eq75_e1959_d_b0 + (s.db[410][0] * s.v[158]));
        let eq75_e1964_d_b1: f64 = (eq75_e1959_d_b1 + (s.db[410][1] * s.v[158]));
        let eq75_e1964_d_b2: f64 = (eq75_e1959_d_b2 + (s.db[410][2] * s.v[158]));
        let eq75_e1964_d_b3: f64 = (eq75_e1959_d_b3 + (s.db[410][3] * s.v[158]));
        let eq75_e1964_d_b4: f64 = (eq75_e1959_d_b4 + (s.db[410][4] * s.v[158]));
        let eq75_e1964_d_b5: f64 = (eq75_e1959_d_b5 + (s.db[410][5] * s.v[158]));
        let eq75_e1964_d_b6: f64 = (eq75_e1959_d_b6 + (s.db[410][6] * s.v[158]));
        let eq75_e1964_d_b7: f64 = (eq75_e1959_d_b7 + (s.db[410][7] * s.v[158]));
        let eq75_e1964_d_b8: f64 = (eq75_e1959_d_b8 + (s.db[410][8] * s.v[158]));
        let eq75_e1964_d_b9: f64 = (eq75_e1959_d_b9 + (s.db[410][9] * s.v[158]));
        let eq75_e1964_d_b10: f64 = (eq75_e1959_d_b10 + (s.db[410][10] * s.v[158]));
        let eq75_e1964_d_b11: f64 = (eq75_e1959_d_b11 + (s.db[410][11] * s.v[158]));
        let eq75_e1964_d_b12: f64 = (eq75_e1959_d_b12 + (s.db[410][12] * s.v[158]));
        let eq75_e1964_d_b13: f64 = (eq75_e1959_d_b13 + (s.db[410][13] * s.v[158]));
        let eq75_e1964_d_b14: f64 = (eq75_e1959_d_b14 + (s.db[410][14] * s.v[158]));
        let eq75_e1964_d_b15: f64 = (eq75_e1959_d_b15 + (s.db[410][15] * s.v[158]));
        let eq75_e1964_d_b16: f64 = (eq75_e1959_d_b16 + (s.db[410][16] * s.v[158]));
        let eq75_e1964_d_b17: f64 = (eq75_e1959_d_b17 + (s.db[410][17] * s.v[158]));
        let eq75_e1964_q: f64 = eq75_e1963_q;
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq75_e1967: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq75_e1967_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq75_e1967_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq75_e1967_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n0: f64 = (eq75_e1964_d_n0 + eq75_e1967_d_n0);
        let eq75_e1968_d_n1: f64 = (eq75_e1964_d_n1 + eq75_e1967_d_n1);
        let eq75_e1968_d_n2: f64 = (eq75_e1964_d_n2 + eq75_e1967_d_n2);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_d_n13: f64 = (eq75_e1964_d_n13 + eq75_e1967_d_n13);
        let eq75_e1968_d_b0: f64 = (eq75_e1964_d_b0 + eq75_e1967_d_b0);
        let eq75_e1968_d_b1: f64 = (eq75_e1964_d_b1 + eq75_e1967_d_b1);
        let eq75_e1968_d_b2: f64 = (eq75_e1964_d_b2 + eq75_e1967_d_b2);
        let eq75_e1968_d_b3: f64 = (eq75_e1964_d_b3 + eq75_e1967_d_b3);
        let eq75_e1968_d_b4: f64 = (eq75_e1964_d_b4 + eq75_e1967_d_b4);
        let eq75_e1968_d_b5: f64 = (eq75_e1964_d_b5 + eq75_e1967_d_b5);
        let eq75_e1968_d_b6: f64 = (eq75_e1964_d_b6 + eq75_e1967_d_b6);
        let eq75_e1968_d_b7: f64 = (eq75_e1964_d_b7 + eq75_e1967_d_b7);
        let eq75_e1968_d_b8: f64 = (eq75_e1964_d_b8 + eq75_e1967_d_b8);
        let eq75_e1968_d_b9: f64 = (eq75_e1964_d_b9 + eq75_e1967_d_b9);
        let eq75_e1968_d_b10: f64 = (eq75_e1964_d_b10 + eq75_e1967_d_b10);
        let eq75_e1968_d_b11: f64 = (eq75_e1964_d_b11 + eq75_e1967_d_b11);
        let eq75_e1968_d_b12: f64 = (eq75_e1964_d_b12 + eq75_e1967_d_b12);
        let eq75_e1968_d_b13: f64 = (eq75_e1964_d_b13 + eq75_e1967_d_b13);
        let eq75_e1968_d_b14: f64 = (eq75_e1964_d_b14 + eq75_e1967_d_b14);
        let eq75_e1968_d_b15: f64 = (eq75_e1964_d_b15 + eq75_e1967_d_b15);
        let eq75_e1968_d_b16: f64 = (eq75_e1964_d_b16 + eq75_e1967_d_b16);
        let eq75_e1968_d_b17: f64 = (eq75_e1964_d_b17 + eq75_e1967_d_b17);
        let eq75_e1968_q: f64 = eq75_e1964_q;
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13, eq75_e1968_d_b0, eq75_e1968_d_b1, eq75_e1968_d_b2, eq75_e1968_d_b3, eq75_e1968_d_b4, eq75_e1968_d_b5, eq75_e1968_d_b6, eq75_e1968_d_b7, eq75_e1968_d_b8, eq75_e1968_d_b9, eq75_e1968_d_b10, eq75_e1968_d_b11, eq75_e1968_d_b12, eq75_e1968_d_b13, eq75_e1968_d_b14, eq75_e1968_d_b15, eq75_e1968_d_b16, eq75_e1968_d_b17, eq75_e1968_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_reactive_node_derivatives: [f64; 14] = [eq75_e1970_q_d_n0, eq75_e1970_q_d_n1, eq75_e1970_q_d_n2, eq75_e1970_q_d_n3, eq75_e1970_q_d_n4, eq75_e1970_q_d_n5, eq75_e1970_q_d_n6, eq75_e1970_q_d_n7, eq75_e1970_q_d_n8, eq75_e1970_q_d_n9, eq75_e1970_q_d_n10, eq75_e1970_q_d_n11, eq75_e1970_q_d_n12, eq75_e1970_q_d_n13];
        let eq75_reactive_branch_derivatives: [f64; 18] = [eq75_e1970_q_d_b0, eq75_e1970_q_d_b1, eq75_e1970_q_d_b2, eq75_e1970_q_d_b3, eq75_e1970_q_d_b4, eq75_e1970_q_d_b5, eq75_e1970_q_d_b6, eq75_e1970_q_d_b7, eq75_e1970_q_d_b8, eq75_e1970_q_d_b9, eq75_e1970_q_d_b10, eq75_e1970_q_d_b11, eq75_e1970_q_d_b12, eq75_e1970_q_d_b13, eq75_e1970_q_d_b14, eq75_e1970_q_d_b15, eq75_e1970_q_d_b16, eq75_e1970_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq75_reactive_node_derivatives,
            branches,
            &eq75_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13, eq76_e1992_d_b0, eq76_e1992_d_b1, eq76_e1992_d_b2, eq76_e1992_d_b3, eq76_e1992_d_b4, eq76_e1992_d_b5, eq76_e1992_d_b6, eq76_e1992_d_b7, eq76_e1992_d_b8, eq76_e1992_d_b9, eq76_e1992_d_b10, eq76_e1992_d_b11, eq76_e1992_d_b12, eq76_e1992_d_b13, eq76_e1992_d_b14, eq76_e1992_d_b15, eq76_e1992_d_b16, eq76_e1992_d_b17, eq76_e1992_q, eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13, eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && (!s.b[1564])) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1981_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq76_e1979 * s.db[822][0]));
        let eq76_e1981_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq76_e1979 * s.db[822][1]));
        let eq76_e1981_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq76_e1979 * s.db[822][2]));
        let eq76_e1981_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq76_e1979 * s.db[822][3]));
        let eq76_e1981_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq76_e1979 * s.db[822][4]));
        let eq76_e1981_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq76_e1979 * s.db[822][5]));
        let eq76_e1981_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq76_e1979 * s.db[822][6]));
        let eq76_e1981_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq76_e1979 * s.db[822][7]));
        let eq76_e1981_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq76_e1979 * s.db[822][8]));
        let eq76_e1981_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq76_e1979 * s.db[822][9]));
        let eq76_e1981_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq76_e1979 * s.db[822][10]));
        let eq76_e1981_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq76_e1979 * s.db[822][11]));
        let eq76_e1981_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq76_e1979 * s.db[822][12]));
        let eq76_e1981_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq76_e1979 * s.db[822][13]));
        let eq76_e1981_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq76_e1979 * s.db[822][14]));
        let eq76_e1981_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq76_e1979 * s.db[822][15]));
        let eq76_e1981_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq76_e1979 * s.db[822][16]));
        let eq76_e1981_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq76_e1979 * s.db[822][17]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1985_q: f64 = eq76_e1984;
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1984);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + (s.dn[410][0] * s.v[158]));
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + (s.dn[410][1] * s.v[158]));
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + (s.dn[410][2] * s.v[158]));
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + (s.dn[410][3] * s.v[158]));
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + (s.dn[410][4] * s.v[158]));
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + (s.dn[410][5] * s.v[158]));
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + (s.dn[410][6] * s.v[158]));
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + (s.dn[410][7] * s.v[158]));
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + (s.dn[410][8] * s.v[158]));
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + (s.dn[410][9] * s.v[158]));
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + (s.dn[410][10] * s.v[158]));
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + (s.dn[410][11] * s.v[158]));
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + (s.dn[410][12] * s.v[158]));
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + (s.dn[410][13] * s.v[158]));
        let eq76_e1986_d_b0: f64 = (eq76_e1981_d_b0 + (s.db[410][0] * s.v[158]));
        let eq76_e1986_d_b1: f64 = (eq76_e1981_d_b1 + (s.db[410][1] * s.v[158]));
        let eq76_e1986_d_b2: f64 = (eq76_e1981_d_b2 + (s.db[410][2] * s.v[158]));
        let eq76_e1986_d_b3: f64 = (eq76_e1981_d_b3 + (s.db[410][3] * s.v[158]));
        let eq76_e1986_d_b4: f64 = (eq76_e1981_d_b4 + (s.db[410][4] * s.v[158]));
        let eq76_e1986_d_b5: f64 = (eq76_e1981_d_b5 + (s.db[410][5] * s.v[158]));
        let eq76_e1986_d_b6: f64 = (eq76_e1981_d_b6 + (s.db[410][6] * s.v[158]));
        let eq76_e1986_d_b7: f64 = (eq76_e1981_d_b7 + (s.db[410][7] * s.v[158]));
        let eq76_e1986_d_b8: f64 = (eq76_e1981_d_b8 + (s.db[410][8] * s.v[158]));
        let eq76_e1986_d_b9: f64 = (eq76_e1981_d_b9 + (s.db[410][9] * s.v[158]));
        let eq76_e1986_d_b10: f64 = (eq76_e1981_d_b10 + (s.db[410][10] * s.v[158]));
        let eq76_e1986_d_b11: f64 = (eq76_e1981_d_b11 + (s.db[410][11] * s.v[158]));
        let eq76_e1986_d_b12: f64 = (eq76_e1981_d_b12 + (s.db[410][12] * s.v[158]));
        let eq76_e1986_d_b13: f64 = (eq76_e1981_d_b13 + (s.db[410][13] * s.v[158]));
        let eq76_e1986_d_b14: f64 = (eq76_e1981_d_b14 + (s.db[410][14] * s.v[158]));
        let eq76_e1986_d_b15: f64 = (eq76_e1981_d_b15 + (s.db[410][15] * s.v[158]));
        let eq76_e1986_d_b16: f64 = (eq76_e1981_d_b16 + (s.db[410][16] * s.v[158]));
        let eq76_e1986_d_b17: f64 = (eq76_e1981_d_b17 + (s.db[410][17] * s.v[158]));
        let eq76_e1986_q: f64 = eq76_e1985_q;
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq76_e1989: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq76_e1989_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq76_e1989_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq76_e1989_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n0: f64 = (eq76_e1986_d_n0 + eq76_e1989_d_n0);
        let eq76_e1990_d_n1: f64 = (eq76_e1986_d_n1 + eq76_e1989_d_n1);
        let eq76_e1990_d_n2: f64 = (eq76_e1986_d_n2 + eq76_e1989_d_n2);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_d_n13: f64 = (eq76_e1986_d_n13 + eq76_e1989_d_n13);
        let eq76_e1990_d_b0: f64 = (eq76_e1986_d_b0 + eq76_e1989_d_b0);
        let eq76_e1990_d_b1: f64 = (eq76_e1986_d_b1 + eq76_e1989_d_b1);
        let eq76_e1990_d_b2: f64 = (eq76_e1986_d_b2 + eq76_e1989_d_b2);
        let eq76_e1990_d_b3: f64 = (eq76_e1986_d_b3 + eq76_e1989_d_b3);
        let eq76_e1990_d_b4: f64 = (eq76_e1986_d_b4 + eq76_e1989_d_b4);
        let eq76_e1990_d_b5: f64 = (eq76_e1986_d_b5 + eq76_e1989_d_b5);
        let eq76_e1990_d_b6: f64 = (eq76_e1986_d_b6 + eq76_e1989_d_b6);
        let eq76_e1990_d_b7: f64 = (eq76_e1986_d_b7 + eq76_e1989_d_b7);
        let eq76_e1990_d_b8: f64 = (eq76_e1986_d_b8 + eq76_e1989_d_b8);
        let eq76_e1990_d_b9: f64 = (eq76_e1986_d_b9 + eq76_e1989_d_b9);
        let eq76_e1990_d_b10: f64 = (eq76_e1986_d_b10 + eq76_e1989_d_b10);
        let eq76_e1990_d_b11: f64 = (eq76_e1986_d_b11 + eq76_e1989_d_b11);
        let eq76_e1990_d_b12: f64 = (eq76_e1986_d_b12 + eq76_e1989_d_b12);
        let eq76_e1990_d_b13: f64 = (eq76_e1986_d_b13 + eq76_e1989_d_b13);
        let eq76_e1990_d_b14: f64 = (eq76_e1986_d_b14 + eq76_e1989_d_b14);
        let eq76_e1990_d_b15: f64 = (eq76_e1986_d_b15 + eq76_e1989_d_b15);
        let eq76_e1990_d_b16: f64 = (eq76_e1986_d_b16 + eq76_e1989_d_b16);
        let eq76_e1990_d_b17: f64 = (eq76_e1986_d_b17 + eq76_e1989_d_b17);
        let eq76_e1990_q: f64 = eq76_e1986_q;
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13, eq76_e1990_d_b0, eq76_e1990_d_b1, eq76_e1990_d_b2, eq76_e1990_d_b3, eq76_e1990_d_b4, eq76_e1990_d_b5, eq76_e1990_d_b6, eq76_e1990_d_b7, eq76_e1990_d_b8, eq76_e1990_d_b9, eq76_e1990_d_b10, eq76_e1990_d_b11, eq76_e1990_d_b12, eq76_e1990_d_b13, eq76_e1990_d_b14, eq76_e1990_d_b15, eq76_e1990_d_b16, eq76_e1990_d_b17, eq76_e1990_q, (s.dn[410][0] * s.v[158]), (s.dn[410][1] * s.v[158]), (s.dn[410][2] * s.v[158]), (s.dn[410][3] * s.v[158]), (s.dn[410][4] * s.v[158]), (s.dn[410][5] * s.v[158]), (s.dn[410][6] * s.v[158]), (s.dn[410][7] * s.v[158]), (s.dn[410][8] * s.v[158]), (s.dn[410][9] * s.v[158]), (s.dn[410][10] * s.v[158]), (s.dn[410][11] * s.v[158]), (s.dn[410][12] * s.v[158]), (s.dn[410][13] * s.v[158]), (s.db[410][0] * s.v[158]), (s.db[410][1] * s.v[158]), (s.db[410][2] * s.v[158]), (s.db[410][3] * s.v[158]), (s.db[410][4] * s.v[158]), (s.db[410][5] * s.v[158]), (s.db[410][6] * s.v[158]), (s.db[410][7] * s.v[158]), (s.db[410][8] * s.v[158]), (s.db[410][9] * s.v[158]), (s.db[410][10] * s.v[158]), (s.db[410][11] * s.v[158]), (s.db[410][12] * s.v[158]), (s.db[410][13] * s.v[158]), (s.db[410][14] * s.v[158]), (s.db[410][15] * s.v[158]), (s.db[410][16] * s.v[158]), (s.db[410][17] * s.v[158]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_reactive_node_derivatives: [f64; 14] = [eq76_e1992_q_d_n0, eq76_e1992_q_d_n1, eq76_e1992_q_d_n2, eq76_e1992_q_d_n3, eq76_e1992_q_d_n4, eq76_e1992_q_d_n5, eq76_e1992_q_d_n6, eq76_e1992_q_d_n7, eq76_e1992_q_d_n8, eq76_e1992_q_d_n9, eq76_e1992_q_d_n10, eq76_e1992_q_d_n11, eq76_e1992_q_d_n12, eq76_e1992_q_d_n13];
        let eq76_reactive_branch_derivatives: [f64; 18] = [eq76_e1992_q_d_b0, eq76_e1992_q_d_b1, eq76_e1992_q_d_b2, eq76_e1992_q_d_b3, eq76_e1992_q_d_b4, eq76_e1992_q_d_b5, eq76_e1992_q_d_b6, eq76_e1992_q_d_b7, eq76_e1992_q_d_b8, eq76_e1992_q_d_b9, eq76_e1992_q_d_b10, eq76_e1992_q_d_b11, eq76_e1992_q_d_b12, eq76_e1992_q_d_b13, eq76_e1992_q_d_b14, eq76_e1992_q_d_b15, eq76_e1992_q_d_b16, eq76_e1992_q_d_b17];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            None,
            nodes,
            &eq76_reactive_node_derivatives,
            branches,
            &eq76_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
