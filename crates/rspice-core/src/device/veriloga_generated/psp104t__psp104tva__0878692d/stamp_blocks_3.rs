#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1494])) {s.store_mul_scale_offset_mixed_ai(1215, A::mul3(s.ad_value(490), s.ad_value(1216), s.ad_value(1216)), 1191, p.p870, 0.0);}
        s.b[1498] = (p.p879 > 1000.0);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1484])) && s.b[1498]) {s.store_scalar(1217, 1.0);}
        s.b[1499] = (s.v[1190] > ((-s.v[445]) * p.p879));s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });s.b[1500] = (p.p882 == 4.0);s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) && s.b[1500]) {s.store_mul_scale_offset_mixed_ai(1191, A::mul3_scaled_output(s.ad_value(1190), s.ad_value(1190), s.ad_value(1190), ((s.v[451] * s.v[451]) * s.v[451])), 1190, s.v[451], 0.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) && (!s.b[1500])) {s.store_powf_ad(1191, A::abs_scaled_input(s.ad_value(1190), s.v[451]), p.p882);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && s.b[1499]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1484])) && (!s.b[1498])) && (!s.b[1499])) {s.store_offset_scaled(1217, 1190, s.v[454], (((((s.v[445] * p.p879)) * (s.v[454]))) + (s.v[448])));}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1484])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(480, 647, 1218, 1.0, 648, 1219, 1.0, 649, 1220, 1.0);s.store_primal_add_scaled_inputs3_indices(668, 647, s.v[388], 648, s.v[389], 649, s.v[390]);s.store_add_scaled_offset_product_rhs_mixed_iia(484, 479, 1.0, 668, A::exp_scaled_input(s.ad_value(489), (s.v[372] * s.v[669])), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_iia(485, 480, 1.0, 668, A::exp_scaled_input(s.ad_value(490), (s.v[372] * s.v[669])), (-1.0), (-1.0));}
        s.b[1501] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });s.b[1502] = ((s.v[479] > 0.0) && (s.v[480] > 0.0));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });s.b[1503] = ((((((s.v[484] / s.v[479]) > 0.001) || ((s.v[485] / s.v[480]) > 0.001)) && (s.v[484] > 0.0)) && (s.v[485] > 0.0)) && (s.v[485] > s.v[484]));s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1502]) && s.b[1503]) {s.store_div(491, 484, 485);s.store_div_scaled_inputs(671, A::ln(s.ad_value(491)), s.v[371], A::sub(s.ad_value(489), s.ad_value(490)), 1.0);s.store_div_scaled_value_offset_denominator(670, s.ad_value(484), 1.0, A::exp(A::mul_scaled_lhs(s.ad_value(489), s.v[372], s.ad_value(671))), (-1.0), 1.0);}
        if ((s.b[1155] && s.b[1172]) && s.b[1501]) {s.store_add_scaled_offset_product_rhs_mixed_aia(481, A::add_scaled_offset_product_rhs(s.ad_value(476), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(486), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, 670, A::exp(A::mul_scaled_lhs(s.ad_value(486), s.v[372], s.ad_value(671))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(482, A::add_scaled_offset_product_rhs(s.ad_value(477), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(487), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, 670, A::exp(A::mul_scaled_lhs(s.ad_value(487), s.v[372], s.ad_value(671))), (-1.0), (-1.0));s.store_add_scaled_offset_product_rhs_mixed_aia(483, A::add_scaled_offset_product_rhs(s.ad_value(478), 1.0, s.ad_value(668), A::exp_scaled_input(s.ad_value(488), (s.v[372] * s.v[669])), (-1.0), (-1.0)), 1.0, 670, A::exp(A::mul_scaled_lhs(s.ad_value(488), s.v[372], s.ad_value(671))), (-1.0), (-1.0));}
        s.b[1504] = (((s.v[476] < 0.0) && (s.v[477] < 0.0)) && (s.v[478] < 0.0));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });s.b[1505] = (((((((s.v[481] / s.v[476]) > 0.001) || ((s.v[482] / s.v[477]) > 0.001)) || ((s.v[483] / s.v[478]) > 0.001)) && (s.v[481] < 0.0)) && (s.v[482] < 0.0)) && (s.v[483] < 0.0));s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) {s.store_div(491, 481, 482);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) {s.store_div_scaled_inputs(492, A::ln(s.ad_value(491)), (-s.v[371]), A::sub(s.ad_value(486), s.ad_value(487)), 1.0);s.store_primal_div_add_scaled_inputs_rhs_indices(494, 487, 487, 1.0, 486, -1.0);s.store_scaled_mul_ad(495, A::offset(s.ad_value(491), (-1.0)), A::offset(A::pow(s.ad_value(491), s.ad_value(494)), (-1.0)), s.v[371]);s.store_primal_div_add_scaled_inputs_rhs_indices(494, 486, 486, 1.0, 487, -1.0);s.store_sub_mixed_ai(496, A::add_scaled_products(A::pow(s.ad_value(491), s.ad_value(494)), A::sub(s.ad_value(487), s.ad_value(486)), 1.0, s.ad_value(491), s.ad_value(486), 1.0), 487);s.store_div(493, 495, 496);s.store_add(673, 492, 493);}
        s.b[1506] = (((((s.v[488] * s.v[372]) * s.v[673])) as f64).abs() < 1e-6);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        let (t0,) = {
    if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && s.b[1506]) {
        (1.0,)
    } else {
        (s.v[667],)
    }
};
        s.store_scalar(667, t0);
        if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && s.b[1506]) {s.store_mul_add_scaled_inputs_rhs_mixed_ai(672, 483, A::div_from_scalar(1.0, s.ad_value(488)), 1.0, 673, (0.5 * s.v[372]));s.store_div_scaled_product_indices(673, 483, 673, ((-0.5) * s.v[372]), 488, 1.0);}
        let (t1,) = {
    if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {
        (0.0,)
    } else {
        (s.v[667],)
    }
};
        s.store_scalar(667, t1);
        if (((((s.b[1155] && s.b[1172]) && s.b[1501]) && s.b[1504]) && s.b[1505]) && (!s.b[1506])) {s.store_div_scaled_value_offset_denominator(672, s.ad_value(483), -1.0, A::exp(A::mul_scaled_lhs(s.ad_value(488), (-s.v[372]), s.ad_value(673))), (-1.0), 1.0);}
        let (t8,) = {
    if (s.b[1155] && s.b[1172]) {
        let t2: f64 = (s.v[647] * s.v[415]);let t3: f64 = (s.v[648] * s.v[416]);let t4: f64 = (t2 + t3);let t5: f64 = (s.v[649] * s.v[417]);let t6: f64 = (t4 + t5);let t7: f64 = (p.p946 * t6);
        (t7,)
    } else {
        (s.v[502],)
    }
};
        s.store_scalar(502, t8);s.b[1507] = ((s.v[647] * s.v[415]) <= s.v[502]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1507]) {
        (0.0,)
    } else {
        (s.v[652],)
    }
};
        s.store_scalar(652, t9);s.b[1508] = ((s.v[648] * s.v[416]) <= s.v[502]);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        let (ta,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1508]) {
        (0.0,)
    } else {
        (s.v[653],)
    }
};
        s.store_scalar(653, ta);s.b[1509] = ((s.v[649] * s.v[417]) <= s.v[502]);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1155] && s.b[1172]) && s.b[1509]) {
        (0.0,)
    } else {
        (s.v[654],)
    }
};
        s.store_scalar(654, tb);s.b[1510] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1510]) {s.store_primal_ln_ad(661, A::div_scalar_offset_denominator((0.5 * p.p839), s.ad_value(668), 1e-21, 1.0));s.store_ln_ad(663, A::div_scalar_offset_denominator((0.5 * p.p839), s.ad_value(670), 1e-21, 1.0));s.store_ln_ad(665, A::div_scalar_offset_denominator((0.5 * p.p839), A::abs(s.ad_value(672)), 1e-21, 1.0));}
        if (s.b[1155] && s.b[1172]) {s.store_primal_min_with_scalar(661, 661, 230.25850929940458);s.store_primal_exp(662, 661);s.store_min_with_scalar(663, 663, 230.25850929940458);s.store_exp(664, 663);s.store_min_with_scalar(665, 665, 230.25850929940458);s.store_exp(666, 665);s.store_scalar(499, 0.4);s.store_scalar(500, 0.65);s.store_scalar(501, 0.8);s.store_primal_mul_scale_offset_indices(486, 553, 499, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(487, 553, 500, -1.0, 0.0);s.store_primal_mul_scale_offset_indices(488, 553, 501, -1.0, 0.0);s.store_scalar(489, 0.1);s.store_scalar(490, 0.2);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1511] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });s.b[1512] = (s.v[486] < s.v[682]);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
    ) {
        s.b[1513] = (((((-0.5) * (s.v[486] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && s.b[1513]) {s.store_primal_exp_scaled_input(1184, 486, (s.v[372] * (-0.5)));}
        s.b[1514] = (((-0.5) * (s.v[486] * s.v[372])) < 0.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && (!s.b[1513])) && s.b[1514]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(486), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) && (!s.b[1513])) && (!s.b[1514])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(486), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(486), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1512]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && (!s.b[1512])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 683, A::sub_scaled_inputs(s.ad_value(486), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1511]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1515] = (s.v[486] > 0.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && s.b[1515]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1511]) && (!s.b[1515])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 486);}
        if ((s.b[1155] && s.b[1172]) && s.b[1511]) {s.store_primal_sub(1187, 684, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 486, 0.5, 1187, 0.5, 486, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 486, 0.5, 687, 0.5, 486, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 486, A::sqrt_square_offset(s.ad_value(486), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1516] = (s.v[674] == 0.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1516]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1516])) {s.store_primal_mul(1192, 564, 1182);}
        s.b[1517] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1517]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {s.store_primal_sub(1194, 570, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1518] = (s.v[512] == 0.5);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && s.b[1518]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && (!s.b[1518])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1519] = (s.v[512] == 0.5);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && s.b[1519]) {s.store_sqrt_mul(1191, 1194, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) && (!s.b[1519])) {s.store_pow_mul_base_indices(1191, 1194, 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1517])) {s.store_mul(1198, 591, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 561, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 523, 1199, 1197);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        s.b[1520] = (s.v[526] == 0.0);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1520]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {s.store_mul_div_scaled_product_indices(1201, 606, 1198, 576, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 603, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1521] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1521]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1521])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, 603, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1522] = (s.v[1212] > 0.0);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1522]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1522])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1523] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1523, if s.b[1523] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1523]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1523])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1524] = (s.v[1212] > 0.0);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && s.b[1524]) {s.copy_ad(1213, 1175);}
        s.b[1525] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) && s.b[1525]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) && (!s.b[1525])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) && (!s.b[1524])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1520])) {s.store_div_scaled_product_indices(1214, 603, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 526, 1199, 1214, 1208, 1.0);}
        s.b[1526] = (s.v[532] == 0.0);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1526]) {s.store_scalar(1215, 0.0);}
        s.b[1527] = (s.v[512] == 0.5);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && s.b[1527]) {s.store_sqrt_mul_sub_lhs(1191, 509, 1189, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1527])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(509), s.ad_value(1189)), 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 579, A::sub(s.ad_value(509), s.ad_value(1189)), 594, 1.0, 1191, 1.0);}
        s.b[1528] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && s.b[1528]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));}
        s.b[1529] = (((-s.v[609]) / s.v[1216]) < 0.0);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1528])) && s.b[1529]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 609, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) && (!s.b[1528])) && (!s.b[1529])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 609, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1526])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 532, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1530] = (s.v[541] > 1000.0);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1516])) && s.b[1530]) {s.store_scalar(1217, 1.0);}
        s.b[1531] = (s.v[1190] > ((-s.v[445]) * s.v[541]));s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });s.b[1532] = (s.v[544] == 4.0);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) && s.b[1532]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), 1190, 615);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) && (!s.b[1532])) {s.store_pow_abs_mul_base_indices(1191, 1190, 615, 544);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && s.b[1531]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1516])) && (!s.b[1530])) && (!s.b[1531])) {s.store_add_scaled_product_mixed_iai(1217, 612, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1516])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1533] = (s.v[675] == 0.0);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1533]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1533])) {s.store_primal_mul(1192, 565, 1182);}
        s.b[1534] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1534]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {s.store_primal_sub(1194, 571, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1535] = (s.v[513] == 0.5);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && s.b[1535]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && (!s.b[1535])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1536] = (s.v[513] == 0.5);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && s.b[1536]) {s.store_sqrt_mul(1191, 1194, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) && (!s.b[1536])) {s.store_pow_mul_base_indices(1191, 1194, 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1534])) {s.store_mul(1198, 592, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 562, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 524, 1199, 1197);}
        s.b[1537] = (s.v[527] == 0.0);s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1537]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {s.store_mul_div_scaled_product_indices(1201, 607, 1198, 577, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 604, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1538] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1538]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1538])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, 604, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1539] = (s.v[1212] > 0.0);s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1539]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1539])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1540] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1540]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1540])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1541] = (s.v[1212] > 0.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && s.b[1541]) {s.copy_ad(1213, 1175);}
        s.b[1542] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) && s.b[1542]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) && (!s.b[1542])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) && (!s.b[1541])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1537])) {s.store_div_scaled_product_indices(1214, 604, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 527, 1199, 1214, 1208, 1.0);}
        s.b[1543] = (s.v[533] == 0.0);s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1543]) {s.store_scalar(1215, 0.0);}
        s.b[1544] = (s.v[513] == 0.5);s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && s.b[1544]) {s.store_sqrt_mul_sub_lhs(1191, 510, 1189, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1544])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(510), s.ad_value(1189)), 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 580, A::sub(s.ad_value(510), s.ad_value(1189)), 595, 1.0, 1191, 1.0);}
        s.b[1545] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && s.b[1545]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));}
        s.b[1546] = (((-s.v[610]) / s.v[1216]) < 0.0);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1545])) && s.b[1546]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 610, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) && (!s.b[1545])) && (!s.b[1546])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 610, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1543])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 533, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1547] = (s.v[542] > 1000.0);s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1533])) && s.b[1547]) {s.store_scalar(1217, 1.0);}
        s.b[1548] = (s.v[1190] > ((-s.v[445]) * s.v[542]));s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });s.b[1549] = (s.v[545] == 4.0);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) && s.b[1549]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), 1190, 616);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) && (!s.b[1549])) {s.store_pow_abs_mul_base_indices(1191, 1190, 616, 545);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && s.b[1548]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1533])) && (!s.b[1547])) && (!s.b[1548])) {s.store_add_scaled_product_mixed_iai(1217, 613, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1533])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1550] = (s.v[676] == 0.0);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1550]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1550])) {s.store_primal_mul(1192, 566, 1182);}
        s.b[1551] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1551]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {s.store_primal_sub(1194, 572, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1552] = (s.v[514] == 0.5);s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && s.b[1552]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && (!s.b[1552])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1553] = (s.v[514] == 0.5);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && s.b[1553]) {s.store_sqrt_mul(1191, 1194, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) && (!s.b[1553])) {s.store_pow_mul_base_indices(1191, 1194, 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1551])) {s.store_mul(1198, 593, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 563, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 525, 1199, 1197);}
        s.b[1554] = (s.v[528] == 0.0);s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1554]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {s.store_mul_div_scaled_product_indices(1201, 608, 1198, 578, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 605, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1555] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1555]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1555])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, 605, 1204, (-1.0), 1201, 1206, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1556] = (s.v[1212] > 0.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1556]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1556])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1557] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1557]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1557])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1558] = (s.v[1212] > 0.0);s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && s.b[1558]) {s.copy_ad(1213, 1175);}
        s.b[1559] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) && s.b[1559]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) && (!s.b[1559])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) && (!s.b[1558])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1554])) {s.store_div_scaled_product_indices(1214, 605, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 528, 1199, 1214, 1208, 1.0);}
        s.b[1560] = (s.v[534] == 0.0);s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1560]) {s.store_scalar(1215, 0.0);}
        s.b[1561] = (s.v[514] == 0.5);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && s.b[1561]) {s.store_sqrt_mul_sub_lhs(1191, 511, 1189, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1561])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(511), s.ad_value(1189)), 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 581, A::sub(s.ad_value(511), s.ad_value(1189)), 596, 1.0, 1191, 1.0);}
        s.b[1562] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && s.b[1562]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));}
        s.b[1563] = (((-s.v[611]) / s.v[1216]) < 0.0);s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1562])) && s.b[1563]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 611, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) && (!s.b[1562])) && (!s.b[1563])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 611, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1560])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 534, A::mul3(s.ad_value(486), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1564] = (s.v[543] > 1000.0);s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1550])) && s.b[1564]) {s.store_scalar(1217, 1.0);}
        s.b[1565] = (s.v[1190] > ((-s.v[445]) * s.v[543]));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });s.b[1566] = (s.v[546] == 4.0);s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) && s.b[1566]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), 1190, 617);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) && (!s.b[1566])) {s.store_pow_abs_mul_base_indices(1191, 1190, 617, 546);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && s.b[1565]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1550])) && (!s.b[1564])) && (!s.b[1565])) {s.store_add_scaled_product_mixed_iai(1217, 614, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), 620, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1155] && s.b[1172]) && (!s.b[1550])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(476, 674, 1218, 1.0, 675, 1219, 1.0, 676, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1567] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });s.b[1568] = (s.v[487] < s.v[682]);s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });s.b[1569] = (((((-0.5) * (s.v[487] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && s.b[1569]) {s.store_primal_exp_scaled_input(1184, 487, (s.v[372] * (-0.5)));}
        s.b[1570] = (((-0.5) * (s.v[487] * s.v[372])) < 0.0);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && (!s.b[1569])) && s.b[1570]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(487), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) && (!s.b[1569])) && (!s.b[1570])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(487), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(487), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1568]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && (!s.b[1568])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 683, A::sub_scaled_inputs(s.ad_value(487), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1567]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1571] = (s.v[487] > 0.0);s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && s.b[1571]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1567]) && (!s.b[1571])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 487);}
        if ((s.b[1155] && s.b[1172]) && s.b[1567]) {s.store_primal_sub(1187, 684, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 487, 0.5, 1187, 0.5, 487, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 487, 0.5, 687, 0.5, 487, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 487, A::sqrt_square_offset(s.ad_value(487), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1572] = (s.v[674] == 0.0);s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1572]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1572])) {s.store_primal_mul(1192, 564, 1182);}
        s.b[1573] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1573]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {s.store_primal_sub(1194, 570, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1574] = (s.v[512] == 0.5);s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && s.b[1574]) {s.store_scalar(1196, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && (!s.b[1574])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1575] = (s.v[512] == 0.5);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && s.b[1575]) {s.store_sqrt_mul(1191, 1194, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) && (!s.b[1575])) {s.store_pow_mul_base_indices(1191, 1194, 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1573])) {s.store_mul(1198, 591, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 561, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 523, 1199, 1197);}
        s.b[1576] = (s.v[526] == 0.0);s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1576]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {s.store_mul_div_scaled_product_indices(1201, 606, 1198, 576, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 603, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1577] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1577]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1577])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, 603, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1578] = (s.v[1212] > 0.0);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1578]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1578])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1579] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1579]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1579])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1580] = (s.v[1212] > 0.0);s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && s.b[1580]) {s.copy_ad(1213, 1175);}
        s.b[1581] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) && s.b[1581]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) && (!s.b[1581])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) && (!s.b[1580])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1576])) {s.store_div_scaled_product_indices(1214, 603, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 526, 1199, 1214, 1208, 1.0);}
        s.b[1582] = (s.v[532] == 0.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1582]) {s.store_scalar(1215, 0.0);}
        s.b[1583] = (s.v[512] == 0.5);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && s.b[1583]) {s.store_sqrt_mul_sub_lhs(1191, 509, 1189, 597);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1583])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(509), s.ad_value(1189)), 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 579, A::sub(s.ad_value(509), s.ad_value(1189)), 594, 1.0, 1191, 1.0);}
        s.b[1584] = (((((-s.v[609]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && s.b[1584]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(1216), 1.0));}
        s.b[1585] = (((-s.v[609]) / s.v[1216]) < 0.0);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1584])) && s.b[1585]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 609, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) && (!s.b[1584])) && (!s.b[1585])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 609, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1582])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 532, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1586] = (s.v[541] > 1000.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1572])) && s.b[1586]) {s.store_scalar(1217, 1.0);}
        s.b[1587] = (s.v[1190] > ((-s.v[445]) * s.v[541]));s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });s.b[1588] = (s.v[544] == 4.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) && s.b[1588]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(615))), s.ad_value(1190), s.ad_value(615)), 1190, 615);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) && (!s.b[1588])) {s.store_pow_abs_mul_base_indices(1191, 1190, 615, 544);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && s.b[1587]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1572])) && (!s.b[1586])) && (!s.b[1587])) {s.store_add_scaled_product_mixed_iai(1217, 612, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1572])) {s.store_mul_scale_offset_mixed_ia(1218, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1589] = (s.v[675] == 0.0);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1589]) {s.store_scalar(1219, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1589])) {s.store_primal_mul(1192, 565, 1182);}
        s.b[1590] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1590]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {s.store_primal_sub(1194, 571, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1591] = (s.v[513] == 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && s.b[1591]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && (!s.b[1591])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1592] = (s.v[513] == 0.5);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && s.b[1592]) {s.store_sqrt_mul(1191, 1194, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) && (!s.b[1592])) {s.store_pow_mul_base_indices(1191, 1194, 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1590])) {s.store_mul(1198, 592, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 562, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 524, 1199, 1197);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        s.b[1593] = (s.v[527] == 0.0);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1593]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {s.store_mul_div_scaled_product_indices(1201, 607, 1198, 577, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 604, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1594] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1594]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1594])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(604), s.ad_value(1202), s.ad_value(1205)), 1.0, 604, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1595] = (s.v[1212] > 0.0);s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1595]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1595])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1596] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1596]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1596])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1597] = (s.v[1212] > 0.0);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && s.b[1597]) {s.copy_ad(1213, 1175);}
        s.b[1598] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1598, if s.b[1598] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) && s.b[1598]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) && (!s.b[1598])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) && (!s.b[1597])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1593])) {s.store_div_scaled_product_indices(1214, 604, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 527, 1199, 1214, 1208, 1.0);}
        s.b[1599] = (s.v[533] == 0.0);s.store_scalar(1599, if s.b[1599] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1599]) {s.store_scalar(1215, 0.0);}
        s.b[1600] = (s.v[513] == 0.5);s.store_scalar(1600, if s.b[1600] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && s.b[1600]) {s.store_sqrt_mul_sub_lhs(1191, 510, 1189, 598);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1600])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(510), s.ad_value(1189)), 598, 513);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 580, A::sub(s.ad_value(510), s.ad_value(1189)), 595, 1.0, 1191, 1.0);}
        s.b[1601] = (((((-s.v[610]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1601, if s.b[1601] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && s.b[1601]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(1216), 1.0));}
        s.b[1602] = (((-s.v[610]) / s.v[1216]) < 0.0);s.store_scalar(1602, if s.b[1602] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1601])) && s.b[1602]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 610, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) && (!s.b[1601])) && (!s.b[1602])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 610, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1599])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 533, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1603] = (s.v[542] > 1000.0);s.store_scalar(1603, if s.b[1603] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1589])) && s.b[1603]) {s.store_scalar(1217, 1.0);}
        s.b[1604] = (s.v[1190] > ((-s.v[445]) * s.v[542]));s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });s.b[1605] = (s.v[545] == 4.0);s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) && s.b[1605]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(616))), s.ad_value(1190), s.ad_value(616)), 1190, 616);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) && (!s.b[1605])) {s.store_pow_abs_mul_base_indices(1191, 1190, 616, 545);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && s.b[1604]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1589])) && (!s.b[1603])) && (!s.b[1604])) {s.store_add_scaled_product_mixed_iai(1217, 613, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1589])) {s.store_mul_scale_offset_mixed_ia(1219, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        s.b[1606] = (s.v[676] == 0.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1606]) {s.store_scalar(1220, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1606])) {s.store_primal_mul(1192, 566, 1182);}
        s.b[1607] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1607]) {s.store_scalar(1193, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {s.store_primal_sub(1194, 572, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1608] = (s.v[514] == 0.5);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && s.b[1608]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && (!s.b[1608])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1609] = (s.v[514] == 0.5);s.store_scalar(1609, if s.b[1609] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && s.b[1609]) {s.store_sqrt_mul(1191, 1194, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) && (!s.b[1609])) {s.store_pow_mul_base_indices(1191, 1194, 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1607])) {s.store_mul(1198, 593, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 563, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 525, 1199, 1197);}
        s.b[1610] = (s.v[528] == 0.0);s.store_scalar(1610, if s.b[1610] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1610]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {s.store_mul_div_scaled_product_indices(1201, 608, 1198, 578, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 605, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1611] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(1611, if s.b[1611] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1611]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
    ) {
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1611])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(605), s.ad_value(1202), s.ad_value(1205)), 1.0, 605, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1612] = (s.v[1212] > 0.0);s.store_scalar(1612, if s.b[1612] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1612]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1612])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1613] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1613, if s.b[1613] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1613]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1613])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1614] = (s.v[1212] > 0.0);s.store_scalar(1614, if s.b[1614] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && s.b[1614]) {s.copy_ad(1213, 1175);}
        s.b[1615] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1615, if s.b[1615] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) && s.b[1615]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) && (!s.b[1615])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) && (!s.b[1614])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1610])) {s.store_div_scaled_product_indices(1214, 605, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 528, 1199, 1214, 1208, 1.0);}
        s.b[1616] = (s.v[534] == 0.0);s.store_scalar(1616, if s.b[1616] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1616]) {s.store_scalar(1215, 0.0);}
        s.b[1617] = (s.v[514] == 0.5);s.store_scalar(1617, if s.b[1617] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && s.b[1617]) {s.store_sqrt_mul_sub_lhs(1191, 511, 1189, 599);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1617])) {s.store_pow_mul_base_mixed_ai(1191, A::sub(s.ad_value(511), s.ad_value(1189)), 599, 514);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) {s.store_mul_div_scaled_product_mixed_iaii(1216, 581, A::sub(s.ad_value(511), s.ad_value(1189)), 596, 1.0, 1191, 1.0);}
        s.b[1618] = (((((-s.v[611]) / s.v[1216])) as f64).abs() < 230.25850929940458);s.store_scalar(1618, if s.b[1618] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && s.b[1618]) {s.store_ad_value(1191, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(1216), 1.0));}
        s.b[1619] = (((-s.v[611]) / s.v[1216]) < 0.0);s.store_scalar(1619, if s.b[1619] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1618])) && s.b[1619]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 611, -1.0, 1216, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) && (!s.b[1618])) && (!s.b[1619])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(1191, 611, -1.0, 1216, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1616])) {s.store_mul_ad_product_lhs_mixed_ia(1215, 534, A::mul3(s.ad_value(487), s.ad_value(1216), s.ad_value(1216)), 1191);}
        s.b[1620] = (s.v[543] > 1000.0);s.store_scalar(1620, if s.b[1620] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1606])) && s.b[1620]) {s.store_scalar(1217, 1.0);}
        s.b[1621] = (s.v[1190] > ((-s.v[445]) * s.v[543]));s.store_scalar(1621, if s.b[1621] { 1.0 } else { 0.0 });s.b[1622] = (s.v[546] == 4.0);s.store_scalar(1622, if s.b[1622] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) && s.b[1622]) {s.store_mul_ad_product_lhs_mixed_ai(1191, A::mul3(A::square(A::mul(s.ad_value(1190), s.ad_value(617))), s.ad_value(1190), s.ad_value(617)), 1190, 617);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) && (!s.b[1622])) {s.store_pow_abs_mul_base_indices(1191, 1190, 617, 546);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && s.b[1621]) {s.store_div_from_scalar_sub_from_scalar_ad(1217, 1.0, 1.0, s.ad_value(1191));}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1606])) && (!s.b[1620])) && (!s.b[1621])) {s.store_add_scaled_product_mixed_iai(1217, 614, 1.0, A::add_scaled_inputs(s.ad_value(1190), 1.0, s.ad_value(543), s.v[445]), 620, 1.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1606])) {s.store_mul_scale_offset_mixed_ia(1220, 1217, A::add_scaled_inputs4(s.ad_value(1192), 1.0, s.ad_value(1193), 1.0, s.ad_value(1200), 1.0, s.ad_value(1215), 1.0), p.p29, 0.0);}
        if (s.b[1155] && s.b[1172]) {s.store_add_scaled_products3_indices(477, 674, 1218, 1.0, 675, 1219, 1.0, 676, 1220, 1.0);s.store_scalar(1189, 0.0);s.store_scalar(1186, 0.0);}
        s.b[1623] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(1623, if s.b[1623] { 1.0 } else { 0.0 });s.b[1624] = (s.v[488] < s.v[682]);s.store_scalar(1624, if s.b[1624] { 1.0 } else { 0.0 });s.b[1625] = (((((-0.5) * (s.v[488] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(1625, if s.b[1625] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && s.b[1625]) {s.store_primal_exp_scaled_input(1184, 488, (s.v[372] * (-0.5)));}
        s.b[1626] = (((-0.5) * (s.v[488] * s.v[372])) < 0.0);s.store_scalar(1626, if s.b[1626] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) && s.b[1626]) {s.store_primal_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1184, 1e-100, (-230.25850929940458), A::scale(s.ad_value(488), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) && (!s.b[1625])) && (!s.b[1626])) {s.store_primal_scaled_offset_ad(1184, A::mul_offset_rhs(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(488), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(488), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1624]) {s.store_primal_div_from_scalar(1185, 1.0, 1184);s.store_primal_square(1182, 1185);}
        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && (!s.b[1624])) {s.store_primal_mul_scale_offset_mixed_ia(1182, 683, A::sub_scaled_inputs(s.ad_value(488), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_primal_sqrt(1185, 1182);s.store_primal_div_from_scalar(1184, 1.0, 1185);}
        if ((s.b[1155] && s.b[1172]) && s.b[1623]) {s.store_primal_offset(1182, 1182, (-1.0));}
        s.b[1627] = (s.v[488] > 0.0);s.store_scalar(1627, if s.b[1627] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && s.b[1627]) {s.store_primal_scaled_ln_ad(1186, A::add(A::offset(s.ad_value(1184), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1184), 1.0, A::offset(s.ad_value(1184), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[1155] && s.b[1172]) && s.b[1623]) && (!s.b[1627])) {s.store_primal_sub_mixed_ai(1186, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(1185), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(1185), 1.0, A::scale_offset(s.ad_value(1185), 3.0, 1.0))))), (s.v[371] * 2.0)), 488);}
        if ((s.b[1155] && s.b[1172]) && s.b[1623]) {s.store_primal_sub(1187, 684, 1186);s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1188, 488, 0.5, 1187, 0.5, 488, 1187, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_primal_add_scaled_inputs3_sqrt_third_sub_square_offset(1189, 488, 0.5, 687, 0.5, 488, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_primal_scaled_sub_mixed_ia(1190, 488, A::sqrt_square_offset(s.ad_value(488), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[1628] = (s.v[674] == 0.0);s.store_scalar(1628, if s.b[1628] { 1.0 } else { 0.0 });
        if ((s.b[1155] && s.b[1172]) && s.b[1628]) {s.store_scalar(1218, 0.0);}
        if ((s.b[1155] && s.b[1172]) && (!s.b[1628])) {s.store_primal_mul(1192, 564, 1182);}
        s.b[1629] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(1629, if s.b[1629] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1629]) {s.store_scalar(1193, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
    ) {
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {s.store_primal_sub(1194, 570, 1188);s.store_primal_sub_from_scalar_ad(1195, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(1186), s.ad_value(1194)))));}
        s.b[1630] = (s.v[512] == 0.5);s.store_scalar(1630, if s.b[1630] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && s.b[1630]) {s.store_scalar(1196, 0.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && (!s.b[1630])) {s.store_primal_mul_scale_offset(1196, A::add(A::div_scaled_product(A::square(s.ad_value(1195)), A::ln(s.ad_value(1195)), 1.0, A::sub_from_scalar(1.0, s.ad_value(1195)), 1.0), s.ad_value(1195)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {s.store_primal_add(1197, 1195, 1196);}
        s.b[1631] = (s.v[512] == 0.5);s.store_scalar(1631, if s.b[1631] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && s.b[1631]) {s.store_sqrt_mul(1191, 1194, 597);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) && (!s.b[1631])) {s.store_pow_mul_base_indices(1191, 1194, 597, 512);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1629])) {s.store_mul(1198, 591, 1191);s.store_mul_ad_product_lhs_mixed_ia(1199, 561, A::offset(s.ad_value(1185), (-1.0)), 1198);s.store_mul3_lhs(1193, 523, 1199, 1197);}
        s.b[1632] = (s.v[526] == 0.0);s.store_scalar(1632, if s.b[1632] { 1.0 } else { 0.0 });
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && s.b[1632]) {s.store_scalar(1200, 0.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {s.store_mul_div_scaled_product_indices(1201, 606, 1198, 576, 1.0, 1194, 1.0);s.store_div_scaled_inputs_indices(1202, 603, 0.666666666666667, 1201, 1.0);s.store_square(1203, 1202);s.store_sqrt_div_scaled_square_offset_denominator(1204, 1203, 1.0, 1.0, 1.0);s.store_sqrt(1205, 1204);s.store_mul(1206, 1204, 1205);}
        s.b[1633] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(1633, if s.b[1633] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1633]) {s.store_div_from_scalar_offset_product(1207, 1.0, 1201, 1206, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1633])) {s.store_pow_ad(1207, A::offset(A::mul(s.ad_value(1201), s.ad_value(1206)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {s.store_div_scaled_product_add_scaled_denominator_indices(1208, 1197, 1207, 1.0, 1197, 1.0, 1207, 1.0, 1.0);s.store_sqrt_scaled_input_ad(1209, A::div(s.ad_value(1201), s.ad_value(1205)), 0.375);s.store_add_scaled_product_indices(1210, 1204, (-1.0), 1202, 1205, 2.0);s.store_add_scaled_value_products_mixed_aiiii(1211, A::mul3(s.ad_value(603), s.ad_value(1202), s.ad_value(1205)), 1.0, 603, 1204, (-1.0), 1201, 1206, 0.5);s.store_mul_scale_offset_indices(1212, 1209, 1210, 1.0, (-1.0));s.store_square(1173, 1212);}
        s.b[1634] = (s.v[1212] > 0.0);s.store_scalar(1634, if s.b[1634] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1634]) {s.store_div_from_scalar_offset_scaled_input(1174, 1.0, 1212, s.v[373], 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1634])) {s.store_div_from_scalar_sub_from_scalar_ad(1174, 1.0, 1.0, A::scale(s.ad_value(1212), s.v[373]));}
        s.b[1635] = (((-s.v[1173]) + s.v[1211]) > (-230.25850929940458));s.store_scalar(1635, if s.b[1635] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1635]) {s.store_exp_sub(1191, 1211, 1173);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1635])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1191, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1211), s.ad_value(1173)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {s.store_mul_mixed_ai(1175, A::add_scaled_inputs_product(s.ad_value(1174), 0.29214664, A::square(s.ad_value(1174)), s.v[374], A::square(s.ad_value(1174)), s.ad_value(1174), s.v[375]), 1191);}
        s.b[1636] = (s.v[1212] > 0.0);s.store_scalar(1636, if s.b[1636] { 1.0 } else { 0.0 });
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && s.b[1636]) {s.copy_ad(1213, 1175);}
        s.b[1637] = (s.v[1211] > (-230.25850929940458));s.store_scalar(1637, if s.b[1637] { 1.0 } else { 0.0 });
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) && s.b[1637]) {s.store_exp(1191, 1211);}
        if (((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) && (!s.b[1637])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1191, 1e-100, (-230.25850929940458), 1211, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) && (!s.b[1636])) {s.store_sub_scaled_inputs(1213, 1191, 2.0, 1175, 1.0);}
        if (((s.b[1155] && s.b[1172]) && (!s.b[1628])) && (!s.b[1632])) {s.store_div_scaled_product_indices(1214, 603, 1213, (1.772453850905516 * 0.5), 1209, 1.0);s.store_mul_product3_indices(1200, 526, 1199, 1214, 1208, 1.0);}
    }
}
