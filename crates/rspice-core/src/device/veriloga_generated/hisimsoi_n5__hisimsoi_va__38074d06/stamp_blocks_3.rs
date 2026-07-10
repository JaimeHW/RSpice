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
            let t3: f64 = (s.v[57] + 1.0);let t4: f64 = if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (s.v[167] <= t3)) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
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
            let (t1,) = {
    if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1277]) {
        let t0: f64 = (-1.0);
        (t0,)
    } else {
        (s.v[338],)
    }
};
            s.store_scalar(338, t1);s.b[1278] = (s.v[181] < 0.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
            if (((s.b[1222] && s.b[1268]) && (!s.b[1271])) && s.b[1278]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1279] = (s.v[181] < 1e-7);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && s.b[1279]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1278])) && (!s.b[1279])) {s.store_mul_scale_offset_indices(501, 225, 1270, 1.0, (-p.p287));s.store_exp(502, 501);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
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
            let (t2,) = {
    if ((((s.b[1222] && s.b[1268]) && (!s.b[1271])) && (!s.b[1280])) && s.b[1282]) {
        (1.0,)
    } else {
        (s.v[430],)
    }
};
            s.store_scalar(430, t2);
            if ((s.b[1222] && s.b[1268]) && (!s.b[1271])) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
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
        let (t6,) = {
    if (s.b[1222] && s.b[1287]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t6);
        let (t7,) = {
    if (s.b[1222] && s.b[1287]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t7);
        if (s.b[1222] && s.b[1287]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1288] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });s.b[1289] = (1.0 == 1.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        let (t8,) = {
    if (((s.b[1222] && s.b[1287]) && s.b[1288]) && s.b[1289]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t8);s.b[1290] = (1.0 == 2.0);s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && s.b[1290]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t9);s.b[1291] = (1.0 == 4.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
        let (ta,) = {
    if (((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && s.b[1291]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, ta);s.b[1292] = (1.0 == 8.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((((((s.b[1222] && s.b[1287]) && s.b[1288]) && (!s.b[1289])) && (!s.b[1290])) && (!s.b[1291])) && s.b[1292]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, tb);
        let (tc,) = {
    if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
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
            let tf: f64 = if (((s.b[1222] && s.b[1287]) && s.b[1288]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1222] && s.b[1287]) && s.b[1288]) {s.store_sqrt(53, 53);}
            let (te,) = {
    if ((s.b[1222] && s.b[1287]) && s.b[1288]) {
        let td: f64 = (s.v[54] + 1.0);
        (td,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, te);
        }
        if ((s.b[1222] && s.b[1287]) && (!s.b[1288])) {s.store_powf(53, 53, (1.0 / 2.0));}
        if (s.b[1222] && s.b[1287]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 225, 0.2, 0.0, 53);s.store_sub_scaled_inputs(328, 225, 0.2, 43, 1.0);}
        if (s.b[1222] && (!s.b[1287])) {s.copy_ad(328, 1231);}
        if s.b[1222] {s.store_sqrt_offset_input(1241, 328, (10.0 * 2.220446049250313e-16));s.store_mul(1242, 1240, 1241);s.store_mul_div_scaled_inputs_indices(1238, 1242, 227, 2.0, 1229, 1.0);s.store_mul_product3_indices(204, 337, 1238, 1237, 107, 1.0);s.store_add(199, 202, 204);}
        s.store_add(201, 203, 204);s.b[1293] = ((p.p43 == 1.0) || (p.p45 == 1.0));s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });s.b[1306] = ((s.v[145] == 1.0) || (p.p25 == 0.0));s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1293] && s.b[1306]) {s.store_scalar(263, 0.0);}
        s.b[1307] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if ((s.b[1293] && (!s.b[1306])) && s.b[1307]) {s.store_scalar(263, 0.0);}
        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {s.store_offset_add_scaled_inputs3_offset_indices(445, 174, 1.0, 185, 1.0, 320, -1.0, (-s.v[136]), p.p48);}
        s.b[1308] = (p.p44 <= 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {s.copy_ad(1294, 445);s.store_square(1301, 323);s.copy_ad(1302, 545);s.store_div(1296, 1302, 1301);s.store_div_from_scalar(1303, 2.0, 1302);s.store_mul(1297, 1303, 1301);s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));s.store_offset_mul(1300, 1297, 1298, 1.0);s.store_sqrt_square_offset(44, 1300, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1299, 1300, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1309] = (s.v[1299] < 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1309]) {s.store_scalar(1299, 0.0);}
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) {s.store_offset(1299, 1299, 1e-50);s.store_sqrt(1299, 1299);s.store_add_scaled_product_mixed_aii(1304, A::mul_sub_from_scalar_rhs(s.ad_value(1296), 1.0, s.ad_value(1299)), 1.0, 1294, 137, 1.0);s.store_add_scaled_inputs3_mixed_iia(1305, 173, p.p122, 176, 1.0, A::mul3(s.ad_value(131), s.ad_value(129), s.ad_value(1304)), -1.0);s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1310] = (s.v[1305] < 0.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && s.b[1308]) && s.b[1310]) {s.store_scalar(1305, 0.0);}
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {s.store_mul(1294, 134, 445);s.store_div_square_rhs(1296, 545, 323);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1297, 2.0, 545, A::square(s.ad_value(323)));s.store_add_scaled_inputs_product_indices(1298, 1294, 1.0, 227, (-1.0), 130, 514, (-1.0));s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));s.store_add_scaled_product_indices(1298, 1298, 1.0, 130, 483, (-1.0));s.store_offset_mul(1299, 1297, 1298, 1.0);s.store_scaled_offset(1301, 1297, 1.0, 2.0);}
        s.b[1311] = ((s.v[1299] < (1e-50 + s.v[1301])) && (s.v[1301] >= 0.0));s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.store_sub_offset_lhs(44, 1301, 1e-50, 1299);s.store_square(49, 44);s.store_square(50, 1301);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);}
        let (t11,) = {
    if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t11);
        let (t12,) = {
    if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {
        (0.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t12);
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) {s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1312] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });s.b[1313] = (4.0 == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        let (t13,) = {
    if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && s.b[1313]) {
        (1.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t13);s.b[1314] = (4.0 == 2.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        let (t14,) = {
    if (((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && s.b[1314]) {
        (2.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t14);s.b[1315] = (4.0 == 4.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        let (t15,) = {
    if ((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && s.b[1315]) {
        (3.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t15);s.b[1316] = (4.0 == 8.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        let (t16,) = {
    if (((((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (!s.b[1313])) && (!s.b[1314])) && (!s.b[1315])) && s.b[1316]) {
        (4.0,)
    } else {
        (s.v[55],)
    }
};
        s.store_scalar(55, t16);
        let (t17,) = {
    if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
        (0.0,)
    } else {
        (s.v[54],)
    }
};
        s.store_scalar(54, t17);let mut t1b: usize = 0;
        while {
            let t1a: f64 = if ((((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {s.store_sqrt(53, 53);}
            let (t19,) = {
    if (((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1311]) && s.b[1312]) {
        let t18: f64 = (s.v[54] + 1.0);
        (t18,)
    } else {
        (s.v[54],)
    }
};
            s.store_scalar(54, t19);
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
        if (((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) {s.store_add_mul_sub_from_scalar_rhs_indices(1304, 1294, 1296, 1.0, 1299);s.store_div_from_scalar_offset_input(1295, s.v[100], 131, s.v[100]);s.store_add_scaled_inputs_product_indices(1305, 173, p.p122, 176, 1.0, 1295, 1304, (-1.0));s.store_sqrt_square_offset(44, 1305, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1305, 1305, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1317] = (s.v[1305] < 0.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if ((((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) && (!s.b[1308])) && s.b[1317]) {s.store_scalar(1305, 0.0);}
        if ((s.b[1293] && (!s.b[1306])) && (!s.b[1307])) {s.store_offset(1305, 1305, 1e-50);s.store_ad_value(1295, A::exp_div_scaled_inputs(s.ad_value(133), -1.0, s.ad_value(1305), 1.0));s.store_mul_product3_indices(263, 1295, 132, 1305, 199, 1.0);}
        s.b[1318] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if s.b[1318] {s.store_mul_scaled_exp_scaled_input_rhs(1319, 107, (1.6021918e-19 * p.p237), 225, (-p.p141));s.store_offset_scaled(1320, 544, (((((36.0 * 1e-7) / 0.0001)) as f64).sqrt() * 13.0), ((((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * 36.0) * (1e20 / 1e-6)));s.store_div_scalar_by_product_indices(1321, (((((13.0 * 1e-7) / 0.0001)) as f64).sqrt() * ((((36.0 * 1e-7) / 0.0001)) as f64).sqrt()), 1319, 1320, 1.0);s.store_mul(567, 263, 1321);s.store_mul_scaled_ln_offset_rhs(1322, 227, p.p140, 567, 1.0);s.store_add_scaled_inputs3_indices(44, 231, 1.0, 1322, (-1.0), 231, (-0.01));s.store_scaled_mul(45, 231, 231, (4.0 * 0.01));}
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
        if s.b[1318] {s.store_sqrt_ad(1325, A::add_scaled_product(A::offset(A::exp(A::mul_scaled_lhs(s.ad_value(225), -1.0, s.ad_value(176))), (-1.0)), 1.0, s.ad_value(225), s.ad_value(176), 1.0));s.store_mul_sub_scaled_inputs_rhs_indices(1326, 1323, 1324, -1.0, 1325, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1318] {s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1326), (-(p.p47 * 0.01)));s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));}
        if s.b[1318] {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if s.b[1318] {s.store_sqrt_square_add(45, 44, 45);s.store_offset_add_scaled_inputs_indices(393, 44, (-0.5), 45, (-0.5), p.p47);}
        if s.b[1318] {s.store_scalar(1319, (if (p.p138 > 0.0) { p.p138 } else { 1.0 }));}
        if s.b[1318] {s.store_div_scaled_value_offset_denominator(398, s.ad_value(1319), 1.0, s.ad_value(263), p.p139, 1.0);s.store_mul(397, 398, 323);s.copy_ad(396, 393);s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));s.copy_ad(393, 596);s.store_div_scaled_inputs2_indices(592, 596, 1.0, 396, (-1.0), 397, 1.0);}
        s.b[1340] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });s.b[1341] = (s.v[56] < 3.0);s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1341]) {s.store_scalar(516, 0.0);s.store_scalar(517, 0.0);}
        if (s.b[1340] && (!s.b[1341])) {
            if (p.p43 == 1.0) {
                s.copy_ad(516, 156);
            } else {
                s.copy_ad(516, 350);
            }
        }
        if (s.b[1340] && (!s.b[1341])) {
            if (p.p43 == 1.0) {
                s.copy_ad(517, 156);
            } else {
                s.copy_ad(517, 353);
            }
        }
        if s.b[1340] {s.store_offset_scaled(1327, 185, p.p147, 1.0);s.store_scaled_mul(1328, 1327, 263, p.p146);s.store_offset_mul_ad(1329, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));s.store_sqrt_square_offset(44, 1329, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1329, 1329, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1342] = (s.v[1329] < 0.0);s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1342]) {s.store_scalar(1329, 0.0);}
        if s.b[1340] {s.store_sqrt(1330, 1329);s.store_mul(1331, 1329, 1330);s.store_offset_mul_ad(1332, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));s.store_sqrt_square_offset(44, 1332, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1332, 1332, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1343] = (s.v[1332] < 0.0);s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });
        if (s.b[1340] && s.b[1343]) {s.store_scalar(1332, 0.0);}
        if s.b[1340] {s.store_sqrt(1333, 1332);s.store_mul(1334, 1332, 1333);s.store_div_from_scalar(1335, 1.0, 1329);s.store_mul3_lhs(328, 225, 1328, 1335);s.store_div_from_scalar(1335, 1.0, 1332);s.store_mul3_lhs(1336, 225, 1328, 1335);s.store_mul_mixed_ia(1337, 238, A::add_scaled_products(s.ad_value(1334), s.ad_value(1336), 1.0, s.ad_value(1331), s.ad_value(328), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(1338, 238, 1333, 1336, ((-1.0) * (0.5)), 1330, 328, 0.5);s.store_add(1339, 1337, 1338);s.store_mul3_lhs(265, 264, 1339, 250);}
        s.store_scalar(1357, (s.v[88] * 100.0));s.store_scale(1358, 323, 0.0001);s.store_scalar(1359, (s.v[97] * 100.0));s.store_primal_scale(1360, 107, 100.0);s.store_scale(1361, 252, 0.01);s.store_scale(1362, 436, 0.0001);s.store_scale(1363, 238, 0.0001);s.b[1364] = (p.p27 == 0.0);s.store_scalar(1364, if s.b[1364] { 1.0 } else { 0.0 });
        if s.b[1364] {s.store_scalar(309, 0.0);s.store_scalar(306, 0.0);s.store_scalar(307, 0.0);s.store_scalar(308, 0.0);s.store_scalar(310, 0.0);}
        s.b[1365] = (s.v[145] == 0.0);s.store_scalar(1365, if s.b[1365] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1365]) {s.store_offset_add(1356, 176, 173, (-(10.0 * 2.220446049250313e-16)));s.store_add_scaled_inputs4_offset_indices(1346, 174, 1.0, 185, (p.p216 * s.v[1359]), 320, (-(p.p216 * s.v[1359])), 1356, (-p.p215), (-s.v[123]));s.store_scalar(1348, (1.0 / s.v[1357]));s.store_mul(1347, 1346, 1348);s.store_scalar(1348, (1.0 / p.p217));s.store_offset_mul(1352, 1361, 1348, 1.0);s.store_mul(1355, 1347, 1352);s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1366] = (s.v[1355] < 0.0);s.store_scalar(1366, if s.b[1366] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((!s.b[1364]) && s.b[1365]) && s.b[1366]) {s.store_scalar(1355, 0.0);}
        if ((!s.b[1364]) && s.b[1365]) {s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1348, 174, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1367] = (s.v[1348] < 0.0);s.store_scalar(1367, if s.b[1367] { 1.0 } else { 0.0 });
        if (((!s.b[1364]) && s.b[1365]) && s.b[1367]) {s.store_scalar(1348, 0.0);}
        if ((!s.b[1364]) && s.b[1365]) {s.store_offset(1348, 1348, (-p.p226));s.store_scale(1344, 1348, 10.0);s.store_offset_square(1347, 1344, 1.0);s.store_sub_from_scalar_ad(1346, 1.0, A::div_from_scalar(1.0, s.ad_value(1347)));s.store_mul(1355, 1355, 1346);s.store_scale(1345, 1360, s.v[1359]);s.store_div_from_scalar_offset_input(1352, p.p219, 1345, p.p219);s.store_scalar(1351, p.p218);s.store_div_add_scaled_inputs_rhs_indices(1353, 1351, 1351, 1.0, 173, 1.0);s.store_div_from_scalar_offset_input(1349, 1.0, 1355, 1e-50);s.store_scaled_mul(1346, 303, 1349, (-p.p214));}
        s.b[1368] = (s.v[1346] < (-34.0));s.store_scalar(1368, if s.b[1368] { 1.0 } else { 0.0 });
        if (((!s.b[1364]) && s.b[1365]) && s.b[1368]) {s.store_scalar(309, 0.0);}
        if (((!s.b[1364]) && s.b[1365]) && (!s.b[1368])) {s.store_exp(1347, 1346);s.store_mul_scale_offset_mixed_ia(1348, 1345, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 0.0);s.store_div_from_scalar(1350, 1.0, 1363);s.store_sqrt_mul_ad(1351, A::add_scaled_inputs(s.ad_value(1362), 1.0, s.ad_value(1358), 1e-12), s.ad_value(1350));s.store_mul3_lhs(1349, 1347, 1348, 1351);s.store_mul3_lhs(1354, 1349, 1355, 1355);s.store_mul3_lhs(309, 1352, 1353, 1354);}
        if ((!s.b[1364]) && (!s.b[1365])) {s.store_scalar(309, 0.0);}
        if (!s.b[1364]) {s.store_offset_scaled(1345, 158, (-p.p221), p.p222);s.store_exp_scaled_input(1347, 1345, s.v[1357]);s.store_scale(1345, 158, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));s.store_mul(1348, 158, 1345);s.store_scale(1349, 1360, (p.p220 / 1000000.0));s.store_mul3_lhs(306, 1349, 1347, 1348);}
        s.b[1369] = (s.v[158] >= 0.0);s.store_scalar(1369, if s.b[1369] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1369]) {s.store_scale(306, 306, (-1.0));}
        if (!s.b[1364]) {s.store_sub(1346, 158, 157);s.store_offset_scaled(1345, 1346, (-p.p221), p.p222);s.store_exp_scaled_input(1347, 1345, s.v[1357]);s.store_scale(1345, 1346, (1.0 / (s.v[1357]) * 1.0 / (s.v[1357])));s.store_mul(1348, 1346, 1345);s.store_scale(1349, 1360, (p.p220 / 1000000.0));s.store_mul3_lhs(307, 1349, 1347, 1348);}
        s.b[1370] = (s.v[1346] >= 0.0);s.store_scalar(1370, if s.b[1370] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1370]) {s.store_scale(307, 307, (-1.0));}
        if (!s.b[1364]) {s.store_offset_scaled_sub(1355, 513, 158, 1.0 / (s.v[1357]), ((((s.v[123]) + (p.p225))) * (1.0 / (s.v[1357]))));s.store_sqrt_square_offset(44, 1355, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1355, 1355, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1371] = (s.v[1355] < 0.0);s.store_scalar(1371, if s.b[1371] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1371]) {s.store_scalar(1355, 0.0);}
        if (!s.b[1364]) {s.store_offset(1355, 1355, 1e-50);s.store_div_from_scalar(1346, (-p.p224), 1355);}
        s.b[1372] = (s.v[1346] < (-34.0));s.store_scalar(1372, if s.b[1372] { 1.0 } else { 0.0 });
        if ((!s.b[1364]) && s.b[1372]) {s.store_scalar(308, 0.0);}
        if ((!s.b[1364]) && (!s.b[1372])) {s.store_exp(1347, 1346);s.store_scale(1348, 1360, (p.p223 * s.v[1359]));s.store_mul_product3_indices(308, 1347, 1348, 1355, 1355, 1.0);}
        if (!s.b[1364]) {s.store_scalar(310, 0.5);}
        s.b[1380] = (p.p28 == 0.0);s.store_scalar(1380, if s.b[1380] { 1.0 } else { 0.0 });
        if s.b[1380] {s.store_scalar(311, 0.0);}
        if (!s.b[1380]) {s.store_add_scaled_inputs4_offset_indices(1373, 157, p.p209, 158, (-1.0), 187, p.p211, 319, p.p211, (p.p210 * p.p209));s.store_scalar(1374, (1.0 / s.v[88]));s.store_mul(1375, 1373, 1374);s.store_sqrt_square_offset(44, 1375, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(304, 1375, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1381] = (s.v[304] < 0.0);s.store_scalar(1381, if s.b[1381] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1381]) {s.store_scalar(304, 0.0);}
        if (!s.b[1380]) {s.store_div_from_scalar_offset_input(1376, 1.0, 304, 1e-50);s.store_scaled_mul(1377, 303, 1376, (-p.p208));}
        s.b[1382] = (s.v[1377] < (-34.0));s.store_scalar(1382, if s.b[1382] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1382]) {s.store_scalar(311, 0.0);}
        if ((!s.b[1380]) && (!s.b[1382])) {s.store_exp(1373, 1377);s.store_mul_scale_offset_mixed_ia(1374, 107, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 0.0);s.store_mul_product3_indices(311, 1373, 1374, 304, 304, 1.0);}
        if (!s.b[1380]) {s.store_sub(1379, 157, 513);}
        s.b[1383] = (s.v[1379] > 0.0);s.store_scalar(1383, if s.b[1383] { 1.0 } else { 0.0 });
        if ((!s.b[1380]) && s.b[1383]) {s.store_square(1374, 1379);s.store_mul(331, 1374, 1379);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((!s.b[1380]) && s.b[1383]) {s.store_offset(1377, 331, p.p212);s.store_div(1378, 331, 1377);s.store_mul(311, 311, 1378);}
        if ((!s.b[1380]) && (!s.b[1383])) {s.store_scalar(311, 0.0);}
        s.b[1391] = (p.p28 == 0.0);s.store_scalar(1391, if s.b[1391] { 1.0 } else { 0.0 });
        if s.b[1391] {s.store_scalar(312, 0.0);}
        if (!s.b[1391]) {s.store_add_scaled_inputs3_mixed_aii(1384, A::add_scaled_inputs3_offset(s.ad_value(157), (-p.p209), s.ad_value(158), -1.0, s.ad_value(157), 1.0, ((p.p210) * (p.p209))), 1.0, 187, p.p211, 319, p.p211);s.store_scalar(1385, (1.0 / s.v[88]));s.store_mul(1386, 1384, 1385);s.store_sqrt_square_offset(44, 1386, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(305, 1386, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1392] = (s.v[305] < 0.0);s.store_scalar(1392, if s.b[1392] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1392]) {s.store_scalar(305, 0.0);}
        if (!s.b[1391]) {s.store_div_from_scalar_offset_input(1387, 1.0, 305, 1e-50);s.store_scaled_mul(1388, 303, 1387, (-p.p208));}
        s.b[1393] = (s.v[1388] < (-34.0));s.store_scalar(1393, if s.b[1393] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1393]) {s.store_scalar(312, 0.0);}
        if ((!s.b[1391]) && (!s.b[1393])) {s.store_exp(1384, 1388);s.store_div_from_scalar(1387, 1.0, 302);s.store_scaled_mul(1385, 1387, 107, (p.p207 * 1.6021918e-19));s.store_mul_product3_indices(312, 1384, 1385, 305, 305, 1.0);}
        if (!s.b[1391]) {s.store_neg(1390, 513);}
        s.b[1394] = (s.v[1390] > 0.0);s.store_scalar(1394, if s.b[1394] { 1.0 } else { 0.0 });
        if ((!s.b[1391]) && s.b[1394]) {s.store_square(1385, 1390);s.store_mul(331, 1385, 1390);s.store_offset(1388, 331, p.p212);s.store_div(1389, 331, 1388);s.store_mul(312, 312, 1389);}
        if ((!s.b[1391]) && (!s.b[1394])) {s.store_scalar(312, 0.0);}
        s.b[1395] = (p.p43 == 1.0);s.store_scalar(1395, if s.b[1395] { 1.0 } else { 0.0 });
        if s.b[1395] {s.store_scalar(1405, s.v[91]);s.store_primal_div_from_scalar(1406, 1.0, 1405);s.store_scalar(1462, 0.0);s.store_scalar(1464, 0.0);s.store_scalar(1466, 0.0);s.store_neg(1398, 534);s.store_mul(1399, 1398, 436);s.store_add_scaled_product_indices(331, 1399, 1.0, 1398, 437, 1.0);s.store_mul(470, 1399, 438);s.store_sub(469, 1399, 470);s.store_mul(468, 331, 438);s.store_sub(467, 331, 468);}
        if (s.b[1395] && (p.p24 != 0.0)) {s.copy_ad(521, 536);}
        let (t1c,) = {
    if (s.b[1395] && (p.p24 != 0.0)) {
        (0.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1c);s.b[1475] = (1.0 == 1.0);s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });s.b[1476] = (1.0 == 2.0);s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1475]) {s.store_primal_scale(522, 533, 0.5);s.store_scalar(523, p.p292);}
        let (t1d,) = {
    if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1475]) {
        (s.v[525],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1d);
        if ((s.b[1395] && (p.p24 != 0.0)) && (s.b[1476] && (!s.b[1475]))) {s.store_primal_scale(522, 534, 0.5);s.store_scalar(523, p.p68);}
        let (t1e,) = {
    if ((s.b[1395] && (p.p24 != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
        (s.v[524],)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1e);
        let (t1f,) = {
    if ((s.b[1395] && (p.p24 != 0.0)) && (s.b[1476] && (!s.b[1475]))) {
        (1.0,)
    } else {
        (s.v[528],)
    }
};
        s.store_scalar(528, t1f);s.b[1477] = (s.v[528] == 0.0);s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_mul_sqrt_mixed_ia(1425, 238, A::div(s.ad_value(521), s.ad_value(536)));s.store_scalar(1407, ((1.0 - -1.0) / 2.0));s.store_scalar(1408, ((1.0 + -1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1422, 1419, 1418);s.store_neg(1423, 1418);s.store_primal_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);s.store_primal_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1396, 1416);}
        s.b[1478] = (s.v[1396] > s.v[141]);s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1478]) {s.store_sub(1397, 1396, 141);s.store_sub(1398, 140, 141);s.store_div(44, 1397, 1398);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1404, 1398, 1404, -1.0, 1.0);s.store_add(1401, 141, 1404);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1478])) {s.copy_ad(1401, 1396);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));s.store_mul(1426, 1425, 1406);s.store_square(1427, 1426);s.store_sub(1428, 1424, 523);s.store_div(1396, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));}
        let (t21,) = {
    if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
        let t20: f64 = (-s.v[1417]);
        (t20,)
    } else {
        (s.v[1430],)
    }
};
        s.store_scalar(1430, t21);s.b[1479] = (s.v[1428] < s.v[1430]);s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {s.store_div_scalar_by_product_indices(1397, 1.0, 225, 1425, 1.0);s.store_mul(1404, 1397, 1405);s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);s.store_sub(1433, 237, 1429);s.store_mul_add_rhs(1403, 225, 1428, 1417);s.store_sub_from_scalar_scaled_mul_mixed_ia(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);s.store_square(1435, 1434);}
        s.b[1480] = (s.v[1432] < (s.v[1435] * 1e-8));s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && s.b[1480]) {s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) && (!s.b[1480])) {s.store_sqrt_add(1436, 1432, 1435);s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1479]) {s.store_powf(1438, 1437, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);s.store_div(1440, 1439, 1438);s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);s.store_add(1397, 1441, 1417);s.store_div(1398, 1397, 1433);s.store_sqrt_square_offset(1399, 1398, 1.0);s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);s.store_sub(1398, 1428, 1442);s.store_mul(459, 1405, 1398);s.copy_ad(458, 459);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_scalar(1440, 3.0);s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1481] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1481]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1482] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1482]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        s.b[1483] = (s.v[1440] < 3.0);s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1483]) {s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), 1.0, 1444, 1444, 9.0);s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_offset_add(1452, 1428, 1417, 0.1);s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1454, 1453, 1459);s.store_mul(1396, 226, 1427);s.store_mul(1455, 225, 1452);s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.store_offset_sub(44, 1455, 1456, (-1.0));s.store_scale(45, 1455, 4.0);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1455, 1455, 1456);s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.copy_ad(1458, 1440);s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);s.store_add_offset_lhs_mixed_ia(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));}
        s.b[1484] = (s.v[1397] < (10.0 * 2.220446049250313e-16));s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1484]) {s.store_scalar(1397, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) {s.store_sqrt(1398, 1397);s.store_mul(458, 1425, 1398);s.store_mul_sub_rhs(459, 1405, 1428, 1442);}
        s.b[1485] = (p.p42 == 1.0);s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1468, 1453, 1459);}
        let (t22,) = {
    if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {
        (0.0,)
    } else {
        (s.v[1413],)
    }
};
        s.store_scalar(1413, t22);
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t27: usize = 0;
        while {
            let t24: f64 = (2.0 * 20.0);let t25: f64 = (t24 + 1.0);let t26: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (s.v[167] <= t25)) { 1.0 } else { 0.0 };
            t26 != 0.0
        } {
            t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_scalar(1464, 0.0);s.store_mul_add_rhs(1440, 225, 1442, 1417);}
            s.b[1486] = (s.v[1440] < 5.0);s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1486]) {s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1462, 1468, 1460, 1460);s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);s.store_mul_scale_offset_mixed_ia(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);}
            s.b[1487] = (s.v[1440] < 80.0);s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && s.b[1487]) {s.store_exp(243, 1440);s.store_mul_scale_offset_indices(1462, 1468, 243, 1.0, (-1.0));s.store_mul3_lhs(1463, 1468, 225, 243);}
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) && (!s.b[1487])) {s.store_exp_mul(1469, 225, 1442);s.store_mul_sub_rhs(1462, 1453, 1469, 1459);s.store_mul3_lhs(1463, 1453, 225, 1469);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1486])) {s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);}
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);}
            s.b[1488] = (s.v[1413] == 1.0);s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1488]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1489] = (((s.v[494]) as f64).abs() > s.v[1472]);s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1489]) {s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) {s.store_add(1442, 1442, 494);}
            s.b[1490] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });
            let (t23,) = {
    if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1488])) && s.b[1490]) {
        (1.0,)
    } else {
        (s.v[1413],)
    }
};
            s.store_scalar(1413, t23);
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1492] = (s.v[1440] < 5.0);s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && s.b[1492]) {s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) && (!s.b[1492])) {s.store_offset(1473, 1440, (-1.0));s.store_sqrt(1474, 1473);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_mul(458, 1425, 1474);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1479])) && s.b[1485]) {s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));s.store_mul3_lhs(460, 1425, 1462, 1397);s.store_add(459, 458, 460);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_sub(460, 459, 458);}
        s.b[1494] = (1.0 == 1.0);s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });s.b[1495] = (1.0 == 2.0);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1494]) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1495] && (!s.b[1494]))) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_scalar(1407, ((1.0 - 1.0) / 2.0));s.store_scalar(1408, ((1.0 + 1.0) / 2.0));s.store_add_scaled_products_mixed_iiia(1418, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1419, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1420, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_add_scaled_products_mixed_iiia(1421, 462, 158, 1.0, 461, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1422, 1419, 1418);s.store_neg(1423, 1418);s.store_primal_add_scaled_products_indices(1409, 1407, 461, 1.0, 1408, 462, 1.0);s.store_primal_add_scaled_products_indices(1410, 1407, 462, 1.0, 1408, 461, 1.0);s.store_add_scaled_products_indices(1424, 1409, 1420, 1.0, 1410, 1421, 1.0);s.store_offset_ad(1416, A::add_scaled_products(s.ad_value(1409), s.ad_value(1423), 1.0, s.ad_value(1410), s.ad_value(1422), 1.0), (10.0 * 2.220446049250313e-16));s.store_neg(1396, 1416);}
        s.b[1496] = (s.v[1396] > s.v[141]);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1496]) {s.store_sub(1397, 1396, 141);s.store_sub(1398, 140, 141);s.store_div(44, 1397, 1398);s.store_square(45, 44);s.store_mul(46, 45, 44);s.store_square(47, 45);s.store_div_from_scalar_ad(1404, 1.0, A::add_scaled_inputs4_offset(s.ad_value(44), 1.0, s.ad_value(45), 1.0, s.ad_value(46), 1.0, s.ad_value(47), 1.0, 1.0));s.store_mul_scale_offset_indices(1404, 1398, 1404, -1.0, 1.0);s.store_add(1401, 141, 1404);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1496])) {s.copy_ad(1401, 1396);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_offset_scaled(1417, 1401, -1.0, (-1e-12));s.store_mul(1426, 1425, 1406);s.store_square(1427, 1426);s.store_sub(1428, 1424, 523);s.store_div(1396, 521, 230);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1429, 2.0, 225, A::ln(s.ad_value(1396)));}
        let (t29,) = {
    if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {
        let t28: f64 = (-s.v[1417]);
        (t28,)
    } else {
        (s.v[1430],)
    }
};
        s.store_scalar(1430, t29);s.b[1497] = (s.v[1428] < s.v[1430]);s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {s.store_div_scalar_by_product_indices(1397, 1.0, 225, 1425, 1.0);s.store_mul(1404, 1397, 1405);s.store_offset_scaled(1431, 1404, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(1432, 1431, 1431, 8.0, 0.0, 1431);s.store_sub(1433, 237, 1429);s.store_mul_add_rhs(1403, 225, 1428, 1417);s.store_sub_from_scalar_scaled_mul_mixed_ia(1434, (7.0 * 1.414213562373095), 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);s.store_square(1435, 1434);}
        s.b[1498] = (s.v[1432] < (s.v[1435] * 1e-8));s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && s.b[1498]) {s.store_add_scaled_inputs_product_mixed_aaia(1437, A::offset(s.ad_value(1434), ((-7.0) * 1.414213562373095)), 1.0, A::div_scaled_inputs(s.ad_value(1432), 0.5, s.ad_value(1434), 1.0), 1.0, 1404, A::offset(s.ad_value(1403), (-2.0)), 9.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) && (!s.b[1498])) {s.store_sqrt_add(1436, 1432, 1435);s.store_add_scaled_offset_product_rhs_mixed_aii(1437, A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), 1.0, 1404, 1403, (-2.0), 9.0);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1497]) {s.store_powf(1438, 1437, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(1439, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1404), 12.0)), 1.0, 1438, 2.0, 1438, 1438, 1.414213562373095);s.store_div(1440, 1439, 1438);s.store_add_scaled_product_indices(1441, 1417, (-1.0), 1440, 227, 1.0);s.store_add(1397, 1441, 1417);s.store_div(1398, 1397, 1433);s.store_sqrt_square_offset(1399, 1398, 1.0);s.store_sub_div_lhs_indices(1442, 1397, 1399, 1417);s.store_sub(1398, 1428, 1442);s.store_mul(459, 1405, 1398);s.copy_ad(458, 459);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_scalar(1440, 3.0);s.store_sub_div_lhs_indices(1443, 1440, 225, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1499] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1499]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);s.store_exp_neg_input(1404, 1440);s.store_offset_div_scaled_inputs2_mixed_aia(1403, A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1428), s.ad_value(1417))), (-1.0)), 4.0, 1404, 4.0, A::mul(s.ad_value(1427), s.ad_value(226)), 1.0, 1.0);}
        s.b[1500] = (s.v[1403] < (10.0 * 2.220446049250313e-16));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1500]) {s.store_scalar(1403, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_add_product3_rhs_mixed_iia(1443, 1428, 1427, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1403))), 1.0 / (2.0));s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        s.b[1501] = (s.v[1440] < 3.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {s.store_scalar(1444, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));s.store_scalar(1445, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));s.store_offset_div_from_scalar_ad(1446, 1.0, A::mul(s.ad_value(225), s.ad_value(1426)), (1.0 / 1.414213562373095));s.store_div_scaled_inputs2_indices(1447, 1428, -1.0, 1417, -1.0, 1426, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(1448, A::div_scaled_product(A::square(s.ad_value(1445)), s.ad_value(1445), 1.0, A::mul3_scaled_output(s.ad_value(1444), s.ad_value(1444), s.ad_value(1444), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(1445), s.ad_value(1446), 1.0, s.ad_value(1444), s.ad_value(1444), 6.0), (-1.0), 1447, 1.0, 1444, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1501]) {s.store_div_scaled_value_by_product_mixed_aii(1449, A::add_scaled_square_product(s.ad_value(1445), (-1.0), s.ad_value(1444), s.ad_value(1446), 3.0), 1.0, 1444, 1444, 9.0);s.store_sqrt_add_scaled_square_cube_product(1400, 1448, 1.0, 1449, 1.0);s.store_powf_ad(1450, A::sub(s.ad_value(1400), s.ad_value(1448)), 0.3333333333333333);s.store_neg_powf_add_input(1451, 1448, 1400, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(1403, 1450, 1.0, 1451, 1.0, 1445, 1.0, 1444, 3.0, -1.0);s.store_add_scaled_product_indices(1443, 1417, (-1.0), 1403, 227, 1.0);s.store_mul_add_rhs(1440, 225, 1443, 1417);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_offset_add(1452, 1428, 1417, 0.1);s.store_offset_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0), 1e-50);s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1454, 1453, 1459);s.store_mul(1396, 226, 1427);s.store_mul(1455, 225, 1452);s.store_add_scaled_inputs_product_mixed_aaii(1456, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.store_offset_sub(44, 1455, 1456, (-1.0));s.store_scale(45, 1455, 4.0);}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, 2.0, s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1456, 1455, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub(1455, 1455, 1456);s.store_add_scaled_inputs(1455, 1455, 1.0, 225, 0.1);s.store_add_scaled_inputs_product_mixed_aaii(1457, A::ln(A::add_scaled_square_product(s.ad_value(1455), 1.0, s.ad_value(1454), s.ad_value(1396), 1.0)), 1.0, A::ln(A::mul(s.ad_value(1453), s.ad_value(1396))), (-1.0), 225, 1417, 1.0);s.copy_ad(1458, 1440);s.store_offset_sub(44, 1457, 1458, (-(0.0008 * 75.0)));s.store_scale(45, 1457, (4.0 * (0.0008 * 75.0)));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt_square_add(45, 44, 45);s.store_offset_scaled_div(1397, 44, 45, 0.5, 0.5);s.store_offset_scaled_ad(1398, A::div_scaled_offset_numerator(s.ad_value(44), 1.0, ((2.0 * 0.0008) * 75.0), s.ad_value(45), 1.0), (-0.5), 0.5);s.store_add_scaled_inputs3_indices(1440, 1457, 1.0, 44, (-0.5), 45, (-0.5));s.store_sub_div_lhs_indices(1442, 1440, 225, 1417);s.store_add_offset_lhs_mixed_ia(1397, 1440, (-1.0), A::exp_scaled_input(s.ad_value(1440), -1.0));}
        s.b[1502] = (s.v[1397] < (10.0 * 2.220446049250313e-16));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1502]) {s.store_scalar(1397, (10.0 * 2.220446049250313e-16));}
        if (((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) {s.store_sqrt(1398, 1397);s.store_mul(458, 1425, 1398);s.store_mul_sub_rhs(459, 1405, 1428, 1442);}
        s.b[1503] = (p.p42 == 1.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_exp_ad(1459, A::mul_scaled_rhs(s.ad_value(225), s.ad_value(1417), -1.0));s.store_div(1396, 230, 521);s.store_square(1453, 1396);s.store_mul(1468, 1453, 1459);}
        let (t2a,) = {
    if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {
        (0.0,)
    } else {
        (s.v[1413],)
    }
};
        s.store_scalar(1413, t2a);
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut t2f: usize = 0;
        while {
            let t2c: f64 = (2.0 * 20.0);let t2d: f64 = (t2c + 1.0);let t2e: f64 = if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (s.v[167] <= t2d)) { 1.0 } else { 0.0 };
            t2e != 0.0
        } {
            t2f += 1;assert!(t2f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_scalar(1464, 0.0);s.store_mul_add_rhs(1440, 225, 1442, 1417);}
            s.b[1504] = (s.v[1440] < 5.0);s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1504]) {s.store_mul3_ad_middle(1460, A::square(s.ad_value(1440)), 1440, A::offset(A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 0.006115288895133179, (-0.07053654284009761))), 0.29693154855771));s.store_mul_scale_offset(1461, A::square(s.ad_value(1440)), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 0.006115288895133179), (4.0 * (-0.07053654284009761)))), 1.0, (3.0 * 0.29693154855771));s.store_mul3_lhs(1462, 1468, 1460, 1460);s.store_mul_product3_indices(1463, 1461, 1468, 225, 1460, 2.0);s.store_mul_scale_offset_mixed_ia(1464, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), 6.36964918866352e-5, (-0.00163730162779191))), 0.0178800506338833), (-0.117851130197758)), 1.0, 0.707106781186548);s.store_offset_mul_offset_rhs_mixed_ia(1465, 1440, A::mul_offset_rhs(s.ad_value(1440), A::mul(s.ad_value(1440), A::scale_offset(s.ad_value(1440), (5.0 * 6.36964918866352e-5), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833)), (2.0 * (-0.117851130197758)), 0.707106781186548);s.store_sqrt_offset_ad(1466, A::add(A::square(s.ad_value(1464)), s.ad_value(1462)), 1e-50);s.store_div_scaled_inputs2_mixed_aii(1467, A::mul3_scaled_output(s.ad_value(225), s.ad_value(1465), s.ad_value(1464), 2.0), 1.0, 1463, 1.0, 1466, 2.0);}
            s.b[1505] = (s.v[1440] < 80.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && s.b[1505]) {s.store_exp(243, 1440);s.store_mul_scale_offset_indices(1462, 1468, 243, 1.0, (-1.0));s.store_mul3_lhs(1463, 1468, 225, 243);}
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) && (!s.b[1505])) {s.store_exp_mul(1469, 225, 1442);s.store_mul_sub_rhs(1462, 1453, 1469, 1459);s.store_mul3_lhs(1463, 1453, 225, 1469);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1504])) {s.store_sqrt_add_ad(1466, A::offset(s.ad_value(1440), (-1.0)), s.ad_value(1462));s.store_scale_ad(1467, A::div_scaled_inputs2(s.ad_value(225), 1.0, s.ad_value(1463), 1.0, s.ad_value(1466), 1.0), 0.5);}
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_add_scaled_inputs_product_indices(1470, 1428, 1.0, 1442, (-1.0), 1426, 1466, (-1.0));s.store_sub_from_scalar_scaled_mul(1471, (-1.0), 1426, 1467, 1.0);}
            s.b[1506] = (s.v[1413] == 1.0);s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1506]) {s.store_scalar(167, ((2.0 * 20.0) + 1.0));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_div_scaled_inputs_indices(494, 1470, -1.0, 1471, 1.0);}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {
                s.store_scaled_offset_ad(1472, {
                    if (1.0 >= ((s.v[1442]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1442))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1507] = (((s.v[494]) as f64).abs() > s.v[1472]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
            if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1507]) {s.store_scale(494, 1472, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) {s.store_add(1442, 1442, 494);}
            s.b[1508] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1470]) as f64).abs() <= 1e-8));s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
            let (t2b,) = {
    if ((((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1506])) && s.b[1508]) {
        (1.0,)
    } else {
        (s.v[1413],)
    }
};
            s.store_scalar(1413, t2b);
            if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_primal_offset(167, 167, 1.0);}
        }
        s.b[1510] = (s.v[1440] < 5.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && s.b[1510]) {s.store_offset_square(1473, 1464, (10.0 * 2.220446049250313e-16));s.store_offset(1474, 1464, (10.0 * 2.220446049250313e-16));}
        if (((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) && (!s.b[1510])) {s.store_offset(1473, 1440, (-1.0));s.store_sqrt(1474, 1473);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_mul(458, 1425, 1474);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (!s.b[1497])) && s.b[1503]) {s.store_div_from_scalar_add_ad(1397, 1.0, s.ad_value(1466), s.ad_value(1474));s.store_mul3_lhs(460, 1425, 1462, 1397);s.store_add(459, 458, 460);}
        if ((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) {s.store_sub(460, 459, 458);}
        s.b[1512] = (1.0 == 1.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });s.b[1513] = (1.0 == 2.0);s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(463, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(465, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && s.b[1512]) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(464, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(466, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1407] != 0.0)) {s.store_mul_scale_offset_indices(467, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(469, 460, 522, -1.0, 0.0);}
        if ((((s.b[1395] && (p.p24 != 0.0)) && s.b[1477]) && (s.b[1513] && (!s.b[1512]))) && (s.v[1408] != 0.0)) {s.store_mul_scale_offset_indices(468, 459, 522, -1.0, 0.0);s.store_mul_scale_offset_indices(470, 460, 522, -1.0, 0.0);}
        s.store_scalar(317, p.p189);s.b[1516] = (s.v[145] != 0.0);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });
        if s.b[1516] {s.store_add(1515, 157, 161);s.store_add_scaled_inputs(314, 1515, s.v[317], 162, (1.0 - s.v[317]));}
        s.b[1517] = (p.p64 != 0.0);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_scalar(315, 0.0);}
        s.b[1518] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1518]) {s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));}
        s.b[1519] = (p.p64 != 0.0);s.store_scalar(1519, if s.b[1519] { 1.0 } else { 0.0 });s.b[1520] = (s.v[246] < 1e-15);s.store_scalar(1520, if s.b[1520] { 1.0 } else { 0.0 });
        if (((!s.b[1516]) && s.b[1519]) && s.b[1520]) {s.store_scalar(315, 0.0);}
        if (((!s.b[1516]) && s.b[1519]) && (!s.b[1520])) {s.store_scale(1514, 227, 1.0 / (s.v[97]));s.store_div_from_scalar(1515, 1.0, 244);s.store_mul3_lhs(315, 246, 1514, 1515);}
        s.store_scalar(1532, s.v[91]);s.store_scalar(1533, (1.0 / s.v[1532]));s.store_scalar(1553, 0.0);s.store_scalar(1593, 0.0);s.store_scalar(1591, 0.0);s.store_scalar(1595, 0.0);s.b[1604] = ((p.p29 >= 1.0) && (p.p188 > 0.0));s.store_scalar(1604, if s.b[1604] { 1.0 } else { 0.0 });
        if ((p.p24 != 0.0) && s.b[1604]) {s.store_scalar(1535, p.p171);s.store_scalar(1536, p.p172);s.copy_ad(1537, 158);s.store_scalar(1534, p.p188);}
        s.b[1605] = ((s.v[69] == 0.0) && (p.p188 > 0.0));s.store_scalar(1605, if s.b[1605] { 1.0 } else { 0.0 });
        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {
            if (p.p43 == 1.0) {
                s.store_scale(1522, 287, s.v[1532]);
            } else {
                s.store_scale(1522, 108, s.v[1532]);
            }
        }
        if (((p.p24 != 0.0) && s.b[1604]) && s.b[1605]) {s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add(s.ad_value(1536), s.ad_value(1537)));s.store_mul(1526, 1534, 1522);s.copy_ad(1530, 161);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_indices(267, 158, 1526, 1.0, 1527, 1525, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(1525, 1535, 1522, A::add_scaled_inputs3(s.ad_value(1536), 1.0, s.ad_value(1537), 1.0, s.ad_value(157), -1.0));s.store_sub(1530, 162, 157);s.store_sub_from_scalar(1527, 1.2, 1530);s.store_add_scaled_products_mixed_aiii(268, A::sub(s.ad_value(158), s.ad_value(157)), 1526, 1.0, 1525, 1527, (-1.0));}
        if (((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) {s.store_mul_sqrt_mixed_ia(1554, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));s.store_scalar(1538, ((1.0 - -1.0) / 2.0));s.store_scalar(1539, ((1.0 + -1.0) / 2.0));}
        s.b[1606] = (p.p43 == 1.0);s.store_scalar(1606, if s.b[1606] { 1.0 } else { 0.0 });
        if ((((p.p24 != 0.0) && s.b[1604]) && (!s.b[1605])) && s.b[1606]) {s.store_add_scaled_products_mixed_iiia(1548, 461, 156, 1.0, 462, A::sub(s.ad_value(156), s.ad_value(157)), 1.0);s.store_add_scaled_products_indices(1549, 461, 157, 1.0, 462, 157, -1.0);s.store_add_scaled_products_mixed_iiia(1550, 461, 158, 1.0, 462, A::sub(s.ad_value(158), s.ad_value(157)), 1.0);s.store_sub(1551, 1549, 1548);s.store_sub(1553, 1550, 1548);s.store_neg(1552, 1548);s.store_primal_add_scaled_products_indices(1540, 1538, 461, 1.0, 1539, 462, 1.0);s.store_primal_add_scaled_products_indices(1541, 1538, 462, 1.0, 1539, 461, 1.0);}
    }
}
