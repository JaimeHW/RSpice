#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_29(
        var_cs_t: f64,
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_e_eff0: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_eta_mu: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1188: f64,
        var_mue_t: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_rsb_i: f64,
        var_rsg_i: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_ther_i: f64,
        var_thesatb_i: f64,
        var_thesatg_i: f64,
        var_thesatt_i: f64,
        var_vsbx: f64,
        var_vsbx_dn5: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xcor_t: f64,
        var_xi0s: f64,
        var_xi0s_dn5: f64,
        var_xi0s_dn6: f64,
        var_xi0s_dn7: f64,
        var_xi0s_dn8: f64,
        var_alphas_slot: &mut f64,
        var_alphas_dn5_slot: &mut f64,
        var_alphas_dn6_slot: &mut f64,
        var_alphas_dn7_slot: &mut f64,
        var_alphas_dn8_slot: &mut f64,
        var_alphas_rv_slot: &mut f64,
        var_ds_slot: &mut f64,
        var_ds_dn5_slot: &mut f64,
        var_ds_dn6_slot: &mut f64,
        var_ds_dn7_slot: &mut f64,
        var_ds_dn8_slot: &mut f64,
        var_ds_rv_slot: &mut f64,
        var_eeffs_slot: &mut f64,
        var_eeffs_dn5_slot: &mut f64,
        var_eeffs_dn6_slot: &mut f64,
        var_eeffs_dn7_slot: &mut f64,
        var_eeffs_dn8_slot: &mut f64,
        var_eeffs_rv_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_rv_slot: &mut f64,
        var_gmobs_slot: &mut f64,
        var_gmobs_dn5_slot: &mut f64,
        var_gmobs_dn6_slot: &mut f64,
        var_gmobs_dn7_slot: &mut f64,
        var_gmobs_dn8_slot: &mut f64,
        var_gmobs_rv_slot: &mut f64,
        var_gr_slot: &mut f64,
        var_gr_dn5_slot: &mut f64,
        var_gr_dn6_slot: &mut f64,
        var_gr_dn7_slot: &mut f64,
        var_gr_dn8_slot: &mut f64,
        var_gr_rv_slot: &mut f64,
        var_guard1191_slot: &mut f64,
        var_guard1191_rv_slot: &mut f64,
        var_guard1192_slot: &mut f64,
        var_guard1192_rv_slot: &mut f64,
        var_guard1193_slot: &mut f64,
        var_guard1193_rv_slot: &mut f64,
        var_guard1194_slot: &mut f64,
        var_guard1194_rv_slot: &mut f64,
        var_guard1195_slot: &mut f64,
        var_guard1195_rv_slot: &mut f64,
        var_guard1196_slot: &mut f64,
        var_guard1196_rv_slot: &mut f64,
        var_mutmp_slot: &mut f64,
        var_mutmp_dn5_slot: &mut f64,
        var_mutmp_dn6_slot: &mut f64,
        var_mutmp_dn7_slot: &mut f64,
        var_mutmp_dn8_slot: &mut f64,
        var_mutmp_rv_slot: &mut f64,
        var_ps_slot: &mut f64,
        var_ps_dn5_slot: &mut f64,
        var_ps_dn6_slot: &mut f64,
        var_ps_dn7_slot: &mut f64,
        var_ps_dn8_slot: &mut f64,
        var_ps_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qis_slot: &mut f64,
        var_qis_dn5_slot: &mut f64,
        var_qis_dn6_slot: &mut f64,
        var_qis_dn7_slot: &mut f64,
        var_qis_dn8_slot: &mut f64,
        var_qis_rv_slot: &mut f64,
        var_rhob_slot: &mut f64,
        var_rhob_dn5_slot: &mut f64,
        var_rhob_dn6_slot: &mut f64,
        var_rhob_dn7_slot: &mut f64,
        var_rhob_dn8_slot: &mut f64,
        var_rhob_rv_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn5_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rhog_rv_slot: &mut f64,
        var_rxcor_slot: &mut f64,
        var_rxcor_dn5_slot: &mut f64,
        var_rxcor_dn6_slot: &mut f64,
        var_rxcor_dn7_slot: &mut f64,
        var_rxcor_dn8_slot: &mut f64,
        var_rxcor_rv_slot: &mut f64,
        var_sqs_slot: &mut f64,
        var_sqs_dn5_slot: &mut f64,
        var_sqs_dn6_slot: &mut f64,
        var_sqs_dn7_slot: &mut f64,
        var_sqs_dn8_slot: &mut f64,
        var_sqs_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_wsat_slot: &mut f64,
        var_wsat_dn5_slot: &mut f64,
        var_wsat_dn6_slot: &mut f64,
        var_wsat_dn7_slot: &mut f64,
        var_wsat_dn8_slot: &mut f64,
        var_wsat_rv_slot: &mut f64,
        var_xgs_slot: &mut f64,
        var_xgs_dn5_slot: &mut f64,
        var_xgs_dn6_slot: &mut f64,
        var_xgs_dn7_slot: &mut f64,
        var_xgs_dn8_slot: &mut f64,
        var_xgs_rv_slot: &mut f64,
        var_xitsb_slot: &mut f64,
        var_xitsb_dn5_slot: &mut f64,
        var_xitsb_dn6_slot: &mut f64,
        var_xitsb_dn7_slot: &mut f64,
        var_xitsb_dn8_slot: &mut f64,
        var_xitsb_rv_slot: &mut f64,
    ) {
        let mut var_alphas: f64 = *var_alphas_slot;
        let mut var_alphas_dn5: f64 = *var_alphas_dn5_slot;
        let mut var_alphas_dn6: f64 = *var_alphas_dn6_slot;
        let mut var_alphas_dn7: f64 = *var_alphas_dn7_slot;
        let mut var_alphas_dn8: f64 = *var_alphas_dn8_slot;
        let mut var_alphas_rv: f64 = *var_alphas_rv_slot;
        let mut var_ds: f64 = *var_ds_slot;
        let mut var_ds_dn5: f64 = *var_ds_dn5_slot;
        let mut var_ds_dn6: f64 = *var_ds_dn6_slot;
        let mut var_ds_dn7: f64 = *var_ds_dn7_slot;
        let mut var_ds_dn8: f64 = *var_ds_dn8_slot;
        let mut var_ds_rv: f64 = *var_ds_rv_slot;
        let mut var_eeffs: f64 = *var_eeffs_slot;
        let mut var_eeffs_dn5: f64 = *var_eeffs_dn5_slot;
        let mut var_eeffs_dn6: f64 = *var_eeffs_dn6_slot;
        let mut var_eeffs_dn7: f64 = *var_eeffs_dn7_slot;
        let mut var_eeffs_dn8: f64 = *var_eeffs_dn8_slot;
        let mut var_eeffs_rv: f64 = *var_eeffs_rv_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_rv: f64 = *var_factheta_rv_slot;
        let mut var_gmobs: f64 = *var_gmobs_slot;
        let mut var_gmobs_dn5: f64 = *var_gmobs_dn5_slot;
        let mut var_gmobs_dn6: f64 = *var_gmobs_dn6_slot;
        let mut var_gmobs_dn7: f64 = *var_gmobs_dn7_slot;
        let mut var_gmobs_dn8: f64 = *var_gmobs_dn8_slot;
        let mut var_gmobs_rv: f64 = *var_gmobs_rv_slot;
        let mut var_gr: f64 = *var_gr_slot;
        let mut var_gr_dn5: f64 = *var_gr_dn5_slot;
        let mut var_gr_dn6: f64 = *var_gr_dn6_slot;
        let mut var_gr_dn7: f64 = *var_gr_dn7_slot;
        let mut var_gr_dn8: f64 = *var_gr_dn8_slot;
        let mut var_gr_rv: f64 = *var_gr_rv_slot;
        let mut var_guard1191: f64 = *var_guard1191_slot;
        let mut var_guard1191_rv: f64 = *var_guard1191_rv_slot;
        let mut var_guard1192: f64 = *var_guard1192_slot;
        let mut var_guard1192_rv: f64 = *var_guard1192_rv_slot;
        let mut var_guard1193: f64 = *var_guard1193_slot;
        let mut var_guard1193_rv: f64 = *var_guard1193_rv_slot;
        let mut var_guard1194: f64 = *var_guard1194_slot;
        let mut var_guard1194_rv: f64 = *var_guard1194_rv_slot;
        let mut var_guard1195: f64 = *var_guard1195_slot;
        let mut var_guard1195_rv: f64 = *var_guard1195_rv_slot;
        let mut var_guard1196: f64 = *var_guard1196_slot;
        let mut var_guard1196_rv: f64 = *var_guard1196_rv_slot;
        let mut var_mutmp: f64 = *var_mutmp_slot;
        let mut var_mutmp_dn5: f64 = *var_mutmp_dn5_slot;
        let mut var_mutmp_dn6: f64 = *var_mutmp_dn6_slot;
        let mut var_mutmp_dn7: f64 = *var_mutmp_dn7_slot;
        let mut var_mutmp_dn8: f64 = *var_mutmp_dn8_slot;
        let mut var_mutmp_rv: f64 = *var_mutmp_rv_slot;
        let mut var_ps: f64 = *var_ps_slot;
        let mut var_ps_dn5: f64 = *var_ps_dn5_slot;
        let mut var_ps_dn6: f64 = *var_ps_dn6_slot;
        let mut var_ps_dn7: f64 = *var_ps_dn7_slot;
        let mut var_ps_dn8: f64 = *var_ps_dn8_slot;
        let mut var_ps_rv: f64 = *var_ps_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qis: f64 = *var_qis_slot;
        let mut var_qis_dn5: f64 = *var_qis_dn5_slot;
        let mut var_qis_dn6: f64 = *var_qis_dn6_slot;
        let mut var_qis_dn7: f64 = *var_qis_dn7_slot;
        let mut var_qis_dn8: f64 = *var_qis_dn8_slot;
        let mut var_qis_rv: f64 = *var_qis_rv_slot;
        let mut var_rhob: f64 = *var_rhob_slot;
        let mut var_rhob_dn5: f64 = *var_rhob_dn5_slot;
        let mut var_rhob_dn6: f64 = *var_rhob_dn6_slot;
        let mut var_rhob_dn7: f64 = *var_rhob_dn7_slot;
        let mut var_rhob_dn8: f64 = *var_rhob_dn8_slot;
        let mut var_rhob_rv: f64 = *var_rhob_rv_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn5: f64 = *var_rhog_dn5_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rhog_rv: f64 = *var_rhog_rv_slot;
        let mut var_rxcor: f64 = *var_rxcor_slot;
        let mut var_rxcor_dn5: f64 = *var_rxcor_dn5_slot;
        let mut var_rxcor_dn6: f64 = *var_rxcor_dn6_slot;
        let mut var_rxcor_dn7: f64 = *var_rxcor_dn7_slot;
        let mut var_rxcor_dn8: f64 = *var_rxcor_dn8_slot;
        let mut var_rxcor_rv: f64 = *var_rxcor_rv_slot;
        let mut var_sqs: f64 = *var_sqs_slot;
        let mut var_sqs_dn5: f64 = *var_sqs_dn5_slot;
        let mut var_sqs_dn6: f64 = *var_sqs_dn6_slot;
        let mut var_sqs_dn7: f64 = *var_sqs_dn7_slot;
        let mut var_sqs_dn8: f64 = *var_sqs_dn8_slot;
        let mut var_sqs_rv: f64 = *var_sqs_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_wsat: f64 = *var_wsat_slot;
        let mut var_wsat_dn5: f64 = *var_wsat_dn5_slot;
        let mut var_wsat_dn6: f64 = *var_wsat_dn6_slot;
        let mut var_wsat_dn7: f64 = *var_wsat_dn7_slot;
        let mut var_wsat_dn8: f64 = *var_wsat_dn8_slot;
        let mut var_wsat_rv: f64 = *var_wsat_rv_slot;
        let mut var_xgs: f64 = *var_xgs_slot;
        let mut var_xgs_dn5: f64 = *var_xgs_dn5_slot;
        let mut var_xgs_dn6: f64 = *var_xgs_dn6_slot;
        let mut var_xgs_dn7: f64 = *var_xgs_dn7_slot;
        let mut var_xgs_dn8: f64 = *var_xgs_dn8_slot;
        let mut var_xgs_rv: f64 = *var_xgs_rv_slot;
        let mut var_xitsb: f64 = *var_xitsb_slot;
        let mut var_xitsb_dn5: f64 = *var_xitsb_dn5_slot;
        let mut var_xitsb_dn6: f64 = *var_xitsb_dn6_slot;
        let mut var_xitsb_dn7: f64 = *var_xitsb_dn7_slot;
        let mut var_xitsb_dn8: f64 = *var_xitsb_dn8_slot;
        let mut var_xitsb_rv: f64 = *var_xitsb_rv_slot;

        let (assign42620_e55853, assign42620_e55853_d_n5, assign42620_e55853_d_n6, assign42620_e55853_d_n7, assign42620_e55853_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42620_e55847: f64 = (var_x_s + 1.0);
        let assign42620_e55849: f64 = (assign42620_e55847 + var_xi0s);
        let assign42620_e55850: f64 = (var_delta_ns * assign42620_e55849);
        let assign42620_e55851: f64 = (var_delta_1s - assign42620_e55850);
        (assign42620_e55851, (var_delta_1s_dn5 - ((var_delta_ns_dn5 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn5 + var_xi0s_dn5)))), (var_delta_1s_dn6 - ((var_delta_ns_dn6 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn6 + var_xi0s_dn6)))), (var_delta_1s_dn7 - ((var_delta_ns_dn7 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn7 + var_xi0s_dn7)))), (var_delta_1s_dn8 - ((var_delta_ns_dn8 * assign42620_e55849) + (var_delta_ns * (var_x_s_dn8 + var_xi0s_dn8)))),)
    } else {
        (var_ds, var_ds_dn5, var_ds_dn6, var_ds_dn7, var_ds_dn8,)
    }
};
        var_ds = assign42620_e55853;
        var_ds_dn5 = assign42620_e55853_d_n5;
        var_ds_dn6 = assign42620_e55853_d_n6;
        var_ds_dn7 = assign42620_e55853_d_n7;
        var_ds_dn8 = assign42620_e55853_d_n8;
        var_ds_rv = 0.0;

        let assign42630_e55856: f64 = if var_x_s < 1e-5 { 1.0 } else { 0.0 };
        var_guard1191 = assign42630_e55856;
        var_guard1191_rv = 0.0;

        let (assign42640_e55878, assign42640_e55878_d_n5, assign42640_e55878_d_n6, assign42640_e55878_d_n7, assign42640_e55878_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42640_e55863: f64 = (var_x_s * var_x_s);
        let assign42640_e55870: f64 = (0.25 * var_x_s);
        let assign42640_e55871: f64 = (1.0 - assign42640_e55870);
        let assign42640_e55872: f64 = (var_x_s * assign42640_e55871);
        let assign42640_e55873: f64 = (0.3333333333333333 * assign42640_e55872);
        let assign42640_e55874: f64 = (1.0 - assign42640_e55873);
        let assign42640_e55875: f64 = (assign42640_e55863 * assign42640_e55874);
        let assign42640_e55876: f64 = (0.5 * assign42640_e55875);
        (assign42640_e55876, (0.5 * ((((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn5 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn5))))))))), (0.5 * ((((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn6 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn6))))))))), (0.5 * ((((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn7 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn7))))))))), (0.5 * ((((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)) * assign42640_e55874) + (assign42640_e55863 * (-(0.3333333333333333 * ((var_x_s_dn8 * assign42640_e55871) + (var_x_s * (-(0.25 * var_x_s_dn8))))))))),)
    } else {
        (var_ps, var_ps_dn5, var_ps_dn6, var_ps_dn7, var_ps_dn8,)
    }
};
        var_ps = assign42640_e55878;
        var_ps_dn5 = assign42640_e55878_d_n5;
        var_ps_dn6 = assign42640_e55878_d_n6;
        var_ps_dn7 = assign42640_e55878_d_n7;
        var_ps_dn8 = assign42640_e55878_d_n8;
        var_ps_rv = 0.0;

        let (assign42650_e55898, assign42650_e55898_d_n5, assign42650_e55898_d_n6, assign42650_e55898_d_n7, assign42650_e55898_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42650_e55885: f64 = (var_delta_ns * var_x_s);
        let assign42650_e55887: f64 = (assign42650_e55885 * var_x_s);
        let assign42650_e55889: f64 = (assign42650_e55887 * var_x_s);
        let assign42650_e55893: f64 = (1.75 * var_x_s);
        let assign42650_e55894: f64 = (1.0 + assign42650_e55893);
        let assign42650_e55895: f64 = (assign42650_e55889 * assign42650_e55894);
        let assign42650_e55896: f64 = (0.16666666666666666 * assign42650_e55895);
        (assign42650_e55896, (0.16666666666666666 * ((((((((var_delta_ns_dn5 * var_x_s) + (var_delta_ns * var_x_s_dn5)) * var_x_s) + (assign42650_e55885 * var_x_s_dn5)) * var_x_s) + (assign42650_e55887 * var_x_s_dn5)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn5)))), (0.16666666666666666 * ((((((((var_delta_ns_dn6 * var_x_s) + (var_delta_ns * var_x_s_dn6)) * var_x_s) + (assign42650_e55885 * var_x_s_dn6)) * var_x_s) + (assign42650_e55887 * var_x_s_dn6)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn6)))), (0.16666666666666666 * ((((((((var_delta_ns_dn7 * var_x_s) + (var_delta_ns * var_x_s_dn7)) * var_x_s) + (assign42650_e55885 * var_x_s_dn7)) * var_x_s) + (assign42650_e55887 * var_x_s_dn7)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn7)))), (0.16666666666666666 * ((((((((var_delta_ns_dn8 * var_x_s) + (var_delta_ns * var_x_s_dn8)) * var_x_s) + (assign42650_e55885 * var_x_s_dn8)) * var_x_s) + (assign42650_e55887 * var_x_s_dn8)) * assign42650_e55894) + (assign42650_e55889 * (1.75 * var_x_s_dn8)))),)
    } else {
        (var_ds, var_ds_dn5, var_ds_dn6, var_ds_dn7, var_ds_dn8,)
    }
};
        var_ds = assign42650_e55898;
        var_ds_dn5 = assign42650_e55898_d_n5;
        var_ds_dn6 = assign42650_e55898_d_n6;
        var_ds_dn7 = assign42650_e55898_d_n7;
        var_ds_dn8 = assign42650_e55898_d_n8;
        var_ds_rv = 0.0;

        let (assign42660_e55915, assign42660_e55915_d_n5, assign42660_e55915_d_n6, assign42660_e55915_d_n7, assign42660_e55915_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42660_e55908: f64 = (0.25 * var_x_s);
        let assign42660_e55909: f64 = (1.0 - assign42660_e55908);
        let assign42660_e55910: f64 = (var_x_s * assign42660_e55909);
        let assign42660_e55911: f64 = (0.3333333333333333 * assign42660_e55910);
        let assign42660_e55912: f64 = (1.0 - assign42660_e55911);
        let assign42660_e55913: f64 = (assign42660_e55912).sqrt();
        (assign42660_e55913, ((-(0.3333333333333333 * ((var_x_s_dn5 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn5)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn6 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn6)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn7 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn7)))))) / (2.0 * assign42660_e55913)), ((-(0.3333333333333333 * ((var_x_s_dn8 * assign42660_e55909) + (var_x_s * (-(0.25 * var_x_s_dn8)))))) / (2.0 * assign42660_e55913)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign42660_e55915;
        var_temp__blk936_dn5 = assign42660_e55915_d_n5;
        var_temp__blk936_dn6 = assign42660_e55915_d_n6;
        var_temp__blk936_dn7 = assign42660_e55915_d_n7;
        var_temp__blk936_dn8 = assign42660_e55915_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign42670_e55925, assign42670_e55925_d_n5, assign42670_e55925_d_n6, assign42670_e55925_d_n7, assign42670_e55925_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42670_e55922: f64 = (var_x_s * var_temp__blk936);
        let assign42670_e55923: f64 = (0.7071067811865475 * assign42670_e55922);
        (assign42670_e55923, (0.7071067811865475 * ((var_x_s_dn5 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn5))), (0.7071067811865475 * ((var_x_s_dn6 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn6))), (0.7071067811865475 * ((var_x_s_dn7 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn7))), (0.7071067811865475 * ((var_x_s_dn8 * var_temp__blk936) + (var_x_s * var_temp__blk936_dn8))),)
    } else {
        (var_sqs, var_sqs_dn5, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8,)
    }
};
        var_sqs = assign42670_e55925;
        var_sqs_dn5 = assign42670_e55925_d_n5;
        var_sqs_dn6 = assign42670_e55925_d_n6;
        var_sqs_dn7 = assign42670_e55925_d_n7;
        var_sqs_dn8 = assign42670_e55925_d_n8;
        var_sqs_rv = 0.0;

        let (assign42680_e55949, assign42680_e55949_d_n5, assign42680_e55949_d_n6, assign42680_e55949_d_n7, assign42680_e55949_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 != 0.0)) {
        let assign42680_e55935: f64 = (0.5 * var_x_s);
        let assign42680_e55936: f64 = (1.0 - assign42680_e55935);
        let assign42680_e55940: f64 = (var_x_s * var_x_s);
        let assign42680_e55941: f64 = (0.16666666666666666 * assign42680_e55940);
        let assign42680_e55942: f64 = (assign42680_e55936 + assign42680_e55941);
        let assign42680_e55943: f64 = (var_gf * assign42680_e55942);
        let assign42680_e55945: f64 = (assign42680_e55943 / var_temp__blk936);
        let assign42680_e55946: f64 = (0.7071067811865475 * assign42680_e55945);
        let assign42680_e55947: f64 = (1.0 + assign42680_e55946);
        (assign42680_e55947, (0.7071067811865475 * (((((var_gf_dn5 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn5)) + (0.16666666666666666 * ((var_x_s_dn5 * var_x_s) + (var_x_s * var_x_s_dn5)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn6 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn6)) + (0.16666666666666666 * ((var_x_s_dn6 * var_x_s) + (var_x_s * var_x_s_dn6)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn7 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn7)) + (0.16666666666666666 * ((var_x_s_dn7 * var_x_s) + (var_x_s * var_x_s_dn7)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936))), (0.7071067811865475 * (((((var_gf_dn8 * assign42680_e55942) + (var_gf * ((-(0.5 * var_x_s_dn8)) + (0.16666666666666666 * ((var_x_s_dn8 * var_x_s) + (var_x_s * var_x_s_dn8)))))) * var_temp__blk936) - (assign42680_e55943 * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936))),)
    } else {
        (var_alphas, var_alphas_dn5, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8,)
    }
};
        var_alphas = assign42680_e55949;
        var_alphas_dn5 = assign42680_e55949_d_n5;
        var_alphas_dn6 = assign42680_e55949_d_n6;
        var_alphas_dn7 = assign42680_e55949_d_n7;
        var_alphas_dn8 = assign42680_e55949_d_n8;
        var_alphas_rv = 0.0;

        let (assign42690_e55960, assign42690_e55960_d_n5, assign42690_e55960_d_n6, assign42690_e55960_d_n7, assign42690_e55960_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42690_e55956: f64 = (var_x_s - 1.0);
        let assign42690_e55958: f64 = (assign42690_e55956 + var_es);
        (assign42690_e55958, (var_x_s_dn5 + var_es_dn5), (var_x_s_dn6 + var_es_dn6), (var_x_s_dn7 + var_es_dn7), (var_x_s_dn8 + var_es_dn8),)
    } else {
        (var_ps, var_ps_dn5, var_ps_dn6, var_ps_dn7, var_ps_dn8,)
    }
};
        var_ps = assign42690_e55960;
        var_ps_dn5 = assign42690_e55960_d_n5;
        var_ps_dn6 = assign42690_e55960_d_n6;
        var_ps_dn7 = assign42690_e55960_d_n7;
        var_ps_dn8 = assign42690_e55960_d_n8;
        var_ps_rv = 0.0;

        let (assign42700_e55968, assign42700_e55968_d_n5, assign42700_e55968_d_n6, assign42700_e55968_d_n7, assign42700_e55968_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42700_e55966: f64 = (var_ps).sqrt();
        (assign42700_e55966, (var_ps_dn5 / (2.0 * assign42700_e55966)), (var_ps_dn6 / (2.0 * assign42700_e55966)), (var_ps_dn7 / (2.0 * assign42700_e55966)), (var_ps_dn8 / (2.0 * assign42700_e55966)),)
    } else {
        (var_sqs, var_sqs_dn5, var_sqs_dn6, var_sqs_dn7, var_sqs_dn8,)
    }
};
        var_sqs = assign42700_e55968;
        var_sqs_dn5 = assign42700_e55968_d_n5;
        var_sqs_dn6 = assign42700_e55968_d_n6;
        var_sqs_dn7 = assign42700_e55968_d_n7;
        var_sqs_dn8 = assign42700_e55968_d_n8;
        var_sqs_rv = 0.0;

        let (assign42710_e55985, assign42710_e55985_d_n5, assign42710_e55985_d_n6, assign42710_e55985_d_n7, assign42710_e55985_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1191 == 0.0)) {
        let assign42710_e55978: f64 = (1.0 - var_es);
        let assign42710_e55979: f64 = (var_gf * assign42710_e55978);
        let assign42710_e55981: f64 = (assign42710_e55979 / var_sqs);
        let assign42710_e55982: f64 = (0.5 * assign42710_e55981);
        let assign42710_e55983: f64 = (1.0 + assign42710_e55982);
        (assign42710_e55983, (0.5 * (((((var_gf_dn5 * assign42710_e55978) + (var_gf * (-var_es_dn5))) * var_sqs) - (assign42710_e55979 * var_sqs_dn5)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn6 * assign42710_e55978) + (var_gf * (-var_es_dn6))) * var_sqs) - (assign42710_e55979 * var_sqs_dn6)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn7 * assign42710_e55978) + (var_gf * (-var_es_dn7))) * var_sqs) - (assign42710_e55979 * var_sqs_dn7)) / (var_sqs * var_sqs))), (0.5 * (((((var_gf_dn8 * assign42710_e55978) + (var_gf * (-var_es_dn8))) * var_sqs) - (assign42710_e55979 * var_sqs_dn8)) / (var_sqs * var_sqs))),)
    } else {
        (var_alphas, var_alphas_dn5, var_alphas_dn6, var_alphas_dn7, var_alphas_dn8,)
    }
};
        var_alphas = assign42710_e55985;
        var_alphas_dn5 = assign42710_e55985_d_n5;
        var_alphas_dn6 = assign42710_e55985_d_n6;
        var_alphas_dn7 = assign42710_e55985_d_n7;
        var_alphas_dn8 = assign42710_e55985_d_n8;
        var_alphas_rv = 0.0;

        let (assign42720_e56001, assign42720_e56001_d_n5, assign42720_e56001_d_n6, assign42720_e56001_d_n7, assign42720_e56001_d_n8,) = {
    if (var_guard1188 != 0.0) {
        let assign42720_e55990: f64 = (0.2 * var_xcor_t);
        let assign42720_e55992: f64 = (assign42720_e55990 * var_vsbx);
        let assign42720_e55993: f64 = (1.0 + assign42720_e55992);
        let assign42720_e55997: f64 = (var_xcor_t * var_vsbx);
        let assign42720_e55998: f64 = (1.0 + assign42720_e55997);
        let assign42720_e55999: f64 = (assign42720_e55993 / assign42720_e55998);
        (assign42720_e55999, ((((assign42720_e55990 * var_vsbx_dn5) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn5))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn6) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn6))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn7) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn7))) / (assign42720_e55998 * assign42720_e55998)), ((((assign42720_e55990 * var_vsbx_dn8) * assign42720_e55998) - (assign42720_e55993 * (var_xcor_t * var_vsbx_dn8))) / (assign42720_e55998 * assign42720_e55998)),)
    } else {
        (var_rxcor, var_rxcor_dn5, var_rxcor_dn6, var_rxcor_dn7, var_rxcor_dn8,)
    }
};
        var_rxcor = assign42720_e56001;
        var_rxcor_dn5 = assign42720_e56001_d_n5;
        var_rxcor_dn6 = assign42720_e56001_d_n6;
        var_rxcor_dn7 = assign42720_e56001_d_n7;
        var_rxcor_dn8 = assign42720_e56001_d_n8;
        var_rxcor_rv = 0.0;

        let assign42730_e56004: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1192 = assign42730_e56004;
        var_guard1192_rv = 0.0;

        let (assign42740_e56015, assign42740_e56015_d_n5, assign42740_e56015_d_n6, assign42740_e56015_d_n7, assign42740_e56015_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42740_e56011: f64 = (var_ps + var_ds);
        let assign42740_e56012: f64 = (assign42740_e56011).sqrt();
        let assign42740_e56013: f64 = (var_gf * assign42740_e56012);
        (assign42740_e56013, ((var_gf_dn5 * assign42740_e56012) + (var_gf * ((var_ps_dn5 + var_ds_dn5) / (2.0 * assign42740_e56012)))), ((var_gf_dn6 * assign42740_e56012) + (var_gf * ((var_ps_dn6 + var_ds_dn6) / (2.0 * assign42740_e56012)))), ((var_gf_dn7 * assign42740_e56012) + (var_gf * ((var_ps_dn7 + var_ds_dn7) / (2.0 * assign42740_e56012)))), ((var_gf_dn8 * assign42740_e56012) + (var_gf * ((var_ps_dn8 + var_ds_dn8) / (2.0 * assign42740_e56012)))),)
    } else {
        (var_xgs, var_xgs_dn5, var_xgs_dn6, var_xgs_dn7, var_xgs_dn8,)
    }
};
        var_xgs = assign42740_e56015;
        var_xgs_dn5 = assign42740_e56015_d_n5;
        var_xgs_dn6 = assign42740_e56015_d_n6;
        var_xgs_dn7 = assign42740_e56015_d_n7;
        var_xgs_dn8 = assign42740_e56015_d_n8;
        var_xgs_rv = 0.0;

        let (assign42750_e56031, assign42750_e56031_d_n5, assign42750_e56031_d_n6, assign42750_e56031_d_n7, assign42750_e56031_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42750_e56021: f64 = (var_gf2 * var_ds);
        let assign42750_e56023: f64 = (assign42750_e56021 * var_phit1);
        let assign42750_e56027: f64 = (var_gf * var_sqs);
        let assign42750_e56028: f64 = (var_xgs + assign42750_e56027);
        let assign42750_e56029: f64 = (assign42750_e56023 / assign42750_e56028);
        (assign42750_e56029, (((((((var_gf2_dn5 * var_ds) + (var_gf2 * var_ds_dn5)) * var_phit1) + (assign42750_e56021 * var_phit1_dn5)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn5 + ((var_gf_dn5 * var_sqs) + (var_gf * var_sqs_dn5))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn6 * var_ds) + (var_gf2 * var_ds_dn6)) * var_phit1) + (assign42750_e56021 * var_phit1_dn6)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn6 + ((var_gf_dn6 * var_sqs) + (var_gf * var_sqs_dn6))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn7 * var_ds) + (var_gf2 * var_ds_dn7)) * var_phit1) + (assign42750_e56021 * var_phit1_dn7)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn7 + ((var_gf_dn7 * var_sqs) + (var_gf * var_sqs_dn7))))) / (assign42750_e56028 * assign42750_e56028)), (((((((var_gf2_dn8 * var_ds) + (var_gf2 * var_ds_dn8)) * var_phit1) + (assign42750_e56021 * var_phit1_dn8)) * assign42750_e56028) - (assign42750_e56023 * (var_xgs_dn8 + ((var_gf_dn8 * var_sqs) + (var_gf * var_sqs_dn8))))) / (assign42750_e56028 * assign42750_e56028)),)
    } else {
        (var_qis, var_qis_dn5, var_qis_dn6, var_qis_dn7, var_qis_dn8,)
    }
};
        var_qis = assign42750_e56031;
        var_qis_dn5 = assign42750_e56031_d_n5;
        var_qis_dn6 = assign42750_e56031_d_n6;
        var_qis_dn7 = assign42750_e56031_d_n7;
        var_qis_dn8 = assign42750_e56031_d_n8;
        var_qis_rv = 0.0;

        let (assign42760_e56041, assign42760_e56041_d_n5, assign42760_e56041_d_n6, assign42760_e56041_d_n7, assign42760_e56041_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42760_e56037: f64 = (var_sqs * var_gf);
        let assign42760_e56039: f64 = (assign42760_e56037 * var_phit1);
        (assign42760_e56039, ((((var_sqs_dn5 * var_gf) + (var_sqs * var_gf_dn5)) * var_phit1) + (assign42760_e56037 * var_phit1_dn5)), ((((var_sqs_dn6 * var_gf) + (var_sqs * var_gf_dn6)) * var_phit1) + (assign42760_e56037 * var_phit1_dn6)), ((((var_sqs_dn7 * var_gf) + (var_sqs * var_gf_dn7)) * var_phit1) + (assign42760_e56037 * var_phit1_dn7)), ((((var_sqs_dn8 * var_gf) + (var_sqs * var_gf_dn8)) * var_phit1) + (assign42760_e56037 * var_phit1_dn8)),)
    } else {
        (var_qbs, var_qbs_dn5, var_qbs_dn6, var_qbs_dn7, var_qbs_dn8,)
    }
};
        var_qbs = assign42760_e56041;
        var_qbs_dn5 = assign42760_e56041_d_n5;
        var_qbs_dn6 = assign42760_e56041_d_n6;
        var_qbs_dn7 = assign42760_e56041_d_n7;
        var_qbs_dn8 = assign42760_e56041_d_n8;
        var_qbs_rv = 0.0;

        let assign42770_e56044: f64 = if var_rsb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1193 = assign42770_e56044;
        var_guard1193_rv = 0.0;

        let (assign42780_e56058, assign42780_e56058_d_n5, assign42780_e56058_d_n6, assign42780_e56058_d_n7, assign42780_e56058_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1193 != 0.0)) {
        let assign42780_e56054: f64 = (var_rsb_i * var_vsbx);
        let assign42780_e56055: f64 = (1.0 - assign42780_e56054);
        let assign42780_e56056: f64 = (1.0 / assign42780_e56055);
        (assign42780_e56056, (-((-(var_rsb_i * var_vsbx_dn5)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn6)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn7)) / (assign42780_e56055 * assign42780_e56055))), (-((-(var_rsb_i * var_vsbx_dn8)) / (assign42780_e56055 * assign42780_e56055))),)
    } else {
        (var_rhob, var_rhob_dn5, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8,)
    }
};
        var_rhob = assign42780_e56058;
        var_rhob_dn5 = assign42780_e56058_d_n5;
        var_rhob_dn6 = assign42780_e56058_d_n6;
        var_rhob_dn7 = assign42780_e56058_d_n7;
        var_rhob_dn8 = assign42780_e56058_d_n8;
        var_rhob_rv = 0.0;

        let (assign42790_e56071, assign42790_e56071_d_n5, assign42790_e56071_d_n6, assign42790_e56071_d_n7, assign42790_e56071_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1193 == 0.0)) {
        let assign42790_e56068: f64 = (var_rsb_i * var_vsbx);
        let assign42790_e56069: f64 = (1.0 + assign42790_e56068);
        (assign42790_e56069, (var_rsb_i * var_vsbx_dn5), (var_rsb_i * var_vsbx_dn6), (var_rsb_i * var_vsbx_dn7), (var_rsb_i * var_vsbx_dn8),)
    } else {
        (var_rhob, var_rhob_dn5, var_rhob_dn6, var_rhob_dn7, var_rhob_dn8,)
    }
};
        var_rhob = assign42790_e56071;
        var_rhob_dn5 = assign42790_e56071_d_n5;
        var_rhob_dn6 = assign42790_e56071_d_n6;
        var_rhob_dn7 = assign42790_e56071_d_n7;
        var_rhob_dn8 = assign42790_e56071_d_n8;
        var_rhob_rv = 0.0;

        let assign42800_e56074: f64 = if var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1194 = assign42800_e56074;
        var_guard1194_rv = 0.0;

        let (assign42810_e56086, assign42810_e56086_d_n5, assign42810_e56086_d_n6, assign42810_e56086_d_n7, assign42810_e56086_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1194 != 0.0)) {
        let assign42810_e56083: f64 = (var_rsg_i * var_qis);
        let assign42810_e56084: f64 = (1.0 - assign42810_e56083);
        (assign42810_e56084, (-(var_rsg_i * var_qis_dn5)), (-(var_rsg_i * var_qis_dn6)), (-(var_rsg_i * var_qis_dn7)), (-(var_rsg_i * var_qis_dn8)),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign42810_e56086;
        var_rhog_dn5 = assign42810_e56086_d_n5;
        var_rhog_dn6 = assign42810_e56086_d_n6;
        var_rhog_dn7 = assign42810_e56086_d_n7;
        var_rhog_dn8 = assign42810_e56086_d_n8;
        var_rhog_rv = 0.0;

        let (assign42820_e56101, assign42820_e56101_d_n5, assign42820_e56101_d_n6, assign42820_e56101_d_n7, assign42820_e56101_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1194 == 0.0)) {
        let assign42820_e56097: f64 = (var_rsg_i * var_qis);
        let assign42820_e56098: f64 = (1.0 + assign42820_e56097);
        let assign42820_e56099: f64 = (1.0 / assign42820_e56098);
        (assign42820_e56099, (-((var_rsg_i * var_qis_dn5) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn6) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn7) / (assign42820_e56098 * assign42820_e56098))), (-((var_rsg_i * var_qis_dn8) / (assign42820_e56098 * assign42820_e56098))),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign42820_e56101;
        var_rhog_dn5 = assign42820_e56101_d_n5;
        var_rhog_dn6 = assign42820_e56101_d_n6;
        var_rhog_dn7 = assign42820_e56101_d_n7;
        var_rhog_dn8 = assign42820_e56101_d_n8;
        var_rhog_rv = 0.0;

        let (assign42830_e56113, assign42830_e56113_d_n5, assign42830_e56113_d_n6, assign42830_e56113_d_n7, assign42830_e56113_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42830_e56107: f64 = (var_ther_i * var_rhob);
        let assign42830_e56109: f64 = (assign42830_e56107 * var_rhog);
        let assign42830_e56111: f64 = (assign42830_e56109 * var_qis);
        (assign42830_e56111, (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign42830_e56107 * var_rhog_dn5)) * var_qis) + (assign42830_e56109 * var_qis_dn5)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign42830_e56107 * var_rhog_dn6)) * var_qis) + (assign42830_e56109 * var_qis_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign42830_e56107 * var_rhog_dn7)) * var_qis) + (assign42830_e56109 * var_qis_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign42830_e56107 * var_rhog_dn8)) * var_qis) + (assign42830_e56109 * var_qis_dn8)),)
    } else {
        (var_gr, var_gr_dn5, var_gr_dn6, var_gr_dn7, var_gr_dn8,)
    }
};
        var_gr = assign42830_e56113;
        var_gr_dn5 = assign42830_e56113_d_n5;
        var_gr_dn6 = assign42830_e56113_d_n6;
        var_gr_dn7 = assign42830_e56113_d_n7;
        var_gr_dn8 = assign42830_e56113_d_n8;
        var_gr_rv = 0.0;

        let (assign42840_e56125, assign42840_e56125_d_n5, assign42840_e56125_d_n6, assign42840_e56125_d_n7, assign42840_e56125_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42840_e56121: f64 = (var_eta_mu * var_qis);
        let assign42840_e56122: f64 = (var_qbs + assign42840_e56121);
        let assign42840_e56123: f64 = (var_e_eff0 * assign42840_e56122);
        (assign42840_e56123, (var_e_eff0 * (var_qbs_dn5 + (var_eta_mu * var_qis_dn5))), (var_e_eff0 * (var_qbs_dn6 + (var_eta_mu * var_qis_dn6))), (var_e_eff0 * (var_qbs_dn7 + (var_eta_mu * var_qis_dn7))), (var_e_eff0 * (var_qbs_dn8 + (var_eta_mu * var_qis_dn8))),)
    } else {
        (var_eeffs, var_eeffs_dn5, var_eeffs_dn6, var_eeffs_dn7, var_eeffs_dn8,)
    }
};
        var_eeffs = assign42840_e56125;
        var_eeffs_dn5 = assign42840_e56125_d_n5;
        var_eeffs_dn6 = assign42840_e56125_d_n6;
        var_eeffs_dn7 = assign42840_e56125_d_n7;
        var_eeffs_dn8 = assign42840_e56125_d_n8;
        var_eeffs_rv = 0.0;

        let (assign42850_e56138, assign42850_e56138_d_n5, assign42850_e56138_d_n6, assign42850_e56138_d_n7, assign42850_e56138_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42850_e56132: f64 = (var_ps + var_ds);
        let assign42850_e56134: f64 = (assign42850_e56132 + 1e-14);
        let assign42850_e56135: f64 = (var_ps / assign42850_e56134);
        let assign42850_e56136: f64 = (assign42850_e56135).ln();
        (assign42850_e56136, ((((var_ps_dn5 * assign42850_e56134) - (var_ps * (var_ps_dn5 + var_ds_dn5))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn6 * assign42850_e56134) - (var_ps * (var_ps_dn6 + var_ds_dn6))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn7 * assign42850_e56134) - (var_ps * (var_ps_dn7 + var_ds_dn7))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135), ((((var_ps_dn8 * assign42850_e56134) - (var_ps * (var_ps_dn8 + var_ds_dn8))) / (assign42850_e56134 * assign42850_e56134)) / assign42850_e56135),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign42850_e56138;
        var_temp1_dn5 = assign42850_e56138_d_n5;
        var_temp1_dn6 = assign42850_e56138_d_n6;
        var_temp1_dn7 = assign42850_e56138_d_n7;
        var_temp1_dn8 = assign42850_e56138_d_n8;
        var_temp1_rv = 0.0;

        let (assign42860_e56157, assign42860_e56157_d_n5, assign42860_e56157_d_n6, assign42860_e56157_d_n7, assign42860_e56157_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42860_e56144: f64 = (var_eeffs * var_mue_t);
        let assign42860_e56146: f64 = (assign42860_e56144).powf(var_themu_t);
        let assign42860_e56150: f64 = (0.5 * var_thecs_t);
        let assign42860_e56152: f64 = (assign42860_e56150 * var_temp1);
        let assign42860_e56153: f64 = (assign42860_e56152).exp();
        let assign42860_e56154: f64 = (var_cs_t * assign42860_e56153);
        let assign42860_e56155: f64 = (assign42860_e56146 + assign42860_e56154);
        (assign42860_e56155, (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn5 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn5 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn5)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn6 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn6 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn6)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn7 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn7 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn7)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign42860_e56144).powf(var_themu_t - 1.0) * (var_eeffs_dn8 * var_mue_t))) } } else { (assign42860_e56146 * (var_themu_t * ((var_eeffs_dn8 * var_mue_t) / assign42860_e56144))) } + (var_cs_t * (assign42860_e56153 * (assign42860_e56150 * var_temp1_dn8)))),)
    } else {
        (var_mutmp, var_mutmp_dn5, var_mutmp_dn6, var_mutmp_dn7, var_mutmp_dn8,)
    }
};
        var_mutmp = assign42860_e56157;
        var_mutmp_dn5 = assign42860_e56157_d_n5;
        var_mutmp_dn6 = assign42860_e56157_d_n6;
        var_mutmp_dn7 = assign42860_e56157_d_n7;
        var_mutmp_dn8 = assign42860_e56157_d_n8;
        var_mutmp_rv = 0.0;

        let (assign42870_e56169, assign42870_e56169_d_n5, assign42870_e56169_d_n6, assign42870_e56169_d_n7, assign42870_e56169_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42870_e56163: f64 = (1.0 + var_mutmp);
        let assign42870_e56165: f64 = (assign42870_e56163 + var_gr);
        let assign42870_e56167: f64 = (assign42870_e56165 * var_rxcor);
        (assign42870_e56167, (((var_mutmp_dn5 + var_gr_dn5) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn5)), (((var_mutmp_dn6 + var_gr_dn6) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn6)), (((var_mutmp_dn7 + var_gr_dn7) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn7)), (((var_mutmp_dn8 + var_gr_dn8) * var_rxcor) + (assign42870_e56165 * var_rxcor_dn8)),)
    } else {
        (var_gmobs, var_gmobs_dn5, var_gmobs_dn6, var_gmobs_dn7, var_gmobs_dn8,)
    }
};
        var_gmobs = assign42870_e56169;
        var_gmobs_dn5 = assign42870_e56169_d_n5;
        var_gmobs_dn6 = assign42870_e56169_d_n6;
        var_gmobs_dn7 = assign42870_e56169_d_n7;
        var_gmobs_dn8 = assign42870_e56169_d_n8;
        var_gmobs_rv = 0.0;

        let assign42880_e56172: f64 = if var_thesatb_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1195 = assign42880_e56172;
        var_guard1195_rv = 0.0;

        let (assign42890_e56186, assign42890_e56186_d_n5, assign42890_e56186_d_n6, assign42890_e56186_d_n7, assign42890_e56186_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1195 != 0.0)) {
        let assign42890_e56182: f64 = (var_thesatb_i * var_vsbx);
        let assign42890_e56183: f64 = (1.0 - assign42890_e56182);
        let assign42890_e56184: f64 = (1.0 / assign42890_e56183);
        (assign42890_e56184, (-((-(var_thesatb_i * var_vsbx_dn5)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn6)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn7)) / (assign42890_e56183 * assign42890_e56183))), (-((-(var_thesatb_i * var_vsbx_dn8)) / (assign42890_e56183 * assign42890_e56183))),)
    } else {
        (var_xitsb, var_xitsb_dn5, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8,)
    }
};
        var_xitsb = assign42890_e56186;
        var_xitsb_dn5 = assign42890_e56186_d_n5;
        var_xitsb_dn6 = assign42890_e56186_d_n6;
        var_xitsb_dn7 = assign42890_e56186_d_n7;
        var_xitsb_dn8 = assign42890_e56186_d_n8;
        var_xitsb_rv = 0.0;

        let (assign42900_e56199, assign42900_e56199_d_n5, assign42900_e56199_d_n6, assign42900_e56199_d_n7, assign42900_e56199_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1195 == 0.0)) {
        let assign42900_e56196: f64 = (var_thesatb_i * var_vsbx);
        let assign42900_e56197: f64 = (1.0 + assign42900_e56196);
        (assign42900_e56197, (var_thesatb_i * var_vsbx_dn5), (var_thesatb_i * var_vsbx_dn6), (var_thesatb_i * var_vsbx_dn7), (var_thesatb_i * var_vsbx_dn8),)
    } else {
        (var_xitsb, var_xitsb_dn5, var_xitsb_dn6, var_xitsb_dn7, var_xitsb_dn8,)
    }
};
        var_xitsb = assign42900_e56199;
        var_xitsb_dn5 = assign42900_e56199_d_n5;
        var_xitsb_dn6 = assign42900_e56199_d_n6;
        var_xitsb_dn7 = assign42900_e56199_d_n7;
        var_xitsb_dn8 = assign42900_e56199_d_n8;
        var_xitsb_rv = 0.0;

        let (assign42910_e56207, assign42910_e56207_d_n5, assign42910_e56207_d_n6, assign42910_e56207_d_n7, assign42910_e56207_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42910_e56205: f64 = (var_qis * var_xitsb);
        (assign42910_e56205, ((var_qis_dn5 * var_xitsb) + (var_qis * var_xitsb_dn5)), ((var_qis_dn6 * var_xitsb) + (var_qis * var_xitsb_dn6)), ((var_qis_dn7 * var_xitsb) + (var_qis * var_xitsb_dn7)), ((var_qis_dn8 * var_xitsb) + (var_qis * var_xitsb_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign42910_e56207;
        var_temp2_dn5 = assign42910_e56207_d_n5;
        var_temp2_dn6 = assign42910_e56207_d_n6;
        var_temp2_dn7 = assign42910_e56207_d_n7;
        var_temp2_dn8 = assign42910_e56207_d_n8;
        var_temp2_rv = 0.0;

        let (assign42920_e56217, assign42920_e56217_d_n5, assign42920_e56217_d_n6, assign42920_e56217_d_n7, assign42920_e56217_d_n8,) = {
    if ((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) {
        let assign42920_e56214: f64 = (var_thesatt_i + var_temp2);
        let assign42920_e56215: f64 = (var_temp2 / assign42920_e56214);
        (assign42920_e56215, (((var_temp2_dn5 * assign42920_e56214) - (var_temp2 * var_temp2_dn5)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn6 * assign42920_e56214) - (var_temp2 * var_temp2_dn6)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn7 * assign42920_e56214) - (var_temp2 * var_temp2_dn7)) / (assign42920_e56214 * assign42920_e56214)), (((var_temp2_dn8 * assign42920_e56214) - (var_temp2 * var_temp2_dn8)) / (assign42920_e56214 * assign42920_e56214)),)
    } else {
        (var_wsat, var_wsat_dn5, var_wsat_dn6, var_wsat_dn7, var_wsat_dn8,)
    }
};
        var_wsat = assign42920_e56217;
        var_wsat_dn5 = assign42920_e56217_d_n5;
        var_wsat_dn6 = assign42920_e56217_d_n6;
        var_wsat_dn7 = assign42920_e56217_d_n7;
        var_wsat_dn8 = assign42920_e56217_d_n8;
        var_wsat_rv = 0.0;

        let assign42930_e56220: f64 = if var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1196 = assign42930_e56220;
        var_guard1196_rv = 0.0;

        let (assign42940_e56234, assign42940_e56234_d_n5, assign42940_e56234_d_n6, assign42940_e56234_d_n7, assign42940_e56234_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1196 != 0.0)) {
        let assign42940_e56230: f64 = (var_thesatg_i * var_wsat);
        let assign42940_e56231: f64 = (1.0 - assign42940_e56230);
        let assign42940_e56232: f64 = (1.0 / assign42940_e56231);
        (assign42940_e56232, (-((-(var_thesatg_i * var_wsat_dn5)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn6)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn7)) / (assign42940_e56231 * assign42940_e56231))), (-((-(var_thesatg_i * var_wsat_dn8)) / (assign42940_e56231 * assign42940_e56231))),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign42940_e56234;
        var_factheta_dn5 = assign42940_e56234_d_n5;
        var_factheta_dn6 = assign42940_e56234_d_n6;
        var_factheta_dn7 = assign42940_e56234_d_n7;
        var_factheta_dn8 = assign42940_e56234_d_n8;
        var_factheta_rv = 0.0;

        *var_alphas_slot = var_alphas;
        *var_alphas_dn5_slot = var_alphas_dn5;
        *var_alphas_dn6_slot = var_alphas_dn6;
        *var_alphas_dn7_slot = var_alphas_dn7;
        *var_alphas_dn8_slot = var_alphas_dn8;
        *var_alphas_rv_slot = var_alphas_rv;
        *var_ds_slot = var_ds;
        *var_ds_dn5_slot = var_ds_dn5;
        *var_ds_dn6_slot = var_ds_dn6;
        *var_ds_dn7_slot = var_ds_dn7;
        *var_ds_dn8_slot = var_ds_dn8;
        *var_ds_rv_slot = var_ds_rv;
        *var_eeffs_slot = var_eeffs;
        *var_eeffs_dn5_slot = var_eeffs_dn5;
        *var_eeffs_dn6_slot = var_eeffs_dn6;
        *var_eeffs_dn7_slot = var_eeffs_dn7;
        *var_eeffs_dn8_slot = var_eeffs_dn8;
        *var_eeffs_rv_slot = var_eeffs_rv;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_rv_slot = var_factheta_rv;
        *var_gmobs_slot = var_gmobs;
        *var_gmobs_dn5_slot = var_gmobs_dn5;
        *var_gmobs_dn6_slot = var_gmobs_dn6;
        *var_gmobs_dn7_slot = var_gmobs_dn7;
        *var_gmobs_dn8_slot = var_gmobs_dn8;
        *var_gmobs_rv_slot = var_gmobs_rv;
        *var_gr_slot = var_gr;
        *var_gr_dn5_slot = var_gr_dn5;
        *var_gr_dn6_slot = var_gr_dn6;
        *var_gr_dn7_slot = var_gr_dn7;
        *var_gr_dn8_slot = var_gr_dn8;
        *var_gr_rv_slot = var_gr_rv;
        *var_guard1191_slot = var_guard1191;
        *var_guard1191_rv_slot = var_guard1191_rv;
        *var_guard1192_slot = var_guard1192;
        *var_guard1192_rv_slot = var_guard1192_rv;
        *var_guard1193_slot = var_guard1193;
        *var_guard1193_rv_slot = var_guard1193_rv;
        *var_guard1194_slot = var_guard1194;
        *var_guard1194_rv_slot = var_guard1194_rv;
        *var_guard1195_slot = var_guard1195;
        *var_guard1195_rv_slot = var_guard1195_rv;
        *var_guard1196_slot = var_guard1196;
        *var_guard1196_rv_slot = var_guard1196_rv;
        *var_mutmp_slot = var_mutmp;
        *var_mutmp_dn5_slot = var_mutmp_dn5;
        *var_mutmp_dn6_slot = var_mutmp_dn6;
        *var_mutmp_dn7_slot = var_mutmp_dn7;
        *var_mutmp_dn8_slot = var_mutmp_dn8;
        *var_mutmp_rv_slot = var_mutmp_rv;
        *var_ps_slot = var_ps;
        *var_ps_dn5_slot = var_ps_dn5;
        *var_ps_dn6_slot = var_ps_dn6;
        *var_ps_dn7_slot = var_ps_dn7;
        *var_ps_dn8_slot = var_ps_dn8;
        *var_ps_rv_slot = var_ps_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qis_slot = var_qis;
        *var_qis_dn5_slot = var_qis_dn5;
        *var_qis_dn6_slot = var_qis_dn6;
        *var_qis_dn7_slot = var_qis_dn7;
        *var_qis_dn8_slot = var_qis_dn8;
        *var_qis_rv_slot = var_qis_rv;
        *var_rhob_slot = var_rhob;
        *var_rhob_dn5_slot = var_rhob_dn5;
        *var_rhob_dn6_slot = var_rhob_dn6;
        *var_rhob_dn7_slot = var_rhob_dn7;
        *var_rhob_dn8_slot = var_rhob_dn8;
        *var_rhob_rv_slot = var_rhob_rv;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn5_slot = var_rhog_dn5;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rhog_rv_slot = var_rhog_rv;
        *var_rxcor_slot = var_rxcor;
        *var_rxcor_dn5_slot = var_rxcor_dn5;
        *var_rxcor_dn6_slot = var_rxcor_dn6;
        *var_rxcor_dn7_slot = var_rxcor_dn7;
        *var_rxcor_dn8_slot = var_rxcor_dn8;
        *var_rxcor_rv_slot = var_rxcor_rv;
        *var_sqs_slot = var_sqs;
        *var_sqs_dn5_slot = var_sqs_dn5;
        *var_sqs_dn6_slot = var_sqs_dn6;
        *var_sqs_dn7_slot = var_sqs_dn7;
        *var_sqs_dn8_slot = var_sqs_dn8;
        *var_sqs_rv_slot = var_sqs_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_wsat_slot = var_wsat;
        *var_wsat_dn5_slot = var_wsat_dn5;
        *var_wsat_dn6_slot = var_wsat_dn6;
        *var_wsat_dn7_slot = var_wsat_dn7;
        *var_wsat_dn8_slot = var_wsat_dn8;
        *var_wsat_rv_slot = var_wsat_rv;
        *var_xgs_slot = var_xgs;
        *var_xgs_dn5_slot = var_xgs_dn5;
        *var_xgs_dn6_slot = var_xgs_dn6;
        *var_xgs_dn7_slot = var_xgs_dn7;
        *var_xgs_dn8_slot = var_xgs_dn8;
        *var_xgs_rv_slot = var_xgs_rv;
        *var_xitsb_slot = var_xitsb;
        *var_xitsb_dn5_slot = var_xitsb_dn5;
        *var_xitsb_dn6_slot = var_xitsb_dn6;
        *var_xitsb_dn7_slot = var_xitsb_dn7;
        *var_xitsb_dn8_slot = var_xitsb_dn8;
        *var_xitsb_rv_slot = var_xitsb_rv;
    }

    pub(super) fn stamp_reactive_block_30(
        var_alphas: f64,
        var_alphas_dn5: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_gmobs: f64,
        var_gmobs_dn5: f64,
        var_gmobs_dn6: f64,
        var_gmobs_dn7: f64,
        var_gmobs_dn8: f64,
        var_guard1188: f64,
        var_guard1192: f64,
        var_guard1196: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn5: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_margin: f64,
        var_margin_dn5: f64,
        var_margin_dn6: f64,
        var_margin_dn7: f64,
        var_margin_dn8: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_ps: f64,
        var_ps_dn5: f64,
        var_ps_dn6: f64,
        var_ps_dn7: f64,
        var_ps_dn8: f64,
        var_qbs: f64,
        var_qbs_dn5: f64,
        var_qbs_dn6: f64,
        var_qbs_dn7: f64,
        var_qbs_dn8: f64,
        var_qis: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rhob: f64,
        var_rhob_dn5: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhog: f64,
        var_rhog_dn5: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_rxcor: f64,
        var_rxcor_dn5: f64,
        var_rxcor_dn6: f64,
        var_rxcor_dn7: f64,
        var_rxcor_dn8: f64,
        var_sp_s_x1: f64,
        var_sp_s_x1_dn5: f64,
        var_sp_s_x1_dn6: f64,
        var_sp_s_x1_dn7: f64,
        var_sp_s_x1_dn8: f64,
        var_sqs: f64,
        var_sqs_dn5: f64,
        var_sqs_dn6: f64,
        var_sqs_dn7: f64,
        var_sqs_dn8: f64,
        var_thesatg_i: f64,
        var_thesatloc: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vgb1: f64,
        var_vgb1_dn5: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vsbx: f64,
        var_vsbx_dn5: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_wsat: f64,
        var_wsat_dn5: f64,
        var_wsat_dn6: f64,
        var_wsat_dn7: f64,
        var_wsat_dn8: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xgs: f64,
        var_xgs_dn5: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_xi: f64,
        var_xi1s: f64,
        var_xi1s_dn5: f64,
        var_xi1s_dn6: f64,
        var_xi1s_dn7: f64,
        var_xi1s_dn8: f64,
        var_xi2s: f64,
        var_xi2s_dn5: f64,
        var_xi2s_dn6: f64,
        var_xi2s_dn7: f64,
        var_xi2s_dn8: f64,
        var_xi_dn5: f64,
        var_xi_dn6: f64,
        var_xi_dn7: f64,
        var_xi_dn8: f64,
        var_xitsb: f64,
        var_xitsb_dn5: f64,
        var_xitsb_dn6: f64,
        var_xitsb_dn7: f64,
        var_xitsb_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_xno_s: f64,
        var_xno_s_dn5: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn5_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alpha_rv_slot: &mut f64,
        var_alphas_dc_slot: &mut f64,
        var_alphas_dc_dn5_slot: &mut f64,
        var_alphas_dc_dn6_slot: &mut f64,
        var_alphas_dc_dn7_slot: &mut f64,
        var_alphas_dc_dn8_slot: &mut f64,
        var_alphas_dc_rv_slot: &mut f64,
        var_asat_slot: &mut f64,
        var_asat_dn5_slot: &mut f64,
        var_asat_dn6_slot: &mut f64,
        var_asat_dn7_slot: &mut f64,
        var_asat_dn8_slot: &mut f64,
        var_asat_rv_slot: &mut f64,
        var_dd_slot: &mut f64,
        var_dd_dn5_slot: &mut f64,
        var_dd_dn6_slot: &mut f64,
        var_dd_dn7_slot: &mut f64,
        var_dd_dn8_slot: &mut f64,
        var_dd_rv_slot: &mut f64,
        var_delta_1s_dc_slot: &mut f64,
        var_delta_1s_dc_dn5_slot: &mut f64,
        var_delta_1s_dc_dn6_slot: &mut f64,
        var_delta_1s_dc_dn7_slot: &mut f64,
        var_delta_1s_dc_dn8_slot: &mut f64,
        var_delta_1s_dc_rv_slot: &mut f64,
        var_delta_ns_dc_slot: &mut f64,
        var_delta_ns_dc_dn5_slot: &mut f64,
        var_delta_ns_dc_dn6_slot: &mut f64,
        var_delta_ns_dc_dn7_slot: &mut f64,
        var_delta_ns_dc_dn8_slot: &mut f64,
        var_delta_ns_dc_rv_slot: &mut f64,
        var_dm_slot: &mut f64,
        var_dm_dn5_slot: &mut f64,
        var_dm_dn6_slot: &mut f64,
        var_dm_dn7_slot: &mut f64,
        var_dm_dn8_slot: &mut f64,
        var_dm_rv_slot: &mut f64,
        var_dps_slot: &mut f64,
        var_dps_dn5_slot: &mut f64,
        var_dps_dn6_slot: &mut f64,
        var_dps_dn7_slot: &mut f64,
        var_dps_dn8_slot: &mut f64,
        var_dps_rv_slot: &mut f64,
        var_ds_dc_slot: &mut f64,
        var_ds_dc_dn5_slot: &mut f64,
        var_ds_dc_dn6_slot: &mut f64,
        var_ds_dc_dn7_slot: &mut f64,
        var_ds_dc_dn8_slot: &mut f64,
        var_ds_dc_rv_slot: &mut f64,
        var_ed_slot: &mut f64,
        var_ed_dn5_slot: &mut f64,
        var_ed_dn6_slot: &mut f64,
        var_ed_dn7_slot: &mut f64,
        var_ed_dn8_slot: &mut f64,
        var_ed_rv_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_rv_slot: &mut f64,
        var_es_dc_slot: &mut f64,
        var_es_dc_dn5_slot: &mut f64,
        var_es_dc_dn6_slot: &mut f64,
        var_es_dc_dn7_slot: &mut f64,
        var_es_dc_dn8_slot: &mut f64,
        var_es_dc_rv_slot: &mut f64,
        var_eta_p_slot: &mut f64,
        var_eta_p_dn5_slot: &mut f64,
        var_eta_p_dn6_slot: &mut f64,
        var_eta_p_dn7_slot: &mut f64,
        var_eta_p_dn8_slot: &mut f64,
        var_eta_p_rv_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dc_slot: &mut f64,
        var_factheta_dc_dn5_slot: &mut f64,
        var_factheta_dc_dn6_slot: &mut f64,
        var_factheta_dc_dn7_slot: &mut f64,
        var_factheta_dc_dn8_slot: &mut f64,
        var_factheta_dc_rv_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_rv_slot: &mut f64,
        var_gf2_dc_slot: &mut f64,
        var_gf2_dc_dn5_slot: &mut f64,
        var_gf2_dc_dn6_slot: &mut f64,
        var_gf2_dc_dn7_slot: &mut f64,
        var_gf2_dc_dn8_slot: &mut f64,
        var_gf2_dc_rv_slot: &mut f64,
        var_gf_dc_slot: &mut f64,
        var_gf_dc_dn5_slot: &mut f64,
        var_gf_dc_dn6_slot: &mut f64,
        var_gf_dc_dn7_slot: &mut f64,
        var_gf_dc_dn8_slot: &mut f64,
        var_gf_dc_rv_slot: &mut f64,
        var_gmob_slot: &mut f64,
        var_gmob_dn5_slot: &mut f64,
        var_gmob_dn6_slot: &mut f64,
        var_gmob_dn7_slot: &mut f64,
        var_gmob_dn8_slot: &mut f64,
        var_gmob_rv_slot: &mut f64,
        var_gmobs_dc_slot: &mut f64,
        var_gmobs_dc_dn5_slot: &mut f64,
        var_gmobs_dc_dn6_slot: &mut f64,
        var_gmobs_dc_dn7_slot: &mut f64,
        var_gmobs_dc_dn8_slot: &mut f64,
        var_gmobs_dc_rv_slot: &mut f64,
        var_guard1197_slot: &mut f64,
        var_guard1197_rv_slot: &mut f64,
        var_guard1198_slot: &mut f64,
        var_guard1198_rv_slot: &mut f64,
        var_inv_gf2_dc_slot: &mut f64,
        var_inv_gf2_dc_dn5_slot: &mut f64,
        var_inv_gf2_dc_dn6_slot: &mut f64,
        var_inv_gf2_dc_dn7_slot: &mut f64,
        var_inv_gf2_dc_dn8_slot: &mut f64,
        var_inv_gf2_dc_rv_slot: &mut f64,
        var_inv_phit1_dc_slot: &mut f64,
        var_inv_phit1_dc_dn5_slot: &mut f64,
        var_inv_phit1_dc_dn6_slot: &mut f64,
        var_inv_phit1_dc_dn7_slot: &mut f64,
        var_inv_phit1_dc_dn8_slot: &mut f64,
        var_inv_phit1_dc_rv_slot: &mut f64,
        var_inv_xi_dc_slot: &mut f64,
        var_inv_xi_dc_dn5_slot: &mut f64,
        var_inv_xi_dc_dn6_slot: &mut f64,
        var_inv_xi_dc_dn7_slot: &mut f64,
        var_inv_xi_dc_dn8_slot: &mut f64,
        var_inv_xi_dc_rv_slot: &mut f64,
        var_margin_dc_slot: &mut f64,
        var_margin_dc_dn5_slot: &mut f64,
        var_margin_dc_dn6_slot: &mut f64,
        var_margin_dc_dn7_slot: &mut f64,
        var_margin_dc_dn8_slot: &mut f64,
        var_margin_dc_rv_slot: &mut f64,
        var_pd_slot: &mut f64,
        var_pd_dn5_slot: &mut f64,
        var_pd_dn6_slot: &mut f64,
        var_pd_dn7_slot: &mut f64,
        var_pd_dn8_slot: &mut f64,
        var_pd_rv_slot: &mut f64,
        var_phit1_dc_slot: &mut f64,
        var_phit1_dc_dn5_slot: &mut f64,
        var_phit1_dc_dn6_slot: &mut f64,
        var_phit1_dc_dn7_slot: &mut f64,
        var_phit1_dc_dn8_slot: &mut f64,
        var_phit1_dc_rv_slot: &mut f64,
        var_pm_slot: &mut f64,
        var_pm_dn5_slot: &mut f64,
        var_pm_dn6_slot: &mut f64,
        var_pm_dn7_slot: &mut f64,
        var_pm_dn8_slot: &mut f64,
        var_pm_rv_slot: &mut f64,
        var_ps_dc_slot: &mut f64,
        var_ps_dc_dn5_slot: &mut f64,
        var_ps_dc_dn6_slot: &mut f64,
        var_ps_dc_dn7_slot: &mut f64,
        var_ps_dc_dn8_slot: &mut f64,
        var_ps_dc_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn5_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_qbm_slot: &mut f64,
        var_qbm_dn5_slot: &mut f64,
        var_qbm_dn6_slot: &mut f64,
        var_qbm_dn7_slot: &mut f64,
        var_qbm_dn8_slot: &mut f64,
        var_qbm_rv_slot: &mut f64,
        var_qbs_dc_slot: &mut f64,
        var_qbs_dc_dn5_slot: &mut f64,
        var_qbs_dc_dn6_slot: &mut f64,
        var_qbs_dc_dn7_slot: &mut f64,
        var_qbs_dc_dn8_slot: &mut f64,
        var_qbs_dc_rv_slot: &mut f64,
        var_qeff1_slot: &mut f64,
        var_qeff1_dn5_slot: &mut f64,
        var_qeff1_dn6_slot: &mut f64,
        var_qeff1_dn7_slot: &mut f64,
        var_qeff1_dn8_slot: &mut f64,
        var_qeff1_rv_slot: &mut f64,
        var_qim_slot: &mut f64,
        var_qim1_slot: &mut f64,
        var_qim1_dn5_slot: &mut f64,
        var_qim1_dn6_slot: &mut f64,
        var_qim1_dn7_slot: &mut f64,
        var_qim1_dn8_slot: &mut f64,
        var_qim1_rv_slot: &mut f64,
        var_qim_dn5_slot: &mut f64,
        var_qim_dn6_slot: &mut f64,
        var_qim_dn7_slot: &mut f64,
        var_qim_dn8_slot: &mut f64,
        var_qim_rv_slot: &mut f64,
        var_qis_dc_slot: &mut f64,
        var_qis_dc_dn5_slot: &mut f64,
        var_qis_dc_dn6_slot: &mut f64,
        var_qis_dc_dn7_slot: &mut f64,
        var_qis_dc_dn8_slot: &mut f64,
        var_qis_dc_rv_slot: &mut f64,
        var_rhob_dc_slot: &mut f64,
        var_rhob_dc_dn5_slot: &mut f64,
        var_rhob_dc_dn6_slot: &mut f64,
        var_rhob_dc_dn7_slot: &mut f64,
        var_rhob_dc_dn8_slot: &mut f64,
        var_rhob_dc_rv_slot: &mut f64,
        var_rhog_dc_slot: &mut f64,
        var_rhog_dc_dn5_slot: &mut f64,
        var_rhog_dc_dn6_slot: &mut f64,
        var_rhog_dc_dn7_slot: &mut f64,
        var_rhog_dc_dn8_slot: &mut f64,
        var_rhog_dc_rv_slot: &mut f64,
        var_rxcor_dc_slot: &mut f64,
        var_rxcor_dc_dn5_slot: &mut f64,
        var_rxcor_dc_dn6_slot: &mut f64,
        var_rxcor_dc_dn7_slot: &mut f64,
        var_rxcor_dc_dn8_slot: &mut f64,
        var_rxcor_dc_rv_slot: &mut f64,
        var_s1_slot: &mut f64,
        var_s1_dn5_slot: &mut f64,
        var_s1_dn6_slot: &mut f64,
        var_s1_dn7_slot: &mut f64,
        var_s1_dn8_slot: &mut f64,
        var_s1_rv_slot: &mut f64,
        var_sp_s_x1_dc_slot: &mut f64,
        var_sp_s_x1_dc_dn5_slot: &mut f64,
        var_sp_s_x1_dc_dn6_slot: &mut f64,
        var_sp_s_x1_dc_dn7_slot: &mut f64,
        var_sp_s_x1_dc_dn8_slot: &mut f64,
        var_sp_s_x1_dc_rv_slot: &mut f64,
        var_sqm_slot: &mut f64,
        var_sqm_dn5_slot: &mut f64,
        var_sqm_dn6_slot: &mut f64,
        var_sqm_dn7_slot: &mut f64,
        var_sqm_dn8_slot: &mut f64,
        var_sqm_rv_slot: &mut f64,
        var_sqs_dc_slot: &mut f64,
        var_sqs_dc_dn5_slot: &mut f64,
        var_sqs_dc_dn6_slot: &mut f64,
        var_sqs_dc_dn7_slot: &mut f64,
        var_sqs_dc_dn8_slot: &mut f64,
        var_sqs_dc_rv_slot: &mut f64,
        var_thesat1_slot: &mut f64,
        var_thesat1_dn5_slot: &mut f64,
        var_thesat1_dn6_slot: &mut f64,
        var_thesat1_dn7_slot: &mut f64,
        var_thesat1_dn8_slot: &mut f64,
        var_thesat1_rv_slot: &mut f64,
        var_thesateff_slot: &mut f64,
        var_thesateff_dn5_slot: &mut f64,
        var_thesateff_dn6_slot: &mut f64,
        var_thesateff_dn7_slot: &mut f64,
        var_thesateff_dn8_slot: &mut f64,
        var_thesateff_rv_slot: &mut f64,
        var_udse_slot: &mut f64,
        var_udse_dn5_slot: &mut f64,
        var_udse_dn6_slot: &mut f64,
        var_udse_dn7_slot: &mut f64,
        var_udse_dn8_slot: &mut f64,
        var_udse_rv_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn5_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_v_dsat_rv_slot: &mut f64,
        var_vdsat_lim_slot: &mut f64,
        var_vdsat_lim_dn5_slot: &mut f64,
        var_vdsat_lim_dn6_slot: &mut f64,
        var_vdsat_lim_dn7_slot: &mut f64,
        var_vdsat_lim_dn8_slot: &mut f64,
        var_vdsat_lim_rv_slot: &mut f64,
        var_vdse_slot: &mut f64,
        var_vdse_dn5_slot: &mut f64,
        var_vdse_dn6_slot: &mut f64,
        var_vdse_dn7_slot: &mut f64,
        var_vdse_dn8_slot: &mut f64,
        var_vdse_rv_slot: &mut f64,
        var_vgb1_dc_slot: &mut f64,
        var_vgb1_dc_dn5_slot: &mut f64,
        var_vgb1_dc_dn6_slot: &mut f64,
        var_vgb1_dc_dn7_slot: &mut f64,
        var_vgb1_dc_dn8_slot: &mut f64,
        var_vgb1_dc_rv_slot: &mut f64,
        var_voxm_slot: &mut f64,
        var_voxm_dn5_slot: &mut f64,
        var_voxm_dn6_slot: &mut f64,
        var_voxm_dn7_slot: &mut f64,
        var_voxm_dn8_slot: &mut f64,
        var_voxm_rv_slot: &mut f64,
        var_vsbx_dc_slot: &mut f64,
        var_vsbx_dc_dn5_slot: &mut f64,
        var_vsbx_dc_dn6_slot: &mut f64,
        var_vsbx_dc_dn7_slot: &mut f64,
        var_vsbx_dc_dn8_slot: &mut f64,
        var_vsbx_dc_rv_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_d_rv_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn5_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_ds_rv_slot: &mut f64,
        var_x_m_slot: &mut f64,
        var_x_m_dn5_slot: &mut f64,
        var_x_m_dn6_slot: &mut f64,
        var_x_m_dn7_slot: &mut f64,
        var_x_m_dn8_slot: &mut f64,
        var_x_m_rv_slot: &mut f64,
        var_x_s_dc_slot: &mut f64,
        var_x_s_dc_dn5_slot: &mut f64,
        var_x_s_dc_dn6_slot: &mut f64,
        var_x_s_dc_dn7_slot: &mut f64,
        var_x_s_dc_dn8_slot: &mut f64,
        var_x_s_dc_rv_slot: &mut f64,
        var_xg_dc_slot: &mut f64,
        var_xg_dc_dn5_slot: &mut f64,
        var_xg_dc_dn6_slot: &mut f64,
        var_xg_dc_dn7_slot: &mut f64,
        var_xg_dc_dn8_slot: &mut f64,
        var_xg_dc_rv_slot: &mut f64,
        var_xgm_slot: &mut f64,
        var_xgm_dn5_slot: &mut f64,
        var_xgm_dn6_slot: &mut f64,
        var_xgm_dn7_slot: &mut f64,
        var_xgm_dn8_slot: &mut f64,
        var_xgm_rv_slot: &mut f64,
        var_xgs_dc_slot: &mut f64,
        var_xgs_dc_dn5_slot: &mut f64,
        var_xgs_dc_dn6_slot: &mut f64,
        var_xgs_dc_dn7_slot: &mut f64,
        var_xgs_dc_dn8_slot: &mut f64,
        var_xgs_dc_rv_slot: &mut f64,
        var_xi1s_dc_slot: &mut f64,
        var_xi1s_dc_dn5_slot: &mut f64,
        var_xi1s_dc_dn6_slot: &mut f64,
        var_xi1s_dc_dn7_slot: &mut f64,
        var_xi1s_dc_dn8_slot: &mut f64,
        var_xi1s_dc_rv_slot: &mut f64,
        var_xi2s_dc_slot: &mut f64,
        var_xi2s_dc_dn5_slot: &mut f64,
        var_xi2s_dc_dn6_slot: &mut f64,
        var_xi2s_dc_dn7_slot: &mut f64,
        var_xi2s_dc_dn8_slot: &mut f64,
        var_xi2s_dc_rv_slot: &mut f64,
        var_xi_dc_slot: &mut f64,
        var_xi_dc_dn5_slot: &mut f64,
        var_xi_dc_dn6_slot: &mut f64,
        var_xi_dc_dn7_slot: &mut f64,
        var_xi_dc_dn8_slot: &mut f64,
        var_xi_dc_rv_slot: &mut f64,
        var_xitsb_dc_slot: &mut f64,
        var_xitsb_dc_dn5_slot: &mut f64,
        var_xitsb_dc_dn6_slot: &mut f64,
        var_xitsb_dc_dn7_slot: &mut f64,
        var_xitsb_dc_dn8_slot: &mut f64,
        var_xitsb_dc_rv_slot: &mut f64,
        var_xn_s_dc_slot: &mut f64,
        var_xn_s_dc_dn5_slot: &mut f64,
        var_xn_s_dc_dn6_slot: &mut f64,
        var_xn_s_dc_dn7_slot: &mut f64,
        var_xn_s_dc_dn8_slot: &mut f64,
        var_xn_s_dc_rv_slot: &mut f64,
        var_xno_s_dc_slot: &mut f64,
        var_xno_s_dc_dn5_slot: &mut f64,
        var_xno_s_dc_dn6_slot: &mut f64,
        var_xno_s_dc_dn7_slot: &mut f64,
        var_xno_s_dc_dn8_slot: &mut f64,
        var_xno_s_dc_rv_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn5: f64 = *var_alpha_dn5_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alpha_rv: f64 = *var_alpha_rv_slot;
        let mut var_alphas_dc: f64 = *var_alphas_dc_slot;
        let mut var_alphas_dc_dn5: f64 = *var_alphas_dc_dn5_slot;
        let mut var_alphas_dc_dn6: f64 = *var_alphas_dc_dn6_slot;
        let mut var_alphas_dc_dn7: f64 = *var_alphas_dc_dn7_slot;
        let mut var_alphas_dc_dn8: f64 = *var_alphas_dc_dn8_slot;
        let mut var_alphas_dc_rv: f64 = *var_alphas_dc_rv_slot;
        let mut var_asat: f64 = *var_asat_slot;
        let mut var_asat_dn5: f64 = *var_asat_dn5_slot;
        let mut var_asat_dn6: f64 = *var_asat_dn6_slot;
        let mut var_asat_dn7: f64 = *var_asat_dn7_slot;
        let mut var_asat_dn8: f64 = *var_asat_dn8_slot;
        let mut var_asat_rv: f64 = *var_asat_rv_slot;
        let mut var_dd: f64 = *var_dd_slot;
        let mut var_dd_dn5: f64 = *var_dd_dn5_slot;
        let mut var_dd_dn6: f64 = *var_dd_dn6_slot;
        let mut var_dd_dn7: f64 = *var_dd_dn7_slot;
        let mut var_dd_dn8: f64 = *var_dd_dn8_slot;
        let mut var_dd_rv: f64 = *var_dd_rv_slot;
        let mut var_delta_1s_dc: f64 = *var_delta_1s_dc_slot;
        let mut var_delta_1s_dc_dn5: f64 = *var_delta_1s_dc_dn5_slot;
        let mut var_delta_1s_dc_dn6: f64 = *var_delta_1s_dc_dn6_slot;
        let mut var_delta_1s_dc_dn7: f64 = *var_delta_1s_dc_dn7_slot;
        let mut var_delta_1s_dc_dn8: f64 = *var_delta_1s_dc_dn8_slot;
        let mut var_delta_1s_dc_rv: f64 = *var_delta_1s_dc_rv_slot;
        let mut var_delta_ns_dc: f64 = *var_delta_ns_dc_slot;
        let mut var_delta_ns_dc_dn5: f64 = *var_delta_ns_dc_dn5_slot;
        let mut var_delta_ns_dc_dn6: f64 = *var_delta_ns_dc_dn6_slot;
        let mut var_delta_ns_dc_dn7: f64 = *var_delta_ns_dc_dn7_slot;
        let mut var_delta_ns_dc_dn8: f64 = *var_delta_ns_dc_dn8_slot;
        let mut var_delta_ns_dc_rv: f64 = *var_delta_ns_dc_rv_slot;
        let mut var_dm: f64 = *var_dm_slot;
        let mut var_dm_dn5: f64 = *var_dm_dn5_slot;
        let mut var_dm_dn6: f64 = *var_dm_dn6_slot;
        let mut var_dm_dn7: f64 = *var_dm_dn7_slot;
        let mut var_dm_dn8: f64 = *var_dm_dn8_slot;
        let mut var_dm_rv: f64 = *var_dm_rv_slot;
        let mut var_dps: f64 = *var_dps_slot;
        let mut var_dps_dn5: f64 = *var_dps_dn5_slot;
        let mut var_dps_dn6: f64 = *var_dps_dn6_slot;
        let mut var_dps_dn7: f64 = *var_dps_dn7_slot;
        let mut var_dps_dn8: f64 = *var_dps_dn8_slot;
        let mut var_dps_rv: f64 = *var_dps_rv_slot;
        let mut var_ds_dc: f64 = *var_ds_dc_slot;
        let mut var_ds_dc_dn5: f64 = *var_ds_dc_dn5_slot;
        let mut var_ds_dc_dn6: f64 = *var_ds_dc_dn6_slot;
        let mut var_ds_dc_dn7: f64 = *var_ds_dc_dn7_slot;
        let mut var_ds_dc_dn8: f64 = *var_ds_dc_dn8_slot;
        let mut var_ds_dc_rv: f64 = *var_ds_dc_rv_slot;
        let mut var_ed: f64 = *var_ed_slot;
        let mut var_ed_dn5: f64 = *var_ed_dn5_slot;
        let mut var_ed_dn6: f64 = *var_ed_dn6_slot;
        let mut var_ed_dn7: f64 = *var_ed_dn7_slot;
        let mut var_ed_dn8: f64 = *var_ed_dn8_slot;
        let mut var_ed_rv: f64 = *var_ed_rv_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_rv: f64 = *var_em_rv_slot;
        let mut var_es_dc: f64 = *var_es_dc_slot;
        let mut var_es_dc_dn5: f64 = *var_es_dc_dn5_slot;
        let mut var_es_dc_dn6: f64 = *var_es_dc_dn6_slot;
        let mut var_es_dc_dn7: f64 = *var_es_dc_dn7_slot;
        let mut var_es_dc_dn8: f64 = *var_es_dc_dn8_slot;
        let mut var_es_dc_rv: f64 = *var_es_dc_rv_slot;
        let mut var_eta_p: f64 = *var_eta_p_slot;
        let mut var_eta_p_dn5: f64 = *var_eta_p_dn5_slot;
        let mut var_eta_p_dn6: f64 = *var_eta_p_dn6_slot;
        let mut var_eta_p_dn7: f64 = *var_eta_p_dn7_slot;
        let mut var_eta_p_dn8: f64 = *var_eta_p_dn8_slot;
        let mut var_eta_p_rv: f64 = *var_eta_p_rv_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dc: f64 = *var_factheta_dc_slot;
        let mut var_factheta_dc_dn5: f64 = *var_factheta_dc_dn5_slot;
        let mut var_factheta_dc_dn6: f64 = *var_factheta_dc_dn6_slot;
        let mut var_factheta_dc_dn7: f64 = *var_factheta_dc_dn7_slot;
        let mut var_factheta_dc_dn8: f64 = *var_factheta_dc_dn8_slot;
        let mut var_factheta_dc_rv: f64 = *var_factheta_dc_rv_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_rv: f64 = *var_factheta_rv_slot;
        let mut var_gf2_dc: f64 = *var_gf2_dc_slot;
        let mut var_gf2_dc_dn5: f64 = *var_gf2_dc_dn5_slot;
        let mut var_gf2_dc_dn6: f64 = *var_gf2_dc_dn6_slot;
        let mut var_gf2_dc_dn7: f64 = *var_gf2_dc_dn7_slot;
        let mut var_gf2_dc_dn8: f64 = *var_gf2_dc_dn8_slot;
        let mut var_gf2_dc_rv: f64 = *var_gf2_dc_rv_slot;
        let mut var_gf_dc: f64 = *var_gf_dc_slot;
        let mut var_gf_dc_dn5: f64 = *var_gf_dc_dn5_slot;
        let mut var_gf_dc_dn6: f64 = *var_gf_dc_dn6_slot;
        let mut var_gf_dc_dn7: f64 = *var_gf_dc_dn7_slot;
        let mut var_gf_dc_dn8: f64 = *var_gf_dc_dn8_slot;
        let mut var_gf_dc_rv: f64 = *var_gf_dc_rv_slot;
        let mut var_gmob: f64 = *var_gmob_slot;
        let mut var_gmob_dn5: f64 = *var_gmob_dn5_slot;
        let mut var_gmob_dn6: f64 = *var_gmob_dn6_slot;
        let mut var_gmob_dn7: f64 = *var_gmob_dn7_slot;
        let mut var_gmob_dn8: f64 = *var_gmob_dn8_slot;
        let mut var_gmob_rv: f64 = *var_gmob_rv_slot;
        let mut var_gmobs_dc: f64 = *var_gmobs_dc_slot;
        let mut var_gmobs_dc_dn5: f64 = *var_gmobs_dc_dn5_slot;
        let mut var_gmobs_dc_dn6: f64 = *var_gmobs_dc_dn6_slot;
        let mut var_gmobs_dc_dn7: f64 = *var_gmobs_dc_dn7_slot;
        let mut var_gmobs_dc_dn8: f64 = *var_gmobs_dc_dn8_slot;
        let mut var_gmobs_dc_rv: f64 = *var_gmobs_dc_rv_slot;
        let mut var_guard1197: f64 = *var_guard1197_slot;
        let mut var_guard1197_rv: f64 = *var_guard1197_rv_slot;
        let mut var_guard1198: f64 = *var_guard1198_slot;
        let mut var_guard1198_rv: f64 = *var_guard1198_rv_slot;
        let mut var_inv_gf2_dc: f64 = *var_inv_gf2_dc_slot;
        let mut var_inv_gf2_dc_dn5: f64 = *var_inv_gf2_dc_dn5_slot;
        let mut var_inv_gf2_dc_dn6: f64 = *var_inv_gf2_dc_dn6_slot;
        let mut var_inv_gf2_dc_dn7: f64 = *var_inv_gf2_dc_dn7_slot;
        let mut var_inv_gf2_dc_dn8: f64 = *var_inv_gf2_dc_dn8_slot;
        let mut var_inv_gf2_dc_rv: f64 = *var_inv_gf2_dc_rv_slot;
        let mut var_inv_phit1_dc: f64 = *var_inv_phit1_dc_slot;
        let mut var_inv_phit1_dc_dn5: f64 = *var_inv_phit1_dc_dn5_slot;
        let mut var_inv_phit1_dc_dn6: f64 = *var_inv_phit1_dc_dn6_slot;
        let mut var_inv_phit1_dc_dn7: f64 = *var_inv_phit1_dc_dn7_slot;
        let mut var_inv_phit1_dc_dn8: f64 = *var_inv_phit1_dc_dn8_slot;
        let mut var_inv_phit1_dc_rv: f64 = *var_inv_phit1_dc_rv_slot;
        let mut var_inv_xi_dc: f64 = *var_inv_xi_dc_slot;
        let mut var_inv_xi_dc_dn5: f64 = *var_inv_xi_dc_dn5_slot;
        let mut var_inv_xi_dc_dn6: f64 = *var_inv_xi_dc_dn6_slot;
        let mut var_inv_xi_dc_dn7: f64 = *var_inv_xi_dc_dn7_slot;
        let mut var_inv_xi_dc_dn8: f64 = *var_inv_xi_dc_dn8_slot;
        let mut var_inv_xi_dc_rv: f64 = *var_inv_xi_dc_rv_slot;
        let mut var_margin_dc: f64 = *var_margin_dc_slot;
        let mut var_margin_dc_dn5: f64 = *var_margin_dc_dn5_slot;
        let mut var_margin_dc_dn6: f64 = *var_margin_dc_dn6_slot;
        let mut var_margin_dc_dn7: f64 = *var_margin_dc_dn7_slot;
        let mut var_margin_dc_dn8: f64 = *var_margin_dc_dn8_slot;
        let mut var_margin_dc_rv: f64 = *var_margin_dc_rv_slot;
        let mut var_pd: f64 = *var_pd_slot;
        let mut var_pd_dn5: f64 = *var_pd_dn5_slot;
        let mut var_pd_dn6: f64 = *var_pd_dn6_slot;
        let mut var_pd_dn7: f64 = *var_pd_dn7_slot;
        let mut var_pd_dn8: f64 = *var_pd_dn8_slot;
        let mut var_pd_rv: f64 = *var_pd_rv_slot;
        let mut var_phit1_dc: f64 = *var_phit1_dc_slot;
        let mut var_phit1_dc_dn5: f64 = *var_phit1_dc_dn5_slot;
        let mut var_phit1_dc_dn6: f64 = *var_phit1_dc_dn6_slot;
        let mut var_phit1_dc_dn7: f64 = *var_phit1_dc_dn7_slot;
        let mut var_phit1_dc_dn8: f64 = *var_phit1_dc_dn8_slot;
        let mut var_phit1_dc_rv: f64 = *var_phit1_dc_rv_slot;
        let mut var_pm: f64 = *var_pm_slot;
        let mut var_pm_dn5: f64 = *var_pm_dn5_slot;
        let mut var_pm_dn6: f64 = *var_pm_dn6_slot;
        let mut var_pm_dn7: f64 = *var_pm_dn7_slot;
        let mut var_pm_dn8: f64 = *var_pm_dn8_slot;
        let mut var_pm_rv: f64 = *var_pm_rv_slot;
        let mut var_ps_dc: f64 = *var_ps_dc_slot;
        let mut var_ps_dc_dn5: f64 = *var_ps_dc_dn5_slot;
        let mut var_ps_dc_dn6: f64 = *var_ps_dc_dn6_slot;
        let mut var_ps_dc_dn7: f64 = *var_ps_dc_dn7_slot;
        let mut var_ps_dc_dn8: f64 = *var_ps_dc_dn8_slot;
        let mut var_ps_dc_rv: f64 = *var_ps_dc_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn5: f64 = *var_qbd_dn5_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_qbm: f64 = *var_qbm_slot;
        let mut var_qbm_dn5: f64 = *var_qbm_dn5_slot;
        let mut var_qbm_dn6: f64 = *var_qbm_dn6_slot;
        let mut var_qbm_dn7: f64 = *var_qbm_dn7_slot;
        let mut var_qbm_dn8: f64 = *var_qbm_dn8_slot;
        let mut var_qbm_rv: f64 = *var_qbm_rv_slot;
        let mut var_qbs_dc: f64 = *var_qbs_dc_slot;
        let mut var_qbs_dc_dn5: f64 = *var_qbs_dc_dn5_slot;
        let mut var_qbs_dc_dn6: f64 = *var_qbs_dc_dn6_slot;
        let mut var_qbs_dc_dn7: f64 = *var_qbs_dc_dn7_slot;
        let mut var_qbs_dc_dn8: f64 = *var_qbs_dc_dn8_slot;
        let mut var_qbs_dc_rv: f64 = *var_qbs_dc_rv_slot;
        let mut var_qeff1: f64 = *var_qeff1_slot;
        let mut var_qeff1_dn5: f64 = *var_qeff1_dn5_slot;
        let mut var_qeff1_dn6: f64 = *var_qeff1_dn6_slot;
        let mut var_qeff1_dn7: f64 = *var_qeff1_dn7_slot;
        let mut var_qeff1_dn8: f64 = *var_qeff1_dn8_slot;
        let mut var_qeff1_rv: f64 = *var_qeff1_rv_slot;
        let mut var_qim: f64 = *var_qim_slot;
        let mut var_qim1: f64 = *var_qim1_slot;
        let mut var_qim1_dn5: f64 = *var_qim1_dn5_slot;
        let mut var_qim1_dn6: f64 = *var_qim1_dn6_slot;
        let mut var_qim1_dn7: f64 = *var_qim1_dn7_slot;
        let mut var_qim1_dn8: f64 = *var_qim1_dn8_slot;
        let mut var_qim1_rv: f64 = *var_qim1_rv_slot;
        let mut var_qim_dn5: f64 = *var_qim_dn5_slot;
        let mut var_qim_dn6: f64 = *var_qim_dn6_slot;
        let mut var_qim_dn7: f64 = *var_qim_dn7_slot;
        let mut var_qim_dn8: f64 = *var_qim_dn8_slot;
        let mut var_qim_rv: f64 = *var_qim_rv_slot;
        let mut var_qis_dc: f64 = *var_qis_dc_slot;
        let mut var_qis_dc_dn5: f64 = *var_qis_dc_dn5_slot;
        let mut var_qis_dc_dn6: f64 = *var_qis_dc_dn6_slot;
        let mut var_qis_dc_dn7: f64 = *var_qis_dc_dn7_slot;
        let mut var_qis_dc_dn8: f64 = *var_qis_dc_dn8_slot;
        let mut var_qis_dc_rv: f64 = *var_qis_dc_rv_slot;
        let mut var_rhob_dc: f64 = *var_rhob_dc_slot;
        let mut var_rhob_dc_dn5: f64 = *var_rhob_dc_dn5_slot;
        let mut var_rhob_dc_dn6: f64 = *var_rhob_dc_dn6_slot;
        let mut var_rhob_dc_dn7: f64 = *var_rhob_dc_dn7_slot;
        let mut var_rhob_dc_dn8: f64 = *var_rhob_dc_dn8_slot;
        let mut var_rhob_dc_rv: f64 = *var_rhob_dc_rv_slot;
        let mut var_rhog_dc: f64 = *var_rhog_dc_slot;
        let mut var_rhog_dc_dn5: f64 = *var_rhog_dc_dn5_slot;
        let mut var_rhog_dc_dn6: f64 = *var_rhog_dc_dn6_slot;
        let mut var_rhog_dc_dn7: f64 = *var_rhog_dc_dn7_slot;
        let mut var_rhog_dc_dn8: f64 = *var_rhog_dc_dn8_slot;
        let mut var_rhog_dc_rv: f64 = *var_rhog_dc_rv_slot;
        let mut var_rxcor_dc: f64 = *var_rxcor_dc_slot;
        let mut var_rxcor_dc_dn5: f64 = *var_rxcor_dc_dn5_slot;
        let mut var_rxcor_dc_dn6: f64 = *var_rxcor_dc_dn6_slot;
        let mut var_rxcor_dc_dn7: f64 = *var_rxcor_dc_dn7_slot;
        let mut var_rxcor_dc_dn8: f64 = *var_rxcor_dc_dn8_slot;
        let mut var_rxcor_dc_rv: f64 = *var_rxcor_dc_rv_slot;
        let mut var_s1: f64 = *var_s1_slot;
        let mut var_s1_dn5: f64 = *var_s1_dn5_slot;
        let mut var_s1_dn6: f64 = *var_s1_dn6_slot;
        let mut var_s1_dn7: f64 = *var_s1_dn7_slot;
        let mut var_s1_dn8: f64 = *var_s1_dn8_slot;
        let mut var_s1_rv: f64 = *var_s1_rv_slot;
        let mut var_sp_s_x1_dc: f64 = *var_sp_s_x1_dc_slot;
        let mut var_sp_s_x1_dc_dn5: f64 = *var_sp_s_x1_dc_dn5_slot;
        let mut var_sp_s_x1_dc_dn6: f64 = *var_sp_s_x1_dc_dn6_slot;
        let mut var_sp_s_x1_dc_dn7: f64 = *var_sp_s_x1_dc_dn7_slot;
        let mut var_sp_s_x1_dc_dn8: f64 = *var_sp_s_x1_dc_dn8_slot;
        let mut var_sp_s_x1_dc_rv: f64 = *var_sp_s_x1_dc_rv_slot;
        let mut var_sqm: f64 = *var_sqm_slot;
        let mut var_sqm_dn5: f64 = *var_sqm_dn5_slot;
        let mut var_sqm_dn6: f64 = *var_sqm_dn6_slot;
        let mut var_sqm_dn7: f64 = *var_sqm_dn7_slot;
        let mut var_sqm_dn8: f64 = *var_sqm_dn8_slot;
        let mut var_sqm_rv: f64 = *var_sqm_rv_slot;
        let mut var_sqs_dc: f64 = *var_sqs_dc_slot;
        let mut var_sqs_dc_dn5: f64 = *var_sqs_dc_dn5_slot;
        let mut var_sqs_dc_dn6: f64 = *var_sqs_dc_dn6_slot;
        let mut var_sqs_dc_dn7: f64 = *var_sqs_dc_dn7_slot;
        let mut var_sqs_dc_dn8: f64 = *var_sqs_dc_dn8_slot;
        let mut var_sqs_dc_rv: f64 = *var_sqs_dc_rv_slot;
        let mut var_thesat1: f64 = *var_thesat1_slot;
        let mut var_thesat1_dn5: f64 = *var_thesat1_dn5_slot;
        let mut var_thesat1_dn6: f64 = *var_thesat1_dn6_slot;
        let mut var_thesat1_dn7: f64 = *var_thesat1_dn7_slot;
        let mut var_thesat1_dn8: f64 = *var_thesat1_dn8_slot;
        let mut var_thesat1_rv: f64 = *var_thesat1_rv_slot;
        let mut var_thesateff: f64 = *var_thesateff_slot;
        let mut var_thesateff_dn5: f64 = *var_thesateff_dn5_slot;
        let mut var_thesateff_dn6: f64 = *var_thesateff_dn6_slot;
        let mut var_thesateff_dn7: f64 = *var_thesateff_dn7_slot;
        let mut var_thesateff_dn8: f64 = *var_thesateff_dn8_slot;
        let mut var_thesateff_rv: f64 = *var_thesateff_rv_slot;
        let mut var_udse: f64 = *var_udse_slot;
        let mut var_udse_dn5: f64 = *var_udse_dn5_slot;
        let mut var_udse_dn6: f64 = *var_udse_dn6_slot;
        let mut var_udse_dn7: f64 = *var_udse_dn7_slot;
        let mut var_udse_dn8: f64 = *var_udse_dn8_slot;
        let mut var_udse_rv: f64 = *var_udse_rv_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn5: f64 = *var_v_dsat_dn5_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_v_dsat_rv: f64 = *var_v_dsat_rv_slot;
        let mut var_vdsat_lim: f64 = *var_vdsat_lim_slot;
        let mut var_vdsat_lim_dn5: f64 = *var_vdsat_lim_dn5_slot;
        let mut var_vdsat_lim_dn6: f64 = *var_vdsat_lim_dn6_slot;
        let mut var_vdsat_lim_dn7: f64 = *var_vdsat_lim_dn7_slot;
        let mut var_vdsat_lim_dn8: f64 = *var_vdsat_lim_dn8_slot;
        let mut var_vdsat_lim_rv: f64 = *var_vdsat_lim_rv_slot;
        let mut var_vdse: f64 = *var_vdse_slot;
        let mut var_vdse_dn5: f64 = *var_vdse_dn5_slot;
        let mut var_vdse_dn6: f64 = *var_vdse_dn6_slot;
        let mut var_vdse_dn7: f64 = *var_vdse_dn7_slot;
        let mut var_vdse_dn8: f64 = *var_vdse_dn8_slot;
        let mut var_vdse_rv: f64 = *var_vdse_rv_slot;
        let mut var_vgb1_dc: f64 = *var_vgb1_dc_slot;
        let mut var_vgb1_dc_dn5: f64 = *var_vgb1_dc_dn5_slot;
        let mut var_vgb1_dc_dn6: f64 = *var_vgb1_dc_dn6_slot;
        let mut var_vgb1_dc_dn7: f64 = *var_vgb1_dc_dn7_slot;
        let mut var_vgb1_dc_dn8: f64 = *var_vgb1_dc_dn8_slot;
        let mut var_vgb1_dc_rv: f64 = *var_vgb1_dc_rv_slot;
        let mut var_voxm: f64 = *var_voxm_slot;
        let mut var_voxm_dn5: f64 = *var_voxm_dn5_slot;
        let mut var_voxm_dn6: f64 = *var_voxm_dn6_slot;
        let mut var_voxm_dn7: f64 = *var_voxm_dn7_slot;
        let mut var_voxm_dn8: f64 = *var_voxm_dn8_slot;
        let mut var_voxm_rv: f64 = *var_voxm_rv_slot;
        let mut var_vsbx_dc: f64 = *var_vsbx_dc_slot;
        let mut var_vsbx_dc_dn5: f64 = *var_vsbx_dc_dn5_slot;
        let mut var_vsbx_dc_dn6: f64 = *var_vsbx_dc_dn6_slot;
        let mut var_vsbx_dc_dn7: f64 = *var_vsbx_dc_dn7_slot;
        let mut var_vsbx_dc_dn8: f64 = *var_vsbx_dc_dn8_slot;
        let mut var_vsbx_dc_rv: f64 = *var_vsbx_dc_rv_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_d_rv: f64 = *var_x_d_rv_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn5: f64 = *var_x_ds_dn5_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_ds_rv: f64 = *var_x_ds_rv_slot;
        let mut var_x_m: f64 = *var_x_m_slot;
        let mut var_x_m_dn5: f64 = *var_x_m_dn5_slot;
        let mut var_x_m_dn6: f64 = *var_x_m_dn6_slot;
        let mut var_x_m_dn7: f64 = *var_x_m_dn7_slot;
        let mut var_x_m_dn8: f64 = *var_x_m_dn8_slot;
        let mut var_x_m_rv: f64 = *var_x_m_rv_slot;
        let mut var_x_s_dc: f64 = *var_x_s_dc_slot;
        let mut var_x_s_dc_dn5: f64 = *var_x_s_dc_dn5_slot;
        let mut var_x_s_dc_dn6: f64 = *var_x_s_dc_dn6_slot;
        let mut var_x_s_dc_dn7: f64 = *var_x_s_dc_dn7_slot;
        let mut var_x_s_dc_dn8: f64 = *var_x_s_dc_dn8_slot;
        let mut var_x_s_dc_rv: f64 = *var_x_s_dc_rv_slot;
        let mut var_xg_dc: f64 = *var_xg_dc_slot;
        let mut var_xg_dc_dn5: f64 = *var_xg_dc_dn5_slot;
        let mut var_xg_dc_dn6: f64 = *var_xg_dc_dn6_slot;
        let mut var_xg_dc_dn7: f64 = *var_xg_dc_dn7_slot;
        let mut var_xg_dc_dn8: f64 = *var_xg_dc_dn8_slot;
        let mut var_xg_dc_rv: f64 = *var_xg_dc_rv_slot;
        let mut var_xgm: f64 = *var_xgm_slot;
        let mut var_xgm_dn5: f64 = *var_xgm_dn5_slot;
        let mut var_xgm_dn6: f64 = *var_xgm_dn6_slot;
        let mut var_xgm_dn7: f64 = *var_xgm_dn7_slot;
        let mut var_xgm_dn8: f64 = *var_xgm_dn8_slot;
        let mut var_xgm_rv: f64 = *var_xgm_rv_slot;
        let mut var_xgs_dc: f64 = *var_xgs_dc_slot;
        let mut var_xgs_dc_dn5: f64 = *var_xgs_dc_dn5_slot;
        let mut var_xgs_dc_dn6: f64 = *var_xgs_dc_dn6_slot;
        let mut var_xgs_dc_dn7: f64 = *var_xgs_dc_dn7_slot;
        let mut var_xgs_dc_dn8: f64 = *var_xgs_dc_dn8_slot;
        let mut var_xgs_dc_rv: f64 = *var_xgs_dc_rv_slot;
        let mut var_xi1s_dc: f64 = *var_xi1s_dc_slot;
        let mut var_xi1s_dc_dn5: f64 = *var_xi1s_dc_dn5_slot;
        let mut var_xi1s_dc_dn6: f64 = *var_xi1s_dc_dn6_slot;
        let mut var_xi1s_dc_dn7: f64 = *var_xi1s_dc_dn7_slot;
        let mut var_xi1s_dc_dn8: f64 = *var_xi1s_dc_dn8_slot;
        let mut var_xi1s_dc_rv: f64 = *var_xi1s_dc_rv_slot;
        let mut var_xi2s_dc: f64 = *var_xi2s_dc_slot;
        let mut var_xi2s_dc_dn5: f64 = *var_xi2s_dc_dn5_slot;
        let mut var_xi2s_dc_dn6: f64 = *var_xi2s_dc_dn6_slot;
        let mut var_xi2s_dc_dn7: f64 = *var_xi2s_dc_dn7_slot;
        let mut var_xi2s_dc_dn8: f64 = *var_xi2s_dc_dn8_slot;
        let mut var_xi2s_dc_rv: f64 = *var_xi2s_dc_rv_slot;
        let mut var_xi_dc: f64 = *var_xi_dc_slot;
        let mut var_xi_dc_dn5: f64 = *var_xi_dc_dn5_slot;
        let mut var_xi_dc_dn6: f64 = *var_xi_dc_dn6_slot;
        let mut var_xi_dc_dn7: f64 = *var_xi_dc_dn7_slot;
        let mut var_xi_dc_dn8: f64 = *var_xi_dc_dn8_slot;
        let mut var_xi_dc_rv: f64 = *var_xi_dc_rv_slot;
        let mut var_xitsb_dc: f64 = *var_xitsb_dc_slot;
        let mut var_xitsb_dc_dn5: f64 = *var_xitsb_dc_dn5_slot;
        let mut var_xitsb_dc_dn6: f64 = *var_xitsb_dc_dn6_slot;
        let mut var_xitsb_dc_dn7: f64 = *var_xitsb_dc_dn7_slot;
        let mut var_xitsb_dc_dn8: f64 = *var_xitsb_dc_dn8_slot;
        let mut var_xitsb_dc_rv: f64 = *var_xitsb_dc_rv_slot;
        let mut var_xn_s_dc: f64 = *var_xn_s_dc_slot;
        let mut var_xn_s_dc_dn5: f64 = *var_xn_s_dc_dn5_slot;
        let mut var_xn_s_dc_dn6: f64 = *var_xn_s_dc_dn6_slot;
        let mut var_xn_s_dc_dn7: f64 = *var_xn_s_dc_dn7_slot;
        let mut var_xn_s_dc_dn8: f64 = *var_xn_s_dc_dn8_slot;
        let mut var_xn_s_dc_rv: f64 = *var_xn_s_dc_rv_slot;
        let mut var_xno_s_dc: f64 = *var_xno_s_dc_slot;
        let mut var_xno_s_dc_dn5: f64 = *var_xno_s_dc_dn5_slot;
        let mut var_xno_s_dc_dn6: f64 = *var_xno_s_dc_dn6_slot;
        let mut var_xno_s_dc_dn7: f64 = *var_xno_s_dc_dn7_slot;
        let mut var_xno_s_dc_dn8: f64 = *var_xno_s_dc_dn8_slot;
        let mut var_xno_s_dc_rv: f64 = *var_xno_s_dc_rv_slot;

        let (assign42950_e56247, assign42950_e56247_d_n5, assign42950_e56247_d_n6, assign42950_e56247_d_n7, assign42950_e56247_d_n8,) = {
    if (((var_guard1188 != 0.0) && (var_guard1192 != 0.0)) && (var_guard1196 == 0.0)) {
        let assign42950_e56244: f64 = (var_thesatg_i * var_wsat);
        let assign42950_e56245: f64 = (1.0 + assign42950_e56244);
        (assign42950_e56245, (var_thesatg_i * var_wsat_dn5), (var_thesatg_i * var_wsat_dn6), (var_thesatg_i * var_wsat_dn7), (var_thesatg_i * var_wsat_dn8),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign42950_e56247;
        var_factheta_dn5 = assign42950_e56247_d_n5;
        var_factheta_dn6 = assign42950_e56247_d_n6;
        var_factheta_dn7 = assign42950_e56247_d_n7;
        var_factheta_dn8 = assign42950_e56247_d_n8;
        var_factheta_rv = 0.0;

        var_vgb1_dc = var_vgb1;
        var_vgb1_dc_dn5 = var_vgb1_dn5;
        var_vgb1_dc_dn6 = var_vgb1_dn6;
        var_vgb1_dc_dn7 = var_vgb1_dn7;
        var_vgb1_dc_dn8 = var_vgb1_dn8;
        var_vgb1_dc_rv = 0.0;

        var_vsbx_dc = var_vsbx;
        var_vsbx_dc_dn5 = var_vsbx_dn5;
        var_vsbx_dc_dn6 = var_vsbx_dn6;
        var_vsbx_dc_dn7 = var_vsbx_dn7;
        var_vsbx_dc_dn8 = var_vsbx_dn8;
        var_vsbx_dc_rv = 0.0;

        var_phit1_dc = var_phit1;
        var_phit1_dc_dn5 = var_phit1_dn5;
        var_phit1_dc_dn6 = var_phit1_dn6;
        var_phit1_dc_dn7 = var_phit1_dn7;
        var_phit1_dc_dn8 = var_phit1_dn8;
        var_phit1_dc_rv = 0.0;

        var_inv_phit1_dc = var_inv_phit1;
        var_inv_phit1_dc_dn5 = var_inv_phit1_dn5;
        var_inv_phit1_dc_dn6 = var_inv_phit1_dn6;
        var_inv_phit1_dc_dn7 = var_inv_phit1_dn7;
        var_inv_phit1_dc_dn8 = var_inv_phit1_dn8;
        var_inv_phit1_dc_rv = 0.0;

        var_gf_dc = var_gf;
        var_gf_dc_dn5 = var_gf_dn5;
        var_gf_dc_dn6 = var_gf_dn6;
        var_gf_dc_dn7 = var_gf_dn7;
        var_gf_dc_dn8 = var_gf_dn8;
        var_gf_dc_rv = 0.0;

        var_gf2_dc = var_gf2;
        var_gf2_dc_dn5 = var_gf2_dn5;
        var_gf2_dc_dn6 = var_gf2_dn6;
        var_gf2_dc_dn7 = var_gf2_dn7;
        var_gf2_dc_dn8 = var_gf2_dn8;
        var_gf2_dc_rv = 0.0;

        var_inv_gf2_dc = var_inv_gf2;
        var_inv_gf2_dc_dn5 = var_inv_gf2_dn5;
        var_inv_gf2_dc_dn6 = var_inv_gf2_dn6;
        var_inv_gf2_dc_dn7 = var_inv_gf2_dn7;
        var_inv_gf2_dc_dn8 = var_inv_gf2_dn8;
        var_inv_gf2_dc_rv = 0.0;

        var_xg_dc = var_xg;
        var_xg_dc_dn5 = var_xg_dn5;
        var_xg_dc_dn6 = var_xg_dn6;
        var_xg_dc_dn7 = var_xg_dn7;
        var_xg_dc_dn8 = var_xg_dn8;
        var_xg_dc_rv = 0.0;

        var_xno_s_dc = var_xno_s;
        var_xno_s_dc_dn5 = var_xno_s_dn5;
        var_xno_s_dc_dn6 = var_xno_s_dn6;
        var_xno_s_dc_dn7 = var_xno_s_dn7;
        var_xno_s_dc_dn8 = var_xno_s_dn8;
        var_xno_s_dc_rv = 0.0;

        var_xn_s_dc = var_xn_s;
        var_xn_s_dc_dn5 = var_xn_s_dn5;
        var_xn_s_dc_dn6 = var_xn_s_dn6;
        var_xn_s_dc_dn7 = var_xn_s_dn7;
        var_xn_s_dc_dn8 = var_xn_s_dn8;
        var_xn_s_dc_rv = 0.0;

        var_xi_dc = var_xi;
        var_xi_dc_dn5 = var_xi_dn5;
        var_xi_dc_dn6 = var_xi_dn6;
        var_xi_dc_dn7 = var_xi_dn7;
        var_xi_dc_dn8 = var_xi_dn8;
        var_xi_dc_rv = 0.0;

        var_margin_dc = var_margin;
        var_margin_dc_dn5 = var_margin_dn5;
        var_margin_dc_dn6 = var_margin_dn6;
        var_margin_dc_dn7 = var_margin_dn7;
        var_margin_dc_dn8 = var_margin_dn8;
        var_margin_dc_rv = 0.0;

        var_inv_xi_dc = var_inv_xi;
        var_inv_xi_dc_dn5 = var_inv_xi_dn5;
        var_inv_xi_dc_dn6 = var_inv_xi_dn6;
        var_inv_xi_dc_dn7 = var_inv_xi_dn7;
        var_inv_xi_dc_dn8 = var_inv_xi_dn8;
        var_inv_xi_dc_rv = 0.0;

        var_sp_s_x1_dc = var_sp_s_x1;
        var_sp_s_x1_dc_dn5 = var_sp_s_x1_dn5;
        var_sp_s_x1_dc_dn6 = var_sp_s_x1_dn6;
        var_sp_s_x1_dc_dn7 = var_sp_s_x1_dn7;
        var_sp_s_x1_dc_dn8 = var_sp_s_x1_dn8;
        var_sp_s_x1_dc_rv = 0.0;

        var_delta_ns_dc = var_delta_ns;
        var_delta_ns_dc_dn5 = var_delta_ns_dn5;
        var_delta_ns_dc_dn6 = var_delta_ns_dn6;
        var_delta_ns_dc_dn7 = var_delta_ns_dn7;
        var_delta_ns_dc_dn8 = var_delta_ns_dn8;
        var_delta_ns_dc_rv = 0.0;

        var_x_s_dc = var_x_s;
        var_x_s_dc_dn5 = var_x_s_dn5;
        var_x_s_dc_dn6 = var_x_s_dn6;
        var_x_s_dc_dn7 = var_x_s_dn7;
        var_x_s_dc_dn8 = var_x_s_dn8;
        var_x_s_dc_rv = 0.0;

        var_xi1s_dc = var_xi1s;
        var_xi1s_dc_dn5 = var_xi1s_dn5;
        var_xi1s_dc_dn6 = var_xi1s_dn6;
        var_xi1s_dc_dn7 = var_xi1s_dn7;
        var_xi1s_dc_dn8 = var_xi1s_dn8;
        var_xi1s_dc_rv = 0.0;

        var_xi2s_dc = var_xi2s;
        var_xi2s_dc_dn5 = var_xi2s_dn5;
        var_xi2s_dc_dn6 = var_xi2s_dn6;
        var_xi2s_dc_dn7 = var_xi2s_dn7;
        var_xi2s_dc_dn8 = var_xi2s_dn8;
        var_xi2s_dc_rv = 0.0;

        var_delta_1s_dc = var_delta_1s;
        var_delta_1s_dc_dn5 = var_delta_1s_dn5;
        var_delta_1s_dc_dn6 = var_delta_1s_dn6;
        var_delta_1s_dc_dn7 = var_delta_1s_dn7;
        var_delta_1s_dc_dn8 = var_delta_1s_dn8;
        var_delta_1s_dc_rv = 0.0;

        var_es_dc = var_es;
        var_es_dc_dn5 = var_es_dn5;
        var_es_dc_dn6 = var_es_dn6;
        var_es_dc_dn7 = var_es_dn7;
        var_es_dc_dn8 = var_es_dn8;
        var_es_dc_rv = 0.0;

        var_ps_dc = var_ps;
        var_ps_dc_dn5 = var_ps_dn5;
        var_ps_dc_dn6 = var_ps_dn6;
        var_ps_dc_dn7 = var_ps_dn7;
        var_ps_dc_dn8 = var_ps_dn8;
        var_ps_dc_rv = 0.0;

        var_ds_dc = var_ds;
        var_ds_dc_dn5 = var_ds_dn5;
        var_ds_dc_dn6 = var_ds_dn6;
        var_ds_dc_dn7 = var_ds_dn7;
        var_ds_dc_dn8 = var_ds_dn8;
        var_ds_dc_rv = 0.0;

        var_sqs_dc = var_sqs;
        var_sqs_dc_dn5 = var_sqs_dn5;
        var_sqs_dc_dn6 = var_sqs_dn6;
        var_sqs_dc_dn7 = var_sqs_dn7;
        var_sqs_dc_dn8 = var_sqs_dn8;
        var_sqs_dc_rv = 0.0;

        var_alphas_dc = var_alphas;
        var_alphas_dc_dn5 = var_alphas_dn5;
        var_alphas_dc_dn6 = var_alphas_dn6;
        var_alphas_dc_dn7 = var_alphas_dn7;
        var_alphas_dc_dn8 = var_alphas_dn8;
        var_alphas_dc_rv = 0.0;

        var_rxcor_dc = var_rxcor;
        var_rxcor_dc_dn5 = var_rxcor_dn5;
        var_rxcor_dc_dn6 = var_rxcor_dn6;
        var_rxcor_dc_dn7 = var_rxcor_dn7;
        var_rxcor_dc_dn8 = var_rxcor_dn8;
        var_rxcor_dc_rv = 0.0;

        var_xgs_dc = var_xgs;
        var_xgs_dc_dn5 = var_xgs_dn5;
        var_xgs_dc_dn6 = var_xgs_dn6;
        var_xgs_dc_dn7 = var_xgs_dn7;
        var_xgs_dc_dn8 = var_xgs_dn8;
        var_xgs_dc_rv = 0.0;

        var_qis_dc = var_qis;
        var_qis_dc_dn5 = var_qis_dn5;
        var_qis_dc_dn6 = var_qis_dn6;
        var_qis_dc_dn7 = var_qis_dn7;
        var_qis_dc_dn8 = var_qis_dn8;
        var_qis_dc_rv = 0.0;

        var_qbs_dc = var_qbs;
        var_qbs_dc_dn5 = var_qbs_dn5;
        var_qbs_dc_dn6 = var_qbs_dn6;
        var_qbs_dc_dn7 = var_qbs_dn7;
        var_qbs_dc_dn8 = var_qbs_dn8;
        var_qbs_dc_rv = 0.0;

        var_rhob_dc = var_rhob;
        var_rhob_dc_dn5 = var_rhob_dn5;
        var_rhob_dc_dn6 = var_rhob_dn6;
        var_rhob_dc_dn7 = var_rhob_dn7;
        var_rhob_dc_dn8 = var_rhob_dn8;
        var_rhob_dc_rv = 0.0;

        var_rhog_dc = var_rhog;
        var_rhog_dc_dn5 = var_rhog_dn5;
        var_rhog_dc_dn6 = var_rhog_dn6;
        var_rhog_dc_dn7 = var_rhog_dn7;
        var_rhog_dc_dn8 = var_rhog_dn8;
        var_rhog_dc_rv = 0.0;

        var_gmobs_dc = var_gmobs;
        var_gmobs_dc_dn5 = var_gmobs_dn5;
        var_gmobs_dc_dn6 = var_gmobs_dn6;
        var_gmobs_dc_dn7 = var_gmobs_dn7;
        var_gmobs_dc_dn8 = var_gmobs_dn8;
        var_gmobs_dc_rv = 0.0;

        var_xitsb_dc = var_xitsb;
        var_xitsb_dc_dn5 = var_xitsb_dn5;
        var_xitsb_dc_dn6 = var_xitsb_dn6;
        var_xitsb_dc_dn7 = var_xitsb_dn7;
        var_xitsb_dc_dn8 = var_xitsb_dn8;
        var_xitsb_dc_rv = 0.0;

        var_factheta_dc = var_factheta;
        var_factheta_dc_dn5 = var_factheta_dn5;
        var_factheta_dc_dn6 = var_factheta_dn6;
        var_factheta_dc_dn7 = var_factheta_dn7;
        var_factheta_dc_dn8 = var_factheta_dn8;
        var_factheta_dc_rv = 0.0;

        var_thesat1 = 0.0;
        var_thesat1_dn5 = 0.0;
        var_thesat1_dn6 = 0.0;
        var_thesat1_dn7 = 0.0;
        var_thesat1_dn8 = 0.0;
        var_thesat1_rv = 0.0;

        let assign43300_e56284: f64 = (var_phit1 * 4.60517018598809);
        var_vdsat_lim = assign43300_e56284;
        var_vdsat_lim_dn5 = (var_phit1_dn5 * 4.60517018598809);
        var_vdsat_lim_dn6 = (var_phit1_dn6 * 4.60517018598809);
        var_vdsat_lim_dn7 = (var_phit1_dn7 * 4.60517018598809);
        var_vdsat_lim_dn8 = (var_phit1_dn8 * 4.60517018598809);
        var_vdsat_lim_rv = 0.0;

        var_v_dsat = var_vdsat_lim;
        var_v_dsat_dn5 = var_vdsat_lim_dn5;
        var_v_dsat_dn6 = var_vdsat_lim_dn6;
        var_v_dsat_dn7 = var_vdsat_lim_dn7;
        var_v_dsat_dn8 = var_vdsat_lim_dn8;
        var_v_dsat_rv = 0.0;

        var_vdse = var_v_ds;
        var_vdse_dn5 = 0.0;
        var_vdse_dn6 = var_v_ds_dn6;
        var_vdse_dn7 = var_v_ds_dn7;
        var_vdse_dn8 = 0.0;
        var_vdse_rv = 0.0;

        let assign43330_e56289: f64 = (var_v_ds * var_inv_phit1);
        var_udse = assign43330_e56289;
        var_udse_dn5 = (var_v_ds * var_inv_phit1_dn5);
        var_udse_dn6 = ((var_v_ds_dn6 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn6));
        var_udse_dn7 = ((var_v_ds_dn7 * var_inv_phit1) + (var_v_ds * var_inv_phit1_dn7));
        var_udse_dn8 = (var_v_ds * var_inv_phit1_dn8);
        var_udse_rv = 0.0;

        var_x_d = var_x_s;
        var_x_d_dn5 = var_x_s_dn5;
        var_x_d_dn6 = var_x_s_dn6;
        var_x_d_dn7 = var_x_s_dn7;
        var_x_d_dn8 = var_x_s_dn8;
        var_x_d_rv = 0.0;

        var_x_ds = 0.0;
        var_x_ds_dn5 = 0.0;
        var_x_ds_dn6 = 0.0;
        var_x_ds_dn7 = 0.0;
        var_x_ds_dn8 = 0.0;
        var_x_ds_rv = 0.0;

        var_dps = 0.0;
        var_dps_dn5 = 0.0;
        var_dps_dn6 = 0.0;
        var_dps_dn7 = 0.0;
        var_dps_dn8 = 0.0;
        var_dps_rv = 0.0;

        var_ed = var_es;
        var_ed_dn5 = var_es_dn5;
        var_ed_dn6 = var_es_dn6;
        var_ed_dn7 = var_es_dn7;
        var_ed_dn8 = var_es_dn8;
        var_ed_rv = 0.0;

        var_pd = var_ps;
        var_pd_dn5 = var_ps_dn5;
        var_pd_dn6 = var_ps_dn6;
        var_pd_dn7 = var_ps_dn7;
        var_pd_dn8 = var_ps_dn8;
        var_pd_rv = 0.0;

        var_dd = var_ds;
        var_dd_dn5 = var_ds_dn5;
        var_dd_dn6 = var_ds_dn6;
        var_dd_dn7 = var_ds_dn7;
        var_dd_dn8 = var_ds_dn8;
        var_dd_rv = 0.0;

        var_qbd = var_qbs;
        var_qbd_dn5 = var_qbs_dn5;
        var_qbd_dn6 = var_qbs_dn6;
        var_qbd_dn7 = var_qbs_dn7;
        var_qbd_dn8 = var_qbs_dn8;
        var_qbd_rv = 0.0;

        var_x_m = var_x_s;
        var_x_m_dn5 = var_x_s_dn5;
        var_x_m_dn6 = var_x_s_dn6;
        var_x_m_dn7 = var_x_s_dn7;
        var_x_m_dn8 = var_x_s_dn8;
        var_x_m_rv = 0.0;

        var_em = var_es;
        var_em_dn5 = var_es_dn5;
        var_em_dn6 = var_es_dn6;
        var_em_dn7 = var_es_dn7;
        var_em_dn8 = var_es_dn8;
        var_em_rv = 0.0;

        var_dm = var_ds;
        var_dm_dn5 = var_ds_dn5;
        var_dm_dn6 = var_ds_dn6;
        var_dm_dn7 = var_ds_dn7;
        var_dm_dn8 = var_ds_dn8;
        var_dm_rv = 0.0;

        var_pm = var_ps;
        var_pm_dn5 = var_ps_dn5;
        var_pm_dn6 = var_ps_dn6;
        var_pm_dn7 = var_ps_dn7;
        var_pm_dn8 = var_ps_dn8;
        var_pm_rv = 0.0;

        let assign43450_e56303: f64 = (var_xg - var_x_s);
        var_xgm = assign43450_e56303;
        var_xgm_dn5 = (var_xg_dn5 - var_x_s_dn5);
        var_xgm_dn6 = (var_xg_dn6 - var_x_s_dn6);
        var_xgm_dn7 = (var_xg_dn7 - var_x_s_dn7);
        var_xgm_dn8 = (var_xg_dn8 - var_x_s_dn8);
        var_xgm_rv = 0.0;

        var_eta_p = 1.0;
        var_eta_p_dn5 = 0.0;
        var_eta_p_dn6 = 0.0;
        var_eta_p_dn7 = 0.0;
        var_eta_p_dn8 = 0.0;
        var_eta_p_rv = 0.0;

        var_alpha = 1.0;
        var_alpha_dn5 = 0.0;
        var_alpha_dn6 = 0.0;
        var_alpha_dn7 = 0.0;
        var_alpha_dn8 = 0.0;
        var_alpha_rv = 0.0;

        var_sqm = 0.0;
        var_sqm_dn5 = 0.0;
        var_sqm_dn6 = 0.0;
        var_sqm_dn7 = 0.0;
        var_sqm_dn8 = 0.0;
        var_sqm_rv = 0.0;

        var_qim = var_qis;
        var_qim_dn5 = var_qis_dn5;
        var_qim_dn6 = var_qis_dn6;
        var_qim_dn7 = var_qis_dn7;
        var_qim_dn8 = var_qis_dn8;
        var_qim_rv = 0.0;

        let assign43500_e56310: f64 = (var_xgm * var_phit1);
        var_qeff1 = assign43500_e56310;
        var_qeff1_dn5 = ((var_xgm_dn5 * var_phit1) + (var_xgm * var_phit1_dn5));
        var_qeff1_dn6 = ((var_xgm_dn6 * var_phit1) + (var_xgm * var_phit1_dn6));
        var_qeff1_dn7 = ((var_xgm_dn7 * var_phit1) + (var_xgm * var_phit1_dn7));
        var_qeff1_dn8 = ((var_xgm_dn8 * var_phit1) + (var_xgm * var_phit1_dn8));
        var_qeff1_rv = 0.0;

        var_qim1 = 0.0;
        var_qim1_dn5 = 0.0;
        var_qim1_dn6 = 0.0;
        var_qim1_dn7 = 0.0;
        var_qim1_dn8 = 0.0;
        var_qim1_rv = 0.0;

        var_qbm = var_qbs;
        var_qbm_dn5 = var_qbs_dn5;
        var_qbm_dn6 = var_qbs_dn6;
        var_qbm_dn7 = var_qbs_dn7;
        var_qbm_dn8 = var_qbs_dn8;
        var_qbm_rv = 0.0;

        var_s1 = 0.0;
        var_s1_dn5 = 0.0;
        var_s1_dn6 = 0.0;
        var_s1_dn7 = 0.0;
        var_s1_dn8 = 0.0;
        var_s1_rv = 0.0;

        var_gmob = 1.0;
        var_gmob_dn5 = 0.0;
        var_gmob_dn6 = 0.0;
        var_gmob_dn7 = 0.0;
        var_gmob_dn8 = 0.0;
        var_gmob_rv = 0.0;

        var_thesateff = var_thesatloc;
        var_thesateff_dn5 = 0.0;
        var_thesateff_dn6 = 0.0;
        var_thesateff_dn7 = 0.0;
        var_thesateff_dn8 = 0.0;
        var_thesateff_rv = 0.0;

        var_voxm = var_qeff1;
        var_voxm_dn5 = var_qeff1_dn5;
        var_voxm_dn6 = var_qeff1_dn6;
        var_voxm_dn7 = var_qeff1_dn7;
        var_voxm_dn8 = var_qeff1_dn8;
        var_voxm_rv = 0.0;

        let assign43570_e56319: f64 = if var_xg > 0.0 { 1.0 } else { 0.0 };
        var_guard1197 = assign43570_e56319;
        var_guard1197_rv = 0.0;

        let assign43580_e56322: f64 = if var_ds > 1e-100 { 1.0 } else { 0.0 };
        var_guard1198 = assign43580_e56322;
        var_guard1198_rv = 0.0;

        let (assign43590_e56330, assign43590_e56330_d_n5, assign43590_e56330_d_n6, assign43590_e56330_d_n7, assign43590_e56330_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43590_e56328: f64 = (var_thesatloc * var_factheta);
        (assign43590_e56328, (var_thesatloc * var_factheta_dn5), (var_thesatloc * var_factheta_dn6), (var_thesatloc * var_factheta_dn7), (var_thesatloc * var_factheta_dn8),)
    } else {
        (var_thesateff, var_thesateff_dn5, var_thesateff_dn6, var_thesateff_dn7, var_thesateff_dn8,)
    }
};
        var_thesateff = assign43590_e56330;
        var_thesateff_dn5 = assign43590_e56330_d_n5;
        var_thesateff_dn6 = assign43590_e56330_d_n6;
        var_thesateff_dn7 = assign43590_e56330_d_n7;
        var_thesateff_dn8 = assign43590_e56330_d_n8;
        var_thesateff_rv = 0.0;

        let (assign43600_e56338, assign43600_e56338_d_n5, assign43600_e56338_d_n6, assign43600_e56338_d_n7, assign43600_e56338_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43600_e56336: f64 = (var_thesateff / var_gmobs);
        (assign43600_e56336, (((var_thesateff_dn5 * var_gmobs) - (var_thesateff * var_gmobs_dn5)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn6 * var_gmobs) - (var_thesateff * var_gmobs_dn6)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn7 * var_gmobs) - (var_thesateff * var_gmobs_dn7)) / (var_gmobs * var_gmobs)), (((var_thesateff_dn8 * var_gmobs) - (var_thesateff * var_gmobs_dn8)) / (var_gmobs * var_gmobs)),)
    } else {
        (var_thesat1, var_thesat1_dn5, var_thesat1_dn6, var_thesat1_dn7, var_thesat1_dn8,)
    }
};
        var_thesat1 = assign43600_e56338;
        var_thesat1_dn5 = assign43600_e56338_d_n5;
        var_thesat1_dn6 = assign43600_e56338_d_n6;
        var_thesat1_dn7 = assign43600_e56338_d_n7;
        var_thesat1_dn8 = assign43600_e56338_d_n8;
        var_thesat1_rv = 0.0;

        let (assign43610_e56348, assign43610_e56348_d_n5, assign43610_e56348_d_n6, assign43610_e56348_d_n7, assign43610_e56348_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43610_e56345: f64 = (0.5 * var_gf2);
        let assign43610_e56346: f64 = (var_xgs + assign43610_e56345);
        (assign43610_e56346, (var_xgs_dn5 + (0.5 * var_gf2_dn5)), (var_xgs_dn6 + (0.5 * var_gf2_dn6)), (var_xgs_dn7 + (0.5 * var_gf2_dn7)), (var_xgs_dn8 + (0.5 * var_gf2_dn8)),)
    } else {
        (var_asat, var_asat_dn5, var_asat_dn6, var_asat_dn7, var_asat_dn8,)
    }
};
        var_asat = assign43610_e56348;
        var_asat_dn5 = assign43610_e56348_d_n5;
        var_asat_dn6 = assign43610_e56348_d_n6;
        var_asat_dn7 = assign43610_e56348_d_n7;
        var_asat_dn8 = assign43610_e56348_d_n8;
        var_asat_rv = 0.0;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn5_slot = var_alpha_dn5;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alpha_rv_slot = var_alpha_rv;
        *var_alphas_dc_slot = var_alphas_dc;
        *var_alphas_dc_dn5_slot = var_alphas_dc_dn5;
        *var_alphas_dc_dn6_slot = var_alphas_dc_dn6;
        *var_alphas_dc_dn7_slot = var_alphas_dc_dn7;
        *var_alphas_dc_dn8_slot = var_alphas_dc_dn8;
        *var_alphas_dc_rv_slot = var_alphas_dc_rv;
        *var_asat_slot = var_asat;
        *var_asat_dn5_slot = var_asat_dn5;
        *var_asat_dn6_slot = var_asat_dn6;
        *var_asat_dn7_slot = var_asat_dn7;
        *var_asat_dn8_slot = var_asat_dn8;
        *var_asat_rv_slot = var_asat_rv;
        *var_dd_slot = var_dd;
        *var_dd_dn5_slot = var_dd_dn5;
        *var_dd_dn6_slot = var_dd_dn6;
        *var_dd_dn7_slot = var_dd_dn7;
        *var_dd_dn8_slot = var_dd_dn8;
        *var_dd_rv_slot = var_dd_rv;
        *var_delta_1s_dc_slot = var_delta_1s_dc;
        *var_delta_1s_dc_dn5_slot = var_delta_1s_dc_dn5;
        *var_delta_1s_dc_dn6_slot = var_delta_1s_dc_dn6;
        *var_delta_1s_dc_dn7_slot = var_delta_1s_dc_dn7;
        *var_delta_1s_dc_dn8_slot = var_delta_1s_dc_dn8;
        *var_delta_1s_dc_rv_slot = var_delta_1s_dc_rv;
        *var_delta_ns_dc_slot = var_delta_ns_dc;
        *var_delta_ns_dc_dn5_slot = var_delta_ns_dc_dn5;
        *var_delta_ns_dc_dn6_slot = var_delta_ns_dc_dn6;
        *var_delta_ns_dc_dn7_slot = var_delta_ns_dc_dn7;
        *var_delta_ns_dc_dn8_slot = var_delta_ns_dc_dn8;
        *var_delta_ns_dc_rv_slot = var_delta_ns_dc_rv;
        *var_dm_slot = var_dm;
        *var_dm_dn5_slot = var_dm_dn5;
        *var_dm_dn6_slot = var_dm_dn6;
        *var_dm_dn7_slot = var_dm_dn7;
        *var_dm_dn8_slot = var_dm_dn8;
        *var_dm_rv_slot = var_dm_rv;
        *var_dps_slot = var_dps;
        *var_dps_dn5_slot = var_dps_dn5;
        *var_dps_dn6_slot = var_dps_dn6;
        *var_dps_dn7_slot = var_dps_dn7;
        *var_dps_dn8_slot = var_dps_dn8;
        *var_dps_rv_slot = var_dps_rv;
        *var_ds_dc_slot = var_ds_dc;
        *var_ds_dc_dn5_slot = var_ds_dc_dn5;
        *var_ds_dc_dn6_slot = var_ds_dc_dn6;
        *var_ds_dc_dn7_slot = var_ds_dc_dn7;
        *var_ds_dc_dn8_slot = var_ds_dc_dn8;
        *var_ds_dc_rv_slot = var_ds_dc_rv;
        *var_ed_slot = var_ed;
        *var_ed_dn5_slot = var_ed_dn5;
        *var_ed_dn6_slot = var_ed_dn6;
        *var_ed_dn7_slot = var_ed_dn7;
        *var_ed_dn8_slot = var_ed_dn8;
        *var_ed_rv_slot = var_ed_rv;
        *var_em_slot = var_em;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_rv_slot = var_em_rv;
        *var_es_dc_slot = var_es_dc;
        *var_es_dc_dn5_slot = var_es_dc_dn5;
        *var_es_dc_dn6_slot = var_es_dc_dn6;
        *var_es_dc_dn7_slot = var_es_dc_dn7;
        *var_es_dc_dn8_slot = var_es_dc_dn8;
        *var_es_dc_rv_slot = var_es_dc_rv;
        *var_eta_p_slot = var_eta_p;
        *var_eta_p_dn5_slot = var_eta_p_dn5;
        *var_eta_p_dn6_slot = var_eta_p_dn6;
        *var_eta_p_dn7_slot = var_eta_p_dn7;
        *var_eta_p_dn8_slot = var_eta_p_dn8;
        *var_eta_p_rv_slot = var_eta_p_rv;
        *var_factheta_slot = var_factheta;
        *var_factheta_dc_slot = var_factheta_dc;
        *var_factheta_dc_dn5_slot = var_factheta_dc_dn5;
        *var_factheta_dc_dn6_slot = var_factheta_dc_dn6;
        *var_factheta_dc_dn7_slot = var_factheta_dc_dn7;
        *var_factheta_dc_dn8_slot = var_factheta_dc_dn8;
        *var_factheta_dc_rv_slot = var_factheta_dc_rv;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_rv_slot = var_factheta_rv;
        *var_gf2_dc_slot = var_gf2_dc;
        *var_gf2_dc_dn5_slot = var_gf2_dc_dn5;
        *var_gf2_dc_dn6_slot = var_gf2_dc_dn6;
        *var_gf2_dc_dn7_slot = var_gf2_dc_dn7;
        *var_gf2_dc_dn8_slot = var_gf2_dc_dn8;
        *var_gf2_dc_rv_slot = var_gf2_dc_rv;
        *var_gf_dc_slot = var_gf_dc;
        *var_gf_dc_dn5_slot = var_gf_dc_dn5;
        *var_gf_dc_dn6_slot = var_gf_dc_dn6;
        *var_gf_dc_dn7_slot = var_gf_dc_dn7;
        *var_gf_dc_dn8_slot = var_gf_dc_dn8;
        *var_gf_dc_rv_slot = var_gf_dc_rv;
        *var_gmob_slot = var_gmob;
        *var_gmob_dn5_slot = var_gmob_dn5;
        *var_gmob_dn6_slot = var_gmob_dn6;
        *var_gmob_dn7_slot = var_gmob_dn7;
        *var_gmob_dn8_slot = var_gmob_dn8;
        *var_gmob_rv_slot = var_gmob_rv;
        *var_gmobs_dc_slot = var_gmobs_dc;
        *var_gmobs_dc_dn5_slot = var_gmobs_dc_dn5;
        *var_gmobs_dc_dn6_slot = var_gmobs_dc_dn6;
        *var_gmobs_dc_dn7_slot = var_gmobs_dc_dn7;
        *var_gmobs_dc_dn8_slot = var_gmobs_dc_dn8;
        *var_gmobs_dc_rv_slot = var_gmobs_dc_rv;
        *var_guard1197_slot = var_guard1197;
        *var_guard1197_rv_slot = var_guard1197_rv;
        *var_guard1198_slot = var_guard1198;
        *var_guard1198_rv_slot = var_guard1198_rv;
        *var_inv_gf2_dc_slot = var_inv_gf2_dc;
        *var_inv_gf2_dc_dn5_slot = var_inv_gf2_dc_dn5;
        *var_inv_gf2_dc_dn6_slot = var_inv_gf2_dc_dn6;
        *var_inv_gf2_dc_dn7_slot = var_inv_gf2_dc_dn7;
        *var_inv_gf2_dc_dn8_slot = var_inv_gf2_dc_dn8;
        *var_inv_gf2_dc_rv_slot = var_inv_gf2_dc_rv;
        *var_inv_phit1_dc_slot = var_inv_phit1_dc;
        *var_inv_phit1_dc_dn5_slot = var_inv_phit1_dc_dn5;
        *var_inv_phit1_dc_dn6_slot = var_inv_phit1_dc_dn6;
        *var_inv_phit1_dc_dn7_slot = var_inv_phit1_dc_dn7;
        *var_inv_phit1_dc_dn8_slot = var_inv_phit1_dc_dn8;
        *var_inv_phit1_dc_rv_slot = var_inv_phit1_dc_rv;
        *var_inv_xi_dc_slot = var_inv_xi_dc;
        *var_inv_xi_dc_dn5_slot = var_inv_xi_dc_dn5;
        *var_inv_xi_dc_dn6_slot = var_inv_xi_dc_dn6;
        *var_inv_xi_dc_dn7_slot = var_inv_xi_dc_dn7;
        *var_inv_xi_dc_dn8_slot = var_inv_xi_dc_dn8;
        *var_inv_xi_dc_rv_slot = var_inv_xi_dc_rv;
        *var_margin_dc_slot = var_margin_dc;
        *var_margin_dc_dn5_slot = var_margin_dc_dn5;
        *var_margin_dc_dn6_slot = var_margin_dc_dn6;
        *var_margin_dc_dn7_slot = var_margin_dc_dn7;
        *var_margin_dc_dn8_slot = var_margin_dc_dn8;
        *var_margin_dc_rv_slot = var_margin_dc_rv;
        *var_pd_slot = var_pd;
        *var_pd_dn5_slot = var_pd_dn5;
        *var_pd_dn6_slot = var_pd_dn6;
        *var_pd_dn7_slot = var_pd_dn7;
        *var_pd_dn8_slot = var_pd_dn8;
        *var_pd_rv_slot = var_pd_rv;
        *var_phit1_dc_slot = var_phit1_dc;
        *var_phit1_dc_dn5_slot = var_phit1_dc_dn5;
        *var_phit1_dc_dn6_slot = var_phit1_dc_dn6;
        *var_phit1_dc_dn7_slot = var_phit1_dc_dn7;
        *var_phit1_dc_dn8_slot = var_phit1_dc_dn8;
        *var_phit1_dc_rv_slot = var_phit1_dc_rv;
        *var_pm_slot = var_pm;
        *var_pm_dn5_slot = var_pm_dn5;
        *var_pm_dn6_slot = var_pm_dn6;
        *var_pm_dn7_slot = var_pm_dn7;
        *var_pm_dn8_slot = var_pm_dn8;
        *var_pm_rv_slot = var_pm_rv;
        *var_ps_dc_slot = var_ps_dc;
        *var_ps_dc_dn5_slot = var_ps_dc_dn5;
        *var_ps_dc_dn6_slot = var_ps_dc_dn6;
        *var_ps_dc_dn7_slot = var_ps_dc_dn7;
        *var_ps_dc_dn8_slot = var_ps_dc_dn8;
        *var_ps_dc_rv_slot = var_ps_dc_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn5_slot = var_qbd_dn5;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_qbm_slot = var_qbm;
        *var_qbm_dn5_slot = var_qbm_dn5;
        *var_qbm_dn6_slot = var_qbm_dn6;
        *var_qbm_dn7_slot = var_qbm_dn7;
        *var_qbm_dn8_slot = var_qbm_dn8;
        *var_qbm_rv_slot = var_qbm_rv;
        *var_qbs_dc_slot = var_qbs_dc;
        *var_qbs_dc_dn5_slot = var_qbs_dc_dn5;
        *var_qbs_dc_dn6_slot = var_qbs_dc_dn6;
        *var_qbs_dc_dn7_slot = var_qbs_dc_dn7;
        *var_qbs_dc_dn8_slot = var_qbs_dc_dn8;
        *var_qbs_dc_rv_slot = var_qbs_dc_rv;
        *var_qeff1_slot = var_qeff1;
        *var_qeff1_dn5_slot = var_qeff1_dn5;
        *var_qeff1_dn6_slot = var_qeff1_dn6;
        *var_qeff1_dn7_slot = var_qeff1_dn7;
        *var_qeff1_dn8_slot = var_qeff1_dn8;
        *var_qeff1_rv_slot = var_qeff1_rv;
        *var_qim_slot = var_qim;
        *var_qim1_slot = var_qim1;
        *var_qim1_dn5_slot = var_qim1_dn5;
        *var_qim1_dn6_slot = var_qim1_dn6;
        *var_qim1_dn7_slot = var_qim1_dn7;
        *var_qim1_dn8_slot = var_qim1_dn8;
        *var_qim1_rv_slot = var_qim1_rv;
        *var_qim_dn5_slot = var_qim_dn5;
        *var_qim_dn6_slot = var_qim_dn6;
        *var_qim_dn7_slot = var_qim_dn7;
        *var_qim_dn8_slot = var_qim_dn8;
        *var_qim_rv_slot = var_qim_rv;
        *var_qis_dc_slot = var_qis_dc;
        *var_qis_dc_dn5_slot = var_qis_dc_dn5;
        *var_qis_dc_dn6_slot = var_qis_dc_dn6;
        *var_qis_dc_dn7_slot = var_qis_dc_dn7;
        *var_qis_dc_dn8_slot = var_qis_dc_dn8;
        *var_qis_dc_rv_slot = var_qis_dc_rv;
        *var_rhob_dc_slot = var_rhob_dc;
        *var_rhob_dc_dn5_slot = var_rhob_dc_dn5;
        *var_rhob_dc_dn6_slot = var_rhob_dc_dn6;
        *var_rhob_dc_dn7_slot = var_rhob_dc_dn7;
        *var_rhob_dc_dn8_slot = var_rhob_dc_dn8;
        *var_rhob_dc_rv_slot = var_rhob_dc_rv;
        *var_rhog_dc_slot = var_rhog_dc;
        *var_rhog_dc_dn5_slot = var_rhog_dc_dn5;
        *var_rhog_dc_dn6_slot = var_rhog_dc_dn6;
        *var_rhog_dc_dn7_slot = var_rhog_dc_dn7;
        *var_rhog_dc_dn8_slot = var_rhog_dc_dn8;
        *var_rhog_dc_rv_slot = var_rhog_dc_rv;
        *var_rxcor_dc_slot = var_rxcor_dc;
        *var_rxcor_dc_dn5_slot = var_rxcor_dc_dn5;
        *var_rxcor_dc_dn6_slot = var_rxcor_dc_dn6;
        *var_rxcor_dc_dn7_slot = var_rxcor_dc_dn7;
        *var_rxcor_dc_dn8_slot = var_rxcor_dc_dn8;
        *var_rxcor_dc_rv_slot = var_rxcor_dc_rv;
        *var_s1_slot = var_s1;
        *var_s1_dn5_slot = var_s1_dn5;
        *var_s1_dn6_slot = var_s1_dn6;
        *var_s1_dn7_slot = var_s1_dn7;
        *var_s1_dn8_slot = var_s1_dn8;
        *var_s1_rv_slot = var_s1_rv;
        *var_sp_s_x1_dc_slot = var_sp_s_x1_dc;
        *var_sp_s_x1_dc_dn5_slot = var_sp_s_x1_dc_dn5;
        *var_sp_s_x1_dc_dn6_slot = var_sp_s_x1_dc_dn6;
        *var_sp_s_x1_dc_dn7_slot = var_sp_s_x1_dc_dn7;
        *var_sp_s_x1_dc_dn8_slot = var_sp_s_x1_dc_dn8;
        *var_sp_s_x1_dc_rv_slot = var_sp_s_x1_dc_rv;
        *var_sqm_slot = var_sqm;
        *var_sqm_dn5_slot = var_sqm_dn5;
        *var_sqm_dn6_slot = var_sqm_dn6;
        *var_sqm_dn7_slot = var_sqm_dn7;
        *var_sqm_dn8_slot = var_sqm_dn8;
        *var_sqm_rv_slot = var_sqm_rv;
        *var_sqs_dc_slot = var_sqs_dc;
        *var_sqs_dc_dn5_slot = var_sqs_dc_dn5;
        *var_sqs_dc_dn6_slot = var_sqs_dc_dn6;
        *var_sqs_dc_dn7_slot = var_sqs_dc_dn7;
        *var_sqs_dc_dn8_slot = var_sqs_dc_dn8;
        *var_sqs_dc_rv_slot = var_sqs_dc_rv;
        *var_thesat1_slot = var_thesat1;
        *var_thesat1_dn5_slot = var_thesat1_dn5;
        *var_thesat1_dn6_slot = var_thesat1_dn6;
        *var_thesat1_dn7_slot = var_thesat1_dn7;
        *var_thesat1_dn8_slot = var_thesat1_dn8;
        *var_thesat1_rv_slot = var_thesat1_rv;
        *var_thesateff_slot = var_thesateff;
        *var_thesateff_dn5_slot = var_thesateff_dn5;
        *var_thesateff_dn6_slot = var_thesateff_dn6;
        *var_thesateff_dn7_slot = var_thesateff_dn7;
        *var_thesateff_dn8_slot = var_thesateff_dn8;
        *var_thesateff_rv_slot = var_thesateff_rv;
        *var_udse_slot = var_udse;
        *var_udse_dn5_slot = var_udse_dn5;
        *var_udse_dn6_slot = var_udse_dn6;
        *var_udse_dn7_slot = var_udse_dn7;
        *var_udse_dn8_slot = var_udse_dn8;
        *var_udse_rv_slot = var_udse_rv;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn5_slot = var_v_dsat_dn5;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_v_dsat_rv_slot = var_v_dsat_rv;
        *var_vdsat_lim_slot = var_vdsat_lim;
        *var_vdsat_lim_dn5_slot = var_vdsat_lim_dn5;
        *var_vdsat_lim_dn6_slot = var_vdsat_lim_dn6;
        *var_vdsat_lim_dn7_slot = var_vdsat_lim_dn7;
        *var_vdsat_lim_dn8_slot = var_vdsat_lim_dn8;
        *var_vdsat_lim_rv_slot = var_vdsat_lim_rv;
        *var_vdse_slot = var_vdse;
        *var_vdse_dn5_slot = var_vdse_dn5;
        *var_vdse_dn6_slot = var_vdse_dn6;
        *var_vdse_dn7_slot = var_vdse_dn7;
        *var_vdse_dn8_slot = var_vdse_dn8;
        *var_vdse_rv_slot = var_vdse_rv;
        *var_vgb1_dc_slot = var_vgb1_dc;
        *var_vgb1_dc_dn5_slot = var_vgb1_dc_dn5;
        *var_vgb1_dc_dn6_slot = var_vgb1_dc_dn6;
        *var_vgb1_dc_dn7_slot = var_vgb1_dc_dn7;
        *var_vgb1_dc_dn8_slot = var_vgb1_dc_dn8;
        *var_vgb1_dc_rv_slot = var_vgb1_dc_rv;
        *var_voxm_slot = var_voxm;
        *var_voxm_dn5_slot = var_voxm_dn5;
        *var_voxm_dn6_slot = var_voxm_dn6;
        *var_voxm_dn7_slot = var_voxm_dn7;
        *var_voxm_dn8_slot = var_voxm_dn8;
        *var_voxm_rv_slot = var_voxm_rv;
        *var_vsbx_dc_slot = var_vsbx_dc;
        *var_vsbx_dc_dn5_slot = var_vsbx_dc_dn5;
        *var_vsbx_dc_dn6_slot = var_vsbx_dc_dn6;
        *var_vsbx_dc_dn7_slot = var_vsbx_dc_dn7;
        *var_vsbx_dc_dn8_slot = var_vsbx_dc_dn8;
        *var_vsbx_dc_rv_slot = var_vsbx_dc_rv;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_d_rv_slot = var_x_d_rv;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn5_slot = var_x_ds_dn5;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_ds_rv_slot = var_x_ds_rv;
        *var_x_m_slot = var_x_m;
        *var_x_m_dn5_slot = var_x_m_dn5;
        *var_x_m_dn6_slot = var_x_m_dn6;
        *var_x_m_dn7_slot = var_x_m_dn7;
        *var_x_m_dn8_slot = var_x_m_dn8;
        *var_x_m_rv_slot = var_x_m_rv;
        *var_x_s_dc_slot = var_x_s_dc;
        *var_x_s_dc_dn5_slot = var_x_s_dc_dn5;
        *var_x_s_dc_dn6_slot = var_x_s_dc_dn6;
        *var_x_s_dc_dn7_slot = var_x_s_dc_dn7;
        *var_x_s_dc_dn8_slot = var_x_s_dc_dn8;
        *var_x_s_dc_rv_slot = var_x_s_dc_rv;
        *var_xg_dc_slot = var_xg_dc;
        *var_xg_dc_dn5_slot = var_xg_dc_dn5;
        *var_xg_dc_dn6_slot = var_xg_dc_dn6;
        *var_xg_dc_dn7_slot = var_xg_dc_dn7;
        *var_xg_dc_dn8_slot = var_xg_dc_dn8;
        *var_xg_dc_rv_slot = var_xg_dc_rv;
        *var_xgm_slot = var_xgm;
        *var_xgm_dn5_slot = var_xgm_dn5;
        *var_xgm_dn6_slot = var_xgm_dn6;
        *var_xgm_dn7_slot = var_xgm_dn7;
        *var_xgm_dn8_slot = var_xgm_dn8;
        *var_xgm_rv_slot = var_xgm_rv;
        *var_xgs_dc_slot = var_xgs_dc;
        *var_xgs_dc_dn5_slot = var_xgs_dc_dn5;
        *var_xgs_dc_dn6_slot = var_xgs_dc_dn6;
        *var_xgs_dc_dn7_slot = var_xgs_dc_dn7;
        *var_xgs_dc_dn8_slot = var_xgs_dc_dn8;
        *var_xgs_dc_rv_slot = var_xgs_dc_rv;
        *var_xi1s_dc_slot = var_xi1s_dc;
        *var_xi1s_dc_dn5_slot = var_xi1s_dc_dn5;
        *var_xi1s_dc_dn6_slot = var_xi1s_dc_dn6;
        *var_xi1s_dc_dn7_slot = var_xi1s_dc_dn7;
        *var_xi1s_dc_dn8_slot = var_xi1s_dc_dn8;
        *var_xi1s_dc_rv_slot = var_xi1s_dc_rv;
        *var_xi2s_dc_slot = var_xi2s_dc;
        *var_xi2s_dc_dn5_slot = var_xi2s_dc_dn5;
        *var_xi2s_dc_dn6_slot = var_xi2s_dc_dn6;
        *var_xi2s_dc_dn7_slot = var_xi2s_dc_dn7;
        *var_xi2s_dc_dn8_slot = var_xi2s_dc_dn8;
        *var_xi2s_dc_rv_slot = var_xi2s_dc_rv;
        *var_xi_dc_slot = var_xi_dc;
        *var_xi_dc_dn5_slot = var_xi_dc_dn5;
        *var_xi_dc_dn6_slot = var_xi_dc_dn6;
        *var_xi_dc_dn7_slot = var_xi_dc_dn7;
        *var_xi_dc_dn8_slot = var_xi_dc_dn8;
        *var_xi_dc_rv_slot = var_xi_dc_rv;
        *var_xitsb_dc_slot = var_xitsb_dc;
        *var_xitsb_dc_dn5_slot = var_xitsb_dc_dn5;
        *var_xitsb_dc_dn6_slot = var_xitsb_dc_dn6;
        *var_xitsb_dc_dn7_slot = var_xitsb_dc_dn7;
        *var_xitsb_dc_dn8_slot = var_xitsb_dc_dn8;
        *var_xitsb_dc_rv_slot = var_xitsb_dc_rv;
        *var_xn_s_dc_slot = var_xn_s_dc;
        *var_xn_s_dc_dn5_slot = var_xn_s_dc_dn5;
        *var_xn_s_dc_dn6_slot = var_xn_s_dc_dn6;
        *var_xn_s_dc_dn7_slot = var_xn_s_dc_dn7;
        *var_xn_s_dc_dn8_slot = var_xn_s_dc_dn8;
        *var_xn_s_dc_rv_slot = var_xn_s_dc_rv;
        *var_xno_s_dc_slot = var_xno_s_dc;
        *var_xno_s_dc_dn5_slot = var_xno_s_dc_dn5;
        *var_xno_s_dc_dn6_slot = var_xno_s_dc_dn6;
        *var_xno_s_dc_dn7_slot = var_xno_s_dc_dn7;
        *var_xno_s_dc_dn8_slot = var_xno_s_dc_dn8;
        *var_xno_s_dc_rv_slot = var_xno_s_dc_rv;
    }

    pub(super) fn stamp_reactive_block_31(
        var_alphas: f64,
        var_alphas_dn5: f64,
        var_alphas_dn6: f64,
        var_alphas_dn7: f64,
        var_alphas_dn8: f64,
        var_asat: f64,
        var_asat_dn5: f64,
        var_asat_dn6: f64,
        var_asat_dn7: f64,
        var_asat_dn8: f64,
        var_chnl_type: f64,
        var_cs_t: f64,
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_e_eff0: f64,
        var_eta_mu: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_guard1197: f64,
        var_guard1198: f64,
        var_mue_t: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_qis: f64,
        var_qis_dn5: f64,
        var_qis_dn6: f64,
        var_qis_dn7: f64,
        var_qis_dn8: f64,
        var_rhob: f64,
        var_rhob_dn5: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rhog: f64,
        var_rhog_dn5: f64,
        var_rhog_dn6: f64,
        var_rhog_dn7: f64,
        var_rhog_dn8: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_ther_i: f64,
        var_thesat1: f64,
        var_thesat1_dn5: f64,
        var_thesat1_dn6: f64,
        var_thesat1_dn7: f64,
        var_thesat1_dn8: f64,
        var_xgs: f64,
        var_xgs_dn5: f64,
        var_xgs_dn6: f64,
        var_xgs_dn7: f64,
        var_xgs_dn8: f64,
        var_alphasat_slot: &mut f64,
        var_alphasat_dn5_slot: &mut f64,
        var_alphasat_dn6_slot: &mut f64,
        var_alphasat_dn7_slot: &mut f64,
        var_alphasat_dn8_slot: &mut f64,
        var_alphasat_rv_slot: &mut f64,
        var_delta_gmob_slot: &mut f64,
        var_delta_gmob_dn5_slot: &mut f64,
        var_delta_gmob_dn6_slot: &mut f64,
        var_delta_gmob_dn7_slot: &mut f64,
        var_delta_gmob_dn8_slot: &mut f64,
        var_delta_gmob_rv_slot: &mut f64,
        var_gmobcssat_slot: &mut f64,
        var_gmobcssat_dn5_slot: &mut f64,
        var_gmobcssat_dn6_slot: &mut f64,
        var_gmobcssat_dn7_slot: &mut f64,
        var_gmobcssat_dn8_slot: &mut f64,
        var_gmobcssat_rv_slot: &mut f64,
        var_gmobmusat_slot: &mut f64,
        var_gmobmusat_dn5_slot: &mut f64,
        var_gmobmusat_dn6_slot: &mut f64,
        var_gmobmusat_dn7_slot: &mut f64,
        var_gmobmusat_dn8_slot: &mut f64,
        var_gmobmusat_rv_slot: &mut f64,
        var_grsat_slot: &mut f64,
        var_grsat_dn5_slot: &mut f64,
        var_grsat_dn6_slot: &mut f64,
        var_grsat_dn7_slot: &mut f64,
        var_grsat_dn8_slot: &mut f64,
        var_grsat_rv_slot: &mut f64,
        var_guard1199_slot: &mut f64,
        var_guard1199_rv_slot: &mut f64,
        var_guard1200_slot: &mut f64,
        var_guard1200_rv_slot: &mut f64,
        var_guard1201_slot: &mut f64,
        var_guard1201_rv_slot: &mut f64,
        var_guard1202_slot: &mut f64,
        var_guard1202_rv_slot: &mut f64,
        var_guard1203_slot: &mut f64,
        var_guard1203_rv_slot: &mut f64,
        var_midphi0_slot: &mut f64,
        var_midphi0_dn5_slot: &mut f64,
        var_midphi0_dn6_slot: &mut f64,
        var_midphi0_dn7_slot: &mut f64,
        var_midphi0_dn8_slot: &mut f64,
        var_midphi0_rv_slot: &mut f64,
        var_qbsat_slot: &mut f64,
        var_qbsat_dn5_slot: &mut f64,
        var_qbsat_dn6_slot: &mut f64,
        var_qbsat_dn7_slot: &mut f64,
        var_qbsat_dn8_slot: &mut f64,
        var_qbsat_rv_slot: &mut f64,
        var_qisat_slot: &mut f64,
        var_qisat_dn5_slot: &mut f64,
        var_qisat_dn6_slot: &mut f64,
        var_qisat_dn7_slot: &mut f64,
        var_qisat_dn8_slot: &mut f64,
        var_qisat_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_x_inf_slot: &mut f64,
        var_x_inf0_slot: &mut f64,
        var_x_inf0_dn5_slot: &mut f64,
        var_x_inf0_dn6_slot: &mut f64,
        var_x_inf0_dn7_slot: &mut f64,
        var_x_inf0_dn8_slot: &mut f64,
        var_x_inf0_rv_slot: &mut f64,
        var_x_inf_dn5_slot: &mut f64,
        var_x_inf_dn6_slot: &mut f64,
        var_x_inf_dn7_slot: &mut f64,
        var_x_inf_dn8_slot: &mut f64,
        var_x_inf_rv_slot: &mut f64,
        var_ysat_slot: &mut f64,
        var_ysat_dn5_slot: &mut f64,
        var_ysat_dn6_slot: &mut f64,
        var_ysat_dn7_slot: &mut f64,
        var_ysat_dn8_slot: &mut f64,
        var_ysat_rv_slot: &mut f64,
        var_za_slot: &mut f64,
        var_za_dn5_slot: &mut f64,
        var_za_dn6_slot: &mut f64,
        var_za_dn7_slot: &mut f64,
        var_za_dn8_slot: &mut f64,
        var_za_rv_slot: &mut f64,
    ) {
        let mut var_alphasat: f64 = *var_alphasat_slot;
        let mut var_alphasat_dn5: f64 = *var_alphasat_dn5_slot;
        let mut var_alphasat_dn6: f64 = *var_alphasat_dn6_slot;
        let mut var_alphasat_dn7: f64 = *var_alphasat_dn7_slot;
        let mut var_alphasat_dn8: f64 = *var_alphasat_dn8_slot;
        let mut var_alphasat_rv: f64 = *var_alphasat_rv_slot;
        let mut var_delta_gmob: f64 = *var_delta_gmob_slot;
        let mut var_delta_gmob_dn5: f64 = *var_delta_gmob_dn5_slot;
        let mut var_delta_gmob_dn6: f64 = *var_delta_gmob_dn6_slot;
        let mut var_delta_gmob_dn7: f64 = *var_delta_gmob_dn7_slot;
        let mut var_delta_gmob_dn8: f64 = *var_delta_gmob_dn8_slot;
        let mut var_delta_gmob_rv: f64 = *var_delta_gmob_rv_slot;
        let mut var_gmobcssat: f64 = *var_gmobcssat_slot;
        let mut var_gmobcssat_dn5: f64 = *var_gmobcssat_dn5_slot;
        let mut var_gmobcssat_dn6: f64 = *var_gmobcssat_dn6_slot;
        let mut var_gmobcssat_dn7: f64 = *var_gmobcssat_dn7_slot;
        let mut var_gmobcssat_dn8: f64 = *var_gmobcssat_dn8_slot;
        let mut var_gmobcssat_rv: f64 = *var_gmobcssat_rv_slot;
        let mut var_gmobmusat: f64 = *var_gmobmusat_slot;
        let mut var_gmobmusat_dn5: f64 = *var_gmobmusat_dn5_slot;
        let mut var_gmobmusat_dn6: f64 = *var_gmobmusat_dn6_slot;
        let mut var_gmobmusat_dn7: f64 = *var_gmobmusat_dn7_slot;
        let mut var_gmobmusat_dn8: f64 = *var_gmobmusat_dn8_slot;
        let mut var_gmobmusat_rv: f64 = *var_gmobmusat_rv_slot;
        let mut var_grsat: f64 = *var_grsat_slot;
        let mut var_grsat_dn5: f64 = *var_grsat_dn5_slot;
        let mut var_grsat_dn6: f64 = *var_grsat_dn6_slot;
        let mut var_grsat_dn7: f64 = *var_grsat_dn7_slot;
        let mut var_grsat_dn8: f64 = *var_grsat_dn8_slot;
        let mut var_grsat_rv: f64 = *var_grsat_rv_slot;
        let mut var_guard1199: f64 = *var_guard1199_slot;
        let mut var_guard1199_rv: f64 = *var_guard1199_rv_slot;
        let mut var_guard1200: f64 = *var_guard1200_slot;
        let mut var_guard1200_rv: f64 = *var_guard1200_rv_slot;
        let mut var_guard1201: f64 = *var_guard1201_slot;
        let mut var_guard1201_rv: f64 = *var_guard1201_rv_slot;
        let mut var_guard1202: f64 = *var_guard1202_slot;
        let mut var_guard1202_rv: f64 = *var_guard1202_rv_slot;
        let mut var_guard1203: f64 = *var_guard1203_slot;
        let mut var_guard1203_rv: f64 = *var_guard1203_rv_slot;
        let mut var_midphi0: f64 = *var_midphi0_slot;
        let mut var_midphi0_dn5: f64 = *var_midphi0_dn5_slot;
        let mut var_midphi0_dn6: f64 = *var_midphi0_dn6_slot;
        let mut var_midphi0_dn7: f64 = *var_midphi0_dn7_slot;
        let mut var_midphi0_dn8: f64 = *var_midphi0_dn8_slot;
        let mut var_midphi0_rv: f64 = *var_midphi0_rv_slot;
        let mut var_qbsat: f64 = *var_qbsat_slot;
        let mut var_qbsat_dn5: f64 = *var_qbsat_dn5_slot;
        let mut var_qbsat_dn6: f64 = *var_qbsat_dn6_slot;
        let mut var_qbsat_dn7: f64 = *var_qbsat_dn7_slot;
        let mut var_qbsat_dn8: f64 = *var_qbsat_dn8_slot;
        let mut var_qbsat_rv: f64 = *var_qbsat_rv_slot;
        let mut var_qisat: f64 = *var_qisat_slot;
        let mut var_qisat_dn5: f64 = *var_qisat_dn5_slot;
        let mut var_qisat_dn6: f64 = *var_qisat_dn6_slot;
        let mut var_qisat_dn7: f64 = *var_qisat_dn7_slot;
        let mut var_qisat_dn8: f64 = *var_qisat_dn8_slot;
        let mut var_qisat_rv: f64 = *var_qisat_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_x_inf: f64 = *var_x_inf_slot;
        let mut var_x_inf0: f64 = *var_x_inf0_slot;
        let mut var_x_inf0_dn5: f64 = *var_x_inf0_dn5_slot;
        let mut var_x_inf0_dn6: f64 = *var_x_inf0_dn6_slot;
        let mut var_x_inf0_dn7: f64 = *var_x_inf0_dn7_slot;
        let mut var_x_inf0_dn8: f64 = *var_x_inf0_dn8_slot;
        let mut var_x_inf0_rv: f64 = *var_x_inf0_rv_slot;
        let mut var_x_inf_dn5: f64 = *var_x_inf_dn5_slot;
        let mut var_x_inf_dn6: f64 = *var_x_inf_dn6_slot;
        let mut var_x_inf_dn7: f64 = *var_x_inf_dn7_slot;
        let mut var_x_inf_dn8: f64 = *var_x_inf_dn8_slot;
        let mut var_x_inf_rv: f64 = *var_x_inf_rv_slot;
        let mut var_ysat: f64 = *var_ysat_slot;
        let mut var_ysat_dn5: f64 = *var_ysat_dn5_slot;
        let mut var_ysat_dn6: f64 = *var_ysat_dn6_slot;
        let mut var_ysat_dn7: f64 = *var_ysat_dn7_slot;
        let mut var_ysat_dn8: f64 = *var_ysat_dn8_slot;
        let mut var_ysat_rv: f64 = *var_ysat_rv_slot;
        let mut var_za: f64 = *var_za_slot;
        let mut var_za_dn5: f64 = *var_za_dn5_slot;
        let mut var_za_dn6: f64 = *var_za_dn6_slot;
        let mut var_za_dn7: f64 = *var_za_dn7_slot;
        let mut var_za_dn8: f64 = *var_za_dn8_slot;
        let mut var_za_rv: f64 = *var_za_rv_slot;

        let (assign43620_e56360, assign43620_e56360_d_n5, assign43620_e56360_d_n6, assign43620_e56360_d_n7, assign43620_e56360_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43620_e56354: f64 = (var_gf2 * var_delta_1s);
        let __rspice_inv_cse_0: f64 = 1.0 / var_asat;
        let assign43620_e56356: f64 = (assign43620_e56354 * __rspice_inv_cse_0);
        let assign43620_e56358: f64 = (assign43620_e56356 * __rspice_inv_cse_0);
        (assign43620_e56358, ((((((((var_gf2_dn5 * var_delta_1s) + (var_gf2 * var_delta_1s_dn5)) * var_asat) - (assign43620_e56354 * var_asat_dn5)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn5)) / (var_asat * var_asat)), ((((((((var_gf2_dn6 * var_delta_1s) + (var_gf2 * var_delta_1s_dn6)) * var_asat) - (assign43620_e56354 * var_asat_dn6)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn6)) / (var_asat * var_asat)), ((((((((var_gf2_dn7 * var_delta_1s) + (var_gf2 * var_delta_1s_dn7)) * var_asat) - (assign43620_e56354 * var_asat_dn7)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn7)) / (var_asat * var_asat)), ((((((((var_gf2_dn8 * var_delta_1s) + (var_gf2 * var_delta_1s_dn8)) * var_asat) - (assign43620_e56354 * var_asat_dn8)) / (var_asat * var_asat)) * var_asat) - (assign43620_e56356 * var_asat_dn8)) / (var_asat * var_asat)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43620_e56360;
        var_temp__blk936_dn5 = assign43620_e56360_d_n5;
        var_temp__blk936_dn6 = assign43620_e56360_d_n6;
        var_temp__blk936_dn7 = assign43620_e56360_d_n7;
        var_temp__blk936_dn8 = assign43620_e56360_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign43630_e56363: f64 = if var_temp__blk936 > 0.0001 { 1.0 } else { 0.0 };
        var_guard1199 = assign43630_e56363;
        var_guard1199_rv = 0.0;

        let (assign43640_e56373, assign43640_e56373_d_n5, assign43640_e56373_d_n6, assign43640_e56373_d_n7, assign43640_e56373_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) {
        let assign43640_e56371: f64 = (1.0 - var_temp__blk936);
        (assign43640_e56371, (-var_temp__blk936_dn5), (-var_temp__blk936_dn6), (-var_temp__blk936_dn7), (-var_temp__blk936_dn8),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43640_e56373;
        var_temp1_dn5 = assign43640_e56373_d_n5;
        var_temp1_dn6 = assign43640_e56373_d_n6;
        var_temp1_dn7 = assign43640_e56373_d_n7;
        var_temp1_dn8 = assign43640_e56373_d_n8;
        var_temp1_rv = 0.0;

        let assign43650_e56376: f64 = if var_temp1 < 1e-10 { 1.0 } else { 0.0 };
        var_guard1200 = assign43650_e56376;
        var_guard1200_rv = 0.0;

        let (assign43660_e56386, assign43660_e56386_d_n5, assign43660_e56386_d_n6, assign43660_e56386_d_n7, assign43660_e56386_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) && (var_guard1200 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43660_e56386;
        var_temp2_dn5 = assign43660_e56386_d_n5;
        var_temp2_dn6 = assign43660_e56386_d_n6;
        var_temp2_dn7 = assign43660_e56386_d_n7;
        var_temp2_dn8 = assign43660_e56386_d_n8;
        var_temp2_rv = 0.0;

        let (assign43670_e56400, assign43670_e56400_d_n5, assign43670_e56400_d_n6, assign43670_e56400_d_n7, assign43670_e56400_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 != 0.0)) && (var_guard1200 == 0.0)) {
        let assign43670_e56397: f64 = (var_temp1).sqrt();
        let assign43670_e56398: f64 = (1.0 - assign43670_e56397);
        (assign43670_e56398, (-(var_temp1_dn5 / (2.0 * assign43670_e56397))), (-(var_temp1_dn6 / (2.0 * assign43670_e56397))), (-(var_temp1_dn7 / (2.0 * assign43670_e56397))), (-(var_temp1_dn8 / (2.0 * assign43670_e56397))),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43670_e56400;
        var_temp2_dn5 = assign43670_e56400_d_n5;
        var_temp2_dn6 = assign43670_e56400_d_n6;
        var_temp2_dn7 = assign43670_e56400_d_n7;
        var_temp2_dn8 = assign43670_e56400_d_n8;
        var_temp2_rv = 0.0;

        let (assign43680_e56411, assign43680_e56411_d_n5, assign43680_e56411_d_n6, assign43680_e56411_d_n7, assign43680_e56411_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1199 == 0.0)) {
        let assign43680_e56409: f64 = (0.5 * var_temp__blk936);
        (assign43680_e56409, (0.5 * var_temp__blk936_dn5), (0.5 * var_temp__blk936_dn6), (0.5 * var_temp__blk936_dn7), (0.5 * var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43680_e56411;
        var_temp2_dn5 = assign43680_e56411_d_n5;
        var_temp2_dn6 = assign43680_e56411_d_n6;
        var_temp2_dn7 = assign43680_e56411_d_n7;
        var_temp2_dn8 = assign43680_e56411_d_n8;
        var_temp2_rv = 0.0;

        let (assign43690_e56419, assign43690_e56419_d_n5, assign43690_e56419_d_n6, assign43690_e56419_d_n7, assign43690_e56419_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43690_e56417: f64 = (var_temp2 * var_asat);
        (assign43690_e56417, ((var_temp2_dn5 * var_asat) + (var_temp2 * var_asat_dn5)), ((var_temp2_dn6 * var_asat) + (var_temp2 * var_asat_dn6)), ((var_temp2_dn7 * var_asat) + (var_temp2 * var_asat_dn7)), ((var_temp2_dn8 * var_asat) + (var_temp2 * var_asat_dn8)),)
    } else {
        (var_x_inf0, var_x_inf0_dn5, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8,)
    }
};
        var_x_inf0 = assign43690_e56419;
        var_x_inf0_dn5 = assign43690_e56419_d_n5;
        var_x_inf0_dn6 = assign43690_e56419_d_n6;
        var_x_inf0_dn7 = assign43690_e56419_d_n7;
        var_x_inf0_dn8 = assign43690_e56419_d_n8;
        var_x_inf0_rv = 0.0;

        let assign43700_e56426: f64 = if ((var_cs_t > 0.0) && (var_thecs_t > 0.0)) { 1.0 } else { 0.0 };
        var_guard1201 = assign43700_e56426;
        var_guard1201_rv = 0.0;

        let (assign43710_e56438, assign43710_e56438_d_n5, assign43710_e56438_d_n6, assign43710_e56438_d_n7, assign43710_e56438_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43710_e56434: f64 = (0.475 * var_phit1);
        let assign43710_e56436: f64 = (assign43710_e56434 * var_x_inf0);
        (assign43710_e56436, (((0.475 * var_phit1_dn5) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn5)), (((0.475 * var_phit1_dn6) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn6)), (((0.475 * var_phit1_dn7) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn7)), (((0.475 * var_phit1_dn8) * var_x_inf0) + (assign43710_e56434 * var_x_inf0_dn8)),)
    } else {
        (var_midphi0, var_midphi0_dn5, var_midphi0_dn6, var_midphi0_dn7, var_midphi0_dn8,)
    }
};
        var_midphi0 = assign43710_e56438;
        var_midphi0_dn5 = assign43710_e56438_d_n5;
        var_midphi0_dn6 = assign43710_e56438_d_n6;
        var_midphi0_dn7 = assign43710_e56438_d_n7;
        var_midphi0_dn8 = assign43710_e56438_d_n8;
        var_midphi0_rv = 0.0;

        let (assign43720_e56450, assign43720_e56450_d_n5, assign43720_e56450_d_n6, assign43720_e56450_d_n7, assign43720_e56450_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43720_e56447: f64 = (var_alphas * var_midphi0);
        let assign43720_e56448: f64 = (var_qis - assign43720_e56447);
        (assign43720_e56448, (var_qis_dn5 - ((var_alphas_dn5 * var_midphi0) + (var_alphas * var_midphi0_dn5))), (var_qis_dn6 - ((var_alphas_dn6 * var_midphi0) + (var_alphas * var_midphi0_dn6))), (var_qis_dn7 - ((var_alphas_dn7 * var_midphi0) + (var_alphas * var_midphi0_dn7))), (var_qis_dn8 - ((var_alphas_dn8 * var_midphi0) + (var_alphas * var_midphi0_dn8))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43720_e56450;
        var_temp__blk936_dn5 = assign43720_e56450_d_n5;
        var_temp__blk936_dn6 = assign43720_e56450_d_n6;
        var_temp__blk936_dn7 = assign43720_e56450_d_n7;
        var_temp__blk936_dn8 = assign43720_e56450_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign43730_e56467, assign43730_e56467_d_n5, assign43730_e56467_d_n6, assign43730_e56467_d_n7, assign43730_e56467_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43730_e56460: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign43730_e56462: f64 = (assign43730_e56460 + 1e-12);
        let assign43730_e56463: f64 = (assign43730_e56462).sqrt();
        let assign43730_e56464: f64 = (var_temp__blk936 + assign43730_e56463);
        let assign43730_e56465: f64 = (0.5 * assign43730_e56464);
        (assign43730_e56465, (0.5 * (var_temp__blk936_dn5 + (((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn6 + (((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn7 + (((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) / (2.0 * assign43730_e56463)))), (0.5 * (var_temp__blk936_dn8 + (((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) / (2.0 * assign43730_e56463)))),)
    } else {
        (var_qisat, var_qisat_dn5, var_qisat_dn6, var_qisat_dn7, var_qisat_dn8,)
    }
};
        var_qisat = assign43730_e56467;
        var_qisat_dn5 = assign43730_e56467_d_n5;
        var_qisat_dn6 = assign43730_e56467_d_n6;
        var_qisat_dn7 = assign43730_e56467_d_n7;
        var_qisat_dn8 = assign43730_e56467_d_n8;
        var_qisat_rv = 0.0;

        let (assign43740_e56485, assign43740_e56485_d_n5, assign43740_e56485_d_n6, assign43740_e56485_d_n7, assign43740_e56485_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43740_e56475: f64 = (var_phit1 * var_xgs);
        let assign43740_e56477: f64 = (assign43740_e56475 - var_qis);
        let assign43740_e56480: f64 = (var_alphas - 1.0);
        let assign43740_e56482: f64 = (assign43740_e56480 * var_midphi0);
        let assign43740_e56483: f64 = (assign43740_e56477 + assign43740_e56482);
        (assign43740_e56483, ((((var_phit1_dn5 * var_xgs) + (var_phit1 * var_xgs_dn5)) - var_qis_dn5) + ((var_alphas_dn5 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn5))), ((((var_phit1_dn6 * var_xgs) + (var_phit1 * var_xgs_dn6)) - var_qis_dn6) + ((var_alphas_dn6 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn6))), ((((var_phit1_dn7 * var_xgs) + (var_phit1 * var_xgs_dn7)) - var_qis_dn7) + ((var_alphas_dn7 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn7))), ((((var_phit1_dn8 * var_xgs) + (var_phit1 * var_xgs_dn8)) - var_qis_dn8) + ((var_alphas_dn8 * var_midphi0) + (assign43740_e56480 * var_midphi0_dn8))),)
    } else {
        (var_qbsat, var_qbsat_dn5, var_qbsat_dn6, var_qbsat_dn7, var_qbsat_dn8,)
    }
};
        var_qbsat = assign43740_e56485;
        var_qbsat_dn5 = assign43740_e56485_d_n5;
        var_qbsat_dn6 = assign43740_e56485_d_n6;
        var_qbsat_dn7 = assign43740_e56485_d_n7;
        var_qbsat_dn8 = assign43740_e56485_d_n8;
        var_qbsat_rv = 0.0;

        let (assign43750_e56501, assign43750_e56501_d_n5, assign43750_e56501_d_n6, assign43750_e56501_d_n7, assign43750_e56501_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43750_e56494: f64 = (0.5 * var_gf2);
        let assign43750_e56496: f64 = (assign43750_e56494 * var_phit1);
        let assign43750_e56498: f64 = (assign43750_e56496 / var_qbsat);
        let assign43750_e56499: f64 = (1.0 + assign43750_e56498);
        (assign43750_e56499, ((((((0.5 * var_gf2_dn5) * var_phit1) + (assign43750_e56494 * var_phit1_dn5)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn5)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn6) * var_phit1) + (assign43750_e56494 * var_phit1_dn6)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn7) * var_phit1) + (assign43750_e56494 * var_phit1_dn7)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), ((((((0.5 * var_gf2_dn8) * var_phit1) + (assign43750_e56494 * var_phit1_dn8)) * var_qbsat) - (assign43750_e56496 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_alphasat, var_alphasat_dn5, var_alphasat_dn6, var_alphasat_dn7, var_alphasat_dn8,)
    }
};
        var_alphasat = assign43750_e56501;
        var_alphasat_dn5 = assign43750_e56501_d_n5;
        var_alphasat_dn6 = assign43750_e56501_d_n6;
        var_alphasat_dn7 = assign43750_e56501_d_n7;
        var_alphasat_dn8 = assign43750_e56501_d_n8;
        var_alphasat_rv = 0.0;

        let (assign43760_e56513, assign43760_e56513_d_n5, assign43760_e56513_d_n6, assign43760_e56513_d_n7, assign43760_e56513_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43760_e56510: f64 = (var_eta_mu * var_qisat);
        let assign43760_e56511: f64 = (var_qbsat + assign43760_e56510);
        (assign43760_e56511, (var_qbsat_dn5 + (var_eta_mu * var_qisat_dn5)), (var_qbsat_dn6 + (var_eta_mu * var_qisat_dn6)), (var_qbsat_dn7 + (var_eta_mu * var_qisat_dn7)), (var_qbsat_dn8 + (var_eta_mu * var_qisat_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43760_e56513;
        var_temp__blk936_dn5 = assign43760_e56513_d_n5;
        var_temp__blk936_dn6 = assign43760_e56513_d_n6;
        var_temp__blk936_dn7 = assign43760_e56513_d_n7;
        var_temp__blk936_dn8 = assign43760_e56513_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign43770_e56527, assign43770_e56527_d_n5, assign43770_e56527_d_n6, assign43770_e56527_d_n7, assign43770_e56527_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43770_e56521: f64 = (var_e_eff0 * var_temp__blk936);
        let assign43770_e56523: f64 = (assign43770_e56521 * var_mue_t);
        let assign43770_e56525: f64 = (assign43770_e56523).powf(var_themu_t);
        (assign43770_e56525, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn5) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn5) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn6) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn6) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn7) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn7) * var_mue_t) / assign43770_e56523))) }, if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign43770_e56523).powf(var_themu_t - 1.0) * ((var_e_eff0 * var_temp__blk936_dn8) * var_mue_t))) } } else { (assign43770_e56525 * (var_themu_t * (((var_e_eff0 * var_temp__blk936_dn8) * var_mue_t) / assign43770_e56523))) },)
    } else {
        (var_gmobmusat, var_gmobmusat_dn5, var_gmobmusat_dn6, var_gmobmusat_dn7, var_gmobmusat_dn8,)
    }
};
        var_gmobmusat = assign43770_e56527;
        var_gmobmusat_dn5 = assign43770_e56527_d_n5;
        var_gmobmusat_dn6 = assign43770_e56527_d_n6;
        var_gmobmusat_dn7 = assign43770_e56527_d_n7;
        var_gmobmusat_dn8 = assign43770_e56527_d_n8;
        var_gmobmusat_rv = 0.0;

        let (assign43780_e56547, assign43780_e56547_d_n5, assign43780_e56547_d_n6, assign43780_e56547_d_n7, assign43780_e56547_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43780_e56537: f64 = (1.0 - var_eta_mu);
        let assign43780_e56538: f64 = (var_alphasat * assign43780_e56537);
        let assign43780_e56540: f64 = (assign43780_e56538 - 1.0);
        let assign43780_e56541: f64 = (var_themu_t * assign43780_e56540);
        let assign43780_e56543: f64 = (assign43780_e56541 / var_temp__blk936);
        let assign43780_e56545: f64 = (assign43780_e56543 * var_gmobmusat);
        (assign43780_e56545, ((((((var_themu_t * (var_alphasat_dn5 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn5)), ((((((var_themu_t * (var_alphasat_dn6 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn6)), ((((((var_themu_t * (var_alphasat_dn7 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn7)), ((((((var_themu_t * (var_alphasat_dn8 * assign43780_e56537)) * var_temp__blk936) - (assign43780_e56541 * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936)) * var_gmobmusat) + (assign43780_e56543 * var_gmobmusat_dn8)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43780_e56547;
        var_temp1_dn5 = assign43780_e56547_d_n5;
        var_temp1_dn6 = assign43780_e56547_d_n6;
        var_temp1_dn7 = assign43780_e56547_d_n7;
        var_temp1_dn8 = assign43780_e56547_d_n8;
        var_temp1_rv = 0.0;

        let (assign43790_e56557, assign43790_e56557_d_n5, assign43790_e56557_d_n6, assign43790_e56557_d_n7, assign43790_e56557_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43790_e56555: f64 = (var_qisat / var_qbsat);
        (assign43790_e56555, (((var_qisat_dn5 * var_qbsat) - (var_qisat * var_qbsat_dn5)) / (var_qbsat * var_qbsat)), (((var_qisat_dn6 * var_qbsat) - (var_qisat * var_qbsat_dn6)) / (var_qbsat * var_qbsat)), (((var_qisat_dn7 * var_qbsat) - (var_qisat * var_qbsat_dn7)) / (var_qbsat * var_qbsat)), (((var_qisat_dn8 * var_qbsat) - (var_qisat * var_qbsat_dn8)) / (var_qbsat * var_qbsat)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43790_e56557;
        var_temp__blk936_dn5 = assign43790_e56557_d_n5;
        var_temp__blk936_dn6 = assign43790_e56557_d_n6;
        var_temp__blk936_dn7 = assign43790_e56557_d_n7;
        var_temp__blk936_dn8 = assign43790_e56557_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign43800_e56572, assign43800_e56572_d_n5, assign43800_e56572_d_n6, assign43800_e56572_d_n7, assign43800_e56572_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43800_e56566: f64 = (1.0 + var_temp__blk936);
        let assign43800_e56568: f64 = (-var_thecs_t);
        let assign43800_e56569: f64 = (assign43800_e56566).powf(assign43800_e56568);
        let assign43800_e56570: f64 = (var_cs_t * assign43800_e56569);
        (assign43800_e56570, (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn5)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn5 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn6)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn6 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn7)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn7 / assign43800_e56566))) }), (var_cs_t * if 0.0 == 0.0 && ((assign43800_e56568) as f64).is_finite() && ((assign43800_e56568) as f64).fract() == 0.0 { if assign43800_e56568 == 0.0 { 0.0 } else { (assign43800_e56568 * ((assign43800_e56566).powf(assign43800_e56568 - 1.0) * var_temp__blk936_dn8)) } } else { (assign43800_e56569 * (assign43800_e56568 * (var_temp__blk936_dn8 / assign43800_e56566))) }),)
    } else {
        (var_gmobcssat, var_gmobcssat_dn5, var_gmobcssat_dn6, var_gmobcssat_dn7, var_gmobcssat_dn8,)
    }
};
        var_gmobcssat = assign43800_e56572;
        var_gmobcssat_dn5 = assign43800_e56572_d_n5;
        var_gmobcssat_dn6 = assign43800_e56572_d_n6;
        var_gmobcssat_dn7 = assign43800_e56572_d_n7;
        var_gmobcssat_dn8 = assign43800_e56572_d_n8;
        var_gmobcssat_rv = 0.0;

        let (assign43810_e56594, assign43810_e56594_d_n5, assign43810_e56594_d_n6, assign43810_e56594_d_n7, assign43810_e56594_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43810_e56581: f64 = (var_alphasat - 1.0);
        let assign43810_e56585: f64 = (var_temp__blk936 + 1.0);
        let assign43810_e56586: f64 = (1.0 / assign43810_e56585);
        let assign43810_e56587: f64 = (assign43810_e56581 + assign43810_e56586);
        let assign43810_e56588: f64 = (var_thecs_t * assign43810_e56587);
        let assign43810_e56590: f64 = (assign43810_e56588 / var_qbsat);
        let assign43810_e56592: f64 = (assign43810_e56590 * var_gmobcssat);
        (assign43810_e56592, ((((((var_thecs_t * (var_alphasat_dn5 + (-(var_temp__blk936_dn5 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn5)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn5)), ((((((var_thecs_t * (var_alphasat_dn6 + (-(var_temp__blk936_dn6 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn6)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn6)), ((((((var_thecs_t * (var_alphasat_dn7 + (-(var_temp__blk936_dn7 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn7)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn7)), ((((((var_thecs_t * (var_alphasat_dn8 + (-(var_temp__blk936_dn8 / (assign43810_e56585 * assign43810_e56585))))) * var_qbsat) - (assign43810_e56588 * var_qbsat_dn8)) / (var_qbsat * var_qbsat)) * var_gmobcssat) + (assign43810_e56590 * var_gmobcssat_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign43810_e56594;
        var_temp2_dn5 = assign43810_e56594_d_n5;
        var_temp2_dn6 = assign43810_e56594_d_n6;
        var_temp2_dn7 = assign43810_e56594_d_n7;
        var_temp2_dn8 = assign43810_e56594_d_n8;
        var_temp2_rv = 0.0;

        let (assign43820_e56608, assign43820_e56608_d_n5, assign43820_e56608_d_n6, assign43820_e56608_d_n7, assign43820_e56608_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43820_e56602: f64 = (var_ther_i * var_rhob);
        let assign43820_e56604: f64 = (assign43820_e56602 * var_rhog);
        let assign43820_e56606: f64 = (assign43820_e56604 * var_qisat);
        (assign43820_e56606, (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign43820_e56602 * var_rhog_dn5)) * var_qisat) + (assign43820_e56604 * var_qisat_dn5)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign43820_e56602 * var_rhog_dn6)) * var_qisat) + (assign43820_e56604 * var_qisat_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign43820_e56602 * var_rhog_dn7)) * var_qisat) + (assign43820_e56604 * var_qisat_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign43820_e56602 * var_rhog_dn8)) * var_qisat) + (assign43820_e56604 * var_qisat_dn8)),)
    } else {
        (var_grsat, var_grsat_dn5, var_grsat_dn6, var_grsat_dn7, var_grsat_dn8,)
    }
};
        var_grsat = assign43820_e56608;
        var_grsat_dn5 = assign43820_e56608_d_n5;
        var_grsat_dn6 = assign43820_e56608_d_n6;
        var_grsat_dn7 = assign43820_e56608_d_n7;
        var_grsat_dn8 = assign43820_e56608_d_n8;
        var_grsat_rv = 0.0;

        let (assign43830_e56628, assign43830_e56628_d_n5, assign43830_e56628_d_n6, assign43830_e56628_d_n7, assign43830_e56628_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43830_e56618: f64 = (var_ther_i * var_rhob);
        let assign43830_e56620: f64 = (assign43830_e56618 * var_rhog);
        let assign43830_e56622: f64 = (assign43830_e56620 * var_alphasat);
        let assign43830_e56623: f64 = (var_temp1 - assign43830_e56622);
        let assign43830_e56625: f64 = (assign43830_e56623 / var_temp2);
        let assign43830_e56626: f64 = (1.0 + assign43830_e56625);
        (assign43830_e56626, ((((var_temp1_dn5 - (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign43830_e56618 * var_rhog_dn5)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn5))) * var_temp2) - (assign43830_e56623 * var_temp2_dn5)) / (var_temp2 * var_temp2)), ((((var_temp1_dn6 - (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign43830_e56618 * var_rhog_dn6)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn6))) * var_temp2) - (assign43830_e56623 * var_temp2_dn6)) / (var_temp2 * var_temp2)), ((((var_temp1_dn7 - (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign43830_e56618 * var_rhog_dn7)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn7))) * var_temp2) - (assign43830_e56623 * var_temp2_dn7)) / (var_temp2 * var_temp2)), ((((var_temp1_dn8 - (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign43830_e56618 * var_rhog_dn8)) * var_alphasat) + (assign43830_e56620 * var_alphasat_dn8))) * var_temp2) - (assign43830_e56623 * var_temp2_dn8)) / (var_temp2 * var_temp2)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43830_e56628;
        var_temp__blk936_dn5 = assign43830_e56628_d_n5;
        var_temp__blk936_dn6 = assign43830_e56628_d_n6;
        var_temp__blk936_dn7 = assign43830_e56628_d_n7;
        var_temp__blk936_dn8 = assign43830_e56628_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign43840_e56631: f64 = if var_temp__blk936 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1202 = assign43840_e56631;
        var_guard1202_rv = 0.0;

        let (assign43850_e56649, assign43850_e56649_d_n5, assign43850_e56649_d_n6, assign43850_e56649_d_n7, assign43850_e56649_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) && (var_guard1202 != 0.0)) {
        let assign43850_e56643: f64 = (2.0 * var_temp__blk936);
        let assign43850_e56644: f64 = (assign43850_e56643).exp();
        let assign43850_e56645: f64 = (1.0 + assign43850_e56644);
        let assign43850_e56646: f64 = (assign43850_e56645).ln();
        let assign43850_e56647: f64 = (0.5 * assign43850_e56646);
        (assign43850_e56647, (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn5)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn6)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn7)) / assign43850_e56645)), (0.5 * ((assign43850_e56644 * (2.0 * var_temp__blk936_dn8)) / assign43850_e56645)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43850_e56649;
        var_temp1_dn5 = assign43850_e56649_d_n5;
        var_temp1_dn6 = assign43850_e56649_d_n6;
        var_temp1_dn7 = assign43850_e56649_d_n7;
        var_temp1_dn8 = assign43850_e56649_d_n8;
        var_temp1_rv = 0.0;

        let (assign43860_e56660, assign43860_e56660_d_n5, assign43860_e56660_d_n6, assign43860_e56660_d_n7, assign43860_e56660_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) && (var_guard1202 == 0.0)) {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign43860_e56660;
        var_temp1_dn5 = assign43860_e56660_d_n5;
        var_temp1_dn6 = assign43860_e56660_d_n6;
        var_temp1_dn7 = assign43860_e56660_d_n7;
        var_temp1_dn8 = assign43860_e56660_d_n8;
        var_temp1_rv = 0.0;

        let (assign43870_e56681, assign43870_e56681_d_n5, assign43870_e56681_d_n6, assign43870_e56681_d_n7, assign43870_e56681_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43870_e56667: f64 = (-var_midphi0);
        let assign43870_e56669: f64 = (assign43870_e56667 * var_temp2);
        let assign43870_e56671: f64 = (assign43870_e56669 * var_temp1);
        let assign43870_e56674: f64 = (1.0 + var_gmobmusat);
        let assign43870_e56676: f64 = (assign43870_e56674 + var_gmobcssat);
        let assign43870_e56678: f64 = (assign43870_e56676 + var_grsat);
        let assign43870_e56679: f64 = (assign43870_e56671 / assign43870_e56678);
        (assign43870_e56679, ((((((((-var_midphi0_dn5) * var_temp2) + (assign43870_e56667 * var_temp2_dn5)) * var_temp1) + (assign43870_e56669 * var_temp1_dn5)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn5 + var_gmobcssat_dn5) + var_grsat_dn5))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn6) * var_temp2) + (assign43870_e56667 * var_temp2_dn6)) * var_temp1) + (assign43870_e56669 * var_temp1_dn6)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn6 + var_gmobcssat_dn6) + var_grsat_dn6))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn7) * var_temp2) + (assign43870_e56667 * var_temp2_dn7)) * var_temp1) + (assign43870_e56669 * var_temp1_dn7)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn7 + var_gmobcssat_dn7) + var_grsat_dn7))) / (assign43870_e56678 * assign43870_e56678)), ((((((((-var_midphi0_dn8) * var_temp2) + (assign43870_e56667 * var_temp2_dn8)) * var_temp1) + (assign43870_e56669 * var_temp1_dn8)) * assign43870_e56678) - (assign43870_e56671 * ((var_gmobmusat_dn8 + var_gmobcssat_dn8) + var_grsat_dn8))) / (assign43870_e56678 * assign43870_e56678)),)
    } else {
        (var_delta_gmob, var_delta_gmob_dn5, var_delta_gmob_dn6, var_delta_gmob_dn7, var_delta_gmob_dn8,)
    }
};
        var_delta_gmob = assign43870_e56681;
        var_delta_gmob_dn5 = assign43870_e56681_d_n5;
        var_delta_gmob_dn6 = assign43870_e56681_d_n6;
        var_delta_gmob_dn7 = assign43870_e56681_d_n7;
        var_delta_gmob_dn8 = assign43870_e56681_d_n8;
        var_delta_gmob_rv = 0.0;

        let (assign43880_e56702, assign43880_e56702_d_n5, assign43880_e56702_d_n6, assign43880_e56702_d_n7, assign43880_e56702_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 != 0.0)) {
        let assign43880_e56694: f64 = (var_delta_gmob * var_delta_gmob);
        let assign43880_e56695: f64 = (1.0 + assign43880_e56694);
        let assign43880_e56696: f64 = (assign43880_e56695).sqrt();
        let assign43880_e56697: f64 = (1.0 + assign43880_e56696);
        let assign43880_e56698: f64 = (var_delta_gmob / assign43880_e56697);
        let assign43880_e56699: f64 = (1.0 + assign43880_e56698);
        let assign43880_e56700: f64 = (var_x_inf0 * assign43880_e56699);
        (assign43880_e56700, ((var_x_inf0_dn5 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn5 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn5 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn5)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn6 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn6 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn6 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn6)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn7 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn7 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn7 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn7)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))), ((var_x_inf0_dn8 * assign43880_e56699) + (var_x_inf0 * (((var_delta_gmob_dn8 * assign43880_e56697) - (var_delta_gmob * (((var_delta_gmob_dn8 * var_delta_gmob) + (var_delta_gmob * var_delta_gmob_dn8)) / (2.0 * assign43880_e56696)))) / (assign43880_e56697 * assign43880_e56697)))),)
    } else {
        (var_x_inf, var_x_inf_dn5, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8,)
    }
};
        var_x_inf = assign43880_e56702;
        var_x_inf_dn5 = assign43880_e56702_d_n5;
        var_x_inf_dn6 = assign43880_e56702_d_n6;
        var_x_inf_dn7 = assign43880_e56702_d_n7;
        var_x_inf_dn8 = assign43880_e56702_d_n8;
        var_x_inf_rv = 0.0;

        let (assign43890_e56711, assign43890_e56711_d_n5, assign43890_e56711_d_n6, assign43890_e56711_d_n7, assign43890_e56711_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1201 == 0.0)) {
        (var_x_inf0, var_x_inf0_dn5, var_x_inf0_dn6, var_x_inf0_dn7, var_x_inf0_dn8,)
    } else {
        (var_x_inf, var_x_inf_dn5, var_x_inf_dn6, var_x_inf_dn7, var_x_inf_dn8,)
    }
};
        var_x_inf = assign43890_e56711;
        var_x_inf_dn5 = assign43890_e56711_d_n5;
        var_x_inf_dn6 = assign43890_e56711_d_n6;
        var_x_inf_dn7 = assign43890_e56711_d_n7;
        var_x_inf_dn8 = assign43890_e56711_d_n8;
        var_x_inf_rv = 0.0;

        let (assign43900_e56723, assign43900_e56723_d_n5, assign43900_e56723_d_n6, assign43900_e56723_d_n7, assign43900_e56723_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43900_e56717: f64 = (var_phit1 * var_thesat1);
        let assign43900_e56719: f64 = (assign43900_e56717 * var_x_inf);
        let assign43900_e56721: f64 = (assign43900_e56719 * 0.7071067811865475);
        (assign43900_e56721, (((((var_phit1_dn5 * var_thesat1) + (var_phit1 * var_thesat1_dn5)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn5)) * 0.7071067811865475), (((((var_phit1_dn6 * var_thesat1) + (var_phit1 * var_thesat1_dn6)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn6)) * 0.7071067811865475), (((((var_phit1_dn7 * var_thesat1) + (var_phit1 * var_thesat1_dn7)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn7)) * 0.7071067811865475), (((((var_phit1_dn8 * var_thesat1) + (var_phit1 * var_thesat1_dn8)) * var_x_inf) + (assign43900_e56717 * var_x_inf_dn8)) * 0.7071067811865475),)
    } else {
        (var_ysat, var_ysat_dn5, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8,)
    }
};
        var_ysat = assign43900_e56723;
        var_ysat_dn5 = assign43900_e56723_d_n5;
        var_ysat_dn6 = assign43900_e56723_d_n6;
        var_ysat_dn7 = assign43900_e56723_d_n7;
        var_ysat_dn8 = assign43900_e56723_d_n8;
        var_ysat_rv = 0.0;

        let assign43910_e56726: f64 = (-1.0);
        let assign43910_e56727: f64 = if var_chnl_type == assign43910_e56726 { 1.0 } else { 0.0 };
        var_guard1203 = assign43910_e56727;
        var_guard1203_rv = 0.0;

        let (assign43920_e56740, assign43920_e56740_d_n5, assign43920_e56740_d_n6, assign43920_e56740_d_n7, assign43920_e56740_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) && (var_guard1203 != 0.0)) {
        let assign43920_e56736: f64 = (1.0 + var_ysat);
        let assign43920_e56737: f64 = (assign43920_e56736).sqrt();
        let assign43920_e56738: f64 = (var_ysat / assign43920_e56737);
        (assign43920_e56738, (((var_ysat_dn5 * assign43920_e56737) - (var_ysat * (var_ysat_dn5 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn6 * assign43920_e56737) - (var_ysat * (var_ysat_dn6 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn7 * assign43920_e56737) - (var_ysat * (var_ysat_dn7 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)), (((var_ysat_dn8 * assign43920_e56737) - (var_ysat * (var_ysat_dn8 / (2.0 * assign43920_e56737)))) / (assign43920_e56737 * assign43920_e56737)),)
    } else {
        (var_ysat, var_ysat_dn5, var_ysat_dn6, var_ysat_dn7, var_ysat_dn8,)
    }
};
        var_ysat = assign43920_e56740;
        var_ysat_dn5 = assign43920_e56740_d_n5;
        var_ysat_dn6 = assign43920_e56740_d_n6;
        var_ysat_dn7 = assign43920_e56740_d_n7;
        var_ysat_dn8 = assign43920_e56740_d_n8;
        var_ysat_rv = 0.0;

        let (assign43930_e56755, assign43930_e56755_d_n5, assign43930_e56755_d_n6, assign43930_e56755_d_n7, assign43930_e56755_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43930_e56749: f64 = (4.0 * var_ysat);
        let assign43930_e56750: f64 = (1.0 + assign43930_e56749);
        let assign43930_e56751: f64 = (assign43930_e56750).sqrt();
        let assign43930_e56752: f64 = (1.0 + assign43930_e56751);
        let assign43930_e56753: f64 = (2.0 / assign43930_e56752);
        (assign43930_e56753, (-((2.0 * ((4.0 * var_ysat_dn5) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn6) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn7) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))), (-((2.0 * ((4.0 * var_ysat_dn8) / (2.0 * assign43930_e56751))) / (assign43930_e56752 * assign43930_e56752))),)
    } else {
        (var_za, var_za_dn5, var_za_dn6, var_za_dn7, var_za_dn8,)
    }
};
        var_za = assign43930_e56755;
        var_za_dn5 = assign43930_e56755_d_n5;
        var_za_dn6 = assign43930_e56755_d_n6;
        var_za_dn7 = assign43930_e56755_d_n7;
        var_za_dn8 = assign43930_e56755_d_n8;
        var_za_rv = 0.0;

        let (assign43940_e56763, assign43940_e56763_d_n5, assign43940_e56763_d_n6, assign43940_e56763_d_n7, assign43940_e56763_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43940_e56761: f64 = (var_za * var_ysat);
        (assign43940_e56761, ((var_za_dn5 * var_ysat) + (var_za * var_ysat_dn5)), ((var_za_dn6 * var_ysat) + (var_za * var_ysat_dn6)), ((var_za_dn7 * var_ysat) + (var_za * var_ysat_dn7)), ((var_za_dn8 * var_ysat) + (var_za * var_ysat_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43940_e56763;
        var_temp__blk936_dn5 = assign43940_e56763_d_n5;
        var_temp__blk936_dn6 = assign43940_e56763_d_n6;
        var_temp__blk936_dn7 = assign43940_e56763_d_n7;
        var_temp__blk936_dn8 = assign43940_e56763_d_n8;
        var_temp__blk936_rv = 0.0;

        *var_alphasat_slot = var_alphasat;
        *var_alphasat_dn5_slot = var_alphasat_dn5;
        *var_alphasat_dn6_slot = var_alphasat_dn6;
        *var_alphasat_dn7_slot = var_alphasat_dn7;
        *var_alphasat_dn8_slot = var_alphasat_dn8;
        *var_alphasat_rv_slot = var_alphasat_rv;
        *var_delta_gmob_slot = var_delta_gmob;
        *var_delta_gmob_dn5_slot = var_delta_gmob_dn5;
        *var_delta_gmob_dn6_slot = var_delta_gmob_dn6;
        *var_delta_gmob_dn7_slot = var_delta_gmob_dn7;
        *var_delta_gmob_dn8_slot = var_delta_gmob_dn8;
        *var_delta_gmob_rv_slot = var_delta_gmob_rv;
        *var_gmobcssat_slot = var_gmobcssat;
        *var_gmobcssat_dn5_slot = var_gmobcssat_dn5;
        *var_gmobcssat_dn6_slot = var_gmobcssat_dn6;
        *var_gmobcssat_dn7_slot = var_gmobcssat_dn7;
        *var_gmobcssat_dn8_slot = var_gmobcssat_dn8;
        *var_gmobcssat_rv_slot = var_gmobcssat_rv;
        *var_gmobmusat_slot = var_gmobmusat;
        *var_gmobmusat_dn5_slot = var_gmobmusat_dn5;
        *var_gmobmusat_dn6_slot = var_gmobmusat_dn6;
        *var_gmobmusat_dn7_slot = var_gmobmusat_dn7;
        *var_gmobmusat_dn8_slot = var_gmobmusat_dn8;
        *var_gmobmusat_rv_slot = var_gmobmusat_rv;
        *var_grsat_slot = var_grsat;
        *var_grsat_dn5_slot = var_grsat_dn5;
        *var_grsat_dn6_slot = var_grsat_dn6;
        *var_grsat_dn7_slot = var_grsat_dn7;
        *var_grsat_dn8_slot = var_grsat_dn8;
        *var_grsat_rv_slot = var_grsat_rv;
        *var_guard1199_slot = var_guard1199;
        *var_guard1199_rv_slot = var_guard1199_rv;
        *var_guard1200_slot = var_guard1200;
        *var_guard1200_rv_slot = var_guard1200_rv;
        *var_guard1201_slot = var_guard1201;
        *var_guard1201_rv_slot = var_guard1201_rv;
        *var_guard1202_slot = var_guard1202;
        *var_guard1202_rv_slot = var_guard1202_rv;
        *var_guard1203_slot = var_guard1203;
        *var_guard1203_rv_slot = var_guard1203_rv;
        *var_midphi0_slot = var_midphi0;
        *var_midphi0_dn5_slot = var_midphi0_dn5;
        *var_midphi0_dn6_slot = var_midphi0_dn6;
        *var_midphi0_dn7_slot = var_midphi0_dn7;
        *var_midphi0_dn8_slot = var_midphi0_dn8;
        *var_midphi0_rv_slot = var_midphi0_rv;
        *var_qbsat_slot = var_qbsat;
        *var_qbsat_dn5_slot = var_qbsat_dn5;
        *var_qbsat_dn6_slot = var_qbsat_dn6;
        *var_qbsat_dn7_slot = var_qbsat_dn7;
        *var_qbsat_dn8_slot = var_qbsat_dn8;
        *var_qbsat_rv_slot = var_qbsat_rv;
        *var_qisat_slot = var_qisat;
        *var_qisat_dn5_slot = var_qisat_dn5;
        *var_qisat_dn6_slot = var_qisat_dn6;
        *var_qisat_dn7_slot = var_qisat_dn7;
        *var_qisat_dn8_slot = var_qisat_dn8;
        *var_qisat_rv_slot = var_qisat_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_x_inf_slot = var_x_inf;
        *var_x_inf0_slot = var_x_inf0;
        *var_x_inf0_dn5_slot = var_x_inf0_dn5;
        *var_x_inf0_dn6_slot = var_x_inf0_dn6;
        *var_x_inf0_dn7_slot = var_x_inf0_dn7;
        *var_x_inf0_dn8_slot = var_x_inf0_dn8;
        *var_x_inf0_rv_slot = var_x_inf0_rv;
        *var_x_inf_dn5_slot = var_x_inf_dn5;
        *var_x_inf_dn6_slot = var_x_inf_dn6;
        *var_x_inf_dn7_slot = var_x_inf_dn7;
        *var_x_inf_dn8_slot = var_x_inf_dn8;
        *var_x_inf_rv_slot = var_x_inf_rv;
        *var_ysat_slot = var_ysat;
        *var_ysat_dn5_slot = var_ysat_dn5;
        *var_ysat_dn6_slot = var_ysat_dn6;
        *var_ysat_dn7_slot = var_ysat_dn7;
        *var_ysat_dn8_slot = var_ysat_dn8;
        *var_ysat_rv_slot = var_ysat_rv;
        *var_za_slot = var_za;
        *var_za_dn5_slot = var_za_dn5;
        *var_za_dn6_slot = var_za_dn6;
        *var_za_dn7_slot = var_za_dn7;
        *var_za_dn8_slot = var_za_dn8;
        *var_za_rv_slot = var_za_rv;
    }

    pub(super) fn stamp_reactive_block_32(
        var_arloc: f64,
        var_asat: f64,
        var_asat_dn5: f64,
        var_asat_dn6: f64,
        var_asat_dn7: f64,
        var_asat_dn8: f64,
        var_delta_ns: f64,
        var_delta_ns_dn5: f64,
        var_delta_ns_dn6: f64,
        var_delta_ns_dn7: f64,
        var_delta_ns_dn8: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1197: f64,
        var_guard1198: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_inv_phit1: f64,
        var_inv_phit1_dn5: f64,
        var_inv_phit1_dn6: f64,
        var_inv_phit1_dn7: f64,
        var_inv_phit1_dn8: f64,
        var_inv_xi: f64,
        var_inv_xi_dn5: f64,
        var_inv_xi_dn6: f64,
        var_inv_xi_dn7: f64,
        var_inv_xi_dn8: f64,
        var_margin: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_sp_s_x1: f64,
        var_sp_s_x1_dn5: f64,
        var_sp_s_x1_dn6: f64,
        var_sp_s_x1_dn7: f64,
        var_sp_s_x1_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsat_lim: f64,
        var_vdsat_lim_dn5: f64,
        var_vdsat_lim_dn6: f64,
        var_vdsat_lim_dn7: f64,
        var_vdsat_lim_dn8: f64,
        var_x_inf: f64,
        var_x_inf_dn5: f64,
        var_x_inf_dn6: f64,
        var_x_inf_dn7: f64,
        var_x_inf_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xn_s: f64,
        var_xn_s_dn5: f64,
        var_xn_s_dn6: f64,
        var_xn_s_dn7: f64,
        var_xn_s_dn8: f64,
        var_za: f64,
        var_za_dn5: f64,
        var_za_dn6: f64,
        var_za_dn7: f64,
        var_za_dn8: f64,
        var_delta_nd_slot: &mut f64,
        var_delta_nd_dn5_slot: &mut f64,
        var_delta_nd_dn6_slot: &mut f64,
        var_delta_nd_dn7_slot: &mut f64,
        var_delta_nd_dn8_slot: &mut f64,
        var_delta_nd_rv_slot: &mut f64,
        var_guard1204_slot: &mut f64,
        var_guard1204_rv_slot: &mut f64,
        var_guard1205_slot: &mut f64,
        var_guard1205_rv_slot: &mut f64,
        var_k_ds_slot: &mut f64,
        var_k_ds_dn5_slot: &mut f64,
        var_k_ds_dn6_slot: &mut f64,
        var_k_ds_dn7_slot: &mut f64,
        var_k_ds_dn8_slot: &mut f64,
        var_k_ds_rv_slot: &mut f64,
        var_sp_s_a_slot: &mut f64,
        var_sp_s_a_dn5_slot: &mut f64,
        var_sp_s_a_dn6_slot: &mut f64,
        var_sp_s_a_dn7_slot: &mut f64,
        var_sp_s_a_dn8_slot: &mut f64,
        var_sp_s_a_rv_slot: &mut f64,
        var_sp_s_bx_slot: &mut f64,
        var_sp_s_bx_dn5_slot: &mut f64,
        var_sp_s_bx_dn6_slot: &mut f64,
        var_sp_s_bx_dn7_slot: &mut f64,
        var_sp_s_bx_dn8_slot: &mut f64,
        var_sp_s_bx_rv_slot: &mut f64,
        var_sp_s_eta_slot: &mut f64,
        var_sp_s_eta_dn5_slot: &mut f64,
        var_sp_s_eta_dn6_slot: &mut f64,
        var_sp_s_eta_dn7_slot: &mut f64,
        var_sp_s_eta_dn8_slot: &mut f64,
        var_sp_s_eta_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp1_slot: &mut f64,
        var_sp_s_temp1_dn5_slot: &mut f64,
        var_sp_s_temp1_dn6_slot: &mut f64,
        var_sp_s_temp1_dn7_slot: &mut f64,
        var_sp_s_temp1_dn8_slot: &mut f64,
        var_sp_s_temp1_rv_slot: &mut f64,
        var_sp_s_temp2_slot: &mut f64,
        var_sp_s_temp2_dn5_slot: &mut f64,
        var_sp_s_temp2_dn6_slot: &mut f64,
        var_sp_s_temp2_dn7_slot: &mut f64,
        var_sp_s_temp2_dn8_slot: &mut f64,
        var_sp_s_temp2_rv_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_rv_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_rv_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_udse_slot: &mut f64,
        var_udse_dn5_slot: &mut f64,
        var_udse_dn6_slot: &mut f64,
        var_udse_dn7_slot: &mut f64,
        var_udse_dn8_slot: &mut f64,
        var_udse_rv_slot: &mut f64,
        var_v_dsat_slot: &mut f64,
        var_v_dsat_dn5_slot: &mut f64,
        var_v_dsat_dn6_slot: &mut f64,
        var_v_dsat_dn7_slot: &mut f64,
        var_v_dsat_dn8_slot: &mut f64,
        var_v_dsat_rv_slot: &mut f64,
        var_vdse_slot: &mut f64,
        var_vdse_dn5_slot: &mut f64,
        var_vdse_dn6_slot: &mut f64,
        var_vdse_dn7_slot: &mut f64,
        var_vdse_dn8_slot: &mut f64,
        var_vdse_rv_slot: &mut f64,
        var_x_0_slot: &mut f64,
        var_x_0_dn5_slot: &mut f64,
        var_x_0_dn6_slot: &mut f64,
        var_x_0_dn7_slot: &mut f64,
        var_x_0_dn8_slot: &mut f64,
        var_x_0_rv_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_d_rv_slot: &mut f64,
        var_x_sat_slot: &mut f64,
        var_x_sat_dn5_slot: &mut f64,
        var_x_sat_dn6_slot: &mut f64,
        var_x_sat_dn7_slot: &mut f64,
        var_x_sat_dn8_slot: &mut f64,
        var_x_sat_rv_slot: &mut f64,
        var_xn_d_slot: &mut f64,
        var_xn_d_dn5_slot: &mut f64,
        var_xn_d_dn6_slot: &mut f64,
        var_xn_d_dn7_slot: &mut f64,
        var_xn_d_dn8_slot: &mut f64,
        var_xn_d_rv_slot: &mut f64,
    ) {
        let mut var_delta_nd: f64 = *var_delta_nd_slot;
        let mut var_delta_nd_dn5: f64 = *var_delta_nd_dn5_slot;
        let mut var_delta_nd_dn6: f64 = *var_delta_nd_dn6_slot;
        let mut var_delta_nd_dn7: f64 = *var_delta_nd_dn7_slot;
        let mut var_delta_nd_dn8: f64 = *var_delta_nd_dn8_slot;
        let mut var_delta_nd_rv: f64 = *var_delta_nd_rv_slot;
        let mut var_guard1204: f64 = *var_guard1204_slot;
        let mut var_guard1204_rv: f64 = *var_guard1204_rv_slot;
        let mut var_guard1205: f64 = *var_guard1205_slot;
        let mut var_guard1205_rv: f64 = *var_guard1205_rv_slot;
        let mut var_k_ds: f64 = *var_k_ds_slot;
        let mut var_k_ds_dn5: f64 = *var_k_ds_dn5_slot;
        let mut var_k_ds_dn6: f64 = *var_k_ds_dn6_slot;
        let mut var_k_ds_dn7: f64 = *var_k_ds_dn7_slot;
        let mut var_k_ds_dn8: f64 = *var_k_ds_dn8_slot;
        let mut var_k_ds_rv: f64 = *var_k_ds_rv_slot;
        let mut var_sp_s_a: f64 = *var_sp_s_a_slot;
        let mut var_sp_s_a_dn5: f64 = *var_sp_s_a_dn5_slot;
        let mut var_sp_s_a_dn6: f64 = *var_sp_s_a_dn6_slot;
        let mut var_sp_s_a_dn7: f64 = *var_sp_s_a_dn7_slot;
        let mut var_sp_s_a_dn8: f64 = *var_sp_s_a_dn8_slot;
        let mut var_sp_s_a_rv: f64 = *var_sp_s_a_rv_slot;
        let mut var_sp_s_bx: f64 = *var_sp_s_bx_slot;
        let mut var_sp_s_bx_dn5: f64 = *var_sp_s_bx_dn5_slot;
        let mut var_sp_s_bx_dn6: f64 = *var_sp_s_bx_dn6_slot;
        let mut var_sp_s_bx_dn7: f64 = *var_sp_s_bx_dn7_slot;
        let mut var_sp_s_bx_dn8: f64 = *var_sp_s_bx_dn8_slot;
        let mut var_sp_s_bx_rv: f64 = *var_sp_s_bx_rv_slot;
        let mut var_sp_s_eta: f64 = *var_sp_s_eta_slot;
        let mut var_sp_s_eta_dn5: f64 = *var_sp_s_eta_dn5_slot;
        let mut var_sp_s_eta_dn6: f64 = *var_sp_s_eta_dn6_slot;
        let mut var_sp_s_eta_dn7: f64 = *var_sp_s_eta_dn7_slot;
        let mut var_sp_s_eta_dn8: f64 = *var_sp_s_eta_dn8_slot;
        let mut var_sp_s_eta_rv: f64 = *var_sp_s_eta_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp1: f64 = *var_sp_s_temp1_slot;
        let mut var_sp_s_temp1_dn5: f64 = *var_sp_s_temp1_dn5_slot;
        let mut var_sp_s_temp1_dn6: f64 = *var_sp_s_temp1_dn6_slot;
        let mut var_sp_s_temp1_dn7: f64 = *var_sp_s_temp1_dn7_slot;
        let mut var_sp_s_temp1_dn8: f64 = *var_sp_s_temp1_dn8_slot;
        let mut var_sp_s_temp1_rv: f64 = *var_sp_s_temp1_rv_slot;
        let mut var_sp_s_temp2: f64 = *var_sp_s_temp2_slot;
        let mut var_sp_s_temp2_dn5: f64 = *var_sp_s_temp2_dn5_slot;
        let mut var_sp_s_temp2_dn6: f64 = *var_sp_s_temp2_dn6_slot;
        let mut var_sp_s_temp2_dn7: f64 = *var_sp_s_temp2_dn7_slot;
        let mut var_sp_s_temp2_dn8: f64 = *var_sp_s_temp2_dn8_slot;
        let mut var_sp_s_temp2_rv: f64 = *var_sp_s_temp2_rv_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_rv: f64 = *var_sp_s_xi0_rv_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_rv: f64 = *var_sp_s_xi1_rv_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_rv: f64 = *var_sp_s_xi2_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_udse: f64 = *var_udse_slot;
        let mut var_udse_dn5: f64 = *var_udse_dn5_slot;
        let mut var_udse_dn6: f64 = *var_udse_dn6_slot;
        let mut var_udse_dn7: f64 = *var_udse_dn7_slot;
        let mut var_udse_dn8: f64 = *var_udse_dn8_slot;
        let mut var_udse_rv: f64 = *var_udse_rv_slot;
        let mut var_v_dsat: f64 = *var_v_dsat_slot;
        let mut var_v_dsat_dn5: f64 = *var_v_dsat_dn5_slot;
        let mut var_v_dsat_dn6: f64 = *var_v_dsat_dn6_slot;
        let mut var_v_dsat_dn7: f64 = *var_v_dsat_dn7_slot;
        let mut var_v_dsat_dn8: f64 = *var_v_dsat_dn8_slot;
        let mut var_v_dsat_rv: f64 = *var_v_dsat_rv_slot;
        let mut var_vdse: f64 = *var_vdse_slot;
        let mut var_vdse_dn5: f64 = *var_vdse_dn5_slot;
        let mut var_vdse_dn6: f64 = *var_vdse_dn6_slot;
        let mut var_vdse_dn7: f64 = *var_vdse_dn7_slot;
        let mut var_vdse_dn8: f64 = *var_vdse_dn8_slot;
        let mut var_vdse_rv: f64 = *var_vdse_rv_slot;
        let mut var_x_0: f64 = *var_x_0_slot;
        let mut var_x_0_dn5: f64 = *var_x_0_dn5_slot;
        let mut var_x_0_dn6: f64 = *var_x_0_dn6_slot;
        let mut var_x_0_dn7: f64 = *var_x_0_dn7_slot;
        let mut var_x_0_dn8: f64 = *var_x_0_dn8_slot;
        let mut var_x_0_rv: f64 = *var_x_0_rv_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_d_rv: f64 = *var_x_d_rv_slot;
        let mut var_x_sat: f64 = *var_x_sat_slot;
        let mut var_x_sat_dn5: f64 = *var_x_sat_dn5_slot;
        let mut var_x_sat_dn6: f64 = *var_x_sat_dn6_slot;
        let mut var_x_sat_dn7: f64 = *var_x_sat_dn7_slot;
        let mut var_x_sat_dn8: f64 = *var_x_sat_dn8_slot;
        let mut var_x_sat_rv: f64 = *var_x_sat_rv_slot;
        let mut var_xn_d: f64 = *var_xn_d_slot;
        let mut var_xn_d_dn5: f64 = *var_xn_d_dn5_slot;
        let mut var_xn_d_dn6: f64 = *var_xn_d_dn6_slot;
        let mut var_xn_d_dn7: f64 = *var_xn_d_dn7_slot;
        let mut var_xn_d_dn8: f64 = *var_xn_d_dn8_slot;
        let mut var_xn_d_rv: f64 = *var_xn_d_rv_slot;

        let (assign43950_e56793, assign43950_e56793_d_n5, assign43950_e56793_d_n6, assign43950_e56793_d_n7, assign43950_e56793_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43950_e56769: f64 = (var_x_inf * var_za);
        let assign43950_e56773: f64 = (0.86 * var_temp__blk936);
        let assign43950_e56777: f64 = (var_temp__blk936 * var_za);
        let assign43950_e56778: f64 = (1.0 - assign43950_e56777);
        let assign43950_e56779: f64 = (assign43950_e56773 * assign43950_e56778);
        let assign43950_e56783: f64 = (4.0 * var_temp__blk936);
        let assign43950_e56785: f64 = (assign43950_e56783 * var_temp__blk936);
        let assign43950_e56787: f64 = (assign43950_e56785 * var_za);
        let assign43950_e56788: f64 = (1.0 + assign43950_e56787);
        let assign43950_e56789: f64 = (assign43950_e56779 / assign43950_e56788);
        let assign43950_e56790: f64 = (1.0 + assign43950_e56789);
        let assign43950_e56791: f64 = (assign43950_e56769 * assign43950_e56790);
        (assign43950_e56791, ((((var_x_inf_dn5 * var_za) + (var_x_inf * var_za_dn5)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn5) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn5 * var_za) + (var_temp__blk936 * var_za_dn5))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn5) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn5)) * var_za) + (assign43950_e56785 * var_za_dn5)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn6 * var_za) + (var_x_inf * var_za_dn6)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn6) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn6 * var_za) + (var_temp__blk936 * var_za_dn6))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn6) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn6)) * var_za) + (assign43950_e56785 * var_za_dn6)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn7 * var_za) + (var_x_inf * var_za_dn7)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn7) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn7 * var_za) + (var_temp__blk936 * var_za_dn7))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn7) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn7)) * var_za) + (assign43950_e56785 * var_za_dn7)))) / (assign43950_e56788 * assign43950_e56788)))), ((((var_x_inf_dn8 * var_za) + (var_x_inf * var_za_dn8)) * assign43950_e56790) + (assign43950_e56769 * ((((((0.86 * var_temp__blk936_dn8) * assign43950_e56778) + (assign43950_e56773 * (-((var_temp__blk936_dn8 * var_za) + (var_temp__blk936 * var_za_dn8))))) * assign43950_e56788) - (assign43950_e56779 * (((((4.0 * var_temp__blk936_dn8) * var_temp__blk936) + (assign43950_e56783 * var_temp__blk936_dn8)) * var_za) + (assign43950_e56785 * var_za_dn8)))) / (assign43950_e56788 * assign43950_e56788)))),)
    } else {
        (var_x_0, var_x_0_dn5, var_x_0_dn6, var_x_0_dn7, var_x_0_dn8,)
    }
};
        var_x_0 = assign43950_e56793;
        var_x_0_dn5 = assign43950_e56793_d_n5;
        var_x_0_dn6 = assign43950_e56793_d_n6;
        var_x_0_dn7 = assign43950_e56793_d_n7;
        var_x_0_dn8 = assign43950_e56793_d_n8;
        var_x_0_rv = 0.0;

        let (assign43960_e56801, assign43960_e56801_d_n5, assign43960_e56801_d_n6, assign43960_e56801_d_n7, assign43960_e56801_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43960_e56799: f64 = (0.99 * var_x_0);
        (assign43960_e56799, (0.99 * var_x_0_dn5), (0.99 * var_x_0_dn6), (0.99 * var_x_0_dn7), (0.99 * var_x_0_dn8),)
    } else {
        (var_x_sat, var_x_sat_dn5, var_x_sat_dn6, var_x_sat_dn7, var_x_sat_dn8,)
    }
};
        var_x_sat = assign43960_e56801;
        var_x_sat_dn5 = assign43960_e56801_d_n5;
        var_x_sat_dn6 = assign43960_e56801_d_n6;
        var_x_sat_dn7 = assign43960_e56801_d_n7;
        var_x_sat_dn8 = assign43960_e56801_d_n8;
        var_x_sat_rv = 0.0;

        let (assign43970_e56817, assign43970_e56817_d_n5, assign43970_e56817_d_n6, assign43970_e56817_d_n7, assign43970_e56817_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43970_e56809: f64 = (2.0 * var_asat);
        let assign43970_e56810: f64 = (var_x_sat - assign43970_e56809);
        let assign43970_e56811: f64 = (var_x_sat * assign43970_e56810);
        let assign43970_e56813: f64 = (assign43970_e56811 * var_inv_gf2);
        let assign43970_e56815: f64 = (assign43970_e56813 / var_ds);
        (assign43970_e56815, (((((((var_x_sat_dn5 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn5 - (2.0 * var_asat_dn5)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn5)) * var_ds) - (assign43970_e56813 * var_ds_dn5)) / (var_ds * var_ds)), (((((((var_x_sat_dn6 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn6 - (2.0 * var_asat_dn6)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn6)) * var_ds) - (assign43970_e56813 * var_ds_dn6)) / (var_ds * var_ds)), (((((((var_x_sat_dn7 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn7 - (2.0 * var_asat_dn7)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn7)) * var_ds) - (assign43970_e56813 * var_ds_dn7)) / (var_ds * var_ds)), (((((((var_x_sat_dn8 * assign43970_e56810) + (var_x_sat * (var_x_sat_dn8 - (2.0 * var_asat_dn8)))) * var_inv_gf2) + (assign43970_e56811 * var_inv_gf2_dn8)) * var_ds) - (assign43970_e56813 * var_ds_dn8)) / (var_ds * var_ds)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign43970_e56817;
        var_temp__blk936_dn5 = assign43970_e56817_d_n5;
        var_temp__blk936_dn6 = assign43970_e56817_d_n6;
        var_temp__blk936_dn7 = assign43970_e56817_d_n7;
        var_temp__blk936_dn8 = assign43970_e56817_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign43980_e56837, assign43980_e56837_d_n5, assign43980_e56837_d_n6, assign43980_e56837_d_n7, assign43980_e56837_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 != 0.0)) {
        let assign43980_e56826: f64 = (-0.99);
        let (assign43980_e56831, assign43980_e56831_d_n5, assign43980_e56831_d_n6, assign43980_e56831_d_n7, assign43980_e56831_d_n8,) = {
            if (var_temp__blk936 > assign43980_e56826) {
                (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
            } else {
                let assign43980_e56830: f64 = (-0.99);
                (assign43980_e56830, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign43980_e56832: f64 = (1.0 + assign43980_e56831);
        let assign43980_e56833: f64 = (assign43980_e56832).ln();
        let assign43980_e56834: f64 = (var_x_sat - assign43980_e56833);
        let assign43980_e56835: f64 = (var_phit1 * assign43980_e56834);
        (assign43980_e56835, ((var_phit1_dn5 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn5 - (assign43980_e56831_d_n5 / assign43980_e56832)))), ((var_phit1_dn6 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn6 - (assign43980_e56831_d_n6 / assign43980_e56832)))), ((var_phit1_dn7 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn7 - (assign43980_e56831_d_n7 / assign43980_e56832)))), ((var_phit1_dn8 * assign43980_e56834) + (var_phit1 * (var_x_sat_dn8 - (assign43980_e56831_d_n8 / assign43980_e56832)))),)
    } else {
        (var_v_dsat, var_v_dsat_dn5, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8,)
    }
};
        var_v_dsat = assign43980_e56837;
        var_v_dsat_dn5 = assign43980_e56837_d_n5;
        var_v_dsat_dn6 = assign43980_e56837_d_n6;
        var_v_dsat_dn7 = assign43980_e56837_d_n7;
        var_v_dsat_dn8 = assign43980_e56837_d_n8;
        var_v_dsat_rv = 0.0;

        let (assign43990_e56844, assign43990_e56844_d_n5, assign43990_e56844_d_n6, assign43990_e56844_d_n7, assign43990_e56844_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1198 == 0.0)) {
        (var_vdsat_lim, var_vdsat_lim_dn5, var_vdsat_lim_dn6, var_vdsat_lim_dn7, var_vdsat_lim_dn8,)
    } else {
        (var_v_dsat, var_v_dsat_dn5, var_v_dsat_dn6, var_v_dsat_dn7, var_v_dsat_dn8,)
    }
};
        var_v_dsat = assign43990_e56844;
        var_v_dsat_dn5 = assign43990_e56844_d_n5;
        var_v_dsat_dn6 = assign43990_e56844_d_n6;
        var_v_dsat_dn7 = assign43990_e56844_d_n7;
        var_v_dsat_dn8 = assign43990_e56844_d_n8;
        var_v_dsat_rv = 0.0;

        let (assign44000_e56850, assign44000_e56850_d_n5, assign44000_e56850_d_n6, assign44000_e56850_d_n7, assign44000_e56850_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44000_e56848: f64 = (1.0 + var_arloc);
        (assign44000_e56848, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44000_e56850;
        var_temp__blk936_dn5 = assign44000_e56850_d_n5;
        var_temp__blk936_dn6 = assign44000_e56850_d_n6;
        var_temp__blk936_dn7 = assign44000_e56850_d_n7;
        var_temp__blk936_dn8 = assign44000_e56850_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44010_e56859, assign44010_e56859_d_n5, assign44010_e56859_d_n6, assign44010_e56859_d_n7, assign44010_e56859_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44010_e56853: f64 = (var_temp__blk936).sqrt();
        let assign44010_e56855: f64 = (assign44010_e56853 * var_v_ds);
        let assign44010_e56857: f64 = (assign44010_e56855 / var_v_dsat);
        (assign44010_e56857, (((((var_temp__blk936_dn5 / (2.0 * assign44010_e56853)) * var_v_ds) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn5)) / (var_v_dsat * var_v_dsat)), ((((((var_temp__blk936_dn6 / (2.0 * assign44010_e56853)) * var_v_ds) + (assign44010_e56853 * var_v_ds_dn6)) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn6)) / (var_v_dsat * var_v_dsat)), ((((((var_temp__blk936_dn7 / (2.0 * assign44010_e56853)) * var_v_ds) + (assign44010_e56853 * var_v_ds_dn7)) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn7)) / (var_v_dsat * var_v_dsat)), (((((var_temp__blk936_dn8 / (2.0 * assign44010_e56853)) * var_v_ds) * var_v_dsat) - (assign44010_e56855 * var_v_dsat_dn8)) / (var_v_dsat * var_v_dsat)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign44010_e56859;
        var_temp1_dn5 = assign44010_e56859_d_n5;
        var_temp1_dn6 = assign44010_e56859_d_n6;
        var_temp1_dn7 = assign44010_e56859_d_n7;
        var_temp1_dn8 = assign44010_e56859_d_n8;
        var_temp1_rv = 0.0;

        let (assign44020_e56867, assign44020_e56867_d_n5, assign44020_e56867_d_n6, assign44020_e56867_d_n7, assign44020_e56867_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44020_e56863: f64 = (var_temp1 * var_temp1);
        let assign44020_e56865: f64 = (assign44020_e56863 + var_temp__blk936);
        (assign44020_e56865, (((var_temp1_dn5 * var_temp1) + (var_temp1 * var_temp1_dn5)) + var_temp__blk936_dn5), (((var_temp1_dn6 * var_temp1) + (var_temp1 * var_temp1_dn6)) + var_temp__blk936_dn6), (((var_temp1_dn7 * var_temp1) + (var_temp1 * var_temp1_dn7)) + var_temp__blk936_dn7), (((var_temp1_dn8 * var_temp1) + (var_temp1 * var_temp1_dn8)) + var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign44020_e56867;
        var_temp2_dn5 = assign44020_e56867_d_n5;
        var_temp2_dn6 = assign44020_e56867_d_n6;
        var_temp2_dn7 = assign44020_e56867_d_n7;
        var_temp2_dn8 = assign44020_e56867_d_n8;
        var_temp2_rv = 0.0;

        let (assign44030_e56873, assign44030_e56873_d_n5, assign44030_e56873_d_n6, assign44030_e56873_d_n7, assign44030_e56873_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44030_e56871: f64 = (2.0 * var_temp1);
        (assign44030_e56871, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44030_e56873;
        var_temp__blk936_dn5 = assign44030_e56873_d_n5;
        var_temp__blk936_dn6 = assign44030_e56873_d_n6;
        var_temp__blk936_dn7 = assign44030_e56873_d_n7;
        var_temp__blk936_dn8 = assign44030_e56873_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44040_e56889, assign44040_e56889_d_n5, assign44040_e56889_d_n6, assign44040_e56889_d_n7, assign44040_e56889_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44040_e56877: f64 = (var_v_dsat * var_temp__blk936);
        let assign44040_e56880: f64 = (var_temp2 - var_temp__blk936);
        let assign44040_e56881: f64 = (assign44040_e56880).sqrt();
        let assign44040_e56884: f64 = (var_temp2 + var_temp__blk936);
        let assign44040_e56885: f64 = (assign44040_e56884).sqrt();
        let assign44040_e56886: f64 = (assign44040_e56881 + assign44040_e56885);
        let assign44040_e56887: f64 = (assign44040_e56877 / assign44040_e56886);
        (assign44040_e56887, (((((var_v_dsat_dn5 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn5)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn5 - var_temp__blk936_dn5) / (2.0 * assign44040_e56881)) + ((var_temp2_dn5 + var_temp__blk936_dn5) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn6 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn6)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn6 - var_temp__blk936_dn6) / (2.0 * assign44040_e56881)) + ((var_temp2_dn6 + var_temp__blk936_dn6) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn7 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn7)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn7 - var_temp__blk936_dn7) / (2.0 * assign44040_e56881)) + ((var_temp2_dn7 + var_temp__blk936_dn7) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)), (((((var_v_dsat_dn8 * var_temp__blk936) + (var_v_dsat * var_temp__blk936_dn8)) * assign44040_e56886) - (assign44040_e56877 * (((var_temp2_dn8 - var_temp__blk936_dn8) / (2.0 * assign44040_e56881)) + ((var_temp2_dn8 + var_temp__blk936_dn8) / (2.0 * assign44040_e56885))))) / (assign44040_e56886 * assign44040_e56886)),)
    } else {
        (var_vdse, var_vdse_dn5, var_vdse_dn6, var_vdse_dn7, var_vdse_dn8,)
    }
};
        var_vdse = assign44040_e56889;
        var_vdse_dn5 = assign44040_e56889_d_n5;
        var_vdse_dn6 = assign44040_e56889_d_n6;
        var_vdse_dn7 = assign44040_e56889_d_n7;
        var_vdse_dn8 = assign44040_e56889_d_n8;
        var_vdse_rv = 0.0;

        let (assign44050_e56895, assign44050_e56895_d_n5, assign44050_e56895_d_n6, assign44050_e56895_d_n7, assign44050_e56895_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44050_e56893: f64 = (var_vdse * var_inv_phit1);
        (assign44050_e56893, ((var_vdse_dn5 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn5)), ((var_vdse_dn6 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn6)), ((var_vdse_dn7 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn7)), ((var_vdse_dn8 * var_inv_phit1) + (var_vdse * var_inv_phit1_dn8)),)
    } else {
        (var_udse, var_udse_dn5, var_udse_dn6, var_udse_dn7, var_udse_dn8,)
    }
};
        var_udse = assign44050_e56895;
        var_udse_dn5 = assign44050_e56895_d_n5;
        var_udse_dn6 = assign44050_e56895_d_n6;
        var_udse_dn7 = assign44050_e56895_d_n7;
        var_udse_dn8 = assign44050_e56895_d_n8;
        var_udse_rv = 0.0;

        let (assign44060_e56901, assign44060_e56901_d_n5, assign44060_e56901_d_n6, assign44060_e56901_d_n7, assign44060_e56901_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44060_e56899: f64 = (var_xn_s + var_udse);
        (assign44060_e56899, (var_xn_s_dn5 + var_udse_dn5), (var_xn_s_dn6 + var_udse_dn6), (var_xn_s_dn7 + var_udse_dn7), (var_xn_s_dn8 + var_udse_dn8),)
    } else {
        (var_xn_d, var_xn_d_dn5, var_xn_d_dn6, var_xn_d_dn7, var_xn_d_dn8,)
    }
};
        var_xn_d = assign44060_e56901;
        var_xn_d_dn5 = assign44060_e56901_d_n5;
        var_xn_d_dn6 = assign44060_e56901_d_n6;
        var_xn_d_dn7 = assign44060_e56901_d_n7;
        var_xn_d_dn8 = assign44060_e56901_d_n8;
        var_xn_d_rv = 0.0;

        let assign44070_e56904: f64 = if var_udse < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1204 = assign44070_e56904;
        var_guard1204_rv = 0.0;

        let (assign44080_e56912, assign44080_e56912_d_n5, assign44080_e56912_d_n6, assign44080_e56912_d_n7, assign44080_e56912_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1204 != 0.0)) {
        let assign44080_e56909: f64 = (-var_udse);
        let assign44080_e56910: f64 = (assign44080_e56909).exp();
        (assign44080_e56910, (assign44080_e56910 * (-var_udse_dn5)), (assign44080_e56910 * (-var_udse_dn6)), (assign44080_e56910 * (-var_udse_dn7)), (assign44080_e56910 * (-var_udse_dn8)),)
    } else {
        (var_k_ds, var_k_ds_dn5, var_k_ds_dn6, var_k_ds_dn7, var_k_ds_dn8,)
    }
};
        var_k_ds = assign44080_e56912;
        var_k_ds_dn5 = assign44080_e56912_d_n5;
        var_k_ds_dn6 = assign44080_e56912_d_n6;
        var_k_ds_dn7 = assign44080_e56912_d_n7;
        var_k_ds_dn8 = assign44080_e56912_d_n8;
        var_k_ds_rv = 0.0;

        let (assign44090_e56941, assign44090_e56941_d_n5, assign44090_e56941_d_n6, assign44090_e56941_d_n7, assign44090_e56941_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1204 == 0.0)) {
        let assign44090_e56921: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56926: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56930: f64 = (var_udse - 460.51701859880916);
        let assign44090_e56932: f64 = (assign44090_e56930 * 0.3333333333333333);
        let assign44090_e56933: f64 = (1.0 + assign44090_e56932);
        let assign44090_e56934: f64 = (assign44090_e56926 * assign44090_e56933);
        let assign44090_e56935: f64 = (0.5 * assign44090_e56934);
        let assign44090_e56936: f64 = (1.0 + assign44090_e56935);
        let assign44090_e56937: f64 = (assign44090_e56921 * assign44090_e56936);
        let assign44090_e56938: f64 = (1.0 + assign44090_e56937);
        let assign44090_e56939: f64 = (1e-200 / assign44090_e56938);
        (assign44090_e56939, (-((1e-200 * ((var_udse_dn5 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn5 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn5 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn6 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn6 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn6 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn7 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn7 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn7 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))), (-((1e-200 * ((var_udse_dn8 * assign44090_e56936) + (assign44090_e56921 * (0.5 * ((var_udse_dn8 * assign44090_e56933) + (assign44090_e56926 * (var_udse_dn8 * 0.3333333333333333))))))) / (assign44090_e56938 * assign44090_e56938))),)
    } else {
        (var_k_ds, var_k_ds_dn5, var_k_ds_dn6, var_k_ds_dn7, var_k_ds_dn8,)
    }
};
        var_k_ds = assign44090_e56941;
        var_k_ds_dn5 = assign44090_e56941_d_n5;
        var_k_ds_dn6 = assign44090_e56941_d_n6;
        var_k_ds_dn7 = assign44090_e56941_d_n7;
        var_k_ds_dn8 = assign44090_e56941_d_n8;
        var_k_ds_rv = 0.0;

        let (assign44100_e56947, assign44100_e56947_d_n5, assign44100_e56947_d_n6, assign44100_e56947_d_n7, assign44100_e56947_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44100_e56945: f64 = (var_delta_ns * var_k_ds);
        (assign44100_e56945, ((var_delta_ns_dn5 * var_k_ds) + (var_delta_ns * var_k_ds_dn5)), ((var_delta_ns_dn6 * var_k_ds) + (var_delta_ns * var_k_ds_dn6)), ((var_delta_ns_dn7 * var_k_ds) + (var_delta_ns * var_k_ds_dn7)), ((var_delta_ns_dn8 * var_k_ds) + (var_delta_ns * var_k_ds_dn8)),)
    } else {
        (var_delta_nd, var_delta_nd_dn5, var_delta_nd_dn6, var_delta_nd_dn7, var_delta_nd_dn8,)
    }
};
        var_delta_nd = assign44100_e56947;
        var_delta_nd_dn5 = assign44100_e56947_d_n5;
        var_delta_nd_dn6 = assign44100_e56947_d_n6;
        var_delta_nd_dn7 = assign44100_e56947_d_n7;
        var_delta_nd_dn8 = assign44100_e56947_d_n8;
        var_delta_nd_rv = 0.0;

        let assign44110_e56949: f64 = (var_xg).abs();
        let assign44110_e56951: f64 = if assign44110_e56949 <= var_margin { 1.0 } else { 0.0 };
        var_guard1205 = assign44110_e56951;
        var_guard1205_rv = 0.0;

        let (assign44120_e56963, assign44120_e56963_d_n5, assign44120_e56963_d_n6, assign44120_e56963_d_n7, assign44120_e56963_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 != 0.0)) {
        let assign44120_e56957: f64 = (var_inv_xi * var_inv_xi);
        let assign44120_e56959: f64 = (assign44120_e56957 * 0.16666666666666666);
        let assign44120_e56961: f64 = (assign44120_e56959 * 0.7071067811865475);
        (assign44120_e56961, ((((var_inv_xi_dn5 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn5)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn6 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn6)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn7 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn7)) * 0.16666666666666666) * 0.7071067811865475), ((((var_inv_xi_dn8 * var_inv_xi) + (var_inv_xi * var_inv_xi_dn8)) * 0.16666666666666666) * 0.7071067811865475),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign44120_e56963;
        var_sp_s_temp1_dn5 = assign44120_e56963_d_n5;
        var_sp_s_temp1_dn6 = assign44120_e56963_d_n6;
        var_sp_s_temp1_dn7 = assign44120_e56963_d_n7;
        var_sp_s_temp1_dn8 = assign44120_e56963_d_n8;
        var_sp_s_temp1_rv = 0.0;

        let (assign44130_e56983, assign44130_e56983_d_n5, assign44130_e56983_d_n6, assign44130_e56983_d_n7, assign44130_e56983_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 != 0.0)) {
        let assign44130_e56969: f64 = (var_xg * var_inv_xi);
        let assign44130_e56974: f64 = (1.0 - var_delta_nd);
        let assign44130_e56975: f64 = (var_xg * assign44130_e56974);
        let assign44130_e56977: f64 = (assign44130_e56975 * var_gf);
        let assign44130_e56979: f64 = (assign44130_e56977 * var_sp_s_temp1);
        let assign44130_e56980: f64 = (1.0 + assign44130_e56979);
        let assign44130_e56981: f64 = (assign44130_e56969 * assign44130_e56980);
        (assign44130_e56981, ((((var_xg_dn5 * var_inv_xi) + (var_xg * var_inv_xi_dn5)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn5 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn5))) * var_gf) + (assign44130_e56975 * var_gf_dn5)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn5)))), ((((var_xg_dn6 * var_inv_xi) + (var_xg * var_inv_xi_dn6)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn6 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn6))) * var_gf) + (assign44130_e56975 * var_gf_dn6)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn6)))), ((((var_xg_dn7 * var_inv_xi) + (var_xg * var_inv_xi_dn7)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn7 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn7))) * var_gf) + (assign44130_e56975 * var_gf_dn7)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn7)))), ((((var_xg_dn8 * var_inv_xi) + (var_xg * var_inv_xi_dn8)) * assign44130_e56980) + (assign44130_e56969 * ((((((var_xg_dn8 * assign44130_e56974) + (var_xg * (-var_delta_nd_dn8))) * var_gf) + (assign44130_e56975 * var_gf_dn8)) * var_sp_s_temp1) + (assign44130_e56977 * var_sp_s_temp1_dn8)))),)
    } else {
        (var_x_d, var_x_d_dn5, var_x_d_dn6, var_x_d_dn7, var_x_d_dn8,)
    }
};
        var_x_d = assign44130_e56983;
        var_x_d_dn5 = assign44130_e56983_d_n5;
        var_x_d_dn6 = assign44130_e56983_d_n6;
        var_x_d_dn7 = assign44130_e56983_d_n7;
        var_x_d_dn8 = assign44130_e56983_d_n8;
        var_x_d_rv = 0.0;

        let (assign44140_e56992, assign44140_e56992_d_n5, assign44140_e56992_d_n6, assign44140_e56992_d_n7, assign44140_e56992_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44140_e56990: f64 = (var_xn_d + 3.0);
        (assign44140_e56990, var_xn_d_dn5, var_xn_d_dn6, var_xn_d_dn7, var_xn_d_dn8,)
    } else {
        (var_sp_s_bx, var_sp_s_bx_dn5, var_sp_s_bx_dn6, var_sp_s_bx_dn7, var_sp_s_bx_dn8,)
    }
};
        var_sp_s_bx = assign44140_e56992;
        var_sp_s_bx_dn5 = assign44140_e56992_d_n5;
        var_sp_s_bx_dn6 = assign44140_e56992_d_n6;
        var_sp_s_bx_dn7 = assign44140_e56992_d_n7;
        var_sp_s_bx_dn8 = assign44140_e56992_d_n8;
        var_sp_s_bx_rv = 0.0;

        let (assign44150_e57025, assign44150_e57025_d_n5, assign44150_e57025_d_n6, assign44150_e57025_d_n7, assign44150_e57025_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44150_e57000: f64 = (var_sp_s_x1 + var_sp_s_bx);
        let assign44150_e57003: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign44150_e57006: f64 = (var_sp_s_x1 - var_sp_s_bx);
        let assign44150_e57007: f64 = (assign44150_e57003 * assign44150_e57006);
        let assign44150_e57009: f64 = (assign44150_e57007 + 5.0);
        let assign44150_e57010: f64 = (assign44150_e57009).sqrt();
        let assign44150_e57011: f64 = (assign44150_e57000 - assign44150_e57010);
        let assign44150_e57012: f64 = (0.5 * assign44150_e57011);
        let assign44150_e57017: f64 = (var_sp_s_bx * var_sp_s_bx);
        let assign44150_e57019: f64 = (assign44150_e57017 + 5.0);
        let assign44150_e57020: f64 = (assign44150_e57019).sqrt();
        let assign44150_e57021: f64 = (var_sp_s_bx - assign44150_e57020);
        let assign44150_e57022: f64 = (0.5 * assign44150_e57021);
        let assign44150_e57023: f64 = (assign44150_e57012 - assign44150_e57022);
        (assign44150_e57023, ((0.5 * ((var_sp_s_x1_dn5 + var_sp_s_bx_dn5) - ((((var_sp_s_x1_dn5 - var_sp_s_bx_dn5) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn5 - var_sp_s_bx_dn5))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn5 - (((var_sp_s_bx_dn5 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn5)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn6 + var_sp_s_bx_dn6) - ((((var_sp_s_x1_dn6 - var_sp_s_bx_dn6) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn6 - var_sp_s_bx_dn6))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn6 - (((var_sp_s_bx_dn6 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn6)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn7 + var_sp_s_bx_dn7) - ((((var_sp_s_x1_dn7 - var_sp_s_bx_dn7) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn7 - var_sp_s_bx_dn7))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn7 - (((var_sp_s_bx_dn7 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn7)) / (2.0 * assign44150_e57020))))), ((0.5 * ((var_sp_s_x1_dn8 + var_sp_s_bx_dn8) - ((((var_sp_s_x1_dn8 - var_sp_s_bx_dn8) * assign44150_e57006) + (assign44150_e57003 * (var_sp_s_x1_dn8 - var_sp_s_bx_dn8))) / (2.0 * assign44150_e57010)))) - (0.5 * (var_sp_s_bx_dn8 - (((var_sp_s_bx_dn8 * var_sp_s_bx) + (var_sp_s_bx * var_sp_s_bx_dn8)) / (2.0 * assign44150_e57020))))),)
    } else {
        (var_sp_s_eta, var_sp_s_eta_dn5, var_sp_s_eta_dn6, var_sp_s_eta_dn7, var_sp_s_eta_dn8,)
    }
};
        var_sp_s_eta = assign44150_e57025;
        var_sp_s_eta_dn5 = assign44150_e57025_d_n5;
        var_sp_s_eta_dn6 = assign44150_e57025_d_n6;
        var_sp_s_eta_dn7 = assign44150_e57025_d_n7;
        var_sp_s_eta_dn8 = assign44150_e57025_d_n8;
        var_sp_s_eta_rv = 0.0;

        let (assign44160_e57034, assign44160_e57034_d_n5, assign44160_e57034_d_n6, assign44160_e57034_d_n7, assign44160_e57034_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44160_e57032: f64 = (var_xg - var_sp_s_eta);
        (assign44160_e57032, (var_xg_dn5 - var_sp_s_eta_dn5), (var_xg_dn6 - var_sp_s_eta_dn6), (var_xg_dn7 - var_sp_s_eta_dn7), (var_xg_dn8 - var_sp_s_eta_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44160_e57034;
        var_sp_s_temp_dn5 = assign44160_e57034_d_n5;
        var_sp_s_temp_dn6 = assign44160_e57034_d_n6;
        var_sp_s_temp_dn7 = assign44160_e57034_d_n7;
        var_sp_s_temp_dn8 = assign44160_e57034_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign44170_e57043, assign44170_e57043_d_n5, assign44170_e57043_d_n6, assign44170_e57043_d_n7, assign44170_e57043_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44170_e57040: f64 = (-var_sp_s_eta);
        let assign44170_e57041: f64 = (assign44170_e57040).exp();
        (assign44170_e57041, (assign44170_e57041 * (-var_sp_s_eta_dn5)), (assign44170_e57041 * (-var_sp_s_eta_dn6)), (assign44170_e57041 * (-var_sp_s_eta_dn7)), (assign44170_e57041 * (-var_sp_s_eta_dn8)),)
    } else {
        (var_sp_s_temp1, var_sp_s_temp1_dn5, var_sp_s_temp1_dn6, var_sp_s_temp1_dn7, var_sp_s_temp1_dn8,)
    }
};
        var_sp_s_temp1 = assign44170_e57043;
        var_sp_s_temp1_dn5 = assign44170_e57043_d_n5;
        var_sp_s_temp1_dn6 = assign44170_e57043_d_n6;
        var_sp_s_temp1_dn7 = assign44170_e57043_d_n7;
        var_sp_s_temp1_dn8 = assign44170_e57043_d_n8;
        var_sp_s_temp1_rv = 0.0;

        let (assign44180_e57056, assign44180_e57056_d_n5, assign44180_e57056_d_n6, assign44180_e57056_d_n7, assign44180_e57056_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44180_e57052: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign44180_e57053: f64 = (2.0 + assign44180_e57052);
        let assign44180_e57054: f64 = (1.0 / assign44180_e57053);
        (assign44180_e57054, (-(((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) / (assign44180_e57053 * assign44180_e57053))), (-(((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) / (assign44180_e57053 * assign44180_e57053))),)
    } else {
        (var_sp_s_temp2, var_sp_s_temp2_dn5, var_sp_s_temp2_dn6, var_sp_s_temp2_dn7, var_sp_s_temp2_dn8,)
    }
};
        var_sp_s_temp2 = assign44180_e57056;
        var_sp_s_temp2_dn5 = assign44180_e57056_d_n5;
        var_sp_s_temp2_dn6 = assign44180_e57056_d_n6;
        var_sp_s_temp2_dn7 = assign44180_e57056_d_n7;
        var_sp_s_temp2_dn8 = assign44180_e57056_d_n8;
        var_sp_s_temp2_rv = 0.0;

        let (assign44190_e57067, assign44190_e57067_d_n5, assign44190_e57067_d_n6, assign44190_e57067_d_n7, assign44190_e57067_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44190_e57063: f64 = (var_sp_s_eta * var_sp_s_eta);
        let assign44190_e57065: f64 = (assign44190_e57063 * var_sp_s_temp2);
        (assign44190_e57065, ((((var_sp_s_eta_dn5 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn5)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn5)), ((((var_sp_s_eta_dn6 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn6)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn6)), ((((var_sp_s_eta_dn7 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn7)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn7)), ((((var_sp_s_eta_dn8 * var_sp_s_eta) + (var_sp_s_eta * var_sp_s_eta_dn8)) * var_sp_s_temp2) + (assign44190_e57063 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign44190_e57067;
        var_sp_s_xi0_dn5 = assign44190_e57067_d_n5;
        var_sp_s_xi0_dn6 = assign44190_e57067_d_n6;
        var_sp_s_xi0_dn7 = assign44190_e57067_d_n7;
        var_sp_s_xi0_dn8 = assign44190_e57067_d_n8;
        var_sp_s_xi0_rv = 0.0;

        let (assign44200_e57080, assign44200_e57080_d_n5, assign44200_e57080_d_n6, assign44200_e57080_d_n7, assign44200_e57080_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44200_e57075: f64 = (var_sp_s_eta * var_sp_s_temp2);
        let assign44200_e57077: f64 = (assign44200_e57075 * var_sp_s_temp2);
        let assign44200_e57078: f64 = (4.0 * assign44200_e57077);
        (assign44200_e57078, (4.0 * ((((var_sp_s_eta_dn5 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn5))), (4.0 * ((((var_sp_s_eta_dn6 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn6))), (4.0 * ((((var_sp_s_eta_dn7 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn7))), (4.0 * ((((var_sp_s_eta_dn8 * var_sp_s_temp2) + (var_sp_s_eta * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign44200_e57075 * var_sp_s_temp2_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign44200_e57080;
        var_sp_s_xi1_dn5 = assign44200_e57080_d_n5;
        var_sp_s_xi1_dn6 = assign44200_e57080_d_n6;
        var_sp_s_xi1_dn7 = assign44200_e57080_d_n7;
        var_sp_s_xi1_dn8 = assign44200_e57080_d_n8;
        var_sp_s_xi1_rv = 0.0;

        let (assign44210_e57097, assign44210_e57097_d_n5, assign44210_e57097_d_n6, assign44210_e57097_d_n7, assign44210_e57097_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44210_e57087: f64 = (8.0 * var_sp_s_temp2);
        let assign44210_e57090: f64 = (12.0 * var_sp_s_xi0);
        let assign44210_e57091: f64 = (assign44210_e57087 - assign44210_e57090);
        let assign44210_e57093: f64 = (assign44210_e57091 * var_sp_s_temp2);
        let assign44210_e57095: f64 = (assign44210_e57093 * var_sp_s_temp2);
        (assign44210_e57095, ((((((8.0 * var_sp_s_temp2_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn5)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn5)), ((((((8.0 * var_sp_s_temp2_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn6)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn6)), ((((((8.0 * var_sp_s_temp2_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn7)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn7)), ((((((8.0 * var_sp_s_temp2_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp2) + (assign44210_e57091 * var_sp_s_temp2_dn8)) * var_sp_s_temp2) + (assign44210_e57093 * var_sp_s_temp2_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign44210_e57097;
        var_sp_s_xi2_dn5 = assign44210_e57097_d_n5;
        var_sp_s_xi2_dn6 = assign44210_e57097_d_n6;
        var_sp_s_xi2_dn7 = assign44210_e57097_d_n7;
        var_sp_s_xi2_dn8 = assign44210_e57097_d_n8;
        var_sp_s_xi2_rv = 0.0;

        let (assign44220_e57145, assign44220_e57145_d_n5, assign44220_e57145_d_n6, assign44220_e57145_d_n7, assign44220_e57145_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44220_e57105: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign44220_e57109: f64 = (var_sp_s_temp1 + var_sp_s_eta);
        let assign44220_e57111: f64 = (assign44220_e57109 - 1.0);
        let assign44220_e57115: f64 = (var_sp_s_eta + 1.0);
        let assign44220_e57117: f64 = (assign44220_e57115 + var_sp_s_xi0);
        let assign44220_e57118: f64 = (var_delta_nd * assign44220_e57117);
        let assign44220_e57119: f64 = (assign44220_e57111 - assign44220_e57118);
        let assign44220_e57120: f64 = (var_gf2 * assign44220_e57119);
        let assign44220_e57121: f64 = (assign44220_e57105 - assign44220_e57120);
        let (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,) = {
            if (1e-40 > assign44220_e57121) {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign44220_e57126: f64 = (var_sp_s_temp * var_sp_s_temp);
                let assign44220_e57130: f64 = (var_sp_s_temp1 + var_sp_s_eta);
                let assign44220_e57132: f64 = (assign44220_e57130 - 1.0);
                let assign44220_e57136: f64 = (var_sp_s_eta + 1.0);
                let assign44220_e57138: f64 = (assign44220_e57136 + var_sp_s_xi0);
                let assign44220_e57139: f64 = (var_delta_nd * assign44220_e57138);
                let assign44220_e57140: f64 = (assign44220_e57132 - assign44220_e57139);
                let assign44220_e57141: f64 = (var_gf2 * assign44220_e57140);
                let assign44220_e57142: f64 = (assign44220_e57126 - assign44220_e57141);
                (assign44220_e57142, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn5 + var_sp_s_eta_dn5) - ((var_delta_nd_dn5 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn6 + var_sp_s_eta_dn6) - ((var_delta_nd_dn6 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn7 + var_sp_s_eta_dn7) - ((var_delta_nd_dn7 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign44220_e57140) + (var_gf2 * ((var_sp_s_temp1_dn8 + var_sp_s_eta_dn8) - ((var_delta_nd_dn8 * assign44220_e57138) + (var_delta_nd * (var_sp_s_eta_dn8 + var_sp_s_xi0_dn8))))))),)
            }
        };
        (assign44220_e57143, assign44220_e57143_d_n5, assign44220_e57143_d_n6, assign44220_e57143_d_n7, assign44220_e57143_d_n8,)
    } else {
        (var_sp_s_a, var_sp_s_a_dn5, var_sp_s_a_dn6, var_sp_s_a_dn7, var_sp_s_a_dn8,)
    }
};
        var_sp_s_a = assign44220_e57145;
        var_sp_s_a_dn5 = assign44220_e57145_d_n5;
        var_sp_s_a_dn6 = assign44220_e57145_d_n6;
        var_sp_s_a_dn7 = assign44220_e57145_d_n7;
        var_sp_s_a_dn8 = assign44220_e57145_d_n8;
        var_sp_s_a_rv = 0.0;

        *var_delta_nd_slot = var_delta_nd;
        *var_delta_nd_dn5_slot = var_delta_nd_dn5;
        *var_delta_nd_dn6_slot = var_delta_nd_dn6;
        *var_delta_nd_dn7_slot = var_delta_nd_dn7;
        *var_delta_nd_dn8_slot = var_delta_nd_dn8;
        *var_delta_nd_rv_slot = var_delta_nd_rv;
        *var_guard1204_slot = var_guard1204;
        *var_guard1204_rv_slot = var_guard1204_rv;
        *var_guard1205_slot = var_guard1205;
        *var_guard1205_rv_slot = var_guard1205_rv;
        *var_k_ds_slot = var_k_ds;
        *var_k_ds_dn5_slot = var_k_ds_dn5;
        *var_k_ds_dn6_slot = var_k_ds_dn6;
        *var_k_ds_dn7_slot = var_k_ds_dn7;
        *var_k_ds_dn8_slot = var_k_ds_dn8;
        *var_k_ds_rv_slot = var_k_ds_rv;
        *var_sp_s_a_slot = var_sp_s_a;
        *var_sp_s_a_dn5_slot = var_sp_s_a_dn5;
        *var_sp_s_a_dn6_slot = var_sp_s_a_dn6;
        *var_sp_s_a_dn7_slot = var_sp_s_a_dn7;
        *var_sp_s_a_dn8_slot = var_sp_s_a_dn8;
        *var_sp_s_a_rv_slot = var_sp_s_a_rv;
        *var_sp_s_bx_slot = var_sp_s_bx;
        *var_sp_s_bx_dn5_slot = var_sp_s_bx_dn5;
        *var_sp_s_bx_dn6_slot = var_sp_s_bx_dn6;
        *var_sp_s_bx_dn7_slot = var_sp_s_bx_dn7;
        *var_sp_s_bx_dn8_slot = var_sp_s_bx_dn8;
        *var_sp_s_bx_rv_slot = var_sp_s_bx_rv;
        *var_sp_s_eta_slot = var_sp_s_eta;
        *var_sp_s_eta_dn5_slot = var_sp_s_eta_dn5;
        *var_sp_s_eta_dn6_slot = var_sp_s_eta_dn6;
        *var_sp_s_eta_dn7_slot = var_sp_s_eta_dn7;
        *var_sp_s_eta_dn8_slot = var_sp_s_eta_dn8;
        *var_sp_s_eta_rv_slot = var_sp_s_eta_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp1_slot = var_sp_s_temp1;
        *var_sp_s_temp1_dn5_slot = var_sp_s_temp1_dn5;
        *var_sp_s_temp1_dn6_slot = var_sp_s_temp1_dn6;
        *var_sp_s_temp1_dn7_slot = var_sp_s_temp1_dn7;
        *var_sp_s_temp1_dn8_slot = var_sp_s_temp1_dn8;
        *var_sp_s_temp1_rv_slot = var_sp_s_temp1_rv;
        *var_sp_s_temp2_slot = var_sp_s_temp2;
        *var_sp_s_temp2_dn5_slot = var_sp_s_temp2_dn5;
        *var_sp_s_temp2_dn6_slot = var_sp_s_temp2_dn6;
        *var_sp_s_temp2_dn7_slot = var_sp_s_temp2_dn7;
        *var_sp_s_temp2_dn8_slot = var_sp_s_temp2_dn8;
        *var_sp_s_temp2_rv_slot = var_sp_s_temp2_rv;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_rv_slot = var_sp_s_xi0_rv;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_rv_slot = var_sp_s_xi1_rv;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_rv_slot = var_sp_s_xi2_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_udse_slot = var_udse;
        *var_udse_dn5_slot = var_udse_dn5;
        *var_udse_dn6_slot = var_udse_dn6;
        *var_udse_dn7_slot = var_udse_dn7;
        *var_udse_dn8_slot = var_udse_dn8;
        *var_udse_rv_slot = var_udse_rv;
        *var_v_dsat_slot = var_v_dsat;
        *var_v_dsat_dn5_slot = var_v_dsat_dn5;
        *var_v_dsat_dn6_slot = var_v_dsat_dn6;
        *var_v_dsat_dn7_slot = var_v_dsat_dn7;
        *var_v_dsat_dn8_slot = var_v_dsat_dn8;
        *var_v_dsat_rv_slot = var_v_dsat_rv;
        *var_vdse_slot = var_vdse;
        *var_vdse_dn5_slot = var_vdse_dn5;
        *var_vdse_dn6_slot = var_vdse_dn6;
        *var_vdse_dn7_slot = var_vdse_dn7;
        *var_vdse_dn8_slot = var_vdse_dn8;
        *var_vdse_rv_slot = var_vdse_rv;
        *var_x_0_slot = var_x_0;
        *var_x_0_dn5_slot = var_x_0_dn5;
        *var_x_0_dn6_slot = var_x_0_dn6;
        *var_x_0_dn7_slot = var_x_0_dn7;
        *var_x_0_dn8_slot = var_x_0_dn8;
        *var_x_0_rv_slot = var_x_0_rv;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_d_rv_slot = var_x_d_rv;
        *var_x_sat_slot = var_x_sat;
        *var_x_sat_dn5_slot = var_x_sat_dn5;
        *var_x_sat_dn6_slot = var_x_sat_dn6;
        *var_x_sat_dn7_slot = var_x_sat_dn7;
        *var_x_sat_dn8_slot = var_x_sat_dn8;
        *var_x_sat_rv_slot = var_x_sat_rv;
        *var_xn_d_slot = var_xn_d;
        *var_xn_d_dn5_slot = var_xn_d_dn5;
        *var_xn_d_dn6_slot = var_xn_d_dn6;
        *var_xn_d_dn7_slot = var_xn_d_dn7;
        *var_xn_d_dn8_slot = var_xn_d_dn8;
        *var_xn_d_rv_slot = var_xn_d_rv;
    }

    pub(super) fn stamp_reactive_block_33(
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_nd: f64,
        var_delta_nd_dn5: f64,
        var_delta_nd_dn6: f64,
        var_delta_nd_dn7: f64,
        var_delta_nd_dn8: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_guard1197: f64,
        var_guard1205: f64,
        var_k_ds: f64,
        var_k_ds_dn5: f64,
        var_k_ds_dn6: f64,
        var_k_ds_dn7: f64,
        var_k_ds_dn8: f64,
        var_sp_s_a: f64,
        var_sp_s_a_dn5: f64,
        var_sp_s_a_dn6: f64,
        var_sp_s_a_dn7: f64,
        var_sp_s_a_dn8: f64,
        var_sp_s_eta: f64,
        var_sp_s_eta_dn5: f64,
        var_sp_s_eta_dn6: f64,
        var_sp_s_eta_dn7: f64,
        var_sp_s_eta_dn8: f64,
        var_sp_s_temp1: f64,
        var_sp_s_temp1_dn5: f64,
        var_sp_s_temp1_dn6: f64,
        var_sp_s_temp1_dn7: f64,
        var_sp_s_temp1_dn8: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xg: f64,
        var_xg_dn5: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xi1s: f64,
        var_xi1s_dn5: f64,
        var_xi1s_dn6: f64,
        var_xi1s_dn7: f64,
        var_xi1s_dn8: f64,
        var_xn_d: f64,
        var_xn_d_dn5: f64,
        var_xn_d_dn6: f64,
        var_xn_d_dn7: f64,
        var_xn_d_dn8: f64,
        var_guard1206_slot: &mut f64,
        var_guard1206_rv_slot: &mut f64,
        var_guard1207_slot: &mut f64,
        var_guard1207_rv_slot: &mut f64,
        var_guard1208_slot: &mut f64,
        var_guard1208_rv_slot: &mut f64,
        var_mutau_slot: &mut f64,
        var_mutau_dn5_slot: &mut f64,
        var_mutau_dn6_slot: &mut f64,
        var_mutau_dn7_slot: &mut f64,
        var_mutau_dn8_slot: &mut f64,
        var_mutau_rv_slot: &mut f64,
        var_nu_slot: &mut f64,
        var_nu_dn5_slot: &mut f64,
        var_nu_dn6_slot: &mut f64,
        var_nu_dn7_slot: &mut f64,
        var_nu_dn8_slot: &mut f64,
        var_nu_rv_slot: &mut f64,
        var_pc_slot: &mut f64,
        var_pc_dn5_slot: &mut f64,
        var_pc_dn6_slot: &mut f64,
        var_pc_dn7_slot: &mut f64,
        var_pc_dn8_slot: &mut f64,
        var_pc_rv_slot: &mut f64,
        var_qc_slot: &mut f64,
        var_qc_dn5_slot: &mut f64,
        var_qc_dn6_slot: &mut f64,
        var_qc_dn7_slot: &mut f64,
        var_qc_dn8_slot: &mut f64,
        var_qc_rv_slot: &mut f64,
        var_sp_s_b_slot: &mut f64,
        var_sp_s_b_dn5_slot: &mut f64,
        var_sp_s_b_dn6_slot: &mut f64,
        var_sp_s_b_dn7_slot: &mut f64,
        var_sp_s_b_dn8_slot: &mut f64,
        var_sp_s_b_rv_slot: &mut f64,
        var_sp_s_c_slot: &mut f64,
        var_sp_s_c_dn5_slot: &mut f64,
        var_sp_s_c_dn6_slot: &mut f64,
        var_sp_s_c_dn7_slot: &mut f64,
        var_sp_s_c_dn8_slot: &mut f64,
        var_sp_s_c_rv_slot: &mut f64,
        var_sp_s_delta0_slot: &mut f64,
        var_sp_s_delta0_dn5_slot: &mut f64,
        var_sp_s_delta0_dn6_slot: &mut f64,
        var_sp_s_delta0_dn7_slot: &mut f64,
        var_sp_s_delta0_dn8_slot: &mut f64,
        var_sp_s_delta0_rv_slot: &mut f64,
        var_sp_s_delta1_slot: &mut f64,
        var_sp_s_delta1_dn5_slot: &mut f64,
        var_sp_s_delta1_dn6_slot: &mut f64,
        var_sp_s_delta1_dn7_slot: &mut f64,
        var_sp_s_delta1_dn8_slot: &mut f64,
        var_sp_s_delta1_rv_slot: &mut f64,
        var_sp_s_pc_slot: &mut f64,
        var_sp_s_pc_dn5_slot: &mut f64,
        var_sp_s_pc_dn6_slot: &mut f64,
        var_sp_s_pc_dn7_slot: &mut f64,
        var_sp_s_pc_dn8_slot: &mut f64,
        var_sp_s_pc_rv_slot: &mut f64,
        var_sp_s_qc_slot: &mut f64,
        var_sp_s_qc_dn5_slot: &mut f64,
        var_sp_s_qc_dn6_slot: &mut f64,
        var_sp_s_qc_dn7_slot: &mut f64,
        var_sp_s_qc_dn8_slot: &mut f64,
        var_sp_s_qc_rv_slot: &mut f64,
        var_sp_s_tau_slot: &mut f64,
        var_sp_s_tau_dn5_slot: &mut f64,
        var_sp_s_tau_dn6_slot: &mut f64,
        var_sp_s_tau_dn7_slot: &mut f64,
        var_sp_s_tau_dn8_slot: &mut f64,
        var_sp_s_tau_rv_slot: &mut f64,
        var_sp_s_temp_slot: &mut f64,
        var_sp_s_temp_dn5_slot: &mut f64,
        var_sp_s_temp_dn6_slot: &mut f64,
        var_sp_s_temp_dn7_slot: &mut f64,
        var_sp_s_temp_dn8_slot: &mut f64,
        var_sp_s_temp_rv_slot: &mut f64,
        var_sp_s_x0_slot: &mut f64,
        var_sp_s_x0_dn5_slot: &mut f64,
        var_sp_s_x0_dn6_slot: &mut f64,
        var_sp_s_x0_dn7_slot: &mut f64,
        var_sp_s_x0_dn8_slot: &mut f64,
        var_sp_s_x0_rv_slot: &mut f64,
        var_sp_s_xi0_slot: &mut f64,
        var_sp_s_xi0_dn5_slot: &mut f64,
        var_sp_s_xi0_dn6_slot: &mut f64,
        var_sp_s_xi0_dn7_slot: &mut f64,
        var_sp_s_xi0_dn8_slot: &mut f64,
        var_sp_s_xi0_rv_slot: &mut f64,
        var_sp_s_xi1_slot: &mut f64,
        var_sp_s_xi1_dn5_slot: &mut f64,
        var_sp_s_xi1_dn6_slot: &mut f64,
        var_sp_s_xi1_dn7_slot: &mut f64,
        var_sp_s_xi1_dn8_slot: &mut f64,
        var_sp_s_xi1_rv_slot: &mut f64,
        var_sp_s_xi2_slot: &mut f64,
        var_sp_s_xi2_dn5_slot: &mut f64,
        var_sp_s_xi2_dn6_slot: &mut f64,
        var_sp_s_xi2_dn7_slot: &mut f64,
        var_sp_s_xi2_dn8_slot: &mut f64,
        var_sp_s_xi2_rv_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_d_rv_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn5_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_ds_rv_slot: &mut f64,
    ) {
        let mut var_guard1206: f64 = *var_guard1206_slot;
        let mut var_guard1206_rv: f64 = *var_guard1206_rv_slot;
        let mut var_guard1207: f64 = *var_guard1207_slot;
        let mut var_guard1207_rv: f64 = *var_guard1207_rv_slot;
        let mut var_guard1208: f64 = *var_guard1208_slot;
        let mut var_guard1208_rv: f64 = *var_guard1208_rv_slot;
        let mut var_mutau: f64 = *var_mutau_slot;
        let mut var_mutau_dn5: f64 = *var_mutau_dn5_slot;
        let mut var_mutau_dn6: f64 = *var_mutau_dn6_slot;
        let mut var_mutau_dn7: f64 = *var_mutau_dn7_slot;
        let mut var_mutau_dn8: f64 = *var_mutau_dn8_slot;
        let mut var_mutau_rv: f64 = *var_mutau_rv_slot;
        let mut var_nu: f64 = *var_nu_slot;
        let mut var_nu_dn5: f64 = *var_nu_dn5_slot;
        let mut var_nu_dn6: f64 = *var_nu_dn6_slot;
        let mut var_nu_dn7: f64 = *var_nu_dn7_slot;
        let mut var_nu_dn8: f64 = *var_nu_dn8_slot;
        let mut var_nu_rv: f64 = *var_nu_rv_slot;
        let mut var_pc: f64 = *var_pc_slot;
        let mut var_pc_dn5: f64 = *var_pc_dn5_slot;
        let mut var_pc_dn6: f64 = *var_pc_dn6_slot;
        let mut var_pc_dn7: f64 = *var_pc_dn7_slot;
        let mut var_pc_dn8: f64 = *var_pc_dn8_slot;
        let mut var_pc_rv: f64 = *var_pc_rv_slot;
        let mut var_qc: f64 = *var_qc_slot;
        let mut var_qc_dn5: f64 = *var_qc_dn5_slot;
        let mut var_qc_dn6: f64 = *var_qc_dn6_slot;
        let mut var_qc_dn7: f64 = *var_qc_dn7_slot;
        let mut var_qc_dn8: f64 = *var_qc_dn8_slot;
        let mut var_qc_rv: f64 = *var_qc_rv_slot;
        let mut var_sp_s_b: f64 = *var_sp_s_b_slot;
        let mut var_sp_s_b_dn5: f64 = *var_sp_s_b_dn5_slot;
        let mut var_sp_s_b_dn6: f64 = *var_sp_s_b_dn6_slot;
        let mut var_sp_s_b_dn7: f64 = *var_sp_s_b_dn7_slot;
        let mut var_sp_s_b_dn8: f64 = *var_sp_s_b_dn8_slot;
        let mut var_sp_s_b_rv: f64 = *var_sp_s_b_rv_slot;
        let mut var_sp_s_c: f64 = *var_sp_s_c_slot;
        let mut var_sp_s_c_dn5: f64 = *var_sp_s_c_dn5_slot;
        let mut var_sp_s_c_dn6: f64 = *var_sp_s_c_dn6_slot;
        let mut var_sp_s_c_dn7: f64 = *var_sp_s_c_dn7_slot;
        let mut var_sp_s_c_dn8: f64 = *var_sp_s_c_dn8_slot;
        let mut var_sp_s_c_rv: f64 = *var_sp_s_c_rv_slot;
        let mut var_sp_s_delta0: f64 = *var_sp_s_delta0_slot;
        let mut var_sp_s_delta0_dn5: f64 = *var_sp_s_delta0_dn5_slot;
        let mut var_sp_s_delta0_dn6: f64 = *var_sp_s_delta0_dn6_slot;
        let mut var_sp_s_delta0_dn7: f64 = *var_sp_s_delta0_dn7_slot;
        let mut var_sp_s_delta0_dn8: f64 = *var_sp_s_delta0_dn8_slot;
        let mut var_sp_s_delta0_rv: f64 = *var_sp_s_delta0_rv_slot;
        let mut var_sp_s_delta1: f64 = *var_sp_s_delta1_slot;
        let mut var_sp_s_delta1_dn5: f64 = *var_sp_s_delta1_dn5_slot;
        let mut var_sp_s_delta1_dn6: f64 = *var_sp_s_delta1_dn6_slot;
        let mut var_sp_s_delta1_dn7: f64 = *var_sp_s_delta1_dn7_slot;
        let mut var_sp_s_delta1_dn8: f64 = *var_sp_s_delta1_dn8_slot;
        let mut var_sp_s_delta1_rv: f64 = *var_sp_s_delta1_rv_slot;
        let mut var_sp_s_pc: f64 = *var_sp_s_pc_slot;
        let mut var_sp_s_pc_dn5: f64 = *var_sp_s_pc_dn5_slot;
        let mut var_sp_s_pc_dn6: f64 = *var_sp_s_pc_dn6_slot;
        let mut var_sp_s_pc_dn7: f64 = *var_sp_s_pc_dn7_slot;
        let mut var_sp_s_pc_dn8: f64 = *var_sp_s_pc_dn8_slot;
        let mut var_sp_s_pc_rv: f64 = *var_sp_s_pc_rv_slot;
        let mut var_sp_s_qc: f64 = *var_sp_s_qc_slot;
        let mut var_sp_s_qc_dn5: f64 = *var_sp_s_qc_dn5_slot;
        let mut var_sp_s_qc_dn6: f64 = *var_sp_s_qc_dn6_slot;
        let mut var_sp_s_qc_dn7: f64 = *var_sp_s_qc_dn7_slot;
        let mut var_sp_s_qc_dn8: f64 = *var_sp_s_qc_dn8_slot;
        let mut var_sp_s_qc_rv: f64 = *var_sp_s_qc_rv_slot;
        let mut var_sp_s_tau: f64 = *var_sp_s_tau_slot;
        let mut var_sp_s_tau_dn5: f64 = *var_sp_s_tau_dn5_slot;
        let mut var_sp_s_tau_dn6: f64 = *var_sp_s_tau_dn6_slot;
        let mut var_sp_s_tau_dn7: f64 = *var_sp_s_tau_dn7_slot;
        let mut var_sp_s_tau_dn8: f64 = *var_sp_s_tau_dn8_slot;
        let mut var_sp_s_tau_rv: f64 = *var_sp_s_tau_rv_slot;
        let mut var_sp_s_temp: f64 = *var_sp_s_temp_slot;
        let mut var_sp_s_temp_dn5: f64 = *var_sp_s_temp_dn5_slot;
        let mut var_sp_s_temp_dn6: f64 = *var_sp_s_temp_dn6_slot;
        let mut var_sp_s_temp_dn7: f64 = *var_sp_s_temp_dn7_slot;
        let mut var_sp_s_temp_dn8: f64 = *var_sp_s_temp_dn8_slot;
        let mut var_sp_s_temp_rv: f64 = *var_sp_s_temp_rv_slot;
        let mut var_sp_s_x0: f64 = *var_sp_s_x0_slot;
        let mut var_sp_s_x0_dn5: f64 = *var_sp_s_x0_dn5_slot;
        let mut var_sp_s_x0_dn6: f64 = *var_sp_s_x0_dn6_slot;
        let mut var_sp_s_x0_dn7: f64 = *var_sp_s_x0_dn7_slot;
        let mut var_sp_s_x0_dn8: f64 = *var_sp_s_x0_dn8_slot;
        let mut var_sp_s_x0_rv: f64 = *var_sp_s_x0_rv_slot;
        let mut var_sp_s_xi0: f64 = *var_sp_s_xi0_slot;
        let mut var_sp_s_xi0_dn5: f64 = *var_sp_s_xi0_dn5_slot;
        let mut var_sp_s_xi0_dn6: f64 = *var_sp_s_xi0_dn6_slot;
        let mut var_sp_s_xi0_dn7: f64 = *var_sp_s_xi0_dn7_slot;
        let mut var_sp_s_xi0_dn8: f64 = *var_sp_s_xi0_dn8_slot;
        let mut var_sp_s_xi0_rv: f64 = *var_sp_s_xi0_rv_slot;
        let mut var_sp_s_xi1: f64 = *var_sp_s_xi1_slot;
        let mut var_sp_s_xi1_dn5: f64 = *var_sp_s_xi1_dn5_slot;
        let mut var_sp_s_xi1_dn6: f64 = *var_sp_s_xi1_dn6_slot;
        let mut var_sp_s_xi1_dn7: f64 = *var_sp_s_xi1_dn7_slot;
        let mut var_sp_s_xi1_dn8: f64 = *var_sp_s_xi1_dn8_slot;
        let mut var_sp_s_xi1_rv: f64 = *var_sp_s_xi1_rv_slot;
        let mut var_sp_s_xi2: f64 = *var_sp_s_xi2_slot;
        let mut var_sp_s_xi2_dn5: f64 = *var_sp_s_xi2_dn5_slot;
        let mut var_sp_s_xi2_dn6: f64 = *var_sp_s_xi2_dn6_slot;
        let mut var_sp_s_xi2_dn7: f64 = *var_sp_s_xi2_dn7_slot;
        let mut var_sp_s_xi2_dn8: f64 = *var_sp_s_xi2_dn8_slot;
        let mut var_sp_s_xi2_rv: f64 = *var_sp_s_xi2_rv_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_d_rv: f64 = *var_x_d_rv_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn5: f64 = *var_x_ds_dn5_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_ds_rv: f64 = *var_x_ds_rv_slot;

        let (assign44230_e57162, assign44230_e57162_d_n5, assign44230_e57162_d_n6, assign44230_e57162_d_n7, assign44230_e57162_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44230_e57156: f64 = (var_delta_nd * var_sp_s_xi2);
        let assign44230_e57157: f64 = (var_sp_s_temp1 - assign44230_e57156);
        let assign44230_e57158: f64 = (var_gf2 * assign44230_e57157);
        let assign44230_e57159: f64 = (0.5 * assign44230_e57158);
        let assign44230_e57160: f64 = (1.0 - assign44230_e57159);
        (assign44230_e57160, (-(0.5 * ((var_gf2_dn5 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn5 - ((var_delta_nd_dn5 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn5))))))), (-(0.5 * ((var_gf2_dn6 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn6 - ((var_delta_nd_dn6 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn6))))))), (-(0.5 * ((var_gf2_dn7 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn7 - ((var_delta_nd_dn7 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn7))))))), (-(0.5 * ((var_gf2_dn8 * assign44230_e57157) + (var_gf2 * (var_sp_s_temp1_dn8 - ((var_delta_nd_dn8 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn8))))))),)
    } else {
        (var_sp_s_b, var_sp_s_b_dn5, var_sp_s_b_dn6, var_sp_s_b_dn7, var_sp_s_b_dn8,)
    }
};
        var_sp_s_b = assign44230_e57162;
        var_sp_s_b_dn5 = assign44230_e57162_d_n5;
        var_sp_s_b_dn6 = assign44230_e57162_d_n6;
        var_sp_s_b_dn7 = assign44230_e57162_d_n7;
        var_sp_s_b_dn8 = assign44230_e57162_d_n8;
        var_sp_s_b_rv = 0.0;

        let (assign44240_e57183, assign44240_e57183_d_n5, assign44240_e57183_d_n6, assign44240_e57183_d_n7, assign44240_e57183_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44240_e57169: f64 = (2.0 * var_sp_s_temp);
        let assign44240_e57173: f64 = (1.0 - var_sp_s_temp1);
        let assign44240_e57177: f64 = (1.0 + var_sp_s_xi1);
        let assign44240_e57178: f64 = (var_delta_nd * assign44240_e57177);
        let assign44240_e57179: f64 = (assign44240_e57173 - assign44240_e57178);
        let assign44240_e57180: f64 = (var_gf2 * assign44240_e57179);
        let assign44240_e57181: f64 = (assign44240_e57169 + assign44240_e57180);
        (assign44240_e57181, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn5) - ((var_delta_nd_dn5 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn6) - ((var_delta_nd_dn6 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn7) - ((var_delta_nd_dn7 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign44240_e57179) + (var_gf2 * ((-var_sp_s_temp1_dn8) - ((var_delta_nd_dn8 * assign44240_e57177) + (var_delta_nd * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_c, var_sp_s_c_dn5, var_sp_s_c_dn6, var_sp_s_c_dn7, var_sp_s_c_dn8,)
    }
};
        var_sp_s_c = assign44240_e57183;
        var_sp_s_c_dn5 = assign44240_e57183_d_n5;
        var_sp_s_c_dn6 = assign44240_e57183_d_n6;
        var_sp_s_c_dn7 = assign44240_e57183_d_n7;
        var_sp_s_c_dn8 = assign44240_e57183_d_n8;
        var_sp_s_c_rv = 0.0;

        let (assign44250_e57197, assign44250_e57197_d_n5, assign44250_e57197_d_n6, assign44250_e57197_d_n7, assign44250_e57197_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44250_e57190: f64 = (var_xn_d - var_sp_s_eta);
        let assign44250_e57193: f64 = (var_sp_s_a / var_gf2);
        let assign44250_e57194: f64 = (assign44250_e57193).ln();
        let assign44250_e57195: f64 = (assign44250_e57190 + assign44250_e57194);
        (assign44250_e57195, ((var_xn_d_dn5 - var_sp_s_eta_dn5) + ((((var_sp_s_a_dn5 * var_gf2) - (var_sp_s_a * var_gf2_dn5)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn6 - var_sp_s_eta_dn6) + ((((var_sp_s_a_dn6 * var_gf2) - (var_sp_s_a * var_gf2_dn6)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn7 - var_sp_s_eta_dn7) + ((((var_sp_s_a_dn7 * var_gf2) - (var_sp_s_a * var_gf2_dn7)) / (var_gf2 * var_gf2)) / assign44250_e57193)), ((var_xn_d_dn8 - var_sp_s_eta_dn8) + ((((var_sp_s_a_dn8 * var_gf2) - (var_sp_s_a * var_gf2_dn8)) / (var_gf2 * var_gf2)) / assign44250_e57193)),)
    } else {
        (var_sp_s_tau, var_sp_s_tau_dn5, var_sp_s_tau_dn6, var_sp_s_tau_dn7, var_sp_s_tau_dn8,)
    }
};
        var_sp_s_tau = assign44250_e57197;
        var_sp_s_tau_dn5 = assign44250_e57197_d_n5;
        var_sp_s_tau_dn6 = assign44250_e57197_d_n6;
        var_sp_s_tau_dn7 = assign44250_e57197_d_n7;
        var_sp_s_tau_dn8 = assign44250_e57197_d_n8;
        var_sp_s_tau_rv = 0.0;

        let (assign44260_e57206, assign44260_e57206_d_n5, assign44260_e57206_d_n6, assign44260_e57206_d_n7, assign44260_e57206_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44260_e57204: f64 = (var_sp_s_a + var_sp_s_c);
        (assign44260_e57204, (var_sp_s_a_dn5 + var_sp_s_c_dn5), (var_sp_s_a_dn6 + var_sp_s_c_dn6), (var_sp_s_a_dn7 + var_sp_s_c_dn7), (var_sp_s_a_dn8 + var_sp_s_c_dn8),)
    } else {
        (var_nu, var_nu_dn5, var_nu_dn6, var_nu_dn7, var_nu_dn8,)
    }
};
        var_nu = assign44260_e57206;
        var_nu_dn5 = assign44260_e57206_d_n5;
        var_nu_dn6 = assign44260_e57206_d_n6;
        var_nu_dn7 = assign44260_e57206_d_n7;
        var_nu_dn8 = assign44260_e57206_d_n8;
        var_nu_rv = 0.0;

        let (assign44270_e57227, assign44270_e57227_d_n5, assign44270_e57227_d_n6, assign44270_e57227_d_n7, assign44270_e57227_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44270_e57213: f64 = (var_nu * var_nu);
        let assign44270_e57218: f64 = (var_sp_s_c * var_sp_s_c);
        let assign44270_e57219: f64 = (0.5 * assign44270_e57218);
        let assign44270_e57222: f64 = (var_sp_s_a * var_sp_s_b);
        let assign44270_e57223: f64 = (assign44270_e57219 - assign44270_e57222);
        let assign44270_e57224: f64 = (var_sp_s_tau * assign44270_e57223);
        let assign44270_e57225: f64 = (assign44270_e57213 + assign44270_e57224);
        (assign44270_e57225, (((var_nu_dn5 * var_nu) + (var_nu * var_nu_dn5)) + ((var_sp_s_tau_dn5 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5))) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))), (((var_nu_dn6 * var_nu) + (var_nu * var_nu_dn6)) + ((var_sp_s_tau_dn6 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6))) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))), (((var_nu_dn7 * var_nu) + (var_nu * var_nu_dn7)) + ((var_sp_s_tau_dn7 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7))) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))), (((var_nu_dn8 * var_nu) + (var_nu * var_nu_dn8)) + ((var_sp_s_tau_dn8 * assign44270_e57223) + (var_sp_s_tau * ((0.5 * ((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8))) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))),)
    } else {
        (var_mutau, var_mutau_dn5, var_mutau_dn6, var_mutau_dn7, var_mutau_dn8,)
    }
};
        var_mutau = assign44270_e57227;
        var_mutau_dn5 = assign44270_e57227_d_n5;
        var_mutau_dn6 = assign44270_e57227_d_n6;
        var_mutau_dn7 = assign44270_e57227_d_n7;
        var_mutau_dn8 = assign44270_e57227_d_n8;
        var_mutau_rv = 0.0;

        let (assign44280_e57262, assign44280_e57262_d_n5, assign44280_e57262_d_n6, assign44280_e57262_d_n7, assign44280_e57262_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44280_e57235: f64 = (var_sp_s_a * var_nu);
        let assign44280_e57237: f64 = (assign44280_e57235 * var_sp_s_tau);
        let assign44280_e57241: f64 = (var_nu / var_mutau);
        let assign44280_e57243: f64 = (assign44280_e57241 * var_sp_s_tau);
        let assign44280_e57245: f64 = (assign44280_e57243 * var_sp_s_tau);
        let assign44280_e57247: f64 = (assign44280_e57245 * var_sp_s_c);
        let assign44280_e57250: f64 = (var_sp_s_c * var_sp_s_c);
        let assign44280_e57252: f64 = (assign44280_e57250 * 0.3333333333333333);
        let assign44280_e57255: f64 = (var_sp_s_a * var_sp_s_b);
        let assign44280_e57256: f64 = (assign44280_e57252 - assign44280_e57255);
        let assign44280_e57257: f64 = (assign44280_e57247 * assign44280_e57256);
        let assign44280_e57258: f64 = (var_mutau + assign44280_e57257);
        let assign44280_e57259: f64 = (assign44280_e57237 / assign44280_e57258);
        let assign44280_e57260: f64 = (var_sp_s_eta + assign44280_e57259);
        (assign44280_e57260, (var_sp_s_eta_dn5 + (((((((var_sp_s_a_dn5 * var_nu) + (var_sp_s_a * var_nu_dn5)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn5)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn5 + (((((((((((var_nu_dn5 * var_mutau) - (var_nu * var_mutau_dn5)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn5)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn5)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn5)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn5 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn5)) * 0.3333333333333333) - ((var_sp_s_a_dn5 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn5)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn6 + (((((((var_sp_s_a_dn6 * var_nu) + (var_sp_s_a * var_nu_dn6)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn6)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn6 + (((((((((((var_nu_dn6 * var_mutau) - (var_nu * var_mutau_dn6)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn6)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn6)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn6)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn6 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn6)) * 0.3333333333333333) - ((var_sp_s_a_dn6 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn6)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn7 + (((((((var_sp_s_a_dn7 * var_nu) + (var_sp_s_a * var_nu_dn7)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn7)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn7 + (((((((((((var_nu_dn7 * var_mutau) - (var_nu * var_mutau_dn7)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn7)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn7)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn7)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn7 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn7)) * 0.3333333333333333) - ((var_sp_s_a_dn7 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn7)))))))) / (assign44280_e57258 * assign44280_e57258))), (var_sp_s_eta_dn8 + (((((((var_sp_s_a_dn8 * var_nu) + (var_sp_s_a * var_nu_dn8)) * var_sp_s_tau) + (assign44280_e57235 * var_sp_s_tau_dn8)) * assign44280_e57258) - (assign44280_e57237 * (var_mutau_dn8 + (((((((((((var_nu_dn8 * var_mutau) - (var_nu * var_mutau_dn8)) / (var_mutau * var_mutau)) * var_sp_s_tau) + (assign44280_e57241 * var_sp_s_tau_dn8)) * var_sp_s_tau) + (assign44280_e57243 * var_sp_s_tau_dn8)) * var_sp_s_c) + (assign44280_e57245 * var_sp_s_c_dn8)) * assign44280_e57256) + (assign44280_e57247 * ((((var_sp_s_c_dn8 * var_sp_s_c) + (var_sp_s_c * var_sp_s_c_dn8)) * 0.3333333333333333) - ((var_sp_s_a_dn8 * var_sp_s_b) + (var_sp_s_a * var_sp_s_b_dn8)))))))) / (assign44280_e57258 * assign44280_e57258))),)
    } else {
        (var_sp_s_x0, var_sp_s_x0_dn5, var_sp_s_x0_dn6, var_sp_s_x0_dn7, var_sp_s_x0_dn8,)
    }
};
        var_sp_s_x0 = assign44280_e57262;
        var_sp_s_x0_dn5 = assign44280_e57262_d_n5;
        var_sp_s_x0_dn6 = assign44280_e57262_d_n6;
        var_sp_s_x0_dn7 = assign44280_e57262_d_n7;
        var_sp_s_x0_dn8 = assign44280_e57262_d_n8;
        var_sp_s_x0_rv = 0.0;

        let assign44290_e57265: f64 = if var_sp_s_x0 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1206 = assign44290_e57265;
        var_guard1206_rv = 0.0;

        let (assign44300_e57275, assign44300_e57275_d_n5, assign44300_e57275_d_n6, assign44300_e57275_d_n7, assign44300_e57275_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44300_e57273: f64 = (var_sp_s_x0).exp();
        (assign44300_e57273, (assign44300_e57273 * var_sp_s_x0_dn5), (assign44300_e57273 * var_sp_s_x0_dn6), (assign44300_e57273 * var_sp_s_x0_dn7), (assign44300_e57273 * var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44300_e57275;
        var_sp_s_delta0_dn5 = assign44300_e57275_d_n5;
        var_sp_s_delta0_dn6 = assign44300_e57275_d_n6;
        var_sp_s_delta0_dn7 = assign44300_e57275_d_n7;
        var_sp_s_delta0_dn8 = assign44300_e57275_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign44310_e57286, assign44310_e57286_d_n5, assign44310_e57286_d_n6, assign44310_e57286_d_n7, assign44310_e57286_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44310_e57284: f64 = (1.0 / var_sp_s_delta0);
        (assign44310_e57284, (-(var_sp_s_delta0_dn5 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn6 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn7 / (var_sp_s_delta0 * var_sp_s_delta0))), (-(var_sp_s_delta0_dn8 / (var_sp_s_delta0 * var_sp_s_delta0))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign44310_e57286;
        var_sp_s_delta1_dn5 = assign44310_e57286_d_n5;
        var_sp_s_delta1_dn6 = assign44310_e57286_d_n6;
        var_sp_s_delta1_dn7 = assign44310_e57286_d_n7;
        var_sp_s_delta1_dn8 = assign44310_e57286_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign44320_e57297, assign44320_e57297_d_n5, assign44320_e57297_d_n6, assign44320_e57297_d_n7, assign44320_e57297_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 != 0.0)) {
        let assign44320_e57295: f64 = (var_delta_nd * var_sp_s_delta0);
        (assign44320_e57295, ((var_delta_nd_dn5 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn5)), ((var_delta_nd_dn6 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn6)), ((var_delta_nd_dn7 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn7)), ((var_delta_nd_dn8 * var_sp_s_delta0) + (var_delta_nd * var_sp_s_delta0_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44320_e57297;
        var_sp_s_delta0_dn5 = assign44320_e57297_d_n5;
        var_sp_s_delta0_dn6 = assign44320_e57297_d_n6;
        var_sp_s_delta0_dn7 = assign44320_e57297_d_n7;
        var_sp_s_delta0_dn8 = assign44320_e57297_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let assign44330_e57301: f64 = (var_xn_d - 230.25850929940458);
        let assign44330_e57302: f64 = if var_sp_s_x0 > assign44330_e57301 { 1.0 } else { 0.0 };
        var_guard1207 = assign44330_e57302;
        var_guard1207_rv = 0.0;

        let (assign44340_e57317, assign44340_e57317_d_n5, assign44340_e57317_d_n6, assign44340_e57317_d_n7, assign44340_e57317_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign44340_e57314: f64 = (var_sp_s_x0 - var_xn_d);
        let assign44340_e57315: f64 = (assign44340_e57314).exp();
        (assign44340_e57315, (assign44340_e57315 * (var_sp_s_x0_dn5 - var_xn_d_dn5)), (assign44340_e57315 * (var_sp_s_x0_dn6 - var_xn_d_dn6)), (assign44340_e57315 * (var_sp_s_x0_dn7 - var_xn_d_dn7)), (assign44340_e57315 * (var_sp_s_x0_dn8 - var_xn_d_dn8)),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44340_e57317;
        var_sp_s_delta0_dn5 = assign44340_e57317_d_n5;
        var_sp_s_delta0_dn6 = assign44340_e57317_d_n6;
        var_sp_s_delta0_dn7 = assign44340_e57317_d_n7;
        var_sp_s_delta0_dn8 = assign44340_e57317_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign44350_e57331, assign44350_e57331_d_n5, assign44350_e57331_d_n6, assign44350_e57331_d_n7, assign44350_e57331_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 != 0.0)) {
        let assign44350_e57329: f64 = (var_delta_nd / var_sp_s_delta0);
        (assign44350_e57329, (((var_delta_nd_dn5 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn5)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn6 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn6)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn7 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn7)) / (var_sp_s_delta0 * var_sp_s_delta0)), (((var_delta_nd_dn8 * var_sp_s_delta0) - (var_delta_nd * var_sp_s_delta0_dn8)) / (var_sp_s_delta0 * var_sp_s_delta0)),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign44350_e57331;
        var_sp_s_delta1_dn5 = assign44350_e57331_d_n5;
        var_sp_s_delta1_dn6 = assign44350_e57331_d_n6;
        var_sp_s_delta1_dn7 = assign44350_e57331_d_n7;
        var_sp_s_delta1_dn8 = assign44350_e57331_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign44360_e57372, assign44360_e57372_d_n5, assign44360_e57372_d_n6, assign44360_e57372_d_n7, assign44360_e57372_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 == 0.0)) {
        let assign44360_e57346: f64 = (var_xn_d - var_sp_s_x0);
        let assign44360_e57348: f64 = (assign44360_e57346 - 230.25850929940458);
        let assign44360_e57353: f64 = (var_xn_d - var_sp_s_x0);
        let assign44360_e57355: f64 = (assign44360_e57353 - 230.25850929940458);
        let assign44360_e57359: f64 = (var_xn_d - var_sp_s_x0);
        let assign44360_e57361: f64 = (assign44360_e57359 - 230.25850929940458);
        let assign44360_e57363: f64 = (assign44360_e57361 * 0.3333333333333333);
        let assign44360_e57364: f64 = (1.0 + assign44360_e57363);
        let assign44360_e57365: f64 = (assign44360_e57355 * assign44360_e57364);
        let assign44360_e57366: f64 = (0.5 * assign44360_e57365);
        let assign44360_e57367: f64 = (1.0 + assign44360_e57366);
        let assign44360_e57368: f64 = (assign44360_e57348 * assign44360_e57367);
        let assign44360_e57369: f64 = (1.0 + assign44360_e57368);
        let assign44360_e57370: f64 = (1e-100 / assign44360_e57369);
        (assign44360_e57370, (-((1e-100 * (((var_xn_d_dn5 - var_sp_s_x0_dn5) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((var_xn_d_dn5 - var_sp_s_x0_dn5) * assign44360_e57364) + (assign44360_e57355 * ((var_xn_d_dn5 - var_sp_s_x0_dn5) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((var_xn_d_dn6 - var_sp_s_x0_dn6) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((var_xn_d_dn6 - var_sp_s_x0_dn6) * assign44360_e57364) + (assign44360_e57355 * ((var_xn_d_dn6 - var_sp_s_x0_dn6) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((var_xn_d_dn7 - var_sp_s_x0_dn7) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((var_xn_d_dn7 - var_sp_s_x0_dn7) * assign44360_e57364) + (assign44360_e57355 * ((var_xn_d_dn7 - var_sp_s_x0_dn7) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))), (-((1e-100 * (((var_xn_d_dn8 - var_sp_s_x0_dn8) * assign44360_e57367) + (assign44360_e57348 * (0.5 * (((var_xn_d_dn8 - var_sp_s_x0_dn8) * assign44360_e57364) + (assign44360_e57355 * ((var_xn_d_dn8 - var_sp_s_x0_dn8) * 0.3333333333333333))))))) / (assign44360_e57369 * assign44360_e57369))),)
    } else {
        (var_sp_s_delta0, var_sp_s_delta0_dn5, var_sp_s_delta0_dn6, var_sp_s_delta0_dn7, var_sp_s_delta0_dn8,)
    }
};
        var_sp_s_delta0 = assign44360_e57372;
        var_sp_s_delta0_dn5 = assign44360_e57372_d_n5;
        var_sp_s_delta0_dn6 = assign44360_e57372_d_n6;
        var_sp_s_delta0_dn7 = assign44360_e57372_d_n7;
        var_sp_s_delta0_dn8 = assign44360_e57372_d_n8;
        var_sp_s_delta0_rv = 0.0;

        let (assign44370_e57407, assign44370_e57407_d_n5, assign44370_e57407_d_n6, assign44370_e57407_d_n7, assign44370_e57407_d_n8,) = {
    if ((((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) && (var_guard1206 == 0.0)) && (var_guard1207 == 0.0)) {
        let assign44370_e57387: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57392: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57396: f64 = (var_sp_s_x0 - 230.25850929940458);
        let assign44370_e57398: f64 = (assign44370_e57396 * 0.3333333333333333);
        let assign44370_e57399: f64 = (1.0 + assign44370_e57398);
        let assign44370_e57400: f64 = (assign44370_e57392 * assign44370_e57399);
        let assign44370_e57401: f64 = (0.5 * assign44370_e57400);
        let assign44370_e57402: f64 = (1.0 + assign44370_e57401);
        let assign44370_e57403: f64 = (assign44370_e57387 * assign44370_e57402);
        let assign44370_e57404: f64 = (1.0 + assign44370_e57403);
        let assign44370_e57405: f64 = (1e-100 / assign44370_e57404);
        (assign44370_e57405, (-((1e-100 * ((var_sp_s_x0_dn5 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((var_sp_s_x0_dn5 * assign44370_e57399) + (assign44370_e57392 * (var_sp_s_x0_dn5 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((var_sp_s_x0_dn6 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((var_sp_s_x0_dn6 * assign44370_e57399) + (assign44370_e57392 * (var_sp_s_x0_dn6 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((var_sp_s_x0_dn7 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((var_sp_s_x0_dn7 * assign44370_e57399) + (assign44370_e57392 * (var_sp_s_x0_dn7 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))), (-((1e-100 * ((var_sp_s_x0_dn8 * assign44370_e57402) + (assign44370_e57387 * (0.5 * ((var_sp_s_x0_dn8 * assign44370_e57399) + (assign44370_e57392 * (var_sp_s_x0_dn8 * 0.3333333333333333))))))) / (assign44370_e57404 * assign44370_e57404))),)
    } else {
        (var_sp_s_delta1, var_sp_s_delta1_dn5, var_sp_s_delta1_dn6, var_sp_s_delta1_dn7, var_sp_s_delta1_dn8,)
    }
};
        var_sp_s_delta1 = assign44370_e57407;
        var_sp_s_delta1_dn5 = assign44370_e57407_d_n5;
        var_sp_s_delta1_dn6 = assign44370_e57407_d_n6;
        var_sp_s_delta1_dn7 = assign44370_e57407_d_n7;
        var_sp_s_delta1_dn8 = assign44370_e57407_d_n8;
        var_sp_s_delta1_rv = 0.0;

        let (assign44380_e57420, assign44380_e57420_d_n5, assign44380_e57420_d_n6, assign44380_e57420_d_n7, assign44380_e57420_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44380_e57416: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign44380_e57417: f64 = (2.0 + assign44380_e57416);
        let assign44380_e57418: f64 = (1.0 / assign44380_e57417);
        (assign44380_e57418, (-(((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) / (assign44380_e57417 * assign44380_e57417))), (-(((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) / (assign44380_e57417 * assign44380_e57417))), (-(((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) / (assign44380_e57417 * assign44380_e57417))), (-(((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) / (assign44380_e57417 * assign44380_e57417))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44380_e57420;
        var_sp_s_temp_dn5 = assign44380_e57420_d_n5;
        var_sp_s_temp_dn6 = assign44380_e57420_d_n6;
        var_sp_s_temp_dn7 = assign44380_e57420_d_n7;
        var_sp_s_temp_dn8 = assign44380_e57420_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign44390_e57431, assign44390_e57431_d_n5, assign44390_e57431_d_n6, assign44390_e57431_d_n7, assign44390_e57431_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44390_e57427: f64 = (var_sp_s_x0 * var_sp_s_x0);
        let assign44390_e57429: f64 = (assign44390_e57427 * var_sp_s_temp);
        (assign44390_e57429, ((((var_sp_s_x0_dn5 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn5)) * var_sp_s_temp) + (assign44390_e57427 * var_sp_s_temp_dn5)), ((((var_sp_s_x0_dn6 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn6)) * var_sp_s_temp) + (assign44390_e57427 * var_sp_s_temp_dn6)), ((((var_sp_s_x0_dn7 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn7)) * var_sp_s_temp) + (assign44390_e57427 * var_sp_s_temp_dn7)), ((((var_sp_s_x0_dn8 * var_sp_s_x0) + (var_sp_s_x0 * var_sp_s_x0_dn8)) * var_sp_s_temp) + (assign44390_e57427 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi0, var_sp_s_xi0_dn5, var_sp_s_xi0_dn6, var_sp_s_xi0_dn7, var_sp_s_xi0_dn8,)
    }
};
        var_sp_s_xi0 = assign44390_e57431;
        var_sp_s_xi0_dn5 = assign44390_e57431_d_n5;
        var_sp_s_xi0_dn6 = assign44390_e57431_d_n6;
        var_sp_s_xi0_dn7 = assign44390_e57431_d_n7;
        var_sp_s_xi0_dn8 = assign44390_e57431_d_n8;
        var_sp_s_xi0_rv = 0.0;

        let (assign44400_e57444, assign44400_e57444_d_n5, assign44400_e57444_d_n6, assign44400_e57444_d_n7, assign44400_e57444_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44400_e57439: f64 = (var_sp_s_x0 * var_sp_s_temp);
        let assign44400_e57441: f64 = (assign44400_e57439 * var_sp_s_temp);
        let assign44400_e57442: f64 = (4.0 * assign44400_e57441);
        (assign44400_e57442, (4.0 * ((((var_sp_s_x0_dn5 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign44400_e57439 * var_sp_s_temp_dn5))), (4.0 * ((((var_sp_s_x0_dn6 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign44400_e57439 * var_sp_s_temp_dn6))), (4.0 * ((((var_sp_s_x0_dn7 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign44400_e57439 * var_sp_s_temp_dn7))), (4.0 * ((((var_sp_s_x0_dn8 * var_sp_s_temp) + (var_sp_s_x0 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign44400_e57439 * var_sp_s_temp_dn8))),)
    } else {
        (var_sp_s_xi1, var_sp_s_xi1_dn5, var_sp_s_xi1_dn6, var_sp_s_xi1_dn7, var_sp_s_xi1_dn8,)
    }
};
        var_sp_s_xi1 = assign44400_e57444;
        var_sp_s_xi1_dn5 = assign44400_e57444_d_n5;
        var_sp_s_xi1_dn6 = assign44400_e57444_d_n6;
        var_sp_s_xi1_dn7 = assign44400_e57444_d_n7;
        var_sp_s_xi1_dn8 = assign44400_e57444_d_n8;
        var_sp_s_xi1_rv = 0.0;

        let (assign44410_e57461, assign44410_e57461_d_n5, assign44410_e57461_d_n6, assign44410_e57461_d_n7, assign44410_e57461_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44410_e57451: f64 = (8.0 * var_sp_s_temp);
        let assign44410_e57454: f64 = (12.0 * var_sp_s_xi0);
        let assign44410_e57455: f64 = (assign44410_e57451 - assign44410_e57454);
        let assign44410_e57457: f64 = (assign44410_e57455 * var_sp_s_temp);
        let assign44410_e57459: f64 = (assign44410_e57457 * var_sp_s_temp);
        (assign44410_e57459, ((((((8.0 * var_sp_s_temp_dn5) - (12.0 * var_sp_s_xi0_dn5)) * var_sp_s_temp) + (assign44410_e57455 * var_sp_s_temp_dn5)) * var_sp_s_temp) + (assign44410_e57457 * var_sp_s_temp_dn5)), ((((((8.0 * var_sp_s_temp_dn6) - (12.0 * var_sp_s_xi0_dn6)) * var_sp_s_temp) + (assign44410_e57455 * var_sp_s_temp_dn6)) * var_sp_s_temp) + (assign44410_e57457 * var_sp_s_temp_dn6)), ((((((8.0 * var_sp_s_temp_dn7) - (12.0 * var_sp_s_xi0_dn7)) * var_sp_s_temp) + (assign44410_e57455 * var_sp_s_temp_dn7)) * var_sp_s_temp) + (assign44410_e57457 * var_sp_s_temp_dn7)), ((((((8.0 * var_sp_s_temp_dn8) - (12.0 * var_sp_s_xi0_dn8)) * var_sp_s_temp) + (assign44410_e57455 * var_sp_s_temp_dn8)) * var_sp_s_temp) + (assign44410_e57457 * var_sp_s_temp_dn8)),)
    } else {
        (var_sp_s_xi2, var_sp_s_xi2_dn5, var_sp_s_xi2_dn6, var_sp_s_xi2_dn7, var_sp_s_xi2_dn8,)
    }
};
        var_sp_s_xi2 = assign44410_e57461;
        var_sp_s_xi2_dn5 = assign44410_e57461_d_n5;
        var_sp_s_xi2_dn6 = assign44410_e57461_d_n6;
        var_sp_s_xi2_dn7 = assign44410_e57461_d_n7;
        var_sp_s_xi2_dn8 = assign44410_e57461_d_n8;
        var_sp_s_xi2_rv = 0.0;

        let (assign44420_e57470, assign44420_e57470_d_n5, assign44420_e57470_d_n6, assign44420_e57470_d_n7, assign44420_e57470_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44420_e57468: f64 = (var_xg - var_sp_s_x0);
        (assign44420_e57468, (var_xg_dn5 - var_sp_s_x0_dn5), (var_xg_dn6 - var_sp_s_x0_dn6), (var_xg_dn7 - var_sp_s_x0_dn7), (var_xg_dn8 - var_sp_s_x0_dn8),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44420_e57470;
        var_sp_s_temp_dn5 = assign44420_e57470_d_n5;
        var_sp_s_temp_dn6 = assign44420_e57470_d_n6;
        var_sp_s_temp_dn7 = assign44420_e57470_d_n7;
        var_sp_s_temp_dn8 = assign44420_e57470_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign44430_e57493, assign44430_e57493_d_n5, assign44430_e57493_d_n6, assign44430_e57493_d_n7, assign44430_e57493_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44430_e57477: f64 = (2.0 * var_sp_s_temp);
        let assign44430_e57481: f64 = (1.0 - var_sp_s_delta1);
        let assign44430_e57483: f64 = (assign44430_e57481 + var_sp_s_delta0);
        let assign44430_e57487: f64 = (1.0 + var_sp_s_xi1);
        let assign44430_e57488: f64 = (var_delta_nd * assign44430_e57487);
        let assign44430_e57489: f64 = (assign44430_e57483 - assign44430_e57488);
        let assign44430_e57490: f64 = (var_gf2 * assign44430_e57489);
        let assign44430_e57491: f64 = (assign44430_e57477 + assign44430_e57490);
        (assign44430_e57491, ((2.0 * var_sp_s_temp_dn5) + ((var_gf2_dn5 * assign44430_e57489) + (var_gf2 * (((-var_sp_s_delta1_dn5) + var_sp_s_delta0_dn5) - ((var_delta_nd_dn5 * assign44430_e57487) + (var_delta_nd * var_sp_s_xi1_dn5)))))), ((2.0 * var_sp_s_temp_dn6) + ((var_gf2_dn6 * assign44430_e57489) + (var_gf2 * (((-var_sp_s_delta1_dn6) + var_sp_s_delta0_dn6) - ((var_delta_nd_dn6 * assign44430_e57487) + (var_delta_nd * var_sp_s_xi1_dn6)))))), ((2.0 * var_sp_s_temp_dn7) + ((var_gf2_dn7 * assign44430_e57489) + (var_gf2 * (((-var_sp_s_delta1_dn7) + var_sp_s_delta0_dn7) - ((var_delta_nd_dn7 * assign44430_e57487) + (var_delta_nd * var_sp_s_xi1_dn7)))))), ((2.0 * var_sp_s_temp_dn8) + ((var_gf2_dn8 * assign44430_e57489) + (var_gf2 * (((-var_sp_s_delta1_dn8) + var_sp_s_delta0_dn8) - ((var_delta_nd_dn8 * assign44430_e57487) + (var_delta_nd * var_sp_s_xi1_dn8)))))),)
    } else {
        (var_sp_s_pc, var_sp_s_pc_dn5, var_sp_s_pc_dn6, var_sp_s_pc_dn7, var_sp_s_pc_dn8,)
    }
};
        var_sp_s_pc = assign44430_e57493;
        var_sp_s_pc_dn5 = assign44430_e57493_d_n5;
        var_sp_s_pc_dn6 = assign44430_e57493_d_n6;
        var_sp_s_pc_dn7 = assign44430_e57493_d_n7;
        var_sp_s_pc_dn8 = assign44430_e57493_d_n8;
        var_sp_s_pc_rv = 0.0;

        let (assign44440_e57520, assign44440_e57520_d_n5, assign44440_e57520_d_n6, assign44440_e57520_d_n7, assign44440_e57520_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44440_e57500: f64 = (var_sp_s_temp * var_sp_s_temp);
        let assign44440_e57504: f64 = (var_sp_s_delta1 + var_sp_s_x0);
        let assign44440_e57506: f64 = (assign44440_e57504 - 1.0);
        let assign44440_e57508: f64 = (assign44440_e57506 + var_sp_s_delta0);
        let assign44440_e57512: f64 = (var_sp_s_x0 + 1.0);
        let assign44440_e57514: f64 = (assign44440_e57512 + var_sp_s_xi0);
        let assign44440_e57515: f64 = (var_delta_nd * assign44440_e57514);
        let assign44440_e57516: f64 = (assign44440_e57508 - assign44440_e57515);
        let assign44440_e57517: f64 = (var_gf2 * assign44440_e57516);
        let assign44440_e57518: f64 = (assign44440_e57500 - assign44440_e57517);
        (assign44440_e57518, (((var_sp_s_temp_dn5 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn5)) - ((var_gf2_dn5 * assign44440_e57516) + (var_gf2 * (((var_sp_s_delta1_dn5 + var_sp_s_x0_dn5) + var_sp_s_delta0_dn5) - ((var_delta_nd_dn5 * assign44440_e57514) + (var_delta_nd * (var_sp_s_x0_dn5 + var_sp_s_xi0_dn5))))))), (((var_sp_s_temp_dn6 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn6)) - ((var_gf2_dn6 * assign44440_e57516) + (var_gf2 * (((var_sp_s_delta1_dn6 + var_sp_s_x0_dn6) + var_sp_s_delta0_dn6) - ((var_delta_nd_dn6 * assign44440_e57514) + (var_delta_nd * (var_sp_s_x0_dn6 + var_sp_s_xi0_dn6))))))), (((var_sp_s_temp_dn7 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn7)) - ((var_gf2_dn7 * assign44440_e57516) + (var_gf2 * (((var_sp_s_delta1_dn7 + var_sp_s_x0_dn7) + var_sp_s_delta0_dn7) - ((var_delta_nd_dn7 * assign44440_e57514) + (var_delta_nd * (var_sp_s_x0_dn7 + var_sp_s_xi0_dn7))))))), (((var_sp_s_temp_dn8 * var_sp_s_temp) + (var_sp_s_temp * var_sp_s_temp_dn8)) - ((var_gf2_dn8 * assign44440_e57516) + (var_gf2 * (((var_sp_s_delta1_dn8 + var_sp_s_x0_dn8) + var_sp_s_delta0_dn8) - ((var_delta_nd_dn8 * assign44440_e57514) + (var_delta_nd * (var_sp_s_x0_dn8 + var_sp_s_xi0_dn8))))))),)
    } else {
        (var_sp_s_qc, var_sp_s_qc_dn5, var_sp_s_qc_dn6, var_sp_s_qc_dn7, var_sp_s_qc_dn8,)
    }
};
        var_sp_s_qc = assign44440_e57520;
        var_sp_s_qc_dn5 = assign44440_e57520_d_n5;
        var_sp_s_qc_dn6 = assign44440_e57520_d_n6;
        var_sp_s_qc_dn7 = assign44440_e57520_d_n7;
        var_sp_s_qc_dn8 = assign44440_e57520_d_n8;
        var_sp_s_qc_rv = 0.0;

        let (assign44450_e57537, assign44450_e57537_d_n5, assign44450_e57537_d_n6, assign44450_e57537_d_n7, assign44450_e57537_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44450_e57529: f64 = (var_sp_s_delta1 + var_sp_s_delta0);
        let assign44450_e57532: f64 = (var_delta_nd * var_sp_s_xi2);
        let assign44450_e57533: f64 = (assign44450_e57529 - assign44450_e57532);
        let assign44450_e57534: f64 = (var_gf2 * assign44450_e57533);
        let assign44450_e57535: f64 = (2.0 - assign44450_e57534);
        (assign44450_e57535, (-((var_gf2_dn5 * assign44450_e57533) + (var_gf2 * ((var_sp_s_delta1_dn5 + var_sp_s_delta0_dn5) - ((var_delta_nd_dn5 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn5)))))), (-((var_gf2_dn6 * assign44450_e57533) + (var_gf2 * ((var_sp_s_delta1_dn6 + var_sp_s_delta0_dn6) - ((var_delta_nd_dn6 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn6)))))), (-((var_gf2_dn7 * assign44450_e57533) + (var_gf2 * ((var_sp_s_delta1_dn7 + var_sp_s_delta0_dn7) - ((var_delta_nd_dn7 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn7)))))), (-((var_gf2_dn8 * assign44450_e57533) + (var_gf2 * ((var_sp_s_delta1_dn8 + var_sp_s_delta0_dn8) - ((var_delta_nd_dn8 * var_sp_s_xi2) + (var_delta_nd * var_sp_s_xi2_dn8)))))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44450_e57537;
        var_sp_s_temp_dn5 = assign44450_e57537_d_n5;
        var_sp_s_temp_dn6 = assign44450_e57537_d_n6;
        var_sp_s_temp_dn7 = assign44450_e57537_d_n7;
        var_sp_s_temp_dn8 = assign44450_e57537_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign44460_e57552, assign44460_e57552_d_n5, assign44460_e57552_d_n6, assign44460_e57552_d_n7, assign44460_e57552_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44460_e57544: f64 = (var_sp_s_pc * var_sp_s_pc);
        let assign44460_e57548: f64 = (var_sp_s_qc * var_sp_s_temp);
        let assign44460_e57549: f64 = (2.0 * assign44460_e57548);
        let assign44460_e57550: f64 = (assign44460_e57544 - assign44460_e57549);
        (assign44460_e57550, (((var_sp_s_pc_dn5 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn5)) - (2.0 * ((var_sp_s_qc_dn5 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn5)))), (((var_sp_s_pc_dn6 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn6)) - (2.0 * ((var_sp_s_qc_dn6 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn6)))), (((var_sp_s_pc_dn7 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn7)) - (2.0 * ((var_sp_s_qc_dn7 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn7)))), (((var_sp_s_pc_dn8 * var_sp_s_pc) + (var_sp_s_pc * var_sp_s_pc_dn8)) - (2.0 * ((var_sp_s_qc_dn8 * var_sp_s_temp) + (var_sp_s_qc * var_sp_s_temp_dn8)))),)
    } else {
        (var_sp_s_temp, var_sp_s_temp_dn5, var_sp_s_temp_dn6, var_sp_s_temp_dn7, var_sp_s_temp_dn8,)
    }
};
        var_sp_s_temp = assign44460_e57552;
        var_sp_s_temp_dn5 = assign44460_e57552_d_n5;
        var_sp_s_temp_dn6 = assign44460_e57552_d_n6;
        var_sp_s_temp_dn7 = assign44460_e57552_d_n7;
        var_sp_s_temp_dn8 = assign44460_e57552_d_n8;
        var_sp_s_temp_rv = 0.0;

        let (assign44470_e57568, assign44470_e57568_d_n5, assign44470_e57568_d_n6, assign44470_e57568_d_n7, assign44470_e57568_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1205 == 0.0)) {
        let assign44470_e57562: f64 = (var_sp_s_temp).sqrt();
        let assign44470_e57563: f64 = (var_sp_s_pc + assign44470_e57562);
        let assign44470_e57564: f64 = (var_sp_s_qc / assign44470_e57563);
        let assign44470_e57565: f64 = (2.0 * assign44470_e57564);
        let assign44470_e57566: f64 = (var_sp_s_x0 + assign44470_e57565);
        (assign44470_e57566, (var_sp_s_x0_dn5 + (2.0 * (((var_sp_s_qc_dn5 * assign44470_e57563) - (var_sp_s_qc * (var_sp_s_pc_dn5 + (var_sp_s_temp_dn5 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (var_sp_s_x0_dn6 + (2.0 * (((var_sp_s_qc_dn6 * assign44470_e57563) - (var_sp_s_qc * (var_sp_s_pc_dn6 + (var_sp_s_temp_dn6 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (var_sp_s_x0_dn7 + (2.0 * (((var_sp_s_qc_dn7 * assign44470_e57563) - (var_sp_s_qc * (var_sp_s_pc_dn7 + (var_sp_s_temp_dn7 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))), (var_sp_s_x0_dn8 + (2.0 * (((var_sp_s_qc_dn8 * assign44470_e57563) - (var_sp_s_qc * (var_sp_s_pc_dn8 + (var_sp_s_temp_dn8 / (2.0 * assign44470_e57562))))) / (assign44470_e57563 * assign44470_e57563)))),)
    } else {
        (var_x_d, var_x_d_dn5, var_x_d_dn6, var_x_d_dn7, var_x_d_dn8,)
    }
};
        var_x_d = assign44470_e57568;
        var_x_d_dn5 = assign44470_e57568_d_n5;
        var_x_d_dn6 = assign44470_e57568_d_n6;
        var_x_d_dn7 = assign44470_e57568_d_n7;
        var_x_d_dn8 = assign44470_e57568_d_n8;
        var_x_d_rv = 0.0;

        let (assign44480_e57574, assign44480_e57574_d_n5, assign44480_e57574_d_n6, assign44480_e57574_d_n7, assign44480_e57574_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44480_e57572: f64 = (var_x_d - var_x_s);
        (assign44480_e57572, (var_x_d_dn5 - var_x_s_dn5), (var_x_d_dn6 - var_x_s_dn6), (var_x_d_dn7 - var_x_s_dn7), (var_x_d_dn8 - var_x_s_dn8),)
    } else {
        (var_x_ds, var_x_ds_dn5, var_x_ds_dn6, var_x_ds_dn7, var_x_ds_dn8,)
    }
};
        var_x_ds = assign44480_e57574;
        var_x_ds_dn5 = assign44480_e57574_d_n5;
        var_x_ds_dn6 = assign44480_e57574_d_n6;
        var_x_ds_dn7 = assign44480_e57574_d_n7;
        var_x_ds_dn8 = assign44480_e57574_d_n8;
        var_x_ds_rv = 0.0;

        let assign44490_e57577: f64 = if var_x_ds < 1e-10 { 1.0 } else { 0.0 };
        var_guard1208 = assign44490_e57577;
        var_guard1208_rv = 0.0;

        let (assign44500_e57603, assign44500_e57603_d_n5, assign44500_e57603_d_n6, assign44500_e57603_d_n7, assign44500_e57603_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44500_e57584: f64 = (var_xg - var_x_s);
        let assign44500_e57585: f64 = (2.0 * assign44500_e57584);
        let assign44500_e57589: f64 = (1.0 - var_es);
        let assign44500_e57592: f64 = (var_delta_1s * var_k_ds);
        let assign44500_e57593: f64 = (assign44500_e57589 + assign44500_e57592);
        let assign44500_e57597: f64 = (1.0 + var_xi1s);
        let assign44500_e57598: f64 = (var_delta_nd * assign44500_e57597);
        let assign44500_e57599: f64 = (assign44500_e57593 - assign44500_e57598);
        let assign44500_e57600: f64 = (var_gf2 * assign44500_e57599);
        let assign44500_e57601: f64 = (assign44500_e57585 + assign44500_e57600);
        (assign44500_e57601, ((2.0 * (var_xg_dn5 - var_x_s_dn5)) + ((var_gf2_dn5 * assign44500_e57599) + (var_gf2 * (((-var_es_dn5) + ((var_delta_1s_dn5 * var_k_ds) + (var_delta_1s * var_k_ds_dn5))) - ((var_delta_nd_dn5 * assign44500_e57597) + (var_delta_nd * var_xi1s_dn5)))))), ((2.0 * (var_xg_dn6 - var_x_s_dn6)) + ((var_gf2_dn6 * assign44500_e57599) + (var_gf2 * (((-var_es_dn6) + ((var_delta_1s_dn6 * var_k_ds) + (var_delta_1s * var_k_ds_dn6))) - ((var_delta_nd_dn6 * assign44500_e57597) + (var_delta_nd * var_xi1s_dn6)))))), ((2.0 * (var_xg_dn7 - var_x_s_dn7)) + ((var_gf2_dn7 * assign44500_e57599) + (var_gf2 * (((-var_es_dn7) + ((var_delta_1s_dn7 * var_k_ds) + (var_delta_1s * var_k_ds_dn7))) - ((var_delta_nd_dn7 * assign44500_e57597) + (var_delta_nd * var_xi1s_dn7)))))), ((2.0 * (var_xg_dn8 - var_x_s_dn8)) + ((var_gf2_dn8 * assign44500_e57599) + (var_gf2 * (((-var_es_dn8) + ((var_delta_1s_dn8 * var_k_ds) + (var_delta_1s * var_k_ds_dn8))) - ((var_delta_nd_dn8 * assign44500_e57597) + (var_delta_nd * var_xi1s_dn8)))))),)
    } else {
        (var_pc, var_pc_dn5, var_pc_dn6, var_pc_dn7, var_pc_dn8,)
    }
};
        var_pc = assign44500_e57603;
        var_pc_dn5 = assign44500_e57603_d_n5;
        var_pc_dn6 = assign44500_e57603_d_n6;
        var_pc_dn7 = assign44500_e57603_d_n7;
        var_pc_dn8 = assign44500_e57603_d_n8;
        var_pc_rv = 0.0;

        let (assign44510_e57615, assign44510_e57615_d_n5, assign44510_e57615_d_n6, assign44510_e57615_d_n7, assign44510_e57615_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44510_e57610: f64 = (1.0 - var_k_ds);
        let assign44510_e57611: f64 = (var_gf2 * assign44510_e57610);
        let assign44510_e57613: f64 = (assign44510_e57611 * var_ds);
        (assign44510_e57613, ((((var_gf2_dn5 * assign44510_e57610) + (var_gf2 * (-var_k_ds_dn5))) * var_ds) + (assign44510_e57611 * var_ds_dn5)), ((((var_gf2_dn6 * assign44510_e57610) + (var_gf2 * (-var_k_ds_dn6))) * var_ds) + (assign44510_e57611 * var_ds_dn6)), ((((var_gf2_dn7 * assign44510_e57610) + (var_gf2 * (-var_k_ds_dn7))) * var_ds) + (assign44510_e57611 * var_ds_dn7)), ((((var_gf2_dn8 * assign44510_e57610) + (var_gf2 * (-var_k_ds_dn8))) * var_ds) + (assign44510_e57611 * var_ds_dn8)),)
    } else {
        (var_qc, var_qc_dn5, var_qc_dn6, var_qc_dn7, var_qc_dn8,)
    }
};
        var_qc = assign44510_e57615;
        var_qc_dn5 = assign44510_e57615_d_n5;
        var_qc_dn6 = assign44510_e57615_d_n6;
        var_qc_dn7 = assign44510_e57615_d_n7;
        var_qc_dn8 = assign44510_e57615_d_n8;
        var_qc_rv = 0.0;

        *var_guard1206_slot = var_guard1206;
        *var_guard1206_rv_slot = var_guard1206_rv;
        *var_guard1207_slot = var_guard1207;
        *var_guard1207_rv_slot = var_guard1207_rv;
        *var_guard1208_slot = var_guard1208;
        *var_guard1208_rv_slot = var_guard1208_rv;
        *var_mutau_slot = var_mutau;
        *var_mutau_dn5_slot = var_mutau_dn5;
        *var_mutau_dn6_slot = var_mutau_dn6;
        *var_mutau_dn7_slot = var_mutau_dn7;
        *var_mutau_dn8_slot = var_mutau_dn8;
        *var_mutau_rv_slot = var_mutau_rv;
        *var_nu_slot = var_nu;
        *var_nu_dn5_slot = var_nu_dn5;
        *var_nu_dn6_slot = var_nu_dn6;
        *var_nu_dn7_slot = var_nu_dn7;
        *var_nu_dn8_slot = var_nu_dn8;
        *var_nu_rv_slot = var_nu_rv;
        *var_pc_slot = var_pc;
        *var_pc_dn5_slot = var_pc_dn5;
        *var_pc_dn6_slot = var_pc_dn6;
        *var_pc_dn7_slot = var_pc_dn7;
        *var_pc_dn8_slot = var_pc_dn8;
        *var_pc_rv_slot = var_pc_rv;
        *var_qc_slot = var_qc;
        *var_qc_dn5_slot = var_qc_dn5;
        *var_qc_dn6_slot = var_qc_dn6;
        *var_qc_dn7_slot = var_qc_dn7;
        *var_qc_dn8_slot = var_qc_dn8;
        *var_qc_rv_slot = var_qc_rv;
        *var_sp_s_b_slot = var_sp_s_b;
        *var_sp_s_b_dn5_slot = var_sp_s_b_dn5;
        *var_sp_s_b_dn6_slot = var_sp_s_b_dn6;
        *var_sp_s_b_dn7_slot = var_sp_s_b_dn7;
        *var_sp_s_b_dn8_slot = var_sp_s_b_dn8;
        *var_sp_s_b_rv_slot = var_sp_s_b_rv;
        *var_sp_s_c_slot = var_sp_s_c;
        *var_sp_s_c_dn5_slot = var_sp_s_c_dn5;
        *var_sp_s_c_dn6_slot = var_sp_s_c_dn6;
        *var_sp_s_c_dn7_slot = var_sp_s_c_dn7;
        *var_sp_s_c_dn8_slot = var_sp_s_c_dn8;
        *var_sp_s_c_rv_slot = var_sp_s_c_rv;
        *var_sp_s_delta0_slot = var_sp_s_delta0;
        *var_sp_s_delta0_dn5_slot = var_sp_s_delta0_dn5;
        *var_sp_s_delta0_dn6_slot = var_sp_s_delta0_dn6;
        *var_sp_s_delta0_dn7_slot = var_sp_s_delta0_dn7;
        *var_sp_s_delta0_dn8_slot = var_sp_s_delta0_dn8;
        *var_sp_s_delta0_rv_slot = var_sp_s_delta0_rv;
        *var_sp_s_delta1_slot = var_sp_s_delta1;
        *var_sp_s_delta1_dn5_slot = var_sp_s_delta1_dn5;
        *var_sp_s_delta1_dn6_slot = var_sp_s_delta1_dn6;
        *var_sp_s_delta1_dn7_slot = var_sp_s_delta1_dn7;
        *var_sp_s_delta1_dn8_slot = var_sp_s_delta1_dn8;
        *var_sp_s_delta1_rv_slot = var_sp_s_delta1_rv;
        *var_sp_s_pc_slot = var_sp_s_pc;
        *var_sp_s_pc_dn5_slot = var_sp_s_pc_dn5;
        *var_sp_s_pc_dn6_slot = var_sp_s_pc_dn6;
        *var_sp_s_pc_dn7_slot = var_sp_s_pc_dn7;
        *var_sp_s_pc_dn8_slot = var_sp_s_pc_dn8;
        *var_sp_s_pc_rv_slot = var_sp_s_pc_rv;
        *var_sp_s_qc_slot = var_sp_s_qc;
        *var_sp_s_qc_dn5_slot = var_sp_s_qc_dn5;
        *var_sp_s_qc_dn6_slot = var_sp_s_qc_dn6;
        *var_sp_s_qc_dn7_slot = var_sp_s_qc_dn7;
        *var_sp_s_qc_dn8_slot = var_sp_s_qc_dn8;
        *var_sp_s_qc_rv_slot = var_sp_s_qc_rv;
        *var_sp_s_tau_slot = var_sp_s_tau;
        *var_sp_s_tau_dn5_slot = var_sp_s_tau_dn5;
        *var_sp_s_tau_dn6_slot = var_sp_s_tau_dn6;
        *var_sp_s_tau_dn7_slot = var_sp_s_tau_dn7;
        *var_sp_s_tau_dn8_slot = var_sp_s_tau_dn8;
        *var_sp_s_tau_rv_slot = var_sp_s_tau_rv;
        *var_sp_s_temp_slot = var_sp_s_temp;
        *var_sp_s_temp_dn5_slot = var_sp_s_temp_dn5;
        *var_sp_s_temp_dn6_slot = var_sp_s_temp_dn6;
        *var_sp_s_temp_dn7_slot = var_sp_s_temp_dn7;
        *var_sp_s_temp_dn8_slot = var_sp_s_temp_dn8;
        *var_sp_s_temp_rv_slot = var_sp_s_temp_rv;
        *var_sp_s_x0_slot = var_sp_s_x0;
        *var_sp_s_x0_dn5_slot = var_sp_s_x0_dn5;
        *var_sp_s_x0_dn6_slot = var_sp_s_x0_dn6;
        *var_sp_s_x0_dn7_slot = var_sp_s_x0_dn7;
        *var_sp_s_x0_dn8_slot = var_sp_s_x0_dn8;
        *var_sp_s_x0_rv_slot = var_sp_s_x0_rv;
        *var_sp_s_xi0_slot = var_sp_s_xi0;
        *var_sp_s_xi0_dn5_slot = var_sp_s_xi0_dn5;
        *var_sp_s_xi0_dn6_slot = var_sp_s_xi0_dn6;
        *var_sp_s_xi0_dn7_slot = var_sp_s_xi0_dn7;
        *var_sp_s_xi0_dn8_slot = var_sp_s_xi0_dn8;
        *var_sp_s_xi0_rv_slot = var_sp_s_xi0_rv;
        *var_sp_s_xi1_slot = var_sp_s_xi1;
        *var_sp_s_xi1_dn5_slot = var_sp_s_xi1_dn5;
        *var_sp_s_xi1_dn6_slot = var_sp_s_xi1_dn6;
        *var_sp_s_xi1_dn7_slot = var_sp_s_xi1_dn7;
        *var_sp_s_xi1_dn8_slot = var_sp_s_xi1_dn8;
        *var_sp_s_xi1_rv_slot = var_sp_s_xi1_rv;
        *var_sp_s_xi2_slot = var_sp_s_xi2;
        *var_sp_s_xi2_dn5_slot = var_sp_s_xi2_dn5;
        *var_sp_s_xi2_dn6_slot = var_sp_s_xi2_dn6;
        *var_sp_s_xi2_dn7_slot = var_sp_s_xi2_dn7;
        *var_sp_s_xi2_dn8_slot = var_sp_s_xi2_dn8;
        *var_sp_s_xi2_rv_slot = var_sp_s_xi2_rv;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_d_rv_slot = var_x_d_rv;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn5_slot = var_x_ds_dn5;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_ds_rv_slot = var_x_ds_rv;
    }

    pub(super) fn stamp_reactive_block_34(
        var_delta_1s: f64,
        var_delta_1s_dn5: f64,
        var_delta_1s_dn6: f64,
        var_delta_1s_dn7: f64,
        var_delta_1s_dn8: f64,
        var_delta_nd: f64,
        var_delta_nd_dn5: f64,
        var_delta_nd_dn6: f64,
        var_delta_nd_dn7: f64,
        var_delta_nd_dn8: f64,
        var_ds: f64,
        var_ds_dn5: f64,
        var_ds_dn6: f64,
        var_ds_dn7: f64,
        var_ds_dn8: f64,
        var_es: f64,
        var_es_dn5: f64,
        var_es_dn6: f64,
        var_es_dn7: f64,
        var_es_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1197: f64,
        var_guard1208: f64,
        var_k_ds: f64,
        var_k_ds_dn5: f64,
        var_k_ds_dn6: f64,
        var_k_ds_dn7: f64,
        var_k_ds_dn8: f64,
        var_pc: f64,
        var_pc_dn5: f64,
        var_pc_dn6: f64,
        var_pc_dn7: f64,
        var_pc_dn8: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_qc: f64,
        var_qc_dn5: f64,
        var_qc_dn6: f64,
        var_qc_dn7: f64,
        var_qc_dn8: f64,
        var_x_s: f64,
        var_x_s_dn5: f64,
        var_x_s_dn6: f64,
        var_x_s_dn7: f64,
        var_x_s_dn8: f64,
        var_xi2s: f64,
        var_xi2s_dn5: f64,
        var_xi2s_dn6: f64,
        var_xi2s_dn7: f64,
        var_xi2s_dn8: f64,
        var_xn_d: f64,
        var_xn_d_dn5: f64,
        var_xn_d_dn6: f64,
        var_xn_d_dn7: f64,
        var_xn_d_dn8: f64,
        var_d_bar_slot: &mut f64,
        var_d_bar_dn5_slot: &mut f64,
        var_d_bar_dn6_slot: &mut f64,
        var_d_bar_dn7_slot: &mut f64,
        var_d_bar_dn8_slot: &mut f64,
        var_d_bar_rv_slot: &mut f64,
        var_dd_slot: &mut f64,
        var_dd_dn5_slot: &mut f64,
        var_dd_dn6_slot: &mut f64,
        var_dd_dn7_slot: &mut f64,
        var_dd_dn8_slot: &mut f64,
        var_dd_rv_slot: &mut f64,
        var_dps_slot: &mut f64,
        var_dps_dn5_slot: &mut f64,
        var_dps_dn6_slot: &mut f64,
        var_dps_dn7_slot: &mut f64,
        var_dps_dn8_slot: &mut f64,
        var_dps_rv_slot: &mut f64,
        var_ed_slot: &mut f64,
        var_ed_dn5_slot: &mut f64,
        var_ed_dn6_slot: &mut f64,
        var_ed_dn7_slot: &mut f64,
        var_ed_dn8_slot: &mut f64,
        var_ed_rv_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_rv_slot: &mut f64,
        var_guard1209_slot: &mut f64,
        var_guard1209_rv_slot: &mut f64,
        var_guard1210_slot: &mut f64,
        var_guard1210_rv_slot: &mut f64,
        var_guard1211_slot: &mut f64,
        var_guard1211_rv_slot: &mut f64,
        var_guard1212_slot: &mut f64,
        var_guard1212_rv_slot: &mut f64,
        var_pd_slot: &mut f64,
        var_pd_dn5_slot: &mut f64,
        var_pd_dn6_slot: &mut f64,
        var_pd_dn7_slot: &mut f64,
        var_pd_dn8_slot: &mut f64,
        var_pd_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn5_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_sqd_slot: &mut f64,
        var_sqd_dn5_slot: &mut f64,
        var_sqd_dn6_slot: &mut f64,
        var_sqd_dn7_slot: &mut f64,
        var_sqd_dn8_slot: &mut f64,
        var_sqd_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_x_d_slot: &mut f64,
        var_x_d_dn5_slot: &mut f64,
        var_x_d_dn6_slot: &mut f64,
        var_x_d_dn7_slot: &mut f64,
        var_x_d_dn8_slot: &mut f64,
        var_x_d_rv_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn5_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_ds_rv_slot: &mut f64,
        var_x_m_slot: &mut f64,
        var_x_m_dn5_slot: &mut f64,
        var_x_m_dn6_slot: &mut f64,
        var_x_m_dn7_slot: &mut f64,
        var_x_m_dn8_slot: &mut f64,
        var_x_m_rv_slot: &mut f64,
        var_xi0d_slot: &mut f64,
        var_xi0d_dn5_slot: &mut f64,
        var_xi0d_dn6_slot: &mut f64,
        var_xi0d_dn7_slot: &mut f64,
        var_xi0d_dn8_slot: &mut f64,
        var_xi0d_rv_slot: &mut f64,
    ) {
        let mut var_d_bar: f64 = *var_d_bar_slot;
        let mut var_d_bar_dn5: f64 = *var_d_bar_dn5_slot;
        let mut var_d_bar_dn6: f64 = *var_d_bar_dn6_slot;
        let mut var_d_bar_dn7: f64 = *var_d_bar_dn7_slot;
        let mut var_d_bar_dn8: f64 = *var_d_bar_dn8_slot;
        let mut var_d_bar_rv: f64 = *var_d_bar_rv_slot;
        let mut var_dd: f64 = *var_dd_slot;
        let mut var_dd_dn5: f64 = *var_dd_dn5_slot;
        let mut var_dd_dn6: f64 = *var_dd_dn6_slot;
        let mut var_dd_dn7: f64 = *var_dd_dn7_slot;
        let mut var_dd_dn8: f64 = *var_dd_dn8_slot;
        let mut var_dd_rv: f64 = *var_dd_rv_slot;
        let mut var_dps: f64 = *var_dps_slot;
        let mut var_dps_dn5: f64 = *var_dps_dn5_slot;
        let mut var_dps_dn6: f64 = *var_dps_dn6_slot;
        let mut var_dps_dn7: f64 = *var_dps_dn7_slot;
        let mut var_dps_dn8: f64 = *var_dps_dn8_slot;
        let mut var_dps_rv: f64 = *var_dps_rv_slot;
        let mut var_ed: f64 = *var_ed_slot;
        let mut var_ed_dn5: f64 = *var_ed_dn5_slot;
        let mut var_ed_dn6: f64 = *var_ed_dn6_slot;
        let mut var_ed_dn7: f64 = *var_ed_dn7_slot;
        let mut var_ed_dn8: f64 = *var_ed_dn8_slot;
        let mut var_ed_rv: f64 = *var_ed_rv_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_rv: f64 = *var_em_rv_slot;
        let mut var_guard1209: f64 = *var_guard1209_slot;
        let mut var_guard1209_rv: f64 = *var_guard1209_rv_slot;
        let mut var_guard1210: f64 = *var_guard1210_slot;
        let mut var_guard1210_rv: f64 = *var_guard1210_rv_slot;
        let mut var_guard1211: f64 = *var_guard1211_slot;
        let mut var_guard1211_rv: f64 = *var_guard1211_rv_slot;
        let mut var_guard1212: f64 = *var_guard1212_slot;
        let mut var_guard1212_rv: f64 = *var_guard1212_rv_slot;
        let mut var_pd: f64 = *var_pd_slot;
        let mut var_pd_dn5: f64 = *var_pd_dn5_slot;
        let mut var_pd_dn6: f64 = *var_pd_dn6_slot;
        let mut var_pd_dn7: f64 = *var_pd_dn7_slot;
        let mut var_pd_dn8: f64 = *var_pd_dn8_slot;
        let mut var_pd_rv: f64 = *var_pd_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn5: f64 = *var_qbd_dn5_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_sqd: f64 = *var_sqd_slot;
        let mut var_sqd_dn5: f64 = *var_sqd_dn5_slot;
        let mut var_sqd_dn6: f64 = *var_sqd_dn6_slot;
        let mut var_sqd_dn7: f64 = *var_sqd_dn7_slot;
        let mut var_sqd_dn8: f64 = *var_sqd_dn8_slot;
        let mut var_sqd_rv: f64 = *var_sqd_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_x_d: f64 = *var_x_d_slot;
        let mut var_x_d_dn5: f64 = *var_x_d_dn5_slot;
        let mut var_x_d_dn6: f64 = *var_x_d_dn6_slot;
        let mut var_x_d_dn7: f64 = *var_x_d_dn7_slot;
        let mut var_x_d_dn8: f64 = *var_x_d_dn8_slot;
        let mut var_x_d_rv: f64 = *var_x_d_rv_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn5: f64 = *var_x_ds_dn5_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_ds_rv: f64 = *var_x_ds_rv_slot;
        let mut var_x_m: f64 = *var_x_m_slot;
        let mut var_x_m_dn5: f64 = *var_x_m_dn5_slot;
        let mut var_x_m_dn6: f64 = *var_x_m_dn6_slot;
        let mut var_x_m_dn7: f64 = *var_x_m_dn7_slot;
        let mut var_x_m_dn8: f64 = *var_x_m_dn8_slot;
        let mut var_x_m_rv: f64 = *var_x_m_rv_slot;
        let mut var_xi0d: f64 = *var_xi0d_slot;
        let mut var_xi0d_dn5: f64 = *var_xi0d_dn5_slot;
        let mut var_xi0d_dn6: f64 = *var_xi0d_dn6_slot;
        let mut var_xi0d_dn7: f64 = *var_xi0d_dn7_slot;
        let mut var_xi0d_dn8: f64 = *var_xi0d_dn8_slot;
        let mut var_xi0d_rv: f64 = *var_xi0d_rv_slot;

        let (assign44520_e57633, assign44520_e57633_d_n5, assign44520_e57633_d_n6, assign44520_e57633_d_n7, assign44520_e57633_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44520_e57624: f64 = (var_delta_1s * var_k_ds);
        let assign44520_e57625: f64 = (var_es + assign44520_e57624);
        let assign44520_e57628: f64 = (var_delta_nd * var_xi2s);
        let assign44520_e57629: f64 = (assign44520_e57625 - assign44520_e57628);
        let assign44520_e57630: f64 = (var_gf2 * assign44520_e57629);
        let assign44520_e57631: f64 = (2.0 - assign44520_e57630);
        (assign44520_e57631, (-((var_gf2_dn5 * assign44520_e57629) + (var_gf2 * ((var_es_dn5 + ((var_delta_1s_dn5 * var_k_ds) + (var_delta_1s * var_k_ds_dn5))) - ((var_delta_nd_dn5 * var_xi2s) + (var_delta_nd * var_xi2s_dn5)))))), (-((var_gf2_dn6 * assign44520_e57629) + (var_gf2 * ((var_es_dn6 + ((var_delta_1s_dn6 * var_k_ds) + (var_delta_1s * var_k_ds_dn6))) - ((var_delta_nd_dn6 * var_xi2s) + (var_delta_nd * var_xi2s_dn6)))))), (-((var_gf2_dn7 * assign44520_e57629) + (var_gf2 * ((var_es_dn7 + ((var_delta_1s_dn7 * var_k_ds) + (var_delta_1s * var_k_ds_dn7))) - ((var_delta_nd_dn7 * var_xi2s) + (var_delta_nd * var_xi2s_dn7)))))), (-((var_gf2_dn8 * assign44520_e57629) + (var_gf2 * ((var_es_dn8 + ((var_delta_1s_dn8 * var_k_ds) + (var_delta_1s * var_k_ds_dn8))) - ((var_delta_nd_dn8 * var_xi2s) + (var_delta_nd * var_xi2s_dn8)))))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44520_e57633;
        var_temp__blk936_dn5 = assign44520_e57633_d_n5;
        var_temp__blk936_dn6 = assign44520_e57633_d_n6;
        var_temp__blk936_dn7 = assign44520_e57633_d_n7;
        var_temp__blk936_dn8 = assign44520_e57633_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44530_e57647, assign44530_e57647_d_n5, assign44530_e57647_d_n6, assign44530_e57647_d_n7, assign44530_e57647_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44530_e57639: f64 = (var_pc * var_pc);
        let assign44530_e57643: f64 = (var_temp__blk936 * var_qc);
        let assign44530_e57644: f64 = (2.0 * assign44530_e57643);
        let assign44530_e57645: f64 = (assign44530_e57639 - assign44530_e57644);
        (assign44530_e57645, (((var_pc_dn5 * var_pc) + (var_pc * var_pc_dn5)) - (2.0 * ((var_temp__blk936_dn5 * var_qc) + (var_temp__blk936 * var_qc_dn5)))), (((var_pc_dn6 * var_pc) + (var_pc * var_pc_dn6)) - (2.0 * ((var_temp__blk936_dn6 * var_qc) + (var_temp__blk936 * var_qc_dn6)))), (((var_pc_dn7 * var_pc) + (var_pc * var_pc_dn7)) - (2.0 * ((var_temp__blk936_dn7 * var_qc) + (var_temp__blk936 * var_qc_dn7)))), (((var_pc_dn8 * var_pc) + (var_pc * var_pc_dn8)) - (2.0 * ((var_temp__blk936_dn8 * var_qc) + (var_temp__blk936 * var_qc_dn8)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44530_e57647;
        var_temp__blk936_dn5 = assign44530_e57647_d_n5;
        var_temp__blk936_dn6 = assign44530_e57647_d_n6;
        var_temp__blk936_dn7 = assign44530_e57647_d_n7;
        var_temp__blk936_dn8 = assign44530_e57647_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44540_e57660, assign44540_e57660_d_n5, assign44540_e57660_d_n6, assign44540_e57660_d_n7, assign44540_e57660_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44540_e57655: f64 = (var_temp__blk936).sqrt();
        let assign44540_e57656: f64 = (var_pc + assign44540_e57655);
        let assign44540_e57657: f64 = (var_qc / assign44540_e57656);
        let assign44540_e57658: f64 = (2.0 * assign44540_e57657);
        (assign44540_e57658, (2.0 * (((var_qc_dn5 * assign44540_e57656) - (var_qc * (var_pc_dn5 + (var_temp__blk936_dn5 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((var_qc_dn6 * assign44540_e57656) - (var_qc * (var_pc_dn6 + (var_temp__blk936_dn6 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((var_qc_dn7 * assign44540_e57656) - (var_qc * (var_pc_dn7 + (var_temp__blk936_dn7 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))), (2.0 * (((var_qc_dn8 * assign44540_e57656) - (var_qc * (var_pc_dn8 + (var_temp__blk936_dn8 / (2.0 * assign44540_e57655))))) / (assign44540_e57656 * assign44540_e57656))),)
    } else {
        (var_x_ds, var_x_ds_dn5, var_x_ds_dn6, var_x_ds_dn7, var_x_ds_dn8,)
    }
};
        var_x_ds = assign44540_e57660;
        var_x_ds_dn5 = assign44540_e57660_d_n5;
        var_x_ds_dn6 = assign44540_e57660_d_n6;
        var_x_ds_dn7 = assign44540_e57660_d_n7;
        var_x_ds_dn8 = assign44540_e57660_d_n8;
        var_x_ds_rv = 0.0;

        let (assign44550_e57668, assign44550_e57668_d_n5, assign44550_e57668_d_n6, assign44550_e57668_d_n7, assign44550_e57668_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1208 != 0.0)) {
        let assign44550_e57666: f64 = (var_x_s + var_x_ds);
        (assign44550_e57666, (var_x_s_dn5 + var_x_ds_dn5), (var_x_s_dn6 + var_x_ds_dn6), (var_x_s_dn7 + var_x_ds_dn7), (var_x_s_dn8 + var_x_ds_dn8),)
    } else {
        (var_x_d, var_x_d_dn5, var_x_d_dn6, var_x_d_dn7, var_x_d_dn8,)
    }
};
        var_x_d = assign44550_e57668;
        var_x_d_dn5 = assign44550_e57668_d_n5;
        var_x_d_dn6 = assign44550_e57668_d_n6;
        var_x_d_dn7 = assign44550_e57668_d_n7;
        var_x_d_dn8 = assign44550_e57668_d_n8;
        var_x_d_rv = 0.0;

        let (assign44560_e57674, assign44560_e57674_d_n5, assign44560_e57674_d_n6, assign44560_e57674_d_n7, assign44560_e57674_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44560_e57672: f64 = (var_x_ds * var_phit1);
        (assign44560_e57672, ((var_x_ds_dn5 * var_phit1) + (var_x_ds * var_phit1_dn5)), ((var_x_ds_dn6 * var_phit1) + (var_x_ds * var_phit1_dn6)), ((var_x_ds_dn7 * var_phit1) + (var_x_ds * var_phit1_dn7)), ((var_x_ds_dn8 * var_phit1) + (var_x_ds * var_phit1_dn8)),)
    } else {
        (var_dps, var_dps_dn5, var_dps_dn6, var_dps_dn7, var_dps_dn8,)
    }
};
        var_dps = assign44560_e57674;
        var_dps_dn5 = assign44560_e57674_d_n5;
        var_dps_dn6 = assign44560_e57674_d_n6;
        var_dps_dn7 = assign44560_e57674_d_n7;
        var_dps_dn8 = assign44560_e57674_d_n8;
        var_dps_rv = 0.0;

        let (assign44570_e57686, assign44570_e57686_d_n5, assign44570_e57686_d_n6, assign44570_e57686_d_n7, assign44570_e57686_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44570_e57678: f64 = (var_x_d * var_x_d);
        let assign44570_e57682: f64 = (var_x_d * var_x_d);
        let assign44570_e57683: f64 = (2.0 + assign44570_e57682);
        let assign44570_e57684: f64 = (assign44570_e57678 / assign44570_e57683);
        (assign44570_e57684, (((((var_x_d_dn5 * var_x_d) + (var_x_d * var_x_d_dn5)) * assign44570_e57683) - (assign44570_e57678 * ((var_x_d_dn5 * var_x_d) + (var_x_d * var_x_d_dn5)))) / (assign44570_e57683 * assign44570_e57683)), (((((var_x_d_dn6 * var_x_d) + (var_x_d * var_x_d_dn6)) * assign44570_e57683) - (assign44570_e57678 * ((var_x_d_dn6 * var_x_d) + (var_x_d * var_x_d_dn6)))) / (assign44570_e57683 * assign44570_e57683)), (((((var_x_d_dn7 * var_x_d) + (var_x_d * var_x_d_dn7)) * assign44570_e57683) - (assign44570_e57678 * ((var_x_d_dn7 * var_x_d) + (var_x_d * var_x_d_dn7)))) / (assign44570_e57683 * assign44570_e57683)), (((((var_x_d_dn8 * var_x_d) + (var_x_d * var_x_d_dn8)) * assign44570_e57683) - (assign44570_e57678 * ((var_x_d_dn8 * var_x_d) + (var_x_d * var_x_d_dn8)))) / (assign44570_e57683 * assign44570_e57683)),)
    } else {
        (var_xi0d, var_xi0d_dn5, var_xi0d_dn6, var_xi0d_dn7, var_xi0d_dn8,)
    }
};
        var_xi0d = assign44570_e57686;
        var_xi0d_dn5 = assign44570_e57686_d_n5;
        var_xi0d_dn6 = assign44570_e57686_d_n6;
        var_xi0d_dn7 = assign44570_e57686_d_n7;
        var_xi0d_dn8 = assign44570_e57686_d_n8;
        var_xi0d_rv = 0.0;

        let assign44580_e57689: f64 = if var_x_d < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1209 = assign44580_e57689;
        var_guard1209_rv = 0.0;

        let (assign44590_e57697, assign44590_e57697_d_n5, assign44590_e57697_d_n6, assign44590_e57697_d_n7, assign44590_e57697_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) {
        let assign44590_e57694: f64 = (-var_x_d);
        let assign44590_e57695: f64 = (assign44590_e57694).exp();
        (assign44590_e57695, (assign44590_e57695 * (-var_x_d_dn5)), (assign44590_e57695 * (-var_x_d_dn6)), (assign44590_e57695 * (-var_x_d_dn7)), (assign44590_e57695 * (-var_x_d_dn8)),)
    } else {
        (var_ed, var_ed_dn5, var_ed_dn6, var_ed_dn7, var_ed_dn8,)
    }
};
        var_ed = assign44590_e57697;
        var_ed_dn5 = assign44590_e57697_d_n5;
        var_ed_dn6 = assign44590_e57697_d_n6;
        var_ed_dn7 = assign44590_e57697_d_n7;
        var_ed_dn8 = assign44590_e57697_d_n8;
        var_ed_rv = 0.0;

        let assign44600_e57700: f64 = if var_x_d < 1e-5 { 1.0 } else { 0.0 };
        var_guard1210 = assign44600_e57700;
        var_guard1210_rv = 0.0;

        let (assign44610_e57724, assign44610_e57724_d_n5, assign44610_e57724_d_n6, assign44610_e57724_d_n7, assign44610_e57724_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 != 0.0)) {
        let assign44610_e57709: f64 = (var_x_d * var_x_d);
        let assign44610_e57716: f64 = (0.25 * var_x_d);
        let assign44610_e57717: f64 = (1.0 - assign44610_e57716);
        let assign44610_e57718: f64 = (var_x_d * assign44610_e57717);
        let assign44610_e57719: f64 = (0.3333333333333333 * assign44610_e57718);
        let assign44610_e57720: f64 = (1.0 - assign44610_e57719);
        let assign44610_e57721: f64 = (assign44610_e57709 * assign44610_e57720);
        let assign44610_e57722: f64 = (0.5 * assign44610_e57721);
        (assign44610_e57722, (0.5 * ((((var_x_d_dn5 * var_x_d) + (var_x_d * var_x_d_dn5)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((var_x_d_dn5 * assign44610_e57717) + (var_x_d * (-(0.25 * var_x_d_dn5))))))))), (0.5 * ((((var_x_d_dn6 * var_x_d) + (var_x_d * var_x_d_dn6)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((var_x_d_dn6 * assign44610_e57717) + (var_x_d * (-(0.25 * var_x_d_dn6))))))))), (0.5 * ((((var_x_d_dn7 * var_x_d) + (var_x_d * var_x_d_dn7)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((var_x_d_dn7 * assign44610_e57717) + (var_x_d * (-(0.25 * var_x_d_dn7))))))))), (0.5 * ((((var_x_d_dn8 * var_x_d) + (var_x_d * var_x_d_dn8)) * assign44610_e57720) + (assign44610_e57709 * (-(0.3333333333333333 * ((var_x_d_dn8 * assign44610_e57717) + (var_x_d * (-(0.25 * var_x_d_dn8))))))))),)
    } else {
        (var_pd, var_pd_dn5, var_pd_dn6, var_pd_dn7, var_pd_dn8,)
    }
};
        var_pd = assign44610_e57724;
        var_pd_dn5 = assign44610_e57724_d_n5;
        var_pd_dn6 = assign44610_e57724_d_n6;
        var_pd_dn7 = assign44610_e57724_d_n7;
        var_pd_dn8 = assign44610_e57724_d_n8;
        var_pd_rv = 0.0;

        let (assign44620_e57743, assign44620_e57743_d_n5, assign44620_e57743_d_n6, assign44620_e57743_d_n7, assign44620_e57743_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 != 0.0)) {
        let assign44620_e57736: f64 = (0.25 * var_x_d);
        let assign44620_e57737: f64 = (1.0 - assign44620_e57736);
        let assign44620_e57738: f64 = (var_x_d * assign44620_e57737);
        let assign44620_e57739: f64 = (0.3333333333333333 * assign44620_e57738);
        let assign44620_e57740: f64 = (1.0 - assign44620_e57739);
        let assign44620_e57741: f64 = (assign44620_e57740).sqrt();
        (assign44620_e57741, ((-(0.3333333333333333 * ((var_x_d_dn5 * assign44620_e57737) + (var_x_d * (-(0.25 * var_x_d_dn5)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((var_x_d_dn6 * assign44620_e57737) + (var_x_d * (-(0.25 * var_x_d_dn6)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((var_x_d_dn7 * assign44620_e57737) + (var_x_d * (-(0.25 * var_x_d_dn7)))))) / (2.0 * assign44620_e57741)), ((-(0.3333333333333333 * ((var_x_d_dn8 * assign44620_e57737) + (var_x_d * (-(0.25 * var_x_d_dn8)))))) / (2.0 * assign44620_e57741)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44620_e57743;
        var_temp__blk936_dn5 = assign44620_e57743_d_n5;
        var_temp__blk936_dn6 = assign44620_e57743_d_n6;
        var_temp__blk936_dn7 = assign44620_e57743_d_n7;
        var_temp__blk936_dn8 = assign44620_e57743_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44630_e57755, assign44630_e57755_d_n5, assign44630_e57755_d_n6, assign44630_e57755_d_n7, assign44630_e57755_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 != 0.0)) {
        let assign44630_e57752: f64 = (var_x_d * var_temp__blk936);
        let assign44630_e57753: f64 = (0.7071067811865475 * assign44630_e57752);
        (assign44630_e57753, (0.7071067811865475 * ((var_x_d_dn5 * var_temp__blk936) + (var_x_d * var_temp__blk936_dn5))), (0.7071067811865475 * ((var_x_d_dn6 * var_temp__blk936) + (var_x_d * var_temp__blk936_dn6))), (0.7071067811865475 * ((var_x_d_dn7 * var_temp__blk936) + (var_x_d * var_temp__blk936_dn7))), (0.7071067811865475 * ((var_x_d_dn8 * var_temp__blk936) + (var_x_d * var_temp__blk936_dn8))),)
    } else {
        (var_sqd, var_sqd_dn5, var_sqd_dn6, var_sqd_dn7, var_sqd_dn8,)
    }
};
        var_sqd = assign44630_e57755;
        var_sqd_dn5 = assign44630_e57755_d_n5;
        var_sqd_dn6 = assign44630_e57755_d_n6;
        var_sqd_dn7 = assign44630_e57755_d_n7;
        var_sqd_dn8 = assign44630_e57755_d_n8;
        var_sqd_rv = 0.0;

        let (assign44640_e57777, assign44640_e57777_d_n5, assign44640_e57777_d_n6, assign44640_e57777_d_n7, assign44640_e57777_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 != 0.0)) {
        let assign44640_e57763: f64 = (0.16666666666666666 * var_delta_nd);
        let assign44640_e57765: f64 = (assign44640_e57763 * var_x_d);
        let assign44640_e57767: f64 = (assign44640_e57765 * var_x_d);
        let assign44640_e57769: f64 = (assign44640_e57767 * var_x_d);
        let assign44640_e57773: f64 = (1.75 * var_x_d);
        let assign44640_e57774: f64 = (1.0 + assign44640_e57773);
        let assign44640_e57775: f64 = (assign44640_e57769 * assign44640_e57774);
        (assign44640_e57775, (((((((((0.16666666666666666 * var_delta_nd_dn5) * var_x_d) + (assign44640_e57763 * var_x_d_dn5)) * var_x_d) + (assign44640_e57765 * var_x_d_dn5)) * var_x_d) + (assign44640_e57767 * var_x_d_dn5)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * var_x_d_dn5))), (((((((((0.16666666666666666 * var_delta_nd_dn6) * var_x_d) + (assign44640_e57763 * var_x_d_dn6)) * var_x_d) + (assign44640_e57765 * var_x_d_dn6)) * var_x_d) + (assign44640_e57767 * var_x_d_dn6)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * var_x_d_dn6))), (((((((((0.16666666666666666 * var_delta_nd_dn7) * var_x_d) + (assign44640_e57763 * var_x_d_dn7)) * var_x_d) + (assign44640_e57765 * var_x_d_dn7)) * var_x_d) + (assign44640_e57767 * var_x_d_dn7)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * var_x_d_dn7))), (((((((((0.16666666666666666 * var_delta_nd_dn8) * var_x_d) + (assign44640_e57763 * var_x_d_dn8)) * var_x_d) + (assign44640_e57765 * var_x_d_dn8)) * var_x_d) + (assign44640_e57767 * var_x_d_dn8)) * assign44640_e57774) + (assign44640_e57769 * (1.75 * var_x_d_dn8))),)
    } else {
        (var_dd, var_dd_dn5, var_dd_dn6, var_dd_dn7, var_dd_dn8,)
    }
};
        var_dd = assign44640_e57777;
        var_dd_dn5 = assign44640_e57777_d_n5;
        var_dd_dn6 = assign44640_e57777_d_n6;
        var_dd_dn7 = assign44640_e57777_d_n7;
        var_dd_dn8 = assign44640_e57777_d_n8;
        var_dd_rv = 0.0;

        let (assign44650_e57790, assign44650_e57790_d_n5, assign44650_e57790_d_n6, assign44650_e57790_d_n7, assign44650_e57790_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 == 0.0)) {
        let assign44650_e57786: f64 = (var_x_d - 1.0);
        let assign44650_e57788: f64 = (assign44650_e57786 + var_ed);
        (assign44650_e57788, (var_x_d_dn5 + var_ed_dn5), (var_x_d_dn6 + var_ed_dn6), (var_x_d_dn7 + var_ed_dn7), (var_x_d_dn8 + var_ed_dn8),)
    } else {
        (var_pd, var_pd_dn5, var_pd_dn6, var_pd_dn7, var_pd_dn8,)
    }
};
        var_pd = assign44650_e57790;
        var_pd_dn5 = assign44650_e57790_d_n5;
        var_pd_dn6 = assign44650_e57790_d_n6;
        var_pd_dn7 = assign44650_e57790_d_n7;
        var_pd_dn8 = assign44650_e57790_d_n8;
        var_pd_rv = 0.0;

        let (assign44660_e57800, assign44660_e57800_d_n5, assign44660_e57800_d_n6, assign44660_e57800_d_n7, assign44660_e57800_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 == 0.0)) {
        let assign44660_e57798: f64 = (var_pd).sqrt();
        (assign44660_e57798, (var_pd_dn5 / (2.0 * assign44660_e57798)), (var_pd_dn6 / (2.0 * assign44660_e57798)), (var_pd_dn7 / (2.0 * assign44660_e57798)), (var_pd_dn8 / (2.0 * assign44660_e57798)),)
    } else {
        (var_sqd, var_sqd_dn5, var_sqd_dn6, var_sqd_dn7, var_sqd_dn8,)
    }
};
        var_sqd = assign44660_e57800;
        var_sqd_dn5 = assign44660_e57800_d_n5;
        var_sqd_dn6 = assign44660_e57800_d_n6;
        var_sqd_dn7 = assign44660_e57800_d_n7;
        var_sqd_dn8 = assign44660_e57800_d_n8;
        var_sqd_rv = 0.0;

        let (assign44670_e57819, assign44670_e57819_d_n5, assign44670_e57819_d_n6, assign44670_e57819_d_n7, assign44670_e57819_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 != 0.0)) && (var_guard1210 == 0.0)) {
        let assign44670_e57810: f64 = (1.0 / var_ed);
        let assign44670_e57812: f64 = (assign44670_e57810 - var_x_d);
        let assign44670_e57814: f64 = (assign44670_e57812 - 1.0);
        let assign44670_e57816: f64 = (assign44670_e57814 - var_xi0d);
        let assign44670_e57817: f64 = (var_delta_nd * assign44670_e57816);
        (assign44670_e57817, ((var_delta_nd_dn5 * assign44670_e57816) + (var_delta_nd * (((-(var_ed_dn5 / (var_ed * var_ed))) - var_x_d_dn5) - var_xi0d_dn5))), ((var_delta_nd_dn6 * assign44670_e57816) + (var_delta_nd * (((-(var_ed_dn6 / (var_ed * var_ed))) - var_x_d_dn6) - var_xi0d_dn6))), ((var_delta_nd_dn7 * assign44670_e57816) + (var_delta_nd * (((-(var_ed_dn7 / (var_ed * var_ed))) - var_x_d_dn7) - var_xi0d_dn7))), ((var_delta_nd_dn8 * assign44670_e57816) + (var_delta_nd * (((-(var_ed_dn8 / (var_ed * var_ed))) - var_x_d_dn8) - var_xi0d_dn8))),)
    } else {
        (var_dd, var_dd_dn5, var_dd_dn6, var_dd_dn7, var_dd_dn8,)
    }
};
        var_dd = assign44670_e57819;
        var_dd_dn5 = assign44670_e57819_d_n5;
        var_dd_dn6 = assign44670_e57819_d_n6;
        var_dd_dn7 = assign44670_e57819_d_n7;
        var_dd_dn8 = assign44670_e57819_d_n8;
        var_dd_rv = 0.0;

        let assign44680_e57823: f64 = (var_xn_d - 230.25850929940458);
        let assign44680_e57824: f64 = if var_x_d > assign44680_e57823 { 1.0 } else { 0.0 };
        var_guard1211 = assign44680_e57824;
        var_guard1211_rv = 0.0;

        let (assign44690_e57836, assign44690_e57836_d_n5, assign44690_e57836_d_n6, assign44690_e57836_d_n7, assign44690_e57836_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 != 0.0)) {
        let assign44690_e57833: f64 = (var_x_d - var_xn_d);
        let assign44690_e57834: f64 = (assign44690_e57833).exp();
        (assign44690_e57834, (assign44690_e57834 * (var_x_d_dn5 - var_xn_d_dn5)), (assign44690_e57834 * (var_x_d_dn6 - var_xn_d_dn6)), (assign44690_e57834 * (var_x_d_dn7 - var_xn_d_dn7)), (assign44690_e57834 * (var_x_d_dn8 - var_xn_d_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44690_e57836;
        var_temp__blk936_dn5 = assign44690_e57836_d_n5;
        var_temp__blk936_dn6 = assign44690_e57836_d_n6;
        var_temp__blk936_dn7 = assign44690_e57836_d_n7;
        var_temp__blk936_dn8 = assign44690_e57836_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44700_e57847, assign44700_e57847_d_n5, assign44700_e57847_d_n6, assign44700_e57847_d_n7, assign44700_e57847_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 != 0.0)) {
        let assign44700_e57845: f64 = (var_delta_nd / var_temp__blk936);
        (assign44700_e57845, (((var_delta_nd_dn5 * var_temp__blk936) - (var_delta_nd * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936)), (((var_delta_nd_dn6 * var_temp__blk936) - (var_delta_nd * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936)), (((var_delta_nd_dn7 * var_temp__blk936) - (var_delta_nd * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936)), (((var_delta_nd_dn8 * var_temp__blk936) - (var_delta_nd * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936)),)
    } else {
        (var_ed, var_ed_dn5, var_ed_dn6, var_ed_dn7, var_ed_dn8,)
    }
};
        var_ed = assign44700_e57847;
        var_ed_dn5 = assign44700_e57847_d_n5;
        var_ed_dn6 = assign44700_e57847_d_n6;
        var_ed_dn7 = assign44700_e57847_d_n7;
        var_ed_dn8 = assign44700_e57847_d_n8;
        var_ed_rv = 0.0;

        let (assign44710_e57864, assign44710_e57864_d_n5, assign44710_e57864_d_n6, assign44710_e57864_d_n7, assign44710_e57864_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 != 0.0)) {
        let assign44710_e57858: f64 = (var_x_d + 1.0);
        let assign44710_e57860: f64 = (assign44710_e57858 + var_xi0d);
        let assign44710_e57861: f64 = (var_delta_nd * assign44710_e57860);
        let assign44710_e57862: f64 = (var_temp__blk936 - assign44710_e57861);
        (assign44710_e57862, (var_temp__blk936_dn5 - ((var_delta_nd_dn5 * assign44710_e57860) + (var_delta_nd * (var_x_d_dn5 + var_xi0d_dn5)))), (var_temp__blk936_dn6 - ((var_delta_nd_dn6 * assign44710_e57860) + (var_delta_nd * (var_x_d_dn6 + var_xi0d_dn6)))), (var_temp__blk936_dn7 - ((var_delta_nd_dn7 * assign44710_e57860) + (var_delta_nd * (var_x_d_dn7 + var_xi0d_dn7)))), (var_temp__blk936_dn8 - ((var_delta_nd_dn8 * assign44710_e57860) + (var_delta_nd * (var_x_d_dn8 + var_xi0d_dn8)))),)
    } else {
        (var_dd, var_dd_dn5, var_dd_dn6, var_dd_dn7, var_dd_dn8,)
    }
};
        var_dd = assign44710_e57864;
        var_dd_dn5 = assign44710_e57864_d_n5;
        var_dd_dn6 = assign44710_e57864_d_n6;
        var_dd_dn7 = assign44710_e57864_d_n7;
        var_dd_dn8 = assign44710_e57864_d_n8;
        var_dd_rv = 0.0;

        let (assign44720_e57896, assign44720_e57896_d_n5, assign44720_e57896_d_n6, assign44720_e57896_d_n7, assign44720_e57896_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 == 0.0)) {
        let assign44720_e57876: f64 = (var_x_d - 230.25850929940458);
        let assign44720_e57881: f64 = (var_x_d - 230.25850929940458);
        let assign44720_e57885: f64 = (var_x_d - 230.25850929940458);
        let assign44720_e57887: f64 = (assign44720_e57885 * 0.3333333333333333);
        let assign44720_e57888: f64 = (1.0 + assign44720_e57887);
        let assign44720_e57889: f64 = (assign44720_e57881 * assign44720_e57888);
        let assign44720_e57890: f64 = (0.5 * assign44720_e57889);
        let assign44720_e57891: f64 = (1.0 + assign44720_e57890);
        let assign44720_e57892: f64 = (assign44720_e57876 * assign44720_e57891);
        let assign44720_e57893: f64 = (1.0 + assign44720_e57892);
        let assign44720_e57894: f64 = (1e-100 / assign44720_e57893);
        (assign44720_e57894, (-((1e-100 * ((var_x_d_dn5 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((var_x_d_dn5 * assign44720_e57888) + (assign44720_e57881 * (var_x_d_dn5 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((var_x_d_dn6 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((var_x_d_dn6 * assign44720_e57888) + (assign44720_e57881 * (var_x_d_dn6 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((var_x_d_dn7 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((var_x_d_dn7 * assign44720_e57888) + (assign44720_e57881 * (var_x_d_dn7 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))), (-((1e-100 * ((var_x_d_dn8 * assign44720_e57891) + (assign44720_e57876 * (0.5 * ((var_x_d_dn8 * assign44720_e57888) + (assign44720_e57881 * (var_x_d_dn8 * 0.3333333333333333))))))) / (assign44720_e57893 * assign44720_e57893))),)
    } else {
        (var_ed, var_ed_dn5, var_ed_dn6, var_ed_dn7, var_ed_dn8,)
    }
};
        var_ed = assign44720_e57896;
        var_ed_dn5 = assign44720_e57896_d_n5;
        var_ed_dn6 = assign44720_e57896_d_n6;
        var_ed_dn7 = assign44720_e57896_d_n7;
        var_ed_dn8 = assign44720_e57896_d_n8;
        var_ed_rv = 0.0;

        let (assign44730_e57934, assign44730_e57934_d_n5, assign44730_e57934_d_n6, assign44730_e57934_d_n7, assign44730_e57934_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 == 0.0)) {
        let assign44730_e57908: f64 = (var_xn_d - var_x_d);
        let assign44730_e57910: f64 = (assign44730_e57908 - 230.25850929940458);
        let assign44730_e57915: f64 = (var_xn_d - var_x_d);
        let assign44730_e57917: f64 = (assign44730_e57915 - 230.25850929940458);
        let assign44730_e57921: f64 = (var_xn_d - var_x_d);
        let assign44730_e57923: f64 = (assign44730_e57921 - 230.25850929940458);
        let assign44730_e57925: f64 = (assign44730_e57923 * 0.3333333333333333);
        let assign44730_e57926: f64 = (1.0 + assign44730_e57925);
        let assign44730_e57927: f64 = (assign44730_e57917 * assign44730_e57926);
        let assign44730_e57928: f64 = (0.5 * assign44730_e57927);
        let assign44730_e57929: f64 = (1.0 + assign44730_e57928);
        let assign44730_e57930: f64 = (assign44730_e57910 * assign44730_e57929);
        let assign44730_e57931: f64 = (1.0 + assign44730_e57930);
        let assign44730_e57932: f64 = (1e-100 / assign44730_e57931);
        (assign44730_e57932, (-((1e-100 * (((var_xn_d_dn5 - var_x_d_dn5) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((var_xn_d_dn5 - var_x_d_dn5) * assign44730_e57926) + (assign44730_e57917 * ((var_xn_d_dn5 - var_x_d_dn5) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((var_xn_d_dn6 - var_x_d_dn6) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((var_xn_d_dn6 - var_x_d_dn6) * assign44730_e57926) + (assign44730_e57917 * ((var_xn_d_dn6 - var_x_d_dn6) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((var_xn_d_dn7 - var_x_d_dn7) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((var_xn_d_dn7 - var_x_d_dn7) * assign44730_e57926) + (assign44730_e57917 * ((var_xn_d_dn7 - var_x_d_dn7) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))), (-((1e-100 * (((var_xn_d_dn8 - var_x_d_dn8) * assign44730_e57929) + (assign44730_e57910 * (0.5 * (((var_xn_d_dn8 - var_x_d_dn8) * assign44730_e57926) + (assign44730_e57917 * ((var_xn_d_dn8 - var_x_d_dn8) * 0.3333333333333333))))))) / (assign44730_e57931 * assign44730_e57931))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44730_e57934;
        var_temp__blk936_dn5 = assign44730_e57934_d_n5;
        var_temp__blk936_dn6 = assign44730_e57934_d_n6;
        var_temp__blk936_dn7 = assign44730_e57934_d_n7;
        var_temp__blk936_dn8 = assign44730_e57934_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44740_e57952, assign44740_e57952_d_n5, assign44740_e57952_d_n6, assign44740_e57952_d_n7, assign44740_e57952_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) && (var_guard1211 == 0.0)) {
        let assign44740_e57946: f64 = (var_x_d + 1.0);
        let assign44740_e57948: f64 = (assign44740_e57946 + var_xi0d);
        let assign44740_e57949: f64 = (var_delta_nd * assign44740_e57948);
        let assign44740_e57950: f64 = (var_temp__blk936 - assign44740_e57949);
        (assign44740_e57950, (var_temp__blk936_dn5 - ((var_delta_nd_dn5 * assign44740_e57948) + (var_delta_nd * (var_x_d_dn5 + var_xi0d_dn5)))), (var_temp__blk936_dn6 - ((var_delta_nd_dn6 * assign44740_e57948) + (var_delta_nd * (var_x_d_dn6 + var_xi0d_dn6)))), (var_temp__blk936_dn7 - ((var_delta_nd_dn7 * assign44740_e57948) + (var_delta_nd * (var_x_d_dn7 + var_xi0d_dn7)))), (var_temp__blk936_dn8 - ((var_delta_nd_dn8 * assign44740_e57948) + (var_delta_nd * (var_x_d_dn8 + var_xi0d_dn8)))),)
    } else {
        (var_dd, var_dd_dn5, var_dd_dn6, var_dd_dn7, var_dd_dn8,)
    }
};
        var_dd = assign44740_e57952;
        var_dd_dn5 = assign44740_e57952_d_n5;
        var_dd_dn6 = assign44740_e57952_d_n6;
        var_dd_dn7 = assign44740_e57952_d_n7;
        var_dd_dn8 = assign44740_e57952_d_n8;
        var_dd_rv = 0.0;

        let (assign44750_e57963, assign44750_e57963_d_n5, assign44750_e57963_d_n6, assign44750_e57963_d_n7, assign44750_e57963_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) {
        let assign44750_e57959: f64 = (var_x_d - 1.0);
        let assign44750_e57961: f64 = (assign44750_e57959 + var_ed);
        (assign44750_e57961, (var_x_d_dn5 + var_ed_dn5), (var_x_d_dn6 + var_ed_dn6), (var_x_d_dn7 + var_ed_dn7), (var_x_d_dn8 + var_ed_dn8),)
    } else {
        (var_pd, var_pd_dn5, var_pd_dn6, var_pd_dn7, var_pd_dn8,)
    }
};
        var_pd = assign44750_e57963;
        var_pd_dn5 = assign44750_e57963_d_n5;
        var_pd_dn6 = assign44750_e57963_d_n6;
        var_pd_dn7 = assign44750_e57963_d_n7;
        var_pd_dn8 = assign44750_e57963_d_n8;
        var_pd_rv = 0.0;

        let (assign44760_e57971, assign44760_e57971_d_n5, assign44760_e57971_d_n6, assign44760_e57971_d_n7, assign44760_e57971_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1209 == 0.0)) {
        let assign44760_e57969: f64 = (var_pd).sqrt();
        (assign44760_e57969, (var_pd_dn5 / (2.0 * assign44760_e57969)), (var_pd_dn6 / (2.0 * assign44760_e57969)), (var_pd_dn7 / (2.0 * assign44760_e57969)), (var_pd_dn8 / (2.0 * assign44760_e57969)),)
    } else {
        (var_sqd, var_sqd_dn5, var_sqd_dn6, var_sqd_dn7, var_sqd_dn8,)
    }
};
        var_sqd = assign44760_e57971;
        var_sqd_dn5 = assign44760_e57971_d_n5;
        var_sqd_dn6 = assign44760_e57971_d_n6;
        var_sqd_dn7 = assign44760_e57971_d_n7;
        var_sqd_dn8 = assign44760_e57971_d_n8;
        var_sqd_rv = 0.0;

        let (assign44770_e57979, assign44770_e57979_d_n5, assign44770_e57979_d_n6, assign44770_e57979_d_n7, assign44770_e57979_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44770_e57975: f64 = (var_sqd * var_gf);
        let assign44770_e57977: f64 = (assign44770_e57975 * var_phit1);
        (assign44770_e57977, ((((var_sqd_dn5 * var_gf) + (var_sqd * var_gf_dn5)) * var_phit1) + (assign44770_e57975 * var_phit1_dn5)), ((((var_sqd_dn6 * var_gf) + (var_sqd * var_gf_dn6)) * var_phit1) + (assign44770_e57975 * var_phit1_dn6)), ((((var_sqd_dn7 * var_gf) + (var_sqd * var_gf_dn7)) * var_phit1) + (assign44770_e57975 * var_phit1_dn7)), ((((var_sqd_dn8 * var_gf) + (var_sqd * var_gf_dn8)) * var_phit1) + (assign44770_e57975 * var_phit1_dn8)),)
    } else {
        (var_qbd, var_qbd_dn5, var_qbd_dn6, var_qbd_dn7, var_qbd_dn8,)
    }
};
        var_qbd = assign44770_e57979;
        var_qbd_dn5 = assign44770_e57979_d_n5;
        var_qbd_dn6 = assign44770_e57979_d_n6;
        var_qbd_dn7 = assign44770_e57979_d_n7;
        var_qbd_dn8 = assign44770_e57979_d_n8;
        var_qbd_rv = 0.0;

        let (assign44780_e57987, assign44780_e57987_d_n5, assign44780_e57987_d_n6, assign44780_e57987_d_n7, assign44780_e57987_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44780_e57984: f64 = (var_x_s + var_x_d);
        let assign44780_e57985: f64 = (0.5 * assign44780_e57984);
        (assign44780_e57985, (0.5 * (var_x_s_dn5 + var_x_d_dn5)), (0.5 * (var_x_s_dn6 + var_x_d_dn6)), (0.5 * (var_x_s_dn7 + var_x_d_dn7)), (0.5 * (var_x_s_dn8 + var_x_d_dn8)),)
    } else {
        (var_x_m, var_x_m_dn5, var_x_m_dn6, var_x_m_dn7, var_x_m_dn8,)
    }
};
        var_x_m = assign44780_e57987;
        var_x_m_dn5 = assign44780_e57987_d_n5;
        var_x_m_dn6 = assign44780_e57987_d_n6;
        var_x_m_dn7 = assign44780_e57987_d_n7;
        var_x_m_dn8 = assign44780_e57987_d_n8;
        var_x_m_rv = 0.0;

        let (assign44790_e57991, assign44790_e57991_d_n5, assign44790_e57991_d_n6, assign44790_e57991_d_n7, assign44790_e57991_d_n8,) = {
    if (var_guard1197 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_em, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8,)
    }
};
        var_em = assign44790_e57991;
        var_em_dn5 = assign44790_e57991_d_n5;
        var_em_dn6 = assign44790_e57991_d_n6;
        var_em_dn7 = assign44790_e57991_d_n7;
        var_em_dn8 = assign44790_e57991_d_n8;
        var_em_rv = 0.0;

        let (assign44800_e57997, assign44800_e57997_d_n5, assign44800_e57997_d_n6, assign44800_e57997_d_n7, assign44800_e57997_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44800_e57995: f64 = (var_ed * var_es);
        (assign44800_e57995, ((var_ed_dn5 * var_es) + (var_ed * var_es_dn5)), ((var_ed_dn6 * var_es) + (var_ed * var_es_dn6)), ((var_ed_dn7 * var_es) + (var_ed * var_es_dn7)), ((var_ed_dn8 * var_es) + (var_ed * var_es_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44800_e57997;
        var_temp__blk936_dn5 = assign44800_e57997_d_n5;
        var_temp__blk936_dn6 = assign44800_e57997_d_n6;
        var_temp__blk936_dn7 = assign44800_e57997_d_n7;
        var_temp__blk936_dn8 = assign44800_e57997_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign44810_e58000: f64 = if var_temp__blk936 > 0.0 { 1.0 } else { 0.0 };
        var_guard1212 = assign44810_e58000;
        var_guard1212_rv = 0.0;

        let (assign44820_e58007, assign44820_e58007_d_n5, assign44820_e58007_d_n6, assign44820_e58007_d_n7, assign44820_e58007_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1212 != 0.0)) {
        let assign44820_e58005: f64 = (var_temp__blk936).sqrt();
        (assign44820_e58005, (var_temp__blk936_dn5 / (2.0 * assign44820_e58005)), (var_temp__blk936_dn6 / (2.0 * assign44820_e58005)), (var_temp__blk936_dn7 / (2.0 * assign44820_e58005)), (var_temp__blk936_dn8 / (2.0 * assign44820_e58005)),)
    } else {
        (var_em, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8,)
    }
};
        var_em = assign44820_e58007;
        var_em_dn5 = assign44820_e58007_d_n5;
        var_em_dn6 = assign44820_e58007_d_n6;
        var_em_dn7 = assign44820_e58007_d_n7;
        var_em_dn8 = assign44820_e58007_d_n8;
        var_em_rv = 0.0;

        let (assign44830_e58015, assign44830_e58015_d_n5, assign44830_e58015_d_n6, assign44830_e58015_d_n7, assign44830_e58015_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44830_e58012: f64 = (var_ds + var_dd);
        let assign44830_e58013: f64 = (0.5 * assign44830_e58012);
        (assign44830_e58013, (0.5 * (var_ds_dn5 + var_dd_dn5)), (0.5 * (var_ds_dn6 + var_dd_dn6)), (0.5 * (var_ds_dn7 + var_dd_dn7)), (0.5 * (var_ds_dn8 + var_dd_dn8)),)
    } else {
        (var_d_bar, var_d_bar_dn5, var_d_bar_dn6, var_d_bar_dn7, var_d_bar_dn8,)
    }
};
        var_d_bar = assign44830_e58015;
        var_d_bar_dn5 = assign44830_e58015_d_n5;
        var_d_bar_dn6 = assign44830_e58015_d_n6;
        var_d_bar_dn7 = assign44830_e58015_d_n7;
        var_d_bar_dn8 = assign44830_e58015_d_n8;
        var_d_bar_rv = 0.0;

        *var_d_bar_slot = var_d_bar;
        *var_d_bar_dn5_slot = var_d_bar_dn5;
        *var_d_bar_dn6_slot = var_d_bar_dn6;
        *var_d_bar_dn7_slot = var_d_bar_dn7;
        *var_d_bar_dn8_slot = var_d_bar_dn8;
        *var_d_bar_rv_slot = var_d_bar_rv;
        *var_dd_slot = var_dd;
        *var_dd_dn5_slot = var_dd_dn5;
        *var_dd_dn6_slot = var_dd_dn6;
        *var_dd_dn7_slot = var_dd_dn7;
        *var_dd_dn8_slot = var_dd_dn8;
        *var_dd_rv_slot = var_dd_rv;
        *var_dps_slot = var_dps;
        *var_dps_dn5_slot = var_dps_dn5;
        *var_dps_dn6_slot = var_dps_dn6;
        *var_dps_dn7_slot = var_dps_dn7;
        *var_dps_dn8_slot = var_dps_dn8;
        *var_dps_rv_slot = var_dps_rv;
        *var_ed_slot = var_ed;
        *var_ed_dn5_slot = var_ed_dn5;
        *var_ed_dn6_slot = var_ed_dn6;
        *var_ed_dn7_slot = var_ed_dn7;
        *var_ed_dn8_slot = var_ed_dn8;
        *var_ed_rv_slot = var_ed_rv;
        *var_em_slot = var_em;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_rv_slot = var_em_rv;
        *var_guard1209_slot = var_guard1209;
        *var_guard1209_rv_slot = var_guard1209_rv;
        *var_guard1210_slot = var_guard1210;
        *var_guard1210_rv_slot = var_guard1210_rv;
        *var_guard1211_slot = var_guard1211;
        *var_guard1211_rv_slot = var_guard1211_rv;
        *var_guard1212_slot = var_guard1212;
        *var_guard1212_rv_slot = var_guard1212_rv;
        *var_pd_slot = var_pd;
        *var_pd_dn5_slot = var_pd_dn5;
        *var_pd_dn6_slot = var_pd_dn6;
        *var_pd_dn7_slot = var_pd_dn7;
        *var_pd_dn8_slot = var_pd_dn8;
        *var_pd_rv_slot = var_pd_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn5_slot = var_qbd_dn5;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_sqd_slot = var_sqd;
        *var_sqd_dn5_slot = var_sqd_dn5;
        *var_sqd_dn6_slot = var_sqd_dn6;
        *var_sqd_dn7_slot = var_sqd_dn7;
        *var_sqd_dn8_slot = var_sqd_dn8;
        *var_sqd_rv_slot = var_sqd_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_x_d_slot = var_x_d;
        *var_x_d_dn5_slot = var_x_d_dn5;
        *var_x_d_dn6_slot = var_x_d_dn6;
        *var_x_d_dn7_slot = var_x_d_dn7;
        *var_x_d_dn8_slot = var_x_d_dn8;
        *var_x_d_rv_slot = var_x_d_rv;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn5_slot = var_x_ds_dn5;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_ds_rv_slot = var_x_ds_rv;
        *var_x_m_slot = var_x_m;
        *var_x_m_dn5_slot = var_x_m_dn5;
        *var_x_m_dn6_slot = var_x_m_dn6;
        *var_x_m_dn7_slot = var_x_m_dn7;
        *var_x_m_dn8_slot = var_x_m_dn8;
        *var_x_m_rv_slot = var_x_m_rv;
        *var_xi0d_slot = var_xi0d;
        *var_xi0d_dn5_slot = var_xi0d_dn5;
        *var_xi0d_dn6_slot = var_xi0d_dn6;
        *var_xi0d_dn7_slot = var_xi0d_dn7;
        *var_xi0d_dn8_slot = var_xi0d_dn8;
        *var_xi0d_rv_slot = var_xi0d_rv;
    }

    pub(super) fn stamp_reactive_block_35(
        var_d_bar: f64,
        var_d_bar_dn5: f64,
        var_d_bar_dn6: f64,
        var_d_bar_dn7: f64,
        var_d_bar_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1197: f64,
        var_inv_gf2: f64,
        var_inv_gf2_dn5: f64,
        var_inv_gf2_dn6: f64,
        var_inv_gf2_dn7: f64,
        var_inv_gf2_dn8: f64,
        var_kp: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn5_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alpha_rv_slot: &mut f64,
        var_d0_slot: &mut f64,
        var_d0_dn5_slot: &mut f64,
        var_d0_dn6_slot: &mut f64,
        var_d0_dn7_slot: &mut f64,
        var_d0_dn8_slot: &mut f64,
        var_d0_rv_slot: &mut f64,
        var_dm_slot: &mut f64,
        var_dm_dn5_slot: &mut f64,
        var_dm_dn6_slot: &mut f64,
        var_dm_dn7_slot: &mut f64,
        var_dm_dn8_slot: &mut f64,
        var_dm_rv_slot: &mut f64,
        var_dps_slot: &mut f64,
        var_dps_dn5_slot: &mut f64,
        var_dps_dn6_slot: &mut f64,
        var_dps_dn7_slot: &mut f64,
        var_dps_dn8_slot: &mut f64,
        var_dps_rv_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_rv_slot: &mut f64,
        var_eta_p_slot: &mut f64,
        var_eta_p_dn5_slot: &mut f64,
        var_eta_p_dn6_slot: &mut f64,
        var_eta_p_dn7_slot: &mut f64,
        var_eta_p_dn8_slot: &mut f64,
        var_eta_p_rv_slot: &mut f64,
        var_guard1213_slot: &mut f64,
        var_guard1213_rv_slot: &mut f64,
        var_guard1214_slot: &mut f64,
        var_guard1214_rv_slot: &mut f64,
        var_guard1215_slot: &mut f64,
        var_guard1215_rv_slot: &mut f64,
        var_km_slot: &mut f64,
        var_km0_slot: &mut f64,
        var_km0_dn5_slot: &mut f64,
        var_km0_dn6_slot: &mut f64,
        var_km0_dn7_slot: &mut f64,
        var_km0_dn8_slot: &mut f64,
        var_km0_rv_slot: &mut f64,
        var_km_dn5_slot: &mut f64,
        var_km_dn6_slot: &mut f64,
        var_km_dn7_slot: &mut f64,
        var_km_dn8_slot: &mut f64,
        var_km_rv_slot: &mut f64,
        var_p_pd_slot: &mut f64,
        var_p_pd_dn5_slot: &mut f64,
        var_p_pd_dn6_slot: &mut f64,
        var_p_pd_dn7_slot: &mut f64,
        var_p_pd_dn8_slot: &mut f64,
        var_p_pd_rv_slot: &mut f64,
        var_pm_slot: &mut f64,
        var_pm_dn5_slot: &mut f64,
        var_pm_dn6_slot: &mut f64,
        var_pm_dn7_slot: &mut f64,
        var_pm_dn8_slot: &mut f64,
        var_pm_rv_slot: &mut f64,
        var_q_pd_slot: &mut f64,
        var_q_pd_dn5_slot: &mut f64,
        var_q_pd_dn6_slot: &mut f64,
        var_q_pd_dn7_slot: &mut f64,
        var_q_pd_dn8_slot: &mut f64,
        var_q_pd_rv_slot: &mut f64,
        var_sqm_slot: &mut f64,
        var_sqm_dn5_slot: &mut f64,
        var_sqm_dn6_slot: &mut f64,
        var_sqm_dn7_slot: &mut f64,
        var_sqm_dn8_slot: &mut f64,
        var_sqm_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_u_pd_slot: &mut f64,
        var_u_pd_dn5_slot: &mut f64,
        var_u_pd_dn6_slot: &mut f64,
        var_u_pd_dn7_slot: &mut f64,
        var_u_pd_dn8_slot: &mut f64,
        var_u_pd_rv_slot: &mut f64,
        var_x_ds_slot: &mut f64,
        var_x_ds_dn5_slot: &mut f64,
        var_x_ds_dn6_slot: &mut f64,
        var_x_ds_dn7_slot: &mut f64,
        var_x_ds_dn8_slot: &mut f64,
        var_x_ds_rv_slot: &mut f64,
        var_x_m_slot: &mut f64,
        var_x_m_dn5_slot: &mut f64,
        var_x_m_dn6_slot: &mut f64,
        var_x_m_dn7_slot: &mut f64,
        var_x_m_dn8_slot: &mut f64,
        var_x_m_rv_slot: &mut f64,
        var_x_pm_slot: &mut f64,
        var_x_pm_dn5_slot: &mut f64,
        var_x_pm_dn6_slot: &mut f64,
        var_x_pm_dn7_slot: &mut f64,
        var_x_pm_dn8_slot: &mut f64,
        var_x_pm_rv_slot: &mut f64,
        var_xgm_slot: &mut f64,
        var_xgm_dn5_slot: &mut f64,
        var_xgm_dn6_slot: &mut f64,
        var_xgm_dn7_slot: &mut f64,
        var_xgm_dn8_slot: &mut f64,
        var_xgm_rv_slot: &mut f64,
        var_xi_pd_slot: &mut f64,
        var_xi_pd_dn5_slot: &mut f64,
        var_xi_pd_dn6_slot: &mut f64,
        var_xi_pd_dn7_slot: &mut f64,
        var_xi_pd_dn8_slot: &mut f64,
        var_xi_pd_rv_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn5: f64 = *var_alpha_dn5_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alpha_rv: f64 = *var_alpha_rv_slot;
        let mut var_d0: f64 = *var_d0_slot;
        let mut var_d0_dn5: f64 = *var_d0_dn5_slot;
        let mut var_d0_dn6: f64 = *var_d0_dn6_slot;
        let mut var_d0_dn7: f64 = *var_d0_dn7_slot;
        let mut var_d0_dn8: f64 = *var_d0_dn8_slot;
        let mut var_d0_rv: f64 = *var_d0_rv_slot;
        let mut var_dm: f64 = *var_dm_slot;
        let mut var_dm_dn5: f64 = *var_dm_dn5_slot;
        let mut var_dm_dn6: f64 = *var_dm_dn6_slot;
        let mut var_dm_dn7: f64 = *var_dm_dn7_slot;
        let mut var_dm_dn8: f64 = *var_dm_dn8_slot;
        let mut var_dm_rv: f64 = *var_dm_rv_slot;
        let mut var_dps: f64 = *var_dps_slot;
        let mut var_dps_dn5: f64 = *var_dps_dn5_slot;
        let mut var_dps_dn6: f64 = *var_dps_dn6_slot;
        let mut var_dps_dn7: f64 = *var_dps_dn7_slot;
        let mut var_dps_dn8: f64 = *var_dps_dn8_slot;
        let mut var_dps_rv: f64 = *var_dps_rv_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_rv: f64 = *var_em_rv_slot;
        let mut var_eta_p: f64 = *var_eta_p_slot;
        let mut var_eta_p_dn5: f64 = *var_eta_p_dn5_slot;
        let mut var_eta_p_dn6: f64 = *var_eta_p_dn6_slot;
        let mut var_eta_p_dn7: f64 = *var_eta_p_dn7_slot;
        let mut var_eta_p_dn8: f64 = *var_eta_p_dn8_slot;
        let mut var_eta_p_rv: f64 = *var_eta_p_rv_slot;
        let mut var_guard1213: f64 = *var_guard1213_slot;
        let mut var_guard1213_rv: f64 = *var_guard1213_rv_slot;
        let mut var_guard1214: f64 = *var_guard1214_slot;
        let mut var_guard1214_rv: f64 = *var_guard1214_rv_slot;
        let mut var_guard1215: f64 = *var_guard1215_slot;
        let mut var_guard1215_rv: f64 = *var_guard1215_rv_slot;
        let mut var_km: f64 = *var_km_slot;
        let mut var_km0: f64 = *var_km0_slot;
        let mut var_km0_dn5: f64 = *var_km0_dn5_slot;
        let mut var_km0_dn6: f64 = *var_km0_dn6_slot;
        let mut var_km0_dn7: f64 = *var_km0_dn7_slot;
        let mut var_km0_dn8: f64 = *var_km0_dn8_slot;
        let mut var_km0_rv: f64 = *var_km0_rv_slot;
        let mut var_km_dn5: f64 = *var_km_dn5_slot;
        let mut var_km_dn6: f64 = *var_km_dn6_slot;
        let mut var_km_dn7: f64 = *var_km_dn7_slot;
        let mut var_km_dn8: f64 = *var_km_dn8_slot;
        let mut var_km_rv: f64 = *var_km_rv_slot;
        let mut var_p_pd: f64 = *var_p_pd_slot;
        let mut var_p_pd_dn5: f64 = *var_p_pd_dn5_slot;
        let mut var_p_pd_dn6: f64 = *var_p_pd_dn6_slot;
        let mut var_p_pd_dn7: f64 = *var_p_pd_dn7_slot;
        let mut var_p_pd_dn8: f64 = *var_p_pd_dn8_slot;
        let mut var_p_pd_rv: f64 = *var_p_pd_rv_slot;
        let mut var_pm: f64 = *var_pm_slot;
        let mut var_pm_dn5: f64 = *var_pm_dn5_slot;
        let mut var_pm_dn6: f64 = *var_pm_dn6_slot;
        let mut var_pm_dn7: f64 = *var_pm_dn7_slot;
        let mut var_pm_dn8: f64 = *var_pm_dn8_slot;
        let mut var_pm_rv: f64 = *var_pm_rv_slot;
        let mut var_q_pd: f64 = *var_q_pd_slot;
        let mut var_q_pd_dn5: f64 = *var_q_pd_dn5_slot;
        let mut var_q_pd_dn6: f64 = *var_q_pd_dn6_slot;
        let mut var_q_pd_dn7: f64 = *var_q_pd_dn7_slot;
        let mut var_q_pd_dn8: f64 = *var_q_pd_dn8_slot;
        let mut var_q_pd_rv: f64 = *var_q_pd_rv_slot;
        let mut var_sqm: f64 = *var_sqm_slot;
        let mut var_sqm_dn5: f64 = *var_sqm_dn5_slot;
        let mut var_sqm_dn6: f64 = *var_sqm_dn6_slot;
        let mut var_sqm_dn7: f64 = *var_sqm_dn7_slot;
        let mut var_sqm_dn8: f64 = *var_sqm_dn8_slot;
        let mut var_sqm_rv: f64 = *var_sqm_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_u_pd: f64 = *var_u_pd_slot;
        let mut var_u_pd_dn5: f64 = *var_u_pd_dn5_slot;
        let mut var_u_pd_dn6: f64 = *var_u_pd_dn6_slot;
        let mut var_u_pd_dn7: f64 = *var_u_pd_dn7_slot;
        let mut var_u_pd_dn8: f64 = *var_u_pd_dn8_slot;
        let mut var_u_pd_rv: f64 = *var_u_pd_rv_slot;
        let mut var_x_ds: f64 = *var_x_ds_slot;
        let mut var_x_ds_dn5: f64 = *var_x_ds_dn5_slot;
        let mut var_x_ds_dn6: f64 = *var_x_ds_dn6_slot;
        let mut var_x_ds_dn7: f64 = *var_x_ds_dn7_slot;
        let mut var_x_ds_dn8: f64 = *var_x_ds_dn8_slot;
        let mut var_x_ds_rv: f64 = *var_x_ds_rv_slot;
        let mut var_x_m: f64 = *var_x_m_slot;
        let mut var_x_m_dn5: f64 = *var_x_m_dn5_slot;
        let mut var_x_m_dn6: f64 = *var_x_m_dn6_slot;
        let mut var_x_m_dn7: f64 = *var_x_m_dn7_slot;
        let mut var_x_m_dn8: f64 = *var_x_m_dn8_slot;
        let mut var_x_m_rv: f64 = *var_x_m_rv_slot;
        let mut var_x_pm: f64 = *var_x_pm_slot;
        let mut var_x_pm_dn5: f64 = *var_x_pm_dn5_slot;
        let mut var_x_pm_dn6: f64 = *var_x_pm_dn6_slot;
        let mut var_x_pm_dn7: f64 = *var_x_pm_dn7_slot;
        let mut var_x_pm_dn8: f64 = *var_x_pm_dn8_slot;
        let mut var_x_pm_rv: f64 = *var_x_pm_rv_slot;
        let mut var_xgm: f64 = *var_xgm_slot;
        let mut var_xgm_dn5: f64 = *var_xgm_dn5_slot;
        let mut var_xgm_dn6: f64 = *var_xgm_dn6_slot;
        let mut var_xgm_dn7: f64 = *var_xgm_dn7_slot;
        let mut var_xgm_dn8: f64 = *var_xgm_dn8_slot;
        let mut var_xgm_rv: f64 = *var_xgm_rv_slot;
        let mut var_xi_pd: f64 = *var_xi_pd_slot;
        let mut var_xi_pd_dn5: f64 = *var_xi_pd_dn5_slot;
        let mut var_xi_pd_dn6: f64 = *var_xi_pd_dn6_slot;
        let mut var_xi_pd_dn7: f64 = *var_xi_pd_dn7_slot;
        let mut var_xi_pd_dn8: f64 = *var_xi_pd_dn8_slot;
        let mut var_xi_pd_rv: f64 = *var_xi_pd_rv_slot;

        let (assign44840_e58031, assign44840_e58031_d_n5, assign44840_e58031_d_n6, assign44840_e58031_d_n7, assign44840_e58031_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign44840_e58021: f64 = (var_x_ds * var_x_ds);
        let assign44840_e58025: f64 = (2.0 * var_inv_gf2);
        let assign44840_e58026: f64 = (var_em - assign44840_e58025);
        let assign44840_e58027: f64 = (assign44840_e58021 * assign44840_e58026);
        let assign44840_e58028: f64 = (0.125 * assign44840_e58027);
        let assign44840_e58029: f64 = (var_d_bar + assign44840_e58028);
        (assign44840_e58029, (var_d_bar_dn5 + (0.125 * ((((var_x_ds_dn5 * var_x_ds) + (var_x_ds * var_x_ds_dn5)) * assign44840_e58026) + (assign44840_e58021 * (var_em_dn5 - (2.0 * var_inv_gf2_dn5)))))), (var_d_bar_dn6 + (0.125 * ((((var_x_ds_dn6 * var_x_ds) + (var_x_ds * var_x_ds_dn6)) * assign44840_e58026) + (assign44840_e58021 * (var_em_dn6 - (2.0 * var_inv_gf2_dn6)))))), (var_d_bar_dn7 + (0.125 * ((((var_x_ds_dn7 * var_x_ds) + (var_x_ds * var_x_ds_dn7)) * assign44840_e58026) + (assign44840_e58021 * (var_em_dn7 - (2.0 * var_inv_gf2_dn7)))))), (var_d_bar_dn8 + (0.125 * ((((var_x_ds_dn8 * var_x_ds) + (var_x_ds * var_x_ds_dn8)) * assign44840_e58026) + (assign44840_e58021 * (var_em_dn8 - (2.0 * var_inv_gf2_dn8)))))),)
    } else {
        (var_dm, var_dm_dn5, var_dm_dn6, var_dm_dn7, var_dm_dn8,)
    }
};
        var_dm = assign44840_e58031;
        var_dm_dn5 = assign44840_e58031_d_n5;
        var_dm_dn6 = assign44840_e58031_d_n6;
        var_dm_dn7 = assign44840_e58031_d_n7;
        var_dm_dn8 = assign44840_e58031_d_n8;
        var_dm_rv = 0.0;

        let assign44850_e58034: f64 = if var_x_m < 1e-5 { 1.0 } else { 0.0 };
        var_guard1213 = assign44850_e58034;
        var_guard1213_rv = 0.0;

        let (assign44860_e58056, assign44860_e58056_d_n5, assign44860_e58056_d_n6, assign44860_e58056_d_n7, assign44860_e58056_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) {
        let assign44860_e58041: f64 = (var_x_m * var_x_m);
        let assign44860_e58048: f64 = (0.25 * var_x_m);
        let assign44860_e58049: f64 = (1.0 - assign44860_e58048);
        let assign44860_e58050: f64 = (var_x_m * assign44860_e58049);
        let assign44860_e58051: f64 = (0.3333333333333333 * assign44860_e58050);
        let assign44860_e58052: f64 = (1.0 - assign44860_e58051);
        let assign44860_e58053: f64 = (assign44860_e58041 * assign44860_e58052);
        let assign44860_e58054: f64 = (0.5 * assign44860_e58053);
        (assign44860_e58054, (0.5 * ((((var_x_m_dn5 * var_x_m) + (var_x_m * var_x_m_dn5)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((var_x_m_dn5 * assign44860_e58049) + (var_x_m * (-(0.25 * var_x_m_dn5))))))))), (0.5 * ((((var_x_m_dn6 * var_x_m) + (var_x_m * var_x_m_dn6)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((var_x_m_dn6 * assign44860_e58049) + (var_x_m * (-(0.25 * var_x_m_dn6))))))))), (0.5 * ((((var_x_m_dn7 * var_x_m) + (var_x_m * var_x_m_dn7)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((var_x_m_dn7 * assign44860_e58049) + (var_x_m * (-(0.25 * var_x_m_dn7))))))))), (0.5 * ((((var_x_m_dn8 * var_x_m) + (var_x_m * var_x_m_dn8)) * assign44860_e58052) + (assign44860_e58041 * (-(0.3333333333333333 * ((var_x_m_dn8 * assign44860_e58049) + (var_x_m * (-(0.25 * var_x_m_dn8))))))))),)
    } else {
        (var_pm, var_pm_dn5, var_pm_dn6, var_pm_dn7, var_pm_dn8,)
    }
};
        var_pm = assign44860_e58056;
        var_pm_dn5 = assign44860_e58056_d_n5;
        var_pm_dn6 = assign44860_e58056_d_n6;
        var_pm_dn7 = assign44860_e58056_d_n7;
        var_pm_dn8 = assign44860_e58056_d_n8;
        var_pm_rv = 0.0;

        let (assign44870_e58067, assign44870_e58067_d_n5, assign44870_e58067_d_n6, assign44870_e58067_d_n7, assign44870_e58067_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) {
        let assign44870_e58063: f64 = (var_dm + var_pm);
        let assign44870_e58064: f64 = (assign44870_e58063).sqrt();
        let assign44870_e58065: f64 = (var_gf * assign44870_e58064);
        (assign44870_e58065, ((var_gf_dn5 * assign44870_e58064) + (var_gf * ((var_dm_dn5 + var_pm_dn5) / (2.0 * assign44870_e58064)))), ((var_gf_dn6 * assign44870_e58064) + (var_gf * ((var_dm_dn6 + var_pm_dn6) / (2.0 * assign44870_e58064)))), ((var_gf_dn7 * assign44870_e58064) + (var_gf * ((var_dm_dn7 + var_pm_dn7) / (2.0 * assign44870_e58064)))), ((var_gf_dn8 * assign44870_e58064) + (var_gf * ((var_dm_dn8 + var_pm_dn8) / (2.0 * assign44870_e58064)))),)
    } else {
        (var_xgm, var_xgm_dn5, var_xgm_dn6, var_xgm_dn7, var_xgm_dn8,)
    }
};
        var_xgm = assign44870_e58067;
        var_xgm_dn5 = assign44870_e58067_d_n5;
        var_xgm_dn6 = assign44870_e58067_d_n6;
        var_xgm_dn7 = assign44870_e58067_d_n7;
        var_xgm_dn8 = assign44870_e58067_d_n8;
        var_xgm_rv = 0.0;

        let assign44880_e58070: f64 = if var_kp > 0.0 { 1.0 } else { 0.0 };
        var_guard1214 = assign44880_e58070;
        var_guard1214_rv = 0.0;

        let (assign44890_e58085, assign44890_e58085_d_n5, assign44890_e58085_d_n6, assign44890_e58085_d_n7, assign44890_e58085_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) && (var_guard1214 != 0.0)) {
        let assign44890_e58080: f64 = (var_kp * var_xgm);
        let assign44890_e58081: f64 = (1.0 + assign44890_e58080);
        let assign44890_e58082: f64 = (assign44890_e58081).sqrt();
        let assign44890_e58083: f64 = (1.0 / assign44890_e58082);
        (assign44890_e58083, (-(((var_kp * var_xgm_dn5) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((var_kp * var_xgm_dn6) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((var_kp * var_xgm_dn7) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))), (-(((var_kp * var_xgm_dn8) / (2.0 * assign44890_e58082)) / (assign44890_e58082 * assign44890_e58082))),)
    } else {
        (var_eta_p, var_eta_p_dn5, var_eta_p_dn6, var_eta_p_dn7, var_eta_p_dn8,)
    }
};
        var_eta_p = assign44890_e58085;
        var_eta_p_dn5 = assign44890_e58085_d_n5;
        var_eta_p_dn6 = assign44890_e58085_d_n6;
        var_eta_p_dn7 = assign44890_e58085_d_n7;
        var_eta_p_dn8 = assign44890_e58085_d_n8;
        var_eta_p_rv = 0.0;

        let (assign44900_e58102, assign44900_e58102_d_n5, assign44900_e58102_d_n6, assign44900_e58102_d_n7, assign44900_e58102_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) {
        let assign44900_e58095: f64 = (0.25 * var_x_m);
        let assign44900_e58096: f64 = (1.0 - assign44900_e58095);
        let assign44900_e58097: f64 = (var_x_m * assign44900_e58096);
        let assign44900_e58098: f64 = (0.3333333333333333 * assign44900_e58097);
        let assign44900_e58099: f64 = (1.0 - assign44900_e58098);
        let assign44900_e58100: f64 = (assign44900_e58099).sqrt();
        (assign44900_e58100, ((-(0.3333333333333333 * ((var_x_m_dn5 * assign44900_e58096) + (var_x_m * (-(0.25 * var_x_m_dn5)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((var_x_m_dn6 * assign44900_e58096) + (var_x_m * (-(0.25 * var_x_m_dn6)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((var_x_m_dn7 * assign44900_e58096) + (var_x_m * (-(0.25 * var_x_m_dn7)))))) / (2.0 * assign44900_e58100)), ((-(0.3333333333333333 * ((var_x_m_dn8 * assign44900_e58096) + (var_x_m * (-(0.25 * var_x_m_dn8)))))) / (2.0 * assign44900_e58100)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44900_e58102;
        var_temp__blk936_dn5 = assign44900_e58102_d_n5;
        var_temp__blk936_dn6 = assign44900_e58102_d_n6;
        var_temp__blk936_dn7 = assign44900_e58102_d_n7;
        var_temp__blk936_dn8 = assign44900_e58102_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44910_e58112, assign44910_e58112_d_n5, assign44910_e58112_d_n6, assign44910_e58112_d_n7, assign44910_e58112_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) {
        let assign44910_e58109: f64 = (var_x_m * var_temp__blk936);
        let assign44910_e58110: f64 = (0.7071067811865475 * assign44910_e58109);
        (assign44910_e58110, (0.7071067811865475 * ((var_x_m_dn5 * var_temp__blk936) + (var_x_m * var_temp__blk936_dn5))), (0.7071067811865475 * ((var_x_m_dn6 * var_temp__blk936) + (var_x_m * var_temp__blk936_dn6))), (0.7071067811865475 * ((var_x_m_dn7 * var_temp__blk936) + (var_x_m * var_temp__blk936_dn7))), (0.7071067811865475 * ((var_x_m_dn8 * var_temp__blk936) + (var_x_m * var_temp__blk936_dn8))),)
    } else {
        (var_sqm, var_sqm_dn5, var_sqm_dn6, var_sqm_dn7, var_sqm_dn8,)
    }
};
        var_sqm = assign44910_e58112;
        var_sqm_dn5 = assign44910_e58112_d_n5;
        var_sqm_dn6 = assign44910_e58112_d_n6;
        var_sqm_dn7 = assign44910_e58112_d_n7;
        var_sqm_dn8 = assign44910_e58112_d_n8;
        var_sqm_rv = 0.0;

        let (assign44920_e58136, assign44920_e58136_d_n5, assign44920_e58136_d_n6, assign44920_e58136_d_n7, assign44920_e58136_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 != 0.0)) {
        let assign44920_e58122: f64 = (0.5 * var_x_m);
        let assign44920_e58123: f64 = (1.0 - assign44920_e58122);
        let assign44920_e58127: f64 = (var_x_m * var_x_m);
        let assign44920_e58128: f64 = (0.16666666666666666 * assign44920_e58127);
        let assign44920_e58129: f64 = (assign44920_e58123 + assign44920_e58128);
        let assign44920_e58130: f64 = (var_gf * assign44920_e58129);
        let assign44920_e58132: f64 = (assign44920_e58130 / var_temp__blk936);
        let assign44920_e58133: f64 = (0.7071067811865475 * assign44920_e58132);
        let assign44920_e58134: f64 = (var_eta_p + assign44920_e58133);
        (assign44920_e58134, (var_eta_p_dn5 + (0.7071067811865475 * (((((var_gf_dn5 * assign44920_e58129) + (var_gf * ((-(0.5 * var_x_m_dn5)) + (0.16666666666666666 * ((var_x_m_dn5 * var_x_m) + (var_x_m * var_x_m_dn5)))))) * var_temp__blk936) - (assign44920_e58130 * var_temp__blk936_dn5)) / (var_temp__blk936 * var_temp__blk936)))), (var_eta_p_dn6 + (0.7071067811865475 * (((((var_gf_dn6 * assign44920_e58129) + (var_gf * ((-(0.5 * var_x_m_dn6)) + (0.16666666666666666 * ((var_x_m_dn6 * var_x_m) + (var_x_m * var_x_m_dn6)))))) * var_temp__blk936) - (assign44920_e58130 * var_temp__blk936_dn6)) / (var_temp__blk936 * var_temp__blk936)))), (var_eta_p_dn7 + (0.7071067811865475 * (((((var_gf_dn7 * assign44920_e58129) + (var_gf * ((-(0.5 * var_x_m_dn7)) + (0.16666666666666666 * ((var_x_m_dn7 * var_x_m) + (var_x_m * var_x_m_dn7)))))) * var_temp__blk936) - (assign44920_e58130 * var_temp__blk936_dn7)) / (var_temp__blk936 * var_temp__blk936)))), (var_eta_p_dn8 + (0.7071067811865475 * (((((var_gf_dn8 * assign44920_e58129) + (var_gf * ((-(0.5 * var_x_m_dn8)) + (0.16666666666666666 * ((var_x_m_dn8 * var_x_m) + (var_x_m * var_x_m_dn8)))))) * var_temp__blk936) - (assign44920_e58130 * var_temp__blk936_dn8)) / (var_temp__blk936 * var_temp__blk936)))),)
    } else {
        (var_alpha, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8,)
    }
};
        var_alpha = assign44920_e58136;
        var_alpha_dn5 = assign44920_e58136_d_n5;
        var_alpha_dn6 = assign44920_e58136_d_n6;
        var_alpha_dn7 = assign44920_e58136_d_n7;
        var_alpha_dn8 = assign44920_e58136_d_n8;
        var_alpha_rv = 0.0;

        let (assign44930_e58147, assign44930_e58147_d_n5, assign44930_e58147_d_n6, assign44930_e58147_d_n7, assign44930_e58147_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) {
        let assign44930_e58143: f64 = (var_x_m - 1.0);
        let assign44930_e58145: f64 = (assign44930_e58143 + var_em);
        (assign44930_e58145, (var_x_m_dn5 + var_em_dn5), (var_x_m_dn6 + var_em_dn6), (var_x_m_dn7 + var_em_dn7), (var_x_m_dn8 + var_em_dn8),)
    } else {
        (var_pm, var_pm_dn5, var_pm_dn6, var_pm_dn7, var_pm_dn8,)
    }
};
        var_pm = assign44930_e58147;
        var_pm_dn5 = assign44930_e58147_d_n5;
        var_pm_dn6 = assign44930_e58147_d_n6;
        var_pm_dn7 = assign44930_e58147_d_n7;
        var_pm_dn8 = assign44930_e58147_d_n8;
        var_pm_rv = 0.0;

        let (assign44940_e58159, assign44940_e58159_d_n5, assign44940_e58159_d_n6, assign44940_e58159_d_n7, assign44940_e58159_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) {
        let assign44940_e58155: f64 = (var_dm + var_pm);
        let assign44940_e58156: f64 = (assign44940_e58155).sqrt();
        let assign44940_e58157: f64 = (var_gf * assign44940_e58156);
        (assign44940_e58157, ((var_gf_dn5 * assign44940_e58156) + (var_gf * ((var_dm_dn5 + var_pm_dn5) / (2.0 * assign44940_e58156)))), ((var_gf_dn6 * assign44940_e58156) + (var_gf * ((var_dm_dn6 + var_pm_dn6) / (2.0 * assign44940_e58156)))), ((var_gf_dn7 * assign44940_e58156) + (var_gf * ((var_dm_dn7 + var_pm_dn7) / (2.0 * assign44940_e58156)))), ((var_gf_dn8 * assign44940_e58156) + (var_gf * ((var_dm_dn8 + var_pm_dn8) / (2.0 * assign44940_e58156)))),)
    } else {
        (var_xgm, var_xgm_dn5, var_xgm_dn6, var_xgm_dn7, var_xgm_dn8,)
    }
};
        var_xgm = assign44940_e58159;
        var_xgm_dn5 = assign44940_e58159_d_n5;
        var_xgm_dn6 = assign44940_e58159_d_n6;
        var_xgm_dn7 = assign44940_e58159_d_n7;
        var_xgm_dn8 = assign44940_e58159_d_n8;
        var_xgm_rv = 0.0;

        let assign44950_e58162: f64 = if var_kp > 0.0 { 1.0 } else { 0.0 };
        var_guard1215 = assign44950_e58162;
        var_guard1215_rv = 0.0;

        let (assign44960_e58179, assign44960_e58179_d_n5, assign44960_e58179_d_n6, assign44960_e58179_d_n7, assign44960_e58179_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign44960_e58171: f64 = (1.0 - var_em);
        let assign44960_e58175: f64 = (var_xgm * var_inv_gf2);
        let assign44960_e58176: f64 = (2.0 * assign44960_e58175);
        let assign44960_e58177: f64 = (assign44960_e58171 + assign44960_e58176);
        (assign44960_e58177, ((-var_em_dn5) + (2.0 * ((var_xgm_dn5 * var_inv_gf2) + (var_xgm * var_inv_gf2_dn5)))), ((-var_em_dn6) + (2.0 * ((var_xgm_dn6 * var_inv_gf2) + (var_xgm * var_inv_gf2_dn6)))), ((-var_em_dn7) + (2.0 * ((var_xgm_dn7 * var_inv_gf2) + (var_xgm * var_inv_gf2_dn7)))), ((-var_em_dn8) + (2.0 * ((var_xgm_dn8 * var_inv_gf2) + (var_xgm * var_inv_gf2_dn8)))),)
    } else {
        (var_d0, var_d0_dn5, var_d0_dn6, var_d0_dn7, var_d0_dn8,)
    }
};
        var_d0 = assign44960_e58179;
        var_d0_dn5 = assign44960_e58179_d_n5;
        var_d0_dn6 = assign44960_e58179_d_n6;
        var_d0_dn7 = assign44960_e58179_d_n7;
        var_d0_dn8 = assign44960_e58179_d_n8;
        var_d0_rv = 0.0;

        let (assign44970_e58195, assign44970_e58195_d_n5, assign44970_e58195_d_n6, assign44970_e58195_d_n7, assign44970_e58195_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign44970_e58190: f64 = (var_kp * var_xgm);
        let assign44970_e58191: f64 = (1.0 + assign44970_e58190);
        let assign44970_e58192: f64 = (assign44970_e58191).sqrt();
        let assign44970_e58193: f64 = (1.0 / assign44970_e58192);
        (assign44970_e58193, (-(((var_kp * var_xgm_dn5) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((var_kp * var_xgm_dn6) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((var_kp * var_xgm_dn7) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))), (-(((var_kp * var_xgm_dn8) / (2.0 * assign44970_e58192)) / (assign44970_e58192 * assign44970_e58192))),)
    } else {
        (var_eta_p, var_eta_p_dn5, var_eta_p_dn6, var_eta_p_dn7, var_eta_p_dn8,)
    }
};
        var_eta_p = assign44970_e58195;
        var_eta_p_dn5 = assign44970_e58195_d_n5;
        var_eta_p_dn6 = assign44970_e58195_d_n6;
        var_eta_p_dn7 = assign44970_e58195_d_n7;
        var_eta_p_dn8 = assign44970_e58195_d_n8;
        var_eta_p_rv = 0.0;

        let (assign44980_e58208, assign44980_e58208_d_n5, assign44980_e58208_d_n6, assign44980_e58208_d_n7, assign44980_e58208_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign44980_e58205: f64 = (var_eta_p + 1.0);
        let assign44980_e58206: f64 = (var_eta_p / assign44980_e58205);
        (assign44980_e58206, (((var_eta_p_dn5 * assign44980_e58205) - (var_eta_p * var_eta_p_dn5)) / (assign44980_e58205 * assign44980_e58205)), (((var_eta_p_dn6 * assign44980_e58205) - (var_eta_p * var_eta_p_dn6)) / (assign44980_e58205 * assign44980_e58205)), (((var_eta_p_dn7 * assign44980_e58205) - (var_eta_p * var_eta_p_dn7)) / (assign44980_e58205 * assign44980_e58205)), (((var_eta_p_dn8 * assign44980_e58205) - (var_eta_p * var_eta_p_dn8)) / (assign44980_e58205 * assign44980_e58205)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign44980_e58208;
        var_temp__blk936_dn5 = assign44980_e58208_d_n5;
        var_temp__blk936_dn6 = assign44980_e58208_d_n6;
        var_temp__blk936_dn7 = assign44980_e58208_d_n7;
        var_temp__blk936_dn8 = assign44980_e58208_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign44990_e58225, assign44990_e58225_d_n5, assign44990_e58225_d_n6, assign44990_e58225_d_n7, assign44990_e58225_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign44990_e58218: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign44990_e58220: f64 = (assign44990_e58218 * var_gf2);
        let assign44990_e58222: f64 = (assign44990_e58220 * var_dm);
        let assign44990_e58223: f64 = (var_kp * assign44990_e58222);
        (assign44990_e58223, (var_kp * ((((((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) * var_gf2) + (assign44990_e58218 * var_gf2_dn5)) * var_dm) + (assign44990_e58220 * var_dm_dn5))), (var_kp * ((((((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) * var_gf2) + (assign44990_e58218 * var_gf2_dn6)) * var_dm) + (assign44990_e58220 * var_dm_dn6))), (var_kp * ((((((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) * var_gf2) + (assign44990_e58218 * var_gf2_dn7)) * var_dm) + (assign44990_e58220 * var_dm_dn7))), (var_kp * ((((((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) * var_gf2) + (assign44990_e58218 * var_gf2_dn8)) * var_dm) + (assign44990_e58220 * var_dm_dn8))),)
    } else {
        (var_x_pm, var_x_pm_dn5, var_x_pm_dn6, var_x_pm_dn7, var_x_pm_dn8,)
    }
};
        var_x_pm = assign44990_e58225;
        var_x_pm_dn5 = assign44990_e58225_d_n5;
        var_x_pm_dn6 = assign44990_e58225_d_n6;
        var_x_pm_dn7 = assign44990_e58225_d_n7;
        var_x_pm_dn8 = assign44990_e58225_d_n8;
        var_x_pm_rv = 0.0;

        let (assign45000_e58246, assign45000_e58246_d_n5, assign45000_e58246_d_n6, assign45000_e58246_d_n7, assign45000_e58246_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45000_e58235: f64 = (var_xgm - var_x_pm);
        let assign45000_e58236: f64 = (2.0 * assign45000_e58235);
        let assign45000_e58240: f64 = (1.0 - var_em);
        let assign45000_e58242: f64 = (assign45000_e58240 + var_dm);
        let assign45000_e58243: f64 = (var_gf2 * assign45000_e58242);
        let assign45000_e58244: f64 = (assign45000_e58236 + assign45000_e58243);
        (assign45000_e58244, ((2.0 * (var_xgm_dn5 - var_x_pm_dn5)) + ((var_gf2_dn5 * assign45000_e58242) + (var_gf2 * ((-var_em_dn5) + var_dm_dn5)))), ((2.0 * (var_xgm_dn6 - var_x_pm_dn6)) + ((var_gf2_dn6 * assign45000_e58242) + (var_gf2 * ((-var_em_dn6) + var_dm_dn6)))), ((2.0 * (var_xgm_dn7 - var_x_pm_dn7)) + ((var_gf2_dn7 * assign45000_e58242) + (var_gf2 * ((-var_em_dn7) + var_dm_dn7)))), ((2.0 * (var_xgm_dn8 - var_x_pm_dn8)) + ((var_gf2_dn8 * assign45000_e58242) + (var_gf2 * ((-var_em_dn8) + var_dm_dn8)))),)
    } else {
        (var_p_pd, var_p_pd_dn5, var_p_pd_dn6, var_p_pd_dn7, var_p_pd_dn8,)
    }
};
        var_p_pd = assign45000_e58246;
        var_p_pd_dn5 = assign45000_e58246_d_n5;
        var_p_pd_dn6 = assign45000_e58246_d_n6;
        var_p_pd_dn7 = assign45000_e58246_d_n7;
        var_p_pd_dn8 = assign45000_e58246_d_n8;
        var_p_pd_rv = 0.0;

        let (assign45010_e58261, assign45010_e58261_d_n5, assign45010_e58261_d_n6, assign45010_e58261_d_n7, assign45010_e58261_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45010_e58257: f64 = (2.0 * var_xgm);
        let assign45010_e58258: f64 = (var_x_pm - assign45010_e58257);
        let assign45010_e58259: f64 = (var_x_pm * assign45010_e58258);
        (assign45010_e58259, ((var_x_pm_dn5 * assign45010_e58258) + (var_x_pm * (var_x_pm_dn5 - (2.0 * var_xgm_dn5)))), ((var_x_pm_dn6 * assign45010_e58258) + (var_x_pm * (var_x_pm_dn6 - (2.0 * var_xgm_dn6)))), ((var_x_pm_dn7 * assign45010_e58258) + (var_x_pm * (var_x_pm_dn7 - (2.0 * var_xgm_dn7)))), ((var_x_pm_dn8 * assign45010_e58258) + (var_x_pm * (var_x_pm_dn8 - (2.0 * var_xgm_dn8)))),)
    } else {
        (var_q_pd, var_q_pd_dn5, var_q_pd_dn6, var_q_pd_dn7, var_q_pd_dn8,)
    }
};
        var_q_pd = assign45010_e58261;
        var_q_pd_dn5 = assign45010_e58261_d_n5;
        var_q_pd_dn6 = assign45010_e58261_d_n6;
        var_q_pd_dn7 = assign45010_e58261_d_n7;
        var_q_pd_dn8 = assign45010_e58261_d_n8;
        var_q_pd_rv = 0.0;

        let (assign45020_e58278, assign45020_e58278_d_n5, assign45020_e58278_d_n6, assign45020_e58278_d_n7, assign45020_e58278_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45020_e58273: f64 = (var_em + var_dm);
        let assign45020_e58274: f64 = (var_gf2 * assign45020_e58273);
        let assign45020_e58275: f64 = (0.5 * assign45020_e58274);
        let assign45020_e58276: f64 = (1.0 - assign45020_e58275);
        (assign45020_e58276, (-(0.5 * ((var_gf2_dn5 * assign45020_e58273) + (var_gf2 * (var_em_dn5 + var_dm_dn5))))), (-(0.5 * ((var_gf2_dn6 * assign45020_e58273) + (var_gf2 * (var_em_dn6 + var_dm_dn6))))), (-(0.5 * ((var_gf2_dn7 * assign45020_e58273) + (var_gf2 * (var_em_dn7 + var_dm_dn7))))), (-(0.5 * ((var_gf2_dn8 * assign45020_e58273) + (var_gf2 * (var_em_dn8 + var_dm_dn8))))),)
    } else {
        (var_xi_pd, var_xi_pd_dn5, var_xi_pd_dn6, var_xi_pd_dn7, var_xi_pd_dn8,)
    }
};
        var_xi_pd = assign45020_e58278;
        var_xi_pd_dn5 = assign45020_e58278_d_n5;
        var_xi_pd_dn6 = assign45020_e58278_d_n6;
        var_xi_pd_dn7 = assign45020_e58278_d_n7;
        var_xi_pd_dn8 = assign45020_e58278_d_n8;
        var_xi_pd_rv = 0.0;

        let (assign45030_e58297, assign45030_e58297_d_n5, assign45030_e58297_d_n6, assign45030_e58297_d_n7, assign45030_e58297_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45030_e58287: f64 = (var_q_pd * var_p_pd);
        let assign45030_e58290: f64 = (var_p_pd * var_p_pd);
        let assign45030_e58293: f64 = (var_xi_pd * var_q_pd);
        let assign45030_e58294: f64 = (assign45030_e58290 - assign45030_e58293);
        let assign45030_e58295: f64 = (assign45030_e58287 / assign45030_e58294);
        (assign45030_e58295, (((((var_q_pd_dn5 * var_p_pd) + (var_q_pd * var_p_pd_dn5)) * assign45030_e58294) - (assign45030_e58287 * (((var_p_pd_dn5 * var_p_pd) + (var_p_pd * var_p_pd_dn5)) - ((var_xi_pd_dn5 * var_q_pd) + (var_xi_pd * var_q_pd_dn5))))) / (assign45030_e58294 * assign45030_e58294)), (((((var_q_pd_dn6 * var_p_pd) + (var_q_pd * var_p_pd_dn6)) * assign45030_e58294) - (assign45030_e58287 * (((var_p_pd_dn6 * var_p_pd) + (var_p_pd * var_p_pd_dn6)) - ((var_xi_pd_dn6 * var_q_pd) + (var_xi_pd * var_q_pd_dn6))))) / (assign45030_e58294 * assign45030_e58294)), (((((var_q_pd_dn7 * var_p_pd) + (var_q_pd * var_p_pd_dn7)) * assign45030_e58294) - (assign45030_e58287 * (((var_p_pd_dn7 * var_p_pd) + (var_p_pd * var_p_pd_dn7)) - ((var_xi_pd_dn7 * var_q_pd) + (var_xi_pd * var_q_pd_dn7))))) / (assign45030_e58294 * assign45030_e58294)), (((((var_q_pd_dn8 * var_p_pd) + (var_q_pd * var_p_pd_dn8)) * assign45030_e58294) - (assign45030_e58287 * (((var_p_pd_dn8 * var_p_pd) + (var_p_pd * var_p_pd_dn8)) - ((var_xi_pd_dn8 * var_q_pd) + (var_xi_pd * var_q_pd_dn8))))) / (assign45030_e58294 * assign45030_e58294)),)
    } else {
        (var_u_pd, var_u_pd_dn5, var_u_pd_dn6, var_u_pd_dn7, var_u_pd_dn8,)
    }
};
        var_u_pd = assign45030_e58297;
        var_u_pd_dn5 = assign45030_e58297_d_n5;
        var_u_pd_dn6 = assign45030_e58297_d_n6;
        var_u_pd_dn7 = assign45030_e58297_d_n7;
        var_u_pd_dn8 = assign45030_e58297_d_n8;
        var_u_pd_rv = 0.0;

        let (assign45040_e58308, assign45040_e58308_d_n5, assign45040_e58308_d_n6, assign45040_e58308_d_n7, assign45040_e58308_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45040_e58306: f64 = (var_x_m + var_u_pd);
        (assign45040_e58306, (var_x_m_dn5 + var_u_pd_dn5), (var_x_m_dn6 + var_u_pd_dn6), (var_x_m_dn7 + var_u_pd_dn7), (var_x_m_dn8 + var_u_pd_dn8),)
    } else {
        (var_x_m, var_x_m_dn5, var_x_m_dn6, var_x_m_dn7, var_x_m_dn8,)
    }
};
        var_x_m = assign45040_e58308;
        var_x_m_dn5 = assign45040_e58308_d_n5;
        var_x_m_dn6 = assign45040_e58308_d_n6;
        var_x_m_dn7 = assign45040_e58308_d_n7;
        var_x_m_dn8 = assign45040_e58308_d_n8;
        var_x_m_rv = 0.0;

        let (assign45050_e58318, assign45050_e58318_d_n5, assign45050_e58318_d_n6, assign45050_e58318_d_n7, assign45050_e58318_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45050_e58316: f64 = (var_u_pd).exp();
        (assign45050_e58316, (assign45050_e58316 * var_u_pd_dn5), (assign45050_e58316 * var_u_pd_dn6), (assign45050_e58316 * var_u_pd_dn7), (assign45050_e58316 * var_u_pd_dn8),)
    } else {
        (var_km, var_km_dn5, var_km_dn6, var_km_dn7, var_km_dn8,)
    }
};
        var_km = assign45050_e58318;
        var_km_dn5 = assign45050_e58318_d_n5;
        var_km_dn6 = assign45050_e58318_d_n6;
        var_km_dn7 = assign45050_e58318_d_n7;
        var_km_dn8 = assign45050_e58318_d_n8;
        var_km_rv = 0.0;

        let (assign45060_e58329, assign45060_e58329_d_n5, assign45060_e58329_d_n6, assign45060_e58329_d_n7, assign45060_e58329_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45060_e58327: f64 = (var_em / var_km);
        (assign45060_e58327, (((var_em_dn5 * var_km) - (var_em * var_km_dn5)) / (var_km * var_km)), (((var_em_dn6 * var_km) - (var_em * var_km_dn6)) / (var_km * var_km)), (((var_em_dn7 * var_km) - (var_em * var_km_dn7)) / (var_km * var_km)), (((var_em_dn8 * var_km) - (var_em * var_km_dn8)) / (var_km * var_km)),)
    } else {
        (var_em, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8,)
    }
};
        var_em = assign45060_e58329;
        var_em_dn5 = assign45060_e58329_d_n5;
        var_em_dn6 = assign45060_e58329_d_n6;
        var_em_dn7 = assign45060_e58329_d_n7;
        var_em_dn8 = assign45060_e58329_d_n8;
        var_em_rv = 0.0;

        let (assign45070_e58340, assign45070_e58340_d_n5, assign45070_e58340_d_n6, assign45070_e58340_d_n7, assign45070_e58340_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45070_e58338: f64 = (var_dm * var_km);
        (assign45070_e58338, ((var_dm_dn5 * var_km) + (var_dm * var_km_dn5)), ((var_dm_dn6 * var_km) + (var_dm * var_km_dn6)), ((var_dm_dn7 * var_km) + (var_dm * var_km_dn7)), ((var_dm_dn8 * var_km) + (var_dm * var_km_dn8)),)
    } else {
        (var_dm, var_dm_dn5, var_dm_dn6, var_dm_dn7, var_dm_dn8,)
    }
};
        var_dm = assign45070_e58340;
        var_dm_dn5 = assign45070_e58340_d_n5;
        var_dm_dn6 = assign45070_e58340_d_n6;
        var_dm_dn7 = assign45070_e58340_d_n7;
        var_dm_dn8 = assign45070_e58340_d_n8;
        var_dm_rv = 0.0;

        let (assign45080_e58353, assign45080_e58353_d_n5, assign45080_e58353_d_n6, assign45080_e58353_d_n7, assign45080_e58353_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45080_e58349: f64 = (var_x_m - 1.0);
        let assign45080_e58351: f64 = (assign45080_e58349 + var_em);
        (assign45080_e58351, (var_x_m_dn5 + var_em_dn5), (var_x_m_dn6 + var_em_dn6), (var_x_m_dn7 + var_em_dn7), (var_x_m_dn8 + var_em_dn8),)
    } else {
        (var_pm, var_pm_dn5, var_pm_dn6, var_pm_dn7, var_pm_dn8,)
    }
};
        var_pm = assign45080_e58353;
        var_pm_dn5 = assign45080_e58353_d_n5;
        var_pm_dn6 = assign45080_e58353_d_n6;
        var_pm_dn7 = assign45080_e58353_d_n7;
        var_pm_dn8 = assign45080_e58353_d_n8;
        var_pm_rv = 0.0;

        let (assign45090_e58367, assign45090_e58367_d_n5, assign45090_e58367_d_n6, assign45090_e58367_d_n7, assign45090_e58367_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45090_e58363: f64 = (var_dm + var_pm);
        let assign45090_e58364: f64 = (assign45090_e58363).sqrt();
        let assign45090_e58365: f64 = (var_gf * assign45090_e58364);
        (assign45090_e58365, ((var_gf_dn5 * assign45090_e58364) + (var_gf * ((var_dm_dn5 + var_pm_dn5) / (2.0 * assign45090_e58364)))), ((var_gf_dn6 * assign45090_e58364) + (var_gf * ((var_dm_dn6 + var_pm_dn6) / (2.0 * assign45090_e58364)))), ((var_gf_dn7 * assign45090_e58364) + (var_gf * ((var_dm_dn7 + var_pm_dn7) / (2.0 * assign45090_e58364)))), ((var_gf_dn8 * assign45090_e58364) + (var_gf * ((var_dm_dn8 + var_pm_dn8) / (2.0 * assign45090_e58364)))),)
    } else {
        (var_xgm, var_xgm_dn5, var_xgm_dn6, var_xgm_dn7, var_xgm_dn8,)
    }
};
        var_xgm = assign45090_e58367;
        var_xgm_dn5 = assign45090_e58367_d_n5;
        var_xgm_dn6 = assign45090_e58367_d_n6;
        var_xgm_dn7 = assign45090_e58367_d_n7;
        var_xgm_dn8 = assign45090_e58367_d_n8;
        var_xgm_rv = 0.0;

        let (assign45100_e58386, assign45100_e58386_d_n5, assign45100_e58386_d_n6, assign45100_e58386_d_n7, assign45100_e58386_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45100_e58376: f64 = (1.0 - var_em);
        let assign45100_e58380: f64 = (var_xgm * var_eta_p);
        let assign45100_e58382: f64 = (assign45100_e58380 * var_inv_gf2);
        let assign45100_e58383: f64 = (2.0 * assign45100_e58382);
        let assign45100_e58384: f64 = (assign45100_e58376 + assign45100_e58383);
        (assign45100_e58384, ((-var_em_dn5) + (2.0 * ((((var_xgm_dn5 * var_eta_p) + (var_xgm * var_eta_p_dn5)) * var_inv_gf2) + (assign45100_e58380 * var_inv_gf2_dn5)))), ((-var_em_dn6) + (2.0 * ((((var_xgm_dn6 * var_eta_p) + (var_xgm * var_eta_p_dn6)) * var_inv_gf2) + (assign45100_e58380 * var_inv_gf2_dn6)))), ((-var_em_dn7) + (2.0 * ((((var_xgm_dn7 * var_eta_p) + (var_xgm * var_eta_p_dn7)) * var_inv_gf2) + (assign45100_e58380 * var_inv_gf2_dn7)))), ((-var_em_dn8) + (2.0 * ((((var_xgm_dn8 * var_eta_p) + (var_xgm * var_eta_p_dn8)) * var_inv_gf2) + (assign45100_e58380 * var_inv_gf2_dn8)))),)
    } else {
        (var_km0, var_km0_dn5, var_km0_dn6, var_km0_dn7, var_km0_dn8,)
    }
};
        var_km0 = assign45100_e58386;
        var_km0_dn5 = assign45100_e58386_d_n5;
        var_km0_dn6 = assign45100_e58386_d_n6;
        var_km0_dn7 = assign45100_e58386_d_n7;
        var_km0_dn8 = assign45100_e58386_d_n8;
        var_km0_rv = 0.0;

        let (assign45110_e58407, assign45110_e58407_d_n5, assign45110_e58407_d_n6, assign45110_e58407_d_n7, assign45110_e58407_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45110_e58395: f64 = (var_x_ds * var_km);
        let assign45110_e58398: f64 = (var_d0 + var_d_bar);
        let assign45110_e58399: f64 = (assign45110_e58395 * assign45110_e58398);
        let assign45110_e58403: f64 = (var_km * var_d_bar);
        let assign45110_e58404: f64 = (var_km0 + assign45110_e58403);
        let assign45110_e58405: f64 = (assign45110_e58399 / assign45110_e58404);
        (assign45110_e58405, (((((((var_x_ds_dn5 * var_km) + (var_x_ds * var_km_dn5)) * assign45110_e58398) + (assign45110_e58395 * (var_d0_dn5 + var_d_bar_dn5))) * assign45110_e58404) - (assign45110_e58399 * (var_km0_dn5 + ((var_km_dn5 * var_d_bar) + (var_km * var_d_bar_dn5))))) / (assign45110_e58404 * assign45110_e58404)), (((((((var_x_ds_dn6 * var_km) + (var_x_ds * var_km_dn6)) * assign45110_e58398) + (assign45110_e58395 * (var_d0_dn6 + var_d_bar_dn6))) * assign45110_e58404) - (assign45110_e58399 * (var_km0_dn6 + ((var_km_dn6 * var_d_bar) + (var_km * var_d_bar_dn6))))) / (assign45110_e58404 * assign45110_e58404)), (((((((var_x_ds_dn7 * var_km) + (var_x_ds * var_km_dn7)) * assign45110_e58398) + (assign45110_e58395 * (var_d0_dn7 + var_d_bar_dn7))) * assign45110_e58404) - (assign45110_e58399 * (var_km0_dn7 + ((var_km_dn7 * var_d_bar) + (var_km * var_d_bar_dn7))))) / (assign45110_e58404 * assign45110_e58404)), (((((((var_x_ds_dn8 * var_km) + (var_x_ds * var_km_dn8)) * assign45110_e58398) + (assign45110_e58395 * (var_d0_dn8 + var_d_bar_dn8))) * assign45110_e58404) - (assign45110_e58399 * (var_km0_dn8 + ((var_km_dn8 * var_d_bar) + (var_km * var_d_bar_dn8))))) / (assign45110_e58404 * assign45110_e58404)),)
    } else {
        (var_x_ds, var_x_ds_dn5, var_x_ds_dn6, var_x_ds_dn7, var_x_ds_dn8,)
    }
};
        var_x_ds = assign45110_e58407;
        var_x_ds_dn5 = assign45110_e58407_d_n5;
        var_x_ds_dn6 = assign45110_e58407_d_n6;
        var_x_ds_dn7 = assign45110_e58407_d_n7;
        var_x_ds_dn8 = assign45110_e58407_d_n8;
        var_x_ds_rv = 0.0;

        let (assign45120_e58418, assign45120_e58418_d_n5, assign45120_e58418_d_n6, assign45120_e58418_d_n7, assign45120_e58418_d_n8,) = {
    if (((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) && (var_guard1215 != 0.0)) {
        let assign45120_e58416: f64 = (var_x_ds * var_phit1);
        (assign45120_e58416, ((var_x_ds_dn5 * var_phit1) + (var_x_ds * var_phit1_dn5)), ((var_x_ds_dn6 * var_phit1) + (var_x_ds * var_phit1_dn6)), ((var_x_ds_dn7 * var_phit1) + (var_x_ds * var_phit1_dn7)), ((var_x_ds_dn8 * var_phit1) + (var_x_ds * var_phit1_dn8)),)
    } else {
        (var_dps, var_dps_dn5, var_dps_dn6, var_dps_dn7, var_dps_dn8,)
    }
};
        var_dps = assign45120_e58418;
        var_dps_dn5 = assign45120_e58418_d_n5;
        var_dps_dn6 = assign45120_e58418_d_n6;
        var_dps_dn7 = assign45120_e58418_d_n7;
        var_dps_dn8 = assign45120_e58418_d_n8;
        var_dps_rv = 0.0;

        let (assign45130_e58426, assign45130_e58426_d_n5, assign45130_e58426_d_n6, assign45130_e58426_d_n7, assign45130_e58426_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) {
        let assign45130_e58424: f64 = (var_pm).sqrt();
        (assign45130_e58424, (var_pm_dn5 / (2.0 * assign45130_e58424)), (var_pm_dn6 / (2.0 * assign45130_e58424)), (var_pm_dn7 / (2.0 * assign45130_e58424)), (var_pm_dn8 / (2.0 * assign45130_e58424)),)
    } else {
        (var_sqm, var_sqm_dn5, var_sqm_dn6, var_sqm_dn7, var_sqm_dn8,)
    }
};
        var_sqm = assign45130_e58426;
        var_sqm_dn5 = assign45130_e58426_d_n5;
        var_sqm_dn6 = assign45130_e58426_d_n6;
        var_sqm_dn7 = assign45130_e58426_d_n7;
        var_sqm_dn8 = assign45130_e58426_d_n8;
        var_sqm_rv = 0.0;

        let (assign45140_e58443, assign45140_e58443_d_n5, assign45140_e58443_d_n6, assign45140_e58443_d_n7, assign45140_e58443_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1213 == 0.0)) {
        let assign45140_e58436: f64 = (1.0 - var_em);
        let assign45140_e58437: f64 = (var_gf * assign45140_e58436);
        let assign45140_e58439: f64 = (assign45140_e58437 / var_sqm);
        let assign45140_e58440: f64 = (0.5 * assign45140_e58439);
        let assign45140_e58441: f64 = (var_eta_p + assign45140_e58440);
        (assign45140_e58441, (var_eta_p_dn5 + (0.5 * (((((var_gf_dn5 * assign45140_e58436) + (var_gf * (-var_em_dn5))) * var_sqm) - (assign45140_e58437 * var_sqm_dn5)) / (var_sqm * var_sqm)))), (var_eta_p_dn6 + (0.5 * (((((var_gf_dn6 * assign45140_e58436) + (var_gf * (-var_em_dn6))) * var_sqm) - (assign45140_e58437 * var_sqm_dn6)) / (var_sqm * var_sqm)))), (var_eta_p_dn7 + (0.5 * (((((var_gf_dn7 * assign45140_e58436) + (var_gf * (-var_em_dn7))) * var_sqm) - (assign45140_e58437 * var_sqm_dn7)) / (var_sqm * var_sqm)))), (var_eta_p_dn8 + (0.5 * (((((var_gf_dn8 * assign45140_e58436) + (var_gf * (-var_em_dn8))) * var_sqm) - (assign45140_e58437 * var_sqm_dn8)) / (var_sqm * var_sqm)))),)
    } else {
        (var_alpha, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8,)
    }
};
        var_alpha = assign45140_e58443;
        var_alpha_dn5 = assign45140_e58443_d_n5;
        var_alpha_dn6 = assign45140_e58443_d_n6;
        var_alpha_dn7 = assign45140_e58443_d_n7;
        var_alpha_dn8 = assign45140_e58443_d_n8;
        var_alpha_rv = 0.0;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn5_slot = var_alpha_dn5;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alpha_rv_slot = var_alpha_rv;
        *var_d0_slot = var_d0;
        *var_d0_dn5_slot = var_d0_dn5;
        *var_d0_dn6_slot = var_d0_dn6;
        *var_d0_dn7_slot = var_d0_dn7;
        *var_d0_dn8_slot = var_d0_dn8;
        *var_d0_rv_slot = var_d0_rv;
        *var_dm_slot = var_dm;
        *var_dm_dn5_slot = var_dm_dn5;
        *var_dm_dn6_slot = var_dm_dn6;
        *var_dm_dn7_slot = var_dm_dn7;
        *var_dm_dn8_slot = var_dm_dn8;
        *var_dm_rv_slot = var_dm_rv;
        *var_dps_slot = var_dps;
        *var_dps_dn5_slot = var_dps_dn5;
        *var_dps_dn6_slot = var_dps_dn6;
        *var_dps_dn7_slot = var_dps_dn7;
        *var_dps_dn8_slot = var_dps_dn8;
        *var_dps_rv_slot = var_dps_rv;
        *var_em_slot = var_em;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_rv_slot = var_em_rv;
        *var_eta_p_slot = var_eta_p;
        *var_eta_p_dn5_slot = var_eta_p_dn5;
        *var_eta_p_dn6_slot = var_eta_p_dn6;
        *var_eta_p_dn7_slot = var_eta_p_dn7;
        *var_eta_p_dn8_slot = var_eta_p_dn8;
        *var_eta_p_rv_slot = var_eta_p_rv;
        *var_guard1213_slot = var_guard1213;
        *var_guard1213_rv_slot = var_guard1213_rv;
        *var_guard1214_slot = var_guard1214;
        *var_guard1214_rv_slot = var_guard1214_rv;
        *var_guard1215_slot = var_guard1215;
        *var_guard1215_rv_slot = var_guard1215_rv;
        *var_km_slot = var_km;
        *var_km0_slot = var_km0;
        *var_km0_dn5_slot = var_km0_dn5;
        *var_km0_dn6_slot = var_km0_dn6;
        *var_km0_dn7_slot = var_km0_dn7;
        *var_km0_dn8_slot = var_km0_dn8;
        *var_km0_rv_slot = var_km0_rv;
        *var_km_dn5_slot = var_km_dn5;
        *var_km_dn6_slot = var_km_dn6;
        *var_km_dn7_slot = var_km_dn7;
        *var_km_dn8_slot = var_km_dn8;
        *var_km_rv_slot = var_km_rv;
        *var_p_pd_slot = var_p_pd;
        *var_p_pd_dn5_slot = var_p_pd_dn5;
        *var_p_pd_dn6_slot = var_p_pd_dn6;
        *var_p_pd_dn7_slot = var_p_pd_dn7;
        *var_p_pd_dn8_slot = var_p_pd_dn8;
        *var_p_pd_rv_slot = var_p_pd_rv;
        *var_pm_slot = var_pm;
        *var_pm_dn5_slot = var_pm_dn5;
        *var_pm_dn6_slot = var_pm_dn6;
        *var_pm_dn7_slot = var_pm_dn7;
        *var_pm_dn8_slot = var_pm_dn8;
        *var_pm_rv_slot = var_pm_rv;
        *var_q_pd_slot = var_q_pd;
        *var_q_pd_dn5_slot = var_q_pd_dn5;
        *var_q_pd_dn6_slot = var_q_pd_dn6;
        *var_q_pd_dn7_slot = var_q_pd_dn7;
        *var_q_pd_dn8_slot = var_q_pd_dn8;
        *var_q_pd_rv_slot = var_q_pd_rv;
        *var_sqm_slot = var_sqm;
        *var_sqm_dn5_slot = var_sqm_dn5;
        *var_sqm_dn6_slot = var_sqm_dn6;
        *var_sqm_dn7_slot = var_sqm_dn7;
        *var_sqm_dn8_slot = var_sqm_dn8;
        *var_sqm_rv_slot = var_sqm_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_u_pd_slot = var_u_pd;
        *var_u_pd_dn5_slot = var_u_pd_dn5;
        *var_u_pd_dn6_slot = var_u_pd_dn6;
        *var_u_pd_dn7_slot = var_u_pd_dn7;
        *var_u_pd_dn8_slot = var_u_pd_dn8;
        *var_u_pd_rv_slot = var_u_pd_rv;
        *var_x_ds_slot = var_x_ds;
        *var_x_ds_dn5_slot = var_x_ds_dn5;
        *var_x_ds_dn6_slot = var_x_ds_dn6;
        *var_x_ds_dn7_slot = var_x_ds_dn7;
        *var_x_ds_dn8_slot = var_x_ds_dn8;
        *var_x_ds_rv_slot = var_x_ds_rv;
        *var_x_m_slot = var_x_m;
        *var_x_m_dn5_slot = var_x_m_dn5;
        *var_x_m_dn6_slot = var_x_m_dn6;
        *var_x_m_dn7_slot = var_x_m_dn7;
        *var_x_m_dn8_slot = var_x_m_dn8;
        *var_x_m_rv_slot = var_x_m_rv;
        *var_x_pm_slot = var_x_pm;
        *var_x_pm_dn5_slot = var_x_pm_dn5;
        *var_x_pm_dn6_slot = var_x_pm_dn6;
        *var_x_pm_dn7_slot = var_x_pm_dn7;
        *var_x_pm_dn8_slot = var_x_pm_dn8;
        *var_x_pm_rv_slot = var_x_pm_rv;
        *var_xgm_slot = var_xgm;
        *var_xgm_dn5_slot = var_xgm_dn5;
        *var_xgm_dn6_slot = var_xgm_dn6;
        *var_xgm_dn7_slot = var_xgm_dn7;
        *var_xgm_dn8_slot = var_xgm_dn8;
        *var_xgm_rv_slot = var_xgm_rv;
        *var_xi_pd_slot = var_xi_pd;
        *var_xi_pd_dn5_slot = var_xi_pd_dn5;
        *var_xi_pd_dn6_slot = var_xi_pd_dn6;
        *var_xi_pd_dn7_slot = var_xi_pd_dn7;
        *var_xi_pd_dn8_slot = var_xi_pd_dn8;
        *var_xi_pd_rv_slot = var_xi_pd_rv;
    }

    pub(super) fn stamp_reactive_block_36(
        var_alpha: f64,
        var_alpha_dn5: f64,
        var_alpha_dn6: f64,
        var_alpha_dn7: f64,
        var_alpha_dn8: f64,
        var_cs_t: f64,
        var_dm: f64,
        var_dm_dn5: f64,
        var_dm_dn6: f64,
        var_dm_dn7: f64,
        var_dm_dn8: f64,
        var_dps: f64,
        var_dps_dn5: f64,
        var_dps_dn6: f64,
        var_dps_dn7: f64,
        var_dps_dn8: f64,
        var_e_eff0: f64,
        var_eta_mu: f64,
        var_eta_mu1: f64,
        var_eta_p: f64,
        var_eta_p_dn5: f64,
        var_eta_p_dn6: f64,
        var_eta_p_dn7: f64,
        var_eta_p_dn8: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn5: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf_dn5: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_guard1197: f64,
        var_inv_vp: f64,
        var_mue_t: f64,
        var_phit1: f64,
        var_phit1_dn5: f64,
        var_phit1_dn6: f64,
        var_phit1_dn7: f64,
        var_phit1_dn8: f64,
        var_pm: f64,
        var_pm_dn5: f64,
        var_pm_dn6: f64,
        var_pm_dn7: f64,
        var_pm_dn8: f64,
        var_qbd: f64,
        var_qbd_dn5: f64,
        var_qbd_dn6: f64,
        var_qbd_dn7: f64,
        var_qbd_dn8: f64,
        var_rhob: f64,
        var_rhob_dn5: f64,
        var_rhob_dn6: f64,
        var_rhob_dn7: f64,
        var_rhob_dn8: f64,
        var_rsg_i: f64,
        var_rxcor: f64,
        var_rxcor_dn5: f64,
        var_rxcor_dn6: f64,
        var_rxcor_dn7: f64,
        var_rxcor_dn8: f64,
        var_sqm: f64,
        var_sqm_dn5: f64,
        var_sqm_dn6: f64,
        var_sqm_dn7: f64,
        var_sqm_dn8: f64,
        var_thecs_t: f64,
        var_themu_t: f64,
        var_ther_i: f64,
        var_thesatg_i: f64,
        var_thesatloc: f64,
        var_thesatt_i: f64,
        var_udse: f64,
        var_udse_dn5: f64,
        var_udse_dn6: f64,
        var_udse_dn7: f64,
        var_udse_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsat_lim: f64,
        var_vdsat_lim_dn5: f64,
        var_vdsat_lim_dn6: f64,
        var_vdsat_lim_dn7: f64,
        var_vdsat_lim_dn8: f64,
        var_vdse: f64,
        var_vdse_dn5: f64,
        var_vdse_dn6: f64,
        var_vdse_dn7: f64,
        var_vdse_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_x_ds: f64,
        var_x_ds_dn5: f64,
        var_x_ds_dn6: f64,
        var_x_ds_dn7: f64,
        var_x_ds_dn8: f64,
        var_x_m: f64,
        var_x_m_dn5: f64,
        var_x_m_dn6: f64,
        var_x_m_dn7: f64,
        var_x_m_dn8: f64,
        var_xg_dc: f64,
        var_xgm: f64,
        var_xgm_dn5: f64,
        var_xgm_dn6: f64,
        var_xgm_dn7: f64,
        var_xgm_dn8: f64,
        var_xitsb: f64,
        var_xitsb_dn5: f64,
        var_xitsb_dn6: f64,
        var_xitsb_dn7: f64,
        var_xitsb_dn8: f64,
        var_alpha_dc_slot: &mut f64,
        var_alpha_dc_dn5_slot: &mut f64,
        var_alpha_dc_dn6_slot: &mut f64,
        var_alpha_dc_dn7_slot: &mut f64,
        var_alpha_dc_dn8_slot: &mut f64,
        var_alpha_dc_rv_slot: &mut f64,
        var_dps_dc_slot: &mut f64,
        var_dps_dc_dn5_slot: &mut f64,
        var_dps_dc_dn6_slot: &mut f64,
        var_dps_dc_dn7_slot: &mut f64,
        var_dps_dc_dn8_slot: &mut f64,
        var_dps_dc_rv_slot: &mut f64,
        var_eeffm_slot: &mut f64,
        var_eeffm_dn5_slot: &mut f64,
        var_eeffm_dn6_slot: &mut f64,
        var_eeffm_dn7_slot: &mut f64,
        var_eeffm_dn8_slot: &mut f64,
        var_eeffm_rv_slot: &mut f64,
        var_eta_p_dc_slot: &mut f64,
        var_eta_p_dc_dn5_slot: &mut f64,
        var_eta_p_dc_dn6_slot: &mut f64,
        var_eta_p_dc_dn7_slot: &mut f64,
        var_eta_p_dc_dn8_slot: &mut f64,
        var_eta_p_dc_rv_slot: &mut f64,
        var_factheta_slot: &mut f64,
        var_factheta_dn5_slot: &mut f64,
        var_factheta_dn6_slot: &mut f64,
        var_factheta_dn7_slot: &mut f64,
        var_factheta_dn8_slot: &mut f64,
        var_factheta_rv_slot: &mut f64,
        var_gdl_dc_slot: &mut f64,
        var_gdl_dc_dn5_slot: &mut f64,
        var_gdl_dc_dn6_slot: &mut f64,
        var_gdl_dc_dn7_slot: &mut f64,
        var_gdl_dc_dn8_slot: &mut f64,
        var_gdl_dc_rv_slot: &mut f64,
        var_gmob_slot: &mut f64,
        var_gmob_dc_slot: &mut f64,
        var_gmob_dc_dn5_slot: &mut f64,
        var_gmob_dc_dn6_slot: &mut f64,
        var_gmob_dc_dn7_slot: &mut f64,
        var_gmob_dc_dn8_slot: &mut f64,
        var_gmob_dc_rv_slot: &mut f64,
        var_gmob_dl_dc_slot: &mut f64,
        var_gmob_dl_dc_dn5_slot: &mut f64,
        var_gmob_dl_dc_dn6_slot: &mut f64,
        var_gmob_dl_dc_dn7_slot: &mut f64,
        var_gmob_dl_dc_dn8_slot: &mut f64,
        var_gmob_dl_dc_rv_slot: &mut f64,
        var_gmob_dn5_slot: &mut f64,
        var_gmob_dn6_slot: &mut f64,
        var_gmob_dn7_slot: &mut f64,
        var_gmob_dn8_slot: &mut f64,
        var_gmob_rv_slot: &mut f64,
        var_gr_slot: &mut f64,
        var_gr_dn5_slot: &mut f64,
        var_gr_dn6_slot: &mut f64,
        var_gr_dn7_slot: &mut f64,
        var_gr_dn8_slot: &mut f64,
        var_gr_rv_slot: &mut f64,
        var_guard1216_slot: &mut f64,
        var_guard1216_rv_slot: &mut f64,
        var_guard1217_slot: &mut f64,
        var_guard1217_rv_slot: &mut f64,
        var_guard1218_slot: &mut f64,
        var_guard1218_rv_slot: &mut f64,
        var_gvsatinv_dc_slot: &mut f64,
        var_gvsatinv_dc_dn5_slot: &mut f64,
        var_gvsatinv_dc_dn6_slot: &mut f64,
        var_gvsatinv_dc_dn7_slot: &mut f64,
        var_gvsatinv_dc_dn8_slot: &mut f64,
        var_gvsatinv_dc_rv_slot: &mut f64,
        var_h_dc_slot: &mut f64,
        var_h_dc_dn5_slot: &mut f64,
        var_h_dc_dn6_slot: &mut f64,
        var_h_dc_dn7_slot: &mut f64,
        var_h_dc_dn8_slot: &mut f64,
        var_h_dc_rv_slot: &mut f64,
        var_i_ds_slot: &mut f64,
        var_i_ds_dn5_slot: &mut f64,
        var_i_ds_dn6_slot: &mut f64,
        var_i_ds_dn7_slot: &mut f64,
        var_i_ds_dn8_slot: &mut f64,
        var_i_ds_rv_slot: &mut f64,
        var_mutmp_slot: &mut f64,
        var_mutmp_dn5_slot: &mut f64,
        var_mutmp_dn6_slot: &mut f64,
        var_mutmp_dn7_slot: &mut f64,
        var_mutmp_dn8_slot: &mut f64,
        var_mutmp_rv_slot: &mut f64,
        var_qbd_dc_slot: &mut f64,
        var_qbd_dc_dn5_slot: &mut f64,
        var_qbd_dc_dn6_slot: &mut f64,
        var_qbd_dc_dn7_slot: &mut f64,
        var_qbd_dc_dn8_slot: &mut f64,
        var_qbd_dc_rv_slot: &mut f64,
        var_qbm_slot: &mut f64,
        var_qbm_dc_slot: &mut f64,
        var_qbm_dc_dn5_slot: &mut f64,
        var_qbm_dc_dn6_slot: &mut f64,
        var_qbm_dc_dn7_slot: &mut f64,
        var_qbm_dc_dn8_slot: &mut f64,
        var_qbm_dc_rv_slot: &mut f64,
        var_qbm_dn5_slot: &mut f64,
        var_qbm_dn6_slot: &mut f64,
        var_qbm_dn7_slot: &mut f64,
        var_qbm_dn8_slot: &mut f64,
        var_qbm_rv_slot: &mut f64,
        var_qeff_slot: &mut f64,
        var_qeff1_slot: &mut f64,
        var_qeff1_dc_slot: &mut f64,
        var_qeff1_dc_dn5_slot: &mut f64,
        var_qeff1_dc_dn6_slot: &mut f64,
        var_qeff1_dc_dn7_slot: &mut f64,
        var_qeff1_dc_dn8_slot: &mut f64,
        var_qeff1_dc_rv_slot: &mut f64,
        var_qeff1_dn5_slot: &mut f64,
        var_qeff1_dn6_slot: &mut f64,
        var_qeff1_dn7_slot: &mut f64,
        var_qeff1_dn8_slot: &mut f64,
        var_qeff1_rv_slot: &mut f64,
        var_qeff_dn5_slot: &mut f64,
        var_qeff_dn6_slot: &mut f64,
        var_qeff_dn7_slot: &mut f64,
        var_qeff_dn8_slot: &mut f64,
        var_qeff_rv_slot: &mut f64,
        var_qim_slot: &mut f64,
        var_qim1_slot: &mut f64,
        var_qim1_dc_slot: &mut f64,
        var_qim1_dc_dn5_slot: &mut f64,
        var_qim1_dc_dn6_slot: &mut f64,
        var_qim1_dc_dn7_slot: &mut f64,
        var_qim1_dc_dn8_slot: &mut f64,
        var_qim1_dc_rv_slot: &mut f64,
        var_qim1_dn5_slot: &mut f64,
        var_qim1_dn6_slot: &mut f64,
        var_qim1_dn7_slot: &mut f64,
        var_qim1_dn8_slot: &mut f64,
        var_qim1_rv_slot: &mut f64,
        var_qim_dc_slot: &mut f64,
        var_qim_dc_dn5_slot: &mut f64,
        var_qim_dc_dn6_slot: &mut f64,
        var_qim_dc_dn7_slot: &mut f64,
        var_qim_dc_dn8_slot: &mut f64,
        var_qim_dc_rv_slot: &mut f64,
        var_qim_dn5_slot: &mut f64,
        var_qim_dn6_slot: &mut f64,
        var_qim_dn7_slot: &mut f64,
        var_qim_dn8_slot: &mut f64,
        var_qim_rv_slot: &mut f64,
        var_rhog_slot: &mut f64,
        var_rhog_dn5_slot: &mut f64,
        var_rhog_dn6_slot: &mut f64,
        var_rhog_dn7_slot: &mut f64,
        var_rhog_dn8_slot: &mut f64,
        var_rhog_rv_slot: &mut f64,
        var_s1_slot: &mut f64,
        var_s1_dc_slot: &mut f64,
        var_s1_dc_dn5_slot: &mut f64,
        var_s1_dc_dn6_slot: &mut f64,
        var_s1_dc_dn7_slot: &mut f64,
        var_s1_dc_dn8_slot: &mut f64,
        var_s1_dc_rv_slot: &mut f64,
        var_s1_dn5_slot: &mut f64,
        var_s1_dn6_slot: &mut f64,
        var_s1_dn7_slot: &mut f64,
        var_s1_dn8_slot: &mut f64,
        var_s1_rv_slot: &mut f64,
        var_s2_slot: &mut f64,
        var_s2_dn6_slot: &mut f64,
        var_s2_dn7_slot: &mut f64,
        var_s2_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_thesateff_slot: &mut f64,
        var_thesateff_dc_slot: &mut f64,
        var_thesateff_dc_dn5_slot: &mut f64,
        var_thesateff_dc_dn6_slot: &mut f64,
        var_thesateff_dc_dn7_slot: &mut f64,
        var_thesateff_dc_dn8_slot: &mut f64,
        var_thesateff_dc_rv_slot: &mut f64,
        var_thesateff_dn5_slot: &mut f64,
        var_thesateff_dn6_slot: &mut f64,
        var_thesateff_dn7_slot: &mut f64,
        var_thesateff_dn8_slot: &mut f64,
        var_thesateff_rv_slot: &mut f64,
        var_udse_dc_slot: &mut f64,
        var_udse_dc_dn5_slot: &mut f64,
        var_udse_dc_dn6_slot: &mut f64,
        var_udse_dc_dn7_slot: &mut f64,
        var_udse_dc_dn8_slot: &mut f64,
        var_udse_dc_rv_slot: &mut f64,
        var_vdsat_lim_dc_slot: &mut f64,
        var_vdsat_lim_dc_dn5_slot: &mut f64,
        var_vdsat_lim_dc_dn6_slot: &mut f64,
        var_vdsat_lim_dc_dn7_slot: &mut f64,
        var_vdsat_lim_dc_dn8_slot: &mut f64,
        var_vdsat_lim_dc_rv_slot: &mut f64,
        var_vdse_dc_slot: &mut f64,
        var_vdse_dc_dn5_slot: &mut f64,
        var_vdse_dc_dn6_slot: &mut f64,
        var_vdse_dc_dn7_slot: &mut f64,
        var_vdse_dc_dn8_slot: &mut f64,
        var_vdse_dc_rv_slot: &mut f64,
        var_voxm_slot: &mut f64,
        var_voxm_dc_slot: &mut f64,
        var_voxm_dc_dn5_slot: &mut f64,
        var_voxm_dc_dn6_slot: &mut f64,
        var_voxm_dc_dn7_slot: &mut f64,
        var_voxm_dc_dn8_slot: &mut f64,
        var_voxm_dc_rv_slot: &mut f64,
        var_voxm_dn5_slot: &mut f64,
        var_voxm_dn6_slot: &mut f64,
        var_voxm_dn7_slot: &mut f64,
        var_voxm_dn8_slot: &mut f64,
        var_voxm_rv_slot: &mut f64,
        var_wsat_slot: &mut f64,
        var_wsat_dn5_slot: &mut f64,
        var_wsat_dn6_slot: &mut f64,
        var_wsat_dn7_slot: &mut f64,
        var_wsat_dn8_slot: &mut f64,
        var_wsat_rv_slot: &mut f64,
        var_x_ds_dc_slot: &mut f64,
        var_x_ds_dc_dn5_slot: &mut f64,
        var_x_ds_dc_dn6_slot: &mut f64,
        var_x_ds_dc_dn7_slot: &mut f64,
        var_x_ds_dc_dn8_slot: &mut f64,
        var_x_ds_dc_rv_slot: &mut f64,
        var_x_m_dc_slot: &mut f64,
        var_x_m_dc_dn5_slot: &mut f64,
        var_x_m_dc_dn6_slot: &mut f64,
        var_x_m_dc_dn7_slot: &mut f64,
        var_x_m_dc_dn8_slot: &mut f64,
        var_x_m_dc_rv_slot: &mut f64,
    ) {
        let mut var_alpha_dc: f64 = *var_alpha_dc_slot;
        let mut var_alpha_dc_dn5: f64 = *var_alpha_dc_dn5_slot;
        let mut var_alpha_dc_dn6: f64 = *var_alpha_dc_dn6_slot;
        let mut var_alpha_dc_dn7: f64 = *var_alpha_dc_dn7_slot;
        let mut var_alpha_dc_dn8: f64 = *var_alpha_dc_dn8_slot;
        let mut var_alpha_dc_rv: f64 = *var_alpha_dc_rv_slot;
        let mut var_dps_dc: f64 = *var_dps_dc_slot;
        let mut var_dps_dc_dn5: f64 = *var_dps_dc_dn5_slot;
        let mut var_dps_dc_dn6: f64 = *var_dps_dc_dn6_slot;
        let mut var_dps_dc_dn7: f64 = *var_dps_dc_dn7_slot;
        let mut var_dps_dc_dn8: f64 = *var_dps_dc_dn8_slot;
        let mut var_dps_dc_rv: f64 = *var_dps_dc_rv_slot;
        let mut var_eeffm: f64 = *var_eeffm_slot;
        let mut var_eeffm_dn5: f64 = *var_eeffm_dn5_slot;
        let mut var_eeffm_dn6: f64 = *var_eeffm_dn6_slot;
        let mut var_eeffm_dn7: f64 = *var_eeffm_dn7_slot;
        let mut var_eeffm_dn8: f64 = *var_eeffm_dn8_slot;
        let mut var_eeffm_rv: f64 = *var_eeffm_rv_slot;
        let mut var_eta_p_dc: f64 = *var_eta_p_dc_slot;
        let mut var_eta_p_dc_dn5: f64 = *var_eta_p_dc_dn5_slot;
        let mut var_eta_p_dc_dn6: f64 = *var_eta_p_dc_dn6_slot;
        let mut var_eta_p_dc_dn7: f64 = *var_eta_p_dc_dn7_slot;
        let mut var_eta_p_dc_dn8: f64 = *var_eta_p_dc_dn8_slot;
        let mut var_eta_p_dc_rv: f64 = *var_eta_p_dc_rv_slot;
        let mut var_factheta: f64 = *var_factheta_slot;
        let mut var_factheta_dn5: f64 = *var_factheta_dn5_slot;
        let mut var_factheta_dn6: f64 = *var_factheta_dn6_slot;
        let mut var_factheta_dn7: f64 = *var_factheta_dn7_slot;
        let mut var_factheta_dn8: f64 = *var_factheta_dn8_slot;
        let mut var_factheta_rv: f64 = *var_factheta_rv_slot;
        let mut var_gdl_dc: f64 = *var_gdl_dc_slot;
        let mut var_gdl_dc_dn5: f64 = *var_gdl_dc_dn5_slot;
        let mut var_gdl_dc_dn6: f64 = *var_gdl_dc_dn6_slot;
        let mut var_gdl_dc_dn7: f64 = *var_gdl_dc_dn7_slot;
        let mut var_gdl_dc_dn8: f64 = *var_gdl_dc_dn8_slot;
        let mut var_gdl_dc_rv: f64 = *var_gdl_dc_rv_slot;
        let mut var_gmob: f64 = *var_gmob_slot;
        let mut var_gmob_dc: f64 = *var_gmob_dc_slot;
        let mut var_gmob_dc_dn5: f64 = *var_gmob_dc_dn5_slot;
        let mut var_gmob_dc_dn6: f64 = *var_gmob_dc_dn6_slot;
        let mut var_gmob_dc_dn7: f64 = *var_gmob_dc_dn7_slot;
        let mut var_gmob_dc_dn8: f64 = *var_gmob_dc_dn8_slot;
        let mut var_gmob_dc_rv: f64 = *var_gmob_dc_rv_slot;
        let mut var_gmob_dl_dc: f64 = *var_gmob_dl_dc_slot;
        let mut var_gmob_dl_dc_dn5: f64 = *var_gmob_dl_dc_dn5_slot;
        let mut var_gmob_dl_dc_dn6: f64 = *var_gmob_dl_dc_dn6_slot;
        let mut var_gmob_dl_dc_dn7: f64 = *var_gmob_dl_dc_dn7_slot;
        let mut var_gmob_dl_dc_dn8: f64 = *var_gmob_dl_dc_dn8_slot;
        let mut var_gmob_dl_dc_rv: f64 = *var_gmob_dl_dc_rv_slot;
        let mut var_gmob_dn5: f64 = *var_gmob_dn5_slot;
        let mut var_gmob_dn6: f64 = *var_gmob_dn6_slot;
        let mut var_gmob_dn7: f64 = *var_gmob_dn7_slot;
        let mut var_gmob_dn8: f64 = *var_gmob_dn8_slot;
        let mut var_gmob_rv: f64 = *var_gmob_rv_slot;
        let mut var_gr: f64 = *var_gr_slot;
        let mut var_gr_dn5: f64 = *var_gr_dn5_slot;
        let mut var_gr_dn6: f64 = *var_gr_dn6_slot;
        let mut var_gr_dn7: f64 = *var_gr_dn7_slot;
        let mut var_gr_dn8: f64 = *var_gr_dn8_slot;
        let mut var_gr_rv: f64 = *var_gr_rv_slot;
        let mut var_guard1216: f64 = *var_guard1216_slot;
        let mut var_guard1216_rv: f64 = *var_guard1216_rv_slot;
        let mut var_guard1217: f64 = *var_guard1217_slot;
        let mut var_guard1217_rv: f64 = *var_guard1217_rv_slot;
        let mut var_guard1218: f64 = *var_guard1218_slot;
        let mut var_guard1218_rv: f64 = *var_guard1218_rv_slot;
        let mut var_gvsatinv_dc: f64 = *var_gvsatinv_dc_slot;
        let mut var_gvsatinv_dc_dn5: f64 = *var_gvsatinv_dc_dn5_slot;
        let mut var_gvsatinv_dc_dn6: f64 = *var_gvsatinv_dc_dn6_slot;
        let mut var_gvsatinv_dc_dn7: f64 = *var_gvsatinv_dc_dn7_slot;
        let mut var_gvsatinv_dc_dn8: f64 = *var_gvsatinv_dc_dn8_slot;
        let mut var_gvsatinv_dc_rv: f64 = *var_gvsatinv_dc_rv_slot;
        let mut var_h_dc: f64 = *var_h_dc_slot;
        let mut var_h_dc_dn5: f64 = *var_h_dc_dn5_slot;
        let mut var_h_dc_dn6: f64 = *var_h_dc_dn6_slot;
        let mut var_h_dc_dn7: f64 = *var_h_dc_dn7_slot;
        let mut var_h_dc_dn8: f64 = *var_h_dc_dn8_slot;
        let mut var_h_dc_rv: f64 = *var_h_dc_rv_slot;
        let mut var_i_ds: f64 = *var_i_ds_slot;
        let mut var_i_ds_dn5: f64 = *var_i_ds_dn5_slot;
        let mut var_i_ds_dn6: f64 = *var_i_ds_dn6_slot;
        let mut var_i_ds_dn7: f64 = *var_i_ds_dn7_slot;
        let mut var_i_ds_dn8: f64 = *var_i_ds_dn8_slot;
        let mut var_i_ds_rv: f64 = *var_i_ds_rv_slot;
        let mut var_mutmp: f64 = *var_mutmp_slot;
        let mut var_mutmp_dn5: f64 = *var_mutmp_dn5_slot;
        let mut var_mutmp_dn6: f64 = *var_mutmp_dn6_slot;
        let mut var_mutmp_dn7: f64 = *var_mutmp_dn7_slot;
        let mut var_mutmp_dn8: f64 = *var_mutmp_dn8_slot;
        let mut var_mutmp_rv: f64 = *var_mutmp_rv_slot;
        let mut var_qbd_dc: f64 = *var_qbd_dc_slot;
        let mut var_qbd_dc_dn5: f64 = *var_qbd_dc_dn5_slot;
        let mut var_qbd_dc_dn6: f64 = *var_qbd_dc_dn6_slot;
        let mut var_qbd_dc_dn7: f64 = *var_qbd_dc_dn7_slot;
        let mut var_qbd_dc_dn8: f64 = *var_qbd_dc_dn8_slot;
        let mut var_qbd_dc_rv: f64 = *var_qbd_dc_rv_slot;
        let mut var_qbm: f64 = *var_qbm_slot;
        let mut var_qbm_dc: f64 = *var_qbm_dc_slot;
        let mut var_qbm_dc_dn5: f64 = *var_qbm_dc_dn5_slot;
        let mut var_qbm_dc_dn6: f64 = *var_qbm_dc_dn6_slot;
        let mut var_qbm_dc_dn7: f64 = *var_qbm_dc_dn7_slot;
        let mut var_qbm_dc_dn8: f64 = *var_qbm_dc_dn8_slot;
        let mut var_qbm_dc_rv: f64 = *var_qbm_dc_rv_slot;
        let mut var_qbm_dn5: f64 = *var_qbm_dn5_slot;
        let mut var_qbm_dn6: f64 = *var_qbm_dn6_slot;
        let mut var_qbm_dn7: f64 = *var_qbm_dn7_slot;
        let mut var_qbm_dn8: f64 = *var_qbm_dn8_slot;
        let mut var_qbm_rv: f64 = *var_qbm_rv_slot;
        let mut var_qeff: f64 = *var_qeff_slot;
        let mut var_qeff1: f64 = *var_qeff1_slot;
        let mut var_qeff1_dc: f64 = *var_qeff1_dc_slot;
        let mut var_qeff1_dc_dn5: f64 = *var_qeff1_dc_dn5_slot;
        let mut var_qeff1_dc_dn6: f64 = *var_qeff1_dc_dn6_slot;
        let mut var_qeff1_dc_dn7: f64 = *var_qeff1_dc_dn7_slot;
        let mut var_qeff1_dc_dn8: f64 = *var_qeff1_dc_dn8_slot;
        let mut var_qeff1_dc_rv: f64 = *var_qeff1_dc_rv_slot;
        let mut var_qeff1_dn5: f64 = *var_qeff1_dn5_slot;
        let mut var_qeff1_dn6: f64 = *var_qeff1_dn6_slot;
        let mut var_qeff1_dn7: f64 = *var_qeff1_dn7_slot;
        let mut var_qeff1_dn8: f64 = *var_qeff1_dn8_slot;
        let mut var_qeff1_rv: f64 = *var_qeff1_rv_slot;
        let mut var_qeff_dn5: f64 = *var_qeff_dn5_slot;
        let mut var_qeff_dn6: f64 = *var_qeff_dn6_slot;
        let mut var_qeff_dn7: f64 = *var_qeff_dn7_slot;
        let mut var_qeff_dn8: f64 = *var_qeff_dn8_slot;
        let mut var_qeff_rv: f64 = *var_qeff_rv_slot;
        let mut var_qim: f64 = *var_qim_slot;
        let mut var_qim1: f64 = *var_qim1_slot;
        let mut var_qim1_dc: f64 = *var_qim1_dc_slot;
        let mut var_qim1_dc_dn5: f64 = *var_qim1_dc_dn5_slot;
        let mut var_qim1_dc_dn6: f64 = *var_qim1_dc_dn6_slot;
        let mut var_qim1_dc_dn7: f64 = *var_qim1_dc_dn7_slot;
        let mut var_qim1_dc_dn8: f64 = *var_qim1_dc_dn8_slot;
        let mut var_qim1_dc_rv: f64 = *var_qim1_dc_rv_slot;
        let mut var_qim1_dn5: f64 = *var_qim1_dn5_slot;
        let mut var_qim1_dn6: f64 = *var_qim1_dn6_slot;
        let mut var_qim1_dn7: f64 = *var_qim1_dn7_slot;
        let mut var_qim1_dn8: f64 = *var_qim1_dn8_slot;
        let mut var_qim1_rv: f64 = *var_qim1_rv_slot;
        let mut var_qim_dc: f64 = *var_qim_dc_slot;
        let mut var_qim_dc_dn5: f64 = *var_qim_dc_dn5_slot;
        let mut var_qim_dc_dn6: f64 = *var_qim_dc_dn6_slot;
        let mut var_qim_dc_dn7: f64 = *var_qim_dc_dn7_slot;
        let mut var_qim_dc_dn8: f64 = *var_qim_dc_dn8_slot;
        let mut var_qim_dc_rv: f64 = *var_qim_dc_rv_slot;
        let mut var_qim_dn5: f64 = *var_qim_dn5_slot;
        let mut var_qim_dn6: f64 = *var_qim_dn6_slot;
        let mut var_qim_dn7: f64 = *var_qim_dn7_slot;
        let mut var_qim_dn8: f64 = *var_qim_dn8_slot;
        let mut var_qim_rv: f64 = *var_qim_rv_slot;
        let mut var_rhog: f64 = *var_rhog_slot;
        let mut var_rhog_dn5: f64 = *var_rhog_dn5_slot;
        let mut var_rhog_dn6: f64 = *var_rhog_dn6_slot;
        let mut var_rhog_dn7: f64 = *var_rhog_dn7_slot;
        let mut var_rhog_dn8: f64 = *var_rhog_dn8_slot;
        let mut var_rhog_rv: f64 = *var_rhog_rv_slot;
        let mut var_s1: f64 = *var_s1_slot;
        let mut var_s1_dc: f64 = *var_s1_dc_slot;
        let mut var_s1_dc_dn5: f64 = *var_s1_dc_dn5_slot;
        let mut var_s1_dc_dn6: f64 = *var_s1_dc_dn6_slot;
        let mut var_s1_dc_dn7: f64 = *var_s1_dc_dn7_slot;
        let mut var_s1_dc_dn8: f64 = *var_s1_dc_dn8_slot;
        let mut var_s1_dc_rv: f64 = *var_s1_dc_rv_slot;
        let mut var_s1_dn5: f64 = *var_s1_dn5_slot;
        let mut var_s1_dn6: f64 = *var_s1_dn6_slot;
        let mut var_s1_dn7: f64 = *var_s1_dn7_slot;
        let mut var_s1_dn8: f64 = *var_s1_dn8_slot;
        let mut var_s1_rv: f64 = *var_s1_rv_slot;
        let mut var_s2: f64 = *var_s2_slot;
        let mut var_s2_dn6: f64 = *var_s2_dn6_slot;
        let mut var_s2_dn7: f64 = *var_s2_dn7_slot;
        let mut var_s2_rv: f64 = *var_s2_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_thesateff: f64 = *var_thesateff_slot;
        let mut var_thesateff_dc: f64 = *var_thesateff_dc_slot;
        let mut var_thesateff_dc_dn5: f64 = *var_thesateff_dc_dn5_slot;
        let mut var_thesateff_dc_dn6: f64 = *var_thesateff_dc_dn6_slot;
        let mut var_thesateff_dc_dn7: f64 = *var_thesateff_dc_dn7_slot;
        let mut var_thesateff_dc_dn8: f64 = *var_thesateff_dc_dn8_slot;
        let mut var_thesateff_dc_rv: f64 = *var_thesateff_dc_rv_slot;
        let mut var_thesateff_dn5: f64 = *var_thesateff_dn5_slot;
        let mut var_thesateff_dn6: f64 = *var_thesateff_dn6_slot;
        let mut var_thesateff_dn7: f64 = *var_thesateff_dn7_slot;
        let mut var_thesateff_dn8: f64 = *var_thesateff_dn8_slot;
        let mut var_thesateff_rv: f64 = *var_thesateff_rv_slot;
        let mut var_udse_dc: f64 = *var_udse_dc_slot;
        let mut var_udse_dc_dn5: f64 = *var_udse_dc_dn5_slot;
        let mut var_udse_dc_dn6: f64 = *var_udse_dc_dn6_slot;
        let mut var_udse_dc_dn7: f64 = *var_udse_dc_dn7_slot;
        let mut var_udse_dc_dn8: f64 = *var_udse_dc_dn8_slot;
        let mut var_udse_dc_rv: f64 = *var_udse_dc_rv_slot;
        let mut var_vdsat_lim_dc: f64 = *var_vdsat_lim_dc_slot;
        let mut var_vdsat_lim_dc_dn5: f64 = *var_vdsat_lim_dc_dn5_slot;
        let mut var_vdsat_lim_dc_dn6: f64 = *var_vdsat_lim_dc_dn6_slot;
        let mut var_vdsat_lim_dc_dn7: f64 = *var_vdsat_lim_dc_dn7_slot;
        let mut var_vdsat_lim_dc_dn8: f64 = *var_vdsat_lim_dc_dn8_slot;
        let mut var_vdsat_lim_dc_rv: f64 = *var_vdsat_lim_dc_rv_slot;
        let mut var_vdse_dc: f64 = *var_vdse_dc_slot;
        let mut var_vdse_dc_dn5: f64 = *var_vdse_dc_dn5_slot;
        let mut var_vdse_dc_dn6: f64 = *var_vdse_dc_dn6_slot;
        let mut var_vdse_dc_dn7: f64 = *var_vdse_dc_dn7_slot;
        let mut var_vdse_dc_dn8: f64 = *var_vdse_dc_dn8_slot;
        let mut var_vdse_dc_rv: f64 = *var_vdse_dc_rv_slot;
        let mut var_voxm: f64 = *var_voxm_slot;
        let mut var_voxm_dc: f64 = *var_voxm_dc_slot;
        let mut var_voxm_dc_dn5: f64 = *var_voxm_dc_dn5_slot;
        let mut var_voxm_dc_dn6: f64 = *var_voxm_dc_dn6_slot;
        let mut var_voxm_dc_dn7: f64 = *var_voxm_dc_dn7_slot;
        let mut var_voxm_dc_dn8: f64 = *var_voxm_dc_dn8_slot;
        let mut var_voxm_dc_rv: f64 = *var_voxm_dc_rv_slot;
        let mut var_voxm_dn5: f64 = *var_voxm_dn5_slot;
        let mut var_voxm_dn6: f64 = *var_voxm_dn6_slot;
        let mut var_voxm_dn7: f64 = *var_voxm_dn7_slot;
        let mut var_voxm_dn8: f64 = *var_voxm_dn8_slot;
        let mut var_voxm_rv: f64 = *var_voxm_rv_slot;
        let mut var_wsat: f64 = *var_wsat_slot;
        let mut var_wsat_dn5: f64 = *var_wsat_dn5_slot;
        let mut var_wsat_dn6: f64 = *var_wsat_dn6_slot;
        let mut var_wsat_dn7: f64 = *var_wsat_dn7_slot;
        let mut var_wsat_dn8: f64 = *var_wsat_dn8_slot;
        let mut var_wsat_rv: f64 = *var_wsat_rv_slot;
        let mut var_x_ds_dc: f64 = *var_x_ds_dc_slot;
        let mut var_x_ds_dc_dn5: f64 = *var_x_ds_dc_dn5_slot;
        let mut var_x_ds_dc_dn6: f64 = *var_x_ds_dc_dn6_slot;
        let mut var_x_ds_dc_dn7: f64 = *var_x_ds_dc_dn7_slot;
        let mut var_x_ds_dc_dn8: f64 = *var_x_ds_dc_dn8_slot;
        let mut var_x_ds_dc_rv: f64 = *var_x_ds_dc_rv_slot;
        let mut var_x_m_dc: f64 = *var_x_m_dc_slot;
        let mut var_x_m_dc_dn5: f64 = *var_x_m_dc_dn5_slot;
        let mut var_x_m_dc_dn6: f64 = *var_x_m_dc_dn6_slot;
        let mut var_x_m_dc_dn7: f64 = *var_x_m_dc_dn7_slot;
        let mut var_x_m_dc_dn8: f64 = *var_x_m_dc_dn8_slot;
        let mut var_x_m_dc_rv: f64 = *var_x_m_dc_rv_slot;

        let (assign45150_e58457, assign45150_e58457_d_n5, assign45150_e58457_d_n6, assign45150_e58457_d_n7, assign45150_e58457_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45150_e58448: f64 = (var_gf2 * var_dm);
        let assign45150_e58452: f64 = (var_gf * var_sqm);
        let assign45150_e58453: f64 = (var_xgm + assign45150_e58452);
        let assign45150_e58454: f64 = (assign45150_e58448 / assign45150_e58453);
        let assign45150_e58455: f64 = (var_phit1 * assign45150_e58454);
        (assign45150_e58455, ((var_phit1_dn5 * assign45150_e58454) + (var_phit1 * (((((var_gf2_dn5 * var_dm) + (var_gf2 * var_dm_dn5)) * assign45150_e58453) - (assign45150_e58448 * (var_xgm_dn5 + ((var_gf_dn5 * var_sqm) + (var_gf * var_sqm_dn5))))) / (assign45150_e58453 * assign45150_e58453)))), ((var_phit1_dn6 * assign45150_e58454) + (var_phit1 * (((((var_gf2_dn6 * var_dm) + (var_gf2 * var_dm_dn6)) * assign45150_e58453) - (assign45150_e58448 * (var_xgm_dn6 + ((var_gf_dn6 * var_sqm) + (var_gf * var_sqm_dn6))))) / (assign45150_e58453 * assign45150_e58453)))), ((var_phit1_dn7 * assign45150_e58454) + (var_phit1 * (((((var_gf2_dn7 * var_dm) + (var_gf2 * var_dm_dn7)) * assign45150_e58453) - (assign45150_e58448 * (var_xgm_dn7 + ((var_gf_dn7 * var_sqm) + (var_gf * var_sqm_dn7))))) / (assign45150_e58453 * assign45150_e58453)))), ((var_phit1_dn8 * assign45150_e58454) + (var_phit1 * (((((var_gf2_dn8 * var_dm) + (var_gf2 * var_dm_dn8)) * assign45150_e58453) - (assign45150_e58448 * (var_xgm_dn8 + ((var_gf_dn8 * var_sqm) + (var_gf * var_sqm_dn8))))) / (assign45150_e58453 * assign45150_e58453)))),)
    } else {
        (var_qim, var_qim_dn5, var_qim_dn6, var_qim_dn7, var_qim_dn8,)
    }
};
        var_qim = assign45150_e58457;
        var_qim_dn5 = assign45150_e58457_d_n5;
        var_qim_dn6 = assign45150_e58457_d_n6;
        var_qim_dn7 = assign45150_e58457_d_n7;
        var_qim_dn8 = assign45150_e58457_d_n8;
        var_qim_rv = 0.0;

        let (assign45160_e58465, assign45160_e58465_d_n5, assign45160_e58465_d_n6, assign45160_e58465_d_n7, assign45160_e58465_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45160_e58462: f64 = (var_phit1 * var_alpha);
        let assign45160_e58463: f64 = (var_qim + assign45160_e58462);
        (assign45160_e58463, (var_qim_dn5 + ((var_phit1_dn5 * var_alpha) + (var_phit1 * var_alpha_dn5))), (var_qim_dn6 + ((var_phit1_dn6 * var_alpha) + (var_phit1 * var_alpha_dn6))), (var_qim_dn7 + ((var_phit1_dn7 * var_alpha) + (var_phit1 * var_alpha_dn7))), (var_qim_dn8 + ((var_phit1_dn8 * var_alpha) + (var_phit1 * var_alpha_dn8))),)
    } else {
        (var_qim1, var_qim1_dn5, var_qim1_dn6, var_qim1_dn7, var_qim1_dn8,)
    }
};
        var_qim1 = assign45160_e58465;
        var_qim1_dn5 = assign45160_e58465_d_n5;
        var_qim1_dn6 = assign45160_e58465_d_n6;
        var_qim1_dn7 = assign45160_e58465_d_n7;
        var_qim1_dn8 = assign45160_e58465_d_n8;
        var_qim1_rv = 0.0;

        let (assign45170_e58473, assign45170_e58473_d_n5, assign45170_e58473_d_n6, assign45170_e58473_d_n7, assign45170_e58473_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45170_e58469: f64 = (var_sqm * var_gf);
        let assign45170_e58471: f64 = (assign45170_e58469 * var_phit1);
        (assign45170_e58471, ((((var_sqm_dn5 * var_gf) + (var_sqm * var_gf_dn5)) * var_phit1) + (assign45170_e58469 * var_phit1_dn5)), ((((var_sqm_dn6 * var_gf) + (var_sqm * var_gf_dn6)) * var_phit1) + (assign45170_e58469 * var_phit1_dn6)), ((((var_sqm_dn7 * var_gf) + (var_sqm * var_gf_dn7)) * var_phit1) + (assign45170_e58469 * var_phit1_dn7)), ((((var_sqm_dn8 * var_gf) + (var_sqm * var_gf_dn8)) * var_phit1) + (assign45170_e58469 * var_phit1_dn8)),)
    } else {
        (var_qbm, var_qbm_dn5, var_qbm_dn6, var_qbm_dn7, var_qbm_dn8,)
    }
};
        var_qbm = assign45170_e58473;
        var_qbm_dn5 = assign45170_e58473_d_n5;
        var_qbm_dn6 = assign45170_e58473_d_n6;
        var_qbm_dn7 = assign45170_e58473_d_n7;
        var_qbm_dn8 = assign45170_e58473_d_n8;
        var_qbm_rv = 0.0;

        let assign45180_e58476: f64 = if var_rsg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1216 = assign45180_e58476;
        var_guard1216_rv = 0.0;

        let (assign45190_e58486, assign45190_e58486_d_n5, assign45190_e58486_d_n6, assign45190_e58486_d_n7, assign45190_e58486_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1216 != 0.0)) {
        let assign45190_e58483: f64 = (var_rsg_i * var_qim);
        let assign45190_e58484: f64 = (1.0 - assign45190_e58483);
        (assign45190_e58484, (-(var_rsg_i * var_qim_dn5)), (-(var_rsg_i * var_qim_dn6)), (-(var_rsg_i * var_qim_dn7)), (-(var_rsg_i * var_qim_dn8)),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign45190_e58486;
        var_rhog_dn5 = assign45190_e58486_d_n5;
        var_rhog_dn6 = assign45190_e58486_d_n6;
        var_rhog_dn7 = assign45190_e58486_d_n7;
        var_rhog_dn8 = assign45190_e58486_d_n8;
        var_rhog_rv = 0.0;

        let (assign45200_e58499, assign45200_e58499_d_n5, assign45200_e58499_d_n6, assign45200_e58499_d_n7, assign45200_e58499_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1216 == 0.0)) {
        let assign45200_e58495: f64 = (var_rsg_i * var_qim);
        let assign45200_e58496: f64 = (1.0 + assign45200_e58495);
        let assign45200_e58497: f64 = (1.0 / assign45200_e58496);
        (assign45200_e58497, (-((var_rsg_i * var_qim_dn5) / (assign45200_e58496 * assign45200_e58496))), (-((var_rsg_i * var_qim_dn6) / (assign45200_e58496 * assign45200_e58496))), (-((var_rsg_i * var_qim_dn7) / (assign45200_e58496 * assign45200_e58496))), (-((var_rsg_i * var_qim_dn8) / (assign45200_e58496 * assign45200_e58496))),)
    } else {
        (var_rhog, var_rhog_dn5, var_rhog_dn6, var_rhog_dn7, var_rhog_dn8,)
    }
};
        var_rhog = assign45200_e58499;
        var_rhog_dn5 = assign45200_e58499_d_n5;
        var_rhog_dn6 = assign45200_e58499_d_n6;
        var_rhog_dn7 = assign45200_e58499_d_n7;
        var_rhog_dn8 = assign45200_e58499_d_n8;
        var_rhog_rv = 0.0;

        let (assign45210_e58509, assign45210_e58509_d_n5, assign45210_e58509_d_n6, assign45210_e58509_d_n7, assign45210_e58509_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45210_e58503: f64 = (var_ther_i * var_rhob);
        let assign45210_e58505: f64 = (assign45210_e58503 * var_rhog);
        let assign45210_e58507: f64 = (assign45210_e58505 * var_qim);
        (assign45210_e58507, (((((var_ther_i * var_rhob_dn5) * var_rhog) + (assign45210_e58503 * var_rhog_dn5)) * var_qim) + (assign45210_e58505 * var_qim_dn5)), (((((var_ther_i * var_rhob_dn6) * var_rhog) + (assign45210_e58503 * var_rhog_dn6)) * var_qim) + (assign45210_e58505 * var_qim_dn6)), (((((var_ther_i * var_rhob_dn7) * var_rhog) + (assign45210_e58503 * var_rhog_dn7)) * var_qim) + (assign45210_e58505 * var_qim_dn7)), (((((var_ther_i * var_rhob_dn8) * var_rhog) + (assign45210_e58503 * var_rhog_dn8)) * var_qim) + (assign45210_e58505 * var_qim_dn8)),)
    } else {
        (var_gr, var_gr_dn5, var_gr_dn6, var_gr_dn7, var_gr_dn8,)
    }
};
        var_gr = assign45210_e58509;
        var_gr_dn5 = assign45210_e58509_d_n5;
        var_gr_dn6 = assign45210_e58509_d_n6;
        var_gr_dn7 = assign45210_e58509_d_n7;
        var_gr_dn8 = assign45210_e58509_d_n8;
        var_gr_rv = 0.0;

        let (assign45220_e58517, assign45220_e58517_d_n5, assign45220_e58517_d_n6, assign45220_e58517_d_n7, assign45220_e58517_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45220_e58514: f64 = (var_eta_mu * var_qim);
        let assign45220_e58515: f64 = (var_qbm + assign45220_e58514);
        (assign45220_e58515, (var_qbm_dn5 + (var_eta_mu * var_qim_dn5)), (var_qbm_dn6 + (var_eta_mu * var_qim_dn6)), (var_qbm_dn7 + (var_eta_mu * var_qim_dn7)), (var_qbm_dn8 + (var_eta_mu * var_qim_dn8)),)
    } else {
        (var_qeff, var_qeff_dn5, var_qeff_dn6, var_qeff_dn7, var_qeff_dn8,)
    }
};
        var_qeff = assign45220_e58517;
        var_qeff_dn5 = assign45220_e58517_d_n5;
        var_qeff_dn6 = assign45220_e58517_d_n6;
        var_qeff_dn7 = assign45220_e58517_d_n7;
        var_qeff_dn8 = assign45220_e58517_d_n8;
        var_qeff_rv = 0.0;

        let (assign45230_e58525, assign45230_e58525_d_n5, assign45230_e58525_d_n6, assign45230_e58525_d_n7, assign45230_e58525_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45230_e58522: f64 = (var_eta_mu1 * var_qim);
        let assign45230_e58523: f64 = (var_qbm + assign45230_e58522);
        (assign45230_e58523, (var_qbm_dn5 + (var_eta_mu1 * var_qim_dn5)), (var_qbm_dn6 + (var_eta_mu1 * var_qim_dn6)), (var_qbm_dn7 + (var_eta_mu1 * var_qim_dn7)), (var_qbm_dn8 + (var_eta_mu1 * var_qim_dn8)),)
    } else {
        (var_qeff1, var_qeff1_dn5, var_qeff1_dn6, var_qeff1_dn7, var_qeff1_dn8,)
    }
};
        var_qeff1 = assign45230_e58525;
        var_qeff1_dn5 = assign45230_e58525_d_n5;
        var_qeff1_dn6 = assign45230_e58525_d_n6;
        var_qeff1_dn7 = assign45230_e58525_d_n7;
        var_qeff1_dn8 = assign45230_e58525_d_n8;
        var_qeff1_rv = 0.0;

        let (assign45240_e58531, assign45240_e58531_d_n5, assign45240_e58531_d_n6, assign45240_e58531_d_n7, assign45240_e58531_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45240_e58529: f64 = (var_e_eff0 * var_qeff);
        (assign45240_e58529, (var_e_eff0 * var_qeff_dn5), (var_e_eff0 * var_qeff_dn6), (var_e_eff0 * var_qeff_dn7), (var_e_eff0 * var_qeff_dn8),)
    } else {
        (var_eeffm, var_eeffm_dn5, var_eeffm_dn6, var_eeffm_dn7, var_eeffm_dn8,)
    }
};
        var_eeffm = assign45240_e58531;
        var_eeffm_dn5 = assign45240_e58531_d_n5;
        var_eeffm_dn6 = assign45240_e58531_d_n6;
        var_eeffm_dn7 = assign45240_e58531_d_n7;
        var_eeffm_dn8 = assign45240_e58531_d_n8;
        var_eeffm_rv = 0.0;

        let (assign45250_e58542, assign45250_e58542_d_n5, assign45250_e58542_d_n6, assign45250_e58542_d_n7, assign45250_e58542_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45250_e58536: f64 = (var_pm + var_dm);
        let assign45250_e58538: f64 = (assign45250_e58536 + 1e-14);
        let assign45250_e58539: f64 = (var_pm / assign45250_e58538);
        let assign45250_e58540: f64 = (assign45250_e58539).ln();
        (assign45250_e58540, ((((var_pm_dn5 * assign45250_e58538) - (var_pm * (var_pm_dn5 + var_dm_dn5))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((var_pm_dn6 * assign45250_e58538) - (var_pm * (var_pm_dn6 + var_dm_dn6))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((var_pm_dn7 * assign45250_e58538) - (var_pm * (var_pm_dn7 + var_dm_dn7))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539), ((((var_pm_dn8 * assign45250_e58538) - (var_pm * (var_pm_dn8 + var_dm_dn8))) / (assign45250_e58538 * assign45250_e58538)) / assign45250_e58539),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign45250_e58542;
        var_temp1_dn5 = assign45250_e58542_d_n5;
        var_temp1_dn6 = assign45250_e58542_d_n6;
        var_temp1_dn7 = assign45250_e58542_d_n7;
        var_temp1_dn8 = assign45250_e58542_d_n8;
        var_temp1_rv = 0.0;

        let (assign45260_e58559, assign45260_e58559_d_n5, assign45260_e58559_d_n6, assign45260_e58559_d_n7, assign45260_e58559_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45260_e58546: f64 = (var_eeffm * var_mue_t);
        let assign45260_e58548: f64 = (assign45260_e58546).powf(var_themu_t);
        let assign45260_e58552: f64 = (0.5 * var_thecs_t);
        let assign45260_e58554: f64 = (assign45260_e58552 * var_temp1);
        let assign45260_e58555: f64 = (assign45260_e58554).exp();
        let assign45260_e58556: f64 = (var_cs_t * assign45260_e58555);
        let assign45260_e58557: f64 = (assign45260_e58548 + assign45260_e58556);
        (assign45260_e58557, (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign45260_e58546).powf(var_themu_t - 1.0) * (var_eeffm_dn5 * var_mue_t))) } } else { (assign45260_e58548 * (var_themu_t * ((var_eeffm_dn5 * var_mue_t) / assign45260_e58546))) } + (var_cs_t * (assign45260_e58555 * (assign45260_e58552 * var_temp1_dn5)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign45260_e58546).powf(var_themu_t - 1.0) * (var_eeffm_dn6 * var_mue_t))) } } else { (assign45260_e58548 * (var_themu_t * ((var_eeffm_dn6 * var_mue_t) / assign45260_e58546))) } + (var_cs_t * (assign45260_e58555 * (assign45260_e58552 * var_temp1_dn6)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign45260_e58546).powf(var_themu_t - 1.0) * (var_eeffm_dn7 * var_mue_t))) } } else { (assign45260_e58548 * (var_themu_t * ((var_eeffm_dn7 * var_mue_t) / assign45260_e58546))) } + (var_cs_t * (assign45260_e58555 * (assign45260_e58552 * var_temp1_dn7)))), (if 0.0 == 0.0 && ((var_themu_t) as f64).is_finite() && ((var_themu_t) as f64).fract() == 0.0 { if var_themu_t == 0.0 { 0.0 } else { (var_themu_t * ((assign45260_e58546).powf(var_themu_t - 1.0) * (var_eeffm_dn8 * var_mue_t))) } } else { (assign45260_e58548 * (var_themu_t * ((var_eeffm_dn8 * var_mue_t) / assign45260_e58546))) } + (var_cs_t * (assign45260_e58555 * (assign45260_e58552 * var_temp1_dn8)))),)
    } else {
        (var_mutmp, var_mutmp_dn5, var_mutmp_dn6, var_mutmp_dn7, var_mutmp_dn8,)
    }
};
        var_mutmp = assign45260_e58559;
        var_mutmp_dn5 = assign45260_e58559_d_n5;
        var_mutmp_dn6 = assign45260_e58559_d_n6;
        var_mutmp_dn7 = assign45260_e58559_d_n7;
        var_mutmp_dn8 = assign45260_e58559_d_n8;
        var_mutmp_rv = 0.0;

        let (assign45270_e58569, assign45270_e58569_d_n5, assign45270_e58569_d_n6, assign45270_e58569_d_n7, assign45270_e58569_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45270_e58563: f64 = (1.0 + var_mutmp);
        let assign45270_e58565: f64 = (assign45270_e58563 + var_gr);
        let assign45270_e58567: f64 = (assign45270_e58565 * var_rxcor);
        (assign45270_e58567, (((var_mutmp_dn5 + var_gr_dn5) * var_rxcor) + (assign45270_e58565 * var_rxcor_dn5)), (((var_mutmp_dn6 + var_gr_dn6) * var_rxcor) + (assign45270_e58565 * var_rxcor_dn6)), (((var_mutmp_dn7 + var_gr_dn7) * var_rxcor) + (assign45270_e58565 * var_rxcor_dn7)), (((var_mutmp_dn8 + var_gr_dn8) * var_rxcor) + (assign45270_e58565 * var_rxcor_dn8)),)
    } else {
        (var_gmob, var_gmob_dn5, var_gmob_dn6, var_gmob_dn7, var_gmob_dn8,)
    }
};
        var_gmob = assign45270_e58569;
        var_gmob_dn5 = assign45270_e58569_d_n5;
        var_gmob_dn6 = assign45270_e58569_d_n6;
        var_gmob_dn7 = assign45270_e58569_d_n7;
        var_gmob_dn8 = assign45270_e58569_d_n8;
        var_gmob_rv = 0.0;

        let (assign45280_e58588, assign45280_e58588_d_n5, assign45280_e58588_d_n6, assign45280_e58588_d_n7, assign45280_e58588_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45280_e58574: f64 = (var_v_ds - var_dps);
        let assign45280_e58576: f64 = (assign45280_e58574 * var_inv_vp);
        let assign45280_e58577: f64 = (1.0 + assign45280_e58576);
        let assign45280_e58581: f64 = (var_vdse - var_dps);
        let assign45280_e58583: f64 = (assign45280_e58581 * var_inv_vp);
        let assign45280_e58584: f64 = (1.0 + assign45280_e58583);
        let assign45280_e58585: f64 = (assign45280_e58577 / assign45280_e58584);
        let assign45280_e58586: f64 = (assign45280_e58585).ln();
        (assign45280_e58586, ((((((-var_dps_dn5) * var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((var_vdse_dn5 - var_dps_dn5) * var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((var_v_ds_dn6 - var_dps_dn6) * var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((var_vdse_dn6 - var_dps_dn6) * var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((var_v_ds_dn7 - var_dps_dn7) * var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((var_vdse_dn7 - var_dps_dn7) * var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585), ((((((-var_dps_dn8) * var_inv_vp) * assign45280_e58584) - (assign45280_e58577 * ((var_vdse_dn8 - var_dps_dn8) * var_inv_vp))) / (assign45280_e58584 * assign45280_e58584)) / assign45280_e58585),)
    } else {
        (var_s1, var_s1_dn5, var_s1_dn6, var_s1_dn7, var_s1_dn8,)
    }
};
        var_s1 = assign45280_e58588;
        var_s1_dn5 = assign45280_e58588_d_n5;
        var_s1_dn6 = assign45280_e58588_d_n6;
        var_s1_dn7 = assign45280_e58588_d_n7;
        var_s1_dn8 = assign45280_e58588_d_n8;
        var_s1_rv = 0.0;

        let (assign45290_e58594, assign45290_e58594_d_n5, assign45290_e58594_d_n6, assign45290_e58594_d_n7, assign45290_e58594_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45290_e58592: f64 = (var_qim * var_xitsb);
        (assign45290_e58592, ((var_qim_dn5 * var_xitsb) + (var_qim * var_xitsb_dn5)), ((var_qim_dn6 * var_xitsb) + (var_qim * var_xitsb_dn6)), ((var_qim_dn7 * var_xitsb) + (var_qim * var_xitsb_dn7)), ((var_qim_dn8 * var_xitsb) + (var_qim * var_xitsb_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign45290_e58594;
        var_temp2_dn5 = assign45290_e58594_d_n5;
        var_temp2_dn6 = assign45290_e58594_d_n6;
        var_temp2_dn7 = assign45290_e58594_d_n7;
        var_temp2_dn8 = assign45290_e58594_d_n8;
        var_temp2_rv = 0.0;

        let (assign45300_e58602, assign45300_e58602_d_n5, assign45300_e58602_d_n6, assign45300_e58602_d_n7, assign45300_e58602_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45300_e58599: f64 = (var_thesatt_i + var_temp2);
        let assign45300_e58600: f64 = (var_temp2 / assign45300_e58599);
        (assign45300_e58600, (((var_temp2_dn5 * assign45300_e58599) - (var_temp2 * var_temp2_dn5)) / (assign45300_e58599 * assign45300_e58599)), (((var_temp2_dn6 * assign45300_e58599) - (var_temp2 * var_temp2_dn6)) / (assign45300_e58599 * assign45300_e58599)), (((var_temp2_dn7 * assign45300_e58599) - (var_temp2 * var_temp2_dn7)) / (assign45300_e58599 * assign45300_e58599)), (((var_temp2_dn8 * assign45300_e58599) - (var_temp2 * var_temp2_dn8)) / (assign45300_e58599 * assign45300_e58599)),)
    } else {
        (var_wsat, var_wsat_dn5, var_wsat_dn6, var_wsat_dn7, var_wsat_dn8,)
    }
};
        var_wsat = assign45300_e58602;
        var_wsat_dn5 = assign45300_e58602_d_n5;
        var_wsat_dn6 = assign45300_e58602_d_n6;
        var_wsat_dn7 = assign45300_e58602_d_n7;
        var_wsat_dn8 = assign45300_e58602_d_n8;
        var_wsat_rv = 0.0;

        let assign45310_e58605: f64 = if var_thesatg_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1217 = assign45310_e58605;
        var_guard1217_rv = 0.0;

        let (assign45320_e58617, assign45320_e58617_d_n5, assign45320_e58617_d_n6, assign45320_e58617_d_n7, assign45320_e58617_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1217 != 0.0)) {
        let assign45320_e58613: f64 = (var_thesatg_i * var_wsat);
        let assign45320_e58614: f64 = (1.0 - assign45320_e58613);
        let assign45320_e58615: f64 = (1.0 / assign45320_e58614);
        (assign45320_e58615, (-((-(var_thesatg_i * var_wsat_dn5)) / (assign45320_e58614 * assign45320_e58614))), (-((-(var_thesatg_i * var_wsat_dn6)) / (assign45320_e58614 * assign45320_e58614))), (-((-(var_thesatg_i * var_wsat_dn7)) / (assign45320_e58614 * assign45320_e58614))), (-((-(var_thesatg_i * var_wsat_dn8)) / (assign45320_e58614 * assign45320_e58614))),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign45320_e58617;
        var_factheta_dn5 = assign45320_e58617_d_n5;
        var_factheta_dn6 = assign45320_e58617_d_n6;
        var_factheta_dn7 = assign45320_e58617_d_n7;
        var_factheta_dn8 = assign45320_e58617_d_n8;
        var_factheta_rv = 0.0;

        let (assign45330_e58628, assign45330_e58628_d_n5, assign45330_e58628_d_n6, assign45330_e58628_d_n7, assign45330_e58628_d_n8,) = {
    if ((var_guard1197 != 0.0) && (var_guard1217 == 0.0)) {
        let assign45330_e58625: f64 = (var_thesatg_i * var_wsat);
        let assign45330_e58626: f64 = (1.0 + assign45330_e58625);
        (assign45330_e58626, (var_thesatg_i * var_wsat_dn5), (var_thesatg_i * var_wsat_dn6), (var_thesatg_i * var_wsat_dn7), (var_thesatg_i * var_wsat_dn8),)
    } else {
        (var_factheta, var_factheta_dn5, var_factheta_dn6, var_factheta_dn7, var_factheta_dn8,)
    }
};
        var_factheta = assign45330_e58628;
        var_factheta_dn5 = assign45330_e58628_d_n5;
        var_factheta_dn6 = assign45330_e58628_d_n6;
        var_factheta_dn7 = assign45330_e58628_d_n7;
        var_factheta_dn8 = assign45330_e58628_d_n8;
        var_factheta_rv = 0.0;

        let (assign45340_e58634, assign45340_e58634_d_n5, assign45340_e58634_d_n6, assign45340_e58634_d_n7, assign45340_e58634_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45340_e58632: f64 = (var_thesatloc * var_factheta);
        (assign45340_e58632, (var_thesatloc * var_factheta_dn5), (var_thesatloc * var_factheta_dn6), (var_thesatloc * var_factheta_dn7), (var_thesatloc * var_factheta_dn8),)
    } else {
        (var_thesateff, var_thesateff_dn5, var_thesateff_dn6, var_thesateff_dn7, var_thesateff_dn8,)
    }
};
        var_thesateff = assign45340_e58634;
        var_thesateff_dn5 = assign45340_e58634_d_n5;
        var_thesateff_dn6 = assign45340_e58634_d_n6;
        var_thesateff_dn7 = assign45340_e58634_d_n7;
        var_thesateff_dn8 = assign45340_e58634_d_n8;
        var_thesateff_rv = 0.0;

        let (assign45350_e58640, assign45350_e58640_d_n5, assign45350_e58640_d_n6, assign45350_e58640_d_n7, assign45350_e58640_d_n8,) = {
    if (var_guard1197 != 0.0) {
        let assign45350_e58638: f64 = (var_xgm * var_phit1);
        (assign45350_e58638, ((var_xgm_dn5 * var_phit1) + (var_xgm * var_phit1_dn5)), ((var_xgm_dn6 * var_phit1) + (var_xgm * var_phit1_dn6)), ((var_xgm_dn7 * var_phit1) + (var_xgm * var_phit1_dn7)), ((var_xgm_dn8 * var_phit1) + (var_xgm * var_phit1_dn8)),)
    } else {
        (var_voxm, var_voxm_dn5, var_voxm_dn6, var_voxm_dn7, var_voxm_dn8,)
    }
};
        var_voxm = assign45350_e58640;
        var_voxm_dn5 = assign45350_e58640_d_n5;
        var_voxm_dn6 = assign45350_e58640_d_n6;
        var_voxm_dn7 = assign45350_e58640_d_n7;
        var_voxm_dn8 = assign45350_e58640_d_n8;
        var_voxm_rv = 0.0;

        var_vdsat_lim_dc = var_vdsat_lim;
        var_vdsat_lim_dc_dn5 = var_vdsat_lim_dn5;
        var_vdsat_lim_dc_dn6 = var_vdsat_lim_dn6;
        var_vdsat_lim_dc_dn7 = var_vdsat_lim_dn7;
        var_vdsat_lim_dc_dn8 = var_vdsat_lim_dn8;
        var_vdsat_lim_dc_rv = 0.0;

        var_vdse_dc = var_vdse;
        var_vdse_dc_dn5 = var_vdse_dn5;
        var_vdse_dc_dn6 = var_vdse_dn6;
        var_vdse_dc_dn7 = var_vdse_dn7;
        var_vdse_dc_dn8 = var_vdse_dn8;
        var_vdse_dc_rv = 0.0;

        var_udse_dc = var_udse;
        var_udse_dc_dn5 = var_udse_dn5;
        var_udse_dc_dn6 = var_udse_dn6;
        var_udse_dc_dn7 = var_udse_dn7;
        var_udse_dc_dn8 = var_udse_dn8;
        var_udse_dc_rv = 0.0;

        var_x_ds_dc = var_x_ds;
        var_x_ds_dc_dn5 = var_x_ds_dn5;
        var_x_ds_dc_dn6 = var_x_ds_dn6;
        var_x_ds_dc_dn7 = var_x_ds_dn7;
        var_x_ds_dc_dn8 = var_x_ds_dn8;
        var_x_ds_dc_rv = 0.0;

        var_dps_dc = var_dps;
        var_dps_dc_dn5 = var_dps_dn5;
        var_dps_dc_dn6 = var_dps_dn6;
        var_dps_dc_dn7 = var_dps_dn7;
        var_dps_dc_dn8 = var_dps_dn8;
        var_dps_dc_rv = 0.0;

        var_x_m_dc = var_x_m;
        var_x_m_dc_dn5 = var_x_m_dn5;
        var_x_m_dc_dn6 = var_x_m_dn6;
        var_x_m_dc_dn7 = var_x_m_dn7;
        var_x_m_dc_dn8 = var_x_m_dn8;
        var_x_m_dc_rv = 0.0;

        var_qbd_dc = var_qbd;
        var_qbd_dc_dn5 = var_qbd_dn5;
        var_qbd_dc_dn6 = var_qbd_dn6;
        var_qbd_dc_dn7 = var_qbd_dn7;
        var_qbd_dc_dn8 = var_qbd_dn8;
        var_qbd_dc_rv = 0.0;

        var_eta_p_dc = var_eta_p;
        var_eta_p_dc_dn5 = var_eta_p_dn5;
        var_eta_p_dc_dn6 = var_eta_p_dn6;
        var_eta_p_dc_dn7 = var_eta_p_dn7;
        var_eta_p_dc_dn8 = var_eta_p_dn8;
        var_eta_p_dc_rv = 0.0;

        var_alpha_dc = var_alpha;
        var_alpha_dc_dn5 = var_alpha_dn5;
        var_alpha_dc_dn6 = var_alpha_dn6;
        var_alpha_dc_dn7 = var_alpha_dn7;
        var_alpha_dc_dn8 = var_alpha_dn8;
        var_alpha_dc_rv = 0.0;

        var_qim_dc = var_qim;
        var_qim_dc_dn5 = var_qim_dn5;
        var_qim_dc_dn6 = var_qim_dn6;
        var_qim_dc_dn7 = var_qim_dn7;
        var_qim_dc_dn8 = var_qim_dn8;
        var_qim_dc_rv = 0.0;

        var_qim1_dc = var_qim1;
        var_qim1_dc_dn5 = var_qim1_dn5;
        var_qim1_dc_dn6 = var_qim1_dn6;
        var_qim1_dc_dn7 = var_qim1_dn7;
        var_qim1_dc_dn8 = var_qim1_dn8;
        var_qim1_dc_rv = 0.0;

        var_qbm_dc = var_qbm;
        var_qbm_dc_dn5 = var_qbm_dn5;
        var_qbm_dc_dn6 = var_qbm_dn6;
        var_qbm_dc_dn7 = var_qbm_dn7;
        var_qbm_dc_dn8 = var_qbm_dn8;
        var_qbm_dc_rv = 0.0;

        var_qeff1_dc = var_qeff1;
        var_qeff1_dc_dn5 = var_qeff1_dn5;
        var_qeff1_dc_dn6 = var_qeff1_dn6;
        var_qeff1_dc_dn7 = var_qeff1_dn7;
        var_qeff1_dc_dn8 = var_qeff1_dn8;
        var_qeff1_dc_rv = 0.0;

        var_gmob_dc = var_gmob;
        var_gmob_dc_dn5 = var_gmob_dn5;
        var_gmob_dc_dn6 = var_gmob_dn6;
        var_gmob_dc_dn7 = var_gmob_dn7;
        var_gmob_dc_dn8 = var_gmob_dn8;
        var_gmob_dc_rv = 0.0;

        var_s1_dc = var_s1;
        var_s1_dc_dn5 = var_s1_dn5;
        var_s1_dc_dn6 = var_s1_dn6;
        var_s1_dc_dn7 = var_s1_dn7;
        var_s1_dc_dn8 = var_s1_dn8;
        var_s1_dc_rv = 0.0;

        var_thesateff_dc = var_thesateff;
        var_thesateff_dc_dn5 = var_thesateff_dn5;
        var_thesateff_dc_dn6 = var_thesateff_dn6;
        var_thesateff_dc_dn7 = var_thesateff_dn7;
        var_thesateff_dc_dn8 = var_thesateff_dn8;
        var_thesateff_dc_rv = 0.0;

        var_voxm_dc = var_voxm;
        var_voxm_dc_dn5 = var_voxm_dn5;
        var_voxm_dc_dn6 = var_voxm_dn6;
        var_voxm_dc_dn7 = var_voxm_dn7;
        var_voxm_dc_dn8 = var_voxm_dn8;
        var_voxm_dc_rv = 0.0;

        var_gdl_dc = 1.0;
        var_gdl_dc_dn5 = 0.0;
        var_gdl_dc_dn6 = 0.0;
        var_gdl_dc_dn7 = 0.0;
        var_gdl_dc_dn8 = 0.0;
        var_gdl_dc_rv = 0.0;

        var_gmob_dl_dc = 1.0;
        var_gmob_dl_dc_dn5 = 0.0;
        var_gmob_dl_dc_dn6 = 0.0;
        var_gmob_dl_dc_dn7 = 0.0;
        var_gmob_dl_dc_dn8 = 0.0;
        var_gmob_dl_dc_rv = 0.0;

        var_gvsatinv_dc = 1.0;
        var_gvsatinv_dc_dn5 = 0.0;
        var_gvsatinv_dc_dn6 = 0.0;
        var_gvsatinv_dc_dn7 = 0.0;
        var_gvsatinv_dc_dn8 = 0.0;
        var_gvsatinv_dc_rv = 0.0;

        var_h_dc = 1.0;
        var_h_dc_dn5 = 0.0;
        var_h_dc_dn6 = 0.0;
        var_h_dc_dn7 = 0.0;
        var_h_dc_dn8 = 0.0;
        var_h_dc_rv = 0.0;

        var_i_ds = 0.0;
        var_i_ds_dn5 = 0.0;
        var_i_ds_dn6 = 0.0;
        var_i_ds_dn7 = 0.0;
        var_i_ds_dn8 = 0.0;
        var_i_ds_rv = 0.0;

        let assign45690_e58714: f64 = if var_xg_dc > 0.0 { 1.0 } else { 0.0 };
        var_guard1218 = assign45690_e58714;
        var_guard1218_rv = 0.0;

        let (assign45700_e58723, assign45700_e58723_d_n6, assign45700_e58723_d_n7,) = {
    if (var_guard1218 != 0.0) {
        let assign45700_e58719: f64 = (var_vdsx * var_inv_vp);
        let assign45700_e58720: f64 = (1.0 + assign45700_e58719);
        let assign45700_e58721: f64 = (assign45700_e58720).ln();
        (assign45700_e58721, ((var_vdsx_dn6 * var_inv_vp) / assign45700_e58720), ((var_vdsx_dn7 * var_inv_vp) / assign45700_e58720),)
    } else {
        (var_s2, var_s2_dn6, var_s2_dn7,)
    }
};
        var_s2 = assign45700_e58723;
        var_s2_dn6 = assign45700_e58723_d_n6;
        var_s2_dn7 = assign45700_e58723_d_n7;
        var_s2_rv = 0.0;

        *var_alpha_dc_slot = var_alpha_dc;
        *var_alpha_dc_dn5_slot = var_alpha_dc_dn5;
        *var_alpha_dc_dn6_slot = var_alpha_dc_dn6;
        *var_alpha_dc_dn7_slot = var_alpha_dc_dn7;
        *var_alpha_dc_dn8_slot = var_alpha_dc_dn8;
        *var_alpha_dc_rv_slot = var_alpha_dc_rv;
        *var_dps_dc_slot = var_dps_dc;
        *var_dps_dc_dn5_slot = var_dps_dc_dn5;
        *var_dps_dc_dn6_slot = var_dps_dc_dn6;
        *var_dps_dc_dn7_slot = var_dps_dc_dn7;
        *var_dps_dc_dn8_slot = var_dps_dc_dn8;
        *var_dps_dc_rv_slot = var_dps_dc_rv;
        *var_eeffm_slot = var_eeffm;
        *var_eeffm_dn5_slot = var_eeffm_dn5;
        *var_eeffm_dn6_slot = var_eeffm_dn6;
        *var_eeffm_dn7_slot = var_eeffm_dn7;
        *var_eeffm_dn8_slot = var_eeffm_dn8;
        *var_eeffm_rv_slot = var_eeffm_rv;
        *var_eta_p_dc_slot = var_eta_p_dc;
        *var_eta_p_dc_dn5_slot = var_eta_p_dc_dn5;
        *var_eta_p_dc_dn6_slot = var_eta_p_dc_dn6;
        *var_eta_p_dc_dn7_slot = var_eta_p_dc_dn7;
        *var_eta_p_dc_dn8_slot = var_eta_p_dc_dn8;
        *var_eta_p_dc_rv_slot = var_eta_p_dc_rv;
        *var_factheta_slot = var_factheta;
        *var_factheta_dn5_slot = var_factheta_dn5;
        *var_factheta_dn6_slot = var_factheta_dn6;
        *var_factheta_dn7_slot = var_factheta_dn7;
        *var_factheta_dn8_slot = var_factheta_dn8;
        *var_factheta_rv_slot = var_factheta_rv;
        *var_gdl_dc_slot = var_gdl_dc;
        *var_gdl_dc_dn5_slot = var_gdl_dc_dn5;
        *var_gdl_dc_dn6_slot = var_gdl_dc_dn6;
        *var_gdl_dc_dn7_slot = var_gdl_dc_dn7;
        *var_gdl_dc_dn8_slot = var_gdl_dc_dn8;
        *var_gdl_dc_rv_slot = var_gdl_dc_rv;
        *var_gmob_slot = var_gmob;
        *var_gmob_dc_slot = var_gmob_dc;
        *var_gmob_dc_dn5_slot = var_gmob_dc_dn5;
        *var_gmob_dc_dn6_slot = var_gmob_dc_dn6;
        *var_gmob_dc_dn7_slot = var_gmob_dc_dn7;
        *var_gmob_dc_dn8_slot = var_gmob_dc_dn8;
        *var_gmob_dc_rv_slot = var_gmob_dc_rv;
        *var_gmob_dl_dc_slot = var_gmob_dl_dc;
        *var_gmob_dl_dc_dn5_slot = var_gmob_dl_dc_dn5;
        *var_gmob_dl_dc_dn6_slot = var_gmob_dl_dc_dn6;
        *var_gmob_dl_dc_dn7_slot = var_gmob_dl_dc_dn7;
        *var_gmob_dl_dc_dn8_slot = var_gmob_dl_dc_dn8;
        *var_gmob_dl_dc_rv_slot = var_gmob_dl_dc_rv;
        *var_gmob_dn5_slot = var_gmob_dn5;
        *var_gmob_dn6_slot = var_gmob_dn6;
        *var_gmob_dn7_slot = var_gmob_dn7;
        *var_gmob_dn8_slot = var_gmob_dn8;
        *var_gmob_rv_slot = var_gmob_rv;
        *var_gr_slot = var_gr;
        *var_gr_dn5_slot = var_gr_dn5;
        *var_gr_dn6_slot = var_gr_dn6;
        *var_gr_dn7_slot = var_gr_dn7;
        *var_gr_dn8_slot = var_gr_dn8;
        *var_gr_rv_slot = var_gr_rv;
        *var_guard1216_slot = var_guard1216;
        *var_guard1216_rv_slot = var_guard1216_rv;
        *var_guard1217_slot = var_guard1217;
        *var_guard1217_rv_slot = var_guard1217_rv;
        *var_guard1218_slot = var_guard1218;
        *var_guard1218_rv_slot = var_guard1218_rv;
        *var_gvsatinv_dc_slot = var_gvsatinv_dc;
        *var_gvsatinv_dc_dn5_slot = var_gvsatinv_dc_dn5;
        *var_gvsatinv_dc_dn6_slot = var_gvsatinv_dc_dn6;
        *var_gvsatinv_dc_dn7_slot = var_gvsatinv_dc_dn7;
        *var_gvsatinv_dc_dn8_slot = var_gvsatinv_dc_dn8;
        *var_gvsatinv_dc_rv_slot = var_gvsatinv_dc_rv;
        *var_h_dc_slot = var_h_dc;
        *var_h_dc_dn5_slot = var_h_dc_dn5;
        *var_h_dc_dn6_slot = var_h_dc_dn6;
        *var_h_dc_dn7_slot = var_h_dc_dn7;
        *var_h_dc_dn8_slot = var_h_dc_dn8;
        *var_h_dc_rv_slot = var_h_dc_rv;
        *var_i_ds_slot = var_i_ds;
        *var_i_ds_dn5_slot = var_i_ds_dn5;
        *var_i_ds_dn6_slot = var_i_ds_dn6;
        *var_i_ds_dn7_slot = var_i_ds_dn7;
        *var_i_ds_dn8_slot = var_i_ds_dn8;
        *var_i_ds_rv_slot = var_i_ds_rv;
        *var_mutmp_slot = var_mutmp;
        *var_mutmp_dn5_slot = var_mutmp_dn5;
        *var_mutmp_dn6_slot = var_mutmp_dn6;
        *var_mutmp_dn7_slot = var_mutmp_dn7;
        *var_mutmp_dn8_slot = var_mutmp_dn8;
        *var_mutmp_rv_slot = var_mutmp_rv;
        *var_qbd_dc_slot = var_qbd_dc;
        *var_qbd_dc_dn5_slot = var_qbd_dc_dn5;
        *var_qbd_dc_dn6_slot = var_qbd_dc_dn6;
        *var_qbd_dc_dn7_slot = var_qbd_dc_dn7;
        *var_qbd_dc_dn8_slot = var_qbd_dc_dn8;
        *var_qbd_dc_rv_slot = var_qbd_dc_rv;
        *var_qbm_slot = var_qbm;
        *var_qbm_dc_slot = var_qbm_dc;
        *var_qbm_dc_dn5_slot = var_qbm_dc_dn5;
        *var_qbm_dc_dn6_slot = var_qbm_dc_dn6;
        *var_qbm_dc_dn7_slot = var_qbm_dc_dn7;
        *var_qbm_dc_dn8_slot = var_qbm_dc_dn8;
        *var_qbm_dc_rv_slot = var_qbm_dc_rv;
        *var_qbm_dn5_slot = var_qbm_dn5;
        *var_qbm_dn6_slot = var_qbm_dn6;
        *var_qbm_dn7_slot = var_qbm_dn7;
        *var_qbm_dn8_slot = var_qbm_dn8;
        *var_qbm_rv_slot = var_qbm_rv;
        *var_qeff_slot = var_qeff;
        *var_qeff1_slot = var_qeff1;
        *var_qeff1_dc_slot = var_qeff1_dc;
        *var_qeff1_dc_dn5_slot = var_qeff1_dc_dn5;
        *var_qeff1_dc_dn6_slot = var_qeff1_dc_dn6;
        *var_qeff1_dc_dn7_slot = var_qeff1_dc_dn7;
        *var_qeff1_dc_dn8_slot = var_qeff1_dc_dn8;
        *var_qeff1_dc_rv_slot = var_qeff1_dc_rv;
        *var_qeff1_dn5_slot = var_qeff1_dn5;
        *var_qeff1_dn6_slot = var_qeff1_dn6;
        *var_qeff1_dn7_slot = var_qeff1_dn7;
        *var_qeff1_dn8_slot = var_qeff1_dn8;
        *var_qeff1_rv_slot = var_qeff1_rv;
        *var_qeff_dn5_slot = var_qeff_dn5;
        *var_qeff_dn6_slot = var_qeff_dn6;
        *var_qeff_dn7_slot = var_qeff_dn7;
        *var_qeff_dn8_slot = var_qeff_dn8;
        *var_qeff_rv_slot = var_qeff_rv;
        *var_qim_slot = var_qim;
        *var_qim1_slot = var_qim1;
        *var_qim1_dc_slot = var_qim1_dc;
        *var_qim1_dc_dn5_slot = var_qim1_dc_dn5;
        *var_qim1_dc_dn6_slot = var_qim1_dc_dn6;
        *var_qim1_dc_dn7_slot = var_qim1_dc_dn7;
        *var_qim1_dc_dn8_slot = var_qim1_dc_dn8;
        *var_qim1_dc_rv_slot = var_qim1_dc_rv;
        *var_qim1_dn5_slot = var_qim1_dn5;
        *var_qim1_dn6_slot = var_qim1_dn6;
        *var_qim1_dn7_slot = var_qim1_dn7;
        *var_qim1_dn8_slot = var_qim1_dn8;
        *var_qim1_rv_slot = var_qim1_rv;
        *var_qim_dc_slot = var_qim_dc;
        *var_qim_dc_dn5_slot = var_qim_dc_dn5;
        *var_qim_dc_dn6_slot = var_qim_dc_dn6;
        *var_qim_dc_dn7_slot = var_qim_dc_dn7;
        *var_qim_dc_dn8_slot = var_qim_dc_dn8;
        *var_qim_dc_rv_slot = var_qim_dc_rv;
        *var_qim_dn5_slot = var_qim_dn5;
        *var_qim_dn6_slot = var_qim_dn6;
        *var_qim_dn7_slot = var_qim_dn7;
        *var_qim_dn8_slot = var_qim_dn8;
        *var_qim_rv_slot = var_qim_rv;
        *var_rhog_slot = var_rhog;
        *var_rhog_dn5_slot = var_rhog_dn5;
        *var_rhog_dn6_slot = var_rhog_dn6;
        *var_rhog_dn7_slot = var_rhog_dn7;
        *var_rhog_dn8_slot = var_rhog_dn8;
        *var_rhog_rv_slot = var_rhog_rv;
        *var_s1_slot = var_s1;
        *var_s1_dc_slot = var_s1_dc;
        *var_s1_dc_dn5_slot = var_s1_dc_dn5;
        *var_s1_dc_dn6_slot = var_s1_dc_dn6;
        *var_s1_dc_dn7_slot = var_s1_dc_dn7;
        *var_s1_dc_dn8_slot = var_s1_dc_dn8;
        *var_s1_dc_rv_slot = var_s1_dc_rv;
        *var_s1_dn5_slot = var_s1_dn5;
        *var_s1_dn6_slot = var_s1_dn6;
        *var_s1_dn7_slot = var_s1_dn7;
        *var_s1_dn8_slot = var_s1_dn8;
        *var_s1_rv_slot = var_s1_rv;
        *var_s2_slot = var_s2;
        *var_s2_dn6_slot = var_s2_dn6;
        *var_s2_dn7_slot = var_s2_dn7;
        *var_s2_rv_slot = var_s2_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_thesateff_slot = var_thesateff;
        *var_thesateff_dc_slot = var_thesateff_dc;
        *var_thesateff_dc_dn5_slot = var_thesateff_dc_dn5;
        *var_thesateff_dc_dn6_slot = var_thesateff_dc_dn6;
        *var_thesateff_dc_dn7_slot = var_thesateff_dc_dn7;
        *var_thesateff_dc_dn8_slot = var_thesateff_dc_dn8;
        *var_thesateff_dc_rv_slot = var_thesateff_dc_rv;
        *var_thesateff_dn5_slot = var_thesateff_dn5;
        *var_thesateff_dn6_slot = var_thesateff_dn6;
        *var_thesateff_dn7_slot = var_thesateff_dn7;
        *var_thesateff_dn8_slot = var_thesateff_dn8;
        *var_thesateff_rv_slot = var_thesateff_rv;
        *var_udse_dc_slot = var_udse_dc;
        *var_udse_dc_dn5_slot = var_udse_dc_dn5;
        *var_udse_dc_dn6_slot = var_udse_dc_dn6;
        *var_udse_dc_dn7_slot = var_udse_dc_dn7;
        *var_udse_dc_dn8_slot = var_udse_dc_dn8;
        *var_udse_dc_rv_slot = var_udse_dc_rv;
        *var_vdsat_lim_dc_slot = var_vdsat_lim_dc;
        *var_vdsat_lim_dc_dn5_slot = var_vdsat_lim_dc_dn5;
        *var_vdsat_lim_dc_dn6_slot = var_vdsat_lim_dc_dn6;
        *var_vdsat_lim_dc_dn7_slot = var_vdsat_lim_dc_dn7;
        *var_vdsat_lim_dc_dn8_slot = var_vdsat_lim_dc_dn8;
        *var_vdsat_lim_dc_rv_slot = var_vdsat_lim_dc_rv;
        *var_vdse_dc_slot = var_vdse_dc;
        *var_vdse_dc_dn5_slot = var_vdse_dc_dn5;
        *var_vdse_dc_dn6_slot = var_vdse_dc_dn6;
        *var_vdse_dc_dn7_slot = var_vdse_dc_dn7;
        *var_vdse_dc_dn8_slot = var_vdse_dc_dn8;
        *var_vdse_dc_rv_slot = var_vdse_dc_rv;
        *var_voxm_slot = var_voxm;
        *var_voxm_dc_slot = var_voxm_dc;
        *var_voxm_dc_dn5_slot = var_voxm_dc_dn5;
        *var_voxm_dc_dn6_slot = var_voxm_dc_dn6;
        *var_voxm_dc_dn7_slot = var_voxm_dc_dn7;
        *var_voxm_dc_dn8_slot = var_voxm_dc_dn8;
        *var_voxm_dc_rv_slot = var_voxm_dc_rv;
        *var_voxm_dn5_slot = var_voxm_dn5;
        *var_voxm_dn6_slot = var_voxm_dn6;
        *var_voxm_dn7_slot = var_voxm_dn7;
        *var_voxm_dn8_slot = var_voxm_dn8;
        *var_voxm_rv_slot = var_voxm_rv;
        *var_wsat_slot = var_wsat;
        *var_wsat_dn5_slot = var_wsat_dn5;
        *var_wsat_dn6_slot = var_wsat_dn6;
        *var_wsat_dn7_slot = var_wsat_dn7;
        *var_wsat_dn8_slot = var_wsat_dn8;
        *var_wsat_rv_slot = var_wsat_rv;
        *var_x_ds_dc_slot = var_x_ds_dc;
        *var_x_ds_dc_dn5_slot = var_x_ds_dc_dn5;
        *var_x_ds_dc_dn6_slot = var_x_ds_dc_dn6;
        *var_x_ds_dc_dn7_slot = var_x_ds_dc_dn7;
        *var_x_ds_dc_dn8_slot = var_x_ds_dc_dn8;
        *var_x_ds_dc_rv_slot = var_x_ds_dc_rv;
        *var_x_m_dc_slot = var_x_m_dc;
        *var_x_m_dc_dn5_slot = var_x_m_dc_dn5;
        *var_x_m_dc_dn6_slot = var_x_m_dc_dn6;
        *var_x_m_dc_dn7_slot = var_x_m_dc_dn7;
        *var_x_m_dc_dn8_slot = var_x_m_dc_dn8;
        *var_x_m_dc_rv_slot = var_x_m_dc_rv;
    }

    pub(super) fn stamp_reactive_block_37(
        p: &Parameters,
        var_agidl_i: f64,
        var_agidld_i: f64,
        var_alp1_i: f64,
        var_alp2_i: f64,
        var_alp_i: f64,
        var_alpha_dc: f64,
        var_alpha_dc_dn5: f64,
        var_alpha_dc_dn6: f64,
        var_alpha_dc_dn7: f64,
        var_alpha_dc_dn8: f64,
        var_bet_i: f64,
        var_bov: f64,
        var_cgov_i: f64,
        var_cgovd_i: f64,
        var_chnl_type: f64,
        var_dps_dc: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_gc2ov_i: f64,
        var_gc3ov_i: f64,
        var_gco_i: f64,
        var_gcqov: f64,
        var_gmob_dc: f64,
        var_gmob_dc_dn5: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_gov2_d: f64,
        var_gov2_s: f64,
        var_gov_d: f64,
        var_gov_s: f64,
        var_guard1218: f64,
        var_igov_i: f64,
        var_igovd_i: f64,
        var_inv_chib: f64,
        var_phit1_dc: f64,
        var_phit1_dc_dn5: f64,
        var_phit1_dc_dn6: f64,
        var_phit1_dc_dn7: f64,
        var_phit1_dc_dn8: f64,
        var_phita: f64,
        var_qbm_dc: f64,
        var_qbm_dc_dn5: f64,
        var_qbm_dc_dn6: f64,
        var_qbm_dc_dn7: f64,
        var_qbm_dc_dn8: f64,
        var_qim1_dc: f64,
        var_qim1_dc_dn5: f64,
        var_qim1_dc_dn6: f64,
        var_qim1_dc_dn7: f64,
        var_qim1_dc_dn8: f64,
        var_qim_dc: f64,
        var_qim_dc_dn5: f64,
        var_qim_dc_dn6: f64,
        var_qim_dc_dn7: f64,
        var_qim_dc_dn8: f64,
        var_s1_dc: f64,
        var_s1_dc_dn5: f64,
        var_s1_dc_dn6: f64,
        var_s1_dc_dn7: f64,
        var_s1_dc_dn8: f64,
        var_s2: f64,
        var_s2_dn6: f64,
        var_s2_dn7: f64,
        var_sp_ov_a_d: f64,
        var_sp_ov_a_s: f64,
        var_sp_ov_delta1_d: f64,
        var_sp_ov_delta1_s: f64,
        var_sp_ov_eps2_d: f64,
        var_sp_ov_eps2_s: f64,
        var_thesateff_dc: f64,
        var_thesateff_dc_dn5: f64,
        var_thesateff_dc_dn6: f64,
        var_thesateff_dc_dn7: f64,
        var_thesateff_dc_dn8: f64,
        var_vgsprime: f64,
        var_vgsprime_dn5: f64,
        var_vgsprime_dn6: f64,
        var_vgsprime_dn7: f64,
        var_xgd_ov: f64,
        var_xgd_ov_dn5: f64,
        var_xgd_ov_dn6: f64,
        var_xgd_ov_dn7: f64,
        var_xgs_ov: f64,
        var_xgs_ov_dn5: f64,
        var_xgs_ov_dn6: f64,
        var_xgs_ov_dn7: f64,
        var_alpha1_slot: &mut f64,
        var_alpha1_dn5_slot: &mut f64,
        var_alpha1_dn6_slot: &mut f64,
        var_alpha1_dn7_slot: &mut f64,
        var_alpha1_dn8_slot: &mut f64,
        var_alpha1_rv_slot: &mut f64,
        var_dl_slot: &mut f64,
        var_dl_dn5_slot: &mut f64,
        var_dl_dn6_slot: &mut f64,
        var_dl_dn7_slot: &mut f64,
        var_dl_dn8_slot: &mut f64,
        var_dl_rv_slot: &mut f64,
        var_fs1_slot: &mut f64,
        var_fs1_dn5_slot: &mut f64,
        var_fs1_dn6_slot: &mut f64,
        var_fs1_dn7_slot: &mut f64,
        var_fs1_rv_slot: &mut f64,
        var_fs2_slot: &mut f64,
        var_fs2_rv_slot: &mut f64,
        var_fs3_slot: &mut f64,
        var_fs3_dn5_slot: &mut f64,
        var_fs3_dn6_slot: &mut f64,
        var_fs3_dn7_slot: &mut f64,
        var_fs3_rv_slot: &mut f64,
        var_gdl_dc_slot: &mut f64,
        var_gdl_dc_dn5_slot: &mut f64,
        var_gdl_dc_dn6_slot: &mut f64,
        var_gdl_dc_dn7_slot: &mut f64,
        var_gdl_dc_dn8_slot: &mut f64,
        var_gdl_dc_rv_slot: &mut f64,
        var_gmob_dl_dc_slot: &mut f64,
        var_gmob_dl_dc_dn5_slot: &mut f64,
        var_gmob_dl_dc_dn6_slot: &mut f64,
        var_gmob_dl_dc_dn7_slot: &mut f64,
        var_gmob_dl_dc_dn8_slot: &mut f64,
        var_gmob_dl_dc_rv_slot: &mut f64,
        var_guard1219_slot: &mut f64,
        var_guard1219_rv_slot: &mut f64,
        var_guard1220_slot: &mut f64,
        var_guard1220_rv_slot: &mut f64,
        var_guard1221_slot: &mut f64,
        var_guard1221_rv_slot: &mut f64,
        var_guard1222_slot: &mut f64,
        var_guard1222_rv_slot: &mut f64,
        var_guard1223_slot: &mut f64,
        var_guard1223_rv_slot: &mut f64,
        var_gvsat_slot: &mut f64,
        var_gvsat_dn5_slot: &mut f64,
        var_gvsat_dn6_slot: &mut f64,
        var_gvsat_dn7_slot: &mut f64,
        var_gvsat_dn8_slot: &mut f64,
        var_gvsat_rv_slot: &mut f64,
        var_gvsatinv_dc_slot: &mut f64,
        var_gvsatinv_dc_dn5_slot: &mut f64,
        var_gvsatinv_dc_dn6_slot: &mut f64,
        var_gvsatinv_dc_dn7_slot: &mut f64,
        var_gvsatinv_dc_dn8_slot: &mut f64,
        var_gvsatinv_dc_rv_slot: &mut f64,
        var_h_dc_slot: &mut f64,
        var_h_dc_dn5_slot: &mut f64,
        var_h_dc_dn6_slot: &mut f64,
        var_h_dc_dn7_slot: &mut f64,
        var_h_dc_dn8_slot: &mut f64,
        var_h_dc_rv_slot: &mut f64,
        var_i_ds_slot: &mut f64,
        var_i_ds_dn5_slot: &mut f64,
        var_i_ds_dn6_slot: &mut f64,
        var_i_ds_dn7_slot: &mut f64,
        var_i_ds_dn8_slot: &mut f64,
        var_i_ds_rv_slot: &mut f64,
        var_sp_ov_xg_slot: &mut f64,
        var_sp_ov_xg_dn5_slot: &mut f64,
        var_sp_ov_xg_dn6_slot: &mut f64,
        var_sp_ov_xg_dn7_slot: &mut f64,
        var_sp_ov_xg_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_thesat1_dc_slot: &mut f64,
        var_thesat1_dc_dn5_slot: &mut f64,
        var_thesat1_dc_dn6_slot: &mut f64,
        var_thesat1_dc_dn7_slot: &mut f64,
        var_thesat1_dc_dn8_slot: &mut f64,
        var_thesat1_dc_rv_slot: &mut f64,
        var_tme1_slot: &mut f64,
        var_tme1_rv_slot: &mut f64,
        var_tme2_slot: &mut f64,
        var_tme2_dn5_slot: &mut f64,
        var_tme2_dn6_slot: &mut f64,
        var_tme2_dn7_slot: &mut f64,
        var_tme2_dn8_slot: &mut f64,
        var_tme2_rv_slot: &mut f64,
        var_vovd_slot: &mut f64,
        var_vovd_dn5_slot: &mut f64,
        var_vovd_dn6_slot: &mut f64,
        var_vovd_dn7_slot: &mut f64,
        var_vovd_rv_slot: &mut f64,
        var_vovs_slot: &mut f64,
        var_vovs_dn5_slot: &mut f64,
        var_vovs_dn6_slot: &mut f64,
        var_vovs_dn7_slot: &mut f64,
        var_vovs_rv_slot: &mut f64,
        var_xd_ov_slot: &mut f64,
        var_xd_ov_dn5_slot: &mut f64,
        var_xd_ov_dn6_slot: &mut f64,
        var_xd_ov_dn7_slot: &mut f64,
        var_xd_ov_rv_slot: &mut f64,
        var_xs_ov_slot: &mut f64,
        var_xs_ov_dn5_slot: &mut f64,
        var_xs_ov_dn6_slot: &mut f64,
        var_xs_ov_dn7_slot: &mut f64,
        var_xs_ov_rv_slot: &mut f64,
        var_zg_slot: &mut f64,
        var_zg_dn5_slot: &mut f64,
        var_zg_dn6_slot: &mut f64,
        var_zg_dn7_slot: &mut f64,
        var_zg_dn8_slot: &mut f64,
        var_zg_rv_slot: &mut f64,
        var_zsat_slot: &mut f64,
        var_zsat_dn5_slot: &mut f64,
        var_zsat_dn6_slot: &mut f64,
        var_zsat_dn7_slot: &mut f64,
        var_zsat_dn8_slot: &mut f64,
        var_zsat_rv_slot: &mut f64,
    ) {
        let mut var_alpha1: f64 = *var_alpha1_slot;
        let mut var_alpha1_dn5: f64 = *var_alpha1_dn5_slot;
        let mut var_alpha1_dn6: f64 = *var_alpha1_dn6_slot;
        let mut var_alpha1_dn7: f64 = *var_alpha1_dn7_slot;
        let mut var_alpha1_dn8: f64 = *var_alpha1_dn8_slot;
        let mut var_alpha1_rv: f64 = *var_alpha1_rv_slot;
        let mut var_dl: f64 = *var_dl_slot;
        let mut var_dl_dn5: f64 = *var_dl_dn5_slot;
        let mut var_dl_dn6: f64 = *var_dl_dn6_slot;
        let mut var_dl_dn7: f64 = *var_dl_dn7_slot;
        let mut var_dl_dn8: f64 = *var_dl_dn8_slot;
        let mut var_dl_rv: f64 = *var_dl_rv_slot;
        let mut var_fs1: f64 = *var_fs1_slot;
        let mut var_fs1_dn5: f64 = *var_fs1_dn5_slot;
        let mut var_fs1_dn6: f64 = *var_fs1_dn6_slot;
        let mut var_fs1_dn7: f64 = *var_fs1_dn7_slot;
        let mut var_fs1_rv: f64 = *var_fs1_rv_slot;
        let mut var_fs2: f64 = *var_fs2_slot;
        let mut var_fs2_rv: f64 = *var_fs2_rv_slot;
        let mut var_fs3: f64 = *var_fs3_slot;
        let mut var_fs3_dn5: f64 = *var_fs3_dn5_slot;
        let mut var_fs3_dn6: f64 = *var_fs3_dn6_slot;
        let mut var_fs3_dn7: f64 = *var_fs3_dn7_slot;
        let mut var_fs3_rv: f64 = *var_fs3_rv_slot;
        let mut var_gdl_dc: f64 = *var_gdl_dc_slot;
        let mut var_gdl_dc_dn5: f64 = *var_gdl_dc_dn5_slot;
        let mut var_gdl_dc_dn6: f64 = *var_gdl_dc_dn6_slot;
        let mut var_gdl_dc_dn7: f64 = *var_gdl_dc_dn7_slot;
        let mut var_gdl_dc_dn8: f64 = *var_gdl_dc_dn8_slot;
        let mut var_gdl_dc_rv: f64 = *var_gdl_dc_rv_slot;
        let mut var_gmob_dl_dc: f64 = *var_gmob_dl_dc_slot;
        let mut var_gmob_dl_dc_dn5: f64 = *var_gmob_dl_dc_dn5_slot;
        let mut var_gmob_dl_dc_dn6: f64 = *var_gmob_dl_dc_dn6_slot;
        let mut var_gmob_dl_dc_dn7: f64 = *var_gmob_dl_dc_dn7_slot;
        let mut var_gmob_dl_dc_dn8: f64 = *var_gmob_dl_dc_dn8_slot;
        let mut var_gmob_dl_dc_rv: f64 = *var_gmob_dl_dc_rv_slot;
        let mut var_guard1219: f64 = *var_guard1219_slot;
        let mut var_guard1219_rv: f64 = *var_guard1219_rv_slot;
        let mut var_guard1220: f64 = *var_guard1220_slot;
        let mut var_guard1220_rv: f64 = *var_guard1220_rv_slot;
        let mut var_guard1221: f64 = *var_guard1221_slot;
        let mut var_guard1221_rv: f64 = *var_guard1221_rv_slot;
        let mut var_guard1222: f64 = *var_guard1222_slot;
        let mut var_guard1222_rv: f64 = *var_guard1222_rv_slot;
        let mut var_guard1223: f64 = *var_guard1223_slot;
        let mut var_guard1223_rv: f64 = *var_guard1223_rv_slot;
        let mut var_gvsat: f64 = *var_gvsat_slot;
        let mut var_gvsat_dn5: f64 = *var_gvsat_dn5_slot;
        let mut var_gvsat_dn6: f64 = *var_gvsat_dn6_slot;
        let mut var_gvsat_dn7: f64 = *var_gvsat_dn7_slot;
        let mut var_gvsat_dn8: f64 = *var_gvsat_dn8_slot;
        let mut var_gvsat_rv: f64 = *var_gvsat_rv_slot;
        let mut var_gvsatinv_dc: f64 = *var_gvsatinv_dc_slot;
        let mut var_gvsatinv_dc_dn5: f64 = *var_gvsatinv_dc_dn5_slot;
        let mut var_gvsatinv_dc_dn6: f64 = *var_gvsatinv_dc_dn6_slot;
        let mut var_gvsatinv_dc_dn7: f64 = *var_gvsatinv_dc_dn7_slot;
        let mut var_gvsatinv_dc_dn8: f64 = *var_gvsatinv_dc_dn8_slot;
        let mut var_gvsatinv_dc_rv: f64 = *var_gvsatinv_dc_rv_slot;
        let mut var_h_dc: f64 = *var_h_dc_slot;
        let mut var_h_dc_dn5: f64 = *var_h_dc_dn5_slot;
        let mut var_h_dc_dn6: f64 = *var_h_dc_dn6_slot;
        let mut var_h_dc_dn7: f64 = *var_h_dc_dn7_slot;
        let mut var_h_dc_dn8: f64 = *var_h_dc_dn8_slot;
        let mut var_h_dc_rv: f64 = *var_h_dc_rv_slot;
        let mut var_i_ds: f64 = *var_i_ds_slot;
        let mut var_i_ds_dn5: f64 = *var_i_ds_dn5_slot;
        let mut var_i_ds_dn6: f64 = *var_i_ds_dn6_slot;
        let mut var_i_ds_dn7: f64 = *var_i_ds_dn7_slot;
        let mut var_i_ds_dn8: f64 = *var_i_ds_dn8_slot;
        let mut var_i_ds_rv: f64 = *var_i_ds_rv_slot;
        let mut var_sp_ov_xg: f64 = *var_sp_ov_xg_slot;
        let mut var_sp_ov_xg_dn5: f64 = *var_sp_ov_xg_dn5_slot;
        let mut var_sp_ov_xg_dn6: f64 = *var_sp_ov_xg_dn6_slot;
        let mut var_sp_ov_xg_dn7: f64 = *var_sp_ov_xg_dn7_slot;
        let mut var_sp_ov_xg_rv: f64 = *var_sp_ov_xg_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_thesat1_dc: f64 = *var_thesat1_dc_slot;
        let mut var_thesat1_dc_dn5: f64 = *var_thesat1_dc_dn5_slot;
        let mut var_thesat1_dc_dn6: f64 = *var_thesat1_dc_dn6_slot;
        let mut var_thesat1_dc_dn7: f64 = *var_thesat1_dc_dn7_slot;
        let mut var_thesat1_dc_dn8: f64 = *var_thesat1_dc_dn8_slot;
        let mut var_thesat1_dc_rv: f64 = *var_thesat1_dc_rv_slot;
        let mut var_tme1: f64 = *var_tme1_slot;
        let mut var_tme1_rv: f64 = *var_tme1_rv_slot;
        let mut var_tme2: f64 = *var_tme2_slot;
        let mut var_tme2_dn5: f64 = *var_tme2_dn5_slot;
        let mut var_tme2_dn6: f64 = *var_tme2_dn6_slot;
        let mut var_tme2_dn7: f64 = *var_tme2_dn7_slot;
        let mut var_tme2_dn8: f64 = *var_tme2_dn8_slot;
        let mut var_tme2_rv: f64 = *var_tme2_rv_slot;
        let mut var_vovd: f64 = *var_vovd_slot;
        let mut var_vovd_dn5: f64 = *var_vovd_dn5_slot;
        let mut var_vovd_dn6: f64 = *var_vovd_dn6_slot;
        let mut var_vovd_dn7: f64 = *var_vovd_dn7_slot;
        let mut var_vovd_rv: f64 = *var_vovd_rv_slot;
        let mut var_vovs: f64 = *var_vovs_slot;
        let mut var_vovs_dn5: f64 = *var_vovs_dn5_slot;
        let mut var_vovs_dn6: f64 = *var_vovs_dn6_slot;
        let mut var_vovs_dn7: f64 = *var_vovs_dn7_slot;
        let mut var_vovs_rv: f64 = *var_vovs_rv_slot;
        let mut var_xd_ov: f64 = *var_xd_ov_slot;
        let mut var_xd_ov_dn5: f64 = *var_xd_ov_dn5_slot;
        let mut var_xd_ov_dn6: f64 = *var_xd_ov_dn6_slot;
        let mut var_xd_ov_dn7: f64 = *var_xd_ov_dn7_slot;
        let mut var_xd_ov_rv: f64 = *var_xd_ov_rv_slot;
        let mut var_xs_ov: f64 = *var_xs_ov_slot;
        let mut var_xs_ov_dn5: f64 = *var_xs_ov_dn5_slot;
        let mut var_xs_ov_dn6: f64 = *var_xs_ov_dn6_slot;
        let mut var_xs_ov_dn7: f64 = *var_xs_ov_dn7_slot;
        let mut var_xs_ov_rv: f64 = *var_xs_ov_rv_slot;
        let mut var_zg: f64 = *var_zg_slot;
        let mut var_zg_dn5: f64 = *var_zg_dn5_slot;
        let mut var_zg_dn6: f64 = *var_zg_dn6_slot;
        let mut var_zg_dn7: f64 = *var_zg_dn7_slot;
        let mut var_zg_dn8: f64 = *var_zg_dn8_slot;
        let mut var_zg_rv: f64 = *var_zg_rv_slot;
        let mut var_zsat: f64 = *var_zsat_slot;
        let mut var_zsat_dn5: f64 = *var_zsat_dn5_slot;
        let mut var_zsat_dn6: f64 = *var_zsat_dn6_slot;
        let mut var_zsat_dn7: f64 = *var_zsat_dn7_slot;
        let mut var_zsat_dn8: f64 = *var_zsat_dn8_slot;
        let mut var_zsat_rv: f64 = *var_zsat_rv_slot;

        let (assign45710_e58731, assign45710_e58731_d_n5, assign45710_e58731_d_n6, assign45710_e58731_d_n7, assign45710_e58731_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45710_e58727: f64 = (var_phit1_dc * var_alpha_dc);
        let assign45710_e58729: f64 = (assign45710_e58727 / var_qim1_dc);
        (assign45710_e58729, (((((var_phit1_dc_dn5 * var_alpha_dc) + (var_phit1_dc * var_alpha_dc_dn5)) * var_qim1_dc) - (assign45710_e58727 * var_qim1_dc_dn5)) / (var_qim1_dc * var_qim1_dc)), (((((var_phit1_dc_dn6 * var_alpha_dc) + (var_phit1_dc * var_alpha_dc_dn6)) * var_qim1_dc) - (assign45710_e58727 * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)), (((((var_phit1_dc_dn7 * var_alpha_dc) + (var_phit1_dc * var_alpha_dc_dn7)) * var_qim1_dc) - (assign45710_e58727 * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)), (((((var_phit1_dc_dn8 * var_alpha_dc) + (var_phit1_dc * var_alpha_dc_dn8)) * var_qim1_dc) - (assign45710_e58727 * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign45710_e58731;
        var_temp__blk936_dn5 = assign45710_e58731_d_n5;
        var_temp__blk936_dn6 = assign45710_e58731_d_n6;
        var_temp__blk936_dn7 = assign45710_e58731_d_n7;
        var_temp__blk936_dn8 = assign45710_e58731_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign45720_e58755, assign45720_e58755_d_n5, assign45720_e58755_d_n6, assign45720_e58755_d_n7, assign45720_e58755_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45720_e58736: f64 = (var_alp1_i / var_qim1_dc);
        let assign45720_e58737: f64 = (var_alp_i + assign45720_e58736);
        let assign45720_e58739: f64 = (assign45720_e58737 * var_qim_dc);
        let assign45720_e58741: f64 = (assign45720_e58739 / var_qim1_dc);
        let assign45720_e58743: f64 = (assign45720_e58741 * var_s1_dc);
        let assign45720_e58746: f64 = (var_alp2_i * var_qbm_dc);
        let assign45720_e58748: f64 = (assign45720_e58746 * var_temp__blk936);
        let assign45720_e58750: f64 = (assign45720_e58748 * var_temp__blk936);
        let assign45720_e58752: f64 = (assign45720_e58750 * var_s2);
        let assign45720_e58753: f64 = (assign45720_e58743 + assign45720_e58752);
        (assign45720_e58753, (((((((((-((var_alp1_i * var_qim1_dc_dn5) / (var_qim1_dc * var_qim1_dc))) * var_qim_dc) + (assign45720_e58737 * var_qim_dc_dn5)) * var_qim1_dc) - (assign45720_e58739 * var_qim1_dc_dn5)) / (var_qim1_dc * var_qim1_dc)) * var_s1_dc) + (assign45720_e58741 * var_s1_dc_dn5)) + ((((((var_alp2_i * var_qbm_dc_dn5) * var_temp__blk936) + (assign45720_e58746 * var_temp__blk936_dn5)) * var_temp__blk936) + (assign45720_e58748 * var_temp__blk936_dn5)) * var_s2)), (((((((((-((var_alp1_i * var_qim1_dc_dn6) / (var_qim1_dc * var_qim1_dc))) * var_qim_dc) + (assign45720_e58737 * var_qim_dc_dn6)) * var_qim1_dc) - (assign45720_e58739 * var_qim1_dc_dn6)) / (var_qim1_dc * var_qim1_dc)) * var_s1_dc) + (assign45720_e58741 * var_s1_dc_dn6)) + (((((((var_alp2_i * var_qbm_dc_dn6) * var_temp__blk936) + (assign45720_e58746 * var_temp__blk936_dn6)) * var_temp__blk936) + (assign45720_e58748 * var_temp__blk936_dn6)) * var_s2) + (assign45720_e58750 * var_s2_dn6))), (((((((((-((var_alp1_i * var_qim1_dc_dn7) / (var_qim1_dc * var_qim1_dc))) * var_qim_dc) + (assign45720_e58737 * var_qim_dc_dn7)) * var_qim1_dc) - (assign45720_e58739 * var_qim1_dc_dn7)) / (var_qim1_dc * var_qim1_dc)) * var_s1_dc) + (assign45720_e58741 * var_s1_dc_dn7)) + (((((((var_alp2_i * var_qbm_dc_dn7) * var_temp__blk936) + (assign45720_e58746 * var_temp__blk936_dn7)) * var_temp__blk936) + (assign45720_e58748 * var_temp__blk936_dn7)) * var_s2) + (assign45720_e58750 * var_s2_dn7))), (((((((((-((var_alp1_i * var_qim1_dc_dn8) / (var_qim1_dc * var_qim1_dc))) * var_qim_dc) + (assign45720_e58737 * var_qim_dc_dn8)) * var_qim1_dc) - (assign45720_e58739 * var_qim1_dc_dn8)) / (var_qim1_dc * var_qim1_dc)) * var_s1_dc) + (assign45720_e58741 * var_s1_dc_dn8)) + ((((((var_alp2_i * var_qbm_dc_dn8) * var_temp__blk936) + (assign45720_e58746 * var_temp__blk936_dn8)) * var_temp__blk936) + (assign45720_e58748 * var_temp__blk936_dn8)) * var_s2)),)
    } else {
        (var_dl, var_dl_dn5, var_dl_dn6, var_dl_dn7, var_dl_dn8,)
    }
};
        var_dl = assign45720_e58755;
        var_dl_dn5 = assign45720_e58755_d_n5;
        var_dl_dn6 = assign45720_e58755_d_n6;
        var_dl_dn7 = assign45720_e58755_d_n7;
        var_dl_dn8 = assign45720_e58755_d_n8;
        var_dl_rv = 0.0;

        let (assign45730_e58767, assign45730_e58767_d_n5, assign45730_e58767_d_n6, assign45730_e58767_d_n7, assign45730_e58767_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45730_e58760: f64 = (1.0 + var_dl);
        let assign45730_e58763: f64 = (var_dl * var_dl);
        let assign45730_e58764: f64 = (assign45730_e58760 + assign45730_e58763);
        let assign45730_e58765: f64 = (1.0 / assign45730_e58764);
        (assign45730_e58765, (-((var_dl_dn5 + ((var_dl_dn5 * var_dl) + (var_dl * var_dl_dn5))) / (assign45730_e58764 * assign45730_e58764))), (-((var_dl_dn6 + ((var_dl_dn6 * var_dl) + (var_dl * var_dl_dn6))) / (assign45730_e58764 * assign45730_e58764))), (-((var_dl_dn7 + ((var_dl_dn7 * var_dl) + (var_dl * var_dl_dn7))) / (assign45730_e58764 * assign45730_e58764))), (-((var_dl_dn8 + ((var_dl_dn8 * var_dl) + (var_dl * var_dl_dn8))) / (assign45730_e58764 * assign45730_e58764))),)
    } else {
        (var_gdl_dc, var_gdl_dc_dn5, var_gdl_dc_dn6, var_gdl_dc_dn7, var_gdl_dc_dn8,)
    }
};
        var_gdl_dc = assign45730_e58767;
        var_gdl_dc_dn5 = assign45730_e58767_d_n5;
        var_gdl_dc_dn6 = assign45730_e58767_d_n6;
        var_gdl_dc_dn7 = assign45730_e58767_d_n7;
        var_gdl_dc_dn8 = assign45730_e58767_d_n8;
        var_gdl_dc_rv = 0.0;

        let (assign45740_e58773, assign45740_e58773_d_n5, assign45740_e58773_d_n6, assign45740_e58773_d_n7, assign45740_e58773_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45740_e58771: f64 = (var_gmob_dc * var_gdl_dc);
        (assign45740_e58771, ((var_gmob_dc_dn5 * var_gdl_dc) + (var_gmob_dc * var_gdl_dc_dn5)), ((var_gmob_dc_dn6 * var_gdl_dc) + (var_gmob_dc * var_gdl_dc_dn6)), ((var_gmob_dc_dn7 * var_gdl_dc) + (var_gmob_dc * var_gdl_dc_dn7)), ((var_gmob_dc_dn8 * var_gdl_dc) + (var_gmob_dc * var_gdl_dc_dn8)),)
    } else {
        (var_gmob_dl_dc, var_gmob_dl_dc_dn5, var_gmob_dl_dc_dn6, var_gmob_dl_dc_dn7, var_gmob_dl_dc_dn8,)
    }
};
        var_gmob_dl_dc = assign45740_e58773;
        var_gmob_dl_dc_dn5 = assign45740_e58773_d_n5;
        var_gmob_dl_dc_dn6 = assign45740_e58773_d_n6;
        var_gmob_dl_dc_dn7 = assign45740_e58773_d_n7;
        var_gmob_dl_dc_dn8 = assign45740_e58773_d_n8;
        var_gmob_dl_dc_rv = 0.0;

        let (assign45750_e58779, assign45750_e58779_d_n5, assign45750_e58779_d_n6, assign45750_e58779_d_n7, assign45750_e58779_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45750_e58777: f64 = (var_thesateff_dc / var_gmob_dl_dc);
        (assign45750_e58777, (((var_thesateff_dc_dn5 * var_gmob_dl_dc) - (var_thesateff_dc * var_gmob_dl_dc_dn5)) / (var_gmob_dl_dc * var_gmob_dl_dc)), (((var_thesateff_dc_dn6 * var_gmob_dl_dc) - (var_thesateff_dc * var_gmob_dl_dc_dn6)) / (var_gmob_dl_dc * var_gmob_dl_dc)), (((var_thesateff_dc_dn7 * var_gmob_dl_dc) - (var_thesateff_dc * var_gmob_dl_dc_dn7)) / (var_gmob_dl_dc * var_gmob_dl_dc)), (((var_thesateff_dc_dn8 * var_gmob_dl_dc) - (var_thesateff_dc * var_gmob_dl_dc_dn8)) / (var_gmob_dl_dc * var_gmob_dl_dc)),)
    } else {
        (var_thesat1_dc, var_thesat1_dc_dn5, var_thesat1_dc_dn6, var_thesat1_dc_dn7, var_thesat1_dc_dn8,)
    }
};
        var_thesat1_dc = assign45750_e58779;
        var_thesat1_dc_dn5 = assign45750_e58779_d_n5;
        var_thesat1_dc_dn6 = assign45750_e58779_d_n6;
        var_thesat1_dc_dn7 = assign45750_e58779_d_n7;
        var_thesat1_dc_dn8 = assign45750_e58779_d_n8;
        var_thesat1_dc_rv = 0.0;

        let (assign45760_e58789, assign45760_e58789_d_n5, assign45760_e58789_d_n6, assign45760_e58789_d_n7, assign45760_e58789_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45760_e58783: f64 = (var_thesat1_dc * var_thesat1_dc);
        let assign45760_e58785: f64 = (assign45760_e58783 * var_dps_dc);
        let assign45760_e58787: f64 = (assign45760_e58785 * var_dps_dc);
        (assign45760_e58787, ((((((var_thesat1_dc_dn5 * var_thesat1_dc) + (var_thesat1_dc * var_thesat1_dc_dn5)) * var_dps_dc) + (assign45760_e58783 * var_dps_dc_dn5)) * var_dps_dc) + (assign45760_e58785 * var_dps_dc_dn5)), ((((((var_thesat1_dc_dn6 * var_thesat1_dc) + (var_thesat1_dc * var_thesat1_dc_dn6)) * var_dps_dc) + (assign45760_e58783 * var_dps_dc_dn6)) * var_dps_dc) + (assign45760_e58785 * var_dps_dc_dn6)), ((((((var_thesat1_dc_dn7 * var_thesat1_dc) + (var_thesat1_dc * var_thesat1_dc_dn7)) * var_dps_dc) + (assign45760_e58783 * var_dps_dc_dn7)) * var_dps_dc) + (assign45760_e58785 * var_dps_dc_dn7)), ((((((var_thesat1_dc_dn8 * var_thesat1_dc) + (var_thesat1_dc * var_thesat1_dc_dn8)) * var_dps_dc) + (assign45760_e58783 * var_dps_dc_dn8)) * var_dps_dc) + (assign45760_e58785 * var_dps_dc_dn8)),)
    } else {
        (var_zsat, var_zsat_dn5, var_zsat_dn6, var_zsat_dn7, var_zsat_dn8,)
    }
};
        var_zsat = assign45760_e58789;
        var_zsat_dn5 = assign45760_e58789_d_n5;
        var_zsat_dn6 = assign45760_e58789_d_n6;
        var_zsat_dn7 = assign45760_e58789_d_n7;
        var_zsat_dn8 = assign45760_e58789_d_n8;
        var_zsat_rv = 0.0;

        let assign45770_e58792: f64 = (-1.0);
        let assign45770_e58793: f64 = if var_chnl_type == assign45770_e58792 { 1.0 } else { 0.0 };
        var_guard1219 = assign45770_e58793;
        var_guard1219_rv = 0.0;

        let (assign45780_e58805, assign45780_e58805_d_n5, assign45780_e58805_d_n6, assign45780_e58805_d_n7, assign45780_e58805_d_n8,) = {
    if ((var_guard1218 != 0.0) && (var_guard1219 != 0.0)) {
        let assign45780_e58801: f64 = (var_thesat1_dc * var_dps_dc);
        let assign45780_e58802: f64 = (1.0 + assign45780_e58801);
        let assign45780_e58803: f64 = (var_zsat / assign45780_e58802);
        (assign45780_e58803, (((var_zsat_dn5 * assign45780_e58802) - (var_zsat * ((var_thesat1_dc_dn5 * var_dps_dc) + (var_thesat1_dc * var_dps_dc_dn5)))) / (assign45780_e58802 * assign45780_e58802)), (((var_zsat_dn6 * assign45780_e58802) - (var_zsat * ((var_thesat1_dc_dn6 * var_dps_dc) + (var_thesat1_dc * var_dps_dc_dn6)))) / (assign45780_e58802 * assign45780_e58802)), (((var_zsat_dn7 * assign45780_e58802) - (var_zsat * ((var_thesat1_dc_dn7 * var_dps_dc) + (var_thesat1_dc * var_dps_dc_dn7)))) / (assign45780_e58802 * assign45780_e58802)), (((var_zsat_dn8 * assign45780_e58802) - (var_zsat * ((var_thesat1_dc_dn8 * var_dps_dc) + (var_thesat1_dc * var_dps_dc_dn8)))) / (assign45780_e58802 * assign45780_e58802)),)
    } else {
        (var_zsat, var_zsat_dn5, var_zsat_dn6, var_zsat_dn7, var_zsat_dn8,)
    }
};
        var_zsat = assign45780_e58805;
        var_zsat_dn5 = assign45780_e58805_d_n5;
        var_zsat_dn6 = assign45780_e58805_d_n6;
        var_zsat_dn7 = assign45780_e58805_d_n7;
        var_zsat_dn8 = assign45780_e58805_d_n8;
        var_zsat_rv = 0.0;

        let (assign45790_e58820, assign45790_e58820_d_n5, assign45790_e58820_d_n6, assign45790_e58820_d_n7, assign45790_e58820_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45790_e58813: f64 = (2.0 * var_zsat);
        let assign45790_e58814: f64 = (1.0 + assign45790_e58813);
        let assign45790_e58815: f64 = (assign45790_e58814).sqrt();
        let assign45790_e58816: f64 = (1.0 + assign45790_e58815);
        let assign45790_e58817: f64 = (var_gmob_dl_dc * assign45790_e58816);
        let assign45790_e58818: f64 = (0.5 * assign45790_e58817);
        (assign45790_e58818, (0.5 * ((var_gmob_dl_dc_dn5 * assign45790_e58816) + (var_gmob_dl_dc * ((2.0 * var_zsat_dn5) / (2.0 * assign45790_e58815))))), (0.5 * ((var_gmob_dl_dc_dn6 * assign45790_e58816) + (var_gmob_dl_dc * ((2.0 * var_zsat_dn6) / (2.0 * assign45790_e58815))))), (0.5 * ((var_gmob_dl_dc_dn7 * assign45790_e58816) + (var_gmob_dl_dc * ((2.0 * var_zsat_dn7) / (2.0 * assign45790_e58815))))), (0.5 * ((var_gmob_dl_dc_dn8 * assign45790_e58816) + (var_gmob_dl_dc * ((2.0 * var_zsat_dn8) / (2.0 * assign45790_e58815))))),)
    } else {
        (var_gvsat, var_gvsat_dn5, var_gvsat_dn6, var_gvsat_dn7, var_gvsat_dn8,)
    }
};
        var_gvsat = assign45790_e58820;
        var_gvsat_dn5 = assign45790_e58820_d_n5;
        var_gvsat_dn6 = assign45790_e58820_d_n6;
        var_gvsat_dn7 = assign45790_e58820_d_n7;
        var_gvsat_dn8 = assign45790_e58820_d_n8;
        var_gvsat_rv = 0.0;

        let (assign45800_e58826, assign45800_e58826_d_n5, assign45800_e58826_d_n6, assign45800_e58826_d_n7, assign45800_e58826_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45800_e58824: f64 = (1.0 / var_gvsat);
        (assign45800_e58824, (-(var_gvsat_dn5 / (var_gvsat * var_gvsat))), (-(var_gvsat_dn6 / (var_gvsat * var_gvsat))), (-(var_gvsat_dn7 / (var_gvsat * var_gvsat))), (-(var_gvsat_dn8 / (var_gvsat * var_gvsat))),)
    } else {
        (var_gvsatinv_dc, var_gvsatinv_dc_dn5, var_gvsatinv_dc_dn6, var_gvsatinv_dc_dn7, var_gvsatinv_dc_dn8,)
    }
};
        var_gvsatinv_dc = assign45800_e58826;
        var_gvsatinv_dc_dn5 = assign45800_e58826_d_n5;
        var_gvsatinv_dc_dn6 = assign45800_e58826_d_n6;
        var_gvsatinv_dc_dn7 = assign45800_e58826_d_n7;
        var_gvsatinv_dc_dn8 = assign45800_e58826_d_n8;
        var_gvsatinv_dc_rv = 0.0;

        let (assign45810_e58832, assign45810_e58832_d_n5, assign45810_e58832_d_n6, assign45810_e58832_d_n7, assign45810_e58832_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45810_e58830: f64 = (var_gmob_dl_dc * var_gvsatinv_dc);
        (assign45810_e58830, ((var_gmob_dl_dc_dn5 * var_gvsatinv_dc) + (var_gmob_dl_dc * var_gvsatinv_dc_dn5)), ((var_gmob_dl_dc_dn6 * var_gvsatinv_dc) + (var_gmob_dl_dc * var_gvsatinv_dc_dn6)), ((var_gmob_dl_dc_dn7 * var_gvsatinv_dc) + (var_gmob_dl_dc * var_gvsatinv_dc_dn7)), ((var_gmob_dl_dc_dn8 * var_gvsatinv_dc) + (var_gmob_dl_dc * var_gvsatinv_dc_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign45810_e58832;
        var_temp__blk936_dn5 = assign45810_e58832_d_n5;
        var_temp__blk936_dn6 = assign45810_e58832_d_n6;
        var_temp__blk936_dn7 = assign45810_e58832_d_n7;
        var_temp__blk936_dn8 = assign45810_e58832_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign45820_e58846, assign45820_e58846_d_n5, assign45820_e58846_d_n6, assign45820_e58846_d_n7, assign45820_e58846_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45820_e58839: f64 = (var_zsat * var_temp__blk936);
        let assign45820_e58841: f64 = (assign45820_e58839 * var_temp__blk936);
        let assign45820_e58842: f64 = (0.5 * assign45820_e58841);
        let assign45820_e58843: f64 = (1.0 + assign45820_e58842);
        let assign45820_e58844: f64 = (var_alpha_dc * assign45820_e58843);
        (assign45820_e58844, ((var_alpha_dc_dn5 * assign45820_e58843) + (var_alpha_dc * (0.5 * ((((var_zsat_dn5 * var_temp__blk936) + (var_zsat * var_temp__blk936_dn5)) * var_temp__blk936) + (assign45820_e58839 * var_temp__blk936_dn5))))), ((var_alpha_dc_dn6 * assign45820_e58843) + (var_alpha_dc * (0.5 * ((((var_zsat_dn6 * var_temp__blk936) + (var_zsat * var_temp__blk936_dn6)) * var_temp__blk936) + (assign45820_e58839 * var_temp__blk936_dn6))))), ((var_alpha_dc_dn7 * assign45820_e58843) + (var_alpha_dc * (0.5 * ((((var_zsat_dn7 * var_temp__blk936) + (var_zsat * var_temp__blk936_dn7)) * var_temp__blk936) + (assign45820_e58839 * var_temp__blk936_dn7))))), ((var_alpha_dc_dn8 * assign45820_e58843) + (var_alpha_dc * (0.5 * ((((var_zsat_dn8 * var_temp__blk936) + (var_zsat * var_temp__blk936_dn8)) * var_temp__blk936) + (assign45820_e58839 * var_temp__blk936_dn8))))),)
    } else {
        (var_alpha1, var_alpha1_dn5, var_alpha1_dn6, var_alpha1_dn7, var_alpha1_dn8,)
    }
};
        var_alpha1 = assign45820_e58846;
        var_alpha1_dn5 = assign45820_e58846_d_n5;
        var_alpha1_dn6 = assign45820_e58846_d_n6;
        var_alpha1_dn7 = assign45820_e58846_d_n7;
        var_alpha1_dn8 = assign45820_e58846_d_n8;
        var_alpha1_rv = 0.0;

        let (assign45830_e58854, assign45830_e58854_d_n5, assign45830_e58854_d_n6, assign45830_e58854_d_n7, assign45830_e58854_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45830_e58850: f64 = (var_temp__blk936 * var_qim1_dc);
        let assign45830_e58852: f64 = (assign45830_e58850 / var_alpha1);
        (assign45830_e58852, (((((var_temp__blk936_dn5 * var_qim1_dc) + (var_temp__blk936 * var_qim1_dc_dn5)) * var_alpha1) - (assign45830_e58850 * var_alpha1_dn5)) / (var_alpha1 * var_alpha1)), (((((var_temp__blk936_dn6 * var_qim1_dc) + (var_temp__blk936 * var_qim1_dc_dn6)) * var_alpha1) - (assign45830_e58850 * var_alpha1_dn6)) / (var_alpha1 * var_alpha1)), (((((var_temp__blk936_dn7 * var_qim1_dc) + (var_temp__blk936 * var_qim1_dc_dn7)) * var_alpha1) - (assign45830_e58850 * var_alpha1_dn7)) / (var_alpha1 * var_alpha1)), (((((var_temp__blk936_dn8 * var_qim1_dc) + (var_temp__blk936 * var_qim1_dc_dn8)) * var_alpha1) - (assign45830_e58850 * var_alpha1_dn8)) / (var_alpha1 * var_alpha1)),)
    } else {
        (var_h_dc, var_h_dc_dn5, var_h_dc_dn6, var_h_dc_dn7, var_h_dc_dn8,)
    }
};
        var_h_dc = assign45830_e58854;
        var_h_dc_dn5 = assign45830_e58854_d_n5;
        var_h_dc_dn6 = assign45830_e58854_d_n6;
        var_h_dc_dn7 = assign45830_e58854_d_n7;
        var_h_dc_dn8 = assign45830_e58854_d_n8;
        var_h_dc_rv = 0.0;

        let (assign45840_e58864, assign45840_e58864_d_n5, assign45840_e58864_d_n6, assign45840_e58864_d_n7, assign45840_e58864_d_n8,) = {
    if (var_guard1218 != 0.0) {
        let assign45840_e58858: f64 = (var_bet_i * var_qim1_dc);
        let assign45840_e58860: f64 = (assign45840_e58858 * var_dps_dc);
        let assign45840_e58862: f64 = (assign45840_e58860 * var_gvsatinv_dc);
        (assign45840_e58862, (((((var_bet_i * var_qim1_dc_dn5) * var_dps_dc) + (assign45840_e58858 * var_dps_dc_dn5)) * var_gvsatinv_dc) + (assign45840_e58860 * var_gvsatinv_dc_dn5)), (((((var_bet_i * var_qim1_dc_dn6) * var_dps_dc) + (assign45840_e58858 * var_dps_dc_dn6)) * var_gvsatinv_dc) + (assign45840_e58860 * var_gvsatinv_dc_dn6)), (((((var_bet_i * var_qim1_dc_dn7) * var_dps_dc) + (assign45840_e58858 * var_dps_dc_dn7)) * var_gvsatinv_dc) + (assign45840_e58860 * var_gvsatinv_dc_dn7)), (((((var_bet_i * var_qim1_dc_dn8) * var_dps_dc) + (assign45840_e58858 * var_dps_dc_dn8)) * var_gvsatinv_dc) + (assign45840_e58860 * var_gvsatinv_dc_dn8)),)
    } else {
        (var_i_ds, var_i_ds_dn5, var_i_ds_dn6, var_i_ds_dn7, var_i_ds_dn8,)
    }
};
        var_i_ds = assign45840_e58864;
        var_i_ds_dn5 = assign45840_e58864_d_n5;
        var_i_ds_dn6 = assign45840_e58864_d_n6;
        var_i_ds_dn7 = assign45840_e58864_d_n7;
        var_i_ds_dn8 = assign45840_e58864_d_n8;
        var_i_ds_rv = 0.0;

        var_xs_ov = 0.0;
        var_xs_ov_dn5 = 0.0;
        var_xs_ov_dn6 = 0.0;
        var_xs_ov_dn7 = 0.0;
        var_xs_ov_rv = 0.0;

        var_xd_ov = 0.0;
        var_xd_ov_dn5 = 0.0;
        var_xd_ov_dn6 = 0.0;
        var_xd_ov_dn7 = 0.0;
        var_xd_ov_rv = 0.0;

        var_vovs = 0.0;
        var_vovs_dn5 = 0.0;
        var_vovs_dn6 = 0.0;
        var_vovs_dn7 = 0.0;
        var_vovs_rv = 0.0;

        var_vovd = 0.0;
        var_vovd_dn5 = 0.0;
        var_vovd_dn6 = 0.0;
        var_vovd_dn7 = 0.0;
        var_vovd_rv = 0.0;

        let assign45890_e58899: f64 = if (((((p.p40 != 0.0) && ((var_igov_i > 0.0) || (var_igovd_i > 0.0))) || ((p.p42 != 0.0) && ((var_agidl_i > 0.0) || (var_agidld_i > 0.0)))) || (var_cgov_i > 0.0)) || (var_cgovd_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1220 = assign45890_e58899;
        var_guard1220_rv = 0.0;

        let (assign45900_e58912, assign45900_e58912_d_n5, assign45900_e58912_d_n6, assign45900_e58912_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45900_e58905: f64 = (var_xgs_ov * var_xgs_ov);
        let assign45900_e58907: f64 = (assign45900_e58905 + var_sp_ov_eps2_s);
        let assign45900_e58908: f64 = (assign45900_e58907).sqrt();
        let assign45900_e58909: f64 = (var_xgs_ov + assign45900_e58908);
        let assign45900_e58910: f64 = (0.5 * assign45900_e58909);
        (assign45900_e58910, (0.5 * (var_xgs_ov_dn5 + (((var_xgs_ov_dn5 * var_xgs_ov) + (var_xgs_ov * var_xgs_ov_dn5)) / (2.0 * assign45900_e58908)))), (0.5 * (var_xgs_ov_dn6 + (((var_xgs_ov_dn6 * var_xgs_ov) + (var_xgs_ov * var_xgs_ov_dn6)) / (2.0 * assign45900_e58908)))), (0.5 * (var_xgs_ov_dn7 + (((var_xgs_ov_dn7 * var_xgs_ov) + (var_xgs_ov * var_xgs_ov_dn7)) / (2.0 * assign45900_e58908)))),)
    } else {
        (var_sp_ov_xg, var_sp_ov_xg_dn5, var_sp_ov_xg_dn6, var_sp_ov_xg_dn7,)
    }
};
        var_sp_ov_xg = assign45900_e58912;
        var_sp_ov_xg_dn5 = assign45900_e58912_d_n5;
        var_sp_ov_xg_dn6 = assign45900_e58912_d_n6;
        var_sp_ov_xg_dn7 = assign45900_e58912_d_n7;
        var_sp_ov_xg_rv = 0.0;

        let (assign45910_e58934, assign45910_e58934_d_n5, assign45910_e58934_d_n6, assign45910_e58934_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45910_e58915: f64 = (-var_sp_ov_xg);
        let assign45910_e58918: f64 = (var_gov2_s * 0.5);
        let assign45910_e58919: f64 = (assign45910_e58915 - assign45910_e58918);
        let assign45910_e58924: f64 = (var_gov2_s * 0.25);
        let assign45910_e58925: f64 = (var_sp_ov_xg + assign45910_e58924);
        let assign45910_e58927: f64 = (assign45910_e58925 + var_sp_ov_a_s);
        let assign45910_e58928: f64 = (assign45910_e58927).sqrt();
        let assign45910_e58929: f64 = (var_gov_s * assign45910_e58928);
        let assign45910_e58930: f64 = (assign45910_e58919 + assign45910_e58929);
        let assign45910_e58932: f64 = (assign45910_e58930 + var_sp_ov_delta1_s);
        (assign45910_e58932, ((-var_sp_ov_xg_dn5) + (var_gov_s * (var_sp_ov_xg_dn5 / (2.0 * assign45910_e58928)))), ((-var_sp_ov_xg_dn6) + (var_gov_s * (var_sp_ov_xg_dn6 / (2.0 * assign45910_e58928)))), ((-var_sp_ov_xg_dn7) + (var_gov_s * (var_sp_ov_xg_dn7 / (2.0 * assign45910_e58928)))),)
    } else {
        (var_xs_ov, var_xs_ov_dn5, var_xs_ov_dn6, var_xs_ov_dn7,)
    }
};
        var_xs_ov = assign45910_e58934;
        var_xs_ov_dn5 = assign45910_e58934_d_n5;
        var_xs_ov_dn6 = assign45910_e58934_d_n6;
        var_xs_ov_dn7 = assign45910_e58934_d_n7;
        var_xs_ov_rv = 0.0;

        let (assign45920_e58947, assign45920_e58947_d_n5, assign45920_e58947_d_n6, assign45920_e58947_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45920_e58940: f64 = (var_xgd_ov * var_xgd_ov);
        let assign45920_e58942: f64 = (assign45920_e58940 + var_sp_ov_eps2_d);
        let assign45920_e58943: f64 = (assign45920_e58942).sqrt();
        let assign45920_e58944: f64 = (var_xgd_ov + assign45920_e58943);
        let assign45920_e58945: f64 = (0.5 * assign45920_e58944);
        (assign45920_e58945, (0.5 * (var_xgd_ov_dn5 + (((var_xgd_ov_dn5 * var_xgd_ov) + (var_xgd_ov * var_xgd_ov_dn5)) / (2.0 * assign45920_e58943)))), (0.5 * (var_xgd_ov_dn6 + (((var_xgd_ov_dn6 * var_xgd_ov) + (var_xgd_ov * var_xgd_ov_dn6)) / (2.0 * assign45920_e58943)))), (0.5 * (var_xgd_ov_dn7 + (((var_xgd_ov_dn7 * var_xgd_ov) + (var_xgd_ov * var_xgd_ov_dn7)) / (2.0 * assign45920_e58943)))),)
    } else {
        (var_sp_ov_xg, var_sp_ov_xg_dn5, var_sp_ov_xg_dn6, var_sp_ov_xg_dn7,)
    }
};
        var_sp_ov_xg = assign45920_e58947;
        var_sp_ov_xg_dn5 = assign45920_e58947_d_n5;
        var_sp_ov_xg_dn6 = assign45920_e58947_d_n6;
        var_sp_ov_xg_dn7 = assign45920_e58947_d_n7;
        var_sp_ov_xg_rv = 0.0;

        let (assign45930_e58969, assign45930_e58969_d_n5, assign45930_e58969_d_n6, assign45930_e58969_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45930_e58950: f64 = (-var_sp_ov_xg);
        let assign45930_e58953: f64 = (var_gov2_d * 0.5);
        let assign45930_e58954: f64 = (assign45930_e58950 - assign45930_e58953);
        let assign45930_e58959: f64 = (var_gov2_d * 0.25);
        let assign45930_e58960: f64 = (var_sp_ov_xg + assign45930_e58959);
        let assign45930_e58962: f64 = (assign45930_e58960 + var_sp_ov_a_d);
        let assign45930_e58963: f64 = (assign45930_e58962).sqrt();
        let assign45930_e58964: f64 = (var_gov_d * assign45930_e58963);
        let assign45930_e58965: f64 = (assign45930_e58954 + assign45930_e58964);
        let assign45930_e58967: f64 = (assign45930_e58965 + var_sp_ov_delta1_d);
        (assign45930_e58967, ((-var_sp_ov_xg_dn5) + (var_gov_d * (var_sp_ov_xg_dn5 / (2.0 * assign45930_e58963)))), ((-var_sp_ov_xg_dn6) + (var_gov_d * (var_sp_ov_xg_dn6 / (2.0 * assign45930_e58963)))), ((-var_sp_ov_xg_dn7) + (var_gov_d * (var_sp_ov_xg_dn7 / (2.0 * assign45930_e58963)))),)
    } else {
        (var_xd_ov, var_xd_ov_dn5, var_xd_ov_dn6, var_xd_ov_dn7,)
    }
};
        var_xd_ov = assign45930_e58969;
        var_xd_ov_dn5 = assign45930_e58969_d_n5;
        var_xd_ov_dn6 = assign45930_e58969_d_n6;
        var_xd_ov_dn7 = assign45930_e58969_d_n7;
        var_xd_ov_rv = 0.0;

        let (assign45940_e58978, assign45940_e58978_d_n5, assign45940_e58978_d_n6, assign45940_e58978_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45940_e58972: f64 = (-var_phita);
        let assign45940_e58975: f64 = (var_xgs_ov + var_xs_ov);
        let assign45940_e58976: f64 = (assign45940_e58972 * assign45940_e58975);
        (assign45940_e58976, (assign45940_e58972 * (var_xgs_ov_dn5 + var_xs_ov_dn5)), (assign45940_e58972 * (var_xgs_ov_dn6 + var_xs_ov_dn6)), (assign45940_e58972 * (var_xgs_ov_dn7 + var_xs_ov_dn7)),)
    } else {
        (var_vovs, var_vovs_dn5, var_vovs_dn6, var_vovs_dn7,)
    }
};
        var_vovs = assign45940_e58978;
        var_vovs_dn5 = assign45940_e58978_d_n5;
        var_vovs_dn6 = assign45940_e58978_d_n6;
        var_vovs_dn7 = assign45940_e58978_d_n7;
        var_vovs_rv = 0.0;

        let (assign45950_e58987, assign45950_e58987_d_n5, assign45950_e58987_d_n6, assign45950_e58987_d_n7,) = {
    if (var_guard1220 != 0.0) {
        let assign45950_e58981: f64 = (-var_phita);
        let assign45950_e58984: f64 = (var_xgd_ov + var_xd_ov);
        let assign45950_e58985: f64 = (assign45950_e58981 * assign45950_e58984);
        (assign45950_e58985, (assign45950_e58981 * (var_xgd_ov_dn5 + var_xd_ov_dn5)), (assign45950_e58981 * (var_xgd_ov_dn6 + var_xd_ov_dn6)), (assign45950_e58981 * (var_xgd_ov_dn7 + var_xd_ov_dn7)),)
    } else {
        (var_vovd, var_vovd_dn5, var_vovd_dn6, var_vovd_dn7,)
    }
};
        var_vovd = assign45950_e58987;
        var_vovd_dn5 = assign45950_e58987_d_n5;
        var_vovd_dn6 = assign45950_e58987_d_n6;
        var_vovd_dn7 = assign45950_e58987_d_n7;
        var_vovd_rv = 0.0;

        let assign46020_e58996: f64 = if p.p40 != 0.0 { 1.0 } else { 0.0 };
        var_guard1221 = assign46020_e58996;
        var_guard1221_rv = 0.0;

        let assign46030_e58999: f64 = if var_igov_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1222 = assign46030_e58999;
        var_guard1222_rv = 0.0;

        let (assign46040_e59012, assign46040_e59012_d_n5, assign46040_e59012_d_n6, assign46040_e59012_d_n7, assign46040_e59012_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46040_e59005: f64 = (var_vovs * var_vovs);
        let assign46040_e59007: f64 = (assign46040_e59005 + 1e-6);
        let assign46040_e59008: f64 = (assign46040_e59007).sqrt();
        let assign46040_e59010: f64 = (assign46040_e59008 * var_inv_chib);
        (assign46040_e59010, ((((var_vovs_dn5 * var_vovs) + (var_vovs * var_vovs_dn5)) / (2.0 * assign46040_e59008)) * var_inv_chib), ((((var_vovs_dn6 * var_vovs) + (var_vovs * var_vovs_dn6)) / (2.0 * assign46040_e59008)) * var_inv_chib), ((((var_vovs_dn7 * var_vovs) + (var_vovs * var_vovs_dn7)) / (2.0 * assign46040_e59008)) * var_inv_chib), 0.0,)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46040_e59012;
        var_zg_dn5 = assign46040_e59012_d_n5;
        var_zg_dn6 = assign46040_e59012_d_n6;
        var_zg_dn7 = assign46040_e59012_d_n7;
        var_zg_dn8 = assign46040_e59012_d_n8;
        var_zg_rv = 0.0;

        let assign46050_e59015: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1223 = assign46050_e59015;
        var_guard1223_rv = 0.0;

        let (assign46060_e59038, assign46060_e59038_d_n5, assign46060_e59038_d_n6, assign46060_e59038_d_n7, assign46060_e59038_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) && (var_guard1223 != 0.0)) {
        let assign46060_e59024: f64 = (var_zg + var_gcqov);
        let assign46060_e59027: f64 = (var_zg - var_gcqov);
        let assign46060_e59030: f64 = (var_zg - var_gcqov);
        let assign46060_e59031: f64 = (assign46060_e59027 * assign46060_e59030);
        let assign46060_e59033: f64 = (assign46060_e59031 + 1e-6);
        let assign46060_e59034: f64 = (assign46060_e59033).sqrt();
        let assign46060_e59035: f64 = (assign46060_e59024 - assign46060_e59034);
        let assign46060_e59036: f64 = (0.5 * assign46060_e59035);
        (assign46060_e59036, (0.5 * (var_zg_dn5 - (((var_zg_dn5 * assign46060_e59030) + (assign46060_e59027 * var_zg_dn5)) / (2.0 * assign46060_e59034)))), (0.5 * (var_zg_dn6 - (((var_zg_dn6 * assign46060_e59030) + (assign46060_e59027 * var_zg_dn6)) / (2.0 * assign46060_e59034)))), (0.5 * (var_zg_dn7 - (((var_zg_dn7 * assign46060_e59030) + (assign46060_e59027 * var_zg_dn7)) / (2.0 * assign46060_e59034)))), (0.5 * (var_zg_dn8 - (((var_zg_dn8 * assign46060_e59030) + (assign46060_e59027 * var_zg_dn8)) / (2.0 * assign46060_e59034)))),)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46060_e59038;
        var_zg_dn5 = assign46060_e59038_d_n5;
        var_zg_dn6 = assign46060_e59038_d_n6;
        var_zg_dn7 = assign46060_e59038_d_n7;
        var_zg_dn8 = assign46060_e59038_d_n8;
        var_zg_rv = 0.0;

        let (assign46070_e59055, assign46070_e59055_d_n5, assign46070_e59055_d_n6, assign46070_e59055_d_n7, assign46070_e59055_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46070_e59044: f64 = (-1.5);
        let assign46070_e59049: f64 = (var_gc3ov_i * var_zg);
        let assign46070_e59050: f64 = (var_gc2ov_i + assign46070_e59049);
        let assign46070_e59051: f64 = (var_zg * assign46070_e59050);
        let assign46070_e59052: f64 = (assign46070_e59044 + assign46070_e59051);
        let assign46070_e59053: f64 = (var_bov * assign46070_e59052);
        (assign46070_e59053, (var_bov * ((var_zg_dn5 * assign46070_e59050) + (var_zg * (var_gc3ov_i * var_zg_dn5)))), (var_bov * ((var_zg_dn6 * assign46070_e59050) + (var_zg * (var_gc3ov_i * var_zg_dn6)))), (var_bov * ((var_zg_dn7 * assign46070_e59050) + (var_zg * (var_gc3ov_i * var_zg_dn7)))), (var_bov * ((var_zg_dn8 * assign46070_e59050) + (var_zg * (var_gc3ov_i * var_zg_dn8)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46070_e59055;
        var_temp__blk936_dn5 = assign46070_e59055_d_n5;
        var_temp__blk936_dn6 = assign46070_e59055_d_n6;
        var_temp__blk936_dn7 = assign46070_e59055_d_n7;
        var_temp__blk936_dn8 = assign46070_e59055_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46130_e59141, assign46130_e59141_d_n5, assign46130_e59141_d_n6, assign46130_e59141_d_n7,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46130_e59139: f64 = (3.0 + var_xs_ov);
        (assign46130_e59139, var_xs_ov_dn5, var_xs_ov_dn6, var_xs_ov_dn7,)
    } else {
        (var_fs1, var_fs1_dn5, var_fs1_dn6, var_fs1_dn7,)
    }
};
        var_fs1 = assign46130_e59141;
        var_fs1_dn5 = assign46130_e59141_d_n5;
        var_fs1_dn6 = assign46130_e59141_d_n6;
        var_fs1_dn7 = assign46130_e59141_d_n7;
        var_fs1_rv = 0.0;

        let (assign46140_e59150,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46140_e59146: f64 = (-3.0);
        let assign46140_e59148: f64 = (assign46140_e59146 - var_gco_i);
        (assign46140_e59148,)
    } else {
        (var_fs2,)
    }
};
        var_fs2 = assign46140_e59150;
        var_fs2_rv = 0.0;

        let (assign46150_e59158, assign46150_e59158_d_n5, assign46150_e59158_d_n6, assign46150_e59158_d_n7,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46150_e59156: f64 = (30.0 * var_vgsprime);
        (assign46150_e59156, (30.0 * var_vgsprime_dn5), (30.0 * var_vgsprime_dn6), (30.0 * var_vgsprime_dn7),)
    } else {
        (var_fs3, var_fs3_dn5, var_fs3_dn6, var_fs3_dn7,)
    }
};
        var_fs3 = assign46150_e59158;
        var_fs3_dn5 = assign46150_e59158_d_n5;
        var_fs3_dn6 = assign46150_e59158_d_n6;
        var_fs3_dn7 = assign46150_e59158_d_n7;
        var_fs3_rv = 0.0;

        let (assign46160_e59166,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46160_e59164: f64 = (4.0 - 0.9);
        (assign46160_e59164,)
    } else {
        (var_tme1,)
    }
};
        var_tme1 = assign46160_e59166;
        var_tme1_rv = 0.0;

        let (assign46170_e59174, assign46170_e59174_d_n5, assign46170_e59174_d_n6, assign46170_e59174_d_n7, assign46170_e59174_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46170_e59172: f64 = (var_fs1 + var_fs3);
        (assign46170_e59172, (var_fs1_dn5 + var_fs3_dn5), (var_fs1_dn6 + var_fs3_dn6), (var_fs1_dn7 + var_fs3_dn7), 0.0,)
    } else {
        (var_tme2, var_tme2_dn5, var_tme2_dn6, var_tme2_dn7, var_tme2_dn8,)
    }
};
        var_tme2 = assign46170_e59174;
        var_tme2_dn5 = assign46170_e59174_d_n5;
        var_tme2_dn6 = assign46170_e59174_d_n6;
        var_tme2_dn7 = assign46170_e59174_d_n7;
        var_tme2_dn8 = assign46170_e59174_d_n8;
        var_tme2_rv = 0.0;

        *var_alpha1_slot = var_alpha1;
        *var_alpha1_dn5_slot = var_alpha1_dn5;
        *var_alpha1_dn6_slot = var_alpha1_dn6;
        *var_alpha1_dn7_slot = var_alpha1_dn7;
        *var_alpha1_dn8_slot = var_alpha1_dn8;
        *var_alpha1_rv_slot = var_alpha1_rv;
        *var_dl_slot = var_dl;
        *var_dl_dn5_slot = var_dl_dn5;
        *var_dl_dn6_slot = var_dl_dn6;
        *var_dl_dn7_slot = var_dl_dn7;
        *var_dl_dn8_slot = var_dl_dn8;
        *var_dl_rv_slot = var_dl_rv;
        *var_fs1_slot = var_fs1;
        *var_fs1_dn5_slot = var_fs1_dn5;
        *var_fs1_dn6_slot = var_fs1_dn6;
        *var_fs1_dn7_slot = var_fs1_dn7;
        *var_fs1_rv_slot = var_fs1_rv;
        *var_fs2_slot = var_fs2;
        *var_fs2_rv_slot = var_fs2_rv;
        *var_fs3_slot = var_fs3;
        *var_fs3_dn5_slot = var_fs3_dn5;
        *var_fs3_dn6_slot = var_fs3_dn6;
        *var_fs3_dn7_slot = var_fs3_dn7;
        *var_fs3_rv_slot = var_fs3_rv;
        *var_gdl_dc_slot = var_gdl_dc;
        *var_gdl_dc_dn5_slot = var_gdl_dc_dn5;
        *var_gdl_dc_dn6_slot = var_gdl_dc_dn6;
        *var_gdl_dc_dn7_slot = var_gdl_dc_dn7;
        *var_gdl_dc_dn8_slot = var_gdl_dc_dn8;
        *var_gdl_dc_rv_slot = var_gdl_dc_rv;
        *var_gmob_dl_dc_slot = var_gmob_dl_dc;
        *var_gmob_dl_dc_dn5_slot = var_gmob_dl_dc_dn5;
        *var_gmob_dl_dc_dn6_slot = var_gmob_dl_dc_dn6;
        *var_gmob_dl_dc_dn7_slot = var_gmob_dl_dc_dn7;
        *var_gmob_dl_dc_dn8_slot = var_gmob_dl_dc_dn8;
        *var_gmob_dl_dc_rv_slot = var_gmob_dl_dc_rv;
        *var_guard1219_slot = var_guard1219;
        *var_guard1219_rv_slot = var_guard1219_rv;
        *var_guard1220_slot = var_guard1220;
        *var_guard1220_rv_slot = var_guard1220_rv;
        *var_guard1221_slot = var_guard1221;
        *var_guard1221_rv_slot = var_guard1221_rv;
        *var_guard1222_slot = var_guard1222;
        *var_guard1222_rv_slot = var_guard1222_rv;
        *var_guard1223_slot = var_guard1223;
        *var_guard1223_rv_slot = var_guard1223_rv;
        *var_gvsat_slot = var_gvsat;
        *var_gvsat_dn5_slot = var_gvsat_dn5;
        *var_gvsat_dn6_slot = var_gvsat_dn6;
        *var_gvsat_dn7_slot = var_gvsat_dn7;
        *var_gvsat_dn8_slot = var_gvsat_dn8;
        *var_gvsat_rv_slot = var_gvsat_rv;
        *var_gvsatinv_dc_slot = var_gvsatinv_dc;
        *var_gvsatinv_dc_dn5_slot = var_gvsatinv_dc_dn5;
        *var_gvsatinv_dc_dn6_slot = var_gvsatinv_dc_dn6;
        *var_gvsatinv_dc_dn7_slot = var_gvsatinv_dc_dn7;
        *var_gvsatinv_dc_dn8_slot = var_gvsatinv_dc_dn8;
        *var_gvsatinv_dc_rv_slot = var_gvsatinv_dc_rv;
        *var_h_dc_slot = var_h_dc;
        *var_h_dc_dn5_slot = var_h_dc_dn5;
        *var_h_dc_dn6_slot = var_h_dc_dn6;
        *var_h_dc_dn7_slot = var_h_dc_dn7;
        *var_h_dc_dn8_slot = var_h_dc_dn8;
        *var_h_dc_rv_slot = var_h_dc_rv;
        *var_i_ds_slot = var_i_ds;
        *var_i_ds_dn5_slot = var_i_ds_dn5;
        *var_i_ds_dn6_slot = var_i_ds_dn6;
        *var_i_ds_dn7_slot = var_i_ds_dn7;
        *var_i_ds_dn8_slot = var_i_ds_dn8;
        *var_i_ds_rv_slot = var_i_ds_rv;
        *var_sp_ov_xg_slot = var_sp_ov_xg;
        *var_sp_ov_xg_dn5_slot = var_sp_ov_xg_dn5;
        *var_sp_ov_xg_dn6_slot = var_sp_ov_xg_dn6;
        *var_sp_ov_xg_dn7_slot = var_sp_ov_xg_dn7;
        *var_sp_ov_xg_rv_slot = var_sp_ov_xg_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_thesat1_dc_slot = var_thesat1_dc;
        *var_thesat1_dc_dn5_slot = var_thesat1_dc_dn5;
        *var_thesat1_dc_dn6_slot = var_thesat1_dc_dn6;
        *var_thesat1_dc_dn7_slot = var_thesat1_dc_dn7;
        *var_thesat1_dc_dn8_slot = var_thesat1_dc_dn8;
        *var_thesat1_dc_rv_slot = var_thesat1_dc_rv;
        *var_tme1_slot = var_tme1;
        *var_tme1_rv_slot = var_tme1_rv;
        *var_tme2_slot = var_tme2;
        *var_tme2_dn5_slot = var_tme2_dn5;
        *var_tme2_dn6_slot = var_tme2_dn6;
        *var_tme2_dn7_slot = var_tme2_dn7;
        *var_tme2_dn8_slot = var_tme2_dn8;
        *var_tme2_rv_slot = var_tme2_rv;
        *var_vovd_slot = var_vovd;
        *var_vovd_dn5_slot = var_vovd_dn5;
        *var_vovd_dn6_slot = var_vovd_dn6;
        *var_vovd_dn7_slot = var_vovd_dn7;
        *var_vovd_rv_slot = var_vovd_rv;
        *var_vovs_slot = var_vovs;
        *var_vovs_dn5_slot = var_vovs_dn5;
        *var_vovs_dn6_slot = var_vovs_dn6;
        *var_vovs_dn7_slot = var_vovs_dn7;
        *var_vovs_rv_slot = var_vovs_rv;
        *var_xd_ov_slot = var_xd_ov;
        *var_xd_ov_dn5_slot = var_xd_ov_dn5;
        *var_xd_ov_dn6_slot = var_xd_ov_dn6;
        *var_xd_ov_dn7_slot = var_xd_ov_dn7;
        *var_xd_ov_rv_slot = var_xd_ov_rv;
        *var_xs_ov_slot = var_xs_ov;
        *var_xs_ov_dn5_slot = var_xs_ov_dn5;
        *var_xs_ov_dn6_slot = var_xs_ov_dn6;
        *var_xs_ov_dn7_slot = var_xs_ov_dn7;
        *var_xs_ov_rv_slot = var_xs_ov_rv;
        *var_zg_slot = var_zg;
        *var_zg_dn5_slot = var_zg_dn5;
        *var_zg_dn6_slot = var_zg_dn6;
        *var_zg_dn7_slot = var_zg_dn7;
        *var_zg_dn8_slot = var_zg_dn8;
        *var_zg_rv_slot = var_zg_rv;
        *var_zsat_slot = var_zsat;
        *var_zsat_dn5_slot = var_zsat_dn5;
        *var_zsat_dn6_slot = var_zsat_dn6;
        *var_zsat_dn7_slot = var_zsat_dn7;
        *var_zsat_dn8_slot = var_zsat_dn8;
        *var_zsat_rv_slot = var_zsat_rv;
    }

    pub(super) fn stamp_reactive_block_38(
        var_alpha_b: f64,
        var_ar: f64,
        var_bov_d: f64,
        var_gc2ovd_i: f64,
        var_gc3_i: f64,
        var_gc3ovd_i: f64,
        var_gco_i: f64,
        var_gcq: f64,
        var_gcqovd: f64,
        var_guard1221: f64,
        var_guard1222: f64,
        var_iginv_i: f64,
        var_igovd_i: f64,
        var_inv_chib: f64,
        var_inv_phit1_dc: f64,
        var_inv_phit1_dc_dn5: f64,
        var_inv_phit1_dc_dn6: f64,
        var_inv_phit1_dc_dn7: f64,
        var_inv_phit1_dc_dn8: f64,
        var_phit1_dc: f64,
        var_phit1_dc_dn5: f64,
        var_phit1_dc_dn6: f64,
        var_phit1_dc_dn7: f64,
        var_phit1_dc_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsat_lim_dc: f64,
        var_vdsat_lim_dc_dn5: f64,
        var_vdsat_lim_dc_dn6: f64,
        var_vdsat_lim_dc_dn7: f64,
        var_vdsat_lim_dc_dn8: f64,
        var_vgdprime: f64,
        var_vgdprime_dn5: f64,
        var_vgdprime_dn6: f64,
        var_vgdprime_dn7: f64,
        var_vovd: f64,
        var_vovd_dn5: f64,
        var_vovd_dn6: f64,
        var_vovd_dn7: f64,
        var_voxm_dc: f64,
        var_voxm_dc_dn5: f64,
        var_voxm_dc_dn6: f64,
        var_voxm_dc_dn7: f64,
        var_voxm_dc_dn8: f64,
        var_vsbstar_dc: f64,
        var_vsbstar_dc_dn5: f64,
        var_vsbstar_dc_dn6: f64,
        var_vsbstar_dc_dn7: f64,
        var_vsbstar_dc_dn8: f64,
        var_x_ds_dc: f64,
        var_x_ds_dc_dn5: f64,
        var_x_ds_dc_dn6: f64,
        var_x_ds_dc_dn7: f64,
        var_x_ds_dc_dn8: f64,
        var_x_m_dc: f64,
        var_x_m_dc_dn5: f64,
        var_x_m_dc_dn6: f64,
        var_x_m_dc_dn7: f64,
        var_x_m_dc_dn8: f64,
        var_xd_ov: f64,
        var_xd_ov_dn5: f64,
        var_xd_ov_dn6: f64,
        var_xd_ov_dn7: f64,
        var_xg_dc: f64,
        var_arg1_slot: &mut f64,
        var_arg1_dn5_slot: &mut f64,
        var_arg1_dn6_slot: &mut f64,
        var_arg1_dn7_slot: &mut f64,
        var_arg1_dn8_slot: &mut f64,
        var_arg1_rv_slot: &mut f64,
        var_arg2mina_slot: &mut f64,
        var_arg2mina_dn5_slot: &mut f64,
        var_arg2mina_dn6_slot: &mut f64,
        var_arg2mina_dn7_slot: &mut f64,
        var_arg2mina_dn8_slot: &mut f64,
        var_arg2mina_rv_slot: &mut f64,
        var_dch_slot: &mut f64,
        var_dch_dn5_slot: &mut f64,
        var_dch_dn6_slot: &mut f64,
        var_dch_dn7_slot: &mut f64,
        var_dch_dn8_slot: &mut f64,
        var_dch_rv_slot: &mut f64,
        var_fs1_slot: &mut f64,
        var_fs1_dn5_slot: &mut f64,
        var_fs1_dn6_slot: &mut f64,
        var_fs1_dn7_slot: &mut f64,
        var_fs1_rv_slot: &mut f64,
        var_fs2_slot: &mut f64,
        var_fs2_rv_slot: &mut f64,
        var_fs3_slot: &mut f64,
        var_fs3_dn5_slot: &mut f64,
        var_fs3_dn6_slot: &mut f64,
        var_fs3_dn7_slot: &mut f64,
        var_fs3_rv_slot: &mut f64,
        var_guard1226_slot: &mut f64,
        var_guard1226_rv_slot: &mut f64,
        var_guard1227_slot: &mut f64,
        var_guard1227_rv_slot: &mut f64,
        var_guard1230_slot: &mut f64,
        var_guard1230_rv_slot: &mut f64,
        var_guard1231_slot: &mut f64,
        var_guard1231_rv_slot: &mut f64,
        var_guard1232_slot: &mut f64,
        var_guard1232_rv_slot: &mut f64,
        var_guard1233_slot: &mut f64,
        var_guard1233_rv_slot: &mut f64,
        var_psi_t_slot: &mut f64,
        var_psi_t_dn5_slot: &mut f64,
        var_psi_t_dn6_slot: &mut f64,
        var_psi_t_dn7_slot: &mut f64,
        var_psi_t_dn8_slot: &mut f64,
        var_psi_t_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_tme1_slot: &mut f64,
        var_tme1_rv_slot: &mut f64,
        var_tme2_slot: &mut f64,
        var_tme2_dn5_slot: &mut f64,
        var_tme2_dn6_slot: &mut f64,
        var_tme2_dn7_slot: &mut f64,
        var_tme2_dn8_slot: &mut f64,
        var_tme2_rv_slot: &mut f64,
        var_udse_dc_slot: &mut f64,
        var_udse_dc_dn5_slot: &mut f64,
        var_udse_dc_dn6_slot: &mut f64,
        var_udse_dc_dn7_slot: &mut f64,
        var_udse_dc_dn8_slot: &mut f64,
        var_udse_dc_rv_slot: &mut f64,
        var_vm_slot: &mut f64,
        var_vm_dn5_slot: &mut f64,
        var_vm_dn6_slot: &mut f64,
        var_vm_dn7_slot: &mut f64,
        var_vm_dn8_slot: &mut f64,
        var_vm_rv_slot: &mut f64,
        var_zg_slot: &mut f64,
        var_zg_dn5_slot: &mut f64,
        var_zg_dn6_slot: &mut f64,
        var_zg_dn7_slot: &mut f64,
        var_zg_dn8_slot: &mut f64,
        var_zg_rv_slot: &mut f64,
    ) {
        let mut var_arg1: f64 = *var_arg1_slot;
        let mut var_arg1_dn5: f64 = *var_arg1_dn5_slot;
        let mut var_arg1_dn6: f64 = *var_arg1_dn6_slot;
        let mut var_arg1_dn7: f64 = *var_arg1_dn7_slot;
        let mut var_arg1_dn8: f64 = *var_arg1_dn8_slot;
        let mut var_arg1_rv: f64 = *var_arg1_rv_slot;
        let mut var_arg2mina: f64 = *var_arg2mina_slot;
        let mut var_arg2mina_dn5: f64 = *var_arg2mina_dn5_slot;
        let mut var_arg2mina_dn6: f64 = *var_arg2mina_dn6_slot;
        let mut var_arg2mina_dn7: f64 = *var_arg2mina_dn7_slot;
        let mut var_arg2mina_dn8: f64 = *var_arg2mina_dn8_slot;
        let mut var_arg2mina_rv: f64 = *var_arg2mina_rv_slot;
        let mut var_dch: f64 = *var_dch_slot;
        let mut var_dch_dn5: f64 = *var_dch_dn5_slot;
        let mut var_dch_dn6: f64 = *var_dch_dn6_slot;
        let mut var_dch_dn7: f64 = *var_dch_dn7_slot;
        let mut var_dch_dn8: f64 = *var_dch_dn8_slot;
        let mut var_dch_rv: f64 = *var_dch_rv_slot;
        let mut var_fs1: f64 = *var_fs1_slot;
        let mut var_fs1_dn5: f64 = *var_fs1_dn5_slot;
        let mut var_fs1_dn6: f64 = *var_fs1_dn6_slot;
        let mut var_fs1_dn7: f64 = *var_fs1_dn7_slot;
        let mut var_fs1_rv: f64 = *var_fs1_rv_slot;
        let mut var_fs2: f64 = *var_fs2_slot;
        let mut var_fs2_rv: f64 = *var_fs2_rv_slot;
        let mut var_fs3: f64 = *var_fs3_slot;
        let mut var_fs3_dn5: f64 = *var_fs3_dn5_slot;
        let mut var_fs3_dn6: f64 = *var_fs3_dn6_slot;
        let mut var_fs3_dn7: f64 = *var_fs3_dn7_slot;
        let mut var_fs3_rv: f64 = *var_fs3_rv_slot;
        let mut var_guard1226: f64 = *var_guard1226_slot;
        let mut var_guard1226_rv: f64 = *var_guard1226_rv_slot;
        let mut var_guard1227: f64 = *var_guard1227_slot;
        let mut var_guard1227_rv: f64 = *var_guard1227_rv_slot;
        let mut var_guard1230: f64 = *var_guard1230_slot;
        let mut var_guard1230_rv: f64 = *var_guard1230_rv_slot;
        let mut var_guard1231: f64 = *var_guard1231_slot;
        let mut var_guard1231_rv: f64 = *var_guard1231_rv_slot;
        let mut var_guard1232: f64 = *var_guard1232_slot;
        let mut var_guard1232_rv: f64 = *var_guard1232_rv_slot;
        let mut var_guard1233: f64 = *var_guard1233_slot;
        let mut var_guard1233_rv: f64 = *var_guard1233_rv_slot;
        let mut var_psi_t: f64 = *var_psi_t_slot;
        let mut var_psi_t_dn5: f64 = *var_psi_t_dn5_slot;
        let mut var_psi_t_dn6: f64 = *var_psi_t_dn6_slot;
        let mut var_psi_t_dn7: f64 = *var_psi_t_dn7_slot;
        let mut var_psi_t_dn8: f64 = *var_psi_t_dn8_slot;
        let mut var_psi_t_rv: f64 = *var_psi_t_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_tme1: f64 = *var_tme1_slot;
        let mut var_tme1_rv: f64 = *var_tme1_rv_slot;
        let mut var_tme2: f64 = *var_tme2_slot;
        let mut var_tme2_dn5: f64 = *var_tme2_dn5_slot;
        let mut var_tme2_dn6: f64 = *var_tme2_dn6_slot;
        let mut var_tme2_dn7: f64 = *var_tme2_dn7_slot;
        let mut var_tme2_dn8: f64 = *var_tme2_dn8_slot;
        let mut var_tme2_rv: f64 = *var_tme2_rv_slot;
        let mut var_udse_dc: f64 = *var_udse_dc_slot;
        let mut var_udse_dc_dn5: f64 = *var_udse_dc_dn5_slot;
        let mut var_udse_dc_dn6: f64 = *var_udse_dc_dn6_slot;
        let mut var_udse_dc_dn7: f64 = *var_udse_dc_dn7_slot;
        let mut var_udse_dc_dn8: f64 = *var_udse_dc_dn8_slot;
        let mut var_udse_dc_rv: f64 = *var_udse_dc_rv_slot;
        let mut var_vm: f64 = *var_vm_slot;
        let mut var_vm_dn5: f64 = *var_vm_dn5_slot;
        let mut var_vm_dn6: f64 = *var_vm_dn6_slot;
        let mut var_vm_dn7: f64 = *var_vm_dn7_slot;
        let mut var_vm_dn8: f64 = *var_vm_dn8_slot;
        let mut var_vm_rv: f64 = *var_vm_rv_slot;
        let mut var_zg: f64 = *var_zg_slot;
        let mut var_zg_dn5: f64 = *var_zg_dn5_slot;
        let mut var_zg_dn6: f64 = *var_zg_dn6_slot;
        let mut var_zg_dn7: f64 = *var_zg_dn7_slot;
        let mut var_zg_dn8: f64 = *var_zg_dn8_slot;
        let mut var_zg_rv: f64 = *var_zg_rv_slot;

        let (assign46180_e59195, assign46180_e59195_d_n5, assign46180_e59195_d_n6, assign46180_e59195_d_n7, assign46180_e59195_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46180_e59180: f64 = (2.0 / var_tme1);
        let assign46180_e59184: f64 = (var_tme2 * var_tme2);
        let assign46180_e59187: f64 = (var_tme1 * var_fs1);
        let assign46180_e59189: f64 = (assign46180_e59187 * var_fs3);
        let assign46180_e59190: f64 = (assign46180_e59184 - assign46180_e59189);
        let assign46180_e59191: f64 = (assign46180_e59190).sqrt();
        let assign46180_e59192: f64 = (var_tme2 - assign46180_e59191);
        let assign46180_e59193: f64 = (assign46180_e59180 * assign46180_e59192);
        (assign46180_e59193, (assign46180_e59180 * (var_tme2_dn5 - ((((var_tme2_dn5 * var_tme2) + (var_tme2 * var_tme2_dn5)) - (((var_tme1 * var_fs1_dn5) * var_fs3) + (assign46180_e59187 * var_fs3_dn5))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (var_tme2_dn6 - ((((var_tme2_dn6 * var_tme2) + (var_tme2 * var_tme2_dn6)) - (((var_tme1 * var_fs1_dn6) * var_fs3) + (assign46180_e59187 * var_fs3_dn6))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (var_tme2_dn7 - ((((var_tme2_dn7 * var_tme2) + (var_tme2 * var_tme2_dn7)) - (((var_tme1 * var_fs1_dn7) * var_fs3) + (assign46180_e59187 * var_fs3_dn7))) / (2.0 * assign46180_e59191)))), (assign46180_e59180 * (var_tme2_dn8 - (((var_tme2_dn8 * var_tme2) + (var_tme2 * var_tme2_dn8)) / (2.0 * assign46180_e59191)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46180_e59195;
        var_temp__blk936_dn5 = assign46180_e59195_d_n5;
        var_temp__blk936_dn6 = assign46180_e59195_d_n6;
        var_temp__blk936_dn7 = assign46180_e59195_d_n7;
        var_temp__blk936_dn8 = assign46180_e59195_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46190_e59203,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46190_e59201: f64 = (4.0 - 0.3);
        (assign46190_e59201,)
    } else {
        (var_tme1,)
    }
};
        var_tme1 = assign46190_e59203;
        var_tme1_rv = 0.0;

        let (assign46200_e59211, assign46200_e59211_d_n5, assign46200_e59211_d_n6, assign46200_e59211_d_n7, assign46200_e59211_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1222 != 0.0)) {
        let assign46200_e59209: f64 = (var_fs2 + var_temp__blk936);
        (assign46200_e59209, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    } else {
        (var_tme2, var_tme2_dn5, var_tme2_dn6, var_tme2_dn7, var_tme2_dn8,)
    }
};
        var_tme2 = assign46200_e59211;
        var_tme2_dn5 = assign46200_e59211_d_n5;
        var_tme2_dn6 = assign46200_e59211_d_n6;
        var_tme2_dn7 = assign46200_e59211_d_n7;
        var_tme2_dn8 = assign46200_e59211_d_n8;
        var_tme2_rv = 0.0;

        let assign46230_e59245: f64 = if var_igovd_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1226 = assign46230_e59245;
        var_guard1226_rv = 0.0;

        let (assign46240_e59258, assign46240_e59258_d_n5, assign46240_e59258_d_n6, assign46240_e59258_d_n7, assign46240_e59258_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46240_e59251: f64 = (var_vovd * var_vovd);
        let assign46240_e59253: f64 = (assign46240_e59251 + 1e-6);
        let assign46240_e59254: f64 = (assign46240_e59253).sqrt();
        let assign46240_e59256: f64 = (assign46240_e59254 * var_inv_chib);
        (assign46240_e59256, ((((var_vovd_dn5 * var_vovd) + (var_vovd * var_vovd_dn5)) / (2.0 * assign46240_e59254)) * var_inv_chib), ((((var_vovd_dn6 * var_vovd) + (var_vovd * var_vovd_dn6)) / (2.0 * assign46240_e59254)) * var_inv_chib), ((((var_vovd_dn7 * var_vovd) + (var_vovd * var_vovd_dn7)) / (2.0 * assign46240_e59254)) * var_inv_chib), 0.0,)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46240_e59258;
        var_zg_dn5 = assign46240_e59258_d_n5;
        var_zg_dn6 = assign46240_e59258_d_n6;
        var_zg_dn7 = assign46240_e59258_d_n7;
        var_zg_dn8 = assign46240_e59258_d_n8;
        var_zg_rv = 0.0;

        let assign46250_e59261: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1227 = assign46250_e59261;
        var_guard1227_rv = 0.0;

        let (assign46260_e59284, assign46260_e59284_d_n5, assign46260_e59284_d_n6, assign46260_e59284_d_n7, assign46260_e59284_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) && (var_guard1227 != 0.0)) {
        let assign46260_e59270: f64 = (var_zg + var_gcqovd);
        let assign46260_e59273: f64 = (var_zg - var_gcqovd);
        let assign46260_e59276: f64 = (var_zg - var_gcqovd);
        let assign46260_e59277: f64 = (assign46260_e59273 * assign46260_e59276);
        let assign46260_e59279: f64 = (assign46260_e59277 + 1e-6);
        let assign46260_e59280: f64 = (assign46260_e59279).sqrt();
        let assign46260_e59281: f64 = (assign46260_e59270 - assign46260_e59280);
        let assign46260_e59282: f64 = (0.5 * assign46260_e59281);
        (assign46260_e59282, (0.5 * (var_zg_dn5 - (((var_zg_dn5 * assign46260_e59276) + (assign46260_e59273 * var_zg_dn5)) / (2.0 * assign46260_e59280)))), (0.5 * (var_zg_dn6 - (((var_zg_dn6 * assign46260_e59276) + (assign46260_e59273 * var_zg_dn6)) / (2.0 * assign46260_e59280)))), (0.5 * (var_zg_dn7 - (((var_zg_dn7 * assign46260_e59276) + (assign46260_e59273 * var_zg_dn7)) / (2.0 * assign46260_e59280)))), (0.5 * (var_zg_dn8 - (((var_zg_dn8 * assign46260_e59276) + (assign46260_e59273 * var_zg_dn8)) / (2.0 * assign46260_e59280)))),)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46260_e59284;
        var_zg_dn5 = assign46260_e59284_d_n5;
        var_zg_dn6 = assign46260_e59284_d_n6;
        var_zg_dn7 = assign46260_e59284_d_n7;
        var_zg_dn8 = assign46260_e59284_d_n8;
        var_zg_rv = 0.0;

        let (assign46270_e59301, assign46270_e59301_d_n5, assign46270_e59301_d_n6, assign46270_e59301_d_n7, assign46270_e59301_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46270_e59290: f64 = (-1.5);
        let assign46270_e59295: f64 = (var_gc3ovd_i * var_zg);
        let assign46270_e59296: f64 = (var_gc2ovd_i + assign46270_e59295);
        let assign46270_e59297: f64 = (var_zg * assign46270_e59296);
        let assign46270_e59298: f64 = (assign46270_e59290 + assign46270_e59297);
        let assign46270_e59299: f64 = (var_bov_d * assign46270_e59298);
        (assign46270_e59299, (var_bov_d * ((var_zg_dn5 * assign46270_e59296) + (var_zg * (var_gc3ovd_i * var_zg_dn5)))), (var_bov_d * ((var_zg_dn6 * assign46270_e59296) + (var_zg * (var_gc3ovd_i * var_zg_dn6)))), (var_bov_d * ((var_zg_dn7 * assign46270_e59296) + (var_zg * (var_gc3ovd_i * var_zg_dn7)))), (var_bov_d * ((var_zg_dn8 * assign46270_e59296) + (var_zg * (var_gc3ovd_i * var_zg_dn8)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46270_e59301;
        var_temp__blk936_dn5 = assign46270_e59301_d_n5;
        var_temp__blk936_dn6 = assign46270_e59301_d_n6;
        var_temp__blk936_dn7 = assign46270_e59301_d_n7;
        var_temp__blk936_dn8 = assign46270_e59301_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46330_e59387, assign46330_e59387_d_n5, assign46330_e59387_d_n6, assign46330_e59387_d_n7,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46330_e59385: f64 = (3.0 + var_xd_ov);
        (assign46330_e59385, var_xd_ov_dn5, var_xd_ov_dn6, var_xd_ov_dn7,)
    } else {
        (var_fs1, var_fs1_dn5, var_fs1_dn6, var_fs1_dn7,)
    }
};
        var_fs1 = assign46330_e59387;
        var_fs1_dn5 = assign46330_e59387_d_n5;
        var_fs1_dn6 = assign46330_e59387_d_n6;
        var_fs1_dn7 = assign46330_e59387_d_n7;
        var_fs1_rv = 0.0;

        let (assign46340_e59396,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46340_e59392: f64 = (-3.0);
        let assign46340_e59394: f64 = (assign46340_e59392 - var_gco_i);
        (assign46340_e59394,)
    } else {
        (var_fs2,)
    }
};
        var_fs2 = assign46340_e59396;
        var_fs2_rv = 0.0;

        let (assign46350_e59404, assign46350_e59404_d_n5, assign46350_e59404_d_n6, assign46350_e59404_d_n7,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46350_e59402: f64 = (30.0 * var_vgdprime);
        (assign46350_e59402, (30.0 * var_vgdprime_dn5), (30.0 * var_vgdprime_dn6), (30.0 * var_vgdprime_dn7),)
    } else {
        (var_fs3, var_fs3_dn5, var_fs3_dn6, var_fs3_dn7,)
    }
};
        var_fs3 = assign46350_e59404;
        var_fs3_dn5 = assign46350_e59404_d_n5;
        var_fs3_dn6 = assign46350_e59404_d_n6;
        var_fs3_dn7 = assign46350_e59404_d_n7;
        var_fs3_rv = 0.0;

        let (assign46360_e59412,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46360_e59410: f64 = (4.0 - 0.9);
        (assign46360_e59410,)
    } else {
        (var_tme1,)
    }
};
        var_tme1 = assign46360_e59412;
        var_tme1_rv = 0.0;

        let (assign46370_e59420, assign46370_e59420_d_n5, assign46370_e59420_d_n6, assign46370_e59420_d_n7, assign46370_e59420_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46370_e59418: f64 = (var_fs1 + var_fs3);
        (assign46370_e59418, (var_fs1_dn5 + var_fs3_dn5), (var_fs1_dn6 + var_fs3_dn6), (var_fs1_dn7 + var_fs3_dn7), 0.0,)
    } else {
        (var_tme2, var_tme2_dn5, var_tme2_dn6, var_tme2_dn7, var_tme2_dn8,)
    }
};
        var_tme2 = assign46370_e59420;
        var_tme2_dn5 = assign46370_e59420_d_n5;
        var_tme2_dn6 = assign46370_e59420_d_n6;
        var_tme2_dn7 = assign46370_e59420_d_n7;
        var_tme2_dn8 = assign46370_e59420_d_n8;
        var_tme2_rv = 0.0;

        let (assign46380_e59441, assign46380_e59441_d_n5, assign46380_e59441_d_n6, assign46380_e59441_d_n7, assign46380_e59441_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46380_e59426: f64 = (2.0 / var_tme1);
        let assign46380_e59430: f64 = (var_tme2 * var_tme2);
        let assign46380_e59433: f64 = (var_tme1 * var_fs1);
        let assign46380_e59435: f64 = (assign46380_e59433 * var_fs3);
        let assign46380_e59436: f64 = (assign46380_e59430 - assign46380_e59435);
        let assign46380_e59437: f64 = (assign46380_e59436).sqrt();
        let assign46380_e59438: f64 = (var_tme2 - assign46380_e59437);
        let assign46380_e59439: f64 = (assign46380_e59426 * assign46380_e59438);
        (assign46380_e59439, (assign46380_e59426 * (var_tme2_dn5 - ((((var_tme2_dn5 * var_tme2) + (var_tme2 * var_tme2_dn5)) - (((var_tme1 * var_fs1_dn5) * var_fs3) + (assign46380_e59433 * var_fs3_dn5))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (var_tme2_dn6 - ((((var_tme2_dn6 * var_tme2) + (var_tme2 * var_tme2_dn6)) - (((var_tme1 * var_fs1_dn6) * var_fs3) + (assign46380_e59433 * var_fs3_dn6))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (var_tme2_dn7 - ((((var_tme2_dn7 * var_tme2) + (var_tme2 * var_tme2_dn7)) - (((var_tme1 * var_fs1_dn7) * var_fs3) + (assign46380_e59433 * var_fs3_dn7))) / (2.0 * assign46380_e59437)))), (assign46380_e59426 * (var_tme2_dn8 - (((var_tme2_dn8 * var_tme2) + (var_tme2 * var_tme2_dn8)) / (2.0 * assign46380_e59437)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46380_e59441;
        var_temp__blk936_dn5 = assign46380_e59441_d_n5;
        var_temp__blk936_dn6 = assign46380_e59441_d_n6;
        var_temp__blk936_dn7 = assign46380_e59441_d_n7;
        var_temp__blk936_dn8 = assign46380_e59441_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46390_e59449,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46390_e59447: f64 = (4.0 - 0.3);
        (assign46390_e59447,)
    } else {
        (var_tme1,)
    }
};
        var_tme1 = assign46390_e59449;
        var_tme1_rv = 0.0;

        let (assign46400_e59457, assign46400_e59457_d_n5, assign46400_e59457_d_n6, assign46400_e59457_d_n7, assign46400_e59457_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1226 != 0.0)) {
        let assign46400_e59455: f64 = (var_fs2 + var_temp__blk936);
        (assign46400_e59455, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    } else {
        (var_tme2, var_tme2_dn5, var_tme2_dn6, var_tme2_dn7, var_tme2_dn8,)
    }
};
        var_tme2 = assign46400_e59457;
        var_tme2_dn5 = assign46400_e59457_d_n5;
        var_tme2_dn6 = assign46400_e59457_d_n6;
        var_tme2_dn7 = assign46400_e59457_d_n7;
        var_tme2_dn8 = assign46400_e59457_d_n8;
        var_tme2_rv = 0.0;

        let assign46430_e59491: f64 = if var_iginv_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1230 = assign46430_e59491;
        var_guard1230_rv = 0.0;

        let assign46440_e59494: f64 = if var_xg_dc <= 0.0 { 1.0 } else { 0.0 };
        var_guard1231 = assign46440_e59494;
        var_guard1231_rv = 0.0;

        let (assign46450_e59504, assign46450_e59504_d_n5, assign46450_e59504_d_n6, assign46450_e59504_d_n7, assign46450_e59504_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign46450_e59502: f64 = (1.0 + var_ar);
        (assign46450_e59502, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46450_e59504;
        var_temp__blk936_dn5 = assign46450_e59504_d_n5;
        var_temp__blk936_dn6 = assign46450_e59504_d_n6;
        var_temp__blk936_dn7 = assign46450_e59504_d_n7;
        var_temp__blk936_dn8 = assign46450_e59504_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46460_e59517, assign46460_e59517_d_n5, assign46460_e59517_d_n6, assign46460_e59517_d_n7, assign46460_e59517_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign46460_e59511: f64 = (var_temp__blk936).sqrt();
        let assign46460_e59513: f64 = (assign46460_e59511 * var_v_ds);
        let assign46460_e59515: f64 = (assign46460_e59513 / var_vdsat_lim_dc);
        (assign46460_e59515, (((((var_temp__blk936_dn5 / (2.0 * assign46460_e59511)) * var_v_ds) * var_vdsat_lim_dc) - (assign46460_e59513 * var_vdsat_lim_dc_dn5)) / (var_vdsat_lim_dc * var_vdsat_lim_dc)), ((((((var_temp__blk936_dn6 / (2.0 * assign46460_e59511)) * var_v_ds) + (assign46460_e59511 * var_v_ds_dn6)) * var_vdsat_lim_dc) - (assign46460_e59513 * var_vdsat_lim_dc_dn6)) / (var_vdsat_lim_dc * var_vdsat_lim_dc)), ((((((var_temp__blk936_dn7 / (2.0 * assign46460_e59511)) * var_v_ds) + (assign46460_e59511 * var_v_ds_dn7)) * var_vdsat_lim_dc) - (assign46460_e59513 * var_vdsat_lim_dc_dn7)) / (var_vdsat_lim_dc * var_vdsat_lim_dc)), (((((var_temp__blk936_dn8 / (2.0 * assign46460_e59511)) * var_v_ds) * var_vdsat_lim_dc) - (assign46460_e59513 * var_vdsat_lim_dc_dn8)) / (var_vdsat_lim_dc * var_vdsat_lim_dc)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign46460_e59517;
        var_temp1_dn5 = assign46460_e59517_d_n5;
        var_temp1_dn6 = assign46460_e59517_d_n6;
        var_temp1_dn7 = assign46460_e59517_d_n7;
        var_temp1_dn8 = assign46460_e59517_d_n8;
        var_temp1_rv = 0.0;

        let (assign46470_e59529, assign46470_e59529_d_n5, assign46470_e59529_d_n6, assign46470_e59529_d_n7, assign46470_e59529_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign46470_e59525: f64 = (var_temp1 * var_temp1);
        let assign46470_e59527: f64 = (assign46470_e59525 + var_temp__blk936);
        (assign46470_e59527, (((var_temp1_dn5 * var_temp1) + (var_temp1 * var_temp1_dn5)) + var_temp__blk936_dn5), (((var_temp1_dn6 * var_temp1) + (var_temp1 * var_temp1_dn6)) + var_temp__blk936_dn6), (((var_temp1_dn7 * var_temp1) + (var_temp1 * var_temp1_dn7)) + var_temp__blk936_dn7), (((var_temp1_dn8 * var_temp1) + (var_temp1 * var_temp1_dn8)) + var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign46470_e59529;
        var_temp2_dn5 = assign46470_e59529_d_n5;
        var_temp2_dn6 = assign46470_e59529_d_n6;
        var_temp2_dn7 = assign46470_e59529_d_n7;
        var_temp2_dn8 = assign46470_e59529_d_n8;
        var_temp2_rv = 0.0;

        let (assign46480_e59539, assign46480_e59539_d_n5, assign46480_e59539_d_n6, assign46480_e59539_d_n7, assign46480_e59539_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign46480_e59537: f64 = (2.0 * var_temp1);
        (assign46480_e59537, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46480_e59539;
        var_temp__blk936_dn5 = assign46480_e59539_d_n5;
        var_temp__blk936_dn6 = assign46480_e59539_d_n6;
        var_temp__blk936_dn7 = assign46480_e59539_d_n7;
        var_temp__blk936_dn8 = assign46480_e59539_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46490_e59561, assign46490_e59561_d_n5, assign46490_e59561_d_n6, assign46490_e59561_d_n7, assign46490_e59561_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1231 != 0.0)) {
        let assign46490_e59547: f64 = (var_vdsat_lim_dc * var_inv_phit1_dc);
        let assign46490_e59549: f64 = (assign46490_e59547 * var_temp__blk936);
        let assign46490_e59552: f64 = (var_temp2 - var_temp__blk936);
        let assign46490_e59553: f64 = (assign46490_e59552).sqrt();
        let assign46490_e59556: f64 = (var_temp2 + var_temp__blk936);
        let assign46490_e59557: f64 = (assign46490_e59556).sqrt();
        let assign46490_e59558: f64 = (assign46490_e59553 + assign46490_e59557);
        let assign46490_e59559: f64 = (assign46490_e59549 / assign46490_e59558);
        (assign46490_e59559, (((((((var_vdsat_lim_dc_dn5 * var_inv_phit1_dc) + (var_vdsat_lim_dc * var_inv_phit1_dc_dn5)) * var_temp__blk936) + (assign46490_e59547 * var_temp__blk936_dn5)) * assign46490_e59558) - (assign46490_e59549 * (((var_temp2_dn5 - var_temp__blk936_dn5) / (2.0 * assign46490_e59553)) + ((var_temp2_dn5 + var_temp__blk936_dn5) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((var_vdsat_lim_dc_dn6 * var_inv_phit1_dc) + (var_vdsat_lim_dc * var_inv_phit1_dc_dn6)) * var_temp__blk936) + (assign46490_e59547 * var_temp__blk936_dn6)) * assign46490_e59558) - (assign46490_e59549 * (((var_temp2_dn6 - var_temp__blk936_dn6) / (2.0 * assign46490_e59553)) + ((var_temp2_dn6 + var_temp__blk936_dn6) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((var_vdsat_lim_dc_dn7 * var_inv_phit1_dc) + (var_vdsat_lim_dc * var_inv_phit1_dc_dn7)) * var_temp__blk936) + (assign46490_e59547 * var_temp__blk936_dn7)) * assign46490_e59558) - (assign46490_e59549 * (((var_temp2_dn7 - var_temp__blk936_dn7) / (2.0 * assign46490_e59553)) + ((var_temp2_dn7 + var_temp__blk936_dn7) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)), (((((((var_vdsat_lim_dc_dn8 * var_inv_phit1_dc) + (var_vdsat_lim_dc * var_inv_phit1_dc_dn8)) * var_temp__blk936) + (assign46490_e59547 * var_temp__blk936_dn8)) * assign46490_e59558) - (assign46490_e59549 * (((var_temp2_dn8 - var_temp__blk936_dn8) / (2.0 * assign46490_e59553)) + ((var_temp2_dn8 + var_temp__blk936_dn8) / (2.0 * assign46490_e59557))))) / (assign46490_e59558 * assign46490_e59558)),)
    } else {
        (var_udse_dc, var_udse_dc_dn5, var_udse_dc_dn6, var_udse_dc_dn7, var_udse_dc_dn8,)
    }
};
        var_udse_dc = assign46490_e59561;
        var_udse_dc_dn5 = assign46490_e59561_d_n5;
        var_udse_dc_dn6 = assign46490_e59561_d_n6;
        var_udse_dc_dn7 = assign46490_e59561_d_n7;
        var_udse_dc_dn8 = assign46490_e59561_d_n8;
        var_udse_dc_rv = 0.0;

        let assign46500_e59564: f64 = (var_x_ds_dc - var_udse_dc);
        let assign46500_e59566: f64 = (-230.25850929940458);
        let assign46500_e59567: f64 = if assign46500_e59564 > assign46500_e59566 { 1.0 } else { 0.0 };
        var_guard1232 = assign46500_e59567;
        var_guard1232_rv = 0.0;

        let (assign46510_e59578, assign46510_e59578_d_n5, assign46510_e59578_d_n6, assign46510_e59578_d_n7, assign46510_e59578_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1232 != 0.0)) {
        let assign46510_e59575: f64 = (var_x_ds_dc - var_udse_dc);
        let assign46510_e59576: f64 = (assign46510_e59575).exp();
        (assign46510_e59576, (assign46510_e59576 * (var_x_ds_dc_dn5 - var_udse_dc_dn5)), (assign46510_e59576 * (var_x_ds_dc_dn6 - var_udse_dc_dn6)), (assign46510_e59576 * (var_x_ds_dc_dn7 - var_udse_dc_dn7)), (assign46510_e59576 * (var_x_ds_dc_dn8 - var_udse_dc_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46510_e59578;
        var_temp__blk936_dn5 = assign46510_e59578_d_n5;
        var_temp__blk936_dn6 = assign46510_e59578_d_n6;
        var_temp__blk936_dn7 = assign46510_e59578_d_n7;
        var_temp__blk936_dn8 = assign46510_e59578_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46520_e59618, assign46520_e59618_d_n5, assign46520_e59618_d_n6, assign46520_e59618_d_n7, assign46520_e59618_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1232 == 0.0)) {
        let assign46520_e59588: f64 = (-230.25850929940458);
        let assign46520_e59591: f64 = (var_x_ds_dc - var_udse_dc);
        let assign46520_e59592: f64 = (assign46520_e59588 - assign46520_e59591);
        let assign46520_e59596: f64 = (-230.25850929940458);
        let assign46520_e59599: f64 = (var_x_ds_dc - var_udse_dc);
        let assign46520_e59600: f64 = (assign46520_e59596 - assign46520_e59599);
        let assign46520_e59603: f64 = (-230.25850929940458);
        let assign46520_e59606: f64 = (var_x_ds_dc - var_udse_dc);
        let assign46520_e59607: f64 = (assign46520_e59603 - assign46520_e59606);
        let assign46520_e59609: f64 = (assign46520_e59607 * 0.3333333333333333);
        let assign46520_e59610: f64 = (1.0 + assign46520_e59609);
        let assign46520_e59611: f64 = (assign46520_e59600 * assign46520_e59610);
        let assign46520_e59612: f64 = (0.5 * assign46520_e59611);
        let assign46520_e59613: f64 = (1.0 + assign46520_e59612);
        let assign46520_e59614: f64 = (assign46520_e59592 * assign46520_e59613);
        let assign46520_e59615: f64 = (1.0 + assign46520_e59614);
        let assign46520_e59616: f64 = (1e-100 / assign46520_e59615);
        (assign46520_e59616, (-((1e-100 * (((-(var_x_ds_dc_dn5 - var_udse_dc_dn5)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(var_x_ds_dc_dn5 - var_udse_dc_dn5)) * assign46520_e59610) + (assign46520_e59600 * ((-(var_x_ds_dc_dn5 - var_udse_dc_dn5)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(var_x_ds_dc_dn6 - var_udse_dc_dn6)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(var_x_ds_dc_dn6 - var_udse_dc_dn6)) * assign46520_e59610) + (assign46520_e59600 * ((-(var_x_ds_dc_dn6 - var_udse_dc_dn6)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(var_x_ds_dc_dn7 - var_udse_dc_dn7)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(var_x_ds_dc_dn7 - var_udse_dc_dn7)) * assign46520_e59610) + (assign46520_e59600 * ((-(var_x_ds_dc_dn7 - var_udse_dc_dn7)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))), (-((1e-100 * (((-(var_x_ds_dc_dn8 - var_udse_dc_dn8)) * assign46520_e59613) + (assign46520_e59592 * (0.5 * (((-(var_x_ds_dc_dn8 - var_udse_dc_dn8)) * assign46520_e59610) + (assign46520_e59600 * ((-(var_x_ds_dc_dn8 - var_udse_dc_dn8)) * 0.3333333333333333))))))) / (assign46520_e59615 * assign46520_e59615))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46520_e59618;
        var_temp__blk936_dn5 = assign46520_e59618_d_n5;
        var_temp__blk936_dn6 = assign46520_e59618_d_n6;
        var_temp__blk936_dn7 = assign46520_e59618_d_n7;
        var_temp__blk936_dn8 = assign46520_e59618_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46530_e59637, assign46530_e59637_d_n5, assign46530_e59637_d_n6, assign46530_e59637_d_n7, assign46530_e59637_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46530_e59626: f64 = (0.5 * var_x_ds_dc);
        let assign46530_e59630: f64 = (1.0 + var_temp__blk936);
        let assign46530_e59631: f64 = (0.5 * assign46530_e59630);
        let assign46530_e59632: f64 = (assign46530_e59631).ln();
        let assign46530_e59633: f64 = (assign46530_e59626 - assign46530_e59632);
        let assign46530_e59634: f64 = (var_phit1_dc * assign46530_e59633);
        let assign46530_e59635: f64 = (var_vsbstar_dc + assign46530_e59634);
        (assign46530_e59635, (var_vsbstar_dc_dn5 + ((var_phit1_dc_dn5 * assign46530_e59633) + (var_phit1_dc * ((0.5 * var_x_ds_dc_dn5) - ((0.5 * var_temp__blk936_dn5) / assign46530_e59631))))), (var_vsbstar_dc_dn6 + ((var_phit1_dc_dn6 * assign46530_e59633) + (var_phit1_dc * ((0.5 * var_x_ds_dc_dn6) - ((0.5 * var_temp__blk936_dn6) / assign46530_e59631))))), (var_vsbstar_dc_dn7 + ((var_phit1_dc_dn7 * assign46530_e59633) + (var_phit1_dc * ((0.5 * var_x_ds_dc_dn7) - ((0.5 * var_temp__blk936_dn7) / assign46530_e59631))))), (var_vsbstar_dc_dn8 + ((var_phit1_dc_dn8 * assign46530_e59633) + (var_phit1_dc * ((0.5 * var_x_ds_dc_dn8) - ((0.5 * var_temp__blk936_dn8) / assign46530_e59631))))),)
    } else {
        (var_vm, var_vm_dn5, var_vm_dn6, var_vm_dn7, var_vm_dn8,)
    }
};
        var_vm = assign46530_e59637;
        var_vm_dn5 = assign46530_e59637_d_n5;
        var_vm_dn6 = assign46530_e59637_d_n6;
        var_vm_dn7 = assign46530_e59637_d_n7;
        var_vm_dn8 = assign46530_e59637_d_n8;
        var_vm_rv = 0.0;

        let (assign46540_e59645, assign46540_e59645_d_n5, assign46540_e59645_d_n6, assign46540_e59645_d_n7, assign46540_e59645_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46540_e59643: f64 = (var_gco_i * var_phit1_dc);
        (assign46540_e59643, (var_gco_i * var_phit1_dc_dn5), (var_gco_i * var_phit1_dc_dn6), (var_gco_i * var_phit1_dc_dn7), (var_gco_i * var_phit1_dc_dn8),)
    } else {
        (var_dch, var_dch_dn5, var_dch_dn6, var_dch_dn7, var_dch_dn8,)
    }
};
        var_dch = assign46540_e59645;
        var_dch_dn5 = assign46540_e59645_d_n5;
        var_dch_dn6 = assign46540_e59645_d_n6;
        var_dch_dn7 = assign46540_e59645_d_n7;
        var_dch_dn8 = assign46540_e59645_d_n8;
        var_dch_rv = 0.0;

        let (assign46550_e59653, assign46550_e59653_d_n5, assign46550_e59653_d_n6, assign46550_e59653_d_n7, assign46550_e59653_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46550_e59651: f64 = (var_voxm_dc + var_dch);
        (assign46550_e59651, (var_voxm_dc_dn5 + var_dch_dn5), (var_voxm_dc_dn6 + var_dch_dn6), (var_voxm_dc_dn7 + var_dch_dn7), (var_voxm_dc_dn8 + var_dch_dn8),)
    } else {
        (var_arg2mina, var_arg2mina_dn5, var_arg2mina_dn6, var_arg2mina_dn7, var_arg2mina_dn8,)
    }
};
        var_arg2mina = assign46550_e59653;
        var_arg2mina_dn5 = assign46550_e59653_d_n5;
        var_arg2mina_dn6 = assign46550_e59653_d_n6;
        var_arg2mina_dn7 = assign46550_e59653_d_n7;
        var_arg2mina_dn8 = assign46550_e59653_d_n8;
        var_arg2mina_rv = 0.0;

        let (assign46560_e59674, assign46560_e59674_d_n5, assign46560_e59674_d_n6, assign46560_e59674_d_n7, assign46560_e59674_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46560_e59660: f64 = var_arg2mina;
        let assign46560_e59663: f64 = (-var_arg2mina);
        let assign46560_e59666: f64 = (-var_arg2mina);
        let assign46560_e59667: f64 = (assign46560_e59663 * assign46560_e59666);
        let assign46560_e59669: f64 = (assign46560_e59667 + 0.01);
        let assign46560_e59670: f64 = (assign46560_e59669).sqrt();
        let assign46560_e59671: f64 = (assign46560_e59660 - assign46560_e59670);
        let assign46560_e59672: f64 = (0.5 * assign46560_e59671);
        (assign46560_e59672, (0.5 * (var_arg2mina_dn5 - ((((-var_arg2mina_dn5) * assign46560_e59666) + (assign46560_e59663 * (-var_arg2mina_dn5))) / (2.0 * assign46560_e59670)))), (0.5 * (var_arg2mina_dn6 - ((((-var_arg2mina_dn6) * assign46560_e59666) + (assign46560_e59663 * (-var_arg2mina_dn6))) / (2.0 * assign46560_e59670)))), (0.5 * (var_arg2mina_dn7 - ((((-var_arg2mina_dn7) * assign46560_e59666) + (assign46560_e59663 * (-var_arg2mina_dn7))) / (2.0 * assign46560_e59670)))), (0.5 * (var_arg2mina_dn8 - ((((-var_arg2mina_dn8) * assign46560_e59666) + (assign46560_e59663 * (-var_arg2mina_dn8))) / (2.0 * assign46560_e59670)))),)
    } else {
        (var_psi_t, var_psi_t_dn5, var_psi_t_dn6, var_psi_t_dn7, var_psi_t_dn8,)
    }
};
        var_psi_t = assign46560_e59674;
        var_psi_t_dn5 = assign46560_e59674_d_n5;
        var_psi_t_dn6 = assign46560_e59674_d_n6;
        var_psi_t_dn7 = assign46560_e59674_d_n7;
        var_psi_t_dn8 = assign46560_e59674_d_n8;
        var_psi_t_rv = 0.0;

        let (assign46570_e59687, assign46570_e59687_d_n5, assign46570_e59687_d_n6, assign46570_e59687_d_n7, assign46570_e59687_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46570_e59680: f64 = (var_voxm_dc * var_voxm_dc);
        let assign46570_e59682: f64 = (assign46570_e59680 + 1e-6);
        let assign46570_e59683: f64 = (assign46570_e59682).sqrt();
        let assign46570_e59685: f64 = (assign46570_e59683 * var_inv_chib);
        (assign46570_e59685, ((((var_voxm_dc_dn5 * var_voxm_dc) + (var_voxm_dc * var_voxm_dc_dn5)) / (2.0 * assign46570_e59683)) * var_inv_chib), ((((var_voxm_dc_dn6 * var_voxm_dc) + (var_voxm_dc * var_voxm_dc_dn6)) / (2.0 * assign46570_e59683)) * var_inv_chib), ((((var_voxm_dc_dn7 * var_voxm_dc) + (var_voxm_dc * var_voxm_dc_dn7)) / (2.0 * assign46570_e59683)) * var_inv_chib), ((((var_voxm_dc_dn8 * var_voxm_dc) + (var_voxm_dc * var_voxm_dc_dn8)) / (2.0 * assign46570_e59683)) * var_inv_chib),)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46570_e59687;
        var_zg_dn5 = assign46570_e59687_d_n5;
        var_zg_dn6 = assign46570_e59687_d_n6;
        var_zg_dn7 = assign46570_e59687_d_n7;
        var_zg_dn8 = assign46570_e59687_d_n8;
        var_zg_rv = 0.0;

        let assign46580_e59690: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard1233 = assign46580_e59690;
        var_guard1233_rv = 0.0;

        let (assign46590_e59713, assign46590_e59713_d_n5, assign46590_e59713_d_n6, assign46590_e59713_d_n7, assign46590_e59713_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1233 != 0.0)) {
        let assign46590_e59699: f64 = (var_zg + var_gcq);
        let assign46590_e59702: f64 = (var_zg - var_gcq);
        let assign46590_e59705: f64 = (var_zg - var_gcq);
        let assign46590_e59706: f64 = (assign46590_e59702 * assign46590_e59705);
        let assign46590_e59708: f64 = (assign46590_e59706 + 1e-6);
        let assign46590_e59709: f64 = (assign46590_e59708).sqrt();
        let assign46590_e59710: f64 = (assign46590_e59699 - assign46590_e59709);
        let assign46590_e59711: f64 = (0.5 * assign46590_e59710);
        (assign46590_e59711, (0.5 * (var_zg_dn5 - (((var_zg_dn5 * assign46590_e59705) + (assign46590_e59702 * var_zg_dn5)) / (2.0 * assign46590_e59709)))), (0.5 * (var_zg_dn6 - (((var_zg_dn6 * assign46590_e59705) + (assign46590_e59702 * var_zg_dn6)) / (2.0 * assign46590_e59709)))), (0.5 * (var_zg_dn7 - (((var_zg_dn7 * assign46590_e59705) + (assign46590_e59702 * var_zg_dn7)) / (2.0 * assign46590_e59709)))), (0.5 * (var_zg_dn8 - (((var_zg_dn8 * assign46590_e59705) + (assign46590_e59702 * var_zg_dn8)) / (2.0 * assign46590_e59709)))),)
    } else {
        (var_zg, var_zg_dn5, var_zg_dn6, var_zg_dn7, var_zg_dn8,)
    }
};
        var_zg = assign46590_e59713;
        var_zg_dn5 = assign46590_e59713_d_n5;
        var_zg_dn6 = assign46590_e59713_d_n6;
        var_zg_dn7 = assign46590_e59713_d_n7;
        var_zg_dn8 = assign46590_e59713_d_n8;
        var_zg_rv = 0.0;

        let (assign46600_e59727, assign46600_e59727_d_n5, assign46600_e59727_d_n6, assign46600_e59727_d_n7, assign46600_e59727_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46600_e59720: f64 = (var_psi_t - var_alpha_b);
        let assign46600_e59722: f64 = (assign46600_e59720 - var_vm);
        let assign46600_e59724: f64 = (assign46600_e59722 * var_inv_phit1_dc);
        let assign46600_e59725: f64 = (var_x_m_dc + assign46600_e59724);
        (assign46600_e59725, (var_x_m_dc_dn5 + (((var_psi_t_dn5 - var_vm_dn5) * var_inv_phit1_dc) + (assign46600_e59722 * var_inv_phit1_dc_dn5))), (var_x_m_dc_dn6 + (((var_psi_t_dn6 - var_vm_dn6) * var_inv_phit1_dc) + (assign46600_e59722 * var_inv_phit1_dc_dn6))), (var_x_m_dc_dn7 + (((var_psi_t_dn7 - var_vm_dn7) * var_inv_phit1_dc) + (assign46600_e59722 * var_inv_phit1_dc_dn7))), (var_x_m_dc_dn8 + (((var_psi_t_dn8 - var_vm_dn8) * var_inv_phit1_dc) + (assign46600_e59722 * var_inv_phit1_dc_dn8))),)
    } else {
        (var_arg1, var_arg1_dn5, var_arg1_dn6, var_arg1_dn7, var_arg1_dn8,)
    }
};
        var_arg1 = assign46600_e59727;
        var_arg1_dn5 = assign46600_e59727_d_n5;
        var_arg1_dn6 = assign46600_e59727_d_n6;
        var_arg1_dn7 = assign46600_e59727_d_n7;
        var_arg1_dn8 = assign46600_e59727_d_n8;
        var_arg1_rv = 0.0;

        *var_arg1_slot = var_arg1;
        *var_arg1_dn5_slot = var_arg1_dn5;
        *var_arg1_dn6_slot = var_arg1_dn6;
        *var_arg1_dn7_slot = var_arg1_dn7;
        *var_arg1_dn8_slot = var_arg1_dn8;
        *var_arg1_rv_slot = var_arg1_rv;
        *var_arg2mina_slot = var_arg2mina;
        *var_arg2mina_dn5_slot = var_arg2mina_dn5;
        *var_arg2mina_dn6_slot = var_arg2mina_dn6;
        *var_arg2mina_dn7_slot = var_arg2mina_dn7;
        *var_arg2mina_dn8_slot = var_arg2mina_dn8;
        *var_arg2mina_rv_slot = var_arg2mina_rv;
        *var_dch_slot = var_dch;
        *var_dch_dn5_slot = var_dch_dn5;
        *var_dch_dn6_slot = var_dch_dn6;
        *var_dch_dn7_slot = var_dch_dn7;
        *var_dch_dn8_slot = var_dch_dn8;
        *var_dch_rv_slot = var_dch_rv;
        *var_fs1_slot = var_fs1;
        *var_fs1_dn5_slot = var_fs1_dn5;
        *var_fs1_dn6_slot = var_fs1_dn6;
        *var_fs1_dn7_slot = var_fs1_dn7;
        *var_fs1_rv_slot = var_fs1_rv;
        *var_fs2_slot = var_fs2;
        *var_fs2_rv_slot = var_fs2_rv;
        *var_fs3_slot = var_fs3;
        *var_fs3_dn5_slot = var_fs3_dn5;
        *var_fs3_dn6_slot = var_fs3_dn6;
        *var_fs3_dn7_slot = var_fs3_dn7;
        *var_fs3_rv_slot = var_fs3_rv;
        *var_guard1226_slot = var_guard1226;
        *var_guard1226_rv_slot = var_guard1226_rv;
        *var_guard1227_slot = var_guard1227;
        *var_guard1227_rv_slot = var_guard1227_rv;
        *var_guard1230_slot = var_guard1230;
        *var_guard1230_rv_slot = var_guard1230_rv;
        *var_guard1231_slot = var_guard1231;
        *var_guard1231_rv_slot = var_guard1231_rv;
        *var_guard1232_slot = var_guard1232;
        *var_guard1232_rv_slot = var_guard1232_rv;
        *var_guard1233_slot = var_guard1233;
        *var_guard1233_rv_slot = var_guard1233_rv;
        *var_psi_t_slot = var_psi_t;
        *var_psi_t_dn5_slot = var_psi_t_dn5;
        *var_psi_t_dn6_slot = var_psi_t_dn6;
        *var_psi_t_dn7_slot = var_psi_t_dn7;
        *var_psi_t_dn8_slot = var_psi_t_dn8;
        *var_psi_t_rv_slot = var_psi_t_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_tme1_slot = var_tme1;
        *var_tme1_rv_slot = var_tme1_rv;
        *var_tme2_slot = var_tme2;
        *var_tme2_dn5_slot = var_tme2_dn5;
        *var_tme2_dn6_slot = var_tme2_dn6;
        *var_tme2_dn7_slot = var_tme2_dn7;
        *var_tme2_dn8_slot = var_tme2_dn8;
        *var_tme2_rv_slot = var_tme2_rv;
        *var_udse_dc_slot = var_udse_dc;
        *var_udse_dc_dn5_slot = var_udse_dc_dn5;
        *var_udse_dc_dn6_slot = var_udse_dc_dn6;
        *var_udse_dc_dn7_slot = var_udse_dc_dn7;
        *var_udse_dc_dn8_slot = var_udse_dc_dn8;
        *var_udse_dc_rv_slot = var_udse_dc_rv;
        *var_vm_slot = var_vm;
        *var_vm_dn5_slot = var_vm_dn5;
        *var_vm_dn6_slot = var_vm_dn6;
        *var_vm_dn7_slot = var_vm_dn7;
        *var_vm_dn8_slot = var_vm_dn8;
        *var_vm_rv_slot = var_vm_rv;
        *var_zg_slot = var_zg;
        *var_zg_dn5_slot = var_zg_dn5;
        *var_zg_dn6_slot = var_zg_dn6;
        *var_zg_dn7_slot = var_zg_dn7;
        *var_zg_dn8_slot = var_zg_dn8;
        *var_zg_rv_slot = var_zg_rv;
    }

    pub(super) fn stamp_reactive_block_39(
        p: &Parameters,
        var_agidl_i: f64,
        var_agidld_i: f64,
        var_bch: f64,
        var_bgidlds: f64,
        var_bgidls: f64,
        var_cgidl_i: f64,
        var_cgidld_i: f64,
        var_chib_i: f64,
        var_dps_dc: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_gc2_i: f64,
        var_gc3_i: f64,
        var_guard1221: f64,
        var_guard1230: f64,
        var_inv_phit1_dc: f64,
        var_inv_phit1_dc_dn5: f64,
        var_inv_phit1_dc_dn6: f64,
        var_inv_phit1_dc_dn7: f64,
        var_inv_phit1_dc_dn8: f64,
        var_phit: f64,
        var_v_gs: f64,
        var_v_gs_dn5: f64,
        var_v_gs_dn6: f64,
        var_v_gs_dn7: f64,
        var_vdbprime: f64,
        var_vdbprime_dn6: f64,
        var_vdbprime_dn7: f64,
        var_vdbprime_dn8: f64,
        var_vm: f64,
        var_vm_dn5: f64,
        var_vm_dn6: f64,
        var_vm_dn7: f64,
        var_vm_dn8: f64,
        var_vovd: f64,
        var_vovd_dn5: f64,
        var_vovd_dn6: f64,
        var_vovd_dn7: f64,
        var_vovs: f64,
        var_vovs_dn5: f64,
        var_vovs_dn6: f64,
        var_vovs_dn7: f64,
        var_vsbprime: f64,
        var_vsbprime_dn6: f64,
        var_vsbprime_dn7: f64,
        var_vsbprime_dn8: f64,
        var_vsbstar_dc: f64,
        var_vsbstar_dc_dn5: f64,
        var_vsbstar_dc_dn6: f64,
        var_vsbstar_dc_dn7: f64,
        var_vsbstar_dc_dn8: f64,
        var_xg_dc: f64,
        var_zg: f64,
        var_zg_dn5: f64,
        var_zg_dn6: f64,
        var_zg_dn7: f64,
        var_zg_dn8: f64,
        var_arg1_slot: &mut f64,
        var_arg1_dn5_slot: &mut f64,
        var_arg1_dn6_slot: &mut f64,
        var_arg1_dn7_slot: &mut f64,
        var_arg1_dn8_slot: &mut f64,
        var_arg1_rv_slot: &mut f64,
        var_dsqredge_slot: &mut f64,
        var_dsqredge_dn5_slot: &mut f64,
        var_dsqredge_dn6_slot: &mut f64,
        var_dsqredge_dn7_slot: &mut f64,
        var_dsqredge_dn8_slot: &mut f64,
        var_dsqredge_rv_slot: &mut f64,
        var_ex_slot: &mut f64,
        var_ex_dn5_slot: &mut f64,
        var_ex_dn6_slot: &mut f64,
        var_ex_dn7_slot: &mut f64,
        var_ex_dn8_slot: &mut f64,
        var_ex_rv_slot: &mut f64,
        var_guard1236_slot: &mut f64,
        var_guard1236_rv_slot: &mut f64,
        var_guard1237_slot: &mut f64,
        var_guard1237_rv_slot: &mut f64,
        var_guard1240_slot: &mut f64,
        var_guard1240_rv_slot: &mut f64,
        var_guard1241_slot: &mut f64,
        var_guard1241_rv_slot: &mut f64,
        var_guard1242_slot: &mut f64,
        var_guard1242_rv_slot: &mut f64,
        var_guard1243_slot: &mut f64,
        var_guard1243_rv_slot: &mut f64,
        var_guard1244_slot: &mut f64,
        var_guard1244_rv_slot: &mut f64,
        var_guard1245_slot: &mut f64,
        var_guard1245_rv_slot: &mut f64,
        var_guard1246_slot: &mut f64,
        var_guard1246_rv_slot: &mut f64,
        var_guard1247_slot: &mut f64,
        var_guard1247_rv_slot: &mut f64,
        var_guard1248_slot: &mut f64,
        var_guard1248_rv_slot: &mut f64,
        var_inv_ex_slot: &mut f64,
        var_inv_ex_dn5_slot: &mut f64,
        var_inv_ex_dn6_slot: &mut f64,
        var_inv_ex_dn7_slot: &mut f64,
        var_inv_ex_dn8_slot: &mut f64,
        var_inv_ex_rv_slot: &mut f64,
        var_phit1edge_slot: &mut f64,
        var_phit1edge_dn5_slot: &mut f64,
        var_phit1edge_dn6_slot: &mut f64,
        var_phit1edge_dn7_slot: &mut f64,
        var_phit1edge_dn8_slot: &mut f64,
        var_phit1edge_rv_slot: &mut f64,
        var_qdseffedge_slot: &mut f64,
        var_qdseffedge_dn5_slot: &mut f64,
        var_qdseffedge_dn6_slot: &mut f64,
        var_qdseffedge_dn7_slot: &mut f64,
        var_qdseffedge_dn8_slot: &mut f64,
        var_qdseffedge_rv_slot: &mut f64,
        var_qmeffedge_slot: &mut f64,
        var_qmeffedge_dn5_slot: &mut f64,
        var_qmeffedge_dn6_slot: &mut f64,
        var_qmeffedge_dn7_slot: &mut f64,
        var_qmeffedge_dn8_slot: &mut f64,
        var_qmeffedge_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_u0_slot: &mut f64,
        var_u0_dn5_slot: &mut f64,
        var_u0_dn6_slot: &mut f64,
        var_u0_dn7_slot: &mut f64,
        var_u0_dn8_slot: &mut f64,
        var_u0_rv_slot: &mut f64,
        var_vtovd_slot: &mut f64,
        var_vtovd_dn5_slot: &mut f64,
        var_vtovd_dn6_slot: &mut f64,
        var_vtovd_dn7_slot: &mut f64,
        var_vtovd_dn8_slot: &mut f64,
        var_vtovd_rv_slot: &mut f64,
        var_vtovs_slot: &mut f64,
        var_vtovs_dn5_slot: &mut f64,
        var_vtovs_dn6_slot: &mut f64,
        var_vtovs_dn7_slot: &mut f64,
        var_vtovs_dn8_slot: &mut f64,
        var_vtovs_rv_slot: &mut f64,
        var_x_slot: &mut f64,
        var_x_dn5_slot: &mut f64,
        var_x_dn6_slot: &mut f64,
        var_x_dn7_slot: &mut f64,
        var_x_dn8_slot: &mut f64,
        var_x_rv_slot: &mut f64,
        var_xgedge_slot: &mut f64,
        var_xgedge_dn5_slot: &mut f64,
        var_xgedge_dn6_slot: &mut f64,
        var_xgedge_dn7_slot: &mut f64,
        var_xgedge_dn8_slot: &mut f64,
        var_xgedge_rv_slot: &mut f64,
    ) {
        let mut var_arg1: f64 = *var_arg1_slot;
        let mut var_arg1_dn5: f64 = *var_arg1_dn5_slot;
        let mut var_arg1_dn6: f64 = *var_arg1_dn6_slot;
        let mut var_arg1_dn7: f64 = *var_arg1_dn7_slot;
        let mut var_arg1_dn8: f64 = *var_arg1_dn8_slot;
        let mut var_arg1_rv: f64 = *var_arg1_rv_slot;
        let mut var_dsqredge: f64 = *var_dsqredge_slot;
        let mut var_dsqredge_dn5: f64 = *var_dsqredge_dn5_slot;
        let mut var_dsqredge_dn6: f64 = *var_dsqredge_dn6_slot;
        let mut var_dsqredge_dn7: f64 = *var_dsqredge_dn7_slot;
        let mut var_dsqredge_dn8: f64 = *var_dsqredge_dn8_slot;
        let mut var_dsqredge_rv: f64 = *var_dsqredge_rv_slot;
        let mut var_ex: f64 = *var_ex_slot;
        let mut var_ex_dn5: f64 = *var_ex_dn5_slot;
        let mut var_ex_dn6: f64 = *var_ex_dn6_slot;
        let mut var_ex_dn7: f64 = *var_ex_dn7_slot;
        let mut var_ex_dn8: f64 = *var_ex_dn8_slot;
        let mut var_ex_rv: f64 = *var_ex_rv_slot;
        let mut var_guard1236: f64 = *var_guard1236_slot;
        let mut var_guard1236_rv: f64 = *var_guard1236_rv_slot;
        let mut var_guard1237: f64 = *var_guard1237_slot;
        let mut var_guard1237_rv: f64 = *var_guard1237_rv_slot;
        let mut var_guard1240: f64 = *var_guard1240_slot;
        let mut var_guard1240_rv: f64 = *var_guard1240_rv_slot;
        let mut var_guard1241: f64 = *var_guard1241_slot;
        let mut var_guard1241_rv: f64 = *var_guard1241_rv_slot;
        let mut var_guard1242: f64 = *var_guard1242_slot;
        let mut var_guard1242_rv: f64 = *var_guard1242_rv_slot;
        let mut var_guard1243: f64 = *var_guard1243_slot;
        let mut var_guard1243_rv: f64 = *var_guard1243_rv_slot;
        let mut var_guard1244: f64 = *var_guard1244_slot;
        let mut var_guard1244_rv: f64 = *var_guard1244_rv_slot;
        let mut var_guard1245: f64 = *var_guard1245_slot;
        let mut var_guard1245_rv: f64 = *var_guard1245_rv_slot;
        let mut var_guard1246: f64 = *var_guard1246_slot;
        let mut var_guard1246_rv: f64 = *var_guard1246_rv_slot;
        let mut var_guard1247: f64 = *var_guard1247_slot;
        let mut var_guard1247_rv: f64 = *var_guard1247_rv_slot;
        let mut var_guard1248: f64 = *var_guard1248_slot;
        let mut var_guard1248_rv: f64 = *var_guard1248_rv_slot;
        let mut var_inv_ex: f64 = *var_inv_ex_slot;
        let mut var_inv_ex_dn5: f64 = *var_inv_ex_dn5_slot;
        let mut var_inv_ex_dn6: f64 = *var_inv_ex_dn6_slot;
        let mut var_inv_ex_dn7: f64 = *var_inv_ex_dn7_slot;
        let mut var_inv_ex_dn8: f64 = *var_inv_ex_dn8_slot;
        let mut var_inv_ex_rv: f64 = *var_inv_ex_rv_slot;
        let mut var_phit1edge: f64 = *var_phit1edge_slot;
        let mut var_phit1edge_dn5: f64 = *var_phit1edge_dn5_slot;
        let mut var_phit1edge_dn6: f64 = *var_phit1edge_dn6_slot;
        let mut var_phit1edge_dn7: f64 = *var_phit1edge_dn7_slot;
        let mut var_phit1edge_dn8: f64 = *var_phit1edge_dn8_slot;
        let mut var_phit1edge_rv: f64 = *var_phit1edge_rv_slot;
        let mut var_qdseffedge: f64 = *var_qdseffedge_slot;
        let mut var_qdseffedge_dn5: f64 = *var_qdseffedge_dn5_slot;
        let mut var_qdseffedge_dn6: f64 = *var_qdseffedge_dn6_slot;
        let mut var_qdseffedge_dn7: f64 = *var_qdseffedge_dn7_slot;
        let mut var_qdseffedge_dn8: f64 = *var_qdseffedge_dn8_slot;
        let mut var_qdseffedge_rv: f64 = *var_qdseffedge_rv_slot;
        let mut var_qmeffedge: f64 = *var_qmeffedge_slot;
        let mut var_qmeffedge_dn5: f64 = *var_qmeffedge_dn5_slot;
        let mut var_qmeffedge_dn6: f64 = *var_qmeffedge_dn6_slot;
        let mut var_qmeffedge_dn7: f64 = *var_qmeffedge_dn7_slot;
        let mut var_qmeffedge_dn8: f64 = *var_qmeffedge_dn8_slot;
        let mut var_qmeffedge_rv: f64 = *var_qmeffedge_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_u0: f64 = *var_u0_slot;
        let mut var_u0_dn5: f64 = *var_u0_dn5_slot;
        let mut var_u0_dn6: f64 = *var_u0_dn6_slot;
        let mut var_u0_dn7: f64 = *var_u0_dn7_slot;
        let mut var_u0_dn8: f64 = *var_u0_dn8_slot;
        let mut var_u0_rv: f64 = *var_u0_rv_slot;
        let mut var_vtovd: f64 = *var_vtovd_slot;
        let mut var_vtovd_dn5: f64 = *var_vtovd_dn5_slot;
        let mut var_vtovd_dn6: f64 = *var_vtovd_dn6_slot;
        let mut var_vtovd_dn7: f64 = *var_vtovd_dn7_slot;
        let mut var_vtovd_dn8: f64 = *var_vtovd_dn8_slot;
        let mut var_vtovd_rv: f64 = *var_vtovd_rv_slot;
        let mut var_vtovs: f64 = *var_vtovs_slot;
        let mut var_vtovs_dn5: f64 = *var_vtovs_dn5_slot;
        let mut var_vtovs_dn6: f64 = *var_vtovs_dn6_slot;
        let mut var_vtovs_dn7: f64 = *var_vtovs_dn7_slot;
        let mut var_vtovs_dn8: f64 = *var_vtovs_dn8_slot;
        let mut var_vtovs_rv: f64 = *var_vtovs_rv_slot;
        let mut var_x: f64 = *var_x_slot;
        let mut var_x_dn5: f64 = *var_x_dn5_slot;
        let mut var_x_dn6: f64 = *var_x_dn6_slot;
        let mut var_x_dn7: f64 = *var_x_dn7_slot;
        let mut var_x_dn8: f64 = *var_x_dn8_slot;
        let mut var_x_rv: f64 = *var_x_rv_slot;
        let mut var_xgedge: f64 = *var_xgedge_slot;
        let mut var_xgedge_dn5: f64 = *var_xgedge_dn5_slot;
        let mut var_xgedge_dn6: f64 = *var_xgedge_dn6_slot;
        let mut var_xgedge_dn7: f64 = *var_xgedge_dn7_slot;
        let mut var_xgedge_dn8: f64 = *var_xgedge_dn8_slot;
        let mut var_xgedge_rv: f64 = *var_xgedge_rv_slot;

        let (assign46660_e59826, assign46660_e59826_d_n5, assign46660_e59826_d_n6, assign46660_e59826_d_n7, assign46660_e59826_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46660_e59819: f64 = (var_v_gs + var_vsbstar_dc);
        let assign46660_e59821: f64 = (assign46660_e59819 - var_vm);
        let assign46660_e59822: f64 = (-assign46660_e59821);
        let assign46660_e59824: f64 = (assign46660_e59822 * var_inv_phit1_dc);
        (assign46660_e59824, (((-((var_v_gs_dn5 + var_vsbstar_dc_dn5) - var_vm_dn5)) * var_inv_phit1_dc) + (assign46660_e59822 * var_inv_phit1_dc_dn5)), (((-((var_v_gs_dn6 + var_vsbstar_dc_dn6) - var_vm_dn6)) * var_inv_phit1_dc) + (assign46660_e59822 * var_inv_phit1_dc_dn6)), (((-((var_v_gs_dn7 + var_vsbstar_dc_dn7) - var_vm_dn7)) * var_inv_phit1_dc) + (assign46660_e59822 * var_inv_phit1_dc_dn7)), (((-(var_vsbstar_dc_dn8 - var_vm_dn8)) * var_inv_phit1_dc) + (assign46660_e59822 * var_inv_phit1_dc_dn8)),)
    } else {
        (var_arg1, var_arg1_dn5, var_arg1_dn6, var_arg1_dn7, var_arg1_dn8,)
    }
};
        var_arg1 = assign46660_e59826;
        var_arg1_dn5 = assign46660_e59826_d_n5;
        var_arg1_dn6 = assign46660_e59826_d_n6;
        var_arg1_dn7 = assign46660_e59826_d_n7;
        var_arg1_dn8 = assign46660_e59826_d_n8;
        var_arg1_rv = 0.0;

        let assign46670_e59828: f64 = (var_arg1).abs();
        let assign46670_e59830: f64 = if assign46670_e59828 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1236 = assign46670_e59830;
        var_guard1236_rv = 0.0;

        let (assign46680_e59839, assign46680_e59839_d_n5, assign46680_e59839_d_n6, assign46680_e59839_d_n7, assign46680_e59839_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1236 != 0.0)) {
        let assign46680_e59837: f64 = (var_arg1).exp();
        (assign46680_e59837, (assign46680_e59837 * var_arg1_dn5), (assign46680_e59837 * var_arg1_dn6), (assign46680_e59837 * var_arg1_dn7), (assign46680_e59837 * var_arg1_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46680_e59839;
        var_temp__blk936_dn5 = assign46680_e59839_d_n5;
        var_temp__blk936_dn6 = assign46680_e59839_d_n6;
        var_temp__blk936_dn7 = assign46680_e59839_d_n7;
        var_temp__blk936_dn8 = assign46680_e59839_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign46690_e59842: f64 = if var_arg1 < 0.0 { 1.0 } else { 0.0 };
        var_guard1237 = assign46690_e59842;
        var_guard1237_rv = 0.0;

        let (assign46700_e59878, assign46700_e59878_d_n5, assign46700_e59878_d_n6, assign46700_e59878_d_n7, assign46700_e59878_d_n8,) = {
    if ((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1236 == 0.0)) && (var_guard1237 != 0.0)) {
        let assign46700_e59854: f64 = (-230.25850929940458);
        let assign46700_e59856: f64 = (assign46700_e59854 - var_arg1);
        let assign46700_e59860: f64 = (-230.25850929940458);
        let assign46700_e59862: f64 = (assign46700_e59860 - var_arg1);
        let assign46700_e59865: f64 = (-230.25850929940458);
        let assign46700_e59867: f64 = (assign46700_e59865 - var_arg1);
        let assign46700_e59869: f64 = (assign46700_e59867 * 0.3333333333333333);
        let assign46700_e59870: f64 = (1.0 + assign46700_e59869);
        let assign46700_e59871: f64 = (assign46700_e59862 * assign46700_e59870);
        let assign46700_e59872: f64 = (0.5 * assign46700_e59871);
        let assign46700_e59873: f64 = (1.0 + assign46700_e59872);
        let assign46700_e59874: f64 = (assign46700_e59856 * assign46700_e59873);
        let assign46700_e59875: f64 = (1.0 + assign46700_e59874);
        let assign46700_e59876: f64 = (1e-100 / assign46700_e59875);
        (assign46700_e59876, (-((1e-100 * (((-var_arg1_dn5) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-var_arg1_dn5) * assign46700_e59870) + (assign46700_e59862 * ((-var_arg1_dn5) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-var_arg1_dn6) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-var_arg1_dn6) * assign46700_e59870) + (assign46700_e59862 * ((-var_arg1_dn6) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-var_arg1_dn7) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-var_arg1_dn7) * assign46700_e59870) + (assign46700_e59862 * ((-var_arg1_dn7) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))), (-((1e-100 * (((-var_arg1_dn8) * assign46700_e59873) + (assign46700_e59856 * (0.5 * (((-var_arg1_dn8) * assign46700_e59870) + (assign46700_e59862 * ((-var_arg1_dn8) * 0.3333333333333333))))))) / (assign46700_e59875 * assign46700_e59875))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46700_e59878;
        var_temp__blk936_dn5 = assign46700_e59878_d_n5;
        var_temp__blk936_dn6 = assign46700_e59878_d_n6;
        var_temp__blk936_dn7 = assign46700_e59878_d_n7;
        var_temp__blk936_dn8 = assign46700_e59878_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46710_e59912, assign46710_e59912_d_n5, assign46710_e59912_d_n6, assign46710_e59912_d_n7, assign46710_e59912_d_n8,) = {
    if ((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1236 == 0.0)) && (var_guard1237 == 0.0)) {
        let assign46710_e59892: f64 = (var_arg1 - 230.25850929940458);
        let assign46710_e59897: f64 = (var_arg1 - 230.25850929940458);
        let assign46710_e59901: f64 = (var_arg1 - 230.25850929940458);
        let assign46710_e59903: f64 = (assign46710_e59901 * 0.3333333333333333);
        let assign46710_e59904: f64 = (1.0 + assign46710_e59903);
        let assign46710_e59905: f64 = (assign46710_e59897 * assign46710_e59904);
        let assign46710_e59906: f64 = (0.5 * assign46710_e59905);
        let assign46710_e59907: f64 = (1.0 + assign46710_e59906);
        let assign46710_e59908: f64 = (assign46710_e59892 * assign46710_e59907);
        let assign46710_e59909: f64 = (1.0 + assign46710_e59908);
        let assign46710_e59910: f64 = (1e100 * assign46710_e59909);
        (assign46710_e59910, (1e100 * ((var_arg1_dn5 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((var_arg1_dn5 * assign46710_e59904) + (assign46710_e59897 * (var_arg1_dn5 * 0.3333333333333333))))))), (1e100 * ((var_arg1_dn6 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((var_arg1_dn6 * assign46710_e59904) + (assign46710_e59897 * (var_arg1_dn6 * 0.3333333333333333))))))), (1e100 * ((var_arg1_dn7 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((var_arg1_dn7 * assign46710_e59904) + (assign46710_e59897 * (var_arg1_dn7 * 0.3333333333333333))))))), (1e100 * ((var_arg1_dn8 * assign46710_e59907) + (assign46710_e59892 * (0.5 * ((var_arg1_dn8 * assign46710_e59904) + (assign46710_e59897 * (var_arg1_dn8 * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46710_e59912;
        var_temp__blk936_dn5 = assign46710_e59912_d_n5;
        var_temp__blk936_dn6 = assign46710_e59912_d_n6;
        var_temp__blk936_dn7 = assign46710_e59912_d_n7;
        var_temp__blk936_dn8 = assign46710_e59912_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46730_e59937, assign46730_e59937_d_n5, assign46730_e59937_d_n6, assign46730_e59937_d_n7, assign46730_e59937_d_n8,) = {
    if ((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) {
        let assign46730_e59926: f64 = (-1.5);
        let assign46730_e59931: f64 = (var_gc3_i * var_zg);
        let assign46730_e59932: f64 = (var_gc2_i + assign46730_e59931);
        let assign46730_e59933: f64 = (var_zg * assign46730_e59932);
        let assign46730_e59934: f64 = (assign46730_e59926 + assign46730_e59933);
        let assign46730_e59935: f64 = (var_bch * assign46730_e59934);
        (assign46730_e59935, (var_bch * ((var_zg_dn5 * assign46730_e59932) + (var_zg * (var_gc3_i * var_zg_dn5)))), (var_bch * ((var_zg_dn6 * assign46730_e59932) + (var_zg * (var_gc3_i * var_zg_dn6)))), (var_bch * ((var_zg_dn7 * assign46730_e59932) + (var_zg * (var_gc3_i * var_zg_dn7)))), (var_bch * ((var_zg_dn8 * assign46730_e59932) + (var_zg * (var_gc3_i * var_zg_dn8)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46730_e59937;
        var_temp__blk936_dn5 = assign46730_e59937_d_n5;
        var_temp__blk936_dn6 = assign46730_e59937_d_n6;
        var_temp__blk936_dn7 = assign46730_e59937_d_n7;
        var_temp__blk936_dn8 = assign46730_e59937_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign46800_e60043: f64 = if ((var_xg_dc <= 0.0) || ((var_gc2_i == 0.0) && (var_gc3_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard1240 = assign46800_e60043;
        var_guard1240_rv = 0.0;

        let (assign46830_e60074, assign46830_e60074_d_n5, assign46830_e60074_d_n6, assign46830_e60074_d_n7, assign46830_e60074_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) {
        let assign46830_e60069: f64 = (2.0 * var_gc3_i);
        let assign46830_e60071: f64 = (assign46830_e60069 * var_zg);
        let assign46830_e60072: f64 = (var_gc2_i + assign46830_e60071);
        (assign46830_e60072, (assign46830_e60069 * var_zg_dn5), (assign46830_e60069 * var_zg_dn6), (assign46830_e60069 * var_zg_dn7), (assign46830_e60069 * var_zg_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign46830_e60074;
        var_temp__blk936_dn5 = assign46830_e60074_d_n5;
        var_temp__blk936_dn6 = assign46830_e60074_d_n6;
        var_temp__blk936_dn7 = assign46830_e60074_d_n7;
        var_temp__blk936_dn8 = assign46830_e60074_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign46840_e60087, assign46840_e60087_d_n5, assign46840_e60087_d_n6, assign46840_e60087_d_n7, assign46840_e60087_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) {
        let assign46840_e60084: f64 = (var_temp__blk936 * var_bch);
        let assign46840_e60085: f64 = (var_chib_i / assign46840_e60084);
        (assign46840_e60085, (-((var_chib_i * (var_temp__blk936_dn5 * var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((var_chib_i * (var_temp__blk936_dn6 * var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((var_chib_i * (var_temp__blk936_dn7 * var_bch)) / (assign46840_e60084 * assign46840_e60084))), (-((var_chib_i * (var_temp__blk936_dn8 * var_bch)) / (assign46840_e60084 * assign46840_e60084))),)
    } else {
        (var_u0, var_u0_dn5, var_u0_dn6, var_u0_dn7, var_u0_dn8,)
    }
};
        var_u0 = assign46840_e60087;
        var_u0_dn5 = assign46840_e60087_d_n5;
        var_u0_dn6 = assign46840_e60087_d_n6;
        var_u0_dn7 = assign46840_e60087_d_n7;
        var_u0_dn8 = assign46840_e60087_d_n8;
        var_u0_rv = 0.0;

        let (assign46850_e60100, assign46850_e60100_d_n5, assign46850_e60100_d_n6, assign46850_e60100_d_n7, assign46850_e60100_d_n8,) = {
    if (((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) {
        let assign46850_e60097: f64 = (var_dps_dc / var_u0);
        let assign46850_e60098: f64 = (0.5 * assign46850_e60097);
        (assign46850_e60098, (0.5 * (((var_dps_dc_dn5 * var_u0) - (var_dps_dc * var_u0_dn5)) / (var_u0 * var_u0))), (0.5 * (((var_dps_dc_dn6 * var_u0) - (var_dps_dc * var_u0_dn6)) / (var_u0 * var_u0))), (0.5 * (((var_dps_dc_dn7 * var_u0) - (var_dps_dc * var_u0_dn7)) / (var_u0 * var_u0))), (0.5 * (((var_dps_dc_dn8 * var_u0) - (var_dps_dc * var_u0_dn8)) / (var_u0 * var_u0))),)
    } else {
        (var_x, var_x_dn5, var_x_dn6, var_x_dn7, var_x_dn8,)
    }
};
        var_x = assign46850_e60100;
        var_x_dn5 = assign46850_e60100_d_n5;
        var_x_dn6 = assign46850_e60100_d_n6;
        var_x_dn7 = assign46850_e60100_d_n7;
        var_x_dn8 = assign46850_e60100_d_n8;
        var_x_rv = 0.0;

        let assign46890_e60142: f64 = if var_x < 0.001 { 1.0 } else { 0.0 };
        var_guard1241 = assign46890_e60142;
        var_guard1241_rv = 0.0;

        let assign46940_e60235: f64 = (var_x).abs();
        let assign46940_e60237: f64 = if assign46940_e60235 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1242 = assign46940_e60237;
        var_guard1242_rv = 0.0;

        let (assign46950_e60252, assign46950_e60252_d_n5, assign46950_e60252_d_n6, assign46950_e60252_d_n7, assign46950_e60252_d_n8,) = {
    if (((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) && (var_guard1242 != 0.0)) {
        let assign46950_e60250: f64 = (var_x).exp();
        (assign46950_e60250, (assign46950_e60250 * var_x_dn5), (assign46950_e60250 * var_x_dn6), (assign46950_e60250 * var_x_dn7), (assign46950_e60250 * var_x_dn8),)
    } else {
        (var_ex, var_ex_dn5, var_ex_dn6, var_ex_dn7, var_ex_dn8,)
    }
};
        var_ex = assign46950_e60252;
        var_ex_dn5 = assign46950_e60252_d_n5;
        var_ex_dn6 = assign46950_e60252_d_n6;
        var_ex_dn7 = assign46950_e60252_d_n7;
        var_ex_dn8 = assign46950_e60252_d_n8;
        var_ex_rv = 0.0;

        let assign46960_e60255: f64 = if var_x < 0.0 { 1.0 } else { 0.0 };
        var_guard1243 = assign46960_e60255;
        var_guard1243_rv = 0.0;

        let (assign46970_e60297, assign46970_e60297_d_n5, assign46970_e60297_d_n6, assign46970_e60297_d_n7, assign46970_e60297_d_n8,) = {
    if ((((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) && (var_guard1242 == 0.0)) && (var_guard1243 != 0.0)) {
        let assign46970_e60273: f64 = (-230.25850929940458);
        let assign46970_e60275: f64 = (assign46970_e60273 - var_x);
        let assign46970_e60279: f64 = (-230.25850929940458);
        let assign46970_e60281: f64 = (assign46970_e60279 - var_x);
        let assign46970_e60284: f64 = (-230.25850929940458);
        let assign46970_e60286: f64 = (assign46970_e60284 - var_x);
        let assign46970_e60288: f64 = (assign46970_e60286 * 0.3333333333333333);
        let assign46970_e60289: f64 = (1.0 + assign46970_e60288);
        let assign46970_e60290: f64 = (assign46970_e60281 * assign46970_e60289);
        let assign46970_e60291: f64 = (0.5 * assign46970_e60290);
        let assign46970_e60292: f64 = (1.0 + assign46970_e60291);
        let assign46970_e60293: f64 = (assign46970_e60275 * assign46970_e60292);
        let assign46970_e60294: f64 = (1.0 + assign46970_e60293);
        let assign46970_e60295: f64 = (1e-100 / assign46970_e60294);
        (assign46970_e60295, (-((1e-100 * (((-var_x_dn5) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-var_x_dn5) * assign46970_e60289) + (assign46970_e60281 * ((-var_x_dn5) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-var_x_dn6) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-var_x_dn6) * assign46970_e60289) + (assign46970_e60281 * ((-var_x_dn6) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-var_x_dn7) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-var_x_dn7) * assign46970_e60289) + (assign46970_e60281 * ((-var_x_dn7) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))), (-((1e-100 * (((-var_x_dn8) * assign46970_e60292) + (assign46970_e60275 * (0.5 * (((-var_x_dn8) * assign46970_e60289) + (assign46970_e60281 * ((-var_x_dn8) * 0.3333333333333333))))))) / (assign46970_e60294 * assign46970_e60294))),)
    } else {
        (var_ex, var_ex_dn5, var_ex_dn6, var_ex_dn7, var_ex_dn8,)
    }
};
        var_ex = assign46970_e60297;
        var_ex_dn5 = assign46970_e60297_d_n5;
        var_ex_dn6 = assign46970_e60297_d_n6;
        var_ex_dn7 = assign46970_e60297_d_n7;
        var_ex_dn8 = assign46970_e60297_d_n8;
        var_ex_rv = 0.0;

        let (assign46980_e60337, assign46980_e60337_d_n5, assign46980_e60337_d_n6, assign46980_e60337_d_n7, assign46980_e60337_d_n8,) = {
    if ((((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) && (var_guard1242 == 0.0)) && (var_guard1243 == 0.0)) {
        let assign46980_e60317: f64 = (var_x - 230.25850929940458);
        let assign46980_e60322: f64 = (var_x - 230.25850929940458);
        let assign46980_e60326: f64 = (var_x - 230.25850929940458);
        let assign46980_e60328: f64 = (assign46980_e60326 * 0.3333333333333333);
        let assign46980_e60329: f64 = (1.0 + assign46980_e60328);
        let assign46980_e60330: f64 = (assign46980_e60322 * assign46980_e60329);
        let assign46980_e60331: f64 = (0.5 * assign46980_e60330);
        let assign46980_e60332: f64 = (1.0 + assign46980_e60331);
        let assign46980_e60333: f64 = (assign46980_e60317 * assign46980_e60332);
        let assign46980_e60334: f64 = (1.0 + assign46980_e60333);
        let assign46980_e60335: f64 = (1e100 * assign46980_e60334);
        (assign46980_e60335, (1e100 * ((var_x_dn5 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((var_x_dn5 * assign46980_e60329) + (assign46980_e60322 * (var_x_dn5 * 0.3333333333333333))))))), (1e100 * ((var_x_dn6 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((var_x_dn6 * assign46980_e60329) + (assign46980_e60322 * (var_x_dn6 * 0.3333333333333333))))))), (1e100 * ((var_x_dn7 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((var_x_dn7 * assign46980_e60329) + (assign46980_e60322 * (var_x_dn7 * 0.3333333333333333))))))), (1e100 * ((var_x_dn8 * assign46980_e60332) + (assign46980_e60317 * (0.5 * ((var_x_dn8 * assign46980_e60329) + (assign46980_e60322 * (var_x_dn8 * 0.3333333333333333))))))),)
    } else {
        (var_ex, var_ex_dn5, var_ex_dn6, var_ex_dn7, var_ex_dn8,)
    }
};
        var_ex = assign46980_e60337;
        var_ex_dn5 = assign46980_e60337_d_n5;
        var_ex_dn6 = assign46980_e60337_d_n6;
        var_ex_dn7 = assign46980_e60337_d_n7;
        var_ex_dn8 = assign46980_e60337_d_n8;
        var_ex_rv = 0.0;

        let (assign46990_e60351, assign46990_e60351_d_n5, assign46990_e60351_d_n6, assign46990_e60351_d_n7, assign46990_e60351_d_n8,) = {
    if ((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) {
        let assign46990_e60349: f64 = (1.0 / var_ex);
        (assign46990_e60349, (-(var_ex_dn5 / (var_ex * var_ex))), (-(var_ex_dn6 / (var_ex * var_ex))), (-(var_ex_dn7 / (var_ex * var_ex))), (-(var_ex_dn8 / (var_ex * var_ex))),)
    } else {
        (var_inv_ex, var_inv_ex_dn5, var_inv_ex_dn6, var_inv_ex_dn7, var_inv_ex_dn8,)
    }
};
        var_inv_ex = assign46990_e60351;
        var_inv_ex_dn5 = assign46990_e60351_d_n5;
        var_inv_ex_dn6 = assign46990_e60351_d_n6;
        var_inv_ex_dn7 = assign46990_e60351_d_n7;
        var_inv_ex_dn8 = assign46990_e60351_d_n8;
        var_inv_ex_rv = 0.0;

        let (assign47000_e60365, assign47000_e60365_d_n5, assign47000_e60365_d_n6, assign47000_e60365_d_n7, assign47000_e60365_d_n8,) = {
    if ((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) {
        let assign47000_e60363: f64 = (var_ex - var_inv_ex);
        (assign47000_e60363, (var_ex_dn5 - var_inv_ex_dn5), (var_ex_dn6 - var_inv_ex_dn6), (var_ex_dn7 - var_inv_ex_dn7), (var_ex_dn8 - var_inv_ex_dn8),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47000_e60365;
        var_temp__blk936_dn5 = assign47000_e60365_d_n5;
        var_temp__blk936_dn6 = assign47000_e60365_d_n6;
        var_temp__blk936_dn7 = assign47000_e60365_d_n7;
        var_temp__blk936_dn8 = assign47000_e60365_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign47010_e60379, assign47010_e60379_d_n5, assign47010_e60379_d_n6, assign47010_e60379_d_n7, assign47010_e60379_d_n8,) = {
    if ((((var_guard1221 != 0.0) && (var_guard1230 != 0.0)) && (var_guard1240 == 0.0)) && (var_guard1241 == 0.0)) {
        let assign47010_e60377: f64 = (var_ex + var_inv_ex);
        (assign47010_e60377, (var_ex_dn5 + var_inv_ex_dn5), (var_ex_dn6 + var_inv_ex_dn6), (var_ex_dn7 + var_inv_ex_dn7), (var_ex_dn8 + var_inv_ex_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign47010_e60379;
        var_temp2_dn5 = assign47010_e60379_d_n5;
        var_temp2_dn6 = assign47010_e60379_d_n6;
        var_temp2_dn7 = assign47010_e60379_d_n7;
        var_temp2_dn8 = assign47010_e60379_d_n8;
        var_temp2_rv = 0.0;

        let assign47110_e60495: f64 = if p.p42 != 0.0 { 1.0 } else { 0.0 };
        var_guard1244 = assign47110_e60495;
        var_guard1244_rv = 0.0;

        let assign47120_e60502: f64 = if ((var_agidld_i > 0.0) && (var_vovd < 0.0)) { 1.0 } else { 0.0 };
        var_guard1245 = assign47120_e60502;
        var_guard1245_rv = 0.0;

        let (assign47130_e60521, assign47130_e60521_d_n5, assign47130_e60521_d_n6, assign47130_e60521_d_n7, assign47130_e60521_d_n8,) = {
    if ((var_guard1244 != 0.0) && (var_guard1245 != 0.0)) {
        let assign47130_e60508: f64 = (var_vovd * var_vovd);
        let assign47130_e60511: f64 = (var_cgidld_i * var_cgidld_i);
        let assign47130_e60514: f64 = (var_vdbprime * var_vdbprime);
        let assign47130_e60515: f64 = (assign47130_e60511 * assign47130_e60514);
        let assign47130_e60516: f64 = (assign47130_e60508 + assign47130_e60515);
        let assign47130_e60518: f64 = (assign47130_e60516 + 1e-6);
        let assign47130_e60519: f64 = (assign47130_e60518).sqrt();
        (assign47130_e60519, (((var_vovd_dn5 * var_vovd) + (var_vovd * var_vovd_dn5)) / (2.0 * assign47130_e60519)), ((((var_vovd_dn6 * var_vovd) + (var_vovd * var_vovd_dn6)) + (assign47130_e60511 * ((var_vdbprime_dn6 * var_vdbprime) + (var_vdbprime * var_vdbprime_dn6)))) / (2.0 * assign47130_e60519)), ((((var_vovd_dn7 * var_vovd) + (var_vovd * var_vovd_dn7)) + (assign47130_e60511 * ((var_vdbprime_dn7 * var_vdbprime) + (var_vdbprime * var_vdbprime_dn7)))) / (2.0 * assign47130_e60519)), ((assign47130_e60511 * ((var_vdbprime_dn8 * var_vdbprime) + (var_vdbprime * var_vdbprime_dn8))) / (2.0 * assign47130_e60519)),)
    } else {
        (var_vtovd, var_vtovd_dn5, var_vtovd_dn6, var_vtovd_dn7, var_vtovd_dn8,)
    }
};
        var_vtovd = assign47130_e60521;
        var_vtovd_dn5 = assign47130_e60521_d_n5;
        var_vtovd_dn6 = assign47130_e60521_d_n6;
        var_vtovd_dn7 = assign47130_e60521_d_n7;
        var_vtovd_dn8 = assign47130_e60521_d_n8;
        var_vtovd_rv = 0.0;

        let (assign47140_e60530, assign47140_e60530_d_n5, assign47140_e60530_d_n6, assign47140_e60530_d_n7, assign47140_e60530_d_n8,) = {
    if ((var_guard1244 != 0.0) && (var_guard1245 != 0.0)) {
        let assign47140_e60526: f64 = (-var_bgidlds);
        let assign47140_e60528: f64 = (assign47140_e60526 / var_vtovd);
        (assign47140_e60528, (-((assign47140_e60526 * var_vtovd_dn5) / (var_vtovd * var_vtovd))), (-((assign47140_e60526 * var_vtovd_dn6) / (var_vtovd * var_vtovd))), (-((assign47140_e60526 * var_vtovd_dn7) / (var_vtovd * var_vtovd))), (-((assign47140_e60526 * var_vtovd_dn8) / (var_vtovd * var_vtovd))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47140_e60530;
        var_temp__blk936_dn5 = assign47140_e60530_d_n5;
        var_temp__blk936_dn6 = assign47140_e60530_d_n6;
        var_temp__blk936_dn7 = assign47140_e60530_d_n7;
        var_temp__blk936_dn8 = assign47140_e60530_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign47150_e60533: f64 = (-230.25850929940458);
        let assign47150_e60534: f64 = if var_temp__blk936 > assign47150_e60533 { 1.0 } else { 0.0 };
        var_guard1246 = assign47150_e60534;
        var_guard1246_rv = 0.0;

        let (assign47160_e60543, assign47160_e60543_d_n5, assign47160_e60543_d_n6, assign47160_e60543_d_n7, assign47160_e60543_d_n8,) = {
    if (((var_guard1244 != 0.0) && (var_guard1245 != 0.0)) && (var_guard1246 != 0.0)) {
        let assign47160_e60541: f64 = (var_temp__blk936).exp();
        (assign47160_e60541, (assign47160_e60541 * var_temp__blk936_dn5), (assign47160_e60541 * var_temp__blk936_dn6), (assign47160_e60541 * var_temp__blk936_dn7), (assign47160_e60541 * var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign47160_e60543;
        var_temp2_dn5 = assign47160_e60543_d_n5;
        var_temp2_dn6 = assign47160_e60543_d_n6;
        var_temp2_dn7 = assign47160_e60543_d_n7;
        var_temp2_dn8 = assign47160_e60543_d_n8;
        var_temp2_rv = 0.0;

        let (assign47170_e60577, assign47170_e60577_d_n5, assign47170_e60577_d_n6, assign47170_e60577_d_n7, assign47170_e60577_d_n8,) = {
    if (((var_guard1244 != 0.0) && (var_guard1245 != 0.0)) && (var_guard1246 == 0.0)) {
        let assign47170_e60553: f64 = (-230.25850929940458);
        let assign47170_e60555: f64 = (assign47170_e60553 - var_temp__blk936);
        let assign47170_e60559: f64 = (-230.25850929940458);
        let assign47170_e60561: f64 = (assign47170_e60559 - var_temp__blk936);
        let assign47170_e60564: f64 = (-230.25850929940458);
        let assign47170_e60566: f64 = (assign47170_e60564 - var_temp__blk936);
        let assign47170_e60568: f64 = (assign47170_e60566 * 0.3333333333333333);
        let assign47170_e60569: f64 = (1.0 + assign47170_e60568);
        let assign47170_e60570: f64 = (assign47170_e60561 * assign47170_e60569);
        let assign47170_e60571: f64 = (0.5 * assign47170_e60570);
        let assign47170_e60572: f64 = (1.0 + assign47170_e60571);
        let assign47170_e60573: f64 = (assign47170_e60555 * assign47170_e60572);
        let assign47170_e60574: f64 = (1.0 + assign47170_e60573);
        let assign47170_e60575: f64 = (1e-100 / assign47170_e60574);
        (assign47170_e60575, (-((1e-100 * (((-var_temp__blk936_dn5) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-var_temp__blk936_dn5) * assign47170_e60569) + (assign47170_e60561 * ((-var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-var_temp__blk936_dn6) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-var_temp__blk936_dn6) * assign47170_e60569) + (assign47170_e60561 * ((-var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-var_temp__blk936_dn7) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-var_temp__blk936_dn7) * assign47170_e60569) + (assign47170_e60561 * ((-var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))), (-((1e-100 * (((-var_temp__blk936_dn8) * assign47170_e60572) + (assign47170_e60555 * (0.5 * (((-var_temp__blk936_dn8) * assign47170_e60569) + (assign47170_e60561 * ((-var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47170_e60574 * assign47170_e60574))),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign47170_e60577;
        var_temp2_dn5 = assign47170_e60577_d_n5;
        var_temp2_dn6 = assign47170_e60577_d_n6;
        var_temp2_dn7 = assign47170_e60577_d_n7;
        var_temp2_dn8 = assign47170_e60577_d_n8;
        var_temp2_rv = 0.0;

        let assign47190_e60599: f64 = if ((var_agidl_i > 0.0) && (var_vovs < 0.0)) { 1.0 } else { 0.0 };
        var_guard1247 = assign47190_e60599;
        var_guard1247_rv = 0.0;

        let (assign47200_e60618, assign47200_e60618_d_n5, assign47200_e60618_d_n6, assign47200_e60618_d_n7, assign47200_e60618_d_n8,) = {
    if ((var_guard1244 != 0.0) && (var_guard1247 != 0.0)) {
        let assign47200_e60605: f64 = (var_vovs * var_vovs);
        let assign47200_e60608: f64 = (var_cgidl_i * var_cgidl_i);
        let assign47200_e60611: f64 = (var_vsbprime * var_vsbprime);
        let assign47200_e60612: f64 = (assign47200_e60608 * assign47200_e60611);
        let assign47200_e60613: f64 = (assign47200_e60605 + assign47200_e60612);
        let assign47200_e60615: f64 = (assign47200_e60613 + 1e-6);
        let assign47200_e60616: f64 = (assign47200_e60615).sqrt();
        (assign47200_e60616, (((var_vovs_dn5 * var_vovs) + (var_vovs * var_vovs_dn5)) / (2.0 * assign47200_e60616)), ((((var_vovs_dn6 * var_vovs) + (var_vovs * var_vovs_dn6)) + (assign47200_e60608 * ((var_vsbprime_dn6 * var_vsbprime) + (var_vsbprime * var_vsbprime_dn6)))) / (2.0 * assign47200_e60616)), ((((var_vovs_dn7 * var_vovs) + (var_vovs * var_vovs_dn7)) + (assign47200_e60608 * ((var_vsbprime_dn7 * var_vsbprime) + (var_vsbprime * var_vsbprime_dn7)))) / (2.0 * assign47200_e60616)), ((assign47200_e60608 * ((var_vsbprime_dn8 * var_vsbprime) + (var_vsbprime * var_vsbprime_dn8))) / (2.0 * assign47200_e60616)),)
    } else {
        (var_vtovs, var_vtovs_dn5, var_vtovs_dn6, var_vtovs_dn7, var_vtovs_dn8,)
    }
};
        var_vtovs = assign47200_e60618;
        var_vtovs_dn5 = assign47200_e60618_d_n5;
        var_vtovs_dn6 = assign47200_e60618_d_n6;
        var_vtovs_dn7 = assign47200_e60618_d_n7;
        var_vtovs_dn8 = assign47200_e60618_d_n8;
        var_vtovs_rv = 0.0;

        let (assign47210_e60627, assign47210_e60627_d_n5, assign47210_e60627_d_n6, assign47210_e60627_d_n7, assign47210_e60627_d_n8,) = {
    if ((var_guard1244 != 0.0) && (var_guard1247 != 0.0)) {
        let assign47210_e60623: f64 = (-var_bgidls);
        let assign47210_e60625: f64 = (assign47210_e60623 / var_vtovs);
        (assign47210_e60625, (-((assign47210_e60623 * var_vtovs_dn5) / (var_vtovs * var_vtovs))), (-((assign47210_e60623 * var_vtovs_dn6) / (var_vtovs * var_vtovs))), (-((assign47210_e60623 * var_vtovs_dn7) / (var_vtovs * var_vtovs))), (-((assign47210_e60623 * var_vtovs_dn8) / (var_vtovs * var_vtovs))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47210_e60627;
        var_temp__blk936_dn5 = assign47210_e60627_d_n5;
        var_temp__blk936_dn6 = assign47210_e60627_d_n6;
        var_temp__blk936_dn7 = assign47210_e60627_d_n7;
        var_temp__blk936_dn8 = assign47210_e60627_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign47220_e60630: f64 = (-230.25850929940458);
        let assign47220_e60631: f64 = if var_temp__blk936 > assign47220_e60630 { 1.0 } else { 0.0 };
        var_guard1248 = assign47220_e60631;
        var_guard1248_rv = 0.0;

        let (assign47230_e60640, assign47230_e60640_d_n5, assign47230_e60640_d_n6, assign47230_e60640_d_n7, assign47230_e60640_d_n8,) = {
    if (((var_guard1244 != 0.0) && (var_guard1247 != 0.0)) && (var_guard1248 != 0.0)) {
        let assign47230_e60638: f64 = (var_temp__blk936).exp();
        (assign47230_e60638, (assign47230_e60638 * var_temp__blk936_dn5), (assign47230_e60638 * var_temp__blk936_dn6), (assign47230_e60638 * var_temp__blk936_dn7), (assign47230_e60638 * var_temp__blk936_dn8),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign47230_e60640;
        var_temp2_dn5 = assign47230_e60640_d_n5;
        var_temp2_dn6 = assign47230_e60640_d_n6;
        var_temp2_dn7 = assign47230_e60640_d_n7;
        var_temp2_dn8 = assign47230_e60640_d_n8;
        var_temp2_rv = 0.0;

        let (assign47240_e60674, assign47240_e60674_d_n5, assign47240_e60674_d_n6, assign47240_e60674_d_n7, assign47240_e60674_d_n8,) = {
    if (((var_guard1244 != 0.0) && (var_guard1247 != 0.0)) && (var_guard1248 == 0.0)) {
        let assign47240_e60650: f64 = (-230.25850929940458);
        let assign47240_e60652: f64 = (assign47240_e60650 - var_temp__blk936);
        let assign47240_e60656: f64 = (-230.25850929940458);
        let assign47240_e60658: f64 = (assign47240_e60656 - var_temp__blk936);
        let assign47240_e60661: f64 = (-230.25850929940458);
        let assign47240_e60663: f64 = (assign47240_e60661 - var_temp__blk936);
        let assign47240_e60665: f64 = (assign47240_e60663 * 0.3333333333333333);
        let assign47240_e60666: f64 = (1.0 + assign47240_e60665);
        let assign47240_e60667: f64 = (assign47240_e60658 * assign47240_e60666);
        let assign47240_e60668: f64 = (0.5 * assign47240_e60667);
        let assign47240_e60669: f64 = (1.0 + assign47240_e60668);
        let assign47240_e60670: f64 = (assign47240_e60652 * assign47240_e60669);
        let assign47240_e60671: f64 = (1.0 + assign47240_e60670);
        let assign47240_e60672: f64 = (1e-100 / assign47240_e60671);
        (assign47240_e60672, (-((1e-100 * (((-var_temp__blk936_dn5) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-var_temp__blk936_dn5) * assign47240_e60666) + (assign47240_e60658 * ((-var_temp__blk936_dn5) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-var_temp__blk936_dn6) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-var_temp__blk936_dn6) * assign47240_e60666) + (assign47240_e60658 * ((-var_temp__blk936_dn6) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-var_temp__blk936_dn7) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-var_temp__blk936_dn7) * assign47240_e60666) + (assign47240_e60658 * ((-var_temp__blk936_dn7) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))), (-((1e-100 * (((-var_temp__blk936_dn8) * assign47240_e60669) + (assign47240_e60652 * (0.5 * (((-var_temp__blk936_dn8) * assign47240_e60666) + (assign47240_e60658 * ((-var_temp__blk936_dn8) * 0.3333333333333333))))))) / (assign47240_e60671 * assign47240_e60671))),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign47240_e60674;
        var_temp2_dn5 = assign47240_e60674_d_n5;
        var_temp2_dn6 = assign47240_e60674_d_n6;
        var_temp2_dn7 = assign47240_e60674_d_n7;
        var_temp2_dn8 = assign47240_e60674_d_n8;
        var_temp2_rv = 0.0;

        var_phit1edge = var_phit;
        var_phit1edge_dn5 = 0.0;
        var_phit1edge_dn6 = 0.0;
        var_phit1edge_dn7 = 0.0;
        var_phit1edge_dn8 = 0.0;
        var_phit1edge_rv = 0.0;

        var_xgedge = 0.0;
        var_xgedge_dn5 = 0.0;
        var_xgedge_dn6 = 0.0;
        var_xgedge_dn7 = 0.0;
        var_xgedge_dn8 = 0.0;
        var_xgedge_rv = 0.0;

        var_qdseffedge = 0.0;
        var_qdseffedge_dn5 = 0.0;
        var_qdseffedge_dn6 = 0.0;
        var_qdseffedge_dn7 = 0.0;
        var_qdseffedge_dn8 = 0.0;
        var_qdseffedge_rv = 0.0;

        var_qmeffedge = 0.0;
        var_qmeffedge_dn5 = 0.0;
        var_qmeffedge_dn6 = 0.0;
        var_qmeffedge_dn7 = 0.0;
        var_qmeffedge_dn8 = 0.0;
        var_qmeffedge_rv = 0.0;

        var_dsqredge = 1e-40;
        var_dsqredge_dn5 = 0.0;
        var_dsqredge_dn6 = 0.0;
        var_dsqredge_dn7 = 0.0;
        var_dsqredge_dn8 = 0.0;
        var_dsqredge_rv = 0.0;

        *var_arg1_slot = var_arg1;
        *var_arg1_dn5_slot = var_arg1_dn5;
        *var_arg1_dn6_slot = var_arg1_dn6;
        *var_arg1_dn7_slot = var_arg1_dn7;
        *var_arg1_dn8_slot = var_arg1_dn8;
        *var_arg1_rv_slot = var_arg1_rv;
        *var_dsqredge_slot = var_dsqredge;
        *var_dsqredge_dn5_slot = var_dsqredge_dn5;
        *var_dsqredge_dn6_slot = var_dsqredge_dn6;
        *var_dsqredge_dn7_slot = var_dsqredge_dn7;
        *var_dsqredge_dn8_slot = var_dsqredge_dn8;
        *var_dsqredge_rv_slot = var_dsqredge_rv;
        *var_ex_slot = var_ex;
        *var_ex_dn5_slot = var_ex_dn5;
        *var_ex_dn6_slot = var_ex_dn6;
        *var_ex_dn7_slot = var_ex_dn7;
        *var_ex_dn8_slot = var_ex_dn8;
        *var_ex_rv_slot = var_ex_rv;
        *var_guard1236_slot = var_guard1236;
        *var_guard1236_rv_slot = var_guard1236_rv;
        *var_guard1237_slot = var_guard1237;
        *var_guard1237_rv_slot = var_guard1237_rv;
        *var_guard1240_slot = var_guard1240;
        *var_guard1240_rv_slot = var_guard1240_rv;
        *var_guard1241_slot = var_guard1241;
        *var_guard1241_rv_slot = var_guard1241_rv;
        *var_guard1242_slot = var_guard1242;
        *var_guard1242_rv_slot = var_guard1242_rv;
        *var_guard1243_slot = var_guard1243;
        *var_guard1243_rv_slot = var_guard1243_rv;
        *var_guard1244_slot = var_guard1244;
        *var_guard1244_rv_slot = var_guard1244_rv;
        *var_guard1245_slot = var_guard1245;
        *var_guard1245_rv_slot = var_guard1245_rv;
        *var_guard1246_slot = var_guard1246;
        *var_guard1246_rv_slot = var_guard1246_rv;
        *var_guard1247_slot = var_guard1247;
        *var_guard1247_rv_slot = var_guard1247_rv;
        *var_guard1248_slot = var_guard1248;
        *var_guard1248_rv_slot = var_guard1248_rv;
        *var_inv_ex_slot = var_inv_ex;
        *var_inv_ex_dn5_slot = var_inv_ex_dn5;
        *var_inv_ex_dn6_slot = var_inv_ex_dn6;
        *var_inv_ex_dn7_slot = var_inv_ex_dn7;
        *var_inv_ex_dn8_slot = var_inv_ex_dn8;
        *var_inv_ex_rv_slot = var_inv_ex_rv;
        *var_phit1edge_slot = var_phit1edge;
        *var_phit1edge_dn5_slot = var_phit1edge_dn5;
        *var_phit1edge_dn6_slot = var_phit1edge_dn6;
        *var_phit1edge_dn7_slot = var_phit1edge_dn7;
        *var_phit1edge_dn8_slot = var_phit1edge_dn8;
        *var_phit1edge_rv_slot = var_phit1edge_rv;
        *var_qdseffedge_slot = var_qdseffedge;
        *var_qdseffedge_dn5_slot = var_qdseffedge_dn5;
        *var_qdseffedge_dn6_slot = var_qdseffedge_dn6;
        *var_qdseffedge_dn7_slot = var_qdseffedge_dn7;
        *var_qdseffedge_dn8_slot = var_qdseffedge_dn8;
        *var_qdseffedge_rv_slot = var_qdseffedge_rv;
        *var_qmeffedge_slot = var_qmeffedge;
        *var_qmeffedge_dn5_slot = var_qmeffedge_dn5;
        *var_qmeffedge_dn6_slot = var_qmeffedge_dn6;
        *var_qmeffedge_dn7_slot = var_qmeffedge_dn7;
        *var_qmeffedge_dn8_slot = var_qmeffedge_dn8;
        *var_qmeffedge_rv_slot = var_qmeffedge_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_u0_slot = var_u0;
        *var_u0_dn5_slot = var_u0_dn5;
        *var_u0_dn6_slot = var_u0_dn6;
        *var_u0_dn7_slot = var_u0_dn7;
        *var_u0_dn8_slot = var_u0_dn8;
        *var_u0_rv_slot = var_u0_rv;
        *var_vtovd_slot = var_vtovd;
        *var_vtovd_dn5_slot = var_vtovd_dn5;
        *var_vtovd_dn6_slot = var_vtovd_dn6;
        *var_vtovd_dn7_slot = var_vtovd_dn7;
        *var_vtovd_dn8_slot = var_vtovd_dn8;
        *var_vtovd_rv_slot = var_vtovd_rv;
        *var_vtovs_slot = var_vtovs;
        *var_vtovs_dn5_slot = var_vtovs_dn5;
        *var_vtovs_dn6_slot = var_vtovs_dn6;
        *var_vtovs_dn7_slot = var_vtovs_dn7;
        *var_vtovs_dn8_slot = var_vtovs_dn8;
        *var_vtovs_rv_slot = var_vtovs_rv;
        *var_x_slot = var_x;
        *var_x_dn5_slot = var_x_dn5;
        *var_x_dn6_slot = var_x_dn6;
        *var_x_dn7_slot = var_x_dn7;
        *var_x_dn8_slot = var_x_dn8;
        *var_x_rv_slot = var_x_rv;
        *var_xgedge_slot = var_xgedge;
        *var_xgedge_dn5_slot = var_xgedge_dn5;
        *var_xgedge_dn6_slot = var_xgedge_dn6;
        *var_xgedge_dn7_slot = var_xgedge_dn7;
        *var_xgedge_dn8_slot = var_xgedge_dn8;
        *var_xgedge_rv_slot = var_xgedge_rv;
    }

    pub(super) fn stamp_reactive_block_40(
        p: &Parameters,
        var_aphiedge: f64,
        var_betnedge_i: f64,
        var_bphiedge: f64,
        var_cfbedge_i: f64,
        var_cfdedge_i: f64,
        var_cfedge_i: f64,
        var_gfedge: f64,
        var_gfedge2: f64,
        var_lngfedge2: f64,
        var_phibedge: f64,
        var_phit0edge: f64,
        var_phix1edge: f64,
        var_phixedge: f64,
        var_pscebedge_i: f64,
        var_pscededge_i: f64,
        var_psceedge_i: f64,
        var_v_db: f64,
        var_v_db_dn6: f64,
        var_v_db_dn7: f64,
        var_v_db_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_v_sb: f64,
        var_v_sb_dn6: f64,
        var_v_sb_dn7: f64,
        var_v_sb_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vfbedge_t: f64,
        var_vgb: f64,
        var_vgb_dn5: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_alphabmedge_slot: &mut f64,
        var_alphabmedge_dn5_slot: &mut f64,
        var_alphabmedge_dn6_slot: &mut f64,
        var_alphabmedge_dn7_slot: &mut f64,
        var_alphabmedge_dn8_slot: &mut f64,
        var_alphabmedge_rv_slot: &mut f64,
        var_delvgedge_slot: &mut f64,
        var_delvgedge_dn5_slot: &mut f64,
        var_delvgedge_dn6_slot: &mut f64,
        var_delvgedge_dn7_slot: &mut f64,
        var_delvgedge_dn8_slot: &mut f64,
        var_delvgedge_rv_slot: &mut f64,
        var_dphit1edge_slot: &mut f64,
        var_dphit1edge_dn5_slot: &mut f64,
        var_dphit1edge_dn6_slot: &mut f64,
        var_dphit1edge_dn7_slot: &mut f64,
        var_dphit1edge_dn8_slot: &mut f64,
        var_dphit1edge_rv_slot: &mut f64,
        var_dxthedge_slot: &mut f64,
        var_dxthedge_dn5_slot: &mut f64,
        var_dxthedge_dn6_slot: &mut f64,
        var_dxthedge_dn7_slot: &mut f64,
        var_dxthedge_dn8_slot: &mut f64,
        var_dxthedge_rv_slot: &mut f64,
        var_guard1249_slot: &mut f64,
        var_guard1249_rv_slot: &mut f64,
        var_guard1250_slot: &mut f64,
        var_guard1250_rv_slot: &mut f64,
        var_guard1251_slot: &mut f64,
        var_guard1251_rv_slot: &mut f64,
        var_i_dsedge_slot: &mut f64,
        var_i_dsedge_dn5_slot: &mut f64,
        var_i_dsedge_dn6_slot: &mut f64,
        var_i_dsedge_dn7_slot: &mut f64,
        var_i_dsedge_dn8_slot: &mut f64,
        var_i_dsedge_rv_slot: &mut f64,
        var_inv_phit1edge_slot: &mut f64,
        var_inv_phit1edge_dn5_slot: &mut f64,
        var_inv_phit1edge_dn6_slot: &mut f64,
        var_inv_phit1edge_dn7_slot: &mut f64,
        var_inv_phit1edge_dn8_slot: &mut f64,
        var_inv_phit1edge_rv_slot: &mut f64,
        var_phit1edge_slot: &mut f64,
        var_phit1edge_dn5_slot: &mut f64,
        var_phit1edge_dn6_slot: &mut f64,
        var_phit1edge_dn7_slot: &mut f64,
        var_phit1edge_dn8_slot: &mut f64,
        var_phit1edge_rv_slot: &mut f64,
        var_q_edge_d0_slot: &mut f64,
        var_q_edge_d0_dn5_slot: &mut f64,
        var_q_edge_d0_dn6_slot: &mut f64,
        var_q_edge_d0_dn7_slot: &mut f64,
        var_q_edge_d0_dn8_slot: &mut f64,
        var_q_edge_d0_rv_slot: &mut f64,
        var_q_edge_d0p_slot: &mut f64,
        var_q_edge_d0p_dn5_slot: &mut f64,
        var_q_edge_d0p_dn6_slot: &mut f64,
        var_q_edge_d0p_dn7_slot: &mut f64,
        var_q_edge_d0p_dn8_slot: &mut f64,
        var_q_edge_d0p_rv_slot: &mut f64,
        var_q_edge_exp_x_slot: &mut f64,
        var_q_edge_exp_x_dn5_slot: &mut f64,
        var_q_edge_exp_x_dn6_slot: &mut f64,
        var_q_edge_exp_x_dn7_slot: &mut f64,
        var_q_edge_exp_x_dn8_slot: &mut f64,
        var_q_edge_exp_x_rv_slot: &mut f64,
        var_q_edge_n_slot: &mut f64,
        var_q_edge_n_dn5_slot: &mut f64,
        var_q_edge_n_dn6_slot: &mut f64,
        var_q_edge_n_dn7_slot: &mut f64,
        var_q_edge_n_dn8_slot: &mut f64,
        var_q_edge_n_inv_slot: &mut f64,
        var_q_edge_n_inv_dn5_slot: &mut f64,
        var_q_edge_n_inv_dn6_slot: &mut f64,
        var_q_edge_n_inv_dn7_slot: &mut f64,
        var_q_edge_n_inv_dn8_slot: &mut f64,
        var_q_edge_n_inv_rv_slot: &mut f64,
        var_q_edge_n_rv_slot: &mut f64,
        var_q_edge_qi0_slot: &mut f64,
        var_q_edge_qi0_dn5_slot: &mut f64,
        var_q_edge_qi0_dn6_slot: &mut f64,
        var_q_edge_qi0_dn7_slot: &mut f64,
        var_q_edge_qi0_dn8_slot: &mut f64,
        var_q_edge_qi0_rv_slot: &mut f64,
        var_q_edge_qi0si_slot: &mut f64,
        var_q_edge_qi0si_dn5_slot: &mut f64,
        var_q_edge_qi0si_dn6_slot: &mut f64,
        var_q_edge_qi0si_dn7_slot: &mut f64,
        var_q_edge_qi0si_dn8_slot: &mut f64,
        var_q_edge_qi0si_rv_slot: &mut f64,
        var_q_edge_sqerr_slot: &mut f64,
        var_q_edge_sqerr_dn5_slot: &mut f64,
        var_q_edge_sqerr_dn6_slot: &mut f64,
        var_q_edge_sqerr_dn7_slot: &mut f64,
        var_q_edge_sqerr_dn8_slot: &mut f64,
        var_q_edge_sqerr_rv_slot: &mut f64,
        var_q_edge_xgt_slot: &mut f64,
        var_q_edge_xgt0_slot: &mut f64,
        var_q_edge_xgt0_dn5_slot: &mut f64,
        var_q_edge_xgt0_dn6_slot: &mut f64,
        var_q_edge_xgt0_dn7_slot: &mut f64,
        var_q_edge_xgt0_dn8_slot: &mut f64,
        var_q_edge_xgt0_rv_slot: &mut f64,
        var_q_edge_xgt0e_slot: &mut f64,
        var_q_edge_xgt0e_dn5_slot: &mut f64,
        var_q_edge_xgt0e_dn6_slot: &mut f64,
        var_q_edge_xgt0e_dn7_slot: &mut f64,
        var_q_edge_xgt0e_dn8_slot: &mut f64,
        var_q_edge_xgt0e_rv_slot: &mut f64,
        var_q_edge_xgt_dn5_slot: &mut f64,
        var_q_edge_xgt_dn6_slot: &mut f64,
        var_q_edge_xgt_dn7_slot: &mut f64,
        var_q_edge_xgt_dn8_slot: &mut f64,
        var_q_edge_xgt_rv_slot: &mut f64,
        var_q_edge_xsth_slot: &mut f64,
        var_q_edge_xsth_dn5_slot: &mut f64,
        var_q_edge_xsth_dn6_slot: &mut f64,
        var_q_edge_xsth_dn7_slot: &mut f64,
        var_q_edge_xsth_dn8_slot: &mut f64,
        var_q_edge_xsth_rv_slot: &mut f64,
        var_q_edge_xth_slot: &mut f64,
        var_q_edge_xth0_slot: &mut f64,
        var_q_edge_xth0_dn5_slot: &mut f64,
        var_q_edge_xth0_dn6_slot: &mut f64,
        var_q_edge_xth0_dn7_slot: &mut f64,
        var_q_edge_xth0_dn8_slot: &mut f64,
        var_q_edge_xth0_rv_slot: &mut f64,
        var_q_edge_xth_dn5_slot: &mut f64,
        var_q_edge_xth_dn6_slot: &mut f64,
        var_q_edge_xth_dn7_slot: &mut f64,
        var_q_edge_xth_dn8_slot: &mut f64,
        var_q_edge_xth_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_vdspedge_slot: &mut f64,
        var_vdspedge_dn6_slot: &mut f64,
        var_vdspedge_dn7_slot: &mut f64,
        var_vdspedge_rv_slot: &mut f64,
        var_vsbstaredge_slot: &mut f64,
        var_vsbstaredge_dn5_slot: &mut f64,
        var_vsbstaredge_dn6_slot: &mut f64,
        var_vsbstaredge_dn7_slot: &mut f64,
        var_vsbstaredge_dn8_slot: &mut f64,
        var_vsbstaredge_rv_slot: &mut f64,
        var_vsbxedge_slot: &mut f64,
        var_vsbxedge_dn5_slot: &mut f64,
        var_vsbxedge_dn6_slot: &mut f64,
        var_vsbxedge_dn7_slot: &mut f64,
        var_vsbxedge_dn8_slot: &mut f64,
        var_vsbxedge_rv_slot: &mut f64,
        var_xbedge_slot: &mut f64,
        var_xbedge_dn5_slot: &mut f64,
        var_xbedge_dn6_slot: &mut f64,
        var_xbedge_dn7_slot: &mut f64,
        var_xbedge_dn8_slot: &mut f64,
        var_xbedge_rv_slot: &mut f64,
        var_xgedge_slot: &mut f64,
        var_xgedge_dn5_slot: &mut f64,
        var_xgedge_dn6_slot: &mut f64,
        var_xgedge_dn7_slot: &mut f64,
        var_xgedge_dn8_slot: &mut f64,
        var_xgedge_rv_slot: &mut f64,
        var_xnedge_s_slot: &mut f64,
        var_xnedge_s_dn5_slot: &mut f64,
        var_xnedge_s_dn6_slot: &mut f64,
        var_xnedge_s_dn7_slot: &mut f64,
        var_xnedge_s_dn8_slot: &mut f64,
        var_xnedge_s_rv_slot: &mut f64,
    ) {
        let mut var_alphabmedge: f64 = *var_alphabmedge_slot;
        let mut var_alphabmedge_dn5: f64 = *var_alphabmedge_dn5_slot;
        let mut var_alphabmedge_dn6: f64 = *var_alphabmedge_dn6_slot;
        let mut var_alphabmedge_dn7: f64 = *var_alphabmedge_dn7_slot;
        let mut var_alphabmedge_dn8: f64 = *var_alphabmedge_dn8_slot;
        let mut var_alphabmedge_rv: f64 = *var_alphabmedge_rv_slot;
        let mut var_delvgedge: f64 = *var_delvgedge_slot;
        let mut var_delvgedge_dn5: f64 = *var_delvgedge_dn5_slot;
        let mut var_delvgedge_dn6: f64 = *var_delvgedge_dn6_slot;
        let mut var_delvgedge_dn7: f64 = *var_delvgedge_dn7_slot;
        let mut var_delvgedge_dn8: f64 = *var_delvgedge_dn8_slot;
        let mut var_delvgedge_rv: f64 = *var_delvgedge_rv_slot;
        let mut var_dphit1edge: f64 = *var_dphit1edge_slot;
        let mut var_dphit1edge_dn5: f64 = *var_dphit1edge_dn5_slot;
        let mut var_dphit1edge_dn6: f64 = *var_dphit1edge_dn6_slot;
        let mut var_dphit1edge_dn7: f64 = *var_dphit1edge_dn7_slot;
        let mut var_dphit1edge_dn8: f64 = *var_dphit1edge_dn8_slot;
        let mut var_dphit1edge_rv: f64 = *var_dphit1edge_rv_slot;
        let mut var_dxthedge: f64 = *var_dxthedge_slot;
        let mut var_dxthedge_dn5: f64 = *var_dxthedge_dn5_slot;
        let mut var_dxthedge_dn6: f64 = *var_dxthedge_dn6_slot;
        let mut var_dxthedge_dn7: f64 = *var_dxthedge_dn7_slot;
        let mut var_dxthedge_dn8: f64 = *var_dxthedge_dn8_slot;
        let mut var_dxthedge_rv: f64 = *var_dxthedge_rv_slot;
        let mut var_guard1249: f64 = *var_guard1249_slot;
        let mut var_guard1249_rv: f64 = *var_guard1249_rv_slot;
        let mut var_guard1250: f64 = *var_guard1250_slot;
        let mut var_guard1250_rv: f64 = *var_guard1250_rv_slot;
        let mut var_guard1251: f64 = *var_guard1251_slot;
        let mut var_guard1251_rv: f64 = *var_guard1251_rv_slot;
        let mut var_i_dsedge: f64 = *var_i_dsedge_slot;
        let mut var_i_dsedge_dn5: f64 = *var_i_dsedge_dn5_slot;
        let mut var_i_dsedge_dn6: f64 = *var_i_dsedge_dn6_slot;
        let mut var_i_dsedge_dn7: f64 = *var_i_dsedge_dn7_slot;
        let mut var_i_dsedge_dn8: f64 = *var_i_dsedge_dn8_slot;
        let mut var_i_dsedge_rv: f64 = *var_i_dsedge_rv_slot;
        let mut var_inv_phit1edge: f64 = *var_inv_phit1edge_slot;
        let mut var_inv_phit1edge_dn5: f64 = *var_inv_phit1edge_dn5_slot;
        let mut var_inv_phit1edge_dn6: f64 = *var_inv_phit1edge_dn6_slot;
        let mut var_inv_phit1edge_dn7: f64 = *var_inv_phit1edge_dn7_slot;
        let mut var_inv_phit1edge_dn8: f64 = *var_inv_phit1edge_dn8_slot;
        let mut var_inv_phit1edge_rv: f64 = *var_inv_phit1edge_rv_slot;
        let mut var_phit1edge: f64 = *var_phit1edge_slot;
        let mut var_phit1edge_dn5: f64 = *var_phit1edge_dn5_slot;
        let mut var_phit1edge_dn6: f64 = *var_phit1edge_dn6_slot;
        let mut var_phit1edge_dn7: f64 = *var_phit1edge_dn7_slot;
        let mut var_phit1edge_dn8: f64 = *var_phit1edge_dn8_slot;
        let mut var_phit1edge_rv: f64 = *var_phit1edge_rv_slot;
        let mut var_q_edge_d0: f64 = *var_q_edge_d0_slot;
        let mut var_q_edge_d0_dn5: f64 = *var_q_edge_d0_dn5_slot;
        let mut var_q_edge_d0_dn6: f64 = *var_q_edge_d0_dn6_slot;
        let mut var_q_edge_d0_dn7: f64 = *var_q_edge_d0_dn7_slot;
        let mut var_q_edge_d0_dn8: f64 = *var_q_edge_d0_dn8_slot;
        let mut var_q_edge_d0_rv: f64 = *var_q_edge_d0_rv_slot;
        let mut var_q_edge_d0p: f64 = *var_q_edge_d0p_slot;
        let mut var_q_edge_d0p_dn5: f64 = *var_q_edge_d0p_dn5_slot;
        let mut var_q_edge_d0p_dn6: f64 = *var_q_edge_d0p_dn6_slot;
        let mut var_q_edge_d0p_dn7: f64 = *var_q_edge_d0p_dn7_slot;
        let mut var_q_edge_d0p_dn8: f64 = *var_q_edge_d0p_dn8_slot;
        let mut var_q_edge_d0p_rv: f64 = *var_q_edge_d0p_rv_slot;
        let mut var_q_edge_exp_x: f64 = *var_q_edge_exp_x_slot;
        let mut var_q_edge_exp_x_dn5: f64 = *var_q_edge_exp_x_dn5_slot;
        let mut var_q_edge_exp_x_dn6: f64 = *var_q_edge_exp_x_dn6_slot;
        let mut var_q_edge_exp_x_dn7: f64 = *var_q_edge_exp_x_dn7_slot;
        let mut var_q_edge_exp_x_dn8: f64 = *var_q_edge_exp_x_dn8_slot;
        let mut var_q_edge_exp_x_rv: f64 = *var_q_edge_exp_x_rv_slot;
        let mut var_q_edge_n: f64 = *var_q_edge_n_slot;
        let mut var_q_edge_n_dn5: f64 = *var_q_edge_n_dn5_slot;
        let mut var_q_edge_n_dn6: f64 = *var_q_edge_n_dn6_slot;
        let mut var_q_edge_n_dn7: f64 = *var_q_edge_n_dn7_slot;
        let mut var_q_edge_n_dn8: f64 = *var_q_edge_n_dn8_slot;
        let mut var_q_edge_n_inv: f64 = *var_q_edge_n_inv_slot;
        let mut var_q_edge_n_inv_dn5: f64 = *var_q_edge_n_inv_dn5_slot;
        let mut var_q_edge_n_inv_dn6: f64 = *var_q_edge_n_inv_dn6_slot;
        let mut var_q_edge_n_inv_dn7: f64 = *var_q_edge_n_inv_dn7_slot;
        let mut var_q_edge_n_inv_dn8: f64 = *var_q_edge_n_inv_dn8_slot;
        let mut var_q_edge_n_inv_rv: f64 = *var_q_edge_n_inv_rv_slot;
        let mut var_q_edge_n_rv: f64 = *var_q_edge_n_rv_slot;
        let mut var_q_edge_qi0: f64 = *var_q_edge_qi0_slot;
        let mut var_q_edge_qi0_dn5: f64 = *var_q_edge_qi0_dn5_slot;
        let mut var_q_edge_qi0_dn6: f64 = *var_q_edge_qi0_dn6_slot;
        let mut var_q_edge_qi0_dn7: f64 = *var_q_edge_qi0_dn7_slot;
        let mut var_q_edge_qi0_dn8: f64 = *var_q_edge_qi0_dn8_slot;
        let mut var_q_edge_qi0_rv: f64 = *var_q_edge_qi0_rv_slot;
        let mut var_q_edge_qi0si: f64 = *var_q_edge_qi0si_slot;
        let mut var_q_edge_qi0si_dn5: f64 = *var_q_edge_qi0si_dn5_slot;
        let mut var_q_edge_qi0si_dn6: f64 = *var_q_edge_qi0si_dn6_slot;
        let mut var_q_edge_qi0si_dn7: f64 = *var_q_edge_qi0si_dn7_slot;
        let mut var_q_edge_qi0si_dn8: f64 = *var_q_edge_qi0si_dn8_slot;
        let mut var_q_edge_qi0si_rv: f64 = *var_q_edge_qi0si_rv_slot;
        let mut var_q_edge_sqerr: f64 = *var_q_edge_sqerr_slot;
        let mut var_q_edge_sqerr_dn5: f64 = *var_q_edge_sqerr_dn5_slot;
        let mut var_q_edge_sqerr_dn6: f64 = *var_q_edge_sqerr_dn6_slot;
        let mut var_q_edge_sqerr_dn7: f64 = *var_q_edge_sqerr_dn7_slot;
        let mut var_q_edge_sqerr_dn8: f64 = *var_q_edge_sqerr_dn8_slot;
        let mut var_q_edge_sqerr_rv: f64 = *var_q_edge_sqerr_rv_slot;
        let mut var_q_edge_xgt: f64 = *var_q_edge_xgt_slot;
        let mut var_q_edge_xgt0: f64 = *var_q_edge_xgt0_slot;
        let mut var_q_edge_xgt0_dn5: f64 = *var_q_edge_xgt0_dn5_slot;
        let mut var_q_edge_xgt0_dn6: f64 = *var_q_edge_xgt0_dn6_slot;
        let mut var_q_edge_xgt0_dn7: f64 = *var_q_edge_xgt0_dn7_slot;
        let mut var_q_edge_xgt0_dn8: f64 = *var_q_edge_xgt0_dn8_slot;
        let mut var_q_edge_xgt0_rv: f64 = *var_q_edge_xgt0_rv_slot;
        let mut var_q_edge_xgt0e: f64 = *var_q_edge_xgt0e_slot;
        let mut var_q_edge_xgt0e_dn5: f64 = *var_q_edge_xgt0e_dn5_slot;
        let mut var_q_edge_xgt0e_dn6: f64 = *var_q_edge_xgt0e_dn6_slot;
        let mut var_q_edge_xgt0e_dn7: f64 = *var_q_edge_xgt0e_dn7_slot;
        let mut var_q_edge_xgt0e_dn8: f64 = *var_q_edge_xgt0e_dn8_slot;
        let mut var_q_edge_xgt0e_rv: f64 = *var_q_edge_xgt0e_rv_slot;
        let mut var_q_edge_xgt_dn5: f64 = *var_q_edge_xgt_dn5_slot;
        let mut var_q_edge_xgt_dn6: f64 = *var_q_edge_xgt_dn6_slot;
        let mut var_q_edge_xgt_dn7: f64 = *var_q_edge_xgt_dn7_slot;
        let mut var_q_edge_xgt_dn8: f64 = *var_q_edge_xgt_dn8_slot;
        let mut var_q_edge_xgt_rv: f64 = *var_q_edge_xgt_rv_slot;
        let mut var_q_edge_xsth: f64 = *var_q_edge_xsth_slot;
        let mut var_q_edge_xsth_dn5: f64 = *var_q_edge_xsth_dn5_slot;
        let mut var_q_edge_xsth_dn6: f64 = *var_q_edge_xsth_dn6_slot;
        let mut var_q_edge_xsth_dn7: f64 = *var_q_edge_xsth_dn7_slot;
        let mut var_q_edge_xsth_dn8: f64 = *var_q_edge_xsth_dn8_slot;
        let mut var_q_edge_xsth_rv: f64 = *var_q_edge_xsth_rv_slot;
        let mut var_q_edge_xth: f64 = *var_q_edge_xth_slot;
        let mut var_q_edge_xth0: f64 = *var_q_edge_xth0_slot;
        let mut var_q_edge_xth0_dn5: f64 = *var_q_edge_xth0_dn5_slot;
        let mut var_q_edge_xth0_dn6: f64 = *var_q_edge_xth0_dn6_slot;
        let mut var_q_edge_xth0_dn7: f64 = *var_q_edge_xth0_dn7_slot;
        let mut var_q_edge_xth0_dn8: f64 = *var_q_edge_xth0_dn8_slot;
        let mut var_q_edge_xth0_rv: f64 = *var_q_edge_xth0_rv_slot;
        let mut var_q_edge_xth_dn5: f64 = *var_q_edge_xth_dn5_slot;
        let mut var_q_edge_xth_dn6: f64 = *var_q_edge_xth_dn6_slot;
        let mut var_q_edge_xth_dn7: f64 = *var_q_edge_xth_dn7_slot;
        let mut var_q_edge_xth_dn8: f64 = *var_q_edge_xth_dn8_slot;
        let mut var_q_edge_xth_rv: f64 = *var_q_edge_xth_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_vdspedge: f64 = *var_vdspedge_slot;
        let mut var_vdspedge_dn6: f64 = *var_vdspedge_dn6_slot;
        let mut var_vdspedge_dn7: f64 = *var_vdspedge_dn7_slot;
        let mut var_vdspedge_rv: f64 = *var_vdspedge_rv_slot;
        let mut var_vsbstaredge: f64 = *var_vsbstaredge_slot;
        let mut var_vsbstaredge_dn5: f64 = *var_vsbstaredge_dn5_slot;
        let mut var_vsbstaredge_dn6: f64 = *var_vsbstaredge_dn6_slot;
        let mut var_vsbstaredge_dn7: f64 = *var_vsbstaredge_dn7_slot;
        let mut var_vsbstaredge_dn8: f64 = *var_vsbstaredge_dn8_slot;
        let mut var_vsbstaredge_rv: f64 = *var_vsbstaredge_rv_slot;
        let mut var_vsbxedge: f64 = *var_vsbxedge_slot;
        let mut var_vsbxedge_dn5: f64 = *var_vsbxedge_dn5_slot;
        let mut var_vsbxedge_dn6: f64 = *var_vsbxedge_dn6_slot;
        let mut var_vsbxedge_dn7: f64 = *var_vsbxedge_dn7_slot;
        let mut var_vsbxedge_dn8: f64 = *var_vsbxedge_dn8_slot;
        let mut var_vsbxedge_rv: f64 = *var_vsbxedge_rv_slot;
        let mut var_xbedge: f64 = *var_xbedge_slot;
        let mut var_xbedge_dn5: f64 = *var_xbedge_dn5_slot;
        let mut var_xbedge_dn6: f64 = *var_xbedge_dn6_slot;
        let mut var_xbedge_dn7: f64 = *var_xbedge_dn7_slot;
        let mut var_xbedge_dn8: f64 = *var_xbedge_dn8_slot;
        let mut var_xbedge_rv: f64 = *var_xbedge_rv_slot;
        let mut var_xgedge: f64 = *var_xgedge_slot;
        let mut var_xgedge_dn5: f64 = *var_xgedge_dn5_slot;
        let mut var_xgedge_dn6: f64 = *var_xgedge_dn6_slot;
        let mut var_xgedge_dn7: f64 = *var_xgedge_dn7_slot;
        let mut var_xgedge_dn8: f64 = *var_xgedge_dn8_slot;
        let mut var_xgedge_rv: f64 = *var_xgedge_rv_slot;
        let mut var_xnedge_s: f64 = *var_xnedge_s_slot;
        let mut var_xnedge_s_dn5: f64 = *var_xnedge_s_dn5_slot;
        let mut var_xnedge_s_dn6: f64 = *var_xnedge_s_dn6_slot;
        let mut var_xnedge_s_dn7: f64 = *var_xnedge_s_dn7_slot;
        let mut var_xnedge_s_dn8: f64 = *var_xnedge_s_dn8_slot;
        let mut var_xnedge_s_rv: f64 = *var_xnedge_s_rv_slot;

        var_alphabmedge = 1.0;
        var_alphabmedge_dn5 = 0.0;
        var_alphabmedge_dn6 = 0.0;
        var_alphabmedge_dn7 = 0.0;
        var_alphabmedge_dn8 = 0.0;
        var_alphabmedge_rv = 0.0;

        var_i_dsedge = 0.0;
        var_i_dsedge_dn5 = 0.0;
        var_i_dsedge_dn6 = 0.0;
        var_i_dsedge_dn7 = 0.0;
        var_i_dsedge_dn8 = 0.0;
        var_i_dsedge_rv = 0.0;

        let assign47330_e60703: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1249 = assign47330_e60703;
        var_guard1249_rv = 0.0;

        let (assign47340_e60724, assign47340_e60724_d_n5, assign47340_e60724_d_n6, assign47340_e60724_d_n7, assign47340_e60724_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47340_e60708: f64 = (var_v_db + var_v_sb);
        let assign47340_e60711: f64 = (var_v_db - var_v_sb);
        let assign47340_e60714: f64 = (var_v_db - var_v_sb);
        let assign47340_e60715: f64 = (assign47340_e60711 * assign47340_e60714);
        let assign47340_e60717: f64 = (assign47340_e60715 + var_bphiedge);
        let assign47340_e60718: f64 = (assign47340_e60717).sqrt();
        let assign47340_e60719: f64 = (assign47340_e60708 - assign47340_e60718);
        let assign47340_e60720: f64 = (0.5 * assign47340_e60719);
        let assign47340_e60722: f64 = (assign47340_e60720 + var_phixedge);
        (assign47340_e60722, 0.0, (0.5 * ((var_v_db_dn6 + var_v_sb_dn6) - ((((var_v_db_dn6 - var_v_sb_dn6) * assign47340_e60714) + (assign47340_e60711 * (var_v_db_dn6 - var_v_sb_dn6))) / (2.0 * assign47340_e60718)))), (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign47340_e60714) + (assign47340_e60711 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign47340_e60718)))), (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign47340_e60714) + (assign47340_e60711 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign47340_e60718)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47340_e60724;
        var_temp__blk936_dn5 = assign47340_e60724_d_n5;
        var_temp__blk936_dn6 = assign47340_e60724_d_n6;
        var_temp__blk936_dn7 = assign47340_e60724_d_n7;
        var_temp__blk936_dn8 = assign47340_e60724_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign47350_e60747, assign47350_e60747_d_n5, assign47350_e60747_d_n6, assign47350_e60747_d_n7, assign47350_e60747_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47350_e60730: f64 = var_temp__blk936;
        let assign47350_e60733: f64 = var_temp__blk936;
        let assign47350_e60736: f64 = var_temp__blk936;
        let assign47350_e60737: f64 = (assign47350_e60733 * assign47350_e60736);
        let assign47350_e60739: f64 = (assign47350_e60737 + var_aphiedge);
        let assign47350_e60740: f64 = (assign47350_e60739).sqrt();
        let assign47350_e60741: f64 = (assign47350_e60730 - assign47350_e60740);
        let assign47350_e60742: f64 = (0.5 * assign47350_e60741);
        let assign47350_e60743: f64 = (var_v_sb - assign47350_e60742);
        let assign47350_e60745: f64 = (assign47350_e60743 + var_phix1edge);
        (assign47350_e60745, (-(0.5 * (var_temp__blk936_dn5 - (((var_temp__blk936_dn5 * assign47350_e60736) + (assign47350_e60733 * var_temp__blk936_dn5)) / (2.0 * assign47350_e60740))))), (var_v_sb_dn6 - (0.5 * (var_temp__blk936_dn6 - (((var_temp__blk936_dn6 * assign47350_e60736) + (assign47350_e60733 * var_temp__blk936_dn6)) / (2.0 * assign47350_e60740))))), (var_v_sb_dn7 - (0.5 * (var_temp__blk936_dn7 - (((var_temp__blk936_dn7 * assign47350_e60736) + (assign47350_e60733 * var_temp__blk936_dn7)) / (2.0 * assign47350_e60740))))), (var_v_sb_dn8 - (0.5 * (var_temp__blk936_dn8 - (((var_temp__blk936_dn8 * assign47350_e60736) + (assign47350_e60733 * var_temp__blk936_dn8)) / (2.0 * assign47350_e60740))))),)
    } else {
        (var_vsbstaredge, var_vsbstaredge_dn5, var_vsbstaredge_dn6, var_vsbstaredge_dn7, var_vsbstaredge_dn8,)
    }
};
        var_vsbstaredge = assign47350_e60747;
        var_vsbstaredge_dn5 = assign47350_e60747_d_n5;
        var_vsbstaredge_dn6 = assign47350_e60747_d_n6;
        var_vsbstaredge_dn7 = assign47350_e60747_d_n7;
        var_vsbstaredge_dn8 = assign47350_e60747_d_n8;
        var_vsbstaredge_rv = 0.0;

        let (assign47360_e60757, assign47360_e60757_d_n5, assign47360_e60757_d_n6, assign47360_e60757_d_n7, assign47360_e60757_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47360_e60753: f64 = (var_v_ds - var_vdsx);
        let assign47360_e60754: f64 = (0.5 * assign47360_e60753);
        let assign47360_e60755: f64 = (var_vsbstaredge + assign47360_e60754);
        (assign47360_e60755, var_vsbstaredge_dn5, (var_vsbstaredge_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vsbstaredge_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vsbstaredge_dn8,)
    } else {
        (var_vsbxedge, var_vsbxedge_dn5, var_vsbxedge_dn6, var_vsbxedge_dn7, var_vsbxedge_dn8,)
    }
};
        var_vsbxedge = assign47360_e60757;
        var_vsbxedge_dn5 = assign47360_e60757_d_n5;
        var_vsbxedge_dn6 = assign47360_e60757_d_n6;
        var_vsbxedge_dn7 = assign47360_e60757_d_n7;
        var_vsbxedge_dn8 = assign47360_e60757_d_n8;
        var_vsbxedge_rv = 0.0;

        let (assign47370_e60773, assign47370_e60773_d_n5, assign47370_e60773_d_n6, assign47370_e60773_d_n7, assign47370_e60773_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47370_e60763: f64 = (var_pscededge_i * var_vdsx);
        let assign47370_e60764: f64 = (1.0 + assign47370_e60763);
        let assign47370_e60765: f64 = (var_psceedge_i * assign47370_e60764);
        let assign47370_e60769: f64 = (var_pscebedge_i * var_vsbxedge);
        let assign47370_e60770: f64 = (1.0 + assign47370_e60769);
        let assign47370_e60771: f64 = (assign47370_e60765 * assign47370_e60770);
        (assign47370_e60771, (assign47370_e60765 * (var_pscebedge_i * var_vsbxedge_dn5)), (((var_psceedge_i * (var_pscededge_i * var_vdsx_dn6)) * assign47370_e60770) + (assign47370_e60765 * (var_pscebedge_i * var_vsbxedge_dn6))), (((var_psceedge_i * (var_pscededge_i * var_vdsx_dn7)) * assign47370_e60770) + (assign47370_e60765 * (var_pscebedge_i * var_vsbxedge_dn7))), (assign47370_e60765 * (var_pscebedge_i * var_vsbxedge_dn8)),)
    } else {
        (var_dphit1edge, var_dphit1edge_dn5, var_dphit1edge_dn6, var_dphit1edge_dn7, var_dphit1edge_dn8,)
    }
};
        var_dphit1edge = assign47370_e60773;
        var_dphit1edge_dn5 = assign47370_e60773_d_n5;
        var_dphit1edge_dn6 = assign47370_e60773_d_n6;
        var_dphit1edge_dn7 = assign47370_e60773_d_n7;
        var_dphit1edge_dn8 = assign47370_e60773_d_n8;
        var_dphit1edge_rv = 0.0;

        let (assign47380_e60781, assign47380_e60781_d_n5, assign47380_e60781_d_n6, assign47380_e60781_d_n7, assign47380_e60781_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47380_e60778: f64 = (1.0 + var_dphit1edge);
        let assign47380_e60779: f64 = (var_phit0edge * assign47380_e60778);
        (assign47380_e60779, (var_phit0edge * var_dphit1edge_dn5), (var_phit0edge * var_dphit1edge_dn6), (var_phit0edge * var_dphit1edge_dn7), (var_phit0edge * var_dphit1edge_dn8),)
    } else {
        (var_phit1edge, var_phit1edge_dn5, var_phit1edge_dn6, var_phit1edge_dn7, var_phit1edge_dn8,)
    }
};
        var_phit1edge = assign47380_e60781;
        var_phit1edge_dn5 = assign47380_e60781_d_n5;
        var_phit1edge_dn6 = assign47380_e60781_d_n6;
        var_phit1edge_dn7 = assign47380_e60781_d_n7;
        var_phit1edge_dn8 = assign47380_e60781_d_n8;
        var_phit1edge_rv = 0.0;

        let (assign47390_e60787, assign47390_e60787_d_n5, assign47390_e60787_d_n6, assign47390_e60787_d_n7, assign47390_e60787_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47390_e60785: f64 = (1.0 / var_phit1edge);
        (assign47390_e60785, (-(var_phit1edge_dn5 / (var_phit1edge * var_phit1edge))), (-(var_phit1edge_dn6 / (var_phit1edge * var_phit1edge))), (-(var_phit1edge_dn7 / (var_phit1edge * var_phit1edge))), (-(var_phit1edge_dn8 / (var_phit1edge * var_phit1edge))),)
    } else {
        (var_inv_phit1edge, var_inv_phit1edge_dn5, var_inv_phit1edge_dn6, var_inv_phit1edge_dn7, var_inv_phit1edge_dn8,)
    }
};
        var_inv_phit1edge = assign47390_e60787;
        var_inv_phit1edge_dn5 = assign47390_e60787_d_n5;
        var_inv_phit1edge_dn6 = assign47390_e60787_d_n6;
        var_inv_phit1edge_dn7 = assign47390_e60787_d_n7;
        var_inv_phit1edge_dn8 = assign47390_e60787_d_n8;
        var_inv_phit1edge_rv = 0.0;

        let (assign47400_e60802, assign47400_e60802_d_n6, assign47400_e60802_d_n7,) = {
    if (var_guard1249 != 0.0) {
        let assign47400_e60791: f64 = (2.0 * var_vdsx);
        let assign47400_e60796: f64 = (var_cfdedge_i * var_vdsx);
        let assign47400_e60797: f64 = (1.0 + assign47400_e60796);
        let assign47400_e60798: f64 = (assign47400_e60797).sqrt();
        let assign47400_e60799: f64 = (1.0 + assign47400_e60798);
        let assign47400_e60800: f64 = (assign47400_e60791 / assign47400_e60799);
        (assign47400_e60800, ((((2.0 * var_vdsx_dn6) * assign47400_e60799) - (assign47400_e60791 * ((var_cfdedge_i * var_vdsx_dn6) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)), ((((2.0 * var_vdsx_dn7) * assign47400_e60799) - (assign47400_e60791 * ((var_cfdedge_i * var_vdsx_dn7) / (2.0 * assign47400_e60798)))) / (assign47400_e60799 * assign47400_e60799)),)
    } else {
        (var_vdspedge, var_vdspedge_dn6, var_vdspedge_dn7,)
    }
};
        var_vdspedge = assign47400_e60802;
        var_vdspedge_dn6 = assign47400_e60802_d_n6;
        var_vdspedge_dn7 = assign47400_e60802_d_n7;
        var_vdspedge_rv = 0.0;

        let (assign47410_e60814, assign47410_e60814_d_n5, assign47410_e60814_d_n6, assign47410_e60814_d_n7, assign47410_e60814_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47410_e60806: f64 = (var_cfedge_i * var_vdspedge);
        let assign47410_e60810: f64 = (var_cfbedge_i * var_vsbxedge);
        let assign47410_e60811: f64 = (1.0 + assign47410_e60810);
        let assign47410_e60812: f64 = (assign47410_e60806 * assign47410_e60811);
        (assign47410_e60812, (assign47410_e60806 * (var_cfbedge_i * var_vsbxedge_dn5)), (((var_cfedge_i * var_vdspedge_dn6) * assign47410_e60811) + (assign47410_e60806 * (var_cfbedge_i * var_vsbxedge_dn6))), (((var_cfedge_i * var_vdspedge_dn7) * assign47410_e60811) + (assign47410_e60806 * (var_cfbedge_i * var_vsbxedge_dn7))), (assign47410_e60806 * (var_cfbedge_i * var_vsbxedge_dn8)),)
    } else {
        (var_delvgedge, var_delvgedge_dn5, var_delvgedge_dn6, var_delvgedge_dn7, var_delvgedge_dn8,)
    }
};
        var_delvgedge = assign47410_e60814;
        var_delvgedge_dn5 = assign47410_e60814_d_n5;
        var_delvgedge_dn6 = assign47410_e60814_d_n6;
        var_delvgedge_dn7 = assign47410_e60814_d_n7;
        var_delvgedge_dn8 = assign47410_e60814_d_n8;
        var_delvgedge_rv = 0.0;

        let (assign47420_e60824, assign47420_e60824_d_n5, assign47420_e60824_d_n6, assign47420_e60824_d_n7, assign47420_e60824_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47420_e60819: f64 = (var_vgb + var_delvgedge);
        let assign47420_e60821: f64 = (assign47420_e60819 - var_vfbedge_t);
        let assign47420_e60822: f64 = (var_inv_phit1edge * assign47420_e60821);
        (assign47420_e60822, ((var_inv_phit1edge_dn5 * assign47420_e60821) + (var_inv_phit1edge * (var_vgb_dn5 + var_delvgedge_dn5))), ((var_inv_phit1edge_dn6 * assign47420_e60821) + (var_inv_phit1edge * (var_vgb_dn6 + var_delvgedge_dn6))), ((var_inv_phit1edge_dn7 * assign47420_e60821) + (var_inv_phit1edge * (var_vgb_dn7 + var_delvgedge_dn7))), ((var_inv_phit1edge_dn8 * assign47420_e60821) + (var_inv_phit1edge * (var_vgb_dn8 + var_delvgedge_dn8))),)
    } else {
        (var_xgedge, var_xgedge_dn5, var_xgedge_dn6, var_xgedge_dn7, var_xgedge_dn8,)
    }
};
        var_xgedge = assign47420_e60824;
        var_xgedge_dn5 = assign47420_e60824_d_n5;
        var_xgedge_dn6 = assign47420_e60824_d_n6;
        var_xgedge_dn7 = assign47420_e60824_d_n7;
        var_xgedge_dn8 = assign47420_e60824_d_n8;
        var_xgedge_rv = 0.0;

        let (assign47430_e60830, assign47430_e60830_d_n5, assign47430_e60830_d_n6, assign47430_e60830_d_n7, assign47430_e60830_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47430_e60828: f64 = (var_inv_phit1edge * var_phibedge);
        (assign47430_e60828, (var_inv_phit1edge_dn5 * var_phibedge), (var_inv_phit1edge_dn6 * var_phibedge), (var_inv_phit1edge_dn7 * var_phibedge), (var_inv_phit1edge_dn8 * var_phibedge),)
    } else {
        (var_xbedge, var_xbedge_dn5, var_xbedge_dn6, var_xbedge_dn7, var_xbedge_dn8,)
    }
};
        var_xbedge = assign47430_e60830;
        var_xbedge_dn5 = assign47430_e60830_d_n5;
        var_xbedge_dn6 = assign47430_e60830_d_n6;
        var_xbedge_dn7 = assign47430_e60830_d_n7;
        var_xbedge_dn8 = assign47430_e60830_d_n8;
        var_xbedge_rv = 0.0;

        let (assign47440_e60842, assign47440_e60842_d_n5, assign47440_e60842_d_n6, assign47440_e60842_d_n7, assign47440_e60842_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47440_e60835: f64 = (var_xbedge / var_gfedge);
        let assign47440_e60837: f64 = (var_xbedge).sqrt();
        let assign47440_e60838: f64 = (assign47440_e60835 + assign47440_e60837);
        let assign47440_e60839: f64 = (assign47440_e60838).ln();
        let assign47440_e60840: f64 = (2.0 * assign47440_e60839);
        (assign47440_e60840, (2.0 * (((var_xbedge_dn5 / var_gfedge) + (var_xbedge_dn5 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((var_xbedge_dn6 / var_gfedge) + (var_xbedge_dn6 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((var_xbedge_dn7 / var_gfedge) + (var_xbedge_dn7 / (2.0 * assign47440_e60837))) / assign47440_e60838)), (2.0 * (((var_xbedge_dn8 / var_gfedge) + (var_xbedge_dn8 / (2.0 * assign47440_e60837))) / assign47440_e60838)),)
    } else {
        (var_dxthedge, var_dxthedge_dn5, var_dxthedge_dn6, var_dxthedge_dn7, var_dxthedge_dn8,)
    }
};
        var_dxthedge = assign47440_e60842;
        var_dxthedge_dn5 = assign47440_e60842_d_n5;
        var_dxthedge_dn6 = assign47440_e60842_d_n6;
        var_dxthedge_dn7 = assign47440_e60842_d_n7;
        var_dxthedge_dn8 = assign47440_e60842_d_n8;
        var_dxthedge_rv = 0.0;

        let (assign47450_e60848, assign47450_e60848_d_n5, assign47450_e60848_d_n6, assign47450_e60848_d_n7, assign47450_e60848_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47450_e60846: f64 = (var_inv_phit1edge * var_vsbstaredge);
        (assign47450_e60846, ((var_inv_phit1edge_dn5 * var_vsbstaredge) + (var_inv_phit1edge * var_vsbstaredge_dn5)), ((var_inv_phit1edge_dn6 * var_vsbstaredge) + (var_inv_phit1edge * var_vsbstaredge_dn6)), ((var_inv_phit1edge_dn7 * var_vsbstaredge) + (var_inv_phit1edge * var_vsbstaredge_dn7)), ((var_inv_phit1edge_dn8 * var_vsbstaredge) + (var_inv_phit1edge * var_vsbstaredge_dn8)),)
    } else {
        (var_xnedge_s, var_xnedge_s_dn5, var_xnedge_s_dn6, var_xnedge_s_dn7, var_xnedge_s_dn8,)
    }
};
        var_xnedge_s = assign47450_e60848;
        var_xnedge_s_dn5 = assign47450_e60848_d_n5;
        var_xnedge_s_dn6 = assign47450_e60848_d_n6;
        var_xnedge_s_dn7 = assign47450_e60848_d_n7;
        var_xnedge_s_dn8 = assign47450_e60848_d_n8;
        var_xnedge_s_rv = 0.0;

        let (assign47460_e60854, assign47460_e60854_d_n5, assign47460_e60854_d_n6, assign47460_e60854_d_n7, assign47460_e60854_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47460_e60852: f64 = (var_xbedge + var_xnedge_s);
        (assign47460_e60852, (var_xbedge_dn5 + var_xnedge_s_dn5), (var_xbedge_dn6 + var_xnedge_s_dn6), (var_xbedge_dn7 + var_xnedge_s_dn7), (var_xbedge_dn8 + var_xnedge_s_dn8),)
    } else {
        (var_q_edge_xsth, var_q_edge_xsth_dn5, var_q_edge_xsth_dn6, var_q_edge_xsth_dn7, var_q_edge_xsth_dn8,)
    }
};
        var_q_edge_xsth = assign47460_e60854;
        var_q_edge_xsth_dn5 = assign47460_e60854_d_n5;
        var_q_edge_xsth_dn6 = assign47460_e60854_d_n6;
        var_q_edge_xsth_dn7 = assign47460_e60854_d_n7;
        var_q_edge_xsth_dn8 = assign47460_e60854_d_n8;
        var_q_edge_xsth_rv = 0.0;

        let (assign47470_e60863, assign47470_e60863_d_n5, assign47470_e60863_d_n6, assign47470_e60863_d_n7, assign47470_e60863_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47470_e60859: f64 = (var_q_edge_xsth).sqrt();
        let assign47470_e60860: f64 = (var_gfedge * assign47470_e60859);
        let assign47470_e60861: f64 = (var_q_edge_xsth + assign47470_e60860);
        (assign47470_e60861, (var_q_edge_xsth_dn5 + (var_gfedge * (var_q_edge_xsth_dn5 / (2.0 * assign47470_e60859)))), (var_q_edge_xsth_dn6 + (var_gfedge * (var_q_edge_xsth_dn6 / (2.0 * assign47470_e60859)))), (var_q_edge_xsth_dn7 + (var_gfedge * (var_q_edge_xsth_dn7 / (2.0 * assign47470_e60859)))), (var_q_edge_xsth_dn8 + (var_gfedge * (var_q_edge_xsth_dn8 / (2.0 * assign47470_e60859)))),)
    } else {
        (var_q_edge_xth0, var_q_edge_xth0_dn5, var_q_edge_xth0_dn6, var_q_edge_xth0_dn7, var_q_edge_xth0_dn8,)
    }
};
        var_q_edge_xth0 = assign47470_e60863;
        var_q_edge_xth0_dn5 = assign47470_e60863_d_n5;
        var_q_edge_xth0_dn6 = assign47470_e60863_d_n6;
        var_q_edge_xth0_dn7 = assign47470_e60863_d_n7;
        var_q_edge_xth0_dn8 = assign47470_e60863_d_n8;
        var_q_edge_xth0_rv = 0.0;

        let (assign47480_e60869, assign47480_e60869_d_n5, assign47480_e60869_d_n6, assign47480_e60869_d_n7, assign47480_e60869_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47480_e60867: f64 = (var_q_edge_xth0 + var_dxthedge);
        (assign47480_e60867, (var_q_edge_xth0_dn5 + var_dxthedge_dn5), (var_q_edge_xth0_dn6 + var_dxthedge_dn6), (var_q_edge_xth0_dn7 + var_dxthedge_dn7), (var_q_edge_xth0_dn8 + var_dxthedge_dn8),)
    } else {
        (var_q_edge_xth, var_q_edge_xth_dn5, var_q_edge_xth_dn6, var_q_edge_xth_dn7, var_q_edge_xth_dn8,)
    }
};
        var_q_edge_xth = assign47480_e60869;
        var_q_edge_xth_dn5 = assign47480_e60869_d_n5;
        var_q_edge_xth_dn6 = assign47480_e60869_d_n6;
        var_q_edge_xth_dn7 = assign47480_e60869_d_n7;
        var_q_edge_xth_dn8 = assign47480_e60869_d_n8;
        var_q_edge_xth_rv = 0.0;

        let (assign47490_e60880, assign47490_e60880_d_n5, assign47490_e60880_d_n6, assign47490_e60880_d_n7, assign47490_e60880_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47490_e60875: f64 = (var_q_edge_xsth).sqrt();
        let assign47490_e60876: f64 = (2.0 * assign47490_e60875);
        let assign47490_e60877: f64 = (var_gfedge / assign47490_e60876);
        let assign47490_e60878: f64 = (1.0 + assign47490_e60877);
        (assign47490_e60878, (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn5 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn6 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn7 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn8 / (2.0 * assign47490_e60875)))) / (assign47490_e60876 * assign47490_e60876))),)
    } else {
        (var_q_edge_n, var_q_edge_n_dn5, var_q_edge_n_dn6, var_q_edge_n_dn7, var_q_edge_n_dn8,)
    }
};
        var_q_edge_n = assign47490_e60880;
        var_q_edge_n_dn5 = assign47490_e60880_d_n5;
        var_q_edge_n_dn6 = assign47490_e60880_d_n6;
        var_q_edge_n_dn7 = assign47490_e60880_d_n7;
        var_q_edge_n_dn8 = assign47490_e60880_d_n8;
        var_q_edge_n_rv = 0.0;

        let (assign47500_e60886, assign47500_e60886_d_n5, assign47500_e60886_d_n6, assign47500_e60886_d_n7, assign47500_e60886_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47500_e60884: f64 = (1.0 / var_q_edge_n);
        (assign47500_e60884, (-(var_q_edge_n_dn5 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn6 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn7 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn8 / (var_q_edge_n * var_q_edge_n))),)
    } else {
        (var_q_edge_n_inv, var_q_edge_n_inv_dn5, var_q_edge_n_inv_dn6, var_q_edge_n_inv_dn7, var_q_edge_n_inv_dn8,)
    }
};
        var_q_edge_n_inv = assign47500_e60886;
        var_q_edge_n_inv_dn5 = assign47500_e60886_d_n5;
        var_q_edge_n_inv_dn6 = assign47500_e60886_d_n6;
        var_q_edge_n_inv_dn7 = assign47500_e60886_d_n7;
        var_q_edge_n_inv_dn8 = assign47500_e60886_d_n8;
        var_q_edge_n_inv_rv = 0.0;

        let (assign47510_e60892, assign47510_e60892_d_n5, assign47510_e60892_d_n6, assign47510_e60892_d_n7, assign47510_e60892_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47510_e60890: f64 = (var_xgedge - var_q_edge_xth);
        (assign47510_e60890, (var_xgedge_dn5 - var_q_edge_xth_dn5), (var_xgedge_dn6 - var_q_edge_xth_dn6), (var_xgedge_dn7 - var_q_edge_xth_dn7), (var_xgedge_dn8 - var_q_edge_xth_dn8),)
    } else {
        (var_q_edge_xgt, var_q_edge_xgt_dn5, var_q_edge_xgt_dn6, var_q_edge_xgt_dn7, var_q_edge_xgt_dn8,)
    }
};
        var_q_edge_xgt = assign47510_e60892;
        var_q_edge_xgt_dn5 = assign47510_e60892_d_n5;
        var_q_edge_xgt_dn6 = assign47510_e60892_d_n6;
        var_q_edge_xgt_dn7 = assign47510_e60892_d_n7;
        var_q_edge_xgt_dn8 = assign47510_e60892_d_n8;
        var_q_edge_xgt_rv = 0.0;

        let assign47520_e60895: f64 = (-12.0);
        let assign47520_e60896: f64 = if var_q_edge_xgt > assign47520_e60895 { 1.0 } else { 0.0 };
        var_guard1250 = assign47520_e60896;
        var_guard1250_rv = 0.0;

        let (assign47530_e60906, assign47530_e60906_d_n5, assign47530_e60906_d_n6, assign47530_e60906_d_n7, assign47530_e60906_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47530_e60902: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47530_e60904: f64 = (assign47530_e60902 - 1.0);
        (assign47530_e60904, var_q_edge_xgt_dn5, var_q_edge_xgt_dn6, var_q_edge_xgt_dn7, var_q_edge_xgt_dn8,)
    } else {
        (var_q_edge_xgt0, var_q_edge_xgt0_dn5, var_q_edge_xgt0_dn6, var_q_edge_xgt0_dn7, var_q_edge_xgt0_dn8,)
    }
};
        var_q_edge_xgt0 = assign47530_e60906;
        var_q_edge_xgt0_dn5 = assign47530_e60906_d_n5;
        var_q_edge_xgt0_dn6 = assign47530_e60906_d_n6;
        var_q_edge_xgt0_dn7 = assign47530_e60906_d_n7;
        var_q_edge_xgt0_dn8 = assign47530_e60906_d_n8;
        var_q_edge_xgt0_rv = 0.0;

        let (assign47540_e60921, assign47540_e60921_d_n5, assign47540_e60921_d_n6, assign47540_e60921_d_n7, assign47540_e60921_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47540_e60914: f64 = (var_q_edge_xgt0 * var_q_edge_xgt0);
        let assign47540_e60916: f64 = (assign47540_e60914 + 10.0);
        let assign47540_e60917: f64 = (assign47540_e60916).sqrt();
        let assign47540_e60918: f64 = (var_q_edge_xgt0 + assign47540_e60917);
        let assign47540_e60919: f64 = (0.5 * assign47540_e60918);
        (assign47540_e60919, (0.5 * (var_q_edge_xgt0_dn5 + (((var_q_edge_xgt0_dn5 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn5)) / (2.0 * assign47540_e60917)))), (0.5 * (var_q_edge_xgt0_dn6 + (((var_q_edge_xgt0_dn6 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn6)) / (2.0 * assign47540_e60917)))), (0.5 * (var_q_edge_xgt0_dn7 + (((var_q_edge_xgt0_dn7 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn7)) / (2.0 * assign47540_e60917)))), (0.5 * (var_q_edge_xgt0_dn8 + (((var_q_edge_xgt0_dn8 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn8)) / (2.0 * assign47540_e60917)))),)
    } else {
        (var_q_edge_xgt0e, var_q_edge_xgt0e_dn5, var_q_edge_xgt0e_dn6, var_q_edge_xgt0e_dn7, var_q_edge_xgt0e_dn8,)
    }
};
        var_q_edge_xgt0e = assign47540_e60921;
        var_q_edge_xgt0e_dn5 = assign47540_e60921_d_n5;
        var_q_edge_xgt0e_dn6 = assign47540_e60921_d_n6;
        var_q_edge_xgt0e_dn7 = assign47540_e60921_d_n7;
        var_q_edge_xgt0e_dn8 = assign47540_e60921_d_n8;
        var_q_edge_xgt0e_rv = 0.0;

        let (assign47550_e60934, assign47550_e60934_d_n5, assign47550_e60934_d_n6, assign47550_e60934_d_n7, assign47550_e60934_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47550_e60928: f64 = (var_q_edge_xgt0e).ln();
        let assign47550_e60929: f64 = (var_q_edge_n * assign47550_e60928);
        let assign47550_e60930: f64 = (var_q_edge_xgt - assign47550_e60929);
        let assign47550_e60932: f64 = (assign47550_e60930 + var_lngfedge2);
        (assign47550_e60932, (var_q_edge_xgt_dn5 - ((var_q_edge_n_dn5 * assign47550_e60928) + (var_q_edge_n * (var_q_edge_xgt0e_dn5 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn6 - ((var_q_edge_n_dn6 * assign47550_e60928) + (var_q_edge_n * (var_q_edge_xgt0e_dn6 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn7 - ((var_q_edge_n_dn7 * assign47550_e60928) + (var_q_edge_n * (var_q_edge_xgt0e_dn7 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn8 - ((var_q_edge_n_dn8 * assign47550_e60928) + (var_q_edge_n * (var_q_edge_xgt0e_dn8 / var_q_edge_xgt0e)))),)
    } else {
        (var_q_edge_qi0si, var_q_edge_qi0si_dn5, var_q_edge_qi0si_dn6, var_q_edge_qi0si_dn7, var_q_edge_qi0si_dn8,)
    }
};
        var_q_edge_qi0si = assign47550_e60934;
        var_q_edge_qi0si_dn5 = assign47550_e60934_d_n5;
        var_q_edge_qi0si_dn6 = assign47550_e60934_d_n6;
        var_q_edge_qi0si_dn7 = assign47550_e60934_d_n7;
        var_q_edge_qi0si_dn8 = assign47550_e60934_d_n8;
        var_q_edge_qi0si_rv = 0.0;

        let (assign47560_e60949, assign47560_e60949_d_n5, assign47560_e60949_d_n6, assign47560_e60949_d_n7, assign47560_e60949_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47560_e60942: f64 = (var_q_edge_qi0si * var_q_edge_qi0si);
        let assign47560_e60944: f64 = (assign47560_e60942 + 2.0);
        let assign47560_e60945: f64 = (assign47560_e60944).sqrt();
        let assign47560_e60946: f64 = (var_q_edge_qi0si + assign47560_e60945);
        let assign47560_e60947: f64 = (0.5 * assign47560_e60946);
        (assign47560_e60947, (0.5 * (var_q_edge_qi0si_dn5 + (((var_q_edge_qi0si_dn5 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn5)) / (2.0 * assign47560_e60945)))), (0.5 * (var_q_edge_qi0si_dn6 + (((var_q_edge_qi0si_dn6 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn6)) / (2.0 * assign47560_e60945)))), (0.5 * (var_q_edge_qi0si_dn7 + (((var_q_edge_qi0si_dn7 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn7)) / (2.0 * assign47560_e60945)))), (0.5 * (var_q_edge_qi0si_dn8 + (((var_q_edge_qi0si_dn8 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn8)) / (2.0 * assign47560_e60945)))),)
    } else {
        (var_q_edge_qi0, var_q_edge_qi0_dn5, var_q_edge_qi0_dn6, var_q_edge_qi0_dn7, var_q_edge_qi0_dn8,)
    }
};
        var_q_edge_qi0 = assign47560_e60949;
        var_q_edge_qi0_dn5 = assign47560_e60949_d_n5;
        var_q_edge_qi0_dn6 = assign47560_e60949_d_n6;
        var_q_edge_qi0_dn7 = assign47560_e60949_d_n7;
        var_q_edge_qi0_dn8 = assign47560_e60949_d_n8;
        var_q_edge_qi0_rv = 0.0;

        let assign47570_e60952: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47570_e60954: f64 = if assign47570_e60952 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1251 = assign47570_e60954;
        var_guard1251_rv = 0.0;

        let (assign47580_e60965, assign47580_e60965_d_n5, assign47580_e60965_d_n6, assign47580_e60965_d_n7, assign47580_e60965_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) && (var_guard1251 != 0.0)) {
        let assign47580_e60962: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47580_e60963: f64 = (assign47580_e60962).exp();
        (assign47580_e60963, (assign47580_e60963 * (var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5)), (assign47580_e60963 * (var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6)), (assign47580_e60963 * (var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7)), (assign47580_e60963 * (var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8)),)
    } else {
        (var_q_edge_exp_x, var_q_edge_exp_x_dn5, var_q_edge_exp_x_dn6, var_q_edge_exp_x_dn7, var_q_edge_exp_x_dn8,)
    }
};
        var_q_edge_exp_x = assign47580_e60965;
        var_q_edge_exp_x_dn5 = assign47580_e60965_d_n5;
        var_q_edge_exp_x_dn6 = assign47580_e60965_d_n6;
        var_q_edge_exp_x_dn7 = assign47580_e60965_d_n7;
        var_q_edge_exp_x_dn8 = assign47580_e60965_d_n8;
        var_q_edge_exp_x_rv = 0.0;

        let (assign47590_e61002, assign47590_e61002_d_n5, assign47590_e61002_d_n6, assign47590_e61002_d_n7, assign47590_e61002_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) && (var_guard1251 == 0.0)) {
        let assign47590_e60976: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47590_e60978: f64 = (assign47590_e60976 - 230.25850929940458);
        let assign47590_e60983: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47590_e60985: f64 = (assign47590_e60983 - 230.25850929940458);
        let assign47590_e60989: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47590_e60991: f64 = (assign47590_e60989 - 230.25850929940458);
        let assign47590_e60993: f64 = (assign47590_e60991 * 0.3333333333333333);
        let assign47590_e60994: f64 = (1.0 + assign47590_e60993);
        let assign47590_e60995: f64 = (assign47590_e60985 * assign47590_e60994);
        let assign47590_e60996: f64 = (0.5 * assign47590_e60995);
        let assign47590_e60997: f64 = (1.0 + assign47590_e60996);
        let assign47590_e60998: f64 = (assign47590_e60978 * assign47590_e60997);
        let assign47590_e60999: f64 = (1.0 + assign47590_e60998);
        let assign47590_e61000: f64 = (1e100 * assign47590_e60999);
        (assign47590_e61000, (1e100 * (((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * assign47590_e60994) + (assign47590_e60985 * ((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * assign47590_e60994) + (assign47590_e60985 * ((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * assign47590_e60994) + (assign47590_e60985 * ((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * assign47590_e60997) + (assign47590_e60978 * (0.5 * (((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * assign47590_e60994) + (assign47590_e60985 * ((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (var_q_edge_exp_x, var_q_edge_exp_x_dn5, var_q_edge_exp_x_dn6, var_q_edge_exp_x_dn7, var_q_edge_exp_x_dn8,)
    }
};
        var_q_edge_exp_x = assign47590_e61002;
        var_q_edge_exp_x_dn5 = assign47590_e61002_d_n5;
        var_q_edge_exp_x_dn6 = assign47590_e61002_d_n6;
        var_q_edge_exp_x_dn7 = assign47590_e61002_d_n7;
        var_q_edge_exp_x_dn8 = assign47590_e61002_d_n8;
        var_q_edge_exp_x_rv = 0.0;

        let (assign47600_e61010, assign47600_e61010_d_n5, assign47600_e61010_d_n6, assign47600_e61010_d_n7, assign47600_e61010_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47600_e61008: f64 = (var_gfedge2 * var_q_edge_exp_x);
        (assign47600_e61008, (var_gfedge2 * var_q_edge_exp_x_dn5), (var_gfedge2 * var_q_edge_exp_x_dn6), (var_gfedge2 * var_q_edge_exp_x_dn7), (var_gfedge2 * var_q_edge_exp_x_dn8),)
    } else {
        (var_q_edge_d0, var_q_edge_d0_dn5, var_q_edge_d0_dn6, var_q_edge_d0_dn7, var_q_edge_d0_dn8,)
    }
};
        var_q_edge_d0 = assign47600_e61010;
        var_q_edge_d0_dn5 = assign47600_e61010_d_n5;
        var_q_edge_d0_dn6 = assign47600_e61010_d_n6;
        var_q_edge_d0_dn7 = assign47600_e61010_d_n7;
        var_q_edge_d0_dn8 = assign47600_e61010_d_n8;
        var_q_edge_d0_rv = 0.0;

        let (assign47610_e61018, assign47610_e61018_d_n5, assign47610_e61018_d_n6, assign47610_e61018_d_n7, assign47610_e61018_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47610_e61016: f64 = (var_q_edge_d0).powf(var_q_edge_n_inv);
        (assign47610_e61016, if var_q_edge_n_inv_dn5 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn5)) } } else { (assign47610_e61016 * ((var_q_edge_n_inv_dn5 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn5 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn6 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn6)) } } else { (assign47610_e61016 * ((var_q_edge_n_inv_dn6 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn6 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn7 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn7)) } } else { (assign47610_e61016 * ((var_q_edge_n_inv_dn7 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn7 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn8 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn8)) } } else { (assign47610_e61016 * ((var_q_edge_n_inv_dn8 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn8 / var_q_edge_d0)))) },)
    } else {
        (var_q_edge_d0p, var_q_edge_d0p_dn5, var_q_edge_d0p_dn6, var_q_edge_d0p_dn7, var_q_edge_d0p_dn8,)
    }
};
        var_q_edge_d0p = assign47610_e61018;
        var_q_edge_d0p_dn5 = assign47610_e61018_d_n5;
        var_q_edge_d0p_dn6 = assign47610_e61018_d_n6;
        var_q_edge_d0p_dn7 = assign47610_e61018_d_n7;
        var_q_edge_d0p_dn8 = assign47610_e61018_d_n8;
        var_q_edge_d0p_rv = 0.0;

        let (assign47620_e61036, assign47620_e61036_d_n5, assign47620_e61036_d_n6, assign47620_e61036_d_n7, assign47620_e61036_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47620_e61024: f64 = (var_q_edge_n * var_q_edge_n);
        let assign47620_e61028: f64 = (var_q_edge_qi0 + var_q_edge_n);
        let assign47620_e61029: f64 = (2.0 * assign47620_e61028);
        let assign47620_e61031: f64 = (assign47620_e61029 - var_q_edge_d0p);
        let assign47620_e61033: f64 = (assign47620_e61031 * var_q_edge_d0p);
        let assign47620_e61034: f64 = (assign47620_e61024 + assign47620_e61033);
        (assign47620_e61034, (((var_q_edge_n_dn5 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn5)) + ((((2.0 * (var_q_edge_qi0_dn5 + var_q_edge_n_dn5)) - var_q_edge_d0p_dn5) * var_q_edge_d0p) + (assign47620_e61031 * var_q_edge_d0p_dn5))), (((var_q_edge_n_dn6 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn6)) + ((((2.0 * (var_q_edge_qi0_dn6 + var_q_edge_n_dn6)) - var_q_edge_d0p_dn6) * var_q_edge_d0p) + (assign47620_e61031 * var_q_edge_d0p_dn6))), (((var_q_edge_n_dn7 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn7)) + ((((2.0 * (var_q_edge_qi0_dn7 + var_q_edge_n_dn7)) - var_q_edge_d0p_dn7) * var_q_edge_d0p) + (assign47620_e61031 * var_q_edge_d0p_dn7))), (((var_q_edge_n_dn8 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn8)) + ((((2.0 * (var_q_edge_qi0_dn8 + var_q_edge_n_dn8)) - var_q_edge_d0p_dn8) * var_q_edge_d0p) + (assign47620_e61031 * var_q_edge_d0p_dn8))),)
    } else {
        (var_q_edge_sqerr, var_q_edge_sqerr_dn5, var_q_edge_sqerr_dn6, var_q_edge_sqerr_dn7, var_q_edge_sqerr_dn8,)
    }
};
        var_q_edge_sqerr = assign47620_e61036;
        var_q_edge_sqerr_dn5 = assign47620_e61036_d_n5;
        var_q_edge_sqerr_dn6 = assign47620_e61036_d_n6;
        var_q_edge_sqerr_dn7 = assign47620_e61036_d_n7;
        var_q_edge_sqerr_dn8 = assign47620_e61036_d_n8;
        var_q_edge_sqerr_rv = 0.0;

        *var_alphabmedge_slot = var_alphabmedge;
        *var_alphabmedge_dn5_slot = var_alphabmedge_dn5;
        *var_alphabmedge_dn6_slot = var_alphabmedge_dn6;
        *var_alphabmedge_dn7_slot = var_alphabmedge_dn7;
        *var_alphabmedge_dn8_slot = var_alphabmedge_dn8;
        *var_alphabmedge_rv_slot = var_alphabmedge_rv;
        *var_delvgedge_slot = var_delvgedge;
        *var_delvgedge_dn5_slot = var_delvgedge_dn5;
        *var_delvgedge_dn6_slot = var_delvgedge_dn6;
        *var_delvgedge_dn7_slot = var_delvgedge_dn7;
        *var_delvgedge_dn8_slot = var_delvgedge_dn8;
        *var_delvgedge_rv_slot = var_delvgedge_rv;
        *var_dphit1edge_slot = var_dphit1edge;
        *var_dphit1edge_dn5_slot = var_dphit1edge_dn5;
        *var_dphit1edge_dn6_slot = var_dphit1edge_dn6;
        *var_dphit1edge_dn7_slot = var_dphit1edge_dn7;
        *var_dphit1edge_dn8_slot = var_dphit1edge_dn8;
        *var_dphit1edge_rv_slot = var_dphit1edge_rv;
        *var_dxthedge_slot = var_dxthedge;
        *var_dxthedge_dn5_slot = var_dxthedge_dn5;
        *var_dxthedge_dn6_slot = var_dxthedge_dn6;
        *var_dxthedge_dn7_slot = var_dxthedge_dn7;
        *var_dxthedge_dn8_slot = var_dxthedge_dn8;
        *var_dxthedge_rv_slot = var_dxthedge_rv;
        *var_guard1249_slot = var_guard1249;
        *var_guard1249_rv_slot = var_guard1249_rv;
        *var_guard1250_slot = var_guard1250;
        *var_guard1250_rv_slot = var_guard1250_rv;
        *var_guard1251_slot = var_guard1251;
        *var_guard1251_rv_slot = var_guard1251_rv;
        *var_i_dsedge_slot = var_i_dsedge;
        *var_i_dsedge_dn5_slot = var_i_dsedge_dn5;
        *var_i_dsedge_dn6_slot = var_i_dsedge_dn6;
        *var_i_dsedge_dn7_slot = var_i_dsedge_dn7;
        *var_i_dsedge_dn8_slot = var_i_dsedge_dn8;
        *var_i_dsedge_rv_slot = var_i_dsedge_rv;
        *var_inv_phit1edge_slot = var_inv_phit1edge;
        *var_inv_phit1edge_dn5_slot = var_inv_phit1edge_dn5;
        *var_inv_phit1edge_dn6_slot = var_inv_phit1edge_dn6;
        *var_inv_phit1edge_dn7_slot = var_inv_phit1edge_dn7;
        *var_inv_phit1edge_dn8_slot = var_inv_phit1edge_dn8;
        *var_inv_phit1edge_rv_slot = var_inv_phit1edge_rv;
        *var_phit1edge_slot = var_phit1edge;
        *var_phit1edge_dn5_slot = var_phit1edge_dn5;
        *var_phit1edge_dn6_slot = var_phit1edge_dn6;
        *var_phit1edge_dn7_slot = var_phit1edge_dn7;
        *var_phit1edge_dn8_slot = var_phit1edge_dn8;
        *var_phit1edge_rv_slot = var_phit1edge_rv;
        *var_q_edge_d0_slot = var_q_edge_d0;
        *var_q_edge_d0_dn5_slot = var_q_edge_d0_dn5;
        *var_q_edge_d0_dn6_slot = var_q_edge_d0_dn6;
        *var_q_edge_d0_dn7_slot = var_q_edge_d0_dn7;
        *var_q_edge_d0_dn8_slot = var_q_edge_d0_dn8;
        *var_q_edge_d0_rv_slot = var_q_edge_d0_rv;
        *var_q_edge_d0p_slot = var_q_edge_d0p;
        *var_q_edge_d0p_dn5_slot = var_q_edge_d0p_dn5;
        *var_q_edge_d0p_dn6_slot = var_q_edge_d0p_dn6;
        *var_q_edge_d0p_dn7_slot = var_q_edge_d0p_dn7;
        *var_q_edge_d0p_dn8_slot = var_q_edge_d0p_dn8;
        *var_q_edge_d0p_rv_slot = var_q_edge_d0p_rv;
        *var_q_edge_exp_x_slot = var_q_edge_exp_x;
        *var_q_edge_exp_x_dn5_slot = var_q_edge_exp_x_dn5;
        *var_q_edge_exp_x_dn6_slot = var_q_edge_exp_x_dn6;
        *var_q_edge_exp_x_dn7_slot = var_q_edge_exp_x_dn7;
        *var_q_edge_exp_x_dn8_slot = var_q_edge_exp_x_dn8;
        *var_q_edge_exp_x_rv_slot = var_q_edge_exp_x_rv;
        *var_q_edge_n_slot = var_q_edge_n;
        *var_q_edge_n_dn5_slot = var_q_edge_n_dn5;
        *var_q_edge_n_dn6_slot = var_q_edge_n_dn6;
        *var_q_edge_n_dn7_slot = var_q_edge_n_dn7;
        *var_q_edge_n_dn8_slot = var_q_edge_n_dn8;
        *var_q_edge_n_inv_slot = var_q_edge_n_inv;
        *var_q_edge_n_inv_dn5_slot = var_q_edge_n_inv_dn5;
        *var_q_edge_n_inv_dn6_slot = var_q_edge_n_inv_dn6;
        *var_q_edge_n_inv_dn7_slot = var_q_edge_n_inv_dn7;
        *var_q_edge_n_inv_dn8_slot = var_q_edge_n_inv_dn8;
        *var_q_edge_n_inv_rv_slot = var_q_edge_n_inv_rv;
        *var_q_edge_n_rv_slot = var_q_edge_n_rv;
        *var_q_edge_qi0_slot = var_q_edge_qi0;
        *var_q_edge_qi0_dn5_slot = var_q_edge_qi0_dn5;
        *var_q_edge_qi0_dn6_slot = var_q_edge_qi0_dn6;
        *var_q_edge_qi0_dn7_slot = var_q_edge_qi0_dn7;
        *var_q_edge_qi0_dn8_slot = var_q_edge_qi0_dn8;
        *var_q_edge_qi0_rv_slot = var_q_edge_qi0_rv;
        *var_q_edge_qi0si_slot = var_q_edge_qi0si;
        *var_q_edge_qi0si_dn5_slot = var_q_edge_qi0si_dn5;
        *var_q_edge_qi0si_dn6_slot = var_q_edge_qi0si_dn6;
        *var_q_edge_qi0si_dn7_slot = var_q_edge_qi0si_dn7;
        *var_q_edge_qi0si_dn8_slot = var_q_edge_qi0si_dn8;
        *var_q_edge_qi0si_rv_slot = var_q_edge_qi0si_rv;
        *var_q_edge_sqerr_slot = var_q_edge_sqerr;
        *var_q_edge_sqerr_dn5_slot = var_q_edge_sqerr_dn5;
        *var_q_edge_sqerr_dn6_slot = var_q_edge_sqerr_dn6;
        *var_q_edge_sqerr_dn7_slot = var_q_edge_sqerr_dn7;
        *var_q_edge_sqerr_dn8_slot = var_q_edge_sqerr_dn8;
        *var_q_edge_sqerr_rv_slot = var_q_edge_sqerr_rv;
        *var_q_edge_xgt_slot = var_q_edge_xgt;
        *var_q_edge_xgt0_slot = var_q_edge_xgt0;
        *var_q_edge_xgt0_dn5_slot = var_q_edge_xgt0_dn5;
        *var_q_edge_xgt0_dn6_slot = var_q_edge_xgt0_dn6;
        *var_q_edge_xgt0_dn7_slot = var_q_edge_xgt0_dn7;
        *var_q_edge_xgt0_dn8_slot = var_q_edge_xgt0_dn8;
        *var_q_edge_xgt0_rv_slot = var_q_edge_xgt0_rv;
        *var_q_edge_xgt0e_slot = var_q_edge_xgt0e;
        *var_q_edge_xgt0e_dn5_slot = var_q_edge_xgt0e_dn5;
        *var_q_edge_xgt0e_dn6_slot = var_q_edge_xgt0e_dn6;
        *var_q_edge_xgt0e_dn7_slot = var_q_edge_xgt0e_dn7;
        *var_q_edge_xgt0e_dn8_slot = var_q_edge_xgt0e_dn8;
        *var_q_edge_xgt0e_rv_slot = var_q_edge_xgt0e_rv;
        *var_q_edge_xgt_dn5_slot = var_q_edge_xgt_dn5;
        *var_q_edge_xgt_dn6_slot = var_q_edge_xgt_dn6;
        *var_q_edge_xgt_dn7_slot = var_q_edge_xgt_dn7;
        *var_q_edge_xgt_dn8_slot = var_q_edge_xgt_dn8;
        *var_q_edge_xgt_rv_slot = var_q_edge_xgt_rv;
        *var_q_edge_xsth_slot = var_q_edge_xsth;
        *var_q_edge_xsth_dn5_slot = var_q_edge_xsth_dn5;
        *var_q_edge_xsth_dn6_slot = var_q_edge_xsth_dn6;
        *var_q_edge_xsth_dn7_slot = var_q_edge_xsth_dn7;
        *var_q_edge_xsth_dn8_slot = var_q_edge_xsth_dn8;
        *var_q_edge_xsth_rv_slot = var_q_edge_xsth_rv;
        *var_q_edge_xth_slot = var_q_edge_xth;
        *var_q_edge_xth0_slot = var_q_edge_xth0;
        *var_q_edge_xth0_dn5_slot = var_q_edge_xth0_dn5;
        *var_q_edge_xth0_dn6_slot = var_q_edge_xth0_dn6;
        *var_q_edge_xth0_dn7_slot = var_q_edge_xth0_dn7;
        *var_q_edge_xth0_dn8_slot = var_q_edge_xth0_dn8;
        *var_q_edge_xth0_rv_slot = var_q_edge_xth0_rv;
        *var_q_edge_xth_dn5_slot = var_q_edge_xth_dn5;
        *var_q_edge_xth_dn6_slot = var_q_edge_xth_dn6;
        *var_q_edge_xth_dn7_slot = var_q_edge_xth_dn7;
        *var_q_edge_xth_dn8_slot = var_q_edge_xth_dn8;
        *var_q_edge_xth_rv_slot = var_q_edge_xth_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_vdspedge_slot = var_vdspedge;
        *var_vdspedge_dn6_slot = var_vdspedge_dn6;
        *var_vdspedge_dn7_slot = var_vdspedge_dn7;
        *var_vdspedge_rv_slot = var_vdspedge_rv;
        *var_vsbstaredge_slot = var_vsbstaredge;
        *var_vsbstaredge_dn5_slot = var_vsbstaredge_dn5;
        *var_vsbstaredge_dn6_slot = var_vsbstaredge_dn6;
        *var_vsbstaredge_dn7_slot = var_vsbstaredge_dn7;
        *var_vsbstaredge_dn8_slot = var_vsbstaredge_dn8;
        *var_vsbstaredge_rv_slot = var_vsbstaredge_rv;
        *var_vsbxedge_slot = var_vsbxedge;
        *var_vsbxedge_dn5_slot = var_vsbxedge_dn5;
        *var_vsbxedge_dn6_slot = var_vsbxedge_dn6;
        *var_vsbxedge_dn7_slot = var_vsbxedge_dn7;
        *var_vsbxedge_dn8_slot = var_vsbxedge_dn8;
        *var_vsbxedge_rv_slot = var_vsbxedge_rv;
        *var_xbedge_slot = var_xbedge;
        *var_xbedge_dn5_slot = var_xbedge_dn5;
        *var_xbedge_dn6_slot = var_xbedge_dn6;
        *var_xbedge_dn7_slot = var_xbedge_dn7;
        *var_xbedge_dn8_slot = var_xbedge_dn8;
        *var_xbedge_rv_slot = var_xbedge_rv;
        *var_xgedge_slot = var_xgedge;
        *var_xgedge_dn5_slot = var_xgedge_dn5;
        *var_xgedge_dn6_slot = var_xgedge_dn6;
        *var_xgedge_dn7_slot = var_xgedge_dn7;
        *var_xgedge_dn8_slot = var_xgedge_dn8;
        *var_xgedge_rv_slot = var_xgedge_rv;
        *var_xnedge_s_slot = var_xnedge_s;
        *var_xnedge_s_dn5_slot = var_xnedge_s_dn5;
        *var_xnedge_s_dn6_slot = var_xnedge_s_dn6;
        *var_xnedge_s_dn7_slot = var_xnedge_s_dn7;
        *var_xnedge_s_dn8_slot = var_xnedge_s_dn8;
        *var_xnedge_s_rv_slot = var_xnedge_s_rv;
    }

    pub(super) fn stamp_reactive_block_41(
        var_dxthedge: f64,
        var_dxthedge_dn5: f64,
        var_dxthedge_dn6: f64,
        var_dxthedge_dn7: f64,
        var_dxthedge_dn8: f64,
        var_gfedge: f64,
        var_gfedge2: f64,
        var_guard1249: f64,
        var_guard1250: f64,
        var_inv_phit1edge: f64,
        var_inv_phit1edge_dn5: f64,
        var_inv_phit1edge_dn6: f64,
        var_inv_phit1edge_dn7: f64,
        var_inv_phit1edge_dn8: f64,
        var_lngfedge2: f64,
        var_vdse_dc: f64,
        var_vdse_dc_dn5: f64,
        var_vdse_dc_dn6: f64,
        var_vdse_dc_dn7: f64,
        var_vdse_dc_dn8: f64,
        var_vsbstaredge: f64,
        var_vsbstaredge_dn5: f64,
        var_vsbstaredge_dn6: f64,
        var_vsbstaredge_dn7: f64,
        var_vsbstaredge_dn8: f64,
        var_xbedge: f64,
        var_xbedge_dn5: f64,
        var_xbedge_dn6: f64,
        var_xbedge_dn7: f64,
        var_xbedge_dn8: f64,
        var_xgedge: f64,
        var_xgedge_dn5: f64,
        var_xgedge_dn6: f64,
        var_xgedge_dn7: f64,
        var_xgedge_dn8: f64,
        var_xnedge_s: f64,
        var_xnedge_s_dn5: f64,
        var_xnedge_s_dn6: f64,
        var_xnedge_s_dn7: f64,
        var_xnedge_s_dn8: f64,
        var_guard1252_slot: &mut f64,
        var_guard1252_rv_slot: &mut f64,
        var_guard1253_slot: &mut f64,
        var_guard1253_rv_slot: &mut f64,
        var_guard1254_slot: &mut f64,
        var_guard1254_rv_slot: &mut f64,
        var_guard1255_slot: &mut f64,
        var_guard1255_rv_slot: &mut f64,
        var_guard1256_slot: &mut f64,
        var_guard1256_rv_slot: &mut f64,
        var_q_edge_d0_slot: &mut f64,
        var_q_edge_d0_dn5_slot: &mut f64,
        var_q_edge_d0_dn6_slot: &mut f64,
        var_q_edge_d0_dn7_slot: &mut f64,
        var_q_edge_d0_dn8_slot: &mut f64,
        var_q_edge_d0_rv_slot: &mut f64,
        var_q_edge_d0p_slot: &mut f64,
        var_q_edge_d0p_dn5_slot: &mut f64,
        var_q_edge_d0p_dn6_slot: &mut f64,
        var_q_edge_d0p_dn7_slot: &mut f64,
        var_q_edge_d0p_dn8_slot: &mut f64,
        var_q_edge_d0p_rv_slot: &mut f64,
        var_q_edge_errq_slot: &mut f64,
        var_q_edge_errq_dn5_slot: &mut f64,
        var_q_edge_errq_dn6_slot: &mut f64,
        var_q_edge_errq_dn7_slot: &mut f64,
        var_q_edge_errq_dn8_slot: &mut f64,
        var_q_edge_errq_rv_slot: &mut f64,
        var_q_edge_exp_x_slot: &mut f64,
        var_q_edge_exp_x_dn5_slot: &mut f64,
        var_q_edge_exp_x_dn6_slot: &mut f64,
        var_q_edge_exp_x_dn7_slot: &mut f64,
        var_q_edge_exp_x_dn8_slot: &mut f64,
        var_q_edge_exp_x_rv_slot: &mut f64,
        var_q_edge_n_slot: &mut f64,
        var_q_edge_n_dn5_slot: &mut f64,
        var_q_edge_n_dn6_slot: &mut f64,
        var_q_edge_n_dn7_slot: &mut f64,
        var_q_edge_n_dn8_slot: &mut f64,
        var_q_edge_n_inv_slot: &mut f64,
        var_q_edge_n_inv_dn5_slot: &mut f64,
        var_q_edge_n_inv_dn6_slot: &mut f64,
        var_q_edge_n_inv_dn7_slot: &mut f64,
        var_q_edge_n_inv_dn8_slot: &mut f64,
        var_q_edge_n_inv_rv_slot: &mut f64,
        var_q_edge_n_rv_slot: &mut f64,
        var_q_edge_qi0_slot: &mut f64,
        var_q_edge_qi0_dn5_slot: &mut f64,
        var_q_edge_qi0_dn6_slot: &mut f64,
        var_q_edge_qi0_dn7_slot: &mut f64,
        var_q_edge_qi0_dn8_slot: &mut f64,
        var_q_edge_qi0_rv_slot: &mut f64,
        var_q_edge_qi0si_slot: &mut f64,
        var_q_edge_qi0si_dn5_slot: &mut f64,
        var_q_edge_qi0si_dn6_slot: &mut f64,
        var_q_edge_qi0si_dn7_slot: &mut f64,
        var_q_edge_qi0si_dn8_slot: &mut f64,
        var_q_edge_qi0si_rv_slot: &mut f64,
        var_q_edge_sqerr_slot: &mut f64,
        var_q_edge_sqerr_dn5_slot: &mut f64,
        var_q_edge_sqerr_dn6_slot: &mut f64,
        var_q_edge_sqerr_dn7_slot: &mut f64,
        var_q_edge_sqerr_dn8_slot: &mut f64,
        var_q_edge_sqerr_rv_slot: &mut f64,
        var_q_edge_xgt_slot: &mut f64,
        var_q_edge_xgt0_slot: &mut f64,
        var_q_edge_xgt0_dn5_slot: &mut f64,
        var_q_edge_xgt0_dn6_slot: &mut f64,
        var_q_edge_xgt0_dn7_slot: &mut f64,
        var_q_edge_xgt0_dn8_slot: &mut f64,
        var_q_edge_xgt0_rv_slot: &mut f64,
        var_q_edge_xgt0e_slot: &mut f64,
        var_q_edge_xgt0e_dn5_slot: &mut f64,
        var_q_edge_xgt0e_dn6_slot: &mut f64,
        var_q_edge_xgt0e_dn7_slot: &mut f64,
        var_q_edge_xgt0e_dn8_slot: &mut f64,
        var_q_edge_xgt0e_rv_slot: &mut f64,
        var_q_edge_xgt_dn5_slot: &mut f64,
        var_q_edge_xgt_dn6_slot: &mut f64,
        var_q_edge_xgt_dn7_slot: &mut f64,
        var_q_edge_xgt_dn8_slot: &mut f64,
        var_q_edge_xgt_rv_slot: &mut f64,
        var_q_edge_xsth_slot: &mut f64,
        var_q_edge_xsth_dn5_slot: &mut f64,
        var_q_edge_xsth_dn6_slot: &mut f64,
        var_q_edge_xsth_dn7_slot: &mut f64,
        var_q_edge_xsth_dn8_slot: &mut f64,
        var_q_edge_xsth_rv_slot: &mut f64,
        var_q_edge_xth_slot: &mut f64,
        var_q_edge_xth0_slot: &mut f64,
        var_q_edge_xth0_dn5_slot: &mut f64,
        var_q_edge_xth0_dn6_slot: &mut f64,
        var_q_edge_xth0_dn7_slot: &mut f64,
        var_q_edge_xth0_dn8_slot: &mut f64,
        var_q_edge_xth0_rv_slot: &mut f64,
        var_q_edge_xth_dn5_slot: &mut f64,
        var_q_edge_xth_dn6_slot: &mut f64,
        var_q_edge_xth_dn7_slot: &mut f64,
        var_q_edge_xth_dn8_slot: &mut f64,
        var_q_edge_xth_rv_slot: &mut f64,
        var_qdeffedge_slot: &mut f64,
        var_qdeffedge_dn5_slot: &mut f64,
        var_qdeffedge_dn6_slot: &mut f64,
        var_qdeffedge_dn7_slot: &mut f64,
        var_qdeffedge_dn8_slot: &mut f64,
        var_qdeffedge_rv_slot: &mut f64,
        var_qdseffedge_slot: &mut f64,
        var_qdseffedge_dn5_slot: &mut f64,
        var_qdseffedge_dn6_slot: &mut f64,
        var_qdseffedge_dn7_slot: &mut f64,
        var_qdseffedge_dn8_slot: &mut f64,
        var_qdseffedge_rv_slot: &mut f64,
        var_qseffedge_slot: &mut f64,
        var_qseffedge_dn5_slot: &mut f64,
        var_qseffedge_dn6_slot: &mut f64,
        var_qseffedge_dn7_slot: &mut f64,
        var_qseffedge_dn8_slot: &mut f64,
        var_qseffedge_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_xnedge_d_slot: &mut f64,
        var_xnedge_d_dn5_slot: &mut f64,
        var_xnedge_d_dn6_slot: &mut f64,
        var_xnedge_d_dn7_slot: &mut f64,
        var_xnedge_d_dn8_slot: &mut f64,
        var_xnedge_d_rv_slot: &mut f64,
    ) {
        let mut var_guard1252: f64 = *var_guard1252_slot;
        let mut var_guard1252_rv: f64 = *var_guard1252_rv_slot;
        let mut var_guard1253: f64 = *var_guard1253_slot;
        let mut var_guard1253_rv: f64 = *var_guard1253_rv_slot;
        let mut var_guard1254: f64 = *var_guard1254_slot;
        let mut var_guard1254_rv: f64 = *var_guard1254_rv_slot;
        let mut var_guard1255: f64 = *var_guard1255_slot;
        let mut var_guard1255_rv: f64 = *var_guard1255_rv_slot;
        let mut var_guard1256: f64 = *var_guard1256_slot;
        let mut var_guard1256_rv: f64 = *var_guard1256_rv_slot;
        let mut var_q_edge_d0: f64 = *var_q_edge_d0_slot;
        let mut var_q_edge_d0_dn5: f64 = *var_q_edge_d0_dn5_slot;
        let mut var_q_edge_d0_dn6: f64 = *var_q_edge_d0_dn6_slot;
        let mut var_q_edge_d0_dn7: f64 = *var_q_edge_d0_dn7_slot;
        let mut var_q_edge_d0_dn8: f64 = *var_q_edge_d0_dn8_slot;
        let mut var_q_edge_d0_rv: f64 = *var_q_edge_d0_rv_slot;
        let mut var_q_edge_d0p: f64 = *var_q_edge_d0p_slot;
        let mut var_q_edge_d0p_dn5: f64 = *var_q_edge_d0p_dn5_slot;
        let mut var_q_edge_d0p_dn6: f64 = *var_q_edge_d0p_dn6_slot;
        let mut var_q_edge_d0p_dn7: f64 = *var_q_edge_d0p_dn7_slot;
        let mut var_q_edge_d0p_dn8: f64 = *var_q_edge_d0p_dn8_slot;
        let mut var_q_edge_d0p_rv: f64 = *var_q_edge_d0p_rv_slot;
        let mut var_q_edge_errq: f64 = *var_q_edge_errq_slot;
        let mut var_q_edge_errq_dn5: f64 = *var_q_edge_errq_dn5_slot;
        let mut var_q_edge_errq_dn6: f64 = *var_q_edge_errq_dn6_slot;
        let mut var_q_edge_errq_dn7: f64 = *var_q_edge_errq_dn7_slot;
        let mut var_q_edge_errq_dn8: f64 = *var_q_edge_errq_dn8_slot;
        let mut var_q_edge_errq_rv: f64 = *var_q_edge_errq_rv_slot;
        let mut var_q_edge_exp_x: f64 = *var_q_edge_exp_x_slot;
        let mut var_q_edge_exp_x_dn5: f64 = *var_q_edge_exp_x_dn5_slot;
        let mut var_q_edge_exp_x_dn6: f64 = *var_q_edge_exp_x_dn6_slot;
        let mut var_q_edge_exp_x_dn7: f64 = *var_q_edge_exp_x_dn7_slot;
        let mut var_q_edge_exp_x_dn8: f64 = *var_q_edge_exp_x_dn8_slot;
        let mut var_q_edge_exp_x_rv: f64 = *var_q_edge_exp_x_rv_slot;
        let mut var_q_edge_n: f64 = *var_q_edge_n_slot;
        let mut var_q_edge_n_dn5: f64 = *var_q_edge_n_dn5_slot;
        let mut var_q_edge_n_dn6: f64 = *var_q_edge_n_dn6_slot;
        let mut var_q_edge_n_dn7: f64 = *var_q_edge_n_dn7_slot;
        let mut var_q_edge_n_dn8: f64 = *var_q_edge_n_dn8_slot;
        let mut var_q_edge_n_inv: f64 = *var_q_edge_n_inv_slot;
        let mut var_q_edge_n_inv_dn5: f64 = *var_q_edge_n_inv_dn5_slot;
        let mut var_q_edge_n_inv_dn6: f64 = *var_q_edge_n_inv_dn6_slot;
        let mut var_q_edge_n_inv_dn7: f64 = *var_q_edge_n_inv_dn7_slot;
        let mut var_q_edge_n_inv_dn8: f64 = *var_q_edge_n_inv_dn8_slot;
        let mut var_q_edge_n_inv_rv: f64 = *var_q_edge_n_inv_rv_slot;
        let mut var_q_edge_n_rv: f64 = *var_q_edge_n_rv_slot;
        let mut var_q_edge_qi0: f64 = *var_q_edge_qi0_slot;
        let mut var_q_edge_qi0_dn5: f64 = *var_q_edge_qi0_dn5_slot;
        let mut var_q_edge_qi0_dn6: f64 = *var_q_edge_qi0_dn6_slot;
        let mut var_q_edge_qi0_dn7: f64 = *var_q_edge_qi0_dn7_slot;
        let mut var_q_edge_qi0_dn8: f64 = *var_q_edge_qi0_dn8_slot;
        let mut var_q_edge_qi0_rv: f64 = *var_q_edge_qi0_rv_slot;
        let mut var_q_edge_qi0si: f64 = *var_q_edge_qi0si_slot;
        let mut var_q_edge_qi0si_dn5: f64 = *var_q_edge_qi0si_dn5_slot;
        let mut var_q_edge_qi0si_dn6: f64 = *var_q_edge_qi0si_dn6_slot;
        let mut var_q_edge_qi0si_dn7: f64 = *var_q_edge_qi0si_dn7_slot;
        let mut var_q_edge_qi0si_dn8: f64 = *var_q_edge_qi0si_dn8_slot;
        let mut var_q_edge_qi0si_rv: f64 = *var_q_edge_qi0si_rv_slot;
        let mut var_q_edge_sqerr: f64 = *var_q_edge_sqerr_slot;
        let mut var_q_edge_sqerr_dn5: f64 = *var_q_edge_sqerr_dn5_slot;
        let mut var_q_edge_sqerr_dn6: f64 = *var_q_edge_sqerr_dn6_slot;
        let mut var_q_edge_sqerr_dn7: f64 = *var_q_edge_sqerr_dn7_slot;
        let mut var_q_edge_sqerr_dn8: f64 = *var_q_edge_sqerr_dn8_slot;
        let mut var_q_edge_sqerr_rv: f64 = *var_q_edge_sqerr_rv_slot;
        let mut var_q_edge_xgt: f64 = *var_q_edge_xgt_slot;
        let mut var_q_edge_xgt0: f64 = *var_q_edge_xgt0_slot;
        let mut var_q_edge_xgt0_dn5: f64 = *var_q_edge_xgt0_dn5_slot;
        let mut var_q_edge_xgt0_dn6: f64 = *var_q_edge_xgt0_dn6_slot;
        let mut var_q_edge_xgt0_dn7: f64 = *var_q_edge_xgt0_dn7_slot;
        let mut var_q_edge_xgt0_dn8: f64 = *var_q_edge_xgt0_dn8_slot;
        let mut var_q_edge_xgt0_rv: f64 = *var_q_edge_xgt0_rv_slot;
        let mut var_q_edge_xgt0e: f64 = *var_q_edge_xgt0e_slot;
        let mut var_q_edge_xgt0e_dn5: f64 = *var_q_edge_xgt0e_dn5_slot;
        let mut var_q_edge_xgt0e_dn6: f64 = *var_q_edge_xgt0e_dn6_slot;
        let mut var_q_edge_xgt0e_dn7: f64 = *var_q_edge_xgt0e_dn7_slot;
        let mut var_q_edge_xgt0e_dn8: f64 = *var_q_edge_xgt0e_dn8_slot;
        let mut var_q_edge_xgt0e_rv: f64 = *var_q_edge_xgt0e_rv_slot;
        let mut var_q_edge_xgt_dn5: f64 = *var_q_edge_xgt_dn5_slot;
        let mut var_q_edge_xgt_dn6: f64 = *var_q_edge_xgt_dn6_slot;
        let mut var_q_edge_xgt_dn7: f64 = *var_q_edge_xgt_dn7_slot;
        let mut var_q_edge_xgt_dn8: f64 = *var_q_edge_xgt_dn8_slot;
        let mut var_q_edge_xgt_rv: f64 = *var_q_edge_xgt_rv_slot;
        let mut var_q_edge_xsth: f64 = *var_q_edge_xsth_slot;
        let mut var_q_edge_xsth_dn5: f64 = *var_q_edge_xsth_dn5_slot;
        let mut var_q_edge_xsth_dn6: f64 = *var_q_edge_xsth_dn6_slot;
        let mut var_q_edge_xsth_dn7: f64 = *var_q_edge_xsth_dn7_slot;
        let mut var_q_edge_xsth_dn8: f64 = *var_q_edge_xsth_dn8_slot;
        let mut var_q_edge_xsth_rv: f64 = *var_q_edge_xsth_rv_slot;
        let mut var_q_edge_xth: f64 = *var_q_edge_xth_slot;
        let mut var_q_edge_xth0: f64 = *var_q_edge_xth0_slot;
        let mut var_q_edge_xth0_dn5: f64 = *var_q_edge_xth0_dn5_slot;
        let mut var_q_edge_xth0_dn6: f64 = *var_q_edge_xth0_dn6_slot;
        let mut var_q_edge_xth0_dn7: f64 = *var_q_edge_xth0_dn7_slot;
        let mut var_q_edge_xth0_dn8: f64 = *var_q_edge_xth0_dn8_slot;
        let mut var_q_edge_xth0_rv: f64 = *var_q_edge_xth0_rv_slot;
        let mut var_q_edge_xth_dn5: f64 = *var_q_edge_xth_dn5_slot;
        let mut var_q_edge_xth_dn6: f64 = *var_q_edge_xth_dn6_slot;
        let mut var_q_edge_xth_dn7: f64 = *var_q_edge_xth_dn7_slot;
        let mut var_q_edge_xth_dn8: f64 = *var_q_edge_xth_dn8_slot;
        let mut var_q_edge_xth_rv: f64 = *var_q_edge_xth_rv_slot;
        let mut var_qdeffedge: f64 = *var_qdeffedge_slot;
        let mut var_qdeffedge_dn5: f64 = *var_qdeffedge_dn5_slot;
        let mut var_qdeffedge_dn6: f64 = *var_qdeffedge_dn6_slot;
        let mut var_qdeffedge_dn7: f64 = *var_qdeffedge_dn7_slot;
        let mut var_qdeffedge_dn8: f64 = *var_qdeffedge_dn8_slot;
        let mut var_qdeffedge_rv: f64 = *var_qdeffedge_rv_slot;
        let mut var_qdseffedge: f64 = *var_qdseffedge_slot;
        let mut var_qdseffedge_dn5: f64 = *var_qdseffedge_dn5_slot;
        let mut var_qdseffedge_dn6: f64 = *var_qdseffedge_dn6_slot;
        let mut var_qdseffedge_dn7: f64 = *var_qdseffedge_dn7_slot;
        let mut var_qdseffedge_dn8: f64 = *var_qdseffedge_dn8_slot;
        let mut var_qdseffedge_rv: f64 = *var_qdseffedge_rv_slot;
        let mut var_qseffedge: f64 = *var_qseffedge_slot;
        let mut var_qseffedge_dn5: f64 = *var_qseffedge_dn5_slot;
        let mut var_qseffedge_dn6: f64 = *var_qseffedge_dn6_slot;
        let mut var_qseffedge_dn7: f64 = *var_qseffedge_dn7_slot;
        let mut var_qseffedge_dn8: f64 = *var_qseffedge_dn8_slot;
        let mut var_qseffedge_rv: f64 = *var_qseffedge_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_xnedge_d: f64 = *var_xnedge_d_slot;
        let mut var_xnedge_d_dn5: f64 = *var_xnedge_d_dn5_slot;
        let mut var_xnedge_d_dn6: f64 = *var_xnedge_d_dn6_slot;
        let mut var_xnedge_d_dn7: f64 = *var_xnedge_d_dn7_slot;
        let mut var_xnedge_d_dn8: f64 = *var_xnedge_d_dn8_slot;
        let mut var_xnedge_d_rv: f64 = *var_xnedge_d_rv_slot;

        let (assign47630_e61051, assign47630_e61051_d_n5, assign47630_e61051_d_n6, assign47630_e61051_d_n7, assign47630_e61051_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47630_e61042: f64 = (var_q_edge_sqerr).sqrt();
        let assign47630_e61044: f64 = (assign47630_e61042 - var_q_edge_n);
        let assign47630_e61046: f64 = (assign47630_e61044 / var_q_edge_d0p);
        let assign47630_e61048: f64 = (assign47630_e61046 - 1.0);
        let assign47630_e61049: f64 = (var_q_edge_n * assign47630_e61048);
        (assign47630_e61049, ((var_q_edge_n_dn5 * assign47630_e61048) + (var_q_edge_n * (((((var_q_edge_sqerr_dn5 / (2.0 * assign47630_e61042)) - var_q_edge_n_dn5) * var_q_edge_d0p) - (assign47630_e61044 * var_q_edge_d0p_dn5)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn6 * assign47630_e61048) + (var_q_edge_n * (((((var_q_edge_sqerr_dn6 / (2.0 * assign47630_e61042)) - var_q_edge_n_dn6) * var_q_edge_d0p) - (assign47630_e61044 * var_q_edge_d0p_dn6)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn7 * assign47630_e61048) + (var_q_edge_n * (((((var_q_edge_sqerr_dn7 / (2.0 * assign47630_e61042)) - var_q_edge_n_dn7) * var_q_edge_d0p) - (assign47630_e61044 * var_q_edge_d0p_dn7)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn8 * assign47630_e61048) + (var_q_edge_n * (((((var_q_edge_sqerr_dn8 / (2.0 * assign47630_e61042)) - var_q_edge_n_dn8) * var_q_edge_d0p) - (assign47630_e61044 * var_q_edge_d0p_dn8)) / (var_q_edge_d0p * var_q_edge_d0p)))),)
    } else {
        (var_q_edge_errq, var_q_edge_errq_dn5, var_q_edge_errq_dn6, var_q_edge_errq_dn7, var_q_edge_errq_dn8,)
    }
};
        var_q_edge_errq = assign47630_e61051;
        var_q_edge_errq_dn5 = assign47630_e61051_d_n5;
        var_q_edge_errq_dn6 = assign47630_e61051_d_n6;
        var_q_edge_errq_dn7 = assign47630_e61051_d_n7;
        var_q_edge_errq_dn8 = assign47630_e61051_d_n8;
        var_q_edge_errq_rv = 0.0;

        let (assign47640_e61059, assign47640_e61059_d_n5, assign47640_e61059_d_n6, assign47640_e61059_d_n7, assign47640_e61059_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1250 != 0.0)) {
        let assign47640_e61057: f64 = (var_q_edge_qi0 - var_q_edge_errq);
        (assign47640_e61057, (var_q_edge_qi0_dn5 - var_q_edge_errq_dn5), (var_q_edge_qi0_dn6 - var_q_edge_errq_dn6), (var_q_edge_qi0_dn7 - var_q_edge_errq_dn7), (var_q_edge_qi0_dn8 - var_q_edge_errq_dn8),)
    } else {
        (var_qseffedge, var_qseffedge_dn5, var_qseffedge_dn6, var_qseffedge_dn7, var_qseffedge_dn8,)
    }
};
        var_qseffedge = assign47640_e61059;
        var_qseffedge_dn5 = assign47640_e61059_d_n5;
        var_qseffedge_dn6 = assign47640_e61059_d_n6;
        var_qseffedge_dn7 = assign47640_e61059_d_n7;
        var_qseffedge_dn8 = assign47640_e61059_d_n8;
        var_qseffedge_rv = 0.0;

        let assign47650_e61063: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47650_e61064: f64 = (var_q_edge_n_inv * assign47650_e61063);
        let assign47650_e61066: f64 = (-230.25850929940458);
        let assign47650_e61067: f64 = if assign47650_e61064 > assign47650_e61066 { 1.0 } else { 0.0 };
        var_guard1252 = assign47650_e61067;
        var_guard1252_rv = 0.0;

        let (assign47660_e61081, assign47660_e61081_d_n5, assign47660_e61081_d_n6, assign47660_e61081_d_n7, assign47660_e61081_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1250 == 0.0)) && (var_guard1252 != 0.0)) {
        let assign47660_e61077: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47660_e61078: f64 = (var_q_edge_n_inv * assign47660_e61077);
        let assign47660_e61079: f64 = (assign47660_e61078).exp();
        (assign47660_e61079, (assign47660_e61079 * ((var_q_edge_n_inv_dn5 * assign47660_e61077) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))), (assign47660_e61079 * ((var_q_edge_n_inv_dn6 * assign47660_e61077) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))), (assign47660_e61079 * ((var_q_edge_n_inv_dn7 * assign47660_e61077) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))), (assign47660_e61079 * ((var_q_edge_n_inv_dn8 * assign47660_e61077) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))),)
    } else {
        (var_qseffedge, var_qseffedge_dn5, var_qseffedge_dn6, var_qseffedge_dn7, var_qseffedge_dn8,)
    }
};
        var_qseffedge = assign47660_e61081;
        var_qseffedge_dn5 = assign47660_e61081_d_n5;
        var_qseffedge_dn6 = assign47660_e61081_d_n6;
        var_qseffedge_dn7 = assign47660_e61081_d_n7;
        var_qseffedge_dn8 = assign47660_e61081_d_n8;
        var_qseffedge_rv = 0.0;

        let (assign47670_e61128, assign47670_e61128_d_n5, assign47670_e61128_d_n6, assign47670_e61128_d_n7, assign47670_e61128_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1250 == 0.0)) && (var_guard1252 == 0.0)) {
        let assign47670_e61092: f64 = (-230.25850929940458);
        let assign47670_e61096: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47670_e61097: f64 = (var_q_edge_n_inv * assign47670_e61096);
        let assign47670_e61098: f64 = (assign47670_e61092 - assign47670_e61097);
        let assign47670_e61102: f64 = (-230.25850929940458);
        let assign47670_e61106: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47670_e61107: f64 = (var_q_edge_n_inv * assign47670_e61106);
        let assign47670_e61108: f64 = (assign47670_e61102 - assign47670_e61107);
        let assign47670_e61111: f64 = (-230.25850929940458);
        let assign47670_e61115: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47670_e61116: f64 = (var_q_edge_n_inv * assign47670_e61115);
        let assign47670_e61117: f64 = (assign47670_e61111 - assign47670_e61116);
        let assign47670_e61119: f64 = (assign47670_e61117 * 0.3333333333333333);
        let assign47670_e61120: f64 = (1.0 + assign47670_e61119);
        let assign47670_e61121: f64 = (assign47670_e61108 * assign47670_e61120);
        let assign47670_e61122: f64 = (0.5 * assign47670_e61121);
        let assign47670_e61123: f64 = (1.0 + assign47670_e61122);
        let assign47670_e61124: f64 = (assign47670_e61098 * assign47670_e61123);
        let assign47670_e61125: f64 = (1.0 + assign47670_e61124);
        let assign47670_e61126: f64 = (1e-100 / assign47670_e61125);
        (assign47670_e61126, (-((1e-100 * (((-((var_q_edge_n_inv_dn5 * assign47670_e61096) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((var_q_edge_n_inv_dn5 * assign47670_e61106) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * assign47670_e61120) + (assign47670_e61108 * ((-((var_q_edge_n_inv_dn5 * assign47670_e61115) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((var_q_edge_n_inv_dn6 * assign47670_e61096) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((var_q_edge_n_inv_dn6 * assign47670_e61106) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * assign47670_e61120) + (assign47670_e61108 * ((-((var_q_edge_n_inv_dn6 * assign47670_e61115) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((var_q_edge_n_inv_dn7 * assign47670_e61096) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((var_q_edge_n_inv_dn7 * assign47670_e61106) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * assign47670_e61120) + (assign47670_e61108 * ((-((var_q_edge_n_inv_dn7 * assign47670_e61115) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))), (-((1e-100 * (((-((var_q_edge_n_inv_dn8 * assign47670_e61096) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * assign47670_e61123) + (assign47670_e61098 * (0.5 * (((-((var_q_edge_n_inv_dn8 * assign47670_e61106) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * assign47670_e61120) + (assign47670_e61108 * ((-((var_q_edge_n_inv_dn8 * assign47670_e61115) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47670_e61125 * assign47670_e61125))),)
    } else {
        (var_qseffedge, var_qseffedge_dn5, var_qseffedge_dn6, var_qseffedge_dn7, var_qseffedge_dn8,)
    }
};
        var_qseffedge = assign47670_e61128;
        var_qseffedge_dn5 = assign47670_e61128_d_n5;
        var_qseffedge_dn6 = assign47670_e61128_d_n6;
        var_qseffedge_dn7 = assign47670_e61128_d_n7;
        var_qseffedge_dn8 = assign47670_e61128_d_n8;
        var_qseffedge_rv = 0.0;

        let (assign47680_e61136, assign47680_e61136_d_n5, assign47680_e61136_d_n6, assign47680_e61136_d_n7, assign47680_e61136_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47680_e61133: f64 = (var_vdse_dc + var_vsbstaredge);
        let assign47680_e61134: f64 = (var_inv_phit1edge * assign47680_e61133);
        (assign47680_e61134, ((var_inv_phit1edge_dn5 * assign47680_e61133) + (var_inv_phit1edge * (var_vdse_dc_dn5 + var_vsbstaredge_dn5))), ((var_inv_phit1edge_dn6 * assign47680_e61133) + (var_inv_phit1edge * (var_vdse_dc_dn6 + var_vsbstaredge_dn6))), ((var_inv_phit1edge_dn7 * assign47680_e61133) + (var_inv_phit1edge * (var_vdse_dc_dn7 + var_vsbstaredge_dn7))), ((var_inv_phit1edge_dn8 * assign47680_e61133) + (var_inv_phit1edge * (var_vdse_dc_dn8 + var_vsbstaredge_dn8))),)
    } else {
        (var_xnedge_d, var_xnedge_d_dn5, var_xnedge_d_dn6, var_xnedge_d_dn7, var_xnedge_d_dn8,)
    }
};
        var_xnedge_d = assign47680_e61136;
        var_xnedge_d_dn5 = assign47680_e61136_d_n5;
        var_xnedge_d_dn6 = assign47680_e61136_d_n6;
        var_xnedge_d_dn7 = assign47680_e61136_d_n7;
        var_xnedge_d_dn8 = assign47680_e61136_d_n8;
        var_xnedge_d_rv = 0.0;

        let assign47690_e61143: f64 = if ((var_qseffedge < 0.001) && (var_vdse_dc < 1e-6)) { 1.0 } else { 0.0 };
        var_guard1253 = assign47690_e61143;
        var_guard1253_rv = 0.0;

        let assign47700_e61145: f64 = (-var_xnedge_d);
        let assign47700_e61147: f64 = (assign47700_e61145 + var_xnedge_s);
        let assign47700_e61149: f64 = (-230.25850929940458);
        let assign47700_e61150: f64 = if assign47700_e61147 > assign47700_e61149 { 1.0 } else { 0.0 };
        var_guard1254 = assign47700_e61150;
        var_guard1254_rv = 0.0;

        let (assign47710_e61162, assign47710_e61162_d_n5, assign47710_e61162_d_n6, assign47710_e61162_d_n7, assign47710_e61162_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 != 0.0)) && (var_guard1254 != 0.0)) {
        let assign47710_e61157: f64 = (-var_xnedge_d);
        let assign47710_e61159: f64 = (assign47710_e61157 + var_xnedge_s);
        let assign47710_e61160: f64 = (assign47710_e61159).exp();
        (assign47710_e61160, (assign47710_e61160 * ((-var_xnedge_d_dn5) + var_xnedge_s_dn5)), (assign47710_e61160 * ((-var_xnedge_d_dn6) + var_xnedge_s_dn6)), (assign47710_e61160 * ((-var_xnedge_d_dn7) + var_xnedge_s_dn7)), (assign47710_e61160 * ((-var_xnedge_d_dn8) + var_xnedge_s_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47710_e61162;
        var_temp__blk936_dn5 = assign47710_e61162_d_n5;
        var_temp__blk936_dn6 = assign47710_e61162_d_n6;
        var_temp__blk936_dn7 = assign47710_e61162_d_n7;
        var_temp__blk936_dn8 = assign47710_e61162_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign47720_e61205, assign47720_e61205_d_n5, assign47720_e61205_d_n6, assign47720_e61205_d_n7, assign47720_e61205_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 != 0.0)) && (var_guard1254 == 0.0)) {
        let assign47720_e61172: f64 = (-230.25850929940458);
        let assign47720_e61174: f64 = (-var_xnedge_d);
        let assign47720_e61176: f64 = (assign47720_e61174 + var_xnedge_s);
        let assign47720_e61177: f64 = (assign47720_e61172 - assign47720_e61176);
        let assign47720_e61181: f64 = (-230.25850929940458);
        let assign47720_e61183: f64 = (-var_xnedge_d);
        let assign47720_e61185: f64 = (assign47720_e61183 + var_xnedge_s);
        let assign47720_e61186: f64 = (assign47720_e61181 - assign47720_e61185);
        let assign47720_e61189: f64 = (-230.25850929940458);
        let assign47720_e61191: f64 = (-var_xnedge_d);
        let assign47720_e61193: f64 = (assign47720_e61191 + var_xnedge_s);
        let assign47720_e61194: f64 = (assign47720_e61189 - assign47720_e61193);
        let assign47720_e61196: f64 = (assign47720_e61194 * 0.3333333333333333);
        let assign47720_e61197: f64 = (1.0 + assign47720_e61196);
        let assign47720_e61198: f64 = (assign47720_e61186 * assign47720_e61197);
        let assign47720_e61199: f64 = (0.5 * assign47720_e61198);
        let assign47720_e61200: f64 = (1.0 + assign47720_e61199);
        let assign47720_e61201: f64 = (assign47720_e61177 * assign47720_e61200);
        let assign47720_e61202: f64 = (1.0 + assign47720_e61201);
        let assign47720_e61203: f64 = (1e-100 / assign47720_e61202);
        (assign47720_e61203, (-((1e-100 * (((-((-var_xnedge_d_dn5) + var_xnedge_s_dn5)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-var_xnedge_d_dn5) + var_xnedge_s_dn5)) * assign47720_e61197) + (assign47720_e61186 * ((-((-var_xnedge_d_dn5) + var_xnedge_s_dn5)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-var_xnedge_d_dn6) + var_xnedge_s_dn6)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-var_xnedge_d_dn6) + var_xnedge_s_dn6)) * assign47720_e61197) + (assign47720_e61186 * ((-((-var_xnedge_d_dn6) + var_xnedge_s_dn6)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-var_xnedge_d_dn7) + var_xnedge_s_dn7)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-var_xnedge_d_dn7) + var_xnedge_s_dn7)) * assign47720_e61197) + (assign47720_e61186 * ((-((-var_xnedge_d_dn7) + var_xnedge_s_dn7)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))), (-((1e-100 * (((-((-var_xnedge_d_dn8) + var_xnedge_s_dn8)) * assign47720_e61200) + (assign47720_e61177 * (0.5 * (((-((-var_xnedge_d_dn8) + var_xnedge_s_dn8)) * assign47720_e61197) + (assign47720_e61186 * ((-((-var_xnedge_d_dn8) + var_xnedge_s_dn8)) * 0.3333333333333333))))))) / (assign47720_e61202 * assign47720_e61202))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign47720_e61205;
        var_temp__blk936_dn5 = assign47720_e61205_d_n5;
        var_temp__blk936_dn6 = assign47720_e61205_d_n6;
        var_temp__blk936_dn7 = assign47720_e61205_d_n7;
        var_temp__blk936_dn8 = assign47720_e61205_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign47730_e61215, assign47730_e61215_d_n5, assign47730_e61215_d_n6, assign47730_e61215_d_n7, assign47730_e61215_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 != 0.0)) {
        let assign47730_e61212: f64 = (var_temp__blk936 - 1.0);
        let assign47730_e61213: f64 = (var_qseffedge * assign47730_e61212);
        (assign47730_e61213, ((var_qseffedge_dn5 * assign47730_e61212) + (var_qseffedge * var_temp__blk936_dn5)), ((var_qseffedge_dn6 * assign47730_e61212) + (var_qseffedge * var_temp__blk936_dn6)), ((var_qseffedge_dn7 * assign47730_e61212) + (var_qseffedge * var_temp__blk936_dn7)), ((var_qseffedge_dn8 * assign47730_e61212) + (var_qseffedge * var_temp__blk936_dn8)),)
    } else {
        (var_qdseffedge, var_qdseffedge_dn5, var_qdseffedge_dn6, var_qdseffedge_dn7, var_qdseffedge_dn8,)
    }
};
        var_qdseffedge = assign47730_e61215;
        var_qdseffedge_dn5 = assign47730_e61215_d_n5;
        var_qdseffedge_dn6 = assign47730_e61215_d_n6;
        var_qdseffedge_dn7 = assign47730_e61215_d_n7;
        var_qdseffedge_dn8 = assign47730_e61215_d_n8;
        var_qdseffedge_rv = 0.0;

        let (assign47740_e61223, assign47740_e61223_d_n5, assign47740_e61223_d_n6, assign47740_e61223_d_n7, assign47740_e61223_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 != 0.0)) {
        let assign47740_e61221: f64 = (var_qdseffedge + var_qseffedge);
        (assign47740_e61221, (var_qdseffedge_dn5 + var_qseffedge_dn5), (var_qdseffedge_dn6 + var_qseffedge_dn6), (var_qdseffedge_dn7 + var_qseffedge_dn7), (var_qdseffedge_dn8 + var_qseffedge_dn8),)
    } else {
        (var_qdeffedge, var_qdeffedge_dn5, var_qdeffedge_dn6, var_qdeffedge_dn7, var_qdeffedge_dn8,)
    }
};
        var_qdeffedge = assign47740_e61223;
        var_qdeffedge_dn5 = assign47740_e61223_d_n5;
        var_qdeffedge_dn6 = assign47740_e61223_d_n6;
        var_qdeffedge_dn7 = assign47740_e61223_d_n7;
        var_qdeffedge_dn8 = assign47740_e61223_d_n8;
        var_qdeffedge_rv = 0.0;

        let (assign47750_e61232, assign47750_e61232_d_n5, assign47750_e61232_d_n6, assign47750_e61232_d_n7, assign47750_e61232_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47750_e61230: f64 = (var_xbedge + var_xnedge_d);
        (assign47750_e61230, (var_xbedge_dn5 + var_xnedge_d_dn5), (var_xbedge_dn6 + var_xnedge_d_dn6), (var_xbedge_dn7 + var_xnedge_d_dn7), (var_xbedge_dn8 + var_xnedge_d_dn8),)
    } else {
        (var_q_edge_xsth, var_q_edge_xsth_dn5, var_q_edge_xsth_dn6, var_q_edge_xsth_dn7, var_q_edge_xsth_dn8,)
    }
};
        var_q_edge_xsth = assign47750_e61232;
        var_q_edge_xsth_dn5 = assign47750_e61232_d_n5;
        var_q_edge_xsth_dn6 = assign47750_e61232_d_n6;
        var_q_edge_xsth_dn7 = assign47750_e61232_d_n7;
        var_q_edge_xsth_dn8 = assign47750_e61232_d_n8;
        var_q_edge_xsth_rv = 0.0;

        let (assign47760_e61244, assign47760_e61244_d_n5, assign47760_e61244_d_n6, assign47760_e61244_d_n7, assign47760_e61244_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47760_e61240: f64 = (var_q_edge_xsth).sqrt();
        let assign47760_e61241: f64 = (var_gfedge * assign47760_e61240);
        let assign47760_e61242: f64 = (var_q_edge_xsth + assign47760_e61241);
        (assign47760_e61242, (var_q_edge_xsth_dn5 + (var_gfedge * (var_q_edge_xsth_dn5 / (2.0 * assign47760_e61240)))), (var_q_edge_xsth_dn6 + (var_gfedge * (var_q_edge_xsth_dn6 / (2.0 * assign47760_e61240)))), (var_q_edge_xsth_dn7 + (var_gfedge * (var_q_edge_xsth_dn7 / (2.0 * assign47760_e61240)))), (var_q_edge_xsth_dn8 + (var_gfedge * (var_q_edge_xsth_dn8 / (2.0 * assign47760_e61240)))),)
    } else {
        (var_q_edge_xth0, var_q_edge_xth0_dn5, var_q_edge_xth0_dn6, var_q_edge_xth0_dn7, var_q_edge_xth0_dn8,)
    }
};
        var_q_edge_xth0 = assign47760_e61244;
        var_q_edge_xth0_dn5 = assign47760_e61244_d_n5;
        var_q_edge_xth0_dn6 = assign47760_e61244_d_n6;
        var_q_edge_xth0_dn7 = assign47760_e61244_d_n7;
        var_q_edge_xth0_dn8 = assign47760_e61244_d_n8;
        var_q_edge_xth0_rv = 0.0;

        let (assign47770_e61253, assign47770_e61253_d_n5, assign47770_e61253_d_n6, assign47770_e61253_d_n7, assign47770_e61253_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47770_e61251: f64 = (var_q_edge_xth0 + var_dxthedge);
        (assign47770_e61251, (var_q_edge_xth0_dn5 + var_dxthedge_dn5), (var_q_edge_xth0_dn6 + var_dxthedge_dn6), (var_q_edge_xth0_dn7 + var_dxthedge_dn7), (var_q_edge_xth0_dn8 + var_dxthedge_dn8),)
    } else {
        (var_q_edge_xth, var_q_edge_xth_dn5, var_q_edge_xth_dn6, var_q_edge_xth_dn7, var_q_edge_xth_dn8,)
    }
};
        var_q_edge_xth = assign47770_e61253;
        var_q_edge_xth_dn5 = assign47770_e61253_d_n5;
        var_q_edge_xth_dn6 = assign47770_e61253_d_n6;
        var_q_edge_xth_dn7 = assign47770_e61253_d_n7;
        var_q_edge_xth_dn8 = assign47770_e61253_d_n8;
        var_q_edge_xth_rv = 0.0;

        let (assign47780_e61267, assign47780_e61267_d_n5, assign47780_e61267_d_n6, assign47780_e61267_d_n7, assign47780_e61267_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47780_e61262: f64 = (var_q_edge_xsth).sqrt();
        let assign47780_e61263: f64 = (2.0 * assign47780_e61262);
        let assign47780_e61264: f64 = (var_gfedge / assign47780_e61263);
        let assign47780_e61265: f64 = (1.0 + assign47780_e61264);
        (assign47780_e61265, (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn5 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn6 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn7 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))), (-((var_gfedge * (2.0 * (var_q_edge_xsth_dn8 / (2.0 * assign47780_e61262)))) / (assign47780_e61263 * assign47780_e61263))),)
    } else {
        (var_q_edge_n, var_q_edge_n_dn5, var_q_edge_n_dn6, var_q_edge_n_dn7, var_q_edge_n_dn8,)
    }
};
        var_q_edge_n = assign47780_e61267;
        var_q_edge_n_dn5 = assign47780_e61267_d_n5;
        var_q_edge_n_dn6 = assign47780_e61267_d_n6;
        var_q_edge_n_dn7 = assign47780_e61267_d_n7;
        var_q_edge_n_dn8 = assign47780_e61267_d_n8;
        var_q_edge_n_rv = 0.0;

        let (assign47790_e61276, assign47790_e61276_d_n5, assign47790_e61276_d_n6, assign47790_e61276_d_n7, assign47790_e61276_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47790_e61274: f64 = (1.0 / var_q_edge_n);
        (assign47790_e61274, (-(var_q_edge_n_dn5 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn6 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn7 / (var_q_edge_n * var_q_edge_n))), (-(var_q_edge_n_dn8 / (var_q_edge_n * var_q_edge_n))),)
    } else {
        (var_q_edge_n_inv, var_q_edge_n_inv_dn5, var_q_edge_n_inv_dn6, var_q_edge_n_inv_dn7, var_q_edge_n_inv_dn8,)
    }
};
        var_q_edge_n_inv = assign47790_e61276;
        var_q_edge_n_inv_dn5 = assign47790_e61276_d_n5;
        var_q_edge_n_inv_dn6 = assign47790_e61276_d_n6;
        var_q_edge_n_inv_dn7 = assign47790_e61276_d_n7;
        var_q_edge_n_inv_dn8 = assign47790_e61276_d_n8;
        var_q_edge_n_inv_rv = 0.0;

        let (assign47800_e61285, assign47800_e61285_d_n5, assign47800_e61285_d_n6, assign47800_e61285_d_n7, assign47800_e61285_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47800_e61283: f64 = (var_xgedge - var_q_edge_xth);
        (assign47800_e61283, (var_xgedge_dn5 - var_q_edge_xth_dn5), (var_xgedge_dn6 - var_q_edge_xth_dn6), (var_xgedge_dn7 - var_q_edge_xth_dn7), (var_xgedge_dn8 - var_q_edge_xth_dn8),)
    } else {
        (var_q_edge_xgt, var_q_edge_xgt_dn5, var_q_edge_xgt_dn6, var_q_edge_xgt_dn7, var_q_edge_xgt_dn8,)
    }
};
        var_q_edge_xgt = assign47800_e61285;
        var_q_edge_xgt_dn5 = assign47800_e61285_d_n5;
        var_q_edge_xgt_dn6 = assign47800_e61285_d_n6;
        var_q_edge_xgt_dn7 = assign47800_e61285_d_n7;
        var_q_edge_xgt_dn8 = assign47800_e61285_d_n8;
        var_q_edge_xgt_rv = 0.0;

        let assign47810_e61288: f64 = (-12.0);
        let assign47810_e61289: f64 = if var_q_edge_xgt > assign47810_e61288 { 1.0 } else { 0.0 };
        var_guard1255 = assign47810_e61289;
        var_guard1255_rv = 0.0;

        let (assign47820_e61302, assign47820_e61302_d_n5, assign47820_e61302_d_n6, assign47820_e61302_d_n7, assign47820_e61302_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47820_e61298: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47820_e61300: f64 = (assign47820_e61298 - 1.0);
        (assign47820_e61300, var_q_edge_xgt_dn5, var_q_edge_xgt_dn6, var_q_edge_xgt_dn7, var_q_edge_xgt_dn8,)
    } else {
        (var_q_edge_xgt0, var_q_edge_xgt0_dn5, var_q_edge_xgt0_dn6, var_q_edge_xgt0_dn7, var_q_edge_xgt0_dn8,)
    }
};
        var_q_edge_xgt0 = assign47820_e61302;
        var_q_edge_xgt0_dn5 = assign47820_e61302_d_n5;
        var_q_edge_xgt0_dn6 = assign47820_e61302_d_n6;
        var_q_edge_xgt0_dn7 = assign47820_e61302_d_n7;
        var_q_edge_xgt0_dn8 = assign47820_e61302_d_n8;
        var_q_edge_xgt0_rv = 0.0;

        let (assign47830_e61320, assign47830_e61320_d_n5, assign47830_e61320_d_n6, assign47830_e61320_d_n7, assign47830_e61320_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47830_e61313: f64 = (var_q_edge_xgt0 * var_q_edge_xgt0);
        let assign47830_e61315: f64 = (assign47830_e61313 + 10.0);
        let assign47830_e61316: f64 = (assign47830_e61315).sqrt();
        let assign47830_e61317: f64 = (var_q_edge_xgt0 + assign47830_e61316);
        let assign47830_e61318: f64 = (0.5 * assign47830_e61317);
        (assign47830_e61318, (0.5 * (var_q_edge_xgt0_dn5 + (((var_q_edge_xgt0_dn5 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn5)) / (2.0 * assign47830_e61316)))), (0.5 * (var_q_edge_xgt0_dn6 + (((var_q_edge_xgt0_dn6 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn6)) / (2.0 * assign47830_e61316)))), (0.5 * (var_q_edge_xgt0_dn7 + (((var_q_edge_xgt0_dn7 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn7)) / (2.0 * assign47830_e61316)))), (0.5 * (var_q_edge_xgt0_dn8 + (((var_q_edge_xgt0_dn8 * var_q_edge_xgt0) + (var_q_edge_xgt0 * var_q_edge_xgt0_dn8)) / (2.0 * assign47830_e61316)))),)
    } else {
        (var_q_edge_xgt0e, var_q_edge_xgt0e_dn5, var_q_edge_xgt0e_dn6, var_q_edge_xgt0e_dn7, var_q_edge_xgt0e_dn8,)
    }
};
        var_q_edge_xgt0e = assign47830_e61320;
        var_q_edge_xgt0e_dn5 = assign47830_e61320_d_n5;
        var_q_edge_xgt0e_dn6 = assign47830_e61320_d_n6;
        var_q_edge_xgt0e_dn7 = assign47830_e61320_d_n7;
        var_q_edge_xgt0e_dn8 = assign47830_e61320_d_n8;
        var_q_edge_xgt0e_rv = 0.0;

        let (assign47840_e61336, assign47840_e61336_d_n5, assign47840_e61336_d_n6, assign47840_e61336_d_n7, assign47840_e61336_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47840_e61330: f64 = (var_q_edge_xgt0e).ln();
        let assign47840_e61331: f64 = (var_q_edge_n * assign47840_e61330);
        let assign47840_e61332: f64 = (var_q_edge_xgt - assign47840_e61331);
        let assign47840_e61334: f64 = (assign47840_e61332 + var_lngfedge2);
        (assign47840_e61334, (var_q_edge_xgt_dn5 - ((var_q_edge_n_dn5 * assign47840_e61330) + (var_q_edge_n * (var_q_edge_xgt0e_dn5 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn6 - ((var_q_edge_n_dn6 * assign47840_e61330) + (var_q_edge_n * (var_q_edge_xgt0e_dn6 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn7 - ((var_q_edge_n_dn7 * assign47840_e61330) + (var_q_edge_n * (var_q_edge_xgt0e_dn7 / var_q_edge_xgt0e)))), (var_q_edge_xgt_dn8 - ((var_q_edge_n_dn8 * assign47840_e61330) + (var_q_edge_n * (var_q_edge_xgt0e_dn8 / var_q_edge_xgt0e)))),)
    } else {
        (var_q_edge_qi0si, var_q_edge_qi0si_dn5, var_q_edge_qi0si_dn6, var_q_edge_qi0si_dn7, var_q_edge_qi0si_dn8,)
    }
};
        var_q_edge_qi0si = assign47840_e61336;
        var_q_edge_qi0si_dn5 = assign47840_e61336_d_n5;
        var_q_edge_qi0si_dn6 = assign47840_e61336_d_n6;
        var_q_edge_qi0si_dn7 = assign47840_e61336_d_n7;
        var_q_edge_qi0si_dn8 = assign47840_e61336_d_n8;
        var_q_edge_qi0si_rv = 0.0;

        let (assign47850_e61354, assign47850_e61354_d_n5, assign47850_e61354_d_n6, assign47850_e61354_d_n7, assign47850_e61354_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47850_e61347: f64 = (var_q_edge_qi0si * var_q_edge_qi0si);
        let assign47850_e61349: f64 = (assign47850_e61347 + 2.0);
        let assign47850_e61350: f64 = (assign47850_e61349).sqrt();
        let assign47850_e61351: f64 = (var_q_edge_qi0si + assign47850_e61350);
        let assign47850_e61352: f64 = (0.5 * assign47850_e61351);
        (assign47850_e61352, (0.5 * (var_q_edge_qi0si_dn5 + (((var_q_edge_qi0si_dn5 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn5)) / (2.0 * assign47850_e61350)))), (0.5 * (var_q_edge_qi0si_dn6 + (((var_q_edge_qi0si_dn6 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn6)) / (2.0 * assign47850_e61350)))), (0.5 * (var_q_edge_qi0si_dn7 + (((var_q_edge_qi0si_dn7 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn7)) / (2.0 * assign47850_e61350)))), (0.5 * (var_q_edge_qi0si_dn8 + (((var_q_edge_qi0si_dn8 * var_q_edge_qi0si) + (var_q_edge_qi0si * var_q_edge_qi0si_dn8)) / (2.0 * assign47850_e61350)))),)
    } else {
        (var_q_edge_qi0, var_q_edge_qi0_dn5, var_q_edge_qi0_dn6, var_q_edge_qi0_dn7, var_q_edge_qi0_dn8,)
    }
};
        var_q_edge_qi0 = assign47850_e61354;
        var_q_edge_qi0_dn5 = assign47850_e61354_d_n5;
        var_q_edge_qi0_dn6 = assign47850_e61354_d_n6;
        var_q_edge_qi0_dn7 = assign47850_e61354_d_n7;
        var_q_edge_qi0_dn8 = assign47850_e61354_d_n8;
        var_q_edge_qi0_rv = 0.0;

        let assign47860_e61357: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47860_e61359: f64 = if assign47860_e61357 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1256 = assign47860_e61359;
        var_guard1256_rv = 0.0;

        let (assign47870_e61373, assign47870_e61373_d_n5, assign47870_e61373_d_n6, assign47870_e61373_d_n7, assign47870_e61373_d_n8,) = {
    if ((((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) && (var_guard1256 != 0.0)) {
        let assign47870_e61370: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47870_e61371: f64 = (assign47870_e61370).exp();
        (assign47870_e61371, (assign47870_e61371 * (var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5)), (assign47870_e61371 * (var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6)), (assign47870_e61371 * (var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7)), (assign47870_e61371 * (var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8)),)
    } else {
        (var_q_edge_exp_x, var_q_edge_exp_x_dn5, var_q_edge_exp_x_dn6, var_q_edge_exp_x_dn7, var_q_edge_exp_x_dn8,)
    }
};
        var_q_edge_exp_x = assign47870_e61373;
        var_q_edge_exp_x_dn5 = assign47870_e61373_d_n5;
        var_q_edge_exp_x_dn6 = assign47870_e61373_d_n6;
        var_q_edge_exp_x_dn7 = assign47870_e61373_d_n7;
        var_q_edge_exp_x_dn8 = assign47870_e61373_d_n8;
        var_q_edge_exp_x_rv = 0.0;

        let (assign47880_e61413, assign47880_e61413_d_n5, assign47880_e61413_d_n6, assign47880_e61413_d_n7, assign47880_e61413_d_n8,) = {
    if ((((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) && (var_guard1256 == 0.0)) {
        let assign47880_e61387: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47880_e61389: f64 = (assign47880_e61387 - 230.25850929940458);
        let assign47880_e61394: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47880_e61396: f64 = (assign47880_e61394 - 230.25850929940458);
        let assign47880_e61400: f64 = (var_q_edge_xgt - var_q_edge_qi0);
        let assign47880_e61402: f64 = (assign47880_e61400 - 230.25850929940458);
        let assign47880_e61404: f64 = (assign47880_e61402 * 0.3333333333333333);
        let assign47880_e61405: f64 = (1.0 + assign47880_e61404);
        let assign47880_e61406: f64 = (assign47880_e61396 * assign47880_e61405);
        let assign47880_e61407: f64 = (0.5 * assign47880_e61406);
        let assign47880_e61408: f64 = (1.0 + assign47880_e61407);
        let assign47880_e61409: f64 = (assign47880_e61389 * assign47880_e61408);
        let assign47880_e61410: f64 = (1.0 + assign47880_e61409);
        let assign47880_e61411: f64 = (1e100 * assign47880_e61410);
        (assign47880_e61411, (1e100 * (((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * assign47880_e61405) + (assign47880_e61396 * ((var_q_edge_xgt_dn5 - var_q_edge_qi0_dn5) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * assign47880_e61405) + (assign47880_e61396 * ((var_q_edge_xgt_dn6 - var_q_edge_qi0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * assign47880_e61405) + (assign47880_e61396 * ((var_q_edge_xgt_dn7 - var_q_edge_qi0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * assign47880_e61408) + (assign47880_e61389 * (0.5 * (((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * assign47880_e61405) + (assign47880_e61396 * ((var_q_edge_xgt_dn8 - var_q_edge_qi0_dn8) * 0.3333333333333333))))))),)
    } else {
        (var_q_edge_exp_x, var_q_edge_exp_x_dn5, var_q_edge_exp_x_dn6, var_q_edge_exp_x_dn7, var_q_edge_exp_x_dn8,)
    }
};
        var_q_edge_exp_x = assign47880_e61413;
        var_q_edge_exp_x_dn5 = assign47880_e61413_d_n5;
        var_q_edge_exp_x_dn6 = assign47880_e61413_d_n6;
        var_q_edge_exp_x_dn7 = assign47880_e61413_d_n7;
        var_q_edge_exp_x_dn8 = assign47880_e61413_d_n8;
        var_q_edge_exp_x_rv = 0.0;

        let (assign47890_e61424, assign47890_e61424_d_n5, assign47890_e61424_d_n6, assign47890_e61424_d_n7, assign47890_e61424_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47890_e61422: f64 = (var_gfedge2 * var_q_edge_exp_x);
        (assign47890_e61422, (var_gfedge2 * var_q_edge_exp_x_dn5), (var_gfedge2 * var_q_edge_exp_x_dn6), (var_gfedge2 * var_q_edge_exp_x_dn7), (var_gfedge2 * var_q_edge_exp_x_dn8),)
    } else {
        (var_q_edge_d0, var_q_edge_d0_dn5, var_q_edge_d0_dn6, var_q_edge_d0_dn7, var_q_edge_d0_dn8,)
    }
};
        var_q_edge_d0 = assign47890_e61424;
        var_q_edge_d0_dn5 = assign47890_e61424_d_n5;
        var_q_edge_d0_dn6 = assign47890_e61424_d_n6;
        var_q_edge_d0_dn7 = assign47890_e61424_d_n7;
        var_q_edge_d0_dn8 = assign47890_e61424_d_n8;
        var_q_edge_d0_rv = 0.0;

        let (assign47900_e61435, assign47900_e61435_d_n5, assign47900_e61435_d_n6, assign47900_e61435_d_n7, assign47900_e61435_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47900_e61433: f64 = (var_q_edge_d0).powf(var_q_edge_n_inv);
        (assign47900_e61433, if var_q_edge_n_inv_dn5 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn5)) } } else { (assign47900_e61433 * ((var_q_edge_n_inv_dn5 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn5 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn6 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn6)) } } else { (assign47900_e61433 * ((var_q_edge_n_inv_dn6 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn6 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn7 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn7)) } } else { (assign47900_e61433 * ((var_q_edge_n_inv_dn7 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn7 / var_q_edge_d0)))) }, if var_q_edge_n_inv_dn8 == 0.0 && ((var_q_edge_n_inv) as f64).is_finite() && ((var_q_edge_n_inv) as f64).fract() == 0.0 { if var_q_edge_n_inv == 0.0 { 0.0 } else { (var_q_edge_n_inv * ((var_q_edge_d0).powf(var_q_edge_n_inv - 1.0) * var_q_edge_d0_dn8)) } } else { (assign47900_e61433 * ((var_q_edge_n_inv_dn8 * (var_q_edge_d0).ln()) + (var_q_edge_n_inv * (var_q_edge_d0_dn8 / var_q_edge_d0)))) },)
    } else {
        (var_q_edge_d0p, var_q_edge_d0p_dn5, var_q_edge_d0p_dn6, var_q_edge_d0p_dn7, var_q_edge_d0p_dn8,)
    }
};
        var_q_edge_d0p = assign47900_e61435;
        var_q_edge_d0p_dn5 = assign47900_e61435_d_n5;
        var_q_edge_d0p_dn6 = assign47900_e61435_d_n6;
        var_q_edge_d0p_dn7 = assign47900_e61435_d_n7;
        var_q_edge_d0p_dn8 = assign47900_e61435_d_n8;
        var_q_edge_d0p_rv = 0.0;

        let (assign47910_e61456, assign47910_e61456_d_n5, assign47910_e61456_d_n6, assign47910_e61456_d_n7, assign47910_e61456_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47910_e61444: f64 = (var_q_edge_n * var_q_edge_n);
        let assign47910_e61448: f64 = (var_q_edge_qi0 + var_q_edge_n);
        let assign47910_e61449: f64 = (2.0 * assign47910_e61448);
        let assign47910_e61451: f64 = (assign47910_e61449 - var_q_edge_d0p);
        let assign47910_e61453: f64 = (assign47910_e61451 * var_q_edge_d0p);
        let assign47910_e61454: f64 = (assign47910_e61444 + assign47910_e61453);
        (assign47910_e61454, (((var_q_edge_n_dn5 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn5)) + ((((2.0 * (var_q_edge_qi0_dn5 + var_q_edge_n_dn5)) - var_q_edge_d0p_dn5) * var_q_edge_d0p) + (assign47910_e61451 * var_q_edge_d0p_dn5))), (((var_q_edge_n_dn6 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn6)) + ((((2.0 * (var_q_edge_qi0_dn6 + var_q_edge_n_dn6)) - var_q_edge_d0p_dn6) * var_q_edge_d0p) + (assign47910_e61451 * var_q_edge_d0p_dn6))), (((var_q_edge_n_dn7 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn7)) + ((((2.0 * (var_q_edge_qi0_dn7 + var_q_edge_n_dn7)) - var_q_edge_d0p_dn7) * var_q_edge_d0p) + (assign47910_e61451 * var_q_edge_d0p_dn7))), (((var_q_edge_n_dn8 * var_q_edge_n) + (var_q_edge_n * var_q_edge_n_dn8)) + ((((2.0 * (var_q_edge_qi0_dn8 + var_q_edge_n_dn8)) - var_q_edge_d0p_dn8) * var_q_edge_d0p) + (assign47910_e61451 * var_q_edge_d0p_dn8))),)
    } else {
        (var_q_edge_sqerr, var_q_edge_sqerr_dn5, var_q_edge_sqerr_dn6, var_q_edge_sqerr_dn7, var_q_edge_sqerr_dn8,)
    }
};
        var_q_edge_sqerr = assign47910_e61456;
        var_q_edge_sqerr_dn5 = assign47910_e61456_d_n5;
        var_q_edge_sqerr_dn6 = assign47910_e61456_d_n6;
        var_q_edge_sqerr_dn7 = assign47910_e61456_d_n7;
        var_q_edge_sqerr_dn8 = assign47910_e61456_d_n8;
        var_q_edge_sqerr_rv = 0.0;

        let (assign47920_e61474, assign47920_e61474_d_n5, assign47920_e61474_d_n6, assign47920_e61474_d_n7, assign47920_e61474_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47920_e61465: f64 = (var_q_edge_sqerr).sqrt();
        let assign47920_e61467: f64 = (assign47920_e61465 - var_q_edge_n);
        let assign47920_e61469: f64 = (assign47920_e61467 / var_q_edge_d0p);
        let assign47920_e61471: f64 = (assign47920_e61469 - 1.0);
        let assign47920_e61472: f64 = (var_q_edge_n * assign47920_e61471);
        (assign47920_e61472, ((var_q_edge_n_dn5 * assign47920_e61471) + (var_q_edge_n * (((((var_q_edge_sqerr_dn5 / (2.0 * assign47920_e61465)) - var_q_edge_n_dn5) * var_q_edge_d0p) - (assign47920_e61467 * var_q_edge_d0p_dn5)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn6 * assign47920_e61471) + (var_q_edge_n * (((((var_q_edge_sqerr_dn6 / (2.0 * assign47920_e61465)) - var_q_edge_n_dn6) * var_q_edge_d0p) - (assign47920_e61467 * var_q_edge_d0p_dn6)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn7 * assign47920_e61471) + (var_q_edge_n * (((((var_q_edge_sqerr_dn7 / (2.0 * assign47920_e61465)) - var_q_edge_n_dn7) * var_q_edge_d0p) - (assign47920_e61467 * var_q_edge_d0p_dn7)) / (var_q_edge_d0p * var_q_edge_d0p)))), ((var_q_edge_n_dn8 * assign47920_e61471) + (var_q_edge_n * (((((var_q_edge_sqerr_dn8 / (2.0 * assign47920_e61465)) - var_q_edge_n_dn8) * var_q_edge_d0p) - (assign47920_e61467 * var_q_edge_d0p_dn8)) / (var_q_edge_d0p * var_q_edge_d0p)))),)
    } else {
        (var_q_edge_errq, var_q_edge_errq_dn5, var_q_edge_errq_dn6, var_q_edge_errq_dn7, var_q_edge_errq_dn8,)
    }
};
        var_q_edge_errq = assign47920_e61474;
        var_q_edge_errq_dn5 = assign47920_e61474_d_n5;
        var_q_edge_errq_dn6 = assign47920_e61474_d_n6;
        var_q_edge_errq_dn7 = assign47920_e61474_d_n7;
        var_q_edge_errq_dn8 = assign47920_e61474_d_n8;
        var_q_edge_errq_rv = 0.0;

        let (assign47930_e61485, assign47930_e61485_d_n5, assign47930_e61485_d_n6, assign47930_e61485_d_n7, assign47930_e61485_d_n8,) = {
    if (((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 != 0.0)) {
        let assign47930_e61483: f64 = (var_q_edge_qi0 - var_q_edge_errq);
        (assign47930_e61483, (var_q_edge_qi0_dn5 - var_q_edge_errq_dn5), (var_q_edge_qi0_dn6 - var_q_edge_errq_dn6), (var_q_edge_qi0_dn7 - var_q_edge_errq_dn7), (var_q_edge_qi0_dn8 - var_q_edge_errq_dn8),)
    } else {
        (var_qdeffedge, var_qdeffedge_dn5, var_qdeffedge_dn6, var_qdeffedge_dn7, var_qdeffedge_dn8,)
    }
};
        var_qdeffedge = assign47930_e61485;
        var_qdeffedge_dn5 = assign47930_e61485_d_n5;
        var_qdeffedge_dn6 = assign47930_e61485_d_n6;
        var_qdeffedge_dn7 = assign47930_e61485_d_n7;
        var_qdeffedge_dn8 = assign47930_e61485_d_n8;
        var_qdeffedge_rv = 0.0;

        *var_guard1252_slot = var_guard1252;
        *var_guard1252_rv_slot = var_guard1252_rv;
        *var_guard1253_slot = var_guard1253;
        *var_guard1253_rv_slot = var_guard1253_rv;
        *var_guard1254_slot = var_guard1254;
        *var_guard1254_rv_slot = var_guard1254_rv;
        *var_guard1255_slot = var_guard1255;
        *var_guard1255_rv_slot = var_guard1255_rv;
        *var_guard1256_slot = var_guard1256;
        *var_guard1256_rv_slot = var_guard1256_rv;
        *var_q_edge_d0_slot = var_q_edge_d0;
        *var_q_edge_d0_dn5_slot = var_q_edge_d0_dn5;
        *var_q_edge_d0_dn6_slot = var_q_edge_d0_dn6;
        *var_q_edge_d0_dn7_slot = var_q_edge_d0_dn7;
        *var_q_edge_d0_dn8_slot = var_q_edge_d0_dn8;
        *var_q_edge_d0_rv_slot = var_q_edge_d0_rv;
        *var_q_edge_d0p_slot = var_q_edge_d0p;
        *var_q_edge_d0p_dn5_slot = var_q_edge_d0p_dn5;
        *var_q_edge_d0p_dn6_slot = var_q_edge_d0p_dn6;
        *var_q_edge_d0p_dn7_slot = var_q_edge_d0p_dn7;
        *var_q_edge_d0p_dn8_slot = var_q_edge_d0p_dn8;
        *var_q_edge_d0p_rv_slot = var_q_edge_d0p_rv;
        *var_q_edge_errq_slot = var_q_edge_errq;
        *var_q_edge_errq_dn5_slot = var_q_edge_errq_dn5;
        *var_q_edge_errq_dn6_slot = var_q_edge_errq_dn6;
        *var_q_edge_errq_dn7_slot = var_q_edge_errq_dn7;
        *var_q_edge_errq_dn8_slot = var_q_edge_errq_dn8;
        *var_q_edge_errq_rv_slot = var_q_edge_errq_rv;
        *var_q_edge_exp_x_slot = var_q_edge_exp_x;
        *var_q_edge_exp_x_dn5_slot = var_q_edge_exp_x_dn5;
        *var_q_edge_exp_x_dn6_slot = var_q_edge_exp_x_dn6;
        *var_q_edge_exp_x_dn7_slot = var_q_edge_exp_x_dn7;
        *var_q_edge_exp_x_dn8_slot = var_q_edge_exp_x_dn8;
        *var_q_edge_exp_x_rv_slot = var_q_edge_exp_x_rv;
        *var_q_edge_n_slot = var_q_edge_n;
        *var_q_edge_n_dn5_slot = var_q_edge_n_dn5;
        *var_q_edge_n_dn6_slot = var_q_edge_n_dn6;
        *var_q_edge_n_dn7_slot = var_q_edge_n_dn7;
        *var_q_edge_n_dn8_slot = var_q_edge_n_dn8;
        *var_q_edge_n_inv_slot = var_q_edge_n_inv;
        *var_q_edge_n_inv_dn5_slot = var_q_edge_n_inv_dn5;
        *var_q_edge_n_inv_dn6_slot = var_q_edge_n_inv_dn6;
        *var_q_edge_n_inv_dn7_slot = var_q_edge_n_inv_dn7;
        *var_q_edge_n_inv_dn8_slot = var_q_edge_n_inv_dn8;
        *var_q_edge_n_inv_rv_slot = var_q_edge_n_inv_rv;
        *var_q_edge_n_rv_slot = var_q_edge_n_rv;
        *var_q_edge_qi0_slot = var_q_edge_qi0;
        *var_q_edge_qi0_dn5_slot = var_q_edge_qi0_dn5;
        *var_q_edge_qi0_dn6_slot = var_q_edge_qi0_dn6;
        *var_q_edge_qi0_dn7_slot = var_q_edge_qi0_dn7;
        *var_q_edge_qi0_dn8_slot = var_q_edge_qi0_dn8;
        *var_q_edge_qi0_rv_slot = var_q_edge_qi0_rv;
        *var_q_edge_qi0si_slot = var_q_edge_qi0si;
        *var_q_edge_qi0si_dn5_slot = var_q_edge_qi0si_dn5;
        *var_q_edge_qi0si_dn6_slot = var_q_edge_qi0si_dn6;
        *var_q_edge_qi0si_dn7_slot = var_q_edge_qi0si_dn7;
        *var_q_edge_qi0si_dn8_slot = var_q_edge_qi0si_dn8;
        *var_q_edge_qi0si_rv_slot = var_q_edge_qi0si_rv;
        *var_q_edge_sqerr_slot = var_q_edge_sqerr;
        *var_q_edge_sqerr_dn5_slot = var_q_edge_sqerr_dn5;
        *var_q_edge_sqerr_dn6_slot = var_q_edge_sqerr_dn6;
        *var_q_edge_sqerr_dn7_slot = var_q_edge_sqerr_dn7;
        *var_q_edge_sqerr_dn8_slot = var_q_edge_sqerr_dn8;
        *var_q_edge_sqerr_rv_slot = var_q_edge_sqerr_rv;
        *var_q_edge_xgt_slot = var_q_edge_xgt;
        *var_q_edge_xgt0_slot = var_q_edge_xgt0;
        *var_q_edge_xgt0_dn5_slot = var_q_edge_xgt0_dn5;
        *var_q_edge_xgt0_dn6_slot = var_q_edge_xgt0_dn6;
        *var_q_edge_xgt0_dn7_slot = var_q_edge_xgt0_dn7;
        *var_q_edge_xgt0_dn8_slot = var_q_edge_xgt0_dn8;
        *var_q_edge_xgt0_rv_slot = var_q_edge_xgt0_rv;
        *var_q_edge_xgt0e_slot = var_q_edge_xgt0e;
        *var_q_edge_xgt0e_dn5_slot = var_q_edge_xgt0e_dn5;
        *var_q_edge_xgt0e_dn6_slot = var_q_edge_xgt0e_dn6;
        *var_q_edge_xgt0e_dn7_slot = var_q_edge_xgt0e_dn7;
        *var_q_edge_xgt0e_dn8_slot = var_q_edge_xgt0e_dn8;
        *var_q_edge_xgt0e_rv_slot = var_q_edge_xgt0e_rv;
        *var_q_edge_xgt_dn5_slot = var_q_edge_xgt_dn5;
        *var_q_edge_xgt_dn6_slot = var_q_edge_xgt_dn6;
        *var_q_edge_xgt_dn7_slot = var_q_edge_xgt_dn7;
        *var_q_edge_xgt_dn8_slot = var_q_edge_xgt_dn8;
        *var_q_edge_xgt_rv_slot = var_q_edge_xgt_rv;
        *var_q_edge_xsth_slot = var_q_edge_xsth;
        *var_q_edge_xsth_dn5_slot = var_q_edge_xsth_dn5;
        *var_q_edge_xsth_dn6_slot = var_q_edge_xsth_dn6;
        *var_q_edge_xsth_dn7_slot = var_q_edge_xsth_dn7;
        *var_q_edge_xsth_dn8_slot = var_q_edge_xsth_dn8;
        *var_q_edge_xsth_rv_slot = var_q_edge_xsth_rv;
        *var_q_edge_xth_slot = var_q_edge_xth;
        *var_q_edge_xth0_slot = var_q_edge_xth0;
        *var_q_edge_xth0_dn5_slot = var_q_edge_xth0_dn5;
        *var_q_edge_xth0_dn6_slot = var_q_edge_xth0_dn6;
        *var_q_edge_xth0_dn7_slot = var_q_edge_xth0_dn7;
        *var_q_edge_xth0_dn8_slot = var_q_edge_xth0_dn8;
        *var_q_edge_xth0_rv_slot = var_q_edge_xth0_rv;
        *var_q_edge_xth_dn5_slot = var_q_edge_xth_dn5;
        *var_q_edge_xth_dn6_slot = var_q_edge_xth_dn6;
        *var_q_edge_xth_dn7_slot = var_q_edge_xth_dn7;
        *var_q_edge_xth_dn8_slot = var_q_edge_xth_dn8;
        *var_q_edge_xth_rv_slot = var_q_edge_xth_rv;
        *var_qdeffedge_slot = var_qdeffedge;
        *var_qdeffedge_dn5_slot = var_qdeffedge_dn5;
        *var_qdeffedge_dn6_slot = var_qdeffedge_dn6;
        *var_qdeffedge_dn7_slot = var_qdeffedge_dn7;
        *var_qdeffedge_dn8_slot = var_qdeffedge_dn8;
        *var_qdeffedge_rv_slot = var_qdeffedge_rv;
        *var_qdseffedge_slot = var_qdseffedge;
        *var_qdseffedge_dn5_slot = var_qdseffedge_dn5;
        *var_qdseffedge_dn6_slot = var_qdseffedge_dn6;
        *var_qdseffedge_dn7_slot = var_qdseffedge_dn7;
        *var_qdseffedge_dn8_slot = var_qdseffedge_dn8;
        *var_qdseffedge_rv_slot = var_qdseffedge_rv;
        *var_qseffedge_slot = var_qseffedge;
        *var_qseffedge_dn5_slot = var_qseffedge_dn5;
        *var_qseffedge_dn6_slot = var_qseffedge_dn6;
        *var_qseffedge_dn7_slot = var_qseffedge_dn7;
        *var_qseffedge_dn8_slot = var_qseffedge_dn8;
        *var_qseffedge_rv_slot = var_qseffedge_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_xnedge_d_slot = var_xnedge_d;
        *var_xnedge_d_dn5_slot = var_xnedge_d_dn5;
        *var_xnedge_d_dn6_slot = var_xnedge_d_dn6;
        *var_xnedge_d_dn7_slot = var_xnedge_d_dn7;
        *var_xnedge_d_dn8_slot = var_xnedge_d_dn8;
        *var_xnedge_d_rv_slot = var_xnedge_d_rv;
    }

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        var_a1_i: f64,
        var_a2_t: f64,
        var_a3_i: f64,
        var_a4_i: f64,
        var_aphi_ac: f64,
        var_aphi_dc: f64,
        var_betedge_i: f64,
        var_bphi_ac: f64,
        var_dps_dc: f64,
        var_dps_dc_dn5: f64,
        var_dps_dc_dn6: f64,
        var_dps_dc_dn7: f64,
        var_dps_dc_dn8: f64,
        var_g_0_dc: f64,
        var_gfedge: f64,
        var_gfedge2: f64,
        var_gmob_dc: f64,
        var_gmob_dc_dn5: f64,
        var_gmob_dc_dn6: f64,
        var_gmob_dc_dn7: f64,
        var_gmob_dc_dn8: f64,
        var_guard1249: f64,
        var_guard1253: f64,
        var_guard1255: f64,
        var_i_ds: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_imaxii_i: f64,
        var_lngfedge2: f64,
        var_phib_dc: f64,
        var_phit1edge: f64,
        var_phit1edge_dn5: f64,
        var_phit1edge_dn6: f64,
        var_phit1edge_dn7: f64,
        var_phit1edge_dn8: f64,
        var_phix1_ac: f64,
        var_phix_ac: f64,
        var_q_edge_n_inv: f64,
        var_q_edge_n_inv_dn5: f64,
        var_q_edge_n_inv_dn6: f64,
        var_q_edge_n_inv_dn7: f64,
        var_q_edge_n_inv_dn8: f64,
        var_q_edge_xgt: f64,
        var_q_edge_xgt_dn5: f64,
        var_q_edge_xgt_dn6: f64,
        var_q_edge_xgt_dn7: f64,
        var_q_edge_xgt_dn8: f64,
        var_qseffedge: f64,
        var_qseffedge_dn5: f64,
        var_qseffedge_dn6: f64,
        var_qseffedge_dn7: f64,
        var_qseffedge_dn8: f64,
        var_sqrt_phib_dc: f64,
        var_v_db: f64,
        var_v_db_dn6: f64,
        var_v_db_dn7: f64,
        var_v_db_dn8: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_v_sb: f64,
        var_v_sb_dn6: f64,
        var_v_sb_dn7: f64,
        var_v_sb_dn8: f64,
        var_v_xb_dc_tmp: f64,
        var_v_xb_dc_tmp_dn6: f64,
        var_v_xb_dc_tmp_dn7: f64,
        var_v_xb_dc_tmp_dn8: f64,
        var_vsbstar_dc: f64,
        var_vsbstar_dc_dn5: f64,
        var_vsbstar_dc_dn6: f64,
        var_vsbstar_dc_dn7: f64,
        var_vsbstar_dc_dn8: f64,
        var_vsbstar_dc_tmp: f64,
        var_vsbstar_dc_tmp_dn5: f64,
        var_vsbstar_dc_tmp_dn6: f64,
        var_vsbstar_dc_tmp_dn7: f64,
        var_vsbstar_dc_tmp_dn8: f64,
        var_xg_dc: f64,
        var_xgedge: f64,
        var_xgedge_dn5: f64,
        var_xgedge_dn6: f64,
        var_xgedge_dn7: f64,
        var_xgedge_dn8: f64,
        var_alphabmedge_slot: &mut f64,
        var_alphabmedge_dn5_slot: &mut f64,
        var_alphabmedge_dn6_slot: &mut f64,
        var_alphabmedge_dn7_slot: &mut f64,
        var_alphabmedge_dn8_slot: &mut f64,
        var_alphabmedge_rv_slot: &mut f64,
        var_aphi__blk1298_slot: &mut f64,
        var_aphi__blk1298_rv_slot: &mut f64,
        var_delvsat_slot: &mut f64,
        var_delvsat_dn5_slot: &mut f64,
        var_delvsat_dn6_slot: &mut f64,
        var_delvsat_dn7_slot: &mut f64,
        var_delvsat_dn8_slot: &mut f64,
        var_delvsat_rv_slot: &mut f64,
        var_dsqredge_slot: &mut f64,
        var_dsqredge_dn5_slot: &mut f64,
        var_dsqredge_dn6_slot: &mut f64,
        var_dsqredge_dn7_slot: &mut f64,
        var_dsqredge_dn8_slot: &mut f64,
        var_dsqredge_rv_slot: &mut f64,
        var_dvbstar__blk1305_slot: &mut f64,
        var_dvbstar__blk1305_rv_slot: &mut f64,
        var_g_0__blk1299_slot: &mut f64,
        var_g_0__blk1299_rv_slot: &mut f64,
        var_guard1257_slot: &mut f64,
        var_guard1257_rv_slot: &mut f64,
        var_guard1258_slot: &mut f64,
        var_guard1258_rv_slot: &mut f64,
        var_guard1259_slot: &mut f64,
        var_guard1259_rv_slot: &mut f64,
        var_guard1260_slot: &mut f64,
        var_guard1260_rv_slot: &mut f64,
        var_guard1261_slot: &mut f64,
        var_guard1261_rv_slot: &mut f64,
        var_guard1262_slot: &mut f64,
        var_guard1262_rv_slot: &mut f64,
        var_guard1456_slot: &mut f64,
        var_guard1456_rv_slot: &mut f64,
        var_guard1457_slot: &mut f64,
        var_guard1457_rv_slot: &mut f64,
        var_guard1458_slot: &mut f64,
        var_guard1458_rv_slot: &mut f64,
        var_i_dsedge_slot: &mut f64,
        var_i_dsedge_dn5_slot: &mut f64,
        var_i_dsedge_dn6_slot: &mut f64,
        var_i_dsedge_dn7_slot: &mut f64,
        var_i_dsedge_dn8_slot: &mut f64,
        var_i_dsedge_rv_slot: &mut f64,
        var_iimpact_slot: &mut f64,
        var_iimpact_dn5_slot: &mut f64,
        var_iimpact_dn6_slot: &mut f64,
        var_iimpact_dn7_slot: &mut f64,
        var_iimpact_dn8_slot: &mut f64,
        var_iimpact_rv_slot: &mut f64,
        var_mavl_slot: &mut f64,
        var_mavl_dn5_slot: &mut f64,
        var_mavl_dn6_slot: &mut f64,
        var_mavl_dn7_slot: &mut f64,
        var_mavl_dn8_slot: &mut f64,
        var_mavl_rv_slot: &mut f64,
        var_phib__blk1297_slot: &mut f64,
        var_phib__blk1297_rv_slot: &mut f64,
        var_qdeffedge_slot: &mut f64,
        var_qdeffedge_dn5_slot: &mut f64,
        var_qdeffedge_dn6_slot: &mut f64,
        var_qdeffedge_dn7_slot: &mut f64,
        var_qdeffedge_dn8_slot: &mut f64,
        var_qdeffedge_rv_slot: &mut f64,
        var_qdseffedge_slot: &mut f64,
        var_qdseffedge_dn5_slot: &mut f64,
        var_qdseffedge_dn6_slot: &mut f64,
        var_qdseffedge_dn7_slot: &mut f64,
        var_qdseffedge_dn8_slot: &mut f64,
        var_qdseffedge_rv_slot: &mut f64,
        var_qmeffedge_slot: &mut f64,
        var_qmeffedge_dn5_slot: &mut f64,
        var_qmeffedge_dn6_slot: &mut f64,
        var_qmeffedge_dn7_slot: &mut f64,
        var_qmeffedge_dn8_slot: &mut f64,
        var_qmeffedge_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_v_xb__blk1300_slot: &mut f64,
        var_v_xb__blk1300_dn6_slot: &mut f64,
        var_v_xb__blk1300_dn7_slot: &mut f64,
        var_v_xb__blk1300_dn8_slot: &mut f64,
        var_v_xb__blk1300_rv_slot: &mut f64,
        var_vsbstar__blk1301_slot: &mut f64,
        var_vsbstar__blk1301_dn5_slot: &mut f64,
        var_vsbstar__blk1301_dn6_slot: &mut f64,
        var_vsbstar__blk1301_dn7_slot: &mut f64,
        var_vsbstar__blk1301_dn8_slot: &mut f64,
        var_vsbstar__blk1301_rv_slot: &mut f64,
        var_vsbstar_ac_slot: &mut f64,
        var_vsbstar_ac_dn6_slot: &mut f64,
        var_vsbstar_ac_dn7_slot: &mut f64,
        var_vsbstar_ac_dn8_slot: &mut f64,
        var_vsbstar_ac_rv_slot: &mut f64,
    ) {
        let mut var_alphabmedge: f64 = *var_alphabmedge_slot;
        let mut var_alphabmedge_dn5: f64 = *var_alphabmedge_dn5_slot;
        let mut var_alphabmedge_dn6: f64 = *var_alphabmedge_dn6_slot;
        let mut var_alphabmedge_dn7: f64 = *var_alphabmedge_dn7_slot;
        let mut var_alphabmedge_dn8: f64 = *var_alphabmedge_dn8_slot;
        let mut var_alphabmedge_rv: f64 = *var_alphabmedge_rv_slot;
        let mut var_aphi__blk1298: f64 = *var_aphi__blk1298_slot;
        let mut var_aphi__blk1298_rv: f64 = *var_aphi__blk1298_rv_slot;
        let mut var_delvsat: f64 = *var_delvsat_slot;
        let mut var_delvsat_dn5: f64 = *var_delvsat_dn5_slot;
        let mut var_delvsat_dn6: f64 = *var_delvsat_dn6_slot;
        let mut var_delvsat_dn7: f64 = *var_delvsat_dn7_slot;
        let mut var_delvsat_dn8: f64 = *var_delvsat_dn8_slot;
        let mut var_delvsat_rv: f64 = *var_delvsat_rv_slot;
        let mut var_dsqredge: f64 = *var_dsqredge_slot;
        let mut var_dsqredge_dn5: f64 = *var_dsqredge_dn5_slot;
        let mut var_dsqredge_dn6: f64 = *var_dsqredge_dn6_slot;
        let mut var_dsqredge_dn7: f64 = *var_dsqredge_dn7_slot;
        let mut var_dsqredge_dn8: f64 = *var_dsqredge_dn8_slot;
        let mut var_dsqredge_rv: f64 = *var_dsqredge_rv_slot;
        let mut var_dvbstar__blk1305: f64 = *var_dvbstar__blk1305_slot;
        let mut var_dvbstar__blk1305_rv: f64 = *var_dvbstar__blk1305_rv_slot;
        let mut var_g_0__blk1299: f64 = *var_g_0__blk1299_slot;
        let mut var_g_0__blk1299_rv: f64 = *var_g_0__blk1299_rv_slot;
        let mut var_guard1257: f64 = *var_guard1257_slot;
        let mut var_guard1257_rv: f64 = *var_guard1257_rv_slot;
        let mut var_guard1258: f64 = *var_guard1258_slot;
        let mut var_guard1258_rv: f64 = *var_guard1258_rv_slot;
        let mut var_guard1259: f64 = *var_guard1259_slot;
        let mut var_guard1259_rv: f64 = *var_guard1259_rv_slot;
        let mut var_guard1260: f64 = *var_guard1260_slot;
        let mut var_guard1260_rv: f64 = *var_guard1260_rv_slot;
        let mut var_guard1261: f64 = *var_guard1261_slot;
        let mut var_guard1261_rv: f64 = *var_guard1261_rv_slot;
        let mut var_guard1262: f64 = *var_guard1262_slot;
        let mut var_guard1262_rv: f64 = *var_guard1262_rv_slot;
        let mut var_guard1456: f64 = *var_guard1456_slot;
        let mut var_guard1456_rv: f64 = *var_guard1456_rv_slot;
        let mut var_guard1457: f64 = *var_guard1457_slot;
        let mut var_guard1457_rv: f64 = *var_guard1457_rv_slot;
        let mut var_guard1458: f64 = *var_guard1458_slot;
        let mut var_guard1458_rv: f64 = *var_guard1458_rv_slot;
        let mut var_i_dsedge: f64 = *var_i_dsedge_slot;
        let mut var_i_dsedge_dn5: f64 = *var_i_dsedge_dn5_slot;
        let mut var_i_dsedge_dn6: f64 = *var_i_dsedge_dn6_slot;
        let mut var_i_dsedge_dn7: f64 = *var_i_dsedge_dn7_slot;
        let mut var_i_dsedge_dn8: f64 = *var_i_dsedge_dn8_slot;
        let mut var_i_dsedge_rv: f64 = *var_i_dsedge_rv_slot;
        let mut var_iimpact: f64 = *var_iimpact_slot;
        let mut var_iimpact_dn5: f64 = *var_iimpact_dn5_slot;
        let mut var_iimpact_dn6: f64 = *var_iimpact_dn6_slot;
        let mut var_iimpact_dn7: f64 = *var_iimpact_dn7_slot;
        let mut var_iimpact_dn8: f64 = *var_iimpact_dn8_slot;
        let mut var_iimpact_rv: f64 = *var_iimpact_rv_slot;
        let mut var_mavl: f64 = *var_mavl_slot;
        let mut var_mavl_dn5: f64 = *var_mavl_dn5_slot;
        let mut var_mavl_dn6: f64 = *var_mavl_dn6_slot;
        let mut var_mavl_dn7: f64 = *var_mavl_dn7_slot;
        let mut var_mavl_dn8: f64 = *var_mavl_dn8_slot;
        let mut var_mavl_rv: f64 = *var_mavl_rv_slot;
        let mut var_phib__blk1297: f64 = *var_phib__blk1297_slot;
        let mut var_phib__blk1297_rv: f64 = *var_phib__blk1297_rv_slot;
        let mut var_qdeffedge: f64 = *var_qdeffedge_slot;
        let mut var_qdeffedge_dn5: f64 = *var_qdeffedge_dn5_slot;
        let mut var_qdeffedge_dn6: f64 = *var_qdeffedge_dn6_slot;
        let mut var_qdeffedge_dn7: f64 = *var_qdeffedge_dn7_slot;
        let mut var_qdeffedge_dn8: f64 = *var_qdeffedge_dn8_slot;
        let mut var_qdeffedge_rv: f64 = *var_qdeffedge_rv_slot;
        let mut var_qdseffedge: f64 = *var_qdseffedge_slot;
        let mut var_qdseffedge_dn5: f64 = *var_qdseffedge_dn5_slot;
        let mut var_qdseffedge_dn6: f64 = *var_qdseffedge_dn6_slot;
        let mut var_qdseffedge_dn7: f64 = *var_qdseffedge_dn7_slot;
        let mut var_qdseffedge_dn8: f64 = *var_qdseffedge_dn8_slot;
        let mut var_qdseffedge_rv: f64 = *var_qdseffedge_rv_slot;
        let mut var_qmeffedge: f64 = *var_qmeffedge_slot;
        let mut var_qmeffedge_dn5: f64 = *var_qmeffedge_dn5_slot;
        let mut var_qmeffedge_dn6: f64 = *var_qmeffedge_dn6_slot;
        let mut var_qmeffedge_dn7: f64 = *var_qmeffedge_dn7_slot;
        let mut var_qmeffedge_dn8: f64 = *var_qmeffedge_dn8_slot;
        let mut var_qmeffedge_rv: f64 = *var_qmeffedge_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_v_xb__blk1300: f64 = *var_v_xb__blk1300_slot;
        let mut var_v_xb__blk1300_dn6: f64 = *var_v_xb__blk1300_dn6_slot;
        let mut var_v_xb__blk1300_dn7: f64 = *var_v_xb__blk1300_dn7_slot;
        let mut var_v_xb__blk1300_dn8: f64 = *var_v_xb__blk1300_dn8_slot;
        let mut var_v_xb__blk1300_rv: f64 = *var_v_xb__blk1300_rv_slot;
        let mut var_vsbstar__blk1301: f64 = *var_vsbstar__blk1301_slot;
        let mut var_vsbstar__blk1301_dn5: f64 = *var_vsbstar__blk1301_dn5_slot;
        let mut var_vsbstar__blk1301_dn6: f64 = *var_vsbstar__blk1301_dn6_slot;
        let mut var_vsbstar__blk1301_dn7: f64 = *var_vsbstar__blk1301_dn7_slot;
        let mut var_vsbstar__blk1301_dn8: f64 = *var_vsbstar__blk1301_dn8_slot;
        let mut var_vsbstar__blk1301_rv: f64 = *var_vsbstar__blk1301_rv_slot;
        let mut var_vsbstar_ac: f64 = *var_vsbstar_ac_slot;
        let mut var_vsbstar_ac_dn6: f64 = *var_vsbstar_ac_dn6_slot;
        let mut var_vsbstar_ac_dn7: f64 = *var_vsbstar_ac_dn7_slot;
        let mut var_vsbstar_ac_dn8: f64 = *var_vsbstar_ac_dn8_slot;
        let mut var_vsbstar_ac_rv: f64 = *var_vsbstar_ac_rv_slot;

        let assign47940_e61489: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47940_e61490: f64 = (var_q_edge_n_inv * assign47940_e61489);
        let assign47940_e61492: f64 = (-230.25850929940458);
        let assign47940_e61493: f64 = if assign47940_e61490 > assign47940_e61492 { 1.0 } else { 0.0 };
        var_guard1257 = assign47940_e61493;
        var_guard1257_rv = 0.0;

        let (assign47950_e61510, assign47950_e61510_d_n5, assign47950_e61510_d_n6, assign47950_e61510_d_n7, assign47950_e61510_d_n8,) = {
    if ((((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 == 0.0)) && (var_guard1257 != 0.0)) {
        let assign47950_e61506: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47950_e61507: f64 = (var_q_edge_n_inv * assign47950_e61506);
        let assign47950_e61508: f64 = (assign47950_e61507).exp();
        (assign47950_e61508, (assign47950_e61508 * ((var_q_edge_n_inv_dn5 * assign47950_e61506) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))), (assign47950_e61508 * ((var_q_edge_n_inv_dn6 * assign47950_e61506) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))), (assign47950_e61508 * ((var_q_edge_n_inv_dn7 * assign47950_e61506) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))), (assign47950_e61508 * ((var_q_edge_n_inv_dn8 * assign47950_e61506) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))),)
    } else {
        (var_qdeffedge, var_qdeffedge_dn5, var_qdeffedge_dn6, var_qdeffedge_dn7, var_qdeffedge_dn8,)
    }
};
        var_qdeffedge = assign47950_e61510;
        var_qdeffedge_dn5 = assign47950_e61510_d_n5;
        var_qdeffedge_dn6 = assign47950_e61510_d_n6;
        var_qdeffedge_dn7 = assign47950_e61510_d_n7;
        var_qdeffedge_dn8 = assign47950_e61510_d_n8;
        var_qdeffedge_rv = 0.0;

        let (assign47960_e61560, assign47960_e61560_d_n5, assign47960_e61560_d_n6, assign47960_e61560_d_n7, assign47960_e61560_d_n8,) = {
    if ((((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) && (var_guard1255 == 0.0)) && (var_guard1257 == 0.0)) {
        let assign47960_e61524: f64 = (-230.25850929940458);
        let assign47960_e61528: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47960_e61529: f64 = (var_q_edge_n_inv * assign47960_e61528);
        let assign47960_e61530: f64 = (assign47960_e61524 - assign47960_e61529);
        let assign47960_e61534: f64 = (-230.25850929940458);
        let assign47960_e61538: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47960_e61539: f64 = (var_q_edge_n_inv * assign47960_e61538);
        let assign47960_e61540: f64 = (assign47960_e61534 - assign47960_e61539);
        let assign47960_e61543: f64 = (-230.25850929940458);
        let assign47960_e61547: f64 = (var_q_edge_xgt + var_lngfedge2);
        let assign47960_e61548: f64 = (var_q_edge_n_inv * assign47960_e61547);
        let assign47960_e61549: f64 = (assign47960_e61543 - assign47960_e61548);
        let assign47960_e61551: f64 = (assign47960_e61549 * 0.3333333333333333);
        let assign47960_e61552: f64 = (1.0 + assign47960_e61551);
        let assign47960_e61553: f64 = (assign47960_e61540 * assign47960_e61552);
        let assign47960_e61554: f64 = (0.5 * assign47960_e61553);
        let assign47960_e61555: f64 = (1.0 + assign47960_e61554);
        let assign47960_e61556: f64 = (assign47960_e61530 * assign47960_e61555);
        let assign47960_e61557: f64 = (1.0 + assign47960_e61556);
        let assign47960_e61558: f64 = (1e-100 / assign47960_e61557);
        (assign47960_e61558, (-((1e-100 * (((-((var_q_edge_n_inv_dn5 * assign47960_e61528) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((var_q_edge_n_inv_dn5 * assign47960_e61538) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * assign47960_e61552) + (assign47960_e61540 * ((-((var_q_edge_n_inv_dn5 * assign47960_e61547) + (var_q_edge_n_inv * var_q_edge_xgt_dn5))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((var_q_edge_n_inv_dn6 * assign47960_e61528) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((var_q_edge_n_inv_dn6 * assign47960_e61538) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * assign47960_e61552) + (assign47960_e61540 * ((-((var_q_edge_n_inv_dn6 * assign47960_e61547) + (var_q_edge_n_inv * var_q_edge_xgt_dn6))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((var_q_edge_n_inv_dn7 * assign47960_e61528) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((var_q_edge_n_inv_dn7 * assign47960_e61538) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * assign47960_e61552) + (assign47960_e61540 * ((-((var_q_edge_n_inv_dn7 * assign47960_e61547) + (var_q_edge_n_inv * var_q_edge_xgt_dn7))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))), (-((1e-100 * (((-((var_q_edge_n_inv_dn8 * assign47960_e61528) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * assign47960_e61555) + (assign47960_e61530 * (0.5 * (((-((var_q_edge_n_inv_dn8 * assign47960_e61538) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * assign47960_e61552) + (assign47960_e61540 * ((-((var_q_edge_n_inv_dn8 * assign47960_e61547) + (var_q_edge_n_inv * var_q_edge_xgt_dn8))) * 0.3333333333333333))))))) / (assign47960_e61557 * assign47960_e61557))),)
    } else {
        (var_qdeffedge, var_qdeffedge_dn5, var_qdeffedge_dn6, var_qdeffedge_dn7, var_qdeffedge_dn8,)
    }
};
        var_qdeffedge = assign47960_e61560;
        var_qdeffedge_dn5 = assign47960_e61560_d_n5;
        var_qdeffedge_dn6 = assign47960_e61560_d_n6;
        var_qdeffedge_dn7 = assign47960_e61560_d_n7;
        var_qdeffedge_dn8 = assign47960_e61560_d_n8;
        var_qdeffedge_rv = 0.0;

        let (assign47970_e61569, assign47970_e61569_d_n5, assign47970_e61569_d_n6, assign47970_e61569_d_n7, assign47970_e61569_d_n8,) = {
    if ((var_guard1249 != 0.0) && (var_guard1253 == 0.0)) {
        let assign47970_e61567: f64 = (var_qdeffedge - var_qseffedge);
        (assign47970_e61567, (var_qdeffedge_dn5 - var_qseffedge_dn5), (var_qdeffedge_dn6 - var_qseffedge_dn6), (var_qdeffedge_dn7 - var_qseffedge_dn7), (var_qdeffedge_dn8 - var_qseffedge_dn8),)
    } else {
        (var_qdseffedge, var_qdseffedge_dn5, var_qdseffedge_dn6, var_qdseffedge_dn7, var_qdseffedge_dn8,)
    }
};
        var_qdseffedge = assign47970_e61569;
        var_qdseffedge_dn5 = assign47970_e61569_d_n5;
        var_qdseffedge_dn6 = assign47970_e61569_d_n6;
        var_qdseffedge_dn7 = assign47970_e61569_d_n7;
        var_qdseffedge_dn8 = assign47970_e61569_d_n8;
        var_qdseffedge_rv = 0.0;

        let (assign47980_e61577, assign47980_e61577_d_n5, assign47980_e61577_d_n6, assign47980_e61577_d_n7, assign47980_e61577_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47980_e61574: f64 = (var_qdeffedge + var_qseffedge);
        let assign47980_e61575: f64 = (0.5 * assign47980_e61574);
        (assign47980_e61575, (0.5 * (var_qdeffedge_dn5 + var_qseffedge_dn5)), (0.5 * (var_qdeffedge_dn6 + var_qseffedge_dn6)), (0.5 * (var_qdeffedge_dn7 + var_qseffedge_dn7)), (0.5 * (var_qdeffedge_dn8 + var_qseffedge_dn8)),)
    } else {
        (var_qmeffedge, var_qmeffedge_dn5, var_qmeffedge_dn6, var_qmeffedge_dn7, var_qmeffedge_dn8,)
    }
};
        var_qmeffedge = assign47980_e61577;
        var_qmeffedge_dn5 = assign47980_e61577_d_n5;
        var_qmeffedge_dn6 = assign47980_e61577_d_n6;
        var_qmeffedge_dn7 = assign47980_e61577_d_n7;
        var_qmeffedge_dn8 = assign47980_e61577_d_n8;
        var_qmeffedge_rv = 0.0;

        let (assign47990_e61590, assign47990_e61590_d_n5, assign47990_e61590_d_n6, assign47990_e61590_d_n7, assign47990_e61590_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign47990_e61581: f64 = (var_xgedge - var_qmeffedge);
        let (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,) = {
            if (assign47990_e61581 > 1e-40) {
                let assign47990_e61586: f64 = (var_xgedge - var_qmeffedge);
                (assign47990_e61586, (var_xgedge_dn5 - var_qmeffedge_dn5), (var_xgedge_dn6 - var_qmeffedge_dn6), (var_xgedge_dn7 - var_qmeffedge_dn7), (var_xgedge_dn8 - var_qmeffedge_dn8),)
            } else {
                (1e-40, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign47990_e61588, assign47990_e61588_d_n5, assign47990_e61588_d_n6, assign47990_e61588_d_n7, assign47990_e61588_d_n8,)
    } else {
        (var_dsqredge, var_dsqredge_dn5, var_dsqredge_dn6, var_dsqredge_dn7, var_dsqredge_dn8,)
    }
};
        var_dsqredge = assign47990_e61590;
        var_dsqredge_dn5 = assign47990_e61590_d_n5;
        var_dsqredge_dn6 = assign47990_e61590_d_n6;
        var_dsqredge_dn7 = assign47990_e61590_d_n7;
        var_dsqredge_dn8 = assign47990_e61590_d_n8;
        var_dsqredge_rv = 0.0;

        let (assign48000_e61605, assign48000_e61605_d_n5, assign48000_e61605_d_n6, assign48000_e61605_d_n7, assign48000_e61605_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign48000_e61595: f64 = (0.5 * var_gfedge);
        let assign48000_e61599: f64 = (0.25 * var_gfedge2);
        let assign48000_e61600: f64 = (var_dsqredge + assign48000_e61599);
        let assign48000_e61601: f64 = (assign48000_e61600).sqrt();
        let assign48000_e61602: f64 = (assign48000_e61595 / assign48000_e61601);
        let assign48000_e61603: f64 = (1.0 - assign48000_e61602);
        (assign48000_e61603, (-(-((assign48000_e61595 * (var_dsqredge_dn5 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (var_dsqredge_dn6 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (var_dsqredge_dn7 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))), (-(-((assign48000_e61595 * (var_dsqredge_dn8 / (2.0 * assign48000_e61601))) / (assign48000_e61601 * assign48000_e61601)))),)
    } else {
        (var_alphabmedge, var_alphabmedge_dn5, var_alphabmedge_dn6, var_alphabmedge_dn7, var_alphabmedge_dn8,)
    }
};
        var_alphabmedge = assign48000_e61605;
        var_alphabmedge_dn5 = assign48000_e61605_d_n5;
        var_alphabmedge_dn6 = assign48000_e61605_d_n6;
        var_alphabmedge_dn7 = assign48000_e61605_d_n7;
        var_alphabmedge_dn8 = assign48000_e61605_d_n8;
        var_alphabmedge_rv = 0.0;

        let (assign48010_e61624, assign48010_e61624_d_n5, assign48010_e61624_d_n6, assign48010_e61624_d_n7, assign48010_e61624_d_n8,) = {
    if (var_guard1249 != 0.0) {
        let assign48010_e61608: f64 = (-var_betedge_i);
        let assign48010_e61610: f64 = (assign48010_e61608 * var_phit1edge);
        let assign48010_e61612: f64 = (assign48010_e61610 * var_phit1edge);
        let assign48010_e61615: f64 = (var_alphabmedge * var_qmeffedge);
        let assign48010_e61617: f64 = (assign48010_e61615 + 1.0);
        let assign48010_e61618: f64 = (assign48010_e61612 * assign48010_e61617);
        let assign48010_e61620: f64 = (assign48010_e61618 * var_qdseffedge);
        let assign48010_e61622: f64 = (assign48010_e61620 / var_gmob_dc);
        (assign48010_e61622, ((((((((((assign48010_e61608 * var_phit1edge_dn5) * var_phit1edge) + (assign48010_e61610 * var_phit1edge_dn5)) * assign48010_e61617) + (assign48010_e61612 * ((var_alphabmedge_dn5 * var_qmeffedge) + (var_alphabmedge * var_qmeffedge_dn5)))) * var_qdseffedge) + (assign48010_e61618 * var_qdseffedge_dn5)) * var_gmob_dc) - (assign48010_e61620 * var_gmob_dc_dn5)) / (var_gmob_dc * var_gmob_dc)), ((((((((((assign48010_e61608 * var_phit1edge_dn6) * var_phit1edge) + (assign48010_e61610 * var_phit1edge_dn6)) * assign48010_e61617) + (assign48010_e61612 * ((var_alphabmedge_dn6 * var_qmeffedge) + (var_alphabmedge * var_qmeffedge_dn6)))) * var_qdseffedge) + (assign48010_e61618 * var_qdseffedge_dn6)) * var_gmob_dc) - (assign48010_e61620 * var_gmob_dc_dn6)) / (var_gmob_dc * var_gmob_dc)), ((((((((((assign48010_e61608 * var_phit1edge_dn7) * var_phit1edge) + (assign48010_e61610 * var_phit1edge_dn7)) * assign48010_e61617) + (assign48010_e61612 * ((var_alphabmedge_dn7 * var_qmeffedge) + (var_alphabmedge * var_qmeffedge_dn7)))) * var_qdseffedge) + (assign48010_e61618 * var_qdseffedge_dn7)) * var_gmob_dc) - (assign48010_e61620 * var_gmob_dc_dn7)) / (var_gmob_dc * var_gmob_dc)), ((((((((((assign48010_e61608 * var_phit1edge_dn8) * var_phit1edge) + (assign48010_e61610 * var_phit1edge_dn8)) * assign48010_e61617) + (assign48010_e61612 * ((var_alphabmedge_dn8 * var_qmeffedge) + (var_alphabmedge * var_qmeffedge_dn8)))) * var_qdseffedge) + (assign48010_e61618 * var_qdseffedge_dn8)) * var_gmob_dc) - (assign48010_e61620 * var_gmob_dc_dn8)) / (var_gmob_dc * var_gmob_dc)),)
    } else {
        (var_i_dsedge, var_i_dsedge_dn5, var_i_dsedge_dn6, var_i_dsedge_dn7, var_i_dsedge_dn8,)
    }
};
        var_i_dsedge = assign48010_e61624;
        var_i_dsedge_dn5 = assign48010_e61624_d_n5;
        var_i_dsedge_dn6 = assign48010_e61624_d_n6;
        var_i_dsedge_dn7 = assign48010_e61624_d_n7;
        var_i_dsedge_dn8 = assign48010_e61624_d_n8;
        var_i_dsedge_rv = 0.0;

        var_mavl = 0.0;
        var_mavl_dn5 = 0.0;
        var_mavl_dn6 = 0.0;
        var_mavl_dn7 = 0.0;
        var_mavl_dn8 = 0.0;
        var_mavl_rv = 0.0;

        var_iimpact = 0.0;
        var_iimpact_dn5 = 0.0;
        var_iimpact_dn6 = 0.0;
        var_iimpact_dn7 = 0.0;
        var_iimpact_dn8 = 0.0;
        var_iimpact_rv = 0.0;

        let assign48040_e61633: f64 = if ((var_xg_dc > 0.0) && (p.p41 != 0.0)) { 1.0 } else { 0.0 };
        var_guard1258 = assign48040_e61633;
        var_guard1258_rv = 0.0;

        let (assign48050_e61641, assign48050_e61641_d_n5, assign48050_e61641_d_n6, assign48050_e61641_d_n7, assign48050_e61641_d_n8,) = {
    if (var_guard1258 != 0.0) {
        let assign48050_e61638: f64 = (var_a3_i * var_dps_dc);
        let assign48050_e61639: f64 = (var_v_ds - assign48050_e61638);
        (assign48050_e61639, (-(var_a3_i * var_dps_dc_dn5)), (var_v_ds_dn6 - (var_a3_i * var_dps_dc_dn6)), (var_v_ds_dn7 - (var_a3_i * var_dps_dc_dn7)), (-(var_a3_i * var_dps_dc_dn8)),)
    } else {
        (var_delvsat, var_delvsat_dn5, var_delvsat_dn6, var_delvsat_dn7, var_delvsat_dn8,)
    }
};
        var_delvsat = assign48050_e61641;
        var_delvsat_dn5 = assign48050_e61641_d_n5;
        var_delvsat_dn6 = assign48050_e61641_d_n6;
        var_delvsat_dn7 = assign48050_e61641_d_n7;
        var_delvsat_dn8 = assign48050_e61641_d_n8;
        var_delvsat_rv = 0.0;

        let assign48060_e61644: f64 = if var_delvsat > 0.0 { 1.0 } else { 0.0 };
        var_guard1259 = assign48060_e61644;
        var_guard1259_rv = 0.0;

        let (assign48070_e61665, assign48070_e61665_d_n5, assign48070_e61665_d_n6, assign48070_e61665_d_n7, assign48070_e61665_d_n8,) = {
    if ((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) {
        let assign48070_e61653: f64 = (var_phib_dc + var_vsbstar_dc);
        let assign48070_e61654: f64 = (assign48070_e61653).sqrt();
        let assign48070_e61656: f64 = (assign48070_e61654 - var_sqrt_phib_dc);
        let assign48070_e61657: f64 = (var_a4_i * assign48070_e61656);
        let assign48070_e61658: f64 = (1.0 + assign48070_e61657);
        let assign48070_e61661: f64 = (var_delvsat + 1e-30);
        let assign48070_e61662: f64 = (assign48070_e61658 / assign48070_e61661);
        let assign48070_e61663: f64 = (var_a2_t * assign48070_e61662);
        (assign48070_e61663, (var_a2_t * ((((var_a4_i * (var_vsbstar_dc_dn5 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * var_delvsat_dn5)) / (assign48070_e61661 * assign48070_e61661))), (var_a2_t * ((((var_a4_i * (var_vsbstar_dc_dn6 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * var_delvsat_dn6)) / (assign48070_e61661 * assign48070_e61661))), (var_a2_t * ((((var_a4_i * (var_vsbstar_dc_dn7 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * var_delvsat_dn7)) / (assign48070_e61661 * assign48070_e61661))), (var_a2_t * ((((var_a4_i * (var_vsbstar_dc_dn8 / (2.0 * assign48070_e61654))) * assign48070_e61661) - (assign48070_e61658 * var_delvsat_dn8)) / (assign48070_e61661 * assign48070_e61661))),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign48070_e61665;
        var_temp2_dn5 = assign48070_e61665_d_n5;
        var_temp2_dn6 = assign48070_e61665_d_n6;
        var_temp2_dn7 = assign48070_e61665_d_n7;
        var_temp2_dn8 = assign48070_e61665_d_n8;
        var_temp2_rv = 0.0;

        let assign48080_e61667: f64 = (-var_temp2);
        let assign48080_e61668: f64 = (assign48080_e61667).abs();
        let assign48080_e61670: f64 = if assign48080_e61668 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1260 = assign48080_e61670;
        var_guard1260_rv = 0.0;

        let (assign48090_e61680, assign48090_e61680_d_n5, assign48090_e61680_d_n6, assign48090_e61680_d_n7, assign48090_e61680_d_n8,) = {
    if (((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) && (var_guard1260 != 0.0)) {
        let assign48090_e61677: f64 = (-var_temp2);
        let assign48090_e61678: f64 = (assign48090_e61677).exp();
        (assign48090_e61678, (assign48090_e61678 * (-var_temp2_dn5)), (assign48090_e61678 * (-var_temp2_dn6)), (assign48090_e61678 * (-var_temp2_dn7)), (assign48090_e61678 * (-var_temp2_dn8)),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48090_e61680;
        var_temp__blk936_dn5 = assign48090_e61680_d_n5;
        var_temp__blk936_dn6 = assign48090_e61680_d_n6;
        var_temp__blk936_dn7 = assign48090_e61680_d_n7;
        var_temp__blk936_dn8 = assign48090_e61680_d_n8;
        var_temp__blk936_rv = 0.0;

        let assign48100_e61682: f64 = (-var_temp2);
        let assign48100_e61684: f64 = if assign48100_e61682 < 0.0 { 1.0 } else { 0.0 };
        var_guard1261 = assign48100_e61684;
        var_guard1261_rv = 0.0;

        let (assign48110_e61723, assign48110_e61723_d_n5, assign48110_e61723_d_n6, assign48110_e61723_d_n7, assign48110_e61723_d_n8,) = {
    if ((((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) && (var_guard1260 == 0.0)) && (var_guard1261 != 0.0)) {
        let assign48110_e61696: f64 = (-230.25850929940458);
        let assign48110_e61698: f64 = (-var_temp2);
        let assign48110_e61699: f64 = (assign48110_e61696 - assign48110_e61698);
        let assign48110_e61703: f64 = (-230.25850929940458);
        let assign48110_e61705: f64 = (-var_temp2);
        let assign48110_e61706: f64 = (assign48110_e61703 - assign48110_e61705);
        let assign48110_e61709: f64 = (-230.25850929940458);
        let assign48110_e61711: f64 = (-var_temp2);
        let assign48110_e61712: f64 = (assign48110_e61709 - assign48110_e61711);
        let assign48110_e61714: f64 = (assign48110_e61712 * 0.3333333333333333);
        let assign48110_e61715: f64 = (1.0 + assign48110_e61714);
        let assign48110_e61716: f64 = (assign48110_e61706 * assign48110_e61715);
        let assign48110_e61717: f64 = (0.5 * assign48110_e61716);
        let assign48110_e61718: f64 = (1.0 + assign48110_e61717);
        let assign48110_e61719: f64 = (assign48110_e61699 * assign48110_e61718);
        let assign48110_e61720: f64 = (1.0 + assign48110_e61719);
        let assign48110_e61721: f64 = (1e-100 / assign48110_e61720);
        (assign48110_e61721, (-((1e-100 * (((-(-var_temp2_dn5)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-var_temp2_dn5)) * assign48110_e61715) + (assign48110_e61706 * ((-(-var_temp2_dn5)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-var_temp2_dn6)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-var_temp2_dn6)) * assign48110_e61715) + (assign48110_e61706 * ((-(-var_temp2_dn6)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-var_temp2_dn7)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-var_temp2_dn7)) * assign48110_e61715) + (assign48110_e61706 * ((-(-var_temp2_dn7)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))), (-((1e-100 * (((-(-var_temp2_dn8)) * assign48110_e61718) + (assign48110_e61699 * (0.5 * (((-(-var_temp2_dn8)) * assign48110_e61715) + (assign48110_e61706 * ((-(-var_temp2_dn8)) * 0.3333333333333333))))))) / (assign48110_e61720 * assign48110_e61720))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48110_e61723;
        var_temp__blk936_dn5 = assign48110_e61723_d_n5;
        var_temp__blk936_dn6 = assign48110_e61723_d_n6;
        var_temp__blk936_dn7 = assign48110_e61723_d_n7;
        var_temp__blk936_dn8 = assign48110_e61723_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign48120_e61760, assign48120_e61760_d_n5, assign48120_e61760_d_n6, assign48120_e61760_d_n7, assign48120_e61760_d_n8,) = {
    if ((((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) && (var_guard1260 == 0.0)) && (var_guard1261 == 0.0)) {
        let assign48120_e61736: f64 = (-var_temp2);
        let assign48120_e61738: f64 = (assign48120_e61736 - 230.25850929940458);
        let assign48120_e61742: f64 = (-var_temp2);
        let assign48120_e61744: f64 = (assign48120_e61742 - 230.25850929940458);
        let assign48120_e61747: f64 = (-var_temp2);
        let assign48120_e61749: f64 = (assign48120_e61747 - 230.25850929940458);
        let assign48120_e61751: f64 = (assign48120_e61749 * 0.3333333333333333);
        let assign48120_e61752: f64 = (1.0 + assign48120_e61751);
        let assign48120_e61753: f64 = (assign48120_e61744 * assign48120_e61752);
        let assign48120_e61754: f64 = (0.5 * assign48120_e61753);
        let assign48120_e61755: f64 = (1.0 + assign48120_e61754);
        let assign48120_e61756: f64 = (assign48120_e61738 * assign48120_e61755);
        let assign48120_e61757: f64 = (1.0 + assign48120_e61756);
        let assign48120_e61758: f64 = (1e100 * assign48120_e61757);
        (assign48120_e61758, (1e100 * (((-var_temp2_dn5) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-var_temp2_dn5) * assign48120_e61752) + (assign48120_e61744 * ((-var_temp2_dn5) * 0.3333333333333333))))))), (1e100 * (((-var_temp2_dn6) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-var_temp2_dn6) * assign48120_e61752) + (assign48120_e61744 * ((-var_temp2_dn6) * 0.3333333333333333))))))), (1e100 * (((-var_temp2_dn7) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-var_temp2_dn7) * assign48120_e61752) + (assign48120_e61744 * ((-var_temp2_dn7) * 0.3333333333333333))))))), (1e100 * (((-var_temp2_dn8) * assign48120_e61755) + (assign48120_e61738 * (0.5 * (((-var_temp2_dn8) * assign48120_e61752) + (assign48120_e61744 * ((-var_temp2_dn8) * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48120_e61760;
        var_temp__blk936_dn5 = assign48120_e61760_d_n5;
        var_temp__blk936_dn6 = assign48120_e61760_d_n6;
        var_temp__blk936_dn7 = assign48120_e61760_d_n7;
        var_temp__blk936_dn8 = assign48120_e61760_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign48130_e61770, assign48130_e61770_d_n5, assign48130_e61770_d_n6, assign48130_e61770_d_n7, assign48130_e61770_d_n8,) = {
    if ((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) {
        let assign48130_e61767: f64 = (var_delvsat * var_temp__blk936);
        let assign48130_e61768: f64 = (var_a1_i * assign48130_e61767);
        (assign48130_e61768, (var_a1_i * ((var_delvsat_dn5 * var_temp__blk936) + (var_delvsat * var_temp__blk936_dn5))), (var_a1_i * ((var_delvsat_dn6 * var_temp__blk936) + (var_delvsat * var_temp__blk936_dn6))), (var_a1_i * ((var_delvsat_dn7 * var_temp__blk936) + (var_delvsat * var_temp__blk936_dn7))), (var_a1_i * ((var_delvsat_dn8 * var_temp__blk936) + (var_delvsat * var_temp__blk936_dn8))),)
    } else {
        (var_mavl, var_mavl_dn5, var_mavl_dn6, var_mavl_dn7, var_mavl_dn8,)
    }
};
        var_mavl = assign48130_e61770;
        var_mavl_dn5 = assign48130_e61770_d_n5;
        var_mavl_dn6 = assign48130_e61770_d_n6;
        var_mavl_dn7 = assign48130_e61770_d_n7;
        var_mavl_dn8 = assign48130_e61770_d_n8;
        var_mavl_rv = 0.0;

        let (assign48140_e61780, assign48140_e61780_d_n5, assign48140_e61780_d_n6, assign48140_e61780_d_n7, assign48140_e61780_d_n8,) = {
    if ((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) {
        let assign48140_e61777: f64 = (var_i_ds + var_i_dsedge);
        let assign48140_e61778: f64 = (var_mavl * assign48140_e61777);
        (assign48140_e61778, ((var_mavl_dn5 * assign48140_e61777) + (var_mavl * (var_i_ds_dn5 + var_i_dsedge_dn5))), ((var_mavl_dn6 * assign48140_e61777) + (var_mavl * (var_i_ds_dn6 + var_i_dsedge_dn6))), ((var_mavl_dn7 * assign48140_e61777) + (var_mavl * (var_i_ds_dn7 + var_i_dsedge_dn7))), ((var_mavl_dn8 * assign48140_e61777) + (var_mavl * (var_i_ds_dn8 + var_i_dsedge_dn8))),)
    } else {
        (var_iimpact, var_iimpact_dn5, var_iimpact_dn6, var_iimpact_dn7, var_iimpact_dn8,)
    }
};
        var_iimpact = assign48140_e61780;
        var_iimpact_dn5 = assign48140_e61780_d_n5;
        var_iimpact_dn6 = assign48140_e61780_d_n6;
        var_iimpact_dn7 = assign48140_e61780_d_n7;
        var_iimpact_dn8 = assign48140_e61780_d_n8;
        var_iimpact_rv = 0.0;

        let assign48150_e61784: f64 = (0.5 * var_imaxii_i);
        let assign48150_e61785: f64 = if var_iimpact > assign48150_e61784 { 1.0 } else { 0.0 };
        var_guard1262 = assign48150_e61785;
        var_guard1262_rv = 0.0;

        let (assign48160_e61799, assign48160_e61799_d_n5, assign48160_e61799_d_n6, assign48160_e61799_d_n7, assign48160_e61799_d_n8,) = {
    if (((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) && (var_guard1262 != 0.0)) {
        let assign48160_e61793: f64 = (2.0 * var_iimpact);
        let assign48160_e61795: f64 = (assign48160_e61793 / var_imaxii_i);
        let assign48160_e61797: f64 = (assign48160_e61795 - 1.0);
        (assign48160_e61797, ((2.0 * var_iimpact_dn5) / var_imaxii_i), ((2.0 * var_iimpact_dn6) / var_imaxii_i), ((2.0 * var_iimpact_dn7) / var_imaxii_i), ((2.0 * var_iimpact_dn8) / var_imaxii_i),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48160_e61799;
        var_temp__blk936_dn5 = assign48160_e61799_d_n5;
        var_temp__blk936_dn6 = assign48160_e61799_d_n6;
        var_temp__blk936_dn7 = assign48160_e61799_d_n7;
        var_temp__blk936_dn8 = assign48160_e61799_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign48170_e61820, assign48170_e61820_d_n5, assign48170_e61820_d_n6, assign48170_e61820_d_n7, assign48170_e61820_d_n8,) = {
    if (((var_guard1258 != 0.0) && (var_guard1259 != 0.0)) && (var_guard1262 != 0.0)) {
        let assign48170_e61807: f64 = (0.5 * var_imaxii_i);
        let assign48170_e61813: f64 = (var_temp__blk936 * var_temp__blk936);
        let assign48170_e61814: f64 = (1.0 + assign48170_e61813);
        let assign48170_e61815: f64 = (assign48170_e61814).sqrt();
        let assign48170_e61816: f64 = (var_temp__blk936 / assign48170_e61815);
        let assign48170_e61817: f64 = (1.0 + assign48170_e61816);
        let assign48170_e61818: f64 = (assign48170_e61807 * assign48170_e61817);
        (assign48170_e61818, (assign48170_e61807 * (((var_temp__blk936_dn5 * assign48170_e61815) - (var_temp__blk936 * (((var_temp__blk936_dn5 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn5)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((var_temp__blk936_dn6 * assign48170_e61815) - (var_temp__blk936 * (((var_temp__blk936_dn6 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn6)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((var_temp__blk936_dn7 * assign48170_e61815) - (var_temp__blk936 * (((var_temp__blk936_dn7 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn7)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))), (assign48170_e61807 * (((var_temp__blk936_dn8 * assign48170_e61815) - (var_temp__blk936 * (((var_temp__blk936_dn8 * var_temp__blk936) + (var_temp__blk936 * var_temp__blk936_dn8)) / (2.0 * assign48170_e61815)))) / (assign48170_e61815 * assign48170_e61815))),)
    } else {
        (var_iimpact, var_iimpact_dn5, var_iimpact_dn6, var_iimpact_dn7, var_iimpact_dn8,)
    }
};
        var_iimpact = assign48170_e61820;
        var_iimpact_dn5 = assign48170_e61820_d_n5;
        var_iimpact_dn6 = assign48170_e61820_d_n6;
        var_iimpact_dn7 = assign48170_e61820_d_n7;
        var_iimpact_dn8 = assign48170_e61820_d_n8;
        var_iimpact_rv = 0.0;

        let assign48180_e61831: f64 = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1456 = assign48180_e61831;
        var_guard1456_rv = 0.0;

        let assign48190_e61838: f64 = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };
        var_guard1457 = assign48190_e61838;
        var_guard1457_rv = 0.0;

        let (assign48200_e61844,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (var_phib_dc,)
    } else {
        (var_phib__blk1297,)
    }
};
        var_phib__blk1297 = assign48200_e61844;
        var_phib__blk1297_rv = 0.0;

        let (assign48210_e61850,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (var_aphi_dc,)
    } else {
        (var_aphi__blk1298,)
    }
};
        var_aphi__blk1298 = assign48210_e61850;
        var_aphi__blk1298_rv = 0.0;

        let (assign48220_e61856,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (var_g_0_dc,)
    } else {
        (var_g_0__blk1299,)
    }
};
        var_g_0__blk1299 = assign48220_e61856;
        var_g_0__blk1299_rv = 0.0;

        let (assign48230_e61862, assign48230_e61862_d_n6, assign48230_e61862_d_n7, assign48230_e61862_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (var_v_xb_dc_tmp, var_v_xb_dc_tmp_dn6, var_v_xb_dc_tmp_dn7, var_v_xb_dc_tmp_dn8,)
    } else {
        (var_v_xb__blk1300, var_v_xb__blk1300_dn6, var_v_xb__blk1300_dn7, var_v_xb__blk1300_dn8,)
    }
};
        var_v_xb__blk1300 = assign48230_e61862;
        var_v_xb__blk1300_dn6 = assign48230_e61862_d_n6;
        var_v_xb__blk1300_dn7 = assign48230_e61862_d_n7;
        var_v_xb__blk1300_dn8 = assign48230_e61862_d_n8;
        var_v_xb__blk1300_rv = 0.0;

        let (assign48240_e61868, assign48240_e61868_d_n5, assign48240_e61868_d_n6, assign48240_e61868_d_n7, assign48240_e61868_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (var_vsbstar_dc_tmp, var_vsbstar_dc_tmp_dn5, var_vsbstar_dc_tmp_dn6, var_vsbstar_dc_tmp_dn7, var_vsbstar_dc_tmp_dn8,)
    } else {
        (var_vsbstar__blk1301, var_vsbstar__blk1301_dn5, var_vsbstar__blk1301_dn6, var_vsbstar__blk1301_dn7, var_vsbstar__blk1301_dn8,)
    }
};
        var_vsbstar__blk1301 = assign48240_e61868;
        var_vsbstar__blk1301_dn5 = assign48240_e61868_d_n5;
        var_vsbstar__blk1301_dn6 = assign48240_e61868_d_n6;
        var_vsbstar__blk1301_dn7 = assign48240_e61868_d_n7;
        var_vsbstar__blk1301_dn8 = assign48240_e61868_d_n8;
        var_vsbstar__blk1301_rv = 0.0;

        let (assign48250_e61874,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (0.0,)
    } else {
        (var_dvbstar__blk1305,)
    }
};
        var_dvbstar__blk1305 = assign48250_e61874;
        var_dvbstar__blk1305_rv = 0.0;

        let assign48260_e61877: f64 = if p.p47 > 0.0 { 1.0 } else { 0.0 };
        var_guard1458 = assign48260_e61877;
        var_guard1458_rv = 0.0;

        let (assign48270_e61902, assign48270_e61902_d_n6, assign48270_e61902_d_n7, assign48270_e61902_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        let assign48270_e61886: f64 = (var_v_db + var_v_sb);
        let assign48270_e61889: f64 = (var_v_db - var_v_sb);
        let assign48270_e61892: f64 = (var_v_db - var_v_sb);
        let assign48270_e61893: f64 = (assign48270_e61889 * assign48270_e61892);
        let assign48270_e61895: f64 = (assign48270_e61893 + var_bphi_ac);
        let assign48270_e61896: f64 = (assign48270_e61895).sqrt();
        let assign48270_e61897: f64 = (assign48270_e61886 - assign48270_e61896);
        let assign48270_e61898: f64 = (0.5 * assign48270_e61897);
        let assign48270_e61900: f64 = (assign48270_e61898 + var_phix_ac);
        (assign48270_e61900, (0.5 * ((var_v_db_dn6 + var_v_sb_dn6) - ((((var_v_db_dn6 - var_v_sb_dn6) * assign48270_e61892) + (assign48270_e61889 * (var_v_db_dn6 - var_v_sb_dn6))) / (2.0 * assign48270_e61896)))), (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign48270_e61892) + (assign48270_e61889 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign48270_e61896)))), (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign48270_e61892) + (assign48270_e61889 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign48270_e61896)))),)
    } else {
        (var_v_xb__blk1300, var_v_xb__blk1300_dn6, var_v_xb__blk1300_dn7, var_v_xb__blk1300_dn8,)
    }
};
        var_v_xb__blk1300 = assign48270_e61902;
        var_v_xb__blk1300_dn6 = assign48270_e61902_d_n6;
        var_v_xb__blk1300_dn7 = assign48270_e61902_d_n7;
        var_v_xb__blk1300_dn8 = assign48270_e61902_d_n8;
        var_v_xb__blk1300_rv = 0.0;

        let (assign48280_e61929, assign48280_e61929_d_n6, assign48280_e61929_d_n7, assign48280_e61929_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        let assign48280_e61912: f64 = var_v_xb__blk1300;
        let assign48280_e61915: f64 = var_v_xb__blk1300;
        let assign48280_e61918: f64 = var_v_xb__blk1300;
        let assign48280_e61919: f64 = (assign48280_e61915 * assign48280_e61918);
        let assign48280_e61921: f64 = (assign48280_e61919 + var_aphi_ac);
        let assign48280_e61922: f64 = (assign48280_e61921).sqrt();
        let assign48280_e61923: f64 = (assign48280_e61912 - assign48280_e61922);
        let assign48280_e61924: f64 = (0.5 * assign48280_e61923);
        let assign48280_e61925: f64 = (var_v_sb - assign48280_e61924);
        let assign48280_e61927: f64 = (assign48280_e61925 + var_phix1_ac);
        (assign48280_e61927, (var_v_sb_dn6 - (0.5 * (var_v_xb__blk1300_dn6 - (((var_v_xb__blk1300_dn6 * assign48280_e61918) + (assign48280_e61915 * var_v_xb__blk1300_dn6)) / (2.0 * assign48280_e61922))))), (var_v_sb_dn7 - (0.5 * (var_v_xb__blk1300_dn7 - (((var_v_xb__blk1300_dn7 * assign48280_e61918) + (assign48280_e61915 * var_v_xb__blk1300_dn7)) / (2.0 * assign48280_e61922))))), (var_v_sb_dn8 - (0.5 * (var_v_xb__blk1300_dn8 - (((var_v_xb__blk1300_dn8 * assign48280_e61918) + (assign48280_e61915 * var_v_xb__blk1300_dn8)) / (2.0 * assign48280_e61922))))),)
    } else {
        (var_vsbstar_ac, var_vsbstar_ac_dn6, var_vsbstar_ac_dn7, var_vsbstar_ac_dn8,)
    }
};
        var_vsbstar_ac = assign48280_e61929;
        var_vsbstar_ac_dn6 = assign48280_e61929_d_n6;
        var_vsbstar_ac_dn7 = assign48280_e61929_d_n7;
        var_vsbstar_ac_dn8 = assign48280_e61929_d_n8;
        var_vsbstar_ac_rv = 0.0;

        *var_alphabmedge_slot = var_alphabmedge;
        *var_alphabmedge_dn5_slot = var_alphabmedge_dn5;
        *var_alphabmedge_dn6_slot = var_alphabmedge_dn6;
        *var_alphabmedge_dn7_slot = var_alphabmedge_dn7;
        *var_alphabmedge_dn8_slot = var_alphabmedge_dn8;
        *var_alphabmedge_rv_slot = var_alphabmedge_rv;
        *var_aphi__blk1298_slot = var_aphi__blk1298;
        *var_aphi__blk1298_rv_slot = var_aphi__blk1298_rv;
        *var_delvsat_slot = var_delvsat;
        *var_delvsat_dn5_slot = var_delvsat_dn5;
        *var_delvsat_dn6_slot = var_delvsat_dn6;
        *var_delvsat_dn7_slot = var_delvsat_dn7;
        *var_delvsat_dn8_slot = var_delvsat_dn8;
        *var_delvsat_rv_slot = var_delvsat_rv;
        *var_dsqredge_slot = var_dsqredge;
        *var_dsqredge_dn5_slot = var_dsqredge_dn5;
        *var_dsqredge_dn6_slot = var_dsqredge_dn6;
        *var_dsqredge_dn7_slot = var_dsqredge_dn7;
        *var_dsqredge_dn8_slot = var_dsqredge_dn8;
        *var_dsqredge_rv_slot = var_dsqredge_rv;
        *var_dvbstar__blk1305_slot = var_dvbstar__blk1305;
        *var_dvbstar__blk1305_rv_slot = var_dvbstar__blk1305_rv;
        *var_g_0__blk1299_slot = var_g_0__blk1299;
        *var_g_0__blk1299_rv_slot = var_g_0__blk1299_rv;
        *var_guard1257_slot = var_guard1257;
        *var_guard1257_rv_slot = var_guard1257_rv;
        *var_guard1258_slot = var_guard1258;
        *var_guard1258_rv_slot = var_guard1258_rv;
        *var_guard1259_slot = var_guard1259;
        *var_guard1259_rv_slot = var_guard1259_rv;
        *var_guard1260_slot = var_guard1260;
        *var_guard1260_rv_slot = var_guard1260_rv;
        *var_guard1261_slot = var_guard1261;
        *var_guard1261_rv_slot = var_guard1261_rv;
        *var_guard1262_slot = var_guard1262;
        *var_guard1262_rv_slot = var_guard1262_rv;
        *var_guard1456_slot = var_guard1456;
        *var_guard1456_rv_slot = var_guard1456_rv;
        *var_guard1457_slot = var_guard1457;
        *var_guard1457_rv_slot = var_guard1457_rv;
        *var_guard1458_slot = var_guard1458;
        *var_guard1458_rv_slot = var_guard1458_rv;
        *var_i_dsedge_slot = var_i_dsedge;
        *var_i_dsedge_dn5_slot = var_i_dsedge_dn5;
        *var_i_dsedge_dn6_slot = var_i_dsedge_dn6;
        *var_i_dsedge_dn7_slot = var_i_dsedge_dn7;
        *var_i_dsedge_dn8_slot = var_i_dsedge_dn8;
        *var_i_dsedge_rv_slot = var_i_dsedge_rv;
        *var_iimpact_slot = var_iimpact;
        *var_iimpact_dn5_slot = var_iimpact_dn5;
        *var_iimpact_dn6_slot = var_iimpact_dn6;
        *var_iimpact_dn7_slot = var_iimpact_dn7;
        *var_iimpact_dn8_slot = var_iimpact_dn8;
        *var_iimpact_rv_slot = var_iimpact_rv;
        *var_mavl_slot = var_mavl;
        *var_mavl_dn5_slot = var_mavl_dn5;
        *var_mavl_dn6_slot = var_mavl_dn6;
        *var_mavl_dn7_slot = var_mavl_dn7;
        *var_mavl_dn8_slot = var_mavl_dn8;
        *var_mavl_rv_slot = var_mavl_rv;
        *var_phib__blk1297_slot = var_phib__blk1297;
        *var_phib__blk1297_rv_slot = var_phib__blk1297_rv;
        *var_qdeffedge_slot = var_qdeffedge;
        *var_qdeffedge_dn5_slot = var_qdeffedge_dn5;
        *var_qdeffedge_dn6_slot = var_qdeffedge_dn6;
        *var_qdeffedge_dn7_slot = var_qdeffedge_dn7;
        *var_qdeffedge_dn8_slot = var_qdeffedge_dn8;
        *var_qdeffedge_rv_slot = var_qdeffedge_rv;
        *var_qdseffedge_slot = var_qdseffedge;
        *var_qdseffedge_dn5_slot = var_qdseffedge_dn5;
        *var_qdseffedge_dn6_slot = var_qdseffedge_dn6;
        *var_qdseffedge_dn7_slot = var_qdseffedge_dn7;
        *var_qdseffedge_dn8_slot = var_qdseffedge_dn8;
        *var_qdseffedge_rv_slot = var_qdseffedge_rv;
        *var_qmeffedge_slot = var_qmeffedge;
        *var_qmeffedge_dn5_slot = var_qmeffedge_dn5;
        *var_qmeffedge_dn6_slot = var_qmeffedge_dn6;
        *var_qmeffedge_dn7_slot = var_qmeffedge_dn7;
        *var_qmeffedge_dn8_slot = var_qmeffedge_dn8;
        *var_qmeffedge_rv_slot = var_qmeffedge_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_v_xb__blk1300_slot = var_v_xb__blk1300;
        *var_v_xb__blk1300_dn6_slot = var_v_xb__blk1300_dn6;
        *var_v_xb__blk1300_dn7_slot = var_v_xb__blk1300_dn7;
        *var_v_xb__blk1300_dn8_slot = var_v_xb__blk1300_dn8;
        *var_v_xb__blk1300_rv_slot = var_v_xb__blk1300_rv;
        *var_vsbstar__blk1301_slot = var_vsbstar__blk1301;
        *var_vsbstar__blk1301_dn5_slot = var_vsbstar__blk1301_dn5;
        *var_vsbstar__blk1301_dn6_slot = var_vsbstar__blk1301_dn6;
        *var_vsbstar__blk1301_dn7_slot = var_vsbstar__blk1301_dn7;
        *var_vsbstar__blk1301_dn8_slot = var_vsbstar__blk1301_dn8;
        *var_vsbstar__blk1301_rv_slot = var_vsbstar__blk1301_rv;
        *var_vsbstar_ac_slot = var_vsbstar_ac;
        *var_vsbstar_ac_dn6_slot = var_vsbstar_ac_dn6;
        *var_vsbstar_ac_dn7_slot = var_vsbstar_ac_dn7;
        *var_vsbstar_ac_dn8_slot = var_vsbstar_ac_dn8;
        *var_vsbstar_ac_rv_slot = var_vsbstar_ac_rv;
    }

    pub(super) fn stamp_reactive_block_43(
        var_aphi_ac: f64,
        var_ct_t: f64,
        var_ctb_i: f64,
        var_ctg_i: f64,
        var_ctg_t: f64,
        var_dvbstar__blk1305: f64,
        var_g_0_ac: f64,
        var_guard1456: f64,
        var_guard1457: f64,
        var_guard1458: f64,
        var_inv_phit: f64,
        var_phib_ac: f64,
        var_phit: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_ds: f64,
        var_v_ds_dn6: f64,
        var_v_ds_dn7: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vfb_t: f64,
        var_vgb: f64,
        var_vgb_dn5: f64,
        var_vgb_dn6: f64,
        var_vgb_dn7: f64,
        var_vgb_dn8: f64,
        var_vsbstar_ac: f64,
        var_vsbstar_ac_dn6: f64,
        var_vsbstar_ac_dn7: f64,
        var_vsbstar_ac_dn8: f64,
        var_aphi__blk1298_slot: &mut f64,
        var_aphi__blk1298_rv_slot: &mut f64,
        var_ct_fact__blk1319_slot: &mut f64,
        var_ct_fact__blk1319_dn5_slot: &mut f64,
        var_ct_fact__blk1319_dn6_slot: &mut f64,
        var_ct_fact__blk1319_dn7_slot: &mut f64,
        var_ct_fact__blk1319_dn8_slot: &mut f64,
        var_ct_fact__blk1319_rv_slot: &mut f64,
        var_dctg__blk1318_slot: &mut f64,
        var_dctg__blk1318_dn5_slot: &mut f64,
        var_dctg__blk1318_dn6_slot: &mut f64,
        var_dctg__blk1318_dn7_slot: &mut f64,
        var_dctg__blk1318_dn8_slot: &mut f64,
        var_dctg__blk1318_rv_slot: &mut f64,
        var_dphit1__blk1321_slot: &mut f64,
        var_dphit1__blk1321_dn5_slot: &mut f64,
        var_dphit1__blk1321_dn6_slot: &mut f64,
        var_dphit1__blk1321_dn7_slot: &mut f64,
        var_dphit1__blk1321_dn8_slot: &mut f64,
        var_dphit1__blk1321_rv_slot: &mut f64,
        var_g_0__blk1299_slot: &mut f64,
        var_g_0__blk1299_rv_slot: &mut f64,
        var_guard1459_slot: &mut f64,
        var_guard1459_rv_slot: &mut f64,
        var_guard1460_slot: &mut f64,
        var_guard1460_rv_slot: &mut f64,
        var_phib__blk1297_slot: &mut f64,
        var_phib__blk1297_rv_slot: &mut f64,
        var_phit1__blk1322_slot: &mut f64,
        var_phit1__blk1322_dn5_slot: &mut f64,
        var_phit1__blk1322_dn6_slot: &mut f64,
        var_phit1__blk1322_dn7_slot: &mut f64,
        var_phit1__blk1322_dn8_slot: &mut f64,
        var_phit1__blk1322_rv_slot: &mut f64,
        var_phitct__blk1320_slot: &mut f64,
        var_phitct__blk1320_dn5_slot: &mut f64,
        var_phitct__blk1320_dn6_slot: &mut f64,
        var_phitct__blk1320_dn7_slot: &mut f64,
        var_phitct__blk1320_dn8_slot: &mut f64,
        var_phitct__blk1320_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_vgb1__blk1304_slot: &mut f64,
        var_vgb1__blk1304_dn5_slot: &mut f64,
        var_vgb1__blk1304_dn6_slot: &mut f64,
        var_vgb1__blk1304_dn7_slot: &mut f64,
        var_vgb1__blk1304_dn8_slot: &mut f64,
        var_vgb1__blk1304_rv_slot: &mut f64,
        var_vsbstar__blk1301_slot: &mut f64,
        var_vsbstar__blk1301_dn5_slot: &mut f64,
        var_vsbstar__blk1301_dn6_slot: &mut f64,
        var_vsbstar__blk1301_dn7_slot: &mut f64,
        var_vsbstar__blk1301_dn8_slot: &mut f64,
        var_vsbstar__blk1301_rv_slot: &mut f64,
        var_vsbx__blk1306_slot: &mut f64,
        var_vsbx__blk1306_dn5_slot: &mut f64,
        var_vsbx__blk1306_dn6_slot: &mut f64,
        var_vsbx__blk1306_dn7_slot: &mut f64,
        var_vsbx__blk1306_dn8_slot: &mut f64,
        var_vsbx__blk1306_rv_slot: &mut f64,
        var_xbct__blk1309_slot: &mut f64,
        var_xbct__blk1309_rv_slot: &mut f64,
        var_xct__blk1317_slot: &mut f64,
        var_xct__blk1317_dn5_slot: &mut f64,
        var_xct__blk1317_dn6_slot: &mut f64,
        var_xct__blk1317_dn7_slot: &mut f64,
        var_xct__blk1317_dn8_slot: &mut f64,
        var_xct__blk1317_rv_slot: &mut f64,
        var_xctmax__blk1313_slot: &mut f64,
        var_xctmax__blk1313_rv_slot: &mut f64,
        var_xgct__blk1311_slot: &mut f64,
        var_xgct__blk1311_dn5_slot: &mut f64,
        var_xgct__blk1311_dn6_slot: &mut f64,
        var_xgct__blk1311_dn7_slot: &mut f64,
        var_xgct__blk1311_dn8_slot: &mut f64,
        var_xgct__blk1311_rv_slot: &mut f64,
        var_xmict__blk1315_slot: &mut f64,
        var_xmict__blk1315_dn5_slot: &mut f64,
        var_xmict__blk1315_dn6_slot: &mut f64,
        var_xmict__blk1315_dn7_slot: &mut f64,
        var_xmict__blk1315_dn8_slot: &mut f64,
        var_xmict__blk1315_rv_slot: &mut f64,
        var_xnct__blk1314_slot: &mut f64,
        var_xnct__blk1314_dn5_slot: &mut f64,
        var_xnct__blk1314_dn6_slot: &mut f64,
        var_xnct__blk1314_dn7_slot: &mut f64,
        var_xnct__blk1314_dn8_slot: &mut f64,
        var_xnct__blk1314_rv_slot: &mut f64,
        var_xsbstar__blk1310_slot: &mut f64,
        var_xsbstar__blk1310_dn5_slot: &mut f64,
        var_xsbstar__blk1310_dn6_slot: &mut f64,
        var_xsbstar__blk1310_dn7_slot: &mut f64,
        var_xsbstar__blk1310_dn8_slot: &mut f64,
        var_xsbstar__blk1310_rv_slot: &mut f64,
        var_xsubct__blk1316_slot: &mut f64,
        var_xsubct__blk1316_dn5_slot: &mut f64,
        var_xsubct__blk1316_dn6_slot: &mut f64,
        var_xsubct__blk1316_dn7_slot: &mut f64,
        var_xsubct__blk1316_dn8_slot: &mut f64,
        var_xsubct__blk1316_rv_slot: &mut f64,
        var_xwict__blk1312_slot: &mut f64,
        var_xwict__blk1312_dn5_slot: &mut f64,
        var_xwict__blk1312_dn6_slot: &mut f64,
        var_xwict__blk1312_dn7_slot: &mut f64,
        var_xwict__blk1312_dn8_slot: &mut f64,
        var_xwict__blk1312_rv_slot: &mut f64,
    ) {
        let mut var_aphi__blk1298: f64 = *var_aphi__blk1298_slot;
        let mut var_aphi__blk1298_rv: f64 = *var_aphi__blk1298_rv_slot;
        let mut var_ct_fact__blk1319: f64 = *var_ct_fact__blk1319_slot;
        let mut var_ct_fact__blk1319_dn5: f64 = *var_ct_fact__blk1319_dn5_slot;
        let mut var_ct_fact__blk1319_dn6: f64 = *var_ct_fact__blk1319_dn6_slot;
        let mut var_ct_fact__blk1319_dn7: f64 = *var_ct_fact__blk1319_dn7_slot;
        let mut var_ct_fact__blk1319_dn8: f64 = *var_ct_fact__blk1319_dn8_slot;
        let mut var_ct_fact__blk1319_rv: f64 = *var_ct_fact__blk1319_rv_slot;
        let mut var_dctg__blk1318: f64 = *var_dctg__blk1318_slot;
        let mut var_dctg__blk1318_dn5: f64 = *var_dctg__blk1318_dn5_slot;
        let mut var_dctg__blk1318_dn6: f64 = *var_dctg__blk1318_dn6_slot;
        let mut var_dctg__blk1318_dn7: f64 = *var_dctg__blk1318_dn7_slot;
        let mut var_dctg__blk1318_dn8: f64 = *var_dctg__blk1318_dn8_slot;
        let mut var_dctg__blk1318_rv: f64 = *var_dctg__blk1318_rv_slot;
        let mut var_dphit1__blk1321: f64 = *var_dphit1__blk1321_slot;
        let mut var_dphit1__blk1321_dn5: f64 = *var_dphit1__blk1321_dn5_slot;
        let mut var_dphit1__blk1321_dn6: f64 = *var_dphit1__blk1321_dn6_slot;
        let mut var_dphit1__blk1321_dn7: f64 = *var_dphit1__blk1321_dn7_slot;
        let mut var_dphit1__blk1321_dn8: f64 = *var_dphit1__blk1321_dn8_slot;
        let mut var_dphit1__blk1321_rv: f64 = *var_dphit1__blk1321_rv_slot;
        let mut var_g_0__blk1299: f64 = *var_g_0__blk1299_slot;
        let mut var_g_0__blk1299_rv: f64 = *var_g_0__blk1299_rv_slot;
        let mut var_guard1459: f64 = *var_guard1459_slot;
        let mut var_guard1459_rv: f64 = *var_guard1459_rv_slot;
        let mut var_guard1460: f64 = *var_guard1460_slot;
        let mut var_guard1460_rv: f64 = *var_guard1460_rv_slot;
        let mut var_phib__blk1297: f64 = *var_phib__blk1297_slot;
        let mut var_phib__blk1297_rv: f64 = *var_phib__blk1297_rv_slot;
        let mut var_phit1__blk1322: f64 = *var_phit1__blk1322_slot;
        let mut var_phit1__blk1322_dn5: f64 = *var_phit1__blk1322_dn5_slot;
        let mut var_phit1__blk1322_dn6: f64 = *var_phit1__blk1322_dn6_slot;
        let mut var_phit1__blk1322_dn7: f64 = *var_phit1__blk1322_dn7_slot;
        let mut var_phit1__blk1322_dn8: f64 = *var_phit1__blk1322_dn8_slot;
        let mut var_phit1__blk1322_rv: f64 = *var_phit1__blk1322_rv_slot;
        let mut var_phitct__blk1320: f64 = *var_phitct__blk1320_slot;
        let mut var_phitct__blk1320_dn5: f64 = *var_phitct__blk1320_dn5_slot;
        let mut var_phitct__blk1320_dn6: f64 = *var_phitct__blk1320_dn6_slot;
        let mut var_phitct__blk1320_dn7: f64 = *var_phitct__blk1320_dn7_slot;
        let mut var_phitct__blk1320_dn8: f64 = *var_phitct__blk1320_dn8_slot;
        let mut var_phitct__blk1320_rv: f64 = *var_phitct__blk1320_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_vgb1__blk1304: f64 = *var_vgb1__blk1304_slot;
        let mut var_vgb1__blk1304_dn5: f64 = *var_vgb1__blk1304_dn5_slot;
        let mut var_vgb1__blk1304_dn6: f64 = *var_vgb1__blk1304_dn6_slot;
        let mut var_vgb1__blk1304_dn7: f64 = *var_vgb1__blk1304_dn7_slot;
        let mut var_vgb1__blk1304_dn8: f64 = *var_vgb1__blk1304_dn8_slot;
        let mut var_vgb1__blk1304_rv: f64 = *var_vgb1__blk1304_rv_slot;
        let mut var_vsbstar__blk1301: f64 = *var_vsbstar__blk1301_slot;
        let mut var_vsbstar__blk1301_dn5: f64 = *var_vsbstar__blk1301_dn5_slot;
        let mut var_vsbstar__blk1301_dn6: f64 = *var_vsbstar__blk1301_dn6_slot;
        let mut var_vsbstar__blk1301_dn7: f64 = *var_vsbstar__blk1301_dn7_slot;
        let mut var_vsbstar__blk1301_dn8: f64 = *var_vsbstar__blk1301_dn8_slot;
        let mut var_vsbstar__blk1301_rv: f64 = *var_vsbstar__blk1301_rv_slot;
        let mut var_vsbx__blk1306: f64 = *var_vsbx__blk1306_slot;
        let mut var_vsbx__blk1306_dn5: f64 = *var_vsbx__blk1306_dn5_slot;
        let mut var_vsbx__blk1306_dn6: f64 = *var_vsbx__blk1306_dn6_slot;
        let mut var_vsbx__blk1306_dn7: f64 = *var_vsbx__blk1306_dn7_slot;
        let mut var_vsbx__blk1306_dn8: f64 = *var_vsbx__blk1306_dn8_slot;
        let mut var_vsbx__blk1306_rv: f64 = *var_vsbx__blk1306_rv_slot;
        let mut var_xbct__blk1309: f64 = *var_xbct__blk1309_slot;
        let mut var_xbct__blk1309_rv: f64 = *var_xbct__blk1309_rv_slot;
        let mut var_xct__blk1317: f64 = *var_xct__blk1317_slot;
        let mut var_xct__blk1317_dn5: f64 = *var_xct__blk1317_dn5_slot;
        let mut var_xct__blk1317_dn6: f64 = *var_xct__blk1317_dn6_slot;
        let mut var_xct__blk1317_dn7: f64 = *var_xct__blk1317_dn7_slot;
        let mut var_xct__blk1317_dn8: f64 = *var_xct__blk1317_dn8_slot;
        let mut var_xct__blk1317_rv: f64 = *var_xct__blk1317_rv_slot;
        let mut var_xctmax__blk1313: f64 = *var_xctmax__blk1313_slot;
        let mut var_xctmax__blk1313_rv: f64 = *var_xctmax__blk1313_rv_slot;
        let mut var_xgct__blk1311: f64 = *var_xgct__blk1311_slot;
        let mut var_xgct__blk1311_dn5: f64 = *var_xgct__blk1311_dn5_slot;
        let mut var_xgct__blk1311_dn6: f64 = *var_xgct__blk1311_dn6_slot;
        let mut var_xgct__blk1311_dn7: f64 = *var_xgct__blk1311_dn7_slot;
        let mut var_xgct__blk1311_dn8: f64 = *var_xgct__blk1311_dn8_slot;
        let mut var_xgct__blk1311_rv: f64 = *var_xgct__blk1311_rv_slot;
        let mut var_xmict__blk1315: f64 = *var_xmict__blk1315_slot;
        let mut var_xmict__blk1315_dn5: f64 = *var_xmict__blk1315_dn5_slot;
        let mut var_xmict__blk1315_dn6: f64 = *var_xmict__blk1315_dn6_slot;
        let mut var_xmict__blk1315_dn7: f64 = *var_xmict__blk1315_dn7_slot;
        let mut var_xmict__blk1315_dn8: f64 = *var_xmict__blk1315_dn8_slot;
        let mut var_xmict__blk1315_rv: f64 = *var_xmict__blk1315_rv_slot;
        let mut var_xnct__blk1314: f64 = *var_xnct__blk1314_slot;
        let mut var_xnct__blk1314_dn5: f64 = *var_xnct__blk1314_dn5_slot;
        let mut var_xnct__blk1314_dn6: f64 = *var_xnct__blk1314_dn6_slot;
        let mut var_xnct__blk1314_dn7: f64 = *var_xnct__blk1314_dn7_slot;
        let mut var_xnct__blk1314_dn8: f64 = *var_xnct__blk1314_dn8_slot;
        let mut var_xnct__blk1314_rv: f64 = *var_xnct__blk1314_rv_slot;
        let mut var_xsbstar__blk1310: f64 = *var_xsbstar__blk1310_slot;
        let mut var_xsbstar__blk1310_dn5: f64 = *var_xsbstar__blk1310_dn5_slot;
        let mut var_xsbstar__blk1310_dn6: f64 = *var_xsbstar__blk1310_dn6_slot;
        let mut var_xsbstar__blk1310_dn7: f64 = *var_xsbstar__blk1310_dn7_slot;
        let mut var_xsbstar__blk1310_dn8: f64 = *var_xsbstar__blk1310_dn8_slot;
        let mut var_xsbstar__blk1310_rv: f64 = *var_xsbstar__blk1310_rv_slot;
        let mut var_xsubct__blk1316: f64 = *var_xsubct__blk1316_slot;
        let mut var_xsubct__blk1316_dn5: f64 = *var_xsubct__blk1316_dn5_slot;
        let mut var_xsubct__blk1316_dn6: f64 = *var_xsubct__blk1316_dn6_slot;
        let mut var_xsubct__blk1316_dn7: f64 = *var_xsubct__blk1316_dn7_slot;
        let mut var_xsubct__blk1316_dn8: f64 = *var_xsubct__blk1316_dn8_slot;
        let mut var_xsubct__blk1316_rv: f64 = *var_xsubct__blk1316_rv_slot;
        let mut var_xwict__blk1312: f64 = *var_xwict__blk1312_slot;
        let mut var_xwict__blk1312_dn5: f64 = *var_xwict__blk1312_dn5_slot;
        let mut var_xwict__blk1312_dn6: f64 = *var_xwict__blk1312_dn6_slot;
        let mut var_xwict__blk1312_dn7: f64 = *var_xwict__blk1312_dn7_slot;
        let mut var_xwict__blk1312_dn8: f64 = *var_xwict__blk1312_dn8_slot;
        let mut var_xwict__blk1312_rv: f64 = *var_xwict__blk1312_rv_slot;

        let (assign48290_e61937, assign48290_e61937_d_n5, assign48290_e61937_d_n6, assign48290_e61937_d_n7, assign48290_e61937_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        (var_vsbstar_ac, 0.0, var_vsbstar_ac_dn6, var_vsbstar_ac_dn7, var_vsbstar_ac_dn8,)
    } else {
        (var_vsbstar__blk1301, var_vsbstar__blk1301_dn5, var_vsbstar__blk1301_dn6, var_vsbstar__blk1301_dn7, var_vsbstar__blk1301_dn8,)
    }
};
        var_vsbstar__blk1301 = assign48290_e61937;
        var_vsbstar__blk1301_dn5 = assign48290_e61937_d_n5;
        var_vsbstar__blk1301_dn6 = assign48290_e61937_d_n6;
        var_vsbstar__blk1301_dn7 = assign48290_e61937_d_n7;
        var_vsbstar__blk1301_dn8 = assign48290_e61937_d_n8;
        var_vsbstar__blk1301_rv = 0.0;

        let (assign48300_e61945,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        (var_phib_ac,)
    } else {
        (var_phib__blk1297,)
    }
};
        var_phib__blk1297 = assign48300_e61945;
        var_phib__blk1297_rv = 0.0;

        let (assign48310_e61953,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        (var_aphi_ac,)
    } else {
        (var_aphi__blk1298,)
    }
};
        var_aphi__blk1298 = assign48310_e61953;
        var_aphi__blk1298_rv = 0.0;

        let (assign48320_e61961,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1458 != 0.0)) {
        (var_g_0_ac,)
    } else {
        (var_g_0__blk1299,)
    }
};
        var_g_0__blk1299 = assign48320_e61961;
        var_g_0__blk1299_rv = 0.0;

        let (assign48330_e61971, assign48330_e61971_d_n5, assign48330_e61971_d_n6, assign48330_e61971_d_n7, assign48330_e61971_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48330_e61967: f64 = (var_vgb - var_dvbstar__blk1305);
        let assign48330_e61969: f64 = (assign48330_e61967 - var_vfb_t);
        (assign48330_e61969, var_vgb_dn5, var_vgb_dn6, var_vgb_dn7, var_vgb_dn8,)
    } else {
        (var_vgb1__blk1304, var_vgb1__blk1304_dn5, var_vgb1__blk1304_dn6, var_vgb1__blk1304_dn7, var_vgb1__blk1304_dn8,)
    }
};
        var_vgb1__blk1304 = assign48330_e61971;
        var_vgb1__blk1304_dn5 = assign48330_e61971_d_n5;
        var_vgb1__blk1304_dn6 = assign48330_e61971_d_n6;
        var_vgb1__blk1304_dn7 = assign48330_e61971_d_n7;
        var_vgb1__blk1304_dn8 = assign48330_e61971_d_n8;
        var_vgb1__blk1304_rv = 0.0;

        let (assign48340_e61983, assign48340_e61983_d_n5, assign48340_e61983_d_n6, assign48340_e61983_d_n7, assign48340_e61983_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48340_e61979: f64 = (var_v_ds - var_vdsx);
        let assign48340_e61980: f64 = (0.5 * assign48340_e61979);
        let assign48340_e61981: f64 = (var_vsbstar__blk1301 + assign48340_e61980);
        (assign48340_e61981, var_vsbstar__blk1301_dn5, (var_vsbstar__blk1301_dn6 + (0.5 * (var_v_ds_dn6 - var_vdsx_dn6))), (var_vsbstar__blk1301_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), var_vsbstar__blk1301_dn8,)
    } else {
        (var_vsbx__blk1306, var_vsbx__blk1306_dn5, var_vsbx__blk1306_dn6, var_vsbx__blk1306_dn7, var_vsbx__blk1306_dn8,)
    }
};
        var_vsbx__blk1306 = assign48340_e61983;
        var_vsbx__blk1306_dn5 = assign48340_e61983_d_n5;
        var_vsbx__blk1306_dn6 = assign48340_e61983_d_n6;
        var_vsbx__blk1306_dn7 = assign48340_e61983_d_n7;
        var_vsbx__blk1306_dn8 = assign48340_e61983_d_n8;
        var_vsbx__blk1306_rv = 0.0;

        let (assign48350_e61989, assign48350_e61989_d_n5, assign48350_e61989_d_n6, assign48350_e61989_d_n7, assign48350_e61989_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dctg__blk1318, var_dctg__blk1318_dn5, var_dctg__blk1318_dn6, var_dctg__blk1318_dn7, var_dctg__blk1318_dn8,)
    }
};
        var_dctg__blk1318 = assign48350_e61989;
        var_dctg__blk1318_dn5 = assign48350_e61989_d_n5;
        var_dctg__blk1318_dn6 = assign48350_e61989_d_n6;
        var_dctg__blk1318_dn7 = assign48350_e61989_d_n7;
        var_dctg__blk1318_dn8 = assign48350_e61989_d_n8;
        var_dctg__blk1318_rv = 0.0;

        let assign48360_e61992: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1459 = assign48360_e61992;
        var_guard1459_rv = 0.0;

        let (assign48370_e62002,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48370_e62000: f64 = (var_phib__blk1297 * var_inv_phit);
        (assign48370_e62000,)
    } else {
        (var_xbct__blk1309,)
    }
};
        var_xbct__blk1309 = assign48370_e62002;
        var_xbct__blk1309_rv = 0.0;

        let (assign48380_e62012, assign48380_e62012_d_n5, assign48380_e62012_d_n6, assign48380_e62012_d_n7, assign48380_e62012_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48380_e62010: f64 = (var_vsbx__blk1306 * var_inv_phit);
        (assign48380_e62010, (var_vsbx__blk1306_dn5 * var_inv_phit), (var_vsbx__blk1306_dn6 * var_inv_phit), (var_vsbx__blk1306_dn7 * var_inv_phit), (var_vsbx__blk1306_dn8 * var_inv_phit),)
    } else {
        (var_xsbstar__blk1310, var_xsbstar__blk1310_dn5, var_xsbstar__blk1310_dn6, var_xsbstar__blk1310_dn7, var_xsbstar__blk1310_dn8,)
    }
};
        var_xsbstar__blk1310 = assign48380_e62012;
        var_xsbstar__blk1310_dn5 = assign48380_e62012_d_n5;
        var_xsbstar__blk1310_dn6 = assign48380_e62012_d_n6;
        var_xsbstar__blk1310_dn7 = assign48380_e62012_d_n7;
        var_xsbstar__blk1310_dn8 = assign48380_e62012_d_n8;
        var_xsbstar__blk1310_rv = 0.0;

        let (assign48390_e62022, assign48390_e62022_d_n5, assign48390_e62022_d_n6, assign48390_e62022_d_n7, assign48390_e62022_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48390_e62020: f64 = (var_vgb1__blk1304 * var_inv_phit);
        (assign48390_e62020, (var_vgb1__blk1304_dn5 * var_inv_phit), (var_vgb1__blk1304_dn6 * var_inv_phit), (var_vgb1__blk1304_dn7 * var_inv_phit), (var_vgb1__blk1304_dn8 * var_inv_phit),)
    } else {
        (var_xgct__blk1311, var_xgct__blk1311_dn5, var_xgct__blk1311_dn6, var_xgct__blk1311_dn7, var_xgct__blk1311_dn8,)
    }
};
        var_xgct__blk1311 = assign48390_e62022;
        var_xgct__blk1311_dn5 = assign48390_e62022_d_n5;
        var_xgct__blk1311_dn6 = assign48390_e62022_d_n6;
        var_xgct__blk1311_dn7 = assign48390_e62022_d_n7;
        var_xgct__blk1311_dn8 = assign48390_e62022_d_n8;
        var_xgct__blk1311_rv = 0.0;

        let (assign48400_e62037, assign48400_e62037_d_n5, assign48400_e62037_d_n6, assign48400_e62037_d_n7, assign48400_e62037_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48400_e62031: f64 = (0.5 * var_g_0__blk1299);
        let assign48400_e62033: f64 = (var_xbct__blk1309).sqrt();
        let assign48400_e62034: f64 = (assign48400_e62031 / assign48400_e62033);
        let assign48400_e62035: f64 = (1.0 + assign48400_e62034);
        (assign48400_e62035, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign48400_e62037;
        var_temp1_dn5 = assign48400_e62037_d_n5;
        var_temp1_dn6 = assign48400_e62037_d_n6;
        var_temp1_dn7 = assign48400_e62037_d_n7;
        var_temp1_dn8 = assign48400_e62037_d_n8;
        var_temp1_rv = 0.0;

        let (assign48410_e62050, assign48410_e62050_d_n5, assign48410_e62050_d_n6, assign48410_e62050_d_n7, assign48410_e62050_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48410_e62046: f64 = (var_xbct__blk1309).sqrt();
        let assign48410_e62047: f64 = (var_g_0__blk1299 * assign48410_e62046);
        let assign48410_e62048: f64 = (var_xbct__blk1309 + assign48410_e62047);
        (assign48410_e62048, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign48410_e62050;
        var_temp2_dn5 = assign48410_e62050_d_n5;
        var_temp2_dn6 = assign48410_e62050_d_n6;
        var_temp2_dn7 = assign48410_e62050_d_n7;
        var_temp2_dn8 = assign48410_e62050_d_n8;
        var_temp2_rv = 0.0;

        let (assign48420_e62072, assign48420_e62072_d_n5, assign48420_e62072_d_n6, assign48420_e62072_d_n7, assign48420_e62072_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48420_e62058: f64 = (var_xgct__blk1311 - var_temp2);
        let assign48420_e62060: f64 = (assign48420_e62058 / var_temp1);
        let assign48420_e62063: f64 = (0.5 * var_xbct__blk1309);
        let assign48420_e62064: f64 = (assign48420_e62060 + assign48420_e62063);
        let assign48420_e62067: f64 = (1.0 + var_ctb_i);
        let assign48420_e62069: f64 = (assign48420_e62067 * var_xsbstar__blk1310);
        let assign48420_e62070: f64 = (assign48420_e62064 - assign48420_e62069);
        (assign48420_e62070, (((((var_xgct__blk1311_dn5 - var_temp2_dn5) * var_temp1) - (assign48420_e62058 * var_temp1_dn5)) / (var_temp1 * var_temp1)) - (assign48420_e62067 * var_xsbstar__blk1310_dn5)), (((((var_xgct__blk1311_dn6 - var_temp2_dn6) * var_temp1) - (assign48420_e62058 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign48420_e62067 * var_xsbstar__blk1310_dn6)), (((((var_xgct__blk1311_dn7 - var_temp2_dn7) * var_temp1) - (assign48420_e62058 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign48420_e62067 * var_xsbstar__blk1310_dn7)), (((((var_xgct__blk1311_dn8 - var_temp2_dn8) * var_temp1) - (assign48420_e62058 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign48420_e62067 * var_xsbstar__blk1310_dn8)),)
    } else {
        (var_xwict__blk1312, var_xwict__blk1312_dn5, var_xwict__blk1312_dn6, var_xwict__blk1312_dn7, var_xwict__blk1312_dn8,)
    }
};
        var_xwict__blk1312 = assign48420_e62072;
        var_xwict__blk1312_dn5 = assign48420_e62072_d_n5;
        var_xwict__blk1312_dn6 = assign48420_e62072_d_n6;
        var_xwict__blk1312_dn7 = assign48420_e62072_d_n7;
        var_xwict__blk1312_dn8 = assign48420_e62072_d_n8;
        var_xwict__blk1312_rv = 0.0;

        let (assign48430_e62084,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48430_e62080: f64 = (0.5 * var_xbct__blk1309);
        let assign48430_e62082: f64 = (assign48430_e62080 + 2.0);
        (assign48430_e62082,)
    } else {
        (var_xctmax__blk1313,)
    }
};
        var_xctmax__blk1313 = assign48430_e62084;
        var_xctmax__blk1313_rv = 0.0;

        let (assign48440_e62094, assign48440_e62094_d_n5, assign48440_e62094_d_n6, assign48440_e62094_d_n7, assign48440_e62094_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48440_e62092: f64 = (var_xbct__blk1309 + var_xsbstar__blk1310);
        (assign48440_e62092, var_xsbstar__blk1310_dn5, var_xsbstar__blk1310_dn6, var_xsbstar__blk1310_dn7, var_xsbstar__blk1310_dn8,)
    } else {
        (var_xnct__blk1314, var_xnct__blk1314_dn5, var_xnct__blk1314_dn6, var_xnct__blk1314_dn7, var_xnct__blk1314_dn8,)
    }
};
        var_xnct__blk1314 = assign48440_e62094;
        var_xnct__blk1314_dn5 = assign48440_e62094_d_n5;
        var_xnct__blk1314_dn6 = assign48440_e62094_d_n6;
        var_xnct__blk1314_dn7 = assign48440_e62094_d_n7;
        var_xnct__blk1314_dn8 = assign48440_e62094_d_n8;
        var_xnct__blk1314_rv = 0.0;

        let (assign48450_e62119, assign48450_e62119_d_n5, assign48450_e62119_d_n6, assign48450_e62119_d_n7, assign48450_e62119_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48450_e62102: f64 = (var_xgct__blk1311 - var_xnct__blk1314);
        let assign48450_e62105: f64 = (var_xnct__blk1314).sqrt();
        let assign48450_e62106: f64 = (var_g_0__blk1299 * assign48450_e62105);
        let assign48450_e62107: f64 = (assign48450_e62102 - assign48450_e62106);
        let assign48450_e62111: f64 = (var_xbct__blk1309 / var_g_0__blk1299);
        let assign48450_e62113: f64 = (var_xbct__blk1309).sqrt();
        let assign48450_e62114: f64 = (assign48450_e62111 + assign48450_e62113);
        let assign48450_e62115: f64 = (assign48450_e62114).ln();
        let assign48450_e62116: f64 = (2.0 * assign48450_e62115);
        let assign48450_e62117: f64 = (assign48450_e62107 - assign48450_e62116);
        (assign48450_e62117, ((var_xgct__blk1311_dn5 - var_xnct__blk1314_dn5) - (var_g_0__blk1299 * (var_xnct__blk1314_dn5 / (2.0 * assign48450_e62105)))), ((var_xgct__blk1311_dn6 - var_xnct__blk1314_dn6) - (var_g_0__blk1299 * (var_xnct__blk1314_dn6 / (2.0 * assign48450_e62105)))), ((var_xgct__blk1311_dn7 - var_xnct__blk1314_dn7) - (var_g_0__blk1299 * (var_xnct__blk1314_dn7 / (2.0 * assign48450_e62105)))), ((var_xgct__blk1311_dn8 - var_xnct__blk1314_dn8) - (var_g_0__blk1299 * (var_xnct__blk1314_dn8 / (2.0 * assign48450_e62105)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign48450_e62119;
        var_temp1_dn5 = assign48450_e62119_d_n5;
        var_temp1_dn6 = assign48450_e62119_d_n6;
        var_temp1_dn7 = assign48450_e62119_d_n7;
        var_temp1_dn8 = assign48450_e62119_d_n8;
        var_temp1_rv = 0.0;

        let (assign48460_e62131, assign48460_e62131_d_n5, assign48460_e62131_d_n6, assign48460_e62131_d_n7, assign48460_e62131_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48460_e62127: f64 = (2.0 * var_temp1);
        let assign48460_e62129: f64 = (assign48460_e62127 + var_xctmax__blk1313);
        (assign48460_e62129, (2.0 * var_temp1_dn5), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8),)
    } else {
        (var_xmict__blk1315, var_xmict__blk1315_dn5, var_xmict__blk1315_dn6, var_xmict__blk1315_dn7, var_xmict__blk1315_dn8,)
    }
};
        var_xmict__blk1315 = assign48460_e62131;
        var_xmict__blk1315_dn5 = assign48460_e62131_d_n5;
        var_xmict__blk1315_dn6 = assign48460_e62131_d_n6;
        var_xmict__blk1315_dn7 = assign48460_e62131_d_n7;
        var_xmict__blk1315_dn8 = assign48460_e62131_d_n8;
        var_xmict__blk1315_rv = 0.0;

        let (assign48470_e62154, assign48470_e62154_d_n5, assign48470_e62154_d_n6, assign48470_e62154_d_n7, assign48470_e62154_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48470_e62140: f64 = (var_xwict__blk1312 + var_xmict__blk1315);
        let assign48470_e62143: f64 = (var_xwict__blk1312 - var_xmict__blk1315);
        let assign48470_e62146: f64 = (var_xwict__blk1312 - var_xmict__blk1315);
        let assign48470_e62147: f64 = (assign48470_e62143 * assign48470_e62146);
        let assign48470_e62149: f64 = (assign48470_e62147 + 20.0);
        let assign48470_e62150: f64 = (assign48470_e62149).sqrt();
        let assign48470_e62151: f64 = (assign48470_e62140 + assign48470_e62150);
        let assign48470_e62152: f64 = (0.5 * assign48470_e62151);
        (assign48470_e62152, (0.5 * ((var_xwict__blk1312_dn5 + var_xmict__blk1315_dn5) + ((((var_xwict__blk1312_dn5 - var_xmict__blk1315_dn5) * assign48470_e62146) + (assign48470_e62143 * (var_xwict__blk1312_dn5 - var_xmict__blk1315_dn5))) / (2.0 * assign48470_e62150)))), (0.5 * ((var_xwict__blk1312_dn6 + var_xmict__blk1315_dn6) + ((((var_xwict__blk1312_dn6 - var_xmict__blk1315_dn6) * assign48470_e62146) + (assign48470_e62143 * (var_xwict__blk1312_dn6 - var_xmict__blk1315_dn6))) / (2.0 * assign48470_e62150)))), (0.5 * ((var_xwict__blk1312_dn7 + var_xmict__blk1315_dn7) + ((((var_xwict__blk1312_dn7 - var_xmict__blk1315_dn7) * assign48470_e62146) + (assign48470_e62143 * (var_xwict__blk1312_dn7 - var_xmict__blk1315_dn7))) / (2.0 * assign48470_e62150)))), (0.5 * ((var_xwict__blk1312_dn8 + var_xmict__blk1315_dn8) + ((((var_xwict__blk1312_dn8 - var_xmict__blk1315_dn8) * assign48470_e62146) + (assign48470_e62143 * (var_xwict__blk1312_dn8 - var_xmict__blk1315_dn8))) / (2.0 * assign48470_e62150)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign48470_e62154;
        var_temp1_dn5 = assign48470_e62154_d_n5;
        var_temp1_dn6 = assign48470_e62154_d_n6;
        var_temp1_dn7 = assign48470_e62154_d_n7;
        var_temp1_dn8 = assign48470_e62154_d_n8;
        var_temp1_rv = 0.0;

        let (assign48480_e62168, assign48480_e62168_d_n5, assign48480_e62168_d_n6, assign48480_e62168_d_n7, assign48480_e62168_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48480_e62163: f64 = (var_xgct__blk1311 - var_xsbstar__blk1310);
        let assign48480_e62164: f64 = (2.0 * assign48480_e62163);
        let assign48480_e62166: f64 = (assign48480_e62164 - var_xctmax__blk1313);
        (assign48480_e62166, (2.0 * (var_xgct__blk1311_dn5 - var_xsbstar__blk1310_dn5)), (2.0 * (var_xgct__blk1311_dn6 - var_xsbstar__blk1310_dn6)), (2.0 * (var_xgct__blk1311_dn7 - var_xsbstar__blk1310_dn7)), (2.0 * (var_xgct__blk1311_dn8 - var_xsbstar__blk1310_dn8)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign48480_e62168;
        var_temp2_dn5 = assign48480_e62168_d_n5;
        var_temp2_dn6 = assign48480_e62168_d_n6;
        var_temp2_dn7 = assign48480_e62168_d_n7;
        var_temp2_dn8 = assign48480_e62168_d_n8;
        var_temp2_rv = 0.0;

        let (assign48490_e62191, assign48490_e62191_d_n5, assign48490_e62191_d_n6, assign48490_e62191_d_n7, assign48490_e62191_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48490_e62177: f64 = (var_temp1 + var_temp2);
        let assign48490_e62180: f64 = (var_temp1 - var_temp2);
        let assign48490_e62183: f64 = (var_temp1 - var_temp2);
        let assign48490_e62184: f64 = (assign48490_e62180 * assign48490_e62183);
        let assign48490_e62186: f64 = (assign48490_e62184 + 20.0);
        let assign48490_e62187: f64 = (assign48490_e62186).sqrt();
        let assign48490_e62188: f64 = (assign48490_e62177 - assign48490_e62187);
        let assign48490_e62189: f64 = (0.5 * assign48490_e62188);
        (assign48490_e62189, (0.5 * ((var_temp1_dn5 + var_temp2_dn5) - ((((var_temp1_dn5 - var_temp2_dn5) * assign48490_e62183) + (assign48490_e62180 * (var_temp1_dn5 - var_temp2_dn5))) / (2.0 * assign48490_e62187)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign48490_e62183) + (assign48490_e62180 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign48490_e62187)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign48490_e62183) + (assign48490_e62180 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign48490_e62187)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign48490_e62183) + (assign48490_e62180 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign48490_e62187)))),)
    } else {
        (var_xsubct__blk1316, var_xsubct__blk1316_dn5, var_xsubct__blk1316_dn6, var_xsubct__blk1316_dn7, var_xsubct__blk1316_dn8,)
    }
};
        var_xsubct__blk1316 = assign48490_e62191;
        var_xsubct__blk1316_dn5 = assign48490_e62191_d_n5;
        var_xsubct__blk1316_dn6 = assign48490_e62191_d_n6;
        var_xsubct__blk1316_dn7 = assign48490_e62191_d_n7;
        var_xsubct__blk1316_dn8 = assign48490_e62191_d_n8;
        var_xsubct__blk1316_rv = 0.0;

        let (assign48500_e62214, assign48500_e62214_d_n5, assign48500_e62214_d_n6, assign48500_e62214_d_n7, assign48500_e62214_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48500_e62200: f64 = (var_xsubct__blk1316 + var_xctmax__blk1313);
        let assign48500_e62203: f64 = (var_xsubct__blk1316 - var_xctmax__blk1313);
        let assign48500_e62206: f64 = (var_xsubct__blk1316 - var_xctmax__blk1313);
        let assign48500_e62207: f64 = (assign48500_e62203 * assign48500_e62206);
        let assign48500_e62209: f64 = (assign48500_e62207 + 5.0);
        let assign48500_e62210: f64 = (assign48500_e62209).sqrt();
        let assign48500_e62211: f64 = (assign48500_e62200 - assign48500_e62210);
        let assign48500_e62212: f64 = (0.5 * assign48500_e62211);
        (assign48500_e62212, (0.5 * (var_xsubct__blk1316_dn5 - (((var_xsubct__blk1316_dn5 * assign48500_e62206) + (assign48500_e62203 * var_xsubct__blk1316_dn5)) / (2.0 * assign48500_e62210)))), (0.5 * (var_xsubct__blk1316_dn6 - (((var_xsubct__blk1316_dn6 * assign48500_e62206) + (assign48500_e62203 * var_xsubct__blk1316_dn6)) / (2.0 * assign48500_e62210)))), (0.5 * (var_xsubct__blk1316_dn7 - (((var_xsubct__blk1316_dn7 * assign48500_e62206) + (assign48500_e62203 * var_xsubct__blk1316_dn7)) / (2.0 * assign48500_e62210)))), (0.5 * (var_xsubct__blk1316_dn8 - (((var_xsubct__blk1316_dn8 * assign48500_e62206) + (assign48500_e62203 * var_xsubct__blk1316_dn8)) / (2.0 * assign48500_e62210)))),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign48500_e62214;
        var_temp1_dn5 = assign48500_e62214_d_n5;
        var_temp1_dn6 = assign48500_e62214_d_n6;
        var_temp1_dn7 = assign48500_e62214_d_n7;
        var_temp1_dn8 = assign48500_e62214_d_n8;
        var_temp1_rv = 0.0;

        let (assign48510_e62240, assign48510_e62240_d_n5, assign48510_e62240_d_n6, assign48510_e62240_d_n7, assign48510_e62240_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48510_e62223: f64 = (-var_xctmax__blk1313);
        let assign48510_e62224: f64 = (var_temp1 + assign48510_e62223);
        let assign48510_e62227: f64 = (-var_xctmax__blk1313);
        let assign48510_e62228: f64 = (var_temp1 - assign48510_e62227);
        let assign48510_e62231: f64 = (-var_xctmax__blk1313);
        let assign48510_e62232: f64 = (var_temp1 - assign48510_e62231);
        let assign48510_e62233: f64 = (assign48510_e62228 * assign48510_e62232);
        let assign48510_e62235: f64 = (assign48510_e62233 + 20.0);
        let assign48510_e62236: f64 = (assign48510_e62235).sqrt();
        let assign48510_e62237: f64 = (assign48510_e62224 + assign48510_e62236);
        let assign48510_e62238: f64 = (0.5 * assign48510_e62237);
        (assign48510_e62238, (0.5 * (var_temp1_dn5 + (((var_temp1_dn5 * assign48510_e62232) + (assign48510_e62228 * var_temp1_dn5)) / (2.0 * assign48510_e62236)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign48510_e62232) + (assign48510_e62228 * var_temp1_dn6)) / (2.0 * assign48510_e62236)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign48510_e62232) + (assign48510_e62228 * var_temp1_dn7)) / (2.0 * assign48510_e62236)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign48510_e62232) + (assign48510_e62228 * var_temp1_dn8)) / (2.0 * assign48510_e62236)))),)
    } else {
        (var_xct__blk1317, var_xct__blk1317_dn5, var_xct__blk1317_dn6, var_xct__blk1317_dn7, var_xct__blk1317_dn8,)
    }
};
        var_xct__blk1317 = assign48510_e62240;
        var_xct__blk1317_dn5 = assign48510_e62240_d_n5;
        var_xct__blk1317_dn6 = assign48510_e62240_d_n6;
        var_xct__blk1317_dn7 = assign48510_e62240_d_n7;
        var_xct__blk1317_dn8 = assign48510_e62240_d_n8;
        var_xct__blk1317_rv = 0.0;

        let (assign48520_e62254, assign48520_e62254_d_n5, assign48520_e62254_d_n6, assign48520_e62254_d_n7, assign48520_e62254_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) {
        let assign48520_e62249: f64 = (var_xct__blk1317 / var_xctmax__blk1313);
        let assign48520_e62251: f64 = (assign48520_e62249 + 1.0);
        let assign48520_e62252: f64 = (var_ctg_t * assign48520_e62251);
        (assign48520_e62252, (var_ctg_t * (var_xct__blk1317_dn5 / var_xctmax__blk1313)), (var_ctg_t * (var_xct__blk1317_dn6 / var_xctmax__blk1313)), (var_ctg_t * (var_xct__blk1317_dn7 / var_xctmax__blk1313)), (var_ctg_t * (var_xct__blk1317_dn8 / var_xctmax__blk1313)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign48520_e62254;
        var_temp2_dn5 = assign48520_e62254_d_n5;
        var_temp2_dn6 = assign48520_e62254_d_n6;
        var_temp2_dn7 = assign48520_e62254_d_n7;
        var_temp2_dn8 = assign48520_e62254_d_n8;
        var_temp2_rv = 0.0;

        let assign48530_e62257: f64 = (-230.25850929940458);
        let assign48530_e62258: f64 = if var_temp2 > assign48530_e62257 { 1.0 } else { 0.0 };
        var_guard1460 = assign48530_e62258;
        var_guard1460_rv = 0.0;

        let (assign48540_e62269, assign48540_e62269_d_n5, assign48540_e62269_d_n6, assign48540_e62269_d_n7, assign48540_e62269_d_n8,) = {
    if ((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) && (var_guard1460 != 0.0)) {
        let assign48540_e62267: f64 = (var_temp2).exp();
        (assign48540_e62267, (assign48540_e62267 * var_temp2_dn5), (assign48540_e62267 * var_temp2_dn6), (assign48540_e62267 * var_temp2_dn7), (assign48540_e62267 * var_temp2_dn8),)
    } else {
        (var_dctg__blk1318, var_dctg__blk1318_dn5, var_dctg__blk1318_dn6, var_dctg__blk1318_dn7, var_dctg__blk1318_dn8,)
    }
};
        var_dctg__blk1318 = assign48540_e62269;
        var_dctg__blk1318_dn5 = assign48540_e62269_d_n5;
        var_dctg__blk1318_dn6 = assign48540_e62269_d_n6;
        var_dctg__blk1318_dn7 = assign48540_e62269_d_n7;
        var_dctg__blk1318_dn8 = assign48540_e62269_d_n8;
        var_dctg__blk1318_rv = 0.0;

        let (assign48550_e62305, assign48550_e62305_d_n5, assign48550_e62305_d_n6, assign48550_e62305_d_n7, assign48550_e62305_d_n8,) = {
    if ((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1459 != 0.0)) && (var_guard1460 == 0.0)) {
        let assign48550_e62281: f64 = (-230.25850929940458);
        let assign48550_e62283: f64 = (assign48550_e62281 - var_temp2);
        let assign48550_e62287: f64 = (-230.25850929940458);
        let assign48550_e62289: f64 = (assign48550_e62287 - var_temp2);
        let assign48550_e62292: f64 = (-230.25850929940458);
        let assign48550_e62294: f64 = (assign48550_e62292 - var_temp2);
        let assign48550_e62296: f64 = (assign48550_e62294 * 0.3333333333333333);
        let assign48550_e62297: f64 = (1.0 + assign48550_e62296);
        let assign48550_e62298: f64 = (assign48550_e62289 * assign48550_e62297);
        let assign48550_e62299: f64 = (0.5 * assign48550_e62298);
        let assign48550_e62300: f64 = (1.0 + assign48550_e62299);
        let assign48550_e62301: f64 = (assign48550_e62283 * assign48550_e62300);
        let assign48550_e62302: f64 = (1.0 + assign48550_e62301);
        let assign48550_e62303: f64 = (1e-100 / assign48550_e62302);
        (assign48550_e62303, (-((1e-100 * (((-var_temp2_dn5) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-var_temp2_dn5) * assign48550_e62297) + (assign48550_e62289 * ((-var_temp2_dn5) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-var_temp2_dn6) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-var_temp2_dn6) * assign48550_e62297) + (assign48550_e62289 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-var_temp2_dn7) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-var_temp2_dn7) * assign48550_e62297) + (assign48550_e62289 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))), (-((1e-100 * (((-var_temp2_dn8) * assign48550_e62300) + (assign48550_e62283 * (0.5 * (((-var_temp2_dn8) * assign48550_e62297) + (assign48550_e62289 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign48550_e62302 * assign48550_e62302))),)
    } else {
        (var_dctg__blk1318, var_dctg__blk1318_dn5, var_dctg__blk1318_dn6, var_dctg__blk1318_dn7, var_dctg__blk1318_dn8,)
    }
};
        var_dctg__blk1318 = assign48550_e62305;
        var_dctg__blk1318_dn5 = assign48550_e62305_d_n5;
        var_dctg__blk1318_dn6 = assign48550_e62305_d_n6;
        var_dctg__blk1318_dn7 = assign48550_e62305_d_n7;
        var_dctg__blk1318_dn8 = assign48550_e62305_d_n8;
        var_dctg__blk1318_rv = 0.0;

        let (assign48560_e62315, assign48560_e62315_d_n5, assign48560_e62315_d_n6, assign48560_e62315_d_n7, assign48560_e62315_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48560_e62312: f64 = (var_ct_t * var_dctg__blk1318);
        let assign48560_e62313: f64 = (1.0 + assign48560_e62312);
        (assign48560_e62313, (var_ct_t * var_dctg__blk1318_dn5), (var_ct_t * var_dctg__blk1318_dn6), (var_ct_t * var_dctg__blk1318_dn7), (var_ct_t * var_dctg__blk1318_dn8),)
    } else {
        (var_ct_fact__blk1319, var_ct_fact__blk1319_dn5, var_ct_fact__blk1319_dn6, var_ct_fact__blk1319_dn7, var_ct_fact__blk1319_dn8,)
    }
};
        var_ct_fact__blk1319 = assign48560_e62315;
        var_ct_fact__blk1319_dn5 = assign48560_e62315_d_n5;
        var_ct_fact__blk1319_dn6 = assign48560_e62315_d_n6;
        var_ct_fact__blk1319_dn7 = assign48560_e62315_d_n7;
        var_ct_fact__blk1319_dn8 = assign48560_e62315_d_n8;
        var_ct_fact__blk1319_rv = 0.0;

        let (assign48570_e62323, assign48570_e62323_d_n5, assign48570_e62323_d_n6, assign48570_e62323_d_n7, assign48570_e62323_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48570_e62321: f64 = (var_phit * var_ct_fact__blk1319);
        (assign48570_e62321, (var_phit * var_ct_fact__blk1319_dn5), (var_phit * var_ct_fact__blk1319_dn6), (var_phit * var_ct_fact__blk1319_dn7), (var_phit * var_ct_fact__blk1319_dn8),)
    } else {
        (var_phitct__blk1320, var_phitct__blk1320_dn5, var_phitct__blk1320_dn6, var_phitct__blk1320_dn7, var_phitct__blk1320_dn8,)
    }
};
        var_phitct__blk1320 = assign48570_e62323;
        var_phitct__blk1320_dn5 = assign48570_e62323_d_n5;
        var_phitct__blk1320_dn6 = assign48570_e62323_d_n6;
        var_phitct__blk1320_dn7 = assign48570_e62323_d_n7;
        var_phitct__blk1320_dn8 = assign48570_e62323_d_n8;
        var_phitct__blk1320_rv = 0.0;

        let (assign48580_e62341, assign48580_e62341_d_n5, assign48580_e62341_d_n6, assign48580_e62341_d_n7, assign48580_e62341_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48580_e62331: f64 = (var_psced_i * var_vdsx);
        let assign48580_e62332: f64 = (1.0 + assign48580_e62331);
        let assign48580_e62333: f64 = (var_psce_i * assign48580_e62332);
        let assign48580_e62337: f64 = (var_psceb_i * var_vsbx__blk1306);
        let assign48580_e62338: f64 = (1.0 + assign48580_e62337);
        let assign48580_e62339: f64 = (assign48580_e62333 * assign48580_e62338);
        (assign48580_e62339, (assign48580_e62333 * (var_psceb_i * var_vsbx__blk1306_dn5)), (((var_psce_i * (var_psced_i * var_vdsx_dn6)) * assign48580_e62338) + (assign48580_e62333 * (var_psceb_i * var_vsbx__blk1306_dn6))), (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign48580_e62338) + (assign48580_e62333 * (var_psceb_i * var_vsbx__blk1306_dn7))), (assign48580_e62333 * (var_psceb_i * var_vsbx__blk1306_dn8)),)
    } else {
        (var_dphit1__blk1321, var_dphit1__blk1321_dn5, var_dphit1__blk1321_dn6, var_dphit1__blk1321_dn7, var_dphit1__blk1321_dn8,)
    }
};
        var_dphit1__blk1321 = assign48580_e62341;
        var_dphit1__blk1321_dn5 = assign48580_e62341_d_n5;
        var_dphit1__blk1321_dn6 = assign48580_e62341_d_n6;
        var_dphit1__blk1321_dn7 = assign48580_e62341_d_n7;
        var_dphit1__blk1321_dn8 = assign48580_e62341_d_n8;
        var_dphit1__blk1321_rv = 0.0;

        let (assign48590_e62351, assign48590_e62351_d_n5, assign48590_e62351_d_n6, assign48590_e62351_d_n7, assign48590_e62351_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48590_e62348: f64 = (1.0 + var_dphit1__blk1321);
        let assign48590_e62349: f64 = (var_phitct__blk1320 * assign48590_e62348);
        (assign48590_e62349, ((var_phitct__blk1320_dn5 * assign48590_e62348) + (var_phitct__blk1320 * var_dphit1__blk1321_dn5)), ((var_phitct__blk1320_dn6 * assign48590_e62348) + (var_phitct__blk1320 * var_dphit1__blk1321_dn6)), ((var_phitct__blk1320_dn7 * assign48590_e62348) + (var_phitct__blk1320 * var_dphit1__blk1321_dn7)), ((var_phitct__blk1320_dn8 * assign48590_e62348) + (var_phitct__blk1320 * var_dphit1__blk1321_dn8)),)
    } else {
        (var_phit1__blk1322, var_phit1__blk1322_dn5, var_phit1__blk1322_dn6, var_phit1__blk1322_dn7, var_phit1__blk1322_dn8,)
    }
};
        var_phit1__blk1322 = assign48590_e62351;
        var_phit1__blk1322_dn5 = assign48590_e62351_d_n5;
        var_phit1__blk1322_dn6 = assign48590_e62351_d_n6;
        var_phit1__blk1322_dn7 = assign48590_e62351_d_n7;
        var_phit1__blk1322_dn8 = assign48590_e62351_d_n8;
        var_phit1__blk1322_rv = 0.0;

        *var_aphi__blk1298_slot = var_aphi__blk1298;
        *var_aphi__blk1298_rv_slot = var_aphi__blk1298_rv;
        *var_ct_fact__blk1319_slot = var_ct_fact__blk1319;
        *var_ct_fact__blk1319_dn5_slot = var_ct_fact__blk1319_dn5;
        *var_ct_fact__blk1319_dn6_slot = var_ct_fact__blk1319_dn6;
        *var_ct_fact__blk1319_dn7_slot = var_ct_fact__blk1319_dn7;
        *var_ct_fact__blk1319_dn8_slot = var_ct_fact__blk1319_dn8;
        *var_ct_fact__blk1319_rv_slot = var_ct_fact__blk1319_rv;
        *var_dctg__blk1318_slot = var_dctg__blk1318;
        *var_dctg__blk1318_dn5_slot = var_dctg__blk1318_dn5;
        *var_dctg__blk1318_dn6_slot = var_dctg__blk1318_dn6;
        *var_dctg__blk1318_dn7_slot = var_dctg__blk1318_dn7;
        *var_dctg__blk1318_dn8_slot = var_dctg__blk1318_dn8;
        *var_dctg__blk1318_rv_slot = var_dctg__blk1318_rv;
        *var_dphit1__blk1321_slot = var_dphit1__blk1321;
        *var_dphit1__blk1321_dn5_slot = var_dphit1__blk1321_dn5;
        *var_dphit1__blk1321_dn6_slot = var_dphit1__blk1321_dn6;
        *var_dphit1__blk1321_dn7_slot = var_dphit1__blk1321_dn7;
        *var_dphit1__blk1321_dn8_slot = var_dphit1__blk1321_dn8;
        *var_dphit1__blk1321_rv_slot = var_dphit1__blk1321_rv;
        *var_g_0__blk1299_slot = var_g_0__blk1299;
        *var_g_0__blk1299_rv_slot = var_g_0__blk1299_rv;
        *var_guard1459_slot = var_guard1459;
        *var_guard1459_rv_slot = var_guard1459_rv;
        *var_guard1460_slot = var_guard1460;
        *var_guard1460_rv_slot = var_guard1460_rv;
        *var_phib__blk1297_slot = var_phib__blk1297;
        *var_phib__blk1297_rv_slot = var_phib__blk1297_rv;
        *var_phit1__blk1322_slot = var_phit1__blk1322;
        *var_phit1__blk1322_dn5_slot = var_phit1__blk1322_dn5;
        *var_phit1__blk1322_dn6_slot = var_phit1__blk1322_dn6;
        *var_phit1__blk1322_dn7_slot = var_phit1__blk1322_dn7;
        *var_phit1__blk1322_dn8_slot = var_phit1__blk1322_dn8;
        *var_phit1__blk1322_rv_slot = var_phit1__blk1322_rv;
        *var_phitct__blk1320_slot = var_phitct__blk1320;
        *var_phitct__blk1320_dn5_slot = var_phitct__blk1320_dn5;
        *var_phitct__blk1320_dn6_slot = var_phitct__blk1320_dn6;
        *var_phitct__blk1320_dn7_slot = var_phitct__blk1320_dn7;
        *var_phitct__blk1320_dn8_slot = var_phitct__blk1320_dn8;
        *var_phitct__blk1320_rv_slot = var_phitct__blk1320_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_vgb1__blk1304_slot = var_vgb1__blk1304;
        *var_vgb1__blk1304_dn5_slot = var_vgb1__blk1304_dn5;
        *var_vgb1__blk1304_dn6_slot = var_vgb1__blk1304_dn6;
        *var_vgb1__blk1304_dn7_slot = var_vgb1__blk1304_dn7;
        *var_vgb1__blk1304_dn8_slot = var_vgb1__blk1304_dn8;
        *var_vgb1__blk1304_rv_slot = var_vgb1__blk1304_rv;
        *var_vsbstar__blk1301_slot = var_vsbstar__blk1301;
        *var_vsbstar__blk1301_dn5_slot = var_vsbstar__blk1301_dn5;
        *var_vsbstar__blk1301_dn6_slot = var_vsbstar__blk1301_dn6;
        *var_vsbstar__blk1301_dn7_slot = var_vsbstar__blk1301_dn7;
        *var_vsbstar__blk1301_dn8_slot = var_vsbstar__blk1301_dn8;
        *var_vsbstar__blk1301_rv_slot = var_vsbstar__blk1301_rv;
        *var_vsbx__blk1306_slot = var_vsbx__blk1306;
        *var_vsbx__blk1306_dn5_slot = var_vsbx__blk1306_dn5;
        *var_vsbx__blk1306_dn6_slot = var_vsbx__blk1306_dn6;
        *var_vsbx__blk1306_dn7_slot = var_vsbx__blk1306_dn7;
        *var_vsbx__blk1306_dn8_slot = var_vsbx__blk1306_dn8;
        *var_vsbx__blk1306_rv_slot = var_vsbx__blk1306_rv;
        *var_xbct__blk1309_slot = var_xbct__blk1309;
        *var_xbct__blk1309_rv_slot = var_xbct__blk1309_rv;
        *var_xct__blk1317_slot = var_xct__blk1317;
        *var_xct__blk1317_dn5_slot = var_xct__blk1317_dn5;
        *var_xct__blk1317_dn6_slot = var_xct__blk1317_dn6;
        *var_xct__blk1317_dn7_slot = var_xct__blk1317_dn7;
        *var_xct__blk1317_dn8_slot = var_xct__blk1317_dn8;
        *var_xct__blk1317_rv_slot = var_xct__blk1317_rv;
        *var_xctmax__blk1313_slot = var_xctmax__blk1313;
        *var_xctmax__blk1313_rv_slot = var_xctmax__blk1313_rv;
        *var_xgct__blk1311_slot = var_xgct__blk1311;
        *var_xgct__blk1311_dn5_slot = var_xgct__blk1311_dn5;
        *var_xgct__blk1311_dn6_slot = var_xgct__blk1311_dn6;
        *var_xgct__blk1311_dn7_slot = var_xgct__blk1311_dn7;
        *var_xgct__blk1311_dn8_slot = var_xgct__blk1311_dn8;
        *var_xgct__blk1311_rv_slot = var_xgct__blk1311_rv;
        *var_xmict__blk1315_slot = var_xmict__blk1315;
        *var_xmict__blk1315_dn5_slot = var_xmict__blk1315_dn5;
        *var_xmict__blk1315_dn6_slot = var_xmict__blk1315_dn6;
        *var_xmict__blk1315_dn7_slot = var_xmict__blk1315_dn7;
        *var_xmict__blk1315_dn8_slot = var_xmict__blk1315_dn8;
        *var_xmict__blk1315_rv_slot = var_xmict__blk1315_rv;
        *var_xnct__blk1314_slot = var_xnct__blk1314;
        *var_xnct__blk1314_dn5_slot = var_xnct__blk1314_dn5;
        *var_xnct__blk1314_dn6_slot = var_xnct__blk1314_dn6;
        *var_xnct__blk1314_dn7_slot = var_xnct__blk1314_dn7;
        *var_xnct__blk1314_dn8_slot = var_xnct__blk1314_dn8;
        *var_xnct__blk1314_rv_slot = var_xnct__blk1314_rv;
        *var_xsbstar__blk1310_slot = var_xsbstar__blk1310;
        *var_xsbstar__blk1310_dn5_slot = var_xsbstar__blk1310_dn5;
        *var_xsbstar__blk1310_dn6_slot = var_xsbstar__blk1310_dn6;
        *var_xsbstar__blk1310_dn7_slot = var_xsbstar__blk1310_dn7;
        *var_xsbstar__blk1310_dn8_slot = var_xsbstar__blk1310_dn8;
        *var_xsbstar__blk1310_rv_slot = var_xsbstar__blk1310_rv;
        *var_xsubct__blk1316_slot = var_xsubct__blk1316;
        *var_xsubct__blk1316_dn5_slot = var_xsubct__blk1316_dn5;
        *var_xsubct__blk1316_dn6_slot = var_xsubct__blk1316_dn6;
        *var_xsubct__blk1316_dn7_slot = var_xsubct__blk1316_dn7;
        *var_xsubct__blk1316_dn8_slot = var_xsubct__blk1316_dn8;
        *var_xsubct__blk1316_rv_slot = var_xsubct__blk1316_rv;
        *var_xwict__blk1312_slot = var_xwict__blk1312;
        *var_xwict__blk1312_dn5_slot = var_xwict__blk1312_dn5;
        *var_xwict__blk1312_dn6_slot = var_xwict__blk1312_dn6;
        *var_xwict__blk1312_dn7_slot = var_xwict__blk1312_dn7;
        *var_xwict__blk1312_dn8_slot = var_xwict__blk1312_dn8;
        *var_xwict__blk1312_rv_slot = var_xwict__blk1312_rv;
    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        var_aphi__blk1298: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_g_0__blk1299: f64,
        var_guard1456: f64,
        var_guard1457: f64,
        var_phib__blk1297: f64,
        var_phit: f64,
        var_phit1__blk1322: f64,
        var_phit1__blk1322_dn5: f64,
        var_phit1__blk1322_dn6: f64,
        var_phit1__blk1322_dn7: f64,
        var_phit1__blk1322_dn8: f64,
        var_v_xb__blk1300: f64,
        var_v_xb__blk1300_dn6: f64,
        var_v_xb__blk1300_dn7: f64,
        var_v_xb__blk1300_dn8: f64,
        var_vdsx: f64,
        var_vdsx_dn6: f64,
        var_vdsx_dn7: f64,
        var_vgb1__blk1304: f64,
        var_vgb1__blk1304_dn5: f64,
        var_vgb1__blk1304_dn6: f64,
        var_vgb1__blk1304_dn7: f64,
        var_vgb1__blk1304_dn8: f64,
        var_vsbstar__blk1301: f64,
        var_vsbstar__blk1301_dn5: f64,
        var_vsbstar__blk1301_dn6: f64,
        var_vsbstar__blk1301_dn7: f64,
        var_vsbstar__blk1301_dn8: f64,
        var_vsbx__blk1306: f64,
        var_vsbx__blk1306_dn5: f64,
        var_vsbx__blk1306_dn6: f64,
        var_vsbx__blk1306_dn7: f64,
        var_vsbx__blk1306_dn8: f64,
        var_delphib__blk1328_slot: &mut f64,
        var_delphib__blk1328_dn5_slot: &mut f64,
        var_delphib__blk1328_dn6_slot: &mut f64,
        var_delphib__blk1328_dn7_slot: &mut f64,
        var_delphib__blk1328_dn8_slot: &mut f64,
        var_delphib__blk1328_rv_slot: &mut f64,
        var_delta_ns__blk1347_slot: &mut f64,
        var_delta_ns__blk1347_dn5_slot: &mut f64,
        var_delta_ns__blk1347_dn6_slot: &mut f64,
        var_delta_ns__blk1347_dn7_slot: &mut f64,
        var_delta_ns__blk1347_dn8_slot: &mut f64,
        var_delta_ns__blk1347_rv_slot: &mut f64,
        var_delxb__blk1330_slot: &mut f64,
        var_delxb__blk1330_dn5_slot: &mut f64,
        var_delxb__blk1330_dn6_slot: &mut f64,
        var_delxb__blk1330_dn7_slot: &mut f64,
        var_delxb__blk1330_dn8_slot: &mut f64,
        var_delxb__blk1330_rv_slot: &mut f64,
        var_fscr__blk1342_slot: &mut f64,
        var_fscr__blk1342_dn5_slot: &mut f64,
        var_fscr__blk1342_dn6_slot: &mut f64,
        var_fscr__blk1342_dn7_slot: &mut f64,
        var_fscr__blk1342_dn8_slot: &mut f64,
        var_fscr__blk1342_rv_slot: &mut f64,
        var_gf2__blk1308_slot: &mut f64,
        var_gf2__blk1308_dn5_slot: &mut f64,
        var_gf2__blk1308_dn6_slot: &mut f64,
        var_gf2__blk1308_dn7_slot: &mut f64,
        var_gf2__blk1308_dn8_slot: &mut f64,
        var_gf2__blk1308_rv_slot: &mut f64,
        var_gf__blk1307_slot: &mut f64,
        var_gf__blk1307_dn5_slot: &mut f64,
        var_gf__blk1307_dn6_slot: &mut f64,
        var_gf__blk1307_dn7_slot: &mut f64,
        var_gf__blk1307_dn8_slot: &mut f64,
        var_gf__blk1307_rv_slot: &mut f64,
        var_guard1461_slot: &mut f64,
        var_guard1461_rv_slot: &mut f64,
        var_guard1462_slot: &mut f64,
        var_guard1462_rv_slot: &mut f64,
        var_guard1463_slot: &mut f64,
        var_guard1463_rv_slot: &mut f64,
        var_guard1464_slot: &mut f64,
        var_guard1464_rv_slot: &mut f64,
        var_inv_gf2__blk1324_slot: &mut f64,
        var_inv_gf2__blk1324_dn5_slot: &mut f64,
        var_inv_gf2__blk1324_dn6_slot: &mut f64,
        var_inv_gf2__blk1324_dn7_slot: &mut f64,
        var_inv_gf2__blk1324_dn8_slot: &mut f64,
        var_inv_gf2__blk1324_rv_slot: &mut f64,
        var_inv_phit1__blk1323_slot: &mut f64,
        var_inv_phit1__blk1323_dn5_slot: &mut f64,
        var_inv_phit1__blk1323_dn6_slot: &mut f64,
        var_inv_phit1__blk1323_dn7_slot: &mut f64,
        var_inv_phit1__blk1323_dn8_slot: &mut f64,
        var_inv_phit1__blk1323_rv_slot: &mut f64,
        var_nscr__blk1333_slot: &mut f64,
        var_nscr__blk1333_dn5_slot: &mut f64,
        var_nscr__blk1333_dn6_slot: &mut f64,
        var_nscr__blk1333_dn7_slot: &mut f64,
        var_nscr__blk1333_dn8_slot: &mut f64,
        var_nscr__blk1333_rv_slot: &mut f64,
        var_qbscr__blk1341_slot: &mut f64,
        var_qbscr__blk1341_dn5_slot: &mut f64,
        var_qbscr__blk1341_dn6_slot: &mut f64,
        var_qbscr__blk1341_dn7_slot: &mut f64,
        var_qbscr__blk1341_dn8_slot: &mut f64,
        var_qbscr__blk1341_rv_slot: &mut f64,
        var_qiscr0si__blk1337_slot: &mut f64,
        var_qiscr0si__blk1337_dn5_slot: &mut f64,
        var_qiscr0si__blk1337_dn6_slot: &mut f64,
        var_qiscr0si__blk1337_dn7_slot: &mut f64,
        var_qiscr0si__blk1337_dn8_slot: &mut f64,
        var_qiscr0si__blk1337_rv_slot: &mut f64,
        var_qiscr__blk1340_slot: &mut f64,
        var_qiscr__blk1340_dn5_slot: &mut f64,
        var_qiscr__blk1340_dn6_slot: &mut f64,
        var_qiscr__blk1340_dn7_slot: &mut f64,
        var_qiscr__blk1340_dn8_slot: &mut f64,
        var_qiscr__blk1340_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_temp__blk936_slot: &mut f64,
        var_temp__blk936_dn5_slot: &mut f64,
        var_temp__blk936_dn6_slot: &mut f64,
        var_temp__blk936_dn7_slot: &mut f64,
        var_temp__blk936_dn8_slot: &mut f64,
        var_temp__blk936_rv_slot: &mut f64,
        var_ux__blk1325_slot: &mut f64,
        var_ux__blk1325_dn5_slot: &mut f64,
        var_ux__blk1325_dn6_slot: &mut f64,
        var_ux__blk1325_dn7_slot: &mut f64,
        var_ux__blk1325_dn8_slot: &mut f64,
        var_ux__blk1325_rv_slot: &mut f64,
        var_vdsp__blk1327_slot: &mut f64,
        var_vdsp__blk1327_dn6_slot: &mut f64,
        var_vdsp__blk1327_dn7_slot: &mut f64,
        var_vdsp__blk1327_rv_slot: &mut f64,
        var_xb__blk1329_slot: &mut f64,
        var_xb__blk1329_dn5_slot: &mut f64,
        var_xb__blk1329_dn6_slot: &mut f64,
        var_xb__blk1329_dn7_slot: &mut f64,
        var_xb__blk1329_dn8_slot: &mut f64,
        var_xb__blk1329_rv_slot: &mut f64,
        var_xg__blk1326_slot: &mut f64,
        var_xg__blk1326_dn5_slot: &mut f64,
        var_xg__blk1326_dn6_slot: &mut f64,
        var_xg__blk1326_dn7_slot: &mut f64,
        var_xg__blk1326_dn8_slot: &mut f64,
        var_xg__blk1326_rv_slot: &mut f64,
        var_xgtscr0__blk1336_slot: &mut f64,
        var_xgtscr0__blk1336_dn5_slot: &mut f64,
        var_xgtscr0__blk1336_dn6_slot: &mut f64,
        var_xgtscr0__blk1336_dn7_slot: &mut f64,
        var_xgtscr0__blk1336_dn8_slot: &mut f64,
        var_xgtscr0__blk1336_rv_slot: &mut f64,
        var_xgtscr__blk1335_slot: &mut f64,
        var_xgtscr__blk1335_dn5_slot: &mut f64,
        var_xgtscr__blk1335_dn6_slot: &mut f64,
        var_xgtscr__blk1335_dn7_slot: &mut f64,
        var_xgtscr__blk1335_dn8_slot: &mut f64,
        var_xgtscr__blk1335_rv_slot: &mut f64,
        var_xn_s__blk1332_slot: &mut f64,
        var_xn_s__blk1332_dn5_slot: &mut f64,
        var_xn_s__blk1332_dn6_slot: &mut f64,
        var_xn_s__blk1332_dn7_slot: &mut f64,
        var_xn_s__blk1332_dn8_slot: &mut f64,
        var_xn_s__blk1332_rv_slot: &mut f64,
        var_xno_s__blk1331_slot: &mut f64,
        var_xno_s__blk1331_dn5_slot: &mut f64,
        var_xno_s__blk1331_dn6_slot: &mut f64,
        var_xno_s__blk1331_dn7_slot: &mut f64,
        var_xno_s__blk1331_dn8_slot: &mut f64,
        var_xno_s__blk1331_rv_slot: &mut f64,
        var_xthscr__blk1334_slot: &mut f64,
        var_xthscr__blk1334_dn5_slot: &mut f64,
        var_xthscr__blk1334_dn6_slot: &mut f64,
        var_xthscr__blk1334_dn7_slot: &mut f64,
        var_xthscr__blk1334_dn8_slot: &mut f64,
        var_xthscr__blk1334_rv_slot: &mut f64,
    ) {
        let mut var_delphib__blk1328: f64 = *var_delphib__blk1328_slot;
        let mut var_delphib__blk1328_dn5: f64 = *var_delphib__blk1328_dn5_slot;
        let mut var_delphib__blk1328_dn6: f64 = *var_delphib__blk1328_dn6_slot;
        let mut var_delphib__blk1328_dn7: f64 = *var_delphib__blk1328_dn7_slot;
        let mut var_delphib__blk1328_dn8: f64 = *var_delphib__blk1328_dn8_slot;
        let mut var_delphib__blk1328_rv: f64 = *var_delphib__blk1328_rv_slot;
        let mut var_delta_ns__blk1347: f64 = *var_delta_ns__blk1347_slot;
        let mut var_delta_ns__blk1347_dn5: f64 = *var_delta_ns__blk1347_dn5_slot;
        let mut var_delta_ns__blk1347_dn6: f64 = *var_delta_ns__blk1347_dn6_slot;
        let mut var_delta_ns__blk1347_dn7: f64 = *var_delta_ns__blk1347_dn7_slot;
        let mut var_delta_ns__blk1347_dn8: f64 = *var_delta_ns__blk1347_dn8_slot;
        let mut var_delta_ns__blk1347_rv: f64 = *var_delta_ns__blk1347_rv_slot;
        let mut var_delxb__blk1330: f64 = *var_delxb__blk1330_slot;
        let mut var_delxb__blk1330_dn5: f64 = *var_delxb__blk1330_dn5_slot;
        let mut var_delxb__blk1330_dn6: f64 = *var_delxb__blk1330_dn6_slot;
        let mut var_delxb__blk1330_dn7: f64 = *var_delxb__blk1330_dn7_slot;
        let mut var_delxb__blk1330_dn8: f64 = *var_delxb__blk1330_dn8_slot;
        let mut var_delxb__blk1330_rv: f64 = *var_delxb__blk1330_rv_slot;
        let mut var_fscr__blk1342: f64 = *var_fscr__blk1342_slot;
        let mut var_fscr__blk1342_dn5: f64 = *var_fscr__blk1342_dn5_slot;
        let mut var_fscr__blk1342_dn6: f64 = *var_fscr__blk1342_dn6_slot;
        let mut var_fscr__blk1342_dn7: f64 = *var_fscr__blk1342_dn7_slot;
        let mut var_fscr__blk1342_dn8: f64 = *var_fscr__blk1342_dn8_slot;
        let mut var_fscr__blk1342_rv: f64 = *var_fscr__blk1342_rv_slot;
        let mut var_gf2__blk1308: f64 = *var_gf2__blk1308_slot;
        let mut var_gf2__blk1308_dn5: f64 = *var_gf2__blk1308_dn5_slot;
        let mut var_gf2__blk1308_dn6: f64 = *var_gf2__blk1308_dn6_slot;
        let mut var_gf2__blk1308_dn7: f64 = *var_gf2__blk1308_dn7_slot;
        let mut var_gf2__blk1308_dn8: f64 = *var_gf2__blk1308_dn8_slot;
        let mut var_gf2__blk1308_rv: f64 = *var_gf2__blk1308_rv_slot;
        let mut var_gf__blk1307: f64 = *var_gf__blk1307_slot;
        let mut var_gf__blk1307_dn5: f64 = *var_gf__blk1307_dn5_slot;
        let mut var_gf__blk1307_dn6: f64 = *var_gf__blk1307_dn6_slot;
        let mut var_gf__blk1307_dn7: f64 = *var_gf__blk1307_dn7_slot;
        let mut var_gf__blk1307_dn8: f64 = *var_gf__blk1307_dn8_slot;
        let mut var_gf__blk1307_rv: f64 = *var_gf__blk1307_rv_slot;
        let mut var_guard1461: f64 = *var_guard1461_slot;
        let mut var_guard1461_rv: f64 = *var_guard1461_rv_slot;
        let mut var_guard1462: f64 = *var_guard1462_slot;
        let mut var_guard1462_rv: f64 = *var_guard1462_rv_slot;
        let mut var_guard1463: f64 = *var_guard1463_slot;
        let mut var_guard1463_rv: f64 = *var_guard1463_rv_slot;
        let mut var_guard1464: f64 = *var_guard1464_slot;
        let mut var_guard1464_rv: f64 = *var_guard1464_rv_slot;
        let mut var_inv_gf2__blk1324: f64 = *var_inv_gf2__blk1324_slot;
        let mut var_inv_gf2__blk1324_dn5: f64 = *var_inv_gf2__blk1324_dn5_slot;
        let mut var_inv_gf2__blk1324_dn6: f64 = *var_inv_gf2__blk1324_dn6_slot;
        let mut var_inv_gf2__blk1324_dn7: f64 = *var_inv_gf2__blk1324_dn7_slot;
        let mut var_inv_gf2__blk1324_dn8: f64 = *var_inv_gf2__blk1324_dn8_slot;
        let mut var_inv_gf2__blk1324_rv: f64 = *var_inv_gf2__blk1324_rv_slot;
        let mut var_inv_phit1__blk1323: f64 = *var_inv_phit1__blk1323_slot;
        let mut var_inv_phit1__blk1323_dn5: f64 = *var_inv_phit1__blk1323_dn5_slot;
        let mut var_inv_phit1__blk1323_dn6: f64 = *var_inv_phit1__blk1323_dn6_slot;
        let mut var_inv_phit1__blk1323_dn7: f64 = *var_inv_phit1__blk1323_dn7_slot;
        let mut var_inv_phit1__blk1323_dn8: f64 = *var_inv_phit1__blk1323_dn8_slot;
        let mut var_inv_phit1__blk1323_rv: f64 = *var_inv_phit1__blk1323_rv_slot;
        let mut var_nscr__blk1333: f64 = *var_nscr__blk1333_slot;
        let mut var_nscr__blk1333_dn5: f64 = *var_nscr__blk1333_dn5_slot;
        let mut var_nscr__blk1333_dn6: f64 = *var_nscr__blk1333_dn6_slot;
        let mut var_nscr__blk1333_dn7: f64 = *var_nscr__blk1333_dn7_slot;
        let mut var_nscr__blk1333_dn8: f64 = *var_nscr__blk1333_dn8_slot;
        let mut var_nscr__blk1333_rv: f64 = *var_nscr__blk1333_rv_slot;
        let mut var_qbscr__blk1341: f64 = *var_qbscr__blk1341_slot;
        let mut var_qbscr__blk1341_dn5: f64 = *var_qbscr__blk1341_dn5_slot;
        let mut var_qbscr__blk1341_dn6: f64 = *var_qbscr__blk1341_dn6_slot;
        let mut var_qbscr__blk1341_dn7: f64 = *var_qbscr__blk1341_dn7_slot;
        let mut var_qbscr__blk1341_dn8: f64 = *var_qbscr__blk1341_dn8_slot;
        let mut var_qbscr__blk1341_rv: f64 = *var_qbscr__blk1341_rv_slot;
        let mut var_qiscr0si__blk1337: f64 = *var_qiscr0si__blk1337_slot;
        let mut var_qiscr0si__blk1337_dn5: f64 = *var_qiscr0si__blk1337_dn5_slot;
        let mut var_qiscr0si__blk1337_dn6: f64 = *var_qiscr0si__blk1337_dn6_slot;
        let mut var_qiscr0si__blk1337_dn7: f64 = *var_qiscr0si__blk1337_dn7_slot;
        let mut var_qiscr0si__blk1337_dn8: f64 = *var_qiscr0si__blk1337_dn8_slot;
        let mut var_qiscr0si__blk1337_rv: f64 = *var_qiscr0si__blk1337_rv_slot;
        let mut var_qiscr__blk1340: f64 = *var_qiscr__blk1340_slot;
        let mut var_qiscr__blk1340_dn5: f64 = *var_qiscr__blk1340_dn5_slot;
        let mut var_qiscr__blk1340_dn6: f64 = *var_qiscr__blk1340_dn6_slot;
        let mut var_qiscr__blk1340_dn7: f64 = *var_qiscr__blk1340_dn7_slot;
        let mut var_qiscr__blk1340_dn8: f64 = *var_qiscr__blk1340_dn8_slot;
        let mut var_qiscr__blk1340_rv: f64 = *var_qiscr__blk1340_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_temp__blk936: f64 = *var_temp__blk936_slot;
        let mut var_temp__blk936_dn5: f64 = *var_temp__blk936_dn5_slot;
        let mut var_temp__blk936_dn6: f64 = *var_temp__blk936_dn6_slot;
        let mut var_temp__blk936_dn7: f64 = *var_temp__blk936_dn7_slot;
        let mut var_temp__blk936_dn8: f64 = *var_temp__blk936_dn8_slot;
        let mut var_temp__blk936_rv: f64 = *var_temp__blk936_rv_slot;
        let mut var_ux__blk1325: f64 = *var_ux__blk1325_slot;
        let mut var_ux__blk1325_dn5: f64 = *var_ux__blk1325_dn5_slot;
        let mut var_ux__blk1325_dn6: f64 = *var_ux__blk1325_dn6_slot;
        let mut var_ux__blk1325_dn7: f64 = *var_ux__blk1325_dn7_slot;
        let mut var_ux__blk1325_dn8: f64 = *var_ux__blk1325_dn8_slot;
        let mut var_ux__blk1325_rv: f64 = *var_ux__blk1325_rv_slot;
        let mut var_vdsp__blk1327: f64 = *var_vdsp__blk1327_slot;
        let mut var_vdsp__blk1327_dn6: f64 = *var_vdsp__blk1327_dn6_slot;
        let mut var_vdsp__blk1327_dn7: f64 = *var_vdsp__blk1327_dn7_slot;
        let mut var_vdsp__blk1327_rv: f64 = *var_vdsp__blk1327_rv_slot;
        let mut var_xb__blk1329: f64 = *var_xb__blk1329_slot;
        let mut var_xb__blk1329_dn5: f64 = *var_xb__blk1329_dn5_slot;
        let mut var_xb__blk1329_dn6: f64 = *var_xb__blk1329_dn6_slot;
        let mut var_xb__blk1329_dn7: f64 = *var_xb__blk1329_dn7_slot;
        let mut var_xb__blk1329_dn8: f64 = *var_xb__blk1329_dn8_slot;
        let mut var_xb__blk1329_rv: f64 = *var_xb__blk1329_rv_slot;
        let mut var_xg__blk1326: f64 = *var_xg__blk1326_slot;
        let mut var_xg__blk1326_dn5: f64 = *var_xg__blk1326_dn5_slot;
        let mut var_xg__blk1326_dn6: f64 = *var_xg__blk1326_dn6_slot;
        let mut var_xg__blk1326_dn7: f64 = *var_xg__blk1326_dn7_slot;
        let mut var_xg__blk1326_dn8: f64 = *var_xg__blk1326_dn8_slot;
        let mut var_xg__blk1326_rv: f64 = *var_xg__blk1326_rv_slot;
        let mut var_xgtscr0__blk1336: f64 = *var_xgtscr0__blk1336_slot;
        let mut var_xgtscr0__blk1336_dn5: f64 = *var_xgtscr0__blk1336_dn5_slot;
        let mut var_xgtscr0__blk1336_dn6: f64 = *var_xgtscr0__blk1336_dn6_slot;
        let mut var_xgtscr0__blk1336_dn7: f64 = *var_xgtscr0__blk1336_dn7_slot;
        let mut var_xgtscr0__blk1336_dn8: f64 = *var_xgtscr0__blk1336_dn8_slot;
        let mut var_xgtscr0__blk1336_rv: f64 = *var_xgtscr0__blk1336_rv_slot;
        let mut var_xgtscr__blk1335: f64 = *var_xgtscr__blk1335_slot;
        let mut var_xgtscr__blk1335_dn5: f64 = *var_xgtscr__blk1335_dn5_slot;
        let mut var_xgtscr__blk1335_dn6: f64 = *var_xgtscr__blk1335_dn6_slot;
        let mut var_xgtscr__blk1335_dn7: f64 = *var_xgtscr__blk1335_dn7_slot;
        let mut var_xgtscr__blk1335_dn8: f64 = *var_xgtscr__blk1335_dn8_slot;
        let mut var_xgtscr__blk1335_rv: f64 = *var_xgtscr__blk1335_rv_slot;
        let mut var_xn_s__blk1332: f64 = *var_xn_s__blk1332_slot;
        let mut var_xn_s__blk1332_dn5: f64 = *var_xn_s__blk1332_dn5_slot;
        let mut var_xn_s__blk1332_dn6: f64 = *var_xn_s__blk1332_dn6_slot;
        let mut var_xn_s__blk1332_dn7: f64 = *var_xn_s__blk1332_dn7_slot;
        let mut var_xn_s__blk1332_dn8: f64 = *var_xn_s__blk1332_dn8_slot;
        let mut var_xn_s__blk1332_rv: f64 = *var_xn_s__blk1332_rv_slot;
        let mut var_xno_s__blk1331: f64 = *var_xno_s__blk1331_slot;
        let mut var_xno_s__blk1331_dn5: f64 = *var_xno_s__blk1331_dn5_slot;
        let mut var_xno_s__blk1331_dn6: f64 = *var_xno_s__blk1331_dn6_slot;
        let mut var_xno_s__blk1331_dn7: f64 = *var_xno_s__blk1331_dn7_slot;
        let mut var_xno_s__blk1331_dn8: f64 = *var_xno_s__blk1331_dn8_slot;
        let mut var_xno_s__blk1331_rv: f64 = *var_xno_s__blk1331_rv_slot;
        let mut var_xthscr__blk1334: f64 = *var_xthscr__blk1334_slot;
        let mut var_xthscr__blk1334_dn5: f64 = *var_xthscr__blk1334_dn5_slot;
        let mut var_xthscr__blk1334_dn6: f64 = *var_xthscr__blk1334_dn6_slot;
        let mut var_xthscr__blk1334_dn7: f64 = *var_xthscr__blk1334_dn7_slot;
        let mut var_xthscr__blk1334_dn8: f64 = *var_xthscr__blk1334_dn8_slot;
        let mut var_xthscr__blk1334_rv: f64 = *var_xthscr__blk1334_rv_slot;

        let (assign48600_e62359, assign48600_e62359_d_n5, assign48600_e62359_d_n6, assign48600_e62359_d_n7, assign48600_e62359_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48600_e62357: f64 = (1.0 / var_phit1__blk1322);
        (assign48600_e62357, (-(var_phit1__blk1322_dn5 / (var_phit1__blk1322 * var_phit1__blk1322))), (-(var_phit1__blk1322_dn6 / (var_phit1__blk1322 * var_phit1__blk1322))), (-(var_phit1__blk1322_dn7 / (var_phit1__blk1322 * var_phit1__blk1322))), (-(var_phit1__blk1322_dn8 / (var_phit1__blk1322 * var_phit1__blk1322))),)
    } else {
        (var_inv_phit1__blk1323, var_inv_phit1__blk1323_dn5, var_inv_phit1__blk1323_dn6, var_inv_phit1__blk1323_dn7, var_inv_phit1__blk1323_dn8,)
    }
};
        var_inv_phit1__blk1323 = assign48600_e62359;
        var_inv_phit1__blk1323_dn5 = assign48600_e62359_d_n5;
        var_inv_phit1__blk1323_dn6 = assign48600_e62359_d_n6;
        var_inv_phit1__blk1323_dn7 = assign48600_e62359_d_n7;
        var_inv_phit1__blk1323_dn8 = assign48600_e62359_d_n8;
        var_inv_phit1__blk1323_rv = 0.0;

        let (assign48610_e62370, assign48610_e62370_d_n5, assign48610_e62370_d_n6, assign48610_e62370_d_n7, assign48610_e62370_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48610_e62366: f64 = (var_phit * var_inv_phit1__blk1323);
        let assign48610_e62367: f64 = (assign48610_e62366).sqrt();
        let assign48610_e62368: f64 = (var_g_0__blk1299 * assign48610_e62367);
        (assign48610_e62368, (var_g_0__blk1299 * ((var_phit * var_inv_phit1__blk1323_dn5) / (2.0 * assign48610_e62367))), (var_g_0__blk1299 * ((var_phit * var_inv_phit1__blk1323_dn6) / (2.0 * assign48610_e62367))), (var_g_0__blk1299 * ((var_phit * var_inv_phit1__blk1323_dn7) / (2.0 * assign48610_e62367))), (var_g_0__blk1299 * ((var_phit * var_inv_phit1__blk1323_dn8) / (2.0 * assign48610_e62367))),)
    } else {
        (var_gf__blk1307, var_gf__blk1307_dn5, var_gf__blk1307_dn6, var_gf__blk1307_dn7, var_gf__blk1307_dn8,)
    }
};
        var_gf__blk1307 = assign48610_e62370;
        var_gf__blk1307_dn5 = assign48610_e62370_d_n5;
        var_gf__blk1307_dn6 = assign48610_e62370_d_n6;
        var_gf__blk1307_dn7 = assign48610_e62370_d_n7;
        var_gf__blk1307_dn8 = assign48610_e62370_d_n8;
        var_gf__blk1307_rv = 0.0;

        let (assign48620_e62378, assign48620_e62378_d_n5, assign48620_e62378_d_n6, assign48620_e62378_d_n7, assign48620_e62378_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48620_e62376: f64 = (var_gf__blk1307 * var_gf__blk1307);
        (assign48620_e62376, ((var_gf__blk1307_dn5 * var_gf__blk1307) + (var_gf__blk1307 * var_gf__blk1307_dn5)), ((var_gf__blk1307_dn6 * var_gf__blk1307) + (var_gf__blk1307 * var_gf__blk1307_dn6)), ((var_gf__blk1307_dn7 * var_gf__blk1307) + (var_gf__blk1307 * var_gf__blk1307_dn7)), ((var_gf__blk1307_dn8 * var_gf__blk1307) + (var_gf__blk1307 * var_gf__blk1307_dn8)),)
    } else {
        (var_gf2__blk1308, var_gf2__blk1308_dn5, var_gf2__blk1308_dn6, var_gf2__blk1308_dn7, var_gf2__blk1308_dn8,)
    }
};
        var_gf2__blk1308 = assign48620_e62378;
        var_gf2__blk1308_dn5 = assign48620_e62378_d_n5;
        var_gf2__blk1308_dn6 = assign48620_e62378_d_n6;
        var_gf2__blk1308_dn7 = assign48620_e62378_d_n7;
        var_gf2__blk1308_dn8 = assign48620_e62378_d_n8;
        var_gf2__blk1308_rv = 0.0;

        let (assign48630_e62386, assign48630_e62386_d_n5, assign48630_e62386_d_n6, assign48630_e62386_d_n7, assign48630_e62386_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48630_e62384: f64 = (1.0 / var_gf2__blk1308);
        (assign48630_e62384, (-(var_gf2__blk1308_dn5 / (var_gf2__blk1308 * var_gf2__blk1308))), (-(var_gf2__blk1308_dn6 / (var_gf2__blk1308 * var_gf2__blk1308))), (-(var_gf2__blk1308_dn7 / (var_gf2__blk1308 * var_gf2__blk1308))), (-(var_gf2__blk1308_dn8 / (var_gf2__blk1308 * var_gf2__blk1308))),)
    } else {
        (var_inv_gf2__blk1324, var_inv_gf2__blk1324_dn5, var_inv_gf2__blk1324_dn6, var_inv_gf2__blk1324_dn7, var_inv_gf2__blk1324_dn8,)
    }
};
        var_inv_gf2__blk1324 = assign48630_e62386;
        var_inv_gf2__blk1324_dn5 = assign48630_e62386_d_n5;
        var_inv_gf2__blk1324_dn6 = assign48630_e62386_d_n6;
        var_inv_gf2__blk1324_dn7 = assign48630_e62386_d_n7;
        var_inv_gf2__blk1324_dn8 = assign48630_e62386_d_n8;
        var_inv_gf2__blk1324_rv = 0.0;

        let (assign48640_e62394, assign48640_e62394_d_n5, assign48640_e62394_d_n6, assign48640_e62394_d_n7, assign48640_e62394_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48640_e62392: f64 = (var_vsbstar__blk1301 * var_inv_phit1__blk1323);
        (assign48640_e62392, ((var_vsbstar__blk1301_dn5 * var_inv_phit1__blk1323) + (var_vsbstar__blk1301 * var_inv_phit1__blk1323_dn5)), ((var_vsbstar__blk1301_dn6 * var_inv_phit1__blk1323) + (var_vsbstar__blk1301 * var_inv_phit1__blk1323_dn6)), ((var_vsbstar__blk1301_dn7 * var_inv_phit1__blk1323) + (var_vsbstar__blk1301 * var_inv_phit1__blk1323_dn7)), ((var_vsbstar__blk1301_dn8 * var_inv_phit1__blk1323) + (var_vsbstar__blk1301 * var_inv_phit1__blk1323_dn8)),)
    } else {
        (var_ux__blk1325, var_ux__blk1325_dn5, var_ux__blk1325_dn6, var_ux__blk1325_dn7, var_ux__blk1325_dn8,)
    }
};
        var_ux__blk1325 = assign48640_e62394;
        var_ux__blk1325_dn5 = assign48640_e62394_d_n5;
        var_ux__blk1325_dn6 = assign48640_e62394_d_n6;
        var_ux__blk1325_dn7 = assign48640_e62394_d_n7;
        var_ux__blk1325_dn8 = assign48640_e62394_d_n8;
        var_ux__blk1325_rv = 0.0;

        let (assign48650_e62402, assign48650_e62402_d_n5, assign48650_e62402_d_n6, assign48650_e62402_d_n7, assign48650_e62402_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48650_e62400: f64 = (var_vgb1__blk1304 * var_inv_phit1__blk1323);
        (assign48650_e62400, ((var_vgb1__blk1304_dn5 * var_inv_phit1__blk1323) + (var_vgb1__blk1304 * var_inv_phit1__blk1323_dn5)), ((var_vgb1__blk1304_dn6 * var_inv_phit1__blk1323) + (var_vgb1__blk1304 * var_inv_phit1__blk1323_dn6)), ((var_vgb1__blk1304_dn7 * var_inv_phit1__blk1323) + (var_vgb1__blk1304 * var_inv_phit1__blk1323_dn7)), ((var_vgb1__blk1304_dn8 * var_inv_phit1__blk1323) + (var_vgb1__blk1304 * var_inv_phit1__blk1323_dn8)),)
    } else {
        (var_xg__blk1326, var_xg__blk1326_dn5, var_xg__blk1326_dn6, var_xg__blk1326_dn7, var_xg__blk1326_dn8,)
    }
};
        var_xg__blk1326 = assign48650_e62402;
        var_xg__blk1326_dn5 = assign48650_e62402_d_n5;
        var_xg__blk1326_dn6 = assign48650_e62402_d_n6;
        var_xg__blk1326_dn7 = assign48650_e62402_d_n7;
        var_xg__blk1326_dn8 = assign48650_e62402_d_n8;
        var_xg__blk1326_rv = 0.0;

        let (assign48660_e62419, assign48660_e62419_d_n6, assign48660_e62419_d_n7,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48660_e62408: f64 = (2.0 * var_vdsx);
        let assign48660_e62413: f64 = (var_cfd_i * var_vdsx);
        let assign48660_e62414: f64 = (1.0 + assign48660_e62413);
        let assign48660_e62415: f64 = (assign48660_e62414).sqrt();
        let assign48660_e62416: f64 = (1.0 + assign48660_e62415);
        let assign48660_e62417: f64 = (assign48660_e62408 / assign48660_e62416);
        (assign48660_e62417, ((((2.0 * var_vdsx_dn6) * assign48660_e62416) - (assign48660_e62408 * ((var_cfd_i * var_vdsx_dn6) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)), ((((2.0 * var_vdsx_dn7) * assign48660_e62416) - (assign48660_e62408 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign48660_e62415)))) / (assign48660_e62416 * assign48660_e62416)),)
    } else {
        (var_vdsp__blk1327, var_vdsp__blk1327_dn6, var_vdsp__blk1327_dn7,)
    }
};
        var_vdsp__blk1327 = assign48660_e62419;
        var_vdsp__blk1327_dn6 = assign48660_e62419_d_n6;
        var_vdsp__blk1327_dn7 = assign48660_e62419_d_n7;
        var_vdsp__blk1327_rv = 0.0;

        let (assign48670_e62433, assign48670_e62433_d_n5, assign48670_e62433_d_n6, assign48670_e62433_d_n7, assign48670_e62433_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48670_e62425: f64 = (var_cf_i * var_vdsp__blk1327);
        let assign48670_e62429: f64 = (var_cfb_i * var_vsbx__blk1306);
        let assign48670_e62430: f64 = (1.0 + assign48670_e62429);
        let assign48670_e62431: f64 = (assign48670_e62425 * assign48670_e62430);
        (assign48670_e62431, (assign48670_e62425 * (var_cfb_i * var_vsbx__blk1306_dn5)), (((var_cf_i * var_vdsp__blk1327_dn6) * assign48670_e62430) + (assign48670_e62425 * (var_cfb_i * var_vsbx__blk1306_dn6))), (((var_cf_i * var_vdsp__blk1327_dn7) * assign48670_e62430) + (assign48670_e62425 * (var_cfb_i * var_vsbx__blk1306_dn7))), (assign48670_e62425 * (var_cfb_i * var_vsbx__blk1306_dn8)),)
    } else {
        (var_delphib__blk1328, var_delphib__blk1328_dn5, var_delphib__blk1328_dn6, var_delphib__blk1328_dn7, var_delphib__blk1328_dn8,)
    }
};
        var_delphib__blk1328 = assign48670_e62433;
        var_delphib__blk1328_dn5 = assign48670_e62433_d_n5;
        var_delphib__blk1328_dn6 = assign48670_e62433_d_n6;
        var_delphib__blk1328_dn7 = assign48670_e62433_d_n7;
        var_delphib__blk1328_dn8 = assign48670_e62433_d_n8;
        var_delphib__blk1328_rv = 0.0;

        let (assign48680_e62441, assign48680_e62441_d_n5, assign48680_e62441_d_n6, assign48680_e62441_d_n7, assign48680_e62441_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48680_e62439: f64 = (var_phib__blk1297 * var_inv_phit1__blk1323);
        (assign48680_e62439, (var_phib__blk1297 * var_inv_phit1__blk1323_dn5), (var_phib__blk1297 * var_inv_phit1__blk1323_dn6), (var_phib__blk1297 * var_inv_phit1__blk1323_dn7), (var_phib__blk1297 * var_inv_phit1__blk1323_dn8),)
    } else {
        (var_xb__blk1329, var_xb__blk1329_dn5, var_xb__blk1329_dn6, var_xb__blk1329_dn7, var_xb__blk1329_dn8,)
    }
};
        var_xb__blk1329 = assign48680_e62441;
        var_xb__blk1329_dn5 = assign48680_e62441_d_n5;
        var_xb__blk1329_dn6 = assign48680_e62441_d_n6;
        var_xb__blk1329_dn7 = assign48680_e62441_d_n7;
        var_xb__blk1329_dn8 = assign48680_e62441_d_n8;
        var_xb__blk1329_rv = 0.0;

        let (assign48690_e62452, assign48690_e62452_d_n5, assign48690_e62452_d_n6, assign48690_e62452_d_n7, assign48690_e62452_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48690_e62447: f64 = (var_v_xb__blk1300 * var_v_xb__blk1300);
        let assign48690_e62449: f64 = (assign48690_e62447 + var_aphi__blk1298);
        let assign48690_e62450: f64 = (assign48690_e62449).sqrt();
        (assign48690_e62450, 0.0, (((var_v_xb__blk1300_dn6 * var_v_xb__blk1300) + (var_v_xb__blk1300 * var_v_xb__blk1300_dn6)) / (2.0 * assign48690_e62450)), (((var_v_xb__blk1300_dn7 * var_v_xb__blk1300) + (var_v_xb__blk1300 * var_v_xb__blk1300_dn7)) / (2.0 * assign48690_e62450)), (((var_v_xb__blk1300_dn8 * var_v_xb__blk1300) + (var_v_xb__blk1300 * var_v_xb__blk1300_dn8)) / (2.0 * assign48690_e62450)),)
    } else {
        (var_temp1, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8,)
    }
};
        var_temp1 = assign48690_e62452;
        var_temp1_dn5 = assign48690_e62452_d_n5;
        var_temp1_dn6 = assign48690_e62452_d_n6;
        var_temp1_dn7 = assign48690_e62452_d_n7;
        var_temp1_dn8 = assign48690_e62452_d_n8;
        var_temp1_rv = 0.0;

        let (assign48700_e62467, assign48700_e62467_d_n5, assign48700_e62467_d_n6, assign48700_e62467_d_n7, assign48700_e62467_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48700_e62458: f64 = (var_v_xb__blk1300 - var_delphib__blk1328);
        let assign48700_e62461: f64 = (var_v_xb__blk1300 - var_delphib__blk1328);
        let assign48700_e62462: f64 = (assign48700_e62458 * assign48700_e62461);
        let assign48700_e62464: f64 = (assign48700_e62462 + var_aphi__blk1298);
        let assign48700_e62465: f64 = (assign48700_e62464).sqrt();
        (assign48700_e62465, ((((-var_delphib__blk1328_dn5) * assign48700_e62461) + (assign48700_e62458 * (-var_delphib__blk1328_dn5))) / (2.0 * assign48700_e62465)), ((((var_v_xb__blk1300_dn6 - var_delphib__blk1328_dn6) * assign48700_e62461) + (assign48700_e62458 * (var_v_xb__blk1300_dn6 - var_delphib__blk1328_dn6))) / (2.0 * assign48700_e62465)), ((((var_v_xb__blk1300_dn7 - var_delphib__blk1328_dn7) * assign48700_e62461) + (assign48700_e62458 * (var_v_xb__blk1300_dn7 - var_delphib__blk1328_dn7))) / (2.0 * assign48700_e62465)), ((((var_v_xb__blk1300_dn8 - var_delphib__blk1328_dn8) * assign48700_e62461) + (assign48700_e62458 * (var_v_xb__blk1300_dn8 - var_delphib__blk1328_dn8))) / (2.0 * assign48700_e62465)),)
    } else {
        (var_temp2, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8,)
    }
};
        var_temp2 = assign48700_e62467;
        var_temp2_dn5 = assign48700_e62467_d_n5;
        var_temp2_dn6 = assign48700_e62467_d_n6;
        var_temp2_dn7 = assign48700_e62467_d_n7;
        var_temp2_dn8 = assign48700_e62467_d_n8;
        var_temp2_rv = 0.0;

        let (assign48710_e62481, assign48710_e62481_d_n5, assign48710_e62481_d_n6, assign48710_e62481_d_n7, assign48710_e62481_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48710_e62473: f64 = (0.5 * var_inv_phit1__blk1323);
        let assign48710_e62476: f64 = (var_delphib__blk1328 + var_temp1);
        let assign48710_e62478: f64 = (assign48710_e62476 - var_temp2);
        let assign48710_e62479: f64 = (assign48710_e62473 * assign48710_e62478);
        (assign48710_e62479, (((0.5 * var_inv_phit1__blk1323_dn5) * assign48710_e62478) + (assign48710_e62473 * ((var_delphib__blk1328_dn5 + var_temp1_dn5) - var_temp2_dn5))), (((0.5 * var_inv_phit1__blk1323_dn6) * assign48710_e62478) + (assign48710_e62473 * ((var_delphib__blk1328_dn6 + var_temp1_dn6) - var_temp2_dn6))), (((0.5 * var_inv_phit1__blk1323_dn7) * assign48710_e62478) + (assign48710_e62473 * ((var_delphib__blk1328_dn7 + var_temp1_dn7) - var_temp2_dn7))), (((0.5 * var_inv_phit1__blk1323_dn8) * assign48710_e62478) + (assign48710_e62473 * ((var_delphib__blk1328_dn8 + var_temp1_dn8) - var_temp2_dn8))),)
    } else {
        (var_delxb__blk1330, var_delxb__blk1330_dn5, var_delxb__blk1330_dn6, var_delxb__blk1330_dn7, var_delxb__blk1330_dn8,)
    }
};
        var_delxb__blk1330 = assign48710_e62481;
        var_delxb__blk1330_dn5 = assign48710_e62481_d_n5;
        var_delxb__blk1330_dn6 = assign48710_e62481_d_n6;
        var_delxb__blk1330_dn7 = assign48710_e62481_d_n7;
        var_delxb__blk1330_dn8 = assign48710_e62481_d_n8;
        var_delxb__blk1330_rv = 0.0;

        let (assign48720_e62489, assign48720_e62489_d_n5, assign48720_e62489_d_n6, assign48720_e62489_d_n7, assign48720_e62489_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48720_e62487: f64 = (var_xb__blk1329 + var_ux__blk1325);
        (assign48720_e62487, (var_xb__blk1329_dn5 + var_ux__blk1325_dn5), (var_xb__blk1329_dn6 + var_ux__blk1325_dn6), (var_xb__blk1329_dn7 + var_ux__blk1325_dn7), (var_xb__blk1329_dn8 + var_ux__blk1325_dn8),)
    } else {
        (var_xno_s__blk1331, var_xno_s__blk1331_dn5, var_xno_s__blk1331_dn6, var_xno_s__blk1331_dn7, var_xno_s__blk1331_dn8,)
    }
};
        var_xno_s__blk1331 = assign48720_e62489;
        var_xno_s__blk1331_dn5 = assign48720_e62489_d_n5;
        var_xno_s__blk1331_dn6 = assign48720_e62489_d_n6;
        var_xno_s__blk1331_dn7 = assign48720_e62489_d_n7;
        var_xno_s__blk1331_dn8 = assign48720_e62489_d_n8;
        var_xno_s__blk1331_rv = 0.0;

        let (assign48730_e62497, assign48730_e62497_d_n5, assign48730_e62497_d_n6, assign48730_e62497_d_n7, assign48730_e62497_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48730_e62495: f64 = (var_xno_s__blk1331 - var_delxb__blk1330);
        (assign48730_e62495, (var_xno_s__blk1331_dn5 - var_delxb__blk1330_dn5), (var_xno_s__blk1331_dn6 - var_delxb__blk1330_dn6), (var_xno_s__blk1331_dn7 - var_delxb__blk1330_dn7), (var_xno_s__blk1331_dn8 - var_delxb__blk1330_dn8),)
    } else {
        (var_xn_s__blk1332, var_xn_s__blk1332_dn5, var_xn_s__blk1332_dn6, var_xn_s__blk1332_dn7, var_xn_s__blk1332_dn8,)
    }
};
        var_xn_s__blk1332 = assign48730_e62497;
        var_xn_s__blk1332_dn5 = assign48730_e62497_d_n5;
        var_xn_s__blk1332_dn6 = assign48730_e62497_d_n6;
        var_xn_s__blk1332_dn7 = assign48730_e62497_d_n7;
        var_xn_s__blk1332_dn8 = assign48730_e62497_d_n8;
        var_xn_s__blk1332_rv = 0.0;

        let assign48740_e62500: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1461 = assign48740_e62500;
        var_guard1461_rv = 0.0;

        let assign48750_e62502: f64 = (var_xn_s__blk1332).abs();
        let assign48750_e62504: f64 = if assign48750_e62502 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1462 = assign48750_e62504;
        var_guard1462_rv = 0.0;

        let (assign48760_e62528, assign48760_e62528_d_n5, assign48760_e62528_d_n6, assign48760_e62528_d_n7, assign48760_e62528_d_n8,) = {
    if ((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) {
        let assign48760_e62517: f64 = (0.5 * var_xn_s__blk1332);
        let assign48760_e62521: f64 = (0.3125 * var_xn_s__blk1332);
        let assign48760_e62522: f64 = (1.0 - assign48760_e62521);
        let assign48760_e62523: f64 = (assign48760_e62517 * assign48760_e62522);
        let assign48760_e62524: f64 = (1.0 - assign48760_e62523);
        let assign48760_e62525: f64 = (var_gf__blk1307 * assign48760_e62524);
        let assign48760_e62526: f64 = (1.0 + assign48760_e62525);
        (assign48760_e62526, ((var_gf__blk1307_dn5 * assign48760_e62524) + (var_gf__blk1307 * (-(((0.5 * var_xn_s__blk1332_dn5) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * var_xn_s__blk1332_dn5))))))), ((var_gf__blk1307_dn6 * assign48760_e62524) + (var_gf__blk1307 * (-(((0.5 * var_xn_s__blk1332_dn6) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * var_xn_s__blk1332_dn6))))))), ((var_gf__blk1307_dn7 * assign48760_e62524) + (var_gf__blk1307 * (-(((0.5 * var_xn_s__blk1332_dn7) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * var_xn_s__blk1332_dn7))))))), ((var_gf__blk1307_dn8 * assign48760_e62524) + (var_gf__blk1307 * (-(((0.5 * var_xn_s__blk1332_dn8) * assign48760_e62522) + (assign48760_e62517 * (-(0.3125 * var_xn_s__blk1332_dn8))))))),)
    } else {
        (var_nscr__blk1333, var_nscr__blk1333_dn5, var_nscr__blk1333_dn6, var_nscr__blk1333_dn7, var_nscr__blk1333_dn8,)
    }
};
        var_nscr__blk1333 = assign48760_e62528;
        var_nscr__blk1333_dn5 = assign48760_e62528_d_n5;
        var_nscr__blk1333_dn6 = assign48760_e62528_d_n6;
        var_nscr__blk1333_dn7 = assign48760_e62528_d_n7;
        var_nscr__blk1333_dn8 = assign48760_e62528_d_n8;
        var_nscr__blk1333_rv = 0.0;

        let assign48770_e62531: f64 = if var_xn_s__blk1332 < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1463 = assign48770_e62531;
        var_guard1463_rv = 0.0;

        let (assign48780_e62546, assign48780_e62546_d_n5, assign48780_e62546_d_n6, assign48780_e62546_d_n7, assign48780_e62546_d_n8,) = {
    if (((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1463 != 0.0)) {
        let assign48780_e62543: f64 = (-var_xn_s__blk1332);
        let assign48780_e62544: f64 = (assign48780_e62543).exp();
        (assign48780_e62544, (assign48780_e62544 * (-var_xn_s__blk1332_dn5)), (assign48780_e62544 * (-var_xn_s__blk1332_dn6)), (assign48780_e62544 * (-var_xn_s__blk1332_dn7)), (assign48780_e62544 * (-var_xn_s__blk1332_dn8)),)
    } else {
        (var_delta_ns__blk1347, var_delta_ns__blk1347_dn5, var_delta_ns__blk1347_dn6, var_delta_ns__blk1347_dn7, var_delta_ns__blk1347_dn8,)
    }
};
        var_delta_ns__blk1347 = assign48780_e62546;
        var_delta_ns__blk1347_dn5 = assign48780_e62546_d_n5;
        var_delta_ns__blk1347_dn6 = assign48780_e62546_d_n6;
        var_delta_ns__blk1347_dn7 = assign48780_e62546_d_n7;
        var_delta_ns__blk1347_dn8 = assign48780_e62546_d_n8;
        var_delta_ns__blk1347_rv = 0.0;

        let (assign48790_e62582, assign48790_e62582_d_n5, assign48790_e62582_d_n6, assign48790_e62582_d_n7, assign48790_e62582_d_n8,) = {
    if (((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1463 == 0.0)) {
        let assign48790_e62562: f64 = (var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62567: f64 = (var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62571: f64 = (var_xn_s__blk1332 - 460.51701859880916);
        let assign48790_e62573: f64 = (assign48790_e62571 * 0.3333333333333333);
        let assign48790_e62574: f64 = (1.0 + assign48790_e62573);
        let assign48790_e62575: f64 = (assign48790_e62567 * assign48790_e62574);
        let assign48790_e62576: f64 = (0.5 * assign48790_e62575);
        let assign48790_e62577: f64 = (1.0 + assign48790_e62576);
        let assign48790_e62578: f64 = (assign48790_e62562 * assign48790_e62577);
        let assign48790_e62579: f64 = (1.0 + assign48790_e62578);
        let assign48790_e62580: f64 = (1e-200 / assign48790_e62579);
        (assign48790_e62580, (-((1e-200 * ((var_xn_s__blk1332_dn5 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((var_xn_s__blk1332_dn5 * assign48790_e62574) + (assign48790_e62567 * (var_xn_s__blk1332_dn5 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((var_xn_s__blk1332_dn6 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((var_xn_s__blk1332_dn6 * assign48790_e62574) + (assign48790_e62567 * (var_xn_s__blk1332_dn6 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((var_xn_s__blk1332_dn7 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((var_xn_s__blk1332_dn7 * assign48790_e62574) + (assign48790_e62567 * (var_xn_s__blk1332_dn7 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))), (-((1e-200 * ((var_xn_s__blk1332_dn8 * assign48790_e62577) + (assign48790_e62562 * (0.5 * ((var_xn_s__blk1332_dn8 * assign48790_e62574) + (assign48790_e62567 * (var_xn_s__blk1332_dn8 * 0.3333333333333333))))))) / (assign48790_e62579 * assign48790_e62579))),)
    } else {
        (var_delta_ns__blk1347, var_delta_ns__blk1347_dn5, var_delta_ns__blk1347_dn6, var_delta_ns__blk1347_dn7, var_delta_ns__blk1347_dn8,)
    }
};
        var_delta_ns__blk1347 = assign48790_e62582;
        var_delta_ns__blk1347_dn5 = assign48790_e62582_d_n5;
        var_delta_ns__blk1347_dn6 = assign48790_e62582_d_n6;
        var_delta_ns__blk1347_dn7 = assign48790_e62582_d_n7;
        var_delta_ns__blk1347_dn8 = assign48790_e62582_d_n8;
        var_delta_ns__blk1347_rv = 0.0;

        let (assign48800_e62599, assign48800_e62599_d_n5, assign48800_e62599_d_n6, assign48800_e62599_d_n7, assign48800_e62599_d_n8,) = {
    if ((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let (assign48800_e62597,) = {
            if (var_xn_s__blk1332 > 0.0) {
                (1.0,)
            } else {
                let assign48800_e62596: f64 = (-1.0);
                (assign48800_e62596,)
            }
        };
        (assign48800_e62597, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48800_e62599;
        var_temp__blk936_dn5 = assign48800_e62599_d_n5;
        var_temp__blk936_dn6 = assign48800_e62599_d_n6;
        var_temp__blk936_dn7 = assign48800_e62599_d_n7;
        var_temp__blk936_dn8 = assign48800_e62599_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign48810_e62631, assign48810_e62631_d_n5, assign48810_e62631_d_n6, assign48810_e62631_d_n7, assign48810_e62631_d_n8,) = {
    if ((((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign48810_e62611: f64 = (var_temp__blk936 * var_gf__blk1307);
        let assign48810_e62616: f64 = (1.0 - var_xn_s__blk1332);
        let assign48810_e62617: f64 = (var_delta_ns__blk1347 * assign48810_e62616);
        let assign48810_e62618: f64 = (1.0 - assign48810_e62617);
        let assign48810_e62619: f64 = (assign48810_e62611 * assign48810_e62618);
        let assign48810_e62624: f64 = (1.0 - var_delta_ns__blk1347);
        let assign48810_e62625: f64 = (var_xn_s__blk1332 * assign48810_e62624);
        let assign48810_e62626: f64 = (assign48810_e62625).sqrt();
        let assign48810_e62627: f64 = (2.0 * assign48810_e62626);
        let assign48810_e62628: f64 = (assign48810_e62619 / assign48810_e62627);
        let assign48810_e62629: f64 = (1.0 + assign48810_e62628);
        (assign48810_e62629, (((((((var_temp__blk936_dn5 * var_gf__blk1307) + (var_temp__blk936 * var_gf__blk1307_dn5)) * assign48810_e62618) + (assign48810_e62611 * (-((var_delta_ns__blk1347_dn5 * assign48810_e62616) + (var_delta_ns__blk1347 * (-var_xn_s__blk1332_dn5)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((var_xn_s__blk1332_dn5 * assign48810_e62624) + (var_xn_s__blk1332 * (-var_delta_ns__blk1347_dn5))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((var_temp__blk936_dn6 * var_gf__blk1307) + (var_temp__blk936 * var_gf__blk1307_dn6)) * assign48810_e62618) + (assign48810_e62611 * (-((var_delta_ns__blk1347_dn6 * assign48810_e62616) + (var_delta_ns__blk1347 * (-var_xn_s__blk1332_dn6)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((var_xn_s__blk1332_dn6 * assign48810_e62624) + (var_xn_s__blk1332 * (-var_delta_ns__blk1347_dn6))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((var_temp__blk936_dn7 * var_gf__blk1307) + (var_temp__blk936 * var_gf__blk1307_dn7)) * assign48810_e62618) + (assign48810_e62611 * (-((var_delta_ns__blk1347_dn7 * assign48810_e62616) + (var_delta_ns__blk1347 * (-var_xn_s__blk1332_dn7)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((var_xn_s__blk1332_dn7 * assign48810_e62624) + (var_xn_s__blk1332 * (-var_delta_ns__blk1347_dn7))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)), (((((((var_temp__blk936_dn8 * var_gf__blk1307) + (var_temp__blk936 * var_gf__blk1307_dn8)) * assign48810_e62618) + (assign48810_e62611 * (-((var_delta_ns__blk1347_dn8 * assign48810_e62616) + (var_delta_ns__blk1347 * (-var_xn_s__blk1332_dn8)))))) * assign48810_e62627) - (assign48810_e62619 * (2.0 * (((var_xn_s__blk1332_dn8 * assign48810_e62624) + (var_xn_s__blk1332 * (-var_delta_ns__blk1347_dn8))) / (2.0 * assign48810_e62626))))) / (assign48810_e62627 * assign48810_e62627)),)
    } else {
        (var_nscr__blk1333, var_nscr__blk1333_dn5, var_nscr__blk1333_dn6, var_nscr__blk1333_dn7, var_nscr__blk1333_dn8,)
    }
};
        var_nscr__blk1333 = assign48810_e62631;
        var_nscr__blk1333_dn5 = assign48810_e62631_d_n5;
        var_nscr__blk1333_dn6 = assign48810_e62631_d_n6;
        var_nscr__blk1333_dn7 = assign48810_e62631_d_n7;
        var_nscr__blk1333_dn8 = assign48810_e62631_d_n8;
        var_nscr__blk1333_rv = 0.0;

        let (assign48820_e62647, assign48820_e62647_d_n5, assign48820_e62647_d_n6, assign48820_e62647_d_n7, assign48820_e62647_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1461 == 0.0)) {
        let assign48820_e62641: f64 = (0.5 * var_gf__blk1307);
        let assign48820_e62643: f64 = (var_xn_s__blk1332).sqrt();
        let assign48820_e62644: f64 = (assign48820_e62641 / assign48820_e62643);
        let assign48820_e62645: f64 = (1.0 + assign48820_e62644);
        (assign48820_e62645, ((((0.5 * var_gf__blk1307_dn5) * assign48820_e62643) - (assign48820_e62641 * (var_xn_s__blk1332_dn5 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * var_gf__blk1307_dn6) * assign48820_e62643) - (assign48820_e62641 * (var_xn_s__blk1332_dn6 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * var_gf__blk1307_dn7) * assign48820_e62643) - (assign48820_e62641 * (var_xn_s__blk1332_dn7 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)), ((((0.5 * var_gf__blk1307_dn8) * assign48820_e62643) - (assign48820_e62641 * (var_xn_s__blk1332_dn8 / (2.0 * assign48820_e62643)))) / (assign48820_e62643 * assign48820_e62643)),)
    } else {
        (var_nscr__blk1333, var_nscr__blk1333_dn5, var_nscr__blk1333_dn6, var_nscr__blk1333_dn7, var_nscr__blk1333_dn8,)
    }
};
        var_nscr__blk1333 = assign48820_e62647;
        var_nscr__blk1333_dn5 = assign48820_e62647_d_n5;
        var_nscr__blk1333_dn6 = assign48820_e62647_d_n6;
        var_nscr__blk1333_dn7 = assign48820_e62647_d_n7;
        var_nscr__blk1333_dn8 = assign48820_e62647_d_n8;
        var_nscr__blk1333_rv = 0.0;

        let (assign48830_e62665, assign48830_e62665_d_n5, assign48830_e62665_d_n6, assign48830_e62665_d_n7, assign48830_e62665_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48830_e62654: f64 = (var_xn_s__blk1332).sqrt();
        let assign48830_e62655: f64 = (var_gf__blk1307 * assign48830_e62654);
        let assign48830_e62656: f64 = (var_xn_s__blk1332 + assign48830_e62655);
        let assign48830_e62660: f64 = (var_nscr__blk1333 - 1.0);
        let assign48830_e62661: f64 = (assign48830_e62660).ln();
        let assign48830_e62662: f64 = (var_nscr__blk1333 * assign48830_e62661);
        let assign48830_e62663: f64 = (assign48830_e62656 - assign48830_e62662);
        (assign48830_e62663, ((var_xn_s__blk1332_dn5 + ((var_gf__blk1307_dn5 * assign48830_e62654) + (var_gf__blk1307 * (var_xn_s__blk1332_dn5 / (2.0 * assign48830_e62654))))) - ((var_nscr__blk1333_dn5 * assign48830_e62661) + (var_nscr__blk1333 * (var_nscr__blk1333_dn5 / assign48830_e62660)))), ((var_xn_s__blk1332_dn6 + ((var_gf__blk1307_dn6 * assign48830_e62654) + (var_gf__blk1307 * (var_xn_s__blk1332_dn6 / (2.0 * assign48830_e62654))))) - ((var_nscr__blk1333_dn6 * assign48830_e62661) + (var_nscr__blk1333 * (var_nscr__blk1333_dn6 / assign48830_e62660)))), ((var_xn_s__blk1332_dn7 + ((var_gf__blk1307_dn7 * assign48830_e62654) + (var_gf__blk1307 * (var_xn_s__blk1332_dn7 / (2.0 * assign48830_e62654))))) - ((var_nscr__blk1333_dn7 * assign48830_e62661) + (var_nscr__blk1333 * (var_nscr__blk1333_dn7 / assign48830_e62660)))), ((var_xn_s__blk1332_dn8 + ((var_gf__blk1307_dn8 * assign48830_e62654) + (var_gf__blk1307 * (var_xn_s__blk1332_dn8 / (2.0 * assign48830_e62654))))) - ((var_nscr__blk1333_dn8 * assign48830_e62661) + (var_nscr__blk1333 * (var_nscr__blk1333_dn8 / assign48830_e62660)))),)
    } else {
        (var_xthscr__blk1334, var_xthscr__blk1334_dn5, var_xthscr__blk1334_dn6, var_xthscr__blk1334_dn7, var_xthscr__blk1334_dn8,)
    }
};
        var_xthscr__blk1334 = assign48830_e62665;
        var_xthscr__blk1334_dn5 = assign48830_e62665_d_n5;
        var_xthscr__blk1334_dn6 = assign48830_e62665_d_n6;
        var_xthscr__blk1334_dn7 = assign48830_e62665_d_n7;
        var_xthscr__blk1334_dn8 = assign48830_e62665_d_n8;
        var_xthscr__blk1334_rv = 0.0;

        let (assign48840_e62675, assign48840_e62675_d_n5, assign48840_e62675_d_n6, assign48840_e62675_d_n7, assign48840_e62675_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48840_e62671: f64 = (var_xg__blk1326 - var_xthscr__blk1334);
        let assign48840_e62673: f64 = (assign48840_e62671 / var_nscr__blk1333);
        (assign48840_e62673, ((((var_xg__blk1326_dn5 - var_xthscr__blk1334_dn5) * var_nscr__blk1333) - (assign48840_e62671 * var_nscr__blk1333_dn5)) / (var_nscr__blk1333 * var_nscr__blk1333)), ((((var_xg__blk1326_dn6 - var_xthscr__blk1334_dn6) * var_nscr__blk1333) - (assign48840_e62671 * var_nscr__blk1333_dn6)) / (var_nscr__blk1333 * var_nscr__blk1333)), ((((var_xg__blk1326_dn7 - var_xthscr__blk1334_dn7) * var_nscr__blk1333) - (assign48840_e62671 * var_nscr__blk1333_dn7)) / (var_nscr__blk1333 * var_nscr__blk1333)), ((((var_xg__blk1326_dn8 - var_xthscr__blk1334_dn8) * var_nscr__blk1333) - (assign48840_e62671 * var_nscr__blk1333_dn8)) / (var_nscr__blk1333 * var_nscr__blk1333)),)
    } else {
        (var_xgtscr__blk1335, var_xgtscr__blk1335_dn5, var_xgtscr__blk1335_dn6, var_xgtscr__blk1335_dn7, var_xgtscr__blk1335_dn8,)
    }
};
        var_xgtscr__blk1335 = assign48840_e62675;
        var_xgtscr__blk1335_dn5 = assign48840_e62675_d_n5;
        var_xgtscr__blk1335_dn6 = assign48840_e62675_d_n6;
        var_xgtscr__blk1335_dn7 = assign48840_e62675_d_n7;
        var_xgtscr__blk1335_dn8 = assign48840_e62675_d_n8;
        var_xgtscr__blk1335_rv = 0.0;

        let (assign48850_e62692, assign48850_e62692_d_n5, assign48850_e62692_d_n6, assign48850_e62692_d_n7, assign48850_e62692_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        let assign48850_e62681: f64 = (0.5 * var_gf2__blk1308);
        let assign48850_e62685: f64 = (8.0 / var_gf2__blk1308);
        let assign48850_e62686: f64 = (1.0 + assign48850_e62685);
        let assign48850_e62687: f64 = (assign48850_e62686).sqrt();
        let assign48850_e62689: f64 = (assign48850_e62687 - 1.0);
        let assign48850_e62690: f64 = (assign48850_e62681 * assign48850_e62689);
        (assign48850_e62690, (((0.5 * var_gf2__blk1308_dn5) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * var_gf2__blk1308_dn5) / (var_gf2__blk1308 * var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * var_gf2__blk1308_dn6) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * var_gf2__blk1308_dn6) / (var_gf2__blk1308 * var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * var_gf2__blk1308_dn7) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * var_gf2__blk1308_dn7) / (var_gf2__blk1308 * var_gf2__blk1308))) / (2.0 * assign48850_e62687)))), (((0.5 * var_gf2__blk1308_dn8) * assign48850_e62689) + (assign48850_e62681 * ((-((8.0 * var_gf2__blk1308_dn8) / (var_gf2__blk1308 * var_gf2__blk1308))) / (2.0 * assign48850_e62687)))),)
    } else {
        (var_qbscr__blk1341, var_qbscr__blk1341_dn5, var_qbscr__blk1341_dn6, var_qbscr__blk1341_dn7, var_qbscr__blk1341_dn8,)
    }
};
        var_qbscr__blk1341 = assign48850_e62692;
        var_qbscr__blk1341_dn5 = assign48850_e62692_d_n5;
        var_qbscr__blk1341_dn6 = assign48850_e62692_d_n6;
        var_qbscr__blk1341_dn7 = assign48850_e62692_d_n7;
        var_qbscr__blk1341_dn8 = assign48850_e62692_d_n8;
        var_qbscr__blk1341_rv = 0.0;

        let (assign48860_e62698, assign48860_e62698_d_n5, assign48860_e62698_d_n6, assign48860_e62698_d_n7, assign48860_e62698_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qiscr__blk1340, var_qiscr__blk1340_dn5, var_qiscr__blk1340_dn6, var_qiscr__blk1340_dn7, var_qiscr__blk1340_dn8,)
    }
};
        var_qiscr__blk1340 = assign48860_e62698;
        var_qiscr__blk1340_dn5 = assign48860_e62698_d_n5;
        var_qiscr__blk1340_dn6 = assign48860_e62698_d_n6;
        var_qiscr__blk1340_dn7 = assign48860_e62698_d_n7;
        var_qiscr__blk1340_dn8 = assign48860_e62698_d_n8;
        var_qiscr__blk1340_rv = 0.0;

        let (assign48870_e62704, assign48870_e62704_d_n5, assign48870_e62704_d_n6, assign48870_e62704_d_n7, assign48870_e62704_d_n8,) = {
    if ((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fscr__blk1342, var_fscr__blk1342_dn5, var_fscr__blk1342_dn6, var_fscr__blk1342_dn7, var_fscr__blk1342_dn8,)
    }
};
        var_fscr__blk1342 = assign48870_e62704;
        var_fscr__blk1342_dn5 = assign48870_e62704_d_n5;
        var_fscr__blk1342_dn6 = assign48870_e62704_d_n6;
        var_fscr__blk1342_dn7 = assign48870_e62704_d_n7;
        var_fscr__blk1342_dn8 = assign48870_e62704_d_n8;
        var_fscr__blk1342_rv = 0.0;

        let assign48880_e62707: f64 = (-30.0);
        let assign48880_e62708: f64 = if var_xgtscr__blk1335 > assign48880_e62707 { 1.0 } else { 0.0 };
        var_guard1464 = assign48880_e62708;
        var_guard1464_rv = 0.0;

        let (assign48890_e62720, assign48890_e62720_d_n5, assign48890_e62720_d_n6, assign48890_e62720_d_n7, assign48890_e62720_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1464 != 0.0)) {
        let assign48890_e62716: f64 = (var_nscr__blk1333 * var_xgtscr__blk1335);
        let assign48890_e62718: f64 = (assign48890_e62716 - 1.0);
        (assign48890_e62718, ((var_nscr__blk1333_dn5 * var_xgtscr__blk1335) + (var_nscr__blk1333 * var_xgtscr__blk1335_dn5)), ((var_nscr__blk1333_dn6 * var_xgtscr__blk1335) + (var_nscr__blk1333 * var_xgtscr__blk1335_dn6)), ((var_nscr__blk1333_dn7 * var_xgtscr__blk1335) + (var_nscr__blk1333 * var_xgtscr__blk1335_dn7)), ((var_nscr__blk1333_dn8 * var_xgtscr__blk1335) + (var_nscr__blk1333 * var_xgtscr__blk1335_dn8)),)
    } else {
        (var_xgtscr0__blk1336, var_xgtscr0__blk1336_dn5, var_xgtscr0__blk1336_dn6, var_xgtscr0__blk1336_dn7, var_xgtscr0__blk1336_dn8,)
    }
};
        var_xgtscr0__blk1336 = assign48890_e62720;
        var_xgtscr0__blk1336_dn5 = assign48890_e62720_d_n5;
        var_xgtscr0__blk1336_dn6 = assign48890_e62720_d_n6;
        var_xgtscr0__blk1336_dn7 = assign48890_e62720_d_n7;
        var_xgtscr0__blk1336_dn8 = assign48890_e62720_d_n8;
        var_xgtscr0__blk1336_rv = 0.0;

        let (assign48900_e62737, assign48900_e62737_d_n5, assign48900_e62737_d_n6, assign48900_e62737_d_n7, assign48900_e62737_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1464 != 0.0)) {
        let assign48900_e62730: f64 = (var_xgtscr0__blk1336 * var_xgtscr0__blk1336);
        let assign48900_e62732: f64 = (assign48900_e62730 + 10.0);
        let assign48900_e62733: f64 = (assign48900_e62732).sqrt();
        let assign48900_e62734: f64 = (var_xgtscr0__blk1336 + assign48900_e62733);
        let assign48900_e62735: f64 = (0.5 * assign48900_e62734);
        (assign48900_e62735, (0.5 * (var_xgtscr0__blk1336_dn5 + (((var_xgtscr0__blk1336_dn5 * var_xgtscr0__blk1336) + (var_xgtscr0__blk1336 * var_xgtscr0__blk1336_dn5)) / (2.0 * assign48900_e62733)))), (0.5 * (var_xgtscr0__blk1336_dn6 + (((var_xgtscr0__blk1336_dn6 * var_xgtscr0__blk1336) + (var_xgtscr0__blk1336 * var_xgtscr0__blk1336_dn6)) / (2.0 * assign48900_e62733)))), (0.5 * (var_xgtscr0__blk1336_dn7 + (((var_xgtscr0__blk1336_dn7 * var_xgtscr0__blk1336) + (var_xgtscr0__blk1336 * var_xgtscr0__blk1336_dn7)) / (2.0 * assign48900_e62733)))), (0.5 * (var_xgtscr0__blk1336_dn8 + (((var_xgtscr0__blk1336_dn8 * var_xgtscr0__blk1336) + (var_xgtscr0__blk1336 * var_xgtscr0__blk1336_dn8)) / (2.0 * assign48900_e62733)))),)
    } else {
        (var_temp__blk936, var_temp__blk936_dn5, var_temp__blk936_dn6, var_temp__blk936_dn7, var_temp__blk936_dn8,)
    }
};
        var_temp__blk936 = assign48900_e62737;
        var_temp__blk936_dn5 = assign48900_e62737_d_n5;
        var_temp__blk936_dn6 = assign48900_e62737_d_n6;
        var_temp__blk936_dn7 = assign48900_e62737_d_n7;
        var_temp__blk936_dn8 = assign48900_e62737_d_n8;
        var_temp__blk936_rv = 0.0;

        let (assign48910_e62748, assign48910_e62748_d_n5, assign48910_e62748_d_n6, assign48910_e62748_d_n7, assign48910_e62748_d_n8,) = {
    if (((var_guard1456 != 0.0) && (var_guard1457 != 0.0)) && (var_guard1464 != 0.0)) {
        let assign48910_e62745: f64 = (var_temp__blk936).ln();
        let assign48910_e62746: f64 = (var_xgtscr__blk1335 - assign48910_e62745);
        (assign48910_e62746, (var_xgtscr__blk1335_dn5 - (var_temp__blk936_dn5 / var_temp__blk936)), (var_xgtscr__blk1335_dn6 - (var_temp__blk936_dn6 / var_temp__blk936)), (var_xgtscr__blk1335_dn7 - (var_temp__blk936_dn7 / var_temp__blk936)), (var_xgtscr__blk1335_dn8 - (var_temp__blk936_dn8 / var_temp__blk936)),)
    } else {
        (var_qiscr0si__blk1337, var_qiscr0si__blk1337_dn5, var_qiscr0si__blk1337_dn6, var_qiscr0si__blk1337_dn7, var_qiscr0si__blk1337_dn8,)
    }
};
        var_qiscr0si__blk1337 = assign48910_e62748;
        var_qiscr0si__blk1337_dn5 = assign48910_e62748_d_n5;
        var_qiscr0si__blk1337_dn6 = assign48910_e62748_d_n6;
        var_qiscr0si__blk1337_dn7 = assign48910_e62748_d_n7;
        var_qiscr0si__blk1337_dn8 = assign48910_e62748_d_n8;
        var_qiscr0si__blk1337_rv = 0.0;

        *var_delphib__blk1328_slot = var_delphib__blk1328;
        *var_delphib__blk1328_dn5_slot = var_delphib__blk1328_dn5;
        *var_delphib__blk1328_dn6_slot = var_delphib__blk1328_dn6;
        *var_delphib__blk1328_dn7_slot = var_delphib__blk1328_dn7;
        *var_delphib__blk1328_dn8_slot = var_delphib__blk1328_dn8;
        *var_delphib__blk1328_rv_slot = var_delphib__blk1328_rv;
        *var_delta_ns__blk1347_slot = var_delta_ns__blk1347;
        *var_delta_ns__blk1347_dn5_slot = var_delta_ns__blk1347_dn5;
        *var_delta_ns__blk1347_dn6_slot = var_delta_ns__blk1347_dn6;
        *var_delta_ns__blk1347_dn7_slot = var_delta_ns__blk1347_dn7;
        *var_delta_ns__blk1347_dn8_slot = var_delta_ns__blk1347_dn8;
        *var_delta_ns__blk1347_rv_slot = var_delta_ns__blk1347_rv;
        *var_delxb__blk1330_slot = var_delxb__blk1330;
        *var_delxb__blk1330_dn5_slot = var_delxb__blk1330_dn5;
        *var_delxb__blk1330_dn6_slot = var_delxb__blk1330_dn6;
        *var_delxb__blk1330_dn7_slot = var_delxb__blk1330_dn7;
        *var_delxb__blk1330_dn8_slot = var_delxb__blk1330_dn8;
        *var_delxb__blk1330_rv_slot = var_delxb__blk1330_rv;
        *var_fscr__blk1342_slot = var_fscr__blk1342;
        *var_fscr__blk1342_dn5_slot = var_fscr__blk1342_dn5;
        *var_fscr__blk1342_dn6_slot = var_fscr__blk1342_dn6;
        *var_fscr__blk1342_dn7_slot = var_fscr__blk1342_dn7;
        *var_fscr__blk1342_dn8_slot = var_fscr__blk1342_dn8;
        *var_fscr__blk1342_rv_slot = var_fscr__blk1342_rv;
        *var_gf2__blk1308_slot = var_gf2__blk1308;
        *var_gf2__blk1308_dn5_slot = var_gf2__blk1308_dn5;
        *var_gf2__blk1308_dn6_slot = var_gf2__blk1308_dn6;
        *var_gf2__blk1308_dn7_slot = var_gf2__blk1308_dn7;
        *var_gf2__blk1308_dn8_slot = var_gf2__blk1308_dn8;
        *var_gf2__blk1308_rv_slot = var_gf2__blk1308_rv;
        *var_gf__blk1307_slot = var_gf__blk1307;
        *var_gf__blk1307_dn5_slot = var_gf__blk1307_dn5;
        *var_gf__blk1307_dn6_slot = var_gf__blk1307_dn6;
        *var_gf__blk1307_dn7_slot = var_gf__blk1307_dn7;
        *var_gf__blk1307_dn8_slot = var_gf__blk1307_dn8;
        *var_gf__blk1307_rv_slot = var_gf__blk1307_rv;
        *var_guard1461_slot = var_guard1461;
        *var_guard1461_rv_slot = var_guard1461_rv;
        *var_guard1462_slot = var_guard1462;
        *var_guard1462_rv_slot = var_guard1462_rv;
        *var_guard1463_slot = var_guard1463;
        *var_guard1463_rv_slot = var_guard1463_rv;
        *var_guard1464_slot = var_guard1464;
        *var_guard1464_rv_slot = var_guard1464_rv;
        *var_inv_gf2__blk1324_slot = var_inv_gf2__blk1324;
        *var_inv_gf2__blk1324_dn5_slot = var_inv_gf2__blk1324_dn5;
        *var_inv_gf2__blk1324_dn6_slot = var_inv_gf2__blk1324_dn6;
        *var_inv_gf2__blk1324_dn7_slot = var_inv_gf2__blk1324_dn7;
        *var_inv_gf2__blk1324_dn8_slot = var_inv_gf2__blk1324_dn8;
        *var_inv_gf2__blk1324_rv_slot = var_inv_gf2__blk1324_rv;
        *var_inv_phit1__blk1323_slot = var_inv_phit1__blk1323;
        *var_inv_phit1__blk1323_dn5_slot = var_inv_phit1__blk1323_dn5;
        *var_inv_phit1__blk1323_dn6_slot = var_inv_phit1__blk1323_dn6;
        *var_inv_phit1__blk1323_dn7_slot = var_inv_phit1__blk1323_dn7;
        *var_inv_phit1__blk1323_dn8_slot = var_inv_phit1__blk1323_dn8;
        *var_inv_phit1__blk1323_rv_slot = var_inv_phit1__blk1323_rv;
        *var_nscr__blk1333_slot = var_nscr__blk1333;
        *var_nscr__blk1333_dn5_slot = var_nscr__blk1333_dn5;
        *var_nscr__blk1333_dn6_slot = var_nscr__blk1333_dn6;
        *var_nscr__blk1333_dn7_slot = var_nscr__blk1333_dn7;
        *var_nscr__blk1333_dn8_slot = var_nscr__blk1333_dn8;
        *var_nscr__blk1333_rv_slot = var_nscr__blk1333_rv;
        *var_qbscr__blk1341_slot = var_qbscr__blk1341;
        *var_qbscr__blk1341_dn5_slot = var_qbscr__blk1341_dn5;
        *var_qbscr__blk1341_dn6_slot = var_qbscr__blk1341_dn6;
        *var_qbscr__blk1341_dn7_slot = var_qbscr__blk1341_dn7;
        *var_qbscr__blk1341_dn8_slot = var_qbscr__blk1341_dn8;
        *var_qbscr__blk1341_rv_slot = var_qbscr__blk1341_rv;
        *var_qiscr0si__blk1337_slot = var_qiscr0si__blk1337;
        *var_qiscr0si__blk1337_dn5_slot = var_qiscr0si__blk1337_dn5;
        *var_qiscr0si__blk1337_dn6_slot = var_qiscr0si__blk1337_dn6;
        *var_qiscr0si__blk1337_dn7_slot = var_qiscr0si__blk1337_dn7;
        *var_qiscr0si__blk1337_dn8_slot = var_qiscr0si__blk1337_dn8;
        *var_qiscr0si__blk1337_rv_slot = var_qiscr0si__blk1337_rv;
        *var_qiscr__blk1340_slot = var_qiscr__blk1340;
        *var_qiscr__blk1340_dn5_slot = var_qiscr__blk1340_dn5;
        *var_qiscr__blk1340_dn6_slot = var_qiscr__blk1340_dn6;
        *var_qiscr__blk1340_dn7_slot = var_qiscr__blk1340_dn7;
        *var_qiscr__blk1340_dn8_slot = var_qiscr__blk1340_dn8;
        *var_qiscr__blk1340_rv_slot = var_qiscr__blk1340_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_temp__blk936_slot = var_temp__blk936;
        *var_temp__blk936_dn5_slot = var_temp__blk936_dn5;
        *var_temp__blk936_dn6_slot = var_temp__blk936_dn6;
        *var_temp__blk936_dn7_slot = var_temp__blk936_dn7;
        *var_temp__blk936_dn8_slot = var_temp__blk936_dn8;
        *var_temp__blk936_rv_slot = var_temp__blk936_rv;
        *var_ux__blk1325_slot = var_ux__blk1325;
        *var_ux__blk1325_dn5_slot = var_ux__blk1325_dn5;
        *var_ux__blk1325_dn6_slot = var_ux__blk1325_dn6;
        *var_ux__blk1325_dn7_slot = var_ux__blk1325_dn7;
        *var_ux__blk1325_dn8_slot = var_ux__blk1325_dn8;
        *var_ux__blk1325_rv_slot = var_ux__blk1325_rv;
        *var_vdsp__blk1327_slot = var_vdsp__blk1327;
        *var_vdsp__blk1327_dn6_slot = var_vdsp__blk1327_dn6;
        *var_vdsp__blk1327_dn7_slot = var_vdsp__blk1327_dn7;
        *var_vdsp__blk1327_rv_slot = var_vdsp__blk1327_rv;
        *var_xb__blk1329_slot = var_xb__blk1329;
        *var_xb__blk1329_dn5_slot = var_xb__blk1329_dn5;
        *var_xb__blk1329_dn6_slot = var_xb__blk1329_dn6;
        *var_xb__blk1329_dn7_slot = var_xb__blk1329_dn7;
        *var_xb__blk1329_dn8_slot = var_xb__blk1329_dn8;
        *var_xb__blk1329_rv_slot = var_xb__blk1329_rv;
        *var_xg__blk1326_slot = var_xg__blk1326;
        *var_xg__blk1326_dn5_slot = var_xg__blk1326_dn5;
        *var_xg__blk1326_dn6_slot = var_xg__blk1326_dn6;
        *var_xg__blk1326_dn7_slot = var_xg__blk1326_dn7;
        *var_xg__blk1326_dn8_slot = var_xg__blk1326_dn8;
        *var_xg__blk1326_rv_slot = var_xg__blk1326_rv;
        *var_xgtscr0__blk1336_slot = var_xgtscr0__blk1336;
        *var_xgtscr0__blk1336_dn5_slot = var_xgtscr0__blk1336_dn5;
        *var_xgtscr0__blk1336_dn6_slot = var_xgtscr0__blk1336_dn6;
        *var_xgtscr0__blk1336_dn7_slot = var_xgtscr0__blk1336_dn7;
        *var_xgtscr0__blk1336_dn8_slot = var_xgtscr0__blk1336_dn8;
        *var_xgtscr0__blk1336_rv_slot = var_xgtscr0__blk1336_rv;
        *var_xgtscr__blk1335_slot = var_xgtscr__blk1335;
        *var_xgtscr__blk1335_dn5_slot = var_xgtscr__blk1335_dn5;
        *var_xgtscr__blk1335_dn6_slot = var_xgtscr__blk1335_dn6;
        *var_xgtscr__blk1335_dn7_slot = var_xgtscr__blk1335_dn7;
        *var_xgtscr__blk1335_dn8_slot = var_xgtscr__blk1335_dn8;
        *var_xgtscr__blk1335_rv_slot = var_xgtscr__blk1335_rv;
        *var_xn_s__blk1332_slot = var_xn_s__blk1332;
        *var_xn_s__blk1332_dn5_slot = var_xn_s__blk1332_dn5;
        *var_xn_s__blk1332_dn6_slot = var_xn_s__blk1332_dn6;
        *var_xn_s__blk1332_dn7_slot = var_xn_s__blk1332_dn7;
        *var_xn_s__blk1332_dn8_slot = var_xn_s__blk1332_dn8;
        *var_xn_s__blk1332_rv_slot = var_xn_s__blk1332_rv;
        *var_xno_s__blk1331_slot = var_xno_s__blk1331;
        *var_xno_s__blk1331_dn5_slot = var_xno_s__blk1331_dn5;
        *var_xno_s__blk1331_dn6_slot = var_xno_s__blk1331_dn6;
        *var_xno_s__blk1331_dn7_slot = var_xno_s__blk1331_dn7;
        *var_xno_s__blk1331_dn8_slot = var_xno_s__blk1331_dn8;
        *var_xno_s__blk1331_rv_slot = var_xno_s__blk1331_rv;
        *var_xthscr__blk1334_slot = var_xthscr__blk1334;
        *var_xthscr__blk1334_dn5_slot = var_xthscr__blk1334_dn5;
        *var_xthscr__blk1334_dn6_slot = var_xthscr__blk1334_dn6;
        *var_xthscr__blk1334_dn7_slot = var_xthscr__blk1334_dn7;
        *var_xthscr__blk1334_dn8_slot = var_xthscr__blk1334_dn8;
        *var_xthscr__blk1334_rv_slot = var_xthscr__blk1334_rv;
    }
}
