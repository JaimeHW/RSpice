#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
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
        var_guard2313: f64,
        var_guard2314: f64,
        var_guard2413: f64,
        var_ibd: f64,
        var_ibd_dn0: f64,
        var_ibd_dn10: f64,
        var_ibd_dn11: f64,
        var_ibd_dn14: f64,
        var_ibd_dn2: f64,
        var_ibd_dn4: f64,
        var_ibd_dn5: f64,
        var_ibd_dn6: f64,
        var_ibd_dn7: f64,
        var_ibd_dn8: f64,
        var_ibd_dn9: f64,
        var_ibdi: f64,
        var_ibdi_dn0: f64,
        var_ibdi_dn10: f64,
        var_ibdi_dn11: f64,
        var_ibdi_dn14: f64,
        var_ibdi_dn2: f64,
        var_ibdi_dn4: f64,
        var_ibdi_dn5: f64,
        var_ibdi_dn6: f64,
        var_ibdi_dn7: f64,
        var_ibdi_dn8: f64,
        var_ibdi_dn9: f64,
        var_ibjt: f64,
        var_ibjt_dn0: f64,
        var_ibjt_dn10: f64,
        var_ibjt_dn11: f64,
        var_ibjt_dn14: f64,
        var_ibjt_dn2: f64,
        var_ibjt_dn4: f64,
        var_ibjt_dn5: f64,
        var_ibjt_dn6: f64,
        var_ibjt_dn7: f64,
        var_ibjt_dn8: f64,
        var_ibjt_dn9: f64,
        var_ibjts: f64,
        var_ibjts_dn0: f64,
        var_ibjts_dn10: f64,
        var_ibjts_dn11: f64,
        var_ibjts_dn14: f64,
        var_ibjts_dn2: f64,
        var_ibjts_dn4: f64,
        var_ibjts_dn5: f64,
        var_ibjts_dn6: f64,
        var_ibjts_dn7: f64,
        var_ibjts_dn8: f64,
        var_ibjts_dn9: f64,
        var_ibreak: f64,
        var_ibreak_dn0: f64,
        var_ibreak_dn10: f64,
        var_ibreak_dn11: f64,
        var_ibreak_dn14: f64,
        var_ibreak_dn2: f64,
        var_ibreak_dn4: f64,
        var_ibreak_dn5: f64,
        var_ibreak_dn6: f64,
        var_ibreak_dn7: f64,
        var_ibreak_dn8: f64,
        var_ibreak_dn9: f64,
        var_ibreaks: f64,
        var_ibreaks_dn0: f64,
        var_ibreaks_dn10: f64,
        var_ibreaks_dn11: f64,
        var_ibreaks_dn14: f64,
        var_ibreaks_dn2: f64,
        var_ibreaks_dn4: f64,
        var_ibreaks_dn5: f64,
        var_ibreaks_dn6: f64,
        var_ibreaks_dn7: f64,
        var_ibreaks_dn8: f64,
        var_ibreaks_dn9: f64,
        var_ibs: f64,
        var_ibs_dn0: f64,
        var_ibs_dn10: f64,
        var_ibs_dn11: f64,
        var_ibs_dn14: f64,
        var_ibs_dn2: f64,
        var_ibs_dn4: f64,
        var_ibs_dn5: f64,
        var_ibs_dn6: f64,
        var_ibs_dn7: f64,
        var_ibs_dn8: f64,
        var_ibs_dn9: f64,
        var_ibsi: f64,
        var_ibsi_dn0: f64,
        var_ibsi_dn10: f64,
        var_ibsi_dn11: f64,
        var_ibsi_dn14: f64,
        var_ibsi_dn2: f64,
        var_ibsi_dn4: f64,
        var_ibsi_dn5: f64,
        var_ibsi_dn6: f64,
        var_ibsi_dn7: f64,
        var_ibsi_dn8: f64,
        var_ibsi_dn9: f64,
        var_ids: f64,
        var_ids_dn0: f64,
        var_ids_dn10: f64,
        var_ids_dn11: f64,
        var_ids_dn14: f64,
        var_ids_dn2: f64,
        var_ids_dn4: f64,
        var_ids_dn5: f64,
        var_ids_dn6: f64,
        var_ids_dn7: f64,
        var_ids_dn8: f64,
        var_ids_dn9: f64,
        var_idsibpc: f64,
        var_idsibpc_dn0: f64,
        var_idsibpc_dn10: f64,
        var_idsibpc_dn11: f64,
        var_idsibpc_dn14: f64,
        var_idsibpc_dn2: f64,
        var_idsibpc_dn4: f64,
        var_idsibpc_dn5: f64,
        var_idsibpc_dn6: f64,
        var_idsibpc_dn7: f64,
        var_idsibpc_dn8: f64,
        var_idsibpc_dn9: f64,
        var_idsibpcs: f64,
        var_idsibpcs_dn0: f64,
        var_idsibpcs_dn10: f64,
        var_idsibpcs_dn11: f64,
        var_idsibpcs_dn14: f64,
        var_idsibpcs_dn2: f64,
        var_idsibpcs_dn4: f64,
        var_idsibpcs_dn5: f64,
        var_idsibpcs_dn6: f64,
        var_idsibpcs_dn7: f64,
        var_idsibpcs_dn8: f64,
        var_idsibpcs_dn9: f64,
        var_igidl: f64,
        var_igidl_dn0: f64,
        var_igidl_dn10: f64,
        var_igidl_dn11: f64,
        var_igidl_dn14: f64,
        var_igidl_dn2: f64,
        var_igidl_dn4: f64,
        var_igidl_dn5: f64,
        var_igidl_dn6: f64,
        var_igidl_dn7: f64,
        var_igidl_dn8: f64,
        var_igidl_dn9: f64,
        var_igisl: f64,
        var_igisl_dn0: f64,
        var_igisl_dn10: f64,
        var_igisl_dn11: f64,
        var_igisl_dn14: f64,
        var_igisl_dn2: f64,
        var_igisl_dn4: f64,
        var_igisl_dn5: f64,
        var_igisl_dn6: f64,
        var_igisl_dn7: f64,
        var_igisl_dn8: f64,
        var_igisl_dn9: f64,
        var_inqs0_a: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn10: f64,
        var_inqs0_a_dn11: f64,
        var_inqs0_a_dn14: f64,
        var_inqs0_a_dn16: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn4: f64,
        var_inqs0_a_dn5: f64,
        var_inqs0_a_dn6: f64,
        var_inqs0_a_dn7: f64,
        var_inqs0_a_dn8: f64,
        var_inqs0_a_dn9: f64,
        var_inqs0_k: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn10: f64,
        var_inqs0_k_dn11: f64,
        var_inqs0_k_dn14: f64,
        var_inqs0_k_dn17: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn4: f64,
        var_inqs0_k_dn5: f64,
        var_inqs0_k_dn6: f64,
        var_inqs0_k_dn7: f64,
        var_inqs0_k_dn8: f64,
        var_inqs0_k_dn9: f64,
        var_isub: f64,
        var_isub_dn0: f64,
        var_isub_dn10: f64,
        var_isub_dn11: f64,
        var_isub_dn14: f64,
        var_isub_dn2: f64,
        var_isub_dn4: f64,
        var_isub_dn5: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_isubld: f64,
        var_isubld_dn0: f64,
        var_isubld_dn10: f64,
        var_isubld_dn11: f64,
        var_isubld_dn14: f64,
        var_isubld_dn2: f64,
        var_isubld_dn4: f64,
        var_isubld_dn5: f64,
        var_isubld_dn6: f64,
        var_isubld_dn7: f64,
        var_isubld_dn8: f64,
        var_isubld_dn9: f64,
        var_isublds: f64,
        var_isublds_dn0: f64,
        var_isublds_dn10: f64,
        var_isublds_dn11: f64,
        var_isublds_dn14: f64,
        var_isublds_dn2: f64,
        var_isublds_dn4: f64,
        var_isublds_dn5: f64,
        var_isublds_dn6: f64,
        var_isublds_dn7: f64,
        var_isublds_dn8: f64,
        var_isublds_dn9: f64,
        var_isubs: f64,
        var_isubs_dn0: f64,
        var_isubs_dn10: f64,
        var_isubs_dn11: f64,
        var_isubs_dn14: f64,
        var_isubs_dn2: f64,
        var_isubs_dn4: f64,
        var_isubs_dn5: f64,
        var_isubs_dn6: f64,
        var_isubs_dn7: f64,
        var_isubs_dn8: f64,
        var_isubs_dn9: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn10: f64,
        var_iwnqs0_a_dn11: f64,
        var_iwnqs0_a_dn14: f64,
        var_iwnqs0_a_dn18: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn4: f64,
        var_iwnqs0_a_dn5: f64,
        var_iwnqs0_a_dn6: f64,
        var_iwnqs0_a_dn7: f64,
        var_iwnqs0_a_dn8: f64,
        var_iwnqs0_a_dn9: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_dn16: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_dn17: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn14: f64,
        var_qbd_dn16: f64,
        var_qbd_dn17: f64,
        var_qbd_dn18: f64,
        var_qbd_dn2: f64,
        var_qbd_dn4: f64,
        var_qbd_dn5: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_qbd_dn8: f64,
        var_qbd_dn9: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn14: f64,
        var_qbs_dn2: f64,
        var_qbs_dn4: f64,
        var_qbs_dn5: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbs_dn8: f64,
        var_qbs_dn9: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_dn18: f64,
    ) {
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16,) = {
    if (var_guard2313 != 0.0) {
        let eq1_e1019: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_q_nqs_a);
        let eq1_e1020: f64 = (var_inqs0_a + eq1_e1019);
        let eq1_e1020_d_n16: f64 = (var_inqs0_a_dn16 + (var_q_nqs_a_dn16 * ddt_scale));
        (eq1_e1020, var_inqs0_a_dn0, var_inqs0_a_dn2, var_inqs0_a_dn4, var_inqs0_a_dn5, var_inqs0_a_dn6, var_inqs0_a_dn7, var_inqs0_a_dn8, var_inqs0_a_dn9, var_inqs0_a_dn10, var_inqs0_a_dn11, var_inqs0_a_dn14, eq1_e1020_d_n16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1022;
        let eq1_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16];
        let eq1_node_derivatives: [f64; 12] = [eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17,) = {
    if (var_guard2313 != 0.0) {
        let eq2_e1026: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_q_nqs_k);
        let eq2_e1027: f64 = (var_inqs0_k + eq2_e1026);
        let eq2_e1027_d_n17: f64 = (var_inqs0_k_dn17 + (var_q_nqs_k_dn17 * ddt_scale));
        (eq2_e1027, var_inqs0_k_dn0, var_inqs0_k_dn2, var_inqs0_k_dn4, var_inqs0_k_dn5, var_inqs0_k_dn6, var_inqs0_k_dn7, var_inqs0_k_dn8, var_inqs0_k_dn9, var_inqs0_k_dn10, var_inqs0_k_dn11, var_inqs0_k_dn14, eq2_e1027_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1029;
        let eq2_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 17];
        let eq2_node_derivatives: [f64; 12] = [eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(17),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18,) = {
    if (var_guard2314 != 0.0) {
        let eq5_e1043: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_w_nqs_a);
        let eq5_e1044: f64 = (var_iwnqs0_a + eq5_e1043);
        let eq5_e1044_d_n18: f64 = (var_iwnqs0_a_dn18 + (var_w_nqs_a_dn18 * ddt_scale));
        (eq5_e1044, var_iwnqs0_a_dn0, var_iwnqs0_a_dn2, var_iwnqs0_a_dn4, var_iwnqs0_a_dn5, var_iwnqs0_a_dn6, var_iwnqs0_a_dn7, var_iwnqs0_a_dn8, var_iwnqs0_a_dn9, var_iwnqs0_a_dn10, var_iwnqs0_a_dn11, var_iwnqs0_a_dn14, eq5_e1044_d_n18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1046;
        let eq5_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 18];
        let eq5_node_derivatives: [f64; 12] = [eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(18),
            None,
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq7_e1055: f64 = (var_ids + var_idsibpc);
        let eq7_e1055_d_n0: f64 = (var_ids_dn0 + var_idsibpc_dn0);
        let eq7_e1055_d_n2: f64 = (var_ids_dn2 + var_idsibpc_dn2);
        let eq7_e1055_d_n4: f64 = (var_ids_dn4 + var_idsibpc_dn4);
        let eq7_e1055_d_n5: f64 = (var_ids_dn5 + var_idsibpc_dn5);
        let eq7_e1055_d_n6: f64 = (var_ids_dn6 + var_idsibpc_dn6);
        let eq7_e1055_d_n7: f64 = (var_ids_dn7 + var_idsibpc_dn7);
        let eq7_e1055_d_n8: f64 = (var_ids_dn8 + var_idsibpc_dn8);
        let eq7_e1055_d_n9: f64 = (var_ids_dn9 + var_idsibpc_dn9);
        let eq7_e1055_d_n10: f64 = (var_ids_dn10 + var_idsibpc_dn10);
        let eq7_e1055_d_n11: f64 = (var_ids_dn11 + var_idsibpc_dn11);
        let eq7_e1055_d_n14: f64 = (var_ids_dn14 + var_idsibpc_dn14);
        let eq7_e1057: f64 = (eq7_e1055 - var_idsibpcs);
        let eq7_e1057_d_n0: f64 = (eq7_e1055_d_n0 - var_idsibpcs_dn0);
        let eq7_e1057_d_n2: f64 = (eq7_e1055_d_n2 - var_idsibpcs_dn2);
        let eq7_e1057_d_n4: f64 = (eq7_e1055_d_n4 - var_idsibpcs_dn4);
        let eq7_e1057_d_n5: f64 = (eq7_e1055_d_n5 - var_idsibpcs_dn5);
        let eq7_e1057_d_n6: f64 = (eq7_e1055_d_n6 - var_idsibpcs_dn6);
        let eq7_e1057_d_n7: f64 = (eq7_e1055_d_n7 - var_idsibpcs_dn7);
        let eq7_e1057_d_n8: f64 = (eq7_e1055_d_n8 - var_idsibpcs_dn8);
        let eq7_e1057_d_n9: f64 = (eq7_e1055_d_n9 - var_idsibpcs_dn9);
        let eq7_e1057_d_n10: f64 = (eq7_e1055_d_n10 - var_idsibpcs_dn10);
        let eq7_e1057_d_n11: f64 = (eq7_e1055_d_n11 - var_idsibpcs_dn11);
        let eq7_e1057_d_n14: f64 = (eq7_e1055_d_n14 - var_idsibpcs_dn14);
        let eq7_e1058: f64 = (p.p87 * eq7_e1057);
        let eq7_e1058_d_n0: f64 = (p.p87 * eq7_e1057_d_n0);
        let eq7_e1058_d_n2: f64 = (p.p87 * eq7_e1057_d_n2);
        let eq7_e1058_d_n4: f64 = (p.p87 * eq7_e1057_d_n4);
        let eq7_e1058_d_n5: f64 = (p.p87 * eq7_e1057_d_n5);
        let eq7_e1058_d_n6: f64 = (p.p87 * eq7_e1057_d_n6);
        let eq7_e1058_d_n7: f64 = (p.p87 * eq7_e1057_d_n7);
        let eq7_e1058_d_n8: f64 = (p.p87 * eq7_e1057_d_n8);
        let eq7_e1058_d_n9: f64 = (p.p87 * eq7_e1057_d_n9);
        let eq7_e1058_d_n10: f64 = (p.p87 * eq7_e1057_d_n10);
        let eq7_e1058_d_n11: f64 = (p.p87 * eq7_e1057_d_n11);
        let eq7_e1058_d_n14: f64 = (p.p87 * eq7_e1057_d_n14);
        let eq7_value: f64 = eq7_e1058;
        let eq7_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq7_node_derivatives: [f64; 11] = [eq7_e1058_d_n0, eq7_e1058_d_n2, eq7_e1058_d_n4, eq7_e1058_d_n5, eq7_e1058_d_n6, eq7_e1058_d_n7, eq7_e1058_d_n8, eq7_e1058_d_n9, eq7_e1058_d_n10, eq7_e1058_d_n11, eq7_e1058_d_n14];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1062: f64 = (var_ibreak - var_ibreaks);
        let eq8_e1062_d_n0: f64 = (var_ibreak_dn0 - var_ibreaks_dn0);
        let eq8_e1062_d_n2: f64 = (var_ibreak_dn2 - var_ibreaks_dn2);
        let eq8_e1062_d_n4: f64 = (var_ibreak_dn4 - var_ibreaks_dn4);
        let eq8_e1062_d_n5: f64 = (var_ibreak_dn5 - var_ibreaks_dn5);
        let eq8_e1062_d_n6: f64 = (var_ibreak_dn6 - var_ibreaks_dn6);
        let eq8_e1062_d_n7: f64 = (var_ibreak_dn7 - var_ibreaks_dn7);
        let eq8_e1062_d_n8: f64 = (var_ibreak_dn8 - var_ibreaks_dn8);
        let eq8_e1062_d_n9: f64 = (var_ibreak_dn9 - var_ibreaks_dn9);
        let eq8_e1062_d_n10: f64 = (var_ibreak_dn10 - var_ibreaks_dn10);
        let eq8_e1062_d_n11: f64 = (var_ibreak_dn11 - var_ibreaks_dn11);
        let eq8_e1062_d_n14: f64 = (var_ibreak_dn14 - var_ibreaks_dn14);
        let eq8_e1063: f64 = (p.p87 * eq8_e1062);
        let eq8_e1063_d_n0: f64 = (p.p87 * eq8_e1062_d_n0);
        let eq8_e1063_d_n2: f64 = (p.p87 * eq8_e1062_d_n2);
        let eq8_e1063_d_n4: f64 = (p.p87 * eq8_e1062_d_n4);
        let eq8_e1063_d_n5: f64 = (p.p87 * eq8_e1062_d_n5);
        let eq8_e1063_d_n6: f64 = (p.p87 * eq8_e1062_d_n6);
        let eq8_e1063_d_n7: f64 = (p.p87 * eq8_e1062_d_n7);
        let eq8_e1063_d_n8: f64 = (p.p87 * eq8_e1062_d_n8);
        let eq8_e1063_d_n9: f64 = (p.p87 * eq8_e1062_d_n9);
        let eq8_e1063_d_n10: f64 = (p.p87 * eq8_e1062_d_n10);
        let eq8_e1063_d_n11: f64 = (p.p87 * eq8_e1062_d_n11);
        let eq8_e1063_d_n14: f64 = (p.p87 * eq8_e1062_d_n14);
        let eq8_value: f64 = eq8_e1063;
        let eq8_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq8_node_derivatives: [f64; 11] = [eq8_e1063_d_n0, eq8_e1063_d_n2, eq8_e1063_d_n4, eq8_e1063_d_n5, eq8_e1063_d_n6, eq8_e1063_d_n7, eq8_e1063_d_n8, eq8_e1063_d_n9, eq8_e1063_d_n10, eq8_e1063_d_n11, eq8_e1063_d_n14];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1067: f64 = (var_igidl + var_isub);
        let eq9_e1067_d_n0: f64 = (var_igidl_dn0 + var_isub_dn0);
        let eq9_e1067_d_n2: f64 = (var_igidl_dn2 + var_isub_dn2);
        let eq9_e1067_d_n4: f64 = (var_igidl_dn4 + var_isub_dn4);
        let eq9_e1067_d_n5: f64 = (var_igidl_dn5 + var_isub_dn5);
        let eq9_e1067_d_n6: f64 = (var_igidl_dn6 + var_isub_dn6);
        let eq9_e1067_d_n7: f64 = (var_igidl_dn7 + var_isub_dn7);
        let eq9_e1067_d_n8: f64 = (var_igidl_dn8 + var_isub_dn8);
        let eq9_e1067_d_n9: f64 = (var_igidl_dn9 + var_isub_dn9);
        let eq9_e1067_d_n10: f64 = (var_igidl_dn10 + var_isub_dn10);
        let eq9_e1067_d_n11: f64 = (var_igidl_dn11 + var_isub_dn11);
        let eq9_e1067_d_n14: f64 = (var_igidl_dn14 + var_isub_dn14);
        let eq9_e1069: f64 = (eq9_e1067 + var_ibjt);
        let eq9_e1069_d_n0: f64 = (eq9_e1067_d_n0 + var_ibjt_dn0);
        let eq9_e1069_d_n2: f64 = (eq9_e1067_d_n2 + var_ibjt_dn2);
        let eq9_e1069_d_n4: f64 = (eq9_e1067_d_n4 + var_ibjt_dn4);
        let eq9_e1069_d_n5: f64 = (eq9_e1067_d_n5 + var_ibjt_dn5);
        let eq9_e1069_d_n6: f64 = (eq9_e1067_d_n6 + var_ibjt_dn6);
        let eq9_e1069_d_n7: f64 = (eq9_e1067_d_n7 + var_ibjt_dn7);
        let eq9_e1069_d_n8: f64 = (eq9_e1067_d_n8 + var_ibjt_dn8);
        let eq9_e1069_d_n9: f64 = (eq9_e1067_d_n9 + var_ibjt_dn9);
        let eq9_e1069_d_n10: f64 = (eq9_e1067_d_n10 + var_ibjt_dn10);
        let eq9_e1069_d_n11: f64 = (eq9_e1067_d_n11 + var_ibjt_dn11);
        let eq9_e1069_d_n14: f64 = (eq9_e1067_d_n14 + var_ibjt_dn14);
        let eq9_e1070: f64 = (p.p87 * eq9_e1069);
        let eq9_e1070_d_n0: f64 = (p.p87 * eq9_e1069_d_n0);
        let eq9_e1070_d_n2: f64 = (p.p87 * eq9_e1069_d_n2);
        let eq9_e1070_d_n4: f64 = (p.p87 * eq9_e1069_d_n4);
        let eq9_e1070_d_n5: f64 = (p.p87 * eq9_e1069_d_n5);
        let eq9_e1070_d_n6: f64 = (p.p87 * eq9_e1069_d_n6);
        let eq9_e1070_d_n7: f64 = (p.p87 * eq9_e1069_d_n7);
        let eq9_e1070_d_n8: f64 = (p.p87 * eq9_e1069_d_n8);
        let eq9_e1070_d_n9: f64 = (p.p87 * eq9_e1069_d_n9);
        let eq9_e1070_d_n10: f64 = (p.p87 * eq9_e1069_d_n10);
        let eq9_e1070_d_n11: f64 = (p.p87 * eq9_e1069_d_n11);
        let eq9_e1070_d_n14: f64 = (p.p87 * eq9_e1069_d_n14);
        let eq9_value: f64 = eq9_e1070;
        let eq9_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq9_node_derivatives: [f64; 11] = [eq9_e1070_d_n0, eq9_e1070_d_n2, eq9_e1070_d_n4, eq9_e1070_d_n5, eq9_e1070_d_n6, eq9_e1070_d_n7, eq9_e1070_d_n8, eq9_e1070_d_n9, eq9_e1070_d_n10, eq9_e1070_d_n11, eq9_e1070_d_n14];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1074: f64 = (var_igisl + var_isubs);
        let eq10_e1074_d_n0: f64 = (var_igisl_dn0 + var_isubs_dn0);
        let eq10_e1074_d_n2: f64 = (var_igisl_dn2 + var_isubs_dn2);
        let eq10_e1074_d_n4: f64 = (var_igisl_dn4 + var_isubs_dn4);
        let eq10_e1074_d_n5: f64 = (var_igisl_dn5 + var_isubs_dn5);
        let eq10_e1074_d_n6: f64 = (var_igisl_dn6 + var_isubs_dn6);
        let eq10_e1074_d_n7: f64 = (var_igisl_dn7 + var_isubs_dn7);
        let eq10_e1074_d_n8: f64 = (var_igisl_dn8 + var_isubs_dn8);
        let eq10_e1074_d_n9: f64 = (var_igisl_dn9 + var_isubs_dn9);
        let eq10_e1074_d_n10: f64 = (var_igisl_dn10 + var_isubs_dn10);
        let eq10_e1074_d_n11: f64 = (var_igisl_dn11 + var_isubs_dn11);
        let eq10_e1074_d_n14: f64 = (var_igisl_dn14 + var_isubs_dn14);
        let eq10_e1076: f64 = (eq10_e1074 + var_ibjts);
        let eq10_e1076_d_n0: f64 = (eq10_e1074_d_n0 + var_ibjts_dn0);
        let eq10_e1076_d_n2: f64 = (eq10_e1074_d_n2 + var_ibjts_dn2);
        let eq10_e1076_d_n4: f64 = (eq10_e1074_d_n4 + var_ibjts_dn4);
        let eq10_e1076_d_n5: f64 = (eq10_e1074_d_n5 + var_ibjts_dn5);
        let eq10_e1076_d_n6: f64 = (eq10_e1074_d_n6 + var_ibjts_dn6);
        let eq10_e1076_d_n7: f64 = (eq10_e1074_d_n7 + var_ibjts_dn7);
        let eq10_e1076_d_n8: f64 = (eq10_e1074_d_n8 + var_ibjts_dn8);
        let eq10_e1076_d_n9: f64 = (eq10_e1074_d_n9 + var_ibjts_dn9);
        let eq10_e1076_d_n10: f64 = (eq10_e1074_d_n10 + var_ibjts_dn10);
        let eq10_e1076_d_n11: f64 = (eq10_e1074_d_n11 + var_ibjts_dn11);
        let eq10_e1076_d_n14: f64 = (eq10_e1074_d_n14 + var_ibjts_dn14);
        let eq10_e1077: f64 = (p.p87 * eq10_e1076);
        let eq10_e1077_d_n0: f64 = (p.p87 * eq10_e1076_d_n0);
        let eq10_e1077_d_n2: f64 = (p.p87 * eq10_e1076_d_n2);
        let eq10_e1077_d_n4: f64 = (p.p87 * eq10_e1076_d_n4);
        let eq10_e1077_d_n5: f64 = (p.p87 * eq10_e1076_d_n5);
        let eq10_e1077_d_n6: f64 = (p.p87 * eq10_e1076_d_n6);
        let eq10_e1077_d_n7: f64 = (p.p87 * eq10_e1076_d_n7);
        let eq10_e1077_d_n8: f64 = (p.p87 * eq10_e1076_d_n8);
        let eq10_e1077_d_n9: f64 = (p.p87 * eq10_e1076_d_n9);
        let eq10_e1077_d_n10: f64 = (p.p87 * eq10_e1076_d_n10);
        let eq10_e1077_d_n11: f64 = (p.p87 * eq10_e1076_d_n11);
        let eq10_e1077_d_n14: f64 = (p.p87 * eq10_e1076_d_n14);
        let eq10_value: f64 = eq10_e1077;
        let eq10_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq10_node_derivatives: [f64; 11] = [eq10_e1077_d_n0, eq10_e1077_d_n2, eq10_e1077_d_n4, eq10_e1077_d_n5, eq10_e1077_d_n6, eq10_e1077_d_n7, eq10_e1077_d_n8, eq10_e1077_d_n9, eq10_e1077_d_n10, eq10_e1077_d_n11, eq10_e1077_d_n14];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1080: f64 = (p.p87 * var_isubld);
        let eq11_e1080_d_n0: f64 = (p.p87 * var_isubld_dn0);
        let eq11_e1080_d_n2: f64 = (p.p87 * var_isubld_dn2);
        let eq11_e1080_d_n4: f64 = (p.p87 * var_isubld_dn4);
        let eq11_e1080_d_n5: f64 = (p.p87 * var_isubld_dn5);
        let eq11_e1080_d_n6: f64 = (p.p87 * var_isubld_dn6);
        let eq11_e1080_d_n7: f64 = (p.p87 * var_isubld_dn7);
        let eq11_e1080_d_n8: f64 = (p.p87 * var_isubld_dn8);
        let eq11_e1080_d_n9: f64 = (p.p87 * var_isubld_dn9);
        let eq11_e1080_d_n10: f64 = (p.p87 * var_isubld_dn10);
        let eq11_e1080_d_n11: f64 = (p.p87 * var_isubld_dn11);
        let eq11_e1080_d_n14: f64 = (p.p87 * var_isubld_dn14);
        let eq11_value: f64 = eq11_e1080;
        let eq11_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq11_node_derivatives: [f64; 11] = [eq11_e1080_d_n0, eq11_e1080_d_n2, eq11_e1080_d_n4, eq11_e1080_d_n5, eq11_e1080_d_n6, eq11_e1080_d_n7, eq11_e1080_d_n8, eq11_e1080_d_n9, eq11_e1080_d_n10, eq11_e1080_d_n11, eq11_e1080_d_n14];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1083: f64 = (p.p87 * var_isublds);
        let eq12_e1083_d_n0: f64 = (p.p87 * var_isublds_dn0);
        let eq12_e1083_d_n2: f64 = (p.p87 * var_isublds_dn2);
        let eq12_e1083_d_n4: f64 = (p.p87 * var_isublds_dn4);
        let eq12_e1083_d_n5: f64 = (p.p87 * var_isublds_dn5);
        let eq12_e1083_d_n6: f64 = (p.p87 * var_isublds_dn6);
        let eq12_e1083_d_n7: f64 = (p.p87 * var_isublds_dn7);
        let eq12_e1083_d_n8: f64 = (p.p87 * var_isublds_dn8);
        let eq12_e1083_d_n9: f64 = (p.p87 * var_isublds_dn9);
        let eq12_e1083_d_n10: f64 = (p.p87 * var_isublds_dn10);
        let eq12_e1083_d_n11: f64 = (p.p87 * var_isublds_dn11);
        let eq12_e1083_d_n14: f64 = (p.p87 * var_isublds_dn14);
        let eq12_value: f64 = eq12_e1083;
        let eq12_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq12_node_derivatives: [f64; 11] = [eq12_e1083_d_n0, eq12_e1083_d_n2, eq12_e1083_d_n4, eq12_e1083_d_n5, eq12_e1083_d_n6, eq12_e1083_d_n7, eq12_e1083_d_n8, eq12_e1083_d_n9, eq12_e1083_d_n10, eq12_e1083_d_n11, eq12_e1083_d_n14];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(9),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1086: f64 = (p.p87 * var_ibs);
        let eq13_e1086_d_n0: f64 = (p.p87 * var_ibs_dn0);
        let eq13_e1086_d_n2: f64 = (p.p87 * var_ibs_dn2);
        let eq13_e1086_d_n4: f64 = (p.p87 * var_ibs_dn4);
        let eq13_e1086_d_n5: f64 = (p.p87 * var_ibs_dn5);
        let eq13_e1086_d_n6: f64 = (p.p87 * var_ibs_dn6);
        let eq13_e1086_d_n7: f64 = (p.p87 * var_ibs_dn7);
        let eq13_e1086_d_n8: f64 = (p.p87 * var_ibs_dn8);
        let eq13_e1086_d_n9: f64 = (p.p87 * var_ibs_dn9);
        let eq13_e1086_d_n10: f64 = (p.p87 * var_ibs_dn10);
        let eq13_e1086_d_n11: f64 = (p.p87 * var_ibs_dn11);
        let eq13_e1086_d_n14: f64 = (p.p87 * var_ibs_dn14);
        let eq13_value: f64 = eq13_e1086;
        let eq13_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq13_node_derivatives: [f64; 11] = [eq13_e1086_d_n0, eq13_e1086_d_n2, eq13_e1086_d_n4, eq13_e1086_d_n5, eq13_e1086_d_n6, eq13_e1086_d_n7, eq13_e1086_d_n8, eq13_e1086_d_n9, eq13_e1086_d_n10, eq13_e1086_d_n11, eq13_e1086_d_n14];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1089: f64 = (p.p87 * var_ibd);
        let eq14_e1089_d_n0: f64 = (p.p87 * var_ibd_dn0);
        let eq14_e1089_d_n2: f64 = (p.p87 * var_ibd_dn2);
        let eq14_e1089_d_n4: f64 = (p.p87 * var_ibd_dn4);
        let eq14_e1089_d_n5: f64 = (p.p87 * var_ibd_dn5);
        let eq14_e1089_d_n6: f64 = (p.p87 * var_ibd_dn6);
        let eq14_e1089_d_n7: f64 = (p.p87 * var_ibd_dn7);
        let eq14_e1089_d_n8: f64 = (p.p87 * var_ibd_dn8);
        let eq14_e1089_d_n9: f64 = (p.p87 * var_ibd_dn9);
        let eq14_e1089_d_n10: f64 = (p.p87 * var_ibd_dn10);
        let eq14_e1089_d_n11: f64 = (p.p87 * var_ibd_dn11);
        let eq14_e1089_d_n14: f64 = (p.p87 * var_ibd_dn14);
        let eq14_value: f64 = eq14_e1089;
        let eq14_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq14_node_derivatives: [f64; 11] = [eq14_e1089_d_n0, eq14_e1089_d_n2, eq14_e1089_d_n4, eq14_e1089_d_n5, eq14_e1089_d_n6, eq14_e1089_d_n7, eq14_e1089_d_n8, eq14_e1089_d_n9, eq14_e1089_d_n10, eq14_e1089_d_n11, eq14_e1089_d_n14];
        let eq14_branch_derivative_indices: [usize; 0] = [];
        let eq14_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq14_value),
            &eq14_node_derivative_indices,
            &eq14_node_derivatives,
            &eq14_branch_derivative_indices,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e1092: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, var_qbs);
        let eq15_e1093: f64 = (p.p87 * eq15_e1092);
        let eq15_e1093_d_n0: f64 = (p.p87 * (var_qbs_dn0 * ddt_scale));
        let eq15_e1093_d_n2: f64 = (p.p87 * (var_qbs_dn2 * ddt_scale));
        let eq15_e1093_d_n4: f64 = (p.p87 * (var_qbs_dn4 * ddt_scale));
        let eq15_e1093_d_n5: f64 = (p.p87 * (var_qbs_dn5 * ddt_scale));
        let eq15_e1093_d_n6: f64 = (p.p87 * (var_qbs_dn6 * ddt_scale));
        let eq15_e1093_d_n7: f64 = (p.p87 * (var_qbs_dn7 * ddt_scale));
        let eq15_e1093_d_n8: f64 = (p.p87 * (var_qbs_dn8 * ddt_scale));
        let eq15_e1093_d_n9: f64 = (p.p87 * (var_qbs_dn9 * ddt_scale));
        let eq15_e1093_d_n10: f64 = (p.p87 * (var_qbs_dn10 * ddt_scale));
        let eq15_e1093_d_n11: f64 = (p.p87 * (var_qbs_dn11 * ddt_scale));
        let eq15_e1093_d_n14: f64 = (p.p87 * (var_qbs_dn14 * ddt_scale));
        let eq15_value: f64 = eq15_e1093;
        let eq15_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq15_node_derivatives: [f64; 11] = [eq15_e1093_d_n0, eq15_e1093_d_n2, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, eq15_e1093_d_n14];
        let eq15_branch_derivative_indices: [usize; 0] = [];
        let eq15_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(2),
            multiplicity * (eq15_value),
            &eq15_node_derivative_indices,
            &eq15_node_derivatives,
            &eq15_branch_derivative_indices,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, var_qbd);
        let eq16_e1097: f64 = (p.p87 * eq16_e1096);
        let eq16_e1097_d_n0: f64 = (p.p87 * (var_qbd_dn0 * ddt_scale));
        let eq16_e1097_d_n2: f64 = (p.p87 * (var_qbd_dn2 * ddt_scale));
        let eq16_e1097_d_n4: f64 = (p.p87 * (var_qbd_dn4 * ddt_scale));
        let eq16_e1097_d_n5: f64 = (p.p87 * (var_qbd_dn5 * ddt_scale));
        let eq16_e1097_d_n6: f64 = (p.p87 * (var_qbd_dn6 * ddt_scale));
        let eq16_e1097_d_n7: f64 = (p.p87 * (var_qbd_dn7 * ddt_scale));
        let eq16_e1097_d_n8: f64 = (p.p87 * (var_qbd_dn8 * ddt_scale));
        let eq16_e1097_d_n9: f64 = (p.p87 * (var_qbd_dn9 * ddt_scale));
        let eq16_e1097_d_n10: f64 = (p.p87 * (var_qbd_dn10 * ddt_scale));
        let eq16_e1097_d_n11: f64 = (p.p87 * (var_qbd_dn11 * ddt_scale));
        let eq16_e1097_d_n14: f64 = (p.p87 * (var_qbd_dn14 * ddt_scale));
        let eq16_e1097_d_n16: f64 = (p.p87 * (var_qbd_dn16 * ddt_scale));
        let eq16_e1097_d_n17: f64 = (p.p87 * (var_qbd_dn17 * ddt_scale));
        let eq16_e1097_d_n18: f64 = (p.p87 * (var_qbd_dn18 * ddt_scale));
        let eq16_value: f64 = eq16_e1097;
        let eq16_node_derivative_indices: [usize; 14] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 16, 17, 18];
        let eq16_node_derivatives: [f64; 14] = [eq16_e1097_d_n0, eq16_e1097_d_n2, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, eq16_e1097_d_n14, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(0),
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1103, eq17_e1103_d_n0, eq17_e1103_d_n2, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n14,) = {
    if (var_guard2413 != 0.0) {
        let eq17_e1101: f64 = (p.p87 * var_ibsi);
        let eq17_e1101_d_n0: f64 = (p.p87 * var_ibsi_dn0);
        let eq17_e1101_d_n2: f64 = (p.p87 * var_ibsi_dn2);
        let eq17_e1101_d_n4: f64 = (p.p87 * var_ibsi_dn4);
        let eq17_e1101_d_n5: f64 = (p.p87 * var_ibsi_dn5);
        let eq17_e1101_d_n6: f64 = (p.p87 * var_ibsi_dn6);
        let eq17_e1101_d_n7: f64 = (p.p87 * var_ibsi_dn7);
        let eq17_e1101_d_n8: f64 = (p.p87 * var_ibsi_dn8);
        let eq17_e1101_d_n9: f64 = (p.p87 * var_ibsi_dn9);
        let eq17_e1101_d_n10: f64 = (p.p87 * var_ibsi_dn10);
        let eq17_e1101_d_n11: f64 = (p.p87 * var_ibsi_dn11);
        let eq17_e1101_d_n14: f64 = (p.p87 * var_ibsi_dn14);
        (eq17_e1101, eq17_e1101_d_n0, eq17_e1101_d_n2, eq17_e1101_d_n4, eq17_e1101_d_n5, eq17_e1101_d_n6, eq17_e1101_d_n7, eq17_e1101_d_n8, eq17_e1101_d_n9, eq17_e1101_d_n10, eq17_e1101_d_n11, eq17_e1101_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq17_value: f64 = eq17_e1103;
        let eq17_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq17_node_derivatives: [f64; 11] = [eq17_e1103_d_n0, eq17_e1103_d_n2, eq17_e1103_d_n4, eq17_e1103_d_n5, eq17_e1103_d_n6, eq17_e1103_d_n7, eq17_e1103_d_n8, eq17_e1103_d_n9, eq17_e1103_d_n10, eq17_e1103_d_n11, eq17_e1103_d_n14];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let (eq18_e1109, eq18_e1109_d_n0, eq18_e1109_d_n2, eq18_e1109_d_n4, eq18_e1109_d_n5, eq18_e1109_d_n6, eq18_e1109_d_n7, eq18_e1109_d_n8, eq18_e1109_d_n9, eq18_e1109_d_n10, eq18_e1109_d_n11, eq18_e1109_d_n14,) = {
    if (var_guard2413 != 0.0) {
        let eq18_e1107: f64 = (p.p87 * var_ibdi);
        let eq18_e1107_d_n0: f64 = (p.p87 * var_ibdi_dn0);
        let eq18_e1107_d_n2: f64 = (p.p87 * var_ibdi_dn2);
        let eq18_e1107_d_n4: f64 = (p.p87 * var_ibdi_dn4);
        let eq18_e1107_d_n5: f64 = (p.p87 * var_ibdi_dn5);
        let eq18_e1107_d_n6: f64 = (p.p87 * var_ibdi_dn6);
        let eq18_e1107_d_n7: f64 = (p.p87 * var_ibdi_dn7);
        let eq18_e1107_d_n8: f64 = (p.p87 * var_ibdi_dn8);
        let eq18_e1107_d_n9: f64 = (p.p87 * var_ibdi_dn9);
        let eq18_e1107_d_n10: f64 = (p.p87 * var_ibdi_dn10);
        let eq18_e1107_d_n11: f64 = (p.p87 * var_ibdi_dn11);
        let eq18_e1107_d_n14: f64 = (p.p87 * var_ibdi_dn14);
        (eq18_e1107, eq18_e1107_d_n0, eq18_e1107_d_n2, eq18_e1107_d_n4, eq18_e1107_d_n5, eq18_e1107_d_n6, eq18_e1107_d_n7, eq18_e1107_d_n8, eq18_e1107_d_n9, eq18_e1107_d_n10, eq18_e1107_d_n11, eq18_e1107_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1109;
        let eq18_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq18_node_derivatives: [f64; 11] = [eq18_e1109_d_n0, eq18_e1109_d_n2, eq18_e1109_d_n4, eq18_e1109_d_n5, eq18_e1109_d_n6, eq18_e1109_d_n7, eq18_e1109_d_n8, eq18_e1109_d_n9, eq18_e1109_d_n10, eq18_e1109_d_n11, eq18_e1109_d_n14];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
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
        var_flg_rd: f64,
        var_flg_rs: f64,
        var_guard2413: f64,
        var_guard2414: f64,
        var_igb: f64,
        var_igb_dn0: f64,
        var_igb_dn10: f64,
        var_igb_dn11: f64,
        var_igb_dn14: f64,
        var_igb_dn2: f64,
        var_igb_dn4: f64,
        var_igb_dn5: f64,
        var_igb_dn6: f64,
        var_igb_dn7: f64,
        var_igb_dn8: f64,
        var_igb_dn9: f64,
        var_igd: f64,
        var_igd_dn0: f64,
        var_igd_dn10: f64,
        var_igd_dn11: f64,
        var_igd_dn14: f64,
        var_igd_dn2: f64,
        var_igd_dn4: f64,
        var_igd_dn5: f64,
        var_igd_dn6: f64,
        var_igd_dn7: f64,
        var_igd_dn8: f64,
        var_igd_dn9: f64,
        var_igs: f64,
        var_igs_dn0: f64,
        var_igs_dn10: f64,
        var_igs_dn11: f64,
        var_igs_dn14: f64,
        var_igs_dn2: f64,
        var_igs_dn4: f64,
        var_igs_dn5: f64,
        var_igs_dn6: f64,
        var_igs_dn7: f64,
        var_igs_dn8: f64,
        var_igs_dn9: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn14: f64,
        var_qb_dn2: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qbdi: f64,
        var_qbdi_dn0: f64,
        var_qbdi_dn10: f64,
        var_qbdi_dn11: f64,
        var_qbdi_dn14: f64,
        var_qbdi_dn2: f64,
        var_qbdi_dn4: f64,
        var_qbdi_dn5: f64,
        var_qbdi_dn6: f64,
        var_qbdi_dn7: f64,
        var_qbdi_dn8: f64,
        var_qbdi_dn9: f64,
        var_qbext: f64,
        var_qbext_dn0: f64,
        var_qbext_dn10: f64,
        var_qbext_dn11: f64,
        var_qbext_dn14: f64,
        var_qbext_dn2: f64,
        var_qbext_dn4: f64,
        var_qbext_dn5: f64,
        var_qbext_dn6: f64,
        var_qbext_dn7: f64,
        var_qbext_dn8: f64,
        var_qbext_dn9: f64,
        var_qbsi: f64,
        var_qbsi_dn0: f64,
        var_qbsi_dn10: f64,
        var_qbsi_dn11: f64,
        var_qbsi_dn14: f64,
        var_qbsi_dn2: f64,
        var_qbsi_dn4: f64,
        var_qbsi_dn5: f64,
        var_qbsi_dn6: f64,
        var_qbsi_dn7: f64,
        var_qbsi_dn8: f64,
        var_qbsi_dn9: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn14: f64,
        var_qd_dn2: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn14: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn4: f64,
        var_qd_nqs_dn5: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qd_nqs_dn8: f64,
        var_qd_nqs_dn9: f64,
        var_qdext: f64,
        var_qdext_dn0: f64,
        var_qdext_dn10: f64,
        var_qdext_dn11: f64,
        var_qdext_dn14: f64,
        var_qdext_dn2: f64,
        var_qdext_dn4: f64,
        var_qdext_dn5: f64,
        var_qdext_dn6: f64,
        var_qdext_dn7: f64,
        var_qdext_dn8: f64,
        var_qdext_dn9: f64,
        var_qfd: f64,
        var_qfd_dn0: f64,
        var_qfd_dn2: f64,
        var_qfd_dn7: f64,
        var_qfs: f64,
        var_qfs_dn2: f64,
        var_qfs_dn7: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn14: f64,
        var_qg_dn2: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qg_nqs: f64,
        var_qg_nqs_dn12: f64,
        var_qg_nqs_dn13: f64,
        var_qgext: f64,
        var_qgext_dn0: f64,
        var_qgext_dn10: f64,
        var_qgext_dn11: f64,
        var_qgext_dn14: f64,
        var_qgext_dn2: f64,
        var_qgext_dn4: f64,
        var_qgext_dn5: f64,
        var_qgext_dn6: f64,
        var_qgext_dn7: f64,
        var_qgext_dn8: f64,
        var_qgext_dn9: f64,
        var_qs_nqs: f64,
        var_qs_nqs_dn0: f64,
        var_qs_nqs_dn10: f64,
        var_qs_nqs_dn11: f64,
        var_qs_nqs_dn12: f64,
        var_qs_nqs_dn14: f64,
        var_qs_nqs_dn2: f64,
        var_qs_nqs_dn4: f64,
        var_qs_nqs_dn5: f64,
        var_qs_nqs_dn6: f64,
        var_qs_nqs_dn7: f64,
        var_qs_nqs_dn8: f64,
        var_qs_nqs_dn9: f64,
        var_rdd: f64,
        var_rdd_dn0: f64,
        var_rdd_dn10: f64,
        var_rdd_dn11: f64,
        var_rdd_dn14: f64,
        var_rdd_dn2: f64,
        var_rdd_dn4: f64,
        var_rdd_dn5: f64,
        var_rdd_dn6: f64,
        var_rdd_dn7: f64,
        var_rdd_dn8: f64,
        var_rdd_dn9: f64,
        var_rsd: f64,
        var_rsd_dn0: f64,
        var_rsd_dn10: f64,
        var_rsd_dn11: f64,
        var_rsd_dn14: f64,
        var_rsd_dn2: f64,
        var_rsd_dn4: f64,
        var_rsd_dn5: f64,
        var_rsd_dn6: f64,
        var_rsd_dn7: f64,
        var_rsd_dn8: f64,
        var_rsd_dn9: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14,) = {
    if (var_guard2413 != 0.0) {
        let eq19_e1113: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, var_qbsi);
        let eq19_e1114: f64 = (p.p87 * eq19_e1113);
        let eq19_e1114_d_n0: f64 = (p.p87 * (var_qbsi_dn0 * ddt_scale));
        let eq19_e1114_d_n2: f64 = (p.p87 * (var_qbsi_dn2 * ddt_scale));
        let eq19_e1114_d_n4: f64 = (p.p87 * (var_qbsi_dn4 * ddt_scale));
        let eq19_e1114_d_n5: f64 = (p.p87 * (var_qbsi_dn5 * ddt_scale));
        let eq19_e1114_d_n6: f64 = (p.p87 * (var_qbsi_dn6 * ddt_scale));
        let eq19_e1114_d_n7: f64 = (p.p87 * (var_qbsi_dn7 * ddt_scale));
        let eq19_e1114_d_n8: f64 = (p.p87 * (var_qbsi_dn8 * ddt_scale));
        let eq19_e1114_d_n9: f64 = (p.p87 * (var_qbsi_dn9 * ddt_scale));
        let eq19_e1114_d_n10: f64 = (p.p87 * (var_qbsi_dn10 * ddt_scale));
        let eq19_e1114_d_n11: f64 = (p.p87 * (var_qbsi_dn11 * ddt_scale));
        let eq19_e1114_d_n14: f64 = (p.p87 * (var_qbsi_dn14 * ddt_scale));
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_value: f64 = eq19_e1116;
        let eq19_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq19_node_derivatives: [f64; 11] = [eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14,) = {
    if (var_guard2413 != 0.0) {
        let eq20_e1120: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, var_qbdi);
        let eq20_e1121: f64 = (p.p87 * eq20_e1120);
        let eq20_e1121_d_n0: f64 = (p.p87 * (var_qbdi_dn0 * ddt_scale));
        let eq20_e1121_d_n2: f64 = (p.p87 * (var_qbdi_dn2 * ddt_scale));
        let eq20_e1121_d_n4: f64 = (p.p87 * (var_qbdi_dn4 * ddt_scale));
        let eq20_e1121_d_n5: f64 = (p.p87 * (var_qbdi_dn5 * ddt_scale));
        let eq20_e1121_d_n6: f64 = (p.p87 * (var_qbdi_dn6 * ddt_scale));
        let eq20_e1121_d_n7: f64 = (p.p87 * (var_qbdi_dn7 * ddt_scale));
        let eq20_e1121_d_n8: f64 = (p.p87 * (var_qbdi_dn8 * ddt_scale));
        let eq20_e1121_d_n9: f64 = (p.p87 * (var_qbdi_dn9 * ddt_scale));
        let eq20_e1121_d_n10: f64 = (p.p87 * (var_qbdi_dn10 * ddt_scale));
        let eq20_e1121_d_n11: f64 = (p.p87 * (var_qbdi_dn11 * ddt_scale));
        let eq20_e1121_d_n14: f64 = (p.p87 * (var_qbdi_dn14 * ddt_scale));
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_value: f64 = eq20_e1123;
        let eq20_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq20_node_derivatives: [f64; 11] = [eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq21_e1129, eq21_e1129_d_n0, eq21_e1129_d_n2, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n14,) = {
    if (var_guard2414 != 0.0) {
        let eq21_e1127: f64 = (p.p87 * var_igs);
        let eq21_e1127_d_n0: f64 = (p.p87 * var_igs_dn0);
        let eq21_e1127_d_n2: f64 = (p.p87 * var_igs_dn2);
        let eq21_e1127_d_n4: f64 = (p.p87 * var_igs_dn4);
        let eq21_e1127_d_n5: f64 = (p.p87 * var_igs_dn5);
        let eq21_e1127_d_n6: f64 = (p.p87 * var_igs_dn6);
        let eq21_e1127_d_n7: f64 = (p.p87 * var_igs_dn7);
        let eq21_e1127_d_n8: f64 = (p.p87 * var_igs_dn8);
        let eq21_e1127_d_n9: f64 = (p.p87 * var_igs_dn9);
        let eq21_e1127_d_n10: f64 = (p.p87 * var_igs_dn10);
        let eq21_e1127_d_n11: f64 = (p.p87 * var_igs_dn11);
        let eq21_e1127_d_n14: f64 = (p.p87 * var_igs_dn14);
        (eq21_e1127, eq21_e1127_d_n0, eq21_e1127_d_n2, eq21_e1127_d_n4, eq21_e1127_d_n5, eq21_e1127_d_n6, eq21_e1127_d_n7, eq21_e1127_d_n8, eq21_e1127_d_n9, eq21_e1127_d_n10, eq21_e1127_d_n11, eq21_e1127_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1129;
        let eq21_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq21_node_derivatives: [f64; 11] = [eq21_e1129_d_n0, eq21_e1129_d_n2, eq21_e1129_d_n4, eq21_e1129_d_n5, eq21_e1129_d_n6, eq21_e1129_d_n7, eq21_e1129_d_n8, eq21_e1129_d_n9, eq21_e1129_d_n10, eq21_e1129_d_n11, eq21_e1129_d_n14];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1135, eq22_e1135_d_n0, eq22_e1135_d_n2, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n14,) = {
    if (var_guard2414 != 0.0) {
        let eq22_e1133: f64 = (p.p87 * var_igd);
        let eq22_e1133_d_n0: f64 = (p.p87 * var_igd_dn0);
        let eq22_e1133_d_n2: f64 = (p.p87 * var_igd_dn2);
        let eq22_e1133_d_n4: f64 = (p.p87 * var_igd_dn4);
        let eq22_e1133_d_n5: f64 = (p.p87 * var_igd_dn5);
        let eq22_e1133_d_n6: f64 = (p.p87 * var_igd_dn6);
        let eq22_e1133_d_n7: f64 = (p.p87 * var_igd_dn7);
        let eq22_e1133_d_n8: f64 = (p.p87 * var_igd_dn8);
        let eq22_e1133_d_n9: f64 = (p.p87 * var_igd_dn9);
        let eq22_e1133_d_n10: f64 = (p.p87 * var_igd_dn10);
        let eq22_e1133_d_n11: f64 = (p.p87 * var_igd_dn11);
        let eq22_e1133_d_n14: f64 = (p.p87 * var_igd_dn14);
        (eq22_e1133, eq22_e1133_d_n0, eq22_e1133_d_n2, eq22_e1133_d_n4, eq22_e1133_d_n5, eq22_e1133_d_n6, eq22_e1133_d_n7, eq22_e1133_d_n8, eq22_e1133_d_n9, eq22_e1133_d_n10, eq22_e1133_d_n11, eq22_e1133_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq22_value: f64 = eq22_e1135;
        let eq22_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq22_node_derivatives: [f64; 11] = [eq22_e1135_d_n0, eq22_e1135_d_n2, eq22_e1135_d_n4, eq22_e1135_d_n5, eq22_e1135_d_n6, eq22_e1135_d_n7, eq22_e1135_d_n8, eq22_e1135_d_n9, eq22_e1135_d_n10, eq22_e1135_d_n11, eq22_e1135_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1141, eq23_e1141_d_n0, eq23_e1141_d_n2, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n14,) = {
    if (var_guard2414 != 0.0) {
        let eq23_e1139: f64 = (p.p87 * var_igb);
        let eq23_e1139_d_n0: f64 = (p.p87 * var_igb_dn0);
        let eq23_e1139_d_n2: f64 = (p.p87 * var_igb_dn2);
        let eq23_e1139_d_n4: f64 = (p.p87 * var_igb_dn4);
        let eq23_e1139_d_n5: f64 = (p.p87 * var_igb_dn5);
        let eq23_e1139_d_n6: f64 = (p.p87 * var_igb_dn6);
        let eq23_e1139_d_n7: f64 = (p.p87 * var_igb_dn7);
        let eq23_e1139_d_n8: f64 = (p.p87 * var_igb_dn8);
        let eq23_e1139_d_n9: f64 = (p.p87 * var_igb_dn9);
        let eq23_e1139_d_n10: f64 = (p.p87 * var_igb_dn10);
        let eq23_e1139_d_n11: f64 = (p.p87 * var_igb_dn11);
        let eq23_e1139_d_n14: f64 = (p.p87 * var_igb_dn14);
        (eq23_e1139, eq23_e1139_d_n0, eq23_e1139_d_n2, eq23_e1139_d_n4, eq23_e1139_d_n5, eq23_e1139_d_n6, eq23_e1139_d_n7, eq23_e1139_d_n8, eq23_e1139_d_n9, eq23_e1139_d_n10, eq23_e1139_d_n11, eq23_e1139_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq23_value: f64 = eq23_e1141;
        let eq23_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq23_node_derivatives: [f64; 11] = [eq23_e1141_d_n0, eq23_e1141_d_n2, eq23_e1141_d_n4, eq23_e1141_d_n5, eq23_e1141_d_n6, eq23_e1141_d_n7, eq23_e1141_d_n8, eq23_e1141_d_n9, eq23_e1141_d_n10, eq23_e1141_d_n11, eq23_e1141_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e1147, eq24_e1147_d_n0, eq24_e1147_d_n2, eq24_e1147_d_n4, eq24_e1147_d_n5, eq24_e1147_d_n6, eq24_e1147_d_n7, eq24_e1147_d_n8, eq24_e1147_d_n9, eq24_e1147_d_n10, eq24_e1147_d_n11, eq24_e1147_d_n14,) = {
    if (var_flg_rd != 0.0) {
        let eq24_e1145: f64 = ((nv0 - nv6) / var_rdd);
        let eq24_e1145_d_n0: f64 = ((var_rdd - ((nv0 - nv6) * var_rdd_dn0)) / (var_rdd * var_rdd));
        let eq24_e1145_d_n2: f64 = (-(((nv0 - nv6) * var_rdd_dn2) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n4: f64 = (-(((nv0 - nv6) * var_rdd_dn4) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n5: f64 = (-(((nv0 - nv6) * var_rdd_dn5) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n6: f64 = (((-var_rdd) - ((nv0 - nv6) * var_rdd_dn6)) / (var_rdd * var_rdd));
        let eq24_e1145_d_n7: f64 = (-(((nv0 - nv6) * var_rdd_dn7) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n8: f64 = (-(((nv0 - nv6) * var_rdd_dn8) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n9: f64 = (-(((nv0 - nv6) * var_rdd_dn9) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n10: f64 = (-(((nv0 - nv6) * var_rdd_dn10) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n11: f64 = (-(((nv0 - nv6) * var_rdd_dn11) / (var_rdd * var_rdd)));
        let eq24_e1145_d_n14: f64 = (-(((nv0 - nv6) * var_rdd_dn14) / (var_rdd * var_rdd)));
        (eq24_e1145, eq24_e1145_d_n0, eq24_e1145_d_n2, eq24_e1145_d_n4, eq24_e1145_d_n5, eq24_e1145_d_n6, eq24_e1145_d_n7, eq24_e1145_d_n8, eq24_e1145_d_n9, eq24_e1145_d_n10, eq24_e1145_d_n11, eq24_e1145_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1147;
        let eq24_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq24_node_derivatives: [f64; 11] = [eq24_e1147_d_n0, eq24_e1147_d_n2, eq24_e1147_d_n4, eq24_e1147_d_n5, eq24_e1147_d_n6, eq24_e1147_d_n7, eq24_e1147_d_n8, eq24_e1147_d_n9, eq24_e1147_d_n10, eq24_e1147_d_n11, eq24_e1147_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1158, eq26_e1158_d_n0, eq26_e1158_d_n2, eq26_e1158_d_n4, eq26_e1158_d_n5, eq26_e1158_d_n6, eq26_e1158_d_n7, eq26_e1158_d_n8, eq26_e1158_d_n9, eq26_e1158_d_n10, eq26_e1158_d_n11, eq26_e1158_d_n14,) = {
    if (var_flg_rs != 0.0) {
        let eq26_e1156: f64 = ((nv8 - nv2) / var_rsd);
        let eq26_e1156_d_n0: f64 = (-(((nv8 - nv2) * var_rsd_dn0) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n2: f64 = (((-var_rsd) - ((nv8 - nv2) * var_rsd_dn2)) / (var_rsd * var_rsd));
        let eq26_e1156_d_n4: f64 = (-(((nv8 - nv2) * var_rsd_dn4) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n5: f64 = (-(((nv8 - nv2) * var_rsd_dn5) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n6: f64 = (-(((nv8 - nv2) * var_rsd_dn6) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n7: f64 = (-(((nv8 - nv2) * var_rsd_dn7) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n8: f64 = ((var_rsd - ((nv8 - nv2) * var_rsd_dn8)) / (var_rsd * var_rsd));
        let eq26_e1156_d_n9: f64 = (-(((nv8 - nv2) * var_rsd_dn9) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n10: f64 = (-(((nv8 - nv2) * var_rsd_dn10) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n11: f64 = (-(((nv8 - nv2) * var_rsd_dn11) / (var_rsd * var_rsd)));
        let eq26_e1156_d_n14: f64 = (-(((nv8 - nv2) * var_rsd_dn14) / (var_rsd * var_rsd)));
        (eq26_e1156, eq26_e1156_d_n0, eq26_e1156_d_n2, eq26_e1156_d_n4, eq26_e1156_d_n5, eq26_e1156_d_n6, eq26_e1156_d_n7, eq26_e1156_d_n8, eq26_e1156_d_n9, eq26_e1156_d_n10, eq26_e1156_d_n11, eq26_e1156_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1158;
        let eq26_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq26_node_derivatives: [f64; 11] = [eq26_e1158_d_n0, eq26_e1158_d_n2, eq26_e1158_d_n4, eq26_e1158_d_n5, eq26_e1158_d_n6, eq26_e1158_d_n7, eq26_e1158_d_n8, eq26_e1158_d_n9, eq26_e1158_d_n10, eq26_e1158_d_n11, eq26_e1158_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(2),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq28_e1167: f64 = (var_qg + var_qg_nqs);
        let eq28_e1168: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq28_e1167);
        let eq28_e1169: f64 = (p.p87 * eq28_e1168);
        let eq28_e1169_d_n0: f64 = (p.p87 * (var_qg_dn0 * ddt_scale));
        let eq28_e1169_d_n2: f64 = (p.p87 * (var_qg_dn2 * ddt_scale));
        let eq28_e1169_d_n4: f64 = (p.p87 * (var_qg_dn4 * ddt_scale));
        let eq28_e1169_d_n5: f64 = (p.p87 * (var_qg_dn5 * ddt_scale));
        let eq28_e1169_d_n6: f64 = (p.p87 * (var_qg_dn6 * ddt_scale));
        let eq28_e1169_d_n7: f64 = (p.p87 * (var_qg_dn7 * ddt_scale));
        let eq28_e1169_d_n8: f64 = (p.p87 * (var_qg_dn8 * ddt_scale));
        let eq28_e1169_d_n9: f64 = (p.p87 * (var_qg_dn9 * ddt_scale));
        let eq28_e1169_d_n10: f64 = (p.p87 * (var_qg_dn10 * ddt_scale));
        let eq28_e1169_d_n11: f64 = (p.p87 * (var_qg_dn11 * ddt_scale));
        let eq28_e1169_d_n12: f64 = (p.p87 * (var_qg_nqs_dn12 * ddt_scale));
        let eq28_e1169_d_n13: f64 = (p.p87 * (var_qg_nqs_dn13 * ddt_scale));
        let eq28_e1169_d_n14: f64 = (p.p87 * (var_qg_dn14 * ddt_scale));
        let eq28_value: f64 = eq28_e1169;
        let eq28_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq28_node_derivatives: [f64; 13] = [eq28_e1169_d_n0, eq28_e1169_d_n2, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (var_qd + var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (var_qd_dn0 + var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (var_qd_dn2 + var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (var_qd_dn4 + var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (var_qd_dn5 + var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (var_qd_dn6 + var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (var_qd_dn7 + var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (var_qd_dn8 + var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (var_qd_dn9 + var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (var_qd_dn10 + var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (var_qd_dn11 + var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (var_qd_dn14 + var_qd_nqs_dn14);
        let eq29_e1174: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq29_e1173);
        let eq29_e1175: f64 = (p.p87 * eq29_e1174);
        let eq29_e1175_d_n0: f64 = (p.p87 * (eq29_e1173_d_n0 * ddt_scale));
        let eq29_e1175_d_n2: f64 = (p.p87 * (eq29_e1173_d_n2 * ddt_scale));
        let eq29_e1175_d_n4: f64 = (p.p87 * (eq29_e1173_d_n4 * ddt_scale));
        let eq29_e1175_d_n5: f64 = (p.p87 * (eq29_e1173_d_n5 * ddt_scale));
        let eq29_e1175_d_n6: f64 = (p.p87 * (eq29_e1173_d_n6 * ddt_scale));
        let eq29_e1175_d_n7: f64 = (p.p87 * (eq29_e1173_d_n7 * ddt_scale));
        let eq29_e1175_d_n8: f64 = (p.p87 * (eq29_e1173_d_n8 * ddt_scale));
        let eq29_e1175_d_n9: f64 = (p.p87 * (eq29_e1173_d_n9 * ddt_scale));
        let eq29_e1175_d_n10: f64 = (p.p87 * (eq29_e1173_d_n10 * ddt_scale));
        let eq29_e1175_d_n11: f64 = (p.p87 * (eq29_e1173_d_n11 * ddt_scale));
        let eq29_e1175_d_n12: f64 = (p.p87 * (var_qd_nqs_dn12 * ddt_scale));
        let eq29_e1175_d_n14: f64 = (p.p87 * (eq29_e1173_d_n14 * ddt_scale));
        let eq29_value: f64 = eq29_e1175;
        let eq29_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];
        let eq29_node_derivatives: [f64; 12] = [eq29_e1175_d_n0, eq29_e1175_d_n2, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, eq29_e1175_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (var_qg_nqs + var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (var_qg_nqs_dn12 + var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (var_qd_nqs_dn0 + var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (var_qd_nqs_dn2 + var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (var_qd_nqs_dn4 + var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (var_qd_nqs_dn5 + var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (var_qd_nqs_dn6 + var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (var_qd_nqs_dn7 + var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (var_qd_nqs_dn8 + var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (var_qd_nqs_dn9 + var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (var_qd_nqs_dn10 + var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (var_qd_nqs_dn11 + var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (var_qd_nqs_dn14 + var_qs_nqs_dn14);
        let eq30_e1183: f64 = (var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq30_e1183);
        let eq30_e1185: f64 = (p.p87 * eq30_e1184);
        let eq30_e1185_d_n0: f64 = (p.p87 * (eq30_e1183_d_n0 * ddt_scale));
        let eq30_e1185_d_n2: f64 = (p.p87 * (eq30_e1183_d_n2 * ddt_scale));
        let eq30_e1185_d_n4: f64 = (p.p87 * (eq30_e1183_d_n4 * ddt_scale));
        let eq30_e1185_d_n5: f64 = (p.p87 * (eq30_e1183_d_n5 * ddt_scale));
        let eq30_e1185_d_n6: f64 = (p.p87 * (eq30_e1183_d_n6 * ddt_scale));
        let eq30_e1185_d_n7: f64 = (p.p87 * (eq30_e1183_d_n7 * ddt_scale));
        let eq30_e1185_d_n8: f64 = (p.p87 * (eq30_e1183_d_n8 * ddt_scale));
        let eq30_e1185_d_n9: f64 = (p.p87 * (eq30_e1183_d_n9 * ddt_scale));
        let eq30_e1185_d_n10: f64 = (p.p87 * (eq30_e1183_d_n10 * ddt_scale));
        let eq30_e1185_d_n11: f64 = (p.p87 * (eq30_e1183_d_n11 * ddt_scale));
        let eq30_e1185_d_n12: f64 = (p.p87 * ((-eq30_e1182_d_n12) * ddt_scale));
        let eq30_e1185_d_n13: f64 = (p.p87 * ((-var_qg_nqs_dn13) * ddt_scale));
        let eq30_e1185_d_n14: f64 = (p.p87 * (eq30_e1183_d_n14 * ddt_scale));
        let eq30_value: f64 = eq30_e1185;
        let eq30_node_derivative_indices: [usize; 13] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq30_node_derivatives: [f64; 13] = [eq30_e1185_d_n0, eq30_e1185_d_n2, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(8),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, var_qgext);
        let eq31_e1189: f64 = (p.p87 * eq31_e1188);
        let eq31_e1189_d_n0: f64 = (p.p87 * (var_qgext_dn0 * ddt_scale));
        let eq31_e1189_d_n2: f64 = (p.p87 * (var_qgext_dn2 * ddt_scale));
        let eq31_e1189_d_n4: f64 = (p.p87 * (var_qgext_dn4 * ddt_scale));
        let eq31_e1189_d_n5: f64 = (p.p87 * (var_qgext_dn5 * ddt_scale));
        let eq31_e1189_d_n6: f64 = (p.p87 * (var_qgext_dn6 * ddt_scale));
        let eq31_e1189_d_n7: f64 = (p.p87 * (var_qgext_dn7 * ddt_scale));
        let eq31_e1189_d_n8: f64 = (p.p87 * (var_qgext_dn8 * ddt_scale));
        let eq31_e1189_d_n9: f64 = (p.p87 * (var_qgext_dn9 * ddt_scale));
        let eq31_e1189_d_n10: f64 = (p.p87 * (var_qgext_dn10 * ddt_scale));
        let eq31_e1189_d_n11: f64 = (p.p87 * (var_qgext_dn11 * ddt_scale));
        let eq31_e1189_d_n14: f64 = (p.p87 * (var_qgext_dn14 * ddt_scale));
        let eq31_value: f64 = eq31_e1189;
        let eq31_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq31_node_derivatives: [f64; 11] = [eq31_e1189_d_n0, eq31_e1189_d_n2, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, eq31_e1189_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(2),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, var_qdext);
        let eq32_e1193: f64 = (p.p87 * eq32_e1192);
        let eq32_e1193_d_n0: f64 = (p.p87 * (var_qdext_dn0 * ddt_scale));
        let eq32_e1193_d_n2: f64 = (p.p87 * (var_qdext_dn2 * ddt_scale));
        let eq32_e1193_d_n4: f64 = (p.p87 * (var_qdext_dn4 * ddt_scale));
        let eq32_e1193_d_n5: f64 = (p.p87 * (var_qdext_dn5 * ddt_scale));
        let eq32_e1193_d_n6: f64 = (p.p87 * (var_qdext_dn6 * ddt_scale));
        let eq32_e1193_d_n7: f64 = (p.p87 * (var_qdext_dn7 * ddt_scale));
        let eq32_e1193_d_n8: f64 = (p.p87 * (var_qdext_dn8 * ddt_scale));
        let eq32_e1193_d_n9: f64 = (p.p87 * (var_qdext_dn9 * ddt_scale));
        let eq32_e1193_d_n10: f64 = (p.p87 * (var_qdext_dn10 * ddt_scale));
        let eq32_e1193_d_n11: f64 = (p.p87 * (var_qdext_dn11 * ddt_scale));
        let eq32_e1193_d_n14: f64 = (p.p87 * (var_qdext_dn14 * ddt_scale));
        let eq32_value: f64 = eq32_e1193;
        let eq32_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq32_node_derivatives: [f64; 11] = [eq32_e1193_d_n0, eq32_e1193_d_n2, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, eq32_e1193_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(2),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, var_qbext);
        let eq33_e1197: f64 = (p.p87 * eq33_e1196);
        let eq33_e1197_d_n0: f64 = (p.p87 * (var_qbext_dn0 * ddt_scale));
        let eq33_e1197_d_n2: f64 = (p.p87 * (var_qbext_dn2 * ddt_scale));
        let eq33_e1197_d_n4: f64 = (p.p87 * (var_qbext_dn4 * ddt_scale));
        let eq33_e1197_d_n5: f64 = (p.p87 * (var_qbext_dn5 * ddt_scale));
        let eq33_e1197_d_n6: f64 = (p.p87 * (var_qbext_dn6 * ddt_scale));
        let eq33_e1197_d_n7: f64 = (p.p87 * (var_qbext_dn7 * ddt_scale));
        let eq33_e1197_d_n8: f64 = (p.p87 * (var_qbext_dn8 * ddt_scale));
        let eq33_e1197_d_n9: f64 = (p.p87 * (var_qbext_dn9 * ddt_scale));
        let eq33_e1197_d_n10: f64 = (p.p87 * (var_qbext_dn10 * ddt_scale));
        let eq33_e1197_d_n11: f64 = (p.p87 * (var_qbext_dn11 * ddt_scale));
        let eq33_e1197_d_n14: f64 = (p.p87 * (var_qbext_dn14 * ddt_scale));
        let eq33_value: f64 = eq33_e1197;
        let eq33_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq33_node_derivatives: [f64; 11] = [eq33_e1197_d_n0, eq33_e1197_d_n2, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, eq33_e1197_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(2),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, var_qfd);
        let eq34_e1202: f64 = (eq34_e1199 * eq34_e1201);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * (var_qfd_dn0 * ddt_scale));
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * (var_qfd_dn2 * ddt_scale));
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * (var_qfd_dn7 * ddt_scale));
        let eq34_value: f64 = eq34_e1202;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(0),
            multiplicity * (eq34_value),
            0,
            multiplicity * (eq34_e1202_d_n0),
            2,
            multiplicity * (eq34_e1202_d_n2),
            7,
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, var_qfs);
        let eq35_e1207: f64 = (eq35_e1204 * eq35_e1206);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * (var_qfs_dn2 * ddt_scale));
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * (var_qfs_dn7 * ddt_scale));
        let eq35_value: f64 = eq35_e1207;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(2),
            multiplicity * (eq35_value),
            2,
            multiplicity * (eq35_e1207_d_n2),
            7,
            multiplicity * (eq35_e1207_d_n7),
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
        var_ci: f64,
        var_ci_dn0: f64,
        var_ci_dn10: f64,
        var_ci_dn11: f64,
        var_ci_dn14: f64,
        var_ci_dn2: f64,
        var_ci_dn4: f64,
        var_ci_dn5: f64,
        var_ci_dn6: f64,
        var_ci_dn7: f64,
        var_ci_dn8: f64,
        var_ci_dn9: f64,
        var_cqb: f64,
        var_cqi: f64,
        var_guard2417: f64,
        var_ibd_nqs: f64,
        var_ibd_nqs_dn0: f64,
        var_ibd_nqs_dn10: f64,
        var_ibd_nqs_dn11: f64,
        var_ibd_nqs_dn14: f64,
        var_ibd_nqs_dn2: f64,
        var_ibd_nqs_dn4: f64,
        var_ibd_nqs_dn5: f64,
        var_ibd_nqs_dn6: f64,
        var_ibd_nqs_dn7: f64,
        var_ibd_nqs_dn8: f64,
        var_ibd_nqs_dn9: f64,
        var_iqb_nqs: f64,
        var_iqb_nqs_dn0: f64,
        var_iqb_nqs_dn10: f64,
        var_iqb_nqs_dn11: f64,
        var_iqb_nqs_dn13: f64,
        var_iqb_nqs_dn14: f64,
        var_iqb_nqs_dn2: f64,
        var_iqb_nqs_dn4: f64,
        var_iqb_nqs_dn5: f64,
        var_iqb_nqs_dn6: f64,
        var_iqb_nqs_dn7: f64,
        var_iqb_nqs_dn8: f64,
        var_iqb_nqs_dn9: f64,
        var_iqi_nqs: f64,
        var_iqi_nqs_dn0: f64,
        var_iqi_nqs_dn10: f64,
        var_iqi_nqs_dn11: f64,
        var_iqi_nqs_dn12: f64,
        var_iqi_nqs_dn14: f64,
        var_iqi_nqs_dn2: f64,
        var_iqi_nqs_dn4: f64,
        var_iqi_nqs_dn5: f64,
        var_iqi_nqs_dn6: f64,
        var_iqi_nqs_dn7: f64,
        var_iqi_nqs_dn8: f64,
        var_iqi_nqs_dn9: f64,
        var_itemp: f64,
        var_itemp_dn0: f64,
        var_itemp_dn10: f64,
        var_itemp_dn11: f64,
        var_itemp_dn14: f64,
        var_itemp_dn2: f64,
        var_itemp_dn4: f64,
        var_itemp_dn5: f64,
        var_itemp_dn6: f64,
        var_itemp_dn7: f64,
        var_itemp_dn8: f64,
        var_itemp_dn9: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn14: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn4: f64,
        var_sigrat_d_dn5: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_d_dn8: f64,
        var_sigrat_d_dn9: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn14: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn4: f64,
        var_sigrat_s_dn5: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
        var_sigrat_s_dn8: f64,
        var_sigrat_s_dn9: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let eq40_e1233: f64 = (var_ci * (nv15 - 0.0));
        let eq40_e1233_d_n0: f64 = (var_ci_dn0 * (nv15 - 0.0));
        let eq40_e1233_d_n2: f64 = (var_ci_dn2 * (nv15 - 0.0));
        let eq40_e1233_d_n4: f64 = (var_ci_dn4 * (nv15 - 0.0));
        let eq40_e1233_d_n5: f64 = (var_ci_dn5 * (nv15 - 0.0));
        let eq40_e1233_d_n6: f64 = (var_ci_dn6 * (nv15 - 0.0));
        let eq40_e1233_d_n7: f64 = (var_ci_dn7 * (nv15 - 0.0));
        let eq40_e1233_d_n8: f64 = (var_ci_dn8 * (nv15 - 0.0));
        let eq40_e1233_d_n9: f64 = (var_ci_dn9 * (nv15 - 0.0));
        let eq40_e1233_d_n10: f64 = (var_ci_dn10 * (nv15 - 0.0));
        let eq40_e1233_d_n11: f64 = (var_ci_dn11 * (nv15 - 0.0));
        let eq40_e1233_d_n14: f64 = (var_ci_dn14 * (nv15 - 0.0));
        let eq40_value: f64 = eq40_e1233;
        let eq40_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq40_node_derivatives: [f64; 12] = [eq40_e1233_d_n0, eq40_e1233_d_n2, eq40_e1233_d_n4, eq40_e1233_d_n5, eq40_e1233_d_n6, eq40_e1233_d_n7, eq40_e1233_d_n8, eq40_e1233_d_n9, eq40_e1233_d_n10, eq40_e1233_d_n11, eq40_e1233_d_n14, var_ci];
        let eq40_branch_derivative_indices: [usize; 0] = [];
        let eq40_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq40_value),
            &eq40_node_derivative_indices,
            &eq40_node_derivatives,
            &eq40_branch_derivative_indices,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * var_sigrat_s_dn14);
        let eq41_e1237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq41_e1236);
        let eq41_value: f64 = eq41_e1237;
        let eq41_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq41_node_derivatives: [f64; 12] = [(eq41_e1236_d_n0 * ddt_scale), (eq41_e1236_d_n2 * ddt_scale), (eq41_e1236_d_n4 * ddt_scale), (eq41_e1236_d_n5 * ddt_scale), (eq41_e1236_d_n6 * ddt_scale), (eq41_e1236_d_n7 * ddt_scale), (eq41_e1236_d_n8 * ddt_scale), (eq41_e1236_d_n9 * ddt_scale), (eq41_e1236_d_n10 * ddt_scale), (eq41_e1236_d_n11 * ddt_scale), (eq41_e1236_d_n14 * ddt_scale), (var_sigrat_s * ddt_scale)];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq41_value),
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * var_sigrat_d_dn14);
        let eq42_e1241: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 16, eq42_e1240);
        let eq42_value: f64 = eq42_e1241;
        let eq42_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14, 15];
        let eq42_node_derivatives: [f64; 12] = [(eq42_e1240_d_n0 * ddt_scale), (eq42_e1240_d_n2 * ddt_scale), (eq42_e1240_d_n4 * ddt_scale), (eq42_e1240_d_n5 * ddt_scale), (eq42_e1240_d_n6 * ddt_scale), (eq42_e1240_d_n7 * ddt_scale), (eq42_e1240_d_n8 * ddt_scale), (eq42_e1240_d_n9 * ddt_scale), (eq42_e1240_d_n10 * ddt_scale), (eq42_e1240_d_n11 * ddt_scale), (eq42_e1240_d_n14 * ddt_scale), (var_sigrat_d * ddt_scale)];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq57_e1336, eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14,) = {
    if (var_guard2417 != 0.0) {
        let eq57_e1334: f64 = (-var_itemp);
        (eq57_e1334, (-var_itemp_dn0), (-var_itemp_dn2), (-var_itemp_dn4), (-var_itemp_dn5), (-var_itemp_dn6), (-var_itemp_dn7), (-var_itemp_dn8), (-var_itemp_dn9), (-var_itemp_dn10), (-var_itemp_dn11), (-var_itemp_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq57_value: f64 = eq57_e1336;
        let eq57_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq57_node_derivatives: [f64; 11] = [eq57_e1336_d_n0, eq57_e1336_d_n2, eq57_e1336_d_n4, eq57_e1336_d_n5, eq57_e1336_d_n6, eq57_e1336_d_n7, eq57_e1336_d_n8, eq57_e1336_d_n9, eq57_e1336_d_n10, eq57_e1336_d_n11, eq57_e1336_d_n14];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            None,
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let (eq60_e1351, eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n14,) = {
    if (p.p28 != 0.0) {
        (var_iqi_nqs, var_iqi_nqs_dn0, var_iqi_nqs_dn2, var_iqi_nqs_dn4, var_iqi_nqs_dn5, var_iqi_nqs_dn6, var_iqi_nqs_dn7, var_iqi_nqs_dn8, var_iqi_nqs_dn9, var_iqi_nqs_dn10, var_iqi_nqs_dn11, var_iqi_nqs_dn12, var_iqi_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e1351;
        let eq60_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14];
        let eq60_node_derivatives: [f64; 12] = [eq60_e1351_d_n0, eq60_e1351_d_n2, eq60_e1351_d_n4, eq60_e1351_d_n5, eq60_e1351_d_n6, eq60_e1351_d_n7, eq60_e1351_d_n8, eq60_e1351_d_n9, eq60_e1351_d_n10, eq60_e1351_d_n11, eq60_e1351_d_n12, eq60_e1351_d_n14];
        let eq60_branch_derivative_indices: [usize; 0] = [];
        let eq60_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            None,
            multiplicity * (eq60_value),
            &eq60_node_derivative_indices,
            &eq60_node_derivatives,
            &eq60_branch_derivative_indices,
            &eq60_branch_derivatives,
            multiplicity,
        );
        let (eq61_e1355, eq61_e1355_d_n0, eq61_e1355_d_n2, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n13, eq61_e1355_d_n14,) = {
    if (p.p28 != 0.0) {
        (var_iqb_nqs, var_iqb_nqs_dn0, var_iqb_nqs_dn2, var_iqb_nqs_dn4, var_iqb_nqs_dn5, var_iqb_nqs_dn6, var_iqb_nqs_dn7, var_iqb_nqs_dn8, var_iqb_nqs_dn9, var_iqb_nqs_dn10, var_iqb_nqs_dn11, var_iqb_nqs_dn13, var_iqb_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq61_value: f64 = eq61_e1355;
        let eq61_node_derivative_indices: [usize; 12] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14];
        let eq61_node_derivatives: [f64; 12] = [eq61_e1355_d_n0, eq61_e1355_d_n2, eq61_e1355_d_n4, eq61_e1355_d_n5, eq61_e1355_d_n6, eq61_e1355_d_n7, eq61_e1355_d_n8, eq61_e1355_d_n9, eq61_e1355_d_n10, eq61_e1355_d_n11, eq61_e1355_d_n13, eq61_e1355_d_n14];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            None,
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (var_cqi * (nv12 - 0.0));
        let eq62_e1360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 18, eq62_e1359);
        (eq62_e1360, (var_cqi * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq62_value: f64 = eq62_e1362;
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (eq62_value),
            12,
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (var_cqb * (nv13 - 0.0));
        let eq63_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 19, eq63_e1366);
        (eq63_e1367, (var_cqb * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq63_value: f64 = eq63_e1369;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq63_value),
            13,
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq66_e1383, eq66_e1383_d_n0, eq66_e1383_d_n2, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n14,) = {
    if (p.p29 != 0.0) {
        (var_ibd_nqs, var_ibd_nqs_dn0, var_ibd_nqs_dn2, var_ibd_nqs_dn4, var_ibd_nqs_dn5, var_ibd_nqs_dn6, var_ibd_nqs_dn7, var_ibd_nqs_dn8, var_ibd_nqs_dn9, var_ibd_nqs_dn10, var_ibd_nqs_dn11, var_ibd_nqs_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq66_value: f64 = eq66_e1383;
        let eq66_node_derivative_indices: [usize; 11] = [0, 2, 4, 5, 6, 7, 8, 9, 10, 11, 14];
        let eq66_node_derivatives: [f64; 11] = [eq66_e1383_d_n0, eq66_e1383_d_n2, eq66_e1383_d_n4, eq66_e1383_d_n5, eq66_e1383_d_n6, eq66_e1383_d_n7, eq66_e1383_d_n8, eq66_e1383_d_n9, eq66_e1383_d_n10, eq66_e1383_d_n11, eq66_e1383_d_n14];
        let eq66_branch_derivative_indices: [usize; 0] = [];
        let eq66_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(14),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivative_indices,
            &eq66_node_derivatives,
            &eq66_branch_derivative_indices,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let (eq67_e1388, eq67_e1388_d_n14,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 20, (nv14 - 0.0));
        (eq67_e1386, ddt_scale,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e1388;
        stamper.stamp_current_node1_local(
            Some(14),
            None,
            multiplicity * (eq67_value),
            14,
            multiplicity * (eq67_e1388_d_n14),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cqb: f64,
        var_cqi: f64,
        var_guard2313: f64,
        var_guard2314: f64,
        var_guard2413: f64,
        var_inqs0_a: f64,
        var_inqs0_a_dn0: f64,
        var_inqs0_a_dn10: f64,
        var_inqs0_a_dn11: f64,
        var_inqs0_a_dn14: f64,
        var_inqs0_a_dn16: f64,
        var_inqs0_a_dn2: f64,
        var_inqs0_a_dn4: f64,
        var_inqs0_a_dn5: f64,
        var_inqs0_a_dn6: f64,
        var_inqs0_a_dn7: f64,
        var_inqs0_a_dn8: f64,
        var_inqs0_a_dn9: f64,
        var_inqs0_k: f64,
        var_inqs0_k_dn0: f64,
        var_inqs0_k_dn10: f64,
        var_inqs0_k_dn11: f64,
        var_inqs0_k_dn14: f64,
        var_inqs0_k_dn17: f64,
        var_inqs0_k_dn2: f64,
        var_inqs0_k_dn4: f64,
        var_inqs0_k_dn5: f64,
        var_inqs0_k_dn6: f64,
        var_inqs0_k_dn7: f64,
        var_inqs0_k_dn8: f64,
        var_inqs0_k_dn9: f64,
        var_iwnqs0_a: f64,
        var_iwnqs0_a_dn0: f64,
        var_iwnqs0_a_dn10: f64,
        var_iwnqs0_a_dn11: f64,
        var_iwnqs0_a_dn14: f64,
        var_iwnqs0_a_dn18: f64,
        var_iwnqs0_a_dn2: f64,
        var_iwnqs0_a_dn4: f64,
        var_iwnqs0_a_dn5: f64,
        var_iwnqs0_a_dn6: f64,
        var_iwnqs0_a_dn7: f64,
        var_iwnqs0_a_dn8: f64,
        var_iwnqs0_a_dn9: f64,
        var_q_nqs_a: f64,
        var_q_nqs_a_dn16: f64,
        var_q_nqs_k: f64,
        var_q_nqs_k_dn17: f64,
        var_qb: f64,
        var_qb_dn0: f64,
        var_qb_dn10: f64,
        var_qb_dn11: f64,
        var_qb_dn14: f64,
        var_qb_dn2: f64,
        var_qb_dn4: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qbd: f64,
        var_qbd_dn0: f64,
        var_qbd_dn10: f64,
        var_qbd_dn11: f64,
        var_qbd_dn14: f64,
        var_qbd_dn16: f64,
        var_qbd_dn17: f64,
        var_qbd_dn18: f64,
        var_qbd_dn2: f64,
        var_qbd_dn4: f64,
        var_qbd_dn5: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_qbd_dn8: f64,
        var_qbd_dn9: f64,
        var_qbdi: f64,
        var_qbdi_dn0: f64,
        var_qbdi_dn10: f64,
        var_qbdi_dn11: f64,
        var_qbdi_dn14: f64,
        var_qbdi_dn2: f64,
        var_qbdi_dn4: f64,
        var_qbdi_dn5: f64,
        var_qbdi_dn6: f64,
        var_qbdi_dn7: f64,
        var_qbdi_dn8: f64,
        var_qbdi_dn9: f64,
        var_qbext: f64,
        var_qbext_dn0: f64,
        var_qbext_dn10: f64,
        var_qbext_dn11: f64,
        var_qbext_dn14: f64,
        var_qbext_dn2: f64,
        var_qbext_dn4: f64,
        var_qbext_dn5: f64,
        var_qbext_dn6: f64,
        var_qbext_dn7: f64,
        var_qbext_dn8: f64,
        var_qbext_dn9: f64,
        var_qbs: f64,
        var_qbs_dn0: f64,
        var_qbs_dn10: f64,
        var_qbs_dn11: f64,
        var_qbs_dn14: f64,
        var_qbs_dn2: f64,
        var_qbs_dn4: f64,
        var_qbs_dn5: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbs_dn8: f64,
        var_qbs_dn9: f64,
        var_qbsi: f64,
        var_qbsi_dn0: f64,
        var_qbsi_dn10: f64,
        var_qbsi_dn11: f64,
        var_qbsi_dn14: f64,
        var_qbsi_dn2: f64,
        var_qbsi_dn4: f64,
        var_qbsi_dn5: f64,
        var_qbsi_dn6: f64,
        var_qbsi_dn7: f64,
        var_qbsi_dn8: f64,
        var_qbsi_dn9: f64,
        var_qd: f64,
        var_qd_dn0: f64,
        var_qd_dn10: f64,
        var_qd_dn11: f64,
        var_qd_dn14: f64,
        var_qd_dn2: f64,
        var_qd_dn4: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qd_nqs: f64,
        var_qd_nqs_dn0: f64,
        var_qd_nqs_dn10: f64,
        var_qd_nqs_dn11: f64,
        var_qd_nqs_dn12: f64,
        var_qd_nqs_dn14: f64,
        var_qd_nqs_dn2: f64,
        var_qd_nqs_dn4: f64,
        var_qd_nqs_dn5: f64,
        var_qd_nqs_dn6: f64,
        var_qd_nqs_dn7: f64,
        var_qd_nqs_dn8: f64,
        var_qd_nqs_dn9: f64,
        var_qdext: f64,
        var_qdext_dn0: f64,
        var_qdext_dn10: f64,
        var_qdext_dn11: f64,
        var_qdext_dn14: f64,
        var_qdext_dn2: f64,
        var_qdext_dn4: f64,
        var_qdext_dn5: f64,
        var_qdext_dn6: f64,
        var_qdext_dn7: f64,
        var_qdext_dn8: f64,
        var_qdext_dn9: f64,
        var_qfd: f64,
        var_qfd_dn0: f64,
        var_qfd_dn2: f64,
        var_qfd_dn7: f64,
        var_qfs: f64,
        var_qfs_dn2: f64,
        var_qfs_dn7: f64,
        var_qg: f64,
        var_qg_dn0: f64,
        var_qg_dn10: f64,
        var_qg_dn11: f64,
        var_qg_dn14: f64,
        var_qg_dn2: f64,
        var_qg_dn4: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qg_nqs: f64,
        var_qg_nqs_dn12: f64,
        var_qg_nqs_dn13: f64,
        var_qgext: f64,
        var_qgext_dn0: f64,
        var_qgext_dn10: f64,
        var_qgext_dn11: f64,
        var_qgext_dn14: f64,
        var_qgext_dn2: f64,
        var_qgext_dn4: f64,
        var_qgext_dn5: f64,
        var_qgext_dn6: f64,
        var_qgext_dn7: f64,
        var_qgext_dn8: f64,
        var_qgext_dn9: f64,
        var_qs_nqs: f64,
        var_qs_nqs_dn0: f64,
        var_qs_nqs_dn10: f64,
        var_qs_nqs_dn11: f64,
        var_qs_nqs_dn12: f64,
        var_qs_nqs_dn14: f64,
        var_qs_nqs_dn2: f64,
        var_qs_nqs_dn4: f64,
        var_qs_nqs_dn5: f64,
        var_qs_nqs_dn6: f64,
        var_qs_nqs_dn7: f64,
        var_qs_nqs_dn8: f64,
        var_qs_nqs_dn9: f64,
        var_sigrat_d: f64,
        var_sigrat_d_dn0: f64,
        var_sigrat_d_dn10: f64,
        var_sigrat_d_dn11: f64,
        var_sigrat_d_dn14: f64,
        var_sigrat_d_dn2: f64,
        var_sigrat_d_dn4: f64,
        var_sigrat_d_dn5: f64,
        var_sigrat_d_dn6: f64,
        var_sigrat_d_dn7: f64,
        var_sigrat_d_dn8: f64,
        var_sigrat_d_dn9: f64,
        var_sigrat_s: f64,
        var_sigrat_s_dn0: f64,
        var_sigrat_s_dn10: f64,
        var_sigrat_s_dn11: f64,
        var_sigrat_s_dn14: f64,
        var_sigrat_s_dn2: f64,
        var_sigrat_s_dn4: f64,
        var_sigrat_s_dn5: f64,
        var_sigrat_s_dn6: f64,
        var_sigrat_s_dn7: f64,
        var_sigrat_s_dn8: f64,
        var_sigrat_s_dn9: f64,
        var_w_nqs_a: f64,
        var_w_nqs_a_dn18: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq1_e1022, eq1_e1022_d_n0, eq1_e1022_d_n2, eq1_e1022_d_n4, eq1_e1022_d_n5, eq1_e1022_d_n6, eq1_e1022_d_n7, eq1_e1022_d_n8, eq1_e1022_d_n9, eq1_e1022_d_n10, eq1_e1022_d_n11, eq1_e1022_d_n14, eq1_e1022_d_n16, eq1_e1022_q, eq1_e1022_q_d_n16,) = {
    if (var_guard2313 != 0.0) {
        let eq1_e1019_q: f64 = var_q_nqs_a;
        let eq1_e1020: f64 = (var_inqs0_a + var_q_nqs_a);
        let eq1_e1020_d_n16: f64 = (var_inqs0_a_dn16 + var_q_nqs_a_dn16);
        let eq1_e1020_q: f64 = eq1_e1019_q;
        (eq1_e1020, var_inqs0_a_dn0, var_inqs0_a_dn2, var_inqs0_a_dn4, var_inqs0_a_dn5, var_inqs0_a_dn6, var_inqs0_a_dn7, var_inqs0_a_dn8, var_inqs0_a_dn9, var_inqs0_a_dn10, var_inqs0_a_dn11, var_inqs0_a_dn14, eq1_e1020_d_n16, eq1_e1020_q, var_q_nqs_a_dn16,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq1_e1022_q_d_n16),
        );
        let (eq2_e1029, eq2_e1029_d_n0, eq2_e1029_d_n2, eq2_e1029_d_n4, eq2_e1029_d_n5, eq2_e1029_d_n6, eq2_e1029_d_n7, eq2_e1029_d_n8, eq2_e1029_d_n9, eq2_e1029_d_n10, eq2_e1029_d_n11, eq2_e1029_d_n14, eq2_e1029_d_n17, eq2_e1029_q, eq2_e1029_q_d_n17,) = {
    if (var_guard2313 != 0.0) {
        let eq2_e1026_q: f64 = var_q_nqs_k;
        let eq2_e1027: f64 = (var_inqs0_k + var_q_nqs_k);
        let eq2_e1027_d_n17: f64 = (var_inqs0_k_dn17 + var_q_nqs_k_dn17);
        let eq2_e1027_q: f64 = eq2_e1026_q;
        (eq2_e1027, var_inqs0_k_dn0, var_inqs0_k_dn2, var_inqs0_k_dn4, var_inqs0_k_dn5, var_inqs0_k_dn6, var_inqs0_k_dn7, var_inqs0_k_dn8, var_inqs0_k_dn9, var_inqs0_k_dn10, var_inqs0_k_dn11, var_inqs0_k_dn14, eq2_e1027_d_n17, eq2_e1027_q, var_q_nqs_k_dn17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq2_e1029_q_d_n17),
        );
        let (eq5_e1046, eq5_e1046_d_n0, eq5_e1046_d_n2, eq5_e1046_d_n4, eq5_e1046_d_n5, eq5_e1046_d_n6, eq5_e1046_d_n7, eq5_e1046_d_n8, eq5_e1046_d_n9, eq5_e1046_d_n10, eq5_e1046_d_n11, eq5_e1046_d_n14, eq5_e1046_d_n18, eq5_e1046_q, eq5_e1046_q_d_n18,) = {
    if (var_guard2314 != 0.0) {
        let eq5_e1043_q: f64 = var_w_nqs_a;
        let eq5_e1044: f64 = (var_iwnqs0_a + var_w_nqs_a);
        let eq5_e1044_d_n18: f64 = (var_iwnqs0_a_dn18 + var_w_nqs_a_dn18);
        let eq5_e1044_q: f64 = eq5_e1043_q;
        (eq5_e1044, var_iwnqs0_a_dn0, var_iwnqs0_a_dn2, var_iwnqs0_a_dn4, var_iwnqs0_a_dn5, var_iwnqs0_a_dn6, var_iwnqs0_a_dn7, var_iwnqs0_a_dn8, var_iwnqs0_a_dn9, var_iwnqs0_a_dn10, var_iwnqs0_a_dn11, var_iwnqs0_a_dn14, eq5_e1044_d_n18, eq5_e1044_q, var_w_nqs_a_dn18,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq5_e1046_q_d_n18),
        );
        let eq15_e1092_q: f64 = var_qbs;
        let eq15_e1093: f64 = (p.p87 * var_qbs);
        let eq15_e1093_d_n0: f64 = (p.p87 * var_qbs_dn0);
        let eq15_e1093_d_n2: f64 = (p.p87 * var_qbs_dn2);
        let eq15_e1093_d_n4: f64 = (p.p87 * var_qbs_dn4);
        let eq15_e1093_d_n5: f64 = (p.p87 * var_qbs_dn5);
        let eq15_e1093_d_n6: f64 = (p.p87 * var_qbs_dn6);
        let eq15_e1093_d_n7: f64 = (p.p87 * var_qbs_dn7);
        let eq15_e1093_d_n8: f64 = (p.p87 * var_qbs_dn8);
        let eq15_e1093_d_n9: f64 = (p.p87 * var_qbs_dn9);
        let eq15_e1093_d_n10: f64 = (p.p87 * var_qbs_dn10);
        let eq15_e1093_d_n11: f64 = (p.p87 * var_qbs_dn11);
        let eq15_e1093_d_n14: f64 = (p.p87 * var_qbs_dn14);
        let eq15_e1093_q: f64 = (p.p87 * eq15_e1092_q);
        let eq15_reactive_node_derivatives: [f64; 19] = [eq15_e1093_d_n0, 0.0, eq15_e1093_d_n2, 0.0, eq15_e1093_d_n4, eq15_e1093_d_n5, eq15_e1093_d_n6, eq15_e1093_d_n7, eq15_e1093_d_n8, eq15_e1093_d_n9, eq15_e1093_d_n10, eq15_e1093_d_n11, 0.0, 0.0, eq15_e1093_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[2]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e1096_q: f64 = var_qbd;
        let eq16_e1097: f64 = (p.p87 * var_qbd);
        let eq16_e1097_d_n0: f64 = (p.p87 * var_qbd_dn0);
        let eq16_e1097_d_n2: f64 = (p.p87 * var_qbd_dn2);
        let eq16_e1097_d_n4: f64 = (p.p87 * var_qbd_dn4);
        let eq16_e1097_d_n5: f64 = (p.p87 * var_qbd_dn5);
        let eq16_e1097_d_n6: f64 = (p.p87 * var_qbd_dn6);
        let eq16_e1097_d_n7: f64 = (p.p87 * var_qbd_dn7);
        let eq16_e1097_d_n8: f64 = (p.p87 * var_qbd_dn8);
        let eq16_e1097_d_n9: f64 = (p.p87 * var_qbd_dn9);
        let eq16_e1097_d_n10: f64 = (p.p87 * var_qbd_dn10);
        let eq16_e1097_d_n11: f64 = (p.p87 * var_qbd_dn11);
        let eq16_e1097_d_n14: f64 = (p.p87 * var_qbd_dn14);
        let eq16_e1097_d_n16: f64 = (p.p87 * var_qbd_dn16);
        let eq16_e1097_d_n17: f64 = (p.p87 * var_qbd_dn17);
        let eq16_e1097_d_n18: f64 = (p.p87 * var_qbd_dn18);
        let eq16_e1097_q: f64 = (p.p87 * eq16_e1096_q);
        let eq16_reactive_node_derivatives: [f64; 19] = [eq16_e1097_d_n0, 0.0, eq16_e1097_d_n2, 0.0, eq16_e1097_d_n4, eq16_e1097_d_n5, eq16_e1097_d_n6, eq16_e1097_d_n7, eq16_e1097_d_n8, eq16_e1097_d_n9, eq16_e1097_d_n10, eq16_e1097_d_n11, 0.0, 0.0, eq16_e1097_d_n14, 0.0, eq16_e1097_d_n16, eq16_e1097_d_n17, eq16_e1097_d_n18];
        let eq16_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[0]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1116, eq19_e1116_d_n0, eq19_e1116_d_n2, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, eq19_e1116_d_n14, eq19_e1116_q,) = {
    if (var_guard2413 != 0.0) {
        let eq19_e1113_q: f64 = var_qbsi;
        let eq19_e1114: f64 = (p.p87 * var_qbsi);
        let eq19_e1114_d_n0: f64 = (p.p87 * var_qbsi_dn0);
        let eq19_e1114_d_n2: f64 = (p.p87 * var_qbsi_dn2);
        let eq19_e1114_d_n4: f64 = (p.p87 * var_qbsi_dn4);
        let eq19_e1114_d_n5: f64 = (p.p87 * var_qbsi_dn5);
        let eq19_e1114_d_n6: f64 = (p.p87 * var_qbsi_dn6);
        let eq19_e1114_d_n7: f64 = (p.p87 * var_qbsi_dn7);
        let eq19_e1114_d_n8: f64 = (p.p87 * var_qbsi_dn8);
        let eq19_e1114_d_n9: f64 = (p.p87 * var_qbsi_dn9);
        let eq19_e1114_d_n10: f64 = (p.p87 * var_qbsi_dn10);
        let eq19_e1114_d_n11: f64 = (p.p87 * var_qbsi_dn11);
        let eq19_e1114_d_n14: f64 = (p.p87 * var_qbsi_dn14);
        let eq19_e1114_q: f64 = (p.p87 * eq19_e1113_q);
        (eq19_e1114, eq19_e1114_d_n0, eq19_e1114_d_n2, eq19_e1114_d_n4, eq19_e1114_d_n5, eq19_e1114_d_n6, eq19_e1114_d_n7, eq19_e1114_d_n8, eq19_e1114_d_n9, eq19_e1114_d_n10, eq19_e1114_d_n11, eq19_e1114_d_n14, eq19_e1114_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e1116_d_n0, 0.0, eq19_e1116_d_n2, 0.0, eq19_e1116_d_n4, eq19_e1116_d_n5, eq19_e1116_d_n6, eq19_e1116_d_n7, eq19_e1116_d_n8, eq19_e1116_d_n9, eq19_e1116_d_n10, eq19_e1116_d_n11, 0.0, 0.0, eq19_e1116_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1123, eq20_e1123_d_n0, eq20_e1123_d_n2, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, eq20_e1123_d_n14, eq20_e1123_q,) = {
    if (var_guard2413 != 0.0) {
        let eq20_e1120_q: f64 = var_qbdi;
        let eq20_e1121: f64 = (p.p87 * var_qbdi);
        let eq20_e1121_d_n0: f64 = (p.p87 * var_qbdi_dn0);
        let eq20_e1121_d_n2: f64 = (p.p87 * var_qbdi_dn2);
        let eq20_e1121_d_n4: f64 = (p.p87 * var_qbdi_dn4);
        let eq20_e1121_d_n5: f64 = (p.p87 * var_qbdi_dn5);
        let eq20_e1121_d_n6: f64 = (p.p87 * var_qbdi_dn6);
        let eq20_e1121_d_n7: f64 = (p.p87 * var_qbdi_dn7);
        let eq20_e1121_d_n8: f64 = (p.p87 * var_qbdi_dn8);
        let eq20_e1121_d_n9: f64 = (p.p87 * var_qbdi_dn9);
        let eq20_e1121_d_n10: f64 = (p.p87 * var_qbdi_dn10);
        let eq20_e1121_d_n11: f64 = (p.p87 * var_qbdi_dn11);
        let eq20_e1121_d_n14: f64 = (p.p87 * var_qbdi_dn14);
        let eq20_e1121_q: f64 = (p.p87 * eq20_e1120_q);
        (eq20_e1121, eq20_e1121_d_n0, eq20_e1121_d_n2, eq20_e1121_d_n4, eq20_e1121_d_n5, eq20_e1121_d_n6, eq20_e1121_d_n7, eq20_e1121_d_n8, eq20_e1121_d_n9, eq20_e1121_d_n10, eq20_e1121_d_n11, eq20_e1121_d_n14, eq20_e1121_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e1123_d_n0, 0.0, eq20_e1123_d_n2, 0.0, eq20_e1123_d_n4, eq20_e1123_d_n5, eq20_e1123_d_n6, eq20_e1123_d_n7, eq20_e1123_d_n8, eq20_e1123_d_n9, eq20_e1123_d_n10, eq20_e1123_d_n11, 0.0, 0.0, eq20_e1123_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq28_e1167: f64 = (var_qg + var_qg_nqs);
        let eq28_e1168_q: f64 = eq28_e1167;
        let eq28_e1169: f64 = (p.p87 * eq28_e1167);
        let eq28_e1169_d_n0: f64 = (p.p87 * var_qg_dn0);
        let eq28_e1169_d_n2: f64 = (p.p87 * var_qg_dn2);
        let eq28_e1169_d_n4: f64 = (p.p87 * var_qg_dn4);
        let eq28_e1169_d_n5: f64 = (p.p87 * var_qg_dn5);
        let eq28_e1169_d_n6: f64 = (p.p87 * var_qg_dn6);
        let eq28_e1169_d_n7: f64 = (p.p87 * var_qg_dn7);
        let eq28_e1169_d_n8: f64 = (p.p87 * var_qg_dn8);
        let eq28_e1169_d_n9: f64 = (p.p87 * var_qg_dn9);
        let eq28_e1169_d_n10: f64 = (p.p87 * var_qg_dn10);
        let eq28_e1169_d_n11: f64 = (p.p87 * var_qg_dn11);
        let eq28_e1169_d_n12: f64 = (p.p87 * var_qg_nqs_dn12);
        let eq28_e1169_d_n13: f64 = (p.p87 * var_qg_nqs_dn13);
        let eq28_e1169_d_n14: f64 = (p.p87 * var_qg_dn14);
        let eq28_e1169_q: f64 = (p.p87 * eq28_e1168_q);
        let eq28_reactive_node_derivatives: [f64; 19] = [eq28_e1169_d_n0, 0.0, eq28_e1169_d_n2, 0.0, eq28_e1169_d_n4, eq28_e1169_d_n5, eq28_e1169_d_n6, eq28_e1169_d_n7, eq28_e1169_d_n8, eq28_e1169_d_n9, eq28_e1169_d_n10, eq28_e1169_d_n11, eq28_e1169_d_n12, eq28_e1169_d_n13, eq28_e1169_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq28_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq29_e1173: f64 = (var_qd + var_qd_nqs);
        let eq29_e1173_d_n0: f64 = (var_qd_dn0 + var_qd_nqs_dn0);
        let eq29_e1173_d_n2: f64 = (var_qd_dn2 + var_qd_nqs_dn2);
        let eq29_e1173_d_n4: f64 = (var_qd_dn4 + var_qd_nqs_dn4);
        let eq29_e1173_d_n5: f64 = (var_qd_dn5 + var_qd_nqs_dn5);
        let eq29_e1173_d_n6: f64 = (var_qd_dn6 + var_qd_nqs_dn6);
        let eq29_e1173_d_n7: f64 = (var_qd_dn7 + var_qd_nqs_dn7);
        let eq29_e1173_d_n8: f64 = (var_qd_dn8 + var_qd_nqs_dn8);
        let eq29_e1173_d_n9: f64 = (var_qd_dn9 + var_qd_nqs_dn9);
        let eq29_e1173_d_n10: f64 = (var_qd_dn10 + var_qd_nqs_dn10);
        let eq29_e1173_d_n11: f64 = (var_qd_dn11 + var_qd_nqs_dn11);
        let eq29_e1173_d_n14: f64 = (var_qd_dn14 + var_qd_nqs_dn14);
        let eq29_e1174_q: f64 = eq29_e1173;
        let eq29_e1175: f64 = (p.p87 * eq29_e1173);
        let eq29_e1175_d_n0: f64 = (p.p87 * eq29_e1173_d_n0);
        let eq29_e1175_d_n2: f64 = (p.p87 * eq29_e1173_d_n2);
        let eq29_e1175_d_n4: f64 = (p.p87 * eq29_e1173_d_n4);
        let eq29_e1175_d_n5: f64 = (p.p87 * eq29_e1173_d_n5);
        let eq29_e1175_d_n6: f64 = (p.p87 * eq29_e1173_d_n6);
        let eq29_e1175_d_n7: f64 = (p.p87 * eq29_e1173_d_n7);
        let eq29_e1175_d_n8: f64 = (p.p87 * eq29_e1173_d_n8);
        let eq29_e1175_d_n9: f64 = (p.p87 * eq29_e1173_d_n9);
        let eq29_e1175_d_n10: f64 = (p.p87 * eq29_e1173_d_n10);
        let eq29_e1175_d_n11: f64 = (p.p87 * eq29_e1173_d_n11);
        let eq29_e1175_d_n12: f64 = (p.p87 * var_qd_nqs_dn12);
        let eq29_e1175_d_n14: f64 = (p.p87 * eq29_e1173_d_n14);
        let eq29_e1175_q: f64 = (p.p87 * eq29_e1174_q);
        let eq29_reactive_node_derivatives: [f64; 19] = [eq29_e1175_d_n0, 0.0, eq29_e1175_d_n2, 0.0, eq29_e1175_d_n4, eq29_e1175_d_n5, eq29_e1175_d_n6, eq29_e1175_d_n7, eq29_e1175_d_n8, eq29_e1175_d_n9, eq29_e1175_d_n10, eq29_e1175_d_n11, eq29_e1175_d_n12, 0.0, eq29_e1175_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq29_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq29_reactive_node_derivatives,
            branches,
            &eq29_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e1180: f64 = (var_qg_nqs + var_qd_nqs);
        let eq30_e1180_d_n12: f64 = (var_qg_nqs_dn12 + var_qd_nqs_dn12);
        let eq30_e1182: f64 = (eq30_e1180 + var_qs_nqs);
        let eq30_e1182_d_n0: f64 = (var_qd_nqs_dn0 + var_qs_nqs_dn0);
        let eq30_e1182_d_n2: f64 = (var_qd_nqs_dn2 + var_qs_nqs_dn2);
        let eq30_e1182_d_n4: f64 = (var_qd_nqs_dn4 + var_qs_nqs_dn4);
        let eq30_e1182_d_n5: f64 = (var_qd_nqs_dn5 + var_qs_nqs_dn5);
        let eq30_e1182_d_n6: f64 = (var_qd_nqs_dn6 + var_qs_nqs_dn6);
        let eq30_e1182_d_n7: f64 = (var_qd_nqs_dn7 + var_qs_nqs_dn7);
        let eq30_e1182_d_n8: f64 = (var_qd_nqs_dn8 + var_qs_nqs_dn8);
        let eq30_e1182_d_n9: f64 = (var_qd_nqs_dn9 + var_qs_nqs_dn9);
        let eq30_e1182_d_n10: f64 = (var_qd_nqs_dn10 + var_qs_nqs_dn10);
        let eq30_e1182_d_n11: f64 = (var_qd_nqs_dn11 + var_qs_nqs_dn11);
        let eq30_e1182_d_n12: f64 = (eq30_e1180_d_n12 + var_qs_nqs_dn12);
        let eq30_e1182_d_n14: f64 = (var_qd_nqs_dn14 + var_qs_nqs_dn14);
        let eq30_e1183: f64 = (var_qb - eq30_e1182);
        let eq30_e1183_d_n0: f64 = (var_qb_dn0 - eq30_e1182_d_n0);
        let eq30_e1183_d_n2: f64 = (var_qb_dn2 - eq30_e1182_d_n2);
        let eq30_e1183_d_n4: f64 = (var_qb_dn4 - eq30_e1182_d_n4);
        let eq30_e1183_d_n5: f64 = (var_qb_dn5 - eq30_e1182_d_n5);
        let eq30_e1183_d_n6: f64 = (var_qb_dn6 - eq30_e1182_d_n6);
        let eq30_e1183_d_n7: f64 = (var_qb_dn7 - eq30_e1182_d_n7);
        let eq30_e1183_d_n8: f64 = (var_qb_dn8 - eq30_e1182_d_n8);
        let eq30_e1183_d_n9: f64 = (var_qb_dn9 - eq30_e1182_d_n9);
        let eq30_e1183_d_n10: f64 = (var_qb_dn10 - eq30_e1182_d_n10);
        let eq30_e1183_d_n11: f64 = (var_qb_dn11 - eq30_e1182_d_n11);
        let eq30_e1183_d_n14: f64 = (var_qb_dn14 - eq30_e1182_d_n14);
        let eq30_e1184_q: f64 = eq30_e1183;
        let eq30_e1185: f64 = (p.p87 * eq30_e1183);
        let eq30_e1185_d_n0: f64 = (p.p87 * eq30_e1183_d_n0);
        let eq30_e1185_d_n2: f64 = (p.p87 * eq30_e1183_d_n2);
        let eq30_e1185_d_n4: f64 = (p.p87 * eq30_e1183_d_n4);
        let eq30_e1185_d_n5: f64 = (p.p87 * eq30_e1183_d_n5);
        let eq30_e1185_d_n6: f64 = (p.p87 * eq30_e1183_d_n6);
        let eq30_e1185_d_n7: f64 = (p.p87 * eq30_e1183_d_n7);
        let eq30_e1185_d_n8: f64 = (p.p87 * eq30_e1183_d_n8);
        let eq30_e1185_d_n9: f64 = (p.p87 * eq30_e1183_d_n9);
        let eq30_e1185_d_n10: f64 = (p.p87 * eq30_e1183_d_n10);
        let eq30_e1185_d_n11: f64 = (p.p87 * eq30_e1183_d_n11);
        let eq30_e1185_d_n12: f64 = (p.p87 * (-eq30_e1182_d_n12));
        let eq30_e1185_d_n13: f64 = (p.p87 * (-var_qg_nqs_dn13));
        let eq30_e1185_d_n14: f64 = (p.p87 * eq30_e1183_d_n14);
        let eq30_e1185_q: f64 = (p.p87 * eq30_e1184_q);
        let eq30_reactive_node_derivatives: [f64; 19] = [eq30_e1185_d_n0, 0.0, eq30_e1185_d_n2, 0.0, eq30_e1185_d_n4, eq30_e1185_d_n5, eq30_e1185_d_n6, eq30_e1185_d_n7, eq30_e1185_d_n8, eq30_e1185_d_n9, eq30_e1185_d_n10, eq30_e1185_d_n11, eq30_e1185_d_n12, eq30_e1185_d_n13, eq30_e1185_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq30_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[8]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
        let eq31_e1188_q: f64 = var_qgext;
        let eq31_e1189: f64 = (p.p87 * var_qgext);
        let eq31_e1189_d_n0: f64 = (p.p87 * var_qgext_dn0);
        let eq31_e1189_d_n2: f64 = (p.p87 * var_qgext_dn2);
        let eq31_e1189_d_n4: f64 = (p.p87 * var_qgext_dn4);
        let eq31_e1189_d_n5: f64 = (p.p87 * var_qgext_dn5);
        let eq31_e1189_d_n6: f64 = (p.p87 * var_qgext_dn6);
        let eq31_e1189_d_n7: f64 = (p.p87 * var_qgext_dn7);
        let eq31_e1189_d_n8: f64 = (p.p87 * var_qgext_dn8);
        let eq31_e1189_d_n9: f64 = (p.p87 * var_qgext_dn9);
        let eq31_e1189_d_n10: f64 = (p.p87 * var_qgext_dn10);
        let eq31_e1189_d_n11: f64 = (p.p87 * var_qgext_dn11);
        let eq31_e1189_d_n14: f64 = (p.p87 * var_qgext_dn14);
        let eq31_e1189_q: f64 = (p.p87 * eq31_e1188_q);
        let eq31_reactive_node_derivatives: [f64; 19] = [eq31_e1189_d_n0, 0.0, eq31_e1189_d_n2, 0.0, eq31_e1189_d_n4, eq31_e1189_d_n5, eq31_e1189_d_n6, eq31_e1189_d_n7, eq31_e1189_d_n8, eq31_e1189_d_n9, eq31_e1189_d_n10, eq31_e1189_d_n11, 0.0, 0.0, eq31_e1189_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq31_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes,
            &eq31_reactive_node_derivatives,
            branches,
            &eq31_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e1192_q: f64 = var_qdext;
        let eq32_e1193: f64 = (p.p87 * var_qdext);
        let eq32_e1193_d_n0: f64 = (p.p87 * var_qdext_dn0);
        let eq32_e1193_d_n2: f64 = (p.p87 * var_qdext_dn2);
        let eq32_e1193_d_n4: f64 = (p.p87 * var_qdext_dn4);
        let eq32_e1193_d_n5: f64 = (p.p87 * var_qdext_dn5);
        let eq32_e1193_d_n6: f64 = (p.p87 * var_qdext_dn6);
        let eq32_e1193_d_n7: f64 = (p.p87 * var_qdext_dn7);
        let eq32_e1193_d_n8: f64 = (p.p87 * var_qdext_dn8);
        let eq32_e1193_d_n9: f64 = (p.p87 * var_qdext_dn9);
        let eq32_e1193_d_n10: f64 = (p.p87 * var_qdext_dn10);
        let eq32_e1193_d_n11: f64 = (p.p87 * var_qdext_dn11);
        let eq32_e1193_d_n14: f64 = (p.p87 * var_qdext_dn14);
        let eq32_e1193_q: f64 = (p.p87 * eq32_e1192_q);
        let eq32_reactive_node_derivatives: [f64; 19] = [eq32_e1193_d_n0, 0.0, eq32_e1193_d_n2, 0.0, eq32_e1193_d_n4, eq32_e1193_d_n5, eq32_e1193_d_n6, eq32_e1193_d_n7, eq32_e1193_d_n8, eq32_e1193_d_n9, eq32_e1193_d_n10, eq32_e1193_d_n11, 0.0, 0.0, eq32_e1193_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq32_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[2]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
        let eq33_e1196_q: f64 = var_qbext;
        let eq33_e1197: f64 = (p.p87 * var_qbext);
        let eq33_e1197_d_n0: f64 = (p.p87 * var_qbext_dn0);
        let eq33_e1197_d_n2: f64 = (p.p87 * var_qbext_dn2);
        let eq33_e1197_d_n4: f64 = (p.p87 * var_qbext_dn4);
        let eq33_e1197_d_n5: f64 = (p.p87 * var_qbext_dn5);
        let eq33_e1197_d_n6: f64 = (p.p87 * var_qbext_dn6);
        let eq33_e1197_d_n7: f64 = (p.p87 * var_qbext_dn7);
        let eq33_e1197_d_n8: f64 = (p.p87 * var_qbext_dn8);
        let eq33_e1197_d_n9: f64 = (p.p87 * var_qbext_dn9);
        let eq33_e1197_d_n10: f64 = (p.p87 * var_qbext_dn10);
        let eq33_e1197_d_n11: f64 = (p.p87 * var_qbext_dn11);
        let eq33_e1197_d_n14: f64 = (p.p87 * var_qbext_dn14);
        let eq33_e1197_q: f64 = (p.p87 * eq33_e1196_q);
        let eq33_reactive_node_derivatives: [f64; 19] = [eq33_e1197_d_n0, 0.0, eq33_e1197_d_n2, 0.0, eq33_e1197_d_n4, eq33_e1197_d_n5, eq33_e1197_d_n6, eq33_e1197_d_n7, eq33_e1197_d_n8, eq33_e1197_d_n9, eq33_e1197_d_n10, eq33_e1197_d_n11, 0.0, 0.0, eq33_e1197_d_n14, 0.0, 0.0, 0.0, 0.0];
        let eq33_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[2]),
            nodes,
            &eq33_reactive_node_derivatives,
            branches,
            &eq33_reactive_branch_derivatives,
            multiplicity,
        );
        let eq34_e1199: f64 = (-p.p87);
        let eq34_e1201_q: f64 = var_qfd;
        let eq34_e1202: f64 = (eq34_e1199 * var_qfd);
        let eq34_e1202_d_n0: f64 = (eq34_e1199 * var_qfd_dn0);
        let eq34_e1202_d_n2: f64 = (eq34_e1199 * var_qfd_dn2);
        let eq34_e1202_d_n7: f64 = (eq34_e1199 * var_qfd_dn7);
        let eq34_e1202_q: f64 = (eq34_e1199 * eq34_e1201_q);
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq34_e1202_d_n0),
            nodes[2],
            multiplicity * (eq34_e1202_d_n2),
            nodes[7],
            multiplicity * (eq34_e1202_d_n7),
        );
        let eq35_e1204: f64 = (-p.p87);
        let eq35_e1206_q: f64 = var_qfs;
        let eq35_e1207: f64 = (eq35_e1204 * var_qfs);
        let eq35_e1207_d_n2: f64 = (eq35_e1204 * var_qfs_dn2);
        let eq35_e1207_d_n7: f64 = (eq35_e1204 * var_qfs_dn7);
        let eq35_e1207_q: f64 = (eq35_e1204 * eq35_e1206_q);
        stamper.stamp_current_reactive_node2(
            Some(nodes[7]),
            Some(nodes[2]),
            nodes[2],
            multiplicity * (eq35_e1207_d_n2),
            nodes[7],
            multiplicity * (eq35_e1207_d_n7),
        );
        let eq41_e1236: f64 = ((nv15 - 0.0) * var_sigrat_s);
        let eq41_e1236_d_n0: f64 = ((nv15 - 0.0) * var_sigrat_s_dn0);
        let eq41_e1236_d_n2: f64 = ((nv15 - 0.0) * var_sigrat_s_dn2);
        let eq41_e1236_d_n4: f64 = ((nv15 - 0.0) * var_sigrat_s_dn4);
        let eq41_e1236_d_n5: f64 = ((nv15 - 0.0) * var_sigrat_s_dn5);
        let eq41_e1236_d_n6: f64 = ((nv15 - 0.0) * var_sigrat_s_dn6);
        let eq41_e1236_d_n7: f64 = ((nv15 - 0.0) * var_sigrat_s_dn7);
        let eq41_e1236_d_n8: f64 = ((nv15 - 0.0) * var_sigrat_s_dn8);
        let eq41_e1236_d_n9: f64 = ((nv15 - 0.0) * var_sigrat_s_dn9);
        let eq41_e1236_d_n10: f64 = ((nv15 - 0.0) * var_sigrat_s_dn10);
        let eq41_e1236_d_n11: f64 = ((nv15 - 0.0) * var_sigrat_s_dn11);
        let eq41_e1236_d_n14: f64 = ((nv15 - 0.0) * var_sigrat_s_dn14);
        let eq41_e1237_q: f64 = eq41_e1236;
        let eq41_reactive_node_derivatives: [f64; 19] = [eq41_e1236_d_n0, 0.0, eq41_e1236_d_n2, 0.0, eq41_e1236_d_n4, eq41_e1236_d_n5, eq41_e1236_d_n6, eq41_e1236_d_n7, eq41_e1236_d_n8, eq41_e1236_d_n9, eq41_e1236_d_n10, eq41_e1236_d_n11, 0.0, 0.0, eq41_e1236_d_n14, var_sigrat_s, 0.0, 0.0, 0.0];
        let eq41_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[8]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1240: f64 = ((nv15 - 0.0) * var_sigrat_d);
        let eq42_e1240_d_n0: f64 = ((nv15 - 0.0) * var_sigrat_d_dn0);
        let eq42_e1240_d_n2: f64 = ((nv15 - 0.0) * var_sigrat_d_dn2);
        let eq42_e1240_d_n4: f64 = ((nv15 - 0.0) * var_sigrat_d_dn4);
        let eq42_e1240_d_n5: f64 = ((nv15 - 0.0) * var_sigrat_d_dn5);
        let eq42_e1240_d_n6: f64 = ((nv15 - 0.0) * var_sigrat_d_dn6);
        let eq42_e1240_d_n7: f64 = ((nv15 - 0.0) * var_sigrat_d_dn7);
        let eq42_e1240_d_n8: f64 = ((nv15 - 0.0) * var_sigrat_d_dn8);
        let eq42_e1240_d_n9: f64 = ((nv15 - 0.0) * var_sigrat_d_dn9);
        let eq42_e1240_d_n10: f64 = ((nv15 - 0.0) * var_sigrat_d_dn10);
        let eq42_e1240_d_n11: f64 = ((nv15 - 0.0) * var_sigrat_d_dn11);
        let eq42_e1240_d_n14: f64 = ((nv15 - 0.0) * var_sigrat_d_dn14);
        let eq42_e1241_q: f64 = eq42_e1240;
        let eq42_reactive_node_derivatives: [f64; 19] = [eq42_e1240_d_n0, 0.0, eq42_e1240_d_n2, 0.0, eq42_e1240_d_n4, eq42_e1240_d_n5, eq42_e1240_d_n6, eq42_e1240_d_n7, eq42_e1240_d_n8, eq42_e1240_d_n9, eq42_e1240_d_n10, eq42_e1240_d_n11, 0.0, 0.0, eq42_e1240_d_n14, var_sigrat_d, 0.0, 0.0, 0.0];
        let eq42_reactive_branch_derivatives: [f64; 13] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq62_e1362, eq62_e1362_d_n12, eq62_e1362_q,) = {
    if (p.p28 != 0.0) {
        let eq62_e1359: f64 = (var_cqi * (nv12 - 0.0));
        let eq62_e1360_q: f64 = eq62_e1359;
        (eq62_e1359, var_cqi, eq62_e1360_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (eq62_e1362_d_n12),
        );
        let (eq63_e1369, eq63_e1369_d_n13, eq63_e1369_q,) = {
    if (p.p28 != 0.0) {
        let eq63_e1366: f64 = (var_cqb * (nv13 - 0.0));
        let eq63_e1367_q: f64 = eq63_e1366;
        (eq63_e1366, var_cqb, eq63_e1367_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq63_e1369_d_n13),
        );
        let (eq67_e1388, eq67_e1388_d_n14, eq67_e1388_q,) = {
    if (p.p29 != 0.0) {
        let eq67_e1386_q: f64 = (nv14 - 0.0);
        ((nv14 - 0.0), 1.0, eq67_e1386_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[14]),
            None,
            nodes[14],
            multiplicity * (eq67_e1388_d_n14),
        );
    }
}
