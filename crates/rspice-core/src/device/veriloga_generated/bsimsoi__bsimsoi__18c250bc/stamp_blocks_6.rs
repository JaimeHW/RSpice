#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1424] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1424, if s.b[1424] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1424]) {s.store_add_mixed_ai(1015, A::add_scaled_inputs_product(s.ad_value(1014), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(943), (-1.0)), 324);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1424])) {s.store_add(1015, 67, 324);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_add_scaled_inputs3_offset_indices(813, 1015, 1.0, 825, (-1.0), 841, 1.0, (-0.02));}
        s.b[1425] = (s.v[1015] <= 0.0);s.store_scalar(1425, if s.b[1425] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1425]) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (-(4.0 * 0.02)));}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1425])) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (4.0 * 0.02));}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_add_scaled_inputs3_indices(812, 1015, 1.0, 813, (-0.5), 843, (-0.5));}
        s.b[1426] = (p.p27 > 0.0);s.store_scalar(1426, if s.b[1426] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {s.store_offset(1126, 1015, p.p1033);s.store_add_scaled_inputs3_offset_indices(813, 1126, 1.0, 1125, (-1.0), 841, 1.0, (-0.02));}
        s.b[1427] = (s.v[1126] <= 0.0);s.store_scalar(1427, if s.b[1427] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && s.b[1427]) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (-(100.0 * 0.02)));}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && (!s.b[1427])) {s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (100.0 * 0.02));}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {s.store_add_scaled_inputs3_indices(1128, 1126, 1.0, 813, (-0.5), 843, (-0.5));}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div_scaled_inputs3_indices(843, 825, 1.0, 841, (-1.0), 1015, -1.0, 998, 1.0);s.store_mul(859, 843, 361);}
        s.b[1428] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));s.store_scalar(1428, if s.b[1428] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1428]) {s.store_mul_exp_rhs(999, 360, 859);}
        s.b[1429] = (s.v[859] <= (-100.0));s.store_scalar(1429, if s.b[1429] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && s.b[1429]) {s.store_scale(999, 360, 3.720075976e-44);}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && (!s.b[1429])) {s.store_scale(999, 360, 2.688117142e43);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_scale(1000, 62, 0.001);s.store_add_scaled_inputs3_indices(813, 360, 1.0, 999, (-1.0), 1000, -1.0);s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);s.store_add_scaled_inputs3_indices(999, 360, 1.0, 813, (-0.5), 814, (-0.5));}
        s.b[1430] = (s.v[999] < 1e-15);s.store_scalar(1430, if s.b[1430] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1430]) {s.store_scalar(999, 1e-15);}
        s.b[1431] = (p.p27 > 0.0);s.store_scalar(1431, if s.b[1431] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {s.store_div_scaled_inputs3_indices(843, 1125, 1.0, 841, (-1.0), 1126, -1.0, 998, 1.0);s.store_mul(859, 843, 361);}
        s.b[1432] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));s.store_scalar(1432, if s.b[1432] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1432]) {s.store_mul_exp_rhs(1131, 360, 859);}
        s.b[1433] = (s.v[859] <= (-100.0));s.store_scalar(1433, if s.b[1433] { 1.0 } else { 0.0 });
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && s.b[1433]) {s.store_scale(1131, 360, 3.720075976e-44);}
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && (!s.b[1433])) {s.store_scale(1131, 360, 2.688117142e43);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {s.store_add_scaled_inputs3_indices(813, 360, 1.0, 1131, (-1.0), 1000, -1.0);s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);s.store_add_scaled_inputs3_indices(1131, 360, 1.0, 813, (-0.5), 814, (-0.5));}
        s.b[1434] = (s.v[1131] < 1e-15);s.store_scalar(1434, if s.b[1434] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1434]) {s.store_scalar(1131, 1e-15);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div(1001, 417, 999);s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1001, 1.0);s.store_mul(1002, 845, 1001);}
        s.b[1435] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1435, if s.b[1435] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1435]) {s.store_div(1132, 417, 1131);s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1132, 1.0);s.store_mul(1133, 845, 1132);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);}
        s.b[1436] = (p.p27 > 0.0);s.store_scalar(1436, if s.b[1436] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1436]) {s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_mul_sub_rhs(938, 982, 812, 1015);}
        s.b[1437] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1437, if s.b[1437] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1437]) {s.store_mul_sub_rhs(1123, 1135, 1128, 1126);s.store_add(938, 938, 1123);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_scale(843, 376, 0.5);s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);}
        s.b[1438] = (s.v[376] == 0.0);s.store_scalar(1438, if s.b[1438] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1438]) {s.store_scalar(844, 0.0);}
        s.b[1439] = (s.v[846] < 0.0);s.store_scalar(1439, if s.b[1439] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && s.b[1439]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && (!s.b[1439])) {s.store_sqrt_square_add(844, 843, 846);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {s.store_mul_ad_product_rhs_mixed_ia(937, 982, 376, A::sub(s.ad_value(844), s.ad_value(843)));}
        s.b[1440] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1440, if s.b[1440] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);}
        s.b[1441] = (s.v[376] == 0.0);s.store_scalar(1441, if s.b[1441] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && s.b[1441]) {s.store_scalar(844, 0.0);}
        s.b[1442] = (s.v[846] < 0.0);s.store_scalar(1442, if s.b[1442] { 1.0 } else { 0.0 });
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && s.b[1442]) {s.store_add_div_rhs_indices(844, 843, 846, 376);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && (!s.b[1442])) {s.store_sqrt_square_add(844, 843, 846);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {s.store_mul_ad_product_rhs_mixed_ia(1124, 1135, 376, A::sub(s.ad_value(844), s.ad_value(843)));s.store_add(937, 937, 1124);}
        s.b[1443] = (s.v[376] <= 0.0);s.store_scalar(1443, if s.b[1443] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1443]) {s.store_scaled_mul(936, 362, 832, 0.25);s.store_scale(843, 339, 0.5);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1443])) {s.store_mul_product3_indices(936, 376, 362, 832, 376, 1.0);s.store_mul(843, 376, 339);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs(844, 843, 2.0, 875, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {
            s.store_mul_mixed_ia(1004, 832, {
                            if ((1.0 + ((s.v[844] * s.v[875]) / s.v[936])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(875), 1.0, s.ad_value(936), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        s.b[1444] = (p.p27 > 0.0);s.store_scalar(1444, if s.b[1444] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {s.store_add_scaled_inputs(844, 843, 2.0, 1118, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_mul_mixed_ia(1136, 832, {
                            if ((1.0 + ((s.v[844] * s.v[1118]) / s.v[936])) > 1e-38) {
                                A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(1118), 1.0, s.ad_value(936), 1.0), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs3_indices(846, 829, 4.0, 1015, ((-1.0) * 4.0), 942, (-4.0));s.store_sqrt_square_offset(845, 846, 0.0001);s.store_scaled_add(847, 846, 845, 0.5);s.store_scale(998, 998, 2.0);s.store_div_scaled_inputs2_indices(843, 875, 1.0, 847, 1.0, 998, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }
        if ((!s.b[1402]) && s.b[1420]) {s.store_offset(844, 859, 1.0);s.store_div_from_scalar(999, (p.p58 * 1.9e-9), 844);s.store_div(1001, 417, 999);s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1001, 1.0);s.store_mul(1002, 843, 1001);s.store_div_scaled_product_indices(1003, 842, 1002, 1.0, 997, 1.0);s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);}
        s.b[1445] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1445, if s.b[1445] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {s.store_add_scaled_inputs3_offset_indices(846, 829, 4.0, 1126, ((-1.0) * 4.0), 942, (-4.0), (p.p1033 * 4.0));s.store_sqrt_square_offset(845, 846, 0.0001);s.store_scaled_add(847, 846, 845, 0.5);s.store_div_scaled_inputs2_indices(843, 1118, 1.0, 847, 1.0, 998, 1.0);}
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }
        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {s.store_offset(844, 859, 1.0);s.store_div_from_scalar(1131, (p.p58 * 1.9e-9), 844);s.store_div(1132, 417, 1131);s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1132, 1.0);s.store_mul(1133, 843, 1132);s.store_div_scaled_product_indices(1134, 1115, 1133, 1.0, 997, 1.0);s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_sub(844, 875, 1004);s.store_mul(894, 861, 333);s.store_div(891, 844, 894);s.store_offset_sub(814, 891, 822, (-0.02));s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));s.store_mul(843, 894, 877);s.store_scaled_offset_ad(845, A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);s.store_div(846, 843, 845);s.store_mul_sub_mixed_iia(915, 1003, 844, A::mul_sub_from_scalar_rhs(s.ad_value(843), 0.5, s.ad_value(846)));s.copy_ad(916, 915);}
        s.b[1446] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1446, if s.b[1446] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {s.store_sub(855, 1118, 1136);s.store_div(1129, 855, 894);s.store_offset_sub(814, 1129, 822, (-0.02));s.store_sqrt_add_scaled_square_input(1121, 814, 1.0, 1129, (4.0 * 0.02));s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 1121, (-0.5));s.store_mul(1121, 894, 1130);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {s.store_scaled_offset_ad(1122, A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);s.store_div(846, 1121, 1122);s.store_mul_sub_mixed_iia(850, 1134, 855, A::mul_sub_from_scalar_rhs(s.ad_value(1121), 0.5, s.ad_value(846)));s.store_add(915, 915, 850);s.copy_ad(916, 915);}
        s.b[1447] = (s.v[37] == 2.0);s.store_scalar(1447, if s.b[1447] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1447]) {s.store_scalar(1006, 0.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) {s.store_sub_from_scalar(850, 1.0, 894);s.store_mul_ad_product_rhs_mixed_ia(1006, 982, 850, A::sub_scaled_inputs(s.ad_value(877), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(877), 1.0, s.ad_value(845), 1.0), 1.0));}
        s.b[1448] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1448, if s.b[1448] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) && s.b[1448]) {s.store_mul_ad_product_rhs_mixed_ia(1138, 1135, 850, A::sub_scaled_inputs(s.ad_value(1130), 0.5, A::div_scaled_product(s.ad_value(1121), s.ad_value(1130), 1.0, s.ad_value(1122), 1.0), 1.0));s.store_add(1006, 1006, 1138);}
        s.b[1449] = (p.p129 > 0.5);s.store_scalar(1449, if s.b[1449] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1449]) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_iia(919, 1003, 844, ((0.5) * (-1.0)), 843, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 0.5, s.ad_value(845), 1.0), ((-1.0) * (-1.0)), 0.0);}
        s.b[1450] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1450, if s.b[1450] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && s.b[1449]) && s.b[1450]) {s.store_mul_add_scaled_inputs4_rhs_mixed_iiia(1137, 1134, 1118, ((0.5) * (-1.0)), 1136, (((-0.5)) * (-1.0)), 1121, ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 0.5, s.ad_value(1122), 1.0), ((-1.0) * (-1.0)));s.store_add(919, 919, 1137);}
        s.b[1451] = (p.p129 < 0.5);s.store_scalar(1451, if s.b[1451] { 1.0 } else { 0.0 });
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) {s.store_scale(845, 845, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(846, 1003, 0.5, 845, 1.0);s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 844, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(844), A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(919, 847, 846, -1.0, 0.0);}
        s.b[1452] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));s.store_scalar(1452, if s.b[1452] { 1.0 } else { 0.0 });
        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) && s.b[1452]) {s.store_scale(1122, 1122, 0.08333333333333333);s.store_div_scaled_inputs_square_rhs(846, 1134, 0.5, 1122, 1.0);s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 855, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(855), A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);s.store_mul_scale_offset_indices(1137, 847, 846, -1.0, 0.0);s.store_add(919, 919, 1137);}
        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && (!s.b[1451])) {s.store_scale(919, 916, (-0.5));}
        s.b[1453] = (s.v[37] == 2.0);s.store_scalar(1453, if s.b[1453] { 1.0 } else { 0.0 });
        if (((!s.b[1402]) && s.b[1420]) && s.b[1453]) {s.store_scalar(939, 0.0);}
        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1453])) {s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));s.store_mul_sub_rhs(939, 914, 902, 824);}
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs4_indices(916, 916, 1.0, 938, 1.0, 937, 1.0, 1006, -1.0);s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);s.copy_ad(920, 939);s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 917, (-1.0), 920, (-1.0), 919, (-1.0));}
        if ((!s.b[1402]) && (!s.b[1420])) {s.store_scalar(938, 0.0);s.store_scalar(937, 0.0);s.store_scalar(920, 0.0);s.store_scalar(917, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_46(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1402]) && (!s.b[1420])) {s.store_scalar(919, 0.0);s.store_scalar(918, 0.0);s.store_scalar(916, 0.0);}
        s.b[1454] = (s.v[37] == 2.0);s.store_scalar(1454, if s.b[1454] { 1.0 } else { 0.0 });
        if s.b[1454] {s.store_scalar(909, 0.0);s.store_scalar(910, 0.0);}
        if (!s.b[1454]) {s.copy_ad(815, 48);s.store_scalar(980, (-p.p363));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p.p183);s.store_scalar(976, ((((p.p185 * s.v[350]) * p.p155) * p.p3) / 1e-7));s.store_scale(979, 976, p.p362);s.store_add_scaled_product_right_sub(976, 976, 1.0, 979, 409, 429, 1.0);s.store_scalar(977, ((((p.p186 * s.v[349]) * p.p155) * p.p3) / 1e-7));s.store_scale(978, 977, p.p364);s.store_add_scaled_product_right_sub(977, 977, 1.0, 978, 409, 429, 1.0);s.store_scale(994, 815, 0.9);}
        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1087] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1087)
                }
            }, 815);
        }
        s.b[1455] = (s.v[816] == 0.5);s.store_scalar(1455, if s.b[1455] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1455]) {s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));}
        if ((!s.b[1454]) && (!s.b[1455])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }
        if (!s.b[1454]) {s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));}
        s.b[1456] = (s.v[1087] > s.v[994]);s.store_scalar(1456, if s.b[1456] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1456]) {s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1087, 994, 1.0);}
        if (!s.b[1454]) {s.store_add_scaled_product_indices(910, 987, (p.p351 * p.p3), 976, 846, 1.0);s.copy_ad(815, 41);s.store_scalar(980, (-p.p365));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p.p184);s.store_scale(994, 815, 0.9);}
        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1088] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1088)
                }
            }, 815);
        }
        s.b[1457] = (s.v[816] == 0.5);s.store_scalar(1457, if s.b[1457] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1457]) {s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));}
        if ((!s.b[1454]) && (!s.b[1457])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }
        if (!s.b[1454]) {s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));}
        s.b[1458] = (s.v[1088] > s.v[994]);s.store_scalar(1458, if s.b[1458] { 1.0 } else { 0.0 });
        if ((!s.b[1454]) && s.b[1458]) {s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1088, 994, 1.0);}
        if (!s.b[1454]) {s.store_add_scaled_product_indices(909, 988, (p.p351 * p.p3), 977, 846, 1.0);}
        s.store_scale(853, 897, (-p.p37));s.store_scaled_sub(854, 819, 897, p.p37);s.b[1459] = (s.v[43] != 0.0);s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });s.b[1460] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });s.b[1461] = (s.v[853] < s.v[322]);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1460]) && s.b[1461]) {s.store_scaled_sub(86, 853, 322, s.v[52]);}
        s.b[1462] = (s.v[853] < s.v[175]);s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1460]) && (!s.b[1461])) && s.b[1462]) {s.store_sub(843, 853, 322);s.store_square(844, 843);s.store_mul_scale_offset_mixed_ia(86, 843, A::mul_scaled_lhs(s.ad_value(176), 1.0 / (3.0), s.ad_value(844)), -1.0, s.v[52]);}
        s.b[1463] = (s.v[853] < s.v[323]);s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_sub(843, 853, 323);s.store_square(844, 843);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_47(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));}
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && (!s.b[1463])) {s.store_add_scaled_product_indices(86, 56, 1.0, 53, 853, 1.0);}
        s.b[1464] = (s.v[853] < s.v[323]);s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });
        if ((s.b[1459] && (!s.b[1460])) && s.b[1464]) {s.store_mul_sub_rhs(86, 53, 853, 323);}
        s.b[1465] = (s.v[853] < s.v[175]);s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });
        if (((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && s.b[1465]) {s.store_sub(843, 853, 323);s.store_square(844, 843);s.store_mul_add_scaled_product_rhs_indices(86, 843, 53, 1.0, 176, 844, (-1.0 / (3.0)));}
        s.b[1466] = (s.v[853] < s.v[322]);s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && s.b[1466]) {s.store_sub(843, 853, 322);s.store_square(844, 843);s.store_add_scaled_inputs3_mixed_iia(86, 853, s.v[52], 56, 1.0, A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && (!s.b[1466])) {s.store_add_scaled_inputs(86, 853, s.v[52], 56, 1.0);}
        s.b[1467] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });s.b[1468] = (s.v[854] < s.v[322]);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
        if ((s.b[1459] && s.b[1467]) && s.b[1468]) {s.store_scaled_sub(87, 854, 322, s.v[54]);}
        s.b[1469] = (s.v[854] < s.v[175]);s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });
        if (((s.b[1459] && s.b[1467]) && (!s.b[1468])) && s.b[1469]) {s.store_sub(843, 854, 322);s.store_square(844, 843);s.store_mul_scale_offset_mixed_ia(87, 843, A::mul_scaled_lhs(s.ad_value(178), 1.0 / (3.0), s.ad_value(844)), -1.0, s.v[54]);}
        s.b[1470] = (s.v[854] < s.v[323]);s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && s.b[1470]) {s.store_sub(843, 854, 323);s.store_square(844, 843);s.store_add_ad(87, A::add_scaled_product(s.ad_value(57), 1.0, s.ad_value(55), s.ad_value(854), 1.0), A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));}
        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && (!s.b[1470])) {s.store_add_scaled_product_indices(87, 57, 1.0, 55, 854, 1.0);}
        s.b[1471] = (s.v[854] < s.v[323]);s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });
        if ((s.b[1459] && (!s.b[1467])) && s.b[1471]) {s.store_mul_sub_rhs(87, 55, 854, 323);}
        s.b[1472] = (s.v[854] < s.v[175]);s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });
        if (((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {s.store_sub(843, 854, 323);s.store_square(844, 843);s.store_mul_add_scaled_product_rhs_indices(87, 843, 55, 1.0, 178, 844, (-1.0 / (3.0)));}
        s.b[1473] = (s.v[854] < s.v[322]);s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });
        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {s.store_sub(843, 854, 322);s.store_square(844, 843);s.store_add_scaled_inputs3_mixed_iia(87, 854, s.v[54], 57, 1.0, A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);}
        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {s.store_add_scaled_inputs(87, 854, s.v[54], 57, 1.0);}
        if (!s.b[1459]) {s.store_scale(86, 853, s.v[52]);s.store_scale(87, 854, s.v[54]);}
        s.store_add_scaled_product_indices(86, 86, 1.0, 58, 853, 1.0);s.store_add_scaled_product_indices(87, 87, 1.0, 59, 854, 1.0);s.b[1474] = (p.p39 == 3.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if s.b[1474] {s.store_offset(843, 1019, 0.02);}
        if (!s.b[1474]) {s.store_offset(843, 820, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 237, s.v[349]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1475] = (p.p39 == 3.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_48(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1475] {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1475]) {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1476] = (p.p39 == 3.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if s.b[1476] {s.store_offset(843, 1018, 0.02);}
        if (!s.b[1476]) {s.store_offset(843, 821, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 236, s.v[350]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1477] = (p.p39 == 3.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if s.b[1477] {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1477]) {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1478] = (p.p3 != 1.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if s.b[1478] {s.store_scale(895, 895, p.p3);s.store_scale(896, 896, p.p3);}
        s.b[1505] = (p.p223 == 0.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = (p.p223 == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });s.b[1507] = (p.p223 == 2.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });s.b[1508] = (p.p223 == 3.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);s.store_square(843, 843);s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);s.store_square(848, 848);s.store_offset_scaled(1487, 848, (((p.p227 * s.v[327])) * (p.p229)), p.p229);s.store_add_scaled_product_mixed_iia(844, 84, 1.0, 1487, A::add(s.ad_value(83), s.ad_value(85)), 1.0);s.store_div_scaled_product_indices(845, 844, 844, 1.0, 78, 1.0);}
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_sub_from_scalar_scaled_mul(1491, 1.0, 77, 76, 1.0);s.store_sub_from_scalar(843, 1.0, 1491);s.store_offset(844, 1491, 1.0);s.store_add_mixed_ia(845, 844, A::div_scaled_product_offset_denominator(s.ad_value(74), s.ad_value(49), 2.0, s.ad_value(72), 1e-10, 1.0));s.store_offset_scaled_div(1495, 77, 838, s.v[892], s.v[892]);s.store_div_from_scalar(849, s.v[892], 1495);s.store_square(846, 845);s.store_square(847, 843);s.store_square(848, 846);s.store_div(850, 843, 845);s.store_div(851, 72, 838);s.store_square(851, 851);s.store_offset_scaled(1487, 851, (((p.p227 * s.v[892])) * (p.p229)), p.p229);s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));}
        s.b[1548] = (s.v[398] > 0.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if s.b[1548] {s.store_scale(92, 918, p.p37);s.store_scale(93, 919, p.p37);}
        if (!s.b[1548]) {s.store_scale(93, 918, p.p37);s.store_scale(92, 919, p.p37);}
        s.b[1553] = (p.p39 == 3.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });s.b[1559] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_49(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1560] = ((p.p35 != 0.0) && (!true));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = true;s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = true;s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = (p.p430 == 2.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });s.b[1564] = (p.p430 == 2.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });s.copy_ad(426, 916);s.copy_ad(427, 918);s.copy_ad(428, 919);s.store_add(425, 896, 895);s.store_sub(918, 427, 895);s.store_sub(919, 428, 896);s.store_add(916, 426, 425);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq0_e1156,) = {
    if (((s.b[431] && s.b[432]) && s.b[433]) && s.b[434]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1156;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e1169,) = {
    if ((((s.b[431] && s.b[432]) && s.b[433]) && (!s.b[434])) && s.b[435]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq1_value: f64 = eq1_e1169;
        stamper.stamp_potential_const_local(
            1,
            eq1_value,
        );
        let (eq2_e1180,) = {
    if (((s.b[431] && s.b[432]) && (!s.b[433])) && s.b[436]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq2_value: f64 = eq2_e1180;
        stamper.stamp_potential_const_local(
            2,
            eq2_value,
        );
        let (eq3_e1191,) = {
    if (((s.b[431] && (!s.b[432])) && s.b[437]) && s.b[438]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1191;
        stamper.stamp_potential_const_local(
            3,
            eq3_value,
        );
        let (eq4_e1205,) = {
    if ((((s.b[431] && (!s.b[432])) && s.b[437]) && (!s.b[438])) && s.b[439]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1205;
        stamper.stamp_potential_const_local(
            4,
            eq4_value,
        );
        let (eq5_e1222,) = {
    if (((((s.b[431] && (!s.b[432])) && s.b[437]) && (!s.b[438])) && (!s.b[439])) && s.b[440]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq5_value: f64 = eq5_e1222;
        stamper.stamp_potential_const_local(
            5,
            eq5_value,
        );
        let (eq6_e1234,) = {
    if (((s.b[431] && (!s.b[432])) && (!s.b[437])) && s.b[441]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq6_value: f64 = eq6_e1234;
        stamper.stamp_potential_const_local(
            6,
            eq6_value,
        );
        let (eq11_e1314, eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13, eq11_e1314_d_b0, eq11_e1314_d_b1, eq11_e1314_d_b2, eq11_e1314_d_b3, eq11_e1314_d_b4, eq11_e1314_d_b5, eq11_e1314_d_b6, eq11_e1314_d_b7, eq11_e1314_d_b8, eq11_e1314_d_b9, eq11_e1314_d_b10, eq11_e1314_d_b11, eq11_e1314_d_b12, eq11_e1314_d_b13, eq11_e1314_d_b14, eq11_e1314_d_b15, eq11_e1314_d_b16, eq11_e1314_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq11_e1308: f64 = (p.p32 * (nv13 - 0.0));let eq11_e1310: f64 = (eq11_e1308 * s.v[1497]);let eq11_e1310_d_n13: f64 = ((p.p32 * s.v[1497]) + (eq11_e1308 * s.dn[1497][13]));let eq11_e1312: f64 = (eq11_e1310 * p.p226);let eq11_e1312_d_n0: f64 = ((eq11_e1308 * s.dn[1497][0]) * p.p226);let eq11_e1312_d_n1: f64 = ((eq11_e1308 * s.dn[1497][1]) * p.p226);let eq11_e1312_d_n2: f64 = ((eq11_e1308 * s.dn[1497][2]) * p.p226);let eq11_e1312_d_n3: f64 = ((eq11_e1308 * s.dn[1497][3]) * p.p226);let eq11_e1312_d_n4: f64 = ((eq11_e1308 * s.dn[1497][4]) * p.p226);let eq11_e1312_d_n5: f64 = ((eq11_e1308 * s.dn[1497][5]) * p.p226);let eq11_e1312_d_n6: f64 = ((eq11_e1308 * s.dn[1497][6]) * p.p226);let eq11_e1312_d_n7: f64 = ((eq11_e1308 * s.dn[1497][7]) * p.p226);let eq11_e1312_d_n8: f64 = ((eq11_e1308 * s.dn[1497][8]) * p.p226);let eq11_e1312_d_n9: f64 = ((eq11_e1308 * s.dn[1497][9]) * p.p226);let eq11_e1312_d_n10: f64 = ((eq11_e1308 * s.dn[1497][10]) * p.p226);let eq11_e1312_d_n11: f64 = ((eq11_e1308 * s.dn[1497][11]) * p.p226);let eq11_e1312_d_n12: f64 = ((eq11_e1308 * s.dn[1497][12]) * p.p226);let eq11_e1312_d_n13: f64 = (eq11_e1310_d_n13 * p.p226);let eq11_e1312_d_b0: f64 = ((eq11_e1308 * s.db[1497][0]) * p.p226);let eq11_e1312_d_b1: f64 = ((eq11_e1308 * s.db[1497][1]) * p.p226);let eq11_e1312_d_b2: f64 = ((eq11_e1308 * s.db[1497][2]) * p.p226);let eq11_e1312_d_b3: f64 = ((eq11_e1308 * s.db[1497][3]) * p.p226);let eq11_e1312_d_b4: f64 = ((eq11_e1308 * s.db[1497][4]) * p.p226);let eq11_e1312_d_b5: f64 = ((eq11_e1308 * s.db[1497][5]) * p.p226);let eq11_e1312_d_b6: f64 = ((eq11_e1308 * s.db[1497][6]) * p.p226);let eq11_e1312_d_b7: f64 = ((eq11_e1308 * s.db[1497][7]) * p.p226);let eq11_e1312_d_b8: f64 = ((eq11_e1308 * s.db[1497][8]) * p.p226);let eq11_e1312_d_b9: f64 = ((eq11_e1308 * s.db[1497][9]) * p.p226);let eq11_e1312_d_b10: f64 = ((eq11_e1308 * s.db[1497][10]) * p.p226);let eq11_e1312_d_b11: f64 = ((eq11_e1308 * s.db[1497][11]) * p.p226);let eq11_e1312_d_b12: f64 = ((eq11_e1308 * s.db[1497][12]) * p.p226);let eq11_e1312_d_b13: f64 = ((eq11_e1308 * s.db[1497][13]) * p.p226);let eq11_e1312_d_b14: f64 = ((eq11_e1308 * s.db[1497][14]) * p.p226);let eq11_e1312_d_b15: f64 = ((eq11_e1308 * s.db[1497][15]) * p.p226);let eq11_e1312_d_b16: f64 = ((eq11_e1308 * s.db[1497][16]) * p.p226);let eq11_e1312_d_b17: f64 = ((eq11_e1308 * s.db[1497][17]) * p.p226);
        (eq11_e1312, eq11_e1312_d_n0, eq11_e1312_d_n1, eq11_e1312_d_n2, eq11_e1312_d_n3, eq11_e1312_d_n4, eq11_e1312_d_n5, eq11_e1312_d_n6, eq11_e1312_d_n7, eq11_e1312_d_n8, eq11_e1312_d_n9, eq11_e1312_d_n10, eq11_e1312_d_n11, eq11_e1312_d_n12, eq11_e1312_d_n13, eq11_e1312_d_b0, eq11_e1312_d_b1, eq11_e1312_d_b2, eq11_e1312_d_b3, eq11_e1312_d_b4, eq11_e1312_d_b5, eq11_e1312_d_b6, eq11_e1312_d_b7, eq11_e1312_d_b8, eq11_e1312_d_b9, eq11_e1312_d_b10, eq11_e1312_d_b11, eq11_e1312_d_b12, eq11_e1312_d_b13, eq11_e1312_d_b14, eq11_e1312_d_b15, eq11_e1312_d_b16, eq11_e1312_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1314;let eq11_node_derivatives: [f64; 14] = [eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13];let eq11_branch_derivatives: [f64; 18] = [eq11_e1314_d_b0, eq11_e1314_d_b1, eq11_e1314_d_b2, eq11_e1314_d_b3, eq11_e1314_d_b4, eq11_e1314_d_b5, eq11_e1314_d_b6, eq11_e1314_d_b7, eq11_e1314_d_b8, eq11_e1314_d_b9, eq11_e1314_d_b10, eq11_e1314_d_b11, eq11_e1314_d_b12, eq11_e1314_d_b13, eq11_e1314_d_b14, eq11_e1314_d_b15, eq11_e1314_d_b16, eq11_e1314_d_b17];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq13_e1356, eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13, eq13_e1356_d_b0, eq13_e1356_d_b1, eq13_e1356_d_b2, eq13_e1356_d_b3, eq13_e1356_d_b4, eq13_e1356_d_b5, eq13_e1356_d_b6, eq13_e1356_d_b7, eq13_e1356_d_b8, eq13_e1356_d_b9, eq13_e1356_d_b10, eq13_e1356_d_b11, eq13_e1356_d_b12, eq13_e1356_d_b13, eq13_e1356_d_b14, eq13_e1356_d_b15, eq13_e1356_d_b16, eq13_e1356_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq13_e1348: f64 = (p.p32 * s.v[1498]);let eq13_e1350: f64 = (eq13_e1348 * (nv13 - 0.0));let eq13_e1350_d_n0: f64 = ((p.p32 * s.dn[1498][0]) * (nv13 - 0.0));let eq13_e1350_d_n1: f64 = ((p.p32 * s.dn[1498][1]) * (nv13 - 0.0));let eq13_e1350_d_n2: f64 = ((p.p32 * s.dn[1498][2]) * (nv13 - 0.0));let eq13_e1350_d_n3: f64 = ((p.p32 * s.dn[1498][3]) * (nv13 - 0.0));let eq13_e1350_d_n4: f64 = ((p.p32 * s.dn[1498][4]) * (nv13 - 0.0));let eq13_e1350_d_n5: f64 = ((p.p32 * s.dn[1498][5]) * (nv13 - 0.0));let eq13_e1350_d_n6: f64 = ((p.p32 * s.dn[1498][6]) * (nv13 - 0.0));let eq13_e1350_d_n7: f64 = ((p.p32 * s.dn[1498][7]) * (nv13 - 0.0));let eq13_e1350_d_n8: f64 = ((p.p32 * s.dn[1498][8]) * (nv13 - 0.0));let eq13_e1350_d_n9: f64 = ((p.p32 * s.dn[1498][9]) * (nv13 - 0.0));let eq13_e1350_d_n10: f64 = ((p.p32 * s.dn[1498][10]) * (nv13 - 0.0));let eq13_e1350_d_n11: f64 = ((p.p32 * s.dn[1498][11]) * (nv13 - 0.0));let eq13_e1350_d_n12: f64 = ((p.p32 * s.dn[1498][12]) * (nv13 - 0.0));let eq13_e1350_d_n13: f64 = (((p.p32 * s.dn[1498][13]) * (nv13 - 0.0)) + eq13_e1348);let eq13_e1350_d_b0: f64 = ((p.p32 * s.db[1498][0]) * (nv13 - 0.0));let eq13_e1350_d_b1: f64 = ((p.p32 * s.db[1498][1]) * (nv13 - 0.0));let eq13_e1350_d_b2: f64 = ((p.p32 * s.db[1498][2]) * (nv13 - 0.0));let eq13_e1350_d_b3: f64 = ((p.p32 * s.db[1498][3]) * (nv13 - 0.0));let eq13_e1350_d_b4: f64 = ((p.p32 * s.db[1498][4]) * (nv13 - 0.0));let eq13_e1350_d_b5: f64 = ((p.p32 * s.db[1498][5]) * (nv13 - 0.0));let eq13_e1350_d_b6: f64 = ((p.p32 * s.db[1498][6]) * (nv13 - 0.0));let eq13_e1350_d_b7: f64 = ((p.p32 * s.db[1498][7]) * (nv13 - 0.0));let eq13_e1350_d_b8: f64 = ((p.p32 * s.db[1498][8]) * (nv13 - 0.0));let eq13_e1350_d_b9: f64 = ((p.p32 * s.db[1498][9]) * (nv13 - 0.0));let eq13_e1350_d_b10: f64 = ((p.p32 * s.db[1498][10]) * (nv13 - 0.0));let eq13_e1350_d_b11: f64 = ((p.p32 * s.db[1498][11]) * (nv13 - 0.0));let eq13_e1350_d_b12: f64 = ((p.p32 * s.db[1498][12]) * (nv13 - 0.0));let eq13_e1350_d_b13: f64 = ((p.p32 * s.db[1498][13]) * (nv13 - 0.0));let eq13_e1350_d_b14: f64 = ((p.p32 * s.db[1498][14]) * (nv13 - 0.0));let eq13_e1350_d_b15: f64 = ((p.p32 * s.db[1498][15]) * (nv13 - 0.0));let eq13_e1350_d_b16: f64 = ((p.p32 * s.db[1498][16]) * (nv13 - 0.0));let eq13_e1350_d_b17: f64 = ((p.p32 * s.db[1498][17]) * (nv13 - 0.0));let eq13_e1352: f64 = (eq13_e1350 * s.v[1497]);let eq13_e1352_d_n0: f64 = ((eq13_e1350_d_n0 * s.v[1497]) + (eq13_e1350 * s.dn[1497][0]));let eq13_e1352_d_n1: f64 = ((eq13_e1350_d_n1 * s.v[1497]) + (eq13_e1350 * s.dn[1497][1]));let eq13_e1352_d_n2: f64 = ((eq13_e1350_d_n2 * s.v[1497]) + (eq13_e1350 * s.dn[1497][2]));let eq13_e1352_d_n3: f64 = ((eq13_e1350_d_n3 * s.v[1497]) + (eq13_e1350 * s.dn[1497][3]));let eq13_e1352_d_n4: f64 = ((eq13_e1350_d_n4 * s.v[1497]) + (eq13_e1350 * s.dn[1497][4]));let eq13_e1352_d_n5: f64 = ((eq13_e1350_d_n5 * s.v[1497]) + (eq13_e1350 * s.dn[1497][5]));let eq13_e1352_d_n6: f64 = ((eq13_e1350_d_n6 * s.v[1497]) + (eq13_e1350 * s.dn[1497][6]));let eq13_e1352_d_n7: f64 = ((eq13_e1350_d_n7 * s.v[1497]) + (eq13_e1350 * s.dn[1497][7]));let eq13_e1352_d_n8: f64 = ((eq13_e1350_d_n8 * s.v[1497]) + (eq13_e1350 * s.dn[1497][8]));let eq13_e1352_d_n9: f64 = ((eq13_e1350_d_n9 * s.v[1497]) + (eq13_e1350 * s.dn[1497][9]));let eq13_e1352_d_n10: f64 = ((eq13_e1350_d_n10 * s.v[1497]) + (eq13_e1350 * s.dn[1497][10]));let eq13_e1352_d_n11: f64 = ((eq13_e1350_d_n11 * s.v[1497]) + (eq13_e1350 * s.dn[1497][11]));let eq13_e1352_d_n12: f64 = ((eq13_e1350_d_n12 * s.v[1497]) + (eq13_e1350 * s.dn[1497][12]));let eq13_e1352_d_n13: f64 = ((eq13_e1350_d_n13 * s.v[1497]) + (eq13_e1350 * s.dn[1497][13]));let eq13_e1352_d_b0: f64 = ((eq13_e1350_d_b0 * s.v[1497]) + (eq13_e1350 * s.db[1497][0]));let eq13_e1352_d_b1: f64 = ((eq13_e1350_d_b1 * s.v[1497]) + (eq13_e1350 * s.db[1497][1]));let eq13_e1352_d_b2: f64 = ((eq13_e1350_d_b2 * s.v[1497]) + (eq13_e1350 * s.db[1497][2]));let eq13_e1352_d_b3: f64 = ((eq13_e1350_d_b3 * s.v[1497]) + (eq13_e1350 * s.db[1497][3]));let eq13_e1352_d_b4: f64 = ((eq13_e1350_d_b4 * s.v[1497]) + (eq13_e1350 * s.db[1497][4]));
        let eq13_e1352_d_b5: f64 = ((eq13_e1350_d_b5 * s.v[1497]) + (eq13_e1350 * s.db[1497][5]));let eq13_e1352_d_b6: f64 = ((eq13_e1350_d_b6 * s.v[1497]) + (eq13_e1350 * s.db[1497][6]));let eq13_e1352_d_b7: f64 = ((eq13_e1350_d_b7 * s.v[1497]) + (eq13_e1350 * s.db[1497][7]));let eq13_e1352_d_b8: f64 = ((eq13_e1350_d_b8 * s.v[1497]) + (eq13_e1350 * s.db[1497][8]));let eq13_e1352_d_b9: f64 = ((eq13_e1350_d_b9 * s.v[1497]) + (eq13_e1350 * s.db[1497][9]));let eq13_e1352_d_b10: f64 = ((eq13_e1350_d_b10 * s.v[1497]) + (eq13_e1350 * s.db[1497][10]));let eq13_e1352_d_b11: f64 = ((eq13_e1350_d_b11 * s.v[1497]) + (eq13_e1350 * s.db[1497][11]));let eq13_e1352_d_b12: f64 = ((eq13_e1350_d_b12 * s.v[1497]) + (eq13_e1350 * s.db[1497][12]));let eq13_e1352_d_b13: f64 = ((eq13_e1350_d_b13 * s.v[1497]) + (eq13_e1350 * s.db[1497][13]));let eq13_e1352_d_b14: f64 = ((eq13_e1350_d_b14 * s.v[1497]) + (eq13_e1350 * s.db[1497][14]));let eq13_e1352_d_b15: f64 = ((eq13_e1350_d_b15 * s.v[1497]) + (eq13_e1350 * s.db[1497][15]));let eq13_e1352_d_b16: f64 = ((eq13_e1350_d_b16 * s.v[1497]) + (eq13_e1350 * s.db[1497][16]));let eq13_e1352_d_b17: f64 = ((eq13_e1350_d_b17 * s.v[1497]) + (eq13_e1350 * s.db[1497][17]));let eq13_e1354: f64 = (eq13_e1352 * p.p226);let eq13_e1354_d_n0: f64 = (eq13_e1352_d_n0 * p.p226);let eq13_e1354_d_n1: f64 = (eq13_e1352_d_n1 * p.p226);let eq13_e1354_d_n2: f64 = (eq13_e1352_d_n2 * p.p226);let eq13_e1354_d_n3: f64 = (eq13_e1352_d_n3 * p.p226);let eq13_e1354_d_n4: f64 = (eq13_e1352_d_n4 * p.p226);let eq13_e1354_d_n5: f64 = (eq13_e1352_d_n5 * p.p226);let eq13_e1354_d_n6: f64 = (eq13_e1352_d_n6 * p.p226);let eq13_e1354_d_n7: f64 = (eq13_e1352_d_n7 * p.p226);let eq13_e1354_d_n8: f64 = (eq13_e1352_d_n8 * p.p226);let eq13_e1354_d_n9: f64 = (eq13_e1352_d_n9 * p.p226);let eq13_e1354_d_n10: f64 = (eq13_e1352_d_n10 * p.p226);let eq13_e1354_d_n11: f64 = (eq13_e1352_d_n11 * p.p226);let eq13_e1354_d_n12: f64 = (eq13_e1352_d_n12 * p.p226);let eq13_e1354_d_n13: f64 = (eq13_e1352_d_n13 * p.p226);let eq13_e1354_d_b0: f64 = (eq13_e1352_d_b0 * p.p226);let eq13_e1354_d_b1: f64 = (eq13_e1352_d_b1 * p.p226);let eq13_e1354_d_b2: f64 = (eq13_e1352_d_b2 * p.p226);let eq13_e1354_d_b3: f64 = (eq13_e1352_d_b3 * p.p226);let eq13_e1354_d_b4: f64 = (eq13_e1352_d_b4 * p.p226);let eq13_e1354_d_b5: f64 = (eq13_e1352_d_b5 * p.p226);let eq13_e1354_d_b6: f64 = (eq13_e1352_d_b6 * p.p226);let eq13_e1354_d_b7: f64 = (eq13_e1352_d_b7 * p.p226);let eq13_e1354_d_b8: f64 = (eq13_e1352_d_b8 * p.p226);let eq13_e1354_d_b9: f64 = (eq13_e1352_d_b9 * p.p226);let eq13_e1354_d_b10: f64 = (eq13_e1352_d_b10 * p.p226);let eq13_e1354_d_b11: f64 = (eq13_e1352_d_b11 * p.p226);let eq13_e1354_d_b12: f64 = (eq13_e1352_d_b12 * p.p226);let eq13_e1354_d_b13: f64 = (eq13_e1352_d_b13 * p.p226);let eq13_e1354_d_b14: f64 = (eq13_e1352_d_b14 * p.p226);let eq13_e1354_d_b15: f64 = (eq13_e1352_d_b15 * p.p226);let eq13_e1354_d_b16: f64 = (eq13_e1352_d_b16 * p.p226);let eq13_e1354_d_b17: f64 = (eq13_e1352_d_b17 * p.p226);
        (eq13_e1354, eq13_e1354_d_n0, eq13_e1354_d_n1, eq13_e1354_d_n2, eq13_e1354_d_n3, eq13_e1354_d_n4, eq13_e1354_d_n5, eq13_e1354_d_n6, eq13_e1354_d_n7, eq13_e1354_d_n8, eq13_e1354_d_n9, eq13_e1354_d_n10, eq13_e1354_d_n11, eq13_e1354_d_n12, eq13_e1354_d_n13, eq13_e1354_d_b0, eq13_e1354_d_b1, eq13_e1354_d_b2, eq13_e1354_d_b3, eq13_e1354_d_b4, eq13_e1354_d_b5, eq13_e1354_d_b6, eq13_e1354_d_b7, eq13_e1354_d_b8, eq13_e1354_d_b9, eq13_e1354_d_b10, eq13_e1354_d_b11, eq13_e1354_d_b12, eq13_e1354_d_b13, eq13_e1354_d_b14, eq13_e1354_d_b15, eq13_e1354_d_b16, eq13_e1354_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1356;let eq13_node_derivatives: [f64; 14] = [eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13];let eq13_branch_derivatives: [f64; 18] = [eq13_e1356_d_b0, eq13_e1356_d_b1, eq13_e1356_d_b2, eq13_e1356_d_b3, eq13_e1356_d_b4, eq13_e1356_d_b5, eq13_e1356_d_b6, eq13_e1356_d_b7, eq13_e1356_d_b8, eq13_e1356_d_b9, eq13_e1356_d_b10, eq13_e1356_d_b11, eq13_e1356_d_b12, eq13_e1356_d_b13, eq13_e1356_d_b14, eq13_e1356_d_b15, eq13_e1356_d_b16, eq13_e1356_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);let eq14_e1369: f64 = (eq14_e1367 * s.v[1501]);let eq14_e1371: f64 = (eq14_e1369 * p.p226);let eq14_e1371_d_n0: f64 = ((eq14_e1367 * s.dn[1501][0]) * p.p226);let eq14_e1371_d_n1: f64 = ((eq14_e1367 * s.dn[1501][1]) * p.p226);let eq14_e1371_d_n2: f64 = ((eq14_e1367 * s.dn[1501][2]) * p.p226);let eq14_e1371_d_n3: f64 = ((eq14_e1367 * s.dn[1501][3]) * p.p226);let eq14_e1371_d_n4: f64 = ((eq14_e1367 * s.dn[1501][4]) * p.p226);let eq14_e1371_d_n5: f64 = ((eq14_e1367 * s.dn[1501][5]) * p.p226);let eq14_e1371_d_n6: f64 = ((eq14_e1367 * s.dn[1501][6]) * p.p226);let eq14_e1371_d_n7: f64 = ((eq14_e1367 * s.dn[1501][7]) * p.p226);let eq14_e1371_d_n8: f64 = ((eq14_e1367 * s.dn[1501][8]) * p.p226);let eq14_e1371_d_n9: f64 = ((eq14_e1367 * s.dn[1501][9]) * p.p226);let eq14_e1371_d_n10: f64 = ((eq14_e1367 * s.dn[1501][10]) * p.p226);let eq14_e1371_d_n11: f64 = ((eq14_e1367 * s.dn[1501][11]) * p.p226);let eq14_e1371_d_n12: f64 = ((eq14_e1367 * s.dn[1501][12]) * p.p226);let eq14_e1371_d_n13: f64 = ((eq14_e1367 * s.dn[1501][13]) * p.p226);let eq14_e1371_d_b0: f64 = ((eq14_e1367 * s.db[1501][0]) * p.p226);let eq14_e1371_d_b1: f64 = ((eq14_e1367 * s.db[1501][1]) * p.p226);let eq14_e1371_d_b2: f64 = ((eq14_e1367 * s.db[1501][2]) * p.p226);let eq14_e1371_d_b3: f64 = ((eq14_e1367 * s.db[1501][3]) * p.p226);let eq14_e1371_d_b4: f64 = ((eq14_e1367 * s.db[1501][4]) * p.p226);let eq14_e1371_d_b5: f64 = ((eq14_e1367 * s.db[1501][5]) * p.p226);let eq14_e1371_d_b6: f64 = ((eq14_e1367 * s.db[1501][6]) * p.p226);let eq14_e1371_d_b7: f64 = ((eq14_e1367 * s.db[1501][7]) * p.p226);let eq14_e1371_d_b8: f64 = ((eq14_e1367 * s.db[1501][8]) * p.p226);let eq14_e1371_d_b9: f64 = ((eq14_e1367 * s.db[1501][9]) * p.p226);let eq14_e1371_d_b10: f64 = ((eq14_e1367 * s.db[1501][10]) * p.p226);let eq14_e1371_d_b11: f64 = ((eq14_e1367 * s.db[1501][11]) * p.p226);let eq14_e1371_d_b12: f64 = ((eq14_e1367 * s.db[1501][12]) * p.p226);let eq14_e1371_d_b13: f64 = ((eq14_e1367 * s.db[1501][13]) * p.p226);let eq14_e1371_d_b14: f64 = ((eq14_e1367 * s.db[1501][14]) * p.p226);let eq14_e1371_d_b15: f64 = ((eq14_e1367 * s.db[1501][15]) * p.p226);let eq14_e1371_d_b16: f64 = ((eq14_e1367 * s.db[1501][16]) * p.p226);let eq14_e1371_d_b17: f64 = ((eq14_e1367 * s.db[1501][17]) * p.p226);let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));let eq14_e1373_d_n0: f64 = (eq14_e1371_d_n0 * (nv13 - 0.0));let eq14_e1373_d_n1: f64 = (eq14_e1371_d_n1 * (nv13 - 0.0));let eq14_e1373_d_n2: f64 = (eq14_e1371_d_n2 * (nv13 - 0.0));let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));let eq14_e1373_d_n13: f64 = ((eq14_e1371_d_n13 * (nv13 - 0.0)) + eq14_e1371);let eq14_e1373_d_b0: f64 = (eq14_e1371_d_b0 * (nv13 - 0.0));let eq14_e1373_d_b1: f64 = (eq14_e1371_d_b1 * (nv13 - 0.0));let eq14_e1373_d_b2: f64 = (eq14_e1371_d_b2 * (nv13 - 0.0));let eq14_e1373_d_b3: f64 = (eq14_e1371_d_b3 * (nv13 - 0.0));let eq14_e1373_d_b4: f64 = (eq14_e1371_d_b4 * (nv13 - 0.0));let eq14_e1373_d_b5: f64 = (eq14_e1371_d_b5 * (nv13 - 0.0));let eq14_e1373_d_b6: f64 = (eq14_e1371_d_b6 * (nv13 - 0.0));let eq14_e1373_d_b7: f64 = (eq14_e1371_d_b7 * (nv13 - 0.0));let eq14_e1373_d_b8: f64 = (eq14_e1371_d_b8 * (nv13 - 0.0));let eq14_e1373_d_b9: f64 = (eq14_e1371_d_b9 * (nv13 - 0.0));let eq14_e1373_d_b10: f64 = (eq14_e1371_d_b10 * (nv13 - 0.0));let eq14_e1373_d_b11: f64 = (eq14_e1371_d_b11 * (nv13 - 0.0));let eq14_e1373_d_b12: f64 = (eq14_e1371_d_b12 * (nv13 - 0.0));let eq14_e1373_d_b13: f64 = (eq14_e1371_d_b13 * (nv13 - 0.0));
        let eq14_e1373_d_b14: f64 = (eq14_e1371_d_b14 * (nv13 - 0.0));let eq14_e1373_d_b15: f64 = (eq14_e1371_d_b15 * (nv13 - 0.0));let eq14_e1373_d_b16: f64 = (eq14_e1371_d_b16 * (nv13 - 0.0));let eq14_e1373_d_b17: f64 = (eq14_e1371_d_b17 * (nv13 - 0.0));let eq14_e1374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq14_e1373);
        (eq14_e1374, (eq14_e1373_d_n0 * ddt_scale), (eq14_e1373_d_n1 * ddt_scale), (eq14_e1373_d_n2 * ddt_scale), (eq14_e1373_d_n3 * ddt_scale), (eq14_e1373_d_n4 * ddt_scale), (eq14_e1373_d_n5 * ddt_scale), (eq14_e1373_d_n6 * ddt_scale), (eq14_e1373_d_n7 * ddt_scale), (eq14_e1373_d_n8 * ddt_scale), (eq14_e1373_d_n9 * ddt_scale), (eq14_e1373_d_n10 * ddt_scale), (eq14_e1373_d_n11 * ddt_scale), (eq14_e1373_d_n12 * ddt_scale), (eq14_e1373_d_n13 * ddt_scale), (eq14_e1373_d_b0 * ddt_scale), (eq14_e1373_d_b1 * ddt_scale), (eq14_e1373_d_b2 * ddt_scale), (eq14_e1373_d_b3 * ddt_scale), (eq14_e1373_d_b4 * ddt_scale), (eq14_e1373_d_b5 * ddt_scale), (eq14_e1373_d_b6 * ddt_scale), (eq14_e1373_d_b7 * ddt_scale), (eq14_e1373_d_b8 * ddt_scale), (eq14_e1373_d_b9 * ddt_scale), (eq14_e1373_d_b10 * ddt_scale), (eq14_e1373_d_b11 * ddt_scale), (eq14_e1373_d_b12 * ddt_scale), (eq14_e1373_d_b13 * ddt_scale), (eq14_e1373_d_b14 * ddt_scale), (eq14_e1373_d_b15 * ddt_scale), (eq14_e1373_d_b16 * ddt_scale), (eq14_e1373_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e1376;let eq14_node_derivatives: [f64; 14] = [eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];let eq14_branch_derivatives: [f64; 18] = [eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);let eq15_e1389: f64 = (eq15_e1387 * s.v[1501]);let eq15_e1391: f64 = (eq15_e1389 * p.p226);let eq15_e1391_d_n0: f64 = ((eq15_e1387 * s.dn[1501][0]) * p.p226);let eq15_e1391_d_n1: f64 = ((eq15_e1387 * s.dn[1501][1]) * p.p226);let eq15_e1391_d_n2: f64 = ((eq15_e1387 * s.dn[1501][2]) * p.p226);let eq15_e1391_d_n3: f64 = ((eq15_e1387 * s.dn[1501][3]) * p.p226);let eq15_e1391_d_n4: f64 = ((eq15_e1387 * s.dn[1501][4]) * p.p226);let eq15_e1391_d_n5: f64 = ((eq15_e1387 * s.dn[1501][5]) * p.p226);let eq15_e1391_d_n6: f64 = ((eq15_e1387 * s.dn[1501][6]) * p.p226);let eq15_e1391_d_n7: f64 = ((eq15_e1387 * s.dn[1501][7]) * p.p226);let eq15_e1391_d_n8: f64 = ((eq15_e1387 * s.dn[1501][8]) * p.p226);let eq15_e1391_d_n9: f64 = ((eq15_e1387 * s.dn[1501][9]) * p.p226);let eq15_e1391_d_n10: f64 = ((eq15_e1387 * s.dn[1501][10]) * p.p226);let eq15_e1391_d_n11: f64 = ((eq15_e1387 * s.dn[1501][11]) * p.p226);let eq15_e1391_d_n12: f64 = ((eq15_e1387 * s.dn[1501][12]) * p.p226);let eq15_e1391_d_n13: f64 = ((eq15_e1387 * s.dn[1501][13]) * p.p226);let eq15_e1391_d_b0: f64 = ((eq15_e1387 * s.db[1501][0]) * p.p226);let eq15_e1391_d_b1: f64 = ((eq15_e1387 * s.db[1501][1]) * p.p226);let eq15_e1391_d_b2: f64 = ((eq15_e1387 * s.db[1501][2]) * p.p226);let eq15_e1391_d_b3: f64 = ((eq15_e1387 * s.db[1501][3]) * p.p226);let eq15_e1391_d_b4: f64 = ((eq15_e1387 * s.db[1501][4]) * p.p226);let eq15_e1391_d_b5: f64 = ((eq15_e1387 * s.db[1501][5]) * p.p226);let eq15_e1391_d_b6: f64 = ((eq15_e1387 * s.db[1501][6]) * p.p226);let eq15_e1391_d_b7: f64 = ((eq15_e1387 * s.db[1501][7]) * p.p226);let eq15_e1391_d_b8: f64 = ((eq15_e1387 * s.db[1501][8]) * p.p226);let eq15_e1391_d_b9: f64 = ((eq15_e1387 * s.db[1501][9]) * p.p226);let eq15_e1391_d_b10: f64 = ((eq15_e1387 * s.db[1501][10]) * p.p226);let eq15_e1391_d_b11: f64 = ((eq15_e1387 * s.db[1501][11]) * p.p226);let eq15_e1391_d_b12: f64 = ((eq15_e1387 * s.db[1501][12]) * p.p226);let eq15_e1391_d_b13: f64 = ((eq15_e1387 * s.db[1501][13]) * p.p226);let eq15_e1391_d_b14: f64 = ((eq15_e1387 * s.db[1501][14]) * p.p226);let eq15_e1391_d_b15: f64 = ((eq15_e1387 * s.db[1501][15]) * p.p226);let eq15_e1391_d_b16: f64 = ((eq15_e1387 * s.db[1501][16]) * p.p226);let eq15_e1391_d_b17: f64 = ((eq15_e1387 * s.db[1501][17]) * p.p226);let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));let eq15_e1393_d_n0: f64 = (eq15_e1391_d_n0 * (nv13 - 0.0));let eq15_e1393_d_n1: f64 = (eq15_e1391_d_n1 * (nv13 - 0.0));let eq15_e1393_d_n2: f64 = (eq15_e1391_d_n2 * (nv13 - 0.0));let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));let eq15_e1393_d_n13: f64 = ((eq15_e1391_d_n13 * (nv13 - 0.0)) + eq15_e1391);let eq15_e1393_d_b0: f64 = (eq15_e1391_d_b0 * (nv13 - 0.0));let eq15_e1393_d_b1: f64 = (eq15_e1391_d_b1 * (nv13 - 0.0));let eq15_e1393_d_b2: f64 = (eq15_e1391_d_b2 * (nv13 - 0.0));let eq15_e1393_d_b3: f64 = (eq15_e1391_d_b3 * (nv13 - 0.0));let eq15_e1393_d_b4: f64 = (eq15_e1391_d_b4 * (nv13 - 0.0));let eq15_e1393_d_b5: f64 = (eq15_e1391_d_b5 * (nv13 - 0.0));let eq15_e1393_d_b6: f64 = (eq15_e1391_d_b6 * (nv13 - 0.0));let eq15_e1393_d_b7: f64 = (eq15_e1391_d_b7 * (nv13 - 0.0));let eq15_e1393_d_b8: f64 = (eq15_e1391_d_b8 * (nv13 - 0.0));let eq15_e1393_d_b9: f64 = (eq15_e1391_d_b9 * (nv13 - 0.0));let eq15_e1393_d_b10: f64 = (eq15_e1391_d_b10 * (nv13 - 0.0));let eq15_e1393_d_b11: f64 = (eq15_e1391_d_b11 * (nv13 - 0.0));let eq15_e1393_d_b12: f64 = (eq15_e1391_d_b12 * (nv13 - 0.0));let eq15_e1393_d_b13: f64 = (eq15_e1391_d_b13 * (nv13 - 0.0));
        let eq15_e1393_d_b14: f64 = (eq15_e1391_d_b14 * (nv13 - 0.0));let eq15_e1393_d_b15: f64 = (eq15_e1391_d_b15 * (nv13 - 0.0));let eq15_e1393_d_b16: f64 = (eq15_e1391_d_b16 * (nv13 - 0.0));let eq15_e1393_d_b17: f64 = (eq15_e1391_d_b17 * (nv13 - 0.0));let eq15_e1394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq15_e1393);
        (eq15_e1394, (eq15_e1393_d_n0 * ddt_scale), (eq15_e1393_d_n1 * ddt_scale), (eq15_e1393_d_n2 * ddt_scale), (eq15_e1393_d_n3 * ddt_scale), (eq15_e1393_d_n4 * ddt_scale), (eq15_e1393_d_n5 * ddt_scale), (eq15_e1393_d_n6 * ddt_scale), (eq15_e1393_d_n7 * ddt_scale), (eq15_e1393_d_n8 * ddt_scale), (eq15_e1393_d_n9 * ddt_scale), (eq15_e1393_d_n10 * ddt_scale), (eq15_e1393_d_n11 * ddt_scale), (eq15_e1393_d_n12 * ddt_scale), (eq15_e1393_d_n13 * ddt_scale), (eq15_e1393_d_b0 * ddt_scale), (eq15_e1393_d_b1 * ddt_scale), (eq15_e1393_d_b2 * ddt_scale), (eq15_e1393_d_b3 * ddt_scale), (eq15_e1393_d_b4 * ddt_scale), (eq15_e1393_d_b5 * ddt_scale), (eq15_e1393_d_b6 * ddt_scale), (eq15_e1393_d_b7 * ddt_scale), (eq15_e1393_d_b8 * ddt_scale), (eq15_e1393_d_b9 * ddt_scale), (eq15_e1393_d_b10 * ddt_scale), (eq15_e1393_d_b11 * ddt_scale), (eq15_e1393_d_b12 * ddt_scale), (eq15_e1393_d_b13 * ddt_scale), (eq15_e1393_d_b14 * ddt_scale), (eq15_e1393_d_b15 * ddt_scale), (eq15_e1393_d_b16 * ddt_scale), (eq15_e1393_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1396;let eq15_node_derivatives: [f64; 14] = [eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];let eq15_branch_derivatives: [f64; 18] = [eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1400, eq16_e1400_d_n13,) = {
    if s.b[1514] {
        ((nv13 - 0.0), 1.0,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e1400;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq16_value),
            13,
            multiplicity * (eq16_e1400_d_n13),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq18_e1414, eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13, eq18_e1414_d_b0, eq18_e1414_d_b1, eq18_e1414_d_b2, eq18_e1414_d_b3, eq18_e1414_d_b4, eq18_e1414_d_b5, eq18_e1414_d_b6, eq18_e1414_d_b7, eq18_e1414_d_b8, eq18_e1414_d_b9, eq18_e1414_d_b10, eq18_e1414_d_b11, eq18_e1414_d_b12, eq18_e1414_d_b13, eq18_e1414_d_b14, eq18_e1414_d_b15, eq18_e1414_d_b16, eq18_e1414_d_b17,) = {
    if s.b[1546] {
        let eq18_e1410: f64 = (p.p32 * (nv0 - nv7));let eq18_e1412: f64 = (eq18_e1410 / s.v[1099]);let eq18_e1412_d_n0: f64 = (((p.p32 * s.v[1099]) - (eq18_e1410 * s.dn[1099][0])) / (s.v[1099] * s.v[1099]));let eq18_e1412_d_n1: f64 = (-((eq18_e1410 * s.dn[1099][1]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n2: f64 = (-((eq18_e1410 * s.dn[1099][2]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n3: f64 = (-((eq18_e1410 * s.dn[1099][3]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n4: f64 = (-((eq18_e1410 * s.dn[1099][4]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n5: f64 = (-((eq18_e1410 * s.dn[1099][5]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n6: f64 = (-((eq18_e1410 * s.dn[1099][6]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n7: f64 = ((((-p.p32) * s.v[1099]) - (eq18_e1410 * s.dn[1099][7])) / (s.v[1099] * s.v[1099]));let eq18_e1412_d_n8: f64 = (-((eq18_e1410 * s.dn[1099][8]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n9: f64 = (-((eq18_e1410 * s.dn[1099][9]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n10: f64 = (-((eq18_e1410 * s.dn[1099][10]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n11: f64 = (-((eq18_e1410 * s.dn[1099][11]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n12: f64 = (-((eq18_e1410 * s.dn[1099][12]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_n13: f64 = (-((eq18_e1410 * s.dn[1099][13]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b0: f64 = (-((eq18_e1410 * s.db[1099][0]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b1: f64 = (-((eq18_e1410 * s.db[1099][1]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b2: f64 = (-((eq18_e1410 * s.db[1099][2]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b3: f64 = (-((eq18_e1410 * s.db[1099][3]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b4: f64 = (-((eq18_e1410 * s.db[1099][4]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b5: f64 = (-((eq18_e1410 * s.db[1099][5]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b6: f64 = (-((eq18_e1410 * s.db[1099][6]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b7: f64 = (-((eq18_e1410 * s.db[1099][7]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b8: f64 = (-((eq18_e1410 * s.db[1099][8]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b9: f64 = (-((eq18_e1410 * s.db[1099][9]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b10: f64 = (-((eq18_e1410 * s.db[1099][10]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b11: f64 = (-((eq18_e1410 * s.db[1099][11]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b12: f64 = (-((eq18_e1410 * s.db[1099][12]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b13: f64 = (-((eq18_e1410 * s.db[1099][13]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b14: f64 = (-((eq18_e1410 * s.db[1099][14]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b15: f64 = (-((eq18_e1410 * s.db[1099][15]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b16: f64 = (-((eq18_e1410 * s.db[1099][16]) / (s.v[1099] * s.v[1099])));let eq18_e1412_d_b17: f64 = (-((eq18_e1410 * s.db[1099][17]) / (s.v[1099] * s.v[1099])));
        (eq18_e1412, eq18_e1412_d_n0, eq18_e1412_d_n1, eq18_e1412_d_n2, eq18_e1412_d_n3, eq18_e1412_d_n4, eq18_e1412_d_n5, eq18_e1412_d_n6, eq18_e1412_d_n7, eq18_e1412_d_n8, eq18_e1412_d_n9, eq18_e1412_d_n10, eq18_e1412_d_n11, eq18_e1412_d_n12, eq18_e1412_d_n13, eq18_e1412_d_b0, eq18_e1412_d_b1, eq18_e1412_d_b2, eq18_e1412_d_b3, eq18_e1412_d_b4, eq18_e1412_d_b5, eq18_e1412_d_b6, eq18_e1412_d_b7, eq18_e1412_d_b8, eq18_e1412_d_b9, eq18_e1412_d_b10, eq18_e1412_d_b11, eq18_e1412_d_b12, eq18_e1412_d_b13, eq18_e1412_d_b14, eq18_e1412_d_b15, eq18_e1412_d_b16, eq18_e1412_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1414;let eq18_node_derivatives: [f64; 14] = [eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13];let eq18_branch_derivatives: [f64; 18] = [eq18_e1414_d_b0, eq18_e1414_d_b1, eq18_e1414_d_b2, eq18_e1414_d_b3, eq18_e1414_d_b4, eq18_e1414_d_b5, eq18_e1414_d_b6, eq18_e1414_d_b7, eq18_e1414_d_b8, eq18_e1414_d_b9, eq18_e1414_d_b10, eq18_e1414_d_b11, eq18_e1414_d_b12, eq18_e1414_d_b13, eq18_e1414_d_b14, eq18_e1414_d_b15, eq18_e1414_d_b16, eq18_e1414_d_b17];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1430,) = {
    if (!s.b[1546]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1430;
        stamper.stamp_potential_const_local(
            7,
            eq20_value,
        );
        let (eq21_e1438, eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13, eq21_e1438_d_b0, eq21_e1438_d_b1, eq21_e1438_d_b2, eq21_e1438_d_b3, eq21_e1438_d_b4, eq21_e1438_d_b5, eq21_e1438_d_b6, eq21_e1438_d_b7, eq21_e1438_d_b8, eq21_e1438_d_b9, eq21_e1438_d_b10, eq21_e1438_d_b11, eq21_e1438_d_b12, eq21_e1438_d_b13, eq21_e1438_d_b14, eq21_e1438_d_b15, eq21_e1438_d_b16, eq21_e1438_d_b17,) = {
    if s.b[1547] {
        let eq21_e1434: f64 = (p.p32 * (nv2 - nv8));let eq21_e1436: f64 = (eq21_e1434 / s.v[1100]);let eq21_e1436_d_n0: f64 = (-((eq21_e1434 * s.dn[1100][0]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n1: f64 = (-((eq21_e1434 * s.dn[1100][1]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n2: f64 = (((p.p32 * s.v[1100]) - (eq21_e1434 * s.dn[1100][2])) / (s.v[1100] * s.v[1100]));let eq21_e1436_d_n3: f64 = (-((eq21_e1434 * s.dn[1100][3]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n4: f64 = (-((eq21_e1434 * s.dn[1100][4]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n5: f64 = (-((eq21_e1434 * s.dn[1100][5]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n6: f64 = (-((eq21_e1434 * s.dn[1100][6]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n7: f64 = (-((eq21_e1434 * s.dn[1100][7]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n8: f64 = ((((-p.p32) * s.v[1100]) - (eq21_e1434 * s.dn[1100][8])) / (s.v[1100] * s.v[1100]));let eq21_e1436_d_n9: f64 = (-((eq21_e1434 * s.dn[1100][9]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n10: f64 = (-((eq21_e1434 * s.dn[1100][10]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n11: f64 = (-((eq21_e1434 * s.dn[1100][11]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n12: f64 = (-((eq21_e1434 * s.dn[1100][12]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_n13: f64 = (-((eq21_e1434 * s.dn[1100][13]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b0: f64 = (-((eq21_e1434 * s.db[1100][0]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b1: f64 = (-((eq21_e1434 * s.db[1100][1]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b2: f64 = (-((eq21_e1434 * s.db[1100][2]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b3: f64 = (-((eq21_e1434 * s.db[1100][3]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b4: f64 = (-((eq21_e1434 * s.db[1100][4]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b5: f64 = (-((eq21_e1434 * s.db[1100][5]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b6: f64 = (-((eq21_e1434 * s.db[1100][6]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b7: f64 = (-((eq21_e1434 * s.db[1100][7]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b8: f64 = (-((eq21_e1434 * s.db[1100][8]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b9: f64 = (-((eq21_e1434 * s.db[1100][9]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b10: f64 = (-((eq21_e1434 * s.db[1100][10]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b11: f64 = (-((eq21_e1434 * s.db[1100][11]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b12: f64 = (-((eq21_e1434 * s.db[1100][12]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b13: f64 = (-((eq21_e1434 * s.db[1100][13]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b14: f64 = (-((eq21_e1434 * s.db[1100][14]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b15: f64 = (-((eq21_e1434 * s.db[1100][15]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b16: f64 = (-((eq21_e1434 * s.db[1100][16]) / (s.v[1100] * s.v[1100])));let eq21_e1436_d_b17: f64 = (-((eq21_e1434 * s.db[1100][17]) / (s.v[1100] * s.v[1100])));
        (eq21_e1436, eq21_e1436_d_n0, eq21_e1436_d_n1, eq21_e1436_d_n2, eq21_e1436_d_n3, eq21_e1436_d_n4, eq21_e1436_d_n5, eq21_e1436_d_n6, eq21_e1436_d_n7, eq21_e1436_d_n8, eq21_e1436_d_n9, eq21_e1436_d_n10, eq21_e1436_d_n11, eq21_e1436_d_n12, eq21_e1436_d_n13, eq21_e1436_d_b0, eq21_e1436_d_b1, eq21_e1436_d_b2, eq21_e1436_d_b3, eq21_e1436_d_b4, eq21_e1436_d_b5, eq21_e1436_d_b6, eq21_e1436_d_b7, eq21_e1436_d_b8, eq21_e1436_d_b9, eq21_e1436_d_b10, eq21_e1436_d_b11, eq21_e1436_d_b12, eq21_e1436_d_b13, eq21_e1436_d_b14, eq21_e1436_d_b15, eq21_e1436_d_b16, eq21_e1436_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1438;let eq21_node_derivatives: [f64; 14] = [eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13];let eq21_branch_derivatives: [f64; 18] = [eq21_e1438_d_b0, eq21_e1438_d_b1, eq21_e1438_d_b2, eq21_e1438_d_b3, eq21_e1438_d_b4, eq21_e1438_d_b5, eq21_e1438_d_b6, eq21_e1438_d_b7, eq21_e1438_d_b8, eq21_e1438_d_b9, eq21_e1438_d_b10, eq21_e1438_d_b11, eq21_e1438_d_b12, eq21_e1438_d_b13, eq21_e1438_d_b14, eq21_e1438_d_b15, eq21_e1438_d_b16, eq21_e1438_d_b17];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1454,) = {
    if (!s.b[1547]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1454;
        stamper.stamp_potential_const_local(
            8,
            eq23_value,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq24_e1472, eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13, eq24_e1472_d_b0, eq24_e1472_d_b1, eq24_e1472_d_b2, eq24_e1472_d_b3, eq24_e1472_d_b4, eq24_e1472_d_b5, eq24_e1472_d_b6, eq24_e1472_d_b7, eq24_e1472_d_b8, eq24_e1472_d_b9, eq24_e1472_d_b10, eq24_e1472_d_b11, eq24_e1472_d_b12, eq24_e1472_d_b13, eq24_e1472_d_b14, eq24_e1472_d_b15, eq24_e1472_d_b16, eq24_e1472_d_b17,) = {
    if s.b[1548] {
        let eq24_e1458: f64 = (p.p37 * p.p32);let eq24_e1461: f64 = (s.v[885] + s.v[933]);let eq24_e1461_d_n0: f64 = (s.dn[885][0] + s.dn[933][0]);let eq24_e1461_d_n1: f64 = (s.dn[885][1] + s.dn[933][1]);let eq24_e1461_d_n2: f64 = (s.dn[885][2] + s.dn[933][2]);let eq24_e1461_d_n3: f64 = (s.dn[885][3] + s.dn[933][3]);let eq24_e1461_d_n4: f64 = (s.dn[885][4] + s.dn[933][4]);let eq24_e1461_d_n5: f64 = (s.dn[885][5] + s.dn[933][5]);let eq24_e1461_d_n6: f64 = (s.dn[885][6] + s.dn[933][6]);let eq24_e1461_d_n7: f64 = (s.dn[885][7] + s.dn[933][7]);let eq24_e1461_d_n8: f64 = (s.dn[885][8] + s.dn[933][8]);let eq24_e1461_d_n9: f64 = (s.dn[885][9] + s.dn[933][9]);let eq24_e1461_d_n10: f64 = (s.dn[885][10] + s.dn[933][10]);let eq24_e1461_d_n11: f64 = (s.dn[885][11] + s.dn[933][11]);let eq24_e1461_d_n12: f64 = (s.dn[885][12] + s.dn[933][12]);let eq24_e1461_d_n13: f64 = (s.dn[885][13] + s.dn[933][13]);let eq24_e1461_d_b0: f64 = (s.db[885][0] + s.db[933][0]);let eq24_e1461_d_b1: f64 = (s.db[885][1] + s.db[933][1]);let eq24_e1461_d_b2: f64 = (s.db[885][2] + s.db[933][2]);let eq24_e1461_d_b3: f64 = (s.db[885][3] + s.db[933][3]);let eq24_e1461_d_b4: f64 = (s.db[885][4] + s.db[933][4]);let eq24_e1461_d_b5: f64 = (s.db[885][5] + s.db[933][5]);let eq24_e1461_d_b6: f64 = (s.db[885][6] + s.db[933][6]);let eq24_e1461_d_b7: f64 = (s.db[885][7] + s.db[933][7]);let eq24_e1461_d_b8: f64 = (s.db[885][8] + s.db[933][8]);let eq24_e1461_d_b9: f64 = (s.db[885][9] + s.db[933][9]);let eq24_e1461_d_b10: f64 = (s.db[885][10] + s.db[933][10]);let eq24_e1461_d_b11: f64 = (s.db[885][11] + s.db[933][11]);let eq24_e1461_d_b12: f64 = (s.db[885][12] + s.db[933][12]);let eq24_e1461_d_b13: f64 = (s.db[885][13] + s.db[933][13]);let eq24_e1461_d_b14: f64 = (s.db[885][14] + s.db[933][14]);let eq24_e1461_d_b15: f64 = (s.db[885][15] + s.db[933][15]);let eq24_e1461_d_b16: f64 = (s.db[885][16] + s.db[933][16]);let eq24_e1461_d_b17: f64 = (s.db[885][17] + s.db[933][17]);let eq24_e1462: f64 = (eq24_e1458 * eq24_e1461);let eq24_e1462_d_n0: f64 = (eq24_e1458 * eq24_e1461_d_n0);let eq24_e1462_d_n1: f64 = (eq24_e1458 * eq24_e1461_d_n1);let eq24_e1462_d_n2: f64 = (eq24_e1458 * eq24_e1461_d_n2);let eq24_e1462_d_n3: f64 = (eq24_e1458 * eq24_e1461_d_n3);let eq24_e1462_d_n4: f64 = (eq24_e1458 * eq24_e1461_d_n4);let eq24_e1462_d_n5: f64 = (eq24_e1458 * eq24_e1461_d_n5);let eq24_e1462_d_n6: f64 = (eq24_e1458 * eq24_e1461_d_n6);let eq24_e1462_d_n7: f64 = (eq24_e1458 * eq24_e1461_d_n7);let eq24_e1462_d_n8: f64 = (eq24_e1458 * eq24_e1461_d_n8);let eq24_e1462_d_n9: f64 = (eq24_e1458 * eq24_e1461_d_n9);let eq24_e1462_d_n10: f64 = (eq24_e1458 * eq24_e1461_d_n10);let eq24_e1462_d_n11: f64 = (eq24_e1458 * eq24_e1461_d_n11);let eq24_e1462_d_n12: f64 = (eq24_e1458 * eq24_e1461_d_n12);let eq24_e1462_d_n13: f64 = (eq24_e1458 * eq24_e1461_d_n13);let eq24_e1462_d_b0: f64 = (eq24_e1458 * eq24_e1461_d_b0);let eq24_e1462_d_b1: f64 = (eq24_e1458 * eq24_e1461_d_b1);let eq24_e1462_d_b2: f64 = (eq24_e1458 * eq24_e1461_d_b2);let eq24_e1462_d_b3: f64 = (eq24_e1458 * eq24_e1461_d_b3);let eq24_e1462_d_b4: f64 = (eq24_e1458 * eq24_e1461_d_b4);let eq24_e1462_d_b5: f64 = (eq24_e1458 * eq24_e1461_d_b5);let eq24_e1462_d_b6: f64 = (eq24_e1458 * eq24_e1461_d_b6);let eq24_e1462_d_b7: f64 = (eq24_e1458 * eq24_e1461_d_b7);let eq24_e1462_d_b8: f64 = (eq24_e1458 * eq24_e1461_d_b8);let eq24_e1462_d_b9: f64 = (eq24_e1458 * eq24_e1461_d_b9);let eq24_e1462_d_b10: f64 = (eq24_e1458 * eq24_e1461_d_b10);let eq24_e1462_d_b11: f64 = (eq24_e1458 * eq24_e1461_d_b11);let eq24_e1462_d_b12: f64 = (eq24_e1458 * eq24_e1461_d_b12);let eq24_e1462_d_b13: f64 = (eq24_e1458 * eq24_e1461_d_b13);let eq24_e1462_d_b14: f64 = (eq24_e1458 * eq24_e1461_d_b14);let eq24_e1462_d_b15: f64 = (eq24_e1458 * eq24_e1461_d_b15);let eq24_e1462_d_b16: f64 = (eq24_e1458 * eq24_e1461_d_b16);let eq24_e1462_d_b17: f64 = (eq24_e1458 * eq24_e1461_d_b17);let eq24_e1466: f64 = 0.0;let eq24_e1468: f64 = (eq24_e1466 * (nv7 - nv8));let eq24_e1469: f64 = (p.p32 * eq24_e1468);let eq24_e1469_d_n7: f64 = (p.p32 * eq24_e1466);let eq24_e1469_d_n8: f64 = (p.p32 * (-eq24_e1466));
        let eq24_e1470: f64 = (eq24_e1462 + eq24_e1469);let eq24_e1470_d_n7: f64 = (eq24_e1462_d_n7 + eq24_e1469_d_n7);let eq24_e1470_d_n8: f64 = (eq24_e1462_d_n8 + eq24_e1469_d_n8);
        (eq24_e1470, eq24_e1462_d_n0, eq24_e1462_d_n1, eq24_e1462_d_n2, eq24_e1462_d_n3, eq24_e1462_d_n4, eq24_e1462_d_n5, eq24_e1462_d_n6, eq24_e1470_d_n7, eq24_e1470_d_n8, eq24_e1462_d_n9, eq24_e1462_d_n10, eq24_e1462_d_n11, eq24_e1462_d_n12, eq24_e1462_d_n13, eq24_e1462_d_b0, eq24_e1462_d_b1, eq24_e1462_d_b2, eq24_e1462_d_b3, eq24_e1462_d_b4, eq24_e1462_d_b5, eq24_e1462_d_b6, eq24_e1462_d_b7, eq24_e1462_d_b8, eq24_e1462_d_b9, eq24_e1462_d_b10, eq24_e1462_d_b11, eq24_e1462_d_b12, eq24_e1462_d_b13, eq24_e1462_d_b14, eq24_e1462_d_b15, eq24_e1462_d_b16, eq24_e1462_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1472;let eq24_node_derivatives: [f64; 14] = [eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13];let eq24_branch_derivatives: [f64; 18] = [eq24_e1472_d_b0, eq24_e1472_d_b1, eq24_e1472_d_b2, eq24_e1472_d_b3, eq24_e1472_d_b4, eq24_e1472_d_b5, eq24_e1472_d_b6, eq24_e1472_d_b7, eq24_e1472_d_b8, eq24_e1472_d_b9, eq24_e1472_d_b10, eq24_e1472_d_b11, eq24_e1472_d_b12, eq24_e1472_d_b13, eq24_e1472_d_b14, eq24_e1472_d_b15, eq24_e1472_d_b16, eq24_e1472_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1480, eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13, eq25_e1480_d_b0, eq25_e1480_d_b1, eq25_e1480_d_b2, eq25_e1480_d_b3, eq25_e1480_d_b4, eq25_e1480_d_b5, eq25_e1480_d_b6, eq25_e1480_d_b7, eq25_e1480_d_b8, eq25_e1480_d_b9, eq25_e1480_d_b10, eq25_e1480_d_b11, eq25_e1480_d_b12, eq25_e1480_d_b13, eq25_e1480_d_b14, eq25_e1480_d_b15, eq25_e1480_d_b16, eq25_e1480_d_b17,) = {
    if s.b[1548] {
        let eq25_e1476: f64 = (p.p37 * p.p32);let eq25_e1478: f64 = (eq25_e1476 * s.v[908]);
        (eq25_e1478, (eq25_e1476 * s.dn[908][0]), (eq25_e1476 * s.dn[908][1]), (eq25_e1476 * s.dn[908][2]), (eq25_e1476 * s.dn[908][3]), (eq25_e1476 * s.dn[908][4]), (eq25_e1476 * s.dn[908][5]), (eq25_e1476 * s.dn[908][6]), (eq25_e1476 * s.dn[908][7]), (eq25_e1476 * s.dn[908][8]), (eq25_e1476 * s.dn[908][9]), (eq25_e1476 * s.dn[908][10]), (eq25_e1476 * s.dn[908][11]), (eq25_e1476 * s.dn[908][12]), (eq25_e1476 * s.dn[908][13]), (eq25_e1476 * s.db[908][0]), (eq25_e1476 * s.db[908][1]), (eq25_e1476 * s.db[908][2]), (eq25_e1476 * s.db[908][3]), (eq25_e1476 * s.db[908][4]), (eq25_e1476 * s.db[908][5]), (eq25_e1476 * s.db[908][6]), (eq25_e1476 * s.db[908][7]), (eq25_e1476 * s.db[908][8]), (eq25_e1476 * s.db[908][9]), (eq25_e1476 * s.db[908][10]), (eq25_e1476 * s.db[908][11]), (eq25_e1476 * s.db[908][12]), (eq25_e1476 * s.db[908][13]), (eq25_e1476 * s.db[908][14]), (eq25_e1476 * s.db[908][15]), (eq25_e1476 * s.db[908][16]), (eq25_e1476 * s.db[908][17]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1480;let eq25_node_derivatives: [f64; 14] = [eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13];let eq25_branch_derivatives: [f64; 18] = [eq25_e1480_d_b0, eq25_e1480_d_b1, eq25_e1480_d_b2, eq25_e1480_d_b3, eq25_e1480_d_b4, eq25_e1480_d_b5, eq25_e1480_d_b6, eq25_e1480_d_b7, eq25_e1480_d_b8, eq25_e1480_d_b9, eq25_e1480_d_b10, eq25_e1480_d_b11, eq25_e1480_d_b12, eq25_e1480_d_b13, eq25_e1480_d_b14, eq25_e1480_d_b15, eq25_e1480_d_b16, eq25_e1480_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        let (eq26_e1499, eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13, eq26_e1499_d_b0, eq26_e1499_d_b1, eq26_e1499_d_b2, eq26_e1499_d_b3, eq26_e1499_d_b4, eq26_e1499_d_b5, eq26_e1499_d_b6, eq26_e1499_d_b7, eq26_e1499_d_b8, eq26_e1499_d_b9, eq26_e1499_d_b10, eq26_e1499_d_b11, eq26_e1499_d_b12, eq26_e1499_d_b13, eq26_e1499_d_b14, eq26_e1499_d_b15, eq26_e1499_d_b16, eq26_e1499_d_b17,) = {
    if (!s.b[1548]) {
        let eq26_e1485: f64 = (p.p37 * p.p32);let eq26_e1488: f64 = (s.v[885] - s.v[933]);let eq26_e1488_d_n0: f64 = (s.dn[885][0] - s.dn[933][0]);let eq26_e1488_d_n1: f64 = (s.dn[885][1] - s.dn[933][1]);let eq26_e1488_d_n2: f64 = (s.dn[885][2] - s.dn[933][2]);let eq26_e1488_d_n3: f64 = (s.dn[885][3] - s.dn[933][3]);let eq26_e1488_d_n4: f64 = (s.dn[885][4] - s.dn[933][4]);let eq26_e1488_d_n5: f64 = (s.dn[885][5] - s.dn[933][5]);let eq26_e1488_d_n6: f64 = (s.dn[885][6] - s.dn[933][6]);let eq26_e1488_d_n7: f64 = (s.dn[885][7] - s.dn[933][7]);let eq26_e1488_d_n8: f64 = (s.dn[885][8] - s.dn[933][8]);let eq26_e1488_d_n9: f64 = (s.dn[885][9] - s.dn[933][9]);let eq26_e1488_d_n10: f64 = (s.dn[885][10] - s.dn[933][10]);let eq26_e1488_d_n11: f64 = (s.dn[885][11] - s.dn[933][11]);let eq26_e1488_d_n12: f64 = (s.dn[885][12] - s.dn[933][12]);let eq26_e1488_d_n13: f64 = (s.dn[885][13] - s.dn[933][13]);let eq26_e1488_d_b0: f64 = (s.db[885][0] - s.db[933][0]);let eq26_e1488_d_b1: f64 = (s.db[885][1] - s.db[933][1]);let eq26_e1488_d_b2: f64 = (s.db[885][2] - s.db[933][2]);let eq26_e1488_d_b3: f64 = (s.db[885][3] - s.db[933][3]);let eq26_e1488_d_b4: f64 = (s.db[885][4] - s.db[933][4]);let eq26_e1488_d_b5: f64 = (s.db[885][5] - s.db[933][5]);let eq26_e1488_d_b6: f64 = (s.db[885][6] - s.db[933][6]);let eq26_e1488_d_b7: f64 = (s.db[885][7] - s.db[933][7]);let eq26_e1488_d_b8: f64 = (s.db[885][8] - s.db[933][8]);let eq26_e1488_d_b9: f64 = (s.db[885][9] - s.db[933][9]);let eq26_e1488_d_b10: f64 = (s.db[885][10] - s.db[933][10]);let eq26_e1488_d_b11: f64 = (s.db[885][11] - s.db[933][11]);let eq26_e1488_d_b12: f64 = (s.db[885][12] - s.db[933][12]);let eq26_e1488_d_b13: f64 = (s.db[885][13] - s.db[933][13]);let eq26_e1488_d_b14: f64 = (s.db[885][14] - s.db[933][14]);let eq26_e1488_d_b15: f64 = (s.db[885][15] - s.db[933][15]);let eq26_e1488_d_b16: f64 = (s.db[885][16] - s.db[933][16]);let eq26_e1488_d_b17: f64 = (s.db[885][17] - s.db[933][17]);let eq26_e1489: f64 = (eq26_e1485 * eq26_e1488);let eq26_e1489_d_n0: f64 = (eq26_e1485 * eq26_e1488_d_n0);let eq26_e1489_d_n1: f64 = (eq26_e1485 * eq26_e1488_d_n1);let eq26_e1489_d_n2: f64 = (eq26_e1485 * eq26_e1488_d_n2);let eq26_e1489_d_n3: f64 = (eq26_e1485 * eq26_e1488_d_n3);let eq26_e1489_d_n4: f64 = (eq26_e1485 * eq26_e1488_d_n4);let eq26_e1489_d_n5: f64 = (eq26_e1485 * eq26_e1488_d_n5);let eq26_e1489_d_n6: f64 = (eq26_e1485 * eq26_e1488_d_n6);let eq26_e1489_d_n7: f64 = (eq26_e1485 * eq26_e1488_d_n7);let eq26_e1489_d_n8: f64 = (eq26_e1485 * eq26_e1488_d_n8);let eq26_e1489_d_n9: f64 = (eq26_e1485 * eq26_e1488_d_n9);let eq26_e1489_d_n10: f64 = (eq26_e1485 * eq26_e1488_d_n10);let eq26_e1489_d_n11: f64 = (eq26_e1485 * eq26_e1488_d_n11);let eq26_e1489_d_n12: f64 = (eq26_e1485 * eq26_e1488_d_n12);let eq26_e1489_d_n13: f64 = (eq26_e1485 * eq26_e1488_d_n13);let eq26_e1489_d_b0: f64 = (eq26_e1485 * eq26_e1488_d_b0);let eq26_e1489_d_b1: f64 = (eq26_e1485 * eq26_e1488_d_b1);let eq26_e1489_d_b2: f64 = (eq26_e1485 * eq26_e1488_d_b2);let eq26_e1489_d_b3: f64 = (eq26_e1485 * eq26_e1488_d_b3);let eq26_e1489_d_b4: f64 = (eq26_e1485 * eq26_e1488_d_b4);let eq26_e1489_d_b5: f64 = (eq26_e1485 * eq26_e1488_d_b5);let eq26_e1489_d_b6: f64 = (eq26_e1485 * eq26_e1488_d_b6);let eq26_e1489_d_b7: f64 = (eq26_e1485 * eq26_e1488_d_b7);let eq26_e1489_d_b8: f64 = (eq26_e1485 * eq26_e1488_d_b8);let eq26_e1489_d_b9: f64 = (eq26_e1485 * eq26_e1488_d_b9);let eq26_e1489_d_b10: f64 = (eq26_e1485 * eq26_e1488_d_b10);let eq26_e1489_d_b11: f64 = (eq26_e1485 * eq26_e1488_d_b11);let eq26_e1489_d_b12: f64 = (eq26_e1485 * eq26_e1488_d_b12);let eq26_e1489_d_b13: f64 = (eq26_e1485 * eq26_e1488_d_b13);let eq26_e1489_d_b14: f64 = (eq26_e1485 * eq26_e1488_d_b14);let eq26_e1489_d_b15: f64 = (eq26_e1485 * eq26_e1488_d_b15);let eq26_e1489_d_b16: f64 = (eq26_e1485 * eq26_e1488_d_b16);let eq26_e1489_d_b17: f64 = (eq26_e1485 * eq26_e1488_d_b17);let eq26_e1493: f64 = 0.0;let eq26_e1495: f64 = (eq26_e1493 * (nv8 - nv7));let eq26_e1496: f64 = (p.p32 * eq26_e1495);let eq26_e1496_d_n7: f64 = (p.p32 * (-eq26_e1493));let eq26_e1496_d_n8: f64 = (p.p32 * eq26_e1493);
        let eq26_e1497: f64 = (eq26_e1489 + eq26_e1496);let eq26_e1497_d_n7: f64 = (eq26_e1489_d_n7 + eq26_e1496_d_n7);let eq26_e1497_d_n8: f64 = (eq26_e1489_d_n8 + eq26_e1496_d_n8);
        (eq26_e1497, eq26_e1489_d_n0, eq26_e1489_d_n1, eq26_e1489_d_n2, eq26_e1489_d_n3, eq26_e1489_d_n4, eq26_e1489_d_n5, eq26_e1489_d_n6, eq26_e1497_d_n7, eq26_e1497_d_n8, eq26_e1489_d_n9, eq26_e1489_d_n10, eq26_e1489_d_n11, eq26_e1489_d_n12, eq26_e1489_d_n13, eq26_e1489_d_b0, eq26_e1489_d_b1, eq26_e1489_d_b2, eq26_e1489_d_b3, eq26_e1489_d_b4, eq26_e1489_d_b5, eq26_e1489_d_b6, eq26_e1489_d_b7, eq26_e1489_d_b8, eq26_e1489_d_b9, eq26_e1489_d_b10, eq26_e1489_d_b11, eq26_e1489_d_b12, eq26_e1489_d_b13, eq26_e1489_d_b14, eq26_e1489_d_b15, eq26_e1489_d_b16, eq26_e1489_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1499;let eq26_node_derivatives: [f64; 14] = [eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13];let eq26_branch_derivatives: [f64; 18] = [eq26_e1499_d_b0, eq26_e1499_d_b1, eq26_e1499_d_b2, eq26_e1499_d_b3, eq26_e1499_d_b4, eq26_e1499_d_b5, eq26_e1499_d_b6, eq26_e1499_d_b7, eq26_e1499_d_b8, eq26_e1499_d_b9, eq26_e1499_d_b10, eq26_e1499_d_b11, eq26_e1499_d_b12, eq26_e1499_d_b13, eq26_e1499_d_b14, eq26_e1499_d_b15, eq26_e1499_d_b16, eq26_e1499_d_b17];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1508, eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13, eq27_e1508_d_b0, eq27_e1508_d_b1, eq27_e1508_d_b2, eq27_e1508_d_b3, eq27_e1508_d_b4, eq27_e1508_d_b5, eq27_e1508_d_b6, eq27_e1508_d_b7, eq27_e1508_d_b8, eq27_e1508_d_b9, eq27_e1508_d_b10, eq27_e1508_d_b11, eq27_e1508_d_b12, eq27_e1508_d_b13, eq27_e1508_d_b14, eq27_e1508_d_b15, eq27_e1508_d_b16, eq27_e1508_d_b17,) = {
    if (!s.b[1548]) {
        let eq27_e1504: f64 = (p.p37 * p.p32);let eq27_e1506: f64 = (eq27_e1504 * s.v[908]);
        (eq27_e1506, (eq27_e1504 * s.dn[908][0]), (eq27_e1504 * s.dn[908][1]), (eq27_e1504 * s.dn[908][2]), (eq27_e1504 * s.dn[908][3]), (eq27_e1504 * s.dn[908][4]), (eq27_e1504 * s.dn[908][5]), (eq27_e1504 * s.dn[908][6]), (eq27_e1504 * s.dn[908][7]), (eq27_e1504 * s.dn[908][8]), (eq27_e1504 * s.dn[908][9]), (eq27_e1504 * s.dn[908][10]), (eq27_e1504 * s.dn[908][11]), (eq27_e1504 * s.dn[908][12]), (eq27_e1504 * s.dn[908][13]), (eq27_e1504 * s.db[908][0]), (eq27_e1504 * s.db[908][1]), (eq27_e1504 * s.db[908][2]), (eq27_e1504 * s.db[908][3]), (eq27_e1504 * s.db[908][4]), (eq27_e1504 * s.db[908][5]), (eq27_e1504 * s.db[908][6]), (eq27_e1504 * s.db[908][7]), (eq27_e1504 * s.db[908][8]), (eq27_e1504 * s.db[908][9]), (eq27_e1504 * s.db[908][10]), (eq27_e1504 * s.db[908][11]), (eq27_e1504 * s.db[908][12]), (eq27_e1504 * s.db[908][13]), (eq27_e1504 * s.db[908][14]), (eq27_e1504 * s.db[908][15]), (eq27_e1504 * s.db[908][16]), (eq27_e1504 * s.db[908][17]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1508;let eq27_node_derivatives: [f64; 14] = [eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13];let eq27_branch_derivatives: [f64; 18] = [eq27_e1508_d_b0, eq27_e1508_d_b1, eq27_e1508_d_b2, eq27_e1508_d_b3, eq27_e1508_d_b4, eq27_e1508_d_b5, eq27_e1508_d_b6, eq27_e1508_d_b7, eq27_e1508_d_b8, eq27_e1508_d_b9, eq27_e1508_d_b10, eq27_e1508_d_b11, eq27_e1508_d_b12, eq27_e1508_d_b13, eq27_e1508_d_b14, eq27_e1508_d_b15, eq27_e1508_d_b16, eq27_e1508_d_b17];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );let eq28_e1511: f64 = (p.p32 * s.v[88]);let eq28_value: f64 = eq28_e1511;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq28_value),
            &s.dn[88],
            &s.db[88],
            (multiplicity) * (p.p32),
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv9 = ctx.node_voltage(nodes[9]);let eq29_e1514: f64 = (p.p32 * s.v[89]);let eq29_value: f64 = eq29_e1514;
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq29_value),
            &s.dn[89],
            &s.db[89],
            (multiplicity) * (p.p32),
        );let eq30_e1517: f64 = (p.p37 * p.p32);let eq30_e1519: f64 = (eq30_e1517 * s.v[935]);let eq30_value: f64 = eq30_e1519;
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq30_value),
            &s.dn[935],
            &s.db[935],
            (multiplicity) * (eq30_e1517),
        );let eq31_e1522: f64 = (p.p37 * p.p32);let eq31_e1524: f64 = (eq31_e1522 * s.v[934]);let eq31_value: f64 = eq31_e1524;
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            &s.dn[934],
            &s.db[934],
            (multiplicity) * (eq31_e1522),
        );let eq32_e1528: f64 = (s.v[94] + s.v[90]);let eq32_e1528_d_n0: f64 = (s.dn[94][0] + s.dn[90][0]);let eq32_e1528_d_n1: f64 = (s.dn[94][1] + s.dn[90][1]);let eq32_e1528_d_n2: f64 = (s.dn[94][2] + s.dn[90][2]);let eq32_e1528_d_n3: f64 = (s.dn[94][3] + s.dn[90][3]);let eq32_e1528_d_n4: f64 = (s.dn[94][4] + s.dn[90][4]);let eq32_e1528_d_n5: f64 = (s.dn[94][5] + s.dn[90][5]);let eq32_e1528_d_n6: f64 = (s.dn[94][6] + s.dn[90][6]);let eq32_e1528_d_n7: f64 = (s.dn[94][7] + s.dn[90][7]);let eq32_e1528_d_n8: f64 = (s.dn[94][8] + s.dn[90][8]);let eq32_e1528_d_n9: f64 = (s.dn[94][9] + s.dn[90][9]);let eq32_e1528_d_n10: f64 = (s.dn[94][10] + s.dn[90][10]);let eq32_e1528_d_n11: f64 = (s.dn[94][11] + s.dn[90][11]);let eq32_e1528_d_n12: f64 = (s.dn[94][12] + s.dn[90][12]);let eq32_e1528_d_n13: f64 = (s.dn[94][13] + s.dn[90][13]);let eq32_e1528_d_b0: f64 = (s.db[94][0] + s.db[90][0]);let eq32_e1528_d_b1: f64 = (s.db[94][1] + s.db[90][1]);let eq32_e1528_d_b2: f64 = (s.db[94][2] + s.db[90][2]);let eq32_e1528_d_b3: f64 = (s.db[94][3] + s.db[90][3]);let eq32_e1528_d_b4: f64 = (s.db[94][4] + s.db[90][4]);let eq32_e1528_d_b5: f64 = (s.db[94][5] + s.db[90][5]);let eq32_e1528_d_b6: f64 = (s.db[94][6] + s.db[90][6]);let eq32_e1528_d_b7: f64 = (s.db[94][7] + s.db[90][7]);let eq32_e1528_d_b8: f64 = (s.db[94][8] + s.db[90][8]);let eq32_e1528_d_b9: f64 = (s.db[94][9] + s.db[90][9]);let eq32_e1528_d_b10: f64 = (s.db[94][10] + s.db[90][10]);let eq32_e1528_d_b11: f64 = (s.db[94][11] + s.db[90][11]);let eq32_e1528_d_b12: f64 = (s.db[94][12] + s.db[90][12]);let eq32_e1528_d_b13: f64 = (s.db[94][13] + s.db[90][13]);let eq32_e1528_d_b14: f64 = (s.db[94][14] + s.db[90][14]);let eq32_e1528_d_b15: f64 = (s.db[94][15] + s.db[90][15]);let eq32_e1528_d_b16: f64 = (s.db[94][16] + s.db[90][16]);let eq32_e1528_d_b17: f64 = (s.db[94][17] + s.db[90][17]);let eq32_e1529: f64 = (p.p32 * eq32_e1528);let eq32_e1529_d_n0: f64 = (p.p32 * eq32_e1528_d_n0);let eq32_e1529_d_n1: f64 = (p.p32 * eq32_e1528_d_n1);let eq32_e1529_d_n2: f64 = (p.p32 * eq32_e1528_d_n2);let eq32_e1529_d_n3: f64 = (p.p32 * eq32_e1528_d_n3);let eq32_e1529_d_n4: f64 = (p.p32 * eq32_e1528_d_n4);let eq32_e1529_d_n5: f64 = (p.p32 * eq32_e1528_d_n5);let eq32_e1529_d_n6: f64 = (p.p32 * eq32_e1528_d_n6);let eq32_e1529_d_n7: f64 = (p.p32 * eq32_e1528_d_n7);let eq32_e1529_d_n8: f64 = (p.p32 * eq32_e1528_d_n8);let eq32_e1529_d_n9: f64 = (p.p32 * eq32_e1528_d_n9);let eq32_e1529_d_n10: f64 = (p.p32 * eq32_e1528_d_n10);let eq32_e1529_d_n11: f64 = (p.p32 * eq32_e1528_d_n11);let eq32_e1529_d_n12: f64 = (p.p32 * eq32_e1528_d_n12);let eq32_e1529_d_n13: f64 = (p.p32 * eq32_e1528_d_n13);let eq32_e1529_d_b0: f64 = (p.p32 * eq32_e1528_d_b0);let eq32_e1529_d_b1: f64 = (p.p32 * eq32_e1528_d_b1);let eq32_e1529_d_b2: f64 = (p.p32 * eq32_e1528_d_b2);let eq32_e1529_d_b3: f64 = (p.p32 * eq32_e1528_d_b3);let eq32_e1529_d_b4: f64 = (p.p32 * eq32_e1528_d_b4);let eq32_e1529_d_b5: f64 = (p.p32 * eq32_e1528_d_b5);let eq32_e1529_d_b6: f64 = (p.p32 * eq32_e1528_d_b6);let eq32_e1529_d_b7: f64 = (p.p32 * eq32_e1528_d_b7);let eq32_e1529_d_b8: f64 = (p.p32 * eq32_e1528_d_b8);let eq32_e1529_d_b9: f64 = (p.p32 * eq32_e1528_d_b9);let eq32_e1529_d_b10: f64 = (p.p32 * eq32_e1528_d_b10);let eq32_e1529_d_b11: f64 = (p.p32 * eq32_e1528_d_b11);let eq32_e1529_d_b12: f64 = (p.p32 * eq32_e1528_d_b12);let eq32_e1529_d_b13: f64 = (p.p32 * eq32_e1528_d_b13);let eq32_e1529_d_b14: f64 = (p.p32 * eq32_e1528_d_b14);let eq32_e1529_d_b15: f64 = (p.p32 * eq32_e1528_d_b15);let eq32_e1529_d_b16: f64 = (p.p32 * eq32_e1528_d_b16);let eq32_e1529_d_b17: f64 = (p.p32 * eq32_e1528_d_b17);let eq32_e1533: f64 = 0.0;let eq32_e1535: f64 = (eq32_e1533 * (nv9 - nv7));let eq32_e1536: f64 = (p.p32 * eq32_e1535);let eq32_e1536_d_n7: f64 = (p.p32 * (-eq32_e1533));let eq32_e1536_d_n9: f64 = (p.p32 * eq32_e1533);let eq32_e1537: f64 = (eq32_e1529 + eq32_e1536);let eq32_e1537_d_n7: f64 = (eq32_e1529_d_n7 + eq32_e1536_d_n7);let eq32_e1537_d_n9: f64 = (eq32_e1529_d_n9 + eq32_e1536_d_n9);let eq32_value: f64 = eq32_e1537;
        let eq32_node_derivatives: [f64; 14] = [eq32_e1529_d_n0, eq32_e1529_d_n1, eq32_e1529_d_n2, eq32_e1529_d_n3, eq32_e1529_d_n4, eq32_e1529_d_n5, eq32_e1529_d_n6, eq32_e1537_d_n7, eq32_e1529_d_n8, eq32_e1537_d_n9, eq32_e1529_d_n10, eq32_e1529_d_n11, eq32_e1529_d_n12, eq32_e1529_d_n13];let eq32_branch_derivatives: [f64; 18] = [eq32_e1529_d_b0, eq32_e1529_d_b1, eq32_e1529_d_b2, eq32_e1529_d_b3, eq32_e1529_d_b4, eq32_e1529_d_b5, eq32_e1529_d_b6, eq32_e1529_d_b7, eq32_e1529_d_b8, eq32_e1529_d_b9, eq32_e1529_d_b10, eq32_e1529_d_b11, eq32_e1529_d_b12, eq32_e1529_d_b13, eq32_e1529_d_b14, eq32_e1529_d_b15, eq32_e1529_d_b16, eq32_e1529_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let eq33_e1541: f64 = (s.v[95] + s.v[91]);let eq33_e1541_d_n0: f64 = (s.dn[95][0] + s.dn[91][0]);let eq33_e1541_d_n1: f64 = (s.dn[95][1] + s.dn[91][1]);let eq33_e1541_d_n2: f64 = (s.dn[95][2] + s.dn[91][2]);let eq33_e1541_d_n3: f64 = (s.dn[95][3] + s.dn[91][3]);let eq33_e1541_d_n4: f64 = (s.dn[95][4] + s.dn[91][4]);let eq33_e1541_d_n5: f64 = (s.dn[95][5] + s.dn[91][5]);let eq33_e1541_d_n6: f64 = (s.dn[95][6] + s.dn[91][6]);let eq33_e1541_d_n7: f64 = (s.dn[95][7] + s.dn[91][7]);let eq33_e1541_d_n8: f64 = (s.dn[95][8] + s.dn[91][8]);let eq33_e1541_d_n9: f64 = (s.dn[95][9] + s.dn[91][9]);let eq33_e1541_d_n10: f64 = (s.dn[95][10] + s.dn[91][10]);let eq33_e1541_d_n11: f64 = (s.dn[95][11] + s.dn[91][11]);let eq33_e1541_d_n12: f64 = (s.dn[95][12] + s.dn[91][12]);let eq33_e1541_d_n13: f64 = (s.dn[95][13] + s.dn[91][13]);let eq33_e1541_d_b0: f64 = (s.db[95][0] + s.db[91][0]);let eq33_e1541_d_b1: f64 = (s.db[95][1] + s.db[91][1]);let eq33_e1541_d_b2: f64 = (s.db[95][2] + s.db[91][2]);let eq33_e1541_d_b3: f64 = (s.db[95][3] + s.db[91][3]);let eq33_e1541_d_b4: f64 = (s.db[95][4] + s.db[91][4]);let eq33_e1541_d_b5: f64 = (s.db[95][5] + s.db[91][5]);let eq33_e1541_d_b6: f64 = (s.db[95][6] + s.db[91][6]);let eq33_e1541_d_b7: f64 = (s.db[95][7] + s.db[91][7]);let eq33_e1541_d_b8: f64 = (s.db[95][8] + s.db[91][8]);let eq33_e1541_d_b9: f64 = (s.db[95][9] + s.db[91][9]);let eq33_e1541_d_b10: f64 = (s.db[95][10] + s.db[91][10]);let eq33_e1541_d_b11: f64 = (s.db[95][11] + s.db[91][11]);let eq33_e1541_d_b12: f64 = (s.db[95][12] + s.db[91][12]);let eq33_e1541_d_b13: f64 = (s.db[95][13] + s.db[91][13]);let eq33_e1541_d_b14: f64 = (s.db[95][14] + s.db[91][14]);let eq33_e1541_d_b15: f64 = (s.db[95][15] + s.db[91][15]);let eq33_e1541_d_b16: f64 = (s.db[95][16] + s.db[91][16]);let eq33_e1541_d_b17: f64 = (s.db[95][17] + s.db[91][17]);let eq33_e1542: f64 = (p.p32 * eq33_e1541);let eq33_e1542_d_n0: f64 = (p.p32 * eq33_e1541_d_n0);let eq33_e1542_d_n1: f64 = (p.p32 * eq33_e1541_d_n1);let eq33_e1542_d_n2: f64 = (p.p32 * eq33_e1541_d_n2);let eq33_e1542_d_n3: f64 = (p.p32 * eq33_e1541_d_n3);let eq33_e1542_d_n4: f64 = (p.p32 * eq33_e1541_d_n4);let eq33_e1542_d_n5: f64 = (p.p32 * eq33_e1541_d_n5);let eq33_e1542_d_n6: f64 = (p.p32 * eq33_e1541_d_n6);let eq33_e1542_d_n7: f64 = (p.p32 * eq33_e1541_d_n7);let eq33_e1542_d_n8: f64 = (p.p32 * eq33_e1541_d_n8);let eq33_e1542_d_n9: f64 = (p.p32 * eq33_e1541_d_n9);let eq33_e1542_d_n10: f64 = (p.p32 * eq33_e1541_d_n10);let eq33_e1542_d_n11: f64 = (p.p32 * eq33_e1541_d_n11);let eq33_e1542_d_n12: f64 = (p.p32 * eq33_e1541_d_n12);let eq33_e1542_d_n13: f64 = (p.p32 * eq33_e1541_d_n13);let eq33_e1542_d_b0: f64 = (p.p32 * eq33_e1541_d_b0);let eq33_e1542_d_b1: f64 = (p.p32 * eq33_e1541_d_b1);let eq33_e1542_d_b2: f64 = (p.p32 * eq33_e1541_d_b2);let eq33_e1542_d_b3: f64 = (p.p32 * eq33_e1541_d_b3);let eq33_e1542_d_b4: f64 = (p.p32 * eq33_e1541_d_b4);let eq33_e1542_d_b5: f64 = (p.p32 * eq33_e1541_d_b5);let eq33_e1542_d_b6: f64 = (p.p32 * eq33_e1541_d_b6);let eq33_e1542_d_b7: f64 = (p.p32 * eq33_e1541_d_b7);let eq33_e1542_d_b8: f64 = (p.p32 * eq33_e1541_d_b8);let eq33_e1542_d_b9: f64 = (p.p32 * eq33_e1541_d_b9);let eq33_e1542_d_b10: f64 = (p.p32 * eq33_e1541_d_b10);let eq33_e1542_d_b11: f64 = (p.p32 * eq33_e1541_d_b11);let eq33_e1542_d_b12: f64 = (p.p32 * eq33_e1541_d_b12);let eq33_e1542_d_b13: f64 = (p.p32 * eq33_e1541_d_b13);let eq33_e1542_d_b14: f64 = (p.p32 * eq33_e1541_d_b14);let eq33_e1542_d_b15: f64 = (p.p32 * eq33_e1541_d_b15);let eq33_e1542_d_b16: f64 = (p.p32 * eq33_e1541_d_b16);let eq33_e1542_d_b17: f64 = (p.p32 * eq33_e1541_d_b17);let eq33_e1546: f64 = 0.0;let eq33_e1548: f64 = (eq33_e1546 * (nv9 - nv8));let eq33_e1549: f64 = (p.p32 * eq33_e1548);let eq33_e1549_d_n8: f64 = (p.p32 * (-eq33_e1546));let eq33_e1549_d_n9: f64 = (p.p32 * eq33_e1546);let eq33_e1550: f64 = (eq33_e1542 + eq33_e1549);let eq33_e1550_d_n8: f64 = (eq33_e1542_d_n8 + eq33_e1549_d_n8);let eq33_e1550_d_n9: f64 = (eq33_e1542_d_n9 + eq33_e1549_d_n9);
        let eq33_value: f64 = eq33_e1550;let eq33_node_derivatives: [f64; 14] = [eq33_e1542_d_n0, eq33_e1542_d_n1, eq33_e1542_d_n2, eq33_e1542_d_n3, eq33_e1542_d_n4, eq33_e1542_d_n5, eq33_e1542_d_n6, eq33_e1542_d_n7, eq33_e1550_d_n8, eq33_e1550_d_n9, eq33_e1542_d_n10, eq33_e1542_d_n11, eq33_e1542_d_n12, eq33_e1542_d_n13];let eq33_branch_derivatives: [f64; 18] = [eq33_e1542_d_b0, eq33_e1542_d_b1, eq33_e1542_d_b2, eq33_e1542_d_b3, eq33_e1542_d_b4, eq33_e1542_d_b5, eq33_e1542_d_b6, eq33_e1542_d_b7, eq33_e1542_d_b8, eq33_e1542_d_b9, eq33_e1542_d_b10, eq33_e1542_d_b11, eq33_e1542_d_b12, eq33_e1542_d_b13, eq33_e1542_d_b14, eq33_e1542_d_b15, eq33_e1542_d_b16, eq33_e1542_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );let eq34_e1553: f64 = (p.p32 * s.v[79]);let eq34_value: f64 = eq34_e1553;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq34_value),
            &s.dn[79],
            &s.db[79],
            (multiplicity) * (p.p32),
        );let eq35_e1556: f64 = (p.p32 * s.v[80]);let eq35_value: f64 = eq35_e1556;
        stamper.stamp_current_dense_local(
            Some(9),
            Some(4),
            multiplicity * (eq35_value),
            &s.dn[80],
            &s.db[80],
            (multiplicity) * (p.p32),
        );
        let (eq36_e1560,) = {
    if s.b[1552] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq36_value: f64 = eq36_e1560;
        stamper.stamp_potential_const_local(
            9,
            eq36_value,
        );
    }
}
