#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1131] && s.b[1194]) && s.b[1196]) {s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 172, -1.0);}
        if ((s.b[1131] && s.b[1194]) && (!s.b[1196])) {s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 350, -1.0);}
        if (s.b[1131] && s.b[1194]) {s.store_add_product3_rhs_indices(1189, 1189, 173, 1186, 1190, 1.0);s.store_mul(1191, 1188, 1189);s.copy_ad(1188, 1191);}
        if (s.b[1131] && (!s.b[1194])) {s.store_scalar(1188, 0.0);}
        s.b[1197] = (p[248] != 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1197]) {s.store_scale(1185, 225, s.v[118]);s.store_mul(1193, 323, 1185);s.store_mul(1192, 1193, 173);}
        if (s.b[1131] && (!s.b[1197])) {s.store_scalar(1192, 0.0);}
        s.b[1198] = ((s.v[1188] + s.v[1192]) > 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1198]) {s.store_mul_add_rhs(247, 164, 1188, 1192);s.store_mul3_lhs(201, 264, 247, 250);}
        if s.b[1131] {s.store_add(199, 200, 201);s.copy_ad(203, 201);}
        s.b[1208] = (p[33] != 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1208]) {s.copy_ad(1201, 549);s.store_scalar(1202, (s.v[124] - p[71]));s.store_div_from_scalar_square_ad(1203, 1.0, s.ad_value(1202));s.store_mul_ad_product_lhs_mixed_ai(1204, A::mul_sub_from_scalar_lhs_scaled_output(p[69], s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), 1201, 1203);s.store_mul(186, 1204, 235);s.store_offset_scaled(1200, 173, p[155], p[154]);s.store_mul(206, 186, 1200);s.store_sub_from_scalar_scaled_input(1199, p[156], 157, p[157]);s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1199, 1.0, 206, 1.0, (-s.v[123]));s.store_mul3_lhs(210, 205, 324, 324);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_offset_sub_ad(1205, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p[156])))), s.ad_value(206), 1e-50);s.store_offset_sub(1199, 174, 1205, (-0.005));}
        if (s.b[1131] && s.b[1208]) {s.store_scalar(327, (if (s.v[1205] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_add_scaled_square_product(1201, 1199, 1.0, 327, 1205, (4.0 * 0.005));s.store_sub_mixed_ai(1202, A::add_scaled_inputs4_offset(s.ad_value(1205), 1.0, s.ad_value(1199), 0.5, s.ad_value(1201), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p[156]))), 514);s.store_offset_mul(1203, 225, 1202, (-1.0));s.store_div_from_scalar(1204, 4.0, 212);s.store_offset_mul(1200, 1203, 1204, 1.0);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1209] = (s.v[1199] < 0.0);s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1209]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(213, 1199, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));s.store_mul_ln_mixed_ia(216, 327, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))));s.store_div_scaled_value_offset_denominator(1202, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);s.store_offset_sub(217, 216, 215, (-0.002));s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));s.store_div_from_scalar(1199, 1.0, 327);s.store_mul_exp_mixed_ia(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1131] && s.b[1208]) {s.store_add_offset_lhs_mixed_ai(1200, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1210] = (s.v[1199] < 0.0);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1210]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(219, 1199, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(1200, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1211] = (s.v[1199] < 0.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1211]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(220, 1199, (10.0 * 2.220446049250313e-16));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_sub(1200, 215, 218);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1212] = (s.v[1199] < 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1212]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_div_scaled_value_offset_denominator(1206, s.ad_value(157), 1.0, s.ad_value(1199), (10.0 * 2.220446049250313e-16), 1.0);s.store_square(49, 1206);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t0,) = {
    if (s.b[1131] && s.b[1208]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t0);
        let (t1,) = {
    if (s.b[1131] && s.b[1208]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t1);
        if (s.b[1131] && s.b[1208]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1213] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });s.b[1214] = (4.0 == 1.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((s.b[1131] && s.b[1208]) && s.b[1213]) && s.b[1214]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2);s.b[1215] = (4.0 == 2.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && s.b[1215]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3);s.b[1216] = (4.0 == 4.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && s.b[1216]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4);s.b[1217] = (4.0 == 8.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1216])) && s.b[1217]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5);
        let (t6,) = {
    if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((s.b[1131] && s.b[1208]) && s.b[1213]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1131] && s.b[1208]) && s.b[1213]) {s.store_sqrt(53, 53);}
            let (t8,) = {
    if ((s.b[1131] && s.b[1208]) && s.b[1213]) {
        let t7: f64 = (s.v[54] + 1.0);
        (t7,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t8);
        }
        if ((s.b[1131] && s.b[1208]) && (!s.b[1213])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (s.b[1131] && s.b[1208]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(1207, 1206, 53, 1.0);s.store_scale(214, 227, ((2.0 * s.v[126]) * p[9]));s.store_div_scaled_product_mixed_aii(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1207, 1.0, 441, 1.0);s.store_add(199, 199, 222);}
        s.b[1218] = ((p[30] != 0.0) && (p[32] != 0.0));s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1218]) {s.store_square(294, 192);s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1131] && s.b[1218]) {s.store_sub(296, 294, 295);s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1219] = (s.v[294] < 0.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1218]) && s.b[1219]) {s.store_scalar(294, 0.0);}
        if (s.b[1131] && s.b[1218]) {s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1220] = (s.v[296] < 0.0);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1218]) && s.b[1220]) {s.store_scalar(296, 0.0);}
        if (s.b[1131] && s.b[1218]) {s.store_sub(297, 294, 296);}
        s.b[1221] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1131] && s.b[1218]) && s.b[1221]) {
        (0.0,)
    } else {
        (s.v[146],)
    }
};
        s.store_scalar(146, tb);
        let (tc,) = {
    if ((s.b[1131] && s.b[1218]) && (!s.b[1221])) {
        (1.0,)
    } else {
        (s.v[146],)
    }
};
        s.store_scalar(146, tc);s.copy_ad(202, 199);s.store_scalar(204, 0.0);s.b[1222] = ((p[281] > 0.0) && (p[285] > 0.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if s.b[1222] {s.store_scalar(1229, s.v[99]);s.store_scalar(1233, p[237]);s.store_offset_add_scaled_inputs3_offset_indices(1234, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p[286]));}
        let (te,) = {
    if s.b[1222] {
        let td: f64 = (s.v[182] + p[286]);
        (td,)
    } else {
        (s.v[1235],)
    }
};
        s.store_scalar(1235, te);
        if s.b[1222] {s.store_scalar(1237, p[285]);s.store_scalar(1236, p[283]);s.store_scalar(1227, s.v[70]);s.store_mul_ln_mixed_ia(1228, 227, A::div_scaled_product_by_product(s.ad_value(1227), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));}
        if s.b[1222] {
            if (p[43] == 1.0) {
                s.copy_ad(1225, 435);
            } else {
                s.copy_ad(1225, 350);
            }
        }
        if s.b[1222] {s.store_sqrt_ad(1230, A::div_scaled_product3(A::sub(s.ad_value(1228), s.ad_value(1225)), s.ad_value(536), s.ad_value(1227), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1227)), 1.0));s.store_mul(1224, 1230, 1229);s.store_div_scaled_product_add_scaled_denominator_indices(1223, 1224, 1224, (-0.25), 157, 1.0, 1224, 1.0, 1.0);s.copy_ad(1249, 1223);}
        let (tf,) = {
    if s.b[1222] {
        (s.v[1235],)
    } else {
        (s.v[1250],)
    }
};
        s.store_scalar(1250, tf);
        if s.b[1222] {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1234), s.ad_value(1249))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if s.b[1222] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[1222] {s.store_add_product3_rhs_mixed_iia(376, 1234, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);}
        s.b[1251] = (s.v[158] < ((s.v[123] + s.v[1250]) * 0.5));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        let (t10,) = {
    if (s.b[1222] && s.b[1251]) {
        (0.0,)
    } else {
        (s.v[144],)
    }
};
        s.store_scalar(144, t10);s.b[1252] = ((s.v[144] == 0.0) || (1.0 != 0.0));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1252]) {s.store_mul_sub_rhs(181, 225, 376, 1249);}
        s.b[1253] = (s.v[181] < 3.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((s.b[1222] && s.b[1252]) && s.b[1253]) {s.store_mul_sub_rhs(337, 225, 1234, 1249);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1222] && s.b[1252]) && s.b[1253]) {s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 1249, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[1254] = ((s.v[158] - s.v[383]) <= s.v[1250]);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });s.b[1255] = (p[43] == 0.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) && s.b[1255]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1233, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1234, 331, 323);}
        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) {s.copy_ad(378, 376);}
        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1222] && s.b[1252]) && (!s.b[1253])) && (!s.b[1254])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        s.b[1256] = (p[43] == 0.0);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });s.b[1257] = ((s.v[158] - s.v[383]) <= s.v[1250]);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && s.b[1257]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1233, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1234, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1233, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) {s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1234), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1234, 331, 323);s.copy_ad(378, 376);}
        s.b[1258] = ((s.v[1234] - s.v[383]) > 0.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1234), s.ad_value(383)), A::sub(s.ad_value(1234), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1234), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);}
        s.b[1259] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t11,) = {
    if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t11);
        let (t12,) = {
    if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t12);
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1260] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });s.b[1261] = (2.0 == 1.0);s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        let (t13,) = {
    if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && s.b[1261]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t13);s.b[1262] = (2.0 == 2.0);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        let (t14,) = {
    if ((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && s.b[1262]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t14);s.b[1263] = (2.0 == 4.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        let (t15,) = {
    if (((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && s.b[1263]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t15);s.b[1264] = (2.0 == 8.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        let (t16,) = {
    if ((((((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) && (!s.b[1263])) && s.b[1264]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t16);
        let (t17,) = {
    if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t17);let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {s.store_sqrt(53, 53);}
            let (t19,) = {
    if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && s.b[1260]) {
        let t18: f64 = (s.v[54] + 1.0);
        (t18,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t19);
        }
        if ((((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) && (!s.b[1260])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && s.b[1259]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if (((((s.b[1222] && s.b[1252]) && s.b[1256]) && (!s.b[1257])) && s.b[1258]) && (!s.b[1259])) {s.copy_ad(378, 376);}
        if s.b[1222] {s.store_offset(336, 1249, (5e-12 / 2.0));}
        s.b[1265] = (s.v[378] < s.v[336]);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1265]) {s.copy_ad(378, 336);}
        if s.b[1222] {s.copy_ad(1232, 378);s.copy_ad(163, 376);}
        if (s.b[1222] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1232]) >= 0.0) {
                s.store_sub(166, 376, 1232);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (s.b[1222] && (0.0 != 0.0)) {s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p[287])) + ((-0.03))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1222] && (0.0 != 0.0)) {s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (s.b[1222] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1222] && (0.0 != 0.0)) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (s.b[1222] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1266] = (s.v[165] < 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((s.b[1222] && (0.0 != 0.0)) && s.b[1266]) {s.store_scalar(165, 0.0);}
        s.b[1267] = (s.v[165] > s.v[157]);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (((s.b[1222] && (0.0 != 0.0)) && (!s.b[1266])) && s.b[1267]) {s.copy_ad(165, 157);}
        if (s.b[1222] && (0.0 != 0.0)) {s.store_add(163, 1232, 165);}
        s.b[1268] = (p[282] == 1.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1268]) {s.copy_ad(378, 1232);s.copy_ad(1269, 1223);}
        let (t20,) = {
    if (s.b[1222] && s.b[1268]) {
        let t1c: f64 = (s.v[123] - s.v[185]);let t1d: f64 = (t1c + s.v[320]);let t1e: f64 = (t1d + s.v[1269]);let t1f: f64 = (t1e + p[286]);
        (t1f,)
    } else {
        (s.v[160],)
    }
};
        s.store_scalar(160, t20);s.b[1271] = (s.v[158] < s.v[160]);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        let (t22,) = {
    if ((s.b[1222] && s.b[1268]) && s.b[1271]) {
        let t21: f64 = (-1.0);
        (t21,)
    } else {
        (s.v[338],)
    }
};
        s.store_scalar(338, t22);
        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));s.store_mul_sub_rhs(336, 225, 1234, 1269);s.store_div_scalar_by_product_indices(328, 1.0, 225, 238, 1.0);s.store_mul(337, 328, 323);s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);s.store_offset(331, 336, (-2.0));s.store_scaled_mul(332, 337, 331, 9.0);s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);s.store_square(259, 261);}
        s.b[1272] = (s.v[260] < (s.v[259] * 1e-8));s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && s.b[1272]) {s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));}
        if (((s.b[1222] && s.b[1268]) && s.b[1271]) && (!s.b[1272])) {s.store_sqrt_add(258, 260, 259);s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);}
        if ((s.b[1222] && s.b[1268]) && s.b[1271]) {s.store_powf(256, 257, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);s.store_div_from_scalar(328, 1.0, 256);s.store_mul(181, 255, 328);s.store_add_scaled_product_indices(313, 1269, 1.0, 181, 227, 1.0);s.store_sub(328, 313, 1269);s.store_div(329, 328, 254);s.store_sqrt_square_offset(330, 329, 1.0);s.store_add_div_lhs_indices(1232, 328, 330, 1269);}
        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1269), (-p[287])));}
        let (t23,) = {
    if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t23);
        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.copy_ad(1270, 378);s.store_scale(419, 229, ((p[237] * (p[237] * 0.5)) * 9662367879.197212));s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);s.store_div_ln_lhs(420, 328, 419);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t29: usize = 0;
        while {
            let t27: f64 = (s.v[57] + 1.0);let t28: f64 = if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (s.v[167] <= t27)) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_sub(417, 1270, 1269);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1273] = (s.v[337] < 80.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1273]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1273])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_mul(421, 225, 422);}
            s.b[1274] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1275] = (s.v[181] < 0.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1274]) && s.b[1275]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1276] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && s.b[1276]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1274])) && (!s.b[1276])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1277] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
            let (t25,) = {
    if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1277]) {
        let t24: f64 = (-1.0);
        (t24,)
    } else {
        (s.v[338],)
    }
};
            s.store_scalar(338, t25);s.b[1278] = (s.v[181] < 0.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1278]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1279] = (s.v[181] < 1e-7);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && s.b[1279]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && (!s.b[1279])) {s.store_mul_scale_offset_indices(501, 225, 1270, 1.0, (-p[287]));s.store_exp(502, 501);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_add_scaled_inputs_product_indices(492, 1270, 1.0, 1234, (-1.0), 240, 490, 1.0);s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1280] = (s.v[430] == 1.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1280]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1270]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1270))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1281] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1281]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) {s.store_add(1270, 1270, 494);}
            s.b[1282] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
            let (t26,) = {
    if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1282]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t26);
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.copy_ad(1232, 1270);}
        if s.b[1222] {s.store_mul_sub_scaled_inputs_rhs_indices(332, 225, 1232, -1.0, 1223, -1.0);}
        if s.b[1222] {s.store_scalar(1247, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));}
        if s.b[1222] {s.store_mul(1248, 1247, 332);s.store_exp(333, 332);s.store_sub_offset_lhs(334, 333, (-1.0), 332);}
        s.b[1283] = (s.v[332] > 1e-7);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1283]) {s.store_mul_scaled_sqrt_rhs(437, 238, -1.0, 334);}
        s.b[1284] = (s.v[1248] > 1e-7);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if ((s.b[1222] && (!s.b[1283])) && s.b[1284]) {s.store_mul_sqrt_rhs(437, 238, 334);}
        if ((s.b[1222] && (!s.b[1283])) && (!s.b[1284])) {s.store_mul_ad_affine_product_rhs(437, 1247, s.ad_value(1248), A::sqrt_scaled_lhs_product_offset(s.ad_value(1248), 0.3333333333333333, A::scale_offset(s.ad_value(1248), 0.25, 1.0), 1.0), (-0.7071067811865475), 0.0);}
        if s.b[1222] {s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));s.store_offset_add_scaled_inputs_indices(1244, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));}
        s.b[1285] = (s.v[1244] < 0.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1285]) {s.store_scalar(1244, 0.0);}
        if s.b[1222] {s.store_div_scaled_inputs_indices(1245, 1244, 1.0, 536, 1.6021918e-19);s.store_sub(328, 1245, 1236);s.store_scale(1246, 1245, 0.01);s.store_sqrt_add_scaled_square_product(44, 328, 1.0, 1246, 1246, 4.0);s.store_add_scaled_inputs3_indices(329, 328, 0.5, 44, 0.5, 1246, 1e-10);}
        s.b[1286] = (s.v[329] < 0.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1286]) {s.store_scalar(329, 0.0);}
        if s.b[1222] {s.store_div_scaled_product_by_product_indices(1243, 329, 329, 1.0, 1245, 1245, 1.0);s.store_add_scaled_product_mixed_iai(1226, 1223, 1.0, A::sub(s.ad_value(1232), s.ad_value(1223)), 1243, 1.0);s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1226))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1226), s.ad_value(157)))));s.store_primal_sqrt_scaled_input(1239, 1227, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_mul_sqrt_rhs(1240, 1239, 227);s.store_mul_sub_rhs(1231, 225, 1226, 1223);}
        s.b[1287] = ((s.v[1231] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1287]) {s.store_sub_scaled_inputs(44, 225, 0.2, 1231, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 225, 225, (0.2 * 0.2));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2a,) = {
    if (s.b[1222] && s.b[1287]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2a);
        let (t2b,) = {
    if (s.b[1222] && s.b[1287]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2b);
        if (s.b[1222] && s.b[1287]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1288] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });s.b[1289] = (1.0 == 1.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        let (t2c,) = {
    if (((s.b[1222] && s.b[1287]) && s.b[1288]) && s.b[1289]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2c);s.b[1290] = (1.0 == 2.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        let (t2d,) = {
    if ((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && s.b[1290]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2d);s.b[1291] = (1.0 == 4.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        let (t2e,) = {
    if (((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && s.b[1291]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2e);s.b[1292] = (1.0 == 8.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        let (t2f,) = {
    if ((((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && (!s.b[1291])) && s.b[1292]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2f);
        let (t30,) = {
    if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t30);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t34: usize = 0;
        while {
            let t33: f64 = if (((s.b[1222] && s.b[1287]) && s.b[1288]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t33 != 0.0
        } {
            t34 += 1;
            if t34 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t34, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1222] && s.b[1287]) && s.b[1288]) {s.store_sqrt(53, 53);}
            let (t32,) = {
    if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
        let t31: f64 = (s.v[54] + 1.0);
        (t31,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t32);
        }
        if ((s.b[1222] && s.b[1287]) && (!s.b[1288])) {s.store_powf(53, 53, (1.0 / 2.0));}
        if (s.b[1222] && s.b[1287]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);}
        if (s.b[1222] && (!s.b[1287])) {s.copy_ad(328, 1231);}
        if s.b[1222] {s.store_sqrt_offset_input(1241, 328, (10.0 * 2.220446049250313e-16));s.store_mul(1242, 1240, 1241);s.store_mul_div_scaled_inputs_indices(1238, 1242, 227, 2.0, 1229, 1.0);s.store_mul_product3_indices(204, 337, 1238, 1237, 107, 1.0);s.store_add(199, 202, 204);}
        s.store_add(201, 203, 204);s.b[1293] = ((p[43] == 1.0) || (p[45] == 1.0));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });s.b[1306] = ((s.v[145] == 1.0) || (p[25] == 0.0));s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1293] && s.b[1306]) {s.store_scalar(263, 0.0);}
        s.b[1307] = ((p[117] <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1293] && (!s.b[1306])) && s.b[1307]) {s.store_scalar(263, 0.0);}
        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p[48]);}
        s.b[1308] = (p[44] <= 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {s.copy_ad(1294, 445);s.store_square(1301, 323);s.copy_ad(1302, 545);s.store_div(1296, 1302, 1301);s.store_div_from_scalar(1303, 2.0, 1302);s.store_mul(1297, 1303, 1301);s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));s.store_offset_mul(1300, 1297, 1298, 1.0);s.store_sqrt_square_offset(44, 1300, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1299, 1300, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1309] = (s.v[1299] < 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1309]) {s.store_scalar(1299, 0.0);}
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {s.store_offset(1299, 1299, 1e-50);s.store_sqrt(1299, 1299);s.store_add_scaled_product_mixed_aii(1304, A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)), 1.0, 1294, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(1305, 173, p[122], 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1304)), -1.0);s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1310] = (s.v[1305] < 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1310]) {s.store_scalar(1305, 0.0);}
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {s.store_mul(1294, 134, 445);s.store_div_square_rhs(1296, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1297, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));s.store_offset_mul(1299, 1297, 1298, 1.0);s.store_scaled_offset(1301, 1297, 1.0, 2.0);}
        s.b[1311] = ((s.v[1299] < (1e-50 + s.v[1301])) && (s.v[1301] >= 0.0));s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.store_sub_offset_lhs(44, 1301, 1e-50, 1299);s.store_square(49, 44);s.store_square(50, 1301);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t35,) = {
    if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t35);
        let (t36,) = {
    if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t36);
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.copy_ad(53, 48);}
        s.b[1312] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });s.b[1313] = (4.0 == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        let (t37,) = {
    if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && s.b[1313]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t37);s.b[1314] = (4.0 == 2.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        let (t38,) = {
    if (((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && s.b[1314]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t38);s.b[1315] = (4.0 == 4.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        let (t39,) = {
    if ((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && s.b[1315]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t39);s.b[1316] = (4.0 == 8.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        let (t3a,) = {
    if (((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && (!s.b[1315])) && s.b[1316]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3a);
        let (t3b,) = {
    if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3b);let mut t3f: usize = 0;
        while {
            let t3e: f64 = if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t3e != 0.0
        } {
            t3f += 1;
            if t3f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {s.store_sqrt(53, 53);}
            let (t3d,) = {
    if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
        let t3c: f64 = (s.v[54] + 1.0);
        (t3c,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t3d);
        }
        if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && (!s.b[1312])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 1301, 53);s.store_sub_offset_lhs(1299, 1301, 1e-50, 43);}
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && (!s.b[1311])) {
        }
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {
            if (s.v[1299] <= 0.0) {
                s.store_scalar(1299, 0.0);
            } else {
                s.store_sqrt(1299, 1299);
            }
        }
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {s.store_add_mul_sub_from_scalar_rhs_indices(1304, 1294, 1296, 1.0, 1299);s.store_div_from_scalar_offset_input(1295, s.v[100], 131, s.v[100]);s.store_add_scaled_inputs_product_indices(1305, 173, p[122], 176, 1.0, 1295, 1304, (-1.0));s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1317] = (s.v[1305] < 0.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1317]) {s.store_scalar(1305, 0.0);}
        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {s.store_offset(1305, 1305, 1e-50);s.store_ad_value(1295, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1305), 1.0));s.store_mul_product3_indices(263, 1295, 132, 1305, 199, 1.0);}
        s.b[1318] = (((p[25] == 1.0) && (p[26] == 2.0)) && (p[43] == 1.0));s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {s.store_mul_scaled_exp_scaled_input_rhs(1319, 107, (1.6021918e-19 * p[237]), 225, (-p[141]));s.store_offset_scaled(1320, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(1321, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 1319, 1320, 1.0);s.store_mul(567, 263, 1321);s.store_mul_scaled_ln_offset_rhs(1322, 227, p[140], 567, 1.0);s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1322, (-1.0), 231, (-0.01));s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));}
        if s.b[1318] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1318] {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(1322, 231, 1.0, 44, (-0.5), 45, (-0.5));s.store_sqrt_mul_scaled_lhs(1323, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_add_scaled_product_mixed_aia(1324, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(176), s.ad_value(1322)))), (-1.0)), 1.0, 225, A::sub(s.ad_value(176), s.ad_value(1322)), 1.0);}
        if s.b[1318] {
            if (s.v[1324] > 0.0) {
                s.store_sqrt(1324, 1324);
            } else {
                s.store_scaled_sqrt_scaled_input(1324, 1324, -1.0, -1.0);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1318] {s.store_sqrt_ad(1325, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(176))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(176), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(1326, 1323, 1324, -1.0, 1325, -1.0);s.store_offset_sub_from_scalar_ad(44, p[47], s.ad_value(1326), (-(p[47] * 0.01)));s.store_scalar(45, ((4.0 * p[47]) * (p[47] * 0.01)));}
        if s.b[1318] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1318] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p[47]);}
        if s.b[1318] {s.store_scalar(1319, (if (p[138] > 0.0) { p[138] } else { 1.0 }));}
        if s.b[1318] {s.store_div_scaled_value_offset_denominator(398, s.ad_value(1319), 1.0, s.ad_value(263), p[139], 1.0);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        s.b[1340] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p[146] != 0.0));s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });s.b[1341] = (s.v[56] < 3.0);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1341]) {s.store_scalar(516, 0.0);s.store_scalar(517, 0.0);}
        if (s.b[1340] && (!s.b[1341])) {
            if (p[43] == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }
        if (s.b[1340] && (!s.b[1341])) {
            if (p[43] == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }
        if s.b[1340] {s.store_offset_scaled(1327, 185, p[147], 1.0);s.store_scaled_mul(1328, 1327, 263, p[146]);s.store_offset_mul_ad(1329, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));s.store_sqrt_square_offset(44, 1329, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1329, 1329, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1342] = (s.v[1329] < 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1342]) {s.store_scalar(1329, 0.0);}
        if s.b[1340] {s.store_sqrt(1330, 1329);s.store_mul(1331, 1329, 1330);s.store_offset_mul_ad(1332, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));s.store_sqrt_square_offset(44, 1332, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1332, 1332, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1343] = (s.v[1332] < 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1343]) {s.store_scalar(1332, 0.0);}
        if s.b[1340] {s.store_sqrt(1333, 1332);s.store_mul(1334, 1332, 1333);s.store_div_from_scalar(1335, 1.0, 1329);s.store_mul3_lhs(328, 225, 1328, 1335);s.store_div_from_scalar(1335, 1.0, 1332);s.store_mul3_lhs(1336, 225, 1328, 1335);s.store_mul_mixed_ia(1337, 238, A::add_scaled_products(s.ad_value(1334), s.ad_value(1336), 1.0, s.ad_value(1331), s.ad_value(328), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(1338, 238, 1333, 1336, ((-1.0) * (0.5)), 1330, 328, 0.5);s.store_add(1339, 1337, 1338);s.store_mul3_lhs(265, 264, 1339, 250);}
        s.store_scalar(1357, (s.v[88] * 100.0));s.store_scale(1358, 323, 0.0001);s.store_scalar(1359, (s.v[97] * 100.0));s.store_primal_scale(1360, 107, 100.0);s.store_scale(1361, 252, 0.01);s.store_scale(1362, 436, 0.0001);s.store_scale(1363, 238, 0.0001);s.b[1364] = (p[27] == 0.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_scalar(309, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(308, 0.0);s.store_scalar(310, 0.0);}
        s.b[1365] = (s.v[145] == 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1365]) {s.store_offset_add(1356, 176, 173, (-(10.0 * 2.220446049250313e-16)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1364]) && s.b[1365]) {s.store_add_scaled_inputs4_offset_indices(1346, 174, 1.0, 185, (p[216] * s.v[1359]), 320, (-(p[216] * s.v[1359])), 1356, (-p[215]), (-s.v[123]));s.store_scalar(1348, (1.0 / s.v[1357]));s.store_mul(1347, 1346, 1348);s.store_scalar(1348, (1.0 / p[217]));s.store_offset_mul(1352, 1361, 1348, 1.0);s.store_mul(1355, 1347, 1352);s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1366] = (s.v[1355] < 0.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if (((!s.b[1364]) && s.b[1365]) && s.b[1366]) {s.store_scalar(1355, 0.0);}
        if ((!s.b[1364]) && s.b[1365]) {s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1348, 174, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1367] = (s.v[1348] < 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (((!s.b[1364]) && s.b[1365]) && s.b[1367]) {s.store_scalar(1348, 0.0);}
        if ((!s.b[1364]) && s.b[1365]) {s.store_offset(1348, 1348, (-p[226]));s.store_scale(1344, 1348, 10.0);s.store_offset_square(1347, 1344, 1.0);s.store_sub_from_scalar_ad(1346, 1.0, A::div_from_scalar(1.0, s.ad_value(1347)));s.store_mul(1355, 1355, 1346);s.store_scale(1345, 1360, s.v[1359]);s.store_div_from_scalar_offset_input(1352, p[219], 1345, p[219]);s.store_scalar(1351, p[218]);s.store_div_add_scaled_inputs_rhs_indices(1353, 1351, 1351, 1.0, 173, 1.0);s.store_div_from_scalar_offset_input(1349, 1.0, 1355, 1e-50);s.store_scaled_mul(1346, 303, 1349, (-p[214]));}
        s.b[1368] = (s.v[1346] < (-34.0));s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((!s.b[1364]) && s.b[1365]) && s.b[1368]) {s.store_scalar(309, 0.0);}
        if (((!s.b[1364]) && s.b[1365]) && (!s.b[1368])) {s.store_exp(1347, 1346);s.store_mul_scale_offset_mixed_ia(1348, 1345, A::div_from_scalar(p[213], s.ad_value(302)), 1.6021918e-19, 0.0);s.store_div_from_scalar(1350, 1.0, 1363);s.store_sqrt_mul_ad(1351, A::add_scaled_inputs(s.ad_value(1362), 1.0, s.ad_value(1358), 1e-12), s.ad_value(1350));s.store_mul3_lhs(1349, 1347, 1348, 1351);s.store_mul3_lhs(1354, 1349, 1355, 1355);s.store_mul3_lhs(309, 1352, 1353, 1354);}
        if ((!s.b[1364]) && (!s.b[1365])) {s.store_scalar(309, 0.0);}
        if (!s.b[1364]) {s.store_offset_scaled(1345, 158, (-p[221]), p[222]);s.store_exp_scaled_input(1347, 1345, s.v[1357]);s.store_scale(1345, 158, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));s.store_mul(1348, 158, 1345);s.store_scale(1349, 1360, (p[220] / 1000000.0));s.store_mul3_lhs(306, 1349, 1347, 1348);}
        s.b[1369] = (s.v[158] >= 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1369]) {s.store_scale(306, 306, (-1.0));}
        if (!s.b[1364]) {s.store_sub(1346, 158, 157);s.store_offset_scaled(1345, 1346, (-p[221]), p[222]);s.store_exp_scaled_input(1347, 1345, s.v[1357]);s.store_scale(1345, 1346, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));s.store_mul(1348, 1346, 1345);s.store_scale(1349, 1360, (p[220] / 1000000.0));s.store_mul3_lhs(307, 1349, 1347, 1348);}
        s.b[1370] = (s.v[1346] >= 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1370]) {s.store_scale(307, 307, (-1.0));}
        if (!s.b[1364]) {s.store_offset_scaled_sub(1355, 513, 158, 1.0 / (s.v[1357]), ((((s.v[123]) + (p[225]))) * (1.0 / (s.v[1357]))));s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1371] = (s.v[1355] < 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1371]) {s.store_scalar(1355, 0.0);}
        if (!s.b[1364]) {s.store_offset(1355, 1355, 1e-50);s.store_div_from_scalar(1346, (-p[224]), 1355);}
        s.b[1372] = (s.v[1346] < (-34.0));s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1372]) {s.store_scalar(308, 0.0);}
        if ((!s.b[1364]) && (!s.b[1372])) {s.store_exp(1347, 1346);s.store_scale(1348, 1360, (p[223] * s.v[1359]));s.store_mul_product3_indices(308, 1347, 1348, 1355, 1355, 1.0);}
        if (!s.b[1364]) {s.store_scalar(310, 0.5);}
        s.b[1380] = (p[28] == 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if s.b[1380] {s.store_scalar(311, 0.0);}
        if (!s.b[1380]) {s.store_add_scaled_inputs4_offset_indices(1373, 157, p[209], 158, (-1.0), 187, p[211], 319, p[211], (p[210] * p[209]));s.store_scalar(1374, (1.0 / s.v[88]));s.store_mul(1375, 1373, 1374);s.store_sqrt_square_offset(44, 1375, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(304, 1375, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1381] = (s.v[304] < 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1381]) {s.store_scalar(304, 0.0);}
        if (!s.b[1380]) {s.store_div_from_scalar_offset_input(1376, 1.0, 304, 1e-50);s.store_scaled_mul(1377, 303, 1376, (-p[208]));}
        s.b[1382] = (s.v[1377] < (-34.0));s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1382]) {s.store_scalar(311, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1380]) && (!s.b[1382])) {s.store_exp(1373, 1377);s.store_mul_scale_offset_mixed_ia(1374, 107, A::div_from_scalar(p[207], s.ad_value(302)), 1.6021918e-19, 0.0);s.store_mul_product3_indices(311, 1373, 1374, 304, 304, 1.0);}
        if (!s.b[1380]) {s.store_sub(1379, 157, 513);}
        s.b[1383] = (s.v[1379] > 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1383]) {s.store_square(1374, 1379);s.store_mul(331, 1374, 1379);s.store_offset(1377, 331, p[212]);s.store_div(1378, 331, 1377);s.store_mul(311, 311, 1378);}
        if ((!s.b[1380]) && (!s.b[1383])) {s.store_scalar(311, 0.0);}
        s.b[1391] = (p[28] == 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if s.b[1391] {s.store_scalar(312, 0.0);}
        if (!s.b[1391]) {s.store_add_scaled_inputs3_mixed_aii(1384, A::add_scaled_inputs3_offset(s.ad_value(157), (-p[209]), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p[210]) * (p[209]))), 1.0, 187, p[211], 319, p[211]);s.store_scalar(1385, (1.0 / s.v[88]));s.store_mul(1386, 1384, 1385);s.store_sqrt_square_offset(44, 1386, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(305, 1386, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1392] = (s.v[305] < 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1392]) {s.store_scalar(305, 0.0);}
        if (!s.b[1391]) {s.store_div_from_scalar_offset_input(1387, 1.0, 305, 1e-50);s.store_scaled_mul(1388, 303, 1387, (-p[208]));}
        s.b[1393] = (s.v[1388] < (-34.0));s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1393]) {s.store_scalar(312, 0.0);}
        if ((!s.b[1391]) && (!s.b[1393])) {s.store_exp(1384, 1388);s.store_div_from_scalar(1387, 1.0, 302);s.store_scaled_mul(1385, 1387, 107, (p[207] * 1.6021918e-19));s.store_mul_product3_indices(312, 1384, 1385, 305, 305, 1.0);}
        if (!s.b[1391]) {s.store_neg(1390, 513);}
        s.b[1394] = (s.v[1390] > 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1394]) {s.store_square(1385, 1390);s.store_mul(331, 1385, 1390);s.store_offset(1388, 331, p[212]);s.store_div(1389, 331, 1388);s.store_mul(312, 312, 1389);}
        if ((!s.b[1391]) && (!s.b[1394])) {s.store_scalar(312, 0.0);}
        s.b[1395] = (p[43] == 1.0);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if s.b[1395] {s.store_scalar(1405, s.v[91]);s.store_primal_div_from_scalar(1406, 1.0, 1405);s.store_scalar(1462, 0.0);s.store_scalar(1464, 0.0);s.store_scalar(1466, 0.0);s.store_neg(1398, 534);s.store_mul(1399, 1398, 436);s.store_add_scaled_product_indices(331, 1399, 1.0, 1398, 437, 1.0);s.store_mul(470, 1399, 438);s.store_sub(469, 1399, 470);s.store_mul(468, 331, 438);s.store_sub(467, 331, 468);}
        if (s.b[1395] && (p[24] != 0.0)) {s.copy_ad(521, 536);}
        let (t40,) = {
    if (s.b[1395] && (p[24] != 0.0)) {
        (0.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t40);s.b[1475] = (1.0 == 1.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });s.b[1476] = (1.0 == 2.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1475]) {s.store_primal_scale(522, 533, 0.5);s.store_scalar(523, p[292]);}
        let (t41,) = {
    if ((s.b[1395] && (p[24] != 0.0)) && s.b[1475]) {
        (s.v[525],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t41);
        if ((s.b[1395] && (p[24] != 0.0)) && (s.b[1476] && (!s.b[1475]))) {s.store_primal_scale(522, 534, 0.5);s.store_scalar(523, p[68]);}
        let (t42,) = {
    if ((s.b[1395] && (p[24] != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
        (s.v[524],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t42);
        let (t43,) = {
    if ((s.b[1395] && (p[24] != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
        (1.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t43);s.b[1477] = (s.v[528] == 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_mul_sqrt_mixed_ia(1425, 238, A::div(s.ad_value(521), s.ad_value(536)));s.store_scalar(1407, ((1.0 - -1.0) / 2.0));s.store_scalar(1408, ((1.0 + -1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_add_scaled_products_mixed_iiia(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1422, 1419, 1418);s.store_neg(1423, 1418);s.store_primal_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);s.store_primal_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1396, 1416);}
        s.b[1478] = (s.v[1396] > s.v[141]);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1478]) {s.store_sub(1397, 1396, 141);s.store_sub(1398, 140, 141);s.store_div(44, 1397, 1398);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1404, 1398, 1404, -1.0, 1.0);s.store_add(1401, 141, 1404);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1478])) {s.copy_ad(1401, 1396);}
        if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));s.store_mul(1426, 1425, 1406);s.store_square(1427, 1426);s.store_sub(1428, 1424, 523);s.store_div(1396, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));}
        let (t45,) = {
    if ((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) {
        let t44: f64 = (-s.v[1417]);
        (t44,)
    } else {
        (s.v[1430],)
    }
};
        s.store_scalar(1430, t45);s.b[1479] = (s.v[1428] < s.v[1430]);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1479]) {s.store_div_scalar_by_product_indices(1397, 1.0, 225, 1425, 1.0);s.store_mul(1404, 1397, 1405);s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);s.store_sub(1433, 237, 1429);s.store_mul_add_rhs(1403, 225, 1428, 1417);s.store_sub_from_scalar_scaled_mul_mixed_ia(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);s.store_square(1435, 1434);}
        s.b[1480] = (s.v[1432] < (s.v[1435] * 1e-8));s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1479]) && s.b[1480]) {s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);}
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1479]) && (!s.b[1480])) {s.store_sqrt_add(1436, 1432, 1435);s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && s.b[1479]) {s.store_powf(1438, 1437, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);s.store_div(1440, 1439, 1438);s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);s.store_add(1397, 1441, 1417);s.store_div(1398, 1397, 1433);s.store_sqrt_square_offset(1399, 1398, 1.0);s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);s.store_sub(1398, 1428, 1442);s.store_mul(459, 1405, 1398);s.copy_ad(458, 459);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_scalar(1440, 3.0);s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);s.store_exp_neg_input(1404, 1440);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1481] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1481]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1482] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1482]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        s.b[1483] = (s.v[1440] < 3.0);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1483]) {s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), 1.0, 1444, 1444, 9.0);s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_offset_add(1452, 1428, 1417, 0.1);s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1454, 1453, 1459);s.store_mul(1396, 226, 1427);s.store_mul(1455, 225, 1452);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.store_offset_sub(44, 1455, 1456, (-1.0));s.store_scale(45, 1455, 4.0);}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1455, 1455, 1456);s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.copy_ad(1458, 1440);s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);s.store_add_offset_lhs_mixed_ia(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));}
        s.b[1484] = (s.v[1397] < (10.0 * 2.220446049250313e-16));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1484]) {s.store_scalar(1397, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt(1398, 1397);s.store_mul(458, 1425, 1398);s.store_mul_sub_rhs(459, 1405, 1428, 1442);}
        s.b[1485] = (p[42] == 1.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1468, 1453, 1459);}
        let (t46,) = {
    if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
        (0.0,)
    } else {
        (s.v[1413],)
    }
};
        s.store_scalar(1413, t46);
        if ((((s.b[1395] && (p[24] != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_scalar(167, 1.0);}
    }
}
