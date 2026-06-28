#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1259] = (s.v[1074] < 0.01);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if s.b[1259] {
            s.store_div_from_scalar_sub_from_scalar_ad(852, 1.0, 3.0, A::scale(s.ad_value(1074), 200.0));
            s.store_mul_sub_from_scalar_lhs(1074, 0.02, 1074, 852);
        }

        if (p.p41 != 0.0) {
            s.store_scaled_offset_ad(965, A::sub_from_scalar((p.p52 - p.p53), A::scale(s.ad_value(912), 0.5)), 0.45, (2.0 * p.p37));
            s.store_scalar(1109, ((p.p45 * p.p47) / 3.9));
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        if (p.p41 == 0.0) {
            s.store_scalar(965, 0.0);
            s.store_scalar(1109, p.p66);
            s.store_scaled_sub(856, 897, 941, p.p123);
        }

        s.b[1260] = (p.p62 == 1.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if s.b[1260] {
            s.store_add_scaled_inputs4_indices(843, 875, 1.0, 829, 1.0, 829, 1.0, 965, -1.0);
            s.store_add_scaled_product_indices(845, 956, 1.0, 958, 841, 1.0);
            s.store_div(846, 843, 1109);
            s.store_mul_ad_rhs(848, 846, A::add_scaled_inputs_product(s.ad_value(845), 1.0, s.ad_value(856), 1.0, s.ad_value(957), s.ad_value(846), 1.0));
        }

        s.b[1261] = (p.p62 == 2.0);
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if ((!s.b[1260]) && s.b[1261]) {
            s.store_mul_ad(848, A::div_scaled_inputs2(s.ad_value(875), 1.0, s.ad_value(965), (-1.0), s.ad_value(415), 1.0), A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(956), 1.0, s.ad_value(958), s.ad_value(841), 1.0), 1.0, s.ad_value(856), 1.0, A::div_scaled_product(s.ad_value(957), A::sub(s.ad_value(875), s.ad_value(965)), 1.0, s.ad_value(415), 1.0), 1.0));
        }

        s.b[1262] = (p.p62 == 3.0);
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        if (((!s.b[1260]) && (!s.b[1261])) && s.b[1262]) {
            s.store_add_scaled_inputs4_indices(843, 875, 1.0, 829, 1.0, 829, 1.0, 965, -1.0);
            s.store_offset_mul(845, 958, 841, 1.0);
            s.store_div(846, 843, 1109);
            s.store_mul_add_scaled_product_rhs(847, 846, s.ad_value(956), 1.0, s.ad_value(957), s.ad_value(846), 1.0);
            s.store_mul(848, 847, 845);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_scale_ad(843, A::div_scaled_inputs2(s.ad_value(875), 1e-8, s.ad_value(68), 1e-8, s.ad_value(415), 1.0), 0.16666666666666666);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(844, A::mul(s.ad_value(148), {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_add_scaled_product_indices(845, 956, 1.0, 958, 841, 1.0);
            s.store_mul_pow_ad_rhs(1157, 149, s.ad_value(411), s.ad_value(150));
            s.store_mul_pow_ad_rhs(1158, 146, s.ad_value(411), s.ad_value(147));
            s.copy_ad(1108, 69);
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_exp_ad(853, A::mul(s.ad_value(1157), {
                if ((1.0 + (s.v[875] / s.v[1108])) > 1e-38) {
                    A::ln(A::offset(A::div(s.ad_value(875), s.ad_value(1108)), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }));
        }

        if (((!s.b[1260]) && (!s.b[1261])) && (!s.b[1262])) {
            s.store_div(854, 1158, 853);
            s.store_add_scaled_product_indices(848, 854, 1.0, 844, 845, 1.0);
        }

        s.b[1263] = (s.v[848] >= (-0.8));
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if s.b[1263] {
            s.store_offset(936, 848, 1.0);
        }

        if (!s.b[1263]) {
            s.store_div_from_scalar_offset_scaled_input(852, 1.0, 848, 10.0, 7.0);
            s.store_mul_offset_lhs(936, 848, 0.6, 852);
        }

        s.store_div_scaled_inputs3_indices(835, 945, 1.0, 897, p.p124, 941, (-p.p124), 936, 1.0);

        s.store_scale(835, 835, p.p31);

        s.copy_ad(75, 835);

        s.store_mul3_lhs(888, 893, 946, 396);

        s.store_mul(889, 888, 887);

        s.store_div_scaled_inputs_indices(836, 946, 2.0, 835, 1.0);

        s.store_scale(838, 836, s.v[892]);

        s.b[1264] = (s.v[105] == 0.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if s.b[1264] {
            s.copy_ad(874, 106);
        }

        s.b[1265] = (s.v[105] > 0.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if ((!s.b[1264]) && s.b[1265]) {
            s.store_sub_from_scalar(843, 1.0, 106);
            s.store_offset_add_scaled_product(844, s.ad_value(843), 1.0, s.ad_value(105), s.ad_value(875), (-1.0), (-0.0001));
            s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 843, 0.0004);
            s.store_add_scaled_inputs4_indices(874, 106, 1.0, 843, 1.0, 844, (-0.5), 845, (-0.5));
        }

        if ((!s.b[1264]) && (!s.b[1265])) {
            s.store_offset_add_scaled_product(844, s.ad_value(106), 1.0, s.ad_value(105), s.ad_value(875), 1.0, (-0.0001));
            s.store_sqrt_add_scaled_square_input(845, 844, 1.0, 106, 0.0004);
            s.store_scaled_add(874, 844, 845, 0.5);
        }

        s.store_div(76, 860, 890);

        s.b[1266] = ((s.v[887] == 0.0) && (s.v[874] == 1.0));
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if s.b[1266] {
            s.store_div_from_scalar_ad(843, 1.0, A::add_scaled_product(s.ad_value(890), 1.0, s.ad_value(860), s.ad_value(838), 1.0));
            s.store_mul(846, 838, 890);
            s.store_mul(837, 846, 843);
        }

        if (!s.b[1266]) {
            s.store_mul(852, 860, 889);
            s.store_mul(850, 890, 852);
            s.store_mul(849, 890, 889);
            s.store_mul_add_scaled_inputs_rhs(843, 860, A::offset(s.ad_value(852), (-1.0)), 2.0, A::div_from_scalar(1.0, s.ad_value(874)), 2.0);
            s.store_add_scaled_ad_lhs(844, A::add_scaled_products(s.ad_value(890), A::offset(A::div_from_scalar(2.0, s.ad_value(874)), (-1.0)), 1.0, s.ad_value(860), s.ad_value(838), 1.0), 850, 3.0);
            s.store_mul_add_scaled_inputs_rhs(845, 890, s.ad_value(838), 1.0, s.ad_value(849), 2.0);
            s.store_sqrt_add_scaled_square_product(846, 844, 1.0, 843, 845, (-2.0));
            s.store_div_scaled_inputs2_indices(837, 844, 1.0, 846, (-1.0), 843, 1.0);
        }

        s.store_add_scaled_inputs3_indices(844, 837, 1.0, 822, (-1.0), 180, -1.0);

        s.store_sqrt_add_scaled_square_product(845, 844, 1.0, 180, 837, 4.0);

        s.store_add_scaled_inputs3_indices(876, 837, 1.0, 844, (-0.5), 845, (-0.5));

        s.b[1267] = (s.v[876] > s.v[822]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if s.b[1267] {
            s.copy_ad(876, 822);
        }

        s.store_sub(878, 822, 876);

        s.copy_ad(77, 876);

        s.store_sub_from_scalar_ad(872, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(837), 0.5, s.ad_value(890), 1.0));

        s.store_mul(852, 889, 875);

        s.store_add_scaled_inputs_product_indices(843, 838, 1.0, 837, 1.0, 852, 872, 2.0);

        s.store_mul(852, 889, 860);

        s.store_add_offset_ad_lhs(844, A::div_from_scalar(2.0, s.ad_value(874)), (-1.0), 852);

        s.store_div(840, 843, 844);

        s.b[1268] = ((s.v[191] > 0.0) && (s.v[878] > 1e-10));
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if s.b[1268] {
            s.store_div_from_scalar_ad(843, 1.0, A::mul3(s.ad_value(191), s.ad_value(860), s.ad_value(119)));
            s.store_div(845, 875, 838);
            s.store_scaled_add(844, 860, 845, s.v[892]);
            s.store_mul(852, 843, 844);
            s.store_mul(862, 852, 878);
        }

        if (!s.b[1268]) {
            s.store_scalar(862, 2.688117142e43);
        }

        s.b[1269] = (s.v[1142] > 0.0);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if s.b[1269] {
            s.store_mul(851, 860, 837);
            s.store_mul(843, 890, 851);
            s.store_add(844, 890, 851);
            s.copy_ad(845, 1142);
            s.store_div_scaled_inputs2_mixed_iai(863, 890, 1.0, A::div(s.ad_value(843), s.ad_value(844)), (-1.0), 845, 1.0);
            s.store_mul(850, 194, 841);
        }

        s.b[1270] = (s.v[850] >= (-0.9));
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1269] && s.b[1270]) {
            s.store_div_from_scalar_offset_input(846, 1.0, 850, 1.0);
            s.store_mul(863, 863, 846);
        }

        if (s.b[1269] && (!s.b[1270])) {
            s.store_div_from_scalar_offset_input(847, 1.0, 850, 0.8);
            s.store_mul_scale_offset_rhs(846, 847, 850, 20.0, 17.0);
            s.store_mul(863, 863, 846);
        }

        if (!s.b[1269]) {
            s.store_scalar(863, 2.688117142e43);
        }

        s.store_mul(843, 387, 822);

        s.b[1271] = (s.v[843] > 100.0);
        s.v[1271] = if s.b[1271] { 1.0 } else { 0.0 };

        if s.b[1271] {
            s.store_scalar(844, 2.688117142e43);
        }

        if (!s.b[1271]) {
            s.store_exp(844, 843);
        }

        s.b[1272] = (s.v[386] > 3.720075976e-44);
        s.v[1272] = if s.b[1272] { 1.0 } else { 0.0 };

        if s.b[1272] {
            s.store_scalar(845, (1.0 + (p.p283 * s.v[892])));
            s.store_div_scaled_offset_numerator(1093, A::mul(s.ad_value(845), s.ad_value(844)), 1.0, 1.0, s.ad_value(386), 1.0);
            s.store_mul(1093, 1093, 1092);
        }

        if (!s.b[1272]) {
            s.store_scalar(1093, 2.688117142e43);
        }

        s.store_div(851, 195, 838);

        s.store_mul(852, 851, 875);

        s.b[1273] = (s.v[852] > (-0.9));
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if s.b[1273] {
            s.store_offset(843, 852, 1.0);
        }

        if (!s.b[1273]) {
            s.store_div_from_scalar_offset_scaled_input(844, 1.0, 852, 20.0, 17.0);
            s.store_mul_offset_lhs(843, 852, 0.8, 844);
        }

        s.store_add(871, 862, 863);

        s.store_div_scaled_product_indices(844, 862, 863, 1.0, 871, 1.0);

        s.store_add(871, 844, 1093);

        s.store_div_scaled_product_indices(845, 844, 1093, 1.0, 871, 1.0);

        s.store_add_scaled_product_indices(839, 840, 1.0, 843, 845, 1.0);

        s.store_scaled_mul(886, 396, 893, 1.0 / (s.v[892]));

        s.store_mul(880, 835, 886);

        s.store_sub_from_scalar_ad(843, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0));

        s.store_mul(882, 875, 843);

        s.store_div(852, 876, 838);

        s.store_offset(883, 852, 1.0);

        s.store_div_scaled_product_indices(881, 880, 882, 1.0, 883, 1.0);

        s.store_offset_mul(843, 881, 887, 1.0);

        s.store_div(852, 876, 843);

        s.store_mul(884, 881, 852);

        s.store_div(1085, 881, 843);

        s.store_div(852, 878, 839);

        s.store_offset(843, 852, 1.0);

        s.store_scaled_mul(885, 884, 843, 1.0 / (p.p23));

        s.store_scale(885, 885, p.p30);

        s.store_scaled_mul(78, 1085, 843, 1.0 / (p.p23));

        s.b[1274] = (s.v[78] < 1e-9);
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if s.b[1274] {
            s.store_scalar(78, 1e-9);
        }

        s.store_scaled_mul(1086, 1085, 843, 1.0 / (p.p23));

        s.b[1275] = (s.v[37] != 2.0);
        s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };

        s.b[1276] = (p.p41 == 0.0);
        s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1276]) {
            s.store_mul_div_from_scalar_lhs(843, (3.0 * 3.9), 416, 415);
        }

        if (s.b[1275] && (!s.b[1276])) {
            s.store_div_scaled_inputs_indices(843, 415, p.p47, 416, 1.0);
        }

        s.b[1277] = (p.p43 == 0.0);
        s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };

        s.b[1278] = (p.p41 == 0.0);
        s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1278]) {
            s.store_div_scaled_inputs3_indices(844, 822, -1.0, 1111, (-1.0), 1153, -1.0, 843, 1.0);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1278])) {
            s.store_div_scaled_inputs4_indices(844, 822, -1.0, 1111, (-1.0), 1153, -1.0, 375, 1.0, 843, 1.0);
        }

        s.b[1279] = (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0));
        s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && (!s.b[1279])) {
            s.store_scaled_add_sqrt_square_offset_rhs(844, 844, 844, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(845, s.ad_value(1151), 1.0, s.ad_value(844), 0.001, 1.0);
            s.store_square(847, 824);
            s.store_mul_neg_lhs(848, 824, 847);
            s.store_offset_add_ad(849, s.ad_value(1152), A::abs(s.ad_value(848)), 1e-9);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1275] && s.b[1277]) && (!s.b[1279])) {
            s.store_offset_add_scaled_inputs(850, A::div(s.ad_value(848), s.ad_value(849)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(848), s.ad_value(849)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));
        }

        s.b[1280] = (p.p41 == 0.0);
        s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && s.b[1280]) {
            s.store_div_scaled_inputs3_indices(844, 822, 1.0, 825, (-1.0), 1146, -1.0, 843, 1.0);
        }

        if ((s.b[1275] && s.b[1277]) && (!s.b[1280])) {
            s.store_div_scaled_inputs4_indices(844, 822, 1.0, 825, (-1.0), 1146, -1.0, 375, 1.0, 843, 1.0);
        }

        s.b[1281] = (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0));
        s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };

        if ((s.b[1275] && s.b[1277]) && (!s.b[1281])) {
            s.store_scaled_add_sqrt_square_offset_rhs(844, 844, 844, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(845, s.ad_value(1144), 1.0, s.ad_value(844), 0.001, 1.0);
            s.store_square(847, 900);
            s.store_mul_neg_lhs(848, 900, 847);
            s.store_offset_add_ad(849, s.ad_value(1145), A::abs(s.ad_value(848)), 1e-9);
            s.store_offset_add_scaled_inputs(850, A::div(s.ad_value(848), s.ad_value(849)), 0.5, A::sqrt_square_offset(A::div(s.ad_value(848), s.ad_value(849)), ((4.0 * 1e-6) * 1e-6)), 0.5, (-1e-6));
        }

        s.b[1282] = (p.p41 == 0.0);
        s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1282]) {
            s.store_div_scaled_inputs2_mixed_aii(844, A::add_scaled_product(s.ad_value(822), -1.0, s.ad_value(1154), s.ad_value(1111), (-1.0)), 1.0, 1153, (-1.0), 843, 1.0);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1282])) {
            s.store_div_scaled_inputs3_mixed_aiii(844, A::add_scaled_product(s.ad_value(822), -1.0, s.ad_value(1154), s.ad_value(1111), (-1.0)), 1.0, 1153, (-1.0), 375, 1.0, 843, 1.0);
        }

        s.b[1283] = (((s.v[1150] <= 0.0) || (s.v[1151] <= 0.0)) || (s.v[1152] < 0.0));
        s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) {
            s.store_scaled_add_sqrt_square_offset_rhs(844, 844, 844, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(845, s.ad_value(1151), 1.0, s.ad_value(844), 0.001, 1.0);
            s.store_sub(847, 824, 1156);
        }

        s.b[1284] = (s.v[847] >= ((-1.0) / 100.0));
        s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) && s.b[1284]) {
            s.store_scale(848, 1155, (-100.0));
        }

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) && (!s.b[1284])) {
            s.store_div(848, 1155, 847);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1283])) {
            s.store_exp(849, 848);
        }

        s.b[1285] = (p.p41 == 0.0);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && s.b[1285]) {
            s.store_div_scaled_inputs2_mixed_aii(844, A::add_scaled_product(s.ad_value(822), 1.0, s.ad_value(1147), s.ad_value(825), (-1.0)), 1.0, 1146, (-1.0), 843, 1.0);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1285])) {
            s.store_div_scaled_inputs3_mixed_aiii(844, A::add_scaled_product(s.ad_value(822), 1.0, s.ad_value(1147), s.ad_value(825), (-1.0)), 1.0, 1146, (-1.0), 375, 1.0, 843, 1.0);
        }

        s.b[1286] = (((s.v[1143] <= 0.0) || (s.v[1144] <= 0.0)) || (s.v[1145] < 0.0));
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) {
            s.store_scaled_add_sqrt_square_offset_rhs(844, 844, 844, ((4.0 * 0.01) * 0.01), 0.5);
            s.store_div_scaled_value_offset_denominator(845, s.ad_value(1144), 1.0, s.ad_value(844), 0.001, 1.0);
            s.store_sub(847, 900, 1149);
        }

        s.b[1287] = (s.v[847] >= ((-1.0) / 100.0));
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) && s.b[1287]) {
            s.store_scale(848, 1148, (-100.0));
        }

        if (((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) && (!s.b[1287])) {
            s.store_div(848, 1148, 847);
        }

        if ((s.b[1275] && (!s.b[1277])) && (!s.b[1286])) {
            s.store_exp(849, 848);
        }

        if s.b[1275] {
            s.store_scalar(974, (s.v[347] * p.p155));
            s.store_scalar(975, (s.v[348] * p.p155));
            s.store_mul(931, 832, 300);
            s.store_div(843, 1087, 931);
        }

        s.b[1288] = (s.v[843] > 100.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1288]) {
            s.store_scaled_offset(983, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1289] = (s.v[843] < (-100.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1288])) && s.b[1289]) {
            s.store_scalar(983, 3.720075976e-44);
        }

        if ((s.b[1275] && (!s.b[1288])) && (!s.b[1289])) {
            s.store_exp(983, 843);
        }

        if s.b[1275] {
            s.store_mul(931, 832, 301);
            s.store_div(843, 1088, 931);
        }

        s.b[1290] = (s.v[843] > 100.0);
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1290]) {
            s.store_scaled_offset(984, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1291] = (s.v[843] < (-100.0));
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1290])) && s.b[1291]) {
            s.store_scalar(984, 3.720075976e-44);
        }

        if ((s.b[1275] && (!s.b[1290])) && (!s.b[1291])) {
            s.store_exp(984, 843);
        }

        s.b[1292] = (s.v[947] <= 0.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if (s.b[1275] && (!s.b[1292])) {
            s.store_mul(843, 974, 947);
        }

        s.b[1293] = (s.v[948] <= 0.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (s.b[1275] && (!s.b[1293])) {
            s.store_mul(843, 975, 948);
        }

        s.b[1294] = (s.v[951] <= 0.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if (s.b[1275] && (!s.b[1294])) {
            s.store_mul_scaled_offset_ad_rhs(970, 302, p.p1043, A::mul(s.ad_value(254), s.ad_value(430)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(971, 304, p.p1043, A::mul(s.ad_value(255), s.ad_value(430)), 1.0);
            s.store_div(843, 1087, 970);
        }

        s.b[1295] = (s.v[843] > 100.0);
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1294])) && s.b[1295]) {
            s.store_scaled_offset(853, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1296] = (s.v[843] < (-100.0));
        s.v[1296] = if s.b[1296] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1295])) && s.b[1296]) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1295])) && (!s.b[1296])) {
            s.store_exp(853, 843);
        }

        s.b[1297] = ((s.v[314] - s.v[1087]) < 0.001);
        s.v[1297] = if s.b[1297] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1294])) && s.b[1297]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(971), 1.0), s.ad_value(314), 844);
        }

        s.b[1298] = (s.v[843] > 100.0);
        s.v[1298] = if s.b[1298] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && s.b[1297]) && s.b[1298]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1299] = (s.v[843] < (-100.0));
        s.v[1299] = if s.b[1299] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1294])) && s.b[1297]) && (!s.b[1298])) && s.b[1299]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1294])) && s.b[1297]) && (!s.b[1298])) && (!s.b[1299])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1294])) && s.b[1297]) {
            s.store_neg(854, 854);
        }

        if ((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(314), s.ad_value(1087));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(971), 1.0), s.ad_value(314), 844);
        }

        s.b[1300] = (s.v[843] > 100.0);
        s.v[1300] = if s.b[1300] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && s.b[1300]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1301] = (s.v[843] < (-100.0));
        s.v[1301] = if s.b[1301] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && (!s.b[1300])) && s.b[1301]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) && (!s.b[1300])) && (!s.b[1301])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1294])) && (!s.b[1297])) {
            s.store_neg(854, 854);
        }

        if (s.b[1275] && (!s.b[1294])) {
            s.store_mul(846, 974, 951);
        }

        s.b[1302] = (s.v[952] <= 0.0);
        s.v[1302] = if s.b[1302] { 1.0 } else { 0.0 };

        if (s.b[1275] && (!s.b[1302])) {
            s.store_mul_scaled_offset_ad_rhs(970, 303, p.p1043, A::mul(s.ad_value(254), s.ad_value(430)), 1.0);
            s.store_mul_scaled_offset_ad_rhs(971, 305, p.p1043, A::mul(s.ad_value(255), s.ad_value(430)), 1.0);
            s.store_div(843, 1088, 970);
        }

        s.b[1303] = (s.v[843] > 100.0);
        s.v[1303] = if s.b[1303] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1302])) && s.b[1303]) {
            s.store_scaled_offset(853, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1304] = (s.v[843] < (-100.0));
        s.v[1304] = if s.b[1304] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1303])) && s.b[1304]) {
            s.store_scalar(853, 3.720075976e-44);
        }

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1303])) && (!s.b[1304])) {
            s.store_exp(853, 843);
        }

        s.b[1305] = ((s.v[315] - s.v[1088]) < 0.001);
        s.v[1305] = if s.b[1305] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1302])) && s.b[1305]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(971), 1.0), s.ad_value(315), 844);
        }

        s.b[1306] = (s.v[843] > 100.0);
        s.v[1306] = if s.b[1306] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && s.b[1305]) && s.b[1306]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1307] = (s.v[843] < (-100.0));
        s.v[1307] = if s.b[1307] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1302])) && s.b[1305]) && (!s.b[1306])) && s.b[1307]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1302])) && s.b[1305]) && (!s.b[1306])) && (!s.b[1307])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1302])) && s.b[1305]) {
            s.store_neg(854, 854);
        }

        if ((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(315), s.ad_value(1088));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(971), 1.0), s.ad_value(315), 844);
        }

        s.b[1308] = (s.v[843] > 100.0);
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && s.b[1308]) {
            s.store_scaled_offset(854, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1309] = (s.v[843] < (-100.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && (!s.b[1308])) && s.b[1309]) {
            s.store_scalar(854, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) && (!s.b[1308])) && (!s.b[1309])) {
            s.store_exp(854, 843);
        }

        if ((s.b[1275] && (!s.b[1302])) && (!s.b[1305])) {
            s.store_neg(854, 854);
        }

        if (s.b[1275] && (!s.b[1302])) {
            s.store_mul(846, 975, 952);
        }

        if s.b[1275] {
            s.store_scalar(930, ((s.v[328] / p.p23) * p.p155));
        }

        s.b[1310] = ((s.v[949] <= 0.0) && (s.v[950] <= 0.0));
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if (s.b[1275] && s.b[1310]) {
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(933, 0.0);
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_mul_offset_rhs(989, 972, 983, (-1.0));
        }

        s.b[1311] = (s.v[989] < 1e-5);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1311]) {
            s.store_scalar(989, 0.0);
            s.store_scalar(991, 1.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1311])) {
            s.store_div_from_scalar_sqrt_ad(991, 1.0, A::offset(s.ad_value(989), 1.0));
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_mul_offset_rhs(990, 973, 984, (-1.0));
        }

        s.b[1312] = (s.v[990] < 1e-5);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1312]) {
            s.store_scalar(990, 0.0);
            s.store_scalar(992, 1.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1312])) {
            s.store_div_from_scalar_sqrt_ad(992, 1.0, A::offset(s.ad_value(990), 1.0));
        }

        if (s.b[1275] && (!s.b[1310])) {
            s.store_sub_from_scalar(843, 1.0, 351);
            s.store_mul3_lhs(985, 930, 949, 352);
            s.store_mul(844, 843, 985);
            s.store_mul3_lhs(985, 930, 950, 352);
            s.store_mul(844, 843, 985);
            s.store_mul3_lhs(986, 930, 949, 353);
            s.store_mul_ad_product_lhs(987, s.ad_value(986), A::offset(s.ad_value(983), (-1.0)), 991);
            s.store_mul3_lhs(986, 930, 950, 353);
            s.store_mul_ad_product_lhs(988, s.ad_value(986), A::offset(s.ad_value(984), (-1.0)), 992);
        }

        s.b[1313] = (p.p13 == 1.0);
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1310])) && s.b[1313]) {
            s.store_scalar(933, 0.0);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) {
            s.store_offset_div_scaled_inputs2_indices(843, 1087, 1.0, 1088, 1.0, 354, 1.0, 1.0);
            s.store_add(844, 989, 990);
            s.store_sqrt_add_scaled_square_input(846, 843, 1.0, 844, 4.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) {
            s.store_scaled_add(845, 843, 846, 0.5);
        }

        s.b[1314] = (s.v[845] < 0.1);
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) && s.b[1314]) {
            s.store_scalar(993, 10.0);
        }

        if (((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) && (!s.b[1314])) {
            s.store_div_from_scalar(993, 1.0, 845);
        }

        if ((s.b[1275] && (!s.b[1310])) && (!s.b[1313])) {
            s.store_mul(843, 351, 985);
            s.store_mul_ad_product_lhs(933, s.ad_value(843), A::sub(s.ad_value(983), s.ad_value(984)), 993);
        }

        s.b[1315] = ((s.v[953] <= 0.0) && (s.v[954] <= 0.0));
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if (s.b[1275] && (!s.b[1315])) {
            s.store_scale(932, 298, p.p1043);
        }

        s.b[1316] = ((s.v[316] - s.v[1087]) < 0.001);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(932), 1.0), s.ad_value(316), 844);
        }

        s.b[1317] = (s.v[843] > 100.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && s.b[1316]) && s.b[1317]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1318] = (s.v[843] < (-100.0));
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) && s.b[1318]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1316]) && (!s.b[1317])) && (!s.b[1318])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && s.b[1316]) {
            s.store_mul(846, 974, 953);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(316), s.ad_value(1087));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1087), -1.0, s.ad_value(932), 1.0), s.ad_value(316), 844);
        }

        s.b[1319] = (s.v[843] > 100.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && s.b[1319]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1320] = (s.v[843] < (-100.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && s.b[1320]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1319])) && (!s.b[1320])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1316])) {
            s.store_mul(846, 974, 953);
        }

        if (s.b[1275] && (!s.b[1315])) {
            s.store_scale(932, 299, p.p1043);
        }

        s.b[1321] = ((s.v[317] - s.v[1088]) < 0.001);
        s.v[1321] = if s.b[1321] { 1.0 } else { 0.0 };

        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {
            s.store_scalar(844, 1000.0);
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(932), 1.0), s.ad_value(317), 844);
        }

        s.b[1322] = (s.v[843] > 100.0);
        s.v[1322] = if s.b[1322] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && s.b[1321]) && s.b[1322]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1323] = (s.v[843] < (-100.0));
        s.v[1323] = if s.b[1323] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && s.b[1323]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && s.b[1321]) && (!s.b[1322])) && (!s.b[1323])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && s.b[1321]) {
            s.store_mul(846, 975, 954);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {
            s.store_div_from_scalar_sub_ad(844, 1.0, s.ad_value(317), s.ad_value(1088));
            s.store_mul_ad_product_lhs(843, A::div_scaled_inputs(s.ad_value(1088), -1.0, s.ad_value(932), 1.0), s.ad_value(317), 844);
        }

        s.b[1324] = (s.v[843] > 100.0);
        s.v[1324] = if s.b[1324] { 1.0 } else { 0.0 };

        if (((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && s.b[1324]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1325] = (s.v[843] < (-100.0));
        s.v[1325] = if s.b[1325] { 1.0 } else { 0.0 };

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) && (!s.b[1324])) && (!s.b[1325])) {
            s.store_exp(844, 843);
        }

        if ((s.b[1275] && (!s.b[1315])) && (!s.b[1321])) {
            s.store_mul(846, 975, 954);
        }

        if (!s.b[1275]) {
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(933, 0.0);
        }

        s.store_add_scaled_product_indices(203, 203, 1.0, 204, 430, 1.0);

        s.store_add_scaled_product_indices(207, 207, 1.0, 208, 430, 1.0);

        s.store_add_scaled_product_indices(243, 243, 1.0, 244, 430, 1.0);

        s.store_add_scaled_product_indices(246, 246, 1.0, 247, 430, 1.0);

        s.store_add_scaled_product_indices(250, 250, 1.0, 248, 430, 1.0);

        s.b[1326] = ((p.p374 != 0.0) || (p.p375 != 0.0));
        s.v[1326] = if s.b[1326] { 1.0 } else { 0.0 };

        if s.b[1326] {
            s.store_sub(1075, 825, 824);
            s.store_add_scaled_inputs_product_indices(826, 408, p.p37, 942, (-1.0), 405, 943, (-1.0));
            s.store_add_scaled_inputs3_offset_indices(846, 826, 1.0, 825, (-1.0), 824, 1.0, (-0.02));
        }

        s.b[1327] = (s.v[826] <= 0.0);
        s.v[1327] = if s.b[1327] { 1.0 } else { 0.0 };

        if (s.b[1326] && s.b[1327]) {
            s.store_sqrt_add_scaled_square_input(843, 846, 1.0, 826, (-(4.0 * 0.02)));
        }

        if (s.b[1326] && (!s.b[1327])) {
            s.store_sqrt_add_scaled_square_input(843, 846, 1.0, 826, (4.0 * 0.02));
        }

        if s.b[1326] {
            s.store_add_scaled_inputs3_indices(812, 826, 1.0, 846, (-0.5), 843, (-0.5));
            s.store_sub(1081, 826, 812);
        }

        s.b[1328] = (s.v[1081] < 0.0);
        s.v[1328] = if s.b[1328] { 1.0 } else { 0.0 };

        if (s.b[1326] && s.b[1328]) {
            s.store_scalar(1081, 0.0);
        }

        s.b[1329] = (s.v[376] == 0.0);
        s.v[1329] = if s.b[1329] { 1.0 } else { 0.0 };

        if (s.b[1326] && s.b[1329]) {
            s.store_scalar(1082, 0.0);
        }

        if (s.b[1326] && (!s.b[1329])) {
            s.store_add_scaled_inputs4_indices(843, 825, 1.0, 875, (-1.0), 812, -1.0, 841, -1.0);
        }

        s.b[1330] = (s.v[843] < 0.0);
        s.v[1330] = if s.b[1330] { 1.0 } else { 0.0 };

        if ((s.b[1326] && (!s.b[1329])) && s.b[1330]) {
            s.store_div(844, 843, 376);
        }

        if ((s.b[1326] && (!s.b[1329])) && (!s.b[1330])) {
            s.store_mul_scaled_offset_ad_rhs(844, 376, 1.0 / (2.0), A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(843), 4.0, s.ad_value(376), s.ad_value(376), 1.0), 1.0)), (-1.0));
        }

        if (s.b[1326] && (!s.b[1329])) {
            s.store_add_scaled_inputs4_mixed_iaii(1082, 825, 1.0, A::square(s.ad_value(844)), -1.0, 824, -1.0, 826, -1.0);
        }

        if (!s.b[1326]) {
            s.store_scalar(826, 0.0);
            s.store_scalar(1075, 0.0);
            s.store_scalar(1081, 0.0);
            s.store_scalar(1082, 0.0);
        }

        if (p.p375 != 0.0) {
            s.store_mul(843, 832, 211);
            s.store_div_scaled_inputs2_indices(1028, 825, 1.0, 408, (-p.p37), 843, 1.0);
        }

        s.b[1331] = (s.v[1028] > 100.0);
        s.v[1331] = if s.b[1331] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1331]) {
            s.store_sub_scaled_inputs(1078, 825, 1.0, 408, p.p37);
        }

        s.b[1332] = (s.v[1028] < (-100.0));
        s.v[1332] = if s.b[1332] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1331])) && s.b[1332]) {
            s.store_scale(1078, 843, (((1.0 + 3.720075976e-44)) as f64).ln());
        }

        if (((p.p375 != 0.0) && (!s.b[1331])) && (!s.b[1332])) {
            s.store_exp(1029, 1028);
            s.store_mul_ln_ad_rhs(1078, 843, A::offset(s.ad_value(1029), 1.0));
        }

        if (p.p375 != 0.0) {
            s.store_mul(845, 825, 1078);
            s.store_scalar(854, s.v[369]);
            s.store_scalar(855, s.v[370]);
            s.store_add_scaled_product_indices(846, 205, (-1.0), 203, 206, 1.0);
            s.store_mul(847, 205, 206);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(203), 1.0, s.ad_value(846), s.ad_value(1082), 1.0), A::mul3(s.ad_value(847), s.ad_value(1082), s.ad_value(1082)));
        }

        s.b[1333] = (s.v[848] > 100.0);
        s.v[1333] = if s.b[1333] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1333]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1334] = (s.v[848] < (-100.0));
        s.v[1334] = if s.b[1334] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1333])) && s.b[1334]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1333])) && (!s.b[1334])) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_mul_neg_lhs(850, 212, 822);
            s.store_offset_square(851, 850, 0.0002);
        }

        s.b[1335] = (s.v[850] > 100.0);
        s.v[1335] = if s.b[1335] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1335]) {
            s.store_scalar(852, 2.688117142e43);
        }

        s.b[1336] = (s.v[850] < (-100.0));
        s.v[1336] = if s.b[1336] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1335])) && s.b[1336]) {
            s.store_scalar(852, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1335])) && (!s.b[1336])) {
            s.store_exp(852, 850);
        }

        if (p.p375 != 0.0) {
            s.store_offset(844, 852, (((-1.0)) + (0.0001)));
            s.store_div_scaled_inputs2_indices(853, 844, 1.0, 850, (-1.0), 851, 1.0);
            s.store_offset(844, 852, (((-1.0)) + ((-0.0001))));
            s.store_div_scaled_add_product(853, s.ad_value(844), (-1.0), s.ad_value(850), s.ad_value(852), 1.0, s.ad_value(851), 1.0);
            s.store_sub(843, 821, 375);
            s.store_sqrt_square_offset(1026, 843, 0.0001);
            s.store_mul(845, 821, 1026);
            s.copy_ad(964, 372);
            s.copy_ad(965, 373);
            s.copy_ad(855, 374);
            s.store_add_scaled_product_indices(846, 209, (-1.0), 207, 210, 1.0);
            s.store_mul(847, 209, 210);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1026), 1.0), A::mul3(s.ad_value(847), s.ad_value(1026), s.ad_value(1026)));
        }

        s.b[1337] = (s.v[848] > 100.0);
        s.v[1337] = if s.b[1337] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1337]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1338] = (s.v[848] < (-100.0));
        s.v[1338] = if s.b[1338] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1337])) && s.b[1338]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1337])) && (!s.b[1338])) {
            s.store_exp(849, 848);
        }

        if (p.p375 != 0.0) {
            s.store_sub(843, 820, 375);
            s.store_sqrt_square_offset(1027, 843, 0.0001);
            s.store_mul(845, 820, 1027);
            s.store_mul_sub_ad_rhs(848, 855, A::add_scaled_product(s.ad_value(207), 1.0, s.ad_value(846), s.ad_value(1027), 1.0), A::mul3(s.ad_value(847), s.ad_value(1027), s.ad_value(1027)));
        }

        s.b[1339] = (s.v[848] > 100.0);
        s.v[1339] = if s.b[1339] { 1.0 } else { 0.0 };

        if ((p.p375 != 0.0) && s.b[1339]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1340] = (s.v[848] < (-100.0));
        s.v[1340] = if s.b[1340] { 1.0 } else { 0.0 };

        if (((p.p375 != 0.0) && (!s.b[1339])) && s.b[1340]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if (((p.p375 != 0.0) && (!s.b[1339])) && (!s.b[1340])) {
            s.store_exp(849, 848);
        }

        s.b[1341] = ((p.p374 != 0.0) && (s.v[37] != 2.0));
        s.v[1341] = if s.b[1341] { 1.0 } else { 0.0 };

        if s.b[1341] {
            s.store_scalar(1077, s.v[345]);
            s.copy_ad(1076, 1082);
            s.store_scalar(843, p.p396);
            s.store_offset_sub(844, 843, 1076, (-p.p397));
            s.store_sqrt_add_scaled_square_input(846, 844, 1.0, 843, (4.0 * p.p397));
            s.store_add_scaled_inputs3_indices(1080, 843, 1.0, 844, (-0.5), 846, (-0.5));
            s.copy_ad(1076, 1080);
            s.store_scaled_offset(843, 1076, (-p.p381), 1.0 / (p.p382));
        }

        s.b[1342] = (s.v[843] > 100.0);
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1342]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1343] = (s.v[843] < (-100.0));
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1341] && (!s.b[1342])) && s.b[1343]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1342])) && (!s.b[1343])) {
            s.store_exp(844, 843);
        }

        if s.b[1341] {
            s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p.p382);
        }

        s.b[1344] = (p.p386 != 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1344]) {
            s.store_sub_from_scalar_scaled_input(843, 1.0, 1076, 1.0 / (p.p386));
        }

        if (s.b[1341] && (!s.b[1344])) {
            s.store_scalar(843, 1.0);
        }

        s.b[1345] = (s.v[843] < 0.01);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1345]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1341] {
            s.store_mul_scale_ad_lhs(844, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p.p23)), (p.p28 / p.p3)), p.p1035, 1077);
            s.store_scalar(845, (p.p1036 * p.p376));
            s.copy_ad(846, 243);
            s.copy_ad(847, 245);
            s.store_div_scaled_product_right_ad(849, 845, A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, 843, 1.0);
        }

        s.b[1346] = (s.v[849] > 100.0);
        s.v[1346] = if s.b[1346] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1346]) {
            s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1347] = (s.v[849] < (-100.0));
        s.v[1347] = if s.b[1347] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1346])) && s.b[1347]) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1346])) && (!s.b[1347])) {
            s.store_exp(848, 849);
        }

        if s.b[1341] {
            s.copy_ad(1076, 1081);
            s.store_scalar(843, p.p396);
            s.store_offset_sub(844, 843, 1076, (-p.p397));
            s.store_sqrt_add_scaled_square_input(846, 844, 1.0, 843, (4.0 * p.p397));
            s.store_add_scaled_inputs3_indices(1080, 843, 1.0, 844, (-0.5), 846, (-0.5));
            s.copy_ad(1076, 1080);
            s.store_scaled_sub(843, 826, 1075, 1.0 / (p.p387));
        }

        s.b[1348] = (s.v[843] > 100.0);
        s.v[1348] = if s.b[1348] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1348]) {
            s.store_scaled_offset(844, 843, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1349] = (s.v[843] < (-100.0));
        s.v[1349] = if s.b[1349] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1348])) && s.b[1349]) {
            s.store_scalar(844, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1348])) && (!s.b[1349])) {
            s.store_exp(844, 843);
        }

        if s.b[1341] {
            s.store_scaled_ln_ad(1078, A::offset(s.ad_value(844), 1.0), p.p387);
        }

        s.b[1350] = (p.p391 != 0.0);
        s.v[1350] = if s.b[1350] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1350]) {
            s.store_sub_from_scalar_scaled_input(843, 1.0, 1076, 1.0 / (p.p391));
        }

        if (s.b[1341] && (!s.b[1350])) {
            s.store_scalar(843, 1.0);
        }

        s.b[1351] = (s.v[843] < 0.01);
        s.v[1351] = if s.b[1351] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1351]) {
            s.store_scalar(843, 0.01);
        }

        if s.b[1341] {
            s.store_mul_scale_ad_lhs(844, A::scale_offset(s.ad_value(893), (s.v[892] * 1.0 / (p.p23)), (p.p28 / p.p3)), p.p1037, 1077);
            s.store_scalar(845, (p.p1038 * p.p376));
            s.copy_ad(846, 246);
            s.copy_ad(847, 249);
            s.store_div_scaled_product_right_ad(849, 845, A::add_scaled_product(s.ad_value(846), 1.0, s.ad_value(847), s.ad_value(1076), (-1.0)), 1.0, 843, 1.0);
        }

        s.b[1352] = (s.v[849] > 100.0);
        s.v[1352] = if s.b[1352] { 1.0 } else { 0.0 };

        if (s.b[1341] && s.b[1352]) {
            s.store_scaled_offset(848, 849, ((1.0) + ((-100.0))), 2.688117142e43);
        }

        s.b[1353] = (s.v[849] < (-100.0));
        s.v[1353] = if s.b[1353] { 1.0 } else { 0.0 };

        if ((s.b[1341] && (!s.b[1352])) && s.b[1353]) {
            s.store_scalar(848, 3.720075976e-44);
        }

        if ((s.b[1341] && (!s.b[1352])) && (!s.b[1353])) {
            s.store_exp(848, 849);
        }

        if s.b[1341] {
            s.store_offset(1127, 826, p.p1033);
        }

        s.b[1355] = (((((p.p374 != 0.0) && (s.v[37] != 2.0)) && (s.v[399] != 0.0)) && (p.p27 > 0.0)) && (s.v[1114] < s.v[1127]));
        s.v[1355] = if s.b[1355] { 1.0 } else { 0.0 };

        if s.b[1355] {
            s.store_sub(843, 1114, 1127);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_offset_scaled_sub(1113, 844, 843, 0.5, (((-0.01)) * (0.5)));
        }

        if s.b[1355] {
            s.store_scalar(854, (if (p.p37 == 1.0) { p.p1039 } else { p.p1040 }));
        }

        if s.b[1355] {
            s.store_scalar(855, (if (p.p37 == 1.0) { p.p1041 } else { p.p1042 }));
        }

        if s.b[1355] {
            s.store_mul(845, 1114, 1113);
            s.store_add_scaled_product_indices(846, 251, (-1.0), 250, 252, 1.0);
            s.store_mul(847, 251, 252);
            s.store_mul_sub_scaled_inputs_rhs(848, 855, A::add_scaled_product(s.ad_value(250), 1.0, s.ad_value(846), s.ad_value(1113), 1.0), (-p.p376), A::mul3(s.ad_value(847), s.ad_value(1113), s.ad_value(1113)), (-p.p376));
        }

        s.b[1356] = (s.v[848] > 100.0);
        s.v[1356] = if s.b[1356] { 1.0 } else { 0.0 };

        if (s.b[1355] && s.b[1356]) {
            s.store_scalar(849, 2.688117142e43);
        }

        s.b[1357] = (s.v[848] < (-100.0));
        s.v[1357] = if s.b[1357] { 1.0 } else { 0.0 };

        if ((s.b[1355] && (!s.b[1356])) && s.b[1357]) {
            s.store_scalar(849, 3.720075976e-44);
        }

        if ((s.b[1355] && (!s.b[1356])) && (!s.b[1357])) {
            s.store_exp(849, 848);
        }

        if s.b[1355] {
            s.store_scale(854, 854, (p.p27 * s.v[345]));
        }

        s.b[1358] = (s.v[37] != 2.0);
        s.v[1358] = if s.b[1358] { 1.0 } else { 0.0 };

        s.b[1359] = (p.p44 == 0.0);
        s.v[1359] = if s.b[1359] { 1.0 } else { 0.0 };

        s.b[1360] = (s.v[201] <= 0.0);
        s.v[1360] = if s.b[1360] { 1.0 } else { 0.0 };

        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {
            s.store_add_scaled_product_right_ad(966, 276, (-1.0 / (s.v[892])), 275, A::scale_offset(s.ad_value(430), p.p308, 1.0), 1.0);
            s.store_scale(843, 277, s.v[892]);
            s.store_div_scaled_product_offset_denominator(844, s.ad_value(278), s.ad_value(843), 1.0, s.ad_value(843), 1.0, 1.0);
            s.store_div_from_scalar_offset_product(843, 1.0, 279, 875, 1.0);
            s.store_add(846, 843, 280);
            s.store_mul(845, 830, 846);
            s.store_div_from_scalar_offset_product(846, 1.0, 281, 822, 1.0);
            s.store_mul3_lhs(967, 844, 845, 846);
            s.store_add(921, 966, 967);
            s.store_sub(969, 822, 921);
            s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));
        }

        s.b[1361] = (s.v[843] < 1e-5);
        s.v[1361] = if s.b[1361] { 1.0 } else { 0.0 };

        if (((s.b[1358] && s.b[1359]) && (!s.b[1360])) && s.b[1361]) {
            s.store_scalar(843, 1e-5);
        }

        if ((s.b[1358] && s.b[1359]) && (!s.b[1360])) {
            s.store_add_ad_rhs(843, 885, A::mul3(s.ad_value(267), s.ad_value(398), s.ad_value(933)));
        }

        s.b[1365] = (s.v[201] <= 0.0);
        s.v[1365] = if s.b[1365] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {
            s.store_add_scaled_product_right_ad(966, 276, (-1.0 / (s.v[892])), 275, A::scale_offset(s.ad_value(430), p.p308, 1.0), 1.0);
            s.store_scale(843, 277, s.v[892]);
            s.store_div_scaled_product_offset_denominator(844, s.ad_value(278), s.ad_value(843), 1.0, s.ad_value(843), 1.0, 1.0);
            s.store_div_from_scalar_offset_product(843, 1.0, 279, 875, 1.0);
            s.store_add(846, 843, 280);
            s.store_mul(845, 830, 846);
            s.store_div_from_scalar_offset_product(846, 1.0, 281, 822, 1.0);
            s.store_mul3_lhs(967, 844, 845, 846);
            s.store_add(921, 966, 967);
            s.store_sub(969, 822, 921);
            s.store_add_ad(843, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(273), s.ad_value(969), 1.0), A::mul3(s.ad_value(202), s.ad_value(969), s.ad_value(969)));
        }

        s.b[1366] = (s.v[843] < 1e-5);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) && s.b[1366]) {
            s.store_scalar(843, 1e-5);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1365])) {
            s.copy_ad(843, 885);
        }

        if (s.b[1358] && (!s.b[1359])) {
            s.store_add_scaled_inputs(843, 269, 1.0 / (s.v[892]), 268, (s.v[892] * 1.0 / (s.v[892])));
            s.store_mul_scale_offset_rhs(1105, 270, 430, p.p320, 1.0);
        }

        s.b[1370] = (s.v[398] > 0.0);
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1370]) {
            s.store_sub(844, 1105, 1088);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1370])) {
            s.store_sub(844, 1105, 1087);
        }

        if (s.b[1358] && (!s.b[1359])) {
            s.store_offset(845, 272, (-1.0));
        }

        s.b[1371] = (s.v[844] <= 0.0);
        s.v[1371] = if s.b[1371] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1371]) {
            s.store_scalar(846, 0.0);
        }

        if ((s.b[1358] && (!s.b[1359])) && (!s.b[1371])) {
            s.store_mul_scaled_pow_ad_rhs(846, 271, -1.0, s.ad_value(844), s.ad_value(845));
        }

        s.b[1372] = (s.v[846] > 100.0);
        s.v[1372] = if s.b[1372] { 1.0 } else { 0.0 };

        if ((s.b[1358] && (!s.b[1359])) && s.b[1372]) {
            s.store_scalar(847, 2.688117142e43);
        }

        s.b[1373] = (s.v[846] < (-100.0));
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && s.b[1373]) {
            s.store_scalar(847, 3.720075976e-44);
        }

        if (((s.b[1358] && (!s.b[1359])) && (!s.b[1372])) && (!s.b[1373])) {
            s.store_exp(847, 846);
        }

        s.b[1374] = ((s.v[399] == 0.0) || (s.v[399] == 2.0));
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        s.b[1375] = (s.v[156] < 0.001);
        s.v[1375] = if s.b[1375] { 1.0 } else { 0.0 };

        s.b[1376] = (s.v[50] <= 0.001);
        s.v[1376] = if s.b[1376] { 1.0 } else { 0.0 };

        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && s.b[1376]) {
            s.store_scalar(843, (1.0 / 0.001));
        }

        if (((s.b[1358] && (!s.b[1374])) && s.b[1375]) && (!s.b[1376])) {
            s.store_scalar(843, (1.0 / s.v[50]));
        }

        s.b[1377] = (p.p39 > 1.0);
        s.v[1377] = if s.b[1377] { 1.0 } else { 0.0 };

        if s.b[1377] {
            s.store_mul(852, 230, 49);
            s.store_mul(843, 852, 880);
            s.store_mul_add_rhs(81, 229, 843, 1086);
        }

        s.b[1378] = (p.p3 != 1.0);
        s.v[1378] = if s.b[1378] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1378]) {
            s.store_scale(81, 81, p.p3);
        }

        s.b[1379] = (p.p39 == 2.0);
        s.v[1379] = if s.b[1379] { 1.0 } else { 0.0 };

        if (s.b[1377] && s.b[1379]) {
            s.store_add(854, 64, 81);
            s.store_div_scaled_product_indices(81, 64, 81, 1.0, 854, 1.0);
        }

        if (!s.b[1377]) {
            s.store_scalar(81, 0.0);
        }

        s.b[1380] = (p.p429 == 0.0);
        s.v[1380] = if s.b[1380] { 1.0 } else { 0.0 };

        s.b[1385] = (p.p429 == 1.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if ((!s.b[1380]) && s.b[1385]) {
            s.store_scalar(887, 0.0);
            s.store_sub(843, 821, 375);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_scaled_add(1026, 843, 844, 0.5);
            s.store_offset_mul(843, 183, 1026, 1.0);
            s.store_mul_neg_lhs(844, 184, 818);
            s.store_add_scaled_inputs_product_mixed_aiia(845, A::div_from_scalar(1.0, s.ad_value(843)), 1.0, 844, 1.0, 185, A::sub(s.ad_value(897), s.ad_value(941)), 1.0);
            s.store_add_ad_rhs(846, 845, A::sqrt_square_offset(s.ad_value(845), 0.01));
            s.store_scale(847, 1096, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1380]) && s.b[1385]) {
            s.store_sub(843, 820, 375);
            s.store_sqrt_square_offset(844, 843, 0.0001);
            s.store_scaled_add(1027, 843, 844, 0.5);
            s.store_offset_mul(843, 183, 1027, 1.0);
            s.store_mul_neg_lhs(844, 184, 817);
            s.store_add_scaled_inputs_product_mixed_aiia(845, A::div_from_scalar(1.0, s.ad_value(843)), 1.0, 844, 1.0, 185, A::sub(s.ad_value(897), s.ad_value(941)), 1.0);
            s.store_add_ad_rhs(846, 845, A::sqrt_square_offset(s.ad_value(845), 0.01));
            s.store_scale(847, 1095, 0.5);
        }

        s.store_mul_sub_from_scalar_ad_rhs(844, 875, 1.0, A::div_scaled_product(s.ad_value(860), s.ad_value(876), 0.5, s.ad_value(890), 1.0));

        s.b[1389] = (p.p3 != 1.0);
        s.v[1389] = if s.b[1389] { 1.0 } else { 0.0 };

        if s.b[1389] {
            s.store_scale(885, 885, p.p3);
            s.store_scale(933, 933, p.p3);
            s.store_scale(78, 78, p.p3);
        }

        s.store_scalar(83, (A::ddx_projection(&s.ad_value(885), Some(9), None) * p.p37));

        s.b[1390] = (s.v[398] > 0.0);
        s.v[1390] = if s.b[1390] { 1.0 } else { 0.0 };

        if s.b[1390] {
            s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(7), None) * p.p37));
        }

        if (!s.b[1390]) {
            s.store_scalar(84, (A::ddx_projection(&s.ad_value(885), Some(8), None) * p.p37));
        }

        s.store_scalar(85, (A::ddx_projection(&s.ad_value(885), Some(5), None) * p.p37));

        s.store_scale(842, 396, ((((s.v[332] / p.p23) * p.p3) * s.v[331]) + p.p26));

        s.store_scale(981, 396, (p.p361 * ((((s.v[332] / p.p23) * p.p3) * s.v[365]) + p.p26)));

        s.store_scale(1115, 396, p.p27);

        s.store_scale(1116, 396, (p.p361 * p.p27));

        s.store_sub(830, 825, 1073);

        s.store_mul(853, 1059, 832);

        s.store_div_scaled_product_indices(809, 384, 830, 1.0, 853, 1.0);

        s.store_mul3_lhs(1016, 1059, 363, 832);

        s.store_mul3_lhs(1017, 1059, 364, 832);

        s.b[1391] = (p.p42 == 0.0);
        s.v[1391] = if s.b[1391] { 1.0 } else { 0.0 };

        s.b[1392] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));
        s.v[1392] = if s.b[1392] { 1.0 } else { 0.0 };

        if (s.b[1391] && s.b[1392]) {
            let assign26000_ad_e23337: A = A::exp(s.ad_value(809));
            s.store_square_ad(810, assign26000_ad_e23337);
        }

        if (s.b[1391] && s.b[1392]) {
            s.store_mul_ad_rhs(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));
        }

        if (s.b[1391] && s.b[1392]) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1393] = (p.p27 > 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {
            s.store_mul_exp_ad_rhs(1117, 810, A::div_scalar_by_product((-p.p1033), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));
        }

        if ((s.b[1391] && s.b[1392]) && s.b[1393]) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1394] = (p.p42 == 1.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        s.b[1395] = ((s.v[809] > (-100.0)) && (s.v[809] < 100.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {
            s.store_exp_ad(810, A::div(s.ad_value(809), A::mul(s.ad_value(384), s.ad_value(363))));
            s.store_mul_ad_rhs(810, 810, A::exp_scaled_input(A::div(s.ad_value(324), s.ad_value(1016)), -1.0));
        }

        if (((!s.b[1391]) && s.b[1394]) && s.b[1395]) {
            s.store_mul_ad_rhs(875, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1396] = (p.p27 > 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {
            s.store_mul_exp_ad_rhs(1117, 810, A::div_scalar_by_product((-p.p1033), s.ad_value(1017), A::square(s.ad_value(832)), 1.0));
        }

        if ((((!s.b[1391]) && s.b[1394]) && s.b[1395]) && s.b[1396]) {
            s.store_mul_ad_rhs(1118, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1391]) && (!s.b[1394])) {
            s.store_div_scaled_product_right_ad(809, 388, A::sub(s.ad_value(830), s.ad_value(324)), 1.0, 1016, 1.0);
            s.store_div_scaled_inputs2_mixed_iai(833, 390, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324))), (-1.0), 1016, 1.0);
        }

        s.b[1397] = (s.v[809] > 100.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1397]) {
            s.store_sub(875, 830, 324);
        }

        s.b[1398] = (s.v[833] > 100.0);
        s.v[1398] = if s.b[1398] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && s.b[1398]) {
            s.store_div_scaled_inputs3_indices(843, 830, 1.0, 324, (-1.0), 390, -1.0, 1016, 1.0);
            s.store_exp(810, 843);
            s.store_mul_div_scaled_product_rhs(875, 810, s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0);
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_exp(810, 809);
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_mul_ad_rhs(844, 1016, {
                if ((1.0 + s.v[810]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(810), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((((!s.b[1391]) && (!s.b[1394])) && (!s.b[1397])) && (!s.b[1398])) {
            s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(833)), A::sub_from_scalar(1.0, s.ad_value(388)));
            s.store_sub_ad_rhs(845, 388, A::div_scaled_product(s.ad_value(1016), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));
            s.store_div(875, 844, 845);
        }

        s.b[1399] = (p.p27 > 0.0);
        s.v[1399] = if s.b[1399] { 1.0 } else { 0.0 };

        if (((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) {
            s.store_div_scaled_product_offset_rhs(1119, s.ad_value(388), A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033), 1.0, s.ad_value(1017), 1.0);
            s.store_div_scaled_inputs2_mixed_iai(1120, 390, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(388), A::offset(A::sub(s.ad_value(830), s.ad_value(324)), (-p.p1033))), (-1.0), 1017, 1.0);
        }

        s.b[1400] = (s.v[1119] > 100.0);
        s.v[1400] = if s.b[1400] { 1.0 } else { 0.0 };

        if ((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && s.b[1400]) {
            s.store_offset_sub(1118, 830, 324, (-p.p1033));
        }

        s.b[1401] = (s.v[1120] > 100.0);
        s.v[1401] = if s.b[1401] { 1.0 } else { 0.0 };

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && s.b[1401]) {
            s.store_div_scaled_offset_numerator(843, A::add_scaled_inputs3(s.ad_value(830), 1.0, s.ad_value(324), (-1.0), s.ad_value(390), -1.0), 1.0, (-p.p1033), s.ad_value(1017), 1.0);
            s.store_exp(1117, 843);
            s.store_mul_div_scaled_product_rhs(1118, 1117, s.ad_value(832), s.ad_value(1140), 1.0, s.ad_value(396), 1.0);
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_exp(1117, 1119);
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_mul_ad_rhs(844, 1017, {
                if ((1.0 + s.v[1117]) > 1e-38) {
                    A::ln(A::offset(s.ad_value(1117), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (((((!s.b[1391]) && (!s.b[1394])) && s.b[1399]) && (!s.b[1400])) && (!s.b[1401])) {
            s.store_mul3_ad(857, A::div_scaled_inputs(s.ad_value(396), -1.0, A::mul(s.ad_value(832), s.ad_value(1140)), 1.0), A::exp(s.ad_value(1120)), A::sub_from_scalar(1.0, s.ad_value(388)));
            s.store_sub_ad_rhs(845, 388, A::div_scaled_product(s.ad_value(1017), s.ad_value(857), 1.0, A::sub_from_scalar(1.0, s.ad_value(388)), 1.0));
            s.store_div(1118, 844, 845);
        }

        s.copy_ad(829, 1073);

        s.copy_ad(828, 1054);

        s.copy_ad(841, 1044);

        s.b[1402] = (p.p61 == 2.0);
        s.v[1402] = if s.b[1402] { 1.0 } else { 0.0 };

        s.b[1403] = (s.v[37] == 2.0);
        s.v[1403] = if s.b[1403] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1403]) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_add_ad_lhs(826, A::add_scaled_inputs_product(s.ad_value(829), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(828), (-1.0)), 324);
            s.store_add_scaled_inputs3_offset_indices(813, 826, 1.0, 825, (-1.0), 841, 1.0, (-0.08));
        }

        s.b[1404] = (s.v[826] <= 0.0);
        s.v[1404] = if s.b[1404] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1404]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (-(4.0 * 0.08)));
        }

        if ((s.b[1402] && (!s.b[1403])) && (!s.b[1404])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 826, (4.0 * 0.08));
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_add_scaled_inputs3_indices(812, 826, 1.0, 813, (-0.5), 843, (-0.5));
            s.store_mul_sub_rhs(938, 981, 812, 826);
        }

        s.b[1405] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1405] = if s.b[1405] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {
            s.store_offset(1127, 826, p.p1033);
            s.store_scalar(1139, 0.08);
            s.store_add_scaled_inputs4_indices(813, 1127, 1.0, 1125, (-1.0), 841, 1.0, 1139, -1.0);
        }

        s.b[1406] = (s.v[1127] <= 0.0);
        s.v[1406] = if s.b[1406] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && s.b[1406]) {
            s.store_sqrt_add_scaled_square_product(843, 813, 1.0, 1139, 1127, (-100.0));
        }

        if (((s.b[1402] && (!s.b[1403])) && s.b[1405]) && (!s.b[1406])) {
            s.store_sqrt_add_scaled_square_product(843, 813, 1.0, 1139, 1127, 100.0);
        }

        if ((s.b[1402] && (!s.b[1403])) && s.b[1405]) {
            s.store_add_scaled_inputs3_indices(1128, 1127, 1.0, 813, (-0.5), 843, (-0.5));
            s.store_add_scaled_product_right_sub(938, 938, 1.0, 1116, 1128, 1127, 1.0);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_scale(843, 376, 0.5);
            s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);
        }

        s.b[1407] = (s.v[376] == 0.0);
        s.v[1407] = if s.b[1407] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1407]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1408] = (s.v[846] < 0.0);
        s.v[1408] = if s.b[1408] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && s.b[1408]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if (((s.b[1402] && (!s.b[1403])) && (!s.b[1407])) && (!s.b[1408])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if (s.b[1402] && (!s.b[1403])) {
            s.store_mul_ad_product_rhs(937, 981, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.b[1409] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1409] = if s.b[1409] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {
            s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);
        }

        s.b[1410] = (s.v[846] < 0.0);
        s.v[1410] = if s.b[1410] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && s.b[1410]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if (((s.b[1402] && (!s.b[1403])) && s.b[1409]) && (!s.b[1410])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if ((s.b[1402] && (!s.b[1403])) && s.b[1409]) {
            s.store_add_ad_rhs(937, 937, A::mul3(s.ad_value(1116), s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843))));
        }

        if s.b[1402] {
            s.store_mul(894, 861, 333);
            s.store_div(891, 875, 894);
            s.store_offset_sub(814, 891, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));
        }

        s.b[1411] = (p.p27 > 0.0);
        s.v[1411] = if s.b[1411] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1411]) {
            s.store_div(1129, 1118, 894);
            s.store_offset_sub(814, 1129, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 1129, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 843, (-0.5));
        }

        s.b[1412] = (s.v[37] == 2.0);
        s.v[1412] = if s.b[1412] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1412]) {
            s.store_scalar(1006, 0.0);
        }

        if (s.b[1402] && (!s.b[1412])) {
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 877, 844);
            s.store_mul(846, 843, 845);
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_mul_ad_product_rhs(1006, 981, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(877), 0.5, s.ad_value(846), 1.0));
        }

        s.b[1413] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1413] = if s.b[1413] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1412])) && s.b[1413]) {
            s.store_mul(843, 894, 1130);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 1130, 844);
            s.store_mul(846, 843, 845);
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_add_ad_rhs(1006, 1006, A::mul3(s.ad_value(1116), s.ad_value(850), A::sub_scaled_inputs(s.ad_value(1130), 0.5, s.ad_value(846), 1.0)));
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1402] {
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(844, A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(845, 843, 844);
            s.store_mul(846, 843, 845);
            s.store_mul_add_scaled_inputs3_offset_rhs(915, 842, s.ad_value(875), 1.0, s.ad_value(843), (-0.5), s.ad_value(846), 1.0, 0.0);
        }

        s.b[1414] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1414] = if s.b[1414] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1414]) {
            s.store_mul(1121, 894, 1130);
            s.store_scaled_offset_ad(855, A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);
            s.store_div(845, 1121, 855);
            s.store_mul(846, 1121, 845);
            s.store_add_scaled_product_right_ad(915, 915, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 1.0, s.ad_value(1121), (-0.5), s.ad_value(846), 1.0), 1.0);
        }

        s.b[1415] = (p.p129 > 0.5);
        s.v[1415] = if s.b[1415] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1415]) {
            s.store_scale(844, 844, 2.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 842, s.ad_value(875), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 1.0, s.ad_value(844), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1416] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1416] = if s.b[1416] { 1.0 } else { 0.0 };

        if ((s.b[1402] && s.b[1415]) && s.b[1416]) {
            s.store_scale(855, 855, 2.0);
            s.store_add_scaled_product_right_ad(919, 919, 1.0, 1115, A::add_scaled_inputs3(s.ad_value(1118), 0.5, s.ad_value(1121), 0.25, A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 1.0, s.ad_value(855), 1.0), -1.0), (-1.0));
        }

        s.b[1417] = (p.p129 < 0.5);
        s.v[1417] = if s.b[1417] { 1.0 } else { 0.0 };

        if ((s.b[1402] && (!s.b[1415])) && s.b[1417]) {
            s.store_scale(844, 844, 0.08333333333333333);
            s.store_div_scaled_inputs_mixed_ia(845, 842, 0.5, A::square(s.ad_value(844)), 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 875, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(875), A::sub_scaled_inputs(s.ad_value(875), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 845, 846);
        }

        s.b[1418] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1418] = if s.b[1418] { 1.0 } else { 0.0 };

        if (((s.b[1402] && (!s.b[1415])) && s.b[1417]) && s.b[1418]) {
            s.store_scale(855, 855, 0.08333333333333333);
            s.store_div_scaled_inputs_mixed_ia(845, 1115, 0.5, A::square(s.ad_value(855)), 1.0);
            s.store_add_scaled_product_mixed_aia(846, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 1118, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(1118), A::sub_scaled_inputs(s.ad_value(1118), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 845, 846);
            s.store_add(919, 919, 1137);
        }

        if ((s.b[1402] && (!s.b[1415])) && (!s.b[1417])) {
            s.store_scaled_add(919, 915, 1006, (-0.5));
        }

        s.b[1419] = (s.v[37] == 2.0);
        s.v[1419] = if s.b[1419] { 1.0 } else { 0.0 };

        if (s.b[1402] && s.b[1419]) {
            s.store_scalar(939, 0.0);
        }

        if (s.b[1402] && (!s.b[1419])) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
            s.store_mul_sub_rhs(939, 914, 902, 824);
        }

        if s.b[1402] {
            s.store_add_scaled_inputs3_indices(916, 915, 1.0, 938, 1.0, 937, 1.0);
            s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);
            s.copy_ad(920, 939);
            s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 919, (-1.0), 917, (-1.0), 920, (-1.0));
        }

        s.b[1420] = (p.p61 == 3.0);
        s.v[1420] = if s.b[1420] { 1.0 } else { 0.0 };

        s.b[1421] = (p.p41 == 0.0);
        s.v[1421] = if s.b[1421] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1421]) {
            s.store_div_from_scalar(997, 3.453133e-11, 62);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1421])) {
            s.store_div_scaled_inputs_indices(997, 416, 8.85418e-12, 62, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_div_scaled_product_indices(842, 842, 415, 1.0, 62, 1.0);
            s.store_div_scaled_inputs_indices(981, 981, p.p66, 62, 1.0);
            s.store_scale(998, 62, 100000000.0);
        }

        s.b[1422] = (p.p27 > 0.0);
        s.v[1422] = if s.b[1422] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1422]) {
            s.store_div_scaled_inputs_indices(1115, 1115, p.p66, 62, 1.0);
            s.store_div_scaled_inputs_indices(1116, 1116, p.p66, 62, 1.0);
        }

        s.b[1423] = (s.v[37] == 2.0);
        s.v[1423] = if s.b[1423] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1423]) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
            s.store_scalar(1015, 0.0);
        }

        s.b[1424] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1424] = if s.b[1424] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1424]) {
            s.store_add_ad_lhs(1015, A::add_scaled_inputs_product(s.ad_value(1014), 1.0, s.ad_value(942), (-1.0), s.ad_value(405), s.ad_value(943), (-1.0)), 324);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1424])) {
            s.store_add(1015, 67, 324);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_add_scaled_inputs3_offset_indices(813, 1015, 1.0, 825, (-1.0), 841, 1.0, (-0.02));
        }

        s.b[1425] = (s.v[1015] <= 0.0);
        s.v[1425] = if s.b[1425] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1425]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (-(4.0 * 0.02)));
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1425])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1015, (4.0 * 0.02));
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_add_scaled_inputs3_indices(812, 1015, 1.0, 813, (-0.5), 843, (-0.5));
        }

        s.b[1426] = (p.p27 > 0.0);
        s.v[1426] = if s.b[1426] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {
            s.store_offset(1126, 1015, p.p1033);
            s.store_add_scaled_inputs3_offset_indices(813, 1126, 1.0, 1125, (-1.0), 841, 1.0, (-0.02));
        }

        s.b[1427] = (s.v[1126] <= 0.0);
        s.v[1427] = if s.b[1427] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && s.b[1427]) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (-(100.0 * 0.02)));
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) && (!s.b[1427])) {
            s.store_sqrt_add_scaled_square_input(843, 813, 1.0, 1126, (100.0 * 0.02));
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1426]) {
            s.store_add_scaled_inputs3_indices(1128, 1126, 1.0, 813, (-0.5), 843, (-0.5));
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div_scaled_inputs3_indices(843, 825, 1.0, 841, (-1.0), 1015, -1.0, 998, 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1428] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.v[1428] = if s.b[1428] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1428]) {
            s.store_mul_exp_rhs(999, 360, 859);
        }

        s.b[1429] = (s.v[859] <= (-100.0));
        s.v[1429] = if s.b[1429] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && s.b[1429]) {
            s.store_scale(999, 360, 3.720075976e-44);
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1428])) && (!s.b[1429])) {
            s.store_scale(999, 360, 2.688117142e43);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(1000, 62, 0.001);
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 999, (-1.0), 1000, -1.0);
            s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);
            s.store_add_scaled_inputs3_indices(999, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1430] = (s.v[999] < 1e-15);
        s.v[1430] = if s.b[1430] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1430]) {
            s.store_scalar(999, 1e-15);
        }

        s.b[1431] = (p.p27 > 0.0);
        s.v[1431] = if s.b[1431] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_div_scaled_inputs3_indices(843, 1125, 1.0, 841, (-1.0), 1126, -1.0, 998, 1.0);
            s.store_mul(859, 843, 361);
        }

        s.b[1432] = (((-100.0) < s.v[859]) && (s.v[859] < 100.0));
        s.v[1432] = if s.b[1432] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1432]) {
            s.store_mul_exp_rhs(1131, 360, 859);
        }

        s.b[1433] = (s.v[859] <= (-100.0));
        s.v[1433] = if s.b[1433] { 1.0 } else { 0.0 };

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && s.b[1433]) {
            s.store_scale(1131, 360, 3.720075976e-44);
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && (!s.b[1432])) && (!s.b[1433])) {
            s.store_scale(1131, 360, 2.688117142e43);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) {
            s.store_add_scaled_inputs3_indices(813, 360, 1.0, 1131, (-1.0), 1000, -1.0);
            s.store_sqrt_add_scaled_square_product(814, 813, 1.0, 1000, 360, 4.0);
            s.store_add_scaled_inputs3_indices(1131, 360, 1.0, 813, (-0.5), 814, (-0.5));
        }

        s.b[1434] = (s.v[1131] < 1e-15);
        s.v[1434] = if s.b[1434] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1431]) && s.b[1434]) {
            s.store_scalar(1131, 1e-15);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div(1001, 417, 999);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1001, 1.0);
            s.store_mul(1002, 845, 1001);
        }

        s.b[1435] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1435] = if s.b[1435] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1435]) {
            s.store_div(1132, 417, 1131);
            s.store_div_add_scaled_inputs_rhs_indices(845, 997, 997, 1.0, 1132, 1.0);
            s.store_mul(1133, 845, 1132);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);
        }

        s.b[1436] = (p.p27 > 0.0);
        s.v[1436] = if s.b[1436] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1436]) {
            s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_sub_rhs(938, 982, 812, 1015);
        }

        s.b[1437] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1437] = if s.b[1437] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1437]) {
            s.store_mul_sub_rhs(1123, 1135, 1128, 1126);
            s.store_add(938, 938, 1123);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_scale(843, 376, 0.5);
            s.store_add_scaled_inputs4_indices(846, 825, 1.0, 812, (-1.0), 841, -1.0, 875, -1.0);
        }

        s.b[1438] = (s.v[376] == 0.0);
        s.v[1438] = if s.b[1438] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1438]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1439] = (s.v[846] < 0.0);
        s.v[1439] = if s.b[1439] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && s.b[1439]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && (!s.b[1438])) && (!s.b[1439])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) {
            s.store_mul_ad_product_rhs(937, 982, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
        }

        s.b[1440] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1440] = if s.b[1440] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_add_scaled_inputs4_indices(846, 1125, 1.0, 1128, (-1.0), 841, -1.0, 1118, -1.0);
        }

        s.b[1441] = (s.v[376] == 0.0);
        s.v[1441] = if s.b[1441] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && s.b[1441]) {
            s.store_scalar(844, 0.0);
        }

        s.b[1442] = (s.v[846] < 0.0);
        s.v[1442] = if s.b[1442] { 1.0 } else { 0.0 };

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && s.b[1442]) {
            s.store_add_div_rhs_indices(844, 843, 846, 376);
        }

        if ((((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) && (!s.b[1441])) && (!s.b[1442])) {
            s.store_sqrt_square_add(844, 843, 846);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1423])) && s.b[1440]) {
            s.store_mul_ad_product_rhs(1124, 1135, s.ad_value(376), A::sub(s.ad_value(844), s.ad_value(843)));
            s.store_add(937, 937, 1124);
        }

        s.b[1443] = (s.v[376] <= 0.0);
        s.v[1443] = if s.b[1443] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1443]) {
            s.store_scaled_mul(936, 362, 832, 0.25);
            s.store_scale(843, 339, 0.5);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1443])) {
            s.store_mul_product3_rhs(936, 376, s.ad_value(362), s.ad_value(832), s.ad_value(376), 1.0);
            s.store_mul(843, 376, 339);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs(844, 843, 2.0, 875, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_mul_ad_rhs(1004, 832, {
                if ((1.0 + ((s.v[844] * s.v[875]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(875), 1.0, s.ad_value(936), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        s.b[1444] = (p.p27 > 0.0);
        s.v[1444] = if s.b[1444] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_add_scaled_inputs(844, 843, 2.0, 1118, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1402]) && s.b[1420]) && s.b[1444]) {
            s.store_mul_ad_rhs(1136, 832, {
                if ((1.0 + ((s.v[844] * s.v[1118]) / s.v[936])) > 1e-38) {
                    A::ln(A::offset(A::div_scaled_product(s.ad_value(844), s.ad_value(1118), 1.0, s.ad_value(936), 1.0), 1.0))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs3_indices(846, 829, 4.0, 1015, ((-1.0) * 4.0), 942, (-4.0));
            s.store_sqrt_square_offset(845, 846, 0.0001);
            s.store_scaled_add(847, 846, 845, 0.5);
            s.store_scale(998, 998, 2.0);
            s.store_div_scaled_inputs2_indices(843, 875, 1.0, 847, 1.0, 998, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_offset(844, 859, 1.0);
            s.store_div_from_scalar(999, (p.p58 * 1.9e-9), 844);
            s.store_div(1001, 417, 999);
            s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1001, 1.0);
            s.store_mul(1002, 843, 1001);
            s.store_div_scaled_product_indices(1003, 842, 1002, 1.0, 997, 1.0);
            s.store_div_scaled_product_indices(982, 981, 1002, 1.0, 997, 1.0);
        }

        s.b[1445] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1445] = if s.b[1445] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_add_scaled_inputs3_offset_indices(846, 829, 4.0, 1126, ((-1.0) * 4.0), 942, (-4.0), (p.p1033 * 4.0));
            s.store_sqrt_square_offset(845, 846, 0.0001);
            s.store_scaled_add(847, 846, 845, 0.5);
            s.store_div_scaled_inputs2_indices(843, 1118, 1.0, 847, 1.0, 998, 1.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_exp_scaled_input_ad(859, {
                if (s.v[843] > 1e-38) {
                    A::ln(s.ad_value(843))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, (p.p59 * 0.7));
        }

        if (((!s.b[1402]) && s.b[1420]) && s.b[1445]) {
            s.store_offset(844, 859, 1.0);
            s.store_div_from_scalar(1131, (p.p58 * 1.9e-9), 844);
            s.store_div(1132, 417, 1131);
            s.store_div_add_scaled_inputs_rhs_indices(843, 997, 997, 1.0, 1132, 1.0);
            s.store_mul(1133, 843, 1132);
            s.store_div_scaled_product_indices(1134, 1115, 1133, 1.0, 997, 1.0);
            s.store_div_scaled_product_indices(1135, 1116, 1133, 1.0, 997, 1.0);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_sub(844, 875, 1004);
            s.store_mul(894, 861, 333);
            s.store_div(891, 844, 894);
            s.store_offset_sub(814, 891, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(843, 814, 1.0, 891, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(877, 891, 1.0, 814, (-0.5), 843, (-0.5));
            s.store_mul(843, 894, 877);
            s.store_scaled_offset_ad(845, A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), 0.5), 1e-20, 12.0);
            s.store_div(846, 843, 845);
            s.store_mul_sub_ad_rhs(915, 1003, s.ad_value(844), A::mul_sub_from_scalar_rhs(s.ad_value(843), 0.5, s.ad_value(846)));
            s.copy_ad(916, 915);
        }

        s.b[1446] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1446] = if s.b[1446] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1446]) {
            s.store_sub(855, 1118, 1136);
            s.store_div(1129, 855, 894);
            s.store_offset_sub(814, 1129, 822, (-0.02));
            s.store_sqrt_add_scaled_square_input(1121, 814, 1.0, 1129, (4.0 * 0.02));
            s.store_add_scaled_inputs3_indices(1130, 1129, 1.0, 814, (-0.5), 1121, (-0.5));
            s.store_mul(1121, 894, 1130);
            s.store_scaled_offset_ad(1122, A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), 0.5), 1e-20, 12.0);
            s.store_div(846, 1121, 1122);
            s.store_mul_sub_ad_rhs(850, 1134, s.ad_value(855), A::mul_sub_from_scalar_rhs(s.ad_value(1121), 0.5, s.ad_value(846)));
            s.store_add(915, 915, 850);
            s.copy_ad(916, 915);
        }

        s.b[1447] = (s.v[37] == 2.0);
        s.v[1447] = if s.b[1447] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1447]) {
            s.store_scalar(1006, 0.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) {
            s.store_sub_from_scalar(850, 1.0, 894);
            s.store_mul_ad_product_rhs(1006, 982, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(877), 0.5, A::div_scaled_product(s.ad_value(843), s.ad_value(877), 1.0, s.ad_value(845), 1.0), 1.0));
        }

        s.b[1448] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1448] = if s.b[1448] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1447])) && s.b[1448]) {
            s.store_mul_ad_product_rhs(1138, 1135, s.ad_value(850), A::sub_scaled_inputs(s.ad_value(1130), 0.5, A::div_scaled_product(s.ad_value(1121), s.ad_value(1130), 1.0, s.ad_value(1122), 1.0), 1.0));
            s.store_add(1006, 1006, 1138);
        }

        s.b[1449] = (p.p129 > 0.5);
        s.v[1449] = if s.b[1449] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1449]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(919, 1003, s.ad_value(844), ((0.5) * (-1.0)), s.ad_value(843), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(843), s.ad_value(843), 0.5, s.ad_value(845), 1.0), ((-1.0) * (-1.0)), 0.0);
        }

        s.b[1450] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1450] = if s.b[1450] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && s.b[1449]) && s.b[1450]) {
            s.store_mul_add_scaled_inputs4_rhs(1137, 1134, s.ad_value(1118), ((0.5) * (-1.0)), s.ad_value(1136), (((-0.5)) * (-1.0)), s.ad_value(1121), ((0.25) * (-1.0)), A::div_scaled_product(s.ad_value(1121), s.ad_value(1121), 0.5, s.ad_value(1122), 1.0), ((-1.0) * (-1.0)));
            s.store_add(919, 919, 1137);
        }

        s.b[1451] = (p.p129 < 0.5);
        s.v[1451] = if s.b[1451] { 1.0 } else { 0.0 };

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) {
            s.store_scale(845, 845, 0.08333333333333333);
            s.store_div_scaled_inputs_mixed_ia(846, 1003, 0.5, A::square(s.ad_value(845)), 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(843), s.ad_value(843), s.ad_value(843), (2.0 * 0.06666666666666667)), (-1.0), 844, A::add_scaled_products(s.ad_value(843), s.ad_value(843), (2.0 * 0.3333333333333333), s.ad_value(844), A::sub_scaled_inputs(s.ad_value(844), 1.0, s.ad_value(843), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(919, 846, 847);
        }

        s.b[1452] = (((s.v[37] != 2.0) && (s.v[399] != 0.0)) && (p.p27 > 0.0));
        s.v[1452] = if s.b[1452] { 1.0 } else { 0.0 };

        if (((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && s.b[1451]) && s.b[1452]) {
            s.store_scale(1122, 1122, 0.08333333333333333);
            s.store_div_scaled_inputs_mixed_ia(846, 1134, 0.5, A::square(s.ad_value(1122)), 1.0);
            s.store_add_scaled_product_mixed_aia(847, A::mul3_scaled_output(s.ad_value(1121), s.ad_value(1121), s.ad_value(1121), (2.0 * 0.06666666666666667)), (-1.0), 855, A::add_scaled_products(s.ad_value(1121), s.ad_value(1121), (2.0 * 0.3333333333333333), s.ad_value(855), A::sub_scaled_inputs(s.ad_value(855), 1.0, s.ad_value(1121), (4.0 * 0.3333333333333333)), 1.0), 1.0);
            s.store_mul_neg_lhs(1137, 846, 847);
            s.store_add(919, 919, 1137);
        }

        if ((((!s.b[1402]) && s.b[1420]) && (!s.b[1449])) && (!s.b[1451])) {
            s.store_scale(919, 916, (-0.5));
        }

        s.b[1453] = (s.v[37] == 2.0);
        s.v[1453] = if s.b[1453] { 1.0 } else { 0.0 };

        if (((!s.b[1402]) && s.b[1420]) && s.b[1453]) {
            s.store_scalar(939, 0.0);
        }

        if (((!s.b[1402]) && s.b[1420]) && (!s.b[1453])) {
            s.store_scale(914, 263, (p.p361 * (s.v[913] * ((((s.v[332] / p.p23) * p.p3) * s.v[366]) + p.p29))));
            s.store_mul_sub_rhs(939, 914, 902, 824);
        }

        if ((!s.b[1402]) && s.b[1420]) {
            s.store_add_scaled_inputs4_indices(916, 916, 1.0, 938, 1.0, 937, 1.0, 1006, -1.0);
            s.store_add_scaled_inputs4_indices(917, 1006, 1.0, 938, (-1.0), 937, -1.0, 939, -1.0);
            s.copy_ad(920, 939);
            s.store_add_scaled_inputs4_indices(918, 916, (-1.0), 917, (-1.0), 920, (-1.0), 919, (-1.0));
        }

        if ((!s.b[1402]) && (!s.b[1420])) {
            s.store_scalar(938, 0.0);
            s.store_scalar(937, 0.0);
            s.store_scalar(920, 0.0);
            s.store_scalar(917, 0.0);
            s.store_scalar(919, 0.0);
            s.store_scalar(918, 0.0);
            s.store_scalar(916, 0.0);
        }

        s.b[1454] = (s.v[37] == 2.0);
        s.v[1454] = if s.b[1454] { 1.0 } else { 0.0 };

        if s.b[1454] {
            s.store_scalar(909, 0.0);
            s.store_scalar(910, 0.0);
        }

        if (!s.b[1454]) {
            s.copy_ad(815, 48);
            s.store_scalar(980, (-p.p363));
            s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);
            s.store_scalar(816, p.p183);
            s.store_scalar(976, ((((p.p185 * s.v[350]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(979, 976, p.p362);
            s.store_add_scaled_product_right_sub(976, 976, 1.0, 979, 409, 429, 1.0);
            s.store_scalar(977, ((((p.p186 * s.v[349]) * p.p155) * p.p3) / 1e-7));
            s.store_scale(978, 977, p.p364);
            s.store_add_scaled_product_right_sub(977, 977, 1.0, 978, 409, 429, 1.0);
            s.store_scale(994, 815, 0.9);
        }

        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1087] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1087)
                }
            }, 815);
        }

        s.b[1455] = (s.v[816] == 0.5);
        s.v[1455] = if s.b[1455] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1455]) {
            s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));
        }

        if ((!s.b[1454]) && (!s.b[1455])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

        if (!s.b[1454]) {
            s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.b[1456] = (s.v[1087] > s.v[994]);
        s.v[1456] = if s.b[1456] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1456]) {
            s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1087, 994, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(910, 987, (p.p351 * p.p3), 976, 846, 1.0);
            s.copy_ad(815, 41);
            s.store_scalar(980, (-p.p365));
            s.store_add_scaled_product_right_sub(815, 815, 1.0, 980, 409, 429, 1.0);
            s.store_scalar(816, p.p184);
            s.store_scale(994, 815, 0.9);
        }

        if (!s.b[1454]) {
            s.store_sub_from_scalar_div_mixed_ai(811, 1.0, {
                if (s.v[1088] > s.v[994]) {
                    s.ad_value(994)
                } else {
                    s.ad_value(1088)
                }
            }, 815);
        }

        s.b[1457] = (s.v[816] == 0.5);
        s.v[1457] = if s.b[1457] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1457]) {
            s.store_div_from_scalar_sqrt_ad(858, 1.0, s.ad_value(811));
        }

        if ((!s.b[1454]) && (!s.b[1457])) {
            s.store_exp_mul_scaled_lhs_mixed_ia(858, 816, -1.0, {
                if (s.v[811] > 1e-38) {
                    A::ln(s.ad_value(811))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            });
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1454]) {
            s.store_div_ad(846, A::mul_sub_from_scalar_lhs(1.0, A::mul(s.ad_value(811), s.ad_value(858)), s.ad_value(815)), A::sub_from_scalar(1.0, s.ad_value(816)));
        }

        s.b[1458] = (s.v[1088] > s.v[994]);
        s.v[1458] = if s.b[1458] { 1.0 } else { 0.0 };

        if ((!s.b[1454]) && s.b[1458]) {
            s.store_add_scaled_product_right_sub(846, 846, 1.0, 858, 1088, 994, 1.0);
        }

        if (!s.b[1454]) {
            s.store_add_scaled_product_indices(909, 988, (p.p351 * p.p3), 977, 846, 1.0);
        }

        s.store_scale(853, 897, (-p.p37));

        s.store_scaled_sub(854, 819, 897, p.p37);

        s.b[1459] = (s.v[43] != 0.0);
        s.v[1459] = if s.b[1459] { 1.0 } else { 0.0 };

        s.b[1460] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[1460] = if s.b[1460] { 1.0 } else { 0.0 };

        s.b[1461] = (s.v[853] < s.v[322]);
        s.v[1461] = if s.b[1461] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1460]) && s.b[1461]) {
            s.store_scaled_sub(86, 853, 322, s.v[52]);
        }

        s.b[1462] = (s.v[853] < s.v[175]);
        s.v[1462] = if s.b[1462] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1460]) && (!s.b[1461])) && s.b[1462]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(86, 843, s.v[52], A::mul_scaled_lhs(s.ad_value(176), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1463] = (s.v[853] < s.v[323]);
        s.v[1463] = if s.b[1463] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && s.b[1463]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_add_ad(86, A::add_scaled_product(s.ad_value(56), 1.0, s.ad_value(53), s.ad_value(853), 1.0), A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1460]) && (!s.b[1461])) && (!s.b[1462])) && (!s.b[1463])) {
            s.store_add_scaled_product_indices(86, 56, 1.0, 53, 853, 1.0);
        }

        s.b[1464] = (s.v[853] < s.v[323]);
        s.v[1464] = if s.b[1464] { 1.0 } else { 0.0 };

        if ((s.b[1459] && (!s.b[1460])) && s.b[1464]) {
            s.store_mul_sub_rhs(86, 53, 853, 323);
        }

        s.b[1465] = (s.v[853] < s.v[175]);
        s.v[1465] = if s.b[1465] { 1.0 } else { 0.0 };

        if (((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && s.b[1465]) {
            s.store_sub(843, 853, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(86, 843, s.ad_value(53), 1.0, s.ad_value(176), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1466] = (s.v[853] < s.v[322]);
        s.v[1466] = if s.b[1466] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && s.b[1466]) {
            s.store_sub(843, 853, 322);
            s.store_square(844, 843);
            s.store_add_scaled_inputs3_mixed_iia(86, 853, s.v[52], 56, 1.0, A::mul3_scaled_output(s.ad_value(177), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1459] && (!s.b[1460])) && (!s.b[1464])) && (!s.b[1465])) && (!s.b[1466])) {
            s.store_add_scaled_inputs(86, 853, s.v[52], 56, 1.0);
        }

        s.b[1467] = (((s.v[109] > 0.0) && (p.p37 > 0.0)) || ((s.v[109] < 0.0) && (p.p37 < 0.0)));
        s.v[1467] = if s.b[1467] { 1.0 } else { 0.0 };

        s.b[1468] = (s.v[854] < s.v[322]);
        s.v[1468] = if s.b[1468] { 1.0 } else { 0.0 };

        if ((s.b[1459] && s.b[1467]) && s.b[1468]) {
            s.store_scaled_sub(87, 854, 322, s.v[54]);
        }

        s.b[1469] = (s.v[854] < s.v[175]);
        s.v[1469] = if s.b[1469] { 1.0 } else { 0.0 };

        if (((s.b[1459] && s.b[1467]) && (!s.b[1468])) && s.b[1469]) {
            s.store_sub(843, 854, 322);
            s.store_square(844, 843);
            s.store_mul_sub_from_scalar_ad_rhs(87, 843, s.v[54], A::mul_scaled_lhs(s.ad_value(178), 1.0 / (3.0), s.ad_value(844)));
        }

        s.b[1470] = (s.v[854] < s.v[323]);
        s.v[1470] = if s.b[1470] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && s.b[1470]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_add_ad(87, A::add_scaled_product(s.ad_value(57), 1.0, s.ad_value(55), s.ad_value(854), 1.0), A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)));
        }

        if ((((s.b[1459] && s.b[1467]) && (!s.b[1468])) && (!s.b[1469])) && (!s.b[1470])) {
            s.store_add_scaled_product_indices(87, 57, 1.0, 55, 854, 1.0);
        }

        s.b[1471] = (s.v[854] < s.v[323]);
        s.v[1471] = if s.b[1471] { 1.0 } else { 0.0 };

        if ((s.b[1459] && (!s.b[1467])) && s.b[1471]) {
            s.store_mul_sub_rhs(87, 55, 854, 323);
        }

        s.b[1472] = (s.v[854] < s.v[175]);
        s.v[1472] = if s.b[1472] { 1.0 } else { 0.0 };

        if (((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && s.b[1472]) {
            s.store_sub(843, 854, 323);
            s.store_square(844, 843);
            s.store_mul_add_scaled_product_rhs(87, 843, s.ad_value(55), 1.0, s.ad_value(178), s.ad_value(844), (-1.0 / (3.0)));
        }

        s.b[1473] = (s.v[854] < s.v[322]);
        s.v[1473] = if s.b[1473] { 1.0 } else { 0.0 };

        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && s.b[1473]) {
            s.store_sub(843, 854, 322);
            s.store_square(844, 843);
            s.store_add_scaled_inputs3_mixed_iia(87, 854, s.v[54], 57, 1.0, A::mul3_scaled_output(s.ad_value(179), s.ad_value(843), s.ad_value(844), 1.0 / (3.0)), 1.0);
        }

        if ((((s.b[1459] && (!s.b[1467])) && (!s.b[1471])) && (!s.b[1472])) && (!s.b[1473])) {
            s.store_add_scaled_inputs(87, 854, s.v[54], 57, 1.0);
        }

        if (!s.b[1459]) {
            s.store_scale(86, 853, s.v[52]);
            s.store_scale(87, 854, s.v[54]);
        }

        s.store_add_scaled_product_indices(86, 86, 1.0, 58, 853, 1.0);

        s.store_add_scaled_product_indices(87, 87, 1.0, 59, 854, 1.0);

        s.b[1474] = (p.p39 == 3.0);
        s.v[1474] = if s.b[1474] { 1.0 } else { 0.0 };

        if s.b[1474] {
            s.store_offset(843, 1019, 0.02);
        }

        if (!s.b[1474]) {
            s.store_offset(843, 820, 0.02);
        }

        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 237, s.v[349]);

        s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));

        s.b[1475] = (p.p39 == 3.0);
        s.v[1475] = if s.b[1475] { 1.0 } else { 0.0 };

        if s.b[1475] {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 1019, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1475]) {
            s.store_add_scaled_products_mixed_aiia(895, A::add(s.ad_value(335), s.ad_value(846)), 820, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1476] = (p.p39 == 3.0);
        s.v[1476] = if s.b[1476] { 1.0 } else { 0.0 };

        if s.b[1476] {
            s.store_offset(843, 1018, 0.02);
        }

        if (!s.b[1476]) {
            s.store_offset(843, 821, 0.02);
        }

        s.store_sqrt_square_offset(844, 843, (4.0 * 0.02));

        s.store_scaled_sub(845, 843, 844, 0.5);

        s.store_scale(846, 236, s.v[350]);

        s.store_sqrt_sub_from_scalar_ad(847, 1.0, A::div_scaled_inputs(s.ad_value(845), 4.0, s.ad_value(238), 1.0));

        s.b[1477] = (p.p39 == 3.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        if s.b[1477] {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 1018, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        if (!s.b[1477]) {
            s.store_add_scaled_products_mixed_aiia(896, A::add(s.ad_value(334), s.ad_value(846)), 821, 1.0, 846, A::add_scaled_offset_product_rhs(s.ad_value(845), 1.0, s.ad_value(238), s.ad_value(847), (-1.0), 0.5), (-1.0));
        }

        s.b[1478] = (p.p3 != 1.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if s.b[1478] {
            s.store_scale(895, 895, p.p3);
            s.store_scale(896, 896, p.p3);
        }

        s.b[1505] = (p.p223 == 0.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        s.b[1506] = (p.p223 == 1.0);
        s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };

        s.b[1507] = (p.p223 == 2.0);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        s.b[1508] = (p.p223 == 3.0);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if (s.b[1506] && (!s.b[1505])) {
            s.store_add_scaled_inputs3_indices(843, 83, 1.0, 84, 1.0, 85, 1.0);
            s.store_square(843, 843);
            s.store_div_scaled_inputs_indices(1486, 946, 2.0, 75, 1.0);
            s.store_div_scaled_inputs_indices(848, 72, 1.0, 1486, s.v[327]);
            s.store_square(848, 848);
            s.store_offset_scaled(1487, 848, (((p.p227 * s.v[327])) * (p.p229)), p.p229);
            s.store_add_scaled_product_right_ad(844, 84, 1.0, 1487, A::add(s.ad_value(83), s.ad_value(85)), 1.0);
            s.store_div_scaled_product_indices(845, 844, 844, 1.0, 78, 1.0);
        }

        if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
            s.store_sub_from_scalar_scaled_mul(1491, 1.0, 77, 76, 1.0);
            s.store_sub_from_scalar(843, 1.0, 1491);
            s.store_offset(844, 1491, 1.0);
            s.store_add_ad_rhs(845, 844, A::div_scaled_product_offset_denominator(s.ad_value(74), s.ad_value(49), 2.0, s.ad_value(72), 1e-10, 1.0));
            s.store_offset_scaled_div(1495, 77, 838, s.v[892], s.v[892]);
            s.store_div_from_scalar(849, s.v[892], 1495);
            s.store_square(846, 845);
            s.store_square(847, 843);
            s.store_square(848, 846);
            s.store_div(850, 843, 845);
            s.store_div(851, 72, 838);
            s.store_square(851, 851);
            s.store_offset_scaled(1487, 851, (((p.p227 * s.v[892])) * (p.p229)), p.p229);
            s.store_scale(1501, 396, (p.p3 * (s.v[332] * s.v[331])));
        }

        s.b[1548] = (s.v[398] > 0.0);
        s.v[1548] = if s.b[1548] { 1.0 } else { 0.0 };

        if s.b[1548] {
            s.store_scale(92, 918, p.p37);
            s.store_scale(93, 919, p.p37);
        }

        if (!s.b[1548]) {
            s.store_scale(93, 918, p.p37);
            s.store_scale(92, 919, p.p37);
        }

        s.b[1553] = (p.p39 == 3.0);
        s.v[1553] = if s.b[1553] { 1.0 } else { 0.0 };

        s.b[1559] = ((p.p36 == 1.0) && (p.p14 != 0.0));
        s.v[1559] = if s.b[1559] { 1.0 } else { 0.0 };

        s.b[1560] = ((p.p35 != 0.0) && (!true));
        s.v[1560] = if s.b[1560] { 1.0 } else { 0.0 };

        s.b[1561] = true;
        s.v[1561] = if s.b[1561] { 1.0 } else { 0.0 };

        s.b[1562] = true;
        s.v[1562] = if s.b[1562] { 1.0 } else { 0.0 };

        s.b[1563] = (p.p430 == 2.0);
        s.v[1563] = if s.b[1563] { 1.0 } else { 0.0 };

        s.b[1564] = (p.p430 == 2.0);
        s.v[1564] = if s.b[1564] { 1.0 } else { 0.0 };

        s.copy_ad(426, 916);

        s.copy_ad(427, 918);

        s.copy_ad(428, 919);

        s.store_add(425, 896, 895);

        s.store_sub(918, 427, 895);

        s.store_sub(919, 428, 896);

        s.store_add(916, 426, 425);

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv13 = ctx.node_voltage(nodes[13]);
        let (eq11_e1314, eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13, eq11_e1314_d_b0, eq11_e1314_d_b1, eq11_e1314_d_b2, eq11_e1314_d_b3, eq11_e1314_d_b4, eq11_e1314_d_b5, eq11_e1314_d_b6, eq11_e1314_d_b7, eq11_e1314_d_b8, eq11_e1314_d_b9, eq11_e1314_d_b10, eq11_e1314_d_b11, eq11_e1314_d_b12, eq11_e1314_d_b13, eq11_e1314_d_b14, eq11_e1314_d_b15, eq11_e1314_d_b16, eq11_e1314_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq11_e1308: f64 = (p.p32 * (nv13 - 0.0));
        let eq11_e1310: f64 = (eq11_e1308 * s.v[1497]);
        let eq11_e1310_d_n0: f64 = (eq11_e1308 * s.dn[1497][0]);
        let eq11_e1310_d_n1: f64 = (eq11_e1308 * s.dn[1497][1]);
        let eq11_e1310_d_n2: f64 = (eq11_e1308 * s.dn[1497][2]);
        let eq11_e1310_d_n3: f64 = (eq11_e1308 * s.dn[1497][3]);
        let eq11_e1310_d_n4: f64 = (eq11_e1308 * s.dn[1497][4]);
        let eq11_e1310_d_n5: f64 = (eq11_e1308 * s.dn[1497][5]);
        let eq11_e1310_d_n6: f64 = (eq11_e1308 * s.dn[1497][6]);
        let eq11_e1310_d_n7: f64 = (eq11_e1308 * s.dn[1497][7]);
        let eq11_e1310_d_n8: f64 = (eq11_e1308 * s.dn[1497][8]);
        let eq11_e1310_d_n9: f64 = (eq11_e1308 * s.dn[1497][9]);
        let eq11_e1310_d_n10: f64 = (eq11_e1308 * s.dn[1497][10]);
        let eq11_e1310_d_n11: f64 = (eq11_e1308 * s.dn[1497][11]);
        let eq11_e1310_d_n12: f64 = (eq11_e1308 * s.dn[1497][12]);
        let eq11_e1310_d_n13: f64 = ((p.p32 * s.v[1497]) + (eq11_e1308 * s.dn[1497][13]));
        let eq11_e1310_d_b0: f64 = (eq11_e1308 * s.db[1497][0]);
        let eq11_e1310_d_b1: f64 = (eq11_e1308 * s.db[1497][1]);
        let eq11_e1310_d_b2: f64 = (eq11_e1308 * s.db[1497][2]);
        let eq11_e1310_d_b3: f64 = (eq11_e1308 * s.db[1497][3]);
        let eq11_e1310_d_b4: f64 = (eq11_e1308 * s.db[1497][4]);
        let eq11_e1310_d_b5: f64 = (eq11_e1308 * s.db[1497][5]);
        let eq11_e1310_d_b6: f64 = (eq11_e1308 * s.db[1497][6]);
        let eq11_e1310_d_b7: f64 = (eq11_e1308 * s.db[1497][7]);
        let eq11_e1310_d_b8: f64 = (eq11_e1308 * s.db[1497][8]);
        let eq11_e1310_d_b9: f64 = (eq11_e1308 * s.db[1497][9]);
        let eq11_e1310_d_b10: f64 = (eq11_e1308 * s.db[1497][10]);
        let eq11_e1310_d_b11: f64 = (eq11_e1308 * s.db[1497][11]);
        let eq11_e1310_d_b12: f64 = (eq11_e1308 * s.db[1497][12]);
        let eq11_e1310_d_b13: f64 = (eq11_e1308 * s.db[1497][13]);
        let eq11_e1310_d_b14: f64 = (eq11_e1308 * s.db[1497][14]);
        let eq11_e1310_d_b15: f64 = (eq11_e1308 * s.db[1497][15]);
        let eq11_e1310_d_b16: f64 = (eq11_e1308 * s.db[1497][16]);
        let eq11_e1310_d_b17: f64 = (eq11_e1308 * s.db[1497][17]);
        let eq11_e1312: f64 = (eq11_e1310 * p.p226);
        let eq11_e1312_d_n0: f64 = (eq11_e1310_d_n0 * p.p226);
        let eq11_e1312_d_n1: f64 = (eq11_e1310_d_n1 * p.p226);
        let eq11_e1312_d_n2: f64 = (eq11_e1310_d_n2 * p.p226);
        let eq11_e1312_d_n3: f64 = (eq11_e1310_d_n3 * p.p226);
        let eq11_e1312_d_n4: f64 = (eq11_e1310_d_n4 * p.p226);
        let eq11_e1312_d_n5: f64 = (eq11_e1310_d_n5 * p.p226);
        let eq11_e1312_d_n6: f64 = (eq11_e1310_d_n6 * p.p226);
        let eq11_e1312_d_n7: f64 = (eq11_e1310_d_n7 * p.p226);
        let eq11_e1312_d_n8: f64 = (eq11_e1310_d_n8 * p.p226);
        let eq11_e1312_d_n9: f64 = (eq11_e1310_d_n9 * p.p226);
        let eq11_e1312_d_n10: f64 = (eq11_e1310_d_n10 * p.p226);
        let eq11_e1312_d_n11: f64 = (eq11_e1310_d_n11 * p.p226);
        let eq11_e1312_d_n12: f64 = (eq11_e1310_d_n12 * p.p226);
        let eq11_e1312_d_n13: f64 = (eq11_e1310_d_n13 * p.p226);
        let eq11_e1312_d_b0: f64 = (eq11_e1310_d_b0 * p.p226);
        let eq11_e1312_d_b1: f64 = (eq11_e1310_d_b1 * p.p226);
        let eq11_e1312_d_b2: f64 = (eq11_e1310_d_b2 * p.p226);
        let eq11_e1312_d_b3: f64 = (eq11_e1310_d_b3 * p.p226);
        let eq11_e1312_d_b4: f64 = (eq11_e1310_d_b4 * p.p226);
        let eq11_e1312_d_b5: f64 = (eq11_e1310_d_b5 * p.p226);
        let eq11_e1312_d_b6: f64 = (eq11_e1310_d_b6 * p.p226);
        let eq11_e1312_d_b7: f64 = (eq11_e1310_d_b7 * p.p226);
        let eq11_e1312_d_b8: f64 = (eq11_e1310_d_b8 * p.p226);
        let eq11_e1312_d_b9: f64 = (eq11_e1310_d_b9 * p.p226);
        let eq11_e1312_d_b10: f64 = (eq11_e1310_d_b10 * p.p226);
        let eq11_e1312_d_b11: f64 = (eq11_e1310_d_b11 * p.p226);
        let eq11_e1312_d_b12: f64 = (eq11_e1310_d_b12 * p.p226);
        let eq11_e1312_d_b13: f64 = (eq11_e1310_d_b13 * p.p226);
        let eq11_e1312_d_b14: f64 = (eq11_e1310_d_b14 * p.p226);
        let eq11_e1312_d_b15: f64 = (eq11_e1310_d_b15 * p.p226);
        let eq11_e1312_d_b16: f64 = (eq11_e1310_d_b16 * p.p226);
        let eq11_e1312_d_b17: f64 = (eq11_e1310_d_b17 * p.p226);
        (eq11_e1312, eq11_e1312_d_n0, eq11_e1312_d_n1, eq11_e1312_d_n2, eq11_e1312_d_n3, eq11_e1312_d_n4, eq11_e1312_d_n5, eq11_e1312_d_n6, eq11_e1312_d_n7, eq11_e1312_d_n8, eq11_e1312_d_n9, eq11_e1312_d_n10, eq11_e1312_d_n11, eq11_e1312_d_n12, eq11_e1312_d_n13, eq11_e1312_d_b0, eq11_e1312_d_b1, eq11_e1312_d_b2, eq11_e1312_d_b3, eq11_e1312_d_b4, eq11_e1312_d_b5, eq11_e1312_d_b6, eq11_e1312_d_b7, eq11_e1312_d_b8, eq11_e1312_d_b9, eq11_e1312_d_b10, eq11_e1312_d_b11, eq11_e1312_d_b12, eq11_e1312_d_b13, eq11_e1312_d_b14, eq11_e1312_d_b15, eq11_e1312_d_b16, eq11_e1312_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1314;
        let eq11_node_derivatives: [f64; 14] = [eq11_e1314_d_n0, eq11_e1314_d_n1, eq11_e1314_d_n2, eq11_e1314_d_n3, eq11_e1314_d_n4, eq11_e1314_d_n5, eq11_e1314_d_n6, eq11_e1314_d_n7, eq11_e1314_d_n8, eq11_e1314_d_n9, eq11_e1314_d_n10, eq11_e1314_d_n11, eq11_e1314_d_n12, eq11_e1314_d_n13];
        let eq11_branch_derivatives: [f64; 18] = [eq11_e1314_d_b0, eq11_e1314_d_b1, eq11_e1314_d_b2, eq11_e1314_d_b3, eq11_e1314_d_b4, eq11_e1314_d_b5, eq11_e1314_d_b6, eq11_e1314_d_b7, eq11_e1314_d_b8, eq11_e1314_d_b9, eq11_e1314_d_b10, eq11_e1314_d_b11, eq11_e1314_d_b12, eq11_e1314_d_b13, eq11_e1314_d_b14, eq11_e1314_d_b15, eq11_e1314_d_b16, eq11_e1314_d_b17];
        stamper.stamp_current_dense_local(
            Some(13),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let (eq13_e1356, eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13, eq13_e1356_d_b0, eq13_e1356_d_b1, eq13_e1356_d_b2, eq13_e1356_d_b3, eq13_e1356_d_b4, eq13_e1356_d_b5, eq13_e1356_d_b6, eq13_e1356_d_b7, eq13_e1356_d_b8, eq13_e1356_d_b9, eq13_e1356_d_b10, eq13_e1356_d_b11, eq13_e1356_d_b12, eq13_e1356_d_b13, eq13_e1356_d_b14, eq13_e1356_d_b15, eq13_e1356_d_b16, eq13_e1356_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq13_e1348: f64 = (p.p32 * s.v[1498]);
        let eq13_e1348_d_n0: f64 = (p.p32 * s.dn[1498][0]);
        let eq13_e1348_d_n1: f64 = (p.p32 * s.dn[1498][1]);
        let eq13_e1348_d_n2: f64 = (p.p32 * s.dn[1498][2]);
        let eq13_e1348_d_n3: f64 = (p.p32 * s.dn[1498][3]);
        let eq13_e1348_d_n4: f64 = (p.p32 * s.dn[1498][4]);
        let eq13_e1348_d_n5: f64 = (p.p32 * s.dn[1498][5]);
        let eq13_e1348_d_n6: f64 = (p.p32 * s.dn[1498][6]);
        let eq13_e1348_d_n7: f64 = (p.p32 * s.dn[1498][7]);
        let eq13_e1348_d_n8: f64 = (p.p32 * s.dn[1498][8]);
        let eq13_e1348_d_n9: f64 = (p.p32 * s.dn[1498][9]);
        let eq13_e1348_d_n10: f64 = (p.p32 * s.dn[1498][10]);
        let eq13_e1348_d_n11: f64 = (p.p32 * s.dn[1498][11]);
        let eq13_e1348_d_n12: f64 = (p.p32 * s.dn[1498][12]);
        let eq13_e1348_d_n13: f64 = (p.p32 * s.dn[1498][13]);
        let eq13_e1348_d_b0: f64 = (p.p32 * s.db[1498][0]);
        let eq13_e1348_d_b1: f64 = (p.p32 * s.db[1498][1]);
        let eq13_e1348_d_b2: f64 = (p.p32 * s.db[1498][2]);
        let eq13_e1348_d_b3: f64 = (p.p32 * s.db[1498][3]);
        let eq13_e1348_d_b4: f64 = (p.p32 * s.db[1498][4]);
        let eq13_e1348_d_b5: f64 = (p.p32 * s.db[1498][5]);
        let eq13_e1348_d_b6: f64 = (p.p32 * s.db[1498][6]);
        let eq13_e1348_d_b7: f64 = (p.p32 * s.db[1498][7]);
        let eq13_e1348_d_b8: f64 = (p.p32 * s.db[1498][8]);
        let eq13_e1348_d_b9: f64 = (p.p32 * s.db[1498][9]);
        let eq13_e1348_d_b10: f64 = (p.p32 * s.db[1498][10]);
        let eq13_e1348_d_b11: f64 = (p.p32 * s.db[1498][11]);
        let eq13_e1348_d_b12: f64 = (p.p32 * s.db[1498][12]);
        let eq13_e1348_d_b13: f64 = (p.p32 * s.db[1498][13]);
        let eq13_e1348_d_b14: f64 = (p.p32 * s.db[1498][14]);
        let eq13_e1348_d_b15: f64 = (p.p32 * s.db[1498][15]);
        let eq13_e1348_d_b16: f64 = (p.p32 * s.db[1498][16]);
        let eq13_e1348_d_b17: f64 = (p.p32 * s.db[1498][17]);
        let eq13_e1350: f64 = (eq13_e1348 * (nv13 - 0.0));
        let eq13_e1350_d_n0: f64 = (eq13_e1348_d_n0 * (nv13 - 0.0));
        let eq13_e1350_d_n1: f64 = (eq13_e1348_d_n1 * (nv13 - 0.0));
        let eq13_e1350_d_n2: f64 = (eq13_e1348_d_n2 * (nv13 - 0.0));
        let eq13_e1350_d_n3: f64 = (eq13_e1348_d_n3 * (nv13 - 0.0));
        let eq13_e1350_d_n4: f64 = (eq13_e1348_d_n4 * (nv13 - 0.0));
        let eq13_e1350_d_n5: f64 = (eq13_e1348_d_n5 * (nv13 - 0.0));
        let eq13_e1350_d_n6: f64 = (eq13_e1348_d_n6 * (nv13 - 0.0));
        let eq13_e1350_d_n7: f64 = (eq13_e1348_d_n7 * (nv13 - 0.0));
        let eq13_e1350_d_n8: f64 = (eq13_e1348_d_n8 * (nv13 - 0.0));
        let eq13_e1350_d_n9: f64 = (eq13_e1348_d_n9 * (nv13 - 0.0));
        let eq13_e1350_d_n10: f64 = (eq13_e1348_d_n10 * (nv13 - 0.0));
        let eq13_e1350_d_n11: f64 = (eq13_e1348_d_n11 * (nv13 - 0.0));
        let eq13_e1350_d_n12: f64 = (eq13_e1348_d_n12 * (nv13 - 0.0));
        let eq13_e1350_d_n13: f64 = ((eq13_e1348_d_n13 * (nv13 - 0.0)) + eq13_e1348);
        let eq13_e1350_d_b0: f64 = (eq13_e1348_d_b0 * (nv13 - 0.0));
        let eq13_e1350_d_b1: f64 = (eq13_e1348_d_b1 * (nv13 - 0.0));
        let eq13_e1350_d_b2: f64 = (eq13_e1348_d_b2 * (nv13 - 0.0));
        let eq13_e1350_d_b3: f64 = (eq13_e1348_d_b3 * (nv13 - 0.0));
        let eq13_e1350_d_b4: f64 = (eq13_e1348_d_b4 * (nv13 - 0.0));
        let eq13_e1350_d_b5: f64 = (eq13_e1348_d_b5 * (nv13 - 0.0));
        let eq13_e1350_d_b6: f64 = (eq13_e1348_d_b6 * (nv13 - 0.0));
        let eq13_e1350_d_b7: f64 = (eq13_e1348_d_b7 * (nv13 - 0.0));
        let eq13_e1350_d_b8: f64 = (eq13_e1348_d_b8 * (nv13 - 0.0));
        let eq13_e1350_d_b9: f64 = (eq13_e1348_d_b9 * (nv13 - 0.0));
        let eq13_e1350_d_b10: f64 = (eq13_e1348_d_b10 * (nv13 - 0.0));
        let eq13_e1350_d_b11: f64 = (eq13_e1348_d_b11 * (nv13 - 0.0));
        let eq13_e1350_d_b12: f64 = (eq13_e1348_d_b12 * (nv13 - 0.0));
        let eq13_e1350_d_b13: f64 = (eq13_e1348_d_b13 * (nv13 - 0.0));
        let eq13_e1350_d_b14: f64 = (eq13_e1348_d_b14 * (nv13 - 0.0));
        let eq13_e1350_d_b15: f64 = (eq13_e1348_d_b15 * (nv13 - 0.0));
        let eq13_e1350_d_b16: f64 = (eq13_e1348_d_b16 * (nv13 - 0.0));
        let eq13_e1350_d_b17: f64 = (eq13_e1348_d_b17 * (nv13 - 0.0));
        let eq13_e1352: f64 = (eq13_e1350 * s.v[1497]);
        let eq13_e1352_d_n0: f64 = ((eq13_e1350_d_n0 * s.v[1497]) + (eq13_e1350 * s.dn[1497][0]));
        let eq13_e1352_d_n1: f64 = ((eq13_e1350_d_n1 * s.v[1497]) + (eq13_e1350 * s.dn[1497][1]));
        let eq13_e1352_d_n2: f64 = ((eq13_e1350_d_n2 * s.v[1497]) + (eq13_e1350 * s.dn[1497][2]));
        let eq13_e1352_d_n3: f64 = ((eq13_e1350_d_n3 * s.v[1497]) + (eq13_e1350 * s.dn[1497][3]));
        let eq13_e1352_d_n4: f64 = ((eq13_e1350_d_n4 * s.v[1497]) + (eq13_e1350 * s.dn[1497][4]));
        let eq13_e1352_d_n5: f64 = ((eq13_e1350_d_n5 * s.v[1497]) + (eq13_e1350 * s.dn[1497][5]));
        let eq13_e1352_d_n6: f64 = ((eq13_e1350_d_n6 * s.v[1497]) + (eq13_e1350 * s.dn[1497][6]));
        let eq13_e1352_d_n7: f64 = ((eq13_e1350_d_n7 * s.v[1497]) + (eq13_e1350 * s.dn[1497][7]));
        let eq13_e1352_d_n8: f64 = ((eq13_e1350_d_n8 * s.v[1497]) + (eq13_e1350 * s.dn[1497][8]));
        let eq13_e1352_d_n9: f64 = ((eq13_e1350_d_n9 * s.v[1497]) + (eq13_e1350 * s.dn[1497][9]));
        let eq13_e1352_d_n10: f64 = ((eq13_e1350_d_n10 * s.v[1497]) + (eq13_e1350 * s.dn[1497][10]));
        let eq13_e1352_d_n11: f64 = ((eq13_e1350_d_n11 * s.v[1497]) + (eq13_e1350 * s.dn[1497][11]));
        let eq13_e1352_d_n12: f64 = ((eq13_e1350_d_n12 * s.v[1497]) + (eq13_e1350 * s.dn[1497][12]));
        let eq13_e1352_d_n13: f64 = ((eq13_e1350_d_n13 * s.v[1497]) + (eq13_e1350 * s.dn[1497][13]));
        let eq13_e1352_d_b0: f64 = ((eq13_e1350_d_b0 * s.v[1497]) + (eq13_e1350 * s.db[1497][0]));
        let eq13_e1352_d_b1: f64 = ((eq13_e1350_d_b1 * s.v[1497]) + (eq13_e1350 * s.db[1497][1]));
        let eq13_e1352_d_b2: f64 = ((eq13_e1350_d_b2 * s.v[1497]) + (eq13_e1350 * s.db[1497][2]));
        let eq13_e1352_d_b3: f64 = ((eq13_e1350_d_b3 * s.v[1497]) + (eq13_e1350 * s.db[1497][3]));
        let eq13_e1352_d_b4: f64 = ((eq13_e1350_d_b4 * s.v[1497]) + (eq13_e1350 * s.db[1497][4]));
        let eq13_e1352_d_b5: f64 = ((eq13_e1350_d_b5 * s.v[1497]) + (eq13_e1350 * s.db[1497][5]));
        let eq13_e1352_d_b6: f64 = ((eq13_e1350_d_b6 * s.v[1497]) + (eq13_e1350 * s.db[1497][6]));
        let eq13_e1352_d_b7: f64 = ((eq13_e1350_d_b7 * s.v[1497]) + (eq13_e1350 * s.db[1497][7]));
        let eq13_e1352_d_b8: f64 = ((eq13_e1350_d_b8 * s.v[1497]) + (eq13_e1350 * s.db[1497][8]));
        let eq13_e1352_d_b9: f64 = ((eq13_e1350_d_b9 * s.v[1497]) + (eq13_e1350 * s.db[1497][9]));
        let eq13_e1352_d_b10: f64 = ((eq13_e1350_d_b10 * s.v[1497]) + (eq13_e1350 * s.db[1497][10]));
        let eq13_e1352_d_b11: f64 = ((eq13_e1350_d_b11 * s.v[1497]) + (eq13_e1350 * s.db[1497][11]));
        let eq13_e1352_d_b12: f64 = ((eq13_e1350_d_b12 * s.v[1497]) + (eq13_e1350 * s.db[1497][12]));
        let eq13_e1352_d_b13: f64 = ((eq13_e1350_d_b13 * s.v[1497]) + (eq13_e1350 * s.db[1497][13]));
        let eq13_e1352_d_b14: f64 = ((eq13_e1350_d_b14 * s.v[1497]) + (eq13_e1350 * s.db[1497][14]));
        let eq13_e1352_d_b15: f64 = ((eq13_e1350_d_b15 * s.v[1497]) + (eq13_e1350 * s.db[1497][15]));
        let eq13_e1352_d_b16: f64 = ((eq13_e1350_d_b16 * s.v[1497]) + (eq13_e1350 * s.db[1497][16]));
        let eq13_e1352_d_b17: f64 = ((eq13_e1350_d_b17 * s.v[1497]) + (eq13_e1350 * s.db[1497][17]));
        let eq13_e1354: f64 = (eq13_e1352 * p.p226);
        let eq13_e1354_d_n0: f64 = (eq13_e1352_d_n0 * p.p226);
        let eq13_e1354_d_n1: f64 = (eq13_e1352_d_n1 * p.p226);
        let eq13_e1354_d_n2: f64 = (eq13_e1352_d_n2 * p.p226);
        let eq13_e1354_d_n3: f64 = (eq13_e1352_d_n3 * p.p226);
        let eq13_e1354_d_n4: f64 = (eq13_e1352_d_n4 * p.p226);
        let eq13_e1354_d_n5: f64 = (eq13_e1352_d_n5 * p.p226);
        let eq13_e1354_d_n6: f64 = (eq13_e1352_d_n6 * p.p226);
        let eq13_e1354_d_n7: f64 = (eq13_e1352_d_n7 * p.p226);
        let eq13_e1354_d_n8: f64 = (eq13_e1352_d_n8 * p.p226);
        let eq13_e1354_d_n9: f64 = (eq13_e1352_d_n9 * p.p226);
        let eq13_e1354_d_n10: f64 = (eq13_e1352_d_n10 * p.p226);
        let eq13_e1354_d_n11: f64 = (eq13_e1352_d_n11 * p.p226);
        let eq13_e1354_d_n12: f64 = (eq13_e1352_d_n12 * p.p226);
        let eq13_e1354_d_n13: f64 = (eq13_e1352_d_n13 * p.p226);
        let eq13_e1354_d_b0: f64 = (eq13_e1352_d_b0 * p.p226);
        let eq13_e1354_d_b1: f64 = (eq13_e1352_d_b1 * p.p226);
        let eq13_e1354_d_b2: f64 = (eq13_e1352_d_b2 * p.p226);
        let eq13_e1354_d_b3: f64 = (eq13_e1352_d_b3 * p.p226);
        let eq13_e1354_d_b4: f64 = (eq13_e1352_d_b4 * p.p226);
        let eq13_e1354_d_b5: f64 = (eq13_e1352_d_b5 * p.p226);
        let eq13_e1354_d_b6: f64 = (eq13_e1352_d_b6 * p.p226);
        let eq13_e1354_d_b7: f64 = (eq13_e1352_d_b7 * p.p226);
        let eq13_e1354_d_b8: f64 = (eq13_e1352_d_b8 * p.p226);
        let eq13_e1354_d_b9: f64 = (eq13_e1352_d_b9 * p.p226);
        let eq13_e1354_d_b10: f64 = (eq13_e1352_d_b10 * p.p226);
        let eq13_e1354_d_b11: f64 = (eq13_e1352_d_b11 * p.p226);
        let eq13_e1354_d_b12: f64 = (eq13_e1352_d_b12 * p.p226);
        let eq13_e1354_d_b13: f64 = (eq13_e1352_d_b13 * p.p226);
        let eq13_e1354_d_b14: f64 = (eq13_e1352_d_b14 * p.p226);
        let eq13_e1354_d_b15: f64 = (eq13_e1352_d_b15 * p.p226);
        let eq13_e1354_d_b16: f64 = (eq13_e1352_d_b16 * p.p226);
        let eq13_e1354_d_b17: f64 = (eq13_e1352_d_b17 * p.p226);
        (eq13_e1354, eq13_e1354_d_n0, eq13_e1354_d_n1, eq13_e1354_d_n2, eq13_e1354_d_n3, eq13_e1354_d_n4, eq13_e1354_d_n5, eq13_e1354_d_n6, eq13_e1354_d_n7, eq13_e1354_d_n8, eq13_e1354_d_n9, eq13_e1354_d_n10, eq13_e1354_d_n11, eq13_e1354_d_n12, eq13_e1354_d_n13, eq13_e1354_d_b0, eq13_e1354_d_b1, eq13_e1354_d_b2, eq13_e1354_d_b3, eq13_e1354_d_b4, eq13_e1354_d_b5, eq13_e1354_d_b6, eq13_e1354_d_b7, eq13_e1354_d_b8, eq13_e1354_d_b9, eq13_e1354_d_b10, eq13_e1354_d_b11, eq13_e1354_d_b12, eq13_e1354_d_b13, eq13_e1354_d_b14, eq13_e1354_d_b15, eq13_e1354_d_b16, eq13_e1354_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e1356;
        let eq13_node_derivatives: [f64; 14] = [eq13_e1356_d_n0, eq13_e1356_d_n1, eq13_e1356_d_n2, eq13_e1356_d_n3, eq13_e1356_d_n4, eq13_e1356_d_n5, eq13_e1356_d_n6, eq13_e1356_d_n7, eq13_e1356_d_n8, eq13_e1356_d_n9, eq13_e1356_d_n10, eq13_e1356_d_n11, eq13_e1356_d_n12, eq13_e1356_d_n13];
        let eq13_branch_derivatives: [f64; 18] = [eq13_e1356_d_b0, eq13_e1356_d_b1, eq13_e1356_d_b2, eq13_e1356_d_b3, eq13_e1356_d_b4, eq13_e1356_d_b5, eq13_e1356_d_b6, eq13_e1356_d_b7, eq13_e1356_d_b8, eq13_e1356_d_b9, eq13_e1356_d_b10, eq13_e1356_d_b11, eq13_e1356_d_b12, eq13_e1356_d_b13, eq13_e1356_d_b14, eq13_e1356_d_b15, eq13_e1356_d_b16, eq13_e1356_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e1376, eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13, eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq14_e1367: f64 = (p.p33 * 0.5);
        let eq14_e1369: f64 = (eq14_e1367 * s.v[1501]);
        let eq14_e1369_d_n0: f64 = (eq14_e1367 * s.dn[1501][0]);
        let eq14_e1369_d_n1: f64 = (eq14_e1367 * s.dn[1501][1]);
        let eq14_e1369_d_n2: f64 = (eq14_e1367 * s.dn[1501][2]);
        let eq14_e1369_d_n3: f64 = (eq14_e1367 * s.dn[1501][3]);
        let eq14_e1369_d_n4: f64 = (eq14_e1367 * s.dn[1501][4]);
        let eq14_e1369_d_n5: f64 = (eq14_e1367 * s.dn[1501][5]);
        let eq14_e1369_d_n6: f64 = (eq14_e1367 * s.dn[1501][6]);
        let eq14_e1369_d_n7: f64 = (eq14_e1367 * s.dn[1501][7]);
        let eq14_e1369_d_n8: f64 = (eq14_e1367 * s.dn[1501][8]);
        let eq14_e1369_d_n9: f64 = (eq14_e1367 * s.dn[1501][9]);
        let eq14_e1369_d_n10: f64 = (eq14_e1367 * s.dn[1501][10]);
        let eq14_e1369_d_n11: f64 = (eq14_e1367 * s.dn[1501][11]);
        let eq14_e1369_d_n12: f64 = (eq14_e1367 * s.dn[1501][12]);
        let eq14_e1369_d_n13: f64 = (eq14_e1367 * s.dn[1501][13]);
        let eq14_e1369_d_b0: f64 = (eq14_e1367 * s.db[1501][0]);
        let eq14_e1369_d_b1: f64 = (eq14_e1367 * s.db[1501][1]);
        let eq14_e1369_d_b2: f64 = (eq14_e1367 * s.db[1501][2]);
        let eq14_e1369_d_b3: f64 = (eq14_e1367 * s.db[1501][3]);
        let eq14_e1369_d_b4: f64 = (eq14_e1367 * s.db[1501][4]);
        let eq14_e1369_d_b5: f64 = (eq14_e1367 * s.db[1501][5]);
        let eq14_e1369_d_b6: f64 = (eq14_e1367 * s.db[1501][6]);
        let eq14_e1369_d_b7: f64 = (eq14_e1367 * s.db[1501][7]);
        let eq14_e1369_d_b8: f64 = (eq14_e1367 * s.db[1501][8]);
        let eq14_e1369_d_b9: f64 = (eq14_e1367 * s.db[1501][9]);
        let eq14_e1369_d_b10: f64 = (eq14_e1367 * s.db[1501][10]);
        let eq14_e1369_d_b11: f64 = (eq14_e1367 * s.db[1501][11]);
        let eq14_e1369_d_b12: f64 = (eq14_e1367 * s.db[1501][12]);
        let eq14_e1369_d_b13: f64 = (eq14_e1367 * s.db[1501][13]);
        let eq14_e1369_d_b14: f64 = (eq14_e1367 * s.db[1501][14]);
        let eq14_e1369_d_b15: f64 = (eq14_e1367 * s.db[1501][15]);
        let eq14_e1369_d_b16: f64 = (eq14_e1367 * s.db[1501][16]);
        let eq14_e1369_d_b17: f64 = (eq14_e1367 * s.db[1501][17]);
        let eq14_e1371: f64 = (eq14_e1369 * p.p226);
        let eq14_e1371_d_n0: f64 = (eq14_e1369_d_n0 * p.p226);
        let eq14_e1371_d_n1: f64 = (eq14_e1369_d_n1 * p.p226);
        let eq14_e1371_d_n2: f64 = (eq14_e1369_d_n2 * p.p226);
        let eq14_e1371_d_n3: f64 = (eq14_e1369_d_n3 * p.p226);
        let eq14_e1371_d_n4: f64 = (eq14_e1369_d_n4 * p.p226);
        let eq14_e1371_d_n5: f64 = (eq14_e1369_d_n5 * p.p226);
        let eq14_e1371_d_n6: f64 = (eq14_e1369_d_n6 * p.p226);
        let eq14_e1371_d_n7: f64 = (eq14_e1369_d_n7 * p.p226);
        let eq14_e1371_d_n8: f64 = (eq14_e1369_d_n8 * p.p226);
        let eq14_e1371_d_n9: f64 = (eq14_e1369_d_n9 * p.p226);
        let eq14_e1371_d_n10: f64 = (eq14_e1369_d_n10 * p.p226);
        let eq14_e1371_d_n11: f64 = (eq14_e1369_d_n11 * p.p226);
        let eq14_e1371_d_n12: f64 = (eq14_e1369_d_n12 * p.p226);
        let eq14_e1371_d_n13: f64 = (eq14_e1369_d_n13 * p.p226);
        let eq14_e1371_d_b0: f64 = (eq14_e1369_d_b0 * p.p226);
        let eq14_e1371_d_b1: f64 = (eq14_e1369_d_b1 * p.p226);
        let eq14_e1371_d_b2: f64 = (eq14_e1369_d_b2 * p.p226);
        let eq14_e1371_d_b3: f64 = (eq14_e1369_d_b3 * p.p226);
        let eq14_e1371_d_b4: f64 = (eq14_e1369_d_b4 * p.p226);
        let eq14_e1371_d_b5: f64 = (eq14_e1369_d_b5 * p.p226);
        let eq14_e1371_d_b6: f64 = (eq14_e1369_d_b6 * p.p226);
        let eq14_e1371_d_b7: f64 = (eq14_e1369_d_b7 * p.p226);
        let eq14_e1371_d_b8: f64 = (eq14_e1369_d_b8 * p.p226);
        let eq14_e1371_d_b9: f64 = (eq14_e1369_d_b9 * p.p226);
        let eq14_e1371_d_b10: f64 = (eq14_e1369_d_b10 * p.p226);
        let eq14_e1371_d_b11: f64 = (eq14_e1369_d_b11 * p.p226);
        let eq14_e1371_d_b12: f64 = (eq14_e1369_d_b12 * p.p226);
        let eq14_e1371_d_b13: f64 = (eq14_e1369_d_b13 * p.p226);
        let eq14_e1371_d_b14: f64 = (eq14_e1369_d_b14 * p.p226);
        let eq14_e1371_d_b15: f64 = (eq14_e1369_d_b15 * p.p226);
        let eq14_e1371_d_b16: f64 = (eq14_e1369_d_b16 * p.p226);
        let eq14_e1371_d_b17: f64 = (eq14_e1369_d_b17 * p.p226);
        let eq14_e1373: f64 = (eq14_e1371 * (nv13 - 0.0));
        let eq14_e1373_d_n0: f64 = (eq14_e1371_d_n0 * (nv13 - 0.0));
        let eq14_e1373_d_n1: f64 = (eq14_e1371_d_n1 * (nv13 - 0.0));
        let eq14_e1373_d_n2: f64 = (eq14_e1371_d_n2 * (nv13 - 0.0));
        let eq14_e1373_d_n3: f64 = (eq14_e1371_d_n3 * (nv13 - 0.0));
        let eq14_e1373_d_n4: f64 = (eq14_e1371_d_n4 * (nv13 - 0.0));
        let eq14_e1373_d_n5: f64 = (eq14_e1371_d_n5 * (nv13 - 0.0));
        let eq14_e1373_d_n6: f64 = (eq14_e1371_d_n6 * (nv13 - 0.0));
        let eq14_e1373_d_n7: f64 = (eq14_e1371_d_n7 * (nv13 - 0.0));
        let eq14_e1373_d_n8: f64 = (eq14_e1371_d_n8 * (nv13 - 0.0));
        let eq14_e1373_d_n9: f64 = (eq14_e1371_d_n9 * (nv13 - 0.0));
        let eq14_e1373_d_n10: f64 = (eq14_e1371_d_n10 * (nv13 - 0.0));
        let eq14_e1373_d_n11: f64 = (eq14_e1371_d_n11 * (nv13 - 0.0));
        let eq14_e1373_d_n12: f64 = (eq14_e1371_d_n12 * (nv13 - 0.0));
        let eq14_e1373_d_n13: f64 = ((eq14_e1371_d_n13 * (nv13 - 0.0)) + eq14_e1371);
        let eq14_e1373_d_b0: f64 = (eq14_e1371_d_b0 * (nv13 - 0.0));
        let eq14_e1373_d_b1: f64 = (eq14_e1371_d_b1 * (nv13 - 0.0));
        let eq14_e1373_d_b2: f64 = (eq14_e1371_d_b2 * (nv13 - 0.0));
        let eq14_e1373_d_b3: f64 = (eq14_e1371_d_b3 * (nv13 - 0.0));
        let eq14_e1373_d_b4: f64 = (eq14_e1371_d_b4 * (nv13 - 0.0));
        let eq14_e1373_d_b5: f64 = (eq14_e1371_d_b5 * (nv13 - 0.0));
        let eq14_e1373_d_b6: f64 = (eq14_e1371_d_b6 * (nv13 - 0.0));
        let eq14_e1373_d_b7: f64 = (eq14_e1371_d_b7 * (nv13 - 0.0));
        let eq14_e1373_d_b8: f64 = (eq14_e1371_d_b8 * (nv13 - 0.0));
        let eq14_e1373_d_b9: f64 = (eq14_e1371_d_b9 * (nv13 - 0.0));
        let eq14_e1373_d_b10: f64 = (eq14_e1371_d_b10 * (nv13 - 0.0));
        let eq14_e1373_d_b11: f64 = (eq14_e1371_d_b11 * (nv13 - 0.0));
        let eq14_e1373_d_b12: f64 = (eq14_e1371_d_b12 * (nv13 - 0.0));
        let eq14_e1373_d_b13: f64 = (eq14_e1371_d_b13 * (nv13 - 0.0));
        let eq14_e1373_d_b14: f64 = (eq14_e1371_d_b14 * (nv13 - 0.0));
        let eq14_e1373_d_b15: f64 = (eq14_e1371_d_b15 * (nv13 - 0.0));
        let eq14_e1373_d_b16: f64 = (eq14_e1371_d_b16 * (nv13 - 0.0));
        let eq14_e1373_d_b17: f64 = (eq14_e1371_d_b17 * (nv13 - 0.0));
        let eq14_e1374: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq14_e1373);
        (eq14_e1374, (eq14_e1373_d_n0 * ddt_scale), (eq14_e1373_d_n1 * ddt_scale), (eq14_e1373_d_n2 * ddt_scale), (eq14_e1373_d_n3 * ddt_scale), (eq14_e1373_d_n4 * ddt_scale), (eq14_e1373_d_n5 * ddt_scale), (eq14_e1373_d_n6 * ddt_scale), (eq14_e1373_d_n7 * ddt_scale), (eq14_e1373_d_n8 * ddt_scale), (eq14_e1373_d_n9 * ddt_scale), (eq14_e1373_d_n10 * ddt_scale), (eq14_e1373_d_n11 * ddt_scale), (eq14_e1373_d_n12 * ddt_scale), (eq14_e1373_d_n13 * ddt_scale), (eq14_e1373_d_b0 * ddt_scale), (eq14_e1373_d_b1 * ddt_scale), (eq14_e1373_d_b2 * ddt_scale), (eq14_e1373_d_b3 * ddt_scale), (eq14_e1373_d_b4 * ddt_scale), (eq14_e1373_d_b5 * ddt_scale), (eq14_e1373_d_b6 * ddt_scale), (eq14_e1373_d_b7 * ddt_scale), (eq14_e1373_d_b8 * ddt_scale), (eq14_e1373_d_b9 * ddt_scale), (eq14_e1373_d_b10 * ddt_scale), (eq14_e1373_d_b11 * ddt_scale), (eq14_e1373_d_b12 * ddt_scale), (eq14_e1373_d_b13 * ddt_scale), (eq14_e1373_d_b14 * ddt_scale), (eq14_e1373_d_b15 * ddt_scale), (eq14_e1373_d_b16 * ddt_scale), (eq14_e1373_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e1376;
        let eq14_node_derivatives: [f64; 14] = [eq14_e1376_d_n0, eq14_e1376_d_n1, eq14_e1376_d_n2, eq14_e1376_d_n3, eq14_e1376_d_n4, eq14_e1376_d_n5, eq14_e1376_d_n6, eq14_e1376_d_n7, eq14_e1376_d_n8, eq14_e1376_d_n9, eq14_e1376_d_n10, eq14_e1376_d_n11, eq14_e1376_d_n12, eq14_e1376_d_n13];
        let eq14_branch_derivatives: [f64; 18] = [eq14_e1376_d_b0, eq14_e1376_d_b1, eq14_e1376_d_b2, eq14_e1376_d_b3, eq14_e1376_d_b4, eq14_e1376_d_b5, eq14_e1376_d_b6, eq14_e1376_d_b7, eq14_e1376_d_b8, eq14_e1376_d_b9, eq14_e1376_d_b10, eq14_e1376_d_b11, eq14_e1376_d_b12, eq14_e1376_d_b13, eq14_e1376_d_b14, eq14_e1376_d_b15, eq14_e1376_d_b16, eq14_e1376_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1396, eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13, eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17,) = {
    if (s.b[1508] && (!((s.b[1505] || s.b[1506]) || s.b[1507]))) {
        let eq15_e1387: f64 = (p.p33 * 0.5);
        let eq15_e1389: f64 = (eq15_e1387 * s.v[1501]);
        let eq15_e1389_d_n0: f64 = (eq15_e1387 * s.dn[1501][0]);
        let eq15_e1389_d_n1: f64 = (eq15_e1387 * s.dn[1501][1]);
        let eq15_e1389_d_n2: f64 = (eq15_e1387 * s.dn[1501][2]);
        let eq15_e1389_d_n3: f64 = (eq15_e1387 * s.dn[1501][3]);
        let eq15_e1389_d_n4: f64 = (eq15_e1387 * s.dn[1501][4]);
        let eq15_e1389_d_n5: f64 = (eq15_e1387 * s.dn[1501][5]);
        let eq15_e1389_d_n6: f64 = (eq15_e1387 * s.dn[1501][6]);
        let eq15_e1389_d_n7: f64 = (eq15_e1387 * s.dn[1501][7]);
        let eq15_e1389_d_n8: f64 = (eq15_e1387 * s.dn[1501][8]);
        let eq15_e1389_d_n9: f64 = (eq15_e1387 * s.dn[1501][9]);
        let eq15_e1389_d_n10: f64 = (eq15_e1387 * s.dn[1501][10]);
        let eq15_e1389_d_n11: f64 = (eq15_e1387 * s.dn[1501][11]);
        let eq15_e1389_d_n12: f64 = (eq15_e1387 * s.dn[1501][12]);
        let eq15_e1389_d_n13: f64 = (eq15_e1387 * s.dn[1501][13]);
        let eq15_e1389_d_b0: f64 = (eq15_e1387 * s.db[1501][0]);
        let eq15_e1389_d_b1: f64 = (eq15_e1387 * s.db[1501][1]);
        let eq15_e1389_d_b2: f64 = (eq15_e1387 * s.db[1501][2]);
        let eq15_e1389_d_b3: f64 = (eq15_e1387 * s.db[1501][3]);
        let eq15_e1389_d_b4: f64 = (eq15_e1387 * s.db[1501][4]);
        let eq15_e1389_d_b5: f64 = (eq15_e1387 * s.db[1501][5]);
        let eq15_e1389_d_b6: f64 = (eq15_e1387 * s.db[1501][6]);
        let eq15_e1389_d_b7: f64 = (eq15_e1387 * s.db[1501][7]);
        let eq15_e1389_d_b8: f64 = (eq15_e1387 * s.db[1501][8]);
        let eq15_e1389_d_b9: f64 = (eq15_e1387 * s.db[1501][9]);
        let eq15_e1389_d_b10: f64 = (eq15_e1387 * s.db[1501][10]);
        let eq15_e1389_d_b11: f64 = (eq15_e1387 * s.db[1501][11]);
        let eq15_e1389_d_b12: f64 = (eq15_e1387 * s.db[1501][12]);
        let eq15_e1389_d_b13: f64 = (eq15_e1387 * s.db[1501][13]);
        let eq15_e1389_d_b14: f64 = (eq15_e1387 * s.db[1501][14]);
        let eq15_e1389_d_b15: f64 = (eq15_e1387 * s.db[1501][15]);
        let eq15_e1389_d_b16: f64 = (eq15_e1387 * s.db[1501][16]);
        let eq15_e1389_d_b17: f64 = (eq15_e1387 * s.db[1501][17]);
        let eq15_e1391: f64 = (eq15_e1389 * p.p226);
        let eq15_e1391_d_n0: f64 = (eq15_e1389_d_n0 * p.p226);
        let eq15_e1391_d_n1: f64 = (eq15_e1389_d_n1 * p.p226);
        let eq15_e1391_d_n2: f64 = (eq15_e1389_d_n2 * p.p226);
        let eq15_e1391_d_n3: f64 = (eq15_e1389_d_n3 * p.p226);
        let eq15_e1391_d_n4: f64 = (eq15_e1389_d_n4 * p.p226);
        let eq15_e1391_d_n5: f64 = (eq15_e1389_d_n5 * p.p226);
        let eq15_e1391_d_n6: f64 = (eq15_e1389_d_n6 * p.p226);
        let eq15_e1391_d_n7: f64 = (eq15_e1389_d_n7 * p.p226);
        let eq15_e1391_d_n8: f64 = (eq15_e1389_d_n8 * p.p226);
        let eq15_e1391_d_n9: f64 = (eq15_e1389_d_n9 * p.p226);
        let eq15_e1391_d_n10: f64 = (eq15_e1389_d_n10 * p.p226);
        let eq15_e1391_d_n11: f64 = (eq15_e1389_d_n11 * p.p226);
        let eq15_e1391_d_n12: f64 = (eq15_e1389_d_n12 * p.p226);
        let eq15_e1391_d_n13: f64 = (eq15_e1389_d_n13 * p.p226);
        let eq15_e1391_d_b0: f64 = (eq15_e1389_d_b0 * p.p226);
        let eq15_e1391_d_b1: f64 = (eq15_e1389_d_b1 * p.p226);
        let eq15_e1391_d_b2: f64 = (eq15_e1389_d_b2 * p.p226);
        let eq15_e1391_d_b3: f64 = (eq15_e1389_d_b3 * p.p226);
        let eq15_e1391_d_b4: f64 = (eq15_e1389_d_b4 * p.p226);
        let eq15_e1391_d_b5: f64 = (eq15_e1389_d_b5 * p.p226);
        let eq15_e1391_d_b6: f64 = (eq15_e1389_d_b6 * p.p226);
        let eq15_e1391_d_b7: f64 = (eq15_e1389_d_b7 * p.p226);
        let eq15_e1391_d_b8: f64 = (eq15_e1389_d_b8 * p.p226);
        let eq15_e1391_d_b9: f64 = (eq15_e1389_d_b9 * p.p226);
        let eq15_e1391_d_b10: f64 = (eq15_e1389_d_b10 * p.p226);
        let eq15_e1391_d_b11: f64 = (eq15_e1389_d_b11 * p.p226);
        let eq15_e1391_d_b12: f64 = (eq15_e1389_d_b12 * p.p226);
        let eq15_e1391_d_b13: f64 = (eq15_e1389_d_b13 * p.p226);
        let eq15_e1391_d_b14: f64 = (eq15_e1389_d_b14 * p.p226);
        let eq15_e1391_d_b15: f64 = (eq15_e1389_d_b15 * p.p226);
        let eq15_e1391_d_b16: f64 = (eq15_e1389_d_b16 * p.p226);
        let eq15_e1391_d_b17: f64 = (eq15_e1389_d_b17 * p.p226);
        let eq15_e1393: f64 = (eq15_e1391 * (nv13 - 0.0));
        let eq15_e1393_d_n0: f64 = (eq15_e1391_d_n0 * (nv13 - 0.0));
        let eq15_e1393_d_n1: f64 = (eq15_e1391_d_n1 * (nv13 - 0.0));
        let eq15_e1393_d_n2: f64 = (eq15_e1391_d_n2 * (nv13 - 0.0));
        let eq15_e1393_d_n3: f64 = (eq15_e1391_d_n3 * (nv13 - 0.0));
        let eq15_e1393_d_n4: f64 = (eq15_e1391_d_n4 * (nv13 - 0.0));
        let eq15_e1393_d_n5: f64 = (eq15_e1391_d_n5 * (nv13 - 0.0));
        let eq15_e1393_d_n6: f64 = (eq15_e1391_d_n6 * (nv13 - 0.0));
        let eq15_e1393_d_n7: f64 = (eq15_e1391_d_n7 * (nv13 - 0.0));
        let eq15_e1393_d_n8: f64 = (eq15_e1391_d_n8 * (nv13 - 0.0));
        let eq15_e1393_d_n9: f64 = (eq15_e1391_d_n9 * (nv13 - 0.0));
        let eq15_e1393_d_n10: f64 = (eq15_e1391_d_n10 * (nv13 - 0.0));
        let eq15_e1393_d_n11: f64 = (eq15_e1391_d_n11 * (nv13 - 0.0));
        let eq15_e1393_d_n12: f64 = (eq15_e1391_d_n12 * (nv13 - 0.0));
        let eq15_e1393_d_n13: f64 = ((eq15_e1391_d_n13 * (nv13 - 0.0)) + eq15_e1391);
        let eq15_e1393_d_b0: f64 = (eq15_e1391_d_b0 * (nv13 - 0.0));
        let eq15_e1393_d_b1: f64 = (eq15_e1391_d_b1 * (nv13 - 0.0));
        let eq15_e1393_d_b2: f64 = (eq15_e1391_d_b2 * (nv13 - 0.0));
        let eq15_e1393_d_b3: f64 = (eq15_e1391_d_b3 * (nv13 - 0.0));
        let eq15_e1393_d_b4: f64 = (eq15_e1391_d_b4 * (nv13 - 0.0));
        let eq15_e1393_d_b5: f64 = (eq15_e1391_d_b5 * (nv13 - 0.0));
        let eq15_e1393_d_b6: f64 = (eq15_e1391_d_b6 * (nv13 - 0.0));
        let eq15_e1393_d_b7: f64 = (eq15_e1391_d_b7 * (nv13 - 0.0));
        let eq15_e1393_d_b8: f64 = (eq15_e1391_d_b8 * (nv13 - 0.0));
        let eq15_e1393_d_b9: f64 = (eq15_e1391_d_b9 * (nv13 - 0.0));
        let eq15_e1393_d_b10: f64 = (eq15_e1391_d_b10 * (nv13 - 0.0));
        let eq15_e1393_d_b11: f64 = (eq15_e1391_d_b11 * (nv13 - 0.0));
        let eq15_e1393_d_b12: f64 = (eq15_e1391_d_b12 * (nv13 - 0.0));
        let eq15_e1393_d_b13: f64 = (eq15_e1391_d_b13 * (nv13 - 0.0));
        let eq15_e1393_d_b14: f64 = (eq15_e1391_d_b14 * (nv13 - 0.0));
        let eq15_e1393_d_b15: f64 = (eq15_e1391_d_b15 * (nv13 - 0.0));
        let eq15_e1393_d_b16: f64 = (eq15_e1391_d_b16 * (nv13 - 0.0));
        let eq15_e1393_d_b17: f64 = (eq15_e1391_d_b17 * (nv13 - 0.0));
        let eq15_e1394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq15_e1393);
        (eq15_e1394, (eq15_e1393_d_n0 * ddt_scale), (eq15_e1393_d_n1 * ddt_scale), (eq15_e1393_d_n2 * ddt_scale), (eq15_e1393_d_n3 * ddt_scale), (eq15_e1393_d_n4 * ddt_scale), (eq15_e1393_d_n5 * ddt_scale), (eq15_e1393_d_n6 * ddt_scale), (eq15_e1393_d_n7 * ddt_scale), (eq15_e1393_d_n8 * ddt_scale), (eq15_e1393_d_n9 * ddt_scale), (eq15_e1393_d_n10 * ddt_scale), (eq15_e1393_d_n11 * ddt_scale), (eq15_e1393_d_n12 * ddt_scale), (eq15_e1393_d_n13 * ddt_scale), (eq15_e1393_d_b0 * ddt_scale), (eq15_e1393_d_b1 * ddt_scale), (eq15_e1393_d_b2 * ddt_scale), (eq15_e1393_d_b3 * ddt_scale), (eq15_e1393_d_b4 * ddt_scale), (eq15_e1393_d_b5 * ddt_scale), (eq15_e1393_d_b6 * ddt_scale), (eq15_e1393_d_b7 * ddt_scale), (eq15_e1393_d_b8 * ddt_scale), (eq15_e1393_d_b9 * ddt_scale), (eq15_e1393_d_b10 * ddt_scale), (eq15_e1393_d_b11 * ddt_scale), (eq15_e1393_d_b12 * ddt_scale), (eq15_e1393_d_b13 * ddt_scale), (eq15_e1393_d_b14 * ddt_scale), (eq15_e1393_d_b15 * ddt_scale), (eq15_e1393_d_b16 * ddt_scale), (eq15_e1393_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1396;
        let eq15_node_derivatives: [f64; 14] = [eq15_e1396_d_n0, eq15_e1396_d_n1, eq15_e1396_d_n2, eq15_e1396_d_n3, eq15_e1396_d_n4, eq15_e1396_d_n5, eq15_e1396_d_n6, eq15_e1396_d_n7, eq15_e1396_d_n8, eq15_e1396_d_n9, eq15_e1396_d_n10, eq15_e1396_d_n11, eq15_e1396_d_n12, eq15_e1396_d_n13];
        let eq15_branch_derivatives: [f64; 18] = [eq15_e1396_d_b0, eq15_e1396_d_b1, eq15_e1396_d_b2, eq15_e1396_d_b3, eq15_e1396_d_b4, eq15_e1396_d_b5, eq15_e1396_d_b6, eq15_e1396_d_b7, eq15_e1396_d_b8, eq15_e1396_d_b9, eq15_e1396_d_b10, eq15_e1396_d_b11, eq15_e1396_d_b12, eq15_e1396_d_b13, eq15_e1396_d_b14, eq15_e1396_d_b15, eq15_e1396_d_b16, eq15_e1396_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq18_e1414, eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13, eq18_e1414_d_b0, eq18_e1414_d_b1, eq18_e1414_d_b2, eq18_e1414_d_b3, eq18_e1414_d_b4, eq18_e1414_d_b5, eq18_e1414_d_b6, eq18_e1414_d_b7, eq18_e1414_d_b8, eq18_e1414_d_b9, eq18_e1414_d_b10, eq18_e1414_d_b11, eq18_e1414_d_b12, eq18_e1414_d_b13, eq18_e1414_d_b14, eq18_e1414_d_b15, eq18_e1414_d_b16, eq18_e1414_d_b17,) = {
    if s.b[1546] {
        let eq18_e1410: f64 = (p.p32 * (nv0 - nv7));
        let eq18_e1412: f64 = (eq18_e1410 / s.v[1099]);
        let eq18_e1412_d_n0: f64 = (((p.p32 * s.v[1099]) - (eq18_e1410 * s.dn[1099][0])) / (s.v[1099] * s.v[1099]));
        let eq18_e1412_d_n1: f64 = (-((eq18_e1410 * s.dn[1099][1]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n2: f64 = (-((eq18_e1410 * s.dn[1099][2]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n3: f64 = (-((eq18_e1410 * s.dn[1099][3]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n4: f64 = (-((eq18_e1410 * s.dn[1099][4]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n5: f64 = (-((eq18_e1410 * s.dn[1099][5]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n6: f64 = (-((eq18_e1410 * s.dn[1099][6]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n7: f64 = ((((-p.p32) * s.v[1099]) - (eq18_e1410 * s.dn[1099][7])) / (s.v[1099] * s.v[1099]));
        let eq18_e1412_d_n8: f64 = (-((eq18_e1410 * s.dn[1099][8]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n9: f64 = (-((eq18_e1410 * s.dn[1099][9]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n10: f64 = (-((eq18_e1410 * s.dn[1099][10]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n11: f64 = (-((eq18_e1410 * s.dn[1099][11]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n12: f64 = (-((eq18_e1410 * s.dn[1099][12]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_n13: f64 = (-((eq18_e1410 * s.dn[1099][13]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b0: f64 = (-((eq18_e1410 * s.db[1099][0]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b1: f64 = (-((eq18_e1410 * s.db[1099][1]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b2: f64 = (-((eq18_e1410 * s.db[1099][2]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b3: f64 = (-((eq18_e1410 * s.db[1099][3]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b4: f64 = (-((eq18_e1410 * s.db[1099][4]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b5: f64 = (-((eq18_e1410 * s.db[1099][5]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b6: f64 = (-((eq18_e1410 * s.db[1099][6]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b7: f64 = (-((eq18_e1410 * s.db[1099][7]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b8: f64 = (-((eq18_e1410 * s.db[1099][8]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b9: f64 = (-((eq18_e1410 * s.db[1099][9]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b10: f64 = (-((eq18_e1410 * s.db[1099][10]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b11: f64 = (-((eq18_e1410 * s.db[1099][11]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b12: f64 = (-((eq18_e1410 * s.db[1099][12]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b13: f64 = (-((eq18_e1410 * s.db[1099][13]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b14: f64 = (-((eq18_e1410 * s.db[1099][14]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b15: f64 = (-((eq18_e1410 * s.db[1099][15]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b16: f64 = (-((eq18_e1410 * s.db[1099][16]) / (s.v[1099] * s.v[1099])));
        let eq18_e1412_d_b17: f64 = (-((eq18_e1410 * s.db[1099][17]) / (s.v[1099] * s.v[1099])));
        (eq18_e1412, eq18_e1412_d_n0, eq18_e1412_d_n1, eq18_e1412_d_n2, eq18_e1412_d_n3, eq18_e1412_d_n4, eq18_e1412_d_n5, eq18_e1412_d_n6, eq18_e1412_d_n7, eq18_e1412_d_n8, eq18_e1412_d_n9, eq18_e1412_d_n10, eq18_e1412_d_n11, eq18_e1412_d_n12, eq18_e1412_d_n13, eq18_e1412_d_b0, eq18_e1412_d_b1, eq18_e1412_d_b2, eq18_e1412_d_b3, eq18_e1412_d_b4, eq18_e1412_d_b5, eq18_e1412_d_b6, eq18_e1412_d_b7, eq18_e1412_d_b8, eq18_e1412_d_b9, eq18_e1412_d_b10, eq18_e1412_d_b11, eq18_e1412_d_b12, eq18_e1412_d_b13, eq18_e1412_d_b14, eq18_e1412_d_b15, eq18_e1412_d_b16, eq18_e1412_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1414;
        let eq18_node_derivatives: [f64; 14] = [eq18_e1414_d_n0, eq18_e1414_d_n1, eq18_e1414_d_n2, eq18_e1414_d_n3, eq18_e1414_d_n4, eq18_e1414_d_n5, eq18_e1414_d_n6, eq18_e1414_d_n7, eq18_e1414_d_n8, eq18_e1414_d_n9, eq18_e1414_d_n10, eq18_e1414_d_n11, eq18_e1414_d_n12, eq18_e1414_d_n13];
        let eq18_branch_derivatives: [f64; 18] = [eq18_e1414_d_b0, eq18_e1414_d_b1, eq18_e1414_d_b2, eq18_e1414_d_b3, eq18_e1414_d_b4, eq18_e1414_d_b5, eq18_e1414_d_b6, eq18_e1414_d_b7, eq18_e1414_d_b8, eq18_e1414_d_b9, eq18_e1414_d_b10, eq18_e1414_d_b11, eq18_e1414_d_b12, eq18_e1414_d_b13, eq18_e1414_d_b14, eq18_e1414_d_b15, eq18_e1414_d_b16, eq18_e1414_d_b17];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1438, eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13, eq21_e1438_d_b0, eq21_e1438_d_b1, eq21_e1438_d_b2, eq21_e1438_d_b3, eq21_e1438_d_b4, eq21_e1438_d_b5, eq21_e1438_d_b6, eq21_e1438_d_b7, eq21_e1438_d_b8, eq21_e1438_d_b9, eq21_e1438_d_b10, eq21_e1438_d_b11, eq21_e1438_d_b12, eq21_e1438_d_b13, eq21_e1438_d_b14, eq21_e1438_d_b15, eq21_e1438_d_b16, eq21_e1438_d_b17,) = {
    if s.b[1547] {
        let eq21_e1434: f64 = (p.p32 * (nv2 - nv8));
        let eq21_e1436: f64 = (eq21_e1434 / s.v[1100]);
        let eq21_e1436_d_n0: f64 = (-((eq21_e1434 * s.dn[1100][0]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n1: f64 = (-((eq21_e1434 * s.dn[1100][1]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n2: f64 = (((p.p32 * s.v[1100]) - (eq21_e1434 * s.dn[1100][2])) / (s.v[1100] * s.v[1100]));
        let eq21_e1436_d_n3: f64 = (-((eq21_e1434 * s.dn[1100][3]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n4: f64 = (-((eq21_e1434 * s.dn[1100][4]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n5: f64 = (-((eq21_e1434 * s.dn[1100][5]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n6: f64 = (-((eq21_e1434 * s.dn[1100][6]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n7: f64 = (-((eq21_e1434 * s.dn[1100][7]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n8: f64 = ((((-p.p32) * s.v[1100]) - (eq21_e1434 * s.dn[1100][8])) / (s.v[1100] * s.v[1100]));
        let eq21_e1436_d_n9: f64 = (-((eq21_e1434 * s.dn[1100][9]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n10: f64 = (-((eq21_e1434 * s.dn[1100][10]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n11: f64 = (-((eq21_e1434 * s.dn[1100][11]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n12: f64 = (-((eq21_e1434 * s.dn[1100][12]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_n13: f64 = (-((eq21_e1434 * s.dn[1100][13]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b0: f64 = (-((eq21_e1434 * s.db[1100][0]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b1: f64 = (-((eq21_e1434 * s.db[1100][1]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b2: f64 = (-((eq21_e1434 * s.db[1100][2]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b3: f64 = (-((eq21_e1434 * s.db[1100][3]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b4: f64 = (-((eq21_e1434 * s.db[1100][4]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b5: f64 = (-((eq21_e1434 * s.db[1100][5]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b6: f64 = (-((eq21_e1434 * s.db[1100][6]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b7: f64 = (-((eq21_e1434 * s.db[1100][7]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b8: f64 = (-((eq21_e1434 * s.db[1100][8]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b9: f64 = (-((eq21_e1434 * s.db[1100][9]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b10: f64 = (-((eq21_e1434 * s.db[1100][10]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b11: f64 = (-((eq21_e1434 * s.db[1100][11]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b12: f64 = (-((eq21_e1434 * s.db[1100][12]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b13: f64 = (-((eq21_e1434 * s.db[1100][13]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b14: f64 = (-((eq21_e1434 * s.db[1100][14]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b15: f64 = (-((eq21_e1434 * s.db[1100][15]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b16: f64 = (-((eq21_e1434 * s.db[1100][16]) / (s.v[1100] * s.v[1100])));
        let eq21_e1436_d_b17: f64 = (-((eq21_e1434 * s.db[1100][17]) / (s.v[1100] * s.v[1100])));
        (eq21_e1436, eq21_e1436_d_n0, eq21_e1436_d_n1, eq21_e1436_d_n2, eq21_e1436_d_n3, eq21_e1436_d_n4, eq21_e1436_d_n5, eq21_e1436_d_n6, eq21_e1436_d_n7, eq21_e1436_d_n8, eq21_e1436_d_n9, eq21_e1436_d_n10, eq21_e1436_d_n11, eq21_e1436_d_n12, eq21_e1436_d_n13, eq21_e1436_d_b0, eq21_e1436_d_b1, eq21_e1436_d_b2, eq21_e1436_d_b3, eq21_e1436_d_b4, eq21_e1436_d_b5, eq21_e1436_d_b6, eq21_e1436_d_b7, eq21_e1436_d_b8, eq21_e1436_d_b9, eq21_e1436_d_b10, eq21_e1436_d_b11, eq21_e1436_d_b12, eq21_e1436_d_b13, eq21_e1436_d_b14, eq21_e1436_d_b15, eq21_e1436_d_b16, eq21_e1436_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1438;
        let eq21_node_derivatives: [f64; 14] = [eq21_e1438_d_n0, eq21_e1438_d_n1, eq21_e1438_d_n2, eq21_e1438_d_n3, eq21_e1438_d_n4, eq21_e1438_d_n5, eq21_e1438_d_n6, eq21_e1438_d_n7, eq21_e1438_d_n8, eq21_e1438_d_n9, eq21_e1438_d_n10, eq21_e1438_d_n11, eq21_e1438_d_n12, eq21_e1438_d_n13];
        let eq21_branch_derivatives: [f64; 18] = [eq21_e1438_d_b0, eq21_e1438_d_b1, eq21_e1438_d_b2, eq21_e1438_d_b3, eq21_e1438_d_b4, eq21_e1438_d_b5, eq21_e1438_d_b6, eq21_e1438_d_b7, eq21_e1438_d_b8, eq21_e1438_d_b9, eq21_e1438_d_b10, eq21_e1438_d_b11, eq21_e1438_d_b12, eq21_e1438_d_b13, eq21_e1438_d_b14, eq21_e1438_d_b15, eq21_e1438_d_b16, eq21_e1438_d_b17];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1472, eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13, eq24_e1472_d_b0, eq24_e1472_d_b1, eq24_e1472_d_b2, eq24_e1472_d_b3, eq24_e1472_d_b4, eq24_e1472_d_b5, eq24_e1472_d_b6, eq24_e1472_d_b7, eq24_e1472_d_b8, eq24_e1472_d_b9, eq24_e1472_d_b10, eq24_e1472_d_b11, eq24_e1472_d_b12, eq24_e1472_d_b13, eq24_e1472_d_b14, eq24_e1472_d_b15, eq24_e1472_d_b16, eq24_e1472_d_b17,) = {
    if s.b[1548] {
        let eq24_e1458: f64 = (p.p37 * p.p32);
        let eq24_e1461: f64 = (s.v[885] + s.v[933]);
        let eq24_e1461_d_n0: f64 = (s.dn[885][0] + s.dn[933][0]);
        let eq24_e1461_d_n1: f64 = (s.dn[885][1] + s.dn[933][1]);
        let eq24_e1461_d_n2: f64 = (s.dn[885][2] + s.dn[933][2]);
        let eq24_e1461_d_n3: f64 = (s.dn[885][3] + s.dn[933][3]);
        let eq24_e1461_d_n4: f64 = (s.dn[885][4] + s.dn[933][4]);
        let eq24_e1461_d_n5: f64 = (s.dn[885][5] + s.dn[933][5]);
        let eq24_e1461_d_n6: f64 = (s.dn[885][6] + s.dn[933][6]);
        let eq24_e1461_d_n7: f64 = (s.dn[885][7] + s.dn[933][7]);
        let eq24_e1461_d_n8: f64 = (s.dn[885][8] + s.dn[933][8]);
        let eq24_e1461_d_n9: f64 = (s.dn[885][9] + s.dn[933][9]);
        let eq24_e1461_d_n10: f64 = (s.dn[885][10] + s.dn[933][10]);
        let eq24_e1461_d_n11: f64 = (s.dn[885][11] + s.dn[933][11]);
        let eq24_e1461_d_n12: f64 = (s.dn[885][12] + s.dn[933][12]);
        let eq24_e1461_d_n13: f64 = (s.dn[885][13] + s.dn[933][13]);
        let eq24_e1461_d_b0: f64 = (s.db[885][0] + s.db[933][0]);
        let eq24_e1461_d_b1: f64 = (s.db[885][1] + s.db[933][1]);
        let eq24_e1461_d_b2: f64 = (s.db[885][2] + s.db[933][2]);
        let eq24_e1461_d_b3: f64 = (s.db[885][3] + s.db[933][3]);
        let eq24_e1461_d_b4: f64 = (s.db[885][4] + s.db[933][4]);
        let eq24_e1461_d_b5: f64 = (s.db[885][5] + s.db[933][5]);
        let eq24_e1461_d_b6: f64 = (s.db[885][6] + s.db[933][6]);
        let eq24_e1461_d_b7: f64 = (s.db[885][7] + s.db[933][7]);
        let eq24_e1461_d_b8: f64 = (s.db[885][8] + s.db[933][8]);
        let eq24_e1461_d_b9: f64 = (s.db[885][9] + s.db[933][9]);
        let eq24_e1461_d_b10: f64 = (s.db[885][10] + s.db[933][10]);
        let eq24_e1461_d_b11: f64 = (s.db[885][11] + s.db[933][11]);
        let eq24_e1461_d_b12: f64 = (s.db[885][12] + s.db[933][12]);
        let eq24_e1461_d_b13: f64 = (s.db[885][13] + s.db[933][13]);
        let eq24_e1461_d_b14: f64 = (s.db[885][14] + s.db[933][14]);
        let eq24_e1461_d_b15: f64 = (s.db[885][15] + s.db[933][15]);
        let eq24_e1461_d_b16: f64 = (s.db[885][16] + s.db[933][16]);
        let eq24_e1461_d_b17: f64 = (s.db[885][17] + s.db[933][17]);
        let eq24_e1462: f64 = (eq24_e1458 * eq24_e1461);
        let eq24_e1462_d_n0: f64 = (eq24_e1458 * eq24_e1461_d_n0);
        let eq24_e1462_d_n1: f64 = (eq24_e1458 * eq24_e1461_d_n1);
        let eq24_e1462_d_n2: f64 = (eq24_e1458 * eq24_e1461_d_n2);
        let eq24_e1462_d_n3: f64 = (eq24_e1458 * eq24_e1461_d_n3);
        let eq24_e1462_d_n4: f64 = (eq24_e1458 * eq24_e1461_d_n4);
        let eq24_e1462_d_n5: f64 = (eq24_e1458 * eq24_e1461_d_n5);
        let eq24_e1462_d_n6: f64 = (eq24_e1458 * eq24_e1461_d_n6);
        let eq24_e1462_d_n7: f64 = (eq24_e1458 * eq24_e1461_d_n7);
        let eq24_e1462_d_n8: f64 = (eq24_e1458 * eq24_e1461_d_n8);
        let eq24_e1462_d_n9: f64 = (eq24_e1458 * eq24_e1461_d_n9);
        let eq24_e1462_d_n10: f64 = (eq24_e1458 * eq24_e1461_d_n10);
        let eq24_e1462_d_n11: f64 = (eq24_e1458 * eq24_e1461_d_n11);
        let eq24_e1462_d_n12: f64 = (eq24_e1458 * eq24_e1461_d_n12);
        let eq24_e1462_d_n13: f64 = (eq24_e1458 * eq24_e1461_d_n13);
        let eq24_e1462_d_b0: f64 = (eq24_e1458 * eq24_e1461_d_b0);
        let eq24_e1462_d_b1: f64 = (eq24_e1458 * eq24_e1461_d_b1);
        let eq24_e1462_d_b2: f64 = (eq24_e1458 * eq24_e1461_d_b2);
        let eq24_e1462_d_b3: f64 = (eq24_e1458 * eq24_e1461_d_b3);
        let eq24_e1462_d_b4: f64 = (eq24_e1458 * eq24_e1461_d_b4);
        let eq24_e1462_d_b5: f64 = (eq24_e1458 * eq24_e1461_d_b5);
        let eq24_e1462_d_b6: f64 = (eq24_e1458 * eq24_e1461_d_b6);
        let eq24_e1462_d_b7: f64 = (eq24_e1458 * eq24_e1461_d_b7);
        let eq24_e1462_d_b8: f64 = (eq24_e1458 * eq24_e1461_d_b8);
        let eq24_e1462_d_b9: f64 = (eq24_e1458 * eq24_e1461_d_b9);
        let eq24_e1462_d_b10: f64 = (eq24_e1458 * eq24_e1461_d_b10);
        let eq24_e1462_d_b11: f64 = (eq24_e1458 * eq24_e1461_d_b11);
        let eq24_e1462_d_b12: f64 = (eq24_e1458 * eq24_e1461_d_b12);
        let eq24_e1462_d_b13: f64 = (eq24_e1458 * eq24_e1461_d_b13);
        let eq24_e1462_d_b14: f64 = (eq24_e1458 * eq24_e1461_d_b14);
        let eq24_e1462_d_b15: f64 = (eq24_e1458 * eq24_e1461_d_b15);
        let eq24_e1462_d_b16: f64 = (eq24_e1458 * eq24_e1461_d_b16);
        let eq24_e1462_d_b17: f64 = (eq24_e1458 * eq24_e1461_d_b17);
        let eq24_e1466: f64 = 0.0;
        let eq24_e1468: f64 = (eq24_e1466 * (nv7 - nv8));
        let eq24_e1469: f64 = (p.p32 * eq24_e1468);
        let eq24_e1469_d_n7: f64 = (p.p32 * eq24_e1466);
        let eq24_e1469_d_n8: f64 = (p.p32 * (-eq24_e1466));
        let eq24_e1470: f64 = (eq24_e1462 + eq24_e1469);
        let eq24_e1470_d_n7: f64 = (eq24_e1462_d_n7 + eq24_e1469_d_n7);
        let eq24_e1470_d_n8: f64 = (eq24_e1462_d_n8 + eq24_e1469_d_n8);
        (eq24_e1470, eq24_e1462_d_n0, eq24_e1462_d_n1, eq24_e1462_d_n2, eq24_e1462_d_n3, eq24_e1462_d_n4, eq24_e1462_d_n5, eq24_e1462_d_n6, eq24_e1470_d_n7, eq24_e1470_d_n8, eq24_e1462_d_n9, eq24_e1462_d_n10, eq24_e1462_d_n11, eq24_e1462_d_n12, eq24_e1462_d_n13, eq24_e1462_d_b0, eq24_e1462_d_b1, eq24_e1462_d_b2, eq24_e1462_d_b3, eq24_e1462_d_b4, eq24_e1462_d_b5, eq24_e1462_d_b6, eq24_e1462_d_b7, eq24_e1462_d_b8, eq24_e1462_d_b9, eq24_e1462_d_b10, eq24_e1462_d_b11, eq24_e1462_d_b12, eq24_e1462_d_b13, eq24_e1462_d_b14, eq24_e1462_d_b15, eq24_e1462_d_b16, eq24_e1462_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1472;
        let eq24_node_derivatives: [f64; 14] = [eq24_e1472_d_n0, eq24_e1472_d_n1, eq24_e1472_d_n2, eq24_e1472_d_n3, eq24_e1472_d_n4, eq24_e1472_d_n5, eq24_e1472_d_n6, eq24_e1472_d_n7, eq24_e1472_d_n8, eq24_e1472_d_n9, eq24_e1472_d_n10, eq24_e1472_d_n11, eq24_e1472_d_n12, eq24_e1472_d_n13];
        let eq24_branch_derivatives: [f64; 18] = [eq24_e1472_d_b0, eq24_e1472_d_b1, eq24_e1472_d_b2, eq24_e1472_d_b3, eq24_e1472_d_b4, eq24_e1472_d_b5, eq24_e1472_d_b6, eq24_e1472_d_b7, eq24_e1472_d_b8, eq24_e1472_d_b9, eq24_e1472_d_b10, eq24_e1472_d_b11, eq24_e1472_d_b12, eq24_e1472_d_b13, eq24_e1472_d_b14, eq24_e1472_d_b15, eq24_e1472_d_b16, eq24_e1472_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1480, eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13, eq25_e1480_d_b0, eq25_e1480_d_b1, eq25_e1480_d_b2, eq25_e1480_d_b3, eq25_e1480_d_b4, eq25_e1480_d_b5, eq25_e1480_d_b6, eq25_e1480_d_b7, eq25_e1480_d_b8, eq25_e1480_d_b9, eq25_e1480_d_b10, eq25_e1480_d_b11, eq25_e1480_d_b12, eq25_e1480_d_b13, eq25_e1480_d_b14, eq25_e1480_d_b15, eq25_e1480_d_b16, eq25_e1480_d_b17,) = {
    if s.b[1548] {
        let eq25_e1476: f64 = (p.p37 * p.p32);
        let eq25_e1478: f64 = (eq25_e1476 * s.v[908]);
        let eq25_e1478_d_n0: f64 = (eq25_e1476 * s.dn[908][0]);
        let eq25_e1478_d_n1: f64 = (eq25_e1476 * s.dn[908][1]);
        let eq25_e1478_d_n2: f64 = (eq25_e1476 * s.dn[908][2]);
        let eq25_e1478_d_n3: f64 = (eq25_e1476 * s.dn[908][3]);
        let eq25_e1478_d_n4: f64 = (eq25_e1476 * s.dn[908][4]);
        let eq25_e1478_d_n5: f64 = (eq25_e1476 * s.dn[908][5]);
        let eq25_e1478_d_n6: f64 = (eq25_e1476 * s.dn[908][6]);
        let eq25_e1478_d_n7: f64 = (eq25_e1476 * s.dn[908][7]);
        let eq25_e1478_d_n8: f64 = (eq25_e1476 * s.dn[908][8]);
        let eq25_e1478_d_n9: f64 = (eq25_e1476 * s.dn[908][9]);
        let eq25_e1478_d_n10: f64 = (eq25_e1476 * s.dn[908][10]);
        let eq25_e1478_d_n11: f64 = (eq25_e1476 * s.dn[908][11]);
        let eq25_e1478_d_n12: f64 = (eq25_e1476 * s.dn[908][12]);
        let eq25_e1478_d_n13: f64 = (eq25_e1476 * s.dn[908][13]);
        let eq25_e1478_d_b0: f64 = (eq25_e1476 * s.db[908][0]);
        let eq25_e1478_d_b1: f64 = (eq25_e1476 * s.db[908][1]);
        let eq25_e1478_d_b2: f64 = (eq25_e1476 * s.db[908][2]);
        let eq25_e1478_d_b3: f64 = (eq25_e1476 * s.db[908][3]);
        let eq25_e1478_d_b4: f64 = (eq25_e1476 * s.db[908][4]);
        let eq25_e1478_d_b5: f64 = (eq25_e1476 * s.db[908][5]);
        let eq25_e1478_d_b6: f64 = (eq25_e1476 * s.db[908][6]);
        let eq25_e1478_d_b7: f64 = (eq25_e1476 * s.db[908][7]);
        let eq25_e1478_d_b8: f64 = (eq25_e1476 * s.db[908][8]);
        let eq25_e1478_d_b9: f64 = (eq25_e1476 * s.db[908][9]);
        let eq25_e1478_d_b10: f64 = (eq25_e1476 * s.db[908][10]);
        let eq25_e1478_d_b11: f64 = (eq25_e1476 * s.db[908][11]);
        let eq25_e1478_d_b12: f64 = (eq25_e1476 * s.db[908][12]);
        let eq25_e1478_d_b13: f64 = (eq25_e1476 * s.db[908][13]);
        let eq25_e1478_d_b14: f64 = (eq25_e1476 * s.db[908][14]);
        let eq25_e1478_d_b15: f64 = (eq25_e1476 * s.db[908][15]);
        let eq25_e1478_d_b16: f64 = (eq25_e1476 * s.db[908][16]);
        let eq25_e1478_d_b17: f64 = (eq25_e1476 * s.db[908][17]);
        (eq25_e1478, eq25_e1478_d_n0, eq25_e1478_d_n1, eq25_e1478_d_n2, eq25_e1478_d_n3, eq25_e1478_d_n4, eq25_e1478_d_n5, eq25_e1478_d_n6, eq25_e1478_d_n7, eq25_e1478_d_n8, eq25_e1478_d_n9, eq25_e1478_d_n10, eq25_e1478_d_n11, eq25_e1478_d_n12, eq25_e1478_d_n13, eq25_e1478_d_b0, eq25_e1478_d_b1, eq25_e1478_d_b2, eq25_e1478_d_b3, eq25_e1478_d_b4, eq25_e1478_d_b5, eq25_e1478_d_b6, eq25_e1478_d_b7, eq25_e1478_d_b8, eq25_e1478_d_b9, eq25_e1478_d_b10, eq25_e1478_d_b11, eq25_e1478_d_b12, eq25_e1478_d_b13, eq25_e1478_d_b14, eq25_e1478_d_b15, eq25_e1478_d_b16, eq25_e1478_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e1480;
        let eq25_node_derivatives: [f64; 14] = [eq25_e1480_d_n0, eq25_e1480_d_n1, eq25_e1480_d_n2, eq25_e1480_d_n3, eq25_e1480_d_n4, eq25_e1480_d_n5, eq25_e1480_d_n6, eq25_e1480_d_n7, eq25_e1480_d_n8, eq25_e1480_d_n9, eq25_e1480_d_n10, eq25_e1480_d_n11, eq25_e1480_d_n12, eq25_e1480_d_n13];
        let eq25_branch_derivatives: [f64; 18] = [eq25_e1480_d_b0, eq25_e1480_d_b1, eq25_e1480_d_b2, eq25_e1480_d_b3, eq25_e1480_d_b4, eq25_e1480_d_b5, eq25_e1480_d_b6, eq25_e1480_d_b7, eq25_e1480_d_b8, eq25_e1480_d_b9, eq25_e1480_d_b10, eq25_e1480_d_b11, eq25_e1480_d_b12, eq25_e1480_d_b13, eq25_e1480_d_b14, eq25_e1480_d_b15, eq25_e1480_d_b16, eq25_e1480_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1499, eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13, eq26_e1499_d_b0, eq26_e1499_d_b1, eq26_e1499_d_b2, eq26_e1499_d_b3, eq26_e1499_d_b4, eq26_e1499_d_b5, eq26_e1499_d_b6, eq26_e1499_d_b7, eq26_e1499_d_b8, eq26_e1499_d_b9, eq26_e1499_d_b10, eq26_e1499_d_b11, eq26_e1499_d_b12, eq26_e1499_d_b13, eq26_e1499_d_b14, eq26_e1499_d_b15, eq26_e1499_d_b16, eq26_e1499_d_b17,) = {
    if (!s.b[1548]) {
        let eq26_e1485: f64 = (p.p37 * p.p32);
        let eq26_e1488: f64 = (s.v[885] - s.v[933]);
        let eq26_e1488_d_n0: f64 = (s.dn[885][0] - s.dn[933][0]);
        let eq26_e1488_d_n1: f64 = (s.dn[885][1] - s.dn[933][1]);
        let eq26_e1488_d_n2: f64 = (s.dn[885][2] - s.dn[933][2]);
        let eq26_e1488_d_n3: f64 = (s.dn[885][3] - s.dn[933][3]);
        let eq26_e1488_d_n4: f64 = (s.dn[885][4] - s.dn[933][4]);
        let eq26_e1488_d_n5: f64 = (s.dn[885][5] - s.dn[933][5]);
        let eq26_e1488_d_n6: f64 = (s.dn[885][6] - s.dn[933][6]);
        let eq26_e1488_d_n7: f64 = (s.dn[885][7] - s.dn[933][7]);
        let eq26_e1488_d_n8: f64 = (s.dn[885][8] - s.dn[933][8]);
        let eq26_e1488_d_n9: f64 = (s.dn[885][9] - s.dn[933][9]);
        let eq26_e1488_d_n10: f64 = (s.dn[885][10] - s.dn[933][10]);
        let eq26_e1488_d_n11: f64 = (s.dn[885][11] - s.dn[933][11]);
        let eq26_e1488_d_n12: f64 = (s.dn[885][12] - s.dn[933][12]);
        let eq26_e1488_d_n13: f64 = (s.dn[885][13] - s.dn[933][13]);
        let eq26_e1488_d_b0: f64 = (s.db[885][0] - s.db[933][0]);
        let eq26_e1488_d_b1: f64 = (s.db[885][1] - s.db[933][1]);
        let eq26_e1488_d_b2: f64 = (s.db[885][2] - s.db[933][2]);
        let eq26_e1488_d_b3: f64 = (s.db[885][3] - s.db[933][3]);
        let eq26_e1488_d_b4: f64 = (s.db[885][4] - s.db[933][4]);
        let eq26_e1488_d_b5: f64 = (s.db[885][5] - s.db[933][5]);
        let eq26_e1488_d_b6: f64 = (s.db[885][6] - s.db[933][6]);
        let eq26_e1488_d_b7: f64 = (s.db[885][7] - s.db[933][7]);
        let eq26_e1488_d_b8: f64 = (s.db[885][8] - s.db[933][8]);
        let eq26_e1488_d_b9: f64 = (s.db[885][9] - s.db[933][9]);
        let eq26_e1488_d_b10: f64 = (s.db[885][10] - s.db[933][10]);
        let eq26_e1488_d_b11: f64 = (s.db[885][11] - s.db[933][11]);
        let eq26_e1488_d_b12: f64 = (s.db[885][12] - s.db[933][12]);
        let eq26_e1488_d_b13: f64 = (s.db[885][13] - s.db[933][13]);
        let eq26_e1488_d_b14: f64 = (s.db[885][14] - s.db[933][14]);
        let eq26_e1488_d_b15: f64 = (s.db[885][15] - s.db[933][15]);
        let eq26_e1488_d_b16: f64 = (s.db[885][16] - s.db[933][16]);
        let eq26_e1488_d_b17: f64 = (s.db[885][17] - s.db[933][17]);
        let eq26_e1489: f64 = (eq26_e1485 * eq26_e1488);
        let eq26_e1489_d_n0: f64 = (eq26_e1485 * eq26_e1488_d_n0);
        let eq26_e1489_d_n1: f64 = (eq26_e1485 * eq26_e1488_d_n1);
        let eq26_e1489_d_n2: f64 = (eq26_e1485 * eq26_e1488_d_n2);
        let eq26_e1489_d_n3: f64 = (eq26_e1485 * eq26_e1488_d_n3);
        let eq26_e1489_d_n4: f64 = (eq26_e1485 * eq26_e1488_d_n4);
        let eq26_e1489_d_n5: f64 = (eq26_e1485 * eq26_e1488_d_n5);
        let eq26_e1489_d_n6: f64 = (eq26_e1485 * eq26_e1488_d_n6);
        let eq26_e1489_d_n7: f64 = (eq26_e1485 * eq26_e1488_d_n7);
        let eq26_e1489_d_n8: f64 = (eq26_e1485 * eq26_e1488_d_n8);
        let eq26_e1489_d_n9: f64 = (eq26_e1485 * eq26_e1488_d_n9);
        let eq26_e1489_d_n10: f64 = (eq26_e1485 * eq26_e1488_d_n10);
        let eq26_e1489_d_n11: f64 = (eq26_e1485 * eq26_e1488_d_n11);
        let eq26_e1489_d_n12: f64 = (eq26_e1485 * eq26_e1488_d_n12);
        let eq26_e1489_d_n13: f64 = (eq26_e1485 * eq26_e1488_d_n13);
        let eq26_e1489_d_b0: f64 = (eq26_e1485 * eq26_e1488_d_b0);
        let eq26_e1489_d_b1: f64 = (eq26_e1485 * eq26_e1488_d_b1);
        let eq26_e1489_d_b2: f64 = (eq26_e1485 * eq26_e1488_d_b2);
        let eq26_e1489_d_b3: f64 = (eq26_e1485 * eq26_e1488_d_b3);
        let eq26_e1489_d_b4: f64 = (eq26_e1485 * eq26_e1488_d_b4);
        let eq26_e1489_d_b5: f64 = (eq26_e1485 * eq26_e1488_d_b5);
        let eq26_e1489_d_b6: f64 = (eq26_e1485 * eq26_e1488_d_b6);
        let eq26_e1489_d_b7: f64 = (eq26_e1485 * eq26_e1488_d_b7);
        let eq26_e1489_d_b8: f64 = (eq26_e1485 * eq26_e1488_d_b8);
        let eq26_e1489_d_b9: f64 = (eq26_e1485 * eq26_e1488_d_b9);
        let eq26_e1489_d_b10: f64 = (eq26_e1485 * eq26_e1488_d_b10);
        let eq26_e1489_d_b11: f64 = (eq26_e1485 * eq26_e1488_d_b11);
        let eq26_e1489_d_b12: f64 = (eq26_e1485 * eq26_e1488_d_b12);
        let eq26_e1489_d_b13: f64 = (eq26_e1485 * eq26_e1488_d_b13);
        let eq26_e1489_d_b14: f64 = (eq26_e1485 * eq26_e1488_d_b14);
        let eq26_e1489_d_b15: f64 = (eq26_e1485 * eq26_e1488_d_b15);
        let eq26_e1489_d_b16: f64 = (eq26_e1485 * eq26_e1488_d_b16);
        let eq26_e1489_d_b17: f64 = (eq26_e1485 * eq26_e1488_d_b17);
        let eq26_e1493: f64 = 0.0;
        let eq26_e1495: f64 = (eq26_e1493 * (nv8 - nv7));
        let eq26_e1496: f64 = (p.p32 * eq26_e1495);
        let eq26_e1496_d_n7: f64 = (p.p32 * (-eq26_e1493));
        let eq26_e1496_d_n8: f64 = (p.p32 * eq26_e1493);
        let eq26_e1497: f64 = (eq26_e1489 + eq26_e1496);
        let eq26_e1497_d_n7: f64 = (eq26_e1489_d_n7 + eq26_e1496_d_n7);
        let eq26_e1497_d_n8: f64 = (eq26_e1489_d_n8 + eq26_e1496_d_n8);
        (eq26_e1497, eq26_e1489_d_n0, eq26_e1489_d_n1, eq26_e1489_d_n2, eq26_e1489_d_n3, eq26_e1489_d_n4, eq26_e1489_d_n5, eq26_e1489_d_n6, eq26_e1497_d_n7, eq26_e1497_d_n8, eq26_e1489_d_n9, eq26_e1489_d_n10, eq26_e1489_d_n11, eq26_e1489_d_n12, eq26_e1489_d_n13, eq26_e1489_d_b0, eq26_e1489_d_b1, eq26_e1489_d_b2, eq26_e1489_d_b3, eq26_e1489_d_b4, eq26_e1489_d_b5, eq26_e1489_d_b6, eq26_e1489_d_b7, eq26_e1489_d_b8, eq26_e1489_d_b9, eq26_e1489_d_b10, eq26_e1489_d_b11, eq26_e1489_d_b12, eq26_e1489_d_b13, eq26_e1489_d_b14, eq26_e1489_d_b15, eq26_e1489_d_b16, eq26_e1489_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1499;
        let eq26_node_derivatives: [f64; 14] = [eq26_e1499_d_n0, eq26_e1499_d_n1, eq26_e1499_d_n2, eq26_e1499_d_n3, eq26_e1499_d_n4, eq26_e1499_d_n5, eq26_e1499_d_n6, eq26_e1499_d_n7, eq26_e1499_d_n8, eq26_e1499_d_n9, eq26_e1499_d_n10, eq26_e1499_d_n11, eq26_e1499_d_n12, eq26_e1499_d_n13];
        let eq26_branch_derivatives: [f64; 18] = [eq26_e1499_d_b0, eq26_e1499_d_b1, eq26_e1499_d_b2, eq26_e1499_d_b3, eq26_e1499_d_b4, eq26_e1499_d_b5, eq26_e1499_d_b6, eq26_e1499_d_b7, eq26_e1499_d_b8, eq26_e1499_d_b9, eq26_e1499_d_b10, eq26_e1499_d_b11, eq26_e1499_d_b12, eq26_e1499_d_b13, eq26_e1499_d_b14, eq26_e1499_d_b15, eq26_e1499_d_b16, eq26_e1499_d_b17];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1508, eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13, eq27_e1508_d_b0, eq27_e1508_d_b1, eq27_e1508_d_b2, eq27_e1508_d_b3, eq27_e1508_d_b4, eq27_e1508_d_b5, eq27_e1508_d_b6, eq27_e1508_d_b7, eq27_e1508_d_b8, eq27_e1508_d_b9, eq27_e1508_d_b10, eq27_e1508_d_b11, eq27_e1508_d_b12, eq27_e1508_d_b13, eq27_e1508_d_b14, eq27_e1508_d_b15, eq27_e1508_d_b16, eq27_e1508_d_b17,) = {
    if (!s.b[1548]) {
        let eq27_e1504: f64 = (p.p37 * p.p32);
        let eq27_e1506: f64 = (eq27_e1504 * s.v[908]);
        let eq27_e1506_d_n0: f64 = (eq27_e1504 * s.dn[908][0]);
        let eq27_e1506_d_n1: f64 = (eq27_e1504 * s.dn[908][1]);
        let eq27_e1506_d_n2: f64 = (eq27_e1504 * s.dn[908][2]);
        let eq27_e1506_d_n3: f64 = (eq27_e1504 * s.dn[908][3]);
        let eq27_e1506_d_n4: f64 = (eq27_e1504 * s.dn[908][4]);
        let eq27_e1506_d_n5: f64 = (eq27_e1504 * s.dn[908][5]);
        let eq27_e1506_d_n6: f64 = (eq27_e1504 * s.dn[908][6]);
        let eq27_e1506_d_n7: f64 = (eq27_e1504 * s.dn[908][7]);
        let eq27_e1506_d_n8: f64 = (eq27_e1504 * s.dn[908][8]);
        let eq27_e1506_d_n9: f64 = (eq27_e1504 * s.dn[908][9]);
        let eq27_e1506_d_n10: f64 = (eq27_e1504 * s.dn[908][10]);
        let eq27_e1506_d_n11: f64 = (eq27_e1504 * s.dn[908][11]);
        let eq27_e1506_d_n12: f64 = (eq27_e1504 * s.dn[908][12]);
        let eq27_e1506_d_n13: f64 = (eq27_e1504 * s.dn[908][13]);
        let eq27_e1506_d_b0: f64 = (eq27_e1504 * s.db[908][0]);
        let eq27_e1506_d_b1: f64 = (eq27_e1504 * s.db[908][1]);
        let eq27_e1506_d_b2: f64 = (eq27_e1504 * s.db[908][2]);
        let eq27_e1506_d_b3: f64 = (eq27_e1504 * s.db[908][3]);
        let eq27_e1506_d_b4: f64 = (eq27_e1504 * s.db[908][4]);
        let eq27_e1506_d_b5: f64 = (eq27_e1504 * s.db[908][5]);
        let eq27_e1506_d_b6: f64 = (eq27_e1504 * s.db[908][6]);
        let eq27_e1506_d_b7: f64 = (eq27_e1504 * s.db[908][7]);
        let eq27_e1506_d_b8: f64 = (eq27_e1504 * s.db[908][8]);
        let eq27_e1506_d_b9: f64 = (eq27_e1504 * s.db[908][9]);
        let eq27_e1506_d_b10: f64 = (eq27_e1504 * s.db[908][10]);
        let eq27_e1506_d_b11: f64 = (eq27_e1504 * s.db[908][11]);
        let eq27_e1506_d_b12: f64 = (eq27_e1504 * s.db[908][12]);
        let eq27_e1506_d_b13: f64 = (eq27_e1504 * s.db[908][13]);
        let eq27_e1506_d_b14: f64 = (eq27_e1504 * s.db[908][14]);
        let eq27_e1506_d_b15: f64 = (eq27_e1504 * s.db[908][15]);
        let eq27_e1506_d_b16: f64 = (eq27_e1504 * s.db[908][16]);
        let eq27_e1506_d_b17: f64 = (eq27_e1504 * s.db[908][17]);
        (eq27_e1506, eq27_e1506_d_n0, eq27_e1506_d_n1, eq27_e1506_d_n2, eq27_e1506_d_n3, eq27_e1506_d_n4, eq27_e1506_d_n5, eq27_e1506_d_n6, eq27_e1506_d_n7, eq27_e1506_d_n8, eq27_e1506_d_n9, eq27_e1506_d_n10, eq27_e1506_d_n11, eq27_e1506_d_n12, eq27_e1506_d_n13, eq27_e1506_d_b0, eq27_e1506_d_b1, eq27_e1506_d_b2, eq27_e1506_d_b3, eq27_e1506_d_b4, eq27_e1506_d_b5, eq27_e1506_d_b6, eq27_e1506_d_b7, eq27_e1506_d_b8, eq27_e1506_d_b9, eq27_e1506_d_b10, eq27_e1506_d_b11, eq27_e1506_d_b12, eq27_e1506_d_b13, eq27_e1506_d_b14, eq27_e1506_d_b15, eq27_e1506_d_b16, eq27_e1506_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1508;
        let eq27_node_derivatives: [f64; 14] = [eq27_e1508_d_n0, eq27_e1508_d_n1, eq27_e1508_d_n2, eq27_e1508_d_n3, eq27_e1508_d_n4, eq27_e1508_d_n5, eq27_e1508_d_n6, eq27_e1508_d_n7, eq27_e1508_d_n8, eq27_e1508_d_n9, eq27_e1508_d_n10, eq27_e1508_d_n11, eq27_e1508_d_n12, eq27_e1508_d_n13];
        let eq27_branch_derivatives: [f64; 18] = [eq27_e1508_d_b0, eq27_e1508_d_b1, eq27_e1508_d_b2, eq27_e1508_d_b3, eq27_e1508_d_b4, eq27_e1508_d_b5, eq27_e1508_d_b6, eq27_e1508_d_b7, eq27_e1508_d_b8, eq27_e1508_d_b9, eq27_e1508_d_b10, eq27_e1508_d_b11, eq27_e1508_d_b12, eq27_e1508_d_b13, eq27_e1508_d_b14, eq27_e1508_d_b15, eq27_e1508_d_b16, eq27_e1508_d_b17];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e1511: f64 = (p.p32 * s.v[88]);
        let eq28_e1511_d_n0: f64 = (p.p32 * s.dn[88][0]);
        let eq28_e1511_d_n1: f64 = (p.p32 * s.dn[88][1]);
        let eq28_e1511_d_n2: f64 = (p.p32 * s.dn[88][2]);
        let eq28_e1511_d_n3: f64 = (p.p32 * s.dn[88][3]);
        let eq28_e1511_d_n4: f64 = (p.p32 * s.dn[88][4]);
        let eq28_e1511_d_n5: f64 = (p.p32 * s.dn[88][5]);
        let eq28_e1511_d_n6: f64 = (p.p32 * s.dn[88][6]);
        let eq28_e1511_d_n7: f64 = (p.p32 * s.dn[88][7]);
        let eq28_e1511_d_n8: f64 = (p.p32 * s.dn[88][8]);
        let eq28_e1511_d_n9: f64 = (p.p32 * s.dn[88][9]);
        let eq28_e1511_d_n10: f64 = (p.p32 * s.dn[88][10]);
        let eq28_e1511_d_n11: f64 = (p.p32 * s.dn[88][11]);
        let eq28_e1511_d_n12: f64 = (p.p32 * s.dn[88][12]);
        let eq28_e1511_d_n13: f64 = (p.p32 * s.dn[88][13]);
        let eq28_e1511_d_b0: f64 = (p.p32 * s.db[88][0]);
        let eq28_e1511_d_b1: f64 = (p.p32 * s.db[88][1]);
        let eq28_e1511_d_b2: f64 = (p.p32 * s.db[88][2]);
        let eq28_e1511_d_b3: f64 = (p.p32 * s.db[88][3]);
        let eq28_e1511_d_b4: f64 = (p.p32 * s.db[88][4]);
        let eq28_e1511_d_b5: f64 = (p.p32 * s.db[88][5]);
        let eq28_e1511_d_b6: f64 = (p.p32 * s.db[88][6]);
        let eq28_e1511_d_b7: f64 = (p.p32 * s.db[88][7]);
        let eq28_e1511_d_b8: f64 = (p.p32 * s.db[88][8]);
        let eq28_e1511_d_b9: f64 = (p.p32 * s.db[88][9]);
        let eq28_e1511_d_b10: f64 = (p.p32 * s.db[88][10]);
        let eq28_e1511_d_b11: f64 = (p.p32 * s.db[88][11]);
        let eq28_e1511_d_b12: f64 = (p.p32 * s.db[88][12]);
        let eq28_e1511_d_b13: f64 = (p.p32 * s.db[88][13]);
        let eq28_e1511_d_b14: f64 = (p.p32 * s.db[88][14]);
        let eq28_e1511_d_b15: f64 = (p.p32 * s.db[88][15]);
        let eq28_e1511_d_b16: f64 = (p.p32 * s.db[88][16]);
        let eq28_e1511_d_b17: f64 = (p.p32 * s.db[88][17]);
        let eq28_value: f64 = eq28_e1511;
        let eq28_node_derivatives: [f64; 14] = [eq28_e1511_d_n0, eq28_e1511_d_n1, eq28_e1511_d_n2, eq28_e1511_d_n3, eq28_e1511_d_n4, eq28_e1511_d_n5, eq28_e1511_d_n6, eq28_e1511_d_n7, eq28_e1511_d_n8, eq28_e1511_d_n9, eq28_e1511_d_n10, eq28_e1511_d_n11, eq28_e1511_d_n12, eq28_e1511_d_n13];
        let eq28_branch_derivatives: [f64; 18] = [eq28_e1511_d_b0, eq28_e1511_d_b1, eq28_e1511_d_b2, eq28_e1511_d_b3, eq28_e1511_d_b4, eq28_e1511_d_b5, eq28_e1511_d_b6, eq28_e1511_d_b7, eq28_e1511_d_b8, eq28_e1511_d_b9, eq28_e1511_d_b10, eq28_e1511_d_b11, eq28_e1511_d_b12, eq28_e1511_d_b13, eq28_e1511_d_b14, eq28_e1511_d_b15, eq28_e1511_d_b16, eq28_e1511_d_b17];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq28_value),
            &eq28_node_derivatives,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1514: f64 = (p.p32 * s.v[89]);
        let eq29_e1514_d_n0: f64 = (p.p32 * s.dn[89][0]);
        let eq29_e1514_d_n1: f64 = (p.p32 * s.dn[89][1]);
        let eq29_e1514_d_n2: f64 = (p.p32 * s.dn[89][2]);
        let eq29_e1514_d_n3: f64 = (p.p32 * s.dn[89][3]);
        let eq29_e1514_d_n4: f64 = (p.p32 * s.dn[89][4]);
        let eq29_e1514_d_n5: f64 = (p.p32 * s.dn[89][5]);
        let eq29_e1514_d_n6: f64 = (p.p32 * s.dn[89][6]);
        let eq29_e1514_d_n7: f64 = (p.p32 * s.dn[89][7]);
        let eq29_e1514_d_n8: f64 = (p.p32 * s.dn[89][8]);
        let eq29_e1514_d_n9: f64 = (p.p32 * s.dn[89][9]);
        let eq29_e1514_d_n10: f64 = (p.p32 * s.dn[89][10]);
        let eq29_e1514_d_n11: f64 = (p.p32 * s.dn[89][11]);
        let eq29_e1514_d_n12: f64 = (p.p32 * s.dn[89][12]);
        let eq29_e1514_d_n13: f64 = (p.p32 * s.dn[89][13]);
        let eq29_e1514_d_b0: f64 = (p.p32 * s.db[89][0]);
        let eq29_e1514_d_b1: f64 = (p.p32 * s.db[89][1]);
        let eq29_e1514_d_b2: f64 = (p.p32 * s.db[89][2]);
        let eq29_e1514_d_b3: f64 = (p.p32 * s.db[89][3]);
        let eq29_e1514_d_b4: f64 = (p.p32 * s.db[89][4]);
        let eq29_e1514_d_b5: f64 = (p.p32 * s.db[89][5]);
        let eq29_e1514_d_b6: f64 = (p.p32 * s.db[89][6]);
        let eq29_e1514_d_b7: f64 = (p.p32 * s.db[89][7]);
        let eq29_e1514_d_b8: f64 = (p.p32 * s.db[89][8]);
        let eq29_e1514_d_b9: f64 = (p.p32 * s.db[89][9]);
        let eq29_e1514_d_b10: f64 = (p.p32 * s.db[89][10]);
        let eq29_e1514_d_b11: f64 = (p.p32 * s.db[89][11]);
        let eq29_e1514_d_b12: f64 = (p.p32 * s.db[89][12]);
        let eq29_e1514_d_b13: f64 = (p.p32 * s.db[89][13]);
        let eq29_e1514_d_b14: f64 = (p.p32 * s.db[89][14]);
        let eq29_e1514_d_b15: f64 = (p.p32 * s.db[89][15]);
        let eq29_e1514_d_b16: f64 = (p.p32 * s.db[89][16]);
        let eq29_e1514_d_b17: f64 = (p.p32 * s.db[89][17]);
        let eq29_value: f64 = eq29_e1514;
        let eq29_node_derivatives: [f64; 14] = [eq29_e1514_d_n0, eq29_e1514_d_n1, eq29_e1514_d_n2, eq29_e1514_d_n3, eq29_e1514_d_n4, eq29_e1514_d_n5, eq29_e1514_d_n6, eq29_e1514_d_n7, eq29_e1514_d_n8, eq29_e1514_d_n9, eq29_e1514_d_n10, eq29_e1514_d_n11, eq29_e1514_d_n12, eq29_e1514_d_n13];
        let eq29_branch_derivatives: [f64; 18] = [eq29_e1514_d_b0, eq29_e1514_d_b1, eq29_e1514_d_b2, eq29_e1514_d_b3, eq29_e1514_d_b4, eq29_e1514_d_b5, eq29_e1514_d_b6, eq29_e1514_d_b7, eq29_e1514_d_b8, eq29_e1514_d_b9, eq29_e1514_d_b10, eq29_e1514_d_b11, eq29_e1514_d_b12, eq29_e1514_d_b13, eq29_e1514_d_b14, eq29_e1514_d_b15, eq29_e1514_d_b16, eq29_e1514_d_b17];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq29_value),
            &eq29_node_derivatives,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let eq30_e1517: f64 = (p.p37 * p.p32);
        let eq30_e1519: f64 = (eq30_e1517 * s.v[935]);
        let eq30_e1519_d_n0: f64 = (eq30_e1517 * s.dn[935][0]);
        let eq30_e1519_d_n1: f64 = (eq30_e1517 * s.dn[935][1]);
        let eq30_e1519_d_n2: f64 = (eq30_e1517 * s.dn[935][2]);
        let eq30_e1519_d_n3: f64 = (eq30_e1517 * s.dn[935][3]);
        let eq30_e1519_d_n4: f64 = (eq30_e1517 * s.dn[935][4]);
        let eq30_e1519_d_n5: f64 = (eq30_e1517 * s.dn[935][5]);
        let eq30_e1519_d_n6: f64 = (eq30_e1517 * s.dn[935][6]);
        let eq30_e1519_d_n7: f64 = (eq30_e1517 * s.dn[935][7]);
        let eq30_e1519_d_n8: f64 = (eq30_e1517 * s.dn[935][8]);
        let eq30_e1519_d_n9: f64 = (eq30_e1517 * s.dn[935][9]);
        let eq30_e1519_d_n10: f64 = (eq30_e1517 * s.dn[935][10]);
        let eq30_e1519_d_n11: f64 = (eq30_e1517 * s.dn[935][11]);
        let eq30_e1519_d_n12: f64 = (eq30_e1517 * s.dn[935][12]);
        let eq30_e1519_d_n13: f64 = (eq30_e1517 * s.dn[935][13]);
        let eq30_e1519_d_b0: f64 = (eq30_e1517 * s.db[935][0]);
        let eq30_e1519_d_b1: f64 = (eq30_e1517 * s.db[935][1]);
        let eq30_e1519_d_b2: f64 = (eq30_e1517 * s.db[935][2]);
        let eq30_e1519_d_b3: f64 = (eq30_e1517 * s.db[935][3]);
        let eq30_e1519_d_b4: f64 = (eq30_e1517 * s.db[935][4]);
        let eq30_e1519_d_b5: f64 = (eq30_e1517 * s.db[935][5]);
        let eq30_e1519_d_b6: f64 = (eq30_e1517 * s.db[935][6]);
        let eq30_e1519_d_b7: f64 = (eq30_e1517 * s.db[935][7]);
        let eq30_e1519_d_b8: f64 = (eq30_e1517 * s.db[935][8]);
        let eq30_e1519_d_b9: f64 = (eq30_e1517 * s.db[935][9]);
        let eq30_e1519_d_b10: f64 = (eq30_e1517 * s.db[935][10]);
        let eq30_e1519_d_b11: f64 = (eq30_e1517 * s.db[935][11]);
        let eq30_e1519_d_b12: f64 = (eq30_e1517 * s.db[935][12]);
        let eq30_e1519_d_b13: f64 = (eq30_e1517 * s.db[935][13]);
        let eq30_e1519_d_b14: f64 = (eq30_e1517 * s.db[935][14]);
        let eq30_e1519_d_b15: f64 = (eq30_e1517 * s.db[935][15]);
        let eq30_e1519_d_b16: f64 = (eq30_e1517 * s.db[935][16]);
        let eq30_e1519_d_b17: f64 = (eq30_e1517 * s.db[935][17]);
        let eq30_value: f64 = eq30_e1519;
        let eq30_node_derivatives: [f64; 14] = [eq30_e1519_d_n0, eq30_e1519_d_n1, eq30_e1519_d_n2, eq30_e1519_d_n3, eq30_e1519_d_n4, eq30_e1519_d_n5, eq30_e1519_d_n6, eq30_e1519_d_n7, eq30_e1519_d_n8, eq30_e1519_d_n9, eq30_e1519_d_n10, eq30_e1519_d_n11, eq30_e1519_d_n12, eq30_e1519_d_n13];
        let eq30_branch_derivatives: [f64; 18] = [eq30_e1519_d_b0, eq30_e1519_d_b1, eq30_e1519_d_b2, eq30_e1519_d_b3, eq30_e1519_d_b4, eq30_e1519_d_b5, eq30_e1519_d_b6, eq30_e1519_d_b7, eq30_e1519_d_b8, eq30_e1519_d_b9, eq30_e1519_d_b10, eq30_e1519_d_b11, eq30_e1519_d_b12, eq30_e1519_d_b13, eq30_e1519_d_b14, eq30_e1519_d_b15, eq30_e1519_d_b16, eq30_e1519_d_b17];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1522: f64 = (p.p37 * p.p32);
        let eq31_e1524: f64 = (eq31_e1522 * s.v[934]);
        let eq31_e1524_d_n0: f64 = (eq31_e1522 * s.dn[934][0]);
        let eq31_e1524_d_n1: f64 = (eq31_e1522 * s.dn[934][1]);
        let eq31_e1524_d_n2: f64 = (eq31_e1522 * s.dn[934][2]);
        let eq31_e1524_d_n3: f64 = (eq31_e1522 * s.dn[934][3]);
        let eq31_e1524_d_n4: f64 = (eq31_e1522 * s.dn[934][4]);
        let eq31_e1524_d_n5: f64 = (eq31_e1522 * s.dn[934][5]);
        let eq31_e1524_d_n6: f64 = (eq31_e1522 * s.dn[934][6]);
        let eq31_e1524_d_n7: f64 = (eq31_e1522 * s.dn[934][7]);
        let eq31_e1524_d_n8: f64 = (eq31_e1522 * s.dn[934][8]);
        let eq31_e1524_d_n9: f64 = (eq31_e1522 * s.dn[934][9]);
        let eq31_e1524_d_n10: f64 = (eq31_e1522 * s.dn[934][10]);
        let eq31_e1524_d_n11: f64 = (eq31_e1522 * s.dn[934][11]);
        let eq31_e1524_d_n12: f64 = (eq31_e1522 * s.dn[934][12]);
        let eq31_e1524_d_n13: f64 = (eq31_e1522 * s.dn[934][13]);
        let eq31_e1524_d_b0: f64 = (eq31_e1522 * s.db[934][0]);
        let eq31_e1524_d_b1: f64 = (eq31_e1522 * s.db[934][1]);
        let eq31_e1524_d_b2: f64 = (eq31_e1522 * s.db[934][2]);
        let eq31_e1524_d_b3: f64 = (eq31_e1522 * s.db[934][3]);
        let eq31_e1524_d_b4: f64 = (eq31_e1522 * s.db[934][4]);
        let eq31_e1524_d_b5: f64 = (eq31_e1522 * s.db[934][5]);
        let eq31_e1524_d_b6: f64 = (eq31_e1522 * s.db[934][6]);
        let eq31_e1524_d_b7: f64 = (eq31_e1522 * s.db[934][7]);
        let eq31_e1524_d_b8: f64 = (eq31_e1522 * s.db[934][8]);
        let eq31_e1524_d_b9: f64 = (eq31_e1522 * s.db[934][9]);
        let eq31_e1524_d_b10: f64 = (eq31_e1522 * s.db[934][10]);
        let eq31_e1524_d_b11: f64 = (eq31_e1522 * s.db[934][11]);
        let eq31_e1524_d_b12: f64 = (eq31_e1522 * s.db[934][12]);
        let eq31_e1524_d_b13: f64 = (eq31_e1522 * s.db[934][13]);
        let eq31_e1524_d_b14: f64 = (eq31_e1522 * s.db[934][14]);
        let eq31_e1524_d_b15: f64 = (eq31_e1522 * s.db[934][15]);
        let eq31_e1524_d_b16: f64 = (eq31_e1522 * s.db[934][16]);
        let eq31_e1524_d_b17: f64 = (eq31_e1522 * s.db[934][17]);
        let eq31_value: f64 = eq31_e1524;
        let eq31_node_derivatives: [f64; 14] = [eq31_e1524_d_n0, eq31_e1524_d_n1, eq31_e1524_d_n2, eq31_e1524_d_n3, eq31_e1524_d_n4, eq31_e1524_d_n5, eq31_e1524_d_n6, eq31_e1524_d_n7, eq31_e1524_d_n8, eq31_e1524_d_n9, eq31_e1524_d_n10, eq31_e1524_d_n11, eq31_e1524_d_n12, eq31_e1524_d_n13];
        let eq31_branch_derivatives: [f64; 18] = [eq31_e1524_d_b0, eq31_e1524_d_b1, eq31_e1524_d_b2, eq31_e1524_d_b3, eq31_e1524_d_b4, eq31_e1524_d_b5, eq31_e1524_d_b6, eq31_e1524_d_b7, eq31_e1524_d_b8, eq31_e1524_d_b9, eq31_e1524_d_b10, eq31_e1524_d_b11, eq31_e1524_d_b12, eq31_e1524_d_b13, eq31_e1524_d_b14, eq31_e1524_d_b15, eq31_e1524_d_b16, eq31_e1524_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1528: f64 = (s.v[94] + s.v[90]);
        let eq32_e1528_d_n0: f64 = (s.dn[94][0] + s.dn[90][0]);
        let eq32_e1528_d_n1: f64 = (s.dn[94][1] + s.dn[90][1]);
        let eq32_e1528_d_n2: f64 = (s.dn[94][2] + s.dn[90][2]);
        let eq32_e1528_d_n3: f64 = (s.dn[94][3] + s.dn[90][3]);
        let eq32_e1528_d_n4: f64 = (s.dn[94][4] + s.dn[90][4]);
        let eq32_e1528_d_n5: f64 = (s.dn[94][5] + s.dn[90][5]);
        let eq32_e1528_d_n6: f64 = (s.dn[94][6] + s.dn[90][6]);
        let eq32_e1528_d_n7: f64 = (s.dn[94][7] + s.dn[90][7]);
        let eq32_e1528_d_n8: f64 = (s.dn[94][8] + s.dn[90][8]);
        let eq32_e1528_d_n9: f64 = (s.dn[94][9] + s.dn[90][9]);
        let eq32_e1528_d_n10: f64 = (s.dn[94][10] + s.dn[90][10]);
        let eq32_e1528_d_n11: f64 = (s.dn[94][11] + s.dn[90][11]);
        let eq32_e1528_d_n12: f64 = (s.dn[94][12] + s.dn[90][12]);
        let eq32_e1528_d_n13: f64 = (s.dn[94][13] + s.dn[90][13]);
        let eq32_e1528_d_b0: f64 = (s.db[94][0] + s.db[90][0]);
        let eq32_e1528_d_b1: f64 = (s.db[94][1] + s.db[90][1]);
        let eq32_e1528_d_b2: f64 = (s.db[94][2] + s.db[90][2]);
        let eq32_e1528_d_b3: f64 = (s.db[94][3] + s.db[90][3]);
        let eq32_e1528_d_b4: f64 = (s.db[94][4] + s.db[90][4]);
        let eq32_e1528_d_b5: f64 = (s.db[94][5] + s.db[90][5]);
        let eq32_e1528_d_b6: f64 = (s.db[94][6] + s.db[90][6]);
        let eq32_e1528_d_b7: f64 = (s.db[94][7] + s.db[90][7]);
        let eq32_e1528_d_b8: f64 = (s.db[94][8] + s.db[90][8]);
        let eq32_e1528_d_b9: f64 = (s.db[94][9] + s.db[90][9]);
        let eq32_e1528_d_b10: f64 = (s.db[94][10] + s.db[90][10]);
        let eq32_e1528_d_b11: f64 = (s.db[94][11] + s.db[90][11]);
        let eq32_e1528_d_b12: f64 = (s.db[94][12] + s.db[90][12]);
        let eq32_e1528_d_b13: f64 = (s.db[94][13] + s.db[90][13]);
        let eq32_e1528_d_b14: f64 = (s.db[94][14] + s.db[90][14]);
        let eq32_e1528_d_b15: f64 = (s.db[94][15] + s.db[90][15]);
        let eq32_e1528_d_b16: f64 = (s.db[94][16] + s.db[90][16]);
        let eq32_e1528_d_b17: f64 = (s.db[94][17] + s.db[90][17]);
        let eq32_e1529: f64 = (p.p32 * eq32_e1528);
        let eq32_e1529_d_n0: f64 = (p.p32 * eq32_e1528_d_n0);
        let eq32_e1529_d_n1: f64 = (p.p32 * eq32_e1528_d_n1);
        let eq32_e1529_d_n2: f64 = (p.p32 * eq32_e1528_d_n2);
        let eq32_e1529_d_n3: f64 = (p.p32 * eq32_e1528_d_n3);
        let eq32_e1529_d_n4: f64 = (p.p32 * eq32_e1528_d_n4);
        let eq32_e1529_d_n5: f64 = (p.p32 * eq32_e1528_d_n5);
        let eq32_e1529_d_n6: f64 = (p.p32 * eq32_e1528_d_n6);
        let eq32_e1529_d_n7: f64 = (p.p32 * eq32_e1528_d_n7);
        let eq32_e1529_d_n8: f64 = (p.p32 * eq32_e1528_d_n8);
        let eq32_e1529_d_n9: f64 = (p.p32 * eq32_e1528_d_n9);
        let eq32_e1529_d_n10: f64 = (p.p32 * eq32_e1528_d_n10);
        let eq32_e1529_d_n11: f64 = (p.p32 * eq32_e1528_d_n11);
        let eq32_e1529_d_n12: f64 = (p.p32 * eq32_e1528_d_n12);
        let eq32_e1529_d_n13: f64 = (p.p32 * eq32_e1528_d_n13);
        let eq32_e1529_d_b0: f64 = (p.p32 * eq32_e1528_d_b0);
        let eq32_e1529_d_b1: f64 = (p.p32 * eq32_e1528_d_b1);
        let eq32_e1529_d_b2: f64 = (p.p32 * eq32_e1528_d_b2);
        let eq32_e1529_d_b3: f64 = (p.p32 * eq32_e1528_d_b3);
        let eq32_e1529_d_b4: f64 = (p.p32 * eq32_e1528_d_b4);
        let eq32_e1529_d_b5: f64 = (p.p32 * eq32_e1528_d_b5);
        let eq32_e1529_d_b6: f64 = (p.p32 * eq32_e1528_d_b6);
        let eq32_e1529_d_b7: f64 = (p.p32 * eq32_e1528_d_b7);
        let eq32_e1529_d_b8: f64 = (p.p32 * eq32_e1528_d_b8);
        let eq32_e1529_d_b9: f64 = (p.p32 * eq32_e1528_d_b9);
        let eq32_e1529_d_b10: f64 = (p.p32 * eq32_e1528_d_b10);
        let eq32_e1529_d_b11: f64 = (p.p32 * eq32_e1528_d_b11);
        let eq32_e1529_d_b12: f64 = (p.p32 * eq32_e1528_d_b12);
        let eq32_e1529_d_b13: f64 = (p.p32 * eq32_e1528_d_b13);
        let eq32_e1529_d_b14: f64 = (p.p32 * eq32_e1528_d_b14);
        let eq32_e1529_d_b15: f64 = (p.p32 * eq32_e1528_d_b15);
        let eq32_e1529_d_b16: f64 = (p.p32 * eq32_e1528_d_b16);
        let eq32_e1529_d_b17: f64 = (p.p32 * eq32_e1528_d_b17);
        let eq32_e1533: f64 = 0.0;
        let eq32_e1535: f64 = (eq32_e1533 * (nv9 - nv7));
        let eq32_e1536: f64 = (p.p32 * eq32_e1535);
        let eq32_e1536_d_n7: f64 = (p.p32 * (-eq32_e1533));
        let eq32_e1536_d_n9: f64 = (p.p32 * eq32_e1533);
        let eq32_e1537: f64 = (eq32_e1529 + eq32_e1536);
        let eq32_e1537_d_n7: f64 = (eq32_e1529_d_n7 + eq32_e1536_d_n7);
        let eq32_e1537_d_n9: f64 = (eq32_e1529_d_n9 + eq32_e1536_d_n9);
        let eq32_value: f64 = eq32_e1537;
        let eq32_node_derivatives: [f64; 14] = [eq32_e1529_d_n0, eq32_e1529_d_n1, eq32_e1529_d_n2, eq32_e1529_d_n3, eq32_e1529_d_n4, eq32_e1529_d_n5, eq32_e1529_d_n6, eq32_e1537_d_n7, eq32_e1529_d_n8, eq32_e1537_d_n9, eq32_e1529_d_n10, eq32_e1529_d_n11, eq32_e1529_d_n12, eq32_e1529_d_n13];
        let eq32_branch_derivatives: [f64; 18] = [eq32_e1529_d_b0, eq32_e1529_d_b1, eq32_e1529_d_b2, eq32_e1529_d_b3, eq32_e1529_d_b4, eq32_e1529_d_b5, eq32_e1529_d_b6, eq32_e1529_d_b7, eq32_e1529_d_b8, eq32_e1529_d_b9, eq32_e1529_d_b10, eq32_e1529_d_b11, eq32_e1529_d_b12, eq32_e1529_d_b13, eq32_e1529_d_b14, eq32_e1529_d_b15, eq32_e1529_d_b16, eq32_e1529_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1541: f64 = (s.v[95] + s.v[91]);
        let eq33_e1541_d_n0: f64 = (s.dn[95][0] + s.dn[91][0]);
        let eq33_e1541_d_n1: f64 = (s.dn[95][1] + s.dn[91][1]);
        let eq33_e1541_d_n2: f64 = (s.dn[95][2] + s.dn[91][2]);
        let eq33_e1541_d_n3: f64 = (s.dn[95][3] + s.dn[91][3]);
        let eq33_e1541_d_n4: f64 = (s.dn[95][4] + s.dn[91][4]);
        let eq33_e1541_d_n5: f64 = (s.dn[95][5] + s.dn[91][5]);
        let eq33_e1541_d_n6: f64 = (s.dn[95][6] + s.dn[91][6]);
        let eq33_e1541_d_n7: f64 = (s.dn[95][7] + s.dn[91][7]);
        let eq33_e1541_d_n8: f64 = (s.dn[95][8] + s.dn[91][8]);
        let eq33_e1541_d_n9: f64 = (s.dn[95][9] + s.dn[91][9]);
        let eq33_e1541_d_n10: f64 = (s.dn[95][10] + s.dn[91][10]);
        let eq33_e1541_d_n11: f64 = (s.dn[95][11] + s.dn[91][11]);
        let eq33_e1541_d_n12: f64 = (s.dn[95][12] + s.dn[91][12]);
        let eq33_e1541_d_n13: f64 = (s.dn[95][13] + s.dn[91][13]);
        let eq33_e1541_d_b0: f64 = (s.db[95][0] + s.db[91][0]);
        let eq33_e1541_d_b1: f64 = (s.db[95][1] + s.db[91][1]);
        let eq33_e1541_d_b2: f64 = (s.db[95][2] + s.db[91][2]);
        let eq33_e1541_d_b3: f64 = (s.db[95][3] + s.db[91][3]);
        let eq33_e1541_d_b4: f64 = (s.db[95][4] + s.db[91][4]);
        let eq33_e1541_d_b5: f64 = (s.db[95][5] + s.db[91][5]);
        let eq33_e1541_d_b6: f64 = (s.db[95][6] + s.db[91][6]);
        let eq33_e1541_d_b7: f64 = (s.db[95][7] + s.db[91][7]);
        let eq33_e1541_d_b8: f64 = (s.db[95][8] + s.db[91][8]);
        let eq33_e1541_d_b9: f64 = (s.db[95][9] + s.db[91][9]);
        let eq33_e1541_d_b10: f64 = (s.db[95][10] + s.db[91][10]);
        let eq33_e1541_d_b11: f64 = (s.db[95][11] + s.db[91][11]);
        let eq33_e1541_d_b12: f64 = (s.db[95][12] + s.db[91][12]);
        let eq33_e1541_d_b13: f64 = (s.db[95][13] + s.db[91][13]);
        let eq33_e1541_d_b14: f64 = (s.db[95][14] + s.db[91][14]);
        let eq33_e1541_d_b15: f64 = (s.db[95][15] + s.db[91][15]);
        let eq33_e1541_d_b16: f64 = (s.db[95][16] + s.db[91][16]);
        let eq33_e1541_d_b17: f64 = (s.db[95][17] + s.db[91][17]);
        let eq33_e1542: f64 = (p.p32 * eq33_e1541);
        let eq33_e1542_d_n0: f64 = (p.p32 * eq33_e1541_d_n0);
        let eq33_e1542_d_n1: f64 = (p.p32 * eq33_e1541_d_n1);
        let eq33_e1542_d_n2: f64 = (p.p32 * eq33_e1541_d_n2);
        let eq33_e1542_d_n3: f64 = (p.p32 * eq33_e1541_d_n3);
        let eq33_e1542_d_n4: f64 = (p.p32 * eq33_e1541_d_n4);
        let eq33_e1542_d_n5: f64 = (p.p32 * eq33_e1541_d_n5);
        let eq33_e1542_d_n6: f64 = (p.p32 * eq33_e1541_d_n6);
        let eq33_e1542_d_n7: f64 = (p.p32 * eq33_e1541_d_n7);
        let eq33_e1542_d_n8: f64 = (p.p32 * eq33_e1541_d_n8);
        let eq33_e1542_d_n9: f64 = (p.p32 * eq33_e1541_d_n9);
        let eq33_e1542_d_n10: f64 = (p.p32 * eq33_e1541_d_n10);
        let eq33_e1542_d_n11: f64 = (p.p32 * eq33_e1541_d_n11);
        let eq33_e1542_d_n12: f64 = (p.p32 * eq33_e1541_d_n12);
        let eq33_e1542_d_n13: f64 = (p.p32 * eq33_e1541_d_n13);
        let eq33_e1542_d_b0: f64 = (p.p32 * eq33_e1541_d_b0);
        let eq33_e1542_d_b1: f64 = (p.p32 * eq33_e1541_d_b1);
        let eq33_e1542_d_b2: f64 = (p.p32 * eq33_e1541_d_b2);
        let eq33_e1542_d_b3: f64 = (p.p32 * eq33_e1541_d_b3);
        let eq33_e1542_d_b4: f64 = (p.p32 * eq33_e1541_d_b4);
        let eq33_e1542_d_b5: f64 = (p.p32 * eq33_e1541_d_b5);
        let eq33_e1542_d_b6: f64 = (p.p32 * eq33_e1541_d_b6);
        let eq33_e1542_d_b7: f64 = (p.p32 * eq33_e1541_d_b7);
        let eq33_e1542_d_b8: f64 = (p.p32 * eq33_e1541_d_b8);
        let eq33_e1542_d_b9: f64 = (p.p32 * eq33_e1541_d_b9);
        let eq33_e1542_d_b10: f64 = (p.p32 * eq33_e1541_d_b10);
        let eq33_e1542_d_b11: f64 = (p.p32 * eq33_e1541_d_b11);
        let eq33_e1542_d_b12: f64 = (p.p32 * eq33_e1541_d_b12);
        let eq33_e1542_d_b13: f64 = (p.p32 * eq33_e1541_d_b13);
        let eq33_e1542_d_b14: f64 = (p.p32 * eq33_e1541_d_b14);
        let eq33_e1542_d_b15: f64 = (p.p32 * eq33_e1541_d_b15);
        let eq33_e1542_d_b16: f64 = (p.p32 * eq33_e1541_d_b16);
        let eq33_e1542_d_b17: f64 = (p.p32 * eq33_e1541_d_b17);
        let eq33_e1546: f64 = 0.0;
        let eq33_e1548: f64 = (eq33_e1546 * (nv9 - nv8));
        let eq33_e1549: f64 = (p.p32 * eq33_e1548);
        let eq33_e1549_d_n8: f64 = (p.p32 * (-eq33_e1546));
        let eq33_e1549_d_n9: f64 = (p.p32 * eq33_e1546);
        let eq33_e1550: f64 = (eq33_e1542 + eq33_e1549);
        let eq33_e1550_d_n8: f64 = (eq33_e1542_d_n8 + eq33_e1549_d_n8);
        let eq33_e1550_d_n9: f64 = (eq33_e1542_d_n9 + eq33_e1549_d_n9);
        let eq33_value: f64 = eq33_e1550;
        let eq33_node_derivatives: [f64; 14] = [eq33_e1542_d_n0, eq33_e1542_d_n1, eq33_e1542_d_n2, eq33_e1542_d_n3, eq33_e1542_d_n4, eq33_e1542_d_n5, eq33_e1542_d_n6, eq33_e1542_d_n7, eq33_e1550_d_n8, eq33_e1550_d_n9, eq33_e1542_d_n10, eq33_e1542_d_n11, eq33_e1542_d_n12, eq33_e1542_d_n13];
        let eq33_branch_derivatives: [f64; 18] = [eq33_e1542_d_b0, eq33_e1542_d_b1, eq33_e1542_d_b2, eq33_e1542_d_b3, eq33_e1542_d_b4, eq33_e1542_d_b5, eq33_e1542_d_b6, eq33_e1542_d_b7, eq33_e1542_d_b8, eq33_e1542_d_b9, eq33_e1542_d_b10, eq33_e1542_d_b11, eq33_e1542_d_b12, eq33_e1542_d_b13, eq33_e1542_d_b14, eq33_e1542_d_b15, eq33_e1542_d_b16, eq33_e1542_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1553: f64 = (p.p32 * s.v[79]);
        let eq34_e1553_d_n0: f64 = (p.p32 * s.dn[79][0]);
        let eq34_e1553_d_n1: f64 = (p.p32 * s.dn[79][1]);
        let eq34_e1553_d_n2: f64 = (p.p32 * s.dn[79][2]);
        let eq34_e1553_d_n3: f64 = (p.p32 * s.dn[79][3]);
        let eq34_e1553_d_n4: f64 = (p.p32 * s.dn[79][4]);
        let eq34_e1553_d_n5: f64 = (p.p32 * s.dn[79][5]);
        let eq34_e1553_d_n6: f64 = (p.p32 * s.dn[79][6]);
        let eq34_e1553_d_n7: f64 = (p.p32 * s.dn[79][7]);
        let eq34_e1553_d_n8: f64 = (p.p32 * s.dn[79][8]);
        let eq34_e1553_d_n9: f64 = (p.p32 * s.dn[79][9]);
        let eq34_e1553_d_n10: f64 = (p.p32 * s.dn[79][10]);
        let eq34_e1553_d_n11: f64 = (p.p32 * s.dn[79][11]);
        let eq34_e1553_d_n12: f64 = (p.p32 * s.dn[79][12]);
        let eq34_e1553_d_n13: f64 = (p.p32 * s.dn[79][13]);
        let eq34_e1553_d_b0: f64 = (p.p32 * s.db[79][0]);
        let eq34_e1553_d_b1: f64 = (p.p32 * s.db[79][1]);
        let eq34_e1553_d_b2: f64 = (p.p32 * s.db[79][2]);
        let eq34_e1553_d_b3: f64 = (p.p32 * s.db[79][3]);
        let eq34_e1553_d_b4: f64 = (p.p32 * s.db[79][4]);
        let eq34_e1553_d_b5: f64 = (p.p32 * s.db[79][5]);
        let eq34_e1553_d_b6: f64 = (p.p32 * s.db[79][6]);
        let eq34_e1553_d_b7: f64 = (p.p32 * s.db[79][7]);
        let eq34_e1553_d_b8: f64 = (p.p32 * s.db[79][8]);
        let eq34_e1553_d_b9: f64 = (p.p32 * s.db[79][9]);
        let eq34_e1553_d_b10: f64 = (p.p32 * s.db[79][10]);
        let eq34_e1553_d_b11: f64 = (p.p32 * s.db[79][11]);
        let eq34_e1553_d_b12: f64 = (p.p32 * s.db[79][12]);
        let eq34_e1553_d_b13: f64 = (p.p32 * s.db[79][13]);
        let eq34_e1553_d_b14: f64 = (p.p32 * s.db[79][14]);
        let eq34_e1553_d_b15: f64 = (p.p32 * s.db[79][15]);
        let eq34_e1553_d_b16: f64 = (p.p32 * s.db[79][16]);
        let eq34_e1553_d_b17: f64 = (p.p32 * s.db[79][17]);
        let eq34_value: f64 = eq34_e1553;
        let eq34_node_derivatives: [f64; 14] = [eq34_e1553_d_n0, eq34_e1553_d_n1, eq34_e1553_d_n2, eq34_e1553_d_n3, eq34_e1553_d_n4, eq34_e1553_d_n5, eq34_e1553_d_n6, eq34_e1553_d_n7, eq34_e1553_d_n8, eq34_e1553_d_n9, eq34_e1553_d_n10, eq34_e1553_d_n11, eq34_e1553_d_n12, eq34_e1553_d_n13];
        let eq34_branch_derivatives: [f64; 18] = [eq34_e1553_d_b0, eq34_e1553_d_b1, eq34_e1553_d_b2, eq34_e1553_d_b3, eq34_e1553_d_b4, eq34_e1553_d_b5, eq34_e1553_d_b6, eq34_e1553_d_b7, eq34_e1553_d_b8, eq34_e1553_d_b9, eq34_e1553_d_b10, eq34_e1553_d_b11, eq34_e1553_d_b12, eq34_e1553_d_b13, eq34_e1553_d_b14, eq34_e1553_d_b15, eq34_e1553_d_b16, eq34_e1553_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq34_value),
            &eq34_node_derivatives,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let eq35_e1556: f64 = (p.p32 * s.v[80]);
        let eq35_e1556_d_n0: f64 = (p.p32 * s.dn[80][0]);
        let eq35_e1556_d_n1: f64 = (p.p32 * s.dn[80][1]);
        let eq35_e1556_d_n2: f64 = (p.p32 * s.dn[80][2]);
        let eq35_e1556_d_n3: f64 = (p.p32 * s.dn[80][3]);
        let eq35_e1556_d_n4: f64 = (p.p32 * s.dn[80][4]);
        let eq35_e1556_d_n5: f64 = (p.p32 * s.dn[80][5]);
        let eq35_e1556_d_n6: f64 = (p.p32 * s.dn[80][6]);
        let eq35_e1556_d_n7: f64 = (p.p32 * s.dn[80][7]);
        let eq35_e1556_d_n8: f64 = (p.p32 * s.dn[80][8]);
        let eq35_e1556_d_n9: f64 = (p.p32 * s.dn[80][9]);
        let eq35_e1556_d_n10: f64 = (p.p32 * s.dn[80][10]);
        let eq35_e1556_d_n11: f64 = (p.p32 * s.dn[80][11]);
        let eq35_e1556_d_n12: f64 = (p.p32 * s.dn[80][12]);
        let eq35_e1556_d_n13: f64 = (p.p32 * s.dn[80][13]);
        let eq35_e1556_d_b0: f64 = (p.p32 * s.db[80][0]);
        let eq35_e1556_d_b1: f64 = (p.p32 * s.db[80][1]);
        let eq35_e1556_d_b2: f64 = (p.p32 * s.db[80][2]);
        let eq35_e1556_d_b3: f64 = (p.p32 * s.db[80][3]);
        let eq35_e1556_d_b4: f64 = (p.p32 * s.db[80][4]);
        let eq35_e1556_d_b5: f64 = (p.p32 * s.db[80][5]);
        let eq35_e1556_d_b6: f64 = (p.p32 * s.db[80][6]);
        let eq35_e1556_d_b7: f64 = (p.p32 * s.db[80][7]);
        let eq35_e1556_d_b8: f64 = (p.p32 * s.db[80][8]);
        let eq35_e1556_d_b9: f64 = (p.p32 * s.db[80][9]);
        let eq35_e1556_d_b10: f64 = (p.p32 * s.db[80][10]);
        let eq35_e1556_d_b11: f64 = (p.p32 * s.db[80][11]);
        let eq35_e1556_d_b12: f64 = (p.p32 * s.db[80][12]);
        let eq35_e1556_d_b13: f64 = (p.p32 * s.db[80][13]);
        let eq35_e1556_d_b14: f64 = (p.p32 * s.db[80][14]);
        let eq35_e1556_d_b15: f64 = (p.p32 * s.db[80][15]);
        let eq35_e1556_d_b16: f64 = (p.p32 * s.db[80][16]);
        let eq35_e1556_d_b17: f64 = (p.p32 * s.db[80][17]);
        let eq35_value: f64 = eq35_e1556;
        let eq35_node_derivatives: [f64; 14] = [eq35_e1556_d_n0, eq35_e1556_d_n1, eq35_e1556_d_n2, eq35_e1556_d_n3, eq35_e1556_d_n4, eq35_e1556_d_n5, eq35_e1556_d_n6, eq35_e1556_d_n7, eq35_e1556_d_n8, eq35_e1556_d_n9, eq35_e1556_d_n10, eq35_e1556_d_n11, eq35_e1556_d_n12, eq35_e1556_d_n13];
        let eq35_branch_derivatives: [f64; 18] = [eq35_e1556_d_b0, eq35_e1556_d_b1, eq35_e1556_d_b2, eq35_e1556_d_b3, eq35_e1556_d_b4, eq35_e1556_d_b5, eq35_e1556_d_b6, eq35_e1556_d_b7, eq35_e1556_d_b8, eq35_e1556_d_b9, eq35_e1556_d_b10, eq35_e1556_d_b11, eq35_e1556_d_b12, eq35_e1556_d_b13, eq35_e1556_d_b14, eq35_e1556_d_b15, eq35_e1556_d_b16, eq35_e1556_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(4),
            multiplicity * (eq35_value),
            &eq35_node_derivatives,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let (eq37_e1569, eq37_e1569_d_n0, eq37_e1569_d_n1, eq37_e1569_d_n2, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12, eq37_e1569_d_n13, eq37_e1569_d_b0, eq37_e1569_d_b1, eq37_e1569_d_b2, eq37_e1569_d_b3, eq37_e1569_d_b4, eq37_e1569_d_b5, eq37_e1569_d_b6, eq37_e1569_d_b7, eq37_e1569_d_b8, eq37_e1569_d_b9, eq37_e1569_d_b10, eq37_e1569_d_b11, eq37_e1569_d_b12, eq37_e1569_d_b13, eq37_e1569_d_b14, eq37_e1569_d_b15, eq37_e1569_d_b16, eq37_e1569_d_b17,) = {
    if (!s.b[1552]) {
        let eq37_e1565: f64 = (p.p37 * p.p32);
        let eq37_e1567: f64 = (eq37_e1565 * s.v[907]);
        let eq37_e1567_d_n0: f64 = (eq37_e1565 * s.dn[907][0]);
        let eq37_e1567_d_n1: f64 = (eq37_e1565 * s.dn[907][1]);
        let eq37_e1567_d_n2: f64 = (eq37_e1565 * s.dn[907][2]);
        let eq37_e1567_d_n3: f64 = (eq37_e1565 * s.dn[907][3]);
        let eq37_e1567_d_n4: f64 = (eq37_e1565 * s.dn[907][4]);
        let eq37_e1567_d_n5: f64 = (eq37_e1565 * s.dn[907][5]);
        let eq37_e1567_d_n6: f64 = (eq37_e1565 * s.dn[907][6]);
        let eq37_e1567_d_n7: f64 = (eq37_e1565 * s.dn[907][7]);
        let eq37_e1567_d_n8: f64 = (eq37_e1565 * s.dn[907][8]);
        let eq37_e1567_d_n9: f64 = (eq37_e1565 * s.dn[907][9]);
        let eq37_e1567_d_n10: f64 = (eq37_e1565 * s.dn[907][10]);
        let eq37_e1567_d_n11: f64 = (eq37_e1565 * s.dn[907][11]);
        let eq37_e1567_d_n12: f64 = (eq37_e1565 * s.dn[907][12]);
        let eq37_e1567_d_n13: f64 = (eq37_e1565 * s.dn[907][13]);
        let eq37_e1567_d_b0: f64 = (eq37_e1565 * s.db[907][0]);
        let eq37_e1567_d_b1: f64 = (eq37_e1565 * s.db[907][1]);
        let eq37_e1567_d_b2: f64 = (eq37_e1565 * s.db[907][2]);
        let eq37_e1567_d_b3: f64 = (eq37_e1565 * s.db[907][3]);
        let eq37_e1567_d_b4: f64 = (eq37_e1565 * s.db[907][4]);
        let eq37_e1567_d_b5: f64 = (eq37_e1565 * s.db[907][5]);
        let eq37_e1567_d_b6: f64 = (eq37_e1565 * s.db[907][6]);
        let eq37_e1567_d_b7: f64 = (eq37_e1565 * s.db[907][7]);
        let eq37_e1567_d_b8: f64 = (eq37_e1565 * s.db[907][8]);
        let eq37_e1567_d_b9: f64 = (eq37_e1565 * s.db[907][9]);
        let eq37_e1567_d_b10: f64 = (eq37_e1565 * s.db[907][10]);
        let eq37_e1567_d_b11: f64 = (eq37_e1565 * s.db[907][11]);
        let eq37_e1567_d_b12: f64 = (eq37_e1565 * s.db[907][12]);
        let eq37_e1567_d_b13: f64 = (eq37_e1565 * s.db[907][13]);
        let eq37_e1567_d_b14: f64 = (eq37_e1565 * s.db[907][14]);
        let eq37_e1567_d_b15: f64 = (eq37_e1565 * s.db[907][15]);
        let eq37_e1567_d_b16: f64 = (eq37_e1565 * s.db[907][16]);
        let eq37_e1567_d_b17: f64 = (eq37_e1565 * s.db[907][17]);
        (eq37_e1567, eq37_e1567_d_n0, eq37_e1567_d_n1, eq37_e1567_d_n2, eq37_e1567_d_n3, eq37_e1567_d_n4, eq37_e1567_d_n5, eq37_e1567_d_n6, eq37_e1567_d_n7, eq37_e1567_d_n8, eq37_e1567_d_n9, eq37_e1567_d_n10, eq37_e1567_d_n11, eq37_e1567_d_n12, eq37_e1567_d_n13, eq37_e1567_d_b0, eq37_e1567_d_b1, eq37_e1567_d_b2, eq37_e1567_d_b3, eq37_e1567_d_b4, eq37_e1567_d_b5, eq37_e1567_d_b6, eq37_e1567_d_b7, eq37_e1567_d_b8, eq37_e1567_d_b9, eq37_e1567_d_b10, eq37_e1567_d_b11, eq37_e1567_d_b12, eq37_e1567_d_b13, eq37_e1567_d_b14, eq37_e1567_d_b15, eq37_e1567_d_b16, eq37_e1567_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e1569;
        let eq37_node_derivatives: [f64; 14] = [eq37_e1569_d_n0, eq37_e1569_d_n1, eq37_e1569_d_n2, eq37_e1569_d_n3, eq37_e1569_d_n4, eq37_e1569_d_n5, eq37_e1569_d_n6, eq37_e1569_d_n7, eq37_e1569_d_n8, eq37_e1569_d_n9, eq37_e1569_d_n10, eq37_e1569_d_n11, eq37_e1569_d_n12, eq37_e1569_d_n13];
        let eq37_branch_derivatives: [f64; 18] = [eq37_e1569_d_b0, eq37_e1569_d_b1, eq37_e1569_d_b2, eq37_e1569_d_b3, eq37_e1569_d_b4, eq37_e1569_d_b5, eq37_e1569_d_b6, eq37_e1569_d_b7, eq37_e1569_d_b8, eq37_e1569_d_b9, eq37_e1569_d_b10, eq37_e1569_d_b11, eq37_e1569_d_b12, eq37_e1569_d_b13, eq37_e1569_d_b14, eq37_e1569_d_b15, eq37_e1569_d_b16, eq37_e1569_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq37_value),
            &eq37_node_derivatives,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq44_e1647: f64 = (p.p33 * s.v[92]);
        let eq44_e1647_d_n0: f64 = (p.p33 * s.dn[92][0]);
        let eq44_e1647_d_n1: f64 = (p.p33 * s.dn[92][1]);
        let eq44_e1647_d_n2: f64 = (p.p33 * s.dn[92][2]);
        let eq44_e1647_d_n3: f64 = (p.p33 * s.dn[92][3]);
        let eq44_e1647_d_n4: f64 = (p.p33 * s.dn[92][4]);
        let eq44_e1647_d_n5: f64 = (p.p33 * s.dn[92][5]);
        let eq44_e1647_d_n6: f64 = (p.p33 * s.dn[92][6]);
        let eq44_e1647_d_n7: f64 = (p.p33 * s.dn[92][7]);
        let eq44_e1647_d_n8: f64 = (p.p33 * s.dn[92][8]);
        let eq44_e1647_d_n9: f64 = (p.p33 * s.dn[92][9]);
        let eq44_e1647_d_n10: f64 = (p.p33 * s.dn[92][10]);
        let eq44_e1647_d_n11: f64 = (p.p33 * s.dn[92][11]);
        let eq44_e1647_d_n12: f64 = (p.p33 * s.dn[92][12]);
        let eq44_e1647_d_n13: f64 = (p.p33 * s.dn[92][13]);
        let eq44_e1647_d_b0: f64 = (p.p33 * s.db[92][0]);
        let eq44_e1647_d_b1: f64 = (p.p33 * s.db[92][1]);
        let eq44_e1647_d_b2: f64 = (p.p33 * s.db[92][2]);
        let eq44_e1647_d_b3: f64 = (p.p33 * s.db[92][3]);
        let eq44_e1647_d_b4: f64 = (p.p33 * s.db[92][4]);
        let eq44_e1647_d_b5: f64 = (p.p33 * s.db[92][5]);
        let eq44_e1647_d_b6: f64 = (p.p33 * s.db[92][6]);
        let eq44_e1647_d_b7: f64 = (p.p33 * s.db[92][7]);
        let eq44_e1647_d_b8: f64 = (p.p33 * s.db[92][8]);
        let eq44_e1647_d_b9: f64 = (p.p33 * s.db[92][9]);
        let eq44_e1647_d_b10: f64 = (p.p33 * s.db[92][10]);
        let eq44_e1647_d_b11: f64 = (p.p33 * s.db[92][11]);
        let eq44_e1647_d_b12: f64 = (p.p33 * s.db[92][12]);
        let eq44_e1647_d_b13: f64 = (p.p33 * s.db[92][13]);
        let eq44_e1647_d_b14: f64 = (p.p33 * s.db[92][14]);
        let eq44_e1647_d_b15: f64 = (p.p33 * s.db[92][15]);
        let eq44_e1647_d_b16: f64 = (p.p33 * s.db[92][16]);
        let eq44_e1647_d_b17: f64 = (p.p33 * s.db[92][17]);
        let eq44_e1648: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq44_e1647);
        let eq44_value: f64 = eq44_e1648;
        let eq44_node_derivatives: [f64; 14] = [(eq44_e1647_d_n0 * ddt_scale), (eq44_e1647_d_n1 * ddt_scale), (eq44_e1647_d_n2 * ddt_scale), (eq44_e1647_d_n3 * ddt_scale), (eq44_e1647_d_n4 * ddt_scale), (eq44_e1647_d_n5 * ddt_scale), (eq44_e1647_d_n6 * ddt_scale), (eq44_e1647_d_n7 * ddt_scale), (eq44_e1647_d_n8 * ddt_scale), (eq44_e1647_d_n9 * ddt_scale), (eq44_e1647_d_n10 * ddt_scale), (eq44_e1647_d_n11 * ddt_scale), (eq44_e1647_d_n12 * ddt_scale), (eq44_e1647_d_n13 * ddt_scale)];
        let eq44_branch_derivatives: [f64; 18] = [(eq44_e1647_d_b0 * ddt_scale), (eq44_e1647_d_b1 * ddt_scale), (eq44_e1647_d_b2 * ddt_scale), (eq44_e1647_d_b3 * ddt_scale), (eq44_e1647_d_b4 * ddt_scale), (eq44_e1647_d_b5 * ddt_scale), (eq44_e1647_d_b6 * ddt_scale), (eq44_e1647_d_b7 * ddt_scale), (eq44_e1647_d_b8 * ddt_scale), (eq44_e1647_d_b9 * ddt_scale), (eq44_e1647_d_b10 * ddt_scale), (eq44_e1647_d_b11 * ddt_scale), (eq44_e1647_d_b12 * ddt_scale), (eq44_e1647_d_b13 * ddt_scale), (eq44_e1647_d_b14 * ddt_scale), (eq44_e1647_d_b15 * ddt_scale), (eq44_e1647_d_b16 * ddt_scale), (eq44_e1647_d_b17 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let eq45_e1651: f64 = (p.p33 * s.v[93]);
        let eq45_e1651_d_n0: f64 = (p.p33 * s.dn[93][0]);
        let eq45_e1651_d_n1: f64 = (p.p33 * s.dn[93][1]);
        let eq45_e1651_d_n2: f64 = (p.p33 * s.dn[93][2]);
        let eq45_e1651_d_n3: f64 = (p.p33 * s.dn[93][3]);
        let eq45_e1651_d_n4: f64 = (p.p33 * s.dn[93][4]);
        let eq45_e1651_d_n5: f64 = (p.p33 * s.dn[93][5]);
        let eq45_e1651_d_n6: f64 = (p.p33 * s.dn[93][6]);
        let eq45_e1651_d_n7: f64 = (p.p33 * s.dn[93][7]);
        let eq45_e1651_d_n8: f64 = (p.p33 * s.dn[93][8]);
        let eq45_e1651_d_n9: f64 = (p.p33 * s.dn[93][9]);
        let eq45_e1651_d_n10: f64 = (p.p33 * s.dn[93][10]);
        let eq45_e1651_d_n11: f64 = (p.p33 * s.dn[93][11]);
        let eq45_e1651_d_n12: f64 = (p.p33 * s.dn[93][12]);
        let eq45_e1651_d_n13: f64 = (p.p33 * s.dn[93][13]);
        let eq45_e1651_d_b0: f64 = (p.p33 * s.db[93][0]);
        let eq45_e1651_d_b1: f64 = (p.p33 * s.db[93][1]);
        let eq45_e1651_d_b2: f64 = (p.p33 * s.db[93][2]);
        let eq45_e1651_d_b3: f64 = (p.p33 * s.db[93][3]);
        let eq45_e1651_d_b4: f64 = (p.p33 * s.db[93][4]);
        let eq45_e1651_d_b5: f64 = (p.p33 * s.db[93][5]);
        let eq45_e1651_d_b6: f64 = (p.p33 * s.db[93][6]);
        let eq45_e1651_d_b7: f64 = (p.p33 * s.db[93][7]);
        let eq45_e1651_d_b8: f64 = (p.p33 * s.db[93][8]);
        let eq45_e1651_d_b9: f64 = (p.p33 * s.db[93][9]);
        let eq45_e1651_d_b10: f64 = (p.p33 * s.db[93][10]);
        let eq45_e1651_d_b11: f64 = (p.p33 * s.db[93][11]);
        let eq45_e1651_d_b12: f64 = (p.p33 * s.db[93][12]);
        let eq45_e1651_d_b13: f64 = (p.p33 * s.db[93][13]);
        let eq45_e1651_d_b14: f64 = (p.p33 * s.db[93][14]);
        let eq45_e1651_d_b15: f64 = (p.p33 * s.db[93][15]);
        let eq45_e1651_d_b16: f64 = (p.p33 * s.db[93][16]);
        let eq45_e1651_d_b17: f64 = (p.p33 * s.db[93][17]);
        let eq45_e1652: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq45_e1651);
        let eq45_value: f64 = eq45_e1652;
        let eq45_node_derivatives: [f64; 14] = [(eq45_e1651_d_n0 * ddt_scale), (eq45_e1651_d_n1 * ddt_scale), (eq45_e1651_d_n2 * ddt_scale), (eq45_e1651_d_n3 * ddt_scale), (eq45_e1651_d_n4 * ddt_scale), (eq45_e1651_d_n5 * ddt_scale), (eq45_e1651_d_n6 * ddt_scale), (eq45_e1651_d_n7 * ddt_scale), (eq45_e1651_d_n8 * ddt_scale), (eq45_e1651_d_n9 * ddt_scale), (eq45_e1651_d_n10 * ddt_scale), (eq45_e1651_d_n11 * ddt_scale), (eq45_e1651_d_n12 * ddt_scale), (eq45_e1651_d_n13 * ddt_scale)];
        let eq45_branch_derivatives: [f64; 18] = [(eq45_e1651_d_b0 * ddt_scale), (eq45_e1651_d_b1 * ddt_scale), (eq45_e1651_d_b2 * ddt_scale), (eq45_e1651_d_b3 * ddt_scale), (eq45_e1651_d_b4 * ddt_scale), (eq45_e1651_d_b5 * ddt_scale), (eq45_e1651_d_b6 * ddt_scale), (eq45_e1651_d_b7 * ddt_scale), (eq45_e1651_d_b8 * ddt_scale), (eq45_e1651_d_b9 * ddt_scale), (eq45_e1651_d_b10 * ddt_scale), (eq45_e1651_d_b11 * ddt_scale), (eq45_e1651_d_b12 * ddt_scale), (eq45_e1651_d_b13 * ddt_scale), (eq45_e1651_d_b14 * ddt_scale), (eq45_e1651_d_b15 * ddt_scale), (eq45_e1651_d_b16 * ddt_scale), (eq45_e1651_d_b17 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(5),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq46_e1656: f64 = (p.p33 * s.v[916]);
        let eq46_e1656_d_n0: f64 = (p.p33 * s.dn[916][0]);
        let eq46_e1656_d_n1: f64 = (p.p33 * s.dn[916][1]);
        let eq46_e1656_d_n2: f64 = (p.p33 * s.dn[916][2]);
        let eq46_e1656_d_n3: f64 = (p.p33 * s.dn[916][3]);
        let eq46_e1656_d_n4: f64 = (p.p33 * s.dn[916][4]);
        let eq46_e1656_d_n5: f64 = (p.p33 * s.dn[916][5]);
        let eq46_e1656_d_n6: f64 = (p.p33 * s.dn[916][6]);
        let eq46_e1656_d_n7: f64 = (p.p33 * s.dn[916][7]);
        let eq46_e1656_d_n8: f64 = (p.p33 * s.dn[916][8]);
        let eq46_e1656_d_n9: f64 = (p.p33 * s.dn[916][9]);
        let eq46_e1656_d_n10: f64 = (p.p33 * s.dn[916][10]);
        let eq46_e1656_d_n11: f64 = (p.p33 * s.dn[916][11]);
        let eq46_e1656_d_n12: f64 = (p.p33 * s.dn[916][12]);
        let eq46_e1656_d_n13: f64 = (p.p33 * s.dn[916][13]);
        let eq46_e1656_d_b0: f64 = (p.p33 * s.db[916][0]);
        let eq46_e1656_d_b1: f64 = (p.p33 * s.db[916][1]);
        let eq46_e1656_d_b2: f64 = (p.p33 * s.db[916][2]);
        let eq46_e1656_d_b3: f64 = (p.p33 * s.db[916][3]);
        let eq46_e1656_d_b4: f64 = (p.p33 * s.db[916][4]);
        let eq46_e1656_d_b5: f64 = (p.p33 * s.db[916][5]);
        let eq46_e1656_d_b6: f64 = (p.p33 * s.db[916][6]);
        let eq46_e1656_d_b7: f64 = (p.p33 * s.db[916][7]);
        let eq46_e1656_d_b8: f64 = (p.p33 * s.db[916][8]);
        let eq46_e1656_d_b9: f64 = (p.p33 * s.db[916][9]);
        let eq46_e1656_d_b10: f64 = (p.p33 * s.db[916][10]);
        let eq46_e1656_d_b11: f64 = (p.p33 * s.db[916][11]);
        let eq46_e1656_d_b12: f64 = (p.p33 * s.db[916][12]);
        let eq46_e1656_d_b13: f64 = (p.p33 * s.db[916][13]);
        let eq46_e1656_d_b14: f64 = (p.p33 * s.db[916][14]);
        let eq46_e1656_d_b15: f64 = (p.p33 * s.db[916][15]);
        let eq46_e1656_d_b16: f64 = (p.p33 * s.db[916][16]);
        let eq46_e1656_d_b17: f64 = (p.p33 * s.db[916][17]);
        let eq46_e1657: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq46_e1656);
        let eq46_e1658: f64 = (p.p37 * eq46_e1657);
        let eq46_e1658_d_n0: f64 = (p.p37 * (eq46_e1656_d_n0 * ddt_scale));
        let eq46_e1658_d_n1: f64 = (p.p37 * (eq46_e1656_d_n1 * ddt_scale));
        let eq46_e1658_d_n2: f64 = (p.p37 * (eq46_e1656_d_n2 * ddt_scale));
        let eq46_e1658_d_n3: f64 = (p.p37 * (eq46_e1656_d_n3 * ddt_scale));
        let eq46_e1658_d_n4: f64 = (p.p37 * (eq46_e1656_d_n4 * ddt_scale));
        let eq46_e1658_d_n5: f64 = (p.p37 * (eq46_e1656_d_n5 * ddt_scale));
        let eq46_e1658_d_n6: f64 = (p.p37 * (eq46_e1656_d_n6 * ddt_scale));
        let eq46_e1658_d_n7: f64 = (p.p37 * (eq46_e1656_d_n7 * ddt_scale));
        let eq46_e1658_d_n8: f64 = (p.p37 * (eq46_e1656_d_n8 * ddt_scale));
        let eq46_e1658_d_n9: f64 = (p.p37 * (eq46_e1656_d_n9 * ddt_scale));
        let eq46_e1658_d_n10: f64 = (p.p37 * (eq46_e1656_d_n10 * ddt_scale));
        let eq46_e1658_d_n11: f64 = (p.p37 * (eq46_e1656_d_n11 * ddt_scale));
        let eq46_e1658_d_n12: f64 = (p.p37 * (eq46_e1656_d_n12 * ddt_scale));
        let eq46_e1658_d_n13: f64 = (p.p37 * (eq46_e1656_d_n13 * ddt_scale));
        let eq46_e1658_d_b0: f64 = (p.p37 * (eq46_e1656_d_b0 * ddt_scale));
        let eq46_e1658_d_b1: f64 = (p.p37 * (eq46_e1656_d_b1 * ddt_scale));
        let eq46_e1658_d_b2: f64 = (p.p37 * (eq46_e1656_d_b2 * ddt_scale));
        let eq46_e1658_d_b3: f64 = (p.p37 * (eq46_e1656_d_b3 * ddt_scale));
        let eq46_e1658_d_b4: f64 = (p.p37 * (eq46_e1656_d_b4 * ddt_scale));
        let eq46_e1658_d_b5: f64 = (p.p37 * (eq46_e1656_d_b5 * ddt_scale));
        let eq46_e1658_d_b6: f64 = (p.p37 * (eq46_e1656_d_b6 * ddt_scale));
        let eq46_e1658_d_b7: f64 = (p.p37 * (eq46_e1656_d_b7 * ddt_scale));
        let eq46_e1658_d_b8: f64 = (p.p37 * (eq46_e1656_d_b8 * ddt_scale));
        let eq46_e1658_d_b9: f64 = (p.p37 * (eq46_e1656_d_b9 * ddt_scale));
        let eq46_e1658_d_b10: f64 = (p.p37 * (eq46_e1656_d_b10 * ddt_scale));
        let eq46_e1658_d_b11: f64 = (p.p37 * (eq46_e1656_d_b11 * ddt_scale));
        let eq46_e1658_d_b12: f64 = (p.p37 * (eq46_e1656_d_b12 * ddt_scale));
        let eq46_e1658_d_b13: f64 = (p.p37 * (eq46_e1656_d_b13 * ddt_scale));
        let eq46_e1658_d_b14: f64 = (p.p37 * (eq46_e1656_d_b14 * ddt_scale));
        let eq46_e1658_d_b15: f64 = (p.p37 * (eq46_e1656_d_b15 * ddt_scale));
        let eq46_e1658_d_b16: f64 = (p.p37 * (eq46_e1656_d_b16 * ddt_scale));
        let eq46_e1658_d_b17: f64 = (p.p37 * (eq46_e1656_d_b17 * ddt_scale));
        let eq46_value: f64 = eq46_e1658;
        let eq46_node_derivatives: [f64; 14] = [eq46_e1658_d_n0, eq46_e1658_d_n1, eq46_e1658_d_n2, eq46_e1658_d_n3, eq46_e1658_d_n4, eq46_e1658_d_n5, eq46_e1658_d_n6, eq46_e1658_d_n7, eq46_e1658_d_n8, eq46_e1658_d_n9, eq46_e1658_d_n10, eq46_e1658_d_n11, eq46_e1658_d_n12, eq46_e1658_d_n13];
        let eq46_branch_derivatives: [f64; 18] = [eq46_e1658_d_b0, eq46_e1658_d_b1, eq46_e1658_d_b2, eq46_e1658_d_b3, eq46_e1658_d_b4, eq46_e1658_d_b5, eq46_e1658_d_b6, eq46_e1658_d_b7, eq46_e1658_d_b8, eq46_e1658_d_b9, eq46_e1658_d_b10, eq46_e1658_d_b11, eq46_e1658_d_b12, eq46_e1658_d_b13, eq46_e1658_d_b14, eq46_e1658_d_b15, eq46_e1658_d_b16, eq46_e1658_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let eq47_e1662: f64 = (p.p33 * s.v[920]);
        let eq47_e1662_d_n0: f64 = (p.p33 * s.dn[920][0]);
        let eq47_e1662_d_n1: f64 = (p.p33 * s.dn[920][1]);
        let eq47_e1662_d_n2: f64 = (p.p33 * s.dn[920][2]);
        let eq47_e1662_d_n3: f64 = (p.p33 * s.dn[920][3]);
        let eq47_e1662_d_n4: f64 = (p.p33 * s.dn[920][4]);
        let eq47_e1662_d_n5: f64 = (p.p33 * s.dn[920][5]);
        let eq47_e1662_d_n6: f64 = (p.p33 * s.dn[920][6]);
        let eq47_e1662_d_n7: f64 = (p.p33 * s.dn[920][7]);
        let eq47_e1662_d_n8: f64 = (p.p33 * s.dn[920][8]);
        let eq47_e1662_d_n9: f64 = (p.p33 * s.dn[920][9]);
        let eq47_e1662_d_n10: f64 = (p.p33 * s.dn[920][10]);
        let eq47_e1662_d_n11: f64 = (p.p33 * s.dn[920][11]);
        let eq47_e1662_d_n12: f64 = (p.p33 * s.dn[920][12]);
        let eq47_e1662_d_n13: f64 = (p.p33 * s.dn[920][13]);
        let eq47_e1662_d_b0: f64 = (p.p33 * s.db[920][0]);
        let eq47_e1662_d_b1: f64 = (p.p33 * s.db[920][1]);
        let eq47_e1662_d_b2: f64 = (p.p33 * s.db[920][2]);
        let eq47_e1662_d_b3: f64 = (p.p33 * s.db[920][3]);
        let eq47_e1662_d_b4: f64 = (p.p33 * s.db[920][4]);
        let eq47_e1662_d_b5: f64 = (p.p33 * s.db[920][5]);
        let eq47_e1662_d_b6: f64 = (p.p33 * s.db[920][6]);
        let eq47_e1662_d_b7: f64 = (p.p33 * s.db[920][7]);
        let eq47_e1662_d_b8: f64 = (p.p33 * s.db[920][8]);
        let eq47_e1662_d_b9: f64 = (p.p33 * s.db[920][9]);
        let eq47_e1662_d_b10: f64 = (p.p33 * s.db[920][10]);
        let eq47_e1662_d_b11: f64 = (p.p33 * s.db[920][11]);
        let eq47_e1662_d_b12: f64 = (p.p33 * s.db[920][12]);
        let eq47_e1662_d_b13: f64 = (p.p33 * s.db[920][13]);
        let eq47_e1662_d_b14: f64 = (p.p33 * s.db[920][14]);
        let eq47_e1662_d_b15: f64 = (p.p33 * s.db[920][15]);
        let eq47_e1662_d_b16: f64 = (p.p33 * s.db[920][16]);
        let eq47_e1662_d_b17: f64 = (p.p33 * s.db[920][17]);
        let eq47_e1663: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq47_e1662);
        let eq47_e1664: f64 = (p.p37 * eq47_e1663);
        let eq47_e1664_d_n0: f64 = (p.p37 * (eq47_e1662_d_n0 * ddt_scale));
        let eq47_e1664_d_n1: f64 = (p.p37 * (eq47_e1662_d_n1 * ddt_scale));
        let eq47_e1664_d_n2: f64 = (p.p37 * (eq47_e1662_d_n2 * ddt_scale));
        let eq47_e1664_d_n3: f64 = (p.p37 * (eq47_e1662_d_n3 * ddt_scale));
        let eq47_e1664_d_n4: f64 = (p.p37 * (eq47_e1662_d_n4 * ddt_scale));
        let eq47_e1664_d_n5: f64 = (p.p37 * (eq47_e1662_d_n5 * ddt_scale));
        let eq47_e1664_d_n6: f64 = (p.p37 * (eq47_e1662_d_n6 * ddt_scale));
        let eq47_e1664_d_n7: f64 = (p.p37 * (eq47_e1662_d_n7 * ddt_scale));
        let eq47_e1664_d_n8: f64 = (p.p37 * (eq47_e1662_d_n8 * ddt_scale));
        let eq47_e1664_d_n9: f64 = (p.p37 * (eq47_e1662_d_n9 * ddt_scale));
        let eq47_e1664_d_n10: f64 = (p.p37 * (eq47_e1662_d_n10 * ddt_scale));
        let eq47_e1664_d_n11: f64 = (p.p37 * (eq47_e1662_d_n11 * ddt_scale));
        let eq47_e1664_d_n12: f64 = (p.p37 * (eq47_e1662_d_n12 * ddt_scale));
        let eq47_e1664_d_n13: f64 = (p.p37 * (eq47_e1662_d_n13 * ddt_scale));
        let eq47_e1664_d_b0: f64 = (p.p37 * (eq47_e1662_d_b0 * ddt_scale));
        let eq47_e1664_d_b1: f64 = (p.p37 * (eq47_e1662_d_b1 * ddt_scale));
        let eq47_e1664_d_b2: f64 = (p.p37 * (eq47_e1662_d_b2 * ddt_scale));
        let eq47_e1664_d_b3: f64 = (p.p37 * (eq47_e1662_d_b3 * ddt_scale));
        let eq47_e1664_d_b4: f64 = (p.p37 * (eq47_e1662_d_b4 * ddt_scale));
        let eq47_e1664_d_b5: f64 = (p.p37 * (eq47_e1662_d_b5 * ddt_scale));
        let eq47_e1664_d_b6: f64 = (p.p37 * (eq47_e1662_d_b6 * ddt_scale));
        let eq47_e1664_d_b7: f64 = (p.p37 * (eq47_e1662_d_b7 * ddt_scale));
        let eq47_e1664_d_b8: f64 = (p.p37 * (eq47_e1662_d_b8 * ddt_scale));
        let eq47_e1664_d_b9: f64 = (p.p37 * (eq47_e1662_d_b9 * ddt_scale));
        let eq47_e1664_d_b10: f64 = (p.p37 * (eq47_e1662_d_b10 * ddt_scale));
        let eq47_e1664_d_b11: f64 = (p.p37 * (eq47_e1662_d_b11 * ddt_scale));
        let eq47_e1664_d_b12: f64 = (p.p37 * (eq47_e1662_d_b12 * ddt_scale));
        let eq47_e1664_d_b13: f64 = (p.p37 * (eq47_e1662_d_b13 * ddt_scale));
        let eq47_e1664_d_b14: f64 = (p.p37 * (eq47_e1662_d_b14 * ddt_scale));
        let eq47_e1664_d_b15: f64 = (p.p37 * (eq47_e1662_d_b15 * ddt_scale));
        let eq47_e1664_d_b16: f64 = (p.p37 * (eq47_e1662_d_b16 * ddt_scale));
        let eq47_e1664_d_b17: f64 = (p.p37 * (eq47_e1662_d_b17 * ddt_scale));
        let eq47_value: f64 = eq47_e1664;
        let eq47_node_derivatives: [f64; 14] = [eq47_e1664_d_n0, eq47_e1664_d_n1, eq47_e1664_d_n2, eq47_e1664_d_n3, eq47_e1664_d_n4, eq47_e1664_d_n5, eq47_e1664_d_n6, eq47_e1664_d_n7, eq47_e1664_d_n8, eq47_e1664_d_n9, eq47_e1664_d_n10, eq47_e1664_d_n11, eq47_e1664_d_n12, eq47_e1664_d_n13];
        let eq47_branch_derivatives: [f64; 18] = [eq47_e1664_d_b0, eq47_e1664_d_b1, eq47_e1664_d_b2, eq47_e1664_d_b3, eq47_e1664_d_b4, eq47_e1664_d_b5, eq47_e1664_d_b6, eq47_e1664_d_b7, eq47_e1664_d_b8, eq47_e1664_d_b9, eq47_e1664_d_b10, eq47_e1664_d_b11, eq47_e1664_d_b12, eq47_e1664_d_b13, eq47_e1664_d_b14, eq47_e1664_d_b15, eq47_e1664_d_b16, eq47_e1664_d_b17];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let eq48_e1668: f64 = (p.p33 * s.v[909]);
        let eq48_e1668_d_n0: f64 = (p.p33 * s.dn[909][0]);
        let eq48_e1668_d_n1: f64 = (p.p33 * s.dn[909][1]);
        let eq48_e1668_d_n2: f64 = (p.p33 * s.dn[909][2]);
        let eq48_e1668_d_n3: f64 = (p.p33 * s.dn[909][3]);
        let eq48_e1668_d_n4: f64 = (p.p33 * s.dn[909][4]);
        let eq48_e1668_d_n5: f64 = (p.p33 * s.dn[909][5]);
        let eq48_e1668_d_n6: f64 = (p.p33 * s.dn[909][6]);
        let eq48_e1668_d_n7: f64 = (p.p33 * s.dn[909][7]);
        let eq48_e1668_d_n8: f64 = (p.p33 * s.dn[909][8]);
        let eq48_e1668_d_n9: f64 = (p.p33 * s.dn[909][9]);
        let eq48_e1668_d_n10: f64 = (p.p33 * s.dn[909][10]);
        let eq48_e1668_d_n11: f64 = (p.p33 * s.dn[909][11]);
        let eq48_e1668_d_n12: f64 = (p.p33 * s.dn[909][12]);
        let eq48_e1668_d_n13: f64 = (p.p33 * s.dn[909][13]);
        let eq48_e1668_d_b0: f64 = (p.p33 * s.db[909][0]);
        let eq48_e1668_d_b1: f64 = (p.p33 * s.db[909][1]);
        let eq48_e1668_d_b2: f64 = (p.p33 * s.db[909][2]);
        let eq48_e1668_d_b3: f64 = (p.p33 * s.db[909][3]);
        let eq48_e1668_d_b4: f64 = (p.p33 * s.db[909][4]);
        let eq48_e1668_d_b5: f64 = (p.p33 * s.db[909][5]);
        let eq48_e1668_d_b6: f64 = (p.p33 * s.db[909][6]);
        let eq48_e1668_d_b7: f64 = (p.p33 * s.db[909][7]);
        let eq48_e1668_d_b8: f64 = (p.p33 * s.db[909][8]);
        let eq48_e1668_d_b9: f64 = (p.p33 * s.db[909][9]);
        let eq48_e1668_d_b10: f64 = (p.p33 * s.db[909][10]);
        let eq48_e1668_d_b11: f64 = (p.p33 * s.db[909][11]);
        let eq48_e1668_d_b12: f64 = (p.p33 * s.db[909][12]);
        let eq48_e1668_d_b13: f64 = (p.p33 * s.db[909][13]);
        let eq48_e1668_d_b14: f64 = (p.p33 * s.db[909][14]);
        let eq48_e1668_d_b15: f64 = (p.p33 * s.db[909][15]);
        let eq48_e1668_d_b16: f64 = (p.p33 * s.db[909][16]);
        let eq48_e1668_d_b17: f64 = (p.p33 * s.db[909][17]);
        let eq48_e1669: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq48_e1668);
        let eq48_e1670: f64 = (p.p37 * eq48_e1669);
        let eq48_e1670_d_n0: f64 = (p.p37 * (eq48_e1668_d_n0 * ddt_scale));
        let eq48_e1670_d_n1: f64 = (p.p37 * (eq48_e1668_d_n1 * ddt_scale));
        let eq48_e1670_d_n2: f64 = (p.p37 * (eq48_e1668_d_n2 * ddt_scale));
        let eq48_e1670_d_n3: f64 = (p.p37 * (eq48_e1668_d_n3 * ddt_scale));
        let eq48_e1670_d_n4: f64 = (p.p37 * (eq48_e1668_d_n4 * ddt_scale));
        let eq48_e1670_d_n5: f64 = (p.p37 * (eq48_e1668_d_n5 * ddt_scale));
        let eq48_e1670_d_n6: f64 = (p.p37 * (eq48_e1668_d_n6 * ddt_scale));
        let eq48_e1670_d_n7: f64 = (p.p37 * (eq48_e1668_d_n7 * ddt_scale));
        let eq48_e1670_d_n8: f64 = (p.p37 * (eq48_e1668_d_n8 * ddt_scale));
        let eq48_e1670_d_n9: f64 = (p.p37 * (eq48_e1668_d_n9 * ddt_scale));
        let eq48_e1670_d_n10: f64 = (p.p37 * (eq48_e1668_d_n10 * ddt_scale));
        let eq48_e1670_d_n11: f64 = (p.p37 * (eq48_e1668_d_n11 * ddt_scale));
        let eq48_e1670_d_n12: f64 = (p.p37 * (eq48_e1668_d_n12 * ddt_scale));
        let eq48_e1670_d_n13: f64 = (p.p37 * (eq48_e1668_d_n13 * ddt_scale));
        let eq48_e1670_d_b0: f64 = (p.p37 * (eq48_e1668_d_b0 * ddt_scale));
        let eq48_e1670_d_b1: f64 = (p.p37 * (eq48_e1668_d_b1 * ddt_scale));
        let eq48_e1670_d_b2: f64 = (p.p37 * (eq48_e1668_d_b2 * ddt_scale));
        let eq48_e1670_d_b3: f64 = (p.p37 * (eq48_e1668_d_b3 * ddt_scale));
        let eq48_e1670_d_b4: f64 = (p.p37 * (eq48_e1668_d_b4 * ddt_scale));
        let eq48_e1670_d_b5: f64 = (p.p37 * (eq48_e1668_d_b5 * ddt_scale));
        let eq48_e1670_d_b6: f64 = (p.p37 * (eq48_e1668_d_b6 * ddt_scale));
        let eq48_e1670_d_b7: f64 = (p.p37 * (eq48_e1668_d_b7 * ddt_scale));
        let eq48_e1670_d_b8: f64 = (p.p37 * (eq48_e1668_d_b8 * ddt_scale));
        let eq48_e1670_d_b9: f64 = (p.p37 * (eq48_e1668_d_b9 * ddt_scale));
        let eq48_e1670_d_b10: f64 = (p.p37 * (eq48_e1668_d_b10 * ddt_scale));
        let eq48_e1670_d_b11: f64 = (p.p37 * (eq48_e1668_d_b11 * ddt_scale));
        let eq48_e1670_d_b12: f64 = (p.p37 * (eq48_e1668_d_b12 * ddt_scale));
        let eq48_e1670_d_b13: f64 = (p.p37 * (eq48_e1668_d_b13 * ddt_scale));
        let eq48_e1670_d_b14: f64 = (p.p37 * (eq48_e1668_d_b14 * ddt_scale));
        let eq48_e1670_d_b15: f64 = (p.p37 * (eq48_e1668_d_b15 * ddt_scale));
        let eq48_e1670_d_b16: f64 = (p.p37 * (eq48_e1668_d_b16 * ddt_scale));
        let eq48_e1670_d_b17: f64 = (p.p37 * (eq48_e1668_d_b17 * ddt_scale));
        let eq48_value: f64 = eq48_e1670;
        let eq48_node_derivatives: [f64; 14] = [eq48_e1670_d_n0, eq48_e1670_d_n1, eq48_e1670_d_n2, eq48_e1670_d_n3, eq48_e1670_d_n4, eq48_e1670_d_n5, eq48_e1670_d_n6, eq48_e1670_d_n7, eq48_e1670_d_n8, eq48_e1670_d_n9, eq48_e1670_d_n10, eq48_e1670_d_n11, eq48_e1670_d_n12, eq48_e1670_d_n13];
        let eq48_branch_derivatives: [f64; 18] = [eq48_e1670_d_b0, eq48_e1670_d_b1, eq48_e1670_d_b2, eq48_e1670_d_b3, eq48_e1670_d_b4, eq48_e1670_d_b5, eq48_e1670_d_b6, eq48_e1670_d_b7, eq48_e1670_d_b8, eq48_e1670_d_b9, eq48_e1670_d_b10, eq48_e1670_d_b11, eq48_e1670_d_b12, eq48_e1670_d_b13, eq48_e1670_d_b14, eq48_e1670_d_b15, eq48_e1670_d_b16, eq48_e1670_d_b17];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let eq49_e1674: f64 = (p.p33 * s.v[910]);
        let eq49_e1674_d_n0: f64 = (p.p33 * s.dn[910][0]);
        let eq49_e1674_d_n1: f64 = (p.p33 * s.dn[910][1]);
        let eq49_e1674_d_n2: f64 = (p.p33 * s.dn[910][2]);
        let eq49_e1674_d_n3: f64 = (p.p33 * s.dn[910][3]);
        let eq49_e1674_d_n4: f64 = (p.p33 * s.dn[910][4]);
        let eq49_e1674_d_n5: f64 = (p.p33 * s.dn[910][5]);
        let eq49_e1674_d_n6: f64 = (p.p33 * s.dn[910][6]);
        let eq49_e1674_d_n7: f64 = (p.p33 * s.dn[910][7]);
        let eq49_e1674_d_n8: f64 = (p.p33 * s.dn[910][8]);
        let eq49_e1674_d_n9: f64 = (p.p33 * s.dn[910][9]);
        let eq49_e1674_d_n10: f64 = (p.p33 * s.dn[910][10]);
        let eq49_e1674_d_n11: f64 = (p.p33 * s.dn[910][11]);
        let eq49_e1674_d_n12: f64 = (p.p33 * s.dn[910][12]);
        let eq49_e1674_d_n13: f64 = (p.p33 * s.dn[910][13]);
        let eq49_e1674_d_b0: f64 = (p.p33 * s.db[910][0]);
        let eq49_e1674_d_b1: f64 = (p.p33 * s.db[910][1]);
        let eq49_e1674_d_b2: f64 = (p.p33 * s.db[910][2]);
        let eq49_e1674_d_b3: f64 = (p.p33 * s.db[910][3]);
        let eq49_e1674_d_b4: f64 = (p.p33 * s.db[910][4]);
        let eq49_e1674_d_b5: f64 = (p.p33 * s.db[910][5]);
        let eq49_e1674_d_b6: f64 = (p.p33 * s.db[910][6]);
        let eq49_e1674_d_b7: f64 = (p.p33 * s.db[910][7]);
        let eq49_e1674_d_b8: f64 = (p.p33 * s.db[910][8]);
        let eq49_e1674_d_b9: f64 = (p.p33 * s.db[910][9]);
        let eq49_e1674_d_b10: f64 = (p.p33 * s.db[910][10]);
        let eq49_e1674_d_b11: f64 = (p.p33 * s.db[910][11]);
        let eq49_e1674_d_b12: f64 = (p.p33 * s.db[910][12]);
        let eq49_e1674_d_b13: f64 = (p.p33 * s.db[910][13]);
        let eq49_e1674_d_b14: f64 = (p.p33 * s.db[910][14]);
        let eq49_e1674_d_b15: f64 = (p.p33 * s.db[910][15]);
        let eq49_e1674_d_b16: f64 = (p.p33 * s.db[910][16]);
        let eq49_e1674_d_b17: f64 = (p.p33 * s.db[910][17]);
        let eq49_e1675: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq49_e1674);
        let eq49_e1676: f64 = (p.p37 * eq49_e1675);
        let eq49_e1676_d_n0: f64 = (p.p37 * (eq49_e1674_d_n0 * ddt_scale));
        let eq49_e1676_d_n1: f64 = (p.p37 * (eq49_e1674_d_n1 * ddt_scale));
        let eq49_e1676_d_n2: f64 = (p.p37 * (eq49_e1674_d_n2 * ddt_scale));
        let eq49_e1676_d_n3: f64 = (p.p37 * (eq49_e1674_d_n3 * ddt_scale));
        let eq49_e1676_d_n4: f64 = (p.p37 * (eq49_e1674_d_n4 * ddt_scale));
        let eq49_e1676_d_n5: f64 = (p.p37 * (eq49_e1674_d_n5 * ddt_scale));
        let eq49_e1676_d_n6: f64 = (p.p37 * (eq49_e1674_d_n6 * ddt_scale));
        let eq49_e1676_d_n7: f64 = (p.p37 * (eq49_e1674_d_n7 * ddt_scale));
        let eq49_e1676_d_n8: f64 = (p.p37 * (eq49_e1674_d_n8 * ddt_scale));
        let eq49_e1676_d_n9: f64 = (p.p37 * (eq49_e1674_d_n9 * ddt_scale));
        let eq49_e1676_d_n10: f64 = (p.p37 * (eq49_e1674_d_n10 * ddt_scale));
        let eq49_e1676_d_n11: f64 = (p.p37 * (eq49_e1674_d_n11 * ddt_scale));
        let eq49_e1676_d_n12: f64 = (p.p37 * (eq49_e1674_d_n12 * ddt_scale));
        let eq49_e1676_d_n13: f64 = (p.p37 * (eq49_e1674_d_n13 * ddt_scale));
        let eq49_e1676_d_b0: f64 = (p.p37 * (eq49_e1674_d_b0 * ddt_scale));
        let eq49_e1676_d_b1: f64 = (p.p37 * (eq49_e1674_d_b1 * ddt_scale));
        let eq49_e1676_d_b2: f64 = (p.p37 * (eq49_e1674_d_b2 * ddt_scale));
        let eq49_e1676_d_b3: f64 = (p.p37 * (eq49_e1674_d_b3 * ddt_scale));
        let eq49_e1676_d_b4: f64 = (p.p37 * (eq49_e1674_d_b4 * ddt_scale));
        let eq49_e1676_d_b5: f64 = (p.p37 * (eq49_e1674_d_b5 * ddt_scale));
        let eq49_e1676_d_b6: f64 = (p.p37 * (eq49_e1674_d_b6 * ddt_scale));
        let eq49_e1676_d_b7: f64 = (p.p37 * (eq49_e1674_d_b7 * ddt_scale));
        let eq49_e1676_d_b8: f64 = (p.p37 * (eq49_e1674_d_b8 * ddt_scale));
        let eq49_e1676_d_b9: f64 = (p.p37 * (eq49_e1674_d_b9 * ddt_scale));
        let eq49_e1676_d_b10: f64 = (p.p37 * (eq49_e1674_d_b10 * ddt_scale));
        let eq49_e1676_d_b11: f64 = (p.p37 * (eq49_e1674_d_b11 * ddt_scale));
        let eq49_e1676_d_b12: f64 = (p.p37 * (eq49_e1674_d_b12 * ddt_scale));
        let eq49_e1676_d_b13: f64 = (p.p37 * (eq49_e1674_d_b13 * ddt_scale));
        let eq49_e1676_d_b14: f64 = (p.p37 * (eq49_e1674_d_b14 * ddt_scale));
        let eq49_e1676_d_b15: f64 = (p.p37 * (eq49_e1674_d_b15 * ddt_scale));
        let eq49_e1676_d_b16: f64 = (p.p37 * (eq49_e1674_d_b16 * ddt_scale));
        let eq49_e1676_d_b17: f64 = (p.p37 * (eq49_e1674_d_b17 * ddt_scale));
        let eq49_value: f64 = eq49_e1676;
        let eq49_node_derivatives: [f64; 14] = [eq49_e1676_d_n0, eq49_e1676_d_n1, eq49_e1676_d_n2, eq49_e1676_d_n3, eq49_e1676_d_n4, eq49_e1676_d_n5, eq49_e1676_d_n6, eq49_e1676_d_n7, eq49_e1676_d_n8, eq49_e1676_d_n9, eq49_e1676_d_n10, eq49_e1676_d_n11, eq49_e1676_d_n12, eq49_e1676_d_n13];
        let eq49_branch_derivatives: [f64; 18] = [eq49_e1676_d_b0, eq49_e1676_d_b1, eq49_e1676_d_b2, eq49_e1676_d_b3, eq49_e1676_d_b4, eq49_e1676_d_b5, eq49_e1676_d_b6, eq49_e1676_d_b7, eq49_e1676_d_b8, eq49_e1676_d_b9, eq49_e1676_d_b10, eq49_e1676_d_b11, eq49_e1676_d_b12, eq49_e1676_d_b13, eq49_e1676_d_b14, eq49_e1676_d_b15, eq49_e1676_d_b16, eq49_e1676_d_b17];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(8),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e1685, eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13, eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17,) = {
    if s.b[1553] {
        let eq50_e1681: f64 = (p.p33 * s.v[895]);
        let eq50_e1681_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq50_e1681_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq50_e1681_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq50_e1681_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq50_e1681_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq50_e1681_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq50_e1681_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq50_e1681_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq50_e1681_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq50_e1681_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq50_e1681_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq50_e1681_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq50_e1681_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq50_e1681_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq50_e1681_d_b0: f64 = (p.p33 * s.db[895][0]);
        let eq50_e1681_d_b1: f64 = (p.p33 * s.db[895][1]);
        let eq50_e1681_d_b2: f64 = (p.p33 * s.db[895][2]);
        let eq50_e1681_d_b3: f64 = (p.p33 * s.db[895][3]);
        let eq50_e1681_d_b4: f64 = (p.p33 * s.db[895][4]);
        let eq50_e1681_d_b5: f64 = (p.p33 * s.db[895][5]);
        let eq50_e1681_d_b6: f64 = (p.p33 * s.db[895][6]);
        let eq50_e1681_d_b7: f64 = (p.p33 * s.db[895][7]);
        let eq50_e1681_d_b8: f64 = (p.p33 * s.db[895][8]);
        let eq50_e1681_d_b9: f64 = (p.p33 * s.db[895][9]);
        let eq50_e1681_d_b10: f64 = (p.p33 * s.db[895][10]);
        let eq50_e1681_d_b11: f64 = (p.p33 * s.db[895][11]);
        let eq50_e1681_d_b12: f64 = (p.p33 * s.db[895][12]);
        let eq50_e1681_d_b13: f64 = (p.p33 * s.db[895][13]);
        let eq50_e1681_d_b14: f64 = (p.p33 * s.db[895][14]);
        let eq50_e1681_d_b15: f64 = (p.p33 * s.db[895][15]);
        let eq50_e1681_d_b16: f64 = (p.p33 * s.db[895][16]);
        let eq50_e1681_d_b17: f64 = (p.p33 * s.db[895][17]);
        let eq50_e1682: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq50_e1681);
        let eq50_e1683: f64 = (p.p37 * eq50_e1682);
        let eq50_e1683_d_n0: f64 = (p.p37 * (eq50_e1681_d_n0 * ddt_scale));
        let eq50_e1683_d_n1: f64 = (p.p37 * (eq50_e1681_d_n1 * ddt_scale));
        let eq50_e1683_d_n2: f64 = (p.p37 * (eq50_e1681_d_n2 * ddt_scale));
        let eq50_e1683_d_n3: f64 = (p.p37 * (eq50_e1681_d_n3 * ddt_scale));
        let eq50_e1683_d_n4: f64 = (p.p37 * (eq50_e1681_d_n4 * ddt_scale));
        let eq50_e1683_d_n5: f64 = (p.p37 * (eq50_e1681_d_n5 * ddt_scale));
        let eq50_e1683_d_n6: f64 = (p.p37 * (eq50_e1681_d_n6 * ddt_scale));
        let eq50_e1683_d_n7: f64 = (p.p37 * (eq50_e1681_d_n7 * ddt_scale));
        let eq50_e1683_d_n8: f64 = (p.p37 * (eq50_e1681_d_n8 * ddt_scale));
        let eq50_e1683_d_n9: f64 = (p.p37 * (eq50_e1681_d_n9 * ddt_scale));
        let eq50_e1683_d_n10: f64 = (p.p37 * (eq50_e1681_d_n10 * ddt_scale));
        let eq50_e1683_d_n11: f64 = (p.p37 * (eq50_e1681_d_n11 * ddt_scale));
        let eq50_e1683_d_n12: f64 = (p.p37 * (eq50_e1681_d_n12 * ddt_scale));
        let eq50_e1683_d_n13: f64 = (p.p37 * (eq50_e1681_d_n13 * ddt_scale));
        let eq50_e1683_d_b0: f64 = (p.p37 * (eq50_e1681_d_b0 * ddt_scale));
        let eq50_e1683_d_b1: f64 = (p.p37 * (eq50_e1681_d_b1 * ddt_scale));
        let eq50_e1683_d_b2: f64 = (p.p37 * (eq50_e1681_d_b2 * ddt_scale));
        let eq50_e1683_d_b3: f64 = (p.p37 * (eq50_e1681_d_b3 * ddt_scale));
        let eq50_e1683_d_b4: f64 = (p.p37 * (eq50_e1681_d_b4 * ddt_scale));
        let eq50_e1683_d_b5: f64 = (p.p37 * (eq50_e1681_d_b5 * ddt_scale));
        let eq50_e1683_d_b6: f64 = (p.p37 * (eq50_e1681_d_b6 * ddt_scale));
        let eq50_e1683_d_b7: f64 = (p.p37 * (eq50_e1681_d_b7 * ddt_scale));
        let eq50_e1683_d_b8: f64 = (p.p37 * (eq50_e1681_d_b8 * ddt_scale));
        let eq50_e1683_d_b9: f64 = (p.p37 * (eq50_e1681_d_b9 * ddt_scale));
        let eq50_e1683_d_b10: f64 = (p.p37 * (eq50_e1681_d_b10 * ddt_scale));
        let eq50_e1683_d_b11: f64 = (p.p37 * (eq50_e1681_d_b11 * ddt_scale));
        let eq50_e1683_d_b12: f64 = (p.p37 * (eq50_e1681_d_b12 * ddt_scale));
        let eq50_e1683_d_b13: f64 = (p.p37 * (eq50_e1681_d_b13 * ddt_scale));
        let eq50_e1683_d_b14: f64 = (p.p37 * (eq50_e1681_d_b14 * ddt_scale));
        let eq50_e1683_d_b15: f64 = (p.p37 * (eq50_e1681_d_b15 * ddt_scale));
        let eq50_e1683_d_b16: f64 = (p.p37 * (eq50_e1681_d_b16 * ddt_scale));
        let eq50_e1683_d_b17: f64 = (p.p37 * (eq50_e1681_d_b17 * ddt_scale));
        (eq50_e1683, eq50_e1683_d_n0, eq50_e1683_d_n1, eq50_e1683_d_n2, eq50_e1683_d_n3, eq50_e1683_d_n4, eq50_e1683_d_n5, eq50_e1683_d_n6, eq50_e1683_d_n7, eq50_e1683_d_n8, eq50_e1683_d_n9, eq50_e1683_d_n10, eq50_e1683_d_n11, eq50_e1683_d_n12, eq50_e1683_d_n13, eq50_e1683_d_b0, eq50_e1683_d_b1, eq50_e1683_d_b2, eq50_e1683_d_b3, eq50_e1683_d_b4, eq50_e1683_d_b5, eq50_e1683_d_b6, eq50_e1683_d_b7, eq50_e1683_d_b8, eq50_e1683_d_b9, eq50_e1683_d_b10, eq50_e1683_d_b11, eq50_e1683_d_b12, eq50_e1683_d_b13, eq50_e1683_d_b14, eq50_e1683_d_b15, eq50_e1683_d_b16, eq50_e1683_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e1685;
        let eq50_node_derivatives: [f64; 14] = [eq50_e1685_d_n0, eq50_e1685_d_n1, eq50_e1685_d_n2, eq50_e1685_d_n3, eq50_e1685_d_n4, eq50_e1685_d_n5, eq50_e1685_d_n6, eq50_e1685_d_n7, eq50_e1685_d_n8, eq50_e1685_d_n9, eq50_e1685_d_n10, eq50_e1685_d_n11, eq50_e1685_d_n12, eq50_e1685_d_n13];
        let eq50_branch_derivatives: [f64; 18] = [eq50_e1685_d_b0, eq50_e1685_d_b1, eq50_e1685_d_b2, eq50_e1685_d_b3, eq50_e1685_d_b4, eq50_e1685_d_b5, eq50_e1685_d_b6, eq50_e1685_d_b7, eq50_e1685_d_b8, eq50_e1685_d_b9, eq50_e1685_d_b10, eq50_e1685_d_b11, eq50_e1685_d_b12, eq50_e1685_d_b13, eq50_e1685_d_b14, eq50_e1685_d_b15, eq50_e1685_d_b16, eq50_e1685_d_b17];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e1694, eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13, eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17,) = {
    if s.b[1553] {
        let eq51_e1690: f64 = (p.p33 * s.v[896]);
        let eq51_e1690_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq51_e1690_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq51_e1690_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq51_e1690_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq51_e1690_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq51_e1690_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq51_e1690_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq51_e1690_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq51_e1690_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq51_e1690_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq51_e1690_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq51_e1690_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq51_e1690_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq51_e1690_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq51_e1690_d_b0: f64 = (p.p33 * s.db[896][0]);
        let eq51_e1690_d_b1: f64 = (p.p33 * s.db[896][1]);
        let eq51_e1690_d_b2: f64 = (p.p33 * s.db[896][2]);
        let eq51_e1690_d_b3: f64 = (p.p33 * s.db[896][3]);
        let eq51_e1690_d_b4: f64 = (p.p33 * s.db[896][4]);
        let eq51_e1690_d_b5: f64 = (p.p33 * s.db[896][5]);
        let eq51_e1690_d_b6: f64 = (p.p33 * s.db[896][6]);
        let eq51_e1690_d_b7: f64 = (p.p33 * s.db[896][7]);
        let eq51_e1690_d_b8: f64 = (p.p33 * s.db[896][8]);
        let eq51_e1690_d_b9: f64 = (p.p33 * s.db[896][9]);
        let eq51_e1690_d_b10: f64 = (p.p33 * s.db[896][10]);
        let eq51_e1690_d_b11: f64 = (p.p33 * s.db[896][11]);
        let eq51_e1690_d_b12: f64 = (p.p33 * s.db[896][12]);
        let eq51_e1690_d_b13: f64 = (p.p33 * s.db[896][13]);
        let eq51_e1690_d_b14: f64 = (p.p33 * s.db[896][14]);
        let eq51_e1690_d_b15: f64 = (p.p33 * s.db[896][15]);
        let eq51_e1690_d_b16: f64 = (p.p33 * s.db[896][16]);
        let eq51_e1690_d_b17: f64 = (p.p33 * s.db[896][17]);
        let eq51_e1691: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq51_e1690);
        let eq51_e1692: f64 = (p.p37 * eq51_e1691);
        let eq51_e1692_d_n0: f64 = (p.p37 * (eq51_e1690_d_n0 * ddt_scale));
        let eq51_e1692_d_n1: f64 = (p.p37 * (eq51_e1690_d_n1 * ddt_scale));
        let eq51_e1692_d_n2: f64 = (p.p37 * (eq51_e1690_d_n2 * ddt_scale));
        let eq51_e1692_d_n3: f64 = (p.p37 * (eq51_e1690_d_n3 * ddt_scale));
        let eq51_e1692_d_n4: f64 = (p.p37 * (eq51_e1690_d_n4 * ddt_scale));
        let eq51_e1692_d_n5: f64 = (p.p37 * (eq51_e1690_d_n5 * ddt_scale));
        let eq51_e1692_d_n6: f64 = (p.p37 * (eq51_e1690_d_n6 * ddt_scale));
        let eq51_e1692_d_n7: f64 = (p.p37 * (eq51_e1690_d_n7 * ddt_scale));
        let eq51_e1692_d_n8: f64 = (p.p37 * (eq51_e1690_d_n8 * ddt_scale));
        let eq51_e1692_d_n9: f64 = (p.p37 * (eq51_e1690_d_n9 * ddt_scale));
        let eq51_e1692_d_n10: f64 = (p.p37 * (eq51_e1690_d_n10 * ddt_scale));
        let eq51_e1692_d_n11: f64 = (p.p37 * (eq51_e1690_d_n11 * ddt_scale));
        let eq51_e1692_d_n12: f64 = (p.p37 * (eq51_e1690_d_n12 * ddt_scale));
        let eq51_e1692_d_n13: f64 = (p.p37 * (eq51_e1690_d_n13 * ddt_scale));
        let eq51_e1692_d_b0: f64 = (p.p37 * (eq51_e1690_d_b0 * ddt_scale));
        let eq51_e1692_d_b1: f64 = (p.p37 * (eq51_e1690_d_b1 * ddt_scale));
        let eq51_e1692_d_b2: f64 = (p.p37 * (eq51_e1690_d_b2 * ddt_scale));
        let eq51_e1692_d_b3: f64 = (p.p37 * (eq51_e1690_d_b3 * ddt_scale));
        let eq51_e1692_d_b4: f64 = (p.p37 * (eq51_e1690_d_b4 * ddt_scale));
        let eq51_e1692_d_b5: f64 = (p.p37 * (eq51_e1690_d_b5 * ddt_scale));
        let eq51_e1692_d_b6: f64 = (p.p37 * (eq51_e1690_d_b6 * ddt_scale));
        let eq51_e1692_d_b7: f64 = (p.p37 * (eq51_e1690_d_b7 * ddt_scale));
        let eq51_e1692_d_b8: f64 = (p.p37 * (eq51_e1690_d_b8 * ddt_scale));
        let eq51_e1692_d_b9: f64 = (p.p37 * (eq51_e1690_d_b9 * ddt_scale));
        let eq51_e1692_d_b10: f64 = (p.p37 * (eq51_e1690_d_b10 * ddt_scale));
        let eq51_e1692_d_b11: f64 = (p.p37 * (eq51_e1690_d_b11 * ddt_scale));
        let eq51_e1692_d_b12: f64 = (p.p37 * (eq51_e1690_d_b12 * ddt_scale));
        let eq51_e1692_d_b13: f64 = (p.p37 * (eq51_e1690_d_b13 * ddt_scale));
        let eq51_e1692_d_b14: f64 = (p.p37 * (eq51_e1690_d_b14 * ddt_scale));
        let eq51_e1692_d_b15: f64 = (p.p37 * (eq51_e1690_d_b15 * ddt_scale));
        let eq51_e1692_d_b16: f64 = (p.p37 * (eq51_e1690_d_b16 * ddt_scale));
        let eq51_e1692_d_b17: f64 = (p.p37 * (eq51_e1690_d_b17 * ddt_scale));
        (eq51_e1692, eq51_e1692_d_n0, eq51_e1692_d_n1, eq51_e1692_d_n2, eq51_e1692_d_n3, eq51_e1692_d_n4, eq51_e1692_d_n5, eq51_e1692_d_n6, eq51_e1692_d_n7, eq51_e1692_d_n8, eq51_e1692_d_n9, eq51_e1692_d_n10, eq51_e1692_d_n11, eq51_e1692_d_n12, eq51_e1692_d_n13, eq51_e1692_d_b0, eq51_e1692_d_b1, eq51_e1692_d_b2, eq51_e1692_d_b3, eq51_e1692_d_b4, eq51_e1692_d_b5, eq51_e1692_d_b6, eq51_e1692_d_b7, eq51_e1692_d_b8, eq51_e1692_d_b9, eq51_e1692_d_b10, eq51_e1692_d_b11, eq51_e1692_d_b12, eq51_e1692_d_b13, eq51_e1692_d_b14, eq51_e1692_d_b15, eq51_e1692_d_b16, eq51_e1692_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1694;
        let eq51_node_derivatives: [f64; 14] = [eq51_e1694_d_n0, eq51_e1694_d_n1, eq51_e1694_d_n2, eq51_e1694_d_n3, eq51_e1694_d_n4, eq51_e1694_d_n5, eq51_e1694_d_n6, eq51_e1694_d_n7, eq51_e1694_d_n8, eq51_e1694_d_n9, eq51_e1694_d_n10, eq51_e1694_d_n11, eq51_e1694_d_n12, eq51_e1694_d_n13];
        let eq51_branch_derivatives: [f64; 18] = [eq51_e1694_d_b0, eq51_e1694_d_b1, eq51_e1694_d_b2, eq51_e1694_d_b3, eq51_e1694_d_b4, eq51_e1694_d_b5, eq51_e1694_d_b6, eq51_e1694_d_b7, eq51_e1694_d_b8, eq51_e1694_d_b9, eq51_e1694_d_b10, eq51_e1694_d_b11, eq51_e1694_d_b12, eq51_e1694_d_b13, eq51_e1694_d_b14, eq51_e1694_d_b15, eq51_e1694_d_b16, eq51_e1694_d_b17];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(8),
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let (eq52_e1703, eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13, eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17,) = {
    if s.b[1553] {
        let eq52_e1698: f64 = (p.p33 * (nv10 - nv3));
        let eq52_e1700: f64 = (eq52_e1698 * s.v[336]);
        let eq52_e1700_d_n0: f64 = (eq52_e1698 * s.dn[336][0]);
        let eq52_e1700_d_n1: f64 = (eq52_e1698 * s.dn[336][1]);
        let eq52_e1700_d_n2: f64 = (eq52_e1698 * s.dn[336][2]);
        let eq52_e1700_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq52_e1698 * s.dn[336][3]));
        let eq52_e1700_d_n4: f64 = (eq52_e1698 * s.dn[336][4]);
        let eq52_e1700_d_n5: f64 = (eq52_e1698 * s.dn[336][5]);
        let eq52_e1700_d_n6: f64 = (eq52_e1698 * s.dn[336][6]);
        let eq52_e1700_d_n7: f64 = (eq52_e1698 * s.dn[336][7]);
        let eq52_e1700_d_n8: f64 = (eq52_e1698 * s.dn[336][8]);
        let eq52_e1700_d_n9: f64 = (eq52_e1698 * s.dn[336][9]);
        let eq52_e1700_d_n10: f64 = ((p.p33 * s.v[336]) + (eq52_e1698 * s.dn[336][10]));
        let eq52_e1700_d_n11: f64 = (eq52_e1698 * s.dn[336][11]);
        let eq52_e1700_d_n12: f64 = (eq52_e1698 * s.dn[336][12]);
        let eq52_e1700_d_n13: f64 = (eq52_e1698 * s.dn[336][13]);
        let eq52_e1700_d_b0: f64 = (eq52_e1698 * s.db[336][0]);
        let eq52_e1700_d_b1: f64 = (eq52_e1698 * s.db[336][1]);
        let eq52_e1700_d_b2: f64 = (eq52_e1698 * s.db[336][2]);
        let eq52_e1700_d_b3: f64 = (eq52_e1698 * s.db[336][3]);
        let eq52_e1700_d_b4: f64 = (eq52_e1698 * s.db[336][4]);
        let eq52_e1700_d_b5: f64 = (eq52_e1698 * s.db[336][5]);
        let eq52_e1700_d_b6: f64 = (eq52_e1698 * s.db[336][6]);
        let eq52_e1700_d_b7: f64 = (eq52_e1698 * s.db[336][7]);
        let eq52_e1700_d_b8: f64 = (eq52_e1698 * s.db[336][8]);
        let eq52_e1700_d_b9: f64 = (eq52_e1698 * s.db[336][9]);
        let eq52_e1700_d_b10: f64 = (eq52_e1698 * s.db[336][10]);
        let eq52_e1700_d_b11: f64 = (eq52_e1698 * s.db[336][11]);
        let eq52_e1700_d_b12: f64 = (eq52_e1698 * s.db[336][12]);
        let eq52_e1700_d_b13: f64 = (eq52_e1698 * s.db[336][13]);
        let eq52_e1700_d_b14: f64 = (eq52_e1698 * s.db[336][14]);
        let eq52_e1700_d_b15: f64 = (eq52_e1698 * s.db[336][15]);
        let eq52_e1700_d_b16: f64 = (eq52_e1698 * s.db[336][16]);
        let eq52_e1700_d_b17: f64 = (eq52_e1698 * s.db[336][17]);
        let eq52_e1701: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq52_e1700);
        (eq52_e1701, (eq52_e1700_d_n0 * ddt_scale), (eq52_e1700_d_n1 * ddt_scale), (eq52_e1700_d_n2 * ddt_scale), (eq52_e1700_d_n3 * ddt_scale), (eq52_e1700_d_n4 * ddt_scale), (eq52_e1700_d_n5 * ddt_scale), (eq52_e1700_d_n6 * ddt_scale), (eq52_e1700_d_n7 * ddt_scale), (eq52_e1700_d_n8 * ddt_scale), (eq52_e1700_d_n9 * ddt_scale), (eq52_e1700_d_n10 * ddt_scale), (eq52_e1700_d_n11 * ddt_scale), (eq52_e1700_d_n12 * ddt_scale), (eq52_e1700_d_n13 * ddt_scale), (eq52_e1700_d_b0 * ddt_scale), (eq52_e1700_d_b1 * ddt_scale), (eq52_e1700_d_b2 * ddt_scale), (eq52_e1700_d_b3 * ddt_scale), (eq52_e1700_d_b4 * ddt_scale), (eq52_e1700_d_b5 * ddt_scale), (eq52_e1700_d_b6 * ddt_scale), (eq52_e1700_d_b7 * ddt_scale), (eq52_e1700_d_b8 * ddt_scale), (eq52_e1700_d_b9 * ddt_scale), (eq52_e1700_d_b10 * ddt_scale), (eq52_e1700_d_b11 * ddt_scale), (eq52_e1700_d_b12 * ddt_scale), (eq52_e1700_d_b13 * ddt_scale), (eq52_e1700_d_b14 * ddt_scale), (eq52_e1700_d_b15 * ddt_scale), (eq52_e1700_d_b16 * ddt_scale), (eq52_e1700_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e1703;
        let eq52_node_derivatives: [f64; 14] = [eq52_e1703_d_n0, eq52_e1703_d_n1, eq52_e1703_d_n2, eq52_e1703_d_n3, eq52_e1703_d_n4, eq52_e1703_d_n5, eq52_e1703_d_n6, eq52_e1703_d_n7, eq52_e1703_d_n8, eq52_e1703_d_n9, eq52_e1703_d_n10, eq52_e1703_d_n11, eq52_e1703_d_n12, eq52_e1703_d_n13];
        let eq52_branch_derivatives: [f64; 18] = [eq52_e1703_d_b0, eq52_e1703_d_b1, eq52_e1703_d_b2, eq52_e1703_d_b3, eq52_e1703_d_b4, eq52_e1703_d_b5, eq52_e1703_d_b6, eq52_e1703_d_b7, eq52_e1703_d_b8, eq52_e1703_d_b9, eq52_e1703_d_b10, eq52_e1703_d_b11, eq52_e1703_d_b12, eq52_e1703_d_b13, eq52_e1703_d_b14, eq52_e1703_d_b15, eq52_e1703_d_b16, eq52_e1703_d_b17];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(3),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1713, eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13, eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17,) = {
    if (!s.b[1553]) {
        let eq53_e1709: f64 = (p.p33 * s.v[895]);
        let eq53_e1709_d_n0: f64 = (p.p33 * s.dn[895][0]);
        let eq53_e1709_d_n1: f64 = (p.p33 * s.dn[895][1]);
        let eq53_e1709_d_n2: f64 = (p.p33 * s.dn[895][2]);
        let eq53_e1709_d_n3: f64 = (p.p33 * s.dn[895][3]);
        let eq53_e1709_d_n4: f64 = (p.p33 * s.dn[895][4]);
        let eq53_e1709_d_n5: f64 = (p.p33 * s.dn[895][5]);
        let eq53_e1709_d_n6: f64 = (p.p33 * s.dn[895][6]);
        let eq53_e1709_d_n7: f64 = (p.p33 * s.dn[895][7]);
        let eq53_e1709_d_n8: f64 = (p.p33 * s.dn[895][8]);
        let eq53_e1709_d_n9: f64 = (p.p33 * s.dn[895][9]);
        let eq53_e1709_d_n10: f64 = (p.p33 * s.dn[895][10]);
        let eq53_e1709_d_n11: f64 = (p.p33 * s.dn[895][11]);
        let eq53_e1709_d_n12: f64 = (p.p33 * s.dn[895][12]);
        let eq53_e1709_d_n13: f64 = (p.p33 * s.dn[895][13]);
        let eq53_e1709_d_b0: f64 = (p.p33 * s.db[895][0]);
        let eq53_e1709_d_b1: f64 = (p.p33 * s.db[895][1]);
        let eq53_e1709_d_b2: f64 = (p.p33 * s.db[895][2]);
        let eq53_e1709_d_b3: f64 = (p.p33 * s.db[895][3]);
        let eq53_e1709_d_b4: f64 = (p.p33 * s.db[895][4]);
        let eq53_e1709_d_b5: f64 = (p.p33 * s.db[895][5]);
        let eq53_e1709_d_b6: f64 = (p.p33 * s.db[895][6]);
        let eq53_e1709_d_b7: f64 = (p.p33 * s.db[895][7]);
        let eq53_e1709_d_b8: f64 = (p.p33 * s.db[895][8]);
        let eq53_e1709_d_b9: f64 = (p.p33 * s.db[895][9]);
        let eq53_e1709_d_b10: f64 = (p.p33 * s.db[895][10]);
        let eq53_e1709_d_b11: f64 = (p.p33 * s.db[895][11]);
        let eq53_e1709_d_b12: f64 = (p.p33 * s.db[895][12]);
        let eq53_e1709_d_b13: f64 = (p.p33 * s.db[895][13]);
        let eq53_e1709_d_b14: f64 = (p.p33 * s.db[895][14]);
        let eq53_e1709_d_b15: f64 = (p.p33 * s.db[895][15]);
        let eq53_e1709_d_b16: f64 = (p.p33 * s.db[895][16]);
        let eq53_e1709_d_b17: f64 = (p.p33 * s.db[895][17]);
        let eq53_e1710: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq53_e1709);
        let eq53_e1711: f64 = (p.p37 * eq53_e1710);
        let eq53_e1711_d_n0: f64 = (p.p37 * (eq53_e1709_d_n0 * ddt_scale));
        let eq53_e1711_d_n1: f64 = (p.p37 * (eq53_e1709_d_n1 * ddt_scale));
        let eq53_e1711_d_n2: f64 = (p.p37 * (eq53_e1709_d_n2 * ddt_scale));
        let eq53_e1711_d_n3: f64 = (p.p37 * (eq53_e1709_d_n3 * ddt_scale));
        let eq53_e1711_d_n4: f64 = (p.p37 * (eq53_e1709_d_n4 * ddt_scale));
        let eq53_e1711_d_n5: f64 = (p.p37 * (eq53_e1709_d_n5 * ddt_scale));
        let eq53_e1711_d_n6: f64 = (p.p37 * (eq53_e1709_d_n6 * ddt_scale));
        let eq53_e1711_d_n7: f64 = (p.p37 * (eq53_e1709_d_n7 * ddt_scale));
        let eq53_e1711_d_n8: f64 = (p.p37 * (eq53_e1709_d_n8 * ddt_scale));
        let eq53_e1711_d_n9: f64 = (p.p37 * (eq53_e1709_d_n9 * ddt_scale));
        let eq53_e1711_d_n10: f64 = (p.p37 * (eq53_e1709_d_n10 * ddt_scale));
        let eq53_e1711_d_n11: f64 = (p.p37 * (eq53_e1709_d_n11 * ddt_scale));
        let eq53_e1711_d_n12: f64 = (p.p37 * (eq53_e1709_d_n12 * ddt_scale));
        let eq53_e1711_d_n13: f64 = (p.p37 * (eq53_e1709_d_n13 * ddt_scale));
        let eq53_e1711_d_b0: f64 = (p.p37 * (eq53_e1709_d_b0 * ddt_scale));
        let eq53_e1711_d_b1: f64 = (p.p37 * (eq53_e1709_d_b1 * ddt_scale));
        let eq53_e1711_d_b2: f64 = (p.p37 * (eq53_e1709_d_b2 * ddt_scale));
        let eq53_e1711_d_b3: f64 = (p.p37 * (eq53_e1709_d_b3 * ddt_scale));
        let eq53_e1711_d_b4: f64 = (p.p37 * (eq53_e1709_d_b4 * ddt_scale));
        let eq53_e1711_d_b5: f64 = (p.p37 * (eq53_e1709_d_b5 * ddt_scale));
        let eq53_e1711_d_b6: f64 = (p.p37 * (eq53_e1709_d_b6 * ddt_scale));
        let eq53_e1711_d_b7: f64 = (p.p37 * (eq53_e1709_d_b7 * ddt_scale));
        let eq53_e1711_d_b8: f64 = (p.p37 * (eq53_e1709_d_b8 * ddt_scale));
        let eq53_e1711_d_b9: f64 = (p.p37 * (eq53_e1709_d_b9 * ddt_scale));
        let eq53_e1711_d_b10: f64 = (p.p37 * (eq53_e1709_d_b10 * ddt_scale));
        let eq53_e1711_d_b11: f64 = (p.p37 * (eq53_e1709_d_b11 * ddt_scale));
        let eq53_e1711_d_b12: f64 = (p.p37 * (eq53_e1709_d_b12 * ddt_scale));
        let eq53_e1711_d_b13: f64 = (p.p37 * (eq53_e1709_d_b13 * ddt_scale));
        let eq53_e1711_d_b14: f64 = (p.p37 * (eq53_e1709_d_b14 * ddt_scale));
        let eq53_e1711_d_b15: f64 = (p.p37 * (eq53_e1709_d_b15 * ddt_scale));
        let eq53_e1711_d_b16: f64 = (p.p37 * (eq53_e1709_d_b16 * ddt_scale));
        let eq53_e1711_d_b17: f64 = (p.p37 * (eq53_e1709_d_b17 * ddt_scale));
        (eq53_e1711, eq53_e1711_d_n0, eq53_e1711_d_n1, eq53_e1711_d_n2, eq53_e1711_d_n3, eq53_e1711_d_n4, eq53_e1711_d_n5, eq53_e1711_d_n6, eq53_e1711_d_n7, eq53_e1711_d_n8, eq53_e1711_d_n9, eq53_e1711_d_n10, eq53_e1711_d_n11, eq53_e1711_d_n12, eq53_e1711_d_n13, eq53_e1711_d_b0, eq53_e1711_d_b1, eq53_e1711_d_b2, eq53_e1711_d_b3, eq53_e1711_d_b4, eq53_e1711_d_b5, eq53_e1711_d_b6, eq53_e1711_d_b7, eq53_e1711_d_b8, eq53_e1711_d_b9, eq53_e1711_d_b10, eq53_e1711_d_b11, eq53_e1711_d_b12, eq53_e1711_d_b13, eq53_e1711_d_b14, eq53_e1711_d_b15, eq53_e1711_d_b16, eq53_e1711_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1713;
        let eq53_node_derivatives: [f64; 14] = [eq53_e1713_d_n0, eq53_e1713_d_n1, eq53_e1713_d_n2, eq53_e1713_d_n3, eq53_e1713_d_n4, eq53_e1713_d_n5, eq53_e1713_d_n6, eq53_e1713_d_n7, eq53_e1713_d_n8, eq53_e1713_d_n9, eq53_e1713_d_n10, eq53_e1713_d_n11, eq53_e1713_d_n12, eq53_e1713_d_n13];
        let eq53_branch_derivatives: [f64; 18] = [eq53_e1713_d_b0, eq53_e1713_d_b1, eq53_e1713_d_b2, eq53_e1713_d_b3, eq53_e1713_d_b4, eq53_e1713_d_b5, eq53_e1713_d_b6, eq53_e1713_d_b7, eq53_e1713_d_b8, eq53_e1713_d_b9, eq53_e1713_d_b10, eq53_e1713_d_b11, eq53_e1713_d_b12, eq53_e1713_d_b13, eq53_e1713_d_b14, eq53_e1713_d_b15, eq53_e1713_d_b16, eq53_e1713_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e1723, eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13, eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17,) = {
    if (!s.b[1553]) {
        let eq54_e1719: f64 = (p.p33 * s.v[896]);
        let eq54_e1719_d_n0: f64 = (p.p33 * s.dn[896][0]);
        let eq54_e1719_d_n1: f64 = (p.p33 * s.dn[896][1]);
        let eq54_e1719_d_n2: f64 = (p.p33 * s.dn[896][2]);
        let eq54_e1719_d_n3: f64 = (p.p33 * s.dn[896][3]);
        let eq54_e1719_d_n4: f64 = (p.p33 * s.dn[896][4]);
        let eq54_e1719_d_n5: f64 = (p.p33 * s.dn[896][5]);
        let eq54_e1719_d_n6: f64 = (p.p33 * s.dn[896][6]);
        let eq54_e1719_d_n7: f64 = (p.p33 * s.dn[896][7]);
        let eq54_e1719_d_n8: f64 = (p.p33 * s.dn[896][8]);
        let eq54_e1719_d_n9: f64 = (p.p33 * s.dn[896][9]);
        let eq54_e1719_d_n10: f64 = (p.p33 * s.dn[896][10]);
        let eq54_e1719_d_n11: f64 = (p.p33 * s.dn[896][11]);
        let eq54_e1719_d_n12: f64 = (p.p33 * s.dn[896][12]);
        let eq54_e1719_d_n13: f64 = (p.p33 * s.dn[896][13]);
        let eq54_e1719_d_b0: f64 = (p.p33 * s.db[896][0]);
        let eq54_e1719_d_b1: f64 = (p.p33 * s.db[896][1]);
        let eq54_e1719_d_b2: f64 = (p.p33 * s.db[896][2]);
        let eq54_e1719_d_b3: f64 = (p.p33 * s.db[896][3]);
        let eq54_e1719_d_b4: f64 = (p.p33 * s.db[896][4]);
        let eq54_e1719_d_b5: f64 = (p.p33 * s.db[896][5]);
        let eq54_e1719_d_b6: f64 = (p.p33 * s.db[896][6]);
        let eq54_e1719_d_b7: f64 = (p.p33 * s.db[896][7]);
        let eq54_e1719_d_b8: f64 = (p.p33 * s.db[896][8]);
        let eq54_e1719_d_b9: f64 = (p.p33 * s.db[896][9]);
        let eq54_e1719_d_b10: f64 = (p.p33 * s.db[896][10]);
        let eq54_e1719_d_b11: f64 = (p.p33 * s.db[896][11]);
        let eq54_e1719_d_b12: f64 = (p.p33 * s.db[896][12]);
        let eq54_e1719_d_b13: f64 = (p.p33 * s.db[896][13]);
        let eq54_e1719_d_b14: f64 = (p.p33 * s.db[896][14]);
        let eq54_e1719_d_b15: f64 = (p.p33 * s.db[896][15]);
        let eq54_e1719_d_b16: f64 = (p.p33 * s.db[896][16]);
        let eq54_e1719_d_b17: f64 = (p.p33 * s.db[896][17]);
        let eq54_e1720: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 12, eq54_e1719);
        let eq54_e1721: f64 = (p.p37 * eq54_e1720);
        let eq54_e1721_d_n0: f64 = (p.p37 * (eq54_e1719_d_n0 * ddt_scale));
        let eq54_e1721_d_n1: f64 = (p.p37 * (eq54_e1719_d_n1 * ddt_scale));
        let eq54_e1721_d_n2: f64 = (p.p37 * (eq54_e1719_d_n2 * ddt_scale));
        let eq54_e1721_d_n3: f64 = (p.p37 * (eq54_e1719_d_n3 * ddt_scale));
        let eq54_e1721_d_n4: f64 = (p.p37 * (eq54_e1719_d_n4 * ddt_scale));
        let eq54_e1721_d_n5: f64 = (p.p37 * (eq54_e1719_d_n5 * ddt_scale));
        let eq54_e1721_d_n6: f64 = (p.p37 * (eq54_e1719_d_n6 * ddt_scale));
        let eq54_e1721_d_n7: f64 = (p.p37 * (eq54_e1719_d_n7 * ddt_scale));
        let eq54_e1721_d_n8: f64 = (p.p37 * (eq54_e1719_d_n8 * ddt_scale));
        let eq54_e1721_d_n9: f64 = (p.p37 * (eq54_e1719_d_n9 * ddt_scale));
        let eq54_e1721_d_n10: f64 = (p.p37 * (eq54_e1719_d_n10 * ddt_scale));
        let eq54_e1721_d_n11: f64 = (p.p37 * (eq54_e1719_d_n11 * ddt_scale));
        let eq54_e1721_d_n12: f64 = (p.p37 * (eq54_e1719_d_n12 * ddt_scale));
        let eq54_e1721_d_n13: f64 = (p.p37 * (eq54_e1719_d_n13 * ddt_scale));
        let eq54_e1721_d_b0: f64 = (p.p37 * (eq54_e1719_d_b0 * ddt_scale));
        let eq54_e1721_d_b1: f64 = (p.p37 * (eq54_e1719_d_b1 * ddt_scale));
        let eq54_e1721_d_b2: f64 = (p.p37 * (eq54_e1719_d_b2 * ddt_scale));
        let eq54_e1721_d_b3: f64 = (p.p37 * (eq54_e1719_d_b3 * ddt_scale));
        let eq54_e1721_d_b4: f64 = (p.p37 * (eq54_e1719_d_b4 * ddt_scale));
        let eq54_e1721_d_b5: f64 = (p.p37 * (eq54_e1719_d_b5 * ddt_scale));
        let eq54_e1721_d_b6: f64 = (p.p37 * (eq54_e1719_d_b6 * ddt_scale));
        let eq54_e1721_d_b7: f64 = (p.p37 * (eq54_e1719_d_b7 * ddt_scale));
        let eq54_e1721_d_b8: f64 = (p.p37 * (eq54_e1719_d_b8 * ddt_scale));
        let eq54_e1721_d_b9: f64 = (p.p37 * (eq54_e1719_d_b9 * ddt_scale));
        let eq54_e1721_d_b10: f64 = (p.p37 * (eq54_e1719_d_b10 * ddt_scale));
        let eq54_e1721_d_b11: f64 = (p.p37 * (eq54_e1719_d_b11 * ddt_scale));
        let eq54_e1721_d_b12: f64 = (p.p37 * (eq54_e1719_d_b12 * ddt_scale));
        let eq54_e1721_d_b13: f64 = (p.p37 * (eq54_e1719_d_b13 * ddt_scale));
        let eq54_e1721_d_b14: f64 = (p.p37 * (eq54_e1719_d_b14 * ddt_scale));
        let eq54_e1721_d_b15: f64 = (p.p37 * (eq54_e1719_d_b15 * ddt_scale));
        let eq54_e1721_d_b16: f64 = (p.p37 * (eq54_e1719_d_b16 * ddt_scale));
        let eq54_e1721_d_b17: f64 = (p.p37 * (eq54_e1719_d_b17 * ddt_scale));
        (eq54_e1721, eq54_e1721_d_n0, eq54_e1721_d_n1, eq54_e1721_d_n2, eq54_e1721_d_n3, eq54_e1721_d_n4, eq54_e1721_d_n5, eq54_e1721_d_n6, eq54_e1721_d_n7, eq54_e1721_d_n8, eq54_e1721_d_n9, eq54_e1721_d_n10, eq54_e1721_d_n11, eq54_e1721_d_n12, eq54_e1721_d_n13, eq54_e1721_d_b0, eq54_e1721_d_b1, eq54_e1721_d_b2, eq54_e1721_d_b3, eq54_e1721_d_b4, eq54_e1721_d_b5, eq54_e1721_d_b6, eq54_e1721_d_b7, eq54_e1721_d_b8, eq54_e1721_d_b9, eq54_e1721_d_b10, eq54_e1721_d_b11, eq54_e1721_d_b12, eq54_e1721_d_b13, eq54_e1721_d_b14, eq54_e1721_d_b15, eq54_e1721_d_b16, eq54_e1721_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e1723;
        let eq54_node_derivatives: [f64; 14] = [eq54_e1723_d_n0, eq54_e1723_d_n1, eq54_e1723_d_n2, eq54_e1723_d_n3, eq54_e1723_d_n4, eq54_e1723_d_n5, eq54_e1723_d_n6, eq54_e1723_d_n7, eq54_e1723_d_n8, eq54_e1723_d_n9, eq54_e1723_d_n10, eq54_e1723_d_n11, eq54_e1723_d_n12, eq54_e1723_d_n13];
        let eq54_branch_derivatives: [f64; 18] = [eq54_e1723_d_b0, eq54_e1723_d_b1, eq54_e1723_d_b2, eq54_e1723_d_b3, eq54_e1723_d_b4, eq54_e1723_d_b5, eq54_e1723_d_b6, eq54_e1723_d_b7, eq54_e1723_d_b8, eq54_e1723_d_b9, eq54_e1723_d_b10, eq54_e1723_d_b11, eq54_e1723_d_b12, eq54_e1723_d_b13, eq54_e1723_d_b14, eq54_e1723_d_b15, eq54_e1723_d_b16, eq54_e1723_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq54_value),
            &eq54_node_derivatives,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1733, eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13, eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17,) = {
    if (!s.b[1553]) {
        let eq55_e1728: f64 = (p.p33 * (nv9 - nv3));
        let eq55_e1730: f64 = (eq55_e1728 * s.v[336]);
        let eq55_e1730_d_n0: f64 = (eq55_e1728 * s.dn[336][0]);
        let eq55_e1730_d_n1: f64 = (eq55_e1728 * s.dn[336][1]);
        let eq55_e1730_d_n2: f64 = (eq55_e1728 * s.dn[336][2]);
        let eq55_e1730_d_n3: f64 = (((-p.p33) * s.v[336]) + (eq55_e1728 * s.dn[336][3]));
        let eq55_e1730_d_n4: f64 = (eq55_e1728 * s.dn[336][4]);
        let eq55_e1730_d_n5: f64 = (eq55_e1728 * s.dn[336][5]);
        let eq55_e1730_d_n6: f64 = (eq55_e1728 * s.dn[336][6]);
        let eq55_e1730_d_n7: f64 = (eq55_e1728 * s.dn[336][7]);
        let eq55_e1730_d_n8: f64 = (eq55_e1728 * s.dn[336][8]);
        let eq55_e1730_d_n9: f64 = ((p.p33 * s.v[336]) + (eq55_e1728 * s.dn[336][9]));
        let eq55_e1730_d_n10: f64 = (eq55_e1728 * s.dn[336][10]);
        let eq55_e1730_d_n11: f64 = (eq55_e1728 * s.dn[336][11]);
        let eq55_e1730_d_n12: f64 = (eq55_e1728 * s.dn[336][12]);
        let eq55_e1730_d_n13: f64 = (eq55_e1728 * s.dn[336][13]);
        let eq55_e1730_d_b0: f64 = (eq55_e1728 * s.db[336][0]);
        let eq55_e1730_d_b1: f64 = (eq55_e1728 * s.db[336][1]);
        let eq55_e1730_d_b2: f64 = (eq55_e1728 * s.db[336][2]);
        let eq55_e1730_d_b3: f64 = (eq55_e1728 * s.db[336][3]);
        let eq55_e1730_d_b4: f64 = (eq55_e1728 * s.db[336][4]);
        let eq55_e1730_d_b5: f64 = (eq55_e1728 * s.db[336][5]);
        let eq55_e1730_d_b6: f64 = (eq55_e1728 * s.db[336][6]);
        let eq55_e1730_d_b7: f64 = (eq55_e1728 * s.db[336][7]);
        let eq55_e1730_d_b8: f64 = (eq55_e1728 * s.db[336][8]);
        let eq55_e1730_d_b9: f64 = (eq55_e1728 * s.db[336][9]);
        let eq55_e1730_d_b10: f64 = (eq55_e1728 * s.db[336][10]);
        let eq55_e1730_d_b11: f64 = (eq55_e1728 * s.db[336][11]);
        let eq55_e1730_d_b12: f64 = (eq55_e1728 * s.db[336][12]);
        let eq55_e1730_d_b13: f64 = (eq55_e1728 * s.db[336][13]);
        let eq55_e1730_d_b14: f64 = (eq55_e1728 * s.db[336][14]);
        let eq55_e1730_d_b15: f64 = (eq55_e1728 * s.db[336][15]);
        let eq55_e1730_d_b16: f64 = (eq55_e1728 * s.db[336][16]);
        let eq55_e1730_d_b17: f64 = (eq55_e1728 * s.db[336][17]);
        let eq55_e1731: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 13, eq55_e1730);
        (eq55_e1731, (eq55_e1730_d_n0 * ddt_scale), (eq55_e1730_d_n1 * ddt_scale), (eq55_e1730_d_n2 * ddt_scale), (eq55_e1730_d_n3 * ddt_scale), (eq55_e1730_d_n4 * ddt_scale), (eq55_e1730_d_n5 * ddt_scale), (eq55_e1730_d_n6 * ddt_scale), (eq55_e1730_d_n7 * ddt_scale), (eq55_e1730_d_n8 * ddt_scale), (eq55_e1730_d_n9 * ddt_scale), (eq55_e1730_d_n10 * ddt_scale), (eq55_e1730_d_n11 * ddt_scale), (eq55_e1730_d_n12 * ddt_scale), (eq55_e1730_d_n13 * ddt_scale), (eq55_e1730_d_b0 * ddt_scale), (eq55_e1730_d_b1 * ddt_scale), (eq55_e1730_d_b2 * ddt_scale), (eq55_e1730_d_b3 * ddt_scale), (eq55_e1730_d_b4 * ddt_scale), (eq55_e1730_d_b5 * ddt_scale), (eq55_e1730_d_b6 * ddt_scale), (eq55_e1730_d_b7 * ddt_scale), (eq55_e1730_d_b8 * ddt_scale), (eq55_e1730_d_b9 * ddt_scale), (eq55_e1730_d_b10 * ddt_scale), (eq55_e1730_d_b11 * ddt_scale), (eq55_e1730_d_b12 * ddt_scale), (eq55_e1730_d_b13 * ddt_scale), (eq55_e1730_d_b14 * ddt_scale), (eq55_e1730_d_b15 * ddt_scale), (eq55_e1730_d_b16 * ddt_scale), (eq55_e1730_d_b17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1733;
        let eq55_node_derivatives: [f64; 14] = [eq55_e1733_d_n0, eq55_e1733_d_n1, eq55_e1733_d_n2, eq55_e1733_d_n3, eq55_e1733_d_n4, eq55_e1733_d_n5, eq55_e1733_d_n6, eq55_e1733_d_n7, eq55_e1733_d_n8, eq55_e1733_d_n9, eq55_e1733_d_n10, eq55_e1733_d_n11, eq55_e1733_d_n12, eq55_e1733_d_n13];
        let eq55_branch_derivatives: [f64; 18] = [eq55_e1733_d_b0, eq55_e1733_d_b1, eq55_e1733_d_b2, eq55_e1733_d_b3, eq55_e1733_d_b4, eq55_e1733_d_b5, eq55_e1733_d_b6, eq55_e1733_d_b7, eq55_e1733_d_b8, eq55_e1733_d_b9, eq55_e1733_d_b10, eq55_e1733_d_b11, eq55_e1733_d_b12, eq55_e1733_d_b13, eq55_e1733_d_b14, eq55_e1733_d_b15, eq55_e1733_d_b16, eq55_e1733_d_b17];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(3),
            multiplicity * (eq55_value),
            &eq55_node_derivatives,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let eq56_e1736: f64 = (p.p33 * s.v[87]);
        let eq56_e1736_d_n0: f64 = (p.p33 * s.dn[87][0]);
        let eq56_e1736_d_n1: f64 = (p.p33 * s.dn[87][1]);
        let eq56_e1736_d_n2: f64 = (p.p33 * s.dn[87][2]);
        let eq56_e1736_d_n3: f64 = (p.p33 * s.dn[87][3]);
        let eq56_e1736_d_n4: f64 = (p.p33 * s.dn[87][4]);
        let eq56_e1736_d_n5: f64 = (p.p33 * s.dn[87][5]);
        let eq56_e1736_d_n6: f64 = (p.p33 * s.dn[87][6]);
        let eq56_e1736_d_n7: f64 = (p.p33 * s.dn[87][7]);
        let eq56_e1736_d_n8: f64 = (p.p33 * s.dn[87][8]);
        let eq56_e1736_d_n9: f64 = (p.p33 * s.dn[87][9]);
        let eq56_e1736_d_n10: f64 = (p.p33 * s.dn[87][10]);
        let eq56_e1736_d_n11: f64 = (p.p33 * s.dn[87][11]);
        let eq56_e1736_d_n12: f64 = (p.p33 * s.dn[87][12]);
        let eq56_e1736_d_n13: f64 = (p.p33 * s.dn[87][13]);
        let eq56_e1736_d_b0: f64 = (p.p33 * s.db[87][0]);
        let eq56_e1736_d_b1: f64 = (p.p33 * s.db[87][1]);
        let eq56_e1736_d_b2: f64 = (p.p33 * s.db[87][2]);
        let eq56_e1736_d_b3: f64 = (p.p33 * s.db[87][3]);
        let eq56_e1736_d_b4: f64 = (p.p33 * s.db[87][4]);
        let eq56_e1736_d_b5: f64 = (p.p33 * s.db[87][5]);
        let eq56_e1736_d_b6: f64 = (p.p33 * s.db[87][6]);
        let eq56_e1736_d_b7: f64 = (p.p33 * s.db[87][7]);
        let eq56_e1736_d_b8: f64 = (p.p33 * s.db[87][8]);
        let eq56_e1736_d_b9: f64 = (p.p33 * s.db[87][9]);
        let eq56_e1736_d_b10: f64 = (p.p33 * s.db[87][10]);
        let eq56_e1736_d_b11: f64 = (p.p33 * s.db[87][11]);
        let eq56_e1736_d_b12: f64 = (p.p33 * s.db[87][12]);
        let eq56_e1736_d_b13: f64 = (p.p33 * s.db[87][13]);
        let eq56_e1736_d_b14: f64 = (p.p33 * s.db[87][14]);
        let eq56_e1736_d_b15: f64 = (p.p33 * s.db[87][15]);
        let eq56_e1736_d_b16: f64 = (p.p33 * s.db[87][16]);
        let eq56_e1736_d_b17: f64 = (p.p33 * s.db[87][17]);
        let eq56_e1737: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 14, eq56_e1736);
        let eq56_value: f64 = eq56_e1737;
        let eq56_node_derivatives: [f64; 14] = [(eq56_e1736_d_n0 * ddt_scale), (eq56_e1736_d_n1 * ddt_scale), (eq56_e1736_d_n2 * ddt_scale), (eq56_e1736_d_n3 * ddt_scale), (eq56_e1736_d_n4 * ddt_scale), (eq56_e1736_d_n5 * ddt_scale), (eq56_e1736_d_n6 * ddt_scale), (eq56_e1736_d_n7 * ddt_scale), (eq56_e1736_d_n8 * ddt_scale), (eq56_e1736_d_n9 * ddt_scale), (eq56_e1736_d_n10 * ddt_scale), (eq56_e1736_d_n11 * ddt_scale), (eq56_e1736_d_n12 * ddt_scale), (eq56_e1736_d_n13 * ddt_scale)];
        let eq56_branch_derivatives: [f64; 18] = [(eq56_e1736_d_b0 * ddt_scale), (eq56_e1736_d_b1 * ddt_scale), (eq56_e1736_d_b2 * ddt_scale), (eq56_e1736_d_b3 * ddt_scale), (eq56_e1736_d_b4 * ddt_scale), (eq56_e1736_d_b5 * ddt_scale), (eq56_e1736_d_b6 * ddt_scale), (eq56_e1736_d_b7 * ddt_scale), (eq56_e1736_d_b8 * ddt_scale), (eq56_e1736_d_b9 * ddt_scale), (eq56_e1736_d_b10 * ddt_scale), (eq56_e1736_d_b11 * ddt_scale), (eq56_e1736_d_b12 * ddt_scale), (eq56_e1736_d_b13 * ddt_scale), (eq56_e1736_d_b14 * ddt_scale), (eq56_e1736_d_b15 * ddt_scale), (eq56_e1736_d_b16 * ddt_scale), (eq56_e1736_d_b17 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(3),
            multiplicity * (eq56_value),
            &eq56_node_derivatives,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let eq57_e1740: f64 = (p.p33 * s.v[86]);
        let eq57_e1740_d_n0: f64 = (p.p33 * s.dn[86][0]);
        let eq57_e1740_d_n1: f64 = (p.p33 * s.dn[86][1]);
        let eq57_e1740_d_n2: f64 = (p.p33 * s.dn[86][2]);
        let eq57_e1740_d_n3: f64 = (p.p33 * s.dn[86][3]);
        let eq57_e1740_d_n4: f64 = (p.p33 * s.dn[86][4]);
        let eq57_e1740_d_n5: f64 = (p.p33 * s.dn[86][5]);
        let eq57_e1740_d_n6: f64 = (p.p33 * s.dn[86][6]);
        let eq57_e1740_d_n7: f64 = (p.p33 * s.dn[86][7]);
        let eq57_e1740_d_n8: f64 = (p.p33 * s.dn[86][8]);
        let eq57_e1740_d_n9: f64 = (p.p33 * s.dn[86][9]);
        let eq57_e1740_d_n10: f64 = (p.p33 * s.dn[86][10]);
        let eq57_e1740_d_n11: f64 = (p.p33 * s.dn[86][11]);
        let eq57_e1740_d_n12: f64 = (p.p33 * s.dn[86][12]);
        let eq57_e1740_d_n13: f64 = (p.p33 * s.dn[86][13]);
        let eq57_e1740_d_b0: f64 = (p.p33 * s.db[86][0]);
        let eq57_e1740_d_b1: f64 = (p.p33 * s.db[86][1]);
        let eq57_e1740_d_b2: f64 = (p.p33 * s.db[86][2]);
        let eq57_e1740_d_b3: f64 = (p.p33 * s.db[86][3]);
        let eq57_e1740_d_b4: f64 = (p.p33 * s.db[86][4]);
        let eq57_e1740_d_b5: f64 = (p.p33 * s.db[86][5]);
        let eq57_e1740_d_b6: f64 = (p.p33 * s.db[86][6]);
        let eq57_e1740_d_b7: f64 = (p.p33 * s.db[86][7]);
        let eq57_e1740_d_b8: f64 = (p.p33 * s.db[86][8]);
        let eq57_e1740_d_b9: f64 = (p.p33 * s.db[86][9]);
        let eq57_e1740_d_b10: f64 = (p.p33 * s.db[86][10]);
        let eq57_e1740_d_b11: f64 = (p.p33 * s.db[86][11]);
        let eq57_e1740_d_b12: f64 = (p.p33 * s.db[86][12]);
        let eq57_e1740_d_b13: f64 = (p.p33 * s.db[86][13]);
        let eq57_e1740_d_b14: f64 = (p.p33 * s.db[86][14]);
        let eq57_e1740_d_b15: f64 = (p.p33 * s.db[86][15]);
        let eq57_e1740_d_b16: f64 = (p.p33 * s.db[86][16]);
        let eq57_e1740_d_b17: f64 = (p.p33 * s.db[86][17]);
        let eq57_e1741: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 15, eq57_e1740);
        let eq57_value: f64 = eq57_e1741;
        let eq57_node_derivatives: [f64; 14] = [(eq57_e1740_d_n0 * ddt_scale), (eq57_e1740_d_n1 * ddt_scale), (eq57_e1740_d_n2 * ddt_scale), (eq57_e1740_d_n3 * ddt_scale), (eq57_e1740_d_n4 * ddt_scale), (eq57_e1740_d_n5 * ddt_scale), (eq57_e1740_d_n6 * ddt_scale), (eq57_e1740_d_n7 * ddt_scale), (eq57_e1740_d_n8 * ddt_scale), (eq57_e1740_d_n9 * ddt_scale), (eq57_e1740_d_n10 * ddt_scale), (eq57_e1740_d_n11 * ddt_scale), (eq57_e1740_d_n12 * ddt_scale), (eq57_e1740_d_n13 * ddt_scale)];
        let eq57_branch_derivatives: [f64; 18] = [(eq57_e1740_d_b0 * ddt_scale), (eq57_e1740_d_b1 * ddt_scale), (eq57_e1740_d_b2 * ddt_scale), (eq57_e1740_d_b3 * ddt_scale), (eq57_e1740_d_b4 * ddt_scale), (eq57_e1740_d_b5 * ddt_scale), (eq57_e1740_d_b6 * ddt_scale), (eq57_e1740_d_b7 * ddt_scale), (eq57_e1740_d_b8 * ddt_scale), (eq57_e1740_d_b9 * ddt_scale), (eq57_e1740_d_b10 * ddt_scale), (eq57_e1740_d_b11 * ddt_scale), (eq57_e1740_d_b12 * ddt_scale), (eq57_e1740_d_b13 * ddt_scale), (eq57_e1740_d_b14 * ddt_scale), (eq57_e1740_d_b15 * ddt_scale), (eq57_e1740_d_b16 * ddt_scale), (eq57_e1740_d_b17 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(3),
            multiplicity * (eq57_value),
            &eq57_node_derivatives,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1779, eq62_e1779_d_n0, eq62_e1779_d_n1, eq62_e1779_d_n2, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12, eq62_e1779_d_n13, eq62_e1779_d_b0, eq62_e1779_d_b1, eq62_e1779_d_b2, eq62_e1779_d_b3, eq62_e1779_d_b4, eq62_e1779_d_b5, eq62_e1779_d_b6, eq62_e1779_d_b7, eq62_e1779_d_b8, eq62_e1779_d_b9, eq62_e1779_d_b10, eq62_e1779_d_b11, eq62_e1779_d_b12, eq62_e1779_d_b13, eq62_e1779_d_b14, eq62_e1779_d_b15, eq62_e1779_d_b16, eq62_e1779_d_b17,) = {
    if (!s.b[1555]) {
        let eq62_e1775: f64 = (p.p32 * (nv10 - nv9));
        let eq62_e1777: f64 = (eq62_e1775 * s.v[81]);
        let eq62_e1777_d_n0: f64 = (eq62_e1775 * s.dn[81][0]);
        let eq62_e1777_d_n1: f64 = (eq62_e1775 * s.dn[81][1]);
        let eq62_e1777_d_n2: f64 = (eq62_e1775 * s.dn[81][2]);
        let eq62_e1777_d_n3: f64 = (eq62_e1775 * s.dn[81][3]);
        let eq62_e1777_d_n4: f64 = (eq62_e1775 * s.dn[81][4]);
        let eq62_e1777_d_n5: f64 = (eq62_e1775 * s.dn[81][5]);
        let eq62_e1777_d_n6: f64 = (eq62_e1775 * s.dn[81][6]);
        let eq62_e1777_d_n7: f64 = (eq62_e1775 * s.dn[81][7]);
        let eq62_e1777_d_n8: f64 = (eq62_e1775 * s.dn[81][8]);
        let eq62_e1777_d_n9: f64 = (((-p.p32) * s.v[81]) + (eq62_e1775 * s.dn[81][9]));
        let eq62_e1777_d_n10: f64 = ((p.p32 * s.v[81]) + (eq62_e1775 * s.dn[81][10]));
        let eq62_e1777_d_n11: f64 = (eq62_e1775 * s.dn[81][11]);
        let eq62_e1777_d_n12: f64 = (eq62_e1775 * s.dn[81][12]);
        let eq62_e1777_d_n13: f64 = (eq62_e1775 * s.dn[81][13]);
        let eq62_e1777_d_b0: f64 = (eq62_e1775 * s.db[81][0]);
        let eq62_e1777_d_b1: f64 = (eq62_e1775 * s.db[81][1]);
        let eq62_e1777_d_b2: f64 = (eq62_e1775 * s.db[81][2]);
        let eq62_e1777_d_b3: f64 = (eq62_e1775 * s.db[81][3]);
        let eq62_e1777_d_b4: f64 = (eq62_e1775 * s.db[81][4]);
        let eq62_e1777_d_b5: f64 = (eq62_e1775 * s.db[81][5]);
        let eq62_e1777_d_b6: f64 = (eq62_e1775 * s.db[81][6]);
        let eq62_e1777_d_b7: f64 = (eq62_e1775 * s.db[81][7]);
        let eq62_e1777_d_b8: f64 = (eq62_e1775 * s.db[81][8]);
        let eq62_e1777_d_b9: f64 = (eq62_e1775 * s.db[81][9]);
        let eq62_e1777_d_b10: f64 = (eq62_e1775 * s.db[81][10]);
        let eq62_e1777_d_b11: f64 = (eq62_e1775 * s.db[81][11]);
        let eq62_e1777_d_b12: f64 = (eq62_e1775 * s.db[81][12]);
        let eq62_e1777_d_b13: f64 = (eq62_e1775 * s.db[81][13]);
        let eq62_e1777_d_b14: f64 = (eq62_e1775 * s.db[81][14]);
        let eq62_e1777_d_b15: f64 = (eq62_e1775 * s.db[81][15]);
        let eq62_e1777_d_b16: f64 = (eq62_e1775 * s.db[81][16]);
        let eq62_e1777_d_b17: f64 = (eq62_e1775 * s.db[81][17]);
        (eq62_e1777, eq62_e1777_d_n0, eq62_e1777_d_n1, eq62_e1777_d_n2, eq62_e1777_d_n3, eq62_e1777_d_n4, eq62_e1777_d_n5, eq62_e1777_d_n6, eq62_e1777_d_n7, eq62_e1777_d_n8, eq62_e1777_d_n9, eq62_e1777_d_n10, eq62_e1777_d_n11, eq62_e1777_d_n12, eq62_e1777_d_n13, eq62_e1777_d_b0, eq62_e1777_d_b1, eq62_e1777_d_b2, eq62_e1777_d_b3, eq62_e1777_d_b4, eq62_e1777_d_b5, eq62_e1777_d_b6, eq62_e1777_d_b7, eq62_e1777_d_b8, eq62_e1777_d_b9, eq62_e1777_d_b10, eq62_e1777_d_b11, eq62_e1777_d_b12, eq62_e1777_d_b13, eq62_e1777_d_b14, eq62_e1777_d_b15, eq62_e1777_d_b16, eq62_e1777_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1779;
        let eq62_node_derivatives: [f64; 14] = [eq62_e1779_d_n0, eq62_e1779_d_n1, eq62_e1779_d_n2, eq62_e1779_d_n3, eq62_e1779_d_n4, eq62_e1779_d_n5, eq62_e1779_d_n6, eq62_e1779_d_n7, eq62_e1779_d_n8, eq62_e1779_d_n9, eq62_e1779_d_n10, eq62_e1779_d_n11, eq62_e1779_d_n12, eq62_e1779_d_n13];
        let eq62_branch_derivatives: [f64; 18] = [eq62_e1779_d_b0, eq62_e1779_d_b1, eq62_e1779_d_b2, eq62_e1779_d_b3, eq62_e1779_d_b4, eq62_e1779_d_b5, eq62_e1779_d_b6, eq62_e1779_d_b7, eq62_e1779_d_b8, eq62_e1779_d_b9, eq62_e1779_d_b10, eq62_e1779_d_b11, eq62_e1779_d_b12, eq62_e1779_d_b13, eq62_e1779_d_b14, eq62_e1779_d_b15, eq62_e1779_d_b16, eq62_e1779_d_b17];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq62_value),
            &eq62_node_derivatives,
            &eq62_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (s.dn[410][0] * s.v[158]);
        let __rspice_deriv_cse_1: f64 = (s.dn[410][1] * s.v[158]);
        let __rspice_deriv_cse_2: f64 = (s.dn[410][2] * s.v[158]);
        let __rspice_deriv_cse_3: f64 = (s.dn[410][3] * s.v[158]);
        let __rspice_deriv_cse_4: f64 = (s.dn[410][4] * s.v[158]);
        let __rspice_deriv_cse_5: f64 = (s.dn[410][5] * s.v[158]);
        let __rspice_deriv_cse_6: f64 = (s.dn[410][6] * s.v[158]);
        let __rspice_deriv_cse_7: f64 = (s.dn[410][7] * s.v[158]);
        let __rspice_deriv_cse_8: f64 = (s.dn[410][8] * s.v[158]);
        let __rspice_deriv_cse_9: f64 = (s.dn[410][9] * s.v[158]);
        let __rspice_deriv_cse_10: f64 = (s.dn[410][10] * s.v[158]);
        let __rspice_deriv_cse_11: f64 = (s.dn[410][11] * s.v[158]);
        let __rspice_deriv_cse_12: f64 = (s.dn[410][12] * s.v[158]);
        let __rspice_deriv_cse_13: f64 = (s.dn[410][13] * s.v[158]);
        let __rspice_deriv_cse_14: f64 = (s.db[410][0] * s.v[158]);
        let __rspice_deriv_cse_15: f64 = (s.db[410][1] * s.v[158]);
        let __rspice_deriv_cse_16: f64 = (s.db[410][2] * s.v[158]);
        let __rspice_deriv_cse_17: f64 = (s.db[410][3] * s.v[158]);
        let __rspice_deriv_cse_18: f64 = (s.db[410][4] * s.v[158]);
        let __rspice_deriv_cse_19: f64 = (s.db[410][5] * s.v[158]);
        let __rspice_deriv_cse_20: f64 = (s.db[410][6] * s.v[158]);
        let __rspice_deriv_cse_21: f64 = (s.db[410][7] * s.v[158]);
        let __rspice_deriv_cse_22: f64 = (s.db[410][8] * s.v[158]);
        let __rspice_deriv_cse_23: f64 = (s.db[410][9] * s.v[158]);
        let __rspice_deriv_cse_24: f64 = (s.db[410][10] * s.v[158]);
        let __rspice_deriv_cse_25: f64 = (s.db[410][11] * s.v[158]);
        let __rspice_deriv_cse_26: f64 = (s.db[410][12] * s.v[158]);
        let __rspice_deriv_cse_27: f64 = (s.db[410][13] * s.v[158]);
        let __rspice_deriv_cse_28: f64 = (s.db[410][14] * s.v[158]);
        let __rspice_deriv_cse_29: f64 = (s.db[410][15] * s.v[158]);
        let __rspice_deriv_cse_30: f64 = (s.db[410][16] * s.v[158]);
        let __rspice_deriv_cse_31: f64 = (s.db[410][17] * s.v[158]);
        let (eq71_e1869, eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13, eq71_e1869_d_b0, eq71_e1869_d_b1, eq71_e1869_d_b2, eq71_e1869_d_b3, eq71_e1869_d_b4, eq71_e1869_d_b5, eq71_e1869_d_b6, eq71_e1869_d_b7, eq71_e1869_d_b8, eq71_e1869_d_b9, eq71_e1869_d_b10, eq71_e1869_d_b11, eq71_e1869_d_b12, eq71_e1869_d_b13, eq71_e1869_d_b14, eq71_e1869_d_b15, eq71_e1869_d_b16, eq71_e1869_d_b17,) = {
    if ((s.b[1559] && s.b[1560]) && s.b[1561]) {
        let eq71_e1856: f64 = (-s.v[885]);
        let eq71_e1858: f64 = (eq71_e1856 * s.v[822]);
        let eq71_e1858_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq71_e1856 * s.dn[822][0]));
        let eq71_e1858_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq71_e1856 * s.dn[822][1]));
        let eq71_e1858_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq71_e1856 * s.dn[822][2]));
        let eq71_e1858_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq71_e1856 * s.dn[822][3]));
        let eq71_e1858_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq71_e1856 * s.dn[822][4]));
        let eq71_e1858_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq71_e1856 * s.dn[822][5]));
        let eq71_e1858_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq71_e1856 * s.dn[822][6]));
        let eq71_e1858_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq71_e1856 * s.dn[822][7]));
        let eq71_e1858_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq71_e1856 * s.dn[822][8]));
        let eq71_e1858_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq71_e1856 * s.dn[822][9]));
        let eq71_e1858_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq71_e1856 * s.dn[822][10]));
        let eq71_e1858_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq71_e1856 * s.dn[822][11]));
        let eq71_e1858_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq71_e1856 * s.dn[822][12]));
        let eq71_e1858_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq71_e1856 * s.dn[822][13]));
        let eq71_e1858_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq71_e1856 * s.db[822][0]));
        let eq71_e1858_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq71_e1856 * s.db[822][1]));
        let eq71_e1858_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq71_e1856 * s.db[822][2]));
        let eq71_e1858_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq71_e1856 * s.db[822][3]));
        let eq71_e1858_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq71_e1856 * s.db[822][4]));
        let eq71_e1858_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq71_e1856 * s.db[822][5]));
        let eq71_e1858_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq71_e1856 * s.db[822][6]));
        let eq71_e1858_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq71_e1856 * s.db[822][7]));
        let eq71_e1858_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq71_e1856 * s.db[822][8]));
        let eq71_e1858_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq71_e1856 * s.db[822][9]));
        let eq71_e1858_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq71_e1856 * s.db[822][10]));
        let eq71_e1858_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq71_e1856 * s.db[822][11]));
        let eq71_e1858_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq71_e1856 * s.db[822][12]));
        let eq71_e1858_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq71_e1856 * s.db[822][13]));
        let eq71_e1858_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq71_e1856 * s.db[822][14]));
        let eq71_e1858_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq71_e1856 * s.db[822][15]));
        let eq71_e1858_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq71_e1856 * s.db[822][16]));
        let eq71_e1858_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq71_e1856 * s.db[822][17]));
        let eq71_e1861: f64 = (s.v[410] * s.v[158]);
        let eq71_e1862: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 16, eq71_e1861);
        let eq71_e1863: f64 = (eq71_e1858 + eq71_e1862);
        let eq71_e1863_d_n0: f64 = (eq71_e1858_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq71_e1863_d_n1: f64 = (eq71_e1858_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq71_e1863_d_n2: f64 = (eq71_e1858_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq71_e1863_d_n3: f64 = (eq71_e1858_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq71_e1863_d_n4: f64 = (eq71_e1858_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq71_e1863_d_n5: f64 = (eq71_e1858_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq71_e1863_d_n6: f64 = (eq71_e1858_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq71_e1863_d_n7: f64 = (eq71_e1858_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq71_e1863_d_n8: f64 = (eq71_e1858_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq71_e1863_d_n9: f64 = (eq71_e1858_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq71_e1863_d_n10: f64 = (eq71_e1858_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq71_e1863_d_n11: f64 = (eq71_e1858_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq71_e1863_d_n12: f64 = (eq71_e1858_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq71_e1863_d_n13: f64 = (eq71_e1858_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq71_e1863_d_b0: f64 = (eq71_e1858_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq71_e1863_d_b1: f64 = (eq71_e1858_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq71_e1863_d_b2: f64 = (eq71_e1858_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq71_e1863_d_b3: f64 = (eq71_e1858_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq71_e1863_d_b4: f64 = (eq71_e1858_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq71_e1863_d_b5: f64 = (eq71_e1858_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq71_e1863_d_b6: f64 = (eq71_e1858_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq71_e1863_d_b7: f64 = (eq71_e1858_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq71_e1863_d_b8: f64 = (eq71_e1858_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq71_e1863_d_b9: f64 = (eq71_e1858_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq71_e1863_d_b10: f64 = (eq71_e1858_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq71_e1863_d_b11: f64 = (eq71_e1858_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq71_e1863_d_b12: f64 = (eq71_e1858_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq71_e1863_d_b13: f64 = (eq71_e1858_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq71_e1863_d_b14: f64 = (eq71_e1858_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq71_e1863_d_b15: f64 = (eq71_e1858_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq71_e1863_d_b16: f64 = (eq71_e1858_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq71_e1863_d_b17: f64 = (eq71_e1858_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_0: f64 = 1.0 / s.v[157];
        let eq71_e1866: f64 = (s.v[410] * __rspice_inv_cse_0);
        let eq71_e1866_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_0);
        let eq71_e1866_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_0);
        let eq71_e1866_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_0);
        let eq71_e1866_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_0);
        let eq71_e1866_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_0);
        let eq71_e1866_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_0);
        let eq71_e1866_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_0);
        let eq71_e1866_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_0);
        let eq71_e1866_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_0);
        let eq71_e1866_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_0);
        let eq71_e1866_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_0);
        let eq71_e1866_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_0);
        let eq71_e1866_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_0);
        let eq71_e1866_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_0);
        let eq71_e1866_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_0);
        let eq71_e1866_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_0);
        let eq71_e1866_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_0);
        let eq71_e1866_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_0);
        let eq71_e1867: f64 = (eq71_e1863 + eq71_e1866);
        let eq71_e1867_d_n0: f64 = (eq71_e1863_d_n0 + eq71_e1866_d_n0);
        let eq71_e1867_d_n1: f64 = (eq71_e1863_d_n1 + eq71_e1866_d_n1);
        let eq71_e1867_d_n2: f64 = (eq71_e1863_d_n2 + eq71_e1866_d_n2);
        let eq71_e1867_d_n3: f64 = (eq71_e1863_d_n3 + eq71_e1866_d_n3);
        let eq71_e1867_d_n4: f64 = (eq71_e1863_d_n4 + eq71_e1866_d_n4);
        let eq71_e1867_d_n5: f64 = (eq71_e1863_d_n5 + eq71_e1866_d_n5);
        let eq71_e1867_d_n6: f64 = (eq71_e1863_d_n6 + eq71_e1866_d_n6);
        let eq71_e1867_d_n7: f64 = (eq71_e1863_d_n7 + eq71_e1866_d_n7);
        let eq71_e1867_d_n8: f64 = (eq71_e1863_d_n8 + eq71_e1866_d_n8);
        let eq71_e1867_d_n9: f64 = (eq71_e1863_d_n9 + eq71_e1866_d_n9);
        let eq71_e1867_d_n10: f64 = (eq71_e1863_d_n10 + eq71_e1866_d_n10);
        let eq71_e1867_d_n11: f64 = (eq71_e1863_d_n11 + eq71_e1866_d_n11);
        let eq71_e1867_d_n12: f64 = (eq71_e1863_d_n12 + eq71_e1866_d_n12);
        let eq71_e1867_d_n13: f64 = (eq71_e1863_d_n13 + eq71_e1866_d_n13);
        let eq71_e1867_d_b0: f64 = (eq71_e1863_d_b0 + eq71_e1866_d_b0);
        let eq71_e1867_d_b1: f64 = (eq71_e1863_d_b1 + eq71_e1866_d_b1);
        let eq71_e1867_d_b2: f64 = (eq71_e1863_d_b2 + eq71_e1866_d_b2);
        let eq71_e1867_d_b3: f64 = (eq71_e1863_d_b3 + eq71_e1866_d_b3);
        let eq71_e1867_d_b4: f64 = (eq71_e1863_d_b4 + eq71_e1866_d_b4);
        let eq71_e1867_d_b5: f64 = (eq71_e1863_d_b5 + eq71_e1866_d_b5);
        let eq71_e1867_d_b6: f64 = (eq71_e1863_d_b6 + eq71_e1866_d_b6);
        let eq71_e1867_d_b7: f64 = (eq71_e1863_d_b7 + eq71_e1866_d_b7);
        let eq71_e1867_d_b8: f64 = (eq71_e1863_d_b8 + eq71_e1866_d_b8);
        let eq71_e1867_d_b9: f64 = (eq71_e1863_d_b9 + eq71_e1866_d_b9);
        let eq71_e1867_d_b10: f64 = (eq71_e1863_d_b10 + eq71_e1866_d_b10);
        let eq71_e1867_d_b11: f64 = (eq71_e1863_d_b11 + eq71_e1866_d_b11);
        let eq71_e1867_d_b12: f64 = (eq71_e1863_d_b12 + eq71_e1866_d_b12);
        let eq71_e1867_d_b13: f64 = (eq71_e1863_d_b13 + eq71_e1866_d_b13);
        let eq71_e1867_d_b14: f64 = (eq71_e1863_d_b14 + eq71_e1866_d_b14);
        let eq71_e1867_d_b15: f64 = (eq71_e1863_d_b15 + eq71_e1866_d_b15);
        let eq71_e1867_d_b16: f64 = (eq71_e1863_d_b16 + eq71_e1866_d_b16);
        let eq71_e1867_d_b17: f64 = (eq71_e1863_d_b17 + eq71_e1866_d_b17);
        (eq71_e1867, eq71_e1867_d_n0, eq71_e1867_d_n1, eq71_e1867_d_n2, eq71_e1867_d_n3, eq71_e1867_d_n4, eq71_e1867_d_n5, eq71_e1867_d_n6, eq71_e1867_d_n7, eq71_e1867_d_n8, eq71_e1867_d_n9, eq71_e1867_d_n10, eq71_e1867_d_n11, eq71_e1867_d_n12, eq71_e1867_d_n13, eq71_e1867_d_b0, eq71_e1867_d_b1, eq71_e1867_d_b2, eq71_e1867_d_b3, eq71_e1867_d_b4, eq71_e1867_d_b5, eq71_e1867_d_b6, eq71_e1867_d_b7, eq71_e1867_d_b8, eq71_e1867_d_b9, eq71_e1867_d_b10, eq71_e1867_d_b11, eq71_e1867_d_b12, eq71_e1867_d_b13, eq71_e1867_d_b14, eq71_e1867_d_b15, eq71_e1867_d_b16, eq71_e1867_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1869;
        let eq71_node_derivatives: [f64; 14] = [eq71_e1869_d_n0, eq71_e1869_d_n1, eq71_e1869_d_n2, eq71_e1869_d_n3, eq71_e1869_d_n4, eq71_e1869_d_n5, eq71_e1869_d_n6, eq71_e1869_d_n7, eq71_e1869_d_n8, eq71_e1869_d_n9, eq71_e1869_d_n10, eq71_e1869_d_n11, eq71_e1869_d_n12, eq71_e1869_d_n13];
        let eq71_branch_derivatives: [f64; 18] = [eq71_e1869_d_b0, eq71_e1869_d_b1, eq71_e1869_d_b2, eq71_e1869_d_b3, eq71_e1869_d_b4, eq71_e1869_d_b5, eq71_e1869_d_b6, eq71_e1869_d_b7, eq71_e1869_d_b8, eq71_e1869_d_b9, eq71_e1869_d_b10, eq71_e1869_d_b11, eq71_e1869_d_b12, eq71_e1869_d_b13, eq71_e1869_d_b14, eq71_e1869_d_b15, eq71_e1869_d_b16, eq71_e1869_d_b17];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq71_value),
            &eq71_node_derivatives,
            &eq71_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1892, eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13, eq72_e1892_d_b0, eq72_e1892_d_b1, eq72_e1892_d_b2, eq72_e1892_d_b3, eq72_e1892_d_b4, eq72_e1892_d_b5, eq72_e1892_d_b6, eq72_e1892_d_b7, eq72_e1892_d_b8, eq72_e1892_d_b9, eq72_e1892_d_b10, eq72_e1892_d_b11, eq72_e1892_d_b12, eq72_e1892_d_b13, eq72_e1892_d_b14, eq72_e1892_d_b15, eq72_e1892_d_b16, eq72_e1892_d_b17,) = {
    if (((s.b[1559] && s.b[1560]) && (!s.b[1561])) && s.b[1562]) {
        let eq72_e1879: f64 = (-s.v[885]);
        let eq72_e1881: f64 = (eq72_e1879 * s.v[822]);
        let eq72_e1881_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq72_e1879 * s.dn[822][0]));
        let eq72_e1881_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq72_e1879 * s.dn[822][1]));
        let eq72_e1881_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq72_e1879 * s.dn[822][2]));
        let eq72_e1881_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq72_e1879 * s.dn[822][3]));
        let eq72_e1881_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq72_e1879 * s.dn[822][4]));
        let eq72_e1881_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq72_e1879 * s.dn[822][5]));
        let eq72_e1881_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq72_e1879 * s.dn[822][6]));
        let eq72_e1881_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq72_e1879 * s.dn[822][7]));
        let eq72_e1881_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq72_e1879 * s.dn[822][8]));
        let eq72_e1881_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq72_e1879 * s.dn[822][9]));
        let eq72_e1881_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq72_e1879 * s.dn[822][10]));
        let eq72_e1881_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq72_e1879 * s.dn[822][11]));
        let eq72_e1881_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq72_e1879 * s.dn[822][12]));
        let eq72_e1881_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq72_e1879 * s.dn[822][13]));
        let eq72_e1881_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq72_e1879 * s.db[822][0]));
        let eq72_e1881_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq72_e1879 * s.db[822][1]));
        let eq72_e1881_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq72_e1879 * s.db[822][2]));
        let eq72_e1881_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq72_e1879 * s.db[822][3]));
        let eq72_e1881_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq72_e1879 * s.db[822][4]));
        let eq72_e1881_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq72_e1879 * s.db[822][5]));
        let eq72_e1881_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq72_e1879 * s.db[822][6]));
        let eq72_e1881_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq72_e1879 * s.db[822][7]));
        let eq72_e1881_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq72_e1879 * s.db[822][8]));
        let eq72_e1881_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq72_e1879 * s.db[822][9]));
        let eq72_e1881_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq72_e1879 * s.db[822][10]));
        let eq72_e1881_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq72_e1879 * s.db[822][11]));
        let eq72_e1881_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq72_e1879 * s.db[822][12]));
        let eq72_e1881_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq72_e1879 * s.db[822][13]));
        let eq72_e1881_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq72_e1879 * s.db[822][14]));
        let eq72_e1881_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq72_e1879 * s.db[822][15]));
        let eq72_e1881_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq72_e1879 * s.db[822][16]));
        let eq72_e1881_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq72_e1879 * s.db[822][17]));
        let eq72_e1884: f64 = (s.v[410] * s.v[158]);
        let eq72_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 17, eq72_e1884);
        let eq72_e1886: f64 = (eq72_e1881 + eq72_e1885);
        let eq72_e1886_d_n0: f64 = (eq72_e1881_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq72_e1886_d_n1: f64 = (eq72_e1881_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq72_e1886_d_n2: f64 = (eq72_e1881_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq72_e1886_d_n3: f64 = (eq72_e1881_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq72_e1886_d_n4: f64 = (eq72_e1881_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq72_e1886_d_n5: f64 = (eq72_e1881_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq72_e1886_d_n6: f64 = (eq72_e1881_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq72_e1886_d_n7: f64 = (eq72_e1881_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq72_e1886_d_n8: f64 = (eq72_e1881_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq72_e1886_d_n9: f64 = (eq72_e1881_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq72_e1886_d_n10: f64 = (eq72_e1881_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq72_e1886_d_n11: f64 = (eq72_e1881_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq72_e1886_d_n12: f64 = (eq72_e1881_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq72_e1886_d_n13: f64 = (eq72_e1881_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq72_e1886_d_b0: f64 = (eq72_e1881_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq72_e1886_d_b1: f64 = (eq72_e1881_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq72_e1886_d_b2: f64 = (eq72_e1881_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq72_e1886_d_b3: f64 = (eq72_e1881_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq72_e1886_d_b4: f64 = (eq72_e1881_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq72_e1886_d_b5: f64 = (eq72_e1881_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq72_e1886_d_b6: f64 = (eq72_e1881_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq72_e1886_d_b7: f64 = (eq72_e1881_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq72_e1886_d_b8: f64 = (eq72_e1881_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq72_e1886_d_b9: f64 = (eq72_e1881_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq72_e1886_d_b10: f64 = (eq72_e1881_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq72_e1886_d_b11: f64 = (eq72_e1881_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq72_e1886_d_b12: f64 = (eq72_e1881_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq72_e1886_d_b13: f64 = (eq72_e1881_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq72_e1886_d_b14: f64 = (eq72_e1881_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq72_e1886_d_b15: f64 = (eq72_e1881_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq72_e1886_d_b16: f64 = (eq72_e1881_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq72_e1886_d_b17: f64 = (eq72_e1881_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq72_e1889: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq72_e1889_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq72_e1889_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq72_e1889_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq72_e1889_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq72_e1889_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq72_e1889_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq72_e1889_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq72_e1889_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq72_e1889_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq72_e1889_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq72_e1889_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq72_e1889_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq72_e1889_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq72_e1889_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq72_e1889_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq72_e1889_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq72_e1889_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq72_e1889_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq72_e1890: f64 = (eq72_e1886 + eq72_e1889);
        let eq72_e1890_d_n0: f64 = (eq72_e1886_d_n0 + eq72_e1889_d_n0);
        let eq72_e1890_d_n1: f64 = (eq72_e1886_d_n1 + eq72_e1889_d_n1);
        let eq72_e1890_d_n2: f64 = (eq72_e1886_d_n2 + eq72_e1889_d_n2);
        let eq72_e1890_d_n3: f64 = (eq72_e1886_d_n3 + eq72_e1889_d_n3);
        let eq72_e1890_d_n4: f64 = (eq72_e1886_d_n4 + eq72_e1889_d_n4);
        let eq72_e1890_d_n5: f64 = (eq72_e1886_d_n5 + eq72_e1889_d_n5);
        let eq72_e1890_d_n6: f64 = (eq72_e1886_d_n6 + eq72_e1889_d_n6);
        let eq72_e1890_d_n7: f64 = (eq72_e1886_d_n7 + eq72_e1889_d_n7);
        let eq72_e1890_d_n8: f64 = (eq72_e1886_d_n8 + eq72_e1889_d_n8);
        let eq72_e1890_d_n9: f64 = (eq72_e1886_d_n9 + eq72_e1889_d_n9);
        let eq72_e1890_d_n10: f64 = (eq72_e1886_d_n10 + eq72_e1889_d_n10);
        let eq72_e1890_d_n11: f64 = (eq72_e1886_d_n11 + eq72_e1889_d_n11);
        let eq72_e1890_d_n12: f64 = (eq72_e1886_d_n12 + eq72_e1889_d_n12);
        let eq72_e1890_d_n13: f64 = (eq72_e1886_d_n13 + eq72_e1889_d_n13);
        let eq72_e1890_d_b0: f64 = (eq72_e1886_d_b0 + eq72_e1889_d_b0);
        let eq72_e1890_d_b1: f64 = (eq72_e1886_d_b1 + eq72_e1889_d_b1);
        let eq72_e1890_d_b2: f64 = (eq72_e1886_d_b2 + eq72_e1889_d_b2);
        let eq72_e1890_d_b3: f64 = (eq72_e1886_d_b3 + eq72_e1889_d_b3);
        let eq72_e1890_d_b4: f64 = (eq72_e1886_d_b4 + eq72_e1889_d_b4);
        let eq72_e1890_d_b5: f64 = (eq72_e1886_d_b5 + eq72_e1889_d_b5);
        let eq72_e1890_d_b6: f64 = (eq72_e1886_d_b6 + eq72_e1889_d_b6);
        let eq72_e1890_d_b7: f64 = (eq72_e1886_d_b7 + eq72_e1889_d_b7);
        let eq72_e1890_d_b8: f64 = (eq72_e1886_d_b8 + eq72_e1889_d_b8);
        let eq72_e1890_d_b9: f64 = (eq72_e1886_d_b9 + eq72_e1889_d_b9);
        let eq72_e1890_d_b10: f64 = (eq72_e1886_d_b10 + eq72_e1889_d_b10);
        let eq72_e1890_d_b11: f64 = (eq72_e1886_d_b11 + eq72_e1889_d_b11);
        let eq72_e1890_d_b12: f64 = (eq72_e1886_d_b12 + eq72_e1889_d_b12);
        let eq72_e1890_d_b13: f64 = (eq72_e1886_d_b13 + eq72_e1889_d_b13);
        let eq72_e1890_d_b14: f64 = (eq72_e1886_d_b14 + eq72_e1889_d_b14);
        let eq72_e1890_d_b15: f64 = (eq72_e1886_d_b15 + eq72_e1889_d_b15);
        let eq72_e1890_d_b16: f64 = (eq72_e1886_d_b16 + eq72_e1889_d_b16);
        let eq72_e1890_d_b17: f64 = (eq72_e1886_d_b17 + eq72_e1889_d_b17);
        (eq72_e1890, eq72_e1890_d_n0, eq72_e1890_d_n1, eq72_e1890_d_n2, eq72_e1890_d_n3, eq72_e1890_d_n4, eq72_e1890_d_n5, eq72_e1890_d_n6, eq72_e1890_d_n7, eq72_e1890_d_n8, eq72_e1890_d_n9, eq72_e1890_d_n10, eq72_e1890_d_n11, eq72_e1890_d_n12, eq72_e1890_d_n13, eq72_e1890_d_b0, eq72_e1890_d_b1, eq72_e1890_d_b2, eq72_e1890_d_b3, eq72_e1890_d_b4, eq72_e1890_d_b5, eq72_e1890_d_b6, eq72_e1890_d_b7, eq72_e1890_d_b8, eq72_e1890_d_b9, eq72_e1890_d_b10, eq72_e1890_d_b11, eq72_e1890_d_b12, eq72_e1890_d_b13, eq72_e1890_d_b14, eq72_e1890_d_b15, eq72_e1890_d_b16, eq72_e1890_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1892;
        let eq72_node_derivatives: [f64; 14] = [eq72_e1892_d_n0, eq72_e1892_d_n1, eq72_e1892_d_n2, eq72_e1892_d_n3, eq72_e1892_d_n4, eq72_e1892_d_n5, eq72_e1892_d_n6, eq72_e1892_d_n7, eq72_e1892_d_n8, eq72_e1892_d_n9, eq72_e1892_d_n10, eq72_e1892_d_n11, eq72_e1892_d_n12, eq72_e1892_d_n13];
        let eq72_branch_derivatives: [f64; 18] = [eq72_e1892_d_b0, eq72_e1892_d_b1, eq72_e1892_d_b2, eq72_e1892_d_b3, eq72_e1892_d_b4, eq72_e1892_d_b5, eq72_e1892_d_b6, eq72_e1892_d_b7, eq72_e1892_d_b8, eq72_e1892_d_b9, eq72_e1892_d_b10, eq72_e1892_d_b11, eq72_e1892_d_b12, eq72_e1892_d_b13, eq72_e1892_d_b14, eq72_e1892_d_b15, eq72_e1892_d_b16, eq72_e1892_d_b17];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq72_value),
            &eq72_node_derivatives,
            &eq72_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (s.dn[410][0] * s.v[158]);
        let __rspice_deriv_cse_1: f64 = (s.dn[410][1] * s.v[158]);
        let __rspice_deriv_cse_2: f64 = (s.dn[410][2] * s.v[158]);
        let __rspice_deriv_cse_3: f64 = (s.dn[410][3] * s.v[158]);
        let __rspice_deriv_cse_4: f64 = (s.dn[410][4] * s.v[158]);
        let __rspice_deriv_cse_5: f64 = (s.dn[410][5] * s.v[158]);
        let __rspice_deriv_cse_6: f64 = (s.dn[410][6] * s.v[158]);
        let __rspice_deriv_cse_7: f64 = (s.dn[410][7] * s.v[158]);
        let __rspice_deriv_cse_8: f64 = (s.dn[410][8] * s.v[158]);
        let __rspice_deriv_cse_9: f64 = (s.dn[410][9] * s.v[158]);
        let __rspice_deriv_cse_10: f64 = (s.dn[410][10] * s.v[158]);
        let __rspice_deriv_cse_11: f64 = (s.dn[410][11] * s.v[158]);
        let __rspice_deriv_cse_12: f64 = (s.dn[410][12] * s.v[158]);
        let __rspice_deriv_cse_13: f64 = (s.dn[410][13] * s.v[158]);
        let __rspice_deriv_cse_14: f64 = (s.db[410][0] * s.v[158]);
        let __rspice_deriv_cse_15: f64 = (s.db[410][1] * s.v[158]);
        let __rspice_deriv_cse_16: f64 = (s.db[410][2] * s.v[158]);
        let __rspice_deriv_cse_17: f64 = (s.db[410][3] * s.v[158]);
        let __rspice_deriv_cse_18: f64 = (s.db[410][4] * s.v[158]);
        let __rspice_deriv_cse_19: f64 = (s.db[410][5] * s.v[158]);
        let __rspice_deriv_cse_20: f64 = (s.db[410][6] * s.v[158]);
        let __rspice_deriv_cse_21: f64 = (s.db[410][7] * s.v[158]);
        let __rspice_deriv_cse_22: f64 = (s.db[410][8] * s.v[158]);
        let __rspice_deriv_cse_23: f64 = (s.db[410][9] * s.v[158]);
        let __rspice_deriv_cse_24: f64 = (s.db[410][10] * s.v[158]);
        let __rspice_deriv_cse_25: f64 = (s.db[410][11] * s.v[158]);
        let __rspice_deriv_cse_26: f64 = (s.db[410][12] * s.v[158]);
        let __rspice_deriv_cse_27: f64 = (s.db[410][13] * s.v[158]);
        let __rspice_deriv_cse_28: f64 = (s.db[410][14] * s.v[158]);
        let __rspice_deriv_cse_29: f64 = (s.db[410][15] * s.v[158]);
        let __rspice_deriv_cse_30: f64 = (s.db[410][16] * s.v[158]);
        let __rspice_deriv_cse_31: f64 = (s.db[410][17] * s.v[158]);
        let (eq73_e1920, eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13, eq73_e1920_d_b0, eq73_e1920_d_b1, eq73_e1920_d_b2, eq73_e1920_d_b3, eq73_e1920_d_b4, eq73_e1920_d_b5, eq73_e1920_d_b6, eq73_e1920_d_b7, eq73_e1920_d_b8, eq73_e1920_d_b9, eq73_e1920_d_b10, eq73_e1920_d_b11, eq73_e1920_d_b12, eq73_e1920_d_b13, eq73_e1920_d_b14, eq73_e1920_d_b15, eq73_e1920_d_b16, eq73_e1920_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && s.b[1563]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq73_e1906: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq73_e1906_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq73_e1906_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq73_e1906_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq73_e1906_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq73_e1906_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq73_e1906_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq73_e1906_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq73_e1906_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq73_e1906_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq73_e1906_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq73_e1906_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq73_e1906_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq73_e1906_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq73_e1906_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq73_e1906_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq73_e1906_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq73_e1906_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq73_e1906_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq73_e1907: f64 = (-eq73_e1906);
        let eq73_e1909: f64 = (eq73_e1907 * s.v[822]);
        let eq73_e1909_d_n0: f64 = (((-eq73_e1906_d_n0) * s.v[822]) + (eq73_e1907 * s.dn[822][0]));
        let eq73_e1909_d_n1: f64 = (((-eq73_e1906_d_n1) * s.v[822]) + (eq73_e1907 * s.dn[822][1]));
        let eq73_e1909_d_n2: f64 = (((-eq73_e1906_d_n2) * s.v[822]) + (eq73_e1907 * s.dn[822][2]));
        let eq73_e1909_d_n3: f64 = (((-eq73_e1906_d_n3) * s.v[822]) + (eq73_e1907 * s.dn[822][3]));
        let eq73_e1909_d_n4: f64 = (((-eq73_e1906_d_n4) * s.v[822]) + (eq73_e1907 * s.dn[822][4]));
        let eq73_e1909_d_n5: f64 = (((-eq73_e1906_d_n5) * s.v[822]) + (eq73_e1907 * s.dn[822][5]));
        let eq73_e1909_d_n6: f64 = (((-eq73_e1906_d_n6) * s.v[822]) + (eq73_e1907 * s.dn[822][6]));
        let eq73_e1909_d_n7: f64 = (((-eq73_e1906_d_n7) * s.v[822]) + (eq73_e1907 * s.dn[822][7]));
        let eq73_e1909_d_n8: f64 = (((-eq73_e1906_d_n8) * s.v[822]) + (eq73_e1907 * s.dn[822][8]));
        let eq73_e1909_d_n9: f64 = (((-eq73_e1906_d_n9) * s.v[822]) + (eq73_e1907 * s.dn[822][9]));
        let eq73_e1909_d_n10: f64 = (((-eq73_e1906_d_n10) * s.v[822]) + (eq73_e1907 * s.dn[822][10]));
        let eq73_e1909_d_n11: f64 = (((-eq73_e1906_d_n11) * s.v[822]) + (eq73_e1907 * s.dn[822][11]));
        let eq73_e1909_d_n12: f64 = (((-eq73_e1906_d_n12) * s.v[822]) + (eq73_e1907 * s.dn[822][12]));
        let eq73_e1909_d_n13: f64 = (((-eq73_e1906_d_n13) * s.v[822]) + (eq73_e1907 * s.dn[822][13]));
        let eq73_e1909_d_b0: f64 = (((-eq73_e1906_d_b0) * s.v[822]) + (eq73_e1907 * s.db[822][0]));
        let eq73_e1909_d_b1: f64 = (((-eq73_e1906_d_b1) * s.v[822]) + (eq73_e1907 * s.db[822][1]));
        let eq73_e1909_d_b2: f64 = (((-eq73_e1906_d_b2) * s.v[822]) + (eq73_e1907 * s.db[822][2]));
        let eq73_e1909_d_b3: f64 = (((-eq73_e1906_d_b3) * s.v[822]) + (eq73_e1907 * s.db[822][3]));
        let eq73_e1909_d_b4: f64 = (((-eq73_e1906_d_b4) * s.v[822]) + (eq73_e1907 * s.db[822][4]));
        let eq73_e1909_d_b5: f64 = (((-eq73_e1906_d_b5) * s.v[822]) + (eq73_e1907 * s.db[822][5]));
        let eq73_e1909_d_b6: f64 = (((-eq73_e1906_d_b6) * s.v[822]) + (eq73_e1907 * s.db[822][6]));
        let eq73_e1909_d_b7: f64 = (((-eq73_e1906_d_b7) * s.v[822]) + (eq73_e1907 * s.db[822][7]));
        let eq73_e1909_d_b8: f64 = (((-eq73_e1906_d_b8) * s.v[822]) + (eq73_e1907 * s.db[822][8]));
        let eq73_e1909_d_b9: f64 = (((-eq73_e1906_d_b9) * s.v[822]) + (eq73_e1907 * s.db[822][9]));
        let eq73_e1909_d_b10: f64 = (((-eq73_e1906_d_b10) * s.v[822]) + (eq73_e1907 * s.db[822][10]));
        let eq73_e1909_d_b11: f64 = (((-eq73_e1906_d_b11) * s.v[822]) + (eq73_e1907 * s.db[822][11]));
        let eq73_e1909_d_b12: f64 = (((-eq73_e1906_d_b12) * s.v[822]) + (eq73_e1907 * s.db[822][12]));
        let eq73_e1909_d_b13: f64 = (((-eq73_e1906_d_b13) * s.v[822]) + (eq73_e1907 * s.db[822][13]));
        let eq73_e1909_d_b14: f64 = (((-eq73_e1906_d_b14) * s.v[822]) + (eq73_e1907 * s.db[822][14]));
        let eq73_e1909_d_b15: f64 = (((-eq73_e1906_d_b15) * s.v[822]) + (eq73_e1907 * s.db[822][15]));
        let eq73_e1909_d_b16: f64 = (((-eq73_e1906_d_b16) * s.v[822]) + (eq73_e1907 * s.db[822][16]));
        let eq73_e1909_d_b17: f64 = (((-eq73_e1906_d_b17) * s.v[822]) + (eq73_e1907 * s.db[822][17]));
        let eq73_e1912: f64 = (s.v[410] * s.v[158]);
        let eq73_e1913: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 18, eq73_e1912);
        let eq73_e1914: f64 = (eq73_e1909 + eq73_e1913);
        let eq73_e1914_d_n0: f64 = (eq73_e1909_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq73_e1914_d_n1: f64 = (eq73_e1909_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq73_e1914_d_n2: f64 = (eq73_e1909_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq73_e1914_d_n3: f64 = (eq73_e1909_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq73_e1914_d_n4: f64 = (eq73_e1909_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq73_e1914_d_n5: f64 = (eq73_e1909_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq73_e1914_d_n6: f64 = (eq73_e1909_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq73_e1914_d_n7: f64 = (eq73_e1909_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq73_e1914_d_n8: f64 = (eq73_e1909_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq73_e1914_d_n9: f64 = (eq73_e1909_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq73_e1914_d_n10: f64 = (eq73_e1909_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq73_e1914_d_n11: f64 = (eq73_e1909_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq73_e1914_d_n12: f64 = (eq73_e1909_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq73_e1914_d_n13: f64 = (eq73_e1909_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq73_e1914_d_b0: f64 = (eq73_e1909_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq73_e1914_d_b1: f64 = (eq73_e1909_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq73_e1914_d_b2: f64 = (eq73_e1909_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq73_e1914_d_b3: f64 = (eq73_e1909_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq73_e1914_d_b4: f64 = (eq73_e1909_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq73_e1914_d_b5: f64 = (eq73_e1909_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq73_e1914_d_b6: f64 = (eq73_e1909_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq73_e1914_d_b7: f64 = (eq73_e1909_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq73_e1914_d_b8: f64 = (eq73_e1909_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq73_e1914_d_b9: f64 = (eq73_e1909_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq73_e1914_d_b10: f64 = (eq73_e1909_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq73_e1914_d_b11: f64 = (eq73_e1909_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq73_e1914_d_b12: f64 = (eq73_e1909_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq73_e1914_d_b13: f64 = (eq73_e1909_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq73_e1914_d_b14: f64 = (eq73_e1909_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq73_e1914_d_b15: f64 = (eq73_e1909_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq73_e1914_d_b16: f64 = (eq73_e1909_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq73_e1914_d_b17: f64 = (eq73_e1909_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq73_e1917: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq73_e1917_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq73_e1917_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq73_e1917_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq73_e1917_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq73_e1917_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq73_e1917_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq73_e1917_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq73_e1917_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq73_e1917_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq73_e1917_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq73_e1917_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq73_e1917_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq73_e1917_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq73_e1917_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq73_e1917_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq73_e1917_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq73_e1917_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq73_e1917_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq73_e1918: f64 = (eq73_e1914 + eq73_e1917);
        let eq73_e1918_d_n0: f64 = (eq73_e1914_d_n0 + eq73_e1917_d_n0);
        let eq73_e1918_d_n1: f64 = (eq73_e1914_d_n1 + eq73_e1917_d_n1);
        let eq73_e1918_d_n2: f64 = (eq73_e1914_d_n2 + eq73_e1917_d_n2);
        let eq73_e1918_d_n3: f64 = (eq73_e1914_d_n3 + eq73_e1917_d_n3);
        let eq73_e1918_d_n4: f64 = (eq73_e1914_d_n4 + eq73_e1917_d_n4);
        let eq73_e1918_d_n5: f64 = (eq73_e1914_d_n5 + eq73_e1917_d_n5);
        let eq73_e1918_d_n6: f64 = (eq73_e1914_d_n6 + eq73_e1917_d_n6);
        let eq73_e1918_d_n7: f64 = (eq73_e1914_d_n7 + eq73_e1917_d_n7);
        let eq73_e1918_d_n8: f64 = (eq73_e1914_d_n8 + eq73_e1917_d_n8);
        let eq73_e1918_d_n9: f64 = (eq73_e1914_d_n9 + eq73_e1917_d_n9);
        let eq73_e1918_d_n10: f64 = (eq73_e1914_d_n10 + eq73_e1917_d_n10);
        let eq73_e1918_d_n11: f64 = (eq73_e1914_d_n11 + eq73_e1917_d_n11);
        let eq73_e1918_d_n12: f64 = (eq73_e1914_d_n12 + eq73_e1917_d_n12);
        let eq73_e1918_d_n13: f64 = (eq73_e1914_d_n13 + eq73_e1917_d_n13);
        let eq73_e1918_d_b0: f64 = (eq73_e1914_d_b0 + eq73_e1917_d_b0);
        let eq73_e1918_d_b1: f64 = (eq73_e1914_d_b1 + eq73_e1917_d_b1);
        let eq73_e1918_d_b2: f64 = (eq73_e1914_d_b2 + eq73_e1917_d_b2);
        let eq73_e1918_d_b3: f64 = (eq73_e1914_d_b3 + eq73_e1917_d_b3);
        let eq73_e1918_d_b4: f64 = (eq73_e1914_d_b4 + eq73_e1917_d_b4);
        let eq73_e1918_d_b5: f64 = (eq73_e1914_d_b5 + eq73_e1917_d_b5);
        let eq73_e1918_d_b6: f64 = (eq73_e1914_d_b6 + eq73_e1917_d_b6);
        let eq73_e1918_d_b7: f64 = (eq73_e1914_d_b7 + eq73_e1917_d_b7);
        let eq73_e1918_d_b8: f64 = (eq73_e1914_d_b8 + eq73_e1917_d_b8);
        let eq73_e1918_d_b9: f64 = (eq73_e1914_d_b9 + eq73_e1917_d_b9);
        let eq73_e1918_d_b10: f64 = (eq73_e1914_d_b10 + eq73_e1917_d_b10);
        let eq73_e1918_d_b11: f64 = (eq73_e1914_d_b11 + eq73_e1917_d_b11);
        let eq73_e1918_d_b12: f64 = (eq73_e1914_d_b12 + eq73_e1917_d_b12);
        let eq73_e1918_d_b13: f64 = (eq73_e1914_d_b13 + eq73_e1917_d_b13);
        let eq73_e1918_d_b14: f64 = (eq73_e1914_d_b14 + eq73_e1917_d_b14);
        let eq73_e1918_d_b15: f64 = (eq73_e1914_d_b15 + eq73_e1917_d_b15);
        let eq73_e1918_d_b16: f64 = (eq73_e1914_d_b16 + eq73_e1917_d_b16);
        let eq73_e1918_d_b17: f64 = (eq73_e1914_d_b17 + eq73_e1917_d_b17);
        (eq73_e1918, eq73_e1918_d_n0, eq73_e1918_d_n1, eq73_e1918_d_n2, eq73_e1918_d_n3, eq73_e1918_d_n4, eq73_e1918_d_n5, eq73_e1918_d_n6, eq73_e1918_d_n7, eq73_e1918_d_n8, eq73_e1918_d_n9, eq73_e1918_d_n10, eq73_e1918_d_n11, eq73_e1918_d_n12, eq73_e1918_d_n13, eq73_e1918_d_b0, eq73_e1918_d_b1, eq73_e1918_d_b2, eq73_e1918_d_b3, eq73_e1918_d_b4, eq73_e1918_d_b5, eq73_e1918_d_b6, eq73_e1918_d_b7, eq73_e1918_d_b8, eq73_e1918_d_b9, eq73_e1918_d_b10, eq73_e1918_d_b11, eq73_e1918_d_b12, eq73_e1918_d_b13, eq73_e1918_d_b14, eq73_e1918_d_b15, eq73_e1918_d_b16, eq73_e1918_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1920;
        let eq73_node_derivatives: [f64; 14] = [eq73_e1920_d_n0, eq73_e1920_d_n1, eq73_e1920_d_n2, eq73_e1920_d_n3, eq73_e1920_d_n4, eq73_e1920_d_n5, eq73_e1920_d_n6, eq73_e1920_d_n7, eq73_e1920_d_n8, eq73_e1920_d_n9, eq73_e1920_d_n10, eq73_e1920_d_n11, eq73_e1920_d_n12, eq73_e1920_d_n13];
        let eq73_branch_derivatives: [f64; 18] = [eq73_e1920_d_b0, eq73_e1920_d_b1, eq73_e1920_d_b2, eq73_e1920_d_b3, eq73_e1920_d_b4, eq73_e1920_d_b5, eq73_e1920_d_b6, eq73_e1920_d_b7, eq73_e1920_d_b8, eq73_e1920_d_b9, eq73_e1920_d_b10, eq73_e1920_d_b11, eq73_e1920_d_b12, eq73_e1920_d_b13, eq73_e1920_d_b14, eq73_e1920_d_b15, eq73_e1920_d_b16, eq73_e1920_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq73_value),
            &eq73_node_derivatives,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1947, eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13, eq74_e1947_d_b0, eq74_e1947_d_b1, eq74_e1947_d_b2, eq74_e1947_d_b3, eq74_e1947_d_b4, eq74_e1947_d_b5, eq74_e1947_d_b6, eq74_e1947_d_b7, eq74_e1947_d_b8, eq74_e1947_d_b9, eq74_e1947_d_b10, eq74_e1947_d_b11, eq74_e1947_d_b12, eq74_e1947_d_b13, eq74_e1947_d_b14, eq74_e1947_d_b15, eq74_e1947_d_b16, eq74_e1947_d_b17,) = {
    if ((((s.b[1559] && s.b[1560]) && (!s.b[1561])) && (!s.b[1562])) && (!s.b[1563])) {
        let eq74_e1934: f64 = (-s.v[885]);
        let eq74_e1936: f64 = (eq74_e1934 * s.v[822]);
        let eq74_e1936_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq74_e1934 * s.dn[822][0]));
        let eq74_e1936_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq74_e1934 * s.dn[822][1]));
        let eq74_e1936_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq74_e1934 * s.dn[822][2]));
        let eq74_e1936_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq74_e1934 * s.dn[822][3]));
        let eq74_e1936_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq74_e1934 * s.dn[822][4]));
        let eq74_e1936_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq74_e1934 * s.dn[822][5]));
        let eq74_e1936_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq74_e1934 * s.dn[822][6]));
        let eq74_e1936_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq74_e1934 * s.dn[822][7]));
        let eq74_e1936_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq74_e1934 * s.dn[822][8]));
        let eq74_e1936_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq74_e1934 * s.dn[822][9]));
        let eq74_e1936_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq74_e1934 * s.dn[822][10]));
        let eq74_e1936_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq74_e1934 * s.dn[822][11]));
        let eq74_e1936_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq74_e1934 * s.dn[822][12]));
        let eq74_e1936_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq74_e1934 * s.dn[822][13]));
        let eq74_e1936_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq74_e1934 * s.db[822][0]));
        let eq74_e1936_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq74_e1934 * s.db[822][1]));
        let eq74_e1936_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq74_e1934 * s.db[822][2]));
        let eq74_e1936_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq74_e1934 * s.db[822][3]));
        let eq74_e1936_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq74_e1934 * s.db[822][4]));
        let eq74_e1936_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq74_e1934 * s.db[822][5]));
        let eq74_e1936_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq74_e1934 * s.db[822][6]));
        let eq74_e1936_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq74_e1934 * s.db[822][7]));
        let eq74_e1936_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq74_e1934 * s.db[822][8]));
        let eq74_e1936_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq74_e1934 * s.db[822][9]));
        let eq74_e1936_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq74_e1934 * s.db[822][10]));
        let eq74_e1936_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq74_e1934 * s.db[822][11]));
        let eq74_e1936_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq74_e1934 * s.db[822][12]));
        let eq74_e1936_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq74_e1934 * s.db[822][13]));
        let eq74_e1936_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq74_e1934 * s.db[822][14]));
        let eq74_e1936_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq74_e1934 * s.db[822][15]));
        let eq74_e1936_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq74_e1934 * s.db[822][16]));
        let eq74_e1936_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq74_e1934 * s.db[822][17]));
        let eq74_e1939: f64 = (s.v[410] * s.v[158]);
        let eq74_e1940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 19, eq74_e1939);
        let eq74_e1941: f64 = (eq74_e1936 + eq74_e1940);
        let eq74_e1941_d_n0: f64 = (eq74_e1936_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq74_e1941_d_n1: f64 = (eq74_e1936_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq74_e1941_d_n2: f64 = (eq74_e1936_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq74_e1941_d_n3: f64 = (eq74_e1936_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq74_e1941_d_n4: f64 = (eq74_e1936_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq74_e1941_d_n5: f64 = (eq74_e1936_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq74_e1941_d_n6: f64 = (eq74_e1936_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq74_e1941_d_n7: f64 = (eq74_e1936_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq74_e1941_d_n8: f64 = (eq74_e1936_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq74_e1941_d_n9: f64 = (eq74_e1936_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq74_e1941_d_n10: f64 = (eq74_e1936_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq74_e1941_d_n11: f64 = (eq74_e1936_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq74_e1941_d_n12: f64 = (eq74_e1936_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq74_e1941_d_n13: f64 = (eq74_e1936_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq74_e1941_d_b0: f64 = (eq74_e1936_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq74_e1941_d_b1: f64 = (eq74_e1936_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq74_e1941_d_b2: f64 = (eq74_e1936_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq74_e1941_d_b3: f64 = (eq74_e1936_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq74_e1941_d_b4: f64 = (eq74_e1936_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq74_e1941_d_b5: f64 = (eq74_e1936_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq74_e1941_d_b6: f64 = (eq74_e1936_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq74_e1941_d_b7: f64 = (eq74_e1936_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq74_e1941_d_b8: f64 = (eq74_e1936_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq74_e1941_d_b9: f64 = (eq74_e1936_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq74_e1941_d_b10: f64 = (eq74_e1936_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq74_e1941_d_b11: f64 = (eq74_e1936_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq74_e1941_d_b12: f64 = (eq74_e1936_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq74_e1941_d_b13: f64 = (eq74_e1936_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq74_e1941_d_b14: f64 = (eq74_e1936_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq74_e1941_d_b15: f64 = (eq74_e1936_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq74_e1941_d_b16: f64 = (eq74_e1936_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq74_e1941_d_b17: f64 = (eq74_e1936_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq74_e1944: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq74_e1944_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq74_e1944_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq74_e1944_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq74_e1944_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq74_e1944_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq74_e1944_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq74_e1944_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq74_e1944_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq74_e1944_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq74_e1944_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq74_e1944_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq74_e1944_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq74_e1944_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq74_e1944_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq74_e1944_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq74_e1944_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq74_e1944_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq74_e1944_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq74_e1945: f64 = (eq74_e1941 + eq74_e1944);
        let eq74_e1945_d_n0: f64 = (eq74_e1941_d_n0 + eq74_e1944_d_n0);
        let eq74_e1945_d_n1: f64 = (eq74_e1941_d_n1 + eq74_e1944_d_n1);
        let eq74_e1945_d_n2: f64 = (eq74_e1941_d_n2 + eq74_e1944_d_n2);
        let eq74_e1945_d_n3: f64 = (eq74_e1941_d_n3 + eq74_e1944_d_n3);
        let eq74_e1945_d_n4: f64 = (eq74_e1941_d_n4 + eq74_e1944_d_n4);
        let eq74_e1945_d_n5: f64 = (eq74_e1941_d_n5 + eq74_e1944_d_n5);
        let eq74_e1945_d_n6: f64 = (eq74_e1941_d_n6 + eq74_e1944_d_n6);
        let eq74_e1945_d_n7: f64 = (eq74_e1941_d_n7 + eq74_e1944_d_n7);
        let eq74_e1945_d_n8: f64 = (eq74_e1941_d_n8 + eq74_e1944_d_n8);
        let eq74_e1945_d_n9: f64 = (eq74_e1941_d_n9 + eq74_e1944_d_n9);
        let eq74_e1945_d_n10: f64 = (eq74_e1941_d_n10 + eq74_e1944_d_n10);
        let eq74_e1945_d_n11: f64 = (eq74_e1941_d_n11 + eq74_e1944_d_n11);
        let eq74_e1945_d_n12: f64 = (eq74_e1941_d_n12 + eq74_e1944_d_n12);
        let eq74_e1945_d_n13: f64 = (eq74_e1941_d_n13 + eq74_e1944_d_n13);
        let eq74_e1945_d_b0: f64 = (eq74_e1941_d_b0 + eq74_e1944_d_b0);
        let eq74_e1945_d_b1: f64 = (eq74_e1941_d_b1 + eq74_e1944_d_b1);
        let eq74_e1945_d_b2: f64 = (eq74_e1941_d_b2 + eq74_e1944_d_b2);
        let eq74_e1945_d_b3: f64 = (eq74_e1941_d_b3 + eq74_e1944_d_b3);
        let eq74_e1945_d_b4: f64 = (eq74_e1941_d_b4 + eq74_e1944_d_b4);
        let eq74_e1945_d_b5: f64 = (eq74_e1941_d_b5 + eq74_e1944_d_b5);
        let eq74_e1945_d_b6: f64 = (eq74_e1941_d_b6 + eq74_e1944_d_b6);
        let eq74_e1945_d_b7: f64 = (eq74_e1941_d_b7 + eq74_e1944_d_b7);
        let eq74_e1945_d_b8: f64 = (eq74_e1941_d_b8 + eq74_e1944_d_b8);
        let eq74_e1945_d_b9: f64 = (eq74_e1941_d_b9 + eq74_e1944_d_b9);
        let eq74_e1945_d_b10: f64 = (eq74_e1941_d_b10 + eq74_e1944_d_b10);
        let eq74_e1945_d_b11: f64 = (eq74_e1941_d_b11 + eq74_e1944_d_b11);
        let eq74_e1945_d_b12: f64 = (eq74_e1941_d_b12 + eq74_e1944_d_b12);
        let eq74_e1945_d_b13: f64 = (eq74_e1941_d_b13 + eq74_e1944_d_b13);
        let eq74_e1945_d_b14: f64 = (eq74_e1941_d_b14 + eq74_e1944_d_b14);
        let eq74_e1945_d_b15: f64 = (eq74_e1941_d_b15 + eq74_e1944_d_b15);
        let eq74_e1945_d_b16: f64 = (eq74_e1941_d_b16 + eq74_e1944_d_b16);
        let eq74_e1945_d_b17: f64 = (eq74_e1941_d_b17 + eq74_e1944_d_b17);
        (eq74_e1945, eq74_e1945_d_n0, eq74_e1945_d_n1, eq74_e1945_d_n2, eq74_e1945_d_n3, eq74_e1945_d_n4, eq74_e1945_d_n5, eq74_e1945_d_n6, eq74_e1945_d_n7, eq74_e1945_d_n8, eq74_e1945_d_n9, eq74_e1945_d_n10, eq74_e1945_d_n11, eq74_e1945_d_n12, eq74_e1945_d_n13, eq74_e1945_d_b0, eq74_e1945_d_b1, eq74_e1945_d_b2, eq74_e1945_d_b3, eq74_e1945_d_b4, eq74_e1945_d_b5, eq74_e1945_d_b6, eq74_e1945_d_b7, eq74_e1945_d_b8, eq74_e1945_d_b9, eq74_e1945_d_b10, eq74_e1945_d_b11, eq74_e1945_d_b12, eq74_e1945_d_b13, eq74_e1945_d_b14, eq74_e1945_d_b15, eq74_e1945_d_b16, eq74_e1945_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1947;
        let eq74_node_derivatives: [f64; 14] = [eq74_e1947_d_n0, eq74_e1947_d_n1, eq74_e1947_d_n2, eq74_e1947_d_n3, eq74_e1947_d_n4, eq74_e1947_d_n5, eq74_e1947_d_n6, eq74_e1947_d_n7, eq74_e1947_d_n8, eq74_e1947_d_n9, eq74_e1947_d_n10, eq74_e1947_d_n11, eq74_e1947_d_n12, eq74_e1947_d_n13];
        let eq74_branch_derivatives: [f64; 18] = [eq74_e1947_d_b0, eq74_e1947_d_b1, eq74_e1947_d_b2, eq74_e1947_d_b3, eq74_e1947_d_b4, eq74_e1947_d_b5, eq74_e1947_d_b6, eq74_e1947_d_b7, eq74_e1947_d_b8, eq74_e1947_d_b9, eq74_e1947_d_b10, eq74_e1947_d_b11, eq74_e1947_d_b12, eq74_e1947_d_b13, eq74_e1947_d_b14, eq74_e1947_d_b15, eq74_e1947_d_b16, eq74_e1947_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq74_value),
            &eq74_node_derivatives,
            &eq74_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let __rspice_deriv_cse_0: f64 = (s.dn[410][0] * s.v[158]);
        let __rspice_deriv_cse_1: f64 = (s.dn[410][1] * s.v[158]);
        let __rspice_deriv_cse_2: f64 = (s.dn[410][2] * s.v[158]);
        let __rspice_deriv_cse_3: f64 = (s.dn[410][3] * s.v[158]);
        let __rspice_deriv_cse_4: f64 = (s.dn[410][4] * s.v[158]);
        let __rspice_deriv_cse_5: f64 = (s.dn[410][5] * s.v[158]);
        let __rspice_deriv_cse_6: f64 = (s.dn[410][6] * s.v[158]);
        let __rspice_deriv_cse_7: f64 = (s.dn[410][7] * s.v[158]);
        let __rspice_deriv_cse_8: f64 = (s.dn[410][8] * s.v[158]);
        let __rspice_deriv_cse_9: f64 = (s.dn[410][9] * s.v[158]);
        let __rspice_deriv_cse_10: f64 = (s.dn[410][10] * s.v[158]);
        let __rspice_deriv_cse_11: f64 = (s.dn[410][11] * s.v[158]);
        let __rspice_deriv_cse_12: f64 = (s.dn[410][12] * s.v[158]);
        let __rspice_deriv_cse_13: f64 = (s.dn[410][13] * s.v[158]);
        let __rspice_deriv_cse_14: f64 = (s.db[410][0] * s.v[158]);
        let __rspice_deriv_cse_15: f64 = (s.db[410][1] * s.v[158]);
        let __rspice_deriv_cse_16: f64 = (s.db[410][2] * s.v[158]);
        let __rspice_deriv_cse_17: f64 = (s.db[410][3] * s.v[158]);
        let __rspice_deriv_cse_18: f64 = (s.db[410][4] * s.v[158]);
        let __rspice_deriv_cse_19: f64 = (s.db[410][5] * s.v[158]);
        let __rspice_deriv_cse_20: f64 = (s.db[410][6] * s.v[158]);
        let __rspice_deriv_cse_21: f64 = (s.db[410][7] * s.v[158]);
        let __rspice_deriv_cse_22: f64 = (s.db[410][8] * s.v[158]);
        let __rspice_deriv_cse_23: f64 = (s.db[410][9] * s.v[158]);
        let __rspice_deriv_cse_24: f64 = (s.db[410][10] * s.v[158]);
        let __rspice_deriv_cse_25: f64 = (s.db[410][11] * s.v[158]);
        let __rspice_deriv_cse_26: f64 = (s.db[410][12] * s.v[158]);
        let __rspice_deriv_cse_27: f64 = (s.db[410][13] * s.v[158]);
        let __rspice_deriv_cse_28: f64 = (s.db[410][14] * s.v[158]);
        let __rspice_deriv_cse_29: f64 = (s.db[410][15] * s.v[158]);
        let __rspice_deriv_cse_30: f64 = (s.db[410][16] * s.v[158]);
        let __rspice_deriv_cse_31: f64 = (s.db[410][17] * s.v[158]);
        let (eq75_e1970, eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13, eq75_e1970_d_b0, eq75_e1970_d_b1, eq75_e1970_d_b2, eq75_e1970_d_b3, eq75_e1970_d_b4, eq75_e1970_d_b5, eq75_e1970_d_b6, eq75_e1970_d_b7, eq75_e1970_d_b8, eq75_e1970_d_b9, eq75_e1970_d_b10, eq75_e1970_d_b11, eq75_e1970_d_b12, eq75_e1970_d_b13, eq75_e1970_d_b14, eq75_e1970_d_b15, eq75_e1970_d_b16, eq75_e1970_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && s.b[1564]) {
        let __rspice_inv_cse_0: f64 = 1.0 / p.p30;
        let eq75_e1956: f64 = (s.v[885] * __rspice_inv_cse_0);
        let eq75_e1956_d_n0: f64 = (s.dn[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_n1: f64 = (s.dn[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_n2: f64 = (s.dn[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_n3: f64 = (s.dn[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_n4: f64 = (s.dn[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_n5: f64 = (s.dn[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_n6: f64 = (s.dn[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_n7: f64 = (s.dn[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_n8: f64 = (s.dn[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_n9: f64 = (s.dn[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_n10: f64 = (s.dn[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_n11: f64 = (s.dn[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_n12: f64 = (s.dn[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_n13: f64 = (s.dn[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b0: f64 = (s.db[885][0] * __rspice_inv_cse_0);
        let eq75_e1956_d_b1: f64 = (s.db[885][1] * __rspice_inv_cse_0);
        let eq75_e1956_d_b2: f64 = (s.db[885][2] * __rspice_inv_cse_0);
        let eq75_e1956_d_b3: f64 = (s.db[885][3] * __rspice_inv_cse_0);
        let eq75_e1956_d_b4: f64 = (s.db[885][4] * __rspice_inv_cse_0);
        let eq75_e1956_d_b5: f64 = (s.db[885][5] * __rspice_inv_cse_0);
        let eq75_e1956_d_b6: f64 = (s.db[885][6] * __rspice_inv_cse_0);
        let eq75_e1956_d_b7: f64 = (s.db[885][7] * __rspice_inv_cse_0);
        let eq75_e1956_d_b8: f64 = (s.db[885][8] * __rspice_inv_cse_0);
        let eq75_e1956_d_b9: f64 = (s.db[885][9] * __rspice_inv_cse_0);
        let eq75_e1956_d_b10: f64 = (s.db[885][10] * __rspice_inv_cse_0);
        let eq75_e1956_d_b11: f64 = (s.db[885][11] * __rspice_inv_cse_0);
        let eq75_e1956_d_b12: f64 = (s.db[885][12] * __rspice_inv_cse_0);
        let eq75_e1956_d_b13: f64 = (s.db[885][13] * __rspice_inv_cse_0);
        let eq75_e1956_d_b14: f64 = (s.db[885][14] * __rspice_inv_cse_0);
        let eq75_e1956_d_b15: f64 = (s.db[885][15] * __rspice_inv_cse_0);
        let eq75_e1956_d_b16: f64 = (s.db[885][16] * __rspice_inv_cse_0);
        let eq75_e1956_d_b17: f64 = (s.db[885][17] * __rspice_inv_cse_0);
        let eq75_e1957: f64 = (-eq75_e1956);
        let eq75_e1959: f64 = (eq75_e1957 * s.v[822]);
        let eq75_e1959_d_n0: f64 = (((-eq75_e1956_d_n0) * s.v[822]) + (eq75_e1957 * s.dn[822][0]));
        let eq75_e1959_d_n1: f64 = (((-eq75_e1956_d_n1) * s.v[822]) + (eq75_e1957 * s.dn[822][1]));
        let eq75_e1959_d_n2: f64 = (((-eq75_e1956_d_n2) * s.v[822]) + (eq75_e1957 * s.dn[822][2]));
        let eq75_e1959_d_n3: f64 = (((-eq75_e1956_d_n3) * s.v[822]) + (eq75_e1957 * s.dn[822][3]));
        let eq75_e1959_d_n4: f64 = (((-eq75_e1956_d_n4) * s.v[822]) + (eq75_e1957 * s.dn[822][4]));
        let eq75_e1959_d_n5: f64 = (((-eq75_e1956_d_n5) * s.v[822]) + (eq75_e1957 * s.dn[822][5]));
        let eq75_e1959_d_n6: f64 = (((-eq75_e1956_d_n6) * s.v[822]) + (eq75_e1957 * s.dn[822][6]));
        let eq75_e1959_d_n7: f64 = (((-eq75_e1956_d_n7) * s.v[822]) + (eq75_e1957 * s.dn[822][7]));
        let eq75_e1959_d_n8: f64 = (((-eq75_e1956_d_n8) * s.v[822]) + (eq75_e1957 * s.dn[822][8]));
        let eq75_e1959_d_n9: f64 = (((-eq75_e1956_d_n9) * s.v[822]) + (eq75_e1957 * s.dn[822][9]));
        let eq75_e1959_d_n10: f64 = (((-eq75_e1956_d_n10) * s.v[822]) + (eq75_e1957 * s.dn[822][10]));
        let eq75_e1959_d_n11: f64 = (((-eq75_e1956_d_n11) * s.v[822]) + (eq75_e1957 * s.dn[822][11]));
        let eq75_e1959_d_n12: f64 = (((-eq75_e1956_d_n12) * s.v[822]) + (eq75_e1957 * s.dn[822][12]));
        let eq75_e1959_d_n13: f64 = (((-eq75_e1956_d_n13) * s.v[822]) + (eq75_e1957 * s.dn[822][13]));
        let eq75_e1959_d_b0: f64 = (((-eq75_e1956_d_b0) * s.v[822]) + (eq75_e1957 * s.db[822][0]));
        let eq75_e1959_d_b1: f64 = (((-eq75_e1956_d_b1) * s.v[822]) + (eq75_e1957 * s.db[822][1]));
        let eq75_e1959_d_b2: f64 = (((-eq75_e1956_d_b2) * s.v[822]) + (eq75_e1957 * s.db[822][2]));
        let eq75_e1959_d_b3: f64 = (((-eq75_e1956_d_b3) * s.v[822]) + (eq75_e1957 * s.db[822][3]));
        let eq75_e1959_d_b4: f64 = (((-eq75_e1956_d_b4) * s.v[822]) + (eq75_e1957 * s.db[822][4]));
        let eq75_e1959_d_b5: f64 = (((-eq75_e1956_d_b5) * s.v[822]) + (eq75_e1957 * s.db[822][5]));
        let eq75_e1959_d_b6: f64 = (((-eq75_e1956_d_b6) * s.v[822]) + (eq75_e1957 * s.db[822][6]));
        let eq75_e1959_d_b7: f64 = (((-eq75_e1956_d_b7) * s.v[822]) + (eq75_e1957 * s.db[822][7]));
        let eq75_e1959_d_b8: f64 = (((-eq75_e1956_d_b8) * s.v[822]) + (eq75_e1957 * s.db[822][8]));
        let eq75_e1959_d_b9: f64 = (((-eq75_e1956_d_b9) * s.v[822]) + (eq75_e1957 * s.db[822][9]));
        let eq75_e1959_d_b10: f64 = (((-eq75_e1956_d_b10) * s.v[822]) + (eq75_e1957 * s.db[822][10]));
        let eq75_e1959_d_b11: f64 = (((-eq75_e1956_d_b11) * s.v[822]) + (eq75_e1957 * s.db[822][11]));
        let eq75_e1959_d_b12: f64 = (((-eq75_e1956_d_b12) * s.v[822]) + (eq75_e1957 * s.db[822][12]));
        let eq75_e1959_d_b13: f64 = (((-eq75_e1956_d_b13) * s.v[822]) + (eq75_e1957 * s.db[822][13]));
        let eq75_e1959_d_b14: f64 = (((-eq75_e1956_d_b14) * s.v[822]) + (eq75_e1957 * s.db[822][14]));
        let eq75_e1959_d_b15: f64 = (((-eq75_e1956_d_b15) * s.v[822]) + (eq75_e1957 * s.db[822][15]));
        let eq75_e1959_d_b16: f64 = (((-eq75_e1956_d_b16) * s.v[822]) + (eq75_e1957 * s.db[822][16]));
        let eq75_e1959_d_b17: f64 = (((-eq75_e1956_d_b17) * s.v[822]) + (eq75_e1957 * s.db[822][17]));
        let eq75_e1962: f64 = (s.v[410] * s.v[158]);
        let eq75_e1963: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 20, eq75_e1962);
        let eq75_e1964: f64 = (eq75_e1959 + eq75_e1963);
        let eq75_e1964_d_n0: f64 = (eq75_e1959_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq75_e1964_d_n1: f64 = (eq75_e1959_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq75_e1964_d_n2: f64 = (eq75_e1959_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq75_e1964_d_n3: f64 = (eq75_e1959_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq75_e1964_d_n4: f64 = (eq75_e1959_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq75_e1964_d_n5: f64 = (eq75_e1959_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq75_e1964_d_n6: f64 = (eq75_e1959_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq75_e1964_d_n7: f64 = (eq75_e1959_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq75_e1964_d_n8: f64 = (eq75_e1959_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq75_e1964_d_n9: f64 = (eq75_e1959_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq75_e1964_d_n10: f64 = (eq75_e1959_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq75_e1964_d_n11: f64 = (eq75_e1959_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq75_e1964_d_n12: f64 = (eq75_e1959_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq75_e1964_d_n13: f64 = (eq75_e1959_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq75_e1964_d_b0: f64 = (eq75_e1959_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq75_e1964_d_b1: f64 = (eq75_e1959_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq75_e1964_d_b2: f64 = (eq75_e1959_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq75_e1964_d_b3: f64 = (eq75_e1959_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq75_e1964_d_b4: f64 = (eq75_e1959_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq75_e1964_d_b5: f64 = (eq75_e1959_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq75_e1964_d_b6: f64 = (eq75_e1959_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq75_e1964_d_b7: f64 = (eq75_e1959_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq75_e1964_d_b8: f64 = (eq75_e1959_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq75_e1964_d_b9: f64 = (eq75_e1959_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq75_e1964_d_b10: f64 = (eq75_e1959_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq75_e1964_d_b11: f64 = (eq75_e1959_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq75_e1964_d_b12: f64 = (eq75_e1959_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq75_e1964_d_b13: f64 = (eq75_e1959_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq75_e1964_d_b14: f64 = (eq75_e1959_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq75_e1964_d_b15: f64 = (eq75_e1959_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq75_e1964_d_b16: f64 = (eq75_e1959_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq75_e1964_d_b17: f64 = (eq75_e1959_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_1: f64 = 1.0 / s.v[157];
        let eq75_e1967: f64 = (s.v[410] * __rspice_inv_cse_1);
        let eq75_e1967_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_1);
        let eq75_e1967_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_1);
        let eq75_e1967_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_1);
        let eq75_e1967_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_1);
        let eq75_e1967_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_1);
        let eq75_e1967_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_1);
        let eq75_e1967_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_1);
        let eq75_e1967_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_1);
        let eq75_e1967_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_1);
        let eq75_e1967_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_1);
        let eq75_e1967_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_1);
        let eq75_e1967_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_1);
        let eq75_e1967_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_1);
        let eq75_e1967_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_1);
        let eq75_e1967_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_1);
        let eq75_e1967_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_1);
        let eq75_e1967_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_1);
        let eq75_e1967_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_1);
        let eq75_e1968: f64 = (eq75_e1964 + eq75_e1967);
        let eq75_e1968_d_n0: f64 = (eq75_e1964_d_n0 + eq75_e1967_d_n0);
        let eq75_e1968_d_n1: f64 = (eq75_e1964_d_n1 + eq75_e1967_d_n1);
        let eq75_e1968_d_n2: f64 = (eq75_e1964_d_n2 + eq75_e1967_d_n2);
        let eq75_e1968_d_n3: f64 = (eq75_e1964_d_n3 + eq75_e1967_d_n3);
        let eq75_e1968_d_n4: f64 = (eq75_e1964_d_n4 + eq75_e1967_d_n4);
        let eq75_e1968_d_n5: f64 = (eq75_e1964_d_n5 + eq75_e1967_d_n5);
        let eq75_e1968_d_n6: f64 = (eq75_e1964_d_n6 + eq75_e1967_d_n6);
        let eq75_e1968_d_n7: f64 = (eq75_e1964_d_n7 + eq75_e1967_d_n7);
        let eq75_e1968_d_n8: f64 = (eq75_e1964_d_n8 + eq75_e1967_d_n8);
        let eq75_e1968_d_n9: f64 = (eq75_e1964_d_n9 + eq75_e1967_d_n9);
        let eq75_e1968_d_n10: f64 = (eq75_e1964_d_n10 + eq75_e1967_d_n10);
        let eq75_e1968_d_n11: f64 = (eq75_e1964_d_n11 + eq75_e1967_d_n11);
        let eq75_e1968_d_n12: f64 = (eq75_e1964_d_n12 + eq75_e1967_d_n12);
        let eq75_e1968_d_n13: f64 = (eq75_e1964_d_n13 + eq75_e1967_d_n13);
        let eq75_e1968_d_b0: f64 = (eq75_e1964_d_b0 + eq75_e1967_d_b0);
        let eq75_e1968_d_b1: f64 = (eq75_e1964_d_b1 + eq75_e1967_d_b1);
        let eq75_e1968_d_b2: f64 = (eq75_e1964_d_b2 + eq75_e1967_d_b2);
        let eq75_e1968_d_b3: f64 = (eq75_e1964_d_b3 + eq75_e1967_d_b3);
        let eq75_e1968_d_b4: f64 = (eq75_e1964_d_b4 + eq75_e1967_d_b4);
        let eq75_e1968_d_b5: f64 = (eq75_e1964_d_b5 + eq75_e1967_d_b5);
        let eq75_e1968_d_b6: f64 = (eq75_e1964_d_b6 + eq75_e1967_d_b6);
        let eq75_e1968_d_b7: f64 = (eq75_e1964_d_b7 + eq75_e1967_d_b7);
        let eq75_e1968_d_b8: f64 = (eq75_e1964_d_b8 + eq75_e1967_d_b8);
        let eq75_e1968_d_b9: f64 = (eq75_e1964_d_b9 + eq75_e1967_d_b9);
        let eq75_e1968_d_b10: f64 = (eq75_e1964_d_b10 + eq75_e1967_d_b10);
        let eq75_e1968_d_b11: f64 = (eq75_e1964_d_b11 + eq75_e1967_d_b11);
        let eq75_e1968_d_b12: f64 = (eq75_e1964_d_b12 + eq75_e1967_d_b12);
        let eq75_e1968_d_b13: f64 = (eq75_e1964_d_b13 + eq75_e1967_d_b13);
        let eq75_e1968_d_b14: f64 = (eq75_e1964_d_b14 + eq75_e1967_d_b14);
        let eq75_e1968_d_b15: f64 = (eq75_e1964_d_b15 + eq75_e1967_d_b15);
        let eq75_e1968_d_b16: f64 = (eq75_e1964_d_b16 + eq75_e1967_d_b16);
        let eq75_e1968_d_b17: f64 = (eq75_e1964_d_b17 + eq75_e1967_d_b17);
        (eq75_e1968, eq75_e1968_d_n0, eq75_e1968_d_n1, eq75_e1968_d_n2, eq75_e1968_d_n3, eq75_e1968_d_n4, eq75_e1968_d_n5, eq75_e1968_d_n6, eq75_e1968_d_n7, eq75_e1968_d_n8, eq75_e1968_d_n9, eq75_e1968_d_n10, eq75_e1968_d_n11, eq75_e1968_d_n12, eq75_e1968_d_n13, eq75_e1968_d_b0, eq75_e1968_d_b1, eq75_e1968_d_b2, eq75_e1968_d_b3, eq75_e1968_d_b4, eq75_e1968_d_b5, eq75_e1968_d_b6, eq75_e1968_d_b7, eq75_e1968_d_b8, eq75_e1968_d_b9, eq75_e1968_d_b10, eq75_e1968_d_b11, eq75_e1968_d_b12, eq75_e1968_d_b13, eq75_e1968_d_b14, eq75_e1968_d_b15, eq75_e1968_d_b16, eq75_e1968_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1970;
        let eq75_node_derivatives: [f64; 14] = [eq75_e1970_d_n0, eq75_e1970_d_n1, eq75_e1970_d_n2, eq75_e1970_d_n3, eq75_e1970_d_n4, eq75_e1970_d_n5, eq75_e1970_d_n6, eq75_e1970_d_n7, eq75_e1970_d_n8, eq75_e1970_d_n9, eq75_e1970_d_n10, eq75_e1970_d_n11, eq75_e1970_d_n12, eq75_e1970_d_n13];
        let eq75_branch_derivatives: [f64; 18] = [eq75_e1970_d_b0, eq75_e1970_d_b1, eq75_e1970_d_b2, eq75_e1970_d_b3, eq75_e1970_d_b4, eq75_e1970_d_b5, eq75_e1970_d_b6, eq75_e1970_d_b7, eq75_e1970_d_b8, eq75_e1970_d_b9, eq75_e1970_d_b10, eq75_e1970_d_b11, eq75_e1970_d_b12, eq75_e1970_d_b13, eq75_e1970_d_b14, eq75_e1970_d_b15, eq75_e1970_d_b16, eq75_e1970_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq75_value),
            &eq75_node_derivatives,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1992, eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13, eq76_e1992_d_b0, eq76_e1992_d_b1, eq76_e1992_d_b2, eq76_e1992_d_b3, eq76_e1992_d_b4, eq76_e1992_d_b5, eq76_e1992_d_b6, eq76_e1992_d_b7, eq76_e1992_d_b8, eq76_e1992_d_b9, eq76_e1992_d_b10, eq76_e1992_d_b11, eq76_e1992_d_b12, eq76_e1992_d_b13, eq76_e1992_d_b14, eq76_e1992_d_b15, eq76_e1992_d_b16, eq76_e1992_d_b17,) = {
    if ((s.b[1559] && (!s.b[1560])) && (!s.b[1564])) {
        let eq76_e1979: f64 = (-s.v[885]);
        let eq76_e1981: f64 = (eq76_e1979 * s.v[822]);
        let eq76_e1981_d_n0: f64 = (((-s.dn[885][0]) * s.v[822]) + (eq76_e1979 * s.dn[822][0]));
        let eq76_e1981_d_n1: f64 = (((-s.dn[885][1]) * s.v[822]) + (eq76_e1979 * s.dn[822][1]));
        let eq76_e1981_d_n2: f64 = (((-s.dn[885][2]) * s.v[822]) + (eq76_e1979 * s.dn[822][2]));
        let eq76_e1981_d_n3: f64 = (((-s.dn[885][3]) * s.v[822]) + (eq76_e1979 * s.dn[822][3]));
        let eq76_e1981_d_n4: f64 = (((-s.dn[885][4]) * s.v[822]) + (eq76_e1979 * s.dn[822][4]));
        let eq76_e1981_d_n5: f64 = (((-s.dn[885][5]) * s.v[822]) + (eq76_e1979 * s.dn[822][5]));
        let eq76_e1981_d_n6: f64 = (((-s.dn[885][6]) * s.v[822]) + (eq76_e1979 * s.dn[822][6]));
        let eq76_e1981_d_n7: f64 = (((-s.dn[885][7]) * s.v[822]) + (eq76_e1979 * s.dn[822][7]));
        let eq76_e1981_d_n8: f64 = (((-s.dn[885][8]) * s.v[822]) + (eq76_e1979 * s.dn[822][8]));
        let eq76_e1981_d_n9: f64 = (((-s.dn[885][9]) * s.v[822]) + (eq76_e1979 * s.dn[822][9]));
        let eq76_e1981_d_n10: f64 = (((-s.dn[885][10]) * s.v[822]) + (eq76_e1979 * s.dn[822][10]));
        let eq76_e1981_d_n11: f64 = (((-s.dn[885][11]) * s.v[822]) + (eq76_e1979 * s.dn[822][11]));
        let eq76_e1981_d_n12: f64 = (((-s.dn[885][12]) * s.v[822]) + (eq76_e1979 * s.dn[822][12]));
        let eq76_e1981_d_n13: f64 = (((-s.dn[885][13]) * s.v[822]) + (eq76_e1979 * s.dn[822][13]));
        let eq76_e1981_d_b0: f64 = (((-s.db[885][0]) * s.v[822]) + (eq76_e1979 * s.db[822][0]));
        let eq76_e1981_d_b1: f64 = (((-s.db[885][1]) * s.v[822]) + (eq76_e1979 * s.db[822][1]));
        let eq76_e1981_d_b2: f64 = (((-s.db[885][2]) * s.v[822]) + (eq76_e1979 * s.db[822][2]));
        let eq76_e1981_d_b3: f64 = (((-s.db[885][3]) * s.v[822]) + (eq76_e1979 * s.db[822][3]));
        let eq76_e1981_d_b4: f64 = (((-s.db[885][4]) * s.v[822]) + (eq76_e1979 * s.db[822][4]));
        let eq76_e1981_d_b5: f64 = (((-s.db[885][5]) * s.v[822]) + (eq76_e1979 * s.db[822][5]));
        let eq76_e1981_d_b6: f64 = (((-s.db[885][6]) * s.v[822]) + (eq76_e1979 * s.db[822][6]));
        let eq76_e1981_d_b7: f64 = (((-s.db[885][7]) * s.v[822]) + (eq76_e1979 * s.db[822][7]));
        let eq76_e1981_d_b8: f64 = (((-s.db[885][8]) * s.v[822]) + (eq76_e1979 * s.db[822][8]));
        let eq76_e1981_d_b9: f64 = (((-s.db[885][9]) * s.v[822]) + (eq76_e1979 * s.db[822][9]));
        let eq76_e1981_d_b10: f64 = (((-s.db[885][10]) * s.v[822]) + (eq76_e1979 * s.db[822][10]));
        let eq76_e1981_d_b11: f64 = (((-s.db[885][11]) * s.v[822]) + (eq76_e1979 * s.db[822][11]));
        let eq76_e1981_d_b12: f64 = (((-s.db[885][12]) * s.v[822]) + (eq76_e1979 * s.db[822][12]));
        let eq76_e1981_d_b13: f64 = (((-s.db[885][13]) * s.v[822]) + (eq76_e1979 * s.db[822][13]));
        let eq76_e1981_d_b14: f64 = (((-s.db[885][14]) * s.v[822]) + (eq76_e1979 * s.db[822][14]));
        let eq76_e1981_d_b15: f64 = (((-s.db[885][15]) * s.v[822]) + (eq76_e1979 * s.db[822][15]));
        let eq76_e1981_d_b16: f64 = (((-s.db[885][16]) * s.v[822]) + (eq76_e1979 * s.db[822][16]));
        let eq76_e1981_d_b17: f64 = (((-s.db[885][17]) * s.v[822]) + (eq76_e1979 * s.db[822][17]));
        let eq76_e1984: f64 = (s.v[410] * s.v[158]);
        let eq76_e1985: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 21, eq76_e1984);
        let eq76_e1986: f64 = (eq76_e1981 + eq76_e1985);
        let eq76_e1986_d_n0: f64 = (eq76_e1981_d_n0 + (__rspice_deriv_cse_0 * ddt_scale));
        let eq76_e1986_d_n1: f64 = (eq76_e1981_d_n1 + (__rspice_deriv_cse_1 * ddt_scale));
        let eq76_e1986_d_n2: f64 = (eq76_e1981_d_n2 + (__rspice_deriv_cse_2 * ddt_scale));
        let eq76_e1986_d_n3: f64 = (eq76_e1981_d_n3 + (__rspice_deriv_cse_3 * ddt_scale));
        let eq76_e1986_d_n4: f64 = (eq76_e1981_d_n4 + (__rspice_deriv_cse_4 * ddt_scale));
        let eq76_e1986_d_n5: f64 = (eq76_e1981_d_n5 + (__rspice_deriv_cse_5 * ddt_scale));
        let eq76_e1986_d_n6: f64 = (eq76_e1981_d_n6 + (__rspice_deriv_cse_6 * ddt_scale));
        let eq76_e1986_d_n7: f64 = (eq76_e1981_d_n7 + (__rspice_deriv_cse_7 * ddt_scale));
        let eq76_e1986_d_n8: f64 = (eq76_e1981_d_n8 + (__rspice_deriv_cse_8 * ddt_scale));
        let eq76_e1986_d_n9: f64 = (eq76_e1981_d_n9 + (__rspice_deriv_cse_9 * ddt_scale));
        let eq76_e1986_d_n10: f64 = (eq76_e1981_d_n10 + (__rspice_deriv_cse_10 * ddt_scale));
        let eq76_e1986_d_n11: f64 = (eq76_e1981_d_n11 + (__rspice_deriv_cse_11 * ddt_scale));
        let eq76_e1986_d_n12: f64 = (eq76_e1981_d_n12 + (__rspice_deriv_cse_12 * ddt_scale));
        let eq76_e1986_d_n13: f64 = (eq76_e1981_d_n13 + (__rspice_deriv_cse_13 * ddt_scale));
        let eq76_e1986_d_b0: f64 = (eq76_e1981_d_b0 + (__rspice_deriv_cse_14 * ddt_scale));
        let eq76_e1986_d_b1: f64 = (eq76_e1981_d_b1 + (__rspice_deriv_cse_15 * ddt_scale));
        let eq76_e1986_d_b2: f64 = (eq76_e1981_d_b2 + (__rspice_deriv_cse_16 * ddt_scale));
        let eq76_e1986_d_b3: f64 = (eq76_e1981_d_b3 + (__rspice_deriv_cse_17 * ddt_scale));
        let eq76_e1986_d_b4: f64 = (eq76_e1981_d_b4 + (__rspice_deriv_cse_18 * ddt_scale));
        let eq76_e1986_d_b5: f64 = (eq76_e1981_d_b5 + (__rspice_deriv_cse_19 * ddt_scale));
        let eq76_e1986_d_b6: f64 = (eq76_e1981_d_b6 + (__rspice_deriv_cse_20 * ddt_scale));
        let eq76_e1986_d_b7: f64 = (eq76_e1981_d_b7 + (__rspice_deriv_cse_21 * ddt_scale));
        let eq76_e1986_d_b8: f64 = (eq76_e1981_d_b8 + (__rspice_deriv_cse_22 * ddt_scale));
        let eq76_e1986_d_b9: f64 = (eq76_e1981_d_b9 + (__rspice_deriv_cse_23 * ddt_scale));
        let eq76_e1986_d_b10: f64 = (eq76_e1981_d_b10 + (__rspice_deriv_cse_24 * ddt_scale));
        let eq76_e1986_d_b11: f64 = (eq76_e1981_d_b11 + (__rspice_deriv_cse_25 * ddt_scale));
        let eq76_e1986_d_b12: f64 = (eq76_e1981_d_b12 + (__rspice_deriv_cse_26 * ddt_scale));
        let eq76_e1986_d_b13: f64 = (eq76_e1981_d_b13 + (__rspice_deriv_cse_27 * ddt_scale));
        let eq76_e1986_d_b14: f64 = (eq76_e1981_d_b14 + (__rspice_deriv_cse_28 * ddt_scale));
        let eq76_e1986_d_b15: f64 = (eq76_e1981_d_b15 + (__rspice_deriv_cse_29 * ddt_scale));
        let eq76_e1986_d_b16: f64 = (eq76_e1981_d_b16 + (__rspice_deriv_cse_30 * ddt_scale));
        let eq76_e1986_d_b17: f64 = (eq76_e1981_d_b17 + (__rspice_deriv_cse_31 * ddt_scale));
        let __rspice_inv_cse_2: f64 = 1.0 / s.v[157];
        let eq76_e1989: f64 = (s.v[410] * __rspice_inv_cse_2);
        let eq76_e1989_d_n0: f64 = (s.dn[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_n1: f64 = (s.dn[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_n2: f64 = (s.dn[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_n3: f64 = (s.dn[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_n4: f64 = (s.dn[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_n5: f64 = (s.dn[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_n6: f64 = (s.dn[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_n7: f64 = (s.dn[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_n8: f64 = (s.dn[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_n9: f64 = (s.dn[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_n10: f64 = (s.dn[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_n11: f64 = (s.dn[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_n12: f64 = (s.dn[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_n13: f64 = (s.dn[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b0: f64 = (s.db[410][0] * __rspice_inv_cse_2);
        let eq76_e1989_d_b1: f64 = (s.db[410][1] * __rspice_inv_cse_2);
        let eq76_e1989_d_b2: f64 = (s.db[410][2] * __rspice_inv_cse_2);
        let eq76_e1989_d_b3: f64 = (s.db[410][3] * __rspice_inv_cse_2);
        let eq76_e1989_d_b4: f64 = (s.db[410][4] * __rspice_inv_cse_2);
        let eq76_e1989_d_b5: f64 = (s.db[410][5] * __rspice_inv_cse_2);
        let eq76_e1989_d_b6: f64 = (s.db[410][6] * __rspice_inv_cse_2);
        let eq76_e1989_d_b7: f64 = (s.db[410][7] * __rspice_inv_cse_2);
        let eq76_e1989_d_b8: f64 = (s.db[410][8] * __rspice_inv_cse_2);
        let eq76_e1989_d_b9: f64 = (s.db[410][9] * __rspice_inv_cse_2);
        let eq76_e1989_d_b10: f64 = (s.db[410][10] * __rspice_inv_cse_2);
        let eq76_e1989_d_b11: f64 = (s.db[410][11] * __rspice_inv_cse_2);
        let eq76_e1989_d_b12: f64 = (s.db[410][12] * __rspice_inv_cse_2);
        let eq76_e1989_d_b13: f64 = (s.db[410][13] * __rspice_inv_cse_2);
        let eq76_e1989_d_b14: f64 = (s.db[410][14] * __rspice_inv_cse_2);
        let eq76_e1989_d_b15: f64 = (s.db[410][15] * __rspice_inv_cse_2);
        let eq76_e1989_d_b16: f64 = (s.db[410][16] * __rspice_inv_cse_2);
        let eq76_e1989_d_b17: f64 = (s.db[410][17] * __rspice_inv_cse_2);
        let eq76_e1990: f64 = (eq76_e1986 + eq76_e1989);
        let eq76_e1990_d_n0: f64 = (eq76_e1986_d_n0 + eq76_e1989_d_n0);
        let eq76_e1990_d_n1: f64 = (eq76_e1986_d_n1 + eq76_e1989_d_n1);
        let eq76_e1990_d_n2: f64 = (eq76_e1986_d_n2 + eq76_e1989_d_n2);
        let eq76_e1990_d_n3: f64 = (eq76_e1986_d_n3 + eq76_e1989_d_n3);
        let eq76_e1990_d_n4: f64 = (eq76_e1986_d_n4 + eq76_e1989_d_n4);
        let eq76_e1990_d_n5: f64 = (eq76_e1986_d_n5 + eq76_e1989_d_n5);
        let eq76_e1990_d_n6: f64 = (eq76_e1986_d_n6 + eq76_e1989_d_n6);
        let eq76_e1990_d_n7: f64 = (eq76_e1986_d_n7 + eq76_e1989_d_n7);
        let eq76_e1990_d_n8: f64 = (eq76_e1986_d_n8 + eq76_e1989_d_n8);
        let eq76_e1990_d_n9: f64 = (eq76_e1986_d_n9 + eq76_e1989_d_n9);
        let eq76_e1990_d_n10: f64 = (eq76_e1986_d_n10 + eq76_e1989_d_n10);
        let eq76_e1990_d_n11: f64 = (eq76_e1986_d_n11 + eq76_e1989_d_n11);
        let eq76_e1990_d_n12: f64 = (eq76_e1986_d_n12 + eq76_e1989_d_n12);
        let eq76_e1990_d_n13: f64 = (eq76_e1986_d_n13 + eq76_e1989_d_n13);
        let eq76_e1990_d_b0: f64 = (eq76_e1986_d_b0 + eq76_e1989_d_b0);
        let eq76_e1990_d_b1: f64 = (eq76_e1986_d_b1 + eq76_e1989_d_b1);
        let eq76_e1990_d_b2: f64 = (eq76_e1986_d_b2 + eq76_e1989_d_b2);
        let eq76_e1990_d_b3: f64 = (eq76_e1986_d_b3 + eq76_e1989_d_b3);
        let eq76_e1990_d_b4: f64 = (eq76_e1986_d_b4 + eq76_e1989_d_b4);
        let eq76_e1990_d_b5: f64 = (eq76_e1986_d_b5 + eq76_e1989_d_b5);
        let eq76_e1990_d_b6: f64 = (eq76_e1986_d_b6 + eq76_e1989_d_b6);
        let eq76_e1990_d_b7: f64 = (eq76_e1986_d_b7 + eq76_e1989_d_b7);
        let eq76_e1990_d_b8: f64 = (eq76_e1986_d_b8 + eq76_e1989_d_b8);
        let eq76_e1990_d_b9: f64 = (eq76_e1986_d_b9 + eq76_e1989_d_b9);
        let eq76_e1990_d_b10: f64 = (eq76_e1986_d_b10 + eq76_e1989_d_b10);
        let eq76_e1990_d_b11: f64 = (eq76_e1986_d_b11 + eq76_e1989_d_b11);
        let eq76_e1990_d_b12: f64 = (eq76_e1986_d_b12 + eq76_e1989_d_b12);
        let eq76_e1990_d_b13: f64 = (eq76_e1986_d_b13 + eq76_e1989_d_b13);
        let eq76_e1990_d_b14: f64 = (eq76_e1986_d_b14 + eq76_e1989_d_b14);
        let eq76_e1990_d_b15: f64 = (eq76_e1986_d_b15 + eq76_e1989_d_b15);
        let eq76_e1990_d_b16: f64 = (eq76_e1986_d_b16 + eq76_e1989_d_b16);
        let eq76_e1990_d_b17: f64 = (eq76_e1986_d_b17 + eq76_e1989_d_b17);
        (eq76_e1990, eq76_e1990_d_n0, eq76_e1990_d_n1, eq76_e1990_d_n2, eq76_e1990_d_n3, eq76_e1990_d_n4, eq76_e1990_d_n5, eq76_e1990_d_n6, eq76_e1990_d_n7, eq76_e1990_d_n8, eq76_e1990_d_n9, eq76_e1990_d_n10, eq76_e1990_d_n11, eq76_e1990_d_n12, eq76_e1990_d_n13, eq76_e1990_d_b0, eq76_e1990_d_b1, eq76_e1990_d_b2, eq76_e1990_d_b3, eq76_e1990_d_b4, eq76_e1990_d_b5, eq76_e1990_d_b6, eq76_e1990_d_b7, eq76_e1990_d_b8, eq76_e1990_d_b9, eq76_e1990_d_b10, eq76_e1990_d_b11, eq76_e1990_d_b12, eq76_e1990_d_b13, eq76_e1990_d_b14, eq76_e1990_d_b15, eq76_e1990_d_b16, eq76_e1990_d_b17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1992;
        let eq76_node_derivatives: [f64; 14] = [eq76_e1992_d_n0, eq76_e1992_d_n1, eq76_e1992_d_n2, eq76_e1992_d_n3, eq76_e1992_d_n4, eq76_e1992_d_n5, eq76_e1992_d_n6, eq76_e1992_d_n7, eq76_e1992_d_n8, eq76_e1992_d_n9, eq76_e1992_d_n10, eq76_e1992_d_n11, eq76_e1992_d_n12, eq76_e1992_d_n13];
        let eq76_branch_derivatives: [f64; 18] = [eq76_e1992_d_b0, eq76_e1992_d_b1, eq76_e1992_d_b2, eq76_e1992_d_b3, eq76_e1992_d_b4, eq76_e1992_d_b5, eq76_e1992_d_b6, eq76_e1992_d_b7, eq76_e1992_d_b8, eq76_e1992_d_b9, eq76_e1992_d_b10, eq76_e1992_d_b11, eq76_e1992_d_b12, eq76_e1992_d_b13, eq76_e1992_d_b14, eq76_e1992_d_b15, eq76_e1992_d_b16, eq76_e1992_d_b17];
        stamper.stamp_current_dense_local(
            Some(6),
            None,
            multiplicity * (eq76_value),
            &eq76_node_derivatives,
            &eq76_branch_derivatives,
            multiplicity,
        );
    }
}
