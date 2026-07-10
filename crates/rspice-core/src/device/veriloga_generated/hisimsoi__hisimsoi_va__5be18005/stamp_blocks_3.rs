#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t3: f64 = (s.v[57] + 1.0);let t4: f64 = if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (s.v[167] <= t3)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
            let (t1,) = {
    if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1279]) {
        let t0: f64 = (-1.0);
        (t0,)
    } else {
        (s.v[338],)
    }
};
            s.store_scalar(338, t1);s.b[1280] = (s.v[181] < 0.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1280]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1281] = (s.v[181] < 1e-7);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && s.b[1281]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && (!s.b[1281])) {s.store_mul_scale_offset_indices(501, 225, 1272, 1.0, (-p.p287));s.store_exp(502, 501);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
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
            let (t2,) = {
    if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1284]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t2);
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
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
        let (t6,) = {
    if (s.b[1224] && s.b[1289]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6);
        let (t7,) = {
    if (s.b[1224] && s.b[1289]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t7);
        if (s.b[1224] && s.b[1289]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1290] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });s.b[1291] = (1.0 == 1.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        let (t8,) = {
    if (((s.b[1224] && s.b[1289]) && s.b[1290]) && s.b[1291]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t8);s.b[1292] = (1.0 == 2.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && s.b[1292]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t9);s.b[1293] = (1.0 == 4.0);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        let (ta,) = {
    if (((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, ta);s.b[1294] = (1.0 == 8.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, tb);
        let (tc,) = {
    if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, tc);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t10: usize = 0;
        while {
            let tf: f64 = if (((s.b[1224] && s.b[1289]) && s.b[1290]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1224] && s.b[1289]) && s.b[1290]) {s.store_sqrt(53, 53);}
            let (te,) = {
    if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
        let td: f64 = (s.v[54] + 1.0);
        (td,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, te);
        }
        if ((s.b[1224] && s.b[1289]) && (!s.b[1290])) {s.store_powf(53, 53, (1.0 / 2.0));}
        if (s.b[1224] && s.b[1289]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);}
        if (s.b[1224] && (!s.b[1289])) {s.copy_ad(328, 1233);}
        if s.b[1224] {s.store_sqrt_offset_input(1243, 328, (10.0 * 2.220446049250313e-16));s.store_mul(1244, 1242, 1243);s.store_mul_div_scaled_inputs_indices(1240, 1244, 227, 2.0, 1231, 1.0);s.store_mul_product3_indices(204, 337, 1240, 1239, 107, 1.0);s.store_add(199, 202, 204);}
        s.store_add(201, 203, 204);s.b[1295] = ((p.p43 == 1.0) || (p.p45 == 1.0));s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });s.b[1308] = ((s.v[145] == 1.0) || (p.p25 == 0.0));s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (s.b[1295] && s.b[1308]) {s.store_scalar(263, 0.0);}
        s.b[1309] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1295] && (!s.b[1308])) && s.b[1309]) {s.store_scalar(263, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p.p48);}
        s.b[1310] = (p.p44 <= 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.copy_ad(1296, 445);s.store_square(1303, 323);s.copy_ad(1304, 545);s.store_div(1298, 1304, 1303);s.store_div_from_scalar(1305, 2.0, 1304);s.store_mul(1299, 1305, 1303);s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1302, 1299, 1300, 1.0);s.store_sqrt_square_offset(44, 1302, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1301, 1302, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1311] = (s.v[1301] < 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1311]) {s.store_scalar(1301, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {s.store_offset(1301, 1301, 1e-50);s.store_sqrt(1301, 1301);s.store_add_scaled_product_mixed_aii(1306, A::mul_sub_from_scalar_rhs(s.ad_value(1298), 1.0, s.ad_value(1301)), 1.0, 1296, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(1307, 173, p.p122, 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1306)), -1.0);s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1312] = (s.v[1307] < 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1312]) {s.store_scalar(1307, 0.0);}
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_mul(1296, 134, 445);s.store_div_square_rhs(1298, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1299, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(1300, 1296, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1300, 1300, 1.0, 130, 483, (-1.0));s.store_offset_mul(1301, 1299, 1300, 1.0);s.store_scaled_offset(1303, 1299, 1.0, 2.0);}
        s.b[1313] = ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0));s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_sub_offset_lhs(44, 1303, 1e-50, 1301);s.store_square(49, 44);s.store_square(50, 1303);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t11,) = {
    if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t11);
        let (t12,) = {
    if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t12);
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });s.b[1315] = (4.0 == 1.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        let (t13,) = {
    if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && s.b[1315]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t13);s.b[1316] = (4.0 == 2.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        let (t14,) = {
    if (((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && s.b[1316]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t14);s.b[1317] = (4.0 == 4.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        let (t15,) = {
    if ((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t15);s.b[1318] = (4.0 == 8.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        let (t16,) = {
    if (((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t16);
        let (t17,) = {
    if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t17);let mut t1b: usize = 0;
        while {
            let t1a: f64 = if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {s.store_sqrt(53, 53);}
            let (t19,) = {
    if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
        let t18: f64 = (s.v[54] + 1.0);
        (t18,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t19);
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
        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {s.store_add_mul_sub_from_scalar_rhs_indices(1306, 1296, 1298, 1.0, 1301);s.store_div_from_scalar_offset_input(1297, s.v[100], 131, s.v[100]);s.store_add_scaled_inputs_product_indices(1307, 173, p.p122, 176, 1.0, 1297, 1306, (-1.0));s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1307, 1307, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1319] = (s.v[1307] < 0.0);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1319]) {s.store_scalar(1307, 0.0);}
        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {s.store_offset(1307, 1307, 1e-50);s.store_ad_value(1297, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1307), 1.0));s.store_mul_product3_indices(263, 1297, 132, 1307, 199, 1.0);}
        s.b[1320] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if s.b[1320] {s.store_mul_scaled_exp_scaled_input_rhs(1321, 107, (1.6021918e-19 * p.p237), 225, (-p.p141));s.store_offset_scaled(1322, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(1323, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 1321, 1322, 1.0);s.store_mul(567, 263, 1323);s.store_mul_scaled_ln_offset_rhs(1324, 227, p.p140, 567, 1.0);s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1324, (-1.0), 231, (-0.01));s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));}
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
        if s.b[1320] {s.store_sqrt_ad(1327, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(176))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(176), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(1328, 1325, 1326, -1.0, 1327, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1320] {s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1328), (-(p.p47 * 0.01)));s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));}
        if s.b[1320] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1320] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p.p47);}
        if s.b[1320] {s.store_scalar(1321, (if (p.p138 > 0.0) { p.p138 } else { 1.0 }));}
        if s.b[1320] {s.store_div_scaled_value_offset_denominator(398, s.ad_value(1321), 1.0, s.ad_value(263), p.p139, 1.0);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        s.b[1342] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });s.b[1343] = (s.v[56] < 3.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1343]) {s.store_scalar(516, 0.0);s.store_scalar(517, 0.0);}
        if (s.b[1342] && (!s.b[1343])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }
        if (s.b[1342] && (!s.b[1343])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }
        if s.b[1342] {s.store_offset_scaled(1329, 185, p.p147, 1.0);s.store_scaled_mul(1330, 1329, 263, p.p146);s.store_offset_mul_ad(1331, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));s.store_sqrt_square_offset(44, 1331, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1331, 1331, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1344] = (s.v[1331] < 0.0);s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1344]) {s.store_scalar(1331, 0.0);}
        if s.b[1342] {s.store_sqrt(1332, 1331);s.store_mul(1333, 1331, 1332);s.store_offset_mul_ad(1334, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));s.store_sqrt_square_offset(44, 1334, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1334, 1334, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1345] = (s.v[1334] < 0.0);s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });
        if (s.b[1342] && s.b[1345]) {s.store_scalar(1334, 0.0);}
        if s.b[1342] {s.store_sqrt(1335, 1334);s.store_mul(1336, 1334, 1335);s.store_div_from_scalar(1337, 1.0, 1331);s.store_mul3_lhs(328, 225, 1330, 1337);s.store_div_from_scalar(1337, 1.0, 1334);s.store_mul3_lhs(1338, 225, 1330, 1337);s.store_mul_mixed_ia(1339, 238, A::add_scaled_products(s.ad_value(1336), s.ad_value(1338), 1.0, s.ad_value(1333), s.ad_value(328), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(1340, 238, 1335, 1338, ((-1.0) * (0.5)), 1332, 328, 0.5);s.store_add(1341, 1339, 1340);s.store_mul3_lhs(265, 264, 1341, 250);}
        s.store_scalar(1359, (s.v[88] * 100.0));s.store_scale(1360, 323, 0.0001);s.store_scalar(1361, (s.v[97] * 100.0));s.store_primal_scale(1362, 107, 100.0);s.store_scale(1363, 252, 0.01);s.store_scale(1364, 436, 0.0001);s.store_scale(1365, 238, 0.0001);s.b[1366] = (p.p27 == 0.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
        if s.b[1366] {s.store_scalar(309, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(308, 0.0);s.store_scalar(310, 0.0);}
        s.b[1367] = (s.v[145] == 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset_add(1358, 176, 173, (-(10.0 * 2.220446049250313e-16)));s.store_add_scaled_inputs4_offset_indices(1348, 174, 1.0, 185, (p.p216 * s.v[1361]), 320, (-(p.p216 * s.v[1361])), 1358, (-p.p215), (-s.v[123]));s.store_scalar(1350, (1.0 / s.v[1359]));s.store_mul(1349, 1348, 1350);s.store_scalar(1350, (1.0 / p.p217));s.store_offset_mul(1354, 1363, 1350, 1.0);s.store_mul(1357, 1349, 1354);s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1368] = (s.v[1357] < 0.0);s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1366]) && s.b[1367]) && s.b[1368]) {s.store_scalar(1357, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1350, 174, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1369] = (s.v[1350] < 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1369]) {s.store_scalar(1350, 0.0);}
        if ((!s.b[1366]) && s.b[1367]) {s.store_offset(1350, 1350, (-p.p226));s.store_scale(1346, 1350, 10.0);s.store_offset_square(1349, 1346, 1.0);s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));s.store_mul(1357, 1357, 1348);s.store_scale(1347, 1362, s.v[1361]);s.store_div_from_scalar_offset_input(1354, p.p219, 1347, p.p219);s.store_scalar(1353, p.p218);s.store_div_add_scaled_inputs_rhs_indices(1355, 1353, 1353, 1.0, 173, 1.0);s.store_div_from_scalar_offset_input(1351, 1.0, 1357, 1e-50);s.store_scaled_mul(1348, 303, 1351, (-p.p214));}
        s.b[1370] = (s.v[1348] < (-34.0));s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if (((!s.b[1366]) && s.b[1367]) && s.b[1370]) {s.store_scalar(309, 0.0);}
        if (((!s.b[1366]) && s.b[1367]) && (!s.b[1370])) {s.store_exp(1349, 1348);s.store_mul_scale_offset_mixed_ia(1350, 1347, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 0.0);s.store_div_from_scalar(1352, 1.0, 1365);s.store_sqrt_mul_ad(1353, A::add_scaled_inputs(s.ad_value(1364), 1.0, s.ad_value(1360), 1e-12), s.ad_value(1352));s.store_mul3_lhs(1351, 1349, 1350, 1353);s.store_mul3_lhs(1356, 1351, 1357, 1357);s.store_mul3_lhs(309, 1354, 1355, 1356);}
        if ((!s.b[1366]) && (!s.b[1367])) {s.store_scalar(309, 0.0);}
        if (!s.b[1366]) {s.store_offset_scaled(1347, 158, (-p.p221), p.p222);s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 158, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 158, 1347);s.store_scale(1351, 1362, (p.p220 / 1000000.0));s.store_mul3_lhs(306, 1351, 1349, 1350);}
        s.b[1371] = (s.v[158] >= 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1371]) {s.store_scale(306, 306, (-1.0));}
        if (!s.b[1366]) {s.store_sub(1348, 158, 157);s.store_offset_scaled(1347, 1348, (-p.p221), p.p222);s.store_exp_scaled_input(1349, 1347, s.v[1359]);s.store_scale(1347, 1348, (1.0 / (s.v[1359]) * 1.0 / (s.v[1359])));s.store_mul(1350, 1348, 1347);s.store_scale(1351, 1362, (p.p220 / 1000000.0));s.store_mul3_lhs(307, 1351, 1349, 1350);}
        s.b[1372] = (s.v[1348] >= 0.0);s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1372]) {s.store_scale(307, 307, (-1.0));}
        if (!s.b[1366]) {s.store_offset_scaled_sub(1357, 513, 158, 1.0 / (s.v[1359]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1359]))));s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1357, 1357, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1373] = (s.v[1357] < 0.0);s.store_scalar(1373, if s.b[1373] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1373]) {s.store_scalar(1357, 0.0);}
        if (!s.b[1366]) {s.store_offset(1357, 1357, 1e-50);s.store_div_from_scalar(1348, (-p.p224), 1357);}
        s.b[1374] = (s.v[1348] < (-34.0));s.store_scalar(1374, if s.b[1374] { 1.0 } else { 0.0 });
        if ((!s.b[1366]) && s.b[1374]) {s.store_scalar(308, 0.0);}
        if ((!s.b[1366]) && (!s.b[1374])) {s.store_exp(1349, 1348);s.store_scale(1350, 1362, (p.p223 * s.v[1361]));s.store_mul_product3_indices(308, 1349, 1350, 1357, 1357, 1.0);}
        if (!s.b[1366]) {s.store_scalar(310, 0.5);}
        s.b[1382] = (p.p28 == 0.0);s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if s.b[1382] {s.store_scalar(311, 0.0);}
        if (!s.b[1382]) {s.store_add_scaled_inputs4_offset_indices(1375, 157, p.p209, 158, (-1.0), 187, p.p211, 319, p.p211, (p.p210 * p.p209));s.store_scalar(1376, (1.0 / s.v[88]));s.store_mul(1377, 1375, 1376);s.store_sqrt_square_offset(44, 1377, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(304, 1377, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1383] = (s.v[304] < 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1383]) {s.store_scalar(304, 0.0);}
        if (!s.b[1382]) {s.store_div_from_scalar_offset_input(1378, 1.0, 304, 1e-50);s.store_scaled_mul(1379, 303, 1378, (-p.p208));}
        s.b[1384] = (s.v[1379] < (-34.0));s.store_scalar(1384, if s.b[1384] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1384]) {s.store_scalar(311, 0.0);}
        if ((!s.b[1382]) && (!s.b[1384])) {s.store_exp(1375, 1379);s.store_mul_scale_offset_mixed_ia(1376, 107, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 0.0);s.store_mul_product3_indices(311, 1375, 1376, 304, 304, 1.0);}
        if (!s.b[1382]) {s.store_sub(1381, 157, 513);}
        s.b[1385] = (s.v[1381] > 0.0);s.store_scalar(1385, if s.b[1385] { 1.0 } else { 0.0 });
        if ((!s.b[1382]) && s.b[1385]) {s.store_square(1376, 1381);s.store_mul(331, 1376, 1381);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1382]) && s.b[1385]) {s.store_offset(1379, 331, p.p212);s.store_div(1380, 331, 1379);s.store_mul(311, 311, 1380);}
        if ((!s.b[1382]) && (!s.b[1385])) {s.store_scalar(311, 0.0);}
        s.b[1393] = (p.p28 == 0.0);s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if s.b[1393] {s.store_scalar(312, 0.0);}
        if (!s.b[1393]) {s.store_add_scaled_inputs3_mixed_aii(1386, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, 187, p.p211, 319, p.p211);s.store_scalar(1387, (1.0 / s.v[88]));s.store_mul(1388, 1386, 1387);s.store_sqrt_square_offset(44, 1388, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(305, 1388, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1394] = (s.v[305] < 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1394]) {s.store_scalar(305, 0.0);}
        if (!s.b[1393]) {s.store_div_from_scalar_offset_input(1389, 1.0, 305, 1e-50);s.store_scaled_mul(1390, 303, 1389, (-p.p208));}
        s.b[1395] = (s.v[1390] < (-34.0));s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1395]) {s.store_scalar(312, 0.0);}
        if ((!s.b[1393]) && (!s.b[1395])) {s.store_exp(1386, 1390);s.store_div_from_scalar(1389, 1.0, 302);s.store_scaled_mul(1387, 1389, 107, (p.p207 * 1.6021918e-19));s.store_mul_product3_indices(312, 1386, 1387, 305, 305, 1.0);}
        if (!s.b[1393]) {s.store_neg(1392, 513);}
        s.b[1396] = (s.v[1392] > 0.0);s.store_scalar(1396, if s.b[1396] { 1.0 } else { 0.0 });
        if ((!s.b[1393]) && s.b[1396]) {s.store_square(1387, 1392);s.store_mul(331, 1387, 1392);s.store_offset(1390, 331, p.p212);s.store_div(1391, 331, 1390);s.store_mul(312, 312, 1391);}
        if ((!s.b[1393]) && (!s.b[1396])) {s.store_scalar(312, 0.0);}
        s.b[1397] = (p.p43 == 1.0);s.store_scalar(1397, if s.b[1397] { 1.0 } else { 0.0 });
        if s.b[1397] {s.store_scalar(1407, s.v[91]);s.store_primal_div_from_scalar(1408, 1.0, 1407);s.store_scalar(1464, 0.0);s.store_scalar(1466, 0.0);s.store_scalar(1468, 0.0);s.store_neg(1400, 534);s.store_mul(1401, 1400, 436);s.store_add_scaled_product_indices(331, 1401, 1.0, 1400, 437, 1.0);s.store_mul(470, 1401, 438);s.store_sub(469, 1401, 470);s.store_mul(468, 331, 438);s.store_sub(467, 331, 468);}
        if (s.b[1397] && (p.p24 != 0.0)) {s.copy_ad(521, 536);}
        let (t1c,) = {
    if (s.b[1397] && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1c);s.b[1477] = (1.0 == 1.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });s.b[1478] = (1.0 == 2.0);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1477]) {s.store_primal_scale(522, 533, 0.5);s.store_scalar(523, p.p292);}
        let (t1d,) = {
    if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1477]) {
        (s.v[525],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1d);
        if ((s.b[1397] && (p.p24 != 0.0)) && (s.b[1478] && (!s.b[1477]))) {s.store_primal_scale(522, 534, 0.5);s.store_scalar(523, p.p68);}
        let (t1e,) = {
    if ((s.b[1397] && (p.p24 != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
        (s.v[524],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1e);
        let (t1f,) = {
    if ((s.b[1397] && (p.p24 != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
        (1.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1f);s.b[1479] = (s.v[528] == 0.0);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_mul_sqrt_mixed_ia(1427, 238, A::div(s.ad_value(521), s.ad_value(536)));s.store_scalar(1409, ((1.0 - -1.0) / 2.0));s.store_scalar(1410, ((1.0 + -1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1424, 1421, 1420);s.store_neg(1425, 1420);s.store_primal_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);s.store_primal_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1398, 1418);}
        s.b[1480] = (s.v[1398] > s.v[141]);s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1480]) {s.store_sub(1399, 1398, 141);s.store_sub(1400, 140, 141);s.store_div(44, 1399, 1400);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1406, 1400, 1406, -1.0, 1.0);s.store_add(1403, 141, 1406);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1480])) {s.copy_ad(1403, 1398);}
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));s.store_mul(1428, 1427, 1408);s.store_square(1429, 1428);s.store_sub(1430, 1426, 523);s.store_div(1398, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1431, 2.0, 225, A::ln(s.ad_value(1398)));}
        let (t21,) = {
    if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
        let t20: f64 = (-s.v[1419]);
        (t20,)
    } else {
        (s.v[1432],)
    }
};
        s.store_scalar(1432, t21);s.b[1481] = (s.v[1430] < s.v[1432]);s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_div_scalar_by_product_indices(1399, 1.0, 225, 1427, 1.0);s.store_mul(1406, 1399, 1407);s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);s.store_sub(1435, 237, 1431);s.store_mul_add_rhs(1405, 225, 1430, 1419);s.store_sub_from_scalar_scaled_mul_mixed_ia(1436, (7.0 * 1.414213562373095), 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);s.store_square(1437, 1436);}
        s.b[1482] = (s.v[1434] < (s.v[1437] * 1e-8));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && s.b[1482]) {s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && (!s.b[1482])) {s.store_sqrt_add(1438, 1434, 1437);s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {s.store_powf(1440, 1439, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);s.store_div(1442, 1441, 1440);s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);s.store_add(1399, 1443, 1419);s.store_div(1400, 1399, 1435);s.store_sqrt_square_offset(1401, 1400, 1.0);s.store_sub_div_lhs_indices(1444, 1399, 1401, 1419);s.store_sub(1400, 1430, 1444);s.store_mul(459, 1407, 1400);s.copy_ad(458, 459);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_scalar(1442, 3.0);s.store_sub_div_lhs_indices(1445, 1442, 225, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1483] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1483]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1484] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1484]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        s.b[1485] = (s.v[1442] < 3.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1449, 1430, -1.0, 1419, -1.0, 1428, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), 1449, 1.0, 1446, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), 1.0, 1446, 1446, 9.0);s.store_sqrt_add_scaled_square_cube_product(1402, 1450, 1.0, 1451, 1.0);s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);s.store_neg_powf_add_input(1453, 1450, 1402, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1405, 1452, 1.0, 1453, 1.0, 1447, 1.0, 1446, 3.0, -1.0);s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_add(1454, 1430, 1419, 0.1);s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1456, 1455, 1461);s.store_mul(1398, 226, 1429);s.store_mul(1457, 225, 1454);s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.store_offset_sub(44, 1457, 1458, (-1.0));s.store_scale(45, 1457, 4.0);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1458, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1457, 1457, 1458);s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.copy_ad(1460, 1442);s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1442, 1459, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1444, 1442, 225, 1419);s.store_add_offset_lhs_mixed_ia(1399, 1442, (-1.0), A::exp_scaled_input(s.ad_value(1442), -1.0));}
        s.b[1486] = (s.v[1399] < (10.0 * 2.220446049250313e-16));s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1486]) {s.store_scalar(1399, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {s.store_sqrt(1400, 1399);s.store_mul(458, 1427, 1400);s.store_mul_sub_rhs(459, 1407, 1430, 1444);}
        s.b[1487] = (p.p42 == 1.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1470, 1455, 1461);}
        let (t22,) = {
    if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
        (0.0,)
    } else {
        (s.v[1415],)
    }
};
        s.store_scalar(1415, t22);
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t27: usize = 0;
        while {
            let t24: f64 = (2.0 * 20.0);let t25: f64 = (t24 + 1.0);let t26: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (s.v[167] <= t25)) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_scalar(1466, 0.0);s.store_mul_add_rhs(1442, 225, 1444, 1419);}
            s.b[1488] = (s.v[1442] < 5.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1488]) {s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1464, 1470, 1462, 1462);s.store_mul_product3_indices(1465, 1463, 1470, 225, 1462, 2.0);s.store_mul_scale_offset_mixed_ia(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, 1465, 1.0, 1468, 2.0);}
            s.b[1489] = (s.v[1442] < 80.0);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {s.store_exp(243, 1442);s.store_mul_scale_offset_indices(1464, 1470, 243, 1.0, (-1.0));s.store_mul3_lhs(1465, 1470, 225, 243);}
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {s.store_exp_mul(1471, 225, 1444);s.store_mul_sub_rhs(1464, 1455, 1471, 1461);s.store_mul3_lhs(1465, 1455, 225, 1471);}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) {s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);}
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));s.store_sub_from_scalar_scaled_mul(1473, (-1.0), 1428, 1469, 1.0);}
            s.b[1490] = (s.v[1415] == 1.0);s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1490]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {s.store_div_scaled_inputs_indices(494, 1472, -1.0, 1473, 1.0);}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1491] = (((s.v[494]) as f64).abs() > s.v[1474]);s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {s.store_add(1444, 1444, 494);}
            s.b[1492] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
            let (t23,) = {
    if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1492]) {
        (1.0,)
    } else {
        (s.v[1415],)
    }
};
            s.store_scalar(1415, t23);
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1494] = (s.v[1442] < 5.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1494]) {s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1494])) {s.store_offset(1475, 1442, (-1.0));s.store_sqrt(1476, 1475);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_mul(458, 1427, 1476);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));s.store_mul3_lhs(460, 1427, 1464, 1399);s.store_add(459, 458, 460);}
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_sub(460, 459, 458);}
        s.b[1496] = (1.0 == 1.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });s.b[1497] = (1.0 == 2.0);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_scalar(1409, ((1.0 - 1.0) / 2.0));s.store_scalar(1410, ((1.0 + 1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1420, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1421, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1422, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1423, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1424, 1421, 1420);s.store_neg(1425, 1420);s.store_primal_add_scaled_products_indices(1411, 1409, 461, 1.0, 1410, 462, 1.0);s.store_primal_add_scaled_products_indices(1412, 1409, 462, 1.0, 1410, 461, 1.0);s.store_add_scaled_products_indices(1426, 1411, 1422, 1.0, 1412, 1423, 1.0);s.store_offset_ad(1418, A::add_scaled_products(s.ad_value(1411), s.ad_value(1425), 1.0, s.ad_value(1412), s.ad_value(1424), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1398, 1418);}
        s.b[1498] = (s.v[1398] > s.v[141]);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1498]) {s.store_sub(1399, 1398, 141);s.store_sub(1400, 140, 141);s.store_div(44, 1399, 1400);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1406, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1406, 1400, 1406, -1.0, 1.0);s.store_add(1403, 141, 1406);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1498])) {s.copy_ad(1403, 1398);}
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));s.store_mul(1428, 1427, 1408);s.store_square(1429, 1428);s.store_sub(1430, 1426, 523);s.store_div(1398, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1431, 2.0, 225, A::ln(s.ad_value(1398)));}
        let (t29,) = {
    if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
        let t28: f64 = (-s.v[1419]);
        (t28,)
    } else {
        (s.v[1432],)
    }
};
        s.store_scalar(1432, t29);s.b[1499] = (s.v[1430] < s.v[1432]);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {s.store_div_scalar_by_product_indices(1399, 1.0, 225, 1427, 1.0);s.store_mul(1406, 1399, 1407);s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1434, 1433, 1433, 8.0, 0.0, 1433);s.store_sub(1435, 237, 1431);s.store_mul_add_rhs(1405, 225, 1430, 1419);s.store_sub_from_scalar_scaled_mul_mixed_ia(1436, (7.0 * 1.414213562373095), 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);s.store_square(1437, 1436);}
        s.b[1500] = (s.v[1434] < (s.v[1437] * 1e-8));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && s.b[1500]) {s.store_add_scaled_inputs_product_mixed_aaia(1439, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1434), 0.5, s.ad_value(1436), 1.0), 1.0, 1406, A::offset(s.ad_value(1405), (-2.0)), 9.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && (!s.b[1500])) {s.store_sqrt_add(1438, 1434, 1437);s.store_add_scaled_offset_product_rhs_mixed_aii(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), 1.0, 1406, 1405, (-2.0), 9.0);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {s.store_powf(1440, 1439, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1441, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), 1.0, 1440, 2.0, 1440, 1440, 1.414213562373095);s.store_div(1442, 1441, 1440);s.store_add_scaled_product_indices(1443, 1419, (-1.0), 1442, 227, 1.0);s.store_add(1399, 1443, 1419);s.store_div(1400, 1399, 1435);s.store_sqrt_square_offset(1401, 1400, 1.0);s.store_sub_div_lhs_indices(1444, 1399, 1401, 1419);s.store_sub(1400, 1430, 1444);s.store_mul(459, 1407, 1400);s.copy_ad(458, 459);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_scalar(1442, 3.0);s.store_sub_div_lhs_indices(1445, 1442, 225, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1501] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1501]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);s.store_exp_neg_input(1406, 1442);s.store_offset_div_scaled_inputs2_mixed_aia(1405, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), 4.0, 1406, 4.0, A::mul(s.ad_value(1429), s.ad_value(226)), 1.0, 1.0);}
        s.b[1502] = (s.v[1405] < (10.0 * 2.220446049250313e-16));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1502]) {s.store_scalar(1405, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_add_product3_rhs_mixed_iia(1445, 1430, 1429, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405))), 1.0 / (2.0));s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        s.b[1503] = (s.v[1442] < 3.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1449, 1430, -1.0, 1419, -1.0, 1428, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1450, A::div_scaled_product(A::square(s.ad_value(1447)), s.ad_value(1447), 1.0, A::mul3_scaled_output(s.ad_value(1446), s.ad_value(1446), s.ad_value(1446), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1447), s.ad_value(1448), 1.0, s.ad_value(1446), s.ad_value(1446), 6.0), (-1.0), 1449, 1.0, 1446, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {s.store_div_scaled_value_by_product_mixed_aii(1451, A::add_scaled_square_product(s.ad_value(1447), (-1.0), s.ad_value(1446), s.ad_value(1448), 3.0), 1.0, 1446, 1446, 9.0);s.store_sqrt_add_scaled_square_cube_product(1402, 1450, 1.0, 1451, 1.0);s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);s.store_neg_powf_add_input(1453, 1450, 1402, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1405, 1452, 1.0, 1453, 1.0, 1447, 1.0, 1446, 3.0, -1.0);s.store_add_scaled_product_indices(1445, 1419, (-1.0), 1405, 227, 1.0);s.store_mul_add_rhs(1442, 225, 1445, 1419);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_offset_add(1454, 1430, 1419, 0.1);s.store_offset_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0), 1e-50);s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1456, 1455, 1461);s.store_mul(1398, 226, 1429);s.store_mul(1457, 225, 1454);s.store_add_scaled_inputs_product_mixed_aaii(1458, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.store_offset_sub(44, 1457, 1458, (-1.0));s.store_scale(45, 1457, 4.0);}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1458, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1457, 1457, 1458);s.store_add_scaled_inputs(1457, 1457, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1459, A::ln(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1398), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1455), s.ad_value(1398))), (-1.0), 225, 1419, 1.0);s.copy_ad(1460, 1442);s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1400, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1442, 1459, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1444, 1442, 225, 1419);s.store_add_offset_lhs_mixed_ia(1399, 1442, (-1.0), A::exp_scaled_input(s.ad_value(1442), -1.0));}
        s.b[1504] = (s.v[1399] < (10.0 * 2.220446049250313e-16));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1504]) {s.store_scalar(1399, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {s.store_sqrt(1400, 1399);s.store_mul(458, 1427, 1400);s.store_mul_sub_rhs(459, 1407, 1430, 1444);}
        s.b[1505] = (p.p42 == 1.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_exp_ad(1461, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1419), -1.0));s.store_div(1398, 230, 521);s.store_square(1455, 1398);s.store_mul(1470, 1455, 1461);}
        let (t2a,) = {
    if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
        (0.0,)
    } else {
        (s.v[1415],)
    }
};
        s.store_scalar(1415, t2a);
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t2f: usize = 0;
        while {
            let t2c: f64 = (2.0 * 20.0);let t2d: f64 = (t2c + 1.0);let t2e: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (s.v[167] <= t2d)) { 1.0 } else { 0.0 };
            t2e != 0.0
        } {
            t2f += 1;assert!(t2f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_scalar(1466, 0.0);s.store_mul_add_rhs(1442, 225, 1444, 1419);}
            s.b[1506] = (s.v[1442] < 5.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1506]) {s.store_mul3_ad_middle(1462, A::square(s.ad_value(1442)), 1442, A::offset(A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1463, A::square(s.ad_value(1442)), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1464, 1470, 1462, 1462);s.store_mul_product3_indices(1465, 1463, 1470, 225, 1462, 2.0);s.store_mul_scale_offset_mixed_ia(1466, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1467, 1442, A::mul_offset_rhs(s.ad_value(1442), A::mul(s.ad_value(1442), A::scale_offset(s.ad_value(1442), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1469, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1467), s.ad_value(1466), 2.0), 1.0, 1465, 1.0, 1468, 2.0);}
            s.b[1507] = (s.v[1442] < 80.0);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && s.b[1507]) {s.store_exp(243, 1442);s.store_mul_scale_offset_indices(1464, 1470, 243, 1.0, (-1.0));s.store_mul3_lhs(1465, 1470, 225, 243);}
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && (!s.b[1507])) {s.store_exp_mul(1471, 225, 1444);s.store_mul_sub_rhs(1464, 1455, 1471, 1461);s.store_mul3_lhs(1465, 1455, 225, 1471);}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) {s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));s.store_scale_ad(1469, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1465), 1.0, s.ad_value(1468), 1.0), 0.5);}
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_add_scaled_inputs_product_indices(1472, 1430, 1.0, 1444, (-1.0), 1428, 1468, (-1.0));s.store_sub_from_scalar_scaled_mul(1473, (-1.0), 1428, 1469, 1.0);}
            s.b[1508] = (s.v[1415] == 1.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1508]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {s.store_div_scaled_inputs_indices(494, 1472, -1.0, 1473, 1.0);}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1509] = (((s.v[494]) as f64).abs() > s.v[1474]);s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1509]) {s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {s.store_add(1444, 1444, 494);}
            s.b[1510] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
            let (t2b,) = {
    if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1510]) {
        (1.0,)
    } else {
        (s.v[1415],)
    }
};
            s.store_scalar(1415, t2b);
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1512] = (s.v[1442] < 5.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1512]) {s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1512])) {s.store_offset(1475, 1442, (-1.0));s.store_sqrt(1476, 1475);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_mul(458, 1427, 1476);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));s.store_mul3_lhs(460, 1427, 1464, 1399);s.store_add(459, 458, 460);}
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {s.store_sub(460, 459, 458);}
        s.b[1514] = (1.0 == 1.0);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });s.b[1515] = (1.0 == 2.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1409] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1410] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p.p189);s.b[1518] = (s.v[145] != 0.0);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if s.b[1518] {s.store_add(1517, 157, 161);s.store_add_scaled_inputs(314, 1517, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1519] = (p.p64 != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });
        if (s.b[1518] && s.b[1519]) {s.store_scalar(315, 0.0);}
        s.b[1520] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (s.b[1518] && s.b[1520]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1521] = (p.p64 != 0.0);s.store_scalar(1521, if s.b[1521] { 1.0 } else { 0.0 });s.b[1522] = (s.v[246] < 1e-15);s.store_scalar(1522, if s.b[1522] { 1.0 } else { 0.0 });
        if (((!s.b[1518]) && s.b[1521]) && s.b[1522]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1518]) && s.b[1521]) && (!s.b[1522])) {s.store_scale(1516, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1517, 1.0, 244);s.store_mul3_lhs(315, 246, 1516, 1517);}
        s.store_scalar(1534, s.v[91]);s.store_scalar(1535, (1.0 / s.v[1534]));s.store_scalar(1555, 0.0);s.store_scalar(1595, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1597, 0.0);s.b[1606] = ((p.p29 >= 1.0) && (p.p188 > 0.0));s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((p.p24 != 0.0) && s.b[1606]) {s.store_scalar(1537, p.p171);s.store_scalar(1538, p.p172);s.copy_ad(1539, 158);s.store_scalar(1536, p.p188);}
        s.b[1607] = ((s.v[69] == 0.0) && (p.p188 > 0.0));s.store_scalar(1607, if s.b[1607] { 1.0 } else { 0.0 });
        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {
            if (p.p43 == 1.0) {
                s.store_scale(1524, 287, s.v[1534]);
            } else {
                s.store_scale(1524, 108, s.v[1534]);
            }
        }
        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {s.store_mul_ad_product_rhs_mixed_ia(1527, 1537, 1524, A::add(s.ad_value(1538), s.ad_value(1539)));s.store_mul(1528, 1536, 1524);s.copy_ad(1532, 161);s.store_sub_from_scalar(1529, 1.2, 1532);s.store_add_scaled_products_indices(267, 158, 1528, 1.0, 1529, 1527, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1527, 1537, 1524, A::add_scaled_inputs3(s.ad_value(1538), 1.0, s.ad_value(1539), 1.0, s.ad_value(157), -1.0));s.store_sub(1532, 162, 157);s.store_sub_from_scalar(1529, 1.2, 1532);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1528, 1.0, 1527, 1529, (-1.0));}
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {s.store_mul_sqrt_mixed_ia(1556, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1540, ((1.0 - -1.0) / 2.0));s.store_scalar(1541, ((1.0 + -1.0) / 2.0));}
        s.b[1608] = (p.p43 == 1.0);s.store_scalar(1608, if s.b[1608] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {s.store_add_scaled_products_mixed_iiia(1550, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1551, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1552, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1553, 1551, 1550);s.store_sub(1555, 1552, 1550);s.store_neg(1554, 1550);s.store_primal_add_scaled_products_indices(1542, 1540, 461, 1.0, 1541, 462, 1.0);s.store_primal_add_scaled_products_indices(1543, 1540, 462, 1.0, 1541, 461, 1.0);}
    }
}
