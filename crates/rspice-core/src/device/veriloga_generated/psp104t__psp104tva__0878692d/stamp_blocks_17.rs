#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2712] = (s.v[578] == 0.5);
        s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2712]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2712])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 575, 578);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add_scaled_inputs3_indices(2567, 822, 1.0, 551, 1.0, 2567, -1.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2713] = (s.v[631] == 0.5);
        s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2713]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(630)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 630, 631);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1907, 1907, 473);
        }

        s.b[2714] = (s.v[578] == 0.5);
        s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 575, 578);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.store_add_scaled_inputs3_indices(839, 840, (-1.0), 841, (-1.0), 842, (-1.0));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_scaled_products3(846, s.ad_value(647), s.ad_value(1902), 1.0, s.ad_value(648), s.ad_value(1903), 1.0, s.ad_value(649), s.ad_value(1904), 1.0);

        s.store_add_scaled_products3(847, s.ad_value(674), s.ad_value(1905), 1.0, s.ad_value(675), s.ad_value(1906), 1.0, s.ad_value(676), s.ad_value(1907), 1.0);

        s.b[2729] = (s.v[820] < 0.0);
        s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });

        if s.b[2729] {
            s.copy_ad(2728, 842);
            s.copy_ad(842, 839);
            s.copy_ad(839, 2728);
        }

        s.store_mul(849, 1888, 1879);

        s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));
        s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });

        s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });

        if (s.b[2762] && s.b[2767]) {
            s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);
        }

        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));
        s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });

        if s.b[2771] {
            s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);
            s.store_mul(1930, 760, 1916);
            s.store_mul(1930, 1848, 1861);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        var_chnl_type: f64,
        var_guard1735: f64,
        var_i_ds: f64,
        var_i_ds_db0: f64,
        var_i_ds_db1: f64,
        var_i_ds_db2: f64,
        var_i_ds_db3: f64,
        var_i_ds_db4: f64,
        var_i_ds_db5: f64,
        var_i_ds_db6: f64,
        var_i_ds_dn0: f64,
        var_i_ds_dn1: f64,
        var_i_ds_dn10: f64,
        var_i_ds_dn11: f64,
        var_i_ds_dn12: f64,
        var_i_ds_dn2: f64,
        var_i_ds_dn3: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_i_dsedge: f64,
        var_i_dsedge_db0: f64,
        var_i_dsedge_db1: f64,
        var_i_dsedge_db2: f64,
        var_i_dsedge_db3: f64,
        var_i_dsedge_db4: f64,
        var_i_dsedge_db5: f64,
        var_i_dsedge_db6: f64,
        var_i_dsedge_dn0: f64,
        var_i_dsedge_dn1: f64,
        var_i_dsedge_dn10: f64,
        var_i_dsedge_dn11: f64,
        var_i_dsedge_dn12: f64,
        var_i_dsedge_dn2: f64,
        var_i_dsedge_dn3: f64,
        var_i_dsedge_dn4: f64,
        var_i_dsedge_dn5: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_dsedge_dn9: f64,
        var_i_gb: f64,
        var_i_gb_db0: f64,
        var_i_gb_db1: f64,
        var_i_gb_db2: f64,
        var_i_gb_db3: f64,
        var_i_gb_db4: f64,
        var_i_gb_db5: f64,
        var_i_gb_db6: f64,
        var_i_gb_dn0: f64,
        var_i_gb_dn1: f64,
        var_i_gb_dn10: f64,
        var_i_gb_dn11: f64,
        var_i_gb_dn12: f64,
        var_i_gb_dn2: f64,
        var_i_gb_dn3: f64,
        var_i_gb_dn4: f64,
        var_i_gb_dn5: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gb_dn9: f64,
        var_i_gcd: f64,
        var_i_gcd_db0: f64,
        var_i_gcd_db1: f64,
        var_i_gcd_db2: f64,
        var_i_gcd_db3: f64,
        var_i_gcd_db4: f64,
        var_i_gcd_db5: f64,
        var_i_gcd_db6: f64,
        var_i_gcd_dn0: f64,
        var_i_gcd_dn1: f64,
        var_i_gcd_dn10: f64,
        var_i_gcd_dn11: f64,
        var_i_gcd_dn12: f64,
        var_i_gcd_dn2: f64,
        var_i_gcd_dn3: f64,
        var_i_gcd_dn4: f64,
        var_i_gcd_dn5: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcd_dn9: f64,
        var_i_gcs: f64,
        var_i_gcs_db0: f64,
        var_i_gcs_db1: f64,
        var_i_gcs_db2: f64,
        var_i_gcs_db3: f64,
        var_i_gcs_db4: f64,
        var_i_gcs_db5: f64,
        var_i_gcs_db6: f64,
        var_i_gcs_dn0: f64,
        var_i_gcs_dn1: f64,
        var_i_gcs_dn10: f64,
        var_i_gcs_dn11: f64,
        var_i_gcs_dn12: f64,
        var_i_gcs_dn2: f64,
        var_i_gcs_dn3: f64,
        var_i_gcs_dn4: f64,
        var_i_gcs_dn5: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gcs_dn9: f64,
        var_i_gisl: f64,
        var_i_gisl_db0: f64,
        var_i_gisl_db1: f64,
        var_i_gisl_db2: f64,
        var_i_gisl_db3: f64,
        var_i_gisl_db4: f64,
        var_i_gisl_db5: f64,
        var_i_gisl_db6: f64,
        var_i_gisl_dn0: f64,
        var_i_gisl_dn1: f64,
        var_i_gisl_dn10: f64,
        var_i_gisl_dn11: f64,
        var_i_gisl_dn12: f64,
        var_i_gisl_dn2: f64,
        var_i_gisl_dn3: f64,
        var_i_gisl_dn4: f64,
        var_i_gisl_dn5: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_i_gisl_dn9: f64,
        var_igdov: f64,
        var_igdov_db0: f64,
        var_igdov_db1: f64,
        var_igdov_db2: f64,
        var_igdov_db3: f64,
        var_igdov_db4: f64,
        var_igdov_db5: f64,
        var_igdov_db6: f64,
        var_igdov_dn0: f64,
        var_igdov_dn1: f64,
        var_igdov_dn10: f64,
        var_igdov_dn11: f64,
        var_igdov_dn12: f64,
        var_igdov_dn2: f64,
        var_igdov_dn3: f64,
        var_igdov_dn4: f64,
        var_igdov_dn5: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igdov_dn9: f64,
        var_igsov: f64,
        var_igsov_db0: f64,
        var_igsov_db1: f64,
        var_igsov_db2: f64,
        var_igsov_db3: f64,
        var_igsov_db4: f64,
        var_igsov_db5: f64,
        var_igsov_db6: f64,
        var_igsov_dn0: f64,
        var_igsov_dn1: f64,
        var_igsov_dn10: f64,
        var_igsov_dn11: f64,
        var_igsov_dn12: f64,
        var_igsov_dn2: f64,
        var_igsov_dn3: f64,
        var_igsov_dn4: f64,
        var_igsov_dn5: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_igsov_dn9: f64,
        var_iimpact: f64,
        var_iimpact_db0: f64,
        var_iimpact_db1: f64,
        var_iimpact_db2: f64,
        var_iimpact_db3: f64,
        var_iimpact_db4: f64,
        var_iimpact_db5: f64,
        var_iimpact_db6: f64,
        var_iimpact_dn0: f64,
        var_iimpact_dn1: f64,
        var_iimpact_dn10: f64,
        var_iimpact_dn11: f64,
        var_iimpact_dn12: f64,
        var_iimpact_dn2: f64,
        var_iimpact_dn3: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn5: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_mult_inst: f64,
    ) {
        let (eq0_e972, eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12, eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6,) = {
    if (var_guard1735 != 0.0) {
        let eq0_e966: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * var_iimpact);
        let eq0_e970_d_n0: f64 = (eq0_e968 * var_iimpact_dn0);
        let eq0_e970_d_n1: f64 = (eq0_e968 * var_iimpact_dn1);
        let eq0_e970_d_n2: f64 = (eq0_e968 * var_iimpact_dn2);
        let eq0_e970_d_n3: f64 = (eq0_e968 * var_iimpact_dn3);
        let eq0_e970_d_n4: f64 = (eq0_e968 * var_iimpact_dn4);
        let eq0_e970_d_n5: f64 = (eq0_e968 * var_iimpact_dn5);
        let eq0_e970_d_n6: f64 = (eq0_e968 * var_iimpact_dn6);
        let eq0_e970_d_n7: f64 = (eq0_e968 * var_iimpact_dn7);
        let eq0_e970_d_n8: f64 = (eq0_e968 * var_iimpact_dn8);
        let eq0_e970_d_n9: f64 = (eq0_e968 * var_iimpact_dn9);
        let eq0_e970_d_n10: f64 = (eq0_e968 * var_iimpact_dn10);
        let eq0_e970_d_n11: f64 = (eq0_e968 * var_iimpact_dn11);
        let eq0_e970_d_n12: f64 = (eq0_e968 * var_iimpact_dn12);
        let eq0_e970_d_b0: f64 = (eq0_e968 * var_iimpact_db0);
        let eq0_e970_d_b1: f64 = (eq0_e968 * var_iimpact_db1);
        let eq0_e970_d_b2: f64 = (eq0_e968 * var_iimpact_db2);
        let eq0_e970_d_b3: f64 = (eq0_e968 * var_iimpact_db3);
        let eq0_e970_d_b4: f64 = (eq0_e968 * var_iimpact_db4);
        let eq0_e970_d_b5: f64 = (eq0_e968 * var_iimpact_db5);
        let eq0_e970_d_b6: f64 = (eq0_e968 * var_iimpact_db6);
        (eq0_e970, eq0_e970_d_n0, eq0_e970_d_n1, eq0_e970_d_n2, eq0_e970_d_n3, eq0_e970_d_n4, eq0_e970_d_n5, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9, eq0_e970_d_n10, eq0_e970_d_n11, eq0_e970_d_n12, eq0_e970_d_b0, eq0_e970_d_b1, eq0_e970_d_b2, eq0_e970_d_b3, eq0_e970_d_b4, eq0_e970_d_b5, eq0_e970_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        let eq0_node_derivatives: [f64; 13] = [eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e984, eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12, eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6,) = {
    if (var_guard1735 != 0.0) {
        let eq1_e976: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e981: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e981_d_n0: f64 = (var_i_ds_dn0 + var_i_dsedge_dn0);
        let eq1_e981_d_n1: f64 = (var_i_ds_dn1 + var_i_dsedge_dn1);
        let eq1_e981_d_n2: f64 = (var_i_ds_dn2 + var_i_dsedge_dn2);
        let eq1_e981_d_n3: f64 = (var_i_ds_dn3 + var_i_dsedge_dn3);
        let eq1_e981_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq1_e981_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq1_e981_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e981_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e981_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e981_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq1_e981_d_n10: f64 = (var_i_ds_dn10 + var_i_dsedge_dn10);
        let eq1_e981_d_n11: f64 = (var_i_ds_dn11 + var_i_dsedge_dn11);
        let eq1_e981_d_n12: f64 = (var_i_ds_dn12 + var_i_dsedge_dn12);
        let eq1_e981_d_b0: f64 = (var_i_ds_db0 + var_i_dsedge_db0);
        let eq1_e981_d_b1: f64 = (var_i_ds_db1 + var_i_dsedge_db1);
        let eq1_e981_d_b2: f64 = (var_i_ds_db2 + var_i_dsedge_db2);
        let eq1_e981_d_b3: f64 = (var_i_ds_db3 + var_i_dsedge_db3);
        let eq1_e981_d_b4: f64 = (var_i_ds_db4 + var_i_dsedge_db4);
        let eq1_e981_d_b5: f64 = (var_i_ds_db5 + var_i_dsedge_db5);
        let eq1_e981_d_b6: f64 = (var_i_ds_db6 + var_i_dsedge_db6);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n0: f64 = (eq1_e978 * eq1_e981_d_n0);
        let eq1_e982_d_n1: f64 = (eq1_e978 * eq1_e981_d_n1);
        let eq1_e982_d_n2: f64 = (eq1_e978 * eq1_e981_d_n2);
        let eq1_e982_d_n3: f64 = (eq1_e978 * eq1_e981_d_n3);
        let eq1_e982_d_n4: f64 = (eq1_e978 * eq1_e981_d_n4);
        let eq1_e982_d_n5: f64 = (eq1_e978 * eq1_e981_d_n5);
        let eq1_e982_d_n6: f64 = (eq1_e978 * eq1_e981_d_n6);
        let eq1_e982_d_n7: f64 = (eq1_e978 * eq1_e981_d_n7);
        let eq1_e982_d_n8: f64 = (eq1_e978 * eq1_e981_d_n8);
        let eq1_e982_d_n9: f64 = (eq1_e978 * eq1_e981_d_n9);
        let eq1_e982_d_n10: f64 = (eq1_e978 * eq1_e981_d_n10);
        let eq1_e982_d_n11: f64 = (eq1_e978 * eq1_e981_d_n11);
        let eq1_e982_d_n12: f64 = (eq1_e978 * eq1_e981_d_n12);
        let eq1_e982_d_b0: f64 = (eq1_e978 * eq1_e981_d_b0);
        let eq1_e982_d_b1: f64 = (eq1_e978 * eq1_e981_d_b1);
        let eq1_e982_d_b2: f64 = (eq1_e978 * eq1_e981_d_b2);
        let eq1_e982_d_b3: f64 = (eq1_e978 * eq1_e981_d_b3);
        let eq1_e982_d_b4: f64 = (eq1_e978 * eq1_e981_d_b4);
        let eq1_e982_d_b5: f64 = (eq1_e978 * eq1_e981_d_b5);
        let eq1_e982_d_b6: f64 = (eq1_e978 * eq1_e981_d_b6);
        (eq1_e982, eq1_e982_d_n0, eq1_e982_d_n1, eq1_e982_d_n2, eq1_e982_d_n3, eq1_e982_d_n4, eq1_e982_d_n5, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9, eq1_e982_d_n10, eq1_e982_d_n11, eq1_e982_d_n12, eq1_e982_d_b0, eq1_e982_d_b1, eq1_e982_d_b2, eq1_e982_d_b3, eq1_e982_d_b4, eq1_e982_d_b5, eq1_e982_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        let eq1_node_derivatives: [f64; 13] = [eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12];
        let eq1_branch_derivatives: [f64; 7] = [eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e994, eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12, eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6,) = {
    if (var_guard1735 != 0.0) {
        let eq2_e988: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * var_i_gcs);
        let eq2_e992_d_n0: f64 = (eq2_e990 * var_i_gcs_dn0);
        let eq2_e992_d_n1: f64 = (eq2_e990 * var_i_gcs_dn1);
        let eq2_e992_d_n2: f64 = (eq2_e990 * var_i_gcs_dn2);
        let eq2_e992_d_n3: f64 = (eq2_e990 * var_i_gcs_dn3);
        let eq2_e992_d_n4: f64 = (eq2_e990 * var_i_gcs_dn4);
        let eq2_e992_d_n5: f64 = (eq2_e990 * var_i_gcs_dn5);
        let eq2_e992_d_n6: f64 = (eq2_e990 * var_i_gcs_dn6);
        let eq2_e992_d_n7: f64 = (eq2_e990 * var_i_gcs_dn7);
        let eq2_e992_d_n8: f64 = (eq2_e990 * var_i_gcs_dn8);
        let eq2_e992_d_n9: f64 = (eq2_e990 * var_i_gcs_dn9);
        let eq2_e992_d_n10: f64 = (eq2_e990 * var_i_gcs_dn10);
        let eq2_e992_d_n11: f64 = (eq2_e990 * var_i_gcs_dn11);
        let eq2_e992_d_n12: f64 = (eq2_e990 * var_i_gcs_dn12);
        let eq2_e992_d_b0: f64 = (eq2_e990 * var_i_gcs_db0);
        let eq2_e992_d_b1: f64 = (eq2_e990 * var_i_gcs_db1);
        let eq2_e992_d_b2: f64 = (eq2_e990 * var_i_gcs_db2);
        let eq2_e992_d_b3: f64 = (eq2_e990 * var_i_gcs_db3);
        let eq2_e992_d_b4: f64 = (eq2_e990 * var_i_gcs_db4);
        let eq2_e992_d_b5: f64 = (eq2_e990 * var_i_gcs_db5);
        let eq2_e992_d_b6: f64 = (eq2_e990 * var_i_gcs_db6);
        (eq2_e992, eq2_e992_d_n0, eq2_e992_d_n1, eq2_e992_d_n2, eq2_e992_d_n3, eq2_e992_d_n4, eq2_e992_d_n5, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9, eq2_e992_d_n10, eq2_e992_d_n11, eq2_e992_d_n12, eq2_e992_d_b0, eq2_e992_d_b1, eq2_e992_d_b2, eq2_e992_d_b3, eq2_e992_d_b4, eq2_e992_d_b5, eq2_e992_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        let eq2_node_derivatives: [f64; 13] = [eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1004, eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12, eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6,) = {
    if (var_guard1735 != 0.0) {
        let eq3_e998: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * var_i_gcd);
        let eq3_e1002_d_n0: f64 = (eq3_e1000 * var_i_gcd_dn0);
        let eq3_e1002_d_n1: f64 = (eq3_e1000 * var_i_gcd_dn1);
        let eq3_e1002_d_n2: f64 = (eq3_e1000 * var_i_gcd_dn2);
        let eq3_e1002_d_n3: f64 = (eq3_e1000 * var_i_gcd_dn3);
        let eq3_e1002_d_n4: f64 = (eq3_e1000 * var_i_gcd_dn4);
        let eq3_e1002_d_n5: f64 = (eq3_e1000 * var_i_gcd_dn5);
        let eq3_e1002_d_n6: f64 = (eq3_e1000 * var_i_gcd_dn6);
        let eq3_e1002_d_n7: f64 = (eq3_e1000 * var_i_gcd_dn7);
        let eq3_e1002_d_n8: f64 = (eq3_e1000 * var_i_gcd_dn8);
        let eq3_e1002_d_n9: f64 = (eq3_e1000 * var_i_gcd_dn9);
        let eq3_e1002_d_n10: f64 = (eq3_e1000 * var_i_gcd_dn10);
        let eq3_e1002_d_n11: f64 = (eq3_e1000 * var_i_gcd_dn11);
        let eq3_e1002_d_n12: f64 = (eq3_e1000 * var_i_gcd_dn12);
        let eq3_e1002_d_b0: f64 = (eq3_e1000 * var_i_gcd_db0);
        let eq3_e1002_d_b1: f64 = (eq3_e1000 * var_i_gcd_db1);
        let eq3_e1002_d_b2: f64 = (eq3_e1000 * var_i_gcd_db2);
        let eq3_e1002_d_b3: f64 = (eq3_e1000 * var_i_gcd_db3);
        let eq3_e1002_d_b4: f64 = (eq3_e1000 * var_i_gcd_db4);
        let eq3_e1002_d_b5: f64 = (eq3_e1000 * var_i_gcd_db5);
        let eq3_e1002_d_b6: f64 = (eq3_e1000 * var_i_gcd_db6);
        (eq3_e1002, eq3_e1002_d_n0, eq3_e1002_d_n1, eq3_e1002_d_n2, eq3_e1002_d_n3, eq3_e1002_d_n4, eq3_e1002_d_n5, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9, eq3_e1002_d_n10, eq3_e1002_d_n11, eq3_e1002_d_n12, eq3_e1002_d_b0, eq3_e1002_d_b1, eq3_e1002_d_b2, eq3_e1002_d_b3, eq3_e1002_d_b4, eq3_e1002_d_b5, eq3_e1002_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        let eq3_node_derivatives: [f64; 13] = [eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12];
        let eq3_branch_derivatives: [f64; 7] = [eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1015, eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12, eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6,) = {
    if (var_guard1735 == 0.0) {
        let eq4_e1009: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * var_iimpact);
        let eq4_e1013_d_n0: f64 = (eq4_e1011 * var_iimpact_dn0);
        let eq4_e1013_d_n1: f64 = (eq4_e1011 * var_iimpact_dn1);
        let eq4_e1013_d_n2: f64 = (eq4_e1011 * var_iimpact_dn2);
        let eq4_e1013_d_n3: f64 = (eq4_e1011 * var_iimpact_dn3);
        let eq4_e1013_d_n4: f64 = (eq4_e1011 * var_iimpact_dn4);
        let eq4_e1013_d_n5: f64 = (eq4_e1011 * var_iimpact_dn5);
        let eq4_e1013_d_n6: f64 = (eq4_e1011 * var_iimpact_dn6);
        let eq4_e1013_d_n7: f64 = (eq4_e1011 * var_iimpact_dn7);
        let eq4_e1013_d_n8: f64 = (eq4_e1011 * var_iimpact_dn8);
        let eq4_e1013_d_n9: f64 = (eq4_e1011 * var_iimpact_dn9);
        let eq4_e1013_d_n10: f64 = (eq4_e1011 * var_iimpact_dn10);
        let eq4_e1013_d_n11: f64 = (eq4_e1011 * var_iimpact_dn11);
        let eq4_e1013_d_n12: f64 = (eq4_e1011 * var_iimpact_dn12);
        let eq4_e1013_d_b0: f64 = (eq4_e1011 * var_iimpact_db0);
        let eq4_e1013_d_b1: f64 = (eq4_e1011 * var_iimpact_db1);
        let eq4_e1013_d_b2: f64 = (eq4_e1011 * var_iimpact_db2);
        let eq4_e1013_d_b3: f64 = (eq4_e1011 * var_iimpact_db3);
        let eq4_e1013_d_b4: f64 = (eq4_e1011 * var_iimpact_db4);
        let eq4_e1013_d_b5: f64 = (eq4_e1011 * var_iimpact_db5);
        let eq4_e1013_d_b6: f64 = (eq4_e1011 * var_iimpact_db6);
        (eq4_e1013, eq4_e1013_d_n0, eq4_e1013_d_n1, eq4_e1013_d_n2, eq4_e1013_d_n3, eq4_e1013_d_n4, eq4_e1013_d_n5, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9, eq4_e1013_d_n10, eq4_e1013_d_n11, eq4_e1013_d_n12, eq4_e1013_d_b0, eq4_e1013_d_b1, eq4_e1013_d_b2, eq4_e1013_d_b3, eq4_e1013_d_b4, eq4_e1013_d_b5, eq4_e1013_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        let eq4_node_derivatives: [f64; 13] = [eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1028, eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12, eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6,) = {
    if (var_guard1735 == 0.0) {
        let eq5_e1020: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1025: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1025_d_n0: f64 = (var_i_ds_dn0 + var_i_dsedge_dn0);
        let eq5_e1025_d_n1: f64 = (var_i_ds_dn1 + var_i_dsedge_dn1);
        let eq5_e1025_d_n2: f64 = (var_i_ds_dn2 + var_i_dsedge_dn2);
        let eq5_e1025_d_n3: f64 = (var_i_ds_dn3 + var_i_dsedge_dn3);
        let eq5_e1025_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq5_e1025_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq5_e1025_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1025_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1025_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1025_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq5_e1025_d_n10: f64 = (var_i_ds_dn10 + var_i_dsedge_dn10);
        let eq5_e1025_d_n11: f64 = (var_i_ds_dn11 + var_i_dsedge_dn11);
        let eq5_e1025_d_n12: f64 = (var_i_ds_dn12 + var_i_dsedge_dn12);
        let eq5_e1025_d_b0: f64 = (var_i_ds_db0 + var_i_dsedge_db0);
        let eq5_e1025_d_b1: f64 = (var_i_ds_db1 + var_i_dsedge_db1);
        let eq5_e1025_d_b2: f64 = (var_i_ds_db2 + var_i_dsedge_db2);
        let eq5_e1025_d_b3: f64 = (var_i_ds_db3 + var_i_dsedge_db3);
        let eq5_e1025_d_b4: f64 = (var_i_ds_db4 + var_i_dsedge_db4);
        let eq5_e1025_d_b5: f64 = (var_i_ds_db5 + var_i_dsedge_db5);
        let eq5_e1025_d_b6: f64 = (var_i_ds_db6 + var_i_dsedge_db6);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n0: f64 = (eq5_e1022 * eq5_e1025_d_n0);
        let eq5_e1026_d_n1: f64 = (eq5_e1022 * eq5_e1025_d_n1);
        let eq5_e1026_d_n2: f64 = (eq5_e1022 * eq5_e1025_d_n2);
        let eq5_e1026_d_n3: f64 = (eq5_e1022 * eq5_e1025_d_n3);
        let eq5_e1026_d_n4: f64 = (eq5_e1022 * eq5_e1025_d_n4);
        let eq5_e1026_d_n5: f64 = (eq5_e1022 * eq5_e1025_d_n5);
        let eq5_e1026_d_n6: f64 = (eq5_e1022 * eq5_e1025_d_n6);
        let eq5_e1026_d_n7: f64 = (eq5_e1022 * eq5_e1025_d_n7);
        let eq5_e1026_d_n8: f64 = (eq5_e1022 * eq5_e1025_d_n8);
        let eq5_e1026_d_n9: f64 = (eq5_e1022 * eq5_e1025_d_n9);
        let eq5_e1026_d_n10: f64 = (eq5_e1022 * eq5_e1025_d_n10);
        let eq5_e1026_d_n11: f64 = (eq5_e1022 * eq5_e1025_d_n11);
        let eq5_e1026_d_n12: f64 = (eq5_e1022 * eq5_e1025_d_n12);
        let eq5_e1026_d_b0: f64 = (eq5_e1022 * eq5_e1025_d_b0);
        let eq5_e1026_d_b1: f64 = (eq5_e1022 * eq5_e1025_d_b1);
        let eq5_e1026_d_b2: f64 = (eq5_e1022 * eq5_e1025_d_b2);
        let eq5_e1026_d_b3: f64 = (eq5_e1022 * eq5_e1025_d_b3);
        let eq5_e1026_d_b4: f64 = (eq5_e1022 * eq5_e1025_d_b4);
        let eq5_e1026_d_b5: f64 = (eq5_e1022 * eq5_e1025_d_b5);
        let eq5_e1026_d_b6: f64 = (eq5_e1022 * eq5_e1025_d_b6);
        (eq5_e1026, eq5_e1026_d_n0, eq5_e1026_d_n1, eq5_e1026_d_n2, eq5_e1026_d_n3, eq5_e1026_d_n4, eq5_e1026_d_n5, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9, eq5_e1026_d_n10, eq5_e1026_d_n11, eq5_e1026_d_n12, eq5_e1026_d_b0, eq5_e1026_d_b1, eq5_e1026_d_b2, eq5_e1026_d_b3, eq5_e1026_d_b4, eq5_e1026_d_b5, eq5_e1026_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        let eq5_node_derivatives: [f64; 13] = [eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12];
        let eq5_branch_derivatives: [f64; 7] = [eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1039, eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12, eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6,) = {
    if (var_guard1735 == 0.0) {
        let eq6_e1033: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * var_i_gcs);
        let eq6_e1037_d_n0: f64 = (eq6_e1035 * var_i_gcs_dn0);
        let eq6_e1037_d_n1: f64 = (eq6_e1035 * var_i_gcs_dn1);
        let eq6_e1037_d_n2: f64 = (eq6_e1035 * var_i_gcs_dn2);
        let eq6_e1037_d_n3: f64 = (eq6_e1035 * var_i_gcs_dn3);
        let eq6_e1037_d_n4: f64 = (eq6_e1035 * var_i_gcs_dn4);
        let eq6_e1037_d_n5: f64 = (eq6_e1035 * var_i_gcs_dn5);
        let eq6_e1037_d_n6: f64 = (eq6_e1035 * var_i_gcs_dn6);
        let eq6_e1037_d_n7: f64 = (eq6_e1035 * var_i_gcs_dn7);
        let eq6_e1037_d_n8: f64 = (eq6_e1035 * var_i_gcs_dn8);
        let eq6_e1037_d_n9: f64 = (eq6_e1035 * var_i_gcs_dn9);
        let eq6_e1037_d_n10: f64 = (eq6_e1035 * var_i_gcs_dn10);
        let eq6_e1037_d_n11: f64 = (eq6_e1035 * var_i_gcs_dn11);
        let eq6_e1037_d_n12: f64 = (eq6_e1035 * var_i_gcs_dn12);
        let eq6_e1037_d_b0: f64 = (eq6_e1035 * var_i_gcs_db0);
        let eq6_e1037_d_b1: f64 = (eq6_e1035 * var_i_gcs_db1);
        let eq6_e1037_d_b2: f64 = (eq6_e1035 * var_i_gcs_db2);
        let eq6_e1037_d_b3: f64 = (eq6_e1035 * var_i_gcs_db3);
        let eq6_e1037_d_b4: f64 = (eq6_e1035 * var_i_gcs_db4);
        let eq6_e1037_d_b5: f64 = (eq6_e1035 * var_i_gcs_db5);
        let eq6_e1037_d_b6: f64 = (eq6_e1035 * var_i_gcs_db6);
        (eq6_e1037, eq6_e1037_d_n0, eq6_e1037_d_n1, eq6_e1037_d_n2, eq6_e1037_d_n3, eq6_e1037_d_n4, eq6_e1037_d_n5, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9, eq6_e1037_d_n10, eq6_e1037_d_n11, eq6_e1037_d_n12, eq6_e1037_d_b0, eq6_e1037_d_b1, eq6_e1037_d_b2, eq6_e1037_d_b3, eq6_e1037_d_b4, eq6_e1037_d_b5, eq6_e1037_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        let eq6_node_derivatives: [f64; 13] = [eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12];
        let eq6_branch_derivatives: [f64; 7] = [eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1050, eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12, eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6,) = {
    if (var_guard1735 == 0.0) {
        let eq7_e1044: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * var_i_gcd);
        let eq7_e1048_d_n0: f64 = (eq7_e1046 * var_i_gcd_dn0);
        let eq7_e1048_d_n1: f64 = (eq7_e1046 * var_i_gcd_dn1);
        let eq7_e1048_d_n2: f64 = (eq7_e1046 * var_i_gcd_dn2);
        let eq7_e1048_d_n3: f64 = (eq7_e1046 * var_i_gcd_dn3);
        let eq7_e1048_d_n4: f64 = (eq7_e1046 * var_i_gcd_dn4);
        let eq7_e1048_d_n5: f64 = (eq7_e1046 * var_i_gcd_dn5);
        let eq7_e1048_d_n6: f64 = (eq7_e1046 * var_i_gcd_dn6);
        let eq7_e1048_d_n7: f64 = (eq7_e1046 * var_i_gcd_dn7);
        let eq7_e1048_d_n8: f64 = (eq7_e1046 * var_i_gcd_dn8);
        let eq7_e1048_d_n9: f64 = (eq7_e1046 * var_i_gcd_dn9);
        let eq7_e1048_d_n10: f64 = (eq7_e1046 * var_i_gcd_dn10);
        let eq7_e1048_d_n11: f64 = (eq7_e1046 * var_i_gcd_dn11);
        let eq7_e1048_d_n12: f64 = (eq7_e1046 * var_i_gcd_dn12);
        let eq7_e1048_d_b0: f64 = (eq7_e1046 * var_i_gcd_db0);
        let eq7_e1048_d_b1: f64 = (eq7_e1046 * var_i_gcd_db1);
        let eq7_e1048_d_b2: f64 = (eq7_e1046 * var_i_gcd_db2);
        let eq7_e1048_d_b3: f64 = (eq7_e1046 * var_i_gcd_db3);
        let eq7_e1048_d_b4: f64 = (eq7_e1046 * var_i_gcd_db4);
        let eq7_e1048_d_b5: f64 = (eq7_e1046 * var_i_gcd_db5);
        let eq7_e1048_d_b6: f64 = (eq7_e1046 * var_i_gcd_db6);
        (eq7_e1048, eq7_e1048_d_n0, eq7_e1048_d_n1, eq7_e1048_d_n2, eq7_e1048_d_n3, eq7_e1048_d_n4, eq7_e1048_d_n5, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9, eq7_e1048_d_n10, eq7_e1048_d_n11, eq7_e1048_d_n12, eq7_e1048_d_b0, eq7_e1048_d_b1, eq7_e1048_d_b2, eq7_e1048_d_b3, eq7_e1048_d_b4, eq7_e1048_d_b5, eq7_e1048_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        let eq7_node_derivatives: [f64; 13] = [eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12];
        let eq7_branch_derivatives: [f64; 7] = [eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1053: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * var_i_gb);
        let eq8_e1057_d_n0: f64 = (eq8_e1055 * var_i_gb_dn0);
        let eq8_e1057_d_n1: f64 = (eq8_e1055 * var_i_gb_dn1);
        let eq8_e1057_d_n2: f64 = (eq8_e1055 * var_i_gb_dn2);
        let eq8_e1057_d_n3: f64 = (eq8_e1055 * var_i_gb_dn3);
        let eq8_e1057_d_n4: f64 = (eq8_e1055 * var_i_gb_dn4);
        let eq8_e1057_d_n5: f64 = (eq8_e1055 * var_i_gb_dn5);
        let eq8_e1057_d_n6: f64 = (eq8_e1055 * var_i_gb_dn6);
        let eq8_e1057_d_n7: f64 = (eq8_e1055 * var_i_gb_dn7);
        let eq8_e1057_d_n8: f64 = (eq8_e1055 * var_i_gb_dn8);
        let eq8_e1057_d_n9: f64 = (eq8_e1055 * var_i_gb_dn9);
        let eq8_e1057_d_n10: f64 = (eq8_e1055 * var_i_gb_dn10);
        let eq8_e1057_d_n11: f64 = (eq8_e1055 * var_i_gb_dn11);
        let eq8_e1057_d_n12: f64 = (eq8_e1055 * var_i_gb_dn12);
        let eq8_e1057_d_b0: f64 = (eq8_e1055 * var_i_gb_db0);
        let eq8_e1057_d_b1: f64 = (eq8_e1055 * var_i_gb_db1);
        let eq8_e1057_d_b2: f64 = (eq8_e1055 * var_i_gb_db2);
        let eq8_e1057_d_b3: f64 = (eq8_e1055 * var_i_gb_db3);
        let eq8_e1057_d_b4: f64 = (eq8_e1055 * var_i_gb_db4);
        let eq8_e1057_d_b5: f64 = (eq8_e1055 * var_i_gb_db5);
        let eq8_e1057_d_b6: f64 = (eq8_e1055 * var_i_gb_db6);
        let eq8_value: f64 = eq8_e1057;
        let eq8_node_derivatives: [f64; 13] = [eq8_e1057_d_n0, eq8_e1057_d_n1, eq8_e1057_d_n2, eq8_e1057_d_n3, eq8_e1057_d_n4, eq8_e1057_d_n5, eq8_e1057_d_n6, eq8_e1057_d_n7, eq8_e1057_d_n8, eq8_e1057_d_n9, eq8_e1057_d_n10, eq8_e1057_d_n11, eq8_e1057_d_n12];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e1057_d_b0, eq8_e1057_d_b1, eq8_e1057_d_b2, eq8_e1057_d_b3, eq8_e1057_d_b4, eq8_e1057_d_b5, eq8_e1057_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1060: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * var_igsov);
        let eq9_e1064_d_n0: f64 = (eq9_e1062 * var_igsov_dn0);
        let eq9_e1064_d_n1: f64 = (eq9_e1062 * var_igsov_dn1);
        let eq9_e1064_d_n2: f64 = (eq9_e1062 * var_igsov_dn2);
        let eq9_e1064_d_n3: f64 = (eq9_e1062 * var_igsov_dn3);
        let eq9_e1064_d_n4: f64 = (eq9_e1062 * var_igsov_dn4);
        let eq9_e1064_d_n5: f64 = (eq9_e1062 * var_igsov_dn5);
        let eq9_e1064_d_n6: f64 = (eq9_e1062 * var_igsov_dn6);
        let eq9_e1064_d_n7: f64 = (eq9_e1062 * var_igsov_dn7);
        let eq9_e1064_d_n8: f64 = (eq9_e1062 * var_igsov_dn8);
        let eq9_e1064_d_n9: f64 = (eq9_e1062 * var_igsov_dn9);
        let eq9_e1064_d_n10: f64 = (eq9_e1062 * var_igsov_dn10);
        let eq9_e1064_d_n11: f64 = (eq9_e1062 * var_igsov_dn11);
        let eq9_e1064_d_n12: f64 = (eq9_e1062 * var_igsov_dn12);
        let eq9_e1064_d_b0: f64 = (eq9_e1062 * var_igsov_db0);
        let eq9_e1064_d_b1: f64 = (eq9_e1062 * var_igsov_db1);
        let eq9_e1064_d_b2: f64 = (eq9_e1062 * var_igsov_db2);
        let eq9_e1064_d_b3: f64 = (eq9_e1062 * var_igsov_db3);
        let eq9_e1064_d_b4: f64 = (eq9_e1062 * var_igsov_db4);
        let eq9_e1064_d_b5: f64 = (eq9_e1062 * var_igsov_db5);
        let eq9_e1064_d_b6: f64 = (eq9_e1062 * var_igsov_db6);
        let eq9_value: f64 = eq9_e1064;
        let eq9_node_derivatives: [f64; 13] = [eq9_e1064_d_n0, eq9_e1064_d_n1, eq9_e1064_d_n2, eq9_e1064_d_n3, eq9_e1064_d_n4, eq9_e1064_d_n5, eq9_e1064_d_n6, eq9_e1064_d_n7, eq9_e1064_d_n8, eq9_e1064_d_n9, eq9_e1064_d_n10, eq9_e1064_d_n11, eq9_e1064_d_n12];
        let eq9_branch_derivatives: [f64; 7] = [eq9_e1064_d_b0, eq9_e1064_d_b1, eq9_e1064_d_b2, eq9_e1064_d_b3, eq9_e1064_d_b4, eq9_e1064_d_b5, eq9_e1064_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1067: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * var_igdov);
        let eq10_e1071_d_n0: f64 = (eq10_e1069 * var_igdov_dn0);
        let eq10_e1071_d_n1: f64 = (eq10_e1069 * var_igdov_dn1);
        let eq10_e1071_d_n2: f64 = (eq10_e1069 * var_igdov_dn2);
        let eq10_e1071_d_n3: f64 = (eq10_e1069 * var_igdov_dn3);
        let eq10_e1071_d_n4: f64 = (eq10_e1069 * var_igdov_dn4);
        let eq10_e1071_d_n5: f64 = (eq10_e1069 * var_igdov_dn5);
        let eq10_e1071_d_n6: f64 = (eq10_e1069 * var_igdov_dn6);
        let eq10_e1071_d_n7: f64 = (eq10_e1069 * var_igdov_dn7);
        let eq10_e1071_d_n8: f64 = (eq10_e1069 * var_igdov_dn8);
        let eq10_e1071_d_n9: f64 = (eq10_e1069 * var_igdov_dn9);
        let eq10_e1071_d_n10: f64 = (eq10_e1069 * var_igdov_dn10);
        let eq10_e1071_d_n11: f64 = (eq10_e1069 * var_igdov_dn11);
        let eq10_e1071_d_n12: f64 = (eq10_e1069 * var_igdov_dn12);
        let eq10_e1071_d_b0: f64 = (eq10_e1069 * var_igdov_db0);
        let eq10_e1071_d_b1: f64 = (eq10_e1069 * var_igdov_db1);
        let eq10_e1071_d_b2: f64 = (eq10_e1069 * var_igdov_db2);
        let eq10_e1071_d_b3: f64 = (eq10_e1069 * var_igdov_db3);
        let eq10_e1071_d_b4: f64 = (eq10_e1069 * var_igdov_db4);
        let eq10_e1071_d_b5: f64 = (eq10_e1069 * var_igdov_db5);
        let eq10_e1071_d_b6: f64 = (eq10_e1069 * var_igdov_db6);
        let eq10_value: f64 = eq10_e1071;
        let eq10_node_derivatives: [f64; 13] = [eq10_e1071_d_n0, eq10_e1071_d_n1, eq10_e1071_d_n2, eq10_e1071_d_n3, eq10_e1071_d_n4, eq10_e1071_d_n5, eq10_e1071_d_n6, eq10_e1071_d_n7, eq10_e1071_d_n8, eq10_e1071_d_n9, eq10_e1071_d_n10, eq10_e1071_d_n11, eq10_e1071_d_n12];
        let eq10_branch_derivatives: [f64; 7] = [eq10_e1071_d_b0, eq10_e1071_d_b1, eq10_e1071_d_b2, eq10_e1071_d_b3, eq10_e1071_d_b4, eq10_e1071_d_b5, eq10_e1071_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1074: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * var_i_gisl);
        let eq11_e1078_d_n0: f64 = (eq11_e1076 * var_i_gisl_dn0);
        let eq11_e1078_d_n1: f64 = (eq11_e1076 * var_i_gisl_dn1);
        let eq11_e1078_d_n2: f64 = (eq11_e1076 * var_i_gisl_dn2);
        let eq11_e1078_d_n3: f64 = (eq11_e1076 * var_i_gisl_dn3);
        let eq11_e1078_d_n4: f64 = (eq11_e1076 * var_i_gisl_dn4);
        let eq11_e1078_d_n5: f64 = (eq11_e1076 * var_i_gisl_dn5);
        let eq11_e1078_d_n6: f64 = (eq11_e1076 * var_i_gisl_dn6);
        let eq11_e1078_d_n7: f64 = (eq11_e1076 * var_i_gisl_dn7);
        let eq11_e1078_d_n8: f64 = (eq11_e1076 * var_i_gisl_dn8);
        let eq11_e1078_d_n9: f64 = (eq11_e1076 * var_i_gisl_dn9);
        let eq11_e1078_d_n10: f64 = (eq11_e1076 * var_i_gisl_dn10);
        let eq11_e1078_d_n11: f64 = (eq11_e1076 * var_i_gisl_dn11);
        let eq11_e1078_d_n12: f64 = (eq11_e1076 * var_i_gisl_dn12);
        let eq11_e1078_d_b0: f64 = (eq11_e1076 * var_i_gisl_db0);
        let eq11_e1078_d_b1: f64 = (eq11_e1076 * var_i_gisl_db1);
        let eq11_e1078_d_b2: f64 = (eq11_e1076 * var_i_gisl_db2);
        let eq11_e1078_d_b3: f64 = (eq11_e1076 * var_i_gisl_db3);
        let eq11_e1078_d_b4: f64 = (eq11_e1076 * var_i_gisl_db4);
        let eq11_e1078_d_b5: f64 = (eq11_e1076 * var_i_gisl_db5);
        let eq11_e1078_d_b6: f64 = (eq11_e1076 * var_i_gisl_db6);
        let eq11_value: f64 = eq11_e1078;
        let eq11_node_derivatives: [f64; 13] = [eq11_e1078_d_n0, eq11_e1078_d_n1, eq11_e1078_d_n2, eq11_e1078_d_n3, eq11_e1078_d_n4, eq11_e1078_d_n5, eq11_e1078_d_n6, eq11_e1078_d_n7, eq11_e1078_d_n8, eq11_e1078_d_n9, eq11_e1078_d_n10, eq11_e1078_d_n11, eq11_e1078_d_n12];
        let eq11_branch_derivatives: [f64; 7] = [eq11_e1078_d_b0, eq11_e1078_d_b1, eq11_e1078_d_b2, eq11_e1078_d_b3, eq11_e1078_d_b4, eq11_e1078_d_b5, eq11_e1078_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
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
        var_chnl_type: f64,
        var_cth_i: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjund: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1736: f64,
        var_guard1737: f64,
        var_guard1738: f64,
        var_guard1739: f64,
        var_guard1740: f64,
        var_guard1741: f64,
        var_guard1742: f64,
        var_gwell: f64,
        var_i_gidl: f64,
        var_i_gidl_db0: f64,
        var_i_gidl_db1: f64,
        var_i_gidl_db2: f64,
        var_i_gidl_db3: f64,
        var_i_gidl_db4: f64,
        var_i_gidl_db5: f64,
        var_i_gidl_db6: f64,
        var_i_gidl_dn0: f64,
        var_i_gidl_dn1: f64,
        var_i_gidl_dn10: f64,
        var_i_gidl_dn11: f64,
        var_i_gidl_dn12: f64,
        var_i_gidl_dn2: f64,
        var_i_gidl_dn3: f64,
        var_i_gidl_dn4: f64,
        var_i_gidl_dn5: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gidl_dn9: f64,
        var_ijun_d: f64,
        var_ijun_d_db0: f64,
        var_ijun_d_db1: f64,
        var_ijun_d_db2: f64,
        var_ijun_d_db3: f64,
        var_ijun_d_db4: f64,
        var_ijun_d_db5: f64,
        var_ijun_d_db6: f64,
        var_ijun_d_dn0: f64,
        var_ijun_d_dn1: f64,
        var_ijun_d_dn10: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn12: f64,
        var_ijun_d_dn2: f64,
        var_ijun_d_dn3: f64,
        var_ijun_d_dn4: f64,
        var_ijun_d_dn5: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_d_dn9: f64,
        var_ijun_s: f64,
        var_ijun_s_db0: f64,
        var_ijun_s_db1: f64,
        var_ijun_s_db2: f64,
        var_ijun_s_db3: f64,
        var_ijun_s_db4: f64,
        var_ijun_s_db5: f64,
        var_ijun_s_db6: f64,
        var_ijun_s_dn0: f64,
        var_ijun_s_dn1: f64,
        var_ijun_s_dn10: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn12: f64,
        var_ijun_s_dn2: f64,
        var_ijun_s_dn3: f64,
        var_ijun_s_dn4: f64,
        var_ijun_s_dn5: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_ijun_s_dn9: f64,
        var_mult_inst: f64,
        var_pdiss_1: f64,
        var_pdiss_1_db0: f64,
        var_pdiss_1_db1: f64,
        var_pdiss_1_db2: f64,
        var_pdiss_1_db3: f64,
        var_pdiss_1_db4: f64,
        var_pdiss_1_db5: f64,
        var_pdiss_1_db6: f64,
        var_pdiss_1_dn0: f64,
        var_pdiss_1_dn1: f64,
        var_pdiss_1_dn10: f64,
        var_pdiss_1_dn11: f64,
        var_pdiss_1_dn12: f64,
        var_pdiss_1_dn2: f64,
        var_pdiss_1_dn3: f64,
        var_pdiss_1_dn4: f64,
        var_pdiss_1_dn5: f64,
        var_pdiss_1_dn6: f64,
        var_pdiss_1_dn7: f64,
        var_pdiss_1_dn8: f64,
        var_pdiss_1_dn9: f64,
        var_rth_t: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq12_e1081: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * var_i_gidl);
        let eq12_e1085_d_n0: f64 = (eq12_e1083 * var_i_gidl_dn0);
        let eq12_e1085_d_n1: f64 = (eq12_e1083 * var_i_gidl_dn1);
        let eq12_e1085_d_n2: f64 = (eq12_e1083 * var_i_gidl_dn2);
        let eq12_e1085_d_n3: f64 = (eq12_e1083 * var_i_gidl_dn3);
        let eq12_e1085_d_n4: f64 = (eq12_e1083 * var_i_gidl_dn4);
        let eq12_e1085_d_n5: f64 = (eq12_e1083 * var_i_gidl_dn5);
        let eq12_e1085_d_n6: f64 = (eq12_e1083 * var_i_gidl_dn6);
        let eq12_e1085_d_n7: f64 = (eq12_e1083 * var_i_gidl_dn7);
        let eq12_e1085_d_n8: f64 = (eq12_e1083 * var_i_gidl_dn8);
        let eq12_e1085_d_n9: f64 = (eq12_e1083 * var_i_gidl_dn9);
        let eq12_e1085_d_n10: f64 = (eq12_e1083 * var_i_gidl_dn10);
        let eq12_e1085_d_n11: f64 = (eq12_e1083 * var_i_gidl_dn11);
        let eq12_e1085_d_n12: f64 = (eq12_e1083 * var_i_gidl_dn12);
        let eq12_e1085_d_b0: f64 = (eq12_e1083 * var_i_gidl_db0);
        let eq12_e1085_d_b1: f64 = (eq12_e1083 * var_i_gidl_db1);
        let eq12_e1085_d_b2: f64 = (eq12_e1083 * var_i_gidl_db2);
        let eq12_e1085_d_b3: f64 = (eq12_e1083 * var_i_gidl_db3);
        let eq12_e1085_d_b4: f64 = (eq12_e1083 * var_i_gidl_db4);
        let eq12_e1085_d_b5: f64 = (eq12_e1083 * var_i_gidl_db5);
        let eq12_e1085_d_b6: f64 = (eq12_e1083 * var_i_gidl_db6);
        let eq12_value: f64 = eq12_e1085;
        let eq12_node_derivatives: [f64; 13] = [eq12_e1085_d_n0, eq12_e1085_d_n1, eq12_e1085_d_n2, eq12_e1085_d_n3, eq12_e1085_d_n4, eq12_e1085_d_n5, eq12_e1085_d_n6, eq12_e1085_d_n7, eq12_e1085_d_n8, eq12_e1085_d_n9, eq12_e1085_d_n10, eq12_e1085_d_n11, eq12_e1085_d_n12];
        let eq12_branch_derivatives: [f64; 7] = [eq12_e1085_d_b0, eq12_e1085_d_b1, eq12_e1085_d_b2, eq12_e1085_d_b3, eq12_e1085_d_b4, eq12_e1085_d_b5, eq12_e1085_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1088: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1090: f64 = (eq13_e1088 * p.p32);
        let eq13_e1092: f64 = (eq13_e1090 * var_ijun_s);
        let eq13_e1092_d_n0: f64 = (eq13_e1090 * var_ijun_s_dn0);
        let eq13_e1092_d_n1: f64 = (eq13_e1090 * var_ijun_s_dn1);
        let eq13_e1092_d_n2: f64 = (eq13_e1090 * var_ijun_s_dn2);
        let eq13_e1092_d_n3: f64 = (eq13_e1090 * var_ijun_s_dn3);
        let eq13_e1092_d_n4: f64 = (eq13_e1090 * var_ijun_s_dn4);
        let eq13_e1092_d_n5: f64 = (eq13_e1090 * var_ijun_s_dn5);
        let eq13_e1092_d_n6: f64 = (eq13_e1090 * var_ijun_s_dn6);
        let eq13_e1092_d_n7: f64 = (eq13_e1090 * var_ijun_s_dn7);
        let eq13_e1092_d_n8: f64 = (eq13_e1090 * var_ijun_s_dn8);
        let eq13_e1092_d_n9: f64 = (eq13_e1090 * var_ijun_s_dn9);
        let eq13_e1092_d_n10: f64 = (eq13_e1090 * var_ijun_s_dn10);
        let eq13_e1092_d_n11: f64 = (eq13_e1090 * var_ijun_s_dn11);
        let eq13_e1092_d_n12: f64 = (eq13_e1090 * var_ijun_s_dn12);
        let eq13_e1092_d_b0: f64 = (eq13_e1090 * var_ijun_s_db0);
        let eq13_e1092_d_b1: f64 = (eq13_e1090 * var_ijun_s_db1);
        let eq13_e1092_d_b2: f64 = (eq13_e1090 * var_ijun_s_db2);
        let eq13_e1092_d_b3: f64 = (eq13_e1090 * var_ijun_s_db3);
        let eq13_e1092_d_b4: f64 = (eq13_e1090 * var_ijun_s_db4);
        let eq13_e1092_d_b5: f64 = (eq13_e1090 * var_ijun_s_db5);
        let eq13_e1092_d_b6: f64 = (eq13_e1090 * var_ijun_s_db6);
        let eq13_value: f64 = eq13_e1092;
        let eq13_node_derivatives: [f64; 13] = [eq13_e1092_d_n0, eq13_e1092_d_n1, eq13_e1092_d_n2, eq13_e1092_d_n3, eq13_e1092_d_n4, eq13_e1092_d_n5, eq13_e1092_d_n6, eq13_e1092_d_n7, eq13_e1092_d_n8, eq13_e1092_d_n9, eq13_e1092_d_n10, eq13_e1092_d_n11, eq13_e1092_d_n12];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e1092_d_b0, eq13_e1092_d_b1, eq13_e1092_d_b2, eq13_e1092_d_b3, eq13_e1092_d_b4, eq13_e1092_d_b5, eq13_e1092_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1095: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1097: f64 = (eq14_e1095 * p.p32);
        let eq14_e1099: f64 = (eq14_e1097 * var_ijun_d);
        let eq14_e1099_d_n0: f64 = (eq14_e1097 * var_ijun_d_dn0);
        let eq14_e1099_d_n1: f64 = (eq14_e1097 * var_ijun_d_dn1);
        let eq14_e1099_d_n2: f64 = (eq14_e1097 * var_ijun_d_dn2);
        let eq14_e1099_d_n3: f64 = (eq14_e1097 * var_ijun_d_dn3);
        let eq14_e1099_d_n4: f64 = (eq14_e1097 * var_ijun_d_dn4);
        let eq14_e1099_d_n5: f64 = (eq14_e1097 * var_ijun_d_dn5);
        let eq14_e1099_d_n6: f64 = (eq14_e1097 * var_ijun_d_dn6);
        let eq14_e1099_d_n7: f64 = (eq14_e1097 * var_ijun_d_dn7);
        let eq14_e1099_d_n8: f64 = (eq14_e1097 * var_ijun_d_dn8);
        let eq14_e1099_d_n9: f64 = (eq14_e1097 * var_ijun_d_dn9);
        let eq14_e1099_d_n10: f64 = (eq14_e1097 * var_ijun_d_dn10);
        let eq14_e1099_d_n11: f64 = (eq14_e1097 * var_ijun_d_dn11);
        let eq14_e1099_d_n12: f64 = (eq14_e1097 * var_ijun_d_dn12);
        let eq14_e1099_d_b0: f64 = (eq14_e1097 * var_ijun_d_db0);
        let eq14_e1099_d_b1: f64 = (eq14_e1097 * var_ijun_d_db1);
        let eq14_e1099_d_b2: f64 = (eq14_e1097 * var_ijun_d_db2);
        let eq14_e1099_d_b3: f64 = (eq14_e1097 * var_ijun_d_db3);
        let eq14_e1099_d_b4: f64 = (eq14_e1097 * var_ijun_d_db4);
        let eq14_e1099_d_b5: f64 = (eq14_e1097 * var_ijun_d_db5);
        let eq14_e1099_d_b6: f64 = (eq14_e1097 * var_ijun_d_db6);
        let eq14_value: f64 = eq14_e1099;
        let eq14_node_derivatives: [f64; 13] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12];
        let eq14_branch_derivatives: [f64; 7] = [eq14_e1099_d_b0, eq14_e1099_d_b1, eq14_e1099_d_b2, eq14_e1099_d_b3, eq14_e1099_d_b4, eq14_e1099_d_b5, eq14_e1099_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1109, eq15_e1109_d_n1, eq15_e1109_d_n6,) = {
    if (var_guard1736 != 0.0) {
        let eq15_e1103: f64 = (var_mult_inst * p.p32);
        let eq15_e1105: f64 = (eq15_e1103 * var_ggate);
        let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        (eq15_e1107, eq15_e1105, (-eq15_e1105),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1109_d_n1),
            6,
            multiplicity * (eq15_e1109_d_n6),
        );
        let (eq17_e1124,) = {
    if (var_guard1736 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1134, eq18_e1134_d_n2, eq18_e1134_d_n7,) = {
    if (var_guard1737 != 0.0) {
        let eq18_e1128: f64 = (var_mult_inst * p.p32);
        let eq18_e1130: f64 = (eq18_e1128 * var_gsource);
        let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        (eq18_e1132, eq18_e1130, (-eq18_e1130),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1134_d_n2),
            7,
            multiplicity * (eq18_e1134_d_n7),
        );
        let (eq20_e1149,) = {
    if (var_guard1737 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n8,) = {
    if (var_guard1738 != 0.0) {
        let eq21_e1153: f64 = (var_mult_inst * p.p32);
        let eq21_e1155: f64 = (eq21_e1153 * var_gdrain);
        let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        (eq21_e1157, eq21_e1155, (-eq21_e1155),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1159_d_n0),
            8,
            multiplicity * (eq21_e1159_d_n8),
        );
        let (eq23_e1174,) = {
    if (var_guard1738 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1184, eq24_e1184_d_n9, eq24_e1184_d_n10,) = {
    if (var_guard1739 != 0.0) {
        let eq24_e1178: f64 = (var_mult_inst * p.p32);
        let eq24_e1180: f64 = (eq24_e1178 * var_gbulk);
        let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        (eq24_e1182, eq24_e1180, (-eq24_e1180),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            9,
            multiplicity * (eq24_e1184_d_n9),
            10,
            multiplicity * (eq24_e1184_d_n10),
        );
        let (eq26_e1199,) = {
    if (var_guard1739 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1209, eq27_e1209_d_n10, eq27_e1209_d_n11,) = {
    if (var_guard1740 != 0.0) {
        let eq27_e1203: f64 = (var_mult_inst * p.p32);
        let eq27_e1205: f64 = (eq27_e1203 * var_gjuns);
        let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        (eq27_e1207, (-eq27_e1205), eq27_e1205,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e1209_d_n10),
            11,
            multiplicity * (eq27_e1209_d_n11),
        );
        let (eq29_e1224,) = {
    if (var_guard1740 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1234, eq30_e1234_d_n10, eq30_e1234_d_n12,) = {
    if (var_guard1741 != 0.0) {
        let eq30_e1228: f64 = (var_mult_inst * p.p32);
        let eq30_e1230: f64 = (eq30_e1228 * var_gjund);
        let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        (eq30_e1232, (-eq30_e1230), eq30_e1230,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e1234_d_n10),
            12,
            multiplicity * (eq30_e1234_d_n12),
        );
        let (eq32_e1249,) = {
    if (var_guard1741 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1259, eq33_e1259_d_n3, eq33_e1259_d_n10,) = {
    if (var_guard1742 != 0.0) {
        let eq33_e1253: f64 = (var_mult_inst * p.p32);
        let eq33_e1255: f64 = (eq33_e1253 * var_gwell);
        let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        (eq33_e1257, eq33_e1255, (-eq33_e1255),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1259_d_n3),
            10,
            multiplicity * (eq33_e1259_d_n10),
        );
        let (eq35_e1274,) = {
    if (var_guard1742 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq38_e1286: f64 = (-var_mult_inst);
        let eq38_e1288: f64 = (eq38_e1286 * var_pdiss_1);
        let eq38_e1288_d_n0: f64 = (eq38_e1286 * var_pdiss_1_dn0);
        let eq38_e1288_d_n1: f64 = (eq38_e1286 * var_pdiss_1_dn1);
        let eq38_e1288_d_n2: f64 = (eq38_e1286 * var_pdiss_1_dn2);
        let eq38_e1288_d_n3: f64 = (eq38_e1286 * var_pdiss_1_dn3);
        let eq38_e1288_d_n4: f64 = (eq38_e1286 * var_pdiss_1_dn4);
        let eq38_e1288_d_n5: f64 = (eq38_e1286 * var_pdiss_1_dn5);
        let eq38_e1288_d_n6: f64 = (eq38_e1286 * var_pdiss_1_dn6);
        let eq38_e1288_d_n7: f64 = (eq38_e1286 * var_pdiss_1_dn7);
        let eq38_e1288_d_n8: f64 = (eq38_e1286 * var_pdiss_1_dn8);
        let eq38_e1288_d_n9: f64 = (eq38_e1286 * var_pdiss_1_dn9);
        let eq38_e1288_d_n10: f64 = (eq38_e1286 * var_pdiss_1_dn10);
        let eq38_e1288_d_n11: f64 = (eq38_e1286 * var_pdiss_1_dn11);
        let eq38_e1288_d_n12: f64 = (eq38_e1286 * var_pdiss_1_dn12);
        let eq38_e1288_d_b0: f64 = (eq38_e1286 * var_pdiss_1_db0);
        let eq38_e1288_d_b1: f64 = (eq38_e1286 * var_pdiss_1_db1);
        let eq38_e1288_d_b2: f64 = (eq38_e1286 * var_pdiss_1_db2);
        let eq38_e1288_d_b3: f64 = (eq38_e1286 * var_pdiss_1_db3);
        let eq38_e1288_d_b4: f64 = (eq38_e1286 * var_pdiss_1_db4);
        let eq38_e1288_d_b5: f64 = (eq38_e1286 * var_pdiss_1_db5);
        let eq38_e1288_d_b6: f64 = (eq38_e1286 * var_pdiss_1_db6);
        let eq38_value: f64 = eq38_e1288;
        let eq38_node_derivatives: [f64; 13] = [eq38_e1288_d_n0, eq38_e1288_d_n1, eq38_e1288_d_n2, eq38_e1288_d_n3, eq38_e1288_d_n4, eq38_e1288_d_n5, eq38_e1288_d_n6, eq38_e1288_d_n7, eq38_e1288_d_n8, eq38_e1288_d_n9, eq38_e1288_d_n10, eq38_e1288_d_n11, eq38_e1288_d_n12];
        let eq38_branch_derivatives: [f64; 7] = [eq38_e1288_d_b0, eq38_e1288_d_b1, eq38_e1288_d_b2, eq38_e1288_d_b3, eq38_e1288_d_b4, eq38_e1288_d_b5, eq38_e1288_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e1291: f64 = (var_mult_inst * var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);
        let eq39_value: f64 = eq39_e1294;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * ((eq39_e1291 * ddt_scale)),
        );
        let eq40_e1297: f64 = (var_mult_inst * (nv4 - 0.0));
        let __rspice_inv_cse_0: f64 = 1.0 / var_rth_t;
        let eq40_e1299: f64 = (eq40_e1297 * __rspice_inv_cse_0);
        let eq40_e1299_d_n4: f64 = (var_mult_inst * __rspice_inv_cse_0);
        let eq40_value: f64 = eq40_e1299;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1299_d_n4),
        );
        let eq41_e1302: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);
        let eq41_e1307_d_n0: f64 = ((eq41_e1304 * s.dn[840][0]) * ddt_scale);
        let eq41_e1307_d_n1: f64 = ((eq41_e1304 * s.dn[840][1]) * ddt_scale);
        let eq41_e1307_d_n2: f64 = ((eq41_e1304 * s.dn[840][2]) * ddt_scale);
        let eq41_e1307_d_n3: f64 = ((eq41_e1304 * s.dn[840][3]) * ddt_scale);
        let eq41_e1307_d_n4: f64 = ((eq41_e1304 * s.dn[840][4]) * ddt_scale);
        let eq41_e1307_d_n5: f64 = ((eq41_e1304 * s.dn[840][5]) * ddt_scale);
        let eq41_e1307_d_n6: f64 = ((eq41_e1304 * s.dn[840][6]) * ddt_scale);
        let eq41_e1307_d_n7: f64 = ((eq41_e1304 * s.dn[840][7]) * ddt_scale);
        let eq41_e1307_d_n8: f64 = ((eq41_e1304 * s.dn[840][8]) * ddt_scale);
        let eq41_e1307_d_n9: f64 = ((eq41_e1304 * s.dn[840][9]) * ddt_scale);
        let eq41_e1307_d_n10: f64 = ((eq41_e1304 * s.dn[840][10]) * ddt_scale);
        let eq41_e1307_d_n11: f64 = ((eq41_e1304 * s.dn[840][11]) * ddt_scale);
        let eq41_e1307_d_n12: f64 = ((eq41_e1304 * s.dn[840][12]) * ddt_scale);
        let eq41_e1307_d_b0: f64 = ((eq41_e1304 * s.db[840][0]) * ddt_scale);
        let eq41_e1307_d_b1: f64 = ((eq41_e1304 * s.db[840][1]) * ddt_scale);
        let eq41_e1307_d_b2: f64 = ((eq41_e1304 * s.db[840][2]) * ddt_scale);
        let eq41_e1307_d_b3: f64 = ((eq41_e1304 * s.db[840][3]) * ddt_scale);
        let eq41_e1307_d_b4: f64 = ((eq41_e1304 * s.db[840][4]) * ddt_scale);
        let eq41_e1307_d_b5: f64 = ((eq41_e1304 * s.db[840][5]) * ddt_scale);
        let eq41_e1307_d_b6: f64 = ((eq41_e1304 * s.db[840][6]) * ddt_scale);
        let eq41_value: f64 = eq41_e1307;
        let eq41_node_derivatives: [f64; 13] = [eq41_e1307_d_n0, eq41_e1307_d_n1, eq41_e1307_d_n2, eq41_e1307_d_n3, eq41_e1307_d_n4, eq41_e1307_d_n5, eq41_e1307_d_n6, eq41_e1307_d_n7, eq41_e1307_d_n8, eq41_e1307_d_n9, eq41_e1307_d_n10, eq41_e1307_d_n11, eq41_e1307_d_n12];
        let eq41_branch_derivatives: [f64; 7] = [eq41_e1307_d_b0, eq41_e1307_d_b1, eq41_e1307_d_b2, eq41_e1307_d_b3, eq41_e1307_d_b4, eq41_e1307_d_b5, eq41_e1307_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);
        let eq42_e1315_d_n0: f64 = ((eq42_e1312 * s.dn[841][0]) * ddt_scale);
        let eq42_e1315_d_n1: f64 = ((eq42_e1312 * s.dn[841][1]) * ddt_scale);
        let eq42_e1315_d_n2: f64 = ((eq42_e1312 * s.dn[841][2]) * ddt_scale);
        let eq42_e1315_d_n3: f64 = ((eq42_e1312 * s.dn[841][3]) * ddt_scale);
        let eq42_e1315_d_n4: f64 = ((eq42_e1312 * s.dn[841][4]) * ddt_scale);
        let eq42_e1315_d_n5: f64 = ((eq42_e1312 * s.dn[841][5]) * ddt_scale);
        let eq42_e1315_d_n6: f64 = ((eq42_e1312 * s.dn[841][6]) * ddt_scale);
        let eq42_e1315_d_n7: f64 = ((eq42_e1312 * s.dn[841][7]) * ddt_scale);
        let eq42_e1315_d_n8: f64 = ((eq42_e1312 * s.dn[841][8]) * ddt_scale);
        let eq42_e1315_d_n9: f64 = ((eq42_e1312 * s.dn[841][9]) * ddt_scale);
        let eq42_e1315_d_n10: f64 = ((eq42_e1312 * s.dn[841][10]) * ddt_scale);
        let eq42_e1315_d_n11: f64 = ((eq42_e1312 * s.dn[841][11]) * ddt_scale);
        let eq42_e1315_d_n12: f64 = ((eq42_e1312 * s.dn[841][12]) * ddt_scale);
        let eq42_e1315_d_b0: f64 = ((eq42_e1312 * s.db[841][0]) * ddt_scale);
        let eq42_e1315_d_b1: f64 = ((eq42_e1312 * s.db[841][1]) * ddt_scale);
        let eq42_e1315_d_b2: f64 = ((eq42_e1312 * s.db[841][2]) * ddt_scale);
        let eq42_e1315_d_b3: f64 = ((eq42_e1312 * s.db[841][3]) * ddt_scale);
        let eq42_e1315_d_b4: f64 = ((eq42_e1312 * s.db[841][4]) * ddt_scale);
        let eq42_e1315_d_b5: f64 = ((eq42_e1312 * s.db[841][5]) * ddt_scale);
        let eq42_e1315_d_b6: f64 = ((eq42_e1312 * s.db[841][6]) * ddt_scale);
        let eq42_value: f64 = eq42_e1315;
        let eq42_node_derivatives: [f64; 13] = [eq42_e1315_d_n0, eq42_e1315_d_n1, eq42_e1315_d_n2, eq42_e1315_d_n3, eq42_e1315_d_n4, eq42_e1315_d_n5, eq42_e1315_d_n6, eq42_e1315_d_n7, eq42_e1315_d_n8, eq42_e1315_d_n9, eq42_e1315_d_n10, eq42_e1315_d_n11, eq42_e1315_d_n12];
        let eq42_branch_derivatives: [f64; 7] = [eq42_e1315_d_b0, eq42_e1315_d_b1, eq42_e1315_d_b2, eq42_e1315_d_b3, eq42_e1315_d_b4, eq42_e1315_d_b5, eq42_e1315_d_b6];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);
        let eq43_e1323_d_n0: f64 = ((eq43_e1320 * s.dn[842][0]) * ddt_scale);
        let eq43_e1323_d_n1: f64 = ((eq43_e1320 * s.dn[842][1]) * ddt_scale);
        let eq43_e1323_d_n2: f64 = ((eq43_e1320 * s.dn[842][2]) * ddt_scale);
        let eq43_e1323_d_n3: f64 = ((eq43_e1320 * s.dn[842][3]) * ddt_scale);
        let eq43_e1323_d_n4: f64 = ((eq43_e1320 * s.dn[842][4]) * ddt_scale);
        let eq43_e1323_d_n5: f64 = ((eq43_e1320 * s.dn[842][5]) * ddt_scale);
        let eq43_e1323_d_n6: f64 = ((eq43_e1320 * s.dn[842][6]) * ddt_scale);
        let eq43_e1323_d_n7: f64 = ((eq43_e1320 * s.dn[842][7]) * ddt_scale);
        let eq43_e1323_d_n8: f64 = ((eq43_e1320 * s.dn[842][8]) * ddt_scale);
        let eq43_e1323_d_n9: f64 = ((eq43_e1320 * s.dn[842][9]) * ddt_scale);
        let eq43_e1323_d_n10: f64 = ((eq43_e1320 * s.dn[842][10]) * ddt_scale);
        let eq43_e1323_d_n11: f64 = ((eq43_e1320 * s.dn[842][11]) * ddt_scale);
        let eq43_e1323_d_n12: f64 = ((eq43_e1320 * s.dn[842][12]) * ddt_scale);
        let eq43_e1323_d_b0: f64 = ((eq43_e1320 * s.db[842][0]) * ddt_scale);
        let eq43_e1323_d_b1: f64 = ((eq43_e1320 * s.db[842][1]) * ddt_scale);
        let eq43_e1323_d_b2: f64 = ((eq43_e1320 * s.db[842][2]) * ddt_scale);
        let eq43_e1323_d_b3: f64 = ((eq43_e1320 * s.db[842][3]) * ddt_scale);
        let eq43_e1323_d_b4: f64 = ((eq43_e1320 * s.db[842][4]) * ddt_scale);
        let eq43_e1323_d_b5: f64 = ((eq43_e1320 * s.db[842][5]) * ddt_scale);
        let eq43_e1323_d_b6: f64 = ((eq43_e1320 * s.db[842][6]) * ddt_scale);
        let eq43_value: f64 = eq43_e1323;
        let eq43_node_derivatives: [f64; 13] = [eq43_e1323_d_n0, eq43_e1323_d_n1, eq43_e1323_d_n2, eq43_e1323_d_n3, eq43_e1323_d_n4, eq43_e1323_d_n5, eq43_e1323_d_n6, eq43_e1323_d_n7, eq43_e1323_d_n8, eq43_e1323_d_n9, eq43_e1323_d_n10, eq43_e1323_d_n11, eq43_e1323_d_n12];
        let eq43_branch_derivatives: [f64; 7] = [eq43_e1323_d_b0, eq43_e1323_d_b1, eq43_e1323_d_b2, eq43_e1323_d_b3, eq43_e1323_d_b4, eq43_e1323_d_b5, eq43_e1323_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
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
        var_chnl_type: f64,
        var_mult_inst: f64,
        var_qfgd: f64,
        var_qfgd_db0: f64,
        var_qfgd_db1: f64,
        var_qfgd_db2: f64,
        var_qfgd_db3: f64,
        var_qfgd_db4: f64,
        var_qfgd_db5: f64,
        var_qfgd_db6: f64,
        var_qfgd_dn0: f64,
        var_qfgd_dn1: f64,
        var_qfgd_dn10: f64,
        var_qfgd_dn11: f64,
        var_qfgd_dn12: f64,
        var_qfgd_dn2: f64,
        var_qfgd_dn3: f64,
        var_qfgd_dn4: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgd_dn9: f64,
        var_qfgs: f64,
        var_qfgs_db0: f64,
        var_qfgs_db1: f64,
        var_qfgs_db2: f64,
        var_qfgs_db3: f64,
        var_qfgs_db4: f64,
        var_qfgs_db5: f64,
        var_qfgs_db6: f64,
        var_qfgs_dn0: f64,
        var_qfgs_dn1: f64,
        var_qfgs_dn10: f64,
        var_qfgs_dn11: f64,
        var_qfgs_dn12: f64,
        var_qfgs_dn2: f64,
        var_qfgs_dn3: f64,
        var_qfgs_dn4: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qfgs_dn9: f64,
        var_qgb_ov: f64,
        var_qgb_ov_db0: f64,
        var_qgb_ov_db1: f64,
        var_qgb_ov_db2: f64,
        var_qgb_ov_db3: f64,
        var_qgb_ov_db4: f64,
        var_qgb_ov_db5: f64,
        var_qgb_ov_db6: f64,
        var_qgb_ov_dn0: f64,
        var_qgb_ov_dn1: f64,
        var_qgb_ov_dn10: f64,
        var_qgb_ov_dn11: f64,
        var_qgb_ov_dn12: f64,
        var_qgb_ov_dn2: f64,
        var_qgb_ov_dn3: f64,
        var_qgb_ov_dn4: f64,
        var_qgb_ov_dn5: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qgb_ov_dn9: f64,
    ) {
        let eq44_e1326: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * var_qfgs);
        let eq44_e1330_d_n0: f64 = (eq44_e1328 * var_qfgs_dn0);
        let eq44_e1330_d_n1: f64 = (eq44_e1328 * var_qfgs_dn1);
        let eq44_e1330_d_n2: f64 = (eq44_e1328 * var_qfgs_dn2);
        let eq44_e1330_d_n3: f64 = (eq44_e1328 * var_qfgs_dn3);
        let eq44_e1330_d_n4: f64 = (eq44_e1328 * var_qfgs_dn4);
        let eq44_e1330_d_n5: f64 = (eq44_e1328 * var_qfgs_dn5);
        let eq44_e1330_d_n6: f64 = (eq44_e1328 * var_qfgs_dn6);
        let eq44_e1330_d_n7: f64 = (eq44_e1328 * var_qfgs_dn7);
        let eq44_e1330_d_n8: f64 = (eq44_e1328 * var_qfgs_dn8);
        let eq44_e1330_d_n9: f64 = (eq44_e1328 * var_qfgs_dn9);
        let eq44_e1330_d_n10: f64 = (eq44_e1328 * var_qfgs_dn10);
        let eq44_e1330_d_n11: f64 = (eq44_e1328 * var_qfgs_dn11);
        let eq44_e1330_d_n12: f64 = (eq44_e1328 * var_qfgs_dn12);
        let eq44_e1330_d_b0: f64 = (eq44_e1328 * var_qfgs_db0);
        let eq44_e1330_d_b1: f64 = (eq44_e1328 * var_qfgs_db1);
        let eq44_e1330_d_b2: f64 = (eq44_e1328 * var_qfgs_db2);
        let eq44_e1330_d_b3: f64 = (eq44_e1328 * var_qfgs_db3);
        let eq44_e1330_d_b4: f64 = (eq44_e1328 * var_qfgs_db4);
        let eq44_e1330_d_b5: f64 = (eq44_e1328 * var_qfgs_db5);
        let eq44_e1330_d_b6: f64 = (eq44_e1328 * var_qfgs_db6);
        let eq44_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq44_e1330);
        let eq44_value: f64 = eq44_e1331;
        let eq44_node_derivatives: [f64; 13] = [(eq44_e1330_d_n0 * ddt_scale), (eq44_e1330_d_n1 * ddt_scale), (eq44_e1330_d_n2 * ddt_scale), (eq44_e1330_d_n3 * ddt_scale), (eq44_e1330_d_n4 * ddt_scale), (eq44_e1330_d_n5 * ddt_scale), (eq44_e1330_d_n6 * ddt_scale), (eq44_e1330_d_n7 * ddt_scale), (eq44_e1330_d_n8 * ddt_scale), (eq44_e1330_d_n9 * ddt_scale), (eq44_e1330_d_n10 * ddt_scale), (eq44_e1330_d_n11 * ddt_scale), (eq44_e1330_d_n12 * ddt_scale)];
        let eq44_branch_derivatives: [f64; 7] = [(eq44_e1330_d_b0 * ddt_scale), (eq44_e1330_d_b1 * ddt_scale), (eq44_e1330_d_b2 * ddt_scale), (eq44_e1330_d_b3 * ddt_scale), (eq44_e1330_d_b4 * ddt_scale), (eq44_e1330_d_b5 * ddt_scale), (eq44_e1330_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let eq45_e1334: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * var_qfgd);
        let eq45_e1338_d_n0: f64 = (eq45_e1336 * var_qfgd_dn0);
        let eq45_e1338_d_n1: f64 = (eq45_e1336 * var_qfgd_dn1);
        let eq45_e1338_d_n2: f64 = (eq45_e1336 * var_qfgd_dn2);
        let eq45_e1338_d_n3: f64 = (eq45_e1336 * var_qfgd_dn3);
        let eq45_e1338_d_n4: f64 = (eq45_e1336 * var_qfgd_dn4);
        let eq45_e1338_d_n5: f64 = (eq45_e1336 * var_qfgd_dn5);
        let eq45_e1338_d_n6: f64 = (eq45_e1336 * var_qfgd_dn6);
        let eq45_e1338_d_n7: f64 = (eq45_e1336 * var_qfgd_dn7);
        let eq45_e1338_d_n8: f64 = (eq45_e1336 * var_qfgd_dn8);
        let eq45_e1338_d_n9: f64 = (eq45_e1336 * var_qfgd_dn9);
        let eq45_e1338_d_n10: f64 = (eq45_e1336 * var_qfgd_dn10);
        let eq45_e1338_d_n11: f64 = (eq45_e1336 * var_qfgd_dn11);
        let eq45_e1338_d_n12: f64 = (eq45_e1336 * var_qfgd_dn12);
        let eq45_e1338_d_b0: f64 = (eq45_e1336 * var_qfgd_db0);
        let eq45_e1338_d_b1: f64 = (eq45_e1336 * var_qfgd_db1);
        let eq45_e1338_d_b2: f64 = (eq45_e1336 * var_qfgd_db2);
        let eq45_e1338_d_b3: f64 = (eq45_e1336 * var_qfgd_db3);
        let eq45_e1338_d_b4: f64 = (eq45_e1336 * var_qfgd_db4);
        let eq45_e1338_d_b5: f64 = (eq45_e1336 * var_qfgd_db5);
        let eq45_e1338_d_b6: f64 = (eq45_e1336 * var_qfgd_db6);
        let eq45_e1339: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq45_e1338);
        let eq45_value: f64 = eq45_e1339;
        let eq45_node_derivatives: [f64; 13] = [(eq45_e1338_d_n0 * ddt_scale), (eq45_e1338_d_n1 * ddt_scale), (eq45_e1338_d_n2 * ddt_scale), (eq45_e1338_d_n3 * ddt_scale), (eq45_e1338_d_n4 * ddt_scale), (eq45_e1338_d_n5 * ddt_scale), (eq45_e1338_d_n6 * ddt_scale), (eq45_e1338_d_n7 * ddt_scale), (eq45_e1338_d_n8 * ddt_scale), (eq45_e1338_d_n9 * ddt_scale), (eq45_e1338_d_n10 * ddt_scale), (eq45_e1338_d_n11 * ddt_scale), (eq45_e1338_d_n12 * ddt_scale)];
        let eq45_branch_derivatives: [f64; 7] = [(eq45_e1338_d_b0 * ddt_scale), (eq45_e1338_d_b1 * ddt_scale), (eq45_e1338_d_b2 * ddt_scale), (eq45_e1338_d_b3 * ddt_scale), (eq45_e1338_d_b4 * ddt_scale), (eq45_e1338_d_b5 * ddt_scale), (eq45_e1338_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (var_chnl_type * var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * var_qgb_ov);
        let eq46_e1346_d_n0: f64 = (eq46_e1344 * var_qgb_ov_dn0);
        let eq46_e1346_d_n1: f64 = (eq46_e1344 * var_qgb_ov_dn1);
        let eq46_e1346_d_n2: f64 = (eq46_e1344 * var_qgb_ov_dn2);
        let eq46_e1346_d_n3: f64 = (eq46_e1344 * var_qgb_ov_dn3);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * var_qgb_ov_dn4);
        let eq46_e1346_d_n5: f64 = (eq46_e1344 * var_qgb_ov_dn5);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * var_qgb_ov_dn9);
        let eq46_e1346_d_n10: f64 = (eq46_e1344 * var_qgb_ov_dn10);
        let eq46_e1346_d_n11: f64 = (eq46_e1344 * var_qgb_ov_dn11);
        let eq46_e1346_d_n12: f64 = (eq46_e1344 * var_qgb_ov_dn12);
        let eq46_e1346_d_b0: f64 = (eq46_e1344 * var_qgb_ov_db0);
        let eq46_e1346_d_b1: f64 = (eq46_e1344 * var_qgb_ov_db1);
        let eq46_e1346_d_b2: f64 = (eq46_e1344 * var_qgb_ov_db2);
        let eq46_e1346_d_b3: f64 = (eq46_e1344 * var_qgb_ov_db3);
        let eq46_e1346_d_b4: f64 = (eq46_e1344 * var_qgb_ov_db4);
        let eq46_e1346_d_b5: f64 = (eq46_e1344 * var_qgb_ov_db5);
        let eq46_e1346_d_b6: f64 = (eq46_e1344 * var_qgb_ov_db6);
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);
        let eq46_value: f64 = eq46_e1347;
        let eq46_node_derivatives: [f64; 13] = [(eq46_e1346_d_n0 * ddt_scale), (eq46_e1346_d_n1 * ddt_scale), (eq46_e1346_d_n2 * ddt_scale), (eq46_e1346_d_n3 * ddt_scale), (eq46_e1346_d_n4 * ddt_scale), (eq46_e1346_d_n5 * ddt_scale), (eq46_e1346_d_n6 * ddt_scale), (eq46_e1346_d_n7 * ddt_scale), (eq46_e1346_d_n8 * ddt_scale), (eq46_e1346_d_n9 * ddt_scale), (eq46_e1346_d_n10 * ddt_scale), (eq46_e1346_d_n11 * ddt_scale), (eq46_e1346_d_n12 * ddt_scale)];
        let eq46_branch_derivatives: [f64; 7] = [(eq46_e1346_d_b0 * ddt_scale), (eq46_e1346_d_b1 * ddt_scale), (eq46_e1346_d_b2 * ddt_scale), (eq46_e1346_d_b3 * ddt_scale), (eq46_e1346_d_b4 * ddt_scale), (eq46_e1346_d_b5 * ddt_scale), (eq46_e1346_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let eq47_e1350: f64 = (var_chnl_type * var_mult_inst);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq47_e1354);
        let eq47_e1355_d_n0: f64 = ((eq47_e1352 * s.dn[846][0]) * ddt_scale);
        let eq47_e1355_d_n1: f64 = ((eq47_e1352 * s.dn[846][1]) * ddt_scale);
        let eq47_e1355_d_n2: f64 = ((eq47_e1352 * s.dn[846][2]) * ddt_scale);
        let eq47_e1355_d_n3: f64 = ((eq47_e1352 * s.dn[846][3]) * ddt_scale);
        let eq47_e1355_d_n4: f64 = ((eq47_e1352 * s.dn[846][4]) * ddt_scale);
        let eq47_e1355_d_n5: f64 = ((eq47_e1352 * s.dn[846][5]) * ddt_scale);
        let eq47_e1355_d_n6: f64 = ((eq47_e1352 * s.dn[846][6]) * ddt_scale);
        let eq47_e1355_d_n7: f64 = ((eq47_e1352 * s.dn[846][7]) * ddt_scale);
        let eq47_e1355_d_n8: f64 = ((eq47_e1352 * s.dn[846][8]) * ddt_scale);
        let eq47_e1355_d_n9: f64 = ((eq47_e1352 * s.dn[846][9]) * ddt_scale);
        let eq47_e1355_d_n10: f64 = ((eq47_e1352 * s.dn[846][10]) * ddt_scale);
        let eq47_e1355_d_n11: f64 = ((eq47_e1352 * s.dn[846][11]) * ddt_scale);
        let eq47_e1355_d_n12: f64 = ((eq47_e1352 * s.dn[846][12]) * ddt_scale);
        let eq47_e1355_d_b0: f64 = ((eq47_e1352 * s.db[846][0]) * ddt_scale);
        let eq47_e1355_d_b1: f64 = ((eq47_e1352 * s.db[846][1]) * ddt_scale);
        let eq47_e1355_d_b2: f64 = ((eq47_e1352 * s.db[846][2]) * ddt_scale);
        let eq47_e1355_d_b3: f64 = ((eq47_e1352 * s.db[846][3]) * ddt_scale);
        let eq47_e1355_d_b4: f64 = ((eq47_e1352 * s.db[846][4]) * ddt_scale);
        let eq47_e1355_d_b5: f64 = ((eq47_e1352 * s.db[846][5]) * ddt_scale);
        let eq47_e1355_d_b6: f64 = ((eq47_e1352 * s.db[846][6]) * ddt_scale);
        let eq47_value: f64 = eq47_e1355;
        let eq47_node_derivatives: [f64; 13] = [eq47_e1355_d_n0, eq47_e1355_d_n1, eq47_e1355_d_n2, eq47_e1355_d_n3, eq47_e1355_d_n4, eq47_e1355_d_n5, eq47_e1355_d_n6, eq47_e1355_d_n7, eq47_e1355_d_n8, eq47_e1355_d_n9, eq47_e1355_d_n10, eq47_e1355_d_n11, eq47_e1355_d_n12];
        let eq47_branch_derivatives: [f64; 7] = [eq47_e1355_d_b0, eq47_e1355_d_b1, eq47_e1355_d_b2, eq47_e1355_d_b3, eq47_e1355_d_b4, eq47_e1355_d_b5, eq47_e1355_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (var_chnl_type * var_mult_inst);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1362);
        let eq48_e1363_d_n0: f64 = ((eq48_e1360 * s.dn[847][0]) * ddt_scale);
        let eq48_e1363_d_n1: f64 = ((eq48_e1360 * s.dn[847][1]) * ddt_scale);
        let eq48_e1363_d_n2: f64 = ((eq48_e1360 * s.dn[847][2]) * ddt_scale);
        let eq48_e1363_d_n3: f64 = ((eq48_e1360 * s.dn[847][3]) * ddt_scale);
        let eq48_e1363_d_n4: f64 = ((eq48_e1360 * s.dn[847][4]) * ddt_scale);
        let eq48_e1363_d_n5: f64 = ((eq48_e1360 * s.dn[847][5]) * ddt_scale);
        let eq48_e1363_d_n6: f64 = ((eq48_e1360 * s.dn[847][6]) * ddt_scale);
        let eq48_e1363_d_n7: f64 = ((eq48_e1360 * s.dn[847][7]) * ddt_scale);
        let eq48_e1363_d_n8: f64 = ((eq48_e1360 * s.dn[847][8]) * ddt_scale);
        let eq48_e1363_d_n9: f64 = ((eq48_e1360 * s.dn[847][9]) * ddt_scale);
        let eq48_e1363_d_n10: f64 = ((eq48_e1360 * s.dn[847][10]) * ddt_scale);
        let eq48_e1363_d_n11: f64 = ((eq48_e1360 * s.dn[847][11]) * ddt_scale);
        let eq48_e1363_d_n12: f64 = ((eq48_e1360 * s.dn[847][12]) * ddt_scale);
        let eq48_e1363_d_b0: f64 = ((eq48_e1360 * s.db[847][0]) * ddt_scale);
        let eq48_e1363_d_b1: f64 = ((eq48_e1360 * s.db[847][1]) * ddt_scale);
        let eq48_e1363_d_b2: f64 = ((eq48_e1360 * s.db[847][2]) * ddt_scale);
        let eq48_e1363_d_b3: f64 = ((eq48_e1360 * s.db[847][3]) * ddt_scale);
        let eq48_e1363_d_b4: f64 = ((eq48_e1360 * s.db[847][4]) * ddt_scale);
        let eq48_e1363_d_b5: f64 = ((eq48_e1360 * s.db[847][5]) * ddt_scale);
        let eq48_e1363_d_b6: f64 = ((eq48_e1360 * s.db[847][6]) * ddt_scale);
        let eq48_value: f64 = eq48_e1363;
        let eq48_node_derivatives: [f64; 13] = [eq48_e1363_d_n0, eq48_e1363_d_n1, eq48_e1363_d_n2, eq48_e1363_d_n3, eq48_e1363_d_n4, eq48_e1363_d_n5, eq48_e1363_d_n6, eq48_e1363_d_n7, eq48_e1363_d_n8, eq48_e1363_d_n9, eq48_e1363_d_n10, eq48_e1363_d_n11, eq48_e1363_d_n12];
        let eq48_branch_derivatives: [f64; 7] = [eq48_e1363_d_b0, eq48_e1363_d_b1, eq48_e1363_d_b2, eq48_e1363_d_b3, eq48_e1363_d_b4, eq48_e1363_d_b5, eq48_e1363_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
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
        var_mult_inst: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / s.v[848]);
        let eq50_e1371_d_n0: f64 = (-(((nv5 - 0.0) * s.dn[848][0]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n1: f64 = (-(((nv5 - 0.0) * s.dn[848][1]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n2: f64 = (-(((nv5 - 0.0) * s.dn[848][2]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n3: f64 = (-(((nv5 - 0.0) * s.dn[848][3]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * s.dn[848][4]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n5: f64 = ((s.v[848] - ((nv5 - 0.0) * s.dn[848][5])) / (s.v[848] * s.v[848]));
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * s.dn[848][6]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * s.dn[848][7]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * s.dn[848][8]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * s.dn[848][9]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n10: f64 = (-(((nv5 - 0.0) * s.dn[848][10]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n11: f64 = (-(((nv5 - 0.0) * s.dn[848][11]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n12: f64 = (-(((nv5 - 0.0) * s.dn[848][12]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b0: f64 = (-(((nv5 - 0.0) * s.db[848][0]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b1: f64 = (-(((nv5 - 0.0) * s.db[848][1]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b2: f64 = (-(((nv5 - 0.0) * s.db[848][2]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b3: f64 = (-(((nv5 - 0.0) * s.db[848][3]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b4: f64 = (-(((nv5 - 0.0) * s.db[848][4]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b5: f64 = (-(((nv5 - 0.0) * s.db[848][5]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b6: f64 = (-(((nv5 - 0.0) * s.db[848][6]) / (s.v[848] * s.v[848])));
        let eq50_value: f64 = eq50_e1371;
        let eq50_node_derivatives: [f64; 13] = [eq50_e1371_d_n0, eq50_e1371_d_n1, eq50_e1371_d_n2, eq50_e1371_d_n3, eq50_e1371_d_n4, eq50_e1371_d_n5, eq50_e1371_d_n6, eq50_e1371_d_n7, eq50_e1371_d_n8, eq50_e1371_d_n9, eq50_e1371_d_n10, eq50_e1371_d_n11, eq50_e1371_d_n12];
        let eq50_branch_derivatives: [f64; 7] = [eq50_e1371_d_b0, eq50_e1371_d_b1, eq50_e1371_d_b2, eq50_e1371_d_b3, eq50_e1371_d_b4, eq50_e1371_d_b5, eq50_e1371_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1374_d_b0: f64 = (s.db[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_b1: f64 = (s.db[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_b2: f64 = (s.db[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_b3: f64 = (s.db[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_b4: f64 = (s.db[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_b5: f64 = (s.db[849][5] * (nv5 - 0.0));
        let eq51_e1374_d_b6: f64 = (s.db[849][6] * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);
        let eq51_value: f64 = eq51_e1375;
        let eq51_node_derivatives: [f64; 13] = [(eq51_e1374_d_n0 * ddt_scale), (eq51_e1374_d_n1 * ddt_scale), (eq51_e1374_d_n2 * ddt_scale), (eq51_e1374_d_n3 * ddt_scale), (eq51_e1374_d_n4 * ddt_scale), (eq51_e1374_d_n5 * ddt_scale), (eq51_e1374_d_n6 * ddt_scale), (eq51_e1374_d_n7 * ddt_scale), (eq51_e1374_d_n8 * ddt_scale), (eq51_e1374_d_n9 * ddt_scale), (eq51_e1374_d_n10 * ddt_scale), (eq51_e1374_d_n11 * ddt_scale), (eq51_e1374_d_n12 * ddt_scale)];
        let eq51_branch_derivatives: [f64; 7] = [(eq51_e1374_d_b0 * ddt_scale), (eq51_e1374_d_b1 * ddt_scale), (eq51_e1374_d_b2 * ddt_scale), (eq51_e1374_d_b3 * ddt_scale), (eq51_e1374_d_b4 * ddt_scale), (eq51_e1374_d_b5 * ddt_scale), (eq51_e1374_d_b6 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = ((eq52_e1381 * s.dn[849][0]) * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = ((eq52_e1381 * s.dn[849][1]) * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = ((eq52_e1381 * s.dn[849][2]) * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = ((eq52_e1381 * s.dn[849][3]) * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = ((eq52_e1381 * s.dn[849][4]) * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = (((eq52_e1381 * s.dn[849][5]) * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = ((eq52_e1381 * s.dn[849][6]) * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = ((eq52_e1381 * s.dn[849][7]) * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = ((eq52_e1381 * s.dn[849][8]) * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = ((eq52_e1381 * s.dn[849][9]) * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = ((eq52_e1381 * s.dn[849][10]) * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = ((eq52_e1381 * s.dn[849][11]) * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = ((eq52_e1381 * s.dn[849][12]) * (nv5 - 0.0));
        let eq52_e1385_d_b0: f64 = ((eq52_e1381 * s.db[849][0]) * (nv5 - 0.0));
        let eq52_e1385_d_b1: f64 = ((eq52_e1381 * s.db[849][1]) * (nv5 - 0.0));
        let eq52_e1385_d_b2: f64 = ((eq52_e1381 * s.db[849][2]) * (nv5 - 0.0));
        let eq52_e1385_d_b3: f64 = ((eq52_e1381 * s.db[849][3]) * (nv5 - 0.0));
        let eq52_e1385_d_b4: f64 = ((eq52_e1381 * s.db[849][4]) * (nv5 - 0.0));
        let eq52_e1385_d_b5: f64 = ((eq52_e1381 * s.db[849][5]) * (nv5 - 0.0));
        let eq52_e1385_d_b6: f64 = ((eq52_e1381 * s.db[849][6]) * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n0: f64 = (-(eq52_e1385_d_n0 * ddt_scale));
        let eq52_e1387_d_n1: f64 = (-(eq52_e1385_d_n1 * ddt_scale));
        let eq52_e1387_d_n2: f64 = (-(eq52_e1385_d_n2 * ddt_scale));
        let eq52_e1387_d_n3: f64 = (-(eq52_e1385_d_n3 * ddt_scale));
        let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));
        let eq52_e1387_d_n5: f64 = (-(eq52_e1385_d_n5 * ddt_scale));
        let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));
        let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));
        let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));
        let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));
        let eq52_e1387_d_n10: f64 = (-(eq52_e1385_d_n10 * ddt_scale));
        let eq52_e1387_d_n11: f64 = (-(eq52_e1385_d_n11 * ddt_scale));
        let eq52_e1387_d_n12: f64 = (-(eq52_e1385_d_n12 * ddt_scale));
        let eq52_e1387_d_b0: f64 = (-(eq52_e1385_d_b0 * ddt_scale));
        let eq52_e1387_d_b1: f64 = (-(eq52_e1385_d_b1 * ddt_scale));
        let eq52_e1387_d_b2: f64 = (-(eq52_e1385_d_b2 * ddt_scale));
        let eq52_e1387_d_b3: f64 = (-(eq52_e1385_d_b3 * ddt_scale));
        let eq52_e1387_d_b4: f64 = (-(eq52_e1385_d_b4 * ddt_scale));
        let eq52_e1387_d_b5: f64 = (-(eq52_e1385_d_b5 * ddt_scale));
        let eq52_e1387_d_b6: f64 = (-(eq52_e1385_d_b6 * ddt_scale));
        let eq52_value: f64 = eq52_e1387;
        let eq52_node_derivatives: [f64; 13] = [eq52_e1387_d_n0, eq52_e1387_d_n1, eq52_e1387_d_n2, eq52_e1387_d_n3, eq52_e1387_d_n4, eq52_e1387_d_n5, eq52_e1387_d_n6, eq52_e1387_d_n7, eq52_e1387_d_n8, eq52_e1387_d_n9, eq52_e1387_d_n10, eq52_e1387_d_n11, eq52_e1387_d_n12];
        let eq52_branch_derivatives: [f64; 7] = [eq52_e1387_d_b0, eq52_e1387_d_b1, eq52_e1387_d_b2, eq52_e1387_d_b3, eq52_e1387_d_b4, eq52_e1387_d_b5, eq52_e1387_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = ((eq53_e1393 * s.dn[849][0]) * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = ((eq53_e1393 * s.dn[849][1]) * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = ((eq53_e1393 * s.dn[849][2]) * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = ((eq53_e1393 * s.dn[849][3]) * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = ((eq53_e1393 * s.dn[849][4]) * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = (((eq53_e1393 * s.dn[849][5]) * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = ((eq53_e1393 * s.dn[849][6]) * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = ((eq53_e1393 * s.dn[849][7]) * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = ((eq53_e1393 * s.dn[849][8]) * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = ((eq53_e1393 * s.dn[849][9]) * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = ((eq53_e1393 * s.dn[849][10]) * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = ((eq53_e1393 * s.dn[849][11]) * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = ((eq53_e1393 * s.dn[849][12]) * (nv5 - 0.0));
        let eq53_e1397_d_b0: f64 = ((eq53_e1393 * s.db[849][0]) * (nv5 - 0.0));
        let eq53_e1397_d_b1: f64 = ((eq53_e1393 * s.db[849][1]) * (nv5 - 0.0));
        let eq53_e1397_d_b2: f64 = ((eq53_e1393 * s.db[849][2]) * (nv5 - 0.0));
        let eq53_e1397_d_b3: f64 = ((eq53_e1393 * s.db[849][3]) * (nv5 - 0.0));
        let eq53_e1397_d_b4: f64 = ((eq53_e1393 * s.db[849][4]) * (nv5 - 0.0));
        let eq53_e1397_d_b5: f64 = ((eq53_e1393 * s.db[849][5]) * (nv5 - 0.0));
        let eq53_e1397_d_b6: f64 = ((eq53_e1393 * s.db[849][6]) * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n0: f64 = (-(eq53_e1397_d_n0 * ddt_scale));
        let eq53_e1399_d_n1: f64 = (-(eq53_e1397_d_n1 * ddt_scale));
        let eq53_e1399_d_n2: f64 = (-(eq53_e1397_d_n2 * ddt_scale));
        let eq53_e1399_d_n3: f64 = (-(eq53_e1397_d_n3 * ddt_scale));
        let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));
        let eq53_e1399_d_n5: f64 = (-(eq53_e1397_d_n5 * ddt_scale));
        let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));
        let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));
        let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));
        let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));
        let eq53_e1399_d_n10: f64 = (-(eq53_e1397_d_n10 * ddt_scale));
        let eq53_e1399_d_n11: f64 = (-(eq53_e1397_d_n11 * ddt_scale));
        let eq53_e1399_d_n12: f64 = (-(eq53_e1397_d_n12 * ddt_scale));
        let eq53_e1399_d_b0: f64 = (-(eq53_e1397_d_b0 * ddt_scale));
        let eq53_e1399_d_b1: f64 = (-(eq53_e1397_d_b1 * ddt_scale));
        let eq53_e1399_d_b2: f64 = (-(eq53_e1397_d_b2 * ddt_scale));
        let eq53_e1399_d_b3: f64 = (-(eq53_e1397_d_b3 * ddt_scale));
        let eq53_e1399_d_b4: f64 = (-(eq53_e1397_d_b4 * ddt_scale));
        let eq53_e1399_d_b5: f64 = (-(eq53_e1397_d_b5 * ddt_scale));
        let eq53_e1399_d_b6: f64 = (-(eq53_e1397_d_b6 * ddt_scale));
        let eq53_value: f64 = eq53_e1399;
        let eq53_node_derivatives: [f64; 13] = [eq53_e1399_d_n0, eq53_e1399_d_n1, eq53_e1399_d_n2, eq53_e1399_d_n3, eq53_e1399_d_n4, eq53_e1399_d_n5, eq53_e1399_d_n6, eq53_e1399_d_n7, eq53_e1399_d_n8, eq53_e1399_d_n9, eq53_e1399_d_n10, eq53_e1399_d_n11, eq53_e1399_d_n12];
        let eq53_branch_derivatives: [f64; 7] = [eq53_e1399_d_b0, eq53_e1399_d_b1, eq53_e1399_d_b2, eq53_e1399_d_b3, eq53_e1399_d_b4, eq53_e1399_d_b5, eq53_e1399_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let __rspice_deriv_cse_13: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let __rspice_deriv_cse_14: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let __rspice_deriv_cse_15: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let __rspice_deriv_cse_16: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let __rspice_deriv_cse_17: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let __rspice_deriv_cse_18: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let __rspice_deriv_cse_19: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1291_d_b0: f64 = ((s.db[15][0] * s.v[306]) + (s.v[15] * s.db[306][0]));
        let eq39_e1291_d_b1: f64 = ((s.db[15][1] * s.v[306]) + (s.v[15] * s.db[306][1]));
        let eq39_e1291_d_b2: f64 = ((s.db[15][2] * s.v[306]) + (s.v[15] * s.db[306][2]));
        let eq39_e1291_d_b3: f64 = ((s.db[15][3] * s.v[306]) + (s.v[15] * s.db[306][3]));
        let eq39_e1291_d_b4: f64 = ((s.db[15][4] * s.v[306]) + (s.v[15] * s.db[306][4]));
        let eq39_e1291_d_b5: f64 = ((s.db[15][5] * s.v[306]) + (s.v[15] * s.db[306][5]));
        let eq39_e1291_d_b6: f64 = ((s.db[15][6] * s.v[306]) + (s.v[15] * s.db[306][6]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1293_d_b0: f64 = (eq39_e1291_d_b0 * (nv4 - 0.0));
        let eq39_e1293_d_b1: f64 = (eq39_e1291_d_b1 * (nv4 - 0.0));
        let eq39_e1293_d_b2: f64 = (eq39_e1291_d_b2 * (nv4 - 0.0));
        let eq39_e1293_d_b3: f64 = (eq39_e1291_d_b3 * (nv4 - 0.0));
        let eq39_e1293_d_b4: f64 = (eq39_e1291_d_b4 * (nv4 - 0.0));
        let eq39_e1293_d_b5: f64 = (eq39_e1291_d_b5 * (nv4 - 0.0));
        let eq39_e1293_d_b6: f64 = (eq39_e1291_d_b6 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1293_d_n0, eq39_e1293_d_n1, eq39_e1293_d_n2, eq39_e1293_d_n3, eq39_e1293_d_n4, eq39_e1293_d_n5, eq39_e1293_d_n6, eq39_e1293_d_n7, eq39_e1293_d_n8, eq39_e1293_d_n9, eq39_e1293_d_n10, eq39_e1293_d_n11, eq39_e1293_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 7] = [eq39_e1293_d_b0, eq39_e1293_d_b1, eq39_e1293_d_b2, eq39_e1293_d_b3, eq39_e1293_d_b4, eq39_e1293_d_b5, eq39_e1293_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq41_e1304_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq41_e1304_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq41_e1304_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq41_e1304_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq41_e1304_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq41_e1304_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq41_e1304_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq41_e1304_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq41_e1304_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq41_e1304_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq41_e1304_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq41_e1304_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq41_e1304_d_b0: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq41_e1304_d_b1: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq41_e1304_d_b2: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq41_e1304_d_b3: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq41_e1304_d_b4: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq41_e1304_d_b5: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq41_e1304_d_b6: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1306_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[840]) + (eq41_e1304 * s.db[840][0]));
        let eq41_e1306_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[840]) + (eq41_e1304 * s.db[840][1]));
        let eq41_e1306_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[840]) + (eq41_e1304 * s.db[840][2]));
        let eq41_e1306_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[840]) + (eq41_e1304 * s.db[840][3]));
        let eq41_e1306_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[840]) + (eq41_e1304 * s.db[840][4]));
        let eq41_e1306_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[840]) + (eq41_e1304 * s.db[840][5]));
        let eq41_e1306_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[840]) + (eq41_e1304 * s.db[840][6]));
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1306_d_n0, eq41_e1306_d_n1, eq41_e1306_d_n2, eq41_e1306_d_n3, eq41_e1306_d_n4, eq41_e1306_d_n5, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, eq41_e1306_d_n10, eq41_e1306_d_n11, eq41_e1306_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 7] = [eq41_e1306_d_b0, eq41_e1306_d_b1, eq41_e1306_d_b2, eq41_e1306_d_b3, eq41_e1306_d_b4, eq41_e1306_d_b5, eq41_e1306_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1314_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[841]) + (eq42_e1312 * s.db[841][0]));
        let eq42_e1314_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[841]) + (eq42_e1312 * s.db[841][1]));
        let eq42_e1314_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[841]) + (eq42_e1312 * s.db[841][2]));
        let eq42_e1314_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[841]) + (eq42_e1312 * s.db[841][3]));
        let eq42_e1314_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[841]) + (eq42_e1312 * s.db[841][4]));
        let eq42_e1314_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[841]) + (eq42_e1312 * s.db[841][5]));
        let eq42_e1314_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[841]) + (eq42_e1312 * s.db[841][6]));
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [eq42_e1314_d_n0, eq42_e1314_d_n1, eq42_e1314_d_n2, eq42_e1314_d_n3, eq42_e1314_d_n4, eq42_e1314_d_n5, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, eq42_e1314_d_n10, eq42_e1314_d_n11, eq42_e1314_d_n12];
        let eq42_reactive_branch_derivatives: [f64; 7] = [eq42_e1314_d_b0, eq42_e1314_d_b1, eq42_e1314_d_b2, eq42_e1314_d_b3, eq42_e1314_d_b4, eq42_e1314_d_b5, eq42_e1314_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1322_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[842]) + (eq43_e1320 * s.db[842][0]));
        let eq43_e1322_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[842]) + (eq43_e1320 * s.db[842][1]));
        let eq43_e1322_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[842]) + (eq43_e1320 * s.db[842][2]));
        let eq43_e1322_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[842]) + (eq43_e1320 * s.db[842][3]));
        let eq43_e1322_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[842]) + (eq43_e1320 * s.db[842][4]));
        let eq43_e1322_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[842]) + (eq43_e1320 * s.db[842][5]));
        let eq43_e1322_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[842]) + (eq43_e1320 * s.db[842][6]));
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [eq43_e1322_d_n0, eq43_e1322_d_n1, eq43_e1322_d_n2, eq43_e1322_d_n3, eq43_e1322_d_n4, eq43_e1322_d_n5, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, eq43_e1322_d_n10, eq43_e1322_d_n11, eq43_e1322_d_n12];
        let eq43_reactive_branch_derivatives: [f64; 7] = [eq43_e1322_d_b0, eq43_e1322_d_b1, eq43_e1322_d_b2, eq43_e1322_d_b3, eq43_e1322_d_b4, eq43_e1322_d_b5, eq43_e1322_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1330_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[843]) + (eq44_e1328 * s.db[843][0]));
        let eq44_e1330_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[843]) + (eq44_e1328 * s.db[843][1]));
        let eq44_e1330_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[843]) + (eq44_e1328 * s.db[843][2]));
        let eq44_e1330_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[843]) + (eq44_e1328 * s.db[843][3]));
        let eq44_e1330_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[843]) + (eq44_e1328 * s.db[843][4]));
        let eq44_e1330_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[843]) + (eq44_e1328 * s.db[843][5]));
        let eq44_e1330_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[843]) + (eq44_e1328 * s.db[843][6]));
        let eq44_e1331_q: f64 = eq44_e1330;
        let eq44_reactive_node_derivatives: [f64; 13] = [eq44_e1330_d_n0, eq44_e1330_d_n1, eq44_e1330_d_n2, eq44_e1330_d_n3, eq44_e1330_d_n4, eq44_e1330_d_n5, eq44_e1330_d_n6, eq44_e1330_d_n7, eq44_e1330_d_n8, eq44_e1330_d_n9, eq44_e1330_d_n10, eq44_e1330_d_n11, eq44_e1330_d_n12];
        let eq44_reactive_branch_derivatives: [f64; 7] = [eq44_e1330_d_b0, eq44_e1330_d_b1, eq44_e1330_d_b2, eq44_e1330_d_b3, eq44_e1330_d_b4, eq44_e1330_d_b5, eq44_e1330_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1338_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[844]) + (eq45_e1336 * s.db[844][0]));
        let eq45_e1338_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[844]) + (eq45_e1336 * s.db[844][1]));
        let eq45_e1338_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[844]) + (eq45_e1336 * s.db[844][2]));
        let eq45_e1338_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[844]) + (eq45_e1336 * s.db[844][3]));
        let eq45_e1338_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[844]) + (eq45_e1336 * s.db[844][4]));
        let eq45_e1338_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[844]) + (eq45_e1336 * s.db[844][5]));
        let eq45_e1338_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[844]) + (eq45_e1336 * s.db[844][6]));
        let eq45_e1339_q: f64 = eq45_e1338;
        let eq45_reactive_node_derivatives: [f64; 13] = [eq45_e1338_d_n0, eq45_e1338_d_n1, eq45_e1338_d_n2, eq45_e1338_d_n3, eq45_e1338_d_n4, eq45_e1338_d_n5, eq45_e1338_d_n6, eq45_e1338_d_n7, eq45_e1338_d_n8, eq45_e1338_d_n9, eq45_e1338_d_n10, eq45_e1338_d_n11, eq45_e1338_d_n12];
        let eq45_reactive_branch_derivatives: [f64; 7] = [eq45_e1338_d_b0, eq45_e1338_d_b1, eq45_e1338_d_b2, eq45_e1338_d_b3, eq45_e1338_d_b4, eq45_e1338_d_b5, eq45_e1338_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1346_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[845]) + (eq46_e1344 * s.db[845][0]));
        let eq46_e1346_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[845]) + (eq46_e1344 * s.db[845][1]));
        let eq46_e1346_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[845]) + (eq46_e1344 * s.db[845][2]));
        let eq46_e1346_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[845]) + (eq46_e1344 * s.db[845][3]));
        let eq46_e1346_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[845]) + (eq46_e1344 * s.db[845][4]));
        let eq46_e1346_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[845]) + (eq46_e1344 * s.db[845][5]));
        let eq46_e1346_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[845]) + (eq46_e1344 * s.db[845][6]));
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [eq46_e1346_d_n0, eq46_e1346_d_n1, eq46_e1346_d_n2, eq46_e1346_d_n3, eq46_e1346_d_n4, eq46_e1346_d_n5, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, eq46_e1346_d_n10, eq46_e1346_d_n11, eq46_e1346_d_n12];
        let eq46_reactive_branch_derivatives: [f64; 7] = [eq46_e1346_d_b0, eq46_e1346_d_b1, eq46_e1346_d_b2, eq46_e1346_d_b3, eq46_e1346_d_b4, eq46_e1346_d_b5, eq46_e1346_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let __rspice_deriv_cse_0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let __rspice_deriv_cse_1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let __rspice_deriv_cse_2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let __rspice_deriv_cse_3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let __rspice_deriv_cse_4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let __rspice_deriv_cse_5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let __rspice_deriv_cse_6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let __rspice_deriv_cse_7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let __rspice_deriv_cse_8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let __rspice_deriv_cse_9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let __rspice_deriv_cse_10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let __rspice_deriv_cse_11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let __rspice_deriv_cse_12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let __rspice_deriv_cse_13: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let __rspice_deriv_cse_14: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let __rspice_deriv_cse_15: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let __rspice_deriv_cse_16: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let __rspice_deriv_cse_17: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let __rspice_deriv_cse_18: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let __rspice_deriv_cse_19: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq47_e1350: f64 = (s.v[0] * s.v[15]);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1352_d_n0: f64 = (__rspice_deriv_cse_0 * p.p33);
        let eq47_e1352_d_n1: f64 = (__rspice_deriv_cse_1 * p.p33);
        let eq47_e1352_d_n2: f64 = (__rspice_deriv_cse_2 * p.p33);
        let eq47_e1352_d_n3: f64 = (__rspice_deriv_cse_3 * p.p33);
        let eq47_e1352_d_n4: f64 = (__rspice_deriv_cse_4 * p.p33);
        let eq47_e1352_d_n5: f64 = (__rspice_deriv_cse_5 * p.p33);
        let eq47_e1352_d_n6: f64 = (__rspice_deriv_cse_6 * p.p33);
        let eq47_e1352_d_n7: f64 = (__rspice_deriv_cse_7 * p.p33);
        let eq47_e1352_d_n8: f64 = (__rspice_deriv_cse_8 * p.p33);
        let eq47_e1352_d_n9: f64 = (__rspice_deriv_cse_9 * p.p33);
        let eq47_e1352_d_n10: f64 = (__rspice_deriv_cse_10 * p.p33);
        let eq47_e1352_d_n11: f64 = (__rspice_deriv_cse_11 * p.p33);
        let eq47_e1352_d_n12: f64 = (__rspice_deriv_cse_12 * p.p33);
        let eq47_e1352_d_b0: f64 = (__rspice_deriv_cse_13 * p.p33);
        let eq47_e1352_d_b1: f64 = (__rspice_deriv_cse_14 * p.p33);
        let eq47_e1352_d_b2: f64 = (__rspice_deriv_cse_15 * p.p33);
        let eq47_e1352_d_b3: f64 = (__rspice_deriv_cse_16 * p.p33);
        let eq47_e1352_d_b4: f64 = (__rspice_deriv_cse_17 * p.p33);
        let eq47_e1352_d_b5: f64 = (__rspice_deriv_cse_18 * p.p33);
        let eq47_e1352_d_b6: f64 = (__rspice_deriv_cse_19 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1354_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[846]) + (eq47_e1352 * s.dn[846][0]));
        let eq47_e1354_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[846]) + (eq47_e1352 * s.dn[846][1]));
        let eq47_e1354_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[846]) + (eq47_e1352 * s.dn[846][2]));
        let eq47_e1354_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[846]) + (eq47_e1352 * s.dn[846][3]));
        let eq47_e1354_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[846]) + (eq47_e1352 * s.dn[846][4]));
        let eq47_e1354_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[846]) + (eq47_e1352 * s.dn[846][5]));
        let eq47_e1354_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[846]) + (eq47_e1352 * s.dn[846][6]));
        let eq47_e1354_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[846]) + (eq47_e1352 * s.dn[846][7]));
        let eq47_e1354_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[846]) + (eq47_e1352 * s.dn[846][8]));
        let eq47_e1354_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[846]) + (eq47_e1352 * s.dn[846][9]));
        let eq47_e1354_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[846]) + (eq47_e1352 * s.dn[846][10]));
        let eq47_e1354_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[846]) + (eq47_e1352 * s.dn[846][11]));
        let eq47_e1354_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[846]) + (eq47_e1352 * s.dn[846][12]));
        let eq47_e1354_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[846]) + (eq47_e1352 * s.db[846][0]));
        let eq47_e1354_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[846]) + (eq47_e1352 * s.db[846][1]));
        let eq47_e1354_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[846]) + (eq47_e1352 * s.db[846][2]));
        let eq47_e1354_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[846]) + (eq47_e1352 * s.db[846][3]));
        let eq47_e1354_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[846]) + (eq47_e1352 * s.db[846][4]));
        let eq47_e1354_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[846]) + (eq47_e1352 * s.db[846][5]));
        let eq47_e1354_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[846]) + (eq47_e1352 * s.db[846][6]));
        let eq47_e1355_q: f64 = eq47_e1354;
        let eq47_reactive_node_derivatives: [f64; 13] = [eq47_e1354_d_n0, eq47_e1354_d_n1, eq47_e1354_d_n2, eq47_e1354_d_n3, eq47_e1354_d_n4, eq47_e1354_d_n5, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, eq47_e1354_d_n10, eq47_e1354_d_n11, eq47_e1354_d_n12];
        let eq47_reactive_branch_derivatives: [f64; 7] = [eq47_e1354_d_b0, eq47_e1354_d_b1, eq47_e1354_d_b2, eq47_e1354_d_b3, eq47_e1354_d_b4, eq47_e1354_d_b5, eq47_e1354_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (s.v[0] * s.v[15]);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1362_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[847]) + (eq48_e1360 * s.dn[847][0]));
        let eq48_e1362_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[847]) + (eq48_e1360 * s.dn[847][1]));
        let eq48_e1362_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[847]) + (eq48_e1360 * s.dn[847][2]));
        let eq48_e1362_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[847]) + (eq48_e1360 * s.dn[847][3]));
        let eq48_e1362_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[847]) + (eq48_e1360 * s.dn[847][4]));
        let eq48_e1362_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[847]) + (eq48_e1360 * s.dn[847][5]));
        let eq48_e1362_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[847]) + (eq48_e1360 * s.dn[847][6]));
        let eq48_e1362_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[847]) + (eq48_e1360 * s.dn[847][7]));
        let eq48_e1362_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[847]) + (eq48_e1360 * s.dn[847][8]));
        let eq48_e1362_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[847]) + (eq48_e1360 * s.dn[847][9]));
        let eq48_e1362_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[847]) + (eq48_e1360 * s.dn[847][10]));
        let eq48_e1362_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[847]) + (eq48_e1360 * s.dn[847][11]));
        let eq48_e1362_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[847]) + (eq48_e1360 * s.dn[847][12]));
        let eq48_e1362_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[847]) + (eq48_e1360 * s.db[847][0]));
        let eq48_e1362_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[847]) + (eq48_e1360 * s.db[847][1]));
        let eq48_e1362_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[847]) + (eq48_e1360 * s.db[847][2]));
        let eq48_e1362_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[847]) + (eq48_e1360 * s.db[847][3]));
        let eq48_e1362_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[847]) + (eq48_e1360 * s.db[847][4]));
        let eq48_e1362_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[847]) + (eq48_e1360 * s.db[847][5]));
        let eq48_e1362_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[847]) + (eq48_e1360 * s.db[847][6]));
        let eq48_e1363_q: f64 = eq48_e1362;
        let eq48_reactive_node_derivatives: [f64; 13] = [eq48_e1362_d_n0, eq48_e1362_d_n1, eq48_e1362_d_n2, eq48_e1362_d_n3, eq48_e1362_d_n4, eq48_e1362_d_n5, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, eq48_e1362_d_n10, eq48_e1362_d_n11, eq48_e1362_d_n12];
        let eq48_reactive_branch_derivatives: [f64; 7] = [eq48_e1362_d_b0, eq48_e1362_d_b1, eq48_e1362_d_b2, eq48_e1362_d_b3, eq48_e1362_d_b4, eq48_e1362_d_b5, eq48_e1362_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1374_d_b0: f64 = (s.db[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_b1: f64 = (s.db[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_b2: f64 = (s.db[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_b3: f64 = (s.db[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_b4: f64 = (s.db[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_b5: f64 = (s.db[849][5] * (nv5 - 0.0));
        let eq51_e1374_d_b6: f64 = (s.db[849][6] * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [eq51_e1374_d_n0, eq51_e1374_d_n1, eq51_e1374_d_n2, eq51_e1374_d_n3, eq51_e1374_d_n4, eq51_e1374_d_n5, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, eq51_e1374_d_n10, eq51_e1374_d_n11, eq51_e1374_d_n12];
        let eq51_reactive_branch_derivatives: [f64; 7] = [eq51_e1374_d_b0, eq51_e1374_d_b1, eq51_e1374_d_b2, eq51_e1374_d_b3, eq51_e1374_d_b4, eq51_e1374_d_b5, eq51_e1374_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (s.v[15] * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let __rspice_inv_cse_0: f64 = 1.0 / (2.0 * eq52_e1379);
        let eq52_e1379_d_n0: f64 = ((s.dn[15][0] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n1: f64 = ((s.dn[15][1] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n2: f64 = ((s.dn[15][2] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n3: f64 = ((s.dn[15][3] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n4: f64 = ((s.dn[15][4] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n5: f64 = ((s.dn[15][5] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n6: f64 = ((s.dn[15][6] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n7: f64 = ((s.dn[15][7] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n8: f64 = ((s.dn[15][8] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n9: f64 = ((s.dn[15][9] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n10: f64 = ((s.dn[15][10] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n11: f64 = ((s.dn[15][11] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_n12: f64 = ((s.dn[15][12] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b0: f64 = ((s.db[15][0] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b1: f64 = ((s.db[15][1] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b2: f64 = ((s.db[15][2] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b3: f64 = ((s.db[15][3] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b4: f64 = ((s.db[15][4] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b5: f64 = ((s.db[15][5] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1379_d_b6: f64 = ((s.db[15][6] * p.p32) * __rspice_inv_cse_0);
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1381_d_n0: f64 = (eq52_e1379_d_n0 * 0.5);
        let eq52_e1381_d_n1: f64 = (eq52_e1379_d_n1 * 0.5);
        let eq52_e1381_d_n2: f64 = (eq52_e1379_d_n2 * 0.5);
        let eq52_e1381_d_n3: f64 = (eq52_e1379_d_n3 * 0.5);
        let eq52_e1381_d_n4: f64 = (eq52_e1379_d_n4 * 0.5);
        let eq52_e1381_d_n5: f64 = (eq52_e1379_d_n5 * 0.5);
        let eq52_e1381_d_n6: f64 = (eq52_e1379_d_n6 * 0.5);
        let eq52_e1381_d_n7: f64 = (eq52_e1379_d_n7 * 0.5);
        let eq52_e1381_d_n8: f64 = (eq52_e1379_d_n8 * 0.5);
        let eq52_e1381_d_n9: f64 = (eq52_e1379_d_n9 * 0.5);
        let eq52_e1381_d_n10: f64 = (eq52_e1379_d_n10 * 0.5);
        let eq52_e1381_d_n11: f64 = (eq52_e1379_d_n11 * 0.5);
        let eq52_e1381_d_n12: f64 = (eq52_e1379_d_n12 * 0.5);
        let eq52_e1381_d_b0: f64 = (eq52_e1379_d_b0 * 0.5);
        let eq52_e1381_d_b1: f64 = (eq52_e1379_d_b1 * 0.5);
        let eq52_e1381_d_b2: f64 = (eq52_e1379_d_b2 * 0.5);
        let eq52_e1381_d_b3: f64 = (eq52_e1379_d_b3 * 0.5);
        let eq52_e1381_d_b4: f64 = (eq52_e1379_d_b4 * 0.5);
        let eq52_e1381_d_b5: f64 = (eq52_e1379_d_b5 * 0.5);
        let eq52_e1381_d_b6: f64 = (eq52_e1379_d_b6 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1383_d_n0: f64 = ((eq52_e1381_d_n0 * s.v[849]) + (eq52_e1381 * s.dn[849][0]));
        let eq52_e1383_d_n1: f64 = ((eq52_e1381_d_n1 * s.v[849]) + (eq52_e1381 * s.dn[849][1]));
        let eq52_e1383_d_n2: f64 = ((eq52_e1381_d_n2 * s.v[849]) + (eq52_e1381 * s.dn[849][2]));
        let eq52_e1383_d_n3: f64 = ((eq52_e1381_d_n3 * s.v[849]) + (eq52_e1381 * s.dn[849][3]));
        let eq52_e1383_d_n4: f64 = ((eq52_e1381_d_n4 * s.v[849]) + (eq52_e1381 * s.dn[849][4]));
        let eq52_e1383_d_n5: f64 = ((eq52_e1381_d_n5 * s.v[849]) + (eq52_e1381 * s.dn[849][5]));
        let eq52_e1383_d_n6: f64 = ((eq52_e1381_d_n6 * s.v[849]) + (eq52_e1381 * s.dn[849][6]));
        let eq52_e1383_d_n7: f64 = ((eq52_e1381_d_n7 * s.v[849]) + (eq52_e1381 * s.dn[849][7]));
        let eq52_e1383_d_n8: f64 = ((eq52_e1381_d_n8 * s.v[849]) + (eq52_e1381 * s.dn[849][8]));
        let eq52_e1383_d_n9: f64 = ((eq52_e1381_d_n9 * s.v[849]) + (eq52_e1381 * s.dn[849][9]));
        let eq52_e1383_d_n10: f64 = ((eq52_e1381_d_n10 * s.v[849]) + (eq52_e1381 * s.dn[849][10]));
        let eq52_e1383_d_n11: f64 = ((eq52_e1381_d_n11 * s.v[849]) + (eq52_e1381 * s.dn[849][11]));
        let eq52_e1383_d_n12: f64 = ((eq52_e1381_d_n12 * s.v[849]) + (eq52_e1381 * s.dn[849][12]));
        let eq52_e1383_d_b0: f64 = ((eq52_e1381_d_b0 * s.v[849]) + (eq52_e1381 * s.db[849][0]));
        let eq52_e1383_d_b1: f64 = ((eq52_e1381_d_b1 * s.v[849]) + (eq52_e1381 * s.db[849][1]));
        let eq52_e1383_d_b2: f64 = ((eq52_e1381_d_b2 * s.v[849]) + (eq52_e1381 * s.db[849][2]));
        let eq52_e1383_d_b3: f64 = ((eq52_e1381_d_b3 * s.v[849]) + (eq52_e1381 * s.db[849][3]));
        let eq52_e1383_d_b4: f64 = ((eq52_e1381_d_b4 * s.v[849]) + (eq52_e1381 * s.db[849][4]));
        let eq52_e1383_d_b5: f64 = ((eq52_e1381_d_b5 * s.v[849]) + (eq52_e1381 * s.db[849][5]));
        let eq52_e1383_d_b6: f64 = ((eq52_e1381_d_b6 * s.v[849]) + (eq52_e1381 * s.db[849][6]));
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = (eq52_e1383_d_n0 * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = (eq52_e1383_d_n1 * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = (eq52_e1383_d_n2 * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = (eq52_e1383_d_n3 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = ((eq52_e1383_d_n5 * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = (eq52_e1383_d_n10 * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = (eq52_e1383_d_n11 * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = (eq52_e1383_d_n12 * (nv5 - 0.0));
        let eq52_e1385_d_b0: f64 = (eq52_e1383_d_b0 * (nv5 - 0.0));
        let eq52_e1385_d_b1: f64 = (eq52_e1383_d_b1 * (nv5 - 0.0));
        let eq52_e1385_d_b2: f64 = (eq52_e1383_d_b2 * (nv5 - 0.0));
        let eq52_e1385_d_b3: f64 = (eq52_e1383_d_b3 * (nv5 - 0.0));
        let eq52_e1385_d_b4: f64 = (eq52_e1383_d_b4 * (nv5 - 0.0));
        let eq52_e1385_d_b5: f64 = (eq52_e1383_d_b5 * (nv5 - 0.0));
        let eq52_e1385_d_b6: f64 = (eq52_e1383_d_b6 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_reactive_node_derivatives: [f64; 13] = [(-eq52_e1385_d_n0), (-eq52_e1385_d_n1), (-eq52_e1385_d_n2), (-eq52_e1385_d_n3), (-eq52_e1385_d_n4), (-eq52_e1385_d_n5), (-eq52_e1385_d_n6), (-eq52_e1385_d_n7), (-eq52_e1385_d_n8), (-eq52_e1385_d_n9), (-eq52_e1385_d_n10), (-eq52_e1385_d_n11), (-eq52_e1385_d_n12)];
        let eq52_reactive_branch_derivatives: [f64; 7] = [(-eq52_e1385_d_b0), (-eq52_e1385_d_b1), (-eq52_e1385_d_b2), (-eq52_e1385_d_b3), (-eq52_e1385_d_b4), (-eq52_e1385_d_b5), (-eq52_e1385_d_b6)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (s.v[15] * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let __rspice_inv_cse_1: f64 = 1.0 / (2.0 * eq53_e1391);
        let eq53_e1391_d_n0: f64 = ((s.dn[15][0] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n1: f64 = ((s.dn[15][1] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n2: f64 = ((s.dn[15][2] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n3: f64 = ((s.dn[15][3] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n4: f64 = ((s.dn[15][4] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n5: f64 = ((s.dn[15][5] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n6: f64 = ((s.dn[15][6] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n7: f64 = ((s.dn[15][7] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n8: f64 = ((s.dn[15][8] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n9: f64 = ((s.dn[15][9] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n10: f64 = ((s.dn[15][10] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n11: f64 = ((s.dn[15][11] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_n12: f64 = ((s.dn[15][12] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b0: f64 = ((s.db[15][0] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b1: f64 = ((s.db[15][1] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b2: f64 = ((s.db[15][2] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b3: f64 = ((s.db[15][3] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b4: f64 = ((s.db[15][4] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b5: f64 = ((s.db[15][5] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1391_d_b6: f64 = ((s.db[15][6] * p.p32) * __rspice_inv_cse_1);
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1393_d_n0: f64 = (eq53_e1391_d_n0 * 0.5);
        let eq53_e1393_d_n1: f64 = (eq53_e1391_d_n1 * 0.5);
        let eq53_e1393_d_n2: f64 = (eq53_e1391_d_n2 * 0.5);
        let eq53_e1393_d_n3: f64 = (eq53_e1391_d_n3 * 0.5);
        let eq53_e1393_d_n4: f64 = (eq53_e1391_d_n4 * 0.5);
        let eq53_e1393_d_n5: f64 = (eq53_e1391_d_n5 * 0.5);
        let eq53_e1393_d_n6: f64 = (eq53_e1391_d_n6 * 0.5);
        let eq53_e1393_d_n7: f64 = (eq53_e1391_d_n7 * 0.5);
        let eq53_e1393_d_n8: f64 = (eq53_e1391_d_n8 * 0.5);
        let eq53_e1393_d_n9: f64 = (eq53_e1391_d_n9 * 0.5);
        let eq53_e1393_d_n10: f64 = (eq53_e1391_d_n10 * 0.5);
        let eq53_e1393_d_n11: f64 = (eq53_e1391_d_n11 * 0.5);
        let eq53_e1393_d_n12: f64 = (eq53_e1391_d_n12 * 0.5);
        let eq53_e1393_d_b0: f64 = (eq53_e1391_d_b0 * 0.5);
        let eq53_e1393_d_b1: f64 = (eq53_e1391_d_b1 * 0.5);
        let eq53_e1393_d_b2: f64 = (eq53_e1391_d_b2 * 0.5);
        let eq53_e1393_d_b3: f64 = (eq53_e1391_d_b3 * 0.5);
        let eq53_e1393_d_b4: f64 = (eq53_e1391_d_b4 * 0.5);
        let eq53_e1393_d_b5: f64 = (eq53_e1391_d_b5 * 0.5);
        let eq53_e1393_d_b6: f64 = (eq53_e1391_d_b6 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1395_d_n0: f64 = ((eq53_e1393_d_n0 * s.v[849]) + (eq53_e1393 * s.dn[849][0]));
        let eq53_e1395_d_n1: f64 = ((eq53_e1393_d_n1 * s.v[849]) + (eq53_e1393 * s.dn[849][1]));
        let eq53_e1395_d_n2: f64 = ((eq53_e1393_d_n2 * s.v[849]) + (eq53_e1393 * s.dn[849][2]));
        let eq53_e1395_d_n3: f64 = ((eq53_e1393_d_n3 * s.v[849]) + (eq53_e1393 * s.dn[849][3]));
        let eq53_e1395_d_n4: f64 = ((eq53_e1393_d_n4 * s.v[849]) + (eq53_e1393 * s.dn[849][4]));
        let eq53_e1395_d_n5: f64 = ((eq53_e1393_d_n5 * s.v[849]) + (eq53_e1393 * s.dn[849][5]));
        let eq53_e1395_d_n6: f64 = ((eq53_e1393_d_n6 * s.v[849]) + (eq53_e1393 * s.dn[849][6]));
        let eq53_e1395_d_n7: f64 = ((eq53_e1393_d_n7 * s.v[849]) + (eq53_e1393 * s.dn[849][7]));
        let eq53_e1395_d_n8: f64 = ((eq53_e1393_d_n8 * s.v[849]) + (eq53_e1393 * s.dn[849][8]));
        let eq53_e1395_d_n9: f64 = ((eq53_e1393_d_n9 * s.v[849]) + (eq53_e1393 * s.dn[849][9]));
        let eq53_e1395_d_n10: f64 = ((eq53_e1393_d_n10 * s.v[849]) + (eq53_e1393 * s.dn[849][10]));
        let eq53_e1395_d_n11: f64 = ((eq53_e1393_d_n11 * s.v[849]) + (eq53_e1393 * s.dn[849][11]));
        let eq53_e1395_d_n12: f64 = ((eq53_e1393_d_n12 * s.v[849]) + (eq53_e1393 * s.dn[849][12]));
        let eq53_e1395_d_b0: f64 = ((eq53_e1393_d_b0 * s.v[849]) + (eq53_e1393 * s.db[849][0]));
        let eq53_e1395_d_b1: f64 = ((eq53_e1393_d_b1 * s.v[849]) + (eq53_e1393 * s.db[849][1]));
        let eq53_e1395_d_b2: f64 = ((eq53_e1393_d_b2 * s.v[849]) + (eq53_e1393 * s.db[849][2]));
        let eq53_e1395_d_b3: f64 = ((eq53_e1393_d_b3 * s.v[849]) + (eq53_e1393 * s.db[849][3]));
        let eq53_e1395_d_b4: f64 = ((eq53_e1393_d_b4 * s.v[849]) + (eq53_e1393 * s.db[849][4]));
        let eq53_e1395_d_b5: f64 = ((eq53_e1393_d_b5 * s.v[849]) + (eq53_e1393 * s.db[849][5]));
        let eq53_e1395_d_b6: f64 = ((eq53_e1393_d_b6 * s.v[849]) + (eq53_e1393 * s.db[849][6]));
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = (eq53_e1395_d_n0 * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = (eq53_e1395_d_n1 * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = (eq53_e1395_d_n2 * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = (eq53_e1395_d_n3 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = ((eq53_e1395_d_n5 * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = (eq53_e1395_d_n10 * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = (eq53_e1395_d_n11 * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = (eq53_e1395_d_n12 * (nv5 - 0.0));
        let eq53_e1397_d_b0: f64 = (eq53_e1395_d_b0 * (nv5 - 0.0));
        let eq53_e1397_d_b1: f64 = (eq53_e1395_d_b1 * (nv5 - 0.0));
        let eq53_e1397_d_b2: f64 = (eq53_e1395_d_b2 * (nv5 - 0.0));
        let eq53_e1397_d_b3: f64 = (eq53_e1395_d_b3 * (nv5 - 0.0));
        let eq53_e1397_d_b4: f64 = (eq53_e1395_d_b4 * (nv5 - 0.0));
        let eq53_e1397_d_b5: f64 = (eq53_e1395_d_b5 * (nv5 - 0.0));
        let eq53_e1397_d_b6: f64 = (eq53_e1395_d_b6 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_reactive_node_derivatives: [f64; 13] = [(-eq53_e1397_d_n0), (-eq53_e1397_d_n1), (-eq53_e1397_d_n2), (-eq53_e1397_d_n3), (-eq53_e1397_d_n4), (-eq53_e1397_d_n5), (-eq53_e1397_d_n6), (-eq53_e1397_d_n7), (-eq53_e1397_d_n8), (-eq53_e1397_d_n9), (-eq53_e1397_d_n10), (-eq53_e1397_d_n11), (-eq53_e1397_d_n12)];
        let eq53_reactive_branch_derivatives: [f64; 7] = [(-eq53_e1397_d_b0), (-eq53_e1397_d_b1), (-eq53_e1397_d_b2), (-eq53_e1397_d_b3), (-eq53_e1397_d_b4), (-eq53_e1397_d_b5), (-eq53_e1397_d_b6)];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
