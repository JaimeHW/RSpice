#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_adrain_i_slot: &mut f64,
        var_asource_i_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cic1_i_slot: &mut f64,
        var_cic2_i_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_dt_slot: &mut f64,
        var_dt_dn4_slot: &mut f64,
        var_dt_dn6_slot: &mut f64,
        var_dt_dn7_slot: &mut f64,
        var_dt_dn8_slot: &mut f64,
        var_dt_dn9_slot: &mut f64,
        var_dtc_slot: &mut f64,
        var_dtc_dn4_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_inv_phit0_slot: &mut f64,
        var_inv_phit0_dn4_slot: &mut f64,
        var_inv_phit0_dn6_slot: &mut f64,
        var_inv_phit0_dn7_slot: &mut f64,
        var_inv_phit0_dn8_slot: &mut f64,
        var_inv_phit0_dn9_slot: &mut f64,
        var_mult_i_int_slot: &mut f64,
        var_nch_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_dn4_slot: &mut f64,
        var_np_i_dn6_slot: &mut f64,
        var_np_i_dn7_slot: &mut f64,
        var_np_i_dn8_slot: &mut f64,
        var_np_i_dn9_slot: &mut f64,
        var_nsddc_i_slot: &mut f64,
        var_nsub_i_slot: &mut f64,
        var_pdrain_i_slot: &mut f64,
        var_phit0_slot: &mut f64,
        var_phit0_dn4_slot: &mut f64,
        var_phit0_dn6_slot: &mut f64,
        var_phit0_dn7_slot: &mut f64,
        var_phit0_dn8_slot: &mut f64,
        var_phit0_dn9_slot: &mut f64,
        var_pnce_i_slot: &mut f64,
        var_psce1_i_slot: &mut f64,
        var_psce2_i_slot: &mut f64,
        var_pscedlb_i_slot: &mut f64,
        var_psource_i_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_dn4_slot: &mut f64,
        var_rt_dn6_slot: &mut f64,
        var_rt_dn7_slot: &mut f64,
        var_rt_dn8_slot: &mut f64,
        var_rt_dn9_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_dn4_slot: &mut f64,
        var_rtn_dn6_slot: &mut f64,
        var_rtn_dn7_slot: &mut f64,
        var_rtn_dn8_slot: &mut f64,
        var_rtn_dn9_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_swshe_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_tkc_slot: &mut f64,
        var_tkc_dn4_slot: &mut f64,
        var_tkc_dn6_slot: &mut f64,
        var_tkc_dn7_slot: &mut f64,
        var_tkc_dn8_slot: &mut f64,
        var_tkc_dn9_slot: &mut f64,
        var_tkc_sq_slot: &mut f64,
        var_tkc_sq_dn4_slot: &mut f64,
        var_tkc_sq_dn6_slot: &mut f64,
        var_tkc_sq_dn7_slot: &mut f64,
        var_tkc_sq_dn8_slot: &mut f64,
        var_tkc_sq_dn9_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_dn4_slot: &mut f64,
        var_tkd_dn6_slot: &mut f64,
        var_tkd_dn7_slot: &mut f64,
        var_tkd_dn8_slot: &mut f64,
        var_tkd_dn9_slot: &mut f64,
        var_tkr_slot: &mut f64,
        var_tox1_i_slot: &mut f64,
        var_tox2_i_slot: &mut f64,
        var_toxp_i_slot: &mut f64,
        var_tsi_i_slot: &mut f64,
        var_typech_i_slot: &mut f64,
        var_typesub_i_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_xge_i_slot: &mut f64,
        var_xsatmax_slot: &mut f64,
        var_xsatmax_dn4_slot: &mut f64,
        var_xsatmax_dn6_slot: &mut f64,
        var_xsatmax_dn7_slot: &mut f64,
        var_xsatmax_dn8_slot: &mut f64,
        var_xsatmax_dn9_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_adrain_i: f64 = *var_adrain_i_slot;
        let mut var_asource_i: f64 = *var_asource_i_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cic1_i: f64 = *var_cic1_i_slot;
        let mut var_cic2_i: f64 = *var_cic2_i_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_dt: f64 = *var_dt_slot;
        let mut var_dt_dn4: f64 = *var_dt_dn4_slot;
        let mut var_dt_dn6: f64 = *var_dt_dn6_slot;
        let mut var_dt_dn7: f64 = *var_dt_dn7_slot;
        let mut var_dt_dn8: f64 = *var_dt_dn8_slot;
        let mut var_dt_dn9: f64 = *var_dt_dn9_slot;
        let mut var_dtc: f64 = *var_dtc_slot;
        let mut var_dtc_dn4: f64 = *var_dtc_dn4_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_inv_phit0: f64 = *var_inv_phit0_slot;
        let mut var_inv_phit0_dn4: f64 = *var_inv_phit0_dn4_slot;
        let mut var_inv_phit0_dn6: f64 = *var_inv_phit0_dn6_slot;
        let mut var_inv_phit0_dn7: f64 = *var_inv_phit0_dn7_slot;
        let mut var_inv_phit0_dn8: f64 = *var_inv_phit0_dn8_slot;
        let mut var_inv_phit0_dn9: f64 = *var_inv_phit0_dn9_slot;
        let mut var_mult_i_int: f64 = *var_mult_i_int_slot;
        let mut var_nch_i: f64 = *var_nch_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_dn4: f64 = *var_np_i_dn4_slot;
        let mut var_np_i_dn6: f64 = *var_np_i_dn6_slot;
        let mut var_np_i_dn7: f64 = *var_np_i_dn7_slot;
        let mut var_np_i_dn8: f64 = *var_np_i_dn8_slot;
        let mut var_np_i_dn9: f64 = *var_np_i_dn9_slot;
        let mut var_nsddc_i: f64 = *var_nsddc_i_slot;
        let mut var_nsub_i: f64 = *var_nsub_i_slot;
        let mut var_pdrain_i: f64 = *var_pdrain_i_slot;
        let mut var_phit0: f64 = *var_phit0_slot;
        let mut var_phit0_dn4: f64 = *var_phit0_dn4_slot;
        let mut var_phit0_dn6: f64 = *var_phit0_dn6_slot;
        let mut var_phit0_dn7: f64 = *var_phit0_dn7_slot;
        let mut var_phit0_dn8: f64 = *var_phit0_dn8_slot;
        let mut var_phit0_dn9: f64 = *var_phit0_dn9_slot;
        let mut var_pnce_i: f64 = *var_pnce_i_slot;
        let mut var_psce1_i: f64 = *var_psce1_i_slot;
        let mut var_psce2_i: f64 = *var_psce2_i_slot;
        let mut var_pscedlb_i: f64 = *var_pscedlb_i_slot;
        let mut var_psource_i: f64 = *var_psource_i_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_dn4: f64 = *var_rt_dn4_slot;
        let mut var_rt_dn6: f64 = *var_rt_dn6_slot;
        let mut var_rt_dn7: f64 = *var_rt_dn7_slot;
        let mut var_rt_dn8: f64 = *var_rt_dn8_slot;
        let mut var_rt_dn9: f64 = *var_rt_dn9_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_dn4: f64 = *var_rtn_dn4_slot;
        let mut var_rtn_dn6: f64 = *var_rtn_dn6_slot;
        let mut var_rtn_dn7: f64 = *var_rtn_dn7_slot;
        let mut var_rtn_dn8: f64 = *var_rtn_dn8_slot;
        let mut var_rtn_dn9: f64 = *var_rtn_dn9_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_swshe_i: f64 = *var_swshe_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_tkc: f64 = *var_tkc_slot;
        let mut var_tkc_dn4: f64 = *var_tkc_dn4_slot;
        let mut var_tkc_dn6: f64 = *var_tkc_dn6_slot;
        let mut var_tkc_dn7: f64 = *var_tkc_dn7_slot;
        let mut var_tkc_dn8: f64 = *var_tkc_dn8_slot;
        let mut var_tkc_dn9: f64 = *var_tkc_dn9_slot;
        let mut var_tkc_sq: f64 = *var_tkc_sq_slot;
        let mut var_tkc_sq_dn4: f64 = *var_tkc_sq_dn4_slot;
        let mut var_tkc_sq_dn6: f64 = *var_tkc_sq_dn6_slot;
        let mut var_tkc_sq_dn7: f64 = *var_tkc_sq_dn7_slot;
        let mut var_tkc_sq_dn8: f64 = *var_tkc_sq_dn8_slot;
        let mut var_tkc_sq_dn9: f64 = *var_tkc_sq_dn9_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_dn4: f64 = *var_tkd_dn4_slot;
        let mut var_tkd_dn6: f64 = *var_tkd_dn6_slot;
        let mut var_tkd_dn7: f64 = *var_tkd_dn7_slot;
        let mut var_tkd_dn8: f64 = *var_tkd_dn8_slot;
        let mut var_tkd_dn9: f64 = *var_tkd_dn9_slot;
        let mut var_tkr: f64 = *var_tkr_slot;
        let mut var_tox1_i: f64 = *var_tox1_i_slot;
        let mut var_tox2_i: f64 = *var_tox2_i_slot;
        let mut var_toxp_i: f64 = *var_toxp_i_slot;
        let mut var_tsi_i: f64 = *var_tsi_i_slot;
        let mut var_typech_i: f64 = *var_typech_i_slot;
        let mut var_typesub_i: f64 = *var_typesub_i_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_xge_i: f64 = *var_xge_i_slot;
        let mut var_xsatmax: f64 = *var_xsatmax_slot;
        let mut var_xsatmax_dn4: f64 = *var_xsatmax_dn4_slot;
        let mut var_xsatmax_dn6: f64 = *var_xsatmax_dn6_slot;
        let mut var_xsatmax_dn7: f64 = *var_xsatmax_dn7_slot;
        let mut var_xsatmax_dn8: f64 = *var_xsatmax_dn8_slot;
        let mut var_xsatmax_dn9: f64 = *var_xsatmax_dn9_slot;

        let assign00_e799: f64 = (273.15 + p.p15);
        var_tkr = assign00_e799;

        let assign10_e800: f64 = ctx_temp;
        let assign10_e802: f64 = (assign10_e800 + p.p36);
        let assign10_e804: f64 = (assign10_e802).min(1000.0);
        var_temp = assign10_e804;
        var_temp_dn4 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;

        let assign20_e807: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        var_guard1 = assign20_e807;

        let (assign30_e838, assign30_e838_d_n4, assign30_e838_d_n6, assign30_e838_d_n7, assign30_e838_d_n8, assign30_e838_d_n9,) = {
    if (var_guard1 != 0.0) {
        let assign30_e814: f64 = (p.p18 * var_temp);
        let assign30_e815: f64 = (p.p17 + assign30_e814);
        let assign30_e816: f64 = (var_temp + assign30_e815);
        let assign30_e821: f64 = (p.p18 * var_temp);
        let assign30_e822: f64 = (p.p17 + assign30_e821);
        let assign30_e823: f64 = (var_temp - assign30_e822);
        let assign30_e828: f64 = (p.p18 * var_temp);
        let assign30_e829: f64 = (p.p17 + assign30_e828);
        let assign30_e830: f64 = (var_temp - assign30_e829);
        let assign30_e831: f64 = (assign30_e823 * assign30_e830);
        let assign30_e833: f64 = (assign30_e831 + p.p19);
        let assign30_e834: f64 = (assign30_e833).sqrt();
        let assign30_e835: f64 = (assign30_e816 + assign30_e834);
        let assign30_e836: f64 = (0.5 * assign30_e835);
        (assign30_e836, (0.5 * ((var_temp_dn4 + (p.p18 * var_temp_dn4)) + ((((var_temp_dn4 - (p.p18 * var_temp_dn4)) * assign30_e830) + (assign30_e823 * (var_temp_dn4 - (p.p18 * var_temp_dn4)))) / (2.0 * assign30_e834)))), (0.5 * ((var_temp_dn6 + (p.p18 * var_temp_dn6)) + ((((var_temp_dn6 - (p.p18 * var_temp_dn6)) * assign30_e830) + (assign30_e823 * (var_temp_dn6 - (p.p18 * var_temp_dn6)))) / (2.0 * assign30_e834)))), (0.5 * ((var_temp_dn7 + (p.p18 * var_temp_dn7)) + ((((var_temp_dn7 - (p.p18 * var_temp_dn7)) * assign30_e830) + (assign30_e823 * (var_temp_dn7 - (p.p18 * var_temp_dn7)))) / (2.0 * assign30_e834)))), (0.5 * ((var_temp_dn8 + (p.p18 * var_temp_dn8)) + ((((var_temp_dn8 - (p.p18 * var_temp_dn8)) * assign30_e830) + (assign30_e823 * (var_temp_dn8 - (p.p18 * var_temp_dn8)))) / (2.0 * assign30_e834)))), (0.5 * ((var_temp_dn9 + (p.p18 * var_temp_dn9)) + ((((var_temp_dn9 - (p.p18 * var_temp_dn9)) * assign30_e830) + (assign30_e823 * (var_temp_dn9 - (p.p18 * var_temp_dn9)))) / (2.0 * assign30_e834)))),)
    } else {
        (var_tkd, var_tkd_dn4, var_tkd_dn6, var_tkd_dn7, var_tkd_dn8, var_tkd_dn9,)
    }
};
        var_tkd = assign30_e838;
        var_tkd_dn4 = assign30_e838_d_n4;
        var_tkd_dn6 = assign30_e838_d_n6;
        var_tkd_dn7 = assign30_e838_d_n7;
        var_tkd_dn8 = assign30_e838_d_n8;
        var_tkd_dn9 = assign30_e838_d_n9;

        let (assign40_e869, assign40_e869_d_n4, assign40_e869_d_n6, assign40_e869_d_n7, assign40_e869_d_n8, assign40_e869_d_n9,) = {
    if (var_guard1 != 0.0) {
        let assign40_e844: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e845: f64 = (10.0 / assign40_e844);
        let assign40_e847: f64 = (assign40_e845 + 600.0);
        let assign40_e851: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e852: f64 = (10.0 / assign40_e851);
        let assign40_e854: f64 = (assign40_e852 - 600.0);
        let assign40_e858: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e859: f64 = (10.0 / assign40_e858);
        let assign40_e861: f64 = (assign40_e859 - 600.0);
        let assign40_e862: f64 = (assign40_e854 * assign40_e861);
        let assign40_e864: f64 = (assign40_e862 + 0.01);
        let assign40_e865: f64 = (assign40_e864).sqrt();
        let assign40_e866: f64 = (assign40_e847 + assign40_e865);
        let assign40_e867: f64 = (0.5 * assign40_e866);
        (assign40_e867, (0.5 * ((-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))), (0.5 * ((-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e844 * assign40_e844))) + ((((-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e851 * assign40_e851))) * assign40_e861) + (assign40_e854 * (-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e858 * assign40_e858))))) / (2.0 * assign40_e865)))),)
    } else {
        (var_xsatmax, var_xsatmax_dn4, var_xsatmax_dn6, var_xsatmax_dn7, var_xsatmax_dn8, var_xsatmax_dn9,)
    }
};
        var_xsatmax = assign40_e869;
        var_xsatmax_dn4 = assign40_e869_d_n4;
        var_xsatmax_dn6 = assign40_e869_d_n6;
        var_xsatmax_dn7 = assign40_e869_d_n7;
        var_xsatmax_dn8 = assign40_e869_d_n8;
        var_xsatmax_dn9 = assign40_e869_d_n9;

        let (assign50_e889, assign50_e889_d_n4, assign50_e889_d_n6, assign50_e889_d_n7, assign50_e889_d_n8, assign50_e889_d_n9,) = {
    if (var_guard1 == 0.0) {
        let assign50_e875: f64 = (var_temp + 1.0);
        let assign50_e878: f64 = (var_temp - 1.0);
        let assign50_e881: f64 = (var_temp - 1.0);
        let assign50_e882: f64 = (assign50_e878 * assign50_e881);
        let assign50_e884: f64 = (assign50_e882 + 0.001);
        let assign50_e885: f64 = (assign50_e884).sqrt();
        let assign50_e886: f64 = (assign50_e875 + assign50_e885);
        let assign50_e887: f64 = (0.5 * assign50_e886);
        (assign50_e887, (0.5 * (var_temp_dn4 + (((var_temp_dn4 * assign50_e881) + (assign50_e878 * var_temp_dn4)) / (2.0 * assign50_e885)))), (0.5 * (var_temp_dn6 + (((var_temp_dn6 * assign50_e881) + (assign50_e878 * var_temp_dn6)) / (2.0 * assign50_e885)))), (0.5 * (var_temp_dn7 + (((var_temp_dn7 * assign50_e881) + (assign50_e878 * var_temp_dn7)) / (2.0 * assign50_e885)))), (0.5 * (var_temp_dn8 + (((var_temp_dn8 * assign50_e881) + (assign50_e878 * var_temp_dn8)) / (2.0 * assign50_e885)))), (0.5 * (var_temp_dn9 + (((var_temp_dn9 * assign50_e881) + (assign50_e878 * var_temp_dn9)) / (2.0 * assign50_e885)))),)
    } else {
        (var_tkd, var_tkd_dn4, var_tkd_dn6, var_tkd_dn7, var_tkd_dn8, var_tkd_dn9,)
    }
};
        var_tkd = assign50_e889;
        var_tkd_dn4 = assign50_e889_d_n4;
        var_tkd_dn6 = assign50_e889_d_n6;
        var_tkd_dn7 = assign50_e889_d_n7;
        var_tkd_dn8 = assign50_e889_d_n8;
        var_tkd_dn9 = assign50_e889_d_n9;

        let (assign60_e894, assign60_e894_d_n4, assign60_e894_d_n6, assign60_e894_d_n7, assign60_e894_d_n8, assign60_e894_d_n9,) = {
    if (var_guard1 == 0.0) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xsatmax, var_xsatmax_dn4, var_xsatmax_dn6, var_xsatmax_dn7, var_xsatmax_dn8, var_xsatmax_dn9,)
    }
};
        var_xsatmax = assign60_e894;
        var_xsatmax_dn4 = assign60_e894_d_n4;
        var_xsatmax_dn6 = assign60_e894_d_n6;
        var_xsatmax_dn7 = assign60_e894_d_n7;
        var_xsatmax_dn8 = assign60_e894_d_n8;
        var_xsatmax_dn9 = assign60_e894_d_n9;

        let assign70_e909: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p443 > 0.0))) { 1.0 } else { 0.0 };
        var_guard2 = assign70_e909;

        let (assign80_e913,) = {
    if (var_guard2 != 0.0) {
        (p.p5,)
    } else {
        (var_swshe_i,)
    }
};
        var_swshe_i = assign80_e913;

        let (assign90_e918,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swshe_i,)
    }
};
        var_swshe_i = assign90_e918;

        var_dtc = 0.0;
        var_dtc_dn4 = 0.0;

        var_tkc = var_tkd;
        var_tkc_dn4 = var_tkd_dn4;
        var_tkc_dn6 = var_tkd_dn6;
        var_tkc_dn7 = var_tkd_dn7;
        var_tkc_dn8 = var_tkd_dn8;
        var_tkc_dn9 = var_tkd_dn9;

        let assign140_e928: f64 = (var_tkc * var_tkc);
        var_tkc_sq = assign140_e928;
        var_tkc_sq_dn4 = ((var_tkc_dn4 * var_tkc) + (var_tkc * var_tkc_dn4));
        var_tkc_sq_dn6 = ((var_tkc_dn6 * var_tkc) + (var_tkc * var_tkc_dn6));
        var_tkc_sq_dn7 = ((var_tkc_dn7 * var_tkc) + (var_tkc * var_tkc_dn7));
        var_tkc_sq_dn8 = ((var_tkc_dn8 * var_tkc) + (var_tkc * var_tkc_dn8));
        var_tkc_sq_dn9 = ((var_tkc_dn9 * var_tkc) + (var_tkc * var_tkc_dn9));

        let assign150_e931: f64 = (var_tkc - var_tkr);
        var_dt = assign150_e931;
        var_dt_dn4 = var_tkc_dn4;
        var_dt_dn6 = var_tkc_dn6;
        var_dt_dn7 = var_tkc_dn7;
        var_dt_dn8 = var_tkc_dn8;
        var_dt_dn9 = var_tkc_dn9;

        let assign160_e934: f64 = (var_tkc / var_tkr);
        var_rt = assign160_e934;
        var_rt_dn4 = (var_tkc_dn4 / var_tkr);
        var_rt_dn6 = (var_tkc_dn6 / var_tkr);
        var_rt_dn7 = (var_tkc_dn7 / var_tkr);
        var_rt_dn8 = (var_tkc_dn8 / var_tkr);
        var_rt_dn9 = (var_tkc_dn9 / var_tkr);

        let assign170_e937: f64 = (var_tkr / var_tkc);
        var_rtn = assign170_e937;
        var_rtn_dn4 = (-((var_tkr * var_tkc_dn4) / (var_tkc * var_tkc)));
        var_rtn_dn6 = (-((var_tkr * var_tkc_dn6) / (var_tkc * var_tkc)));
        var_rtn_dn7 = (-((var_tkr * var_tkc_dn7) / (var_tkc * var_tkc)));
        var_rtn_dn8 = (-((var_tkr * var_tkc_dn8) / (var_tkc * var_tkc)));
        var_rtn_dn9 = (-((var_tkr * var_tkc_dn9) / (var_tkc * var_tkc)));

        let assign180_e940: f64 = (var_tkc * 8.617332384961e-5);
        var_phit0 = assign180_e940;
        var_phit0_dn4 = (var_tkc_dn4 * 8.617332384961e-5);
        var_phit0_dn6 = (var_tkc_dn6 * 8.617332384961e-5);
        var_phit0_dn7 = (var_tkc_dn7 * 8.617332384961e-5);
        var_phit0_dn8 = (var_tkc_dn8 * 8.617332384961e-5);
        var_phit0_dn9 = (var_tkc_dn9 * 8.617332384961e-5);

        let assign190_e943: f64 = (1.0 / var_phit0);
        var_inv_phit0 = assign190_e943;
        var_inv_phit0_dn4 = (-(var_phit0_dn4 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn6 = (-(var_phit0_dn6 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn7 = (-(var_phit0_dn7 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn8 = (-(var_phit0_dn8 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn9 = (-(var_phit0_dn9 / (var_phit0 * var_phit0)));

        let assign200_e946: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard83 = assign200_e946;

        let (assign210_e950,) = {
    if (var_guard83 != 0.0) {
        (p.p23,)
    } else {
        (var_adrain_i,)
    }
};
        var_adrain_i = assign210_e950;

        let (assign220_e954,) = {
    if (var_guard83 != 0.0) {
        (p.p22,)
    } else {
        (var_asource_i,)
    }
};
        var_asource_i = assign220_e954;

        let (assign230_e958,) = {
    if (var_guard83 != 0.0) {
        (p.p25,)
    } else {
        (var_pdrain_i,)
    }
};
        var_pdrain_i = assign230_e958;

        let (assign240_e962,) = {
    if (var_guard83 != 0.0) {
        (p.p24,)
    } else {
        (var_psource_i,)
    }
};
        var_psource_i = assign240_e962;

        let (assign250_e966,) = {
    if (var_guard83 != 0.0) {
        (p.p30,)
    } else {
        (var_mult_i_int,)
    }
};
        var_mult_i_int = assign250_e966;

        let (assign260_e970,) = {
    if (var_guard83 != 0.0) {
        (p.p41,)
    } else {
        (var_tox1_i,)
    }
};
        var_tox1_i = assign260_e970;

        let (assign270_e974,) = {
    if (var_guard83 != 0.0) {
        (p.p42,)
    } else {
        (var_tsi_i,)
    }
};
        var_tsi_i = assign270_e974;

        let (assign280_e978,) = {
    if (var_guard83 != 0.0) {
        (p.p43,)
    } else {
        (var_xge_i,)
    }
};
        var_xge_i = assign280_e978;

        let (assign290_e982,) = {
    if (var_guard83 != 0.0) {
        (p.p44,)
    } else {
        (var_tox2_i,)
    }
};
        var_tox2_i = assign290_e982;

        let (assign300_e986,) = {
    if (var_guard83 != 0.0) {
        (1.0,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign300_e986;

        let assign310_e989: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        var_guard84 = assign310_e989;

        let (assign320_e996,) = {
    if ((var_guard83 != 0.0) && (var_guard84 != 0.0)) {
        let assign320_e994: f64 = (-1.0);
        (assign320_e994,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign320_e996;

        let (assign330_e1005,) = {
    if (var_guard83 != 0.0) {
        let assign330_e999: f64 = (p.p45).abs();
        let assign330_e1001: f64 = (assign330_e999).min(1e19);
        let assign330_e1003: f64 = (assign330_e1001 * 1000000.0);
        (assign330_e1003,)
    } else {
        (var_nch_i,)
    }
};
        var_nch_i = assign330_e1005;

        let (assign340_e1009,) = {
    if (var_guard83 != 0.0) {
        (1.0,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign340_e1009;

        let assign350_e1012: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        var_guard85 = assign350_e1012;

        let (assign360_e1019,) = {
    if ((var_guard83 != 0.0) && (var_guard85 != 0.0)) {
        let assign360_e1017: f64 = (-1.0);
        (assign360_e1017,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign360_e1019;

        let (assign370_e1030,) = {
    if (var_guard83 != 0.0) {
        let assign370_e1022: f64 = (p.p46).abs();
        let assign370_e1024: f64 = (assign370_e1022).max(1e16);
        let assign370_e1026: f64 = (assign370_e1024).min(1e21);
        let assign370_e1028: f64 = (assign370_e1026 * 1000000.0);
        (assign370_e1028,)
    } else {
        (var_nsub_i,)
    }
};
        var_nsub_i = assign370_e1030;

        let (assign380_e1034,) = {
    if (var_guard83 != 0.0) {
        (p.p47,)
    } else {
        (var_ct_i,)
    }
};
        var_ct_i = assign380_e1034;

        let (assign390_e1038,) = {
    if (var_guard83 != 0.0) {
        (p.p48,)
    } else {
        (var_toxp_i,)
    }
};
        var_toxp_i = assign390_e1038;

        let (assign400_e1044,) = {
    if (var_guard83 != 0.0) {
        let assign400_e1042: f64 = (p.p49 * 1000000.0);
        (assign400_e1042,)
    } else {
        (var_nov_i,)
    }
};
        var_nov_i = assign400_e1044;

        let (assign410_e1050,) = {
    if (var_guard83 != 0.0) {
        let assign410_e1048: f64 = (p.p50 * 1000000.0);
        (assign410_e1048,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign410_e1050;

        let (assign420_e1054, assign420_e1054_d_n4, assign420_e1054_d_n6, assign420_e1054_d_n7, assign420_e1054_d_n8, assign420_e1054_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign420_e1054;
        var_vfb1_t_dn4 = assign420_e1054_d_n4;
        var_vfb1_t_dn6 = assign420_e1054_d_n6;
        var_vfb1_t_dn7 = assign420_e1054_d_n7;
        var_vfb1_t_dn8 = assign420_e1054_d_n8;
        var_vfb1_t_dn9 = assign420_e1054_d_n9;

        let (assign430_e1058, assign430_e1058_d_n4, assign430_e1058_d_n6, assign430_e1058_d_n7, assign430_e1058_d_n8, assign430_e1058_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign430_e1058;
        var_vfb2_t_dn4 = assign430_e1058_d_n4;
        var_vfb2_t_dn6 = assign430_e1058_d_n6;
        var_vfb2_t_dn7 = assign430_e1058_d_n7;
        var_vfb2_t_dn8 = assign430_e1058_d_n8;
        var_vfb2_t_dn9 = assign430_e1058_d_n9;

        let (assign440_e1062,) = {
    if (var_guard83 != 0.0) {
        (p.p53,)
    } else {
        (var_stvfb_i,)
    }
};
        var_stvfb_i = assign440_e1062;

        let (assign450_e1068, assign450_e1068_d_n4, assign450_e1068_d_n6, assign450_e1068_d_n7, assign450_e1068_d_n8, assign450_e1068_d_n9,) = {
    if (var_guard83 != 0.0) {
        let assign450_e1066: f64 = (p.p54 * 1000000.0);
        (assign450_e1066, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_np_i, var_np_i_dn4, var_np_i_dn6, var_np_i_dn7, var_np_i_dn8, var_np_i_dn9,)
    }
};
        var_np_i = assign450_e1068;
        var_np_i_dn4 = assign450_e1068_d_n4;
        var_np_i_dn6 = assign450_e1068_d_n6;
        var_np_i_dn7 = assign450_e1068_d_n7;
        var_np_i_dn8 = assign450_e1068_d_n8;
        var_np_i_dn9 = assign450_e1068_d_n9;

        let (assign460_e1072,) = {
    if (var_guard83 != 0.0) {
        (p.p55,)
    } else {
        (var_cic1_i,)
    }
};
        var_cic1_i = assign460_e1072;

        let (assign470_e1076,) = {
    if (var_guard83 != 0.0) {
        (p.p56,)
    } else {
        (var_cic2_i,)
    }
};
        var_cic2_i = assign470_e1076;

        let (assign480_e1080,) = {
    if (var_guard83 != 0.0) {
        (p.p57,)
    } else {
        (var_psce1_i,)
    }
};
        var_psce1_i = assign480_e1080;

        let (assign490_e1090,) = {
    if (var_guard83 != 0.0) {
        let assign490_e1084: f64 = (p.p58 * var_psce1_i);
        let assign490_e1086: f64 = (assign490_e1084 * var_tox2_i);
        let assign490_e1088: f64 = (assign490_e1086 / var_tox1_i);
        (assign490_e1088,)
    } else {
        (var_psce2_i,)
    }
};
        var_psce2_i = assign490_e1090;

        let (assign500_e1096,) = {
    if (var_guard83 != 0.0) {
        let assign500_e1094: f64 = (p.p59 * 1000000.0);
        (assign500_e1094,)
    } else {
        (var_nsddc_i,)
    }
};
        var_nsddc_i = assign500_e1096;

        let (assign510_e1100,) = {
    if (var_guard83 != 0.0) {
        (p.p60,)
    } else {
        (var_pscedlb_i,)
    }
};
        var_pscedlb_i = assign510_e1100;

        let (assign520_e1104,) = {
    if (var_guard83 != 0.0) {
        (p.p61,)
    } else {
        (var_pnce_i,)
    }
};
        var_pnce_i = assign520_e1104;

        let (assign530_e1108, assign530_e1108_d_n4, assign530_e1108_d_n6, assign530_e1108_d_n7, assign530_e1108_d_n8, assign530_e1108_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign530_e1108;
        var_cf1_t_dn4 = assign530_e1108_d_n4;
        var_cf1_t_dn6 = assign530_e1108_d_n6;
        var_cf1_t_dn7 = assign530_e1108_d_n7;
        var_cf1_t_dn8 = assign530_e1108_d_n8;
        var_cf1_t_dn9 = assign530_e1108_d_n9;

        *var_adrain_i_slot = var_adrain_i;
        *var_asource_i_slot = var_asource_i;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cic1_i_slot = var_cic1_i;
        *var_cic2_i_slot = var_cic2_i;
        *var_ct_i_slot = var_ct_i;
        *var_dt_slot = var_dt;
        *var_dt_dn4_slot = var_dt_dn4;
        *var_dt_dn6_slot = var_dt_dn6;
        *var_dt_dn7_slot = var_dt_dn7;
        *var_dt_dn8_slot = var_dt_dn8;
        *var_dt_dn9_slot = var_dt_dn9;
        *var_dtc_slot = var_dtc;
        *var_dtc_dn4_slot = var_dtc_dn4;
        *var_guard1_slot = var_guard1;
        *var_guard2_slot = var_guard2;
        *var_guard83_slot = var_guard83;
        *var_guard84_slot = var_guard84;
        *var_guard85_slot = var_guard85;
        *var_inv_phit0_slot = var_inv_phit0;
        *var_inv_phit0_dn4_slot = var_inv_phit0_dn4;
        *var_inv_phit0_dn6_slot = var_inv_phit0_dn6;
        *var_inv_phit0_dn7_slot = var_inv_phit0_dn7;
        *var_inv_phit0_dn8_slot = var_inv_phit0_dn8;
        *var_inv_phit0_dn9_slot = var_inv_phit0_dn9;
        *var_mult_i_int_slot = var_mult_i_int;
        *var_nch_i_slot = var_nch_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_np_i_slot = var_np_i;
        *var_np_i_dn4_slot = var_np_i_dn4;
        *var_np_i_dn6_slot = var_np_i_dn6;
        *var_np_i_dn7_slot = var_np_i_dn7;
        *var_np_i_dn8_slot = var_np_i_dn8;
        *var_np_i_dn9_slot = var_np_i_dn9;
        *var_nsddc_i_slot = var_nsddc_i;
        *var_nsub_i_slot = var_nsub_i;
        *var_pdrain_i_slot = var_pdrain_i;
        *var_phit0_slot = var_phit0;
        *var_phit0_dn4_slot = var_phit0_dn4;
        *var_phit0_dn6_slot = var_phit0_dn6;
        *var_phit0_dn7_slot = var_phit0_dn7;
        *var_phit0_dn8_slot = var_phit0_dn8;
        *var_phit0_dn9_slot = var_phit0_dn9;
        *var_pnce_i_slot = var_pnce_i;
        *var_psce1_i_slot = var_psce1_i;
        *var_psce2_i_slot = var_psce2_i;
        *var_pscedlb_i_slot = var_pscedlb_i;
        *var_psource_i_slot = var_psource_i;
        *var_rt_slot = var_rt;
        *var_rt_dn4_slot = var_rt_dn4;
        *var_rt_dn6_slot = var_rt_dn6;
        *var_rt_dn7_slot = var_rt_dn7;
        *var_rt_dn8_slot = var_rt_dn8;
        *var_rt_dn9_slot = var_rt_dn9;
        *var_rtn_slot = var_rtn;
        *var_rtn_dn4_slot = var_rtn_dn4;
        *var_rtn_dn6_slot = var_rtn_dn6;
        *var_rtn_dn7_slot = var_rtn_dn7;
        *var_rtn_dn8_slot = var_rtn_dn8;
        *var_rtn_dn9_slot = var_rtn_dn9;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_swshe_i_slot = var_swshe_i;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_tkc_slot = var_tkc;
        *var_tkc_dn4_slot = var_tkc_dn4;
        *var_tkc_dn6_slot = var_tkc_dn6;
        *var_tkc_dn7_slot = var_tkc_dn7;
        *var_tkc_dn8_slot = var_tkc_dn8;
        *var_tkc_dn9_slot = var_tkc_dn9;
        *var_tkc_sq_slot = var_tkc_sq;
        *var_tkc_sq_dn4_slot = var_tkc_sq_dn4;
        *var_tkc_sq_dn6_slot = var_tkc_sq_dn6;
        *var_tkc_sq_dn7_slot = var_tkc_sq_dn7;
        *var_tkc_sq_dn8_slot = var_tkc_sq_dn8;
        *var_tkc_sq_dn9_slot = var_tkc_sq_dn9;
        *var_tkd_slot = var_tkd;
        *var_tkd_dn4_slot = var_tkd_dn4;
        *var_tkd_dn6_slot = var_tkd_dn6;
        *var_tkd_dn7_slot = var_tkd_dn7;
        *var_tkd_dn8_slot = var_tkd_dn8;
        *var_tkd_dn9_slot = var_tkd_dn9;
        *var_tkr_slot = var_tkr;
        *var_tox1_i_slot = var_tox1_i;
        *var_tox2_i_slot = var_tox2_i;
        *var_toxp_i_slot = var_toxp_i;
        *var_tsi_i_slot = var_tsi_i;
        *var_typech_i_slot = var_typech_i;
        *var_typesub_i_slot = var_typesub_i;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_xge_i_slot = var_xge_i;
        *var_xsatmax_slot = var_xsatmax;
        *var_xsatmax_dn4_slot = var_xsatmax_dn4;
        *var_xsatmax_dn6_slot = var_xsatmax_dn6;
        *var_xsatmax_dn7_slot = var_xsatmax_dn7;
        *var_xsatmax_dn8_slot = var_xsatmax_dn8;
        *var_xsatmax_dn9_slot = var_xsatmax_dn9;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_guard83: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_alp1_i_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alpb_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfdl_i_slot: &mut f64,
        var_cfdlb_i_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_csbi_i_slot: &mut f64,
        var_csfi_i_slot: &mut f64,
        var_csthr_i_slot: &mut f64,
        var_csthrb_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_fnovinv_t_slot: &mut f64,
        var_fnovinvd_t_slot: &mut f64,
        var_gc2ch_i_slot: &mut f64,
        var_gc2ovinv_i_slot: &mut f64,
        var_gc3ch_i_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_iginv_t_slot: &mut f64,
        var_igovacc_t_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovinv_t_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsig_i_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stcf_i_slot: &mut f64,
        var_stcf_i_dn4_slot: &mut f64,
        var_stcf_i_dn6_slot: &mut f64,
        var_stcf_i_dn7_slot: &mut f64,
        var_stcf_i_dn8_slot: &mut f64,
        var_stcf_i_dn9_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stigfn_i_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_thersg_i_slot: &mut f64,
        var_thesat1_i_slot: &mut f64,
        var_thesat2_i_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vpg_i_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcorb_i_slot: &mut f64,
    ) {
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alpb_i: f64 = *var_alpb_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfdl_i: f64 = *var_cfdl_i_slot;
        let mut var_cfdlb_i: f64 = *var_cfdlb_i_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_csbi_i: f64 = *var_csbi_i_slot;
        let mut var_csfi_i: f64 = *var_csfi_i_slot;
        let mut var_csthr_i: f64 = *var_csthr_i_slot;
        let mut var_csthrb_i: f64 = *var_csthrb_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_fnovinv_t: f64 = *var_fnovinv_t_slot;
        let mut var_fnovinvd_t: f64 = *var_fnovinvd_t_slot;
        let mut var_gc2ch_i: f64 = *var_gc2ch_i_slot;
        let mut var_gc2ovinv_i: f64 = *var_gc2ovinv_i_slot;
        let mut var_gc3ch_i: f64 = *var_gc3ch_i_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_iginv_t: f64 = *var_iginv_t_slot;
        let mut var_igovacc_t: f64 = *var_igovacc_t_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovinv_t: f64 = *var_igovinv_t_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsig_i: f64 = *var_rsig_i_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stcf_i: f64 = *var_stcf_i_slot;
        let mut var_stcf_i_dn4: f64 = *var_stcf_i_dn4_slot;
        let mut var_stcf_i_dn6: f64 = *var_stcf_i_dn6_slot;
        let mut var_stcf_i_dn7: f64 = *var_stcf_i_dn7_slot;
        let mut var_stcf_i_dn8: f64 = *var_stcf_i_dn8_slot;
        let mut var_stcf_i_dn9: f64 = *var_stcf_i_dn9_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stigfn_i: f64 = *var_stigfn_i_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_thersg_i: f64 = *var_thersg_i_slot;
        let mut var_thesat1_i: f64 = *var_thesat1_i_slot;
        let mut var_thesat2_i: f64 = *var_thesat2_i_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vpg_i: f64 = *var_vpg_i_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcorb_i: f64 = *var_xcorb_i_slot;

        let (assign540_e1118, assign540_e1118_d_n4, assign540_e1118_d_n6, assign540_e1118_d_n7, assign540_e1118_d_n8, assign540_e1118_d_n9,) = {
    if (var_guard83 != 0.0) {
        let assign540_e1112: f64 = (p.p63 * var_cf1_t);
        let assign540_e1114: f64 = (assign540_e1112 * var_tox2_i);
        let assign540_e1116: f64 = (assign540_e1114 / var_tox1_i);
        (assign540_e1116, (((p.p63 * var_cf1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign540_e1118;
        var_cf2_t_dn4 = assign540_e1118_d_n4;
        var_cf2_t_dn6 = assign540_e1118_d_n6;
        var_cf2_t_dn7 = assign540_e1118_d_n7;
        var_cf2_t_dn8 = assign540_e1118_d_n8;
        var_cf2_t_dn9 = assign540_e1118_d_n9;

        let (assign550_e1122, assign550_e1122_d_n4, assign550_e1122_d_n6, assign550_e1122_d_n7, assign550_e1122_d_n8, assign550_e1122_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_stcf_i, var_stcf_i_dn4, var_stcf_i_dn6, var_stcf_i_dn7, var_stcf_i_dn8, var_stcf_i_dn9,)
    }
};
        var_stcf_i = assign550_e1122;
        var_stcf_i_dn4 = assign550_e1122_d_n4;
        var_stcf_i_dn6 = assign550_e1122_d_n6;
        var_stcf_i_dn7 = assign550_e1122_d_n7;
        var_stcf_i_dn8 = assign550_e1122_d_n8;
        var_stcf_i_dn9 = assign550_e1122_d_n9;

        let (assign560_e1126,) = {
    if (var_guard83 != 0.0) {
        (p.p65,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign560_e1126;

        let (assign570_e1130,) = {
    if (var_guard83 != 0.0) {
        (p.p66,)
    } else {
        (var_cfdl_i,)
    }
};
        var_cfdl_i = assign570_e1130;

        let (assign580_e1134,) = {
    if (var_guard83 != 0.0) {
        (p.p67,)
    } else {
        (var_cfdlb_i,)
    }
};
        var_cfdlb_i = assign580_e1134;

        let (assign590_e1138, assign590_e1138_d_n4, assign590_e1138_d_n6, assign590_e1138_d_n7, assign590_e1138_d_n8, assign590_e1138_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign590_e1138;
        var_betn1_t_dn4 = assign590_e1138_d_n4;
        var_betn1_t_dn6 = assign590_e1138_d_n6;
        var_betn1_t_dn7 = assign590_e1138_d_n7;
        var_betn1_t_dn8 = assign590_e1138_d_n8;
        var_betn1_t_dn9 = assign590_e1138_d_n9;

        let (assign600_e1144, assign600_e1144_d_n4, assign600_e1144_d_n6, assign600_e1144_d_n7, assign600_e1144_d_n8, assign600_e1144_d_n9,) = {
    if (var_guard83 != 0.0) {
        let assign600_e1142: f64 = (p.p69 * var_betn1_t);
        (assign600_e1142, (p.p69 * var_betn1_t_dn4), (p.p69 * var_betn1_t_dn6), (p.p69 * var_betn1_t_dn7), (p.p69 * var_betn1_t_dn8), (p.p69 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign600_e1144;
        var_betn2_t_dn4 = assign600_e1144_d_n4;
        var_betn2_t_dn6 = assign600_e1144_d_n6;
        var_betn2_t_dn7 = assign600_e1144_d_n7;
        var_betn2_t_dn8 = assign600_e1144_d_n8;
        var_betn2_t_dn9 = assign600_e1144_d_n9;

        let (assign610_e1148,) = {
    if (var_guard83 != 0.0) {
        (p.p70,)
    } else {
        (var_stbet_i,)
    }
};
        var_stbet_i = assign610_e1148;

        let (assign620_e1152,) = {
    if (var_guard83 != 0.0) {
        (p.p71,)
    } else {
        (var_cs_t,)
    }
};
        var_cs_t = assign620_e1152;

        let (assign630_e1156,) = {
    if (var_guard83 != 0.0) {
        (p.p72,)
    } else {
        (var_csfi_i,)
    }
};
        var_csfi_i = assign630_e1156;

        let (assign640_e1160,) = {
    if (var_guard83 != 0.0) {
        (p.p73,)
    } else {
        (var_csbi_i,)
    }
};
        var_csbi_i = assign640_e1160;

        let (assign650_e1164,) = {
    if (var_guard83 != 0.0) {
        (p.p74,)
    } else {
        (var_stcs_i,)
    }
};
        var_stcs_i = assign650_e1164;

        let (assign660_e1168,) = {
    if (var_guard83 != 0.0) {
        (p.p75,)
    } else {
        (var_thecs_t,)
    }
};
        var_thecs_t = assign660_e1168;

        let (assign670_e1172,) = {
    if (var_guard83 != 0.0) {
        (p.p76,)
    } else {
        (var_stthecs_i,)
    }
};
        var_stthecs_i = assign670_e1172;

        let (assign680_e1176,) = {
    if (var_guard83 != 0.0) {
        (p.p77,)
    } else {
        (var_csthr_i,)
    }
};
        var_csthr_i = assign680_e1176;

        let (assign690_e1180,) = {
    if (var_guard83 != 0.0) {
        (p.p78,)
    } else {
        (var_csthrb_i,)
    }
};
        var_csthrb_i = assign690_e1180;

        let (assign700_e1184,) = {
    if (var_guard83 != 0.0) {
        (p.p79,)
    } else {
        (var_mue_t,)
    }
};
        var_mue_t = assign700_e1184;

        let (assign710_e1188,) = {
    if (var_guard83 != 0.0) {
        (p.p80,)
    } else {
        (var_stmue_i,)
    }
};
        var_stmue_i = assign710_e1188;

        let (assign720_e1192,) = {
    if (var_guard83 != 0.0) {
        (p.p81,)
    } else {
        (var_themu_t,)
    }
};
        var_themu_t = assign720_e1192;

        let (assign730_e1196,) = {
    if (var_guard83 != 0.0) {
        (p.p82,)
    } else {
        (var_stthemu_i,)
    }
};
        var_stthemu_i = assign730_e1196;

        let (assign740_e1200,) = {
    if (var_guard83 != 0.0) {
        (p.p83,)
    } else {
        (var_xcor_t,)
    }
};
        var_xcor_t = assign740_e1200;

        let (assign750_e1204,) = {
    if (var_guard83 != 0.0) {
        (p.p84,)
    } else {
        (var_xcorb_i,)
    }
};
        var_xcorb_i = assign750_e1204;

        let (assign760_e1208,) = {
    if (var_guard83 != 0.0) {
        (p.p85,)
    } else {
        (var_stxcor_i,)
    }
};
        var_stxcor_i = assign760_e1208;

        let (assign770_e1212,) = {
    if (var_guard83 != 0.0) {
        (p.p86,)
    } else {
        (var_feta_i,)
    }
};
        var_feta_i = assign770_e1212;

        let (assign780_e1216,) = {
    if (var_guard83 != 0.0) {
        (p.p87,)
    } else {
        (var_rs_t,)
    }
};
        var_rs_t = assign780_e1216;

        let (assign790_e1220,) = {
    if (var_guard83 != 0.0) {
        (p.p88,)
    } else {
        (var_rsig_i,)
    }
};
        var_rsig_i = assign790_e1220;

        let (assign800_e1224,) = {
    if (var_guard83 != 0.0) {
        (p.p89,)
    } else {
        (var_strs_i,)
    }
};
        var_strs_i = assign800_e1224;

        let (assign810_e1228,) = {
    if (var_guard83 != 0.0) {
        (p.p90,)
    } else {
        (var_rsg_i,)
    }
};
        var_rsg_i = assign810_e1228;

        let (assign820_e1232,) = {
    if (var_guard83 != 0.0) {
        (p.p91,)
    } else {
        (var_thersg_i,)
    }
};
        var_thersg_i = assign820_e1232;

        let (assign830_e1236,) = {
    if (var_guard83 != 0.0) {
        (p.p92,)
    } else {
        (var_rsb_i,)
    }
};
        var_rsb_i = assign830_e1236;

        let (assign840_e1240, assign840_e1240_d_n4, assign840_e1240_d_n6, assign840_e1240_d_n7, assign840_e1240_d_n8, assign840_e1240_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign840_e1240;
        var_thesat_t_dn4 = assign840_e1240_d_n4;
        var_thesat_t_dn6 = assign840_e1240_d_n6;
        var_thesat_t_dn7 = assign840_e1240_d_n7;
        var_thesat_t_dn8 = assign840_e1240_d_n8;
        var_thesat_t_dn9 = assign840_e1240_d_n9;

        let (assign850_e1244,) = {
    if (var_guard83 != 0.0) {
        (p.p94,)
    } else {
        (var_stthesat_i,)
    }
};
        var_stthesat_i = assign850_e1244;

        let (assign860_e1248,) = {
    if (var_guard83 != 0.0) {
        (p.p95,)
    } else {
        (var_thesat1_i,)
    }
};
        var_thesat1_i = assign860_e1248;

        let (assign870_e1252,) = {
    if (var_guard83 != 0.0) {
        (p.p96,)
    } else {
        (var_thesat2_i,)
    }
};
        var_thesat2_i = assign870_e1252;

        let (assign880_e1256,) = {
    if (var_guard83 != 0.0) {
        (p.p97,)
    } else {
        (var_ax_i,)
    }
};
        var_ax_i = assign880_e1256;

        let (assign890_e1260,) = {
    if (var_guard83 != 0.0) {
        (p.p98,)
    } else {
        (var_alp_i,)
    }
};
        var_alp_i = assign890_e1260;

        let (assign900_e1264,) = {
    if (var_guard83 != 0.0) {
        (p.p99,)
    } else {
        (var_alp1_i,)
    }
};
        var_alp1_i = assign900_e1264;

        let (assign910_e1268,) = {
    if (var_guard83 != 0.0) {
        (p.p100,)
    } else {
        (var_alpb_i,)
    }
};
        var_alpb_i = assign910_e1268;

        let (assign920_e1272,) = {
    if (var_guard83 != 0.0) {
        (p.p101,)
    } else {
        (var_vp_i,)
    }
};
        var_vp_i = assign920_e1272;

        let (assign930_e1276,) = {
    if (var_guard83 != 0.0) {
        (p.p102,)
    } else {
        (var_vpg_i,)
    }
};
        var_vpg_i = assign930_e1276;

        let (assign940_e1280,) = {
    if (var_guard83 != 0.0) {
        (p.p103,)
    } else {
        (var_gco_i,)
    }
};
        var_gco_i = assign940_e1280;

        let (assign950_e1284,) = {
    if (var_guard83 != 0.0) {
        (p.p104,)
    } else {
        (var_iginv_t,)
    }
};
        var_iginv_t = assign950_e1284;

        let (assign960_e1288,) = {
    if (var_guard83 != 0.0) {
        (p.p105,)
    } else {
        (var_igovinv_t,)
    }
};
        var_igovinv_t = assign960_e1288;

        let (assign970_e1292,) = {
    if (var_guard83 != 0.0) {
        (p.p106,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign970_e1292;

        let (assign980_e1296,) = {
    if (var_guard83 != 0.0) {
        (p.p120,)
    } else {
        (var_fnovinv_t,)
    }
};
        var_fnovinv_t = assign980_e1296;

        let (assign990_e1300,) = {
    if (var_guard83 != 0.0) {
        (p.p121,)
    } else {
        (var_fnovinvd_t,)
    }
};
        var_fnovinvd_t = assign990_e1300;

        let (assign1000_e1304,) = {
    if (var_guard83 != 0.0) {
        (p.p107,)
    } else {
        (var_igovacc_t,)
    }
};
        var_igovacc_t = assign1000_e1304;

        let (assign1010_e1308,) = {
    if (var_guard83 != 0.0) {
        (p.p108,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign1010_e1308;

        let (assign1020_e1312,) = {
    if (var_guard83 != 0.0) {
        (p.p109,)
    } else {
        (var_stig_i,)
    }
};
        var_stig_i = assign1020_e1312;

        let (assign1030_e1316,) = {
    if (var_guard83 != 0.0) {
        (p.p123,)
    } else {
        (var_stigfn_i,)
    }
};
        var_stigfn_i = assign1030_e1316;

        let (assign1040_e1320,) = {
    if (var_guard83 != 0.0) {
        (p.p110,)
    } else {
        (var_gc2ch_i,)
    }
};
        var_gc2ch_i = assign1040_e1320;

        let (assign1050_e1324,) = {
    if (var_guard83 != 0.0) {
        (p.p111,)
    } else {
        (var_gc3ch_i,)
    }
};
        var_gc3ch_i = assign1050_e1324;

        let (assign1060_e1328,) = {
    if (var_guard83 != 0.0) {
        (p.p112,)
    } else {
        (var_gc2ovinv_i,)
    }
};
        var_gc2ovinv_i = assign1060_e1328;

        *var_alp1_i_slot = var_alp1_i;
        *var_alp_i_slot = var_alp_i;
        *var_alpb_i_slot = var_alpb_i;
        *var_ax_i_slot = var_ax_i;
        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfdl_i_slot = var_cfdl_i;
        *var_cfdlb_i_slot = var_cfdlb_i;
        *var_cs_t_slot = var_cs_t;
        *var_csbi_i_slot = var_csbi_i;
        *var_csfi_i_slot = var_csfi_i;
        *var_csthr_i_slot = var_csthr_i;
        *var_csthrb_i_slot = var_csthrb_i;
        *var_feta_i_slot = var_feta_i;
        *var_fnovinv_t_slot = var_fnovinv_t;
        *var_fnovinvd_t_slot = var_fnovinvd_t;
        *var_gc2ch_i_slot = var_gc2ch_i;
        *var_gc2ovinv_i_slot = var_gc2ovinv_i;
        *var_gc3ch_i_slot = var_gc3ch_i;
        *var_gco_i_slot = var_gco_i;
        *var_iginv_t_slot = var_iginv_t;
        *var_igovacc_t_slot = var_igovacc_t;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovinv_t_slot = var_igovinv_t;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_mue_t_slot = var_mue_t;
        *var_rs_t_slot = var_rs_t;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsig_i_slot = var_rsig_i;
        *var_stbet_i_slot = var_stbet_i;
        *var_stcf_i_slot = var_stcf_i;
        *var_stcf_i_dn4_slot = var_stcf_i_dn4;
        *var_stcf_i_dn6_slot = var_stcf_i_dn6;
        *var_stcf_i_dn7_slot = var_stcf_i_dn7;
        *var_stcf_i_dn8_slot = var_stcf_i_dn8;
        *var_stcf_i_dn9_slot = var_stcf_i_dn9;
        *var_stcs_i_slot = var_stcs_i;
        *var_stig_i_slot = var_stig_i;
        *var_stigfn_i_slot = var_stigfn_i;
        *var_stmue_i_slot = var_stmue_i;
        *var_strs_i_slot = var_strs_i;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_thecs_t_slot = var_thecs_t;
        *var_themu_t_slot = var_themu_t;
        *var_thersg_i_slot = var_thersg_i;
        *var_thesat1_i_slot = var_thesat1_i;
        *var_thesat2_i_slot = var_thesat2_i;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_vp_i_slot = var_vp_i;
        *var_vpg_i_slot = var_vpg_i;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcorb_i_slot = var_xcorb_i;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_guard83: f64,
        var_psce1_i: f64,
        var_psce2_i: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_a1_i_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_areaq_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_dn6_slot: &mut f64,
        var_betnedge_t_dn7_slot: &mut f64,
        var_betnedge_t_dn8_slot: &mut f64,
        var_betnedge_t_dn9_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_cf1edge_i_slot: &mut f64,
        var_cf1edge_i_dn4_slot: &mut f64,
        var_cf1edge_i_dn6_slot: &mut f64,
        var_cf1edge_i_dn7_slot: &mut f64,
        var_cf1edge_i_dn8_slot: &mut f64,
        var_cf1edge_i_dn9_slot: &mut f64,
        var_cf2edge_i_slot: &mut f64,
        var_cf2edge_i_dn4_slot: &mut f64,
        var_cf2edge_i_dn6_slot: &mut f64,
        var_cf2edge_i_dn7_slot: &mut f64,
        var_cf2edge_i_dn8_slot: &mut f64,
        var_cf2edge_i_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_dn4_slot: &mut f64,
        var_cgbov_i_dn6_slot: &mut f64,
        var_cgbov_i_dn7_slot: &mut f64,
        var_cgbov_i_dn8_slot: &mut f64,
        var_cgbov_i_dn9_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_cic1edge_i_slot: &mut f64,
        var_cic2edge_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_dgidl_i_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_gc2ovacc_i_slot: &mut f64,
        var_gc3ovacc_i_slot: &mut f64,
        var_gc3ovinv_i_slot: &mut f64,
        var_gcdov_i_slot: &mut f64,
        var_gcovinvfn_i_slot: &mut f64,
        var_gcvdov_i_slot: &mut f64,
        var_niginv_i_slot: &mut f64,
        var_nsdac_i_slot: &mut f64,
        var_psce1edge_i_slot: &mut f64,
        var_psce1edge_i_dn4_slot: &mut f64,
        var_psce1edge_i_dn6_slot: &mut f64,
        var_psce1edge_i_dn7_slot: &mut f64,
        var_psce1edge_i_dn8_slot: &mut f64,
        var_psce1edge_i_dn9_slot: &mut f64,
        var_psce2edge_i_slot: &mut f64,
        var_psce2edge_i_dn4_slot: &mut f64,
        var_psce2edge_i_dn6_slot: &mut f64,
        var_psce2edge_i_dn7_slot: &mut f64,
        var_psce2edge_i_dn8_slot: &mut f64,
        var_psce2edge_i_dn9_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfb1edge_t_slot: &mut f64,
        var_vfb1edge_t_dn4_slot: &mut f64,
        var_vfb1edge_t_dn6_slot: &mut f64,
        var_vfb1edge_t_dn7_slot: &mut f64,
        var_vfb1edge_t_dn8_slot: &mut f64,
        var_vfb1edge_t_dn9_slot: &mut f64,
        var_vfb2edge_t_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_areaq_i: f64 = *var_areaq_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_dn6: f64 = *var_betnedge_t_dn6_slot;
        let mut var_betnedge_t_dn7: f64 = *var_betnedge_t_dn7_slot;
        let mut var_betnedge_t_dn8: f64 = *var_betnedge_t_dn8_slot;
        let mut var_betnedge_t_dn9: f64 = *var_betnedge_t_dn9_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_cf1edge_i: f64 = *var_cf1edge_i_slot;
        let mut var_cf1edge_i_dn4: f64 = *var_cf1edge_i_dn4_slot;
        let mut var_cf1edge_i_dn6: f64 = *var_cf1edge_i_dn6_slot;
        let mut var_cf1edge_i_dn7: f64 = *var_cf1edge_i_dn7_slot;
        let mut var_cf1edge_i_dn8: f64 = *var_cf1edge_i_dn8_slot;
        let mut var_cf1edge_i_dn9: f64 = *var_cf1edge_i_dn9_slot;
        let mut var_cf2edge_i: f64 = *var_cf2edge_i_slot;
        let mut var_cf2edge_i_dn4: f64 = *var_cf2edge_i_dn4_slot;
        let mut var_cf2edge_i_dn6: f64 = *var_cf2edge_i_dn6_slot;
        let mut var_cf2edge_i_dn7: f64 = *var_cf2edge_i_dn7_slot;
        let mut var_cf2edge_i_dn8: f64 = *var_cf2edge_i_dn8_slot;
        let mut var_cf2edge_i_dn9: f64 = *var_cf2edge_i_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_dn4: f64 = *var_cgbov_i_dn4_slot;
        let mut var_cgbov_i_dn6: f64 = *var_cgbov_i_dn6_slot;
        let mut var_cgbov_i_dn7: f64 = *var_cgbov_i_dn7_slot;
        let mut var_cgbov_i_dn8: f64 = *var_cgbov_i_dn8_slot;
        let mut var_cgbov_i_dn9: f64 = *var_cgbov_i_dn9_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_cic1edge_i: f64 = *var_cic1edge_i_slot;
        let mut var_cic2edge_i: f64 = *var_cic2edge_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_dgidl_i: f64 = *var_dgidl_i_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_gc2ovacc_i: f64 = *var_gc2ovacc_i_slot;
        let mut var_gc3ovacc_i: f64 = *var_gc3ovacc_i_slot;
        let mut var_gc3ovinv_i: f64 = *var_gc3ovinv_i_slot;
        let mut var_gcdov_i: f64 = *var_gcdov_i_slot;
        let mut var_gcovinvfn_i: f64 = *var_gcovinvfn_i_slot;
        let mut var_gcvdov_i: f64 = *var_gcvdov_i_slot;
        let mut var_niginv_i: f64 = *var_niginv_i_slot;
        let mut var_nsdac_i: f64 = *var_nsdac_i_slot;
        let mut var_psce1edge_i: f64 = *var_psce1edge_i_slot;
        let mut var_psce1edge_i_dn4: f64 = *var_psce1edge_i_dn4_slot;
        let mut var_psce1edge_i_dn6: f64 = *var_psce1edge_i_dn6_slot;
        let mut var_psce1edge_i_dn7: f64 = *var_psce1edge_i_dn7_slot;
        let mut var_psce1edge_i_dn8: f64 = *var_psce1edge_i_dn8_slot;
        let mut var_psce1edge_i_dn9: f64 = *var_psce1edge_i_dn9_slot;
        let mut var_psce2edge_i: f64 = *var_psce2edge_i_slot;
        let mut var_psce2edge_i_dn4: f64 = *var_psce2edge_i_dn4_slot;
        let mut var_psce2edge_i_dn6: f64 = *var_psce2edge_i_dn6_slot;
        let mut var_psce2edge_i_dn7: f64 = *var_psce2edge_i_dn7_slot;
        let mut var_psce2edge_i_dn8: f64 = *var_psce2edge_i_dn8_slot;
        let mut var_psce2edge_i_dn9: f64 = *var_psce2edge_i_dn9_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfb1edge_t: f64 = *var_vfb1edge_t_slot;
        let mut var_vfb1edge_t_dn4: f64 = *var_vfb1edge_t_dn4_slot;
        let mut var_vfb1edge_t_dn6: f64 = *var_vfb1edge_t_dn6_slot;
        let mut var_vfb1edge_t_dn7: f64 = *var_vfb1edge_t_dn7_slot;
        let mut var_vfb1edge_t_dn8: f64 = *var_vfb1edge_t_dn8_slot;
        let mut var_vfb1edge_t_dn9: f64 = *var_vfb1edge_t_dn9_slot;
        let mut var_vfb2edge_t: f64 = *var_vfb2edge_t_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign1070_e1332,) = {
    if (var_guard83 != 0.0) {
        (p.p122,)
    } else {
        (var_gcovinvfn_i,)
    }
};
        var_gcovinvfn_i = assign1070_e1332;

        let (assign1080_e1336,) = {
    if (var_guard83 != 0.0) {
        (p.p113,)
    } else {
        (var_gc3ovinv_i,)
    }
};
        var_gc3ovinv_i = assign1080_e1336;

        let (assign1090_e1340,) = {
    if (var_guard83 != 0.0) {
        (p.p114,)
    } else {
        (var_gc2ovacc_i,)
    }
};
        var_gc2ovacc_i = assign1090_e1340;

        let (assign1100_e1344,) = {
    if (var_guard83 != 0.0) {
        (p.p115,)
    } else {
        (var_gc3ovacc_i,)
    }
};
        var_gc3ovacc_i = assign1100_e1344;

        let (assign1110_e1348,) = {
    if (var_guard83 != 0.0) {
        (p.p116,)
    } else {
        (var_gcdov_i,)
    }
};
        var_gcdov_i = assign1110_e1348;

        let (assign1120_e1352,) = {
    if (var_guard83 != 0.0) {
        (p.p117,)
    } else {
        (var_gcvdov_i,)
    }
};
        var_gcvdov_i = assign1120_e1352;

        let (assign1130_e1356,) = {
    if (var_guard83 != 0.0) {
        (p.p118,)
    } else {
        (var_chib_i,)
    }
};
        var_chib_i = assign1130_e1356;

        let (assign1140_e1360,) = {
    if (var_guard83 != 0.0) {
        (p.p119,)
    } else {
        (var_niginv_i,)
    }
};
        var_niginv_i = assign1140_e1360;

        let (assign1150_e1364, assign1150_e1364_d_n4, assign1150_e1364_d_n6, assign1150_e1364_d_n7, assign1150_e1364_d_n8, assign1150_e1364_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidl_i, var_agidl_i_dn4, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9,)
    }
};
        var_agidl_i = assign1150_e1364;
        var_agidl_i_dn4 = assign1150_e1364_d_n4;
        var_agidl_i_dn6 = assign1150_e1364_d_n6;
        var_agidl_i_dn7 = assign1150_e1364_d_n7;
        var_agidl_i_dn8 = assign1150_e1364_d_n8;
        var_agidl_i_dn9 = assign1150_e1364_d_n9;

        let (assign1160_e1368, assign1160_e1368_d_n4, assign1160_e1368_d_n6, assign1160_e1368_d_n7, assign1160_e1368_d_n8, assign1160_e1368_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidld_i, var_agidld_i_dn4, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9,)
    }
};
        var_agidld_i = assign1160_e1368;
        var_agidld_i_dn4 = assign1160_e1368_d_n4;
        var_agidld_i_dn6 = assign1160_e1368_d_n6;
        var_agidld_i_dn7 = assign1160_e1368_d_n7;
        var_agidld_i_dn8 = assign1160_e1368_d_n8;
        var_agidld_i_dn9 = assign1160_e1368_d_n9;

        let (assign1170_e1372,) = {
    if (var_guard83 != 0.0) {
        (p.p126,)
    } else {
        (var_bgidl_t,)
    }
};
        var_bgidl_t = assign1170_e1372;

        let (assign1180_e1376,) = {
    if (var_guard83 != 0.0) {
        (p.p127,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign1180_e1376;

        let (assign1190_e1380,) = {
    if (var_guard83 != 0.0) {
        (p.p128,)
    } else {
        (var_stbgidl_i,)
    }
};
        var_stbgidl_i = assign1190_e1380;

        let (assign1200_e1384,) = {
    if (var_guard83 != 0.0) {
        (p.p129,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign1200_e1384;

        let (assign1210_e1388,) = {
    if (var_guard83 != 0.0) {
        (p.p130,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign1210_e1388;

        let (assign1220_e1392,) = {
    if (var_guard83 != 0.0) {
        (p.p131,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign1220_e1392;

        let (assign1230_e1396,) = {
    if (var_guard83 != 0.0) {
        (p.p132,)
    } else {
        (var_dgidl_i,)
    }
};
        var_dgidl_i = assign1230_e1396;

        let (assign1240_e1400,) = {
    if (var_guard83 != 0.0) {
        (p.p133,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign1240_e1400;

        let (assign1250_e1404,) = {
    if (var_guard83 != 0.0) {
        (p.p147,)
    } else {
        (var_a1_i,)
    }
};
        var_a1_i = assign1250_e1404;

        let (assign1260_e1408,) = {
    if (var_guard83 != 0.0) {
        (p.p148,)
    } else {
        (var_a2_t,)
    }
};
        var_a2_t = assign1260_e1408;

        let (assign1270_e1412,) = {
    if (var_guard83 != 0.0) {
        (p.p149,)
    } else {
        (var_sta2_i,)
    }
};
        var_sta2_i = assign1270_e1412;

        let (assign1280_e1416,) = {
    if (var_guard83 != 0.0) {
        (p.p150,)
    } else {
        (var_a3_i,)
    }
};
        var_a3_i = assign1280_e1416;

        let (assign1290_e1420,) = {
    if (var_guard83 != 0.0) {
        (p.p134,)
    } else {
        (var_ctedge_i,)
    }
};
        var_ctedge_i = assign1290_e1420;

        let (assign1300_e1424, assign1300_e1424_d_n4, assign1300_e1424_d_n6, assign1300_e1424_d_n7, assign1300_e1424_d_n8, assign1300_e1424_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb1edge_t, var_vfb1edge_t_dn4, var_vfb1edge_t_dn6, var_vfb1edge_t_dn7, var_vfb1edge_t_dn8, var_vfb1edge_t_dn9,)
    }
};
        var_vfb1edge_t = assign1300_e1424;
        var_vfb1edge_t_dn4 = assign1300_e1424_d_n4;
        var_vfb1edge_t_dn6 = assign1300_e1424_d_n6;
        var_vfb1edge_t_dn7 = assign1300_e1424_d_n7;
        var_vfb1edge_t_dn8 = assign1300_e1424_d_n8;
        var_vfb1edge_t_dn9 = assign1300_e1424_d_n9;

        let (assign1310_e1428,) = {
    if (var_guard83 != 0.0) {
        (p.p136,)
    } else {
        (var_vfb2edge_t,)
    }
};
        var_vfb2edge_t = assign1310_e1428;

        let (assign1320_e1432,) = {
    if (var_guard83 != 0.0) {
        (p.p137,)
    } else {
        (var_stvfbedge_i,)
    }
};
        var_stvfbedge_i = assign1320_e1432;

        let (assign1330_e1436,) = {
    if (var_guard83 != 0.0) {
        (p.p138,)
    } else {
        (var_cic1edge_i,)
    }
};
        var_cic1edge_i = assign1330_e1436;

        let (assign1340_e1440,) = {
    if (var_guard83 != 0.0) {
        (p.p139,)
    } else {
        (var_cic2edge_i,)
    }
};
        var_cic2edge_i = assign1340_e1440;

        let (assign1350_e1444, assign1350_e1444_d_n4, assign1350_e1444_d_n6, assign1350_e1444_d_n7, assign1350_e1444_d_n8, assign1350_e1444_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_psce1edge_i, var_psce1edge_i_dn4, var_psce1edge_i_dn6, var_psce1edge_i_dn7, var_psce1edge_i_dn8, var_psce1edge_i_dn9,)
    }
};
        var_psce1edge_i = assign1350_e1444;
        var_psce1edge_i_dn4 = assign1350_e1444_d_n4;
        var_psce1edge_i_dn6 = assign1350_e1444_d_n6;
        var_psce1edge_i_dn7 = assign1350_e1444_d_n7;
        var_psce1edge_i_dn8 = assign1350_e1444_d_n8;
        var_psce1edge_i_dn9 = assign1350_e1444_d_n9;

        let (assign1360_e1454, assign1360_e1454_d_n4, assign1360_e1454_d_n6, assign1360_e1454_d_n7, assign1360_e1454_d_n8, assign1360_e1454_d_n9,) = {
    if (var_guard83 != 0.0) {
        let assign1360_e1448: f64 = (p.p141 * var_psce1edge_i);
        let assign1360_e1450: f64 = (assign1360_e1448 * var_tox2_i);
        let assign1360_e1452: f64 = (assign1360_e1450 / var_tox1_i);
        (assign1360_e1452, (((p.p141 * var_psce1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_psce2edge_i, var_psce2edge_i_dn4, var_psce2edge_i_dn6, var_psce2edge_i_dn7, var_psce2edge_i_dn8, var_psce2edge_i_dn9,)
    }
};
        var_psce2edge_i = assign1360_e1454;
        var_psce2edge_i_dn4 = assign1360_e1454_d_n4;
        var_psce2edge_i_dn6 = assign1360_e1454_d_n6;
        var_psce2edge_i_dn7 = assign1360_e1454_d_n7;
        var_psce2edge_i_dn8 = assign1360_e1454_d_n8;
        var_psce2edge_i_dn9 = assign1360_e1454_d_n9;

        let (assign1370_e1458, assign1370_e1458_d_n4, assign1370_e1458_d_n6, assign1370_e1458_d_n7, assign1370_e1458_d_n8, assign1370_e1458_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cf1edge_i, var_cf1edge_i_dn4, var_cf1edge_i_dn6, var_cf1edge_i_dn7, var_cf1edge_i_dn8, var_cf1edge_i_dn9,)
    }
};
        var_cf1edge_i = assign1370_e1458;
        var_cf1edge_i_dn4 = assign1370_e1458_d_n4;
        var_cf1edge_i_dn6 = assign1370_e1458_d_n6;
        var_cf1edge_i_dn7 = assign1370_e1458_d_n7;
        var_cf1edge_i_dn8 = assign1370_e1458_d_n8;
        var_cf1edge_i_dn9 = assign1370_e1458_d_n9;

        let (assign1380_e1468, assign1380_e1468_d_n4, assign1380_e1468_d_n6, assign1380_e1468_d_n7, assign1380_e1468_d_n8, assign1380_e1468_d_n9,) = {
    if (var_guard83 != 0.0) {
        let assign1380_e1462: f64 = (p.p143 * var_cf1edge_i);
        let assign1380_e1464: f64 = (assign1380_e1462 * var_tox2_i);
        let assign1380_e1466: f64 = (assign1380_e1464 / var_tox1_i);
        (assign1380_e1466, (((p.p143 * var_cf1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2edge_i, var_cf2edge_i_dn4, var_cf2edge_i_dn6, var_cf2edge_i_dn7, var_cf2edge_i_dn8, var_cf2edge_i_dn9,)
    }
};
        var_cf2edge_i = assign1380_e1468;
        var_cf2edge_i_dn4 = assign1380_e1468_d_n4;
        var_cf2edge_i_dn6 = assign1380_e1468_d_n6;
        var_cf2edge_i_dn7 = assign1380_e1468_d_n7;
        var_cf2edge_i_dn8 = assign1380_e1468_d_n8;
        var_cf2edge_i_dn9 = assign1380_e1468_d_n9;

        let (assign1390_e1472,) = {
    if (var_guard83 != 0.0) {
        (p.p144,)
    } else {
        (var_cfdedge_i,)
    }
};
        var_cfdedge_i = assign1390_e1472;

        let (assign1400_e1476, assign1400_e1476_d_n4, assign1400_e1476_d_n6, assign1400_e1476_d_n7, assign1400_e1476_d_n8, assign1400_e1476_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4, var_betnedge_t_dn6, var_betnedge_t_dn7, var_betnedge_t_dn8, var_betnedge_t_dn9,)
    }
};
        var_betnedge_t = assign1400_e1476;
        var_betnedge_t_dn4 = assign1400_e1476_d_n4;
        var_betnedge_t_dn6 = assign1400_e1476_d_n6;
        var_betnedge_t_dn7 = assign1400_e1476_d_n7;
        var_betnedge_t_dn8 = assign1400_e1476_d_n8;
        var_betnedge_t_dn9 = assign1400_e1476_d_n9;

        let (assign1410_e1480,) = {
    if (var_guard83 != 0.0) {
        (p.p146,)
    } else {
        (var_stbetedge_i,)
    }
};
        var_stbetedge_i = assign1410_e1480;

        let (assign1420_e1484,) = {
    if (var_guard83 != 0.0) {
        (p.p151,)
    } else {
        (var_areaq_i,)
    }
};
        var_areaq_i = assign1420_e1484;

        let (assign1430_e1488, assign1430_e1488_d_n4, assign1430_e1488_d_n6, assign1430_e1488_d_n7, assign1430_e1488_d_n8, assign1430_e1488_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgbov_i, var_cgbov_i_dn4, var_cgbov_i_dn6, var_cgbov_i_dn7, var_cgbov_i_dn8, var_cgbov_i_dn9,)
    }
};
        var_cgbov_i = assign1430_e1488;
        var_cgbov_i_dn4 = assign1430_e1488_d_n4;
        var_cgbov_i_dn6 = assign1430_e1488_d_n6;
        var_cgbov_i_dn7 = assign1430_e1488_d_n7;
        var_cgbov_i_dn8 = assign1430_e1488_d_n8;
        var_cgbov_i_dn9 = assign1430_e1488_d_n9;

        let (assign1440_e1494,) = {
    if (var_guard83 != 0.0) {
        let assign1440_e1492: f64 = (p.p153 * 1000000.0);
        (assign1440_e1492,)
    } else {
        (var_nsdac_i,)
    }
};
        var_nsdac_i = assign1440_e1494;

        let (assign1450_e1498,) = {
    if (var_guard83 != 0.0) {
        (p.p154,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign1450_e1498;

        let (assign1460_e1502,) = {
    if (var_guard83 != 0.0) {
        (p.p155,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign1460_e1502;

        let (assign1470_e1506, assign1470_e1506_d_n4, assign1470_e1506_d_n6, assign1470_e1506_d_n7, assign1470_e1506_d_n8, assign1470_e1506_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1470_e1506;
        var_vfbac1_t_dn4 = assign1470_e1506_d_n4;
        var_vfbac1_t_dn6 = assign1470_e1506_d_n6;
        var_vfbac1_t_dn7 = assign1470_e1506_d_n7;
        var_vfbac1_t_dn8 = assign1470_e1506_d_n8;
        var_vfbac1_t_dn9 = assign1470_e1506_d_n9;

        let (assign1480_e1510, assign1480_e1510_d_n4, assign1480_e1510_d_n6, assign1480_e1510_d_n7, assign1480_e1510_d_n8, assign1480_e1510_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1480_e1510;
        var_vfbac2_t_dn4 = assign1480_e1510_d_n4;
        var_vfbac2_t_dn6 = assign1480_e1510_d_n6;
        var_vfbac2_t_dn7 = assign1480_e1510_d_n7;
        var_vfbac2_t_dn8 = assign1480_e1510_d_n8;
        var_vfbac2_t_dn9 = assign1480_e1510_d_n9;

        let (assign1490_e1514,) = {
    if (var_guard83 != 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1490_e1514;

        let (assign1500_e1518,) = {
    if (var_guard83 != 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1500_e1518;

        let (assign1510_e1522, assign1510_e1522_d_n4, assign1510_e1522_d_n6, assign1510_e1522_d_n7, assign1510_e1522_d_n8, assign1510_e1522_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1510_e1522;
        var_cfac1_t_dn4 = assign1510_e1522_d_n4;
        var_cfac1_t_dn6 = assign1510_e1522_d_n6;
        var_cfac1_t_dn7 = assign1510_e1522_d_n7;
        var_cfac1_t_dn8 = assign1510_e1522_d_n8;
        var_cfac1_t_dn9 = assign1510_e1522_d_n9;

        let (assign1520_e1526, assign1520_e1526_d_n4, assign1520_e1526_d_n6, assign1520_e1526_d_n7, assign1520_e1526_d_n8, assign1520_e1526_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign1520_e1526;
        var_cfac2_t_dn4 = assign1520_e1526_d_n4;
        var_cfac2_t_dn6 = assign1520_e1526_d_n6;
        var_cfac2_t_dn7 = assign1520_e1526_d_n7;
        var_cfac2_t_dn8 = assign1520_e1526_d_n8;
        var_cfac2_t_dn9 = assign1520_e1526_d_n9;

        let (assign1530_e1530, assign1530_e1530_d_n4, assign1530_e1530_d_n6, assign1530_e1530_d_n7, assign1530_e1530_d_n8, assign1530_e1530_d_n9,) = {
    if (var_guard83 != 0.0) {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1530_e1530;
        var_thesatac_t_dn4 = assign1530_e1530_d_n4;
        var_thesatac_t_dn6 = assign1530_e1530_d_n6;
        var_thesatac_t_dn7 = assign1530_e1530_d_n7;
        var_thesatac_t_dn8 = assign1530_e1530_d_n8;
        var_thesatac_t_dn9 = assign1530_e1530_d_n9;

        let (assign1540_e1534,) = {
    if (var_guard83 != 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1540_e1534;

        *var_a1_i_slot = var_a1_i;
        *var_a2_t_slot = var_a2_t;
        *var_a3_i_slot = var_a3_i;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_areaq_i_slot = var_areaq_i;
        *var_axac_i_slot = var_axac_i;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_dn6_slot = var_betnedge_t_dn6;
        *var_betnedge_t_dn7_slot = var_betnedge_t_dn7;
        *var_betnedge_t_dn8_slot = var_betnedge_t_dn8;
        *var_betnedge_t_dn9_slot = var_betnedge_t_dn9;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_cf1edge_i_slot = var_cf1edge_i;
        *var_cf1edge_i_dn4_slot = var_cf1edge_i_dn4;
        *var_cf1edge_i_dn6_slot = var_cf1edge_i_dn6;
        *var_cf1edge_i_dn7_slot = var_cf1edge_i_dn7;
        *var_cf1edge_i_dn8_slot = var_cf1edge_i_dn8;
        *var_cf1edge_i_dn9_slot = var_cf1edge_i_dn9;
        *var_cf2edge_i_slot = var_cf2edge_i;
        *var_cf2edge_i_dn4_slot = var_cf2edge_i_dn4;
        *var_cf2edge_i_dn6_slot = var_cf2edge_i_dn6;
        *var_cf2edge_i_dn7_slot = var_cf2edge_i_dn7;
        *var_cf2edge_i_dn8_slot = var_cf2edge_i_dn8;
        *var_cf2edge_i_dn9_slot = var_cf2edge_i_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_dn4_slot = var_cgbov_i_dn4;
        *var_cgbov_i_dn6_slot = var_cgbov_i_dn6;
        *var_cgbov_i_dn7_slot = var_cgbov_i_dn7;
        *var_cgbov_i_dn8_slot = var_cgbov_i_dn8;
        *var_cgbov_i_dn9_slot = var_cgbov_i_dn9;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_chib_i_slot = var_chib_i;
        *var_cic1edge_i_slot = var_cic1edge_i;
        *var_cic2edge_i_slot = var_cic2edge_i;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_dgidl_i_slot = var_dgidl_i;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_fif_i_slot = var_fif_i;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_gc2ovacc_i_slot = var_gc2ovacc_i;
        *var_gc3ovacc_i_slot = var_gc3ovacc_i;
        *var_gc3ovinv_i_slot = var_gc3ovinv_i;
        *var_gcdov_i_slot = var_gcdov_i;
        *var_gcovinvfn_i_slot = var_gcovinvfn_i;
        *var_gcvdov_i_slot = var_gcvdov_i;
        *var_niginv_i_slot = var_niginv_i;
        *var_nsdac_i_slot = var_nsdac_i;
        *var_psce1edge_i_slot = var_psce1edge_i;
        *var_psce1edge_i_dn4_slot = var_psce1edge_i_dn4;
        *var_psce1edge_i_dn6_slot = var_psce1edge_i_dn6;
        *var_psce1edge_i_dn7_slot = var_psce1edge_i_dn7;
        *var_psce1edge_i_dn8_slot = var_psce1edge_i_dn8;
        *var_psce1edge_i_dn9_slot = var_psce1edge_i_dn9;
        *var_psce2edge_i_slot = var_psce2edge_i;
        *var_psce2edge_i_dn4_slot = var_psce2edge_i_dn4;
        *var_psce2edge_i_dn6_slot = var_psce2edge_i_dn6;
        *var_psce2edge_i_dn7_slot = var_psce2edge_i_dn7;
        *var_psce2edge_i_dn8_slot = var_psce2edge_i_dn8;
        *var_psce2edge_i_dn9_slot = var_psce2edge_i_dn9;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfb1edge_t_slot = var_vfb1edge_t;
        *var_vfb1edge_t_dn4_slot = var_vfb1edge_t_dn4;
        *var_vfb1edge_t_dn6_slot = var_vfb1edge_t_dn6;
        *var_vfb1edge_t_dn7_slot = var_vfb1edge_t_dn7;
        *var_vfb1edge_t_dn8_slot = var_vfb1edge_t_dn8;
        *var_vfb1edge_t_dn9_slot = var_vfb1edge_t_dn9;
        *var_vfb2edge_t_slot = var_vfb2edge_t;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_guard83: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_alpac_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covdl_i_slot: &mut f64,
        var_covdlb_i_slot: &mut f64,
        var_csd_i_slot: &mut f64,
        var_csdbp_i_slot: &mut f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_dvfbov_i_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fntexc_i_slot: &mut f64,
        var_fracinv_i_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_kdiff_i_slot: &mut f64,
        var_kdiff_i_dn4_slot: &mut f64,
        var_kdiff_i_dn6_slot: &mut f64,
        var_kdiff_i_dn7_slot: &mut f64,
        var_kdiff_i_dn8_slot: &mut f64,
        var_kdiff_i_dn9_slot: &mut f64,
        var_kdrift_i_slot: &mut f64,
        var_kdrift_i_dn4_slot: &mut f64,
        var_kdrift_i_dn6_slot: &mut f64,
        var_kdrift_i_dn7_slot: &mut f64,
        var_kdrift_i_dn8_slot: &mut f64,
        var_kdrift_i_dn9_slot: &mut f64,
        var_kfracinv_i_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_w_i_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covdl_i: f64 = *var_covdl_i_slot;
        let mut var_covdlb_i: f64 = *var_covdlb_i_slot;
        let mut var_csd_i: f64 = *var_csd_i_slot;
        let mut var_csdbp_i: f64 = *var_csdbp_i_slot;
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_dvfbov_i: f64 = *var_dvfbov_i_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fntexc_i: f64 = *var_fntexc_i_slot;
        let mut var_fracinv_i: f64 = *var_fracinv_i_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_kdiff_i: f64 = *var_kdiff_i_slot;
        let mut var_kdiff_i_dn4: f64 = *var_kdiff_i_dn4_slot;
        let mut var_kdiff_i_dn6: f64 = *var_kdiff_i_dn6_slot;
        let mut var_kdiff_i_dn7: f64 = *var_kdiff_i_dn7_slot;
        let mut var_kdiff_i_dn8: f64 = *var_kdiff_i_dn8_slot;
        let mut var_kdiff_i_dn9: f64 = *var_kdiff_i_dn9_slot;
        let mut var_kdrift_i: f64 = *var_kdrift_i_slot;
        let mut var_kdrift_i_dn4: f64 = *var_kdrift_i_dn4_slot;
        let mut var_kdrift_i_dn6: f64 = *var_kdrift_i_dn6_slot;
        let mut var_kdrift_i_dn7: f64 = *var_kdrift_i_dn7_slot;
        let mut var_kdrift_i_dn8: f64 = *var_kdrift_i_dn8_slot;
        let mut var_kdrift_i_dn9: f64 = *var_kdrift_i_dn9_slot;
        let mut var_kfracinv_i: f64 = *var_kfracinv_i_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_w_i: f64 = *var_w_i_slot;

        let (assign1550_e1538,) = {
    if (var_guard83 != 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1550_e1538;

        let assign1560_e1541: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard86 = assign1560_e1541;

        let (assign1570_e1547, assign1570_e1547_d_n4, assign1570_e1547_d_n6, assign1570_e1547_d_n7, assign1570_e1547_d_n8, assign1570_e1547_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1570_e1547;
        var_vfbac1_t_dn4 = assign1570_e1547_d_n4;
        var_vfbac1_t_dn6 = assign1570_e1547_d_n6;
        var_vfbac1_t_dn7 = assign1570_e1547_d_n7;
        var_vfbac1_t_dn8 = assign1570_e1547_d_n8;
        var_vfbac1_t_dn9 = assign1570_e1547_d_n9;

        let assign1580_e1549: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1551: f64 = if assign1580_e1549 == 1.0 { 1.0 } else { 0.0 };
        var_guard87 = assign1580_e1551;

        let (assign1590_e1559, assign1590_e1559_d_n4, assign1590_e1559_d_n6, assign1590_e1559_d_n7, assign1590_e1559_d_n8, assign1590_e1559_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) {
        (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign1590_e1559;
        var_vfbac1_t_dn4 = assign1590_e1559_d_n4;
        var_vfbac1_t_dn6 = assign1590_e1559_d_n6;
        var_vfbac1_t_dn7 = assign1590_e1559_d_n7;
        var_vfbac1_t_dn8 = assign1590_e1559_d_n8;
        var_vfbac1_t_dn9 = assign1590_e1559_d_n9;

        let (assign1600_e1565, assign1600_e1565_d_n4, assign1600_e1565_d_n6, assign1600_e1565_d_n7, assign1600_e1565_d_n8, assign1600_e1565_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1600_e1565;
        var_vfbac2_t_dn4 = assign1600_e1565_d_n4;
        var_vfbac2_t_dn6 = assign1600_e1565_d_n6;
        var_vfbac2_t_dn7 = assign1600_e1565_d_n7;
        var_vfbac2_t_dn8 = assign1600_e1565_d_n8;
        var_vfbac2_t_dn9 = assign1600_e1565_d_n9;

        let assign1610_e1567: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1569: f64 = if assign1610_e1567 == 1.0 { 1.0 } else { 0.0 };
        var_guard88 = assign1610_e1569;

        let (assign1620_e1577, assign1620_e1577_d_n4, assign1620_e1577_d_n6, assign1620_e1577_d_n7, assign1620_e1577_d_n8, assign1620_e1577_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard88 != 0.0)) {
        (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign1620_e1577;
        var_vfbac2_t_dn4 = assign1620_e1577_d_n4;
        var_vfbac2_t_dn6 = assign1620_e1577_d_n6;
        var_vfbac2_t_dn7 = assign1620_e1577_d_n7;
        var_vfbac2_t_dn8 = assign1620_e1577_d_n8;
        var_vfbac2_t_dn9 = assign1620_e1577_d_n9;

        let (assign1630_e1583,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p57,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1630_e1583;

        let assign1640_e1585: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1587: f64 = if assign1640_e1585 == 1.0 { 1.0 } else { 0.0 };
        var_guard89 = assign1640_e1587;

        let (assign1650_e1595,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard89 != 0.0)) {
        (p.p158,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1650_e1595;

        let (assign1660_e1607,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1660_e1601: f64 = (p.p58 * var_psceac1_i);
        let assign1660_e1603: f64 = (assign1660_e1601 * var_tox2_i);
        let assign1660_e1605: f64 = (assign1660_e1603 / var_tox1_i);
        (assign1660_e1605,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1660_e1607;

        let (assign1670_e1613, assign1670_e1613_d_n4, assign1670_e1613_d_n6, assign1670_e1613_d_n7, assign1670_e1613_d_n8, assign1670_e1613_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1670_e1613;
        var_cfac1_t_dn4 = assign1670_e1613_d_n4;
        var_cfac1_t_dn6 = assign1670_e1613_d_n6;
        var_cfac1_t_dn7 = assign1670_e1613_d_n7;
        var_cfac1_t_dn8 = assign1670_e1613_d_n8;
        var_cfac1_t_dn9 = assign1670_e1613_d_n9;

        let assign1680_e1615: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1617: f64 = if assign1680_e1615 == 1.0 { 1.0 } else { 0.0 };
        var_guard90 = assign1680_e1617;

        let (assign1690_e1625, assign1690_e1625_d_n4, assign1690_e1625_d_n6, assign1690_e1625_d_n7, assign1690_e1625_d_n8, assign1690_e1625_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard90 != 0.0)) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign1690_e1625;
        var_cfac1_t_dn4 = assign1690_e1625_d_n4;
        var_cfac1_t_dn6 = assign1690_e1625_d_n6;
        var_cfac1_t_dn7 = assign1690_e1625_d_n7;
        var_cfac1_t_dn8 = assign1690_e1625_d_n8;
        var_cfac1_t_dn9 = assign1690_e1625_d_n9;

        let (assign1700_e1637, assign1700_e1637_d_n4, assign1700_e1637_d_n6, assign1700_e1637_d_n7, assign1700_e1637_d_n8, assign1700_e1637_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1700_e1631: f64 = (p.p63 * var_cfac1_t);
        let assign1700_e1633: f64 = (assign1700_e1631 * var_tox2_i);
        let assign1700_e1635: f64 = (assign1700_e1633 / var_tox1_i);
        (assign1700_e1635, (((p.p63 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign1700_e1637;
        var_cfac2_t_dn4 = assign1700_e1637_d_n4;
        var_cfac2_t_dn6 = assign1700_e1637_d_n6;
        var_cfac2_t_dn7 = assign1700_e1637_d_n7;
        var_cfac2_t_dn8 = assign1700_e1637_d_n8;
        var_cfac2_t_dn9 = assign1700_e1637_d_n9;

        let (assign1710_e1643, assign1710_e1643_d_n4, assign1710_e1643_d_n6, assign1710_e1643_d_n7, assign1710_e1643_d_n8, assign1710_e1643_d_n9,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1710_e1643;
        var_thesatac_t_dn4 = assign1710_e1643_d_n4;
        var_thesatac_t_dn6 = assign1710_e1643_d_n6;
        var_thesatac_t_dn7 = assign1710_e1643_d_n7;
        var_thesatac_t_dn8 = assign1710_e1643_d_n8;
        var_thesatac_t_dn9 = assign1710_e1643_d_n9;

        let assign1720_e1645: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1647: f64 = if assign1720_e1645 == 1.0 { 1.0 } else { 0.0 };
        var_guard91 = assign1720_e1647;

        let (assign1730_e1655, assign1730_e1655_d_n4, assign1730_e1655_d_n6, assign1730_e1655_d_n7, assign1730_e1655_d_n8, assign1730_e1655_d_n9,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard91 != 0.0)) {
        (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign1730_e1655;
        var_thesatac_t_dn4 = assign1730_e1655_d_n4;
        var_thesatac_t_dn6 = assign1730_e1655_d_n6;
        var_thesatac_t_dn7 = assign1730_e1655_d_n7;
        var_thesatac_t_dn8 = assign1730_e1655_d_n8;
        var_thesatac_t_dn9 = assign1730_e1655_d_n9;

        let (assign1740_e1661,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p97,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1740_e1661;

        let assign1750_e1663: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1665: f64 = if assign1750_e1663 == 1.0 { 1.0 } else { 0.0 };
        var_guard92 = assign1750_e1665;

        let (assign1760_e1673,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard92 != 0.0)) {
        (p.p161,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1760_e1673;

        let (assign1770_e1679,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p98,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1770_e1679;

        let assign1780_e1681: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1683: f64 = if assign1780_e1681 == 1.0 { 1.0 } else { 0.0 };
        var_guard93 = assign1780_e1683;

        let (assign1790_e1691,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard93 != 0.0)) {
        (p.p162,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1790_e1691;

        let (assign1800_e1695, assign1800_e1695_d_n4, assign1800_e1695_d_n6, assign1800_e1695_d_n7, assign1800_e1695_d_n8, assign1800_e1695_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    }
};
        var_cov_i = assign1800_e1695;
        var_cov_i_dn4 = assign1800_e1695_d_n4;
        var_cov_i_dn6 = assign1800_e1695_d_n6;
        var_cov_i_dn7 = assign1800_e1695_d_n7;
        var_cov_i_dn8 = assign1800_e1695_d_n8;
        var_cov_i_dn9 = assign1800_e1695_d_n9;

        let (assign1810_e1699, assign1810_e1699_d_n4, assign1810_e1699_d_n6, assign1810_e1699_d_n7, assign1810_e1699_d_n8, assign1810_e1699_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign1810_e1699;
        var_covd_i_dn4 = assign1810_e1699_d_n4;
        var_covd_i_dn6 = assign1810_e1699_d_n6;
        var_covd_i_dn7 = assign1810_e1699_d_n7;
        var_covd_i_dn8 = assign1810_e1699_d_n8;
        var_covd_i_dn9 = assign1810_e1699_d_n9;

        let (assign1820_e1703,) = {
    if (var_guard83 != 0.0) {
        (p.p165,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign1820_e1703;

        let (assign1830_e1707,) = {
    if (var_guard83 != 0.0) {
        (p.p166,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign1830_e1707;

        let (assign1840_e1711,) = {
    if (var_guard83 != 0.0) {
        (p.p167,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign1840_e1711;

        let (assign1850_e1715, assign1850_e1715_d_n4, assign1850_e1715_d_n6, assign1850_e1715_d_n7, assign1850_e1715_d_n8, assign1850_e1715_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    }
};
        var_cfr_i = assign1850_e1715;
        var_cfr_i_dn4 = assign1850_e1715_d_n4;
        var_cfr_i_dn6 = assign1850_e1715_d_n6;
        var_cfr_i_dn7 = assign1850_e1715_d_n7;
        var_cfr_i_dn8 = assign1850_e1715_d_n8;
        var_cfr_i_dn9 = assign1850_e1715_d_n9;

        let (assign1860_e1719, assign1860_e1719_d_n4, assign1860_e1719_d_n6, assign1860_e1719_d_n7, assign1860_e1719_d_n8, assign1860_e1719_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign1860_e1719;
        var_cfrd_i_dn4 = assign1860_e1719_d_n4;
        var_cfrd_i_dn6 = assign1860_e1719_d_n6;
        var_cfrd_i_dn7 = assign1860_e1719_d_n7;
        var_cfrd_i_dn8 = assign1860_e1719_d_n8;
        var_cfrd_i_dn9 = assign1860_e1719_d_n9;

        let (assign1870_e1723,) = {
    if (var_guard83 != 0.0) {
        (p.p170,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign1870_e1723;

        let (assign1880_e1727,) = {
    if (var_guard83 != 0.0) {
        (p.p171,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign1880_e1727;

        let (assign1890_e1731, assign1890_e1731_d_n4, assign1890_e1731_d_n6, assign1890_e1731_d_n7, assign1890_e1731_d_n8, assign1890_e1731_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rth_t, var_rth_t_dn4, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9,)
    }
};
        var_rth_t = assign1890_e1731;
        var_rth_t_dn4 = assign1890_e1731_d_n4;
        var_rth_t_dn6 = assign1890_e1731_d_n6;
        var_rth_t_dn7 = assign1890_e1731_d_n7;
        var_rth_t_dn8 = assign1890_e1731_d_n8;
        var_rth_t_dn9 = assign1890_e1731_d_n9;

        let (assign1900_e1735,) = {
    if (var_guard83 != 0.0) {
        (p.p173,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign1900_e1735;

        let (assign1910_e1739, assign1910_e1739_d_n4, assign1910_e1739_d_n6, assign1910_e1739_d_n7, assign1910_e1739_d_n8, assign1910_e1739_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cth_i, var_cth_i_dn4, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9,)
    }
};
        var_cth_i = assign1910_e1739;
        var_cth_i_dn4 = assign1910_e1739_d_n4;
        var_cth_i_dn6 = assign1910_e1739_d_n6;
        var_cth_i_dn7 = assign1910_e1739_d_n7;
        var_cth_i_dn8 = assign1910_e1739_d_n8;
        var_cth_i_dn9 = assign1910_e1739_d_n9;

        let (assign1920_e1743,) = {
    if (var_guard83 != 0.0) {
        (p.p175,)
    } else {
        (var_fnt_i,)
    }
};
        var_fnt_i = assign1920_e1743;

        let (assign1930_e1747,) = {
    if (var_guard83 != 0.0) {
        (p.p176,)
    } else {
        (var_fntexc_i,)
    }
};
        var_fntexc_i = assign1930_e1747;

        let (assign1940_e1751,) = {
    if (var_guard83 != 0.0) {
        (p.p177,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign1940_e1751;

        let (assign1950_e1755,) = {
    if (var_guard83 != 0.0) {
        (p.p178,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign1950_e1755;

        let (assign1960_e1759,) = {
    if (var_guard83 != 0.0) {
        (p.p179,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign1960_e1759;

        let (assign1970_e1763,) = {
    if (var_guard83 != 0.0) {
        (p.p180,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign1970_e1763;

        let (assign1980_e1767,) = {
    if (var_guard83 != 0.0) {
        (p.p181,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign1980_e1767;

        let (assign2000_e1775, assign2000_e1775_d_n4, assign2000_e1775_d_n6, assign2000_e1775_d_n7, assign2000_e1775_d_n8, assign2000_e1775_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p183, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kdrift_i, var_kdrift_i_dn4, var_kdrift_i_dn6, var_kdrift_i_dn7, var_kdrift_i_dn8, var_kdrift_i_dn9,)
    }
};
        var_kdrift_i = assign2000_e1775;
        var_kdrift_i_dn4 = assign2000_e1775_d_n4;
        var_kdrift_i_dn6 = assign2000_e1775_d_n6;
        var_kdrift_i_dn7 = assign2000_e1775_d_n7;
        var_kdrift_i_dn8 = assign2000_e1775_d_n8;
        var_kdrift_i_dn9 = assign2000_e1775_d_n9;

        let (assign2010_e1779, assign2010_e1779_d_n4, assign2010_e1779_d_n6, assign2010_e1779_d_n7, assign2010_e1779_d_n8, assign2010_e1779_d_n9,) = {
    if (var_guard83 != 0.0) {
        (p.p184, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kdiff_i, var_kdiff_i_dn4, var_kdiff_i_dn6, var_kdiff_i_dn7, var_kdiff_i_dn8, var_kdiff_i_dn9,)
    }
};
        var_kdiff_i = assign2010_e1779;
        var_kdiff_i_dn4 = assign2010_e1779_d_n4;
        var_kdiff_i_dn6 = assign2010_e1779_d_n6;
        var_kdiff_i_dn7 = assign2010_e1779_d_n7;
        var_kdiff_i_dn8 = assign2010_e1779_d_n8;
        var_kdiff_i_dn9 = assign2010_e1779_d_n9;

        let (assign2020_e1783,) = {
    if (var_guard83 != 0.0) {
        (p.p185,)
    } else {
        (var_fracinv_i,)
    }
};
        var_fracinv_i = assign2020_e1783;

        let (assign2030_e1787,) = {
    if (var_guard83 != 0.0) {
        (p.p186,)
    } else {
        (var_kfracinv_i,)
    }
};
        var_kfracinv_i = assign2030_e1787;

        let (assign2080_e1810,) = {
    if (var_guard83 == 0.0) {
        let assign2080_e1808: f64 = (1.0 / p.p29);
        (assign2080_e1808,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign2080_e1810;

        let (assign2090_e1819,) = {
    if (var_guard83 == 0.0) {
        let assign2090_e1815: f64 = (p.p21 * var_invnf);
        let assign2090_e1817: f64 = (assign2090_e1815).max(1e-9);
        (assign2090_e1817,)
    } else {
        (var_w_i,)
    }
};
        var_w_i = assign2090_e1819;

        *var_alpac_i_slot = var_alpac_i;
        *var_axac_i_slot = var_axac_i;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covdl_i_slot = var_covdl_i;
        *var_covdlb_i_slot = var_covdlb_i;
        *var_csd_i_slot = var_csd_i;
        *var_csdbp_i_slot = var_csdbp_i;
        *var_cth_i_slot = var_cth_i;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_dvfbov_i_slot = var_dvfbov_i;
        *var_fnt_i_slot = var_fnt_i;
        *var_fntexc_i_slot = var_fntexc_i;
        *var_fracinv_i_slot = var_fracinv_i;
        *var_guard86_slot = var_guard86;
        *var_guard87_slot = var_guard87;
        *var_guard88_slot = var_guard88;
        *var_guard89_slot = var_guard89;
        *var_guard90_slot = var_guard90;
        *var_guard91_slot = var_guard91;
        *var_guard92_slot = var_guard92;
        *var_guard93_slot = var_guard93;
        *var_invnf_slot = var_invnf;
        *var_kdiff_i_slot = var_kdiff_i;
        *var_kdiff_i_dn4_slot = var_kdiff_i_dn4;
        *var_kdiff_i_dn6_slot = var_kdiff_i_dn6;
        *var_kdiff_i_dn7_slot = var_kdiff_i_dn7;
        *var_kdiff_i_dn8_slot = var_kdiff_i_dn8;
        *var_kdiff_i_dn9_slot = var_kdiff_i_dn9;
        *var_kdrift_i_slot = var_kdrift_i;
        *var_kdrift_i_dn4_slot = var_kdrift_i_dn4;
        *var_kdrift_i_dn6_slot = var_kdrift_i_dn6;
        *var_kdrift_i_dn7_slot = var_kdrift_i_dn7;
        *var_kdrift_i_dn8_slot = var_kdrift_i_dn8;
        *var_kdrift_i_dn9_slot = var_kdrift_i_dn9;
        *var_kfracinv_i_slot = var_kfracinv_i;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_strth_i_slot = var_strth_i;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_w_i_slot = var_w_i;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        var_guard83: f64,
        var_invnf: f64,
        var_w_i: f64,
        var_adrain_i_slot: &mut f64,
        var_asource_i_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_dellps_slot: &mut f64,
        var_delwod_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_iae_slot: &mut f64,
        var_il_slot: &mut f64,
        var_ile_slot: &mut f64,
        var_iw_slot: &mut f64,
        var_iwe_slot: &mut f64,
        var_le_slot: &mut f64,
        var_lecv_slot: &mut f64,
        var_len_slot: &mut f64,
        var_lphy_slot: &mut f64,
        var_lphy_dn4_slot: &mut f64,
        var_lphy_dn6_slot: &mut f64,
        var_lphy_dn7_slot: &mut f64,
        var_lphy_dn8_slot: &mut f64,
        var_lphy_dn9_slot: &mut f64,
        var_mult_i_int_slot: &mut f64,
        var_nch_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_nsub_i_slot: &mut f64,
        var_pdrain_i_slot: &mut f64,
        var_psource_i_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_tox1_i_slot: &mut f64,
        var_tox2_i_slot: &mut f64,
        var_toxp_i_slot: &mut f64,
        var_tsi_i_slot: &mut f64,
        var_typech_i_slot: &mut f64,
        var_typesub_i_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_we_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wen_slot: &mut f64,
        var_wphy_slot: &mut f64,
        var_wphy_dn4_slot: &mut f64,
        var_wphy_dn6_slot: &mut f64,
        var_wphy_dn7_slot: &mut f64,
        var_wphy_dn8_slot: &mut f64,
        var_wphy_dn9_slot: &mut f64,
        var_xge_i_slot: &mut f64,
    ) {
        let mut var_adrain_i: f64 = *var_adrain_i_slot;
        let mut var_asource_i: f64 = *var_asource_i_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_dellps: f64 = *var_dellps_slot;
        let mut var_delwod: f64 = *var_delwod_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_iae: f64 = *var_iae_slot;
        let mut var_il: f64 = *var_il_slot;
        let mut var_ile: f64 = *var_ile_slot;
        let mut var_iw: f64 = *var_iw_slot;
        let mut var_iwe: f64 = *var_iwe_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_lecv: f64 = *var_lecv_slot;
        let mut var_len: f64 = *var_len_slot;
        let mut var_lphy: f64 = *var_lphy_slot;
        let mut var_lphy_dn4: f64 = *var_lphy_dn4_slot;
        let mut var_lphy_dn6: f64 = *var_lphy_dn6_slot;
        let mut var_lphy_dn7: f64 = *var_lphy_dn7_slot;
        let mut var_lphy_dn8: f64 = *var_lphy_dn8_slot;
        let mut var_lphy_dn9: f64 = *var_lphy_dn9_slot;
        let mut var_mult_i_int: f64 = *var_mult_i_int_slot;
        let mut var_nch_i: f64 = *var_nch_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_nsub_i: f64 = *var_nsub_i_slot;
        let mut var_pdrain_i: f64 = *var_pdrain_i_slot;
        let mut var_psource_i: f64 = *var_psource_i_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_tox1_i: f64 = *var_tox1_i_slot;
        let mut var_tox2_i: f64 = *var_tox2_i_slot;
        let mut var_toxp_i: f64 = *var_toxp_i_slot;
        let mut var_tsi_i: f64 = *var_tsi_i_slot;
        let mut var_typech_i: f64 = *var_typech_i_slot;
        let mut var_typesub_i: f64 = *var_typesub_i_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wen: f64 = *var_wen_slot;
        let mut var_wphy: f64 = *var_wphy_slot;
        let mut var_wphy_dn4: f64 = *var_wphy_dn4_slot;
        let mut var_wphy_dn6: f64 = *var_wphy_dn6_slot;
        let mut var_wphy_dn7: f64 = *var_wphy_dn7_slot;
        let mut var_wphy_dn8: f64 = *var_wphy_dn8_slot;
        let mut var_wphy_dn9: f64 = *var_wphy_dn9_slot;
        let mut var_xge_i: f64 = *var_xge_i_slot;

        let (assign2100_e1826,) = {
    if (var_guard83 == 0.0) {
        let assign2100_e1824: f64 = (p.p23 * var_invnf);
        (assign2100_e1824,)
    } else {
        (var_adrain_i,)
    }
};
        var_adrain_i = assign2100_e1826;

        let (assign2110_e1833,) = {
    if (var_guard83 == 0.0) {
        let assign2110_e1831: f64 = (p.p22 * var_invnf);
        (assign2110_e1831,)
    } else {
        (var_asource_i,)
    }
};
        var_asource_i = assign2110_e1833;

        let (assign2120_e1840,) = {
    if (var_guard83 == 0.0) {
        let assign2120_e1838: f64 = (p.p25 * var_invnf);
        (assign2120_e1838,)
    } else {
        (var_pdrain_i,)
    }
};
        var_pdrain_i = assign2120_e1840;

        let (assign2130_e1847,) = {
    if (var_guard83 == 0.0) {
        let assign2130_e1845: f64 = (p.p24 * var_invnf);
        (assign2130_e1845,)
    } else {
        (var_psource_i,)
    }
};
        var_psource_i = assign2130_e1847;

        let (assign2140_e1854,) = {
    if (var_guard83 == 0.0) {
        let assign2140_e1852: f64 = (p.p30 * p.p29);
        (assign2140_e1852,)
    } else {
        (var_mult_i_int,)
    }
};
        var_mult_i_int = assign2140_e1854;

        let (assign2150_e1859,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_len,)
    }
};
        var_len = assign2150_e1859;

        let (assign2160_e1864,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_wen,)
    }
};
        var_wen = assign2160_e1864;

        let (assign2170_e1871,) = {
    if (var_guard83 == 0.0) {
        let assign2170_e1869: f64 = (var_len / p.p20);
        (assign2170_e1869,)
    } else {
        (var_il,)
    }
};
        var_il = assign2170_e1871;

        let (assign2180_e1878,) = {
    if (var_guard83 == 0.0) {
        let assign2180_e1876: f64 = (var_wen / var_w_i);
        (assign2180_e1876,)
    } else {
        (var_iw,)
    }
};
        var_iw = assign2180_e1878;

        let (assign2190_e1895,) = {
    if (var_guard83 == 0.0) {
        let assign2190_e1885: f64 = (p.p192 * var_il);
        let assign2190_e1886: f64 = (1.0 + assign2190_e1885);
        let assign2190_e1887: f64 = (p.p191 * assign2190_e1886);
        let assign2190_e1891: f64 = (p.p193 * var_iw);
        let assign2190_e1892: f64 = (1.0 + assign2190_e1891);
        let assign2190_e1893: f64 = (assign2190_e1887 * assign2190_e1892);
        (assign2190_e1893,)
    } else {
        (var_dellps,)
    }
};
        var_dellps = assign2190_e1895;

        let (assign2200_e1912,) = {
    if (var_guard83 == 0.0) {
        let assign2200_e1902: f64 = (p.p197 * var_iw);
        let assign2200_e1903: f64 = (1.0 + assign2200_e1902);
        let assign2200_e1904: f64 = (p.p195 * assign2200_e1903);
        let assign2200_e1908: f64 = (p.p196 * var_il);
        let assign2200_e1909: f64 = (1.0 + assign2200_e1908);
        let assign2200_e1910: f64 = (assign2200_e1904 * assign2200_e1909);
        (assign2200_e1910,)
    } else {
        (var_delwod,)
    }
};
        var_delwod = assign2200_e1912;

        let (assign2210_e1925,) = {
    if (var_guard83 == 0.0) {
        let assign2210_e1917: f64 = (p.p20 + var_dellps);
        let assign2210_e1920: f64 = (2.0 * p.p194);
        let assign2210_e1921: f64 = (assign2210_e1917 - assign2210_e1920);
        let assign2210_e1923: f64 = (assign2210_e1921).max(1e-9);
        (assign2210_e1923,)
    } else {
        (var_le,)
    }
};
        var_le = assign2210_e1925;

        let (assign2220_e1938,) = {
    if (var_guard83 == 0.0) {
        let assign2220_e1930: f64 = (var_w_i + var_delwod);
        let assign2220_e1933: f64 = (2.0 * p.p198);
        let assign2220_e1934: f64 = (assign2220_e1930 - assign2220_e1933);
        let assign2220_e1936: f64 = (assign2220_e1934).max(1e-9);
        (assign2220_e1936,)
    } else {
        (var_we,)
    }
};
        var_we = assign2220_e1938;

        let (assign2230_e1953,) = {
    if (var_guard83 == 0.0) {
        let assign2230_e1943: f64 = (p.p20 + var_dellps);
        let assign2230_e1946: f64 = (2.0 * p.p194);
        let assign2230_e1947: f64 = (assign2230_e1943 - assign2230_e1946);
        let assign2230_e1949: f64 = (assign2230_e1947 + p.p199);
        let assign2230_e1951: f64 = (assign2230_e1949).max(1e-9);
        (assign2230_e1951,)
    } else {
        (var_lecv,)
    }
};
        var_lecv = assign2230_e1953;

        let (assign2240_e1968,) = {
    if (var_guard83 == 0.0) {
        let assign2240_e1958: f64 = (var_w_i + var_delwod);
        let assign2240_e1961: f64 = (2.0 * p.p198);
        let assign2240_e1962: f64 = (assign2240_e1958 - assign2240_e1961);
        let assign2240_e1964: f64 = (assign2240_e1962 + p.p200);
        let assign2240_e1966: f64 = (assign2240_e1964).max(1e-9);
        (assign2240_e1966,)
    } else {
        (var_wecv,)
    }
};
        var_wecv = assign2240_e1968;

        let (assign2250_e1975,) = {
    if (var_guard83 == 0.0) {
        let assign2250_e1973: f64 = (var_len / var_le);
        (assign2250_e1973,)
    } else {
        (var_ile,)
    }
};
        var_ile = assign2250_e1975;

        let (assign2260_e1982,) = {
    if (var_guard83 == 0.0) {
        let assign2260_e1980: f64 = (var_wen / var_we);
        (assign2260_e1980,)
    } else {
        (var_iwe,)
    }
};
        var_iwe = assign2260_e1982;

        let (assign2270_e1989,) = {
    if (var_guard83 == 0.0) {
        let assign2270_e1987: f64 = (var_ile * var_iwe);
        (assign2270_e1987,)
    } else {
        (var_iae,)
    }
};
        var_iae = assign2270_e1989;

        let (assign2280_e1998, assign2280_e1998_d_n4, assign2280_e1998_d_n6, assign2280_e1998_d_n7, assign2280_e1998_d_n8, assign2280_e1998_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2280_e1994: f64 = (p.p20 + var_dellps);
        let assign2280_e1996: f64 = (assign2280_e1994).max(1e-9);
        (assign2280_e1996, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2280_e1998;
        var_temp_dn4 = assign2280_e1998_d_n4;
        var_temp_dn6 = assign2280_e1998_d_n6;
        var_temp_dn7 = assign2280_e1998_d_n7;
        var_temp_dn8 = assign2280_e1998_d_n8;
        var_temp_dn9 = assign2280_e1998_d_n9;

        let (assign2290_e2005, assign2290_e2005_d_n4, assign2290_e2005_d_n6, assign2290_e2005_d_n7, assign2290_e2005_d_n8, assign2290_e2005_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2290_e2003: f64 = (var_temp / var_len);
        (assign2290_e2003, (var_temp_dn4 / var_len), (var_temp_dn6 / var_len), (var_temp_dn7 / var_len), (var_temp_dn8 / var_len), (var_temp_dn9 / var_len),)
    } else {
        (var_lphy, var_lphy_dn4, var_lphy_dn6, var_lphy_dn7, var_lphy_dn8, var_lphy_dn9,)
    }
};
        var_lphy = assign2290_e2005;
        var_lphy_dn4 = assign2290_e2005_d_n4;
        var_lphy_dn6 = assign2290_e2005_d_n6;
        var_lphy_dn7 = assign2290_e2005_d_n7;
        var_lphy_dn8 = assign2290_e2005_d_n8;
        var_lphy_dn9 = assign2290_e2005_d_n9;

        let (assign2300_e2014, assign2300_e2014_d_n4, assign2300_e2014_d_n6, assign2300_e2014_d_n7, assign2300_e2014_d_n8, assign2300_e2014_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2300_e2010: f64 = (var_w_i + var_delwod);
        let assign2300_e2012: f64 = (assign2300_e2010).max(1e-9);
        (assign2300_e2012, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2300_e2014;
        var_temp_dn4 = assign2300_e2014_d_n4;
        var_temp_dn6 = assign2300_e2014_d_n6;
        var_temp_dn7 = assign2300_e2014_d_n7;
        var_temp_dn8 = assign2300_e2014_d_n8;
        var_temp_dn9 = assign2300_e2014_d_n9;

        let (assign2310_e2021, assign2310_e2021_d_n4, assign2310_e2021_d_n6, assign2310_e2021_d_n7, assign2310_e2021_d_n8, assign2310_e2021_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2310_e2019: f64 = (var_temp / var_wen);
        (assign2310_e2019, (var_temp_dn4 / var_wen), (var_temp_dn6 / var_wen), (var_temp_dn7 / var_wen), (var_temp_dn8 / var_wen), (var_temp_dn9 / var_wen),)
    } else {
        (var_wphy, var_wphy_dn4, var_wphy_dn6, var_wphy_dn7, var_wphy_dn8, var_wphy_dn9,)
    }
};
        var_wphy = assign2310_e2021;
        var_wphy_dn4 = assign2310_e2021_d_n4;
        var_wphy_dn6 = assign2310_e2021_d_n6;
        var_wphy_dn7 = assign2310_e2021_d_n7;
        var_wphy_dn8 = assign2310_e2021_d_n8;
        var_wphy_dn9 = assign2310_e2021_d_n9;

        let (assign2360_e2064,) = {
    if (var_guard83 == 0.0) {
        (p.p201,)
    } else {
        (var_tox1_i,)
    }
};
        var_tox1_i = assign2360_e2064;

        let (assign2370_e2069,) = {
    if (var_guard83 == 0.0) {
        (p.p202,)
    } else {
        (var_tsi_i,)
    }
};
        var_tsi_i = assign2370_e2069;

        let (assign2380_e2074,) = {
    if (var_guard83 == 0.0) {
        (p.p203,)
    } else {
        (var_xge_i,)
    }
};
        var_xge_i = assign2380_e2074;

        let (assign2390_e2079,) = {
    if (var_guard83 == 0.0) {
        (p.p204,)
    } else {
        (var_tox2_i,)
    }
};
        var_tox2_i = assign2390_e2079;

        let (assign2400_e2084,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2400_e2084;

        let assign2410_e2087: f64 = if p.p205 < 0.0 { 1.0 } else { 0.0 };
        var_guard94 = assign2410_e2087;

        let (assign2420_e2095,) = {
    if ((var_guard83 == 0.0) && (var_guard94 != 0.0)) {
        let assign2420_e2093: f64 = (-1.0);
        (assign2420_e2093,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2420_e2095;

        let (assign2430_e2105,) = {
    if (var_guard83 == 0.0) {
        let assign2430_e2099: f64 = (p.p205).abs();
        let assign2430_e2101: f64 = (assign2430_e2099).min(1e19);
        let assign2430_e2103: f64 = (assign2430_e2101 * 1000000.0);
        (assign2430_e2103,)
    } else {
        (var_nch_i,)
    }
};
        var_nch_i = assign2430_e2105;

        let (assign2440_e2110,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2440_e2110;

        let assign2450_e2113: f64 = if p.p206 < 0.0 { 1.0 } else { 0.0 };
        var_guard95 = assign2450_e2113;

        let (assign2460_e2121,) = {
    if ((var_guard83 == 0.0) && (var_guard95 != 0.0)) {
        let assign2460_e2119: f64 = (-1.0);
        (assign2460_e2119,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2460_e2121;

        let (assign2470_e2133,) = {
    if (var_guard83 == 0.0) {
        let assign2470_e2125: f64 = (p.p206).abs();
        let assign2470_e2127: f64 = (assign2470_e2125).max(1e16);
        let assign2470_e2129: f64 = (assign2470_e2127).min(1e21);
        let assign2470_e2131: f64 = (assign2470_e2129 * 1000000.0);
        (assign2470_e2131,)
    } else {
        (var_nsub_i,)
    }
};
        var_nsub_i = assign2470_e2133;

        let (assign2480_e2138,) = {
    if (var_guard83 == 0.0) {
        (p.p207,)
    } else {
        (var_ct_i,)
    }
};
        var_ct_i = assign2480_e2138;

        let (assign2490_e2143,) = {
    if (var_guard83 == 0.0) {
        (p.p208,)
    } else {
        (var_toxp_i,)
    }
};
        var_toxp_i = assign2490_e2143;

        let (assign2500_e2150,) = {
    if (var_guard83 == 0.0) {
        let assign2500_e2148: f64 = (p.p209 * 1000000.0);
        (assign2500_e2148,)
    } else {
        (var_nov_i,)
    }
};
        var_nov_i = assign2500_e2150;

        let (assign2510_e2157,) = {
    if (var_guard83 == 0.0) {
        let assign2510_e2155: f64 = (p.p210 * 1000000.0);
        (assign2510_e2155,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign2510_e2157;

        let (assign2520_e2174, assign2520_e2174_d_n4, assign2520_e2174_d_n6, assign2520_e2174_d_n7, assign2520_e2174_d_n8, assign2520_e2174_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2520_e2163: f64 = (var_ile).powf(p.p213);
        let assign2520_e2164: f64 = (p.p212 * assign2520_e2163);
        let assign2520_e2169: f64 = (var_ile).powf(p.p215);
        let assign2520_e2170: f64 = (p.p214 * assign2520_e2169);
        let assign2520_e2171: f64 = (1.0 + assign2520_e2170);
        let assign2520_e2172: f64 = (assign2520_e2164 / assign2520_e2171);
        (assign2520_e2172, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2520_e2174;
        var_temp_dn4 = assign2520_e2174_d_n4;
        var_temp_dn6 = assign2520_e2174_d_n6;
        var_temp_dn7 = assign2520_e2174_d_n7;
        var_temp_dn8 = assign2520_e2174_d_n8;
        var_temp_dn9 = assign2520_e2174_d_n9;

        let (assign2530_e2189, assign2530_e2189_d_n4, assign2530_e2189_d_n6, assign2530_e2189_d_n7, assign2530_e2189_d_n8, assign2530_e2189_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2530_e2179: f64 = (p.p211 + var_temp);
        let assign2530_e2182: f64 = (p.p216 * var_iwe);
        let assign2530_e2183: f64 = (assign2530_e2179 + assign2530_e2182);
        let assign2530_e2186: f64 = (p.p217 * var_iae);
        let assign2530_e2187: f64 = (assign2530_e2183 + assign2530_e2186);
        (assign2530_e2187, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign2530_e2189;
        var_vfb1_t_dn4 = assign2530_e2189_d_n4;
        var_vfb1_t_dn6 = assign2530_e2189_d_n6;
        var_vfb1_t_dn7 = assign2530_e2189_d_n7;
        var_vfb1_t_dn8 = assign2530_e2189_d_n8;
        var_vfb1_t_dn9 = assign2530_e2189_d_n9;

        let (assign2540_e2202, assign2540_e2202_d_n4, assign2540_e2202_d_n6, assign2540_e2202_d_n7, assign2540_e2202_d_n8, assign2540_e2202_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2540_e2195: f64 = (p.p219 * var_tox2_i);
        let assign2540_e2197: f64 = (assign2540_e2195 / var_tox1_i);
        let assign2540_e2199: f64 = (assign2540_e2197 * var_temp);
        let assign2540_e2200: f64 = (p.p218 + assign2540_e2199);
        (assign2540_e2200, (assign2540_e2197 * var_temp_dn4), (assign2540_e2197 * var_temp_dn6), (assign2540_e2197 * var_temp_dn7), (assign2540_e2197 * var_temp_dn8), (assign2540_e2197 * var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign2540_e2202;
        var_vfb2_t_dn4 = assign2540_e2202_d_n4;
        var_vfb2_t_dn6 = assign2540_e2202_d_n6;
        var_vfb2_t_dn7 = assign2540_e2202_d_n7;
        var_vfb2_t_dn8 = assign2540_e2202_d_n8;
        var_vfb2_t_dn9 = assign2540_e2202_d_n9;

        let (assign2550_e2225,) = {
    if (var_guard83 == 0.0) {
        let assign2550_e2209: f64 = (p.p221 * var_ile);
        let assign2550_e2210: f64 = (1.0 + assign2550_e2209);
        let assign2550_e2211: f64 = (p.p220 * assign2550_e2210);
        let assign2550_e2215: f64 = (p.p222 * var_iwe);
        let assign2550_e2216: f64 = (1.0 + assign2550_e2215);
        let assign2550_e2217: f64 = (assign2550_e2211 * assign2550_e2216);
        let assign2550_e2221: f64 = (p.p223 * var_iae);
        let assign2550_e2222: f64 = (1.0 + assign2550_e2221);
        let assign2550_e2223: f64 = (assign2550_e2217 * assign2550_e2222);
        (assign2550_e2223,)
    } else {
        (var_stvfb_i,)
    }
};
        var_stvfb_i = assign2550_e2225;

        let (assign2560_e2238, assign2560_e2238_d_n4, assign2560_e2238_d_n6, assign2560_e2238_d_n7, assign2560_e2238_d_n8, assign2560_e2238_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2560_e2232: f64 = (p.p225 * var_ile);
        let assign2560_e2233: f64 = (1.0 + assign2560_e2232);
        let assign2560_e2234: f64 = (p.p224 * assign2560_e2233);
        let assign2560_e2236: f64 = (assign2560_e2234 * 1000000.0);
        (assign2560_e2236, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign2560_e2238;
        var_temp0__blk79_dn4 = assign2560_e2238_d_n4;
        var_temp0__blk79_dn6 = assign2560_e2238_d_n6;
        var_temp0__blk79_dn7 = assign2560_e2238_d_n7;
        var_temp0__blk79_dn8 = assign2560_e2238_d_n8;
        var_temp0__blk79_dn9 = assign2560_e2238_d_n9;

        *var_adrain_i_slot = var_adrain_i;
        *var_asource_i_slot = var_asource_i;
        *var_ct_i_slot = var_ct_i;
        *var_dellps_slot = var_dellps;
        *var_delwod_slot = var_delwod;
        *var_guard94_slot = var_guard94;
        *var_guard95_slot = var_guard95;
        *var_iae_slot = var_iae;
        *var_il_slot = var_il;
        *var_ile_slot = var_ile;
        *var_iw_slot = var_iw;
        *var_iwe_slot = var_iwe;
        *var_le_slot = var_le;
        *var_lecv_slot = var_lecv;
        *var_len_slot = var_len;
        *var_lphy_slot = var_lphy;
        *var_lphy_dn4_slot = var_lphy_dn4;
        *var_lphy_dn6_slot = var_lphy_dn6;
        *var_lphy_dn7_slot = var_lphy_dn7;
        *var_lphy_dn8_slot = var_lphy_dn8;
        *var_lphy_dn9_slot = var_lphy_dn9;
        *var_mult_i_int_slot = var_mult_i_int;
        *var_nch_i_slot = var_nch_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_nsub_i_slot = var_nsub_i;
        *var_pdrain_i_slot = var_pdrain_i;
        *var_psource_i_slot = var_psource_i;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_tox1_i_slot = var_tox1_i;
        *var_tox2_i_slot = var_tox2_i;
        *var_toxp_i_slot = var_toxp_i;
        *var_tsi_i_slot = var_tsi_i;
        *var_typech_i_slot = var_typech_i;
        *var_typesub_i_slot = var_typesub_i;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_wen_slot = var_wen;
        *var_wphy_slot = var_wphy;
        *var_wphy_dn4_slot = var_wphy_dn4;
        *var_wphy_dn6_slot = var_wphy_dn6;
        *var_wphy_dn7_slot = var_wphy_dn7;
        *var_wphy_dn8_slot = var_wphy_dn8;
        *var_wphy_dn9_slot = var_wphy_dn9;
        *var_xge_i_slot = var_xge_i;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_guard83: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_temp0__blk79: f64,
        var_temp0__blk79_dn4: f64,
        var_temp0__blk79_dn6: f64,
        var_temp0__blk79_dn7: f64,
        var_temp0__blk79_dn8: f64,
        var_temp0__blk79_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_tsi_i: f64,
        var_we: f64,
        var_xge_i: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfdl_i_slot: &mut f64,
        var_cfdlb_i_slot: &mut f64,
        var_cic1_i_slot: &mut f64,
        var_cic2_i_slot: &mut f64,
        var_epsch_slot: &mut f64,
        var_ge_slot: &mut f64,
        var_ge_dn4_slot: &mut f64,
        var_ge_dn6_slot: &mut f64,
        var_ge_dn7_slot: &mut f64,
        var_ge_dn8_slot: &mut f64,
        var_ge_dn9_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_dn4_slot: &mut f64,
        var_gpe_dn6_slot: &mut f64,
        var_gpe_dn7_slot: &mut f64,
        var_gpe_dn8_slot: &mut f64,
        var_gpe_dn9_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_gwe_slot: &mut f64,
        var_lambda_le_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_dn4_slot: &mut f64,
        var_np_i_dn6_slot: &mut f64,
        var_np_i_dn7_slot: &mut f64,
        var_np_i_dn8_slot: &mut f64,
        var_np_i_dn9_slot: &mut f64,
        var_nsddc_i_slot: &mut f64,
        var_one_m_xge_slot: &mut f64,
        var_pnce_i_slot: &mut f64,
        var_pnce_p_slot: &mut f64,
        var_psce1_i_slot: &mut f64,
        var_psce2_i_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_pscedlb_i_slot: &mut f64,
        var_stcf_i_slot: &mut f64,
        var_stcf_i_dn4_slot: &mut f64,
        var_stcf_i_dn6_slot: &mut f64,
        var_stcf_i_dn7_slot: &mut f64,
        var_stcf_i_dn8_slot: &mut f64,
        var_stcf_i_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfdl_i: f64 = *var_cfdl_i_slot;
        let mut var_cfdlb_i: f64 = *var_cfdlb_i_slot;
        let mut var_cic1_i: f64 = *var_cic1_i_slot;
        let mut var_cic2_i: f64 = *var_cic2_i_slot;
        let mut var_epsch: f64 = *var_epsch_slot;
        let mut var_ge: f64 = *var_ge_slot;
        let mut var_ge_dn4: f64 = *var_ge_dn4_slot;
        let mut var_ge_dn6: f64 = *var_ge_dn6_slot;
        let mut var_ge_dn7: f64 = *var_ge_dn7_slot;
        let mut var_ge_dn8: f64 = *var_ge_dn8_slot;
        let mut var_ge_dn9: f64 = *var_ge_dn9_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_dn4: f64 = *var_gpe_dn4_slot;
        let mut var_gpe_dn6: f64 = *var_gpe_dn6_slot;
        let mut var_gpe_dn7: f64 = *var_gpe_dn7_slot;
        let mut var_gpe_dn8: f64 = *var_gpe_dn8_slot;
        let mut var_gpe_dn9: f64 = *var_gpe_dn9_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
        let mut var_lambda_le: f64 = *var_lambda_le_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_dn4: f64 = *var_np_i_dn4_slot;
        let mut var_np_i_dn6: f64 = *var_np_i_dn6_slot;
        let mut var_np_i_dn7: f64 = *var_np_i_dn7_slot;
        let mut var_np_i_dn8: f64 = *var_np_i_dn8_slot;
        let mut var_np_i_dn9: f64 = *var_np_i_dn9_slot;
        let mut var_nsddc_i: f64 = *var_nsddc_i_slot;
        let mut var_one_m_xge: f64 = *var_one_m_xge_slot;
        let mut var_pnce_i: f64 = *var_pnce_i_slot;
        let mut var_pnce_p: f64 = *var_pnce_p_slot;
        let mut var_psce1_i: f64 = *var_psce1_i_slot;
        let mut var_psce2_i: f64 = *var_psce2_i_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_pscedlb_i: f64 = *var_pscedlb_i_slot;
        let mut var_stcf_i: f64 = *var_stcf_i_slot;
        let mut var_stcf_i_dn4: f64 = *var_stcf_i_dn4_slot;
        let mut var_stcf_i_dn6: f64 = *var_stcf_i_dn6_slot;
        let mut var_stcf_i_dn7: f64 = *var_stcf_i_dn7_slot;
        let mut var_stcf_i_dn8: f64 = *var_stcf_i_dn8_slot;
        let mut var_stcf_i_dn9: f64 = *var_stcf_i_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign2570_e2247, assign2570_e2247_d_n4, assign2570_e2247_d_n6, assign2570_e2247_d_n7, assign2570_e2247_d_n8, assign2570_e2247_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2570_e2243: f64 = (var_temp0__blk79).max(1e25);
        let assign2570_e2245: f64 = (assign2570_e2243).min(1e28);
        (assign2570_e2245, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2570_e2243 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_np_i, var_np_i_dn4, var_np_i_dn6, var_np_i_dn7, var_np_i_dn8, var_np_i_dn9,)
    }
};
        var_np_i = assign2570_e2247;
        var_np_i_dn4 = assign2570_e2247_d_n4;
        var_np_i_dn6 = assign2570_e2247_d_n6;
        var_np_i_dn7 = assign2570_e2247_d_n7;
        var_np_i_dn8 = assign2570_e2247_d_n8;
        var_np_i_dn9 = assign2570_e2247_d_n9;

        let (assign2580_e2252,) = {
    if (var_guard83 == 0.0) {
        (p.p226,)
    } else {
        (var_cic1_i,)
    }
};
        var_cic1_i = assign2580_e2252;

        let (assign2590_e2257,) = {
    if (var_guard83 == 0.0) {
        (p.p227,)
    } else {
        (var_cic2_i,)
    }
};
        var_cic2_i = assign2590_e2257;

        let (assign2600_e2264,) = {
    if (var_guard83 == 0.0) {
        let assign2600_e2262: f64 = (1.0 - var_xge_i);
        (assign2600_e2262,)
    } else {
        (var_one_m_xge,)
    }
};
        var_one_m_xge = assign2600_e2264;

        let (assign2610_e2275,) = {
    if (var_guard83 == 0.0) {
        let assign2610_e2269: f64 = (1.04479e-10 * var_one_m_xge);
        let assign2610_e2272: f64 = (1.43438e-10 * var_xge_i);
        let assign2610_e2273: f64 = (assign2610_e2269 + assign2610_e2272);
        (assign2610_e2273,)
    } else {
        (var_epsch,)
    }
};
        var_epsch = assign2610_e2275;

        let (assign2620_e2291,) = {
    if (var_guard83 == 0.0) {
        let assign2620_e2280: f64 = (var_epsch / 3.45313e-11);
        let assign2620_e2282: f64 = (assign2620_e2280 * var_tsi_i);
        let assign2620_e2285: f64 = (var_tox1_i + 4e-10);
        let assign2620_e2286: f64 = (assign2620_e2282 * assign2620_e2285);
        let assign2620_e2287: f64 = (assign2620_e2286).sqrt();
        let assign2620_e2289: f64 = (assign2620_e2287 / var_le);
        (assign2620_e2289,)
    } else {
        (var_lambda_le,)
    }
};
        var_lambda_le = assign2620_e2291;

        let (assign2630_e2308,) = {
    if (var_guard83 == 0.0) {
        let assign2630_e2296: f64 = (p.p228 * 2.0);
        let assign2630_e2299: f64 = (var_lambda_le).powf(p.p229);
        let assign2630_e2300: f64 = (assign2630_e2296 * assign2630_e2299);
        let assign2630_e2304: f64 = (p.p230 * var_iwe);
        let assign2630_e2305: f64 = (1.0 + assign2630_e2304);
        let assign2630_e2306: f64 = (assign2630_e2300 * assign2630_e2305);
        (assign2630_e2306,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign2630_e2308;

        let (assign2640_e2317,) = {
    if (var_guard83 == 0.0) {
        let assign2640_e2313: f64 = (var_psce_p).max(0.0);
        let assign2640_e2315: f64 = (assign2640_e2313).min(5.0);
        (assign2640_e2315,)
    } else {
        (var_psce1_i,)
    }
};
        var_psce1_i = assign2640_e2317;

        let (assign2650_e2328,) = {
    if (var_guard83 == 0.0) {
        let assign2650_e2322: f64 = (p.p231 * var_psce1_i);
        let assign2650_e2324: f64 = (assign2650_e2322 * var_tox2_i);
        let assign2650_e2326: f64 = (assign2650_e2324 / var_tox1_i);
        (assign2650_e2326,)
    } else {
        (var_psce2_i,)
    }
};
        var_psce2_i = assign2650_e2328;

        let (assign2660_e2335,) = {
    if (var_guard83 == 0.0) {
        let assign2660_e2333: f64 = (p.p232 * 1000000.0);
        (assign2660_e2333,)
    } else {
        (var_nsddc_i,)
    }
};
        var_nsddc_i = assign2660_e2335;

        let (assign2670_e2340,) = {
    if (var_guard83 == 0.0) {
        (p.p233,)
    } else {
        (var_pscedlb_i,)
    }
};
        var_pscedlb_i = assign2670_e2340;

        let (assign2680_e2347,) = {
    if (var_guard83 == 0.0) {
        let assign2680_e2345: f64 = (p.p234 * var_iwe);
        (assign2680_e2345,)
    } else {
        (var_pnce_p,)
    }
};
        var_pnce_p = assign2680_e2347;

        let (assign2690_e2357,) = {
    if (var_guard83 == 0.0) {
        let assign2690_e2352: f64 = (-1.0);
        let assign2690_e2353: f64 = (var_pnce_p).max(assign2690_e2352);
        let assign2690_e2355: f64 = (assign2690_e2353).min(1.0);
        (assign2690_e2355,)
    } else {
        (var_pnce_i,)
    }
};
        var_pnce_i = assign2690_e2357;

        let (assign2700_e2370, assign2700_e2370_d_n4, assign2700_e2370_d_n6, assign2700_e2370_d_n7, assign2700_e2370_d_n8, assign2700_e2370_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2700_e2362: f64 = (var_lambda_le).powf(p.p236);
        let assign2700_e2366: f64 = (p.p237 * var_iwe);
        let assign2700_e2367: f64 = (1.0 + assign2700_e2366);
        let assign2700_e2368: f64 = (assign2700_e2362 * assign2700_e2367);
        (assign2700_e2368, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign2700_e2370;
        var_temp_dn4 = assign2700_e2370_d_n4;
        var_temp_dn6 = assign2700_e2370_d_n6;
        var_temp_dn7 = assign2700_e2370_d_n7;
        var_temp_dn8 = assign2700_e2370_d_n8;
        var_temp_dn9 = assign2700_e2370_d_n9;

        let (assign2710_e2377, assign2710_e2377_d_n4, assign2710_e2377_d_n6, assign2710_e2377_d_n7, assign2710_e2377_d_n8, assign2710_e2377_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2710_e2375: f64 = (p.p235 * var_temp);
        (assign2710_e2375, (p.p235 * var_temp_dn4), (p.p235 * var_temp_dn6), (p.p235 * var_temp_dn7), (p.p235 * var_temp_dn8), (p.p235 * var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign2710_e2377;
        var_cf_p_dn4 = assign2710_e2377_d_n4;
        var_cf_p_dn6 = assign2710_e2377_d_n6;
        var_cf_p_dn7 = assign2710_e2377_d_n7;
        var_cf_p_dn8 = assign2710_e2377_d_n8;
        var_cf_p_dn9 = assign2710_e2377_d_n9;

        let (assign2720_e2384, assign2720_e2384_d_n4, assign2720_e2384_d_n6, assign2720_e2384_d_n7, assign2720_e2384_d_n8, assign2720_e2384_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2720_e2382: f64 = (var_cf_p).max(0.0);
        (assign2720_e2382, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign2720_e2384;
        var_cf1_t_dn4 = assign2720_e2384_d_n4;
        var_cf1_t_dn6 = assign2720_e2384_d_n6;
        var_cf1_t_dn7 = assign2720_e2384_d_n7;
        var_cf1_t_dn8 = assign2720_e2384_d_n8;
        var_cf1_t_dn9 = assign2720_e2384_d_n9;

        let (assign2730_e2395, assign2730_e2395_d_n4, assign2730_e2395_d_n6, assign2730_e2395_d_n7, assign2730_e2395_d_n8, assign2730_e2395_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2730_e2389: f64 = (p.p238 * var_cf1_t);
        let assign2730_e2391: f64 = (assign2730_e2389 * var_tox2_i);
        let assign2730_e2393: f64 = (assign2730_e2391 / var_tox1_i);
        (assign2730_e2393, (((p.p238 * var_cf1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cf1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign2730_e2395;
        var_cf2_t_dn4 = assign2730_e2395_d_n4;
        var_cf2_t_dn6 = assign2730_e2395_d_n6;
        var_cf2_t_dn7 = assign2730_e2395_d_n7;
        var_cf2_t_dn8 = assign2730_e2395_d_n8;
        var_cf2_t_dn9 = assign2730_e2395_d_n9;

        let (assign2740_e2402, assign2740_e2402_d_n4, assign2740_e2402_d_n6, assign2740_e2402_d_n7, assign2740_e2402_d_n8, assign2740_e2402_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2740_e2400: f64 = (p.p239 * var_temp);
        (assign2740_e2400, (p.p239 * var_temp_dn4), (p.p239 * var_temp_dn6), (p.p239 * var_temp_dn7), (p.p239 * var_temp_dn8), (p.p239 * var_temp_dn9),)
    } else {
        (var_stcf_i, var_stcf_i_dn4, var_stcf_i_dn6, var_stcf_i_dn7, var_stcf_i_dn8, var_stcf_i_dn9,)
    }
};
        var_stcf_i = assign2740_e2402;
        var_stcf_i_dn4 = assign2740_e2402_d_n4;
        var_stcf_i_dn6 = assign2740_e2402_d_n6;
        var_stcf_i_dn7 = assign2740_e2402_d_n7;
        var_stcf_i_dn8 = assign2740_e2402_d_n8;
        var_stcf_i_dn9 = assign2740_e2402_d_n9;

        let (assign2750_e2407,) = {
    if (var_guard83 == 0.0) {
        (p.p240,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign2750_e2407;

        let (assign2760_e2422,) = {
    if (var_guard83 == 0.0) {
        let assign2760_e2412: f64 = (p.p241 * var_ile);
        let assign2760_e2416: f64 = (p.p242 * var_iwe);
        let assign2760_e2417: f64 = (1.0 + assign2760_e2416);
        let assign2760_e2419: f64 = (assign2760_e2417).max(0.001);
        let assign2760_e2420: f64 = (assign2760_e2412 / assign2760_e2419);
        (assign2760_e2420,)
    } else {
        (var_cfdl_i,)
    }
};
        var_cfdl_i = assign2760_e2422;

        let (assign2770_e2427,) = {
    if (var_guard83 == 0.0) {
        (p.p243,)
    } else {
        (var_cfdlb_i,)
    }
};
        var_cfdlb_i = assign2770_e2427;

        let (assign2780_e2443, assign2780_e2443_d_n4, assign2780_e2443_d_n6, assign2780_e2443_d_n7, assign2780_e2443_d_n8, assign2780_e2443_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2780_e2431: f64 = (-var_le);
        let assign2780_e2436: f64 = (p.p248 * var_iwe);
        let assign2780_e2437: f64 = (1.0 + assign2780_e2436);
        let assign2780_e2439: f64 = (assign2780_e2437).max(0.001);
        let assign2780_e2440: f64 = (p.p247 * assign2780_e2439);
        let assign2780_e2441: f64 = (assign2780_e2431 / assign2780_e2440);
        (assign2780_e2441, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign2780_e2443;
        var_temp1_dn4 = assign2780_e2443_d_n4;
        var_temp1_dn6 = assign2780_e2443_d_n6;
        var_temp1_dn7 = assign2780_e2443_d_n7;
        var_temp1_dn8 = assign2780_e2443_d_n8;
        var_temp1_dn9 = assign2780_e2443_d_n9;

        let assign2790_e2446: f64 = (-80.0);
        let assign2790_e2447: f64 = if var_temp1 > assign2790_e2446 { 1.0 } else { 0.0 };
        var_guard96 = assign2790_e2447;

        let (assign2800_e2455, assign2800_e2455_d_n4, assign2800_e2455_d_n6, assign2800_e2455_d_n7, assign2800_e2455_d_n8, assign2800_e2455_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard96 != 0.0)) {
        let assign2800_e2453: f64 = (var_temp1).exp();
        (assign2800_e2453, (assign2800_e2453 * var_temp1_dn4), (assign2800_e2453 * var_temp1_dn6), (assign2800_e2453 * var_temp1_dn7), (assign2800_e2453 * var_temp1_dn8), (assign2800_e2453 * var_temp1_dn9),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign2800_e2455;
        var_temp2_dn4 = assign2800_e2455_d_n4;
        var_temp2_dn6 = assign2800_e2455_d_n6;
        var_temp2_dn7 = assign2800_e2455_d_n7;
        var_temp2_dn8 = assign2800_e2455_d_n8;
        var_temp2_dn9 = assign2800_e2455_d_n9;

        let (assign2810_e2488, assign2810_e2488_d_n4, assign2810_e2488_d_n6, assign2810_e2488_d_n7, assign2810_e2488_d_n8, assign2810_e2488_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard96 == 0.0)) {
        let assign2810_e2464: f64 = (-var_temp1);
        let assign2810_e2466: f64 = (assign2810_e2464 - 80.0);
        let assign2810_e2470: f64 = (-var_temp1);
        let assign2810_e2472: f64 = (assign2810_e2470 - 80.0);
        let assign2810_e2473: f64 = (0.5 * assign2810_e2472);
        let assign2810_e2476: f64 = (-var_temp1);
        let assign2810_e2478: f64 = (assign2810_e2476 - 80.0);
        let assign2810_e2480: f64 = (assign2810_e2478 * 0.3333333333333);
        let assign2810_e2481: f64 = (1.0 + assign2810_e2480);
        let assign2810_e2482: f64 = (assign2810_e2473 * assign2810_e2481);
        let assign2810_e2483: f64 = (1.0 + assign2810_e2482);
        let assign2810_e2484: f64 = (assign2810_e2466 * assign2810_e2483);
        let assign2810_e2485: f64 = (1.0 + assign2810_e2484);
        let assign2810_e2486: f64 = (1.80485e-35 / assign2810_e2485);
        (assign2810_e2486, (-((1.80485e-35 * (((-var_temp1_dn4) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn4)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn4) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn6) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn6)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn6) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn7) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn7)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn7) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn8) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn8)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn8) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))), (-((1.80485e-35 * (((-var_temp1_dn9) * assign2810_e2483) + (assign2810_e2466 * (((0.5 * (-var_temp1_dn9)) * assign2810_e2481) + (assign2810_e2473 * ((-var_temp1_dn9) * 0.3333333333333)))))) / (assign2810_e2485 * assign2810_e2485))),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign2810_e2488;
        var_temp2_dn4 = assign2810_e2488_d_n4;
        var_temp2_dn6 = assign2810_e2488_d_n6;
        var_temp2_dn7 = assign2810_e2488_d_n7;
        var_temp2_dn8 = assign2810_e2488_d_n8;
        var_temp2_dn9 = assign2810_e2488_d_n9;

        let (assign2820_e2496, assign2820_e2496_d_n4, assign2820_e2496_d_n6, assign2820_e2496_d_n7, assign2820_e2496_d_n8, assign2820_e2496_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2820_e2492: f64 = (-var_le);
        let assign2820_e2494: f64 = (assign2820_e2492 / p.p250);
        (assign2820_e2494, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign2820_e2496;
        var_temp3_dn4 = assign2820_e2496_d_n4;
        var_temp3_dn6 = assign2820_e2496_d_n6;
        var_temp3_dn7 = assign2820_e2496_d_n7;
        var_temp3_dn8 = assign2820_e2496_d_n8;
        var_temp3_dn9 = assign2820_e2496_d_n9;

        let assign2830_e2499: f64 = (-80.0);
        let assign2830_e2500: f64 = if var_temp3 > assign2830_e2499 { 1.0 } else { 0.0 };
        var_guard97 = assign2830_e2500;

        let (assign2840_e2508, assign2840_e2508_d_n4, assign2840_e2508_d_n6, assign2840_e2508_d_n7, assign2840_e2508_d_n8, assign2840_e2508_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard97 != 0.0)) {
        let assign2840_e2506: f64 = (var_temp3).exp();
        (assign2840_e2506, (assign2840_e2506 * var_temp3_dn4), (assign2840_e2506 * var_temp3_dn6), (assign2840_e2506 * var_temp3_dn7), (assign2840_e2506 * var_temp3_dn8), (assign2840_e2506 * var_temp3_dn9),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign2840_e2508;
        var_temp4_dn4 = assign2840_e2508_d_n4;
        var_temp4_dn6 = assign2840_e2508_d_n6;
        var_temp4_dn7 = assign2840_e2508_d_n7;
        var_temp4_dn8 = assign2840_e2508_d_n8;
        var_temp4_dn9 = assign2840_e2508_d_n9;

        let (assign2850_e2541, assign2850_e2541_d_n4, assign2850_e2541_d_n6, assign2850_e2541_d_n7, assign2850_e2541_d_n8, assign2850_e2541_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard97 == 0.0)) {
        let assign2850_e2517: f64 = (-var_temp3);
        let assign2850_e2519: f64 = (assign2850_e2517 - 80.0);
        let assign2850_e2523: f64 = (-var_temp3);
        let assign2850_e2525: f64 = (assign2850_e2523 - 80.0);
        let assign2850_e2526: f64 = (0.5 * assign2850_e2525);
        let assign2850_e2529: f64 = (-var_temp3);
        let assign2850_e2531: f64 = (assign2850_e2529 - 80.0);
        let assign2850_e2533: f64 = (assign2850_e2531 * 0.3333333333333);
        let assign2850_e2534: f64 = (1.0 + assign2850_e2533);
        let assign2850_e2535: f64 = (assign2850_e2526 * assign2850_e2534);
        let assign2850_e2536: f64 = (1.0 + assign2850_e2535);
        let assign2850_e2537: f64 = (assign2850_e2519 * assign2850_e2536);
        let assign2850_e2538: f64 = (1.0 + assign2850_e2537);
        let assign2850_e2539: f64 = (1.80485e-35 / assign2850_e2538);
        (assign2850_e2539, (-((1.80485e-35 * (((-var_temp3_dn4) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn4)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn4) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn6) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn6)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn6) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn7) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn7)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn7) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn8) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn8)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn8) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))), (-((1.80485e-35 * (((-var_temp3_dn9) * assign2850_e2536) + (assign2850_e2519 * (((0.5 * (-var_temp3_dn9)) * assign2850_e2534) + (assign2850_e2526 * ((-var_temp3_dn9) * 0.3333333333333)))))) / (assign2850_e2538 * assign2850_e2538))),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign2850_e2541;
        var_temp4_dn4 = assign2850_e2541_d_n4;
        var_temp4_dn6 = assign2850_e2541_d_n6;
        var_temp4_dn7 = assign2850_e2541_d_n7;
        var_temp4_dn8 = assign2850_e2541_d_n8;
        var_temp4_dn9 = assign2850_e2541_d_n9;

        let (assign2860_e2570, assign2860_e2570_d_n4, assign2860_e2570_d_n6, assign2860_e2570_d_n7, assign2860_e2570_d_n8, assign2860_e2570_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2860_e2549: f64 = (p.p246 * var_iwe);
        let assign2860_e2550: f64 = (1.0 + assign2860_e2549);
        let assign2860_e2551: f64 = (p.p245 * assign2860_e2550);
        let assign2860_e2554: f64 = (var_temp2 - 1.0);
        let assign2860_e2555: f64 = (assign2860_e2551 * assign2860_e2554);
        let assign2860_e2557: f64 = (assign2860_e2555 / var_temp1);
        let assign2860_e2558: f64 = (1.0 + assign2860_e2557);
        let assign2860_e2562: f64 = (var_temp4 - 1.0);
        let assign2860_e2563: f64 = (p.p249 * assign2860_e2562);
        let assign2860_e2565: f64 = (assign2860_e2563 / var_temp3);
        let assign2860_e2566: f64 = (assign2860_e2558 + assign2860_e2565);
        let assign2860_e2568: f64 = (assign2860_e2566).max(1e-6);
        (assign2860_e2568, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn4) * var_temp1) - (assign2860_e2555 * var_temp1_dn4)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn4) * var_temp3) - (assign2860_e2563 * var_temp3_dn4)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn6) * var_temp1) - (assign2860_e2555 * var_temp1_dn6)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn6) * var_temp3) - (assign2860_e2563 * var_temp3_dn6)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn7) * var_temp1) - (assign2860_e2555 * var_temp1_dn7)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn7) * var_temp3) - (assign2860_e2563 * var_temp3_dn7)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn8) * var_temp1) - (assign2860_e2555 * var_temp1_dn8)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn8) * var_temp3) - (assign2860_e2563 * var_temp3_dn8)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2860_e2566 >= 1e-6 { (((((assign2860_e2551 * var_temp2_dn9) * var_temp1) - (assign2860_e2555 * var_temp1_dn9)) / (var_temp1 * var_temp1)) + ((((p.p249 * var_temp4_dn9) * var_temp3) - (assign2860_e2563 * var_temp3_dn9)) / (var_temp3 * var_temp3))) } else { 0.0 },)
    } else {
        (var_gpe, var_gpe_dn4, var_gpe_dn6, var_gpe_dn7, var_gpe_dn8, var_gpe_dn9,)
    }
};
        var_gpe = assign2860_e2570;
        var_gpe_dn4 = assign2860_e2570_d_n4;
        var_gpe_dn6 = assign2860_e2570_d_n6;
        var_gpe_dn7 = assign2860_e2570_d_n7;
        var_gpe_dn8 = assign2860_e2570_d_n8;
        var_gpe_dn9 = assign2860_e2570_d_n9;

        let (assign2870_e2592,) = {
    if (var_guard83 == 0.0) {
        let assign2870_e2576: f64 = (p.p251 * var_iwe);
        let assign2870_e2577: f64 = (1.0 + assign2870_e2576);
        let assign2870_e2580: f64 = (p.p252 * var_iwe);
        let assign2870_e2584: f64 = (var_we / p.p253);
        let assign2870_e2585: f64 = (1.0 + assign2870_e2584);
        let assign2870_e2586: f64 = (assign2870_e2585).ln();
        let assign2870_e2587: f64 = (assign2870_e2580 * assign2870_e2586);
        let assign2870_e2588: f64 = (assign2870_e2577 + assign2870_e2587);
        let assign2870_e2590: f64 = (assign2870_e2588).max(1e-6);
        (assign2870_e2590,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign2870_e2592;

        let (assign2880_e2601, assign2880_e2601_d_n4, assign2880_e2601_d_n6, assign2880_e2601_d_n7, assign2880_e2601_d_n8, assign2880_e2601_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2880_e2597: f64 = (p.p244 / var_gpe);
        let assign2880_e2599: f64 = (assign2880_e2597 * var_gwe);
        (assign2880_e2599, ((-((p.p244 * var_gpe_dn4) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn6) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn7) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn8) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p244 * var_gpe_dn9) / (var_gpe * var_gpe))) * var_gwe),)
    } else {
        (var_ge, var_ge_dn4, var_ge_dn6, var_ge_dn7, var_ge_dn8, var_ge_dn9,)
    }
};
        var_ge = assign2880_e2601;
        var_ge_dn4 = assign2880_e2601_d_n4;
        var_ge_dn6 = assign2880_e2601_d_n6;
        var_ge_dn7 = assign2880_e2601_d_n7;
        var_ge_dn8 = assign2880_e2601_d_n8;
        var_ge_dn9 = assign2880_e2601_d_n9;

        let (assign2890_e2610, assign2890_e2610_d_n4, assign2890_e2610_d_n6, assign2890_e2610_d_n7, assign2890_e2610_d_n8, assign2890_e2610_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2890_e2606: f64 = (var_ge * var_we);
        let assign2890_e2608: f64 = (assign2890_e2606 / var_le);
        (assign2890_e2608, ((var_ge_dn4 * var_we) / var_le), ((var_ge_dn6 * var_we) / var_le), ((var_ge_dn7 * var_we) / var_le), ((var_ge_dn8 * var_we) / var_le), ((var_ge_dn9 * var_we) / var_le),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign2890_e2610;
        var_betn_p_dn4 = assign2890_e2610_d_n4;
        var_betn_p_dn6 = assign2890_e2610_d_n6;
        var_betn_p_dn7 = assign2890_e2610_d_n7;
        var_betn_p_dn8 = assign2890_e2610_d_n8;
        var_betn_p_dn9 = assign2890_e2610_d_n9;

        let (assign2900_e2617, assign2900_e2617_d_n4, assign2900_e2617_d_n6, assign2900_e2617_d_n7, assign2900_e2617_d_n8, assign2900_e2617_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2900_e2615: f64 = (var_betn_p).max(1e-10);
        (assign2900_e2615, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign2900_e2617;
        var_betn1_t_dn4 = assign2900_e2617_d_n4;
        var_betn1_t_dn6 = assign2900_e2617_d_n6;
        var_betn1_t_dn7 = assign2900_e2617_d_n7;
        var_betn1_t_dn8 = assign2900_e2617_d_n8;
        var_betn1_t_dn9 = assign2900_e2617_d_n9;

        let (assign2910_e2624, assign2910_e2624_d_n4, assign2910_e2624_d_n6, assign2910_e2624_d_n7, assign2910_e2624_d_n8, assign2910_e2624_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign2910_e2622: f64 = (p.p254 * var_betn1_t);
        (assign2910_e2622, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign2910_e2624;
        var_betn2_t_dn4 = assign2910_e2624_d_n4;
        var_betn2_t_dn6 = assign2910_e2624_d_n6;
        var_betn2_t_dn7 = assign2910_e2624_d_n7;
        var_betn2_t_dn8 = assign2910_e2624_d_n8;
        var_betn2_t_dn9 = assign2910_e2624_d_n9;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfdl_i_slot = var_cfdl_i;
        *var_cfdlb_i_slot = var_cfdlb_i;
        *var_cic1_i_slot = var_cic1_i;
        *var_cic2_i_slot = var_cic2_i;
        *var_epsch_slot = var_epsch;
        *var_ge_slot = var_ge;
        *var_ge_dn4_slot = var_ge_dn4;
        *var_ge_dn6_slot = var_ge_dn6;
        *var_ge_dn7_slot = var_ge_dn7;
        *var_ge_dn8_slot = var_ge_dn8;
        *var_ge_dn9_slot = var_ge_dn9;
        *var_gpe_slot = var_gpe;
        *var_gpe_dn4_slot = var_gpe_dn4;
        *var_gpe_dn6_slot = var_gpe_dn6;
        *var_gpe_dn7_slot = var_gpe_dn7;
        *var_gpe_dn8_slot = var_gpe_dn8;
        *var_gpe_dn9_slot = var_gpe_dn9;
        *var_guard96_slot = var_guard96;
        *var_guard97_slot = var_guard97;
        *var_gwe_slot = var_gwe;
        *var_lambda_le_slot = var_lambda_le;
        *var_np_i_slot = var_np_i;
        *var_np_i_dn4_slot = var_np_i_dn4;
        *var_np_i_dn6_slot = var_np_i_dn6;
        *var_np_i_dn7_slot = var_np_i_dn7;
        *var_np_i_dn8_slot = var_np_i_dn8;
        *var_np_i_dn9_slot = var_np_i_dn9;
        *var_nsddc_i_slot = var_nsddc_i;
        *var_one_m_xge_slot = var_one_m_xge;
        *var_pnce_i_slot = var_pnce_i;
        *var_pnce_p_slot = var_pnce_p;
        *var_psce1_i_slot = var_psce1_i;
        *var_psce2_i_slot = var_psce2_i;
        *var_psce_p_slot = var_psce_p;
        *var_pscedlb_i_slot = var_pscedlb_i;
        *var_stcf_i_slot = var_stcf_i;
        *var_stcf_i_dn4_slot = var_stcf_i_dn4;
        *var_stcf_i_dn6_slot = var_stcf_i_dn6;
        *var_stcf_i_dn7_slot = var_stcf_i_dn7;
        *var_stcf_i_dn8_slot = var_stcf_i_dn8;
        *var_stcf_i_dn9_slot = var_stcf_i_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_ge: f64,
        var_ge_dn4: f64,
        var_ge_dn6: f64,
        var_ge_dn7: f64,
        var_ge_dn8: f64,
        var_ge_dn9: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alpb_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_csbi_i_slot: &mut f64,
        var_csfi_i_slot: &mut f64,
        var_csthr_i_slot: &mut f64,
        var_csthrb_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_fnovinv_t_slot: &mut f64,
        var_fnovinvd_t_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_iginv_t_slot: &mut f64,
        var_igovinv_t_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsig_i_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_thersg_i_slot: &mut f64,
        var_thesat1_i_slot: &mut f64,
        var_thesat2_i_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vpg_i_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcorb_i_slot: &mut f64,
    ) {
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alpb_i: f64 = *var_alpb_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_csbi_i: f64 = *var_csbi_i_slot;
        let mut var_csfi_i: f64 = *var_csfi_i_slot;
        let mut var_csthr_i: f64 = *var_csthr_i_slot;
        let mut var_csthrb_i: f64 = *var_csthrb_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_fnovinv_t: f64 = *var_fnovinv_t_slot;
        let mut var_fnovinvd_t: f64 = *var_fnovinvd_t_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_iginv_t: f64 = *var_iginv_t_slot;
        let mut var_igovinv_t: f64 = *var_igovinv_t_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsig_i: f64 = *var_rsig_i_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_thersg_i: f64 = *var_thersg_i_slot;
        let mut var_thesat1_i: f64 = *var_thesat1_i_slot;
        let mut var_thesat2_i: f64 = *var_thesat2_i_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vpg_i: f64 = *var_vpg_i_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcorb_i: f64 = *var_xcorb_i_slot;

        let (assign2920_e2647,) = {
    if (var_guard83 == 0.0) {
        let assign2920_e2631: f64 = (p.p256 * var_ile);
        let assign2920_e2632: f64 = (1.0 + assign2920_e2631);
        let assign2920_e2633: f64 = (p.p255 * assign2920_e2632);
        let assign2920_e2637: f64 = (p.p257 * var_iwe);
        let assign2920_e2638: f64 = (1.0 + assign2920_e2637);
        let assign2920_e2639: f64 = (assign2920_e2633 * assign2920_e2638);
        let assign2920_e2643: f64 = (p.p258 * var_iae);
        let assign2920_e2644: f64 = (1.0 + assign2920_e2643);
        let assign2920_e2645: f64 = (assign2920_e2639 * assign2920_e2644);
        (assign2920_e2645,)
    } else {
        (var_stbet_i,)
    }
};
        var_stbet_i = assign2920_e2647;

        let (assign2930_e2670,) = {
    if (var_guard83 == 0.0) {
        let assign2930_e2654: f64 = (var_ile).powf(p.p261);
        let assign2930_e2655: f64 = (p.p260 * assign2930_e2654);
        let assign2930_e2656: f64 = (p.p259 + assign2930_e2655);
        let assign2930_e2660: f64 = (p.p262 * var_iwe);
        let assign2930_e2661: f64 = (1.0 + assign2930_e2660);
        let assign2930_e2662: f64 = (assign2930_e2656 * assign2930_e2661);
        let assign2930_e2666: f64 = (p.p263 * var_iae);
        let assign2930_e2667: f64 = (1.0 + assign2930_e2666);
        let assign2930_e2668: f64 = (assign2930_e2662 * assign2930_e2667);
        (assign2930_e2668,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign2930_e2670;

        let (assign2940_e2677,) = {
    if (var_guard83 == 0.0) {
        let assign2940_e2675: f64 = (var_cs_p).max(0.0);
        (assign2940_e2675,)
    } else {
        (var_cs_t,)
    }
};
        var_cs_t = assign2940_e2677;

        let (assign2950_e2682,) = {
    if (var_guard83 == 0.0) {
        (p.p264,)
    } else {
        (var_csfi_i,)
    }
};
        var_csfi_i = assign2950_e2682;

        let (assign2960_e2687,) = {
    if (var_guard83 == 0.0) {
        (p.p265,)
    } else {
        (var_csbi_i,)
    }
};
        var_csbi_i = assign2960_e2687;

        let (assign2970_e2710,) = {
    if (var_guard83 == 0.0) {
        let assign2970_e2694: f64 = (p.p267 * var_ile);
        let assign2970_e2695: f64 = (1.0 + assign2970_e2694);
        let assign2970_e2696: f64 = (p.p266 * assign2970_e2695);
        let assign2970_e2700: f64 = (p.p268 * var_iwe);
        let assign2970_e2701: f64 = (1.0 + assign2970_e2700);
        let assign2970_e2702: f64 = (assign2970_e2696 * assign2970_e2701);
        let assign2970_e2706: f64 = (p.p269 * var_iae);
        let assign2970_e2707: f64 = (1.0 + assign2970_e2706);
        let assign2970_e2708: f64 = (assign2970_e2702 * assign2970_e2707);
        (assign2970_e2708,)
    } else {
        (var_stcs_i,)
    }
};
        var_stcs_i = assign2970_e2710;

        let (assign2980_e2715,) = {
    if (var_guard83 == 0.0) {
        (p.p270,)
    } else {
        (var_thecs_t,)
    }
};
        var_thecs_t = assign2980_e2715;

        let (assign2990_e2720,) = {
    if (var_guard83 == 0.0) {
        (p.p271,)
    } else {
        (var_stthecs_i,)
    }
};
        var_stthecs_i = assign2990_e2720;

        let (assign3000_e2725,) = {
    if (var_guard83 == 0.0) {
        (p.p272,)
    } else {
        (var_csthr_i,)
    }
};
        var_csthr_i = assign3000_e2725;

        let (assign3010_e2730,) = {
    if (var_guard83 == 0.0) {
        (p.p273,)
    } else {
        (var_csthrb_i,)
    }
};
        var_csthrb_i = assign3010_e2730;

        let (assign3020_e2735,) = {
    if (var_guard83 == 0.0) {
        (p.p274,)
    } else {
        (var_mue_t,)
    }
};
        var_mue_t = assign3020_e2735;

        let (assign3030_e2740,) = {
    if (var_guard83 == 0.0) {
        (p.p275,)
    } else {
        (var_stmue_i,)
    }
};
        var_stmue_i = assign3030_e2740;

        let (assign3040_e2745,) = {
    if (var_guard83 == 0.0) {
        (p.p276,)
    } else {
        (var_themu_t,)
    }
};
        var_themu_t = assign3040_e2745;

        let (assign3050_e2750,) = {
    if (var_guard83 == 0.0) {
        (p.p277,)
    } else {
        (var_stthemu_i,)
    }
};
        var_stthemu_i = assign3050_e2750;

        let (assign3060_e2773,) = {
    if (var_guard83 == 0.0) {
        let assign3060_e2757: f64 = (var_ile).powf(p.p280);
        let assign3060_e2758: f64 = (p.p279 * assign3060_e2757);
        let assign3060_e2759: f64 = (p.p278 + assign3060_e2758);
        let assign3060_e2763: f64 = (p.p281 * var_iwe);
        let assign3060_e2764: f64 = (1.0 + assign3060_e2763);
        let assign3060_e2765: f64 = (assign3060_e2759 * assign3060_e2764);
        let assign3060_e2769: f64 = (p.p282 * var_iae);
        let assign3060_e2770: f64 = (1.0 + assign3060_e2769);
        let assign3060_e2771: f64 = (assign3060_e2765 * assign3060_e2770);
        (assign3060_e2771,)
    } else {
        (var_xcor_t,)
    }
};
        var_xcor_t = assign3060_e2773;

        let (assign3070_e2778,) = {
    if (var_guard83 == 0.0) {
        (p.p283,)
    } else {
        (var_xcorb_i,)
    }
};
        var_xcorb_i = assign3070_e2778;

        let (assign3080_e2783,) = {
    if (var_guard83 == 0.0) {
        (p.p284,)
    } else {
        (var_stxcor_i,)
    }
};
        var_stxcor_i = assign3080_e2783;

        let (assign3090_e2788,) = {
    if (var_guard83 == 0.0) {
        (p.p285,)
    } else {
        (var_feta_i,)
    }
};
        var_feta_i = assign3090_e2788;

        let (assign3100_e2801,) = {
    if (var_guard83 == 0.0) {
        let assign3100_e2793: f64 = (p.p286 * var_iwe);
        let assign3100_e2797: f64 = (p.p287 * var_iwe);
        let assign3100_e2798: f64 = (1.0 + assign3100_e2797);
        let assign3100_e2799: f64 = (assign3100_e2793 * assign3100_e2798);
        (assign3100_e2799,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign3100_e2801;

        let (assign3110_e2808,) = {
    if (var_guard83 == 0.0) {
        let assign3110_e2806: f64 = (var_rs_p).max(0.0);
        (assign3110_e2806,)
    } else {
        (var_rs_t,)
    }
};
        var_rs_t = assign3110_e2808;

        let (assign3120_e2813,) = {
    if (var_guard83 == 0.0) {
        (p.p288,)
    } else {
        (var_rsig_i,)
    }
};
        var_rsig_i = assign3120_e2813;

        let (assign3130_e2818,) = {
    if (var_guard83 == 0.0) {
        (p.p289,)
    } else {
        (var_strs_i,)
    }
};
        var_strs_i = assign3130_e2818;

        let (assign3140_e2823,) = {
    if (var_guard83 == 0.0) {
        (p.p290,)
    } else {
        (var_rsg_i,)
    }
};
        var_rsg_i = assign3140_e2823;

        let (assign3150_e2828,) = {
    if (var_guard83 == 0.0) {
        (p.p291,)
    } else {
        (var_thersg_i,)
    }
};
        var_thersg_i = assign3150_e2828;

        let (assign3160_e2833,) = {
    if (var_guard83 == 0.0) {
        (p.p292,)
    } else {
        (var_rsb_i,)
    }
};
        var_rsb_i = assign3160_e2833;

        let (assign3170_e2858, assign3170_e2858_d_n4, assign3170_e2858_d_n6, assign3170_e2858_d_n7, assign3170_e2858_d_n8, assign3170_e2858_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3170_e2841: f64 = (var_ile).powf(p.p295);
        let assign3170_e2842: f64 = (p.p294 * assign3170_e2841);
        let assign3170_e2843: f64 = (p.p293 + assign3170_e2842);
        let assign3170_e2844: f64 = (var_ge * assign3170_e2843);
        let assign3170_e2848: f64 = (p.p296 * var_iwe);
        let assign3170_e2849: f64 = (1.0 + assign3170_e2848);
        let assign3170_e2850: f64 = (assign3170_e2844 * assign3170_e2849);
        let assign3170_e2854: f64 = (p.p297 * var_iae);
        let assign3170_e2855: f64 = (1.0 + assign3170_e2854);
        let assign3170_e2856: f64 = (assign3170_e2850 * assign3170_e2855);
        (assign3170_e2856, (((var_ge_dn4 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn6 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn7 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn8 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855), (((var_ge_dn9 * assign3170_e2843) * assign3170_e2849) * assign3170_e2855),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign3170_e2858;
        var_thesat_p_dn4 = assign3170_e2858_d_n4;
        var_thesat_p_dn6 = assign3170_e2858_d_n6;
        var_thesat_p_dn7 = assign3170_e2858_d_n7;
        var_thesat_p_dn8 = assign3170_e2858_d_n8;
        var_thesat_p_dn9 = assign3170_e2858_d_n9;

        let (assign3180_e2865, assign3180_e2865_d_n4, assign3180_e2865_d_n6, assign3180_e2865_d_n7, assign3180_e2865_d_n8, assign3180_e2865_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3180_e2863: f64 = (var_thesat_p).max(0.0);
        (assign3180_e2863, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign3180_e2865;
        var_thesat_t_dn4 = assign3180_e2865_d_n4;
        var_thesat_t_dn6 = assign3180_e2865_d_n6;
        var_thesat_t_dn7 = assign3180_e2865_d_n7;
        var_thesat_t_dn8 = assign3180_e2865_d_n8;
        var_thesat_t_dn9 = assign3180_e2865_d_n9;

        let (assign3190_e2888,) = {
    if (var_guard83 == 0.0) {
        let assign3190_e2872: f64 = (p.p299 * var_ile);
        let assign3190_e2873: f64 = (1.0 + assign3190_e2872);
        let assign3190_e2874: f64 = (p.p298 * assign3190_e2873);
        let assign3190_e2878: f64 = (p.p300 * var_iwe);
        let assign3190_e2879: f64 = (1.0 + assign3190_e2878);
        let assign3190_e2880: f64 = (assign3190_e2874 * assign3190_e2879);
        let assign3190_e2884: f64 = (p.p301 * var_iae);
        let assign3190_e2885: f64 = (1.0 + assign3190_e2884);
        let assign3190_e2886: f64 = (assign3190_e2880 * assign3190_e2885);
        (assign3190_e2886,)
    } else {
        (var_stthesat_i,)
    }
};
        var_stthesat_i = assign3190_e2888;

        let (assign3200_e2893,) = {
    if (var_guard83 == 0.0) {
        (p.p302,)
    } else {
        (var_thesat1_i,)
    }
};
        var_thesat1_i = assign3200_e2893;

        let (assign3210_e2898,) = {
    if (var_guard83 == 0.0) {
        (p.p303,)
    } else {
        (var_thesat2_i,)
    }
};
        var_thesat2_i = assign3210_e2898;

        let (assign3220_e2919,) = {
    if (var_guard83 == 0.0) {
        let assign3220_e2906: f64 = (var_ile).powf(p.p306);
        let assign3220_e2907: f64 = (p.p305 * assign3220_e2906);
        let assign3220_e2912: f64 = (var_ile).powf(p.p308);
        let assign3220_e2913: f64 = (p.p307 * assign3220_e2912);
        let assign3220_e2914: f64 = (1.0 + assign3220_e2913);
        let assign3220_e2915: f64 = (assign3220_e2907 / assign3220_e2914);
        let assign3220_e2916: f64 = (1.0 + assign3220_e2915);
        let assign3220_e2917: f64 = (p.p304 / assign3220_e2916);
        (assign3220_e2917,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign3220_e2919;

        let (assign3230_e2928,) = {
    if (var_guard83 == 0.0) {
        let assign3230_e2924: f64 = (var_ax_p).max(1.0);
        let assign3230_e2926: f64 = (assign3230_e2924).min(16.0);
        (assign3230_e2926,)
    } else {
        (var_ax_i,)
    }
};
        var_ax_i = assign3230_e2928;

        let (assign3240_e2951,) = {
    if (var_guard83 == 0.0) {
        let assign3240_e2934: f64 = (var_ile).powf(p.p310);
        let assign3240_e2935: f64 = (p.p309 * assign3240_e2934);
        let assign3240_e2939: f64 = (p.p313 * var_iwe);
        let assign3240_e2940: f64 = (1.0 + assign3240_e2939);
        let assign3240_e2941: f64 = (assign3240_e2935 * assign3240_e2940);
        let assign3240_e2946: f64 = (var_ile).powf(p.p312);
        let assign3240_e2947: f64 = (p.p311 * assign3240_e2946);
        let assign3240_e2948: f64 = (1.0 + assign3240_e2947);
        let assign3240_e2949: f64 = (assign3240_e2941 / assign3240_e2948);
        (assign3240_e2949,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign3240_e2951;

        let (assign3250_e2958,) = {
    if (var_guard83 == 0.0) {
        let assign3250_e2956: f64 = (var_alp_p).max(0.0);
        (assign3250_e2956,)
    } else {
        (var_alp_i,)
    }
};
        var_alp_i = assign3250_e2958;

        let (assign3260_e2981,) = {
    if (var_guard83 == 0.0) {
        let assign3260_e2964: f64 = (var_ile).powf(p.p315);
        let assign3260_e2965: f64 = (p.p314 * assign3260_e2964);
        let assign3260_e2969: f64 = (p.p318 * var_iwe);
        let assign3260_e2970: f64 = (1.0 + assign3260_e2969);
        let assign3260_e2971: f64 = (assign3260_e2965 * assign3260_e2970);
        let assign3260_e2976: f64 = (var_ile).powf(p.p317);
        let assign3260_e2977: f64 = (p.p316 * assign3260_e2976);
        let assign3260_e2978: f64 = (1.0 + assign3260_e2977);
        let assign3260_e2979: f64 = (assign3260_e2971 / assign3260_e2978);
        (assign3260_e2979,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign3260_e2981;

        let (assign3270_e2988,) = {
    if (var_guard83 == 0.0) {
        let assign3270_e2986: f64 = (var_alp1_p).max(0.0);
        (assign3270_e2986,)
    } else {
        (var_alp1_i,)
    }
};
        var_alp1_i = assign3270_e2988;

        let (assign3280_e2993,) = {
    if (var_guard83 == 0.0) {
        (p.p319,)
    } else {
        (var_alpb_i,)
    }
};
        var_alpb_i = assign3280_e2993;

        let (assign3290_e2998,) = {
    if (var_guard83 == 0.0) {
        (p.p320,)
    } else {
        (var_vp_i,)
    }
};
        var_vp_i = assign3290_e2998;

        let (assign3300_e3003,) = {
    if (var_guard83 == 0.0) {
        (p.p321,)
    } else {
        (var_vpg_i,)
    }
};
        var_vpg_i = assign3300_e3003;

        let (assign3310_e3008,) = {
    if (var_guard83 == 0.0) {
        (p.p322,)
    } else {
        (var_gco_i,)
    }
};
        var_gco_i = assign3310_e3008;

        let (assign3320_e3015,) = {
    if (var_guard83 == 0.0) {
        let assign3320_e3013: f64 = (p.p323 / var_iae);
        (assign3320_e3013,)
    } else {
        (var_iginv_t,)
    }
};
        var_iginv_t = assign3320_e3015;

        let (assign3330_e3022,) = {
    if (var_guard83 == 0.0) {
        let assign3330_e3020: f64 = (p.p324 / var_iwe);
        (assign3330_e3020,)
    } else {
        (var_igovinv_t,)
    }
};
        var_igovinv_t = assign3330_e3022;

        let (assign3340_e3029,) = {
    if (var_guard83 == 0.0) {
        let assign3340_e3027: f64 = (p.p325 / var_iwe);
        (assign3340_e3027,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign3340_e3029;

        let (assign3350_e3036,) = {
    if (var_guard83 == 0.0) {
        let assign3350_e3034: f64 = (p.p339 / var_iwe);
        (assign3350_e3034,)
    } else {
        (var_fnovinv_t,)
    }
};
        var_fnovinv_t = assign3350_e3036;

        let (assign3360_e3043,) = {
    if (var_guard83 == 0.0) {
        let assign3360_e3041: f64 = (p.p340 / var_iwe);
        (assign3360_e3041,)
    } else {
        (var_fnovinvd_t,)
    }
};
        var_fnovinvd_t = assign3360_e3043;

        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp_i_slot = var_alp_i;
        *var_alp_p_slot = var_alp_p;
        *var_alpb_i_slot = var_alpb_i;
        *var_ax_i_slot = var_ax_i;
        *var_ax_p_slot = var_ax_p;
        *var_cs_p_slot = var_cs_p;
        *var_cs_t_slot = var_cs_t;
        *var_csbi_i_slot = var_csbi_i;
        *var_csfi_i_slot = var_csfi_i;
        *var_csthr_i_slot = var_csthr_i;
        *var_csthrb_i_slot = var_csthrb_i;
        *var_feta_i_slot = var_feta_i;
        *var_fnovinv_t_slot = var_fnovinv_t;
        *var_fnovinvd_t_slot = var_fnovinvd_t;
        *var_gco_i_slot = var_gco_i;
        *var_iginv_t_slot = var_iginv_t;
        *var_igovinv_t_slot = var_igovinv_t;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_mue_t_slot = var_mue_t;
        *var_rs_p_slot = var_rs_p;
        *var_rs_t_slot = var_rs_t;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsig_i_slot = var_rsig_i;
        *var_stbet_i_slot = var_stbet_i;
        *var_stcs_i_slot = var_stcs_i;
        *var_stmue_i_slot = var_stmue_i;
        *var_strs_i_slot = var_strs_i;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_thecs_t_slot = var_thecs_t;
        *var_themu_t_slot = var_themu_t;
        *var_thersg_i_slot = var_thersg_i;
        *var_thesat1_i_slot = var_thesat1_i;
        *var_thesat2_i_slot = var_thesat2_i;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_vp_i_slot = var_vp_i;
        *var_vpg_i_slot = var_vpg_i;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcorb_i_slot = var_xcorb_i;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_we: f64,
        var_a1_i_slot: &mut f64,
        var_a1_p_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_cic1edge_i_slot: &mut f64,
        var_cic2edge_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_dgidl_i_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_gc2ch_i_slot: &mut f64,
        var_gc2ovacc_i_slot: &mut f64,
        var_gc2ovinv_i_slot: &mut f64,
        var_gc3ch_i_slot: &mut f64,
        var_gc3ovacc_i_slot: &mut f64,
        var_gc3ovinv_i_slot: &mut f64,
        var_gcdov_i_slot: &mut f64,
        var_gcovinvfn_i_slot: &mut f64,
        var_gcvdov_i_slot: &mut f64,
        var_igovacc_t_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_niginv_i_slot: &mut f64,
        var_psce1edge_i_slot: &mut f64,
        var_psce1edge_i_dn4_slot: &mut f64,
        var_psce1edge_i_dn6_slot: &mut f64,
        var_psce1edge_i_dn7_slot: &mut f64,
        var_psce1edge_i_dn8_slot: &mut f64,
        var_psce1edge_i_dn9_slot: &mut f64,
        var_psce2edge_i_slot: &mut f64,
        var_psce2edge_i_dn4_slot: &mut f64,
        var_psce2edge_i_dn6_slot: &mut f64,
        var_psce2edge_i_dn7_slot: &mut f64,
        var_psce2edge_i_dn8_slot: &mut f64,
        var_psce2edge_i_dn9_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stigfn_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vfb1edge_t_slot: &mut f64,
        var_vfb1edge_t_dn4_slot: &mut f64,
        var_vfb1edge_t_dn6_slot: &mut f64,
        var_vfb1edge_t_dn7_slot: &mut f64,
        var_vfb1edge_t_dn8_slot: &mut f64,
        var_vfb1edge_t_dn9_slot: &mut f64,
        var_vfb2edge_t_slot: &mut f64,
        var_we_edge_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_cic1edge_i: f64 = *var_cic1edge_i_slot;
        let mut var_cic2edge_i: f64 = *var_cic2edge_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_dgidl_i: f64 = *var_dgidl_i_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_gc2ch_i: f64 = *var_gc2ch_i_slot;
        let mut var_gc2ovacc_i: f64 = *var_gc2ovacc_i_slot;
        let mut var_gc2ovinv_i: f64 = *var_gc2ovinv_i_slot;
        let mut var_gc3ch_i: f64 = *var_gc3ch_i_slot;
        let mut var_gc3ovacc_i: f64 = *var_gc3ovacc_i_slot;
        let mut var_gc3ovinv_i: f64 = *var_gc3ovinv_i_slot;
        let mut var_gcdov_i: f64 = *var_gcdov_i_slot;
        let mut var_gcovinvfn_i: f64 = *var_gcovinvfn_i_slot;
        let mut var_gcvdov_i: f64 = *var_gcvdov_i_slot;
        let mut var_igovacc_t: f64 = *var_igovacc_t_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_niginv_i: f64 = *var_niginv_i_slot;
        let mut var_psce1edge_i: f64 = *var_psce1edge_i_slot;
        let mut var_psce1edge_i_dn4: f64 = *var_psce1edge_i_dn4_slot;
        let mut var_psce1edge_i_dn6: f64 = *var_psce1edge_i_dn6_slot;
        let mut var_psce1edge_i_dn7: f64 = *var_psce1edge_i_dn7_slot;
        let mut var_psce1edge_i_dn8: f64 = *var_psce1edge_i_dn8_slot;
        let mut var_psce1edge_i_dn9: f64 = *var_psce1edge_i_dn9_slot;
        let mut var_psce2edge_i: f64 = *var_psce2edge_i_slot;
        let mut var_psce2edge_i_dn4: f64 = *var_psce2edge_i_dn4_slot;
        let mut var_psce2edge_i_dn6: f64 = *var_psce2edge_i_dn6_slot;
        let mut var_psce2edge_i_dn7: f64 = *var_psce2edge_i_dn7_slot;
        let mut var_psce2edge_i_dn8: f64 = *var_psce2edge_i_dn8_slot;
        let mut var_psce2edge_i_dn9: f64 = *var_psce2edge_i_dn9_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stigfn_i: f64 = *var_stigfn_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vfb1edge_t: f64 = *var_vfb1edge_t_slot;
        let mut var_vfb1edge_t_dn4: f64 = *var_vfb1edge_t_dn4_slot;
        let mut var_vfb1edge_t_dn6: f64 = *var_vfb1edge_t_dn6_slot;
        let mut var_vfb1edge_t_dn7: f64 = *var_vfb1edge_t_dn7_slot;
        let mut var_vfb1edge_t_dn8: f64 = *var_vfb1edge_t_dn8_slot;
        let mut var_vfb1edge_t_dn9: f64 = *var_vfb1edge_t_dn9_slot;
        let mut var_vfb2edge_t: f64 = *var_vfb2edge_t_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;

        let (assign3370_e3050,) = {
    if (var_guard83 == 0.0) {
        let assign3370_e3048: f64 = (p.p326 / var_iwe);
        (assign3370_e3048,)
    } else {
        (var_igovacc_t,)
    }
};
        var_igovacc_t = assign3370_e3050;

        let (assign3380_e3057,) = {
    if (var_guard83 == 0.0) {
        let assign3380_e3055: f64 = (p.p327 / var_iwe);
        (assign3380_e3055,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign3380_e3057;

        let (assign3390_e3062,) = {
    if (var_guard83 == 0.0) {
        (p.p328,)
    } else {
        (var_stig_i,)
    }
};
        var_stig_i = assign3390_e3062;

        let (assign3400_e3067,) = {
    if (var_guard83 == 0.0) {
        (p.p342,)
    } else {
        (var_stigfn_i,)
    }
};
        var_stigfn_i = assign3400_e3067;

        let (assign3410_e3072,) = {
    if (var_guard83 == 0.0) {
        (p.p329,)
    } else {
        (var_gc2ch_i,)
    }
};
        var_gc2ch_i = assign3410_e3072;

        let (assign3420_e3077,) = {
    if (var_guard83 == 0.0) {
        (p.p330,)
    } else {
        (var_gc3ch_i,)
    }
};
        var_gc3ch_i = assign3420_e3077;

        let (assign3430_e3082,) = {
    if (var_guard83 == 0.0) {
        (p.p331,)
    } else {
        (var_gc2ovinv_i,)
    }
};
        var_gc2ovinv_i = assign3430_e3082;

        let (assign3440_e3087,) = {
    if (var_guard83 == 0.0) {
        (p.p341,)
    } else {
        (var_gcovinvfn_i,)
    }
};
        var_gcovinvfn_i = assign3440_e3087;

        let (assign3450_e3092,) = {
    if (var_guard83 == 0.0) {
        (p.p332,)
    } else {
        (var_gc3ovinv_i,)
    }
};
        var_gc3ovinv_i = assign3450_e3092;

        let (assign3460_e3097,) = {
    if (var_guard83 == 0.0) {
        (p.p333,)
    } else {
        (var_gc2ovacc_i,)
    }
};
        var_gc2ovacc_i = assign3460_e3097;

        let (assign3470_e3102,) = {
    if (var_guard83 == 0.0) {
        (p.p334,)
    } else {
        (var_gc3ovacc_i,)
    }
};
        var_gc3ovacc_i = assign3470_e3102;

        let (assign3480_e3109,) = {
    if (var_guard83 == 0.0) {
        let assign3480_e3107: f64 = (p.p335 * var_ile);
        (assign3480_e3107,)
    } else {
        (var_gcdov_i,)
    }
};
        var_gcdov_i = assign3480_e3109;

        let (assign3490_e3114,) = {
    if (var_guard83 == 0.0) {
        (p.p336,)
    } else {
        (var_gcvdov_i,)
    }
};
        var_gcvdov_i = assign3490_e3114;

        let (assign3500_e3119,) = {
    if (var_guard83 == 0.0) {
        (p.p337,)
    } else {
        (var_chib_i,)
    }
};
        var_chib_i = assign3500_e3119;

        let (assign3510_e3124,) = {
    if (var_guard83 == 0.0) {
        (p.p338,)
    } else {
        (var_niginv_i,)
    }
};
        var_niginv_i = assign3510_e3124;

        let (assign3520_e3133,) = {
    if (var_guard83 == 0.0) {
        let assign3520_e3130: f64 = (p.p345 / var_iwe);
        let assign3520_e3131: f64 = (p.p343 + assign3520_e3130);
        (assign3520_e3131,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign3520_e3133;

        let (assign3530_e3140, assign3530_e3140_d_n4, assign3530_e3140_d_n6, assign3530_e3140_d_n7, assign3530_e3140_d_n8, assign3530_e3140_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3530_e3138: f64 = (var_agidl_p).max(0.0);
        (assign3530_e3138, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidl_i, var_agidl_i_dn4, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9,)
    }
};
        var_agidl_i = assign3530_e3140;
        var_agidl_i_dn4 = assign3530_e3140_d_n4;
        var_agidl_i_dn6 = assign3530_e3140_d_n6;
        var_agidl_i_dn7 = assign3530_e3140_d_n7;
        var_agidl_i_dn8 = assign3530_e3140_d_n8;
        var_agidl_i_dn9 = assign3530_e3140_d_n9;

        let (assign3540_e3149,) = {
    if (var_guard83 == 0.0) {
        let assign3540_e3146: f64 = (p.p346 / var_iwe);
        let assign3540_e3147: f64 = (p.p344 + assign3540_e3146);
        (assign3540_e3147,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign3540_e3149;

        let (assign3550_e3156, assign3550_e3156_d_n4, assign3550_e3156_d_n6, assign3550_e3156_d_n7, assign3550_e3156_d_n8, assign3550_e3156_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3550_e3154: f64 = (var_agidld_p).max(0.0);
        (assign3550_e3154, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidld_i, var_agidld_i_dn4, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9,)
    }
};
        var_agidld_i = assign3550_e3156;
        var_agidld_i_dn4 = assign3550_e3156_d_n4;
        var_agidld_i_dn6 = assign3550_e3156_d_n6;
        var_agidld_i_dn7 = assign3550_e3156_d_n7;
        var_agidld_i_dn8 = assign3550_e3156_d_n8;
        var_agidld_i_dn9 = assign3550_e3156_d_n9;

        let (assign3560_e3161,) = {
    if (var_guard83 == 0.0) {
        (p.p347,)
    } else {
        (var_bgidl_t,)
    }
};
        var_bgidl_t = assign3560_e3161;

        let (assign3570_e3166,) = {
    if (var_guard83 == 0.0) {
        (p.p348,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign3570_e3166;

        let (assign3580_e3171,) = {
    if (var_guard83 == 0.0) {
        (p.p349,)
    } else {
        (var_stbgidl_i,)
    }
};
        var_stbgidl_i = assign3580_e3171;

        let (assign3590_e3176,) = {
    if (var_guard83 == 0.0) {
        (p.p350,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign3590_e3176;

        let (assign3600_e3181,) = {
    if (var_guard83 == 0.0) {
        (p.p351,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign3600_e3181;

        let (assign3610_e3186,) = {
    if (var_guard83 == 0.0) {
        (p.p352,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign3610_e3186;

        let (assign3620_e3195,) = {
    if (var_guard83 == 0.0) {
        let assign3620_e3192: f64 = (p.p355 * var_ile);
        let assign3620_e3193: f64 = (p.p353 + assign3620_e3192);
        (assign3620_e3193,)
    } else {
        (var_dgidl_i,)
    }
};
        var_dgidl_i = assign3620_e3195;

        let (assign3630_e3204,) = {
    if (var_guard83 == 0.0) {
        let assign3630_e3201: f64 = (p.p356 * var_ile);
        let assign3630_e3202: f64 = (p.p354 + assign3630_e3201);
        (assign3630_e3202,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign3630_e3204;

        let (assign3640_e3221,) = {
    if (var_guard83 == 0.0) {
        let assign3640_e3211: f64 = (p.p389 * var_ile);
        let assign3640_e3212: f64 = (1.0 + assign3640_e3211);
        let assign3640_e3213: f64 = (p.p388 * assign3640_e3212);
        let assign3640_e3217: f64 = (p.p390 * var_iwe);
        let assign3640_e3218: f64 = (1.0 + assign3640_e3217);
        let assign3640_e3219: f64 = (assign3640_e3213 * assign3640_e3218);
        (assign3640_e3219,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign3640_e3221;

        let (assign3650_e3228,) = {
    if (var_guard83 == 0.0) {
        let assign3650_e3226: f64 = (var_a1_p).max(0.0);
        (assign3650_e3226,)
    } else {
        (var_a1_i,)
    }
};
        var_a1_i = assign3650_e3228;

        let (assign3660_e3233,) = {
    if (var_guard83 == 0.0) {
        (p.p391,)
    } else {
        (var_a2_t,)
    }
};
        var_a2_t = assign3660_e3233;

        let (assign3670_e3238,) = {
    if (var_guard83 == 0.0) {
        (p.p392,)
    } else {
        (var_sta2_i,)
    }
};
        var_sta2_i = assign3670_e3238;

        let (assign3680_e3255,) = {
    if (var_guard83 == 0.0) {
        let assign3680_e3245: f64 = (p.p394 * var_ile);
        let assign3680_e3246: f64 = (1.0 + assign3680_e3245);
        let assign3680_e3247: f64 = (p.p393 * assign3680_e3246);
        let assign3680_e3251: f64 = (p.p395 * var_iwe);
        let assign3680_e3252: f64 = (1.0 + assign3680_e3251);
        let assign3680_e3253: f64 = (assign3680_e3247 * assign3680_e3252);
        (assign3680_e3253,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign3680_e3255;

        let (assign3690_e3262,) = {
    if (var_guard83 == 0.0) {
        let assign3690_e3260: f64 = (var_a3_p).max(0.0);
        (assign3690_e3260,)
    } else {
        (var_a3_i,)
    }
};
        var_a3_i = assign3690_e3262;

        let (assign3700_e3273,) = {
    if (var_guard83 == 0.0) {
        let assign3700_e3267: f64 = (2.0 * p.p357);
        let assign3700_e3270: f64 = (p.p358 * var_we);
        let assign3700_e3271: f64 = (assign3700_e3267 + assign3700_e3270);
        (assign3700_e3271,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign3700_e3273;

        let (assign3710_e3278,) = {
    if (var_guard83 == 0.0) {
        (p.p359,)
    } else {
        (var_ctedge_i,)
    }
};
        var_ctedge_i = assign3710_e3278;

        let (assign3720_e3287, assign3720_e3287_d_n4, assign3720_e3287_d_n6, assign3720_e3287_d_n7, assign3720_e3287_d_n8, assign3720_e3287_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3720_e3284: f64 = (var_ile).powf(p.p362);
        let assign3720_e3285: f64 = (p.p361 * assign3720_e3284);
        (assign3720_e3285, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3720_e3287;
        var_temp_dn4 = assign3720_e3287_d_n4;
        var_temp_dn6 = assign3720_e3287_d_n6;
        var_temp_dn7 = assign3720_e3287_d_n7;
        var_temp_dn8 = assign3720_e3287_d_n8;
        var_temp_dn9 = assign3720_e3287_d_n9;

        let (assign3730_e3302, assign3730_e3302_d_n4, assign3730_e3302_d_n6, assign3730_e3302_d_n7, assign3730_e3302_d_n8, assign3730_e3302_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3730_e3292: f64 = (p.p360 + var_temp);
        let assign3730_e3295: f64 = (p.p363 * var_iwe);
        let assign3730_e3296: f64 = (assign3730_e3292 + assign3730_e3295);
        let assign3730_e3299: f64 = (p.p364 * var_iae);
        let assign3730_e3300: f64 = (assign3730_e3296 + assign3730_e3299);
        (assign3730_e3300, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfb1edge_t, var_vfb1edge_t_dn4, var_vfb1edge_t_dn6, var_vfb1edge_t_dn7, var_vfb1edge_t_dn8, var_vfb1edge_t_dn9,)
    }
};
        var_vfb1edge_t = assign3730_e3302;
        var_vfb1edge_t_dn4 = assign3730_e3302_d_n4;
        var_vfb1edge_t_dn6 = assign3730_e3302_d_n6;
        var_vfb1edge_t_dn7 = assign3730_e3302_d_n7;
        var_vfb1edge_t_dn8 = assign3730_e3302_d_n8;
        var_vfb1edge_t_dn9 = assign3730_e3302_d_n9;

        let (assign3740_e3307,) = {
    if (var_guard83 == 0.0) {
        (p.p365,)
    } else {
        (var_vfb2edge_t,)
    }
};
        var_vfb2edge_t = assign3740_e3307;

        let (assign3750_e3330,) = {
    if (var_guard83 == 0.0) {
        let assign3750_e3314: f64 = (p.p367 * var_ile);
        let assign3750_e3315: f64 = (1.0 + assign3750_e3314);
        let assign3750_e3316: f64 = (p.p366 * assign3750_e3315);
        let assign3750_e3320: f64 = (p.p368 * var_iwe);
        let assign3750_e3321: f64 = (1.0 + assign3750_e3320);
        let assign3750_e3322: f64 = (assign3750_e3316 * assign3750_e3321);
        let assign3750_e3326: f64 = (p.p369 * var_iae);
        let assign3750_e3327: f64 = (1.0 + assign3750_e3326);
        let assign3750_e3328: f64 = (assign3750_e3322 * assign3750_e3327);
        (assign3750_e3328,)
    } else {
        (var_stvfbedge_i,)
    }
};
        var_stvfbedge_i = assign3750_e3330;

        let (assign3760_e3335,) = {
    if (var_guard83 == 0.0) {
        (p.p370,)
    } else {
        (var_cic1edge_i,)
    }
};
        var_cic1edge_i = assign3760_e3335;

        let (assign3770_e3340,) = {
    if (var_guard83 == 0.0) {
        (p.p371,)
    } else {
        (var_cic2edge_i,)
    }
};
        var_cic2edge_i = assign3770_e3340;

        let (assign3780_e3357, assign3780_e3357_d_n4, assign3780_e3357_d_n6, assign3780_e3357_d_n7, assign3780_e3357_d_n8, assign3780_e3357_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3780_e3345: f64 = (p.p372 * 2.0);
        let assign3780_e3348: f64 = (var_lambda_le).powf(p.p373);
        let assign3780_e3349: f64 = (assign3780_e3345 * assign3780_e3348);
        let assign3780_e3353: f64 = (p.p374 * var_iwe);
        let assign3780_e3354: f64 = (1.0 + assign3780_e3353);
        let assign3780_e3355: f64 = (assign3780_e3349 * assign3780_e3354);
        (assign3780_e3355, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3780_e3357;
        var_temp_dn4 = assign3780_e3357_d_n4;
        var_temp_dn6 = assign3780_e3357_d_n6;
        var_temp_dn7 = assign3780_e3357_d_n7;
        var_temp_dn8 = assign3780_e3357_d_n8;
        var_temp_dn9 = assign3780_e3357_d_n9;

        let (assign3790_e3366, assign3790_e3366_d_n4, assign3790_e3366_d_n6, assign3790_e3366_d_n7, assign3790_e3366_d_n8, assign3790_e3366_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3790_e3362: f64 = (var_temp).max(0.0);
        let assign3790_e3364: f64 = (assign3790_e3362).min(5.0);
        (assign3790_e3364, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3790_e3362 <= 5.0 { if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_psce1edge_i, var_psce1edge_i_dn4, var_psce1edge_i_dn6, var_psce1edge_i_dn7, var_psce1edge_i_dn8, var_psce1edge_i_dn9,)
    }
};
        var_psce1edge_i = assign3790_e3366;
        var_psce1edge_i_dn4 = assign3790_e3366_d_n4;
        var_psce1edge_i_dn6 = assign3790_e3366_d_n6;
        var_psce1edge_i_dn7 = assign3790_e3366_d_n7;
        var_psce1edge_i_dn8 = assign3790_e3366_d_n8;
        var_psce1edge_i_dn9 = assign3790_e3366_d_n9;

        let (assign3800_e3377, assign3800_e3377_d_n4, assign3800_e3377_d_n6, assign3800_e3377_d_n7, assign3800_e3377_d_n8, assign3800_e3377_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3800_e3371: f64 = (p.p375 * var_psce1edge_i);
        let assign3800_e3373: f64 = (assign3800_e3371 * var_tox2_i);
        let assign3800_e3375: f64 = (assign3800_e3373 / var_tox1_i);
        (assign3800_e3375, (((p.p375 * var_psce1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p375 * var_psce1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_psce2edge_i, var_psce2edge_i_dn4, var_psce2edge_i_dn6, var_psce2edge_i_dn7, var_psce2edge_i_dn8, var_psce2edge_i_dn9,)
    }
};
        var_psce2edge_i = assign3800_e3377;
        var_psce2edge_i_dn4 = assign3800_e3377_d_n4;
        var_psce2edge_i_dn6 = assign3800_e3377_d_n6;
        var_psce2edge_i_dn7 = assign3800_e3377_d_n7;
        var_psce2edge_i_dn8 = assign3800_e3377_d_n8;
        var_psce2edge_i_dn9 = assign3800_e3377_d_n9;

        let (assign3810_e3390, assign3810_e3390_d_n4, assign3810_e3390_d_n6, assign3810_e3390_d_n7, assign3810_e3390_d_n8, assign3810_e3390_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3810_e3382: f64 = (var_lambda_le).powf(p.p377);
        let assign3810_e3386: f64 = (p.p378 * var_iwe);
        let assign3810_e3387: f64 = (1.0 + assign3810_e3386);
        let assign3810_e3388: f64 = (assign3810_e3382 * assign3810_e3387);
        (assign3810_e3388, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3810_e3390;
        var_temp_dn4 = assign3810_e3390_d_n4;
        var_temp_dn6 = assign3810_e3390_d_n6;
        var_temp_dn7 = assign3810_e3390_d_n7;
        var_temp_dn8 = assign3810_e3390_d_n8;
        var_temp_dn9 = assign3810_e3390_d_n9;

        *var_a1_i_slot = var_a1_i;
        *var_a1_p_slot = var_a1_p;
        *var_a2_t_slot = var_a2_t;
        *var_a3_i_slot = var_a3_i;
        *var_a3_p_slot = var_a3_p;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_agidld_p_slot = var_agidld_p;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_chib_i_slot = var_chib_i;
        *var_cic1edge_i_slot = var_cic1edge_i;
        *var_cic2edge_i_slot = var_cic2edge_i;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_dgidl_i_slot = var_dgidl_i;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_gc2ch_i_slot = var_gc2ch_i;
        *var_gc2ovacc_i_slot = var_gc2ovacc_i;
        *var_gc2ovinv_i_slot = var_gc2ovinv_i;
        *var_gc3ch_i_slot = var_gc3ch_i;
        *var_gc3ovacc_i_slot = var_gc3ovacc_i;
        *var_gc3ovinv_i_slot = var_gc3ovinv_i;
        *var_gcdov_i_slot = var_gcdov_i;
        *var_gcovinvfn_i_slot = var_gcovinvfn_i;
        *var_gcvdov_i_slot = var_gcvdov_i;
        *var_igovacc_t_slot = var_igovacc_t;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_niginv_i_slot = var_niginv_i;
        *var_psce1edge_i_slot = var_psce1edge_i;
        *var_psce1edge_i_dn4_slot = var_psce1edge_i_dn4;
        *var_psce1edge_i_dn6_slot = var_psce1edge_i_dn6;
        *var_psce1edge_i_dn7_slot = var_psce1edge_i_dn7;
        *var_psce1edge_i_dn8_slot = var_psce1edge_i_dn8;
        *var_psce1edge_i_dn9_slot = var_psce1edge_i_dn9;
        *var_psce2edge_i_slot = var_psce2edge_i;
        *var_psce2edge_i_dn4_slot = var_psce2edge_i_dn4;
        *var_psce2edge_i_dn6_slot = var_psce2edge_i_dn6;
        *var_psce2edge_i_dn7_slot = var_psce2edge_i_dn7;
        *var_psce2edge_i_dn8_slot = var_psce2edge_i_dn8;
        *var_psce2edge_i_dn9_slot = var_psce2edge_i_dn9;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stig_i_slot = var_stig_i;
        *var_stigfn_i_slot = var_stigfn_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vfb1edge_t_slot = var_vfb1edge_t;
        *var_vfb1edge_t_dn4_slot = var_vfb1edge_t_dn4;
        *var_vfb1edge_t_dn6_slot = var_vfb1edge_t_dn6;
        *var_vfb1edge_t_dn7_slot = var_vfb1edge_t_dn7;
        *var_vfb1edge_t_dn8_slot = var_vfb1edge_t_dn8;
        *var_vfb1edge_t_dn9_slot = var_vfb1edge_t_dn9;
        *var_vfb2edge_t_slot = var_vfb2edge_t;
        *var_we_edge_slot = var_we_edge;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_cf_p: f64,
        var_cf_p_dn4: f64,
        var_cf_p_dn6: f64,
        var_cf_p_dn7: f64,
        var_cf_p_dn8: f64,
        var_cf_p_dn9: f64,
        var_guard83: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_lecv: f64,
        var_lphy: f64,
        var_lphy_dn4: f64,
        var_lphy_dn6: f64,
        var_lphy_dn7: f64,
        var_lphy_dn8: f64,
        var_lphy_dn9: f64,
        var_psce1_i: f64,
        var_psce2_i: f64,
        var_thesat_p: f64,
        var_thesat_p_dn4: f64,
        var_thesat_p_dn6: f64,
        var_thesat_p_dn7: f64,
        var_thesat_p_dn8: f64,
        var_thesat_p_dn9: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_we_edge: f64,
        var_wecv: f64,
        var_wen: f64,
        var_alpac_i_slot: &mut f64,
        var_areaq_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_dn6_slot: &mut f64,
        var_betnedge_t_dn7_slot: &mut f64,
        var_betnedge_t_dn8_slot: &mut f64,
        var_betnedge_t_dn9_slot: &mut f64,
        var_cf1edge_i_slot: &mut f64,
        var_cf1edge_i_dn4_slot: &mut f64,
        var_cf1edge_i_dn6_slot: &mut f64,
        var_cf1edge_i_dn7_slot: &mut f64,
        var_cf1edge_i_dn8_slot: &mut f64,
        var_cf1edge_i_dn9_slot: &mut f64,
        var_cf2edge_i_slot: &mut f64,
        var_cf2edge_i_dn4_slot: &mut f64,
        var_cf2edge_i_dn6_slot: &mut f64,
        var_cf2edge_i_dn7_slot: &mut f64,
        var_cf2edge_i_dn8_slot: &mut f64,
        var_cf2edge_i_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_dn4_slot: &mut f64,
        var_cgbov_i_dn6_slot: &mut f64,
        var_cgbov_i_dn7_slot: &mut f64,
        var_cgbov_i_dn8_slot: &mut f64,
        var_cgbov_i_dn9_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_dn4_slot: &mut f64,
        var_cgbov_p_dn6_slot: &mut f64,
        var_cgbov_p_dn7_slot: &mut f64,
        var_cgbov_p_dn8_slot: &mut f64,
        var_cgbov_p_dn9_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_nsdac_i_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbacl2_i_slot: &mut f64,
        var_vfbacl_i_slot: &mut f64,
        var_vfbaclexp2_i_slot: &mut f64,
        var_vfbaclexp_i_slot: &mut f64,
        var_vfbaclw_i_slot: &mut f64,
        var_vfbaco_i_slot: &mut f64,
        var_vfbacw_i_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_areaq_i: f64 = *var_areaq_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_dn6: f64 = *var_betnedge_t_dn6_slot;
        let mut var_betnedge_t_dn7: f64 = *var_betnedge_t_dn7_slot;
        let mut var_betnedge_t_dn8: f64 = *var_betnedge_t_dn8_slot;
        let mut var_betnedge_t_dn9: f64 = *var_betnedge_t_dn9_slot;
        let mut var_cf1edge_i: f64 = *var_cf1edge_i_slot;
        let mut var_cf1edge_i_dn4: f64 = *var_cf1edge_i_dn4_slot;
        let mut var_cf1edge_i_dn6: f64 = *var_cf1edge_i_dn6_slot;
        let mut var_cf1edge_i_dn7: f64 = *var_cf1edge_i_dn7_slot;
        let mut var_cf1edge_i_dn8: f64 = *var_cf1edge_i_dn8_slot;
        let mut var_cf1edge_i_dn9: f64 = *var_cf1edge_i_dn9_slot;
        let mut var_cf2edge_i: f64 = *var_cf2edge_i_slot;
        let mut var_cf2edge_i_dn4: f64 = *var_cf2edge_i_dn4_slot;
        let mut var_cf2edge_i_dn6: f64 = *var_cf2edge_i_dn6_slot;
        let mut var_cf2edge_i_dn7: f64 = *var_cf2edge_i_dn7_slot;
        let mut var_cf2edge_i_dn8: f64 = *var_cf2edge_i_dn8_slot;
        let mut var_cf2edge_i_dn9: f64 = *var_cf2edge_i_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_dn4: f64 = *var_cgbov_i_dn4_slot;
        let mut var_cgbov_i_dn6: f64 = *var_cgbov_i_dn6_slot;
        let mut var_cgbov_i_dn7: f64 = *var_cgbov_i_dn7_slot;
        let mut var_cgbov_i_dn8: f64 = *var_cgbov_i_dn8_slot;
        let mut var_cgbov_i_dn9: f64 = *var_cgbov_i_dn9_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_dn4: f64 = *var_cgbov_p_dn4_slot;
        let mut var_cgbov_p_dn6: f64 = *var_cgbov_p_dn6_slot;
        let mut var_cgbov_p_dn7: f64 = *var_cgbov_p_dn7_slot;
        let mut var_cgbov_p_dn8: f64 = *var_cgbov_p_dn8_slot;
        let mut var_cgbov_p_dn9: f64 = *var_cgbov_p_dn9_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_nsdac_i: f64 = *var_nsdac_i_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbacl2_i: f64 = *var_vfbacl2_i_slot;
        let mut var_vfbacl_i: f64 = *var_vfbacl_i_slot;
        let mut var_vfbaclexp2_i: f64 = *var_vfbaclexp2_i_slot;
        let mut var_vfbaclexp_i: f64 = *var_vfbaclexp_i_slot;
        let mut var_vfbaclw_i: f64 = *var_vfbaclw_i_slot;
        let mut var_vfbaco_i: f64 = *var_vfbaco_i_slot;
        let mut var_vfbacw_i: f64 = *var_vfbacw_i_slot;

        let (assign3820_e3397, assign3820_e3397_d_n4, assign3820_e3397_d_n6, assign3820_e3397_d_n7, assign3820_e3397_d_n8, assign3820_e3397_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3820_e3395: f64 = (p.p376 * var_temp);
        (assign3820_e3395, (p.p376 * var_temp_dn4), (p.p376 * var_temp_dn6), (p.p376 * var_temp_dn7), (p.p376 * var_temp_dn8), (p.p376 * var_temp_dn9),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3820_e3397;
        var_temp_dn4 = assign3820_e3397_d_n4;
        var_temp_dn6 = assign3820_e3397_d_n6;
        var_temp_dn7 = assign3820_e3397_d_n7;
        var_temp_dn8 = assign3820_e3397_d_n8;
        var_temp_dn9 = assign3820_e3397_d_n9;

        let (assign3830_e3404, assign3830_e3404_d_n4, assign3830_e3404_d_n6, assign3830_e3404_d_n7, assign3830_e3404_d_n8, assign3830_e3404_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3830_e3402: f64 = (var_temp).max(0.0);
        (assign3830_e3402, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_cf1edge_i, var_cf1edge_i_dn4, var_cf1edge_i_dn6, var_cf1edge_i_dn7, var_cf1edge_i_dn8, var_cf1edge_i_dn9,)
    }
};
        var_cf1edge_i = assign3830_e3404;
        var_cf1edge_i_dn4 = assign3830_e3404_d_n4;
        var_cf1edge_i_dn6 = assign3830_e3404_d_n6;
        var_cf1edge_i_dn7 = assign3830_e3404_d_n7;
        var_cf1edge_i_dn8 = assign3830_e3404_d_n8;
        var_cf1edge_i_dn9 = assign3830_e3404_d_n9;

        let (assign3840_e3415, assign3840_e3415_d_n4, assign3840_e3415_d_n6, assign3840_e3415_d_n7, assign3840_e3415_d_n8, assign3840_e3415_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3840_e3409: f64 = (p.p379 * var_cf1edge_i);
        let assign3840_e3411: f64 = (assign3840_e3409 * var_tox2_i);
        let assign3840_e3413: f64 = (assign3840_e3411 / var_tox1_i);
        (assign3840_e3413, (((p.p379 * var_cf1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p379 * var_cf1edge_i_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2edge_i, var_cf2edge_i_dn4, var_cf2edge_i_dn6, var_cf2edge_i_dn7, var_cf2edge_i_dn8, var_cf2edge_i_dn9,)
    }
};
        var_cf2edge_i = assign3840_e3415;
        var_cf2edge_i_dn4 = assign3840_e3415_d_n4;
        var_cf2edge_i_dn6 = assign3840_e3415_d_n6;
        var_cf2edge_i_dn7 = assign3840_e3415_d_n7;
        var_cf2edge_i_dn8 = assign3840_e3415_d_n8;
        var_cf2edge_i_dn9 = assign3840_e3415_d_n9;

        let (assign3850_e3420,) = {
    if (var_guard83 == 0.0) {
        (p.p380,)
    } else {
        (var_cfdedge_i,)
    }
};
        var_cfdedge_i = assign3850_e3420;

        let (assign3860_e3439, assign3860_e3439_d_n4, assign3860_e3439_d_n6, assign3860_e3439_d_n7, assign3860_e3439_d_n8, assign3860_e3439_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3860_e3426: f64 = (p.p381 * p.p382);
        let assign3860_e3428: f64 = (assign3860_e3426 / var_le);
        let assign3860_e3431: f64 = (-var_le);
        let assign3860_e3433: f64 = (assign3860_e3431 / p.p382);
        let assign3860_e3434: f64 = (assign3860_e3433).exp();
        let assign3860_e3435: f64 = (1.0 - assign3860_e3434);
        let assign3860_e3436: f64 = (assign3860_e3428 * assign3860_e3435);
        let assign3860_e3437: f64 = (1.0 + assign3860_e3436);
        (assign3860_e3437, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3860_e3439;
        var_temp_dn4 = assign3860_e3439_d_n4;
        var_temp_dn6 = assign3860_e3439_d_n6;
        var_temp_dn7 = assign3860_e3439_d_n7;
        var_temp_dn8 = assign3860_e3439_d_n8;
        var_temp_dn9 = assign3860_e3439_d_n9;

        let (assign3870_e3446, assign3870_e3446_d_n4, assign3870_e3446_d_n6, assign3870_e3446_d_n7, assign3870_e3446_d_n8, assign3870_e3446_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3870_e3444: f64 = (var_temp).max(1e-15);
        (assign3870_e3444, if var_temp >= 1e-15 { var_temp_dn4 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn6 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn7 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn8 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign3870_e3446;
        var_temp_dn4 = assign3870_e3446_d_n4;
        var_temp_dn6 = assign3870_e3446_d_n6;
        var_temp_dn7 = assign3870_e3446_d_n7;
        var_temp_dn8 = assign3870_e3446_d_n8;
        var_temp_dn9 = assign3870_e3446_d_n9;

        let (assign3880_e3463, assign3880_e3463_d_n4, assign3880_e3463_d_n6, assign3880_e3463_d_n7, assign3880_e3463_d_n8, assign3880_e3463_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3880_e3451: f64 = (p.p244 * var_we_edge);
        let assign3880_e3454: f64 = (var_temp * var_le);
        let assign3880_e3455: f64 = (assign3880_e3451 / assign3880_e3454);
        let assign3880_e3459: f64 = (p.p383 * var_iwe);
        let assign3880_e3460: f64 = (1.0 + assign3880_e3459);
        let assign3880_e3461: f64 = (assign3880_e3455 * assign3880_e3460);
        (assign3880_e3461, ((-((assign3880_e3451 * (var_temp_dn4 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn6 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn7 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn8 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460), ((-((assign3880_e3451 * (var_temp_dn9 * var_le)) / (assign3880_e3454 * assign3880_e3454))) * assign3880_e3460),)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4, var_betnedge_t_dn6, var_betnedge_t_dn7, var_betnedge_t_dn8, var_betnedge_t_dn9,)
    }
};
        var_betnedge_t = assign3880_e3463;
        var_betnedge_t_dn4 = assign3880_e3463_d_n4;
        var_betnedge_t_dn6 = assign3880_e3463_d_n6;
        var_betnedge_t_dn7 = assign3880_e3463_d_n7;
        var_betnedge_t_dn8 = assign3880_e3463_d_n8;
        var_betnedge_t_dn9 = assign3880_e3463_d_n9;

        let (assign3890_e3482,) = {
    if (var_guard83 == 0.0) {
        let assign3890_e3469: f64 = (p.p385 * var_ile);
        let assign3890_e3470: f64 = (p.p384 + assign3890_e3469);
        let assign3890_e3473: f64 = (p.p386 * var_iwe);
        let assign3890_e3474: f64 = (assign3890_e3470 + assign3890_e3473);
        let assign3890_e3477: f64 = (p.p387 * var_ile);
        let assign3890_e3479: f64 = (assign3890_e3477 * var_iwe);
        let assign3890_e3480: f64 = (assign3890_e3474 + assign3890_e3479);
        (assign3890_e3480,)
    } else {
        (var_stbetedge_i,)
    }
};
        var_stbetedge_i = assign3890_e3482;

        let (assign3900_e3489,) = {
    if (var_guard83 == 0.0) {
        let assign3900_e3487: f64 = (var_wecv * var_lecv);
        (assign3900_e3487,)
    } else {
        (var_areaq_i,)
    }
};
        var_areaq_i = assign3900_e3489;

        let (assign3910_e3498, assign3910_e3498_d_n4, assign3910_e3498_d_n6, assign3910_e3498_d_n7, assign3910_e3498_d_n8, assign3910_e3498_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3910_e3495: f64 = (p.p397 * var_lphy);
        let assign3910_e3496: f64 = (p.p396 + assign3910_e3495);
        (assign3910_e3496, (p.p397 * var_lphy_dn4), (p.p397 * var_lphy_dn6), (p.p397 * var_lphy_dn7), (p.p397 * var_lphy_dn8), (p.p397 * var_lphy_dn9),)
    } else {
        (var_cgbov_p, var_cgbov_p_dn4, var_cgbov_p_dn6, var_cgbov_p_dn7, var_cgbov_p_dn8, var_cgbov_p_dn9,)
    }
};
        var_cgbov_p = assign3910_e3498;
        var_cgbov_p_dn4 = assign3910_e3498_d_n4;
        var_cgbov_p_dn6 = assign3910_e3498_d_n6;
        var_cgbov_p_dn7 = assign3910_e3498_d_n7;
        var_cgbov_p_dn8 = assign3910_e3498_d_n8;
        var_cgbov_p_dn9 = assign3910_e3498_d_n9;

        let (assign3920_e3505, assign3920_e3505_d_n4, assign3920_e3505_d_n6, assign3920_e3505_d_n7, assign3920_e3505_d_n8, assign3920_e3505_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign3920_e3503: f64 = (var_cgbov_p).max(0.0);
        (assign3920_e3503, if var_cgbov_p >= 0.0 { var_cgbov_p_dn4 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn6 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn7 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn8 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn9 } else { 0.0 },)
    } else {
        (var_cgbov_i, var_cgbov_i_dn4, var_cgbov_i_dn6, var_cgbov_i_dn7, var_cgbov_i_dn8, var_cgbov_i_dn9,)
    }
};
        var_cgbov_i = assign3920_e3505;
        var_cgbov_i_dn4 = assign3920_e3505_d_n4;
        var_cgbov_i_dn6 = assign3920_e3505_d_n6;
        var_cgbov_i_dn7 = assign3920_e3505_d_n7;
        var_cgbov_i_dn8 = assign3920_e3505_d_n8;
        var_cgbov_i_dn9 = assign3920_e3505_d_n9;

        let (assign3930_e3512,) = {
    if (var_guard83 == 0.0) {
        let assign3930_e3510: f64 = (p.p398 * 1000000.0);
        (assign3930_e3510,)
    } else {
        (var_nsdac_i,)
    }
};
        var_nsdac_i = assign3930_e3512;

        let (assign3940_e3521,) = {
    if (var_guard83 == 0.0) {
        let assign3940_e3517: f64 = (p.p399 * var_wecv);
        let assign3940_e3519: f64 = (assign3940_e3517 / var_wen);
        (assign3940_e3519,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign3940_e3521;

        let (assign3950_e3526,) = {
    if (var_guard83 == 0.0) {
        (p.p400,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign3950_e3526;

        let (assign3960_e3531, assign3960_e3531_d_n4, assign3960_e3531_d_n6, assign3960_e3531_d_n7, assign3960_e3531_d_n8, assign3960_e3531_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign3960_e3531;
        var_vfbac1_t_dn4 = assign3960_e3531_d_n4;
        var_vfbac1_t_dn6 = assign3960_e3531_d_n6;
        var_vfbac1_t_dn7 = assign3960_e3531_d_n7;
        var_vfbac1_t_dn8 = assign3960_e3531_d_n8;
        var_vfbac1_t_dn9 = assign3960_e3531_d_n9;

        let (assign3970_e3536, assign3970_e3536_d_n4, assign3970_e3536_d_n6, assign3970_e3536_d_n7, assign3970_e3536_d_n8, assign3970_e3536_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign3970_e3536;
        var_vfbac2_t_dn4 = assign3970_e3536_d_n4;
        var_vfbac2_t_dn6 = assign3970_e3536_d_n6;
        var_vfbac2_t_dn7 = assign3970_e3536_d_n7;
        var_vfbac2_t_dn8 = assign3970_e3536_d_n8;
        var_vfbac2_t_dn9 = assign3970_e3536_d_n9;

        let (assign3980_e3541,) = {
    if (var_guard83 == 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign3980_e3541;

        let (assign3990_e3546,) = {
    if (var_guard83 == 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign3990_e3546;

        let (assign4000_e3551, assign4000_e3551_d_n4, assign4000_e3551_d_n6, assign4000_e3551_d_n7, assign4000_e3551_d_n8, assign4000_e3551_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign4000_e3551;
        var_cfac_p_dn4 = assign4000_e3551_d_n4;
        var_cfac_p_dn6 = assign4000_e3551_d_n6;
        var_cfac_p_dn7 = assign4000_e3551_d_n7;
        var_cfac_p_dn8 = assign4000_e3551_d_n8;
        var_cfac_p_dn9 = assign4000_e3551_d_n9;

        let (assign4010_e3556, assign4010_e3556_d_n4, assign4010_e3556_d_n6, assign4010_e3556_d_n7, assign4010_e3556_d_n8, assign4010_e3556_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign4010_e3556;
        var_cfac1_t_dn4 = assign4010_e3556_d_n4;
        var_cfac1_t_dn6 = assign4010_e3556_d_n6;
        var_cfac1_t_dn7 = assign4010_e3556_d_n7;
        var_cfac1_t_dn8 = assign4010_e3556_d_n8;
        var_cfac1_t_dn9 = assign4010_e3556_d_n9;

        let (assign4020_e3561, assign4020_e3561_d_n4, assign4020_e3561_d_n6, assign4020_e3561_d_n7, assign4020_e3561_d_n8, assign4020_e3561_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign4020_e3561;
        var_cfac2_t_dn4 = assign4020_e3561_d_n4;
        var_cfac2_t_dn6 = assign4020_e3561_d_n6;
        var_cfac2_t_dn7 = assign4020_e3561_d_n7;
        var_cfac2_t_dn8 = assign4020_e3561_d_n8;
        var_cfac2_t_dn9 = assign4020_e3561_d_n9;

        let (assign4030_e3566, assign4030_e3566_d_n4, assign4030_e3566_d_n6, assign4030_e3566_d_n7, assign4030_e3566_d_n8, assign4030_e3566_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign4030_e3566;
        var_thesatac_p_dn4 = assign4030_e3566_d_n4;
        var_thesatac_p_dn6 = assign4030_e3566_d_n6;
        var_thesatac_p_dn7 = assign4030_e3566_d_n7;
        var_thesatac_p_dn8 = assign4030_e3566_d_n8;
        var_thesatac_p_dn9 = assign4030_e3566_d_n9;

        let (assign4040_e3571, assign4040_e3571_d_n4, assign4040_e3571_d_n6, assign4040_e3571_d_n7, assign4040_e3571_d_n8, assign4040_e3571_d_n9,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign4040_e3571;
        var_thesatac_t_dn4 = assign4040_e3571_d_n4;
        var_thesatac_t_dn6 = assign4040_e3571_d_n6;
        var_thesatac_t_dn7 = assign4040_e3571_d_n7;
        var_thesatac_t_dn8 = assign4040_e3571_d_n8;
        var_thesatac_t_dn9 = assign4040_e3571_d_n9;

        let (assign4050_e3576,) = {
    if (var_guard83 == 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4050_e3576;

        let (assign4060_e3581,) = {
    if (var_guard83 == 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign4060_e3581;

        let assign4070_e3584: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard98 = assign4070_e3584;

        let (assign4080_e3591,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p211,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4080_e3591;

        let assign4090_e3593: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4090_e3595: f64 = if assign4090_e3593 == 1.0 { 1.0 } else { 0.0 };
        var_guard99 = assign4090_e3595;

        let (assign4100_e3604,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard99 != 0.0)) {
        (p.p401,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4100_e3604;

        let (assign4110_e3611,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p212,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4110_e3611;

        let assign4120_e3613: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4120_e3615: f64 = if assign4120_e3613 == 1.0 { 1.0 } else { 0.0 };
        var_guard100 = assign4120_e3615;

        let (assign4130_e3624,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard100 != 0.0)) {
        (p.p402,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4130_e3624;

        let (assign4140_e3631,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p213,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4140_e3631;

        let assign4150_e3633: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4150_e3635: f64 = if assign4150_e3633 == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign4150_e3635;

        let (assign4160_e3644,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard101 != 0.0)) {
        (p.p403,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4160_e3644;

        let (assign4170_e3651,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p216,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4170_e3651;

        let assign4180_e3653: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4180_e3655: f64 = if assign4180_e3653 == 1.0 { 1.0 } else { 0.0 };
        var_guard102 = assign4180_e3655;

        let (assign4190_e3664,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard102 != 0.0)) {
        (p.p406,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4190_e3664;

        let (assign4200_e3671,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p217,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4200_e3671;

        let assign4210_e3673: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4210_e3675: f64 = if assign4210_e3673 == 1.0 { 1.0 } else { 0.0 };
        var_guard103 = assign4210_e3675;

        let (assign4220_e3684,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard103 != 0.0)) {
        (p.p407,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4220_e3684;

        let (assign4230_e3691,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p214,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4230_e3691;

        let assign4240_e3693: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4240_e3695: f64 = if assign4240_e3693 == 1.0 { 1.0 } else { 0.0 };
        var_guard104 = assign4240_e3695;

        let (assign4250_e3704,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard104 != 0.0)) {
        (p.p404,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4250_e3704;

        let (assign4260_e3711,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p215,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4260_e3711;

        let assign4270_e3713: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4270_e3715: f64 = if assign4270_e3713 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign4270_e3715;

        let (assign4280_e3724,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p405,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4280_e3724;

        let (assign4290_e3743, assign4290_e3743_d_n4, assign4290_e3743_d_n6, assign4290_e3743_d_n7, assign4290_e3743_d_n8, assign4290_e3743_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4290_e3732: f64 = (var_ile).powf(var_vfbaclexp_i);
        let assign4290_e3733: f64 = (var_vfbacl_i * assign4290_e3732);
        let assign4290_e3738: f64 = (var_ile).powf(var_vfbaclexp2_i);
        let assign4290_e3739: f64 = (var_vfbacl2_i * assign4290_e3738);
        let assign4290_e3740: f64 = (1.0 + assign4290_e3739);
        let assign4290_e3741: f64 = (assign4290_e3733 / assign4290_e3740);
        (assign4290_e3741, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign4290_e3743;
        var_temp_dn4 = assign4290_e3743_d_n4;
        var_temp_dn6 = assign4290_e3743_d_n6;
        var_temp_dn7 = assign4290_e3743_d_n7;
        var_temp_dn8 = assign4290_e3743_d_n8;
        var_temp_dn9 = assign4290_e3743_d_n9;

        *var_alpac_i_slot = var_alpac_i;
        *var_areaq_i_slot = var_areaq_i;
        *var_axac_i_slot = var_axac_i;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_dn6_slot = var_betnedge_t_dn6;
        *var_betnedge_t_dn7_slot = var_betnedge_t_dn7;
        *var_betnedge_t_dn8_slot = var_betnedge_t_dn8;
        *var_betnedge_t_dn9_slot = var_betnedge_t_dn9;
        *var_cf1edge_i_slot = var_cf1edge_i;
        *var_cf1edge_i_dn4_slot = var_cf1edge_i_dn4;
        *var_cf1edge_i_dn6_slot = var_cf1edge_i_dn6;
        *var_cf1edge_i_dn7_slot = var_cf1edge_i_dn7;
        *var_cf1edge_i_dn8_slot = var_cf1edge_i_dn8;
        *var_cf1edge_i_dn9_slot = var_cf1edge_i_dn9;
        *var_cf2edge_i_slot = var_cf2edge_i;
        *var_cf2edge_i_dn4_slot = var_cf2edge_i_dn4;
        *var_cf2edge_i_dn6_slot = var_cf2edge_i_dn6;
        *var_cf2edge_i_dn7_slot = var_cf2edge_i_dn7;
        *var_cf2edge_i_dn8_slot = var_cf2edge_i_dn8;
        *var_cf2edge_i_dn9_slot = var_cf2edge_i_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_dn4_slot = var_cgbov_i_dn4;
        *var_cgbov_i_dn6_slot = var_cgbov_i_dn6;
        *var_cgbov_i_dn7_slot = var_cgbov_i_dn7;
        *var_cgbov_i_dn8_slot = var_cgbov_i_dn8;
        *var_cgbov_i_dn9_slot = var_cgbov_i_dn9;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_dn4_slot = var_cgbov_p_dn4;
        *var_cgbov_p_dn6_slot = var_cgbov_p_dn6;
        *var_cgbov_p_dn7_slot = var_cgbov_p_dn7;
        *var_cgbov_p_dn8_slot = var_cgbov_p_dn8;
        *var_cgbov_p_dn9_slot = var_cgbov_p_dn9;
        *var_fif_i_slot = var_fif_i;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_guard100_slot = var_guard100;
        *var_guard101_slot = var_guard101;
        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_guard98_slot = var_guard98;
        *var_guard99_slot = var_guard99;
        *var_nsdac_i_slot = var_nsdac_i;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbacl2_i_slot = var_vfbacl2_i;
        *var_vfbacl_i_slot = var_vfbacl_i;
        *var_vfbaclexp2_i_slot = var_vfbaclexp2_i;
        *var_vfbaclexp_i_slot = var_vfbaclexp_i;
        *var_vfbaclw_i_slot = var_vfbaclw_i;
        *var_vfbaco_i_slot = var_vfbaco_i;
        *var_vfbacw_i_slot = var_vfbacw_i;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_ge: f64,
        var_ge_dn4: f64,
        var_ge_dn6: f64,
        var_ge_dn7: f64,
        var_ge_dn8: f64,
        var_ge_dn9: f64,
        var_guard83: f64,
        var_guard98: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfbaclw_i: f64,
        var_vfbaco_i: f64,
        var_vfbacw_i: f64,
        var_axacl_i_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfacl_i_slot: &mut f64,
        var_cfaclexp_i_slot: &mut f64,
        var_cfacw_i_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_psceac_p_slot: &mut f64,
        var_psceacl_i_slot: &mut f64,
        var_psceaclexp_i_slot: &mut f64,
        var_psceacw_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_vfbbaco_i_slot: &mut f64,
        var_vfblbaco_i_slot: &mut f64,
    ) {
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfacl_i: f64 = *var_cfacl_i_slot;
        let mut var_cfaclexp_i: f64 = *var_cfaclexp_i_slot;
        let mut var_cfacw_i: f64 = *var_cfacw_i_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_psceac_p: f64 = *var_psceac_p_slot;
        let mut var_psceacl_i: f64 = *var_psceacl_i_slot;
        let mut var_psceaclexp_i: f64 = *var_psceaclexp_i_slot;
        let mut var_psceacw_i: f64 = *var_psceacw_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_vfbbaco_i: f64 = *var_vfbbaco_i_slot;
        let mut var_vfblbaco_i: f64 = *var_vfblbaco_i_slot;

        let (assign4300_e3760, assign4300_e3760_d_n4, assign4300_e3760_d_n6, assign4300_e3760_d_n7, assign4300_e3760_d_n8, assign4300_e3760_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4300_e3750: f64 = (var_vfbaco_i + var_temp);
        let assign4300_e3753: f64 = (var_vfbacw_i * var_iwe);
        let assign4300_e3754: f64 = (assign4300_e3750 + assign4300_e3753);
        let assign4300_e3757: f64 = (var_vfbaclw_i * var_iae);
        let assign4300_e3758: f64 = (assign4300_e3754 + assign4300_e3757);
        (assign4300_e3758, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign4300_e3760;
        var_vfbac1_t_dn4 = assign4300_e3760_d_n4;
        var_vfbac1_t_dn6 = assign4300_e3760_d_n6;
        var_vfbac1_t_dn7 = assign4300_e3760_d_n7;
        var_vfbac1_t_dn8 = assign4300_e3760_d_n8;
        var_vfbac1_t_dn9 = assign4300_e3760_d_n9;

        let (assign4310_e3767,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p218,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4310_e3767;

        let assign4320_e3769: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4320_e3771: f64 = if assign4320_e3769 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign4320_e3771;

        let (assign4330_e3780,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p408,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4330_e3780;

        let (assign4340_e3787,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p219,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4340_e3787;

        let assign4350_e3789: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4350_e3791: f64 = if assign4350_e3789 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign4350_e3791;

        let (assign4360_e3800,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p409,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4360_e3800;

        let (assign4370_e3815, assign4370_e3815_d_n4, assign4370_e3815_d_n6, assign4370_e3815_d_n7, assign4370_e3815_d_n8, assign4370_e3815_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4370_e3808: f64 = (var_vfblbaco_i * var_tox2_i);
        let assign4370_e3810: f64 = (assign4370_e3808 / var_tox1_i);
        let assign4370_e3812: f64 = (assign4370_e3810 * var_temp);
        let assign4370_e3813: f64 = (var_vfbbaco_i + assign4370_e3812);
        (assign4370_e3813, (assign4370_e3810 * var_temp_dn4), (assign4370_e3810 * var_temp_dn6), (assign4370_e3810 * var_temp_dn7), (assign4370_e3810 * var_temp_dn8), (assign4370_e3810 * var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign4370_e3815;
        var_vfbac2_t_dn4 = assign4370_e3815_d_n4;
        var_vfbac2_t_dn6 = assign4370_e3815_d_n6;
        var_vfbac2_t_dn7 = assign4370_e3815_d_n7;
        var_vfbac2_t_dn8 = assign4370_e3815_d_n8;
        var_vfbac2_t_dn9 = assign4370_e3815_d_n9;

        let (assign4380_e3822,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p228,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4380_e3822;

        let assign4390_e3824: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4390_e3826: f64 = if assign4390_e3824 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign4390_e3826;

        let (assign4400_e3835,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p410,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4400_e3835;

        let (assign4410_e3842,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p229,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4410_e3842;

        let assign4420_e3844: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4420_e3846: f64 = if assign4420_e3844 == 1.0 { 1.0 } else { 0.0 };
        var_guard109 = assign4420_e3846;

        let (assign4430_e3855,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard109 != 0.0)) {
        (p.p411,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4430_e3855;

        let (assign4440_e3862,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p230,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4440_e3862;

        let assign4450_e3864: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4450_e3866: f64 = if assign4450_e3864 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign4450_e3866;

        let (assign4460_e3875,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p412,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4460_e3875;

        let (assign4470_e3894,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4470_e3882: f64 = (var_psceacl_i * 2.0);
        let assign4470_e3885: f64 = (var_lambda_le).powf(var_psceaclexp_i);
        let assign4470_e3886: f64 = (assign4470_e3882 * assign4470_e3885);
        let assign4470_e3890: f64 = (var_psceacw_i * var_iwe);
        let assign4470_e3891: f64 = (1.0 + assign4470_e3890);
        let assign4470_e3892: f64 = (assign4470_e3886 * assign4470_e3891);
        (assign4470_e3892,)
    } else {
        (var_psceac_p,)
    }
};
        var_psceac_p = assign4470_e3894;

        let (assign4480_e3905,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4480_e3901: f64 = (var_psceac_p).max(0.0);
        let assign4480_e3903: f64 = (assign4480_e3901).min(5.0);
        (assign4480_e3903,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign4480_e3905;

        let (assign4490_e3918,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4490_e3912: f64 = (p.p231 * var_psceac1_i);
        let assign4490_e3914: f64 = (assign4490_e3912 * var_tox2_i);
        let assign4490_e3916: f64 = (assign4490_e3914 / var_tox1_i);
        (assign4490_e3916,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign4490_e3918;

        let (assign4500_e3925,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p235,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4500_e3925;

        let assign4510_e3927: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4510_e3929: f64 = if assign4510_e3927 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign4510_e3929;

        let (assign4520_e3938,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p413,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4520_e3938;

        let (assign4530_e3945,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p236,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4530_e3945;

        let assign4540_e3947: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4540_e3949: f64 = if assign4540_e3947 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign4540_e3949;

        let (assign4550_e3958,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p414,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4550_e3958;

        let (assign4560_e3965,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p237,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4560_e3965;

        let assign4570_e3967: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4570_e3969: f64 = if assign4570_e3967 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign4570_e3969;

        let (assign4580_e3978,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p415,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4580_e3978;

        let (assign4590_e3993, assign4590_e3993_d_n4, assign4590_e3993_d_n6, assign4590_e3993_d_n7, assign4590_e3993_d_n8, assign4590_e3993_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4590_e3985: f64 = (var_lambda_le).powf(var_cfaclexp_i);
        let assign4590_e3989: f64 = (var_cfacw_i * var_iwe);
        let assign4590_e3990: f64 = (1.0 + assign4590_e3989);
        let assign4590_e3991: f64 = (assign4590_e3985 * assign4590_e3990);
        (assign4590_e3991, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign4590_e3993;
        var_temp_dn4 = assign4590_e3993_d_n4;
        var_temp_dn6 = assign4590_e3993_d_n6;
        var_temp_dn7 = assign4590_e3993_d_n7;
        var_temp_dn8 = assign4590_e3993_d_n8;
        var_temp_dn9 = assign4590_e3993_d_n9;

        let (assign4600_e4002, assign4600_e4002_d_n4, assign4600_e4002_d_n6, assign4600_e4002_d_n7, assign4600_e4002_d_n8, assign4600_e4002_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4600_e4000: f64 = (var_cfacl_i * var_temp);
        (assign4600_e4000, (var_cfacl_i * var_temp_dn4), (var_cfacl_i * var_temp_dn6), (var_cfacl_i * var_temp_dn7), (var_cfacl_i * var_temp_dn8), (var_cfacl_i * var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign4600_e4002;
        var_cfac_p_dn4 = assign4600_e4002_d_n4;
        var_cfac_p_dn6 = assign4600_e4002_d_n6;
        var_cfac_p_dn7 = assign4600_e4002_d_n7;
        var_cfac_p_dn8 = assign4600_e4002_d_n8;
        var_cfac_p_dn9 = assign4600_e4002_d_n9;

        let (assign4610_e4011, assign4610_e4011_d_n4, assign4610_e4011_d_n6, assign4610_e4011_d_n7, assign4610_e4011_d_n8, assign4610_e4011_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4610_e4009: f64 = (var_cfac_p).max(0.0);
        (assign4610_e4009, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign4610_e4011;
        var_cfac1_t_dn4 = assign4610_e4011_d_n4;
        var_cfac1_t_dn6 = assign4610_e4011_d_n6;
        var_cfac1_t_dn7 = assign4610_e4011_d_n7;
        var_cfac1_t_dn8 = assign4610_e4011_d_n8;
        var_cfac1_t_dn9 = assign4610_e4011_d_n9;

        let (assign4620_e4024, assign4620_e4024_d_n4, assign4620_e4024_d_n6, assign4620_e4024_d_n7, assign4620_e4024_d_n8, assign4620_e4024_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4620_e4018: f64 = (p.p238 * var_cfac1_t);
        let assign4620_e4020: f64 = (assign4620_e4018 * var_tox2_i);
        let assign4620_e4022: f64 = (assign4620_e4020 / var_tox1_i);
        (assign4620_e4022, (((p.p238 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p238 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign4620_e4024;
        var_cfac2_t_dn4 = assign4620_e4024_d_n4;
        var_cfac2_t_dn6 = assign4620_e4024_d_n6;
        var_cfac2_t_dn7 = assign4620_e4024_d_n7;
        var_cfac2_t_dn8 = assign4620_e4024_d_n8;
        var_cfac2_t_dn9 = assign4620_e4024_d_n9;

        let (assign4630_e4031,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p293,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4630_e4031;

        let assign4640_e4033: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4640_e4035: f64 = if assign4640_e4033 == 1.0 { 1.0 } else { 0.0 };
        var_guard114 = assign4640_e4035;

        let (assign4650_e4044,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard114 != 0.0)) {
        (p.p416,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4650_e4044;

        let (assign4660_e4051,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p294,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4660_e4051;

        let assign4670_e4053: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4670_e4055: f64 = if assign4670_e4053 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign4670_e4055;

        let (assign4680_e4064,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard115 != 0.0)) {
        (p.p417,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4680_e4064;

        let (assign4690_e4071,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p295,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4690_e4071;

        let assign4700_e4073: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4700_e4075: f64 = if assign4700_e4073 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign4700_e4075;

        let (assign4710_e4084,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard116 != 0.0)) {
        (p.p418,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4710_e4084;

        let (assign4720_e4091,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p296,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4720_e4091;

        let assign4730_e4093: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4730_e4095: f64 = if assign4730_e4093 == 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign4730_e4095;

        let (assign4740_e4104,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard117 != 0.0)) {
        (p.p419,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4740_e4104;

        let (assign4750_e4111,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p297,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4750_e4111;

        let assign4760_e4113: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4760_e4115: f64 = if assign4760_e4113 == 1.0 { 1.0 } else { 0.0 };
        var_guard118 = assign4760_e4115;

        let (assign4770_e4124,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard118 != 0.0)) {
        (p.p420,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4770_e4124;

        let (assign4780_e4151, assign4780_e4151_d_n4, assign4780_e4151_d_n6, assign4780_e4151_d_n7, assign4780_e4151_d_n8, assign4780_e4151_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4780_e4134: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign4780_e4135: f64 = (var_thesatacl_i * assign4780_e4134);
        let assign4780_e4136: f64 = (var_thesataco_i + assign4780_e4135);
        let assign4780_e4137: f64 = (var_ge * assign4780_e4136);
        let assign4780_e4141: f64 = (var_thesatacw_i * var_iwe);
        let assign4780_e4142: f64 = (1.0 + assign4780_e4141);
        let assign4780_e4143: f64 = (assign4780_e4137 * assign4780_e4142);
        let assign4780_e4147: f64 = (var_thesataclw_i * var_iae);
        let assign4780_e4148: f64 = (1.0 + assign4780_e4147);
        let assign4780_e4149: f64 = (assign4780_e4143 * assign4780_e4148);
        (assign4780_e4149, (((var_ge_dn4 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn6 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn7 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn8 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148), (((var_ge_dn9 * assign4780_e4136) * assign4780_e4142) * assign4780_e4148),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign4780_e4151;
        var_thesatac_p_dn4 = assign4780_e4151_d_n4;
        var_thesatac_p_dn6 = assign4780_e4151_d_n6;
        var_thesatac_p_dn7 = assign4780_e4151_d_n7;
        var_thesatac_p_dn8 = assign4780_e4151_d_n8;
        var_thesatac_p_dn9 = assign4780_e4151_d_n9;

        let (assign4790_e4160, assign4790_e4160_d_n4, assign4790_e4160_d_n6, assign4790_e4160_d_n7, assign4790_e4160_d_n8, assign4790_e4160_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4790_e4158: f64 = (var_thesatac_p).max(0.0);
        (assign4790_e4158, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign4790_e4160;
        var_thesatac_t_dn4 = assign4790_e4160_d_n4;
        var_thesatac_t_dn6 = assign4790_e4160_d_n6;
        var_thesatac_t_dn7 = assign4790_e4160_d_n7;
        var_thesatac_t_dn8 = assign4790_e4160_d_n8;
        var_thesatac_t_dn9 = assign4790_e4160_d_n9;

        let (assign4800_e4167,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p304,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4800_e4167;

        let assign4810_e4169: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4810_e4171: f64 = if assign4810_e4169 == 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign4810_e4171;

        let (assign4820_e4180,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard119 != 0.0)) {
        (p.p421,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4820_e4180;

        let (assign4830_e4187,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p305,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4830_e4187;

        let assign4840_e4189: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4840_e4191: f64 = if assign4840_e4189 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign4840_e4191;

        let (assign4850_e4200,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard120 != 0.0)) {
        (p.p422,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4850_e4200;

        *var_axacl_i_slot = var_axacl_i;
        *var_axaco_i_slot = var_axaco_i;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfacl_i_slot = var_cfacl_i;
        *var_cfaclexp_i_slot = var_cfaclexp_i;
        *var_cfacw_i_slot = var_cfacw_i;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard108_slot = var_guard108;
        *var_guard109_slot = var_guard109;
        *var_guard110_slot = var_guard110;
        *var_guard111_slot = var_guard111;
        *var_guard112_slot = var_guard112;
        *var_guard113_slot = var_guard113;
        *var_guard114_slot = var_guard114;
        *var_guard115_slot = var_guard115;
        *var_guard116_slot = var_guard116;
        *var_guard117_slot = var_guard117;
        *var_guard118_slot = var_guard118;
        *var_guard119_slot = var_guard119;
        *var_guard120_slot = var_guard120;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_psceac_p_slot = var_psceac_p;
        *var_psceacl_i_slot = var_psceacl_i;
        *var_psceaclexp_i_slot = var_psceaclexp_i;
        *var_psceacw_i_slot = var_psceacw_i;
        *var_temp_slot = var_temp;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_vfbbaco_i_slot = var_vfbbaco_i;
        *var_vfblbaco_i_slot = var_vfblbaco_i;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_axacl_i: f64,
        var_axaco_i: f64,
        var_epsch: f64,
        var_guard83: f64,
        var_guard98: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_lphy: f64,
        var_lphy_dn4: f64,
        var_lphy_dn6: f64,
        var_lphy_dn7: f64,
        var_lphy_dn8: f64,
        var_lphy_dn9: f64,
        var_tox1_i: f64,
        var_tsi_i: f64,
        var_we: f64,
        var_wecv: f64,
        var_wen: f64,
        var_wphy: f64,
        var_wphy_dn4: f64,
        var_wphy_dn6: f64,
        var_wphy_dn7: f64,
        var_wphy_dn8: f64,
        var_wphy_dn9: f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpacl1_i_slot: &mut f64,
        var_alpacl2_i_slot: &mut f64,
        var_alpaclexp2_i_slot: &mut f64,
        var_alpaclexp_i_slot: &mut f64,
        var_alpacw_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axacl2_i_slot: &mut f64,
        var_axaclexp2_i_slot: &mut f64,
        var_axaclexp_i_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_dn4_slot: &mut f64,
        var_cfr_p_dn6_slot: &mut f64,
        var_cfr_p_dn7_slot: &mut f64,
        var_cfr_p_dn8_slot: &mut f64,
        var_cfr_p_dn9_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_dn4_slot: &mut f64,
        var_cfrd_p_dn6_slot: &mut f64,
        var_cfrd_p_dn7_slot: &mut f64,
        var_cfrd_p_dn8_slot: &mut f64,
        var_cfrd_p_dn9_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covdl_i_slot: &mut f64,
        var_covdlb_i_slot: &mut f64,
        var_csd_i_slot: &mut f64,
        var_csdbp_i_slot: &mut f64,
        var_dvfbov_i_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpacl1_i: f64 = *var_alpacl1_i_slot;
        let mut var_alpacl2_i: f64 = *var_alpacl2_i_slot;
        let mut var_alpaclexp2_i: f64 = *var_alpaclexp2_i_slot;
        let mut var_alpaclexp_i: f64 = *var_alpaclexp_i_slot;
        let mut var_alpacw_i: f64 = *var_alpacw_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axacl2_i: f64 = *var_axacl2_i_slot;
        let mut var_axaclexp2_i: f64 = *var_axaclexp2_i_slot;
        let mut var_axaclexp_i: f64 = *var_axaclexp_i_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_dn4: f64 = *var_cfr_p_dn4_slot;
        let mut var_cfr_p_dn6: f64 = *var_cfr_p_dn6_slot;
        let mut var_cfr_p_dn7: f64 = *var_cfr_p_dn7_slot;
        let mut var_cfr_p_dn8: f64 = *var_cfr_p_dn8_slot;
        let mut var_cfr_p_dn9: f64 = *var_cfr_p_dn9_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_dn4: f64 = *var_cfrd_p_dn4_slot;
        let mut var_cfrd_p_dn6: f64 = *var_cfrd_p_dn6_slot;
        let mut var_cfrd_p_dn7: f64 = *var_cfrd_p_dn7_slot;
        let mut var_cfrd_p_dn8: f64 = *var_cfrd_p_dn8_slot;
        let mut var_cfrd_p_dn9: f64 = *var_cfrd_p_dn9_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covdl_i: f64 = *var_covdl_i_slot;
        let mut var_covdlb_i: f64 = *var_covdlb_i_slot;
        let mut var_csd_i: f64 = *var_csd_i_slot;
        let mut var_csdbp_i: f64 = *var_csdbp_i_slot;
        let mut var_dvfbov_i: f64 = *var_dvfbov_i_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign4860_e4207,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p306,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4860_e4207;

        let assign4870_e4209: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4870_e4211: f64 = if assign4870_e4209 == 1.0 { 1.0 } else { 0.0 };
        var_guard121 = assign4870_e4211;

        let (assign4880_e4220,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard121 != 0.0)) {
        (p.p423,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4880_e4220;

        let (assign4890_e4227,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p307,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4890_e4227;

        let assign4900_e4229: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign4900_e4231: f64 = if assign4900_e4229 == 1.0 { 1.0 } else { 0.0 };
        var_guard122 = assign4900_e4231;

        let (assign4910_e4240,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard122 != 0.0)) {
        (p.p424,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4910_e4240;

        let (assign4920_e4247,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p308,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4920_e4247;

        let assign4930_e4249: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign4930_e4251: f64 = if assign4930_e4249 == 1.0 { 1.0 } else { 0.0 };
        var_guard123 = assign4930_e4251;

        let (assign4940_e4260,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard123 != 0.0)) {
        (p.p425,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4940_e4260;

        let (assign4950_e4283,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4950_e4270: f64 = (var_ile).powf(var_axaclexp_i);
        let assign4950_e4271: f64 = (var_axacl_i * assign4950_e4270);
        let assign4950_e4276: f64 = (var_ile).powf(var_axaclexp2_i);
        let assign4950_e4277: f64 = (var_axacl2_i * assign4950_e4276);
        let assign4950_e4278: f64 = (1.0 + assign4950_e4277);
        let assign4950_e4279: f64 = (assign4950_e4271 / assign4950_e4278);
        let assign4950_e4280: f64 = (1.0 + assign4950_e4279);
        let assign4950_e4281: f64 = (var_axaco_i / assign4950_e4280);
        (assign4950_e4281,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4950_e4283;

        let (assign4960_e4294,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4960_e4290: f64 = (var_axac_p).max(1.0);
        let assign4960_e4292: f64 = (assign4960_e4290).min(16.0);
        (assign4960_e4292,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4960_e4294;

        let (assign4970_e4301,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p309,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4970_e4301;

        let assign4980_e4303: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign4980_e4305: f64 = if assign4980_e4303 == 1.0 { 1.0 } else { 0.0 };
        var_guard124 = assign4980_e4305;

        let (assign4990_e4314,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard124 != 0.0)) {
        (p.p426,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4990_e4314;

        let (assign5000_e4321,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p310,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign5000_e4321;

        let assign5010_e4323: f64 = if param_given[427] { 1.0 } else { 0.0 };
        let assign5010_e4325: f64 = if assign5010_e4323 == 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign5010_e4325;

        let (assign5020_e4334,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard125 != 0.0)) {
        (p.p427,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign5020_e4334;

        let (assign5030_e4341,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p311,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign5030_e4341;

        let assign5040_e4343: f64 = if param_given[428] { 1.0 } else { 0.0 };
        let assign5040_e4345: f64 = if assign5040_e4343 == 1.0 { 1.0 } else { 0.0 };
        var_guard126 = assign5040_e4345;

        let (assign5050_e4354,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard126 != 0.0)) {
        (p.p428,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign5050_e4354;

        let (assign5060_e4361,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p312,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5060_e4361;

        let assign5070_e4363: f64 = if param_given[429] { 1.0 } else { 0.0 };
        let assign5070_e4365: f64 = if assign5070_e4363 == 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign5070_e4365;

        let (assign5080_e4374,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard127 != 0.0)) {
        (p.p429,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5080_e4374;

        let (assign5090_e4381,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p313,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5090_e4381;

        let assign5100_e4383: f64 = if param_given[430] { 1.0 } else { 0.0 };
        let assign5100_e4385: f64 = if assign5100_e4383 == 1.0 { 1.0 } else { 0.0 };
        var_guard128 = assign5100_e4385;

        let (assign5110_e4394,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard128 != 0.0)) {
        (p.p430,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5110_e4394;

        let (assign5120_e4419,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5120_e4402: f64 = (var_ile).powf(var_alpaclexp_i);
        let assign5120_e4403: f64 = (var_alpacl1_i * assign5120_e4402);
        let assign5120_e4407: f64 = (var_alpacw_i * var_iwe);
        let assign5120_e4408: f64 = (1.0 + assign5120_e4407);
        let assign5120_e4409: f64 = (assign5120_e4403 * assign5120_e4408);
        let assign5120_e4414: f64 = (var_ile).powf(var_alpaclexp2_i);
        let assign5120_e4415: f64 = (var_alpacl2_i * assign5120_e4414);
        let assign5120_e4416: f64 = (1.0 + assign5120_e4415);
        let assign5120_e4417: f64 = (assign5120_e4409 / assign5120_e4416);
        (assign5120_e4417,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign5120_e4419;

        let (assign5130_e4428,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5130_e4426: f64 = (var_alpac_p).max(0.0);
        (assign5130_e4426,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign5130_e4428;

        let (assign5140_e4437, assign5140_e4437_d_n4, assign5140_e4437_d_n6, assign5140_e4437_d_n7, assign5140_e4437_d_n8, assign5140_e4437_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5140_e4433: f64 = (3.45313e-11 / var_tox1_i);
        let assign5140_e4435: f64 = (assign5140_e4433 * var_wecv);
        (assign5140_e4435, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5140_e4437;
        var_temp_dn4 = assign5140_e4437_d_n4;
        var_temp_dn6 = assign5140_e4437_d_n6;
        var_temp_dn7 = assign5140_e4437_d_n7;
        var_temp_dn8 = assign5140_e4437_d_n8;
        var_temp_dn9 = assign5140_e4437_d_n9;

        let (assign5150_e4444, assign5150_e4444_d_n4, assign5150_e4444_d_n6, assign5150_e4444_d_n7, assign5150_e4444_d_n8, assign5150_e4444_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5150_e4442: f64 = (var_temp * p.p431);
        (assign5150_e4442, (var_temp_dn4 * p.p431), (var_temp_dn6 * p.p431), (var_temp_dn7 * p.p431), (var_temp_dn8 * p.p431), (var_temp_dn9 * p.p431),)
    } else {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    }
};
        var_cov_i = assign5150_e4444;
        var_cov_i_dn4 = assign5150_e4444_d_n4;
        var_cov_i_dn6 = assign5150_e4444_d_n6;
        var_cov_i_dn7 = assign5150_e4444_d_n7;
        var_cov_i_dn8 = assign5150_e4444_d_n8;
        var_cov_i_dn9 = assign5150_e4444_d_n9;

        let (assign5160_e4451, assign5160_e4451_d_n4, assign5160_e4451_d_n6, assign5160_e4451_d_n7, assign5160_e4451_d_n8, assign5160_e4451_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5160_e4449: f64 = (var_temp * p.p432);
        (assign5160_e4449, (var_temp_dn4 * p.p432), (var_temp_dn6 * p.p432), (var_temp_dn7 * p.p432), (var_temp_dn8 * p.p432), (var_temp_dn9 * p.p432),)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign5160_e4451;
        var_covd_i_dn4 = assign5160_e4451_d_n4;
        var_covd_i_dn6 = assign5160_e4451_d_n6;
        var_covd_i_dn7 = assign5160_e4451_d_n7;
        var_covd_i_dn8 = assign5160_e4451_d_n8;
        var_covd_i_dn9 = assign5160_e4451_d_n9;

        let (assign5170_e4466,) = {
    if (var_guard83 == 0.0) {
        let assign5170_e4458: f64 = (p.p434 * var_wen);
        let assign5170_e4460: f64 = (assign5170_e4458 / var_wecv);
        let assign5170_e4461: f64 = (1.0 + assign5170_e4460);
        let assign5170_e4463: f64 = (assign5170_e4461).max(0.001);
        let assign5170_e4464: f64 = (p.p433 / assign5170_e4463);
        (assign5170_e4464,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign5170_e4466;

        let (assign5180_e4471,) = {
    if (var_guard83 == 0.0) {
        (p.p435,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign5180_e4471;

        let (assign5190_e4476,) = {
    if (var_guard83 == 0.0) {
        (p.p436,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign5190_e4476;

        let (assign5200_e4485, assign5200_e4485_d_n4, assign5200_e4485_d_n6, assign5200_e4485_d_n7, assign5200_e4485_d_n8, assign5200_e4485_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5200_e4482: f64 = (p.p439 * var_wphy);
        let assign5200_e4483: f64 = (p.p437 + assign5200_e4482);
        (assign5200_e4483, (p.p439 * var_wphy_dn4), (p.p439 * var_wphy_dn6), (p.p439 * var_wphy_dn7), (p.p439 * var_wphy_dn8), (p.p439 * var_wphy_dn9),)
    } else {
        (var_cfr_p, var_cfr_p_dn4, var_cfr_p_dn6, var_cfr_p_dn7, var_cfr_p_dn8, var_cfr_p_dn9,)
    }
};
        var_cfr_p = assign5200_e4485;
        var_cfr_p_dn4 = assign5200_e4485_d_n4;
        var_cfr_p_dn6 = assign5200_e4485_d_n6;
        var_cfr_p_dn7 = assign5200_e4485_d_n7;
        var_cfr_p_dn8 = assign5200_e4485_d_n8;
        var_cfr_p_dn9 = assign5200_e4485_d_n9;

        let (assign5210_e4492, assign5210_e4492_d_n4, assign5210_e4492_d_n6, assign5210_e4492_d_n7, assign5210_e4492_d_n8, assign5210_e4492_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5210_e4490: f64 = (var_cfr_p).max(0.0);
        (assign5210_e4490, if var_cfr_p >= 0.0 { var_cfr_p_dn4 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn6 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn7 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn8 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn9 } else { 0.0 },)
    } else {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    }
};
        var_cfr_i = assign5210_e4492;
        var_cfr_i_dn4 = assign5210_e4492_d_n4;
        var_cfr_i_dn6 = assign5210_e4492_d_n6;
        var_cfr_i_dn7 = assign5210_e4492_d_n7;
        var_cfr_i_dn8 = assign5210_e4492_d_n8;
        var_cfr_i_dn9 = assign5210_e4492_d_n9;

        let (assign5220_e4501, assign5220_e4501_d_n4, assign5220_e4501_d_n6, assign5220_e4501_d_n7, assign5220_e4501_d_n8, assign5220_e4501_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5220_e4498: f64 = (p.p440 * var_wphy);
        let assign5220_e4499: f64 = (p.p438 + assign5220_e4498);
        (assign5220_e4499, (p.p440 * var_wphy_dn4), (p.p440 * var_wphy_dn6), (p.p440 * var_wphy_dn7), (p.p440 * var_wphy_dn8), (p.p440 * var_wphy_dn9),)
    } else {
        (var_cfrd_p, var_cfrd_p_dn4, var_cfrd_p_dn6, var_cfrd_p_dn7, var_cfrd_p_dn8, var_cfrd_p_dn9,)
    }
};
        var_cfrd_p = assign5220_e4501;
        var_cfrd_p_dn4 = assign5220_e4501_d_n4;
        var_cfrd_p_dn6 = assign5220_e4501_d_n6;
        var_cfrd_p_dn7 = assign5220_e4501_d_n7;
        var_cfrd_p_dn8 = assign5220_e4501_d_n8;
        var_cfrd_p_dn9 = assign5220_e4501_d_n9;

        let (assign5230_e4508, assign5230_e4508_d_n4, assign5230_e4508_d_n6, assign5230_e4508_d_n7, assign5230_e4508_d_n8, assign5230_e4508_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5230_e4506: f64 = (var_cfrd_p).max(0.0);
        (assign5230_e4506, if var_cfrd_p >= 0.0 { var_cfrd_p_dn4 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn6 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn7 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn8 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn9 } else { 0.0 },)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign5230_e4508;
        var_cfrd_i_dn4 = assign5230_e4508_d_n4;
        var_cfrd_i_dn6 = assign5230_e4508_d_n6;
        var_cfrd_i_dn7 = assign5230_e4508_d_n7;
        var_cfrd_i_dn8 = assign5230_e4508_d_n8;
        var_cfrd_i_dn9 = assign5230_e4508_d_n9;

        let (assign5240_e4521,) = {
    if (var_guard83 == 0.0) {
        let assign5240_e4513: f64 = (p.p441 * var_epsch);
        let assign5240_e4515: f64 = (assign5240_e4513 * var_tsi_i);
        let assign5240_e4517: f64 = (assign5240_e4515 * var_we);
        let assign5240_e4519: f64 = (assign5240_e4517 / var_le);
        (assign5240_e4519,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign5240_e4521;

        let (assign5250_e4526,) = {
    if (var_guard83 == 0.0) {
        (p.p442,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign5250_e4526;

        let (assign5260_e4547, assign5260_e4547_d_n4, assign5260_e4547_d_n6, assign5260_e4547_d_n7, assign5260_e4547_d_n8, assign5260_e4547_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5260_e4532: f64 = (p.p444 * var_lphy);
        let assign5260_e4533: f64 = (1.0 + assign5260_e4532);
        let assign5260_e4536: f64 = (p.p445 * var_wphy);
        let assign5260_e4537: f64 = (assign5260_e4533 + assign5260_e4536);
        let assign5260_e4540: f64 = (p.p446 * var_lphy);
        let assign5260_e4542: f64 = (assign5260_e4540 * var_wphy);
        let assign5260_e4543: f64 = (assign5260_e4537 + assign5260_e4542);
        let assign5260_e4545: f64 = (assign5260_e4543).max(1e-10);
        (assign5260_e4545, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn4) + (p.p445 * var_wphy_dn4)) + (((p.p446 * var_lphy_dn4) * var_wphy) + (assign5260_e4540 * var_wphy_dn4))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn6) + (p.p445 * var_wphy_dn6)) + (((p.p446 * var_lphy_dn6) * var_wphy) + (assign5260_e4540 * var_wphy_dn6))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn7) + (p.p445 * var_wphy_dn7)) + (((p.p446 * var_lphy_dn7) * var_wphy) + (assign5260_e4540 * var_wphy_dn7))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn8) + (p.p445 * var_wphy_dn8)) + (((p.p446 * var_lphy_dn8) * var_wphy) + (assign5260_e4540 * var_wphy_dn8))) } else { 0.0 }, if assign5260_e4543 >= 1e-10 { (((p.p444 * var_lphy_dn9) + (p.p445 * var_wphy_dn9)) + (((p.p446 * var_lphy_dn9) * var_wphy) + (assign5260_e4540 * var_wphy_dn9))) } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5260_e4547;
        var_temp_dn4 = assign5260_e4547_d_n4;
        var_temp_dn6 = assign5260_e4547_d_n6;
        var_temp_dn7 = assign5260_e4547_d_n7;
        var_temp_dn8 = assign5260_e4547_d_n8;
        var_temp_dn9 = assign5260_e4547_d_n9;

        let (assign5270_e4552, assign5270_e4552_d_n4, assign5270_e4552_d_n6, assign5270_e4552_d_n7, assign5270_e4552_d_n8, assign5270_e4552_d_n9,) = {
    if (var_guard83 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign5270_e4552;
        var_temp1_dn4 = assign5270_e4552_d_n4;
        var_temp1_dn6 = assign5270_e4552_d_n6;
        var_temp1_dn7 = assign5270_e4552_d_n7;
        var_temp1_dn8 = assign5270_e4552_d_n8;
        var_temp1_dn9 = assign5270_e4552_d_n9;

        let assign5280_e4559: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        var_guard129 = assign5280_e4559;

        let (assign5290_e4571, assign5290_e4571_d_n4, assign5290_e4571_d_n6, assign5290_e4571_d_n7, assign5290_e4571_d_n8, assign5290_e4571_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5290_e4566: f64 = (p.p28 + p.p20);
        let assign5290_e4567: f64 = (-assign5290_e4566);
        let assign5290_e4569: f64 = (assign5290_e4567 / p.p449);
        (assign5290_e4569, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign5290_e4571;
        var_temp2_dn4 = assign5290_e4571_d_n4;
        var_temp2_dn6 = assign5290_e4571_d_n6;
        var_temp2_dn7 = assign5290_e4571_d_n7;
        var_temp2_dn8 = assign5290_e4571_d_n8;
        var_temp2_dn9 = assign5290_e4571_d_n9;

        let assign5300_e4573: f64 = (var_temp2).abs();
        let assign5300_e4575: f64 = if assign5300_e4573 < 80.0 { 1.0 } else { 0.0 };
        var_guard130 = assign5300_e4575;

        let (assign5310_e4585, assign5310_e4585_d_n4, assign5310_e4585_d_n6, assign5310_e4585_d_n7, assign5310_e4585_d_n8, assign5310_e4585_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 != 0.0)) {
        let assign5310_e4583: f64 = (var_temp2).exp();
        (assign5310_e4583, (assign5310_e4583 * var_temp2_dn4), (assign5310_e4583 * var_temp2_dn6), (assign5310_e4583 * var_temp2_dn7), (assign5310_e4583 * var_temp2_dn8), (assign5310_e4583 * var_temp2_dn9),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5310_e4585;
        var_temp3_dn4 = assign5310_e4585_d_n4;
        var_temp3_dn6 = assign5310_e4585_d_n6;
        var_temp3_dn7 = assign5310_e4585_d_n7;
        var_temp3_dn8 = assign5310_e4585_d_n8;
        var_temp3_dn9 = assign5310_e4585_d_n9;

        let assign5320_e4588: f64 = (-80.0);
        let assign5320_e4589: f64 = if var_temp2 < assign5320_e4588 { 1.0 } else { 0.0 };
        var_guard131 = assign5320_e4589;

        let (assign5330_e4626, assign5330_e4626_d_n4, assign5330_e4626_d_n6, assign5330_e4626_d_n7, assign5330_e4626_d_n8, assign5330_e4626_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 != 0.0)) {
        let assign5330_e4602: f64 = (-var_temp2);
        let assign5330_e4604: f64 = (assign5330_e4602 - 80.0);
        let assign5330_e4608: f64 = (-var_temp2);
        let assign5330_e4610: f64 = (assign5330_e4608 - 80.0);
        let assign5330_e4611: f64 = (0.5 * assign5330_e4610);
        let assign5330_e4614: f64 = (-var_temp2);
        let assign5330_e4616: f64 = (assign5330_e4614 - 80.0);
        let assign5330_e4618: f64 = (assign5330_e4616 * 0.3333333333333);
        let assign5330_e4619: f64 = (1.0 + assign5330_e4618);
        let assign5330_e4620: f64 = (assign5330_e4611 * assign5330_e4619);
        let assign5330_e4621: f64 = (1.0 + assign5330_e4620);
        let assign5330_e4622: f64 = (assign5330_e4604 * assign5330_e4621);
        let assign5330_e4623: f64 = (1.0 + assign5330_e4622);
        let assign5330_e4624: f64 = (1.80485e-35 / assign5330_e4623);
        (assign5330_e4624, (-((1.80485e-35 * (((-var_temp2_dn4) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn4)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn4) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn6) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn6)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn6) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn7) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn7)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn7) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn8) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn8)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn8) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))), (-((1.80485e-35 * (((-var_temp2_dn9) * assign5330_e4621) + (assign5330_e4604 * (((0.5 * (-var_temp2_dn9)) * assign5330_e4619) + (assign5330_e4611 * ((-var_temp2_dn9) * 0.3333333333333)))))) / (assign5330_e4623 * assign5330_e4623))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5330_e4626;
        var_temp3_dn4 = assign5330_e4626_d_n4;
        var_temp3_dn6 = assign5330_e4626_d_n6;
        var_temp3_dn7 = assign5330_e4626_d_n7;
        var_temp3_dn8 = assign5330_e4626_d_n8;
        var_temp3_dn9 = assign5330_e4626_d_n9;

        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpacl1_i_slot = var_alpacl1_i;
        *var_alpacl2_i_slot = var_alpacl2_i;
        *var_alpaclexp2_i_slot = var_alpaclexp2_i;
        *var_alpaclexp_i_slot = var_alpaclexp_i;
        *var_alpacw_i_slot = var_alpacw_i;
        *var_axac_i_slot = var_axac_i;
        *var_axac_p_slot = var_axac_p;
        *var_axacl2_i_slot = var_axacl2_i;
        *var_axaclexp2_i_slot = var_axaclexp2_i;
        *var_axaclexp_i_slot = var_axaclexp_i;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_dn4_slot = var_cfr_p_dn4;
        *var_cfr_p_dn6_slot = var_cfr_p_dn6;
        *var_cfr_p_dn7_slot = var_cfr_p_dn7;
        *var_cfr_p_dn8_slot = var_cfr_p_dn8;
        *var_cfr_p_dn9_slot = var_cfr_p_dn9;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_dn4_slot = var_cfrd_p_dn4;
        *var_cfrd_p_dn6_slot = var_cfrd_p_dn6;
        *var_cfrd_p_dn7_slot = var_cfrd_p_dn7;
        *var_cfrd_p_dn8_slot = var_cfrd_p_dn8;
        *var_cfrd_p_dn9_slot = var_cfrd_p_dn9;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covdl_i_slot = var_covdl_i;
        *var_covdlb_i_slot = var_covdlb_i;
        *var_csd_i_slot = var_csd_i;
        *var_csdbp_i_slot = var_csdbp_i;
        *var_dvfbov_i_slot = var_dvfbov_i;
        *var_guard121_slot = var_guard121;
        *var_guard122_slot = var_guard122;
        *var_guard123_slot = var_guard123;
        *var_guard124_slot = var_guard124;
        *var_guard125_slot = var_guard125;
        *var_guard126_slot = var_guard126;
        *var_guard127_slot = var_guard127;
        *var_guard128_slot = var_guard128;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        var_betn_p: f64,
        var_dellps: f64,
        var_delwod: f64,
        var_guard129: f64,
        var_guard130: f64,
        var_guard131: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_temp2: f64,
        var_temp2_dn4: f64,
        var_temp2_dn6: f64,
        var_temp2_dn7: f64,
        var_temp2_dn8: f64,
        var_temp2_dn9: f64,
        var_w_i: f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_dn4_slot: &mut f64,
        var_cth_p_dn6_slot: &mut f64,
        var_cth_p_dn7_slot: &mut f64,
        var_cth_p_dn8_slot: &mut f64,
        var_cth_p_dn9_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fntexc_i_slot: &mut f64,
        var_fracinv_i_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_dn4_slot: &mut f64,
        var_invsa_dn6_slot: &mut f64,
        var_invsa_dn7_slot: &mut f64,
        var_invsa_dn8_slot: &mut f64,
        var_invsa_dn9_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_kdiff_i_slot: &mut f64,
        var_kdiff_i_dn4_slot: &mut f64,
        var_kdiff_i_dn6_slot: &mut f64,
        var_kdiff_i_dn7_slot: &mut f64,
        var_kdiff_i_dn8_slot: &mut f64,
        var_kdiff_i_dn9_slot: &mut f64,
        var_kdrift_i_slot: &mut f64,
        var_kdrift_i_dn4_slot: &mut f64,
        var_kdrift_i_dn6_slot: &mut f64,
        var_kdrift_i_dn7_slot: &mut f64,
        var_kdrift_i_dn8_slot: &mut f64,
        var_kdrift_i_dn9_slot: &mut f64,
        var_kfracinv_i_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfa_p_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_rth_p_dn4_slot: &mut f64,
        var_rth_p_dn6_slot: &mut f64,
        var_rth_p_dn7_slot: &mut f64,
        var_rth_p_dn8_slot: &mut f64,
        var_rth_p_dn9_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_wx_slot: &mut f64,
    ) {
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_dn4: f64 = *var_cth_p_dn4_slot;
        let mut var_cth_p_dn6: f64 = *var_cth_p_dn6_slot;
        let mut var_cth_p_dn7: f64 = *var_cth_p_dn7_slot;
        let mut var_cth_p_dn8: f64 = *var_cth_p_dn8_slot;
        let mut var_cth_p_dn9: f64 = *var_cth_p_dn9_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fntexc_i: f64 = *var_fntexc_i_slot;
        let mut var_fracinv_i: f64 = *var_fracinv_i_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_dn4: f64 = *var_invsa_dn4_slot;
        let mut var_invsa_dn6: f64 = *var_invsa_dn6_slot;
        let mut var_invsa_dn7: f64 = *var_invsa_dn7_slot;
        let mut var_invsa_dn8: f64 = *var_invsa_dn8_slot;
        let mut var_invsa_dn9: f64 = *var_invsa_dn9_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_kdiff_i: f64 = *var_kdiff_i_slot;
        let mut var_kdiff_i_dn4: f64 = *var_kdiff_i_dn4_slot;
        let mut var_kdiff_i_dn6: f64 = *var_kdiff_i_dn6_slot;
        let mut var_kdiff_i_dn7: f64 = *var_kdiff_i_dn7_slot;
        let mut var_kdiff_i_dn8: f64 = *var_kdiff_i_dn8_slot;
        let mut var_kdiff_i_dn9: f64 = *var_kdiff_i_dn9_slot;
        let mut var_kdrift_i: f64 = *var_kdrift_i_slot;
        let mut var_kdrift_i_dn4: f64 = *var_kdrift_i_dn4_slot;
        let mut var_kdrift_i_dn6: f64 = *var_kdrift_i_dn6_slot;
        let mut var_kdrift_i_dn7: f64 = *var_kdrift_i_dn7_slot;
        let mut var_kdrift_i_dn8: f64 = *var_kdrift_i_dn8_slot;
        let mut var_kdrift_i_dn9: f64 = *var_kdrift_i_dn9_slot;
        let mut var_kfracinv_i: f64 = *var_kfracinv_i_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfa_p: f64 = *var_nfa_p_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_rth_p_dn4: f64 = *var_rth_p_dn4_slot;
        let mut var_rth_p_dn6: f64 = *var_rth_p_dn6_slot;
        let mut var_rth_p_dn7: f64 = *var_rth_p_dn7_slot;
        let mut var_rth_p_dn8: f64 = *var_rth_p_dn8_slot;
        let mut var_rth_p_dn9: f64 = *var_rth_p_dn9_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_wx: f64 = *var_wx_slot;

        let (assign5340_e4661, assign5340_e4661_d_n4, assign5340_e4661_d_n6, assign5340_e4661_d_n7, assign5340_e4661_d_n8, assign5340_e4661_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 == 0.0)) {
        let assign5340_e4641: f64 = (var_temp2 - 80.0);
        let assign5340_e4646: f64 = (var_temp2 - 80.0);
        let assign5340_e4647: f64 = (0.5 * assign5340_e4646);
        let assign5340_e4651: f64 = (var_temp2 - 80.0);
        let assign5340_e4653: f64 = (assign5340_e4651 * 0.3333333333333);
        let assign5340_e4654: f64 = (1.0 + assign5340_e4653);
        let assign5340_e4655: f64 = (assign5340_e4647 * assign5340_e4654);
        let assign5340_e4656: f64 = (1.0 + assign5340_e4655);
        let assign5340_e4657: f64 = (assign5340_e4641 * assign5340_e4656);
        let assign5340_e4658: f64 = (1.0 + assign5340_e4657);
        let assign5340_e4659: f64 = (5.54062e34 * assign5340_e4658);
        (assign5340_e4659, (5.54062e34 * ((var_temp2_dn4 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn4) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn6 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn6) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn7 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn7) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn8 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn8) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn9 * assign5340_e4656) + (assign5340_e4641 * (((0.5 * var_temp2_dn9) * assign5340_e4654) + (assign5340_e4647 * (var_temp2_dn9 * 0.3333333333333)))))),)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign5340_e4661;
        var_temp3_dn4 = assign5340_e4661_d_n4;
        var_temp3_dn6 = assign5340_e4661_d_n6;
        var_temp3_dn7 = assign5340_e4661_d_n7;
        var_temp3_dn8 = assign5340_e4661_d_n8;
        var_temp3_dn9 = assign5340_e4661_d_n9;

        let (assign5350_e4670, assign5350_e4670_d_n4, assign5350_e4670_d_n6, assign5350_e4670_d_n7, assign5350_e4670_d_n8, assign5350_e4670_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5350_e4668: f64 = (1.0 - var_temp3);
        (assign5350_e4668, (-var_temp3_dn4), (-var_temp3_dn6), (-var_temp3_dn7), (-var_temp3_dn8), (-var_temp3_dn9),)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign5350_e4670;
        var_temp4_dn4 = assign5350_e4670_d_n4;
        var_temp4_dn6 = assign5350_e4670_d_n6;
        var_temp4_dn7 = assign5350_e4670_d_n7;
        var_temp4_dn8 = assign5350_e4670_d_n8;
        var_temp4_dn9 = assign5350_e4670_d_n9;

        let (assign5360_e4695, assign5360_e4695_d_n4, assign5360_e4695_d_n6, assign5360_e4695_d_n7, assign5360_e4695_d_n8, assign5360_e4695_d_n9,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5360_e4677: f64 = (2.0 * p.p450);
        let assign5360_e4679: f64 = (assign5360_e4677 * var_temp3);
        let assign5360_e4684: f64 = (var_temp3).powf(p.p29);
        let assign5360_e4685: f64 = (1.0 - assign5360_e4684);
        let assign5360_e4687: f64 = (assign5360_e4685 / p.p29);
        let assign5360_e4688: f64 = (var_temp4 - assign5360_e4687);
        let assign5360_e4689: f64 = (assign5360_e4679 * assign5360_e4688);
        let assign5360_e4692: f64 = (var_temp4 * var_temp4);
        let assign5360_e4693: f64 = (assign5360_e4689 / assign5360_e4692);
        (assign5360_e4693, ((((((assign5360_e4677 * var_temp3_dn4) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn4)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn4 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn4 * var_temp4) + (var_temp4 * var_temp4_dn4)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn6) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn6)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn6 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn6 * var_temp4) + (var_temp4 * var_temp4_dn6)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn7) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn7)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn7 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn7 * var_temp4) + (var_temp4 * var_temp4_dn7)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn8) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn8)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn8 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn8 * var_temp4) + (var_temp4 * var_temp4_dn8)))) / (assign5360_e4692 * assign5360_e4692)), ((((((assign5360_e4677 * var_temp3_dn9) * assign5360_e4688) + (assign5360_e4679 * (var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn9)) } } else { (assign5360_e4684 * (p.p29 * (var_temp3_dn9 / var_temp3))) }) / p.p29)))) * assign5360_e4692) - (assign5360_e4689 * ((var_temp4_dn9 * var_temp4) + (var_temp4 * var_temp4_dn9)))) / (assign5360_e4692 * assign5360_e4692)),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign5360_e4695;
        var_temp1_dn4 = assign5360_e4695_d_n4;
        var_temp1_dn6 = assign5360_e4695_d_n6;
        var_temp1_dn7 = assign5360_e4695_d_n7;
        var_temp1_dn8 = assign5360_e4695_d_n8;
        var_temp1_dn9 = assign5360_e4695_d_n9;

        let (assign5370_e4704, assign5370_e4704_d_n4, assign5370_e4704_d_n6, assign5370_e4704_d_n7, assign5370_e4704_d_n8, assign5370_e4704_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5370_e4701: f64 = (1.0 + var_temp1);
        let assign5370_e4702: f64 = (var_temp / assign5370_e4701);
        (assign5370_e4702, (((var_temp_dn4 * assign5370_e4701) - (var_temp * var_temp1_dn4)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn6 * assign5370_e4701) - (var_temp * var_temp1_dn6)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn7 * assign5370_e4701) - (var_temp * var_temp1_dn7)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn8 * assign5370_e4701) - (var_temp * var_temp1_dn8)) / (assign5370_e4701 * assign5370_e4701)), (((var_temp_dn9 * assign5370_e4701) - (var_temp * var_temp1_dn9)) / (assign5370_e4701 * assign5370_e4701)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5370_e4704;
        var_temp_dn4 = assign5370_e4704_d_n4;
        var_temp_dn6 = assign5370_e4704_d_n6;
        var_temp_dn7 = assign5370_e4704_d_n7;
        var_temp_dn8 = assign5370_e4704_d_n8;
        var_temp_dn9 = assign5370_e4704_d_n9;

        let (assign5380_e4711, assign5380_e4711_d_n4, assign5380_e4711_d_n6, assign5380_e4711_d_n7, assign5380_e4711_d_n8, assign5380_e4711_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5380_e4709: f64 = (p.p443 / var_temp);
        (assign5380_e4709, (-((p.p443 * var_temp_dn4) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn6) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn7) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn8) / (var_temp * var_temp))), (-((p.p443 * var_temp_dn9) / (var_temp * var_temp))),)
    } else {
        (var_rth_p, var_rth_p_dn4, var_rth_p_dn6, var_rth_p_dn7, var_rth_p_dn8, var_rth_p_dn9,)
    }
};
        var_rth_p = assign5380_e4711;
        var_rth_p_dn4 = assign5380_e4711_d_n4;
        var_rth_p_dn6 = assign5380_e4711_d_n6;
        var_rth_p_dn7 = assign5380_e4711_d_n7;
        var_rth_p_dn8 = assign5380_e4711_d_n8;
        var_rth_p_dn9 = assign5380_e4711_d_n9;

        let (assign5390_e4718, assign5390_e4718_d_n4, assign5390_e4718_d_n6, assign5390_e4718_d_n7, assign5390_e4718_d_n8, assign5390_e4718_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5390_e4716: f64 = (var_rth_p).max(1e-6);
        (assign5390_e4716, if var_rth_p >= 1e-6 { var_rth_p_dn4 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn6 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn7 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn8 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn9 } else { 0.0 },)
    } else {
        (var_rth_t, var_rth_t_dn4, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9,)
    }
};
        var_rth_t = assign5390_e4718;
        var_rth_t_dn4 = assign5390_e4718_d_n4;
        var_rth_t_dn6 = assign5390_e4718_d_n6;
        var_rth_t_dn7 = assign5390_e4718_d_n7;
        var_rth_t_dn8 = assign5390_e4718_d_n8;
        var_rth_t_dn9 = assign5390_e4718_d_n9;

        let (assign5400_e4723,) = {
    if (var_guard83 == 0.0) {
        (p.p447,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign5400_e4723;

        let (assign5410_e4730, assign5410_e4730_d_n4, assign5410_e4730_d_n6, assign5410_e4730_d_n7, assign5410_e4730_d_n8, assign5410_e4730_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5410_e4728: f64 = (p.p448 * var_temp);
        (assign5410_e4728, (p.p448 * var_temp_dn4), (p.p448 * var_temp_dn6), (p.p448 * var_temp_dn7), (p.p448 * var_temp_dn8), (p.p448 * var_temp_dn9),)
    } else {
        (var_cth_p, var_cth_p_dn4, var_cth_p_dn6, var_cth_p_dn7, var_cth_p_dn8, var_cth_p_dn9,)
    }
};
        var_cth_p = assign5410_e4730;
        var_cth_p_dn4 = assign5410_e4730_d_n4;
        var_cth_p_dn6 = assign5410_e4730_d_n6;
        var_cth_p_dn7 = assign5410_e4730_d_n7;
        var_cth_p_dn8 = assign5410_e4730_d_n8;
        var_cth_p_dn9 = assign5410_e4730_d_n9;

        let (assign5420_e4737, assign5420_e4737_d_n4, assign5420_e4737_d_n6, assign5420_e4737_d_n7, assign5420_e4737_d_n8, assign5420_e4737_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5420_e4735: f64 = (var_cth_p).max(0.0);
        (assign5420_e4735, if var_cth_p >= 0.0 { var_cth_p_dn4 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn6 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn7 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn8 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn9 } else { 0.0 },)
    } else {
        (var_cth_i, var_cth_i_dn4, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9,)
    }
};
        var_cth_i = assign5420_e4737;
        var_cth_i_dn4 = assign5420_e4737_d_n4;
        var_cth_i_dn6 = assign5420_e4737_d_n6;
        var_cth_i_dn7 = assign5420_e4737_d_n7;
        var_cth_i_dn8 = assign5420_e4737_d_n8;
        var_cth_i_dn9 = assign5420_e4737_d_n9;

        let (assign5430_e4742,) = {
    if (var_guard83 == 0.0) {
        (p.p451,)
    } else {
        (var_fnt_i,)
    }
};
        var_fnt_i = assign5430_e4742;

        let (assign5440_e4761,) = {
    if (var_guard83 == 0.0) {
        let assign5440_e4747: f64 = (p.p452 * var_betn_p);
        let assign5440_e4749: f64 = (assign5440_e4747 * var_betn_p);
        let assign5440_e4751: f64 = (assign5440_e4749 * var_iwe);
        let assign5440_e4753: f64 = (assign5440_e4751 * var_iwe);
        let assign5440_e4757: f64 = (p.p453 - 2.0);
        let assign5440_e4758: f64 = (var_ile).powf(assign5440_e4757);
        let assign5440_e4759: f64 = (assign5440_e4753 * assign5440_e4758);
        (assign5440_e4759,)
    } else {
        (var_fntexc_i,)
    }
};
        var_fntexc_i = assign5440_e4761;

        let (assign5450_e4772,) = {
    if (var_guard83 == 0.0) {
        let assign5450_e4766: f64 = (p.p454 * var_iae);
        let assign5450_e4769: f64 = (p.p455 * var_iwe);
        let assign5450_e4770: f64 = (assign5450_e4766 + assign5450_e4769);
        (assign5450_e4770,)
    } else {
        (var_nfa_p,)
    }
};
        var_nfa_p = assign5450_e4772;

        let (assign5460_e4779,) = {
    if (var_guard83 == 0.0) {
        let assign5460_e4777: f64 = (var_nfa_p).max(0.0);
        (assign5460_e4777,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign5460_e4779;

        let (assign5470_e4786,) = {
    if (var_guard83 == 0.0) {
        let assign5470_e4784: f64 = (p.p456 * var_iae);
        (assign5470_e4784,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign5470_e4786;

        let (assign5480_e4793,) = {
    if (var_guard83 == 0.0) {
        let assign5480_e4791: f64 = (p.p457 * var_iae);
        (assign5480_e4791,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign5480_e4793;

        let (assign5490_e4798,) = {
    if (var_guard83 == 0.0) {
        (p.p458,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign5490_e4798;

        let (assign5500_e4803,) = {
    if (var_guard83 == 0.0) {
        (p.p459,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign5500_e4803;

        let (assign5520_e4817, assign5520_e4817_d_n4, assign5520_e4817_d_n6, assign5520_e4817_d_n7, assign5520_e4817_d_n8, assign5520_e4817_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5520_e4814: f64 = (p.p490 * var_ile);
        let assign5520_e4815: f64 = (p.p489 + assign5520_e4814);
        (assign5520_e4815, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5520_e4817;
        var_temp_dn4 = assign5520_e4817_d_n4;
        var_temp_dn6 = assign5520_e4817_d_n6;
        var_temp_dn7 = assign5520_e4817_d_n7;
        var_temp_dn8 = assign5520_e4817_d_n8;
        var_temp_dn9 = assign5520_e4817_d_n9;

        let (assign5530_e4824, assign5530_e4824_d_n4, assign5530_e4824_d_n6, assign5530_e4824_d_n7, assign5530_e4824_d_n8, assign5530_e4824_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5530_e4822: f64 = (var_temp).max(0.0);
        (assign5530_e4822, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_kdrift_i, var_kdrift_i_dn4, var_kdrift_i_dn6, var_kdrift_i_dn7, var_kdrift_i_dn8, var_kdrift_i_dn9,)
    }
};
        var_kdrift_i = assign5530_e4824;
        var_kdrift_i_dn4 = assign5530_e4824_d_n4;
        var_kdrift_i_dn6 = assign5530_e4824_d_n6;
        var_kdrift_i_dn7 = assign5530_e4824_d_n7;
        var_kdrift_i_dn8 = assign5530_e4824_d_n8;
        var_kdrift_i_dn9 = assign5530_e4824_d_n9;

        let (assign5540_e4833, assign5540_e4833_d_n4, assign5540_e4833_d_n6, assign5540_e4833_d_n7, assign5540_e4833_d_n8, assign5540_e4833_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5540_e4830: f64 = (p.p492 * var_ile);
        let assign5540_e4831: f64 = (p.p491 + assign5540_e4830);
        (assign5540_e4831, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5540_e4833;
        var_temp_dn4 = assign5540_e4833_d_n4;
        var_temp_dn6 = assign5540_e4833_d_n6;
        var_temp_dn7 = assign5540_e4833_d_n7;
        var_temp_dn8 = assign5540_e4833_d_n8;
        var_temp_dn9 = assign5540_e4833_d_n9;

        let (assign5550_e4840, assign5550_e4840_d_n4, assign5550_e4840_d_n6, assign5550_e4840_d_n7, assign5550_e4840_d_n8, assign5550_e4840_d_n9,) = {
    if (var_guard83 == 0.0) {
        let assign5550_e4838: f64 = (var_temp).max(0.0);
        (assign5550_e4838, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 },)
    } else {
        (var_kdiff_i, var_kdiff_i_dn4, var_kdiff_i_dn6, var_kdiff_i_dn7, var_kdiff_i_dn8, var_kdiff_i_dn9,)
    }
};
        var_kdiff_i = assign5550_e4840;
        var_kdiff_i_dn4 = assign5550_e4840_d_n4;
        var_kdiff_i_dn6 = assign5550_e4840_d_n6;
        var_kdiff_i_dn7 = assign5550_e4840_d_n7;
        var_kdiff_i_dn8 = assign5550_e4840_d_n8;
        var_kdiff_i_dn9 = assign5550_e4840_d_n9;

        let (assign5560_e4845,) = {
    if (var_guard83 == 0.0) {
        (p.p493,)
    } else {
        (var_fracinv_i,)
    }
};
        var_fracinv_i = assign5560_e4845;

        let (assign5570_e4850,) = {
    if (var_guard83 == 0.0) {
        (p.p494,)
    } else {
        (var_kfracinv_i,)
    }
};
        var_kfracinv_i = assign5570_e4850;

        let assign5670_e4958: f64 = if ((((p.p461 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard133 = assign5670_e4958;

        let assign5680_e4961: f64 = if p.p461 == 1.0 { 1.0 } else { 0.0 };
        var_guard134 = assign5680_e4961;

        let (assign5690_e4970, assign5690_e4970_d_n4, assign5690_e4970_d_n6, assign5690_e4970_d_n7, assign5690_e4970_d_n8, assign5690_e4970_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
        var_tmpa = assign5690_e4970;
        var_tmpa_dn4 = assign5690_e4970_d_n4;
        var_tmpa_dn6 = assign5690_e4970_d_n6;
        var_tmpa_dn7 = assign5690_e4970_d_n7;
        var_tmpa_dn8 = assign5690_e4970_d_n8;
        var_tmpa_dn9 = assign5690_e4970_d_n9;

        let (assign5700_e4979,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign5700_e4979;

        let (assign5710_e4988,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign5710_e4988;

        let mut assign5720_loop_guard: usize = 0;
        while {
            let assign5720_cond_e4998: f64 = (p.p29 - 0.5);
            let assign5720_cond_e5000: f64 = if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) && (var_iloop < assign5720_cond_e4998)) { 1.0 } else { 0.0 };
            assign5720_cond_e5000 != 0.0
        } {
            assign5720_loop_guard += 1;
            assert!(assign5720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5720_body0_e5023, assign5720_body0_e5023_d_n4, assign5720_body0_e5023_d_n6, assign5720_body0_e5023_d_n7, assign5720_body0_e5023_d_n8, assign5720_body0_e5023_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body0_e5012: f64 = (0.5 * p.p20);
        let assign5720_body0_e5013: f64 = (p.p26 + assign5720_body0_e5012);
        let assign5720_body0_e5017: f64 = (p.p28 + p.p20);
        let assign5720_body0_e5018: f64 = (var_iloop * assign5720_body0_e5017);
        let assign5720_body0_e5019: f64 = (assign5720_body0_e5013 + assign5720_body0_e5018);
        let assign5720_body0_e5020: f64 = (1.0 / assign5720_body0_e5019);
        let assign5720_body0_e5021: f64 = (var_tmpa + assign5720_body0_e5020);
        (assign5720_body0_e5021, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
            var_tmpa = assign5720_body0_e5023;
            var_tmpa_dn4 = assign5720_body0_e5023_d_n4;
            var_tmpa_dn6 = assign5720_body0_e5023_d_n6;
            var_tmpa_dn7 = assign5720_body0_e5023_d_n7;
            var_tmpa_dn8 = assign5720_body0_e5023_d_n8;
            var_tmpa_dn9 = assign5720_body0_e5023_d_n9;
            let (assign5720_body1_e5046,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body1_e5035: f64 = (0.5 * p.p20);
        let assign5720_body1_e5036: f64 = (p.p27 + assign5720_body1_e5035);
        let assign5720_body1_e5040: f64 = (p.p28 + p.p20);
        let assign5720_body1_e5041: f64 = (var_iloop * assign5720_body1_e5040);
        let assign5720_body1_e5042: f64 = (assign5720_body1_e5036 + assign5720_body1_e5041);
        let assign5720_body1_e5043: f64 = (1.0 / assign5720_body1_e5042);
        let assign5720_body1_e5044: f64 = (var_tmpb + assign5720_body1_e5043);
        (assign5720_body1_e5044,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign5720_body1_e5046;
            let (assign5720_body2_e5057,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_body2_e5055: f64 = (var_iloop + 1.0);
        (assign5720_body2_e5055,)
    } else {
        (var_iloop,)
    }
};
            var_iloop = assign5720_body2_e5057;
        }

        let (assign5730_e5068, assign5730_e5068_d_n4, assign5730_e5068_d_n6, assign5730_e5068_d_n7, assign5730_e5068_d_n8, assign5730_e5068_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5730_e5066: f64 = (var_tmpa / p.p29);
        (assign5730_e5066, (var_tmpa_dn4 / p.p29), (var_tmpa_dn6 / p.p29), (var_tmpa_dn7 / p.p29), (var_tmpa_dn8 / p.p29), (var_tmpa_dn9 / p.p29),)
    } else {
        (var_invsa, var_invsa_dn4, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9,)
    }
};
        var_invsa = assign5730_e5068;
        var_invsa_dn4 = assign5730_e5068_d_n4;
        var_invsa_dn6 = assign5730_e5068_d_n6;
        var_invsa_dn7 = assign5730_e5068_d_n7;
        var_invsa_dn8 = assign5730_e5068_d_n8;
        var_invsa_dn9 = assign5730_e5068_d_n9;

        let (assign5740_e5079,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5740_e5077: f64 = (var_tmpb / p.p29);
        (assign5740_e5077,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign5740_e5079;

        let (assign5750_e5094,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5750_e5090: f64 = (0.5 * p.p20);
        let assign5750_e5091: f64 = (p.p462 + assign5750_e5090);
        let assign5750_e5092: f64 = (1.0 / assign5750_e5091);
        (assign5750_e5092,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign5750_e5094;

        let (assign5760_e5109,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5760_e5105: f64 = (0.5 * p.p20);
        let assign5760_e5106: f64 = (p.p463 + assign5760_e5105);
        let assign5760_e5107: f64 = (1.0 / assign5760_e5106);
        (assign5760_e5107,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign5760_e5109;

        let (assign5770_e5122,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5770_e5118: f64 = (p.p20 + var_dellps);
        let assign5770_e5120: f64 = (assign5770_e5118).max(1e-9);
        (assign5770_e5120,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign5770_e5122;

        let (assign5780_e5137,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5780_e5131: f64 = (var_w_i + var_delwod);
        let assign5780_e5133: f64 = (assign5780_e5131 + p.p464);
        let assign5780_e5135: f64 = (assign5780_e5133).max(1e-9);
        (assign5780_e5135,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign5780_e5137;

        let (assign5790_e5150,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5790_e5147: f64 = (var_lx).powf(p.p471);
        let assign5790_e5148: f64 = (1.0 / assign5790_e5147);
        (assign5790_e5148,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5790_e5150;

        let (assign5800_e5163,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5800_e5160: f64 = (var_wx).powf(p.p472);
        let assign5800_e5161: f64 = (1.0 / assign5800_e5160);
        (assign5800_e5161,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5800_e5163;

        *var_cth_i_slot = var_cth_i;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_dn4_slot = var_cth_p_dn4;
        *var_cth_p_dn6_slot = var_cth_p_dn6;
        *var_cth_p_dn7_slot = var_cth_p_dn7;
        *var_cth_p_dn8_slot = var_cth_p_dn8;
        *var_cth_p_dn9_slot = var_cth_p_dn9;
        *var_fnt_i_slot = var_fnt_i;
        *var_fntexc_i_slot = var_fntexc_i;
        *var_fracinv_i_slot = var_fracinv_i;
        *var_guard133_slot = var_guard133;
        *var_guard134_slot = var_guard134;
        *var_iloop_slot = var_iloop;
        *var_invsa_slot = var_invsa;
        *var_invsa_dn4_slot = var_invsa_dn4;
        *var_invsa_dn6_slot = var_invsa_dn6;
        *var_invsa_dn7_slot = var_invsa_dn7;
        *var_invsa_dn8_slot = var_invsa_dn8;
        *var_invsa_dn9_slot = var_invsa_dn9;
        *var_invsaref_slot = var_invsaref;
        *var_invsb_slot = var_invsb;
        *var_invsbref_slot = var_invsbref;
        *var_kdiff_i_slot = var_kdiff_i;
        *var_kdiff_i_dn4_slot = var_kdiff_i_dn4;
        *var_kdiff_i_dn6_slot = var_kdiff_i_dn6;
        *var_kdiff_i_dn7_slot = var_kdiff_i_dn7;
        *var_kdiff_i_dn8_slot = var_kdiff_i_dn8;
        *var_kdiff_i_dn9_slot = var_kdiff_i_dn9;
        *var_kdrift_i_slot = var_kdrift_i;
        *var_kdrift_i_dn4_slot = var_kdrift_i_dn4;
        *var_kdrift_i_dn6_slot = var_kdrift_i_dn6;
        *var_kdrift_i_dn7_slot = var_kdrift_i_dn7;
        *var_kdrift_i_dn8_slot = var_kdrift_i_dn8;
        *var_kdrift_i_dn9_slot = var_kdrift_i_dn9;
        *var_kfracinv_i_slot = var_kfracinv_i;
        *var_lx_slot = var_lx;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfa_p_slot = var_nfa_p;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_rth_p_slot = var_rth_p;
        *var_rth_p_dn4_slot = var_rth_p_dn4;
        *var_rth_p_dn6_slot = var_rth_p_dn6;
        *var_rth_p_dn7_slot = var_rth_p_dn7;
        *var_rth_p_dn8_slot = var_rth_p_dn8;
        *var_rth_p_dn9_slot = var_rth_p_dn9;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_strth_i_slot = var_strth_i;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_templ_slot = var_templ;
        *var_tempw_slot = var_tempw;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpb_slot = var_tmpb;
        *var_wx_slot = var_wx;
    }

    pub(super) fn stamp_transient_block_12(
        p: &Parameters,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_invsa: f64,
        var_invsa_dn4: f64,
        var_invsa_dn6: f64,
        var_invsa_dn7: f64,
        var_invsa_dn8: f64,
        var_invsa_dn9: f64,
        var_invsaref: f64,
        var_invsb: f64,
        var_invsbref: f64,
        var_lx: f64,
        var_rt: f64,
        var_rt_dn4: f64,
        var_rt_dn6: f64,
        var_rt_dn7: f64,
        var_rt_dn8: f64,
        var_rt_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_wx: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_dn4_slot: &mut f64,
        var_kstressu0_dn6_slot: &mut f64,
        var_kstressu0_dn7_slot: &mut f64,
        var_kstressu0_dn8_slot: &mut f64,
        var_kstressu0_dn9_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_dn4_slot: &mut f64,
        var_rhobeta_dn6_slot: &mut f64,
        var_rhobeta_dn7_slot: &mut f64,
        var_rhobeta_dn8_slot: &mut f64,
        var_rhobeta_dn9_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_dn4_slot: &mut f64,
        var_rhobetaref_dn6_slot: &mut f64,
        var_rhobetaref_dn7_slot: &mut f64,
        var_rhobetaref_dn8_slot: &mut f64,
        var_rhobetaref_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_dn4: f64 = *var_kstressu0_dn4_slot;
        let mut var_kstressu0_dn6: f64 = *var_kstressu0_dn6_slot;
        let mut var_kstressu0_dn7: f64 = *var_kstressu0_dn7_slot;
        let mut var_kstressu0_dn8: f64 = *var_kstressu0_dn8_slot;
        let mut var_kstressu0_dn9: f64 = *var_kstressu0_dn9_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_dn4: f64 = *var_rhobeta_dn4_slot;
        let mut var_rhobeta_dn6: f64 = *var_rhobeta_dn6_slot;
        let mut var_rhobeta_dn7: f64 = *var_rhobeta_dn7_slot;
        let mut var_rhobeta_dn8: f64 = *var_rhobeta_dn8_slot;
        let mut var_rhobeta_dn9: f64 = *var_rhobeta_dn9_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_dn4: f64 = *var_rhobetaref_dn4_slot;
        let mut var_rhobetaref_dn6: f64 = *var_rhobetaref_dn6_slot;
        let mut var_rhobetaref_dn7: f64 = *var_rhobetaref_dn7_slot;
        let mut var_rhobetaref_dn8: f64 = *var_rhobetaref_dn8_slot;
        let mut var_rhobetaref_dn9: f64 = *var_rhobetaref_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign5810_e5194, assign5810_e5194_d_n4, assign5810_e5194_d_n6, assign5810_e5194_d_n7, assign5810_e5194_d_n8, assign5810_e5194_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5810_e5173: f64 = (p.p468 * var_templ);
        let assign5810_e5174: f64 = (1.0 + assign5810_e5173);
        let assign5810_e5177: f64 = (p.p469 * var_tempw);
        let assign5810_e5178: f64 = (assign5810_e5174 + assign5810_e5177);
        let assign5810_e5181: f64 = (p.p470 * var_templ);
        let assign5810_e5183: f64 = (assign5810_e5181 * var_tempw);
        let assign5810_e5184: f64 = (assign5810_e5178 + assign5810_e5183);
        let assign5810_e5189: f64 = (var_rt - 1.0);
        let assign5810_e5190: f64 = (p.p467 * assign5810_e5189);
        let assign5810_e5191: f64 = (1.0 + assign5810_e5190);
        let assign5810_e5192: f64 = (assign5810_e5184 * assign5810_e5191);
        (assign5810_e5192, (assign5810_e5184 * (p.p467 * var_rt_dn4)), (assign5810_e5184 * (p.p467 * var_rt_dn6)), (assign5810_e5184 * (p.p467 * var_rt_dn7)), (assign5810_e5184 * (p.p467 * var_rt_dn8)), (assign5810_e5184 * (p.p467 * var_rt_dn9)),)
    } else {
        (var_kstressu0, var_kstressu0_dn4, var_kstressu0_dn6, var_kstressu0_dn7, var_kstressu0_dn8, var_kstressu0_dn9,)
    }
};
        var_kstressu0 = assign5810_e5194;
        var_kstressu0_dn4 = assign5810_e5194_d_n4;
        var_kstressu0_dn6 = assign5810_e5194_d_n6;
        var_kstressu0_dn7 = assign5810_e5194_d_n7;
        var_kstressu0_dn8 = assign5810_e5194_d_n8;
        var_kstressu0_dn9 = assign5810_e5194_d_n9;

        let (assign5820_e5209, assign5820_e5209_d_n4, assign5820_e5209_d_n6, assign5820_e5209_d_n7, assign5820_e5209_d_n8, assign5820_e5209_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5820_e5204: f64 = (var_invsa + var_invsb);
        let assign5820_e5205: f64 = (p.p465 * assign5820_e5204);
        let assign5820_e5207: f64 = (assign5820_e5205 / var_kstressu0);
        (assign5820_e5207, ((((p.p465 * var_invsa_dn4) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn4)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn6) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn6)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn7) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn7)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn8) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn8)) / (var_kstressu0 * var_kstressu0)), ((((p.p465 * var_invsa_dn9) * var_kstressu0) - (assign5820_e5205 * var_kstressu0_dn9)) / (var_kstressu0 * var_kstressu0)),)
    } else {
        (var_rhobeta, var_rhobeta_dn4, var_rhobeta_dn6, var_rhobeta_dn7, var_rhobeta_dn8, var_rhobeta_dn9,)
    }
};
        var_rhobeta = assign5820_e5209;
        var_rhobeta_dn4 = assign5820_e5209_d_n4;
        var_rhobeta_dn6 = assign5820_e5209_d_n6;
        var_rhobeta_dn7 = assign5820_e5209_d_n7;
        var_rhobeta_dn8 = assign5820_e5209_d_n8;
        var_rhobeta_dn9 = assign5820_e5209_d_n9;

        let (assign5830_e5224, assign5830_e5224_d_n4, assign5830_e5224_d_n6, assign5830_e5224_d_n7, assign5830_e5224_d_n8, assign5830_e5224_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5830_e5219: f64 = (var_invsaref + var_invsbref);
        let assign5830_e5220: f64 = (p.p465 * assign5830_e5219);
        let assign5830_e5222: f64 = (assign5830_e5220 / var_kstressu0);
        (assign5830_e5222, (-((assign5830_e5220 * var_kstressu0_dn4) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn6) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn7) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn8) / (var_kstressu0 * var_kstressu0))), (-((assign5830_e5220 * var_kstressu0_dn9) / (var_kstressu0 * var_kstressu0))),)
    } else {
        (var_rhobetaref, var_rhobetaref_dn4, var_rhobetaref_dn6, var_rhobetaref_dn7, var_rhobetaref_dn8, var_rhobetaref_dn9,)
    }
};
        var_rhobetaref = assign5830_e5224;
        var_rhobetaref_dn4 = assign5830_e5224_d_n4;
        var_rhobetaref_dn6 = assign5830_e5224_d_n6;
        var_rhobetaref_dn7 = assign5830_e5224_d_n7;
        var_rhobetaref_dn8 = assign5830_e5224_d_n8;
        var_rhobetaref_dn9 = assign5830_e5224_d_n9;

        let (assign5840_e5237,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5840_e5234: f64 = (var_lx).powf(p.p477);
        let assign5840_e5235: f64 = (1.0 / assign5840_e5234);
        (assign5840_e5235,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5840_e5237;

        let (assign5850_e5250,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5850_e5247: f64 = (var_wx).powf(p.p478);
        let assign5850_e5248: f64 = (1.0 / assign5850_e5247);
        (assign5850_e5248,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5850_e5250;

        let (assign5860_e5275,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5860_e5260: f64 = (p.p474 * var_templ);
        let assign5860_e5261: f64 = (1.0 + assign5860_e5260);
        let assign5860_e5264: f64 = (p.p475 * var_tempw);
        let assign5860_e5265: f64 = (assign5860_e5261 + assign5860_e5264);
        let assign5860_e5268: f64 = (p.p476 * var_templ);
        let assign5860_e5270: f64 = (assign5860_e5268 * var_tempw);
        let assign5860_e5271: f64 = (assign5860_e5265 + assign5860_e5270);
        let assign5860_e5273: f64 = (assign5860_e5271).max(1e-20);
        (assign5860_e5273,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign5860_e5275;

        let (assign5870_e5290, assign5870_e5290_d_n4, assign5870_e5290_d_n6, assign5870_e5290_d_n7, assign5870_e5290_d_n8, assign5870_e5290_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5870_e5284: f64 = (var_invsa + var_invsb);
        let assign5870_e5286: f64 = (assign5870_e5284 - var_invsaref);
        let assign5870_e5288: f64 = (assign5870_e5286 - var_invsbref);
        (assign5870_e5288, var_invsa_dn4, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign5870_e5290;
        var_temp0__blk79_dn4 = assign5870_e5290_d_n4;
        var_temp0__blk79_dn6 = assign5870_e5290_d_n6;
        var_temp0__blk79_dn7 = assign5870_e5290_d_n7;
        var_temp0__blk79_dn8 = assign5870_e5290_d_n8;
        var_temp0__blk79_dn9 = assign5870_e5290_d_n9;

        let (assign5880_e5307, assign5880_e5307_d_n4, assign5880_e5307_d_n6, assign5880_e5307_d_n7, assign5880_e5307_d_n8, assign5880_e5307_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5880_e5300: f64 = (1.0 + var_rhobeta);
        let assign5880_e5301: f64 = (var_betn_p * assign5880_e5300);
        let assign5880_e5304: f64 = (1.0 + var_rhobetaref);
        let assign5880_e5305: f64 = (assign5880_e5301 / assign5880_e5304);
        (assign5880_e5305, (((((var_betn_p_dn4 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn4)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn4)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn6 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn6)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn6)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn7 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn7)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn7)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn8 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn8)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn8)) / (assign5880_e5304 * assign5880_e5304)), (((((var_betn_p_dn9 * assign5880_e5300) + (var_betn_p * var_rhobeta_dn9)) * assign5880_e5304) - (assign5880_e5301 * var_rhobetaref_dn9)) / (assign5880_e5304 * assign5880_e5304)),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign5880_e5307;
        var_betn_p_dn4 = assign5880_e5307_d_n4;
        var_betn_p_dn6 = assign5880_e5307_d_n6;
        var_betn_p_dn7 = assign5880_e5307_d_n7;
        var_betn_p_dn8 = assign5880_e5307_d_n8;
        var_betn_p_dn9 = assign5880_e5307_d_n9;

        let (assign5890_e5318, assign5890_e5318_d_n4, assign5890_e5318_d_n6, assign5890_e5318_d_n7, assign5890_e5318_d_n8, assign5890_e5318_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5890_e5316: f64 = (var_betn_p).max(1e-10);
        (assign5890_e5316, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign5890_e5318;
        var_betn1_t_dn4 = assign5890_e5318_d_n4;
        var_betn1_t_dn6 = assign5890_e5318_d_n6;
        var_betn1_t_dn7 = assign5890_e5318_d_n7;
        var_betn1_t_dn8 = assign5890_e5318_d_n8;
        var_betn1_t_dn9 = assign5890_e5318_d_n9;

        let (assign5900_e5329, assign5900_e5329_d_n4, assign5900_e5329_d_n6, assign5900_e5329_d_n7, assign5900_e5329_d_n8, assign5900_e5329_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5900_e5327: f64 = (p.p254 * var_betn1_t);
        (assign5900_e5327, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign5900_e5329;
        var_betn2_t_dn4 = assign5900_e5329_d_n4;
        var_betn2_t_dn6 = assign5900_e5329_d_n6;
        var_betn2_t_dn7 = assign5900_e5329_d_n7;
        var_betn2_t_dn8 = assign5900_e5329_d_n8;
        var_betn2_t_dn9 = assign5900_e5329_d_n9;

        let (assign5910_e5356, assign5910_e5356_d_n4, assign5910_e5356_d_n6, assign5910_e5356_d_n7, assign5910_e5356_d_n8, assign5910_e5356_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5910_e5338: f64 = (1.0 + var_rhobeta);
        let assign5910_e5342: f64 = (p.p466 * var_rhobetaref);
        let assign5910_e5343: f64 = (1.0 + assign5910_e5342);
        let assign5910_e5344: f64 = (assign5910_e5338 * assign5910_e5343);
        let assign5910_e5347: f64 = (1.0 + var_rhobetaref);
        let assign5910_e5351: f64 = (p.p466 * var_rhobeta);
        let assign5910_e5352: f64 = (1.0 + assign5910_e5351);
        let assign5910_e5353: f64 = (assign5910_e5347 * assign5910_e5352);
        let assign5910_e5354: f64 = (assign5910_e5344 / assign5910_e5353);
        (assign5910_e5354, (((((var_rhobeta_dn4 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn4))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn4 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn4))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn6 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn6))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn6 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn6))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn7 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn7))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn7 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn7))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn8 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn8))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn8 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn8))))) / (assign5910_e5353 * assign5910_e5353)), (((((var_rhobeta_dn9 * assign5910_e5343) + (assign5910_e5338 * (p.p466 * var_rhobetaref_dn9))) * assign5910_e5353) - (assign5910_e5344 * ((var_rhobetaref_dn9 * assign5910_e5352) + (assign5910_e5347 * (p.p466 * var_rhobeta_dn9))))) / (assign5910_e5353 * assign5910_e5353)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5910_e5356;
        var_temp_dn4 = assign5910_e5356_d_n4;
        var_temp_dn6 = assign5910_e5356_d_n6;
        var_temp_dn7 = assign5910_e5356_d_n7;
        var_temp_dn8 = assign5910_e5356_d_n8;
        var_temp_dn9 = assign5910_e5356_d_n9;

        let (assign5920_e5367, assign5920_e5367_d_n4, assign5920_e5367_d_n6, assign5920_e5367_d_n7, assign5920_e5367_d_n8, assign5920_e5367_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5920_e5365: f64 = (var_thesat_p * var_temp);
        (assign5920_e5365, ((var_thesat_p_dn4 * var_temp) + (var_thesat_p * var_temp_dn4)), ((var_thesat_p_dn6 * var_temp) + (var_thesat_p * var_temp_dn6)), ((var_thesat_p_dn7 * var_temp) + (var_thesat_p * var_temp_dn7)), ((var_thesat_p_dn8 * var_temp) + (var_thesat_p * var_temp_dn8)), ((var_thesat_p_dn9 * var_temp) + (var_thesat_p * var_temp_dn9)),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign5920_e5367;
        var_thesat_p_dn4 = assign5920_e5367_d_n4;
        var_thesat_p_dn6 = assign5920_e5367_d_n6;
        var_thesat_p_dn7 = assign5920_e5367_d_n7;
        var_thesat_p_dn8 = assign5920_e5367_d_n8;
        var_thesat_p_dn9 = assign5920_e5367_d_n9;

        let (assign5930_e5378, assign5930_e5378_d_n4, assign5930_e5378_d_n6, assign5930_e5378_d_n7, assign5930_e5378_d_n8, assign5930_e5378_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5930_e5376: f64 = (var_thesat_p).max(0.0);
        (assign5930_e5376, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign5930_e5378;
        var_thesat_t_dn4 = assign5930_e5378_d_n4;
        var_thesat_t_dn6 = assign5930_e5378_d_n6;
        var_thesat_t_dn7 = assign5930_e5378_d_n7;
        var_thesat_t_dn8 = assign5930_e5378_d_n8;
        var_thesat_t_dn9 = assign5930_e5378_d_n9;

        let (assign5940_e5389, assign5940_e5389_d_n4, assign5940_e5389_d_n6, assign5940_e5389_d_n7, assign5940_e5389_d_n8, assign5940_e5389_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5940_e5387: f64 = (var_thesatac_p * var_temp);
        (assign5940_e5387, ((var_thesatac_p_dn4 * var_temp) + (var_thesatac_p * var_temp_dn4)), ((var_thesatac_p_dn6 * var_temp) + (var_thesatac_p * var_temp_dn6)), ((var_thesatac_p_dn7 * var_temp) + (var_thesatac_p * var_temp_dn7)), ((var_thesatac_p_dn8 * var_temp) + (var_thesatac_p * var_temp_dn8)), ((var_thesatac_p_dn9 * var_temp) + (var_thesatac_p * var_temp_dn9)),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign5940_e5389;
        var_thesatac_p_dn4 = assign5940_e5389_d_n4;
        var_thesatac_p_dn6 = assign5940_e5389_d_n6;
        var_thesatac_p_dn7 = assign5940_e5389_d_n7;
        var_thesatac_p_dn8 = assign5940_e5389_d_n8;
        var_thesatac_p_dn9 = assign5940_e5389_d_n9;

        let (assign5950_e5400, assign5950_e5400_d_n4, assign5950_e5400_d_n6, assign5950_e5400_d_n7, assign5950_e5400_d_n8, assign5950_e5400_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5950_e5398: f64 = (var_thesatac_p).max(0.0);
        (assign5950_e5398, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign5950_e5400;
        var_thesatac_t_dn4 = assign5950_e5400_d_n4;
        var_thesatac_t_dn6 = assign5950_e5400_d_n6;
        var_thesatac_t_dn7 = assign5950_e5400_d_n7;
        var_thesatac_t_dn8 = assign5950_e5400_d_n8;
        var_thesatac_t_dn9 = assign5950_e5400_d_n9;

        let (assign5960_e5413, assign5960_e5413_d_n4, assign5960_e5413_d_n6, assign5960_e5413_d_n7, assign5960_e5413_d_n8, assign5960_e5413_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5960_e5409: f64 = (p.p473 * var_temp0__blk79);
        let assign5960_e5411: f64 = (assign5960_e5409 / var_kstressvth0);
        (assign5960_e5411, ((p.p473 * var_temp0__blk79_dn4) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn6) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn7) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn8) / var_kstressvth0), ((p.p473 * var_temp0__blk79_dn9) / var_kstressvth0),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign5960_e5413;
        var_temp_dn4 = assign5960_e5413_d_n4;
        var_temp_dn6 = assign5960_e5413_d_n6;
        var_temp_dn7 = assign5960_e5413_d_n7;
        var_temp_dn8 = assign5960_e5413_d_n8;
        var_temp_dn9 = assign5960_e5413_d_n9;

        let (assign5970_e5424, assign5970_e5424_d_n4, assign5970_e5424_d_n6, assign5970_e5424_d_n7, assign5970_e5424_d_n8, assign5970_e5424_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5970_e5422: f64 = (var_vfb1_t + var_temp);
        (assign5970_e5422, (var_vfb1_t_dn4 + var_temp_dn4), (var_vfb1_t_dn6 + var_temp_dn6), (var_vfb1_t_dn7 + var_temp_dn7), (var_vfb1_t_dn8 + var_temp_dn8), (var_vfb1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign5970_e5424;
        var_vfb1_t_dn4 = assign5970_e5424_d_n4;
        var_vfb1_t_dn6 = assign5970_e5424_d_n6;
        var_vfb1_t_dn7 = assign5970_e5424_d_n7;
        var_vfb1_t_dn8 = assign5970_e5424_d_n8;
        var_vfb1_t_dn9 = assign5970_e5424_d_n9;

        let (assign5980_e5435, assign5980_e5435_d_n4, assign5980_e5435_d_n6, assign5980_e5435_d_n7, assign5980_e5435_d_n8, assign5980_e5435_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5980_e5433: f64 = (var_vfb2_t + var_temp);
        (assign5980_e5433, (var_vfb2_t_dn4 + var_temp_dn4), (var_vfb2_t_dn6 + var_temp_dn6), (var_vfb2_t_dn7 + var_temp_dn7), (var_vfb2_t_dn8 + var_temp_dn8), (var_vfb2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign5980_e5435;
        var_vfb2_t_dn4 = assign5980_e5435_d_n4;
        var_vfb2_t_dn6 = assign5980_e5435_d_n6;
        var_vfb2_t_dn7 = assign5980_e5435_d_n7;
        var_vfb2_t_dn8 = assign5980_e5435_d_n8;
        var_vfb2_t_dn9 = assign5980_e5435_d_n9;

        let (assign5990_e5446, assign5990_e5446_d_n4, assign5990_e5446_d_n6, assign5990_e5446_d_n7, assign5990_e5446_d_n8, assign5990_e5446_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5990_e5444: f64 = (var_vfbac1_t + var_temp);
        (assign5990_e5444, (var_vfbac1_t_dn4 + var_temp_dn4), (var_vfbac1_t_dn6 + var_temp_dn6), (var_vfbac1_t_dn7 + var_temp_dn7), (var_vfbac1_t_dn8 + var_temp_dn8), (var_vfbac1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign5990_e5446;
        var_vfbac1_t_dn4 = assign5990_e5446_d_n4;
        var_vfbac1_t_dn6 = assign5990_e5446_d_n6;
        var_vfbac1_t_dn7 = assign5990_e5446_d_n7;
        var_vfbac1_t_dn8 = assign5990_e5446_d_n8;
        var_vfbac1_t_dn9 = assign5990_e5446_d_n9;

        let (assign6000_e5457, assign6000_e5457_d_n4, assign6000_e5457_d_n6, assign6000_e5457_d_n7, assign6000_e5457_d_n8, assign6000_e5457_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6000_e5455: f64 = (var_vfbac2_t + var_temp);
        (assign6000_e5455, (var_vfbac2_t_dn4 + var_temp_dn4), (var_vfbac2_t_dn6 + var_temp_dn6), (var_vfbac2_t_dn7 + var_temp_dn7), (var_vfbac2_t_dn8 + var_temp_dn8), (var_vfbac2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign6000_e5457;
        var_vfbac2_t_dn4 = assign6000_e5457_d_n4;
        var_vfbac2_t_dn6 = assign6000_e5457_d_n6;
        var_vfbac2_t_dn7 = assign6000_e5457_d_n7;
        var_vfbac2_t_dn8 = assign6000_e5457_d_n8;
        var_vfbac2_t_dn9 = assign6000_e5457_d_n9;

        let (assign6010_e5472, assign6010_e5472_d_n4, assign6010_e5472_d_n6, assign6010_e5472_d_n7, assign6010_e5472_d_n8, assign6010_e5472_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6010_e5466: f64 = (p.p479 * var_temp0__blk79);
        let assign6010_e5469: f64 = (var_kstressvth0).powf(p.p480);
        let assign6010_e5470: f64 = (assign6010_e5466 / assign6010_e5469);
        (assign6010_e5470, ((p.p479 * var_temp0__blk79_dn4) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn6) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn7) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn8) / assign6010_e5469), ((p.p479 * var_temp0__blk79_dn9) / assign6010_e5469),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6010_e5472;
        var_temp_dn4 = assign6010_e5472_d_n4;
        var_temp_dn6 = assign6010_e5472_d_n6;
        var_temp_dn7 = assign6010_e5472_d_n7;
        var_temp_dn8 = assign6010_e5472_d_n8;
        var_temp_dn9 = assign6010_e5472_d_n9;

        let (assign6020_e5483, assign6020_e5483_d_n4, assign6020_e5483_d_n6, assign6020_e5483_d_n7, assign6020_e5483_d_n8, assign6020_e5483_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6020_e5481: f64 = (var_cf_p + var_temp);
        (assign6020_e5481, (var_cf_p_dn4 + var_temp_dn4), (var_cf_p_dn6 + var_temp_dn6), (var_cf_p_dn7 + var_temp_dn7), (var_cf_p_dn8 + var_temp_dn8), (var_cf_p_dn9 + var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign6020_e5483;
        var_cf_p_dn4 = assign6020_e5483_d_n4;
        var_cf_p_dn6 = assign6020_e5483_d_n6;
        var_cf_p_dn7 = assign6020_e5483_d_n7;
        var_cf_p_dn8 = assign6020_e5483_d_n8;
        var_cf_p_dn9 = assign6020_e5483_d_n9;

        let (assign6030_e5494, assign6030_e5494_d_n4, assign6030_e5494_d_n6, assign6030_e5494_d_n7, assign6030_e5494_d_n8, assign6030_e5494_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6030_e5492: f64 = (var_cf_p).max(0.0);
        (assign6030_e5492, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign6030_e5494;
        var_cf1_t_dn4 = assign6030_e5494_d_n4;
        var_cf1_t_dn6 = assign6030_e5494_d_n6;
        var_cf1_t_dn7 = assign6030_e5494_d_n7;
        var_cf1_t_dn8 = assign6030_e5494_d_n8;
        var_cf1_t_dn9 = assign6030_e5494_d_n9;

        let (assign6040_e5505, assign6040_e5505_d_n4, assign6040_e5505_d_n6, assign6040_e5505_d_n7, assign6040_e5505_d_n8, assign6040_e5505_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6040_e5503: f64 = (var_cfac_p + var_temp);
        (assign6040_e5503, (var_cfac_p_dn4 + var_temp_dn4), (var_cfac_p_dn6 + var_temp_dn6), (var_cfac_p_dn7 + var_temp_dn7), (var_cfac_p_dn8 + var_temp_dn8), (var_cfac_p_dn9 + var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign6040_e5505;
        var_cfac_p_dn4 = assign6040_e5505_d_n4;
        var_cfac_p_dn6 = assign6040_e5505_d_n6;
        var_cfac_p_dn7 = assign6040_e5505_d_n7;
        var_cfac_p_dn8 = assign6040_e5505_d_n8;
        var_cfac_p_dn9 = assign6040_e5505_d_n9;

        let (assign6050_e5516, assign6050_e5516_d_n4, assign6050_e5516_d_n6, assign6050_e5516_d_n7, assign6050_e5516_d_n8, assign6050_e5516_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6050_e5514: f64 = (var_cfac_p).max(0.0);
        (assign6050_e5514, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign6050_e5516;
        var_cfac1_t_dn4 = assign6050_e5516_d_n4;
        var_cfac1_t_dn6 = assign6050_e5516_d_n6;
        var_cfac1_t_dn7 = assign6050_e5516_d_n7;
        var_cfac1_t_dn8 = assign6050_e5516_d_n8;
        var_cfac1_t_dn9 = assign6050_e5516_d_n9;

        let (assign6060_e5529, assign6060_e5529_d_n4, assign6060_e5529_d_n6, assign6060_e5529_d_n7, assign6060_e5529_d_n8, assign6060_e5529_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6060_e5525: f64 = (p.p238 * var_tox2_i);
        let assign6060_e5527: f64 = (assign6060_e5525 / var_tox1_i);
        (assign6060_e5527, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6060_e5529;
        var_temp_dn4 = assign6060_e5529_d_n4;
        var_temp_dn6 = assign6060_e5529_d_n6;
        var_temp_dn7 = assign6060_e5529_d_n7;
        var_temp_dn8 = assign6060_e5529_d_n8;
        var_temp_dn9 = assign6060_e5529_d_n9;

        let (assign6070_e5540, assign6070_e5540_d_n4, assign6070_e5540_d_n6, assign6070_e5540_d_n7, assign6070_e5540_d_n8, assign6070_e5540_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6070_e5538: f64 = (var_cf1_t * var_temp);
        (assign6070_e5538, ((var_cf1_t_dn4 * var_temp) + (var_cf1_t * var_temp_dn4)), ((var_cf1_t_dn6 * var_temp) + (var_cf1_t * var_temp_dn6)), ((var_cf1_t_dn7 * var_temp) + (var_cf1_t * var_temp_dn7)), ((var_cf1_t_dn8 * var_temp) + (var_cf1_t * var_temp_dn8)), ((var_cf1_t_dn9 * var_temp) + (var_cf1_t * var_temp_dn9)),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign6070_e5540;
        var_cf2_t_dn4 = assign6070_e5540_d_n4;
        var_cf2_t_dn6 = assign6070_e5540_d_n6;
        var_cf2_t_dn7 = assign6070_e5540_d_n7;
        var_cf2_t_dn8 = assign6070_e5540_d_n8;
        var_cf2_t_dn9 = assign6070_e5540_d_n9;

        let (assign6080_e5551, assign6080_e5551_d_n4, assign6080_e5551_d_n6, assign6080_e5551_d_n7, assign6080_e5551_d_n8, assign6080_e5551_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign6080_e5549: f64 = (var_cfac1_t * var_temp);
        (assign6080_e5549, ((var_cfac1_t_dn4 * var_temp) + (var_cfac1_t * var_temp_dn4)), ((var_cfac1_t_dn6 * var_temp) + (var_cfac1_t * var_temp_dn6)), ((var_cfac1_t_dn7 * var_temp) + (var_cfac1_t * var_temp_dn7)), ((var_cfac1_t_dn8 * var_temp) + (var_cfac1_t * var_temp_dn8)), ((var_cfac1_t_dn9 * var_temp) + (var_cfac1_t * var_temp_dn9)),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign6080_e5551;
        var_cfac2_t_dn4 = assign6080_e5551_d_n4;
        var_cfac2_t_dn6 = assign6080_e5551_d_n6;
        var_cfac2_t_dn7 = assign6080_e5551_d_n7;
        var_cfac2_t_dn8 = assign6080_e5551_d_n8;
        var_cfac2_t_dn9 = assign6080_e5551_d_n9;

        let (assign6090_e5561, assign6090_e5561_d_n4, assign6090_e5561_d_n6, assign6090_e5561_d_n7, assign6090_e5561_d_n8, assign6090_e5561_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
        var_tmpa = assign6090_e5561;
        var_tmpa_dn4 = assign6090_e5561_d_n4;
        var_tmpa_dn6 = assign6090_e5561_d_n6;
        var_tmpa_dn7 = assign6090_e5561_d_n7;
        var_tmpa_dn8 = assign6090_e5561_d_n8;
        var_tmpa_dn9 = assign6090_e5561_d_n9;

        let (assign6100_e5571,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign6100_e5571;

        let (assign6110_e5584, assign6110_e5584_d_n4, assign6110_e5584_d_n6, assign6110_e5584_d_n7, assign6110_e5584_d_n8, assign6110_e5584_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6110_e5580: f64 = (-1.0);
        let assign6110_e5582: f64 = (assign6110_e5580 / p.p482);
        (assign6110_e5582, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6110_e5584;
        var_temp_dn4 = assign6110_e5584_d_n4;
        var_temp_dn6 = assign6110_e5584_d_n6;
        var_temp_dn7 = assign6110_e5584_d_n7;
        var_temp_dn8 = assign6110_e5584_d_n8;
        var_temp_dn9 = assign6110_e5584_d_n9;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_iloop_slot = var_iloop;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_dn4_slot = var_kstressu0_dn4;
        *var_kstressu0_dn6_slot = var_kstressu0_dn6;
        *var_kstressu0_dn7_slot = var_kstressu0_dn7;
        *var_kstressu0_dn8_slot = var_kstressu0_dn8;
        *var_kstressu0_dn9_slot = var_kstressu0_dn9;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_dn4_slot = var_rhobeta_dn4;
        *var_rhobeta_dn6_slot = var_rhobeta_dn6;
        *var_rhobeta_dn7_slot = var_rhobeta_dn7;
        *var_rhobeta_dn8_slot = var_rhobeta_dn8;
        *var_rhobeta_dn9_slot = var_rhobeta_dn9;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_dn4_slot = var_rhobetaref_dn4;
        *var_rhobetaref_dn6_slot = var_rhobetaref_dn6;
        *var_rhobetaref_dn7_slot = var_rhobetaref_dn7;
        *var_rhobetaref_dn8_slot = var_rhobetaref_dn8;
        *var_rhobetaref_dn9_slot = var_rhobetaref_dn9;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_templ_slot = var_templ;
        *var_tempw_slot = var_tempw;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        var_delwod: f64,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_rt: f64,
        var_rt_dn4: f64,
        var_rt_dn6: f64,
        var_rt_dn7: f64,
        var_rt_dn8: f64,
        var_rt_dn9: f64,
        var_temp: f64,
        var_temp_dn4: f64,
        var_temp_dn6: f64,
        var_temp_dn7: f64,
        var_temp_dn8: f64,
        var_temp_dn9: f64,
        var_w_i: f64,
        var_guard135_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_dn4_slot: &mut f64,
        var_rhobeta_dn6_slot: &mut f64,
        var_rhobeta_dn7_slot: &mut f64,
        var_rhobeta_dn8_slot: &mut f64,
        var_rhobeta_dn9_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_dn4_slot: &mut f64,
        var_rhobetaref_dn6_slot: &mut f64,
        var_rhobetaref_dn7_slot: &mut f64,
        var_rhobetaref_dn8_slot: &mut f64,
        var_rhobetaref_dn9_slot: &mut f64,
        var_ruo_slot: &mut f64,
        var_ruo_dn4_slot: &mut f64,
        var_ruo_dn6_slot: &mut f64,
        var_ruo_dn7_slot: &mut f64,
        var_ruo_dn8_slot: &mut f64,
        var_ruo_dn9_slot: &mut f64,
        var_str_g_slot: &mut f64,
        var_str_g_dn4_slot: &mut f64,
        var_str_g_dn6_slot: &mut f64,
        var_str_g_dn7_slot: &mut f64,
        var_str_g_dn8_slot: &mut f64,
        var_str_g_dn9_slot: &mut f64,
        var_str_gref_slot: &mut f64,
        var_str_gref_dn4_slot: &mut f64,
        var_str_gref_dn6_slot: &mut f64,
        var_str_gref_dn7_slot: &mut f64,
        var_str_gref_dn8_slot: &mut f64,
        var_str_gref_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_wx_slot: &mut f64,
    ) {
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_dn4: f64 = *var_rhobeta_dn4_slot;
        let mut var_rhobeta_dn6: f64 = *var_rhobeta_dn6_slot;
        let mut var_rhobeta_dn7: f64 = *var_rhobeta_dn7_slot;
        let mut var_rhobeta_dn8: f64 = *var_rhobeta_dn8_slot;
        let mut var_rhobeta_dn9: f64 = *var_rhobeta_dn9_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_dn4: f64 = *var_rhobetaref_dn4_slot;
        let mut var_rhobetaref_dn6: f64 = *var_rhobetaref_dn6_slot;
        let mut var_rhobetaref_dn7: f64 = *var_rhobetaref_dn7_slot;
        let mut var_rhobetaref_dn8: f64 = *var_rhobetaref_dn8_slot;
        let mut var_rhobetaref_dn9: f64 = *var_rhobetaref_dn9_slot;
        let mut var_ruo: f64 = *var_ruo_slot;
        let mut var_ruo_dn4: f64 = *var_ruo_dn4_slot;
        let mut var_ruo_dn6: f64 = *var_ruo_dn6_slot;
        let mut var_ruo_dn7: f64 = *var_ruo_dn7_slot;
        let mut var_ruo_dn8: f64 = *var_ruo_dn8_slot;
        let mut var_ruo_dn9: f64 = *var_ruo_dn9_slot;
        let mut var_str_g: f64 = *var_str_g_slot;
        let mut var_str_g_dn4: f64 = *var_str_g_dn4_slot;
        let mut var_str_g_dn6: f64 = *var_str_g_dn6_slot;
        let mut var_str_g_dn7: f64 = *var_str_g_dn7_slot;
        let mut var_str_g_dn8: f64 = *var_str_g_dn8_slot;
        let mut var_str_g_dn9: f64 = *var_str_g_dn9_slot;
        let mut var_str_gref: f64 = *var_str_gref_slot;
        let mut var_str_gref_dn4: f64 = *var_str_gref_dn4_slot;
        let mut var_str_gref_dn6: f64 = *var_str_gref_dn6_slot;
        let mut var_str_gref_dn7: f64 = *var_str_gref_dn7_slot;
        let mut var_str_gref_dn8: f64 = *var_str_gref_dn8_slot;
        let mut var_str_gref_dn9: f64 = *var_str_gref_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_wx: f64 = *var_wx_slot;

        let mut assign6120_loop_guard: usize = 0;
        while {
            let assign6120_cond_e5595: f64 = (p.p29 - 0.5);
            let assign6120_cond_e5597: f64 = if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_iloop < assign6120_cond_e5595)) { 1.0 } else { 0.0 };
            assign6120_cond_e5597 != 0.0
        } {
            assign6120_loop_guard += 1;
            assert!(assign6120_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let assign6120_body0_e5601: f64 = (0.5 * p.p20);
            let assign6120_body0_e5602: f64 = (p.p26 + assign6120_body0_e5601);
            let assign6120_body0_e5606: f64 = (p.p28 + p.p20);
            let assign6120_body0_e5607: f64 = (var_iloop * assign6120_body0_e5606);
            let assign6120_body0_e5608: f64 = (assign6120_body0_e5602 + assign6120_body0_e5607);
            let assign6120_body0_e5609: f64 = (-assign6120_body0_e5608);
            let assign6120_body0_e5611: f64 = (assign6120_body0_e5609 / p.p481);
            let assign6120_body0_e5613: f64 = (-80.0);
            let assign6120_body0_e5614: f64 = if assign6120_body0_e5611 > assign6120_body0_e5613 { 1.0 } else { 0.0 };
            var_guard135 = assign6120_body0_e5614;
            let (assign6120_body1_e5640, assign6120_body1_e5640_d_n4, assign6120_body1_e5640_d_n6, assign6120_body1_e5640_d_n7, assign6120_body1_e5640_d_n8, assign6120_body1_e5640_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard135 != 0.0)) {
        let assign6120_body1_e5627: f64 = (0.5 * p.p20);
        let assign6120_body1_e5628: f64 = (p.p26 + assign6120_body1_e5627);
        let assign6120_body1_e5632: f64 = (p.p28 + p.p20);
        let assign6120_body1_e5633: f64 = (var_iloop * assign6120_body1_e5632);
        let assign6120_body1_e5634: f64 = (assign6120_body1_e5628 + assign6120_body1_e5633);
        let assign6120_body1_e5635: f64 = (-assign6120_body1_e5634);
        let assign6120_body1_e5637: f64 = (assign6120_body1_e5635 / p.p481);
        let assign6120_body1_e5638: f64 = (assign6120_body1_e5637).exp();
        (assign6120_body1_e5638, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
            var_temp1 = assign6120_body1_e5640;
            var_temp1_dn4 = assign6120_body1_e5640_d_n4;
            var_temp1_dn6 = assign6120_body1_e5640_d_n6;
            var_temp1_dn7 = assign6120_body1_e5640_d_n7;
            var_temp1_dn8 = assign6120_body1_e5640_d_n8;
            var_temp1_dn9 = assign6120_body1_e5640_d_n9;
            let (assign6120_body2_e5717, assign6120_body2_e5717_d_n4, assign6120_body2_e5717_d_n6, assign6120_body2_e5717_d_n7, assign6120_body2_e5717_d_n8, assign6120_body2_e5717_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard135 == 0.0)) {
        let assign6120_body2_e5656: f64 = (0.5 * p.p20);
        let assign6120_body2_e5657: f64 = (p.p26 + assign6120_body2_e5656);
        let assign6120_body2_e5661: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5662: f64 = (var_iloop * assign6120_body2_e5661);
        let assign6120_body2_e5663: f64 = (assign6120_body2_e5657 + assign6120_body2_e5662);
        let assign6120_body2_e5664: f64 = (-assign6120_body2_e5663);
        let assign6120_body2_e5666: f64 = (assign6120_body2_e5664 / p.p481);
        let assign6120_body2_e5667: f64 = (-assign6120_body2_e5666);
        let assign6120_body2_e5669: f64 = (assign6120_body2_e5667 - 80.0);
        let assign6120_body2_e5675: f64 = (0.5 * p.p20);
        let assign6120_body2_e5676: f64 = (p.p26 + assign6120_body2_e5675);
        let assign6120_body2_e5680: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5681: f64 = (var_iloop * assign6120_body2_e5680);
        let assign6120_body2_e5682: f64 = (assign6120_body2_e5676 + assign6120_body2_e5681);
        let assign6120_body2_e5683: f64 = (-assign6120_body2_e5682);
        let assign6120_body2_e5685: f64 = (assign6120_body2_e5683 / p.p481);
        let assign6120_body2_e5686: f64 = (-assign6120_body2_e5685);
        let assign6120_body2_e5688: f64 = (assign6120_body2_e5686 - 80.0);
        let assign6120_body2_e5689: f64 = (0.5 * assign6120_body2_e5688);
        let assign6120_body2_e5694: f64 = (0.5 * p.p20);
        let assign6120_body2_e5695: f64 = (p.p26 + assign6120_body2_e5694);
        let assign6120_body2_e5699: f64 = (p.p28 + p.p20);
        let assign6120_body2_e5700: f64 = (var_iloop * assign6120_body2_e5699);
        let assign6120_body2_e5701: f64 = (assign6120_body2_e5695 + assign6120_body2_e5700);
        let assign6120_body2_e5702: f64 = (-assign6120_body2_e5701);
        let assign6120_body2_e5704: f64 = (assign6120_body2_e5702 / p.p481);
        let assign6120_body2_e5705: f64 = (-assign6120_body2_e5704);
        let assign6120_body2_e5707: f64 = (assign6120_body2_e5705 - 80.0);
        let assign6120_body2_e5709: f64 = (assign6120_body2_e5707 * 0.3333333333333);
        let assign6120_body2_e5710: f64 = (1.0 + assign6120_body2_e5709);
        let assign6120_body2_e5711: f64 = (assign6120_body2_e5689 * assign6120_body2_e5710);
        let assign6120_body2_e5712: f64 = (1.0 + assign6120_body2_e5711);
        let assign6120_body2_e5713: f64 = (assign6120_body2_e5669 * assign6120_body2_e5712);
        let assign6120_body2_e5714: f64 = (1.0 + assign6120_body2_e5713);
        let assign6120_body2_e5715: f64 = (1.80485e-35 / assign6120_body2_e5714);
        (assign6120_body2_e5715, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
            var_temp1 = assign6120_body2_e5717;
            var_temp1_dn4 = assign6120_body2_e5717_d_n4;
            var_temp1_dn6 = assign6120_body2_e5717_d_n6;
            var_temp1_dn7 = assign6120_body2_e5717_d_n7;
            var_temp1_dn8 = assign6120_body2_e5717_d_n8;
            var_temp1_dn9 = assign6120_body2_e5717_d_n9;
            let assign6120_body3_e5721: f64 = (0.5 * p.p20);
            let assign6120_body3_e5722: f64 = (p.p27 + assign6120_body3_e5721);
            let assign6120_body3_e5725: f64 = (p.p29 - 1.0);
            let assign6120_body3_e5727: f64 = (assign6120_body3_e5725 - var_iloop);
            let assign6120_body3_e5730: f64 = (p.p28 + p.p20);
            let assign6120_body3_e5731: f64 = (assign6120_body3_e5727 * assign6120_body3_e5730);
            let assign6120_body3_e5732: f64 = (assign6120_body3_e5722 + assign6120_body3_e5731);
            let assign6120_body3_e5733: f64 = (-assign6120_body3_e5732);
            let assign6120_body3_e5735: f64 = (assign6120_body3_e5733 / p.p481);
            let assign6120_body3_e5737: f64 = (-80.0);
            let assign6120_body3_e5738: f64 = if assign6120_body3_e5735 > assign6120_body3_e5737 { 1.0 } else { 0.0 };
            var_guard136 = assign6120_body3_e5738;
            let (assign6120_body4_e5768, assign6120_body4_e5768_d_n4, assign6120_body4_e5768_d_n6, assign6120_body4_e5768_d_n7, assign6120_body4_e5768_d_n8, assign6120_body4_e5768_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard136 != 0.0)) {
        let assign6120_body4_e5751: f64 = (0.5 * p.p20);
        let assign6120_body4_e5752: f64 = (p.p27 + assign6120_body4_e5751);
        let assign6120_body4_e5755: f64 = (p.p29 - 1.0);
        let assign6120_body4_e5757: f64 = (assign6120_body4_e5755 - var_iloop);
        let assign6120_body4_e5760: f64 = (p.p28 + p.p20);
        let assign6120_body4_e5761: f64 = (assign6120_body4_e5757 * assign6120_body4_e5760);
        let assign6120_body4_e5762: f64 = (assign6120_body4_e5752 + assign6120_body4_e5761);
        let assign6120_body4_e5763: f64 = (-assign6120_body4_e5762);
        let assign6120_body4_e5765: f64 = (assign6120_body4_e5763 / p.p481);
        let assign6120_body4_e5766: f64 = (assign6120_body4_e5765).exp();
        (assign6120_body4_e5766, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
            var_temp2 = assign6120_body4_e5768;
            var_temp2_dn4 = assign6120_body4_e5768_d_n4;
            var_temp2_dn6 = assign6120_body4_e5768_d_n6;
            var_temp2_dn7 = assign6120_body4_e5768_d_n7;
            var_temp2_dn8 = assign6120_body4_e5768_d_n8;
            var_temp2_dn9 = assign6120_body4_e5768_d_n9;
            let (assign6120_body5_e5857, assign6120_body5_e5857_d_n4, assign6120_body5_e5857_d_n6, assign6120_body5_e5857_d_n7, assign6120_body5_e5857_d_n8, assign6120_body5_e5857_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard136 == 0.0)) {
        let assign6120_body5_e5784: f64 = (0.5 * p.p20);
        let assign6120_body5_e5785: f64 = (p.p27 + assign6120_body5_e5784);
        let assign6120_body5_e5788: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5790: f64 = (assign6120_body5_e5788 - var_iloop);
        let assign6120_body5_e5793: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5794: f64 = (assign6120_body5_e5790 * assign6120_body5_e5793);
        let assign6120_body5_e5795: f64 = (assign6120_body5_e5785 + assign6120_body5_e5794);
        let assign6120_body5_e5796: f64 = (-assign6120_body5_e5795);
        let assign6120_body5_e5798: f64 = (assign6120_body5_e5796 / p.p481);
        let assign6120_body5_e5799: f64 = (-assign6120_body5_e5798);
        let assign6120_body5_e5801: f64 = (assign6120_body5_e5799 - 80.0);
        let assign6120_body5_e5807: f64 = (0.5 * p.p20);
        let assign6120_body5_e5808: f64 = (p.p27 + assign6120_body5_e5807);
        let assign6120_body5_e5811: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5813: f64 = (assign6120_body5_e5811 - var_iloop);
        let assign6120_body5_e5816: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5817: f64 = (assign6120_body5_e5813 * assign6120_body5_e5816);
        let assign6120_body5_e5818: f64 = (assign6120_body5_e5808 + assign6120_body5_e5817);
        let assign6120_body5_e5819: f64 = (-assign6120_body5_e5818);
        let assign6120_body5_e5821: f64 = (assign6120_body5_e5819 / p.p481);
        let assign6120_body5_e5822: f64 = (-assign6120_body5_e5821);
        let assign6120_body5_e5824: f64 = (assign6120_body5_e5822 - 80.0);
        let assign6120_body5_e5825: f64 = (0.5 * assign6120_body5_e5824);
        let assign6120_body5_e5830: f64 = (0.5 * p.p20);
        let assign6120_body5_e5831: f64 = (p.p27 + assign6120_body5_e5830);
        let assign6120_body5_e5834: f64 = (p.p29 - 1.0);
        let assign6120_body5_e5836: f64 = (assign6120_body5_e5834 - var_iloop);
        let assign6120_body5_e5839: f64 = (p.p28 + p.p20);
        let assign6120_body5_e5840: f64 = (assign6120_body5_e5836 * assign6120_body5_e5839);
        let assign6120_body5_e5841: f64 = (assign6120_body5_e5831 + assign6120_body5_e5840);
        let assign6120_body5_e5842: f64 = (-assign6120_body5_e5841);
        let assign6120_body5_e5844: f64 = (assign6120_body5_e5842 / p.p481);
        let assign6120_body5_e5845: f64 = (-assign6120_body5_e5844);
        let assign6120_body5_e5847: f64 = (assign6120_body5_e5845 - 80.0);
        let assign6120_body5_e5849: f64 = (assign6120_body5_e5847 * 0.3333333333333);
        let assign6120_body5_e5850: f64 = (1.0 + assign6120_body5_e5849);
        let assign6120_body5_e5851: f64 = (assign6120_body5_e5825 * assign6120_body5_e5850);
        let assign6120_body5_e5852: f64 = (1.0 + assign6120_body5_e5851);
        let assign6120_body5_e5853: f64 = (assign6120_body5_e5801 * assign6120_body5_e5852);
        let assign6120_body5_e5854: f64 = (1.0 + assign6120_body5_e5853);
        let assign6120_body5_e5855: f64 = (1.80485e-35 / assign6120_body5_e5854);
        (assign6120_body5_e5855, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
            var_temp2 = assign6120_body5_e5857;
            var_temp2_dn4 = assign6120_body5_e5857_d_n4;
            var_temp2_dn6 = assign6120_body5_e5857_d_n6;
            var_temp2_dn7 = assign6120_body5_e5857_d_n7;
            var_temp2_dn8 = assign6120_body5_e5857_d_n8;
            var_temp2_dn9 = assign6120_body5_e5857_d_n9;
            let (assign6120_body6_e5872, assign6120_body6_e5872_d_n4, assign6120_body6_e5872_d_n6, assign6120_body6_e5872_d_n7, assign6120_body6_e5872_d_n8, assign6120_body6_e5872_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body6_e5867: f64 = (1.0 - var_temp1);
        let assign6120_body6_e5869: f64 = (-p.p482);
        let assign6120_body6_e5870: f64 = (assign6120_body6_e5867).powf(assign6120_body6_e5869);
        (assign6120_body6_e5870, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn4))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn4) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn6))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn6) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn7))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn7) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn8))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn8) / assign6120_body6_e5867))) }, if 0.0 == 0.0 && ((assign6120_body6_e5869) as f64).is_finite() && ((assign6120_body6_e5869) as f64).fract() == 0.0 { if assign6120_body6_e5869 == 0.0 { 0.0 } else { (assign6120_body6_e5869 * ((assign6120_body6_e5867).powf(assign6120_body6_e5869 - 1.0) * (-var_temp1_dn9))) } } else { (assign6120_body6_e5870 * (assign6120_body6_e5869 * ((-var_temp1_dn9) / assign6120_body6_e5867))) },)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
            var_temp3 = assign6120_body6_e5872;
            var_temp3_dn4 = assign6120_body6_e5872_d_n4;
            var_temp3_dn6 = assign6120_body6_e5872_d_n6;
            var_temp3_dn7 = assign6120_body6_e5872_d_n7;
            var_temp3_dn8 = assign6120_body6_e5872_d_n8;
            var_temp3_dn9 = assign6120_body6_e5872_d_n9;
            let (assign6120_body7_e5887, assign6120_body7_e5887_d_n4, assign6120_body7_e5887_d_n6, assign6120_body7_e5887_d_n7, assign6120_body7_e5887_d_n8, assign6120_body7_e5887_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body7_e5882: f64 = (1.0 - var_temp2);
        let assign6120_body7_e5884: f64 = (-p.p482);
        let assign6120_body7_e5885: f64 = (assign6120_body7_e5882).powf(assign6120_body7_e5884);
        (assign6120_body7_e5885, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn4))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn4) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn6))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn6) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn7))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn7) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn8))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn8) / assign6120_body7_e5882))) }, if 0.0 == 0.0 && ((assign6120_body7_e5884) as f64).is_finite() && ((assign6120_body7_e5884) as f64).fract() == 0.0 { if assign6120_body7_e5884 == 0.0 { 0.0 } else { (assign6120_body7_e5884 * ((assign6120_body7_e5882).powf(assign6120_body7_e5884 - 1.0) * (-var_temp2_dn9))) } } else { (assign6120_body7_e5885 * (assign6120_body7_e5884 * ((-var_temp2_dn9) / assign6120_body7_e5882))) },)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
            var_temp4 = assign6120_body7_e5887;
            var_temp4_dn4 = assign6120_body7_e5887_d_n4;
            var_temp4_dn6 = assign6120_body7_e5887_d_n6;
            var_temp4_dn7 = assign6120_body7_e5887_d_n7;
            var_temp4_dn8 = assign6120_body7_e5887_d_n8;
            var_temp4_dn9 = assign6120_body7_e5887_d_n9;
            let (assign6120_body8_e5905, assign6120_body8_e5905_d_n4, assign6120_body8_e5905_d_n6, assign6120_body8_e5905_d_n7, assign6120_body8_e5905_d_n8, assign6120_body8_e5905_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body8_e5899: f64 = (var_temp3 + var_temp4);
        let assign6120_body8_e5900: f64 = (0.5 * assign6120_body8_e5899);
        let assign6120_body8_e5902: f64 = (assign6120_body8_e5900).powf(var_temp);
        let assign6120_body8_e5903: f64 = (var_tmpa + assign6120_body8_e5902);
        (assign6120_body8_e5903, (var_tmpa_dn4 + if var_temp_dn4 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn4 + var_temp4_dn4)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn4 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn4 + var_temp4_dn4)) / assign6120_body8_e5900)))) }), (var_tmpa_dn6 + if var_temp_dn6 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn6 + var_temp4_dn6)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn6 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn6 + var_temp4_dn6)) / assign6120_body8_e5900)))) }), (var_tmpa_dn7 + if var_temp_dn7 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn7 + var_temp4_dn7)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn7 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn7 + var_temp4_dn7)) / assign6120_body8_e5900)))) }), (var_tmpa_dn8 + if var_temp_dn8 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn8 + var_temp4_dn8)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn8 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn8 + var_temp4_dn8)) / assign6120_body8_e5900)))) }), (var_tmpa_dn9 + if var_temp_dn9 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6120_body8_e5900).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn9 + var_temp4_dn9)))) } } else { (assign6120_body8_e5902 * ((var_temp_dn9 * (assign6120_body8_e5900).ln()) + (var_temp * ((0.5 * (var_temp3_dn9 + var_temp4_dn9)) / assign6120_body8_e5900)))) }),)
    } else {
        (var_tmpa, var_tmpa_dn4, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9,)
    }
};
            var_tmpa = assign6120_body8_e5905;
            var_tmpa_dn4 = assign6120_body8_e5905_d_n4;
            var_tmpa_dn6 = assign6120_body8_e5905_d_n6;
            var_tmpa_dn7 = assign6120_body8_e5905_d_n7;
            var_tmpa_dn8 = assign6120_body8_e5905_d_n8;
            var_tmpa_dn9 = assign6120_body8_e5905_d_n9;
            let (assign6120_body9_e5917,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6120_body9_e5915: f64 = (var_iloop + 1.0);
        (assign6120_body9_e5915,)
    } else {
        (var_iloop,)
    }
};
            var_iloop = assign6120_body9_e5917;
        }

        let (assign6130_e5931, assign6130_e5931_d_n4, assign6130_e5931_d_n6, assign6130_e5931_d_n7, assign6130_e5931_d_n8, assign6130_e5931_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6130_e5928: f64 = (var_tmpa / p.p29);
        let assign6130_e5929: f64 = (1.0 - assign6130_e5928);
        (assign6130_e5929, (-(var_tmpa_dn4 / p.p29)), (-(var_tmpa_dn6 / p.p29)), (-(var_tmpa_dn7 / p.p29)), (-(var_tmpa_dn8 / p.p29)), (-(var_tmpa_dn9 / p.p29)),)
    } else {
        (var_str_g, var_str_g_dn4, var_str_g_dn6, var_str_g_dn7, var_str_g_dn8, var_str_g_dn9,)
    }
};
        var_str_g = assign6130_e5931;
        var_str_g_dn4 = assign6130_e5931_d_n4;
        var_str_g_dn6 = assign6130_e5931_d_n6;
        var_str_g_dn7 = assign6130_e5931_d_n7;
        var_str_g_dn8 = assign6130_e5931_d_n8;
        var_str_g_dn9 = assign6130_e5931_d_n9;

        let assign6140_e5935: f64 = (0.5 * p.p20);
        let assign6140_e5936: f64 = (p.p462 + assign6140_e5935);
        let assign6140_e5937: f64 = (-assign6140_e5936);
        let assign6140_e5939: f64 = (assign6140_e5937 / p.p481);
        let assign6140_e5941: f64 = (-80.0);
        let assign6140_e5942: f64 = if assign6140_e5939 > assign6140_e5941 { 1.0 } else { 0.0 };
        var_guard137 = assign6140_e5942;

        let (assign6150_e5962, assign6150_e5962_d_n4, assign6150_e5962_d_n6, assign6150_e5962_d_n7, assign6150_e5962_d_n8, assign6150_e5962_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard137 != 0.0)) {
        let assign6150_e5955: f64 = (0.5 * p.p20);
        let assign6150_e5956: f64 = (p.p462 + assign6150_e5955);
        let assign6150_e5957: f64 = (-assign6150_e5956);
        let assign6150_e5959: f64 = (assign6150_e5957 / p.p481);
        let assign6150_e5960: f64 = (assign6150_e5959).exp();
        (assign6150_e5960, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign6150_e5962;
        var_temp1_dn4 = assign6150_e5962_d_n4;
        var_temp1_dn6 = assign6150_e5962_d_n6;
        var_temp1_dn7 = assign6150_e5962_d_n7;
        var_temp1_dn8 = assign6150_e5962_d_n8;
        var_temp1_dn9 = assign6150_e5962_d_n9;

        let (assign6160_e6021, assign6160_e6021_d_n4, assign6160_e6021_d_n6, assign6160_e6021_d_n7, assign6160_e6021_d_n8, assign6160_e6021_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard137 == 0.0)) {
        let assign6160_e5978: f64 = (0.5 * p.p20);
        let assign6160_e5979: f64 = (p.p462 + assign6160_e5978);
        let assign6160_e5980: f64 = (-assign6160_e5979);
        let assign6160_e5982: f64 = (assign6160_e5980 / p.p481);
        let assign6160_e5983: f64 = (-assign6160_e5982);
        let assign6160_e5985: f64 = (assign6160_e5983 - 80.0);
        let assign6160_e5991: f64 = (0.5 * p.p20);
        let assign6160_e5992: f64 = (p.p462 + assign6160_e5991);
        let assign6160_e5993: f64 = (-assign6160_e5992);
        let assign6160_e5995: f64 = (assign6160_e5993 / p.p481);
        let assign6160_e5996: f64 = (-assign6160_e5995);
        let assign6160_e5998: f64 = (assign6160_e5996 - 80.0);
        let assign6160_e5999: f64 = (0.5 * assign6160_e5998);
        let assign6160_e6004: f64 = (0.5 * p.p20);
        let assign6160_e6005: f64 = (p.p462 + assign6160_e6004);
        let assign6160_e6006: f64 = (-assign6160_e6005);
        let assign6160_e6008: f64 = (assign6160_e6006 / p.p481);
        let assign6160_e6009: f64 = (-assign6160_e6008);
        let assign6160_e6011: f64 = (assign6160_e6009 - 80.0);
        let assign6160_e6013: f64 = (assign6160_e6011 * 0.3333333333333);
        let assign6160_e6014: f64 = (1.0 + assign6160_e6013);
        let assign6160_e6015: f64 = (assign6160_e5999 * assign6160_e6014);
        let assign6160_e6016: f64 = (1.0 + assign6160_e6015);
        let assign6160_e6017: f64 = (assign6160_e5985 * assign6160_e6016);
        let assign6160_e6018: f64 = (1.0 + assign6160_e6017);
        let assign6160_e6019: f64 = (1.80485e-35 / assign6160_e6018);
        (assign6160_e6019, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign6160_e6021;
        var_temp1_dn4 = assign6160_e6021_d_n4;
        var_temp1_dn6 = assign6160_e6021_d_n6;
        var_temp1_dn7 = assign6160_e6021_d_n7;
        var_temp1_dn8 = assign6160_e6021_d_n8;
        var_temp1_dn9 = assign6160_e6021_d_n9;

        let assign6170_e6025: f64 = (0.5 * p.p20);
        let assign6170_e6026: f64 = (p.p463 + assign6170_e6025);
        let assign6170_e6027: f64 = (-assign6170_e6026);
        let assign6170_e6029: f64 = (assign6170_e6027 / p.p481);
        let assign6170_e6031: f64 = (-80.0);
        let assign6170_e6032: f64 = if assign6170_e6029 > assign6170_e6031 { 1.0 } else { 0.0 };
        var_guard138 = assign6170_e6032;

        let (assign6180_e6052, assign6180_e6052_d_n4, assign6180_e6052_d_n6, assign6180_e6052_d_n7, assign6180_e6052_d_n8, assign6180_e6052_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard138 != 0.0)) {
        let assign6180_e6045: f64 = (0.5 * p.p20);
        let assign6180_e6046: f64 = (p.p463 + assign6180_e6045);
        let assign6180_e6047: f64 = (-assign6180_e6046);
        let assign6180_e6049: f64 = (assign6180_e6047 / p.p481);
        let assign6180_e6050: f64 = (assign6180_e6049).exp();
        (assign6180_e6050, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign6180_e6052;
        var_temp2_dn4 = assign6180_e6052_d_n4;
        var_temp2_dn6 = assign6180_e6052_d_n6;
        var_temp2_dn7 = assign6180_e6052_d_n7;
        var_temp2_dn8 = assign6180_e6052_d_n8;
        var_temp2_dn9 = assign6180_e6052_d_n9;

        let (assign6190_e6111, assign6190_e6111_d_n4, assign6190_e6111_d_n6, assign6190_e6111_d_n7, assign6190_e6111_d_n8, assign6190_e6111_d_n9,) = {
    if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) && (var_guard138 == 0.0)) {
        let assign6190_e6068: f64 = (0.5 * p.p20);
        let assign6190_e6069: f64 = (p.p463 + assign6190_e6068);
        let assign6190_e6070: f64 = (-assign6190_e6069);
        let assign6190_e6072: f64 = (assign6190_e6070 / p.p481);
        let assign6190_e6073: f64 = (-assign6190_e6072);
        let assign6190_e6075: f64 = (assign6190_e6073 - 80.0);
        let assign6190_e6081: f64 = (0.5 * p.p20);
        let assign6190_e6082: f64 = (p.p463 + assign6190_e6081);
        let assign6190_e6083: f64 = (-assign6190_e6082);
        let assign6190_e6085: f64 = (assign6190_e6083 / p.p481);
        let assign6190_e6086: f64 = (-assign6190_e6085);
        let assign6190_e6088: f64 = (assign6190_e6086 - 80.0);
        let assign6190_e6089: f64 = (0.5 * assign6190_e6088);
        let assign6190_e6094: f64 = (0.5 * p.p20);
        let assign6190_e6095: f64 = (p.p463 + assign6190_e6094);
        let assign6190_e6096: f64 = (-assign6190_e6095);
        let assign6190_e6098: f64 = (assign6190_e6096 / p.p481);
        let assign6190_e6099: f64 = (-assign6190_e6098);
        let assign6190_e6101: f64 = (assign6190_e6099 - 80.0);
        let assign6190_e6103: f64 = (assign6190_e6101 * 0.3333333333333);
        let assign6190_e6104: f64 = (1.0 + assign6190_e6103);
        let assign6190_e6105: f64 = (assign6190_e6089 * assign6190_e6104);
        let assign6190_e6106: f64 = (1.0 + assign6190_e6105);
        let assign6190_e6107: f64 = (assign6190_e6075 * assign6190_e6106);
        let assign6190_e6108: f64 = (1.0 + assign6190_e6107);
        let assign6190_e6109: f64 = (1.80485e-35 / assign6190_e6108);
        (assign6190_e6109, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign6190_e6111;
        var_temp2_dn4 = assign6190_e6111_d_n4;
        var_temp2_dn6 = assign6190_e6111_d_n6;
        var_temp2_dn7 = assign6190_e6111_d_n7;
        var_temp2_dn8 = assign6190_e6111_d_n8;
        var_temp2_dn9 = assign6190_e6111_d_n9;

        let (assign6200_e6126, assign6200_e6126_d_n4, assign6200_e6126_d_n6, assign6200_e6126_d_n7, assign6200_e6126_d_n8, assign6200_e6126_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6200_e6121: f64 = (1.0 - var_temp1);
        let assign6200_e6123: f64 = (-p.p482);
        let assign6200_e6124: f64 = (assign6200_e6121).powf(assign6200_e6123);
        (assign6200_e6124, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn4))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn4) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn6))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn6) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn7))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn7) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn8))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn8) / assign6200_e6121))) }, if 0.0 == 0.0 && ((assign6200_e6123) as f64).is_finite() && ((assign6200_e6123) as f64).fract() == 0.0 { if assign6200_e6123 == 0.0 { 0.0 } else { (assign6200_e6123 * ((assign6200_e6121).powf(assign6200_e6123 - 1.0) * (-var_temp1_dn9))) } } else { (assign6200_e6124 * (assign6200_e6123 * ((-var_temp1_dn9) / assign6200_e6121))) },)
    } else {
        (var_temp3, var_temp3_dn4, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9,)
    }
};
        var_temp3 = assign6200_e6126;
        var_temp3_dn4 = assign6200_e6126_d_n4;
        var_temp3_dn6 = assign6200_e6126_d_n6;
        var_temp3_dn7 = assign6200_e6126_d_n7;
        var_temp3_dn8 = assign6200_e6126_d_n8;
        var_temp3_dn9 = assign6200_e6126_d_n9;

        let (assign6210_e6141, assign6210_e6141_d_n4, assign6210_e6141_d_n6, assign6210_e6141_d_n7, assign6210_e6141_d_n8, assign6210_e6141_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6210_e6136: f64 = (1.0 - var_temp2);
        let assign6210_e6138: f64 = (-p.p482);
        let assign6210_e6139: f64 = (assign6210_e6136).powf(assign6210_e6138);
        (assign6210_e6139, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn4))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn4) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn6))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn6) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn7))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn7) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn8))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn8) / assign6210_e6136))) }, if 0.0 == 0.0 && ((assign6210_e6138) as f64).is_finite() && ((assign6210_e6138) as f64).fract() == 0.0 { if assign6210_e6138 == 0.0 { 0.0 } else { (assign6210_e6138 * ((assign6210_e6136).powf(assign6210_e6138 - 1.0) * (-var_temp2_dn9))) } } else { (assign6210_e6139 * (assign6210_e6138 * ((-var_temp2_dn9) / assign6210_e6136))) },)
    } else {
        (var_temp4, var_temp4_dn4, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9,)
    }
};
        var_temp4 = assign6210_e6141;
        var_temp4_dn4 = assign6210_e6141_d_n4;
        var_temp4_dn6 = assign6210_e6141_d_n6;
        var_temp4_dn7 = assign6210_e6141_d_n7;
        var_temp4_dn8 = assign6210_e6141_d_n8;
        var_temp4_dn9 = assign6210_e6141_d_n9;

        let (assign6220_e6159, assign6220_e6159_d_n4, assign6220_e6159_d_n6, assign6220_e6159_d_n7, assign6220_e6159_d_n8, assign6220_e6159_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6220_e6153: f64 = (var_temp3 + var_temp4);
        let assign6220_e6154: f64 = (0.5 * assign6220_e6153);
        let assign6220_e6156: f64 = (assign6220_e6154).powf(var_temp);
        let assign6220_e6157: f64 = (1.0 - assign6220_e6156);
        (assign6220_e6157, (-if var_temp_dn4 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn4 + var_temp4_dn4)))) } } else { (assign6220_e6156 * ((var_temp_dn4 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn4 + var_temp4_dn4)) / assign6220_e6154)))) }), (-if var_temp_dn6 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn6 + var_temp4_dn6)))) } } else { (assign6220_e6156 * ((var_temp_dn6 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn6 + var_temp4_dn6)) / assign6220_e6154)))) }), (-if var_temp_dn7 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn7 + var_temp4_dn7)))) } } else { (assign6220_e6156 * ((var_temp_dn7 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn7 + var_temp4_dn7)) / assign6220_e6154)))) }), (-if var_temp_dn8 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn8 + var_temp4_dn8)))) } } else { (assign6220_e6156 * ((var_temp_dn8 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn8 + var_temp4_dn8)) / assign6220_e6154)))) }), (-if var_temp_dn9 == 0.0 && ((var_temp) as f64).is_finite() && ((var_temp) as f64).fract() == 0.0 { if var_temp == 0.0 { 0.0 } else { (var_temp * ((assign6220_e6154).powf(var_temp - 1.0) * (0.5 * (var_temp3_dn9 + var_temp4_dn9)))) } } else { (assign6220_e6156 * ((var_temp_dn9 * (assign6220_e6154).ln()) + (var_temp * ((0.5 * (var_temp3_dn9 + var_temp4_dn9)) / assign6220_e6154)))) }),)
    } else {
        (var_str_gref, var_str_gref_dn4, var_str_gref_dn6, var_str_gref_dn7, var_str_gref_dn8, var_str_gref_dn9,)
    }
};
        var_str_gref = assign6220_e6159;
        var_str_gref_dn4 = assign6220_e6159_d_n4;
        var_str_gref_dn6 = assign6220_e6159_d_n6;
        var_str_gref_dn7 = assign6220_e6159_d_n7;
        var_str_gref_dn8 = assign6220_e6159_d_n8;
        var_str_gref_dn9 = assign6220_e6159_d_n9;

        let (assign6230_e6175,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6230_e6169: f64 = (var_w_i + var_delwod);
        let assign6230_e6171: f64 = (assign6230_e6169 + p.p464);
        let assign6230_e6173: f64 = (assign6230_e6171).max(1e-9);
        (assign6230_e6173,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign6230_e6175;

        let (assign6240_e6193, assign6240_e6193_d_n4, assign6240_e6193_d_n6, assign6240_e6193_d_n7, assign6240_e6193_d_n8, assign6240_e6193_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6240_e6188: f64 = (var_rt - 1.0);
        let assign6240_e6189: f64 = (p.p487 * assign6240_e6188);
        let assign6240_e6190: f64 = (1.0 + assign6240_e6189);
        let assign6240_e6191: f64 = (p.p486 / assign6240_e6190);
        (assign6240_e6191, (-((p.p486 * (p.p487 * var_rt_dn4)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn6)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn7)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn8)) / (assign6240_e6190 * assign6240_e6190))), (-((p.p486 * (p.p487 * var_rt_dn9)) / (assign6240_e6190 * assign6240_e6190))),)
    } else {
        (var_ruo, var_ruo_dn4, var_ruo_dn6, var_ruo_dn7, var_ruo_dn8, var_ruo_dn9,)
    }
};
        var_ruo = assign6240_e6193;
        var_ruo_dn4 = assign6240_e6193_d_n4;
        var_ruo_dn6 = assign6240_e6193_d_n6;
        var_ruo_dn7 = assign6240_e6193_d_n7;
        var_ruo_dn8 = assign6240_e6193_d_n8;
        var_ruo_dn9 = assign6240_e6193_d_n9;

        let (assign6250_e6205, assign6250_e6205_d_n4, assign6250_e6205_d_n6, assign6250_e6205_d_n7, assign6250_e6205_d_n8, assign6250_e6205_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6250_e6203: f64 = (var_ruo * var_str_g);
        (assign6250_e6203, ((var_ruo_dn4 * var_str_g) + (var_ruo * var_str_g_dn4)), ((var_ruo_dn6 * var_str_g) + (var_ruo * var_str_g_dn6)), ((var_ruo_dn7 * var_str_g) + (var_ruo * var_str_g_dn7)), ((var_ruo_dn8 * var_str_g) + (var_ruo * var_str_g_dn8)), ((var_ruo_dn9 * var_str_g) + (var_ruo * var_str_g_dn9)),)
    } else {
        (var_rhobeta, var_rhobeta_dn4, var_rhobeta_dn6, var_rhobeta_dn7, var_rhobeta_dn8, var_rhobeta_dn9,)
    }
};
        var_rhobeta = assign6250_e6205;
        var_rhobeta_dn4 = assign6250_e6205_d_n4;
        var_rhobeta_dn6 = assign6250_e6205_d_n6;
        var_rhobeta_dn7 = assign6250_e6205_d_n7;
        var_rhobeta_dn8 = assign6250_e6205_d_n8;
        var_rhobeta_dn9 = assign6250_e6205_d_n9;

        let (assign6260_e6217, assign6260_e6217_d_n4, assign6260_e6217_d_n6, assign6260_e6217_d_n7, assign6260_e6217_d_n8, assign6260_e6217_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6260_e6215: f64 = (var_ruo * var_str_gref);
        (assign6260_e6215, ((var_ruo_dn4 * var_str_gref) + (var_ruo * var_str_gref_dn4)), ((var_ruo_dn6 * var_str_gref) + (var_ruo * var_str_gref_dn6)), ((var_ruo_dn7 * var_str_gref) + (var_ruo * var_str_gref_dn7)), ((var_ruo_dn8 * var_str_gref) + (var_ruo * var_str_gref_dn8)), ((var_ruo_dn9 * var_str_gref) + (var_ruo * var_str_gref_dn9)),)
    } else {
        (var_rhobetaref, var_rhobetaref_dn4, var_rhobetaref_dn6, var_rhobetaref_dn7, var_rhobetaref_dn8, var_rhobetaref_dn9,)
    }
};
        var_rhobetaref = assign6260_e6217;
        var_rhobetaref_dn4 = assign6260_e6217_d_n4;
        var_rhobetaref_dn6 = assign6260_e6217_d_n6;
        var_rhobetaref_dn7 = assign6260_e6217_d_n7;
        var_rhobetaref_dn8 = assign6260_e6217_d_n8;
        var_rhobetaref_dn9 = assign6260_e6217_d_n9;

        *var_guard135_slot = var_guard135;
        *var_guard136_slot = var_guard136;
        *var_guard137_slot = var_guard137;
        *var_guard138_slot = var_guard138;
        *var_iloop_slot = var_iloop;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_dn4_slot = var_rhobeta_dn4;
        *var_rhobeta_dn6_slot = var_rhobeta_dn6;
        *var_rhobeta_dn7_slot = var_rhobeta_dn7;
        *var_rhobeta_dn8_slot = var_rhobeta_dn8;
        *var_rhobeta_dn9_slot = var_rhobeta_dn9;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_dn4_slot = var_rhobetaref_dn4;
        *var_rhobetaref_dn6_slot = var_rhobetaref_dn6;
        *var_rhobetaref_dn7_slot = var_rhobetaref_dn7;
        *var_rhobetaref_dn8_slot = var_rhobetaref_dn8;
        *var_rhobetaref_dn9_slot = var_rhobetaref_dn9;
        *var_ruo_slot = var_ruo;
        *var_ruo_dn4_slot = var_ruo_dn4;
        *var_ruo_dn6_slot = var_ruo_dn6;
        *var_ruo_dn7_slot = var_ruo_dn7;
        *var_ruo_dn8_slot = var_ruo_dn8;
        *var_ruo_dn9_slot = var_ruo_dn9;
        *var_str_g_slot = var_str_g;
        *var_str_g_dn4_slot = var_str_g_dn4;
        *var_str_g_dn6_slot = var_str_g_dn6;
        *var_str_g_dn7_slot = var_str_g_dn7;
        *var_str_g_dn8_slot = var_str_g_dn8;
        *var_str_g_dn9_slot = var_str_g_dn9;
        *var_str_gref_slot = var_str_gref;
        *var_str_gref_dn4_slot = var_str_gref_dn4;
        *var_str_gref_dn6_slot = var_str_gref_dn6;
        *var_str_gref_dn7_slot = var_str_gref_dn7;
        *var_str_gref_dn8_slot = var_str_gref_dn8;
        *var_str_gref_dn9_slot = var_str_gref_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_wx_slot = var_wx;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_agidl_i: f64,
        var_agidl_i_dn4: f64,
        var_agidl_i_dn6: f64,
        var_agidl_i_dn7: f64,
        var_agidl_i_dn8: f64,
        var_agidl_i_dn9: f64,
        var_bgidl_t: f64,
        var_cfr_i: f64,
        var_cfr_i_dn4: f64,
        var_cfr_i_dn6: f64,
        var_cfr_i_dn7: f64,
        var_cfr_i_dn8: f64,
        var_cfr_i_dn9: f64,
        var_cgidl_i: f64,
        var_cov_i: f64,
        var_cov_i_dn4: f64,
        var_cov_i_dn6: f64,
        var_cov_i_dn7: f64,
        var_cov_i_dn8: f64,
        var_cov_i_dn9: f64,
        var_dgidl_i: f64,
        var_fnovinv_t: f64,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_igovacc_t: f64,
        var_igovinv_t: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_nov_i: f64,
        var_rhobeta: f64,
        var_rhobeta_dn4: f64,
        var_rhobeta_dn6: f64,
        var_rhobeta_dn7: f64,
        var_rhobeta_dn8: f64,
        var_rhobeta_dn9: f64,
        var_rhobetaref: f64,
        var_rhobetaref_dn4: f64,
        var_rhobetaref_dn6: f64,
        var_rhobetaref_dn7: f64,
        var_rhobetaref_dn8: f64,
        var_rhobetaref_dn9: f64,
        var_stbgidl_i: f64,
        var_str_g: f64,
        var_str_g_dn4: f64,
        var_str_g_dn6: f64,
        var_str_g_dn7: f64,
        var_str_g_dn8: f64,
        var_str_g_dn9: f64,
        var_str_gref: f64,
        var_str_gref_dn4: f64,
        var_str_gref_dn6: f64,
        var_str_gref_dn7: f64,
        var_str_gref_dn8: f64,
        var_str_gref_dn9: f64,
        var_tkc: f64,
        var_tkc_dn4: f64,
        var_tkc_dn6: f64,
        var_tkc_dn7: f64,
        var_tkc_dn8: f64,
        var_tkc_dn9: f64,
        var_tkc_sq: f64,
        var_tkc_sq_dn4: f64,
        var_tkc_sq_dn6: f64,
        var_tkc_sq_dn7: f64,
        var_tkc_sq_dn8: f64,
        var_tkc_sq_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_wen: f64,
        var_wx: f64,
        var_xge_i: f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_egge_slot: &mut f64,
        var_egge_dn4_slot: &mut f64,
        var_egge_dn6_slot: &mut f64,
        var_egge_dn7_slot: &mut f64,
        var_egge_dn8_slot: &mut f64,
        var_egge_dn9_slot: &mut f64,
        var_egsi_slot: &mut f64,
        var_egsi_dn4_slot: &mut f64,
        var_egsi_dn6_slot: &mut f64,
        var_egsi_dn7_slot: &mut f64,
        var_egsi_dn8_slot: &mut f64,
        var_egsi_dn9_slot: &mut f64,
        var_epsch_slot: &mut f64,
        var_fnovinvd_t_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_one_m_xge_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_egge: f64 = *var_egge_slot;
        let mut var_egge_dn4: f64 = *var_egge_dn4_slot;
        let mut var_egge_dn6: f64 = *var_egge_dn6_slot;
        let mut var_egge_dn7: f64 = *var_egge_dn7_slot;
        let mut var_egge_dn8: f64 = *var_egge_dn8_slot;
        let mut var_egge_dn9: f64 = *var_egge_dn9_slot;
        let mut var_egsi: f64 = *var_egsi_slot;
        let mut var_egsi_dn4: f64 = *var_egsi_dn4_slot;
        let mut var_egsi_dn6: f64 = *var_egsi_dn6_slot;
        let mut var_egsi_dn7: f64 = *var_egsi_dn7_slot;
        let mut var_egsi_dn8: f64 = *var_egsi_dn8_slot;
        let mut var_egsi_dn9: f64 = *var_egsi_dn9_slot;
        let mut var_epsch: f64 = *var_epsch_slot;
        let mut var_fnovinvd_t: f64 = *var_fnovinvd_t_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_one_m_xge: f64 = *var_one_m_xge_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign6270_e6229, assign6270_e6229_d_n4, assign6270_e6229_d_n6, assign6270_e6229_d_n7, assign6270_e6229_d_n8, assign6270_e6229_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6270_e6227: f64 = (var_str_g - var_str_gref);
        (assign6270_e6227, (var_str_g_dn4 - var_str_gref_dn4), (var_str_g_dn6 - var_str_gref_dn6), (var_str_g_dn7 - var_str_gref_dn7), (var_str_g_dn8 - var_str_gref_dn8), (var_str_g_dn9 - var_str_gref_dn9),)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn4, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9,)
    }
};
        var_temp0__blk79 = assign6270_e6229;
        var_temp0__blk79_dn4 = assign6270_e6229_d_n4;
        var_temp0__blk79_dn6 = assign6270_e6229_d_n6;
        var_temp0__blk79_dn7 = assign6270_e6229_d_n7;
        var_temp0__blk79_dn8 = assign6270_e6229_d_n8;
        var_temp0__blk79_dn9 = assign6270_e6229_d_n9;

        let (assign6280_e6247,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6280_e6240: f64 = (p.p484 * var_wx);
        let assign6280_e6242: f64 = (assign6280_e6240 / var_wen);
        let assign6280_e6243: f64 = (1.0 + assign6280_e6242);
        let assign6280_e6245: f64 = (assign6280_e6243).max(1e-20);
        (assign6280_e6245,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign6280_e6247;

        let (assign6290_e6265, assign6290_e6265_d_n4, assign6290_e6265_d_n6, assign6290_e6265_d_n7, assign6290_e6265_d_n8, assign6290_e6265_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6290_e6258: f64 = (1.0 + var_rhobeta);
        let assign6290_e6259: f64 = (var_betn_p * assign6290_e6258);
        let assign6290_e6262: f64 = (1.0 + var_rhobetaref);
        let assign6290_e6263: f64 = (assign6290_e6259 / assign6290_e6262);
        (assign6290_e6263, (((((var_betn_p_dn4 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn4)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn4)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn6 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn6)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn6)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn7 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn7)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn7)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn8 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn8)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn8)) / (assign6290_e6262 * assign6290_e6262)), (((((var_betn_p_dn9 * assign6290_e6258) + (var_betn_p * var_rhobeta_dn9)) * assign6290_e6262) - (assign6290_e6259 * var_rhobetaref_dn9)) / (assign6290_e6262 * assign6290_e6262)),)
    } else {
        (var_betn_p, var_betn_p_dn4, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9,)
    }
};
        var_betn_p = assign6290_e6265;
        var_betn_p_dn4 = assign6290_e6265_d_n4;
        var_betn_p_dn6 = assign6290_e6265_d_n6;
        var_betn_p_dn7 = assign6290_e6265_d_n7;
        var_betn_p_dn8 = assign6290_e6265_d_n8;
        var_betn_p_dn9 = assign6290_e6265_d_n9;

        let (assign6300_e6277, assign6300_e6277_d_n4, assign6300_e6277_d_n6, assign6300_e6277_d_n7, assign6300_e6277_d_n8, assign6300_e6277_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6300_e6275: f64 = (var_betn_p).max(1e-10);
        (assign6300_e6275, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn4, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9,)
    }
};
        var_betn1_t = assign6300_e6277;
        var_betn1_t_dn4 = assign6300_e6277_d_n4;
        var_betn1_t_dn6 = assign6300_e6277_d_n6;
        var_betn1_t_dn7 = assign6300_e6277_d_n7;
        var_betn1_t_dn8 = assign6300_e6277_d_n8;
        var_betn1_t_dn9 = assign6300_e6277_d_n9;

        let (assign6310_e6289, assign6310_e6289_d_n4, assign6310_e6289_d_n6, assign6310_e6289_d_n7, assign6310_e6289_d_n8, assign6310_e6289_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6310_e6287: f64 = (p.p254 * var_betn1_t);
        (assign6310_e6287, (p.p254 * var_betn1_t_dn4), (p.p254 * var_betn1_t_dn6), (p.p254 * var_betn1_t_dn7), (p.p254 * var_betn1_t_dn8), (p.p254 * var_betn1_t_dn9),)
    } else {
        (var_betn2_t, var_betn2_t_dn4, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9,)
    }
};
        var_betn2_t = assign6310_e6289;
        var_betn2_t_dn4 = assign6310_e6289_d_n4;
        var_betn2_t_dn6 = assign6310_e6289_d_n6;
        var_betn2_t_dn7 = assign6310_e6289_d_n7;
        var_betn2_t_dn8 = assign6310_e6289_d_n8;
        var_betn2_t_dn9 = assign6310_e6289_d_n9;

        let (assign6320_e6317, assign6320_e6317_d_n4, assign6320_e6317_d_n6, assign6320_e6317_d_n7, assign6320_e6317_d_n8, assign6320_e6317_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6320_e6299: f64 = (1.0 + var_rhobeta);
        let assign6320_e6303: f64 = (p.p488 * var_rhobetaref);
        let assign6320_e6304: f64 = (1.0 + assign6320_e6303);
        let assign6320_e6305: f64 = (assign6320_e6299 * assign6320_e6304);
        let assign6320_e6308: f64 = (1.0 + var_rhobetaref);
        let assign6320_e6312: f64 = (p.p488 * var_rhobeta);
        let assign6320_e6313: f64 = (1.0 + assign6320_e6312);
        let assign6320_e6314: f64 = (assign6320_e6308 * assign6320_e6313);
        let assign6320_e6315: f64 = (assign6320_e6305 / assign6320_e6314);
        (assign6320_e6315, (((((var_rhobeta_dn4 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn4))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn4 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn4))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn6 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn6))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn6 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn6))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn7 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn7))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn7 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn7))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn8 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn8))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn8 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn8))))) / (assign6320_e6314 * assign6320_e6314)), (((((var_rhobeta_dn9 * assign6320_e6304) + (assign6320_e6299 * (p.p488 * var_rhobetaref_dn9))) * assign6320_e6314) - (assign6320_e6305 * ((var_rhobetaref_dn9 * assign6320_e6313) + (assign6320_e6308 * (p.p488 * var_rhobeta_dn9))))) / (assign6320_e6314 * assign6320_e6314)),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6320_e6317;
        var_temp_dn4 = assign6320_e6317_d_n4;
        var_temp_dn6 = assign6320_e6317_d_n6;
        var_temp_dn7 = assign6320_e6317_d_n7;
        var_temp_dn8 = assign6320_e6317_d_n8;
        var_temp_dn9 = assign6320_e6317_d_n9;

        let (assign6330_e6329, assign6330_e6329_d_n4, assign6330_e6329_d_n6, assign6330_e6329_d_n7, assign6330_e6329_d_n8, assign6330_e6329_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6330_e6327: f64 = (var_thesat_p * var_temp);
        (assign6330_e6327, ((var_thesat_p_dn4 * var_temp) + (var_thesat_p * var_temp_dn4)), ((var_thesat_p_dn6 * var_temp) + (var_thesat_p * var_temp_dn6)), ((var_thesat_p_dn7 * var_temp) + (var_thesat_p * var_temp_dn7)), ((var_thesat_p_dn8 * var_temp) + (var_thesat_p * var_temp_dn8)), ((var_thesat_p_dn9 * var_temp) + (var_thesat_p * var_temp_dn9)),)
    } else {
        (var_thesat_p, var_thesat_p_dn4, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9,)
    }
};
        var_thesat_p = assign6330_e6329;
        var_thesat_p_dn4 = assign6330_e6329_d_n4;
        var_thesat_p_dn6 = assign6330_e6329_d_n6;
        var_thesat_p_dn7 = assign6330_e6329_d_n7;
        var_thesat_p_dn8 = assign6330_e6329_d_n8;
        var_thesat_p_dn9 = assign6330_e6329_d_n9;

        let (assign6340_e6341, assign6340_e6341_d_n4, assign6340_e6341_d_n6, assign6340_e6341_d_n7, assign6340_e6341_d_n8, assign6340_e6341_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6340_e6339: f64 = (var_thesat_p).max(0.0);
        (assign6340_e6339, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn4, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9,)
    }
};
        var_thesat_t = assign6340_e6341;
        var_thesat_t_dn4 = assign6340_e6341_d_n4;
        var_thesat_t_dn6 = assign6340_e6341_d_n6;
        var_thesat_t_dn7 = assign6340_e6341_d_n7;
        var_thesat_t_dn8 = assign6340_e6341_d_n8;
        var_thesat_t_dn9 = assign6340_e6341_d_n9;

        let (assign6350_e6353, assign6350_e6353_d_n4, assign6350_e6353_d_n6, assign6350_e6353_d_n7, assign6350_e6353_d_n8, assign6350_e6353_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6350_e6351: f64 = (var_thesatac_p * var_temp);
        (assign6350_e6351, ((var_thesatac_p_dn4 * var_temp) + (var_thesatac_p * var_temp_dn4)), ((var_thesatac_p_dn6 * var_temp) + (var_thesatac_p * var_temp_dn6)), ((var_thesatac_p_dn7 * var_temp) + (var_thesatac_p * var_temp_dn7)), ((var_thesatac_p_dn8 * var_temp) + (var_thesatac_p * var_temp_dn8)), ((var_thesatac_p_dn9 * var_temp) + (var_thesatac_p * var_temp_dn9)),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn4, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9,)
    }
};
        var_thesatac_p = assign6350_e6353;
        var_thesatac_p_dn4 = assign6350_e6353_d_n4;
        var_thesatac_p_dn6 = assign6350_e6353_d_n6;
        var_thesatac_p_dn7 = assign6350_e6353_d_n7;
        var_thesatac_p_dn8 = assign6350_e6353_d_n8;
        var_thesatac_p_dn9 = assign6350_e6353_d_n9;

        let (assign6360_e6365, assign6360_e6365_d_n4, assign6360_e6365_d_n6, assign6360_e6365_d_n7, assign6360_e6365_d_n8, assign6360_e6365_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6360_e6363: f64 = (var_thesatac_p).max(0.0);
        (assign6360_e6363, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn4, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9,)
    }
};
        var_thesatac_t = assign6360_e6365;
        var_thesatac_t_dn4 = assign6360_e6365_d_n4;
        var_thesatac_t_dn6 = assign6360_e6365_d_n6;
        var_thesatac_t_dn7 = assign6360_e6365_d_n7;
        var_thesatac_t_dn8 = assign6360_e6365_d_n8;
        var_thesatac_t_dn9 = assign6360_e6365_d_n9;

        let (assign6370_e6379, assign6370_e6379_d_n4, assign6370_e6379_d_n6, assign6370_e6379_d_n7, assign6370_e6379_d_n8, assign6370_e6379_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6370_e6375: f64 = (p.p483 * var_temp0__blk79);
        let assign6370_e6377: f64 = (assign6370_e6375 / var_kstressvth0);
        (assign6370_e6377, ((p.p483 * var_temp0__blk79_dn4) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn6) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn7) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn8) / var_kstressvth0), ((p.p483 * var_temp0__blk79_dn9) / var_kstressvth0),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6370_e6379;
        var_temp_dn4 = assign6370_e6379_d_n4;
        var_temp_dn6 = assign6370_e6379_d_n6;
        var_temp_dn7 = assign6370_e6379_d_n7;
        var_temp_dn8 = assign6370_e6379_d_n8;
        var_temp_dn9 = assign6370_e6379_d_n9;

        let (assign6380_e6391, assign6380_e6391_d_n4, assign6380_e6391_d_n6, assign6380_e6391_d_n7, assign6380_e6391_d_n8, assign6380_e6391_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6380_e6389: f64 = (var_vfb1_t + var_temp);
        (assign6380_e6389, (var_vfb1_t_dn4 + var_temp_dn4), (var_vfb1_t_dn6 + var_temp_dn6), (var_vfb1_t_dn7 + var_temp_dn7), (var_vfb1_t_dn8 + var_temp_dn8), (var_vfb1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb1_t, var_vfb1_t_dn4, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9,)
    }
};
        var_vfb1_t = assign6380_e6391;
        var_vfb1_t_dn4 = assign6380_e6391_d_n4;
        var_vfb1_t_dn6 = assign6380_e6391_d_n6;
        var_vfb1_t_dn7 = assign6380_e6391_d_n7;
        var_vfb1_t_dn8 = assign6380_e6391_d_n8;
        var_vfb1_t_dn9 = assign6380_e6391_d_n9;

        let (assign6390_e6403, assign6390_e6403_d_n4, assign6390_e6403_d_n6, assign6390_e6403_d_n7, assign6390_e6403_d_n8, assign6390_e6403_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6390_e6401: f64 = (var_vfb2_t + var_temp);
        (assign6390_e6401, (var_vfb2_t_dn4 + var_temp_dn4), (var_vfb2_t_dn6 + var_temp_dn6), (var_vfb2_t_dn7 + var_temp_dn7), (var_vfb2_t_dn8 + var_temp_dn8), (var_vfb2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign6390_e6403;
        var_vfb2_t_dn4 = assign6390_e6403_d_n4;
        var_vfb2_t_dn6 = assign6390_e6403_d_n6;
        var_vfb2_t_dn7 = assign6390_e6403_d_n7;
        var_vfb2_t_dn8 = assign6390_e6403_d_n8;
        var_vfb2_t_dn9 = assign6390_e6403_d_n9;

        let (assign6400_e6415, assign6400_e6415_d_n4, assign6400_e6415_d_n6, assign6400_e6415_d_n7, assign6400_e6415_d_n8, assign6400_e6415_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6400_e6413: f64 = (var_vfbac1_t + var_temp);
        (assign6400_e6413, (var_vfbac1_t_dn4 + var_temp_dn4), (var_vfbac1_t_dn6 + var_temp_dn6), (var_vfbac1_t_dn7 + var_temp_dn7), (var_vfbac1_t_dn8 + var_temp_dn8), (var_vfbac1_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn4, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9,)
    }
};
        var_vfbac1_t = assign6400_e6415;
        var_vfbac1_t_dn4 = assign6400_e6415_d_n4;
        var_vfbac1_t_dn6 = assign6400_e6415_d_n6;
        var_vfbac1_t_dn7 = assign6400_e6415_d_n7;
        var_vfbac1_t_dn8 = assign6400_e6415_d_n8;
        var_vfbac1_t_dn9 = assign6400_e6415_d_n9;

        let (assign6410_e6427, assign6410_e6427_d_n4, assign6410_e6427_d_n6, assign6410_e6427_d_n7, assign6410_e6427_d_n8, assign6410_e6427_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6410_e6425: f64 = (var_vfbac2_t + var_temp);
        (assign6410_e6425, (var_vfbac2_t_dn4 + var_temp_dn4), (var_vfbac2_t_dn6 + var_temp_dn6), (var_vfbac2_t_dn7 + var_temp_dn7), (var_vfbac2_t_dn8 + var_temp_dn8), (var_vfbac2_t_dn9 + var_temp_dn9),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign6410_e6427;
        var_vfbac2_t_dn4 = assign6410_e6427_d_n4;
        var_vfbac2_t_dn6 = assign6410_e6427_d_n6;
        var_vfbac2_t_dn7 = assign6410_e6427_d_n7;
        var_vfbac2_t_dn8 = assign6410_e6427_d_n8;
        var_vfbac2_t_dn9 = assign6410_e6427_d_n9;

        let (assign6420_e6449, assign6420_e6449_d_n4, assign6420_e6449_d_n6, assign6420_e6449_d_n7, assign6420_e6449_d_n8, assign6420_e6449_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6420_e6437: f64 = (p.p485 * var_temp0__blk79);
        let assign6420_e6440: f64 = (var_lambda_le).powf(p.p236);
        let assign6420_e6441: f64 = (assign6420_e6437 * assign6420_e6440);
        let assign6420_e6445: f64 = (p.p237 * var_iwe);
        let assign6420_e6446: f64 = (1.0 + assign6420_e6445);
        let assign6420_e6447: f64 = (assign6420_e6441 * assign6420_e6446);
        (assign6420_e6447, (((p.p485 * var_temp0__blk79_dn4) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn6) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn7) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn8) * assign6420_e6440) * assign6420_e6446), (((p.p485 * var_temp0__blk79_dn9) * assign6420_e6440) * assign6420_e6446),)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6420_e6449;
        var_temp_dn4 = assign6420_e6449_d_n4;
        var_temp_dn6 = assign6420_e6449_d_n6;
        var_temp_dn7 = assign6420_e6449_d_n7;
        var_temp_dn8 = assign6420_e6449_d_n8;
        var_temp_dn9 = assign6420_e6449_d_n9;

        let (assign6430_e6461, assign6430_e6461_d_n4, assign6430_e6461_d_n6, assign6430_e6461_d_n7, assign6430_e6461_d_n8, assign6430_e6461_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6430_e6459: f64 = (var_cf_p + var_temp);
        (assign6430_e6459, (var_cf_p_dn4 + var_temp_dn4), (var_cf_p_dn6 + var_temp_dn6), (var_cf_p_dn7 + var_temp_dn7), (var_cf_p_dn8 + var_temp_dn8), (var_cf_p_dn9 + var_temp_dn9),)
    } else {
        (var_cf_p, var_cf_p_dn4, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9,)
    }
};
        var_cf_p = assign6430_e6461;
        var_cf_p_dn4 = assign6430_e6461_d_n4;
        var_cf_p_dn6 = assign6430_e6461_d_n6;
        var_cf_p_dn7 = assign6430_e6461_d_n7;
        var_cf_p_dn8 = assign6430_e6461_d_n8;
        var_cf_p_dn9 = assign6430_e6461_d_n9;

        let (assign6440_e6473, assign6440_e6473_d_n4, assign6440_e6473_d_n6, assign6440_e6473_d_n7, assign6440_e6473_d_n8, assign6440_e6473_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6440_e6471: f64 = (var_cf_p).max(0.0);
        (assign6440_e6471, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn4, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9,)
    }
};
        var_cf1_t = assign6440_e6473;
        var_cf1_t_dn4 = assign6440_e6473_d_n4;
        var_cf1_t_dn6 = assign6440_e6473_d_n6;
        var_cf1_t_dn7 = assign6440_e6473_d_n7;
        var_cf1_t_dn8 = assign6440_e6473_d_n8;
        var_cf1_t_dn9 = assign6440_e6473_d_n9;

        let (assign6450_e6485, assign6450_e6485_d_n4, assign6450_e6485_d_n6, assign6450_e6485_d_n7, assign6450_e6485_d_n8, assign6450_e6485_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6450_e6483: f64 = (var_cfac_p + var_temp);
        (assign6450_e6483, (var_cfac_p_dn4 + var_temp_dn4), (var_cfac_p_dn6 + var_temp_dn6), (var_cfac_p_dn7 + var_temp_dn7), (var_cfac_p_dn8 + var_temp_dn8), (var_cfac_p_dn9 + var_temp_dn9),)
    } else {
        (var_cfac_p, var_cfac_p_dn4, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9,)
    }
};
        var_cfac_p = assign6450_e6485;
        var_cfac_p_dn4 = assign6450_e6485_d_n4;
        var_cfac_p_dn6 = assign6450_e6485_d_n6;
        var_cfac_p_dn7 = assign6450_e6485_d_n7;
        var_cfac_p_dn8 = assign6450_e6485_d_n8;
        var_cfac_p_dn9 = assign6450_e6485_d_n9;

        let (assign6460_e6497, assign6460_e6497_d_n4, assign6460_e6497_d_n6, assign6460_e6497_d_n7, assign6460_e6497_d_n8, assign6460_e6497_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6460_e6495: f64 = (var_cfac_p).max(0.0);
        (assign6460_e6495, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn4, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9,)
    }
};
        var_cfac1_t = assign6460_e6497;
        var_cfac1_t_dn4 = assign6460_e6497_d_n4;
        var_cfac1_t_dn6 = assign6460_e6497_d_n6;
        var_cfac1_t_dn7 = assign6460_e6497_d_n7;
        var_cfac1_t_dn8 = assign6460_e6497_d_n8;
        var_cfac1_t_dn9 = assign6460_e6497_d_n9;

        let (assign6470_e6511, assign6470_e6511_d_n4, assign6470_e6511_d_n6, assign6470_e6511_d_n7, assign6470_e6511_d_n8, assign6470_e6511_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6470_e6507: f64 = (p.p238 * var_tox2_i);
        let assign6470_e6509: f64 = (assign6470_e6507 / var_tox1_i);
        (assign6470_e6509, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn4, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9,)
    }
};
        var_temp = assign6470_e6511;
        var_temp_dn4 = assign6470_e6511_d_n4;
        var_temp_dn6 = assign6470_e6511_d_n6;
        var_temp_dn7 = assign6470_e6511_d_n7;
        var_temp_dn8 = assign6470_e6511_d_n8;
        var_temp_dn9 = assign6470_e6511_d_n9;

        let (assign6480_e6523, assign6480_e6523_d_n4, assign6480_e6523_d_n6, assign6480_e6523_d_n7, assign6480_e6523_d_n8, assign6480_e6523_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6480_e6521: f64 = (var_cf1_t * var_temp);
        (assign6480_e6521, ((var_cf1_t_dn4 * var_temp) + (var_cf1_t * var_temp_dn4)), ((var_cf1_t_dn6 * var_temp) + (var_cf1_t * var_temp_dn6)), ((var_cf1_t_dn7 * var_temp) + (var_cf1_t * var_temp_dn7)), ((var_cf1_t_dn8 * var_temp) + (var_cf1_t * var_temp_dn8)), ((var_cf1_t_dn9 * var_temp) + (var_cf1_t * var_temp_dn9)),)
    } else {
        (var_cf2_t, var_cf2_t_dn4, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9,)
    }
};
        var_cf2_t = assign6480_e6523;
        var_cf2_t_dn4 = assign6480_e6523_d_n4;
        var_cf2_t_dn6 = assign6480_e6523_d_n6;
        var_cf2_t_dn7 = assign6480_e6523_d_n7;
        var_cf2_t_dn8 = assign6480_e6523_d_n8;
        var_cf2_t_dn9 = assign6480_e6523_d_n9;

        let (assign6490_e6535, assign6490_e6535_d_n4, assign6490_e6535_d_n6, assign6490_e6535_d_n7, assign6490_e6535_d_n8, assign6490_e6535_d_n9,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6490_e6533: f64 = (var_cfac1_t * var_temp);
        (assign6490_e6533, ((var_cfac1_t_dn4 * var_temp) + (var_cfac1_t * var_temp_dn4)), ((var_cfac1_t_dn6 * var_temp) + (var_cfac1_t * var_temp_dn6)), ((var_cfac1_t_dn7 * var_temp) + (var_cfac1_t * var_temp_dn7)), ((var_cfac1_t_dn8 * var_temp) + (var_cfac1_t * var_temp_dn8)), ((var_cfac1_t_dn9 * var_temp) + (var_cfac1_t * var_temp_dn9)),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn4, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9,)
    }
};
        var_cfac2_t = assign6490_e6535;
        var_cfac2_t_dn4 = assign6490_e6535_d_n4;
        var_cfac2_t_dn6 = assign6490_e6535_d_n6;
        var_cfac2_t_dn7 = assign6490_e6535_d_n7;
        var_cfac2_t_dn8 = assign6490_e6535_d_n8;
        var_cfac2_t_dn9 = assign6490_e6535_d_n9;

        let assign6500_e6538: f64 = if p.p7 == 0.0 { 1.0 } else { 0.0 };
        var_guard139 = assign6500_e6538;

        let (assign6510_e6542,) = {
    if (var_guard139 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign6510_e6542;

        let (assign6520_e6546,) = {
    if (var_guard139 != 0.0) {
        (var_igovinv_t,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign6520_e6546;

        let (assign6530_e6550,) = {
    if (var_guard139 != 0.0) {
        (var_fnovinv_t,)
    } else {
        (var_fnovinvd_t,)
    }
};
        var_fnovinvd_t = assign6530_e6550;

        let (assign6540_e6554,) = {
    if (var_guard139 != 0.0) {
        (var_igovacc_t,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign6540_e6554;

        let (assign6550_e6558, assign6550_e6558_d_n4, assign6550_e6558_d_n6, assign6550_e6558_d_n7, assign6550_e6558_d_n8, assign6550_e6558_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_agidl_i, var_agidl_i_dn4, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9,)
    } else {
        (var_agidld_i, var_agidld_i_dn4, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9,)
    }
};
        var_agidld_i = assign6550_e6558;
        var_agidld_i_dn4 = assign6550_e6558_d_n4;
        var_agidld_i_dn6 = assign6550_e6558_d_n6;
        var_agidld_i_dn7 = assign6550_e6558_d_n7;
        var_agidld_i_dn8 = assign6550_e6558_d_n8;
        var_agidld_i_dn9 = assign6550_e6558_d_n9;

        let (assign6560_e6562,) = {
    if (var_guard139 != 0.0) {
        (var_bgidl_t,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign6560_e6562;

        let (assign6570_e6566,) = {
    if (var_guard139 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign6570_e6566;

        let (assign6580_e6570,) = {
    if (var_guard139 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign6580_e6570;

        let (assign6590_e6574,) = {
    if (var_guard139 != 0.0) {
        (var_dgidl_i,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign6590_e6574;

        let (assign6600_e6578, assign6600_e6578_d_n4, assign6600_e6578_d_n6, assign6600_e6578_d_n7, assign6600_e6578_d_n8, assign6600_e6578_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_cov_i, var_cov_i_dn4, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9,)
    } else {
        (var_covd_i, var_covd_i_dn4, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9,)
    }
};
        var_covd_i = assign6600_e6578;
        var_covd_i_dn4 = assign6600_e6578_d_n4;
        var_covd_i_dn6 = assign6600_e6578_d_n6;
        var_covd_i_dn7 = assign6600_e6578_d_n7;
        var_covd_i_dn8 = assign6600_e6578_d_n8;
        var_covd_i_dn9 = assign6600_e6578_d_n9;

        let (assign6610_e6582, assign6610_e6582_d_n4, assign6610_e6582_d_n6, assign6610_e6582_d_n7, assign6610_e6582_d_n8, assign6610_e6582_d_n9,) = {
    if (var_guard139 != 0.0) {
        (var_cfr_i, var_cfr_i_dn4, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9,)
    } else {
        (var_cfrd_i, var_cfrd_i_dn4, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9,)
    }
};
        var_cfrd_i = assign6610_e6582;
        var_cfrd_i_dn4 = assign6610_e6582_d_n4;
        var_cfrd_i_dn6 = assign6610_e6582_d_n6;
        var_cfrd_i_dn7 = assign6610_e6582_d_n7;
        var_cfrd_i_dn8 = assign6610_e6582_d_n8;
        var_cfrd_i_dn9 = assign6610_e6582_d_n9;

        let assign6620_e6585: f64 = (1.0 - var_xge_i);
        var_one_m_xge = assign6620_e6585;

        let assign6630_e6588: f64 = (1.04479e-10 * var_one_m_xge);
        let assign6630_e6591: f64 = (1.43438e-10 * var_xge_i);
        let assign6630_e6592: f64 = (assign6630_e6588 + assign6630_e6591);
        var_epsch = assign6630_e6592;

        let assign6640_e6596: f64 = (0.000473 * var_tkc_sq);
        let assign6640_e6599: f64 = (636.0 + var_tkc);
        let assign6640_e6600: f64 = (assign6640_e6596 / assign6640_e6599);
        let assign6640_e6601: f64 = (1.17 - assign6640_e6600);
        var_egsi = assign6640_e6601;
        var_egsi_dn4 = (-((((0.000473 * var_tkc_sq_dn4) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn4)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn6 = (-((((0.000473 * var_tkc_sq_dn6) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn6)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn7 = (-((((0.000473 * var_tkc_sq_dn7) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn7)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn8 = (-((((0.000473 * var_tkc_sq_dn8) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn8)) / (assign6640_e6599 * assign6640_e6599)));
        var_egsi_dn9 = (-((((0.000473 * var_tkc_sq_dn9) * assign6640_e6599) - (assign6640_e6596 * var_tkc_dn9)) / (assign6640_e6599 * assign6640_e6599)));

        let assign6650_e6605: f64 = (0.0004774 * var_tkc_sq);
        let assign6650_e6608: f64 = (235.0 + var_tkc);
        let assign6650_e6609: f64 = (assign6650_e6605 / assign6650_e6608);
        let assign6650_e6610: f64 = (0.744 - assign6650_e6609);
        var_egge = assign6650_e6610;
        var_egge_dn4 = (-((((0.0004774 * var_tkc_sq_dn4) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn4)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn6 = (-((((0.0004774 * var_tkc_sq_dn6) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn6)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn7 = (-((((0.0004774 * var_tkc_sq_dn7) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn7)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn8 = (-((((0.0004774 * var_tkc_sq_dn8) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn8)) / (assign6650_e6608 * assign6650_e6608)));
        var_egge_dn9 = (-((((0.0004774 * var_tkc_sq_dn9) * assign6650_e6608) - (assign6650_e6605 * var_tkc_dn9)) / (assign6650_e6608 * assign6650_e6608)));

        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_egge_slot = var_egge;
        *var_egge_dn4_slot = var_egge_dn4;
        *var_egge_dn6_slot = var_egge_dn6;
        *var_egge_dn7_slot = var_egge_dn7;
        *var_egge_dn8_slot = var_egge_dn8;
        *var_egge_dn9_slot = var_egge_dn9;
        *var_egsi_slot = var_egsi;
        *var_egsi_dn4_slot = var_egsi_dn4;
        *var_egsi_dn6_slot = var_egsi_dn6;
        *var_egsi_dn7_slot = var_egsi_dn7;
        *var_egsi_dn8_slot = var_egsi_dn8;
        *var_egsi_dn9_slot = var_egsi_dn9;
        *var_epsch_slot = var_epsch;
        *var_fnovinvd_t_slot = var_fnovinvd_t;
        *var_guard139_slot = var_guard139;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_novd_i_slot = var_novd_i;
        *var_one_m_xge_slot = var_one_m_xge;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_cf1_t: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_cfac1_t: f64,
        var_cfac1_t_dn4: f64,
        var_cfac1_t_dn6: f64,
        var_cfac1_t_dn7: f64,
        var_cfac1_t_dn8: f64,
        var_cfac1_t_dn9: f64,
        var_cfac2_t: f64,
        var_cfac2_t_dn4: f64,
        var_cfac2_t_dn6: f64,
        var_cfac2_t_dn7: f64,
        var_cfac2_t_dn8: f64,
        var_cfac2_t_dn9: f64,
        var_cfd_i: f64,
        var_ct_i: f64,
        var_dt: f64,
        var_dt_dn4: f64,
        var_dt_dn6: f64,
        var_dt_dn7: f64,
        var_dt_dn8: f64,
        var_dt_dn9: f64,
        var_egge: f64,
        var_egge_dn4: f64,
        var_egge_dn6: f64,
        var_egge_dn7: f64,
        var_egge_dn8: f64,
        var_egge_dn9: f64,
        var_egsi: f64,
        var_egsi_dn4: f64,
        var_egsi_dn6: f64,
        var_egsi_dn7: f64,
        var_egsi_dn8: f64,
        var_egsi_dn9: f64,
        var_epsch: f64,
        var_inv_phit0: f64,
        var_inv_phit0_dn4: f64,
        var_inv_phit0_dn6: f64,
        var_inv_phit0_dn7: f64,
        var_inv_phit0_dn8: f64,
        var_inv_phit0_dn9: f64,
        var_nch_i: f64,
        var_np_i: f64,
        var_np_i_dn4: f64,
        var_np_i_dn6: f64,
        var_np_i_dn7: f64,
        var_np_i_dn8: f64,
        var_np_i_dn9: f64,
        var_nsddc_i: f64,
        var_nsub_i: f64,
        var_one_m_xge: f64,
        var_phit0: f64,
        var_phit0_dn4: f64,
        var_phit0_dn6: f64,
        var_phit0_dn7: f64,
        var_phit0_dn8: f64,
        var_phit0_dn9: f64,
        var_pnce_i: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_rtn_dn6: f64,
        var_rtn_dn7: f64,
        var_rtn_dn8: f64,
        var_rtn_dn9: f64,
        var_stcf_i: f64,
        var_stcf_i_dn4: f64,
        var_stcf_i_dn6: f64,
        var_stcf_i_dn7: f64,
        var_stcf_i_dn8: f64,
        var_stcf_i_dn9: f64,
        var_tkc: f64,
        var_tkc_dn4: f64,
        var_tkc_dn6: f64,
        var_tkc_dn7: f64,
        var_tkc_dn8: f64,
        var_tkc_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_tsi_i: f64,
        var_typech_i: f64,
        var_typesub_i: f64,
        var_xge_i: f64,
        var_a0_csisq_slot: &mut f64,
        var_a0_csisq_dn4_slot: &mut f64,
        var_a0_csisq_dn6_slot: &mut f64,
        var_a0_csisq_dn7_slot: &mut f64,
        var_a0_csisq_dn8_slot: &mut f64,
        var_a0_csisq_dn9_slot: &mut f64,
        var_cf1_i_slot: &mut f64,
        var_cf1_i_dn4_slot: &mut f64,
        var_cf1_i_dn6_slot: &mut f64,
        var_cf1_i_dn7_slot: &mut f64,
        var_cf1_i_dn8_slot: &mut f64,
        var_cf1_i_dn9_slot: &mut f64,
        var_cf2_i_slot: &mut f64,
        var_cf2_i_dn4_slot: &mut f64,
        var_cf2_i_dn6_slot: &mut f64,
        var_cf2_i_dn7_slot: &mut f64,
        var_cf2_i_dn8_slot: &mut f64,
        var_cf2_i_dn9_slot: &mut f64,
        var_cfac1_i_slot: &mut f64,
        var_cfac1_i_dn4_slot: &mut f64,
        var_cfac1_i_dn6_slot: &mut f64,
        var_cfac1_i_dn7_slot: &mut f64,
        var_cfac1_i_dn8_slot: &mut f64,
        var_cfac1_i_dn9_slot: &mut f64,
        var_cfac2_i_slot: &mut f64,
        var_cfac2_i_dn4_slot: &mut f64,
        var_cfac2_i_dn6_slot: &mut f64,
        var_cfac2_i_dn7_slot: &mut f64,
        var_cfac2_i_dn8_slot: &mut f64,
        var_cfac2_i_dn9_slot: &mut f64,
        var_cox1init_slot: &mut f64,
        var_cox1prime_slot: &mut f64,
        var_cox2init_slot: &mut f64,
        var_cox2prime_slot: &mut f64,
        var_csiprime_0_slot: &mut f64,
        var_deg_slot: &mut f64,
        var_deg_dn4_slot: &mut f64,
        var_deg_dn6_slot: &mut f64,
        var_deg_dn7_slot: &mut f64,
        var_deg_dn8_slot: &mut f64,
        var_deg_dn9_slot: &mut f64,
        var_dvfb1nch_slot: &mut f64,
        var_dvfb1nch_dn4_slot: &mut f64,
        var_dvfb1nch_dn6_slot: &mut f64,
        var_dvfb1nch_dn7_slot: &mut f64,
        var_dvfb1nch_dn8_slot: &mut f64,
        var_dvfb1nch_dn9_slot: &mut f64,
        var_dvfb2nch_slot: &mut f64,
        var_dvfb2nch_dn4_slot: &mut f64,
        var_dvfb2nch_dn6_slot: &mut f64,
        var_dvfb2nch_dn7_slot: &mut f64,
        var_dvfb2nch_dn8_slot: &mut f64,
        var_dvfb2nch_dn9_slot: &mut f64,
        var_dvfbch_slot: &mut f64,
        var_dvfbch_dn4_slot: &mut f64,
        var_dvfbch_dn6_slot: &mut f64,
        var_dvfbch_dn7_slot: &mut f64,
        var_dvfbch_dn8_slot: &mut f64,
        var_dvfbch_dn9_slot: &mut f64,
        var_dvfbpdep_slot: &mut f64,
        var_dvfbpdep_dn4_slot: &mut f64,
        var_dvfbpdep_dn6_slot: &mut f64,
        var_dvfbpdep_dn7_slot: &mut f64,
        var_dvfbpdep_dn8_slot: &mut f64,
        var_dvfbpdep_dn9_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_2phit_slot: &mut f64,
        var_eg_2phit0_slot: &mut f64,
        var_eg_2phit0_dn4_slot: &mut f64,
        var_eg_2phit0_dn6_slot: &mut f64,
        var_eg_2phit0_dn7_slot: &mut f64,
        var_eg_2phit0_dn8_slot: &mut f64,
        var_eg_2phit0_dn9_slot: &mut f64,
        var_eg_2phit0_woshe_slot: &mut f64,
        var_eg_2phit0_woshe_dn4_slot: &mut f64,
        var_eg_2phit0_woshe_dn6_slot: &mut f64,
        var_eg_2phit0_woshe_dn7_slot: &mut f64,
        var_eg_2phit0_woshe_dn8_slot: &mut f64,
        var_eg_2phit0_woshe_dn9_slot: &mut f64,
        var_eg_2phit_dn4_slot: &mut f64,
        var_eg_2phit_dn6_slot: &mut f64,
        var_eg_2phit_dn7_slot: &mut f64,
        var_eg_2phit_dn8_slot: &mut f64,
        var_eg_2phit_dn9_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_dn8_slot: &mut f64,
        var_eg_dn9_slot: &mut f64,
        var_emin_slot: &mut f64,
        var_emin_dn4_slot: &mut f64,
        var_emin_dn6_slot: &mut f64,
        var_emin_dn7_slot: &mut f64,
        var_emin_dn8_slot: &mut f64,
        var_emin_dn9_slot: &mut f64,
        var_gfsub_slot: &mut f64,
        var_gfsub2_slot: &mut f64,
        var_gfsub2_dn4_slot: &mut f64,
        var_gfsub2_dn6_slot: &mut f64,
        var_gfsub2_dn7_slot: &mut f64,
        var_gfsub2_dn8_slot: &mut f64,
        var_gfsub2_dn9_slot: &mut f64,
        var_gfsub_dn4_slot: &mut f64,
        var_gfsub_dn6_slot: &mut f64,
        var_gfsub_dn7_slot: &mut f64,
        var_gfsub_dn8_slot: &mut f64,
        var_gfsub_dn9_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard142_slot: &mut f64,
        var_guard143_slot: &mut f64,
        var_inv_gfsub2_slot: &mut f64,
        var_inv_gfsub2_dn4_slot: &mut f64,
        var_inv_gfsub2_dn6_slot: &mut f64,
        var_inv_gfsub2_dn7_slot: &mut f64,
        var_inv_gfsub2_dn8_slot: &mut f64,
        var_inv_gfsub2_dn9_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_dn4_slot: &mut f64,
        var_inv_phit_dn6_slot: &mut f64,
        var_inv_phit_dn7_slot: &mut f64,
        var_inv_phit_dn8_slot: &mut f64,
        var_inv_phit_dn9_slot: &mut f64,
        var_inv_xisub_slot: &mut f64,
        var_inv_xisub_dn4_slot: &mut f64,
        var_inv_xisub_dn6_slot: &mut f64,
        var_inv_xisub_dn7_slot: &mut f64,
        var_inv_xisub_dn8_slot: &mut f64,
        var_inv_xisub_dn9_slot: &mut f64,
        var_k1_1d_slot: &mut f64,
        var_k2_1d_slot: &mut f64,
        var_keq_1d_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_dn4_slot: &mut f64,
        var_kp_dn6_slot: &mut f64,
        var_kp_dn7_slot: &mut f64,
        var_kp_dn8_slot: &mut f64,
        var_kp_dn9_slot: &mut f64,
        var_margin_sub_slot: &mut f64,
        var_neff_slot: &mut f64,
        var_neff_dn4_slot: &mut f64,
        var_neff_dn6_slot: &mut f64,
        var_neff_dn7_slot: &mut f64,
        var_neff_dn8_slot: &mut f64,
        var_neff_dn9_slot: &mut f64,
        var_neff_poly_slot: &mut f64,
        var_neff_poly_dn4_slot: &mut f64,
        var_neff_poly_dn6_slot: &mut f64,
        var_neff_poly_dn7_slot: &mut f64,
        var_neff_poly_dn8_slot: &mut f64,
        var_neff_poly_dn9_slot: &mut f64,
        var_neff_sub_slot: &mut f64,
        var_neff_sub_dn4_slot: &mut f64,
        var_neff_sub_dn6_slot: &mut f64,
        var_neff_sub_dn7_slot: &mut f64,
        var_neff_sub_dn8_slot: &mut f64,
        var_neff_sub_dn9_slot: &mut f64,
        var_niratio_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_dn4_slot: &mut f64,
        var_phit_dn6_slot: &mut f64,
        var_phit_dn7_slot: &mut f64,
        var_phit_dn8_slot: &mut f64,
        var_phit_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
        var_xb_sub_slot: &mut f64,
        var_xb_sub_dn4_slot: &mut f64,
        var_xb_sub_dn6_slot: &mut f64,
        var_xb_sub_dn7_slot: &mut f64,
        var_xb_sub_dn8_slot: &mut f64,
        var_xb_sub_dn9_slot: &mut f64,
        var_xd0_slot: &mut f64,
        var_xd0_dn4_slot: &mut f64,
        var_xd0_dn6_slot: &mut f64,
        var_xd0_dn7_slot: &mut f64,
        var_xd0_dn8_slot: &mut f64,
        var_xd0_dn9_slot: &mut f64,
        var_xisub_slot: &mut f64,
        var_xisub_dn4_slot: &mut f64,
        var_xisub_dn6_slot: &mut f64,
        var_xisub_dn7_slot: &mut f64,
        var_xisub_dn8_slot: &mut f64,
        var_xisub_dn9_slot: &mut f64,
        var_xn_sub_slot: &mut f64,
        var_xn_sub_dn4_slot: &mut f64,
        var_xn_sub_dn6_slot: &mut f64,
        var_xn_sub_dn7_slot: &mut f64,
        var_xn_sub_dn8_slot: &mut f64,
        var_xn_sub_dn9_slot: &mut f64,
        var_xsddep_slot: &mut f64,
        var_xsddep_dn4_slot: &mut f64,
        var_xsddep_dn6_slot: &mut f64,
        var_xsddep_dn7_slot: &mut f64,
        var_xsddep_dn8_slot: &mut f64,
        var_xsddep_dn9_slot: &mut f64,
        var_xth_1d_slot: &mut f64,
        var_xth_1d_dn4_slot: &mut f64,
        var_xth_1d_dn6_slot: &mut f64,
        var_xth_1d_dn7_slot: &mut f64,
        var_xth_1d_dn8_slot: &mut f64,
        var_xth_1d_dn9_slot: &mut f64,
    ) {
        let mut var_a0_csisq: f64 = *var_a0_csisq_slot;
        let mut var_a0_csisq_dn4: f64 = *var_a0_csisq_dn4_slot;
        let mut var_a0_csisq_dn6: f64 = *var_a0_csisq_dn6_slot;
        let mut var_a0_csisq_dn7: f64 = *var_a0_csisq_dn7_slot;
        let mut var_a0_csisq_dn8: f64 = *var_a0_csisq_dn8_slot;
        let mut var_a0_csisq_dn9: f64 = *var_a0_csisq_dn9_slot;
        let mut var_cf1_i: f64 = *var_cf1_i_slot;
        let mut var_cf1_i_dn4: f64 = *var_cf1_i_dn4_slot;
        let mut var_cf1_i_dn6: f64 = *var_cf1_i_dn6_slot;
        let mut var_cf1_i_dn7: f64 = *var_cf1_i_dn7_slot;
        let mut var_cf1_i_dn8: f64 = *var_cf1_i_dn8_slot;
        let mut var_cf1_i_dn9: f64 = *var_cf1_i_dn9_slot;
        let mut var_cf2_i: f64 = *var_cf2_i_slot;
        let mut var_cf2_i_dn4: f64 = *var_cf2_i_dn4_slot;
        let mut var_cf2_i_dn6: f64 = *var_cf2_i_dn6_slot;
        let mut var_cf2_i_dn7: f64 = *var_cf2_i_dn7_slot;
        let mut var_cf2_i_dn8: f64 = *var_cf2_i_dn8_slot;
        let mut var_cf2_i_dn9: f64 = *var_cf2_i_dn9_slot;
        let mut var_cfac1_i: f64 = *var_cfac1_i_slot;
        let mut var_cfac1_i_dn4: f64 = *var_cfac1_i_dn4_slot;
        let mut var_cfac1_i_dn6: f64 = *var_cfac1_i_dn6_slot;
        let mut var_cfac1_i_dn7: f64 = *var_cfac1_i_dn7_slot;
        let mut var_cfac1_i_dn8: f64 = *var_cfac1_i_dn8_slot;
        let mut var_cfac1_i_dn9: f64 = *var_cfac1_i_dn9_slot;
        let mut var_cfac2_i: f64 = *var_cfac2_i_slot;
        let mut var_cfac2_i_dn4: f64 = *var_cfac2_i_dn4_slot;
        let mut var_cfac2_i_dn6: f64 = *var_cfac2_i_dn6_slot;
        let mut var_cfac2_i_dn7: f64 = *var_cfac2_i_dn7_slot;
        let mut var_cfac2_i_dn8: f64 = *var_cfac2_i_dn8_slot;
        let mut var_cfac2_i_dn9: f64 = *var_cfac2_i_dn9_slot;
        let mut var_cox1init: f64 = *var_cox1init_slot;
        let mut var_cox1prime: f64 = *var_cox1prime_slot;
        let mut var_cox2init: f64 = *var_cox2init_slot;
        let mut var_cox2prime: f64 = *var_cox2prime_slot;
        let mut var_csiprime_0: f64 = *var_csiprime_0_slot;
        let mut var_deg: f64 = *var_deg_slot;
        let mut var_deg_dn4: f64 = *var_deg_dn4_slot;
        let mut var_deg_dn6: f64 = *var_deg_dn6_slot;
        let mut var_deg_dn7: f64 = *var_deg_dn7_slot;
        let mut var_deg_dn8: f64 = *var_deg_dn8_slot;
        let mut var_deg_dn9: f64 = *var_deg_dn9_slot;
        let mut var_dvfb1nch: f64 = *var_dvfb1nch_slot;
        let mut var_dvfb1nch_dn4: f64 = *var_dvfb1nch_dn4_slot;
        let mut var_dvfb1nch_dn6: f64 = *var_dvfb1nch_dn6_slot;
        let mut var_dvfb1nch_dn7: f64 = *var_dvfb1nch_dn7_slot;
        let mut var_dvfb1nch_dn8: f64 = *var_dvfb1nch_dn8_slot;
        let mut var_dvfb1nch_dn9: f64 = *var_dvfb1nch_dn9_slot;
        let mut var_dvfb2nch: f64 = *var_dvfb2nch_slot;
        let mut var_dvfb2nch_dn4: f64 = *var_dvfb2nch_dn4_slot;
        let mut var_dvfb2nch_dn6: f64 = *var_dvfb2nch_dn6_slot;
        let mut var_dvfb2nch_dn7: f64 = *var_dvfb2nch_dn7_slot;
        let mut var_dvfb2nch_dn8: f64 = *var_dvfb2nch_dn8_slot;
        let mut var_dvfb2nch_dn9: f64 = *var_dvfb2nch_dn9_slot;
        let mut var_dvfbch: f64 = *var_dvfbch_slot;
        let mut var_dvfbch_dn4: f64 = *var_dvfbch_dn4_slot;
        let mut var_dvfbch_dn6: f64 = *var_dvfbch_dn6_slot;
        let mut var_dvfbch_dn7: f64 = *var_dvfbch_dn7_slot;
        let mut var_dvfbch_dn8: f64 = *var_dvfbch_dn8_slot;
        let mut var_dvfbch_dn9: f64 = *var_dvfbch_dn9_slot;
        let mut var_dvfbpdep: f64 = *var_dvfbpdep_slot;
        let mut var_dvfbpdep_dn4: f64 = *var_dvfbpdep_dn4_slot;
        let mut var_dvfbpdep_dn6: f64 = *var_dvfbpdep_dn6_slot;
        let mut var_dvfbpdep_dn7: f64 = *var_dvfbpdep_dn7_slot;
        let mut var_dvfbpdep_dn8: f64 = *var_dvfbpdep_dn8_slot;
        let mut var_dvfbpdep_dn9: f64 = *var_dvfbpdep_dn9_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_2phit: f64 = *var_eg_2phit_slot;
        let mut var_eg_2phit0: f64 = *var_eg_2phit0_slot;
        let mut var_eg_2phit0_dn4: f64 = *var_eg_2phit0_dn4_slot;
        let mut var_eg_2phit0_dn6: f64 = *var_eg_2phit0_dn6_slot;
        let mut var_eg_2phit0_dn7: f64 = *var_eg_2phit0_dn7_slot;
        let mut var_eg_2phit0_dn8: f64 = *var_eg_2phit0_dn8_slot;
        let mut var_eg_2phit0_dn9: f64 = *var_eg_2phit0_dn9_slot;
        let mut var_eg_2phit0_woshe: f64 = *var_eg_2phit0_woshe_slot;
        let mut var_eg_2phit0_woshe_dn4: f64 = *var_eg_2phit0_woshe_dn4_slot;
        let mut var_eg_2phit0_woshe_dn6: f64 = *var_eg_2phit0_woshe_dn6_slot;
        let mut var_eg_2phit0_woshe_dn7: f64 = *var_eg_2phit0_woshe_dn7_slot;
        let mut var_eg_2phit0_woshe_dn8: f64 = *var_eg_2phit0_woshe_dn8_slot;
        let mut var_eg_2phit0_woshe_dn9: f64 = *var_eg_2phit0_woshe_dn9_slot;
        let mut var_eg_2phit_dn4: f64 = *var_eg_2phit_dn4_slot;
        let mut var_eg_2phit_dn6: f64 = *var_eg_2phit_dn6_slot;
        let mut var_eg_2phit_dn7: f64 = *var_eg_2phit_dn7_slot;
        let mut var_eg_2phit_dn8: f64 = *var_eg_2phit_dn8_slot;
        let mut var_eg_2phit_dn9: f64 = *var_eg_2phit_dn9_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_dn8: f64 = *var_eg_dn8_slot;
        let mut var_eg_dn9: f64 = *var_eg_dn9_slot;
        let mut var_emin: f64 = *var_emin_slot;
        let mut var_emin_dn4: f64 = *var_emin_dn4_slot;
        let mut var_emin_dn6: f64 = *var_emin_dn6_slot;
        let mut var_emin_dn7: f64 = *var_emin_dn7_slot;
        let mut var_emin_dn8: f64 = *var_emin_dn8_slot;
        let mut var_emin_dn9: f64 = *var_emin_dn9_slot;
        let mut var_gfsub: f64 = *var_gfsub_slot;
        let mut var_gfsub2: f64 = *var_gfsub2_slot;
        let mut var_gfsub2_dn4: f64 = *var_gfsub2_dn4_slot;
        let mut var_gfsub2_dn6: f64 = *var_gfsub2_dn6_slot;
        let mut var_gfsub2_dn7: f64 = *var_gfsub2_dn7_slot;
        let mut var_gfsub2_dn8: f64 = *var_gfsub2_dn8_slot;
        let mut var_gfsub2_dn9: f64 = *var_gfsub2_dn9_slot;
        let mut var_gfsub_dn4: f64 = *var_gfsub_dn4_slot;
        let mut var_gfsub_dn6: f64 = *var_gfsub_dn6_slot;
        let mut var_gfsub_dn7: f64 = *var_gfsub_dn7_slot;
        let mut var_gfsub_dn8: f64 = *var_gfsub_dn8_slot;
        let mut var_gfsub_dn9: f64 = *var_gfsub_dn9_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard142: f64 = *var_guard142_slot;
        let mut var_guard143: f64 = *var_guard143_slot;
        let mut var_inv_gfsub2: f64 = *var_inv_gfsub2_slot;
        let mut var_inv_gfsub2_dn4: f64 = *var_inv_gfsub2_dn4_slot;
        let mut var_inv_gfsub2_dn6: f64 = *var_inv_gfsub2_dn6_slot;
        let mut var_inv_gfsub2_dn7: f64 = *var_inv_gfsub2_dn7_slot;
        let mut var_inv_gfsub2_dn8: f64 = *var_inv_gfsub2_dn8_slot;
        let mut var_inv_gfsub2_dn9: f64 = *var_inv_gfsub2_dn9_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_dn4: f64 = *var_inv_phit_dn4_slot;
        let mut var_inv_phit_dn6: f64 = *var_inv_phit_dn6_slot;
        let mut var_inv_phit_dn7: f64 = *var_inv_phit_dn7_slot;
        let mut var_inv_phit_dn8: f64 = *var_inv_phit_dn8_slot;
        let mut var_inv_phit_dn9: f64 = *var_inv_phit_dn9_slot;
        let mut var_inv_xisub: f64 = *var_inv_xisub_slot;
        let mut var_inv_xisub_dn4: f64 = *var_inv_xisub_dn4_slot;
        let mut var_inv_xisub_dn6: f64 = *var_inv_xisub_dn6_slot;
        let mut var_inv_xisub_dn7: f64 = *var_inv_xisub_dn7_slot;
        let mut var_inv_xisub_dn8: f64 = *var_inv_xisub_dn8_slot;
        let mut var_inv_xisub_dn9: f64 = *var_inv_xisub_dn9_slot;
        let mut var_k1_1d: f64 = *var_k1_1d_slot;
        let mut var_k2_1d: f64 = *var_k2_1d_slot;
        let mut var_keq_1d: f64 = *var_keq_1d_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_dn4: f64 = *var_kp_dn4_slot;
        let mut var_kp_dn6: f64 = *var_kp_dn6_slot;
        let mut var_kp_dn7: f64 = *var_kp_dn7_slot;
        let mut var_kp_dn8: f64 = *var_kp_dn8_slot;
        let mut var_kp_dn9: f64 = *var_kp_dn9_slot;
        let mut var_margin_sub: f64 = *var_margin_sub_slot;
        let mut var_neff: f64 = *var_neff_slot;
        let mut var_neff_dn4: f64 = *var_neff_dn4_slot;
        let mut var_neff_dn6: f64 = *var_neff_dn6_slot;
        let mut var_neff_dn7: f64 = *var_neff_dn7_slot;
        let mut var_neff_dn8: f64 = *var_neff_dn8_slot;
        let mut var_neff_dn9: f64 = *var_neff_dn9_slot;
        let mut var_neff_poly: f64 = *var_neff_poly_slot;
        let mut var_neff_poly_dn4: f64 = *var_neff_poly_dn4_slot;
        let mut var_neff_poly_dn6: f64 = *var_neff_poly_dn6_slot;
        let mut var_neff_poly_dn7: f64 = *var_neff_poly_dn7_slot;
        let mut var_neff_poly_dn8: f64 = *var_neff_poly_dn8_slot;
        let mut var_neff_poly_dn9: f64 = *var_neff_poly_dn9_slot;
        let mut var_neff_sub: f64 = *var_neff_sub_slot;
        let mut var_neff_sub_dn4: f64 = *var_neff_sub_dn4_slot;
        let mut var_neff_sub_dn6: f64 = *var_neff_sub_dn6_slot;
        let mut var_neff_sub_dn7: f64 = *var_neff_sub_dn7_slot;
        let mut var_neff_sub_dn8: f64 = *var_neff_sub_dn8_slot;
        let mut var_neff_sub_dn9: f64 = *var_neff_sub_dn9_slot;
        let mut var_niratio: f64 = *var_niratio_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_dn4: f64 = *var_phit_dn4_slot;
        let mut var_phit_dn6: f64 = *var_phit_dn6_slot;
        let mut var_phit_dn7: f64 = *var_phit_dn7_slot;
        let mut var_phit_dn8: f64 = *var_phit_dn8_slot;
        let mut var_phit_dn9: f64 = *var_phit_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;
        let mut var_xb_sub: f64 = *var_xb_sub_slot;
        let mut var_xb_sub_dn4: f64 = *var_xb_sub_dn4_slot;
        let mut var_xb_sub_dn6: f64 = *var_xb_sub_dn6_slot;
        let mut var_xb_sub_dn7: f64 = *var_xb_sub_dn7_slot;
        let mut var_xb_sub_dn8: f64 = *var_xb_sub_dn8_slot;
        let mut var_xb_sub_dn9: f64 = *var_xb_sub_dn9_slot;
        let mut var_xd0: f64 = *var_xd0_slot;
        let mut var_xd0_dn4: f64 = *var_xd0_dn4_slot;
        let mut var_xd0_dn6: f64 = *var_xd0_dn6_slot;
        let mut var_xd0_dn7: f64 = *var_xd0_dn7_slot;
        let mut var_xd0_dn8: f64 = *var_xd0_dn8_slot;
        let mut var_xd0_dn9: f64 = *var_xd0_dn9_slot;
        let mut var_xisub: f64 = *var_xisub_slot;
        let mut var_xisub_dn4: f64 = *var_xisub_dn4_slot;
        let mut var_xisub_dn6: f64 = *var_xisub_dn6_slot;
        let mut var_xisub_dn7: f64 = *var_xisub_dn7_slot;
        let mut var_xisub_dn8: f64 = *var_xisub_dn8_slot;
        let mut var_xisub_dn9: f64 = *var_xisub_dn9_slot;
        let mut var_xn_sub: f64 = *var_xn_sub_slot;
        let mut var_xn_sub_dn4: f64 = *var_xn_sub_dn4_slot;
        let mut var_xn_sub_dn6: f64 = *var_xn_sub_dn6_slot;
        let mut var_xn_sub_dn7: f64 = *var_xn_sub_dn7_slot;
        let mut var_xn_sub_dn8: f64 = *var_xn_sub_dn8_slot;
        let mut var_xn_sub_dn9: f64 = *var_xn_sub_dn9_slot;
        let mut var_xsddep: f64 = *var_xsddep_slot;
        let mut var_xsddep_dn4: f64 = *var_xsddep_dn4_slot;
        let mut var_xsddep_dn6: f64 = *var_xsddep_dn6_slot;
        let mut var_xsddep_dn7: f64 = *var_xsddep_dn7_slot;
        let mut var_xsddep_dn8: f64 = *var_xsddep_dn8_slot;
        let mut var_xsddep_dn9: f64 = *var_xsddep_dn9_slot;
        let mut var_xth_1d: f64 = *var_xth_1d_slot;
        let mut var_xth_1d_dn4: f64 = *var_xth_1d_dn4_slot;
        let mut var_xth_1d_dn6: f64 = *var_xth_1d_dn6_slot;
        let mut var_xth_1d_dn7: f64 = *var_xth_1d_dn7_slot;
        let mut var_xth_1d_dn8: f64 = *var_xth_1d_dn8_slot;
        let mut var_xth_1d_dn9: f64 = *var_xth_1d_dn9_slot;

        let assign6660_e6613: f64 = (var_egge - var_egsi);
        let assign6660_e6615: f64 = (-0.4);
        let assign6660_e6617: f64 = (assign6660_e6615 * var_one_m_xge);
        let assign6660_e6618: f64 = (assign6660_e6613 + assign6660_e6617);
        let assign6660_e6620: f64 = (assign6660_e6618 * var_xge_i);
        var_deg = assign6660_e6620;
        var_deg_dn4 = ((var_egge_dn4 - var_egsi_dn4) * var_xge_i);
        var_deg_dn6 = ((var_egge_dn6 - var_egsi_dn6) * var_xge_i);
        var_deg_dn7 = ((var_egge_dn7 - var_egsi_dn7) * var_xge_i);
        var_deg_dn8 = ((var_egge_dn8 - var_egsi_dn8) * var_xge_i);
        var_deg_dn9 = ((var_egge_dn9 - var_egsi_dn9) * var_xge_i);

        let assign6670_e6623: f64 = (var_egsi + var_deg);
        var_eg = assign6670_e6623;
        var_eg_dn4 = (var_egsi_dn4 + var_deg_dn4);
        var_eg_dn6 = (var_egsi_dn6 + var_deg_dn6);
        var_eg_dn7 = (var_egsi_dn7 + var_deg_dn7);
        var_eg_dn8 = (var_egsi_dn8 + var_deg_dn8);
        var_eg_dn9 = (var_egsi_dn9 + var_deg_dn9);

        let assign6680_e6626: f64 = (0.5 * var_eg);
        let assign6680_e6628: f64 = (assign6680_e6626 * var_inv_phit0);
        var_eg_2phit0 = assign6680_e6628;
        var_eg_2phit0_dn4 = (((0.5 * var_eg_dn4) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn4));
        var_eg_2phit0_dn6 = (((0.5 * var_eg_dn6) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn6));
        var_eg_2phit0_dn7 = (((0.5 * var_eg_dn7) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn7));
        var_eg_2phit0_dn8 = (((0.5 * var_eg_dn8) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn8));
        var_eg_2phit0_dn9 = (((0.5 * var_eg_dn9) * var_inv_phit0) + (assign6680_e6626 * var_inv_phit0_dn9));

        var_eg_2phit0_woshe = var_eg_2phit0;
        var_eg_2phit0_woshe_dn4 = var_eg_2phit0_dn4;
        var_eg_2phit0_woshe_dn6 = var_eg_2phit0_dn6;
        var_eg_2phit0_woshe_dn7 = var_eg_2phit0_dn7;
        var_eg_2phit0_woshe_dn8 = var_eg_2phit0_dn8;
        var_eg_2phit0_woshe_dn9 = var_eg_2phit0_dn9;

        let assign6700_e6634: f64 = (10.0 * var_xge_i);
        let assign6700_e6635: f64 = (assign6700_e6634).sqrt();
        let assign6700_e6636: f64 = (1.0 + assign6700_e6635);
        let assign6700_e6637: f64 = (1.0 / assign6700_e6636);
        var_niratio = assign6700_e6637;

        let assign6710_e6640: f64 = (0.05 * var_xge_i);
        let assign6710_e6643: f64 = (0.5 * var_deg);
        let assign6710_e6644: f64 = (assign6710_e6640 - assign6710_e6643);
        var_dvfbch = assign6710_e6644;
        var_dvfbch_dn4 = (-(0.5 * var_deg_dn4));
        var_dvfbch_dn6 = (-(0.5 * var_deg_dn6));
        var_dvfbch_dn7 = (-(0.5 * var_deg_dn7));
        var_dvfbch_dn8 = (-(0.5 * var_deg_dn8));
        var_dvfbch_dn9 = (-(0.5 * var_deg_dn9));

        let assign6720_e6647: f64 = (1.602176565e-19 * var_nch_i);
        let assign6720_e6649: f64 = (assign6720_e6647 * 0.5);
        let assign6720_e6651: f64 = (assign6720_e6649 * var_tsi_i);
        let assign6720_e6653: f64 = (assign6720_e6651 / 3.45313e-11);
        var_temp = assign6720_e6653;
        var_temp_dn4 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;

        let assign6730_e6656: f64 = if var_typech_i > 0.0 { 1.0 } else { 0.0 };
        var_guard140 = assign6730_e6656;

        let (assign6740_e6666, assign6740_e6666_d_n4, assign6740_e6666_d_n6, assign6740_e6666_d_n7, assign6740_e6666_d_n8, assign6740_e6666_d_n9,) = {
    if (var_guard140 != 0.0) {
        let assign6740_e6662: f64 = (p.p13 * 4e-10);
        let assign6740_e6663: f64 = (var_tox1_i + assign6740_e6662);
        let assign6740_e6664: f64 = (var_temp * assign6740_e6663);
        (assign6740_e6664, (var_temp_dn4 * assign6740_e6663), (var_temp_dn6 * assign6740_e6663), (var_temp_dn7 * assign6740_e6663), (var_temp_dn8 * assign6740_e6663), (var_temp_dn9 * assign6740_e6663),)
    } else {
        (var_dvfb1nch, var_dvfb1nch_dn4, var_dvfb1nch_dn6, var_dvfb1nch_dn7, var_dvfb1nch_dn8, var_dvfb1nch_dn9,)
    }
};
        var_dvfb1nch = assign6740_e6666;
        var_dvfb1nch_dn4 = assign6740_e6666_d_n4;
        var_dvfb1nch_dn6 = assign6740_e6666_d_n6;
        var_dvfb1nch_dn7 = assign6740_e6666_d_n7;
        var_dvfb1nch_dn8 = assign6740_e6666_d_n8;
        var_dvfb1nch_dn9 = assign6740_e6666_d_n9;

        let (assign6750_e6676, assign6750_e6676_d_n4, assign6750_e6676_d_n6, assign6750_e6676_d_n7, assign6750_e6676_d_n8, assign6750_e6676_d_n9,) = {
    if (var_guard140 != 0.0) {
        let assign6750_e6672: f64 = (p.p13 * 4e-10);
        let assign6750_e6673: f64 = (var_tox2_i + assign6750_e6672);
        let assign6750_e6674: f64 = (var_temp * assign6750_e6673);
        (assign6750_e6674, (var_temp_dn4 * assign6750_e6673), (var_temp_dn6 * assign6750_e6673), (var_temp_dn7 * assign6750_e6673), (var_temp_dn8 * assign6750_e6673), (var_temp_dn9 * assign6750_e6673),)
    } else {
        (var_dvfb2nch, var_dvfb2nch_dn4, var_dvfb2nch_dn6, var_dvfb2nch_dn7, var_dvfb2nch_dn8, var_dvfb2nch_dn9,)
    }
};
        var_dvfb2nch = assign6750_e6676;
        var_dvfb2nch_dn4 = assign6750_e6676_d_n4;
        var_dvfb2nch_dn6 = assign6750_e6676_d_n6;
        var_dvfb2nch_dn7 = assign6750_e6676_d_n7;
        var_dvfb2nch_dn8 = assign6750_e6676_d_n8;
        var_dvfb2nch_dn9 = assign6750_e6676_d_n9;

        let (assign6760_e6688, assign6760_e6688_d_n4, assign6760_e6688_d_n6, assign6760_e6688_d_n7, assign6760_e6688_d_n8, assign6760_e6688_d_n9,) = {
    if (var_guard140 == 0.0) {
        let assign6760_e6680: f64 = (-var_temp);
        let assign6760_e6684: f64 = (p.p13 * 4e-10);
        let assign6760_e6685: f64 = (var_tox1_i + assign6760_e6684);
        let assign6760_e6686: f64 = (assign6760_e6680 * assign6760_e6685);
        (assign6760_e6686, ((-var_temp_dn4) * assign6760_e6685), ((-var_temp_dn6) * assign6760_e6685), ((-var_temp_dn7) * assign6760_e6685), ((-var_temp_dn8) * assign6760_e6685), ((-var_temp_dn9) * assign6760_e6685),)
    } else {
        (var_dvfb1nch, var_dvfb1nch_dn4, var_dvfb1nch_dn6, var_dvfb1nch_dn7, var_dvfb1nch_dn8, var_dvfb1nch_dn9,)
    }
};
        var_dvfb1nch = assign6760_e6688;
        var_dvfb1nch_dn4 = assign6760_e6688_d_n4;
        var_dvfb1nch_dn6 = assign6760_e6688_d_n6;
        var_dvfb1nch_dn7 = assign6760_e6688_d_n7;
        var_dvfb1nch_dn8 = assign6760_e6688_d_n8;
        var_dvfb1nch_dn9 = assign6760_e6688_d_n9;

        let (assign6770_e6700, assign6770_e6700_d_n4, assign6770_e6700_d_n6, assign6770_e6700_d_n7, assign6770_e6700_d_n8, assign6770_e6700_d_n9,) = {
    if (var_guard140 == 0.0) {
        let assign6770_e6692: f64 = (-var_temp);
        let assign6770_e6696: f64 = (p.p13 * 4e-10);
        let assign6770_e6697: f64 = (var_tox2_i + assign6770_e6696);
        let assign6770_e6698: f64 = (assign6770_e6692 * assign6770_e6697);
        (assign6770_e6698, ((-var_temp_dn4) * assign6770_e6697), ((-var_temp_dn6) * assign6770_e6697), ((-var_temp_dn7) * assign6770_e6697), ((-var_temp_dn8) * assign6770_e6697), ((-var_temp_dn9) * assign6770_e6697),)
    } else {
        (var_dvfb2nch, var_dvfb2nch_dn4, var_dvfb2nch_dn6, var_dvfb2nch_dn7, var_dvfb2nch_dn8, var_dvfb2nch_dn9,)
    }
};
        var_dvfb2nch = assign6770_e6700;
        var_dvfb2nch_dn4 = assign6770_e6700_d_n4;
        var_dvfb2nch_dn6 = assign6770_e6700_d_n6;
        var_dvfb2nch_dn7 = assign6770_e6700_d_n7;
        var_dvfb2nch_dn8 = assign6770_e6700_d_n8;
        var_dvfb2nch_dn9 = assign6770_e6700_d_n9;

        let assign6780_e6703: f64 = (var_tkc * 0.0033333333333);
        let assign6780_e6704: f64 = (assign6780_e6703).sqrt();
        var_temp = assign6780_e6704;
        var_temp_dn4 = ((var_tkc_dn4 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn6 = ((var_tkc_dn6 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn7 = ((var_tkc_dn7 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn8 = ((var_tkc_dn8 * 0.0033333333333) / (2.0 * assign6780_e6704));
        var_temp_dn9 = ((var_tkc_dn9 * 0.0033333333333) / (2.0 * assign6780_e6704));

        let assign6790_e6707: f64 = (4.05e25 * var_temp);
        let assign6790_e6709: f64 = (assign6790_e6707 * var_temp);
        let assign6790_e6711: f64 = (assign6790_e6709 * var_temp);
        var_temp1 = assign6790_e6711;
        var_temp1_dn4 = (((((4.05e25 * var_temp_dn4) * var_temp) + (assign6790_e6707 * var_temp_dn4)) * var_temp) + (assign6790_e6709 * var_temp_dn4));
        var_temp1_dn6 = (((((4.05e25 * var_temp_dn6) * var_temp) + (assign6790_e6707 * var_temp_dn6)) * var_temp) + (assign6790_e6709 * var_temp_dn6));
        var_temp1_dn7 = (((((4.05e25 * var_temp_dn7) * var_temp) + (assign6790_e6707 * var_temp_dn7)) * var_temp) + (assign6790_e6709 * var_temp_dn7));
        var_temp1_dn8 = (((((4.05e25 * var_temp_dn8) * var_temp) + (assign6790_e6707 * var_temp_dn8)) * var_temp) + (assign6790_e6709 * var_temp_dn8));
        var_temp1_dn9 = (((((4.05e25 * var_temp_dn9) * var_temp) + (assign6790_e6707 * var_temp_dn9)) * var_temp) + (assign6790_e6709 * var_temp_dn9));

        let assign6800_e6714: f64 = (var_temp1 * var_niratio);
        var_neff = assign6800_e6714;
        var_neff_dn4 = (var_temp1_dn4 * var_niratio);
        var_neff_dn6 = (var_temp1_dn6 * var_niratio);
        var_neff_dn7 = (var_temp1_dn7 * var_niratio);
        var_neff_dn8 = (var_temp1_dn8 * var_niratio);
        var_neff_dn9 = (var_temp1_dn9 * var_niratio);

        let assign6810_e6718: f64 = (0.5 * var_deg);
        let assign6810_e6720: f64 = (assign6810_e6718 * var_inv_phit0);
        let assign6810_e6721: f64 = (assign6810_e6720).exp();
        let assign6810_e6722: f64 = (var_temp1 * assign6810_e6721);
        var_neff_poly = assign6810_e6722;
        var_neff_poly_dn4 = ((var_temp1_dn4 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn4) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn4)))));
        var_neff_poly_dn6 = ((var_temp1_dn6 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn6) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn6)))));
        var_neff_poly_dn7 = ((var_temp1_dn7 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn7) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn7)))));
        var_neff_poly_dn8 = ((var_temp1_dn8 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn8) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn8)))));
        var_neff_poly_dn9 = ((var_temp1_dn9 * assign6810_e6721) + (var_temp1 * (assign6810_e6721 * (((0.5 * var_deg_dn9) * var_inv_phit0) + (assign6810_e6718 * var_inv_phit0_dn9)))));

        let assign6820_e6726: f64 = (0.5 * var_deg);
        let assign6820_e6728: f64 = (assign6820_e6726 * var_inv_phit0);
        let assign6820_e6729: f64 = (assign6820_e6728).exp();
        let assign6820_e6730: f64 = (var_temp1 * assign6820_e6729);
        var_neff_sub = assign6820_e6730;
        var_neff_sub_dn4 = ((var_temp1_dn4 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn4) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn4)))));
        var_neff_sub_dn6 = ((var_temp1_dn6 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn6) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn6)))));
        var_neff_sub_dn7 = ((var_temp1_dn7 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn7) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn7)))));
        var_neff_sub_dn8 = ((var_temp1_dn8 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn8) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn8)))));
        var_neff_sub_dn9 = ((var_temp1_dn9 * assign6820_e6729) + (var_temp1 * (assign6820_e6729 * (((0.5 * var_deg_dn9) * var_inv_phit0) + (assign6820_e6726 * var_inv_phit0_dn9)))));

        let assign6830_e6733: f64 = (3.45313e-11 / var_tox1_i);
        var_cox1init = assign6830_e6733;

        let assign6840_e6736: f64 = (3.45313e-11 / var_tox2_i);
        var_cox2init = assign6840_e6736;

        let assign6850_e6739: f64 = if var_pnce_i > 0.0 { 1.0 } else { 0.0 };
        var_guard141 = assign6850_e6739;

        let (assign6860_e6747,) = {
    if (var_guard141 != 0.0) {
        let assign6860_e6744: f64 = (1.0 + var_pnce_i);
        let assign6860_e6745: f64 = (var_cox1init * assign6860_e6744);
        (assign6860_e6745,)
    } else {
        (var_cox1prime,)
    }
};
        var_cox1prime = assign6860_e6747;

        let (assign6870_e6751,) = {
    if (var_guard141 != 0.0) {
        (var_cox2init,)
    } else {
        (var_cox2prime,)
    }
};
        var_cox2prime = assign6870_e6751;

        let (assign6880_e6756,) = {
    if (var_guard141 == 0.0) {
        (var_cox1init,)
    } else {
        (var_cox1prime,)
    }
};
        var_cox1prime = assign6880_e6756;

        let (assign6890_e6765,) = {
    if (var_guard141 == 0.0) {
        let assign6890_e6762: f64 = (1.0 - var_pnce_i);
        let assign6890_e6763: f64 = (var_cox2init * assign6890_e6762);
        (assign6890_e6763,)
    } else {
        (var_cox2prime,)
    }
};
        var_cox2prime = assign6890_e6765;

        let assign6900_e6768: f64 = (var_epsch / var_tsi_i);
        var_csiprime_0 = assign6900_e6768;

        let assign6910_e6773: f64 = (var_ct_i * var_rtn);
        let assign6910_e6774: f64 = (1.0 + assign6910_e6773);
        let assign6910_e6775: f64 = (var_phit0 * assign6910_e6774);
        var_phit = assign6910_e6775;
        var_phit_dn4 = ((var_phit0_dn4 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn4)));
        var_phit_dn6 = ((var_phit0_dn6 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn6)));
        var_phit_dn7 = ((var_phit0_dn7 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn7)));
        var_phit_dn8 = ((var_phit0_dn8 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn8)));
        var_phit_dn9 = ((var_phit0_dn9 * assign6910_e6774) + (var_phit0 * (var_ct_i * var_rtn_dn9)));

        let assign6920_e6778: f64 = (1.0 / var_phit);
        var_inv_phit = assign6920_e6778;
        var_inv_phit_dn4 = (-(var_phit_dn4 / (var_phit * var_phit)));
        var_inv_phit_dn6 = (-(var_phit_dn6 / (var_phit * var_phit)));
        var_inv_phit_dn7 = (-(var_phit_dn7 / (var_phit * var_phit)));
        var_inv_phit_dn8 = (-(var_phit_dn8 / (var_phit * var_phit)));
        var_inv_phit_dn9 = (-(var_phit_dn9 / (var_phit * var_phit)));

        let assign6930_e6781: f64 = (0.5 * var_eg);
        let assign6930_e6783: f64 = (assign6930_e6781 * var_inv_phit);
        var_eg_2phit = assign6930_e6783;
        var_eg_2phit_dn4 = (((0.5 * var_eg_dn4) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn4));
        var_eg_2phit_dn6 = (((0.5 * var_eg_dn6) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn6));
        var_eg_2phit_dn7 = (((0.5 * var_eg_dn7) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn7));
        var_eg_2phit_dn8 = (((0.5 * var_eg_dn8) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn8));
        var_eg_2phit_dn9 = (((0.5 * var_eg_dn9) * var_inv_phit) + (assign6930_e6781 * var_inv_phit_dn9));

        let assign6940_e6786: f64 = (var_cox1prime / var_csiprime_0);
        var_k1_1d = assign6940_e6786;

        let assign6950_e6789: f64 = (var_cox2prime / var_csiprime_0);
        var_k2_1d = assign6950_e6789;

        let assign6960_e6794: f64 = (1.0 / var_k1_1d);
        let assign6960_e6795: f64 = (1.0 + assign6960_e6794);
        let assign6960_e6798: f64 = (1.0 / var_k2_1d);
        let assign6960_e6799: f64 = (assign6960_e6795 + assign6960_e6798);
        let assign6960_e6800: f64 = (1.0 / assign6960_e6799);
        var_keq_1d = assign6960_e6800;

        let assign6970_e6803: f64 = (2.0 * 1.602176565e-19);
        let assign6970_e6805: f64 = (assign6970_e6803 * var_neff);
        let assign6970_e6807: f64 = (assign6970_e6805 * var_epsch);
        let assign6970_e6809: f64 = (assign6970_e6807 * var_inv_phit);
        var_a0_csisq = assign6970_e6809;
        var_a0_csisq_dn4 = ((((assign6970_e6803 * var_neff_dn4) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn4));
        var_a0_csisq_dn6 = ((((assign6970_e6803 * var_neff_dn6) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn6));
        var_a0_csisq_dn7 = ((((assign6970_e6803 * var_neff_dn7) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn7));
        var_a0_csisq_dn8 = ((((assign6970_e6803 * var_neff_dn8) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn8));
        var_a0_csisq_dn9 = ((((assign6970_e6803 * var_neff_dn9) * var_epsch) * var_inv_phit) + (assign6970_e6807 * var_inv_phit_dn9));

        let assign6980_e6812: f64 = (var_csiprime_0 * var_csiprime_0);
        let assign6980_e6814: f64 = (assign6980_e6812 / var_a0_csisq);
        let assign6980_e6815: f64 = (assign6980_e6814).ln();
        let assign6980_e6817: f64 = (assign6980_e6815 - 0.6931471805599);
        var_xth_1d = assign6980_e6817;
        var_xth_1d_dn4 = ((-((assign6980_e6812 * var_a0_csisq_dn4) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn6 = ((-((assign6980_e6812 * var_a0_csisq_dn6) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn7 = ((-((assign6980_e6812 * var_a0_csisq_dn7) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn8 = ((-((assign6980_e6812 * var_a0_csisq_dn8) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);
        var_xth_1d_dn9 = ((-((assign6980_e6812 * var_a0_csisq_dn9) / (var_a0_csisq * var_a0_csisq))) / assign6980_e6814);

        let assign6990_e6820: f64 = (0.5 * 1.602176565e-19);
        let assign6990_e6822: f64 = (assign6990_e6820 * var_nsddc_i);
        let assign6990_e6824: f64 = (assign6990_e6822 * var_tsi_i);
        let assign6990_e6827: f64 = (var_cox1prime + var_cox2prime);
        let assign6990_e6828: f64 = (assign6990_e6824 / assign6990_e6827);
        let assign6990_e6830: f64 = (assign6990_e6828 * var_inv_phit);
        var_xsddep = assign6990_e6830;
        var_xsddep_dn4 = (assign6990_e6828 * var_inv_phit_dn4);
        var_xsddep_dn6 = (assign6990_e6828 * var_inv_phit_dn6);
        var_xsddep_dn7 = (assign6990_e6828 * var_inv_phit_dn7);
        var_xsddep_dn8 = (assign6990_e6828 * var_inv_phit_dn8);
        var_xsddep_dn9 = (assign6990_e6828 * var_inv_phit_dn9);

        let assign7000_e6833: f64 = (var_stcf_i * var_dt);
        var_temp = assign7000_e6833;
        var_temp_dn4 = ((var_stcf_i_dn4 * var_dt) + (var_stcf_i * var_dt_dn4));
        var_temp_dn6 = ((var_stcf_i_dn6 * var_dt) + (var_stcf_i * var_dt_dn6));
        var_temp_dn7 = ((var_stcf_i_dn7 * var_dt) + (var_stcf_i * var_dt_dn7));
        var_temp_dn8 = ((var_stcf_i_dn8 * var_dt) + (var_stcf_i * var_dt_dn8));
        var_temp_dn9 = ((var_stcf_i_dn9 * var_dt) + (var_stcf_i * var_dt_dn9));

        let assign7010_e6836: f64 = (var_cf1_t + var_temp);
        var_cf1_i = assign7010_e6836;
        var_cf1_i_dn4 = (var_cf1_t_dn4 + var_temp_dn4);
        var_cf1_i_dn6 = (var_cf1_t_dn6 + var_temp_dn6);
        var_cf1_i_dn7 = (var_cf1_t_dn7 + var_temp_dn7);
        var_cf1_i_dn8 = (var_cf1_t_dn8 + var_temp_dn8);
        var_cf1_i_dn9 = (var_cf1_t_dn9 + var_temp_dn9);

        let assign7020_e6839: f64 = (var_cf2_t + var_temp);
        var_cf2_i = assign7020_e6839;
        var_cf2_i_dn4 = (var_cf2_t_dn4 + var_temp_dn4);
        var_cf2_i_dn6 = (var_cf2_t_dn6 + var_temp_dn6);
        var_cf2_i_dn7 = (var_cf2_t_dn7 + var_temp_dn7);
        var_cf2_i_dn8 = (var_cf2_t_dn8 + var_temp_dn8);
        var_cf2_i_dn9 = (var_cf2_t_dn9 + var_temp_dn9);

        let assign7030_e6842: f64 = (var_cfac1_t + var_temp);
        var_cfac1_i = assign7030_e6842;
        var_cfac1_i_dn4 = (var_cfac1_t_dn4 + var_temp_dn4);
        var_cfac1_i_dn6 = (var_cfac1_t_dn6 + var_temp_dn6);
        var_cfac1_i_dn7 = (var_cfac1_t_dn7 + var_temp_dn7);
        var_cfac1_i_dn8 = (var_cfac1_t_dn8 + var_temp_dn8);
        var_cfac1_i_dn9 = (var_cfac1_t_dn9 + var_temp_dn9);

        let assign7040_e6845: f64 = (var_cfac2_t + var_temp);
        var_cfac2_i = assign7040_e6845;
        var_cfac2_i_dn4 = (var_cfac2_t_dn4 + var_temp_dn4);
        var_cfac2_i_dn6 = (var_cfac2_t_dn6 + var_temp_dn6);
        var_cfac2_i_dn7 = (var_cfac2_t_dn7 + var_temp_dn7);
        var_cfac2_i_dn8 = (var_cfac2_t_dn8 + var_temp_dn8);
        var_cfac2_i_dn9 = (var_cfac2_t_dn9 + var_temp_dn9);

        let assign7050_e6848: f64 = (var_cfd_i * var_inv_phit);
        var_xd0 = assign7050_e6848;
        var_xd0_dn4 = (var_cfd_i * var_inv_phit_dn4);
        var_xd0_dn6 = (var_cfd_i * var_inv_phit_dn6);
        var_xd0_dn7 = (var_cfd_i * var_inv_phit_dn7);
        var_xd0_dn8 = (var_cfd_i * var_inv_phit_dn8);
        var_xd0_dn9 = (var_cfd_i * var_inv_phit_dn9);

        let assign7060_e6851: f64 = (2.0 * 1.602176565e-19);
        let assign7060_e6853: f64 = (assign7060_e6851 * var_nsub_i);
        let assign7060_e6855: f64 = (assign7060_e6853 * 1.04479e-10);
        let assign7060_e6857: f64 = (assign7060_e6855 * var_inv_phit0);
        let assign7060_e6858: f64 = (assign7060_e6857).sqrt();
        let assign7060_e6860: f64 = (assign7060_e6858 / var_cox2prime);
        var_gfsub = assign7060_e6860;
        var_gfsub_dn4 = (((assign7060_e6855 * var_inv_phit0_dn4) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn6 = (((assign7060_e6855 * var_inv_phit0_dn6) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn7 = (((assign7060_e6855 * var_inv_phit0_dn7) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn8 = (((assign7060_e6855 * var_inv_phit0_dn8) / (2.0 * assign7060_e6858)) / var_cox2prime);
        var_gfsub_dn9 = (((assign7060_e6855 * var_inv_phit0_dn9) / (2.0 * assign7060_e6858)) / var_cox2prime);

        let assign7070_e6863: f64 = (var_gfsub * var_gfsub);
        var_gfsub2 = assign7070_e6863;
        var_gfsub2_dn4 = ((var_gfsub_dn4 * var_gfsub) + (var_gfsub * var_gfsub_dn4));
        var_gfsub2_dn6 = ((var_gfsub_dn6 * var_gfsub) + (var_gfsub * var_gfsub_dn6));
        var_gfsub2_dn7 = ((var_gfsub_dn7 * var_gfsub) + (var_gfsub * var_gfsub_dn7));
        var_gfsub2_dn8 = ((var_gfsub_dn8 * var_gfsub) + (var_gfsub * var_gfsub_dn8));
        var_gfsub2_dn9 = ((var_gfsub_dn9 * var_gfsub) + (var_gfsub * var_gfsub_dn9));

        let assign7080_e6866: f64 = (1.0 / var_gfsub2);
        var_inv_gfsub2 = assign7080_e6866;
        var_inv_gfsub2_dn4 = (-(var_gfsub2_dn4 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn6 = (-(var_gfsub2_dn6 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn7 = (-(var_gfsub2_dn7 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn8 = (-(var_gfsub2_dn8 / (var_gfsub2 * var_gfsub2)));
        var_inv_gfsub2_dn9 = (-(var_gfsub2_dn9 / (var_gfsub2 * var_gfsub2)));

        let assign7090_e6870: f64 = (var_gfsub / 1.4142135623731);
        let assign7090_e6871: f64 = (1.0 + assign7090_e6870);
        var_xisub = assign7090_e6871;
        var_xisub_dn4 = (var_gfsub_dn4 / 1.4142135623731);
        var_xisub_dn6 = (var_gfsub_dn6 / 1.4142135623731);
        var_xisub_dn7 = (var_gfsub_dn7 / 1.4142135623731);
        var_xisub_dn8 = (var_gfsub_dn8 / 1.4142135623731);
        var_xisub_dn9 = (var_gfsub_dn9 / 1.4142135623731);

        let assign7100_e6874: f64 = (1.0 / var_xisub);
        var_inv_xisub = assign7100_e6874;
        var_inv_xisub_dn4 = (-(var_xisub_dn4 / (var_xisub * var_xisub)));
        var_inv_xisub_dn6 = (-(var_xisub_dn6 / (var_xisub * var_xisub)));
        var_inv_xisub_dn7 = (-(var_xisub_dn7 / (var_xisub * var_xisub)));
        var_inv_xisub_dn8 = (-(var_xisub_dn8 / (var_xisub * var_xisub)));
        var_inv_xisub_dn9 = (-(var_xisub_dn9 / (var_xisub * var_xisub)));

        let assign7110_e6877: f64 = (1e-5 * var_xisub);
        var_margin_sub = assign7110_e6877;

        let assign7120_e6880: f64 = (var_nsub_i / var_neff_sub);
        let assign7120_e6881: f64 = (assign7120_e6880).ln();
        let assign7120_e6883: f64 = (assign7120_e6881 + var_eg_2phit0);
        var_xb_sub = assign7120_e6883;
        var_xb_sub_dn4 = (((-((var_nsub_i * var_neff_sub_dn4) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn4);
        var_xb_sub_dn6 = (((-((var_nsub_i * var_neff_sub_dn6) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn6);
        var_xb_sub_dn7 = (((-((var_nsub_i * var_neff_sub_dn7) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn7);
        var_xb_sub_dn8 = (((-((var_nsub_i * var_neff_sub_dn8) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn8);
        var_xb_sub_dn9 = (((-((var_nsub_i * var_neff_sub_dn9) / (var_neff_sub * var_neff_sub))) / assign7120_e6880) + var_eg_2phit0_dn9);

        let assign7130_e6886: f64 = (2.0 * var_xb_sub);
        var_xn_sub = assign7130_e6886;
        var_xn_sub_dn4 = (2.0 * var_xb_sub_dn4);
        var_xn_sub_dn6 = (2.0 * var_xb_sub_dn6);
        var_xn_sub_dn7 = (2.0 * var_xb_sub_dn7);
        var_xn_sub_dn8 = (2.0 * var_xb_sub_dn8);
        var_xn_sub_dn9 = (2.0 * var_xb_sub_dn9);

        let assign7140_e6889: f64 = if p.p2 > 0.0 { 1.0 } else { 0.0 };
        var_guard142 = assign7140_e6889;

        let (assign7150_e6899, assign7150_e6899_d_n4, assign7150_e6899_d_n6, assign7150_e6899_d_n7, assign7150_e6899_d_n8, assign7150_e6899_d_n9,) = {
    if (var_guard142 != 0.0) {
        let assign7150_e6894: f64 = (var_typesub_i * var_phit0);
        let assign7150_e6896: f64 = (assign7150_e6894 * var_xb_sub);
        let assign7150_e6897: f64 = (var_vfb2_t + assign7150_e6896);
        (assign7150_e6897, (var_vfb2_t_dn4 + (((var_typesub_i * var_phit0_dn4) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn4))), (var_vfb2_t_dn6 + (((var_typesub_i * var_phit0_dn6) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn6))), (var_vfb2_t_dn7 + (((var_typesub_i * var_phit0_dn7) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn7))), (var_vfb2_t_dn8 + (((var_typesub_i * var_phit0_dn8) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn8))), (var_vfb2_t_dn9 + (((var_typesub_i * var_phit0_dn9) * var_xb_sub) + (assign7150_e6894 * var_xb_sub_dn9))),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn4, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9,)
    }
};
        var_vfb2_t = assign7150_e6899;
        var_vfb2_t_dn4 = assign7150_e6899_d_n4;
        var_vfb2_t_dn6 = assign7150_e6899_d_n6;
        var_vfb2_t_dn7 = assign7150_e6899_d_n7;
        var_vfb2_t_dn8 = assign7150_e6899_d_n8;
        var_vfb2_t_dn9 = assign7150_e6899_d_n9;

        let (assign7160_e6909, assign7160_e6909_d_n4, assign7160_e6909_d_n6, assign7160_e6909_d_n7, assign7160_e6909_d_n8, assign7160_e6909_d_n9,) = {
    if (var_guard142 != 0.0) {
        let assign7160_e6904: f64 = (var_typesub_i * var_phit0);
        let assign7160_e6906: f64 = (assign7160_e6904 * var_xb_sub);
        let assign7160_e6907: f64 = (var_vfbac2_t + assign7160_e6906);
        (assign7160_e6907, (var_vfbac2_t_dn4 + (((var_typesub_i * var_phit0_dn4) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn4))), (var_vfbac2_t_dn6 + (((var_typesub_i * var_phit0_dn6) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn6))), (var_vfbac2_t_dn7 + (((var_typesub_i * var_phit0_dn7) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn7))), (var_vfbac2_t_dn8 + (((var_typesub_i * var_phit0_dn8) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn8))), (var_vfbac2_t_dn9 + (((var_typesub_i * var_phit0_dn9) * var_xb_sub) + (assign7160_e6904 * var_xb_sub_dn9))),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn4, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9,)
    }
};
        var_vfbac2_t = assign7160_e6909;
        var_vfbac2_t_dn4 = assign7160_e6909_d_n4;
        var_vfbac2_t_dn6 = assign7160_e6909_d_n6;
        var_vfbac2_t_dn7 = assign7160_e6909_d_n7;
        var_vfbac2_t_dn8 = assign7160_e6909_d_n8;
        var_vfbac2_t_dn9 = assign7160_e6909_d_n9;

        var_dvfbpdep = 0.0;
        var_dvfbpdep_dn4 = 0.0;
        var_dvfbpdep_dn6 = 0.0;
        var_dvfbpdep_dn7 = 0.0;
        var_dvfbpdep_dn8 = 0.0;
        var_dvfbpdep_dn9 = 0.0;

        let assign7180_e6913: f64 = if p.p9 > 0.0 { 1.0 } else { 0.0 };
        var_guard143 = assign7180_e6913;

        let (assign7190_e6924, assign7190_e6924_d_n4, assign7190_e6924_d_n6, assign7190_e6924_d_n7, assign7190_e6924_d_n8, assign7190_e6924_d_n9,) = {
    if (var_guard143 != 0.0) {
        let assign7190_e6918: f64 = (var_np_i / var_neff_poly);
        let assign7190_e6919: f64 = (assign7190_e6918).ln();
        let assign7190_e6921: f64 = (assign7190_e6919 + var_eg_2phit0);
        let assign7190_e6922: f64 = (var_phit0 * assign7190_e6921);
        (assign7190_e6922, ((var_phit0_dn4 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn4 * var_neff_poly) - (var_np_i * var_neff_poly_dn4)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn4))), ((var_phit0_dn6 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn6 * var_neff_poly) - (var_np_i * var_neff_poly_dn6)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn6))), ((var_phit0_dn7 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn7 * var_neff_poly) - (var_np_i * var_neff_poly_dn7)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn7))), ((var_phit0_dn8 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn8 * var_neff_poly) - (var_np_i * var_neff_poly_dn8)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn8))), ((var_phit0_dn9 * assign7190_e6921) + (var_phit0 * (((((var_np_i_dn9 * var_neff_poly) - (var_np_i * var_neff_poly_dn9)) / (var_neff_poly * var_neff_poly)) / assign7190_e6918) + var_eg_2phit0_dn9))),)
    } else {
        (var_dvfbpdep, var_dvfbpdep_dn4, var_dvfbpdep_dn6, var_dvfbpdep_dn7, var_dvfbpdep_dn8, var_dvfbpdep_dn9,)
    }
};
        var_dvfbpdep = assign7190_e6924;
        var_dvfbpdep_dn4 = assign7190_e6924_d_n4;
        var_dvfbpdep_dn6 = assign7190_e6924_d_n6;
        var_dvfbpdep_dn7 = assign7190_e6924_d_n7;
        var_dvfbpdep_dn8 = assign7190_e6924_d_n8;
        var_dvfbpdep_dn9 = assign7190_e6924_d_n9;

        let assign7200_e6927: f64 = (2.0 * 1.602176565e-19);
        let assign7200_e6929: f64 = (assign7200_e6927 * var_epsch);
        let assign7200_e6931: f64 = (assign7200_e6929 * var_np_i);
        let assign7200_e6932: f64 = (assign7200_e6931).sqrt();
        let assign7200_e6934: f64 = (assign7200_e6932 / var_cox1init);
        var_kp = assign7200_e6934;
        var_kp_dn4 = (((assign7200_e6929 * var_np_i_dn4) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn6 = (((assign7200_e6929 * var_np_i_dn6) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn7 = (((assign7200_e6929 * var_np_i_dn7) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn8 = (((assign7200_e6929 * var_np_i_dn8) / (2.0 * assign7200_e6932)) / var_cox1init);
        var_kp_dn9 = (((assign7200_e6929 * var_np_i_dn9) / (2.0 * assign7200_e6932)) / var_cox1init);

        var_emin = 15.0;
        var_emin_dn4 = 0.0;
        var_emin_dn6 = 0.0;
        var_emin_dn7 = 0.0;
        var_emin_dn8 = 0.0;
        var_emin_dn9 = 0.0;

        *var_a0_csisq_slot = var_a0_csisq;
        *var_a0_csisq_dn4_slot = var_a0_csisq_dn4;
        *var_a0_csisq_dn6_slot = var_a0_csisq_dn6;
        *var_a0_csisq_dn7_slot = var_a0_csisq_dn7;
        *var_a0_csisq_dn8_slot = var_a0_csisq_dn8;
        *var_a0_csisq_dn9_slot = var_a0_csisq_dn9;
        *var_cf1_i_slot = var_cf1_i;
        *var_cf1_i_dn4_slot = var_cf1_i_dn4;
        *var_cf1_i_dn6_slot = var_cf1_i_dn6;
        *var_cf1_i_dn7_slot = var_cf1_i_dn7;
        *var_cf1_i_dn8_slot = var_cf1_i_dn8;
        *var_cf1_i_dn9_slot = var_cf1_i_dn9;
        *var_cf2_i_slot = var_cf2_i;
        *var_cf2_i_dn4_slot = var_cf2_i_dn4;
        *var_cf2_i_dn6_slot = var_cf2_i_dn6;
        *var_cf2_i_dn7_slot = var_cf2_i_dn7;
        *var_cf2_i_dn8_slot = var_cf2_i_dn8;
        *var_cf2_i_dn9_slot = var_cf2_i_dn9;
        *var_cfac1_i_slot = var_cfac1_i;
        *var_cfac1_i_dn4_slot = var_cfac1_i_dn4;
        *var_cfac1_i_dn6_slot = var_cfac1_i_dn6;
        *var_cfac1_i_dn7_slot = var_cfac1_i_dn7;
        *var_cfac1_i_dn8_slot = var_cfac1_i_dn8;
        *var_cfac1_i_dn9_slot = var_cfac1_i_dn9;
        *var_cfac2_i_slot = var_cfac2_i;
        *var_cfac2_i_dn4_slot = var_cfac2_i_dn4;
        *var_cfac2_i_dn6_slot = var_cfac2_i_dn6;
        *var_cfac2_i_dn7_slot = var_cfac2_i_dn7;
        *var_cfac2_i_dn8_slot = var_cfac2_i_dn8;
        *var_cfac2_i_dn9_slot = var_cfac2_i_dn9;
        *var_cox1init_slot = var_cox1init;
        *var_cox1prime_slot = var_cox1prime;
        *var_cox2init_slot = var_cox2init;
        *var_cox2prime_slot = var_cox2prime;
        *var_csiprime_0_slot = var_csiprime_0;
        *var_deg_slot = var_deg;
        *var_deg_dn4_slot = var_deg_dn4;
        *var_deg_dn6_slot = var_deg_dn6;
        *var_deg_dn7_slot = var_deg_dn7;
        *var_deg_dn8_slot = var_deg_dn8;
        *var_deg_dn9_slot = var_deg_dn9;
        *var_dvfb1nch_slot = var_dvfb1nch;
        *var_dvfb1nch_dn4_slot = var_dvfb1nch_dn4;
        *var_dvfb1nch_dn6_slot = var_dvfb1nch_dn6;
        *var_dvfb1nch_dn7_slot = var_dvfb1nch_dn7;
        *var_dvfb1nch_dn8_slot = var_dvfb1nch_dn8;
        *var_dvfb1nch_dn9_slot = var_dvfb1nch_dn9;
        *var_dvfb2nch_slot = var_dvfb2nch;
        *var_dvfb2nch_dn4_slot = var_dvfb2nch_dn4;
        *var_dvfb2nch_dn6_slot = var_dvfb2nch_dn6;
        *var_dvfb2nch_dn7_slot = var_dvfb2nch_dn7;
        *var_dvfb2nch_dn8_slot = var_dvfb2nch_dn8;
        *var_dvfb2nch_dn9_slot = var_dvfb2nch_dn9;
        *var_dvfbch_slot = var_dvfbch;
        *var_dvfbch_dn4_slot = var_dvfbch_dn4;
        *var_dvfbch_dn6_slot = var_dvfbch_dn6;
        *var_dvfbch_dn7_slot = var_dvfbch_dn7;
        *var_dvfbch_dn8_slot = var_dvfbch_dn8;
        *var_dvfbch_dn9_slot = var_dvfbch_dn9;
        *var_dvfbpdep_slot = var_dvfbpdep;
        *var_dvfbpdep_dn4_slot = var_dvfbpdep_dn4;
        *var_dvfbpdep_dn6_slot = var_dvfbpdep_dn6;
        *var_dvfbpdep_dn7_slot = var_dvfbpdep_dn7;
        *var_dvfbpdep_dn8_slot = var_dvfbpdep_dn8;
        *var_dvfbpdep_dn9_slot = var_dvfbpdep_dn9;
        *var_eg_slot = var_eg;
        *var_eg_2phit_slot = var_eg_2phit;
        *var_eg_2phit0_slot = var_eg_2phit0;
        *var_eg_2phit0_dn4_slot = var_eg_2phit0_dn4;
        *var_eg_2phit0_dn6_slot = var_eg_2phit0_dn6;
        *var_eg_2phit0_dn7_slot = var_eg_2phit0_dn7;
        *var_eg_2phit0_dn8_slot = var_eg_2phit0_dn8;
        *var_eg_2phit0_dn9_slot = var_eg_2phit0_dn9;
        *var_eg_2phit0_woshe_slot = var_eg_2phit0_woshe;
        *var_eg_2phit0_woshe_dn4_slot = var_eg_2phit0_woshe_dn4;
        *var_eg_2phit0_woshe_dn6_slot = var_eg_2phit0_woshe_dn6;
        *var_eg_2phit0_woshe_dn7_slot = var_eg_2phit0_woshe_dn7;
        *var_eg_2phit0_woshe_dn8_slot = var_eg_2phit0_woshe_dn8;
        *var_eg_2phit0_woshe_dn9_slot = var_eg_2phit0_woshe_dn9;
        *var_eg_2phit_dn4_slot = var_eg_2phit_dn4;
        *var_eg_2phit_dn6_slot = var_eg_2phit_dn6;
        *var_eg_2phit_dn7_slot = var_eg_2phit_dn7;
        *var_eg_2phit_dn8_slot = var_eg_2phit_dn8;
        *var_eg_2phit_dn9_slot = var_eg_2phit_dn9;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_dn8_slot = var_eg_dn8;
        *var_eg_dn9_slot = var_eg_dn9;
        *var_emin_slot = var_emin;
        *var_emin_dn4_slot = var_emin_dn4;
        *var_emin_dn6_slot = var_emin_dn6;
        *var_emin_dn7_slot = var_emin_dn7;
        *var_emin_dn8_slot = var_emin_dn8;
        *var_emin_dn9_slot = var_emin_dn9;
        *var_gfsub_slot = var_gfsub;
        *var_gfsub2_slot = var_gfsub2;
        *var_gfsub2_dn4_slot = var_gfsub2_dn4;
        *var_gfsub2_dn6_slot = var_gfsub2_dn6;
        *var_gfsub2_dn7_slot = var_gfsub2_dn7;
        *var_gfsub2_dn8_slot = var_gfsub2_dn8;
        *var_gfsub2_dn9_slot = var_gfsub2_dn9;
        *var_gfsub_dn4_slot = var_gfsub_dn4;
        *var_gfsub_dn6_slot = var_gfsub_dn6;
        *var_gfsub_dn7_slot = var_gfsub_dn7;
        *var_gfsub_dn8_slot = var_gfsub_dn8;
        *var_gfsub_dn9_slot = var_gfsub_dn9;
        *var_guard140_slot = var_guard140;
        *var_guard141_slot = var_guard141;
        *var_guard142_slot = var_guard142;
        *var_guard143_slot = var_guard143;
        *var_inv_gfsub2_slot = var_inv_gfsub2;
        *var_inv_gfsub2_dn4_slot = var_inv_gfsub2_dn4;
        *var_inv_gfsub2_dn6_slot = var_inv_gfsub2_dn6;
        *var_inv_gfsub2_dn7_slot = var_inv_gfsub2_dn7;
        *var_inv_gfsub2_dn8_slot = var_inv_gfsub2_dn8;
        *var_inv_gfsub2_dn9_slot = var_inv_gfsub2_dn9;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_dn4_slot = var_inv_phit_dn4;
        *var_inv_phit_dn6_slot = var_inv_phit_dn6;
        *var_inv_phit_dn7_slot = var_inv_phit_dn7;
        *var_inv_phit_dn8_slot = var_inv_phit_dn8;
        *var_inv_phit_dn9_slot = var_inv_phit_dn9;
        *var_inv_xisub_slot = var_inv_xisub;
        *var_inv_xisub_dn4_slot = var_inv_xisub_dn4;
        *var_inv_xisub_dn6_slot = var_inv_xisub_dn6;
        *var_inv_xisub_dn7_slot = var_inv_xisub_dn7;
        *var_inv_xisub_dn8_slot = var_inv_xisub_dn8;
        *var_inv_xisub_dn9_slot = var_inv_xisub_dn9;
        *var_k1_1d_slot = var_k1_1d;
        *var_k2_1d_slot = var_k2_1d;
        *var_keq_1d_slot = var_keq_1d;
        *var_kp_slot = var_kp;
        *var_kp_dn4_slot = var_kp_dn4;
        *var_kp_dn6_slot = var_kp_dn6;
        *var_kp_dn7_slot = var_kp_dn7;
        *var_kp_dn8_slot = var_kp_dn8;
        *var_kp_dn9_slot = var_kp_dn9;
        *var_margin_sub_slot = var_margin_sub;
        *var_neff_slot = var_neff;
        *var_neff_dn4_slot = var_neff_dn4;
        *var_neff_dn6_slot = var_neff_dn6;
        *var_neff_dn7_slot = var_neff_dn7;
        *var_neff_dn8_slot = var_neff_dn8;
        *var_neff_dn9_slot = var_neff_dn9;
        *var_neff_poly_slot = var_neff_poly;
        *var_neff_poly_dn4_slot = var_neff_poly_dn4;
        *var_neff_poly_dn6_slot = var_neff_poly_dn6;
        *var_neff_poly_dn7_slot = var_neff_poly_dn7;
        *var_neff_poly_dn8_slot = var_neff_poly_dn8;
        *var_neff_poly_dn9_slot = var_neff_poly_dn9;
        *var_neff_sub_slot = var_neff_sub;
        *var_neff_sub_dn4_slot = var_neff_sub_dn4;
        *var_neff_sub_dn6_slot = var_neff_sub_dn6;
        *var_neff_sub_dn7_slot = var_neff_sub_dn7;
        *var_neff_sub_dn8_slot = var_neff_sub_dn8;
        *var_neff_sub_dn9_slot = var_neff_sub_dn9;
        *var_niratio_slot = var_niratio;
        *var_phit_slot = var_phit;
        *var_phit_dn4_slot = var_phit_dn4;
        *var_phit_dn6_slot = var_phit_dn6;
        *var_phit_dn7_slot = var_phit_dn7;
        *var_phit_dn8_slot = var_phit_dn8;
        *var_phit_dn9_slot = var_phit_dn9;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
        *var_xb_sub_slot = var_xb_sub;
        *var_xb_sub_dn4_slot = var_xb_sub_dn4;
        *var_xb_sub_dn6_slot = var_xb_sub_dn6;
        *var_xb_sub_dn7_slot = var_xb_sub_dn7;
        *var_xb_sub_dn8_slot = var_xb_sub_dn8;
        *var_xb_sub_dn9_slot = var_xb_sub_dn9;
        *var_xd0_slot = var_xd0;
        *var_xd0_dn4_slot = var_xd0_dn4;
        *var_xd0_dn6_slot = var_xd0_dn6;
        *var_xd0_dn7_slot = var_xd0_dn7;
        *var_xd0_dn8_slot = var_xd0_dn8;
        *var_xd0_dn9_slot = var_xd0_dn9;
        *var_xisub_slot = var_xisub;
        *var_xisub_dn4_slot = var_xisub_dn4;
        *var_xisub_dn6_slot = var_xisub_dn6;
        *var_xisub_dn7_slot = var_xisub_dn7;
        *var_xisub_dn8_slot = var_xisub_dn8;
        *var_xisub_dn9_slot = var_xisub_dn9;
        *var_xn_sub_slot = var_xn_sub;
        *var_xn_sub_dn4_slot = var_xn_sub_dn4;
        *var_xn_sub_dn6_slot = var_xn_sub_dn6;
        *var_xn_sub_dn7_slot = var_xn_sub_dn7;
        *var_xn_sub_dn8_slot = var_xn_sub_dn8;
        *var_xn_sub_dn9_slot = var_xn_sub_dn9;
        *var_xsddep_slot = var_xsddep;
        *var_xsddep_dn4_slot = var_xsddep_dn4;
        *var_xsddep_dn6_slot = var_xsddep_dn6;
        *var_xsddep_dn7_slot = var_xsddep_dn7;
        *var_xsddep_dn8_slot = var_xsddep_dn8;
        *var_xsddep_dn9_slot = var_xsddep_dn9;
        *var_xth_1d_slot = var_xth_1d;
        *var_xth_1d_dn4_slot = var_xth_1d_dn4;
        *var_xth_1d_dn6_slot = var_xth_1d_dn6;
        *var_xth_1d_dn7_slot = var_xth_1d_dn7;
        *var_xth_1d_dn8_slot = var_xth_1d_dn8;
        *var_xth_1d_dn9_slot = var_xth_1d_dn9;
    }
}
