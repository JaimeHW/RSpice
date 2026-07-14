#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
    ) {
        let mut t1c: usize = 0;
        while {
            let t3: f64 = (s.v[879] - s.v[880]);let t4: f64 = (t3).abs();let te: f64 = if t3 >= 0.0 { s.dn[879][0] } else { (-s.dn[879][0]) };let tf: f64 = if t3 >= 0.0 { s.dn[879][1] } else { (-s.dn[879][1]) };let t13: f64 = if t3 >= 0.0 { s.dn[879][2] } else { (-s.dn[879][2]) };let t14: f64 = if t3 >= 0.0 { s.dn[879][3] } else { (-s.dn[879][3]) };let t15: f64 = if t3 >= 0.0 { s.dn[879][4] } else { (-s.dn[879][4]) };let t16: f64 = if t3 >= 0.0 { s.dn[879][5] } else { (-s.dn[879][5]) };let t17: f64 = if t3 >= 0.0 { s.dn[879][6] } else { (-s.dn[879][6]) };let t18: f64 = if t3 >= 0.0 { s.dn[879][7] } else { (-s.dn[879][7]) };let t19: f64 = if t3 >= 0.0 { s.dn[879][8] } else { (-s.dn[879][8]) };let t1a: f64 = if t3 >= 0.0 { s.dn[879][9] } else { (-s.dn[879][9]) };let t10: f64 = if t3 >= 0.0 { s.dn[879][10] } else { (-s.dn[879][10]) };let t11: f64 = if t3 >= 0.0 { s.dn[879][11] } else { (-s.dn[879][11]) };let t12: f64 = if t3 >= 0.0 { s.dn[879][12] } else { (-s.dn[879][12]) };let t5: f64 = if t3 >= 0.0 { s.db[879][0] } else { (-s.db[879][0]) };let t6: f64 = if t3 >= 0.0 { s.db[879][1] } else { (-s.db[879][1]) };let t7: f64 = if t3 >= 0.0 { s.db[879][2] } else { (-s.db[879][2]) };let t8: f64 = if t3 >= 0.0 { s.db[879][3] } else { (-s.db[879][3]) };let t9: f64 = if t3 >= 0.0 { s.db[879][4] } else { (-s.db[879][4]) };let ta: f64 = if t3 >= 0.0 { s.db[879][5] } else { (-s.db[879][5]) };let tb: f64 = if t3 >= 0.0 { s.db[879][6] } else { (-s.db[879][6]) };let tc: f64 = if t3 >= 0.0 { s.db[879][7] } else { (-s.db[879][7]) };let td: f64 = if t3 >= 0.0 { s.db[879][8] } else { (-s.db[879][8]) };let t1b: f64 = if ((!s.b[975]) && ((s.v[878] <= 4.0) && (t4 > 1e-12))) { 1.0 } else { 0.0 };
            t1b != 0.0
        } {
            t1c += 1;assert!(t1c <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (t0,) = {
    if (!s.b[975]) {
        (s.v[879],)
    } else {
        (s.v[880],)
    }
};
            s.store_scalar(880, t0);
            if (!s.b[975]) {s.store_scale(814, 879, 200000000.0);s.store_div_scaled_inputs2_indices(984, 857, 1.0, 869, 1.0, 814, 1.0);}
            if (!s.b[975]) {
                s.store_offset_ad(985, A::exp_scaled_input({
                    if (s.v[984] > 1e-38) {
                        A::ln(s.ad_value(984))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (s.v[86] * 0.7)), 1.0);
            }
            if (!s.b[975]) {s.store_div_from_scalar(881, (s.v[85] * 1.9e-9), 985);s.store_add_scaled_product_indices(879, 776, 1.0, 777, 881, (-1.0 / (s.v[74])));}
            let (t2,) = {
    if (!s.b[975]) {
        let t1: f64 = (s.v[878] + 1.0);
        (t1,)
    } else {
        (s.v[878],)
    }
};
            s.store_scalar(878, t2);
        }
        if (!s.b[975]) {s.copy_ad(92, 879);}
        s.copy_ad(812, 702);s.store_sub(813, 485, 488);s.store_mul(814, 758, 812);s.store_div_scaled_inputs_indices(818, 503, ((-0.5) * (s.v[689] * s.v[688])), 814, 1.0);s.b[986] = (s.v[818] > (-100.0));s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if s.b[986] {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if (!s.b[986]) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        s.store_mul(818, 502, 820);s.store_mul(820, 818, 813);s.store_div_scaled_inputs_indices(818, 500, ((-0.5) * s.v[688]), 814, 1.0);s.b[987] = (s.v[818] > (-100.0));s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });
        if s.b[987] {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(821, 819, 819, 2.0, 1.0);}
        if (!s.b[987]) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(821, 819, 819, 2.0, 1.0);}
        s.store_mul3_lhs(821, 499, 821, 813);s.store_div_scaled_product_offset_denominator_indices(822, 92, 488, 1.0, 497, s.v[689], 1.0);s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[688]), 1.0);s.store_add_scaled_inputs3_mixed_aii(823, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(700)), 1.0, 491, (s.v[827] - 1.0), 492, (1.0 / (s.v[688]) * (s.v[827] - 1.0)));s.store_add_mixed_ai(883, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(507), s.v[36], s.ad_value(820), (-1.0), s.ad_value(821), -1.0), 1.0, s.ad_value(495), s.ad_value(822), 1.0), 823);s.store_add_scaled_inputs_product_indices(720, 883, 1.0, 488, (-1.0), 490, 700, (-1.0));s.store_mul_scale_offset_rhs(705, 478, 498, ((1.0 / (s.v[688])) * ((1.60219e-19 * (1000000.0 * s.v[174])))), (1.60219e-19 * (1000000.0 * s.v[174])));s.store_scalar(421, ((s.v[399] * (s.v[401] + (((s.v[689] / s.v[59]) / 3.0) / s.v[400]))) / ((s.v[400] * s.v[39]) * (s.v[37] - s.v[402]))));s.b[988] = (s.v[421] > 0.0);s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
        if s.b[988] {s.store_scalar(421, (1.0 / s.v[421]));}
        if (!s.b[988]) {s.store_scalar(421, 1000.0);}
        s.b[990] = (s.v[54] < 0.001);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
        if ((s.v[67] != 0.0) && s.b[990]) {s.store_scalar(416, 1000.0);}
        if ((s.v[67] != 0.0) && (!s.b[990])) {s.store_scalar(416, (s.v[263] + (1.0 / s.v[54])));}
        s.b[991] = (s.v[55] < 0.001);s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
        if ((s.v[67] != 0.0) && s.b[991]) {s.store_scalar(415, 1000.0);}
        if ((s.v[67] != 0.0) && (!s.b[991])) {s.store_scalar(415, (s.v[263] + (1.0 / s.v[55])));}
        if (s.v[67] == 0.0) {s.store_scalar(416, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (s.v[67] == 0.0) {s.store_scalar(415, 0.0);}
        s.store_offset(424, 720, (s.v[36] * s.v[56]));s.store_scaled_sqrt_ad(721, A::div_scaled_product(s.ad_value(778), s.ad_value(831), 1.0, s.ad_value(478), (1.60219e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(819, 768, s.v[36], 766, (-1.0), 488, -1.0);s.store_scale(820, 819, 2.0);s.store_scale(821, 819, 2.5);
        if (s.v[36] == 1.0) {
            s.copy_ad(425, 820);
        } else {
            s.copy_ad(425, 821);
        }
        s.b[992] = (s.v[425] < 0.0);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        if s.b[992] {s.store_scalar(425, 0.0);}
        s.b[993] = (s.v[89] == 4.0);s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });
        if s.b[993] {s.store_mul(861, 758, 702);s.store_div_scaled_inputs_indices(818, 500, s.v[688], 861, 1.0);}
        s.b[994] = (s.v[818] < 100.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[994]) {s.store_exp(819, 818);s.store_offset(820, 819, (-1.0));s.store_square(821, 820);s.store_add_scaled_inputs(822, 821, 1.0, 819, (2.0 * 3.720075976e-44));s.store_div(875, 819, 822);}
        if (s.b[993] && (!s.b[994])) {s.store_scalar(875, (1.0 / (2.688117142e43 - 2.0)));}
        if s.b[993] {s.store_div(813, 778, 701);s.store_mul(814, 470, 813);s.store_div_scaled_inputs2_mixed_aii(883, A::add_scaled_product(s.ad_value(814), 1.0, s.ad_value(466), s.ad_value(875), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[995] = (s.v[883] >= (-0.5));s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[995]) {s.store_offset(882, 883, 1.0);}
        if (s.b[993] && (!s.b[995])) {s.store_div_from_scalar_offset_scaled_input(818, 1.0, 883, 8.0, 3.0);s.store_mul_scale_offset_rhs(882, 818, 883, 3.0, 1.0);}
        if s.b[993] {s.store_mul(818, 882, 831);s.copy_ad(819, 521);s.store_div(820, 819, 818);}
        s.b[996] = (s.v[820] < (-100.0));s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[996]) {s.store_div_scaled_inputs_indices(821, 757, 3.720075976e-44, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        s.b[997] = (s.v[820] > 100.0);s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if ((s.b[993] && (!s.b[996])) && s.b[997]) {s.store_div_scaled_inputs_indices(821, 757, 2.688117142e43, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        if ((s.b[993] && (!s.b[996])) && (!s.b[997])) {s.store_div_scaled_product_mixed_aii(821, A::exp(s.ad_value(820)), 757, 1.0, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        if s.b[993] {s.store_div_scaled_inputs_indices(426, 818, 0.6931471805599453, 822, 1.0);}
        if (!s.b[993]) {s.store_scalar(426, 0.0);}
        s.b[1050] = ((p.p35 >= 4.4) || (p.p61 != 0.0));s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });s.b[1051] = (s.v[476] < 0.01);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1050] && s.b[1051]) {s.store_scalar(476, 0.01);}
        s.b[1052] = (s.v[476] > 1.0);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if ((s.b[1050] && (!s.b[1051])) && s.b[1052]) {s.store_scalar(476, 1.0);s.store_scalar(475, 0.0);}
        s.b[1053] = (s.v[551] < 0.0);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if s.b[1053] {s.store_scalar(551, 0.0);s.store_scalar(552, 0.0);}
        s.b[1054] = ((s.v[552] < 0.001) && (s.v[552] != 0.0));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if ((!s.b[1053]) && s.b[1054]) {s.store_scalar(552, 0.0);}
        s.store_scalar(770, 0.0);s.b[1144] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_voltage(770, ctx, nodes, Some(6), None);}
        if (!s.b[1144]) {s.store_scalar(770, 0.0);}
        s.store_offset(769, 770, s.v[769]);s.store_scale(771, 769, 1.0 / (s.v[150]));s.store_offset_scaled(772, 769, 1.0 / (s.v[150]), (-1.0));s.store_scalar(1466, 0.0);s.store_scalar(1467, 0.0);s.store_scalar(1468, 0.0);s.store_scalar(1469, 0.0);s.store_scalar(1464, 0.0);s.store_scalar(1454, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1455, 0.0);s.store_scalar(1463, 0.0);s.store_scalar(1460, 0.0);s.store_scalar(1461, 0.0);s.store_scalar(1459, 0.0);s.store_scalar(1451, 0.0);s.copy_ad(1290, 552);s.copy_ad(1429, 543);s.copy_ad(1430, 544);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.copy_ad(1431, 541);s.copy_ad(1432, 542);s.b[1492] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });s.b[1493] = (s.v[68] == 0.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1493]) {s.store_scale(1168, 769, 8.617087e-5);s.store_offset(1179, 769, 1108.0);s.store_square(1184, 769);s.store_sub_from_scalar_ad(1247, 1.16, A::div_scaled_inputs(s.ad_value(1184), 0.000702, s.ad_value(1179), 1.0));s.store_scalar(1181, 0.00019230584);s.store_sqrt(1184, 769);s.store_mul3_affine_lhs(1182, 769, 1184, 14500000000.0, 0.0, 1181);s.store_sub_from_scalar_ad(1185, 21.5565981, A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0));}
        s.b[1494] = (s.v[1185] > (-100.0));s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if ((s.b[1492] && s.b[1493]) && s.b[1494]) {s.store_exp(1183, 1185);}
        if ((s.b[1492] && s.b[1493]) && (!s.b[1494])) {s.store_scalar(1183, (((-100.0)) as f64).exp());}
        if (s.b[1492] && s.b[1493]) {s.store_mul(1246, 1182, 1183);}
        if (s.b[1492] && s.b[1493]) {
            if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(1179, 478, 1e20, 1246, 1.0);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && s.b[1493]) {s.store_mul(1275, 1168, 1179);}
        if (s.b[1492] && (!s.b[1493])) {s.store_scalar(1435, s.v[150]);s.store_scale(1168, 769, 8.617087e-5);s.store_primal_scale(1437, 1435, 8.617087e-5);s.copy_ad(1436, 755);s.store_sub_from_scalar_ad(1247, s.v[76], A::div_scaled_product_offset_denominator(s.ad_value(769), s.ad_value(769), s.v[77], s.ad_value(769), s.v[78], 1.0));s.store_div_from_scalar_sqrt_ad(1181, 1.0, A::mul(A::square(s.ad_value(1435)), s.ad_value(1435)));s.store_sqrt(1184, 769);s.store_mul3_affine_lhs(1182, 769, 1184, s.v[75], 0.0, 1181);s.store_exp_ad(1183, A::sub(A::div_scaled_inputs(s.ad_value(1436), 1.0, s.ad_value(1437), 2.0), A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0)));s.store_mul(1246, 1182, 1183);}
        if (s.b[1492] && (!s.b[1493])) {
            if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(1179, 478, 1e20, 1246, 1.0);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && (!s.b[1493])) {s.store_mul(1275, 1168, 1179);}
        s.b[1495] = (s.v[479] > 0.0);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1495]) {
            if ((s.v[478] / s.v[479]) > 1e-38) {
                s.store_ln_div(1179, 478, 479);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && s.b[1495]) {s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));}
        if (s.b[1492] && (!s.b[1495])) {
            if (((((-s.v[478]) * s.v[479]) / s.v[1246]) / s.v[1246]) > 1e-38) {
                s.store_ln_ad(1179, A::div_scaled_product_by_product(s.ad_value(478), s.ad_value(479), -1.0, s.ad_value(1246), s.ad_value(1246), 1.0));
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && (!s.b[1495])) {s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));}
        if s.b[1492] {
            s.store_mul_scale_offset_mixed_ia(1277, 1168, {
                if ((s.v[478] / s.v[1246]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(1246)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if s.b[1492] {s.store_sqrt(1278, 1277);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
    ) {
        if s.b[1492] {s.store_mul_sqrt_mixed_ia(1279, 1278, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)));s.store_div_mixed_ai(1473, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(778), 1.60219e-19, s.ad_value(478)), (1000000.0 * 1.0 / (2.0))), 1278);s.store_sqrt_ad(1180, A::mul3(A::div_scaled_inputs(s.ad_value(778), 1.0, s.ad_value(777), 8.85418e-12), s.ad_value(776), s.ad_value(1279)));s.store_ad_value(1179, A::exp_div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));s.store_add_scaled_product_indices(1474, 1179, 1.0, 1179, 1179, 2.0);s.store_ad_value(1179, A::exp_div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));s.store_add_scaled_product_indices(1181, 1179, 1.0, 1179, 1179, 2.0);s.store_add_scaled_product_indices(1475, 562, 1.0, 561, 1181, 1.0);s.copy_ad(409, 1168);s.store_offset(1182, 771, (-1.0));s.store_mul_div_from_scalar_lhs_ad_indices(1183, 1.115, 1168, 1182);s.store_div_scaled_product_indices(1186, 619, 1183, 1.0, 661, 1.0);}
        s.b[1496] = (s.v[1186] > 100.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1496]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1497] = (s.v[1186] < (-100.0));s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1496])) && s.b[1497]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1496])) && (!s.b[1497])) {s.store_exp(1179, 1186);}
        s.b[1498] = (s.v[619] == s.v[620]);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1498]) {s.copy_ad(1180, 1179);}
        if (s.b[1492] && (!s.b[1498])) {s.store_div_scaled_product_indices(1186, 620, 1183, 1.0, 661, 1.0);}
        s.b[1499] = (s.v[1186] > 100.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1498])) && s.b[1499]) {s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1500] = (s.v[1186] < (-100.0));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && s.b[1500]) {s.store_scalar(1180, 3.720075976e-44);}
        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && (!s.b[1500])) {s.store_exp(1180, 1186);}
        if s.b[1492] {s.store_div_scaled_product_indices(1186, 621, 1183, 1.0, 663, 1.0);}
        s.b[1501] = (s.v[1186] > 100.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1501]) {s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1502] = (s.v[1186] < (-100.0));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1501])) && s.b[1502]) {s.store_scalar(1181, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1501])) && (!s.b[1502])) {s.store_exp(1181, 1186);}
        if s.b[1492] {s.store_mul(1307, 716, 1179);s.store_mul(1284, 667, 1179);s.store_mul(1282, 669, 1180);s.store_mul(1286, 671, 1181);s.store_mul(1186, 622, 1182);}
        s.b[1503] = (s.v[1186] > 100.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1503]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1504] = (s.v[1186] < (-100.0));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1503])) && s.b[1504]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1503])) && (!s.b[1504])) {s.store_exp(1179, 1186);}
        if s.b[1492] {s.store_mul(1288, 673, 1179);s.store_div_scaled_product_indices(1186, 619, 1183, 1.0, 662, 1.0);}
        s.b[1505] = (s.v[1186] > 100.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1505]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1506] = (s.v[1186] < (-100.0));s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1505])) && s.b[1506]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1505])) && (!s.b[1506])) {s.store_exp(1179, 1186);}
        s.b[1507] = (s.v[619] == s.v[623]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1507]) {s.copy_ad(1180, 1179);}
        if (s.b[1492] && (!s.b[1507])) {s.store_div_scaled_product_indices(1186, 623, 1183, 1.0, 662, 1.0);}
        s.b[1508] = (s.v[1186] > 100.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1507])) && s.b[1508]) {s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1509] = (s.v[1186] < (-100.0));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && s.b[1509]) {s.store_scalar(1180, 3.720075976e-44);}
        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && (!s.b[1509])) {s.store_exp(1180, 1186);}
        if s.b[1492] {s.store_div_scaled_product_indices(1186, 624, 1183, 1.0, 664, 1.0);}
        s.b[1510] = (s.v[1186] > 100.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1510]) {s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1511] = (s.v[1186] < (-100.0));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1510])) && s.b[1511]) {s.store_scalar(1181, 3.720075976e-44);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1492] && (!s.b[1510])) && (!s.b[1511])) {s.store_exp(1181, 1186);}
        if s.b[1492] {s.store_mul(1308, 717, 1179);s.store_mul(1285, 668, 1179);s.store_mul(1283, 670, 1180);s.store_mul(1287, 672, 1181);s.store_mul(1186, 625, 1182);}
        s.b[1512] = (s.v[1186] > 100.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1512]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1513] = (s.v[1186] < (-100.0));s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1512])) && s.b[1513]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1512])) && (!s.b[1513])) {s.store_exp(1179, 1186);}
        if s.b[1492] {s.store_mul(1289, 674, 1179);s.store_mul_pow_indices(1280, 514, 771, 515);}
        s.b[1514] = (p.p35 < 4.2);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1514]) {s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(771), s.v[252], 1.0), 1e-9);}
        if (s.b[1492] && (!s.b[1514])) {s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(1182), s.v[252], 1.0), 1e-9);}
        if s.b[1492] {s.store_scale(1186, 601, s.v[249]);s.store_div(1295, 1186, 1296);s.store_scale(1183, 414, s.v[249]);s.store_div(1294, 1183, 1296);s.store_offset(1181, 1294, 1.0);s.store_offset(1186, 1295, 1.0);s.store_div(1179, 1181, 1186);s.store_mul(1280, 1280, 1179);s.store_add_scaled_product_indices(1281, 471, 1.0, 472, 1182, (-1.0));s.store_offset_mul(1181, 250, 1294, 1.0);s.store_offset_mul(1186, 250, 1295, 1.0);s.store_div(1179, 1181, 1186);s.store_mul(1281, 1281, 1179);}
        s.b[1515] = (s.v[403] != 1.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1515]) {s.store_div_scaled_add_product_indices(1290, 551, 1.0, 555, 1182, 1.0, 529, 1.0);s.store_scalar(1429, 0.0);s.store_scalar(1430, 0.0);}
        if (s.b[1492] && (!s.b[1515])) {s.store_scalar(1290, 0.0);s.store_scale(1428, 529, s.v[39]);s.store_mul(1189, 555, 1182);s.store_add(1180, 539, 1189);s.store_offset(1181, 1189, s.v[160]);s.store_div(1429, 1180, 1428);s.store_div(1431, 1181, 1428);s.store_add(1186, 540, 1189);s.store_offset(1183, 1189, s.v[159]);s.store_div(1430, 1186, 1428);s.store_div(1432, 1183, 1428);}
        if s.b[1492] {s.store_add_scaled_product_indices(1291, 523, 1.0, 509, 1182, 1.0);s.store_add_scaled_product_indices(1292, 524, 1.0, 511, 1182, 1.0);s.store_add_scaled_product_indices(1293, 525, 1.0, 513, 1182, 1.0);}
        if (!s.b[1492]) {s.copy_ad(1275, 485);s.copy_ad(1276, 530);s.copy_ad(1277, 488);s.copy_ad(1278, 700);s.copy_ad(1279, 701);s.copy_ad(1247, 756);s.copy_ad(1473, 728);s.copy_ad(1474, 703);s.copy_ad(1475, 704);s.copy_ad(1284, 531);s.copy_ad(1285, 532);s.copy_ad(1282, 533);s.copy_ad(1283, 534);s.copy_ad(1286, 535);s.copy_ad(1287, 536);s.copy_ad(1288, 537);s.copy_ad(1289, 538);s.copy_ad(1307, 718);s.copy_ad(1308, 719);s.copy_ad(1280, 765);s.copy_ad(1281, 767);s.copy_ad(1291, 508);s.copy_ad(1292, 510);s.copy_ad(1293, 512);}
        s.b[1516] = (param_given[89] || param_given[93]);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });s.b[1517] = (!param_given[89]);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_scalar(490, 0.53);}
        s.b[1518] = (!param_given[93]);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1518]) {s.store_scalar(494, (-0.0186));}
        s.b[1524] = (!param_given[86]);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] != 0.0)) {s.store_scaled_div_from_scalar_ad(1179, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);}
        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] == 0.0)) {s.store_scalar(1179, 0.00077348);}
        if ((!s.b[1516]) && s.b[1524]) {s.store_add_scaled_product_indices(484, 1277, 1.0, 1179, 478, (-(s.v[487] * s.v[487])));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1525] = (s.v[484] > 0.0);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1525]) {s.store_neg(484, 484);}
        s.b[1526] = (s.v[486] > 0.0);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1526]) {s.store_primal_neg(486, 486);}
        s.b[1527] = (!param_given[84]);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1527]) {s.store_div_scaled_product_mixed_iai(482, 780, A::sqrt(s.ad_value(478)), 1.0, 757, 1.0);}
        s.b[1528] = (!param_given[85]);s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1528]) {s.store_div_scaled_product_mixed_iai(483, 780, A::sqrt(s.ad_value(479)), 1.0, 757, 1.0);}
        if (!s.b[1516]) {s.store_sub(1179, 482, 483);s.store_sub_mixed_ai(1180, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(484))), 1278);s.store_mul_sub_mixed_iai(1181, 1278, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), 1278);s.store_div_scaled_product_add_scaled_denominator_indices(1182, 1179, 1180, 1.0, 1181, 2.0, 486, 1.0, 1.0);s.store_add_scaled_inputs3_indices(763, 763, 1.0, 494, (-1.0), 1182, 1.0);s.store_add_scaled_product_mixed_iia(490, 483, 1.0, 763, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), (-2.0));}
        s.store_offset(1179, 628, s.v[689]);s.b[1529] = (s.v[1179] < 1e-8);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if s.b[1529] {s.store_scalar(1179, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(707, 490, A::div(s.ad_value(627), s.ad_value(1179)), 1.0, 1.0);s.b[1530] = (!param_given[108]);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });s.b[1531] = (param_given[107] || param_given[106]);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if (s.b[1530] && s.b[1531]) {s.store_add_scaled_product_mixed_aii(766, A::add_scaled_inputs4(s.ad_value(766), 1.0, s.ad_value(522), (-1.0), s.ad_value(768), s.v[36], s.ad_value(1277), -1.0), 1.0, 707, 1278, (-1.0));}
        if (s.b[1530] && (!s.b[1531])) {
        }
        s.b[1532] = (!param_given[107]);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if s.b[1532] {s.store_add_scaled_inputs_product_indices(768, 766, s.v[36], 1277, s.v[36], 707, 1278, s.v[36]);}
        s.b[1533] = (p.p35 < 4.2);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if s.b[1533] {s.copy_ad(1429, 543);s.copy_ad(1431, 541);s.copy_ad(1473, 728);s.copy_ad(1474, 703);s.copy_ad(1475, 704);}
        s.b[1534] = (s.v[89] == 4.0);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (s.b[1533] && s.b[1534]) {s.copy_ad(1291, 508);s.copy_ad(1293, 512);}
        s.store_scaled_voltage(1155, ctx, nodes, Some(7), Some(8), s.v[36]);s.store_scaled_voltage(1154, ctx, nodes, Some(5), Some(8), s.v[36]);s.store_scaled_voltage(1157, ctx, nodes, Some(9), Some(8), s.v[36]);s.store_scaled_voltage(1232, ctx, nodes, Some(3), Some(8), s.v[36]);s.store_scaled_voltage(1234, ctx, nodes, Some(5), Some(4), s.v[36]);s.store_scaled_voltage(1447, ctx, nodes, Some(9), Some(4), s.v[36]);s.store_scaled_voltage(1421, ctx, nodes, Some(11), Some(8), s.v[36]);s.store_scaled_voltage(1422, ctx, nodes, Some(12), Some(7), s.v[36]);s.store_scaled_voltage(1353, ctx, nodes, Some(10), Some(8), s.v[36]);s.store_sub(1153, 1154, 1155);s.store_sub(1156, 1157, 1155);s.store_sub(1233, 1232, 1155);s.store_sub(1354, 1353, 1155);s.b[1535] = (s.v[1155] >= 0.0);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if s.b[1535] {s.store_scalar(759, 1.0);s.copy_ad(1158, 1155);s.copy_ad(1159, 1157);s.copy_ad(1160, 1154);s.copy_ad(1235, 1153);s.copy_ad(1236, 1232);s.copy_ad(1443, 1156);s.store_scalar(1330, s.v[708]);s.store_scalar(1331, s.v[709]);s.copy_ad(1476, 645);s.copy_ad(1477, 646);s.copy_ad(1478, 647);s.copy_ad(1479, 648);s.copy_ad(1480, 649);s.copy_ad(1481, 650);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1535] {s.copy_ad(1482, 651);s.copy_ad(1483, 652);s.copy_ad(1484, 653);s.copy_ad(1485, 654);s.copy_ad(1486, 655);s.copy_ad(1487, 656);s.copy_ad(1488, 657);s.copy_ad(1489, 658);}
        if (!s.b[1535]) {s.store_scalar(759, (-1.0));s.store_neg(1158, 1155);s.copy_ad(1159, 1156);s.copy_ad(1160, 1153);s.copy_ad(1235, 1154);s.copy_ad(1236, 1233);s.copy_ad(1443, 1157);s.store_scalar(1330, s.v[709]);s.store_scalar(1331, s.v[708]);s.copy_ad(1476, 652);s.copy_ad(1477, 653);s.copy_ad(1478, 654);s.copy_ad(1479, 655);s.copy_ad(1480, 656);s.copy_ad(1481, 657);s.copy_ad(1482, 658);s.copy_ad(1483, 645);s.copy_ad(1484, 646);s.copy_ad(1485, 647);s.copy_ad(1486, 648);s.copy_ad(1487, 649);s.copy_ad(1488, 650);s.copy_ad(1489, 651);}
        s.store_sub(1237, 1236, 1276);s.store_scalar(1248, s.v[753]);s.store_add(1179, 766, 1277);s.b[1536] = (s.v[68] == 0.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if s.b[1536] {s.copy_ad(779, 778);}
        if (!s.b[1536]) {s.store_scalar(779, (s.v[87] * 8.85418e-12));}
        s.b[1537] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1159] > s.v[1179])) && (s.v[779] != 0.0));s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if s.b[1537] {s.store_div_scaled_product_mixed_iia(1180, 779, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1159), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);s.store_mul_scale_offset_indices(1181, 1180, 1183, 1.0, (-1.0));s.store_div_scaled_product_indices(1182, 1181, 1181, 0.5, 1180, 1.0);s.store_offset_sub(1186, 782, 1182, (-0.05));s.store_sqrt_square_offset(1185, 1186, 0.224);s.store_add_scaled_inputs3_indices(1184, 782, 1.0, 1186, (-0.5), 1185, (-0.5));s.store_sub(1161, 1159, 1184);}
        if (!s.b[1537]) {s.copy_ad(1161, 1159);}
        s.b[1538] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1443] > s.v[1179])) && (s.v[779] != 0.0));s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if s.b[1538] {s.store_div_scaled_product_mixed_iia(1180, 779, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1443), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);s.store_mul_scale_offset_indices(1181, 1180, 1183, 1.0, (-1.0));s.store_div_scaled_product_indices(1182, 1181, 1181, 0.5, 1180, 1.0);s.store_offset_sub(1186, 782, 1182, (-0.05));s.store_sqrt_square_offset(1185, 1186, 0.224);s.store_add_scaled_inputs3_indices(1184, 782, 1.0, 1186, (-0.5), 1185, (-0.5));s.store_sub(1444, 1443, 1184);}
        if (!s.b[1538]) {s.copy_ad(1444, 1443);}
        s.copy_ad(1458, 1159);s.store_scalar(1227, s.v[688]);s.b[1539] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if s.b[1539] {s.store_scale(1168, 769, 8.617087e-5);}
        if (!s.b[1539]) {s.copy_ad(1168, 409);}
        s.store_sub(1170, 1275, 1277);s.b[1540] = (s.v[57] == 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if s.b[1540] {s.copy_ad(1367, 1160);s.copy_ad(1382, 1160);}
        s.b[1541] = (s.v[404] == 0.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1541]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1370, 1277, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
    ) {
        if ((!s.b[1540]) && s.b[1541]) {s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1183, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1365, 1181, 1.0, 1183, 1370, 1.0);}
        if ((!s.b[1540]) && (!s.b[1541])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1277), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1370, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1365, 1370, 1186);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1370, 1365, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1366, 1365, 1.0, 1182, 1183, (-0.5));s.store_offset(1180, 1277, (-0.02));s.store_offset_sub(1181, 1180, 1366, (-0.005));s.store_sqrt_square_offset(1182, 1181, (4.0 * 0.005));s.store_add_scaled_inputs3_indices(1366, 1180, 1.0, 1181, (-0.5), 1182, (-0.5));s.store_sub(1163, 1277, 1366);s.store_sqrt(1164, 1163);s.store_div_scaled_product_indices(1199, 1279, 1164, 1.0, 1278, 1.0);s.store_sqrt(1182, 1199);s.store_mul(1179, 501, 1366);}
        s.b[1542] = (s.v[1179] >= (-0.5));s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1542]) {s.store_offset(1180, 1179, 1.0);}
        if ((!s.b[1540]) && (!s.b[1542])) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        if (!s.b[1540]) {s.store_mul3_lhs(1200, 758, 1182, 1180);s.store_mul(1179, 504, 1366);}
        s.b[1543] = (s.v[1179] >= (-0.5));s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1543]) {s.store_offset(1180, 1179, 1.0);}
        if ((!s.b[1540]) && (!s.b[1543])) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        if (!s.b[1540]) {s.store_mul3_lhs(1201, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1200, 1.0);}
        s.b[1544] = (s.v[1179] > (-100.0));s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1544]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        if ((!s.b[1540]) && (!s.b[1544])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1540]) {s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1199, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1366, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[1545] = (s.v[1183] >= (-0.5));s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1545]) {s.store_offset(1167, 1183, 1.0);}
        if ((!s.b[1540]) && (!s.b[1545])) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1167, 1179, 1183, 3.0, 1.0);}
        s.b[1546] = (s.v[739] > 0.0);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1546]) {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
        s.b[1547] = (s.v[1179] < (-100.0));s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && s.b[1546]) && s.b[1547]) {s.store_scalar(1181, 3.720075976e-44);}
        if (((!s.b[1540]) && s.b[1546]) && (!s.b[1547])) {s.store_exp(1181, 1179);}
        if ((!s.b[1540]) && s.b[1546]) {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1540]) && s.b[1546]) {s.store_mul(1424, 1167, 1183);}
        if ((!s.b[1540]) && (!s.b[1546])) {s.store_scalar(1424, 0.0);}
        if (!s.b[1540]) {s.store_mul(411, 499, 1203);s.store_mul(1202, 411, 1170);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
    ) {
        if (!s.b[1540]) {s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1201, 1.0);}
        s.b[1548] = (s.v[1179] > (-100.0));s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1548]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if ((!s.b[1540]) && (!s.b[1548])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1540]) {s.store_mul(1179, 502, 1181);s.store_mul(1239, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1366, 1.0);s.store_add_scaled_product_mixed_aii(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_div_scaled_product_offset_denominator_indices(1205, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 761, 1.0, 557, 1366, 1.0);}
        s.b[1549] = (s.v[1182] < 0.0001);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1549]) {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        if (!s.b[1540]) {s.store_mul3_lhs(1208, 1182, 1474, 1158);s.store_add_scaled_product_indices(1182, 762, 1.0, 559, 1366, 1.0);}
        s.b[1550] = (s.v[1182] < 0.0001);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1550]) {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        if (!s.b[1540]) {s.store_mul3_lhs(1404, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1371, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1208, -1.0, 1424, -1.0, 1425);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1386, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1404, -1.0, 1424, -1.0, 1425);s.store_sub(1372, 1371, 1161);s.store_mul(1189, 585, 1168);}
        s.b[1551] = (((s.v[1372] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1551]) {s.store_scaled_offset_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
    ) {
        s.b[1552] = (((s.v[1372] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1551])) && s.b[1552]) {s.store_scalar(1373, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1551])) && (!s.b[1552])) {s.store_exp_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1376, 1189, A::offset(s.ad_value(1373), 1.0));s.store_sub(1374, 1161, 1371);}
        s.b[1553] = (((s.v[1374] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1553]) {s.store_scaled_offset_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1554] = (((s.v[1374] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1553])) && s.b[1554]) {s.store_scalar(1375, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1553])) && (!s.b[1554])) {s.store_exp_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1377, 1189, A::offset(s.ad_value(1375), 1.0));s.store_mul_product3_indices(1180, 1168, 592, 737, 1168, 1.0);s.store_add_scaled_product_mixed_iia(1181, 1377, 1.0, 707, A::sqrt(s.ad_value(1277)), 2.0);s.store_offset_div_scaled_product_indices(1179, 1377, 1181, 1.0, 1180, 1.0, 1.0);}
        if (!s.b[1540]) {
            s.store_add_scaled_product_mixed_iia(1368, 1277, 1.0, 1168, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1540]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(1179, 757, 757, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0), 1.0);s.store_add_scaled_product_indices(1369, 1368, 1.0, 1179, 1376, (-1.0));}
        s.b[1555] = (s.v[404] == 0.0);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1555]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1370, 1369, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1365, 1181, 1.0, 1179, 1370, 1.0);}
        if ((!s.b[1540]) && (!s.b[1555])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1369), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1370, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1365, 1370, 1186);}
        s.b[1556] = (s.v[57] == 2.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1556]) {s.store_offset(1364, 1365, 0.02);s.store_offset(1160, 1365, 0.02);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
    ) {
        if ((!s.b[1540]) && (!s.b[1556])) {s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1365), 0.02), (-0.01));s.store_sqrt_square_offset(1181, 1180, 0.0001);s.store_add_scaled_inputs3_offset_indices(1364, 1365, 1.0, 1180, 0.5, 1181, 0.5, 0.02);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1370, 1364, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1367, 1364, 1.0, 1182, 1183, (-0.5));s.store_sub(1394, 1386, 1161);s.store_mul(1189, 585, 1168);}
        s.b[1557] = (((s.v[1394] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1557]) {s.store_scaled_offset_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1558] = (((s.v[1394] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1557])) && s.b[1558]) {s.store_scalar(1395, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1557])) && (!s.b[1558])) {s.store_exp_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1398, 1189, A::offset(s.ad_value(1395), 1.0));s.store_sub(1396, 1161, 1386);}
        s.b[1559] = (((s.v[1396] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1559]) {s.store_scaled_offset_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1560] = (((s.v[1396] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1559])) && s.b[1560]) {s.store_scalar(1397, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1559])) && (!s.b[1560])) {s.store_exp_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1399, 1189, A::offset(s.ad_value(1397), 1.0));s.store_mul_product3_indices(1180, 1168, 592, 737, 1168, 1.0);s.store_add_scaled_product_mixed_iia(1181, 1399, 1.0, 707, A::sqrt(s.ad_value(1277)), 2.0);s.store_offset_div_scaled_product_indices(1179, 1399, 1181, 1.0, 1180, 1.0, 1.0);}
        if (!s.b[1540]) {
            s.store_add_scaled_product_mixed_iia(1383, 1277, 1.0, 1168, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1540]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(1179, 757, 757, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0), 1.0);s.store_add_scaled_product_indices(1384, 1383, 1.0, 1179, 1398, (-1.0));}
        s.b[1561] = (s.v[404] == 0.0);s.store_scalar(1561, if s.b[1561] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1561]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1385, 1384, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1381, 1181, 1.0, 1179, 1385, 1.0);}
        if ((!s.b[1540]) && (!s.b[1561])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
    ) {
        if ((!s.b[1540]) && (!s.b[1561])) {s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1384), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1385, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1381, 1385, 1186);}
        s.b[1562] = (s.v[57] == 2.0);s.store_scalar(1562, if s.b[1562] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1562]) {s.store_offset(1380, 1381, 0.02);s.store_offset(1160, 1381, 0.02);}
        if ((!s.b[1540]) && (!s.b[1562])) {s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1381), 0.02), (-0.01));s.store_sqrt_square_offset(1181, 1180, 0.0001);s.store_add_scaled_inputs3_offset_indices(1380, 1381, 1.0, 1180, 0.5, 1181, 0.5, 0.02);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1385, 1380, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1382, 1380, 1.0, 1182, 1183, (-0.5));}
        s.store_offset(1179, 1367, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(1181, 1179, 0.5, 1180, 0.5, (-5.0));s.store_scalar(1179, 1.5);s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));s.store_offset_add_scaled_inputs_indices(1297, 1180, (-0.5), 1182, (-0.5), s.v[1179]);s.store_scale(1179, 1277, 0.95);s.store_offset_sub(1180, 1179, 1297, (-0.002));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.008);s.store_add_scaled_inputs3_indices(1177, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));s.store_offset(1179, 1382, ((5.0) + ((-0.001))));s.store_sqrt_square_offset(1180, 1179, (-(0.004 * (-5.0))));s.store_offset_add_scaled_inputs_indices(1181, 1179, 0.5, 1180, 0.5, (-5.0));s.store_scalar(1179, 1.5);s.store_offset_sub_from_scalar_ad(1180, s.v[1179], s.ad_value(1181), (-0.002));s.store_sqrt_square_offset(1182, 1180, (0.008 * s.v[1179]));s.store_offset_add_scaled_inputs_indices(1379, 1180, (-0.5), 1182, (-0.5), s.v[1179]);s.store_scale(1179, 1277, 0.95);s.store_offset_sub(1180, 1179, 1379, (-0.002));s.store_sqrt_add_scaled_square_input(1181, 1180, 1.0, 1179, 0.008);s.store_add_scaled_inputs3_indices(1378, 1179, 1.0, 1180, (-0.5), 1181, (-0.5));s.store_sub(1163, 1277, 1177);s.store_sqrt(1164, 1163);s.store_div_scaled_product_indices(1199, 1279, 1164, 1.0, 1278, 1.0);s.store_sqrt(1182, 1199);s.store_mul(1179, 501, 1177);s.b[1563] = (s.v[1179] >= (-0.5));s.store_scalar(1563, if s.b[1563] { 1.0 } else { 0.0 });
        if s.b[1563] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1563]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1200, 758, 1182, 1180);s.store_mul(1179, 504, 1177);s.b[1564] = (s.v[1179] >= (-0.5));s.store_scalar(1564, if s.b[1564] { 1.0 } else { 0.0 });
        if s.b[1564] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1564]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1201, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1200, 1.0);s.b[1565] = (s.v[1179] > (-100.0));s.store_scalar(1565, if s.b[1565] { 1.0 } else { 0.0 });
        if s.b[1565] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1565]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1199, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1177, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, 469, 1.0, 757, 1.0);s.b[1566] = (s.v[1183] >= (-0.5));s.store_scalar(1566, if s.b[1566] { 1.0 } else { 0.0 });
        if s.b[1566] {s.store_offset(1167, 1183, 1.0);}
        if (!s.b[1566]) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1167, 1179, 1183, 3.0, 1.0);}
        s.b[1567] = (s.v[739] > 0.0);s.store_scalar(1567, if s.b[1567] { 1.0 } else { 0.0 });
        if s.b[1567] {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
    ) {
        s.b[1568] = (s.v[1179] < (-100.0));s.store_scalar(1568, if s.b[1568] { 1.0 } else { 0.0 });
        if (s.b[1567] && s.b[1568]) {s.store_scalar(1181, 3.720075976e-44);}
        if (s.b[1567] && (!s.b[1568])) {s.store_exp(1181, 1179);}
        if s.b[1567] {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if s.b[1567] {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1567] {s.store_mul(1424, 1167, 1183);}
        if (!s.b[1567]) {s.store_scalar(1424, 0.0);}
        s.store_mul(411, 499, 1203);s.store_mul(1202, 411, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1201, 1.0);s.b[1569] = (s.v[1179] > (-100.0));s.store_scalar(1569, if s.b[1569] { 1.0 } else { 0.0 });
        if s.b[1569] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1569]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        s.store_mul(1179, 502, 1181);s.store_mul(1239, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1177, 1.0);s.store_add_scaled_product_mixed_aii(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_div_scaled_product_offset_denominator_indices(1205, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 761, 1.0, 557, 1177, 1.0);s.b[1570] = (s.v[1182] < 0.0001);s.store_scalar(1570, if s.b[1570] { 1.0 } else { 0.0 });
        if s.b[1570] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        s.store_mul3_lhs(1208, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_div_from_scalar(1188, 2.2361, 1278);s.store_add_scaled_product_right_sub(1298, 1164, 1.0, 1188, 1297, 1177, (-1.0));s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1165, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1298), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1177), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1177), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1208, -1.0, 1424, -1.0, 1425);s.store_sub(1387, 1277, 1378);s.store_sqrt(1388, 1387);s.store_div_scaled_product_indices(1389, 1279, 1388, 1.0, 1278, 1.0);s.store_sqrt(1182, 1389);s.store_mul(1179, 501, 1378);s.b[1571] = (s.v[1179] >= (-0.5));s.store_scalar(1571, if s.b[1571] { 1.0 } else { 0.0 });
        if s.b[1571] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1571]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1390, 758, 1182, 1180);s.store_mul(1179, 504, 1378);s.b[1572] = (s.v[1179] >= (-0.5));s.store_scalar(1572, if s.b[1572] { 1.0 } else { 0.0 });
        if s.b[1572] {s.store_offset(1180, 1179, 1.0);}
        if (!s.b[1572]) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        s.store_mul3_lhs(1391, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1390, 1.0);s.b[1573] = (s.v[1179] > (-100.0));s.store_scalar(1573, if s.b[1573] { 1.0 } else { 0.0 });
        if s.b[1573] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1573]) {s.store_scalar(1180, 3.720075976e-44);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (!s.b[1573]) {s.store_mul_scale_offset_rhs(1392, 1180, 1180, 2.0, 1.0);}
        s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1389, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1378, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1392), 1.0), 1.0, 469, 1.0, 757, 1.0);s.b[1574] = (s.v[1183] >= (-0.5));s.store_scalar(1574, if s.b[1574] { 1.0 } else { 0.0 });
        if s.b[1574] {s.store_offset(1393, 1183, 1.0);}
        if (!s.b[1574]) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1393, 1179, 1183, 3.0, 1.0);}
        s.b[1575] = (s.v[739] > 0.0);s.store_scalar(1575, if s.b[1575] { 1.0 } else { 0.0 });
        if s.b[1575] {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
        s.b[1576] = (s.v[1179] < (-100.0));s.store_scalar(1576, if s.b[1576] { 1.0 } else { 0.0 });
        if (s.b[1575] && s.b[1576]) {s.store_scalar(1181, 3.720075976e-44);}
        if (s.b[1575] && (!s.b[1576])) {s.store_exp(1181, 1179);}
        if s.b[1575] {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if s.b[1575] {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if s.b[1575] {s.store_mul(1405, 1393, 1183);}
        if (!s.b[1575]) {s.store_scalar(1405, 0.0);}
        s.store_mul(411, 499, 1392);s.store_mul(1401, 411, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1391, 1.0);s.b[1577] = (s.v[1179] > (-100.0));s.store_scalar(1577, if s.b[1577] { 1.0 } else { 0.0 });
        if s.b[1577] {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1577]) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        s.store_mul(1179, 502, 1181);s.store_mul(1402, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1378, 1.0);s.store_add_scaled_product_mixed_aii(1403, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_div_scaled_product_offset_denominator_indices(1400, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 762, 1.0, 559, 1378, 1.0);s.b[1578] = (s.v[1182] < 0.0001);s.store_scalar(1578, if s.b[1578] { 1.0 } else { 0.0 });
        if s.b[1578] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        s.store_mul3_lhs(1404, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_div_from_scalar(1188, 2.2361, 1278);s.store_add_scaled_product_right_sub(1406, 1388, 1.0, 1188, 1379, 1378, (-1.0));s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1407, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1406), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1378), (-1.0)), 1.0, s.ad_value(1401), (-1.0), s.ad_value(1402), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1378), 1.0), s.ad_value(1400), 1.0), 1.0, 1403, 1.0, 1404, -1.0, 1405, -1.0, 1425);s.b[1579] = (((s.v[88] == 3.0) && (p.p33 == 1.0)) && (p.p16 != 0.0));s.store_scalar(1579, if s.b[1579] { 1.0 } else { 0.0 });
        if s.b[1579] {s.store_sqrt(1342, 1279);s.store_mul(1343, 758, 1342);s.store_mul(1344, 758, 1342);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1343, 1.0);}
        s.b[1580] = (s.v[1179] > (-100.0));s.store_scalar(1580, if s.b[1580] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
    ) {
        if (s.b[1579] && s.b[1580]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);}
        if (s.b[1579] && (!s.b[1580])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1345, 1180, 1180, 2.0, 1.0);}
        if s.b[1579] {s.store_mul3_lhs(1346, 499, 1345, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1344, 1.0);}
        s.b[1581] = (s.v[1179] > (-100.0));s.store_scalar(1581, if s.b[1581] { 1.0 } else { 0.0 });
        if (s.b[1579] && s.b[1581]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (s.b[1579] && (!s.b[1581])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if s.b[1579] {s.store_mul(1179, 502, 1181);s.store_mul(1347, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs(1180, 491, 1.0, 492, 1.0 / (s.v[1227]));s.store_add_scaled_product_mixed_aii(1348, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_add_mixed_ai(1349, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(768), s.v[36], s.ad_value(1346), (-1.0), s.ad_value(1347), -1.0), 1.0, s.ad_value(495), s.ad_value(1400), 1.0), 1348);}
        if (!s.b[1579]) {s.store_scalar(1349, 0.0);}
        s.store_sub(1166, 1161, 1165);s.store_mul(1189, 1167, 1168);s.store_div_scaled_product_indices(1145, 745, 1166, 1.0, 1189, 1.0);s.store_div_scaled_inputs2_mixed_iai(1169, 521, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(1166)), (-1.0), 1189, 1.0);s.b[1582] = (s.v[1145] > 100.0);s.store_scalar(1582, if s.b[1582] { 1.0 } else { 0.0 });
        if s.b[1582] {s.copy_ad(1210, 1166);s.store_scalar(1146, 0.0);}
        s.b[1583] = (s.v[1169] > 100.0);s.store_scalar(1583, if s.b[1583] { 1.0 } else { 0.0 });
        if ((!s.b[1582]) && s.b[1583]) {s.store_div_scaled_inputs2_by_product_indices(1179, 1166, 1.0, 521, (-1.0), 1167, 1168, 1.0);s.store_exp(1146, 1179);s.store_mul_div_scaled_product_indices(1210, 1146, 1168, 1473, 1.0, 757, 1.0);}
        if ((!s.b[1582]) && (!s.b[1583])) {s.store_exp(1146, 1145);s.store_mul_ln_mixed_ia(1180, 1189, A::offset(s.ad_value(1146), 1.0));s.store_mul3_ad(1192, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(1168), s.ad_value(1473)), 1.0), A::exp(s.ad_value(1169)), A::sub_from_scalar(1.0, s.ad_value(745)));s.store_sub_mixed_ia(1181, 745, A::div_scaled_product(s.ad_value(1189), s.ad_value(1192), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));s.store_div(1210, 1180, 1181);}
        s.store_add_scaled_inputs(1225, 1210, 1.0, 1168, 2.0);s.copy_ad(451, 1210);s.b[1584] = (s.v[746] <= 0.0);s.store_scalar(1584, if s.b[1584] { 1.0 } else { 0.0 });
        if s.b[1584] {s.store_scalar(1426, 1.0);}
        if (!s.b[1584]) {s.store_div_scaled_inputs_indices(1188, 746, ((s.v[1227]) as f64).sqrt(), 1225, 1.0);s.store_div_from_scalar_offset_input(1426, 1.0, 1188, 1.0);}
        s.store_sub(1188, 1164, 1278);s.store_sub_from_scalar_ad(1228, s.v[689], A::add_scaled_products(s.ad_value(566), s.ad_value(1210), (2.0 - s.v[58]), s.ad_value(567), s.ad_value(1188), (2.0 - s.v[58])));s.b[1585] = (s.v[1228] < 2e-8);s.store_scalar(1585, if s.b[1585] { 1.0 } else { 0.0 });
        if s.b[1585] {s.store_div_from_scalar_sub_from_scalar_ad(1179, 1.0, 6e-8, A::scale(s.ad_value(1228), 2.0));s.store_mul_scale_offset_indices(1228, 1179, 1228, -(2e-8), (4e-8) * (2e-8));}
        s.b[1586] = (s.v[403] == 1.0);s.store_scalar(1586, if s.b[1586] { 1.0 } else { 0.0 });
        if s.b[1586] {s.store_scalar(1222, 0.0);}
        if (!s.b[1586]) {s.store_add_scaled_products_indices(1179, 553, 1210, 1.0, 554, 1188, 1.0);}
        s.b[1587] = (s.v[1179] >= (-0.9));s.store_scalar(1587, if s.b[1587] { 1.0 } else { 0.0 });
        if ((!s.b[1586]) && s.b[1587]) {s.store_mul_scale_offset_indices(1222, 1290, 1179, 1.0, 1.0);}
        if ((!s.b[1586]) && (!s.b[1587])) {s.store_div_from_scalar_offset_scaled_input(1180, 1.0, 1179, 20.0, 17.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
    ) {
        if ((!s.b[1586]) && (!s.b[1587])) {s.store_mul_ad_product_lhs_mixed_ia(1222, 1290, A::offset(s.ad_value(1179), 0.8), 1180);}
        s.b[1588] = (s.v[403] == 2.0);s.store_scalar(1588, if s.b[1588] { 1.0 } else { 0.0 });
        if s.b[1588] {s.store_add_scaled_inputs3_indices(1222, 423, 1.0, 1222, 1.0, 422, 1.0);}
        s.b[1589] = (s.v[473] == 0.0);s.store_scalar(1589, if s.b[1589] { 1.0 } else { 0.0 });
        if s.b[1589] {s.store_scalar(1195, 1.0);s.store_scalar(1196, 1.0);}
        if (!s.b[1589]) {s.store_mul(1189, 477, 1297);}
        s.b[1590] = (s.v[1189] >= (-0.5));s.store_scalar(1590, if s.b[1590] { 1.0 } else { 0.0 });
        if ((!s.b[1589]) && s.b[1590]) {s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);}
        if ((!s.b[1589]) && (!s.b[1590])) {s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);}
        if (!s.b[1589]) {s.store_add(1189, 1277, 629);s.store_div_scaled_product_indices(1299, 1297, 1190, 1.0, 1189, 1.0);}
        s.b[1591] = (s.v[1299] < 0.5);s.store_scalar(1591, if s.b[1591] { 1.0 } else { 0.0 });
        if ((!s.b[1589]) && s.b[1591]) {s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));}
        if ((!s.b[1589]) && (!s.b[1591])) {s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);}
        if (!s.b[1589]) {s.store_div_scaled_product_mixed_iia(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);s.store_mul(1180, 1189, 1300);s.store_sqrt_mul(1188, 608, 1199);s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);s.store_div_from_scalar(1184, s.v[1227], 1204);s.store_mul(1205, 473, 1184);s.store_offset(1206, 569, s.v[689]);s.store_div(1207, 568, 1206);s.store_add(1181, 1205, 1207);s.store_square(1185, 1184);s.store_mul(1186, 1184, 1185);s.store_offset_mul(1196, 1180, 1181, 1.0);s.store_mul3_lhs(1187, 474, 473, 1186);s.store_mul_scale_offset_indices(1214, 1187, 1180, -1.0, 0.0);s.store_add_scaled_product_indices(1195, 1196, 1.0, 1214, 1210, 1.0);}
        s.b[1592] = (s.v[1196] < 0.01);s.store_scalar(1592, if s.b[1592] { 1.0 } else { 0.0 });
        if s.b[1592] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1196), 200.0));s.store_mul_scale_offset_indices(1196, 1188, 1196, -1.0, 0.02);}
        s.b[1593] = (s.v[1195] < 0.01);s.store_scalar(1593, if s.b[1593] { 1.0 } else { 0.0 });
        if s.b[1593] {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1195), 200.0));s.store_mul_scale_offset_indices(1195, 1188, 1195, -1.0, 0.02);}
        s.b[1594] = (s.v[473] == 0.0);s.store_scalar(1594, if s.b[1594] { 1.0 } else { 0.0 });
        if s.b[1594] {s.store_scalar(1408, 1.0);}
        if (!s.b[1594]) {s.store_mul(1189, 477, 1379);}
        s.b[1595] = (s.v[1189] >= (-0.5));s.store_scalar(1595, if s.b[1595] { 1.0 } else { 0.0 });
        if ((!s.b[1594]) && s.b[1595]) {s.store_div_from_scalar_offset_input(1190, 1.0, 1189, 1.0);}
        if ((!s.b[1594]) && (!s.b[1595])) {s.store_scalar(1191, ((-1.0) / ((1.0 - 0.5) * (1.0 - 0.5))));s.store_offset_scaled(1299, 1191, 0.5, (1.0 / (1.0 - 0.5)));s.store_add_scaled_product_indices(1190, 1299, 1.0, 1191, 1189, 1.0);}
        if (!s.b[1594]) {s.store_add(1189, 1277, 629);s.store_div_scaled_product_indices(1299, 1379, 1190, 1.0, 1189, 1.0);}
        s.b[1596] = (s.v[1299] < 0.5);s.store_scalar(1596, if s.b[1596] { 1.0 } else { 0.0 });
        if ((!s.b[1594]) && s.b[1596]) {s.store_div_from_scalar_sqrt_ad(1300, 1.0, A::sub_from_scalar(1.0, s.ad_value(1299)));}
        if ((!s.b[1594]) && (!s.b[1596])) {s.store_scalar(1190, (1.0 / ((2.0 * (1.0 - 0.5)) * (((1.0 - 0.5)) as f64).sqrt())));s.store_sub_from_scalar_scaled_input(1191, (1.0 / (((1.0 - 0.5)) as f64).sqrt()), 1190, 0.5);s.store_add_scaled_product_indices(1300, 1191, 1.0, 1190, 1299, 1.0);}
        if (!s.b[1594]) {s.store_div_scaled_product_mixed_iia(1189, 737, 1423, 0.5, A::sqrt(A::add(s.ad_value(1277), s.ad_value(629))), 1.0);s.store_mul(1180, 1189, 1300);s.store_sqrt_mul(1188, 608, 1389);s.store_offset_scaled(1204, 1188, 2.0, s.v[1227]);s.store_div_from_scalar(1184, s.v[1227], 1204);s.store_mul(1205, 473, 1184);s.store_offset(1206, 569, s.v[689]);s.store_div(1207, 568, 1206);s.store_add(1181, 1205, 1207);s.store_square(1185, 1184);s.store_mul(1186, 1184, 1185);s.store_offset_mul(1408, 1180, 1181, 1.0);}
        s.b[1597] = (s.v[1408] < 0.01);s.store_scalar(1597, if s.b[1597] { 1.0 } else { 0.0 });
    }
}
