#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        var_devsign: f64,
        var_guard644: f64,
        var_guard645: f64,
        var_guard646: f64,
        var_guard647: f64,
        var_guard648: f64,
        var_idsgen_v: f64,
        var_idsgen_v_dn0: f64,
        var_idsgen_v_dn10: f64,
        var_idsgen_v_dn11: f64,
        var_idsgen_v_dn13: f64,
        var_idsgen_v_dn14: f64,
        var_idsgen_v_dn2: f64,
        var_idsgen_v_dn3: f64,
        var_idsgen_v_dn4: f64,
        var_idsgen_v_dn5: f64,
        var_idsgen_v_dn6: f64,
        var_idsgen_v_dn7: f64,
        var_idsgen_v_dn8: f64,
        var_idsgen_v_dn9: f64,
        var_igbacc_v: f64,
        var_igbacc_v_dn0: f64,
        var_igbacc_v_dn10: f64,
        var_igbacc_v_dn11: f64,
        var_igbacc_v_dn13: f64,
        var_igbacc_v_dn14: f64,
        var_igbacc_v_dn2: f64,
        var_igbacc_v_dn3: f64,
        var_igbacc_v_dn4: f64,
        var_igbacc_v_dn5: f64,
        var_igbacc_v_dn6: f64,
        var_igbacc_v_dn7: f64,
        var_igbacc_v_dn8: f64,
        var_igbacc_v_dn9: f64,
        var_igbinv_v: f64,
        var_igbinv_v_dn0: f64,
        var_igbinv_v_dn10: f64,
        var_igbinv_v_dn11: f64,
        var_igbinv_v_dn13: f64,
        var_igbinv_v_dn14: f64,
        var_igbinv_v_dn2: f64,
        var_igbinv_v_dn3: f64,
        var_igbinv_v_dn4: f64,
        var_igbinv_v_dn5: f64,
        var_igbinv_v_dn6: f64,
        var_igbinv_v_dn7: f64,
        var_igbinv_v_dn8: f64,
        var_igbinv_v_dn9: f64,
        var_igcd_v: f64,
        var_igcd_v_dn0: f64,
        var_igcd_v_dn10: f64,
        var_igcd_v_dn11: f64,
        var_igcd_v_dn13: f64,
        var_igcd_v_dn14: f64,
        var_igcd_v_dn2: f64,
        var_igcd_v_dn3: f64,
        var_igcd_v_dn4: f64,
        var_igcd_v_dn5: f64,
        var_igcd_v_dn6: f64,
        var_igcd_v_dn7: f64,
        var_igcd_v_dn8: f64,
        var_igcd_v_dn9: f64,
        var_igcs_v: f64,
        var_igcs_v_dn0: f64,
        var_igcs_v_dn10: f64,
        var_igcs_v_dn11: f64,
        var_igcs_v_dn13: f64,
        var_igcs_v_dn14: f64,
        var_igcs_v_dn2: f64,
        var_igcs_v_dn3: f64,
        var_igcs_v_dn4: f64,
        var_igcs_v_dn5: f64,
        var_igcs_v_dn6: f64,
        var_igcs_v_dn7: f64,
        var_igcs_v_dn8: f64,
        var_igcs_v_dn9: f64,
        var_igd_v: f64,
        var_igd_v_dn0: f64,
        var_igd_v_dn10: f64,
        var_igd_v_dn11: f64,
        var_igd_v_dn13: f64,
        var_igd_v_dn14: f64,
        var_igd_v_dn2: f64,
        var_igd_v_dn3: f64,
        var_igd_v_dn4: f64,
        var_igd_v_dn5: f64,
        var_igd_v_dn6: f64,
        var_igd_v_dn7: f64,
        var_igd_v_dn8: f64,
        var_igd_v_dn9: f64,
        var_igidl_v: f64,
        var_igidl_v_dn0: f64,
        var_igidl_v_dn10: f64,
        var_igidl_v_dn11: f64,
        var_igidl_v_dn13: f64,
        var_igidl_v_dn14: f64,
        var_igidl_v_dn2: f64,
        var_igidl_v_dn3: f64,
        var_igidl_v_dn4: f64,
        var_igidl_v_dn5: f64,
        var_igidl_v_dn6: f64,
        var_igidl_v_dn7: f64,
        var_igidl_v_dn8: f64,
        var_igidl_v_dn9: f64,
        var_igidlb: f64,
        var_igidlb_dn0: f64,
        var_igidlb_dn10: f64,
        var_igidlb_dn11: f64,
        var_igidlb_dn13: f64,
        var_igidlb_dn14: f64,
        var_igidlb_dn2: f64,
        var_igidlb_dn3: f64,
        var_igidlb_dn4: f64,
        var_igidlb_dn5: f64,
        var_igidlb_dn6: f64,
        var_igidlb_dn7: f64,
        var_igidlb_dn8: f64,
        var_igidlb_dn9: f64,
        var_igisl_v: f64,
        var_igisl_v_dn0: f64,
        var_igisl_v_dn10: f64,
        var_igisl_v_dn11: f64,
        var_igisl_v_dn13: f64,
        var_igisl_v_dn14: f64,
        var_igisl_v_dn2: f64,
        var_igisl_v_dn3: f64,
        var_igisl_v_dn4: f64,
        var_igisl_v_dn5: f64,
        var_igisl_v_dn6: f64,
        var_igisl_v_dn7: f64,
        var_igisl_v_dn8: f64,
        var_igisl_v_dn9: f64,
        var_igislb: f64,
        var_igislb_dn0: f64,
        var_igislb_dn10: f64,
        var_igislb_dn11: f64,
        var_igislb_dn13: f64,
        var_igislb_dn14: f64,
        var_igislb_dn2: f64,
        var_igislb_dn3: f64,
        var_igislb_dn4: f64,
        var_igislb_dn5: f64,
        var_igislb_dn6: f64,
        var_igislb_dn7: f64,
        var_igislb_dn8: f64,
        var_igislb_dn9: f64,
        var_igs_v: f64,
        var_igs_v_dn0: f64,
        var_igs_v_dn10: f64,
        var_igs_v_dn11: f64,
        var_igs_v_dn13: f64,
        var_igs_v_dn14: f64,
        var_igs_v_dn2: f64,
        var_igs_v_dn3: f64,
        var_igs_v_dn4: f64,
        var_igs_v_dn5: f64,
        var_igs_v_dn6: f64,
        var_igs_v_dn7: f64,
        var_igs_v_dn8: f64,
        var_igs_v_dn9: f64,
        var_iii_1: f64,
        var_iii_1_dn0: f64,
        var_iii_1_dn10: f64,
        var_iii_1_dn11: f64,
        var_iii_1_dn13: f64,
        var_iii_1_dn14: f64,
        var_iii_1_dn2: f64,
        var_iii_1_dn3: f64,
        var_iii_1_dn4: f64,
        var_iii_1_dn5: f64,
        var_iii_1_dn6: f64,
        var_iii_1_dn7: f64,
        var_iii_1_dn8: f64,
        var_iii_1_dn9: f64,
    ) {
        let (eq13_e2052, eq13_e2052_d_n0, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n13, eq13_e2052_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq13_e2050: f64 = (var_devsign * var_igidlb);
        let eq13_e2050_d_n0: f64 = (var_devsign * var_igidlb_dn0);
        let eq13_e2050_d_n2: f64 = (var_devsign * var_igidlb_dn2);
        let eq13_e2050_d_n3: f64 = (var_devsign * var_igidlb_dn3);
        let eq13_e2050_d_n4: f64 = (var_devsign * var_igidlb_dn4);
        let eq13_e2050_d_n5: f64 = (var_devsign * var_igidlb_dn5);
        let eq13_e2050_d_n6: f64 = (var_devsign * var_igidlb_dn6);
        let eq13_e2050_d_n7: f64 = (var_devsign * var_igidlb_dn7);
        let eq13_e2050_d_n8: f64 = (var_devsign * var_igidlb_dn8);
        let eq13_e2050_d_n9: f64 = (var_devsign * var_igidlb_dn9);
        let eq13_e2050_d_n10: f64 = (var_devsign * var_igidlb_dn10);
        let eq13_e2050_d_n11: f64 = (var_devsign * var_igidlb_dn11);
        let eq13_e2050_d_n13: f64 = (var_devsign * var_igidlb_dn13);
        let eq13_e2050_d_n14: f64 = (var_devsign * var_igidlb_dn14);
        (eq13_e2050, eq13_e2050_d_n0, eq13_e2050_d_n2, eq13_e2050_d_n3, eq13_e2050_d_n4, eq13_e2050_d_n5, eq13_e2050_d_n6, eq13_e2050_d_n7, eq13_e2050_d_n8, eq13_e2050_d_n9, eq13_e2050_d_n10, eq13_e2050_d_n11, eq13_e2050_d_n13, eq13_e2050_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq13_value: f64 = eq13_e2052;
        let eq13_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq13_node_derivatives: [f64; 13] = [eq13_e2052_d_n0, eq13_e2052_d_n2, eq13_e2052_d_n3, eq13_e2052_d_n4, eq13_e2052_d_n5, eq13_e2052_d_n6, eq13_e2052_d_n7, eq13_e2052_d_n8, eq13_e2052_d_n9, eq13_e2052_d_n10, eq13_e2052_d_n11, eq13_e2052_d_n13, eq13_e2052_d_n14];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let (eq14_e2062, eq14_e2062_d_n0, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n13, eq14_e2062_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 != 0.0)) {
        let eq14_e2060: f64 = (var_devsign * var_igislb);
        let eq14_e2060_d_n0: f64 = (var_devsign * var_igislb_dn0);
        let eq14_e2060_d_n2: f64 = (var_devsign * var_igislb_dn2);
        let eq14_e2060_d_n3: f64 = (var_devsign * var_igislb_dn3);
        let eq14_e2060_d_n4: f64 = (var_devsign * var_igislb_dn4);
        let eq14_e2060_d_n5: f64 = (var_devsign * var_igislb_dn5);
        let eq14_e2060_d_n6: f64 = (var_devsign * var_igislb_dn6);
        let eq14_e2060_d_n7: f64 = (var_devsign * var_igislb_dn7);
        let eq14_e2060_d_n8: f64 = (var_devsign * var_igislb_dn8);
        let eq14_e2060_d_n9: f64 = (var_devsign * var_igislb_dn9);
        let eq14_e2060_d_n10: f64 = (var_devsign * var_igislb_dn10);
        let eq14_e2060_d_n11: f64 = (var_devsign * var_igislb_dn11);
        let eq14_e2060_d_n13: f64 = (var_devsign * var_igislb_dn13);
        let eq14_e2060_d_n14: f64 = (var_devsign * var_igislb_dn14);
        (eq14_e2060, eq14_e2060_d_n0, eq14_e2060_d_n2, eq14_e2060_d_n3, eq14_e2060_d_n4, eq14_e2060_d_n5, eq14_e2060_d_n6, eq14_e2060_d_n7, eq14_e2060_d_n8, eq14_e2060_d_n9, eq14_e2060_d_n10, eq14_e2060_d_n11, eq14_e2060_d_n13, eq14_e2060_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq14_value: f64 = eq14_e2062;
        let eq14_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq14_node_derivatives: [f64; 13] = [eq14_e2062_d_n0, eq14_e2062_d_n2, eq14_e2062_d_n3, eq14_e2062_d_n4, eq14_e2062_d_n5, eq14_e2062_d_n6, eq14_e2062_d_n7, eq14_e2062_d_n8, eq14_e2062_d_n9, eq14_e2062_d_n10, eq14_e2062_d_n11, eq14_e2062_d_n13, eq14_e2062_d_n14];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e2075, eq15_e2075_d_n0, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n13, eq15_e2075_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) {
        let eq15_e2072: f64 = (var_igidl_v + var_iii_1);
        let eq15_e2072_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq15_e2072_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq15_e2072_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq15_e2072_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq15_e2072_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq15_e2072_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq15_e2072_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq15_e2072_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq15_e2072_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq15_e2072_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq15_e2072_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq15_e2072_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq15_e2072_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq15_e2073: f64 = (var_devsign * eq15_e2072);
        let eq15_e2073_d_n0: f64 = (var_devsign * eq15_e2072_d_n0);
        let eq15_e2073_d_n2: f64 = (var_devsign * eq15_e2072_d_n2);
        let eq15_e2073_d_n3: f64 = (var_devsign * eq15_e2072_d_n3);
        let eq15_e2073_d_n4: f64 = (var_devsign * eq15_e2072_d_n4);
        let eq15_e2073_d_n5: f64 = (var_devsign * eq15_e2072_d_n5);
        let eq15_e2073_d_n6: f64 = (var_devsign * eq15_e2072_d_n6);
        let eq15_e2073_d_n7: f64 = (var_devsign * eq15_e2072_d_n7);
        let eq15_e2073_d_n8: f64 = (var_devsign * eq15_e2072_d_n8);
        let eq15_e2073_d_n9: f64 = (var_devsign * eq15_e2072_d_n9);
        let eq15_e2073_d_n10: f64 = (var_devsign * eq15_e2072_d_n10);
        let eq15_e2073_d_n11: f64 = (var_devsign * eq15_e2072_d_n11);
        let eq15_e2073_d_n13: f64 = (var_devsign * eq15_e2072_d_n13);
        let eq15_e2073_d_n14: f64 = (var_devsign * eq15_e2072_d_n14);
        (eq15_e2073, eq15_e2073_d_n0, eq15_e2073_d_n2, eq15_e2073_d_n3, eq15_e2073_d_n4, eq15_e2073_d_n5, eq15_e2073_d_n6, eq15_e2073_d_n7, eq15_e2073_d_n8, eq15_e2073_d_n9, eq15_e2073_d_n10, eq15_e2073_d_n11, eq15_e2073_d_n13, eq15_e2073_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e2075;
        let eq15_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq15_node_derivatives: [f64; 13] = [eq15_e2075_d_n0, eq15_e2075_d_n2, eq15_e2075_d_n3, eq15_e2075_d_n4, eq15_e2075_d_n5, eq15_e2075_d_n6, eq15_e2075_d_n7, eq15_e2075_d_n8, eq15_e2075_d_n9, eq15_e2075_d_n10, eq15_e2075_d_n11, eq15_e2075_d_n13, eq15_e2075_d_n14];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e2086, eq16_e2086_d_n0, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n13, eq16_e2086_d_n14,) = {
    if (((var_guard644 != 0.0) && (var_guard645 != 0.0)) && (var_guard646 == 0.0)) {
        let eq16_e2084: f64 = (var_devsign * var_igisl_v);
        let eq16_e2084_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq16_e2084_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq16_e2084_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq16_e2084_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq16_e2084_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq16_e2084_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq16_e2084_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq16_e2084_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq16_e2084_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq16_e2084_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq16_e2084_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq16_e2084_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq16_e2084_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq16_e2084, eq16_e2084_d_n0, eq16_e2084_d_n2, eq16_e2084_d_n3, eq16_e2084_d_n4, eq16_e2084_d_n5, eq16_e2084_d_n6, eq16_e2084_d_n7, eq16_e2084_d_n8, eq16_e2084_d_n9, eq16_e2084_d_n10, eq16_e2084_d_n11, eq16_e2084_d_n13, eq16_e2084_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq16_value: f64 = eq16_e2086;
        let eq16_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq16_node_derivatives: [f64; 13] = [eq16_e2086_d_n0, eq16_e2086_d_n2, eq16_e2086_d_n3, eq16_e2086_d_n4, eq16_e2086_d_n5, eq16_e2086_d_n6, eq16_e2086_d_n7, eq16_e2086_d_n8, eq16_e2086_d_n9, eq16_e2086_d_n10, eq16_e2086_d_n11, eq16_e2086_d_n13, eq16_e2086_d_n14];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e2096, eq17_e2096_d_n0, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n13, eq17_e2096_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 != 0.0)) {
        let eq17_e2093: f64 = (var_igbinv_v + var_igbacc_v);
        let eq17_e2093_d_n0: f64 = (var_igbinv_v_dn0 + var_igbacc_v_dn0);
        let eq17_e2093_d_n2: f64 = (var_igbinv_v_dn2 + var_igbacc_v_dn2);
        let eq17_e2093_d_n3: f64 = (var_igbinv_v_dn3 + var_igbacc_v_dn3);
        let eq17_e2093_d_n4: f64 = (var_igbinv_v_dn4 + var_igbacc_v_dn4);
        let eq17_e2093_d_n5: f64 = (var_igbinv_v_dn5 + var_igbacc_v_dn5);
        let eq17_e2093_d_n6: f64 = (var_igbinv_v_dn6 + var_igbacc_v_dn6);
        let eq17_e2093_d_n7: f64 = (var_igbinv_v_dn7 + var_igbacc_v_dn7);
        let eq17_e2093_d_n8: f64 = (var_igbinv_v_dn8 + var_igbacc_v_dn8);
        let eq17_e2093_d_n9: f64 = (var_igbinv_v_dn9 + var_igbacc_v_dn9);
        let eq17_e2093_d_n10: f64 = (var_igbinv_v_dn10 + var_igbacc_v_dn10);
        let eq17_e2093_d_n11: f64 = (var_igbinv_v_dn11 + var_igbacc_v_dn11);
        let eq17_e2093_d_n13: f64 = (var_igbinv_v_dn13 + var_igbacc_v_dn13);
        let eq17_e2093_d_n14: f64 = (var_igbinv_v_dn14 + var_igbacc_v_dn14);
        let eq17_e2094: f64 = (var_devsign * eq17_e2093);
        let eq17_e2094_d_n0: f64 = (var_devsign * eq17_e2093_d_n0);
        let eq17_e2094_d_n2: f64 = (var_devsign * eq17_e2093_d_n2);
        let eq17_e2094_d_n3: f64 = (var_devsign * eq17_e2093_d_n3);
        let eq17_e2094_d_n4: f64 = (var_devsign * eq17_e2093_d_n4);
        let eq17_e2094_d_n5: f64 = (var_devsign * eq17_e2093_d_n5);
        let eq17_e2094_d_n6: f64 = (var_devsign * eq17_e2093_d_n6);
        let eq17_e2094_d_n7: f64 = (var_devsign * eq17_e2093_d_n7);
        let eq17_e2094_d_n8: f64 = (var_devsign * eq17_e2093_d_n8);
        let eq17_e2094_d_n9: f64 = (var_devsign * eq17_e2093_d_n9);
        let eq17_e2094_d_n10: f64 = (var_devsign * eq17_e2093_d_n10);
        let eq17_e2094_d_n11: f64 = (var_devsign * eq17_e2093_d_n11);
        let eq17_e2094_d_n13: f64 = (var_devsign * eq17_e2093_d_n13);
        let eq17_e2094_d_n14: f64 = (var_devsign * eq17_e2093_d_n14);
        (eq17_e2094, eq17_e2094_d_n0, eq17_e2094_d_n2, eq17_e2094_d_n3, eq17_e2094_d_n4, eq17_e2094_d_n5, eq17_e2094_d_n6, eq17_e2094_d_n7, eq17_e2094_d_n8, eq17_e2094_d_n9, eq17_e2094_d_n10, eq17_e2094_d_n11, eq17_e2094_d_n13, eq17_e2094_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e2096;
        let eq17_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq17_node_derivatives: [f64; 13] = [eq17_e2096_d_n0, eq17_e2096_d_n2, eq17_e2096_d_n3, eq17_e2096_d_n4, eq17_e2096_d_n5, eq17_e2096_d_n6, eq17_e2096_d_n7, eq17_e2096_d_n8, eq17_e2096_d_n9, eq17_e2096_d_n10, eq17_e2096_d_n11, eq17_e2096_d_n13, eq17_e2096_d_n14];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(3),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e2107, eq18_e2107_d_n0, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n13, eq18_e2107_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 == 0.0)) {
        let eq18_e2104: f64 = (var_igidl_v + var_iii_1);
        let eq18_e2104_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq18_e2104_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq18_e2104_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq18_e2104_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq18_e2104_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq18_e2104_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq18_e2104_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq18_e2104_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq18_e2104_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq18_e2104_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq18_e2104_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq18_e2104_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq18_e2104_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq18_e2105: f64 = (var_devsign * eq18_e2104);
        let eq18_e2105_d_n0: f64 = (var_devsign * eq18_e2104_d_n0);
        let eq18_e2105_d_n2: f64 = (var_devsign * eq18_e2104_d_n2);
        let eq18_e2105_d_n3: f64 = (var_devsign * eq18_e2104_d_n3);
        let eq18_e2105_d_n4: f64 = (var_devsign * eq18_e2104_d_n4);
        let eq18_e2105_d_n5: f64 = (var_devsign * eq18_e2104_d_n5);
        let eq18_e2105_d_n6: f64 = (var_devsign * eq18_e2104_d_n6);
        let eq18_e2105_d_n7: f64 = (var_devsign * eq18_e2104_d_n7);
        let eq18_e2105_d_n8: f64 = (var_devsign * eq18_e2104_d_n8);
        let eq18_e2105_d_n9: f64 = (var_devsign * eq18_e2104_d_n9);
        let eq18_e2105_d_n10: f64 = (var_devsign * eq18_e2104_d_n10);
        let eq18_e2105_d_n11: f64 = (var_devsign * eq18_e2104_d_n11);
        let eq18_e2105_d_n13: f64 = (var_devsign * eq18_e2104_d_n13);
        let eq18_e2105_d_n14: f64 = (var_devsign * eq18_e2104_d_n14);
        (eq18_e2105, eq18_e2105_d_n0, eq18_e2105_d_n2, eq18_e2105_d_n3, eq18_e2105_d_n4, eq18_e2105_d_n5, eq18_e2105_d_n6, eq18_e2105_d_n7, eq18_e2105_d_n8, eq18_e2105_d_n9, eq18_e2105_d_n10, eq18_e2105_d_n11, eq18_e2105_d_n13, eq18_e2105_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e2107;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq18_node_derivatives: [f64; 13] = [eq18_e2107_d_n0, eq18_e2107_d_n2, eq18_e2107_d_n3, eq18_e2107_d_n4, eq18_e2107_d_n5, eq18_e2107_d_n6, eq18_e2107_d_n7, eq18_e2107_d_n8, eq18_e2107_d_n9, eq18_e2107_d_n10, eq18_e2107_d_n11, eq18_e2107_d_n13, eq18_e2107_d_n14];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e2116, eq19_e2116_d_n0, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n13, eq19_e2116_d_n14,) = {
    if ((var_guard644 != 0.0) && (var_guard645 == 0.0)) {
        let eq19_e2114: f64 = (var_devsign * var_igisl_v);
        let eq19_e2114_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq19_e2114_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq19_e2114_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq19_e2114_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq19_e2114_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq19_e2114_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq19_e2114_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq19_e2114_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq19_e2114_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq19_e2114_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq19_e2114_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq19_e2114_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq19_e2114_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq19_e2114, eq19_e2114_d_n0, eq19_e2114_d_n2, eq19_e2114_d_n3, eq19_e2114_d_n4, eq19_e2114_d_n5, eq19_e2114_d_n6, eq19_e2114_d_n7, eq19_e2114_d_n8, eq19_e2114_d_n9, eq19_e2114_d_n10, eq19_e2114_d_n11, eq19_e2114_d_n13, eq19_e2114_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e2116;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq19_node_derivatives: [f64; 13] = [eq19_e2116_d_n0, eq19_e2116_d_n2, eq19_e2116_d_n3, eq19_e2116_d_n4, eq19_e2116_d_n5, eq19_e2116_d_n6, eq19_e2116_d_n7, eq19_e2116_d_n8, eq19_e2116_d_n9, eq19_e2116_d_n10, eq19_e2116_d_n11, eq19_e2116_d_n13, eq19_e2116_d_n14];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e2123, eq20_e2123_d_n0, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n13, eq20_e2123_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq20_e2121: f64 = (var_devsign * var_idsgen_v);
        let eq20_e2121_d_n0: f64 = (var_devsign * var_idsgen_v_dn0);
        let eq20_e2121_d_n2: f64 = (var_devsign * var_idsgen_v_dn2);
        let eq20_e2121_d_n3: f64 = (var_devsign * var_idsgen_v_dn3);
        let eq20_e2121_d_n4: f64 = (var_devsign * var_idsgen_v_dn4);
        let eq20_e2121_d_n5: f64 = (var_devsign * var_idsgen_v_dn5);
        let eq20_e2121_d_n6: f64 = (var_devsign * var_idsgen_v_dn6);
        let eq20_e2121_d_n7: f64 = (var_devsign * var_idsgen_v_dn7);
        let eq20_e2121_d_n8: f64 = (var_devsign * var_idsgen_v_dn8);
        let eq20_e2121_d_n9: f64 = (var_devsign * var_idsgen_v_dn9);
        let eq20_e2121_d_n10: f64 = (var_devsign * var_idsgen_v_dn10);
        let eq20_e2121_d_n11: f64 = (var_devsign * var_idsgen_v_dn11);
        let eq20_e2121_d_n13: f64 = (var_devsign * var_idsgen_v_dn13);
        let eq20_e2121_d_n14: f64 = (var_devsign * var_idsgen_v_dn14);
        (eq20_e2121, eq20_e2121_d_n0, eq20_e2121_d_n2, eq20_e2121_d_n3, eq20_e2121_d_n4, eq20_e2121_d_n5, eq20_e2121_d_n6, eq20_e2121_d_n7, eq20_e2121_d_n8, eq20_e2121_d_n9, eq20_e2121_d_n10, eq20_e2121_d_n11, eq20_e2121_d_n13, eq20_e2121_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e2123;
        let eq20_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq20_node_derivatives: [f64; 13] = [eq20_e2123_d_n0, eq20_e2123_d_n2, eq20_e2123_d_n3, eq20_e2123_d_n4, eq20_e2123_d_n5, eq20_e2123_d_n6, eq20_e2123_d_n7, eq20_e2123_d_n8, eq20_e2123_d_n9, eq20_e2123_d_n10, eq20_e2123_d_n11, eq20_e2123_d_n13, eq20_e2123_d_n14];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e2132, eq21_e2132_d_n0, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n13, eq21_e2132_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq21_e2129: f64 = (var_igcs_v + var_igs_v);
        let eq21_e2129_d_n0: f64 = (var_igcs_v_dn0 + var_igs_v_dn0);
        let eq21_e2129_d_n2: f64 = (var_igcs_v_dn2 + var_igs_v_dn2);
        let eq21_e2129_d_n3: f64 = (var_igcs_v_dn3 + var_igs_v_dn3);
        let eq21_e2129_d_n4: f64 = (var_igcs_v_dn4 + var_igs_v_dn4);
        let eq21_e2129_d_n5: f64 = (var_igcs_v_dn5 + var_igs_v_dn5);
        let eq21_e2129_d_n6: f64 = (var_igcs_v_dn6 + var_igs_v_dn6);
        let eq21_e2129_d_n7: f64 = (var_igcs_v_dn7 + var_igs_v_dn7);
        let eq21_e2129_d_n8: f64 = (var_igcs_v_dn8 + var_igs_v_dn8);
        let eq21_e2129_d_n9: f64 = (var_igcs_v_dn9 + var_igs_v_dn9);
        let eq21_e2129_d_n10: f64 = (var_igcs_v_dn10 + var_igs_v_dn10);
        let eq21_e2129_d_n11: f64 = (var_igcs_v_dn11 + var_igs_v_dn11);
        let eq21_e2129_d_n13: f64 = (var_igcs_v_dn13 + var_igs_v_dn13);
        let eq21_e2129_d_n14: f64 = (var_igcs_v_dn14 + var_igs_v_dn14);
        let eq21_e2130: f64 = (var_devsign * eq21_e2129);
        let eq21_e2130_d_n0: f64 = (var_devsign * eq21_e2129_d_n0);
        let eq21_e2130_d_n2: f64 = (var_devsign * eq21_e2129_d_n2);
        let eq21_e2130_d_n3: f64 = (var_devsign * eq21_e2129_d_n3);
        let eq21_e2130_d_n4: f64 = (var_devsign * eq21_e2129_d_n4);
        let eq21_e2130_d_n5: f64 = (var_devsign * eq21_e2129_d_n5);
        let eq21_e2130_d_n6: f64 = (var_devsign * eq21_e2129_d_n6);
        let eq21_e2130_d_n7: f64 = (var_devsign * eq21_e2129_d_n7);
        let eq21_e2130_d_n8: f64 = (var_devsign * eq21_e2129_d_n8);
        let eq21_e2130_d_n9: f64 = (var_devsign * eq21_e2129_d_n9);
        let eq21_e2130_d_n10: f64 = (var_devsign * eq21_e2129_d_n10);
        let eq21_e2130_d_n11: f64 = (var_devsign * eq21_e2129_d_n11);
        let eq21_e2130_d_n13: f64 = (var_devsign * eq21_e2129_d_n13);
        let eq21_e2130_d_n14: f64 = (var_devsign * eq21_e2129_d_n14);
        (eq21_e2130, eq21_e2130_d_n0, eq21_e2130_d_n2, eq21_e2130_d_n3, eq21_e2130_d_n4, eq21_e2130_d_n5, eq21_e2130_d_n6, eq21_e2130_d_n7, eq21_e2130_d_n8, eq21_e2130_d_n9, eq21_e2130_d_n10, eq21_e2130_d_n11, eq21_e2130_d_n13, eq21_e2130_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e2132;
        let eq21_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq21_node_derivatives: [f64; 13] = [eq21_e2132_d_n0, eq21_e2132_d_n2, eq21_e2132_d_n3, eq21_e2132_d_n4, eq21_e2132_d_n5, eq21_e2132_d_n6, eq21_e2132_d_n7, eq21_e2132_d_n8, eq21_e2132_d_n9, eq21_e2132_d_n10, eq21_e2132_d_n11, eq21_e2132_d_n13, eq21_e2132_d_n14];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e2141, eq22_e2141_d_n0, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n13, eq22_e2141_d_n14,) = {
    if (var_guard644 == 0.0) {
        let eq22_e2138: f64 = (var_igcd_v + var_igd_v);
        let eq22_e2138_d_n0: f64 = (var_igcd_v_dn0 + var_igd_v_dn0);
        let eq22_e2138_d_n2: f64 = (var_igcd_v_dn2 + var_igd_v_dn2);
        let eq22_e2138_d_n3: f64 = (var_igcd_v_dn3 + var_igd_v_dn3);
        let eq22_e2138_d_n4: f64 = (var_igcd_v_dn4 + var_igd_v_dn4);
        let eq22_e2138_d_n5: f64 = (var_igcd_v_dn5 + var_igd_v_dn5);
        let eq22_e2138_d_n6: f64 = (var_igcd_v_dn6 + var_igd_v_dn6);
        let eq22_e2138_d_n7: f64 = (var_igcd_v_dn7 + var_igd_v_dn7);
        let eq22_e2138_d_n8: f64 = (var_igcd_v_dn8 + var_igd_v_dn8);
        let eq22_e2138_d_n9: f64 = (var_igcd_v_dn9 + var_igd_v_dn9);
        let eq22_e2138_d_n10: f64 = (var_igcd_v_dn10 + var_igd_v_dn10);
        let eq22_e2138_d_n11: f64 = (var_igcd_v_dn11 + var_igd_v_dn11);
        let eq22_e2138_d_n13: f64 = (var_igcd_v_dn13 + var_igd_v_dn13);
        let eq22_e2138_d_n14: f64 = (var_igcd_v_dn14 + var_igd_v_dn14);
        let eq22_e2139: f64 = (var_devsign * eq22_e2138);
        let eq22_e2139_d_n0: f64 = (var_devsign * eq22_e2138_d_n0);
        let eq22_e2139_d_n2: f64 = (var_devsign * eq22_e2138_d_n2);
        let eq22_e2139_d_n3: f64 = (var_devsign * eq22_e2138_d_n3);
        let eq22_e2139_d_n4: f64 = (var_devsign * eq22_e2138_d_n4);
        let eq22_e2139_d_n5: f64 = (var_devsign * eq22_e2138_d_n5);
        let eq22_e2139_d_n6: f64 = (var_devsign * eq22_e2138_d_n6);
        let eq22_e2139_d_n7: f64 = (var_devsign * eq22_e2138_d_n7);
        let eq22_e2139_d_n8: f64 = (var_devsign * eq22_e2138_d_n8);
        let eq22_e2139_d_n9: f64 = (var_devsign * eq22_e2138_d_n9);
        let eq22_e2139_d_n10: f64 = (var_devsign * eq22_e2138_d_n10);
        let eq22_e2139_d_n11: f64 = (var_devsign * eq22_e2138_d_n11);
        let eq22_e2139_d_n13: f64 = (var_devsign * eq22_e2138_d_n13);
        let eq22_e2139_d_n14: f64 = (var_devsign * eq22_e2138_d_n14);
        (eq22_e2139, eq22_e2139_d_n0, eq22_e2139_d_n2, eq22_e2139_d_n3, eq22_e2139_d_n4, eq22_e2139_d_n5, eq22_e2139_d_n6, eq22_e2139_d_n7, eq22_e2139_d_n8, eq22_e2139_d_n9, eq22_e2139_d_n10, eq22_e2139_d_n11, eq22_e2139_d_n13, eq22_e2139_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e2141;
        let eq22_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq22_node_derivatives: [f64; 13] = [eq22_e2141_d_n0, eq22_e2141_d_n2, eq22_e2141_d_n3, eq22_e2141_d_n4, eq22_e2141_d_n5, eq22_e2141_d_n6, eq22_e2141_d_n7, eq22_e2141_d_n8, eq22_e2141_d_n9, eq22_e2141_d_n10, eq22_e2141_d_n11, eq22_e2141_d_n13, eq22_e2141_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e2154, eq23_e2154_d_n0, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n13, eq23_e2154_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq23_e2151: f64 = (var_igidl_v + var_iii_1);
        let eq23_e2151_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq23_e2151_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq23_e2151_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq23_e2151_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq23_e2151_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq23_e2151_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq23_e2151_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq23_e2151_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq23_e2151_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq23_e2151_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq23_e2151_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq23_e2151_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq23_e2151_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq23_e2152: f64 = (var_devsign * eq23_e2151);
        let eq23_e2152_d_n0: f64 = (var_devsign * eq23_e2151_d_n0);
        let eq23_e2152_d_n2: f64 = (var_devsign * eq23_e2151_d_n2);
        let eq23_e2152_d_n3: f64 = (var_devsign * eq23_e2151_d_n3);
        let eq23_e2152_d_n4: f64 = (var_devsign * eq23_e2151_d_n4);
        let eq23_e2152_d_n5: f64 = (var_devsign * eq23_e2151_d_n5);
        let eq23_e2152_d_n6: f64 = (var_devsign * eq23_e2151_d_n6);
        let eq23_e2152_d_n7: f64 = (var_devsign * eq23_e2151_d_n7);
        let eq23_e2152_d_n8: f64 = (var_devsign * eq23_e2151_d_n8);
        let eq23_e2152_d_n9: f64 = (var_devsign * eq23_e2151_d_n9);
        let eq23_e2152_d_n10: f64 = (var_devsign * eq23_e2151_d_n10);
        let eq23_e2152_d_n11: f64 = (var_devsign * eq23_e2151_d_n11);
        let eq23_e2152_d_n13: f64 = (var_devsign * eq23_e2151_d_n13);
        let eq23_e2152_d_n14: f64 = (var_devsign * eq23_e2151_d_n14);
        (eq23_e2152, eq23_e2152_d_n0, eq23_e2152_d_n2, eq23_e2152_d_n3, eq23_e2152_d_n4, eq23_e2152_d_n5, eq23_e2152_d_n6, eq23_e2152_d_n7, eq23_e2152_d_n8, eq23_e2152_d_n9, eq23_e2152_d_n10, eq23_e2152_d_n11, eq23_e2152_d_n13, eq23_e2152_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e2154;
        let eq23_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq23_node_derivatives: [f64; 13] = [eq23_e2154_d_n0, eq23_e2154_d_n2, eq23_e2154_d_n3, eq23_e2154_d_n4, eq23_e2154_d_n5, eq23_e2154_d_n6, eq23_e2154_d_n7, eq23_e2154_d_n8, eq23_e2154_d_n9, eq23_e2154_d_n10, eq23_e2154_d_n11, eq23_e2154_d_n13, eq23_e2154_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_devsign: f64,
        var_gmin: f64,
        var_guard644: f64,
        var_guard647: f64,
        var_guard648: f64,
        var_guard649: f64,
        var_guard650: f64,
        var_ied: f64,
        var_ied_dn0: f64,
        var_ied_dn10: f64,
        var_ied_dn11: f64,
        var_ied_dn13: f64,
        var_ied_dn14: f64,
        var_ied_dn2: f64,
        var_ied_dn3: f64,
        var_ied_dn4: f64,
        var_ied_dn5: f64,
        var_ied_dn6: f64,
        var_ied_dn7: f64,
        var_ied_dn8: f64,
        var_ied_dn9: f64,
        var_ies: f64,
        var_ies_dn0: f64,
        var_ies_dn10: f64,
        var_ies_dn11: f64,
        var_ies_dn13: f64,
        var_ies_dn14: f64,
        var_ies_dn2: f64,
        var_ies_dn3: f64,
        var_ies_dn4: f64,
        var_ies_dn5: f64,
        var_ies_dn6: f64,
        var_ies_dn7: f64,
        var_ies_dn8: f64,
        var_ies_dn9: f64,
        var_igbacc_v: f64,
        var_igbacc_v_dn0: f64,
        var_igbacc_v_dn10: f64,
        var_igbacc_v_dn11: f64,
        var_igbacc_v_dn13: f64,
        var_igbacc_v_dn14: f64,
        var_igbacc_v_dn2: f64,
        var_igbacc_v_dn3: f64,
        var_igbacc_v_dn4: f64,
        var_igbacc_v_dn5: f64,
        var_igbacc_v_dn6: f64,
        var_igbacc_v_dn7: f64,
        var_igbacc_v_dn8: f64,
        var_igbacc_v_dn9: f64,
        var_igbd_v: f64,
        var_igbd_v_dn0: f64,
        var_igbd_v_dn10: f64,
        var_igbd_v_dn11: f64,
        var_igbd_v_dn13: f64,
        var_igbd_v_dn14: f64,
        var_igbd_v_dn2: f64,
        var_igbd_v_dn3: f64,
        var_igbd_v_dn4: f64,
        var_igbd_v_dn5: f64,
        var_igbd_v_dn6: f64,
        var_igbd_v_dn7: f64,
        var_igbd_v_dn8: f64,
        var_igbd_v_dn9: f64,
        var_igbinv_v: f64,
        var_igbinv_v_dn0: f64,
        var_igbinv_v_dn10: f64,
        var_igbinv_v_dn11: f64,
        var_igbinv_v_dn13: f64,
        var_igbinv_v_dn14: f64,
        var_igbinv_v_dn2: f64,
        var_igbinv_v_dn3: f64,
        var_igbinv_v_dn4: f64,
        var_igbinv_v_dn5: f64,
        var_igbinv_v_dn6: f64,
        var_igbinv_v_dn7: f64,
        var_igbinv_v_dn8: f64,
        var_igbinv_v_dn9: f64,
        var_igbs_v: f64,
        var_igbs_v_dn0: f64,
        var_igbs_v_dn10: f64,
        var_igbs_v_dn11: f64,
        var_igbs_v_dn13: f64,
        var_igbs_v_dn14: f64,
        var_igbs_v_dn2: f64,
        var_igbs_v_dn3: f64,
        var_igbs_v_dn4: f64,
        var_igbs_v_dn5: f64,
        var_igbs_v_dn6: f64,
        var_igbs_v_dn7: f64,
        var_igbs_v_dn8: f64,
        var_igbs_v_dn9: f64,
        var_igidl_v: f64,
        var_igidl_v_dn0: f64,
        var_igidl_v_dn10: f64,
        var_igidl_v_dn11: f64,
        var_igidl_v_dn13: f64,
        var_igidl_v_dn14: f64,
        var_igidl_v_dn2: f64,
        var_igidl_v_dn3: f64,
        var_igidl_v_dn4: f64,
        var_igidl_v_dn5: f64,
        var_igidl_v_dn6: f64,
        var_igidl_v_dn7: f64,
        var_igidl_v_dn8: f64,
        var_igidl_v_dn9: f64,
        var_igidlb: f64,
        var_igidlb_dn0: f64,
        var_igidlb_dn10: f64,
        var_igidlb_dn11: f64,
        var_igidlb_dn13: f64,
        var_igidlb_dn14: f64,
        var_igidlb_dn2: f64,
        var_igidlb_dn3: f64,
        var_igidlb_dn4: f64,
        var_igidlb_dn5: f64,
        var_igidlb_dn6: f64,
        var_igidlb_dn7: f64,
        var_igidlb_dn8: f64,
        var_igidlb_dn9: f64,
        var_igisl_v: f64,
        var_igisl_v_dn0: f64,
        var_igisl_v_dn10: f64,
        var_igisl_v_dn11: f64,
        var_igisl_v_dn13: f64,
        var_igisl_v_dn14: f64,
        var_igisl_v_dn2: f64,
        var_igisl_v_dn3: f64,
        var_igisl_v_dn4: f64,
        var_igisl_v_dn5: f64,
        var_igisl_v_dn6: f64,
        var_igisl_v_dn7: f64,
        var_igisl_v_dn8: f64,
        var_igisl_v_dn9: f64,
        var_igislb: f64,
        var_igislb_dn0: f64,
        var_igislb_dn10: f64,
        var_igislb_dn11: f64,
        var_igislb_dn13: f64,
        var_igislb_dn14: f64,
        var_igislb_dn2: f64,
        var_igislb_dn3: f64,
        var_igislb_dn4: f64,
        var_igislb_dn5: f64,
        var_igislb_dn6: f64,
        var_igislb_dn7: f64,
        var_igislb_dn8: f64,
        var_igislb_dn9: f64,
        var_iii_1: f64,
        var_iii_1_dn0: f64,
        var_iii_1_dn10: f64,
        var_iii_1_dn11: f64,
        var_iii_1_dn13: f64,
        var_iii_1_dn14: f64,
        var_iii_1_dn2: f64,
        var_iii_1_dn3: f64,
        var_iii_1_dn4: f64,
        var_iii_1_dn5: f64,
        var_iii_1_dn6: f64,
        var_iii_1_dn7: f64,
        var_iii_1_dn8: f64,
        var_iii_1_dn9: f64,
        var_qes: f64,
        var_qes_dn0: f64,
        var_qes_dn10: f64,
        var_qes_dn11: f64,
        var_qes_dn13: f64,
        var_qes_dn14: f64,
        var_qes_dn2: f64,
        var_qes_dn3: f64,
        var_qes_dn4: f64,
        var_qes_dn5: f64,
        var_qes_dn6: f64,
        var_qes_dn7: f64,
        var_qes_dn8: f64,
        var_qes_dn9: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (eq24_e2165, eq24_e2165_d_n0, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n13, eq24_e2165_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq24_e2163: f64 = (var_devsign * var_igisl_v);
        let eq24_e2163_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq24_e2163_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq24_e2163_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq24_e2163_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq24_e2163_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq24_e2163_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq24_e2163_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq24_e2163_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq24_e2163_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq24_e2163_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq24_e2163_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq24_e2163_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq24_e2163_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq24_e2163, eq24_e2163_d_n0, eq24_e2163_d_n2, eq24_e2163_d_n3, eq24_e2163_d_n4, eq24_e2163_d_n5, eq24_e2163_d_n6, eq24_e2163_d_n7, eq24_e2163_d_n8, eq24_e2163_d_n9, eq24_e2163_d_n10, eq24_e2163_d_n11, eq24_e2163_d_n13, eq24_e2163_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e2165;
        let eq24_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq24_node_derivatives: [f64; 13] = [eq24_e2165_d_n0, eq24_e2165_d_n2, eq24_e2165_d_n3, eq24_e2165_d_n4, eq24_e2165_d_n5, eq24_e2165_d_n6, eq24_e2165_d_n7, eq24_e2165_d_n8, eq24_e2165_d_n9, eq24_e2165_d_n10, eq24_e2165_d_n11, eq24_e2165_d_n13, eq24_e2165_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e2176, eq25_e2176_d_n0, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n13, eq25_e2176_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq25_e2174: f64 = (var_devsign * var_igidlb);
        let eq25_e2174_d_n0: f64 = (var_devsign * var_igidlb_dn0);
        let eq25_e2174_d_n2: f64 = (var_devsign * var_igidlb_dn2);
        let eq25_e2174_d_n3: f64 = (var_devsign * var_igidlb_dn3);
        let eq25_e2174_d_n4: f64 = (var_devsign * var_igidlb_dn4);
        let eq25_e2174_d_n5: f64 = (var_devsign * var_igidlb_dn5);
        let eq25_e2174_d_n6: f64 = (var_devsign * var_igidlb_dn6);
        let eq25_e2174_d_n7: f64 = (var_devsign * var_igidlb_dn7);
        let eq25_e2174_d_n8: f64 = (var_devsign * var_igidlb_dn8);
        let eq25_e2174_d_n9: f64 = (var_devsign * var_igidlb_dn9);
        let eq25_e2174_d_n10: f64 = (var_devsign * var_igidlb_dn10);
        let eq25_e2174_d_n11: f64 = (var_devsign * var_igidlb_dn11);
        let eq25_e2174_d_n13: f64 = (var_devsign * var_igidlb_dn13);
        let eq25_e2174_d_n14: f64 = (var_devsign * var_igidlb_dn14);
        (eq25_e2174, eq25_e2174_d_n0, eq25_e2174_d_n2, eq25_e2174_d_n3, eq25_e2174_d_n4, eq25_e2174_d_n5, eq25_e2174_d_n6, eq25_e2174_d_n7, eq25_e2174_d_n8, eq25_e2174_d_n9, eq25_e2174_d_n10, eq25_e2174_d_n11, eq25_e2174_d_n13, eq25_e2174_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e2176;
        let eq25_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq25_node_derivatives: [f64; 13] = [eq25_e2176_d_n0, eq25_e2176_d_n2, eq25_e2176_d_n3, eq25_e2176_d_n4, eq25_e2176_d_n5, eq25_e2176_d_n6, eq25_e2176_d_n7, eq25_e2176_d_n8, eq25_e2176_d_n9, eq25_e2176_d_n10, eq25_e2176_d_n11, eq25_e2176_d_n13, eq25_e2176_d_n14];
        let eq25_branch_derivative_indices: [usize; 0] = [];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq25_value),
            &eq25_node_derivative_indices,
            &eq25_node_derivatives,
            &eq25_branch_derivative_indices,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e2187, eq26_e2187_d_n0, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n13, eq26_e2187_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 != 0.0)) {
        let eq26_e2185: f64 = (var_devsign * var_igislb);
        let eq26_e2185_d_n0: f64 = (var_devsign * var_igislb_dn0);
        let eq26_e2185_d_n2: f64 = (var_devsign * var_igislb_dn2);
        let eq26_e2185_d_n3: f64 = (var_devsign * var_igislb_dn3);
        let eq26_e2185_d_n4: f64 = (var_devsign * var_igislb_dn4);
        let eq26_e2185_d_n5: f64 = (var_devsign * var_igislb_dn5);
        let eq26_e2185_d_n6: f64 = (var_devsign * var_igislb_dn6);
        let eq26_e2185_d_n7: f64 = (var_devsign * var_igislb_dn7);
        let eq26_e2185_d_n8: f64 = (var_devsign * var_igislb_dn8);
        let eq26_e2185_d_n9: f64 = (var_devsign * var_igislb_dn9);
        let eq26_e2185_d_n10: f64 = (var_devsign * var_igislb_dn10);
        let eq26_e2185_d_n11: f64 = (var_devsign * var_igislb_dn11);
        let eq26_e2185_d_n13: f64 = (var_devsign * var_igislb_dn13);
        let eq26_e2185_d_n14: f64 = (var_devsign * var_igislb_dn14);
        (eq26_e2185, eq26_e2185_d_n0, eq26_e2185_d_n2, eq26_e2185_d_n3, eq26_e2185_d_n4, eq26_e2185_d_n5, eq26_e2185_d_n6, eq26_e2185_d_n7, eq26_e2185_d_n8, eq26_e2185_d_n9, eq26_e2185_d_n10, eq26_e2185_d_n11, eq26_e2185_d_n13, eq26_e2185_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e2187;
        let eq26_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq26_node_derivatives: [f64; 13] = [eq26_e2187_d_n0, eq26_e2187_d_n2, eq26_e2187_d_n3, eq26_e2187_d_n4, eq26_e2187_d_n5, eq26_e2187_d_n6, eq26_e2187_d_n7, eq26_e2187_d_n8, eq26_e2187_d_n9, eq26_e2187_d_n10, eq26_e2187_d_n11, eq26_e2187_d_n13, eq26_e2187_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e2201, eq27_e2201_d_n0, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n13, eq27_e2201_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 == 0.0)) {
        let eq27_e2198: f64 = (var_igidl_v + var_iii_1);
        let eq27_e2198_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq27_e2198_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq27_e2198_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq27_e2198_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq27_e2198_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq27_e2198_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq27_e2198_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq27_e2198_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq27_e2198_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq27_e2198_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq27_e2198_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq27_e2198_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq27_e2198_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq27_e2199: f64 = (var_devsign * eq27_e2198);
        let eq27_e2199_d_n0: f64 = (var_devsign * eq27_e2198_d_n0);
        let eq27_e2199_d_n2: f64 = (var_devsign * eq27_e2198_d_n2);
        let eq27_e2199_d_n3: f64 = (var_devsign * eq27_e2198_d_n3);
        let eq27_e2199_d_n4: f64 = (var_devsign * eq27_e2198_d_n4);
        let eq27_e2199_d_n5: f64 = (var_devsign * eq27_e2198_d_n5);
        let eq27_e2199_d_n6: f64 = (var_devsign * eq27_e2198_d_n6);
        let eq27_e2199_d_n7: f64 = (var_devsign * eq27_e2198_d_n7);
        let eq27_e2199_d_n8: f64 = (var_devsign * eq27_e2198_d_n8);
        let eq27_e2199_d_n9: f64 = (var_devsign * eq27_e2198_d_n9);
        let eq27_e2199_d_n10: f64 = (var_devsign * eq27_e2198_d_n10);
        let eq27_e2199_d_n11: f64 = (var_devsign * eq27_e2198_d_n11);
        let eq27_e2199_d_n13: f64 = (var_devsign * eq27_e2198_d_n13);
        let eq27_e2199_d_n14: f64 = (var_devsign * eq27_e2198_d_n14);
        (eq27_e2199, eq27_e2199_d_n0, eq27_e2199_d_n2, eq27_e2199_d_n3, eq27_e2199_d_n4, eq27_e2199_d_n5, eq27_e2199_d_n6, eq27_e2199_d_n7, eq27_e2199_d_n8, eq27_e2199_d_n9, eq27_e2199_d_n10, eq27_e2199_d_n11, eq27_e2199_d_n13, eq27_e2199_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e2201;
        let eq27_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq27_node_derivatives: [f64; 13] = [eq27_e2201_d_n0, eq27_e2201_d_n2, eq27_e2201_d_n3, eq27_e2201_d_n4, eq27_e2201_d_n5, eq27_e2201_d_n6, eq27_e2201_d_n7, eq27_e2201_d_n8, eq27_e2201_d_n9, eq27_e2201_d_n10, eq27_e2201_d_n11, eq27_e2201_d_n13, eq27_e2201_d_n14];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(3),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e2213, eq28_e2213_d_n0, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n13, eq28_e2213_d_n14,) = {
    if (((var_guard644 == 0.0) && (var_guard647 != 0.0)) && (var_guard648 == 0.0)) {
        let eq28_e2211: f64 = (var_devsign * var_igisl_v);
        let eq28_e2211_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq28_e2211_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq28_e2211_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq28_e2211_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq28_e2211_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq28_e2211_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq28_e2211_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq28_e2211_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq28_e2211_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq28_e2211_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq28_e2211_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq28_e2211_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq28_e2211_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq28_e2211, eq28_e2211_d_n0, eq28_e2211_d_n2, eq28_e2211_d_n3, eq28_e2211_d_n4, eq28_e2211_d_n5, eq28_e2211_d_n6, eq28_e2211_d_n7, eq28_e2211_d_n8, eq28_e2211_d_n9, eq28_e2211_d_n10, eq28_e2211_d_n11, eq28_e2211_d_n13, eq28_e2211_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e2213;
        let eq28_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq28_node_derivatives: [f64; 13] = [eq28_e2213_d_n0, eq28_e2213_d_n2, eq28_e2213_d_n3, eq28_e2213_d_n4, eq28_e2213_d_n5, eq28_e2213_d_n6, eq28_e2213_d_n7, eq28_e2213_d_n8, eq28_e2213_d_n9, eq28_e2213_d_n10, eq28_e2213_d_n11, eq28_e2213_d_n13, eq28_e2213_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(3),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e2224, eq29_e2224_d_n0, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n13, eq29_e2224_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 != 0.0)) {
        let eq29_e2221: f64 = (var_igbinv_v + var_igbacc_v);
        let eq29_e2221_d_n0: f64 = (var_igbinv_v_dn0 + var_igbacc_v_dn0);
        let eq29_e2221_d_n2: f64 = (var_igbinv_v_dn2 + var_igbacc_v_dn2);
        let eq29_e2221_d_n3: f64 = (var_igbinv_v_dn3 + var_igbacc_v_dn3);
        let eq29_e2221_d_n4: f64 = (var_igbinv_v_dn4 + var_igbacc_v_dn4);
        let eq29_e2221_d_n5: f64 = (var_igbinv_v_dn5 + var_igbacc_v_dn5);
        let eq29_e2221_d_n6: f64 = (var_igbinv_v_dn6 + var_igbacc_v_dn6);
        let eq29_e2221_d_n7: f64 = (var_igbinv_v_dn7 + var_igbacc_v_dn7);
        let eq29_e2221_d_n8: f64 = (var_igbinv_v_dn8 + var_igbacc_v_dn8);
        let eq29_e2221_d_n9: f64 = (var_igbinv_v_dn9 + var_igbacc_v_dn9);
        let eq29_e2221_d_n10: f64 = (var_igbinv_v_dn10 + var_igbacc_v_dn10);
        let eq29_e2221_d_n11: f64 = (var_igbinv_v_dn11 + var_igbacc_v_dn11);
        let eq29_e2221_d_n13: f64 = (var_igbinv_v_dn13 + var_igbacc_v_dn13);
        let eq29_e2221_d_n14: f64 = (var_igbinv_v_dn14 + var_igbacc_v_dn14);
        let eq29_e2222: f64 = (var_devsign * eq29_e2221);
        let eq29_e2222_d_n0: f64 = (var_devsign * eq29_e2221_d_n0);
        let eq29_e2222_d_n2: f64 = (var_devsign * eq29_e2221_d_n2);
        let eq29_e2222_d_n3: f64 = (var_devsign * eq29_e2221_d_n3);
        let eq29_e2222_d_n4: f64 = (var_devsign * eq29_e2221_d_n4);
        let eq29_e2222_d_n5: f64 = (var_devsign * eq29_e2221_d_n5);
        let eq29_e2222_d_n6: f64 = (var_devsign * eq29_e2221_d_n6);
        let eq29_e2222_d_n7: f64 = (var_devsign * eq29_e2221_d_n7);
        let eq29_e2222_d_n8: f64 = (var_devsign * eq29_e2221_d_n8);
        let eq29_e2222_d_n9: f64 = (var_devsign * eq29_e2221_d_n9);
        let eq29_e2222_d_n10: f64 = (var_devsign * eq29_e2221_d_n10);
        let eq29_e2222_d_n11: f64 = (var_devsign * eq29_e2221_d_n11);
        let eq29_e2222_d_n13: f64 = (var_devsign * eq29_e2221_d_n13);
        let eq29_e2222_d_n14: f64 = (var_devsign * eq29_e2221_d_n14);
        (eq29_e2222, eq29_e2222_d_n0, eq29_e2222_d_n2, eq29_e2222_d_n3, eq29_e2222_d_n4, eq29_e2222_d_n5, eq29_e2222_d_n6, eq29_e2222_d_n7, eq29_e2222_d_n8, eq29_e2222_d_n9, eq29_e2222_d_n10, eq29_e2222_d_n11, eq29_e2222_d_n13, eq29_e2222_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e2224;
        let eq29_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq29_node_derivatives: [f64; 13] = [eq29_e2224_d_n0, eq29_e2224_d_n2, eq29_e2224_d_n3, eq29_e2224_d_n4, eq29_e2224_d_n5, eq29_e2224_d_n6, eq29_e2224_d_n7, eq29_e2224_d_n8, eq29_e2224_d_n9, eq29_e2224_d_n10, eq29_e2224_d_n11, eq29_e2224_d_n13, eq29_e2224_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(3),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let (eq30_e2236, eq30_e2236_d_n0, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n13, eq30_e2236_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 == 0.0)) {
        let eq30_e2233: f64 = (var_igidl_v + var_iii_1);
        let eq30_e2233_d_n0: f64 = (var_igidl_v_dn0 + var_iii_1_dn0);
        let eq30_e2233_d_n2: f64 = (var_igidl_v_dn2 + var_iii_1_dn2);
        let eq30_e2233_d_n3: f64 = (var_igidl_v_dn3 + var_iii_1_dn3);
        let eq30_e2233_d_n4: f64 = (var_igidl_v_dn4 + var_iii_1_dn4);
        let eq30_e2233_d_n5: f64 = (var_igidl_v_dn5 + var_iii_1_dn5);
        let eq30_e2233_d_n6: f64 = (var_igidl_v_dn6 + var_iii_1_dn6);
        let eq30_e2233_d_n7: f64 = (var_igidl_v_dn7 + var_iii_1_dn7);
        let eq30_e2233_d_n8: f64 = (var_igidl_v_dn8 + var_iii_1_dn8);
        let eq30_e2233_d_n9: f64 = (var_igidl_v_dn9 + var_iii_1_dn9);
        let eq30_e2233_d_n10: f64 = (var_igidl_v_dn10 + var_iii_1_dn10);
        let eq30_e2233_d_n11: f64 = (var_igidl_v_dn11 + var_iii_1_dn11);
        let eq30_e2233_d_n13: f64 = (var_igidl_v_dn13 + var_iii_1_dn13);
        let eq30_e2233_d_n14: f64 = (var_igidl_v_dn14 + var_iii_1_dn14);
        let eq30_e2234: f64 = (var_devsign * eq30_e2233);
        let eq30_e2234_d_n0: f64 = (var_devsign * eq30_e2233_d_n0);
        let eq30_e2234_d_n2: f64 = (var_devsign * eq30_e2233_d_n2);
        let eq30_e2234_d_n3: f64 = (var_devsign * eq30_e2233_d_n3);
        let eq30_e2234_d_n4: f64 = (var_devsign * eq30_e2233_d_n4);
        let eq30_e2234_d_n5: f64 = (var_devsign * eq30_e2233_d_n5);
        let eq30_e2234_d_n6: f64 = (var_devsign * eq30_e2233_d_n6);
        let eq30_e2234_d_n7: f64 = (var_devsign * eq30_e2233_d_n7);
        let eq30_e2234_d_n8: f64 = (var_devsign * eq30_e2233_d_n8);
        let eq30_e2234_d_n9: f64 = (var_devsign * eq30_e2233_d_n9);
        let eq30_e2234_d_n10: f64 = (var_devsign * eq30_e2233_d_n10);
        let eq30_e2234_d_n11: f64 = (var_devsign * eq30_e2233_d_n11);
        let eq30_e2234_d_n13: f64 = (var_devsign * eq30_e2233_d_n13);
        let eq30_e2234_d_n14: f64 = (var_devsign * eq30_e2233_d_n14);
        (eq30_e2234, eq30_e2234_d_n0, eq30_e2234_d_n2, eq30_e2234_d_n3, eq30_e2234_d_n4, eq30_e2234_d_n5, eq30_e2234_d_n6, eq30_e2234_d_n7, eq30_e2234_d_n8, eq30_e2234_d_n9, eq30_e2234_d_n10, eq30_e2234_d_n11, eq30_e2234_d_n13, eq30_e2234_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e2236;
        let eq30_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq30_node_derivatives: [f64; 13] = [eq30_e2236_d_n0, eq30_e2236_d_n2, eq30_e2236_d_n3, eq30_e2236_d_n4, eq30_e2236_d_n5, eq30_e2236_d_n6, eq30_e2236_d_n7, eq30_e2236_d_n8, eq30_e2236_d_n9, eq30_e2236_d_n10, eq30_e2236_d_n11, eq30_e2236_d_n13, eq30_e2236_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e2246, eq31_e2246_d_n0, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n13, eq31_e2246_d_n14,) = {
    if ((var_guard644 == 0.0) && (var_guard647 == 0.0)) {
        let eq31_e2244: f64 = (var_devsign * var_igisl_v);
        let eq31_e2244_d_n0: f64 = (var_devsign * var_igisl_v_dn0);
        let eq31_e2244_d_n2: f64 = (var_devsign * var_igisl_v_dn2);
        let eq31_e2244_d_n3: f64 = (var_devsign * var_igisl_v_dn3);
        let eq31_e2244_d_n4: f64 = (var_devsign * var_igisl_v_dn4);
        let eq31_e2244_d_n5: f64 = (var_devsign * var_igisl_v_dn5);
        let eq31_e2244_d_n6: f64 = (var_devsign * var_igisl_v_dn6);
        let eq31_e2244_d_n7: f64 = (var_devsign * var_igisl_v_dn7);
        let eq31_e2244_d_n8: f64 = (var_devsign * var_igisl_v_dn8);
        let eq31_e2244_d_n9: f64 = (var_devsign * var_igisl_v_dn9);
        let eq31_e2244_d_n10: f64 = (var_devsign * var_igisl_v_dn10);
        let eq31_e2244_d_n11: f64 = (var_devsign * var_igisl_v_dn11);
        let eq31_e2244_d_n13: f64 = (var_devsign * var_igisl_v_dn13);
        let eq31_e2244_d_n14: f64 = (var_devsign * var_igisl_v_dn14);
        (eq31_e2244, eq31_e2244_d_n0, eq31_e2244_d_n2, eq31_e2244_d_n3, eq31_e2244_d_n4, eq31_e2244_d_n5, eq31_e2244_d_n6, eq31_e2244_d_n7, eq31_e2244_d_n8, eq31_e2244_d_n9, eq31_e2244_d_n10, eq31_e2244_d_n11, eq31_e2244_d_n13, eq31_e2244_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e2246;
        let eq31_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq31_node_derivatives: [f64; 13] = [eq31_e2246_d_n0, eq31_e2246_d_n2, eq31_e2246_d_n3, eq31_e2246_d_n4, eq31_e2246_d_n5, eq31_e2246_d_n6, eq31_e2246_d_n7, eq31_e2246_d_n8, eq31_e2246_d_n9, eq31_e2246_d_n10, eq31_e2246_d_n11, eq31_e2246_d_n13, eq31_e2246_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e2252, eq32_e2252_d_n0, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n13, eq32_e2252_d_n14,) = {
    if (var_guard649 != 0.0) {
        let eq32_e2250: f64 = (var_devsign * var_igbs_v);
        let eq32_e2250_d_n0: f64 = (var_devsign * var_igbs_v_dn0);
        let eq32_e2250_d_n2: f64 = (var_devsign * var_igbs_v_dn2);
        let eq32_e2250_d_n3: f64 = (var_devsign * var_igbs_v_dn3);
        let eq32_e2250_d_n4: f64 = (var_devsign * var_igbs_v_dn4);
        let eq32_e2250_d_n5: f64 = (var_devsign * var_igbs_v_dn5);
        let eq32_e2250_d_n6: f64 = (var_devsign * var_igbs_v_dn6);
        let eq32_e2250_d_n7: f64 = (var_devsign * var_igbs_v_dn7);
        let eq32_e2250_d_n8: f64 = (var_devsign * var_igbs_v_dn8);
        let eq32_e2250_d_n9: f64 = (var_devsign * var_igbs_v_dn9);
        let eq32_e2250_d_n10: f64 = (var_devsign * var_igbs_v_dn10);
        let eq32_e2250_d_n11: f64 = (var_devsign * var_igbs_v_dn11);
        let eq32_e2250_d_n13: f64 = (var_devsign * var_igbs_v_dn13);
        let eq32_e2250_d_n14: f64 = (var_devsign * var_igbs_v_dn14);
        (eq32_e2250, eq32_e2250_d_n0, eq32_e2250_d_n2, eq32_e2250_d_n3, eq32_e2250_d_n4, eq32_e2250_d_n5, eq32_e2250_d_n6, eq32_e2250_d_n7, eq32_e2250_d_n8, eq32_e2250_d_n9, eq32_e2250_d_n10, eq32_e2250_d_n11, eq32_e2250_d_n13, eq32_e2250_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e2252;
        let eq32_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq32_node_derivatives: [f64; 13] = [eq32_e2252_d_n0, eq32_e2252_d_n2, eq32_e2252_d_n3, eq32_e2252_d_n4, eq32_e2252_d_n5, eq32_e2252_d_n6, eq32_e2252_d_n7, eq32_e2252_d_n8, eq32_e2252_d_n9, eq32_e2252_d_n10, eq32_e2252_d_n11, eq32_e2252_d_n13, eq32_e2252_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e2258, eq33_e2258_d_n0, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n13, eq33_e2258_d_n14,) = {
    if (var_guard649 != 0.0) {
        let eq33_e2256: f64 = (var_devsign * var_igbd_v);
        let eq33_e2256_d_n0: f64 = (var_devsign * var_igbd_v_dn0);
        let eq33_e2256_d_n2: f64 = (var_devsign * var_igbd_v_dn2);
        let eq33_e2256_d_n3: f64 = (var_devsign * var_igbd_v_dn3);
        let eq33_e2256_d_n4: f64 = (var_devsign * var_igbd_v_dn4);
        let eq33_e2256_d_n5: f64 = (var_devsign * var_igbd_v_dn5);
        let eq33_e2256_d_n6: f64 = (var_devsign * var_igbd_v_dn6);
        let eq33_e2256_d_n7: f64 = (var_devsign * var_igbd_v_dn7);
        let eq33_e2256_d_n8: f64 = (var_devsign * var_igbd_v_dn8);
        let eq33_e2256_d_n9: f64 = (var_devsign * var_igbd_v_dn9);
        let eq33_e2256_d_n10: f64 = (var_devsign * var_igbd_v_dn10);
        let eq33_e2256_d_n11: f64 = (var_devsign * var_igbd_v_dn11);
        let eq33_e2256_d_n13: f64 = (var_devsign * var_igbd_v_dn13);
        let eq33_e2256_d_n14: f64 = (var_devsign * var_igbd_v_dn14);
        (eq33_e2256, eq33_e2256_d_n0, eq33_e2256_d_n2, eq33_e2256_d_n3, eq33_e2256_d_n4, eq33_e2256_d_n5, eq33_e2256_d_n6, eq33_e2256_d_n7, eq33_e2256_d_n8, eq33_e2256_d_n9, eq33_e2256_d_n10, eq33_e2256_d_n11, eq33_e2256_d_n13, eq33_e2256_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e2258;
        let eq33_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq33_node_derivatives: [f64; 13] = [eq33_e2258_d_n0, eq33_e2258_d_n2, eq33_e2258_d_n3, eq33_e2258_d_n4, eq33_e2258_d_n5, eq33_e2258_d_n6, eq33_e2258_d_n7, eq33_e2258_d_n8, eq33_e2258_d_n9, eq33_e2258_d_n10, eq33_e2258_d_n11, eq33_e2258_d_n13, eq33_e2258_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e2268, eq34_e2268_d_n0, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n13, eq34_e2268_d_n14,) = {
    if (var_guard650 != 0.0) {
        let eq34_e2262: f64 = (var_devsign * var_ies);
        let eq34_e2262_d_n0: f64 = (var_devsign * var_ies_dn0);
        let eq34_e2262_d_n2: f64 = (var_devsign * var_ies_dn2);
        let eq34_e2262_d_n3: f64 = (var_devsign * var_ies_dn3);
        let eq34_e2262_d_n4: f64 = (var_devsign * var_ies_dn4);
        let eq34_e2262_d_n5: f64 = (var_devsign * var_ies_dn5);
        let eq34_e2262_d_n6: f64 = (var_devsign * var_ies_dn6);
        let eq34_e2262_d_n7: f64 = (var_devsign * var_ies_dn7);
        let eq34_e2262_d_n8: f64 = (var_devsign * var_ies_dn8);
        let eq34_e2262_d_n9: f64 = (var_devsign * var_ies_dn9);
        let eq34_e2262_d_n10: f64 = (var_devsign * var_ies_dn10);
        let eq34_e2262_d_n11: f64 = (var_devsign * var_ies_dn11);
        let eq34_e2262_d_n13: f64 = (var_devsign * var_ies_dn13);
        let eq34_e2262_d_n14: f64 = (var_devsign * var_ies_dn14);
        let eq34_e2265: f64 = ((nv3 - nv6) * var_gmin);
        let eq34_e2266: f64 = (eq34_e2262 + eq34_e2265);
        let eq34_e2266_d_n3: f64 = (eq34_e2262_d_n3 + var_gmin);
        let eq34_e2266_d_n6: f64 = (eq34_e2262_d_n6 + (-var_gmin));
        (eq34_e2266, eq34_e2262_d_n0, eq34_e2262_d_n2, eq34_e2266_d_n3, eq34_e2262_d_n4, eq34_e2262_d_n5, eq34_e2266_d_n6, eq34_e2262_d_n7, eq34_e2262_d_n8, eq34_e2262_d_n9, eq34_e2262_d_n10, eq34_e2262_d_n11, eq34_e2262_d_n13, eq34_e2262_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e2268;
        let eq34_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq34_node_derivatives: [f64; 13] = [eq34_e2268_d_n0, eq34_e2268_d_n2, eq34_e2268_d_n3, eq34_e2268_d_n4, eq34_e2268_d_n5, eq34_e2268_d_n6, eq34_e2268_d_n7, eq34_e2268_d_n8, eq34_e2268_d_n9, eq34_e2268_d_n10, eq34_e2268_d_n11, eq34_e2268_d_n13, eq34_e2268_d_n14];
        let eq34_branch_derivative_indices: [usize; 0] = [];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivative_indices,
            &eq34_node_derivatives,
            &eq34_branch_derivative_indices,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq35_e2278, eq35_e2278_d_n0, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n13, eq35_e2278_d_n14,) = {
    if (var_guard650 != 0.0) {
        let eq35_e2272: f64 = (var_devsign * var_ied);
        let eq35_e2272_d_n0: f64 = (var_devsign * var_ied_dn0);
        let eq35_e2272_d_n2: f64 = (var_devsign * var_ied_dn2);
        let eq35_e2272_d_n3: f64 = (var_devsign * var_ied_dn3);
        let eq35_e2272_d_n4: f64 = (var_devsign * var_ied_dn4);
        let eq35_e2272_d_n5: f64 = (var_devsign * var_ied_dn5);
        let eq35_e2272_d_n6: f64 = (var_devsign * var_ied_dn6);
        let eq35_e2272_d_n7: f64 = (var_devsign * var_ied_dn7);
        let eq35_e2272_d_n8: f64 = (var_devsign * var_ied_dn8);
        let eq35_e2272_d_n9: f64 = (var_devsign * var_ied_dn9);
        let eq35_e2272_d_n10: f64 = (var_devsign * var_ied_dn10);
        let eq35_e2272_d_n11: f64 = (var_devsign * var_ied_dn11);
        let eq35_e2272_d_n13: f64 = (var_devsign * var_ied_dn13);
        let eq35_e2272_d_n14: f64 = (var_devsign * var_ied_dn14);
        let eq35_e2275: f64 = ((nv3 - nv5) * var_gmin);
        let eq35_e2276: f64 = (eq35_e2272 + eq35_e2275);
        let eq35_e2276_d_n3: f64 = (eq35_e2272_d_n3 + var_gmin);
        let eq35_e2276_d_n5: f64 = (eq35_e2272_d_n5 + (-var_gmin));
        (eq35_e2276, eq35_e2272_d_n0, eq35_e2272_d_n2, eq35_e2276_d_n3, eq35_e2272_d_n4, eq35_e2276_d_n5, eq35_e2272_d_n6, eq35_e2272_d_n7, eq35_e2272_d_n8, eq35_e2272_d_n9, eq35_e2272_d_n10, eq35_e2272_d_n11, eq35_e2272_d_n13, eq35_e2272_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e2278;
        let eq35_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq35_node_derivatives: [f64; 13] = [eq35_e2278_d_n0, eq35_e2278_d_n2, eq35_e2278_d_n3, eq35_e2278_d_n4, eq35_e2278_d_n5, eq35_e2278_d_n6, eq35_e2278_d_n7, eq35_e2278_d_n8, eq35_e2278_d_n9, eq35_e2278_d_n10, eq35_e2278_d_n11, eq35_e2278_d_n13, eq35_e2278_d_n14];
        let eq35_branch_derivative_indices: [usize; 0] = [];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivative_indices,
            &eq35_node_derivatives,
            &eq35_branch_derivative_indices,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qes);
        let eq36_e2282: f64 = (var_devsign * eq36_e2281);
        let eq36_e2282_d_n0: f64 = (var_devsign * (var_qes_dn0 * ddt_scale));
        let eq36_e2282_d_n2: f64 = (var_devsign * (var_qes_dn2 * ddt_scale));
        let eq36_e2282_d_n3: f64 = (var_devsign * (var_qes_dn3 * ddt_scale));
        let eq36_e2282_d_n4: f64 = (var_devsign * (var_qes_dn4 * ddt_scale));
        let eq36_e2282_d_n5: f64 = (var_devsign * (var_qes_dn5 * ddt_scale));
        let eq36_e2282_d_n6: f64 = (var_devsign * (var_qes_dn6 * ddt_scale));
        let eq36_e2282_d_n7: f64 = (var_devsign * (var_qes_dn7 * ddt_scale));
        let eq36_e2282_d_n8: f64 = (var_devsign * (var_qes_dn8 * ddt_scale));
        let eq36_e2282_d_n9: f64 = (var_devsign * (var_qes_dn9 * ddt_scale));
        let eq36_e2282_d_n10: f64 = (var_devsign * (var_qes_dn10 * ddt_scale));
        let eq36_e2282_d_n11: f64 = (var_devsign * (var_qes_dn11 * ddt_scale));
        let eq36_e2282_d_n13: f64 = (var_devsign * (var_qes_dn13 * ddt_scale));
        let eq36_e2282_d_n14: f64 = (var_devsign * (var_qes_dn14 * ddt_scale));
        let eq36_value: f64 = eq36_e2282;
        let eq36_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq36_node_derivatives: [f64; 13] = [eq36_e2282_d_n0, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, eq36_e2282_d_n13, eq36_e2282_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
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
        var_devsign: f64,
        var_guard651: f64,
        var_guard652: f64,
        var_guard653: f64,
        var_guard654: f64,
        var_guard655: f64,
        var_guard656: f64,
        var_qbov: f64,
        var_qbov_dn0: f64,
        var_qbov_dn10: f64,
        var_qbov_dn11: f64,
        var_qbov_dn13: f64,
        var_qbov_dn14: f64,
        var_qbov_dn2: f64,
        var_qbov_dn3: f64,
        var_qbov_dn4: f64,
        var_qbov_dn5: f64,
        var_qbov_dn6: f64,
        var_qbov_dn7: f64,
        var_qbov_dn8: f64,
        var_qbov_dn9: f64,
        var_qbov_s: f64,
        var_qbov_s_dn0: f64,
        var_qbov_s_dn10: f64,
        var_qbov_s_dn11: f64,
        var_qbov_s_dn13: f64,
        var_qbov_s_dn14: f64,
        var_qbov_s_dn2: f64,
        var_qbov_s_dn3: f64,
        var_qbov_s_dn4: f64,
        var_qbov_s_dn5: f64,
        var_qbov_s_dn6: f64,
        var_qbov_s_dn7: f64,
        var_qbov_s_dn8: f64,
        var_qbov_s_dn9: f64,
        var_qds_fr: f64,
        var_qds_fr_dn0: f64,
        var_qds_fr_dn2: f64,
        var_qed: f64,
        var_qed_dn0: f64,
        var_qed_dn10: f64,
        var_qed_dn11: f64,
        var_qed_dn13: f64,
        var_qed_dn14: f64,
        var_qed_dn2: f64,
        var_qed_dn3: f64,
        var_qed_dn4: f64,
        var_qed_dn5: f64,
        var_qed_dn6: f64,
        var_qed_dn7: f64,
        var_qed_dn8: f64,
        var_qed_dn9: f64,
        var_qeg: f64,
        var_qeg_dn0: f64,
        var_qeg_dn10: f64,
        var_qeg_dn11: f64,
        var_qeg_dn13: f64,
        var_qeg_dn14: f64,
        var_qeg_dn2: f64,
        var_qeg_dn3: f64,
        var_qeg_dn4: f64,
        var_qeg_dn5: f64,
        var_qeg_dn6: f64,
        var_qeg_dn7: f64,
        var_qeg_dn8: f64,
        var_qeg_dn9: f64,
        var_qg_acc: f64,
        var_qg_acc_dn0: f64,
        var_qg_acc_dn10: f64,
        var_qg_acc_dn11: f64,
        var_qg_acc_dn13: f64,
        var_qg_acc_dn14: f64,
        var_qg_acc_dn2: f64,
        var_qg_acc_dn3: f64,
        var_qg_acc_dn4: f64,
        var_qg_acc_dn5: f64,
        var_qg_acc_dn6: f64,
        var_qg_acc_dn7: f64,
        var_qg_acc_dn8: f64,
        var_qg_acc_dn9: f64,
        var_qgd_fr: f64,
        var_qgd_fr_dn0: f64,
        var_qgd_fr_dn10: f64,
        var_qgd_fr_dn11: f64,
        var_qgd_fr_dn13: f64,
        var_qgd_fr_dn14: f64,
        var_qgd_fr_dn2: f64,
        var_qgd_fr_dn3: f64,
        var_qgd_fr_dn4: f64,
        var_qgd_fr_dn5: f64,
        var_qgd_fr_dn6: f64,
        var_qgd_fr_dn7: f64,
        var_qgd_fr_dn8: f64,
        var_qgd_fr_dn9: f64,
        var_qgd_parasitic: f64,
        var_qgd_parasitic_dn0: f64,
        var_qgd_parasitic_dn10: f64,
        var_qgd_parasitic_dn11: f64,
        var_qgd_parasitic_dn13: f64,
        var_qgd_parasitic_dn14: f64,
        var_qgd_parasitic_dn2: f64,
        var_qgd_parasitic_dn3: f64,
        var_qgd_parasitic_dn4: f64,
        var_qgd_parasitic_dn5: f64,
        var_qgd_parasitic_dn6: f64,
        var_qgd_parasitic_dn7: f64,
        var_qgd_parasitic_dn8: f64,
        var_qgd_parasitic_dn9: f64,
        var_qgs_fr: f64,
        var_qgs_fr_dn0: f64,
        var_qgs_fr_dn10: f64,
        var_qgs_fr_dn11: f64,
        var_qgs_fr_dn13: f64,
        var_qgs_fr_dn14: f64,
        var_qgs_fr_dn2: f64,
        var_qgs_fr_dn3: f64,
        var_qgs_fr_dn4: f64,
        var_qgs_fr_dn5: f64,
        var_qgs_fr_dn6: f64,
        var_qgs_fr_dn7: f64,
        var_qgs_fr_dn8: f64,
        var_qgs_fr_dn9: f64,
        var_qgs_parasitic: f64,
        var_qgs_parasitic_dn0: f64,
        var_qgs_parasitic_dn10: f64,
        var_qgs_parasitic_dn11: f64,
        var_qgs_parasitic_dn13: f64,
        var_qgs_parasitic_dn14: f64,
        var_qgs_parasitic_dn2: f64,
        var_qgs_parasitic_dn3: f64,
        var_qgs_parasitic_dn4: f64,
        var_qgs_parasitic_dn5: f64,
        var_qgs_parasitic_dn6: f64,
        var_qgs_parasitic_dn7: f64,
        var_qgs_parasitic_dn8: f64,
        var_qgs_parasitic_dn9: f64,
    ) {
        let eq37_e2285: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qed);
        let eq37_e2286: f64 = (var_devsign * eq37_e2285);
        let eq37_e2286_d_n0: f64 = (var_devsign * (var_qed_dn0 * ddt_scale));
        let eq37_e2286_d_n2: f64 = (var_devsign * (var_qed_dn2 * ddt_scale));
        let eq37_e2286_d_n3: f64 = (var_devsign * (var_qed_dn3 * ddt_scale));
        let eq37_e2286_d_n4: f64 = (var_devsign * (var_qed_dn4 * ddt_scale));
        let eq37_e2286_d_n5: f64 = (var_devsign * (var_qed_dn5 * ddt_scale));
        let eq37_e2286_d_n6: f64 = (var_devsign * (var_qed_dn6 * ddt_scale));
        let eq37_e2286_d_n7: f64 = (var_devsign * (var_qed_dn7 * ddt_scale));
        let eq37_e2286_d_n8: f64 = (var_devsign * (var_qed_dn8 * ddt_scale));
        let eq37_e2286_d_n9: f64 = (var_devsign * (var_qed_dn9 * ddt_scale));
        let eq37_e2286_d_n10: f64 = (var_devsign * (var_qed_dn10 * ddt_scale));
        let eq37_e2286_d_n11: f64 = (var_devsign * (var_qed_dn11 * ddt_scale));
        let eq37_e2286_d_n13: f64 = (var_devsign * (var_qed_dn13 * ddt_scale));
        let eq37_e2286_d_n14: f64 = (var_devsign * (var_qed_dn14 * ddt_scale));
        let eq37_value: f64 = eq37_e2286;
        let eq37_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq37_node_derivatives: [f64; 13] = [eq37_e2286_d_n0, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, eq37_e2286_d_n13, eq37_e2286_d_n14];
        let eq37_branch_derivative_indices: [usize; 0] = [];
        let eq37_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(5),
            multiplicity * (eq37_value),
            &eq37_node_derivative_indices,
            &eq37_node_derivatives,
            &eq37_branch_derivative_indices,
            &eq37_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qeg);
        let eq38_e2290: f64 = (var_devsign * eq38_e2289);
        let eq38_e2290_d_n0: f64 = (var_devsign * (var_qeg_dn0 * ddt_scale));
        let eq38_e2290_d_n2: f64 = (var_devsign * (var_qeg_dn2 * ddt_scale));
        let eq38_e2290_d_n3: f64 = (var_devsign * (var_qeg_dn3 * ddt_scale));
        let eq38_e2290_d_n4: f64 = (var_devsign * (var_qeg_dn4 * ddt_scale));
        let eq38_e2290_d_n5: f64 = (var_devsign * (var_qeg_dn5 * ddt_scale));
        let eq38_e2290_d_n6: f64 = (var_devsign * (var_qeg_dn6 * ddt_scale));
        let eq38_e2290_d_n7: f64 = (var_devsign * (var_qeg_dn7 * ddt_scale));
        let eq38_e2290_d_n8: f64 = (var_devsign * (var_qeg_dn8 * ddt_scale));
        let eq38_e2290_d_n9: f64 = (var_devsign * (var_qeg_dn9 * ddt_scale));
        let eq38_e2290_d_n10: f64 = (var_devsign * (var_qeg_dn10 * ddt_scale));
        let eq38_e2290_d_n11: f64 = (var_devsign * (var_qeg_dn11 * ddt_scale));
        let eq38_e2290_d_n13: f64 = (var_devsign * (var_qeg_dn13 * ddt_scale));
        let eq38_e2290_d_n14: f64 = (var_devsign * (var_qeg_dn14 * ddt_scale));
        let eq38_value: f64 = eq38_e2290;
        let eq38_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq38_node_derivatives: [f64; 13] = [eq38_e2290_d_n0, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, eq38_e2290_d_n13, eq38_e2290_d_n14];
        let eq38_branch_derivative_indices: [usize; 0] = [];
        let eq38_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq38_value),
            &eq38_node_derivative_indices,
            &eq38_node_derivatives,
            &eq38_branch_derivative_indices,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14,) = {
    if (var_guard651 != 0.0) {
        let eq39_e2293: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, var_qgs_parasitic);
        (eq39_e2293, (var_qgs_parasitic_dn0 * ddt_scale), (var_qgs_parasitic_dn2 * ddt_scale), (var_qgs_parasitic_dn3 * ddt_scale), (var_qgs_parasitic_dn4 * ddt_scale), (var_qgs_parasitic_dn5 * ddt_scale), (var_qgs_parasitic_dn6 * ddt_scale), (var_qgs_parasitic_dn7 * ddt_scale), (var_qgs_parasitic_dn8 * ddt_scale), (var_qgs_parasitic_dn9 * ddt_scale), (var_qgs_parasitic_dn10 * ddt_scale), (var_qgs_parasitic_dn11 * ddt_scale), (var_qgs_parasitic_dn13 * ddt_scale), (var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_value: f64 = eq39_e2295;
        let eq39_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq39_node_derivatives: [f64; 13] = [eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq40_e2300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qgd_parasitic);
        (eq40_e2300, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_value: f64 = eq40_e2302;
        let eq40_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq40_node_derivatives: [f64; 13] = [eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq41_e2308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, var_qbov);
        let eq41_e2309: f64 = (var_devsign * eq41_e2308);
        let eq41_e2309_d_n0: f64 = (var_devsign * (var_qbov_dn0 * ddt_scale));
        let eq41_e2309_d_n2: f64 = (var_devsign * (var_qbov_dn2 * ddt_scale));
        let eq41_e2309_d_n3: f64 = (var_devsign * (var_qbov_dn3 * ddt_scale));
        let eq41_e2309_d_n4: f64 = (var_devsign * (var_qbov_dn4 * ddt_scale));
        let eq41_e2309_d_n5: f64 = (var_devsign * (var_qbov_dn5 * ddt_scale));
        let eq41_e2309_d_n6: f64 = (var_devsign * (var_qbov_dn6 * ddt_scale));
        let eq41_e2309_d_n7: f64 = (var_devsign * (var_qbov_dn7 * ddt_scale));
        let eq41_e2309_d_n8: f64 = (var_devsign * (var_qbov_dn8 * ddt_scale));
        let eq41_e2309_d_n9: f64 = (var_devsign * (var_qbov_dn9 * ddt_scale));
        let eq41_e2309_d_n10: f64 = (var_devsign * (var_qbov_dn10 * ddt_scale));
        let eq41_e2309_d_n11: f64 = (var_devsign * (var_qbov_dn11 * ddt_scale));
        let eq41_e2309_d_n13: f64 = (var_devsign * (var_qbov_dn13 * ddt_scale));
        let eq41_e2309_d_n14: f64 = (var_devsign * (var_qbov_dn14 * ddt_scale));
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n13, eq41_e2309_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_value: f64 = eq41_e2311;
        let eq41_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq41_node_derivatives: [f64; 13] = [eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq42_e2317: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, var_qbov_s);
        let eq42_e2318: f64 = (var_devsign * eq42_e2317);
        let eq42_e2318_d_n0: f64 = (var_devsign * (var_qbov_s_dn0 * ddt_scale));
        let eq42_e2318_d_n2: f64 = (var_devsign * (var_qbov_s_dn2 * ddt_scale));
        let eq42_e2318_d_n3: f64 = (var_devsign * (var_qbov_s_dn3 * ddt_scale));
        let eq42_e2318_d_n4: f64 = (var_devsign * (var_qbov_s_dn4 * ddt_scale));
        let eq42_e2318_d_n5: f64 = (var_devsign * (var_qbov_s_dn5 * ddt_scale));
        let eq42_e2318_d_n6: f64 = (var_devsign * (var_qbov_s_dn6 * ddt_scale));
        let eq42_e2318_d_n7: f64 = (var_devsign * (var_qbov_s_dn7 * ddt_scale));
        let eq42_e2318_d_n8: f64 = (var_devsign * (var_qbov_s_dn8 * ddt_scale));
        let eq42_e2318_d_n9: f64 = (var_devsign * (var_qbov_s_dn9 * ddt_scale));
        let eq42_e2318_d_n10: f64 = (var_devsign * (var_qbov_s_dn10 * ddt_scale));
        let eq42_e2318_d_n11: f64 = (var_devsign * (var_qbov_s_dn11 * ddt_scale));
        let eq42_e2318_d_n13: f64 = (var_devsign * (var_qbov_s_dn13 * ddt_scale));
        let eq42_e2318_d_n14: f64 = (var_devsign * (var_qbov_s_dn14 * ddt_scale));
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n13, eq42_e2318_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e2320;
        let eq42_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq42_node_derivatives: [f64; 13] = [eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard652 == 0.0)) {
        let eq43_e2326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, var_qgd_parasitic);
        (eq43_e2326, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_value: f64 = eq43_e2328;
        let eq43_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq43_node_derivatives: [f64; 13] = [eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq43_value),
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n2,) = {
    if (var_guard651 != 0.0) {
        let eq44_e2331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qds_fr);
        (eq44_e2331, (var_qds_fr_dn0 * ddt_scale), (var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e2333;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq44_value),
            0,
            multiplicity * (eq44_e2333_d_n0),
            2,
            multiplicity * (eq44_e2333_d_n2),
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq45_e2338: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qgs_fr);
        (eq45_e2338, (var_qgs_fr_dn0 * ddt_scale), (var_qgs_fr_dn2 * ddt_scale), (var_qgs_fr_dn3 * ddt_scale), (var_qgs_fr_dn4 * ddt_scale), (var_qgs_fr_dn5 * ddt_scale), (var_qgs_fr_dn6 * ddt_scale), (var_qgs_fr_dn7 * ddt_scale), (var_qgs_fr_dn8 * ddt_scale), (var_qgs_fr_dn9 * ddt_scale), (var_qgs_fr_dn10 * ddt_scale), (var_qgs_fr_dn11 * ddt_scale), (var_qgs_fr_dn13 * ddt_scale), (var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_value: f64 = eq45_e2340;
        let eq45_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq45_node_derivatives: [f64; 13] = [eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(2),
            multiplicity * (eq45_value),
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq46_e2345: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qgd_fr);
        (eq46_e2345, (var_qgd_fr_dn0 * ddt_scale), (var_qgd_fr_dn2 * ddt_scale), (var_qgd_fr_dn3 * ddt_scale), (var_qgd_fr_dn4 * ddt_scale), (var_qgd_fr_dn5 * ddt_scale), (var_qgd_fr_dn6 * ddt_scale), (var_qgd_fr_dn7 * ddt_scale), (var_qgd_fr_dn8 * ddt_scale), (var_qgd_fr_dn9 * ddt_scale), (var_qgd_fr_dn10 * ddt_scale), (var_qgd_fr_dn11 * ddt_scale), (var_qgd_fr_dn13 * ddt_scale), (var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e2347;
        let eq46_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq46_node_derivatives: [f64; 13] = [eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14];
        let eq46_branch_derivative_indices: [usize; 0] = [];
        let eq46_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq46_value),
            &eq46_node_derivative_indices,
            &eq46_node_derivatives,
            &eq46_branch_derivative_indices,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14,) = {
    if (var_guard651 == 0.0) {
        let eq47_e2351: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qgs_parasitic);
        (eq47_e2351, (var_qgs_parasitic_dn0 * ddt_scale), (var_qgs_parasitic_dn2 * ddt_scale), (var_qgs_parasitic_dn3 * ddt_scale), (var_qgs_parasitic_dn4 * ddt_scale), (var_qgs_parasitic_dn5 * ddt_scale), (var_qgs_parasitic_dn6 * ddt_scale), (var_qgs_parasitic_dn7 * ddt_scale), (var_qgs_parasitic_dn8 * ddt_scale), (var_qgs_parasitic_dn9 * ddt_scale), (var_qgs_parasitic_dn10 * ddt_scale), (var_qgs_parasitic_dn11 * ddt_scale), (var_qgs_parasitic_dn13 * ddt_scale), (var_qgs_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e2353;
        let eq47_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq47_node_derivatives: [f64; 13] = [eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(6),
            multiplicity * (eq47_value),
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq48_e2359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qgd_parasitic);
        (eq48_e2359, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e2361;
        let eq48_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq48_node_derivatives: [f64; 13] = [eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14];
        let eq48_branch_derivative_indices: [usize; 0] = [];
        let eq48_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq48_value),
            &eq48_node_derivative_indices,
            &eq48_node_derivatives,
            &eq48_branch_derivative_indices,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq49_e2368: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, var_qbov);
        let eq49_e2369: f64 = (var_devsign * eq49_e2368);
        let eq49_e2369_d_n0: f64 = (var_devsign * (var_qbov_dn0 * ddt_scale));
        let eq49_e2369_d_n2: f64 = (var_devsign * (var_qbov_dn2 * ddt_scale));
        let eq49_e2369_d_n3: f64 = (var_devsign * (var_qbov_dn3 * ddt_scale));
        let eq49_e2369_d_n4: f64 = (var_devsign * (var_qbov_dn4 * ddt_scale));
        let eq49_e2369_d_n5: f64 = (var_devsign * (var_qbov_dn5 * ddt_scale));
        let eq49_e2369_d_n6: f64 = (var_devsign * (var_qbov_dn6 * ddt_scale));
        let eq49_e2369_d_n7: f64 = (var_devsign * (var_qbov_dn7 * ddt_scale));
        let eq49_e2369_d_n8: f64 = (var_devsign * (var_qbov_dn8 * ddt_scale));
        let eq49_e2369_d_n9: f64 = (var_devsign * (var_qbov_dn9 * ddt_scale));
        let eq49_e2369_d_n10: f64 = (var_devsign * (var_qbov_dn10 * ddt_scale));
        let eq49_e2369_d_n11: f64 = (var_devsign * (var_qbov_dn11 * ddt_scale));
        let eq49_e2369_d_n13: f64 = (var_devsign * (var_qbov_dn13 * ddt_scale));
        let eq49_e2369_d_n14: f64 = (var_devsign * (var_qbov_dn14 * ddt_scale));
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n13, eq49_e2369_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_value: f64 = eq49_e2371;
        let eq49_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq49_node_derivatives: [f64; 13] = [eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(7),
            multiplicity * (eq49_value),
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq50_e2378: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, var_qbov_s);
        let eq50_e2379: f64 = (var_devsign * eq50_e2378);
        let eq50_e2379_d_n0: f64 = (var_devsign * (var_qbov_s_dn0 * ddt_scale));
        let eq50_e2379_d_n2: f64 = (var_devsign * (var_qbov_s_dn2 * ddt_scale));
        let eq50_e2379_d_n3: f64 = (var_devsign * (var_qbov_s_dn3 * ddt_scale));
        let eq50_e2379_d_n4: f64 = (var_devsign * (var_qbov_s_dn4 * ddt_scale));
        let eq50_e2379_d_n5: f64 = (var_devsign * (var_qbov_s_dn5 * ddt_scale));
        let eq50_e2379_d_n6: f64 = (var_devsign * (var_qbov_s_dn6 * ddt_scale));
        let eq50_e2379_d_n7: f64 = (var_devsign * (var_qbov_s_dn7 * ddt_scale));
        let eq50_e2379_d_n8: f64 = (var_devsign * (var_qbov_s_dn8 * ddt_scale));
        let eq50_e2379_d_n9: f64 = (var_devsign * (var_qbov_s_dn9 * ddt_scale));
        let eq50_e2379_d_n10: f64 = (var_devsign * (var_qbov_s_dn10 * ddt_scale));
        let eq50_e2379_d_n11: f64 = (var_devsign * (var_qbov_s_dn11 * ddt_scale));
        let eq50_e2379_d_n13: f64 = (var_devsign * (var_qbov_s_dn13 * ddt_scale));
        let eq50_e2379_d_n14: f64 = (var_devsign * (var_qbov_s_dn14 * ddt_scale));
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n13, eq50_e2379_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_value: f64 = eq50_e2381;
        let eq50_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq50_node_derivatives: [f64; 13] = [eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14];
        let eq50_branch_derivative_indices: [usize; 0] = [];
        let eq50_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq50_value),
            &eq50_node_derivative_indices,
            &eq50_node_derivatives,
            &eq50_branch_derivative_indices,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard654 == 0.0)) {
        let eq51_e2388: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 17, var_qgd_parasitic);
        (eq51_e2388, (var_qgd_parasitic_dn0 * ddt_scale), (var_qgd_parasitic_dn2 * ddt_scale), (var_qgd_parasitic_dn3 * ddt_scale), (var_qgd_parasitic_dn4 * ddt_scale), (var_qgd_parasitic_dn5 * ddt_scale), (var_qgd_parasitic_dn6 * ddt_scale), (var_qgd_parasitic_dn7 * ddt_scale), (var_qgd_parasitic_dn8 * ddt_scale), (var_qgd_parasitic_dn9 * ddt_scale), (var_qgd_parasitic_dn10 * ddt_scale), (var_qgd_parasitic_dn11 * ddt_scale), (var_qgd_parasitic_dn13 * ddt_scale), (var_qgd_parasitic_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e2390;
        let eq51_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq51_node_derivatives: [f64; 13] = [eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(5),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n2,) = {
    if (var_guard651 == 0.0) {
        let eq52_e2394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, var_qds_fr);
        (eq52_e2394, (var_qds_fr_dn0 * ddt_scale), (var_qds_fr_dn2 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e2396;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(2),
            multiplicity * (eq52_value),
            0,
            multiplicity * (eq52_e2396_d_n0),
            2,
            multiplicity * (eq52_e2396_d_n2),
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq53_e2402: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, var_qgs_fr);
        (eq53_e2402, (var_qgs_fr_dn0 * ddt_scale), (var_qgs_fr_dn2 * ddt_scale), (var_qgs_fr_dn3 * ddt_scale), (var_qgs_fr_dn4 * ddt_scale), (var_qgs_fr_dn5 * ddt_scale), (var_qgs_fr_dn6 * ddt_scale), (var_qgs_fr_dn7 * ddt_scale), (var_qgs_fr_dn8 * ddt_scale), (var_qgs_fr_dn9 * ddt_scale), (var_qgs_fr_dn10 * ddt_scale), (var_qgs_fr_dn11 * ddt_scale), (var_qgs_fr_dn13 * ddt_scale), (var_qgs_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e2404;
        let eq53_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq53_node_derivatives: [f64; 13] = [eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(2),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq54_e2410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, var_qgd_fr);
        (eq54_e2410, (var_qgd_fr_dn0 * ddt_scale), (var_qgd_fr_dn2 * ddt_scale), (var_qgd_fr_dn3 * ddt_scale), (var_qgd_fr_dn4 * ddt_scale), (var_qgd_fr_dn5 * ddt_scale), (var_qgd_fr_dn6 * ddt_scale), (var_qgd_fr_dn7 * ddt_scale), (var_qgd_fr_dn8 * ddt_scale), (var_qgd_fr_dn9 * ddt_scale), (var_qgd_fr_dn10 * ddt_scale), (var_qgd_fr_dn11 * ddt_scale), (var_qgd_fr_dn13 * ddt_scale), (var_qgd_fr_dn14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e2412;
        let eq54_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq54_node_derivatives: [f64; 13] = [eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14];
        let eq54_branch_derivative_indices: [usize; 0] = [];
        let eq54_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            Some(0),
            multiplicity * (eq54_value),
            &eq54_node_derivative_indices,
            &eq54_node_derivatives,
            &eq54_branch_derivative_indices,
            &eq54_branch_derivatives,
            multiplicity,
        );
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14,) = {
    if (var_guard656 != 0.0) {
        let eq55_e2416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 21, var_qg_acc);
        let eq55_e2417: f64 = (var_devsign * eq55_e2416);
        let eq55_e2417_d_n0: f64 = (var_devsign * (var_qg_acc_dn0 * ddt_scale));
        let eq55_e2417_d_n2: f64 = (var_devsign * (var_qg_acc_dn2 * ddt_scale));
        let eq55_e2417_d_n3: f64 = (var_devsign * (var_qg_acc_dn3 * ddt_scale));
        let eq55_e2417_d_n4: f64 = (var_devsign * (var_qg_acc_dn4 * ddt_scale));
        let eq55_e2417_d_n5: f64 = (var_devsign * (var_qg_acc_dn5 * ddt_scale));
        let eq55_e2417_d_n6: f64 = (var_devsign * (var_qg_acc_dn6 * ddt_scale));
        let eq55_e2417_d_n7: f64 = (var_devsign * (var_qg_acc_dn7 * ddt_scale));
        let eq55_e2417_d_n8: f64 = (var_devsign * (var_qg_acc_dn8 * ddt_scale));
        let eq55_e2417_d_n9: f64 = (var_devsign * (var_qg_acc_dn9 * ddt_scale));
        let eq55_e2417_d_n10: f64 = (var_devsign * (var_qg_acc_dn10 * ddt_scale));
        let eq55_e2417_d_n11: f64 = (var_devsign * (var_qg_acc_dn11 * ddt_scale));
        let eq55_e2417_d_n13: f64 = (var_devsign * (var_qg_acc_dn13 * ddt_scale));
        let eq55_e2417_d_n14: f64 = (var_devsign * (var_qg_acc_dn14 * ddt_scale));
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n13, eq55_e2417_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e2419;
        let eq55_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq55_node_derivatives: [f64; 13] = [eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_ctnoi: f64,
        var_ctnoi_dn0: f64,
        var_ctnoi_dn10: f64,
        var_ctnoi_dn11: f64,
        var_ctnoi_dn13: f64,
        var_ctnoi_dn14: f64,
        var_ctnoi_dn2: f64,
        var_ctnoi_dn3: f64,
        var_ctnoi_dn4: f64,
        var_ctnoi_dn5: f64,
        var_ctnoi_dn6: f64,
        var_ctnoi_dn7: f64,
        var_ctnoi_dn8: f64,
        var_ctnoi_dn9: f64,
        var_devsign: f64,
        var_gcrg: f64,
        var_gcrg_dn0: f64,
        var_gcrg_dn10: f64,
        var_gcrg_dn11: f64,
        var_gcrg_dn13: f64,
        var_gcrg_dn14: f64,
        var_gcrg_dn2: f64,
        var_gcrg_dn3: f64,
        var_gcrg_dn4: f64,
        var_gcrg_dn5: f64,
        var_gcrg_dn6: f64,
        var_gcrg_dn7: f64,
        var_gcrg_dn8: f64,
        var_gcrg_dn9: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn13: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn13: f64,
        var_gspr_dn14: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gtau: f64,
        var_gtau_dn0: f64,
        var_gtau_dn10: f64,
        var_gtau_dn11: f64,
        var_gtau_dn13: f64,
        var_gtau_dn14: f64,
        var_gtau_dn2: f64,
        var_gtau_dn3: f64,
        var_gtau_dn4: f64,
        var_gtau_dn5: f64,
        var_gtau_dn6: f64,
        var_gtau_dn7: f64,
        var_gtau_dn8: f64,
        var_gtau_dn9: f64,
        var_guard656: f64,
        var_guard664: f64,
        var_guard665: f64,
        var_guard666: f64,
        var_guard667: f64,
        var_guard668: f64,
        var_guard669: f64,
        var_guard677: f64,
        var_gvs_d: f64,
        var_gvs_d_dn0: f64,
        var_gvs_d_dn10: f64,
        var_gvs_d_dn11: f64,
        var_gvs_d_dn13: f64,
        var_gvs_d_dn14: f64,
        var_gvs_d_dn2: f64,
        var_gvs_d_dn3: f64,
        var_gvs_d_dn4: f64,
        var_gvs_d_dn5: f64,
        var_gvs_d_dn6: f64,
        var_gvs_d_dn7: f64,
        var_gvs_d_dn8: f64,
        var_gvs_d_dn9: f64,
        var_gvs_s: f64,
        var_gvs_s_dn0: f64,
        var_gvs_s_dn10: f64,
        var_gvs_s_dn11: f64,
        var_gvs_s_dn13: f64,
        var_gvs_s_dn14: f64,
        var_gvs_s_dn2: f64,
        var_gvs_s_dn3: f64,
        var_gvs_s_dn4: f64,
        var_gvs_s_dn5: f64,
        var_gvs_s_dn6: f64,
        var_gvs_s_dn7: f64,
        var_gvs_s_dn8: f64,
        var_gvs_s_dn9: f64,
        var_qb_acc: f64,
        var_qb_acc_dn0: f64,
        var_qb_acc_dn10: f64,
        var_qb_acc_dn11: f64,
        var_qb_acc_dn13: f64,
        var_qb_acc_dn14: f64,
        var_qb_acc_dn2: f64,
        var_qb_acc_dn3: f64,
        var_qb_acc_dn4: f64,
        var_qb_acc_dn5: f64,
        var_qb_acc_dn6: f64,
        var_qb_acc_dn7: f64,
        var_qb_acc_dn8: f64,
        var_qb_acc_dn9: f64,
        var_qb_v: f64,
        var_qb_v_dn0: f64,
        var_qb_v_dn10: f64,
        var_qb_v_dn11: f64,
        var_qb_v_dn13: f64,
        var_qb_v_dn14: f64,
        var_qb_v_dn2: f64,
        var_qb_v_dn3: f64,
        var_qb_v_dn4: f64,
        var_qb_v_dn5: f64,
        var_qb_v_dn6: f64,
        var_qb_v_dn7: f64,
        var_qb_v_dn8: f64,
        var_qb_v_dn9: f64,
        var_qg_v: f64,
        var_qg_v_dn0: f64,
        var_qg_v_dn10: f64,
        var_qg_v_dn11: f64,
        var_qg_v_dn13: f64,
        var_qg_v_dn14: f64,
        var_qg_v_dn2: f64,
        var_qg_v_dn3: f64,
        var_qg_v_dn4: f64,
        var_qg_v_dn5: f64,
        var_qg_v_dn6: f64,
        var_qg_v_dn7: f64,
        var_qg_v_dn8: f64,
        var_qg_v_dn9: f64,
        var_sigrat: f64,
        var_sigrat_dn0: f64,
        var_sigrat_dn10: f64,
        var_sigrat_dn11: f64,
        var_sigrat_dn13: f64,
        var_sigrat_dn14: f64,
        var_sigrat_dn2: f64,
        var_sigrat_dn3: f64,
        var_sigrat_dn4: f64,
        var_sigrat_dn5: f64,
        var_sigrat_dn6: f64,
        var_sigrat_dn7: f64,
        var_sigrat_dn8: f64,
        var_sigrat_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14,) = {
    if (var_guard656 != 0.0) {
        let eq56_e2423: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 22, var_qb_acc);
        let eq56_e2424: f64 = (var_devsign * eq56_e2423);
        let eq56_e2424_d_n0: f64 = (var_devsign * (var_qb_acc_dn0 * ddt_scale));
        let eq56_e2424_d_n2: f64 = (var_devsign * (var_qb_acc_dn2 * ddt_scale));
        let eq56_e2424_d_n3: f64 = (var_devsign * (var_qb_acc_dn3 * ddt_scale));
        let eq56_e2424_d_n4: f64 = (var_devsign * (var_qb_acc_dn4 * ddt_scale));
        let eq56_e2424_d_n5: f64 = (var_devsign * (var_qb_acc_dn5 * ddt_scale));
        let eq56_e2424_d_n6: f64 = (var_devsign * (var_qb_acc_dn6 * ddt_scale));
        let eq56_e2424_d_n7: f64 = (var_devsign * (var_qb_acc_dn7 * ddt_scale));
        let eq56_e2424_d_n8: f64 = (var_devsign * (var_qb_acc_dn8 * ddt_scale));
        let eq56_e2424_d_n9: f64 = (var_devsign * (var_qb_acc_dn9 * ddt_scale));
        let eq56_e2424_d_n10: f64 = (var_devsign * (var_qb_acc_dn10 * ddt_scale));
        let eq56_e2424_d_n11: f64 = (var_devsign * (var_qb_acc_dn11 * ddt_scale));
        let eq56_e2424_d_n13: f64 = (var_devsign * (var_qb_acc_dn13 * ddt_scale));
        let eq56_e2424_d_n14: f64 = (var_devsign * (var_qb_acc_dn14 * ddt_scale));
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n13, eq56_e2424_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e2426;
        let eq56_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq56_node_derivatives: [f64; 13] = [eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let (eq57_e2432, eq57_e2432_d_n0, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n13, eq57_e2432_d_n14,) = {
    if (var_guard664 != 0.0) {
        let eq57_e2430: f64 = ((nv0 - nv9) * var_gdpr);
        let eq57_e2430_d_n0: f64 = (var_gdpr + ((nv0 - nv9) * var_gdpr_dn0));
        let eq57_e2430_d_n2: f64 = ((nv0 - nv9) * var_gdpr_dn2);
        let eq57_e2430_d_n3: f64 = ((nv0 - nv9) * var_gdpr_dn3);
        let eq57_e2430_d_n4: f64 = ((nv0 - nv9) * var_gdpr_dn4);
        let eq57_e2430_d_n5: f64 = ((nv0 - nv9) * var_gdpr_dn5);
        let eq57_e2430_d_n6: f64 = ((nv0 - nv9) * var_gdpr_dn6);
        let eq57_e2430_d_n7: f64 = ((nv0 - nv9) * var_gdpr_dn7);
        let eq57_e2430_d_n8: f64 = ((nv0 - nv9) * var_gdpr_dn8);
        let eq57_e2430_d_n9: f64 = ((-var_gdpr) + ((nv0 - nv9) * var_gdpr_dn9));
        let eq57_e2430_d_n10: f64 = ((nv0 - nv9) * var_gdpr_dn10);
        let eq57_e2430_d_n11: f64 = ((nv0 - nv9) * var_gdpr_dn11);
        let eq57_e2430_d_n13: f64 = ((nv0 - nv9) * var_gdpr_dn13);
        let eq57_e2430_d_n14: f64 = ((nv0 - nv9) * var_gdpr_dn14);
        (eq57_e2430, eq57_e2430_d_n0, eq57_e2430_d_n2, eq57_e2430_d_n3, eq57_e2430_d_n4, eq57_e2430_d_n5, eq57_e2430_d_n6, eq57_e2430_d_n7, eq57_e2430_d_n8, eq57_e2430_d_n9, eq57_e2430_d_n10, eq57_e2430_d_n11, eq57_e2430_d_n13, eq57_e2430_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e2432;
        let eq57_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq57_node_derivatives: [f64; 13] = [eq57_e2432_d_n0, eq57_e2432_d_n2, eq57_e2432_d_n3, eq57_e2432_d_n4, eq57_e2432_d_n5, eq57_e2432_d_n6, eq57_e2432_d_n7, eq57_e2432_d_n8, eq57_e2432_d_n9, eq57_e2432_d_n10, eq57_e2432_d_n11, eq57_e2432_d_n13, eq57_e2432_d_n14];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq58_e2440, eq58_e2440_d_n0, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n13, eq58_e2440_d_n14,) = {
    if ((var_guard664 != 0.0) && (var_guard665 != 0.0)) {
        let eq58_e2438: f64 = ((nv9 - nv7) * var_gvs_d);
        let eq58_e2438_d_n0: f64 = ((nv9 - nv7) * var_gvs_d_dn0);
        let eq58_e2438_d_n2: f64 = ((nv9 - nv7) * var_gvs_d_dn2);
        let eq58_e2438_d_n3: f64 = ((nv9 - nv7) * var_gvs_d_dn3);
        let eq58_e2438_d_n4: f64 = ((nv9 - nv7) * var_gvs_d_dn4);
        let eq58_e2438_d_n5: f64 = ((nv9 - nv7) * var_gvs_d_dn5);
        let eq58_e2438_d_n6: f64 = ((nv9 - nv7) * var_gvs_d_dn6);
        let eq58_e2438_d_n7: f64 = ((-var_gvs_d) + ((nv9 - nv7) * var_gvs_d_dn7));
        let eq58_e2438_d_n8: f64 = ((nv9 - nv7) * var_gvs_d_dn8);
        let eq58_e2438_d_n9: f64 = (var_gvs_d + ((nv9 - nv7) * var_gvs_d_dn9));
        let eq58_e2438_d_n10: f64 = ((nv9 - nv7) * var_gvs_d_dn10);
        let eq58_e2438_d_n11: f64 = ((nv9 - nv7) * var_gvs_d_dn11);
        let eq58_e2438_d_n13: f64 = ((nv9 - nv7) * var_gvs_d_dn13);
        let eq58_e2438_d_n14: f64 = ((nv9 - nv7) * var_gvs_d_dn14);
        (eq58_e2438, eq58_e2438_d_n0, eq58_e2438_d_n2, eq58_e2438_d_n3, eq58_e2438_d_n4, eq58_e2438_d_n5, eq58_e2438_d_n6, eq58_e2438_d_n7, eq58_e2438_d_n8, eq58_e2438_d_n9, eq58_e2438_d_n10, eq58_e2438_d_n11, eq58_e2438_d_n13, eq58_e2438_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq58_value: f64 = eq58_e2440;
        let eq58_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq58_node_derivatives: [f64; 13] = [eq58_e2440_d_n0, eq58_e2440_d_n2, eq58_e2440_d_n3, eq58_e2440_d_n4, eq58_e2440_d_n5, eq58_e2440_d_n6, eq58_e2440_d_n7, eq58_e2440_d_n8, eq58_e2440_d_n9, eq58_e2440_d_n10, eq58_e2440_d_n11, eq58_e2440_d_n13, eq58_e2440_d_n14];
        let eq58_branch_derivative_indices: [usize; 0] = [];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq58_value),
            &eq58_node_derivative_indices,
            &eq58_node_derivatives,
            &eq58_branch_derivative_indices,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let (eq59_e2447,) = {
    if ((var_guard664 != 0.0) && (var_guard665 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq59_value: f64 = eq59_e2447;
        stamper.stamp_potential_const_local(
            1,
            eq59_value,
        );
        let (eq60_e2452,) = {
    if (var_guard664 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq60_value: f64 = eq60_e2452;
        stamper.stamp_potential_const_local(
            2,
            eq60_value,
        );
        let (eq61_e2457,) = {
    if (var_guard664 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq61_value: f64 = eq61_e2457;
        stamper.stamp_potential_const_local(
            3,
            eq61_value,
        );
        let (eq62_e2463, eq62_e2463_d_n0, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n13, eq62_e2463_d_n14,) = {
    if (var_guard666 != 0.0) {
        let eq62_e2461: f64 = ((nv2 - nv8) * var_gspr);
        let eq62_e2461_d_n0: f64 = ((nv2 - nv8) * var_gspr_dn0);
        let eq62_e2461_d_n2: f64 = (var_gspr + ((nv2 - nv8) * var_gspr_dn2));
        let eq62_e2461_d_n3: f64 = ((nv2 - nv8) * var_gspr_dn3);
        let eq62_e2461_d_n4: f64 = ((nv2 - nv8) * var_gspr_dn4);
        let eq62_e2461_d_n5: f64 = ((nv2 - nv8) * var_gspr_dn5);
        let eq62_e2461_d_n6: f64 = ((nv2 - nv8) * var_gspr_dn6);
        let eq62_e2461_d_n7: f64 = ((nv2 - nv8) * var_gspr_dn7);
        let eq62_e2461_d_n8: f64 = ((-var_gspr) + ((nv2 - nv8) * var_gspr_dn8));
        let eq62_e2461_d_n9: f64 = ((nv2 - nv8) * var_gspr_dn9);
        let eq62_e2461_d_n10: f64 = ((nv2 - nv8) * var_gspr_dn10);
        let eq62_e2461_d_n11: f64 = ((nv2 - nv8) * var_gspr_dn11);
        let eq62_e2461_d_n13: f64 = ((nv2 - nv8) * var_gspr_dn13);
        let eq62_e2461_d_n14: f64 = ((nv2 - nv8) * var_gspr_dn14);
        (eq62_e2461, eq62_e2461_d_n0, eq62_e2461_d_n2, eq62_e2461_d_n3, eq62_e2461_d_n4, eq62_e2461_d_n5, eq62_e2461_d_n6, eq62_e2461_d_n7, eq62_e2461_d_n8, eq62_e2461_d_n9, eq62_e2461_d_n10, eq62_e2461_d_n11, eq62_e2461_d_n13, eq62_e2461_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e2463;
        let eq62_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq62_node_derivatives: [f64; 13] = [eq62_e2463_d_n0, eq62_e2463_d_n2, eq62_e2463_d_n3, eq62_e2463_d_n4, eq62_e2463_d_n5, eq62_e2463_d_n6, eq62_e2463_d_n7, eq62_e2463_d_n8, eq62_e2463_d_n9, eq62_e2463_d_n10, eq62_e2463_d_n11, eq62_e2463_d_n13, eq62_e2463_d_n14];
        let eq62_branch_derivative_indices: [usize; 0] = [];
        let eq62_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq62_value),
            &eq62_node_derivative_indices,
            &eq62_node_derivatives,
            &eq62_branch_derivative_indices,
            &eq62_branch_derivatives,
            multiplicity,
        );
        let (eq63_e2471, eq63_e2471_d_n0, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n13, eq63_e2471_d_n14,) = {
    if ((var_guard666 != 0.0) && (var_guard667 != 0.0)) {
        let eq63_e2469: f64 = ((nv8 - nv6) * var_gvs_s);
        let eq63_e2469_d_n0: f64 = ((nv8 - nv6) * var_gvs_s_dn0);
        let eq63_e2469_d_n2: f64 = ((nv8 - nv6) * var_gvs_s_dn2);
        let eq63_e2469_d_n3: f64 = ((nv8 - nv6) * var_gvs_s_dn3);
        let eq63_e2469_d_n4: f64 = ((nv8 - nv6) * var_gvs_s_dn4);
        let eq63_e2469_d_n5: f64 = ((nv8 - nv6) * var_gvs_s_dn5);
        let eq63_e2469_d_n6: f64 = ((-var_gvs_s) + ((nv8 - nv6) * var_gvs_s_dn6));
        let eq63_e2469_d_n7: f64 = ((nv8 - nv6) * var_gvs_s_dn7);
        let eq63_e2469_d_n8: f64 = (var_gvs_s + ((nv8 - nv6) * var_gvs_s_dn8));
        let eq63_e2469_d_n9: f64 = ((nv8 - nv6) * var_gvs_s_dn9);
        let eq63_e2469_d_n10: f64 = ((nv8 - nv6) * var_gvs_s_dn10);
        let eq63_e2469_d_n11: f64 = ((nv8 - nv6) * var_gvs_s_dn11);
        let eq63_e2469_d_n13: f64 = ((nv8 - nv6) * var_gvs_s_dn13);
        let eq63_e2469_d_n14: f64 = ((nv8 - nv6) * var_gvs_s_dn14);
        (eq63_e2469, eq63_e2469_d_n0, eq63_e2469_d_n2, eq63_e2469_d_n3, eq63_e2469_d_n4, eq63_e2469_d_n5, eq63_e2469_d_n6, eq63_e2469_d_n7, eq63_e2469_d_n8, eq63_e2469_d_n9, eq63_e2469_d_n10, eq63_e2469_d_n11, eq63_e2469_d_n13, eq63_e2469_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e2471;
        let eq63_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq63_node_derivatives: [f64; 13] = [eq63_e2471_d_n0, eq63_e2471_d_n2, eq63_e2471_d_n3, eq63_e2471_d_n4, eq63_e2471_d_n5, eq63_e2471_d_n6, eq63_e2471_d_n7, eq63_e2471_d_n8, eq63_e2471_d_n9, eq63_e2471_d_n10, eq63_e2471_d_n11, eq63_e2471_d_n13, eq63_e2471_d_n14];
        let eq63_branch_derivative_indices: [usize; 0] = [];
        let eq63_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq63_value),
            &eq63_node_derivative_indices,
            &eq63_node_derivatives,
            &eq63_branch_derivative_indices,
            &eq63_branch_derivatives,
            multiplicity,
        );
        let (eq64_e2478,) = {
    if ((var_guard666 != 0.0) && (var_guard667 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq64_value: f64 = eq64_e2478;
        stamper.stamp_potential_const_local(
            4,
            eq64_value,
        );
        let (eq65_e2483,) = {
    if (var_guard666 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq65_value: f64 = eq65_e2483;
        stamper.stamp_potential_const_local(
            5,
            eq65_value,
        );
        let (eq66_e2488,) = {
    if (var_guard666 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq66_value: f64 = eq66_e2488;
        stamper.stamp_potential_const_local(
            6,
            eq66_value,
        );
        let (eq67_e2494, eq67_e2494_d_n0, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14,) = {
    if (var_guard668 != 0.0) {
        let eq67_e2492: f64 = ((nv12 - nv11) * var_gcrg);
        let eq67_e2492_d_n0: f64 = ((nv12 - nv11) * var_gcrg_dn0);
        let eq67_e2492_d_n2: f64 = ((nv12 - nv11) * var_gcrg_dn2);
        let eq67_e2492_d_n3: f64 = ((nv12 - nv11) * var_gcrg_dn3);
        let eq67_e2492_d_n4: f64 = ((nv12 - nv11) * var_gcrg_dn4);
        let eq67_e2492_d_n5: f64 = ((nv12 - nv11) * var_gcrg_dn5);
        let eq67_e2492_d_n6: f64 = ((nv12 - nv11) * var_gcrg_dn6);
        let eq67_e2492_d_n7: f64 = ((nv12 - nv11) * var_gcrg_dn7);
        let eq67_e2492_d_n8: f64 = ((nv12 - nv11) * var_gcrg_dn8);
        let eq67_e2492_d_n9: f64 = ((nv12 - nv11) * var_gcrg_dn9);
        let eq67_e2492_d_n10: f64 = ((nv12 - nv11) * var_gcrg_dn10);
        let eq67_e2492_d_n11: f64 = ((-var_gcrg) + ((nv12 - nv11) * var_gcrg_dn11));
        let eq67_e2492_d_n13: f64 = ((nv12 - nv11) * var_gcrg_dn13);
        let eq67_e2492_d_n14: f64 = ((nv12 - nv11) * var_gcrg_dn14);
        (eq67_e2492, eq67_e2492_d_n0, eq67_e2492_d_n2, eq67_e2492_d_n3, eq67_e2492_d_n4, eq67_e2492_d_n5, eq67_e2492_d_n6, eq67_e2492_d_n7, eq67_e2492_d_n8, eq67_e2492_d_n9, eq67_e2492_d_n10, eq67_e2492_d_n11, var_gcrg, eq67_e2492_d_n13, eq67_e2492_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e2494;
        let eq67_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq67_node_derivatives: [f64; 14] = [eq67_e2494_d_n0, eq67_e2494_d_n2, eq67_e2494_d_n3, eq67_e2494_d_n4, eq67_e2494_d_n5, eq67_e2494_d_n6, eq67_e2494_d_n7, eq67_e2494_d_n8, eq67_e2494_d_n9, eq67_e2494_d_n10, eq67_e2494_d_n11, eq67_e2494_d_n12, eq67_e2494_d_n13, eq67_e2494_d_n14];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(11),
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14,) = {
    if (var_guard669 != 0.0) {
        let eq69_e2503: f64 = (var_qg_v - var_qb_v);
        let eq69_e2503_d_n0: f64 = (var_qg_v_dn0 - var_qb_v_dn0);
        let eq69_e2503_d_n2: f64 = (var_qg_v_dn2 - var_qb_v_dn2);
        let eq69_e2503_d_n3: f64 = (var_qg_v_dn3 - var_qb_v_dn3);
        let eq69_e2503_d_n4: f64 = (var_qg_v_dn4 - var_qb_v_dn4);
        let eq69_e2503_d_n5: f64 = (var_qg_v_dn5 - var_qb_v_dn5);
        let eq69_e2503_d_n6: f64 = (var_qg_v_dn6 - var_qb_v_dn6);
        let eq69_e2503_d_n7: f64 = (var_qg_v_dn7 - var_qb_v_dn7);
        let eq69_e2503_d_n8: f64 = (var_qg_v_dn8 - var_qb_v_dn8);
        let eq69_e2503_d_n9: f64 = (var_qg_v_dn9 - var_qb_v_dn9);
        let eq69_e2503_d_n10: f64 = (var_qg_v_dn10 - var_qb_v_dn10);
        let eq69_e2503_d_n11: f64 = (var_qg_v_dn11 - var_qb_v_dn11);
        let eq69_e2503_d_n13: f64 = (var_qg_v_dn13 - var_qb_v_dn13);
        let eq69_e2503_d_n14: f64 = (var_qg_v_dn14 - var_qb_v_dn14);
        let eq69_e2504: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 23, eq69_e2503);
        (eq69_e2504, (eq69_e2503_d_n0 * ddt_scale), (eq69_e2503_d_n2 * ddt_scale), (eq69_e2503_d_n3 * ddt_scale), (eq69_e2503_d_n4 * ddt_scale), (eq69_e2503_d_n5 * ddt_scale), (eq69_e2503_d_n6 * ddt_scale), (eq69_e2503_d_n7 * ddt_scale), (eq69_e2503_d_n8 * ddt_scale), (eq69_e2503_d_n9 * ddt_scale), (eq69_e2503_d_n10 * ddt_scale), (eq69_e2503_d_n11 * ddt_scale), (eq69_e2503_d_n13 * ddt_scale), (eq69_e2503_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e2506;
        let eq69_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq69_node_derivatives: [f64; 13] = [eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14];
        let eq69_branch_derivative_indices: [usize; 0] = [];
        let eq69_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq69_value),
            &eq69_node_derivative_indices,
            &eq69_node_derivatives,
            &eq69_branch_derivative_indices,
            &eq69_branch_derivatives,
            multiplicity,
        );
        let (eq70_e2512, eq70_e2512_d_n0, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15,) = {
    if (var_guard669 != 0.0) {
        let eq70_e2510: f64 = ((nv15 - 0.0) * var_gtau);
        let eq70_e2510_d_n0: f64 = ((nv15 - 0.0) * var_gtau_dn0);
        let eq70_e2510_d_n2: f64 = ((nv15 - 0.0) * var_gtau_dn2);
        let eq70_e2510_d_n3: f64 = ((nv15 - 0.0) * var_gtau_dn3);
        let eq70_e2510_d_n4: f64 = ((nv15 - 0.0) * var_gtau_dn4);
        let eq70_e2510_d_n5: f64 = ((nv15 - 0.0) * var_gtau_dn5);
        let eq70_e2510_d_n6: f64 = ((nv15 - 0.0) * var_gtau_dn6);
        let eq70_e2510_d_n7: f64 = ((nv15 - 0.0) * var_gtau_dn7);
        let eq70_e2510_d_n8: f64 = ((nv15 - 0.0) * var_gtau_dn8);
        let eq70_e2510_d_n9: f64 = ((nv15 - 0.0) * var_gtau_dn9);
        let eq70_e2510_d_n10: f64 = ((nv15 - 0.0) * var_gtau_dn10);
        let eq70_e2510_d_n11: f64 = ((nv15 - 0.0) * var_gtau_dn11);
        let eq70_e2510_d_n13: f64 = ((nv15 - 0.0) * var_gtau_dn13);
        let eq70_e2510_d_n14: f64 = ((nv15 - 0.0) * var_gtau_dn14);
        (eq70_e2510, eq70_e2510_d_n0, eq70_e2510_d_n2, eq70_e2510_d_n3, eq70_e2510_d_n4, eq70_e2510_d_n5, eq70_e2510_d_n6, eq70_e2510_d_n7, eq70_e2510_d_n8, eq70_e2510_d_n9, eq70_e2510_d_n10, eq70_e2510_d_n11, eq70_e2510_d_n13, eq70_e2510_d_n14, var_gtau,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e2512;
        let eq70_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15];
        let eq70_node_derivatives: [f64; 14] = [eq70_e2512_d_n0, eq70_e2512_d_n2, eq70_e2512_d_n3, eq70_e2512_d_n4, eq70_e2512_d_n5, eq70_e2512_d_n6, eq70_e2512_d_n7, eq70_e2512_d_n8, eq70_e2512_d_n9, eq70_e2512_d_n10, eq70_e2512_d_n11, eq70_e2512_d_n13, eq70_e2512_d_n14, eq70_e2512_d_n15];
        let eq70_branch_derivative_indices: [usize; 0] = [];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq70_value),
            &eq70_node_derivative_indices,
            &eq70_node_derivatives,
            &eq70_branch_derivative_indices,
            &eq70_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15,) = {
    if (var_guard669 != 0.0) {
        let eq71_e2516: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 24, (nv15 - 0.0));
        let eq71_e2517: f64 = (1e-9 * eq71_e2516);
        (eq71_e2517, (1e-9 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e2519;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq71_value),
            15,
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq95_e2707, eq95_e2707_d_n0, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq95_e2705: f64 = (var_ctnoi * (nv16 - 0.0));
        let eq95_e2705_d_n0: f64 = (var_ctnoi_dn0 * (nv16 - 0.0));
        let eq95_e2705_d_n2: f64 = (var_ctnoi_dn2 * (nv16 - 0.0));
        let eq95_e2705_d_n3: f64 = (var_ctnoi_dn3 * (nv16 - 0.0));
        let eq95_e2705_d_n4: f64 = (var_ctnoi_dn4 * (nv16 - 0.0));
        let eq95_e2705_d_n5: f64 = (var_ctnoi_dn5 * (nv16 - 0.0));
        let eq95_e2705_d_n6: f64 = (var_ctnoi_dn6 * (nv16 - 0.0));
        let eq95_e2705_d_n7: f64 = (var_ctnoi_dn7 * (nv16 - 0.0));
        let eq95_e2705_d_n8: f64 = (var_ctnoi_dn8 * (nv16 - 0.0));
        let eq95_e2705_d_n9: f64 = (var_ctnoi_dn9 * (nv16 - 0.0));
        let eq95_e2705_d_n10: f64 = (var_ctnoi_dn10 * (nv16 - 0.0));
        let eq95_e2705_d_n11: f64 = (var_ctnoi_dn11 * (nv16 - 0.0));
        let eq95_e2705_d_n13: f64 = (var_ctnoi_dn13 * (nv16 - 0.0));
        let eq95_e2705_d_n14: f64 = (var_ctnoi_dn14 * (nv16 - 0.0));
        (eq95_e2705, eq95_e2705_d_n0, eq95_e2705_d_n2, eq95_e2705_d_n3, eq95_e2705_d_n4, eq95_e2705_d_n5, eq95_e2705_d_n6, eq95_e2705_d_n7, eq95_e2705_d_n8, eq95_e2705_d_n9, eq95_e2705_d_n10, eq95_e2705_d_n11, eq95_e2705_d_n13, eq95_e2705_d_n14, var_ctnoi,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq95_value: f64 = eq95_e2707;
        let eq95_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq95_node_derivatives: [f64; 14] = [eq95_e2707_d_n0, eq95_e2707_d_n2, eq95_e2707_d_n3, eq95_e2707_d_n4, eq95_e2707_d_n5, eq95_e2707_d_n6, eq95_e2707_d_n7, eq95_e2707_d_n8, eq95_e2707_d_n9, eq95_e2707_d_n10, eq95_e2707_d_n11, eq95_e2707_d_n13, eq95_e2707_d_n14, eq95_e2707_d_n16];
        let eq95_branch_derivative_indices: [usize; 0] = [];
        let eq95_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq95_value),
            &eq95_node_derivative_indices,
            &eq95_node_derivatives,
            &eq95_branch_derivative_indices,
            &eq95_branch_derivatives,
            multiplicity,
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq96_e2712: f64 = (0.7071 * var_sigrat);
        let eq96_e2712_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq96_e2712_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq96_e2712_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq96_e2712_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq96_e2712_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq96_e2712_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq96_e2712_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq96_e2712_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq96_e2712_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq96_e2712_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq96_e2712_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq96_e2712_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq96_e2712_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2715: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 25, eq96_e2714);
        (eq96_e2715, (eq96_e2714_d_n0 * ddt_scale), (eq96_e2714_d_n2 * ddt_scale), (eq96_e2714_d_n3 * ddt_scale), (eq96_e2714_d_n4 * ddt_scale), (eq96_e2714_d_n5 * ddt_scale), (eq96_e2714_d_n6 * ddt_scale), (eq96_e2714_d_n7 * ddt_scale), (eq96_e2714_d_n8 * ddt_scale), (eq96_e2714_d_n9 * ddt_scale), (eq96_e2714_d_n10 * ddt_scale), (eq96_e2714_d_n11 * ddt_scale), (eq96_e2714_d_n13 * ddt_scale), (eq96_e2714_d_n14 * ddt_scale), (eq96_e2712 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_value: f64 = eq96_e2717;
        let eq96_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq96_node_derivatives: [f64; 14] = [eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16];
        let eq96_branch_derivative_indices: [usize; 0] = [];
        let eq96_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq96_value),
            &eq96_node_derivative_indices,
            &eq96_node_derivatives,
            &eq96_branch_derivative_indices,
            &eq96_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
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
        var_cth: f64,
        var_cth_dn0: f64,
        var_cth_dn10: f64,
        var_cth_dn11: f64,
        var_cth_dn13: f64,
        var_cth_dn14: f64,
        var_cth_dn2: f64,
        var_cth_dn3: f64,
        var_cth_dn4: f64,
        var_cth_dn5: f64,
        var_cth_dn6: f64,
        var_cth_dn7: f64,
        var_cth_dn8: f64,
        var_cth_dn9: f64,
        var_devsign: f64,
        var_gdpr: f64,
        var_gdpr_dn0: f64,
        var_gdpr_dn10: f64,
        var_gdpr_dn11: f64,
        var_gdpr_dn13: f64,
        var_gdpr_dn14: f64,
        var_gdpr_dn2: f64,
        var_gdpr_dn3: f64,
        var_gdpr_dn4: f64,
        var_gdpr_dn5: f64,
        var_gdpr_dn6: f64,
        var_gdpr_dn7: f64,
        var_gdpr_dn8: f64,
        var_gdpr_dn9: f64,
        var_gspr: f64,
        var_gspr_dn0: f64,
        var_gspr_dn10: f64,
        var_gspr_dn11: f64,
        var_gspr_dn13: f64,
        var_gspr_dn14: f64,
        var_gspr_dn2: f64,
        var_gspr_dn3: f64,
        var_gspr_dn4: f64,
        var_gspr_dn5: f64,
        var_gspr_dn6: f64,
        var_gspr_dn7: f64,
        var_gspr_dn8: f64,
        var_gspr_dn9: f64,
        var_gth: f64,
        var_gth_dn0: f64,
        var_gth_dn10: f64,
        var_gth_dn11: f64,
        var_gth_dn13: f64,
        var_gth_dn14: f64,
        var_gth_dn2: f64,
        var_gth_dn3: f64,
        var_gth_dn4: f64,
        var_gth_dn5: f64,
        var_gth_dn6: f64,
        var_gth_dn7: f64,
        var_gth_dn8: f64,
        var_gth_dn9: f64,
        var_guard677: f64,
        var_guard682: f64,
        var_guard683: f64,
        var_guard684: f64,
        var_guard685: f64,
        var_guard686: f64,
        var_gvs_d: f64,
        var_gvs_d_dn0: f64,
        var_gvs_d_dn10: f64,
        var_gvs_d_dn11: f64,
        var_gvs_d_dn13: f64,
        var_gvs_d_dn14: f64,
        var_gvs_d_dn2: f64,
        var_gvs_d_dn3: f64,
        var_gvs_d_dn4: f64,
        var_gvs_d_dn5: f64,
        var_gvs_d_dn6: f64,
        var_gvs_d_dn7: f64,
        var_gvs_d_dn8: f64,
        var_gvs_d_dn9: f64,
        var_gvs_s: f64,
        var_gvs_s_dn0: f64,
        var_gvs_s_dn10: f64,
        var_gvs_s_dn11: f64,
        var_gvs_s_dn13: f64,
        var_gvs_s_dn14: f64,
        var_gvs_s_dn2: f64,
        var_gvs_s_dn3: f64,
        var_gvs_s_dn4: f64,
        var_gvs_s_dn5: f64,
        var_gvs_s_dn6: f64,
        var_gvs_s_dn7: f64,
        var_gvs_s_dn8: f64,
        var_gvs_s_dn9: f64,
        var_ids_v: f64,
        var_ids_v_dn0: f64,
        var_ids_v_dn10: f64,
        var_ids_v_dn11: f64,
        var_ids_v_dn13: f64,
        var_ids_v_dn14: f64,
        var_ids_v_dn2: f64,
        var_ids_v_dn3: f64,
        var_ids_v_dn4: f64,
        var_ids_v_dn5: f64,
        var_ids_v_dn6: f64,
        var_ids_v_dn7: f64,
        var_ids_v_dn8: f64,
        var_ids_v_dn9: f64,
        var_sigrat: f64,
        var_sigrat_dn0: f64,
        var_sigrat_dn10: f64,
        var_sigrat_dn11: f64,
        var_sigrat_dn13: f64,
        var_sigrat_dn14: f64,
        var_sigrat_dn2: f64,
        var_sigrat_dn3: f64,
        var_sigrat_dn4: f64,
        var_sigrat_dn5: f64,
        var_sigrat_dn6: f64,
        var_sigrat_dn7: f64,
        var_sigrat_dn8: f64,
        var_sigrat_dn9: f64,
        var_sigvds: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16,) = {
    if (var_guard677 == 0.0) {
        let eq97_e2722: f64 = (0.7071 * var_sigrat);
        let eq97_e2722_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq97_e2722_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq97_e2722_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq97_e2722_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq97_e2722_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq97_e2722_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq97_e2722_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq97_e2722_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq97_e2722_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq97_e2722_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq97_e2722_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq97_e2722_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq97_e2722_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2725: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 26, eq97_e2724);
        (eq97_e2725, (eq97_e2724_d_n0 * ddt_scale), (eq97_e2724_d_n2 * ddt_scale), (eq97_e2724_d_n3 * ddt_scale), (eq97_e2724_d_n4 * ddt_scale), (eq97_e2724_d_n5 * ddt_scale), (eq97_e2724_d_n6 * ddt_scale), (eq97_e2724_d_n7 * ddt_scale), (eq97_e2724_d_n8 * ddt_scale), (eq97_e2724_d_n9 * ddt_scale), (eq97_e2724_d_n10 * ddt_scale), (eq97_e2724_d_n11 * ddt_scale), (eq97_e2724_d_n13 * ddt_scale), (eq97_e2724_d_n14 * ddt_scale), (eq97_e2722 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_value: f64 = eq97_e2727;
        let eq97_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 16];
        let eq97_node_derivatives: [f64; 14] = [eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16];
        let eq97_branch_derivative_indices: [usize; 0] = [];
        let eq97_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq97_value),
            &eq97_node_derivative_indices,
            &eq97_node_derivatives,
            &eq97_branch_derivative_indices,
            &eq97_branch_derivatives,
            multiplicity,
        );
        let (eq105_e2843, eq105_e2843_d_n0, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n13, eq105_e2843_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq105_e2836: f64 = (var_devsign * var_sigvds);
        let eq105_e2838: f64 = (eq105_e2836 * (nv5 - nv6));
        let eq105_e2840: f64 = (eq105_e2838 * var_ids_v);
        let eq105_e2840_d_n0: f64 = (eq105_e2838 * var_ids_v_dn0);
        let eq105_e2840_d_n2: f64 = (eq105_e2838 * var_ids_v_dn2);
        let eq105_e2840_d_n3: f64 = (eq105_e2838 * var_ids_v_dn3);
        let eq105_e2840_d_n4: f64 = (eq105_e2838 * var_ids_v_dn4);
        let eq105_e2840_d_n5: f64 = ((eq105_e2836 * var_ids_v) + (eq105_e2838 * var_ids_v_dn5));
        let eq105_e2840_d_n6: f64 = (((-eq105_e2836) * var_ids_v) + (eq105_e2838 * var_ids_v_dn6));
        let eq105_e2840_d_n7: f64 = (eq105_e2838 * var_ids_v_dn7);
        let eq105_e2840_d_n8: f64 = (eq105_e2838 * var_ids_v_dn8);
        let eq105_e2840_d_n9: f64 = (eq105_e2838 * var_ids_v_dn9);
        let eq105_e2840_d_n10: f64 = (eq105_e2838 * var_ids_v_dn10);
        let eq105_e2840_d_n11: f64 = (eq105_e2838 * var_ids_v_dn11);
        let eq105_e2840_d_n13: f64 = (eq105_e2838 * var_ids_v_dn13);
        let eq105_e2840_d_n14: f64 = (eq105_e2838 * var_ids_v_dn14);
        let eq105_e2841: f64 = (-eq105_e2840);
        (eq105_e2841, (-eq105_e2840_d_n0), (-eq105_e2840_d_n2), (-eq105_e2840_d_n3), (-eq105_e2840_d_n4), (-eq105_e2840_d_n5), (-eq105_e2840_d_n6), (-eq105_e2840_d_n7), (-eq105_e2840_d_n8), (-eq105_e2840_d_n9), (-eq105_e2840_d_n10), (-eq105_e2840_d_n11), (-eq105_e2840_d_n13), (-eq105_e2840_d_n14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq105_value: f64 = eq105_e2843;
        let eq105_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq105_node_derivatives: [f64; 13] = [eq105_e2843_d_n0, eq105_e2843_d_n2, eq105_e2843_d_n3, eq105_e2843_d_n4, eq105_e2843_d_n5, eq105_e2843_d_n6, eq105_e2843_d_n7, eq105_e2843_d_n8, eq105_e2843_d_n9, eq105_e2843_d_n10, eq105_e2843_d_n11, eq105_e2843_d_n13, eq105_e2843_d_n14];
        let eq105_branch_derivative_indices: [usize; 0] = [];
        let eq105_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq105_value),
            &eq105_node_derivative_indices,
            &eq105_node_derivatives,
            &eq105_branch_derivative_indices,
            &eq105_branch_derivatives,
            multiplicity,
        );
        let (eq106_e2854, eq106_e2854_d_n0, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n13, eq106_e2854_d_n14,) = {
    if ((var_guard682 != 0.0) && (var_guard683 != 0.0)) {
        let eq106_e2848: f64 = (-(nv0 - nv9));
        let eq106_e2850: f64 = (eq106_e2848 * (nv0 - nv9));
        let eq106_e2850_d_n0: f64 = (((-1.0) * (nv0 - nv9)) + eq106_e2848);
        let eq106_e2850_d_n9: f64 = ((nv0 - nv9) + (-eq106_e2848));
        let eq106_e2852: f64 = (eq106_e2850 * var_gdpr);
        let eq106_e2852_d_n0: f64 = ((eq106_e2850_d_n0 * var_gdpr) + (eq106_e2850 * var_gdpr_dn0));
        let eq106_e2852_d_n2: f64 = (eq106_e2850 * var_gdpr_dn2);
        let eq106_e2852_d_n3: f64 = (eq106_e2850 * var_gdpr_dn3);
        let eq106_e2852_d_n4: f64 = (eq106_e2850 * var_gdpr_dn4);
        let eq106_e2852_d_n5: f64 = (eq106_e2850 * var_gdpr_dn5);
        let eq106_e2852_d_n6: f64 = (eq106_e2850 * var_gdpr_dn6);
        let eq106_e2852_d_n7: f64 = (eq106_e2850 * var_gdpr_dn7);
        let eq106_e2852_d_n8: f64 = (eq106_e2850 * var_gdpr_dn8);
        let eq106_e2852_d_n9: f64 = ((eq106_e2850_d_n9 * var_gdpr) + (eq106_e2850 * var_gdpr_dn9));
        let eq106_e2852_d_n10: f64 = (eq106_e2850 * var_gdpr_dn10);
        let eq106_e2852_d_n11: f64 = (eq106_e2850 * var_gdpr_dn11);
        let eq106_e2852_d_n13: f64 = (eq106_e2850 * var_gdpr_dn13);
        let eq106_e2852_d_n14: f64 = (eq106_e2850 * var_gdpr_dn14);
        (eq106_e2852, eq106_e2852_d_n0, eq106_e2852_d_n2, eq106_e2852_d_n3, eq106_e2852_d_n4, eq106_e2852_d_n5, eq106_e2852_d_n6, eq106_e2852_d_n7, eq106_e2852_d_n8, eq106_e2852_d_n9, eq106_e2852_d_n10, eq106_e2852_d_n11, eq106_e2852_d_n13, eq106_e2852_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq106_value: f64 = eq106_e2854;
        let eq106_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq106_node_derivatives: [f64; 13] = [eq106_e2854_d_n0, eq106_e2854_d_n2, eq106_e2854_d_n3, eq106_e2854_d_n4, eq106_e2854_d_n5, eq106_e2854_d_n6, eq106_e2854_d_n7, eq106_e2854_d_n8, eq106_e2854_d_n9, eq106_e2854_d_n10, eq106_e2854_d_n11, eq106_e2854_d_n13, eq106_e2854_d_n14];
        let eq106_branch_derivative_indices: [usize; 0] = [];
        let eq106_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq106_value),
            &eq106_node_derivative_indices,
            &eq106_node_derivatives,
            &eq106_branch_derivative_indices,
            &eq106_branch_derivatives,
            multiplicity,
        );
        let (eq107_e2867, eq107_e2867_d_n0, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n13, eq107_e2867_d_n14,) = {
    if (((var_guard682 != 0.0) && (var_guard683 != 0.0)) && (var_guard684 != 0.0)) {
        let eq107_e2861: f64 = (-(nv9 - nv7));
        let eq107_e2863: f64 = (eq107_e2861 * (nv9 - nv7));
        let eq107_e2863_d_n7: f64 = ((nv9 - nv7) + (-eq107_e2861));
        let eq107_e2863_d_n9: f64 = (((-1.0) * (nv9 - nv7)) + eq107_e2861);
        let eq107_e2865: f64 = (eq107_e2863 * var_gvs_d);
        let eq107_e2865_d_n0: f64 = (eq107_e2863 * var_gvs_d_dn0);
        let eq107_e2865_d_n2: f64 = (eq107_e2863 * var_gvs_d_dn2);
        let eq107_e2865_d_n3: f64 = (eq107_e2863 * var_gvs_d_dn3);
        let eq107_e2865_d_n4: f64 = (eq107_e2863 * var_gvs_d_dn4);
        let eq107_e2865_d_n5: f64 = (eq107_e2863 * var_gvs_d_dn5);
        let eq107_e2865_d_n6: f64 = (eq107_e2863 * var_gvs_d_dn6);
        let eq107_e2865_d_n7: f64 = ((eq107_e2863_d_n7 * var_gvs_d) + (eq107_e2863 * var_gvs_d_dn7));
        let eq107_e2865_d_n8: f64 = (eq107_e2863 * var_gvs_d_dn8);
        let eq107_e2865_d_n9: f64 = ((eq107_e2863_d_n9 * var_gvs_d) + (eq107_e2863 * var_gvs_d_dn9));
        let eq107_e2865_d_n10: f64 = (eq107_e2863 * var_gvs_d_dn10);
        let eq107_e2865_d_n11: f64 = (eq107_e2863 * var_gvs_d_dn11);
        let eq107_e2865_d_n13: f64 = (eq107_e2863 * var_gvs_d_dn13);
        let eq107_e2865_d_n14: f64 = (eq107_e2863 * var_gvs_d_dn14);
        (eq107_e2865, eq107_e2865_d_n0, eq107_e2865_d_n2, eq107_e2865_d_n3, eq107_e2865_d_n4, eq107_e2865_d_n5, eq107_e2865_d_n6, eq107_e2865_d_n7, eq107_e2865_d_n8, eq107_e2865_d_n9, eq107_e2865_d_n10, eq107_e2865_d_n11, eq107_e2865_d_n13, eq107_e2865_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq107_value: f64 = eq107_e2867;
        let eq107_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq107_node_derivatives: [f64; 13] = [eq107_e2867_d_n0, eq107_e2867_d_n2, eq107_e2867_d_n3, eq107_e2867_d_n4, eq107_e2867_d_n5, eq107_e2867_d_n6, eq107_e2867_d_n7, eq107_e2867_d_n8, eq107_e2867_d_n9, eq107_e2867_d_n10, eq107_e2867_d_n11, eq107_e2867_d_n13, eq107_e2867_d_n14];
        let eq107_branch_derivative_indices: [usize; 0] = [];
        let eq107_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq107_value),
            &eq107_node_derivative_indices,
            &eq107_node_derivatives,
            &eq107_branch_derivative_indices,
            &eq107_branch_derivatives,
            multiplicity,
        );
        let (eq108_e2878, eq108_e2878_d_n0, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n13, eq108_e2878_d_n14,) = {
    if ((var_guard682 != 0.0) && (var_guard685 != 0.0)) {
        let eq108_e2872: f64 = (-(nv2 - nv8));
        let eq108_e2874: f64 = (eq108_e2872 * (nv2 - nv8));
        let eq108_e2874_d_n2: f64 = (((-1.0) * (nv2 - nv8)) + eq108_e2872);
        let eq108_e2874_d_n8: f64 = ((nv2 - nv8) + (-eq108_e2872));
        let eq108_e2876: f64 = (eq108_e2874 * var_gspr);
        let eq108_e2876_d_n0: f64 = (eq108_e2874 * var_gspr_dn0);
        let eq108_e2876_d_n2: f64 = ((eq108_e2874_d_n2 * var_gspr) + (eq108_e2874 * var_gspr_dn2));
        let eq108_e2876_d_n3: f64 = (eq108_e2874 * var_gspr_dn3);
        let eq108_e2876_d_n4: f64 = (eq108_e2874 * var_gspr_dn4);
        let eq108_e2876_d_n5: f64 = (eq108_e2874 * var_gspr_dn5);
        let eq108_e2876_d_n6: f64 = (eq108_e2874 * var_gspr_dn6);
        let eq108_e2876_d_n7: f64 = (eq108_e2874 * var_gspr_dn7);
        let eq108_e2876_d_n8: f64 = ((eq108_e2874_d_n8 * var_gspr) + (eq108_e2874 * var_gspr_dn8));
        let eq108_e2876_d_n9: f64 = (eq108_e2874 * var_gspr_dn9);
        let eq108_e2876_d_n10: f64 = (eq108_e2874 * var_gspr_dn10);
        let eq108_e2876_d_n11: f64 = (eq108_e2874 * var_gspr_dn11);
        let eq108_e2876_d_n13: f64 = (eq108_e2874 * var_gspr_dn13);
        let eq108_e2876_d_n14: f64 = (eq108_e2874 * var_gspr_dn14);
        (eq108_e2876, eq108_e2876_d_n0, eq108_e2876_d_n2, eq108_e2876_d_n3, eq108_e2876_d_n4, eq108_e2876_d_n5, eq108_e2876_d_n6, eq108_e2876_d_n7, eq108_e2876_d_n8, eq108_e2876_d_n9, eq108_e2876_d_n10, eq108_e2876_d_n11, eq108_e2876_d_n13, eq108_e2876_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq108_value: f64 = eq108_e2878;
        let eq108_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq108_node_derivatives: [f64; 13] = [eq108_e2878_d_n0, eq108_e2878_d_n2, eq108_e2878_d_n3, eq108_e2878_d_n4, eq108_e2878_d_n5, eq108_e2878_d_n6, eq108_e2878_d_n7, eq108_e2878_d_n8, eq108_e2878_d_n9, eq108_e2878_d_n10, eq108_e2878_d_n11, eq108_e2878_d_n13, eq108_e2878_d_n14];
        let eq108_branch_derivative_indices: [usize; 0] = [];
        let eq108_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq108_value),
            &eq108_node_derivative_indices,
            &eq108_node_derivatives,
            &eq108_branch_derivative_indices,
            &eq108_branch_derivatives,
            multiplicity,
        );
        let (eq109_e2891, eq109_e2891_d_n0, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n13, eq109_e2891_d_n14,) = {
    if (((var_guard682 != 0.0) && (var_guard685 != 0.0)) && (var_guard686 != 0.0)) {
        let eq109_e2885: f64 = (-(nv8 - nv6));
        let eq109_e2887: f64 = (eq109_e2885 * (nv8 - nv6));
        let eq109_e2887_d_n6: f64 = ((nv8 - nv6) + (-eq109_e2885));
        let eq109_e2887_d_n8: f64 = (((-1.0) * (nv8 - nv6)) + eq109_e2885);
        let eq109_e2889: f64 = (eq109_e2887 * var_gvs_s);
        let eq109_e2889_d_n0: f64 = (eq109_e2887 * var_gvs_s_dn0);
        let eq109_e2889_d_n2: f64 = (eq109_e2887 * var_gvs_s_dn2);
        let eq109_e2889_d_n3: f64 = (eq109_e2887 * var_gvs_s_dn3);
        let eq109_e2889_d_n4: f64 = (eq109_e2887 * var_gvs_s_dn4);
        let eq109_e2889_d_n5: f64 = (eq109_e2887 * var_gvs_s_dn5);
        let eq109_e2889_d_n6: f64 = ((eq109_e2887_d_n6 * var_gvs_s) + (eq109_e2887 * var_gvs_s_dn6));
        let eq109_e2889_d_n7: f64 = (eq109_e2887 * var_gvs_s_dn7);
        let eq109_e2889_d_n8: f64 = ((eq109_e2887_d_n8 * var_gvs_s) + (eq109_e2887 * var_gvs_s_dn8));
        let eq109_e2889_d_n9: f64 = (eq109_e2887 * var_gvs_s_dn9);
        let eq109_e2889_d_n10: f64 = (eq109_e2887 * var_gvs_s_dn10);
        let eq109_e2889_d_n11: f64 = (eq109_e2887 * var_gvs_s_dn11);
        let eq109_e2889_d_n13: f64 = (eq109_e2887 * var_gvs_s_dn13);
        let eq109_e2889_d_n14: f64 = (eq109_e2887 * var_gvs_s_dn14);
        (eq109_e2889, eq109_e2889_d_n0, eq109_e2889_d_n2, eq109_e2889_d_n3, eq109_e2889_d_n4, eq109_e2889_d_n5, eq109_e2889_d_n6, eq109_e2889_d_n7, eq109_e2889_d_n8, eq109_e2889_d_n9, eq109_e2889_d_n10, eq109_e2889_d_n11, eq109_e2889_d_n13, eq109_e2889_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq109_value: f64 = eq109_e2891;
        let eq109_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq109_node_derivatives: [f64; 13] = [eq109_e2891_d_n0, eq109_e2891_d_n2, eq109_e2891_d_n3, eq109_e2891_d_n4, eq109_e2891_d_n5, eq109_e2891_d_n6, eq109_e2891_d_n7, eq109_e2891_d_n8, eq109_e2891_d_n9, eq109_e2891_d_n10, eq109_e2891_d_n11, eq109_e2891_d_n13, eq109_e2891_d_n14];
        let eq109_branch_derivative_indices: [usize; 0] = [];
        let eq109_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq109_value),
            &eq109_node_derivative_indices,
            &eq109_node_derivatives,
            &eq109_branch_derivative_indices,
            &eq109_branch_derivatives,
            multiplicity,
        );
        let (eq110_e2897, eq110_e2897_d_n0, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n13, eq110_e2897_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq110_e2895: f64 = ((nv4 - 0.0) * var_gth);
        let eq110_e2895_d_n0: f64 = ((nv4 - 0.0) * var_gth_dn0);
        let eq110_e2895_d_n2: f64 = ((nv4 - 0.0) * var_gth_dn2);
        let eq110_e2895_d_n3: f64 = ((nv4 - 0.0) * var_gth_dn3);
        let eq110_e2895_d_n4: f64 = (var_gth + ((nv4 - 0.0) * var_gth_dn4));
        let eq110_e2895_d_n5: f64 = ((nv4 - 0.0) * var_gth_dn5);
        let eq110_e2895_d_n6: f64 = ((nv4 - 0.0) * var_gth_dn6);
        let eq110_e2895_d_n7: f64 = ((nv4 - 0.0) * var_gth_dn7);
        let eq110_e2895_d_n8: f64 = ((nv4 - 0.0) * var_gth_dn8);
        let eq110_e2895_d_n9: f64 = ((nv4 - 0.0) * var_gth_dn9);
        let eq110_e2895_d_n10: f64 = ((nv4 - 0.0) * var_gth_dn10);
        let eq110_e2895_d_n11: f64 = ((nv4 - 0.0) * var_gth_dn11);
        let eq110_e2895_d_n13: f64 = ((nv4 - 0.0) * var_gth_dn13);
        let eq110_e2895_d_n14: f64 = ((nv4 - 0.0) * var_gth_dn14);
        (eq110_e2895, eq110_e2895_d_n0, eq110_e2895_d_n2, eq110_e2895_d_n3, eq110_e2895_d_n4, eq110_e2895_d_n5, eq110_e2895_d_n6, eq110_e2895_d_n7, eq110_e2895_d_n8, eq110_e2895_d_n9, eq110_e2895_d_n10, eq110_e2895_d_n11, eq110_e2895_d_n13, eq110_e2895_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq110_value: f64 = eq110_e2897;
        let eq110_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq110_node_derivatives: [f64; 13] = [eq110_e2897_d_n0, eq110_e2897_d_n2, eq110_e2897_d_n3, eq110_e2897_d_n4, eq110_e2897_d_n5, eq110_e2897_d_n6, eq110_e2897_d_n7, eq110_e2897_d_n8, eq110_e2897_d_n9, eq110_e2897_d_n10, eq110_e2897_d_n11, eq110_e2897_d_n13, eq110_e2897_d_n14];
        let eq110_branch_derivative_indices: [usize; 0] = [];
        let eq110_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq110_value),
            &eq110_node_derivative_indices,
            &eq110_node_derivatives,
            &eq110_branch_derivative_indices,
            &eq110_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14,) = {
    if (var_guard682 != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * var_cth);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * var_cth_dn0);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * var_cth_dn2);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * var_cth_dn3);
        let eq111_e2901_d_n4: f64 = (var_cth + ((nv4 - 0.0) * var_cth_dn4));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * var_cth_dn5);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * var_cth_dn6);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * var_cth_dn7);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * var_cth_dn8);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * var_cth_dn9);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * var_cth_dn10);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * var_cth_dn11);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * var_cth_dn13);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * var_cth_dn14);
        let eq111_e2902: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 27, eq111_e2901);
        (eq111_e2902, (eq111_e2901_d_n0 * ddt_scale), (eq111_e2901_d_n2 * ddt_scale), (eq111_e2901_d_n3 * ddt_scale), (eq111_e2901_d_n4 * ddt_scale), (eq111_e2901_d_n5 * ddt_scale), (eq111_e2901_d_n6 * ddt_scale), (eq111_e2901_d_n7 * ddt_scale), (eq111_e2901_d_n8 * ddt_scale), (eq111_e2901_d_n9 * ddt_scale), (eq111_e2901_d_n10 * ddt_scale), (eq111_e2901_d_n11 * ddt_scale), (eq111_e2901_d_n13 * ddt_scale), (eq111_e2901_d_n14 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_value: f64 = eq111_e2904;
        let eq111_node_derivative_indices: [usize; 13] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq111_node_derivatives: [f64; 13] = [eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14];
        let eq111_branch_derivative_indices: [usize; 0] = [];
        let eq111_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq111_value),
            &eq111_node_derivative_indices,
            &eq111_node_derivatives,
            &eq111_branch_derivative_indices,
            &eq111_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_devsign: f64,
        var_guard642: f64,
        var_guard651: f64,
        var_guard652: f64,
        var_guard653: f64,
        var_guard654: f64,
        var_guard655: f64,
        var_qbov: f64,
        var_qbov_dn0: f64,
        var_qbov_dn10: f64,
        var_qbov_dn11: f64,
        var_qbov_dn13: f64,
        var_qbov_dn14: f64,
        var_qbov_dn2: f64,
        var_qbov_dn3: f64,
        var_qbov_dn4: f64,
        var_qbov_dn5: f64,
        var_qbov_dn6: f64,
        var_qbov_dn7: f64,
        var_qbov_dn8: f64,
        var_qbov_dn9: f64,
        var_qbov_s: f64,
        var_qbov_s_dn0: f64,
        var_qbov_s_dn10: f64,
        var_qbov_s_dn11: f64,
        var_qbov_s_dn13: f64,
        var_qbov_s_dn14: f64,
        var_qbov_s_dn2: f64,
        var_qbov_s_dn3: f64,
        var_qbov_s_dn4: f64,
        var_qbov_s_dn5: f64,
        var_qbov_s_dn6: f64,
        var_qbov_s_dn7: f64,
        var_qbov_s_dn8: f64,
        var_qbov_s_dn9: f64,
        var_qd_v: f64,
        var_qd_v_dn0: f64,
        var_qd_v_dn10: f64,
        var_qd_v_dn11: f64,
        var_qd_v_dn13: f64,
        var_qd_v_dn14: f64,
        var_qd_v_dn2: f64,
        var_qd_v_dn3: f64,
        var_qd_v_dn4: f64,
        var_qd_v_dn5: f64,
        var_qd_v_dn6: f64,
        var_qd_v_dn7: f64,
        var_qd_v_dn8: f64,
        var_qd_v_dn9: f64,
        var_qds_fr: f64,
        var_qds_fr_dn0: f64,
        var_qds_fr_dn2: f64,
        var_qed: f64,
        var_qed_dn0: f64,
        var_qed_dn10: f64,
        var_qed_dn11: f64,
        var_qed_dn13: f64,
        var_qed_dn14: f64,
        var_qed_dn2: f64,
        var_qed_dn3: f64,
        var_qed_dn4: f64,
        var_qed_dn5: f64,
        var_qed_dn6: f64,
        var_qed_dn7: f64,
        var_qed_dn8: f64,
        var_qed_dn9: f64,
        var_qeg: f64,
        var_qeg_dn0: f64,
        var_qeg_dn10: f64,
        var_qeg_dn11: f64,
        var_qeg_dn13: f64,
        var_qeg_dn14: f64,
        var_qeg_dn2: f64,
        var_qeg_dn3: f64,
        var_qeg_dn4: f64,
        var_qeg_dn5: f64,
        var_qeg_dn6: f64,
        var_qeg_dn7: f64,
        var_qeg_dn8: f64,
        var_qeg_dn9: f64,
        var_qes: f64,
        var_qes_dn0: f64,
        var_qes_dn10: f64,
        var_qes_dn11: f64,
        var_qes_dn13: f64,
        var_qes_dn14: f64,
        var_qes_dn2: f64,
        var_qes_dn3: f64,
        var_qes_dn4: f64,
        var_qes_dn5: f64,
        var_qes_dn6: f64,
        var_qes_dn7: f64,
        var_qes_dn8: f64,
        var_qes_dn9: f64,
        var_qg_v: f64,
        var_qg_v_dn0: f64,
        var_qg_v_dn10: f64,
        var_qg_v_dn11: f64,
        var_qg_v_dn13: f64,
        var_qg_v_dn14: f64,
        var_qg_v_dn2: f64,
        var_qg_v_dn3: f64,
        var_qg_v_dn4: f64,
        var_qg_v_dn5: f64,
        var_qg_v_dn6: f64,
        var_qg_v_dn7: f64,
        var_qg_v_dn8: f64,
        var_qg_v_dn9: f64,
        var_qgd_fr: f64,
        var_qgd_fr_dn0: f64,
        var_qgd_fr_dn10: f64,
        var_qgd_fr_dn11: f64,
        var_qgd_fr_dn13: f64,
        var_qgd_fr_dn14: f64,
        var_qgd_fr_dn2: f64,
        var_qgd_fr_dn3: f64,
        var_qgd_fr_dn4: f64,
        var_qgd_fr_dn5: f64,
        var_qgd_fr_dn6: f64,
        var_qgd_fr_dn7: f64,
        var_qgd_fr_dn8: f64,
        var_qgd_fr_dn9: f64,
        var_qgd_parasitic: f64,
        var_qgd_parasitic_dn0: f64,
        var_qgd_parasitic_dn10: f64,
        var_qgd_parasitic_dn11: f64,
        var_qgd_parasitic_dn13: f64,
        var_qgd_parasitic_dn14: f64,
        var_qgd_parasitic_dn2: f64,
        var_qgd_parasitic_dn3: f64,
        var_qgd_parasitic_dn4: f64,
        var_qgd_parasitic_dn5: f64,
        var_qgd_parasitic_dn6: f64,
        var_qgd_parasitic_dn7: f64,
        var_qgd_parasitic_dn8: f64,
        var_qgd_parasitic_dn9: f64,
        var_qgs_fr: f64,
        var_qgs_fr_dn0: f64,
        var_qgs_fr_dn10: f64,
        var_qgs_fr_dn11: f64,
        var_qgs_fr_dn13: f64,
        var_qgs_fr_dn14: f64,
        var_qgs_fr_dn2: f64,
        var_qgs_fr_dn3: f64,
        var_qgs_fr_dn4: f64,
        var_qgs_fr_dn5: f64,
        var_qgs_fr_dn6: f64,
        var_qgs_fr_dn7: f64,
        var_qgs_fr_dn8: f64,
        var_qgs_fr_dn9: f64,
        var_qgs_parasitic: f64,
        var_qgs_parasitic_dn0: f64,
        var_qgs_parasitic_dn10: f64,
        var_qgs_parasitic_dn11: f64,
        var_qgs_parasitic_dn13: f64,
        var_qgs_parasitic_dn14: f64,
        var_qgs_parasitic_dn2: f64,
        var_qgs_parasitic_dn3: f64,
        var_qgs_parasitic_dn4: f64,
        var_qgs_parasitic_dn5: f64,
        var_qgs_parasitic_dn6: f64,
        var_qgs_parasitic_dn7: f64,
        var_qgs_parasitic_dn8: f64,
        var_qgs_parasitic_dn9: f64,
    ) {
        let (eq4_e1979, eq4_e1979_d_n0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, eq4_e1979_d_n13, eq4_e1979_d_n14, eq4_e1979_q,) = {
    if (var_guard642 == 0.0) {
        let eq4_e1976_q: f64 = var_qd_v;
        let eq4_e1977: f64 = (var_devsign * var_qd_v);
        let eq4_e1977_d_n0: f64 = (var_devsign * var_qd_v_dn0);
        let eq4_e1977_d_n2: f64 = (var_devsign * var_qd_v_dn2);
        let eq4_e1977_d_n3: f64 = (var_devsign * var_qd_v_dn3);
        let eq4_e1977_d_n4: f64 = (var_devsign * var_qd_v_dn4);
        let eq4_e1977_d_n5: f64 = (var_devsign * var_qd_v_dn5);
        let eq4_e1977_d_n6: f64 = (var_devsign * var_qd_v_dn6);
        let eq4_e1977_d_n7: f64 = (var_devsign * var_qd_v_dn7);
        let eq4_e1977_d_n8: f64 = (var_devsign * var_qd_v_dn8);
        let eq4_e1977_d_n9: f64 = (var_devsign * var_qd_v_dn9);
        let eq4_e1977_d_n10: f64 = (var_devsign * var_qd_v_dn10);
        let eq4_e1977_d_n11: f64 = (var_devsign * var_qd_v_dn11);
        let eq4_e1977_d_n13: f64 = (var_devsign * var_qd_v_dn13);
        let eq4_e1977_d_n14: f64 = (var_devsign * var_qd_v_dn14);
        let eq4_e1977_q: f64 = (var_devsign * eq4_e1976_q);
        (eq4_e1977, eq4_e1977_d_n0, eq4_e1977_d_n2, eq4_e1977_d_n3, eq4_e1977_d_n4, eq4_e1977_d_n5, eq4_e1977_d_n6, eq4_e1977_d_n7, eq4_e1977_d_n8, eq4_e1977_d_n9, eq4_e1977_d_n10, eq4_e1977_d_n11, eq4_e1977_d_n13, eq4_e1977_d_n14, eq4_e1977_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_reactive_node_derivatives: [f64; 17] = [eq4_e1979_d_n0, 0.0, eq4_e1979_d_n2, eq4_e1979_d_n3, eq4_e1979_d_n4, eq4_e1979_d_n5, eq4_e1979_d_n6, eq4_e1979_d_n7, eq4_e1979_d_n8, eq4_e1979_d_n9, eq4_e1979_d_n10, eq4_e1979_d_n11, 0.0, eq4_e1979_d_n13, eq4_e1979_d_n14, 0.0, 0.0];
        let eq4_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq4_reactive_node_derivatives,
            branches,
            &eq4_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1987, eq5_e1987_d_n0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, eq5_e1987_d_n13, eq5_e1987_d_n14, eq5_e1987_q,) = {
    if (var_guard642 == 0.0) {
        let eq5_e1984_q: f64 = var_qg_v;
        let eq5_e1985: f64 = (var_devsign * var_qg_v);
        let eq5_e1985_d_n0: f64 = (var_devsign * var_qg_v_dn0);
        let eq5_e1985_d_n2: f64 = (var_devsign * var_qg_v_dn2);
        let eq5_e1985_d_n3: f64 = (var_devsign * var_qg_v_dn3);
        let eq5_e1985_d_n4: f64 = (var_devsign * var_qg_v_dn4);
        let eq5_e1985_d_n5: f64 = (var_devsign * var_qg_v_dn5);
        let eq5_e1985_d_n6: f64 = (var_devsign * var_qg_v_dn6);
        let eq5_e1985_d_n7: f64 = (var_devsign * var_qg_v_dn7);
        let eq5_e1985_d_n8: f64 = (var_devsign * var_qg_v_dn8);
        let eq5_e1985_d_n9: f64 = (var_devsign * var_qg_v_dn9);
        let eq5_e1985_d_n10: f64 = (var_devsign * var_qg_v_dn10);
        let eq5_e1985_d_n11: f64 = (var_devsign * var_qg_v_dn11);
        let eq5_e1985_d_n13: f64 = (var_devsign * var_qg_v_dn13);
        let eq5_e1985_d_n14: f64 = (var_devsign * var_qg_v_dn14);
        let eq5_e1985_q: f64 = (var_devsign * eq5_e1984_q);
        (eq5_e1985, eq5_e1985_d_n0, eq5_e1985_d_n2, eq5_e1985_d_n3, eq5_e1985_d_n4, eq5_e1985_d_n5, eq5_e1985_d_n6, eq5_e1985_d_n7, eq5_e1985_d_n8, eq5_e1985_d_n9, eq5_e1985_d_n10, eq5_e1985_d_n11, eq5_e1985_d_n13, eq5_e1985_d_n14, eq5_e1985_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_reactive_node_derivatives: [f64; 17] = [eq5_e1987_d_n0, 0.0, eq5_e1987_d_n2, eq5_e1987_d_n3, eq5_e1987_d_n4, eq5_e1987_d_n5, eq5_e1987_d_n6, eq5_e1987_d_n7, eq5_e1987_d_n8, eq5_e1987_d_n9, eq5_e1987_d_n10, eq5_e1987_d_n11, 0.0, eq5_e1987_d_n13, eq5_e1987_d_n14, 0.0, 0.0];
        let eq5_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq5_reactive_node_derivatives,
            branches,
            &eq5_reactive_branch_derivatives,
            multiplicity,
        );
        let eq36_e2281_q: f64 = var_qes;
        let eq36_e2282: f64 = (var_devsign * var_qes);
        let eq36_e2282_d_n0: f64 = (var_devsign * var_qes_dn0);
        let eq36_e2282_d_n2: f64 = (var_devsign * var_qes_dn2);
        let eq36_e2282_d_n3: f64 = (var_devsign * var_qes_dn3);
        let eq36_e2282_d_n4: f64 = (var_devsign * var_qes_dn4);
        let eq36_e2282_d_n5: f64 = (var_devsign * var_qes_dn5);
        let eq36_e2282_d_n6: f64 = (var_devsign * var_qes_dn6);
        let eq36_e2282_d_n7: f64 = (var_devsign * var_qes_dn7);
        let eq36_e2282_d_n8: f64 = (var_devsign * var_qes_dn8);
        let eq36_e2282_d_n9: f64 = (var_devsign * var_qes_dn9);
        let eq36_e2282_d_n10: f64 = (var_devsign * var_qes_dn10);
        let eq36_e2282_d_n11: f64 = (var_devsign * var_qes_dn11);
        let eq36_e2282_d_n13: f64 = (var_devsign * var_qes_dn13);
        let eq36_e2282_d_n14: f64 = (var_devsign * var_qes_dn14);
        let eq36_e2282_q: f64 = (var_devsign * eq36_e2281_q);
        let eq36_reactive_node_derivatives: [f64; 17] = [eq36_e2282_d_n0, 0.0, eq36_e2282_d_n2, eq36_e2282_d_n3, eq36_e2282_d_n4, eq36_e2282_d_n5, eq36_e2282_d_n6, eq36_e2282_d_n7, eq36_e2282_d_n8, eq36_e2282_d_n9, eq36_e2282_d_n10, eq36_e2282_d_n11, 0.0, eq36_e2282_d_n13, eq36_e2282_d_n14, 0.0, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let eq37_e2285_q: f64 = var_qed;
        let eq37_e2286: f64 = (var_devsign * var_qed);
        let eq37_e2286_d_n0: f64 = (var_devsign * var_qed_dn0);
        let eq37_e2286_d_n2: f64 = (var_devsign * var_qed_dn2);
        let eq37_e2286_d_n3: f64 = (var_devsign * var_qed_dn3);
        let eq37_e2286_d_n4: f64 = (var_devsign * var_qed_dn4);
        let eq37_e2286_d_n5: f64 = (var_devsign * var_qed_dn5);
        let eq37_e2286_d_n6: f64 = (var_devsign * var_qed_dn6);
        let eq37_e2286_d_n7: f64 = (var_devsign * var_qed_dn7);
        let eq37_e2286_d_n8: f64 = (var_devsign * var_qed_dn8);
        let eq37_e2286_d_n9: f64 = (var_devsign * var_qed_dn9);
        let eq37_e2286_d_n10: f64 = (var_devsign * var_qed_dn10);
        let eq37_e2286_d_n11: f64 = (var_devsign * var_qed_dn11);
        let eq37_e2286_d_n13: f64 = (var_devsign * var_qed_dn13);
        let eq37_e2286_d_n14: f64 = (var_devsign * var_qed_dn14);
        let eq37_e2286_q: f64 = (var_devsign * eq37_e2285_q);
        let eq37_reactive_node_derivatives: [f64; 17] = [eq37_e2286_d_n0, 0.0, eq37_e2286_d_n2, eq37_e2286_d_n3, eq37_e2286_d_n4, eq37_e2286_d_n5, eq37_e2286_d_n6, eq37_e2286_d_n7, eq37_e2286_d_n8, eq37_e2286_d_n9, eq37_e2286_d_n10, eq37_e2286_d_n11, 0.0, eq37_e2286_d_n13, eq37_e2286_d_n14, 0.0, 0.0];
        let eq37_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[5]),
            nodes,
            &eq37_reactive_node_derivatives,
            branches,
            &eq37_reactive_branch_derivatives,
            multiplicity,
        );
        let eq38_e2289_q: f64 = var_qeg;
        let eq38_e2290: f64 = (var_devsign * var_qeg);
        let eq38_e2290_d_n0: f64 = (var_devsign * var_qeg_dn0);
        let eq38_e2290_d_n2: f64 = (var_devsign * var_qeg_dn2);
        let eq38_e2290_d_n3: f64 = (var_devsign * var_qeg_dn3);
        let eq38_e2290_d_n4: f64 = (var_devsign * var_qeg_dn4);
        let eq38_e2290_d_n5: f64 = (var_devsign * var_qeg_dn5);
        let eq38_e2290_d_n6: f64 = (var_devsign * var_qeg_dn6);
        let eq38_e2290_d_n7: f64 = (var_devsign * var_qeg_dn7);
        let eq38_e2290_d_n8: f64 = (var_devsign * var_qeg_dn8);
        let eq38_e2290_d_n9: f64 = (var_devsign * var_qeg_dn9);
        let eq38_e2290_d_n10: f64 = (var_devsign * var_qeg_dn10);
        let eq38_e2290_d_n11: f64 = (var_devsign * var_qeg_dn11);
        let eq38_e2290_d_n13: f64 = (var_devsign * var_qeg_dn13);
        let eq38_e2290_d_n14: f64 = (var_devsign * var_qeg_dn14);
        let eq38_e2290_q: f64 = (var_devsign * eq38_e2289_q);
        let eq38_reactive_node_derivatives: [f64; 17] = [eq38_e2290_d_n0, 0.0, eq38_e2290_d_n2, eq38_e2290_d_n3, eq38_e2290_d_n4, eq38_e2290_d_n5, eq38_e2290_d_n6, eq38_e2290_d_n7, eq38_e2290_d_n8, eq38_e2290_d_n9, eq38_e2290_d_n10, eq38_e2290_d_n11, 0.0, eq38_e2290_d_n13, eq38_e2290_d_n14, 0.0, 0.0];
        let eq38_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[10]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq39_e2295, eq39_e2295_d_n0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, eq39_e2295_d_n13, eq39_e2295_d_n14, eq39_e2295_q,) = {
    if (var_guard651 != 0.0) {
        let eq39_e2293_q: f64 = var_qgs_parasitic;
        (var_qgs_parasitic, var_qgs_parasitic_dn0, var_qgs_parasitic_dn2, var_qgs_parasitic_dn3, var_qgs_parasitic_dn4, var_qgs_parasitic_dn5, var_qgs_parasitic_dn6, var_qgs_parasitic_dn7, var_qgs_parasitic_dn8, var_qgs_parasitic_dn9, var_qgs_parasitic_dn10, var_qgs_parasitic_dn11, var_qgs_parasitic_dn13, var_qgs_parasitic_dn14, eq39_e2293_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq39_reactive_node_derivatives: [f64; 17] = [eq39_e2295_d_n0, 0.0, eq39_e2295_d_n2, eq39_e2295_d_n3, eq39_e2295_d_n4, eq39_e2295_d_n5, eq39_e2295_d_n6, eq39_e2295_d_n7, eq39_e2295_d_n8, eq39_e2295_d_n9, eq39_e2295_d_n10, eq39_e2295_d_n11, 0.0, eq39_e2295_d_n13, eq39_e2295_d_n14, 0.0, 0.0];
        let eq39_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq40_e2302, eq40_e2302_d_n0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, eq40_e2302_d_n13, eq40_e2302_d_n14, eq40_e2302_q,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq40_e2300_q: f64 = var_qgd_parasitic;
        (var_qgd_parasitic, var_qgd_parasitic_dn0, var_qgd_parasitic_dn2, var_qgd_parasitic_dn3, var_qgd_parasitic_dn4, var_qgd_parasitic_dn5, var_qgd_parasitic_dn6, var_qgd_parasitic_dn7, var_qgd_parasitic_dn8, var_qgd_parasitic_dn9, var_qgd_parasitic_dn10, var_qgd_parasitic_dn11, var_qgd_parasitic_dn13, var_qgd_parasitic_dn14, eq40_e2300_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq40_reactive_node_derivatives: [f64; 17] = [eq40_e2302_d_n0, 0.0, eq40_e2302_d_n2, eq40_e2302_d_n3, eq40_e2302_d_n4, eq40_e2302_d_n5, eq40_e2302_d_n6, eq40_e2302_d_n7, eq40_e2302_d_n8, eq40_e2302_d_n9, eq40_e2302_d_n10, eq40_e2302_d_n11, 0.0, eq40_e2302_d_n13, eq40_e2302_d_n14, 0.0, 0.0];
        let eq40_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq41_e2311, eq41_e2311_d_n0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, eq41_e2311_d_n13, eq41_e2311_d_n14, eq41_e2311_q,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq41_e2308_q: f64 = var_qbov;
        let eq41_e2309: f64 = (var_devsign * var_qbov);
        let eq41_e2309_d_n0: f64 = (var_devsign * var_qbov_dn0);
        let eq41_e2309_d_n2: f64 = (var_devsign * var_qbov_dn2);
        let eq41_e2309_d_n3: f64 = (var_devsign * var_qbov_dn3);
        let eq41_e2309_d_n4: f64 = (var_devsign * var_qbov_dn4);
        let eq41_e2309_d_n5: f64 = (var_devsign * var_qbov_dn5);
        let eq41_e2309_d_n6: f64 = (var_devsign * var_qbov_dn6);
        let eq41_e2309_d_n7: f64 = (var_devsign * var_qbov_dn7);
        let eq41_e2309_d_n8: f64 = (var_devsign * var_qbov_dn8);
        let eq41_e2309_d_n9: f64 = (var_devsign * var_qbov_dn9);
        let eq41_e2309_d_n10: f64 = (var_devsign * var_qbov_dn10);
        let eq41_e2309_d_n11: f64 = (var_devsign * var_qbov_dn11);
        let eq41_e2309_d_n13: f64 = (var_devsign * var_qbov_dn13);
        let eq41_e2309_d_n14: f64 = (var_devsign * var_qbov_dn14);
        let eq41_e2309_q: f64 = (var_devsign * eq41_e2308_q);
        (eq41_e2309, eq41_e2309_d_n0, eq41_e2309_d_n2, eq41_e2309_d_n3, eq41_e2309_d_n4, eq41_e2309_d_n5, eq41_e2309_d_n6, eq41_e2309_d_n7, eq41_e2309_d_n8, eq41_e2309_d_n9, eq41_e2309_d_n10, eq41_e2309_d_n11, eq41_e2309_d_n13, eq41_e2309_d_n14, eq41_e2309_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq41_reactive_node_derivatives: [f64; 17] = [eq41_e2311_d_n0, 0.0, eq41_e2311_d_n2, eq41_e2311_d_n3, eq41_e2311_d_n4, eq41_e2311_d_n5, eq41_e2311_d_n6, eq41_e2311_d_n7, eq41_e2311_d_n8, eq41_e2311_d_n9, eq41_e2311_d_n10, eq41_e2311_d_n11, 0.0, eq41_e2311_d_n13, eq41_e2311_d_n14, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq42_e2320, eq42_e2320_d_n0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, eq42_e2320_d_n13, eq42_e2320_d_n14, eq42_e2320_q,) = {
    if ((var_guard651 != 0.0) && (var_guard652 != 0.0)) {
        let eq42_e2317_q: f64 = var_qbov_s;
        let eq42_e2318: f64 = (var_devsign * var_qbov_s);
        let eq42_e2318_d_n0: f64 = (var_devsign * var_qbov_s_dn0);
        let eq42_e2318_d_n2: f64 = (var_devsign * var_qbov_s_dn2);
        let eq42_e2318_d_n3: f64 = (var_devsign * var_qbov_s_dn3);
        let eq42_e2318_d_n4: f64 = (var_devsign * var_qbov_s_dn4);
        let eq42_e2318_d_n5: f64 = (var_devsign * var_qbov_s_dn5);
        let eq42_e2318_d_n6: f64 = (var_devsign * var_qbov_s_dn6);
        let eq42_e2318_d_n7: f64 = (var_devsign * var_qbov_s_dn7);
        let eq42_e2318_d_n8: f64 = (var_devsign * var_qbov_s_dn8);
        let eq42_e2318_d_n9: f64 = (var_devsign * var_qbov_s_dn9);
        let eq42_e2318_d_n10: f64 = (var_devsign * var_qbov_s_dn10);
        let eq42_e2318_d_n11: f64 = (var_devsign * var_qbov_s_dn11);
        let eq42_e2318_d_n13: f64 = (var_devsign * var_qbov_s_dn13);
        let eq42_e2318_d_n14: f64 = (var_devsign * var_qbov_s_dn14);
        let eq42_e2318_q: f64 = (var_devsign * eq42_e2317_q);
        (eq42_e2318, eq42_e2318_d_n0, eq42_e2318_d_n2, eq42_e2318_d_n3, eq42_e2318_d_n4, eq42_e2318_d_n5, eq42_e2318_d_n6, eq42_e2318_d_n7, eq42_e2318_d_n8, eq42_e2318_d_n9, eq42_e2318_d_n10, eq42_e2318_d_n11, eq42_e2318_d_n13, eq42_e2318_d_n14, eq42_e2318_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_reactive_node_derivatives: [f64; 17] = [eq42_e2320_d_n0, 0.0, eq42_e2320_d_n2, eq42_e2320_d_n3, eq42_e2320_d_n4, eq42_e2320_d_n5, eq42_e2320_d_n6, eq42_e2320_d_n7, eq42_e2320_d_n8, eq42_e2320_d_n9, eq42_e2320_d_n10, eq42_e2320_d_n11, 0.0, eq42_e2320_d_n13, eq42_e2320_d_n14, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq43_e2328, eq43_e2328_d_n0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, eq43_e2328_d_n13, eq43_e2328_d_n14, eq43_e2328_q,) = {
    if ((var_guard651 != 0.0) && (var_guard652 == 0.0)) {
        let eq43_e2326_q: f64 = var_qgd_parasitic;
        (var_qgd_parasitic, var_qgd_parasitic_dn0, var_qgd_parasitic_dn2, var_qgd_parasitic_dn3, var_qgd_parasitic_dn4, var_qgd_parasitic_dn5, var_qgd_parasitic_dn6, var_qgd_parasitic_dn7, var_qgd_parasitic_dn8, var_qgd_parasitic_dn9, var_qgd_parasitic_dn10, var_qgd_parasitic_dn11, var_qgd_parasitic_dn13, var_qgd_parasitic_dn14, eq43_e2326_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq43_reactive_node_derivatives: [f64; 17] = [eq43_e2328_d_n0, 0.0, eq43_e2328_d_n2, eq43_e2328_d_n3, eq43_e2328_d_n4, eq43_e2328_d_n5, eq43_e2328_d_n6, eq43_e2328_d_n7, eq43_e2328_d_n8, eq43_e2328_d_n9, eq43_e2328_d_n10, eq43_e2328_d_n11, 0.0, eq43_e2328_d_n13, eq43_e2328_d_n14, 0.0, 0.0];
        let eq43_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq44_e2333, eq44_e2333_d_n0, eq44_e2333_d_n2, eq44_e2333_q,) = {
    if (var_guard651 != 0.0) {
        let eq44_e2331_q: f64 = var_qds_fr;
        (var_qds_fr, var_qds_fr_dn0, var_qds_fr_dn2, eq44_e2331_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq44_e2333_d_n0),
            nodes[2],
            multiplicity * (eq44_e2333_d_n2),
        );
        let (eq45_e2340, eq45_e2340_d_n0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, eq45_e2340_d_n13, eq45_e2340_d_n14, eq45_e2340_q,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq45_e2338_q: f64 = var_qgs_fr;
        (var_qgs_fr, var_qgs_fr_dn0, var_qgs_fr_dn2, var_qgs_fr_dn3, var_qgs_fr_dn4, var_qgs_fr_dn5, var_qgs_fr_dn6, var_qgs_fr_dn7, var_qgs_fr_dn8, var_qgs_fr_dn9, var_qgs_fr_dn10, var_qgs_fr_dn11, var_qgs_fr_dn13, var_qgs_fr_dn14, eq45_e2338_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq45_reactive_node_derivatives: [f64; 17] = [eq45_e2340_d_n0, 0.0, eq45_e2340_d_n2, eq45_e2340_d_n3, eq45_e2340_d_n4, eq45_e2340_d_n5, eq45_e2340_d_n6, eq45_e2340_d_n7, eq45_e2340_d_n8, eq45_e2340_d_n9, eq45_e2340_d_n10, eq45_e2340_d_n11, 0.0, eq45_e2340_d_n13, eq45_e2340_d_n14, 0.0, 0.0];
        let eq45_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[2]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e2347, eq46_e2347_d_n0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, eq46_e2347_d_n13, eq46_e2347_d_n14, eq46_e2347_q,) = {
    if ((var_guard651 != 0.0) && (var_guard653 != 0.0)) {
        let eq46_e2345_q: f64 = var_qgd_fr;
        (var_qgd_fr, var_qgd_fr_dn0, var_qgd_fr_dn2, var_qgd_fr_dn3, var_qgd_fr_dn4, var_qgd_fr_dn5, var_qgd_fr_dn6, var_qgd_fr_dn7, var_qgd_fr_dn8, var_qgd_fr_dn9, var_qgd_fr_dn10, var_qgd_fr_dn11, var_qgd_fr_dn13, var_qgd_fr_dn14, eq46_e2345_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq46_reactive_node_derivatives: [f64; 17] = [eq46_e2347_d_n0, 0.0, eq46_e2347_d_n2, eq46_e2347_d_n3, eq46_e2347_d_n4, eq46_e2347_d_n5, eq46_e2347_d_n6, eq46_e2347_d_n7, eq46_e2347_d_n8, eq46_e2347_d_n9, eq46_e2347_d_n10, eq46_e2347_d_n11, 0.0, eq46_e2347_d_n13, eq46_e2347_d_n14, 0.0, 0.0];
        let eq46_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e2353, eq47_e2353_d_n0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, eq47_e2353_d_n13, eq47_e2353_d_n14, eq47_e2353_q,) = {
    if (var_guard651 == 0.0) {
        let eq47_e2351_q: f64 = var_qgs_parasitic;
        (var_qgs_parasitic, var_qgs_parasitic_dn0, var_qgs_parasitic_dn2, var_qgs_parasitic_dn3, var_qgs_parasitic_dn4, var_qgs_parasitic_dn5, var_qgs_parasitic_dn6, var_qgs_parasitic_dn7, var_qgs_parasitic_dn8, var_qgs_parasitic_dn9, var_qgs_parasitic_dn10, var_qgs_parasitic_dn11, var_qgs_parasitic_dn13, var_qgs_parasitic_dn14, eq47_e2351_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq47_reactive_node_derivatives: [f64; 17] = [eq47_e2353_d_n0, 0.0, eq47_e2353_d_n2, eq47_e2353_d_n3, eq47_e2353_d_n4, eq47_e2353_d_n5, eq47_e2353_d_n6, eq47_e2353_d_n7, eq47_e2353_d_n8, eq47_e2353_d_n9, eq47_e2353_d_n10, eq47_e2353_d_n11, 0.0, eq47_e2353_d_n13, eq47_e2353_d_n14, 0.0, 0.0];
        let eq47_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[6]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq48_e2361, eq48_e2361_d_n0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, eq48_e2361_d_n13, eq48_e2361_d_n14, eq48_e2361_q,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq48_e2359_q: f64 = var_qgd_parasitic;
        (var_qgd_parasitic, var_qgd_parasitic_dn0, var_qgd_parasitic_dn2, var_qgd_parasitic_dn3, var_qgd_parasitic_dn4, var_qgd_parasitic_dn5, var_qgd_parasitic_dn6, var_qgd_parasitic_dn7, var_qgd_parasitic_dn8, var_qgd_parasitic_dn9, var_qgd_parasitic_dn10, var_qgd_parasitic_dn11, var_qgd_parasitic_dn13, var_qgd_parasitic_dn14, eq48_e2359_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq48_reactive_node_derivatives: [f64; 17] = [eq48_e2361_d_n0, 0.0, eq48_e2361_d_n2, eq48_e2361_d_n3, eq48_e2361_d_n4, eq48_e2361_d_n5, eq48_e2361_d_n6, eq48_e2361_d_n7, eq48_e2361_d_n8, eq48_e2361_d_n9, eq48_e2361_d_n10, eq48_e2361_d_n11, 0.0, eq48_e2361_d_n13, eq48_e2361_d_n14, 0.0, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq49_e2371, eq49_e2371_d_n0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, eq49_e2371_d_n13, eq49_e2371_d_n14, eq49_e2371_q,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq49_e2368_q: f64 = var_qbov;
        let eq49_e2369: f64 = (var_devsign * var_qbov);
        let eq49_e2369_d_n0: f64 = (var_devsign * var_qbov_dn0);
        let eq49_e2369_d_n2: f64 = (var_devsign * var_qbov_dn2);
        let eq49_e2369_d_n3: f64 = (var_devsign * var_qbov_dn3);
        let eq49_e2369_d_n4: f64 = (var_devsign * var_qbov_dn4);
        let eq49_e2369_d_n5: f64 = (var_devsign * var_qbov_dn5);
        let eq49_e2369_d_n6: f64 = (var_devsign * var_qbov_dn6);
        let eq49_e2369_d_n7: f64 = (var_devsign * var_qbov_dn7);
        let eq49_e2369_d_n8: f64 = (var_devsign * var_qbov_dn8);
        let eq49_e2369_d_n9: f64 = (var_devsign * var_qbov_dn9);
        let eq49_e2369_d_n10: f64 = (var_devsign * var_qbov_dn10);
        let eq49_e2369_d_n11: f64 = (var_devsign * var_qbov_dn11);
        let eq49_e2369_d_n13: f64 = (var_devsign * var_qbov_dn13);
        let eq49_e2369_d_n14: f64 = (var_devsign * var_qbov_dn14);
        let eq49_e2369_q: f64 = (var_devsign * eq49_e2368_q);
        (eq49_e2369, eq49_e2369_d_n0, eq49_e2369_d_n2, eq49_e2369_d_n3, eq49_e2369_d_n4, eq49_e2369_d_n5, eq49_e2369_d_n6, eq49_e2369_d_n7, eq49_e2369_d_n8, eq49_e2369_d_n9, eq49_e2369_d_n10, eq49_e2369_d_n11, eq49_e2369_d_n13, eq49_e2369_d_n14, eq49_e2369_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq49_reactive_node_derivatives: [f64; 17] = [eq49_e2371_d_n0, 0.0, eq49_e2371_d_n2, eq49_e2371_d_n3, eq49_e2371_d_n4, eq49_e2371_d_n5, eq49_e2371_d_n6, eq49_e2371_d_n7, eq49_e2371_d_n8, eq49_e2371_d_n9, eq49_e2371_d_n10, eq49_e2371_d_n11, 0.0, eq49_e2371_d_n13, eq49_e2371_d_n14, 0.0, 0.0];
        let eq49_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[7]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq50_e2381, eq50_e2381_d_n0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, eq50_e2381_d_n13, eq50_e2381_d_n14, eq50_e2381_q,) = {
    if ((var_guard651 == 0.0) && (var_guard654 != 0.0)) {
        let eq50_e2378_q: f64 = var_qbov_s;
        let eq50_e2379: f64 = (var_devsign * var_qbov_s);
        let eq50_e2379_d_n0: f64 = (var_devsign * var_qbov_s_dn0);
        let eq50_e2379_d_n2: f64 = (var_devsign * var_qbov_s_dn2);
        let eq50_e2379_d_n3: f64 = (var_devsign * var_qbov_s_dn3);
        let eq50_e2379_d_n4: f64 = (var_devsign * var_qbov_s_dn4);
        let eq50_e2379_d_n5: f64 = (var_devsign * var_qbov_s_dn5);
        let eq50_e2379_d_n6: f64 = (var_devsign * var_qbov_s_dn6);
        let eq50_e2379_d_n7: f64 = (var_devsign * var_qbov_s_dn7);
        let eq50_e2379_d_n8: f64 = (var_devsign * var_qbov_s_dn8);
        let eq50_e2379_d_n9: f64 = (var_devsign * var_qbov_s_dn9);
        let eq50_e2379_d_n10: f64 = (var_devsign * var_qbov_s_dn10);
        let eq50_e2379_d_n11: f64 = (var_devsign * var_qbov_s_dn11);
        let eq50_e2379_d_n13: f64 = (var_devsign * var_qbov_s_dn13);
        let eq50_e2379_d_n14: f64 = (var_devsign * var_qbov_s_dn14);
        let eq50_e2379_q: f64 = (var_devsign * eq50_e2378_q);
        (eq50_e2379, eq50_e2379_d_n0, eq50_e2379_d_n2, eq50_e2379_d_n3, eq50_e2379_d_n4, eq50_e2379_d_n5, eq50_e2379_d_n6, eq50_e2379_d_n7, eq50_e2379_d_n8, eq50_e2379_d_n9, eq50_e2379_d_n10, eq50_e2379_d_n11, eq50_e2379_d_n13, eq50_e2379_d_n14, eq50_e2379_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq50_reactive_node_derivatives: [f64; 17] = [eq50_e2381_d_n0, 0.0, eq50_e2381_d_n2, eq50_e2381_d_n3, eq50_e2381_d_n4, eq50_e2381_d_n5, eq50_e2381_d_n6, eq50_e2381_d_n7, eq50_e2381_d_n8, eq50_e2381_d_n9, eq50_e2381_d_n10, eq50_e2381_d_n11, 0.0, eq50_e2381_d_n13, eq50_e2381_d_n14, 0.0, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq51_e2390, eq51_e2390_d_n0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, eq51_e2390_d_n13, eq51_e2390_d_n14, eq51_e2390_q,) = {
    if ((var_guard651 == 0.0) && (var_guard654 == 0.0)) {
        let eq51_e2388_q: f64 = var_qgd_parasitic;
        (var_qgd_parasitic, var_qgd_parasitic_dn0, var_qgd_parasitic_dn2, var_qgd_parasitic_dn3, var_qgd_parasitic_dn4, var_qgd_parasitic_dn5, var_qgd_parasitic_dn6, var_qgd_parasitic_dn7, var_qgd_parasitic_dn8, var_qgd_parasitic_dn9, var_qgd_parasitic_dn10, var_qgd_parasitic_dn11, var_qgd_parasitic_dn13, var_qgd_parasitic_dn14, eq51_e2388_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_reactive_node_derivatives: [f64; 17] = [eq51_e2390_d_n0, 0.0, eq51_e2390_d_n2, eq51_e2390_d_n3, eq51_e2390_d_n4, eq51_e2390_d_n5, eq51_e2390_d_n6, eq51_e2390_d_n7, eq51_e2390_d_n8, eq51_e2390_d_n9, eq51_e2390_d_n10, eq51_e2390_d_n11, 0.0, eq51_e2390_d_n13, eq51_e2390_d_n14, 0.0, 0.0];
        let eq51_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[5]),
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq52_e2396, eq52_e2396_d_n0, eq52_e2396_d_n2, eq52_e2396_q,) = {
    if (var_guard651 == 0.0) {
        let eq52_e2394_q: f64 = var_qds_fr;
        (var_qds_fr, var_qds_fr_dn0, var_qds_fr_dn2, eq52_e2394_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes[0],
            multiplicity * (eq52_e2396_d_n0),
            nodes[2],
            multiplicity * (eq52_e2396_d_n2),
        );
        let (eq53_e2404, eq53_e2404_d_n0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, eq53_e2404_d_n13, eq53_e2404_d_n14, eq53_e2404_q,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq53_e2402_q: f64 = var_qgs_fr;
        (var_qgs_fr, var_qgs_fr_dn0, var_qgs_fr_dn2, var_qgs_fr_dn3, var_qgs_fr_dn4, var_qgs_fr_dn5, var_qgs_fr_dn6, var_qgs_fr_dn7, var_qgs_fr_dn8, var_qgs_fr_dn9, var_qgs_fr_dn10, var_qgs_fr_dn11, var_qgs_fr_dn13, var_qgs_fr_dn14, eq53_e2402_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_reactive_node_derivatives: [f64; 17] = [eq53_e2404_d_n0, 0.0, eq53_e2404_d_n2, eq53_e2404_d_n3, eq53_e2404_d_n4, eq53_e2404_d_n5, eq53_e2404_d_n6, eq53_e2404_d_n7, eq53_e2404_d_n8, eq53_e2404_d_n9, eq53_e2404_d_n10, eq53_e2404_d_n11, 0.0, eq53_e2404_d_n13, eq53_e2404_d_n14, 0.0, 0.0];
        let eq53_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[13]),
            Some(nodes[2]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq54_e2412, eq54_e2412_d_n0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, eq54_e2412_d_n13, eq54_e2412_d_n14, eq54_e2412_q,) = {
    if ((var_guard651 == 0.0) && (var_guard655 != 0.0)) {
        let eq54_e2410_q: f64 = var_qgd_fr;
        (var_qgd_fr, var_qgd_fr_dn0, var_qgd_fr_dn2, var_qgd_fr_dn3, var_qgd_fr_dn4, var_qgd_fr_dn5, var_qgd_fr_dn6, var_qgd_fr_dn7, var_qgd_fr_dn8, var_qgd_fr_dn9, var_qgd_fr_dn10, var_qgd_fr_dn11, var_qgd_fr_dn13, var_qgd_fr_dn14, eq54_e2410_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_reactive_node_derivatives: [f64; 17] = [eq54_e2412_d_n0, 0.0, eq54_e2412_d_n2, eq54_e2412_d_n3, eq54_e2412_d_n4, eq54_e2412_d_n5, eq54_e2412_d_n6, eq54_e2412_d_n7, eq54_e2412_d_n8, eq54_e2412_d_n9, eq54_e2412_d_n10, eq54_e2412_d_n11, 0.0, eq54_e2412_d_n13, eq54_e2412_d_n14, 0.0, 0.0];
        let eq54_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[14]),
            Some(nodes[0]),
            nodes,
            &eq54_reactive_node_derivatives,
            branches,
            &eq54_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cth: f64,
        var_cth_dn0: f64,
        var_cth_dn10: f64,
        var_cth_dn11: f64,
        var_cth_dn13: f64,
        var_cth_dn14: f64,
        var_cth_dn2: f64,
        var_cth_dn3: f64,
        var_cth_dn4: f64,
        var_cth_dn5: f64,
        var_cth_dn6: f64,
        var_cth_dn7: f64,
        var_cth_dn8: f64,
        var_cth_dn9: f64,
        var_devsign: f64,
        var_guard656: f64,
        var_guard669: f64,
        var_guard677: f64,
        var_guard682: f64,
        var_qb_acc: f64,
        var_qb_acc_dn0: f64,
        var_qb_acc_dn10: f64,
        var_qb_acc_dn11: f64,
        var_qb_acc_dn13: f64,
        var_qb_acc_dn14: f64,
        var_qb_acc_dn2: f64,
        var_qb_acc_dn3: f64,
        var_qb_acc_dn4: f64,
        var_qb_acc_dn5: f64,
        var_qb_acc_dn6: f64,
        var_qb_acc_dn7: f64,
        var_qb_acc_dn8: f64,
        var_qb_acc_dn9: f64,
        var_qb_v: f64,
        var_qb_v_dn0: f64,
        var_qb_v_dn10: f64,
        var_qb_v_dn11: f64,
        var_qb_v_dn13: f64,
        var_qb_v_dn14: f64,
        var_qb_v_dn2: f64,
        var_qb_v_dn3: f64,
        var_qb_v_dn4: f64,
        var_qb_v_dn5: f64,
        var_qb_v_dn6: f64,
        var_qb_v_dn7: f64,
        var_qb_v_dn8: f64,
        var_qb_v_dn9: f64,
        var_qg_acc: f64,
        var_qg_acc_dn0: f64,
        var_qg_acc_dn10: f64,
        var_qg_acc_dn11: f64,
        var_qg_acc_dn13: f64,
        var_qg_acc_dn14: f64,
        var_qg_acc_dn2: f64,
        var_qg_acc_dn3: f64,
        var_qg_acc_dn4: f64,
        var_qg_acc_dn5: f64,
        var_qg_acc_dn6: f64,
        var_qg_acc_dn7: f64,
        var_qg_acc_dn8: f64,
        var_qg_acc_dn9: f64,
        var_qg_v: f64,
        var_qg_v_dn0: f64,
        var_qg_v_dn10: f64,
        var_qg_v_dn11: f64,
        var_qg_v_dn13: f64,
        var_qg_v_dn14: f64,
        var_qg_v_dn2: f64,
        var_qg_v_dn3: f64,
        var_qg_v_dn4: f64,
        var_qg_v_dn5: f64,
        var_qg_v_dn6: f64,
        var_qg_v_dn7: f64,
        var_qg_v_dn8: f64,
        var_qg_v_dn9: f64,
        var_sigrat: f64,
        var_sigrat_dn0: f64,
        var_sigrat_dn10: f64,
        var_sigrat_dn11: f64,
        var_sigrat_dn13: f64,
        var_sigrat_dn14: f64,
        var_sigrat_dn2: f64,
        var_sigrat_dn3: f64,
        var_sigrat_dn4: f64,
        var_sigrat_dn5: f64,
        var_sigrat_dn6: f64,
        var_sigrat_dn7: f64,
        var_sigrat_dn8: f64,
        var_sigrat_dn9: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq55_e2419, eq55_e2419_d_n0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, eq55_e2419_d_n13, eq55_e2419_d_n14, eq55_e2419_q,) = {
    if (var_guard656 != 0.0) {
        let eq55_e2416_q: f64 = var_qg_acc;
        let eq55_e2417: f64 = (var_devsign * var_qg_acc);
        let eq55_e2417_d_n0: f64 = (var_devsign * var_qg_acc_dn0);
        let eq55_e2417_d_n2: f64 = (var_devsign * var_qg_acc_dn2);
        let eq55_e2417_d_n3: f64 = (var_devsign * var_qg_acc_dn3);
        let eq55_e2417_d_n4: f64 = (var_devsign * var_qg_acc_dn4);
        let eq55_e2417_d_n5: f64 = (var_devsign * var_qg_acc_dn5);
        let eq55_e2417_d_n6: f64 = (var_devsign * var_qg_acc_dn6);
        let eq55_e2417_d_n7: f64 = (var_devsign * var_qg_acc_dn7);
        let eq55_e2417_d_n8: f64 = (var_devsign * var_qg_acc_dn8);
        let eq55_e2417_d_n9: f64 = (var_devsign * var_qg_acc_dn9);
        let eq55_e2417_d_n10: f64 = (var_devsign * var_qg_acc_dn10);
        let eq55_e2417_d_n11: f64 = (var_devsign * var_qg_acc_dn11);
        let eq55_e2417_d_n13: f64 = (var_devsign * var_qg_acc_dn13);
        let eq55_e2417_d_n14: f64 = (var_devsign * var_qg_acc_dn14);
        let eq55_e2417_q: f64 = (var_devsign * eq55_e2416_q);
        (eq55_e2417, eq55_e2417_d_n0, eq55_e2417_d_n2, eq55_e2417_d_n3, eq55_e2417_d_n4, eq55_e2417_d_n5, eq55_e2417_d_n6, eq55_e2417_d_n7, eq55_e2417_d_n8, eq55_e2417_d_n9, eq55_e2417_d_n10, eq55_e2417_d_n11, eq55_e2417_d_n13, eq55_e2417_d_n14, eq55_e2417_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_reactive_node_derivatives: [f64; 17] = [eq55_e2419_d_n0, 0.0, eq55_e2419_d_n2, eq55_e2419_d_n3, eq55_e2419_d_n4, eq55_e2419_d_n5, eq55_e2419_d_n6, eq55_e2419_d_n7, eq55_e2419_d_n8, eq55_e2419_d_n9, eq55_e2419_d_n10, eq55_e2419_d_n11, 0.0, eq55_e2419_d_n13, eq55_e2419_d_n14, 0.0, 0.0];
        let eq55_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq55_reactive_node_derivatives,
            branches,
            &eq55_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq56_e2426, eq56_e2426_d_n0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, eq56_e2426_d_n13, eq56_e2426_d_n14, eq56_e2426_q,) = {
    if (var_guard656 != 0.0) {
        let eq56_e2423_q: f64 = var_qb_acc;
        let eq56_e2424: f64 = (var_devsign * var_qb_acc);
        let eq56_e2424_d_n0: f64 = (var_devsign * var_qb_acc_dn0);
        let eq56_e2424_d_n2: f64 = (var_devsign * var_qb_acc_dn2);
        let eq56_e2424_d_n3: f64 = (var_devsign * var_qb_acc_dn3);
        let eq56_e2424_d_n4: f64 = (var_devsign * var_qb_acc_dn4);
        let eq56_e2424_d_n5: f64 = (var_devsign * var_qb_acc_dn5);
        let eq56_e2424_d_n6: f64 = (var_devsign * var_qb_acc_dn6);
        let eq56_e2424_d_n7: f64 = (var_devsign * var_qb_acc_dn7);
        let eq56_e2424_d_n8: f64 = (var_devsign * var_qb_acc_dn8);
        let eq56_e2424_d_n9: f64 = (var_devsign * var_qb_acc_dn9);
        let eq56_e2424_d_n10: f64 = (var_devsign * var_qb_acc_dn10);
        let eq56_e2424_d_n11: f64 = (var_devsign * var_qb_acc_dn11);
        let eq56_e2424_d_n13: f64 = (var_devsign * var_qb_acc_dn13);
        let eq56_e2424_d_n14: f64 = (var_devsign * var_qb_acc_dn14);
        let eq56_e2424_q: f64 = (var_devsign * eq56_e2423_q);
        (eq56_e2424, eq56_e2424_d_n0, eq56_e2424_d_n2, eq56_e2424_d_n3, eq56_e2424_d_n4, eq56_e2424_d_n5, eq56_e2424_d_n6, eq56_e2424_d_n7, eq56_e2424_d_n8, eq56_e2424_d_n9, eq56_e2424_d_n10, eq56_e2424_d_n11, eq56_e2424_d_n13, eq56_e2424_d_n14, eq56_e2424_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_reactive_node_derivatives: [f64; 17] = [eq56_e2426_d_n0, 0.0, eq56_e2426_d_n2, eq56_e2426_d_n3, eq56_e2426_d_n4, eq56_e2426_d_n5, eq56_e2426_d_n6, eq56_e2426_d_n7, eq56_e2426_d_n8, eq56_e2426_d_n9, eq56_e2426_d_n10, eq56_e2426_d_n11, 0.0, eq56_e2426_d_n13, eq56_e2426_d_n14, 0.0, 0.0];
        let eq56_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[6]),
            nodes,
            &eq56_reactive_node_derivatives,
            branches,
            &eq56_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq69_e2506, eq69_e2506_d_n0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, eq69_e2506_d_n13, eq69_e2506_d_n14, eq69_e2506_q,) = {
    if (var_guard669 != 0.0) {
        let eq69_e2503: f64 = (var_qg_v - var_qb_v);
        let eq69_e2503_d_n0: f64 = (var_qg_v_dn0 - var_qb_v_dn0);
        let eq69_e2503_d_n2: f64 = (var_qg_v_dn2 - var_qb_v_dn2);
        let eq69_e2503_d_n3: f64 = (var_qg_v_dn3 - var_qb_v_dn3);
        let eq69_e2503_d_n4: f64 = (var_qg_v_dn4 - var_qb_v_dn4);
        let eq69_e2503_d_n5: f64 = (var_qg_v_dn5 - var_qb_v_dn5);
        let eq69_e2503_d_n6: f64 = (var_qg_v_dn6 - var_qb_v_dn6);
        let eq69_e2503_d_n7: f64 = (var_qg_v_dn7 - var_qb_v_dn7);
        let eq69_e2503_d_n8: f64 = (var_qg_v_dn8 - var_qb_v_dn8);
        let eq69_e2503_d_n9: f64 = (var_qg_v_dn9 - var_qb_v_dn9);
        let eq69_e2503_d_n10: f64 = (var_qg_v_dn10 - var_qb_v_dn10);
        let eq69_e2503_d_n11: f64 = (var_qg_v_dn11 - var_qb_v_dn11);
        let eq69_e2503_d_n13: f64 = (var_qg_v_dn13 - var_qb_v_dn13);
        let eq69_e2503_d_n14: f64 = (var_qg_v_dn14 - var_qb_v_dn14);
        let eq69_e2504_q: f64 = eq69_e2503;
        (eq69_e2503, eq69_e2503_d_n0, eq69_e2503_d_n2, eq69_e2503_d_n3, eq69_e2503_d_n4, eq69_e2503_d_n5, eq69_e2503_d_n6, eq69_e2503_d_n7, eq69_e2503_d_n8, eq69_e2503_d_n9, eq69_e2503_d_n10, eq69_e2503_d_n11, eq69_e2503_d_n13, eq69_e2503_d_n14, eq69_e2504_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq69_reactive_node_derivatives: [f64; 17] = [eq69_e2506_d_n0, 0.0, eq69_e2506_d_n2, eq69_e2506_d_n3, eq69_e2506_d_n4, eq69_e2506_d_n5, eq69_e2506_d_n6, eq69_e2506_d_n7, eq69_e2506_d_n8, eq69_e2506_d_n9, eq69_e2506_d_n10, eq69_e2506_d_n11, 0.0, eq69_e2506_d_n13, eq69_e2506_d_n14, 0.0, 0.0];
        let eq69_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq69_reactive_node_derivatives,
            branches,
            &eq69_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq71_e2519, eq71_e2519_d_n15, eq71_e2519_q,) = {
    if (var_guard669 != 0.0) {
        let eq71_e2516_q: f64 = (nv15 - 0.0);
        let eq71_e2517: f64 = (1e-9 * (nv15 - 0.0));
        let eq71_e2517_q: f64 = (1e-9 * eq71_e2516_q);
        (eq71_e2517, 1e-9, eq71_e2517_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq71_e2519_d_n15),
        );
        let (eq96_e2717, eq96_e2717_d_n0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, eq96_e2717_d_n13, eq96_e2717_d_n14, eq96_e2717_d_n16, eq96_e2717_q,) = {
    if (var_guard677 == 0.0) {
        let eq96_e2712: f64 = (0.7071 * var_sigrat);
        let eq96_e2712_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq96_e2712_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq96_e2712_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq96_e2712_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq96_e2712_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq96_e2712_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq96_e2712_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq96_e2712_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq96_e2712_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq96_e2712_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq96_e2712_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq96_e2712_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq96_e2712_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq96_e2714: f64 = (eq96_e2712 * (nv16 - 0.0));
        let eq96_e2714_d_n0: f64 = (eq96_e2712_d_n0 * (nv16 - 0.0));
        let eq96_e2714_d_n2: f64 = (eq96_e2712_d_n2 * (nv16 - 0.0));
        let eq96_e2714_d_n3: f64 = (eq96_e2712_d_n3 * (nv16 - 0.0));
        let eq96_e2714_d_n4: f64 = (eq96_e2712_d_n4 * (nv16 - 0.0));
        let eq96_e2714_d_n5: f64 = (eq96_e2712_d_n5 * (nv16 - 0.0));
        let eq96_e2714_d_n6: f64 = (eq96_e2712_d_n6 * (nv16 - 0.0));
        let eq96_e2714_d_n7: f64 = (eq96_e2712_d_n7 * (nv16 - 0.0));
        let eq96_e2714_d_n8: f64 = (eq96_e2712_d_n8 * (nv16 - 0.0));
        let eq96_e2714_d_n9: f64 = (eq96_e2712_d_n9 * (nv16 - 0.0));
        let eq96_e2714_d_n10: f64 = (eq96_e2712_d_n10 * (nv16 - 0.0));
        let eq96_e2714_d_n11: f64 = (eq96_e2712_d_n11 * (nv16 - 0.0));
        let eq96_e2714_d_n13: f64 = (eq96_e2712_d_n13 * (nv16 - 0.0));
        let eq96_e2714_d_n14: f64 = (eq96_e2712_d_n14 * (nv16 - 0.0));
        let eq96_e2715_q: f64 = eq96_e2714;
        (eq96_e2714, eq96_e2714_d_n0, eq96_e2714_d_n2, eq96_e2714_d_n3, eq96_e2714_d_n4, eq96_e2714_d_n5, eq96_e2714_d_n6, eq96_e2714_d_n7, eq96_e2714_d_n8, eq96_e2714_d_n9, eq96_e2714_d_n10, eq96_e2714_d_n11, eq96_e2714_d_n13, eq96_e2714_d_n14, eq96_e2712, eq96_e2715_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq96_reactive_node_derivatives: [f64; 17] = [eq96_e2717_d_n0, 0.0, eq96_e2717_d_n2, eq96_e2717_d_n3, eq96_e2717_d_n4, eq96_e2717_d_n5, eq96_e2717_d_n6, eq96_e2717_d_n7, eq96_e2717_d_n8, eq96_e2717_d_n9, eq96_e2717_d_n10, eq96_e2717_d_n11, 0.0, eq96_e2717_d_n13, eq96_e2717_d_n14, 0.0, eq96_e2717_d_n16];
        let eq96_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq96_reactive_node_derivatives,
            branches,
            &eq96_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq97_e2727, eq97_e2727_d_n0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, eq97_e2727_d_n13, eq97_e2727_d_n14, eq97_e2727_d_n16, eq97_e2727_q,) = {
    if (var_guard677 == 0.0) {
        let eq97_e2722: f64 = (0.7071 * var_sigrat);
        let eq97_e2722_d_n0: f64 = (0.7071 * var_sigrat_dn0);
        let eq97_e2722_d_n2: f64 = (0.7071 * var_sigrat_dn2);
        let eq97_e2722_d_n3: f64 = (0.7071 * var_sigrat_dn3);
        let eq97_e2722_d_n4: f64 = (0.7071 * var_sigrat_dn4);
        let eq97_e2722_d_n5: f64 = (0.7071 * var_sigrat_dn5);
        let eq97_e2722_d_n6: f64 = (0.7071 * var_sigrat_dn6);
        let eq97_e2722_d_n7: f64 = (0.7071 * var_sigrat_dn7);
        let eq97_e2722_d_n8: f64 = (0.7071 * var_sigrat_dn8);
        let eq97_e2722_d_n9: f64 = (0.7071 * var_sigrat_dn9);
        let eq97_e2722_d_n10: f64 = (0.7071 * var_sigrat_dn10);
        let eq97_e2722_d_n11: f64 = (0.7071 * var_sigrat_dn11);
        let eq97_e2722_d_n13: f64 = (0.7071 * var_sigrat_dn13);
        let eq97_e2722_d_n14: f64 = (0.7071 * var_sigrat_dn14);
        let eq97_e2724: f64 = (eq97_e2722 * (nv16 - 0.0));
        let eq97_e2724_d_n0: f64 = (eq97_e2722_d_n0 * (nv16 - 0.0));
        let eq97_e2724_d_n2: f64 = (eq97_e2722_d_n2 * (nv16 - 0.0));
        let eq97_e2724_d_n3: f64 = (eq97_e2722_d_n3 * (nv16 - 0.0));
        let eq97_e2724_d_n4: f64 = (eq97_e2722_d_n4 * (nv16 - 0.0));
        let eq97_e2724_d_n5: f64 = (eq97_e2722_d_n5 * (nv16 - 0.0));
        let eq97_e2724_d_n6: f64 = (eq97_e2722_d_n6 * (nv16 - 0.0));
        let eq97_e2724_d_n7: f64 = (eq97_e2722_d_n7 * (nv16 - 0.0));
        let eq97_e2724_d_n8: f64 = (eq97_e2722_d_n8 * (nv16 - 0.0));
        let eq97_e2724_d_n9: f64 = (eq97_e2722_d_n9 * (nv16 - 0.0));
        let eq97_e2724_d_n10: f64 = (eq97_e2722_d_n10 * (nv16 - 0.0));
        let eq97_e2724_d_n11: f64 = (eq97_e2722_d_n11 * (nv16 - 0.0));
        let eq97_e2724_d_n13: f64 = (eq97_e2722_d_n13 * (nv16 - 0.0));
        let eq97_e2724_d_n14: f64 = (eq97_e2722_d_n14 * (nv16 - 0.0));
        let eq97_e2725_q: f64 = eq97_e2724;
        (eq97_e2724, eq97_e2724_d_n0, eq97_e2724_d_n2, eq97_e2724_d_n3, eq97_e2724_d_n4, eq97_e2724_d_n5, eq97_e2724_d_n6, eq97_e2724_d_n7, eq97_e2724_d_n8, eq97_e2724_d_n9, eq97_e2724_d_n10, eq97_e2724_d_n11, eq97_e2724_d_n13, eq97_e2724_d_n14, eq97_e2722, eq97_e2725_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq97_reactive_node_derivatives: [f64; 17] = [eq97_e2727_d_n0, 0.0, eq97_e2727_d_n2, eq97_e2727_d_n3, eq97_e2727_d_n4, eq97_e2727_d_n5, eq97_e2727_d_n6, eq97_e2727_d_n7, eq97_e2727_d_n8, eq97_e2727_d_n9, eq97_e2727_d_n10, eq97_e2727_d_n11, 0.0, eq97_e2727_d_n13, eq97_e2727_d_n14, 0.0, eq97_e2727_d_n16];
        let eq97_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[5]),
            nodes,
            &eq97_reactive_node_derivatives,
            branches,
            &eq97_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq111_e2904, eq111_e2904_d_n0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, eq111_e2904_d_n13, eq111_e2904_d_n14, eq111_e2904_q,) = {
    if (var_guard682 != 0.0) {
        let eq111_e2901: f64 = ((nv4 - 0.0) * var_cth);
        let eq111_e2901_d_n0: f64 = ((nv4 - 0.0) * var_cth_dn0);
        let eq111_e2901_d_n2: f64 = ((nv4 - 0.0) * var_cth_dn2);
        let eq111_e2901_d_n3: f64 = ((nv4 - 0.0) * var_cth_dn3);
        let eq111_e2901_d_n4: f64 = (var_cth + ((nv4 - 0.0) * var_cth_dn4));
        let eq111_e2901_d_n5: f64 = ((nv4 - 0.0) * var_cth_dn5);
        let eq111_e2901_d_n6: f64 = ((nv4 - 0.0) * var_cth_dn6);
        let eq111_e2901_d_n7: f64 = ((nv4 - 0.0) * var_cth_dn7);
        let eq111_e2901_d_n8: f64 = ((nv4 - 0.0) * var_cth_dn8);
        let eq111_e2901_d_n9: f64 = ((nv4 - 0.0) * var_cth_dn9);
        let eq111_e2901_d_n10: f64 = ((nv4 - 0.0) * var_cth_dn10);
        let eq111_e2901_d_n11: f64 = ((nv4 - 0.0) * var_cth_dn11);
        let eq111_e2901_d_n13: f64 = ((nv4 - 0.0) * var_cth_dn13);
        let eq111_e2901_d_n14: f64 = ((nv4 - 0.0) * var_cth_dn14);
        let eq111_e2902_q: f64 = eq111_e2901;
        (eq111_e2901, eq111_e2901_d_n0, eq111_e2901_d_n2, eq111_e2901_d_n3, eq111_e2901_d_n4, eq111_e2901_d_n5, eq111_e2901_d_n6, eq111_e2901_d_n7, eq111_e2901_d_n8, eq111_e2901_d_n9, eq111_e2901_d_n10, eq111_e2901_d_n11, eq111_e2901_d_n13, eq111_e2901_d_n14, eq111_e2902_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq111_reactive_node_derivatives: [f64; 17] = [eq111_e2904_d_n0, 0.0, eq111_e2904_d_n2, eq111_e2904_d_n3, eq111_e2904_d_n4, eq111_e2904_d_n5, eq111_e2904_d_n6, eq111_e2904_d_n7, eq111_e2904_d_n8, eq111_e2904_d_n9, eq111_e2904_d_n10, eq111_e2904_d_n11, 0.0, eq111_e2904_d_n13, eq111_e2904_d_n14, 0.0, 0.0];
        let eq111_reactive_branch_derivatives: [f64; 18] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq111_reactive_node_derivatives,
            branches,
            &eq111_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
