#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1380]) && s.b[1385]) {
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_scaled_add(1027, 843, 844, 0.5);
            s.store_offset_mul(843, 183, 1027, 1.0);
            s.store_mul_neg_lhs(844, 184, 817);
            s.store_add_scaled_inputs_product_mixed_aiia(845, A::div_from_scalar(1.0, s.ad_value(843)), 1.0, 844, 1.0, 185, A::sub(s.ad_value(897), s.ad_value(941)), 1.0);
            s.store_add_ad_rhs(846, 845, A::sqrt(A::offset(A::square(s.ad_value(845)), 0.01)));
            s.store_scale(847, 1095, 0.5);
            s.store_add_scaled_inputs3_mixed_aii(1099, A::add_scaled_product(s.ad_value(1097), 1.0, s.ad_value(846), s.ad_value(847), 1.0), 1.0, 61, 1.0, 1102, 1.0);
        }

        s.b[1387] = (s.v[1099] < p.p431);
        s.v[1387] = if s.b[1387] { 1.0 } else { 0.0 };

        if (((!s.b[1380]) && s.b[1385]) && s.b[1387]) {
            s.store_scalar(1099, p.p431);
        }

        if ((!s.b[1380]) && (!s.b[1385])) {
            s.store_scalar(1100, 0.0);
            s.store_scalar(1099, 0.0);
        }

        s.b[1388] = (p.p430 != 0.0);
        s.v[1388] = if s.b[1388] { 1.0 } else { 0.0 };

        if s.b[1388] {
            s.store_scale(1100, 1100, 1.0 / (p.p30));
            s.store_scale(1099, 1099, 1.0 / (p.p30));
        }

        s.store_mul_sub_from_scalar_ad_rhs(844, 875, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0));

        s.b[1389] = (p.p3 != 1.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if s.b[1389] {
            s.store_scale(885, 885, p.p3);
            s.store_scale(933, 933, p.p3);
            s.store_scale(78, 78, p.p3);
            s.store_scale(934, 934, p.p3);
            s.store_scale(935, 935, p.p3);
            s.store_scale(1023, 1023, p.p3);
            s.store_scale(1024, 1024, p.p3);
            s.store_scale(1021, 1021, p.p3);
            s.store_scale(1022, 1022, p.p3);
            s.store_scale(908, 908, p.p3);
            s.store_scale(79, 79, p.p3);
            s.store_scale(905, 905, p.p3);
            s.store_scale(906, 906, p.p3);
        }

        s.store_scalar(83, (A::ddx_projection(&s.ad_value(885), Some(9), None) * p.p37));

        s.b[1390] = (s.v[398] > 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if s.b[1390] {
            s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(7), None) * p.p37));
        }

        if (!s.b[1390]) {
            s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(8), None) * p.p37));
        }

        s.store_scalar(85, (A::ddx_projection(&s.ad_value(885), Some(5), None) * p.p37));

        s.store_scale(842, 396, ((((s.v[332] / p.p23) * p.p3) * s.v[331]) + p.p26));

        s.store_scale(981, 396, (p.p361 * ((((s.v[332] / p.p23) * p.p3) * s.v[365]) + p.p26)));

        s.store_scale(1115, 396, p.p27);

        s.store_scale(1116, 396, (p.p361 * p.p27));

        s.store_sub(830, 825, 1073);

        s.store_mul(853, 1059, 832);

        s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);

        s.store_mul3_lhs(1016, 1059, 363, 832);

        s.store_mul3_lhs(1017, 1059, 364, 832);

        s.b[1391] = (p.p42 == 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        s.b[1392] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (s.b[1391] && s.b[1392]) {
            s.store_mul_ad(810, A::exp(s.ad_value(809)), A::exp(s.ad_value(809)));
            s.store_mul_ad_rhs(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));
        }

        if (s.b[1391] && s.b[1392]) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1393] = (p.p27 > 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {
            s.store_mul_exp_ad_rhs(1117, 810, A::div_scalar_by_product((-p.p1033), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));
        }

        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1394] = (p.p42 == 1.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {
            s.store_exp_ad(810, A::div(s.ad_value(809), A::mul(s.ad_value(384), s.ad_value(363))));
            s.store_mul_ad_rhs(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));
        }

        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1396] = (p.p27 > 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {
            s.store_mul_exp_ad_rhs(1117, 810, A::div_scalar_by_product((-p.p1033), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));
        }

        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1391]) && (!s.b[1394])) {
            s.store_div_scaled_product_right_ad(809, 388, A::sub(s.ad_value(830), s.ad_value(324)), 1.0, 1016, 1.0);
            s.store_div_scaled_inputs2(833, s.ad_value(390), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324))), (-1.0), s.ad_value(1016), 1.0);
        }

        s.b[1397] = (s.v[809] > 100.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1397]) {
            s.store_sub(875, 830, 324);
        }

        s.b[1398] = (s.v[833] > 100.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && s.b[1398]) {
            s.store_div_scaled_inputs3(843, s.ad_value(830), 1.0, s.ad_value(324), (-1.0), s.ad_value(390), -1.0, s.ad_value(1016), 1.0);
            s.store_exp(810, 843);
            s.store_mul_div_scaled_product_rhs(875, 810, s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0);
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_exp(810, 809);
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_mul_ad_rhs(844, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(388)));
            s.store_sub_ad_rhs(845, 388, A::div_scaled_product(s.ad_value(1016), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));
            s.store_div(875, 844, 845);
        }

        s.b[1399] = (p.p27 > 0.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) {
            s.store_div_scaled_product_offset_rhs(1119, s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033), 1.0, s.ad_value(1017), 1.0);
            s.store_div_scaled_inputs2(1120, s.ad_value(390), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033))), (-1.0), s.ad_value(1017), 1.0);
        }

        s.b[1400] = (s.v[1119] > 100.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && s.b[1400]) {
            s.store_offset_sub(1118, 830, 324, (-p.p1033));
        }

        s.b[1401] = (s.v[1120] > 100.0);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && s.b[1401]) {
            s.store_div_scaled_offset_numerator(843, A::add_scaled_inputs3(s.ad_value(830), 1.0, s.ad_value(324), (-1.0), s.ad_value(390), -1.0), 1.0, (-p.p1033), s.ad_value(1017), 1.0);
            s.store_exp(1117, 843);
            s.store_mul_div_scaled_product_rhs(1118, 1117, s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0);
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_exp(1117, 1119);
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_mul_ad_rhs(844, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(1120)), A::sub_from_scalar(1.0, s.ad_value(388)));
            s.store_sub_ad_rhs(845, 388, A::div_scaled_product(s.ad_value(1017), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));
            s.store_div(1118, 844, 845);
        }

        s.copy_ad(829, 1073);

        s.copy_ad(828, 1054);

        s.copy_ad(841, 1044);

        s.b[1402] = (p.p61 == 2.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        s.b[1403] = (s.v[37] == 2.0);
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1403]) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_add_ad_lhs(826, A::add_scaled_inputs_product(s.ad_value(829), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(828), (-1.0)), 324);
            s.store_add_scaled_inputs3_offset_indices(813, 826, 1.0, 825, (-1.0), 841, 1.0, (-0.08));
        }

        s.b[1404] = (s.v[826] <= 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1404]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (-(4.0 * 0.08)));
        }

        if ((s.b[1402] && (!s.b[1403])) && (!s.b[1404])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (4.0 * 0.08));
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_add_scaled_inputs3_indices(812, 826, 1.0, 813, (-0.5), 843, (-0.5));
            s.store_mul_sub_rhs(938, 981, 812, 826);
        }

        s.b[1405] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {
            s.store_offset(1127, 826, p.p1033);
            s.store_scalar(1139, 0.08);
            s.store_add_scaled_inputs4_indices(813, 1127, 1.0, 1125, (-1.0), 841, 1.0, 1139, -1.0);
        }

        s.b[1406] = (s.v[1127] <= 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && s.b[1406]) {
            s.store_sqrt_ad(843, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(1139), s.ad_value(1127), (-100.0)));
        }

        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && (!s.b[1406])) {
            s.store_sqrt_ad(843, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(1139), s.ad_value(1127), 100.0));
        }

        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {
            s.store_add_scaled_inputs3_indices(1128, 1127, 1.0, 813, (-0.5), 843, (-0.5));
            s.store_add_scaled_product_right_ad(938, 938, 1.0, 1116, A::sub(s.ad_value(1128), s.ad_value(1127)), 1.0);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_scale(843, 376, 0.5);
            s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);
        }

        s.b[1407] = (s.v[376] == 0.0);
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1407]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1408] = (s.v[846] < 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && s.b[1408]) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && (!s.b[1408])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_mul_ad_product_rhs(937, 981, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.b[1409] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {
            s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);
        }

        s.b[1410] = (s.v[846] < 0.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && s.b[1410]) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && (!s.b[1410])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {
            s.store_add_ad_rhs(937, 937, A::mul3(s.ad_value(1116), s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843))));
        }

        if s.b[1402] {
            s.store_mul(894, 861, 333);
            s.store_div(891, 875, 894);
            s.store_offset_sub(814, 891, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));
        }

        s.b[1411] = (p.p27 > 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1411]) {
            s.store_div(1129, 1118, 894);
            s.store_offset_sub(814, 1129, 822, (-0.02));
        }

    }

    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1402] && s.b[1411]) {
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 1129, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 843, (-0.5));
        }

        s.b[1412] = (s.v[37] == 2.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1412]) {
            s.store_scalar(1006, 0.0);
        }

        if (s.b[1402] && (!s.b[1412])) {
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 877, 844);
            s.store_mul(846, 843, 845);
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_mul_ad_product_rhs(1006, 981, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(877), 0.5, s.ad_value(846), 1.0));
        }

        s.b[1413] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1412])) && s.b[1413]) {
            s.store_mul(843, 894, 1130);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 1130, 844);
            s.store_mul(846, 843, 845);
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_add_ad_rhs(1006, 1006, A::mul3(s.ad_value(1116), s.ad_value(850), A::sub_scaled_inputs(s.ad_value(1130), 0.5, s.ad_value(846), 1.0)));
        }

        if s.b[1402] {
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 843, 844);
            s.store_mul(846, 843, 845);
            s.store_mul_add_scaled_inputs3_offset_rhs(915, 842, s.ad_value(875), 1.0, s.ad_value(843), (-0.5), s.ad_value(846), 1.0, 0.0);
        }

        s.b[1414] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1414]) {
            s.store_mul(1121, 894, 1130);
            s.store_scaled_offset_ad(855, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);
            s.store_div(845, 1121, 855);
            s.store_mul(846, 1121, 845);
            s.store_add_scaled_product_right_ad(915, 915, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 1.0, s.ad_value(1121), (-0.5), s.ad_value(846), 1.0), 1.0);
        }

        s.b[1415] = (p.p129 > 0.5);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1415]) {
            s.store_scale(844, 844, 2.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 842, s.ad_value(875), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(844), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1416] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((s.b[1402] && s.b[1415]) && s.b[1416]) {
            s.store_scale(855, 855, 2.0);
            s.store_add_scaled_product_right_ad(919, 919, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 0.5, s.ad_value(1121), 0.25, A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 1.0, s.ad_value(855), 1.0), -1.0), (-1.0));
        }

        s.b[1417] = (p.p129 < 0.5);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1415])) && s.b[1417]) {
            s.store_scale(844, 844, 0.08333333333333333);
            s.store_div_scaled_inputs(845, s.ad_value(842), 0.5, A::square(s.ad_value(844)), 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 875, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(875), A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 845, 846);
        }

        s.b[1418] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1415])) && s.b[1417]) && s.b[1418]) {
            s.store_scale(855, 855, 0.08333333333333333);
            s.store_div_scaled_inputs(845, s.ad_value(1115), 0.5, A::square(s.ad_value(855)), 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 1118, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(1118), A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 845, 846);
            s.store_add(919, 919, 1137);
        }

        if ((s.b[1402] && (!s.b[1415])) && (!s.b[1417])) {
            s.store_scaled_add(919, 915, 1006, (-0.5));
        }

        s.b[1419] = (s.v[37] == 2.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

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
            s.store_neg_ad(918, A::add_scaled_inputs4(s.ad_value(916), 1.0, s.ad_value(919), 1.0, s.ad_value(917), 1.0, s.ad_value(920), 1.0));
        }

        s.b[1420] = (p.p61 == 3.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        s.b[1421] = (p.p41 == 0.0);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1421]) {
            s.store_div_from_scalar(997, 3.453133e-11, 62);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1421])) {
            s.store_div_scaled_inputs(997, s.ad_value(416), 8.85418e-12, s.ad_value(62), 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_div_scaled_product_indices(842, 842, 415, 1.0, 62, 1.0);
            s.store_div_scaled_inputs(981, s.ad_value(981), p.p66, s.ad_value(62), 1.0);
            s.store_scale(998, 62, 100000000.0);
        }

        s.b[1422] = (p.p27 > 0.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1422]) {
            s.store_div_scaled_inputs(1115, s.ad_value(1115), p.p66, s.ad_value(62), 1.0);
            s.store_div_scaled_inputs(1116, s.ad_value(1116), p.p66, s.ad_value(62), 1.0);
        }

        s.b[1423] = (s.v[37] == 2.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1423]) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
            s.store_scalar(1015, 0.0);
        }

        s.b[1424] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

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
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

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
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {
            s.store_offset(1126, 1015, p.p1033);
            s.store_add_scaled_inputs3_offset_indices(813, 1126, 1.0, 1125, (-1.0), 841, 1.0, (-0.02));
        }

        s.b[1427] = (s.v[1126] <= 0.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_inputs3(843, s.ad_value(825), 1.0, s.ad_value(841), (-1.0), s.ad_value(1015), -1.0, s.ad_value(998), 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1428] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1428]) {
            s.store_mul_exp_rhs(999, 360, 859);
        }

        s.b[1429] = (s.v[859] <= (-100.0));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && s.b[1429]) {
            s.store_scale(999, 360, 3.720075976e-44);
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_scale(999, 360, 2.688117142e43);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(1000, 62, 0.001);
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 999, (-1.0), 1000, -1.0);
            s.store_sqrt_ad(814, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(1000), s.ad_value(360), 4.0));
            s.store_add_scaled_inputs3_indices(999, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1430] = (s.v[999] < 1e-15);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1430]) {
            s.store_scalar(999, 1e-15);
        }

        s.b[1431] = (p.p27 > 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_div_scaled_inputs3(843, s.ad_value(1125), 1.0, s.ad_value(841), (-1.0), s.ad_value(1126), -1.0, s.ad_value(998), 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1432] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1432]) {
            s.store_mul_exp_rhs(1131, 360, 859);
        }

        s.b[1433] = (s.v[859] <= (-100.0));
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && s.b[1433]) {
            s.store_scale(1131, 360, 3.720075976e-44);
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_scale(1131, 360, 2.688117142e43);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 1131, (-1.0), 1000, -1.0);
            s.store_sqrt_ad(814, A::add_scaled_square_product(s.ad_value(813), 1.0, s.ad_value(1000), s.ad_value(360), 4.0));
            s.store_add_scaled_inputs3_indices(1131, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1434] = (s.v[1131] < 1e-15);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1434]) {
            s.store_scalar(1131, 1e-15);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div(1001, 417, 999);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1001, 1.0);
            s.store_mul(1002, 845, 1001);
        }

        s.b[1435] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1435]) {
            s.store_div(1132, 417, 1131);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1132, 1.0);
            s.store_mul(1133, 845, 1132);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);
        }

        s.b[1436] = (p.p27 > 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1436]) {
            s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_sub_rhs(938, 982, 812, 1015);
        }

        s.b[1437] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1437]) {
            s.store_mul_sub_rhs(1123, 1135, 1128, 1126);
            s.store_add(938, 938, 1123);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(843, 376, 0.5);
            s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);
        }

        s.b[1438] = (s.v[376] == 0.0);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1438]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1439] = (s.v[846] < 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && s.b[1439]) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && (!s.b[1439])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_ad_product_rhs(937, 982, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.b[1440] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);
        }

        s.b[1441] = (s.v[376] == 0.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && s.b[1441]) {
            s.store_scalar(844, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1442] = (s.v[846] < 0.0);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && s.b[1442]) {
            s.store_add_ad_rhs(844, 843, A::div(s.ad_value(846), s.ad_value(376)));
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && (!s.b[1442])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_mul_ad_product_rhs(1124, 1135, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
            s.store_add(937, 937, 1124);
        }

        s.b[1443] = (s.v[376] <= 0.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1443]) {
            s.store_scaled_mul(936, 362, 832, 0.25);
            s.store_scale(843, 339, 0.5);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1443])) {
            s.store_mul_product3_rhs(936, 376, s.ad_value(362), s.ad_value(832), s.ad_value(376), 1.0);
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
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_add_scaled_inputs(844, 843, 2.0, 1118, 1.0);
        }

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
            s.store_div_scaled_inputs2(843, s.ad_value(875), 1.0, s.ad_value(847), 1.0, s.ad_value(998), 1.0);
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
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_add_scaled_inputs3_offset_indices(846, 829, 4.0, 1126, ((-1.0) * 4.0), 942, (-4.0), (p.p1033 * 4.0));
            s.store_sqrt_square_offset(845, 846, 0.0001);
            s.store_scaled_add(847, 846, 845, 0.5);
            s.store_div_scaled_inputs2(843, s.ad_value(1118), 1.0, s.ad_value(847), 1.0, s.ad_value(998), 1.0);
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
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

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
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1447]) {
            s.store_scalar(1006, 0.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) {
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_mul_ad_product_rhs(1006, 982, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(877), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(877), 1.0, s.ad_value(845), 1.0), 1.0));
        }

        s.b[1448] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) && s.b[1448]) {
            s.store_mul_ad_product_rhs(1138, 1135, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(1130), 0.5, A::div_scaled_product(s.ad_value(1121), s.ad_value(1130), 1.0, s.ad_value(1122), 1.0), 1.0));
            s.store_add(1006, 1006, 1138);
        }

        s.b[1449] = (p.p129 > 0.5);
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1449]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 1003, s.ad_value(844), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 0.5, s.ad_value(845), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1450] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && s.b[1449]) && s.b[1450]) {
            s.store_mul_add_scaled_inputs4_rhs(1137, 1134, s.ad_value(1118), ((0.5) * (-1.0)), s.ad_value(1136), (((-0.5)) * (-1.0)), s.ad_value(1121), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 0.5, s.ad_value(1122), 1.0), ((-1.0) * (-1.0)));
            s.store_add(919, 919, 1137);
        }

        s.b[1451] = (p.p129 < 0.5);
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) {
            s.store_scale(845, 845, 0.08333333333333333);
            s.store_div_scaled_inputs(846, s.ad_value(1003), 0.5, A::square(s.ad_value(845)), 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 844, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(844), A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 846, 847);
        }

        s.b[1452] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) && s.b[1452]) {
            s.store_scale(1122, 1122, 0.08333333333333333);
            s.store_div_scaled_inputs(846, s.ad_value(1134), 0.5, A::square(s.ad_value(1122)), 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 855, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(855), A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 846, 847);
            s.store_add(919, 919, 1137);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && (!s.b[1451])) {
            s.store_scale(919, 916, (-0.5));
        }

        s.b[1453] = (s.v[37] == 2.0);
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

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
            s.store_neg_ad(918, A::add_scaled_inputs4(s.ad_value(916), 1.0, s.ad_value(917), 1.0, s.ad_value(920), 1.0, s.ad_value(919), 1.0));
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
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if s.b[1454] {
            s.store_scalar(909, 0.0);
            s.store_scalar(910, 0.0);
        }

        if (!s.b[1454]) {
            s.copy_ad(815, 48);
            s.store_scalar(980, (-p.p363));
            s.store_add_scaled_product_right_ad(815, 815, 1.0, 980, A::sub(s.ad_value(409), s.ad_value(429)), 1.0);
            s.store_scalar(816, p.p183);
            s.store_scalar(976, ((((p.p185 * s.v[350]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(979, 976, p.p362);
            s.store_add_scaled_product_right_ad(976, 976, 1.0, 979, A::sub(s.ad_value(409), s.ad_value(429)), 1.0);
            s.store_scalar(977, ((((p.p186 * s.v[349]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(978, 977, p.p364);
            s.store_add_scaled_product_right_ad(977, 977, 1.0, 978, A::sub(s.ad_value(409), s.ad_value(429)), 1.0);
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
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

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

    }

    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1456] = (s.v[1087] > s.v[994]);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1456]) {
            s.store_add_scaled_product_right_ad(846, 846, 1.0, 858, A::sub(s.ad_value(1087), s.ad_value(994)), 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(910, 987, (p.p351 * p.p3), 976, 846, 1.0);
            s.copy_ad(815, 41);
            s.store_scalar(980, (-p.p365));
            s.store_add_scaled_product_right_ad(815, 815, 1.0, 980, A::sub(s.ad_value(409), s.ad_value(429)), 1.0);
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
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

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

        if (!s.b[1454]) {
            s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.b[1458] = (s.v[1088] > s.v[994]);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1458]) {
            s.store_add_scaled_product_right_ad(846, 846, 1.0, 858, A::sub(s.ad_value(1088), s.ad_value(994)), 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(909, 988, (p.p351 * p.p3), 977, 846, 1.0);
        }

        s.store_scale(853, 897, (-p.p37));

        s.store_scaled_sub(854, 819, 897, p.p37);

        s.b[1459] = (s.v[43] != 0.0);
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        s.b[1460] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        s.b[1461] = (s.v[853] < s.v[322]);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1460]) && s.b[1461]) {
            s.store_scaled_sub(86, 853, 322, s.v[52]);
        }

        s.b[1462] = (s.v[853] < s.v[175]);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1460]) && (!s.b[1461])) && s.b[1462]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(86, 843, s.v[52], A::mul_scaled_lhs(s.ad_value(176), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1463] = (s.v[853] < s.v[323]);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && (!s.b[1463])) {
            s.store_add_scaled_product_indices(86, 56, 1.0, 53, 853, 1.0);
        }

        s.b[1464] = (s.v[853] < s.v[323]);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if ((s.b[1459] && (!s.b[1460])) && s.b[1464]) {
            s.store_mul_sub_rhs(86, 53, 853, 323);
        }

        s.b[1465] = (s.v[853] < s.v[175]);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if (((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && s.b[1465]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(86, 843, s.ad_value(53), 1.0, s.ad_value(176), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1466] = (s.v[853] < s.v[322]);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && s.b[1466]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_add_scaled_inputs3_mixed_iia(86, 853, s.v[52], 56, 1.0, A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && (!s.b[1466])) {
            s.store_add_scaled_inputs(86, 853, s.v[52], 56, 1.0);
        }

        s.b[1467] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        s.b[1468] = (s.v[854] < s.v[322]);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1467]) && s.b[1468]) {
            s.store_scaled_sub(87, 854, 322, s.v[54]);
        }

        s.b[1469] = (s.v[854] < s.v[175]);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1467]) && (!s.b[1468])) && s.b[1469]) {
            s.store_sub(843, 854, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(87, 843, s.v[54], A::mul_scaled_lhs(s.ad_value(178), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1470] = (s.v[854] < s.v[323]);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && s.b[1470]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_add_ad(87, A::add_scaled_product(s.ad_value(57), 1.0, s.ad_value(55), s.ad_value(854), 1.0), A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && (!s.b[1470])) {
            s.store_add_scaled_product_indices(87, 57, 1.0, 55, 854, 1.0);
        }

        s.b[1471] = (s.v[854] < s.v[323]);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((s.b[1459] && (!s.b[1467])) && s.b[1471]) {
            s.store_mul_sub_rhs(87, 55, 854, 323);
        }

        s.b[1472] = (s.v[854] < s.v[175]);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if (((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(87, 843, s.ad_value(55), 1.0, s.ad_value(178), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1473] = (s.v[854] < s.v[322]);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

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
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

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
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if s.b[1475] {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1475]) {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1476] = (p.p39 == 3.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

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
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if s.b[1477] {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1477]) {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1478] = (p.p3 != 1.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if s.b[1478] {
            s.store_scale(895, 895, p.p3);
            s.store_scale(896, 896, p.p3);
        }

        s.b[1505] = (p.p223 == 0.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = (p.p223 == 1.0);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        s.b[1507] = (p.p223 == 2.0);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        s.b[1508] = (p.p223 == 3.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (s.b[1506] && (!s.b[1505])) {
            s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);
            s.store_square(843, 843);
            s.store_div_scaled_inputs(1486, s.ad_value(946), 2.0, s.ad_value(75), 1.0);
            s.store_div_scaled_inputs(848, s.ad_value(72), 1.0, s.ad_value(1486), s.v[327]);
            s.store_square(848, 848);
            s.store_offset_scaled(1487, 848, (((p.p227 * s.v[327])) * (p.p229)), p.p229);
            s.store_offset_scaled(1488, 848, (((p.p228 * s.v[327])) * (p.p230)), p.p230);
        }

        s.b[1509] = (s.v[1488] > 0.9);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if ((s.b[1506] && (!s.b[1505])) && s.b[1509]) {
            s.store_scalar(1488, 0.9);
        }

        s.b[1510] = (s.v[1488] > (0.9 * s.v[1487]));
        s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };

        if ((s.b[1506] && (!s.b[1505])) && s.b[1510]) {
            s.store_scale(1488, 1487, 0.9);
        }

        if (s.b[1506] && (!s.b[1505])) {
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
            s.store_mul_add_scaled_inputs_rhs(1492, 849, s.ad_value(844), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(845), 6.0), 1.0);
            s.store_square(846, 845);
            s.store_square(847, 843);
            s.store_square(848, 846);
            s.store_div_scaled_inputs3(1493, A::div(s.ad_value(844), s.ad_value(846)), 1.0, A::div_scaled_product(A::add_scaled_inputs(s.ad_value(844), 5.0, s.ad_value(845), 1.0), s.ad_value(847), 1.0, s.ad_value(848), 15.0), (-1.0), A::div_scaled_product_by_product(s.ad_value(847), s.ad_value(847), 1.0, s.ad_value(848), s.ad_value(845), 9.0), 1.0, A::mul3_scaled_output(s.ad_value(849), s.ad_value(849), s.ad_value(849), 6.0), 1.0);
            s.store_div(850, 843, 845);
            s.store_div_scaled_add_product(1494, s.ad_value(850), 1.0, A::square(s.ad_value(850)), s.ad_value(850), 0.3333333333333333, s.ad_value(849), 6.0);
            s.store_div(851, 72, 838);
            s.store_square(851, 851);
            s.store_offset_scaled(1490, 851, (((p.p224 * s.v[892])) * (p.p225)), p.p225);
            s.store_mul_scaled_ad_lhs(1498, A::div(s.ad_value(1494), A::sqrt(A::mul(s.ad_value(1492), s.ad_value(1493)))), 1490, 2.5316);
        }

        s.b[1512] = (s.v[1498] > 1.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1512]) {
            s.store_scalar(1498, 1.0);
        }

        s.b[1513] = (s.v[1498] < 0.0);
        s.v[1513] = if s.b[1513] { 1.0 } else { 0.0 };

        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1513]) {
            s.store_scalar(1498, 0.0);
        }

        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
            s.store_offset_scaled(1487, 851, (((p.p227 * s.v[892])) * (p.p229)), p.p229);
            s.store_offset_scaled(1488, 851, (((p.p228 * s.v[892])) * (p.p230)), p.p230);
            s.store_mul3_affine_lhs(1492, 1492, 1487, 3.0, 0.0, 1487);
            s.store_mul3_affine_lhs(1493, 1493, 1488, 3.75, 0.0, 1488);
            s.store_div_scaled_product_offset_denominator(1499, s.ad_value(880), s.ad_value(72), p.p3, A::mul(s.ad_value(881), s.ad_value(887)), 1.0, 1.0);
            s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));
            s.store_div_scaled_offset_numerator(1497, s.ad_value(1499), 1.0, 1e-15, A::sqrt(A::div(s.ad_value(1493), s.ad_value(1492))), 1.0);
        }

        s.b[1546] = ((p.p429 != 2.0) && ((s.v[61] + p.p136) >= p.p431));
        s.v[1546] = if s.b[1546] { 1.0 } else { 0.0 };

        s.b[1547] = ((p.p429 != 2.0) && ((s.v[60] + p.p135) >= p.p431));
        s.v[1547] = if s.b[1547] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1548] = (s.v[398] > 0.0);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        s.b[1549] = (p.p430 != 0.0);
        s.v[1549] = if s.b[1549] { 1.0 } else { 0.0 };

        if (s.b[1548] && s.b[1549]) {
            s.store_scale(88, 905, (p.p37 * p.p30));
            s.store_scale(89, 906, (p.p37 * p.p30));
            s.store_scale(90, 1024, (p.p37 * p.p30));
            s.store_scale(91, 1023, (p.p37 * p.p30));
        }

        if (s.b[1548] && (!s.b[1549])) {
            s.store_scale(88, 905, p.p37);
            s.store_scale(89, 906, p.p37);
            s.store_scale(90, 1024, p.p37);
            s.store_scale(91, 1023, p.p37);
        }

        if s.b[1548] {
            s.store_scale(92, 918, p.p37);
            s.store_scale(93, 919, p.p37);
        }

        s.b[1550] = (p.p430 != 0.0);
        s.v[1550] = if s.b[1550] { 1.0 } else { 0.0 };

        if ((!s.b[1548]) && s.b[1550]) {
            s.store_scale(89, 905, (p.p37 * p.p30));
            s.store_scale(88, 906, (p.p37 * p.p30));
            s.store_scale(91, 1024, (p.p37 * p.p30));
            s.store_scale(90, 1023, (p.p37 * p.p30));
        }

        if ((!s.b[1548]) && (!s.b[1550])) {
            s.store_scale(89, 905, p.p37);
            s.store_scale(88, 906, p.p37);
            s.store_scale(91, 1024, p.p37);
            s.store_scale(90, 1023, p.p37);
        }

        if (!s.b[1548]) {
            s.store_scale(93, 918, p.p37);
            s.store_scale(92, 919, p.p37);
        }

        s.b[1551] = (p.p430 != 0.0);
        s.v[1551] = if s.b[1551] { 1.0 } else { 0.0 };

        if s.b[1551] {
            s.store_scale(94, 1022, (p.p37 * p.p30));
            s.store_scale(95, 1021, (p.p37 * p.p30));
        }

        if (!s.b[1551]) {
            s.store_scale(94, 1022, p.p37);
            s.store_scale(95, 1021, p.p37);
        }

        s.b[1552] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));
        s.v[1552] = if s.b[1552] { 1.0 } else { 0.0 };

        s.b[1553] = (p.p39 == 3.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        s.b[1555] = ((p.p39 == 0.0) || (p.p39 == 1.0));
        s.v[1555] = if s.b[1555] { 1.0 } else { 0.0 };

        s.b[1559] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        s.b[1560] = ((p.p35 != 0.0) && (!true));
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        s.b[1561] = true;
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        s.b[1562] = true;
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        s.b[1563] = (p.p430 == 2.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        s.b[1564] = (p.p430 == 2.0);
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        s.copy_ad(426, 916);

        s.copy_ad(427, 918);

        s.copy_ad(428, 919);

        s.store_add(425, 896, 895);

        s.store_sub(918, 427, 895);

        s.store_sub(919, 428, 896);

        s.store_add(916, 426, 425);

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[409] = (ctx_temp + p.p0);

        s.v[429] = (p.p126 + 273.15);

        s.v[36] = p.p336;

        s.v[37] = p.p21;

        s.v[38] = p.p348;

        s.v[39] = p.p213;

        s.v[40] = p.p127;

        s.v[41] = p.p182;

        s.v[42] = p.p350;

        s.v[43] = p.p355;

        s.v[44] = p.p234;

        s.v[45] = p.p236;

        s.v[46] = p.p373;

        s.v[48] = p.p181;

        if (p.p41 != 0.0) {
            s.store_scalar(416, 3.9);
            s.store_scalar(415, p.p45);
            s.store_scalar(417, (8.85418e-12 * p.p47));
            s.store_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));
            s.store_div_scaled_inputs(396, s.ad_value(416), 8.85418e-12, s.ad_value(415), 1.0);
        }

        if (p.p41 == 0.0) {
            s.store_scalar(416, p.p46);
            s.store_scalar(415, p.p66);
            s.store_scalar(417, 1.03594e-10);
            s.store_scalar(419, 5.753e-12);
            s.store_scalar(396, (3.453133e-11 / p.p66));
        }

        s.b[431] = (s.v[37] == 2.0);
        s.v[431] = if s.b[431] { 1.0 } else { 0.0 };

        if s.b[431] {
            s.store_scalar(399, 0.0);
        }

        s.b[456] = (!true);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if ((!s.b[431]) && s.b[456]) {
            s.store_scalar(399, 0.0);
        }

        s.b[458] = (!true);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        s.b[459] = ((s.v[38] == 0.0) && (p.p349 == 0.0));
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && s.b[459]) {
            s.store_scalar(399, 2.0);
        }

        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && (!s.b[459])) {
            s.store_scalar(399, 1.0);
        }

        s.b[460] = ((s.v[38] == 0.0) && (p.p349 == 0.0));
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {
            s.store_scalar(38, 1.0);
            s.store_scalar(399, 1.0);
        }

        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && (!s.b[460])) {
            s.store_scalar(399, 1.0);
        }

        s.b[461] = param_given[213];
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if s.b[461] {
            s.store_scalar(39, p.p213);
        }

        if (!s.b[461]) {
            s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p.p66))) as f64).ln()));
        }

        s.b[533] = (s.v[48] < 0.1);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if s.b[533] {
            s.store_scalar(48, 0.1);
        }

        s.b[534] = (s.v[41] < 0.1);
        s.v[534] = if s.b[534] { 1.0 } else { 0.0 };

        if s.b[534] {
            s.store_scalar(41, 0.1);
        }

        s.v[429] = (p.p126 + 273.15);

        s.v[476] = (s.v[409] / s.v[429]);

        if (p.p41 != 0.0) {
            s.store_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));
        }

        if (p.p41 == 0.0) {
            s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p.p66)) as f64).sqrt());
        }

        s.b[535] = (p.p41 == 0.0);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if s.b[535] {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
            s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));
            s.copy_ad(394, 466);
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
            s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));
            s.copy_ad(395, 465);
        }

        if s.b[535] {
            s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));
        }

        if (!s.b[535]) {
            s.store_scalar(480, (8.617087e-5 * s.v[429]));
            s.store_scalar(466, (p.p49 - (((p.p50 * s.v[429]) * s.v[429]) / (s.v[429] + p.p51))));
            s.copy_ad(394, 466);
            s.store_scalar(49, (8.617087e-5 * s.v[409]));
            s.store_scalar(465, (p.p49 - (((p.p50 * s.v[409]) * s.v[409]) / (s.v[409] + p.p51))));
            s.copy_ad(395, 465);
        }

        if (!s.b[535]) {
            s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));
        }

        s.v[50] = (p.p16 * p.p349);

        s.v[474] = p.p1;

        s.v[475] = (p.p2 / p.p3);

        s.v[467] = ((s.v[474]) as f64).powf(p.p190);

        s.v[468] = ((s.v[475]) as f64).powf(p.p193);

        s.v[463] = (((p.p188 / s.v[467]) + (p.p191 / s.v[468])) + (p.p194 / (s.v[467] * s.v[468])));

        s.v[326] = (p.p187 + s.v[463]);

        s.v[463] = (((p.p189 / s.v[467]) + (p.p192 / s.v[468])) + (p.p195 / (s.v[467] * s.v[468])));

        s.v[330] = (p.p217 + s.v[463]);

        s.v[215] = (p.p410 + s.v[463]);

        s.b[536] = (s.v[215] < 0.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        if s.b[536] {
            s.store_scalar(215, 0.0);
        }

        s.v[469] = ((s.v[474]) as f64).powf(p.p202);

        s.v[470] = ((s.v[475]) as f64).powf(p.p205);

        s.v[464] = (((p.p200 / s.v[469]) + (p.p203 / s.v[470])) + (p.p206 / (s.v[469] * s.v[470])));

        s.v[325] = (p.p197 + s.v[464]);

        s.v[464] = (((p.p201 / s.v[469]) + (p.p204 / s.v[470])) + (p.p207 / (s.v[469] * s.v[470])));

        s.v[329] = (p.p216 + s.v[464]);

        s.v[327] = (p.p1 - (2.0 * s.v[326]));

        s.v[328] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[325]));

        s.v[348] = ((s.v[328] / p.p23) + p.p24);

        s.v[347] = ((s.v[328] / p.p23) + p.p25);

        s.v[331] = (p.p1 - (2.0 * s.v[330]));

        s.v[332] = (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[329]));

        s.v[349] = ((s.v[332] / p.p23) + p.p24);

        s.v[350] = ((s.v[332] / p.p23) + p.p25);

        s.v[365] = ((p.p1 - (2.0 * s.v[330])) - p.p360);

        s.v[366] = (s.v[365] + (2.0 * p.p372));

        s.v[112] = p.p85;

        s.v[113] = p.p86;

        s.v[114] = p.p87;

        s.v[116] = p.p88;

        s.v[117] = p.p89;

        s.copy_ad(239, 39);

        s.v[240] = p.p214;

        s.v[241] = p.p215;

        s.b[543] = (s.v[241] == 0.0);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_scalar(333, 2.0);
        }

        if (!s.b[543]) {
            s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));
        }

        s.b[544] = (p.p65 == 1.0);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_scalar(477, (1e-6 / s.v[327]));
            s.store_scalar(478, (1e-6 / s.v[328]));
            s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));
        }

        if (!s.b[544]) {
            s.store_scalar(477, (1.0 / s.v[327]));
            s.store_scalar(478, (1.0 / s.v[328]));
            s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));
        }

        s.store_add_scaled_inputs3_offset_indices(108, 477, p.p488, 478, p.p678, 479, p.p868, p.p82);

        s.store_add_scaled_inputs3_offset_indices(109, 477, p.p489, 478, p.p679, 479, p.p869, p.p81);

        s.store_add_scaled_inputs3_offset_indices(110, 477, p.p490, 478, p.p680, 479, p.p871, p.p83);

        s.store_add_scaled_inputs3_offset_indices(111, 477, p.p491, 478, p.p681, 479, p.p870, p.p84);

        s.store_add_scaled_inputs3_offset_indices(137, 477, p.p492, 478, p.p682, 479, p.p872, p.p108);

        s.store_add_scaled_inputs3_offset_indices(152, 477, p.p493, 478, p.p683, 479, p.p873, p.p109);

        s.store_add_scaled_inputs3_offset_indices(120, 477, p.p494, 478, p.p684, 479, p.p874, p.p90);

        s.store_add_scaled_inputs3_offset_indices(124, 477, p.p497, 478, p.p687, 479, p.p877, p.p94);

        s.store_add_scaled_inputs3_offset_indices(264, 477, p.p495, 478, p.p685, 479, p.p875, p.p300);

        s.store_add_scaled_inputs3_offset_indices(265, 477, p.p496, 478, p.p686, 479, p.p876, p.p301);

        s.store_add_scaled_inputs3_offset_indices(125, 477, p.p498, 478, p.p688, 479, p.p878, p.p95);

        s.store_add_scaled_inputs3_offset_indices(126, 477, p.p499, 478, p.p689, 479, p.p879, p.p96);

        s.store_add_scaled_inputs3_offset_indices(263, 477, p.p500, 478, p.p690, 479, p.p880, p.p371);

        s.store_add_scaled_inputs3_offset_indices(127, 477, p.p501, 478, p.p691, 479, p.p881, p.p97);

        s.store_add_scaled_inputs3_offset_indices(128, 477, p.p1024, 478, p.p1027, 479, p.p1030, p.p1021);

        s.store_add_scaled_inputs3_offset_indices(377, 477, p.p502, 478, p.p692, 479, p.p882, p.p98);

        s.store_add_scaled_inputs3_offset_indices(129, 477, p.p503, 478, p.p693, 479, p.p883, p.p99);

        s.store_add_scaled_inputs3_offset_indices(130, 477, p.p504, 478, p.p694, 479, p.p884, p.p100);

        s.store_add_scaled_inputs3_offset_indices(131, 477, p.p505, 478, p.p695, 479, p.p885, p.p101);

        s.store_add_scaled_inputs3_offset_indices(132, 477, p.p506, 478, p.p696, 479, p.p886, p.p102);

        s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);

        s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);

        s.store_add_scaled_inputs3_offset_indices(134, 477, p.p508, 478, p.p698, 479, p.p888, p.p104);

        s.store_add_scaled_inputs3_offset_indices(144, 477, p.p509, 478, p.p699, 479, p.p889, p.p116);

        s.store_add_scaled_inputs3_offset_indices(138, 477, p.p511, 478, p.p701, 479, p.p891, p.p110);

        s.store_add_scaled_inputs3_offset_indices(140, 477, p.p512, 478, p.p702, 479, p.p892, p.p112);

        s.store_add_scaled_inputs3_offset_indices(142, 477, p.p513, 478, p.p703, 479, p.p893, p.p114);

        s.store_add_scaled_inputs3_offset_indices(101, 477, p.p518, 478, p.p708, 479, p.p898, p.p74);

        s.store_add_scaled_inputs3_offset_indices(103, 477, p.p519, 478, p.p709, 479, p.p899, p.p76);

        s.store_add_scaled_inputs3_offset_indices(104, 477, p.p520, 478, p.p710, 479, p.p900, p.p77);

        s.store_add_scaled_inputs3_offset_indices(199, 477, p.p521, 478, p.p711, 479, p.p901, p.p208);

        s.store_add_scaled_inputs3_offset_indices(200, 477, p.p522, 478, p.p712, 479, p.p902, p.p209);

        s.store_add_scaled_inputs3_offset_indices(107, 477, p.p523, 478, p.p713, 479, p.p903, p.p80);

        s.store_add_scaled_inputs3_offset_indices(266, 477, p.p524, 478, p.p714, 479, p.p904, p.p302);

        s.store_add_scaled_inputs3_offset_indices(105, 477, p.p525, 478, p.p715, 479, p.p905, p.p78);

        s.store_add_scaled_inputs3_offset_indices(106, 477, p.p526, 478, p.p716, 479, p.p906, p.p79);

        s.store_add_scaled_inputs3_offset_indices(181, 477, p.p527, 478, p.p717, 479, p.p907, p.p132);

        s.store_add_scaled_inputs3_offset_indices(170, 477, p.p528, 478, p.p718, 479, p.p908, p.p133);

        s.store_add_scaled_inputs3_offset_indices(169, 477, p.p529, 478, p.p719, 479, p.p909, p.p134);

        s.store_add_scaled_inputs3_offset_indices(184, 477, p.p530, 478, p.p720, 479, p.p910, p.p142);

        s.store_add_scaled_inputs3_offset_indices(185, 477, p.p531, 478, p.p721, 479, p.p911, p.p143);

        s.store_add_scaled_inputs3_offset_indices(183, 477, p.p532, 478, p.p722, 479, p.p912, p.p141);

        s.store_add_scaled_inputs3_offset_indices(196, 477, p.p533, 478, p.p723, 479, p.p913, p.p196);

        s.store_add_scaled_inputs3_offset_indices(100, 477, p.p534, 478, p.p724, 479, p.p914, p.p73);

        s.store_add_scaled_inputs3_offset_indices(197, 477, p.p535, 478, p.p725, 479, p.p915, p.p198);

        s.store_add_scaled_inputs3_offset_indices(198, 477, p.p536, 478, p.p726, 479, p.p916, p.p199);

        s.store_add_scaled_inputs3_offset_indices(151, 477, p.p537, 478, p.p727, 479, p.p917, p.p125);

        s.store_add_scaled_inputs3_offset_indices(187, 477, p.p538, 478, p.p728, 479, p.p918, p.p145);

        s.store_add_scaled_inputs3_offset_indices(188, 477, p.p539, 478, p.p729, 479, p.p919, p.p146);

        s.store_add_scaled_inputs3_offset_indices(189, 477, p.p540, 478, p.p730, 479, p.p920, p.p147);

        s.store_add_scaled_inputs3_offset_indices(190, 477, p.p541, 478, p.p731, 479, p.p921, p.p148);

        s.store_add_scaled_inputs3_offset_indices(136, 477, p.p542, 478, p.p732, 479, p.p922, p.p106);

        s.store_add_scaled_inputs3_offset_indices(99, 477, p.p543, 478, p.p733, 479, p.p923, p.p72);

        s.store_add_scaled_inputs3_offset_indices(96, 477, p.p544, 478, p.p734, 479, p.p924, p.p69);

        s.store_add_scaled_inputs3_offset_indices(97, 477, p.p545, 478, p.p735, 479, p.p925, p.p70);

        s.store_add_scaled_inputs3_offset_indices(98, 477, p.p546, 478, p.p736, 479, p.p926, p.p71);

        s.store_add_scaled_inputs3_offset_indices(191, 477, p.p547, 478, p.p737, 479, p.p927, p.p149);

        s.store_add_scaled_inputs3_offset_indices(192, 477, p.p548, 478, p.p738, 479, p.p928, p.p150);

        s.store_add_scaled_inputs3_offset_indices(193, 477, p.p549, 478, p.p739, 479, p.p929, p.p151);

        s.store_add_scaled_inputs3_offset_indices(194, 477, p.p550, 478, p.p740, 479, p.p930, p.p152);

        s.store_add_scaled_inputs3_offset_indices(135, 477, p.p551, 478, p.p741, 479, p.p931, p.p105);

        s.store_add_scaled_inputs3_offset_indices(195, 477, p.p552, 478, p.p742, 479, p.p932, p.p153);

        s.store_add_scaled_inputs3_offset_indices(180, 477, p.p553, 478, p.p743, 479, p.p933, p.p130);

        s.store_add_scaled_inputs3_offset_indices(201, 477, p.p554, 478, p.p744, 479, p.p934, p.p218);

        s.store_add_scaled_inputs3_offset_indices(267, 477, p.p555, 478, p.p745, 479, p.p935, p.p314);

        s.store_add_scaled_inputs3_offset_indices(268, 477, p.p558, 478, p.p748, 479, p.p938, p.p315);

        s.store_add_scaled_inputs3_offset_indices(269, 477, p.p557, 478, p.p747, 479, p.p937, p.p316);

        s.store_add_scaled_inputs3_offset_indices(270, 477, p.p560, 478, p.p750, 479, p.p940, p.p317);

        s.store_add_scaled_inputs3_offset_indices(271, 477, p.p556, 478, p.p746, 479, p.p936, p.p318);

        s.store_add_scaled_inputs3_offset_indices(272, 477, p.p559, 478, p.p749, 479, p.p939, p.p319);

        s.store_add_scaled_inputs3_offset_indices(202, 477, p.p561, 478, p.p751, 479, p.p941, p.p304);

        s.store_add_scaled_inputs3_offset_indices(273, 477, p.p562, 478, p.p752, 479, p.p942, p.p305);

        s.store_add_scaled_inputs3_offset_indices(274, 477, p.p563, 478, p.p753, 479, p.p943, p.p306);

        s.store_add_scaled_inputs3_offset_indices(275, 477, p.p564, 478, p.p754, 479, p.p944, p.p307);

        s.store_add_scaled_inputs3_offset_indices(276, 477, p.p565, 478, p.p755, 479, p.p945, p.p309);

        s.store_add_scaled_inputs3_offset_indices(277, 477, p.p566, 478, p.p756, 479, p.p946, p.p321);

        s.store_add_scaled_inputs3_offset_indices(278, 477, p.p567, 478, p.p757, 479, p.p947, p.p310);

        s.store_add_scaled_inputs3_offset_indices(279, 477, p.p568, 478, p.p758, 479, p.p948, p.p311);

        s.store_add_scaled_inputs3_offset_indices(280, 477, p.p569, 478, p.p759, 479, p.p949, p.p312);

        s.store_add_scaled_inputs3_offset_indices(281, 477, p.p570, 478, p.p760, 479, p.p950, p.p313);

        s.store_add_scaled_inputs3_offset_indices(282, 477, p.p571, 478, p.p761, 479, p.p951, p.p158);

        s.store_add_scaled_inputs3_offset_indices(283, 477, p.p572, 478, p.p762, 479, p.p952, p.p159);

        s.store_add_scaled_inputs3_offset_indices(284, 477, p.p573, 478, p.p763, 479, p.p953, p.p160);

        s.store_add_scaled_inputs3_offset_indices(285, 477, p.p574, 478, p.p764, 479, p.p954, p.p161);

        s.store_add_scaled_inputs3_offset_indices(286, 477, p.p1025, 478, p.p1028, 479, p.p1031, p.p1022);

        s.store_add_scaled_inputs3_offset_indices(287, 477, p.p575, 478, p.p765, 479, p.p955, p.p162);

        s.store_add_scaled_inputs3_offset_indices(288, 477, p.p576, 478, p.p766, 479, p.p956, p.p163);

        s.store_add_scaled_inputs3_offset_indices(289, 477, p.p577, 478, p.p767, 479, p.p957, p.p164);

        s.store_add_scaled_inputs3_offset_indices(290, 477, p.p578, 478, p.p768, 479, p.p958, p.p165);

        s.store_add_scaled_inputs3_offset_indices(291, 477, p.p579, 478, p.p769, 479, p.p959, p.p166);

        s.store_add_scaled_inputs3_offset_indices(292, 477, p.p580, 478, p.p770, 479, p.p960, p.p167);

        s.store_add_scaled_inputs3_offset_indices(293, 477, p.p581, 478, p.p771, 479, p.p961, p.p168);

        s.store_add_scaled_inputs3_offset_indices(294, 477, p.p1026, 478, p.p1029, 479, p.p1032, p.p1023);

        s.store_add_scaled_inputs3_offset_indices(295, 477, p.p582, 478, p.p772, 479, p.p962, p.p169);

        s.store_add_scaled_inputs3_offset_indices(296, 477, p.p583, 478, p.p773, 479, p.p963, p.p170);

        s.store_add_scaled_inputs3_offset_indices(297, 477, p.p584, 478, p.p774, 479, p.p964, p.p171);

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_add_scaled_inputs3_offset_indices(298, 477, p.p585, 478, p.p775, 479, p.p965, p.p322);

        s.store_add_scaled_inputs3_offset_indices(299, 477, p.p586, 478, p.p776, 479, p.p966, p.p323);

        s.store_add_scaled_inputs3_offset_indices(300, 477, p.p587, 478, p.p777, 479, p.p967, p.p172);

        s.store_add_scaled_inputs3_offset_indices(301, 477, p.p588, 478, p.p778, 479, p.p968, p.p173);

        s.store_add_scaled_inputs3_offset_indices(302, 477, p.p589, 478, p.p779, 479, p.p969, p.p324);

        s.store_add_scaled_inputs3_offset_indices(303, 477, p.p590, 478, p.p780, 479, p.p970, p.p325);

        s.store_add_scaled_inputs3_offset_indices(304, 477, p.p591, 478, p.p781, 479, p.p971, p.p326);

        s.store_add_scaled_inputs3_offset_indices(305, 477, p.p592, 478, p.p782, 479, p.p972, p.p327);

        s.store_add_scaled_inputs3_offset_indices(306, 477, p.p593, 478, p.p783, 479, p.p973, p.p328);

        s.store_add_scaled_inputs3_offset_indices(307, 477, p.p594, 478, p.p784, 479, p.p974, p.p329);

        s.store_add_scaled_inputs3_offset_indices(308, 477, p.p595, 478, p.p785, 479, p.p975, p.p330);

        s.store_add_scaled_inputs3_offset_indices(309, 477, p.p596, 478, p.p786, 479, p.p976, p.p331);

        s.store_add_scaled_inputs3_offset_indices(310, 477, p.p597, 478, p.p787, 479, p.p977, p.p332);

        s.store_add_scaled_inputs3_offset_indices(312, 477, p.p599, 478, p.p789, 479, p.p979, p.p334);

        s.store_add_scaled_inputs3_offset_indices(311, 477, p.p598, 478, p.p788, 479, p.p978, p.p333);

        s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);

        s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);

        s.store_add_scaled_inputs3_offset_indices(314, 477, p.p601, 478, p.p791, 479, p.p981, p.p337);

        s.store_add_scaled_inputs3_offset_indices(315, 477, p.p602, 478, p.p792, 479, p.p982, p.p338);

        s.store_add_scaled_inputs3_offset_indices(316, 477, p.p603, 478, p.p793, 479, p.p983, p.p339);

        s.store_add_scaled_inputs3_offset_indices(317, 477, p.p604, 478, p.p794, 479, p.p984, p.p340);

        s.store_add_scaled_inputs3_offset_indices(318, 477, p.p605, 478, p.p795, 479, p.p985, p.p341);

        s.store_add_scaled_inputs3_offset_indices(319, 477, p.p606, 478, p.p796, 479, p.p986, p.p342);

        s.store_add_scaled_inputs3_offset_indices(320, 477, p.p607, 478, p.p797, 479, p.p987, p.p344);

        s.store_add_scaled_inputs3_offset_indices(321, 477, p.p608, 478, p.p798, 479, p.p988, p.p345);

        s.store_add_scaled_inputs3_offset_indices(355, 477, p.p609, 478, p.p799, 479, p.p989, p.p346);

        s.store_add_scaled_inputs3_offset_indices(356, 477, p.p610, 478, p.p800, 479, p.p990, p.p347);

        s.store_add_scaled_inputs3_offset_indices(242, 477, p.p443, 478, p.p633, 479, p.p823, p.p157);

        s.store_add_scaled_inputs3_offset_indices(243, 477, p.p444, 478, p.p634, 479, p.p824, p.p383);

        s.store_add_scaled_inputs3_offset_indices(244, 477, p.p445, 478, p.p635, 479, p.p825, p.p384);

        s.store_add_scaled_inputs3_offset_indices(246, 477, p.p447, 478, p.p637, 479, p.p827, p.p388);

        s.store_add_scaled_inputs3_offset_indices(247, 477, p.p448, 478, p.p638, 479, p.p828, p.p389);

        s.store_add_scaled_inputs3_offset_indices(245, 477, p.p446, 478, p.p636, 479, p.p826, p.p385);

        s.store_add_scaled_inputs3_offset_indices(249, 477, p.p449, 478, p.p639, 479, p.p829, p.p390);

        s.store_add_scaled_inputs3_offset_indices(253, 477, p.p457, 478, p.p647, 479, p.p837, p.p352);

        s.store_add_scaled_inputs3_offset_indices(254, 477, p.p467, 478, p.p657, 479, p.p847, p.p358);

        s.store_add_scaled_inputs3_offset_indices(255, 477, p.p468, 478, p.p658, 479, p.p848, p.p359);

        s.store_add_scaled_inputs3_offset_indices(256, 477, p.p469, 478, p.p659, 479, p.p849, p.p174);

        s.store_add_scaled_inputs3_offset_indices(257, 477, p.p470, 478, p.p660, 479, p.p850, p.p175);

        s.store_add_scaled_inputs3_offset_indices(258, 477, p.p471, 478, p.p661, 479, p.p851, p.p176);

        s.store_add_scaled_inputs3_offset_indices(259, 477, p.p472, 478, p.p662, 479, p.p852, p.p177);

        s.store_add_scaled_inputs3_offset_indices(260, 477, p.p473, 478, p.p663, 479, p.p853, p.p178);

        s.store_add_scaled_inputs3_offset_indices(261, 477, p.p474, 478, p.p664, 479, p.p854, p.p179);

        s.store_add_scaled_inputs3_offset_indices(262, 477, p.p475, 478, p.p665, 479, p.p855, p.p180);

        s.store_add_scaled_inputs3_offset_indices(237, 477, p.p455, 478, p.p645, 479, p.p835, p.p211);

        s.store_add_scaled_inputs3_offset_indices(236, 477, p.p454, 478, p.p644, 479, p.p834, p.p210);

        s.store_add_scaled_inputs3_offset_indices(238, 477, p.p456, 478, p.p646, 479, p.p836, p.p212);

        s.store_add_scaled_inputs3_offset_indices(145, 477, p.p458, 478, p.p648, 479, p.p838, p.p118);

        s.store_add_scaled_inputs3_offset_indices(146, 477, p.p514, 478, p.p704, 479, p.p894, p.p121);

        s.store_add_scaled_inputs3_offset_indices(147, 477, p.p515, 478, p.p705, 479, p.p895, p.p122);

        s.store_add_scaled_inputs3_offset_indices(148, 477, p.p510, 478, p.p700, 479, p.p890, p.p117);

        s.store_add_scaled_inputs3_offset_indices(149, 477, p.p517, 478, p.p707, 479, p.p897, p.p119);

        s.store_add_scaled_inputs3_offset_indices(150, 477, p.p516, 478, p.p706, 479, p.p896, p.p120);

        s.store_add_scaled_inputs3_offset_indices(121, 477, p.p459, 478, p.p649, 479, p.p839, p.p91);

        s.store_add_scaled_inputs3_offset_indices(123, 477, p.p461, 478, p.p651, 479, p.p841, p.p93);

        s.store_add_scaled_inputs3_offset_indices(122, 477, p.p460, 478, p.p650, 479, p.p840, p.p92);

        s.store_add_scaled_inputs3_offset_indices(139, 477, p.p462, 478, p.p652, 479, p.p842, p.p111);

        s.store_add_scaled_inputs3_offset_indices(141, 477, p.p463, 478, p.p653, 479, p.p843, p.p113);

        s.store_add_scaled_inputs3_offset_indices(143, 477, p.p464, 478, p.p654, 479, p.p844, p.p115);

        s.store_add_scaled_inputs3_offset_indices(102, 477, p.p465, 478, p.p655, 479, p.p845, p.p75);

        s.store_add_scaled_inputs3_offset_indices(186, 477, p.p466, 478, p.p656, 479, p.p846, p.p144);

        s.store_add_scaled_inputs3_offset_indices(211, 477, p.p484, 478, p.p674, 479, p.p864, p.p406);

        s.store_add_scaled_inputs3_offset_indices(203, 477, p.p476, 478, p.p666, 479, p.p856, p.p398);

        s.store_add_scaled_inputs3_offset_indices(204, 477, p.p477, 478, p.p667, 479, p.p857, p.p399);

        s.store_add_scaled_inputs3_offset_indices(205, 477, p.p478, 478, p.p668, 479, p.p858, p.p400);

        s.store_add_scaled_inputs3_offset_indices(206, 477, p.p479, 478, p.p669, 479, p.p859, p.p401);

        s.store_add_scaled_inputs3_offset_indices(207, 477, p.p480, 478, p.p670, 479, p.p860, p.p402);

        s.store_add_scaled_inputs3_offset_indices(208, 477, p.p481, 478, p.p671, 479, p.p861, p.p403);

        s.store_add_scaled_inputs3_offset_indices(209, 477, p.p482, 478, p.p672, 479, p.p862, p.p404);

        s.store_add_scaled_inputs3_offset_indices(210, 477, p.p483, 478, p.p673, 479, p.p863, p.p405);

        s.store_add_scaled_inputs3_offset_indices(212, 477, p.p485, 478, p.p675, 479, p.p865, p.p407);

        s.store_add_scaled_inputs3_offset_indices(213, 477, p.p486, 478, p.p676, 479, p.p866, p.p408);

        s.store_add_scaled_inputs3_offset_indices(229, 477, p.p618, 478, p.p808, 479, p.p998, p.p422);

        s.store_add_scaled_inputs3_offset_indices(230, 477, p.p619, 478, p.p809, 479, p.p999, p.p423);

        s.store_add_scaled_inputs3_offset_indices(216, 477, p.p620, 478, p.p810, 479, p.p1000, p.p413);

        s.store_add_scaled_inputs3_offset_indices(217, 477, p.p621, 478, p.p811, 479, p.p1001, p.p433);

        s.store_add_scaled_inputs3_offset_indices(218, 477, p.p622, 478, p.p812, 479, p.p1002, p.p434);

        s.store_add_scaled_inputs3_offset_indices(219, 477, p.p623, 478, p.p813, 479, p.p1003, p.p414);

        s.store_add_scaled_inputs3_offset_indices(220, 477, p.p624, 478, p.p814, 479, p.p1004, p.p415);

        s.store_add_scaled_inputs3_offset_indices(221, 477, p.p625, 478, p.p815, 479, p.p1005, p.p416);

        s.store_add_scaled_inputs3_offset_indices(222, 477, p.p626, 478, p.p816, 479, p.p1006, p.p417);

        s.store_add_scaled_inputs3_offset_indices(223, 477, p.p627, 478, p.p817, 479, p.p1007, p.p418);

        s.store_add_scaled_inputs3_offset_indices(224, 477, p.p628, 478, p.p818, 479, p.p1008, p.p419);

        s.store_add_scaled_inputs3_offset_indices(225, 477, p.p629, 478, p.p819, 479, p.p1009, p.p420);

        s.store_add_scaled_inputs3_offset_indices(226, 477, p.p630, 478, p.p820, 479, p.p1010, p.p421);

        s.store_add_scaled_inputs3_offset_indices(227, 477, p.p631, 478, p.p821, 479, p.p1011, p.p411);

        s.store_add_scaled_inputs3_offset_indices(228, 477, p.p632, 478, p.p822, 479, p.p1012, p.p412);

        s.store_add_scaled_inputs3_offset_indices(322, 477, p.p611, 478, p.p801, 479, p.p991, p.p353);

        s.store_add_scaled_inputs3_offset_indices(323, 477, p.p612, 478, p.p802, 479, p.p992, p.p354);

        s.store_add_scaled_inputs3_offset_indices(324, 477, p.p613, 478, p.p803, 479, p.p993, p.p370);

        s.store_add_scaled_inputs3_offset_indices(361, 477, p.p614, 478, p.p804, 479, p.p994, p.p366);

        s.store_mul_powf_ad_rhs(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));

        s.store_add_scaled_inputs3_offset_indices(362, 477, p.p615, 478, p.p805, 479, p.p995, p.p367);

        s.store_add_scaled_inputs3_offset_indices(363, 477, p.p616, 478, p.p806, 479, p.p996, p.p368);

        s.store_add_scaled_inputs3_offset_indices(364, 477, p.p617, 478, p.p807, 479, p.p997, p.p369);

        s.store_add_scaled_inputs3_offset_indices(378, 477, p.p259, 478, p.p260, 479, p.p261, p.p258);

        s.store_add_scaled_inputs3_offset_indices(379, 477, p.p263, 478, p.p264, 479, p.p265, p.p262);

        s.store_add_scaled_inputs3_offset_indices(380, 477, p.p267, 478, p.p268, 479, p.p269, p.p266);

        s.store_add_scaled_inputs3_offset_indices(381, 477, p.p271, 478, p.p272, 479, p.p273, p.p270);

        s.store_add_scaled_inputs3_offset_indices(382, 477, p.p275, 478, p.p276, 479, p.p277, p.p274);

        s.store_add_scaled_inputs3_offset_indices(383, 477, p.p279, 478, p.p280, 479, p.p281, p.p278);

        s.store_add_scaled_inputs3_offset_indices(389, 477, p.p436, 478, p.p437, 479, p.p438, p.p435);

        s.store_add_scaled_inputs3_offset_indices(390, 477, p.p440, 478, p.p441, 479, p.p442, p.p439);

        s.store_add_scaled_inputs3_offset_indices(385, 477, p.p286, 478, p.p289, 479, p.p292, p.p285);

        s.store_add_scaled_inputs3_offset_indices(386, 477, p.p287, 478, p.p290, 479, p.p293, p.p282);

        s.store_add_scaled_inputs3_offset_indices(387, 477, p.p288, 478, p.p291, 479, p.p294, p.p284);

        s.store_add_scaled_inputs3_offset_indices(250, 477, p.p450, 478, p.p640, 479, p.p830, p.p392);

        s.store_add_scaled_inputs3_offset_indices(248, 477, p.p451, 478, p.p641, 479, p.p831, p.p393);

        s.store_add_scaled_inputs3_offset_indices(251, 477, p.p452, 478, p.p642, 479, p.p832, p.p394);

        s.store_add_scaled_inputs3_offset_indices(252, 477, p.p453, 478, p.p643, 479, p.p833, p.p395);

        s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);

        s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);

        s.v[430] = (s.v[476] - 1.0);

        s.copy_ad(153, 138);

        s.copy_ad(154, 140);

        s.copy_ad(155, 142);

        s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));

        s.v[157] = ((p.p14 / (p.p3 * (s.v[328] + p.p377))) * p.p23);

        s.v[158] = ((p.p15 * (p.p3 * (s.v[328] + p.p377))) / p.p23);

        s.b[547] = (s.v[38] == 0.0);
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        if s.b[547] {
            s.store_scalar(156, 0.0);
        }

        if (!s.b[547]) {
            s.store_div_scaled_inputs(156, s.ad_value(38), (((p.p17 * p.p378) * (s.v[328] * 1.0 / (p.p23))) * 1.0 / (p.p3)), A::scale_offset(s.ad_value(38), 2.0, (p.p378 * s.v[327])), 1.0);
        }

        s.v[345] = (((((p.p380 / p.p376)) as f64).powf(p.p379) / p.p376) / p.p376);

        s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);

        s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);

        s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);

        s.b[548] = (s.v[144] > 1.0);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if s.b[548] {
            s.store_scale(144, 144, 0.0001);
        }

        s.store_mul_ad_rhs(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));

        s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);

        s.store_div_scaled_inputs2(182, s.ad_value(181), 1.0, s.ad_value(186), s.v[430], s.ad_value(159), 1.0);

        s.b[549] = (p.p429 == 1.0);
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        if s.b[549] {
            s.store_scale(496, 159, p.p3);
            s.store_scale(497, 186, s.v[430]);
            s.store_add(468, 169, 497);
            s.store_offset(469, 497, p.p140);
        }

        s.b[550] = (s.v[468] < 0.0);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[550]) {
            s.store_scalar(468, 0.0);
        }

        s.b[551] = (s.v[469] < 0.0);
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[551]) {
            s.store_scalar(469, 0.0);
        }

        if s.b[549] {
            s.store_div(173, 468, 496);
            s.store_add(470, 170, 497);
            s.store_offset(471, 497, p.p139);
        }

        s.b[552] = (s.v[470] < 0.0);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[552]) {
            s.store_scalar(470, 0.0);
        }

        s.b[553] = (s.v[471] < 0.0);
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        if (s.b[549] && s.b[553]) {
            s.store_scalar(471, 0.0);
        }

        if s.b[549] {
            s.store_div(174, 470, 496);
        }

        if (!s.b[549]) {
            s.store_scalar(173, 0.0);
            s.store_scalar(174, 0.0);
        }

        s.b[554] = param_given[128];
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if s.b[554] {
            s.store_scalar(47, p.p128);
        }

        s.b[555] = (param_given[217] && (p.p217 > 0.0));
        s.v[555] = if s.b[555] { 1.0 } else { 0.0 };

        if ((!s.b[554]) && s.b[555]) {
            s.store_sub_scaled_inputs(47, 396, p.p217, 237, 1.0);
        }

        if ((!s.b[554]) && (!s.b[555])) {
            s.store_scale(47, 396, (0.6 * p.p157));
        }

        s.b[556] = param_given[127];
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_scalar(40, p.p127);
        }

        s.b[557] = (param_given[217] && (p.p217 > 0.0));
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if ((!s.b[556]) && s.b[557]) {
            s.store_sub_scaled_inputs(40, 396, p.p217, 236, 1.0);
        }

        if ((!s.b[556]) && (!s.b[557])) {
            s.store_scale(40, 396, (0.6 * p.p157));
        }

        s.b[558] = (s.v[47] < 0.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if s.b[558] {
            s.store_scalar(47, 0.0);
        }

        s.b[559] = (s.v[40] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if s.b[559] {
            s.store_scalar(40, 0.0);
        }

        s.b[560] = (s.v[42] < 0.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_scalar(42, 0.0);
        }

        s.store_scaled_add(335, 47, 239, s.v[349]);

        s.store_scaled_add(334, 40, 239, s.v[350]);

        s.store_scale(336, 42, (s.v[331] * p.p3));

        s.b[561] = ((!param_given[82]) && param_given[85]);
        s.v[561] = if s.b[561] { 1.0 } else { 0.0 };

        if s.b[561] {
            s.store_scale(467, 396, s.v[112]);
            s.store_scaled_mul(108, 467, 467, 3.021e22);
        }

        s.b[562] = (s.v[37] == 2.0);
        s.v[562] = if s.b[562] { 1.0 } else { 0.0 };

        if (s.b[562] && (p.p41 != 0.0)) {
            s.store_scale(422, 417, ((((p.p49 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p156 * p.p156))));
        }

        s.b[563] = (s.v[108] > s.v[422]);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if ((s.b[562] && (p.p41 != 0.0)) && s.b[563]) {
            s.copy_ad(108, 422);
        }

        if (s.b[562] && (p.p41 == 0.0)) {
            s.store_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p155 * p.p155))));
        }

        s.b[564] = (s.v[108] > s.v[422]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if ((s.b[562] && (p.p41 == 0.0)) && s.b[564]) {
            s.copy_ad(108, 422);
        }

        s.v[392] = (3.453133e-11 / p.p154);

        if (p.p41 != 0.0) {
            s.store_scalar(393, (1.03594e-10 / p.p156));
        }

        if (p.p41 == 0.0) {
            s.store_scalar(393, (1.03594e-10 / p.p155));
        }

        if (p.p41 != 0.0) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p156))));
        }

        if (p.p41 == 0.0) {
            s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p155))));
        }

        s.store_add_ad_lhs(421, A::sub_from_scalar(0.8, A::div_scaled_inputs(s.ad_value(420), 0.5, s.ad_value(393), 1.0)), 216);

        s.b[565] = (s.v[37] == 3.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = (s.v[421] > s.v[228]);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        if (s.b[565] && s.b[566]) {
            s.store_scalar(37, 2.0);
        }

        s.b[567] = (s.v[421] < s.v[227]);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        if ((s.b[565] && (!s.b[566])) && s.b[567]) {
            s.store_scalar(37, 0.0);
        }

        if ((s.b[565] && (!s.b[566])) && (!s.b[567])) {
            s.store_scalar(37, 1.0);
        }

        s.store_scale_ad(471, A::div_from_scalar(1.115, s.ad_value(49)), s.v[430]);

        s.store_div_scaled_product_indices(532, 256, 471, 1.0, 300, 1.0);

        s.b[568] = (s.v[532] > 100.0);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if s.b[568] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[569] = (s.v[532] < (-100.0));
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if ((!s.b[568]) && s.b[569]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[568]) && (!s.b[569])) {
            s.store_exp(467, 532);
        }

        s.store_div_scaled_product_indices(532, 257, 471, 1.0, 300, 1.0);

        s.b[570] = (s.v[532] > 100.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        if s.b[570] {
            s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[571] = (s.v[532] < (-100.0));
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if ((!s.b[570]) && s.b[571]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[570]) && (!s.b[571])) {
            s.store_exp(468, 532);
        }

        s.store_div_scaled_product_indices(532, 258, 471, 1.0, 302, 1.0);

        s.b[572] = (s.v[532] > 100.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if s.b[572] {
            s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[573] = (s.v[532] < (-100.0));
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((!s.b[572]) && s.b[573]) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!s.b[572]) && (!s.b[573])) {
            s.store_exp(469, 532);
        }

        s.store_mul(357, 355, 467);

        s.store_mul(161, 306, 467);

        s.store_mul(163, 308, 468);

        s.store_mul(165, 310, 469);

        s.store_scale(532, 259, s.v[430]);

        s.b[574] = (s.v[532] > 100.0);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if s.b[574] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[575] = (s.v[532] < (-100.0));
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        if ((!s.b[574]) && s.b[575]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[574]) && (!s.b[575])) {
            s.store_exp(467, 532);
        }

        s.store_mul(167, 312, 467);

        s.store_div_scaled_product_indices(532, 256, 471, 1.0, 301, 1.0);

        s.b[576] = (s.v[532] > 100.0);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if s.b[576] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[577] = (s.v[532] < (-100.0));
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if ((!s.b[576]) && s.b[577]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[576]) && (!s.b[577])) {
            s.store_exp(467, 532);
        }

        s.store_div_scaled_product_indices(532, 260, 471, 1.0, 301, 1.0);

        s.b[578] = (s.v[532] > 100.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if s.b[578] {
            s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[579] = (s.v[532] < (-100.0));
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if ((!s.b[578]) && s.b[579]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[578]) && (!s.b[579])) {
            s.store_exp(468, 532);
        }

        s.store_div_scaled_product_indices(532, 261, 471, 1.0, 303, 1.0);

        s.b[580] = (s.v[532] > 100.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if s.b[580] {
            s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[581] = (s.v[532] < (-100.0));
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if ((!s.b[580]) && s.b[581]) {
            s.store_scalar(469, 3.720075976e-44);
        }

        if ((!s.b[580]) && (!s.b[581])) {
            s.store_exp(469, 532);
        }

        s.store_mul(358, 356, 467);

        s.store_mul(162, 307, 467);

        s.store_mul(164, 309, 468);

        s.store_mul(166, 311, 469);

        s.store_scale(532, 262, s.v[430]);

        s.b[582] = (s.v[532] > 100.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if s.b[582] {
            s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[583] = (s.v[532] < (-100.0));
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((!s.b[582]) && s.b[583]) {
            s.store_scalar(467, 3.720075976e-44);
        }

        if ((!s.b[582]) && (!s.b[583])) {
            s.store_exp(467, 532);
        }

        s.store_mul(168, 313, 467);

        s.b[584] = (s.v[109] > 0.0);
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if s.b[584] {
            s.store_mul_scaled_ad_rhs(160, 49, (-p.p37), {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[584]) {
            s.store_mul_sub_scaled_inputs_rhs(160, 49, {
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p37), s.ad_value(530), ((2.0) * ((-p.p37))));
        }

        s.b[585] = (!param_given[353]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        s.b[586] = (s.v[109] > 0.0);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p.p37));
        }

        s.b[587] = (s.v[109] < 0.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p.p37));
        }

        s.store_mul_sub_scaled_inputs_rhs(481, 49, {
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, s.ad_value(530), 2.0);

        s.store_mul_scaled_sqrt_ad_rhs(482, 419, 1.0 / (s.v[392]), A::abs(s.ad_value(109)));

        s.b[588] = (!param_given[354]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        s.b[589] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (s.b[588] && s.b[589]) {
            s.store_add_scaled_inputs_product_right_ad(323, 322, 1.0, 481, 1.0, 482, A::sqrt(s.ad_value(481)), 1.0);
        }

        if (s.b[588] && (!s.b[589])) {
            s.store_add_scaled_inputs_product_right_ad(323, 322, 1.0, 481, (-1.0), 482, A::sqrt(s.ad_value(481)), (-1.0));
        }

        s.b[590] = (!param_given[355]);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_sqrt_ad(462, A::div_scaled_product(s.ad_value(417), s.ad_value(481), 2.0, A::abs(s.ad_value(109)), (1.602176462e-19 * 1000000.0)));
            s.store_div(463, 417, 462);
            s.store_div_scaled_value_offset_denominator(43, s.ad_value(463), s.v[392], s.ad_value(463), s.v[392], 1.0);
        }

        s.store_mul_sub_scaled_inputs_rhs(118, 49, {
            if (s.v[108] > 1e-38) {
                A::ln(s.ad_value(108))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, s.ad_value(530), 2.0);

        s.store_sqrt(339, 118);

        s.store_mul_sqrt_ad_lhs(340, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 339);

        s.store_sqrt(341, 340);

        s.b[591] = (p.p41 == 0.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if s.b[591] {
            s.store_sqrt_scaled_input_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p.p66);
        }

        if (!s.b[591]) {
            s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));
        }

        s.store_mul_sub_scaled_inputs_rhs(115, 49, {
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, s.ad_value(530), 2.0);

        s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));

        s.b[592] = (p.p41 == 0.0);
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        s.b[593] = (s.v[110] > 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[592] && s.b[593]) {
            s.store_mul_ad_rhs(375, 480, {
                if ((s.v[110] / 1e20) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(110), 1.0 / (1e20))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_scalar(375, 0.0);
        }

        if (!s.b[592]) {
            s.store_mul_sub_ad_rhs(467, 480, {
                if (s.v[111] > 1e-38) {
                    A::ln(s.ad_value(111))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, s.ad_value(530));
        }

        if (!s.b[592]) {
            s.store_scale(468, 466, 0.5);
        }

        s.b[594] = (s.v[467] > s.v[468]);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((!s.b[592]) && s.b[594]) {
            s.copy_ad(467, 468);
        }

        if (!s.b[592]) {
            s.store_sub_scaled_ad_lhs(469, A::offset(s.ad_value(468), p.p53), 467, p.p37);
            s.store_sub_from_scalar(375, p.p52, 469);
        }

        s.v[368] = (((((p.p379 * (if ((p.p380 / p.p376) > 1e-38) { (((p.p380 / p.p376)) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p.p376) / p.p376);

        s.store_div_scaled_value_by_product(371, A::exp_scaled_input({
            if ((p.p380 / (p.p376 * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p.p380, A::scale(s.ad_value(213), p.p376)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p379), (1.0 / (p.p376) * 1.0 / (p.p376)), s.ad_value(213), s.ad_value(213), 1.0);

        s.v[369] = (if (p.p37 == 1.0) { p.p1040 } else { p.p1039 });

        s.v[370] = (if (p.p37 == 1.0) { p.p1042 } else { p.p1041 });

        s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p25)));

        s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p24)));

        s.store_scale(374, 213, ((-s.v[370]) * p.p376));

        s.v[369] = ((s.v[369] * s.v[368]) * (((s.v[328] / p.p23) * s.v[327]) + (p.p28 / p.p3)));

        s.v[370] = (s.v[370] * (-p.p376));

        s.b[595] = (param_given[90] || param_given[94]);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        s.b[596] = (!param_given[90]);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_scalar(120, 0.53);
        }

        s.b[597] = (!param_given[94]);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[597]) {
            s.store_scalar(124, (-0.0186));
        }

        s.b[603] = (!param_given[87]);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (((!s.b[595]) && s.b[603]) && (p.p41 != 0.0)) {
            s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);
        }

        if (((!s.b[595]) && s.b[603]) && (p.p41 == 0.0)) {
            s.store_scalar(467, 0.00077348);
        }

        if ((!s.b[595]) && s.b[603]) {
            s.store_add_scaled_product_indices(114, 118, 1.0, 467, 108, (-(s.v[117] * s.v[117])));
        }

        s.b[604] = (s.v[114] > 0.0);
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[604]) {
            s.store_neg(114, 114);
        }

        s.b[605] = (s.v[116] > 0.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[605]) {
            s.store_scalar(116, (-s.v[116]));
        }

        s.b[606] = (!param_given[85]);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[606]) {
            s.store_div_scaled_product_right_ad(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);
        }

        s.b[607] = (!param_given[86]);
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if ((!s.b[595]) && s.b[607]) {
            s.store_div_scaled_product_right_ad(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);
        }

        if (!s.b[595]) {
            s.store_sub(467, 112, 113);
            s.store_sub_ad_lhs(468, A::sqrt(A::sub(s.ad_value(118), s.ad_value(114))), 339);
            s.store_mul_sub_ad_rhs(469, 339, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), s.ad_value(339));
            s.store_div_scaled_product_add_scaled_denominator_indices(124, 467, 468, 1.0, 469, 2.0, 116, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(120, 113, 1.0, 124, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), (-2.0));
        }

        s.store_offset(467, 265, s.v[328]);

        s.b[608] = (s.v[467] < 1e-8);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if s.b[608] {
            s.store_scalar(467, 1e-8);
        }

        s.store_mul_offset_ad_rhs(346, 120, A::div(s.ad_value(264), s.ad_value(467)), 1.0);

        s.b[609] = (!param_given[109]);
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        s.b[610] = (param_given[108] || param_given[107]);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[609] && s.b[610]) {
            s.store_add_scaled_inputs_product_indices(152, 137, p.p37, 118, (-1.0), 346, 339, (-1.0));
        }

        if (s.b[609] && (!s.b[610])) {
            s.store_scalar(152, (-1.0));
        }

        s.b[611] = (!param_given[108]);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if s.b[611] {
            s.store_add_scaled_inputs_product_indices(137, 152, p.p37, 118, p.p37, 346, 339, p.p37);
        }

        s.store_scale(376, 346, (p.p66 * 1.0 / (p.p67)));

        s.store_mul(468, 397, 341);

        s.store_exp_ad(467, A::div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));

        s.store_add_scaled_product_indices(342, 467, 1.0, 467, 467, 2.0);

        s.store_exp_ad(467, A::div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));

        s.store_add_scaled_product_indices(469, 467, 1.0, 467, 467, 2.0);

        s.store_add_scaled_product_indices(343, 193, 1.0, 192, 469, 1.0);

        s.store_div_ad_rhs(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));

        s.b[612] = (s.v[44] < 0.0);
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if s.b[612] {
            s.store_scalar(44, 0.0);
        }

        s.v[467] = ((s.v[474]) as f64).powf(p.p239);

        s.store_offset(489, 44, s.v[475]);

        s.store_powf(468, 489, p.p240);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p244, s.ad_value(468)), (p.p243 / s.v[467])), A::div_from_scalar(p.p245, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(231, 463, 1.0);

        s.v[467] = ((s.v[474]) as f64).powf(p.p241);

        s.store_powf(468, 489, p.p242);

        s.store_add_ad(463, A::offset(A::div_from_scalar(p.p247, s.ad_value(468)), (p.p246 / s.v[467])), A::div_from_scalar(p.p248, A::scale(s.ad_value(468), s.v[467])));

        s.store_offset(232, 463, 1.0);

        s.store_sqrt_square_offset(232, 232, 1e-9);

        s.store_offset_scaled(233, 231, (1.0 + (p.p238 * s.v[430])), 1e-9);

        s.v[483] = (1.0 / (p.p232 + (0.5 * s.v[474])));

        s.v[484] = (1.0 / (p.p233 + (0.5 * s.v[474])));

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[235] = (s.v[483] + s.v[484]);

        s.store_scale_ad(234, A::div_from_scalar(p.p235, s.ad_value(233)), s.v[235]);

        s.b[613] = (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0))));
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        if s.b[613] {
            s.store_scalar(485, 0.0);
            s.store_scalar(486, 0.0);
        }

        s.b[614] = (s.v[45] < (-1.0));
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if (s.b[613] && s.b[614]) {
            s.store_scalar(45, (-1.0));
        }

        s.b[615] = (s.v[45] > 1.0);
        s.v[615] = if s.b[615] { 1.0 } else { 0.0 };

        if ((s.b[613] && (!s.b[614])) && s.b[615]) {
            s.store_scalar(45, 1.0);
        }

        if ((s.b[613] && (!s.b[614])) && (!s.b[615])) {
        }

        if s.b[613] {
            s.store_scalar(495, 0.0);
        }

        let mut assign6090_loop_guard: usize = 0;
        while {
            let assign6090_cond_e7340: f64 = if (s.b[613] && (s.v[495] < p.p3)) { 1.0 } else { 0.0 };
            assign6090_cond_e7340 != 0.0
        } {
            assign6090_loop_guard += 1;
            assert!(assign6090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[613] {
                s.store_div_from_scalar_offset_scaled_input(616, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p4 + (0.5 * s.v[474])));
                s.store_div_from_scalar_offset_scaled_input(617, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p5 + (0.5 * s.v[474])));
                s.store_add(485, 485, 616);
                s.store_add(486, 486, 617);
                s.store_offset(495, 495, 1.0);
            }
        }

        if s.b[613] {
            s.store_add(490, 485, 486);
            s.copy_ad(51, 490);
            s.store_mul_div_from_scalar_lhs(487, p.p235, 233, 490);
            s.store_div_scaled_offset_numerator(467, s.ad_value(487), 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);
            s.store_mul(404, 337, 467);
            s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);
            s.store_mul(407, 338, 468);
            s.store_offset(491, 490, (-s.v[235]));
            s.store_mul_div_from_scalar_lhs(488, p.p237, 232, 491);
            s.store_mul_div_from_scalar_ad_lhs(492, p.p249, A::powf(s.ad_value(232), p.p250), 491);
            s.store_mul_div_from_scalar_ad_lhs(493, p.p251, A::powf(s.ad_value(232), p.p252), 491);
            s.store_mul_div_from_scalar_ad_lhs(494, p.p253, A::powf(s.ad_value(232), p.p254), 491);
            s.store_add(408, 137, 488);
            s.store_add(402, 124, 492);
            s.store_add(400, 187, 493);
            s.store_add(401, 189, 494);
        }

        if (!s.b[613]) {
            s.copy_ad(404, 337);
            s.copy_ad(408, 137);
            s.copy_ad(407, 338);
            s.copy_ad(402, 124);
            s.copy_ad(400, 187);
            s.copy_ad(401, 189);
            s.store_scalar(51, 0.0);
            s.store_scalar(235, 0.0);
            s.store_scalar(45, 0.0);
        }

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.store_offset(408, 408, p.p20);

        s.store_offset(406, 152, (p.p37 * p.p20));

        s.v[52] = (s.v[392] * p.p8);

        s.store_scale(53, 43, p.p8);

        s.v[54] = (s.v[392] * p.p7);

        s.store_scale(55, 43, p.p7);

        s.b[618] = (s.v[43] > 0.0);
        s.v[618] = if s.b[618] { 1.0 } else { 0.0 };

        s.b[619] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[619] = if s.b[619] { 1.0 } else { 0.0 };

        if (s.b[618] && s.b[619]) {
            s.store_sub(467, 323, 322);
            s.store_add_scaled_inputs(175, 322, 1.0, 467, p.p356);
            s.store_sub_from_scalar(468, s.v[52], 53);
            s.store_div_scaled_value_by_product(469, s.ad_value(468), 1.0, s.ad_value(467), s.ad_value(467), 1.0);
            s.store_scale(176, 469, 1.0 / (p.p356));
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
            s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 53, 322, (-1.0));
            s.store_sub_from_scalar(468, s.v[54], 55);
            s.store_div_scaled_value_by_product(469, s.ad_value(468), 1.0, s.ad_value(467), s.ad_value(467), 1.0);
            s.store_scale(178, 469, 1.0 / (p.p356));
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
            s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 55, 322, (-1.0));
        }

        if (s.b[618] && (!s.b[619])) {
            s.store_sub(467, 322, 323);
            s.store_add_scaled_inputs(175, 323, 1.0, 467, p.p356);
            s.store_offset(468, 53, (-s.v[52]));
            s.store_div_scaled_value_by_product(469, s.ad_value(468), 1.0, s.ad_value(467), s.ad_value(467), 1.0);
            s.store_scale(176, 469, 1.0 / (p.p356));
            s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));
            s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));
            s.store_offset(468, 55, (-s.v[54]));
            s.store_div_scaled_value_by_product(469, s.ad_value(468), 1.0, s.ad_value(467), s.ad_value(467), 1.0);
            s.store_scale(178, 469, 1.0 / (p.p356));
            s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));
            s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));
        }

        if (!s.b[618]) {
            s.store_scalar(175, 0.0);
            s.store_scalar(176, 0.0);
            s.store_scalar(177, 0.0);
            s.store_scalar(56, 0.0);
            s.store_scalar(178, 0.0);
            s.store_scalar(179, 0.0);
            s.store_scalar(57, 0.0);
        }

        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));
        s.v[620] = if s.b[620] { 1.0 } else { 0.0 };

        if s.b[620] {
            s.store_scalar(46, 1.0);
        }

        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p.p155 / p.p154))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p.p155 / p.p154)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p357);

        s.v[468] = (p.p10 - p.p2);

        s.b[621] = (s.v[468] > 0.0);
        s.v[621] = if s.b[621] { 1.0 } else { 0.0 };

        if s.b[621] {
            s.store_scale(58, 467, s.v[468]);
        }

        if (!s.b[621]) {
            s.store_scalar(58, 0.0);
        }

        s.v[468] = (p.p9 - p.p2);

        s.b[622] = (s.v[468] > 0.0);
        s.v[622] = if s.b[622] { 1.0 } else { 0.0 };

        if s.b[622] {
            s.store_scale(59, 467, s.v[468]);
        }

        if (!s.b[622]) {
            s.store_scalar(59, 0.0);
        }

        s.v[61] = (p.p131 * p.p11);

        s.b[623] = ((p.p429 == 1.0) && (s.v[61] < p.p431));
        s.v[623] = if s.b[623] { 1.0 } else { 0.0 };

        if s.b[623] {
            s.store_scalar(61, p.p431);
        }

        s.v[60] = (p.p131 * p.p12);

        s.b[624] = ((p.p429 == 1.0) && (s.v[60] < p.p431));
        s.v[624] = if s.b[624] { 1.0 } else { 0.0 };

        if s.b[624] {
            s.store_scalar(60, p.p431);
        }

        s.b[625] = (s.v[36] < 1e-15);
        s.v[625] = if s.b[625] { 1.0 } else { 0.0 };

        if s.b[625] {
            s.store_scalar(36, 1e-15);
        }

        s.store_div_scalar_by_product(467, (((-0.5) * s.v[327]) * s.v[327]), s.ad_value(36), s.ad_value(36), 1.0);

        s.b[626] = (s.v[467] > 100.0);
        s.v[626] = if s.b[626] { 1.0 } else { 0.0 };

        if s.b[626] {
            s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[627] = (s.v[467] < (-100.0));
        s.v[627] = if s.b[627] { 1.0 } else { 0.0 };

        if ((!s.b[626]) && s.b[627]) {
            s.store_scalar(468, 3.720075976e-44);
        }

        if ((!s.b[626]) && (!s.b[627])) {
            s.store_exp(468, 467);
        }

        s.copy_ad(351, 468);

        s.store_mul_offset_ad_rhs(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), (1.0 / s.v[327]));

        s.store_pow_ad(352, s.ad_value(467), s.ad_value(318));

        s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p.p343, 1.0);

        s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);

        s.b[628] = (s.v[354] < 1.0);
        s.v[628] = if s.b[628] { 1.0 } else { 0.0 };

        if s.b[628] {
            s.store_scalar(354, 1.0);
        }

        s.b[629] = (p.p41 == 0.0);
        s.v[629] = if s.b[629] { 1.0 } else { 0.0 };

        if s.b[629] {
            s.store_scalar(62, (p.p66 - p.p68));
        }

        if (!s.b[629]) {
            s.store_scalar(498, (8.617087e-5 * p.p57));
            s.copy_ad(499, 498);
        }

        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs(500, 498, {
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(108), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(530), 2.0);
        }

        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs(501, 498, {
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, s.ad_value(530), 2.0);
        }

        if (!s.b[629]) {
            s.store_sqrt(502, 501);
            s.store_add(464, 406, 501);
            s.store_scalar(503, (p.p37 * p.p56));
            s.store_scalar(467, (p.p60 * 8.85418e-12));
        }

        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));
        s.v[630] = if s.b[630] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[630]) {
            s.store_div_scaled_product_denominator_ad(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);
            s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);
            s.store_mul_offset_rhs(469, 468, 471, (-1.0));
            s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);
            s.store_offset_sub_from_scalar_ad(532, p.p1034, s.ad_value(470), (-0.05));
            s.store_sqrt_square_offset(473, 532, 0.224);
            s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p.p1034);
            s.store_sub(504, 503, 472);
        }

        if ((!s.b[629]) && (!s.b[630])) {
            s.copy_ad(504, 503);
        }

        if (!s.b[629]) {
            s.store_sub(506, 500, 501);
            s.copy_ad(470, 341);
            s.store_mul(509, 397, 470);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {
            s.store_mul(510, 397, 470);
            s.store_div_scaled_inputs(467, s.ad_value(130), ((-0.5) * p.p54), s.ad_value(509), 1.0);
        }

        s.b[631] = (s.v[467] > (-100.0));
        s.v[631] = if s.b[631] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[631]) {
            s.store_exp(468, 467);
            s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);
        }

        if ((!s.b[629]) && (!s.b[631])) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);
        }

        if (!s.b[629]) {
            s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);
            s.copy_ad(470, 96);
            s.store_div_scaled_inputs2(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, s.ad_value(99), 1.0, s.ad_value(396), 1.0);
        }

        s.b[632] = (s.v[471] >= (-0.5));
        s.v[632] = if s.b[632] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[632]) {
            s.store_offset(511, 471, 1.0);
        }

        if ((!s.b[629]) && (!s.b[632])) {
            s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);
        }

        s.b[633] = (s.v[378] > 0.0);
        s.v[633] = if s.b[633] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[633]) {
            s.store_offset_scaled(470, 378, 2.0, p.p54);
        }

        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_ad_rhs(471, 499, {
                if ((p.p54 / s.v[470]) > 1e-38) {
                    A::ln(A::div_from_scalar(p.p54, s.ad_value(470)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[629]) && s.b[633]) {
            s.store_mul(519, 511, 471);
        }

        if ((!s.b[629]) && (!s.b[633])) {
            s.store_scalar(519, 0.0);
        }

        if (!s.b[629]) {
            s.store_mul(63, 129, 522);
            s.store_mul(523, 63, 506);
            s.store_div_scaled_inputs(467, s.ad_value(133), ((-0.5) * (p.p55 * p.p54)), s.ad_value(510), 1.0);
        }

        s.b[634] = (s.v[467] > (-100.0));
        s.v[634] = if s.b[634] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[634]) {
            s.store_exp(468, 467);
            s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);
        }

        if ((!s.b[629]) && (!s.b[634])) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);
        }

        if (!s.b[629]) {
            s.store_mul(467, 132, 469);
            s.store_mul(524, 467, 506);
            s.store_scalar(430, ((p.p57 / s.v[429]) - 1.0));
            s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p.p54), 1.0);
            s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p.p54));
            s.store_add_scaled_product_value_ad(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);
            s.store_div_scaled_product_offset_denominator(464, s.ad_value(415), s.ad_value(501), 1.0, s.ad_value(127), p.p55, 1.0);
            s.store_scalar(517, 0.0);
            s.store_scalar(521, 0.0);
            s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p.p54), 1.0);
            s.copy_ad(514, 502);
        }

        if (!s.b[629]) {
            let assign7680_ad_e8694: A = A::add_scaled_inputs4(A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, s.ad_value(520), 1.0, s.ad_value(517), -1.0, s.ad_value(519), -1.0);
            s.store_sub_ad_lhs(507, assign7680_ad_e8694, 521);
        }

        if (!s.b[629]) {
            s.store_sub(508, 504, 507);
            s.store_mul(497, 511, 499);
            s.store_div_scaled_product_indices(512, 384, 508, 1.0, 497, 1.0);
            s.store_div_scaled_inputs2(513, s.ad_value(151), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(508)), (-1.0), s.ad_value(497), 1.0);
        }

        s.b[635] = (s.v[512] > 100.0);
        s.v[635] = if s.b[635] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[635]) {
            s.copy_ad(505, 508);
        }

        s.b[636] = (s.v[513] > 100.0);
        s.v[636] = if s.b[636] { 1.0 } else { 0.0 };

        if (((!s.b[629]) && (!s.b[635])) && s.b[636]) {
            s.store_div_scaled_inputs2(467, s.ad_value(508), 1.0, s.ad_value(151), (-1.0), A::mul(s.ad_value(511), s.ad_value(499)), 1.0);
            s.store_exp(515, 467);
            s.store_mul_div_scaled_product_rhs(505, 515, s.ad_value(499), s.ad_value(367), 1.0, s.ad_value(396), 1.0);
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_exp(515, 512);
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul_ad_rhs(468, 497, {
                if ((1.0 + s.v[515]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(515), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul3_ad(471, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384)));
            s.store_sub_ad_rhs(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));
            s.store_div(505, 468, 469);
        }

        if (!s.b[629]) {
            s.store_add_scaled_inputs3_indices(470, 408, p.p37, 406, (-1.0), 501, -1.0);
            s.store_scale(516, 470, 4.0);
        }

        s.b[637] = (s.v[516] < 0.0);
        s.v[637] = if s.b[637] { 1.0 } else { 0.0 };

        if ((!s.b[629]) && s.b[637]) {
            s.store_scalar(516, 0.0);
        }

        if (!s.b[629]) {
            s.store_scalar(525, 0.0);
            s.copy_ad(526, 415);
            s.store_scalar(527, 1000000.0);
        }

        let mut assign7910_loop_guard: usize = 0;
        while {
            let assign7910_cond_e8932: f64 = (s.v[526] - s.v[527]);
            let assign7910_cond_e8932_d_n0: f64 = (s.dn[526][0] - s.dn[527][0]);
            let assign7910_cond_e8932_d_n1: f64 = (s.dn[526][1] - s.dn[527][1]);
            let assign7910_cond_e8932_d_n2: f64 = (s.dn[526][2] - s.dn[527][2]);
            let assign7910_cond_e8932_d_n3: f64 = (s.dn[526][3] - s.dn[527][3]);
            let assign7910_cond_e8932_d_n4: f64 = (s.dn[526][4] - s.dn[527][4]);
            let assign7910_cond_e8932_d_n5: f64 = (s.dn[526][5] - s.dn[527][5]);
            let assign7910_cond_e8932_d_n6: f64 = (s.dn[526][6] - s.dn[527][6]);
            let assign7910_cond_e8932_d_n7: f64 = (s.dn[526][7] - s.dn[527][7]);
            let assign7910_cond_e8932_d_n8: f64 = (s.dn[526][8] - s.dn[527][8]);
            let assign7910_cond_e8932_d_n9: f64 = (s.dn[526][9] - s.dn[527][9]);
            let assign7910_cond_e8932_d_n10: f64 = (s.dn[526][10] - s.dn[527][10]);
            let assign7910_cond_e8932_d_n11: f64 = (s.dn[526][11] - s.dn[527][11]);
            let assign7910_cond_e8932_d_n12: f64 = (s.dn[526][12] - s.dn[527][12]);
            let assign7910_cond_e8932_d_n13: f64 = (s.dn[526][13] - s.dn[527][13]);
            let assign7910_cond_e8932_d_b0: f64 = (s.db[526][0] - s.db[527][0]);
            let assign7910_cond_e8932_d_b1: f64 = (s.db[526][1] - s.db[527][1]);
            let assign7910_cond_e8932_d_b2: f64 = (s.db[526][2] - s.db[527][2]);
            let assign7910_cond_e8932_d_b3: f64 = (s.db[526][3] - s.db[527][3]);
            let assign7910_cond_e8932_d_b4: f64 = (s.db[526][4] - s.db[527][4]);
            let assign7910_cond_e8932_d_b5: f64 = (s.db[526][5] - s.db[527][5]);
            let assign7910_cond_e8932_d_b6: f64 = (s.db[526][6] - s.db[527][6]);
            let assign7910_cond_e8932_d_b7: f64 = (s.db[526][7] - s.db[527][7]);
            let assign7910_cond_e8932_d_b8: f64 = (s.db[526][8] - s.db[527][8]);
            let assign7910_cond_e8932_d_b9: f64 = (s.db[526][9] - s.db[527][9]);
            let assign7910_cond_e8932_d_b10: f64 = (s.db[526][10] - s.db[527][10]);
            let assign7910_cond_e8932_d_b11: f64 = (s.db[526][11] - s.db[527][11]);
            let assign7910_cond_e8932_d_b12: f64 = (s.db[526][12] - s.db[527][12]);
            let assign7910_cond_e8932_d_b13: f64 = (s.db[526][13] - s.db[527][13]);
            let assign7910_cond_e8932_d_b14: f64 = (s.db[526][14] - s.db[527][14]);
            let assign7910_cond_e8932_d_b15: f64 = (s.db[526][15] - s.db[527][15]);
            let assign7910_cond_e8932_d_b16: f64 = (s.db[526][16] - s.db[527][16]);
            let assign7910_cond_e8932_d_b17: f64 = (s.db[526][17] - s.db[527][17]);
            let assign7910_cond_e8933: f64 = (assign7910_cond_e8932).abs();
            let assign7910_cond_e8933_d_n0: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n0 } else { (-assign7910_cond_e8932_d_n0) };
            let assign7910_cond_e8933_d_n1: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n1 } else { (-assign7910_cond_e8932_d_n1) };
            let assign7910_cond_e8933_d_n2: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n2 } else { (-assign7910_cond_e8932_d_n2) };
            let assign7910_cond_e8933_d_n3: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n3 } else { (-assign7910_cond_e8932_d_n3) };
            let assign7910_cond_e8933_d_n4: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n4 } else { (-assign7910_cond_e8932_d_n4) };
            let assign7910_cond_e8933_d_n5: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n5 } else { (-assign7910_cond_e8932_d_n5) };
            let assign7910_cond_e8933_d_n6: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n6 } else { (-assign7910_cond_e8932_d_n6) };
            let assign7910_cond_e8933_d_n7: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n7 } else { (-assign7910_cond_e8932_d_n7) };
            let assign7910_cond_e8933_d_n8: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n8 } else { (-assign7910_cond_e8932_d_n8) };
            let assign7910_cond_e8933_d_n9: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n9 } else { (-assign7910_cond_e8932_d_n9) };
            let assign7910_cond_e8933_d_n10: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n10 } else { (-assign7910_cond_e8932_d_n10) };
            let assign7910_cond_e8933_d_n11: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n11 } else { (-assign7910_cond_e8932_d_n11) };
            let assign7910_cond_e8933_d_n12: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n12 } else { (-assign7910_cond_e8932_d_n12) };
            let assign7910_cond_e8933_d_n13: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_n13 } else { (-assign7910_cond_e8932_d_n13) };
            let assign7910_cond_e8933_d_b0: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b0 } else { (-assign7910_cond_e8932_d_b0) };
            let assign7910_cond_e8933_d_b1: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b1 } else { (-assign7910_cond_e8932_d_b1) };
            let assign7910_cond_e8933_d_b2: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b2 } else { (-assign7910_cond_e8932_d_b2) };
            let assign7910_cond_e8933_d_b3: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b3 } else { (-assign7910_cond_e8932_d_b3) };
            let assign7910_cond_e8933_d_b4: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b4 } else { (-assign7910_cond_e8932_d_b4) };
            let assign7910_cond_e8933_d_b5: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b5 } else { (-assign7910_cond_e8932_d_b5) };
            let assign7910_cond_e8933_d_b6: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b6 } else { (-assign7910_cond_e8932_d_b6) };
            let assign7910_cond_e8933_d_b7: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b7 } else { (-assign7910_cond_e8932_d_b7) };
            let assign7910_cond_e8933_d_b8: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b8 } else { (-assign7910_cond_e8932_d_b8) };
            let assign7910_cond_e8933_d_b9: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b9 } else { (-assign7910_cond_e8932_d_b9) };
            let assign7910_cond_e8933_d_b10: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b10 } else { (-assign7910_cond_e8932_d_b10) };
            let assign7910_cond_e8933_d_b11: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b11 } else { (-assign7910_cond_e8932_d_b11) };
            let assign7910_cond_e8933_d_b12: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b12 } else { (-assign7910_cond_e8932_d_b12) };
            let assign7910_cond_e8933_d_b13: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b13 } else { (-assign7910_cond_e8932_d_b13) };
            let assign7910_cond_e8933_d_b14: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b14 } else { (-assign7910_cond_e8932_d_b14) };
            let assign7910_cond_e8933_d_b15: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b15 } else { (-assign7910_cond_e8932_d_b15) };
            let assign7910_cond_e8933_d_b16: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b16 } else { (-assign7910_cond_e8932_d_b16) };
            let assign7910_cond_e8933_d_b17: f64 = if assign7910_cond_e8932 >= 0.0 { assign7910_cond_e8932_d_b17 } else { (-assign7910_cond_e8932_d_b17) };
            let assign7910_cond_e8937: f64 = if ((!s.b[629]) && ((s.v[525] <= 4.0) && (assign7910_cond_e8933 > 1e-12))) { 1.0 } else { 0.0 };
            assign7910_cond_e8937 != 0.0
        } {
            assign7910_loop_guard += 1;
            assert!(assign7910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[629]) {
                s.copy_ad(527, 526);
                s.store_scale(464, 526, 200000000.0);
                s.store_div_scaled_inputs2(638, s.ad_value(505), 1.0, s.ad_value(516), 1.0, s.ad_value(464), 1.0);
            }
            if (!s.b[629]) {
                s.store_offset_ad(639, A::exp_scaled_input({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p.p59 * 0.7)), 1.0);
            }
            if (!s.b[629]) {
                s.store_div_from_scalar(528, (p.p58 * 1.9e-9), 639);
                s.store_add_scaled_product_indices(526, 415, 1.0, 416, 528, (-1.0 / (p.p47)));
                s.store_offset(525, 525, 1.0);
            }
        }

        if (!s.b[629]) {
            s.copy_ad(62, 526);
        }

        s.copy_ad(462, 341);

        s.store_sub(463, 115, 118);

        s.store_mul(464, 397, 462);

        s.store_div_scaled_inputs(467, s.ad_value(133), ((-0.5) * (s.v[328] * s.v[327])), s.ad_value(464), 1.0);

        s.b[640] = (s.v[467] > (-100.0));
        s.v[640] = if s.b[640] { 1.0 } else { 0.0 };

        if s.b[640] {
            s.store_exp(468, 467);
            s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);
        }

        if (!s.b[640]) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);
        }

        s.store_mul(467, 132, 469);

        s.store_mul(469, 467, 463);

        s.store_div_scaled_inputs(467, s.ad_value(130), ((-0.5) * s.v[327]), s.ad_value(464), 1.0);

        s.b[641] = (s.v[467] > (-100.0));
        s.v[641] = if s.b[641] { 1.0 } else { 0.0 };

        if s.b[641] {
            s.store_exp(468, 467);
            s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);
        }

        if (!s.b[641]) {
            s.store_scalar(468, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);
        }

        s.store_mul3_lhs(470, 129, 470, 463);

        s.store_div_scaled_product_offset_denominator(471, s.ad_value(62), s.ad_value(118), 1.0, s.ad_value(127), s.v[328], 1.0);

        s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);

        s.store_add_scaled_product_mixed_aai(472, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), 430, 1.0);

        s.store_add_ad_lhs(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);

        s.store_add_scaled_inputs_product_indices(359, 531, 1.0, 118, (-1.0), 120, 339, (-1.0));

        s.store_mul_scale_offset_rhs(344, 108, 128, ((1.0 / (s.v[327])) * ((1.602176462e-19 * (1000000.0 * p.p155)))), (1.602176462e-19 * (1000000.0 * p.p155)));

        s.v[64] = (((p.p424 * (p.p427 + (((s.v[328] / p.p23) / 3.0) / p.p425))) / ((p.p425 * p.p3) * (p.p1 - p.p428))) + (p.p426 / ((p.p1 * s.v[328]) * p.p3)));

        s.b[642] = (s.v[64] > 0.0);
        s.v[642] = if s.b[642] { 1.0 } else { 0.0 };

        if s.b[642] {
            s.store_scalar(64, (1.0 / s.v[64]));
        }

        if (!s.b[642]) {
            s.store_scalar(64, 1000.0);
        }

        s.store_offset(67, 359, (p.p37 * p.p20));

        s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);

        s.store_add_scaled_inputs3_indices(468, 408, p.p37, 406, (-1.0), 118, -1.0);

        s.store_scale(469, 468, 2.0);

        s.store_scale(470, 468, 2.5);

        if (p.p37 == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }

        s.b[646] = (s.v[68] < 0.0);
        s.v[646] = if s.b[646] { 1.0 } else { 0.0 };

        if s.b[646] {
            s.store_scalar(68, 0.0);
        }

        s.b[647] = (p.p62 == 4.0);
        s.v[647] = if s.b[647] { 1.0 } else { 0.0 };

        if s.b[647] {
            s.store_mul(509, 397, 341);
            s.store_div_scaled_inputs(467, s.ad_value(130), s.v[327], s.ad_value(509), 1.0);
        }

        s.b[648] = (s.v[467] < 100.0);
        s.v[648] = if s.b[648] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[648]) {
            s.store_exp(468, 467);
            s.store_offset(469, 468, (-1.0));
            s.store_square(470, 469);
            s.store_add_scaled_inputs(471, 470, 1.0, 468, (2.0 * 3.720075976e-44));
            s.store_div(522, 468, 471);
        }

        if (s.b[647] && (!s.b[648])) {
            s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));
        }

        if s.b[647] {
            s.store_div(463, 417, 340);
            s.store_mul(464, 100, 463);
            s.store_div_scaled_inputs2(531, A::add_scaled_product(s.ad_value(464), 1.0, s.ad_value(96), s.ad_value(522), 1.0), 1.0, s.ad_value(99), 1.0, s.ad_value(396), 1.0);
        }

        s.b[649] = (s.v[531] >= (-0.5));
        s.v[649] = if s.b[649] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.b[647] && s.b[649]) {
            s.store_offset(529, 531, 1.0);
        }

        if (s.b[647] && (!s.b[649])) {
            s.store_div_from_scalar_offset_scaled_input(467, 1.0, 531, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(529, 467, 531, 3.0, 1.0);
        }

        if s.b[647] {
            s.store_mul(467, 529, 480);
            s.copy_ad(468, 151);
            s.store_div(469, 468, 467);
        }

        s.b[650] = (s.v[469] < (-100.0));
        s.v[650] = if s.b[650] { 1.0 } else { 0.0 };

        if (s.b[647] && s.b[650]) {
            s.store_div_scaled_inputs(470, s.ad_value(396), 3.720075976e-44, s.ad_value(367), 1.0);
            s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);
        }

        s.b[651] = (s.v[469] > 100.0);
        s.v[651] = if s.b[651] { 1.0 } else { 0.0 };

        if ((s.b[647] && (!s.b[650])) && s.b[651]) {
            s.store_div_scaled_inputs(470, s.ad_value(396), 2.688117142e43, s.ad_value(367), 1.0);
            s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);
        }

        if ((s.b[647] && (!s.b[650])) && (!s.b[651])) {
            s.store_div_scaled_product_left_ad(470, A::exp(s.ad_value(469)), 396, 1.0, 367, 1.0);
            s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);
        }

        if s.b[647] {
            s.store_div_scaled_inputs(69, s.ad_value(467), 0.6931471805599453, s.ad_value(471), 1.0);
        }

        if (!s.b[647]) {
            s.store_scalar(69, 0.0);
        }

        s.b[704] = ((p.p38 >= 4.4) || (p.p63 != 0.0));
        s.v[704] = if s.b[704] { 1.0 } else { 0.0 };

        s.b[705] = (s.v[106] < 0.01);
        s.v[705] = if s.b[705] { 1.0 } else { 0.0 };

        if (s.b[704] && s.b[705]) {
            s.store_scalar(106, 0.01);
        }

        s.b[706] = (s.v[106] > 1.0);
        s.v[706] = if s.b[706] { 1.0 } else { 0.0 };

        if ((s.b[704] && (!s.b[705])) && s.b[706]) {
            s.store_scalar(106, 1.0);
            s.store_scalar(105, 0.0);
        }

        s.b[707] = (s.v[181] < 0.0);
        s.v[707] = if s.b[707] { 1.0 } else { 0.0 };

        if s.b[707] {
            s.store_scalar(181, 0.0);
            s.store_scalar(182, 0.0);
        }

        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));
        s.v[708] = if s.b[708] { 1.0 } else { 0.0 };

        if ((!s.b[707]) && s.b[708]) {
            s.store_scalar(182, 0.0);
        }

        s.b[738] = (s.v[308] < 0.0);
        s.v[738] = if s.b[738] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[738]) {
            s.store_scalar(308, 0.0);
        }

        s.b[739] = (s.v[309] < 0.0);
        s.v[739] = if s.b[739] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[739]) {
            s.store_scalar(309, 0.0);
        }

        s.b[740] = (s.v[310] < 0.0);
        s.v[740] = if s.b[740] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[740]) {
            s.store_scalar(310, 0.0);
        }

        s.b[741] = (s.v[311] < 0.0);
        s.v[741] = if s.b[741] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[741]) {
            s.store_scalar(311, 0.0);
        }

        s.b[742] = (s.v[312] < 0.0);
        s.v[742] = if s.b[742] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[742]) {
            s.store_scalar(312, 0.0);
        }

        s.b[743] = (s.v[313] < 0.0);
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((p.p63 != 0.0) && s.b[743]) {
            s.store_scalar(313, 0.0);
        }

        s.v[410] = 0.0;

        s.b[805] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[805] = if s.b[805] { 1.0 } else { 0.0 };

        s.b[806] = ((p.p35 != 0.0) && (!true));
        s.v[806] = if s.b[806] { 1.0 } else { 0.0 };

        s.b[807] = true;
        s.v[807] = if s.b[807] { 1.0 } else { 0.0 };

        if ((s.b[805] && s.b[806]) && s.b[807]) {
            s.store_voltage(410, ctx, nodes, Some(5), None);
        }

        s.b[808] = true;
        s.v[808] = if s.b[808] { 1.0 } else { 0.0 };

        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {
            s.store_voltage(410, ctx, nodes, Some(4), None);
        }

        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {
            s.store_voltage(410, ctx, nodes, Some(6), None);
        }

        if (s.b[805] && (!s.b[806])) {
            s.store_voltage(410, ctx, nodes, Some(6), None);
        }

        s.store_offset(409, 410, s.v[409]);

        s.store_scale(411, 409, 1.0 / (s.v[429]));

        s.store_offset(430, 411, (-1.0));

        s.v[1133] = 0.0;

        s.v[1134] = 0.0;

        s.v[1135] = 0.0;

        s.v[1136] = 0.0;

        s.v[1131] = 0.0;

        s.v[1121] = 0.0;

        s.v[855] = 0.0;

        s.v[1122] = 0.0;

        s.v[1130] = 0.0;

        s.v[1127] = 0.0;

        s.v[1128] = 0.0;

        s.v[1126] = 0.0;

        s.v[1118] = 0.0;

        s.copy_ad(955, 182);

        s.copy_ad(1095, 173);

        s.copy_ad(1096, 174);

        s.b[1159] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        s.b[1160] = (p.p41 == 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1160]) {
            s.store_scale(832, 409, 8.617087e-5);
            s.store_offset(843, 409, 1108.0);
            s.store_square(848, 409);
            s.store_sub_from_scalar_ad(912, 1.16, A::div_scaled_inputs(s.ad_value(848), 0.000702, s.ad_value(843), 1.0));
            s.store_scalar(845, 0.00019230584);
            s.store_sqrt(848, 409);
            s.store_mul3_affine_lhs(846, 409, 848, 14500000000.0, 0.0, 845);
            s.store_sub_from_scalar_ad(849, 21.5565981, A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0));
        }

        s.b[1161] = (s.v[849] > (-100.0));
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if ((s.b[1159] && s.b[1160]) && s.b[1161]) {
            s.store_exp(847, 849);
        }

        if ((s.b[1159] && s.b[1160]) && (!s.b[1161])) {
            s.store_scalar(847, (((-100.0)) as f64).exp());
        }

        if (s.b[1159] && s.b[1160]) {
            s.store_mul(911, 846, 847);
        }

        if (s.b[1159] && s.b[1160]) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }

        if (s.b[1159] && s.b[1160]) {
            s.store_mul(940, 832, 843);
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_scalar(429, (p.p126 + 273.15));
            s.store_scale(832, 409, 8.617087e-5);
            s.store_scale(1104, 429, 8.617087e-5);
            s.copy_ad(1103, 394);
            s.store_sub_from_scalar_ad(912, p.p49, A::div_scaled_product_offset_denominator(s.ad_value(409), s.ad_value(409), p.p50, s.ad_value(409), p.p51, 1.0));
            s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));
            s.store_sqrt(848, 409);
            s.store_mul3_affine_lhs(846, 409, 848, p.p48, 0.0, 845);
            s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));
            s.store_mul(911, 846, 847);
        }

        if (s.b[1159] && (!s.b[1160])) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }

        if (s.b[1159] && (!s.b[1160])) {
            s.store_mul(940, 832, 843);
        }

        s.b[1162] = (s.v[109] > 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1162]) {
            if ((s.v[108] / s.v[109]) > 1e-38) {
                s.store_ln_div(843, 108, 109);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }

        if (s.b[1159] && s.b[1162]) {
            s.store_scaled_mul(941, 832, 843, (-p.p37));
        }

        if (s.b[1159] && (!s.b[1162])) {
            if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                s.store_ln_ad(843, A::div_scaled_product_by_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), s.ad_value(911), 1.0));
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }

        if (s.b[1159] && (!s.b[1162])) {
            s.store_scaled_mul(941, 832, 843, (-p.p37));
        }

        if s.b[1159] {
            s.store_mul_scaled_ad_rhs(942, 832, 2.0, {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1159] {
            s.store_sqrt(943, 942);
            s.store_mul_sqrt_ad_lhs(944, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 943);
            s.store_div_ad_lhs(1140, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(417), 1.602176462e-19, s.ad_value(108)), (1000000.0 * 1.0 / (2.0))), 943);
            s.store_sqrt_ad(844, A::mul3(A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415), s.ad_value(944)));
            s.store_exp_ad(843, A::div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));
            s.store_add_scaled_product_indices(1141, 843, 1.0, 843, 843, 2.0);
            s.store_exp_ad(843, A::div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));
            s.store_add_scaled_product_indices(845, 843, 1.0, 843, 843, 2.0);
            s.store_add_scaled_product_indices(1142, 193, 1.0, 192, 845, 1.0);
            s.copy_ad(49, 832);
            s.store_mul_div_from_scalar_lhs(847, 1.115, 832, 430);
            s.store_div_scaled_product_indices(850, 256, 847, 1.0, 300, 1.0);
        }

        s.b[1163] = (s.v[850] > 100.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1163]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1164] = (s.v[850] < (-100.0));
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {
            s.store_exp(843, 850);
        }

        s.b[1165] = (s.v[256] == s.v[257]);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1165]) {
            s.copy_ad(844, 843);
        }

        if (s.b[1159] && (!s.b[1165])) {
            s.store_div_scaled_product_indices(850, 257, 847, 1.0, 300, 1.0);
        }

        s.b[1166] = (s.v[850] > 100.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1165])) && s.b[1166]) {
            s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1167] = (s.v[850] < (-100.0));
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && s.b[1167]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && (!s.b[1167])) {
            s.store_exp(844, 850);
        }

        if s.b[1159] {
            s.store_div_scaled_product_indices(850, 258, 847, 1.0, 302, 1.0);
        }

        s.b[1168] = (s.v[850] > 100.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1168]) {
            s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1169] = (s.v[850] < (-100.0));
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1168])) && s.b[1169]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1168])) && (!s.b[1169])) {
            s.store_exp(845, 850);
        }

        if s.b[1159] {
            s.store_mul(972, 355, 843);
            s.store_mul(949, 306, 843);
            s.store_mul(947, 308, 844);
            s.store_mul(951, 310, 845);
            s.store_mul(850, 259, 430);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1170] = (s.v[850] > 100.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1170]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1171] = (s.v[850] < (-100.0));
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1170])) && s.b[1171]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1170])) && (!s.b[1171])) {
            s.store_exp(843, 850);
        }

        if s.b[1159] {
            s.store_mul(953, 312, 843);
            s.store_div_scaled_product_indices(850, 256, 847, 1.0, 301, 1.0);
        }

        s.b[1172] = (s.v[850] > 100.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1172]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1173] = (s.v[850] < (-100.0));
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1172])) && s.b[1173]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1172])) && (!s.b[1173])) {
            s.store_exp(843, 850);
        }

        s.b[1174] = (s.v[256] == s.v[260]);
        s.v[1174] = if s.b[1174] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1174]) {
            s.copy_ad(844, 843);
        }

        if (s.b[1159] && (!s.b[1174])) {
            s.store_div_scaled_product_indices(850, 260, 847, 1.0, 301, 1.0);
        }

        s.b[1175] = (s.v[850] > 100.0);
        s.v[1175] = if s.b[1175] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1174])) && s.b[1175]) {
            s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1176] = (s.v[850] < (-100.0));
        s.v[1176] = if s.b[1176] { 1.0 } else { 0.0 };

        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && s.b[1176]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && (!s.b[1176])) {
            s.store_exp(844, 850);
        }

        if s.b[1159] {
            s.store_div_scaled_product_indices(850, 261, 847, 1.0, 303, 1.0);
        }

        s.b[1177] = (s.v[850] > 100.0);
        s.v[1177] = if s.b[1177] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1177]) {
            s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1178] = (s.v[850] < (-100.0));
        s.v[1178] = if s.b[1178] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_exp(845, 850);
        }

        if s.b[1159] {
            s.store_mul(973, 356, 843);
            s.store_mul(950, 307, 843);
            s.store_mul(948, 309, 844);
            s.store_mul(952, 311, 845);
            s.store_mul(850, 262, 430);
        }

        s.b[1179] = (s.v[850] > 100.0);
        s.v[1179] = if s.b[1179] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1179]) {
            s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1180] = (s.v[850] < (-100.0));
        s.v[1180] = if s.b[1180] { 1.0 } else { 0.0 };

        if ((s.b[1159] && (!s.b[1179])) && s.b[1180]) {
            s.store_scalar(843, 3.720075976e-44);
        }

        if ((s.b[1159] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_exp(843, 850);
        }

        if s.b[1159] {
            s.store_mul(954, 313, 843);
            s.store_mul_pow_ad_rhs(945, 144, s.ad_value(411), s.ad_value(145));
        }

        s.b[1181] = (p.p38 < 4.2);
        s.v[1181] = if s.b[1181] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1181]) {
            s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(411), p.p238, 1.0), 1e-9);
        }

        if (s.b[1159] && (!s.b[1181])) {
            s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(430), p.p238, 1.0), 1e-9);
        }

        if s.b[1159] {
            s.store_scale(850, 235, p.p235);
            s.store_div(960, 850, 961);
            s.store_scale(847, 51, p.p235);
            s.store_div(959, 847, 961);
            s.store_offset(845, 959, 1.0);
            s.store_offset(850, 960, 1.0);
            s.store_div(843, 845, 850);
            s.store_mul(945, 945, 843);
            s.store_add_scaled_product_indices(946, 101, 1.0, 102, 430, (-1.0));
            s.store_offset_mul(845, 45, 959, 1.0);
            s.store_offset_mul(850, 45, 960, 1.0);
            s.store_div(843, 845, 850);
            s.store_mul(946, 946, 843);
        }

        s.b[1182] = (p.p429 != 1.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1159] && s.b[1182]) {
            s.store_div_scaled_add_product(955, s.ad_value(181), 1.0, s.ad_value(186), s.ad_value(430), 1.0, s.ad_value(159), 1.0);
            s.store_scalar(1095, 0.0);
            s.store_scalar(1096, 0.0);
        }

        if (s.b[1159] && (!s.b[1182])) {
            s.store_scalar(955, 0.0);
            s.store_scale(1094, 159, p.p3);
            s.store_mul(853, 186, 430);
            s.store_add(844, 169, 853);
            s.store_offset(845, 853, p.p140);
            s.store_div(1095, 844, 1094);
            s.store_add(850, 170, 853);
            s.store_offset(847, 853, p.p139);
            s.store_div(1096, 850, 1094);
        }

        if s.b[1159] {
            s.store_add_scaled_product_indices(956, 153, 1.0, 139, 430, 1.0);
            s.store_add_scaled_product_indices(957, 154, 1.0, 141, 430, 1.0);
            s.store_add_scaled_product_indices(958, 155, 1.0, 143, 430, 1.0);
        }

        if (!s.b[1159]) {
            s.copy_ad(940, 115);
            s.copy_ad(941, 160);
            s.copy_ad(942, 118);
            s.copy_ad(943, 339);
            s.copy_ad(944, 340);
            s.copy_ad(912, 395);
            s.copy_ad(1140, 367);
            s.copy_ad(1141, 342);
            s.copy_ad(1142, 343);
            s.copy_ad(949, 161);
            s.copy_ad(950, 162);
            s.copy_ad(947, 163);
            s.copy_ad(948, 164);
            s.copy_ad(951, 165);
            s.copy_ad(952, 166);
            s.copy_ad(953, 167);
            s.copy_ad(954, 168);
            s.copy_ad(972, 357);
            s.copy_ad(973, 358);
            s.copy_ad(945, 404);
            s.copy_ad(946, 407);
            s.copy_ad(956, 138);
            s.copy_ad(957, 140);
            s.copy_ad(958, 142);
        }

        s.b[1183] = (param_given[90] || param_given[94]);
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        s.b[1184] = (!param_given[90]);
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if (s.b[1183] && s.b[1184]) {
            s.store_scalar(120, 0.53);
        }

        s.b[1185] = (!param_given[94]);
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1183] && s.b[1185]) {
            s.store_scalar(124, (-0.0186));
        }

        s.b[1186] = (!param_given[87]);
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if (((!s.b[1183]) && s.b[1186]) && (p.p41 != 0.0)) {
            s.store_scaled_div_from_scalar_ad(843, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);
        }

        if (((!s.b[1183]) && s.b[1186]) && (p.p41 == 0.0)) {
            s.store_scalar(843, 0.00077348);
        }

        if ((!s.b[1183]) && s.b[1186]) {
            s.store_add_scaled_product_indices(114, 942, 1.0, 843, 108, (-(s.v[117] * s.v[117])));
        }

        s.b[1187] = (s.v[114] > 0.0);
        s.v[1187] = if s.b[1187] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1187]) {
            s.store_neg(114, 114);
        }

        s.b[1188] = (s.v[116] > 0.0);
        s.v[1188] = if s.b[1188] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1188]) {
            s.store_neg(116, 116);
        }

        s.b[1189] = (!param_given[85]);
        s.v[1189] = if s.b[1189] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1189]) {
            s.store_div_scaled_product_right_ad(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);
        }

        s.b[1190] = (!param_given[86]);
        s.v[1190] = if s.b[1190] { 1.0 } else { 0.0 };

        if ((!s.b[1183]) && s.b[1190]) {
            s.store_div_scaled_product_right_ad(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);
        }

        if (!s.b[1183]) {
            s.store_sub(843, 112, 113);
            s.store_sub_ad_lhs(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);
            s.store_mul_sub_ad_rhs(845, 943, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), s.ad_value(943));
            s.store_div_scaled_product_add_scaled_denominator_indices(846, 843, 844, 1.0, 845, 2.0, 116, 1.0, 1.0);
            s.store_add_scaled_inputs3_indices(402, 402, 1.0, 124, (-1.0), 846, 1.0);
            s.store_add_scaled_product_right_ad(120, 113, 1.0, 402, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), (-2.0));
        }

        s.store_offset(843, 265, s.v[328]);

        s.b[1191] = (s.v[843] < 1e-8);
        s.v[1191] = if s.b[1191] { 1.0 } else { 0.0 };

        if s.b[1191] {
            s.store_scalar(843, 1e-8);
        }

        s.store_mul_offset_ad_rhs(405, 120, A::div(s.ad_value(264), s.ad_value(843)), 1.0);

        s.store_scale(376, 405, (p.p66 * 1.0 / (p.p67)));

        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));

        s.b[1192] = (!param_given[109]);
        s.v[1192] = if s.b[1192] { 1.0 } else { 0.0 };

        s.b[1193] = (param_given[108] || param_given[107]);
        s.v[1193] = if s.b[1193] { 1.0 } else { 0.0 };

        if (s.b[1192] && s.b[1193]) {
            s.store_add_scaled_product_value_ad(406, A::add_scaled_inputs4(s.ad_value(406), 1.0, s.ad_value(152), (-1.0), s.ad_value(408), p.p37, s.ad_value(942), -1.0), 1.0, 405, 943, (-1.0));
        }

        if (s.b[1192] && (!s.b[1193])) {
        }

        s.b[1194] = (!param_given[108]);
        s.v[1194] = if s.b[1194] { 1.0 } else { 0.0 };

        if s.b[1194] {
            s.store_add_scaled_inputs_product_indices(408, 406, p.p37, 942, p.p37, 405, 943, p.p37);
        }

        s.b[1195] = (p.p38 < 4.2);
        s.v[1195] = if s.b[1195] { 1.0 } else { 0.0 };

        if s.b[1195] {
            s.copy_ad(1095, 173);
            s.copy_ad(1140, 367);
            s.copy_ad(1141, 342);
            s.copy_ad(1142, 343);
        }

        s.b[1196] = (p.p62 == 4.0);
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1195] && s.b[1196]) {
            s.copy_ad(956, 138);
            s.copy_ad(958, 142);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(819, ctx, nodes, Some(7), Some(8), p.p37);

        s.store_scaled_voltage(818, ctx, nodes, Some(5), Some(8), p.p37);

        s.store_scaled_voltage(821, ctx, nodes, Some(9), Some(8), p.p37);

        s.store_scaled_voltage(897, ctx, nodes, Some(3), Some(8), p.p37);

        s.store_scaled_voltage(1114, ctx, nodes, Some(9), Some(4), p.p37);

        s.store_scaled_voltage(1087, ctx, nodes, Some(11), Some(8), p.p37);

        s.store_scaled_voltage(1088, ctx, nodes, Some(12), Some(7), p.p37);

        s.store_scaled_voltage(1018, ctx, nodes, Some(10), Some(8), p.p37);

        s.store_sub(817, 818, 819);

        s.store_sub(820, 821, 819);

        s.store_sub(898, 897, 819);

        s.store_sub(1019, 1018, 819);

        s.b[1197] = (s.v[819] >= 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if s.b[1197] {
            s.store_scalar(398, 1.0);
            s.copy_ad(822, 819);
            s.copy_ad(823, 821);
            s.copy_ad(824, 818);
            s.copy_ad(900, 817);
            s.copy_ad(901, 897);
            s.copy_ad(1110, 820);
            s.copy_ad(1143, 282);
            s.store_add_scaled_product_indices(1144, 283, 1.0, 284, 430, 1.0);
            s.copy_ad(1145, 285);
            s.copy_ad(1146, 286);
            s.copy_ad(1147, 287);
            s.copy_ad(1148, 288);
            s.copy_ad(1149, 289);
            s.copy_ad(1150, 290);
            s.store_add_scaled_product_indices(1151, 291, 1.0, 292, 430, 1.0);
            s.copy_ad(1152, 293);
            s.copy_ad(1153, 294);
            s.copy_ad(1154, 295);
            s.copy_ad(1155, 296);
            s.copy_ad(1156, 297);
        }

        if (!s.b[1197]) {
            s.store_scalar(398, (-1.0));
            s.store_neg(822, 819);
            s.copy_ad(823, 820);
            s.copy_ad(824, 817);
            s.copy_ad(900, 818);
            s.copy_ad(901, 898);
            s.copy_ad(1110, 821);
            s.copy_ad(1143, 290);
            s.store_add_scaled_product_indices(1144, 291, 1.0, 292, 430, 1.0);
            s.copy_ad(1145, 293);
            s.copy_ad(1146, 294);
            s.copy_ad(1147, 295);
            s.copy_ad(1148, 296);
            s.copy_ad(1149, 297);
            s.copy_ad(1150, 282);
            s.store_add_scaled_product_indices(1151, 283, 1.0, 284, 430, 1.0);
            s.copy_ad(1152, 285);
            s.copy_ad(1153, 286);
            s.copy_ad(1154, 287);
            s.copy_ad(1155, 288);
            s.copy_ad(1156, 289);
        }

        s.store_sub(902, 901, 941);

        s.v[913] = s.v[392];

        s.store_add(843, 406, 942);

        s.b[1198] = (p.p41 == 0.0);
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if s.b[1198] {
            s.copy_ad(418, 417);
        }

        if (!s.b[1198]) {
            s.store_scalar(418, (p.p60 * 8.85418e-12));
        }

        s.b[1199] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0));
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if s.b[1199] {
            s.store_div_scaled_product_denominator_ad(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);
            s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(823), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);
            s.store_mul_offset_rhs(845, 844, 847, (-1.0));
            s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);
            s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));
            s.store_sqrt_square_offset(849, 850, 0.224);
            s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);
            s.store_sub(825, 823, 848);
        }

        if (!s.b[1199]) {
            s.copy_ad(825, 823);
        }

        s.b[1200] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0));
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if s.b[1200] {
            s.store_div_scaled_product_denominator_ad(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);
            s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(1110), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);
            s.store_mul_offset_rhs(845, 844, 847, (-1.0));
            s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);
            s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));
            s.store_sqrt_square_offset(849, 850, 0.224);
            s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);
            s.store_sub(1111, 1110, 848);
        }

        if (!s.b[1200]) {
            s.copy_ad(1111, 1110);
        }

        s.copy_ad(1125, 823);

        s.v[892] = s.v[327];

        s.b[1201] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1201] = if s.b[1201] { 1.0 } else { 0.0 };

        if s.b[1201] {
            s.store_scale(832, 409, 8.617087e-5);
        }

        if (!s.b[1201]) {
            s.copy_ad(832, 49);
        }

        s.store_sub(834, 940, 942);

        s.b[1202] = (s.v[37] == 0.0);
        s.v[1202] = if s.b[1202] { 1.0 } else { 0.0 };

        if s.b[1202] {
            s.copy_ad(1033, 824);
            s.copy_ad(1048, 824);
        }

        s.b[1203] = (p.p432 == 0.0);
        s.v[1203] = if s.b[1203] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1203]) {
            s.store_div_scaled_inputs(843, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_div_scaled_inputs(846, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_add_scaled_inputs4_indices(1036, 942, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_div_scaled_inputs(846, s.ad_value(223), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);
            s.store_div_scaled_inputs2(844, s.ad_value(221), 1.0, s.ad_value(848), (-1.0), s.ad_value(843), 1.0);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(847, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_add_scaled_product_indices(1031, 845, 1.0, 847, 1036, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1203])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_div_scaled_inputs(844, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_div_scaled_inputs(847, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(942), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1036, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1031, 1036, 850);
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1036, 1031, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);
            s.store_add_scaled_product_indices(1032, 1031, 1.0, 846, 847, (-0.5));
            s.store_offset(844, 942, (-0.02));
            s.store_offset_sub(845, 844, 1032, (-0.005));
            s.store_sqrt_square_offset(846, 845, (4.0 * 0.005));
            s.store_add_scaled_inputs3_indices(1032, 844, 1.0, 845, (-0.5), 846, (-0.5));
            s.store_sub(827, 942, 1032);
            s.store_sqrt(828, 827);
            s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);
            s.store_sqrt(846, 864);
            s.store_mul(843, 131, 1032);
        }

        s.b[1204] = (s.v[843] >= (-0.5));
        s.v[1204] = if s.b[1204] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1204]) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1204])) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(865, 397, 846, 844);
            s.store_mul(843, 134, 1032);
        }

        s.b[1205] = (s.v[843] >= (-0.5));
        s.v[1205] = if s.b[1205] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1205]) {
            s.store_offset(844, 843, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1205])) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(866, 397, 846, 844);
            s.store_div_scaled_inputs(843, s.ad_value(130), ((-0.5) * s.v[892]), s.ad_value(865), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1206] = (s.v[843] > (-100.0));
        s.v[1206] = if s.b[1206] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1206]) {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1206])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);
            s.store_add_scaled_value_products(846, s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(1032), 1.0, s.ad_value(98), s.ad_value(822), 1.0);
            s.store_div_scaled_inputs2(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, s.ad_value(99), 1.0, s.ad_value(396), 1.0);
        }

        s.b[1207] = (s.v[847] >= (-0.5));
        s.v[1207] = if s.b[1207] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1207]) {
            s.store_offset(831, 847, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1207])) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);
        }

        s.b[1208] = (s.v[378] > 0.0);
        s.v[1208] = if s.b[1208] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1209] = (s.v[843] < (-100.0));
        s.v[1209] = if s.b[1209] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && s.b[1208]) && s.b[1209]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (((!s.b[1202]) && s.b[1208]) && (!s.b[1209])) {
            s.store_exp(845, 843);
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul(1090, 831, 847);
        }

        if ((!s.b[1202]) && (!s.b[1208])) {
            s.store_scalar(1090, 0.0);
        }

        if (!s.b[1202]) {
            s.store_mul(63, 129, 868);
            s.store_mul(867, 63, 834);
            s.store_div_scaled_inputs(843, s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892])), s.ad_value(866), 1.0);
        }

        s.b[1210] = (s.v[843] > (-100.0));
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1210]) {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1210])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_mul(843, 132, 845);
            s.store_mul(904, 843, 834);
            s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);
            s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1032, 1.0);
            s.store_add_scaled_product_value_ad(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);
            s.store_div_scaled_product_offset_denominator(870, s.ad_value(415), s.ad_value(942), 1.0, s.ad_value(127), s.v[328], 1.0);
            s.store_add_scaled_product_indices(846, 400, 1.0, 188, 1032, 1.0);
        }

        s.b[1211] = (s.v[846] < 0.0001);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1211]) {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(873, 846, 1141, 822);
            s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1032, 1.0);
        }

        s.b[1212] = (s.v[846] < 0.0001);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1212]) {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        if (!s.b[1202]) {
            s.store_mul3_lhs(1070, 846, 1141, 822);
            s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);
            s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);
            s.store_div_scaled_product_offset_denominator(1091, s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, s.ad_value(843), 1.0, 1.0);
        }

        if (!s.b[1202]) {
            let assign15050_ad_e13615: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0);
            s.store_sub_ad_lhs(1037, A::add_scaled_inputs4(assign15050_ad_e13615, 1.0, s.ad_value(903), 1.0, s.ad_value(873), -1.0, s.ad_value(1090), -1.0), 1091);
        }

        if (!s.b[1202]) {
            let assign15060_ad_e13656: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0);
            s.store_sub_ad_lhs(1052, A::add_scaled_inputs4(assign15060_ad_e13656, 1.0, s.ad_value(903), 1.0, s.ad_value(1070), -1.0, s.ad_value(1090), -1.0), 1091);
        }

        if (!s.b[1202]) {
            s.store_sub(1038, 1037, 825);
            s.store_mul(853, 219, 832);
        }

        s.b[1213] = (((s.v[1038] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1213]) {
            s.store_scaled_offset_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1214] = (((s.v[1038] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1213])) && s.b[1214]) {
            s.store_scalar(1039, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1213])) && (!s.b[1214])) {
            s.store_exp_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1042, 853, A::offset(s.ad_value(1039), 1.0));
            s.store_sub(1040, 825, 1037);
        }

        s.b[1215] = (((s.v[1040] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1215]) {
            s.store_scaled_offset_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1216] = (((s.v[1040] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1215])) && s.b[1216]) {
            s.store_scalar(1041, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1215])) && (!s.b[1216])) {
            s.store_exp_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1043, 853, A::offset(s.ad_value(1041), 1.0));
            s.store_mul_product3_rhs(844, 832, s.ad_value(226), s.ad_value(376), s.ad_value(832), 1.0);
            s.store_add_scaled_product_right_ad(845, 1043, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);
            s.store_offset_div_scaled_product(843, s.ad_value(1043), s.ad_value(845), 1.0, s.ad_value(844), 1.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_add_scaled_product_right_ad(1034, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }

        if (!s.b[1202]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);
            s.store_add_scaled_product_indices(1035, 1034, 1.0, 843, 1042, (-1.0));
        }

        s.b[1217] = (p.p432 == 0.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1217]) {
            s.store_div_scaled_inputs(843, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_div_scaled_inputs(846, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_add_scaled_inputs4_indices(1036, 1035, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_div_scaled_inputs(846, s.ad_value(223), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);
            s.store_div_scaled_inputs2(844, s.ad_value(221), 1.0, s.ad_value(848), (-1.0), s.ad_value(843), 1.0);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_add_scaled_product_indices(1031, 845, 1.0, 843, 1036, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1217])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_div_scaled_inputs(844, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_div_scaled_inputs(847, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(1035), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1036, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1031, 1036, 850);
        }

        s.b[1218] = (s.v[37] == 2.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1218]) {
            s.store_offset(1030, 1031, 0.02);
            s.store_offset(824, 1031, 0.02);
        }

        if ((!s.b[1202]) && (!s.b[1218])) {
            s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1031), 0.02), (-0.01));
            s.store_sqrt_square_offset(845, 844, 0.0001);
            s.store_add_scaled_inputs3_offset_indices(1030, 1031, 1.0, 844, 0.5, 845, 0.5, 0.02);
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1036, 1030, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);
            s.store_add_scaled_product_indices(1033, 1030, 1.0, 846, 847, (-0.5));
            s.store_sub(1060, 1052, 825);
            s.store_mul(853, 219, 832);
        }

        s.b[1219] = (((s.v[1060] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1219]) {
            s.store_scaled_offset_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1220] = (((s.v[1060] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1219])) && s.b[1220]) {
            s.store_scalar(1061, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1219])) && (!s.b[1220])) {
            s.store_exp_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1064, 853, A::offset(s.ad_value(1061), 1.0));
            s.store_sub(1062, 825, 1052);
        }

        s.b[1221] = (((s.v[1062] - s.v[220]) / s.v[853]) > 100.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1221]) {
            s.store_scaled_offset_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1222] = (((s.v[1062] - s.v[220]) / s.v[853]) < (-100.0));
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if (((!s.b[1202]) && (!s.b[1221])) && s.b[1222]) {
            s.store_scalar(1063, 3.720075976e-44);
        }

        if (((!s.b[1202]) && (!s.b[1221])) && (!s.b[1222])) {
            s.store_exp_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));
        }

        if (!s.b[1202]) {
            s.store_mul_ln_ad_rhs(1065, 853, A::offset(s.ad_value(1063), 1.0));
            s.store_mul_product3_rhs(844, 832, s.ad_value(226), s.ad_value(376), s.ad_value(832), 1.0);
            s.store_add_scaled_product_right_ad(845, 1065, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);
            s.store_offset_div_scaled_product(843, s.ad_value(1065), s.ad_value(845), 1.0, s.ad_value(844), 1.0, 1.0);
        }

        if (!s.b[1202]) {
            s.store_add_scaled_product_right_ad(1049, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }

        if (!s.b[1202]) {
            s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);
            s.store_add_scaled_product_indices(1050, 1049, 1.0, 843, 1064, (-1.0));
        }

        s.b[1223] = (p.p432 == 0.0);
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1202]) && s.b[1223]) {
            s.store_div_scaled_inputs(843, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);
            s.store_mul_sub_rhs(845, 844, 940, 942);
            s.store_div_scaled_inputs(846, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_add_scaled_inputs4_indices(1051, 1050, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);
            s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);
            s.store_div_scaled_inputs(846, s.ad_value(223), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);
            s.store_div_scaled_inputs2(844, s.ad_value(221), 1.0, s.ad_value(848), (-1.0), s.ad_value(843), 1.0);
            s.store_mul(845, 844, 902);
            s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);
            s.store_add_scaled_product_indices(1047, 845, 1.0, 843, 1051, 1.0);
        }

        if ((!s.b[1202]) && (!s.b[1223])) {
            s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));
            s.store_div_scaled_inputs(844, s.ad_value(225), (-s.v[327]), s.ad_value(119), 1.0);
            s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);
            s.store_mul_add_rhs(846, 845, 822, 217);
            s.store_div_scaled_inputs(847, s.ad_value(344), 0.5, s.ad_value(393), 1.0);
            s.store_mul_ad_product_rhs(848, 393, s.ad_value(843), A::add_scaled_inputs3(s.ad_value(1050), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));
            s.store_mul3_lhs(849, 218, 843, 846);
            s.store_add(1051, 848, 849);
            s.store_scaled_mul(850, 843, 902, s.v[913]);
            s.store_add(1047, 1051, 850);
        }

        s.b[1224] = (s.v[37] == 2.0);
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if ((!s.b[1202]) && s.b[1224]) {
            s.store_offset(1046, 1047, 0.02);
            s.store_offset(824, 1047, 0.02);
        }

        if ((!s.b[1202]) && (!s.b[1224])) {
            s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1047), 0.02), (-0.01));
            s.store_sqrt_square_offset(845, 844, 0.0001);
            s.store_add_scaled_inputs3_offset_indices(1046, 1047, 1.0, 844, 0.5, 845, 0.5, 0.02);
        }

        if (!s.b[1202]) {
            s.store_offset_sub(844, 1051, 1046, (-0.005));
            s.store_sqrt_square_offset(845, 844, 2.5e-5);
            s.store_scaled_add(846, 844, 845, 0.5);
            s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);
            s.store_add_scaled_product_indices(1048, 1046, 1.0, 846, 847, (-0.5));
        }

        s.store_offset(843, 1033, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));

        s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));

        s.v[843] = 1.5;

        s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));

        s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));

        s.store_offset_add_scaled_inputs_indices(962, 844, (-0.5), 846, (-0.5), s.v[843]);

        s.store_scale(843, 942, 0.95);

        s.store_offset_sub(844, 843, 962, (-0.002));

        s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);

        s.store_add_scaled_inputs3_indices(841, 843, 1.0, 844, (-0.5), 845, (-0.5));

        s.store_offset(843, 1048, ((5.0) + ((-0.001))));

        s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));

        s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));

        s.v[843] = 1.5;

        s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));

        s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));

        s.store_offset_add_scaled_inputs_indices(1045, 844, (-0.5), 846, (-0.5), s.v[843]);

        s.store_scale(843, 942, 0.95);

        s.store_offset_sub(844, 843, 1045, (-0.002));

        s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);

        s.store_add_scaled_inputs3_indices(1044, 843, 1.0, 844, (-0.5), 845, (-0.5));

        s.store_sub(827, 942, 841);

        s.store_sqrt(828, 827);

        s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);

        s.store_sqrt(846, 864);

        s.store_mul(843, 131, 841);

        s.b[1225] = (s.v[843] >= (-0.5));
        s.v[1225] = if s.b[1225] { 1.0 } else { 0.0 };

        if s.b[1225] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1225]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        s.store_mul3_lhs(865, 397, 846, 844);

        s.store_mul(843, 134, 841);

        s.b[1226] = (s.v[843] >= (-0.5));
        s.v[1226] = if s.b[1226] { 1.0 } else { 0.0 };

        if s.b[1226] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1226]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        s.store_mul3_lhs(866, 397, 846, 844);

        s.store_div_scaled_inputs(843, s.ad_value(130), ((-0.5) * s.v[892]), s.ad_value(865), 1.0);

        s.b[1227] = (s.v[843] > (-100.0));
        s.v[1227] = if s.b[1227] { 1.0 } else { 0.0 };

        if s.b[1227] {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1227]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);
        }

        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);

        s.store_add_scaled_value_products(846, s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(841), 1.0, s.ad_value(98), s.ad_value(822), 1.0);

        s.store_div_scaled_inputs2(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, s.ad_value(99), 1.0, s.ad_value(396), 1.0);

        s.b[1228] = (s.v[847] >= (-0.5));
        s.v[1228] = if s.b[1228] { 1.0 } else { 0.0 };

        if s.b[1228] {
            s.store_offset(831, 847, 1.0);
        }

        if (!s.b[1228]) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);
        }

        s.b[1229] = (s.v[378] > 0.0);
        s.v[1229] = if s.b[1229] { 1.0 } else { 0.0 };

        if s.b[1229] {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1230] = (s.v[843] < (-100.0));
        s.v[1230] = if s.b[1230] { 1.0 } else { 0.0 };

        if (s.b[1229] && s.b[1230]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (s.b[1229] && (!s.b[1230])) {
            s.store_exp(845, 843);
        }

        if s.b[1229] {
            s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);
        }

        if s.b[1229] {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1229] {
            s.store_mul(1090, 831, 847);
        }

        if (!s.b[1229]) {
            s.store_scalar(1090, 0.0);
        }

        s.store_mul(63, 129, 868);

        s.store_mul(867, 63, 834);

        s.store_div_scaled_inputs(843, s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892])), s.ad_value(866), 1.0);

        s.b[1231] = (s.v[843] > (-100.0));
        s.v[1231] = if s.b[1231] { 1.0 } else { 0.0 };

        if s.b[1231] {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1231]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        s.store_mul(843, 132, 845);

        s.store_mul(904, 843, 834);

        s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);

        s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 841, 1.0);

        s.store_add_scaled_product_value_ad(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);

        s.store_div_scaled_product_offset_denominator(870, s.ad_value(415), s.ad_value(942), 1.0, s.ad_value(127), s.v[328], 1.0);

        s.store_add_scaled_product_indices(846, 400, 1.0, 188, 841, 1.0);

        s.b[1232] = (s.v[846] < 0.0001);
        s.v[1232] = if s.b[1232] { 1.0 } else { 0.0 };

        if s.b[1232] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        s.store_mul3_lhs(873, 846, 1141, 822);

        s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_add_scaled_product_right_ad(963, 828, 1.0, 852, A::sub(s.ad_value(962), s.ad_value(841)), (-1.0));

        s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);

        s.store_div_scaled_product_offset_denominator(1091, s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, s.ad_value(843), 1.0, 1.0);

        let assign17020_ad_e15496: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(963), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(841), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(841), 1.0), s.ad_value(870), 1.0);
        s.store_sub_ad_lhs(829, A::add_scaled_inputs4(assign17020_ad_e15496, 1.0, s.ad_value(903), 1.0, s.ad_value(873), -1.0, s.ad_value(1090), -1.0), 1091);

        s.store_sub(1053, 942, 1044);

        s.store_sqrt(1054, 1053);

        s.store_div_scaled_product_indices(1055, 944, 1054, 1.0, 943, 1.0);

        s.store_sqrt(846, 1055);

        s.store_mul(843, 131, 1044);

        s.b[1233] = (s.v[843] >= (-0.5));
        s.v[1233] = if s.b[1233] { 1.0 } else { 0.0 };

        if s.b[1233] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1233]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        s.store_mul3_lhs(1056, 397, 846, 844);

        s.store_mul(843, 134, 1044);

        s.b[1234] = (s.v[843] >= (-0.5));
        s.v[1234] = if s.b[1234] { 1.0 } else { 0.0 };

        if s.b[1234] {
            s.store_offset(844, 843, 1.0);
        }

        if (!s.b[1234]) {
            s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);
        }

        s.store_mul3_lhs(1057, 397, 846, 844);

        s.store_div_scaled_inputs(843, s.ad_value(130), ((-0.5) * s.v[892]), s.ad_value(1056), 1.0);

        s.b[1235] = (s.v[843] > (-100.0));
        s.v[1235] = if s.b[1235] { 1.0 } else { 0.0 };

        if s.b[1235] {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1235]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);
        }

        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 1055, 1.0);

        s.store_add_scaled_value_products(846, s.ad_value(96), 1.0, s.ad_value(97), s.ad_value(1044), 1.0, s.ad_value(98), s.ad_value(822), 1.0);

        s.store_div_scaled_inputs2(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(1058), 1.0), 1.0, s.ad_value(99), 1.0, s.ad_value(396), 1.0);

        s.b[1236] = (s.v[847] >= (-0.5));
        s.v[1236] = if s.b[1236] { 1.0 } else { 0.0 };

        if s.b[1236] {
            s.store_offset(1059, 847, 1.0);
        }

        if (!s.b[1236]) {
            s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);
            s.store_mul_scale_offset_rhs(1059, 843, 847, 3.0, 1.0);
        }

        s.b[1237] = (s.v[378] > 0.0);
        s.v[1237] = if s.b[1237] { 1.0 } else { 0.0 };

        if s.b[1237] {
            s.store_mul_neg_lhs(843, 379, 822);
        }

        s.b[1238] = (s.v[843] < (-100.0));
        s.v[1238] = if s.b[1238] { 1.0 } else { 0.0 };

        if (s.b[1237] && s.b[1238]) {
            s.store_scalar(845, 3.720075976e-44);
        }

        if (s.b[1237] && (!s.b[1238])) {
            s.store_exp(845, 843);
        }

        if s.b[1237] {
            s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);
        }

        if s.b[1237] {
            s.store_mul_ad_rhs(847, 832, {
                if ((s.v[892] / s.v[846]) > 1e-38) {
                    A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if s.b[1237] {
            s.store_mul(1071, 1059, 847);
        }

        if (!s.b[1237]) {
            s.store_scalar(1071, 0.0);
        }

        s.store_mul(63, 129, 1058);

        s.store_mul(1067, 63, 834);

        s.store_div_scaled_inputs(843, s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892])), s.ad_value(1057), 1.0);

        s.b[1239] = (s.v[843] > (-100.0));
        s.v[1239] = if s.b[1239] { 1.0 } else { 0.0 };

        if s.b[1239] {
            s.store_exp(844, 843);
        }

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1239] {
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if (!s.b[1239]) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        s.store_mul(843, 132, 845);

        s.store_mul(1068, 843, 834);

        s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);

        s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1044, 1.0);

        s.store_add_scaled_product_value_ad(1069, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);

        s.store_div_scaled_product_offset_denominator(1066, s.ad_value(415), s.ad_value(942), 1.0, s.ad_value(127), s.v[328], 1.0);

        s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1044, 1.0);

        s.b[1240] = (s.v[846] < 0.0001);
        s.v[1240] = if s.b[1240] { 1.0 } else { 0.0 };

        if s.b[1240] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));
            s.store_mul_sub_from_scalar_lhs(846, 0.0002, 846, 852);
        }

        s.store_mul3_lhs(1070, 846, 1141, 822);

        s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);

        s.store_div_from_scalar(852, 2.2361, 943);

        s.store_add_scaled_product_right_ad(1072, 1054, 1.0, 852, A::sub(s.ad_value(1045), s.ad_value(1044)), (-1.0));

        s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);

        s.store_div_scaled_product_offset_denominator(1091, s.ad_value(391), A::offset(s.ad_value(843), (-1.0)), 1.0, s.ad_value(843), 1.0, 1.0);

        let assign17670_ad_e15953: A = A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(1072), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1044), (-1.0)), 1.0, s.ad_value(1067), (-1.0), s.ad_value(1068), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1044), 1.0), s.ad_value(1066), 1.0);
        s.store_sub_ad_lhs(1073, A::add_scaled_inputs4(assign17670_ad_e15953, 1.0, s.ad_value(1069), 1.0, s.ad_value(1070), -1.0, s.ad_value(1071), -1.0), 1091);

        s.b[1241] = (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0));
        s.v[1241] = if s.b[1241] { 1.0 } else { 0.0 };

        if s.b[1241] {
            s.store_sqrt(1007, 944);
            s.store_mul(1008, 397, 1007);
            s.store_mul(1009, 397, 1007);
            s.store_div_scaled_inputs(843, s.ad_value(130), ((-0.5) * s.v[892]), s.ad_value(1008), 1.0);
        }

        s.b[1242] = (s.v[843] > (-100.0));
        s.v[1242] = if s.b[1242] { 1.0 } else { 0.0 };

        if (s.b[1241] && s.b[1242]) {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);
        }

        if (s.b[1241] && (!s.b[1242])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);
        }

        if s.b[1241] {
            s.store_mul3_lhs(1011, 129, 1010, 834);
            s.store_div_scaled_inputs(843, s.ad_value(133), ((-0.5) * (s.v[328] * s.v[892])), s.ad_value(1009), 1.0);
        }

        s.b[1243] = (s.v[843] > (-100.0));
        s.v[1243] = if s.b[1243] { 1.0 } else { 0.0 };

        if (s.b[1241] && s.b[1243]) {
            s.store_exp(844, 843);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if (s.b[1241] && (!s.b[1243])) {
            s.store_scalar(844, 3.720075976e-44);
            s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);
        }

        if s.b[1241] {
            s.store_mul(843, 132, 845);
            s.store_mul(1012, 843, 834);
            s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);
            s.store_add_scaled_inputs(844, 121, 1.0, 122, 1.0 / (s.v[892]));
            s.store_add_scaled_product_value_ad(1013, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);
            s.store_add_ad_lhs(1014, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(1011), (-1.0), s.ad_value(1012), -1.0), 1.0, s.ad_value(125), s.ad_value(1066), 1.0), 1013);
        }

        if (!s.b[1241]) {
            s.store_scalar(1014, 0.0);
        }

        s.store_sub(830, 825, 829);

        s.store_mul(853, 831, 832);

        s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);

        s.store_div_scaled_inputs2(833, s.ad_value(151), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(830)), (-1.0), s.ad_value(853), 1.0);

        s.b[1244] = (s.v[809] > 100.0);
        s.v[1244] = if s.b[1244] { 1.0 } else { 0.0 };

        if s.b[1244] {
            s.copy_ad(875, 830);
            s.store_scalar(810, 0.0);
        }

        s.b[1245] = (s.v[833] > 100.0);
        s.v[1245] = if s.b[1245] { 1.0 } else { 0.0 };

        if ((!s.b[1244]) && s.b[1245]) {
            s.store_div_scaled_inputs2(843, s.ad_value(830), 1.0, s.ad_value(151), (-1.0), A::mul(s.ad_value(831), s.ad_value(832)), 1.0);
            s.store_exp(810, 843);
            s.store_mul_div_scaled_product_rhs(875, 810, s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0);
        }

        if ((!s.b[1244]) && (!s.b[1245])) {
            s.store_exp(810, 809);
            s.store_mul_ln_ad_rhs(844, 853, A::offset(s.ad_value(810), 1.0));
            s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(384)));
            s.store_sub_ad_rhs(845, 384, A::div_scaled_product(s.ad_value(853), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));
            s.store_div(875, 844, 845);
        }

        s.store_add_scaled_inputs(890, 875, 1.0, 832, 2.0);

        s.copy_ad(72, 875);

        s.b[1246] = (s.v[385] <= 0.0);
        s.v[1246] = if s.b[1246] { 1.0 } else { 0.0 };

        if s.b[1246] {
            s.store_scalar(1092, 1.0);
        }

        if (!s.b[1246]) {
            s.store_div_scaled_inputs(852, s.ad_value(385), ((s.v[892]) as f64).sqrt(), s.ad_value(890), 1.0);
            s.store_div_from_scalar_offset_input(1092, 1.0, 852, 1.0);
        }

        s.store_sub(852, 828, 943);

        s.store_sub_from_scalar_ad(893, s.v[328], A::add_scaled_products(s.ad_value(197), s.ad_value(875), (2.0 - p.p22), s.ad_value(198), s.ad_value(852), (2.0 - p.p22)));

        s.b[1247] = (s.v[893] < 2e-8);
        s.v[1247] = if s.b[1247] { 1.0 } else { 0.0 };

        if s.b[1247] {
            s.store_div_from_scalar_sub_from_scalar_ad(843, 1.0, 6e-8, A::scale(s.ad_value(893), 2.0));
            s.store_mul_sub_from_scalar_lhs_scaled_output(893, 4e-8, 893, 843, 2e-8);
        }

        s.b[1248] = (p.p429 == 1.0);
        s.v[1248] = if s.b[1248] { 1.0 } else { 0.0 };

        if s.b[1248] {
            s.store_scalar(887, 0.0);
        }

        if (!s.b[1248]) {
            s.store_add_scaled_products_indices(843, 183, 875, 1.0, 184, 852, 1.0);
        }

        s.b[1249] = (s.v[843] >= (-0.9));
        s.v[1249] = if s.b[1249] { 1.0 } else { 0.0 };

        if ((!s.b[1248]) && s.b[1249]) {
            s.store_mul_offset_rhs(887, 955, 843, 1.0);
        }

        if ((!s.b[1248]) && (!s.b[1249])) {
            s.store_div_from_scalar_offset_scaled_input(844, 1.0, 843, 20.0, 17.0);
            s.store_mul_ad_product_lhs(887, s.ad_value(955), A::offset(s.ad_value(843), 0.8), 844);
        }

        s.store_offset_scaled(1101, 430, p.p137, p.p135);

        s.store_offset_scaled(1102, 430, p.p138, p.p136);

        s.b[1250] = (p.p429 == 2.0);
        s.v[1250] = if s.b[1250] { 1.0 } else { 0.0 };

        if s.b[1250] {
            s.store_add_ad_lhs(887, A::add_scaled_inputs4(s.ad_value(61), 1.0, s.ad_value(887), 1.0, s.ad_value(60), 1.0, s.ad_value(1102), 1.0), 1101);
        }

        s.b[1251] = (s.v[103] == 0.0);
        s.v[1251] = if s.b[1251] { 1.0 } else { 0.0 };

        if s.b[1251] {
            s.store_scalar(860, 1.0);
            s.store_scalar(861, 1.0);
        }

        if (!s.b[1251]) {
            s.store_mul(853, 107, 962);
        }

        s.b[1252] = (s.v[853] >= (-0.5));
        s.v[1252] = if s.b[1252] { 1.0 } else { 0.0 };

        if ((!s.b[1251]) && s.b[1252]) {
            s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);
        }

        if ((!s.b[1251]) && (!s.b[1252])) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product_indices(854, 964, 1.0, 855, 853, 1.0);
        }

        if (!s.b[1251]) {
            s.store_add(853, 942, 266);
            s.store_div_scaled_product_indices(964, 962, 854, 1.0, 853, 1.0);
        }

        s.b[1253] = (s.v[964] < 0.5);
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if ((!s.b[1251]) && s.b[1253]) {
            s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));
        }

        if ((!s.b[1251]) && (!s.b[1253])) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_scaled_input(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 854, 0.5);
            s.store_add_scaled_product_indices(965, 855, 1.0, 854, 964, 1.0);
        }

        if (!s.b[1251]) {
            s.store_div_scaled_product_denominator_ad(853, 376, 1089, 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0);
            s.store_mul(844, 853, 965);
            s.store_sqrt_mul(852, 242, 864);
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
            s.store_div_from_scalar(848, s.v[892], 869);
            s.store_mul(870, 103, 848);
            s.store_offset(871, 200, s.v[328]);
            s.store_div(872, 199, 871);
            s.store_add(845, 870, 872);
            s.store_square(849, 848);
            s.store_mul(850, 848, 849);
            s.store_offset_mul(861, 844, 845, 1.0);
            s.store_mul3_lhs(851, 104, 103, 850);
            s.store_mul_neg_lhs(879, 844, 851);
            s.store_add_scaled_product_indices(860, 861, 1.0, 879, 875, 1.0);
        }

        s.b[1254] = (s.v[861] < 0.01);
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if s.b[1254] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(861), 200.0));
            s.store_mul_sub_from_scalar_lhs(861, 0.02, 861, 852);
        }

        s.b[1255] = (s.v[860] < 0.01);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if s.b[1255] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(860), 200.0));
            s.store_mul_sub_from_scalar_lhs(860, 0.02, 860, 852);
        }

        s.copy_ad(74, 860);

        s.b[1256] = (s.v[103] == 0.0);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        if s.b[1256] {
            s.store_scalar(1074, 1.0);
        }

        if (!s.b[1256]) {
            s.store_mul(853, 107, 1045);
        }

        s.b[1257] = (s.v[853] >= (-0.5));
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((!s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar_offset_input(854, 1.0, 853, 1.0);
        }

        if ((!s.b[1256]) && (!s.b[1257])) {
            s.store_scalar(855, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));
            s.store_offset_scaled(964, 855, 0.5, (1.0 / (1.0 - 0.5)));
            s.store_add_scaled_product_indices(854, 964, 1.0, 855, 853, 1.0);
        }

        if (!s.b[1256]) {
            s.store_add(853, 942, 266);
            s.store_div_scaled_product_indices(964, 1045, 854, 1.0, 853, 1.0);
        }

        s.b[1258] = (s.v[964] < 0.5);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        if ((!s.b[1256]) && s.b[1258]) {
            s.store_div_from_scalar_sqrt_ad(965, 1.0, A::sub_from_scalar(1.0, s.ad_value(964)));
        }

        if ((!s.b[1256]) && (!s.b[1258])) {
            s.store_scalar(854, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));
            s.store_sub_from_scalar_scaled_input(855, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 854, 0.5);
            s.store_add_scaled_product_indices(965, 855, 1.0, 854, 964, 1.0);
        }

        if (!s.b[1256]) {
            s.store_div_scaled_product_denominator_ad(853, 376, 1089, 0.5, A::sqrt(A::add(s.ad_value(942), s.ad_value(266))), 1.0);
            s.store_mul(844, 853, 965);
            s.store_sqrt_mul(852, 242, 1055);
            s.store_offset_scaled(869, 852, 2.0, s.v[892]);
            s.store_div_from_scalar(848, s.v[892], 869);
            s.store_mul(870, 103, 848);
            s.store_offset(871, 200, s.v[328]);
            s.store_div(872, 199, 871);
            s.store_add(845, 870, 872);
            s.store_square(849, 848);
        }

    }
}
