#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1402]) && s.b[1420]) {s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 917, (-1.0), 920, (-1.0), 919, (-1.0));}
        if ((!s.b[1402]) && (!s.b[1420])) {s.store_scalar(938, 0.0);s.store_scalar(937, 0.0);s.store_scalar(920, 0.0);s.store_scalar(917, 0.0);s.store_scalar(919, 0.0);s.store_scalar(918, 0.0);s.store_scalar(916, 0.0);}
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
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_sub(843, 853, 323);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {s.store_square(844, 843);s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));}
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
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
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
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);s.store_square(843, 843);s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);s.store_square(848, 848);s.store_offset_scaled(1487, 848, (((p.p227 * s.v[327])) * (p.p229)), p.p229);s.store_offset_scaled(1488, 848, (((p.p228 * s.v[327])) * (p.p230)), p.p230);}
        s.b[1509] = (s.v[1488] > 0.9);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if ((s.b[1506] && (!s.b[1505])) && s.b[1509]) {s.store_scalar(1488, 0.9);}
        s.b[1510] = (s.v[1488] > (0.9 * s.v[1487]));s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((s.b[1506] && (!s.b[1505])) && s.b[1510]) {s.store_scale(1488, 1487, 0.9);}
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_product_mixed_iia(844, 84, 1.0, 1487, A::add(s.ad_value(83), s.ad_value(85)), 1.0);s.store_div_scaled_product_indices(845, 844, 844, 1.0, 78, 1.0);}
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_sub_from_scalar_scaled_mul(1491, 1.0, 77, 76, 1.0);s.store_sub_from_scalar(843, 1.0, 1491);s.store_offset(844, 1491, 1.0);s.store_add_mixed_ia(845, 844, A::div_scaled_product_offset_denominator(s.ad_value(74), s.ad_value(49), 2.0, s.ad_value(72), 1e-10, 1.0));s.store_offset_scaled_div(1495, 77, 838, s.v[892], s.v[892]);s.store_div_from_scalar(849, s.v[892], 1495);s.store_mul_add_scaled_inputs_rhs_mixed_ia(1492, 849, 844, 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(845), 6.0), 1.0);s.store_square(846, 845);s.store_square(847, 843);s.store_square(848, 846);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_div_scaled_inputs3(1493, A::div(s.ad_value(844), s.ad_value(846)), 1.0, A::div_scaled_product(A::add_scaled_inputs(s.ad_value(844), 5.0, s.ad_value(845), 1.0), s.ad_value(847), 1.0, s.ad_value(848), 15.0), (-1.0), A::div_scaled_product_by_product(s.ad_value(847), s.ad_value(847), 1.0, s.ad_value(848), s.ad_value(845), 9.0), 1.0, A::mul3_scaled_output(s.ad_value(849), s.ad_value(849), s.ad_value(849), 6.0), 1.0);s.store_div(850, 843, 845);s.store_div_scaled_add_product_mixed_iaii(1494, 850, 1.0, A::square(s.ad_value(850)), 850, 0.3333333333333333, 849, 6.0);s.store_div(851, 72, 838);s.store_square(851, 851);s.store_offset_scaled(1490, 851, (((p.p224 * s.v[892])) * (p.p225)), p.p225);s.store_mul_scale_offset_mixed_ai(1498, A::div(s.ad_value(1494), A::sqrt(A::mul(s.ad_value(1492), s.ad_value(1493)))), 1490, 2.5316, 0.0);}
        s.b[1512] = (s.v[1498] > 1.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1512]) {s.store_scalar(1498, 1.0);}
        s.b[1513] = (s.v[1498] < 0.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1513]) {s.store_scalar(1498, 0.0);}
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_offset_scaled(1487, 851, (((p.p227 * s.v[892])) * (p.p229)), p.p229);s.store_offset_scaled(1488, 851, (((p.p228 * s.v[892])) * (p.p230)), p.p230);s.store_mul3_affine_lhs(1492, 1492, 1487, 3.0, 0.0, 1487);s.store_mul3_affine_lhs(1493, 1493, 1488, 3.75, 0.0, 1488);s.store_div_scaled_product_offset_denominator_mixed_iia(1499, 880, 72, p.p3, A::mul(s.ad_value(881), s.ad_value(887)), 1.0, 1.0);s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));s.store_div_scaled_offset_numerator_mixed_ia(1497, 1499, 1.0, 1e-15, A::sqrt(A::div(s.ad_value(1493), s.ad_value(1492))), 1.0);}
        s.b[1514] = (p.p223 != 3.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });s.b[1546] = ((p.p429 != 2.0) && ((s.v[61] + p.p136) >= p.p431));s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });s.b[1547] = ((p.p429 != 2.0) && ((s.v[60] + p.p135) >= p.p431));s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });s.b[1548] = (s.v[398] > 0.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });s.b[1549] = (p.p430 != 0.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if (s.b[1548] && s.b[1549]) {s.store_scale(88, 905, (p.p37 * p.p30));s.store_scale(89, 906, (p.p37 * p.p30));s.store_scale(90, 1024, (p.p37 * p.p30));s.store_scale(91, 1023, (p.p37 * p.p30));}
        if (s.b[1548] && (!s.b[1549])) {s.store_scale(88, 905, p.p37);s.store_scale(89, 906, p.p37);s.store_scale(90, 1024, p.p37);s.store_scale(91, 1023, p.p37);}
        if s.b[1548] {s.store_scale(92, 918, p.p37);s.store_scale(93, 919, p.p37);}
        s.b[1550] = (p.p430 != 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if ((!s.b[1548]) && s.b[1550]) {s.store_scale(89, 905, (p.p37 * p.p30));s.store_scale(88, 906, (p.p37 * p.p30));s.store_scale(91, 1024, (p.p37 * p.p30));s.store_scale(90, 1023, (p.p37 * p.p30));}
        if ((!s.b[1548]) && (!s.b[1550])) {s.store_scale(89, 905, p.p37);s.store_scale(88, 906, p.p37);s.store_scale(91, 1024, p.p37);s.store_scale(90, 1023, p.p37);}
        if (!s.b[1548]) {s.store_scale(93, 918, p.p37);s.store_scale(92, 919, p.p37);}
        s.b[1551] = (p.p430 != 0.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if s.b[1551] {s.store_scale(94, 1022, (p.p37 * p.p30));s.store_scale(95, 1021, (p.p37 * p.p30));}
        if (!s.b[1551]) {s.store_scale(94, 1022, p.p37);s.store_scale(95, 1021, p.p37);}
        s.b[1552] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = (p.p39 == 3.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1554] = ((p.p39 == 0.0) || (p.p39 == 2.0));s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });s.b[1555] = ((p.p39 == 0.0) || (p.p39 == 1.0));s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });s.b[1556] = (p.p39 == 2.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });s.b[1558] = (s.v[37] == 2.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });s.b[1559] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });s.b[1560] = ((p.p35 != 0.0) && (!true));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = true;s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = true;s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = (p.p430 == 2.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });s.b[1564] = (p.p430 == 2.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });s.b[1565] = ((p.p35 != 0.0) && (!true));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = true;s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });s.b[1567] = true;s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });s.copy_ad(426, 916);s.copy_ad(427, 918);s.copy_ad(428, 919);s.store_add(425, 896, 895);s.store_sub(918, 427, 895);s.store_sub(919, 428, 896);s.store_add(916, 426, 425);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(409, (ctx_temp + p.p0));s.store_scalar(429, (p.p126 + 273.15));s.store_scalar(36, p.p336);s.store_scalar(37, p.p21);s.store_scalar(38, p.p348);s.store_scalar(39, p.p213);s.store_scalar(40, p.p127);s.store_scalar(41, p.p182);s.store_scalar(42, p.p350);s.store_scalar(43, p.p355);s.store_scalar(44, p.p234);s.store_scalar(45, p.p236);s.store_scalar(46, p.p373);s.store_scalar(48, p.p181);
        if (p.p41 != 0.0) {s.store_scalar(416, 3.9);s.store_scalar(415, p.p45);s.store_scalar(417, (8.85418e-12 * p.p47));s.store_primal_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));s.store_primal_div_scaled_inputs_indices(396, 416, 8.85418e-12, 415, 1.0);}
        if (p.p41 == 0.0) {s.store_scalar(416, p.p46);s.store_scalar(415, p.p66);s.store_scalar(417, 1.03594e-10);s.store_scalar(419, 5.753e-12);s.store_scalar(396, (3.453133e-11 / p.p66));}
        s.b[431] = (s.v[37] == 2.0);s.store_scalar(431, if s.b[431] { 1.0 } else { 0.0 });
        if s.b[431] {s.store_scalar(399, 0.0);}
        s.b[456] = (!true);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        if ((!s.b[431]) && s.b[456]) {s.store_scalar(399, 0.0);}
        s.b[458] = (!true);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });s.b[459] = ((s.v[38] == 0.0) && (p.p349 == 0.0));s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && s.b[459]) {s.store_scalar(399, 2.0);}
        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && (!s.b[459])) {s.store_scalar(399, 1.0);}
        s.b[460] = ((s.v[38] == 0.0) && (p.p349 == 0.0));s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {s.store_scalar(38, 1.0);s.store_scalar(399, 1.0);}
        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && (!s.b[460])) {s.store_scalar(399, 1.0);}
        s.b[461] = param_given[213];s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });
        if s.b[461] {s.store_scalar(39, p.p213);}
        if (!s.b[461]) {s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p.p66))) as f64).ln()));}
        s.b[533] = (s.v[48] < 0.1);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if s.b[533] {s.store_scalar(48, 0.1);}
        s.b[534] = (s.v[41] < 0.1);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if s.b[534] {s.store_scalar(41, 0.1);}
        s.store_scalar(429, (p.p126 + 273.15));s.store_scalar(476, (s.v[409] / s.v[429]));
        if (p.p41 != 0.0) {s.store_primal_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));}
        if (p.p41 == 0.0) {s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p.p66)) as f64).sqrt());}
        s.b[535] = (p.p41 == 0.0);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if s.b[535] {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));s.copy_ad(395, 465);}
        if s.b[535] {s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));}
        if (!s.b[535]) {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (p.p49 - (((p.p50 * s.v[429]) * s.v[429]) / (s.v[429] + p.p51))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (p.p49 - (((p.p50 * s.v[409]) * s.v[409]) / (s.v[409] + p.p51))));s.copy_ad(395, 465);}
        if (!s.b[535]) {s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p.p48 * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));}
        s.store_scalar(50, (p.p16 * p.p349));s.store_scalar(474, p.p1);s.store_scalar(475, (p.p2 / p.p3));s.store_scalar(467, ((s.v[474]) as f64).powf(p.p190));s.store_scalar(468, ((s.v[475]) as f64).powf(p.p193));s.store_scalar(463, (((p.p188 / s.v[467]) + (p.p191 / s.v[468])) + (p.p194 / (s.v[467] * s.v[468]))));s.store_scalar(326, (p.p187 + s.v[463]));s.store_scalar(463, (((p.p189 / s.v[467]) + (p.p192 / s.v[468])) + (p.p195 / (s.v[467] * s.v[468]))));s.store_scalar(330, (p.p217 + s.v[463]));s.store_scalar(215, (p.p410 + s.v[463]));s.b[536] = (s.v[215] < 0.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[536] {s.store_scalar(215, 0.0);}
        s.store_scalar(469, ((s.v[474]) as f64).powf(p.p202));s.store_scalar(470, ((s.v[475]) as f64).powf(p.p205));s.store_scalar(464, (((p.p200 / s.v[469]) + (p.p203 / s.v[470])) + (p.p206 / (s.v[469] * s.v[470]))));s.store_scalar(325, (p.p197 + s.v[464]));s.store_scalar(464, (((p.p201 / s.v[469]) + (p.p204 / s.v[470])) + (p.p207 / (s.v[469] * s.v[470]))));s.store_scalar(329, (p.p216 + s.v[464]));s.store_scalar(327, (p.p1 - (2.0 * s.v[326])));s.store_scalar(328, (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[325])));s.store_scalar(348, ((s.v[328] / p.p23) + p.p24));s.store_scalar(347, ((s.v[328] / p.p23) + p.p25));s.store_scalar(331, (p.p1 - (2.0 * s.v[330])));s.store_scalar(332, (((p.p2 / p.p3) - (p.p22 * p.p303)) - ((2.0 - p.p22) * s.v[329])));s.store_scalar(349, ((s.v[332] / p.p23) + p.p24));s.store_scalar(350, ((s.v[332] / p.p23) + p.p25));s.store_scalar(365, ((p.p1 - (2.0 * s.v[330])) - p.p360));s.store_scalar(366, (s.v[365] + (2.0 * p.p372)));s.store_scalar(112, p.p85);s.store_scalar(113, p.p86);s.store_scalar(114, p.p87);s.store_scalar(116, p.p88);s.store_scalar(117, p.p89);s.copy_ad(239, 39);s.store_scalar(240, p.p214);s.store_scalar(241, p.p215);s.b[543] = (s.v[241] == 0.0);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if s.b[543] {s.store_scalar(333, 2.0);}
        if (!s.b[543]) {s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));}
        s.b[544] = (p.p65 == 1.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if s.b[544] {s.store_scalar(477, (1e-6 / s.v[327]));s.store_scalar(478, (1e-6 / s.v[328]));s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));}
        if (!s.b[544]) {s.store_scalar(477, (1.0 / s.v[327]));s.store_scalar(478, (1.0 / s.v[328]));s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));}
        s.store_add_scaled_inputs3_offset_indices(108, 477, p.p488, 478, p.p678, 479, p.p868, p.p82);s.store_add_scaled_inputs3_offset_indices(109, 477, p.p489, 478, p.p679, 479, p.p869, p.p81);s.store_add_scaled_inputs3_offset_indices(110, 477, p.p490, 478, p.p680, 479, p.p871, p.p83);s.store_add_scaled_inputs3_offset_indices(111, 477, p.p491, 478, p.p681, 479, p.p870, p.p84);s.store_add_scaled_inputs3_offset_indices(137, 477, p.p492, 478, p.p682, 479, p.p872, p.p108);s.store_add_scaled_inputs3_offset_indices(152, 477, p.p493, 478, p.p683, 479, p.p873, p.p109);s.store_add_scaled_inputs3_offset_indices(120, 477, p.p494, 478, p.p684, 479, p.p874, p.p90);s.store_add_scaled_inputs3_offset_indices(124, 477, p.p497, 478, p.p687, 479, p.p877, p.p94);s.store_add_scaled_inputs3_offset_indices(264, 477, p.p495, 478, p.p685, 479, p.p875, p.p300);s.store_add_scaled_inputs3_offset_indices(265, 477, p.p496, 478, p.p686, 479, p.p876, p.p301);s.store_add_scaled_inputs3_offset_indices(125, 477, p.p498, 478, p.p688, 479, p.p878, p.p95);s.store_add_scaled_inputs3_offset_indices(126, 477, p.p499, 478, p.p689, 479, p.p879, p.p96);s.store_add_scaled_inputs3_offset_indices(263, 477, p.p500, 478, p.p690, 479, p.p880, p.p371);s.store_add_scaled_inputs3_offset_indices(127, 477, p.p501, 478, p.p691, 479, p.p881, p.p97);s.store_add_scaled_inputs3_offset_indices(128, 477, p.p1024, 478, p.p1027, 479, p.p1030, p.p1021);s.store_add_scaled_inputs3_offset_indices(377, 477, p.p502, 478, p.p692, 479, p.p882, p.p98);s.store_add_scaled_inputs3_offset_indices(129, 477, p.p503, 478, p.p693, 479, p.p883, p.p99);s.store_add_scaled_inputs3_offset_indices(130, 477, p.p504, 478, p.p694, 479, p.p884, p.p100);s.store_add_scaled_inputs3_offset_indices(131, 477, p.p505, 478, p.p695, 479, p.p885, p.p101);s.store_add_scaled_inputs3_offset_indices(132, 477, p.p506, 478, p.p696, 479, p.p886, p.p102);s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);s.store_add_scaled_inputs3_offset_indices(133, 477, p.p507, 478, p.p697, 479, p.p887, p.p103);s.store_add_scaled_inputs3_offset_indices(134, 477, p.p508, 478, p.p698, 479, p.p888, p.p104);s.store_add_scaled_inputs3_offset_indices(144, 477, p.p509, 478, p.p699, 479, p.p889, p.p116);s.store_add_scaled_inputs3_offset_indices(138, 477, p.p511, 478, p.p701, 479, p.p891, p.p110);s.store_add_scaled_inputs3_offset_indices(140, 477, p.p512, 478, p.p702, 479, p.p892, p.p112);s.store_add_scaled_inputs3_offset_indices(142, 477, p.p513, 478, p.p703, 479, p.p893, p.p114);s.store_add_scaled_inputs3_offset_indices(101, 477, p.p518, 478, p.p708, 479, p.p898, p.p74);s.store_add_scaled_inputs3_offset_indices(103, 477, p.p519, 478, p.p709, 479, p.p899, p.p76);s.store_add_scaled_inputs3_offset_indices(104, 477, p.p520, 478, p.p710, 479, p.p900, p.p77);s.store_add_scaled_inputs3_offset_indices(199, 477, p.p521, 478, p.p711, 479, p.p901, p.p208);s.store_add_scaled_inputs3_offset_indices(200, 477, p.p522, 478, p.p712, 479, p.p902, p.p209);s.store_add_scaled_inputs3_offset_indices(107, 477, p.p523, 478, p.p713, 479, p.p903, p.p80);s.store_add_scaled_inputs3_offset_indices(266, 477, p.p524, 478, p.p714, 479, p.p904, p.p302);s.store_add_scaled_inputs3_offset_indices(105, 477, p.p525, 478, p.p715, 479, p.p905, p.p78);s.store_add_scaled_inputs3_offset_indices(106, 477, p.p526, 478, p.p716, 479, p.p906, p.p79);s.store_add_scaled_inputs3_offset_indices(181, 477, p.p527, 478, p.p717, 479, p.p907, p.p132);s.store_add_scaled_inputs3_offset_indices(170, 477, p.p528, 478, p.p718, 479, p.p908, p.p133);s.store_add_scaled_inputs3_offset_indices(169, 477, p.p529, 478, p.p719, 479, p.p909, p.p134);s.store_add_scaled_inputs3_offset_indices(184, 477, p.p530, 478, p.p720, 479, p.p910, p.p142);s.store_add_scaled_inputs3_offset_indices(185, 477, p.p531, 478, p.p721, 479, p.p911, p.p143);s.store_add_scaled_inputs3_offset_indices(183, 477, p.p532, 478, p.p722, 479, p.p912, p.p141);s.store_add_scaled_inputs3_offset_indices(196, 477, p.p533, 478, p.p723, 479, p.p913, p.p196);
        s.store_add_scaled_inputs3_offset_indices(100, 477, p.p534, 478, p.p724, 479, p.p914, p.p73);s.store_add_scaled_inputs3_offset_indices(197, 477, p.p535, 478, p.p725, 479, p.p915, p.p198);s.store_add_scaled_inputs3_offset_indices(198, 477, p.p536, 478, p.p726, 479, p.p916, p.p199);s.store_add_scaled_inputs3_offset_indices(151, 477, p.p537, 478, p.p727, 479, p.p917, p.p125);s.store_add_scaled_inputs3_offset_indices(187, 477, p.p538, 478, p.p728, 479, p.p918, p.p145);s.store_add_scaled_inputs3_offset_indices(188, 477, p.p539, 478, p.p729, 479, p.p919, p.p146);s.store_add_scaled_inputs3_offset_indices(189, 477, p.p540, 478, p.p730, 479, p.p920, p.p147);s.store_add_scaled_inputs3_offset_indices(190, 477, p.p541, 478, p.p731, 479, p.p921, p.p148);s.store_add_scaled_inputs3_offset_indices(136, 477, p.p542, 478, p.p732, 479, p.p922, p.p106);s.store_add_scaled_inputs3_offset_indices(99, 477, p.p543, 478, p.p733, 479, p.p923, p.p72);s.store_add_scaled_inputs3_offset_indices(96, 477, p.p544, 478, p.p734, 479, p.p924, p.p69);s.store_add_scaled_inputs3_offset_indices(97, 477, p.p545, 478, p.p735, 479, p.p925, p.p70);s.store_add_scaled_inputs3_offset_indices(98, 477, p.p546, 478, p.p736, 479, p.p926, p.p71);s.store_add_scaled_inputs3_offset_indices(191, 477, p.p547, 478, p.p737, 479, p.p927, p.p149);s.store_add_scaled_inputs3_offset_indices(192, 477, p.p548, 478, p.p738, 479, p.p928, p.p150);s.store_add_scaled_inputs3_offset_indices(193, 477, p.p549, 478, p.p739, 479, p.p929, p.p151);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(194, 477, p.p550, 478, p.p740, 479, p.p930, p.p152);s.store_add_scaled_inputs3_offset_indices(135, 477, p.p551, 478, p.p741, 479, p.p931, p.p105);s.store_add_scaled_inputs3_offset_indices(195, 477, p.p552, 478, p.p742, 479, p.p932, p.p153);s.store_add_scaled_inputs3_offset_indices(180, 477, p.p553, 478, p.p743, 479, p.p933, p.p130);s.store_add_scaled_inputs3_offset_indices(201, 477, p.p554, 478, p.p744, 479, p.p934, p.p218);s.store_add_scaled_inputs3_offset_indices(267, 477, p.p555, 478, p.p745, 479, p.p935, p.p314);s.store_add_scaled_inputs3_offset_indices(268, 477, p.p558, 478, p.p748, 479, p.p938, p.p315);s.store_add_scaled_inputs3_offset_indices(269, 477, p.p557, 478, p.p747, 479, p.p937, p.p316);s.store_add_scaled_inputs3_offset_indices(270, 477, p.p560, 478, p.p750, 479, p.p940, p.p317);s.store_add_scaled_inputs3_offset_indices(271, 477, p.p556, 478, p.p746, 479, p.p936, p.p318);s.store_add_scaled_inputs3_offset_indices(272, 477, p.p559, 478, p.p749, 479, p.p939, p.p319);s.store_add_scaled_inputs3_offset_indices(202, 477, p.p561, 478, p.p751, 479, p.p941, p.p304);s.store_add_scaled_inputs3_offset_indices(273, 477, p.p562, 478, p.p752, 479, p.p942, p.p305);s.store_add_scaled_inputs3_offset_indices(274, 477, p.p563, 478, p.p753, 479, p.p943, p.p306);s.store_add_scaled_inputs3_offset_indices(275, 477, p.p564, 478, p.p754, 479, p.p944, p.p307);s.store_add_scaled_inputs3_offset_indices(276, 477, p.p565, 478, p.p755, 479, p.p945, p.p309);s.store_add_scaled_inputs3_offset_indices(277, 477, p.p566, 478, p.p756, 479, p.p946, p.p321);s.store_add_scaled_inputs3_offset_indices(278, 477, p.p567, 478, p.p757, 479, p.p947, p.p310);s.store_add_scaled_inputs3_offset_indices(279, 477, p.p568, 478, p.p758, 479, p.p948, p.p311);s.store_add_scaled_inputs3_offset_indices(280, 477, p.p569, 478, p.p759, 479, p.p949, p.p312);s.store_add_scaled_inputs3_offset_indices(281, 477, p.p570, 478, p.p760, 479, p.p950, p.p313);s.store_add_scaled_inputs3_offset_indices(282, 477, p.p571, 478, p.p761, 479, p.p951, p.p158);s.store_add_scaled_inputs3_offset_indices(283, 477, p.p572, 478, p.p762, 479, p.p952, p.p159);s.store_add_scaled_inputs3_offset_indices(284, 477, p.p573, 478, p.p763, 479, p.p953, p.p160);s.store_add_scaled_inputs3_offset_indices(285, 477, p.p574, 478, p.p764, 479, p.p954, p.p161);s.store_add_scaled_inputs3_offset_indices(286, 477, p.p1025, 478, p.p1028, 479, p.p1031, p.p1022);s.store_add_scaled_inputs3_offset_indices(287, 477, p.p575, 478, p.p765, 479, p.p955, p.p162);s.store_add_scaled_inputs3_offset_indices(288, 477, p.p576, 478, p.p766, 479, p.p956, p.p163);s.store_add_scaled_inputs3_offset_indices(289, 477, p.p577, 478, p.p767, 479, p.p957, p.p164);s.store_add_scaled_inputs3_offset_indices(290, 477, p.p578, 478, p.p768, 479, p.p958, p.p165);s.store_add_scaled_inputs3_offset_indices(291, 477, p.p579, 478, p.p769, 479, p.p959, p.p166);s.store_add_scaled_inputs3_offset_indices(292, 477, p.p580, 478, p.p770, 479, p.p960, p.p167);s.store_add_scaled_inputs3_offset_indices(293, 477, p.p581, 478, p.p771, 479, p.p961, p.p168);s.store_add_scaled_inputs3_offset_indices(294, 477, p.p1026, 478, p.p1029, 479, p.p1032, p.p1023);s.store_add_scaled_inputs3_offset_indices(295, 477, p.p582, 478, p.p772, 479, p.p962, p.p169);s.store_add_scaled_inputs3_offset_indices(296, 477, p.p583, 478, p.p773, 479, p.p963, p.p170);s.store_add_scaled_inputs3_offset_indices(297, 477, p.p584, 478, p.p774, 479, p.p964, p.p171);s.store_add_scaled_inputs3_offset_indices(298, 477, p.p585, 478, p.p775, 479, p.p965, p.p322);s.store_add_scaled_inputs3_offset_indices(299, 477, p.p586, 478, p.p776, 479, p.p966, p.p323);s.store_add_scaled_inputs3_offset_indices(300, 477, p.p587, 478, p.p777, 479, p.p967, p.p172);s.store_add_scaled_inputs3_offset_indices(301, 477, p.p588, 478, p.p778, 479, p.p968, p.p173);s.store_add_scaled_inputs3_offset_indices(302, 477, p.p589, 478, p.p779, 479, p.p969, p.p324);s.store_add_scaled_inputs3_offset_indices(303, 477, p.p590, 478, p.p780, 479, p.p970, p.p325);
        s.store_add_scaled_inputs3_offset_indices(304, 477, p.p591, 478, p.p781, 479, p.p971, p.p326);s.store_add_scaled_inputs3_offset_indices(305, 477, p.p592, 478, p.p782, 479, p.p972, p.p327);s.store_add_scaled_inputs3_offset_indices(306, 477, p.p593, 478, p.p783, 479, p.p973, p.p328);s.store_add_scaled_inputs3_offset_indices(307, 477, p.p594, 478, p.p784, 479, p.p974, p.p329);s.store_add_scaled_inputs3_offset_indices(308, 477, p.p595, 478, p.p785, 479, p.p975, p.p330);s.store_add_scaled_inputs3_offset_indices(309, 477, p.p596, 478, p.p786, 479, p.p976, p.p331);s.store_add_scaled_inputs3_offset_indices(310, 477, p.p597, 478, p.p787, 479, p.p977, p.p332);s.store_add_scaled_inputs3_offset_indices(312, 477, p.p599, 478, p.p789, 479, p.p979, p.p334);s.store_add_scaled_inputs3_offset_indices(311, 477, p.p598, 478, p.p788, 479, p.p978, p.p333);s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);s.store_add_scaled_inputs3_offset_indices(313, 477, p.p600, 478, p.p790, 479, p.p980, p.p335);s.store_add_scaled_inputs3_offset_indices(314, 477, p.p601, 478, p.p791, 479, p.p981, p.p337);s.store_add_scaled_inputs3_offset_indices(315, 477, p.p602, 478, p.p792, 479, p.p982, p.p338);s.store_add_scaled_inputs3_offset_indices(316, 477, p.p603, 478, p.p793, 479, p.p983, p.p339);s.store_add_scaled_inputs3_offset_indices(317, 477, p.p604, 478, p.p794, 479, p.p984, p.p340);s.store_add_scaled_inputs3_offset_indices(318, 477, p.p605, 478, p.p795, 479, p.p985, p.p341);s.store_add_scaled_inputs3_offset_indices(319, 477, p.p606, 478, p.p796, 479, p.p986, p.p342);s.store_add_scaled_inputs3_offset_indices(320, 477, p.p607, 478, p.p797, 479, p.p987, p.p344);s.store_add_scaled_inputs3_offset_indices(321, 477, p.p608, 478, p.p798, 479, p.p988, p.p345);s.store_add_scaled_inputs3_offset_indices(355, 477, p.p609, 478, p.p799, 479, p.p989, p.p346);s.store_add_scaled_inputs3_offset_indices(356, 477, p.p610, 478, p.p800, 479, p.p990, p.p347);s.store_add_scaled_inputs3_offset_indices(242, 477, p.p443, 478, p.p633, 479, p.p823, p.p157);s.store_add_scaled_inputs3_offset_indices(243, 477, p.p444, 478, p.p634, 479, p.p824, p.p383);s.store_add_scaled_inputs3_offset_indices(244, 477, p.p445, 478, p.p635, 479, p.p825, p.p384);s.store_add_scaled_inputs3_offset_indices(246, 477, p.p447, 478, p.p637, 479, p.p827, p.p388);s.store_add_scaled_inputs3_offset_indices(247, 477, p.p448, 478, p.p638, 479, p.p828, p.p389);s.store_add_scaled_inputs3_offset_indices(245, 477, p.p446, 478, p.p636, 479, p.p826, p.p385);s.store_add_scaled_inputs3_offset_indices(249, 477, p.p449, 478, p.p639, 479, p.p829, p.p390);s.store_add_scaled_inputs3_offset_indices(253, 477, p.p457, 478, p.p647, 479, p.p837, p.p352);s.store_add_scaled_inputs3_offset_indices(254, 477, p.p467, 478, p.p657, 479, p.p847, p.p358);s.store_add_scaled_inputs3_offset_indices(255, 477, p.p468, 478, p.p658, 479, p.p848, p.p359);s.store_add_scaled_inputs3_offset_indices(256, 477, p.p469, 478, p.p659, 479, p.p849, p.p174);s.store_add_scaled_inputs3_offset_indices(257, 477, p.p470, 478, p.p660, 479, p.p850, p.p175);s.store_add_scaled_inputs3_offset_indices(258, 477, p.p471, 478, p.p661, 479, p.p851, p.p176);s.store_add_scaled_inputs3_offset_indices(259, 477, p.p472, 478, p.p662, 479, p.p852, p.p177);s.store_add_scaled_inputs3_offset_indices(260, 477, p.p473, 478, p.p663, 479, p.p853, p.p178);s.store_add_scaled_inputs3_offset_indices(261, 477, p.p474, 478, p.p664, 479, p.p854, p.p179);s.store_add_scaled_inputs3_offset_indices(262, 477, p.p475, 478, p.p665, 479, p.p855, p.p180);s.store_add_scaled_inputs3_offset_indices(237, 477, p.p455, 478, p.p645, 479, p.p835, p.p211);s.store_add_scaled_inputs3_offset_indices(236, 477, p.p454, 478, p.p644, 479, p.p834, p.p210);s.store_add_scaled_inputs3_offset_indices(238, 477, p.p456, 478, p.p646, 479, p.p836, p.p212);s.store_add_scaled_inputs3_offset_indices(145, 477, p.p458, 478, p.p648, 479, p.p838, p.p118);s.store_add_scaled_inputs3_offset_indices(146, 477, p.p514, 478, p.p704, 479, p.p894, p.p121);
        s.store_add_scaled_inputs3_offset_indices(147, 477, p.p515, 478, p.p705, 479, p.p895, p.p122);s.store_add_scaled_inputs3_offset_indices(148, 477, p.p510, 478, p.p700, 479, p.p890, p.p117);s.store_add_scaled_inputs3_offset_indices(149, 477, p.p517, 478, p.p707, 479, p.p897, p.p119);s.store_add_scaled_inputs3_offset_indices(150, 477, p.p516, 478, p.p706, 479, p.p896, p.p120);s.store_add_scaled_inputs3_offset_indices(121, 477, p.p459, 478, p.p649, 479, p.p839, p.p91);s.store_add_scaled_inputs3_offset_indices(123, 477, p.p461, 478, p.p651, 479, p.p841, p.p93);s.store_add_scaled_inputs3_offset_indices(122, 477, p.p460, 478, p.p650, 479, p.p840, p.p92);s.store_add_scaled_inputs3_offset_indices(139, 477, p.p462, 478, p.p652, 479, p.p842, p.p111);s.store_add_scaled_inputs3_offset_indices(141, 477, p.p463, 478, p.p653, 479, p.p843, p.p113);s.store_add_scaled_inputs3_offset_indices(143, 477, p.p464, 478, p.p654, 479, p.p844, p.p115);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(102, 477, p.p465, 478, p.p655, 479, p.p845, p.p75);s.store_add_scaled_inputs3_offset_indices(186, 477, p.p466, 478, p.p656, 479, p.p846, p.p144);s.store_add_scaled_inputs3_offset_indices(211, 477, p.p484, 478, p.p674, 479, p.p864, p.p406);s.store_add_scaled_inputs3_offset_indices(203, 477, p.p476, 478, p.p666, 479, p.p856, p.p398);s.store_add_scaled_inputs3_offset_indices(204, 477, p.p477, 478, p.p667, 479, p.p857, p.p399);s.store_add_scaled_inputs3_offset_indices(205, 477, p.p478, 478, p.p668, 479, p.p858, p.p400);s.store_add_scaled_inputs3_offset_indices(206, 477, p.p479, 478, p.p669, 479, p.p859, p.p401);s.store_add_scaled_inputs3_offset_indices(207, 477, p.p480, 478, p.p670, 479, p.p860, p.p402);s.store_add_scaled_inputs3_offset_indices(208, 477, p.p481, 478, p.p671, 479, p.p861, p.p403);s.store_add_scaled_inputs3_offset_indices(209, 477, p.p482, 478, p.p672, 479, p.p862, p.p404);s.store_add_scaled_inputs3_offset_indices(210, 477, p.p483, 478, p.p673, 479, p.p863, p.p405);s.store_add_scaled_inputs3_offset_indices(212, 477, p.p485, 478, p.p675, 479, p.p865, p.p407);s.store_add_scaled_inputs3_offset_indices(213, 477, p.p486, 478, p.p676, 479, p.p866, p.p408);s.store_add_scaled_inputs3_offset_indices(229, 477, p.p618, 478, p.p808, 479, p.p998, p.p422);s.store_add_scaled_inputs3_offset_indices(230, 477, p.p619, 478, p.p809, 479, p.p999, p.p423);s.store_add_scaled_inputs3_offset_indices(216, 477, p.p620, 478, p.p810, 479, p.p1000, p.p413);s.store_add_scaled_inputs3_offset_indices(217, 477, p.p621, 478, p.p811, 479, p.p1001, p.p433);s.store_add_scaled_inputs3_offset_indices(218, 477, p.p622, 478, p.p812, 479, p.p1002, p.p434);s.store_add_scaled_inputs3_offset_indices(219, 477, p.p623, 478, p.p813, 479, p.p1003, p.p414);s.store_add_scaled_inputs3_offset_indices(220, 477, p.p624, 478, p.p814, 479, p.p1004, p.p415);s.store_add_scaled_inputs3_offset_indices(221, 477, p.p625, 478, p.p815, 479, p.p1005, p.p416);s.store_add_scaled_inputs3_offset_indices(222, 477, p.p626, 478, p.p816, 479, p.p1006, p.p417);s.store_add_scaled_inputs3_offset_indices(223, 477, p.p627, 478, p.p817, 479, p.p1007, p.p418);s.store_add_scaled_inputs3_offset_indices(224, 477, p.p628, 478, p.p818, 479, p.p1008, p.p419);s.store_add_scaled_inputs3_offset_indices(225, 477, p.p629, 478, p.p819, 479, p.p1009, p.p420);s.store_add_scaled_inputs3_offset_indices(226, 477, p.p630, 478, p.p820, 479, p.p1010, p.p421);s.store_add_scaled_inputs3_offset_indices(227, 477, p.p631, 478, p.p821, 479, p.p1011, p.p411);s.store_add_scaled_inputs3_offset_indices(228, 477, p.p632, 478, p.p822, 479, p.p1012, p.p412);s.store_add_scaled_inputs3_offset_indices(322, 477, p.p611, 478, p.p801, 479, p.p991, p.p353);s.store_add_scaled_inputs3_offset_indices(323, 477, p.p612, 478, p.p802, 479, p.p992, p.p354);s.store_add_scaled_inputs3_offset_indices(324, 477, p.p613, 478, p.p803, 479, p.p993, p.p370);s.store_add_scaled_inputs3_offset_indices(361, 477, p.p614, 478, p.p804, 479, p.p994, p.p366);s.store_mul_powf_mixed_ia(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));s.store_add_scaled_inputs3_offset_indices(362, 477, p.p615, 478, p.p805, 479, p.p995, p.p367);s.store_add_scaled_inputs3_offset_indices(363, 477, p.p616, 478, p.p806, 479, p.p996, p.p368);s.store_add_scaled_inputs3_offset_indices(364, 477, p.p617, 478, p.p807, 479, p.p997, p.p369);s.store_add_scaled_inputs3_offset_indices(378, 477, p.p259, 478, p.p260, 479, p.p261, p.p258);s.store_add_scaled_inputs3_offset_indices(379, 477, p.p263, 478, p.p264, 479, p.p265, p.p262);s.store_add_scaled_inputs3_offset_indices(380, 477, p.p267, 478, p.p268, 479, p.p269, p.p266);s.store_add_scaled_inputs3_offset_indices(381, 477, p.p271, 478, p.p272, 479, p.p273, p.p270);s.store_add_scaled_inputs3_offset_indices(382, 477, p.p275, 478, p.p276, 479, p.p277, p.p274);s.store_add_scaled_inputs3_offset_indices(383, 477, p.p279, 478, p.p280, 479, p.p281, p.p278);s.store_add_scaled_inputs3_offset_indices(389, 477, p.p436, 478, p.p437, 479, p.p438, p.p435);
        s.store_add_scaled_inputs3_offset_indices(390, 477, p.p440, 478, p.p441, 479, p.p442, p.p439);s.store_add_scaled_inputs3_offset_indices(385, 477, p.p286, 478, p.p289, 479, p.p292, p.p285);s.store_add_scaled_inputs3_offset_indices(386, 477, p.p287, 478, p.p290, 479, p.p293, p.p282);s.store_add_scaled_inputs3_offset_indices(387, 477, p.p288, 478, p.p291, 479, p.p294, p.p284);s.store_add_scaled_inputs3_offset_indices(250, 477, p.p450, 478, p.p640, 479, p.p830, p.p392);s.store_add_scaled_inputs3_offset_indices(248, 477, p.p451, 478, p.p641, 479, p.p831, p.p393);s.store_add_scaled_inputs3_offset_indices(251, 477, p.p452, 478, p.p642, 479, p.p832, p.p394);s.store_add_scaled_inputs3_offset_indices(252, 477, p.p453, 478, p.p643, 479, p.p833, p.p395);s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);s.store_scalar(430, (s.v[476] - 1.0));s.copy_ad(153, 138);s.copy_ad(154, 140);s.copy_ad(155, 142);s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));s.store_scalar(157, ((p.p14 / (p.p3 * (s.v[328] + p.p377))) * p.p23));s.store_scalar(158, ((p.p15 * (p.p3 * (s.v[328] + p.p377))) / p.p23));s.b[547] = (s.v[38] == 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });
        if s.b[547] {s.store_scalar(156, 0.0);}
        if (!s.b[547]) {s.store_div_scaled_inputs_mixed_ia(156, 38, (((p.p17 * p.p378) * (s.v[328] * 1.0 / (p.p23))) * 1.0 / (p.p3)), A::scale_offset(s.ad_value(38), 2.0, (p.p378 * s.v[327])), 1.0);}
        s.store_scalar(345, (((((p.p380 / p.p376)) as f64).powf(p.p379) / p.p376) / p.p376));s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);s.b[548] = (s.v[144] > 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if s.b[548] {s.store_scale(144, 144, 0.0001);}
        s.store_mul_mixed_ia(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);s.store_div_scaled_inputs2_indices(182, 181, 1.0, 186, s.v[430], 159, 1.0);s.b[549] = (p.p429 == 1.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if s.b[549] {s.store_scale(496, 159, p.p3);s.store_scale(497, 186, s.v[430]);s.store_add(468, 169, 497);s.store_offset(469, 497, p.p140);}
        s.b[550] = (s.v[468] < 0.0);s.store_scalar(550, if s.b[550] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[550]) {s.store_scalar(468, 0.0);}
        s.b[551] = (s.v[469] < 0.0);s.store_scalar(551, if s.b[551] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[549] && s.b[551]) {s.store_scalar(469, 0.0);}
        if s.b[549] {s.store_div(173, 468, 496);s.store_add(470, 170, 497);s.store_offset(471, 497, p.p139);}
        s.b[552] = (s.v[470] < 0.0);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[552]) {s.store_scalar(470, 0.0);}
        s.b[553] = (s.v[471] < 0.0);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[553]) {s.store_scalar(471, 0.0);}
        if s.b[549] {s.store_div(174, 470, 496);}
        if (!s.b[549]) {s.store_scalar(173, 0.0);s.store_scalar(174, 0.0);}
        s.b[554] = param_given[128];s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if s.b[554] {s.store_scalar(47, p.p128);}
        s.b[555] = (param_given[217] && (p.p217 > 0.0));s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((!s.b[554]) && s.b[555]) {s.store_sub_scaled_inputs(47, 396, p.p217, 237, 1.0);}
        if ((!s.b[554]) && (!s.b[555])) {s.store_scale(47, 396, (0.6 * p.p157));}
        s.b[556] = param_given[127];s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if s.b[556] {s.store_scalar(40, p.p127);}
        s.b[557] = (param_given[217] && (p.p217 > 0.0));s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if ((!s.b[556]) && s.b[557]) {s.store_sub_scaled_inputs(40, 396, p.p217, 236, 1.0);}
        if ((!s.b[556]) && (!s.b[557])) {s.store_scale(40, 396, (0.6 * p.p157));}
        s.b[558] = (s.v[47] < 0.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scalar(47, 0.0);}
        s.b[559] = (s.v[40] < 0.0);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });
        if s.b[559] {s.store_scalar(40, 0.0);}
        s.b[560] = (s.v[42] < 0.0);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if s.b[560] {s.store_scalar(42, 0.0);}
        s.store_scaled_add(335, 47, 239, s.v[349]);s.store_scaled_add(334, 40, 239, s.v[350]);s.store_scale(336, 42, (s.v[331] * p.p3));s.b[561] = ((!param_given[82]) && param_given[85]);s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if s.b[561] {s.store_scale(467, 396, s.v[112]);s.store_scaled_mul(108, 467, 467, 3.021e22);}
        s.b[562] = (s.v[37] == 2.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if (s.b[562] && (p.p41 != 0.0)) {s.store_primal_scale(422, 417, ((((p.p49 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p156 * p.p156))));}
        s.b[563] = (s.v[108] > s.v[422]);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p.p41 != 0.0)) && s.b[563]) {s.copy_ad(108, 422);}
        if (s.b[562] && (p.p41 == 0.0)) {s.store_primal_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p.p155 * p.p155))));}
        s.b[564] = (s.v[108] > s.v[422]);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p.p41 == 0.0)) && s.b[564]) {s.copy_ad(108, 422);}
        s.store_scalar(392, (3.453133e-11 / p.p154));
        if (p.p41 != 0.0) {s.store_scalar(393, (1.03594e-10 / p.p156));}
        if (p.p41 == 0.0) {s.store_scalar(393, (1.03594e-10 / p.p155));}
        if (p.p41 != 0.0) {s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p156))));}
        if (p.p41 == 0.0) {s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p.p1021 / p.p1)) * (1000000.0 * p.p155))));}
        s.store_add_mixed_ai(421, A::sub_from_scalar(0.8, A::div_scaled_inputs(s.ad_value(420), 0.5, s.ad_value(393), 1.0)), 216);s.b[565] = (s.v[37] == 3.0);s.store_scalar(565, if s.b[565] { 1.0 } else { 0.0 });s.b[566] = (s.v[421] > s.v[228]);s.store_scalar(566, if s.b[566] { 1.0 } else { 0.0 });
        if (s.b[565] && s.b[566]) {s.store_scalar(37, 2.0);}
        s.b[567] = (s.v[421] < s.v[227]);s.store_scalar(567, if s.b[567] { 1.0 } else { 0.0 });
        if ((s.b[565] && (!s.b[566])) && s.b[567]) {s.store_scalar(37, 0.0);}
        if ((s.b[565] && (!s.b[566])) && (!s.b[567])) {s.store_scalar(37, 1.0);}
        s.store_scale_ad(471, A::div_from_scalar(1.115, s.ad_value(49)), s.v[430]);s.store_div_scaled_product_indices(532, 256, 471, 1.0, 300, 1.0);s.b[568] = (s.v[532] > 100.0);s.store_scalar(568, if s.b[568] { 1.0 } else { 0.0 });
        if s.b[568] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[569] = (s.v[532] < (-100.0));s.store_scalar(569, if s.b[569] { 1.0 } else { 0.0 });
        if ((!s.b[568]) && s.b[569]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[568]) && (!s.b[569])) {s.store_exp(467, 532);}
        s.store_div_scaled_product_indices(532, 257, 471, 1.0, 300, 1.0);s.b[570] = (s.v[532] > 100.0);s.store_scalar(570, if s.b[570] { 1.0 } else { 0.0 });
        if s.b[570] {s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[571] = (s.v[532] < (-100.0));s.store_scalar(571, if s.b[571] { 1.0 } else { 0.0 });
        if ((!s.b[570]) && s.b[571]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[570]) && (!s.b[571])) {s.store_exp(468, 532);}
        s.store_div_scaled_product_indices(532, 258, 471, 1.0, 302, 1.0);s.b[572] = (s.v[532] > 100.0);s.store_scalar(572, if s.b[572] { 1.0 } else { 0.0 });
        if s.b[572] {s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[573] = (s.v[532] < (-100.0));s.store_scalar(573, if s.b[573] { 1.0 } else { 0.0 });
        if ((!s.b[572]) && s.b[573]) {s.store_scalar(469, 3.720075976e-44);}
        if ((!s.b[572]) && (!s.b[573])) {s.store_exp(469, 532);}
        s.store_mul(357, 355, 467);s.store_mul(161, 306, 467);s.store_mul(163, 308, 468);s.store_mul(165, 310, 469);s.store_scale(532, 259, s.v[430]);s.b[574] = (s.v[532] > 100.0);s.store_scalar(574, if s.b[574] { 1.0 } else { 0.0 });
        if s.b[574] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[575] = (s.v[532] < (-100.0));s.store_scalar(575, if s.b[575] { 1.0 } else { 0.0 });
        if ((!s.b[574]) && s.b[575]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[574]) && (!s.b[575])) {s.store_exp(467, 532);}
        s.store_mul(167, 312, 467);s.store_div_scaled_product_indices(532, 256, 471, 1.0, 301, 1.0);s.b[576] = (s.v[532] > 100.0);s.store_scalar(576, if s.b[576] { 1.0 } else { 0.0 });
        if s.b[576] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[577] = (s.v[532] < (-100.0));s.store_scalar(577, if s.b[577] { 1.0 } else { 0.0 });
        if ((!s.b[576]) && s.b[577]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[576]) && (!s.b[577])) {s.store_exp(467, 532);}
        s.store_div_scaled_product_indices(532, 260, 471, 1.0, 301, 1.0);s.b[578] = (s.v[532] > 100.0);s.store_scalar(578, if s.b[578] { 1.0 } else { 0.0 });
        if s.b[578] {s.store_scaled_offset(468, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[579] = (s.v[532] < (-100.0));s.store_scalar(579, if s.b[579] { 1.0 } else { 0.0 });
        if ((!s.b[578]) && s.b[579]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[578]) && (!s.b[579])) {s.store_exp(468, 532);}
        s.store_div_scaled_product_indices(532, 261, 471, 1.0, 303, 1.0);s.b[580] = (s.v[532] > 100.0);s.store_scalar(580, if s.b[580] { 1.0 } else { 0.0 });
        if s.b[580] {s.store_scaled_offset(469, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[581] = (s.v[532] < (-100.0));s.store_scalar(581, if s.b[581] { 1.0 } else { 0.0 });
        if ((!s.b[580]) && s.b[581]) {s.store_scalar(469, 3.720075976e-44);}
        if ((!s.b[580]) && (!s.b[581])) {s.store_exp(469, 532);}
        s.store_mul(358, 356, 467);s.store_mul(162, 307, 467);s.store_mul(164, 309, 468);s.store_mul(166, 311, 469);s.store_scale(532, 262, s.v[430]);s.b[582] = (s.v[532] > 100.0);s.store_scalar(582, if s.b[582] { 1.0 } else { 0.0 });
        if s.b[582] {s.store_scaled_offset(467, 532, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[583] = (s.v[532] < (-100.0));s.store_scalar(583, if s.b[583] { 1.0 } else { 0.0 });
        if ((!s.b[582]) && s.b[583]) {s.store_scalar(467, 3.720075976e-44);}
        if ((!s.b[582]) && (!s.b[583])) {s.store_exp(467, 532);}
        s.store_mul(168, 313, 467);s.b[584] = (s.v[109] > 0.0);s.store_scalar(584, if s.b[584] { 1.0 } else { 0.0 });
        if s.b[584] {
            s.store_mul_scale_offset_mixed_ia(160, 49, {
                if ((s.v[108] / s.v[109]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p37), 0.0);
        }
        if (!s.b[584]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(160, 49, {
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p.p37), 530, ((2.0) * ((-p.p37))));
        }
        s.b[585] = (!param_given[353]);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (s.v[109] > 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });
        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p.p37));
        }
        s.b[587] = (s.v[109] < 0.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });
        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p.p37));
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(481, 49, {
            if (((s.v[109]) as f64).abs() > 1e-38) {
                A::ln(A::abs(s.ad_value(109)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 530, 2.0);s.store_mul_scaled_sqrt_ad_rhs(482, 419, 1.0 / (s.v[392]), A::abs(s.ad_value(109)));s.b[588] = (!param_given[354]);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });
        if (s.b[588] && s.b[589]) {s.store_add_scaled_inputs_product_mixed_iiia(323, 322, 1.0, 481, 1.0, 482, A::sqrt(s.ad_value(481)), 1.0);}
        if (s.b[588] && (!s.b[589])) {s.store_add_scaled_inputs_product_mixed_iiia(323, 322, 1.0, 481, (-1.0), 482, A::sqrt(s.ad_value(481)), (-1.0));}
        s.b[590] = (!param_given[355]);s.store_scalar(590, if s.b[590] { 1.0 } else { 0.0 });
        if s.b[590] {s.store_sqrt_ad(462, A::div_scaled_product(s.ad_value(417), s.ad_value(481), 2.0, A::abs(s.ad_value(109)), (1.602176462e-19 * 1000000.0)));s.store_div(463, 417, 462);s.store_div_scaled_value_offset_denominator(43, s.ad_value(463), s.v[392], s.ad_value(463), s.v[392], 1.0);}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(118, 49, {
            if (s.v[108] > 1e-38) {
                A::ln(s.ad_value(108))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 2.0, 530, 2.0);s.store_sqrt(339, 118);s.store_mul_sqrt_mixed_ia(340, 339, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_sqrt(341, 340);s.b[591] = (p.p41 == 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });
        if s.b[591] {s.store_sqrt_scaled_input_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p.p66);}
        if (!s.b[591]) {s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(115, 49, {
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, 530, 2.0);s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));s.b[592] = (p.p41 == 0.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (s.v[110] > 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });
        if (s.b[592] && s.b[593]) {
            s.store_mul_mixed_ia(375, 480, {
                            if ((s.v[110] / 1e20) > 1e-38) {
                                A::ln_scaled_input(s.ad_value(110), 1.0 / (1e20))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (s.b[592] && (!s.b[593])) {s.store_scalar(375, 0.0);}
        if (!s.b[592]) {
            s.store_mul_sub_mixed_iai(467, 480, {
                if (s.v[111] > 1e-38) {
                    A::ln(s.ad_value(111))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 530);
        }
        if (!s.b[592]) {s.store_scale(468, 466, 0.5);}
        s.b[594] = (s.v[467] > s.v[468]);s.store_scalar(594, if s.b[594] { 1.0 } else { 0.0 });
        if ((!s.b[592]) && s.b[594]) {s.copy_ad(467, 468);}
        if (!s.b[592]) {s.store_sub_scaled_inputs_mixed_ai(469, A::offset(s.ad_value(468), p.p53), 1.0, 467, p.p37);s.store_sub_from_scalar(375, p.p52, 469);}
        s.store_scalar(368, (((((p.p379 * (if ((p.p380 / p.p376) > 1e-38) { (((p.p380 / p.p376)) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p.p376) / p.p376));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_div_scaled_value_by_product_mixed_aii(371, A::exp_scaled_input({
            if ((p.p380 / (p.p376 * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p.p380, A::scale(s.ad_value(213), p.p376)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p379), (1.0 / (p.p376) * 1.0 / (p.p376)), 213, 213, 1.0);s.store_scalar(369, (if (p.p37 == 1.0) { p.p1040 } else { p.p1039 }));s.store_scalar(370, (if (p.p37 == 1.0) { p.p1042 } else { p.p1041 }));s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p25)));s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p.p23) + p.p24)));s.store_scale(374, 213, ((-s.v[370]) * p.p376));s.store_scalar(369, ((s.v[369] * s.v[368]) * (((s.v[328] / p.p23) * s.v[327]) + (p.p28 / p.p3))));s.store_scalar(370, (s.v[370] * (-p.p376)));s.b[595] = (param_given[90] || param_given[94]);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (!param_given[90]);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[596]) {s.store_scalar(120, 0.53);}
        s.b[597] = (!param_given[94]);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[597]) {s.store_scalar(124, (-0.0186));}
        s.b[603] = (!param_given[87]);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });
        if (((!s.b[595]) && s.b[603]) && (p.p41 != 0.0)) {s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[595]) && s.b[603]) && (p.p41 == 0.0)) {s.store_scalar(467, 0.00077348);}
        if ((!s.b[595]) && s.b[603]) {s.store_add_scaled_product_indices(114, 118, 1.0, 467, 108, (-(s.v[117] * s.v[117])));}
        s.b[604] = (s.v[114] > 0.0);s.store_scalar(604, if s.b[604] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[604]) {s.store_neg(114, 114);}
        s.b[605] = (s.v[116] > 0.0);s.store_scalar(605, if s.b[605] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[605]) {s.store_scalar(116, (-s.v[116]));}
        s.b[606] = (!param_given[85]);s.store_scalar(606, if s.b[606] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[606]) {s.store_div_scaled_product_mixed_iai(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);}
        s.b[607] = (!param_given[86]);s.store_scalar(607, if s.b[607] { 1.0 } else { 0.0 });
        if ((!s.b[595]) && s.b[607]) {s.store_div_scaled_product_mixed_iai(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);}
        if (!s.b[595]) {s.store_sub(467, 112, 113);s.store_sub_mixed_ai(468, A::sqrt(A::sub(s.ad_value(118), s.ad_value(114))), 339);s.store_mul_sub_mixed_iai(469, 339, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), 339);s.store_div_scaled_product_add_scaled_denominator_indices(124, 467, 468, 1.0, 469, 2.0, 116, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(120, 113, 1.0, 124, A::sqrt(A::sub(s.ad_value(118), s.ad_value(116))), (-2.0));}
        s.store_offset(467, 265, s.v[328]);s.b[608] = (s.v[467] < 1e-8);s.store_scalar(608, if s.b[608] { 1.0 } else { 0.0 });
        if s.b[608] {s.store_scalar(467, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(346, 120, A::div(s.ad_value(264), s.ad_value(467)), 1.0, 1.0);s.b[609] = (!param_given[109]);s.store_scalar(609, if s.b[609] { 1.0 } else { 0.0 });s.b[610] = (param_given[108] || param_given[107]);s.store_scalar(610, if s.b[610] { 1.0 } else { 0.0 });
        if (s.b[609] && s.b[610]) {s.store_add_scaled_inputs_product_indices(152, 137, p.p37, 118, (-1.0), 346, 339, (-1.0));}
        if (s.b[609] && (!s.b[610])) {s.store_scalar(152, (-1.0));}
        s.b[611] = (!param_given[108]);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_add_scaled_inputs_product_indices(137, 152, p.p37, 118, p.p37, 346, 339, p.p37);}
        s.store_scale(376, 346, (p.p66 * 1.0 / (p.p67)));s.store_mul(468, 397, 341);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(342, 467, 1.0, 467, 467, 2.0);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(469, 467, 1.0, 467, 467, 2.0);s.store_add_scaled_product_indices(343, 193, 1.0, 192, 469, 1.0);s.store_div_mixed_ia(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[612] = (s.v[44] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if s.b[612] {s.store_scalar(44, 0.0);}
        s.store_scalar(467, ((s.v[474]) as f64).powf(p.p239));s.store_primal_offset(489, 44, s.v[475]);s.store_powf(468, 489, p.p240);s.store_add_ad(463, A::offset(A::div_from_scalar(p.p244, s.ad_value(468)), (p.p243 / s.v[467])), A::div_from_scalar(p.p245, A::scale(s.ad_value(468), s.v[467])));s.store_offset(231, 463, 1.0);s.store_scalar(467, ((s.v[474]) as f64).powf(p.p241));s.store_powf(468, 489, p.p242);s.store_add_ad(463, A::offset(A::div_from_scalar(p.p247, s.ad_value(468)), (p.p246 / s.v[467])), A::div_from_scalar(p.p248, A::scale(s.ad_value(468), s.v[467])));s.store_offset(232, 463, 1.0);s.store_sqrt_square_offset(232, 232, 1e-9);s.store_offset_scaled(233, 231, (1.0 + (p.p238 * s.v[430])), 1e-9);s.store_scalar(483, (1.0 / (p.p232 + (0.5 * s.v[474]))));s.store_scalar(484, (1.0 / (p.p233 + (0.5 * s.v[474]))));s.store_scalar(235, (s.v[483] + s.v[484]));s.store_scale_ad(234, A::div_from_scalar(p.p235, s.ad_value(233)), s.v[235]);s.b[613] = (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0))));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
        if s.b[613] {s.store_scalar(485, 0.0);s.store_scalar(486, 0.0);}
        s.b[614] = (s.v[45] < (-1.0));s.store_scalar(614, if s.b[614] { 1.0 } else { 0.0 });
        if (s.b[613] && s.b[614]) {s.store_scalar(45, (-1.0));}
        s.b[615] = (s.v[45] > 1.0);s.store_scalar(615, if s.b[615] { 1.0 } else { 0.0 });
        if ((s.b[613] && (!s.b[614])) && s.b[615]) {s.store_scalar(45, 1.0);}
        if ((s.b[613] && (!s.b[614])) && (!s.b[615])) {
        }
        if s.b[613] {s.store_scalar(495, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (s.b[613] && (s.v[495] < p.p3)) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[613] {s.store_primal_div_from_scalar_offset_scaled_input(616, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p4 + (0.5 * s.v[474])));s.store_primal_div_from_scalar_offset_scaled_input(617, (1.0 / p.p3), 495, (p.p6 + s.v[474]), (p.p5 + (0.5 * s.v[474])));s.store_primal_add(485, 485, 616);s.store_primal_add(486, 486, 617);s.store_primal_offset(495, 495, 1.0);}
        }
        if s.b[613] {s.store_primal_add(490, 485, 486);s.copy_ad(51, 490);s.store_mul_div_from_scalar_lhs_ad_indices(487, p.p235, 233, 490);s.store_div_scaled_offset_numerator_mixed_ia(467, 487, 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);s.store_mul(404, 337, 467);s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);s.store_mul(407, 338, 468);s.store_primal_offset(491, 490, (-s.v[235]));s.store_mul_div_from_scalar_lhs_ad_indices(488, p.p237, 232, 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(492, p.p249, A::powf(s.ad_value(232), p.p250), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(493, p.p251, A::powf(s.ad_value(232), p.p252), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(494, p.p253, A::powf(s.ad_value(232), p.p254), 491);s.store_add(408, 137, 488);s.store_add(402, 124, 492);s.store_add(400, 187, 493);s.store_add(401, 189, 494);}
        if (!s.b[613]) {s.copy_ad(404, 337);s.copy_ad(408, 137);s.copy_ad(407, 338);s.copy_ad(402, 124);s.copy_ad(400, 187);s.copy_ad(401, 189);s.store_scalar(51, 0.0);s.store_scalar(235, 0.0);s.store_scalar(45, 0.0);}
        s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));s.store_offset(408, 408, p.p20);s.store_offset(406, 152, (p.p37 * p.p20));s.store_scalar(52, (s.v[392] * p.p8));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scale(53, 43, p.p8);s.store_scalar(54, (s.v[392] * p.p7));s.store_scale(55, 43, p.p7);s.b[618] = (s.v[43] > 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (s.b[618] && s.b[619]) {s.store_sub(467, 323, 322);s.store_add_scaled_inputs(175, 322, 1.0, 467, p.p356);s.store_sub_from_scalar(468, s.v[52], 53);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p.p356));s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 53, 322, (-1.0));s.store_sub_from_scalar(468, s.v[54], 55);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p.p356));s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p.p356) * 0.3333333333333333), 55, 322, (-1.0));}
        if (s.b[618] && (!s.b[619])) {s.store_sub(467, 322, 323);s.store_add_scaled_inputs(175, 323, 1.0, 467, p.p356);s.store_offset(468, 53, (-s.v[52]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p.p356));s.store_scale(177, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));s.store_offset(468, 55, (-s.v[54]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p.p356));s.store_scale(179, 469, 1.0 / ((1.0 - p.p356)));s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p.p356) * 0.3333333333333333));}
        if (!s.b[618]) {s.store_scalar(175, 0.0);s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(56, 0.0);s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(57, 0.0);}
        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if s.b[620] {s.store_scalar(46, 1.0);}
        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p.p155 / p.p154))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p.p155 / p.p154)))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p.p357);s.store_scalar(468, (p.p10 - p.p2));s.b[621] = (s.v[468] > 0.0);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if s.b[621] {s.store_scale(58, 467, s.v[468]);}
        if (!s.b[621]) {s.store_scalar(58, 0.0);}
        s.store_scalar(468, (p.p9 - p.p2));s.b[622] = (s.v[468] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if s.b[622] {s.store_scale(59, 467, s.v[468]);}
        if (!s.b[622]) {s.store_scalar(59, 0.0);}
        s.store_scalar(61, (p.p131 * p.p11));s.b[623] = ((p.p429 == 1.0) && (s.v[61] < p.p431));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if s.b[623] {s.store_scalar(61, p.p431);}
        s.store_scalar(60, (p.p131 * p.p12));s.b[624] = ((p.p429 == 1.0) && (s.v[60] < p.p431));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if s.b[624] {s.store_scalar(60, p.p431);}
        s.b[625] = (s.v[36] < 1e-15);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if s.b[625] {s.store_scalar(36, 1e-15);}
        s.store_div_scalar_by_product_indices(467, (((-0.5) * s.v[327]) * s.v[327]), 36, 36, 1.0);s.b[626] = (s.v[467] > 100.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if s.b[626] {s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[627] = (s.v[467] < (-100.0));s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if ((!s.b[626]) && s.b[627]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[626]) && (!s.b[627])) {s.store_exp(468, 467);}
        s.copy_ad(351, 468);s.store_mul_scale_offset_mixed_ia(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), 1.0, (1.0 / s.v[327]));s.store_pow_indices(352, 467, 318);s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p.p343, 1.0);s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);s.b[628] = (s.v[354] < 1.0);s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(354, 1.0);}
        s.b[629] = (p.p41 == 0.0);s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if s.b[629] {s.store_scalar(62, (p.p66 - p.p68));}
        if (!s.b[629]) {s.store_scalar(498, (8.617087e-5 * p.p57));s.copy_ad(499, 498);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(500, 498, {
                if ((1e20 * s.v[108]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(108), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, 530, 2.0);
        }
        if (!s.b[629]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(501, 498, {
                if (s.v[108] > 1e-38) {
                    A::ln(s.ad_value(108))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 530, 2.0);
        }
        if (!s.b[629]) {s.store_sqrt(502, 501);s.store_add(464, 406, 501);s.store_scalar(503, (p.p37 * p.p56));s.store_scalar(467, (p.p60 * 8.85418e-12));}
        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[630]) {s.store_div_scaled_product_mixed_iia(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);s.store_mul_scale_offset_indices(469, 468, 471, 1.0, (-1.0));s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);s.store_offset_sub_from_scalar_ad(532, p.p1034, s.ad_value(470), (-0.05));s.store_sqrt_square_offset(473, 532, 0.224);s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p.p1034);s.store_sub(504, 503, 472);}
        if ((!s.b[629]) && (!s.b[630])) {s.copy_ad(504, 503);}
        if (!s.b[629]) {s.store_sub(506, 500, 501);s.copy_ad(470, 341);s.store_mul(509, 397, 470);s.store_mul(510, 397, 470);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * p.p54), 509, 1.0);}
        s.b[631] = (s.v[467] > (-100.0));s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[631]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[631])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);s.copy_ad(470, 96);s.store_div_scaled_inputs2_mixed_aii(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[632] = (s.v[471] >= (-0.5));s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[632]) {s.store_offset(511, 471, 1.0);}
        if ((!s.b[629]) && (!s.b[632])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);}
        s.b[633] = (s.v[378] > 0.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[633]) {s.store_offset_scaled(470, 378, 2.0, p.p54);}
        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_mixed_ia(471, 499, {
                            if ((p.p54 / s.v[470]) > 1e-38) {
                                A::ln(A::div_from_scalar(p.p54, s.ad_value(470)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[629]) && s.b[633]) {s.store_mul(519, 511, 471);}
        if ((!s.b[629]) && (!s.b[633])) {s.store_scalar(519, 0.0);}
        if (!s.b[629]) {s.store_mul(63, 129, 522);s.store_mul(523, 63, 506);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (p.p55 * p.p54)), 510, 1.0);}
        s.b[634] = (s.v[467] > (-100.0));s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[634]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[634])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_mul(467, 132, 469);s.store_mul(524, 467, 506);s.store_scalar(430, ((p.p57 / s.v[429]) - 1.0));s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p.p54), 1.0);s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p.p54));s.store_add_scaled_product_mixed_aii(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(464, 415, 501, 1.0, 127, p.p55, 1.0);s.store_scalar(517, 0.0);s.store_scalar(521, 0.0);s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p.p54), 1.0);s.copy_ad(514, 502);}
    }
}
