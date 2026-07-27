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
        if (!s.b[1454]) {s.copy_ad(815, 48);s.store_scalar(980, (-p[363]));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p[183]);s.store_scalar(976, ((((p[185] * s.v[350]) * p[155]) * p[3]) / 1e-7));s.store_scale(979, 976, p[362]);s.store_add_scaled_product_right_sub(976, 976, 1.0, 979, 409, 429, 1.0);s.store_scalar(977, ((((p[186] * s.v[349]) * p[155]) * p[3]) / 1e-7));s.store_scale(978, 977, p[364]);s.store_add_scaled_product_right_sub(977, 977, 1.0, 978, 409, 429, 1.0);s.store_scale(994, 815, 0.9);}
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
        if (!s.b[1454]) {s.store_add_scaled_product_indices(910, 987, (p[351] * p[3]), 976, 846, 1.0);s.copy_ad(815, 41);s.store_scalar(980, (-p[365]));s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);s.store_scalar(816, p[184]);s.store_scale(994, 815, 0.9);}
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
        if (!s.b[1454]) {s.store_add_scaled_product_indices(909, 988, (p[351] * p[3]), 977, 846, 1.0);}
        s.store_scale(853, 897, (-p[37]));s.store_scaled_sub(854, 819, 897, p[37]);s.b[1459] = (s.v[43] != 0.0);s.store_scalar(1459, if s.b[1459] { 1.0 } else { 0.0 });s.b[1460] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(1460, if s.b[1460] { 1.0 } else { 0.0 });s.b[1461] = (s.v[853] < s.v[322]);s.store_scalar(1461, if s.b[1461] { 1.0 } else { 0.0 });
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
        s.b[1467] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });s.b[1468] = (s.v[854] < s.v[322]);s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });
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
        s.store_add_scaled_product_indices(86, 86, 1.0, 58, 853, 1.0);s.store_add_scaled_product_indices(87, 87, 1.0, 59, 854, 1.0);s.b[1474] = (p[39] == 3.0);s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });
        if s.b[1474] {s.store_offset(843, 1019, 0.02);}
        if (!s.b[1474]) {s.store_offset(843, 820, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 237, s.v[349]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1475] = (p[39] == 3.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1475] {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1475]) {s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1476] = (p[39] == 3.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if s.b[1476] {s.store_offset(843, 1018, 0.02);}
        if (!s.b[1476]) {s.store_offset(843, 821, 0.02);}
        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));s.store_scaled_sub(845, 843, 844, 0.5);s.store_scale(846, 236, s.v[350]);s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));s.b[1477] = (p[39] == 3.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if s.b[1477] {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        if (!s.b[1477]) {s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));}
        s.b[1478] = (p[3] != 1.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if s.b[1478] {s.store_scale(895, 895, p[3]);s.store_scale(896, 896, p[3]);}
        s.b[1505] = (p[223] == 0.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });s.b[1506] = (p[223] == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });s.b[1507] = (p[223] == 2.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });s.b[1508] = (p[223] == 3.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if (s.b[1506] && (!s.b[1505])) {s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);s.store_square(843, 843);s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);s.store_square(848, 848);s.store_offset_scaled(1487, 848, (((p[227] * s.v[327])) * (p[229])), p[229]);s.store_offset_scaled(1488, 848, (((p[228] * s.v[327])) * (p[230])), p[230]);}
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
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_div_scaled_inputs3(1493, A::div(s.ad_value(844), s.ad_value(846)), 1.0, A::div_scaled_product(A::add_scaled_inputs(s.ad_value(844), 5.0, s.ad_value(845), 1.0), s.ad_value(847), 1.0, s.ad_value(848), 15.0), (-1.0), A::div_scaled_product_by_product(s.ad_value(847), s.ad_value(847), 1.0, s.ad_value(848), s.ad_value(845), 9.0), 1.0, A::mul3_scaled_output(s.ad_value(849), s.ad_value(849), s.ad_value(849), 6.0), 1.0);s.store_div(850, 843, 845);s.store_div_scaled_add_product_mixed_iaii(1494, 850, 1.0, A::square(s.ad_value(850)), 850, 0.3333333333333333, 849, 6.0);s.store_div(851, 72, 838);s.store_square(851, 851);s.store_offset_scaled(1490, 851, (((p[224] * s.v[892])) * (p[225])), p[225]);s.store_mul_scale_offset_mixed_ai(1498, A::div(s.ad_value(1494), A::sqrt(A::mul(s.ad_value(1492), s.ad_value(1493)))), 1490, 2.5316, 0.0);}
        s.b[1512] = (s.v[1498] > 1.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1512]) {s.store_scalar(1498, 1.0);}
        s.b[1513] = (s.v[1498] < 0.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) && s.b[1513]) {s.store_scalar(1498, 0.0);}
        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {s.store_offset_scaled(1487, 851, (((p[227] * s.v[892])) * (p[229])), p[229]);s.store_offset_scaled(1488, 851, (((p[228] * s.v[892])) * (p[230])), p[230]);s.store_mul3_affine_lhs(1492, 1492, 1487, 3.0, 0.0, 1487);s.store_mul3_affine_lhs(1493, 1493, 1488, 3.75, 0.0, 1488);s.store_div_scaled_product_offset_denominator_mixed_iia(1499, 880, 72, p[3], A::mul(s.ad_value(881), s.ad_value(887)), 1.0, 1.0);s.store_scale(1501, 396, (p[3] * (s.v[332] * s.v[331])));s.store_div_scaled_offset_numerator_mixed_ia(1497, 1499, 1.0, 1e-15, A::sqrt(A::div(s.ad_value(1493), s.ad_value(1492))), 1.0);}
        s.b[1514] = (p[223] != 3.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });s.b[1546] = ((p[429] != 2.0) && ((s.v[61] + p[136]) >= p[431]));s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });s.b[1547] = ((p[429] != 2.0) && ((s.v[60] + p[135]) >= p[431]));s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });s.b[1548] = (s.v[398] > 0.0);s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });s.b[1549] = (p[430] != 0.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if (s.b[1548] && s.b[1549]) {s.store_scale(88, 905, (p[37] * p[30]));s.store_scale(89, 906, (p[37] * p[30]));s.store_scale(90, 1024, (p[37] * p[30]));s.store_scale(91, 1023, (p[37] * p[30]));}
        if (s.b[1548] && (!s.b[1549])) {s.store_scale(88, 905, p[37]);s.store_scale(89, 906, p[37]);s.store_scale(90, 1024, p[37]);s.store_scale(91, 1023, p[37]);}
        if s.b[1548] {s.store_scale(92, 918, p[37]);s.store_scale(93, 919, p[37]);}
        s.b[1550] = (p[430] != 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if ((!s.b[1548]) && s.b[1550]) {s.store_scale(89, 905, (p[37] * p[30]));s.store_scale(88, 906, (p[37] * p[30]));s.store_scale(91, 1024, (p[37] * p[30]));s.store_scale(90, 1023, (p[37] * p[30]));}
        if ((!s.b[1548]) && (!s.b[1550])) {s.store_scale(89, 905, p[37]);s.store_scale(88, 906, p[37]);s.store_scale(91, 1024, p[37]);s.store_scale(90, 1023, p[37]);}
        if (!s.b[1548]) {s.store_scale(93, 918, p[37]);s.store_scale(92, 919, p[37]);}
        s.b[1551] = (p[430] != 0.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if s.b[1551] {s.store_scale(94, 1022, (p[37] * p[30]));s.store_scale(95, 1021, (p[37] * p[30]));}
        if (!s.b[1551]) {s.store_scale(94, 1022, p[37]);s.store_scale(95, 1021, p[37]);}
        s.b[1552] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });s.b[1553] = (p[39] == 3.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1554] = ((p[39] == 0.0) || (p[39] == 2.0));s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });s.b[1555] = ((p[39] == 0.0) || (p[39] == 1.0));s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });s.b[1556] = (p[39] == 2.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });s.b[1558] = (s.v[37] == 2.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });s.b[1559] = ((p[36] == 1.0) && (p[14] != 0.0));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });s.b[1560] = ((p[35] != 0.0) && (!true));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });s.b[1561] = true;s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });s.b[1562] = true;s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });s.b[1563] = (p[430] == 2.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });s.b[1564] = (p[430] == 2.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });s.b[1565] = ((p[35] != 0.0) && (!true));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = true;s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });s.b[1567] = true;s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });s.copy_ad(426, 916);s.copy_ad(427, 918);s.copy_ad(428, 919);s.store_add(425, 896, 895);s.store_sub(918, 427, 895);s.store_sub(919, 428, 896);s.store_add(916, 426, 425);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        let ctx_temp = ctx.temperature();s.store_scalar(409, (ctx_temp + p[0]));s.store_scalar(429, (p[126] + 273.15));s.store_scalar(36, p[336]);s.store_scalar(37, p[21]);s.store_scalar(38, p[348]);s.store_scalar(39, p[213]);s.store_scalar(40, p[127]);s.store_scalar(41, p[182]);s.store_scalar(42, p[350]);s.store_scalar(43, p[355]);s.store_scalar(44, p[234]);s.store_scalar(45, p[236]);s.store_scalar(46, p[373]);s.store_scalar(48, p[181]);
        if (p[41] != 0.0) {s.store_scalar(416, 3.9);s.store_scalar(415, p[45]);s.store_scalar(417, (8.85418e-12 * p[47]));s.store_primal_sqrt_scaled_input(419, 417, (2000000.0 * 1.602176462e-19));s.store_primal_div_scaled_inputs_indices(396, 416, 8.85418e-12, 415, 1.0);}
        if (p[41] == 0.0) {s.store_scalar(416, p[46]);s.store_scalar(415, p[66]);s.store_scalar(417, 1.03594e-10);s.store_scalar(419, 5.753e-12);s.store_scalar(396, (3.453133e-11 / p[66]));}
        s.b[431] = (s.v[37] == 2.0);s.store_scalar(431, if s.b[431] { 1.0 } else { 0.0 });
        if s.b[431] {s.store_scalar(399, 0.0);}
        s.b[456] = (!true);s.store_scalar(456, if s.b[456] { 1.0 } else { 0.0 });
        if ((!s.b[431]) && s.b[456]) {s.store_scalar(399, 0.0);}
        s.b[458] = (!true);s.store_scalar(458, if s.b[458] { 1.0 } else { 0.0 });s.b[459] = ((s.v[38] == 0.0) && (p[349] == 0.0));s.store_scalar(459, if s.b[459] { 1.0 } else { 0.0 });
        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && s.b[459]) {s.store_scalar(399, 2.0);}
        if ((((!s.b[431]) && (!s.b[456])) && s.b[458]) && (!s.b[459])) {s.store_scalar(399, 1.0);}
        s.b[460] = ((s.v[38] == 0.0) && (p[349] == 0.0));s.store_scalar(460, if s.b[460] { 1.0 } else { 0.0 });
        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && s.b[460]) {s.store_scalar(38, 1.0);s.store_scalar(399, 1.0);}
        if ((((!s.b[431]) && (!s.b[456])) && (!s.b[458])) && (!s.b[460])) {s.store_scalar(399, 1.0);}
        s.b[461] = param_given[213];s.store_scalar(461, if s.b[461] { 1.0 } else { 0.0 });
        if s.b[461] {s.store_scalar(39, p[213]);}
        if (!s.b[461]) {s.store_scalar(39, (((2.0 * 3.453133e-11) / 3.141592653589793) * (((1.0 + (4e-7 / p[66]))) as f64).ln()));}
        s.b[533] = (s.v[48] < 0.1);s.store_scalar(533, if s.b[533] { 1.0 } else { 0.0 });
        if s.b[533] {s.store_scalar(48, 0.1);}
        s.b[534] = (s.v[41] < 0.1);s.store_scalar(534, if s.b[534] { 1.0 } else { 0.0 });
        if s.b[534] {s.store_scalar(41, 0.1);}
        s.store_scalar(429, (p[126] + 273.15));s.store_scalar(476, (s.v[409] / s.v[429]));
        if (p[41] != 0.0) {s.store_primal_sqrt_mul_ad(397, A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415));}
        if (p[41] == 0.0) {s.store_scalar(397, ((((1.03594e-10 / 3.453133e-11) * p[66])) as f64).sqrt());}
        s.b[535] = (p[41] == 0.0);s.store_scalar(535, if s.b[535] { 1.0 } else { 0.0 });
        if s.b[535] {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (1.16 - (((0.000702 * s.v[429]) * s.v[429]) / (s.v[429] + 1108.0))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (1.16 - (((0.000702 * s.v[409]) * s.v[409]) / (s.v[409] + 1108.0))));s.copy_ad(395, 465);}
        if s.b[535] {s.store_sub_from_scalar_ad(530, ((if (((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt()) > 1e-38) { ((((14500000000.0 * (s.v[409] / 300.15)) * (((s.v[409] / 300.15)) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }) + 21.5565981), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0));}
        if (!s.b[535]) {s.store_scalar(480, (8.617087e-5 * s.v[429]));s.store_scalar(466, (p[49] - (((p[50] * s.v[429]) * s.v[429]) / (s.v[429] + p[51]))));s.copy_ad(394, 466);s.store_scalar(49, (8.617087e-5 * s.v[409]));s.store_scalar(465, (p[49] - (((p[50] * s.v[409]) * s.v[409]) / (s.v[409] + p[51]))));s.copy_ad(395, 465);}
        if (!s.b[535]) {s.store_offset_sub_ad(530, A::div_scaled_inputs(s.ad_value(466), 1.0, s.ad_value(480), 2.0), A::div_scaled_inputs(s.ad_value(465), 1.0, s.ad_value(49), 2.0), (if (((p[48] * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt()) > 1e-38) { ((((p[48] * (s.v[409] / s.v[429])) * (((s.v[409] / s.v[429])) as f64).sqrt())) as f64).ln() } else { (-87.49823353377374) }));}
        s.store_scalar(50, (p[16] * p[349]));s.store_scalar(474, p[1]);s.store_scalar(475, (p[2] / p[3]));s.store_scalar(467, ((s.v[474]) as f64).powf(p[190]));s.store_scalar(468, ((s.v[475]) as f64).powf(p[193]));s.store_scalar(463, (((p[188] / s.v[467]) + (p[191] / s.v[468])) + (p[194] / (s.v[467] * s.v[468]))));s.store_scalar(326, (p[187] + s.v[463]));s.store_scalar(463, (((p[189] / s.v[467]) + (p[192] / s.v[468])) + (p[195] / (s.v[467] * s.v[468]))));s.store_scalar(330, (p[217] + s.v[463]));s.store_scalar(215, (p[410] + s.v[463]));s.b[536] = (s.v[215] < 0.0);s.store_scalar(536, if s.b[536] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[536] {s.store_scalar(215, 0.0);}
        s.store_scalar(469, ((s.v[474]) as f64).powf(p[202]));s.store_scalar(470, ((s.v[475]) as f64).powf(p[205]));s.store_scalar(464, (((p[200] / s.v[469]) + (p[203] / s.v[470])) + (p[206] / (s.v[469] * s.v[470]))));s.store_scalar(325, (p[197] + s.v[464]));s.store_scalar(464, (((p[201] / s.v[469]) + (p[204] / s.v[470])) + (p[207] / (s.v[469] * s.v[470]))));s.store_scalar(329, (p[216] + s.v[464]));s.store_scalar(327, (p[1] - (2.0 * s.v[326])));s.store_scalar(328, (((p[2] / p[3]) - (p[22] * p[303])) - ((2.0 - p[22]) * s.v[325])));s.store_scalar(348, ((s.v[328] / p[23]) + p[24]));s.store_scalar(347, ((s.v[328] / p[23]) + p[25]));s.store_scalar(331, (p[1] - (2.0 * s.v[330])));s.store_scalar(332, (((p[2] / p[3]) - (p[22] * p[303])) - ((2.0 - p[22]) * s.v[329])));s.store_scalar(349, ((s.v[332] / p[23]) + p[24]));s.store_scalar(350, ((s.v[332] / p[23]) + p[25]));s.store_scalar(365, ((p[1] - (2.0 * s.v[330])) - p[360]));s.store_scalar(366, (s.v[365] + (2.0 * p[372])));s.store_scalar(112, p[85]);s.store_scalar(113, p[86]);s.store_scalar(114, p[87]);s.store_scalar(116, p[88]);s.store_scalar(117, p[89]);s.copy_ad(239, 39);s.store_scalar(240, p[214]);s.store_scalar(241, p[215]);s.b[543] = (s.v[241] == 0.0);s.store_scalar(543, if s.b[543] { 1.0 } else { 0.0 });
        if s.b[543] {s.store_scalar(333, 2.0);}
        if (!s.b[543]) {s.store_scalar(333, (1.0 + (((s.v[240] / s.v[327])) as f64).powf(s.v[241])));}
        s.b[544] = (p[65] == 1.0);s.store_scalar(544, if s.b[544] { 1.0 } else { 0.0 });
        if s.b[544] {s.store_scalar(477, (1e-6 / s.v[327]));s.store_scalar(478, (1e-6 / s.v[328]));s.store_scalar(479, (1e-12 / (s.v[327] * s.v[328])));}
        if (!s.b[544]) {s.store_scalar(477, (1.0 / s.v[327]));s.store_scalar(478, (1.0 / s.v[328]));s.store_scalar(479, (1.0 / (s.v[327] * s.v[328])));}
        s.store_add_scaled_inputs3_offset_indices(108, 477, p[488], 478, p[678], 479, p[868], p[82]);s.store_add_scaled_inputs3_offset_indices(109, 477, p[489], 478, p[679], 479, p[869], p[81]);s.store_add_scaled_inputs3_offset_indices(110, 477, p[490], 478, p[680], 479, p[871], p[83]);s.store_add_scaled_inputs3_offset_indices(111, 477, p[491], 478, p[681], 479, p[870], p[84]);s.store_add_scaled_inputs3_offset_indices(137, 477, p[492], 478, p[682], 479, p[872], p[108]);s.store_add_scaled_inputs3_offset_indices(152, 477, p[493], 478, p[683], 479, p[873], p[109]);s.store_add_scaled_inputs3_offset_indices(120, 477, p[494], 478, p[684], 479, p[874], p[90]);s.store_add_scaled_inputs3_offset_indices(124, 477, p[497], 478, p[687], 479, p[877], p[94]);s.store_add_scaled_inputs3_offset_indices(264, 477, p[495], 478, p[685], 479, p[875], p[300]);s.store_add_scaled_inputs3_offset_indices(265, 477, p[496], 478, p[686], 479, p[876], p[301]);s.store_add_scaled_inputs3_offset_indices(125, 477, p[498], 478, p[688], 479, p[878], p[95]);s.store_add_scaled_inputs3_offset_indices(126, 477, p[499], 478, p[689], 479, p[879], p[96]);s.store_add_scaled_inputs3_offset_indices(263, 477, p[500], 478, p[690], 479, p[880], p[371]);s.store_add_scaled_inputs3_offset_indices(127, 477, p[501], 478, p[691], 479, p[881], p[97]);s.store_add_scaled_inputs3_offset_indices(128, 477, p[1024], 478, p[1027], 479, p[1030], p[1021]);s.store_add_scaled_inputs3_offset_indices(377, 477, p[502], 478, p[692], 479, p[882], p[98]);s.store_add_scaled_inputs3_offset_indices(129, 477, p[503], 478, p[693], 479, p[883], p[99]);s.store_add_scaled_inputs3_offset_indices(130, 477, p[504], 478, p[694], 479, p[884], p[100]);s.store_add_scaled_inputs3_offset_indices(131, 477, p[505], 478, p[695], 479, p[885], p[101]);s.store_add_scaled_inputs3_offset_indices(132, 477, p[506], 478, p[696], 479, p[886], p[102]);s.store_add_scaled_inputs3_offset_indices(133, 477, p[507], 478, p[697], 479, p[887], p[103]);s.store_add_scaled_inputs3_offset_indices(133, 477, p[507], 478, p[697], 479, p[887], p[103]);s.store_add_scaled_inputs3_offset_indices(134, 477, p[508], 478, p[698], 479, p[888], p[104]);s.store_add_scaled_inputs3_offset_indices(144, 477, p[509], 478, p[699], 479, p[889], p[116]);s.store_add_scaled_inputs3_offset_indices(138, 477, p[511], 478, p[701], 479, p[891], p[110]);s.store_add_scaled_inputs3_offset_indices(140, 477, p[512], 478, p[702], 479, p[892], p[112]);s.store_add_scaled_inputs3_offset_indices(142, 477, p[513], 478, p[703], 479, p[893], p[114]);s.store_add_scaled_inputs3_offset_indices(101, 477, p[518], 478, p[708], 479, p[898], p[74]);s.store_add_scaled_inputs3_offset_indices(103, 477, p[519], 478, p[709], 479, p[899], p[76]);s.store_add_scaled_inputs3_offset_indices(104, 477, p[520], 478, p[710], 479, p[900], p[77]);s.store_add_scaled_inputs3_offset_indices(199, 477, p[521], 478, p[711], 479, p[901], p[208]);s.store_add_scaled_inputs3_offset_indices(200, 477, p[522], 478, p[712], 479, p[902], p[209]);s.store_add_scaled_inputs3_offset_indices(107, 477, p[523], 478, p[713], 479, p[903], p[80]);s.store_add_scaled_inputs3_offset_indices(266, 477, p[524], 478, p[714], 479, p[904], p[302]);s.store_add_scaled_inputs3_offset_indices(105, 477, p[525], 478, p[715], 479, p[905], p[78]);s.store_add_scaled_inputs3_offset_indices(106, 477, p[526], 478, p[716], 479, p[906], p[79]);s.store_add_scaled_inputs3_offset_indices(181, 477, p[527], 478, p[717], 479, p[907], p[132]);s.store_add_scaled_inputs3_offset_indices(170, 477, p[528], 478, p[718], 479, p[908], p[133]);s.store_add_scaled_inputs3_offset_indices(169, 477, p[529], 478, p[719], 479, p[909], p[134]);s.store_add_scaled_inputs3_offset_indices(184, 477, p[530], 478, p[720], 479, p[910], p[142]);s.store_add_scaled_inputs3_offset_indices(185, 477, p[531], 478, p[721], 479, p[911], p[143]);s.store_add_scaled_inputs3_offset_indices(183, 477, p[532], 478, p[722], 479, p[912], p[141]);s.store_add_scaled_inputs3_offset_indices(196, 477, p[533], 478, p[723], 479, p[913], p[196]);
        s.store_add_scaled_inputs3_offset_indices(100, 477, p[534], 478, p[724], 479, p[914], p[73]);s.store_add_scaled_inputs3_offset_indices(197, 477, p[535], 478, p[725], 479, p[915], p[198]);s.store_add_scaled_inputs3_offset_indices(198, 477, p[536], 478, p[726], 479, p[916], p[199]);s.store_add_scaled_inputs3_offset_indices(151, 477, p[537], 478, p[727], 479, p[917], p[125]);s.store_add_scaled_inputs3_offset_indices(187, 477, p[538], 478, p[728], 479, p[918], p[145]);s.store_add_scaled_inputs3_offset_indices(188, 477, p[539], 478, p[729], 479, p[919], p[146]);s.store_add_scaled_inputs3_offset_indices(189, 477, p[540], 478, p[730], 479, p[920], p[147]);s.store_add_scaled_inputs3_offset_indices(190, 477, p[541], 478, p[731], 479, p[921], p[148]);s.store_add_scaled_inputs3_offset_indices(136, 477, p[542], 478, p[732], 479, p[922], p[106]);s.store_add_scaled_inputs3_offset_indices(99, 477, p[543], 478, p[733], 479, p[923], p[72]);s.store_add_scaled_inputs3_offset_indices(96, 477, p[544], 478, p[734], 479, p[924], p[69]);s.store_add_scaled_inputs3_offset_indices(97, 477, p[545], 478, p[735], 479, p[925], p[70]);s.store_add_scaled_inputs3_offset_indices(98, 477, p[546], 478, p[736], 479, p[926], p[71]);s.store_add_scaled_inputs3_offset_indices(191, 477, p[547], 478, p[737], 479, p[927], p[149]);s.store_add_scaled_inputs3_offset_indices(192, 477, p[548], 478, p[738], 479, p[928], p[150]);s.store_add_scaled_inputs3_offset_indices(193, 477, p[549], 478, p[739], 479, p[929], p[151]);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(194, 477, p[550], 478, p[740], 479, p[930], p[152]);s.store_add_scaled_inputs3_offset_indices(135, 477, p[551], 478, p[741], 479, p[931], p[105]);s.store_add_scaled_inputs3_offset_indices(195, 477, p[552], 478, p[742], 479, p[932], p[153]);s.store_add_scaled_inputs3_offset_indices(180, 477, p[553], 478, p[743], 479, p[933], p[130]);s.store_add_scaled_inputs3_offset_indices(201, 477, p[554], 478, p[744], 479, p[934], p[218]);s.store_add_scaled_inputs3_offset_indices(267, 477, p[555], 478, p[745], 479, p[935], p[314]);s.store_add_scaled_inputs3_offset_indices(268, 477, p[558], 478, p[748], 479, p[938], p[315]);s.store_add_scaled_inputs3_offset_indices(269, 477, p[557], 478, p[747], 479, p[937], p[316]);s.store_add_scaled_inputs3_offset_indices(270, 477, p[560], 478, p[750], 479, p[940], p[317]);s.store_add_scaled_inputs3_offset_indices(271, 477, p[556], 478, p[746], 479, p[936], p[318]);s.store_add_scaled_inputs3_offset_indices(272, 477, p[559], 478, p[749], 479, p[939], p[319]);s.store_add_scaled_inputs3_offset_indices(202, 477, p[561], 478, p[751], 479, p[941], p[304]);s.store_add_scaled_inputs3_offset_indices(273, 477, p[562], 478, p[752], 479, p[942], p[305]);s.store_add_scaled_inputs3_offset_indices(274, 477, p[563], 478, p[753], 479, p[943], p[306]);s.store_add_scaled_inputs3_offset_indices(275, 477, p[564], 478, p[754], 479, p[944], p[307]);s.store_add_scaled_inputs3_offset_indices(276, 477, p[565], 478, p[755], 479, p[945], p[309]);s.store_add_scaled_inputs3_offset_indices(277, 477, p[566], 478, p[756], 479, p[946], p[321]);s.store_add_scaled_inputs3_offset_indices(278, 477, p[567], 478, p[757], 479, p[947], p[310]);s.store_add_scaled_inputs3_offset_indices(279, 477, p[568], 478, p[758], 479, p[948], p[311]);s.store_add_scaled_inputs3_offset_indices(280, 477, p[569], 478, p[759], 479, p[949], p[312]);s.store_add_scaled_inputs3_offset_indices(281, 477, p[570], 478, p[760], 479, p[950], p[313]);s.store_add_scaled_inputs3_offset_indices(282, 477, p[571], 478, p[761], 479, p[951], p[158]);s.store_add_scaled_inputs3_offset_indices(283, 477, p[572], 478, p[762], 479, p[952], p[159]);s.store_add_scaled_inputs3_offset_indices(284, 477, p[573], 478, p[763], 479, p[953], p[160]);s.store_add_scaled_inputs3_offset_indices(285, 477, p[574], 478, p[764], 479, p[954], p[161]);s.store_add_scaled_inputs3_offset_indices(286, 477, p[1025], 478, p[1028], 479, p[1031], p[1022]);s.store_add_scaled_inputs3_offset_indices(287, 477, p[575], 478, p[765], 479, p[955], p[162]);s.store_add_scaled_inputs3_offset_indices(288, 477, p[576], 478, p[766], 479, p[956], p[163]);s.store_add_scaled_inputs3_offset_indices(289, 477, p[577], 478, p[767], 479, p[957], p[164]);s.store_add_scaled_inputs3_offset_indices(290, 477, p[578], 478, p[768], 479, p[958], p[165]);s.store_add_scaled_inputs3_offset_indices(291, 477, p[579], 478, p[769], 479, p[959], p[166]);s.store_add_scaled_inputs3_offset_indices(292, 477, p[580], 478, p[770], 479, p[960], p[167]);s.store_add_scaled_inputs3_offset_indices(293, 477, p[581], 478, p[771], 479, p[961], p[168]);s.store_add_scaled_inputs3_offset_indices(294, 477, p[1026], 478, p[1029], 479, p[1032], p[1023]);s.store_add_scaled_inputs3_offset_indices(295, 477, p[582], 478, p[772], 479, p[962], p[169]);s.store_add_scaled_inputs3_offset_indices(296, 477, p[583], 478, p[773], 479, p[963], p[170]);s.store_add_scaled_inputs3_offset_indices(297, 477, p[584], 478, p[774], 479, p[964], p[171]);s.store_add_scaled_inputs3_offset_indices(298, 477, p[585], 478, p[775], 479, p[965], p[322]);s.store_add_scaled_inputs3_offset_indices(299, 477, p[586], 478, p[776], 479, p[966], p[323]);s.store_add_scaled_inputs3_offset_indices(300, 477, p[587], 478, p[777], 479, p[967], p[172]);s.store_add_scaled_inputs3_offset_indices(301, 477, p[588], 478, p[778], 479, p[968], p[173]);s.store_add_scaled_inputs3_offset_indices(302, 477, p[589], 478, p[779], 479, p[969], p[324]);s.store_add_scaled_inputs3_offset_indices(303, 477, p[590], 478, p[780], 479, p[970], p[325]);
        s.store_add_scaled_inputs3_offset_indices(304, 477, p[591], 478, p[781], 479, p[971], p[326]);s.store_add_scaled_inputs3_offset_indices(305, 477, p[592], 478, p[782], 479, p[972], p[327]);s.store_add_scaled_inputs3_offset_indices(306, 477, p[593], 478, p[783], 479, p[973], p[328]);s.store_add_scaled_inputs3_offset_indices(307, 477, p[594], 478, p[784], 479, p[974], p[329]);s.store_add_scaled_inputs3_offset_indices(308, 477, p[595], 478, p[785], 479, p[975], p[330]);s.store_add_scaled_inputs3_offset_indices(309, 477, p[596], 478, p[786], 479, p[976], p[331]);s.store_add_scaled_inputs3_offset_indices(310, 477, p[597], 478, p[787], 479, p[977], p[332]);s.store_add_scaled_inputs3_offset_indices(312, 477, p[599], 478, p[789], 479, p[979], p[334]);s.store_add_scaled_inputs3_offset_indices(311, 477, p[598], 478, p[788], 479, p[978], p[333]);s.store_add_scaled_inputs3_offset_indices(313, 477, p[600], 478, p[790], 479, p[980], p[335]);s.store_add_scaled_inputs3_offset_indices(313, 477, p[600], 478, p[790], 479, p[980], p[335]);s.store_add_scaled_inputs3_offset_indices(314, 477, p[601], 478, p[791], 479, p[981], p[337]);s.store_add_scaled_inputs3_offset_indices(315, 477, p[602], 478, p[792], 479, p[982], p[338]);s.store_add_scaled_inputs3_offset_indices(316, 477, p[603], 478, p[793], 479, p[983], p[339]);s.store_add_scaled_inputs3_offset_indices(317, 477, p[604], 478, p[794], 479, p[984], p[340]);s.store_add_scaled_inputs3_offset_indices(318, 477, p[605], 478, p[795], 479, p[985], p[341]);s.store_add_scaled_inputs3_offset_indices(319, 477, p[606], 478, p[796], 479, p[986], p[342]);s.store_add_scaled_inputs3_offset_indices(320, 477, p[607], 478, p[797], 479, p[987], p[344]);s.store_add_scaled_inputs3_offset_indices(321, 477, p[608], 478, p[798], 479, p[988], p[345]);s.store_add_scaled_inputs3_offset_indices(355, 477, p[609], 478, p[799], 479, p[989], p[346]);s.store_add_scaled_inputs3_offset_indices(356, 477, p[610], 478, p[800], 479, p[990], p[347]);s.store_add_scaled_inputs3_offset_indices(242, 477, p[443], 478, p[633], 479, p[823], p[157]);s.store_add_scaled_inputs3_offset_indices(243, 477, p[444], 478, p[634], 479, p[824], p[383]);s.store_add_scaled_inputs3_offset_indices(244, 477, p[445], 478, p[635], 479, p[825], p[384]);s.store_add_scaled_inputs3_offset_indices(246, 477, p[447], 478, p[637], 479, p[827], p[388]);s.store_add_scaled_inputs3_offset_indices(247, 477, p[448], 478, p[638], 479, p[828], p[389]);s.store_add_scaled_inputs3_offset_indices(245, 477, p[446], 478, p[636], 479, p[826], p[385]);s.store_add_scaled_inputs3_offset_indices(249, 477, p[449], 478, p[639], 479, p[829], p[390]);s.store_add_scaled_inputs3_offset_indices(253, 477, p[457], 478, p[647], 479, p[837], p[352]);s.store_add_scaled_inputs3_offset_indices(254, 477, p[467], 478, p[657], 479, p[847], p[358]);s.store_add_scaled_inputs3_offset_indices(255, 477, p[468], 478, p[658], 479, p[848], p[359]);s.store_add_scaled_inputs3_offset_indices(256, 477, p[469], 478, p[659], 479, p[849], p[174]);s.store_add_scaled_inputs3_offset_indices(257, 477, p[470], 478, p[660], 479, p[850], p[175]);s.store_add_scaled_inputs3_offset_indices(258, 477, p[471], 478, p[661], 479, p[851], p[176]);s.store_add_scaled_inputs3_offset_indices(259, 477, p[472], 478, p[662], 479, p[852], p[177]);s.store_add_scaled_inputs3_offset_indices(260, 477, p[473], 478, p[663], 479, p[853], p[178]);s.store_add_scaled_inputs3_offset_indices(261, 477, p[474], 478, p[664], 479, p[854], p[179]);s.store_add_scaled_inputs3_offset_indices(262, 477, p[475], 478, p[665], 479, p[855], p[180]);s.store_add_scaled_inputs3_offset_indices(237, 477, p[455], 478, p[645], 479, p[835], p[211]);s.store_add_scaled_inputs3_offset_indices(236, 477, p[454], 478, p[644], 479, p[834], p[210]);s.store_add_scaled_inputs3_offset_indices(238, 477, p[456], 478, p[646], 479, p[836], p[212]);s.store_add_scaled_inputs3_offset_indices(145, 477, p[458], 478, p[648], 479, p[838], p[118]);s.store_add_scaled_inputs3_offset_indices(146, 477, p[514], 478, p[704], 479, p[894], p[121]);
        s.store_add_scaled_inputs3_offset_indices(147, 477, p[515], 478, p[705], 479, p[895], p[122]);s.store_add_scaled_inputs3_offset_indices(148, 477, p[510], 478, p[700], 479, p[890], p[117]);s.store_add_scaled_inputs3_offset_indices(149, 477, p[517], 478, p[707], 479, p[897], p[119]);s.store_add_scaled_inputs3_offset_indices(150, 477, p[516], 478, p[706], 479, p[896], p[120]);s.store_add_scaled_inputs3_offset_indices(121, 477, p[459], 478, p[649], 479, p[839], p[91]);s.store_add_scaled_inputs3_offset_indices(123, 477, p[461], 478, p[651], 479, p[841], p[93]);s.store_add_scaled_inputs3_offset_indices(122, 477, p[460], 478, p[650], 479, p[840], p[92]);s.store_add_scaled_inputs3_offset_indices(139, 477, p[462], 478, p[652], 479, p[842], p[111]);s.store_add_scaled_inputs3_offset_indices(141, 477, p[463], 478, p[653], 479, p[843], p[113]);s.store_add_scaled_inputs3_offset_indices(143, 477, p[464], 478, p[654], 479, p[844], p[115]);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_scaled_inputs3_offset_indices(102, 477, p[465], 478, p[655], 479, p[845], p[75]);s.store_add_scaled_inputs3_offset_indices(186, 477, p[466], 478, p[656], 479, p[846], p[144]);s.store_add_scaled_inputs3_offset_indices(211, 477, p[484], 478, p[674], 479, p[864], p[406]);s.store_add_scaled_inputs3_offset_indices(203, 477, p[476], 478, p[666], 479, p[856], p[398]);s.store_add_scaled_inputs3_offset_indices(204, 477, p[477], 478, p[667], 479, p[857], p[399]);s.store_add_scaled_inputs3_offset_indices(205, 477, p[478], 478, p[668], 479, p[858], p[400]);s.store_add_scaled_inputs3_offset_indices(206, 477, p[479], 478, p[669], 479, p[859], p[401]);s.store_add_scaled_inputs3_offset_indices(207, 477, p[480], 478, p[670], 479, p[860], p[402]);s.store_add_scaled_inputs3_offset_indices(208, 477, p[481], 478, p[671], 479, p[861], p[403]);s.store_add_scaled_inputs3_offset_indices(209, 477, p[482], 478, p[672], 479, p[862], p[404]);s.store_add_scaled_inputs3_offset_indices(210, 477, p[483], 478, p[673], 479, p[863], p[405]);s.store_add_scaled_inputs3_offset_indices(212, 477, p[485], 478, p[675], 479, p[865], p[407]);s.store_add_scaled_inputs3_offset_indices(213, 477, p[486], 478, p[676], 479, p[866], p[408]);s.store_add_scaled_inputs3_offset_indices(229, 477, p[618], 478, p[808], 479, p[998], p[422]);s.store_add_scaled_inputs3_offset_indices(230, 477, p[619], 478, p[809], 479, p[999], p[423]);s.store_add_scaled_inputs3_offset_indices(216, 477, p[620], 478, p[810], 479, p[1000], p[413]);s.store_add_scaled_inputs3_offset_indices(217, 477, p[621], 478, p[811], 479, p[1001], p[433]);s.store_add_scaled_inputs3_offset_indices(218, 477, p[622], 478, p[812], 479, p[1002], p[434]);s.store_add_scaled_inputs3_offset_indices(219, 477, p[623], 478, p[813], 479, p[1003], p[414]);s.store_add_scaled_inputs3_offset_indices(220, 477, p[624], 478, p[814], 479, p[1004], p[415]);s.store_add_scaled_inputs3_offset_indices(221, 477, p[625], 478, p[815], 479, p[1005], p[416]);s.store_add_scaled_inputs3_offset_indices(222, 477, p[626], 478, p[816], 479, p[1006], p[417]);s.store_add_scaled_inputs3_offset_indices(223, 477, p[627], 478, p[817], 479, p[1007], p[418]);s.store_add_scaled_inputs3_offset_indices(224, 477, p[628], 478, p[818], 479, p[1008], p[419]);s.store_add_scaled_inputs3_offset_indices(225, 477, p[629], 478, p[819], 479, p[1009], p[420]);s.store_add_scaled_inputs3_offset_indices(226, 477, p[630], 478, p[820], 479, p[1010], p[421]);s.store_add_scaled_inputs3_offset_indices(227, 477, p[631], 478, p[821], 479, p[1011], p[411]);s.store_add_scaled_inputs3_offset_indices(228, 477, p[632], 478, p[822], 479, p[1012], p[412]);s.store_add_scaled_inputs3_offset_indices(322, 477, p[611], 478, p[801], 479, p[991], p[353]);s.store_add_scaled_inputs3_offset_indices(323, 477, p[612], 478, p[802], 479, p[992], p[354]);s.store_add_scaled_inputs3_offset_indices(324, 477, p[613], 478, p[803], 479, p[993], p[370]);s.store_add_scaled_inputs3_offset_indices(361, 477, p[614], 478, p[804], 479, p[994], p[366]);s.store_mul_powf_mixed_ia(361, 361, A::scale(s.ad_value(108), 5e-17), (-0.25));s.store_add_scaled_inputs3_offset_indices(362, 477, p[615], 478, p[805], 479, p[995], p[367]);s.store_add_scaled_inputs3_offset_indices(363, 477, p[616], 478, p[806], 479, p[996], p[368]);s.store_add_scaled_inputs3_offset_indices(364, 477, p[617], 478, p[807], 479, p[997], p[369]);s.store_add_scaled_inputs3_offset_indices(378, 477, p[259], 478, p[260], 479, p[261], p[258]);s.store_add_scaled_inputs3_offset_indices(379, 477, p[263], 478, p[264], 479, p[265], p[262]);s.store_add_scaled_inputs3_offset_indices(380, 477, p[267], 478, p[268], 479, p[269], p[266]);s.store_add_scaled_inputs3_offset_indices(381, 477, p[271], 478, p[272], 479, p[273], p[270]);s.store_add_scaled_inputs3_offset_indices(382, 477, p[275], 478, p[276], 479, p[277], p[274]);s.store_add_scaled_inputs3_offset_indices(383, 477, p[279], 478, p[280], 479, p[281], p[278]);s.store_add_scaled_inputs3_offset_indices(389, 477, p[436], 478, p[437], 479, p[438], p[435]);
        s.store_add_scaled_inputs3_offset_indices(390, 477, p[440], 478, p[441], 479, p[442], p[439]);s.store_add_scaled_inputs3_offset_indices(385, 477, p[286], 478, p[289], 479, p[292], p[285]);s.store_add_scaled_inputs3_offset_indices(386, 477, p[287], 478, p[290], 479, p[293], p[282]);s.store_add_scaled_inputs3_offset_indices(387, 477, p[288], 478, p[291], 479, p[294], p[284]);s.store_add_scaled_inputs3_offset_indices(250, 477, p[450], 478, p[640], 479, p[830], p[392]);s.store_add_scaled_inputs3_offset_indices(248, 477, p[451], 478, p[641], 479, p[831], p[393]);s.store_add_scaled_inputs3_offset_indices(251, 477, p[452], 478, p[642], 479, p[832], p[394]);s.store_add_scaled_inputs3_offset_indices(252, 477, p[453], 478, p[643], 479, p[833], p[395]);s.store_offset_scaled_ad(384, A::atan(s.ad_value(383)), 0.3183098861837907, 0.5);s.store_offset_scaled_ad(388, A::atan(s.ad_value(389)), 0.3183098861837907, 0.5);s.store_scalar(430, (s.v[476] - 1.0));s.copy_ad(153, 138);s.copy_ad(154, 140);s.copy_ad(155, 142);s.store_pow_from_scalar_ad(159, (s.v[328] * 1000000.0), s.ad_value(196));s.store_scalar(157, ((p[14] / (p[3] * (s.v[328] + p[377]))) * p[23]));s.store_scalar(158, ((p[15] * (p[3] * (s.v[328] + p[377]))) / p[23]));s.b[547] = (s.v[38] == 0.0);s.store_scalar(547, if s.b[547] { 1.0 } else { 0.0 });
        if s.b[547] {s.store_scalar(156, 0.0);}
        if (!s.b[547]) {s.store_div_scaled_inputs_mixed_ia(156, 38, (((p[17] * p[378]) * (s.v[328] * 1.0 / (p[23]))) * 1.0 / (p[3])), A::scale_offset(s.ad_value(38), 2.0, (p[378] * s.v[327])), 1.0);}
        s.store_scalar(345, (((((p[380] / p[376])) as f64).powf(p[379]) / p[376]) / p[376]));s.store_add_scaled_inputs(138, 138, 1.0, 139, s.v[430]);s.store_add_scaled_inputs(140, 140, 1.0, 141, s.v[430]);s.store_add_scaled_inputs(142, 142, 1.0, 143, s.v[430]);s.b[548] = (s.v[144] > 1.0);s.store_scalar(548, if s.b[548] { 1.0 } else { 0.0 });
        if s.b[548] {s.store_scale(144, 144, 0.0001);}
        s.store_mul_mixed_ia(337, 144, A::pow_from_scalar(s.v[476], s.ad_value(145)));s.store_sub_scaled_inputs(338, 101, 1.0, 102, s.v[430]);s.store_div_scaled_inputs2_indices(182, 181, 1.0, 186, s.v[430], 159, 1.0);s.b[549] = (p[429] == 1.0);s.store_scalar(549, if s.b[549] { 1.0 } else { 0.0 });
        if s.b[549] {s.store_scale(496, 159, p[3]);s.store_scale(497, 186, s.v[430]);s.store_add(468, 169, 497);s.store_offset(469, 497, p[140]);}
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
        if s.b[549] {s.store_div(173, 468, 496);s.store_add(470, 170, 497);s.store_offset(471, 497, p[139]);}
        s.b[552] = (s.v[470] < 0.0);s.store_scalar(552, if s.b[552] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[552]) {s.store_scalar(470, 0.0);}
        s.b[553] = (s.v[471] < 0.0);s.store_scalar(553, if s.b[553] { 1.0 } else { 0.0 });
        if (s.b[549] && s.b[553]) {s.store_scalar(471, 0.0);}
        if s.b[549] {s.store_div(174, 470, 496);}
        if (!s.b[549]) {s.store_scalar(173, 0.0);s.store_scalar(174, 0.0);}
        s.b[554] = param_given[128];s.store_scalar(554, if s.b[554] { 1.0 } else { 0.0 });
        if s.b[554] {s.store_scalar(47, p[128]);}
        s.b[555] = (param_given[217] && (p[217] > 0.0));s.store_scalar(555, if s.b[555] { 1.0 } else { 0.0 });
        if ((!s.b[554]) && s.b[555]) {s.store_sub_scaled_inputs(47, 396, p[217], 237, 1.0);}
        if ((!s.b[554]) && (!s.b[555])) {s.store_scale(47, 396, (0.6 * p[157]));}
        s.b[556] = param_given[127];s.store_scalar(556, if s.b[556] { 1.0 } else { 0.0 });
        if s.b[556] {s.store_scalar(40, p[127]);}
        s.b[557] = (param_given[217] && (p[217] > 0.0));s.store_scalar(557, if s.b[557] { 1.0 } else { 0.0 });
        if ((!s.b[556]) && s.b[557]) {s.store_sub_scaled_inputs(40, 396, p[217], 236, 1.0);}
        if ((!s.b[556]) && (!s.b[557])) {s.store_scale(40, 396, (0.6 * p[157]));}
        s.b[558] = (s.v[47] < 0.0);s.store_scalar(558, if s.b[558] { 1.0 } else { 0.0 });
        if s.b[558] {s.store_scalar(47, 0.0);}
        s.b[559] = (s.v[40] < 0.0);s.store_scalar(559, if s.b[559] { 1.0 } else { 0.0 });
        if s.b[559] {s.store_scalar(40, 0.0);}
        s.b[560] = (s.v[42] < 0.0);s.store_scalar(560, if s.b[560] { 1.0 } else { 0.0 });
        if s.b[560] {s.store_scalar(42, 0.0);}
        s.store_scaled_add(335, 47, 239, s.v[349]);s.store_scaled_add(334, 40, 239, s.v[350]);s.store_scale(336, 42, (s.v[331] * p[3]));s.b[561] = ((!param_given[82]) && param_given[85]);s.store_scalar(561, if s.b[561] { 1.0 } else { 0.0 });
        if s.b[561] {s.store_scale(467, 396, s.v[112]);s.store_scaled_mul(108, 467, 467, 3.021e22);}
        s.b[562] = (s.v[37] == 2.0);s.store_scalar(562, if s.b[562] { 1.0 } else { 0.0 });
        if (s.b[562] && (p[41] != 0.0)) {s.store_primal_scale(422, 417, ((((p[49] - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p[156] * p[156]))));}
        s.b[563] = (s.v[108] > s.v[422]);s.store_scalar(563, if s.b[563] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p[41] != 0.0)) && s.b[563]) {s.copy_ad(108, 422);}
        if (s.b[562] && (p[41] == 0.0)) {s.store_primal_scale(422, 417, ((((1.12 - 0.1) / 1.602176462e-19) * 2e-6) * 1.0 / ((p[155] * p[155]))));}
        s.b[564] = (s.v[108] > s.v[422]);s.store_scalar(564, if s.b[564] { 1.0 } else { 0.0 });
        if ((s.b[562] && (p[41] == 0.0)) && s.b[564]) {s.copy_ad(108, 422);}
        s.store_scalar(392, (3.453133e-11 / p[154]));
        if (p[41] != 0.0) {s.store_scalar(393, (1.03594e-10 / p[156]));}
        if (p[41] == 0.0) {s.store_scalar(393, (1.03594e-10 / p[155]));}
        if (p[41] != 0.0) {s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p[1021] / p[1])) * (1000000.0 * p[156]))));}
        if (p[41] == 0.0) {s.store_scale(420, 108, (1.602176462e-19 * ((1.0 + (p[1021] / p[1])) * (1000000.0 * p[155]))));}
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
            }, (-p[37]), 0.0);
        }
        if (!s.b[584]) {
            s.store_mul_sub_scaled_inputs_rhs_mixed_ai(160, 49, {
                if (((-s.v[108]) * s.v[109]) > 1e-38) {
                    A::ln(A::mul_scaled_lhs(s.ad_value(108), -1.0, s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (-p[37]), 530, ((2.0) * ((-p[37]))));
        }
        s.b[585] = (!param_given[353]);s.store_scalar(585, if s.b[585] { 1.0 } else { 0.0 });s.b[586] = (s.v[109] > 0.0);s.store_scalar(586, if s.b[586] { 1.0 } else { 0.0 });
        if (s.b[585] && s.b[586]) {
            s.store_scaled_offset_ad(322, A::add_scaled_products(s.ad_value(49), {
                if ((1e20 * s.v[109]) > 1e-38) {
                    A::ln_scaled_input(s.ad_value(109), 1e20)
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0, s.ad_value(49), s.ad_value(530), (-2.0)), (-0.3), (-p[37]));
        }
        s.b[587] = (s.v[109] < 0.0);s.store_scalar(587, if s.b[587] { 1.0 } else { 0.0 });
        if ((s.b[585] && (!s.b[586])) && s.b[587]) {
            s.store_scaled_offset_ad(322, A::mul(s.ad_value(49), {
                if (((-1e20) / s.v[109]) > 1e-38) {
                    A::ln(A::div_from_scalar((-1e20), s.ad_value(109)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }), 0.3, (-p[37]));
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
        }, 2.0, 530, 2.0);s.store_mul_scaled_sqrt_ad_rhs(482, 419, 1.0 / (s.v[392]), A::abs(s.ad_value(109)));s.b[588] = (!param_given[354]);s.store_scalar(588, if s.b[588] { 1.0 } else { 0.0 });s.b[589] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(589, if s.b[589] { 1.0 } else { 0.0 });
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
        }, 2.0, 530, 2.0);s.store_sqrt(339, 118);s.store_mul_sqrt_mixed_ia(340, 339, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_sqrt(341, 340);s.b[591] = (p[41] == 0.0);s.store_scalar(591, if s.b[591] { 1.0 } else { 0.0 });
        if s.b[591] {s.store_sqrt_scaled_input_ad(119, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(416)), s.ad_value(242)), p[66]);}
        if (!s.b[591]) {s.store_sqrt_ad(119, A::div_scaled_product3(s.ad_value(417), s.ad_value(242), s.ad_value(415), 1.0, s.ad_value(416), 8.85418e-12));}
        s.store_mul_sub_scaled_inputs_rhs_mixed_ai(115, 49, {
            if ((1e20 * s.v[108]) > 1e-38) {
                A::ln_scaled_input(s.ad_value(108), 1e20)
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, 1.0, 530, 2.0);s.store_sqrt_ad(367, A::div_scaled_product(s.ad_value(417), s.ad_value(108), (1.602176462e-19 * (1000000.0 * 0.5)), s.ad_value(118), 1.0));s.b[592] = (p[41] == 0.0);s.store_scalar(592, if s.b[592] { 1.0 } else { 0.0 });s.b[593] = (s.v[110] > 0.0);s.store_scalar(593, if s.b[593] { 1.0 } else { 0.0 });
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
        if (!s.b[592]) {s.store_sub_scaled_inputs_mixed_ai(469, A::offset(s.ad_value(468), p[53]), 1.0, 467, p[37]);s.store_sub_from_scalar(375, p[52], 469);}
        s.store_scalar(368, (((((p[379] * (if ((p[380] / p[376]) > 1e-38) { (((p[380] / p[376])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / p[376]) / p[376]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_div_scaled_value_by_product_mixed_aii(371, A::exp_scaled_input({
            if ((p[380] / (p[376] * s.v[213])) > 1e-38) {
                A::ln(A::div_from_scalar(p[380], A::scale(s.ad_value(213), p[376])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p[379]), (1.0 / (p[376]) * 1.0 / (p[376])), 213, 213, 1.0);s.store_scalar(369, (if (p[37] == 1.0) { p[1040] } else { p[1039] }));s.store_scalar(370, (if (p[37] == 1.0) { p[1042] } else { p[1041] }));s.store_scaled_mul(372, 215, 371, (s.v[369] * ((s.v[328] / p[23]) + p[25])));s.store_scaled_mul(373, 215, 371, (s.v[369] * ((s.v[328] / p[23]) + p[24])));s.store_scale(374, 213, ((-s.v[370]) * p[376]));s.store_scalar(369, ((s.v[369] * s.v[368]) * (((s.v[328] / p[23]) * s.v[327]) + (p[28] / p[3]))));s.store_scalar(370, (s.v[370] * (-p[376])));s.b[595] = (param_given[90] || param_given[94]);s.store_scalar(595, if s.b[595] { 1.0 } else { 0.0 });s.b[596] = (!param_given[90]);s.store_scalar(596, if s.b[596] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[596]) {s.store_scalar(120, 0.53);}
        s.b[597] = (!param_given[94]);s.store_scalar(597, if s.b[597] { 1.0 } else { 0.0 });
        if (s.b[595] && s.b[597]) {s.store_scalar(124, (-0.0186));}
        s.b[603] = (!param_given[87]);s.store_scalar(603, if s.b[603] { 1.0 } else { 0.0 });
        if (((!s.b[595]) && s.b[603]) && (p[41] != 0.0)) {s.store_scaled_div_from_scalar_ad(467, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[595]) && s.b[603]) && (p[41] == 0.0)) {s.store_scalar(467, 0.00077348);}
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
        if (s.b[609] && s.b[610]) {s.store_add_scaled_inputs_product_indices(152, 137, p[37], 118, (-1.0), 346, 339, (-1.0));}
        if (s.b[609] && (!s.b[610])) {s.store_scalar(152, (-1.0));}
        s.b[611] = (!param_given[108]);s.store_scalar(611, if s.b[611] { 1.0 } else { 0.0 });
        if s.b[611] {s.store_add_scaled_inputs_product_indices(137, 152, p[37], 118, p[37], 346, 339, p[37]);}
        s.store_scale(376, 346, (p[66] * 1.0 / (p[67])));s.store_mul(468, 397, 341);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(342, 467, 1.0, 467, 467, 2.0);s.store_ad_value(467, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(468), 1.0));s.store_add_scaled_product_indices(469, 467, 1.0, 467, 467, 2.0);s.store_add_scaled_product_indices(343, 193, 1.0, 192, 469, 1.0);s.store_div_mixed_ia(391, 380, A::exp_scaled_input(s.ad_value(381), (if (s.v[327] > 1e-38) { ((s.v[327]) as f64).ln() } else { (-87.49823353377374) })));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[612] = (s.v[44] < 0.0);s.store_scalar(612, if s.b[612] { 1.0 } else { 0.0 });
        if s.b[612] {s.store_scalar(44, 0.0);}
        s.store_scalar(467, ((s.v[474]) as f64).powf(p[239]));s.store_primal_offset(489, 44, s.v[475]);s.store_powf(468, 489, p[240]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[244], s.ad_value(468)), (p[243] / s.v[467])), A::div_from_scalar(p[245], A::scale(s.ad_value(468), s.v[467])));s.store_offset(231, 463, 1.0);s.store_scalar(467, ((s.v[474]) as f64).powf(p[241]));s.store_powf(468, 489, p[242]);s.store_add_ad(463, A::offset(A::div_from_scalar(p[247], s.ad_value(468)), (p[246] / s.v[467])), A::div_from_scalar(p[248], A::scale(s.ad_value(468), s.v[467])));s.store_offset(232, 463, 1.0);s.store_sqrt_square_offset(232, 232, 1e-9);s.store_offset_scaled(233, 231, (1.0 + (p[238] * s.v[430])), 1e-9);s.store_scalar(483, (1.0 / (p[232] + (0.5 * s.v[474]))));s.store_scalar(484, (1.0 / (p[233] + (0.5 * s.v[474]))));s.store_scalar(235, (s.v[483] + s.v[484]));s.store_scale_ad(234, A::div_from_scalar(p[235], s.ad_value(233)), s.v[235]);s.b[613] = (((p[4] > 0.0) && (p[5] > 0.0)) && ((p[3] == 1.0) || ((p[3] > 1.0) && (p[6] > 0.0))));s.store_scalar(613, if s.b[613] { 1.0 } else { 0.0 });
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
            let t0: f64 = if (s.b[613] && (s.v[495] < p[3])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if s.b[613] {s.store_primal_div_from_scalar_offset_scaled_input(616, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[4] + (0.5 * s.v[474])));s.store_primal_div_from_scalar_offset_scaled_input(617, (1.0 / p[3]), 495, (p[6] + s.v[474]), (p[5] + (0.5 * s.v[474])));s.store_primal_add(485, 485, 616);s.store_primal_add(486, 486, 617);s.store_primal_offset(495, 495, 1.0);}
        }
        if s.b[613] {s.store_primal_add(490, 485, 486);s.copy_ad(51, 490);s.store_mul_div_from_scalar_lhs_ad_indices(487, p[235], 233, 490);s.store_div_scaled_offset_numerator_mixed_ia(467, 487, 1.0, 1.0, A::offset(s.ad_value(234), 1.0), 1.0);s.store_mul(404, 337, 467);s.store_div_scaled_offset_numerator(468, A::mul(s.ad_value(45), s.ad_value(487)), 1.0, 1.0, A::offset(A::mul(s.ad_value(45), s.ad_value(234)), 1.0), 1.0);s.store_mul(407, 338, 468);s.store_primal_offset(491, 490, (-s.v[235]));s.store_mul_div_from_scalar_lhs_ad_indices(488, p[237], 232, 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(492, p[249], A::powf(s.ad_value(232), p[250]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(493, p[251], A::powf(s.ad_value(232), p[252]), 491);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(494, p[253], A::powf(s.ad_value(232), p[254]), 491);s.store_add(408, 137, 488);s.store_add(402, 124, 492);s.store_add(400, 187, 493);s.store_add(401, 189, 494);}
        if (!s.b[613]) {s.copy_ad(404, 337);s.copy_ad(408, 137);s.copy_ad(407, 338);s.copy_ad(402, 124);s.copy_ad(400, 187);s.copy_ad(401, 189);s.store_scalar(51, 0.0);s.store_scalar(235, 0.0);s.store_scalar(45, 0.0);}
        s.store_scale(403, 402, (p[66] * 1.0 / (p[67])));s.store_offset(408, 408, p[20]);s.store_offset(406, 152, (p[37] * p[20]));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(52, (s.v[392] * p[8]));s.store_scale(53, 43, p[8]);s.store_scalar(54, (s.v[392] * p[7]));s.store_scale(55, 43, p[7]);s.b[618] = (s.v[43] > 0.0);s.store_scalar(618, if s.b[618] { 1.0 } else { 0.0 });s.b[619] = (((s.v[109] > 0.0) && (p[37] > 0.0)) || ((s.v[109] < 0.0) && (p[37] < 0.0)));s.store_scalar(619, if s.b[619] { 1.0 } else { 0.0 });
        if (s.b[618] && s.b[619]) {s.store_sub(467, 323, 322);s.store_add_scaled_inputs(175, 322, 1.0, 467, p[356]);s.store_sub_from_scalar(468, s.v[52], 53);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(56, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 53, 322, (-1.0));s.store_sub_from_scalar(468, s.v[54], 55);s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_products_indices(57, 467, 468, ((1.0 + p[356]) * 0.3333333333333333), 55, 322, (-1.0));}
        if (s.b[618] && (!s.b[619])) {s.store_sub(467, 322, 323);s.store_add_scaled_inputs(175, 323, 1.0, 467, p[356]);s.store_offset(468, 53, (-s.v[52]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(176, 469, 1.0 / (p[356]));s.store_scale(177, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(56, 323, (-s.v[52]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));s.store_offset(468, 55, (-s.v[54]));s.store_div_scaled_value_by_product_indices(469, 468, 1.0, 467, 467, 1.0);s.store_scale(178, 469, 1.0 / (p[356]));s.store_scale(179, 469, 1.0 / ((1.0 - p[356])));s.store_add_scaled_product_indices(57, 323, (-s.v[54]), 467, 468, ((1.0 + p[356]) * 0.3333333333333333));}
        if (!s.b[618]) {s.store_scalar(175, 0.0);s.store_scalar(176, 0.0);s.store_scalar(177, 0.0);s.store_scalar(56, 0.0);s.store_scalar(178, 0.0);s.store_scalar(179, 0.0);s.store_scalar(57, 0.0);}
        s.b[620] = ((s.v[46] < 1.0) || (s.v[46] > 2.0));s.store_scalar(620, if s.b[620] { 1.0 } else { 0.0 });
        if s.b[620] {s.store_scalar(46, 1.0);}
        s.store_scale_ad(467, {
            if ((s.v[46] * (1.0 + (p[155] / p[154]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(46), (1.0 + (p[155] / p[154])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, p[357]);s.store_scalar(468, (p[10] - p[2]));s.b[621] = (s.v[468] > 0.0);s.store_scalar(621, if s.b[621] { 1.0 } else { 0.0 });
        if s.b[621] {s.store_scale(58, 467, s.v[468]);}
        if (!s.b[621]) {s.store_scalar(58, 0.0);}
        s.store_scalar(468, (p[9] - p[2]));s.b[622] = (s.v[468] > 0.0);s.store_scalar(622, if s.b[622] { 1.0 } else { 0.0 });
        if s.b[622] {s.store_scale(59, 467, s.v[468]);}
        if (!s.b[622]) {s.store_scalar(59, 0.0);}
        s.store_scalar(61, (p[131] * p[11]));s.b[623] = ((p[429] == 1.0) && (s.v[61] < p[431]));s.store_scalar(623, if s.b[623] { 1.0 } else { 0.0 });
        if s.b[623] {s.store_scalar(61, p[431]);}
        s.store_scalar(60, (p[131] * p[12]));s.b[624] = ((p[429] == 1.0) && (s.v[60] < p[431]));s.store_scalar(624, if s.b[624] { 1.0 } else { 0.0 });
        if s.b[624] {s.store_scalar(60, p[431]);}
        s.b[625] = (s.v[36] < 1e-15);s.store_scalar(625, if s.b[625] { 1.0 } else { 0.0 });
        if s.b[625] {s.store_scalar(36, 1e-15);}
        s.store_div_scalar_by_product_indices(467, (((-0.5) * s.v[327]) * s.v[327]), 36, 36, 1.0);s.b[626] = (s.v[467] > 100.0);s.store_scalar(626, if s.b[626] { 1.0 } else { 0.0 });
        if s.b[626] {s.store_scaled_offset(468, 467, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[627] = (s.v[467] < (-100.0));s.store_scalar(627, if s.b[627] { 1.0 } else { 0.0 });
        if ((!s.b[626]) && s.b[627]) {s.store_scalar(468, 3.720075976e-44);}
        if ((!s.b[626]) && (!s.b[627])) {s.store_exp(468, 467);}
        s.copy_ad(351, 468);s.store_mul_scale_offset_mixed_ia(467, 319, A::div_from_scalar(1.0, s.ad_value(36)), 1.0, (1.0 / s.v[327]));s.store_pow_indices(352, 467, 318);s.store_offset_scaled_ad(353, A::pow(s.ad_value(467), s.ad_value(253)), p[343], 1.0);s.store_add_scaled_inputs(354, 320, 1.0, 321, s.v[327]);s.b[628] = (s.v[354] < 1.0);s.store_scalar(628, if s.b[628] { 1.0 } else { 0.0 });
        if s.b[628] {s.store_scalar(354, 1.0);}
        s.b[629] = (p[41] == 0.0);s.store_scalar(629, if s.b[629] { 1.0 } else { 0.0 });
        if s.b[629] {s.store_scalar(62, (p[66] - p[68]));}
        if (!s.b[629]) {s.store_scalar(498, (8.617087e-5 * p[57]));s.copy_ad(499, 498);}
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
        if (!s.b[629]) {s.store_sqrt(502, 501);s.store_add(464, 406, 501);s.store_scalar(503, (p[37] * p[56]));s.store_scalar(467, (p[60] * 8.85418e-12));}
        s.b[630] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[503] > s.v[464])) && (s.v[467] != 0.0));s.store_scalar(630, if s.b[630] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[630]) {s.store_div_scaled_product_mixed_iia(468, 417, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(471, A::div_scaled_inputs2(s.ad_value(503), 2.0, s.ad_value(467), (-2.0), s.ad_value(468), 1.0), 1.0);s.store_mul_scale_offset_indices(469, 468, 471, 1.0, (-1.0));s.store_div_scaled_product_indices(470, 469, 469, 0.5, 468, 1.0);s.store_offset_sub_from_scalar_ad(532, p[1034], s.ad_value(470), (-0.05));s.store_sqrt_square_offset(473, 532, 0.224);s.store_offset_add_scaled_inputs_indices(472, 532, (-0.5), 473, (-0.5), p[1034]);s.store_sub(504, 503, 472);}
        if ((!s.b[629]) && (!s.b[630])) {s.copy_ad(504, 503);}
        if (!s.b[629]) {s.store_sub(506, 500, 501);s.copy_ad(470, 341);s.store_mul(509, 397, 470);s.store_mul(510, 397, 470);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * p[54]), 509, 1.0);}
        s.b[631] = (s.v[467] > (-100.0));s.store_scalar(631, if s.b[631] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[631]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[631])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(522, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_div_scaled_product_indices(469, 100, 417, 1.0, 340, 1.0);s.copy_ad(470, 96);s.store_div_scaled_inputs2_mixed_aii(471, A::add_scaled_product(s.ad_value(469), 1.0, s.ad_value(470), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[632] = (s.v[471] >= (-0.5));s.store_scalar(632, if s.b[632] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[632]) {s.store_offset(511, 471, 1.0);}
        if ((!s.b[629]) && (!s.b[632])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 471, 8.0, 3.0);s.store_mul_scale_offset_rhs(511, 467, 471, 3.0, 1.0);}
        s.b[633] = (s.v[378] > 0.0);s.store_scalar(633, if s.b[633] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[633]) {s.store_offset_scaled(470, 378, 2.0, p[54]);}
        if ((!s.b[629]) && s.b[633]) {
            s.store_mul_mixed_ia(471, 499, {
                            if ((p[54] / s.v[470]) > 1e-38) {
                                A::ln(A::div_from_scalar(p[54], s.ad_value(470)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[629]) && s.b[633]) {s.store_mul(519, 511, 471);}
        if ((!s.b[629]) && (!s.b[633])) {s.store_scalar(519, 0.0);}
        if (!s.b[629]) {s.store_mul(63, 129, 522);s.store_mul(523, 63, 506);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (p[55] * p[54])), 510, 1.0);}
        s.b[634] = (s.v[467] > (-100.0));s.store_scalar(634, if s.b[634] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[634]) {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if ((!s.b[629]) && (!s.b[634])) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[629]) {s.store_mul(467, 132, 469);s.store_mul(524, 467, 506);s.store_scalar(430, ((p[57] / s.v[429]) - 1.0));s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (p[54]), 1.0);s.store_add_scaled_inputs(468, 121, 1.0, 122, 1.0 / (p[54]));s.store_add_scaled_product_mixed_aii(520, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(502)), 1.0, 468, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(464, 415, 501, 1.0, 127, p[55], 1.0);s.store_scalar(517, 0.0);s.store_scalar(521, 0.0);s.store_sqrt_offset_scaled_input(518, 377, 1.0 / (p[54]), 1.0);s.copy_ad(514, 502);}
    }
}
