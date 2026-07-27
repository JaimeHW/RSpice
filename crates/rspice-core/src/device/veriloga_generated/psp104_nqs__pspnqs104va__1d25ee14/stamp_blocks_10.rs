#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_160(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {s.store_add_scaled_product_mixed_iia(2029, 2400, 1.0, 2390, A::sqrt(s.ad_value(2400)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2403, A::div_scaled_inputs2(s.ad_value(2402), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2400, 0.5, A::offset(s.ad_value(191), 1.0), 2401, (-1.0));s.store_primal_offset_scaled(2404, 2400, 0.5, 2.0);s.store_add(2405, 2400, 2401);s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2402), 1.0, s.ad_value(2405), (-1.0), s.ad_value(2390), A::sqrt(s.ad_value(2405)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0);s.store_add_scaled_inputs(2406, 2028, 2.0, 2404, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2403, 0.5, 2406, 0.5, 2403, 2406, 20.0, 0.5);s.store_add_scaled_inputs3_indices(2029, 2402, 2.0, 2401, (-2.0), 2404, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2407, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2407, 0.5, 2404, 0.5, 2407, 2404, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2408, 2028, 0.5, 2404, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(2029, 702, A::div(s.ad_value(2408), s.ad_value(2404)), 1.0, 1.0);}
        s.b[2551] = (s.v[2029] > (-230.25850929940458));s.store_scalar(2551, if s.b[2551] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && s.b[2551]) {s.store_exp(2409, 2029);}
        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && (!s.b[2551])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2409, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2547] && s.b[2548]) {s.store_offset_mul(2410, 701, 2409, 1.0);s.store_scale(2411, 2410, s.v[715]);s.store_mul_ad_product_rhs(2412, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));s.store_mul_scale_offset_indices(2413, 2411, 2412, 1.0, 1.0);s.store_div_from_scalar(2414, 1.0, 2413);s.store_mul_mixed_ia(2398, 2390, A::sqrt_scaled_input(s.ad_value(2414), s.v[715]));s.store_square(2399, 2398);s.store_div_from_scalar(2415, 1.0, 2399);s.store_mul(2416, 2392, 2414);s.store_mul(2417, 2395, 2414);s.store_div_scaled_value_offset_denominator(2418, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2419, 196, 2418, A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));s.store_mul(2420, 2388, 2414);s.store_sqrt_square_add(2028, 2391, 2389);s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2391), s.ad_value(2419))), s.ad_value(2389));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2421, 2414, 2419, 0.5, 2028, 0.5, 2029, ((-1.0) * (0.5)), 0.0);s.store_add(2422, 2420, 2416);s.store_sub(2423, 2422, 2421);}
        s.b[2552] = (p[45] > 0.0);s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });s.b[2553] = (((s.v[2423]) as f64).abs() < 1e-5);s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_161(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && s.b[2553]) {s.store_offset_ad(2424, A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2423), 1.0, A::scale(s.ad_value(2423), 0.3125), 0.5)), 1.0);}
        s.b[2554] = (s.v[2423] < 460.51701859880916);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && s.b[2554]) {s.store_exp_neg_input(2438, 2423);}
        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && (!s.b[2554])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));}
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {s.store_offset_ad(2424, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2398), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2423))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2423), 1.0, s.ad_value(2438))), 2.0), 1.0);}
        if ((s.b[2547] && s.b[2548]) && (!s.b[2552])) {s.store_offset_div_scaled_inputs_sqrt_rhs(2424, 2398, 0.5, 2423, 1.0, 1.0);}
        if (s.b[2547] && s.b[2548]) {s.store_add_scaled_value_products_mixed_iiaia(2425, 2423, 1.0, 2398, A::sqrt(s.ad_value(2423)), 1.0, 2424, A::ln(A::offset(s.ad_value(2424), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2426, 2417, 1.0, 2425, (-1.0), 2424, 1.0);s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0));s.store_scalar(2431, 0.0);s.store_scalar(2433, 1.0);}
        s.b[2555] = (s.v[2426] > (-30.0));s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_offset_mul(2427, 2424, 2426, (-1.0));s.store_scaled_add_sqrt_square_offset_rhs(2027, 2427, 2427, 10.0, 0.5);s.store_sub_mixed_ia(2428, 2426, A::ln(s.ad_value(2027)));s.store_scaled_add_sqrt_square_offset_rhs(2429, 2428, 2428, 2.0, 0.5);}
        s.b[2556] = ((s.v[2426] - s.v[2429]) < 230.25850929940458);s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2556]) {s.store_exp_sub(2027, 2426, 2429);}
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2556])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_div(2430, 2027, 2424);s.store_sub_mixed_ai(2027, A::scaled_offset(s.ad_value(2429), 1.0, 2.0), 2430);}
        s.b[2557] = (s.v[2430] > 1e-6);s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2557]) {s.store_mul_scale_offset_mixed_ia(2431, 2424, A::sub(s.ad_value(2429), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2430), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2430), 1.0)), 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2557])) {s.store_mul_ad_affine_product_rhs(2431, 2424, s.ad_value(2430), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);}
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2417, 0.5, 2431, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_162(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2433, 2432, 2432, 1.0, 2431, 1.0);s.store_add_scaled_product_indices(2423, 2422, 1.0, 2433, 2421, (-1.0));}
        if (s.b[2547] && s.b[2548]) {s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);}
        let (t1,) = {
    if (s.b[2547] && s.b[2548]) {
        let t0: f64 = (1e-5 * s.v[2434]);
        (t0,)
    } else {
        (s.v[2435],)
    }
};
        s.store_scalar(2435, t1);
        if (s.b[2547] && s.b[2548]) {s.store_div_from_scalar(2436, 1.0, 2434);s.store_scalar(2543, 0.0);s.store_scalar(2437, 0.0);}
        s.b[2558] = (s.v[2423] < 460.51701859880916);s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2558]) {s.store_exp_neg_input(2438, 2423);}
        if ((s.b[2547] && s.b[2548]) && (!s.b[2558])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2559] = (((s.v[2417]) as f64).abs() <= s.v[2435]);s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2559]) {s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2437, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2438)), s.ad_value(2398), s.ad_value(2523)), 1.0));}
        s.b[2560] = (s.v[2417] < (-s.v[2435]));s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {s.store_neg(2525, 2417);s.store_scaled_mul(2526, 2525, 2436, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2527, 2526, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2522, 2525, 2527);s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::offset(s.ad_value(2527), 1.0), 1.0);s.store_sub_scaled_inputs(2529, 2522, 2.0, 2399, 1.0);s.store_sub_ln_mul_lhs(2530, 2528, 2415, 2527);s.store_add(824, 2528, 2529);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.5, s.ad_value(2528), 1.0), 1.0);s.store_add_mixed_ia(2531, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.3333333333333333, s.ad_value(2528), 1.0))), 1.0));}
        s.b[2561] = (s.v[2531] < 230.25850929940458);s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && s.b[2561]) {s.store_exp(2532, 2531);}
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && (!s.b[2561])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2532, 2531, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {s.store_div_from_scalar(2533, 1.0, 2532);s.store_div_from_scalar_offset_square(2522, 1.0, 2531, 2.0);s.store_mul_square_lhs(2534, 2531, 2522);s.store_mul3_affine_lhs(2535, 2531, 2522, 4.0, 0.0, 2522);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);s.store_sub(2522, 2525, 2531);s.store_mul(2523, 2438, 2533);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_163(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {s.store_add_scaled_product_mixed_iia(2537, 2522, 2.0, 2399, A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2523), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2535)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2531), (-1.0), s.ad_value(2523), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2532), 1.0, s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2437, 2531, -1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);}
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {s.store_div_from_scalar_offset_scaled_input(2539, 1.0, 2398, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2540, 2539, A::mul_scaled_lhs(s.ad_value(2434), 1.25, s.ad_value(2539)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2541, 2417, 2436, A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));}
        s.b[2562] = ((-s.v[2541]) > (-230.25850929940458));s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2562]) {s.store_exp_neg_input(2522, 2541);}
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2562])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2522, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2541)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {s.store_sub_from_scalar(2542, 1.0, 2522);s.store_add_scaled_inputs_product_mixed_iiia(2543, 2417, 1.0, 2399, 0.5, 2398, A::sqrt(A::add_scaled_inputs3(s.ad_value(2417), 1.0, s.ad_value(2399), 0.25, s.ad_value(2542), -1.0)), (-1.0));s.store_offset(2544, 2423, 3.0);s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2543), s.ad_value(2544)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt_square_offset(s.ad_value(2544), 5.0), 0.5));s.store_sub(2522, 2417, 2527);s.store_exp_neg_input(2523, 2527);s.store_div_from_scalar_offset_square(2524, 1.0, 2527, 2.0);s.store_mul_square_lhs(2534, 2527, 2524);s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), 2524, 2524);}
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_164(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2545, 1.0, 2399, A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2530, 2423, 1.0, 2527, (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);s.store_add(824, 2528, 2529);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);s.store_add_mixed_ia(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));}
        s.b[2563] = (s.v[2546] < 230.25850929940458);s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2563]) {s.store_exp(2532, 2546);s.store_div_from_scalar(2533, 1.0, 2532);s.store_mul(2532, 2438, 2532);}
        s.b[2564] = (s.v[2546] > (s.v[2423] - 230.25850929940458));s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && s.b[2564]) {s.store_exp_sub(2532, 2546, 2423);s.store_div(2533, 2438, 2532);}
        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && (!s.b[2564])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);s.store_mul_square_lhs(2534, 2546, 2522);s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);s.store_sub(2522, 2417, 2546);s.store_add_scaled_product_mixed_iia(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));s.store_add_scaled_inputs_mixed_ia(2437, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_165(
        s: &mut Scratch,
    ) {
        if (s.b[2547] && s.b[2548]) {s.store_scalar(2440, 0.0);s.store_scalar(2441, 0.0);s.store_scalar(2442, 0.0);s.store_scalar(2443, 0.0);s.store_scalar(2444, 0.0);s.store_scalar(2445, 0.0);s.store_scalar(2446, 0.0);s.store_scalar(2447, 1.0);s.store_scalar(2448, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_166(
        s: &mut Scratch,
    ) {
        if (s.b[2547] && s.b[2548]) {s.store_sub(2449, 2417, 2437);s.store_scalar(2450, 0.0);s.store_mul(2451, 2413, 2449);s.store_scalar(2452, 1.0);s.store_scalar(2453, 1.0);s.store_scalar(2457, 1.0);s.store_scalar(2458, 1.0);s.store_scalar(2460, 1.0);}
        s.b[2565] = (s.v[2417] > 0.0);s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {s.store_div_from_scalar_offset_square(2027, 1.0, 2437, 2.0);s.store_mul_square_lhs(2439, 2437, 2027);s.store_mul3_affine_lhs(2440, 2437, 2027, 4.0, 0.0, 2027);s.store_mul_ad_product_lhs_mixed_ai(2441, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2439), 12.0), 2027, 2027);s.store_scalar(2442, 0.0);}
        s.b[2566] = (s.v[2437] < 230.25850929940458);s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2566]) {s.store_exp(2442, 2437);s.store_div_from_scalar(2443, 1.0, 2442);s.store_mul(2442, 2438, 2442);}
        s.b[2567] = (s.v[2437] > (s.v[2423] - 230.25850929940458));s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {s.store_exp_sub(2442, 2437, 2423);s.store_div(2443, 2438, 2442);}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2442, 1e-100, A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2443, 1e-100, 2437, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {s.store_add_scaled_product_mixed_iia(2444, 2442, 1.0, 2438, A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439)), (-1.0));}
        s.b[2568] = (s.v[2437] < 1e-5);s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2568]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2445, 2437, 1.0, 2437, 1.0, 2437, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2444, A::mul3(s.ad_value(2438), s.ad_value(2437), s.ad_value(2437)), 2437, A::scale_offset(s.ad_value(2437), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2437), 1.0, A::scale(s.ad_value(2437), 0.25), 0.3333333333333333));s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2447, 2398, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), 1.0, A::square(s.ad_value(2437)), 0.16666666666666666), 0.7071067811865475, 2027, 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2568])) {s.store_add_offset_lhs(2445, 2437, (-1.0), 2443);s.store_sqrt(2446, 2445);s.store_offset_scaled_ad(2447, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2443)), s.ad_value(2446)), 0.5, 1.0);}
        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {s.store_div_scaled_offset_numerator(2448, A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2397)), 1.0, 1.0, A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0), 1.0);}
        s.b[2569] = (s.v[2444] > 1e-100);s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul_sqrt_mixed_ia(2449, 2398, A::add(s.ad_value(2445), s.ad_value(2444)));s.store_div_scaled_product3_mixed_iiia(2450, 2399, 2444, 2413, 1.0, A::add_scaled_product(s.ad_value(2449), 1.0, s.ad_value(2398), s.ad_value(2446), 1.0), 1.0);s.store_mul3_lhs(2451, 2446, 2398, 2413);}
        s.b[2570] = (s.v[217] < 0.0);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2570]) {s.store_div_from_scalar_sub_from_scalar_ad(2452, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2397)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2570])) {s.store_offset_mul(2452, 217, 2397, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_167(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2571] = (s.v[218] < 0.0);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2571]) {s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2450, 1.0);}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2571])) {s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2450, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul_product3_indices(2454, 2450, 757, 2452, 2453, 1.0);s.store_mul_add_scaled_product_rhs_indices(2455, 774, 2451, 1.0, 775, 2450, 1.0);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2445), 1.0, A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2457, 2448, A::offset(s.ad_value(2456), 1.0), 2454);}
        s.b[2572] = (s.v[221] < 0.0);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2572]) {s.store_div_from_scalar_sub_from_scalar_ad(2458, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2397)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2572])) {s.store_offset_mul(2458, 221, 2397, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul(2029, 2450, 2458);s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2573] = (s.v[222] < 0.0);s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2573]) {s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2573])) {s.store_offset_mul(2460, 222, 2459, 1.0);}
        if (s.b[2547] && (!s.b[2548])) {s.copy_ad(2395, 1822);s.copy_ad(2397, 1823);s.copy_ad(2413, 1824);s.copy_ad(2414, 1825);s.copy_ad(2398, 1826);s.copy_ad(2399, 1827);s.copy_ad(2415, 1828);s.copy_ad(2417, 1829);s.copy_ad(2422, 1830);s.copy_ad(2423, 1831);s.copy_ad(2434, 1832);}
        let (t2,) = {
    if (s.b[2547] && (!s.b[2548])) {
        (s.v[1833],)
    } else {
        (s.v[2435],)
    }
};
        s.store_scalar(2435, t2);
        if (s.b[2547] && (!s.b[2548])) {s.copy_ad(2436, 1834);s.copy_ad(2543, 1835);s.copy_ad(2438, 1836);s.copy_ad(2437, 1837);s.copy_ad(2440, 1838);s.copy_ad(2441, 1839);s.copy_ad(2442, 1840);s.copy_ad(2443, 1841);s.copy_ad(2445, 1842);s.copy_ad(2444, 1843);s.copy_ad(2446, 1844);s.copy_ad(2447, 1845);s.copy_ad(2448, 1846);s.copy_ad(2449, 1847);s.copy_ad(2450, 1848);s.copy_ad(2451, 1849);s.copy_ad(2452, 1850);s.copy_ad(2453, 1851);s.copy_ad(2457, 1852);s.copy_ad(2458, 1853);s.copy_ad(2460, 1854);}
        if s.b[2547] {s.copy_ad(2393, 720);s.copy_ad(2394, 777);}
        s.b[2574] = (p[48] != 0.0);s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        if (s.b[2547] && s.b[2574]) {s.copy_ad(2393, 721);s.copy_ad(2394, 778);}
        if s.b[2547] {s.store_scalar(2462, 0.0);s.store_scale(2461, 2413, 4.60517018598809);s.copy_ad(2478, 2461);s.copy_ad(2479, 826);s.store_mul(2480, 826, 2414);s.copy_ad(2484, 2437);s.store_scalar(2485, 0.0);s.store_scalar(2488, 0.0);s.copy_ad(2490, 2443);s.copy_ad(2491, 2445);s.copy_ad(2493, 2444);s.copy_ad(2494, 2451);s.copy_ad(2495, 2437);s.copy_ad(2496, 2443);s.copy_ad(2498, 2444);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_168(
        s: &mut Scratch,
    ) {
        if s.b[2547] {s.copy_ad(2499, 2445);s.store_sub(2500, 2417, 2437);s.store_scalar(2501, 1.0);s.store_scalar(2503, 1.0);s.store_scalar(2502, 0.0);s.copy_ad(2512, 2450);s.store_mul(2516, 2500, 2413);s.store_scalar(2513, 0.0);s.copy_ad(2514, 2451);s.store_scalar(2519, 0.0);s.store_scalar(2518, 1.0);s.copy_ad(2521, 2393);s.copy_ad(2520, 2516);}
        s.b[2575] = (s.v[2417] > 0.0);s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });s.b[2576] = (s.v[2444] > 1e-100);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul(2521, 2393, 2460);s.store_div(2462, 2521, 2457);s.store_add_scaled_inputs(2463, 2449, 1.0, 2399, 0.5);s.store_div_scaled_product_by_product_indices(2027, 2399, 2442, 1.0, 2463, 2463, 1.0);}
        s.b[2577] = (s.v[2027] > 0.0001);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) {s.store_sub_from_scalar(2028, 1.0, 2027);}
        s.b[2578] = (s.v[2028] < 1e-10);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && s.b[2578]) {s.store_scalar(2029, 1.0);}
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && (!s.b[2578])) {s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));}
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2577])) {s.store_scale(2029, 2027, 0.5);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul(2464, 2029, 2463);}
        s.b[2579] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {s.store_scaled_mul(2465, 2413, 2464, 0.475);s.store_add_scaled_product_indices(2027, 2450, 1.0, 2447, 2465, (-1.0));s.store_scaled_add_sqrt_square_offset_rhs(2466, 2027, 2027, 1e-12, 0.5);s.store_add_scaled_value_products_mixed_iiiai(2467, 2450, (-1.0), 2413, 2449, 1.0, A::offset(s.ad_value(2447), (-1.0)), 2465, 1.0);s.store_offset_div_scaled_product_indices(2468, 2399, 2413, 0.5, 2467, 1.0, 1.0);s.store_add_scaled_product_indices(2027, 2467, 1.0, 775, 2466, 1.0);s.store_pow_ad(2469, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));s.store_mul_mixed_ai(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2468), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2469);s.store_div(2027, 2466, 2467);s.store_mul_pow_mixed_iaa(2470, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));s.store_mul_div_scaled_product_mixed_iiai(2029, 2470, 707, A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2467, 1.0);s.store_mul_product3_indices(2471, 2466, 757, 2452, 2453, 1.0);s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), s.ad_value(2468), (-1.0), s.ad_value(2029), 1.0), 1.0);}
        s.b[2580] = (s.v[2027] < 230.25850929940458);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && s.b[2580]) {s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);}
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && (!s.b[2580])) {s.copy_ad(2028, 2027);}
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {s.store_div_scaled_product3_mixed_iiia(2472, 2465, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2469), 1.0, s.ad_value(2470), 1.0, s.ad_value(2471), 1.0, 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_169(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {s.store_mul_scale_offset_mixed_ia(2473, 2464, A::div_scaled_value_offset_denominator(s.ad_value(2472), 1.0, A::sqrt_square_offset(s.ad_value(2472), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2579])) {s.copy_ad(2473, 2464);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul3_affine_lhs(2474, 2413, 2462, 0.7071067811865475, 0.0, 2473);}
        s.b[2581] = (s.v[0] == (-1.0));s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2581]) {s.store_div_mixed_ia(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_div_from_scalar_offset_ad(2475, 2.0, A::sqrt(A::scale_offset(s.ad_value(2474), 4.0, 1.0)), 1.0);s.store_mul(2027, 2475, 2474);s.store_mul_ad_product_rhs_mixed_ia(2476, 2473, 2475, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2475)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2475), 4.0), 1.0)), 1.0));s.store_scale(2477, 2476, 0.99);s.store_div_scaled_product3_mixed_iaii(2027, 2477, A::sub_scaled_inputs(s.ad_value(2477), 1.0, s.ad_value(2463), 2.0), 2415, 1.0, 2444, 1.0);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul_sub_mixed_iia(2478, 2413, 2477, A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if ((s.b[2547] && s.b[2575]) && (!s.b[2576])) {s.copy_ad(2478, 2461);}
        if (s.b[2547] && s.b[2575]) {s.store_offset(2027, 2394, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2478, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product_add_scaled_denominator(2479, 2478, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);s.store_mul(2480, 2479, 2414);s.store_add(2481, 2423, 2480);}
        s.b[2582] = (s.v[2480] < 460.51701859880916);s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2582]) {s.store_exp_neg_input(2482, 2480);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2582])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2482, 1e-200, 2480, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2483, 2438, 2482);}
        s.b[2583] = (((s.v[2417]) as f64).abs() <= s.v[2435]);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2583]) {s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2484, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2483)), s.ad_value(2398), s.ad_value(2523)), 1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_offset(2544, 2481, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_170(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2543), s.ad_value(2544)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt_square_offset(s.ad_value(2544), 5.0), 0.5));s.store_sub(2522, 2417, 2527);s.store_exp_neg_input(2523, 2527);s.store_div_from_scalar_offset_square(2524, 1.0, 2527, 2.0);s.store_mul_square_lhs(2534, 2527, 2524);s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), 2524, 2524);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2545, 1.0, 2399, A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2530, 2481, 1.0, 2527, (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);s.store_add(824, 2528, 2529);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);s.store_add_mixed_ia(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));}
        s.b[2584] = (s.v[2546] < 230.25850929940458);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {s.store_exp(2532, 2546);s.store_div_from_scalar(2533, 1.0, 2532);s.store_mul(2532, 2483, 2532);}
        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {s.store_exp_sub(2532, 2546, 2481);s.store_div(2533, 2483, 2532);}
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_171(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_mul_square_lhs(2534, 2546, 2522);s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);s.store_sub(2522, 2417, 2546);s.store_add_scaled_product_mixed_iia(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));s.store_add_scaled_inputs_mixed_ia(2484, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);}
        if (s.b[2547] && s.b[2575]) {s.store_sub(2485, 2484, 2437);}
        s.b[2586] = (s.v[2485] < 1e-10);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {s.store_add_scaled_inputs_product_mixed_iiia(2486, 2417, 2.0, 2437, (-2.0), 2399, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2440), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2399, A::add_scaled_value_products(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2486, 1.0, 2027, 2487, (-2.0));s.store_scaled_div_mixed_ia(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2484, 2437, 2485);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2488, 2485, 2413);s.store_div_scaled_product_offset_denominator_mixed_iia(2489, 2484, 2484, 1.0, A::square(s.ad_value(2484)), 2.0, 1.0);}
        s.b[2587] = (s.v[2484] < 230.25850929940458);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {s.store_exp_neg_input(2490, 2484);}
        s.b[2588] = (s.v[2484] < 1e-5);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2491, 2484, 1.0, 2484, 1.0, 2484, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2493, A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), 2484, A::scale_offset(s.ad_value(2484), 1.75, 1.0));}
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);s.store_sqrt(2492, 2491);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_172(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2493, 2483, A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, 2484, (-1.0), 2489, -1.0, (-1.0));}
        s.b[2589] = (s.v[2484] > (s.v[2481] - 230.25850929940458));s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && s.b[2589]) {s.store_exp_sub(2027, 2484, 2481);s.store_div(2490, 2483, 2027);s.store_add_scaled_product_mixed_iia(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));}
        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && (!s.b[2589])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2490, 1e-100, 2484, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2587])) {s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);s.store_sqrt(2492, 2491);}
        if (s.b[2547] && s.b[2575]) {s.store_mul3_lhs(2494, 2492, 2398, 2413);s.store_scaled_add(2495, 2437, 2484, 0.5);s.store_scalar(2496, 0.0);s.store_mul(2027, 2490, 2443);}
        s.b[2590] = (s.v[2027] > 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2590]) {s.store_sqrt(2496, 2027);}
        if (s.b[2547] && s.b[2575]) {s.store_scaled_add(2497, 2444, 2493, 0.5);s.store_add_scaled_product_mixed_iaa(2498, 2497, 1.0, A::square(s.ad_value(2485)), A::sub_scaled_inputs(s.ad_value(2496), 1.0, s.ad_value(2415), 2.0), 0.125);}
        s.b[2591] = (s.v[2495] < 1e-5);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2499, 2495, 1.0, 2495, 1.0, 2495, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));}
        s.b[2592] = (s.v[730] > 0.0);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2591]) && s.b[2592]) {s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));}
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);s.store_add_mixed_ia(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));}
        s.b[2593] = (s.v[730] > 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {s.store_add_scaled_sub_value_product_indices(2504, 1.0, 2496, 1.0, 2500, 2415, 2.0);s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2501), 1.0, s.ad_value(2501), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2505, 730, A::square(s.ad_value(2027)), 2399, 2498, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2506, 2500, 2.0, 2505, (-2.0), 2399, A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2507, 2505, 2505, 1.0, 2500, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_173(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2508, 1.0, 2399, A::add(s.ad_value(2496), s.ad_value(2498)), 0.5);s.store_div_scaled_product_mixed_iia(2509, 2507, 2506, 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0);s.store_add(2495, 2495, 2509);s.store_exp(2510, 2509);s.store_div(2496, 2496, 2510);s.store_mul(2498, 2498, 2510);s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));s.store_div_scaled_product3_mixed_iiaa(2485, 2485, 2510, A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0);s.store_mul(2488, 2485, 2413);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_sqrt(2502, 2499);s.store_add_scaled_inputs_mixed_ia(2503, 2501, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_div_scaled_product_mixed_iiia(2512, 2413, 2399, 2498, 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0);s.store_add_scaled_product_indices(2513, 2512, 1.0, 2413, 2503, 1.0);s.store_mul3_lhs(2514, 2502, 2398, 2413);}
        s.b[2594] = (s.v[218] < 0.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2512, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2512, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_product3_indices(2454, 2512, 757, 2452, 2453, 1.0);s.store_add_scaled_product_indices(2515, 2514, 1.0, 775, 2512, 1.0);s.store_add_scaled_product_indices(2516, 2514, 1.0, 776, 2512, 1.0);s.store_mul(2517, 774, 2515);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2499), 1.0, A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2518, 2448, A::offset(s.ad_value(2456), 1.0), 2454);s.store_ln_ad(2519, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0), 1.0));s.store_mul(2029, 2512, 2458);s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2595] = (s.v[222] < 0.0);s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_174(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {s.store_offset_mul(2460, 222, 2459, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2521, 2393, 2460);s.store_mul(2520, 2500, 2413);}
        if s.b[2547] {s.copy_ad(1887, 2395);s.copy_ad(1888, 2413);s.copy_ad(1889, 2398);s.copy_ad(1890, 2417);s.copy_ad(1891, 2422);s.copy_ad(1892, 2451);s.copy_ad(1893, 2488);s.copy_ad(1894, 2494);s.copy_ad(1895, 2501);s.copy_ad(1896, 2503);s.copy_ad(1897, 2512);s.copy_ad(1898, 2513);s.copy_ad(1899, 2516);s.copy_ad(1900, 2518);s.copy_ad(1901, 2519);s.copy_ad(1902, 2521);s.copy_ad(1903, 2520);s.copy_ad(1932, 2414);}
        let (t3,) = {
    if s.b[2547] {
        (s.v[2435],)
    } else {
        (s.v[1933],)
    }
};
        s.store_scalar(1933, t3);
        if s.b[2547] {s.copy_ad(1934, 2495);s.copy_ad(1935, 2500);}
        if (!s.b[2547]) {s.copy_ad(745, 728);s.copy_ad(1887, 1822);s.copy_ad(1888, 1824);s.copy_ad(1889, 1826);s.copy_ad(1890, 1829);s.copy_ad(1891, 1830);s.copy_ad(1892, 1849);s.copy_ad(1893, 1860);s.copy_ad(1894, 1861);s.copy_ad(1895, 1863);s.copy_ad(1896, 1864);s.copy_ad(1897, 1865);s.copy_ad(1898, 1866);s.copy_ad(1899, 1868);s.copy_ad(1900, 1869);s.copy_ad(1901, 1871);s.copy_ad(1902, 1870);s.copy_ad(1903, 1872);s.copy_ad(1932, 1825);}
        let (t4,) = {
    if (!s.b[2547]) {
        (s.v[1833],)
    } else {
        (s.v[1933],)
    }
};
        s.store_scalar(1933, t4);
        if (!s.b[2547]) {s.copy_ad(1934, 1862);s.copy_ad(1935, 1931);}
        s.copy_ad(1904, 255);s.b[2596] = (s.v[773] > 0.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if s.b[2596] {s.store_div_scaled_value_offset_denominator(1904, s.ad_value(255), 1.0, A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);}
        s.store_scalar(1905, 1.0);s.store_scalar(1906, 1.0);s.store_scalar(1907, 0.0);s.store_scalar(1908, 1.0);s.store_scalar(1909, 1.0);s.copy_ad(2359, 1903);s.store_scalar(2362, 0.0);s.store_scalar(2361, 0.0);s.copy_ad(2363, 2359);s.b[2597] = (s.v[1890] > 0.0);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if s.b[2597] {s.store_mul_div_scaled_product_mixed_iaii(2354, 1901, A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), 1897, 1.0, 1898, 1.0);}
        s.b[2598] = (s.v[2354] > 0.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2598]) {s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));}
        if (s.b[2597] && (!s.b[2598])) {s.store_sub_from_scalar(1905, 1.0, 2354);}
        if s.b[2597] {s.store_mul(1906, 1900, 1905);s.store_div(1907, 1902, 1906);s.store_mul_ad_product_lhs_mixed_ai(2355, A::square(s.ad_value(1907)), 1893, 1893);}
        s.b[2599] = (s.v[0] == (-1.0));s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2599]) {s.store_div_scaled_value_offset_denominator(2355, s.ad_value(2355), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_175(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2597] {s.store_mul_scale_offset_mixed_ia(1908, 1906, A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div(2027, 1906, 1908);s.store_mul_scale_offset_mixed_ia(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1909, 2027, 1898, 1.0, 2356, 1.0);s.store_scaled_div(2357, 1893, 1909, 0.5);s.store_square(2358, 2357);s.store_add_product3_rhs_mixed_iia(2359, 1903, 1895, 1893, A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5);s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);}
        s.b[2600] = (p[49] == 1.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2600]) {s.store_scalar(2360, 0.0);s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);}
        if (s.b[2597] && (!s.b[2600])) {s.store_mul_scale_offset_mixed_ai(2360, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5)), 1905, -1.0, 1.0);s.store_add_scaled_products_mixed_aaia(2361, A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, 2360, A::offset(s.ad_value(1905), 1.0), 0.5);}
        if s.b[2597] {s.store_add_scaled_product_mixed_iia(2362, 2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0);s.store_sub(2363, 2359, 2362);}
        s.store_mul(851, 2359, 1904);s.store_mul_scale_offset_indices(853, 1904, 2361, -1.0, 0.0);s.store_mul_scale_offset_indices(852, 1904, 2363, -1.0, 0.0);s.store_scalar(2379, 0.0);s.store_scalar(2380, 0.0);s.store_scalar(2378, 0.0);s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if s.b[2601] {s.store_scalar(2368, 1.0);s.copy_ad(2367, 1887);}
        s.b[2602] = (s.v[272] > 1e-10);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2602]) {s.store_add_scaled_inputs3_indices(2364, 1887, 1.0, 270, (-1.0), 808, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2027, 2364, 0.5, 808, 0.5, A::add(A::square(A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)), 0.5);s.store_mul_add_scaled_inputs3_offset_rhs_indices(2028, 2027, 2027, 2.0, 808, (-1.0), 2364, -1.0, 0.0);s.store_div(2029, 808, 2027);s.store_mul(2365, 2364, 2029);s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));s.store_add_scaled_inputs3_mixed_aii(2367, A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, 2364, 1.0, 2365, -1.0);}
    }
}
