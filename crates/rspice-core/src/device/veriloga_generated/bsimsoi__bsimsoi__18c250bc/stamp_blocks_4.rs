#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[629]) {s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(507, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(514), 1.0, s.ad_value(346), s.ad_value(502), (-1.0)), s.ad_value(518), 1.0), 1.0, s.ad_value(523), (-1.0), s.ad_value(524), -1.0), 1.0, s.ad_value(125), s.ad_value(464), 1.0), 1.0, 520, 1.0, 517, -1.0, 519, -1.0, 521);s.store_sub(508, 504, 507);s.store_mul(497, 511, 499);s.store_div_scaled_product_indices(512, 384, 508, 1.0, 497, 1.0);s.store_div_scaled_inputs2_mixed_iai(513, 151, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(384), s.ad_value(508)), (-1.0), 497, 1.0);}
        s.b[635] = (s.v[512] > 100.0);s.store_scalar(635, if s.b[635] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[635]) {s.copy_ad(505, 508);}
        s.b[636] = (s.v[513] > 100.0);s.store_scalar(636, if s.b[636] { 1.0 } else { 0.0 });
        if (((!s.b[629]) && (!s.b[635])) && s.b[636]) {s.store_div_scaled_inputs2_by_product_indices(467, 508, 1.0, 151, (-1.0), 511, 499, 1.0);s.store_exp(515, 467);s.store_mul_div_scaled_product_indices(505, 515, 499, 367, 1.0, 396, 1.0);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_exp(515, 512);}
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {
            s.store_mul_mixed_ia(468, 497, {
                            if ((1.0 + s.v[515]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(515), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((!s.b[629]) && (!s.b[635])) && (!s.b[636])) {s.store_mul3_ad(471, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(498), s.ad_value(367)), 1.0), A::exp(s.ad_value(513)), A::sub_from_scalar(1.0, s.ad_value(384)));s.store_sub_mixed_ia(469, 384, A::div_scaled_product(s.ad_value(497), s.ad_value(471), 1.0, A::sub_from_scalar(1.0, s.ad_value(384)), 1.0));s.store_div(505, 468, 469);}
        if (!s.b[629]) {s.store_add_scaled_inputs3_indices(470, 408, p.p37, 406, (-1.0), 501, -1.0);s.store_scale(516, 470, 4.0);}
        s.b[637] = (s.v[516] < 0.0);s.store_scalar(637, if s.b[637] { 1.0 } else { 0.0 });
        if ((!s.b[629]) && s.b[637]) {s.store_scalar(516, 0.0);}
        if (!s.b[629]) {s.store_scalar(525, 0.0);s.copy_ad(526, 415);s.store_scalar(527, 1000000.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t43: usize = 0;
        while {
            let t0: f64 = (s.v[526] - s.v[527]);let t13: f64 = (s.dn[526][0] - s.dn[527][0]);let t14: f64 = (s.dn[526][1] - s.dn[527][1]);let t19: f64 = (s.dn[526][2] - s.dn[527][2]);let t1a: f64 = (s.dn[526][3] - s.dn[527][3]);let t1b: f64 = (s.dn[526][4] - s.dn[527][4]);let t1c: f64 = (s.dn[526][5] - s.dn[527][5]);let t1d: f64 = (s.dn[526][6] - s.dn[527][6]);let t1e: f64 = (s.dn[526][7] - s.dn[527][7]);let t1f: f64 = (s.dn[526][8] - s.dn[527][8]);let t20: f64 = (s.dn[526][9] - s.dn[527][9]);let t15: f64 = (s.dn[526][10] - s.dn[527][10]);let t16: f64 = (s.dn[526][11] - s.dn[527][11]);let t17: f64 = (s.dn[526][12] - s.dn[527][12]);let t18: f64 = (s.dn[526][13] - s.dn[527][13]);let t1: f64 = (s.db[526][0] - s.db[527][0]);let t2: f64 = (s.db[526][1] - s.db[527][1]);let tb: f64 = (s.db[526][2] - s.db[527][2]);let tc: f64 = (s.db[526][3] - s.db[527][3]);let td: f64 = (s.db[526][4] - s.db[527][4]);let te: f64 = (s.db[526][5] - s.db[527][5]);let tf: f64 = (s.db[526][6] - s.db[527][6]);let t10: f64 = (s.db[526][7] - s.db[527][7]);let t11: f64 = (s.db[526][8] - s.db[527][8]);let t12: f64 = (s.db[526][9] - s.db[527][9]);let t3: f64 = (s.db[526][10] - s.db[527][10]);let t4: f64 = (s.db[526][11] - s.db[527][11]);let t5: f64 = (s.db[526][12] - s.db[527][12]);let t6: f64 = (s.db[526][13] - s.db[527][13]);let t7: f64 = (s.db[526][14] - s.db[527][14]);let t8: f64 = (s.db[526][15] - s.db[527][15]);let t9: f64 = (s.db[526][16] - s.db[527][16]);let ta: f64 = (s.db[526][17] - s.db[527][17]);let t21: f64 = (t0).abs();let t34: f64 = if t0 >= 0.0 { t13 } else { (-t13) };let t35: f64 = if t0 >= 0.0 { t14 } else { (-t14) };let t3a: f64 = if t0 >= 0.0 { t19 } else { (-t19) };let t3b: f64 = if t0 >= 0.0 { t1a } else { (-t1a) };let t3c: f64 = if t0 >= 0.0 { t1b } else { (-t1b) };let t3d: f64 = if t0 >= 0.0 { t1c } else { (-t1c) };let t3e: f64 = if t0 >= 0.0 { t1d } else { (-t1d) };let t3f: f64 = if t0 >= 0.0 { t1e } else { (-t1e) };let t40: f64 = if t0 >= 0.0 { t1f } else { (-t1f) };let t41: f64 = if t0 >= 0.0 { t20 } else { (-t20) };let t36: f64 = if t0 >= 0.0 { t15 } else { (-t15) };let t37: f64 = if t0 >= 0.0 { t16 } else { (-t16) };
            let t38: f64 = if t0 >= 0.0 { t17 } else { (-t17) };let t39: f64 = if t0 >= 0.0 { t18 } else { (-t18) };let t22: f64 = if t0 >= 0.0 { t1 } else { (-t1) };let t23: f64 = if t0 >= 0.0 { t2 } else { (-t2) };let t2c: f64 = if t0 >= 0.0 { tb } else { (-tb) };let t2d: f64 = if t0 >= 0.0 { tc } else { (-tc) };let t2e: f64 = if t0 >= 0.0 { td } else { (-td) };let t2f: f64 = if t0 >= 0.0 { te } else { (-te) };let t30: f64 = if t0 >= 0.0 { tf } else { (-tf) };let t31: f64 = if t0 >= 0.0 { t10 } else { (-t10) };let t32: f64 = if t0 >= 0.0 { t11 } else { (-t11) };let t33: f64 = if t0 >= 0.0 { t12 } else { (-t12) };let t24: f64 = if t0 >= 0.0 { t3 } else { (-t3) };let t25: f64 = if t0 >= 0.0 { t4 } else { (-t4) };let t26: f64 = if t0 >= 0.0 { t5 } else { (-t5) };let t27: f64 = if t0 >= 0.0 { t6 } else { (-t6) };let t28: f64 = if t0 >= 0.0 { t7 } else { (-t7) };let t29: f64 = if t0 >= 0.0 { t8 } else { (-t8) };let t2a: f64 = if t0 >= 0.0 { t9 } else { (-t9) };let t2b: f64 = if t0 >= 0.0 { ta } else { (-ta) };let t42: f64 = if ((!s.b[629]) && ((s.v[525] <= 4.0) && (t21 > 1e-12))) { 1.0 } else { 0.0 };
            t42 != 0.0
        } {
            t43 += 1;assert!(t43 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[629]) {s.copy_ad(527, 526);s.store_scale(464, 526, 200000000.0);s.store_div_scaled_inputs2_indices(638, 505, 1.0, 516, 1.0, 464, 1.0);}
            if (!s.b[629]) {
                s.store_offset_ad(639, A::exp_scaled_input({
                    if (s.v[638] > 1e-38) {
                        A::ln(s.ad_value(638))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (p.p59 * 0.7)), 1.0);
            }
            if (!s.b[629]) {s.store_div_from_scalar(528, (p.p58 * 1.9e-9), 639);s.store_add_scaled_product_indices(526, 415, 1.0, 416, 528, (-1.0 / (p.p47)));s.store_primal_offset(525, 525, 1.0);}
        }
        if (!s.b[629]) {s.copy_ad(62, 526);}
        s.copy_ad(462, 341);s.store_sub(463, 115, 118);s.store_mul(464, 397, 462);s.store_div_scaled_inputs_indices(467, 133, ((-0.5) * (s.v[328] * s.v[327])), 464, 1.0);s.b[640] = (s.v[467] > (-100.0));s.store_scalar(640, if s.b[640] { 1.0 } else { 0.0 });
        if s.b[640] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        if (!s.b[640]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(469, 468, 468, 2.0, 1.0);}
        s.store_mul(467, 132, 469);s.store_mul(469, 467, 463);s.store_div_scaled_inputs_indices(467, 130, ((-0.5) * s.v[327]), 464, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[641] = (s.v[467] > (-100.0));s.store_scalar(641, if s.b[641] { 1.0 } else { 0.0 });
        if s.b[641] {s.store_exp(468, 467);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        if (!s.b[641]) {s.store_scalar(468, 3.720075976e-44);s.store_mul_scale_offset_rhs(470, 468, 468, 2.0, 1.0);}
        s.store_mul3_lhs(470, 129, 470, 463);s.store_div_scaled_product_offset_denominator_indices(471, 62, 118, 1.0, 127, s.v[328], 1.0);s.store_sqrt_offset_scaled_input(467, 128, 1.0 / (s.v[327]), 1.0);s.store_add_scaled_product_mixed_aai(472, A::mul3(s.ad_value(376), A::offset(s.ad_value(467), (-1.0)), s.ad_value(339)), 1.0, A::add_scaled_inputs(s.ad_value(121), 1.0, s.ad_value(122), 1.0 / (s.v[327])), 430, 1.0);s.store_add_mixed_ai(531, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(469), (-1.0), s.ad_value(470), -1.0), 1.0, s.ad_value(125), s.ad_value(471), 1.0), 472);s.store_add_scaled_inputs_product_indices(359, 531, 1.0, 118, (-1.0), 120, 339, (-1.0));s.store_mul_scale_offset_rhs(344, 108, 128, ((1.0 / (s.v[327])) * ((1.602176462e-19 * (1000000.0 * p.p155)))), (1.602176462e-19 * (1000000.0 * p.p155)));s.store_scalar(64, (((p.p424 * (p.p427 + (((s.v[328] / p.p23) / 3.0) / p.p425))) / ((p.p425 * p.p3) * (p.p1 - p.p428))) + (p.p426 / ((p.p1 * s.v[328]) * p.p3))));s.b[642] = (s.v[64] > 0.0);s.store_scalar(642, if s.b[642] { 1.0 } else { 0.0 });
        if s.b[642] {s.store_scalar(64, (1.0 / s.v[64]));}
        if (!s.b[642]) {s.store_scalar(64, 1000.0);}
        s.store_offset(67, 359, (p.p37 * p.p20));s.store_scaled_sqrt_ad(360, A::div_scaled_product(s.ad_value(417), s.ad_value(480), 1.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(468, 408, p.p37, 406, (-1.0), 118, -1.0);s.store_scale(469, 468, 2.0);s.store_scale(470, 468, 2.5);
        if (p.p37 == 1.0) {
            s.copy_ad(68, 469);
        } else {
            s.copy_ad(68, 470);
        }
        s.b[646] = (s.v[68] < 0.0);s.store_scalar(646, if s.b[646] { 1.0 } else { 0.0 });
        if s.b[646] {s.store_scalar(68, 0.0);}
        s.b[647] = (p.p62 == 4.0);s.store_scalar(647, if s.b[647] { 1.0 } else { 0.0 });
        if s.b[647] {s.store_mul(509, 397, 341);s.store_div_scaled_inputs_indices(467, 130, s.v[327], 509, 1.0);}
        s.b[648] = (s.v[467] < 100.0);s.store_scalar(648, if s.b[648] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[648]) {s.store_exp(468, 467);s.store_offset(469, 468, (-1.0));s.store_square(470, 469);s.store_add_scaled_inputs(471, 470, 1.0, 468, (2.0 * 3.720075976e-44));s.store_div(522, 468, 471);}
        if (s.b[647] && (!s.b[648])) {s.store_scalar(522, (1.0 / (2.688117142e43 - 2.0)));}
        if s.b[647] {s.store_div(463, 417, 340);s.store_mul(464, 100, 463);s.store_div_scaled_inputs2_mixed_aii(531, A::add_scaled_product(s.ad_value(464), 1.0, s.ad_value(96), s.ad_value(522), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[649] = (s.v[531] >= (-0.5));s.store_scalar(649, if s.b[649] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[649]) {s.store_offset(529, 531, 1.0);}
        if (s.b[647] && (!s.b[649])) {s.store_div_from_scalar_offset_scaled_input(467, 1.0, 531, 8.0, 3.0);s.store_mul_scale_offset_rhs(529, 467, 531, 3.0, 1.0);}
        if s.b[647] {s.store_mul(467, 529, 480);s.copy_ad(468, 151);s.store_div(469, 468, 467);}
        s.b[650] = (s.v[469] < (-100.0));s.store_scalar(650, if s.b[650] { 1.0 } else { 0.0 });
        if (s.b[647] && s.b[650]) {s.store_div_scaled_inputs_indices(470, 396, 3.720075976e-44, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        s.b[651] = (s.v[469] > 100.0);s.store_scalar(651, if s.b[651] { 1.0 } else { 0.0 });
        if ((s.b[647] && (!s.b[650])) && s.b[651]) {s.store_div_scaled_inputs_indices(470, 396, 2.688117142e43, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if ((s.b[647] && (!s.b[650])) && (!s.b[651])) {s.store_div_scaled_product_mixed_aii(470, A::exp(s.ad_value(469)), 396, 1.0, 367, 1.0);s.store_add_scaled_product_indices(471, 384, 1.0, 470, 529, 1.0);}
        if s.b[647] {s.store_div_scaled_inputs_indices(69, 467, 0.6931471805599453, 471, 1.0);}
        if (!s.b[647]) {s.store_scalar(69, 0.0);}
        s.b[704] = ((p.p38 >= 4.4) || (p.p63 != 0.0));s.store_scalar(704, if s.b[704] { 1.0 } else { 0.0 });s.b[705] = (s.v[106] < 0.01);s.store_scalar(705, if s.b[705] { 1.0 } else { 0.0 });
        if (s.b[704] && s.b[705]) {s.store_scalar(106, 0.01);}
        s.b[706] = (s.v[106] > 1.0);s.store_scalar(706, if s.b[706] { 1.0 } else { 0.0 });
        if ((s.b[704] && (!s.b[705])) && s.b[706]) {s.store_scalar(106, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[704] && (!s.b[705])) && s.b[706]) {s.store_scalar(105, 0.0);}
        s.b[707] = (s.v[181] < 0.0);s.store_scalar(707, if s.b[707] { 1.0 } else { 0.0 });
        if s.b[707] {s.store_scalar(181, 0.0);s.store_scalar(182, 0.0);}
        s.b[708] = ((s.v[182] < 0.001) && (s.v[182] != 0.0));s.store_scalar(708, if s.b[708] { 1.0 } else { 0.0 });
        if ((!s.b[707]) && s.b[708]) {s.store_scalar(182, 0.0);}
        s.b[738] = (s.v[308] < 0.0);s.store_scalar(738, if s.b[738] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[738]) {s.store_scalar(308, 0.0);}
        s.b[739] = (s.v[309] < 0.0);s.store_scalar(739, if s.b[739] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[739]) {s.store_scalar(309, 0.0);}
        s.b[740] = (s.v[310] < 0.0);s.store_scalar(740, if s.b[740] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[740]) {s.store_scalar(310, 0.0);}
        s.b[741] = (s.v[311] < 0.0);s.store_scalar(741, if s.b[741] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[741]) {s.store_scalar(311, 0.0);}
        s.b[742] = (s.v[312] < 0.0);s.store_scalar(742, if s.b[742] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[742]) {s.store_scalar(312, 0.0);}
        s.b[743] = (s.v[313] < 0.0);s.store_scalar(743, if s.b[743] { 1.0 } else { 0.0 });
        if ((p.p63 != 0.0) && s.b[743]) {s.store_scalar(313, 0.0);}
        s.store_scalar(410, 0.0);s.b[805] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(805, if s.b[805] { 1.0 } else { 0.0 });s.b[806] = ((p.p35 != 0.0) && (!true));s.store_scalar(806, if s.b[806] { 1.0 } else { 0.0 });s.b[807] = true;s.store_scalar(807, if s.b[807] { 1.0 } else { 0.0 });
        if ((s.b[805] && s.b[806]) && s.b[807]) {s.store_voltage(410, ctx, nodes, Some(5), None);}
        s.b[808] = true;s.store_scalar(808, if s.b[808] { 1.0 } else { 0.0 });
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && s.b[808]) {s.store_voltage(410, ctx, nodes, Some(4), None);}
        if (((s.b[805] && s.b[806]) && (!s.b[807])) && (!s.b[808])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        if (s.b[805] && (!s.b[806])) {s.store_voltage(410, ctx, nodes, Some(6), None);}
        s.store_offset(409, 410, s.v[409]);s.store_scale(411, 409, 1.0 / (s.v[429]));s.store_offset(430, 411, (-1.0));s.store_scalar(1133, 0.0);s.store_scalar(1134, 0.0);s.store_scalar(1135, 0.0);s.store_scalar(1136, 0.0);s.store_scalar(1131, 0.0);s.store_scalar(1121, 0.0);s.store_scalar(855, 0.0);s.store_scalar(1122, 0.0);s.store_scalar(1130, 0.0);s.store_scalar(1127, 0.0);s.store_scalar(1128, 0.0);s.store_scalar(1126, 0.0);s.store_scalar(1118, 0.0);s.copy_ad(955, 182);s.copy_ad(1095, 173);s.copy_ad(1096, 174);s.b[1159] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });s.b[1160] = (p.p41 == 0.0);s.store_scalar(1160, if s.b[1160] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1160]) {s.store_scale(832, 409, 8.617087e-5);s.store_offset(843, 409, 1108.0);s.store_square(848, 409);s.store_sub_from_scalar_ad(912, 1.16, A::div_scaled_inputs(s.ad_value(848), 0.000702, s.ad_value(843), 1.0));s.store_scalar(845, 0.00019230584);s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, 14500000000.0, 0.0, 845);s.store_sub_from_scalar_ad(849, 21.5565981, A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0));}
        s.b[1161] = (s.v[849] > (-100.0));s.store_scalar(1161, if s.b[1161] { 1.0 } else { 0.0 });
        if ((s.b[1159] && s.b[1160]) && s.b[1161]) {s.store_exp(847, 849);}
        if ((s.b[1159] && s.b[1160]) && (!s.b[1161])) {s.store_scalar(847, (((-100.0)) as f64).exp());}
        if (s.b[1159] && s.b[1160]) {s.store_mul(911, 846, 847);}
        if (s.b[1159] && s.b[1160]) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1160]) {s.store_mul(940, 832, 843);}
        if (s.b[1159] && (!s.b[1160])) {s.store_scalar(429, (p.p126 + 273.15));s.store_scale(832, 409, 8.617087e-5);s.store_primal_scale(1104, 429, 8.617087e-5);s.copy_ad(1103, 394);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1159] && (!s.b[1160])) {s.store_sub_from_scalar_ad(912, p.p49, A::div_scaled_product_offset_denominator(s.ad_value(409), s.ad_value(409), p.p50, s.ad_value(409), p.p51, 1.0));s.store_div_from_scalar_sqrt_ad(845, 1.0, A::mul(A::square(s.ad_value(429)), s.ad_value(429)));s.store_sqrt(848, 409);s.store_mul3_affine_lhs(846, 409, 848, p.p48, 0.0, 845);s.store_exp_ad(847, A::sub(A::div_scaled_inputs(s.ad_value(1103), 1.0, s.ad_value(1104), 2.0), A::div_scaled_inputs(s.ad_value(912), 1.0, s.ad_value(832), 2.0)));s.store_mul(911, 846, 847);}
        if (s.b[1159] && (!s.b[1160])) {
            if (((1e20 * s.v[108]) / (s.v[911] * s.v[911])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(843, 108, 1e20, 911, 1.0);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1160])) {s.store_mul(940, 832, 843);}
        s.b[1162] = (s.v[109] > 0.0);s.store_scalar(1162, if s.b[1162] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1162]) {
            if ((s.v[108] / s.v[109]) > 1e-38) {
                s.store_ln_div(843, 108, 109);
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && s.b[1162]) {s.store_scaled_mul(941, 832, 843, (-p.p37));}
        if (s.b[1159] && (!s.b[1162])) {
            if (((((-s.v[108]) * s.v[109]) / s.v[911]) / s.v[911]) > 1e-38) {
                s.store_ln_ad(843, A::div_scaled_product_by_product(s.ad_value(108), s.ad_value(109), -1.0, s.ad_value(911), s.ad_value(911), 1.0));
            } else {
                s.store_scalar(843, -(87.49823353377374));
            }
        }
        if (s.b[1159] && (!s.b[1162])) {s.store_scaled_mul(941, 832, 843, (-p.p37));}
        if s.b[1159] {
            s.store_mul_scale_offset_mixed_ia(942, 832, {
                if ((s.v[108] / s.v[911]) > 1e-38) {
                    A::ln(A::div(s.ad_value(108), s.ad_value(911)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if s.b[1159] {s.store_sqrt(943, 942);s.store_mul_sqrt_mixed_ia(944, 943, A::div_scaled_inputs(s.ad_value(417), 2.0, s.ad_value(108), (1.602176462e-19 * 1000000.0)));s.store_div_mixed_ai(1140, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(417), 1.602176462e-19, s.ad_value(108)), (1000000.0 * 1.0 / (2.0))), 943);s.store_sqrt_ad(844, A::mul3(A::div_scaled_inputs(s.ad_value(417), 1.0, s.ad_value(416), 8.85418e-12), s.ad_value(415), s.ad_value(944)));s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(136), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));s.store_add_scaled_product_indices(1141, 843, 1.0, 843, 843, 2.0);s.store_ad_value(843, A::exp_div_scaled_inputs(s.ad_value(135), ((-0.5) * s.v[327]), s.ad_value(844), 1.0));s.store_add_scaled_product_indices(845, 843, 1.0, 843, 843, 2.0);s.store_add_scaled_product_indices(1142, 193, 1.0, 192, 845, 1.0);s.copy_ad(49, 832);s.store_mul_div_from_scalar_lhs_ad_indices(847, 1.115, 832, 430);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 300, 1.0);}
        s.b[1163] = (s.v[850] > 100.0);s.store_scalar(1163, if s.b[1163] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1163]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1164] = (s.v[850] < (-100.0));s.store_scalar(1164, if s.b[1164] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1163])) && s.b[1164]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1163])) && (!s.b[1164])) {s.store_exp(843, 850);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1165] = (s.v[256] == s.v[257]);s.store_scalar(1165, if s.b[1165] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1165]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1165])) {s.store_div_scaled_product_indices(850, 257, 847, 1.0, 300, 1.0);}
        s.b[1166] = (s.v[850] > 100.0);s.store_scalar(1166, if s.b[1166] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1165])) && s.b[1166]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1167] = (s.v[850] < (-100.0));s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && s.b[1167]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1165])) && (!s.b[1166])) && (!s.b[1167])) {s.store_exp(844, 850);}
        if s.b[1159] {s.store_div_scaled_product_indices(850, 258, 847, 1.0, 302, 1.0);}
        s.b[1168] = (s.v[850] > 100.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1168]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1169] = (s.v[850] < (-100.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1168])) && s.b[1169]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1168])) && (!s.b[1169])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(972, 355, 843);s.store_mul(949, 306, 843);s.store_mul(947, 308, 844);s.store_mul(951, 310, 845);s.store_mul(850, 259, 430);}
        s.b[1170] = (s.v[850] > 100.0);s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1170]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1171] = (s.v[850] < (-100.0));s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1170])) && s.b[1171]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1170])) && (!s.b[1171])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(953, 312, 843);s.store_div_scaled_product_indices(850, 256, 847, 1.0, 301, 1.0);}
        s.b[1172] = (s.v[850] > 100.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1172]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1173] = (s.v[850] < (-100.0));s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1172])) && s.b[1173]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1172])) && (!s.b[1173])) {s.store_exp(843, 850);}
        s.b[1174] = (s.v[256] == s.v[260]);s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1174]) {s.copy_ad(844, 843);}
        if (s.b[1159] && (!s.b[1174])) {s.store_div_scaled_product_indices(850, 260, 847, 1.0, 301, 1.0);}
        s.b[1175] = (s.v[850] > 100.0);s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1174])) && s.b[1175]) {s.store_scaled_offset(844, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1176] = (s.v[850] < (-100.0));s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && s.b[1176]) {s.store_scalar(844, 3.720075976e-44);}
        if (((s.b[1159] && (!s.b[1174])) && (!s.b[1175])) && (!s.b[1176])) {s.store_exp(844, 850);}
        if s.b[1159] {s.store_div_scaled_product_indices(850, 261, 847, 1.0, 303, 1.0);}
        s.b[1177] = (s.v[850] > 100.0);s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1177]) {s.store_scaled_offset(845, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1178] = (s.v[850] < (-100.0));s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1177])) && s.b[1178]) {s.store_scalar(845, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1177])) && (!s.b[1178])) {s.store_exp(845, 850);}
        if s.b[1159] {s.store_mul(973, 356, 843);s.store_mul(950, 307, 843);s.store_mul(948, 309, 844);s.store_mul(952, 311, 845);s.store_mul(850, 262, 430);}
        s.b[1179] = (s.v[850] > 100.0);s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1179]) {s.store_scaled_offset(843, 850, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1180] = (s.v[850] < (-100.0));s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if ((s.b[1159] && (!s.b[1179])) && s.b[1180]) {s.store_scalar(843, 3.720075976e-44);}
        if ((s.b[1159] && (!s.b[1179])) && (!s.b[1180])) {s.store_exp(843, 850);}
        if s.b[1159] {s.store_mul(954, 313, 843);s.store_mul_pow_indices(945, 144, 411, 145);}
        s.b[1181] = (p.p38 < 4.2);s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1181]) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(411), p.p238, 1.0), 1e-9);}
        if (s.b[1159] && (!s.b[1181])) {s.store_offset_mul_ad(961, s.ad_value(231), A::scale_offset(s.ad_value(430), p.p238, 1.0), 1e-9);}
        if s.b[1159] {s.store_scale(850, 235, p.p235);s.store_div(960, 850, 961);s.store_scale(847, 51, p.p235);s.store_div(959, 847, 961);s.store_offset(845, 959, 1.0);s.store_offset(850, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(945, 945, 843);s.store_add_scaled_product_indices(946, 101, 1.0, 102, 430, (-1.0));s.store_offset_mul(845, 45, 959, 1.0);s.store_offset_mul(850, 45, 960, 1.0);s.store_div(843, 845, 850);s.store_mul(946, 946, 843);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1182] = (p.p429 != 1.0);s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if (s.b[1159] && s.b[1182]) {s.store_div_scaled_add_product_indices(955, 181, 1.0, 186, 430, 1.0, 159, 1.0);s.store_scalar(1095, 0.0);s.store_scalar(1096, 0.0);}
        if (s.b[1159] && (!s.b[1182])) {s.store_scalar(955, 0.0);s.store_scale(1094, 159, p.p3);s.store_mul(853, 186, 430);s.store_add(844, 169, 853);s.store_offset(845, 853, p.p140);s.store_div(1095, 844, 1094);s.store_add(850, 170, 853);s.store_offset(847, 853, p.p139);s.store_div(1096, 850, 1094);}
        if s.b[1159] {s.store_add_scaled_product_indices(956, 153, 1.0, 139, 430, 1.0);s.store_add_scaled_product_indices(957, 154, 1.0, 141, 430, 1.0);s.store_add_scaled_product_indices(958, 155, 1.0, 143, 430, 1.0);}
        if (!s.b[1159]) {s.copy_ad(940, 115);s.copy_ad(941, 160);s.copy_ad(942, 118);s.copy_ad(943, 339);s.copy_ad(944, 340);s.copy_ad(912, 395);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);s.copy_ad(949, 161);s.copy_ad(950, 162);s.copy_ad(947, 163);s.copy_ad(948, 164);s.copy_ad(951, 165);s.copy_ad(952, 166);s.copy_ad(953, 167);s.copy_ad(954, 168);s.copy_ad(972, 357);s.copy_ad(973, 358);s.copy_ad(945, 404);s.copy_ad(946, 407);s.copy_ad(956, 138);s.copy_ad(957, 140);s.copy_ad(958, 142);}
        s.b[1183] = (param_given[90] || param_given[94]);s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });s.b[1184] = (!param_given[90]);s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1184]) {s.store_scalar(120, 0.53);}
        s.b[1185] = (!param_given[94]);s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });
        if (s.b[1183] && s.b[1185]) {s.store_scalar(124, (-0.0186));}
        s.b[1186] = (!param_given[87]);s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });
        if (((!s.b[1183]) && s.b[1186]) && (p.p41 != 0.0)) {s.store_scaled_div_from_scalar_ad(843, 1.602176462e-19, A::scale(s.ad_value(417), 2.0), 1000000.0);}
        if (((!s.b[1183]) && s.b[1186]) && (p.p41 == 0.0)) {s.store_scalar(843, 0.00077348);}
        if ((!s.b[1183]) && s.b[1186]) {s.store_add_scaled_product_indices(114, 942, 1.0, 843, 108, (-(s.v[117] * s.v[117])));}
        s.b[1187] = (s.v[114] > 0.0);s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1187]) {s.store_neg(114, 114);}
        s.b[1188] = (s.v[116] > 0.0);s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1188]) {s.store_primal_neg(116, 116);}
        s.b[1189] = (!param_given[85]);s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1189]) {s.store_div_scaled_product_mixed_iai(112, 419, A::sqrt(s.ad_value(108)), 1.0, 396, 1.0);}
        s.b[1190] = (!param_given[86]);s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });
        if ((!s.b[1183]) && s.b[1190]) {s.store_div_scaled_product_mixed_iai(113, 419, A::sqrt(s.ad_value(109)), 1.0, 396, 1.0);}
        if (!s.b[1183]) {s.store_sub(843, 112, 113);s.store_sub_mixed_ai(844, A::sqrt(A::sub(s.ad_value(942), s.ad_value(114))), 943);s.store_mul_sub_mixed_iai(845, 943, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), 943);s.store_div_scaled_product_add_scaled_denominator_indices(846, 843, 844, 1.0, 845, 2.0, 116, 1.0, 1.0);s.store_add_scaled_inputs3_indices(402, 402, 1.0, 124, (-1.0), 846, 1.0);s.store_add_scaled_product_mixed_iia(120, 113, 1.0, 402, A::sqrt(A::sub(s.ad_value(942), s.ad_value(116))), (-2.0));}
        s.store_offset(843, 265, s.v[328]);s.b[1191] = (s.v[843] < 1e-8);s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });
        if s.b[1191] {s.store_scalar(843, 1e-8);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_mul_scale_offset_mixed_ia(405, 120, A::div(s.ad_value(264), s.ad_value(843)), 1.0, 1.0);s.store_scale(376, 405, (p.p66 * 1.0 / (p.p67)));s.store_scale(403, 402, (p.p66 * 1.0 / (p.p67)));s.b[1192] = (!param_given[109]);s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });s.b[1193] = (param_given[108] || param_given[107]);s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });
        if (s.b[1192] && s.b[1193]) {s.store_add_scaled_product_mixed_aii(406, A::add_scaled_inputs4(s.ad_value(406), 1.0, s.ad_value(152), (-1.0), s.ad_value(408), p.p37, s.ad_value(942), -1.0), 1.0, 405, 943, (-1.0));}
        if (s.b[1192] && (!s.b[1193])) {
        }
        s.b[1194] = (!param_given[108]);s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if s.b[1194] {s.store_add_scaled_inputs_product_indices(408, 406, p.p37, 942, p.p37, 405, 943, p.p37);}
        s.b[1195] = (p.p38 < 4.2);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if s.b[1195] {s.copy_ad(1095, 173);s.copy_ad(1140, 367);s.copy_ad(1141, 342);s.copy_ad(1142, 343);}
        s.b[1196] = (p.p62 == 4.0);s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if (s.b[1195] && s.b[1196]) {s.copy_ad(956, 138);s.copy_ad(958, 142);}
        s.store_scaled_voltage(819, ctx, nodes, Some(7), Some(8), p.p37);s.store_scaled_voltage(818, ctx, nodes, Some(5), Some(8), p.p37);s.store_scaled_voltage(821, ctx, nodes, Some(9), Some(8), p.p37);s.store_scaled_voltage(897, ctx, nodes, Some(3), Some(8), p.p37);s.store_scaled_voltage(1114, ctx, nodes, Some(9), Some(4), p.p37);s.store_scaled_voltage(1087, ctx, nodes, Some(11), Some(8), p.p37);s.store_scaled_voltage(1088, ctx, nodes, Some(12), Some(7), p.p37);s.store_scaled_voltage(1018, ctx, nodes, Some(10), Some(8), p.p37);s.store_sub(817, 818, 819);s.store_sub(820, 821, 819);s.store_sub(898, 897, 819);s.store_sub(1019, 1018, 819);s.b[1197] = (s.v[819] >= 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if s.b[1197] {s.store_scalar(398, 1.0);s.copy_ad(822, 819);s.copy_ad(823, 821);s.copy_ad(824, 818);s.copy_ad(900, 817);s.copy_ad(901, 897);s.copy_ad(1110, 820);s.copy_ad(1143, 282);s.store_add_scaled_product_indices(1144, 283, 1.0, 284, 430, 1.0);s.copy_ad(1145, 285);s.copy_ad(1146, 286);s.copy_ad(1147, 287);s.copy_ad(1148, 288);s.copy_ad(1149, 289);s.copy_ad(1150, 290);s.store_add_scaled_product_indices(1151, 291, 1.0, 292, 430, 1.0);s.copy_ad(1152, 293);s.copy_ad(1153, 294);s.copy_ad(1154, 295);s.copy_ad(1155, 296);s.copy_ad(1156, 297);}
        if (!s.b[1197]) {s.store_scalar(398, (-1.0));s.store_neg(822, 819);s.copy_ad(823, 820);s.copy_ad(824, 817);s.copy_ad(900, 818);s.copy_ad(901, 898);s.copy_ad(1110, 821);s.copy_ad(1143, 290);s.store_add_scaled_product_indices(1144, 291, 1.0, 292, 430, 1.0);s.copy_ad(1145, 293);s.copy_ad(1146, 294);s.copy_ad(1147, 295);s.copy_ad(1148, 296);s.copy_ad(1149, 297);s.copy_ad(1150, 282);s.store_add_scaled_product_indices(1151, 283, 1.0, 284, 430, 1.0);s.copy_ad(1152, 285);s.copy_ad(1153, 286);s.copy_ad(1154, 287);s.copy_ad(1155, 288);s.copy_ad(1156, 289);}
        s.store_sub(902, 901, 941);s.store_scalar(913, s.v[392]);s.store_add(843, 406, 942);s.b[1198] = (p.p41 == 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if s.b[1198] {s.copy_ad(418, 417);}
        if (!s.b[1198]) {s.store_scalar(418, (p.p60 * 8.85418e-12));}
        s.b[1199] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[823] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1199] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(823), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);s.store_sub(825, 823, 848);}
        if (!s.b[1199]) {s.copy_ad(825, 823);}
        s.b[1200] = ((((s.v[110] > 1e18) && (s.v[110] < 1e25)) && (s.v[1110] > s.v[843])) && (s.v[418] != 0.0));s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });
        if s.b[1200] {s.store_div_scaled_product_mixed_iia(844, 418, 110, (1000000.0 * 1.602176462e-19), A::square(s.ad_value(396)), 1.0);s.store_sqrt_offset_ad(847, A::div_scaled_inputs2(s.ad_value(1110), 2.0, s.ad_value(843), (-2.0), s.ad_value(844), 1.0), 1.0);s.store_mul_scale_offset_indices(845, 844, 847, 1.0, (-1.0));s.store_div_scaled_product_indices(846, 845, 845, 0.5, 844, 1.0);s.store_offset_sub_from_scalar_ad(850, p.p1034, s.ad_value(846), (-0.05));s.store_sqrt_square_offset(849, 850, 0.224);s.store_offset_add_scaled_inputs_indices(848, 850, (-0.5), 849, (-0.5), p.p1034);s.store_sub(1111, 1110, 848);}
        if (!s.b[1200]) {s.copy_ad(1111, 1110);}
        s.copy_ad(1125, 823);s.store_scalar(892, s.v[327]);s.b[1201] = ((p.p36 == 1.0) && (p.p14 != 0.0));s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });
        if s.b[1201] {s.store_scale(832, 409, 8.617087e-5);}
        if (!s.b[1201]) {s.copy_ad(832, 49);}
        s.store_sub(834, 940, 942);s.b[1202] = (s.v[37] == 0.0);s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });
        if s.b[1202] {s.copy_ad(1033, 824);s.copy_ad(1048, 824);}
        s.b[1203] = (p.p432 == 0.0);s.store_scalar(1203, if s.b[1203] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1203]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1036, 942, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(847, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1031, 845, 1.0, 847, 1036, 1.0);}
        if ((!s.b[1202]) && (!s.b[1203])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(942), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1036, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);s.store_add(1031, 1036, 850);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1036, 1031, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1032, 1031, 1.0, 846, 847, (-0.5));s.store_offset(844, 942, (-0.02));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1202]) {s.store_offset_sub(845, 844, 1032, (-0.005));s.store_sqrt_square_offset(846, 845, (4.0 * 0.005));s.store_add_scaled_inputs3_indices(1032, 844, 1.0, 845, (-0.5), 846, (-0.5));s.store_sub(827, 942, 1032);s.store_sqrt(828, 827);s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);s.store_sqrt(846, 864);s.store_mul(843, 131, 1032);}
        s.b[1204] = (s.v[843] >= (-0.5));s.store_scalar(1204, if s.b[1204] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1204]) {s.store_offset(844, 843, 1.0);}
        if ((!s.b[1202]) && (!s.b[1204])) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        if (!s.b[1202]) {s.store_mul3_lhs(865, 397, 846, 844);s.store_mul(843, 134, 1032);}
        s.b[1205] = (s.v[843] >= (-0.5));s.store_scalar(1205, if s.b[1205] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1205]) {s.store_offset(844, 843, 1.0);}
        if ((!s.b[1202]) && (!s.b[1205])) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        if (!s.b[1202]) {s.store_mul3_lhs(866, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 865, 1.0);}
        s.b[1206] = (s.v[843] > (-100.0));s.store_scalar(1206, if s.b[1206] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1206]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if ((!s.b[1202]) && (!s.b[1206])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if (!s.b[1202]) {s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 1032, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, 99, 1.0, 396, 1.0);}
        s.b[1207] = (s.v[847] >= (-0.5));s.store_scalar(1207, if s.b[1207] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1207]) {s.store_offset(831, 847, 1.0);}
        if ((!s.b[1202]) && (!s.b[1207])) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);}
        s.b[1208] = (s.v[378] > 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1208]) {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
        s.b[1209] = (s.v[843] < (-100.0));s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && s.b[1208]) && s.b[1209]) {s.store_scalar(845, 3.720075976e-44);}
        if (((!s.b[1202]) && s.b[1208]) && (!s.b[1209])) {s.store_exp(845, 843);}
        if ((!s.b[1202]) && s.b[1208]) {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if ((!s.b[1202]) && s.b[1208]) {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1202]) && s.b[1208]) {s.store_mul(1090, 831, 847);}
        if ((!s.b[1202]) && (!s.b[1208])) {s.store_scalar(1090, 0.0);}
        if (!s.b[1202]) {s.store_mul(63, 129, 868);s.store_mul(867, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 866, 1.0);}
        s.b[1210] = (s.v[843] > (-100.0));s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1210]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if ((!s.b[1202]) && (!s.b[1210])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (!s.b[1202]) {s.store_mul(843, 132, 845);s.store_mul(904, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1032, 1.0);s.store_add_scaled_product_mixed_aii(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(870, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 400, 1.0, 188, 1032, 1.0);}
        s.b[1211] = (s.v[846] < 0.0001);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1211]) {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        if (!s.b[1202]) {s.store_mul3_lhs(873, 846, 1141, 822);s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1032, 1.0);}
        s.b[1212] = (s.v[846] < 0.0001);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1212]) {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        if (!s.b[1202]) {s.store_mul3_lhs(1070, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1202]) {s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1037, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 873, -1.0, 1090, -1.0, 1091);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1052, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(828), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1032), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1032), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 1070, -1.0, 1090, -1.0, 1091);s.store_sub(1038, 1037, 825);s.store_mul(853, 219, 832);}
        s.b[1213] = (((s.v[1038] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1213]) {s.store_scaled_offset_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1214] = (((s.v[1038] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1213])) && s.b[1214]) {s.store_scalar(1039, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1213])) && (!s.b[1214])) {s.store_exp_ad(1039, A::div_scaled_inputs2(s.ad_value(1038), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1042, 853, A::offset(s.ad_value(1039), 1.0));s.store_sub(1040, 825, 1037);}
        s.b[1215] = (((s.v[1040] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1215]) {s.store_scaled_offset_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1216] = (((s.v[1040] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1215])) && s.b[1216]) {s.store_scalar(1041, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1215])) && (!s.b[1216])) {s.store_exp_ad(1041, A::div_scaled_inputs2(s.ad_value(1040), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1043, 853, A::offset(s.ad_value(1041), 1.0));s.store_mul_product3_indices(844, 832, 226, 376, 832, 1.0);s.store_add_scaled_product_mixed_iia(845, 1043, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);s.store_offset_div_scaled_product_indices(843, 1043, 845, 1.0, 844, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1202]) {
            s.store_add_scaled_product_mixed_iia(1034, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1202]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);s.store_add_scaled_product_indices(1035, 1034, 1.0, 843, 1042, (-1.0));}
        s.b[1217] = (p.p432 == 0.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1217]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1036, 1035, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1031, 845, 1.0, 843, 1036, 1.0);}
        if ((!s.b[1202]) && (!s.b[1217])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(1035), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1036, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);s.store_add(1031, 1036, 850);}
        s.b[1218] = (s.v[37] == 2.0);s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1218]) {s.store_offset(1030, 1031, 0.02);s.store_offset(824, 1031, 0.02);}
        if ((!s.b[1202]) && (!s.b[1218])) {s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1031), 0.02), (-0.01));s.store_sqrt_square_offset(845, 844, 0.0001);s.store_add_scaled_inputs3_offset_indices(1030, 1031, 1.0, 844, 0.5, 845, 0.5, 0.02);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1036, 1030, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1033, 1030, 1.0, 846, 847, (-0.5));s.store_sub(1060, 1052, 825);s.store_mul(853, 219, 832);}
        s.b[1219] = (((s.v[1060] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1219]) {s.store_scaled_offset_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1220] = (((s.v[1060] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1219])) && s.b[1220]) {s.store_scalar(1061, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1219])) && (!s.b[1220])) {s.store_exp_ad(1061, A::div_scaled_inputs2(s.ad_value(1060), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1064, 853, A::offset(s.ad_value(1061), 1.0));s.store_sub(1062, 825, 1052);}
        s.b[1221] = (((s.v[1062] - s.v[220]) / s.v[853]) > 100.0);s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1202]) && s.b[1221]) {s.store_scaled_offset_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1222] = (((s.v[1062] - s.v[220]) / s.v[853]) < (-100.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if (((!s.b[1202]) && (!s.b[1221])) && s.b[1222]) {s.store_scalar(1063, 3.720075976e-44);}
        if (((!s.b[1202]) && (!s.b[1221])) && (!s.b[1222])) {s.store_exp_ad(1063, A::div_scaled_inputs2(s.ad_value(1062), 1.0, s.ad_value(220), (-1.0), s.ad_value(853), 1.0));}
        if (!s.b[1202]) {s.store_mul_ln_mixed_ia(1065, 853, A::offset(s.ad_value(1063), 1.0));s.store_mul_product3_indices(844, 832, 226, 376, 832, 1.0);s.store_add_scaled_product_mixed_iia(845, 1065, 1.0, 405, A::sqrt(s.ad_value(942)), 2.0);s.store_offset_div_scaled_product_indices(843, 1065, 845, 1.0, 844, 1.0, 1.0);}
        if (!s.b[1202]) {
            s.store_add_scaled_product_mixed_iia(1049, 942, 1.0, 832, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1202]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(843, 396, 396, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(393)), (1.0 / s.v[913]), 1.0), 1.0);s.store_add_scaled_product_indices(1050, 1049, 1.0, 843, 1064, (-1.0));}
        s.b[1223] = (p.p432 == 0.0);s.store_scalar(1223, if s.b[1223] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1223]) {s.store_div_scaled_inputs_indices(843, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(844, 224, A::exp_scaled_input(s.ad_value(843), 0.5), 1.0, A::exp(s.ad_value(843)), 2.0);s.store_mul_sub_rhs(845, 844, 940, 942);s.store_div_scaled_inputs_indices(846, 344, 0.5, 393, 1.0);s.store_add_scaled_inputs4_indices(1051, 1050, 1.0, 846, (-1.0), 216, 1.0, 845, 1.0);s.store_offset_scaled(843, 393, 1.0 / (s.v[913]), 1.0);s.store_div_scaled_inputs_indices(846, 223, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(848, 222, A::exp_scaled_input(s.ad_value(846), 0.5), 1.0, A::exp(s.ad_value(846)), 2.0);s.store_div_scaled_inputs2_indices(844, 221, 1.0, 848, (-1.0), 843, 1.0);s.store_mul(845, 844, 902);s.store_div_from_scalar_offset_ad(843, 1.0, A::div_from_scalar(s.v[913], s.ad_value(393)), 1.0);s.store_add_scaled_product_indices(1047, 845, 1.0, 843, 1051, 1.0);}
        if ((!s.b[1202]) && (!s.b[1223])) {s.store_div_from_scalar_add_ad(843, 1.0, A::offset(s.ad_value(393), s.v[913]), s.ad_value(218));s.store_div_scaled_inputs_indices(844, 225, (-s.v[327]), 119, 1.0);s.store_mul_add_scaled_inputs_rhs(845, 224, A::exp_scaled_input(s.ad_value(844), 0.5), 1.0, A::exp(s.ad_value(844)), 2.0);s.store_mul_add_rhs(846, 845, 822, 217);s.store_div_scaled_inputs_indices(847, 344, 0.5, 393, 1.0);s.store_mul_ad_product_rhs_mixed_ia(848, 393, 843, A::add_scaled_inputs3(s.ad_value(1050), 1.0, s.ad_value(847), (-1.0), s.ad_value(216), 1.0));s.store_mul3_lhs(849, 218, 843, 846);s.store_add(1051, 848, 849);s.store_scaled_mul(850, 843, 902, s.v[913]);s.store_add(1047, 1051, 850);}
        s.b[1224] = (s.v[37] == 2.0);s.store_scalar(1224, if s.b[1224] { 1.0 } else { 0.0 });
        if ((!s.b[1202]) && s.b[1224]) {s.store_offset(1046, 1047, 0.02);s.store_offset(824, 1047, 0.02);}
        if ((!s.b[1202]) && (!s.b[1224])) {s.store_offset_sub_ad(844, s.ad_value(824), A::offset(s.ad_value(1047), 0.02), (-0.01));s.store_sqrt_square_offset(845, 844, 0.0001);s.store_add_scaled_inputs3_offset_indices(1046, 1047, 1.0, 844, 0.5, 845, 0.5, 0.02);}
        if (!s.b[1202]) {s.store_offset_sub(844, 1051, 1046, (-0.005));s.store_sqrt_square_offset(845, 844, 2.5e-5);s.store_scaled_add(846, 844, 845, 0.5);s.store_div_scaled_product_indices(847, 846, 393, 1.0, 344, 1.0);s.store_add_scaled_product_indices(1048, 1046, 1.0, 846, 847, (-0.5));}
        s.store_offset(843, 1033, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));s.store_scalar(843, 1.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));s.store_offset_add_scaled_inputs_indices(962, 844, (-0.5), 846, (-0.5), s.v[843]);s.store_scale(843, 942, 0.95);s.store_offset_sub(844, 843, 962, (-0.002));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);s.store_add_scaled_inputs3_indices(841, 843, 1.0, 844, (-0.5), 845, (-0.5));s.store_offset(843, 1048, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(844, 843, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(845, 843, 0.5, 844, 0.5, (-5.0));s.store_scalar(843, 1.5);s.store_offset_sub_from_scalar_ad(844, s.v[843], s.ad_value(845), (-0.002));s.store_sqrt_square_offset(846, 844, (0.008 * s.v[843]));s.store_offset_add_scaled_inputs_indices(1045, 844, (-0.5), 846, (-0.5), s.v[843]);s.store_scale(843, 942, 0.95);s.store_offset_sub(844, 843, 1045, (-0.002));s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.008);s.store_add_scaled_inputs3_indices(1044, 843, 1.0, 844, (-0.5), 845, (-0.5));s.store_sub(827, 942, 841);s.store_sqrt(828, 827);s.store_div_scaled_product_indices(864, 944, 828, 1.0, 943, 1.0);s.store_sqrt(846, 864);s.store_mul(843, 131, 841);s.b[1225] = (s.v[843] >= (-0.5));s.store_scalar(1225, if s.b[1225] { 1.0 } else { 0.0 });
        if s.b[1225] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1225]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(865, 397, 846, 844);s.store_mul(843, 134, 841);s.b[1226] = (s.v[843] >= (-0.5));s.store_scalar(1226, if s.b[1226] { 1.0 } else { 0.0 });
        if s.b[1226] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1226]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(866, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 865, 1.0);s.b[1227] = (s.v[843] > (-100.0));s.store_scalar(1227, if s.b[1227] { 1.0 } else { 0.0 });
        if s.b[1227] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        if (!s.b[1227]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(868, 844, 844, 2.0, 1.0);}
        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 864, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 841, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(868), 1.0), 1.0, 99, 1.0, 396, 1.0);s.b[1228] = (s.v[847] >= (-0.5));s.store_scalar(1228, if s.b[1228] { 1.0 } else { 0.0 });
        if s.b[1228] {s.store_offset(831, 847, 1.0);}
        if (!s.b[1228]) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(831, 843, 847, 3.0, 1.0);}
        s.b[1229] = (s.v[378] > 0.0);s.store_scalar(1229, if s.b[1229] { 1.0 } else { 0.0 });
        if s.b[1229] {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
        s.b[1230] = (s.v[843] < (-100.0));s.store_scalar(1230, if s.b[1230] { 1.0 } else { 0.0 });
        if (s.b[1229] && s.b[1230]) {s.store_scalar(845, 3.720075976e-44);}
        if (s.b[1229] && (!s.b[1230])) {s.store_exp(845, 843);}
        if s.b[1229] {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if s.b[1229] {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1229] {s.store_mul(1090, 831, 847);}
        if (!s.b[1229]) {s.store_scalar(1090, 0.0);}
        s.store_mul(63, 129, 868);s.store_mul(867, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 866, 1.0);s.b[1231] = (s.v[843] > (-100.0));s.store_scalar(1231, if s.b[1231] { 1.0 } else { 0.0 });
        if s.b[1231] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (!s.b[1231]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        s.store_mul(843, 132, 845);s.store_mul(904, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 841, 1.0);s.store_add_scaled_product_mixed_aii(903, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(870, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 400, 1.0, 188, 841, 1.0);s.b[1232] = (s.v[846] < 0.0001);s.store_scalar(1232, if s.b[1232] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1232] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        s.store_mul3_lhs(873, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_div_from_scalar(852, 2.2361, 943);s.store_add_scaled_product_right_sub(963, 828, 1.0, 852, 962, 841, (-1.0));s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(829, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(963), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(841), (-1.0)), 1.0, s.ad_value(867), (-1.0), s.ad_value(904), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(841), 1.0), s.ad_value(870), 1.0), 1.0, 903, 1.0, 873, -1.0, 1090, -1.0, 1091);s.store_sub(1053, 942, 1044);s.store_sqrt(1054, 1053);s.store_div_scaled_product_indices(1055, 944, 1054, 1.0, 943, 1.0);s.store_sqrt(846, 1055);s.store_mul(843, 131, 1044);s.b[1233] = (s.v[843] >= (-0.5));s.store_scalar(1233, if s.b[1233] { 1.0 } else { 0.0 });
        if s.b[1233] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1233]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(1056, 397, 846, 844);s.store_mul(843, 134, 1044);s.b[1234] = (s.v[843] >= (-0.5));s.store_scalar(1234, if s.b[1234] { 1.0 } else { 0.0 });
        if s.b[1234] {s.store_offset(844, 843, 1.0);}
        if (!s.b[1234]) {s.store_div_from_scalar_offset_scaled_input(847, 1.0, 843, 8.0, 3.0);s.store_mul_scale_offset_rhs(844, 847, 843, 3.0, 1.0);}
        s.store_mul3_lhs(1057, 397, 846, 844);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 1056, 1.0);s.b[1235] = (s.v[843] > (-100.0));s.store_scalar(1235, if s.b[1235] { 1.0 } else { 0.0 });
        if s.b[1235] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);}
        if (!s.b[1235]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(1058, 844, 844, 2.0, 1.0);}
        s.store_div_scaled_product_indices(845, 100, 417, 1.0, 1055, 1.0);s.store_add_scaled_value_products_indices(846, 96, 1.0, 97, 1044, 1.0, 98, 822, 1.0);s.store_div_scaled_inputs2_mixed_aii(847, A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(846), s.ad_value(1058), 1.0), 1.0, 99, 1.0, 396, 1.0);s.b[1236] = (s.v[847] >= (-0.5));s.store_scalar(1236, if s.b[1236] { 1.0 } else { 0.0 });
        if s.b[1236] {s.store_offset(1059, 847, 1.0);}
        if (!s.b[1236]) {s.store_div_from_scalar_offset_scaled_input(843, 1.0, 847, 8.0, 3.0);s.store_mul_scale_offset_rhs(1059, 843, 847, 3.0, 1.0);}
        s.b[1237] = (s.v[378] > 0.0);s.store_scalar(1237, if s.b[1237] { 1.0 } else { 0.0 });
        if s.b[1237] {s.store_mul_scale_offset_indices(843, 822, 379, -1.0, 0.0);}
        s.b[1238] = (s.v[843] < (-100.0));s.store_scalar(1238, if s.b[1238] { 1.0 } else { 0.0 });
        if (s.b[1237] && s.b[1238]) {s.store_scalar(845, 3.720075976e-44);}
        if (s.b[1237] && (!s.b[1238])) {s.store_exp(845, 843);}
        if s.b[1237] {s.store_offset_mul_offset_rhs(846, 378, 845, 1.0, s.v[892]);}
        if s.b[1237] {
            s.store_mul_mixed_ia(847, 832, {
                            if ((s.v[892] / s.v[846]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[892], s.ad_value(846)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1237] {s.store_mul(1071, 1059, 847);}
        if (!s.b[1237]) {s.store_scalar(1071, 0.0);}
        s.store_mul(63, 129, 1058);s.store_mul(1067, 63, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 1057, 1.0);s.b[1239] = (s.v[843] > (-100.0));s.store_scalar(1239, if s.b[1239] { 1.0 } else { 0.0 });
        if s.b[1239] {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1239]) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        s.store_mul(843, 132, 845);s.store_mul(1068, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs_product_indices(844, 121, 1.0, 122, 1.0 / (s.v[892]), 123, 1044, 1.0);s.store_add_scaled_product_mixed_aii(1069, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_div_scaled_product_offset_denominator_indices(1066, 415, 942, 1.0, 127, s.v[328], 1.0);s.store_add_scaled_product_indices(846, 401, 1.0, 190, 1044, 1.0);s.b[1240] = (s.v[846] < 0.0001);s.store_scalar(1240, if s.b[1240] { 1.0 } else { 0.0 });
        if s.b[1240] {s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(846), 20000.0));s.store_mul_scale_offset_indices(846, 852, 846, -1.0, 0.0002);}
        s.store_mul3_lhs(1070, 846, 1141, 822);s.store_sqrt_offset_scaled_input(1089, 377, 1.0 / (s.v[892]), 1.0);s.store_div_from_scalar(852, 2.2361, 943);s.store_add_scaled_product_right_sub(1072, 1054, 1.0, 852, 1045, 1044, (-1.0));s.store_exp_mul_scaled_lhs_indices(843, 382, 2.0, 822);s.store_div_scaled_product_offset_denominator_mixed_iai(1091, 391, A::offset(s.ad_value(843), (-1.0)), 1.0, 843, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1073, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(408), p.p37, A::add_scaled_products(s.ad_value(376), s.ad_value(1072), 1.0, s.ad_value(405), s.ad_value(943), (-1.0)), s.ad_value(1089), 1.0, s.ad_value(403), s.ad_value(1044), (-1.0)), 1.0, s.ad_value(1067), (-1.0), s.ad_value(1068), -1.0), 1.0, A::add_scaled_product(s.ad_value(125), 1.0, s.ad_value(126), s.ad_value(1044), 1.0), s.ad_value(1066), 1.0), 1.0, 1069, 1.0, 1070, -1.0, 1071, -1.0, 1091);s.b[1241] = (((p.p61 == 3.0) && (p.p36 == 1.0)) && (p.p14 != 0.0));s.store_scalar(1241, if s.b[1241] { 1.0 } else { 0.0 });
        if s.b[1241] {s.store_sqrt(1007, 944);s.store_mul(1008, 397, 1007);s.store_mul(1009, 397, 1007);s.store_div_scaled_inputs_indices(843, 130, ((-0.5) * s.v[892]), 1008, 1.0);}
        s.b[1242] = (s.v[843] > (-100.0));s.store_scalar(1242, if s.b[1242] { 1.0 } else { 0.0 });
        if (s.b[1241] && s.b[1242]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);}
        if (s.b[1241] && (!s.b[1242])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(1010, 844, 844, 2.0, 1.0);}
        if s.b[1241] {s.store_mul3_lhs(1011, 129, 1010, 834);s.store_div_scaled_inputs_indices(843, 133, ((-0.5) * (s.v[328] * s.v[892])), 1009, 1.0);}
        s.b[1243] = (s.v[843] > (-100.0));s.store_scalar(1243, if s.b[1243] { 1.0 } else { 0.0 });
        if (s.b[1241] && s.b[1243]) {s.store_exp(844, 843);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if (s.b[1241] && (!s.b[1243])) {s.store_scalar(844, 3.720075976e-44);s.store_mul_scale_offset_rhs(845, 844, 844, 2.0, 1.0);}
        if s.b[1241] {s.store_mul(843, 132, 845);s.store_mul(1012, 843, 834);s.store_sqrt_offset_scaled_input(843, 128, 1.0 / (s.v[892]), 1.0);s.store_add_scaled_inputs(844, 121, 1.0, 122, 1.0 / (s.v[892]));s.store_add_scaled_product_mixed_aii(1013, A::mul3(s.ad_value(376), A::offset(s.ad_value(843), (-1.0)), s.ad_value(943)), 1.0, 844, 430, 1.0);s.store_add_mixed_ai(1014, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(408), p.p37, s.ad_value(1011), (-1.0), s.ad_value(1012), -1.0), 1.0, s.ad_value(125), s.ad_value(1066), 1.0), 1013);}
        if (!s.b[1241]) {s.store_scalar(1014, 0.0);}
        s.store_sub(830, 825, 829);s.store_mul(853, 831, 832);s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);
    }
}
