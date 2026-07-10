#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        let mut t7: usize = 0;
        while {
            let t6: f64 = if ((s.b[735] && (!s.b[927])) && (s.v[168] <= s.v[58])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;assert!(t7 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[735] && (!s.b[927])) {s.store_sub(975, 354, 475);s.store_mul(974, 225, 975);s.store_exp_neg_input(327, 974);}
            s.b[1011] = (s.v[975] < (-1e-9));s.store_scalar(1011, if s.b[1011] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1011]) {s.store_mul_sqrt_mixed_ia(360, 474, A::offset(A::add(s.ad_value(327), s.ad_value(974)), (-1.0)));s.store_div_scaled_offset_numerator_indices(981, 327, (-s.v[122]), s.v[122], 360, 1.0);}
            s.b[1012] = (s.v[975] > 1e-9);s.store_scalar(1012, if s.b[1012] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1011])) && s.b[1012]) {s.store_exp(976, 974);s.store_mul_scaled_sqrt_ad_rhs(360, 474, -1.0, A::add_scaled_offset_product_rhs(A::offset(A::add(s.ad_value(327), s.ad_value(974)), (-1.0)), 1.0, s.ad_value(239), A::add(s.ad_value(976), s.ad_value(974)), (-1.0), 1.0));s.store_div_mixed_ai(981, A::add_scaled_sub_value_product(1.0, s.ad_value(327), s.v[122], s.ad_value(239), A::offset(s.ad_value(976), 1.0), s.v[122]), 360);}
            if (((s.b[735] && (!s.b[927])) && (!s.b[1011])) && (!s.b[1012])) {s.store_mul_scale_offset_indices(360, 974, 474, -1.0, 0.0);s.store_mul_scale_offset_indices(981, 225, 474, -1.0, 0.0);}
            if (s.b[735] && (!s.b[927])) {s.copy_ad(362, 369);s.store_exp_ad(979, A::mul(s.ad_value(225), A::sub(s.ad_value(352), s.ad_value(157))));s.store_scalar(977, 1.0);s.store_sqrt_ad(978, A::add_scaled_product(A::div_scaled_product(s.ad_value(362), s.ad_value(362), 1.0, A::square(s.ad_value(238)), 1.0), 1.0, s.ad_value(379), A::add_scaled_inputs3(s.ad_value(979), 1.0, s.ad_value(974), 1.0, s.ad_value(977), -1.0), 2.0));s.store_div_scaled_product3_mixed_iiai(1008, 225, 379, A::offset(s.ad_value(979), 1.0), 2.0, 978, 2.0);s.store_add_scaled_product_indices(358, 362, (-1.0), 238, 978, -1.0);s.store_mul_scale_offset_indices(980, 1008, 238, -1.0, 0.0);s.store_div_scaled_inputs2_indices(975, 353, 1.0, 352, (-1.0), 740, 1.0);s.store_mul(974, 225, 975);}
            s.b[1013] = ((-s.v[974]) >= 500.0);s.store_scalar(1013, if s.b[1013] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1013]) {s.store_scaled_offset_ad(327, A::sub_from_scalar(1.0, s.ad_value(974)), (-500.0), 1.403592217853e217);s.store_scalar(333, 1.403592217853e217);}
            if ((s.b[735] && (!s.b[927])) && (!s.b[1013])) {s.store_neg(44, 974);s.store_scalar(327, 1.0);}
            let mut t3: usize = 0;
            while {
                let t2: f64 = if (((s.b[735] && (!s.b[927])) && (!s.b[1013])) && (s.v[44] >= 60.0)) { 1.0 } else { 0.0 };
                t2 != 0.0
            } {
                t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((s.b[735] && (!s.b[927])) && (!s.b[1013])) {s.store_scale(327, 327, 1.14200738981568e26);s.store_offset(44, 44, (-60.0));}
            }
            if ((s.b[735] && (!s.b[927])) && (!s.b[1013])) {s.store_mul_exp_rhs(327, 327, 44);s.copy_ad(333, 327);}
            if (s.b[735] && (!s.b[927])) {s.store_sqrt_offset_ad(976, A::add(s.ad_value(327), s.ad_value(974)), (-1.0));}
            s.b[1014] = (s.v[975] < (-1e-9));s.store_scalar(1014, if s.b[1014] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1014]) {s.store_mul(366, 238, 976);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), 1.0, 976, 740, 2.0);s.store_neg(368, 367);}
            s.b[1015] = (s.v[975] > 1e-9);s.store_scalar(1015, if s.b[1015] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1014])) && s.b[1015]) {s.store_mul_scale_offset_indices(366, 976, 238, -1.0, 0.0);s.store_div_scaled_product3_by_product_mixed_iiaii(367, 238, 225, A::sub_from_scalar(1.0, s.ad_value(333)), -1.0, 976, 740, 2.0);s.store_neg(368, 367);}
            if (((s.b[735] && (!s.b[927])) && (!s.b[1014])) && (!s.b[1015])) {s.store_scaled_mul(366, 238, 974, (-0.7071067811865476));s.store_scaled_mul(367, 238, 225, (-0.7071067811865476));s.store_neg(368, 367);}
            s.b[1016] = ((s.v[366] > (-(-s.v[406]))) && ((-s.v[406]) >= 0.0));s.store_scalar(1016, if s.b[1016] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1016]) {s.store_add_scaled_inputs(44, 366, 1.0, 406, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 406, 406, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1017] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1017, if s.b[1017] { 1.0 } else { 0.0 });s.b[1018] = (2.0 == 1.0);s.store_scalar(1018, if s.b[1018] { 1.0 } else { 0.0 });
            if ((((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) && s.b[1018]) {s.store_scalar(55, 1.0);}
            s.b[1019] = (2.0 == 2.0);s.store_scalar(1019, if s.b[1019] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) && (!s.b[1018])) && s.b[1019]) {s.store_scalar(55, 2.0);}
            s.b[1020] = (2.0 == 4.0);s.store_scalar(1020, if s.b[1020] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) && (!s.b[1018])) && (!s.b[1019])) && s.b[1020]) {s.store_scalar(55, 3.0);}
            s.b[1021] = (2.0 == 8.0);s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
            if (((((((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) && (!s.b[1018])) && (!s.b[1019])) && (!s.b[1020])) && s.b[1021]) {s.store_scalar(55, 4.0);}
            if (((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) {s.store_scalar(54, 0.0);}
            let mut t5: usize = 0;
            while {
                let t4: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t4 != 0.0
            } {
                t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[735] && (!s.b[927])) && s.b[1016]) && s.b[1017]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
            }
            if (((s.b[735] && (!s.b[927])) && s.b[1016]) && (!s.b[1017])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[735] && (!s.b[927])) && s.b[1016]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(1007, 44, 406, -1.0, 0.0, 53);s.store_div_scaled_product3_indices(327, 406, 52, 53, -1.0, 48, 1.0);s.store_add_scaled_inputs_mixed_ai(366, A::neg(s.ad_value(406)), -1.0, 1007, 1.0);}
            if ((s.b[735] && (!s.b[927])) && s.b[1016]) {
            }
            if ((s.b[735] && (!s.b[927])) && (!s.b[1016])) {
            }
            if ((s.b[735] && (!s.b[927])) && (!s.b[1016])) {s.store_scalar(327, 1.0);}
            if (s.b[735] && (!s.b[927])) {s.store_mul(367, 367, 327);s.store_mul(368, 368, 327);}
            s.b[1022] = ((s.v[366] < ((s.v[341] - s.v[362]) + (-(s.v[341] - s.v[362])))) && ((-(s.v[341] - s.v[362])) >= 0.0));s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1022]) {s.store_sub_add_scaled_inputs4_lhs_indices(44, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 366);s.store_square(49, 44);s.store_scaled_mul_ad(50, A::sub(s.ad_value(341), s.ad_value(362)), A::sub(s.ad_value(341), s.ad_value(362)), 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
            s.b[1023] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });s.b[1024] = (2.0 == 1.0);s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
            if ((((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) && s.b[1024]) {s.store_scalar(55, 1.0);}
            s.b[1025] = (2.0 == 2.0);s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) && (!s.b[1024])) && s.b[1025]) {s.store_scalar(55, 2.0);}
            s.b[1026] = (2.0 == 4.0);s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) && (!s.b[1024])) && (!s.b[1025])) && s.b[1026]) {s.store_scalar(55, 3.0);}
            s.b[1027] = (2.0 == 8.0);s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
            if (((((((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) && (!s.b[1024])) && (!s.b[1025])) && (!s.b[1026])) && s.b[1027]) {s.store_scalar(55, 4.0);}
            if (((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) {s.store_scalar(54, 0.0);}
            let mut t1: usize = 0;
            while {
                let t0: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
                t0 != 0.0
            } {
                t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[735] && (!s.b[927])) && s.b[1022]) && s.b[1023]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
            }
            if (((s.b[735] && (!s.b[927])) && s.b[1022]) && (!s.b[1023])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
            if ((s.b[735] && (!s.b[927])) && s.b[1022]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul_ad_affine_product_lhs(1007, s.ad_value(44), A::sub(s.ad_value(341), s.ad_value(362)), -1.0, 0.0, 53);s.store_div_scaled_product3_mixed_aiii(327, A::sub(s.ad_value(341), s.ad_value(362)), 52, 53, -1.0, 48, 1.0);s.store_sub_add_scaled_inputs4_lhs_indices(366, 341, 1.0, 362, (-1.0), 341, -1.0, 362, 1.0, 1007);}
            if ((s.b[735] && (!s.b[927])) && s.b[1022]) {
            }
            if ((s.b[735] && (!s.b[927])) && (!s.b[1022])) {
            }
            if ((s.b[735] && (!s.b[927])) && (!s.b[1022])) {s.store_scalar(327, 1.0);}
            if (s.b[735] && (!s.b[927])) {s.store_mul(368, 368, 327);s.store_mul(367, 367, 327);s.store_add(359, 362, 366);}
            s.b[1028] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
            if ((s.b[735] && (!s.b[927])) && s.b[1028]) {s.copy_ad(611, 168);s.store_scalar(168, s.v[58]);}
            if ((s.b[735] && (!s.b[927])) && (!s.b[1028])) {s.store_add_scaled_inputs_product_mixed_iiia(985, 352, 1.0, 178, (-1.0), 324, A::add(A::add_scaled_inputs4(s.ad_value(360), 1.0, s.ad_value(362), 1.0, s.ad_value(358), 1.0, s.ad_value(366), 1.0), s.ad_value(393)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(986, 1.0, 324, A::add(s.ad_value(980), s.ad_value(368)), 1.0);s.store_mul_scale_offset_indices(987, 367, 324, -1.0, 0.0);s.store_mul_scale_offset_indices(988, 981, 324, -1.0, 0.0);s.store_add_scaled_product_mixed_iia(975, 352, 1.0, 737, A::add_scaled_inputs(s.ad_value(341), 0.5, s.ad_value(360), 1.0), 1.0);s.store_mul(977, 737, 981);s.store_sub(989, 353, 975);s.store_scalar(990, (-1.0));s.store_scalar(991, 1.0);s.store_neg(992, 977);s.store_add_scaled_inputs3_indices(993, 354, 1.0, 353, (-1.0), 360, (-s.v[94]));s.store_scalar(994, (-1.0));s.store_sub_from_scalar_scaled_input(995, 1.0, 981, s.v[94]);s.store_add_scaled_inputs4(996, A::mul3(s.ad_value(986), s.ad_value(991), s.ad_value(995)), 1.0, A::mul3(s.ad_value(986), s.ad_value(992), s.ad_value(994)), (-1.0), A::mul3(s.ad_value(987), s.ad_value(990), s.ad_value(995)), -1.0, A::mul3(s.ad_value(988), s.ad_value(990), s.ad_value(994)), 1.0);s.store_div_from_scalar_offset_input(997, 1.0, 996, 1e-50);s.store_add_scaled_products_indices(998, 991, 995, 1.0, 992, 994, (-1.0));s.store_add_scaled_products_indices(999, 988, 994, 1.0, 987, 995, (-1.0));s.store_add_scaled_products_indices(1000, 987, 992, 1.0, 988, 991, (-1.0));s.store_mul_scale_offset_indices(1001, 995, 990, -1.0, 0.0);s.store_mul(1002, 986, 995);s.store_add_scaled_products_indices(1003, 988, 990, 1.0, 986, 992, (-1.0));s.store_primal_mul(1004, 990, 994);s.store_mul_scale_offset_indices(1005, 994, 986, -1.0, 0.0);s.store_add_scaled_products_indices(1006, 986, 991, 1.0, 987, 990, (-1.0));s.store_mul_add_scaled_products3_indices_rhs(982, 997, 998, 985, -1.0, 999, 989, -1.0, 1000, 993, -1.0);s.store_mul_add_scaled_products3_indices_rhs(983, 997, 1001, 985, -1.0, 1002, 989, -1.0, 1003, 993, -1.0);s.store_mul_add_scaled_products3_indices_rhs(984, 997, 1004, 985, -1.0, 1005, 989, -1.0, 1006, 993, -1.0);s.store_abs(975, 982);}
            s.b[1029] = (s.v[975] < ((s.v[983]) as f64).abs());s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1028])) && s.b[1029]) {s.store_abs(975, 983);}
            s.b[1030] = (s.v[975] < ((s.v[984]) as f64).abs());s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1028])) && s.b[1030]) {s.store_abs(975, 984);}
            if ((s.b[735] && (!s.b[927])) && (!s.b[1028])) {s.store_scalar(407, 1.0);}
            s.b[1031] = (s.v[168] > 80.0);s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1028])) && s.b[1031]) {s.store_scalar(407, 125.0);}
            s.b[1032] = (s.v[168] > 40.0);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
            if ((((s.b[735] && (!s.b[927])) && (!s.b[1028])) && (!s.b[1031])) && s.b[1032]) {s.store_scalar(407, 125.0);}
            s.b[1033] = (s.v[168] > 20.0);s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
            if (((((s.b[735] && (!s.b[927])) && (!s.b[1028])) && (!s.b[1031])) && (!s.b[1032])) && s.b[1033]) {s.store_scalar(407, 25.0);}
            s.b[1034] = (s.v[168] > 10.0);s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
            if ((((((s.b[735] && (!s.b[927])) && (!s.b[1028])) && (!s.b[1031])) && (!s.b[1032])) && (!s.b[1033])) && s.b[1034]) {s.store_scalar(407, 5.0);}
            s.b[1035] = (s.v[975] > (0.1 / s.v[407]));s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1028])) && s.b[1035]) {s.store_mul_mixed_ia(982, 982, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(975), 1.0));s.store_mul_mixed_ia(983, 983, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(975), 1.0));s.store_mul_mixed_ia(984, 984, A::div_scalar_by_product(0.1, s.ad_value(407), s.ad_value(975), 1.0));}
            if ((s.b[735] && (!s.b[927])) && (!s.b[1028])) {s.store_add(352, 352, 982);s.store_add(353, 353, 983);s.store_add(354, 354, 984);s.store_primal_scale(408, 407, 5e-12);}
            s.b[1036] = (s.v[975] < s.v[408]);s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
            if (((s.b[735] && (!s.b[927])) && (!s.b[1028])) && s.b[1036]) {s.store_scalar(430, 1.0);}
            if (s.b[735] && (!s.b[927])) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[735] && (!s.b[927])) {
            if (s.v[611] > 0.0) {
                s.copy_ad(168, 611);
            } else {
            }
        }
        s.b[1037] = (s.v[430] == 0.0);s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1037]) {s.copy_ad(352, 346);s.copy_ad(353, 347);s.copy_ad(354, 348);}
        if (s.b[735] && (!s.b[927])) {s.copy_ad(162, 352);s.copy_ad(157, 453);}
        s.b[1038] = (s.v[349] < 0.0);s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1038]) {s.store_scalar(145, 1.0);}
        if (s.b[735] && (!s.b[927])) {s.copy_ad(374, 349);s.copy_ad(375, 352);s.store_sub(164, 375, 374);s.copy_ad(373, 351);s.store_scale(400, 401, 9662367879.197212);s.store_add_scaled_inputs3_mixed_iia(246, 358, 1.0, 355, (-1.0), A::mul3_scaled_output(s.ad_value(225), A::add(s.ad_value(358), s.ad_value(355)), A::sub(s.ad_value(375), s.ad_value(374)), 0.5), -1.0);}
        s.b[1039] = ((s.v[246] < 0.0) || (s.v[157] == 0.0));s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1039]) {s.store_scalar(246, 0.0);}
        if (s.b[735] && (!s.b[927])) {s.store_scaled_add(437, 359, 356, (-0.5));s.store_sub(411, 352, 349);s.store_offset(411, 411, 5e-12);s.store_div_from_scalar_offset_scaled_input(410, s.v[93], 400, s.v[93], 1.0);s.store_div_scaled_inputs2_mixed_aai(409, A::square(s.ad_value(360)), 1.0, A::square(s.ad_value(357)), (-1.0), 410, 1.0);}
        s.b[1040] = (((-s.v[409]) < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1040]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 409, -1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1041] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });s.b[1042] = (2.0 == 1.0);s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && s.b[1042]) {s.store_scalar(55, 1.0);}
        s.b[1043] = (2.0 == 2.0);s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && s.b[1043]) {s.store_scalar(55, 2.0);}
        s.b[1044] = (2.0 == 4.0);s.store_scalar(1044, if s.b[1044] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && (!s.b[1043])) && s.b[1044]) {s.store_scalar(55, 3.0);}
        s.b[1045] = (2.0 == 8.0);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (!s.b[1042])) && (!s.b[1043])) && (!s.b[1044])) && s.b[1045]) {s.store_scalar(55, 4.0);}
        if (((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) {s.store_scalar(54, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1040]) && s.b[1041]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((s.b[735] && (!s.b[927])) && s.b[1040]) && (!s.b[1041])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[735] && (!s.b[927])) && s.b[1040]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(328, 341, 1e-5, 43, 1.0);}
        if ((s.b[735] && (!s.b[927])) && (!s.b[1040])) {s.store_neg(328, 409);}
        if (s.b[735] && (!s.b[927])) {s.store_neg(409, 328);}
        s.b[1046] = (((s.v[225] * s.v[373]) - 1.0) > 0.0);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1046]) {s.store_sqrt_offset_ad(328, A::mul(s.ad_value(225), s.ad_value(373)), (-1.0));}
        if (s.b[735] && (!s.b[927])) {s.store_sub(414, 355, 358);}
        s.b[1047] = ((s.v[414] < (s.v[341] * 1e-5)) && ((s.v[341] * 1e-5) >= 0.0));s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1047]) {s.store_sub_scaled_inputs(44, 341, 1e-5, 414, 1.0);s.store_square(49, 44);s.store_scaled_mul(50, 341, 341, (1e-5 * 1e-5));s.store_scalar(51, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[735] && (!s.b[927])) && s.b[1047]) {s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1048] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });s.b[1049] = (2.0 == 1.0);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && s.b[1049]) {s.store_scalar(55, 1.0);}
        s.b[1050] = (2.0 == 2.0);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && s.b[1050]) {s.store_scalar(55, 2.0);}
        s.b[1051] = (2.0 == 4.0);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && (!s.b[1050])) && s.b[1051]) {s.store_scalar(55, 3.0);}
        s.b[1052] = (2.0 == 8.0);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (!s.b[1049])) && (!s.b[1050])) && (!s.b[1051])) && s.b[1052]) {s.store_scalar(55, 4.0);}
        if (((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) {s.store_scalar(54, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1047]) && s.b[1048]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((s.b[735] && (!s.b[927])) && s.b[1047]) && (!s.b[1048])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[735] && (!s.b[927])) && s.b[1047]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_affine_lhs(43, 44, 341, 1e-5, 0.0, 53);s.store_sub_scaled_inputs(414, 341, 1e-5, 43, 1.0);}
        if ((s.b[735] && (!s.b[927])) && (!s.b[1047])) {
        }
        if (s.b[735] && (!s.b[927])) {s.store_offset_div_scaled_inputs_mixed_ia(412, 414, (-2.0), A::mul(A::mul3(s.ad_value(225), s.ad_value(323), s.ad_value(411)), s.ad_value(411)), 1.0, 1.0);s.store_mul_ad_product_lhs_mixed_ai(328, A::square(s.ad_value(411)), 411, 411);s.store_mul(415, 412, 411);s.store_sub_from_scalar_div_indices(413, 1.0, 415, 192);}
        s.b[1053] = ((s.v[413] < 1e-5) && (1e-5 >= 0.0));s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if ((s.b[735] && (!s.b[927])) && s.b[1053]) {s.store_sub_from_scalar(44, 1e-5, 413);s.store_square(49, 44);s.store_scalar(50, (1e-5 * 1e-5));s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1054] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });s.b[1055] = (2.0 == 1.0);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if ((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && s.b[1055]) {s.store_scalar(55, 1.0);}
        s.b[1056] = (2.0 == 2.0);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && s.b[1056]) {s.store_scalar(55, 2.0);}
        s.b[1057] = (2.0 == 4.0);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if ((((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && (!s.b[1056])) && s.b[1057]) {s.store_scalar(55, 3.0);}
        s.b[1058] = (2.0 == 8.0);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (((((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (!s.b[1055])) && (!s.b[1056])) && (!s.b[1057])) && s.b[1058]) {s.store_scalar(55, 4.0);}
        if (((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) {s.store_scalar(54, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if ((((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[735] && (!s.b[927])) && s.b[1053]) && s.b[1054]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((s.b[735] && (!s.b[927])) && s.b[1053]) && (!s.b[1054])) {s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));}
        if ((s.b[735] && (!s.b[927])) && s.b[1053]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(43, 44, 53, 1e-5);s.store_sub_from_scalar(413, 1e-5, 43);}
        if ((s.b[735] && (!s.b[927])) && (!s.b[1053])) {
        }
        if (s.b[735] && (!s.b[927])) {s.copy_ad(190, 413);s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);}
        if (s.b[735] && (!s.b[927])) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[735] && (!s.b[927])) {s.store_scaled_add(436, 355, 358, (-0.5));}
        if (!s.b[735]) {s.copy_ad(515, 154);}
        s.b[1065] = (s.v[416] < p.p237);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if ((!s.b[735]) && s.b[1065]) {s.store_scalar(339, 1.0);}
        if ((!s.b[735]) && (!s.b[1065])) {s.store_scalar(339, 2.0);}
        if (!s.b[735]) {s.store_add_scaled_inputs3_offset_indices(160, 185, (-1.0), 320, 1.0, 515, 1.0, s.v[123]);}
        s.b[1066] = (s.v[158] < s.v[160]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if ((!s.b[735]) && s.b[1066]) {s.store_scalar(338, (-1.0));s.store_mul_scaled_ln_ad_rhs(254, 227, 2.0, A::div_from_scalar((-s.v[139]), s.ad_value(240)));s.store_mul_sub_rhs(336, 225, 159, 515);s.store_div_scalar_by_product_indices(328, 1.0, 225, 238, 1.0);s.store_mul(337, 328, 323);s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(260, 262, 262, 8.0, 0.0, 262);s.store_offset(331, 336, (-2.0));s.store_scaled_mul(332, 337, 331, 9.0);s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);s.store_square(259, 261);}
        s.b[1067] = (s.v[260] < (s.v[259] * 1e-8));s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1066]) && s.b[1067]) {s.store_add_scaled_inputs3_offset_mixed_iai(257, 261, 1.0, A::div_scaled_inputs(s.ad_value(260), 0.5, s.ad_value(261), 1.0), 1.0, 332, 1.0, ((-7.0) * 1.414213562373095));}
        if (((!s.b[735]) && s.b[1066]) && (!s.b[1067])) {s.store_sqrt_add(258, 260, 259);s.store_add_offset_lhs(257, 258, ((-7.0) * 1.414213562373095), 332);}
        if ((!s.b[735]) && s.b[1066]) {s.store_powf(256, 257, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(255, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), 1.0, 256, 2.0, 256, 256, 1.414213562373095);s.store_div_from_scalar(328, 1.0, 256);s.store_mul(181, 255, 328);s.store_add_scaled_product_indices(313, 515, 1.0, 181, 227, 1.0);s.store_sub(328, 313, 515);s.store_div(329, 328, 254);s.store_sqrt_square_offset(330, 329, 1.0);s.store_add_div_lhs_indices(161, 328, 330, 515);}
        s.b[1068] = (s.v[144] >= 1.0);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && (!s.b[1066])) && s.b[1068]) {s.store_scalar(349, s.v[619]);s.store_scalar(378, s.v[619]);}
        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(159), s.ad_value(515))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {s.store_add_product3_rhs_mixed_iia(376, 159, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);s.store_mul_sub_rhs(181, 225, 376, 515);}
        s.b[1069] = (s.v[181] < 3.0);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && s.b[1069]) {s.store_mul_sub_rhs(337, 225, 159, 515);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 515, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1070] = (s.v[158] <= s.v[182]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && s.b[1070]) {s.copy_ad(378, 376);}
        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {s.store_div_scalar_by_product_indices(328, 1.0, 379, 434, 1.0);s.store_mul3_lhs(329, 328, 159, 159);s.store_add_div_from_scalar_rhs(330, 225, 2.0, 159);s.store_div_ln_lhs(377, 329, 330);s.store_offset_sub(44, 377, 376, (-0.0008));s.store_scale(45, 377, (4.0 * 0.0008));}
        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if (((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && (!s.b[1069])) && (!s.b[1070])) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(378, 377, 1.0, 44, (-0.5), 45, (-0.5));}
        if (((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) {s.store_offset(336, 515, (5e-12 / 2.0));}
        s.b[1071] = (s.v[378] < s.v[336]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && (!s.b[1066])) && (!s.b[1068])) && s.b[1071]) {s.copy_ad(378, 336);}
        if ((!s.b[735]) && (!s.b[1066])) {s.copy_ad(161, 378);s.copy_ad(163, 376);}
        s.b[1072] = ((p.p25 == 1.0) && (p.p26 == 2.0));s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if ((!s.b[735]) && s.b[1072]) {s.store_scaled_voltage(393, ctx, nodes, Some(17), None, (1e-9 / 0.0001));}
        if ((!s.b[735]) && (!s.b[1072])) {s.store_scalar(393, 0.0);}
        if (!s.b[735]) {s.store_exp_mul(486, 225, 515);s.store_mul(487, 379, 486);s.store_scalar(430, 0.0);s.copy_ad(349, 161);s.store_scale(419, 229, ((p.p237 * (p.p237 * 0.5)) * 9662367879.197212));s.store_sqrt_mul_scaled_lhs(327, 225, 2.0, 419);s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp_scaled_input(s.ad_value(327), -1.0), 0.5);s.store_div_ln_lhs(420, 328, 419);s.store_scalar(167, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t10: usize = 0;
        while {
            let te: f64 = (s.v[57] + 1.0);let tf: f64 = if ((!s.b[735]) && (s.v[167] <= te)) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[735]) {s.store_sub(417, 349, 515);s.store_mul(181, 225, 417);s.store_mul_sub_rhs(337, 420, 417, 419);}
            s.b[1073] = (s.v[337] < 80.0);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1073]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if ((!s.b[735]) && (!s.b[1073])) {s.store_sub(422, 417, 419);s.store_scalar(423, 1.0);}
            if (!s.b[735]) {s.store_mul(421, 225, 422);}
            s.b[1074] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1074]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1075] = (s.v[181] < 0.0);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1074]) && s.b[1075]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1076] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && (!s.b[1074])) && s.b[1076]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if (((!s.b[735]) && (!s.b[1074])) && (!s.b[1076])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1077] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1077]) {s.store_scalar(338, (-1.0));}
            s.b[1078] = (s.v[338] == (-1.0));s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1078]) {s.store_scalar(401, 0.0);}
            if ((!s.b[735]) && (!s.b[1078])) {s.store_mul(401, 444, 242);}
            s.b[1079] = (s.v[401] < (p.p237 * 1.01));s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1079]) {s.store_scalar(339, 1.0);}
            if ((!s.b[735]) && (!s.b[1079])) {s.store_scalar(339, 2.0);}
            if (!s.b[735]) {s.store_mul(370, 229, 401);}
            s.b[1080] = (s.v[181] < 0.0);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1080]) {s.store_neg(490, 242);s.store_neg(491, 443);}
            s.b[1081] = (s.v[181] < 1e-7);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && (!s.b[1080])) && s.b[1081]) {s.copy_ad(490, 242);s.copy_ad(491, 443);}
            s.b[1082] = (s.v[181] < 80.0);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) && s.b[1082]) {s.store_exp(243, 181);s.store_mul_sub_mixed_iia(488, 487, 243, A::offset(s.ad_value(181), 1.0));s.store_mul_ad_product_rhs_mixed_ia(489, 487, 225, A::offset(s.ad_value(243), (-1.0)));}
            if ((((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) && (!s.b[1082])) {s.store_exp_mul(485, 225, 349);s.store_mul_mixed_ia(488, 379, A::add_scaled_offset_product_rhs(s.ad_value(485), 1.0, s.ad_value(486), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(489, 379, 225, A::sub(s.ad_value(485), s.ad_value(486)));}
            if (((!s.b[735]) && (!s.b[1080])) && (!s.b[1081])) {s.store_sqrt_square_add(490, 242, 488);s.store_div_scaled_add_product_indices(491, 489, 0.5, 443, 242, (2.0 * 0.5), 490, 1.0);}
            if (!s.b[735]) {s.store_add_scaled_inputs_products_indices(492, 349, 1.0, 159, (-1.0), 240, 490, 1.0, 324, 393, (-1.0));s.store_offset_mul(493, 240, 491, 1.0);}
            s.b[1083] = (s.v[430] == 1.0);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
            if ((!s.b[735]) && s.b[1083]) {s.store_scalar(167, (s.v[57] + 1.0));}
            if ((!s.b[735]) && (!s.b[1083])) {s.store_div_scaled_inputs_indices(494, 492, -1.0, 493, 1.0);}
            if ((!s.b[735]) && (!s.b[1083])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[349]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(349))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1084] = (((s.v[494]) as f64).abs() > s.v[496]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && (!s.b[1083])) && s.b[1084]) {s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((!s.b[735]) && (!s.b[1083])) {s.store_add(349, 349, 494);}
            s.b[1085] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && (!s.b[1083])) && s.b[1085]) {s.store_scalar(430, 1.0);}
            if (!s.b[735]) {s.store_primal_offset(167, 167, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[735]) {s.store_primal_offset(167, 167, (-1.0));s.copy_ad(371, 370);s.copy_ad(356, 371);s.copy_ad(161, 349);s.store_div(568, 371, 238);s.store_offset_square(169, 568, (10.0 * 2.220446049250313e-16));s.store_scale(328, 568, 2.0);s.store_offset(170, 568, (10.0 * 2.220446049250313e-16));s.store_mul(245, 238, 170);s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(490), s.ad_value(170));s.store_mul3_lhs(244, 238, 488, 328);s.store_neg(355, 244);s.store_mul(192, 244, 324);}
        s.b[1086] = ((s.v[338] == (-1.0)) || (s.v[192] <= 1e-12));s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if ((!s.b[735]) && s.b[1086]) {s.store_scalar(338, 4.0);s.store_scalar(145, 1.0);s.store_sub(329, 159, 161);s.store_mul(437, 323, 329);s.store_scale(327, 108, (-s.v[98]));s.store_mul(196, 327, 437);s.store_scalar(197, 0.0);s.store_scalar(198, 0.0);s.store_mul_scale_offset_indices(329, 437, 534, -1.0, 0.0);s.store_scale(468, 329, s.v[438]);s.store_sub(467, 329, 468);s.store_scalar(470, 0.0);s.store_scalar(469, 0.0);s.store_scalar(199, 0.0);s.store_scalar(192, 0.0);s.store_scalar(145, 1.0);s.copy_ad(352, 349);s.copy_ad(162, 161);s.copy_ad(314, 162);s.store_scalar(612, 1.0);}
        s.b[1087] = (s.v[612] == 0.0);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if ((!s.b[735]) && s.b[1087]) {s.copy_ad(453, 157);s.store_scalar(1094, 1e-50);s.store_div_square_rhs(1089, 545, 323);s.store_offset_mul_ad(1091, A::div_from_scalar(2.0, s.ad_value(1089)), A::sub(s.ad_value(159), s.ad_value(1094)), 1.0);s.store_offset_div_from_scalar_ad(332, 2.0, s.ad_value(1089), 1.0);}
        s.b[1095] = ((s.v[1091] < s.v[332]) && (s.v[332] >= 0.0));s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1087]) && s.b[1095]) {s.store_sub(44, 332, 1091);s.store_square(49, 44);s.store_square(50, 332);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1096] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });s.b[1097] = (4.0 == 1.0);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && s.b[1097]) {s.store_scalar(55, 1.0);}
        s.b[1098] = (4.0 == 2.0);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if ((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && s.b[1098]) {s.store_scalar(55, 2.0);}
        s.b[1099] = (4.0 == 4.0);s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if (((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && (!s.b[1098])) && s.b[1099]) {s.store_scalar(55, 3.0);}
        s.b[1100] = (4.0 == 8.0);s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((((((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (!s.b[1097])) && (!s.b[1098])) && (!s.b[1099])) && s.b[1100]) {s.store_scalar(55, 4.0);}
        if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) {s.store_scalar(54, 0.0);}
        let mut t12: usize = 0;
        while {
            let t11: f64 = if (((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;assert!(t12 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && s.b[1096]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((((!s.b[735]) && s.b[1087]) && s.b[1095]) && (!s.b[1096])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (((!s.b[735]) && s.b[1087]) && s.b[1095]) {s.store_div_from_scalar(53, 1.0, 53);s.store_mul3_lhs(43, 44, 332, 53);s.store_sub(1091, 332, 43);}
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1095])) {
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[735]) && s.b[1087]) {s.store_sqrt(1090, 1091);s.store_add_mul_sub_from_scalar_rhs_indices(1094, 159, 1089, 1.0, 1090);s.store_sqrt_square_offset(44, 1094, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1094, 1094, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1101] = (s.v[1094] < 0.0);s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1087]) && s.b[1101]) {s.store_scalar(1094, 0.0);}
        if ((!s.b[735]) && s.b[1087]) {s.store_div(1088, 157, 1094);s.store_pow_offset_rhs(1089, 1088, 138, (-1.0));s.store_mul(1093, 1089, 1088);s.store_offset(1090, 1093, 1.0);s.store_pow_ad(1091, s.ad_value(1090), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));s.store_mul(1092, 1091, 1090);s.store_div(452, 157, 1092);s.copy_ad(157, 452);s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));}
        s.b[1102] = (s.v[157] <= 0.0);s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1087]) && s.b[1102]) {s.store_scalar(164, 0.0);s.copy_ad(162, 161);s.store_scalar(430, 0.0);}
        s.b[1103] = (s.v[144] >= 1.0);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1103]) {s.store_scalar(352, s.v[622]);s.store_sub_from_scalar(165, s.v[622], 161);}
        s.b[1104] = (s.v[144] == 0.0);s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if ((s.v[163] - s.v[161]) >= 0.0) {
                s.store_sub(166, 163, 161);
            } else {
                s.store_scalar(166, 0.0);
            }
        }
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {s.store_offset_sub_scaled_inputs_indices(44, 166, (1.0 + 0.3), 157, 1.0, (-0.03));s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));}
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if (s.v[45] > 0.0) {
            } else {
                s.store_neg(45, 45);
            }
        }
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {s.store_sqrt_square_add(45, 44, 45);s.store_add_scaled_inputs3_indices(165, 166, (1.0 + 0.3), 44, (-0.5), 45, (-0.5));}
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1104]) {
            if (s.v[165] <= s.v[166]) {
            } else {
                s.copy_ad(165, 166);
            }
        }
        s.b[1105] = (s.v[165] < 0.0);s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && s.b[1105]) {s.store_scalar(165, 0.0);}
        s.b[1106] = (s.v[165] > s.v[157]);s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if (((((!s.b[735]) && s.b[1087]) && (!s.b[1102])) && (!s.b[1105])) && s.b[1106]) {s.copy_ad(165, 157);}
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1102])) {s.copy_ad(164, 165);s.store_add(162, 161, 164);s.store_scalar(430, 0.0);}
        if ((!s.b[735]) && s.b[1087]) {s.copy_ad(352, 162);s.store_scalar(168, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
    ) {
        let mut t15: usize = 0;
        while {
            let t13: f64 = (s.v[58] + 1.0);let t14: f64 = if (((!s.b[735]) && s.b[1087]) && (s.v[168] <= t13)) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;assert!(t15 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[735]) && s.b[1087]) {s.store_sub(418, 352, 515);s.store_mul(181, 225, 418);s.store_mul_sub_rhs(337, 420, 418, 419);}
            s.b[1107] = (s.v[337] < 80.0);s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1087]) && s.b[1107]) {s.store_exp(328, 337);s.store_exp_mul_scaled_lhs_indices(327, 420, -1.0, 419);s.store_sub(329, 328, 327);s.store_div_ln_offset_lhs(422, 329, 1.0, 420);s.store_div_scaled_value_offset_denominator(423, s.ad_value(328), 1.0, s.ad_value(329), 1.0, 1.0);}
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1107])) {s.store_sub(422, 418, 419);s.store_scalar(423, 1.0);}
            if ((!s.b[735]) && s.b[1087]) {s.store_mul(421, 225, 422);}
            s.b[1108] = (((s.v[181]) as f64).abs() < 1e-16);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1087]) && s.b[1108]) {s.store_sqrt_scaled_input_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 1.0 / (2.0));s.store_mul(242, 181, 327);s.store_mul(443, 225, 327);}
            s.b[1109] = (s.v[181] < 0.0);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && s.b[1087]) && s.b[1108]) && s.b[1109]) {s.store_neg(242, 242);s.store_neg(443, 443);}
            s.b[1110] = (((s.v[181]) as f64).abs() < 0.005);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1108])) && s.b[1110]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(327, 181, 1.0, 181, 1.0, 181, 1.0, 181, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(328, 181, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(181), 1.0, A::scale(s.ad_value(181), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(329, 421, 1.0, 421, 1.0, 421, 1.0, 421, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(330, 421, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(421), 1.0, A::scale(s.ad_value(421), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(242, 327, 329);s.store_div_scaled_product_mixed_iai(443, 225, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(423), s.ad_value(330), (-1.0)), 0.5, 242, 1.0);}
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1108])) && (!s.b[1110])) {s.store_exp_neg_input(327, 181);s.store_exp_neg_input(328, 421);s.store_sqrt_ad(242, A::add_scaled_inputs4(s.ad_value(181), 1.0, s.ad_value(421), (-1.0), s.ad_value(327), 1.0, s.ad_value(328), (-1.0)));s.store_div_scaled_product_mixed_iai(443, 225, A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul_sub_from_scalar_rhs(s.ad_value(423), 1.0, s.ad_value(328))), 0.5, 242, 1.0);}
            s.b[1111] = (s.v[338] == (-1.0));s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1087]) && s.b[1111]) {s.store_scalar(401, 0.0);}
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1111])) {s.store_mul(401, 444, 242);}
            if ((!s.b[735]) && s.b[1087]) {s.store_mul(370, 229, 401);}
            s.b[1112] = (s.v[181] < 0.0);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1087]) && s.b[1112]) {s.store_neg(499, 242);s.store_neg(500, 443);}
            s.b[1113] = (s.v[181] < 1e-7);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1112])) && s.b[1113]) {s.copy_ad(499, 242);s.copy_ad(500, 443);}
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1112])) && (!s.b[1113])) {s.store_mul_sub_rhs(501, 225, 352, 157);s.store_exp(502, 501);s.store_mul_mixed_ia(497, 379, A::add_scaled_offset_product_rhs(s.ad_value(502), 1.0, s.ad_value(484), s.ad_value(181), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(498, 379, 225, A::sub(s.ad_value(502), s.ad_value(484)));s.store_sqrt_square_add(499, 242, 497);s.store_div_scaled_add_product_indices(500, 498, 0.5, 443, 242, (2.0 * 0.5), 499, 1.0);}
            if ((!s.b[735]) && s.b[1087]) {s.store_add_scaled_inputs_products_indices(503, 352, 1.0, 159, (-1.0), 240, 499, 1.0, 324, 393, (-1.0));s.store_offset_mul(504, 240, 500, 1.0);}
            s.b[1114] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
            if (((!s.b[735]) && s.b[1087]) && s.b[1114]) {s.store_scalar(168, (s.v[58] + 1.0));}
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {s.store_div_scaled_inputs_indices(495, 503, -1.0, 504, 1.0);}
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1115] = (((s.v[495]) as f64).abs() > s.v[496]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1114])) && s.b[1115]) {s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((!s.b[735]) && s.b[1087]) && (!s.b[1114])) {s.store_add(352, 352, 495);}
            s.b[1116] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
            if ((((!s.b[735]) && s.b[1087]) && (!s.b[1114])) && s.b[1116]) {s.store_scalar(430, 1.0);}
            if ((!s.b[735]) && s.b[1087]) {s.store_primal_offset(168, 168, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[735]) && s.b[1087]) {s.store_primal_offset(168, 168, (-1.0));s.copy_ad(372, 370);s.copy_ad(359, 372);s.copy_ad(162, 352);s.store_div(569, 372, 238);s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(499), s.ad_value(171));s.store_mul3_lhs(358, 238, 497, 328);s.store_neg(358, 358);s.store_sub(164, 162, 161);s.copy_ad(157, 453);s.store_div(328, 225, 169);s.store_mul(505, 328, 164);s.store_offset(506, 505, 1.0);s.store_sqrt(507, 506);s.store_div_from_scalar_offset_input(508, 1.0, 507, 1.0);s.store_div(509, 508, 170);s.store_scaled_add(510, 568, 569, 0.5);s.store_add_scaled_inputs4_indices(328, 159, 1.0, 227, 1.0, 161, (-(2.0 * 0.5)), 164, (-0.5));s.store_sub(329, 509, 510);s.store_mul(330, 225, 323);s.store_mul(331, 225, 238);s.store_add_scaled_products_indices(511, 330, 328, 1.0, 331, 329, 1.0);s.store_scaled_add(424, 359, 356, 0.5);s.store_scaled_add(425, 358, 355, (-0.5));s.store_sub(426, 359, 356);s.store_sub(427, 355, 358);s.store_square(428, 238);}
        s.b[1117] = (s.v[339] <= 1.0);s.store_scalar(1117, if s.b[1117] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1087]) && s.b[1117]) {s.store_add_scaled_inputs3_mixed_aia(246, A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), 1.0, 427, (-1.0), A::div_scaled_product(A::square(s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), -1.0);}
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1117])) {s.store_mul(246, 164, 511);}
        s.b[1118] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));s.store_scalar(1118, if s.b[1118] { 1.0 } else { 0.0 });
        if (((!s.b[735]) && s.b[1087]) && s.b[1118]) {s.store_scalar(246, 0.0);}
        s.b[1119] = (s.v[339] <= 1.0);s.store_scalar(1119, if s.b[1119] { 1.0 } else { 0.0 });s.b[1120] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1120, if s.b[1120] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && s.b[1119]) && s.b[1120]) {s.store_add_scaled_product_mixed_aia(437, A::div_scaled_product(A::mul3(A::add_scaled_inputs_product(s.ad_value(425), 1.0, s.ad_value(424), (-2.0), A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div_scaled_product(s.ad_value(424), s.ad_value(424), 2.0, s.ad_value(428), 1.0)), A::div_scaled_product(s.ad_value(426), s.ad_value(426), 0.1, s.ad_value(428), 1.0)), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0), 1.0, 424, A::sub(A::mul3(s.ad_value(425), s.ad_value(225), s.ad_value(164)), s.ad_value(427)), 1.0);s.store_div(437, 437, 246);}
        if ((((!s.b[735]) && s.b[1087]) && s.b[1119]) && (!s.b[1120])) {s.copy_ad(437, 424);}
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1119])) {s.store_scaled_add(437, 359, 356, 0.5);}
        if ((!s.b[735]) && s.b[1087]) {s.store_scale(328, 240, 2.0);s.store_mul_sub_rhs(512, 328, 510, 170);s.store_add(191, 164, 512);s.store_div_from_scalar(328, 1.0, 192);s.store_mul(329, 191, 328);s.store_sub_from_scalar(330, 1.0, 329);s.store_sub_from_scalar(336, 1.0, 330);s.store_square(49, 336);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[735]) && s.b[1087]) {s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1121] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });s.b[1122] = (4.0 == 1.0);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && s.b[1122]) {s.store_scalar(55, 1.0);}
        s.b[1123] = (4.0 == 2.0);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if (((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && s.b[1123]) {s.store_scalar(55, 2.0);}
        s.b[1124] = (4.0 == 4.0);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if ((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && s.b[1124]) {s.store_scalar(55, 3.0);}
        s.b[1125] = (4.0 == 8.0);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (((((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (!s.b[1122])) && (!s.b[1123])) && (!s.b[1124])) && s.b[1125]) {s.store_scalar(55, 4.0);}
        if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {s.store_scalar(54, 0.0);}
        let mut t17: usize = 0;
        while {
            let t16: f64 = if ((((!s.b[735]) && s.b[1087]) && s.b[1121]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t16 != 0.0
        } {
            t17 += 1;assert!(t17 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[735]) && s.b[1087]) && s.b[1121]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1121])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if ((!s.b[735]) && s.b[1087]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(337, 336, 53, 1.0);s.store_sub_from_scalar(190, 1.0, 337);s.store_offset_mul_offset_rhs(478, 190, 190, 1.0, 1.0);}
        if ((!s.b[735]) && s.b[1087]) {
            if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                s.store_offset(479, 190, 1.0);
            } else {
                s.store_scalar(479, (10.0 * 2.220446049250313e-16));
            }
        }
        if ((!s.b[735]) && s.b[1087]) {s.store_div_scaled_product_indices(328, 192, 478, 0.6666666666666667, 479, 1.0);}
        s.b[1126] = (s.v[339] <= 1.0);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });s.b[1127] = (((s.v[164]) as f64).abs() > 1e-6);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && s.b[1127]) {s.store_sub_ad(436, A::add_scaled_product(A::mul3(A::add_scaled_inputs(A::square(s.ad_value(425)), 1.0, A::square(s.ad_value(427)), 0.08333333333333333), s.ad_value(225), s.ad_value(164)), 1.0, s.ad_value(425), s.ad_value(427), (-1.0)), A::div_scaled_product(A::mul3(A::add_scaled_inputs(s.ad_value(425), 2.0, A::div_scaled_product3_by_product(s.ad_value(323), s.ad_value(426), s.ad_value(426), 0.2, s.ad_value(225), s.ad_value(428), 1.0), 1.0), s.ad_value(426), s.ad_value(426)), s.ad_value(426), 0.16666666666666666, s.ad_value(428), 1.0));s.store_div(436, 436, 246);}
        if ((((!s.b[735]) && s.b[1087]) && s.b[1126]) && (!s.b[1127])) {s.copy_ad(436, 425);}
        if (((!s.b[735]) && s.b[1087]) && (!s.b[1126])) {s.store_scaled_add(436, 355, 358, (-0.5));}
        s.b[1131] = (s.v[612] == 0.0);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if s.b[1131] {s.store_offset(480, 190, 0.5);s.store_mul(481, 479, 478);s.store_div_scaled_inputs_indices(482, 480, 0.4, 481, 1.0);s.store_sub_from_scalar(438, 0.6, 482);}
        s.b[1132] = (s.v[438] > (0.5 + 1e-8));s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1132]) {s.store_scalar(438, 0.5);}
        if s.b[1131] {s.copy_ad(439, 438);s.store_scalar(438, 0.5);}
        s.b[1134] = (s.v[145] == 0.0);s.store_scalar(1134, if s.b[1134] { 1.0 } else { 0.0 });s.b[1150] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1134]) && s.b[1150]) {s.store_scalar(316, 0.0);s.copy_ad(314, 162);}
        s.b[1151] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((s.b[1131] && s.b[1134]) && s.b[1150]) && s.b[1151]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {s.store_scalar(1149, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {s.store_primal_div_from_scalar(1135, 1.0, 1149);s.store_mul(1136, 244, 1135);s.store_scale(1137, 1136, p.p191);s.store_add_scaled_product_indices(1140, 1137, 1.0, 80, 229, 1.0);s.store_div_from_scalar(1136, 1.0, 1140);s.store_scale(1139, 1136, 1.034943e-10);s.store_scalar(1136, (1.0 - p.p189));s.store_add_scaled_inputs_product_indices(314, 157, p.p189, 161, p.p189, 1136, 162, 1.0);}
        s.b[1152] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1152]) {s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));}
        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {s.store_sub(1142, 314, 162);s.store_sqrt_square_offset(44, 1142, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(1141, 1142, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1153] = (s.v[1141] < 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1153]) {s.store_scalar(1141, 0.0);}
        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {s.store_mul(1138, 225, 244);s.store_div_from_scalar(1136, 1.0, 1138);s.store_mul(1140, 246, 1136);}
        s.b[1154] = (s.v[1140] < s.v[227]);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if (((s.b[1131] && s.b[1134]) && (!s.b[1150])) && s.b[1154]) {s.copy_ad(1140, 227);}
        if ((s.b[1131] && s.b[1134]) && (!s.b[1150])) {s.store_scale(1146, 229, 9662367879.197212);s.store_scalar(1136, (100000.0 * 10000.0));s.store_scalar(1137, (1.0 / s.v[97]));s.store_mul_mixed_ai(1148, A::add_scaled_inputs_product(s.ad_value(1140), 2.0, A::mul3_scaled_output(s.ad_value(1146), s.ad_value(1141), s.ad_value(1139), 2.0), 1.0, s.ad_value(1136), s.ad_value(1139), 1.0), 1137);s.store_mul(1143, 1148, 1139);s.store_add_scaled_product_indices(1147, 1136, 4.0, 1146, 1141, (2.0 * 4.0));s.store_mul3_lhs(1144, 1147, 1139, 1139);s.store_sqrt_square_add(1145, 1143, 1144);s.store_mul_sub_scaled_inputs_rhs_indices(316, 326, 1145, 0.5, 1143, 0.5);}
        if (s.b[1131] && s.b[1134]) {s.store_scale(316, 316, s.v[127]);}
        if s.b[1131] {s.store_sub_from_scalar(441, s.v[97], 316);}
        s.b[1155] = (s.v[441] < 1e-9);s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1155]) {s.store_scalar(441, 1e-9);}
        if s.b[1131] {s.store_scale(328, 108, (-s.v[98]));s.store_mul(196, 328, 437);s.store_mul(197, 328, 436);s.store_mul(198, 197, 438);}
        s.b[1156] = (p.p43 == 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1156]) {s.store_scale(477, 196, 0.5);s.store_scale(476, 196, (1.0 - 0.5));s.store_mul_scale_offset_mixed_ia(392, 108, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 0.0);}
        if s.b[1131] {s.store_scaled_sub(1157, 157, 164, 0.5);s.store_scale(44, 1157, (2.0 * 1.0 / (p.p227)));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(177, p.p227, 45);}
        s.b[1158] = (s.v[177] < (10.0 * 2.220446049250313e-16));s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1158]) {s.store_scalar(177, (10.0 * 2.220446049250313e-16));}
        if s.b[1131] {s.store_add(176, 161, 177);s.store_scalar(1168, (1.034943e-10 / 100.0));s.store_scale(1169, 437, 0.0001);s.store_scale(1170, 436, 0.0001);s.store_div_from_scalar(1159, p.p92, 1168);s.store_primal_div_from_scalar(1160, p.p93, 1168);s.store_scalar(1161, p.p94);s.store_offset_mul_ad(1162, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1161), 1.0);s.store_add_scaled_products_indices(1163, 1159, 1169, 1.0, 1160, 1170, 1.0);s.store_div(1164, 1163, 1162);s.copy_ad(248, 1164);s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));s.store_offset_add_scaled_inputs_indices(1161, 248, 0.5, 44, 0.5, (1e-10 * 3000.0));}
        s.b[1171] = (s.v[1161] < 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1171]) {s.store_scalar(1161, 0.0);}
        if s.b[1131] {s.store_powf(1163, 1161, (p.p97 - 1.0));s.store_mul(1165, 1163, 1161);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1131] {s.store_powf(1166, 1161, (s.v[111] - 1.0));s.store_mul(1167, 1166, 1161);s.store_scale(249, 1170, 6.241449993689894e18);s.store_add_scaled_inputs_mixed_ai(1159, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(249), (p.p96 * 1e-11), p.p95)), 1.0, s.ad_value(543), s.ad_value(1165), 1.0), 1.0, 1167, 1.0 / (p.p106));s.store_div_from_scalar(251, 1.0, 1159);s.store_scale(251, 251, 0.0001);s.store_mul3_lhs(1172, 225, 244, 441);s.store_sqrt_square_offset(44, 1172, ((4.0 * 1e-50) * 1e-50));s.store_offset_add_scaled_inputs_indices(1172, 1172, 0.5, 44, 0.5, (1e-10 * 1e-50));}
        s.b[1180] = (s.v[1172] < 0.0);s.store_scalar(1180, if s.b[1180] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1180]) {s.store_scalar(1172, 0.0);}
        if s.b[1131] {s.store_div_from_scalar(1173, 1.0, 1172);s.store_mul(1174, 246, 1173);s.store_div_scaled_inputs_indices(1172, 253, 0.2, 251, 1.0);s.store_sqrt_square_sum(252, 1174, 1172);s.store_mul(1175, 251, 252);s.store_div(1173, 1175, 253);}
        s.b[1181] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1181, if s.b[1181] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1181]) {s.store_scalar(1176, 1.0);}
        s.b[1182] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1182, if s.b[1182] { 1.0 } else { 0.0 });
        if ((s.b[1131] && (!s.b[1181])) && s.b[1182]) {s.copy_ad(1176, 1173);}
        if ((s.b[1131] && (!s.b[1181])) && (!s.b[1182])) {s.store_powf(1176, 1173, (p.p113 - 1.0));}
        if s.b[1131] {s.store_mul(1172, 1173, 1176);s.store_offset(1177, 1172, 1.0);}
        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1183, if s.b[1183] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1183]) {s.store_div_from_scalar(1178, 1.0, 1177);}
        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1184, if s.b[1184] { 1.0 } else { 0.0 });
        if ((s.b[1131] && (!s.b[1183])) && s.b[1184]) {s.store_div_from_scalar_sqrt_ad(1178, 1.0, s.ad_value(1177));}
        if ((s.b[1131] && (!s.b[1183])) && (!s.b[1184])) {s.store_powf(1179, 1177, (((-1.0) / p.p113) - 1.0));s.store_mul(1178, 1177, 1179);}
        if s.b[1131] {s.store_mul(250, 251, 1178);s.store_div_scaled_product_mixed_iia(264, 107, 227, 1.0, A::sub_from_scalar(s.v[97], s.ad_value(316)), 1.0);s.store_mul3_lhs(200, 264, 246, 250);s.store_scalar(201, 0.0);}
        s.b[1194] = ((p.p281 > 0.0) && (p.p244 != 0.0));s.store_scalar(1194, if s.b[1194] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1194]) {s.store_scaled_sub(1185, 157, 164, 0.5);s.store_scale(44, 1185, (2.0 * 100.0));s.store_offset_mul_offset_rhs_mixed_ia(45, 44, A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul_offset_rhs(s.ad_value(44), A::mul(s.ad_value(44), A::scale_offset(s.ad_value(44), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_div_from_scalar(1191, 0.01, 45);s.store_sub_from_scalar_ad(1185, 1.1, A::add(s.ad_value(161), s.ad_value(1191)));s.store_sqrt_square_offset(44, 1185, ((4.0 * 0.05) * 0.05));s.store_offset_add_scaled_inputs_indices(1193, 1185, 0.5, 44, 0.5, (1e-10 * 0.05));}
        s.b[1195] = (s.v[1193] < 0.0);s.store_scalar(1195, if s.b[1195] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1194]) && s.b[1195]) {s.store_scalar(1193, 0.0);}
        if (s.b[1131] && s.b[1194]) {s.store_scale(1186, 225, s.v[116]);s.store_mul(1187, 323, 1186);s.store_powf(1186, 1193, p.p245);s.store_mul(1188, 1187, 1186);s.store_offset_scaled(1189, 173, p.p246, 1.0);s.store_scalar(1186, s.v[117]);}
        s.b[1196] = ((s.v[56] < 3.0) || (p.p43 == 1.0));s.store_scalar(1196, if s.b[1196] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1194]) && s.b[1196]) {s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 172, -1.0);}
        if ((s.b[1131] && s.b[1194]) && (!s.b[1196])) {s.store_add_scaled_inputs3_indices(1190, 161, 1.0, 1191, 1.0, 350, -1.0);}
        if (s.b[1131] && s.b[1194]) {s.store_add_product3_rhs_indices(1189, 1189, 173, 1186, 1190, 1.0);s.store_mul(1191, 1188, 1189);s.copy_ad(1188, 1191);}
        if (s.b[1131] && (!s.b[1194])) {s.store_scalar(1188, 0.0);}
        s.b[1197] = (p.p248 != 0.0);s.store_scalar(1197, if s.b[1197] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1197]) {s.store_scale(1185, 225, s.v[118]);s.store_mul(1193, 323, 1185);s.store_mul(1192, 1193, 173);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1131] && (!s.b[1197])) {s.store_scalar(1192, 0.0);}
        s.b[1198] = ((s.v[1188] + s.v[1192]) > 0.0);s.store_scalar(1198, if s.b[1198] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1198]) {s.store_mul_add_rhs(247, 164, 1188, 1192);s.store_mul3_lhs(201, 264, 247, 250);}
        if s.b[1131] {s.store_add(199, 200, 201);s.copy_ad(203, 201);}
        s.b[1208] = (p.p33 != 0.0);s.store_scalar(1208, if s.b[1208] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1208]) {s.copy_ad(1201, 549);s.store_scalar(1202, (s.v[124] - p.p71));s.store_div_from_scalar_square_ad(1203, 1.0, s.ad_value(1202));s.store_mul_ad_product_lhs_mixed_ai(1204, A::mul_sub_from_scalar_lhs_scaled_output(p.p69, s.ad_value(233), s.ad_value(324), (2.0 * 1.034943e-10)), 1201, 1203);s.store_mul(186, 1204, 235);s.store_offset_scaled(1200, 173, p.p155, p.p154);s.store_mul(206, 186, 1200);s.store_sub_from_scalar_scaled_input(1199, p.p156, 157, p.p157);s.store_add_scaled_inputs3_offset_indices(207, 174, 1.0, 1199, 1.0, 206, 1.0, (-s.v[123]));s.store_mul3_lhs(210, 205, 324, 324);s.store_scaled_mul(211, 210, 225, 0.5);s.store_scaled_mul(212, 211, 225, 2.0);s.store_offset_sub_ad(1205, A::offset(A::add_scaled_product(s.ad_value(227), 1.0, s.ad_value(210), s.ad_value(225), (-0.25)), ((s.v[123]) + ((-p.p156)))), s.ad_value(206), 1e-50);s.store_offset_sub(1199, 174, 1205, (-0.005));}
        if (s.b[1131] && s.b[1208]) {s.store_scalar(327, (if (s.v[1205] >= 0.0) { 1.0 } else { (-1.0) }));}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_add_scaled_square_product(1201, 1199, 1.0, 327, 1205, (4.0 * 0.005));s.store_sub_mixed_ai(1202, A::add_scaled_inputs4_offset(s.ad_value(1205), 1.0, s.ad_value(1199), 0.5, s.ad_value(1201), 0.5, s.ad_value(206), 1.0, (((-s.v[123])) + (p.p156))), 514);s.store_offset_mul(1203, 225, 1202, (-1.0));s.store_div_from_scalar(1204, 4.0, 212);s.store_offset_mul(1200, 1203, 1204, 1.0);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1209] = (s.v[1199] < 0.0);s.store_scalar(1209, if s.b[1209] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1209]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(213, 1199, 1e-50);s.store_add_mul_sub_from_scalar_rhs_indices(215, 207, 211, 1.0, 213);s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_scalar_offset_denominator(2.0, s.ad_value(207), 1e-50, 1.0));s.store_mul_ln_mixed_ia(216, 327, A::mul(A::div_scalar_by_product(1.0, s.ad_value(209), s.ad_value(210), 1.0), A::square(s.ad_value(207))));s.store_div_scaled_value_offset_denominator(1202, s.ad_value(216), 1.0, s.ad_value(207), 1e-50, 1.0);s.store_offset_sub(217, 216, 215, (-0.002));s.store_sqrt_add_scaled_square_input(327, 217, 1.0, 216, (4.0 * 0.002));s.store_add_scaled_inputs3_indices(218, 216, 1.0, 217, (-0.5), 327, (-0.5));s.store_div_from_scalar(1199, 1.0, 327);s.store_mul_exp_mixed_ia(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));s.store_add_offset_lhs_mixed_ai(1200, A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0), 327);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
        s.b[1210] = (s.v[1199] < 0.0);s.store_scalar(1210, if s.b[1210] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1210]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(219, 1199, (10.0 * 2.220446049250313e-16));s.store_offset_mul_ad(1200, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.01) * 0.01));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1211] = (s.v[1199] < 0.0);s.store_scalar(1211, if s.b[1211] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1211]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_sqrt_offset_input(220, 1199, (10.0 * 2.220446049250313e-16));s.store_mul_sub_rhs(221, 208, 219, 220);s.store_sub(1200, 215, 218);s.store_sqrt_square_offset(44, 1200, ((4.0 * 0.1) * 0.1));s.store_offset_add_scaled_inputs_indices(1199, 1200, 0.5, 44, 0.5, (1e-10 * 0.1));}
        s.b[1212] = (s.v[1199] < 0.0);s.store_scalar(1212, if s.b[1212] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1208]) && s.b[1212]) {s.store_scalar(1199, 0.0);}
        if (s.b[1131] && s.b[1208]) {s.store_div_scaled_value_offset_denominator(1206, s.ad_value(157), 1.0, s.ad_value(1199), (10.0 * 2.220446049250313e-16), 1.0);s.store_square(49, 1206);s.store_scalar(50, 1.0);s.store_scalar(51, 1.0);s.store_scalar(52, 1.0);s.store_scalar(54, 0.0);s.store_scalar(55, 0.0);s.store_scalar(48, 0.0);s.store_scalar(53, 0.0);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_mul(51, 51, 49);s.store_mul(52, 52, 50);s.store_add(48, 51, 52);s.copy_ad(53, 48);}
        s.b[1213] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1213, if s.b[1213] { 1.0 } else { 0.0 });s.b[1214] = (4.0 == 1.0);s.store_scalar(1214, if s.b[1214] { 1.0 } else { 0.0 });
        if (((s.b[1131] && s.b[1208]) && s.b[1213]) && s.b[1214]) {s.store_scalar(55, 1.0);}
        s.b[1215] = (4.0 == 2.0);s.store_scalar(1215, if s.b[1215] { 1.0 } else { 0.0 });
        if ((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && s.b[1215]) {s.store_scalar(55, 2.0);}
        s.b[1216] = (4.0 == 4.0);s.store_scalar(1216, if s.b[1216] { 1.0 } else { 0.0 });
        if (((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && s.b[1216]) {s.store_scalar(55, 3.0);}
        s.b[1217] = (4.0 == 8.0);s.store_scalar(1217, if s.b[1217] { 1.0 } else { 0.0 });
        if ((((((s.b[1131] && s.b[1208]) && s.b[1213]) && (!s.b[1214])) && (!s.b[1215])) && (!s.b[1216])) && s.b[1217]) {s.store_scalar(55, 4.0);}
        if ((s.b[1131] && s.b[1208]) && s.b[1213]) {s.store_scalar(54, 0.0);}
        let mut t19: usize = 0;
        while {
            let t18: f64 = if (((s.b[1131] && s.b[1208]) && s.b[1213]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            t18 != 0.0
        } {
            t19 += 1;assert!(t19 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1131] && s.b[1208]) && s.b[1213]) {s.store_sqrt(53, 53);s.store_primal_offset(54, 54, 1.0);}
        }
        if ((s.b[1131] && s.b[1208]) && (!s.b[1213])) {s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));}
        if (s.b[1131] && s.b[1208]) {s.store_div_from_scalar(53, 1.0, 53);s.store_scaled_mul(1207, 1206, 53, 1.0);s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));s.store_div_scaled_product_mixed_aii(222, A::mul3(s.ad_value(214), s.ad_value(250), s.ad_value(221)), 1207, 1.0, 441, 1.0);s.store_add(199, 199, 222);}
        s.b[1218] = ((p.p30 != 0.0) && (p.p32 != 0.0));s.store_scalar(1218, if s.b[1218] { 1.0 } else { 0.0 });
        if (s.b[1131] && s.b[1218]) {s.store_square(294, 192);s.store_mul3_affine_lhs(295, 227, 324, 2.0, 0.0, 246);s.store_sub(296, 294, 295);s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(294, 294, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1219] = (s.v[294] < 0.0);s.store_scalar(1219, if s.b[1219] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1218]) && s.b[1219]) {s.store_scalar(294, 0.0);}
        if (s.b[1131] && s.b[1218]) {s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));s.store_offset_add_scaled_inputs_indices(296, 296, 0.5, 44, 0.5, (1e-10 * 0.001));}
        s.b[1220] = (s.v[296] < 0.0);s.store_scalar(1220, if s.b[1220] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1218]) && s.b[1220]) {s.store_scalar(296, 0.0);}
        if (s.b[1131] && s.b[1218]) {s.store_sub(297, 294, 296);}
        s.b[1221] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));s.store_scalar(1221, if s.b[1221] { 1.0 } else { 0.0 });
        if ((s.b[1131] && s.b[1218]) && s.b[1221]) {s.store_scalar(146, 0.0);}
        if ((s.b[1131] && s.b[1218]) && (!s.b[1221])) {s.store_scalar(146, 1.0);}
        s.copy_ad(202, 199);s.store_scalar(204, 0.0);s.b[1222] = ((p.p281 > 0.0) && (p.p285 > 0.0));s.store_scalar(1222, if s.b[1222] { 1.0 } else { 0.0 });
        if s.b[1222] {s.store_scalar(1229, s.v[99]);s.store_scalar(1233, p.p237);s.store_offset_add_scaled_inputs3_offset_indices(1234, 158, 1.0, 185, 1.0, 320, -1.0, (-s.v[123]), (-p.p286));s.store_offset(1235, 182, p.p286);s.store_scalar(1237, p.p285);s.store_scalar(1236, p.p283);s.store_scalar(1227, s.v[70]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1222] {s.store_mul_ln_mixed_ia(1228, 227, A::div_scaled_product_by_product(s.ad_value(1227), s.ad_value(536), 1.0, s.ad_value(230), s.ad_value(230), 1.0));}
        if s.b[1222] {
            if (p.p43 == 1.0) {
                s.copy_ad(1225, 435);
            } else {
                s.copy_ad(1225, 350);
            }
        }
        if s.b[1222] {s.store_sqrt_ad(1230, A::div_scaled_product3(A::sub(s.ad_value(1228), s.ad_value(1225)), s.ad_value(536), s.ad_value(1227), ((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)), A::add(s.ad_value(536), s.ad_value(1227)), 1.0));s.store_mul(1224, 1230, 1229);s.store_div_scaled_product_add_scaled_denominator_indices(1223, 1224, 1224, (-0.25), 157, 1.0, 1224, 1.0, 1.0);s.copy_ad(1249, 1223);s.copy_ad(1250, 1235);s.store_offset_div_scaled_offset_numerator(336, A::mul(s.ad_value(225), A::sub(s.ad_value(1234), s.ad_value(1249))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0, 1.0);}
        if s.b[1222] {
            if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(336, (10.0 * 2.220446049250313e-16));
            }
        }
        if s.b[1222] {s.store_add_product3_rhs_mixed_iia(376, 1234, 241, 225, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336))), 0.5);}
        s.b[1251] = (s.v[158] < ((s.v[123] + s.v[1250]) * 0.5));s.store_scalar(1251, if s.b[1251] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1251]) {s.store_scalar(144, 0.0);}
        s.b[1252] = ((s.v[144] == 0.0) || (1.0 != 0.0));s.store_scalar(1252, if s.b[1252] { 1.0 } else { 0.0 });
        if (s.b[1222] && s.b[1252]) {s.store_mul_sub_rhs(181, 225, 376, 1249);}
        s.b[1253] = (s.v[181] < 3.0);s.store_scalar(1253, if s.b[1253] { 1.0 } else { 0.0 });
        if ((s.b[1222] && s.b[1252]) && s.b[1253]) {s.store_mul_sub_rhs(337, 225, 1234, 1249);s.store_div_scalar_by_product_indices(328, 1.0, 225, 240, (1.414213562373095 / 108.0));s.store_offset_scaled(329, 328, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(330, (-2916.0), A::scale(s.ad_value(328), 81.0), 1.0, 328, 337, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(331, 1458.0, A::scaled_offset(s.ad_value(328), 54.0, 81.0), 1.0, 328, 337, 27.0);s.store_square(331, 331);s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(329), s.ad_value(329), s.ad_value(329), 4.0), s.ad_value(331)))), 0.3333333333333333);s.store_add_scaled_inputs_mixed_ai(336, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(329), 1.259921049894873, s.ad_value(332), 3.0)), 1.0, 332, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(376, 1249, 1.0, 336, 227, 1.0);s.copy_ad(378, 376);}
        s.b[1254] = ((s.v[158] - s.v[383]) <= s.v[1250]);s.store_scalar(1254, if s.b[1254] { 1.0 } else { 0.0 });s.b[1255] = (p.p43 == 0.0);s.store_scalar(1255, if s.b[1255] { 1.0 } else { 0.0 });
        if ((((s.b[1222] && s.b[1252]) && (!s.b[1253])) && s.b[1254]) && s.b[1255]) {s.store_div_from_scalar(327, 1.0, 323);s.store_scale(328, 1233, 9662367879.197212);s.store_scalar(329, (1.0 / s.v[93]));s.store_div_from_scalar_ad(330, 1.0, A::add_scaled_inputs3(s.ad_value(327), 1.0, s.ad_value(328), 1.0, s.ad_value(329), 1.0));}
    }
}
