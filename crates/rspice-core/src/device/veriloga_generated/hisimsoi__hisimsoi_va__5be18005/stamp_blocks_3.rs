#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1133] && s.b[1196]) && s.b[1198]) {s.store_add_scaled_inputs3_indices(1192, 161, 1.0, 1193, 1.0, 172, -1.0);}
        if ((s.b[1133] && s.b[1196]) && (!s.b[1198])) {s.store_add_scaled_inputs3_indices(1192, 161, 1.0, 1193, 1.0, 350, -1.0);}
        if (s.b[1133] && s.b[1196]) {s.store_add_product3_rhs_indices(1191, 1191, 173, 1188, 1192, 1.0);s.store_mul(1193, 1190, 1191);s.copy_ad(1190, 1193);}
        if (s.b[1133] && (!s.b[1196])) {s.store_scalar(1190, 0.0);}
        s.b[1199] = (p[248] != 0.0);s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1199]) {s.store_scale(1187, 225, s.v[118]);s.store_mul(1195, 323, 1187);s.store_mul(1194, 1195, 173);}
        if (s.b[1133] && (!s.b[1199])) {s.store_scalar(1194, 0.0);}
        s.b[1200] = ((s.v[1190] + s.v[1194]) > 0.0);s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1200]) {s.store_mul_add_rhs(247, 164, 1190, 1194);s.store_mul3_lhs(201, 264, 247, 250);}
        if s.b[1133] {s.store_add(199, 200, 201);s.copy_ad(203, 201);}
        s.b[1210] = (p[33] != 0.0);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1210]) {s.copy_ad(1203, 549);s.store_scalar(1204, (s.v[124] - p[71]));s.store_div_from_scalar_square_ad(1205, 1.0, s.ad_value(1204));s.store_mul_ad_product_lhs_mixed_ai(1206, A::mul_sub_from_scalar_lhs_scaled_output(p[69], s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), 1203, 1205);s.store_mul(186, 1206, 235);s.store_offset_scaled(1202, 173, p[155], p[154]);s.store_mul(206, 186, 1202);s.store_sub_from_scalar_scaled_input(1201, p[156], 157, p[157]);s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1201, 1.0, 206, 1.0, (-s.v[123]));s.store_mul3_lhs(210, 205, 324, 324);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_offset_sub_ad(1207, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p[156])))), s.ad_value(206), 1e-50);s.store_offset_sub(1201, 174, 1207, (-0.005));}
        if (s.b[1133] && s.b[1210]) {s.store_scalar(327, (if (s.v[1207] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_add_scaled_square_product(1203, 1201, 1.0, 327, 1207, (4.0 * 0.005));s.store_sub_mixed_ai(1204, A::add_scaled_inputs4_offset(s.ad_value(1207), 1.0, s.ad_value(1201), 0.5, s.ad_value(1203), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p[156]))), 514);s.store_offset_mul(1205, 225, 1204, (-1.0));s.store_div_from_scalar(1206, 4.0, 212);s.store_offset_mul(1202, 1205, 1206, 1.0);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1211] = (s.v[1201] < 0.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1211]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(213, 1201, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));s.store_mul_ln_mixed_ia(216, 327, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))));s.store_div_scaled_value_offset_denominator(1204, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);s.store_offset_sub(217, 216, 215, (-0.002));s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));s.store_div_from_scalar(1201, 1.0, 327);s.store_mul_exp_mixed_ia(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1133] && s.b[1210]) {s.store_add_offset_lhs_mixed_ai(1202, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1212] = (s.v[1201] < 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1212]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(219, 1201, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(1202, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1213] = (s.v[1201] < 0.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1213]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_sqrt_offset_input(220, 1201, (10.0 * 2.220446049250313e-16));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_sub(1202, 215, 218);s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1201, 1202, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1214] = (s.v[1201] < 0.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1210]) && s.b[1214]) {s.store_scalar(1201, 0.0);}
        if (s.b[1133] && s.b[1210]) {s.store_div_scaled_value_offset_denominator(1208, s.ad_value(157), 1.0, s.ad_value(1201), (10.0 * 2.220446049250313e-16), 1.0);s.store_square(49, 1208);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t0,) = {
    if (s.b[1133] && s.b[1210]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t0);
        let (t1,) = {
    if (s.b[1133] && s.b[1210]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t1);
        if (s.b[1133] && s.b[1210]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1215] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });s.b[1216] = (4.0 == 1.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        let (t2,) = {
    if (((s.b[1133] && s.b[1210]) && s.b[1215]) && s.b[1216]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2);s.b[1217] = (4.0 == 2.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        let (t3,) = {
    if ((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3);s.b[1218] = (4.0 == 4.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        let (t4,) = {
    if (((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && s.b[1218]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t4);s.b[1219] = (4.0 == 8.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        let (t5,) = {
    if ((((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && (!s.b[1218])) && s.b[1219]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t5);
        let (t6,) = {
    if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6);let mut ta: usize = 0;
        while {
            let t9: f64 = if (((s.b[1133] && s.b[1210]) && s.b[1215]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1133] && s.b[1210]) && s.b[1215]) {s.store_sqrt(53, 53);}
            let (t8,) = {
    if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
        let t7: f64 = (s.v[54] + 1.0);
        (t7,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t8);
        }
        if ((s.b[1133] && s.b[1210]) && (!s.b[1215])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (s.b[1133] && s.b[1210]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(1209, 1208, 53, 1.0);s.store_scale(214, 227, ((2.0 * s.v[126]) * p[9]));s.store_div_scaled_product_mixed_aii(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1209, 1.0, 441, 1.0);s.store_add(199, 199, 222);}
        s.b[1220] = ((p[30] != 0.0) && (p[32] != 0.0));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (s.b[1133] && s.b[1220]) {s.store_square(294, 192);s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1133] && s.b[1220]) {s.store_sub(296, 294, 295);s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1221] = (s.v[294] < 0.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1220]) && s.b[1221]) {s.store_scalar(294, 0.0);}
        if (s.b[1133] && s.b[1220]) {s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1222] = (s.v[296] < 0.0);s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if ((s.b[1133] && s.b[1220]) && s.b[1222]) {s.store_scalar(296, 0.0);}
        if (s.b[1133] && s.b[1220]) {s.store_sub(297, 294, 296);}
        s.b[1223] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((s.b[1133] && s.b[1220]) && s.b[1223]) {
        (0.0,)
    } else {
        (s.v[146],)
    }
};
        s.store_scalar(146, tb);
        let (tc,) = {
    if ((s.b[1133] && s.b[1220]) && (!s.b[1223])) {
        (1.0,)
    } else {
        (s.v[146],)
    }
};
        s.store_scalar(146, tc);s.copy_ad(202, 199);s.store_scalar(204, 0.0);s.b[1224] = ((p[281] > 0.0) && (p[285] > 0.0));s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if s.b[1224] {s.store_scalar(1231, s.v[99]);s.store_scalar(1235, p[237]);s.store_offset_add_scaled_inputs3_offset_indices(1236, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p[286]));}
        let (te,) = {
    if s.b[1224] {
        let td: f64 = (s.v[182] + p[286]);
        (td,)
    } else {
        (s.v[1237],)
    }
};
        s.store_scalar(1237, te);
        if s.b[1224] {s.store_scalar(1239, p[285]);s.store_scalar(1238, p[283]);s.store_scalar(1229, s.v[70]);s.store_mul_ln_mixed_ia(1230, 227, A::div_scaled_product_by_product(s.ad_value(1229), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));}
        if s.b[1224] {
            if (p[43] == 1.0) {
                s.copy_ad(1227, 435);
            } else {
                s.copy_ad(1227, 350);
            }
        }
        if s.b[1224] {s.store_sqrt_ad(1232, A::div_scaled_product3(A::sub(s.ad_value(1230), s.ad_value(1227)), s.ad_value(536), s.ad_value(1229), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1229)), 1.0));s.store_mul(1226, 1232, 1231);s.store_div_scaled_product_add_scaled_denominator_indices(1225, 1226, 1226, (-0.25), 157, 1.0, 1226, 1.0, 1.0);s.copy_ad(1251, 1225);}
        let (tf,) = {
    if s.b[1224] {
        (s.v[1237],)
    } else {
        (s.v[1252],)
    }
};
        s.store_scalar(1252, tf);
        if s.b[1224] {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1236), s.ad_value(1251))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if s.b[1224] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[1224] {s.store_add_product3_rhs_mixed_iia(376, 1236, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);}
        s.b[1253] = (s.v[158] < ((s.v[123] + s.v[1252]) * 0.5));s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        let (t10,) = {
    if (s.b[1224] && s.b[1253]) {
        (0.0,)
    } else {
        (s.v[144],)
    }
};
        s.store_scalar(144, t10);s.b[1254] = ((s.v[144] == 0.0) || (1.0 != 0.0));s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1254]) {s.store_mul_sub_rhs(181, 225, 376, 1251);}
        s.b[1255] = (s.v[181] < 3.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((s.b[1224] && s.b[1254]) && s.b[1255]) {s.store_mul_sub_rhs(337, 225, 1236, 1251);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1224] && s.b[1254]) && s.b[1255]) {s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 1251, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[1256] = ((s.v[158] - s.v[383]) <= s.v[1252]);s.store_scalar(1256, if s.b[1256] { 1.0 } else { 0.0 });s.b[1257] = (p[43] == 0.0);s.store_scalar(1257, if s.b[1257] { 1.0 } else { 0.0 });
        if ((((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {s.copy_ad(378, 376);}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        s.b[1258] = (p[43] == 0.0);s.store_scalar(1258, if s.b[1258] { 1.0 } else { 0.0 });s.b[1259] = ((s.v[158] - s.v[383]) <= s.v[1252]);s.store_scalar(1259, if s.b[1259] { 1.0 } else { 0.0 });
        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && s.b[1259]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);s.copy_ad(378, 376);}
        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1235, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) {s.store_mul_mixed_ia(331, 330, A::add_scaled_inputs_product(s.ad_value(1236), 1.0, s.ad_value(475), (-1.0), A::add_scaled_inputs(s.ad_value(329), 1.0, s.ad_value(328), 0.5), s.ad_value(369), -1.0));s.store_sub_div_rhs_indices(376, 1236, 331, 323);s.copy_ad(378, 376);}
        s.b[1260] = ((s.v[1236] - s.v[383]) > 0.0);s.store_scalar(1260, if s.b[1260] { 1.0 } else { 0.0 });
        if ((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul_ad_product_rhs(329, 328, A::sub(s.ad_value(1236), s.ad_value(383)), A::sub(s.ad_value(1236), s.ad_value(383)));s.store_add_mixed_ia(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p[287]);}
        s.b[1261] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));s.store_scalar(1261, if s.b[1261] { 1.0 } else { 0.0 });
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {s.store_offset_sub_scaled_inputs_indices(44, 376, 1.0, 377, 0.98, 0.4);s.store_square(49, 44);s.store_scalar(50, (0.4 * 0.4));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t11,) = {
    if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t11);
        let (t12,) = {
    if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t12);
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1262] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });s.b[1263] = (2.0 == 1.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        let (t13,) = {
    if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && s.b[1263]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t13);s.b[1264] = (2.0 == 2.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        let (t14,) = {
    if ((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && s.b[1264]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t14);s.b[1265] = (2.0 == 4.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        let (t15,) = {
    if (((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && s.b[1265]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t15);s.b[1266] = (2.0 == 8.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        let (t16,) = {
    if ((((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t16);
        let (t17,) = {
    if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t17);let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;
            if t1b > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1b, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {s.store_sqrt(53, 53);}
            let (t19,) = {
    if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
        let t18: f64 = (s.v[54] + 1.0);
        (t18,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t19);
        }
        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && (!s.b[1262])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 0.4);s.store_add_mixed_ai(378, A::scale_offset(s.ad_value(377), 0.98, (-0.4)), 43);}
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && (!s.b[1261])) {s.copy_ad(378, 376);}
        if s.b[1224] {s.store_offset(336, 1251, (5e-12 / 2.0));}
        s.b[1267] = (s.v[378] < s.v[336]);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1267]) {s.copy_ad(378, 336);}
        if s.b[1224] {s.copy_ad(1234, 378);s.copy_ad(163, 376);}
        if (s.b[1224] && (0.0 != 0.0)) {
            if ((s.v[376] - s.v[1234]) >= 0.0) {
                s.store_sub(166, 376, 1234);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if (s.b[1224] && (0.0 != 0.0)) {s.store_offset_scaled(44, 166, (1.0 + 0.3), (((-p[287])) + ((-0.03))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1224] && (0.0 != 0.0)) {s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (s.b[1224] && (0.0 != 0.0)) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if (s.b[1224] && (0.0 != 0.0)) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1268] = (s.v[165] < 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if ((s.b[1224] && (0.0 != 0.0)) && s.b[1268]) {s.store_scalar(165, 0.0);}
        s.b[1269] = (s.v[165] > s.v[157]);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if (((s.b[1224] && (0.0 != 0.0)) && (!s.b[1268])) && s.b[1269]) {s.copy_ad(165, 157);}
        if (s.b[1224] && (0.0 != 0.0)) {s.store_add(163, 1234, 165);}
        s.b[1270] = (p[282] == 1.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1270]) {s.copy_ad(378, 1234);s.copy_ad(1271, 1225);}
        let (t20,) = {
    if (s.b[1224] && s.b[1270]) {
        let t1c: f64 = (s.v[123] - s.v[185]);let t1d: f64 = (t1c + s.v[320]);let t1e: f64 = (t1d + s.v[1271]);let t1f: f64 = (t1e + p[286]);
        (t1f,)
    } else {
        (s.v[160],)
    }
};
        s.store_scalar(160, t20);s.b[1273] = (s.v[158] < s.v[160]);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        let (t22,) = {
    if ((s.b[1224] && s.b[1270]) && s.b[1273]) {
        let t21: f64 = (-1.0);
        (t21,)
    } else {
        (s.v[338],)
    }
};
        s.store_scalar(338, t22);
        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));s.store_mul_sub_rhs(336, 225, 1236, 1271);s.store_div_scalar_by_product_indices(328, 1.0, 225, 238, 1.0);s.store_mul(337, 328, 323);s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);s.store_offset(331, 336, (-2.0));s.store_scaled_mul(332, 337, 331, 9.0);s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);s.store_square(259, 261);}
        s.b[1274] = (s.v[260] < (s.v[259] * 1e-8));s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && s.b[1274]) {s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));}
        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && (!s.b[1274])) {s.store_sqrt_add(258, 260, 259);s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);}
        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {s.store_powf(256, 257, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);s.store_div_from_scalar(328, 1.0, 256);s.store_mul(181, 255, 328);s.store_add_scaled_product_indices(313, 1271, 1.0, 181, 227, 1.0);s.store_sub(328, 313, 1271);s.store_div(329, 328, 254);s.store_sqrt_square_offset(330, 329, 1.0);s.store_add_div_lhs_indices(1234, 328, 330, 1271);}
        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_exp_ad(484, A::mul_offset_rhs(s.ad_value(225), s.ad_value(1271), (-p[287])));}
        let (t23,) = {
    if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
        (0.0,)
    } else {
        (s.v[430],)
    }
};
        s.store_scalar(430, t23);
        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.copy_ad(1272, 378);s.store_scale(419, 229, ((p[237] * (p[237] * 0.5)) * 9662367879.197212));s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);s.store_div_ln_lhs(420, 328, 419);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t29: usize = 0;
        while {
            let t27: f64 = (s.v[57] + 1.0);let t28: f64 = if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (s.v[167] <= t27)) { 1.0 } else { 0.0 };
            t28 != 0.0
        } {
            t29 += 1;
            if t29 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t29, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_sub(417, 1272, 1271);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1275] = (s.v[337] < 80.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1275]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1275])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_mul(421, 225, 422);}
            s.b[1276] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1277] = (s.v[181] < 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) && s.b[1277]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1278] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && s.b[1278]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && (!s.b[1278])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1279] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
            let (t25,) = {
    if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1279]) {
        let t24: f64 = (-1.0);
        (t24,)
    } else {
        (s.v[338],)
    }
};
            s.store_scalar(338, t25);s.b[1280] = (s.v[181] < 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1280]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1281] = (s.v[181] < 1e-7);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && s.b[1281]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && (!s.b[1281])) {s.store_mul_scale_offset_indices(501, 225, 1272, 1.0, (-p[287]));s.store_exp(502, 501);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_add_scaled_inputs_product_indices(492, 1272, 1.0, 1236, (-1.0), 240, 490, 1.0);s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1282] = (s.v[430] == 1.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1282]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1272]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1272))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1283] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1283]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {s.store_add(1272, 1272, 494);}
            s.b[1284] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
            let (t26,) = {
    if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1284]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t26);
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.copy_ad(1234, 1272);}
        if s.b[1224] {s.store_mul_sub_scaled_inputs_rhs_indices(332, 225, 1234, -1.0, 1225, -1.0);}
        if s.b[1224] {s.store_scalar(1249, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));}
        if s.b[1224] {s.store_mul(1250, 1249, 332);s.store_exp(333, 332);s.store_sub_offset_lhs(334, 333, (-1.0), 332);}
        s.b[1285] = (s.v[332] > 1e-7);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1285]) {s.store_mul_scaled_sqrt_rhs(437, 238, -1.0, 334);}
        s.b[1286] = (s.v[1250] > 1e-7);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if ((s.b[1224] && (!s.b[1285])) && s.b[1286]) {s.store_mul_sqrt_rhs(437, 238, 334);}
        if ((s.b[1224] && (!s.b[1285])) && (!s.b[1286])) {s.store_mul_ad_affine_product_rhs(437, 1249, s.ad_value(1250), A::sqrt_scaled_lhs_product_offset(s.ad_value(1250), 0.3333333333333333, A::scale_offset(s.ad_value(1250), 0.25, 1.0), 1.0), (-0.7071067811865475), 0.0);}
        if s.b[1224] {s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));s.store_offset_add_scaled_inputs_indices(1246, 437, 0.5, 44, 0.5, (1e-10 * 1e-6));}
        s.b[1287] = (s.v[1246] < 0.0);s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1287]) {s.store_scalar(1246, 0.0);}
        if s.b[1224] {s.store_div_scaled_inputs_indices(1247, 1246, 1.0, 536, 1.6021918e-19);s.store_sub(328, 1247, 1238);s.store_scale(1248, 1247, 0.01);s.store_sqrt_add_scaled_square_product(44, 328, 1.0, 1248, 1248, 4.0);s.store_add_scaled_inputs3_indices(329, 328, 0.5, 44, 0.5, 1248, 1e-10);}
        s.b[1288] = (s.v[329] < 0.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1288]) {s.store_scalar(329, 0.0);}
        if s.b[1224] {s.store_div_scaled_product_by_product_indices(1245, 329, 329, 1.0, 1247, 1247, 1.0);s.store_add_scaled_product_mixed_iai(1228, 1225, 1.0, A::sub(s.ad_value(1234), s.ad_value(1225)), 1245, 1.0);s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1228))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1228), s.ad_value(157)))));s.store_primal_sqrt_scaled_input(1241, 1229, ((2.0 * 1.6021918e-19) * 1.034943e-10));s.store_mul_sqrt_rhs(1242, 1241, 227);s.store_mul_sub_rhs(1233, 225, 1228, 1225);}
        s.b[1289] = ((s.v[1233] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if (s.b[1224] && s.b[1289]) {s.store_sub_scaled_inputs(44, 225, 0.2, 1233, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 225, 225, (0.2 * 0.2));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t2a,) = {
    if (s.b[1224] && s.b[1289]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t2a);
        let (t2b,) = {
    if (s.b[1224] && s.b[1289]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2b);
        if (s.b[1224] && s.b[1289]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1290] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });s.b[1291] = (1.0 == 1.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        let (t2c,) = {
    if (((s.b[1224] && s.b[1289]) && s.b[1290]) && s.b[1291]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2c);s.b[1292] = (1.0 == 2.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        let (t2d,) = {
    if ((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && s.b[1292]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2d);s.b[1293] = (1.0 == 4.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        let (t2e,) = {
    if (((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2e);s.b[1294] = (1.0 == 8.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        let (t2f,) = {
    if ((((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t2f);
        let (t30,) = {
    if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
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
            let t33: f64 = if (((s.b[1224] && s.b[1289]) && s.b[1290]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t33 != 0.0
        } {
            t34 += 1;
            if t34 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t34, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.b[1224] && s.b[1289]) && s.b[1290]) {s.store_sqrt(53, 53);}
            let (t32,) = {
    if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
        let t31: f64 = (s.v[54] + 1.0);
        (t31,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t32);
        }
        if ((s.b[1224] && s.b[1289]) && (!s.b[1290])) {s.store_powf(53, 53, (1.0 / 2.0));}
        if (s.b[1224] && s.b[1289]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);}
        if (s.b[1224] && (!s.b[1289])) {s.copy_ad(328, 1233);}
        if s.b[1224] {s.store_sqrt_offset_input(1243, 328, (10.0 * 2.220446049250313e-16));s.store_mul(1244, 1242, 1243);s.store_mul_div_scaled_inputs_indices(1240, 1244, 227, 2.0, 1231, 1.0);s.store_mul_product3_indices(204, 337, 1240, 1239, 107, 1.0);s.store_add(199, 202, 204);}
        s.store_add(201, 203, 204);s.b[1295] = ((p[43] == 1.0) || (p[45] == 1.0));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });s.b[1308] = ((s.v[145] == 1.0) || (p[25] == 0.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1295] && s.b[1308]) {s.store_scalar(263, 0.0);}
        s.b[1309] = ((p[117] <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1295] && (!s.b[1308])) && s.b[1309]) {s.store_scalar(263, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p[48]);}
        s.b[1310] = (p[44] <= 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.copy_ad(1296, 445);s.store_square(1303, 323);s.copy_ad(1304, 545);s.store_div(1298, 1304, 1303);s.store_div_from_scalar(1305, 2.0, 1304);s.store_mul(1299, 1305, 1303);s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1302, 1299, 1300, 1.0);s.store_sqrt_square_offset(44, 1302, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1301, 1302, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1311] = (s.v[1301] < 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1311]) {s.store_scalar(1301, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.store_offset(1301, 1301, 1e-50);s.store_sqrt(1301, 1301);s.store_add_scaled_product_mixed_aii(1306, A::mul_sub_from_scalar_rhs(s.ad_value(1298), 1.0, s.ad_value(1301)), 1.0, 1296, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(1307, 173, p[122], 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1306)), -1.0);s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1312] = (s.v[1307] < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1312]) {s.store_scalar(1307, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_mul(1296, 134, 445);s.store_div_square_rhs(1298, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1299, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p[49] * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1301, 1299, 1300, 1.0);s.store_scaled_offset(1303, 1299, 1.0, 2.0);}
        s.b[1313] = ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_sub_offset_lhs(44, 1303, 1e-50, 1301);s.store_square(49, 44);s.store_square(50, 1303);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t35,) = {
    if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t35);
        let (t36,) = {
    if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t36);
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.copy_ad(53, 48);}
        s.b[1314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });s.b[1315] = (4.0 == 1.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        let (t37,) = {
    if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && s.b[1315]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t37);s.b[1316] = (4.0 == 2.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        let (t38,) = {
    if (((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && s.b[1316]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t38);s.b[1317] = (4.0 == 4.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        let (t39,) = {
    if ((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t39);s.b[1318] = (4.0 == 8.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        let (t3a,) = {
    if (((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t3a);
        let (t3b,) = {
    if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t3b);let mut t3f: usize = 0;
        while {
            let t3e: f64 = if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t3e != 0.0
        } {
            t3f += 1;
            if t3f > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t3f, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {s.store_sqrt(53, 53);}
            let (t3d,) = {
    if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
        let t3c: f64 = (s.v[54] + 1.0);
        (t3c,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t3d);
        }
        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && (!s.b[1314])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 1303, 53);s.store_sub_offset_lhs(1301, 1303, 1e-50, 43);}
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && (!s.b[1313])) {
        }
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            if (s.v[1301] <= 0.0) {
                s.store_scalar(1301, 0.0);
            } else {
                s.store_sqrt(1301, 1301);
            }
        }
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_add_mul_sub_from_scalar_rhs_indices(1306, 1296, 1298, 1.0, 1301);s.store_div_from_scalar_offset_input(1297, s.v[100], 131, s.v[100]);s.store_add_scaled_inputs_product_indices(1307, 173, p[122], 176, 1.0, 1297, 1306, (-1.0));s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1319] = (s.v[1307] < 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1319]) {s.store_scalar(1307, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset(1307, 1307, 1e-50);s.store_ad_value(1297, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1307), 1.0));s.store_mul_product3_indices(263, 1297, 132, 1307, 199, 1.0);}
        s.b[1320] = (((p[25] == 1.0) && (p[26] == 2.0)) && (p[43] == 1.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if s.b[1320] {s.store_mul_scaled_exp_scaled_input_rhs(1321, 107, (1.6021918e-19 * p[237]), 225, (-p[141]));s.store_offset_scaled(1322, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(1323, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 1321, 1322, 1.0);s.store_mul(567, 263, 1323);s.store_mul_scaled_ln_offset_rhs(1324, 227, p[140], 567, 1.0);s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1324, (-1.0), 231, (-0.01));s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));}
        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1320] {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(1324, 231, 1.0, 44, (-0.5), 45, (-0.5));s.store_sqrt_mul_scaled_lhs(1325, 544, ((2.0 * 1.034943e-10) * 1.6021918e-19), 227);s.store_add_scaled_product_mixed_aia(1326, A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, A::sub(s.ad_value(176), s.ad_value(1324)))), (-1.0)), 1.0, 225, A::sub(s.ad_value(176), s.ad_value(1324)), 1.0);}
        if s.b[1320] {
            if (s.v[1326] > 0.0) {
                s.store_sqrt(1326, 1326);
            } else {
                s.store_scaled_sqrt_scaled_input(1326, 1326, -1.0, -1.0);
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
        if s.b[1320] {s.store_sqrt_ad(1327, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(176))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(176), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(1328, 1325, 1326, -1.0, 1327, -1.0);s.store_offset_sub_from_scalar_ad(44, p[47], s.ad_value(1328), (-(p[47] * 0.01)));s.store_scalar(45, ((4.0 * p[47]) * (p[47] * 0.01)));}
        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1320] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p[47]);}
        if s.b[1320] {s.store_scalar(1321, (if (p[138] > 0.0) { p[138] } else { 1.0 }));}
        if s.b[1320] {s.store_div_scaled_value_offset_denominator(398, s.ad_value(1321), 1.0, s.ad_value(263), p[139], 1.0);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        s.b[1342] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p[146] != 0.0));s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });s.b[1343] = (s.v[56] < 3.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1343]) {s.store_scalar(516, 0.0);s.store_scalar(517, 0.0);}
        if (s.b[1342] && (!s.b[1343])) {
            if (p[43] == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }
        if (s.b[1342] && (!s.b[1343])) {
            if (p[43] == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }
        if s.b[1342] {s.store_offset_scaled(1329, 185, p[147], 1.0);s.store_scaled_mul(1330, 1329, 263, p[146]);s.store_offset_mul_ad(1331, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));s.store_sqrt_square_offset(44, 1331, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1331, 1331, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1344] = (s.v[1331] < 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1344]) {s.store_scalar(1331, 0.0);}
        if s.b[1342] {s.store_sqrt(1332, 1331);s.store_mul(1333, 1331, 1332);s.store_offset_mul_ad(1334, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));s.store_sqrt_square_offset(44, 1334, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1334, 1334, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1345] = (s.v[1334] < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1345]) {s.store_scalar(1334, 0.0);}
        if s.b[1342] {s.store_sqrt(1335, 1334);s.store_mul(1336, 1334, 1335);s.store_div_from_scalar(1337, 1.0, 1331);s.store_mul3_lhs(328, 225, 1330, 1337);s.store_div_from_scalar(1337, 1.0, 1334);s.store_mul3_lhs(1338, 225, 1330, 1337);s.store_mul_mixed_ia(1339, 238, A::add_scaled_products(s.ad_value(1336), s.ad_value(1338), 1.0, s.ad_value(1333), s.ad_value(328), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(1340, 238, 1335, 1338, ((-1.0) * (0.5)), 1332, 328, 0.5);s.store_add(1341, 1339, 1340);s.store_mul3_lhs(265, 264, 1341, 250);}
        s.store_scalar(1359, (s.v[88] * 100.0));s.store_scale(1360, 323, 0.0001);s.store_scalar(1361, (s.v[97] * 100.0));s.store_primal_scale(1362, 107, 100.0);s.store_scale(1363, 252, 0.01);s.store_scale(1364, 436, 0.0001);s.store_scale(1365, 238, 0.0001);s.b[1366] = (p[27] == 0.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(309, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(308, 0.0);s.store_scalar(310, 0.0);}
        s.b[1367] = (s.v[145] == 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset_add(1358, 176, 173, (-(10.0 * 2.220446049250313e-16)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1366]) && s.b[1367]) {s.store_add_scaled_inputs4_offset_indices(1348, 174, 1.0, 185, (p[216] * s.v[1361]), 320, (-(p[216] * s.v[1361])), 1358, (-p[215]), (-s.v[123]));s.store_scalar(1350, (1.0 / s.v[1359]));s.store_mul(1349, 1348, 1350);s.store_scalar(1350, (1.0 / p[217]));s.store_offset_mul(1354, 1363, 1350, 1.0);s.store_mul(1357, 1349, 1354);s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1368] = (s.v[1357] < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1368]) {s.store_scalar(1357, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1350, 174, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1369] = (s.v[1350] < 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1369]) {s.store_scalar(1350, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset(1350, 1350, (-p[226]));s.store_scale(1346, 1350, 10.0);s.store_offset_square(1349, 1346, 1.0);s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));s.store_mul(1357, 1357, 1348);s.store_scale(1347, 1362, s.v[1361]);s.store_div_from_scalar_offset_input(1354, p[219], 1347, p[219]);s.store_scalar(1353, p[218]);s.store_div_add_scaled_inputs_rhs_indices(1355, 1353, 1353, 1.0, 173, 1.0);s.store_div_from_scalar_offset_input(1351, 1.0, 1357, 1e-50);s.store_scaled_mul(1348, 303, 1351, (-p[214]));}
        s.b[1370] = (s.v[1348] < (-34.0));s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1370]) {s.store_scalar(309, 0.0);}
        if (((!s.b[1366]) && s.b[1367]) && (!s.b[1370])) {s.store_exp(1349, 1348);s.store_mul_scale_offset_mixed_ia(1350, 1347, A::div_from_scalar(p[213], s.ad_value(302)), 1.6021918e-19, 0.0);s.store_div_from_scalar(1352, 1.0, 1365);s.store_sqrt_mul_ad(1353, A::add_scaled_inputs(s.ad_value(1364), 1.0, s.ad_value(1360), 1e-12), s.ad_value(1352));s.store_mul3_lhs(1351, 1349, 1350, 1353);s.store_mul3_lhs(1356, 1351, 1357, 1357);s.store_mul3_lhs(309, 1354, 1355, 1356);}
        if ((!s.b[1366]) && (!s.b[1367])) {s.store_scalar(309, 0.0);}
        if (!s.b[1366]) {s.store_offset_scaled(1347, 158, (-p[221]), p[222]);s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 158, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 158, 1347);s.store_scale(1351, 1362, (p[220] / 1000000.0));s.store_mul3_lhs(306, 1351, 1349, 1350);}
        s.b[1371] = (s.v[158] >= 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1371]) {s.store_scale(306, 306, (-1.0));}
        if (!s.b[1366]) {s.store_sub(1348, 158, 157);s.store_offset_scaled(1347, 1348, (-p[221]), p[222]);s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 1348, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 1348, 1347);s.store_scale(1351, 1362, (p[220] / 1000000.0));s.store_mul3_lhs(307, 1351, 1349, 1350);}
        s.b[1372] = (s.v[1348] >= 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1372]) {s.store_scale(307, 307, (-1.0));}
        if (!s.b[1366]) {s.store_offset_scaled_sub(1357, 513, 158, 1.0 / (s.v[1359]), ((((s.v[123]) + (p[225]))) * (1.0 / (s.v[1359]))));s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1373] = (s.v[1357] < 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1373]) {s.store_scalar(1357, 0.0);}
        if (!s.b[1366]) {s.store_offset(1357, 1357, 1e-50);s.store_div_from_scalar(1348, (-p[224]), 1357);}
        s.b[1374] = (s.v[1348] < (-34.0));s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1374]) {s.store_scalar(308, 0.0);}
        if ((!s.b[1366]) && (!s.b[1374])) {s.store_exp(1349, 1348);s.store_scale(1350, 1362, (p[223] * s.v[1361]));s.store_mul_product3_indices(308, 1349, 1350, 1357, 1357, 1.0);}
        if (!s.b[1366]) {s.store_scalar(310, 0.5);}
        s.b[1382] = (p[28] == 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if s.b[1382] {s.store_scalar(311, 0.0);}
        if (!s.b[1382]) {s.store_add_scaled_inputs4_offset_indices(1375, 157, p[209], 158, (-1.0), 187, p[211], 319, p[211], (p[210] * p[209]));s.store_scalar(1376, (1.0 / s.v[88]));s.store_mul(1377, 1375, 1376);s.store_sqrt_square_offset(44, 1377, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(304, 1377, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1383] = (s.v[304] < 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1383]) {s.store_scalar(304, 0.0);}
        if (!s.b[1382]) {s.store_div_from_scalar_offset_input(1378, 1.0, 304, 1e-50);s.store_scaled_mul(1379, 303, 1378, (-p[208]));}
        s.b[1384] = (s.v[1379] < (-34.0));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1384]) {s.store_scalar(311, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1382]) && (!s.b[1384])) {s.store_exp(1375, 1379);s.store_mul_scale_offset_mixed_ia(1376, 107, A::div_from_scalar(p[207], s.ad_value(302)), 1.6021918e-19, 0.0);s.store_mul_product3_indices(311, 1375, 1376, 304, 304, 1.0);}
        if (!s.b[1382]) {s.store_sub(1381, 157, 513);}
        s.b[1385] = (s.v[1381] > 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1385]) {s.store_square(1376, 1381);s.store_mul(331, 1376, 1381);s.store_offset(1379, 331, p[212]);s.store_div(1380, 331, 1379);s.store_mul(311, 311, 1380);}
        if ((!s.b[1382]) && (!s.b[1385])) {s.store_scalar(311, 0.0);}
        s.b[1393] = (p[28] == 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if s.b[1393] {s.store_scalar(312, 0.0);}
        if (!s.b[1393]) {s.store_add_scaled_inputs3_mixed_aii(1386, A::add_scaled_inputs3_offset(s.ad_value(157), (-p[209]), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p[210]) * (p[209]))), 1.0, 187, p[211], 319, p[211]);s.store_scalar(1387, (1.0 / s.v[88]));s.store_mul(1388, 1386, 1387);s.store_sqrt_square_offset(44, 1388, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(305, 1388, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1394] = (s.v[305] < 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1394]) {s.store_scalar(305, 0.0);}
        if (!s.b[1393]) {s.store_div_from_scalar_offset_input(1389, 1.0, 305, 1e-50);s.store_scaled_mul(1390, 303, 1389, (-p[208]));}
        s.b[1395] = (s.v[1390] < (-34.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1395]) {s.store_scalar(312, 0.0);}
        if ((!s.b[1393]) && (!s.b[1395])) {s.store_exp(1386, 1390);s.store_div_from_scalar(1389, 1.0, 302);s.store_scaled_mul(1387, 1389, 107, (p[207] * 1.6021918e-19));s.store_mul_product3_indices(312, 1386, 1387, 305, 305, 1.0);}
        if (!s.b[1393]) {s.store_neg(1392, 513);}
        s.b[1396] = (s.v[1392] > 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1396]) {s.store_square(1387, 1392);s.store_mul(331, 1387, 1392);s.store_offset(1390, 331, p[212]);s.store_div(1391, 331, 1390);s.store_mul(312, 312, 1391);}
        if ((!s.b[1393]) && (!s.b[1396])) {s.store_scalar(312, 0.0);}
        s.b[1397] = (p[43] == 1.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_scalar(1407, s.v[91]);s.store_primal_div_from_scalar(1408, 1.0, 1407);s.store_scalar(1464, 0.0);s.store_scalar(1466, 0.0);s.store_scalar(1468, 0.0);s.store_neg(1400, 534);s.store_mul(1401, 1400, 436);s.store_add_scaled_product_indices(331, 1401, 1.0, 1400, 437, 1.0);s.store_mul(470, 1401, 438);s.store_sub(469, 1401, 470);s.store_mul(468, 331, 438);s.store_sub(467, 331, 468);}
        if (s.b[1397] && (p[24] != 0.0)) {s.copy_ad(521, 536);}
        let (t40,) = {
    if (s.b[1397] && (p[24] != 0.0)) {
        (0.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t40);s.b[1477] = (1.0 == 1.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });s.b[1478] = (1.0 == 2.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1477]) {s.store_primal_scale(522, 533, 0.5);s.store_scalar(523, p[292]);}
        let (t41,) = {
    if ((s.b[1397] && (p[24] != 0.0)) && s.b[1477]) {
        (s.v[525],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t41);
        if ((s.b[1397] && (p[24] != 0.0)) && (s.b[1478] && (!s.b[1477]))) {s.store_primal_scale(522, 534, 0.5);s.store_scalar(523, p[68]);}
        let (t42,) = {
    if ((s.b[1397] && (p[24] != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
        (s.v[524],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t42);
        let (t43,) = {
    if ((s.b[1397] && (p[24] != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
        (1.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t43);s.b[1479] = (s.v[528] == 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_mul_sqrt_mixed_ia(1427, 238, A::div(s.ad_value(521), s.ad_value(536)));s.store_scalar(1409, ((1.0 - -1.0) / 2.0));s.store_scalar(1410, ((1.0 + -1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_add_scaled_products_mixed_iiia(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1424, 1421, 1420);s.store_neg(1425, 1420);s.store_primal_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);s.store_primal_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1398, 1418);}
        s.b[1480] = (s.v[1398] > s.v[141]);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1480]) {s.store_sub(1399, 1398, 141);s.store_sub(1400, 140, 141);s.store_div(44, 1399, 1400);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1406, 1400, 1406, -1.0, 1.0);s.store_add(1403, 141, 1406);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1480])) {s.copy_ad(1403, 1398);}
        if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));s.store_mul(1428, 1427, 1408);s.store_square(1429, 1428);s.store_sub(1430, 1426, 523);s.store_div(1398, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1431, 2.0, 225, A::ln(s.ad_value(1398)));}
        let (t45,) = {
    if ((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) {
        let t44: f64 = (-s.v[1419]);
        (t44,)
    } else {
        (s.v[1432],)
    }
};
        s.store_scalar(1432, t45);s.b[1481] = (s.v[1430] < s.v[1432]);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_div_scalar_by_product_indices(1399, 1.0, 225, 1427, 1.0);s.store_mul(1406, 1399, 1407);s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);s.store_sub(1435, 237, 1431);s.store_mul_add_rhs(1405, 225, 1430, 1419);s.store_sub_from_scalar_scaled_mul_mixed_ia(1436, (7.0 * 1.414213562373095), 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);s.store_square(1437, 1436);}
        s.b[1482] = (s.v[1434] < (s.v[1437] * 1e-8));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) && s.b[1482]) {s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);}
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) && (!s.b[1482])) {s.store_sqrt_add(1438, 1434, 1437);s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_powf(1440, 1439, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);s.store_div(1442, 1441, 1440);s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);s.store_add(1399, 1443, 1419);s.store_div(1400, 1399, 1435);s.store_sqrt_square_offset(1401, 1400, 1.0);s.store_sub_div_lhs_indices(1444, 1399, 1401, 1419);s.store_sub(1400, 1430, 1444);s.store_mul(459, 1407, 1400);s.copy_ad(458, 459);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_scalar(1442, 3.0);s.store_sub_div_lhs_indices(1445, 1442, 225, 1419);s.store_exp_neg_input(1406, 1442);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1483] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1483]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1484] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1484]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        s.b[1485] = (s.v[1442] < 3.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1449, 1430, -1.0, 1419, -1.0, 1428, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), 1449, 1.0, 1446, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), 1.0, 1446, 1446, 9.0);s.store_sqrt_add_scaled_square_cube_product(1402, 1450, 1.0, 1451, 1.0);s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);s.store_neg_powf_add_input(1453, 1450, 1402, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1405, 1452, 1.0, 1453, 1.0, 1447, 1.0, 1446, 3.0, -1.0);s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_add(1454, 1430, 1419, 0.1);s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1456, 1455, 1461);s.store_mul(1398, 226, 1429);s.store_mul(1457, 225, 1454);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.store_offset_sub(44, 1457, 1458, (-1.0));s.store_scale(45, 1457, 4.0);}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1458, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1457, 1457, 1458);s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.copy_ad(1460, 1442);s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1442, 1459, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1444, 1442, 225, 1419);s.store_add_offset_lhs_mixed_ia(1399, 1442, (-1.0), A::exp_scaled_input(s.ad_value(1442), -1.0));}
        s.b[1486] = (s.v[1399] < (10.0 * 2.220446049250313e-16));s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1486]) {s.store_scalar(1399, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt(1400, 1399);s.store_mul(458, 1427, 1400);s.store_mul_sub_rhs(459, 1407, 1430, 1444);}
        s.b[1487] = (p[42] == 1.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1470, 1455, 1461);}
        let (t46,) = {
    if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
        (0.0,)
    } else {
        (s.v[1415],)
    }
};
        s.store_scalar(1415, t46);
        if ((((s.b[1397] && (p[24] != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_scalar(167, 1.0);}
    }
}
