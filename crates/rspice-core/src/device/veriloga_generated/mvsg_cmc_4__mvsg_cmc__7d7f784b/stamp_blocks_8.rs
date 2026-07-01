#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[908] {
            s.store_scalar(914, 0.0);
            s.store_scalar(915, 0.0);
            s.store_scalar(916, 0.0);
            s.store_scalar(917, 0.0);
            s.copy_ad(918, 60);
            s.copy_ad(919, 61);
            s.store_scalar(920, p.p85);
            s.copy_ad(921, 62);
            s.copy_ad(922, 63);
            s.store_scalar(923, p.p83);
            s.copy_ad(924, 111);
            s.store_scalar(925, s.v[109]);
            s.copy_ad(926, 113);
            s.store_scalar(927, p.p0);
            s.store_scalar(928, p.p79);
            s.copy_ad(929, 20);
            s.store_scalar(930, p.p84);
            s.copy_ad(931, 21);
            s.copy_ad(932, 22);
            s.store_scalar(933, p.p80);
            s.store_scalar(934, p.p94);
            s.store_scalar(935, p.p93);
            s.store_scalar(936, 0.0);
            s.store_scalar(937, p.p95);
            s.store_scalar(938, p.p99);
            s.store_scalar(939, p.p90);
            s.store_scalar(940, p.p91);
            s.store_scalar(941, p.p92);
            s.store_scalar(942, p.p98);
            s.store_scalar(943, p.p97);
            s.store_scalar(944, p.p96);
            s.store_scalar(945, p.p39);
            s.store_scalar(946, p.p47);
            s.store_scalar(947, p.p45);
            s.store_scalar(948, p.p42);
            s.store_scalar(949, p.p2);
            s.store_scalar(950, p.p6);
            s.store_scalar(951, 1.0);
            s.store_scalar(952, 0.0);
            s.store_scalar(953, 0.0);
            s.store_scalar(954, 0.0);
            s.store_scalar(955, 0.0);
            s.store_scalar(956, 0.0);
            s.store_scalar(957, 0.0);
            s.store_scalar(958, 0.0);
            s.store_scalar(959, 0.0);
            s.store_scalar(960, 0.0);
            s.store_scalar(961, 0.0);
            s.store_scalar(962, 0.0);
            s.store_scalar(963, 0.0);
            s.store_scalar(965, 0.0);
            s.store_scalar(966, 0.0);
            s.store_scalar(967, 0.0);
            s.store_scalar(968, 0.0);
            s.store_scalar(969, 0.0);
            s.store_scalar(970, 0.0);
            s.store_scalar(971, 0.0);
            s.store_scalar(972, 0.0);
            s.store_scalar(973, 0.0);
            s.store_scalar(974, 0.0);
            s.store_scalar(975, 0.0);
            s.store_scalar(976, 0.0);
            s.store_scalar(977, 0.0);
            s.store_scalar(978, 0.0);
            s.store_scalar(979, 0.0);
            s.store_scalar(980, 0.0);
            s.store_scalar(981, 0.0);
            s.store_scalar(982, 0.0);
            s.store_scalar(983, 0.0);
            s.store_scalar(984, 0.0);
            s.store_scalar(985, 0.0);
            s.store_scalar(986, 0.0);
            s.store_scalar(987, 0.0);
            s.store_scalar(988, 0.0);
            s.store_scalar(989, 0.0);
            s.store_scalar(990, 0.0);
            s.store_scalar(993, 0.0);
            s.store_scalar(994, 0.0);
            s.store_scalar(995, 0.0);
            s.store_scalar(996, 0.0);
            s.store_scalar(997, 0.0);
            s.store_scalar(998, 0.0);
            s.store_scalar(999, 0.0);
            s.store_scalar(1000, 0.0);
            s.store_scalar(1001, 0.0);
            s.store_scalar(1002, 0.0);
            s.store_scalar(1003, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1007, 0.0);
            s.store_scalar(1008, 0.0);
            s.store_scalar(1009, 0.0);
            s.store_scalar(1010, 0.0);
            s.store_scalar(1011, 0.0);
            s.store_scalar(1012, 0.0);
            s.store_scalar(1013, 0.0);
            s.store_scalar(1014, 0.0);
            s.store_scalar(1015, 0.0);
            s.store_scalar(1016, 0.0);
            s.store_scalar(1017, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
        }

        if s.b[908] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1017, 919, A::tanh_scaled_input(s.ad_value(919), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1017, 919, p.p53);
                } else {
                    s.store_scalar(1017, 0.0);
                }
            }
        }

        if s.b[908] {
            s.store_sub(1018, 918, 919);
            s.store_mul(952, 938, 926);
            s.store_add_scaled_product_value_ad(954, A::div_scaled_inputs(s.ad_value(934), 1.0, s.ad_value(926), 2.302585092994046), 1.0, 937, 1017, 1.0);
            s.store_add_scaled_product_right_sub(955, 933, 1.0, 944, 924, 925, 1.0);
            s.store_pow_ad(973, A::div(s.ad_value(924), s.ad_value(925)), s.ad_value(946));
        }

        s.b[1021] = (s.v[945] != 0.0);
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1021]) {
            s.store_div_ad_rhs(956, 1017, A::pow(A::offset(A::pow(A::div(s.ad_value(1017), s.ad_value(945)), s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941))));
        }

        if (s.b[908] && (!s.b[1021])) {
            s.store_scalar(956, 0.0);
        }

        if s.b[908] {
            s.store_mul_add_scaled_product_rhs(953, 1017, s.ad_value(935), 1.0, s.ad_value(956), s.ad_value(936), (-1.0));
            s.store_sub(916, 955, 953);
            s.store_scaled_mul(958, 954, 926, 2.0);
            s.store_mul(959, 929, 958);
            s.store_sub_scaled_inputs(1016, 916, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1015, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1022] = (s.v[1015] > 50.0);
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1022]) {
            s.store_scalar(974, 0.0);
        }

        s.b[1023] = (s.v[1015] < (-50.0));
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[908] && (!s.b[1022])) && s.b[1023]) {
            s.store_scalar(974, 1.0);
        }

        if ((s.b[908] && (!s.b[1022])) && (!s.b[1023])) {
            s.store_div_from_scalar_offset_ad(974, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(975, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(974), (-(p.p51 * 0.1))), (-1.0), 958, 1.0);
        }

        s.b[1024] = (s.v[975] > 50.0);
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1024]) {
            s.store_mul(976, 959, 975);
        }

        s.b[1025] = (s.v[975] < (-50.0));
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1024])) && s.b[1025]) {
            s.store_mul_exp_rhs(976, 959, 975);
        }

        if ((s.b[908] && (!s.b[1024])) && (!s.b[1025])) {
            s.store_mul_ln_one_plus_exp_rhs(976, 959, 975);
        }

        if s.b[908] {
            s.store_div_ad_rhs(962, 940, A::mul_offset_rhs(s.ad_value(973), A::div_scaled_product(s.ad_value(942), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(963, 939, A::div_scaled_offset_numerator(A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(948), s.ad_value(1017), 1.0, s.ad_value(928), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(943), s.ad_value(976), 1.0, s.ad_value(929), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(980, 963, 928, 1.0, 962, 1.0);
            s.store_add_scaled_product_right_ad(981, 980, (-1.0), 980, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(976), 2.0, s.ad_value(929), s.ad_value(980), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(982, A::mul_sub_from_scalar_rhs(s.ad_value(980), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
            s.store_add_scaled_product_value_ad(917, A::mul_sub_from_scalar_rhs(s.ad_value(981), 1.0, s.ad_value(974)), 1.0, 958, 974, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(983, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::div(s.ad_value(919), s.ad_value(917)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(917))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(917)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(917))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(984, 919, 983);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(985, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(917), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(986, 919, 985);
            s.store_div_scaled_inputs2_indices(1015, 918, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1026] = (s.v[1015] > 50.0);
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1026]) {
            s.store_scalar(957, 0.0);
        }

        s.b[1027] = (s.v[1015] < (-50.0));
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1026])) && s.b[1027]) {
            s.store_scalar(957, 1.0);
        }

        if ((s.b[908] && (!s.b[1026])) && (!s.b[1027])) {
            s.store_div_from_scalar_offset_ad(957, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(960, 1018, 1.0, 986, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(957), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1028] = (s.v[960] > 50.0);
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1028]) {
            s.store_mul(961, 959, 960);
        }

        s.b[1029] = (s.v[960] < (-50.0));
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1028])) && s.b[1029]) {
            s.store_mul_exp_rhs(961, 959, 960);
        }

        if ((s.b[908] && (!s.b[1028])) && (!s.b[1029])) {
            s.store_mul_ln_one_plus_exp_rhs(961, 959, 960);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1015, 1018, 1.0, 1016, (-1.0), 952, 1.0);
        }

        s.b[1030] = (s.v[1015] > 50.0);
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1030]) {
            s.store_scalar(987, 0.0);
        }

        s.b[1031] = (s.v[1015] < (-50.0));
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1030])) && s.b[1031]) {
            s.store_scalar(987, 1.0);
        }

        if ((s.b[908] && (!s.b[1030])) && (!s.b[1031])) {
            s.store_div_from_scalar_offset_ad(987, 1.0, A::exp(s.ad_value(1015)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(988, 918, 1.0, 984, (-1.0), A::add_scaled_product(s.ad_value(916), 1.0, s.ad_value(952), s.ad_value(987), (-(p.p51 * 0.1))), -1.0, 958, 1.0);
        }

        s.b[1032] = (s.v[988] > 50.0);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1032]) {
            s.store_mul(989, 959, 988);
        }

        s.b[1033] = (s.v[988] < (-50.0));
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1032])) && s.b[1033]) {
            s.store_mul_exp_rhs(989, 959, 988);
        }

        if ((s.b[908] && (!s.b[1032])) && (!s.b[1033])) {
            s.store_mul_ln_one_plus_exp_rhs(989, 959, 988);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(990, 961, 1.0, 989, (-1.0), 929, 1.0);
            s.store_div(1016, 990, 982);
            s.store_div_scaled_inputs_indices(965, 934, 1.0, 926, 2.302585092994046);
            s.store_scaled_mul(967, 965, 926, 2.0);
            s.store_mul(968, 929, 967);
            s.store_sub_scaled_inputs(1020, 955, 1.0, 952, (p.p51 * 0.5));
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aii(1019, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1034] = (s.v[1019] > 50.0);
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1034]) {
            s.store_scalar(977, 0.0);
        }

        s.b[1035] = (s.v[1019] < (-50.0));
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1034])) && s.b[1035]) {
            s.store_scalar(977, 1.0);
        }

        if ((s.b[908] && (!s.b[1034])) && (!s.b[1035])) {
            s.store_div_from_scalar_offset_ad(977, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_mixed_aai(978, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sub(s.ad_value(918), s.ad_value(1018)), A::tanh_scaled_input(A::sub(s.ad_value(918), s.ad_value(1018)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(918), 0.5, s.ad_value(1018), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(918), s.ad_value(1018)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(977), (-(p.p51 * 0.1))), (-1.0), 967, 1.0);
        }

        s.b[1036] = (s.v[978] > 50.0);
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1036]) {
            s.store_mul(979, 968, 978);
        }

        s.b[1037] = (s.v[978] < (-50.0));
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1036])) && s.b[1037]) {
            s.store_mul_exp_rhs(979, 968, 978);
        }

        if ((s.b[908] && (!s.b[1036])) && (!s.b[1037])) {
            s.store_mul_ln_one_plus_exp_rhs(979, 968, 978);
        }

        if s.b[908] {
            s.store_div(971, 940, 973);
            s.store_mul_div_scaled_offset_numerator_rhs(972, 939, A::mul(s.ad_value(947), s.ad_value(925)), 1.0, 1.0, A::offset(A::mul(s.ad_value(947), s.ad_value(924)), 1.0), 1.0);
            s.store_div_scaled_product_indices(993, 972, 928, 1.0, 971, 1.0);
            s.store_add_scaled_product_right_ad(994, 993, (-1.0), 993, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(979), 2.0, s.ad_value(929), s.ad_value(993), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(995, A::mul_sub_from_scalar_rhs(s.ad_value(994), 1.0, s.ad_value(977)), 1.0, 967, 977, 1.0);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(996, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::div(s.ad_value(919), s.ad_value(995)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(919), s.ad_value(995))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(919), s.ad_value(995)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(919), s.ad_value(995))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul(997, 919, 996);
        }

        if s.b[908] {
            s.store_div_from_scalar_pow_ad(998, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(919), -1.0, s.ad_value(995), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(941)), 1.0), A::div_from_scalar(1.0, s.ad_value(941)));
        }

        if s.b[908] {
            s.store_mul_neg_lhs(999, 919, 998);
            s.store_div_scaled_inputs2_indices(1019, 918, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1038] = (s.v[1019] > 50.0);
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1038]) {
            s.store_scalar(966, 0.0);
        }

        s.b[1039] = (s.v[1019] < (-50.0));
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1038])) && s.b[1039]) {
            s.store_scalar(966, 1.0);
        }

        if ((s.b[908] && (!s.b[1038])) && (!s.b[1039])) {
            s.store_div_from_scalar_offset_ad(966, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(969, 1018, 1.0, 999, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(966), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1040] = (s.v[969] > 50.0);
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1040]) {
            s.store_mul(970, 968, 969);
        }

        s.b[1041] = (s.v[969] < (-50.0));
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1040])) && s.b[1041]) {
            s.store_mul_exp_rhs(970, 968, 969);
        }

        if ((s.b[908] && (!s.b[1040])) && (!s.b[1041])) {
            s.store_mul_ln_one_plus_exp_rhs(970, 968, 969);
        }

        if s.b[908] {
            s.store_div_scaled_inputs2_indices(1019, 1018, 1.0, 1020, (-1.0), 952, 1.0);
        }

        s.b[1042] = (s.v[1019] > 50.0);
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1042]) {
            s.store_scalar(1000, 0.0);
        }

        s.b[1043] = (s.v[1019] < (-50.0));
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1042])) && s.b[1043]) {
            s.store_scalar(1000, 1.0);
        }

        if ((s.b[908] && (!s.b[1042])) && (!s.b[1043])) {
            s.store_div_from_scalar_offset_ad(1000, 1.0, A::exp(s.ad_value(1019)), 1.0);
        }

        if s.b[908] {
            s.store_div_scaled_inputs3_mixed_iiai(1001, 918, 1.0, 997, (-1.0), A::add_scaled_product(s.ad_value(955), 1.0, s.ad_value(952), s.ad_value(1000), (-(p.p51 * 0.1))), -1.0, 967, 1.0);
        }

        s.b[1044] = (s.v[1001] > 50.0);
        s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1044]) {
            s.store_mul(1002, 968, 1001);
        }

        s.b[1045] = (s.v[1001] < (-50.0));
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if ((s.b[908] && (!s.b[1044])) && s.b[1045]) {
            s.store_mul_exp_rhs(1002, 968, 1001);
        }

        if ((s.b[908] && (!s.b[1044])) && (!s.b[1045])) {
            s.store_mul_ln_one_plus_exp_rhs(1002, 968, 1001);
        }

        if s.b[908] {
            s.store_offset_square(1003, 970, 1e-38);
            s.store_offset_mul(1004, 1003, 970, 1e-57);
            s.store_offset_square(1005, 1002, 1e-38);
            s.store_offset_mul(1006, 1005, 1002, 1e-57);
            s.store_offset_mul(1007, 970, 1002, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1008, 1003, (2.0 / 3.0), 1005, (2.0 / 3.0), 1007, (2.0 / 3.0), A::offset(A::add(s.ad_value(970), s.ad_value(1002)), 2e-19), 1.0);
            s.store_div_ad(1009, A::add_scaled_inputs_products(s.ad_value(1004), (2.0 * 2.0), s.ad_value(1006), (3.0 * 2.0), s.ad_value(1003), s.ad_value(1002), (4.0 * 2.0), s.ad_value(1005), s.ad_value(970), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1003), 15.0, s.ad_value(1005), 15.0, s.ad_value(1007), (2.0 * 15.0)));
            s.store_sub(1010, 1008, 1009);
            s.copy_ad(1011, 1009);
            s.store_mul_product3_mixed_iaii(911, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1010, 1.0);
            s.store_mul_product3_mixed_iaii(912, 951, A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(928)), 950, 1011, 1.0);
        }

        s.b[1046] = (s.v[920] == 1.0);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1046]) {
            s.store_div_scaled_inputs3_indices(1012, 921, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1047] = (s.v[1012] > 50.0);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1047]) {
            s.copy_ad(1015, 1012);
        }

        s.b[1048] = (s.v[1012] < (-50.0));
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && s.b[1048]) {
            s.store_exp(1015, 1012);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1047])) && (!s.b[1048])) {
            s.store_ln_one_plus_exp(1015, 1012);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(913, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(931), s.ad_value(967)), 1015, 951);
            s.store_div_scaled_inputs3_indices(1013, 922, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1049] = (s.v[1013] > 50.0);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1046]) && s.b[1049]) {
            s.copy_ad(1015, 1013);
        }

        s.b[1050] = (s.v[1013] < (-50.0));
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && s.b[1050]) {
            s.store_exp(1015, 1013);
        }

        if (((s.b[908] && s.b[1046]) && (!s.b[1049])) && (!s.b[1050])) {
            s.store_ln_one_plus_exp(1015, 1013);
        }

        if (s.b[908] && s.b[1046]) {
            s.store_mul_ad_product_lhs_mixed_ai(914, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(932), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1046])) {
            s.store_scalar(913, 0.0);
            s.store_scalar(914, 0.0);
        }

        s.b[1051] = (s.v[923] == 1.0);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[908] && s.b[1051]) {
            s.store_div_scaled_inputs3_indices(1014, 918, 1.0, 955, -1.0, 952, (-(-(p.p51 * 0.5))), 967, 1.0);
        }

        s.b[1052] = (s.v[1014] > 50.0);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if ((s.b[908] && s.b[1051]) && s.b[1052]) {
            s.copy_ad(1015, 1014);
        }

        s.b[1053] = (s.v[1014] < (-50.0));
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && s.b[1053]) {
            s.store_exp(1015, 1014);
        }

        if (((s.b[908] && s.b[1051]) && (!s.b[1052])) && (!s.b[1053])) {
            s.store_ln_one_plus_exp(1015, 1014);
        }

        if (s.b[908] && s.b[1051]) {
            s.store_mul_ad_product_lhs_mixed_ai(915, A::mul3(A::mul3(s.ad_value(927), s.ad_value(949), s.ad_value(950)), s.ad_value(930), s.ad_value(967)), 1015, 951);
        }

        if (s.b[908] && (!s.b[1051])) {
            s.store_scalar(915, 0.0);
        }

        if s.b[908] {
            s.copy_ad(167, 911);
            s.copy_ad(168, 912);
            s.copy_ad(169, 913);
            s.copy_ad(170, 914);
            s.copy_ad(171, 915);
        }

        s.b[1054] = (p.p78 == 1.0);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        s.store_scalar(173, 0.0);

        s.store_scalar(174, 0.0);

        s.store_scalar(175, 0.0);

        s.store_scalar(176, 0.0);

        s.store_scalar(177, 0.0);

        s.b[1055] = (p.p101 > p.p354);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if s.b[1055] {
            s.store_scalar(1058, 0.0);
            s.store_scalar(1059, 0.0);
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
            s.store_scalar(1062, 0.0);
            s.store_scalar(1063, 0.0);
            s.store_scalar(1064, 0.0);
            s.copy_ad(1065, 66);
            s.copy_ad(1066, 67);
            s.store_scalar(1067, p.p107);
            s.copy_ad(1068, 68);
            s.copy_ad(1069, 69);
            s.store_scalar(1070, p.p105);
            s.copy_ad(1071, 111);
            s.store_scalar(1072, s.v[109]);
            s.copy_ad(1073, 113);
            s.store_scalar(1074, p.p0);
            s.store_scalar(1075, p.p101);
            s.copy_ad(1076, 23);
            s.store_scalar(1077, p.p106);
            s.copy_ad(1078, 24);
            s.copy_ad(1079, 25);
            s.store_scalar(1080, p.p102);
            s.store_scalar(1081, p.p116);
            s.store_scalar(1082, p.p115);
            s.store_scalar(1083, 0.0);
            s.store_scalar(1084, p.p117);
            s.store_scalar(1085, p.p121);
            s.store_scalar(1086, p.p112);
            s.store_scalar(1087, p.p113);
            s.store_scalar(1088, p.p114);
            s.store_scalar(1089, p.p120);
            s.store_scalar(1090, p.p119);
            s.store_scalar(1091, p.p118);
            s.store_scalar(1092, p.p39);
            s.store_scalar(1093, p.p47);
            s.store_scalar(1094, p.p45);
            s.store_scalar(1095, p.p42);
            s.store_scalar(1096, p.p2);
            s.store_scalar(1097, p.p6);
            s.store_scalar(1098, 1.0);
            s.store_scalar(1099, 0.0);
            s.store_scalar(1100, 0.0);
            s.store_scalar(1101, 0.0);
            s.store_scalar(1102, 0.0);
            s.store_scalar(1103, 0.0);
            s.store_scalar(1104, 0.0);
            s.store_scalar(1105, 0.0);
            s.store_scalar(1106, 0.0);
            s.store_scalar(1107, 0.0);
            s.store_scalar(1108, 0.0);
            s.store_scalar(1109, 0.0);
            s.store_scalar(1110, 0.0);
            s.store_scalar(1112, 0.0);
            s.store_scalar(1113, 0.0);
            s.store_scalar(1114, 0.0);
            s.store_scalar(1115, 0.0);
            s.store_scalar(1116, 0.0);
            s.store_scalar(1117, 0.0);
            s.store_scalar(1118, 0.0);
            s.store_scalar(1119, 0.0);
            s.store_scalar(1120, 0.0);
            s.store_scalar(1121, 0.0);
            s.store_scalar(1122, 0.0);
            s.store_scalar(1123, 0.0);
            s.store_scalar(1124, 0.0);
            s.store_scalar(1125, 0.0);
            s.store_scalar(1126, 0.0);
            s.store_scalar(1127, 0.0);
            s.store_scalar(1128, 0.0);
            s.store_scalar(1129, 0.0);
            s.store_scalar(1130, 0.0);
            s.store_scalar(1131, 0.0);
            s.store_scalar(1132, 0.0);
            s.store_scalar(1133, 0.0);
            s.store_scalar(1134, 0.0);
            s.store_scalar(1135, 0.0);
            s.store_scalar(1136, 0.0);
            s.store_scalar(1137, 0.0);
            s.store_scalar(1140, 0.0);
            s.store_scalar(1141, 0.0);
            s.store_scalar(1142, 0.0);
            s.store_scalar(1143, 0.0);
            s.store_scalar(1144, 0.0);
            s.store_scalar(1145, 0.0);
            s.store_scalar(1146, 0.0);
            s.store_scalar(1147, 0.0);
            s.store_scalar(1148, 0.0);
            s.store_scalar(1149, 0.0);
            s.store_scalar(1150, 0.0);
            s.store_scalar(1151, 0.0);
            s.store_scalar(1152, 0.0);
            s.store_scalar(1153, 0.0);
            s.store_scalar(1154, 0.0);
            s.store_scalar(1155, 0.0);
            s.store_scalar(1156, 0.0);
            s.store_scalar(1157, 0.0);
            s.store_scalar(1158, 0.0);
            s.store_scalar(1159, 0.0);
            s.store_scalar(1160, 0.0);
            s.store_scalar(1161, 0.0);
            s.store_scalar(1162, 0.0);
            s.store_scalar(1163, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1055] {
            s.store_scalar(1164, 0.0);
            s.store_scalar(1165, 0.0);
            s.store_scalar(1166, 0.0);
            s.store_scalar(1167, 0.0);
        }

        if s.b[1055] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1164, 1066, A::tanh_scaled_input(s.ad_value(1066), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1164, 1066, p.p53);
                } else {
                    s.store_scalar(1164, 0.0);
                }
            }
        }

        if s.b[1055] {
            s.store_sub(1165, 1065, 1066);
            s.store_mul(1099, 1085, 1073);
            s.store_add_scaled_product_value_ad(1101, A::div_scaled_inputs(s.ad_value(1081), 1.0, s.ad_value(1073), 2.302585092994046), 1.0, 1084, 1164, 1.0);
            s.store_add_scaled_product_right_sub(1102, 1080, 1.0, 1091, 1071, 1072, 1.0);
            s.store_pow_ad(1120, A::div(s.ad_value(1071), s.ad_value(1072)), s.ad_value(1093));
        }

        s.b[1168] = (s.v[1092] != 0.0);
        s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1168]) {
            s.store_div_ad_rhs(1103, 1164, A::pow(A::offset(A::pow(A::div(s.ad_value(1164), s.ad_value(1092)), s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088))));
        }

        if (s.b[1055] && (!s.b[1168])) {
            s.store_scalar(1103, 0.0);
        }

        if s.b[1055] {
            s.store_mul_add_scaled_product_rhs(1100, 1164, s.ad_value(1082), 1.0, s.ad_value(1103), s.ad_value(1083), (-1.0));
            s.store_sub(1063, 1102, 1100);
            s.store_scaled_mul(1105, 1101, 1073, 2.0);
            s.store_mul(1106, 1076, 1105);
            s.store_sub_scaled_inputs(1163, 1063, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1162, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1169] = (s.v[1162] > 50.0);
        s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1169]) {
            s.store_scalar(1121, 0.0);
        }

        s.b[1170] = (s.v[1162] < (-50.0));
        s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1169])) && s.b[1170]) {
            s.store_scalar(1121, 1.0);
        }

        if ((s.b[1055] && (!s.b[1169])) && (!s.b[1170])) {
            s.store_div_from_scalar_offset_ad(1121, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1122, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1121), (-(p.p51 * 0.1))), (-1.0), 1105, 1.0);
        }

        s.b[1171] = (s.v[1122] > 50.0);
        s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1171]) {
            s.store_mul(1123, 1106, 1122);
        }

        s.b[1172] = (s.v[1122] < (-50.0));
        s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1171])) && s.b[1172]) {
            s.store_mul_exp_rhs(1123, 1106, 1122);
        }

        if ((s.b[1055] && (!s.b[1171])) && (!s.b[1172])) {
            s.store_mul_ln_one_plus_exp_rhs(1123, 1106, 1122);
        }

        if s.b[1055] {
            s.store_div_ad_rhs(1109, 1087, A::mul_offset_rhs(s.ad_value(1120), A::div_scaled_product(s.ad_value(1089), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1110, 1086, A::div_scaled_offset_numerator(A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1095), s.ad_value(1164), 1.0, s.ad_value(1075), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1090), s.ad_value(1123), 1.0, s.ad_value(1076), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1127, 1110, 1075, 1.0, 1109, 1.0);
            s.store_add_scaled_product_right_ad(1128, 1127, (-1.0), 1127, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1123), 2.0, s.ad_value(1076), s.ad_value(1127), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1129, A::mul_sub_from_scalar_rhs(s.ad_value(1127), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
            s.store_add_scaled_product_value_ad(1064, A::mul_sub_from_scalar_rhs(s.ad_value(1128), 1.0, s.ad_value(1121)), 1.0, 1105, 1121, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1130, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::div(s.ad_value(1066), s.ad_value(1064)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1064)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1064))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1131, 1066, 1130);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1132, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1064), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1133, 1066, 1132);
            s.store_div_scaled_inputs2_indices(1162, 1065, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1173] = (s.v[1162] > 50.0);
        s.store_scalar(1173, if s.b[1173] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1173]) {
            s.store_scalar(1104, 0.0);
        }

        s.b[1174] = (s.v[1162] < (-50.0));
        s.store_scalar(1174, if s.b[1174] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1173])) && s.b[1174]) {
            s.store_scalar(1104, 1.0);
        }

        if ((s.b[1055] && (!s.b[1173])) && (!s.b[1174])) {
            s.store_div_from_scalar_offset_ad(1104, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1107, 1165, 1.0, 1133, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1104), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1175] = (s.v[1107] > 50.0);
        s.store_scalar(1175, if s.b[1175] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1175]) {
            s.store_mul(1108, 1106, 1107);
        }

        s.b[1176] = (s.v[1107] < (-50.0));
        s.store_scalar(1176, if s.b[1176] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1175])) && s.b[1176]) {
            s.store_mul_exp_rhs(1108, 1106, 1107);
        }

        if ((s.b[1055] && (!s.b[1175])) && (!s.b[1176])) {
            s.store_mul_ln_one_plus_exp_rhs(1108, 1106, 1107);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1162, 1165, 1.0, 1163, (-1.0), 1099, 1.0);
        }

        s.b[1177] = (s.v[1162] > 50.0);
        s.store_scalar(1177, if s.b[1177] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1177]) {
            s.store_scalar(1134, 0.0);
        }

        s.b[1178] = (s.v[1162] < (-50.0));
        s.store_scalar(1178, if s.b[1178] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1177])) && s.b[1178]) {
            s.store_scalar(1134, 1.0);
        }

        if ((s.b[1055] && (!s.b[1177])) && (!s.b[1178])) {
            s.store_div_from_scalar_offset_ad(1134, 1.0, A::exp(s.ad_value(1162)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1135, 1065, 1.0, 1131, (-1.0), A::add_scaled_product(s.ad_value(1063), 1.0, s.ad_value(1099), s.ad_value(1134), (-(p.p51 * 0.1))), -1.0, 1105, 1.0);
        }

        s.b[1179] = (s.v[1135] > 50.0);
        s.store_scalar(1179, if s.b[1179] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1179]) {
            s.store_mul(1136, 1106, 1135);
        }

        s.b[1180] = (s.v[1135] < (-50.0));
        s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1179])) && s.b[1180]) {
            s.store_mul_exp_rhs(1136, 1106, 1135);
        }

        if ((s.b[1055] && (!s.b[1179])) && (!s.b[1180])) {
            s.store_mul_ln_one_plus_exp_rhs(1136, 1106, 1135);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1137, 1108, 1.0, 1136, (-1.0), 1076, 1.0);
            s.store_div(1163, 1137, 1129);
            s.store_div_scaled_inputs_indices(1112, 1081, 1.0, 1073, 2.302585092994046);
            s.store_scaled_mul(1114, 1112, 1073, 2.0);
            s.store_mul(1115, 1076, 1114);
            s.store_sub_scaled_inputs(1167, 1102, 1.0, 1099, (p.p51 * 0.5));
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aii(1166, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1181] = (s.v[1166] > 50.0);
        s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1181]) {
            s.store_scalar(1124, 0.0);
        }

        s.b[1182] = (s.v[1166] < (-50.0));
        s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1181])) && s.b[1182]) {
            s.store_scalar(1124, 1.0);
        }

        if ((s.b[1055] && (!s.b[1181])) && (!s.b[1182])) {
            s.store_div_from_scalar_offset_ad(1124, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_mixed_aai(1125, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sub(s.ad_value(1065), s.ad_value(1165)), A::tanh_scaled_input(A::sub(s.ad_value(1065), s.ad_value(1165)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1065), 0.5, s.ad_value(1165), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1065), s.ad_value(1165)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1124), (-(p.p51 * 0.1))), (-1.0), 1114, 1.0);
        }

        s.b[1183] = (s.v[1125] > 50.0);
        s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1183]) {
            s.store_mul(1126, 1115, 1125);
        }

        s.b[1184] = (s.v[1125] < (-50.0));
        s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1183])) && s.b[1184]) {
            s.store_mul_exp_rhs(1126, 1115, 1125);
        }

        if ((s.b[1055] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_mul_ln_one_plus_exp_rhs(1126, 1115, 1125);
        }

        if s.b[1055] {
            s.store_div(1118, 1087, 1120);
            s.store_mul_div_scaled_offset_numerator_rhs(1119, 1086, A::mul(s.ad_value(1094), s.ad_value(1072)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1094), s.ad_value(1071)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1140, 1119, 1075, 1.0, 1118, 1.0);
            s.store_add_scaled_product_right_ad(1141, 1140, (-1.0), 1140, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1126), 2.0, s.ad_value(1076), s.ad_value(1140), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1142, A::mul_sub_from_scalar_rhs(s.ad_value(1141), 1.0, s.ad_value(1124)), 1.0, 1114, 1124, 1.0);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1143, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::div(s.ad_value(1066), s.ad_value(1142)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1066), s.ad_value(1142)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1066), s.ad_value(1142))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul(1144, 1066, 1143);
        }

        if s.b[1055] {
            s.store_div_from_scalar_pow_ad(1145, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1066), -1.0, s.ad_value(1142), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1088)), 1.0), A::div_from_scalar(1.0, s.ad_value(1088)));
        }

        if s.b[1055] {
            s.store_mul_neg_lhs(1146, 1066, 1145);
            s.store_div_scaled_inputs2_indices(1166, 1065, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1185] = (s.v[1166] > 50.0);
        s.store_scalar(1185, if s.b[1185] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1185]) {
            s.store_scalar(1113, 0.0);
        }

        s.b[1186] = (s.v[1166] < (-50.0));
        s.store_scalar(1186, if s.b[1186] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1185])) && s.b[1186]) {
            s.store_scalar(1113, 1.0);
        }

        if ((s.b[1055] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_div_from_scalar_offset_ad(1113, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1116, 1165, 1.0, 1146, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1113), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1187] = (s.v[1116] > 50.0);
        s.store_scalar(1187, if s.b[1187] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1187]) {
            s.store_mul(1117, 1115, 1116);
        }

        s.b[1188] = (s.v[1116] < (-50.0));
        s.store_scalar(1188, if s.b[1188] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1187])) && s.b[1188]) {
            s.store_mul_exp_rhs(1117, 1115, 1116);
        }

        if ((s.b[1055] && (!s.b[1187])) && (!s.b[1188])) {
            s.store_mul_ln_one_plus_exp_rhs(1117, 1115, 1116);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs2_indices(1166, 1165, 1.0, 1167, (-1.0), 1099, 1.0);
        }

        s.b[1189] = (s.v[1166] > 50.0);
        s.store_scalar(1189, if s.b[1189] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1189]) {
            s.store_scalar(1147, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1190] = (s.v[1166] < (-50.0));
        s.store_scalar(1190, if s.b[1190] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1189])) && s.b[1190]) {
            s.store_scalar(1147, 1.0);
        }

        if ((s.b[1055] && (!s.b[1189])) && (!s.b[1190])) {
            s.store_div_from_scalar_offset_ad(1147, 1.0, A::exp(s.ad_value(1166)), 1.0);
        }

        if s.b[1055] {
            s.store_div_scaled_inputs3_mixed_iiai(1148, 1065, 1.0, 1144, (-1.0), A::add_scaled_product(s.ad_value(1102), 1.0, s.ad_value(1099), s.ad_value(1147), (-(p.p51 * 0.1))), -1.0, 1114, 1.0);
        }

        s.b[1191] = (s.v[1148] > 50.0);
        s.store_scalar(1191, if s.b[1191] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1191]) {
            s.store_mul(1149, 1115, 1148);
        }

        s.b[1192] = (s.v[1148] < (-50.0));
        s.store_scalar(1192, if s.b[1192] { 1.0 } else { 0.0 });

        if ((s.b[1055] && (!s.b[1191])) && s.b[1192]) {
            s.store_mul_exp_rhs(1149, 1115, 1148);
        }

        if ((s.b[1055] && (!s.b[1191])) && (!s.b[1192])) {
            s.store_mul_ln_one_plus_exp_rhs(1149, 1115, 1148);
        }

        if s.b[1055] {
            s.store_offset_square(1150, 1117, 1e-38);
            s.store_offset_mul(1151, 1150, 1117, 1e-57);
            s.store_offset_square(1152, 1149, 1e-38);
            s.store_offset_mul(1153, 1152, 1149, 1e-57);
            s.store_offset_mul(1154, 1117, 1149, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1155, 1150, (2.0 / 3.0), 1152, (2.0 / 3.0), 1154, (2.0 / 3.0), A::offset(A::add(s.ad_value(1117), s.ad_value(1149)), 2e-19), 1.0);
            s.store_div_ad(1156, A::add_scaled_inputs_products(s.ad_value(1151), (2.0 * 2.0), s.ad_value(1153), (3.0 * 2.0), s.ad_value(1150), s.ad_value(1149), (4.0 * 2.0), s.ad_value(1152), s.ad_value(1117), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1150), 15.0, s.ad_value(1152), 15.0, s.ad_value(1154), (2.0 * 15.0)));
            s.store_sub(1157, 1155, 1156);
            s.copy_ad(1158, 1156);
            s.store_mul_product3_mixed_iaii(1058, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1157, 1.0);
            s.store_mul_product3_mixed_iaii(1059, 1098, A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1075)), 1097, 1158, 1.0);
        }

        s.b[1193] = (s.v[1067] == 1.0);
        s.store_scalar(1193, if s.b[1193] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1193]) {
            s.store_div_scaled_inputs3_indices(1159, 1068, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1194] = (s.v[1159] > 50.0);
        s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1194]) {
            s.copy_ad(1162, 1159);
        }

        s.b[1195] = (s.v[1159] < (-50.0));
        s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && s.b[1195]) {
            s.store_exp(1162, 1159);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1194])) && (!s.b[1195])) {
            s.store_ln_one_plus_exp(1162, 1159);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1060, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1078), s.ad_value(1114)), 1162, 1098);
            s.store_div_scaled_inputs3_indices(1160, 1069, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1196] = (s.v[1160] > 50.0);
        s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1193]) && s.b[1196]) {
            s.copy_ad(1162, 1160);
        }

        s.b[1197] = (s.v[1160] < (-50.0));
        s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && s.b[1197]) {
            s.store_exp(1162, 1160);
        }

        if (((s.b[1055] && s.b[1193]) && (!s.b[1196])) && (!s.b[1197])) {
            s.store_ln_one_plus_exp(1162, 1160);
        }

        if (s.b[1055] && s.b[1193]) {
            s.store_mul_ad_product_lhs_mixed_ai(1061, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1079), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1193])) {
            s.store_scalar(1060, 0.0);
            s.store_scalar(1061, 0.0);
        }

        s.b[1198] = (s.v[1070] == 1.0);
        s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });

        if (s.b[1055] && s.b[1198]) {
            s.store_div_scaled_inputs3_indices(1161, 1065, 1.0, 1102, -1.0, 1099, (-(-(p.p51 * 0.5))), 1114, 1.0);
        }

        s.b[1199] = (s.v[1161] > 50.0);
        s.store_scalar(1199, if s.b[1199] { 1.0 } else { 0.0 });

        if ((s.b[1055] && s.b[1198]) && s.b[1199]) {
            s.copy_ad(1162, 1161);
        }

        s.b[1200] = (s.v[1161] < (-50.0));
        s.store_scalar(1200, if s.b[1200] { 1.0 } else { 0.0 });

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && s.b[1200]) {
            s.store_exp(1162, 1161);
        }

        if (((s.b[1055] && s.b[1198]) && (!s.b[1199])) && (!s.b[1200])) {
            s.store_ln_one_plus_exp(1162, 1161);
        }

        if (s.b[1055] && s.b[1198]) {
            s.store_mul_ad_product_lhs_mixed_ai(1062, A::mul3(A::mul3(s.ad_value(1074), s.ad_value(1096), s.ad_value(1097)), s.ad_value(1077), s.ad_value(1114)), 1162, 1098);
        }

        if (s.b[1055] && (!s.b[1198])) {
            s.store_scalar(1062, 0.0);
        }

        if s.b[1055] {
            s.copy_ad(173, 1058);
            s.copy_ad(174, 1059);
            s.copy_ad(175, 1060);
            s.copy_ad(176, 1061);
            s.copy_ad(177, 1062);
        }

        s.b[1201] = (p.p100 == 1.0);
        s.store_scalar(1201, if s.b[1201] { 1.0 } else { 0.0 });

        s.store_scalar(179, 0.0);

        s.store_scalar(180, 0.0);

        s.store_scalar(181, 0.0);

        s.store_scalar(182, 0.0);

        s.store_scalar(183, 0.0);

        s.b[1202] = (p.p123 > p.p354);
        s.store_scalar(1202, if s.b[1202] { 1.0 } else { 0.0 });

        if s.b[1202] {
            s.store_scalar(1205, 0.0);
            s.store_scalar(1206, 0.0);
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
            s.store_scalar(1209, 0.0);
            s.store_scalar(1210, 0.0);
            s.store_scalar(1211, 0.0);
            s.copy_ad(1212, 72);
            s.copy_ad(1213, 73);
            s.store_scalar(1214, p.p129);
            s.copy_ad(1215, 74);
            s.copy_ad(1216, 75);
            s.store_scalar(1217, p.p127);
            s.copy_ad(1218, 111);
            s.store_scalar(1219, s.v[109]);
            s.copy_ad(1220, 113);
            s.store_scalar(1221, p.p0);
            s.store_scalar(1222, p.p123);
            s.copy_ad(1223, 26);
            s.store_scalar(1224, p.p128);
            s.copy_ad(1225, 27);
            s.copy_ad(1226, 28);
            s.store_scalar(1227, p.p124);
            s.store_scalar(1228, p.p138);
            s.store_scalar(1229, p.p137);
            s.store_scalar(1230, 0.0);
            s.store_scalar(1231, p.p139);
            s.store_scalar(1232, p.p143);
            s.store_scalar(1233, p.p134);
            s.store_scalar(1234, p.p135);
            s.store_scalar(1235, p.p136);
            s.store_scalar(1236, p.p142);
            s.store_scalar(1237, p.p141);
            s.store_scalar(1238, p.p140);
            s.store_scalar(1239, p.p39);
            s.store_scalar(1240, p.p47);
            s.store_scalar(1241, p.p45);
            s.store_scalar(1242, p.p42);
            s.store_scalar(1243, p.p2);
            s.store_scalar(1244, p.p6);
            s.store_scalar(1245, 1.0);
            s.store_scalar(1246, 0.0);
            s.store_scalar(1247, 0.0);
            s.store_scalar(1248, 0.0);
            s.store_scalar(1249, 0.0);
            s.store_scalar(1250, 0.0);
            s.store_scalar(1251, 0.0);
            s.store_scalar(1252, 0.0);
            s.store_scalar(1253, 0.0);
            s.store_scalar(1254, 0.0);
            s.store_scalar(1255, 0.0);
            s.store_scalar(1256, 0.0);
            s.store_scalar(1257, 0.0);
            s.store_scalar(1259, 0.0);
            s.store_scalar(1260, 0.0);
            s.store_scalar(1261, 0.0);
            s.store_scalar(1262, 0.0);
            s.store_scalar(1263, 0.0);
            s.store_scalar(1264, 0.0);
            s.store_scalar(1265, 0.0);
            s.store_scalar(1266, 0.0);
            s.store_scalar(1267, 0.0);
            s.store_scalar(1268, 0.0);
            s.store_scalar(1269, 0.0);
            s.store_scalar(1270, 0.0);
            s.store_scalar(1271, 0.0);
            s.store_scalar(1272, 0.0);
            s.store_scalar(1273, 0.0);
            s.store_scalar(1274, 0.0);
            s.store_scalar(1275, 0.0);
            s.store_scalar(1276, 0.0);
            s.store_scalar(1277, 0.0);
            s.store_scalar(1278, 0.0);
            s.store_scalar(1279, 0.0);
            s.store_scalar(1280, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_scalar(1281, 0.0);
            s.store_scalar(1282, 0.0);
            s.store_scalar(1283, 0.0);
            s.store_scalar(1284, 0.0);
            s.store_scalar(1287, 0.0);
            s.store_scalar(1288, 0.0);
            s.store_scalar(1289, 0.0);
            s.store_scalar(1290, 0.0);
            s.store_scalar(1291, 0.0);
            s.store_scalar(1292, 0.0);
            s.store_scalar(1293, 0.0);
            s.store_scalar(1294, 0.0);
            s.store_scalar(1295, 0.0);
            s.store_scalar(1296, 0.0);
            s.store_scalar(1297, 0.0);
            s.store_scalar(1298, 0.0);
            s.store_scalar(1299, 0.0);
            s.store_scalar(1300, 0.0);
            s.store_scalar(1301, 0.0);
            s.store_scalar(1302, 0.0);
            s.store_scalar(1303, 0.0);
            s.store_scalar(1304, 0.0);
            s.store_scalar(1305, 0.0);
            s.store_scalar(1306, 0.0);
            s.store_scalar(1307, 0.0);
            s.store_scalar(1308, 0.0);
            s.store_scalar(1309, 0.0);
            s.store_scalar(1310, 0.0);
            s.store_scalar(1311, 0.0);
            s.store_scalar(1312, 0.0);
            s.store_scalar(1313, 0.0);
            s.store_scalar(1314, 0.0);
        }

        if s.b[1202] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1311, 1213, A::tanh_scaled_input(s.ad_value(1213), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1311, 1213, p.p53);
                } else {
                    s.store_scalar(1311, 0.0);
                }
            }
        }

        if s.b[1202] {
            s.store_sub(1312, 1212, 1213);
            s.store_mul(1246, 1232, 1220);
            s.store_add_scaled_product_value_ad(1248, A::div_scaled_inputs(s.ad_value(1228), 1.0, s.ad_value(1220), 2.302585092994046), 1.0, 1231, 1311, 1.0);
            s.store_add_scaled_product_right_sub(1249, 1227, 1.0, 1238, 1218, 1219, 1.0);
            s.store_pow_ad(1267, A::div(s.ad_value(1218), s.ad_value(1219)), s.ad_value(1240));
        }

        s.b[1315] = (s.v[1239] != 0.0);
        s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1315]) {
            s.store_div_ad_rhs(1250, 1311, A::pow(A::offset(A::pow(A::div(s.ad_value(1311), s.ad_value(1239)), s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235))));
        }

        if (s.b[1202] && (!s.b[1315])) {
            s.store_scalar(1250, 0.0);
        }

        if s.b[1202] {
            s.store_mul_add_scaled_product_rhs(1247, 1311, s.ad_value(1229), 1.0, s.ad_value(1250), s.ad_value(1230), (-1.0));
            s.store_sub(1210, 1249, 1247);
            s.store_scaled_mul(1252, 1248, 1220, 2.0);
            s.store_mul(1253, 1223, 1252);
            s.store_sub_scaled_inputs(1310, 1210, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1309, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1316] = (s.v[1309] > 50.0);
        s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1316]) {
            s.store_scalar(1268, 0.0);
        }

        s.b[1317] = (s.v[1309] < (-50.0));
        s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(1268, 1.0);
        }

        if ((s.b[1202] && (!s.b[1316])) && (!s.b[1317])) {
            s.store_div_from_scalar_offset_ad(1268, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1269, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1268), (-(p.p51 * 0.1))), (-1.0), 1252, 1.0);
        }

        s.b[1318] = (s.v[1269] > 50.0);
        s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1318]) {
            s.store_mul(1270, 1253, 1269);
        }

        s.b[1319] = (s.v[1269] < (-50.0));
        s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1318])) && s.b[1319]) {
            s.store_mul_exp_rhs(1270, 1253, 1269);
        }

        if ((s.b[1202] && (!s.b[1318])) && (!s.b[1319])) {
            s.store_mul_ln_one_plus_exp_rhs(1270, 1253, 1269);
        }

        if s.b[1202] {
            s.store_div_ad_rhs(1256, 1234, A::mul_offset_rhs(s.ad_value(1267), A::div_scaled_product(s.ad_value(1236), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1257, 1233, A::div_scaled_offset_numerator(A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1242), s.ad_value(1311), 1.0, s.ad_value(1222), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1237), s.ad_value(1270), 1.0, s.ad_value(1223), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1274, 1257, 1222, 1.0, 1256, 1.0);
            s.store_add_scaled_product_right_ad(1275, 1274, (-1.0), 1274, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1270), 2.0, s.ad_value(1223), s.ad_value(1274), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1276, A::mul_sub_from_scalar_rhs(s.ad_value(1274), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
            s.store_add_scaled_product_value_ad(1211, A::mul_sub_from_scalar_rhs(s.ad_value(1275), 1.0, s.ad_value(1268)), 1.0, 1252, 1268, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1277, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::div(s.ad_value(1213), s.ad_value(1211)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1211)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1211))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1278, 1213, 1277);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1279, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1211), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1280, 1213, 1279);
            s.store_div_scaled_inputs2_indices(1309, 1212, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1320] = (s.v[1309] > 50.0);
        s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1320]) {
            s.store_scalar(1251, 0.0);
        }

        s.b[1321] = (s.v[1309] < (-50.0));
        s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1320])) && s.b[1321]) {
            s.store_scalar(1251, 1.0);
        }

        if ((s.b[1202] && (!s.b[1320])) && (!s.b[1321])) {
            s.store_div_from_scalar_offset_ad(1251, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1254, 1312, 1.0, 1280, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1251), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1322] = (s.v[1254] > 50.0);
        s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1322]) {
            s.store_mul(1255, 1253, 1254);
        }

        s.b[1323] = (s.v[1254] < (-50.0));
        s.store_scalar(1323, if s.b[1323] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1322])) && s.b[1323]) {
            s.store_mul_exp_rhs(1255, 1253, 1254);
        }

        if ((s.b[1202] && (!s.b[1322])) && (!s.b[1323])) {
            s.store_mul_ln_one_plus_exp_rhs(1255, 1253, 1254);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1309, 1312, 1.0, 1310, (-1.0), 1246, 1.0);
        }

        s.b[1324] = (s.v[1309] > 50.0);
        s.store_scalar(1324, if s.b[1324] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1324]) {
            s.store_scalar(1281, 0.0);
        }

        s.b[1325] = (s.v[1309] < (-50.0));
        s.store_scalar(1325, if s.b[1325] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1324])) && s.b[1325]) {
            s.store_scalar(1281, 1.0);
        }

        if ((s.b[1202] && (!s.b[1324])) && (!s.b[1325])) {
            s.store_div_from_scalar_offset_ad(1281, 1.0, A::exp(s.ad_value(1309)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1282, 1212, 1.0, 1278, (-1.0), A::add_scaled_product(s.ad_value(1210), 1.0, s.ad_value(1246), s.ad_value(1281), (-(p.p51 * 0.1))), -1.0, 1252, 1.0);
        }

        s.b[1326] = (s.v[1282] > 50.0);
        s.store_scalar(1326, if s.b[1326] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1326]) {
            s.store_mul(1283, 1253, 1282);
        }

        s.b[1327] = (s.v[1282] < (-50.0));
        s.store_scalar(1327, if s.b[1327] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1326])) && s.b[1327]) {
            s.store_mul_exp_rhs(1283, 1253, 1282);
        }

        if ((s.b[1202] && (!s.b[1326])) && (!s.b[1327])) {
            s.store_mul_ln_one_plus_exp_rhs(1283, 1253, 1282);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1284, 1255, 1.0, 1283, (-1.0), 1223, 1.0);
            s.store_div(1310, 1284, 1276);
            s.store_div_scaled_inputs_indices(1259, 1228, 1.0, 1220, 2.302585092994046);
            s.store_scaled_mul(1261, 1259, 1220, 2.0);
            s.store_mul(1262, 1223, 1261);
            s.store_sub_scaled_inputs(1314, 1249, 1.0, 1246, (p.p51 * 0.5));
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aii(1313, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1328] = (s.v[1313] > 50.0);
        s.store_scalar(1328, if s.b[1328] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1328]) {
            s.store_scalar(1271, 0.0);
        }

        s.b[1329] = (s.v[1313] < (-50.0));
        s.store_scalar(1329, if s.b[1329] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1328])) && s.b[1329]) {
            s.store_scalar(1271, 1.0);
        }

        if ((s.b[1202] && (!s.b[1328])) && (!s.b[1329])) {
            s.store_div_from_scalar_offset_ad(1271, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_mixed_aai(1272, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sub(s.ad_value(1212), s.ad_value(1312)), A::tanh_scaled_input(A::sub(s.ad_value(1212), s.ad_value(1312)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1212), 0.5, s.ad_value(1312), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1212), s.ad_value(1312)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1271), (-(p.p51 * 0.1))), (-1.0), 1261, 1.0);
        }

        s.b[1330] = (s.v[1272] > 50.0);
        s.store_scalar(1330, if s.b[1330] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1330]) {
            s.store_mul(1273, 1262, 1272);
        }

        s.b[1331] = (s.v[1272] < (-50.0));
        s.store_scalar(1331, if s.b[1331] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1330])) && s.b[1331]) {
            s.store_mul_exp_rhs(1273, 1262, 1272);
        }

        if ((s.b[1202] && (!s.b[1330])) && (!s.b[1331])) {
            s.store_mul_ln_one_plus_exp_rhs(1273, 1262, 1272);
        }

        if s.b[1202] {
            s.store_div(1265, 1234, 1267);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1202] {
            s.store_mul_div_scaled_offset_numerator_rhs(1266, 1233, A::mul(s.ad_value(1241), s.ad_value(1219)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1241), s.ad_value(1218)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1287, 1266, 1222, 1.0, 1265, 1.0);
            s.store_add_scaled_product_right_ad(1288, 1287, (-1.0), 1287, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1273), 2.0, s.ad_value(1223), s.ad_value(1287), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1289, A::mul_sub_from_scalar_rhs(s.ad_value(1288), 1.0, s.ad_value(1271)), 1.0, 1261, 1271, 1.0);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1290, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::div(s.ad_value(1213), s.ad_value(1289)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1213), s.ad_value(1289)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1213), s.ad_value(1289))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul(1291, 1213, 1290);
        }

        if s.b[1202] {
            s.store_div_from_scalar_pow_ad(1292, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1213), -1.0, s.ad_value(1289), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1235)), 1.0), A::div_from_scalar(1.0, s.ad_value(1235)));
        }

        if s.b[1202] {
            s.store_mul_neg_lhs(1293, 1213, 1292);
            s.store_div_scaled_inputs2_indices(1313, 1212, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1332] = (s.v[1313] > 50.0);
        s.store_scalar(1332, if s.b[1332] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1332]) {
            s.store_scalar(1260, 0.0);
        }

        s.b[1333] = (s.v[1313] < (-50.0));
        s.store_scalar(1333, if s.b[1333] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1332])) && s.b[1333]) {
            s.store_scalar(1260, 1.0);
        }

        if ((s.b[1202] && (!s.b[1332])) && (!s.b[1333])) {
            s.store_div_from_scalar_offset_ad(1260, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1263, 1312, 1.0, 1293, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1260), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1334] = (s.v[1263] > 50.0);
        s.store_scalar(1334, if s.b[1334] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1334]) {
            s.store_mul(1264, 1262, 1263);
        }

        s.b[1335] = (s.v[1263] < (-50.0));
        s.store_scalar(1335, if s.b[1335] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1334])) && s.b[1335]) {
            s.store_mul_exp_rhs(1264, 1262, 1263);
        }

        if ((s.b[1202] && (!s.b[1334])) && (!s.b[1335])) {
            s.store_mul_ln_one_plus_exp_rhs(1264, 1262, 1263);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs2_indices(1313, 1312, 1.0, 1314, (-1.0), 1246, 1.0);
        }

        s.b[1336] = (s.v[1313] > 50.0);
        s.store_scalar(1336, if s.b[1336] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1336]) {
            s.store_scalar(1294, 0.0);
        }

        s.b[1337] = (s.v[1313] < (-50.0));
        s.store_scalar(1337, if s.b[1337] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1336])) && s.b[1337]) {
            s.store_scalar(1294, 1.0);
        }

        if ((s.b[1202] && (!s.b[1336])) && (!s.b[1337])) {
            s.store_div_from_scalar_offset_ad(1294, 1.0, A::exp(s.ad_value(1313)), 1.0);
        }

        if s.b[1202] {
            s.store_div_scaled_inputs3_mixed_iiai(1295, 1212, 1.0, 1291, (-1.0), A::add_scaled_product(s.ad_value(1249), 1.0, s.ad_value(1246), s.ad_value(1294), (-(p.p51 * 0.1))), -1.0, 1261, 1.0);
        }

        s.b[1338] = (s.v[1295] > 50.0);
        s.store_scalar(1338, if s.b[1338] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1338]) {
            s.store_mul(1296, 1262, 1295);
        }

        s.b[1339] = (s.v[1295] < (-50.0));
        s.store_scalar(1339, if s.b[1339] { 1.0 } else { 0.0 });

        if ((s.b[1202] && (!s.b[1338])) && s.b[1339]) {
            s.store_mul_exp_rhs(1296, 1262, 1295);
        }

        if ((s.b[1202] && (!s.b[1338])) && (!s.b[1339])) {
            s.store_mul_ln_one_plus_exp_rhs(1296, 1262, 1295);
        }

        if s.b[1202] {
            s.store_offset_square(1297, 1264, 1e-38);
            s.store_offset_mul(1298, 1297, 1264, 1e-57);
            s.store_offset_square(1299, 1296, 1e-38);
            s.store_offset_mul(1300, 1299, 1296, 1e-57);
            s.store_offset_mul(1301, 1264, 1296, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1302, 1297, (2.0 / 3.0), 1299, (2.0 / 3.0), 1301, (2.0 / 3.0), A::offset(A::add(s.ad_value(1264), s.ad_value(1296)), 2e-19), 1.0);
            s.store_div_ad(1303, A::add_scaled_inputs_products(s.ad_value(1298), (2.0 * 2.0), s.ad_value(1300), (3.0 * 2.0), s.ad_value(1297), s.ad_value(1296), (4.0 * 2.0), s.ad_value(1299), s.ad_value(1264), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1297), 15.0, s.ad_value(1299), 15.0, s.ad_value(1301), (2.0 * 15.0)));
            s.store_sub(1304, 1302, 1303);
            s.copy_ad(1305, 1303);
            s.store_mul_product3_mixed_iaii(1205, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1304, 1.0);
            s.store_mul_product3_mixed_iaii(1206, 1245, A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1222)), 1244, 1305, 1.0);
        }

        s.b[1340] = (s.v[1214] == 1.0);
        s.store_scalar(1340, if s.b[1340] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1340]) {
            s.store_div_scaled_inputs3_indices(1306, 1215, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1341] = (s.v[1306] > 50.0);
        s.store_scalar(1341, if s.b[1341] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1341]) {
            s.copy_ad(1309, 1306);
        }

        s.b[1342] = (s.v[1306] < (-50.0));
        s.store_scalar(1342, if s.b[1342] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && s.b[1342]) {
            s.store_exp(1309, 1306);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1341])) && (!s.b[1342])) {
            s.store_ln_one_plus_exp(1309, 1306);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1207, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1225), s.ad_value(1261)), 1309, 1245);
            s.store_div_scaled_inputs3_indices(1307, 1216, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1343] = (s.v[1307] > 50.0);
        s.store_scalar(1343, if s.b[1343] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1340]) && s.b[1343]) {
            s.copy_ad(1309, 1307);
        }

        s.b[1344] = (s.v[1307] < (-50.0));
        s.store_scalar(1344, if s.b[1344] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && s.b[1344]) {
            s.store_exp(1309, 1307);
        }

        if (((s.b[1202] && s.b[1340]) && (!s.b[1343])) && (!s.b[1344])) {
            s.store_ln_one_plus_exp(1309, 1307);
        }

        if (s.b[1202] && s.b[1340]) {
            s.store_mul_ad_product_lhs_mixed_ai(1208, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1226), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1340])) {
            s.store_scalar(1207, 0.0);
            s.store_scalar(1208, 0.0);
        }

        s.b[1345] = (s.v[1217] == 1.0);
        s.store_scalar(1345, if s.b[1345] { 1.0 } else { 0.0 });

        if (s.b[1202] && s.b[1345]) {
            s.store_div_scaled_inputs3_indices(1308, 1212, 1.0, 1249, -1.0, 1246, (-(-(p.p51 * 0.5))), 1261, 1.0);
        }

        s.b[1346] = (s.v[1308] > 50.0);
        s.store_scalar(1346, if s.b[1346] { 1.0 } else { 0.0 });

        if ((s.b[1202] && s.b[1345]) && s.b[1346]) {
            s.copy_ad(1309, 1308);
        }

        s.b[1347] = (s.v[1308] < (-50.0));
        s.store_scalar(1347, if s.b[1347] { 1.0 } else { 0.0 });

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && s.b[1347]) {
            s.store_exp(1309, 1308);
        }

        if (((s.b[1202] && s.b[1345]) && (!s.b[1346])) && (!s.b[1347])) {
            s.store_ln_one_plus_exp(1309, 1308);
        }

        if (s.b[1202] && s.b[1345]) {
            s.store_mul_ad_product_lhs_mixed_ai(1209, A::mul3(A::mul3(s.ad_value(1221), s.ad_value(1243), s.ad_value(1244)), s.ad_value(1224), s.ad_value(1261)), 1309, 1245);
        }

        if (s.b[1202] && (!s.b[1345])) {
            s.store_scalar(1209, 0.0);
        }

        if s.b[1202] {
            s.copy_ad(179, 1205);
            s.copy_ad(180, 1206);
            s.copy_ad(181, 1207);
            s.copy_ad(182, 1208);
            s.copy_ad(183, 1209);
        }

        s.b[1348] = (p.p122 == 1.0);
        s.store_scalar(1348, if s.b[1348] { 1.0 } else { 0.0 });

        s.store_scalar(185, 0.0);

        s.store_scalar(186, 0.0);

        s.store_scalar(187, 0.0);

        s.store_scalar(188, 0.0);

        s.store_scalar(189, 0.0);

        s.b[1349] = (p.p145 > p.p354);
        s.store_scalar(1349, if s.b[1349] { 1.0 } else { 0.0 });

        if s.b[1349] {
            s.store_scalar(1352, 0.0);
            s.store_scalar(1353, 0.0);
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
            s.store_scalar(1356, 0.0);
            s.store_scalar(1357, 0.0);
            s.store_scalar(1358, 0.0);
            s.copy_ad(1359, 78);
            s.copy_ad(1360, 79);
            s.store_scalar(1361, p.p151);
            s.copy_ad(1362, 80);
            s.copy_ad(1363, 81);
            s.store_scalar(1364, p.p149);
            s.copy_ad(1365, 111);
            s.store_scalar(1366, s.v[109]);
            s.copy_ad(1367, 113);
            s.store_scalar(1368, p.p0);
            s.store_scalar(1369, p.p145);
            s.copy_ad(1370, 29);
            s.store_scalar(1371, p.p150);
            s.copy_ad(1372, 30);
            s.copy_ad(1373, 31);
            s.store_scalar(1374, p.p146);
            s.store_scalar(1375, p.p160);
            s.store_scalar(1376, p.p159);
            s.store_scalar(1377, 0.0);
            s.store_scalar(1378, p.p161);
            s.store_scalar(1379, p.p165);
            s.store_scalar(1380, p.p156);
            s.store_scalar(1381, p.p157);
            s.store_scalar(1382, p.p158);
            s.store_scalar(1383, p.p164);
            s.store_scalar(1384, p.p163);
            s.store_scalar(1385, p.p162);
            s.store_scalar(1386, p.p39);
            s.store_scalar(1387, p.p47);
            s.store_scalar(1388, p.p45);
            s.store_scalar(1389, p.p42);
            s.store_scalar(1390, p.p2);
            s.store_scalar(1391, p.p6);
            s.store_scalar(1392, 1.0);
            s.store_scalar(1393, 0.0);
            s.store_scalar(1394, 0.0);
            s.store_scalar(1395, 0.0);
            s.store_scalar(1396, 0.0);
            s.store_scalar(1397, 0.0);
            s.store_scalar(1398, 0.0);
            s.store_scalar(1399, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1349] {
            s.store_scalar(1400, 0.0);
            s.store_scalar(1401, 0.0);
            s.store_scalar(1402, 0.0);
            s.store_scalar(1403, 0.0);
            s.store_scalar(1404, 0.0);
            s.store_scalar(1406, 0.0);
            s.store_scalar(1407, 0.0);
            s.store_scalar(1408, 0.0);
            s.store_scalar(1409, 0.0);
            s.store_scalar(1410, 0.0);
            s.store_scalar(1411, 0.0);
            s.store_scalar(1412, 0.0);
            s.store_scalar(1413, 0.0);
            s.store_scalar(1414, 0.0);
            s.store_scalar(1415, 0.0);
            s.store_scalar(1416, 0.0);
            s.store_scalar(1417, 0.0);
            s.store_scalar(1418, 0.0);
            s.store_scalar(1419, 0.0);
            s.store_scalar(1420, 0.0);
            s.store_scalar(1421, 0.0);
            s.store_scalar(1422, 0.0);
            s.store_scalar(1423, 0.0);
            s.store_scalar(1424, 0.0);
            s.store_scalar(1425, 0.0);
            s.store_scalar(1426, 0.0);
            s.store_scalar(1427, 0.0);
            s.store_scalar(1428, 0.0);
            s.store_scalar(1429, 0.0);
            s.store_scalar(1430, 0.0);
            s.store_scalar(1431, 0.0);
            s.store_scalar(1434, 0.0);
            s.store_scalar(1435, 0.0);
            s.store_scalar(1436, 0.0);
            s.store_scalar(1437, 0.0);
            s.store_scalar(1438, 0.0);
            s.store_scalar(1439, 0.0);
            s.store_scalar(1440, 0.0);
            s.store_scalar(1441, 0.0);
            s.store_scalar(1442, 0.0);
            s.store_scalar(1443, 0.0);
            s.store_scalar(1444, 0.0);
            s.store_scalar(1445, 0.0);
            s.store_scalar(1446, 0.0);
            s.store_scalar(1447, 0.0);
            s.store_scalar(1448, 0.0);
            s.store_scalar(1449, 0.0);
            s.store_scalar(1450, 0.0);
            s.store_scalar(1451, 0.0);
            s.store_scalar(1452, 0.0);
            s.store_scalar(1453, 0.0);
            s.store_scalar(1454, 0.0);
            s.store_scalar(1455, 0.0);
            s.store_scalar(1456, 0.0);
            s.store_scalar(1457, 0.0);
            s.store_scalar(1458, 0.0);
            s.store_scalar(1459, 0.0);
            s.store_scalar(1460, 0.0);
            s.store_scalar(1461, 0.0);
        }

        if s.b[1349] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(1458, 1360, A::tanh_scaled_input(s.ad_value(1360), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(1458, 1360, p.p53);
                } else {
                    s.store_scalar(1458, 0.0);
                }
            }
        }

        if s.b[1349] {
            s.store_sub(1459, 1359, 1360);
            s.store_mul(1393, 1379, 1367);
            s.store_add_scaled_product_value_ad(1395, A::div_scaled_inputs(s.ad_value(1375), 1.0, s.ad_value(1367), 2.302585092994046), 1.0, 1378, 1458, 1.0);
            s.store_add_scaled_product_right_sub(1396, 1374, 1.0, 1385, 1365, 1366, 1.0);
            s.store_pow_ad(1414, A::div(s.ad_value(1365), s.ad_value(1366)), s.ad_value(1387));
        }

        s.b[1462] = (s.v[1386] != 0.0);
        s.store_scalar(1462, if s.b[1462] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1462]) {
            s.store_div_ad_rhs(1397, 1458, A::pow(A::offset(A::pow(A::div(s.ad_value(1458), s.ad_value(1386)), s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382))));
        }

        if (s.b[1349] && (!s.b[1462])) {
            s.store_scalar(1397, 0.0);
        }

        if s.b[1349] {
            s.store_mul_add_scaled_product_rhs(1394, 1458, s.ad_value(1376), 1.0, s.ad_value(1397), s.ad_value(1377), (-1.0));
            s.store_sub(1357, 1396, 1394);
            s.store_scaled_mul(1399, 1395, 1367, 2.0);
            s.store_mul(1400, 1370, 1399);
            s.store_sub_scaled_inputs(1457, 1357, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1456, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1463] = (s.v[1456] > 50.0);
        s.store_scalar(1463, if s.b[1463] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1463]) {
            s.store_scalar(1415, 0.0);
        }

        s.b[1464] = (s.v[1456] < (-50.0));
        s.store_scalar(1464, if s.b[1464] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1463])) && s.b[1464]) {
            s.store_scalar(1415, 1.0);
        }

        if ((s.b[1349] && (!s.b[1463])) && (!s.b[1464])) {
            s.store_div_from_scalar_offset_ad(1415, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aai(1416, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1415), (-(p.p51 * 0.1))), (-1.0), 1399, 1.0);
        }

        s.b[1465] = (s.v[1416] > 50.0);
        s.store_scalar(1465, if s.b[1465] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1465]) {
            s.store_mul(1417, 1400, 1416);
        }

        s.b[1466] = (s.v[1416] < (-50.0));
        s.store_scalar(1466, if s.b[1466] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1465])) && s.b[1466]) {
            s.store_mul_exp_rhs(1417, 1400, 1416);
        }

        if ((s.b[1349] && (!s.b[1465])) && (!s.b[1466])) {
            s.store_mul_ln_one_plus_exp_rhs(1417, 1400, 1416);
        }

        if s.b[1349] {
            s.store_div_ad_rhs(1403, 1381, A::mul_offset_rhs(s.ad_value(1414), A::div_scaled_product(s.ad_value(1383), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(1404, 1380, A::div_scaled_offset_numerator(A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(1389), s.ad_value(1458), 1.0, s.ad_value(1369), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(1384), s.ad_value(1417), 1.0, s.ad_value(1370), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(1421, 1404, 1369, 1.0, 1403, 1.0);
            s.store_add_scaled_product_right_ad(1422, 1421, (-1.0), 1421, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1417), 2.0, s.ad_value(1370), s.ad_value(1421), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1423, A::mul_sub_from_scalar_rhs(s.ad_value(1421), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
            s.store_add_scaled_product_value_ad(1358, A::mul_sub_from_scalar_rhs(s.ad_value(1422), 1.0, s.ad_value(1415)), 1.0, 1399, 1415, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1424, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::div(s.ad_value(1360), s.ad_value(1358)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1358)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1360), s.ad_value(1358))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1425, 1360, 1424);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1426, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1358), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1427, 1360, 1426);
            s.store_div_scaled_inputs2_indices(1456, 1359, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1467] = (s.v[1456] > 50.0);
        s.store_scalar(1467, if s.b[1467] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1467]) {
            s.store_scalar(1398, 0.0);
        }

        s.b[1468] = (s.v[1456] < (-50.0));
        s.store_scalar(1468, if s.b[1468] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1467])) && s.b[1468]) {
            s.store_scalar(1398, 1.0);
        }

        if ((s.b[1349] && (!s.b[1467])) && (!s.b[1468])) {
            s.store_div_from_scalar_offset_ad(1398, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1401, 1459, 1.0, 1427, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1398), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1469] = (s.v[1401] > 50.0);
        s.store_scalar(1469, if s.b[1469] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1469]) {
            s.store_mul(1402, 1400, 1401);
        }

        s.b[1470] = (s.v[1401] < (-50.0));
        s.store_scalar(1470, if s.b[1470] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1469])) && s.b[1470]) {
            s.store_mul_exp_rhs(1402, 1400, 1401);
        }

        if ((s.b[1349] && (!s.b[1469])) && (!s.b[1470])) {
            s.store_mul_ln_one_plus_exp_rhs(1402, 1400, 1401);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1456, 1459, 1.0, 1457, (-1.0), 1393, 1.0);
        }

        s.b[1471] = (s.v[1456] > 50.0);
        s.store_scalar(1471, if s.b[1471] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1471]) {
            s.store_scalar(1428, 0.0);
        }

        s.b[1472] = (s.v[1456] < (-50.0));
        s.store_scalar(1472, if s.b[1472] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1471])) && s.b[1472]) {
            s.store_scalar(1428, 1.0);
        }

        if ((s.b[1349] && (!s.b[1471])) && (!s.b[1472])) {
            s.store_div_from_scalar_offset_ad(1428, 1.0, A::exp(s.ad_value(1456)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1429, 1359, 1.0, 1425, (-1.0), A::add_scaled_product(s.ad_value(1357), 1.0, s.ad_value(1393), s.ad_value(1428), (-(p.p51 * 0.1))), -1.0, 1399, 1.0);
        }

        s.b[1473] = (s.v[1429] > 50.0);
        s.store_scalar(1473, if s.b[1473] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1473]) {
            s.store_mul(1430, 1400, 1429);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1474] = (s.v[1429] < (-50.0));
        s.store_scalar(1474, if s.b[1474] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1473])) && s.b[1474]) {
            s.store_mul_exp_rhs(1430, 1400, 1429);
        }

        if ((s.b[1349] && (!s.b[1473])) && (!s.b[1474])) {
            s.store_mul_ln_one_plus_exp_rhs(1430, 1400, 1429);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1431, 1402, 1.0, 1430, (-1.0), 1370, 1.0);
            s.store_div(1457, 1431, 1423);
            s.store_div_scaled_inputs_indices(1406, 1375, 1.0, 1367, 2.302585092994046);
            s.store_scaled_mul(1408, 1406, 1367, 2.0);
            s.store_mul(1409, 1370, 1408);
            s.store_sub_scaled_inputs(1461, 1396, 1.0, 1393, (p.p51 * 0.5));
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aii(1460, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1475] = (s.v[1460] > 50.0);
        s.store_scalar(1475, if s.b[1475] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1475]) {
            s.store_scalar(1418, 0.0);
        }

        s.b[1476] = (s.v[1460] < (-50.0));
        s.store_scalar(1476, if s.b[1476] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1475])) && s.b[1476]) {
            s.store_scalar(1418, 1.0);
        }

        if ((s.b[1349] && (!s.b[1475])) && (!s.b[1476])) {
            s.store_div_from_scalar_offset_ad(1418, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_mixed_aai(1419, {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sub(s.ad_value(1359), s.ad_value(1459)), A::tanh_scaled_input(A::sub(s.ad_value(1359), s.ad_value(1459)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(1359), 0.5, s.ad_value(1459), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1359), s.ad_value(1459)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 1.0, A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1418), (-(p.p51 * 0.1))), (-1.0), 1408, 1.0);
        }

        s.b[1477] = (s.v[1419] > 50.0);
        s.store_scalar(1477, if s.b[1477] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1477]) {
            s.store_mul(1420, 1409, 1419);
        }

        s.b[1478] = (s.v[1419] < (-50.0));
        s.store_scalar(1478, if s.b[1478] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1477])) && s.b[1478]) {
            s.store_mul_exp_rhs(1420, 1409, 1419);
        }

        if ((s.b[1349] && (!s.b[1477])) && (!s.b[1478])) {
            s.store_mul_ln_one_plus_exp_rhs(1420, 1409, 1419);
        }

        if s.b[1349] {
            s.store_div(1412, 1381, 1414);
            s.store_mul_div_scaled_offset_numerator_rhs(1413, 1380, A::mul(s.ad_value(1388), s.ad_value(1366)), 1.0, 1.0, A::offset(A::mul(s.ad_value(1388), s.ad_value(1365)), 1.0), 1.0);
            s.store_div_scaled_product_indices(1434, 1413, 1369, 1.0, 1412, 1.0);
            s.store_add_scaled_product_right_ad(1435, 1434, (-1.0), 1434, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1420), 2.0, s.ad_value(1370), s.ad_value(1434), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(1436, A::mul_sub_from_scalar_rhs(s.ad_value(1435), 1.0, s.ad_value(1418)), 1.0, 1408, 1418, 1.0);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1437, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::div(s.ad_value(1360), s.ad_value(1436)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(1360), s.ad_value(1436)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1360), s.ad_value(1436))), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul(1438, 1360, 1437);
        }

        if s.b[1349] {
            s.store_div_from_scalar_pow_ad(1439, 1.0, A::offset(A::pow({
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1360), -1.0, s.ad_value(1436), 1.0)), p.p53), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, s.ad_value(1382)), 1.0), A::div_from_scalar(1.0, s.ad_value(1382)));
        }

        if s.b[1349] {
            s.store_mul_neg_lhs(1440, 1360, 1439);
            s.store_div_scaled_inputs2_indices(1460, 1359, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1479] = (s.v[1460] > 50.0);
        s.store_scalar(1479, if s.b[1479] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1479]) {
            s.store_scalar(1407, 0.0);
        }

        s.b[1480] = (s.v[1460] < (-50.0));
        s.store_scalar(1480, if s.b[1480] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1479])) && s.b[1480]) {
            s.store_scalar(1407, 1.0);
        }

        if ((s.b[1349] && (!s.b[1479])) && (!s.b[1480])) {
            s.store_div_from_scalar_offset_ad(1407, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1410, 1459, 1.0, 1440, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1407), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1481] = (s.v[1410] > 50.0);
        s.store_scalar(1481, if s.b[1481] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1481]) {
            s.store_mul(1411, 1409, 1410);
        }

        s.b[1482] = (s.v[1410] < (-50.0));
        s.store_scalar(1482, if s.b[1482] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1481])) && s.b[1482]) {
            s.store_mul_exp_rhs(1411, 1409, 1410);
        }

        if ((s.b[1349] && (!s.b[1481])) && (!s.b[1482])) {
            s.store_mul_ln_one_plus_exp_rhs(1411, 1409, 1410);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs2_indices(1460, 1459, 1.0, 1461, (-1.0), 1393, 1.0);
        }

        s.b[1483] = (s.v[1460] > 50.0);
        s.store_scalar(1483, if s.b[1483] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1483]) {
            s.store_scalar(1441, 0.0);
        }

        s.b[1484] = (s.v[1460] < (-50.0));
        s.store_scalar(1484, if s.b[1484] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1483])) && s.b[1484]) {
            s.store_scalar(1441, 1.0);
        }

        if ((s.b[1349] && (!s.b[1483])) && (!s.b[1484])) {
            s.store_div_from_scalar_offset_ad(1441, 1.0, A::exp(s.ad_value(1460)), 1.0);
        }

        if s.b[1349] {
            s.store_div_scaled_inputs3_mixed_iiai(1442, 1359, 1.0, 1438, (-1.0), A::add_scaled_product(s.ad_value(1396), 1.0, s.ad_value(1393), s.ad_value(1441), (-(p.p51 * 0.1))), -1.0, 1408, 1.0);
        }

        s.b[1485] = (s.v[1442] > 50.0);
        s.store_scalar(1485, if s.b[1485] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1485]) {
            s.store_mul(1443, 1409, 1442);
        }

        s.b[1486] = (s.v[1442] < (-50.0));
        s.store_scalar(1486, if s.b[1486] { 1.0 } else { 0.0 });

        if ((s.b[1349] && (!s.b[1485])) && s.b[1486]) {
            s.store_mul_exp_rhs(1443, 1409, 1442);
        }

        if ((s.b[1349] && (!s.b[1485])) && (!s.b[1486])) {
            s.store_mul_ln_one_plus_exp_rhs(1443, 1409, 1442);
        }

        if s.b[1349] {
            s.store_offset_square(1444, 1411, 1e-38);
            s.store_offset_mul(1445, 1444, 1411, 1e-57);
            s.store_offset_square(1446, 1443, 1e-38);
            s.store_offset_mul(1447, 1446, 1443, 1e-57);
            s.store_offset_mul(1448, 1411, 1443, 1e-38);
            s.store_div_scaled_inputs3_mixed_iiia(1449, 1444, (2.0 / 3.0), 1446, (2.0 / 3.0), 1448, (2.0 / 3.0), A::offset(A::add(s.ad_value(1411), s.ad_value(1443)), 2e-19), 1.0);
            s.store_div_ad(1450, A::add_scaled_inputs_products(s.ad_value(1445), (2.0 * 2.0), s.ad_value(1447), (3.0 * 2.0), s.ad_value(1444), s.ad_value(1443), (4.0 * 2.0), s.ad_value(1446), s.ad_value(1411), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1444), 15.0, s.ad_value(1446), 15.0, s.ad_value(1448), (2.0 * 15.0)));
            s.store_sub(1451, 1449, 1450);
            s.copy_ad(1452, 1450);
            s.store_mul_product3_mixed_iaii(1352, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), 1391, 1451, 1.0);
            s.store_mul_product3_mixed_iaii(1353, 1392, A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1369)), 1391, 1452, 1.0);
        }

        s.b[1487] = (s.v[1361] == 1.0);
        s.store_scalar(1487, if s.b[1487] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1487]) {
            s.store_div_scaled_inputs3_indices(1453, 1362, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1488] = (s.v[1453] > 50.0);
        s.store_scalar(1488, if s.b[1488] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1487]) && s.b[1488]) {
            s.copy_ad(1456, 1453);
        }

        s.b[1489] = (s.v[1453] < (-50.0));
        s.store_scalar(1489, if s.b[1489] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
            s.store_exp(1456, 1453);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
            s.store_ln_one_plus_exp(1456, 1453);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs_mixed_ai(1354, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1372), s.ad_value(1408)), 1456, 1392);
            s.store_div_scaled_inputs3_indices(1454, 1363, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1490] = (s.v[1454] > 50.0);
        s.store_scalar(1490, if s.b[1490] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1487]) && s.b[1490]) {
            s.copy_ad(1456, 1454);
        }

        s.b[1491] = (s.v[1454] < (-50.0));
        s.store_scalar(1491, if s.b[1491] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
            s.store_exp(1456, 1454);
        }

        if (((s.b[1349] && s.b[1487]) && (!s.b[1490])) && (!s.b[1491])) {
            s.store_ln_one_plus_exp(1456, 1454);
        }

        if (s.b[1349] && s.b[1487]) {
            s.store_mul_ad_product_lhs_mixed_ai(1355, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1373), s.ad_value(1408)), 1456, 1392);
        }

        if (s.b[1349] && (!s.b[1487])) {
            s.store_scalar(1354, 0.0);
            s.store_scalar(1355, 0.0);
        }

        s.b[1492] = (s.v[1364] == 1.0);
        s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });

        if (s.b[1349] && s.b[1492]) {
            s.store_div_scaled_inputs3_indices(1455, 1359, 1.0, 1396, -1.0, 1393, (-(-(p.p51 * 0.5))), 1408, 1.0);
        }

        s.b[1493] = (s.v[1455] > 50.0);
        s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });

        if ((s.b[1349] && s.b[1492]) && s.b[1493]) {
            s.copy_ad(1456, 1455);
        }

        s.b[1494] = (s.v[1455] < (-50.0));
        s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && s.b[1494]) {
            s.store_exp(1456, 1455);
        }

        if (((s.b[1349] && s.b[1492]) && (!s.b[1493])) && (!s.b[1494])) {
            s.store_ln_one_plus_exp(1456, 1455);
        }

        if (s.b[1349] && s.b[1492]) {
            s.store_mul_ad_product_lhs_mixed_ai(1356, A::mul3(A::mul3(s.ad_value(1368), s.ad_value(1390), s.ad_value(1391)), s.ad_value(1371), s.ad_value(1408)), 1456, 1392);
        }

        if (s.b[1349] && (!s.b[1492])) {
            s.store_scalar(1356, 0.0);
        }

        if s.b[1349] {
            s.copy_ad(185, 1352);
            s.copy_ad(186, 1353);
            s.copy_ad(187, 1354);
            s.copy_ad(188, 1355);
            s.copy_ad(189, 1356);
        }

        s.b[1495] = (p.p144 == 1.0);
        s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });

        s.store_scalar(1788, 0.0);

        s.store_scalar(1789, 0.0);

        s.store_scalar(1790, 0.0);

        s.store_scalar(1791, 0.0);

        s.store_scalar(1795, 0.0);

        s.store_scalar(1796, 0.0);

        s.copy_ad(1797, 45);

        s.copy_ad(1798, 44);

        s.store_scalar(1799, 0.0);

        s.store_scalar(1800, 0.0);

        s.store_scalar(1801, 0.0);

        s.store_scalar(1802, 0.0);

        s.copy_ad(1803, 111);

        s.store_scalar(1804, s.v[109]);

        s.copy_ad(1805, 113);

        s.store_scalar(1806, p.p0);

        s.store_scalar(1807, p.p1);

        s.copy_ad(1808, 19);

        s.store_scalar(1812, p.p35);

        s.store_scalar(1813, p.p36);

        s.store_scalar(1814, p.p37);

        s.store_scalar(1815, p.p38);

        s.store_scalar(1816, p.p40);

        s.store_scalar(1817, p.p41);

        s.store_scalar(1818, p.p32);

        s.store_scalar(1819, p.p33);

        s.store_scalar(1820, p.p34);

        s.store_scalar(1821, p.p44);

        s.store_scalar(1822, p.p43);

        s.store_scalar(1823, p.p46);

        s.store_scalar(1824, p.p39);

        s.store_scalar(1825, p.p47);

        s.store_scalar(1826, p.p45);

        s.store_scalar(1827, p.p42);

        s.store_scalar(1828, p.p2);

        s.store_scalar(1829, p.p6);

        s.copy_ad(1830, 230);

        s.store_scalar(1831, 0.0);

        s.store_scalar(1832, 0.0);

        s.store_scalar(1833, 0.0);

        s.store_scalar(1834, 0.0);

        s.store_scalar(1835, 0.0);

        s.store_scalar(1836, 0.0);

        s.store_scalar(1837, 0.0);

        s.store_scalar(1838, 0.0);

        s.store_scalar(1839, 0.0);

        s.store_scalar(1840, 0.0);

        s.store_scalar(1841, 0.0);

        s.store_scalar(1842, 0.0);

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(1843, 0.0);

        s.store_scalar(1844, 0.0);

        s.store_scalar(1845, 0.0);

        s.store_scalar(1846, 0.0);

        s.store_scalar(1847, 0.0);

        s.store_scalar(1848, 0.0);

        s.store_scalar(1849, 0.0);

        s.store_scalar(1850, 0.0);

        s.store_scalar(1851, 0.0);

        s.store_scalar(1852, 0.0);

        s.store_scalar(1853, 0.0);

        s.store_scalar(1854, 0.0);

        s.store_scalar(1855, 0.0);

        s.store_scalar(1856, 0.0);

        s.store_scalar(1857, 0.0);

        s.store_scalar(1858, 0.0);

        s.store_scalar(1859, 0.0);

        s.store_scalar(1860, 0.0);

        s.store_scalar(1861, 0.0);

        s.store_scalar(1862, 0.0);

        s.store_scalar(1863, 0.0);

        s.store_scalar(1864, 0.0);

        s.store_scalar(1865, 0.0);

        s.store_scalar(1866, 0.0);

        s.store_scalar(1867, 0.0);

        s.store_scalar(1868, 0.0);

        s.store_scalar(1869, 0.0);

        s.store_scalar(1870, 0.0);

        s.store_scalar(1871, 0.0);

        s.store_scalar(1872, 0.0);

        s.store_scalar(1873, 0.0);

        s.store_scalar(1874, 0.0);

        s.store_scalar(1875, 0.0);

        s.store_scalar(1876, 0.0);

        s.store_scalar(1877, 0.0);

        s.store_scalar(1878, 0.0);

        s.store_scalar(1879, 0.0);

        s.store_scalar(1880, 0.0);

        s.store_scalar(1881, 0.0);

        s.store_scalar(1882, 0.0);

        s.store_scalar(1883, 0.0);

        s.store_scalar(1884, 0.0);

        s.store_scalar(1885, 0.0);

        s.store_scalar(1886, 0.0);

        s.store_scalar(1887, 0.0);

        s.store_scalar(1888, 0.0);

        s.store_scalar(1889, 0.0);

        s.store_scalar(1890, 0.0);

        s.store_scalar(1891, 0.0);

        s.store_scalar(1892, 0.0);

        s.store_scalar(1893, 0.0);

        s.store_scalar(1894, 0.0);

        s.store_scalar(1895, 0.0);

        s.store_scalar(1896, 0.0);

        s.store_scalar(1897, 0.0);

        s.store_scalar(1898, 0.0);

        s.store_scalar(1899, 0.0);

        if (p.p52 != 0.0) {
            s.store_mul_ad_rhs(1896, 1798, A::tanh_scaled_input(s.ad_value(1798), (0.001 / p.p53)));
        } else {
            if (p.p52 == 0.0) {
                s.store_sqrt_square_offset(1896, 1798, p.p53);
            } else {
                s.store_scalar(1896, 0.0);
            }
        }

        s.store_sub(1897, 1797, 1798);

        s.store_scale(1831, 1805, s.v[1817]);

        s.store_add_scaled_ad_lhs(1833, A::div_from_scalar(s.v[1813], A::scale(s.ad_value(1805), 2.302585092994046)), 1896, s.v[1816]);

        s.store_offset_scaled(1834, 1803, s.v[1823], (((((-s.v[1804])) * (s.v[1823]))) + (s.v[1812])));

        s.store_powf_scaled_input(1852, 1803, 1.0 / (s.v[1804]), s.v[1825]);

        s.b[1900] = (s.v[1824] != 0.0);
        s.store_scalar(1900, if s.b[1900] { 1.0 } else { 0.0 });

        if s.b[1900] {
            s.store_div_ad_rhs(1835, 1896, A::powf(A::offset(A::powf(A::scale(s.ad_value(1896), 1.0 / (s.v[1824])), s.v[1820]), 1.0), (1.0 / s.v[1820])));
        }

        if (!s.b[1900]) {
            s.store_scalar(1835, 0.0);
        }

        s.store_mul_sub_from_scalar_ad_lhs(1832, s.v[1814], A::scale(s.ad_value(1835), s.v[1815]), 1896);

        s.store_sub(1795, 1834, 1832);

        s.store_scaled_mul(1837, 1833, 1805, 2.0);

        s.store_mul(1838, 1808, 1837);

        s.store_sub_scaled_inputs(1895, 1795, 1.0, 1831, (p.p51 * 0.5));

        s.store_div_scaled_inputs2_mixed_aii(1894, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1901] = (s.v[1894] > 50.0);
        s.store_scalar(1901, if s.b[1901] { 1.0 } else { 0.0 });

        if s.b[1901] {
            s.store_scalar(1853, 0.0);
        }

        s.b[1902] = (s.v[1894] < (-50.0));
        s.store_scalar(1902, if s.b[1902] { 1.0 } else { 0.0 });

        if ((!s.b[1901]) && s.b[1902]) {
            s.store_scalar(1853, 1.0);
        }

        if ((!s.b[1901]) && (!s.b[1902])) {
            s.store_div_from_scalar_offset_ad(1853, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs2_mixed_aai(1854, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1853), (-(p.p51 * 0.1))), (-1.0), 1837, 1.0);

        s.b[1903] = (s.v[1854] > 50.0);
        s.store_scalar(1903, if s.b[1903] { 1.0 } else { 0.0 });

        if s.b[1903] {
            s.store_mul(1855, 1838, 1854);
        }

        s.b[1904] = (s.v[1854] < (-50.0));
        s.store_scalar(1904, if s.b[1904] { 1.0 } else { 0.0 });

        if ((!s.b[1903]) && s.b[1904]) {
            s.store_mul_exp_rhs(1855, 1838, 1854);
        }

        if ((!s.b[1903]) && (!s.b[1904])) {
            s.store_mul_ln_one_plus_exp_rhs(1855, 1838, 1854);
        }

        s.store_div_from_scalar_ad(1841, s.v[1819], A::mul_offset_rhs(s.ad_value(1852), A::div_scaled_inputs(s.ad_value(1855), s.v[1821], s.ad_value(1808), 1.0), 1.0));

        s.store_div_scaled_value_by_product(1842, A::scale_offset(s.ad_value(1896), (s.v[1827] * 1.0 / (s.v[1807])), 1.0), (s.v[1818] * (1.0 + (s.v[1826] * s.v[1804]))), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), A::offset(A::div_scaled_inputs(s.ad_value(1855), s.v[1822], s.ad_value(1808), 1.0), 1.0), 1.0);

        s.store_add_ad(1843, A::mul3_scaled_output(s.ad_value(1853), s.ad_value(1805), s.ad_value(1841), (2.0 * 1.0 / (s.v[1807]))), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1853), s.ad_value(1842)));

        s.store_div_scaled_inputs_indices(1859, 1842, s.v[1807], 1841, 1.0);

        s.store_add_scaled_product_right_ad(1860, 1859, (-1.0), 1859, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1855), 2.0, s.ad_value(1808), s.ad_value(1859), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1861, A::mul_sub_from_scalar_rhs(s.ad_value(1859), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_add_scaled_product_value_ad(1796, A::mul_sub_from_scalar_rhs(s.ad_value(1860), 1.0, s.ad_value(1853)), 1.0, 1837, 1853, 1.0);

        s.store_div_from_scalar_powf_ad(1862, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::div(s.ad_value(1798), s.ad_value(1796)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1796)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1798), s.ad_value(1796))), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1863, 1798, 1862);

        s.store_div_from_scalar_powf_ad(1864, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1796), 1.0)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1865, 1798, 1864);

        s.store_div_scaled_inputs2_indices(1894, 1797, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1905] = (s.v[1894] > 50.0);
        s.store_scalar(1905, if s.b[1905] { 1.0 } else { 0.0 });

        if s.b[1905] {
            s.store_scalar(1836, 0.0);
        }

        s.b[1906] = (s.v[1894] < (-50.0));
        s.store_scalar(1906, if s.b[1906] { 1.0 } else { 0.0 });

        if ((!s.b[1905]) && s.b[1906]) {
            s.store_scalar(1836, 1.0);
        }

        if ((!s.b[1905]) && (!s.b[1906])) {
            s.store_div_from_scalar_offset_ad(1836, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1839, 1897, 1.0, 1865, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1836), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1907] = (s.v[1839] > 50.0);
        s.store_scalar(1907, if s.b[1907] { 1.0 } else { 0.0 });

        if s.b[1907] {
            s.store_mul(1840, 1838, 1839);
        }

        s.b[1908] = (s.v[1839] < (-50.0));
        s.store_scalar(1908, if s.b[1908] { 1.0 } else { 0.0 });

        if ((!s.b[1907]) && s.b[1908]) {
            s.store_mul_exp_rhs(1840, 1838, 1839);
        }

        if ((!s.b[1907]) && (!s.b[1908])) {
            s.store_mul_ln_one_plus_exp_rhs(1840, 1838, 1839);
        }

        s.store_div_scaled_inputs2_indices(1894, 1897, 1.0, 1895, (-1.0), 1831, 1.0);

        s.b[1909] = (s.v[1894] > 50.0);
        s.store_scalar(1909, if s.b[1909] { 1.0 } else { 0.0 });

        if s.b[1909] {
            s.store_scalar(1866, 0.0);
        }

        s.b[1910] = (s.v[1894] < (-50.0));
        s.store_scalar(1910, if s.b[1910] { 1.0 } else { 0.0 });

        if ((!s.b[1909]) && s.b[1910]) {
            s.store_scalar(1866, 1.0);
        }

        if ((!s.b[1909]) && (!s.b[1910])) {
            s.store_div_from_scalar_offset_ad(1866, 1.0, A::exp(s.ad_value(1894)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1867, 1797, 1.0, 1863, (-1.0), A::add_scaled_product(s.ad_value(1795), 1.0, s.ad_value(1831), s.ad_value(1866), (-(p.p51 * 0.1))), -1.0, 1837, 1.0);

        s.b[1911] = (s.v[1867] > 50.0);
        s.store_scalar(1911, if s.b[1911] { 1.0 } else { 0.0 });

        if s.b[1911] {
            s.store_mul(1868, 1838, 1867);
        }

        s.b[1912] = (s.v[1867] < (-50.0));
        s.store_scalar(1912, if s.b[1912] { 1.0 } else { 0.0 });

        if ((!s.b[1911]) && s.b[1912]) {
            s.store_mul_exp_rhs(1868, 1838, 1867);
        }

        if ((!s.b[1911]) && (!s.b[1912])) {
            s.store_mul_ln_one_plus_exp_rhs(1868, 1838, 1867);
        }

        s.store_div_scaled_inputs2_indices(1869, 1840, 1.0, 1868, (-1.0), 1808, 1.0);

        s.store_div(1895, 1869, 1861);

        s.store_div_ad_rhs(1870, 1895, A::powf(A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::mul(s.ad_value(1895), A::tanh_scaled_input(s.ad_value(1895), (0.001 / p.p53)))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::sqrt_square_offset(s.ad_value(1895), p.p53)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820])));

        s.store_mul(1871, 1843, 1870);

        s.store_mul_ad_affine_product_lhs(1789, A::add(s.ad_value(1840), s.ad_value(1868)), s.ad_value(1871), (((s.v[1829] * s.v[1806]) * s.v[1828]) * 0.5), 0.0, 1830);

        s.store_div_from_scalar_scaled_input(1844, s.v[1813], 1805, 2.302585092994046);

        s.store_scaled_mul(1846, 1844, 1805, 2.0);

        s.store_mul(1847, 1808, 1846);

        s.store_sub_scaled_inputs(1899, 1834, 1.0, 1831, (p.p51 * 0.5));

        s.store_div_scaled_inputs2_mixed_aii(1898, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1913] = (s.v[1898] > 50.0);
        s.store_scalar(1913, if s.b[1913] { 1.0 } else { 0.0 });

        if s.b[1913] {
            s.store_scalar(1856, 0.0);
        }

        s.b[1914] = (s.v[1898] < (-50.0));
        s.store_scalar(1914, if s.b[1914] { 1.0 } else { 0.0 });

        if ((!s.b[1913]) && s.b[1914]) {
            s.store_scalar(1856, 1.0);
        }

        if ((!s.b[1913]) && (!s.b[1914])) {
            s.store_div_from_scalar_offset_ad(1856, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs2_mixed_aai(1857, {
            if (p.p52 != 0.0) {
                A::add_scaled_inputs_product(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sub(s.ad_value(1797), s.ad_value(1897)), A::tanh_scaled_input(A::sub(s.ad_value(1797), s.ad_value(1897)), (0.001 / p.p53)), 0.5)
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs3(s.ad_value(1797), 0.5, s.ad_value(1897), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(1797), s.ad_value(1897)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, 1.0, A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1856), (-(p.p51 * 0.1))), (-1.0), 1846, 1.0);

        s.b[1915] = (s.v[1857] > 50.0);
        s.store_scalar(1915, if s.b[1915] { 1.0 } else { 0.0 });

        if s.b[1915] {
            s.store_mul(1858, 1847, 1857);
        }

        s.b[1916] = (s.v[1857] < (-50.0));
        s.store_scalar(1916, if s.b[1916] { 1.0 } else { 0.0 });

        if ((!s.b[1915]) && s.b[1916]) {
            s.store_mul_exp_rhs(1858, 1847, 1857);
        }

        if ((!s.b[1915]) && (!s.b[1916])) {
            s.store_mul_ln_one_plus_exp_rhs(1858, 1847, 1857);
        }

        s.store_div_from_scalar(1850, s.v[1819], 1852);

        s.store_scaled_div_from_scalar_ad(1851, (1.0 + (s.v[1826] * s.v[1804])), A::scale_offset(s.ad_value(1803), s.v[1826], 1.0), s.v[1818]);

        s.store_div_scaled_inputs_indices(1872, 1851, s.v[1807], 1850, 1.0);

        s.store_add_scaled_product_right_ad(1873, 1872, (-1.0), 1872, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(1858), 2.0, s.ad_value(1808), s.ad_value(1872), 1.0), 1.0)), 1.0);

        s.store_add_scaled_product_value_ad(1874, A::mul_sub_from_scalar_rhs(s.ad_value(1873), 1.0, s.ad_value(1856)), 1.0, 1846, 1856, 1.0);

        s.store_div_from_scalar_powf_ad(1875, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::div(s.ad_value(1798), s.ad_value(1874)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div(s.ad_value(1798), s.ad_value(1874)), 0.5, A::sqrt_square_offset(A::neg(A::div(s.ad_value(1798), s.ad_value(1874))), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul(1876, 1798, 1875);

        s.store_div_from_scalar_powf_ad(1877, 1.0, A::offset(A::powf({
            if (p.p52 != 0.0) {
                A::add_scaled_product(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), (0.001 / p.p53)), (-0.5))
            } else {
                {
                    if (p.p52 == 0.0) {
                        A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0), 0.5, A::sqrt_square_offset(A::neg(A::div_scaled_inputs(s.ad_value(1798), -1.0, s.ad_value(1874), 1.0)), p.p53), 0.5)
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, s.v[1820]), 1.0), (1.0 / s.v[1820]));

        s.store_mul_neg_lhs(1878, 1798, 1877);

        s.store_div_scaled_inputs2_indices(1898, 1797, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1917] = (s.v[1898] > 50.0);
        s.store_scalar(1917, if s.b[1917] { 1.0 } else { 0.0 });

        if s.b[1917] {
            s.store_scalar(1845, 0.0);
        }

        s.b[1918] = (s.v[1898] < (-50.0));
        s.store_scalar(1918, if s.b[1918] { 1.0 } else { 0.0 });

        if ((!s.b[1917]) && s.b[1918]) {
            s.store_scalar(1845, 1.0);
        }

        if ((!s.b[1917]) && (!s.b[1918])) {
            s.store_div_from_scalar_offset_ad(1845, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1848, 1897, 1.0, 1878, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1845), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1919] = (s.v[1848] > 50.0);
        s.store_scalar(1919, if s.b[1919] { 1.0 } else { 0.0 });

        if s.b[1919] {
            s.store_mul(1849, 1847, 1848);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        s.b[1920] = (s.v[1848] < (-50.0));
        s.store_scalar(1920, if s.b[1920] { 1.0 } else { 0.0 });

        if ((!s.b[1919]) && s.b[1920]) {
            s.store_mul_exp_rhs(1849, 1847, 1848);
        }

        if ((!s.b[1919]) && (!s.b[1920])) {
            s.store_mul_ln_one_plus_exp_rhs(1849, 1847, 1848);
        }

        s.store_div_scaled_inputs2_indices(1898, 1897, 1.0, 1899, (-1.0), 1831, 1.0);

        s.b[1921] = (s.v[1898] > 50.0);
        s.store_scalar(1921, if s.b[1921] { 1.0 } else { 0.0 });

        if s.b[1921] {
            s.store_scalar(1879, 0.0);
        }

        s.b[1922] = (s.v[1898] < (-50.0));
        s.store_scalar(1922, if s.b[1922] { 1.0 } else { 0.0 });

        if ((!s.b[1921]) && s.b[1922]) {
            s.store_scalar(1879, 1.0);
        }

        if ((!s.b[1921]) && (!s.b[1922])) {
            s.store_div_from_scalar_offset_ad(1879, 1.0, A::exp(s.ad_value(1898)), 1.0);
        }

        s.store_div_scaled_inputs3_mixed_iiai(1880, 1797, 1.0, 1876, (-1.0), A::add_scaled_product(s.ad_value(1834), 1.0, s.ad_value(1831), s.ad_value(1879), (-(p.p51 * 0.1))), -1.0, 1846, 1.0);

        s.b[1923] = (s.v[1880] > 50.0);
        s.store_scalar(1923, if s.b[1923] { 1.0 } else { 0.0 });

        if s.b[1923] {
            s.store_mul(1881, 1847, 1880);
        }

        s.b[1924] = (s.v[1880] < (-50.0));
        s.store_scalar(1924, if s.b[1924] { 1.0 } else { 0.0 });

        if ((!s.b[1923]) && s.b[1924]) {
            s.store_mul_exp_rhs(1881, 1847, 1880);
        }

        if ((!s.b[1923]) && (!s.b[1924])) {
            s.store_mul_ln_one_plus_exp_rhs(1881, 1847, 1880);
        }

        s.store_offset_square(1882, 1849, 1e-38);

        s.store_offset_mul(1883, 1882, 1849, 1e-57);

        s.store_offset_square(1884, 1881, 1e-38);

        s.store_offset_mul(1885, 1884, 1881, 1e-57);

        s.store_offset_mul(1886, 1849, 1881, 1e-38);

        s.store_div_scaled_inputs3_mixed_iiia(1887, 1882, (2.0 / 3.0), 1884, (2.0 / 3.0), 1886, (2.0 / 3.0), A::offset(A::add(s.ad_value(1849), s.ad_value(1881)), 2e-19), 1.0);

        s.store_div_ad(1888, A::add_scaled_inputs_products(s.ad_value(1883), (2.0 * 2.0), s.ad_value(1885), (3.0 * 2.0), s.ad_value(1882), s.ad_value(1881), (4.0 * 2.0), s.ad_value(1884), s.ad_value(1849), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(1882), 15.0, s.ad_value(1884), 15.0, s.ad_value(1886), (2.0 * 15.0)));

        s.store_sub(1889, 1887, 1888);

        s.copy_ad(1890, 1888);

        s.store_scaled_mul(1790, 1889, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.store_scaled_mul(1791, 1890, 1830, (((s.v[1806] * s.v[1828]) * s.v[1807]) * s.v[1829]));

        s.b[1925] = (s.v[1799] == 1.0);
        s.store_scalar(1925, if s.b[1925] { 1.0 } else { 0.0 });

        if s.b[1925] {
            s.store_div_ad_lhs(1891, A::sub_from_scalar(s.v[1800], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1926] = (s.v[1891] > 50.0);
        s.store_scalar(1926, if s.b[1926] { 1.0 } else { 0.0 });

        if (s.b[1925] && s.b[1926]) {
            s.copy_ad(1894, 1891);
        }

        s.b[1927] = (s.v[1891] < (-50.0));
        s.store_scalar(1927, if s.b[1927] { 1.0 } else { 0.0 });

        if ((s.b[1925] && (!s.b[1926])) && s.b[1927]) {
            s.store_exp(1894, 1891);
        }

        if ((s.b[1925] && (!s.b[1926])) && (!s.b[1927])) {
            s.store_ln_one_plus_exp(1894, 1891);
        }

        if s.b[1925] {
            s.store_div_ad_lhs(1892, A::sub_from_scalar(s.v[1801], A::sub_scaled_inputs(s.ad_value(1834), 1.0, s.ad_value(1831), (p.p51 * 0.5))), 1846);
        }

        s.b[1928] = (s.v[1892] > 50.0);
        s.store_scalar(1928, if s.b[1928] { 1.0 } else { 0.0 });

        if (s.b[1925] && s.b[1928]) {
            s.copy_ad(1894, 1892);
        }

        s.b[1929] = (s.v[1892] < (-50.0));
        s.store_scalar(1929, if s.b[1929] { 1.0 } else { 0.0 });

        if ((s.b[1925] && (!s.b[1928])) && s.b[1929]) {
            s.store_exp(1894, 1892);
        }

        if ((s.b[1925] && (!s.b[1928])) && (!s.b[1929])) {
            s.store_ln_one_plus_exp(1894, 1892);
        }

        s.b[1930] = (s.v[1802] == 1.0);
        s.store_scalar(1930, if s.b[1930] { 1.0 } else { 0.0 });

        if s.b[1930] {
            s.store_div_scaled_inputs3_indices(1893, 1797, 1.0, 1834, -1.0, 1831, (-(-(p.p51 * 0.5))), 1846, 1.0);
        }

        s.b[1931] = (s.v[1893] > 50.0);
        s.store_scalar(1931, if s.b[1931] { 1.0 } else { 0.0 });

        if (s.b[1930] && s.b[1931]) {
            s.copy_ad(1894, 1893);
        }

        s.b[1932] = (s.v[1893] < (-50.0));
        s.store_scalar(1932, if s.b[1932] { 1.0 } else { 0.0 });

        if ((s.b[1930] && (!s.b[1931])) && s.b[1932]) {
            s.store_exp(1894, 1893);
        }

        if ((s.b[1930] && (!s.b[1931])) && (!s.b[1932])) {
            s.store_ln_one_plus_exp(1894, 1893);
        }

        s.copy_ad(1788, 1789);

        s.copy_ad(115, 1789);

        s.copy_ad(117, 1790);

        s.copy_ad(118, 1791);

        s.copy_ad(115, 1788);

        s.b[1933] = (p.p322 == 0.0);
        s.store_scalar(1933, if s.b[1933] { 1.0 } else { 0.0 });

        s.store_scalar(234, 0.0);

        s.store_scalar(242, 0.0);

        s.store_scalar(243, 0.0);

        s.store_scalar(244, 0.0);

        s.store_scalar(245, 0.0);

        s.store_scalar(246, 0.0);

        s.store_scalar(247, 0.0);

        s.store_scalar(248, 0.0);

        s.store_scalar(254, 0.0);

        s.store_scalar(255, 0.0);

        s.store_scalar(256, 0.0);

        s.store_scalar(257, 0.0);

        s.store_scalar(258, 0.0);

        s.b[2418] = (p.p291 == 1.0);
        s.store_scalar(2418, if s.b[2418] { 1.0 } else { 0.0 });

        if s.b[2418] {
            s.store_scaled_voltage(234, ctx, nodes, Some(8), Some(7), p.p6);
        }

        s.b[2540] = (s.v[234] <= (p.p308 * p.p306));
        s.store_scalar(2540, if s.b[2540] { 1.0 } else { 0.0 });

        if (s.b[2418] && s.b[2540]) {
            s.store_offset_scaled_ad(242, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(234), 1.0 / (p.p306)))), (-((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306)), ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scalar(243, (1.0 - (((1.0 - p.p308)) as f64).sqrt()));
        }

        s.b[2541] = (p.p309 >= 1.0);
        s.store_scalar(2541, if s.b[2541] { 1.0 } else { 0.0 });

        if ((s.b[2418] && (!s.b[2540])) && s.b[2541]) {
            s.store_scalar(249, (1.0 / ((2.0 * p.p306) * (((1.0 - p.p308)) as f64).sqrt())));
            s.store_offset(254, 234, (-(p.p308 * p.p306)));
            s.store_mul(244, 249, 254);
        }

        s.b[2542] = (p.p309 >= 2.0);
        s.store_scalar(2542, if s.b[2542] { 1.0 } else { 0.0 });

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) {
            s.store_scale(250, 249, 1.0 / (((4.0 * p.p306) * (1.0 - p.p308))));
            s.store_square(255, 254);
            s.store_mul(245, 250, 255);
        }

        s.b[2543] = (p.p309 >= 3.0);
        s.store_scalar(2543, if s.b[2543] { 1.0 } else { 0.0 });

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) {
            s.store_scale(251, 250, 1.0 / (((2.0 * p.p306) * (1.0 - p.p308))));
            s.store_mul(256, 255, 254);
            s.store_mul(246, 251, 256);
        }

        s.b[2544] = (p.p309 >= 4.0);
        s.store_scalar(2544, if s.b[2544] { 1.0 } else { 0.0 });

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) {
            s.store_scale(252, 251, (5.0 * 1.0 / (((8.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(257, 256, 254);
            s.store_mul(247, 252, 257);
        }

        s.b[2545] = (p.p309 >= 5.0);
        s.store_scalar(2545, if s.b[2545] { 1.0 } else { 0.0 });

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && s.b[2545]) {
            s.store_scale(253, 252, (7.0 * 1.0 / (((10.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(258, 257, 254);
            s.store_mul(248, 253, 258);
        }

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && (!s.b[2545])) {
            s.store_scalar(253, 0.0);
        }

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && (!s.b[2544])) {
            s.store_scalar(252, 0.0);
        }

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && (!s.b[2543])) {
            s.store_scalar(251, 0.0);
        }

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && (!s.b[2542])) {
            s.store_scalar(250, 0.0);
        }

        if ((s.b[2418] && (!s.b[2540])) && (!s.b[2541])) {
            s.store_scalar(249, 0.0);
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scaled_add_ad_lhs(242, A::add(A::add_scaled_inputs4(s.ad_value(243), 1.0, s.ad_value(244), 1.0, s.ad_value(245), 1.0, s.ad_value(246), 1.0), s.ad_value(247)), 248, ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        s.b[2674] = ((((nv6 - nv2) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        if s.b[2674] {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2675] = ((((nv6 - nv2) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        if ((!s.b[2674]) && s.b[2675]) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2674]) && (!s.b[2675])) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2676] = ((((nv6 - nv0) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        if s.b[2676] {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2677] = ((((nv6 - nv0) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        if ((!s.b[2676]) && s.b[2677]) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2676]) && (!s.b[2677])) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2678] = ((((nv2 - nv0) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });

        if s.b[2678] {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2679] = ((((nv2 - nv0) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        if ((!s.b[2678]) && s.b[2679]) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2678]) && (!s.b[2679])) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2680] = ((((nv3 - nv2) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if s.b[2680] {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2681] = ((((nv3 - nv2) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });

        if ((!s.b[2680]) && s.b[2681]) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2680]) && (!s.b[2681])) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2682] = ((((nv3 - nv0) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });

        if s.b[2682] {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2683] = ((((nv3 - nv0) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });

        if ((!s.b[2682]) && s.b[2683]) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2682]) && (!s.b[2683])) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2684] = ((((nv6 - nv3) - p.p27) / p.p28) > 50.0);
        s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });

        if s.b[2684] {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2685] = ((((nv6 - nv3) - p.p27) / p.p28) < (-50.0));
        s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });

        if ((!s.b[2684]) && s.b[2685]) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2684]) && (!s.b[2685])) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2700] = (p.p320 > 0.0);
        s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard12: f64,
        var_guard13: f64,
        var_guard24: f64,
        var_guard59: f64,
        var_guard60: f64,
        var_guard95: f64,
        var_idsfp3: f64,
        var_idsfp3_dn15: f64,
        var_idsfp3_dn16: f64,
        var_idsfp3_dn2: f64,
        var_idsfp3_dn3: f64,
        var_idsfp3_dn4: f64,
        var_idsfp3_dn7: f64,
        var_idsfp4: f64,
        var_idsfp4_dn16: f64,
        var_idsfp4_dn17: f64,
        var_idsfp4_dn2: f64,
        var_idsfp4_dn3: f64,
        var_idsfp4_dn4: f64,
        var_idsfp4_dn7: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_qbfp4: f64,
        var_qbfp4_dn16: f64,
        var_qbfp4_dn17: f64,
        var_qbfp4_dn2: f64,
        var_qbfp4_dn3: f64,
        var_qbfp4_dn4: f64,
        var_qbfp4_dn7: f64,
        var_qcfp3: f64,
        var_qcfp3_dn15: f64,
        var_qcfp3_dn16: f64,
        var_qcfp3_dn2: f64,
        var_qcfp3_dn3: f64,
        var_qcfp3_dn4: f64,
        var_qcfp3_dn7: f64,
        var_qcfp4: f64,
        var_qcfp4_dn16: f64,
        var_qcfp4_dn17: f64,
        var_qcfp4_dn2: f64,
        var_qcfp4_dn3: f64,
        var_qcfp4_dn4: f64,
        var_qcfp4_dn7: f64,
        var_qgdfp3: f64,
        var_qgdfp3_dn15: f64,
        var_qgdfp3_dn16: f64,
        var_qgdfp3_dn2: f64,
        var_qgdfp3_dn4: f64,
        var_qgdfp3_dn7: f64,
        var_qgdfp4: f64,
        var_qgdfp4_dn16: f64,
        var_qgdfp4_dn17: f64,
        var_qgdfp4_dn2: f64,
        var_qgdfp4_dn4: f64,
        var_qgdfp4_dn7: f64,
        var_qgsfp3: f64,
        var_qgsfp3_dn15: f64,
        var_qgsfp3_dn16: f64,
        var_qgsfp3_dn2: f64,
        var_qgsfp3_dn4: f64,
        var_qgsfp3_dn7: f64,
        var_qgsfp4: f64,
        var_qgsfp4_dn16: f64,
        var_qgsfp4_dn17: f64,
        var_qgsfp4_dn2: f64,
        var_qgsfp4_dn4: f64,
        var_qgsfp4_dn7: f64,
        var_qsfp3: f64,
        var_qsfp3_dn15: f64,
        var_qsfp3_dn16: f64,
        var_qsfp3_dn2: f64,
        var_qsfp3_dn3: f64,
        var_qsfp3_dn4: f64,
        var_qsfp3_dn7: f64,
        var_qsfp4: f64,
        var_qsfp4_dn16: f64,
        var_qsfp4_dn17: f64,
        var_qsfp4_dn2: f64,
        var_qsfp4_dn3: f64,
        var_qsfp4_dn4: f64,
        var_qsfp4_dn7: f64,
        var_tdut: f64,
        var_tdut_dn4: f64,
        var_tnomk: f64,
        var_vdloutput: f64,
        var_vdloutput_dn23: f64,
        var_vgloutput: f64,
        var_vgloutput_dn26: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv20 = ctx.node_voltage(nodes[20]);
        let nv21 = ctx.node_voltage(nodes[21]);
        let nv23 = ctx.node_voltage(nodes[23]);
        let nv24 = ctx.node_voltage(nodes[24]);
        let nv26 = ctx.node_voltage(nodes[26]);
        let nv27 = ctx.node_voltage(nodes[27]);
        let (eq8_e421, eq8_e421_d_n20, eq8_e421_d_n21,) = {
    if (var_guard12 != 0.0) {
        let eq8_e418: f64 = (p.p330 * (nv21 - nv20));
        let eq8_e419: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e418);
        (eq8_e419, ((-p.p330) * ddt_scale), (p.p330 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e421;
        stamper.stamp_current_node2_local(
            Some(21),
            Some(20),
            multiplicity * (eq8_value),
            20,
            multiplicity * (eq8_e421_d_n20),
            21,
            multiplicity * (eq8_e421_d_n21),
        );
        let (eq9_e428, eq9_e428_d_n20,) = {
    if (var_guard12 != 0.0) {
        let eq9_e425: f64 = (p.p332 * (nv20 - 0.0));
        let eq9_e426: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq9_e425);
        (eq9_e426, (p.p332 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq9_value: f64 = eq9_e428;
        stamper.stamp_current_node1_local(
            Some(20),
            None,
            multiplicity * (eq9_value),
            20,
            multiplicity * (eq9_e428_d_n20),
        );
        let eq14_ad_e518: A = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        A::scaled_offset({
            if ((!(((nv24 - nv23) / var_phit) > 50.0)) && (!(((nv24 - nv23) / var_phit) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(24), Some(23)), A::from_derivatives(var_phit, [0.0, 0.0, 0.0, 0.0, var_phit_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))
            } else {
                {
                    if ((!(((nv24 - nv23) / var_phit) > 50.0)) && (((nv24 - nv23) / var_phit) < (-50.0))) {
                        A::exp_scaled_input(A::constant(50.0), -1.0)
                    } else {
                        {
                            if (((nv24 - nv23) / var_phit) > 50.0) {
                                A::scaled_offset(A::div(A::voltage(ctx, nodes, Some(24), Some(23)), A::from_derivatives(var_phit, [0.0, 0.0, 0.0, 0.0, var_phit_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])), (((-50.0)) + (1.0)), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                }
            }
        }, (-1.0), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq14_ad: A = eq14_ad_e518;
        stamper.stamp_current_dense_local(
            Some(24),
            Some(23),
            multiplicity * eq14_ad.value,
            &eq14_ad.dn,
            &eq14_ad.db,
            multiplicity,
        );
        let (eq17_e564, eq17_e564_d_n4, eq17_e564_d_n23,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        let eq17_e543: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_vdloutput);
        let eq17_e544: f64 = (p.p341 * eq17_e543);
        let eq17_e544_d_n23: f64 = (p.p341 * (var_vdloutput_dn23 * ddt_scale));
        let eq17_e549: f64 = (var_tdut - var_tnomk);
        let eq17_e550: f64 = (p.p342 * eq17_e549);
        let eq17_e550_d_n4: f64 = (p.p342 * var_tdut_dn4);
        let eq17_e551: f64 = (1.0 + eq17_e550);
        let eq17_e555: f64 = (var_tdut - var_tnomk);
        let eq17_e556: f64 = (p.p344 * eq17_e555);
        let eq17_e556_d_n4: f64 = (p.p344 * var_tdut_dn4);
        let eq17_e559: f64 = (var_tdut - var_tnomk);
        let eq17_e560: f64 = (eq17_e556 * eq17_e559);
        let eq17_e560_d_n4: f64 = ((eq17_e556_d_n4 * eq17_e559) + (eq17_e556 * var_tdut_dn4));
        let eq17_e561: f64 = (eq17_e551 + eq17_e560);
        let eq17_e561_d_n4: f64 = (eq17_e550_d_n4 + eq17_e560_d_n4);
        let eq17_e562: f64 = (eq17_e544 * eq17_e561);
        let eq17_e562_d_n4: f64 = (eq17_e544 * eq17_e561_d_n4);
        let eq17_e562_d_n23: f64 = (eq17_e544_d_n23 * eq17_e561);
        (eq17_e562, eq17_e562_d_n4, eq17_e562_d_n23,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e564;
        stamper.stamp_current_node2_local(
            Some(23),
            None,
            multiplicity * (eq17_value),
            4,
            multiplicity * (eq17_e564_d_n4),
            23,
            multiplicity * (eq17_e564_d_n23),
        );
        let eq19_ad_e636: A = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        A::scaled_offset({
            if ((!(((nv26 - nv27) / var_phit) > 50.0)) && (!(((nv26 - nv27) / var_phit) < (-50.0)))) {
                A::exp(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), A::from_derivatives(var_phit, [0.0, 0.0, 0.0, 0.0, var_phit_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])))
            } else {
                {
                    if ((!(((nv26 - nv27) / var_phit) > 50.0)) && (((nv26 - nv27) / var_phit) < (-50.0))) {
                        A::exp_scaled_input(A::constant(50.0), -1.0)
                    } else {
                        {
                            if (((nv26 - nv27) / var_phit) > 50.0) {
                                A::scaled_offset(A::div(A::voltage(ctx, nodes, Some(26), Some(27)), A::from_derivatives(var_phit, [0.0, 0.0, 0.0, 0.0, var_phit_dn4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])), (((-50.0)) + (1.0)), ((50.0) as f64).exp())
                            } else {
                                A::constant(0.0)
                            }
                        }
                    }
                }
            }
        }, (-1.0), p.p346)
    } else {
        A::constant(0.0)
    }
};
        let eq19_ad: A = eq19_ad_e636;
        stamper.stamp_current_dense_local(
            Some(26),
            Some(27),
            multiplicity * eq19_ad.value,
            &eq19_ad.dn,
            &eq19_ad.db,
            multiplicity,
        );
        let (eq22_e682, eq22_e682_d_n4, eq22_e682_d_n26,) = {
    if ((var_guard12 == 0.0) && (var_guard13 != 0.0)) {
        let eq22_e661: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_vgloutput);
        let eq22_e662: f64 = (p.p341 * eq22_e661);
        let eq22_e662_d_n26: f64 = (p.p341 * (var_vgloutput_dn26 * ddt_scale));
        let eq22_e667: f64 = (var_tdut - var_tnomk);
        let eq22_e668: f64 = (p.p343 * eq22_e667);
        let eq22_e668_d_n4: f64 = (p.p343 * var_tdut_dn4);
        let eq22_e669: f64 = (1.0 + eq22_e668);
        let eq22_e673: f64 = (var_tdut - var_tnomk);
        let eq22_e674: f64 = (p.p345 * eq22_e673);
        let eq22_e674_d_n4: f64 = (p.p345 * var_tdut_dn4);
        let eq22_e677: f64 = (var_tdut - var_tnomk);
        let eq22_e678: f64 = (eq22_e674 * eq22_e677);
        let eq22_e678_d_n4: f64 = ((eq22_e674_d_n4 * eq22_e677) + (eq22_e674 * var_tdut_dn4));
        let eq22_e679: f64 = (eq22_e669 + eq22_e678);
        let eq22_e679_d_n4: f64 = (eq22_e668_d_n4 + eq22_e678_d_n4);
        let eq22_e680: f64 = (eq22_e662 * eq22_e679);
        let eq22_e680_d_n4: f64 = (eq22_e662 * eq22_e679_d_n4);
        let eq22_e680_d_n26: f64 = (eq22_e662_d_n26 * eq22_e679);
        (eq22_e680, eq22_e680_d_n4, eq22_e680_d_n26,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e682;
        stamper.stamp_current_node2_local(
            Some(26),
            None,
            multiplicity * (eq22_value),
            4,
            multiplicity * (eq22_e682_d_n4),
            26,
            multiplicity * (eq22_e682_d_n26),
        );
        let (eq31_e754, eq31_e754_d_n2, eq31_e754_d_n3, eq31_e754_d_n4, eq31_e754_d_n7, eq31_e754_d_n16, eq31_e754_d_n17,) = {
    if (var_guard24 != 0.0) {
        let eq31_e751: f64 = (var_gmin * (nv17 - nv16));
        let eq31_e752: f64 = (var_idsfp4 + eq31_e751);
        let eq31_e752_d_n16: f64 = (var_idsfp4_dn16 + (-var_gmin));
        let eq31_e752_d_n17: f64 = (var_idsfp4_dn17 + var_gmin);
        (eq31_e752, var_idsfp4_dn2, var_idsfp4_dn3, var_idsfp4_dn4, var_idsfp4_dn7, eq31_e752_d_n16, eq31_e752_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e754;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(17),
            Some(16),
            multiplicity * (eq31_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq31_e754_d_n2), multiplicity * (eq31_e754_d_n3), multiplicity * (eq31_e754_d_n4), multiplicity * (eq31_e754_d_n7), multiplicity * (eq31_e754_d_n16), multiplicity * (eq31_e754_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e769, eq33_e769_d_n2, eq33_e769_d_n4, eq33_e769_d_n7, eq33_e769_d_n16, eq33_e769_d_n17,) = {
    if (var_guard59 != 0.0) {
        let eq33_e762: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qgsfp4);
        let eq33_e765: f64 = (p.p355 * (nv7 - nv16));
        let eq33_e766: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq33_e765);
        let eq33_e767: f64 = (eq33_e762 + eq33_e766);
        let eq33_e767_d_n7: f64 = ((var_qgsfp4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq33_e767_d_n16: f64 = ((var_qgsfp4_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq33_e767, (var_qgsfp4_dn2 * ddt_scale), (var_qgsfp4_dn4 * ddt_scale), eq33_e767_d_n7, eq33_e767_d_n16, (var_qgsfp4_dn17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e769;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq33_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq33_e769_d_n2), multiplicity * (eq33_e769_d_n4), multiplicity * (eq33_e769_d_n7), multiplicity * (eq33_e769_d_n16), multiplicity * (eq33_e769_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e779, eq34_e779_d_n2, eq34_e779_d_n4, eq34_e779_d_n7, eq34_e779_d_n16, eq34_e779_d_n17,) = {
    if (var_guard59 != 0.0) {
        let eq34_e772: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qgdfp4);
        let eq34_e775: f64 = (p.p355 * (nv7 - nv17));
        let eq34_e776: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq34_e775);
        let eq34_e777: f64 = (eq34_e772 + eq34_e776);
        let eq34_e777_d_n7: f64 = ((var_qgdfp4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq34_e777_d_n17: f64 = ((var_qgdfp4_dn17 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq34_e777, (var_qgdfp4_dn2 * ddt_scale), (var_qgdfp4_dn4 * ddt_scale), eq34_e777_d_n7, (var_qgdfp4_dn16 * ddt_scale), eq34_e777_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e779;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(17),
            multiplicity * (eq34_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq34_e779_d_n2), multiplicity * (eq34_e779_d_n4), multiplicity * (eq34_e779_d_n7), multiplicity * (eq34_e779_d_n16), multiplicity * (eq34_e779_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e789, eq35_e789_d_n2, eq35_e789_d_n3, eq35_e789_d_n4, eq35_e789_d_n7, eq35_e789_d_n16, eq35_e789_d_n17,) = {
    if (var_guard59 != 0.0) {
        let eq35_e782: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qcfp4);
        let eq35_e785: f64 = (p.p355 * (nv2 - nv16));
        let eq35_e786: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq35_e785);
        let eq35_e787: f64 = (eq35_e782 + eq35_e786);
        let eq35_e787_d_n2: f64 = ((var_qcfp4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq35_e787_d_n16: f64 = ((var_qcfp4_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq35_e787, eq35_e787_d_n2, (var_qcfp4_dn3 * ddt_scale), (var_qcfp4_dn4 * ddt_scale), (var_qcfp4_dn7 * ddt_scale), eq35_e787_d_n16, (var_qcfp4_dn17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e789;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq35_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq35_e789_d_n2), multiplicity * (eq35_e789_d_n3), multiplicity * (eq35_e789_d_n4), multiplicity * (eq35_e789_d_n7), multiplicity * (eq35_e789_d_n16), multiplicity * (eq35_e789_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq37_e803, eq37_e803_d_n2, eq37_e803_d_n3, eq37_e803_d_n4, eq37_e803_d_n7, eq37_e803_d_n9, eq37_e803_d_n16, eq37_e803_d_n17,) = {
    if (var_guard59 != 0.0) {
        let eq37_e796: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qsfp4);
        let eq37_e799: f64 = (p.p355 * (nv7 - nv9));
        let eq37_e800: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq37_e799);
        let eq37_e801: f64 = (eq37_e796 + eq37_e800);
        let eq37_e801_d_n7: f64 = ((var_qsfp4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq37_e801, (var_qsfp4_dn2 * ddt_scale), (var_qsfp4_dn3 * ddt_scale), (var_qsfp4_dn4 * ddt_scale), eq37_e801_d_n7, ((-p.p355) * ddt_scale), (var_qsfp4_dn16 * ddt_scale), (var_qsfp4_dn17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e803;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq37_value),
            [2, 3, 4, 7, 9, 16, 17],
            [multiplicity * (eq37_e803_d_n2), multiplicity * (eq37_e803_d_n3), multiplicity * (eq37_e803_d_n4), multiplicity * (eq37_e803_d_n7), multiplicity * (eq37_e803_d_n9), multiplicity * (eq37_e803_d_n16), multiplicity * (eq37_e803_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq38_e814, eq38_e814_d_n2, eq38_e814_d_n4, eq38_e814_d_n7, eq38_e814_d_n16, eq38_e814_d_n17,) = {
    if (var_guard59 == 0.0) {
        let eq38_e807: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qgsfp4);
        let eq38_e810: f64 = (p.p355 * (nv2 - nv16));
        let eq38_e811: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq38_e810);
        let eq38_e812: f64 = (eq38_e807 + eq38_e811);
        let eq38_e812_d_n2: f64 = ((var_qgsfp4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq38_e812_d_n16: f64 = ((var_qgsfp4_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq38_e812, eq38_e812_d_n2, (var_qgsfp4_dn4 * ddt_scale), (var_qgsfp4_dn7 * ddt_scale), eq38_e812_d_n16, (var_qgsfp4_dn17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq38_value: f64 = eq38_e814;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq38_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq38_e814_d_n2), multiplicity * (eq38_e814_d_n4), multiplicity * (eq38_e814_d_n7), multiplicity * (eq38_e814_d_n16), multiplicity * (eq38_e814_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq39_e825, eq39_e825_d_n2, eq39_e825_d_n4, eq39_e825_d_n7, eq39_e825_d_n16, eq39_e825_d_n17,) = {
    if (var_guard59 == 0.0) {
        let eq39_e818: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qgdfp4);
        let eq39_e821: f64 = (p.p355 * (nv2 - nv17));
        let eq39_e822: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq39_e821);
        let eq39_e823: f64 = (eq39_e818 + eq39_e822);
        let eq39_e823_d_n2: f64 = ((var_qgdfp4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq39_e823_d_n17: f64 = ((var_qgdfp4_dn17 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq39_e823, eq39_e823_d_n2, (var_qgdfp4_dn4 * ddt_scale), (var_qgdfp4_dn7 * ddt_scale), (var_qgdfp4_dn16 * ddt_scale), eq39_e823_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e825;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(17),
            multiplicity * (eq39_value),
            [2, 4, 7, 16, 17],
            [multiplicity * (eq39_e825_d_n2), multiplicity * (eq39_e825_d_n4), multiplicity * (eq39_e825_d_n7), multiplicity * (eq39_e825_d_n16), multiplicity * (eq39_e825_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq40_e836, eq40_e836_d_n2, eq40_e836_d_n3, eq40_e836_d_n4, eq40_e836_d_n7, eq40_e836_d_n16, eq40_e836_d_n17,) = {
    if (var_guard59 == 0.0) {
        let eq40_e829: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qcfp4);
        let eq40_e832: f64 = (p.p355 * (nv7 - nv16));
        let eq40_e833: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, eq40_e832);
        let eq40_e834: f64 = (eq40_e829 + eq40_e833);
        let eq40_e834_d_n7: f64 = ((var_qcfp4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq40_e834_d_n16: f64 = ((var_qcfp4_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq40_e834, (var_qcfp4_dn2 * ddt_scale), (var_qcfp4_dn3 * ddt_scale), (var_qcfp4_dn4 * ddt_scale), eq40_e834_d_n7, eq40_e834_d_n16, (var_qcfp4_dn17 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e836;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq40_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * (eq40_e836_d_n2), multiplicity * (eq40_e836_d_n3), multiplicity * (eq40_e836_d_n4), multiplicity * (eq40_e836_d_n7), multiplicity * (eq40_e836_d_n16), multiplicity * (eq40_e836_d_n17)],
            [],
            [],
            1.0,
        );
        let eq43_e848: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, var_qbfp4);
        let eq43_e851: f64 = (p.p355 * (nv3 - nv16));
        let eq43_e852: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq43_e851);
        let eq43_e853: f64 = (eq43_e848 + eq43_e852);
        let eq43_e853_d_n3: f64 = ((var_qbfp4_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq43_e853_d_n16: f64 = ((var_qbfp4_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq43_value: f64 = eq43_e853;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(16),
            multiplicity * (eq43_value),
            [2, 3, 4, 7, 16, 17],
            [multiplicity * ((var_qbfp4_dn2 * ddt_scale)), multiplicity * (eq43_e853_d_n3), multiplicity * ((var_qbfp4_dn4 * ddt_scale)), multiplicity * ((var_qbfp4_dn7 * ddt_scale)), multiplicity * (eq43_e853_d_n16), multiplicity * ((var_qbfp4_dn17 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq44_e861, eq44_e861_d_n2, eq44_e861_d_n3, eq44_e861_d_n4, eq44_e861_d_n7, eq44_e861_d_n15, eq44_e861_d_n16,) = {
    if (var_guard60 != 0.0) {
        let eq44_e858: f64 = (var_gmin * (nv16 - nv15));
        let eq44_e859: f64 = (var_idsfp3 + eq44_e858);
        let eq44_e859_d_n15: f64 = (var_idsfp3_dn15 + (-var_gmin));
        let eq44_e859_d_n16: f64 = (var_idsfp3_dn16 + var_gmin);
        (eq44_e859, var_idsfp3_dn2, var_idsfp3_dn3, var_idsfp3_dn4, var_idsfp3_dn7, eq44_e859_d_n15, eq44_e859_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e861;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(16),
            Some(15),
            multiplicity * (eq44_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq44_e861_d_n2), multiplicity * (eq44_e861_d_n3), multiplicity * (eq44_e861_d_n4), multiplicity * (eq44_e861_d_n7), multiplicity * (eq44_e861_d_n15), multiplicity * (eq44_e861_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq46_e876, eq46_e876_d_n2, eq46_e876_d_n4, eq46_e876_d_n7, eq46_e876_d_n15, eq46_e876_d_n16,) = {
    if (var_guard95 != 0.0) {
        let eq46_e869: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qgsfp3);
        let eq46_e872: f64 = (p.p355 * (nv7 - nv15));
        let eq46_e873: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, eq46_e872);
        let eq46_e874: f64 = (eq46_e869 + eq46_e873);
        let eq46_e874_d_n7: f64 = ((var_qgsfp3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq46_e874_d_n15: f64 = ((var_qgsfp3_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq46_e874, (var_qgsfp3_dn2 * ddt_scale), (var_qgsfp3_dn4 * ddt_scale), eq46_e874_d_n7, eq46_e874_d_n15, (var_qgsfp3_dn16 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e876;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq46_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq46_e876_d_n2), multiplicity * (eq46_e876_d_n4), multiplicity * (eq46_e876_d_n7), multiplicity * (eq46_e876_d_n15), multiplicity * (eq46_e876_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq47_e886, eq47_e886_d_n2, eq47_e886_d_n4, eq47_e886_d_n7, eq47_e886_d_n15, eq47_e886_d_n16,) = {
    if (var_guard95 != 0.0) {
        let eq47_e879: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qgdfp3);
        let eq47_e882: f64 = (p.p355 * (nv7 - nv16));
        let eq47_e883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq47_e882);
        let eq47_e884: f64 = (eq47_e879 + eq47_e883);
        let eq47_e884_d_n7: f64 = ((var_qgdfp3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq47_e884_d_n16: f64 = ((var_qgdfp3_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq47_e884, (var_qgdfp3_dn2 * ddt_scale), (var_qgdfp3_dn4 * ddt_scale), eq47_e884_d_n7, (var_qgdfp3_dn15 * ddt_scale), eq47_e884_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e886;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(16),
            multiplicity * (eq47_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq47_e886_d_n2), multiplicity * (eq47_e886_d_n4), multiplicity * (eq47_e886_d_n7), multiplicity * (eq47_e886_d_n15), multiplicity * (eq47_e886_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq48_e896, eq48_e896_d_n2, eq48_e896_d_n3, eq48_e896_d_n4, eq48_e896_d_n7, eq48_e896_d_n15, eq48_e896_d_n16,) = {
    if (var_guard95 != 0.0) {
        let eq48_e889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, var_qcfp3);
        let eq48_e892: f64 = (p.p355 * (nv2 - nv15));
        let eq48_e893: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, eq48_e892);
        let eq48_e894: f64 = (eq48_e889 + eq48_e893);
        let eq48_e894_d_n2: f64 = ((var_qcfp3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq48_e894_d_n15: f64 = ((var_qcfp3_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq48_e894, eq48_e894_d_n2, (var_qcfp3_dn3 * ddt_scale), (var_qcfp3_dn4 * ddt_scale), (var_qcfp3_dn7 * ddt_scale), eq48_e894_d_n15, (var_qcfp3_dn16 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e896;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq48_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq48_e896_d_n2), multiplicity * (eq48_e896_d_n3), multiplicity * (eq48_e896_d_n4), multiplicity * (eq48_e896_d_n7), multiplicity * (eq48_e896_d_n15), multiplicity * (eq48_e896_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq50_e910, eq50_e910_d_n2, eq50_e910_d_n3, eq50_e910_d_n4, eq50_e910_d_n7, eq50_e910_d_n9, eq50_e910_d_n15, eq50_e910_d_n16,) = {
    if (var_guard95 != 0.0) {
        let eq50_e903: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, var_qsfp3);
        let eq50_e906: f64 = (p.p355 * (nv7 - nv9));
        let eq50_e907: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, eq50_e906);
        let eq50_e908: f64 = (eq50_e903 + eq50_e907);
        let eq50_e908_d_n7: f64 = ((var_qsfp3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq50_e908, (var_qsfp3_dn2 * ddt_scale), (var_qsfp3_dn3 * ddt_scale), (var_qsfp3_dn4 * ddt_scale), eq50_e908_d_n7, ((-p.p355) * ddt_scale), (var_qsfp3_dn15 * ddt_scale), (var_qsfp3_dn16 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e910;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq50_value),
            [2, 3, 4, 7, 9, 15, 16],
            [multiplicity * (eq50_e910_d_n2), multiplicity * (eq50_e910_d_n3), multiplicity * (eq50_e910_d_n4), multiplicity * (eq50_e910_d_n7), multiplicity * (eq50_e910_d_n9), multiplicity * (eq50_e910_d_n15), multiplicity * (eq50_e910_d_n16)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard131: f64,
        var_guard132: f64,
        var_guard167: f64,
        var_guard95: f64,
        var_guard96: f64,
        var_idsfp1: f64,
        var_idsfp1_dn14: f64,
        var_idsfp1_dn2: f64,
        var_idsfp1_dn3: f64,
        var_idsfp1_dn4: f64,
        var_idsfp1_dn5: f64,
        var_idsfp1_dn7: f64,
        var_idsfp2: f64,
        var_idsfp2_dn14: f64,
        var_idsfp2_dn15: f64,
        var_idsfp2_dn2: f64,
        var_idsfp2_dn3: f64,
        var_idsfp2_dn4: f64,
        var_idsfp2_dn7: f64,
        var_qbfp1: f64,
        var_qbfp1_dn14: f64,
        var_qbfp1_dn2: f64,
        var_qbfp1_dn3: f64,
        var_qbfp1_dn4: f64,
        var_qbfp1_dn5: f64,
        var_qbfp1_dn7: f64,
        var_qbfp2: f64,
        var_qbfp2_dn14: f64,
        var_qbfp2_dn15: f64,
        var_qbfp2_dn2: f64,
        var_qbfp2_dn3: f64,
        var_qbfp2_dn4: f64,
        var_qbfp2_dn7: f64,
        var_qbfp3: f64,
        var_qbfp3_dn15: f64,
        var_qbfp3_dn16: f64,
        var_qbfp3_dn2: f64,
        var_qbfp3_dn3: f64,
        var_qbfp3_dn4: f64,
        var_qbfp3_dn7: f64,
        var_qcfp1: f64,
        var_qcfp1_dn14: f64,
        var_qcfp1_dn2: f64,
        var_qcfp1_dn3: f64,
        var_qcfp1_dn4: f64,
        var_qcfp1_dn5: f64,
        var_qcfp1_dn7: f64,
        var_qcfp2: f64,
        var_qcfp2_dn14: f64,
        var_qcfp2_dn15: f64,
        var_qcfp2_dn2: f64,
        var_qcfp2_dn3: f64,
        var_qcfp2_dn4: f64,
        var_qcfp2_dn7: f64,
        var_qcfp3: f64,
        var_qcfp3_dn15: f64,
        var_qcfp3_dn16: f64,
        var_qcfp3_dn2: f64,
        var_qcfp3_dn3: f64,
        var_qcfp3_dn4: f64,
        var_qcfp3_dn7: f64,
        var_qgdfp1: f64,
        var_qgdfp1_dn14: f64,
        var_qgdfp1_dn2: f64,
        var_qgdfp1_dn4: f64,
        var_qgdfp1_dn5: f64,
        var_qgdfp1_dn7: f64,
        var_qgdfp2: f64,
        var_qgdfp2_dn14: f64,
        var_qgdfp2_dn15: f64,
        var_qgdfp2_dn2: f64,
        var_qgdfp2_dn4: f64,
        var_qgdfp2_dn7: f64,
        var_qgdfp3: f64,
        var_qgdfp3_dn15: f64,
        var_qgdfp3_dn16: f64,
        var_qgdfp3_dn2: f64,
        var_qgdfp3_dn4: f64,
        var_qgdfp3_dn7: f64,
        var_qgsfp1: f64,
        var_qgsfp1_dn14: f64,
        var_qgsfp1_dn2: f64,
        var_qgsfp1_dn4: f64,
        var_qgsfp1_dn5: f64,
        var_qgsfp1_dn7: f64,
        var_qgsfp2: f64,
        var_qgsfp2_dn14: f64,
        var_qgsfp2_dn15: f64,
        var_qgsfp2_dn2: f64,
        var_qgsfp2_dn4: f64,
        var_qgsfp2_dn7: f64,
        var_qgsfp3: f64,
        var_qgsfp3_dn15: f64,
        var_qgsfp3_dn16: f64,
        var_qgsfp3_dn2: f64,
        var_qgsfp3_dn4: f64,
        var_qgsfp3_dn7: f64,
        var_qsfp1: f64,
        var_qsfp1_dn14: f64,
        var_qsfp1_dn2: f64,
        var_qsfp1_dn3: f64,
        var_qsfp1_dn4: f64,
        var_qsfp1_dn5: f64,
        var_qsfp1_dn7: f64,
        var_qsfp2: f64,
        var_qsfp2_dn14: f64,
        var_qsfp2_dn15: f64,
        var_qsfp2_dn2: f64,
        var_qsfp2_dn3: f64,
        var_qsfp2_dn4: f64,
        var_qsfp2_dn7: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq51_e921, eq51_e921_d_n2, eq51_e921_d_n4, eq51_e921_d_n7, eq51_e921_d_n15, eq51_e921_d_n16,) = {
    if (var_guard95 == 0.0) {
        let eq51_e914: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 28, var_qgsfp3);
        let eq51_e917: f64 = (p.p355 * (nv2 - nv15));
        let eq51_e918: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 29, eq51_e917);
        let eq51_e919: f64 = (eq51_e914 + eq51_e918);
        let eq51_e919_d_n2: f64 = ((var_qgsfp3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq51_e919_d_n15: f64 = ((var_qgsfp3_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq51_e919, eq51_e919_d_n2, (var_qgsfp3_dn4 * ddt_scale), (var_qgsfp3_dn7 * ddt_scale), eq51_e919_d_n15, (var_qgsfp3_dn16 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e921;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq51_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq51_e921_d_n2), multiplicity * (eq51_e921_d_n4), multiplicity * (eq51_e921_d_n7), multiplicity * (eq51_e921_d_n15), multiplicity * (eq51_e921_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq52_e932, eq52_e932_d_n2, eq52_e932_d_n4, eq52_e932_d_n7, eq52_e932_d_n15, eq52_e932_d_n16,) = {
    if (var_guard95 == 0.0) {
        let eq52_e925: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 30, var_qgdfp3);
        let eq52_e928: f64 = (p.p355 * (nv2 - nv16));
        let eq52_e929: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 31, eq52_e928);
        let eq52_e930: f64 = (eq52_e925 + eq52_e929);
        let eq52_e930_d_n2: f64 = ((var_qgdfp3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq52_e930_d_n16: f64 = ((var_qgdfp3_dn16 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq52_e930, eq52_e930_d_n2, (var_qgdfp3_dn4 * ddt_scale), (var_qgdfp3_dn7 * ddt_scale), (var_qgdfp3_dn15 * ddt_scale), eq52_e930_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e932;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(16),
            multiplicity * (eq52_value),
            [2, 4, 7, 15, 16],
            [multiplicity * (eq52_e932_d_n2), multiplicity * (eq52_e932_d_n4), multiplicity * (eq52_e932_d_n7), multiplicity * (eq52_e932_d_n15), multiplicity * (eq52_e932_d_n16)],
            [],
            [],
            1.0,
        );
        let (eq53_e943, eq53_e943_d_n2, eq53_e943_d_n3, eq53_e943_d_n4, eq53_e943_d_n7, eq53_e943_d_n15, eq53_e943_d_n16,) = {
    if (var_guard95 == 0.0) {
        let eq53_e936: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 32, var_qcfp3);
        let eq53_e939: f64 = (p.p355 * (nv7 - nv15));
        let eq53_e940: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 33, eq53_e939);
        let eq53_e941: f64 = (eq53_e936 + eq53_e940);
        let eq53_e941_d_n7: f64 = ((var_qcfp3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq53_e941_d_n15: f64 = ((var_qcfp3_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq53_e941, (var_qcfp3_dn2 * ddt_scale), (var_qcfp3_dn3 * ddt_scale), (var_qcfp3_dn4 * ddt_scale), eq53_e941_d_n7, eq53_e941_d_n15, (var_qcfp3_dn16 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e943;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq53_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * (eq53_e943_d_n2), multiplicity * (eq53_e943_d_n3), multiplicity * (eq53_e943_d_n4), multiplicity * (eq53_e943_d_n7), multiplicity * (eq53_e943_d_n15), multiplicity * (eq53_e943_d_n16)],
            [],
            [],
            1.0,
        );
        let eq56_e955: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 34, var_qbfp3);
        let eq56_e958: f64 = (p.p355 * (nv3 - nv15));
        let eq56_e959: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 35, eq56_e958);
        let eq56_e960: f64 = (eq56_e955 + eq56_e959);
        let eq56_e960_d_n3: f64 = ((var_qbfp3_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq56_e960_d_n15: f64 = ((var_qbfp3_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq56_value: f64 = eq56_e960;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(15),
            multiplicity * (eq56_value),
            [2, 3, 4, 7, 15, 16],
            [multiplicity * ((var_qbfp3_dn2 * ddt_scale)), multiplicity * (eq56_e960_d_n3), multiplicity * ((var_qbfp3_dn4 * ddt_scale)), multiplicity * ((var_qbfp3_dn7 * ddt_scale)), multiplicity * (eq56_e960_d_n15), multiplicity * ((var_qbfp3_dn16 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq57_e968, eq57_e968_d_n2, eq57_e968_d_n3, eq57_e968_d_n4, eq57_e968_d_n7, eq57_e968_d_n14, eq57_e968_d_n15,) = {
    if (var_guard96 != 0.0) {
        let eq57_e965: f64 = (var_gmin * (nv15 - nv14));
        let eq57_e966: f64 = (var_idsfp2 + eq57_e965);
        let eq57_e966_d_n14: f64 = (var_idsfp2_dn14 + (-var_gmin));
        let eq57_e966_d_n15: f64 = (var_idsfp2_dn15 + var_gmin);
        (eq57_e966, var_idsfp2_dn2, var_idsfp2_dn3, var_idsfp2_dn4, var_idsfp2_dn7, eq57_e966_d_n14, eq57_e966_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e968;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(15),
            Some(14),
            multiplicity * (eq57_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq57_e968_d_n2), multiplicity * (eq57_e968_d_n3), multiplicity * (eq57_e968_d_n4), multiplicity * (eq57_e968_d_n7), multiplicity * (eq57_e968_d_n14), multiplicity * (eq57_e968_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq59_e983, eq59_e983_d_n2, eq59_e983_d_n4, eq59_e983_d_n7, eq59_e983_d_n14, eq59_e983_d_n15,) = {
    if (var_guard131 != 0.0) {
        let eq59_e976: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 36, var_qgsfp2);
        let eq59_e979: f64 = (p.p355 * (nv7 - nv14));
        let eq59_e980: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 37, eq59_e979);
        let eq59_e981: f64 = (eq59_e976 + eq59_e980);
        let eq59_e981_d_n7: f64 = ((var_qgsfp2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq59_e981_d_n14: f64 = ((var_qgsfp2_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq59_e981, (var_qgsfp2_dn2 * ddt_scale), (var_qgsfp2_dn4 * ddt_scale), eq59_e981_d_n7, eq59_e981_d_n14, (var_qgsfp2_dn15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e983;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq59_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq59_e983_d_n2), multiplicity * (eq59_e983_d_n4), multiplicity * (eq59_e983_d_n7), multiplicity * (eq59_e983_d_n14), multiplicity * (eq59_e983_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq60_e993, eq60_e993_d_n2, eq60_e993_d_n4, eq60_e993_d_n7, eq60_e993_d_n14, eq60_e993_d_n15,) = {
    if (var_guard131 != 0.0) {
        let eq60_e986: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 38, var_qgdfp2);
        let eq60_e989: f64 = (p.p355 * (nv7 - nv15));
        let eq60_e990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 39, eq60_e989);
        let eq60_e991: f64 = (eq60_e986 + eq60_e990);
        let eq60_e991_d_n7: f64 = ((var_qgdfp2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq60_e991_d_n15: f64 = ((var_qgdfp2_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq60_e991, (var_qgdfp2_dn2 * ddt_scale), (var_qgdfp2_dn4 * ddt_scale), eq60_e991_d_n7, (var_qgdfp2_dn14 * ddt_scale), eq60_e991_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e993;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(15),
            multiplicity * (eq60_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq60_e993_d_n2), multiplicity * (eq60_e993_d_n4), multiplicity * (eq60_e993_d_n7), multiplicity * (eq60_e993_d_n14), multiplicity * (eq60_e993_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq61_e1003, eq61_e1003_d_n2, eq61_e1003_d_n3, eq61_e1003_d_n4, eq61_e1003_d_n7, eq61_e1003_d_n14, eq61_e1003_d_n15,) = {
    if (var_guard131 != 0.0) {
        let eq61_e996: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 40, var_qcfp2);
        let eq61_e999: f64 = (p.p355 * (nv2 - nv14));
        let eq61_e1000: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 41, eq61_e999);
        let eq61_e1001: f64 = (eq61_e996 + eq61_e1000);
        let eq61_e1001_d_n2: f64 = ((var_qcfp2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq61_e1001_d_n14: f64 = ((var_qcfp2_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq61_e1001, eq61_e1001_d_n2, (var_qcfp2_dn3 * ddt_scale), (var_qcfp2_dn4 * ddt_scale), (var_qcfp2_dn7 * ddt_scale), eq61_e1001_d_n14, (var_qcfp2_dn15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1003;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq61_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq61_e1003_d_n2), multiplicity * (eq61_e1003_d_n3), multiplicity * (eq61_e1003_d_n4), multiplicity * (eq61_e1003_d_n7), multiplicity * (eq61_e1003_d_n14), multiplicity * (eq61_e1003_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq63_e1017, eq63_e1017_d_n2, eq63_e1017_d_n3, eq63_e1017_d_n4, eq63_e1017_d_n7, eq63_e1017_d_n9, eq63_e1017_d_n14, eq63_e1017_d_n15,) = {
    if (var_guard131 != 0.0) {
        let eq63_e1010: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 42, var_qsfp2);
        let eq63_e1013: f64 = (p.p355 * (nv7 - nv9));
        let eq63_e1014: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 43, eq63_e1013);
        let eq63_e1015: f64 = (eq63_e1010 + eq63_e1014);
        let eq63_e1015_d_n7: f64 = ((var_qsfp2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq63_e1015, (var_qsfp2_dn2 * ddt_scale), (var_qsfp2_dn3 * ddt_scale), (var_qsfp2_dn4 * ddt_scale), eq63_e1015_d_n7, ((-p.p355) * ddt_scale), (var_qsfp2_dn14 * ddt_scale), (var_qsfp2_dn15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1017;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq63_value),
            [2, 3, 4, 7, 9, 14, 15],
            [multiplicity * (eq63_e1017_d_n2), multiplicity * (eq63_e1017_d_n3), multiplicity * (eq63_e1017_d_n4), multiplicity * (eq63_e1017_d_n7), multiplicity * (eq63_e1017_d_n9), multiplicity * (eq63_e1017_d_n14), multiplicity * (eq63_e1017_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq64_e1028, eq64_e1028_d_n2, eq64_e1028_d_n4, eq64_e1028_d_n7, eq64_e1028_d_n14, eq64_e1028_d_n15,) = {
    if (var_guard131 == 0.0) {
        let eq64_e1021: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 44, var_qgsfp2);
        let eq64_e1024: f64 = (p.p355 * (nv2 - nv14));
        let eq64_e1025: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 45, eq64_e1024);
        let eq64_e1026: f64 = (eq64_e1021 + eq64_e1025);
        let eq64_e1026_d_n2: f64 = ((var_qgsfp2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq64_e1026_d_n14: f64 = ((var_qgsfp2_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq64_e1026, eq64_e1026_d_n2, (var_qgsfp2_dn4 * ddt_scale), (var_qgsfp2_dn7 * ddt_scale), eq64_e1026_d_n14, (var_qgsfp2_dn15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq64_value: f64 = eq64_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq64_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq64_e1028_d_n2), multiplicity * (eq64_e1028_d_n4), multiplicity * (eq64_e1028_d_n7), multiplicity * (eq64_e1028_d_n14), multiplicity * (eq64_e1028_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq65_e1039, eq65_e1039_d_n2, eq65_e1039_d_n4, eq65_e1039_d_n7, eq65_e1039_d_n14, eq65_e1039_d_n15,) = {
    if (var_guard131 == 0.0) {
        let eq65_e1032: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 46, var_qgdfp2);
        let eq65_e1035: f64 = (p.p355 * (nv2 - nv15));
        let eq65_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 47, eq65_e1035);
        let eq65_e1037: f64 = (eq65_e1032 + eq65_e1036);
        let eq65_e1037_d_n2: f64 = ((var_qgdfp2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq65_e1037_d_n15: f64 = ((var_qgdfp2_dn15 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq65_e1037, eq65_e1037_d_n2, (var_qgdfp2_dn4 * ddt_scale), (var_qgdfp2_dn7 * ddt_scale), (var_qgdfp2_dn14 * ddt_scale), eq65_e1037_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq65_value: f64 = eq65_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(15),
            multiplicity * (eq65_value),
            [2, 4, 7, 14, 15],
            [multiplicity * (eq65_e1039_d_n2), multiplicity * (eq65_e1039_d_n4), multiplicity * (eq65_e1039_d_n7), multiplicity * (eq65_e1039_d_n14), multiplicity * (eq65_e1039_d_n15)],
            [],
            [],
            1.0,
        );
        let (eq66_e1050, eq66_e1050_d_n2, eq66_e1050_d_n3, eq66_e1050_d_n4, eq66_e1050_d_n7, eq66_e1050_d_n14, eq66_e1050_d_n15,) = {
    if (var_guard131 == 0.0) {
        let eq66_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 48, var_qcfp2);
        let eq66_e1046: f64 = (p.p355 * (nv7 - nv14));
        let eq66_e1047: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 49, eq66_e1046);
        let eq66_e1048: f64 = (eq66_e1043 + eq66_e1047);
        let eq66_e1048_d_n7: f64 = ((var_qcfp2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq66_e1048_d_n14: f64 = ((var_qcfp2_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq66_e1048, (var_qcfp2_dn2 * ddt_scale), (var_qcfp2_dn3 * ddt_scale), (var_qcfp2_dn4 * ddt_scale), eq66_e1048_d_n7, eq66_e1048_d_n14, (var_qcfp2_dn15 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1050;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq66_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * (eq66_e1050_d_n2), multiplicity * (eq66_e1050_d_n3), multiplicity * (eq66_e1050_d_n4), multiplicity * (eq66_e1050_d_n7), multiplicity * (eq66_e1050_d_n14), multiplicity * (eq66_e1050_d_n15)],
            [],
            [],
            1.0,
        );
        let eq69_e1062: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 50, var_qbfp2);
        let eq69_e1065: f64 = (p.p355 * (nv3 - nv14));
        let eq69_e1066: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 51, eq69_e1065);
        let eq69_e1067: f64 = (eq69_e1062 + eq69_e1066);
        let eq69_e1067_d_n3: f64 = ((var_qbfp2_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq69_e1067_d_n14: f64 = ((var_qbfp2_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq69_value: f64 = eq69_e1067;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(14),
            multiplicity * (eq69_value),
            [2, 3, 4, 7, 14, 15],
            [multiplicity * ((var_qbfp2_dn2 * ddt_scale)), multiplicity * (eq69_e1067_d_n3), multiplicity * ((var_qbfp2_dn4 * ddt_scale)), multiplicity * ((var_qbfp2_dn7 * ddt_scale)), multiplicity * (eq69_e1067_d_n14), multiplicity * ((var_qbfp2_dn15 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq70_e1075, eq70_e1075_d_n2, eq70_e1075_d_n3, eq70_e1075_d_n4, eq70_e1075_d_n5, eq70_e1075_d_n7, eq70_e1075_d_n14,) = {
    if (var_guard132 != 0.0) {
        let eq70_e1072: f64 = (var_gmin * (nv14 - nv5));
        let eq70_e1073: f64 = (var_idsfp1 + eq70_e1072);
        let eq70_e1073_d_n5: f64 = (var_idsfp1_dn5 + (-var_gmin));
        let eq70_e1073_d_n14: f64 = (var_idsfp1_dn14 + var_gmin);
        (eq70_e1073, var_idsfp1_dn2, var_idsfp1_dn3, var_idsfp1_dn4, eq70_e1073_d_n5, var_idsfp1_dn7, eq70_e1073_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1075;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(14),
            Some(5),
            multiplicity * (eq70_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq70_e1075_d_n2), multiplicity * (eq70_e1075_d_n3), multiplicity * (eq70_e1075_d_n4), multiplicity * (eq70_e1075_d_n5), multiplicity * (eq70_e1075_d_n7), multiplicity * (eq70_e1075_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq72_e1090, eq72_e1090_d_n2, eq72_e1090_d_n4, eq72_e1090_d_n5, eq72_e1090_d_n7, eq72_e1090_d_n14,) = {
    if (var_guard167 != 0.0) {
        let eq72_e1083: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 52, var_qgsfp1);
        let eq72_e1086: f64 = (p.p355 * (nv7 - nv5));
        let eq72_e1087: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 53, eq72_e1086);
        let eq72_e1088: f64 = (eq72_e1083 + eq72_e1087);
        let eq72_e1088_d_n5: f64 = ((var_qgsfp1_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq72_e1088_d_n7: f64 = ((var_qgsfp1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq72_e1088, (var_qgsfp1_dn2 * ddt_scale), (var_qgsfp1_dn4 * ddt_scale), eq72_e1088_d_n5, eq72_e1088_d_n7, (var_qgsfp1_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1090;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq72_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq72_e1090_d_n2), multiplicity * (eq72_e1090_d_n4), multiplicity * (eq72_e1090_d_n5), multiplicity * (eq72_e1090_d_n7), multiplicity * (eq72_e1090_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq73_e1100, eq73_e1100_d_n2, eq73_e1100_d_n4, eq73_e1100_d_n5, eq73_e1100_d_n7, eq73_e1100_d_n14,) = {
    if (var_guard167 != 0.0) {
        let eq73_e1093: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 54, var_qgdfp1);
        let eq73_e1096: f64 = (p.p355 * (nv7 - nv14));
        let eq73_e1097: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 55, eq73_e1096);
        let eq73_e1098: f64 = (eq73_e1093 + eq73_e1097);
        let eq73_e1098_d_n7: f64 = ((var_qgdfp1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq73_e1098_d_n14: f64 = ((var_qgdfp1_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq73_e1098, (var_qgdfp1_dn2 * ddt_scale), (var_qgdfp1_dn4 * ddt_scale), (var_qgdfp1_dn5 * ddt_scale), eq73_e1098_d_n7, eq73_e1098_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1100;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(14),
            multiplicity * (eq73_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq73_e1100_d_n2), multiplicity * (eq73_e1100_d_n4), multiplicity * (eq73_e1100_d_n5), multiplicity * (eq73_e1100_d_n7), multiplicity * (eq73_e1100_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq74_e1110, eq74_e1110_d_n2, eq74_e1110_d_n3, eq74_e1110_d_n4, eq74_e1110_d_n5, eq74_e1110_d_n7, eq74_e1110_d_n14,) = {
    if (var_guard167 != 0.0) {
        let eq74_e1103: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 56, var_qcfp1);
        let eq74_e1106: f64 = (p.p355 * (nv2 - nv5));
        let eq74_e1107: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 57, eq74_e1106);
        let eq74_e1108: f64 = (eq74_e1103 + eq74_e1107);
        let eq74_e1108_d_n2: f64 = ((var_qcfp1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq74_e1108_d_n5: f64 = ((var_qcfp1_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq74_e1108, eq74_e1108_d_n2, (var_qcfp1_dn3 * ddt_scale), (var_qcfp1_dn4 * ddt_scale), eq74_e1108_d_n5, (var_qcfp1_dn7 * ddt_scale), (var_qcfp1_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1110;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(5),
            multiplicity * (eq74_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq74_e1110_d_n2), multiplicity * (eq74_e1110_d_n3), multiplicity * (eq74_e1110_d_n4), multiplicity * (eq74_e1110_d_n5), multiplicity * (eq74_e1110_d_n7), multiplicity * (eq74_e1110_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq76_e1124, eq76_e1124_d_n2, eq76_e1124_d_n3, eq76_e1124_d_n4, eq76_e1124_d_n5, eq76_e1124_d_n7, eq76_e1124_d_n9, eq76_e1124_d_n14,) = {
    if (var_guard167 != 0.0) {
        let eq76_e1117: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 58, var_qsfp1);
        let eq76_e1120: f64 = (p.p355 * (nv7 - nv9));
        let eq76_e1121: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 59, eq76_e1120);
        let eq76_e1122: f64 = (eq76_e1117 + eq76_e1121);
        let eq76_e1122_d_n7: f64 = ((var_qsfp1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq76_e1122, (var_qsfp1_dn2 * ddt_scale), (var_qsfp1_dn3 * ddt_scale), (var_qsfp1_dn4 * ddt_scale), (var_qsfp1_dn5 * ddt_scale), eq76_e1122_d_n7, ((-p.p355) * ddt_scale), (var_qsfp1_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1124;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq76_value),
            [2, 3, 4, 5, 7, 9, 14],
            [multiplicity * (eq76_e1124_d_n2), multiplicity * (eq76_e1124_d_n3), multiplicity * (eq76_e1124_d_n4), multiplicity * (eq76_e1124_d_n5), multiplicity * (eq76_e1124_d_n7), multiplicity * (eq76_e1124_d_n9), multiplicity * (eq76_e1124_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq77_e1135, eq77_e1135_d_n2, eq77_e1135_d_n4, eq77_e1135_d_n5, eq77_e1135_d_n7, eq77_e1135_d_n14,) = {
    if (var_guard167 == 0.0) {
        let eq77_e1128: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 60, var_qgsfp1);
        let eq77_e1131: f64 = (p.p355 * (nv2 - nv5));
        let eq77_e1132: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 61, eq77_e1131);
        let eq77_e1133: f64 = (eq77_e1128 + eq77_e1132);
        let eq77_e1133_d_n2: f64 = ((var_qgsfp1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq77_e1133_d_n5: f64 = ((var_qgsfp1_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq77_e1133, eq77_e1133_d_n2, (var_qgsfp1_dn4 * ddt_scale), eq77_e1133_d_n5, (var_qgsfp1_dn7 * ddt_scale), (var_qgsfp1_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1135;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(5),
            multiplicity * (eq77_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq77_e1135_d_n2), multiplicity * (eq77_e1135_d_n4), multiplicity * (eq77_e1135_d_n5), multiplicity * (eq77_e1135_d_n7), multiplicity * (eq77_e1135_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq78_e1146, eq78_e1146_d_n2, eq78_e1146_d_n4, eq78_e1146_d_n5, eq78_e1146_d_n7, eq78_e1146_d_n14,) = {
    if (var_guard167 == 0.0) {
        let eq78_e1139: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 62, var_qgdfp1);
        let eq78_e1142: f64 = (p.p355 * (nv2 - nv14));
        let eq78_e1143: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 63, eq78_e1142);
        let eq78_e1144: f64 = (eq78_e1139 + eq78_e1143);
        let eq78_e1144_d_n2: f64 = ((var_qgdfp1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq78_e1144_d_n14: f64 = ((var_qgdfp1_dn14 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq78_e1144, eq78_e1144_d_n2, (var_qgdfp1_dn4 * ddt_scale), (var_qgdfp1_dn5 * ddt_scale), (var_qgdfp1_dn7 * ddt_scale), eq78_e1144_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq78_value: f64 = eq78_e1146;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(14),
            multiplicity * (eq78_value),
            [2, 4, 5, 7, 14],
            [multiplicity * (eq78_e1146_d_n2), multiplicity * (eq78_e1146_d_n4), multiplicity * (eq78_e1146_d_n5), multiplicity * (eq78_e1146_d_n7), multiplicity * (eq78_e1146_d_n14)],
            [],
            [],
            1.0,
        );
        let (eq79_e1157, eq79_e1157_d_n2, eq79_e1157_d_n3, eq79_e1157_d_n4, eq79_e1157_d_n5, eq79_e1157_d_n7, eq79_e1157_d_n14,) = {
    if (var_guard167 == 0.0) {
        let eq79_e1150: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 64, var_qcfp1);
        let eq79_e1153: f64 = (p.p355 * (nv7 - nv5));
        let eq79_e1154: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 65, eq79_e1153);
        let eq79_e1155: f64 = (eq79_e1150 + eq79_e1154);
        let eq79_e1155_d_n5: f64 = ((var_qcfp1_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq79_e1155_d_n7: f64 = ((var_qcfp1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq79_e1155, (var_qcfp1_dn2 * ddt_scale), (var_qcfp1_dn3 * ddt_scale), (var_qcfp1_dn4 * ddt_scale), eq79_e1155_d_n5, eq79_e1155_d_n7, (var_qcfp1_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq79_value: f64 = eq79_e1157;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * (eq79_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * (eq79_e1157_d_n2), multiplicity * (eq79_e1157_d_n3), multiplicity * (eq79_e1157_d_n4), multiplicity * (eq79_e1157_d_n5), multiplicity * (eq79_e1157_d_n7), multiplicity * (eq79_e1157_d_n14)],
            [],
            [],
            1.0,
        );
        let eq82_e1169: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 66, var_qbfp1);
        let eq82_e1172: f64 = (p.p355 * (nv3 - nv5));
        let eq82_e1173: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 67, eq82_e1172);
        let eq82_e1174: f64 = (eq82_e1169 + eq82_e1173);
        let eq82_e1174_d_n3: f64 = ((var_qbfp1_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq82_e1174_d_n5: f64 = ((var_qbfp1_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq82_value: f64 = eq82_e1174;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(5),
            multiplicity * (eq82_value),
            [2, 3, 4, 5, 7, 14],
            [multiplicity * ((var_qbfp1_dn2 * ddt_scale)), multiplicity * (eq82_e1174_d_n3), multiplicity * ((var_qbfp1_dn4 * ddt_scale)), multiplicity * (eq82_e1174_d_n5), multiplicity * ((var_qbfp1_dn7 * ddt_scale)), multiplicity * ((var_qbfp1_dn14 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard168: f64,
        var_guard203: f64,
        var_guard204: f64,
        var_guard239: f64,
        var_guard240: f64,
        var_guard275: f64,
        var_idsfps1: f64,
        var_idsfps1_dn10: f64,
        var_idsfps1_dn2: f64,
        var_idsfps1_dn3: f64,
        var_idsfps1_dn4: f64,
        var_idsfps1_dn7: f64,
        var_idsfps1_dn9: f64,
        var_idsfps2: f64,
        var_idsfps2_dn10: f64,
        var_idsfps2_dn11: f64,
        var_idsfps2_dn2: f64,
        var_idsfps2_dn3: f64,
        var_idsfps2_dn4: f64,
        var_idsfps2_dn7: f64,
        var_idsfps3: f64,
        var_idsfps3_dn11: f64,
        var_idsfps3_dn12: f64,
        var_idsfps3_dn2: f64,
        var_idsfps3_dn3: f64,
        var_idsfps3_dn4: f64,
        var_idsfps3_dn7: f64,
        var_qbfps1: f64,
        var_qbfps1_dn10: f64,
        var_qbfps1_dn2: f64,
        var_qbfps1_dn3: f64,
        var_qbfps1_dn4: f64,
        var_qbfps1_dn7: f64,
        var_qbfps1_dn9: f64,
        var_qbfps2: f64,
        var_qbfps2_dn10: f64,
        var_qbfps2_dn11: f64,
        var_qbfps2_dn2: f64,
        var_qbfps2_dn3: f64,
        var_qbfps2_dn4: f64,
        var_qbfps2_dn7: f64,
        var_qcfps1: f64,
        var_qcfps1_dn10: f64,
        var_qcfps1_dn2: f64,
        var_qcfps1_dn3: f64,
        var_qcfps1_dn4: f64,
        var_qcfps1_dn7: f64,
        var_qcfps1_dn9: f64,
        var_qcfps2: f64,
        var_qcfps2_dn10: f64,
        var_qcfps2_dn11: f64,
        var_qcfps2_dn2: f64,
        var_qcfps2_dn3: f64,
        var_qcfps2_dn4: f64,
        var_qcfps2_dn7: f64,
        var_qcfps3: f64,
        var_qcfps3_dn11: f64,
        var_qcfps3_dn12: f64,
        var_qcfps3_dn2: f64,
        var_qcfps3_dn3: f64,
        var_qcfps3_dn4: f64,
        var_qcfps3_dn7: f64,
        var_qgdfps1: f64,
        var_qgdfps1_dn10: f64,
        var_qgdfps1_dn2: f64,
        var_qgdfps1_dn4: f64,
        var_qgdfps1_dn7: f64,
        var_qgdfps1_dn9: f64,
        var_qgdfps2: f64,
        var_qgdfps2_dn10: f64,
        var_qgdfps2_dn11: f64,
        var_qgdfps2_dn2: f64,
        var_qgdfps2_dn4: f64,
        var_qgdfps2_dn7: f64,
        var_qgdfps3: f64,
        var_qgdfps3_dn11: f64,
        var_qgdfps3_dn12: f64,
        var_qgdfps3_dn2: f64,
        var_qgdfps3_dn4: f64,
        var_qgdfps3_dn7: f64,
        var_qgsfps1: f64,
        var_qgsfps1_dn10: f64,
        var_qgsfps1_dn2: f64,
        var_qgsfps1_dn4: f64,
        var_qgsfps1_dn7: f64,
        var_qgsfps1_dn9: f64,
        var_qgsfps2: f64,
        var_qgsfps2_dn10: f64,
        var_qgsfps2_dn11: f64,
        var_qgsfps2_dn2: f64,
        var_qgsfps2_dn4: f64,
        var_qgsfps2_dn7: f64,
        var_qgsfps3: f64,
        var_qgsfps3_dn11: f64,
        var_qgsfps3_dn12: f64,
        var_qgsfps3_dn2: f64,
        var_qgsfps3_dn4: f64,
        var_qgsfps3_dn7: f64,
        var_qsfps1: f64,
        var_qsfps1_dn10: f64,
        var_qsfps1_dn2: f64,
        var_qsfps1_dn3: f64,
        var_qsfps1_dn4: f64,
        var_qsfps1_dn7: f64,
        var_qsfps1_dn9: f64,
        var_qsfps2: f64,
        var_qsfps2_dn10: f64,
        var_qsfps2_dn11: f64,
        var_qsfps2_dn2: f64,
        var_qsfps2_dn3: f64,
        var_qsfps2_dn4: f64,
        var_qsfps2_dn7: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq83_e1182, eq83_e1182_d_n2, eq83_e1182_d_n3, eq83_e1182_d_n4, eq83_e1182_d_n7, eq83_e1182_d_n9, eq83_e1182_d_n10,) = {
    if (var_guard168 != 0.0) {
        let eq83_e1179: f64 = (var_gmin * (nv9 - nv10));
        let eq83_e1180: f64 = (var_idsfps1 + eq83_e1179);
        let eq83_e1180_d_n9: f64 = (var_idsfps1_dn9 + var_gmin);
        let eq83_e1180_d_n10: f64 = (var_idsfps1_dn10 + (-var_gmin));
        (eq83_e1180, var_idsfps1_dn2, var_idsfps1_dn3, var_idsfps1_dn4, var_idsfps1_dn7, eq83_e1180_d_n9, eq83_e1180_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1182;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(9),
            Some(10),
            multiplicity * (eq83_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq83_e1182_d_n2), multiplicity * (eq83_e1182_d_n3), multiplicity * (eq83_e1182_d_n4), multiplicity * (eq83_e1182_d_n7), multiplicity * (eq83_e1182_d_n9), multiplicity * (eq83_e1182_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq85_e1197, eq85_e1197_d_n2, eq85_e1197_d_n4, eq85_e1197_d_n7, eq85_e1197_d_n9, eq85_e1197_d_n10,) = {
    if (var_guard203 != 0.0) {
        let eq85_e1190: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 68, var_qgsfps1);
        let eq85_e1193: f64 = (p.p355 * (nv7 - nv10));
        let eq85_e1194: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 69, eq85_e1193);
        let eq85_e1195: f64 = (eq85_e1190 + eq85_e1194);
        let eq85_e1195_d_n7: f64 = ((var_qgsfps1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq85_e1195_d_n10: f64 = ((var_qgsfps1_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq85_e1195, (var_qgsfps1_dn2 * ddt_scale), (var_qgsfps1_dn4 * ddt_scale), eq85_e1195_d_n7, (var_qgsfps1_dn9 * ddt_scale), eq85_e1195_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq85_value: f64 = eq85_e1197;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq85_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq85_e1197_d_n2), multiplicity * (eq85_e1197_d_n4), multiplicity * (eq85_e1197_d_n7), multiplicity * (eq85_e1197_d_n9), multiplicity * (eq85_e1197_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq86_e1207, eq86_e1207_d_n2, eq86_e1207_d_n4, eq86_e1207_d_n7, eq86_e1207_d_n9, eq86_e1207_d_n10,) = {
    if (var_guard203 != 0.0) {
        let eq86_e1200: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 70, var_qgdfps1);
        let eq86_e1203: f64 = (p.p355 * (nv7 - nv9));
        let eq86_e1204: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 71, eq86_e1203);
        let eq86_e1205: f64 = (eq86_e1200 + eq86_e1204);
        let eq86_e1205_d_n7: f64 = ((var_qgdfps1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq86_e1205_d_n9: f64 = ((var_qgdfps1_dn9 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq86_e1205, (var_qgdfps1_dn2 * ddt_scale), (var_qgdfps1_dn4 * ddt_scale), eq86_e1205_d_n7, eq86_e1205_d_n9, (var_qgdfps1_dn10 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq86_value: f64 = eq86_e1207;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq86_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq86_e1207_d_n2), multiplicity * (eq86_e1207_d_n4), multiplicity * (eq86_e1207_d_n7), multiplicity * (eq86_e1207_d_n9), multiplicity * (eq86_e1207_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq87_e1217, eq87_e1217_d_n2, eq87_e1217_d_n3, eq87_e1217_d_n4, eq87_e1217_d_n7, eq87_e1217_d_n9, eq87_e1217_d_n10,) = {
    if (var_guard203 != 0.0) {
        let eq87_e1210: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 72, var_qcfps1);
        let eq87_e1213: f64 = (p.p355 * (nv2 - nv10));
        let eq87_e1214: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 73, eq87_e1213);
        let eq87_e1215: f64 = (eq87_e1210 + eq87_e1214);
        let eq87_e1215_d_n2: f64 = ((var_qcfps1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq87_e1215_d_n10: f64 = ((var_qcfps1_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq87_e1215, eq87_e1215_d_n2, (var_qcfps1_dn3 * ddt_scale), (var_qcfps1_dn4 * ddt_scale), (var_qcfps1_dn7 * ddt_scale), (var_qcfps1_dn9 * ddt_scale), eq87_e1215_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq87_value: f64 = eq87_e1217;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq87_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq87_e1217_d_n2), multiplicity * (eq87_e1217_d_n3), multiplicity * (eq87_e1217_d_n4), multiplicity * (eq87_e1217_d_n7), multiplicity * (eq87_e1217_d_n9), multiplicity * (eq87_e1217_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq89_e1231, eq89_e1231_d_n2, eq89_e1231_d_n3, eq89_e1231_d_n4, eq89_e1231_d_n7, eq89_e1231_d_n9, eq89_e1231_d_n10,) = {
    if (var_guard203 != 0.0) {
        let eq89_e1224: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 74, var_qsfps1);
        let eq89_e1227: f64 = (p.p355 * (nv7 - nv9));
        let eq89_e1228: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 75, eq89_e1227);
        let eq89_e1229: f64 = (eq89_e1224 + eq89_e1228);
        let eq89_e1229_d_n7: f64 = ((var_qsfps1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq89_e1229_d_n9: f64 = ((var_qsfps1_dn9 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq89_e1229, (var_qsfps1_dn2 * ddt_scale), (var_qsfps1_dn3 * ddt_scale), (var_qsfps1_dn4 * ddt_scale), eq89_e1229_d_n7, eq89_e1229_d_n9, (var_qsfps1_dn10 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq89_value: f64 = eq89_e1231;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq89_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq89_e1231_d_n2), multiplicity * (eq89_e1231_d_n3), multiplicity * (eq89_e1231_d_n4), multiplicity * (eq89_e1231_d_n7), multiplicity * (eq89_e1231_d_n9), multiplicity * (eq89_e1231_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq90_e1242, eq90_e1242_d_n2, eq90_e1242_d_n4, eq90_e1242_d_n7, eq90_e1242_d_n9, eq90_e1242_d_n10,) = {
    if (var_guard203 == 0.0) {
        let eq90_e1235: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 76, var_qgsfps1);
        let eq90_e1238: f64 = (p.p355 * (nv2 - nv10));
        let eq90_e1239: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 77, eq90_e1238);
        let eq90_e1240: f64 = (eq90_e1235 + eq90_e1239);
        let eq90_e1240_d_n2: f64 = ((var_qgsfps1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq90_e1240_d_n10: f64 = ((var_qgsfps1_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq90_e1240, eq90_e1240_d_n2, (var_qgsfps1_dn4 * ddt_scale), (var_qgsfps1_dn7 * ddt_scale), (var_qgsfps1_dn9 * ddt_scale), eq90_e1240_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq90_value: f64 = eq90_e1242;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq90_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq90_e1242_d_n2), multiplicity * (eq90_e1242_d_n4), multiplicity * (eq90_e1242_d_n7), multiplicity * (eq90_e1242_d_n9), multiplicity * (eq90_e1242_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq91_e1253, eq91_e1253_d_n2, eq91_e1253_d_n4, eq91_e1253_d_n7, eq91_e1253_d_n9, eq91_e1253_d_n10,) = {
    if (var_guard203 == 0.0) {
        let eq91_e1246: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 78, var_qgdfps1);
        let eq91_e1249: f64 = (p.p355 * (nv2 - nv9));
        let eq91_e1250: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 79, eq91_e1249);
        let eq91_e1251: f64 = (eq91_e1246 + eq91_e1250);
        let eq91_e1251_d_n2: f64 = ((var_qgdfps1_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq91_e1251_d_n9: f64 = ((var_qgdfps1_dn9 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq91_e1251, eq91_e1251_d_n2, (var_qgdfps1_dn4 * ddt_scale), (var_qgdfps1_dn7 * ddt_scale), eq91_e1251_d_n9, (var_qgdfps1_dn10 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq91_value: f64 = eq91_e1253;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(9),
            multiplicity * (eq91_value),
            [2, 4, 7, 9, 10],
            [multiplicity * (eq91_e1253_d_n2), multiplicity * (eq91_e1253_d_n4), multiplicity * (eq91_e1253_d_n7), multiplicity * (eq91_e1253_d_n9), multiplicity * (eq91_e1253_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq92_e1264, eq92_e1264_d_n2, eq92_e1264_d_n3, eq92_e1264_d_n4, eq92_e1264_d_n7, eq92_e1264_d_n9, eq92_e1264_d_n10,) = {
    if (var_guard203 == 0.0) {
        let eq92_e1257: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 80, var_qcfps1);
        let eq92_e1260: f64 = (p.p355 * (nv7 - nv10));
        let eq92_e1261: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 81, eq92_e1260);
        let eq92_e1262: f64 = (eq92_e1257 + eq92_e1261);
        let eq92_e1262_d_n7: f64 = ((var_qcfps1_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq92_e1262_d_n10: f64 = ((var_qcfps1_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq92_e1262, (var_qcfps1_dn2 * ddt_scale), (var_qcfps1_dn3 * ddt_scale), (var_qcfps1_dn4 * ddt_scale), eq92_e1262_d_n7, (var_qcfps1_dn9 * ddt_scale), eq92_e1262_d_n10,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq92_value: f64 = eq92_e1264;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq92_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * (eq92_e1264_d_n2), multiplicity * (eq92_e1264_d_n3), multiplicity * (eq92_e1264_d_n4), multiplicity * (eq92_e1264_d_n7), multiplicity * (eq92_e1264_d_n9), multiplicity * (eq92_e1264_d_n10)],
            [],
            [],
            1.0,
        );
        let eq95_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 82, var_qbfps1);
        let eq95_e1279: f64 = (p.p355 * (nv3 - nv10));
        let eq95_e1280: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 83, eq95_e1279);
        let eq95_e1281: f64 = (eq95_e1276 + eq95_e1280);
        let eq95_e1281_d_n3: f64 = ((var_qbfps1_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq95_e1281_d_n10: f64 = ((var_qbfps1_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq95_value: f64 = eq95_e1281;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(10),
            multiplicity * (eq95_value),
            [2, 3, 4, 7, 9, 10],
            [multiplicity * ((var_qbfps1_dn2 * ddt_scale)), multiplicity * (eq95_e1281_d_n3), multiplicity * ((var_qbfps1_dn4 * ddt_scale)), multiplicity * ((var_qbfps1_dn7 * ddt_scale)), multiplicity * ((var_qbfps1_dn9 * ddt_scale)), multiplicity * (eq95_e1281_d_n10)],
            [],
            [],
            1.0,
        );
        let (eq96_e1289, eq96_e1289_d_n2, eq96_e1289_d_n3, eq96_e1289_d_n4, eq96_e1289_d_n7, eq96_e1289_d_n10, eq96_e1289_d_n11,) = {
    if (var_guard204 != 0.0) {
        let eq96_e1286: f64 = (var_gmin * (nv10 - nv11));
        let eq96_e1287: f64 = (var_idsfps2 + eq96_e1286);
        let eq96_e1287_d_n10: f64 = (var_idsfps2_dn10 + var_gmin);
        let eq96_e1287_d_n11: f64 = (var_idsfps2_dn11 + (-var_gmin));
        (eq96_e1287, var_idsfps2_dn2, var_idsfps2_dn3, var_idsfps2_dn4, var_idsfps2_dn7, eq96_e1287_d_n10, eq96_e1287_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e1289;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(11),
            multiplicity * (eq96_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq96_e1289_d_n2), multiplicity * (eq96_e1289_d_n3), multiplicity * (eq96_e1289_d_n4), multiplicity * (eq96_e1289_d_n7), multiplicity * (eq96_e1289_d_n10), multiplicity * (eq96_e1289_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq98_e1304, eq98_e1304_d_n2, eq98_e1304_d_n4, eq98_e1304_d_n7, eq98_e1304_d_n10, eq98_e1304_d_n11,) = {
    if (var_guard239 != 0.0) {
        let eq98_e1297: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 84, var_qgsfps2);
        let eq98_e1300: f64 = (p.p355 * (nv7 - nv11));
        let eq98_e1301: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 85, eq98_e1300);
        let eq98_e1302: f64 = (eq98_e1297 + eq98_e1301);
        let eq98_e1302_d_n7: f64 = ((var_qgsfps2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq98_e1302_d_n11: f64 = ((var_qgsfps2_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq98_e1302, (var_qgsfps2_dn2 * ddt_scale), (var_qgsfps2_dn4 * ddt_scale), eq98_e1302_d_n7, (var_qgsfps2_dn10 * ddt_scale), eq98_e1302_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq98_value: f64 = eq98_e1304;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq98_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq98_e1304_d_n2), multiplicity * (eq98_e1304_d_n4), multiplicity * (eq98_e1304_d_n7), multiplicity * (eq98_e1304_d_n10), multiplicity * (eq98_e1304_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq99_e1314, eq99_e1314_d_n2, eq99_e1314_d_n4, eq99_e1314_d_n7, eq99_e1314_d_n10, eq99_e1314_d_n11,) = {
    if (var_guard239 != 0.0) {
        let eq99_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 86, var_qgdfps2);
        let eq99_e1310: f64 = (p.p355 * (nv7 - nv10));
        let eq99_e1311: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 87, eq99_e1310);
        let eq99_e1312: f64 = (eq99_e1307 + eq99_e1311);
        let eq99_e1312_d_n7: f64 = ((var_qgdfps2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq99_e1312_d_n10: f64 = ((var_qgdfps2_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq99_e1312, (var_qgdfps2_dn2 * ddt_scale), (var_qgdfps2_dn4 * ddt_scale), eq99_e1312_d_n7, eq99_e1312_d_n10, (var_qgdfps2_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq99_value: f64 = eq99_e1314;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(10),
            multiplicity * (eq99_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq99_e1314_d_n2), multiplicity * (eq99_e1314_d_n4), multiplicity * (eq99_e1314_d_n7), multiplicity * (eq99_e1314_d_n10), multiplicity * (eq99_e1314_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq100_e1324, eq100_e1324_d_n2, eq100_e1324_d_n3, eq100_e1324_d_n4, eq100_e1324_d_n7, eq100_e1324_d_n10, eq100_e1324_d_n11,) = {
    if (var_guard239 != 0.0) {
        let eq100_e1317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 88, var_qcfps2);
        let eq100_e1320: f64 = (p.p355 * (nv2 - nv11));
        let eq100_e1321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 89, eq100_e1320);
        let eq100_e1322: f64 = (eq100_e1317 + eq100_e1321);
        let eq100_e1322_d_n2: f64 = ((var_qcfps2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq100_e1322_d_n11: f64 = ((var_qcfps2_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq100_e1322, eq100_e1322_d_n2, (var_qcfps2_dn3 * ddt_scale), (var_qcfps2_dn4 * ddt_scale), (var_qcfps2_dn7 * ddt_scale), (var_qcfps2_dn10 * ddt_scale), eq100_e1322_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq100_value: f64 = eq100_e1324;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq100_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq100_e1324_d_n2), multiplicity * (eq100_e1324_d_n3), multiplicity * (eq100_e1324_d_n4), multiplicity * (eq100_e1324_d_n7), multiplicity * (eq100_e1324_d_n10), multiplicity * (eq100_e1324_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq102_e1338, eq102_e1338_d_n2, eq102_e1338_d_n3, eq102_e1338_d_n4, eq102_e1338_d_n7, eq102_e1338_d_n9, eq102_e1338_d_n10, eq102_e1338_d_n11,) = {
    if (var_guard239 != 0.0) {
        let eq102_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 90, var_qsfps2);
        let eq102_e1334: f64 = (p.p355 * (nv7 - nv9));
        let eq102_e1335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 91, eq102_e1334);
        let eq102_e1336: f64 = (eq102_e1331 + eq102_e1335);
        let eq102_e1336_d_n7: f64 = ((var_qsfps2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq102_e1336, (var_qsfps2_dn2 * ddt_scale), (var_qsfps2_dn3 * ddt_scale), (var_qsfps2_dn4 * ddt_scale), eq102_e1336_d_n7, ((-p.p355) * ddt_scale), (var_qsfps2_dn10 * ddt_scale), (var_qsfps2_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq102_value: f64 = eq102_e1338;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq102_value),
            [2, 3, 4, 7, 9, 10, 11],
            [multiplicity * (eq102_e1338_d_n2), multiplicity * (eq102_e1338_d_n3), multiplicity * (eq102_e1338_d_n4), multiplicity * (eq102_e1338_d_n7), multiplicity * (eq102_e1338_d_n9), multiplicity * (eq102_e1338_d_n10), multiplicity * (eq102_e1338_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq103_e1349, eq103_e1349_d_n2, eq103_e1349_d_n4, eq103_e1349_d_n7, eq103_e1349_d_n10, eq103_e1349_d_n11,) = {
    if (var_guard239 == 0.0) {
        let eq103_e1342: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 92, var_qgsfps2);
        let eq103_e1345: f64 = (p.p355 * (nv2 - nv11));
        let eq103_e1346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 93, eq103_e1345);
        let eq103_e1347: f64 = (eq103_e1342 + eq103_e1346);
        let eq103_e1347_d_n2: f64 = ((var_qgsfps2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq103_e1347_d_n11: f64 = ((var_qgsfps2_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq103_e1347, eq103_e1347_d_n2, (var_qgsfps2_dn4 * ddt_scale), (var_qgsfps2_dn7 * ddt_scale), (var_qgsfps2_dn10 * ddt_scale), eq103_e1347_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq103_value: f64 = eq103_e1349;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq103_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq103_e1349_d_n2), multiplicity * (eq103_e1349_d_n4), multiplicity * (eq103_e1349_d_n7), multiplicity * (eq103_e1349_d_n10), multiplicity * (eq103_e1349_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq104_e1360, eq104_e1360_d_n2, eq104_e1360_d_n4, eq104_e1360_d_n7, eq104_e1360_d_n10, eq104_e1360_d_n11,) = {
    if (var_guard239 == 0.0) {
        let eq104_e1353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 94, var_qgdfps2);
        let eq104_e1356: f64 = (p.p355 * (nv2 - nv10));
        let eq104_e1357: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 95, eq104_e1356);
        let eq104_e1358: f64 = (eq104_e1353 + eq104_e1357);
        let eq104_e1358_d_n2: f64 = ((var_qgdfps2_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq104_e1358_d_n10: f64 = ((var_qgdfps2_dn10 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq104_e1358, eq104_e1358_d_n2, (var_qgdfps2_dn4 * ddt_scale), (var_qgdfps2_dn7 * ddt_scale), eq104_e1358_d_n10, (var_qgdfps2_dn11 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq104_value: f64 = eq104_e1360;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(10),
            multiplicity * (eq104_value),
            [2, 4, 7, 10, 11],
            [multiplicity * (eq104_e1360_d_n2), multiplicity * (eq104_e1360_d_n4), multiplicity * (eq104_e1360_d_n7), multiplicity * (eq104_e1360_d_n10), multiplicity * (eq104_e1360_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq105_e1371, eq105_e1371_d_n2, eq105_e1371_d_n3, eq105_e1371_d_n4, eq105_e1371_d_n7, eq105_e1371_d_n10, eq105_e1371_d_n11,) = {
    if (var_guard239 == 0.0) {
        let eq105_e1364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 96, var_qcfps2);
        let eq105_e1367: f64 = (p.p355 * (nv7 - nv11));
        let eq105_e1368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 97, eq105_e1367);
        let eq105_e1369: f64 = (eq105_e1364 + eq105_e1368);
        let eq105_e1369_d_n7: f64 = ((var_qcfps2_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq105_e1369_d_n11: f64 = ((var_qcfps2_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq105_e1369, (var_qcfps2_dn2 * ddt_scale), (var_qcfps2_dn3 * ddt_scale), (var_qcfps2_dn4 * ddt_scale), eq105_e1369_d_n7, (var_qcfps2_dn10 * ddt_scale), eq105_e1369_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq105_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * (eq105_e1371_d_n2), multiplicity * (eq105_e1371_d_n3), multiplicity * (eq105_e1371_d_n4), multiplicity * (eq105_e1371_d_n7), multiplicity * (eq105_e1371_d_n10), multiplicity * (eq105_e1371_d_n11)],
            [],
            [],
            1.0,
        );
        let eq108_e1383: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 98, var_qbfps2);
        let eq108_e1386: f64 = (p.p355 * (nv3 - nv11));
        let eq108_e1387: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 99, eq108_e1386);
        let eq108_e1388: f64 = (eq108_e1383 + eq108_e1387);
        let eq108_e1388_d_n3: f64 = ((var_qbfps2_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq108_e1388_d_n11: f64 = ((var_qbfps2_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq108_value: f64 = eq108_e1388;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(11),
            multiplicity * (eq108_value),
            [2, 3, 4, 7, 10, 11],
            [multiplicity * ((var_qbfps2_dn2 * ddt_scale)), multiplicity * (eq108_e1388_d_n3), multiplicity * ((var_qbfps2_dn4 * ddt_scale)), multiplicity * ((var_qbfps2_dn7 * ddt_scale)), multiplicity * ((var_qbfps2_dn10 * ddt_scale)), multiplicity * (eq108_e1388_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq109_e1396, eq109_e1396_d_n2, eq109_e1396_d_n3, eq109_e1396_d_n4, eq109_e1396_d_n7, eq109_e1396_d_n11, eq109_e1396_d_n12,) = {
    if (var_guard240 != 0.0) {
        let eq109_e1393: f64 = (var_gmin * (nv11 - nv12));
        let eq109_e1394: f64 = (var_idsfps3 + eq109_e1393);
        let eq109_e1394_d_n11: f64 = (var_idsfps3_dn11 + var_gmin);
        let eq109_e1394_d_n12: f64 = (var_idsfps3_dn12 + (-var_gmin));
        (eq109_e1394, var_idsfps3_dn2, var_idsfps3_dn3, var_idsfps3_dn4, var_idsfps3_dn7, eq109_e1394_d_n11, eq109_e1394_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e1396;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq109_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq109_e1396_d_n2), multiplicity * (eq109_e1396_d_n3), multiplicity * (eq109_e1396_d_n4), multiplicity * (eq109_e1396_d_n7), multiplicity * (eq109_e1396_d_n11), multiplicity * (eq109_e1396_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq111_e1411, eq111_e1411_d_n2, eq111_e1411_d_n4, eq111_e1411_d_n7, eq111_e1411_d_n11, eq111_e1411_d_n12,) = {
    if (var_guard275 != 0.0) {
        let eq111_e1404: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 100, var_qgsfps3);
        let eq111_e1407: f64 = (p.p355 * (nv7 - nv12));
        let eq111_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 101, eq111_e1407);
        let eq111_e1409: f64 = (eq111_e1404 + eq111_e1408);
        let eq111_e1409_d_n7: f64 = ((var_qgsfps3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq111_e1409_d_n12: f64 = ((var_qgsfps3_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq111_e1409, (var_qgsfps3_dn2 * ddt_scale), (var_qgsfps3_dn4 * ddt_scale), eq111_e1409_d_n7, (var_qgsfps3_dn11 * ddt_scale), eq111_e1409_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e1411;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq111_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq111_e1411_d_n2), multiplicity * (eq111_e1411_d_n4), multiplicity * (eq111_e1411_d_n7), multiplicity * (eq111_e1411_d_n11), multiplicity * (eq111_e1411_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq112_e1421, eq112_e1421_d_n2, eq112_e1421_d_n4, eq112_e1421_d_n7, eq112_e1421_d_n11, eq112_e1421_d_n12,) = {
    if (var_guard275 != 0.0) {
        let eq112_e1414: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 102, var_qgdfps3);
        let eq112_e1417: f64 = (p.p355 * (nv7 - nv11));
        let eq112_e1418: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 103, eq112_e1417);
        let eq112_e1419: f64 = (eq112_e1414 + eq112_e1418);
        let eq112_e1419_d_n7: f64 = ((var_qgdfps3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq112_e1419_d_n11: f64 = ((var_qgdfps3_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq112_e1419, (var_qgdfps3_dn2 * ddt_scale), (var_qgdfps3_dn4 * ddt_scale), eq112_e1419_d_n7, eq112_e1419_d_n11, (var_qgdfps3_dn12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq112_value: f64 = eq112_e1421;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(11),
            multiplicity * (eq112_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq112_e1421_d_n2), multiplicity * (eq112_e1421_d_n4), multiplicity * (eq112_e1421_d_n7), multiplicity * (eq112_e1421_d_n11), multiplicity * (eq112_e1421_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq113_e1431, eq113_e1431_d_n2, eq113_e1431_d_n3, eq113_e1431_d_n4, eq113_e1431_d_n7, eq113_e1431_d_n11, eq113_e1431_d_n12,) = {
    if (var_guard275 != 0.0) {
        let eq113_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 104, var_qcfps3);
        let eq113_e1427: f64 = (p.p355 * (nv2 - nv12));
        let eq113_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 105, eq113_e1427);
        let eq113_e1429: f64 = (eq113_e1424 + eq113_e1428);
        let eq113_e1429_d_n2: f64 = ((var_qcfps3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq113_e1429_d_n12: f64 = ((var_qcfps3_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq113_e1429, eq113_e1429_d_n2, (var_qcfps3_dn3 * ddt_scale), (var_qcfps3_dn4 * ddt_scale), (var_qcfps3_dn7 * ddt_scale), (var_qcfps3_dn11 * ddt_scale), eq113_e1429_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq113_value: f64 = eq113_e1431;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq113_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq113_e1431_d_n2), multiplicity * (eq113_e1431_d_n3), multiplicity * (eq113_e1431_d_n4), multiplicity * (eq113_e1431_d_n7), multiplicity * (eq113_e1431_d_n11), multiplicity * (eq113_e1431_d_n12)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard275: f64,
        var_guard276: f64,
        var_guard311: f64,
        var_guard312: f64,
        var_guard347: f64,
        var_guard416: f64,
        var_guard417: f64,
        var_ids: f64,
        var_ids_dn22: f64,
        var_ids_dn23: f64,
        var_ids_dn25: f64,
        var_ids_dn26: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_idsfps4: f64,
        var_idsfps4_dn12: f64,
        var_idsfps4_dn13: f64,
        var_idsfps4_dn2: f64,
        var_idsfps4_dn3: f64,
        var_idsfps4_dn4: f64,
        var_idsfps4_dn7: f64,
        var_idsrd: f64,
        var_idsrd_dn0: f64,
        var_idsrd_dn17: f64,
        var_idsrd_dn18: f64,
        var_idsrd_dn2: f64,
        var_idsrd_dn20: f64,
        var_idsrd_dn4: f64,
        var_idsrs: f64,
        var_idsrs_dn0: f64,
        var_idsrs_dn13: f64,
        var_idsrs_dn19: f64,
        var_idsrs_dn2: f64,
        var_idsrs_dn4: f64,
        var_igdi: f64,
        var_igdi_dn17: f64,
        var_igdi_dn4: f64,
        var_igdi_dn8: f64,
        var_igsi: f64,
        var_igsi_dn13: f64,
        var_igsi_dn4: f64,
        var_igsi_dn8: f64,
        var_qbfps3: f64,
        var_qbfps3_dn11: f64,
        var_qbfps3_dn12: f64,
        var_qbfps3_dn2: f64,
        var_qbfps3_dn3: f64,
        var_qbfps3_dn4: f64,
        var_qbfps3_dn7: f64,
        var_qbfps4: f64,
        var_qbfps4_dn12: f64,
        var_qbfps4_dn13: f64,
        var_qbfps4_dn2: f64,
        var_qbfps4_dn3: f64,
        var_qbfps4_dn4: f64,
        var_qbfps4_dn7: f64,
        var_qcfps3: f64,
        var_qcfps3_dn11: f64,
        var_qcfps3_dn12: f64,
        var_qcfps3_dn2: f64,
        var_qcfps3_dn3: f64,
        var_qcfps3_dn4: f64,
        var_qcfps3_dn7: f64,
        var_qcfps4: f64,
        var_qcfps4_dn12: f64,
        var_qcfps4_dn13: f64,
        var_qcfps4_dn2: f64,
        var_qcfps4_dn3: f64,
        var_qcfps4_dn4: f64,
        var_qcfps4_dn7: f64,
        var_qgd: f64,
        var_qgd_dn22: f64,
        var_qgd_dn23: f64,
        var_qgd_dn25: f64,
        var_qgd_dn26: f64,
        var_qgd_dn4: f64,
        var_qgd_dn5: f64,
        var_qgd_dn8: f64,
        var_qgd_dn9: f64,
        var_qgdfps3: f64,
        var_qgdfps3_dn11: f64,
        var_qgdfps3_dn12: f64,
        var_qgdfps3_dn2: f64,
        var_qgdfps3_dn4: f64,
        var_qgdfps3_dn7: f64,
        var_qgdfps4: f64,
        var_qgdfps4_dn12: f64,
        var_qgdfps4_dn13: f64,
        var_qgdfps4_dn2: f64,
        var_qgdfps4_dn4: f64,
        var_qgdfps4_dn7: f64,
        var_qgs: f64,
        var_qgs_dn22: f64,
        var_qgs_dn23: f64,
        var_qgs_dn25: f64,
        var_qgs_dn26: f64,
        var_qgs_dn4: f64,
        var_qgs_dn5: f64,
        var_qgs_dn8: f64,
        var_qgs_dn9: f64,
        var_qgsfps3: f64,
        var_qgsfps3_dn11: f64,
        var_qgsfps3_dn12: f64,
        var_qgsfps3_dn2: f64,
        var_qgsfps3_dn4: f64,
        var_qgsfps3_dn7: f64,
        var_qgsfps4: f64,
        var_qgsfps4_dn12: f64,
        var_qgsfps4_dn13: f64,
        var_qgsfps4_dn2: f64,
        var_qgsfps4_dn4: f64,
        var_qgsfps4_dn7: f64,
        var_qsfps3: f64,
        var_qsfps3_dn11: f64,
        var_qsfps3_dn12: f64,
        var_qsfps3_dn2: f64,
        var_qsfps3_dn3: f64,
        var_qsfps3_dn4: f64,
        var_qsfps3_dn7: f64,
        var_qsfps4: f64,
        var_qsfps4_dn12: f64,
        var_qsfps4_dn13: f64,
        var_qsfps4_dn2: f64,
        var_qsfps4_dn3: f64,
        var_qsfps4_dn4: f64,
        var_qsfps4_dn7: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let nv28 = ctx.node_voltage(nodes[28]);
        let nv29 = ctx.node_voltage(nodes[29]);
        let (eq115_e1445, eq115_e1445_d_n2, eq115_e1445_d_n3, eq115_e1445_d_n4, eq115_e1445_d_n7, eq115_e1445_d_n9, eq115_e1445_d_n11, eq115_e1445_d_n12,) = {
    if (var_guard275 != 0.0) {
        let eq115_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 106, var_qsfps3);
        let eq115_e1441: f64 = (p.p355 * (nv7 - nv9));
        let eq115_e1442: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 107, eq115_e1441);
        let eq115_e1443: f64 = (eq115_e1438 + eq115_e1442);
        let eq115_e1443_d_n7: f64 = ((var_qsfps3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq115_e1443, (var_qsfps3_dn2 * ddt_scale), (var_qsfps3_dn3 * ddt_scale), (var_qsfps3_dn4 * ddt_scale), eq115_e1443_d_n7, ((-p.p355) * ddt_scale), (var_qsfps3_dn11 * ddt_scale), (var_qsfps3_dn12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq115_value: f64 = eq115_e1445;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq115_value),
            [2, 3, 4, 7, 9, 11, 12],
            [multiplicity * (eq115_e1445_d_n2), multiplicity * (eq115_e1445_d_n3), multiplicity * (eq115_e1445_d_n4), multiplicity * (eq115_e1445_d_n7), multiplicity * (eq115_e1445_d_n9), multiplicity * (eq115_e1445_d_n11), multiplicity * (eq115_e1445_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq116_e1456, eq116_e1456_d_n2, eq116_e1456_d_n4, eq116_e1456_d_n7, eq116_e1456_d_n11, eq116_e1456_d_n12,) = {
    if (var_guard275 == 0.0) {
        let eq116_e1449: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 108, var_qgsfps3);
        let eq116_e1452: f64 = (p.p355 * (nv2 - nv12));
        let eq116_e1453: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 109, eq116_e1452);
        let eq116_e1454: f64 = (eq116_e1449 + eq116_e1453);
        let eq116_e1454_d_n2: f64 = ((var_qgsfps3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq116_e1454_d_n12: f64 = ((var_qgsfps3_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq116_e1454, eq116_e1454_d_n2, (var_qgsfps3_dn4 * ddt_scale), (var_qgsfps3_dn7 * ddt_scale), (var_qgsfps3_dn11 * ddt_scale), eq116_e1454_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq116_value: f64 = eq116_e1456;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq116_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq116_e1456_d_n2), multiplicity * (eq116_e1456_d_n4), multiplicity * (eq116_e1456_d_n7), multiplicity * (eq116_e1456_d_n11), multiplicity * (eq116_e1456_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq117_e1467, eq117_e1467_d_n2, eq117_e1467_d_n4, eq117_e1467_d_n7, eq117_e1467_d_n11, eq117_e1467_d_n12,) = {
    if (var_guard275 == 0.0) {
        let eq117_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 110, var_qgdfps3);
        let eq117_e1463: f64 = (p.p355 * (nv2 - nv11));
        let eq117_e1464: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 111, eq117_e1463);
        let eq117_e1465: f64 = (eq117_e1460 + eq117_e1464);
        let eq117_e1465_d_n2: f64 = ((var_qgdfps3_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq117_e1465_d_n11: f64 = ((var_qgdfps3_dn11 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq117_e1465, eq117_e1465_d_n2, (var_qgdfps3_dn4 * ddt_scale), (var_qgdfps3_dn7 * ddt_scale), eq117_e1465_d_n11, (var_qgdfps3_dn12 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq117_value: f64 = eq117_e1467;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(11),
            multiplicity * (eq117_value),
            [2, 4, 7, 11, 12],
            [multiplicity * (eq117_e1467_d_n2), multiplicity * (eq117_e1467_d_n4), multiplicity * (eq117_e1467_d_n7), multiplicity * (eq117_e1467_d_n11), multiplicity * (eq117_e1467_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq118_e1478, eq118_e1478_d_n2, eq118_e1478_d_n3, eq118_e1478_d_n4, eq118_e1478_d_n7, eq118_e1478_d_n11, eq118_e1478_d_n12,) = {
    if (var_guard275 == 0.0) {
        let eq118_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 112, var_qcfps3);
        let eq118_e1474: f64 = (p.p355 * (nv7 - nv12));
        let eq118_e1475: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 113, eq118_e1474);
        let eq118_e1476: f64 = (eq118_e1471 + eq118_e1475);
        let eq118_e1476_d_n7: f64 = ((var_qcfps3_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq118_e1476_d_n12: f64 = ((var_qcfps3_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq118_e1476, (var_qcfps3_dn2 * ddt_scale), (var_qcfps3_dn3 * ddt_scale), (var_qcfps3_dn4 * ddt_scale), eq118_e1476_d_n7, (var_qcfps3_dn11 * ddt_scale), eq118_e1476_d_n12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq118_value: f64 = eq118_e1478;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq118_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * (eq118_e1478_d_n2), multiplicity * (eq118_e1478_d_n3), multiplicity * (eq118_e1478_d_n4), multiplicity * (eq118_e1478_d_n7), multiplicity * (eq118_e1478_d_n11), multiplicity * (eq118_e1478_d_n12)],
            [],
            [],
            1.0,
        );
        let eq121_e1490: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 114, var_qbfps3);
        let eq121_e1493: f64 = (p.p355 * (nv3 - nv12));
        let eq121_e1494: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 115, eq121_e1493);
        let eq121_e1495: f64 = (eq121_e1490 + eq121_e1494);
        let eq121_e1495_d_n3: f64 = ((var_qbfps3_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq121_e1495_d_n12: f64 = ((var_qbfps3_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq121_value: f64 = eq121_e1495;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(12),
            multiplicity * (eq121_value),
            [2, 3, 4, 7, 11, 12],
            [multiplicity * ((var_qbfps3_dn2 * ddt_scale)), multiplicity * (eq121_e1495_d_n3), multiplicity * ((var_qbfps3_dn4 * ddt_scale)), multiplicity * ((var_qbfps3_dn7 * ddt_scale)), multiplicity * ((var_qbfps3_dn11 * ddt_scale)), multiplicity * (eq121_e1495_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq122_e1503, eq122_e1503_d_n2, eq122_e1503_d_n3, eq122_e1503_d_n4, eq122_e1503_d_n7, eq122_e1503_d_n12, eq122_e1503_d_n13,) = {
    if (var_guard276 != 0.0) {
        let eq122_e1500: f64 = (var_gmin * (nv12 - nv13));
        let eq122_e1501: f64 = (var_idsfps4 + eq122_e1500);
        let eq122_e1501_d_n12: f64 = (var_idsfps4_dn12 + var_gmin);
        let eq122_e1501_d_n13: f64 = (var_idsfps4_dn13 + (-var_gmin));
        (eq122_e1501, var_idsfps4_dn2, var_idsfps4_dn3, var_idsfps4_dn4, var_idsfps4_dn7, eq122_e1501_d_n12, eq122_e1501_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq122_value: f64 = eq122_e1503;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(13),
            multiplicity * (eq122_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq122_e1503_d_n2), multiplicity * (eq122_e1503_d_n3), multiplicity * (eq122_e1503_d_n4), multiplicity * (eq122_e1503_d_n7), multiplicity * (eq122_e1503_d_n12), multiplicity * (eq122_e1503_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq124_e1518, eq124_e1518_d_n2, eq124_e1518_d_n4, eq124_e1518_d_n7, eq124_e1518_d_n12, eq124_e1518_d_n13,) = {
    if (var_guard311 != 0.0) {
        let eq124_e1511: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 116, var_qgsfps4);
        let eq124_e1514: f64 = (p.p355 * (nv7 - nv13));
        let eq124_e1515: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 117, eq124_e1514);
        let eq124_e1516: f64 = (eq124_e1511 + eq124_e1515);
        let eq124_e1516_d_n7: f64 = ((var_qgsfps4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq124_e1516_d_n13: f64 = ((var_qgsfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq124_e1516, (var_qgsfps4_dn2 * ddt_scale), (var_qgsfps4_dn4 * ddt_scale), eq124_e1516_d_n7, (var_qgsfps4_dn12 * ddt_scale), eq124_e1516_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq124_value: f64 = eq124_e1518;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(13),
            multiplicity * (eq124_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq124_e1518_d_n2), multiplicity * (eq124_e1518_d_n4), multiplicity * (eq124_e1518_d_n7), multiplicity * (eq124_e1518_d_n12), multiplicity * (eq124_e1518_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq125_e1528, eq125_e1528_d_n2, eq125_e1528_d_n4, eq125_e1528_d_n7, eq125_e1528_d_n12, eq125_e1528_d_n13,) = {
    if (var_guard311 != 0.0) {
        let eq125_e1521: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 118, var_qgdfps4);
        let eq125_e1524: f64 = (p.p355 * (nv7 - nv12));
        let eq125_e1525: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 119, eq125_e1524);
        let eq125_e1526: f64 = (eq125_e1521 + eq125_e1525);
        let eq125_e1526_d_n7: f64 = ((var_qgdfps4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq125_e1526_d_n12: f64 = ((var_qgdfps4_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq125_e1526, (var_qgdfps4_dn2 * ddt_scale), (var_qgdfps4_dn4 * ddt_scale), eq125_e1526_d_n7, eq125_e1526_d_n12, (var_qgdfps4_dn13 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq125_value: f64 = eq125_e1528;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq125_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq125_e1528_d_n2), multiplicity * (eq125_e1528_d_n4), multiplicity * (eq125_e1528_d_n7), multiplicity * (eq125_e1528_d_n12), multiplicity * (eq125_e1528_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq126_e1538, eq126_e1538_d_n2, eq126_e1538_d_n3, eq126_e1538_d_n4, eq126_e1538_d_n7, eq126_e1538_d_n12, eq126_e1538_d_n13,) = {
    if (var_guard311 != 0.0) {
        let eq126_e1531: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 120, var_qcfps4);
        let eq126_e1534: f64 = (p.p355 * (nv2 - nv13));
        let eq126_e1535: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 121, eq126_e1534);
        let eq126_e1536: f64 = (eq126_e1531 + eq126_e1535);
        let eq126_e1536_d_n2: f64 = ((var_qcfps4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq126_e1536_d_n13: f64 = ((var_qcfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq126_e1536, eq126_e1536_d_n2, (var_qcfps4_dn3 * ddt_scale), (var_qcfps4_dn4 * ddt_scale), (var_qcfps4_dn7 * ddt_scale), (var_qcfps4_dn12 * ddt_scale), eq126_e1536_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq126_value: f64 = eq126_e1538;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(13),
            multiplicity * (eq126_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq126_e1538_d_n2), multiplicity * (eq126_e1538_d_n3), multiplicity * (eq126_e1538_d_n4), multiplicity * (eq126_e1538_d_n7), multiplicity * (eq126_e1538_d_n12), multiplicity * (eq126_e1538_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq128_e1552, eq128_e1552_d_n2, eq128_e1552_d_n3, eq128_e1552_d_n4, eq128_e1552_d_n7, eq128_e1552_d_n9, eq128_e1552_d_n12, eq128_e1552_d_n13,) = {
    if (var_guard311 != 0.0) {
        let eq128_e1545: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 122, var_qsfps4);
        let eq128_e1548: f64 = (p.p355 * (nv7 - nv9));
        let eq128_e1549: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 123, eq128_e1548);
        let eq128_e1550: f64 = (eq128_e1545 + eq128_e1549);
        let eq128_e1550_d_n7: f64 = ((var_qsfps4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        (eq128_e1550, (var_qsfps4_dn2 * ddt_scale), (var_qsfps4_dn3 * ddt_scale), (var_qsfps4_dn4 * ddt_scale), eq128_e1550_d_n7, ((-p.p355) * ddt_scale), (var_qsfps4_dn12 * ddt_scale), (var_qsfps4_dn13 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq128_value: f64 = eq128_e1552;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq128_value),
            [2, 3, 4, 7, 9, 12, 13],
            [multiplicity * (eq128_e1552_d_n2), multiplicity * (eq128_e1552_d_n3), multiplicity * (eq128_e1552_d_n4), multiplicity * (eq128_e1552_d_n7), multiplicity * (eq128_e1552_d_n9), multiplicity * (eq128_e1552_d_n12), multiplicity * (eq128_e1552_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq129_e1563, eq129_e1563_d_n2, eq129_e1563_d_n4, eq129_e1563_d_n7, eq129_e1563_d_n12, eq129_e1563_d_n13,) = {
    if (var_guard311 == 0.0) {
        let eq129_e1556: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 124, var_qgsfps4);
        let eq129_e1559: f64 = (p.p355 * (nv2 - nv13));
        let eq129_e1560: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 125, eq129_e1559);
        let eq129_e1561: f64 = (eq129_e1556 + eq129_e1560);
        let eq129_e1561_d_n2: f64 = ((var_qgsfps4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq129_e1561_d_n13: f64 = ((var_qgsfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq129_e1561, eq129_e1561_d_n2, (var_qgsfps4_dn4 * ddt_scale), (var_qgsfps4_dn7 * ddt_scale), (var_qgsfps4_dn12 * ddt_scale), eq129_e1561_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq129_value: f64 = eq129_e1563;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(13),
            multiplicity * (eq129_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq129_e1563_d_n2), multiplicity * (eq129_e1563_d_n4), multiplicity * (eq129_e1563_d_n7), multiplicity * (eq129_e1563_d_n12), multiplicity * (eq129_e1563_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq130_e1574, eq130_e1574_d_n2, eq130_e1574_d_n4, eq130_e1574_d_n7, eq130_e1574_d_n12, eq130_e1574_d_n13,) = {
    if (var_guard311 == 0.0) {
        let eq130_e1567: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 126, var_qgdfps4);
        let eq130_e1570: f64 = (p.p355 * (nv2 - nv12));
        let eq130_e1571: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 127, eq130_e1570);
        let eq130_e1572: f64 = (eq130_e1567 + eq130_e1571);
        let eq130_e1572_d_n2: f64 = ((var_qgdfps4_dn2 * ddt_scale) + (p.p355 * ddt_scale));
        let eq130_e1572_d_n12: f64 = ((var_qgdfps4_dn12 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq130_e1572, eq130_e1572_d_n2, (var_qgdfps4_dn4 * ddt_scale), (var_qgdfps4_dn7 * ddt_scale), eq130_e1572_d_n12, (var_qgdfps4_dn13 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq130_value: f64 = eq130_e1574;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(2),
            Some(12),
            multiplicity * (eq130_value),
            [2, 4, 7, 12, 13],
            [multiplicity * (eq130_e1574_d_n2), multiplicity * (eq130_e1574_d_n4), multiplicity * (eq130_e1574_d_n7), multiplicity * (eq130_e1574_d_n12), multiplicity * (eq130_e1574_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq131_e1585, eq131_e1585_d_n2, eq131_e1585_d_n3, eq131_e1585_d_n4, eq131_e1585_d_n7, eq131_e1585_d_n12, eq131_e1585_d_n13,) = {
    if (var_guard311 == 0.0) {
        let eq131_e1578: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 128, var_qcfps4);
        let eq131_e1581: f64 = (p.p355 * (nv7 - nv13));
        let eq131_e1582: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 129, eq131_e1581);
        let eq131_e1583: f64 = (eq131_e1578 + eq131_e1582);
        let eq131_e1583_d_n7: f64 = ((var_qcfps4_dn7 * ddt_scale) + (p.p355 * ddt_scale));
        let eq131_e1583_d_n13: f64 = ((var_qcfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        (eq131_e1583, (var_qcfps4_dn2 * ddt_scale), (var_qcfps4_dn3 * ddt_scale), (var_qcfps4_dn4 * ddt_scale), eq131_e1583_d_n7, (var_qcfps4_dn12 * ddt_scale), eq131_e1583_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq131_value: f64 = eq131_e1585;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(13),
            multiplicity * (eq131_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * (eq131_e1585_d_n2), multiplicity * (eq131_e1585_d_n3), multiplicity * (eq131_e1585_d_n4), multiplicity * (eq131_e1585_d_n7), multiplicity * (eq131_e1585_d_n12), multiplicity * (eq131_e1585_d_n13)],
            [],
            [],
            1.0,
        );
        let eq134_e1597: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 130, var_qbfps4);
        let eq134_e1600: f64 = (p.p355 * (nv3 - nv13));
        let eq134_e1601: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 131, eq134_e1600);
        let eq134_e1602: f64 = (eq134_e1597 + eq134_e1601);
        let eq134_e1602_d_n3: f64 = ((var_qbfps4_dn3 * ddt_scale) + (p.p355 * ddt_scale));
        let eq134_e1602_d_n13: f64 = ((var_qbfps4_dn13 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq134_value: f64 = eq134_e1602;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(3),
            Some(13),
            multiplicity * (eq134_value),
            [2, 3, 4, 7, 12, 13],
            [multiplicity * ((var_qbfps4_dn2 * ddt_scale)), multiplicity * (eq134_e1602_d_n3), multiplicity * ((var_qbfps4_dn4 * ddt_scale)), multiplicity * ((var_qbfps4_dn7 * ddt_scale)), multiplicity * ((var_qbfps4_dn12 * ddt_scale)), multiplicity * (eq134_e1602_d_n13)],
            [],
            [],
            1.0,
        );
        let (eq135_e1610, eq135_e1610_d_n0, eq135_e1610_d_n2, eq135_e1610_d_n4, eq135_e1610_d_n13, eq135_e1610_d_n19,) = {
    if (var_guard312 != 0.0) {
        let eq135_e1607: f64 = (var_gmin * (nv13 - nv19));
        let eq135_e1608: f64 = (var_idsrs + eq135_e1607);
        let eq135_e1608_d_n13: f64 = (var_idsrs_dn13 + var_gmin);
        let eq135_e1608_d_n19: f64 = (var_idsrs_dn19 + (-var_gmin));
        (eq135_e1608, var_idsrs_dn0, var_idsrs_dn2, var_idsrs_dn4, eq135_e1608_d_n13, eq135_e1608_d_n19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq135_value: f64 = eq135_e1610;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(13),
            Some(19),
            multiplicity * (eq135_value),
            [0, 2, 4, 13, 19],
            [multiplicity * (eq135_e1610_d_n0), multiplicity * (eq135_e1610_d_n2), multiplicity * (eq135_e1610_d_n4), multiplicity * (eq135_e1610_d_n13), multiplicity * (eq135_e1610_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq137_e1623, eq137_e1623_d_n0, eq137_e1623_d_n2, eq137_e1623_d_n4, eq137_e1623_d_n17, eq137_e1623_d_n18, eq137_e1623_d_n20,) = {
    if (var_guard347 != 0.0) {
        let eq137_e1620: f64 = (var_gmin * (nv18 - nv17));
        let eq137_e1621: f64 = (var_idsrd + eq137_e1620);
        let eq137_e1621_d_n17: f64 = (var_idsrd_dn17 + (-var_gmin));
        let eq137_e1621_d_n18: f64 = (var_idsrd_dn18 + var_gmin);
        (eq137_e1621, var_idsrd_dn0, var_idsrd_dn2, var_idsrd_dn4, eq137_e1621_d_n17, eq137_e1621_d_n18, var_idsrd_dn20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq137_value: f64 = eq137_e1623;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(18),
            Some(17),
            multiplicity * (eq137_value),
            [0, 2, 4, 17, 18, 20],
            [multiplicity * (eq137_e1623_d_n0), multiplicity * (eq137_e1623_d_n2), multiplicity * (eq137_e1623_d_n4), multiplicity * (eq137_e1623_d_n17), multiplicity * (eq137_e1623_d_n18), multiplicity * (eq137_e1623_d_n20)],
            [],
            [],
            1.0,
        );
        let (eq141_e1644, eq141_e1644_d_n4, eq141_e1644_d_n5, eq141_e1644_d_n8, eq141_e1644_d_n9, eq141_e1644_d_n22, eq141_e1644_d_n23, eq141_e1644_d_n25, eq141_e1644_d_n26,) = {
    if (var_guard416 != 0.0) {
        let eq141_e1641: f64 = (var_gmin * (nv5 - nv9));
        let eq141_e1642: f64 = (var_ids + eq141_e1641);
        let eq141_e1642_d_n5: f64 = (var_ids_dn5 + var_gmin);
        let eq141_e1642_d_n9: f64 = (var_ids_dn9 + (-var_gmin));
        (eq141_e1642, var_ids_dn4, eq141_e1642_d_n5, var_ids_dn8, eq141_e1642_d_n9, var_ids_dn22, var_ids_dn23, var_ids_dn25, var_ids_dn26,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq141_value: f64 = eq141_e1644;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(9),
            multiplicity * (eq141_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * (eq141_e1644_d_n4), multiplicity * (eq141_e1644_d_n5), multiplicity * (eq141_e1644_d_n8), multiplicity * (eq141_e1644_d_n9), multiplicity * (eq141_e1644_d_n22), multiplicity * (eq141_e1644_d_n23), multiplicity * (eq141_e1644_d_n25), multiplicity * (eq141_e1644_d_n26)],
            [],
            [],
            1.0,
        );
        let (eq142_e1656, eq142_e1656_d_n4, eq142_e1656_d_n5, eq142_e1656_d_n8, eq142_e1656_d_n9, eq142_e1656_d_n22, eq142_e1656_d_n23, eq142_e1656_d_n25, eq142_e1656_d_n26, eq142_e1656_d_n28, eq142_e1656_d_n29,) = {
    if (var_guard416 == 0.0) {
        let eq142_e1649: f64 = (var_ids - (nv29 - 0.0));
        let eq142_e1652: f64 = (p.p323 * (nv28 - 0.0));
        let eq142_e1653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 132, eq142_e1652);
        let eq142_e1654: f64 = (eq142_e1649 - eq142_e1653);
        let eq142_e1654_d_n28: f64 = (-(p.p323 * ddt_scale));
        (eq142_e1654, var_ids_dn4, var_ids_dn5, var_ids_dn8, var_ids_dn9, var_ids_dn22, var_ids_dn23, var_ids_dn25, var_ids_dn26, eq142_e1654_d_n28, (-1.0),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq142_value: f64 = eq142_e1656;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(28),
            None,
            multiplicity * (eq142_value),
            [4, 5, 8, 9, 22, 23, 25, 26, 28, 29],
            [multiplicity * (eq142_e1656_d_n4), multiplicity * (eq142_e1656_d_n5), multiplicity * (eq142_e1656_d_n8), multiplicity * (eq142_e1656_d_n9), multiplicity * (eq142_e1656_d_n22), multiplicity * (eq142_e1656_d_n23), multiplicity * (eq142_e1656_d_n25), multiplicity * (eq142_e1656_d_n26), multiplicity * (eq142_e1656_d_n28), multiplicity * (eq142_e1656_d_n29)],
            [],
            [],
            1.0,
        );
        let (eq143_e1670, eq143_e1670_d_n28, eq143_e1670_d_n29,) = {
    if (var_guard416 == 0.0) {
        let eq143_e1661: f64 = ((nv28 - 0.0) - (nv29 - 0.0));
        let eq143_e1664: f64 = (p.p323 / 3.0);
        let eq143_e1666: f64 = (eq143_e1664 * (nv29 - 0.0));
        let eq143_e1667: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 133, eq143_e1666);
        let eq143_e1668: f64 = (eq143_e1661 - eq143_e1667);
        let eq143_e1668_d_n29: f64 = ((-1.0) - (eq143_e1664 * ddt_scale));
        (eq143_e1668, 1.0, eq143_e1668_d_n29,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq143_value: f64 = eq143_e1670;
        stamper.stamp_current_node2_local(
            Some(29),
            None,
            multiplicity * (eq143_value),
            28,
            multiplicity * (eq143_e1670_d_n28),
            29,
            multiplicity * (eq143_e1670_d_n29),
        );
        let eq145_e1681: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 134, var_qgs);
        let eq145_e1684: f64 = (p.p355 * (nv8 - nv9));
        let eq145_e1685: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 135, eq145_e1684);
        let eq145_e1686: f64 = (eq145_e1681 + eq145_e1685);
        let eq145_e1686_d_n8: f64 = ((var_qgs_dn8 * ddt_scale) + (p.p355 * ddt_scale));
        let eq145_e1686_d_n9: f64 = ((var_qgs_dn9 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq145_value: f64 = eq145_e1686;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq145_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((var_qgs_dn4 * ddt_scale)), multiplicity * ((var_qgs_dn5 * ddt_scale)), multiplicity * (eq145_e1686_d_n8), multiplicity * (eq145_e1686_d_n9), multiplicity * ((var_qgs_dn22 * ddt_scale)), multiplicity * ((var_qgs_dn23 * ddt_scale)), multiplicity * ((var_qgs_dn25 * ddt_scale)), multiplicity * ((var_qgs_dn26 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq146_e1688: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 136, var_qgd);
        let eq146_e1691: f64 = (p.p355 * (nv8 - nv5));
        let eq146_e1692: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 137, eq146_e1691);
        let eq146_e1693: f64 = (eq146_e1688 + eq146_e1692);
        let eq146_e1693_d_n5: f64 = ((var_qgd_dn5 * ddt_scale) + ((-p.p355) * ddt_scale));
        let eq146_e1693_d_n8: f64 = ((var_qgd_dn8 * ddt_scale) + (p.p355 * ddt_scale));
        let eq146_value: f64 = eq146_e1693;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(5),
            multiplicity * (eq146_value),
            [4, 5, 8, 9, 22, 23, 25, 26],
            [multiplicity * ((var_qgd_dn4 * ddt_scale)), multiplicity * (eq146_e1693_d_n5), multiplicity * (eq146_e1693_d_n8), multiplicity * ((var_qgd_dn9 * ddt_scale)), multiplicity * ((var_qgd_dn22 * ddt_scale)), multiplicity * ((var_qgd_dn23 * ddt_scale)), multiplicity * ((var_qgd_dn25 * ddt_scale)), multiplicity * ((var_qgd_dn26 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let (eq147_e1701, eq147_e1701_d_n4, eq147_e1701_d_n8, eq147_e1701_d_n13,) = {
    if (var_guard417 != 0.0) {
        let eq147_e1698: f64 = (var_gmin * (nv8 - nv13));
        let eq147_e1699: f64 = (var_igsi + eq147_e1698);
        let eq147_e1699_d_n8: f64 = (var_igsi_dn8 + var_gmin);
        let eq147_e1699_d_n13: f64 = (var_igsi_dn13 + (-var_gmin));
        (eq147_e1699, var_igsi_dn4, eq147_e1699_d_n8, eq147_e1699_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq147_value: f64 = eq147_e1701;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(13),
            multiplicity * (eq147_value),
            4,
            multiplicity * (eq147_e1701_d_n4),
            8,
            multiplicity * (eq147_e1701_d_n8),
            13,
            multiplicity * (eq147_e1701_d_n13),
        );
        let (eq148_e1709, eq148_e1709_d_n4, eq148_e1709_d_n8, eq148_e1709_d_n17,) = {
    if (var_guard417 != 0.0) {
        let eq148_e1706: f64 = (var_gmin * (nv8 - nv17));
        let eq148_e1707: f64 = (var_igdi + eq148_e1706);
        let eq148_e1707_d_n8: f64 = (var_igdi_dn8 + var_gmin);
        let eq148_e1707_d_n17: f64 = (var_igdi_dn17 + (-var_gmin));
        (eq148_e1707, var_igdi_dn4, eq148_e1707_d_n8, eq148_e1707_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq148_value: f64 = eq148_e1709;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(17),
            multiplicity * (eq148_value),
            4,
            multiplicity * (eq148_e1709_d_n4),
            8,
            multiplicity * (eq148_e1709_d_n8),
            17,
            multiplicity * (eq148_e1709_d_n17),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_gmin: f64,
        var_guard417: f64,
        var_guard428: f64,
        var_guard439: f64,
        var_guard450: f64,
        var_guard461: f64,
        var_guard467: f64,
        var_guard480: f64,
        var_guard492: f64,
        var_guard493: f64,
        var_guard494: f64,
        var_guard523: f64,
        var_idsch: f64,
        var_idsch2: f64,
        var_idsch2_dn4: f64,
        var_idsch2_dn7: f64,
        var_idsch2_dn8: f64,
        var_idsch_dn4: f64,
        var_idsch_dn7: f64,
        var_idsch_dn8: f64,
        var_igdcbd: f64,
        var_igdcbd_dn0: f64,
        var_igdcbd_dn18: f64,
        var_igdcbd_dn19: f64,
        var_igdcbd_dn2: f64,
        var_igdcbd_dn4: f64,
        var_igdcbd_dn8: f64,
        var_igdi2: f64,
        var_igdi2_dn17: f64,
        var_igdi2_dn4: f64,
        var_igdi2_dn8: f64,
        var_igdi2db: f64,
        var_igdi2db_dn4: f64,
        var_igdi2db_dn5: f64,
        var_igdi2db_dn8: f64,
        var_igdidb: f64,
        var_igdidb_dn4: f64,
        var_igdidb_dn5: f64,
        var_igdidb_dn8: f64,
        var_igscbd: f64,
        var_igscbd_dn0: f64,
        var_igscbd_dn18: f64,
        var_igscbd_dn19: f64,
        var_igscbd_dn2: f64,
        var_igscbd_dn4: f64,
        var_igscbd_dn8: f64,
        var_igsi2: f64,
        var_igsi2_dn13: f64,
        var_igsi2_dn4: f64,
        var_igsi2_dn8: f64,
        var_igsi2db: f64,
        var_igsi2db_dn4: f64,
        var_igsi2db_dn8: f64,
        var_igsi2db_dn9: f64,
        var_igsidb: f64,
        var_igsidb_dn4: f64,
        var_igsidb_dn8: f64,
        var_igsidb_dn9: f64,
        var_pdiss: f64,
        var_pdiss_dn0: f64,
        var_pdiss_dn10: f64,
        var_pdiss_dn11: f64,
        var_pdiss_dn12: f64,
        var_pdiss_dn13: f64,
        var_pdiss_dn14: f64,
        var_pdiss_dn15: f64,
        var_pdiss_dn16: f64,
        var_pdiss_dn17: f64,
        var_pdiss_dn18: f64,
        var_pdiss_dn19: f64,
        var_pdiss_dn2: f64,
        var_pdiss_dn20: f64,
        var_pdiss_dn22: f64,
        var_pdiss_dn23: f64,
        var_pdiss_dn25: f64,
        var_pdiss_dn26: f64,
        var_pdiss_dn3: f64,
        var_pdiss_dn4: f64,
        var_pdiss_dn5: f64,
        var_pdiss_dn7: f64,
        var_pdiss_dn8: f64,
        var_pdiss_dn9: f64,
        var_qofd: f64,
        var_qofd_dn0: f64,
        var_qofd_dn4: f64,
        var_qofd_dn6: f64,
        var_qofds: f64,
        var_qofds_dn0: f64,
        var_qofds_dn2: f64,
        var_qofds_dn4: f64,
        var_qofdsub: f64,
        var_qofdsub_dn0: f64,
        var_qofdsub_dn3: f64,
        var_qofdsub_dn4: f64,
        var_qofgsub: f64,
        var_qofgsub_dn3: f64,
        var_qofgsub_dn4: f64,
        var_qofgsub_dn6: f64,
        var_qofs: f64,
        var_qofs_dn2: f64,
        var_qofs_dn4: f64,
        var_qofs_dn6: f64,
        var_qofssub: f64,
        var_qofssub_dn2: f64,
        var_qofssub_dn3: f64,
        var_qofssub_dn4: f64,
        var_qsch: f64,
        var_qsch_dn7: f64,
        var_qsch_dn8: f64,
        var_rdi: f64,
        var_rdi_dn4: f64,
        var_rsi: f64,
        var_rsi_dn4: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let nv19 = ctx.node_voltage(nodes[19]);
        let (eq149_e1719, eq149_e1719_d_n4, eq149_e1719_d_n8, eq149_e1719_d_n13,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let eq149_e1716: f64 = (var_gmin * (nv8 - nv13));
        let eq149_e1717: f64 = (var_igsi2 + eq149_e1716);
        let eq149_e1717_d_n8: f64 = (var_igsi2_dn8 + var_gmin);
        let eq149_e1717_d_n13: f64 = (var_igsi2_dn13 + (-var_gmin));
        (eq149_e1717, var_igsi2_dn4, eq149_e1717_d_n8, eq149_e1717_d_n13,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq149_value: f64 = eq149_e1719;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(13),
            multiplicity * (eq149_value),
            4,
            multiplicity * (eq149_e1719_d_n4),
            8,
            multiplicity * (eq149_e1719_d_n8),
            13,
            multiplicity * (eq149_e1719_d_n13),
        );
        let (eq150_e1729, eq150_e1729_d_n4, eq150_e1729_d_n8, eq150_e1729_d_n17,) = {
    if ((var_guard417 != 0.0) && (var_guard428 != 0.0)) {
        let eq150_e1726: f64 = (var_gmin * (nv8 - nv17));
        let eq150_e1727: f64 = (var_igdi2 + eq150_e1726);
        let eq150_e1727_d_n8: f64 = (var_igdi2_dn8 + var_gmin);
        let eq150_e1727_d_n17: f64 = (var_igdi2_dn17 + (-var_gmin));
        (eq150_e1727, var_igdi2_dn4, eq150_e1727_d_n8, eq150_e1727_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq150_value: f64 = eq150_e1729;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(17),
            multiplicity * (eq150_value),
            4,
            multiplicity * (eq150_e1729_d_n4),
            8,
            multiplicity * (eq150_e1729_d_n8),
            17,
            multiplicity * (eq150_e1729_d_n17),
        );
        let (eq151_e1739, eq151_e1739_d_n4, eq151_e1739_d_n8, eq151_e1739_d_n9,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let eq151_e1736: f64 = (var_gmin * (nv8 - nv9));
        let eq151_e1737: f64 = (var_igsidb + eq151_e1736);
        let eq151_e1737_d_n8: f64 = (var_igsidb_dn8 + var_gmin);
        let eq151_e1737_d_n9: f64 = (var_igsidb_dn9 + (-var_gmin));
        (eq151_e1737, var_igsidb_dn4, eq151_e1737_d_n8, eq151_e1737_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq151_value: f64 = eq151_e1739;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(9),
            multiplicity * (eq151_value),
            4,
            multiplicity * (eq151_e1739_d_n4),
            8,
            multiplicity * (eq151_e1739_d_n8),
            9,
            multiplicity * (eq151_e1739_d_n9),
        );
        let (eq152_e1749, eq152_e1749_d_n4, eq152_e1749_d_n5, eq152_e1749_d_n8,) = {
    if ((var_guard417 != 0.0) && (var_guard439 != 0.0)) {
        let eq152_e1746: f64 = (var_gmin * (nv8 - nv5));
        let eq152_e1747: f64 = (var_igdidb + eq152_e1746);
        let eq152_e1747_d_n5: f64 = (var_igdidb_dn5 + (-var_gmin));
        let eq152_e1747_d_n8: f64 = (var_igdidb_dn8 + var_gmin);
        (eq152_e1747, var_igdidb_dn4, eq152_e1747_d_n5, eq152_e1747_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq152_value: f64 = eq152_e1749;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (eq152_value),
            4,
            multiplicity * (eq152_e1749_d_n4),
            5,
            multiplicity * (eq152_e1749_d_n5),
            8,
            multiplicity * (eq152_e1749_d_n8),
        );
        let (eq153_e1761, eq153_e1761_d_n4, eq153_e1761_d_n8, eq153_e1761_d_n9,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let eq153_e1758: f64 = (var_gmin * (nv8 - nv9));
        let eq153_e1759: f64 = (var_igsi2db + eq153_e1758);
        let eq153_e1759_d_n8: f64 = (var_igsi2db_dn8 + var_gmin);
        let eq153_e1759_d_n9: f64 = (var_igsi2db_dn9 + (-var_gmin));
        (eq153_e1759, var_igsi2db_dn4, eq153_e1759_d_n8, eq153_e1759_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq153_value: f64 = eq153_e1761;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(9),
            multiplicity * (eq153_value),
            4,
            multiplicity * (eq153_e1761_d_n4),
            8,
            multiplicity * (eq153_e1761_d_n8),
            9,
            multiplicity * (eq153_e1761_d_n9),
        );
        let (eq154_e1773, eq154_e1773_d_n4, eq154_e1773_d_n5, eq154_e1773_d_n8,) = {
    if (((var_guard417 != 0.0) && (var_guard439 != 0.0)) && (var_guard450 != 0.0)) {
        let eq154_e1770: f64 = (var_gmin * (nv8 - nv5));
        let eq154_e1771: f64 = (var_igdi2db + eq154_e1770);
        let eq154_e1771_d_n5: f64 = (var_igdi2db_dn5 + (-var_gmin));
        let eq154_e1771_d_n8: f64 = (var_igdi2db_dn8 + var_gmin);
        (eq154_e1771, var_igdi2db_dn4, eq154_e1771_d_n5, eq154_e1771_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq154_value: f64 = eq154_e1773;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(5),
            multiplicity * (eq154_value),
            4,
            multiplicity * (eq154_e1773_d_n4),
            5,
            multiplicity * (eq154_e1773_d_n5),
            8,
            multiplicity * (eq154_e1773_d_n8),
        );
        let (eq155_e1781, eq155_e1781_d_n4, eq155_e1781_d_n7, eq155_e1781_d_n8,) = {
    if (var_guard461 != 0.0) {
        let eq155_e1778: f64 = (var_gmin * (nv8 - nv7));
        let eq155_e1779: f64 = (var_idsch + eq155_e1778);
        let eq155_e1779_d_n7: f64 = (var_idsch_dn7 + (-var_gmin));
        let eq155_e1779_d_n8: f64 = (var_idsch_dn8 + var_gmin);
        (eq155_e1779, var_idsch_dn4, eq155_e1779_d_n7, eq155_e1779_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq155_value: f64 = eq155_e1781;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(7),
            multiplicity * (eq155_value),
            4,
            multiplicity * (eq155_e1781_d_n4),
            7,
            multiplicity * (eq155_e1781_d_n7),
            8,
            multiplicity * (eq155_e1781_d_n8),
        );
        let (eq156_e1791, eq156_e1791_d_n4, eq156_e1791_d_n7, eq156_e1791_d_n8,) = {
    if ((var_guard461 != 0.0) && (var_guard467 != 0.0)) {
        let eq156_e1788: f64 = (var_gmin * (nv8 - nv7));
        let eq156_e1789: f64 = (var_idsch2 + eq156_e1788);
        let eq156_e1789_d_n7: f64 = (var_idsch2_dn7 + (-var_gmin));
        let eq156_e1789_d_n8: f64 = (var_idsch2_dn8 + var_gmin);
        (eq156_e1789, var_idsch2_dn4, eq156_e1789_d_n7, eq156_e1789_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq156_value: f64 = eq156_e1791;
        stamper.stamp_current_node3_local(
            Some(8),
            Some(7),
            multiplicity * (eq156_value),
            4,
            multiplicity * (eq156_e1791_d_n4),
            7,
            multiplicity * (eq156_e1791_d_n7),
            8,
            multiplicity * (eq156_e1791_d_n8),
        );
        let (eq157_e1796, eq157_e1796_d_n7, eq157_e1796_d_n8,) = {
    if (var_guard461 != 0.0) {
        let eq157_e1794: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 138, var_qsch);
        (eq157_e1794, (var_qsch_dn7 * ddt_scale), (var_qsch_dn8 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq157_value: f64 = eq157_e1796;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(7),
            multiplicity * (eq157_value),
            7,
            multiplicity * (eq157_e1796_d_n7),
            8,
            multiplicity * (eq157_e1796_d_n8),
        );
        let (eq160_e1815, eq160_e1815_d_n0, eq160_e1815_d_n2, eq160_e1815_d_n4, eq160_e1815_d_n8, eq160_e1815_d_n18, eq160_e1815_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard492 != 0.0)) {
        (var_igscbd, var_igscbd_dn0, var_igscbd_dn2, var_igscbd_dn4, var_igscbd_dn8, var_igscbd_dn18, var_igscbd_dn19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq160_value: f64 = eq160_e1815;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(2),
            Some(0),
            multiplicity * (eq160_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq160_e1815_d_n0), multiplicity * (eq160_e1815_d_n2), multiplicity * (eq160_e1815_d_n4), multiplicity * (eq160_e1815_d_n8), multiplicity * (eq160_e1815_d_n18), multiplicity * (eq160_e1815_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq161_e1821, eq161_e1821_d_n0, eq161_e1821_d_n2, eq161_e1821_d_n4, eq161_e1821_d_n8, eq161_e1821_d_n18, eq161_e1821_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard492 != 0.0)) {
        (var_igdcbd, var_igdcbd_dn0, var_igdcbd_dn2, var_igdcbd_dn4, var_igdcbd_dn8, var_igdcbd_dn18, var_igdcbd_dn19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq161_value: f64 = eq161_e1821;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(0),
            Some(2),
            multiplicity * (eq161_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq161_e1821_d_n0), multiplicity * (eq161_e1821_d_n2), multiplicity * (eq161_e1821_d_n4), multiplicity * (eq161_e1821_d_n8), multiplicity * (eq161_e1821_d_n18), multiplicity * (eq161_e1821_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq162_e1828, eq162_e1828_d_n0, eq162_e1828_d_n2, eq162_e1828_d_n4, eq162_e1828_d_n8, eq162_e1828_d_n18, eq162_e1828_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard492 == 0.0)) {
        (var_igscbd, var_igscbd_dn0, var_igscbd_dn2, var_igscbd_dn4, var_igscbd_dn8, var_igscbd_dn18, var_igscbd_dn19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq162_value: f64 = eq162_e1828;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(19),
            Some(18),
            multiplicity * (eq162_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq162_e1828_d_n0), multiplicity * (eq162_e1828_d_n2), multiplicity * (eq162_e1828_d_n4), multiplicity * (eq162_e1828_d_n8), multiplicity * (eq162_e1828_d_n18), multiplicity * (eq162_e1828_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq163_e1835, eq163_e1835_d_n0, eq163_e1835_d_n2, eq163_e1835_d_n4, eq163_e1835_d_n8, eq163_e1835_d_n18, eq163_e1835_d_n19,) = {
    if ((var_guard480 != 0.0) && (var_guard492 == 0.0)) {
        (var_igdcbd, var_igdcbd_dn0, var_igdcbd_dn2, var_igdcbd_dn4, var_igdcbd_dn8, var_igdcbd_dn18, var_igdcbd_dn19,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq163_value: f64 = eq163_e1835;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(18),
            Some(19),
            multiplicity * (eq163_value),
            [0, 2, 4, 8, 18, 19],
            [multiplicity * (eq163_e1835_d_n0), multiplicity * (eq163_e1835_d_n2), multiplicity * (eq163_e1835_d_n4), multiplicity * (eq163_e1835_d_n8), multiplicity * (eq163_e1835_d_n18), multiplicity * (eq163_e1835_d_n19)],
            [],
            [],
            1.0,
        );
        let (eq164_e1841, eq164_e1841_d_n0, eq164_e1841_d_n4, eq164_e1841_d_n18,) = {
    if (var_guard493 != 0.0) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_rdi;
        let eq164_e1839: f64 = ((nv0 - nv18) * __rspice_inv_cse_0);
        let eq164_e1839_d_n0: f64 = (1.0 * __rspice_inv_cse_0);
        let eq164_e1839_d_n4: f64 = (-(((nv0 - nv18) * var_rdi_dn4) / (var_rdi * var_rdi)));
        let eq164_e1839_d_n18: f64 = (-1.0 / var_rdi);
        (eq164_e1839, eq164_e1839_d_n0, eq164_e1839_d_n4, eq164_e1839_d_n18,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq164_value: f64 = eq164_e1841;
        stamper.stamp_current_node3_local(
            Some(0),
            Some(18),
            multiplicity * (eq164_value),
            0,
            multiplicity * (eq164_e1841_d_n0),
            4,
            multiplicity * (eq164_e1841_d_n4),
            18,
            multiplicity * (eq164_e1841_d_n18),
        );
        let (eq166_e1852, eq166_e1852_d_n2, eq166_e1852_d_n4, eq166_e1852_d_n19,) = {
    if (var_guard494 != 0.0) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_rsi;
        let eq166_e1850: f64 = ((nv19 - nv2) * __rspice_inv_cse_1);
        let eq166_e1850_d_n2: f64 = ((-1.0) * __rspice_inv_cse_1);
        let eq166_e1850_d_n4: f64 = (-(((nv19 - nv2) * var_rsi_dn4) / (var_rsi * var_rsi)));
        let eq166_e1850_d_n19: f64 = (1.0 / var_rsi);
        (eq166_e1850, eq166_e1850_d_n2, eq166_e1850_d_n4, eq166_e1850_d_n19,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq166_value: f64 = eq166_e1852;
        stamper.stamp_current_node3_local(
            Some(19),
            Some(2),
            multiplicity * (eq166_value),
            2,
            multiplicity * (eq166_e1852_d_n2),
            4,
            multiplicity * (eq166_e1852_d_n4),
            19,
            multiplicity * (eq166_e1852_d_n19),
        );
        let eq172_e1881: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 139, var_qofs);
        let eq172_value: f64 = eq172_e1881;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(2),
            multiplicity * (eq172_value),
            2,
            multiplicity * ((var_qofs_dn2 * ddt_scale)),
            4,
            multiplicity * ((var_qofs_dn4 * ddt_scale)),
            6,
            multiplicity * ((var_qofs_dn6 * ddt_scale)),
        );
        let eq173_e1883: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 140, var_qofd);
        let eq173_value: f64 = eq173_e1883;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(0),
            multiplicity * (eq173_value),
            0,
            multiplicity * ((var_qofd_dn0 * ddt_scale)),
            4,
            multiplicity * ((var_qofd_dn4 * ddt_scale)),
            6,
            multiplicity * ((var_qofd_dn6 * ddt_scale)),
        );
        let eq174_e1885: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 141, var_qofds);
        let eq174_value: f64 = eq174_e1885;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(0),
            multiplicity * (eq174_value),
            0,
            multiplicity * ((var_qofds_dn0 * ddt_scale)),
            2,
            multiplicity * ((var_qofds_dn2 * ddt_scale)),
            4,
            multiplicity * ((var_qofds_dn4 * ddt_scale)),
        );
        let eq175_e1887: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 142, var_qofssub);
        let eq175_value: f64 = eq175_e1887;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(2),
            multiplicity * (eq175_value),
            2,
            multiplicity * ((var_qofssub_dn2 * ddt_scale)),
            3,
            multiplicity * ((var_qofssub_dn3 * ddt_scale)),
            4,
            multiplicity * ((var_qofssub_dn4 * ddt_scale)),
        );
        let eq176_e1889: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 143, var_qofdsub);
        let eq176_value: f64 = eq176_e1889;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(0),
            multiplicity * (eq176_value),
            0,
            multiplicity * ((var_qofdsub_dn0 * ddt_scale)),
            3,
            multiplicity * ((var_qofdsub_dn3 * ddt_scale)),
            4,
            multiplicity * ((var_qofdsub_dn4 * ddt_scale)),
        );
        let eq177_e1891: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 144, var_qofgsub);
        let eq177_value: f64 = eq177_e1891;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(3),
            multiplicity * (eq177_value),
            3,
            multiplicity * ((var_qofgsub_dn3 * ddt_scale)),
            4,
            multiplicity * ((var_qofgsub_dn4 * ddt_scale)),
            6,
            multiplicity * ((var_qofgsub_dn6 * ddt_scale)),
        );
        let (eq194_e2167, eq194_e2167_d_n4,) = {
    if (var_guard523 != 0.0) {
        let eq194_e2164: f64 = (p.p321 * (nv4 - 0.0));
        let eq194_e2165: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 145, eq194_e2164);
        (eq194_e2165, (p.p321 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq194_value: f64 = eq194_e2167;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq194_value),
            4,
            multiplicity * (eq194_e2167_d_n4),
        );
        let (eq195_e2172, eq195_e2172_d_n0, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n25, eq195_e2172_d_n26,) = {
    if (var_guard523 != 0.0) {
        let eq195_e2170: f64 = (-var_pdiss);
        (eq195_e2170, (-var_pdiss_dn0), (-var_pdiss_dn2), (-var_pdiss_dn3), (-var_pdiss_dn4), (-var_pdiss_dn5), (-var_pdiss_dn7), (-var_pdiss_dn8), (-var_pdiss_dn9), (-var_pdiss_dn10), (-var_pdiss_dn11), (-var_pdiss_dn12), (-var_pdiss_dn13), (-var_pdiss_dn14), (-var_pdiss_dn15), (-var_pdiss_dn16), (-var_pdiss_dn17), (-var_pdiss_dn18), (-var_pdiss_dn19), (-var_pdiss_dn20), (-var_pdiss_dn22), (-var_pdiss_dn23), (-var_pdiss_dn25), (-var_pdiss_dn26),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq195_value: f64 = eq195_e2172;
        let eq195_node_derivative_indices: [usize; 23] = [0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 25, 26];
        let eq195_node_derivatives: [f64; 23] = [eq195_e2172_d_n0, eq195_e2172_d_n2, eq195_e2172_d_n3, eq195_e2172_d_n4, eq195_e2172_d_n5, eq195_e2172_d_n7, eq195_e2172_d_n8, eq195_e2172_d_n9, eq195_e2172_d_n10, eq195_e2172_d_n11, eq195_e2172_d_n12, eq195_e2172_d_n13, eq195_e2172_d_n14, eq195_e2172_d_n15, eq195_e2172_d_n16, eq195_e2172_d_n17, eq195_e2172_d_n18, eq195_e2172_d_n19, eq195_e2172_d_n20, eq195_e2172_d_n22, eq195_e2172_d_n23, eq195_e2172_d_n25, eq195_e2172_d_n26];
        let eq195_branch_derivative_indices: [usize; 0] = [];
        let eq195_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq195_value),
            &eq195_node_derivative_indices,
            &eq195_node_derivatives,
            &eq195_branch_derivative_indices,
            &eq195_branch_derivatives,
            multiplicity,
        );
        let (eq196_e2178, eq196_e2178_d_n4,) = {
    if (var_guard523 != 0.0) {
        let __rspice_inv_cse_2: f64 = 1.0 / p.p320;
        let eq196_e2176: f64 = ((nv4 - 0.0) * __rspice_inv_cse_2);
        let eq196_e2176_d_n4: f64 = (1.0 * __rspice_inv_cse_2);
        (eq196_e2176, eq196_e2176_d_n4,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq196_value: f64 = eq196_e2178;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq196_value),
            4,
            multiplicity * (eq196_e2178_d_n4),
        );
    }
}
