#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1172]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(646), s.v[387])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1172])) {s.store_scalar(454, 100000000.0);}
        s.b[1173] = ((s.v[388] * s.v[647]) > 0.0);s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1173]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1173])) {s.store_scalar(455, 100000000.0);}
        s.b[1174] = ((s.v[389] * s.v[648]) > 0.0);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1174]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p[822], A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1174])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(654, 454, 455, 456);}
        s.b[1175] = ((((s.v[654] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1175]) {s.store_primal_exp_scaled_input(655, 654, s.v[371]);}
        s.b[1176] = ((s.v[654] * s.v[371]) < 0.0);s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1175])) && s.b[1176]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(655, 1e-100, (-230.25850929940458), A::scale(s.ad_value(654), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1171] && (!s.b[1175])) && (!s.b[1176])) {s.store_primal_scaled_offset_ad(655, A::mul_offset_rhs(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(654), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(654), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.store_scalar(396, s.v[393]);s.store_scalar(397, s.v[394]);s.store_scalar(398, s.v[395]);s.store_scalar(399, p[831]);s.store_scalar(400, p[832]);s.store_scalar(401, p[833]);s.store_scalar(402, p[828]);s.store_scalar(403, p[829]);s.store_scalar(404, p[830]);}
        s.b[1177] = (s.v[646] == 0.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1177]) {s.store_scalar(396, (s.v[394] + s.v[395]));s.store_scalar(399, (0.9 * (p[832]).min(p[833])));s.store_scalar(402, (p[829] + p[830]));}
        s.b[1178] = (s.v[647] == 0.0);s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1178]) {s.store_scalar(397, (s.v[393] + s.v[395]));s.store_scalar(400, (0.9 * (p[831]).min(p[833])));s.store_scalar(403, (p[828] + p[830]));}
        s.b[1179] = (s.v[648] == 0.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1179]) {s.store_scalar(398, (s.v[393] + s.v[394]));s.store_scalar(401, (0.9 * (p[831]).min(p[832])));s.store_scalar(404, (p[828] + p[829]));}
        if s.b[1171] {s.store_min3(656, 396, 397, 398);s.store_primal_scale(657, 656, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(658, 656, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);s.store_primal_offset_min_ad(659, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
        s.b[1180] = ((s.v[563] * s.v[673]) > 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1180]) {s.store_primal_scaled_ln_ad(454, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(563), s.ad_value(673))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1180])) {s.store_scalar(454, 100000000.0);}
        s.b[1181] = ((s.v[564] * s.v[674]) > 0.0);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1181]) {s.store_primal_scaled_ln_ad(455, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1181])) {s.store_scalar(455, 100000000.0);}
        s.b[1182] = ((s.v[565] * s.v[675]) > 0.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1171] && s.b[1182]) {s.store_primal_scaled_ln_ad(456, A::offset(A::div_from_scalar(p[822], A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[370]);}
        if (s.b[1171] && (!s.b[1182])) {s.store_scalar(456, 100000000.0);}
        if s.b[1171] {s.store_min3(681, 454, 455, 456);}
        s.b[1183] = ((((s.v[681] * s.v[371])) as f64).abs() < 230.25850929940458);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1183]) {s.store_primal_exp_scaled_input(682, 681, s.v[371]);}
        s.b[1184] = ((s.v[681] * s.v[371]) < 0.0);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1171] && (!s.b[1183])) && s.b[1184]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(682, 1e-100, (-230.25850929940458), A::scale(s.ad_value(681), s.v[371]), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((s.b[1171] && (!s.b[1183])) && (!s.b[1184])) {s.store_primal_scaled_offset_ad(682, A::mul_offset_rhs(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(681), s.v[371], (-230.25850929940458)), A::scale_offset(s.ad_value(681), ((s.v[371]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if s.b[1171] {s.copy_ad(396, 569);s.copy_ad(397, 570);s.copy_ad(398, 571);s.copy_ad(399, 511);s.copy_ad(400, 512);s.copy_ad(401, 513);s.copy_ad(402, 508);s.copy_ad(403, 509);s.copy_ad(404, 510);}
        s.b[1185] = (s.v[673] == 0.0);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1185]) {s.store_primal_add(396, 570, 571);s.store_primal_scale_ad(399, A::min(s.ad_value(512), s.ad_value(513)), 0.9);s.store_primal_add(402, 509, 510);}
        s.b[1186] = (s.v[674] == 0.0);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1186]) {s.store_primal_add(397, 569, 571);s.store_primal_scale_ad(400, A::min(s.ad_value(511), s.ad_value(513)), 0.9);s.store_primal_add(403, 508, 510);}
        s.b[1187] = (s.v[675] == 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1187]) {s.store_primal_add(398, 569, 570);s.store_primal_scale_ad(401, A::min(s.ad_value(511), s.ad_value(512)), 0.9);s.store_primal_add(404, 508, 509);}
        if s.b[1171] {s.store_min3(683, 396, 397, 398);s.store_primal_scale(684, 683, 0.1);s.store_max3(377, 399, 400, 401);s.store_primal_mul_scale_offset_mixed_ia(685, 683, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(377))), -1.0, 1.0);s.store_primal_offset_min_ad(686, A::min(s.ad_value(402), s.ad_value(403)), s.ad_value(404), (-0.05));}
        s.b[1188] = (s.v[474] == 1.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if (s.b[1171] && s.b[1188]) {s.store_primal_add_scaled_inputs3_indices(501, 646, (s.v[414] * p[929]), 647, (s.v[415] * p[929]), 648, (s.v[416] * p[929]));}
        s.b[1523] = ((s.v[646] * s.v[414]) <= s.v[501]);s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1523]) {s.store_scalar(651, 0.0);}
        s.b[1524] = ((s.v[647] * s.v[415]) <= s.v[501]);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1524]) {s.store_scalar(652, 0.0);}
        s.b[1525] = ((s.v[648] * s.v[416]) <= s.v[501]);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1525]) {s.store_scalar(653, 0.0);}
        if (s.b[1171] && s.b[1188]) {s.store_primal_mul_mixed_ia(501, 553, A::add_scaled_products3(s.ad_value(673), s.ad_value(581), 1.0, s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0));}
        s.b[1813] = ((s.v[673] * s.v[581]) <= s.v[501]);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1813]) {s.store_scalar(678, 0.0);}
        s.b[1814] = ((s.v[674] * s.v[582]) <= s.v[501]);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1814]) {s.store_scalar(679, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1815] = ((s.v[675] * s.v[583]) <= s.v[501]);s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if ((s.b[1171] && s.b[1188]) && s.b[1815]) {s.store_scalar(680, 0.0);}
        s.store_scalar(2027, 0.0);s.store_scalar(2028, 0.0);s.store_scalar(2029, 0.0);s.store_scalar(1937, 1.0);s.store_scalar(1936, 0.0);s.b[2102] = (s.v[0] == 1.0);s.store_scalar(2102, if s.b[2102] { 1.0 } else { 0.0 });
        if s.b[2102] {s.store_voltage(825, ctx, nodes, Some(5), Some(6));s.store_voltage(826, ctx, nodes, Some(7), Some(6));s.store_voltage(827, ctx, nodes, Some(6), Some(8));s.store_scaled_voltage(832, ctx, nodes, Some(6), Some(10), -1.0);s.store_scaled_voltage(833, ctx, nodes, Some(7), Some(11), -1.0);}
        if (!s.b[2102]) {s.store_scaled_voltage(825, ctx, nodes, Some(5), Some(6), -1.0);s.store_scaled_voltage(826, ctx, nodes, Some(7), Some(6), -1.0);s.store_scaled_voltage(827, ctx, nodes, Some(6), Some(8), -1.0);s.store_voltage(832, ctx, nodes, Some(6), Some(10));s.store_voltage(833, ctx, nodes, Some(7), Some(11));}
        s.store_add(829, 825, 827);s.copy_ad(834, 825);s.copy_ad(835, 827);s.store_add(836, 826, 827);s.store_sub(837, 825, 826);s.store_scale(1817, 834, (-s.v[355]));s.store_scale(1818, 837, (-s.v[355]));s.store_scaled_sub(1819, 829, 700, (-s.v[355]));s.store_scalar(831, 1.0);s.b[2103] = (s.v[826] < 0.0);s.store_scalar(2103, if s.b[2103] { 1.0 } else { 0.0 });
        if s.b[2103] {s.store_scalar(831, (-1.0));s.store_sub(825, 825, 826);s.store_add(827, 827, 826);s.store_neg(826, 826);}
        s.store_add(828, 826, 827);s.store_div_scaled_product_offset_denominator_mixed_iia(830, 826, 826, 1.0, A::sqrt_square_offset(s.ad_value(826), 0.01), 0.1, 1.0);s.store_add_scaled_inputs4_mixed_iiai(2107, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(739))), (-0.5), 737, 1.0);s.copy_ad(1820, 2107);s.store_add_scaled_inputs4_mixed_iiai(2030, 827, 1.0, 2107, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2107)), s.ad_value(738))), (-(-0.5)), 741, 1.0);s.copy_ad(1821, 2030);s.store_scalar(2031, 0.0);s.b[2263] = ((p[45] != 0.0) && (s.v[184] != 1.0));s.store_scalar(2263, if s.b[2263] { 1.0 } else { 0.0 });
        if s.b[2263] {s.store_add_scaled_inputs3_indices(2032, 2030, 1.0, 826, 0.5, 830, (-0.5));s.store_sub_mixed_ai(2033, A::sqrt(A::add(s.ad_value(2032), s.ad_value(728))), 736);s.store_offset_div_scaled_inputs2_indices(2027, 2033, 2.0, 743, (-2.0), 744, 1.0, (-1.0));s.store_add_scaled_product_mixed_iaa(2034, 2033, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(184), s.ad_value(744), 0.25), A::add(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 0.4804530139182)), (-1.0));s.store_add_scaled_square_product_indices(2035, 2034, 1.0, 736, 2034, 2.0);s.store_add_scaled_inputs3_indices(2030, 2035, 1.0, 826, (-0.5), 830, (-(-0.5)));s.store_sub(2031, 1821, 2030);}
        s.copy_ad(2104, 728);s.copy_ad(2105, 738);s.copy_ad(2106, 729);s.copy_ad(2108, 2030);s.copy_ad(2112, 2031);s.copy_ad(2109, 720);s.copy_ad(2110, 777);s.store_add_scaled_inputs3_indices(2111, 829, 1.0, 2112, (-1.0), 700, -1.0);s.store_add_scaled_inputs3_indices(2113, 2108, 1.0, 826, 0.5, 830, (-0.5));s.store_scalar(2125, 1.0);s.b[2264] = (s.v[190] > 0.0);s.store_scalar(2264, if s.b[2264] { 1.0 } else { 0.0 });
        if s.b[2264] {s.store_primal_scale(2116, 2104, s.v[361]);s.store_scale(2117, 2113, s.v[361]);s.store_scale(2118, 2111, s.v[361]);s.store_offset_div_scaled_inputs_sqrt_rhs(2028, 2106, 0.5, 2116, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2264] {s.store_add_scaled_product_mixed_iia(2029, 2116, 1.0, 2106, A::sqrt(s.ad_value(2116)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2119, A::div_scaled_inputs2(s.ad_value(2118), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2116, 0.5, A::offset(s.ad_value(191), 1.0), 2117, (-1.0));s.store_primal_offset_scaled(2120, 2116, 0.5, 2.0);s.store_add(2121, 2116, 2117);s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2118), 1.0, s.ad_value(2121), (-1.0), s.ad_value(2106), A::sqrt(s.ad_value(2121)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2116), s.ad_value(2106)), A::sqrt(s.ad_value(2116)))), 2.0);s.store_add_scaled_inputs(2122, 2028, 2.0, 2120, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2119, 0.5, 2122, 0.5, 2119, 2122, 20.0, 0.5);s.store_add_scaled_inputs3_indices(2029, 2118, 2.0, 2117, (-2.0), 2120, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2123, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2123, 0.5, 2120, 0.5, 2123, 2120, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2124, 2028, 0.5, 2120, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2120), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(2029, 702, A::div(s.ad_value(2124), s.ad_value(2120)), 1.0, 1.0);}
        s.b[2265] = (s.v[2029] > (-230.25850929940458));s.store_scalar(2265, if s.b[2265] { 1.0 } else { 0.0 });
        if (s.b[2264] && s.b[2265]) {s.store_exp(2125, 2029);}
        if (s.b[2264] && (!s.b[2265])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2125, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_offset_mul(2126, 701, 2125, 1.0);s.store_scale(2127, 2126, s.v[715]);s.store_mul_ad_product_rhs(2128, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2113)), 1.0));s.store_mul_scale_offset_indices(2129, 2127, 2128, 1.0, 1.0);s.store_div_from_scalar(2130, 1.0, 2129);s.store_mul_mixed_ia(2114, 2106, A::sqrt_scaled_input(s.ad_value(2130), s.v[715]));s.store_square(2115, 2114);s.store_div_from_scalar(2131, 1.0, 2115);s.store_mul(2132, 2108, 2130);s.store_mul(2133, 2111, 2130);s.store_div_scaled_value_offset_denominator(2134, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2135, 196, 2134, A::offset(A::mul(s.ad_value(198), s.ad_value(2113)), 1.0));s.store_mul(2136, 2104, 2130);s.store_sqrt_square_add(2028, 2107, 2105);s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2107), s.ad_value(2135))), s.ad_value(2105));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2137, 2130, 2135, 0.5, 2028, 0.5, 2029, ((-1.0) * (0.5)), 0.0);s.store_add(2138, 2136, 2132);s.store_sub(2139, 2138, 2137);s.b[2266] = (p[45] > 0.0);s.store_scalar(2266, if s.b[2266] { 1.0 } else { 0.0 });s.b[2267] = (((s.v[2139]) as f64).abs() < 1e-5);s.store_scalar(2267, if s.b[2267] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2266] && s.b[2267]) {s.store_offset_ad(2140, A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2139), 1.0, A::scale(s.ad_value(2139), 0.3125), 0.5)), 1.0);}
        s.b[2268] = (s.v[2139] < 460.51701859880916);s.store_scalar(2268, if s.b[2268] { 1.0 } else { 0.0 });
        if ((s.b[2266] && (!s.b[2267])) && s.b[2268]) {s.store_exp_neg_input(2154, 2139);}
        if ((s.b[2266] && (!s.b[2267])) && (!s.b[2268])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2266] && (!s.b[2267])) {s.store_scalar(2027, (if (s.v[2139] > 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[2266] && (!s.b[2267])) {s.store_offset_ad(2140, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2114), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2139))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2139), 1.0, s.ad_value(2154))), 2.0), 1.0);}
        if (!s.b[2266]) {s.store_offset_div_scaled_inputs_sqrt_rhs(2140, 2114, 0.5, 2139, 1.0, 1.0);}
        s.store_add_scaled_value_products_mixed_iiaia(2141, 2139, 1.0, 2114, A::sqrt(s.ad_value(2139)), 1.0, 2140, A::ln(A::offset(s.ad_value(2140), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2142, 2133, 1.0, 2141, (-1.0), 2140, 1.0);s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2115)), 1.0)), (-1.0));s.store_scalar(2147, 0.0);s.store_scalar(2149, 1.0);s.b[2269] = (s.v[2142] > (-30.0));s.store_scalar(2269, if s.b[2269] { 1.0 } else { 0.0 });
        if s.b[2269] {s.store_offset_mul(2143, 2140, 2142, (-1.0));s.store_scaled_add_mixed_ia(2027, 2143, A::sqrt_square_offset(s.ad_value(2143), 10.0), 0.5);s.store_sub_mixed_ia(2144, 2142, A::ln(s.ad_value(2027)));s.store_scaled_add_mixed_ia(2145, 2144, A::sqrt_square_offset(s.ad_value(2144), 2.0), 0.5);}
        s.b[2270] = ((s.v[2142] - s.v[2145]) < 230.25850929940458);s.store_scalar(2270, if s.b[2270] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2270]) {s.store_exp_sub(2027, 2142, 2145);}
        if (s.b[2269] && (!s.b[2270])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2142), s.ad_value(2145)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if s.b[2269] {s.store_div(2146, 2027, 2140);s.store_sub_mixed_ai(2027, A::scaled_offset(s.ad_value(2145), 1.0, 2.0), 2146);}
        s.b[2271] = (s.v[2146] > 1e-6);s.store_scalar(2271, if s.b[2271] { 1.0 } else { 0.0 });
        if (s.b[2269] && s.b[2271]) {s.store_mul_scale_offset_mixed_ia(2147, 2140, A::sub(s.ad_value(2145), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2146), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2146), 1.0)), 1.0, 1.0);}
        if (s.b[2269] && (!s.b[2271])) {s.store_mul_ad_affine_product_rhs(2147, 2140, s.ad_value(2146), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);}
        if s.b[2269] {s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2133, 0.5, 2147, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2133), s.ad_value(2147)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2269] {s.store_mul_scaled_offset_ad_rhs(2148, 2115, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2115)), s.ad_value(2027), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2149, 2148, 2148, 1.0, 2147, 1.0);s.store_add_scaled_product_indices(2139, 2138, 1.0, 2149, 2137, (-1.0));}
        s.store_offset_scaled(2150, 2114, 0.7071067811865475, 1.0);s.store_scale(2151, 2150, 1e-5);s.store_div_from_scalar(2152, 1.0, 2150);s.store_scalar(2259, 0.0);s.store_scalar(2153, 0.0);s.b[2272] = (s.v[2139] < 460.51701859880916);s.store_scalar(2272, if s.b[2272] { 1.0 } else { 0.0 });
        if s.b[2272] {s.store_exp_neg_input(2154, 2139);}
        if (!s.b[2272]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2154, 1e-200, 2139, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2273] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2273, if s.b[2273] { 1.0 } else { 0.0 });
        if s.b[2273] {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2153, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2154)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        s.b[2274] = (s.v[2133] < (-s.v[2151]));s.store_scalar(2274, if s.b[2274] { 1.0 } else { 0.0 });
        if ((!s.b[2273]) && s.b[2274]) {s.store_neg(2241, 2133);s.store_scaled_mul(2242, 2241, 2152, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2243, 2242, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2238, 2241, 2243);s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::offset(s.ad_value(2243), 1.0), 1.0);s.store_sub_scaled_inputs(2245, 2238, 2.0, 2115, 1.0);s.store_sub_ln_mul_lhs(2246, 2244, 2131, 2243);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.5, s.ad_value(2244), 1.0), 1.0);s.store_add_mixed_ia(2247, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::sub_scaled_inputs(A::square(s.ad_value(2245)), 0.3333333333333333, s.ad_value(2244), 1.0))), 1.0));}
        s.b[2275] = (s.v[2247] < 230.25850929940458);s.store_scalar(2275, if s.b[2275] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && s.b[2274]) && s.b[2275]) {s.store_exp(2248, 2247);}
        if (((!s.b[2273]) && s.b[2274]) && (!s.b[2275])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2248, 2247, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((!s.b[2273]) && s.b[2274]) {s.store_div_from_scalar(2249, 1.0, 2248);s.store_div_from_scalar_offset_square(2238, 1.0, 2247, 2.0);s.store_mul_square_lhs(2250, 2247, 2238);s.store_mul3_affine_lhs(2251, 2247, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2241, 2247);s.store_mul(2239, 2154, 2249);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2273]) && s.b[2274]) {s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2239), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2154), 1.0, s.ad_value(2251)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2248), 1.0, s.ad_value(2247), (-1.0), s.ad_value(2239), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::sub(A::offset(s.ad_value(2247), (-1.0)), s.ad_value(2250)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2248), 1.0, s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2153, 2247, -1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_scaled_input(2255, 1.0, 2114, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2256, 2255, A::mul_scaled_lhs(s.ad_value(2150), 1.25, s.ad_value(2255)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2257, 2133, 2152, A::offset(A::mul(s.ad_value(2256), s.ad_value(2133)), 1.0));}
        s.b[2276] = ((-s.v[2257]) > (-230.25850929940458));s.store_scalar(2276, if s.b[2276] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2276]) {s.store_exp_neg_input(2238, 2257);}
        if (((!s.b[2273]) && (!s.b[2274])) && (!s.b[2276])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2238, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2257)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar(2258, 1.0, 2238);s.store_add_scaled_inputs_product_mixed_iiia(2259, 2133, 1.0, 2115, 0.5, 2114, A::sqrt(A::add_scaled_inputs3(s.ad_value(2133), 1.0, s.ad_value(2115), 0.25, s.ad_value(2258), -1.0)), (-1.0));s.store_offset(2260, 2139, 3.0);s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
        if ((!s.b[2273]) && (!s.b[2274])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2154] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2139, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2277] = (s.v[2262] < 230.25850929940458);s.store_scalar(2277, if s.b[2277] { 1.0 } else { 0.0 });
        if (((!s.b[2273]) && (!s.b[2274])) && s.b[2277]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2154, 2248);}
        s.b[2278] = (s.v[2262] > (s.v[2139] - 230.25850929940458));s.store_scalar(2278, if s.b[2278] { 1.0 } else { 0.0 });
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && s.b[2278]) {s.store_exp_sub(2248, 2262, 2139);s.store_div(2249, 2154, 2248);}
        if ((((!s.b[2273]) && (!s.b[2274])) && (!s.b[2277])) && (!s.b[2278])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((!s.b[2273]) && (!s.b[2274])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2154), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2154), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2153, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        s.store_scalar(2156, 0.0);s.store_scalar(2157, 0.0);s.store_scalar(2158, 0.0);s.store_scalar(2159, 0.0);s.store_scalar(2160, 0.0);s.store_scalar(2161, 0.0);s.store_scalar(2162, 0.0);s.store_scalar(2163, 1.0);s.store_scalar(2164, 1.0);s.store_sub(2165, 2133, 2153);s.store_scalar(2166, 0.0);s.store_mul(2167, 2129, 2165);s.store_scalar(2168, 1.0);s.store_scalar(2169, 1.0);s.store_scalar(2173, 1.0);s.store_scalar(2174, 1.0);s.store_scalar(2176, 1.0);s.b[2279] = (s.v[2133] > 0.0);s.store_scalar(2279, if s.b[2279] { 1.0 } else { 0.0 });
        if s.b[2279] {s.store_div_from_scalar_offset_square(2027, 1.0, 2153, 2.0);s.store_mul_square_lhs(2155, 2153, 2027);s.store_mul3_affine_lhs(2156, 2153, 2027, 4.0, 0.0, 2027);s.store_mul_ad_product_lhs_mixed_ai(2157, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2155), 12.0), 2027, 2027);s.store_scalar(2158, 0.0);}
        s.b[2280] = (s.v[2153] < 230.25850929940458);s.store_scalar(2280, if s.b[2280] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2280]) {s.store_exp(2158, 2153);s.store_div_from_scalar(2159, 1.0, 2158);s.store_mul(2158, 2154, 2158);}
        s.b[2281] = (s.v[2153] > (s.v[2139] - 230.25850929940458));s.store_scalar(2281, if s.b[2281] { 1.0 } else { 0.0 });
        if ((s.b[2279] && (!s.b[2280])) && s.b[2281]) {s.store_exp_sub(2158, 2153, 2139);s.store_div(2159, 2154, 2158);}
        if ((s.b[2279] && (!s.b[2280])) && (!s.b[2281])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2158, 1e-100, A::sub(s.ad_value(2139), s.ad_value(2153)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2159, 1e-100, 2153, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2279] {s.store_add_scaled_product_mixed_iia(2160, 2158, 1.0, 2154, A::add(A::offset(s.ad_value(2153), 1.0), s.ad_value(2155)), (-1.0));}
        s.b[2282] = (s.v[2153] < 1e-5);s.store_scalar(2282, if s.b[2282] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2282]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2161, 2153, 1.0, 2153, 1.0, 2153, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2160, A::mul3(s.ad_value(2154), s.ad_value(2153), s.ad_value(2153)), 2153, A::scale_offset(s.ad_value(2153), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2153), 1.0, A::scale(s.ad_value(2153), 0.25), 0.3333333333333333));s.store_scaled_mul(2162, 2153, 2027, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2163, 2114, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2153), 0.5)), 1.0, A::square(s.ad_value(2153)), 0.16666666666666666), 0.7071067811865475, 2027, 1.0, 1.0);}
        if (s.b[2279] && (!s.b[2282])) {s.store_add_offset_lhs(2161, 2153, (-1.0), 2159);s.store_sqrt(2162, 2161);s.store_offset_scaled_ad(2163, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2159)), s.ad_value(2162)), 0.5, 1.0);}
        if s.b[2279] {s.store_div_scaled_offset_numerator(2164, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2113)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2113)), 1.0), 1.0);}
        s.b[2283] = (s.v[2160] > 1e-100);s.store_scalar(2283, if s.b[2283] { 1.0 } else { 0.0 });
        if (s.b[2279] && s.b[2283]) {s.store_mul_sqrt_mixed_ia(2165, 2114, A::add(s.ad_value(2161), s.ad_value(2160)));s.store_div_scaled_product3_mixed_iiia(2166, 2115, 2160, 2129, 1.0, A::add_scaled_product(s.ad_value(2165), 1.0, s.ad_value(2114), s.ad_value(2162), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2279] && s.b[2283]) {s.store_mul3_lhs(2167, 2162, 2114, 2129);}
        s.b[2284] = (s.v[217] < 0.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {s.store_offset_mul(2168, 217, 2113, 1.0);}
        s.b[2285] = (s.v[218] < 0.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2166, 1.0);}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2166, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul_product3_indices(2170, 2166, 757, 2168, 2169, 1.0);s.store_mul_add_scaled_product_rhs_indices(2171, 774, 2167, 1.0, 775, 2166, 1.0);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2173, 2164, A::offset(s.ad_value(2172), 1.0), 2170);}
        s.b[2286] = (s.v[221] < 0.0);s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {s.store_offset_mul(2174, 221, 2113, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul(2029, 2166, 2174);s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2287] = (s.v[222] < 0.0);s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {s.store_offset_mul(2176, 222, 2175, 1.0);}
        s.copy_ad(1822, 2111);s.copy_ad(1823, 2113);s.copy_ad(1824, 2129);s.copy_ad(1825, 2130);s.copy_ad(1826, 2114);s.copy_ad(1827, 2115);s.copy_ad(1828, 2131);s.copy_ad(1829, 2133);s.copy_ad(1830, 2138);s.copy_ad(1831, 2139);s.copy_ad(1832, 2150);s.copy_ad(1833, 2151);s.copy_ad(1834, 2152);s.copy_ad(1835, 2259);s.copy_ad(1836, 2154);s.copy_ad(1837, 2153);s.copy_ad(1838, 2156);s.copy_ad(1839, 2157);s.copy_ad(1840, 2158);s.copy_ad(1841, 2159);s.copy_ad(1842, 2161);s.copy_ad(1843, 2160);s.copy_ad(1844, 2162);s.copy_ad(1845, 2163);s.copy_ad(1846, 2164);s.copy_ad(1847, 2165);s.copy_ad(1848, 2166);s.copy_ad(1849, 2167);s.copy_ad(1850, 2168);s.copy_ad(1851, 2169);s.copy_ad(1852, 2173);s.copy_ad(1853, 2174);s.copy_ad(1854, 2176);s.store_scalar(2178, 0.0);s.store_scale(2177, 2129, 4.60517018598809);s.copy_ad(2194, 2177);s.copy_ad(2195, 826);s.store_mul(2196, 826, 2130);s.copy_ad(2200, 2153);s.store_scalar(2201, 0.0);s.store_scalar(2204, 0.0);s.copy_ad(2206, 2159);s.copy_ad(2207, 2161);s.copy_ad(2209, 2160);s.copy_ad(2210, 2167);s.copy_ad(2211, 2153);s.copy_ad(2212, 2159);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        s.copy_ad(2214, 2160);s.copy_ad(2215, 2161);s.store_sub(2216, 2133, 2153);s.store_scalar(2217, 1.0);s.store_scalar(2219, 1.0);s.store_scalar(2218, 0.0);s.copy_ad(2228, 2166);s.store_mul(2232, 2216, 2129);s.store_scalar(2229, 0.0);s.copy_ad(2230, 2167);s.store_scalar(2235, 0.0);s.store_scalar(2234, 1.0);s.copy_ad(2237, 2109);s.copy_ad(2236, 2232);s.b[2288] = (s.v[2133] > 0.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });s.b[2289] = (s.v[2160] > 1e-100);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2289]) {s.store_mul(2237, 2109, 2176);s.store_div(2178, 2237, 2173);s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);s.store_div_scaled_product_by_product_indices(2027, 2115, 2158, 1.0, 2179, 2179, 1.0);}
        s.b[2290] = (s.v[2027] > 0.0001);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {s.store_sub_from_scalar(2028, 1.0, 2027);}
        s.b[2291] = (s.v[2028] < 1e-10);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {s.store_scalar(2029, 1.0);}
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {s.store_scale(2029, 2027, 0.5);}
        if (s.b[2288] && s.b[2289]) {s.store_mul(2180, 2029, 2179);}
        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_scaled_mul(2181, 2129, 2180, 0.475);s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));s.store_scaled_add_mixed_ia(2182, 2027, A::sqrt_square_offset(s.ad_value(2027), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2183, 2166, (-1.0), 2129, 2165, 1.0, A::offset(s.ad_value(2163), (-1.0)), 2181, 1.0);s.store_offset_div_scaled_product_indices(2184, 2115, 2129, 0.5, 2183, 1.0, 1.0);s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));s.store_mul_mixed_ai(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);s.store_div(2027, 2182, 2183);s.store_mul_pow_mixed_iaa(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));s.store_mul_div_scaled_product_mixed_iiai(2029, 2186, 707, A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2183, 1.0);s.store_mul_product3_indices(2187, 2182, 757, 2168, 2169, 1.0);s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);}
        s.b[2293] = (s.v[2027] < 230.25850929940458);s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);}
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {s.copy_ad(2028, 2027);}
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_mul_scale_offset_mixed_ia(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt_square_offset(s.ad_value(2188), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {s.copy_ad(2189, 2180);}
        if (s.b[2288] && s.b[2289]) {s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);}
        s.b[2294] = (s.v[0] == (-1.0));s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {s.store_div_mixed_ia(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));}
        if (s.b[2288] && s.b[2289]) {s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);s.store_mul(2027, 2191, 2190);s.store_mul_ad_product_rhs_mixed_ia(2192, 2189, 2191, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));s.store_scale(2193, 2192, 0.99);s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);}
        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_mixed_iia(2194, 2129, 2193, A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if (s.b[2288] && (!s.b[2289])) {s.copy_ad(2194, 2177);}
        if s.b[2288] {s.store_offset(2027, 2110, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);s.store_mul(2196, 2195, 2130);s.store_add(2197, 2139, 2196);}
        s.b[2295] = (s.v[2196] < 460.51701859880916);s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2295]) {s.store_exp_neg_input(2198, 2196);}
        if (s.b[2288] && (!s.b[2295])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2288] {s.store_mul(2199, 2154, 2198);}
        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2296]) {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2200, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        if (s.b[2288] && (!s.b[2296])) {s.store_offset(2260, 2197, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
        if (s.b[2288] && (!s.b[2296])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2197, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2297] = (s.v[2262] < 230.25850929940458);s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2199, 2248);}
        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {s.store_exp_sub(2248, 2262, 2197);s.store_div(2249, 2199, 2248);}
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2288] && (!s.b[2296])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if s.b[2288] {s.store_sub(2201, 2200, 2153);}
        s.b[2299] = (s.v[2201] < 1e-10);s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2299]) {s.store_add_scaled_inputs_product_mixed_iiia(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2115, A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));s.store_scaled_div_mixed_ia(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2200, 2153, 2201);}
        if s.b[2288] {s.store_mul(2204, 2201, 2129);s.store_div_scaled_product_offset_denominator_mixed_iia(2205, 2200, 2200, 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);}
        s.b[2300] = (s.v[2200] < 230.25850929940458);s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2300]) {s.store_exp_neg_input(2206, 2200);}
        s.b[2301] = (s.v[2200] < 1e-5);s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));}
        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);s.store_sqrt(2208, 2207);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, 2200, (-1.0), 2205, -1.0, (-1.0));}
        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {s.store_exp_sub(2027, 2200, 2197);s.store_div(2206, 2199, 2027);s.store_add_scaled_product_mixed_iia(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));}
        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));}
        if (s.b[2288] && (!s.b[2300])) {s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);s.store_sqrt(2208, 2207);}
        if s.b[2288] {s.store_mul3_lhs(2210, 2208, 2114, 2129);s.store_scaled_add(2211, 2153, 2200, 0.5);s.store_scalar(2212, 0.0);s.store_mul(2027, 2206, 2159);}
        s.b[2303] = (s.v[2027] > 0.0);s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2303]) {s.store_sqrt(2212, 2027);}
        if s.b[2288] {s.store_scaled_add(2213, 2160, 2209, 0.5);s.store_add_scaled_product_mixed_iaa(2214, 2213, 1.0, A::square(s.ad_value(2201)), A::sub_scaled_inputs(s.ad_value(2212), 1.0, s.ad_value(2131), 2.0), 0.125);}
        s.b[2304] = (s.v[2211] < 1e-5);s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2304]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2215, 2211, 1.0, 2211, 1.0, 2211, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));}
        s.b[2305] = (s.v[730] > 0.0);s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2304]) && s.b[2305]) {s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));}
        if (s.b[2288] && s.b[2304]) {s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2211), 1.0, A::scale(s.ad_value(2211), 0.25), 0.3333333333333333));s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);s.store_add_mixed_ia(2219, 2217, A::div_scaled_product(s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), 1.0, A::square(s.ad_value(2211)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));}
        if (s.b[2288] && (!s.b[2304])) {s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));}
        s.b[2306] = (s.v[730] > 0.0);s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {s.store_add_scaled_sub_value_product_indices(2220, 1.0, 2212, 1.0, 2216, 2131, 2.0);s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2217), 1.0, s.ad_value(2217), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2221, 730, A::square(s.ad_value(2027)), 2115, 2214, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2222, 2216, 2.0, 2221, (-2.0), 2115, A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2223, 2221, 2221, 1.0, 2216, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2224, 1.0, 2115, A::add(s.ad_value(2212), s.ad_value(2214)), 0.5);s.store_div_scaled_product_mixed_iia(2225, 2223, 2222, 1.0, A::add_scaled_square_product(s.ad_value(2222), 1.0, s.ad_value(2224), s.ad_value(2223), (-1.0)), 1.0);s.store_add(2211, 2211, 2225);s.store_exp(2226, 2225);s.store_div(2212, 2212, 2226);s.store_mul(2214, 2214, 2226);s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::mul3_scaled_output(s.ad_value(2216), s.ad_value(2217), s.ad_value(2131), 2.0));s.store_div_scaled_product3_mixed_iiaa(2201, 2201, 2226, A::add(s.ad_value(2220), s.ad_value(2213)), 1.0, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2226), s.ad_value(2213), 1.0), 1.0);s.store_mul(2204, 2201, 2129);}
        if (s.b[2288] && (!s.b[2304])) {s.store_sqrt(2218, 2215);s.store_add_scaled_inputs_mixed_ia(2219, 2217, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2212)), s.ad_value(2218)), 0.5);}
        if s.b[2288] {s.store_mul_div_scaled_product_mixed_iiia(2228, 2129, 2115, 2214, 1.0, A::add_scaled_product(s.ad_value(2216), 1.0, s.ad_value(2114), s.ad_value(2218), 1.0), 1.0);s.store_add_scaled_product_indices(2229, 2228, 1.0, 2129, 2219, 1.0);s.store_mul3_lhs(2230, 2218, 2114, 2129);}
        s.b[2307] = (s.v[218] < 0.0);s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2307]) {s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2228, 1.0);}
        if (s.b[2288] && (!s.b[2307])) {s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2228, 1.0);}
        if s.b[2288] {s.store_mul_product3_indices(2170, 2228, 757, 2168, 2169, 1.0);s.store_add_scaled_product_indices(2231, 2230, 1.0, 775, 2228, 1.0);s.store_add_scaled_product_indices(2232, 2230, 1.0, 776, 2228, 1.0);s.store_mul(2233, 774, 2231);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2215), 1.0, A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2234, 2164, A::offset(s.ad_value(2172), 1.0), 2170);s.store_ln_ad(2235, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0), 1.0));s.store_mul(2029, 2228, 2174);s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2308] = (s.v[222] < 0.0);s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2308]) {s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));}
    }
}
