#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_adrain_i_slot: &mut f64,
        var_asource_i_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_dt_slot: &mut f64,
        var_dt_db0_slot: &mut f64,
        var_dt_db1_slot: &mut f64,
        var_dt_db2_slot: &mut f64,
        var_dt_db3_slot: &mut f64,
        var_dt_dn0_slot: &mut f64,
        var_dt_dn1_slot: &mut f64,
        var_dt_dn2_slot: &mut f64,
        var_dt_dn3_slot: &mut f64,
        var_dt_dn4_slot: &mut f64,
        var_dt_dn5_slot: &mut f64,
        var_dt_dn6_slot: &mut f64,
        var_dt_dn7_slot: &mut f64,
        var_dt_dn8_slot: &mut f64,
        var_dt_dn9_slot: &mut f64,
        var_dtc_slot: &mut f64,
        var_dtc_db0_slot: &mut f64,
        var_dtc_db1_slot: &mut f64,
        var_dtc_db2_slot: &mut f64,
        var_dtc_db3_slot: &mut f64,
        var_dtc_dn0_slot: &mut f64,
        var_dtc_dn1_slot: &mut f64,
        var_dtc_dn2_slot: &mut f64,
        var_dtc_dn3_slot: &mut f64,
        var_dtc_dn4_slot: &mut f64,
        var_dtc_dn5_slot: &mut f64,
        var_dtc_dn6_slot: &mut f64,
        var_dtc_dn7_slot: &mut f64,
        var_dtc_dn8_slot: &mut f64,
        var_dtc_dn9_slot: &mut f64,
        var_guard1_slot: &mut f64,
        var_guard2_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_inv_phit0_slot: &mut f64,
        var_inv_phit0_db0_slot: &mut f64,
        var_inv_phit0_db1_slot: &mut f64,
        var_inv_phit0_db2_slot: &mut f64,
        var_inv_phit0_db3_slot: &mut f64,
        var_inv_phit0_dn0_slot: &mut f64,
        var_inv_phit0_dn1_slot: &mut f64,
        var_inv_phit0_dn2_slot: &mut f64,
        var_inv_phit0_dn3_slot: &mut f64,
        var_inv_phit0_dn4_slot: &mut f64,
        var_inv_phit0_dn5_slot: &mut f64,
        var_inv_phit0_dn6_slot: &mut f64,
        var_inv_phit0_dn7_slot: &mut f64,
        var_inv_phit0_dn8_slot: &mut f64,
        var_inv_phit0_dn9_slot: &mut f64,
        var_mult_i_int_slot: &mut f64,
        var_nch_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_nsub_i_slot: &mut f64,
        var_pdrain_i_slot: &mut f64,
        var_phit0_slot: &mut f64,
        var_phit0_db0_slot: &mut f64,
        var_phit0_db1_slot: &mut f64,
        var_phit0_db2_slot: &mut f64,
        var_phit0_db3_slot: &mut f64,
        var_phit0_dn0_slot: &mut f64,
        var_phit0_dn1_slot: &mut f64,
        var_phit0_dn2_slot: &mut f64,
        var_phit0_dn3_slot: &mut f64,
        var_phit0_dn4_slot: &mut f64,
        var_phit0_dn5_slot: &mut f64,
        var_phit0_dn6_slot: &mut f64,
        var_phit0_dn7_slot: &mut f64,
        var_phit0_dn8_slot: &mut f64,
        var_phit0_dn9_slot: &mut f64,
        var_psource_i_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_db0_slot: &mut f64,
        var_rt_db1_slot: &mut f64,
        var_rt_db2_slot: &mut f64,
        var_rt_db3_slot: &mut f64,
        var_rt_dn0_slot: &mut f64,
        var_rt_dn1_slot: &mut f64,
        var_rt_dn2_slot: &mut f64,
        var_rt_dn3_slot: &mut f64,
        var_rt_dn4_slot: &mut f64,
        var_rt_dn5_slot: &mut f64,
        var_rt_dn6_slot: &mut f64,
        var_rt_dn7_slot: &mut f64,
        var_rt_dn8_slot: &mut f64,
        var_rt_dn9_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_db0_slot: &mut f64,
        var_rtn_db1_slot: &mut f64,
        var_rtn_db2_slot: &mut f64,
        var_rtn_db3_slot: &mut f64,
        var_rtn_dn0_slot: &mut f64,
        var_rtn_dn1_slot: &mut f64,
        var_rtn_dn2_slot: &mut f64,
        var_rtn_dn3_slot: &mut f64,
        var_rtn_dn4_slot: &mut f64,
        var_rtn_dn5_slot: &mut f64,
        var_rtn_dn6_slot: &mut f64,
        var_rtn_dn7_slot: &mut f64,
        var_rtn_dn8_slot: &mut f64,
        var_rtn_dn9_slot: &mut f64,
        var_swshe_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_tkc_slot: &mut f64,
        var_tkc_db0_slot: &mut f64,
        var_tkc_db1_slot: &mut f64,
        var_tkc_db2_slot: &mut f64,
        var_tkc_db3_slot: &mut f64,
        var_tkc_dn0_slot: &mut f64,
        var_tkc_dn1_slot: &mut f64,
        var_tkc_dn2_slot: &mut f64,
        var_tkc_dn3_slot: &mut f64,
        var_tkc_dn4_slot: &mut f64,
        var_tkc_dn5_slot: &mut f64,
        var_tkc_dn6_slot: &mut f64,
        var_tkc_dn7_slot: &mut f64,
        var_tkc_dn8_slot: &mut f64,
        var_tkc_dn9_slot: &mut f64,
        var_tkc_sq_slot: &mut f64,
        var_tkc_sq_db0_slot: &mut f64,
        var_tkc_sq_db1_slot: &mut f64,
        var_tkc_sq_db2_slot: &mut f64,
        var_tkc_sq_db3_slot: &mut f64,
        var_tkc_sq_dn0_slot: &mut f64,
        var_tkc_sq_dn1_slot: &mut f64,
        var_tkc_sq_dn2_slot: &mut f64,
        var_tkc_sq_dn3_slot: &mut f64,
        var_tkc_sq_dn4_slot: &mut f64,
        var_tkc_sq_dn5_slot: &mut f64,
        var_tkc_sq_dn6_slot: &mut f64,
        var_tkc_sq_dn7_slot: &mut f64,
        var_tkc_sq_dn8_slot: &mut f64,
        var_tkc_sq_dn9_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_db0_slot: &mut f64,
        var_tkd_db1_slot: &mut f64,
        var_tkd_db2_slot: &mut f64,
        var_tkd_db3_slot: &mut f64,
        var_tkd_dn0_slot: &mut f64,
        var_tkd_dn1_slot: &mut f64,
        var_tkd_dn2_slot: &mut f64,
        var_tkd_dn3_slot: &mut f64,
        var_tkd_dn4_slot: &mut f64,
        var_tkd_dn5_slot: &mut f64,
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
        var_xge_i_slot: &mut f64,
        var_xsatmax_slot: &mut f64,
        var_xsatmax_db0_slot: &mut f64,
        var_xsatmax_db1_slot: &mut f64,
        var_xsatmax_db2_slot: &mut f64,
        var_xsatmax_db3_slot: &mut f64,
        var_xsatmax_dn0_slot: &mut f64,
        var_xsatmax_dn1_slot: &mut f64,
        var_xsatmax_dn2_slot: &mut f64,
        var_xsatmax_dn3_slot: &mut f64,
        var_xsatmax_dn4_slot: &mut f64,
        var_xsatmax_dn5_slot: &mut f64,
        var_xsatmax_dn6_slot: &mut f64,
        var_xsatmax_dn7_slot: &mut f64,
        var_xsatmax_dn8_slot: &mut f64,
        var_xsatmax_dn9_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_adrain_i: f64 = *var_adrain_i_slot;
        let mut var_asource_i: f64 = *var_asource_i_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_dt: f64 = *var_dt_slot;
        let mut var_dt_db0: f64 = *var_dt_db0_slot;
        let mut var_dt_db1: f64 = *var_dt_db1_slot;
        let mut var_dt_db2: f64 = *var_dt_db2_slot;
        let mut var_dt_db3: f64 = *var_dt_db3_slot;
        let mut var_dt_dn0: f64 = *var_dt_dn0_slot;
        let mut var_dt_dn1: f64 = *var_dt_dn1_slot;
        let mut var_dt_dn2: f64 = *var_dt_dn2_slot;
        let mut var_dt_dn3: f64 = *var_dt_dn3_slot;
        let mut var_dt_dn4: f64 = *var_dt_dn4_slot;
        let mut var_dt_dn5: f64 = *var_dt_dn5_slot;
        let mut var_dt_dn6: f64 = *var_dt_dn6_slot;
        let mut var_dt_dn7: f64 = *var_dt_dn7_slot;
        let mut var_dt_dn8: f64 = *var_dt_dn8_slot;
        let mut var_dt_dn9: f64 = *var_dt_dn9_slot;
        let mut var_dtc: f64 = *var_dtc_slot;
        let mut var_dtc_db0: f64 = *var_dtc_db0_slot;
        let mut var_dtc_db1: f64 = *var_dtc_db1_slot;
        let mut var_dtc_db2: f64 = *var_dtc_db2_slot;
        let mut var_dtc_db3: f64 = *var_dtc_db3_slot;
        let mut var_dtc_dn0: f64 = *var_dtc_dn0_slot;
        let mut var_dtc_dn1: f64 = *var_dtc_dn1_slot;
        let mut var_dtc_dn2: f64 = *var_dtc_dn2_slot;
        let mut var_dtc_dn3: f64 = *var_dtc_dn3_slot;
        let mut var_dtc_dn4: f64 = *var_dtc_dn4_slot;
        let mut var_dtc_dn5: f64 = *var_dtc_dn5_slot;
        let mut var_dtc_dn6: f64 = *var_dtc_dn6_slot;
        let mut var_dtc_dn7: f64 = *var_dtc_dn7_slot;
        let mut var_dtc_dn8: f64 = *var_dtc_dn8_slot;
        let mut var_dtc_dn9: f64 = *var_dtc_dn9_slot;
        let mut var_guard1: f64 = *var_guard1_slot;
        let mut var_guard2: f64 = *var_guard2_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_inv_phit0: f64 = *var_inv_phit0_slot;
        let mut var_inv_phit0_db0: f64 = *var_inv_phit0_db0_slot;
        let mut var_inv_phit0_db1: f64 = *var_inv_phit0_db1_slot;
        let mut var_inv_phit0_db2: f64 = *var_inv_phit0_db2_slot;
        let mut var_inv_phit0_db3: f64 = *var_inv_phit0_db3_slot;
        let mut var_inv_phit0_dn0: f64 = *var_inv_phit0_dn0_slot;
        let mut var_inv_phit0_dn1: f64 = *var_inv_phit0_dn1_slot;
        let mut var_inv_phit0_dn2: f64 = *var_inv_phit0_dn2_slot;
        let mut var_inv_phit0_dn3: f64 = *var_inv_phit0_dn3_slot;
        let mut var_inv_phit0_dn4: f64 = *var_inv_phit0_dn4_slot;
        let mut var_inv_phit0_dn5: f64 = *var_inv_phit0_dn5_slot;
        let mut var_inv_phit0_dn6: f64 = *var_inv_phit0_dn6_slot;
        let mut var_inv_phit0_dn7: f64 = *var_inv_phit0_dn7_slot;
        let mut var_inv_phit0_dn8: f64 = *var_inv_phit0_dn8_slot;
        let mut var_inv_phit0_dn9: f64 = *var_inv_phit0_dn9_slot;
        let mut var_mult_i_int: f64 = *var_mult_i_int_slot;
        let mut var_nch_i: f64 = *var_nch_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_nsub_i: f64 = *var_nsub_i_slot;
        let mut var_pdrain_i: f64 = *var_pdrain_i_slot;
        let mut var_phit0: f64 = *var_phit0_slot;
        let mut var_phit0_db0: f64 = *var_phit0_db0_slot;
        let mut var_phit0_db1: f64 = *var_phit0_db1_slot;
        let mut var_phit0_db2: f64 = *var_phit0_db2_slot;
        let mut var_phit0_db3: f64 = *var_phit0_db3_slot;
        let mut var_phit0_dn0: f64 = *var_phit0_dn0_slot;
        let mut var_phit0_dn1: f64 = *var_phit0_dn1_slot;
        let mut var_phit0_dn2: f64 = *var_phit0_dn2_slot;
        let mut var_phit0_dn3: f64 = *var_phit0_dn3_slot;
        let mut var_phit0_dn4: f64 = *var_phit0_dn4_slot;
        let mut var_phit0_dn5: f64 = *var_phit0_dn5_slot;
        let mut var_phit0_dn6: f64 = *var_phit0_dn6_slot;
        let mut var_phit0_dn7: f64 = *var_phit0_dn7_slot;
        let mut var_phit0_dn8: f64 = *var_phit0_dn8_slot;
        let mut var_phit0_dn9: f64 = *var_phit0_dn9_slot;
        let mut var_psource_i: f64 = *var_psource_i_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_db0: f64 = *var_rt_db0_slot;
        let mut var_rt_db1: f64 = *var_rt_db1_slot;
        let mut var_rt_db2: f64 = *var_rt_db2_slot;
        let mut var_rt_db3: f64 = *var_rt_db3_slot;
        let mut var_rt_dn0: f64 = *var_rt_dn0_slot;
        let mut var_rt_dn1: f64 = *var_rt_dn1_slot;
        let mut var_rt_dn2: f64 = *var_rt_dn2_slot;
        let mut var_rt_dn3: f64 = *var_rt_dn3_slot;
        let mut var_rt_dn4: f64 = *var_rt_dn4_slot;
        let mut var_rt_dn5: f64 = *var_rt_dn5_slot;
        let mut var_rt_dn6: f64 = *var_rt_dn6_slot;
        let mut var_rt_dn7: f64 = *var_rt_dn7_slot;
        let mut var_rt_dn8: f64 = *var_rt_dn8_slot;
        let mut var_rt_dn9: f64 = *var_rt_dn9_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_db0: f64 = *var_rtn_db0_slot;
        let mut var_rtn_db1: f64 = *var_rtn_db1_slot;
        let mut var_rtn_db2: f64 = *var_rtn_db2_slot;
        let mut var_rtn_db3: f64 = *var_rtn_db3_slot;
        let mut var_rtn_dn0: f64 = *var_rtn_dn0_slot;
        let mut var_rtn_dn1: f64 = *var_rtn_dn1_slot;
        let mut var_rtn_dn2: f64 = *var_rtn_dn2_slot;
        let mut var_rtn_dn3: f64 = *var_rtn_dn3_slot;
        let mut var_rtn_dn4: f64 = *var_rtn_dn4_slot;
        let mut var_rtn_dn5: f64 = *var_rtn_dn5_slot;
        let mut var_rtn_dn6: f64 = *var_rtn_dn6_slot;
        let mut var_rtn_dn7: f64 = *var_rtn_dn7_slot;
        let mut var_rtn_dn8: f64 = *var_rtn_dn8_slot;
        let mut var_rtn_dn9: f64 = *var_rtn_dn9_slot;
        let mut var_swshe_i: f64 = *var_swshe_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_tkc: f64 = *var_tkc_slot;
        let mut var_tkc_db0: f64 = *var_tkc_db0_slot;
        let mut var_tkc_db1: f64 = *var_tkc_db1_slot;
        let mut var_tkc_db2: f64 = *var_tkc_db2_slot;
        let mut var_tkc_db3: f64 = *var_tkc_db3_slot;
        let mut var_tkc_dn0: f64 = *var_tkc_dn0_slot;
        let mut var_tkc_dn1: f64 = *var_tkc_dn1_slot;
        let mut var_tkc_dn2: f64 = *var_tkc_dn2_slot;
        let mut var_tkc_dn3: f64 = *var_tkc_dn3_slot;
        let mut var_tkc_dn4: f64 = *var_tkc_dn4_slot;
        let mut var_tkc_dn5: f64 = *var_tkc_dn5_slot;
        let mut var_tkc_dn6: f64 = *var_tkc_dn6_slot;
        let mut var_tkc_dn7: f64 = *var_tkc_dn7_slot;
        let mut var_tkc_dn8: f64 = *var_tkc_dn8_slot;
        let mut var_tkc_dn9: f64 = *var_tkc_dn9_slot;
        let mut var_tkc_sq: f64 = *var_tkc_sq_slot;
        let mut var_tkc_sq_db0: f64 = *var_tkc_sq_db0_slot;
        let mut var_tkc_sq_db1: f64 = *var_tkc_sq_db1_slot;
        let mut var_tkc_sq_db2: f64 = *var_tkc_sq_db2_slot;
        let mut var_tkc_sq_db3: f64 = *var_tkc_sq_db3_slot;
        let mut var_tkc_sq_dn0: f64 = *var_tkc_sq_dn0_slot;
        let mut var_tkc_sq_dn1: f64 = *var_tkc_sq_dn1_slot;
        let mut var_tkc_sq_dn2: f64 = *var_tkc_sq_dn2_slot;
        let mut var_tkc_sq_dn3: f64 = *var_tkc_sq_dn3_slot;
        let mut var_tkc_sq_dn4: f64 = *var_tkc_sq_dn4_slot;
        let mut var_tkc_sq_dn5: f64 = *var_tkc_sq_dn5_slot;
        let mut var_tkc_sq_dn6: f64 = *var_tkc_sq_dn6_slot;
        let mut var_tkc_sq_dn7: f64 = *var_tkc_sq_dn7_slot;
        let mut var_tkc_sq_dn8: f64 = *var_tkc_sq_dn8_slot;
        let mut var_tkc_sq_dn9: f64 = *var_tkc_sq_dn9_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_db0: f64 = *var_tkd_db0_slot;
        let mut var_tkd_db1: f64 = *var_tkd_db1_slot;
        let mut var_tkd_db2: f64 = *var_tkd_db2_slot;
        let mut var_tkd_db3: f64 = *var_tkd_db3_slot;
        let mut var_tkd_dn0: f64 = *var_tkd_dn0_slot;
        let mut var_tkd_dn1: f64 = *var_tkd_dn1_slot;
        let mut var_tkd_dn2: f64 = *var_tkd_dn2_slot;
        let mut var_tkd_dn3: f64 = *var_tkd_dn3_slot;
        let mut var_tkd_dn4: f64 = *var_tkd_dn4_slot;
        let mut var_tkd_dn5: f64 = *var_tkd_dn5_slot;
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
        let mut var_xge_i: f64 = *var_xge_i_slot;
        let mut var_xsatmax: f64 = *var_xsatmax_slot;
        let mut var_xsatmax_db0: f64 = *var_xsatmax_db0_slot;
        let mut var_xsatmax_db1: f64 = *var_xsatmax_db1_slot;
        let mut var_xsatmax_db2: f64 = *var_xsatmax_db2_slot;
        let mut var_xsatmax_db3: f64 = *var_xsatmax_db3_slot;
        let mut var_xsatmax_dn0: f64 = *var_xsatmax_dn0_slot;
        let mut var_xsatmax_dn1: f64 = *var_xsatmax_dn1_slot;
        let mut var_xsatmax_dn2: f64 = *var_xsatmax_dn2_slot;
        let mut var_xsatmax_dn3: f64 = *var_xsatmax_dn3_slot;
        let mut var_xsatmax_dn4: f64 = *var_xsatmax_dn4_slot;
        let mut var_xsatmax_dn5: f64 = *var_xsatmax_dn5_slot;
        let mut var_xsatmax_dn6: f64 = *var_xsatmax_dn6_slot;
        let mut var_xsatmax_dn7: f64 = *var_xsatmax_dn7_slot;
        let mut var_xsatmax_dn8: f64 = *var_xsatmax_dn8_slot;
        let mut var_xsatmax_dn9: f64 = *var_xsatmax_dn9_slot;

        let assign00_e727: f64 = (273.15 + p.p15);
        var_tkr = assign00_e727;

        let assign10_e728: f64 = ctx_temp;
        let assign10_e730: f64 = (assign10_e728 + p.p36);
        let assign10_e732: f64 = (assign10_e730).min(1000.0);
        var_temp = assign10_e732;
        var_temp_dn0 = 0.0;
        var_temp_dn1 = 0.0;
        var_temp_dn2 = 0.0;
        var_temp_dn3 = 0.0;
        var_temp_dn4 = 0.0;
        var_temp_dn5 = 0.0;
        var_temp_dn6 = 0.0;
        var_temp_dn7 = 0.0;
        var_temp_dn8 = 0.0;
        var_temp_dn9 = 0.0;
        var_temp_db0 = 0.0;
        var_temp_db1 = 0.0;
        var_temp_db2 = 0.0;
        var_temp_db3 = 0.0;

        let assign20_e735: f64 = if p.p10 == 1.0 { 1.0 } else { 0.0 };
        var_guard1 = assign20_e735;

        let (assign30_e766, assign30_e766_d_n0, assign30_e766_d_n1, assign30_e766_d_n2, assign30_e766_d_n3, assign30_e766_d_n4, assign30_e766_d_n5, assign30_e766_d_n6, assign30_e766_d_n7, assign30_e766_d_n8, assign30_e766_d_n9, assign30_e766_d_b0, assign30_e766_d_b1, assign30_e766_d_b2, assign30_e766_d_b3,) = {
    if (var_guard1 != 0.0) {
        let assign30_e742: f64 = (p.p18 * var_temp);
        let assign30_e743: f64 = (p.p17 + assign30_e742);
        let assign30_e744: f64 = (var_temp + assign30_e743);
        let assign30_e749: f64 = (p.p18 * var_temp);
        let assign30_e750: f64 = (p.p17 + assign30_e749);
        let assign30_e751: f64 = (var_temp - assign30_e750);
        let assign30_e756: f64 = (p.p18 * var_temp);
        let assign30_e757: f64 = (p.p17 + assign30_e756);
        let assign30_e758: f64 = (var_temp - assign30_e757);
        let assign30_e759: f64 = (assign30_e751 * assign30_e758);
        let assign30_e761: f64 = (assign30_e759 + p.p19);
        let assign30_e762: f64 = (assign30_e761).sqrt();
        let assign30_e763: f64 = (assign30_e744 + assign30_e762);
        let assign30_e764: f64 = (0.5 * assign30_e763);
        (assign30_e764, (0.5 * ((var_temp_dn0 + (p.p18 * var_temp_dn0)) + ((((var_temp_dn0 - (p.p18 * var_temp_dn0)) * assign30_e758) + (assign30_e751 * (var_temp_dn0 - (p.p18 * var_temp_dn0)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn1 + (p.p18 * var_temp_dn1)) + ((((var_temp_dn1 - (p.p18 * var_temp_dn1)) * assign30_e758) + (assign30_e751 * (var_temp_dn1 - (p.p18 * var_temp_dn1)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn2 + (p.p18 * var_temp_dn2)) + ((((var_temp_dn2 - (p.p18 * var_temp_dn2)) * assign30_e758) + (assign30_e751 * (var_temp_dn2 - (p.p18 * var_temp_dn2)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn3 + (p.p18 * var_temp_dn3)) + ((((var_temp_dn3 - (p.p18 * var_temp_dn3)) * assign30_e758) + (assign30_e751 * (var_temp_dn3 - (p.p18 * var_temp_dn3)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn4 + (p.p18 * var_temp_dn4)) + ((((var_temp_dn4 - (p.p18 * var_temp_dn4)) * assign30_e758) + (assign30_e751 * (var_temp_dn4 - (p.p18 * var_temp_dn4)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn5 + (p.p18 * var_temp_dn5)) + ((((var_temp_dn5 - (p.p18 * var_temp_dn5)) * assign30_e758) + (assign30_e751 * (var_temp_dn5 - (p.p18 * var_temp_dn5)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn6 + (p.p18 * var_temp_dn6)) + ((((var_temp_dn6 - (p.p18 * var_temp_dn6)) * assign30_e758) + (assign30_e751 * (var_temp_dn6 - (p.p18 * var_temp_dn6)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn7 + (p.p18 * var_temp_dn7)) + ((((var_temp_dn7 - (p.p18 * var_temp_dn7)) * assign30_e758) + (assign30_e751 * (var_temp_dn7 - (p.p18 * var_temp_dn7)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn8 + (p.p18 * var_temp_dn8)) + ((((var_temp_dn8 - (p.p18 * var_temp_dn8)) * assign30_e758) + (assign30_e751 * (var_temp_dn8 - (p.p18 * var_temp_dn8)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_dn9 + (p.p18 * var_temp_dn9)) + ((((var_temp_dn9 - (p.p18 * var_temp_dn9)) * assign30_e758) + (assign30_e751 * (var_temp_dn9 - (p.p18 * var_temp_dn9)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_db0 + (p.p18 * var_temp_db0)) + ((((var_temp_db0 - (p.p18 * var_temp_db0)) * assign30_e758) + (assign30_e751 * (var_temp_db0 - (p.p18 * var_temp_db0)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_db1 + (p.p18 * var_temp_db1)) + ((((var_temp_db1 - (p.p18 * var_temp_db1)) * assign30_e758) + (assign30_e751 * (var_temp_db1 - (p.p18 * var_temp_db1)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_db2 + (p.p18 * var_temp_db2)) + ((((var_temp_db2 - (p.p18 * var_temp_db2)) * assign30_e758) + (assign30_e751 * (var_temp_db2 - (p.p18 * var_temp_db2)))) / (2.0 * assign30_e762)))), (0.5 * ((var_temp_db3 + (p.p18 * var_temp_db3)) + ((((var_temp_db3 - (p.p18 * var_temp_db3)) * assign30_e758) + (assign30_e751 * (var_temp_db3 - (p.p18 * var_temp_db3)))) / (2.0 * assign30_e762)))),)
    } else {
        (var_tkd, var_tkd_dn0, var_tkd_dn1, var_tkd_dn2, var_tkd_dn3, var_tkd_dn4, var_tkd_dn5, var_tkd_dn6, var_tkd_dn7, var_tkd_dn8, var_tkd_dn9, var_tkd_db0, var_tkd_db1, var_tkd_db2, var_tkd_db3,)
    }
};
        var_tkd = assign30_e766;
        var_tkd_dn0 = assign30_e766_d_n0;
        var_tkd_dn1 = assign30_e766_d_n1;
        var_tkd_dn2 = assign30_e766_d_n2;
        var_tkd_dn3 = assign30_e766_d_n3;
        var_tkd_dn4 = assign30_e766_d_n4;
        var_tkd_dn5 = assign30_e766_d_n5;
        var_tkd_dn6 = assign30_e766_d_n6;
        var_tkd_dn7 = assign30_e766_d_n7;
        var_tkd_dn8 = assign30_e766_d_n8;
        var_tkd_dn9 = assign30_e766_d_n9;
        var_tkd_db0 = assign30_e766_d_b0;
        var_tkd_db1 = assign30_e766_d_b1;
        var_tkd_db2 = assign30_e766_d_b2;
        var_tkd_db3 = assign30_e766_d_b3;

        let (assign40_e797, assign40_e797_d_n0, assign40_e797_d_n1, assign40_e797_d_n2, assign40_e797_d_n3, assign40_e797_d_n4, assign40_e797_d_n5, assign40_e797_d_n6, assign40_e797_d_n7, assign40_e797_d_n8, assign40_e797_d_n9, assign40_e797_d_b0, assign40_e797_d_b1, assign40_e797_d_b2, assign40_e797_d_b3,) = {
    if (var_guard1 != 0.0) {
        let assign40_e772: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e773: f64 = (10.0 / assign40_e772);
        let assign40_e775: f64 = (assign40_e773 + 600.0);
        let assign40_e779: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e780: f64 = (10.0 / assign40_e779);
        let assign40_e782: f64 = (assign40_e780 - 600.0);
        let assign40_e786: f64 = (var_tkd * 8.617332384961e-5);
        let assign40_e787: f64 = (10.0 / assign40_e786);
        let assign40_e789: f64 = (assign40_e787 - 600.0);
        let assign40_e790: f64 = (assign40_e782 * assign40_e789);
        let assign40_e792: f64 = (assign40_e790 + 0.01);
        let assign40_e793: f64 = (assign40_e792).sqrt();
        let assign40_e794: f64 = (assign40_e775 + assign40_e793);
        let assign40_e795: f64 = (0.5 * assign40_e794);
        (assign40_e795, (0.5 * ((-((10.0 * (var_tkd_dn0 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn0 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn0 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn1 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn1 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn1 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn2 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn2 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn2 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn3 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn3 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn3 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn4 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn5 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn5 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn5 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn6 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn7 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn8 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_dn9 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_db0 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_db0 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_db0 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_db1 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_db1 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_db1 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_db2 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_db2 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_db2 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))), (0.5 * ((-((10.0 * (var_tkd_db3 * 8.617332384961e-5)) / (assign40_e772 * assign40_e772))) + ((((-((10.0 * (var_tkd_db3 * 8.617332384961e-5)) / (assign40_e779 * assign40_e779))) * assign40_e789) + (assign40_e782 * (-((10.0 * (var_tkd_db3 * 8.617332384961e-5)) / (assign40_e786 * assign40_e786))))) / (2.0 * assign40_e793)))),)
    } else {
        (var_xsatmax, var_xsatmax_dn0, var_xsatmax_dn1, var_xsatmax_dn2, var_xsatmax_dn3, var_xsatmax_dn4, var_xsatmax_dn5, var_xsatmax_dn6, var_xsatmax_dn7, var_xsatmax_dn8, var_xsatmax_dn9, var_xsatmax_db0, var_xsatmax_db1, var_xsatmax_db2, var_xsatmax_db3,)
    }
};
        var_xsatmax = assign40_e797;
        var_xsatmax_dn0 = assign40_e797_d_n0;
        var_xsatmax_dn1 = assign40_e797_d_n1;
        var_xsatmax_dn2 = assign40_e797_d_n2;
        var_xsatmax_dn3 = assign40_e797_d_n3;
        var_xsatmax_dn4 = assign40_e797_d_n4;
        var_xsatmax_dn5 = assign40_e797_d_n5;
        var_xsatmax_dn6 = assign40_e797_d_n6;
        var_xsatmax_dn7 = assign40_e797_d_n7;
        var_xsatmax_dn8 = assign40_e797_d_n8;
        var_xsatmax_dn9 = assign40_e797_d_n9;
        var_xsatmax_db0 = assign40_e797_d_b0;
        var_xsatmax_db1 = assign40_e797_d_b1;
        var_xsatmax_db2 = assign40_e797_d_b2;
        var_xsatmax_db3 = assign40_e797_d_b3;

        let (assign50_e817, assign50_e817_d_n0, assign50_e817_d_n1, assign50_e817_d_n2, assign50_e817_d_n3, assign50_e817_d_n4, assign50_e817_d_n5, assign50_e817_d_n6, assign50_e817_d_n7, assign50_e817_d_n8, assign50_e817_d_n9, assign50_e817_d_b0, assign50_e817_d_b1, assign50_e817_d_b2, assign50_e817_d_b3,) = {
    if (var_guard1 == 0.0) {
        let assign50_e803: f64 = (var_temp + 1.0);
        let assign50_e806: f64 = (var_temp - 1.0);
        let assign50_e809: f64 = (var_temp - 1.0);
        let assign50_e810: f64 = (assign50_e806 * assign50_e809);
        let assign50_e812: f64 = (assign50_e810 + 0.001);
        let assign50_e813: f64 = (assign50_e812).sqrt();
        let assign50_e814: f64 = (assign50_e803 + assign50_e813);
        let assign50_e815: f64 = (0.5 * assign50_e814);
        (assign50_e815, (0.5 * (var_temp_dn0 + (((var_temp_dn0 * assign50_e809) + (assign50_e806 * var_temp_dn0)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn1 + (((var_temp_dn1 * assign50_e809) + (assign50_e806 * var_temp_dn1)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn2 + (((var_temp_dn2 * assign50_e809) + (assign50_e806 * var_temp_dn2)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn3 + (((var_temp_dn3 * assign50_e809) + (assign50_e806 * var_temp_dn3)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn4 + (((var_temp_dn4 * assign50_e809) + (assign50_e806 * var_temp_dn4)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn5 + (((var_temp_dn5 * assign50_e809) + (assign50_e806 * var_temp_dn5)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn6 + (((var_temp_dn6 * assign50_e809) + (assign50_e806 * var_temp_dn6)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn7 + (((var_temp_dn7 * assign50_e809) + (assign50_e806 * var_temp_dn7)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn8 + (((var_temp_dn8 * assign50_e809) + (assign50_e806 * var_temp_dn8)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_dn9 + (((var_temp_dn9 * assign50_e809) + (assign50_e806 * var_temp_dn9)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_db0 + (((var_temp_db0 * assign50_e809) + (assign50_e806 * var_temp_db0)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_db1 + (((var_temp_db1 * assign50_e809) + (assign50_e806 * var_temp_db1)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_db2 + (((var_temp_db2 * assign50_e809) + (assign50_e806 * var_temp_db2)) / (2.0 * assign50_e813)))), (0.5 * (var_temp_db3 + (((var_temp_db3 * assign50_e809) + (assign50_e806 * var_temp_db3)) / (2.0 * assign50_e813)))),)
    } else {
        (var_tkd, var_tkd_dn0, var_tkd_dn1, var_tkd_dn2, var_tkd_dn3, var_tkd_dn4, var_tkd_dn5, var_tkd_dn6, var_tkd_dn7, var_tkd_dn8, var_tkd_dn9, var_tkd_db0, var_tkd_db1, var_tkd_db2, var_tkd_db3,)
    }
};
        var_tkd = assign50_e817;
        var_tkd_dn0 = assign50_e817_d_n0;
        var_tkd_dn1 = assign50_e817_d_n1;
        var_tkd_dn2 = assign50_e817_d_n2;
        var_tkd_dn3 = assign50_e817_d_n3;
        var_tkd_dn4 = assign50_e817_d_n4;
        var_tkd_dn5 = assign50_e817_d_n5;
        var_tkd_dn6 = assign50_e817_d_n6;
        var_tkd_dn7 = assign50_e817_d_n7;
        var_tkd_dn8 = assign50_e817_d_n8;
        var_tkd_dn9 = assign50_e817_d_n9;
        var_tkd_db0 = assign50_e817_d_b0;
        var_tkd_db1 = assign50_e817_d_b1;
        var_tkd_db2 = assign50_e817_d_b2;
        var_tkd_db3 = assign50_e817_d_b3;

        let (assign60_e822, assign60_e822_d_n0, assign60_e822_d_n1, assign60_e822_d_n2, assign60_e822_d_n3, assign60_e822_d_n4, assign60_e822_d_n5, assign60_e822_d_n6, assign60_e822_d_n7, assign60_e822_d_n8, assign60_e822_d_n9, assign60_e822_d_b0, assign60_e822_d_b1, assign60_e822_d_b2, assign60_e822_d_b3,) = {
    if (var_guard1 == 0.0) {
        (600.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xsatmax, var_xsatmax_dn0, var_xsatmax_dn1, var_xsatmax_dn2, var_xsatmax_dn3, var_xsatmax_dn4, var_xsatmax_dn5, var_xsatmax_dn6, var_xsatmax_dn7, var_xsatmax_dn8, var_xsatmax_dn9, var_xsatmax_db0, var_xsatmax_db1, var_xsatmax_db2, var_xsatmax_db3,)
    }
};
        var_xsatmax = assign60_e822;
        var_xsatmax_dn0 = assign60_e822_d_n0;
        var_xsatmax_dn1 = assign60_e822_d_n1;
        var_xsatmax_dn2 = assign60_e822_d_n2;
        var_xsatmax_dn3 = assign60_e822_d_n3;
        var_xsatmax_dn4 = assign60_e822_d_n4;
        var_xsatmax_dn5 = assign60_e822_d_n5;
        var_xsatmax_dn6 = assign60_e822_d_n6;
        var_xsatmax_dn7 = assign60_e822_d_n7;
        var_xsatmax_dn8 = assign60_e822_d_n8;
        var_xsatmax_dn9 = assign60_e822_d_n9;
        var_xsatmax_db0 = assign60_e822_d_b0;
        var_xsatmax_db1 = assign60_e822_d_b1;
        var_xsatmax_db2 = assign60_e822_d_b2;
        var_xsatmax_db3 = assign60_e822_d_b3;

        let assign70_e837: f64 = if (((p.p0 == 0.0) && (p.p172 > 0.0)) || ((p.p0 > 0.0) && (p.p439 > 0.0))) { 1.0 } else { 0.0 };
        var_guard2 = assign70_e837;

        let (assign80_e841,) = {
    if (var_guard2 != 0.0) {
        (p.p5,)
    } else {
        (var_swshe_i,)
    }
};
        var_swshe_i = assign80_e841;

        let (assign90_e846,) = {
    if (var_guard2 == 0.0) {
        (0.0,)
    } else {
        (var_swshe_i,)
    }
};
        var_swshe_i = assign90_e846;

        var_dtc = 0.0;
        var_dtc_dn0 = 0.0;
        var_dtc_dn1 = 0.0;
        var_dtc_dn2 = 0.0;
        var_dtc_dn3 = 0.0;
        var_dtc_dn4 = 0.0;
        var_dtc_dn5 = 0.0;
        var_dtc_dn6 = 0.0;
        var_dtc_dn7 = 0.0;
        var_dtc_dn8 = 0.0;
        var_dtc_dn9 = 0.0;
        var_dtc_db0 = 0.0;
        var_dtc_db1 = 0.0;
        var_dtc_db2 = 0.0;
        var_dtc_db3 = 0.0;

        var_tkc = var_tkd;
        var_tkc_dn0 = var_tkd_dn0;
        var_tkc_dn1 = var_tkd_dn1;
        var_tkc_dn2 = var_tkd_dn2;
        var_tkc_dn3 = var_tkd_dn3;
        var_tkc_dn4 = var_tkd_dn4;
        var_tkc_dn5 = var_tkd_dn5;
        var_tkc_dn6 = var_tkd_dn6;
        var_tkc_dn7 = var_tkd_dn7;
        var_tkc_dn8 = var_tkd_dn8;
        var_tkc_dn9 = var_tkd_dn9;
        var_tkc_db0 = var_tkd_db0;
        var_tkc_db1 = var_tkd_db1;
        var_tkc_db2 = var_tkd_db2;
        var_tkc_db3 = var_tkd_db3;

        let assign140_e856: f64 = (var_tkc * var_tkc);
        var_tkc_sq = assign140_e856;
        var_tkc_sq_dn0 = ((var_tkc_dn0 * var_tkc) + (var_tkc * var_tkc_dn0));
        var_tkc_sq_dn1 = ((var_tkc_dn1 * var_tkc) + (var_tkc * var_tkc_dn1));
        var_tkc_sq_dn2 = ((var_tkc_dn2 * var_tkc) + (var_tkc * var_tkc_dn2));
        var_tkc_sq_dn3 = ((var_tkc_dn3 * var_tkc) + (var_tkc * var_tkc_dn3));
        var_tkc_sq_dn4 = ((var_tkc_dn4 * var_tkc) + (var_tkc * var_tkc_dn4));
        var_tkc_sq_dn5 = ((var_tkc_dn5 * var_tkc) + (var_tkc * var_tkc_dn5));
        var_tkc_sq_dn6 = ((var_tkc_dn6 * var_tkc) + (var_tkc * var_tkc_dn6));
        var_tkc_sq_dn7 = ((var_tkc_dn7 * var_tkc) + (var_tkc * var_tkc_dn7));
        var_tkc_sq_dn8 = ((var_tkc_dn8 * var_tkc) + (var_tkc * var_tkc_dn8));
        var_tkc_sq_dn9 = ((var_tkc_dn9 * var_tkc) + (var_tkc * var_tkc_dn9));
        var_tkc_sq_db0 = ((var_tkc_db0 * var_tkc) + (var_tkc * var_tkc_db0));
        var_tkc_sq_db1 = ((var_tkc_db1 * var_tkc) + (var_tkc * var_tkc_db1));
        var_tkc_sq_db2 = ((var_tkc_db2 * var_tkc) + (var_tkc * var_tkc_db2));
        var_tkc_sq_db3 = ((var_tkc_db3 * var_tkc) + (var_tkc * var_tkc_db3));

        let assign150_e859: f64 = (var_tkc - var_tkr);
        var_dt = assign150_e859;
        var_dt_dn0 = var_tkc_dn0;
        var_dt_dn1 = var_tkc_dn1;
        var_dt_dn2 = var_tkc_dn2;
        var_dt_dn3 = var_tkc_dn3;
        var_dt_dn4 = var_tkc_dn4;
        var_dt_dn5 = var_tkc_dn5;
        var_dt_dn6 = var_tkc_dn6;
        var_dt_dn7 = var_tkc_dn7;
        var_dt_dn8 = var_tkc_dn8;
        var_dt_dn9 = var_tkc_dn9;
        var_dt_db0 = var_tkc_db0;
        var_dt_db1 = var_tkc_db1;
        var_dt_db2 = var_tkc_db2;
        var_dt_db3 = var_tkc_db3;

        let assign160_e862: f64 = (var_tkc / var_tkr);
        var_rt = assign160_e862;
        var_rt_dn0 = (var_tkc_dn0 / var_tkr);
        var_rt_dn1 = (var_tkc_dn1 / var_tkr);
        var_rt_dn2 = (var_tkc_dn2 / var_tkr);
        var_rt_dn3 = (var_tkc_dn3 / var_tkr);
        var_rt_dn4 = (var_tkc_dn4 / var_tkr);
        var_rt_dn5 = (var_tkc_dn5 / var_tkr);
        var_rt_dn6 = (var_tkc_dn6 / var_tkr);
        var_rt_dn7 = (var_tkc_dn7 / var_tkr);
        var_rt_dn8 = (var_tkc_dn8 / var_tkr);
        var_rt_dn9 = (var_tkc_dn9 / var_tkr);
        var_rt_db0 = (var_tkc_db0 / var_tkr);
        var_rt_db1 = (var_tkc_db1 / var_tkr);
        var_rt_db2 = (var_tkc_db2 / var_tkr);
        var_rt_db3 = (var_tkc_db3 / var_tkr);

        let assign170_e865: f64 = (var_tkr / var_tkc);
        var_rtn = assign170_e865;
        var_rtn_dn0 = (-((var_tkr * var_tkc_dn0) / (var_tkc * var_tkc)));
        var_rtn_dn1 = (-((var_tkr * var_tkc_dn1) / (var_tkc * var_tkc)));
        var_rtn_dn2 = (-((var_tkr * var_tkc_dn2) / (var_tkc * var_tkc)));
        var_rtn_dn3 = (-((var_tkr * var_tkc_dn3) / (var_tkc * var_tkc)));
        var_rtn_dn4 = (-((var_tkr * var_tkc_dn4) / (var_tkc * var_tkc)));
        var_rtn_dn5 = (-((var_tkr * var_tkc_dn5) / (var_tkc * var_tkc)));
        var_rtn_dn6 = (-((var_tkr * var_tkc_dn6) / (var_tkc * var_tkc)));
        var_rtn_dn7 = (-((var_tkr * var_tkc_dn7) / (var_tkc * var_tkc)));
        var_rtn_dn8 = (-((var_tkr * var_tkc_dn8) / (var_tkc * var_tkc)));
        var_rtn_dn9 = (-((var_tkr * var_tkc_dn9) / (var_tkc * var_tkc)));
        var_rtn_db0 = (-((var_tkr * var_tkc_db0) / (var_tkc * var_tkc)));
        var_rtn_db1 = (-((var_tkr * var_tkc_db1) / (var_tkc * var_tkc)));
        var_rtn_db2 = (-((var_tkr * var_tkc_db2) / (var_tkc * var_tkc)));
        var_rtn_db3 = (-((var_tkr * var_tkc_db3) / (var_tkc * var_tkc)));

        let assign180_e868: f64 = (var_tkc * 8.617332384961e-5);
        var_phit0 = assign180_e868;
        var_phit0_dn0 = (var_tkc_dn0 * 8.617332384961e-5);
        var_phit0_dn1 = (var_tkc_dn1 * 8.617332384961e-5);
        var_phit0_dn2 = (var_tkc_dn2 * 8.617332384961e-5);
        var_phit0_dn3 = (var_tkc_dn3 * 8.617332384961e-5);
        var_phit0_dn4 = (var_tkc_dn4 * 8.617332384961e-5);
        var_phit0_dn5 = (var_tkc_dn5 * 8.617332384961e-5);
        var_phit0_dn6 = (var_tkc_dn6 * 8.617332384961e-5);
        var_phit0_dn7 = (var_tkc_dn7 * 8.617332384961e-5);
        var_phit0_dn8 = (var_tkc_dn8 * 8.617332384961e-5);
        var_phit0_dn9 = (var_tkc_dn9 * 8.617332384961e-5);
        var_phit0_db0 = (var_tkc_db0 * 8.617332384961e-5);
        var_phit0_db1 = (var_tkc_db1 * 8.617332384961e-5);
        var_phit0_db2 = (var_tkc_db2 * 8.617332384961e-5);
        var_phit0_db3 = (var_tkc_db3 * 8.617332384961e-5);

        let assign190_e871: f64 = (1.0 / var_phit0);
        var_inv_phit0 = assign190_e871;
        var_inv_phit0_dn0 = (-(var_phit0_dn0 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn1 = (-(var_phit0_dn1 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn2 = (-(var_phit0_dn2 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn3 = (-(var_phit0_dn3 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn4 = (-(var_phit0_dn4 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn5 = (-(var_phit0_dn5 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn6 = (-(var_phit0_dn6 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn7 = (-(var_phit0_dn7 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn8 = (-(var_phit0_dn8 / (var_phit0 * var_phit0)));
        var_inv_phit0_dn9 = (-(var_phit0_dn9 / (var_phit0 * var_phit0)));
        var_inv_phit0_db0 = (-(var_phit0_db0 / (var_phit0 * var_phit0)));
        var_inv_phit0_db1 = (-(var_phit0_db1 / (var_phit0 * var_phit0)));
        var_inv_phit0_db2 = (-(var_phit0_db2 / (var_phit0 * var_phit0)));
        var_inv_phit0_db3 = (-(var_phit0_db3 / (var_phit0 * var_phit0)));

        let assign200_e874: f64 = if p.p0 == 0.0 { 1.0 } else { 0.0 };
        var_guard83 = assign200_e874;

        let (assign210_e878,) = {
    if (var_guard83 != 0.0) {
        (p.p23,)
    } else {
        (var_adrain_i,)
    }
};
        var_adrain_i = assign210_e878;

        let (assign220_e882,) = {
    if (var_guard83 != 0.0) {
        (p.p22,)
    } else {
        (var_asource_i,)
    }
};
        var_asource_i = assign220_e882;

        let (assign230_e886,) = {
    if (var_guard83 != 0.0) {
        (p.p25,)
    } else {
        (var_pdrain_i,)
    }
};
        var_pdrain_i = assign230_e886;

        let (assign240_e890,) = {
    if (var_guard83 != 0.0) {
        (p.p24,)
    } else {
        (var_psource_i,)
    }
};
        var_psource_i = assign240_e890;

        let (assign250_e894,) = {
    if (var_guard83 != 0.0) {
        (p.p30,)
    } else {
        (var_mult_i_int,)
    }
};
        var_mult_i_int = assign250_e894;

        let (assign260_e898,) = {
    if (var_guard83 != 0.0) {
        (p.p41,)
    } else {
        (var_tox1_i,)
    }
};
        var_tox1_i = assign260_e898;

        let (assign270_e902,) = {
    if (var_guard83 != 0.0) {
        (p.p42,)
    } else {
        (var_tsi_i,)
    }
};
        var_tsi_i = assign270_e902;

        let (assign280_e906,) = {
    if (var_guard83 != 0.0) {
        (p.p43,)
    } else {
        (var_xge_i,)
    }
};
        var_xge_i = assign280_e906;

        let (assign290_e910,) = {
    if (var_guard83 != 0.0) {
        (p.p44,)
    } else {
        (var_tox2_i,)
    }
};
        var_tox2_i = assign290_e910;

        let (assign300_e914,) = {
    if (var_guard83 != 0.0) {
        (1.0,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign300_e914;

        let assign310_e917: f64 = if p.p45 < 0.0 { 1.0 } else { 0.0 };
        var_guard84 = assign310_e917;

        let (assign320_e924,) = {
    if ((var_guard83 != 0.0) && (var_guard84 != 0.0)) {
        let assign320_e922: f64 = (-1.0);
        (assign320_e922,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign320_e924;

        let (assign330_e933,) = {
    if (var_guard83 != 0.0) {
        let assign330_e927: f64 = (p.p45).abs();
        let assign330_e929: f64 = (assign330_e927).min(1e19);
        let assign330_e931: f64 = (assign330_e929 * 1000000.0);
        (assign330_e931,)
    } else {
        (var_nch_i,)
    }
};
        var_nch_i = assign330_e933;

        let (assign340_e937,) = {
    if (var_guard83 != 0.0) {
        (1.0,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign340_e937;

        let assign350_e940: f64 = if p.p46 < 0.0 { 1.0 } else { 0.0 };
        var_guard85 = assign350_e940;

        let (assign360_e947,) = {
    if ((var_guard83 != 0.0) && (var_guard85 != 0.0)) {
        let assign360_e945: f64 = (-1.0);
        (assign360_e945,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign360_e947;

        let (assign370_e958,) = {
    if (var_guard83 != 0.0) {
        let assign370_e950: f64 = (p.p46).abs();
        let assign370_e952: f64 = (assign370_e950).max(1e16);
        let assign370_e954: f64 = (assign370_e952).min(1e21);
        let assign370_e956: f64 = (assign370_e954 * 1000000.0);
        (assign370_e956,)
    } else {
        (var_nsub_i,)
    }
};
        var_nsub_i = assign370_e958;

        let (assign380_e962,) = {
    if (var_guard83 != 0.0) {
        (p.p47,)
    } else {
        (var_ct_i,)
    }
};
        var_ct_i = assign380_e962;

        let (assign390_e966,) = {
    if (var_guard83 != 0.0) {
        (p.p48,)
    } else {
        (var_toxp_i,)
    }
};
        var_toxp_i = assign390_e966;

        let (assign400_e972,) = {
    if (var_guard83 != 0.0) {
        let assign400_e970: f64 = (p.p49 * 1000000.0);
        (assign400_e970,)
    } else {
        (var_nov_i,)
    }
};
        var_nov_i = assign400_e972;

        let (assign410_e978,) = {
    if (var_guard83 != 0.0) {
        let assign410_e976: f64 = (p.p50 * 1000000.0);
        (assign410_e976,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign410_e978;

        *var_adrain_i_slot = var_adrain_i;
        *var_asource_i_slot = var_asource_i;
        *var_ct_i_slot = var_ct_i;
        *var_dt_slot = var_dt;
        *var_dt_db0_slot = var_dt_db0;
        *var_dt_db1_slot = var_dt_db1;
        *var_dt_db2_slot = var_dt_db2;
        *var_dt_db3_slot = var_dt_db3;
        *var_dt_dn0_slot = var_dt_dn0;
        *var_dt_dn1_slot = var_dt_dn1;
        *var_dt_dn2_slot = var_dt_dn2;
        *var_dt_dn3_slot = var_dt_dn3;
        *var_dt_dn4_slot = var_dt_dn4;
        *var_dt_dn5_slot = var_dt_dn5;
        *var_dt_dn6_slot = var_dt_dn6;
        *var_dt_dn7_slot = var_dt_dn7;
        *var_dt_dn8_slot = var_dt_dn8;
        *var_dt_dn9_slot = var_dt_dn9;
        *var_dtc_slot = var_dtc;
        *var_dtc_db0_slot = var_dtc_db0;
        *var_dtc_db1_slot = var_dtc_db1;
        *var_dtc_db2_slot = var_dtc_db2;
        *var_dtc_db3_slot = var_dtc_db3;
        *var_dtc_dn0_slot = var_dtc_dn0;
        *var_dtc_dn1_slot = var_dtc_dn1;
        *var_dtc_dn2_slot = var_dtc_dn2;
        *var_dtc_dn3_slot = var_dtc_dn3;
        *var_dtc_dn4_slot = var_dtc_dn4;
        *var_dtc_dn5_slot = var_dtc_dn5;
        *var_dtc_dn6_slot = var_dtc_dn6;
        *var_dtc_dn7_slot = var_dtc_dn7;
        *var_dtc_dn8_slot = var_dtc_dn8;
        *var_dtc_dn9_slot = var_dtc_dn9;
        *var_guard1_slot = var_guard1;
        *var_guard2_slot = var_guard2;
        *var_guard83_slot = var_guard83;
        *var_guard84_slot = var_guard84;
        *var_guard85_slot = var_guard85;
        *var_inv_phit0_slot = var_inv_phit0;
        *var_inv_phit0_db0_slot = var_inv_phit0_db0;
        *var_inv_phit0_db1_slot = var_inv_phit0_db1;
        *var_inv_phit0_db2_slot = var_inv_phit0_db2;
        *var_inv_phit0_db3_slot = var_inv_phit0_db3;
        *var_inv_phit0_dn0_slot = var_inv_phit0_dn0;
        *var_inv_phit0_dn1_slot = var_inv_phit0_dn1;
        *var_inv_phit0_dn2_slot = var_inv_phit0_dn2;
        *var_inv_phit0_dn3_slot = var_inv_phit0_dn3;
        *var_inv_phit0_dn4_slot = var_inv_phit0_dn4;
        *var_inv_phit0_dn5_slot = var_inv_phit0_dn5;
        *var_inv_phit0_dn6_slot = var_inv_phit0_dn6;
        *var_inv_phit0_dn7_slot = var_inv_phit0_dn7;
        *var_inv_phit0_dn8_slot = var_inv_phit0_dn8;
        *var_inv_phit0_dn9_slot = var_inv_phit0_dn9;
        *var_mult_i_int_slot = var_mult_i_int;
        *var_nch_i_slot = var_nch_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_nsub_i_slot = var_nsub_i;
        *var_pdrain_i_slot = var_pdrain_i;
        *var_phit0_slot = var_phit0;
        *var_phit0_db0_slot = var_phit0_db0;
        *var_phit0_db1_slot = var_phit0_db1;
        *var_phit0_db2_slot = var_phit0_db2;
        *var_phit0_db3_slot = var_phit0_db3;
        *var_phit0_dn0_slot = var_phit0_dn0;
        *var_phit0_dn1_slot = var_phit0_dn1;
        *var_phit0_dn2_slot = var_phit0_dn2;
        *var_phit0_dn3_slot = var_phit0_dn3;
        *var_phit0_dn4_slot = var_phit0_dn4;
        *var_phit0_dn5_slot = var_phit0_dn5;
        *var_phit0_dn6_slot = var_phit0_dn6;
        *var_phit0_dn7_slot = var_phit0_dn7;
        *var_phit0_dn8_slot = var_phit0_dn8;
        *var_phit0_dn9_slot = var_phit0_dn9;
        *var_psource_i_slot = var_psource_i;
        *var_rt_slot = var_rt;
        *var_rt_db0_slot = var_rt_db0;
        *var_rt_db1_slot = var_rt_db1;
        *var_rt_db2_slot = var_rt_db2;
        *var_rt_db3_slot = var_rt_db3;
        *var_rt_dn0_slot = var_rt_dn0;
        *var_rt_dn1_slot = var_rt_dn1;
        *var_rt_dn2_slot = var_rt_dn2;
        *var_rt_dn3_slot = var_rt_dn3;
        *var_rt_dn4_slot = var_rt_dn4;
        *var_rt_dn5_slot = var_rt_dn5;
        *var_rt_dn6_slot = var_rt_dn6;
        *var_rt_dn7_slot = var_rt_dn7;
        *var_rt_dn8_slot = var_rt_dn8;
        *var_rt_dn9_slot = var_rt_dn9;
        *var_rtn_slot = var_rtn;
        *var_rtn_db0_slot = var_rtn_db0;
        *var_rtn_db1_slot = var_rtn_db1;
        *var_rtn_db2_slot = var_rtn_db2;
        *var_rtn_db3_slot = var_rtn_db3;
        *var_rtn_dn0_slot = var_rtn_dn0;
        *var_rtn_dn1_slot = var_rtn_dn1;
        *var_rtn_dn2_slot = var_rtn_dn2;
        *var_rtn_dn3_slot = var_rtn_dn3;
        *var_rtn_dn4_slot = var_rtn_dn4;
        *var_rtn_dn5_slot = var_rtn_dn5;
        *var_rtn_dn6_slot = var_rtn_dn6;
        *var_rtn_dn7_slot = var_rtn_dn7;
        *var_rtn_dn8_slot = var_rtn_dn8;
        *var_rtn_dn9_slot = var_rtn_dn9;
        *var_swshe_i_slot = var_swshe_i;
        *var_temp_slot = var_temp;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_tkc_slot = var_tkc;
        *var_tkc_db0_slot = var_tkc_db0;
        *var_tkc_db1_slot = var_tkc_db1;
        *var_tkc_db2_slot = var_tkc_db2;
        *var_tkc_db3_slot = var_tkc_db3;
        *var_tkc_dn0_slot = var_tkc_dn0;
        *var_tkc_dn1_slot = var_tkc_dn1;
        *var_tkc_dn2_slot = var_tkc_dn2;
        *var_tkc_dn3_slot = var_tkc_dn3;
        *var_tkc_dn4_slot = var_tkc_dn4;
        *var_tkc_dn5_slot = var_tkc_dn5;
        *var_tkc_dn6_slot = var_tkc_dn6;
        *var_tkc_dn7_slot = var_tkc_dn7;
        *var_tkc_dn8_slot = var_tkc_dn8;
        *var_tkc_dn9_slot = var_tkc_dn9;
        *var_tkc_sq_slot = var_tkc_sq;
        *var_tkc_sq_db0_slot = var_tkc_sq_db0;
        *var_tkc_sq_db1_slot = var_tkc_sq_db1;
        *var_tkc_sq_db2_slot = var_tkc_sq_db2;
        *var_tkc_sq_db3_slot = var_tkc_sq_db3;
        *var_tkc_sq_dn0_slot = var_tkc_sq_dn0;
        *var_tkc_sq_dn1_slot = var_tkc_sq_dn1;
        *var_tkc_sq_dn2_slot = var_tkc_sq_dn2;
        *var_tkc_sq_dn3_slot = var_tkc_sq_dn3;
        *var_tkc_sq_dn4_slot = var_tkc_sq_dn4;
        *var_tkc_sq_dn5_slot = var_tkc_sq_dn5;
        *var_tkc_sq_dn6_slot = var_tkc_sq_dn6;
        *var_tkc_sq_dn7_slot = var_tkc_sq_dn7;
        *var_tkc_sq_dn8_slot = var_tkc_sq_dn8;
        *var_tkc_sq_dn9_slot = var_tkc_sq_dn9;
        *var_tkd_slot = var_tkd;
        *var_tkd_db0_slot = var_tkd_db0;
        *var_tkd_db1_slot = var_tkd_db1;
        *var_tkd_db2_slot = var_tkd_db2;
        *var_tkd_db3_slot = var_tkd_db3;
        *var_tkd_dn0_slot = var_tkd_dn0;
        *var_tkd_dn1_slot = var_tkd_dn1;
        *var_tkd_dn2_slot = var_tkd_dn2;
        *var_tkd_dn3_slot = var_tkd_dn3;
        *var_tkd_dn4_slot = var_tkd_dn4;
        *var_tkd_dn5_slot = var_tkd_dn5;
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
        *var_xge_i_slot = var_xge_i;
        *var_xsatmax_slot = var_xsatmax;
        *var_xsatmax_db0_slot = var_xsatmax_db0;
        *var_xsatmax_db1_slot = var_xsatmax_db1;
        *var_xsatmax_db2_slot = var_xsatmax_db2;
        *var_xsatmax_db3_slot = var_xsatmax_db3;
        *var_xsatmax_dn0_slot = var_xsatmax_dn0;
        *var_xsatmax_dn1_slot = var_xsatmax_dn1;
        *var_xsatmax_dn2_slot = var_xsatmax_dn2;
        *var_xsatmax_dn3_slot = var_xsatmax_dn3;
        *var_xsatmax_dn4_slot = var_xsatmax_dn4;
        *var_xsatmax_dn5_slot = var_xsatmax_dn5;
        *var_xsatmax_dn6_slot = var_xsatmax_dn6;
        *var_xsatmax_dn7_slot = var_xsatmax_dn7;
        *var_xsatmax_dn8_slot = var_xsatmax_dn8;
        *var_xsatmax_dn9_slot = var_xsatmax_dn9;
    }

    pub(super) fn stamp_transient_block_1(
        p: &Parameters,
        var_guard83: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_db0_slot: &mut f64,
        var_betn1_t_db1_slot: &mut f64,
        var_betn1_t_db2_slot: &mut f64,
        var_betn1_t_db3_slot: &mut f64,
        var_betn1_t_dn0_slot: &mut f64,
        var_betn1_t_dn1_slot: &mut f64,
        var_betn1_t_dn2_slot: &mut f64,
        var_betn1_t_dn3_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn5_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_db0_slot: &mut f64,
        var_betn2_t_db1_slot: &mut f64,
        var_betn2_t_db2_slot: &mut f64,
        var_betn2_t_db3_slot: &mut f64,
        var_betn2_t_dn0_slot: &mut f64,
        var_betn2_t_dn1_slot: &mut f64,
        var_betn2_t_dn2_slot: &mut f64,
        var_betn2_t_dn3_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn5_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_db0_slot: &mut f64,
        var_cf1_t_db1_slot: &mut f64,
        var_cf1_t_db2_slot: &mut f64,
        var_cf1_t_db3_slot: &mut f64,
        var_cf1_t_dn0_slot: &mut f64,
        var_cf1_t_dn1_slot: &mut f64,
        var_cf1_t_dn2_slot: &mut f64,
        var_cf1_t_dn3_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn5_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_db0_slot: &mut f64,
        var_cf2_t_db1_slot: &mut f64,
        var_cf2_t_db2_slot: &mut f64,
        var_cf2_t_db3_slot: &mut f64,
        var_cf2_t_dn0_slot: &mut f64,
        var_cf2_t_dn1_slot: &mut f64,
        var_cf2_t_dn2_slot: &mut f64,
        var_cf2_t_dn3_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn5_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfdl_i_slot: &mut f64,
        var_cfdlb_i_slot: &mut f64,
        var_cic1_i_slot: &mut f64,
        var_cic2_i_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_csbi_i_slot: &mut f64,
        var_csfi_i_slot: &mut f64,
        var_csthr_i_slot: &mut f64,
        var_csthrb_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_db0_slot: &mut f64,
        var_np_i_db1_slot: &mut f64,
        var_np_i_db2_slot: &mut f64,
        var_np_i_db3_slot: &mut f64,
        var_np_i_dn0_slot: &mut f64,
        var_np_i_dn1_slot: &mut f64,
        var_np_i_dn2_slot: &mut f64,
        var_np_i_dn3_slot: &mut f64,
        var_np_i_dn4_slot: &mut f64,
        var_np_i_dn5_slot: &mut f64,
        var_np_i_dn6_slot: &mut f64,
        var_np_i_dn7_slot: &mut f64,
        var_np_i_dn8_slot: &mut f64,
        var_np_i_dn9_slot: &mut f64,
        var_nsddc_i_slot: &mut f64,
        var_pnce_i_slot: &mut f64,
        var_psce1_i_slot: &mut f64,
        var_psce2_i_slot: &mut f64,
        var_pscedlb_i_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsig_i_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stcf_i_slot: &mut f64,
        var_stcf_i_db0_slot: &mut f64,
        var_stcf_i_db1_slot: &mut f64,
        var_stcf_i_db2_slot: &mut f64,
        var_stcf_i_db3_slot: &mut f64,
        var_stcf_i_dn0_slot: &mut f64,
        var_stcf_i_dn1_slot: &mut f64,
        var_stcf_i_dn2_slot: &mut f64,
        var_stcf_i_dn3_slot: &mut f64,
        var_stcf_i_dn4_slot: &mut f64,
        var_stcf_i_dn5_slot: &mut f64,
        var_stcf_i_dn6_slot: &mut f64,
        var_stcf_i_dn7_slot: &mut f64,
        var_stcf_i_dn8_slot: &mut f64,
        var_stcf_i_dn9_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_thersg_i_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_db0_slot: &mut f64,
        var_vfb1_t_db1_slot: &mut f64,
        var_vfb1_t_db2_slot: &mut f64,
        var_vfb1_t_db3_slot: &mut f64,
        var_vfb1_t_dn0_slot: &mut f64,
        var_vfb1_t_dn1_slot: &mut f64,
        var_vfb1_t_dn2_slot: &mut f64,
        var_vfb1_t_dn3_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn5_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_db0_slot: &mut f64,
        var_vfb2_t_db1_slot: &mut f64,
        var_vfb2_t_db2_slot: &mut f64,
        var_vfb2_t_db3_slot: &mut f64,
        var_vfb2_t_dn0_slot: &mut f64,
        var_vfb2_t_dn1_slot: &mut f64,
        var_vfb2_t_dn2_slot: &mut f64,
        var_vfb2_t_dn3_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn5_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcorb_i_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_db0: f64 = *var_betn1_t_db0_slot;
        let mut var_betn1_t_db1: f64 = *var_betn1_t_db1_slot;
        let mut var_betn1_t_db2: f64 = *var_betn1_t_db2_slot;
        let mut var_betn1_t_db3: f64 = *var_betn1_t_db3_slot;
        let mut var_betn1_t_dn0: f64 = *var_betn1_t_dn0_slot;
        let mut var_betn1_t_dn1: f64 = *var_betn1_t_dn1_slot;
        let mut var_betn1_t_dn2: f64 = *var_betn1_t_dn2_slot;
        let mut var_betn1_t_dn3: f64 = *var_betn1_t_dn3_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn5: f64 = *var_betn1_t_dn5_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_db0: f64 = *var_betn2_t_db0_slot;
        let mut var_betn2_t_db1: f64 = *var_betn2_t_db1_slot;
        let mut var_betn2_t_db2: f64 = *var_betn2_t_db2_slot;
        let mut var_betn2_t_db3: f64 = *var_betn2_t_db3_slot;
        let mut var_betn2_t_dn0: f64 = *var_betn2_t_dn0_slot;
        let mut var_betn2_t_dn1: f64 = *var_betn2_t_dn1_slot;
        let mut var_betn2_t_dn2: f64 = *var_betn2_t_dn2_slot;
        let mut var_betn2_t_dn3: f64 = *var_betn2_t_dn3_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn5: f64 = *var_betn2_t_dn5_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_db0: f64 = *var_cf1_t_db0_slot;
        let mut var_cf1_t_db1: f64 = *var_cf1_t_db1_slot;
        let mut var_cf1_t_db2: f64 = *var_cf1_t_db2_slot;
        let mut var_cf1_t_db3: f64 = *var_cf1_t_db3_slot;
        let mut var_cf1_t_dn0: f64 = *var_cf1_t_dn0_slot;
        let mut var_cf1_t_dn1: f64 = *var_cf1_t_dn1_slot;
        let mut var_cf1_t_dn2: f64 = *var_cf1_t_dn2_slot;
        let mut var_cf1_t_dn3: f64 = *var_cf1_t_dn3_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn5: f64 = *var_cf1_t_dn5_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_db0: f64 = *var_cf2_t_db0_slot;
        let mut var_cf2_t_db1: f64 = *var_cf2_t_db1_slot;
        let mut var_cf2_t_db2: f64 = *var_cf2_t_db2_slot;
        let mut var_cf2_t_db3: f64 = *var_cf2_t_db3_slot;
        let mut var_cf2_t_dn0: f64 = *var_cf2_t_dn0_slot;
        let mut var_cf2_t_dn1: f64 = *var_cf2_t_dn1_slot;
        let mut var_cf2_t_dn2: f64 = *var_cf2_t_dn2_slot;
        let mut var_cf2_t_dn3: f64 = *var_cf2_t_dn3_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn5: f64 = *var_cf2_t_dn5_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfdl_i: f64 = *var_cfdl_i_slot;
        let mut var_cfdlb_i: f64 = *var_cfdlb_i_slot;
        let mut var_cic1_i: f64 = *var_cic1_i_slot;
        let mut var_cic2_i: f64 = *var_cic2_i_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_csbi_i: f64 = *var_csbi_i_slot;
        let mut var_csfi_i: f64 = *var_csfi_i_slot;
        let mut var_csthr_i: f64 = *var_csthr_i_slot;
        let mut var_csthrb_i: f64 = *var_csthrb_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_db0: f64 = *var_np_i_db0_slot;
        let mut var_np_i_db1: f64 = *var_np_i_db1_slot;
        let mut var_np_i_db2: f64 = *var_np_i_db2_slot;
        let mut var_np_i_db3: f64 = *var_np_i_db3_slot;
        let mut var_np_i_dn0: f64 = *var_np_i_dn0_slot;
        let mut var_np_i_dn1: f64 = *var_np_i_dn1_slot;
        let mut var_np_i_dn2: f64 = *var_np_i_dn2_slot;
        let mut var_np_i_dn3: f64 = *var_np_i_dn3_slot;
        let mut var_np_i_dn4: f64 = *var_np_i_dn4_slot;
        let mut var_np_i_dn5: f64 = *var_np_i_dn5_slot;
        let mut var_np_i_dn6: f64 = *var_np_i_dn6_slot;
        let mut var_np_i_dn7: f64 = *var_np_i_dn7_slot;
        let mut var_np_i_dn8: f64 = *var_np_i_dn8_slot;
        let mut var_np_i_dn9: f64 = *var_np_i_dn9_slot;
        let mut var_nsddc_i: f64 = *var_nsddc_i_slot;
        let mut var_pnce_i: f64 = *var_pnce_i_slot;
        let mut var_psce1_i: f64 = *var_psce1_i_slot;
        let mut var_psce2_i: f64 = *var_psce2_i_slot;
        let mut var_pscedlb_i: f64 = *var_pscedlb_i_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsig_i: f64 = *var_rsig_i_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stcf_i: f64 = *var_stcf_i_slot;
        let mut var_stcf_i_db0: f64 = *var_stcf_i_db0_slot;
        let mut var_stcf_i_db1: f64 = *var_stcf_i_db1_slot;
        let mut var_stcf_i_db2: f64 = *var_stcf_i_db2_slot;
        let mut var_stcf_i_db3: f64 = *var_stcf_i_db3_slot;
        let mut var_stcf_i_dn0: f64 = *var_stcf_i_dn0_slot;
        let mut var_stcf_i_dn1: f64 = *var_stcf_i_dn1_slot;
        let mut var_stcf_i_dn2: f64 = *var_stcf_i_dn2_slot;
        let mut var_stcf_i_dn3: f64 = *var_stcf_i_dn3_slot;
        let mut var_stcf_i_dn4: f64 = *var_stcf_i_dn4_slot;
        let mut var_stcf_i_dn5: f64 = *var_stcf_i_dn5_slot;
        let mut var_stcf_i_dn6: f64 = *var_stcf_i_dn6_slot;
        let mut var_stcf_i_dn7: f64 = *var_stcf_i_dn7_slot;
        let mut var_stcf_i_dn8: f64 = *var_stcf_i_dn8_slot;
        let mut var_stcf_i_dn9: f64 = *var_stcf_i_dn9_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_thersg_i: f64 = *var_thersg_i_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_db0: f64 = *var_vfb1_t_db0_slot;
        let mut var_vfb1_t_db1: f64 = *var_vfb1_t_db1_slot;
        let mut var_vfb1_t_db2: f64 = *var_vfb1_t_db2_slot;
        let mut var_vfb1_t_db3: f64 = *var_vfb1_t_db3_slot;
        let mut var_vfb1_t_dn0: f64 = *var_vfb1_t_dn0_slot;
        let mut var_vfb1_t_dn1: f64 = *var_vfb1_t_dn1_slot;
        let mut var_vfb1_t_dn2: f64 = *var_vfb1_t_dn2_slot;
        let mut var_vfb1_t_dn3: f64 = *var_vfb1_t_dn3_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn5: f64 = *var_vfb1_t_dn5_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_db0: f64 = *var_vfb2_t_db0_slot;
        let mut var_vfb2_t_db1: f64 = *var_vfb2_t_db1_slot;
        let mut var_vfb2_t_db2: f64 = *var_vfb2_t_db2_slot;
        let mut var_vfb2_t_db3: f64 = *var_vfb2_t_db3_slot;
        let mut var_vfb2_t_dn0: f64 = *var_vfb2_t_dn0_slot;
        let mut var_vfb2_t_dn1: f64 = *var_vfb2_t_dn1_slot;
        let mut var_vfb2_t_dn2: f64 = *var_vfb2_t_dn2_slot;
        let mut var_vfb2_t_dn3: f64 = *var_vfb2_t_dn3_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn5: f64 = *var_vfb2_t_dn5_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcorb_i: f64 = *var_xcorb_i_slot;

        let (assign420_e982, assign420_e982_d_n0, assign420_e982_d_n1, assign420_e982_d_n2, assign420_e982_d_n3, assign420_e982_d_n4, assign420_e982_d_n5, assign420_e982_d_n6, assign420_e982_d_n7, assign420_e982_d_n8, assign420_e982_d_n9, assign420_e982_d_b0, assign420_e982_d_b1, assign420_e982_d_b2, assign420_e982_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb1_t, var_vfb1_t_dn0, var_vfb1_t_dn1, var_vfb1_t_dn2, var_vfb1_t_dn3, var_vfb1_t_dn4, var_vfb1_t_dn5, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9, var_vfb1_t_db0, var_vfb1_t_db1, var_vfb1_t_db2, var_vfb1_t_db3,)
    }
};
        var_vfb1_t = assign420_e982;
        var_vfb1_t_dn0 = assign420_e982_d_n0;
        var_vfb1_t_dn1 = assign420_e982_d_n1;
        var_vfb1_t_dn2 = assign420_e982_d_n2;
        var_vfb1_t_dn3 = assign420_e982_d_n3;
        var_vfb1_t_dn4 = assign420_e982_d_n4;
        var_vfb1_t_dn5 = assign420_e982_d_n5;
        var_vfb1_t_dn6 = assign420_e982_d_n6;
        var_vfb1_t_dn7 = assign420_e982_d_n7;
        var_vfb1_t_dn8 = assign420_e982_d_n8;
        var_vfb1_t_dn9 = assign420_e982_d_n9;
        var_vfb1_t_db0 = assign420_e982_d_b0;
        var_vfb1_t_db1 = assign420_e982_d_b1;
        var_vfb1_t_db2 = assign420_e982_d_b2;
        var_vfb1_t_db3 = assign420_e982_d_b3;

        let (assign430_e986, assign430_e986_d_n0, assign430_e986_d_n1, assign430_e986_d_n2, assign430_e986_d_n3, assign430_e986_d_n4, assign430_e986_d_n5, assign430_e986_d_n6, assign430_e986_d_n7, assign430_e986_d_n8, assign430_e986_d_n9, assign430_e986_d_b0, assign430_e986_d_b1, assign430_e986_d_b2, assign430_e986_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb2_t, var_vfb2_t_dn0, var_vfb2_t_dn1, var_vfb2_t_dn2, var_vfb2_t_dn3, var_vfb2_t_dn4, var_vfb2_t_dn5, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9, var_vfb2_t_db0, var_vfb2_t_db1, var_vfb2_t_db2, var_vfb2_t_db3,)
    }
};
        var_vfb2_t = assign430_e986;
        var_vfb2_t_dn0 = assign430_e986_d_n0;
        var_vfb2_t_dn1 = assign430_e986_d_n1;
        var_vfb2_t_dn2 = assign430_e986_d_n2;
        var_vfb2_t_dn3 = assign430_e986_d_n3;
        var_vfb2_t_dn4 = assign430_e986_d_n4;
        var_vfb2_t_dn5 = assign430_e986_d_n5;
        var_vfb2_t_dn6 = assign430_e986_d_n6;
        var_vfb2_t_dn7 = assign430_e986_d_n7;
        var_vfb2_t_dn8 = assign430_e986_d_n8;
        var_vfb2_t_dn9 = assign430_e986_d_n9;
        var_vfb2_t_db0 = assign430_e986_d_b0;
        var_vfb2_t_db1 = assign430_e986_d_b1;
        var_vfb2_t_db2 = assign430_e986_d_b2;
        var_vfb2_t_db3 = assign430_e986_d_b3;

        let (assign440_e990,) = {
    if (var_guard83 != 0.0) {
        (p.p53,)
    } else {
        (var_stvfb_i,)
    }
};
        var_stvfb_i = assign440_e990;

        let (assign450_e996, assign450_e996_d_n0, assign450_e996_d_n1, assign450_e996_d_n2, assign450_e996_d_n3, assign450_e996_d_n4, assign450_e996_d_n5, assign450_e996_d_n6, assign450_e996_d_n7, assign450_e996_d_n8, assign450_e996_d_n9, assign450_e996_d_b0, assign450_e996_d_b1, assign450_e996_d_b2, assign450_e996_d_b3,) = {
    if (var_guard83 != 0.0) {
        let assign450_e994: f64 = (p.p54 * 1000000.0);
        (assign450_e994, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_np_i, var_np_i_dn0, var_np_i_dn1, var_np_i_dn2, var_np_i_dn3, var_np_i_dn4, var_np_i_dn5, var_np_i_dn6, var_np_i_dn7, var_np_i_dn8, var_np_i_dn9, var_np_i_db0, var_np_i_db1, var_np_i_db2, var_np_i_db3,)
    }
};
        var_np_i = assign450_e996;
        var_np_i_dn0 = assign450_e996_d_n0;
        var_np_i_dn1 = assign450_e996_d_n1;
        var_np_i_dn2 = assign450_e996_d_n2;
        var_np_i_dn3 = assign450_e996_d_n3;
        var_np_i_dn4 = assign450_e996_d_n4;
        var_np_i_dn5 = assign450_e996_d_n5;
        var_np_i_dn6 = assign450_e996_d_n6;
        var_np_i_dn7 = assign450_e996_d_n7;
        var_np_i_dn8 = assign450_e996_d_n8;
        var_np_i_dn9 = assign450_e996_d_n9;
        var_np_i_db0 = assign450_e996_d_b0;
        var_np_i_db1 = assign450_e996_d_b1;
        var_np_i_db2 = assign450_e996_d_b2;
        var_np_i_db3 = assign450_e996_d_b3;

        let (assign460_e1000,) = {
    if (var_guard83 != 0.0) {
        (p.p55,)
    } else {
        (var_cic1_i,)
    }
};
        var_cic1_i = assign460_e1000;

        let (assign470_e1004,) = {
    if (var_guard83 != 0.0) {
        (p.p56,)
    } else {
        (var_cic2_i,)
    }
};
        var_cic2_i = assign470_e1004;

        let (assign480_e1008,) = {
    if (var_guard83 != 0.0) {
        (p.p57,)
    } else {
        (var_psce1_i,)
    }
};
        var_psce1_i = assign480_e1008;

        let (assign490_e1018,) = {
    if (var_guard83 != 0.0) {
        let assign490_e1012: f64 = (p.p58 * var_psce1_i);
        let assign490_e1014: f64 = (assign490_e1012 * var_tox2_i);
        let assign490_e1016: f64 = (assign490_e1014 / var_tox1_i);
        (assign490_e1016,)
    } else {
        (var_psce2_i,)
    }
};
        var_psce2_i = assign490_e1018;

        let (assign500_e1024,) = {
    if (var_guard83 != 0.0) {
        let assign500_e1022: f64 = (p.p59 * 1000000.0);
        (assign500_e1022,)
    } else {
        (var_nsddc_i,)
    }
};
        var_nsddc_i = assign500_e1024;

        let (assign510_e1028,) = {
    if (var_guard83 != 0.0) {
        (p.p60,)
    } else {
        (var_pscedlb_i,)
    }
};
        var_pscedlb_i = assign510_e1028;

        let (assign520_e1032,) = {
    if (var_guard83 != 0.0) {
        (p.p61,)
    } else {
        (var_pnce_i,)
    }
};
        var_pnce_i = assign520_e1032;

        let (assign530_e1036, assign530_e1036_d_n0, assign530_e1036_d_n1, assign530_e1036_d_n2, assign530_e1036_d_n3, assign530_e1036_d_n4, assign530_e1036_d_n5, assign530_e1036_d_n6, assign530_e1036_d_n7, assign530_e1036_d_n8, assign530_e1036_d_n9, assign530_e1036_d_b0, assign530_e1036_d_b1, assign530_e1036_d_b2, assign530_e1036_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cf1_t, var_cf1_t_dn0, var_cf1_t_dn1, var_cf1_t_dn2, var_cf1_t_dn3, var_cf1_t_dn4, var_cf1_t_dn5, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9, var_cf1_t_db0, var_cf1_t_db1, var_cf1_t_db2, var_cf1_t_db3,)
    }
};
        var_cf1_t = assign530_e1036;
        var_cf1_t_dn0 = assign530_e1036_d_n0;
        var_cf1_t_dn1 = assign530_e1036_d_n1;
        var_cf1_t_dn2 = assign530_e1036_d_n2;
        var_cf1_t_dn3 = assign530_e1036_d_n3;
        var_cf1_t_dn4 = assign530_e1036_d_n4;
        var_cf1_t_dn5 = assign530_e1036_d_n5;
        var_cf1_t_dn6 = assign530_e1036_d_n6;
        var_cf1_t_dn7 = assign530_e1036_d_n7;
        var_cf1_t_dn8 = assign530_e1036_d_n8;
        var_cf1_t_dn9 = assign530_e1036_d_n9;
        var_cf1_t_db0 = assign530_e1036_d_b0;
        var_cf1_t_db1 = assign530_e1036_d_b1;
        var_cf1_t_db2 = assign530_e1036_d_b2;
        var_cf1_t_db3 = assign530_e1036_d_b3;

        let (assign540_e1046, assign540_e1046_d_n0, assign540_e1046_d_n1, assign540_e1046_d_n2, assign540_e1046_d_n3, assign540_e1046_d_n4, assign540_e1046_d_n5, assign540_e1046_d_n6, assign540_e1046_d_n7, assign540_e1046_d_n8, assign540_e1046_d_n9, assign540_e1046_d_b0, assign540_e1046_d_b1, assign540_e1046_d_b2, assign540_e1046_d_b3,) = {
    if (var_guard83 != 0.0) {
        let assign540_e1040: f64 = (p.p63 * var_cf1_t);
        let assign540_e1042: f64 = (assign540_e1040 * var_tox2_i);
        let assign540_e1044: f64 = (assign540_e1042 / var_tox1_i);
        (assign540_e1044, (((p.p63 * var_cf1_t_dn0) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn1) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn2) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn3) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn5) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_dn9) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_db0) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_db1) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_db2) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cf1_t_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2_t, var_cf2_t_dn0, var_cf2_t_dn1, var_cf2_t_dn2, var_cf2_t_dn3, var_cf2_t_dn4, var_cf2_t_dn5, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9, var_cf2_t_db0, var_cf2_t_db1, var_cf2_t_db2, var_cf2_t_db3,)
    }
};
        var_cf2_t = assign540_e1046;
        var_cf2_t_dn0 = assign540_e1046_d_n0;
        var_cf2_t_dn1 = assign540_e1046_d_n1;
        var_cf2_t_dn2 = assign540_e1046_d_n2;
        var_cf2_t_dn3 = assign540_e1046_d_n3;
        var_cf2_t_dn4 = assign540_e1046_d_n4;
        var_cf2_t_dn5 = assign540_e1046_d_n5;
        var_cf2_t_dn6 = assign540_e1046_d_n6;
        var_cf2_t_dn7 = assign540_e1046_d_n7;
        var_cf2_t_dn8 = assign540_e1046_d_n8;
        var_cf2_t_dn9 = assign540_e1046_d_n9;
        var_cf2_t_db0 = assign540_e1046_d_b0;
        var_cf2_t_db1 = assign540_e1046_d_b1;
        var_cf2_t_db2 = assign540_e1046_d_b2;
        var_cf2_t_db3 = assign540_e1046_d_b3;

        let (assign550_e1050, assign550_e1050_d_n0, assign550_e1050_d_n1, assign550_e1050_d_n2, assign550_e1050_d_n3, assign550_e1050_d_n4, assign550_e1050_d_n5, assign550_e1050_d_n6, assign550_e1050_d_n7, assign550_e1050_d_n8, assign550_e1050_d_n9, assign550_e1050_d_b0, assign550_e1050_d_b1, assign550_e1050_d_b2, assign550_e1050_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p64, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_stcf_i, var_stcf_i_dn0, var_stcf_i_dn1, var_stcf_i_dn2, var_stcf_i_dn3, var_stcf_i_dn4, var_stcf_i_dn5, var_stcf_i_dn6, var_stcf_i_dn7, var_stcf_i_dn8, var_stcf_i_dn9, var_stcf_i_db0, var_stcf_i_db1, var_stcf_i_db2, var_stcf_i_db3,)
    }
};
        var_stcf_i = assign550_e1050;
        var_stcf_i_dn0 = assign550_e1050_d_n0;
        var_stcf_i_dn1 = assign550_e1050_d_n1;
        var_stcf_i_dn2 = assign550_e1050_d_n2;
        var_stcf_i_dn3 = assign550_e1050_d_n3;
        var_stcf_i_dn4 = assign550_e1050_d_n4;
        var_stcf_i_dn5 = assign550_e1050_d_n5;
        var_stcf_i_dn6 = assign550_e1050_d_n6;
        var_stcf_i_dn7 = assign550_e1050_d_n7;
        var_stcf_i_dn8 = assign550_e1050_d_n8;
        var_stcf_i_dn9 = assign550_e1050_d_n9;
        var_stcf_i_db0 = assign550_e1050_d_b0;
        var_stcf_i_db1 = assign550_e1050_d_b1;
        var_stcf_i_db2 = assign550_e1050_d_b2;
        var_stcf_i_db3 = assign550_e1050_d_b3;

        let (assign560_e1054,) = {
    if (var_guard83 != 0.0) {
        (p.p65,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign560_e1054;

        let (assign570_e1058,) = {
    if (var_guard83 != 0.0) {
        (p.p66,)
    } else {
        (var_cfdl_i,)
    }
};
        var_cfdl_i = assign570_e1058;

        let (assign580_e1062,) = {
    if (var_guard83 != 0.0) {
        (p.p67,)
    } else {
        (var_cfdlb_i,)
    }
};
        var_cfdlb_i = assign580_e1062;

        let (assign590_e1066, assign590_e1066_d_n0, assign590_e1066_d_n1, assign590_e1066_d_n2, assign590_e1066_d_n3, assign590_e1066_d_n4, assign590_e1066_d_n5, assign590_e1066_d_n6, assign590_e1066_d_n7, assign590_e1066_d_n8, assign590_e1066_d_n9, assign590_e1066_d_b0, assign590_e1066_d_b1, assign590_e1066_d_b2, assign590_e1066_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p68, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_betn1_t, var_betn1_t_dn0, var_betn1_t_dn1, var_betn1_t_dn2, var_betn1_t_dn3, var_betn1_t_dn4, var_betn1_t_dn5, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9, var_betn1_t_db0, var_betn1_t_db1, var_betn1_t_db2, var_betn1_t_db3,)
    }
};
        var_betn1_t = assign590_e1066;
        var_betn1_t_dn0 = assign590_e1066_d_n0;
        var_betn1_t_dn1 = assign590_e1066_d_n1;
        var_betn1_t_dn2 = assign590_e1066_d_n2;
        var_betn1_t_dn3 = assign590_e1066_d_n3;
        var_betn1_t_dn4 = assign590_e1066_d_n4;
        var_betn1_t_dn5 = assign590_e1066_d_n5;
        var_betn1_t_dn6 = assign590_e1066_d_n6;
        var_betn1_t_dn7 = assign590_e1066_d_n7;
        var_betn1_t_dn8 = assign590_e1066_d_n8;
        var_betn1_t_dn9 = assign590_e1066_d_n9;
        var_betn1_t_db0 = assign590_e1066_d_b0;
        var_betn1_t_db1 = assign590_e1066_d_b1;
        var_betn1_t_db2 = assign590_e1066_d_b2;
        var_betn1_t_db3 = assign590_e1066_d_b3;

        let (assign600_e1072, assign600_e1072_d_n0, assign600_e1072_d_n1, assign600_e1072_d_n2, assign600_e1072_d_n3, assign600_e1072_d_n4, assign600_e1072_d_n5, assign600_e1072_d_n6, assign600_e1072_d_n7, assign600_e1072_d_n8, assign600_e1072_d_n9, assign600_e1072_d_b0, assign600_e1072_d_b1, assign600_e1072_d_b2, assign600_e1072_d_b3,) = {
    if (var_guard83 != 0.0) {
        let assign600_e1070: f64 = (p.p69 * var_betn1_t);
        (assign600_e1070, (p.p69 * var_betn1_t_dn0), (p.p69 * var_betn1_t_dn1), (p.p69 * var_betn1_t_dn2), (p.p69 * var_betn1_t_dn3), (p.p69 * var_betn1_t_dn4), (p.p69 * var_betn1_t_dn5), (p.p69 * var_betn1_t_dn6), (p.p69 * var_betn1_t_dn7), (p.p69 * var_betn1_t_dn8), (p.p69 * var_betn1_t_dn9), (p.p69 * var_betn1_t_db0), (p.p69 * var_betn1_t_db1), (p.p69 * var_betn1_t_db2), (p.p69 * var_betn1_t_db3),)
    } else {
        (var_betn2_t, var_betn2_t_dn0, var_betn2_t_dn1, var_betn2_t_dn2, var_betn2_t_dn3, var_betn2_t_dn4, var_betn2_t_dn5, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9, var_betn2_t_db0, var_betn2_t_db1, var_betn2_t_db2, var_betn2_t_db3,)
    }
};
        var_betn2_t = assign600_e1072;
        var_betn2_t_dn0 = assign600_e1072_d_n0;
        var_betn2_t_dn1 = assign600_e1072_d_n1;
        var_betn2_t_dn2 = assign600_e1072_d_n2;
        var_betn2_t_dn3 = assign600_e1072_d_n3;
        var_betn2_t_dn4 = assign600_e1072_d_n4;
        var_betn2_t_dn5 = assign600_e1072_d_n5;
        var_betn2_t_dn6 = assign600_e1072_d_n6;
        var_betn2_t_dn7 = assign600_e1072_d_n7;
        var_betn2_t_dn8 = assign600_e1072_d_n8;
        var_betn2_t_dn9 = assign600_e1072_d_n9;
        var_betn2_t_db0 = assign600_e1072_d_b0;
        var_betn2_t_db1 = assign600_e1072_d_b1;
        var_betn2_t_db2 = assign600_e1072_d_b2;
        var_betn2_t_db3 = assign600_e1072_d_b3;

        let (assign610_e1076,) = {
    if (var_guard83 != 0.0) {
        (p.p70,)
    } else {
        (var_stbet_i,)
    }
};
        var_stbet_i = assign610_e1076;

        let (assign620_e1080,) = {
    if (var_guard83 != 0.0) {
        (p.p71,)
    } else {
        (var_cs_t,)
    }
};
        var_cs_t = assign620_e1080;

        let (assign630_e1084,) = {
    if (var_guard83 != 0.0) {
        (p.p72,)
    } else {
        (var_csfi_i,)
    }
};
        var_csfi_i = assign630_e1084;

        let (assign640_e1088,) = {
    if (var_guard83 != 0.0) {
        (p.p73,)
    } else {
        (var_csbi_i,)
    }
};
        var_csbi_i = assign640_e1088;

        let (assign650_e1092,) = {
    if (var_guard83 != 0.0) {
        (p.p74,)
    } else {
        (var_stcs_i,)
    }
};
        var_stcs_i = assign650_e1092;

        let (assign660_e1096,) = {
    if (var_guard83 != 0.0) {
        (p.p75,)
    } else {
        (var_thecs_t,)
    }
};
        var_thecs_t = assign660_e1096;

        let (assign670_e1100,) = {
    if (var_guard83 != 0.0) {
        (p.p76,)
    } else {
        (var_stthecs_i,)
    }
};
        var_stthecs_i = assign670_e1100;

        let (assign680_e1104,) = {
    if (var_guard83 != 0.0) {
        (p.p77,)
    } else {
        (var_csthr_i,)
    }
};
        var_csthr_i = assign680_e1104;

        let (assign690_e1108,) = {
    if (var_guard83 != 0.0) {
        (p.p78,)
    } else {
        (var_csthrb_i,)
    }
};
        var_csthrb_i = assign690_e1108;

        let (assign700_e1112,) = {
    if (var_guard83 != 0.0) {
        (p.p79,)
    } else {
        (var_mue_t,)
    }
};
        var_mue_t = assign700_e1112;

        let (assign710_e1116,) = {
    if (var_guard83 != 0.0) {
        (p.p80,)
    } else {
        (var_stmue_i,)
    }
};
        var_stmue_i = assign710_e1116;

        let (assign720_e1120,) = {
    if (var_guard83 != 0.0) {
        (p.p81,)
    } else {
        (var_themu_t,)
    }
};
        var_themu_t = assign720_e1120;

        let (assign730_e1124,) = {
    if (var_guard83 != 0.0) {
        (p.p82,)
    } else {
        (var_stthemu_i,)
    }
};
        var_stthemu_i = assign730_e1124;

        let (assign740_e1128,) = {
    if (var_guard83 != 0.0) {
        (p.p83,)
    } else {
        (var_xcor_t,)
    }
};
        var_xcor_t = assign740_e1128;

        let (assign750_e1132,) = {
    if (var_guard83 != 0.0) {
        (p.p84,)
    } else {
        (var_xcorb_i,)
    }
};
        var_xcorb_i = assign750_e1132;

        let (assign760_e1136,) = {
    if (var_guard83 != 0.0) {
        (p.p85,)
    } else {
        (var_stxcor_i,)
    }
};
        var_stxcor_i = assign760_e1136;

        let (assign770_e1140,) = {
    if (var_guard83 != 0.0) {
        (p.p86,)
    } else {
        (var_feta_i,)
    }
};
        var_feta_i = assign770_e1140;

        let (assign780_e1144,) = {
    if (var_guard83 != 0.0) {
        (p.p87,)
    } else {
        (var_rs_t,)
    }
};
        var_rs_t = assign780_e1144;

        let (assign790_e1148,) = {
    if (var_guard83 != 0.0) {
        (p.p88,)
    } else {
        (var_rsig_i,)
    }
};
        var_rsig_i = assign790_e1148;

        let (assign800_e1152,) = {
    if (var_guard83 != 0.0) {
        (p.p89,)
    } else {
        (var_strs_i,)
    }
};
        var_strs_i = assign800_e1152;

        let (assign810_e1156,) = {
    if (var_guard83 != 0.0) {
        (p.p90,)
    } else {
        (var_rsg_i,)
    }
};
        var_rsg_i = assign810_e1156;

        let (assign820_e1160,) = {
    if (var_guard83 != 0.0) {
        (p.p91,)
    } else {
        (var_thersg_i,)
    }
};
        var_thersg_i = assign820_e1160;

        let (assign830_e1164,) = {
    if (var_guard83 != 0.0) {
        (p.p92,)
    } else {
        (var_rsb_i,)
    }
};
        var_rsb_i = assign830_e1164;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_db0_slot = var_betn1_t_db0;
        *var_betn1_t_db1_slot = var_betn1_t_db1;
        *var_betn1_t_db2_slot = var_betn1_t_db2;
        *var_betn1_t_db3_slot = var_betn1_t_db3;
        *var_betn1_t_dn0_slot = var_betn1_t_dn0;
        *var_betn1_t_dn1_slot = var_betn1_t_dn1;
        *var_betn1_t_dn2_slot = var_betn1_t_dn2;
        *var_betn1_t_dn3_slot = var_betn1_t_dn3;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn5_slot = var_betn1_t_dn5;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_db0_slot = var_betn2_t_db0;
        *var_betn2_t_db1_slot = var_betn2_t_db1;
        *var_betn2_t_db2_slot = var_betn2_t_db2;
        *var_betn2_t_db3_slot = var_betn2_t_db3;
        *var_betn2_t_dn0_slot = var_betn2_t_dn0;
        *var_betn2_t_dn1_slot = var_betn2_t_dn1;
        *var_betn2_t_dn2_slot = var_betn2_t_dn2;
        *var_betn2_t_dn3_slot = var_betn2_t_dn3;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn5_slot = var_betn2_t_dn5;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_db0_slot = var_cf1_t_db0;
        *var_cf1_t_db1_slot = var_cf1_t_db1;
        *var_cf1_t_db2_slot = var_cf1_t_db2;
        *var_cf1_t_db3_slot = var_cf1_t_db3;
        *var_cf1_t_dn0_slot = var_cf1_t_dn0;
        *var_cf1_t_dn1_slot = var_cf1_t_dn1;
        *var_cf1_t_dn2_slot = var_cf1_t_dn2;
        *var_cf1_t_dn3_slot = var_cf1_t_dn3;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn5_slot = var_cf1_t_dn5;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_db0_slot = var_cf2_t_db0;
        *var_cf2_t_db1_slot = var_cf2_t_db1;
        *var_cf2_t_db2_slot = var_cf2_t_db2;
        *var_cf2_t_db3_slot = var_cf2_t_db3;
        *var_cf2_t_dn0_slot = var_cf2_t_dn0;
        *var_cf2_t_dn1_slot = var_cf2_t_dn1;
        *var_cf2_t_dn2_slot = var_cf2_t_dn2;
        *var_cf2_t_dn3_slot = var_cf2_t_dn3;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn5_slot = var_cf2_t_dn5;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfdl_i_slot = var_cfdl_i;
        *var_cfdlb_i_slot = var_cfdlb_i;
        *var_cic1_i_slot = var_cic1_i;
        *var_cic2_i_slot = var_cic2_i;
        *var_cs_t_slot = var_cs_t;
        *var_csbi_i_slot = var_csbi_i;
        *var_csfi_i_slot = var_csfi_i;
        *var_csthr_i_slot = var_csthr_i;
        *var_csthrb_i_slot = var_csthrb_i;
        *var_feta_i_slot = var_feta_i;
        *var_mue_t_slot = var_mue_t;
        *var_np_i_slot = var_np_i;
        *var_np_i_db0_slot = var_np_i_db0;
        *var_np_i_db1_slot = var_np_i_db1;
        *var_np_i_db2_slot = var_np_i_db2;
        *var_np_i_db3_slot = var_np_i_db3;
        *var_np_i_dn0_slot = var_np_i_dn0;
        *var_np_i_dn1_slot = var_np_i_dn1;
        *var_np_i_dn2_slot = var_np_i_dn2;
        *var_np_i_dn3_slot = var_np_i_dn3;
        *var_np_i_dn4_slot = var_np_i_dn4;
        *var_np_i_dn5_slot = var_np_i_dn5;
        *var_np_i_dn6_slot = var_np_i_dn6;
        *var_np_i_dn7_slot = var_np_i_dn7;
        *var_np_i_dn8_slot = var_np_i_dn8;
        *var_np_i_dn9_slot = var_np_i_dn9;
        *var_nsddc_i_slot = var_nsddc_i;
        *var_pnce_i_slot = var_pnce_i;
        *var_psce1_i_slot = var_psce1_i;
        *var_psce2_i_slot = var_psce2_i;
        *var_pscedlb_i_slot = var_pscedlb_i;
        *var_rs_t_slot = var_rs_t;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsig_i_slot = var_rsig_i;
        *var_stbet_i_slot = var_stbet_i;
        *var_stcf_i_slot = var_stcf_i;
        *var_stcf_i_db0_slot = var_stcf_i_db0;
        *var_stcf_i_db1_slot = var_stcf_i_db1;
        *var_stcf_i_db2_slot = var_stcf_i_db2;
        *var_stcf_i_db3_slot = var_stcf_i_db3;
        *var_stcf_i_dn0_slot = var_stcf_i_dn0;
        *var_stcf_i_dn1_slot = var_stcf_i_dn1;
        *var_stcf_i_dn2_slot = var_stcf_i_dn2;
        *var_stcf_i_dn3_slot = var_stcf_i_dn3;
        *var_stcf_i_dn4_slot = var_stcf_i_dn4;
        *var_stcf_i_dn5_slot = var_stcf_i_dn5;
        *var_stcf_i_dn6_slot = var_stcf_i_dn6;
        *var_stcf_i_dn7_slot = var_stcf_i_dn7;
        *var_stcf_i_dn8_slot = var_stcf_i_dn8;
        *var_stcf_i_dn9_slot = var_stcf_i_dn9;
        *var_stcs_i_slot = var_stcs_i;
        *var_stmue_i_slot = var_stmue_i;
        *var_strs_i_slot = var_strs_i;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_thecs_t_slot = var_thecs_t;
        *var_themu_t_slot = var_themu_t;
        *var_thersg_i_slot = var_thersg_i;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_db0_slot = var_vfb1_t_db0;
        *var_vfb1_t_db1_slot = var_vfb1_t_db1;
        *var_vfb1_t_db2_slot = var_vfb1_t_db2;
        *var_vfb1_t_db3_slot = var_vfb1_t_db3;
        *var_vfb1_t_dn0_slot = var_vfb1_t_dn0;
        *var_vfb1_t_dn1_slot = var_vfb1_t_dn1;
        *var_vfb1_t_dn2_slot = var_vfb1_t_dn2;
        *var_vfb1_t_dn3_slot = var_vfb1_t_dn3;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn5_slot = var_vfb1_t_dn5;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_db0_slot = var_vfb2_t_db0;
        *var_vfb2_t_db1_slot = var_vfb2_t_db1;
        *var_vfb2_t_db2_slot = var_vfb2_t_db2;
        *var_vfb2_t_db3_slot = var_vfb2_t_db3;
        *var_vfb2_t_dn0_slot = var_vfb2_t_dn0;
        *var_vfb2_t_dn1_slot = var_vfb2_t_dn1;
        *var_vfb2_t_dn2_slot = var_vfb2_t_dn2;
        *var_vfb2_t_dn3_slot = var_vfb2_t_dn3;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn5_slot = var_vfb2_t_dn5;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcorb_i_slot = var_xcorb_i;
    }

    pub(super) fn stamp_transient_block_2(
        p: &Parameters,
        var_guard83: f64,
        var_a1_i_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_db0_slot: &mut f64,
        var_agidl_i_db1_slot: &mut f64,
        var_agidl_i_db2_slot: &mut f64,
        var_agidl_i_db3_slot: &mut f64,
        var_agidl_i_dn0_slot: &mut f64,
        var_agidl_i_dn1_slot: &mut f64,
        var_agidl_i_dn2_slot: &mut f64,
        var_agidl_i_dn3_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn5_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_db0_slot: &mut f64,
        var_agidld_i_db1_slot: &mut f64,
        var_agidld_i_db2_slot: &mut f64,
        var_agidld_i_db3_slot: &mut f64,
        var_agidld_i_dn0_slot: &mut f64,
        var_agidld_i_dn1_slot: &mut f64,
        var_agidld_i_dn2_slot: &mut f64,
        var_agidld_i_dn3_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn5_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alpb_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_cic1edge_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_dgidl_i_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_fnovinv_t_slot: &mut f64,
        var_fnovinvd_t_slot: &mut f64,
        var_gc2ch_i_slot: &mut f64,
        var_gc2ovacc_i_slot: &mut f64,
        var_gc2ovinv_i_slot: &mut f64,
        var_gc3ch_i_slot: &mut f64,
        var_gc3ovacc_i_slot: &mut f64,
        var_gc3ovinv_i_slot: &mut f64,
        var_gcdov_i_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gcovinvfn_i_slot: &mut f64,
        var_gcvdov_i_slot: &mut f64,
        var_iginv_t_slot: &mut f64,
        var_igovacc_t_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovinv_t_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_niginv_i_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stigfn_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_thesat1_i_slot: &mut f64,
        var_thesat2_i_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_db0_slot: &mut f64,
        var_thesat_t_db1_slot: &mut f64,
        var_thesat_t_db2_slot: &mut f64,
        var_thesat_t_db3_slot: &mut f64,
        var_thesat_t_dn0_slot: &mut f64,
        var_thesat_t_dn1_slot: &mut f64,
        var_thesat_t_dn2_slot: &mut f64,
        var_thesat_t_dn3_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn5_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_vfb1edge_t_slot: &mut f64,
        var_vfb1edge_t_db0_slot: &mut f64,
        var_vfb1edge_t_db1_slot: &mut f64,
        var_vfb1edge_t_db2_slot: &mut f64,
        var_vfb1edge_t_db3_slot: &mut f64,
        var_vfb1edge_t_dn0_slot: &mut f64,
        var_vfb1edge_t_dn1_slot: &mut f64,
        var_vfb1edge_t_dn2_slot: &mut f64,
        var_vfb1edge_t_dn3_slot: &mut f64,
        var_vfb1edge_t_dn4_slot: &mut f64,
        var_vfb1edge_t_dn5_slot: &mut f64,
        var_vfb1edge_t_dn6_slot: &mut f64,
        var_vfb1edge_t_dn7_slot: &mut f64,
        var_vfb1edge_t_dn8_slot: &mut f64,
        var_vfb1edge_t_dn9_slot: &mut f64,
        var_vfb2edge_t_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vpg_i_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_db0: f64 = *var_agidl_i_db0_slot;
        let mut var_agidl_i_db1: f64 = *var_agidl_i_db1_slot;
        let mut var_agidl_i_db2: f64 = *var_agidl_i_db2_slot;
        let mut var_agidl_i_db3: f64 = *var_agidl_i_db3_slot;
        let mut var_agidl_i_dn0: f64 = *var_agidl_i_dn0_slot;
        let mut var_agidl_i_dn1: f64 = *var_agidl_i_dn1_slot;
        let mut var_agidl_i_dn2: f64 = *var_agidl_i_dn2_slot;
        let mut var_agidl_i_dn3: f64 = *var_agidl_i_dn3_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn5: f64 = *var_agidl_i_dn5_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_db0: f64 = *var_agidld_i_db0_slot;
        let mut var_agidld_i_db1: f64 = *var_agidld_i_db1_slot;
        let mut var_agidld_i_db2: f64 = *var_agidld_i_db2_slot;
        let mut var_agidld_i_db3: f64 = *var_agidld_i_db3_slot;
        let mut var_agidld_i_dn0: f64 = *var_agidld_i_dn0_slot;
        let mut var_agidld_i_dn1: f64 = *var_agidld_i_dn1_slot;
        let mut var_agidld_i_dn2: f64 = *var_agidld_i_dn2_slot;
        let mut var_agidld_i_dn3: f64 = *var_agidld_i_dn3_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn5: f64 = *var_agidld_i_dn5_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alpb_i: f64 = *var_alpb_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_cic1edge_i: f64 = *var_cic1edge_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_dgidl_i: f64 = *var_dgidl_i_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_fnovinv_t: f64 = *var_fnovinv_t_slot;
        let mut var_fnovinvd_t: f64 = *var_fnovinvd_t_slot;
        let mut var_gc2ch_i: f64 = *var_gc2ch_i_slot;
        let mut var_gc2ovacc_i: f64 = *var_gc2ovacc_i_slot;
        let mut var_gc2ovinv_i: f64 = *var_gc2ovinv_i_slot;
        let mut var_gc3ch_i: f64 = *var_gc3ch_i_slot;
        let mut var_gc3ovacc_i: f64 = *var_gc3ovacc_i_slot;
        let mut var_gc3ovinv_i: f64 = *var_gc3ovinv_i_slot;
        let mut var_gcdov_i: f64 = *var_gcdov_i_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gcovinvfn_i: f64 = *var_gcovinvfn_i_slot;
        let mut var_gcvdov_i: f64 = *var_gcvdov_i_slot;
        let mut var_iginv_t: f64 = *var_iginv_t_slot;
        let mut var_igovacc_t: f64 = *var_igovacc_t_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovinv_t: f64 = *var_igovinv_t_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_niginv_i: f64 = *var_niginv_i_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stigfn_i: f64 = *var_stigfn_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_thesat1_i: f64 = *var_thesat1_i_slot;
        let mut var_thesat2_i: f64 = *var_thesat2_i_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_db0: f64 = *var_thesat_t_db0_slot;
        let mut var_thesat_t_db1: f64 = *var_thesat_t_db1_slot;
        let mut var_thesat_t_db2: f64 = *var_thesat_t_db2_slot;
        let mut var_thesat_t_db3: f64 = *var_thesat_t_db3_slot;
        let mut var_thesat_t_dn0: f64 = *var_thesat_t_dn0_slot;
        let mut var_thesat_t_dn1: f64 = *var_thesat_t_dn1_slot;
        let mut var_thesat_t_dn2: f64 = *var_thesat_t_dn2_slot;
        let mut var_thesat_t_dn3: f64 = *var_thesat_t_dn3_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn5: f64 = *var_thesat_t_dn5_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_vfb1edge_t: f64 = *var_vfb1edge_t_slot;
        let mut var_vfb1edge_t_db0: f64 = *var_vfb1edge_t_db0_slot;
        let mut var_vfb1edge_t_db1: f64 = *var_vfb1edge_t_db1_slot;
        let mut var_vfb1edge_t_db2: f64 = *var_vfb1edge_t_db2_slot;
        let mut var_vfb1edge_t_db3: f64 = *var_vfb1edge_t_db3_slot;
        let mut var_vfb1edge_t_dn0: f64 = *var_vfb1edge_t_dn0_slot;
        let mut var_vfb1edge_t_dn1: f64 = *var_vfb1edge_t_dn1_slot;
        let mut var_vfb1edge_t_dn2: f64 = *var_vfb1edge_t_dn2_slot;
        let mut var_vfb1edge_t_dn3: f64 = *var_vfb1edge_t_dn3_slot;
        let mut var_vfb1edge_t_dn4: f64 = *var_vfb1edge_t_dn4_slot;
        let mut var_vfb1edge_t_dn5: f64 = *var_vfb1edge_t_dn5_slot;
        let mut var_vfb1edge_t_dn6: f64 = *var_vfb1edge_t_dn6_slot;
        let mut var_vfb1edge_t_dn7: f64 = *var_vfb1edge_t_dn7_slot;
        let mut var_vfb1edge_t_dn8: f64 = *var_vfb1edge_t_dn8_slot;
        let mut var_vfb1edge_t_dn9: f64 = *var_vfb1edge_t_dn9_slot;
        let mut var_vfb2edge_t: f64 = *var_vfb2edge_t_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vpg_i: f64 = *var_vpg_i_slot;

        let (assign840_e1168, assign840_e1168_d_n0, assign840_e1168_d_n1, assign840_e1168_d_n2, assign840_e1168_d_n3, assign840_e1168_d_n4, assign840_e1168_d_n5, assign840_e1168_d_n6, assign840_e1168_d_n7, assign840_e1168_d_n8, assign840_e1168_d_n9, assign840_e1168_d_b0, assign840_e1168_d_b1, assign840_e1168_d_b2, assign840_e1168_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesat_t, var_thesat_t_dn0, var_thesat_t_dn1, var_thesat_t_dn2, var_thesat_t_dn3, var_thesat_t_dn4, var_thesat_t_dn5, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9, var_thesat_t_db0, var_thesat_t_db1, var_thesat_t_db2, var_thesat_t_db3,)
    }
};
        var_thesat_t = assign840_e1168;
        var_thesat_t_dn0 = assign840_e1168_d_n0;
        var_thesat_t_dn1 = assign840_e1168_d_n1;
        var_thesat_t_dn2 = assign840_e1168_d_n2;
        var_thesat_t_dn3 = assign840_e1168_d_n3;
        var_thesat_t_dn4 = assign840_e1168_d_n4;
        var_thesat_t_dn5 = assign840_e1168_d_n5;
        var_thesat_t_dn6 = assign840_e1168_d_n6;
        var_thesat_t_dn7 = assign840_e1168_d_n7;
        var_thesat_t_dn8 = assign840_e1168_d_n8;
        var_thesat_t_dn9 = assign840_e1168_d_n9;
        var_thesat_t_db0 = assign840_e1168_d_b0;
        var_thesat_t_db1 = assign840_e1168_d_b1;
        var_thesat_t_db2 = assign840_e1168_d_b2;
        var_thesat_t_db3 = assign840_e1168_d_b3;

        let (assign850_e1172,) = {
    if (var_guard83 != 0.0) {
        (p.p94,)
    } else {
        (var_stthesat_i,)
    }
};
        var_stthesat_i = assign850_e1172;

        let (assign860_e1176,) = {
    if (var_guard83 != 0.0) {
        (p.p95,)
    } else {
        (var_thesat1_i,)
    }
};
        var_thesat1_i = assign860_e1176;

        let (assign870_e1180,) = {
    if (var_guard83 != 0.0) {
        (p.p96,)
    } else {
        (var_thesat2_i,)
    }
};
        var_thesat2_i = assign870_e1180;

        let (assign880_e1184,) = {
    if (var_guard83 != 0.0) {
        (p.p97,)
    } else {
        (var_ax_i,)
    }
};
        var_ax_i = assign880_e1184;

        let (assign890_e1188,) = {
    if (var_guard83 != 0.0) {
        (p.p98,)
    } else {
        (var_alp_i,)
    }
};
        var_alp_i = assign890_e1188;

        let (assign900_e1192,) = {
    if (var_guard83 != 0.0) {
        (p.p99,)
    } else {
        (var_alp1_i,)
    }
};
        var_alp1_i = assign900_e1192;

        let (assign910_e1196,) = {
    if (var_guard83 != 0.0) {
        (p.p100,)
    } else {
        (var_alpb_i,)
    }
};
        var_alpb_i = assign910_e1196;

        let (assign920_e1200,) = {
    if (var_guard83 != 0.0) {
        (p.p101,)
    } else {
        (var_vp_i,)
    }
};
        var_vp_i = assign920_e1200;

        let (assign930_e1204,) = {
    if (var_guard83 != 0.0) {
        (p.p102,)
    } else {
        (var_vpg_i,)
    }
};
        var_vpg_i = assign930_e1204;

        let (assign940_e1208,) = {
    if (var_guard83 != 0.0) {
        (p.p103,)
    } else {
        (var_gco_i,)
    }
};
        var_gco_i = assign940_e1208;

        let (assign950_e1212,) = {
    if (var_guard83 != 0.0) {
        (p.p104,)
    } else {
        (var_iginv_t,)
    }
};
        var_iginv_t = assign950_e1212;

        let (assign960_e1216,) = {
    if (var_guard83 != 0.0) {
        (p.p105,)
    } else {
        (var_igovinv_t,)
    }
};
        var_igovinv_t = assign960_e1216;

        let (assign970_e1220,) = {
    if (var_guard83 != 0.0) {
        (p.p106,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign970_e1220;

        let (assign980_e1224,) = {
    if (var_guard83 != 0.0) {
        (p.p120,)
    } else {
        (var_fnovinv_t,)
    }
};
        var_fnovinv_t = assign980_e1224;

        let (assign990_e1228,) = {
    if (var_guard83 != 0.0) {
        (p.p121,)
    } else {
        (var_fnovinvd_t,)
    }
};
        var_fnovinvd_t = assign990_e1228;

        let (assign1000_e1232,) = {
    if (var_guard83 != 0.0) {
        (p.p107,)
    } else {
        (var_igovacc_t,)
    }
};
        var_igovacc_t = assign1000_e1232;

        let (assign1010_e1236,) = {
    if (var_guard83 != 0.0) {
        (p.p108,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign1010_e1236;

        let (assign1020_e1240,) = {
    if (var_guard83 != 0.0) {
        (p.p109,)
    } else {
        (var_stig_i,)
    }
};
        var_stig_i = assign1020_e1240;

        let (assign1030_e1244,) = {
    if (var_guard83 != 0.0) {
        (p.p123,)
    } else {
        (var_stigfn_i,)
    }
};
        var_stigfn_i = assign1030_e1244;

        let (assign1040_e1248,) = {
    if (var_guard83 != 0.0) {
        (p.p110,)
    } else {
        (var_gc2ch_i,)
    }
};
        var_gc2ch_i = assign1040_e1248;

        let (assign1050_e1252,) = {
    if (var_guard83 != 0.0) {
        (p.p111,)
    } else {
        (var_gc3ch_i,)
    }
};
        var_gc3ch_i = assign1050_e1252;

        let (assign1060_e1256,) = {
    if (var_guard83 != 0.0) {
        (p.p112,)
    } else {
        (var_gc2ovinv_i,)
    }
};
        var_gc2ovinv_i = assign1060_e1256;

        let (assign1070_e1260,) = {
    if (var_guard83 != 0.0) {
        (p.p122,)
    } else {
        (var_gcovinvfn_i,)
    }
};
        var_gcovinvfn_i = assign1070_e1260;

        let (assign1080_e1264,) = {
    if (var_guard83 != 0.0) {
        (p.p113,)
    } else {
        (var_gc3ovinv_i,)
    }
};
        var_gc3ovinv_i = assign1080_e1264;

        let (assign1090_e1268,) = {
    if (var_guard83 != 0.0) {
        (p.p114,)
    } else {
        (var_gc2ovacc_i,)
    }
};
        var_gc2ovacc_i = assign1090_e1268;

        let (assign1100_e1272,) = {
    if (var_guard83 != 0.0) {
        (p.p115,)
    } else {
        (var_gc3ovacc_i,)
    }
};
        var_gc3ovacc_i = assign1100_e1272;

        let (assign1110_e1276,) = {
    if (var_guard83 != 0.0) {
        (p.p116,)
    } else {
        (var_gcdov_i,)
    }
};
        var_gcdov_i = assign1110_e1276;

        let (assign1120_e1280,) = {
    if (var_guard83 != 0.0) {
        (p.p117,)
    } else {
        (var_gcvdov_i,)
    }
};
        var_gcvdov_i = assign1120_e1280;

        let (assign1130_e1284,) = {
    if (var_guard83 != 0.0) {
        (p.p118,)
    } else {
        (var_chib_i,)
    }
};
        var_chib_i = assign1130_e1284;

        let (assign1140_e1288,) = {
    if (var_guard83 != 0.0) {
        (p.p119,)
    } else {
        (var_niginv_i,)
    }
};
        var_niginv_i = assign1140_e1288;

        let (assign1150_e1292, assign1150_e1292_d_n0, assign1150_e1292_d_n1, assign1150_e1292_d_n2, assign1150_e1292_d_n3, assign1150_e1292_d_n4, assign1150_e1292_d_n5, assign1150_e1292_d_n6, assign1150_e1292_d_n7, assign1150_e1292_d_n8, assign1150_e1292_d_n9, assign1150_e1292_d_b0, assign1150_e1292_d_b1, assign1150_e1292_d_b2, assign1150_e1292_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p124, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidl_i, var_agidl_i_dn0, var_agidl_i_dn1, var_agidl_i_dn2, var_agidl_i_dn3, var_agidl_i_dn4, var_agidl_i_dn5, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9, var_agidl_i_db0, var_agidl_i_db1, var_agidl_i_db2, var_agidl_i_db3,)
    }
};
        var_agidl_i = assign1150_e1292;
        var_agidl_i_dn0 = assign1150_e1292_d_n0;
        var_agidl_i_dn1 = assign1150_e1292_d_n1;
        var_agidl_i_dn2 = assign1150_e1292_d_n2;
        var_agidl_i_dn3 = assign1150_e1292_d_n3;
        var_agidl_i_dn4 = assign1150_e1292_d_n4;
        var_agidl_i_dn5 = assign1150_e1292_d_n5;
        var_agidl_i_dn6 = assign1150_e1292_d_n6;
        var_agidl_i_dn7 = assign1150_e1292_d_n7;
        var_agidl_i_dn8 = assign1150_e1292_d_n8;
        var_agidl_i_dn9 = assign1150_e1292_d_n9;
        var_agidl_i_db0 = assign1150_e1292_d_b0;
        var_agidl_i_db1 = assign1150_e1292_d_b1;
        var_agidl_i_db2 = assign1150_e1292_d_b2;
        var_agidl_i_db3 = assign1150_e1292_d_b3;

        let (assign1160_e1296, assign1160_e1296_d_n0, assign1160_e1296_d_n1, assign1160_e1296_d_n2, assign1160_e1296_d_n3, assign1160_e1296_d_n4, assign1160_e1296_d_n5, assign1160_e1296_d_n6, assign1160_e1296_d_n7, assign1160_e1296_d_n8, assign1160_e1296_d_n9, assign1160_e1296_d_b0, assign1160_e1296_d_b1, assign1160_e1296_d_b2, assign1160_e1296_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p125, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidld_i, var_agidld_i_dn0, var_agidld_i_dn1, var_agidld_i_dn2, var_agidld_i_dn3, var_agidld_i_dn4, var_agidld_i_dn5, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9, var_agidld_i_db0, var_agidld_i_db1, var_agidld_i_db2, var_agidld_i_db3,)
    }
};
        var_agidld_i = assign1160_e1296;
        var_agidld_i_dn0 = assign1160_e1296_d_n0;
        var_agidld_i_dn1 = assign1160_e1296_d_n1;
        var_agidld_i_dn2 = assign1160_e1296_d_n2;
        var_agidld_i_dn3 = assign1160_e1296_d_n3;
        var_agidld_i_dn4 = assign1160_e1296_d_n4;
        var_agidld_i_dn5 = assign1160_e1296_d_n5;
        var_agidld_i_dn6 = assign1160_e1296_d_n6;
        var_agidld_i_dn7 = assign1160_e1296_d_n7;
        var_agidld_i_dn8 = assign1160_e1296_d_n8;
        var_agidld_i_dn9 = assign1160_e1296_d_n9;
        var_agidld_i_db0 = assign1160_e1296_d_b0;
        var_agidld_i_db1 = assign1160_e1296_d_b1;
        var_agidld_i_db2 = assign1160_e1296_d_b2;
        var_agidld_i_db3 = assign1160_e1296_d_b3;

        let (assign1170_e1300,) = {
    if (var_guard83 != 0.0) {
        (p.p126,)
    } else {
        (var_bgidl_t,)
    }
};
        var_bgidl_t = assign1170_e1300;

        let (assign1180_e1304,) = {
    if (var_guard83 != 0.0) {
        (p.p127,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign1180_e1304;

        let (assign1190_e1308,) = {
    if (var_guard83 != 0.0) {
        (p.p128,)
    } else {
        (var_stbgidl_i,)
    }
};
        var_stbgidl_i = assign1190_e1308;

        let (assign1200_e1312,) = {
    if (var_guard83 != 0.0) {
        (p.p129,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign1200_e1312;

        let (assign1210_e1316,) = {
    if (var_guard83 != 0.0) {
        (p.p130,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign1210_e1316;

        let (assign1220_e1320,) = {
    if (var_guard83 != 0.0) {
        (p.p131,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign1220_e1320;

        let (assign1230_e1324,) = {
    if (var_guard83 != 0.0) {
        (p.p132,)
    } else {
        (var_dgidl_i,)
    }
};
        var_dgidl_i = assign1230_e1324;

        let (assign1240_e1328,) = {
    if (var_guard83 != 0.0) {
        (p.p133,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign1240_e1328;

        let (assign1250_e1332,) = {
    if (var_guard83 != 0.0) {
        (p.p147,)
    } else {
        (var_a1_i,)
    }
};
        var_a1_i = assign1250_e1332;

        let (assign1260_e1336,) = {
    if (var_guard83 != 0.0) {
        (p.p148,)
    } else {
        (var_a2_t,)
    }
};
        var_a2_t = assign1260_e1336;

        let (assign1270_e1340,) = {
    if (var_guard83 != 0.0) {
        (p.p149,)
    } else {
        (var_sta2_i,)
    }
};
        var_sta2_i = assign1270_e1340;

        let (assign1280_e1344,) = {
    if (var_guard83 != 0.0) {
        (p.p150,)
    } else {
        (var_a3_i,)
    }
};
        var_a3_i = assign1280_e1344;

        let (assign1290_e1348,) = {
    if (var_guard83 != 0.0) {
        (p.p134,)
    } else {
        (var_ctedge_i,)
    }
};
        var_ctedge_i = assign1290_e1348;

        let (assign1300_e1352, assign1300_e1352_d_n0, assign1300_e1352_d_n1, assign1300_e1352_d_n2, assign1300_e1352_d_n3, assign1300_e1352_d_n4, assign1300_e1352_d_n5, assign1300_e1352_d_n6, assign1300_e1352_d_n7, assign1300_e1352_d_n8, assign1300_e1352_d_n9, assign1300_e1352_d_b0, assign1300_e1352_d_b1, assign1300_e1352_d_b2, assign1300_e1352_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfb1edge_t, var_vfb1edge_t_dn0, var_vfb1edge_t_dn1, var_vfb1edge_t_dn2, var_vfb1edge_t_dn3, var_vfb1edge_t_dn4, var_vfb1edge_t_dn5, var_vfb1edge_t_dn6, var_vfb1edge_t_dn7, var_vfb1edge_t_dn8, var_vfb1edge_t_dn9, var_vfb1edge_t_db0, var_vfb1edge_t_db1, var_vfb1edge_t_db2, var_vfb1edge_t_db3,)
    }
};
        var_vfb1edge_t = assign1300_e1352;
        var_vfb1edge_t_dn0 = assign1300_e1352_d_n0;
        var_vfb1edge_t_dn1 = assign1300_e1352_d_n1;
        var_vfb1edge_t_dn2 = assign1300_e1352_d_n2;
        var_vfb1edge_t_dn3 = assign1300_e1352_d_n3;
        var_vfb1edge_t_dn4 = assign1300_e1352_d_n4;
        var_vfb1edge_t_dn5 = assign1300_e1352_d_n5;
        var_vfb1edge_t_dn6 = assign1300_e1352_d_n6;
        var_vfb1edge_t_dn7 = assign1300_e1352_d_n7;
        var_vfb1edge_t_dn8 = assign1300_e1352_d_n8;
        var_vfb1edge_t_dn9 = assign1300_e1352_d_n9;
        var_vfb1edge_t_db0 = assign1300_e1352_d_b0;
        var_vfb1edge_t_db1 = assign1300_e1352_d_b1;
        var_vfb1edge_t_db2 = assign1300_e1352_d_b2;
        var_vfb1edge_t_db3 = assign1300_e1352_d_b3;

        let (assign1310_e1356,) = {
    if (var_guard83 != 0.0) {
        (p.p136,)
    } else {
        (var_vfb2edge_t,)
    }
};
        var_vfb2edge_t = assign1310_e1356;

        let (assign1320_e1360,) = {
    if (var_guard83 != 0.0) {
        (p.p137,)
    } else {
        (var_stvfbedge_i,)
    }
};
        var_stvfbedge_i = assign1320_e1360;

        let (assign1330_e1364,) = {
    if (var_guard83 != 0.0) {
        (p.p138,)
    } else {
        (var_cic1edge_i,)
    }
};
        var_cic1edge_i = assign1330_e1364;

        *var_a1_i_slot = var_a1_i;
        *var_a2_t_slot = var_a2_t;
        *var_a3_i_slot = var_a3_i;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_db0_slot = var_agidl_i_db0;
        *var_agidl_i_db1_slot = var_agidl_i_db1;
        *var_agidl_i_db2_slot = var_agidl_i_db2;
        *var_agidl_i_db3_slot = var_agidl_i_db3;
        *var_agidl_i_dn0_slot = var_agidl_i_dn0;
        *var_agidl_i_dn1_slot = var_agidl_i_dn1;
        *var_agidl_i_dn2_slot = var_agidl_i_dn2;
        *var_agidl_i_dn3_slot = var_agidl_i_dn3;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn5_slot = var_agidl_i_dn5;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_db0_slot = var_agidld_i_db0;
        *var_agidld_i_db1_slot = var_agidld_i_db1;
        *var_agidld_i_db2_slot = var_agidld_i_db2;
        *var_agidld_i_db3_slot = var_agidld_i_db3;
        *var_agidld_i_dn0_slot = var_agidld_i_dn0;
        *var_agidld_i_dn1_slot = var_agidld_i_dn1;
        *var_agidld_i_dn2_slot = var_agidld_i_dn2;
        *var_agidld_i_dn3_slot = var_agidld_i_dn3;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn5_slot = var_agidld_i_dn5;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp_i_slot = var_alp_i;
        *var_alpb_i_slot = var_alpb_i;
        *var_ax_i_slot = var_ax_i;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_chib_i_slot = var_chib_i;
        *var_cic1edge_i_slot = var_cic1edge_i;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_dgidl_i_slot = var_dgidl_i;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_fnovinv_t_slot = var_fnovinv_t;
        *var_fnovinvd_t_slot = var_fnovinvd_t;
        *var_gc2ch_i_slot = var_gc2ch_i;
        *var_gc2ovacc_i_slot = var_gc2ovacc_i;
        *var_gc2ovinv_i_slot = var_gc2ovinv_i;
        *var_gc3ch_i_slot = var_gc3ch_i;
        *var_gc3ovacc_i_slot = var_gc3ovacc_i;
        *var_gc3ovinv_i_slot = var_gc3ovinv_i;
        *var_gcdov_i_slot = var_gcdov_i;
        *var_gco_i_slot = var_gco_i;
        *var_gcovinvfn_i_slot = var_gcovinvfn_i;
        *var_gcvdov_i_slot = var_gcvdov_i;
        *var_iginv_t_slot = var_iginv_t;
        *var_igovacc_t_slot = var_igovacc_t;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovinv_t_slot = var_igovinv_t;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_niginv_i_slot = var_niginv_i;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stig_i_slot = var_stig_i;
        *var_stigfn_i_slot = var_stigfn_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_thesat1_i_slot = var_thesat1_i;
        *var_thesat2_i_slot = var_thesat2_i;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_db0_slot = var_thesat_t_db0;
        *var_thesat_t_db1_slot = var_thesat_t_db1;
        *var_thesat_t_db2_slot = var_thesat_t_db2;
        *var_thesat_t_db3_slot = var_thesat_t_db3;
        *var_thesat_t_dn0_slot = var_thesat_t_dn0;
        *var_thesat_t_dn1_slot = var_thesat_t_dn1;
        *var_thesat_t_dn2_slot = var_thesat_t_dn2;
        *var_thesat_t_dn3_slot = var_thesat_t_dn3;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn5_slot = var_thesat_t_dn5;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_vfb1edge_t_slot = var_vfb1edge_t;
        *var_vfb1edge_t_db0_slot = var_vfb1edge_t_db0;
        *var_vfb1edge_t_db1_slot = var_vfb1edge_t_db1;
        *var_vfb1edge_t_db2_slot = var_vfb1edge_t_db2;
        *var_vfb1edge_t_db3_slot = var_vfb1edge_t_db3;
        *var_vfb1edge_t_dn0_slot = var_vfb1edge_t_dn0;
        *var_vfb1edge_t_dn1_slot = var_vfb1edge_t_dn1;
        *var_vfb1edge_t_dn2_slot = var_vfb1edge_t_dn2;
        *var_vfb1edge_t_dn3_slot = var_vfb1edge_t_dn3;
        *var_vfb1edge_t_dn4_slot = var_vfb1edge_t_dn4;
        *var_vfb1edge_t_dn5_slot = var_vfb1edge_t_dn5;
        *var_vfb1edge_t_dn6_slot = var_vfb1edge_t_dn6;
        *var_vfb1edge_t_dn7_slot = var_vfb1edge_t_dn7;
        *var_vfb1edge_t_dn8_slot = var_vfb1edge_t_dn8;
        *var_vfb1edge_t_dn9_slot = var_vfb1edge_t_dn9;
        *var_vfb2edge_t_slot = var_vfb2edge_t;
        *var_vp_i_slot = var_vp_i;
        *var_vpg_i_slot = var_vpg_i;
    }

    pub(super) fn stamp_transient_block_3(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_db0: f64,
        var_cf1_t_db1: f64,
        var_cf1_t_db2: f64,
        var_cf1_t_db3: f64,
        var_cf1_t_dn0: f64,
        var_cf1_t_dn1: f64,
        var_cf1_t_dn2: f64,
        var_cf1_t_dn3: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn5: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_db0: f64,
        var_cf2_t_db1: f64,
        var_cf2_t_db2: f64,
        var_cf2_t_db3: f64,
        var_cf2_t_dn0: f64,
        var_cf2_t_dn1: f64,
        var_cf2_t_dn2: f64,
        var_cf2_t_dn3: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn5: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_guard83: f64,
        var_psce1_i: f64,
        var_psce2_i: f64,
        var_thesat_t: f64,
        var_thesat_t_db0: f64,
        var_thesat_t_db1: f64,
        var_thesat_t_db2: f64,
        var_thesat_t_db3: f64,
        var_thesat_t_dn0: f64,
        var_thesat_t_dn1: f64,
        var_thesat_t_dn2: f64,
        var_thesat_t_dn3: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn5: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_vfb1_t: f64,
        var_vfb1_t_db0: f64,
        var_vfb1_t_db1: f64,
        var_vfb1_t_db2: f64,
        var_vfb1_t_db3: f64,
        var_vfb1_t_dn0: f64,
        var_vfb1_t_dn1: f64,
        var_vfb1_t_dn2: f64,
        var_vfb1_t_dn3: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn5: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_db0: f64,
        var_vfb2_t_db1: f64,
        var_vfb2_t_db2: f64,
        var_vfb2_t_db3: f64,
        var_vfb2_t_dn0: f64,
        var_vfb2_t_dn1: f64,
        var_vfb2_t_dn2: f64,
        var_vfb2_t_dn3: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn5: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_alpac_i_slot: &mut f64,
        var_areaq_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_db0_slot: &mut f64,
        var_betnedge_t_db1_slot: &mut f64,
        var_betnedge_t_db2_slot: &mut f64,
        var_betnedge_t_db3_slot: &mut f64,
        var_betnedge_t_dn0_slot: &mut f64,
        var_betnedge_t_dn1_slot: &mut f64,
        var_betnedge_t_dn2_slot: &mut f64,
        var_betnedge_t_dn3_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_dn5_slot: &mut f64,
        var_betnedge_t_dn6_slot: &mut f64,
        var_betnedge_t_dn7_slot: &mut f64,
        var_betnedge_t_dn8_slot: &mut f64,
        var_betnedge_t_dn9_slot: &mut f64,
        var_cf1edge_i_slot: &mut f64,
        var_cf1edge_i_db0_slot: &mut f64,
        var_cf1edge_i_db1_slot: &mut f64,
        var_cf1edge_i_db2_slot: &mut f64,
        var_cf1edge_i_db3_slot: &mut f64,
        var_cf1edge_i_dn0_slot: &mut f64,
        var_cf1edge_i_dn1_slot: &mut f64,
        var_cf1edge_i_dn2_slot: &mut f64,
        var_cf1edge_i_dn3_slot: &mut f64,
        var_cf1edge_i_dn4_slot: &mut f64,
        var_cf1edge_i_dn5_slot: &mut f64,
        var_cf1edge_i_dn6_slot: &mut f64,
        var_cf1edge_i_dn7_slot: &mut f64,
        var_cf1edge_i_dn8_slot: &mut f64,
        var_cf1edge_i_dn9_slot: &mut f64,
        var_cf2edge_i_slot: &mut f64,
        var_cf2edge_i_db0_slot: &mut f64,
        var_cf2edge_i_db1_slot: &mut f64,
        var_cf2edge_i_db2_slot: &mut f64,
        var_cf2edge_i_db3_slot: &mut f64,
        var_cf2edge_i_dn0_slot: &mut f64,
        var_cf2edge_i_dn1_slot: &mut f64,
        var_cf2edge_i_dn2_slot: &mut f64,
        var_cf2edge_i_dn3_slot: &mut f64,
        var_cf2edge_i_dn4_slot: &mut f64,
        var_cf2edge_i_dn5_slot: &mut f64,
        var_cf2edge_i_dn6_slot: &mut f64,
        var_cf2edge_i_dn7_slot: &mut f64,
        var_cf2edge_i_dn8_slot: &mut f64,
        var_cf2edge_i_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_db0_slot: &mut f64,
        var_cfac1_t_db1_slot: &mut f64,
        var_cfac1_t_db2_slot: &mut f64,
        var_cfac1_t_db3_slot: &mut f64,
        var_cfac1_t_dn0_slot: &mut f64,
        var_cfac1_t_dn1_slot: &mut f64,
        var_cfac1_t_dn2_slot: &mut f64,
        var_cfac1_t_dn3_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn5_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_db0_slot: &mut f64,
        var_cfac2_t_db1_slot: &mut f64,
        var_cfac2_t_db2_slot: &mut f64,
        var_cfac2_t_db3_slot: &mut f64,
        var_cfac2_t_dn0_slot: &mut f64,
        var_cfac2_t_dn1_slot: &mut f64,
        var_cfac2_t_dn2_slot: &mut f64,
        var_cfac2_t_dn3_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn5_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_db0_slot: &mut f64,
        var_cgbov_i_db1_slot: &mut f64,
        var_cgbov_i_db2_slot: &mut f64,
        var_cgbov_i_db3_slot: &mut f64,
        var_cgbov_i_dn0_slot: &mut f64,
        var_cgbov_i_dn1_slot: &mut f64,
        var_cgbov_i_dn2_slot: &mut f64,
        var_cgbov_i_dn3_slot: &mut f64,
        var_cgbov_i_dn4_slot: &mut f64,
        var_cgbov_i_dn5_slot: &mut f64,
        var_cgbov_i_dn6_slot: &mut f64,
        var_cgbov_i_dn7_slot: &mut f64,
        var_cgbov_i_dn8_slot: &mut f64,
        var_cgbov_i_dn9_slot: &mut f64,
        var_cic2edge_i_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_nsdac_i_slot: &mut f64,
        var_psce1edge_i_slot: &mut f64,
        var_psce1edge_i_db0_slot: &mut f64,
        var_psce1edge_i_db1_slot: &mut f64,
        var_psce1edge_i_db2_slot: &mut f64,
        var_psce1edge_i_db3_slot: &mut f64,
        var_psce1edge_i_dn0_slot: &mut f64,
        var_psce1edge_i_dn1_slot: &mut f64,
        var_psce1edge_i_dn2_slot: &mut f64,
        var_psce1edge_i_dn3_slot: &mut f64,
        var_psce1edge_i_dn4_slot: &mut f64,
        var_psce1edge_i_dn5_slot: &mut f64,
        var_psce1edge_i_dn6_slot: &mut f64,
        var_psce1edge_i_dn7_slot: &mut f64,
        var_psce1edge_i_dn8_slot: &mut f64,
        var_psce1edge_i_dn9_slot: &mut f64,
        var_psce2edge_i_slot: &mut f64,
        var_psce2edge_i_db0_slot: &mut f64,
        var_psce2edge_i_db1_slot: &mut f64,
        var_psce2edge_i_db2_slot: &mut f64,
        var_psce2edge_i_db3_slot: &mut f64,
        var_psce2edge_i_dn0_slot: &mut f64,
        var_psce2edge_i_dn1_slot: &mut f64,
        var_psce2edge_i_dn2_slot: &mut f64,
        var_psce2edge_i_dn3_slot: &mut f64,
        var_psce2edge_i_dn4_slot: &mut f64,
        var_psce2edge_i_dn5_slot: &mut f64,
        var_psce2edge_i_dn6_slot: &mut f64,
        var_psce2edge_i_dn7_slot: &mut f64,
        var_psce2edge_i_dn8_slot: &mut f64,
        var_psce2edge_i_dn9_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_db0_slot: &mut f64,
        var_thesatac_t_db1_slot: &mut f64,
        var_thesatac_t_db2_slot: &mut f64,
        var_thesatac_t_db3_slot: &mut f64,
        var_thesatac_t_dn0_slot: &mut f64,
        var_thesatac_t_dn1_slot: &mut f64,
        var_thesatac_t_dn2_slot: &mut f64,
        var_thesatac_t_dn3_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn5_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_db0_slot: &mut f64,
        var_vfbac1_t_db1_slot: &mut f64,
        var_vfbac1_t_db2_slot: &mut f64,
        var_vfbac1_t_db3_slot: &mut f64,
        var_vfbac1_t_dn0_slot: &mut f64,
        var_vfbac1_t_dn1_slot: &mut f64,
        var_vfbac1_t_dn2_slot: &mut f64,
        var_vfbac1_t_dn3_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn5_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_db0_slot: &mut f64,
        var_vfbac2_t_db1_slot: &mut f64,
        var_vfbac2_t_db2_slot: &mut f64,
        var_vfbac2_t_db3_slot: &mut f64,
        var_vfbac2_t_dn0_slot: &mut f64,
        var_vfbac2_t_dn1_slot: &mut f64,
        var_vfbac2_t_dn2_slot: &mut f64,
        var_vfbac2_t_dn3_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn5_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_areaq_i: f64 = *var_areaq_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_db0: f64 = *var_betnedge_t_db0_slot;
        let mut var_betnedge_t_db1: f64 = *var_betnedge_t_db1_slot;
        let mut var_betnedge_t_db2: f64 = *var_betnedge_t_db2_slot;
        let mut var_betnedge_t_db3: f64 = *var_betnedge_t_db3_slot;
        let mut var_betnedge_t_dn0: f64 = *var_betnedge_t_dn0_slot;
        let mut var_betnedge_t_dn1: f64 = *var_betnedge_t_dn1_slot;
        let mut var_betnedge_t_dn2: f64 = *var_betnedge_t_dn2_slot;
        let mut var_betnedge_t_dn3: f64 = *var_betnedge_t_dn3_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_dn5: f64 = *var_betnedge_t_dn5_slot;
        let mut var_betnedge_t_dn6: f64 = *var_betnedge_t_dn6_slot;
        let mut var_betnedge_t_dn7: f64 = *var_betnedge_t_dn7_slot;
        let mut var_betnedge_t_dn8: f64 = *var_betnedge_t_dn8_slot;
        let mut var_betnedge_t_dn9: f64 = *var_betnedge_t_dn9_slot;
        let mut var_cf1edge_i: f64 = *var_cf1edge_i_slot;
        let mut var_cf1edge_i_db0: f64 = *var_cf1edge_i_db0_slot;
        let mut var_cf1edge_i_db1: f64 = *var_cf1edge_i_db1_slot;
        let mut var_cf1edge_i_db2: f64 = *var_cf1edge_i_db2_slot;
        let mut var_cf1edge_i_db3: f64 = *var_cf1edge_i_db3_slot;
        let mut var_cf1edge_i_dn0: f64 = *var_cf1edge_i_dn0_slot;
        let mut var_cf1edge_i_dn1: f64 = *var_cf1edge_i_dn1_slot;
        let mut var_cf1edge_i_dn2: f64 = *var_cf1edge_i_dn2_slot;
        let mut var_cf1edge_i_dn3: f64 = *var_cf1edge_i_dn3_slot;
        let mut var_cf1edge_i_dn4: f64 = *var_cf1edge_i_dn4_slot;
        let mut var_cf1edge_i_dn5: f64 = *var_cf1edge_i_dn5_slot;
        let mut var_cf1edge_i_dn6: f64 = *var_cf1edge_i_dn6_slot;
        let mut var_cf1edge_i_dn7: f64 = *var_cf1edge_i_dn7_slot;
        let mut var_cf1edge_i_dn8: f64 = *var_cf1edge_i_dn8_slot;
        let mut var_cf1edge_i_dn9: f64 = *var_cf1edge_i_dn9_slot;
        let mut var_cf2edge_i: f64 = *var_cf2edge_i_slot;
        let mut var_cf2edge_i_db0: f64 = *var_cf2edge_i_db0_slot;
        let mut var_cf2edge_i_db1: f64 = *var_cf2edge_i_db1_slot;
        let mut var_cf2edge_i_db2: f64 = *var_cf2edge_i_db2_slot;
        let mut var_cf2edge_i_db3: f64 = *var_cf2edge_i_db3_slot;
        let mut var_cf2edge_i_dn0: f64 = *var_cf2edge_i_dn0_slot;
        let mut var_cf2edge_i_dn1: f64 = *var_cf2edge_i_dn1_slot;
        let mut var_cf2edge_i_dn2: f64 = *var_cf2edge_i_dn2_slot;
        let mut var_cf2edge_i_dn3: f64 = *var_cf2edge_i_dn3_slot;
        let mut var_cf2edge_i_dn4: f64 = *var_cf2edge_i_dn4_slot;
        let mut var_cf2edge_i_dn5: f64 = *var_cf2edge_i_dn5_slot;
        let mut var_cf2edge_i_dn6: f64 = *var_cf2edge_i_dn6_slot;
        let mut var_cf2edge_i_dn7: f64 = *var_cf2edge_i_dn7_slot;
        let mut var_cf2edge_i_dn8: f64 = *var_cf2edge_i_dn8_slot;
        let mut var_cf2edge_i_dn9: f64 = *var_cf2edge_i_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_db0: f64 = *var_cfac1_t_db0_slot;
        let mut var_cfac1_t_db1: f64 = *var_cfac1_t_db1_slot;
        let mut var_cfac1_t_db2: f64 = *var_cfac1_t_db2_slot;
        let mut var_cfac1_t_db3: f64 = *var_cfac1_t_db3_slot;
        let mut var_cfac1_t_dn0: f64 = *var_cfac1_t_dn0_slot;
        let mut var_cfac1_t_dn1: f64 = *var_cfac1_t_dn1_slot;
        let mut var_cfac1_t_dn2: f64 = *var_cfac1_t_dn2_slot;
        let mut var_cfac1_t_dn3: f64 = *var_cfac1_t_dn3_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn5: f64 = *var_cfac1_t_dn5_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_db0: f64 = *var_cfac2_t_db0_slot;
        let mut var_cfac2_t_db1: f64 = *var_cfac2_t_db1_slot;
        let mut var_cfac2_t_db2: f64 = *var_cfac2_t_db2_slot;
        let mut var_cfac2_t_db3: f64 = *var_cfac2_t_db3_slot;
        let mut var_cfac2_t_dn0: f64 = *var_cfac2_t_dn0_slot;
        let mut var_cfac2_t_dn1: f64 = *var_cfac2_t_dn1_slot;
        let mut var_cfac2_t_dn2: f64 = *var_cfac2_t_dn2_slot;
        let mut var_cfac2_t_dn3: f64 = *var_cfac2_t_dn3_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn5: f64 = *var_cfac2_t_dn5_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_db0: f64 = *var_cgbov_i_db0_slot;
        let mut var_cgbov_i_db1: f64 = *var_cgbov_i_db1_slot;
        let mut var_cgbov_i_db2: f64 = *var_cgbov_i_db2_slot;
        let mut var_cgbov_i_db3: f64 = *var_cgbov_i_db3_slot;
        let mut var_cgbov_i_dn0: f64 = *var_cgbov_i_dn0_slot;
        let mut var_cgbov_i_dn1: f64 = *var_cgbov_i_dn1_slot;
        let mut var_cgbov_i_dn2: f64 = *var_cgbov_i_dn2_slot;
        let mut var_cgbov_i_dn3: f64 = *var_cgbov_i_dn3_slot;
        let mut var_cgbov_i_dn4: f64 = *var_cgbov_i_dn4_slot;
        let mut var_cgbov_i_dn5: f64 = *var_cgbov_i_dn5_slot;
        let mut var_cgbov_i_dn6: f64 = *var_cgbov_i_dn6_slot;
        let mut var_cgbov_i_dn7: f64 = *var_cgbov_i_dn7_slot;
        let mut var_cgbov_i_dn8: f64 = *var_cgbov_i_dn8_slot;
        let mut var_cgbov_i_dn9: f64 = *var_cgbov_i_dn9_slot;
        let mut var_cic2edge_i: f64 = *var_cic2edge_i_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_nsdac_i: f64 = *var_nsdac_i_slot;
        let mut var_psce1edge_i: f64 = *var_psce1edge_i_slot;
        let mut var_psce1edge_i_db0: f64 = *var_psce1edge_i_db0_slot;
        let mut var_psce1edge_i_db1: f64 = *var_psce1edge_i_db1_slot;
        let mut var_psce1edge_i_db2: f64 = *var_psce1edge_i_db2_slot;
        let mut var_psce1edge_i_db3: f64 = *var_psce1edge_i_db3_slot;
        let mut var_psce1edge_i_dn0: f64 = *var_psce1edge_i_dn0_slot;
        let mut var_psce1edge_i_dn1: f64 = *var_psce1edge_i_dn1_slot;
        let mut var_psce1edge_i_dn2: f64 = *var_psce1edge_i_dn2_slot;
        let mut var_psce1edge_i_dn3: f64 = *var_psce1edge_i_dn3_slot;
        let mut var_psce1edge_i_dn4: f64 = *var_psce1edge_i_dn4_slot;
        let mut var_psce1edge_i_dn5: f64 = *var_psce1edge_i_dn5_slot;
        let mut var_psce1edge_i_dn6: f64 = *var_psce1edge_i_dn6_slot;
        let mut var_psce1edge_i_dn7: f64 = *var_psce1edge_i_dn7_slot;
        let mut var_psce1edge_i_dn8: f64 = *var_psce1edge_i_dn8_slot;
        let mut var_psce1edge_i_dn9: f64 = *var_psce1edge_i_dn9_slot;
        let mut var_psce2edge_i: f64 = *var_psce2edge_i_slot;
        let mut var_psce2edge_i_db0: f64 = *var_psce2edge_i_db0_slot;
        let mut var_psce2edge_i_db1: f64 = *var_psce2edge_i_db1_slot;
        let mut var_psce2edge_i_db2: f64 = *var_psce2edge_i_db2_slot;
        let mut var_psce2edge_i_db3: f64 = *var_psce2edge_i_db3_slot;
        let mut var_psce2edge_i_dn0: f64 = *var_psce2edge_i_dn0_slot;
        let mut var_psce2edge_i_dn1: f64 = *var_psce2edge_i_dn1_slot;
        let mut var_psce2edge_i_dn2: f64 = *var_psce2edge_i_dn2_slot;
        let mut var_psce2edge_i_dn3: f64 = *var_psce2edge_i_dn3_slot;
        let mut var_psce2edge_i_dn4: f64 = *var_psce2edge_i_dn4_slot;
        let mut var_psce2edge_i_dn5: f64 = *var_psce2edge_i_dn5_slot;
        let mut var_psce2edge_i_dn6: f64 = *var_psce2edge_i_dn6_slot;
        let mut var_psce2edge_i_dn7: f64 = *var_psce2edge_i_dn7_slot;
        let mut var_psce2edge_i_dn8: f64 = *var_psce2edge_i_dn8_slot;
        let mut var_psce2edge_i_dn9: f64 = *var_psce2edge_i_dn9_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_db0: f64 = *var_thesatac_t_db0_slot;
        let mut var_thesatac_t_db1: f64 = *var_thesatac_t_db1_slot;
        let mut var_thesatac_t_db2: f64 = *var_thesatac_t_db2_slot;
        let mut var_thesatac_t_db3: f64 = *var_thesatac_t_db3_slot;
        let mut var_thesatac_t_dn0: f64 = *var_thesatac_t_dn0_slot;
        let mut var_thesatac_t_dn1: f64 = *var_thesatac_t_dn1_slot;
        let mut var_thesatac_t_dn2: f64 = *var_thesatac_t_dn2_slot;
        let mut var_thesatac_t_dn3: f64 = *var_thesatac_t_dn3_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn5: f64 = *var_thesatac_t_dn5_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_db0: f64 = *var_vfbac1_t_db0_slot;
        let mut var_vfbac1_t_db1: f64 = *var_vfbac1_t_db1_slot;
        let mut var_vfbac1_t_db2: f64 = *var_vfbac1_t_db2_slot;
        let mut var_vfbac1_t_db3: f64 = *var_vfbac1_t_db3_slot;
        let mut var_vfbac1_t_dn0: f64 = *var_vfbac1_t_dn0_slot;
        let mut var_vfbac1_t_dn1: f64 = *var_vfbac1_t_dn1_slot;
        let mut var_vfbac1_t_dn2: f64 = *var_vfbac1_t_dn2_slot;
        let mut var_vfbac1_t_dn3: f64 = *var_vfbac1_t_dn3_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn5: f64 = *var_vfbac1_t_dn5_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_db0: f64 = *var_vfbac2_t_db0_slot;
        let mut var_vfbac2_t_db1: f64 = *var_vfbac2_t_db1_slot;
        let mut var_vfbac2_t_db2: f64 = *var_vfbac2_t_db2_slot;
        let mut var_vfbac2_t_db3: f64 = *var_vfbac2_t_db3_slot;
        let mut var_vfbac2_t_dn0: f64 = *var_vfbac2_t_dn0_slot;
        let mut var_vfbac2_t_dn1: f64 = *var_vfbac2_t_dn1_slot;
        let mut var_vfbac2_t_dn2: f64 = *var_vfbac2_t_dn2_slot;
        let mut var_vfbac2_t_dn3: f64 = *var_vfbac2_t_dn3_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn5: f64 = *var_vfbac2_t_dn5_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign1340_e1368,) = {
    if (var_guard83 != 0.0) {
        (p.p139,)
    } else {
        (var_cic2edge_i,)
    }
};
        var_cic2edge_i = assign1340_e1368;

        let (assign1350_e1372, assign1350_e1372_d_n0, assign1350_e1372_d_n1, assign1350_e1372_d_n2, assign1350_e1372_d_n3, assign1350_e1372_d_n4, assign1350_e1372_d_n5, assign1350_e1372_d_n6, assign1350_e1372_d_n7, assign1350_e1372_d_n8, assign1350_e1372_d_n9, assign1350_e1372_d_b0, assign1350_e1372_d_b1, assign1350_e1372_d_b2, assign1350_e1372_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p140, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_psce1edge_i, var_psce1edge_i_dn0, var_psce1edge_i_dn1, var_psce1edge_i_dn2, var_psce1edge_i_dn3, var_psce1edge_i_dn4, var_psce1edge_i_dn5, var_psce1edge_i_dn6, var_psce1edge_i_dn7, var_psce1edge_i_dn8, var_psce1edge_i_dn9, var_psce1edge_i_db0, var_psce1edge_i_db1, var_psce1edge_i_db2, var_psce1edge_i_db3,)
    }
};
        var_psce1edge_i = assign1350_e1372;
        var_psce1edge_i_dn0 = assign1350_e1372_d_n0;
        var_psce1edge_i_dn1 = assign1350_e1372_d_n1;
        var_psce1edge_i_dn2 = assign1350_e1372_d_n2;
        var_psce1edge_i_dn3 = assign1350_e1372_d_n3;
        var_psce1edge_i_dn4 = assign1350_e1372_d_n4;
        var_psce1edge_i_dn5 = assign1350_e1372_d_n5;
        var_psce1edge_i_dn6 = assign1350_e1372_d_n6;
        var_psce1edge_i_dn7 = assign1350_e1372_d_n7;
        var_psce1edge_i_dn8 = assign1350_e1372_d_n8;
        var_psce1edge_i_dn9 = assign1350_e1372_d_n9;
        var_psce1edge_i_db0 = assign1350_e1372_d_b0;
        var_psce1edge_i_db1 = assign1350_e1372_d_b1;
        var_psce1edge_i_db2 = assign1350_e1372_d_b2;
        var_psce1edge_i_db3 = assign1350_e1372_d_b3;

        let (assign1360_e1382, assign1360_e1382_d_n0, assign1360_e1382_d_n1, assign1360_e1382_d_n2, assign1360_e1382_d_n3, assign1360_e1382_d_n4, assign1360_e1382_d_n5, assign1360_e1382_d_n6, assign1360_e1382_d_n7, assign1360_e1382_d_n8, assign1360_e1382_d_n9, assign1360_e1382_d_b0, assign1360_e1382_d_b1, assign1360_e1382_d_b2, assign1360_e1382_d_b3,) = {
    if (var_guard83 != 0.0) {
        let assign1360_e1376: f64 = (p.p141 * var_psce1edge_i);
        let assign1360_e1378: f64 = (assign1360_e1376 * var_tox2_i);
        let assign1360_e1380: f64 = (assign1360_e1378 / var_tox1_i);
        (assign1360_e1380, (((p.p141 * var_psce1edge_i_dn0) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn1) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn2) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn3) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn5) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_dn9) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_db0) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_db1) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_db2) * var_tox2_i) / var_tox1_i), (((p.p141 * var_psce1edge_i_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_psce2edge_i, var_psce2edge_i_dn0, var_psce2edge_i_dn1, var_psce2edge_i_dn2, var_psce2edge_i_dn3, var_psce2edge_i_dn4, var_psce2edge_i_dn5, var_psce2edge_i_dn6, var_psce2edge_i_dn7, var_psce2edge_i_dn8, var_psce2edge_i_dn9, var_psce2edge_i_db0, var_psce2edge_i_db1, var_psce2edge_i_db2, var_psce2edge_i_db3,)
    }
};
        var_psce2edge_i = assign1360_e1382;
        var_psce2edge_i_dn0 = assign1360_e1382_d_n0;
        var_psce2edge_i_dn1 = assign1360_e1382_d_n1;
        var_psce2edge_i_dn2 = assign1360_e1382_d_n2;
        var_psce2edge_i_dn3 = assign1360_e1382_d_n3;
        var_psce2edge_i_dn4 = assign1360_e1382_d_n4;
        var_psce2edge_i_dn5 = assign1360_e1382_d_n5;
        var_psce2edge_i_dn6 = assign1360_e1382_d_n6;
        var_psce2edge_i_dn7 = assign1360_e1382_d_n7;
        var_psce2edge_i_dn8 = assign1360_e1382_d_n8;
        var_psce2edge_i_dn9 = assign1360_e1382_d_n9;
        var_psce2edge_i_db0 = assign1360_e1382_d_b0;
        var_psce2edge_i_db1 = assign1360_e1382_d_b1;
        var_psce2edge_i_db2 = assign1360_e1382_d_b2;
        var_psce2edge_i_db3 = assign1360_e1382_d_b3;

        let (assign1370_e1386, assign1370_e1386_d_n0, assign1370_e1386_d_n1, assign1370_e1386_d_n2, assign1370_e1386_d_n3, assign1370_e1386_d_n4, assign1370_e1386_d_n5, assign1370_e1386_d_n6, assign1370_e1386_d_n7, assign1370_e1386_d_n8, assign1370_e1386_d_n9, assign1370_e1386_d_b0, assign1370_e1386_d_b1, assign1370_e1386_d_b2, assign1370_e1386_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p142, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cf1edge_i, var_cf1edge_i_dn0, var_cf1edge_i_dn1, var_cf1edge_i_dn2, var_cf1edge_i_dn3, var_cf1edge_i_dn4, var_cf1edge_i_dn5, var_cf1edge_i_dn6, var_cf1edge_i_dn7, var_cf1edge_i_dn8, var_cf1edge_i_dn9, var_cf1edge_i_db0, var_cf1edge_i_db1, var_cf1edge_i_db2, var_cf1edge_i_db3,)
    }
};
        var_cf1edge_i = assign1370_e1386;
        var_cf1edge_i_dn0 = assign1370_e1386_d_n0;
        var_cf1edge_i_dn1 = assign1370_e1386_d_n1;
        var_cf1edge_i_dn2 = assign1370_e1386_d_n2;
        var_cf1edge_i_dn3 = assign1370_e1386_d_n3;
        var_cf1edge_i_dn4 = assign1370_e1386_d_n4;
        var_cf1edge_i_dn5 = assign1370_e1386_d_n5;
        var_cf1edge_i_dn6 = assign1370_e1386_d_n6;
        var_cf1edge_i_dn7 = assign1370_e1386_d_n7;
        var_cf1edge_i_dn8 = assign1370_e1386_d_n8;
        var_cf1edge_i_dn9 = assign1370_e1386_d_n9;
        var_cf1edge_i_db0 = assign1370_e1386_d_b0;
        var_cf1edge_i_db1 = assign1370_e1386_d_b1;
        var_cf1edge_i_db2 = assign1370_e1386_d_b2;
        var_cf1edge_i_db3 = assign1370_e1386_d_b3;

        let (assign1380_e1396, assign1380_e1396_d_n0, assign1380_e1396_d_n1, assign1380_e1396_d_n2, assign1380_e1396_d_n3, assign1380_e1396_d_n4, assign1380_e1396_d_n5, assign1380_e1396_d_n6, assign1380_e1396_d_n7, assign1380_e1396_d_n8, assign1380_e1396_d_n9, assign1380_e1396_d_b0, assign1380_e1396_d_b1, assign1380_e1396_d_b2, assign1380_e1396_d_b3,) = {
    if (var_guard83 != 0.0) {
        let assign1380_e1390: f64 = (p.p143 * var_cf1edge_i);
        let assign1380_e1392: f64 = (assign1380_e1390 * var_tox2_i);
        let assign1380_e1394: f64 = (assign1380_e1392 / var_tox1_i);
        (assign1380_e1394, (((p.p143 * var_cf1edge_i_dn0) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn1) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn2) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn3) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn5) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_dn9) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_db0) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_db1) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_db2) * var_tox2_i) / var_tox1_i), (((p.p143 * var_cf1edge_i_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2edge_i, var_cf2edge_i_dn0, var_cf2edge_i_dn1, var_cf2edge_i_dn2, var_cf2edge_i_dn3, var_cf2edge_i_dn4, var_cf2edge_i_dn5, var_cf2edge_i_dn6, var_cf2edge_i_dn7, var_cf2edge_i_dn8, var_cf2edge_i_dn9, var_cf2edge_i_db0, var_cf2edge_i_db1, var_cf2edge_i_db2, var_cf2edge_i_db3,)
    }
};
        var_cf2edge_i = assign1380_e1396;
        var_cf2edge_i_dn0 = assign1380_e1396_d_n0;
        var_cf2edge_i_dn1 = assign1380_e1396_d_n1;
        var_cf2edge_i_dn2 = assign1380_e1396_d_n2;
        var_cf2edge_i_dn3 = assign1380_e1396_d_n3;
        var_cf2edge_i_dn4 = assign1380_e1396_d_n4;
        var_cf2edge_i_dn5 = assign1380_e1396_d_n5;
        var_cf2edge_i_dn6 = assign1380_e1396_d_n6;
        var_cf2edge_i_dn7 = assign1380_e1396_d_n7;
        var_cf2edge_i_dn8 = assign1380_e1396_d_n8;
        var_cf2edge_i_dn9 = assign1380_e1396_d_n9;
        var_cf2edge_i_db0 = assign1380_e1396_d_b0;
        var_cf2edge_i_db1 = assign1380_e1396_d_b1;
        var_cf2edge_i_db2 = assign1380_e1396_d_b2;
        var_cf2edge_i_db3 = assign1380_e1396_d_b3;

        let (assign1390_e1400,) = {
    if (var_guard83 != 0.0) {
        (p.p144,)
    } else {
        (var_cfdedge_i,)
    }
};
        var_cfdedge_i = assign1390_e1400;

        let (assign1400_e1404, assign1400_e1404_d_n0, assign1400_e1404_d_n1, assign1400_e1404_d_n2, assign1400_e1404_d_n3, assign1400_e1404_d_n4, assign1400_e1404_d_n5, assign1400_e1404_d_n6, assign1400_e1404_d_n7, assign1400_e1404_d_n8, assign1400_e1404_d_n9, assign1400_e1404_d_b0, assign1400_e1404_d_b1, assign1400_e1404_d_b2, assign1400_e1404_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p145, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_betnedge_t, var_betnedge_t_dn0, var_betnedge_t_dn1, var_betnedge_t_dn2, var_betnedge_t_dn3, var_betnedge_t_dn4, var_betnedge_t_dn5, var_betnedge_t_dn6, var_betnedge_t_dn7, var_betnedge_t_dn8, var_betnedge_t_dn9, var_betnedge_t_db0, var_betnedge_t_db1, var_betnedge_t_db2, var_betnedge_t_db3,)
    }
};
        var_betnedge_t = assign1400_e1404;
        var_betnedge_t_dn0 = assign1400_e1404_d_n0;
        var_betnedge_t_dn1 = assign1400_e1404_d_n1;
        var_betnedge_t_dn2 = assign1400_e1404_d_n2;
        var_betnedge_t_dn3 = assign1400_e1404_d_n3;
        var_betnedge_t_dn4 = assign1400_e1404_d_n4;
        var_betnedge_t_dn5 = assign1400_e1404_d_n5;
        var_betnedge_t_dn6 = assign1400_e1404_d_n6;
        var_betnedge_t_dn7 = assign1400_e1404_d_n7;
        var_betnedge_t_dn8 = assign1400_e1404_d_n8;
        var_betnedge_t_dn9 = assign1400_e1404_d_n9;
        var_betnedge_t_db0 = assign1400_e1404_d_b0;
        var_betnedge_t_db1 = assign1400_e1404_d_b1;
        var_betnedge_t_db2 = assign1400_e1404_d_b2;
        var_betnedge_t_db3 = assign1400_e1404_d_b3;

        let (assign1410_e1408,) = {
    if (var_guard83 != 0.0) {
        (p.p146,)
    } else {
        (var_stbetedge_i,)
    }
};
        var_stbetedge_i = assign1410_e1408;

        let (assign1420_e1412,) = {
    if (var_guard83 != 0.0) {
        (p.p151,)
    } else {
        (var_areaq_i,)
    }
};
        var_areaq_i = assign1420_e1412;

        let (assign1430_e1416, assign1430_e1416_d_n0, assign1430_e1416_d_n1, assign1430_e1416_d_n2, assign1430_e1416_d_n3, assign1430_e1416_d_n4, assign1430_e1416_d_n5, assign1430_e1416_d_n6, assign1430_e1416_d_n7, assign1430_e1416_d_n8, assign1430_e1416_d_n9, assign1430_e1416_d_b0, assign1430_e1416_d_b1, assign1430_e1416_d_b2, assign1430_e1416_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p152, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cgbov_i, var_cgbov_i_dn0, var_cgbov_i_dn1, var_cgbov_i_dn2, var_cgbov_i_dn3, var_cgbov_i_dn4, var_cgbov_i_dn5, var_cgbov_i_dn6, var_cgbov_i_dn7, var_cgbov_i_dn8, var_cgbov_i_dn9, var_cgbov_i_db0, var_cgbov_i_db1, var_cgbov_i_db2, var_cgbov_i_db3,)
    }
};
        var_cgbov_i = assign1430_e1416;
        var_cgbov_i_dn0 = assign1430_e1416_d_n0;
        var_cgbov_i_dn1 = assign1430_e1416_d_n1;
        var_cgbov_i_dn2 = assign1430_e1416_d_n2;
        var_cgbov_i_dn3 = assign1430_e1416_d_n3;
        var_cgbov_i_dn4 = assign1430_e1416_d_n4;
        var_cgbov_i_dn5 = assign1430_e1416_d_n5;
        var_cgbov_i_dn6 = assign1430_e1416_d_n6;
        var_cgbov_i_dn7 = assign1430_e1416_d_n7;
        var_cgbov_i_dn8 = assign1430_e1416_d_n8;
        var_cgbov_i_dn9 = assign1430_e1416_d_n9;
        var_cgbov_i_db0 = assign1430_e1416_d_b0;
        var_cgbov_i_db1 = assign1430_e1416_d_b1;
        var_cgbov_i_db2 = assign1430_e1416_d_b2;
        var_cgbov_i_db3 = assign1430_e1416_d_b3;

        let (assign1440_e1422,) = {
    if (var_guard83 != 0.0) {
        let assign1440_e1420: f64 = (p.p153 * 1000000.0);
        (assign1440_e1420,)
    } else {
        (var_nsdac_i,)
    }
};
        var_nsdac_i = assign1440_e1422;

        let (assign1450_e1426,) = {
    if (var_guard83 != 0.0) {
        (p.p154,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign1450_e1426;

        let (assign1460_e1430,) = {
    if (var_guard83 != 0.0) {
        (p.p155,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign1460_e1430;

        let (assign1470_e1434, assign1470_e1434_d_n0, assign1470_e1434_d_n1, assign1470_e1434_d_n2, assign1470_e1434_d_n3, assign1470_e1434_d_n4, assign1470_e1434_d_n5, assign1470_e1434_d_n6, assign1470_e1434_d_n7, assign1470_e1434_d_n8, assign1470_e1434_d_n9, assign1470_e1434_d_b0, assign1470_e1434_d_b1, assign1470_e1434_d_b2, assign1470_e1434_d_b3,) = {
    if (var_guard83 != 0.0) {
        (var_vfb1_t, var_vfb1_t_dn0, var_vfb1_t_dn1, var_vfb1_t_dn2, var_vfb1_t_dn3, var_vfb1_t_dn4, var_vfb1_t_dn5, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9, var_vfb1_t_db0, var_vfb1_t_db1, var_vfb1_t_db2, var_vfb1_t_db3,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign1470_e1434;
        var_vfbac1_t_dn0 = assign1470_e1434_d_n0;
        var_vfbac1_t_dn1 = assign1470_e1434_d_n1;
        var_vfbac1_t_dn2 = assign1470_e1434_d_n2;
        var_vfbac1_t_dn3 = assign1470_e1434_d_n3;
        var_vfbac1_t_dn4 = assign1470_e1434_d_n4;
        var_vfbac1_t_dn5 = assign1470_e1434_d_n5;
        var_vfbac1_t_dn6 = assign1470_e1434_d_n6;
        var_vfbac1_t_dn7 = assign1470_e1434_d_n7;
        var_vfbac1_t_dn8 = assign1470_e1434_d_n8;
        var_vfbac1_t_dn9 = assign1470_e1434_d_n9;
        var_vfbac1_t_db0 = assign1470_e1434_d_b0;
        var_vfbac1_t_db1 = assign1470_e1434_d_b1;
        var_vfbac1_t_db2 = assign1470_e1434_d_b2;
        var_vfbac1_t_db3 = assign1470_e1434_d_b3;

        let (assign1480_e1438, assign1480_e1438_d_n0, assign1480_e1438_d_n1, assign1480_e1438_d_n2, assign1480_e1438_d_n3, assign1480_e1438_d_n4, assign1480_e1438_d_n5, assign1480_e1438_d_n6, assign1480_e1438_d_n7, assign1480_e1438_d_n8, assign1480_e1438_d_n9, assign1480_e1438_d_b0, assign1480_e1438_d_b1, assign1480_e1438_d_b2, assign1480_e1438_d_b3,) = {
    if (var_guard83 != 0.0) {
        (var_vfb2_t, var_vfb2_t_dn0, var_vfb2_t_dn1, var_vfb2_t_dn2, var_vfb2_t_dn3, var_vfb2_t_dn4, var_vfb2_t_dn5, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9, var_vfb2_t_db0, var_vfb2_t_db1, var_vfb2_t_db2, var_vfb2_t_db3,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign1480_e1438;
        var_vfbac2_t_dn0 = assign1480_e1438_d_n0;
        var_vfbac2_t_dn1 = assign1480_e1438_d_n1;
        var_vfbac2_t_dn2 = assign1480_e1438_d_n2;
        var_vfbac2_t_dn3 = assign1480_e1438_d_n3;
        var_vfbac2_t_dn4 = assign1480_e1438_d_n4;
        var_vfbac2_t_dn5 = assign1480_e1438_d_n5;
        var_vfbac2_t_dn6 = assign1480_e1438_d_n6;
        var_vfbac2_t_dn7 = assign1480_e1438_d_n7;
        var_vfbac2_t_dn8 = assign1480_e1438_d_n8;
        var_vfbac2_t_dn9 = assign1480_e1438_d_n9;
        var_vfbac2_t_db0 = assign1480_e1438_d_b0;
        var_vfbac2_t_db1 = assign1480_e1438_d_b1;
        var_vfbac2_t_db2 = assign1480_e1438_d_b2;
        var_vfbac2_t_db3 = assign1480_e1438_d_b3;

        let (assign1490_e1442,) = {
    if (var_guard83 != 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1490_e1442;

        let (assign1500_e1446,) = {
    if (var_guard83 != 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1500_e1446;

        let (assign1510_e1450, assign1510_e1450_d_n0, assign1510_e1450_d_n1, assign1510_e1450_d_n2, assign1510_e1450_d_n3, assign1510_e1450_d_n4, assign1510_e1450_d_n5, assign1510_e1450_d_n6, assign1510_e1450_d_n7, assign1510_e1450_d_n8, assign1510_e1450_d_n9, assign1510_e1450_d_b0, assign1510_e1450_d_b1, assign1510_e1450_d_b2, assign1510_e1450_d_b3,) = {
    if (var_guard83 != 0.0) {
        (var_cf1_t, var_cf1_t_dn0, var_cf1_t_dn1, var_cf1_t_dn2, var_cf1_t_dn3, var_cf1_t_dn4, var_cf1_t_dn5, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9, var_cf1_t_db0, var_cf1_t_db1, var_cf1_t_db2, var_cf1_t_db3,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign1510_e1450;
        var_cfac1_t_dn0 = assign1510_e1450_d_n0;
        var_cfac1_t_dn1 = assign1510_e1450_d_n1;
        var_cfac1_t_dn2 = assign1510_e1450_d_n2;
        var_cfac1_t_dn3 = assign1510_e1450_d_n3;
        var_cfac1_t_dn4 = assign1510_e1450_d_n4;
        var_cfac1_t_dn5 = assign1510_e1450_d_n5;
        var_cfac1_t_dn6 = assign1510_e1450_d_n6;
        var_cfac1_t_dn7 = assign1510_e1450_d_n7;
        var_cfac1_t_dn8 = assign1510_e1450_d_n8;
        var_cfac1_t_dn9 = assign1510_e1450_d_n9;
        var_cfac1_t_db0 = assign1510_e1450_d_b0;
        var_cfac1_t_db1 = assign1510_e1450_d_b1;
        var_cfac1_t_db2 = assign1510_e1450_d_b2;
        var_cfac1_t_db3 = assign1510_e1450_d_b3;

        let (assign1520_e1454, assign1520_e1454_d_n0, assign1520_e1454_d_n1, assign1520_e1454_d_n2, assign1520_e1454_d_n3, assign1520_e1454_d_n4, assign1520_e1454_d_n5, assign1520_e1454_d_n6, assign1520_e1454_d_n7, assign1520_e1454_d_n8, assign1520_e1454_d_n9, assign1520_e1454_d_b0, assign1520_e1454_d_b1, assign1520_e1454_d_b2, assign1520_e1454_d_b3,) = {
    if (var_guard83 != 0.0) {
        (var_cf2_t, var_cf2_t_dn0, var_cf2_t_dn1, var_cf2_t_dn2, var_cf2_t_dn3, var_cf2_t_dn4, var_cf2_t_dn5, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9, var_cf2_t_db0, var_cf2_t_db1, var_cf2_t_db2, var_cf2_t_db3,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn0, var_cfac2_t_dn1, var_cfac2_t_dn2, var_cfac2_t_dn3, var_cfac2_t_dn4, var_cfac2_t_dn5, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9, var_cfac2_t_db0, var_cfac2_t_db1, var_cfac2_t_db2, var_cfac2_t_db3,)
    }
};
        var_cfac2_t = assign1520_e1454;
        var_cfac2_t_dn0 = assign1520_e1454_d_n0;
        var_cfac2_t_dn1 = assign1520_e1454_d_n1;
        var_cfac2_t_dn2 = assign1520_e1454_d_n2;
        var_cfac2_t_dn3 = assign1520_e1454_d_n3;
        var_cfac2_t_dn4 = assign1520_e1454_d_n4;
        var_cfac2_t_dn5 = assign1520_e1454_d_n5;
        var_cfac2_t_dn6 = assign1520_e1454_d_n6;
        var_cfac2_t_dn7 = assign1520_e1454_d_n7;
        var_cfac2_t_dn8 = assign1520_e1454_d_n8;
        var_cfac2_t_dn9 = assign1520_e1454_d_n9;
        var_cfac2_t_db0 = assign1520_e1454_d_b0;
        var_cfac2_t_db1 = assign1520_e1454_d_b1;
        var_cfac2_t_db2 = assign1520_e1454_d_b2;
        var_cfac2_t_db3 = assign1520_e1454_d_b3;

        let (assign1530_e1458, assign1530_e1458_d_n0, assign1530_e1458_d_n1, assign1530_e1458_d_n2, assign1530_e1458_d_n3, assign1530_e1458_d_n4, assign1530_e1458_d_n5, assign1530_e1458_d_n6, assign1530_e1458_d_n7, assign1530_e1458_d_n8, assign1530_e1458_d_n9, assign1530_e1458_d_b0, assign1530_e1458_d_b1, assign1530_e1458_d_b2, assign1530_e1458_d_b3,) = {
    if (var_guard83 != 0.0) {
        (var_thesat_t, var_thesat_t_dn0, var_thesat_t_dn1, var_thesat_t_dn2, var_thesat_t_dn3, var_thesat_t_dn4, var_thesat_t_dn5, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9, var_thesat_t_db0, var_thesat_t_db1, var_thesat_t_db2, var_thesat_t_db3,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign1530_e1458;
        var_thesatac_t_dn0 = assign1530_e1458_d_n0;
        var_thesatac_t_dn1 = assign1530_e1458_d_n1;
        var_thesatac_t_dn2 = assign1530_e1458_d_n2;
        var_thesatac_t_dn3 = assign1530_e1458_d_n3;
        var_thesatac_t_dn4 = assign1530_e1458_d_n4;
        var_thesatac_t_dn5 = assign1530_e1458_d_n5;
        var_thesatac_t_dn6 = assign1530_e1458_d_n6;
        var_thesatac_t_dn7 = assign1530_e1458_d_n7;
        var_thesatac_t_dn8 = assign1530_e1458_d_n8;
        var_thesatac_t_dn9 = assign1530_e1458_d_n9;
        var_thesatac_t_db0 = assign1530_e1458_d_b0;
        var_thesatac_t_db1 = assign1530_e1458_d_b1;
        var_thesatac_t_db2 = assign1530_e1458_d_b2;
        var_thesatac_t_db3 = assign1530_e1458_d_b3;

        let (assign1540_e1462,) = {
    if (var_guard83 != 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1540_e1462;

        let (assign1550_e1466,) = {
    if (var_guard83 != 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1550_e1466;

        let assign1560_e1469: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard86 = assign1560_e1469;

        let (assign1570_e1475, assign1570_e1475_d_n0, assign1570_e1475_d_n1, assign1570_e1475_d_n2, assign1570_e1475_d_n3, assign1570_e1475_d_n4, assign1570_e1475_d_n5, assign1570_e1475_d_n6, assign1570_e1475_d_n7, assign1570_e1475_d_n8, assign1570_e1475_d_n9, assign1570_e1475_d_b0, assign1570_e1475_d_b1, assign1570_e1475_d_b2, assign1570_e1475_d_b3,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p51, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign1570_e1475;
        var_vfbac1_t_dn0 = assign1570_e1475_d_n0;
        var_vfbac1_t_dn1 = assign1570_e1475_d_n1;
        var_vfbac1_t_dn2 = assign1570_e1475_d_n2;
        var_vfbac1_t_dn3 = assign1570_e1475_d_n3;
        var_vfbac1_t_dn4 = assign1570_e1475_d_n4;
        var_vfbac1_t_dn5 = assign1570_e1475_d_n5;
        var_vfbac1_t_dn6 = assign1570_e1475_d_n6;
        var_vfbac1_t_dn7 = assign1570_e1475_d_n7;
        var_vfbac1_t_dn8 = assign1570_e1475_d_n8;
        var_vfbac1_t_dn9 = assign1570_e1475_d_n9;
        var_vfbac1_t_db0 = assign1570_e1475_d_b0;
        var_vfbac1_t_db1 = assign1570_e1475_d_b1;
        var_vfbac1_t_db2 = assign1570_e1475_d_b2;
        var_vfbac1_t_db3 = assign1570_e1475_d_b3;

        let assign1580_e1477: f64 = if param_given[156] { 1.0 } else { 0.0 };
        let assign1580_e1479: f64 = if assign1580_e1477 == 1.0 { 1.0 } else { 0.0 };
        var_guard87 = assign1580_e1479;

        let (assign1590_e1487, assign1590_e1487_d_n0, assign1590_e1487_d_n1, assign1590_e1487_d_n2, assign1590_e1487_d_n3, assign1590_e1487_d_n4, assign1590_e1487_d_n5, assign1590_e1487_d_n6, assign1590_e1487_d_n7, assign1590_e1487_d_n8, assign1590_e1487_d_n9, assign1590_e1487_d_b0, assign1590_e1487_d_b1, assign1590_e1487_d_b2, assign1590_e1487_d_b3,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard87 != 0.0)) {
        (p.p156, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign1590_e1487;
        var_vfbac1_t_dn0 = assign1590_e1487_d_n0;
        var_vfbac1_t_dn1 = assign1590_e1487_d_n1;
        var_vfbac1_t_dn2 = assign1590_e1487_d_n2;
        var_vfbac1_t_dn3 = assign1590_e1487_d_n3;
        var_vfbac1_t_dn4 = assign1590_e1487_d_n4;
        var_vfbac1_t_dn5 = assign1590_e1487_d_n5;
        var_vfbac1_t_dn6 = assign1590_e1487_d_n6;
        var_vfbac1_t_dn7 = assign1590_e1487_d_n7;
        var_vfbac1_t_dn8 = assign1590_e1487_d_n8;
        var_vfbac1_t_dn9 = assign1590_e1487_d_n9;
        var_vfbac1_t_db0 = assign1590_e1487_d_b0;
        var_vfbac1_t_db1 = assign1590_e1487_d_b1;
        var_vfbac1_t_db2 = assign1590_e1487_d_b2;
        var_vfbac1_t_db3 = assign1590_e1487_d_b3;

        let (assign1600_e1493, assign1600_e1493_d_n0, assign1600_e1493_d_n1, assign1600_e1493_d_n2, assign1600_e1493_d_n3, assign1600_e1493_d_n4, assign1600_e1493_d_n5, assign1600_e1493_d_n6, assign1600_e1493_d_n7, assign1600_e1493_d_n8, assign1600_e1493_d_n9, assign1600_e1493_d_b0, assign1600_e1493_d_b1, assign1600_e1493_d_b2, assign1600_e1493_d_b3,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p52, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign1600_e1493;
        var_vfbac2_t_dn0 = assign1600_e1493_d_n0;
        var_vfbac2_t_dn1 = assign1600_e1493_d_n1;
        var_vfbac2_t_dn2 = assign1600_e1493_d_n2;
        var_vfbac2_t_dn3 = assign1600_e1493_d_n3;
        var_vfbac2_t_dn4 = assign1600_e1493_d_n4;
        var_vfbac2_t_dn5 = assign1600_e1493_d_n5;
        var_vfbac2_t_dn6 = assign1600_e1493_d_n6;
        var_vfbac2_t_dn7 = assign1600_e1493_d_n7;
        var_vfbac2_t_dn8 = assign1600_e1493_d_n8;
        var_vfbac2_t_dn9 = assign1600_e1493_d_n9;
        var_vfbac2_t_db0 = assign1600_e1493_d_b0;
        var_vfbac2_t_db1 = assign1600_e1493_d_b1;
        var_vfbac2_t_db2 = assign1600_e1493_d_b2;
        var_vfbac2_t_db3 = assign1600_e1493_d_b3;

        let assign1610_e1495: f64 = if param_given[157] { 1.0 } else { 0.0 };
        let assign1610_e1497: f64 = if assign1610_e1495 == 1.0 { 1.0 } else { 0.0 };
        var_guard88 = assign1610_e1497;

        let (assign1620_e1505, assign1620_e1505_d_n0, assign1620_e1505_d_n1, assign1620_e1505_d_n2, assign1620_e1505_d_n3, assign1620_e1505_d_n4, assign1620_e1505_d_n5, assign1620_e1505_d_n6, assign1620_e1505_d_n7, assign1620_e1505_d_n8, assign1620_e1505_d_n9, assign1620_e1505_d_b0, assign1620_e1505_d_b1, assign1620_e1505_d_b2, assign1620_e1505_d_b3,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard88 != 0.0)) {
        (p.p157, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign1620_e1505;
        var_vfbac2_t_dn0 = assign1620_e1505_d_n0;
        var_vfbac2_t_dn1 = assign1620_e1505_d_n1;
        var_vfbac2_t_dn2 = assign1620_e1505_d_n2;
        var_vfbac2_t_dn3 = assign1620_e1505_d_n3;
        var_vfbac2_t_dn4 = assign1620_e1505_d_n4;
        var_vfbac2_t_dn5 = assign1620_e1505_d_n5;
        var_vfbac2_t_dn6 = assign1620_e1505_d_n6;
        var_vfbac2_t_dn7 = assign1620_e1505_d_n7;
        var_vfbac2_t_dn8 = assign1620_e1505_d_n8;
        var_vfbac2_t_dn9 = assign1620_e1505_d_n9;
        var_vfbac2_t_db0 = assign1620_e1505_d_b0;
        var_vfbac2_t_db1 = assign1620_e1505_d_b1;
        var_vfbac2_t_db2 = assign1620_e1505_d_b2;
        var_vfbac2_t_db3 = assign1620_e1505_d_b3;

        let (assign1630_e1511,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p57,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1630_e1511;

        let assign1640_e1513: f64 = if param_given[158] { 1.0 } else { 0.0 };
        let assign1640_e1515: f64 = if assign1640_e1513 == 1.0 { 1.0 } else { 0.0 };
        var_guard89 = assign1640_e1515;

        let (assign1650_e1523,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard89 != 0.0)) {
        (p.p158,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign1650_e1523;

        let (assign1660_e1535,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1660_e1529: f64 = (p.p58 * var_psceac1_i);
        let assign1660_e1531: f64 = (assign1660_e1529 * var_tox2_i);
        let assign1660_e1533: f64 = (assign1660_e1531 / var_tox1_i);
        (assign1660_e1533,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign1660_e1535;

        *var_alpac_i_slot = var_alpac_i;
        *var_areaq_i_slot = var_areaq_i;
        *var_axac_i_slot = var_axac_i;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_db0_slot = var_betnedge_t_db0;
        *var_betnedge_t_db1_slot = var_betnedge_t_db1;
        *var_betnedge_t_db2_slot = var_betnedge_t_db2;
        *var_betnedge_t_db3_slot = var_betnedge_t_db3;
        *var_betnedge_t_dn0_slot = var_betnedge_t_dn0;
        *var_betnedge_t_dn1_slot = var_betnedge_t_dn1;
        *var_betnedge_t_dn2_slot = var_betnedge_t_dn2;
        *var_betnedge_t_dn3_slot = var_betnedge_t_dn3;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_dn5_slot = var_betnedge_t_dn5;
        *var_betnedge_t_dn6_slot = var_betnedge_t_dn6;
        *var_betnedge_t_dn7_slot = var_betnedge_t_dn7;
        *var_betnedge_t_dn8_slot = var_betnedge_t_dn8;
        *var_betnedge_t_dn9_slot = var_betnedge_t_dn9;
        *var_cf1edge_i_slot = var_cf1edge_i;
        *var_cf1edge_i_db0_slot = var_cf1edge_i_db0;
        *var_cf1edge_i_db1_slot = var_cf1edge_i_db1;
        *var_cf1edge_i_db2_slot = var_cf1edge_i_db2;
        *var_cf1edge_i_db3_slot = var_cf1edge_i_db3;
        *var_cf1edge_i_dn0_slot = var_cf1edge_i_dn0;
        *var_cf1edge_i_dn1_slot = var_cf1edge_i_dn1;
        *var_cf1edge_i_dn2_slot = var_cf1edge_i_dn2;
        *var_cf1edge_i_dn3_slot = var_cf1edge_i_dn3;
        *var_cf1edge_i_dn4_slot = var_cf1edge_i_dn4;
        *var_cf1edge_i_dn5_slot = var_cf1edge_i_dn5;
        *var_cf1edge_i_dn6_slot = var_cf1edge_i_dn6;
        *var_cf1edge_i_dn7_slot = var_cf1edge_i_dn7;
        *var_cf1edge_i_dn8_slot = var_cf1edge_i_dn8;
        *var_cf1edge_i_dn9_slot = var_cf1edge_i_dn9;
        *var_cf2edge_i_slot = var_cf2edge_i;
        *var_cf2edge_i_db0_slot = var_cf2edge_i_db0;
        *var_cf2edge_i_db1_slot = var_cf2edge_i_db1;
        *var_cf2edge_i_db2_slot = var_cf2edge_i_db2;
        *var_cf2edge_i_db3_slot = var_cf2edge_i_db3;
        *var_cf2edge_i_dn0_slot = var_cf2edge_i_dn0;
        *var_cf2edge_i_dn1_slot = var_cf2edge_i_dn1;
        *var_cf2edge_i_dn2_slot = var_cf2edge_i_dn2;
        *var_cf2edge_i_dn3_slot = var_cf2edge_i_dn3;
        *var_cf2edge_i_dn4_slot = var_cf2edge_i_dn4;
        *var_cf2edge_i_dn5_slot = var_cf2edge_i_dn5;
        *var_cf2edge_i_dn6_slot = var_cf2edge_i_dn6;
        *var_cf2edge_i_dn7_slot = var_cf2edge_i_dn7;
        *var_cf2edge_i_dn8_slot = var_cf2edge_i_dn8;
        *var_cf2edge_i_dn9_slot = var_cf2edge_i_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_db0_slot = var_cfac1_t_db0;
        *var_cfac1_t_db1_slot = var_cfac1_t_db1;
        *var_cfac1_t_db2_slot = var_cfac1_t_db2;
        *var_cfac1_t_db3_slot = var_cfac1_t_db3;
        *var_cfac1_t_dn0_slot = var_cfac1_t_dn0;
        *var_cfac1_t_dn1_slot = var_cfac1_t_dn1;
        *var_cfac1_t_dn2_slot = var_cfac1_t_dn2;
        *var_cfac1_t_dn3_slot = var_cfac1_t_dn3;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn5_slot = var_cfac1_t_dn5;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_db0_slot = var_cfac2_t_db0;
        *var_cfac2_t_db1_slot = var_cfac2_t_db1;
        *var_cfac2_t_db2_slot = var_cfac2_t_db2;
        *var_cfac2_t_db3_slot = var_cfac2_t_db3;
        *var_cfac2_t_dn0_slot = var_cfac2_t_dn0;
        *var_cfac2_t_dn1_slot = var_cfac2_t_dn1;
        *var_cfac2_t_dn2_slot = var_cfac2_t_dn2;
        *var_cfac2_t_dn3_slot = var_cfac2_t_dn3;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn5_slot = var_cfac2_t_dn5;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_db0_slot = var_cgbov_i_db0;
        *var_cgbov_i_db1_slot = var_cgbov_i_db1;
        *var_cgbov_i_db2_slot = var_cgbov_i_db2;
        *var_cgbov_i_db3_slot = var_cgbov_i_db3;
        *var_cgbov_i_dn0_slot = var_cgbov_i_dn0;
        *var_cgbov_i_dn1_slot = var_cgbov_i_dn1;
        *var_cgbov_i_dn2_slot = var_cgbov_i_dn2;
        *var_cgbov_i_dn3_slot = var_cgbov_i_dn3;
        *var_cgbov_i_dn4_slot = var_cgbov_i_dn4;
        *var_cgbov_i_dn5_slot = var_cgbov_i_dn5;
        *var_cgbov_i_dn6_slot = var_cgbov_i_dn6;
        *var_cgbov_i_dn7_slot = var_cgbov_i_dn7;
        *var_cgbov_i_dn8_slot = var_cgbov_i_dn8;
        *var_cgbov_i_dn9_slot = var_cgbov_i_dn9;
        *var_cic2edge_i_slot = var_cic2edge_i;
        *var_fif_i_slot = var_fif_i;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_guard86_slot = var_guard86;
        *var_guard87_slot = var_guard87;
        *var_guard88_slot = var_guard88;
        *var_guard89_slot = var_guard89;
        *var_nsdac_i_slot = var_nsdac_i;
        *var_psce1edge_i_slot = var_psce1edge_i;
        *var_psce1edge_i_db0_slot = var_psce1edge_i_db0;
        *var_psce1edge_i_db1_slot = var_psce1edge_i_db1;
        *var_psce1edge_i_db2_slot = var_psce1edge_i_db2;
        *var_psce1edge_i_db3_slot = var_psce1edge_i_db3;
        *var_psce1edge_i_dn0_slot = var_psce1edge_i_dn0;
        *var_psce1edge_i_dn1_slot = var_psce1edge_i_dn1;
        *var_psce1edge_i_dn2_slot = var_psce1edge_i_dn2;
        *var_psce1edge_i_dn3_slot = var_psce1edge_i_dn3;
        *var_psce1edge_i_dn4_slot = var_psce1edge_i_dn4;
        *var_psce1edge_i_dn5_slot = var_psce1edge_i_dn5;
        *var_psce1edge_i_dn6_slot = var_psce1edge_i_dn6;
        *var_psce1edge_i_dn7_slot = var_psce1edge_i_dn7;
        *var_psce1edge_i_dn8_slot = var_psce1edge_i_dn8;
        *var_psce1edge_i_dn9_slot = var_psce1edge_i_dn9;
        *var_psce2edge_i_slot = var_psce2edge_i;
        *var_psce2edge_i_db0_slot = var_psce2edge_i_db0;
        *var_psce2edge_i_db1_slot = var_psce2edge_i_db1;
        *var_psce2edge_i_db2_slot = var_psce2edge_i_db2;
        *var_psce2edge_i_db3_slot = var_psce2edge_i_db3;
        *var_psce2edge_i_dn0_slot = var_psce2edge_i_dn0;
        *var_psce2edge_i_dn1_slot = var_psce2edge_i_dn1;
        *var_psce2edge_i_dn2_slot = var_psce2edge_i_dn2;
        *var_psce2edge_i_dn3_slot = var_psce2edge_i_dn3;
        *var_psce2edge_i_dn4_slot = var_psce2edge_i_dn4;
        *var_psce2edge_i_dn5_slot = var_psce2edge_i_dn5;
        *var_psce2edge_i_dn6_slot = var_psce2edge_i_dn6;
        *var_psce2edge_i_dn7_slot = var_psce2edge_i_dn7;
        *var_psce2edge_i_dn8_slot = var_psce2edge_i_dn8;
        *var_psce2edge_i_dn9_slot = var_psce2edge_i_dn9;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_db0_slot = var_thesatac_t_db0;
        *var_thesatac_t_db1_slot = var_thesatac_t_db1;
        *var_thesatac_t_db2_slot = var_thesatac_t_db2;
        *var_thesatac_t_db3_slot = var_thesatac_t_db3;
        *var_thesatac_t_dn0_slot = var_thesatac_t_dn0;
        *var_thesatac_t_dn1_slot = var_thesatac_t_dn1;
        *var_thesatac_t_dn2_slot = var_thesatac_t_dn2;
        *var_thesatac_t_dn3_slot = var_thesatac_t_dn3;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn5_slot = var_thesatac_t_dn5;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_db0_slot = var_vfbac1_t_db0;
        *var_vfbac1_t_db1_slot = var_vfbac1_t_db1;
        *var_vfbac1_t_db2_slot = var_vfbac1_t_db2;
        *var_vfbac1_t_db3_slot = var_vfbac1_t_db3;
        *var_vfbac1_t_dn0_slot = var_vfbac1_t_dn0;
        *var_vfbac1_t_dn1_slot = var_vfbac1_t_dn1;
        *var_vfbac1_t_dn2_slot = var_vfbac1_t_dn2;
        *var_vfbac1_t_dn3_slot = var_vfbac1_t_dn3;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn5_slot = var_vfbac1_t_dn5;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_db0_slot = var_vfbac2_t_db0;
        *var_vfbac2_t_db1_slot = var_vfbac2_t_db1;
        *var_vfbac2_t_db2_slot = var_vfbac2_t_db2;
        *var_vfbac2_t_db3_slot = var_vfbac2_t_db3;
        *var_vfbac2_t_dn0_slot = var_vfbac2_t_dn0;
        *var_vfbac2_t_dn1_slot = var_vfbac2_t_dn1;
        *var_vfbac2_t_dn2_slot = var_vfbac2_t_dn2;
        *var_vfbac2_t_dn3_slot = var_vfbac2_t_dn3;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn5_slot = var_vfbac2_t_dn5;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }

    pub(super) fn stamp_transient_block_4(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard83: f64,
        var_guard86: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_adrain_i_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_asource_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_db0_slot: &mut f64,
        var_cfac1_t_db1_slot: &mut f64,
        var_cfac1_t_db2_slot: &mut f64,
        var_cfac1_t_db3_slot: &mut f64,
        var_cfac1_t_dn0_slot: &mut f64,
        var_cfac1_t_dn1_slot: &mut f64,
        var_cfac1_t_dn2_slot: &mut f64,
        var_cfac1_t_dn3_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn5_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_db0_slot: &mut f64,
        var_cfac2_t_db1_slot: &mut f64,
        var_cfac2_t_db2_slot: &mut f64,
        var_cfac2_t_db3_slot: &mut f64,
        var_cfac2_t_dn0_slot: &mut f64,
        var_cfac2_t_dn1_slot: &mut f64,
        var_cfac2_t_dn2_slot: &mut f64,
        var_cfac2_t_dn3_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn5_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_db0_slot: &mut f64,
        var_cfr_i_db1_slot: &mut f64,
        var_cfr_i_db2_slot: &mut f64,
        var_cfr_i_db3_slot: &mut f64,
        var_cfr_i_dn0_slot: &mut f64,
        var_cfr_i_dn1_slot: &mut f64,
        var_cfr_i_dn2_slot: &mut f64,
        var_cfr_i_dn3_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn5_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_db0_slot: &mut f64,
        var_cfrd_i_db1_slot: &mut f64,
        var_cfrd_i_db2_slot: &mut f64,
        var_cfrd_i_db3_slot: &mut f64,
        var_cfrd_i_dn0_slot: &mut f64,
        var_cfrd_i_dn1_slot: &mut f64,
        var_cfrd_i_dn2_slot: &mut f64,
        var_cfrd_i_dn3_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn5_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_db0_slot: &mut f64,
        var_cov_i_db1_slot: &mut f64,
        var_cov_i_db2_slot: &mut f64,
        var_cov_i_db3_slot: &mut f64,
        var_cov_i_dn0_slot: &mut f64,
        var_cov_i_dn1_slot: &mut f64,
        var_cov_i_dn2_slot: &mut f64,
        var_cov_i_dn3_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn5_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_db0_slot: &mut f64,
        var_covd_i_db1_slot: &mut f64,
        var_covd_i_db2_slot: &mut f64,
        var_covd_i_db3_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn1_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
        var_covd_i_dn6_slot: &mut f64,
        var_covd_i_dn7_slot: &mut f64,
        var_covd_i_dn8_slot: &mut f64,
        var_covd_i_dn9_slot: &mut f64,
        var_covdl_i_slot: &mut f64,
        var_covdlb_i_slot: &mut f64,
        var_csd_i_slot: &mut f64,
        var_csdbp_i_slot: &mut f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_db0_slot: &mut f64,
        var_cth_i_db1_slot: &mut f64,
        var_cth_i_db2_slot: &mut f64,
        var_cth_i_db3_slot: &mut f64,
        var_cth_i_dn0_slot: &mut f64,
        var_cth_i_dn1_slot: &mut f64,
        var_cth_i_dn2_slot: &mut f64,
        var_cth_i_dn3_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn5_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_dvfbov_i_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fntexc_i_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_invnf_slot: &mut f64,
        var_len_slot: &mut f64,
        var_mult_i_int_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_pdrain_i_slot: &mut f64,
        var_psource_i_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_db0_slot: &mut f64,
        var_rth_t_db1_slot: &mut f64,
        var_rth_t_db2_slot: &mut f64,
        var_rth_t_db3_slot: &mut f64,
        var_rth_t_dn0_slot: &mut f64,
        var_rth_t_dn1_slot: &mut f64,
        var_rth_t_dn2_slot: &mut f64,
        var_rth_t_dn3_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn5_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_db0_slot: &mut f64,
        var_thesatac_t_db1_slot: &mut f64,
        var_thesatac_t_db2_slot: &mut f64,
        var_thesatac_t_db3_slot: &mut f64,
        var_thesatac_t_dn0_slot: &mut f64,
        var_thesatac_t_dn1_slot: &mut f64,
        var_thesatac_t_dn2_slot: &mut f64,
        var_thesatac_t_dn3_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn5_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_w_i_slot: &mut f64,
    ) {
        let mut var_adrain_i: f64 = *var_adrain_i_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_asource_i: f64 = *var_asource_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_db0: f64 = *var_cfac1_t_db0_slot;
        let mut var_cfac1_t_db1: f64 = *var_cfac1_t_db1_slot;
        let mut var_cfac1_t_db2: f64 = *var_cfac1_t_db2_slot;
        let mut var_cfac1_t_db3: f64 = *var_cfac1_t_db3_slot;
        let mut var_cfac1_t_dn0: f64 = *var_cfac1_t_dn0_slot;
        let mut var_cfac1_t_dn1: f64 = *var_cfac1_t_dn1_slot;
        let mut var_cfac1_t_dn2: f64 = *var_cfac1_t_dn2_slot;
        let mut var_cfac1_t_dn3: f64 = *var_cfac1_t_dn3_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn5: f64 = *var_cfac1_t_dn5_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_db0: f64 = *var_cfac2_t_db0_slot;
        let mut var_cfac2_t_db1: f64 = *var_cfac2_t_db1_slot;
        let mut var_cfac2_t_db2: f64 = *var_cfac2_t_db2_slot;
        let mut var_cfac2_t_db3: f64 = *var_cfac2_t_db3_slot;
        let mut var_cfac2_t_dn0: f64 = *var_cfac2_t_dn0_slot;
        let mut var_cfac2_t_dn1: f64 = *var_cfac2_t_dn1_slot;
        let mut var_cfac2_t_dn2: f64 = *var_cfac2_t_dn2_slot;
        let mut var_cfac2_t_dn3: f64 = *var_cfac2_t_dn3_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn5: f64 = *var_cfac2_t_dn5_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_db0: f64 = *var_cfr_i_db0_slot;
        let mut var_cfr_i_db1: f64 = *var_cfr_i_db1_slot;
        let mut var_cfr_i_db2: f64 = *var_cfr_i_db2_slot;
        let mut var_cfr_i_db3: f64 = *var_cfr_i_db3_slot;
        let mut var_cfr_i_dn0: f64 = *var_cfr_i_dn0_slot;
        let mut var_cfr_i_dn1: f64 = *var_cfr_i_dn1_slot;
        let mut var_cfr_i_dn2: f64 = *var_cfr_i_dn2_slot;
        let mut var_cfr_i_dn3: f64 = *var_cfr_i_dn3_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn5: f64 = *var_cfr_i_dn5_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_db0: f64 = *var_cfrd_i_db0_slot;
        let mut var_cfrd_i_db1: f64 = *var_cfrd_i_db1_slot;
        let mut var_cfrd_i_db2: f64 = *var_cfrd_i_db2_slot;
        let mut var_cfrd_i_db3: f64 = *var_cfrd_i_db3_slot;
        let mut var_cfrd_i_dn0: f64 = *var_cfrd_i_dn0_slot;
        let mut var_cfrd_i_dn1: f64 = *var_cfrd_i_dn1_slot;
        let mut var_cfrd_i_dn2: f64 = *var_cfrd_i_dn2_slot;
        let mut var_cfrd_i_dn3: f64 = *var_cfrd_i_dn3_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn5: f64 = *var_cfrd_i_dn5_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_db0: f64 = *var_cov_i_db0_slot;
        let mut var_cov_i_db1: f64 = *var_cov_i_db1_slot;
        let mut var_cov_i_db2: f64 = *var_cov_i_db2_slot;
        let mut var_cov_i_db3: f64 = *var_cov_i_db3_slot;
        let mut var_cov_i_dn0: f64 = *var_cov_i_dn0_slot;
        let mut var_cov_i_dn1: f64 = *var_cov_i_dn1_slot;
        let mut var_cov_i_dn2: f64 = *var_cov_i_dn2_slot;
        let mut var_cov_i_dn3: f64 = *var_cov_i_dn3_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn5: f64 = *var_cov_i_dn5_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_db0: f64 = *var_covd_i_db0_slot;
        let mut var_covd_i_db1: f64 = *var_covd_i_db1_slot;
        let mut var_covd_i_db2: f64 = *var_covd_i_db2_slot;
        let mut var_covd_i_db3: f64 = *var_covd_i_db3_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn1: f64 = *var_covd_i_dn1_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
        let mut var_covd_i_dn6: f64 = *var_covd_i_dn6_slot;
        let mut var_covd_i_dn7: f64 = *var_covd_i_dn7_slot;
        let mut var_covd_i_dn8: f64 = *var_covd_i_dn8_slot;
        let mut var_covd_i_dn9: f64 = *var_covd_i_dn9_slot;
        let mut var_covdl_i: f64 = *var_covdl_i_slot;
        let mut var_covdlb_i: f64 = *var_covdlb_i_slot;
        let mut var_csd_i: f64 = *var_csd_i_slot;
        let mut var_csdbp_i: f64 = *var_csdbp_i_slot;
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_db0: f64 = *var_cth_i_db0_slot;
        let mut var_cth_i_db1: f64 = *var_cth_i_db1_slot;
        let mut var_cth_i_db2: f64 = *var_cth_i_db2_slot;
        let mut var_cth_i_db3: f64 = *var_cth_i_db3_slot;
        let mut var_cth_i_dn0: f64 = *var_cth_i_dn0_slot;
        let mut var_cth_i_dn1: f64 = *var_cth_i_dn1_slot;
        let mut var_cth_i_dn2: f64 = *var_cth_i_dn2_slot;
        let mut var_cth_i_dn3: f64 = *var_cth_i_dn3_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn5: f64 = *var_cth_i_dn5_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_dvfbov_i: f64 = *var_dvfbov_i_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fntexc_i: f64 = *var_fntexc_i_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_invnf: f64 = *var_invnf_slot;
        let mut var_len: f64 = *var_len_slot;
        let mut var_mult_i_int: f64 = *var_mult_i_int_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_pdrain_i: f64 = *var_pdrain_i_slot;
        let mut var_psource_i: f64 = *var_psource_i_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_db0: f64 = *var_rth_t_db0_slot;
        let mut var_rth_t_db1: f64 = *var_rth_t_db1_slot;
        let mut var_rth_t_db2: f64 = *var_rth_t_db2_slot;
        let mut var_rth_t_db3: f64 = *var_rth_t_db3_slot;
        let mut var_rth_t_dn0: f64 = *var_rth_t_dn0_slot;
        let mut var_rth_t_dn1: f64 = *var_rth_t_dn1_slot;
        let mut var_rth_t_dn2: f64 = *var_rth_t_dn2_slot;
        let mut var_rth_t_dn3: f64 = *var_rth_t_dn3_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn5: f64 = *var_rth_t_dn5_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_db0: f64 = *var_thesatac_t_db0_slot;
        let mut var_thesatac_t_db1: f64 = *var_thesatac_t_db1_slot;
        let mut var_thesatac_t_db2: f64 = *var_thesatac_t_db2_slot;
        let mut var_thesatac_t_db3: f64 = *var_thesatac_t_db3_slot;
        let mut var_thesatac_t_dn0: f64 = *var_thesatac_t_dn0_slot;
        let mut var_thesatac_t_dn1: f64 = *var_thesatac_t_dn1_slot;
        let mut var_thesatac_t_dn2: f64 = *var_thesatac_t_dn2_slot;
        let mut var_thesatac_t_dn3: f64 = *var_thesatac_t_dn3_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn5: f64 = *var_thesatac_t_dn5_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_w_i: f64 = *var_w_i_slot;

        let (assign1670_e1541, assign1670_e1541_d_n0, assign1670_e1541_d_n1, assign1670_e1541_d_n2, assign1670_e1541_d_n3, assign1670_e1541_d_n4, assign1670_e1541_d_n5, assign1670_e1541_d_n6, assign1670_e1541_d_n7, assign1670_e1541_d_n8, assign1670_e1541_d_n9, assign1670_e1541_d_b0, assign1670_e1541_d_b1, assign1670_e1541_d_b2, assign1670_e1541_d_b3,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p62, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign1670_e1541;
        var_cfac1_t_dn0 = assign1670_e1541_d_n0;
        var_cfac1_t_dn1 = assign1670_e1541_d_n1;
        var_cfac1_t_dn2 = assign1670_e1541_d_n2;
        var_cfac1_t_dn3 = assign1670_e1541_d_n3;
        var_cfac1_t_dn4 = assign1670_e1541_d_n4;
        var_cfac1_t_dn5 = assign1670_e1541_d_n5;
        var_cfac1_t_dn6 = assign1670_e1541_d_n6;
        var_cfac1_t_dn7 = assign1670_e1541_d_n7;
        var_cfac1_t_dn8 = assign1670_e1541_d_n8;
        var_cfac1_t_dn9 = assign1670_e1541_d_n9;
        var_cfac1_t_db0 = assign1670_e1541_d_b0;
        var_cfac1_t_db1 = assign1670_e1541_d_b1;
        var_cfac1_t_db2 = assign1670_e1541_d_b2;
        var_cfac1_t_db3 = assign1670_e1541_d_b3;

        let assign1680_e1543: f64 = if param_given[159] { 1.0 } else { 0.0 };
        let assign1680_e1545: f64 = if assign1680_e1543 == 1.0 { 1.0 } else { 0.0 };
        var_guard90 = assign1680_e1545;

        let (assign1690_e1553, assign1690_e1553_d_n0, assign1690_e1553_d_n1, assign1690_e1553_d_n2, assign1690_e1553_d_n3, assign1690_e1553_d_n4, assign1690_e1553_d_n5, assign1690_e1553_d_n6, assign1690_e1553_d_n7, assign1690_e1553_d_n8, assign1690_e1553_d_n9, assign1690_e1553_d_b0, assign1690_e1553_d_b1, assign1690_e1553_d_b2, assign1690_e1553_d_b3,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard90 != 0.0)) {
        (p.p159, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign1690_e1553;
        var_cfac1_t_dn0 = assign1690_e1553_d_n0;
        var_cfac1_t_dn1 = assign1690_e1553_d_n1;
        var_cfac1_t_dn2 = assign1690_e1553_d_n2;
        var_cfac1_t_dn3 = assign1690_e1553_d_n3;
        var_cfac1_t_dn4 = assign1690_e1553_d_n4;
        var_cfac1_t_dn5 = assign1690_e1553_d_n5;
        var_cfac1_t_dn6 = assign1690_e1553_d_n6;
        var_cfac1_t_dn7 = assign1690_e1553_d_n7;
        var_cfac1_t_dn8 = assign1690_e1553_d_n8;
        var_cfac1_t_dn9 = assign1690_e1553_d_n9;
        var_cfac1_t_db0 = assign1690_e1553_d_b0;
        var_cfac1_t_db1 = assign1690_e1553_d_b1;
        var_cfac1_t_db2 = assign1690_e1553_d_b2;
        var_cfac1_t_db3 = assign1690_e1553_d_b3;

        let (assign1700_e1565, assign1700_e1565_d_n0, assign1700_e1565_d_n1, assign1700_e1565_d_n2, assign1700_e1565_d_n3, assign1700_e1565_d_n4, assign1700_e1565_d_n5, assign1700_e1565_d_n6, assign1700_e1565_d_n7, assign1700_e1565_d_n8, assign1700_e1565_d_n9, assign1700_e1565_d_b0, assign1700_e1565_d_b1, assign1700_e1565_d_b2, assign1700_e1565_d_b3,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        let assign1700_e1559: f64 = (p.p63 * var_cfac1_t);
        let assign1700_e1561: f64 = (assign1700_e1559 * var_tox2_i);
        let assign1700_e1563: f64 = (assign1700_e1561 / var_tox1_i);
        (assign1700_e1563, (((p.p63 * var_cfac1_t_dn0) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn1) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn2) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn3) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn5) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_db0) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_db1) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_db2) * var_tox2_i) / var_tox1_i), (((p.p63 * var_cfac1_t_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn0, var_cfac2_t_dn1, var_cfac2_t_dn2, var_cfac2_t_dn3, var_cfac2_t_dn4, var_cfac2_t_dn5, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9, var_cfac2_t_db0, var_cfac2_t_db1, var_cfac2_t_db2, var_cfac2_t_db3,)
    }
};
        var_cfac2_t = assign1700_e1565;
        var_cfac2_t_dn0 = assign1700_e1565_d_n0;
        var_cfac2_t_dn1 = assign1700_e1565_d_n1;
        var_cfac2_t_dn2 = assign1700_e1565_d_n2;
        var_cfac2_t_dn3 = assign1700_e1565_d_n3;
        var_cfac2_t_dn4 = assign1700_e1565_d_n4;
        var_cfac2_t_dn5 = assign1700_e1565_d_n5;
        var_cfac2_t_dn6 = assign1700_e1565_d_n6;
        var_cfac2_t_dn7 = assign1700_e1565_d_n7;
        var_cfac2_t_dn8 = assign1700_e1565_d_n8;
        var_cfac2_t_dn9 = assign1700_e1565_d_n9;
        var_cfac2_t_db0 = assign1700_e1565_d_b0;
        var_cfac2_t_db1 = assign1700_e1565_d_b1;
        var_cfac2_t_db2 = assign1700_e1565_d_b2;
        var_cfac2_t_db3 = assign1700_e1565_d_b3;

        let (assign1710_e1571, assign1710_e1571_d_n0, assign1710_e1571_d_n1, assign1710_e1571_d_n2, assign1710_e1571_d_n3, assign1710_e1571_d_n4, assign1710_e1571_d_n5, assign1710_e1571_d_n6, assign1710_e1571_d_n7, assign1710_e1571_d_n8, assign1710_e1571_d_n9, assign1710_e1571_d_b0, assign1710_e1571_d_b1, assign1710_e1571_d_b2, assign1710_e1571_d_b3,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p93, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign1710_e1571;
        var_thesatac_t_dn0 = assign1710_e1571_d_n0;
        var_thesatac_t_dn1 = assign1710_e1571_d_n1;
        var_thesatac_t_dn2 = assign1710_e1571_d_n2;
        var_thesatac_t_dn3 = assign1710_e1571_d_n3;
        var_thesatac_t_dn4 = assign1710_e1571_d_n4;
        var_thesatac_t_dn5 = assign1710_e1571_d_n5;
        var_thesatac_t_dn6 = assign1710_e1571_d_n6;
        var_thesatac_t_dn7 = assign1710_e1571_d_n7;
        var_thesatac_t_dn8 = assign1710_e1571_d_n8;
        var_thesatac_t_dn9 = assign1710_e1571_d_n9;
        var_thesatac_t_db0 = assign1710_e1571_d_b0;
        var_thesatac_t_db1 = assign1710_e1571_d_b1;
        var_thesatac_t_db2 = assign1710_e1571_d_b2;
        var_thesatac_t_db3 = assign1710_e1571_d_b3;

        let assign1720_e1573: f64 = if param_given[160] { 1.0 } else { 0.0 };
        let assign1720_e1575: f64 = if assign1720_e1573 == 1.0 { 1.0 } else { 0.0 };
        var_guard91 = assign1720_e1575;

        let (assign1730_e1583, assign1730_e1583_d_n0, assign1730_e1583_d_n1, assign1730_e1583_d_n2, assign1730_e1583_d_n3, assign1730_e1583_d_n4, assign1730_e1583_d_n5, assign1730_e1583_d_n6, assign1730_e1583_d_n7, assign1730_e1583_d_n8, assign1730_e1583_d_n9, assign1730_e1583_d_b0, assign1730_e1583_d_b1, assign1730_e1583_d_b2, assign1730_e1583_d_b3,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard91 != 0.0)) {
        (p.p160, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign1730_e1583;
        var_thesatac_t_dn0 = assign1730_e1583_d_n0;
        var_thesatac_t_dn1 = assign1730_e1583_d_n1;
        var_thesatac_t_dn2 = assign1730_e1583_d_n2;
        var_thesatac_t_dn3 = assign1730_e1583_d_n3;
        var_thesatac_t_dn4 = assign1730_e1583_d_n4;
        var_thesatac_t_dn5 = assign1730_e1583_d_n5;
        var_thesatac_t_dn6 = assign1730_e1583_d_n6;
        var_thesatac_t_dn7 = assign1730_e1583_d_n7;
        var_thesatac_t_dn8 = assign1730_e1583_d_n8;
        var_thesatac_t_dn9 = assign1730_e1583_d_n9;
        var_thesatac_t_db0 = assign1730_e1583_d_b0;
        var_thesatac_t_db1 = assign1730_e1583_d_b1;
        var_thesatac_t_db2 = assign1730_e1583_d_b2;
        var_thesatac_t_db3 = assign1730_e1583_d_b3;

        let (assign1740_e1589,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p97,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1740_e1589;

        let assign1750_e1591: f64 = if param_given[161] { 1.0 } else { 0.0 };
        let assign1750_e1593: f64 = if assign1750_e1591 == 1.0 { 1.0 } else { 0.0 };
        var_guard92 = assign1750_e1593;

        let (assign1760_e1601,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard92 != 0.0)) {
        (p.p161,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign1760_e1601;

        let (assign1770_e1607,) = {
    if ((var_guard83 != 0.0) && (var_guard86 != 0.0)) {
        (p.p98,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1770_e1607;

        let assign1780_e1609: f64 = if param_given[162] { 1.0 } else { 0.0 };
        let assign1780_e1611: f64 = if assign1780_e1609 == 1.0 { 1.0 } else { 0.0 };
        var_guard93 = assign1780_e1611;

        let (assign1790_e1619,) = {
    if (((var_guard83 != 0.0) && (var_guard86 != 0.0)) && (var_guard93 != 0.0)) {
        (p.p162,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign1790_e1619;

        let (assign1800_e1623, assign1800_e1623_d_n0, assign1800_e1623_d_n1, assign1800_e1623_d_n2, assign1800_e1623_d_n3, assign1800_e1623_d_n4, assign1800_e1623_d_n5, assign1800_e1623_d_n6, assign1800_e1623_d_n7, assign1800_e1623_d_n8, assign1800_e1623_d_n9, assign1800_e1623_d_b0, assign1800_e1623_d_b1, assign1800_e1623_d_b2, assign1800_e1623_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p163, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cov_i, var_cov_i_dn0, var_cov_i_dn1, var_cov_i_dn2, var_cov_i_dn3, var_cov_i_dn4, var_cov_i_dn5, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9, var_cov_i_db0, var_cov_i_db1, var_cov_i_db2, var_cov_i_db3,)
    }
};
        var_cov_i = assign1800_e1623;
        var_cov_i_dn0 = assign1800_e1623_d_n0;
        var_cov_i_dn1 = assign1800_e1623_d_n1;
        var_cov_i_dn2 = assign1800_e1623_d_n2;
        var_cov_i_dn3 = assign1800_e1623_d_n3;
        var_cov_i_dn4 = assign1800_e1623_d_n4;
        var_cov_i_dn5 = assign1800_e1623_d_n5;
        var_cov_i_dn6 = assign1800_e1623_d_n6;
        var_cov_i_dn7 = assign1800_e1623_d_n7;
        var_cov_i_dn8 = assign1800_e1623_d_n8;
        var_cov_i_dn9 = assign1800_e1623_d_n9;
        var_cov_i_db0 = assign1800_e1623_d_b0;
        var_cov_i_db1 = assign1800_e1623_d_b1;
        var_cov_i_db2 = assign1800_e1623_d_b2;
        var_cov_i_db3 = assign1800_e1623_d_b3;

        let (assign1810_e1627, assign1810_e1627_d_n0, assign1810_e1627_d_n1, assign1810_e1627_d_n2, assign1810_e1627_d_n3, assign1810_e1627_d_n4, assign1810_e1627_d_n5, assign1810_e1627_d_n6, assign1810_e1627_d_n7, assign1810_e1627_d_n8, assign1810_e1627_d_n9, assign1810_e1627_d_b0, assign1810_e1627_d_b1, assign1810_e1627_d_b2, assign1810_e1627_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p164, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_covd_i, var_covd_i_dn0, var_covd_i_dn1, var_covd_i_dn2, var_covd_i_dn3, var_covd_i_dn4, var_covd_i_dn5, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9, var_covd_i_db0, var_covd_i_db1, var_covd_i_db2, var_covd_i_db3,)
    }
};
        var_covd_i = assign1810_e1627;
        var_covd_i_dn0 = assign1810_e1627_d_n0;
        var_covd_i_dn1 = assign1810_e1627_d_n1;
        var_covd_i_dn2 = assign1810_e1627_d_n2;
        var_covd_i_dn3 = assign1810_e1627_d_n3;
        var_covd_i_dn4 = assign1810_e1627_d_n4;
        var_covd_i_dn5 = assign1810_e1627_d_n5;
        var_covd_i_dn6 = assign1810_e1627_d_n6;
        var_covd_i_dn7 = assign1810_e1627_d_n7;
        var_covd_i_dn8 = assign1810_e1627_d_n8;
        var_covd_i_dn9 = assign1810_e1627_d_n9;
        var_covd_i_db0 = assign1810_e1627_d_b0;
        var_covd_i_db1 = assign1810_e1627_d_b1;
        var_covd_i_db2 = assign1810_e1627_d_b2;
        var_covd_i_db3 = assign1810_e1627_d_b3;

        let (assign1820_e1631,) = {
    if (var_guard83 != 0.0) {
        (p.p165,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign1820_e1631;

        let (assign1830_e1635,) = {
    if (var_guard83 != 0.0) {
        (p.p166,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign1830_e1635;

        let (assign1840_e1639,) = {
    if (var_guard83 != 0.0) {
        (p.p167,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign1840_e1639;

        let (assign1850_e1643, assign1850_e1643_d_n0, assign1850_e1643_d_n1, assign1850_e1643_d_n2, assign1850_e1643_d_n3, assign1850_e1643_d_n4, assign1850_e1643_d_n5, assign1850_e1643_d_n6, assign1850_e1643_d_n7, assign1850_e1643_d_n8, assign1850_e1643_d_n9, assign1850_e1643_d_b0, assign1850_e1643_d_b1, assign1850_e1643_d_b2, assign1850_e1643_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfr_i, var_cfr_i_dn0, var_cfr_i_dn1, var_cfr_i_dn2, var_cfr_i_dn3, var_cfr_i_dn4, var_cfr_i_dn5, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9, var_cfr_i_db0, var_cfr_i_db1, var_cfr_i_db2, var_cfr_i_db3,)
    }
};
        var_cfr_i = assign1850_e1643;
        var_cfr_i_dn0 = assign1850_e1643_d_n0;
        var_cfr_i_dn1 = assign1850_e1643_d_n1;
        var_cfr_i_dn2 = assign1850_e1643_d_n2;
        var_cfr_i_dn3 = assign1850_e1643_d_n3;
        var_cfr_i_dn4 = assign1850_e1643_d_n4;
        var_cfr_i_dn5 = assign1850_e1643_d_n5;
        var_cfr_i_dn6 = assign1850_e1643_d_n6;
        var_cfr_i_dn7 = assign1850_e1643_d_n7;
        var_cfr_i_dn8 = assign1850_e1643_d_n8;
        var_cfr_i_dn9 = assign1850_e1643_d_n9;
        var_cfr_i_db0 = assign1850_e1643_d_b0;
        var_cfr_i_db1 = assign1850_e1643_d_b1;
        var_cfr_i_db2 = assign1850_e1643_d_b2;
        var_cfr_i_db3 = assign1850_e1643_d_b3;

        let (assign1860_e1647, assign1860_e1647_d_n0, assign1860_e1647_d_n1, assign1860_e1647_d_n2, assign1860_e1647_d_n3, assign1860_e1647_d_n4, assign1860_e1647_d_n5, assign1860_e1647_d_n6, assign1860_e1647_d_n7, assign1860_e1647_d_n8, assign1860_e1647_d_n9, assign1860_e1647_d_b0, assign1860_e1647_d_b1, assign1860_e1647_d_b2, assign1860_e1647_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p169, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cfrd_i, var_cfrd_i_dn0, var_cfrd_i_dn1, var_cfrd_i_dn2, var_cfrd_i_dn3, var_cfrd_i_dn4, var_cfrd_i_dn5, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9, var_cfrd_i_db0, var_cfrd_i_db1, var_cfrd_i_db2, var_cfrd_i_db3,)
    }
};
        var_cfrd_i = assign1860_e1647;
        var_cfrd_i_dn0 = assign1860_e1647_d_n0;
        var_cfrd_i_dn1 = assign1860_e1647_d_n1;
        var_cfrd_i_dn2 = assign1860_e1647_d_n2;
        var_cfrd_i_dn3 = assign1860_e1647_d_n3;
        var_cfrd_i_dn4 = assign1860_e1647_d_n4;
        var_cfrd_i_dn5 = assign1860_e1647_d_n5;
        var_cfrd_i_dn6 = assign1860_e1647_d_n6;
        var_cfrd_i_dn7 = assign1860_e1647_d_n7;
        var_cfrd_i_dn8 = assign1860_e1647_d_n8;
        var_cfrd_i_dn9 = assign1860_e1647_d_n9;
        var_cfrd_i_db0 = assign1860_e1647_d_b0;
        var_cfrd_i_db1 = assign1860_e1647_d_b1;
        var_cfrd_i_db2 = assign1860_e1647_d_b2;
        var_cfrd_i_db3 = assign1860_e1647_d_b3;

        let (assign1870_e1651,) = {
    if (var_guard83 != 0.0) {
        (p.p170,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign1870_e1651;

        let (assign1880_e1655,) = {
    if (var_guard83 != 0.0) {
        (p.p171,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign1880_e1655;

        let (assign1890_e1659, assign1890_e1659_d_n0, assign1890_e1659_d_n1, assign1890_e1659_d_n2, assign1890_e1659_d_n3, assign1890_e1659_d_n4, assign1890_e1659_d_n5, assign1890_e1659_d_n6, assign1890_e1659_d_n7, assign1890_e1659_d_n8, assign1890_e1659_d_n9, assign1890_e1659_d_b0, assign1890_e1659_d_b1, assign1890_e1659_d_b2, assign1890_e1659_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rth_t, var_rth_t_dn0, var_rth_t_dn1, var_rth_t_dn2, var_rth_t_dn3, var_rth_t_dn4, var_rth_t_dn5, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9, var_rth_t_db0, var_rth_t_db1, var_rth_t_db2, var_rth_t_db3,)
    }
};
        var_rth_t = assign1890_e1659;
        var_rth_t_dn0 = assign1890_e1659_d_n0;
        var_rth_t_dn1 = assign1890_e1659_d_n1;
        var_rth_t_dn2 = assign1890_e1659_d_n2;
        var_rth_t_dn3 = assign1890_e1659_d_n3;
        var_rth_t_dn4 = assign1890_e1659_d_n4;
        var_rth_t_dn5 = assign1890_e1659_d_n5;
        var_rth_t_dn6 = assign1890_e1659_d_n6;
        var_rth_t_dn7 = assign1890_e1659_d_n7;
        var_rth_t_dn8 = assign1890_e1659_d_n8;
        var_rth_t_dn9 = assign1890_e1659_d_n9;
        var_rth_t_db0 = assign1890_e1659_d_b0;
        var_rth_t_db1 = assign1890_e1659_d_b1;
        var_rth_t_db2 = assign1890_e1659_d_b2;
        var_rth_t_db3 = assign1890_e1659_d_b3;

        let (assign1900_e1663,) = {
    if (var_guard83 != 0.0) {
        (p.p173,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign1900_e1663;

        let (assign1910_e1667, assign1910_e1667_d_n0, assign1910_e1667_d_n1, assign1910_e1667_d_n2, assign1910_e1667_d_n3, assign1910_e1667_d_n4, assign1910_e1667_d_n5, assign1910_e1667_d_n6, assign1910_e1667_d_n7, assign1910_e1667_d_n8, assign1910_e1667_d_n9, assign1910_e1667_d_b0, assign1910_e1667_d_b1, assign1910_e1667_d_b2, assign1910_e1667_d_b3,) = {
    if (var_guard83 != 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cth_i, var_cth_i_dn0, var_cth_i_dn1, var_cth_i_dn2, var_cth_i_dn3, var_cth_i_dn4, var_cth_i_dn5, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9, var_cth_i_db0, var_cth_i_db1, var_cth_i_db2, var_cth_i_db3,)
    }
};
        var_cth_i = assign1910_e1667;
        var_cth_i_dn0 = assign1910_e1667_d_n0;
        var_cth_i_dn1 = assign1910_e1667_d_n1;
        var_cth_i_dn2 = assign1910_e1667_d_n2;
        var_cth_i_dn3 = assign1910_e1667_d_n3;
        var_cth_i_dn4 = assign1910_e1667_d_n4;
        var_cth_i_dn5 = assign1910_e1667_d_n5;
        var_cth_i_dn6 = assign1910_e1667_d_n6;
        var_cth_i_dn7 = assign1910_e1667_d_n7;
        var_cth_i_dn8 = assign1910_e1667_d_n8;
        var_cth_i_dn9 = assign1910_e1667_d_n9;
        var_cth_i_db0 = assign1910_e1667_d_b0;
        var_cth_i_db1 = assign1910_e1667_d_b1;
        var_cth_i_db2 = assign1910_e1667_d_b2;
        var_cth_i_db3 = assign1910_e1667_d_b3;

        let (assign1920_e1671,) = {
    if (var_guard83 != 0.0) {
        (p.p175,)
    } else {
        (var_fnt_i,)
    }
};
        var_fnt_i = assign1920_e1671;

        let (assign1930_e1675,) = {
    if (var_guard83 != 0.0) {
        (p.p176,)
    } else {
        (var_fntexc_i,)
    }
};
        var_fntexc_i = assign1930_e1675;

        let (assign1940_e1679,) = {
    if (var_guard83 != 0.0) {
        (p.p177,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign1940_e1679;

        let (assign1950_e1683,) = {
    if (var_guard83 != 0.0) {
        (p.p178,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign1950_e1683;

        let (assign1960_e1687,) = {
    if (var_guard83 != 0.0) {
        (p.p179,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign1960_e1687;

        let (assign1970_e1691,) = {
    if (var_guard83 != 0.0) {
        (p.p180,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign1970_e1691;

        let (assign1980_e1695,) = {
    if (var_guard83 != 0.0) {
        (p.p181,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign1980_e1695;

        let (assign2040_e1722,) = {
    if (var_guard83 == 0.0) {
        let assign2040_e1720: f64 = (1.0 / p.p29);
        (assign2040_e1720,)
    } else {
        (var_invnf,)
    }
};
        var_invnf = assign2040_e1722;

        let (assign2050_e1731,) = {
    if (var_guard83 == 0.0) {
        let assign2050_e1727: f64 = (p.p21 * var_invnf);
        let assign2050_e1729: f64 = (assign2050_e1727).max(1e-9);
        (assign2050_e1729,)
    } else {
        (var_w_i,)
    }
};
        var_w_i = assign2050_e1731;

        let (assign2060_e1738,) = {
    if (var_guard83 == 0.0) {
        let assign2060_e1736: f64 = (p.p23 * var_invnf);
        (assign2060_e1736,)
    } else {
        (var_adrain_i,)
    }
};
        var_adrain_i = assign2060_e1738;

        let (assign2070_e1745,) = {
    if (var_guard83 == 0.0) {
        let assign2070_e1743: f64 = (p.p22 * var_invnf);
        (assign2070_e1743,)
    } else {
        (var_asource_i,)
    }
};
        var_asource_i = assign2070_e1745;

        let (assign2080_e1752,) = {
    if (var_guard83 == 0.0) {
        let assign2080_e1750: f64 = (p.p25 * var_invnf);
        (assign2080_e1750,)
    } else {
        (var_pdrain_i,)
    }
};
        var_pdrain_i = assign2080_e1752;

        let (assign2090_e1759,) = {
    if (var_guard83 == 0.0) {
        let assign2090_e1757: f64 = (p.p24 * var_invnf);
        (assign2090_e1757,)
    } else {
        (var_psource_i,)
    }
};
        var_psource_i = assign2090_e1759;

        let (assign2100_e1766,) = {
    if (var_guard83 == 0.0) {
        let assign2100_e1764: f64 = (p.p30 * p.p29);
        (assign2100_e1764,)
    } else {
        (var_mult_i_int,)
    }
};
        var_mult_i_int = assign2100_e1766;

        let (assign2110_e1771,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_len,)
    }
};
        var_len = assign2110_e1771;

        *var_adrain_i_slot = var_adrain_i;
        *var_alpac_i_slot = var_alpac_i;
        *var_asource_i_slot = var_asource_i;
        *var_axac_i_slot = var_axac_i;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_db0_slot = var_cfac1_t_db0;
        *var_cfac1_t_db1_slot = var_cfac1_t_db1;
        *var_cfac1_t_db2_slot = var_cfac1_t_db2;
        *var_cfac1_t_db3_slot = var_cfac1_t_db3;
        *var_cfac1_t_dn0_slot = var_cfac1_t_dn0;
        *var_cfac1_t_dn1_slot = var_cfac1_t_dn1;
        *var_cfac1_t_dn2_slot = var_cfac1_t_dn2;
        *var_cfac1_t_dn3_slot = var_cfac1_t_dn3;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn5_slot = var_cfac1_t_dn5;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_db0_slot = var_cfac2_t_db0;
        *var_cfac2_t_db1_slot = var_cfac2_t_db1;
        *var_cfac2_t_db2_slot = var_cfac2_t_db2;
        *var_cfac2_t_db3_slot = var_cfac2_t_db3;
        *var_cfac2_t_dn0_slot = var_cfac2_t_dn0;
        *var_cfac2_t_dn1_slot = var_cfac2_t_dn1;
        *var_cfac2_t_dn2_slot = var_cfac2_t_dn2;
        *var_cfac2_t_dn3_slot = var_cfac2_t_dn3;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn5_slot = var_cfac2_t_dn5;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_db0_slot = var_cfr_i_db0;
        *var_cfr_i_db1_slot = var_cfr_i_db1;
        *var_cfr_i_db2_slot = var_cfr_i_db2;
        *var_cfr_i_db3_slot = var_cfr_i_db3;
        *var_cfr_i_dn0_slot = var_cfr_i_dn0;
        *var_cfr_i_dn1_slot = var_cfr_i_dn1;
        *var_cfr_i_dn2_slot = var_cfr_i_dn2;
        *var_cfr_i_dn3_slot = var_cfr_i_dn3;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn5_slot = var_cfr_i_dn5;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_db0_slot = var_cfrd_i_db0;
        *var_cfrd_i_db1_slot = var_cfrd_i_db1;
        *var_cfrd_i_db2_slot = var_cfrd_i_db2;
        *var_cfrd_i_db3_slot = var_cfrd_i_db3;
        *var_cfrd_i_dn0_slot = var_cfrd_i_dn0;
        *var_cfrd_i_dn1_slot = var_cfrd_i_dn1;
        *var_cfrd_i_dn2_slot = var_cfrd_i_dn2;
        *var_cfrd_i_dn3_slot = var_cfrd_i_dn3;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn5_slot = var_cfrd_i_dn5;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_db0_slot = var_cov_i_db0;
        *var_cov_i_db1_slot = var_cov_i_db1;
        *var_cov_i_db2_slot = var_cov_i_db2;
        *var_cov_i_db3_slot = var_cov_i_db3;
        *var_cov_i_dn0_slot = var_cov_i_dn0;
        *var_cov_i_dn1_slot = var_cov_i_dn1;
        *var_cov_i_dn2_slot = var_cov_i_dn2;
        *var_cov_i_dn3_slot = var_cov_i_dn3;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn5_slot = var_cov_i_dn5;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_db0_slot = var_covd_i_db0;
        *var_covd_i_db1_slot = var_covd_i_db1;
        *var_covd_i_db2_slot = var_covd_i_db2;
        *var_covd_i_db3_slot = var_covd_i_db3;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn1_slot = var_covd_i_dn1;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
        *var_covd_i_dn6_slot = var_covd_i_dn6;
        *var_covd_i_dn7_slot = var_covd_i_dn7;
        *var_covd_i_dn8_slot = var_covd_i_dn8;
        *var_covd_i_dn9_slot = var_covd_i_dn9;
        *var_covdl_i_slot = var_covdl_i;
        *var_covdlb_i_slot = var_covdlb_i;
        *var_csd_i_slot = var_csd_i;
        *var_csdbp_i_slot = var_csdbp_i;
        *var_cth_i_slot = var_cth_i;
        *var_cth_i_db0_slot = var_cth_i_db0;
        *var_cth_i_db1_slot = var_cth_i_db1;
        *var_cth_i_db2_slot = var_cth_i_db2;
        *var_cth_i_db3_slot = var_cth_i_db3;
        *var_cth_i_dn0_slot = var_cth_i_dn0;
        *var_cth_i_dn1_slot = var_cth_i_dn1;
        *var_cth_i_dn2_slot = var_cth_i_dn2;
        *var_cth_i_dn3_slot = var_cth_i_dn3;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn5_slot = var_cth_i_dn5;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_dvfbov_i_slot = var_dvfbov_i;
        *var_fnt_i_slot = var_fnt_i;
        *var_fntexc_i_slot = var_fntexc_i;
        *var_guard90_slot = var_guard90;
        *var_guard91_slot = var_guard91;
        *var_guard92_slot = var_guard92;
        *var_guard93_slot = var_guard93;
        *var_invnf_slot = var_invnf;
        *var_len_slot = var_len;
        *var_mult_i_int_slot = var_mult_i_int;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_pdrain_i_slot = var_pdrain_i;
        *var_psource_i_slot = var_psource_i;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_db0_slot = var_rth_t_db0;
        *var_rth_t_db1_slot = var_rth_t_db1;
        *var_rth_t_db2_slot = var_rth_t_db2;
        *var_rth_t_db3_slot = var_rth_t_db3;
        *var_rth_t_dn0_slot = var_rth_t_dn0;
        *var_rth_t_dn1_slot = var_rth_t_dn1;
        *var_rth_t_dn2_slot = var_rth_t_dn2;
        *var_rth_t_dn3_slot = var_rth_t_dn3;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn5_slot = var_rth_t_dn5;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_strth_i_slot = var_strth_i;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_db0_slot = var_thesatac_t_db0;
        *var_thesatac_t_db1_slot = var_thesatac_t_db1;
        *var_thesatac_t_db2_slot = var_thesatac_t_db2;
        *var_thesatac_t_db3_slot = var_thesatac_t_db3;
        *var_thesatac_t_dn0_slot = var_thesatac_t_dn0;
        *var_thesatac_t_dn1_slot = var_thesatac_t_dn1;
        *var_thesatac_t_dn2_slot = var_thesatac_t_dn2;
        *var_thesatac_t_dn3_slot = var_thesatac_t_dn3;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn5_slot = var_thesatac_t_dn5;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_w_i_slot = var_w_i;
    }

    pub(super) fn stamp_transient_block_5(
        p: &Parameters,
        var_guard83: f64,
        var_len: f64,
        var_w_i: f64,
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
        var_lphy_slot: &mut f64,
        var_lphy_db0_slot: &mut f64,
        var_lphy_db1_slot: &mut f64,
        var_lphy_db2_slot: &mut f64,
        var_lphy_db3_slot: &mut f64,
        var_lphy_dn0_slot: &mut f64,
        var_lphy_dn1_slot: &mut f64,
        var_lphy_dn2_slot: &mut f64,
        var_lphy_dn3_slot: &mut f64,
        var_lphy_dn4_slot: &mut f64,
        var_lphy_dn5_slot: &mut f64,
        var_lphy_dn6_slot: &mut f64,
        var_lphy_dn7_slot: &mut f64,
        var_lphy_dn8_slot: &mut f64,
        var_lphy_dn9_slot: &mut f64,
        var_nch_i_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_nsub_i_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
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
        var_vfb1_t_db0_slot: &mut f64,
        var_vfb1_t_db1_slot: &mut f64,
        var_vfb1_t_db2_slot: &mut f64,
        var_vfb1_t_db3_slot: &mut f64,
        var_vfb1_t_dn0_slot: &mut f64,
        var_vfb1_t_dn1_slot: &mut f64,
        var_vfb1_t_dn2_slot: &mut f64,
        var_vfb1_t_dn3_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn5_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_db0_slot: &mut f64,
        var_vfb2_t_db1_slot: &mut f64,
        var_vfb2_t_db2_slot: &mut f64,
        var_vfb2_t_db3_slot: &mut f64,
        var_vfb2_t_dn0_slot: &mut f64,
        var_vfb2_t_dn1_slot: &mut f64,
        var_vfb2_t_dn2_slot: &mut f64,
        var_vfb2_t_dn3_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn5_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_we_slot: &mut f64,
        var_wecv_slot: &mut f64,
        var_wen_slot: &mut f64,
        var_wphy_slot: &mut f64,
        var_wphy_db0_slot: &mut f64,
        var_wphy_db1_slot: &mut f64,
        var_wphy_db2_slot: &mut f64,
        var_wphy_db3_slot: &mut f64,
        var_wphy_dn0_slot: &mut f64,
        var_wphy_dn1_slot: &mut f64,
        var_wphy_dn2_slot: &mut f64,
        var_wphy_dn3_slot: &mut f64,
        var_wphy_dn4_slot: &mut f64,
        var_wphy_dn5_slot: &mut f64,
        var_wphy_dn6_slot: &mut f64,
        var_wphy_dn7_slot: &mut f64,
        var_wphy_dn8_slot: &mut f64,
        var_wphy_dn9_slot: &mut f64,
        var_xge_i_slot: &mut f64,
    ) {
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
        let mut var_lphy: f64 = *var_lphy_slot;
        let mut var_lphy_db0: f64 = *var_lphy_db0_slot;
        let mut var_lphy_db1: f64 = *var_lphy_db1_slot;
        let mut var_lphy_db2: f64 = *var_lphy_db2_slot;
        let mut var_lphy_db3: f64 = *var_lphy_db3_slot;
        let mut var_lphy_dn0: f64 = *var_lphy_dn0_slot;
        let mut var_lphy_dn1: f64 = *var_lphy_dn1_slot;
        let mut var_lphy_dn2: f64 = *var_lphy_dn2_slot;
        let mut var_lphy_dn3: f64 = *var_lphy_dn3_slot;
        let mut var_lphy_dn4: f64 = *var_lphy_dn4_slot;
        let mut var_lphy_dn5: f64 = *var_lphy_dn5_slot;
        let mut var_lphy_dn6: f64 = *var_lphy_dn6_slot;
        let mut var_lphy_dn7: f64 = *var_lphy_dn7_slot;
        let mut var_lphy_dn8: f64 = *var_lphy_dn8_slot;
        let mut var_lphy_dn9: f64 = *var_lphy_dn9_slot;
        let mut var_nch_i: f64 = *var_nch_i_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_nsub_i: f64 = *var_nsub_i_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
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
        let mut var_vfb1_t_db0: f64 = *var_vfb1_t_db0_slot;
        let mut var_vfb1_t_db1: f64 = *var_vfb1_t_db1_slot;
        let mut var_vfb1_t_db2: f64 = *var_vfb1_t_db2_slot;
        let mut var_vfb1_t_db3: f64 = *var_vfb1_t_db3_slot;
        let mut var_vfb1_t_dn0: f64 = *var_vfb1_t_dn0_slot;
        let mut var_vfb1_t_dn1: f64 = *var_vfb1_t_dn1_slot;
        let mut var_vfb1_t_dn2: f64 = *var_vfb1_t_dn2_slot;
        let mut var_vfb1_t_dn3: f64 = *var_vfb1_t_dn3_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn5: f64 = *var_vfb1_t_dn5_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_db0: f64 = *var_vfb2_t_db0_slot;
        let mut var_vfb2_t_db1: f64 = *var_vfb2_t_db1_slot;
        let mut var_vfb2_t_db2: f64 = *var_vfb2_t_db2_slot;
        let mut var_vfb2_t_db3: f64 = *var_vfb2_t_db3_slot;
        let mut var_vfb2_t_dn0: f64 = *var_vfb2_t_dn0_slot;
        let mut var_vfb2_t_dn1: f64 = *var_vfb2_t_dn1_slot;
        let mut var_vfb2_t_dn2: f64 = *var_vfb2_t_dn2_slot;
        let mut var_vfb2_t_dn3: f64 = *var_vfb2_t_dn3_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn5: f64 = *var_vfb2_t_dn5_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_we: f64 = *var_we_slot;
        let mut var_wecv: f64 = *var_wecv_slot;
        let mut var_wen: f64 = *var_wen_slot;
        let mut var_wphy: f64 = *var_wphy_slot;
        let mut var_wphy_db0: f64 = *var_wphy_db0_slot;
        let mut var_wphy_db1: f64 = *var_wphy_db1_slot;
        let mut var_wphy_db2: f64 = *var_wphy_db2_slot;
        let mut var_wphy_db3: f64 = *var_wphy_db3_slot;
        let mut var_wphy_dn0: f64 = *var_wphy_dn0_slot;
        let mut var_wphy_dn1: f64 = *var_wphy_dn1_slot;
        let mut var_wphy_dn2: f64 = *var_wphy_dn2_slot;
        let mut var_wphy_dn3: f64 = *var_wphy_dn3_slot;
        let mut var_wphy_dn4: f64 = *var_wphy_dn4_slot;
        let mut var_wphy_dn5: f64 = *var_wphy_dn5_slot;
        let mut var_wphy_dn6: f64 = *var_wphy_dn6_slot;
        let mut var_wphy_dn7: f64 = *var_wphy_dn7_slot;
        let mut var_wphy_dn8: f64 = *var_wphy_dn8_slot;
        let mut var_wphy_dn9: f64 = *var_wphy_dn9_slot;
        let mut var_xge_i: f64 = *var_xge_i_slot;

        let (assign2120_e1776,) = {
    if (var_guard83 == 0.0) {
        (1e-6,)
    } else {
        (var_wen,)
    }
};
        var_wen = assign2120_e1776;

        let (assign2130_e1783,) = {
    if (var_guard83 == 0.0) {
        let assign2130_e1781: f64 = (var_len / p.p20);
        (assign2130_e1781,)
    } else {
        (var_il,)
    }
};
        var_il = assign2130_e1783;

        let (assign2140_e1790,) = {
    if (var_guard83 == 0.0) {
        let assign2140_e1788: f64 = (var_wen / var_w_i);
        (assign2140_e1788,)
    } else {
        (var_iw,)
    }
};
        var_iw = assign2140_e1790;

        let (assign2150_e1807,) = {
    if (var_guard83 == 0.0) {
        let assign2150_e1797: f64 = (p.p188 * var_il);
        let assign2150_e1798: f64 = (1.0 + assign2150_e1797);
        let assign2150_e1799: f64 = (p.p187 * assign2150_e1798);
        let assign2150_e1803: f64 = (p.p189 * var_iw);
        let assign2150_e1804: f64 = (1.0 + assign2150_e1803);
        let assign2150_e1805: f64 = (assign2150_e1799 * assign2150_e1804);
        (assign2150_e1805,)
    } else {
        (var_dellps,)
    }
};
        var_dellps = assign2150_e1807;

        let (assign2160_e1824,) = {
    if (var_guard83 == 0.0) {
        let assign2160_e1814: f64 = (p.p193 * var_iw);
        let assign2160_e1815: f64 = (1.0 + assign2160_e1814);
        let assign2160_e1816: f64 = (p.p191 * assign2160_e1815);
        let assign2160_e1820: f64 = (p.p192 * var_il);
        let assign2160_e1821: f64 = (1.0 + assign2160_e1820);
        let assign2160_e1822: f64 = (assign2160_e1816 * assign2160_e1821);
        (assign2160_e1822,)
    } else {
        (var_delwod,)
    }
};
        var_delwod = assign2160_e1824;

        let (assign2170_e1837,) = {
    if (var_guard83 == 0.0) {
        let assign2170_e1829: f64 = (p.p20 + var_dellps);
        let assign2170_e1832: f64 = (2.0 * p.p190);
        let assign2170_e1833: f64 = (assign2170_e1829 - assign2170_e1832);
        let assign2170_e1835: f64 = (assign2170_e1833).max(1e-9);
        (assign2170_e1835,)
    } else {
        (var_le,)
    }
};
        var_le = assign2170_e1837;

        let (assign2180_e1850,) = {
    if (var_guard83 == 0.0) {
        let assign2180_e1842: f64 = (var_w_i + var_delwod);
        let assign2180_e1845: f64 = (2.0 * p.p194);
        let assign2180_e1846: f64 = (assign2180_e1842 - assign2180_e1845);
        let assign2180_e1848: f64 = (assign2180_e1846).max(1e-9);
        (assign2180_e1848,)
    } else {
        (var_we,)
    }
};
        var_we = assign2180_e1850;

        let (assign2190_e1865,) = {
    if (var_guard83 == 0.0) {
        let assign2190_e1855: f64 = (p.p20 + var_dellps);
        let assign2190_e1858: f64 = (2.0 * p.p190);
        let assign2190_e1859: f64 = (assign2190_e1855 - assign2190_e1858);
        let assign2190_e1861: f64 = (assign2190_e1859 + p.p195);
        let assign2190_e1863: f64 = (assign2190_e1861).max(1e-9);
        (assign2190_e1863,)
    } else {
        (var_lecv,)
    }
};
        var_lecv = assign2190_e1865;

        let (assign2200_e1880,) = {
    if (var_guard83 == 0.0) {
        let assign2200_e1870: f64 = (var_w_i + var_delwod);
        let assign2200_e1873: f64 = (2.0 * p.p194);
        let assign2200_e1874: f64 = (assign2200_e1870 - assign2200_e1873);
        let assign2200_e1876: f64 = (assign2200_e1874 + p.p196);
        let assign2200_e1878: f64 = (assign2200_e1876).max(1e-9);
        (assign2200_e1878,)
    } else {
        (var_wecv,)
    }
};
        var_wecv = assign2200_e1880;

        let (assign2210_e1887,) = {
    if (var_guard83 == 0.0) {
        let assign2210_e1885: f64 = (var_len / var_le);
        (assign2210_e1885,)
    } else {
        (var_ile,)
    }
};
        var_ile = assign2210_e1887;

        let (assign2220_e1894,) = {
    if (var_guard83 == 0.0) {
        let assign2220_e1892: f64 = (var_wen / var_we);
        (assign2220_e1892,)
    } else {
        (var_iwe,)
    }
};
        var_iwe = assign2220_e1894;

        let (assign2230_e1901,) = {
    if (var_guard83 == 0.0) {
        let assign2230_e1899: f64 = (var_ile * var_iwe);
        (assign2230_e1899,)
    } else {
        (var_iae,)
    }
};
        var_iae = assign2230_e1901;

        let (assign2240_e1910, assign2240_e1910_d_n0, assign2240_e1910_d_n1, assign2240_e1910_d_n2, assign2240_e1910_d_n3, assign2240_e1910_d_n4, assign2240_e1910_d_n5, assign2240_e1910_d_n6, assign2240_e1910_d_n7, assign2240_e1910_d_n8, assign2240_e1910_d_n9, assign2240_e1910_d_b0, assign2240_e1910_d_b1, assign2240_e1910_d_b2, assign2240_e1910_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2240_e1906: f64 = (p.p20 + var_dellps);
        let assign2240_e1908: f64 = (assign2240_e1906).max(1e-9);
        (assign2240_e1908, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign2240_e1910;
        var_temp_dn0 = assign2240_e1910_d_n0;
        var_temp_dn1 = assign2240_e1910_d_n1;
        var_temp_dn2 = assign2240_e1910_d_n2;
        var_temp_dn3 = assign2240_e1910_d_n3;
        var_temp_dn4 = assign2240_e1910_d_n4;
        var_temp_dn5 = assign2240_e1910_d_n5;
        var_temp_dn6 = assign2240_e1910_d_n6;
        var_temp_dn7 = assign2240_e1910_d_n7;
        var_temp_dn8 = assign2240_e1910_d_n8;
        var_temp_dn9 = assign2240_e1910_d_n9;
        var_temp_db0 = assign2240_e1910_d_b0;
        var_temp_db1 = assign2240_e1910_d_b1;
        var_temp_db2 = assign2240_e1910_d_b2;
        var_temp_db3 = assign2240_e1910_d_b3;

        let (assign2250_e1917, assign2250_e1917_d_n0, assign2250_e1917_d_n1, assign2250_e1917_d_n2, assign2250_e1917_d_n3, assign2250_e1917_d_n4, assign2250_e1917_d_n5, assign2250_e1917_d_n6, assign2250_e1917_d_n7, assign2250_e1917_d_n8, assign2250_e1917_d_n9, assign2250_e1917_d_b0, assign2250_e1917_d_b1, assign2250_e1917_d_b2, assign2250_e1917_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2250_e1915: f64 = (var_temp / var_len);
        (assign2250_e1915, (var_temp_dn0 / var_len), (var_temp_dn1 / var_len), (var_temp_dn2 / var_len), (var_temp_dn3 / var_len), (var_temp_dn4 / var_len), (var_temp_dn5 / var_len), (var_temp_dn6 / var_len), (var_temp_dn7 / var_len), (var_temp_dn8 / var_len), (var_temp_dn9 / var_len), (var_temp_db0 / var_len), (var_temp_db1 / var_len), (var_temp_db2 / var_len), (var_temp_db3 / var_len),)
    } else {
        (var_lphy, var_lphy_dn0, var_lphy_dn1, var_lphy_dn2, var_lphy_dn3, var_lphy_dn4, var_lphy_dn5, var_lphy_dn6, var_lphy_dn7, var_lphy_dn8, var_lphy_dn9, var_lphy_db0, var_lphy_db1, var_lphy_db2, var_lphy_db3,)
    }
};
        var_lphy = assign2250_e1917;
        var_lphy_dn0 = assign2250_e1917_d_n0;
        var_lphy_dn1 = assign2250_e1917_d_n1;
        var_lphy_dn2 = assign2250_e1917_d_n2;
        var_lphy_dn3 = assign2250_e1917_d_n3;
        var_lphy_dn4 = assign2250_e1917_d_n4;
        var_lphy_dn5 = assign2250_e1917_d_n5;
        var_lphy_dn6 = assign2250_e1917_d_n6;
        var_lphy_dn7 = assign2250_e1917_d_n7;
        var_lphy_dn8 = assign2250_e1917_d_n8;
        var_lphy_dn9 = assign2250_e1917_d_n9;
        var_lphy_db0 = assign2250_e1917_d_b0;
        var_lphy_db1 = assign2250_e1917_d_b1;
        var_lphy_db2 = assign2250_e1917_d_b2;
        var_lphy_db3 = assign2250_e1917_d_b3;

        let (assign2260_e1926, assign2260_e1926_d_n0, assign2260_e1926_d_n1, assign2260_e1926_d_n2, assign2260_e1926_d_n3, assign2260_e1926_d_n4, assign2260_e1926_d_n5, assign2260_e1926_d_n6, assign2260_e1926_d_n7, assign2260_e1926_d_n8, assign2260_e1926_d_n9, assign2260_e1926_d_b0, assign2260_e1926_d_b1, assign2260_e1926_d_b2, assign2260_e1926_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2260_e1922: f64 = (var_w_i + var_delwod);
        let assign2260_e1924: f64 = (assign2260_e1922).max(1e-9);
        (assign2260_e1924, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign2260_e1926;
        var_temp_dn0 = assign2260_e1926_d_n0;
        var_temp_dn1 = assign2260_e1926_d_n1;
        var_temp_dn2 = assign2260_e1926_d_n2;
        var_temp_dn3 = assign2260_e1926_d_n3;
        var_temp_dn4 = assign2260_e1926_d_n4;
        var_temp_dn5 = assign2260_e1926_d_n5;
        var_temp_dn6 = assign2260_e1926_d_n6;
        var_temp_dn7 = assign2260_e1926_d_n7;
        var_temp_dn8 = assign2260_e1926_d_n8;
        var_temp_dn9 = assign2260_e1926_d_n9;
        var_temp_db0 = assign2260_e1926_d_b0;
        var_temp_db1 = assign2260_e1926_d_b1;
        var_temp_db2 = assign2260_e1926_d_b2;
        var_temp_db3 = assign2260_e1926_d_b3;

        let (assign2270_e1933, assign2270_e1933_d_n0, assign2270_e1933_d_n1, assign2270_e1933_d_n2, assign2270_e1933_d_n3, assign2270_e1933_d_n4, assign2270_e1933_d_n5, assign2270_e1933_d_n6, assign2270_e1933_d_n7, assign2270_e1933_d_n8, assign2270_e1933_d_n9, assign2270_e1933_d_b0, assign2270_e1933_d_b1, assign2270_e1933_d_b2, assign2270_e1933_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2270_e1931: f64 = (var_temp / var_wen);
        (assign2270_e1931, (var_temp_dn0 / var_wen), (var_temp_dn1 / var_wen), (var_temp_dn2 / var_wen), (var_temp_dn3 / var_wen), (var_temp_dn4 / var_wen), (var_temp_dn5 / var_wen), (var_temp_dn6 / var_wen), (var_temp_dn7 / var_wen), (var_temp_dn8 / var_wen), (var_temp_dn9 / var_wen), (var_temp_db0 / var_wen), (var_temp_db1 / var_wen), (var_temp_db2 / var_wen), (var_temp_db3 / var_wen),)
    } else {
        (var_wphy, var_wphy_dn0, var_wphy_dn1, var_wphy_dn2, var_wphy_dn3, var_wphy_dn4, var_wphy_dn5, var_wphy_dn6, var_wphy_dn7, var_wphy_dn8, var_wphy_dn9, var_wphy_db0, var_wphy_db1, var_wphy_db2, var_wphy_db3,)
    }
};
        var_wphy = assign2270_e1933;
        var_wphy_dn0 = assign2270_e1933_d_n0;
        var_wphy_dn1 = assign2270_e1933_d_n1;
        var_wphy_dn2 = assign2270_e1933_d_n2;
        var_wphy_dn3 = assign2270_e1933_d_n3;
        var_wphy_dn4 = assign2270_e1933_d_n4;
        var_wphy_dn5 = assign2270_e1933_d_n5;
        var_wphy_dn6 = assign2270_e1933_d_n6;
        var_wphy_dn7 = assign2270_e1933_d_n7;
        var_wphy_dn8 = assign2270_e1933_d_n8;
        var_wphy_dn9 = assign2270_e1933_d_n9;
        var_wphy_db0 = assign2270_e1933_d_b0;
        var_wphy_db1 = assign2270_e1933_d_b1;
        var_wphy_db2 = assign2270_e1933_d_b2;
        var_wphy_db3 = assign2270_e1933_d_b3;

        let (assign2320_e1976,) = {
    if (var_guard83 == 0.0) {
        (p.p197,)
    } else {
        (var_tox1_i,)
    }
};
        var_tox1_i = assign2320_e1976;

        let (assign2330_e1981,) = {
    if (var_guard83 == 0.0) {
        (p.p198,)
    } else {
        (var_tsi_i,)
    }
};
        var_tsi_i = assign2330_e1981;

        let (assign2340_e1986,) = {
    if (var_guard83 == 0.0) {
        (p.p199,)
    } else {
        (var_xge_i,)
    }
};
        var_xge_i = assign2340_e1986;

        let (assign2350_e1991,) = {
    if (var_guard83 == 0.0) {
        (p.p200,)
    } else {
        (var_tox2_i,)
    }
};
        var_tox2_i = assign2350_e1991;

        let (assign2360_e1996,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2360_e1996;

        let assign2370_e1999: f64 = if p.p201 < 0.0 { 1.0 } else { 0.0 };
        var_guard94 = assign2370_e1999;

        let (assign2380_e2007,) = {
    if ((var_guard83 == 0.0) && (var_guard94 != 0.0)) {
        let assign2380_e2005: f64 = (-1.0);
        (assign2380_e2005,)
    } else {
        (var_typech_i,)
    }
};
        var_typech_i = assign2380_e2007;

        let (assign2390_e2017,) = {
    if (var_guard83 == 0.0) {
        let assign2390_e2011: f64 = (p.p201).abs();
        let assign2390_e2013: f64 = (assign2390_e2011).min(1e19);
        let assign2390_e2015: f64 = (assign2390_e2013 * 1000000.0);
        (assign2390_e2015,)
    } else {
        (var_nch_i,)
    }
};
        var_nch_i = assign2390_e2017;

        let (assign2400_e2022,) = {
    if (var_guard83 == 0.0) {
        (1.0,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2400_e2022;

        let assign2410_e2025: f64 = if p.p202 < 0.0 { 1.0 } else { 0.0 };
        var_guard95 = assign2410_e2025;

        let (assign2420_e2033,) = {
    if ((var_guard83 == 0.0) && (var_guard95 != 0.0)) {
        let assign2420_e2031: f64 = (-1.0);
        (assign2420_e2031,)
    } else {
        (var_typesub_i,)
    }
};
        var_typesub_i = assign2420_e2033;

        let (assign2430_e2045,) = {
    if (var_guard83 == 0.0) {
        let assign2430_e2037: f64 = (p.p202).abs();
        let assign2430_e2039: f64 = (assign2430_e2037).max(1e16);
        let assign2430_e2041: f64 = (assign2430_e2039).min(1e21);
        let assign2430_e2043: f64 = (assign2430_e2041 * 1000000.0);
        (assign2430_e2043,)
    } else {
        (var_nsub_i,)
    }
};
        var_nsub_i = assign2430_e2045;

        let (assign2440_e2050,) = {
    if (var_guard83 == 0.0) {
        (p.p203,)
    } else {
        (var_ct_i,)
    }
};
        var_ct_i = assign2440_e2050;

        let (assign2450_e2055,) = {
    if (var_guard83 == 0.0) {
        (p.p204,)
    } else {
        (var_toxp_i,)
    }
};
        var_toxp_i = assign2450_e2055;

        let (assign2460_e2062,) = {
    if (var_guard83 == 0.0) {
        let assign2460_e2060: f64 = (p.p205 * 1000000.0);
        (assign2460_e2060,)
    } else {
        (var_nov_i,)
    }
};
        var_nov_i = assign2460_e2062;

        let (assign2470_e2069,) = {
    if (var_guard83 == 0.0) {
        let assign2470_e2067: f64 = (p.p206 * 1000000.0);
        (assign2470_e2067,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign2470_e2069;

        let (assign2480_e2086, assign2480_e2086_d_n0, assign2480_e2086_d_n1, assign2480_e2086_d_n2, assign2480_e2086_d_n3, assign2480_e2086_d_n4, assign2480_e2086_d_n5, assign2480_e2086_d_n6, assign2480_e2086_d_n7, assign2480_e2086_d_n8, assign2480_e2086_d_n9, assign2480_e2086_d_b0, assign2480_e2086_d_b1, assign2480_e2086_d_b2, assign2480_e2086_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2480_e2075: f64 = (var_ile).powf(p.p209);
        let assign2480_e2076: f64 = (p.p208 * assign2480_e2075);
        let assign2480_e2081: f64 = (var_ile).powf(p.p211);
        let assign2480_e2082: f64 = (p.p210 * assign2480_e2081);
        let assign2480_e2083: f64 = (1.0 + assign2480_e2082);
        let assign2480_e2084: f64 = (assign2480_e2076 / assign2480_e2083);
        (assign2480_e2084, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign2480_e2086;
        var_temp_dn0 = assign2480_e2086_d_n0;
        var_temp_dn1 = assign2480_e2086_d_n1;
        var_temp_dn2 = assign2480_e2086_d_n2;
        var_temp_dn3 = assign2480_e2086_d_n3;
        var_temp_dn4 = assign2480_e2086_d_n4;
        var_temp_dn5 = assign2480_e2086_d_n5;
        var_temp_dn6 = assign2480_e2086_d_n6;
        var_temp_dn7 = assign2480_e2086_d_n7;
        var_temp_dn8 = assign2480_e2086_d_n8;
        var_temp_dn9 = assign2480_e2086_d_n9;
        var_temp_db0 = assign2480_e2086_d_b0;
        var_temp_db1 = assign2480_e2086_d_b1;
        var_temp_db2 = assign2480_e2086_d_b2;
        var_temp_db3 = assign2480_e2086_d_b3;

        let (assign2490_e2101, assign2490_e2101_d_n0, assign2490_e2101_d_n1, assign2490_e2101_d_n2, assign2490_e2101_d_n3, assign2490_e2101_d_n4, assign2490_e2101_d_n5, assign2490_e2101_d_n6, assign2490_e2101_d_n7, assign2490_e2101_d_n8, assign2490_e2101_d_n9, assign2490_e2101_d_b0, assign2490_e2101_d_b1, assign2490_e2101_d_b2, assign2490_e2101_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2490_e2091: f64 = (p.p207 + var_temp);
        let assign2490_e2094: f64 = (p.p212 * var_iwe);
        let assign2490_e2095: f64 = (assign2490_e2091 + assign2490_e2094);
        let assign2490_e2098: f64 = (p.p213 * var_iae);
        let assign2490_e2099: f64 = (assign2490_e2095 + assign2490_e2098);
        (assign2490_e2099, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    } else {
        (var_vfb1_t, var_vfb1_t_dn0, var_vfb1_t_dn1, var_vfb1_t_dn2, var_vfb1_t_dn3, var_vfb1_t_dn4, var_vfb1_t_dn5, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9, var_vfb1_t_db0, var_vfb1_t_db1, var_vfb1_t_db2, var_vfb1_t_db3,)
    }
};
        var_vfb1_t = assign2490_e2101;
        var_vfb1_t_dn0 = assign2490_e2101_d_n0;
        var_vfb1_t_dn1 = assign2490_e2101_d_n1;
        var_vfb1_t_dn2 = assign2490_e2101_d_n2;
        var_vfb1_t_dn3 = assign2490_e2101_d_n3;
        var_vfb1_t_dn4 = assign2490_e2101_d_n4;
        var_vfb1_t_dn5 = assign2490_e2101_d_n5;
        var_vfb1_t_dn6 = assign2490_e2101_d_n6;
        var_vfb1_t_dn7 = assign2490_e2101_d_n7;
        var_vfb1_t_dn8 = assign2490_e2101_d_n8;
        var_vfb1_t_dn9 = assign2490_e2101_d_n9;
        var_vfb1_t_db0 = assign2490_e2101_d_b0;
        var_vfb1_t_db1 = assign2490_e2101_d_b1;
        var_vfb1_t_db2 = assign2490_e2101_d_b2;
        var_vfb1_t_db3 = assign2490_e2101_d_b3;

        let (assign2500_e2114, assign2500_e2114_d_n0, assign2500_e2114_d_n1, assign2500_e2114_d_n2, assign2500_e2114_d_n3, assign2500_e2114_d_n4, assign2500_e2114_d_n5, assign2500_e2114_d_n6, assign2500_e2114_d_n7, assign2500_e2114_d_n8, assign2500_e2114_d_n9, assign2500_e2114_d_b0, assign2500_e2114_d_b1, assign2500_e2114_d_b2, assign2500_e2114_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2500_e2107: f64 = (p.p215 * var_tox2_i);
        let assign2500_e2109: f64 = (assign2500_e2107 / var_tox1_i);
        let assign2500_e2111: f64 = (assign2500_e2109 * var_temp);
        let assign2500_e2112: f64 = (p.p214 + assign2500_e2111);
        (assign2500_e2112, (assign2500_e2109 * var_temp_dn0), (assign2500_e2109 * var_temp_dn1), (assign2500_e2109 * var_temp_dn2), (assign2500_e2109 * var_temp_dn3), (assign2500_e2109 * var_temp_dn4), (assign2500_e2109 * var_temp_dn5), (assign2500_e2109 * var_temp_dn6), (assign2500_e2109 * var_temp_dn7), (assign2500_e2109 * var_temp_dn8), (assign2500_e2109 * var_temp_dn9), (assign2500_e2109 * var_temp_db0), (assign2500_e2109 * var_temp_db1), (assign2500_e2109 * var_temp_db2), (assign2500_e2109 * var_temp_db3),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn0, var_vfb2_t_dn1, var_vfb2_t_dn2, var_vfb2_t_dn3, var_vfb2_t_dn4, var_vfb2_t_dn5, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9, var_vfb2_t_db0, var_vfb2_t_db1, var_vfb2_t_db2, var_vfb2_t_db3,)
    }
};
        var_vfb2_t = assign2500_e2114;
        var_vfb2_t_dn0 = assign2500_e2114_d_n0;
        var_vfb2_t_dn1 = assign2500_e2114_d_n1;
        var_vfb2_t_dn2 = assign2500_e2114_d_n2;
        var_vfb2_t_dn3 = assign2500_e2114_d_n3;
        var_vfb2_t_dn4 = assign2500_e2114_d_n4;
        var_vfb2_t_dn5 = assign2500_e2114_d_n5;
        var_vfb2_t_dn6 = assign2500_e2114_d_n6;
        var_vfb2_t_dn7 = assign2500_e2114_d_n7;
        var_vfb2_t_dn8 = assign2500_e2114_d_n8;
        var_vfb2_t_dn9 = assign2500_e2114_d_n9;
        var_vfb2_t_db0 = assign2500_e2114_d_b0;
        var_vfb2_t_db1 = assign2500_e2114_d_b1;
        var_vfb2_t_db2 = assign2500_e2114_d_b2;
        var_vfb2_t_db3 = assign2500_e2114_d_b3;

        let (assign2510_e2137,) = {
    if (var_guard83 == 0.0) {
        let assign2510_e2121: f64 = (p.p217 * var_ile);
        let assign2510_e2122: f64 = (1.0 + assign2510_e2121);
        let assign2510_e2123: f64 = (p.p216 * assign2510_e2122);
        let assign2510_e2127: f64 = (p.p218 * var_iwe);
        let assign2510_e2128: f64 = (1.0 + assign2510_e2127);
        let assign2510_e2129: f64 = (assign2510_e2123 * assign2510_e2128);
        let assign2510_e2133: f64 = (p.p219 * var_iae);
        let assign2510_e2134: f64 = (1.0 + assign2510_e2133);
        let assign2510_e2135: f64 = (assign2510_e2129 * assign2510_e2134);
        (assign2510_e2135,)
    } else {
        (var_stvfb_i,)
    }
};
        var_stvfb_i = assign2510_e2137;

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
        *var_lphy_slot = var_lphy;
        *var_lphy_db0_slot = var_lphy_db0;
        *var_lphy_db1_slot = var_lphy_db1;
        *var_lphy_db2_slot = var_lphy_db2;
        *var_lphy_db3_slot = var_lphy_db3;
        *var_lphy_dn0_slot = var_lphy_dn0;
        *var_lphy_dn1_slot = var_lphy_dn1;
        *var_lphy_dn2_slot = var_lphy_dn2;
        *var_lphy_dn3_slot = var_lphy_dn3;
        *var_lphy_dn4_slot = var_lphy_dn4;
        *var_lphy_dn5_slot = var_lphy_dn5;
        *var_lphy_dn6_slot = var_lphy_dn6;
        *var_lphy_dn7_slot = var_lphy_dn7;
        *var_lphy_dn8_slot = var_lphy_dn8;
        *var_lphy_dn9_slot = var_lphy_dn9;
        *var_nch_i_slot = var_nch_i;
        *var_nov_i_slot = var_nov_i;
        *var_novd_i_slot = var_novd_i;
        *var_nsub_i_slot = var_nsub_i;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_temp_slot = var_temp;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
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
        *var_vfb1_t_db0_slot = var_vfb1_t_db0;
        *var_vfb1_t_db1_slot = var_vfb1_t_db1;
        *var_vfb1_t_db2_slot = var_vfb1_t_db2;
        *var_vfb1_t_db3_slot = var_vfb1_t_db3;
        *var_vfb1_t_dn0_slot = var_vfb1_t_dn0;
        *var_vfb1_t_dn1_slot = var_vfb1_t_dn1;
        *var_vfb1_t_dn2_slot = var_vfb1_t_dn2;
        *var_vfb1_t_dn3_slot = var_vfb1_t_dn3;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn5_slot = var_vfb1_t_dn5;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_db0_slot = var_vfb2_t_db0;
        *var_vfb2_t_db1_slot = var_vfb2_t_db1;
        *var_vfb2_t_db2_slot = var_vfb2_t_db2;
        *var_vfb2_t_db3_slot = var_vfb2_t_db3;
        *var_vfb2_t_dn0_slot = var_vfb2_t_dn0;
        *var_vfb2_t_dn1_slot = var_vfb2_t_dn1;
        *var_vfb2_t_dn2_slot = var_vfb2_t_dn2;
        *var_vfb2_t_dn3_slot = var_vfb2_t_dn3;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn5_slot = var_vfb2_t_dn5;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_we_slot = var_we;
        *var_wecv_slot = var_wecv;
        *var_wen_slot = var_wen;
        *var_wphy_slot = var_wphy;
        *var_wphy_db0_slot = var_wphy_db0;
        *var_wphy_db1_slot = var_wphy_db1;
        *var_wphy_db2_slot = var_wphy_db2;
        *var_wphy_db3_slot = var_wphy_db3;
        *var_wphy_dn0_slot = var_wphy_dn0;
        *var_wphy_dn1_slot = var_wphy_dn1;
        *var_wphy_dn2_slot = var_wphy_dn2;
        *var_wphy_dn3_slot = var_wphy_dn3;
        *var_wphy_dn4_slot = var_wphy_dn4;
        *var_wphy_dn5_slot = var_wphy_dn5;
        *var_wphy_dn6_slot = var_wphy_dn6;
        *var_wphy_dn7_slot = var_wphy_dn7;
        *var_wphy_dn8_slot = var_wphy_dn8;
        *var_wphy_dn9_slot = var_wphy_dn9;
        *var_xge_i_slot = var_xge_i;
    }

    pub(super) fn stamp_transient_block_6(
        p: &Parameters,
        var_guard83: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_tsi_i: f64,
        var_xge_i: f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_db0_slot: &mut f64,
        var_cf1_t_db1_slot: &mut f64,
        var_cf1_t_db2_slot: &mut f64,
        var_cf1_t_db3_slot: &mut f64,
        var_cf1_t_dn0_slot: &mut f64,
        var_cf1_t_dn1_slot: &mut f64,
        var_cf1_t_dn2_slot: &mut f64,
        var_cf1_t_dn3_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn5_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_db0_slot: &mut f64,
        var_cf2_t_db1_slot: &mut f64,
        var_cf2_t_db2_slot: &mut f64,
        var_cf2_t_db3_slot: &mut f64,
        var_cf2_t_dn0_slot: &mut f64,
        var_cf2_t_dn1_slot: &mut f64,
        var_cf2_t_dn2_slot: &mut f64,
        var_cf2_t_dn3_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn5_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_db0_slot: &mut f64,
        var_cf_p_db1_slot: &mut f64,
        var_cf_p_db2_slot: &mut f64,
        var_cf_p_db3_slot: &mut f64,
        var_cf_p_dn0_slot: &mut f64,
        var_cf_p_dn1_slot: &mut f64,
        var_cf_p_dn2_slot: &mut f64,
        var_cf_p_dn3_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn5_slot: &mut f64,
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
        var_guard96_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_lambda_le_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_db0_slot: &mut f64,
        var_np_i_db1_slot: &mut f64,
        var_np_i_db2_slot: &mut f64,
        var_np_i_db3_slot: &mut f64,
        var_np_i_dn0_slot: &mut f64,
        var_np_i_dn1_slot: &mut f64,
        var_np_i_dn2_slot: &mut f64,
        var_np_i_dn3_slot: &mut f64,
        var_np_i_dn4_slot: &mut f64,
        var_np_i_dn5_slot: &mut f64,
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
        var_stcf_i_db0_slot: &mut f64,
        var_stcf_i_db1_slot: &mut f64,
        var_stcf_i_db2_slot: &mut f64,
        var_stcf_i_db3_slot: &mut f64,
        var_stcf_i_dn0_slot: &mut f64,
        var_stcf_i_dn1_slot: &mut f64,
        var_stcf_i_dn2_slot: &mut f64,
        var_stcf_i_dn3_slot: &mut f64,
        var_stcf_i_dn4_slot: &mut f64,
        var_stcf_i_dn5_slot: &mut f64,
        var_stcf_i_dn6_slot: &mut f64,
        var_stcf_i_dn7_slot: &mut f64,
        var_stcf_i_dn8_slot: &mut f64,
        var_stcf_i_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_db0_slot: &mut f64,
        var_temp0__blk79_db1_slot: &mut f64,
        var_temp0__blk79_db2_slot: &mut f64,
        var_temp0__blk79_db3_slot: &mut f64,
        var_temp0__blk79_dn0_slot: &mut f64,
        var_temp0__blk79_dn1_slot: &mut f64,
        var_temp0__blk79_dn2_slot: &mut f64,
        var_temp0__blk79_dn3_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn5_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_db0_slot: &mut f64,
        var_temp1_db1_slot: &mut f64,
        var_temp1_db2_slot: &mut f64,
        var_temp1_db3_slot: &mut f64,
        var_temp1_dn0_slot: &mut f64,
        var_temp1_dn1_slot: &mut f64,
        var_temp1_dn2_slot: &mut f64,
        var_temp1_dn3_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_db0_slot: &mut f64,
        var_temp2_db1_slot: &mut f64,
        var_temp2_db2_slot: &mut f64,
        var_temp2_db3_slot: &mut f64,
        var_temp2_dn0_slot: &mut f64,
        var_temp2_dn1_slot: &mut f64,
        var_temp2_dn2_slot: &mut f64,
        var_temp2_dn3_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_db0_slot: &mut f64,
        var_temp3_db1_slot: &mut f64,
        var_temp3_db2_slot: &mut f64,
        var_temp3_db3_slot: &mut f64,
        var_temp3_dn0_slot: &mut f64,
        var_temp3_dn1_slot: &mut f64,
        var_temp3_dn2_slot: &mut f64,
        var_temp3_dn3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn5_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_db0_slot: &mut f64,
        var_temp4_db1_slot: &mut f64,
        var_temp4_db2_slot: &mut f64,
        var_temp4_db3_slot: &mut f64,
        var_temp4_dn0_slot: &mut f64,
        var_temp4_dn1_slot: &mut f64,
        var_temp4_dn2_slot: &mut f64,
        var_temp4_dn3_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn5_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
    ) {
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_db0: f64 = *var_cf1_t_db0_slot;
        let mut var_cf1_t_db1: f64 = *var_cf1_t_db1_slot;
        let mut var_cf1_t_db2: f64 = *var_cf1_t_db2_slot;
        let mut var_cf1_t_db3: f64 = *var_cf1_t_db3_slot;
        let mut var_cf1_t_dn0: f64 = *var_cf1_t_dn0_slot;
        let mut var_cf1_t_dn1: f64 = *var_cf1_t_dn1_slot;
        let mut var_cf1_t_dn2: f64 = *var_cf1_t_dn2_slot;
        let mut var_cf1_t_dn3: f64 = *var_cf1_t_dn3_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn5: f64 = *var_cf1_t_dn5_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_db0: f64 = *var_cf2_t_db0_slot;
        let mut var_cf2_t_db1: f64 = *var_cf2_t_db1_slot;
        let mut var_cf2_t_db2: f64 = *var_cf2_t_db2_slot;
        let mut var_cf2_t_db3: f64 = *var_cf2_t_db3_slot;
        let mut var_cf2_t_dn0: f64 = *var_cf2_t_dn0_slot;
        let mut var_cf2_t_dn1: f64 = *var_cf2_t_dn1_slot;
        let mut var_cf2_t_dn2: f64 = *var_cf2_t_dn2_slot;
        let mut var_cf2_t_dn3: f64 = *var_cf2_t_dn3_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn5: f64 = *var_cf2_t_dn5_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_db0: f64 = *var_cf_p_db0_slot;
        let mut var_cf_p_db1: f64 = *var_cf_p_db1_slot;
        let mut var_cf_p_db2: f64 = *var_cf_p_db2_slot;
        let mut var_cf_p_db3: f64 = *var_cf_p_db3_slot;
        let mut var_cf_p_dn0: f64 = *var_cf_p_dn0_slot;
        let mut var_cf_p_dn1: f64 = *var_cf_p_dn1_slot;
        let mut var_cf_p_dn2: f64 = *var_cf_p_dn2_slot;
        let mut var_cf_p_dn3: f64 = *var_cf_p_dn3_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn5: f64 = *var_cf_p_dn5_slot;
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
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_lambda_le: f64 = *var_lambda_le_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_db0: f64 = *var_np_i_db0_slot;
        let mut var_np_i_db1: f64 = *var_np_i_db1_slot;
        let mut var_np_i_db2: f64 = *var_np_i_db2_slot;
        let mut var_np_i_db3: f64 = *var_np_i_db3_slot;
        let mut var_np_i_dn0: f64 = *var_np_i_dn0_slot;
        let mut var_np_i_dn1: f64 = *var_np_i_dn1_slot;
        let mut var_np_i_dn2: f64 = *var_np_i_dn2_slot;
        let mut var_np_i_dn3: f64 = *var_np_i_dn3_slot;
        let mut var_np_i_dn4: f64 = *var_np_i_dn4_slot;
        let mut var_np_i_dn5: f64 = *var_np_i_dn5_slot;
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
        let mut var_stcf_i_db0: f64 = *var_stcf_i_db0_slot;
        let mut var_stcf_i_db1: f64 = *var_stcf_i_db1_slot;
        let mut var_stcf_i_db2: f64 = *var_stcf_i_db2_slot;
        let mut var_stcf_i_db3: f64 = *var_stcf_i_db3_slot;
        let mut var_stcf_i_dn0: f64 = *var_stcf_i_dn0_slot;
        let mut var_stcf_i_dn1: f64 = *var_stcf_i_dn1_slot;
        let mut var_stcf_i_dn2: f64 = *var_stcf_i_dn2_slot;
        let mut var_stcf_i_dn3: f64 = *var_stcf_i_dn3_slot;
        let mut var_stcf_i_dn4: f64 = *var_stcf_i_dn4_slot;
        let mut var_stcf_i_dn5: f64 = *var_stcf_i_dn5_slot;
        let mut var_stcf_i_dn6: f64 = *var_stcf_i_dn6_slot;
        let mut var_stcf_i_dn7: f64 = *var_stcf_i_dn7_slot;
        let mut var_stcf_i_dn8: f64 = *var_stcf_i_dn8_slot;
        let mut var_stcf_i_dn9: f64 = *var_stcf_i_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_db0: f64 = *var_temp0__blk79_db0_slot;
        let mut var_temp0__blk79_db1: f64 = *var_temp0__blk79_db1_slot;
        let mut var_temp0__blk79_db2: f64 = *var_temp0__blk79_db2_slot;
        let mut var_temp0__blk79_db3: f64 = *var_temp0__blk79_db3_slot;
        let mut var_temp0__blk79_dn0: f64 = *var_temp0__blk79_dn0_slot;
        let mut var_temp0__blk79_dn1: f64 = *var_temp0__blk79_dn1_slot;
        let mut var_temp0__blk79_dn2: f64 = *var_temp0__blk79_dn2_slot;
        let mut var_temp0__blk79_dn3: f64 = *var_temp0__blk79_dn3_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn5: f64 = *var_temp0__blk79_dn5_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_db0: f64 = *var_temp1_db0_slot;
        let mut var_temp1_db1: f64 = *var_temp1_db1_slot;
        let mut var_temp1_db2: f64 = *var_temp1_db2_slot;
        let mut var_temp1_db3: f64 = *var_temp1_db3_slot;
        let mut var_temp1_dn0: f64 = *var_temp1_dn0_slot;
        let mut var_temp1_dn1: f64 = *var_temp1_dn1_slot;
        let mut var_temp1_dn2: f64 = *var_temp1_dn2_slot;
        let mut var_temp1_dn3: f64 = *var_temp1_dn3_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_db0: f64 = *var_temp2_db0_slot;
        let mut var_temp2_db1: f64 = *var_temp2_db1_slot;
        let mut var_temp2_db2: f64 = *var_temp2_db2_slot;
        let mut var_temp2_db3: f64 = *var_temp2_db3_slot;
        let mut var_temp2_dn0: f64 = *var_temp2_dn0_slot;
        let mut var_temp2_dn1: f64 = *var_temp2_dn1_slot;
        let mut var_temp2_dn2: f64 = *var_temp2_dn2_slot;
        let mut var_temp2_dn3: f64 = *var_temp2_dn3_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_db0: f64 = *var_temp3_db0_slot;
        let mut var_temp3_db1: f64 = *var_temp3_db1_slot;
        let mut var_temp3_db2: f64 = *var_temp3_db2_slot;
        let mut var_temp3_db3: f64 = *var_temp3_db3_slot;
        let mut var_temp3_dn0: f64 = *var_temp3_dn0_slot;
        let mut var_temp3_dn1: f64 = *var_temp3_dn1_slot;
        let mut var_temp3_dn2: f64 = *var_temp3_dn2_slot;
        let mut var_temp3_dn3: f64 = *var_temp3_dn3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn5: f64 = *var_temp3_dn5_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_db0: f64 = *var_temp4_db0_slot;
        let mut var_temp4_db1: f64 = *var_temp4_db1_slot;
        let mut var_temp4_db2: f64 = *var_temp4_db2_slot;
        let mut var_temp4_db3: f64 = *var_temp4_db3_slot;
        let mut var_temp4_dn0: f64 = *var_temp4_dn0_slot;
        let mut var_temp4_dn1: f64 = *var_temp4_dn1_slot;
        let mut var_temp4_dn2: f64 = *var_temp4_dn2_slot;
        let mut var_temp4_dn3: f64 = *var_temp4_dn3_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn5: f64 = *var_temp4_dn5_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign2520_e2150, assign2520_e2150_d_n0, assign2520_e2150_d_n1, assign2520_e2150_d_n2, assign2520_e2150_d_n3, assign2520_e2150_d_n4, assign2520_e2150_d_n5, assign2520_e2150_d_n6, assign2520_e2150_d_n7, assign2520_e2150_d_n8, assign2520_e2150_d_n9, assign2520_e2150_d_b0, assign2520_e2150_d_b1, assign2520_e2150_d_b2, assign2520_e2150_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2520_e2144: f64 = (p.p221 * var_ile);
        let assign2520_e2145: f64 = (1.0 + assign2520_e2144);
        let assign2520_e2146: f64 = (p.p220 * assign2520_e2145);
        let assign2520_e2148: f64 = (assign2520_e2146 * 1000000.0);
        (assign2520_e2148, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn0, var_temp0__blk79_dn1, var_temp0__blk79_dn2, var_temp0__blk79_dn3, var_temp0__blk79_dn4, var_temp0__blk79_dn5, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9, var_temp0__blk79_db0, var_temp0__blk79_db1, var_temp0__blk79_db2, var_temp0__blk79_db3,)
    }
};
        var_temp0__blk79 = assign2520_e2150;
        var_temp0__blk79_dn0 = assign2520_e2150_d_n0;
        var_temp0__blk79_dn1 = assign2520_e2150_d_n1;
        var_temp0__blk79_dn2 = assign2520_e2150_d_n2;
        var_temp0__blk79_dn3 = assign2520_e2150_d_n3;
        var_temp0__blk79_dn4 = assign2520_e2150_d_n4;
        var_temp0__blk79_dn5 = assign2520_e2150_d_n5;
        var_temp0__blk79_dn6 = assign2520_e2150_d_n6;
        var_temp0__blk79_dn7 = assign2520_e2150_d_n7;
        var_temp0__blk79_dn8 = assign2520_e2150_d_n8;
        var_temp0__blk79_dn9 = assign2520_e2150_d_n9;
        var_temp0__blk79_db0 = assign2520_e2150_d_b0;
        var_temp0__blk79_db1 = assign2520_e2150_d_b1;
        var_temp0__blk79_db2 = assign2520_e2150_d_b2;
        var_temp0__blk79_db3 = assign2520_e2150_d_b3;

        let (assign2530_e2159, assign2530_e2159_d_n0, assign2530_e2159_d_n1, assign2530_e2159_d_n2, assign2530_e2159_d_n3, assign2530_e2159_d_n4, assign2530_e2159_d_n5, assign2530_e2159_d_n6, assign2530_e2159_d_n7, assign2530_e2159_d_n8, assign2530_e2159_d_n9, assign2530_e2159_d_b0, assign2530_e2159_d_b1, assign2530_e2159_d_b2, assign2530_e2159_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2530_e2155: f64 = (var_temp0__blk79).max(1e25);
        let assign2530_e2157: f64 = (assign2530_e2155).min(1e28);
        (assign2530_e2157, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn0 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn1 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn2 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn3 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn4 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn5 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn6 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn7 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn8 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_dn9 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_db0 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_db1 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_db2 } else { 0.0 } } else { 0.0 }, if assign2530_e2155 <= 1e28 { if var_temp0__blk79 >= 1e25 { var_temp0__blk79_db3 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_np_i, var_np_i_dn0, var_np_i_dn1, var_np_i_dn2, var_np_i_dn3, var_np_i_dn4, var_np_i_dn5, var_np_i_dn6, var_np_i_dn7, var_np_i_dn8, var_np_i_dn9, var_np_i_db0, var_np_i_db1, var_np_i_db2, var_np_i_db3,)
    }
};
        var_np_i = assign2530_e2159;
        var_np_i_dn0 = assign2530_e2159_d_n0;
        var_np_i_dn1 = assign2530_e2159_d_n1;
        var_np_i_dn2 = assign2530_e2159_d_n2;
        var_np_i_dn3 = assign2530_e2159_d_n3;
        var_np_i_dn4 = assign2530_e2159_d_n4;
        var_np_i_dn5 = assign2530_e2159_d_n5;
        var_np_i_dn6 = assign2530_e2159_d_n6;
        var_np_i_dn7 = assign2530_e2159_d_n7;
        var_np_i_dn8 = assign2530_e2159_d_n8;
        var_np_i_dn9 = assign2530_e2159_d_n9;
        var_np_i_db0 = assign2530_e2159_d_b0;
        var_np_i_db1 = assign2530_e2159_d_b1;
        var_np_i_db2 = assign2530_e2159_d_b2;
        var_np_i_db3 = assign2530_e2159_d_b3;

        let (assign2540_e2164,) = {
    if (var_guard83 == 0.0) {
        (p.p222,)
    } else {
        (var_cic1_i,)
    }
};
        var_cic1_i = assign2540_e2164;

        let (assign2550_e2169,) = {
    if (var_guard83 == 0.0) {
        (p.p223,)
    } else {
        (var_cic2_i,)
    }
};
        var_cic2_i = assign2550_e2169;

        let (assign2560_e2176,) = {
    if (var_guard83 == 0.0) {
        let assign2560_e2174: f64 = (1.0 - var_xge_i);
        (assign2560_e2174,)
    } else {
        (var_one_m_xge,)
    }
};
        var_one_m_xge = assign2560_e2176;

        let (assign2570_e2187,) = {
    if (var_guard83 == 0.0) {
        let assign2570_e2181: f64 = (1.04479e-10 * var_one_m_xge);
        let assign2570_e2184: f64 = (1.43438e-10 * var_xge_i);
        let assign2570_e2185: f64 = (assign2570_e2181 + assign2570_e2184);
        (assign2570_e2185,)
    } else {
        (var_epsch,)
    }
};
        var_epsch = assign2570_e2187;

        let (assign2580_e2203,) = {
    if (var_guard83 == 0.0) {
        let assign2580_e2192: f64 = (var_epsch / 3.45313e-11);
        let assign2580_e2194: f64 = (assign2580_e2192 * var_tsi_i);
        let assign2580_e2197: f64 = (var_tox1_i + 4e-10);
        let assign2580_e2198: f64 = (assign2580_e2194 * assign2580_e2197);
        let assign2580_e2199: f64 = (assign2580_e2198).sqrt();
        let assign2580_e2201: f64 = (assign2580_e2199 / var_le);
        (assign2580_e2201,)
    } else {
        (var_lambda_le,)
    }
};
        var_lambda_le = assign2580_e2203;

        let (assign2590_e2220,) = {
    if (var_guard83 == 0.0) {
        let assign2590_e2208: f64 = (p.p224 * 2.0);
        let assign2590_e2211: f64 = (var_lambda_le).powf(p.p225);
        let assign2590_e2212: f64 = (assign2590_e2208 * assign2590_e2211);
        let assign2590_e2216: f64 = (p.p226 * var_iwe);
        let assign2590_e2217: f64 = (1.0 + assign2590_e2216);
        let assign2590_e2218: f64 = (assign2590_e2212 * assign2590_e2217);
        (assign2590_e2218,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign2590_e2220;

        let (assign2600_e2229,) = {
    if (var_guard83 == 0.0) {
        let assign2600_e2225: f64 = (var_psce_p).max(0.0);
        let assign2600_e2227: f64 = (assign2600_e2225).min(5.0);
        (assign2600_e2227,)
    } else {
        (var_psce1_i,)
    }
};
        var_psce1_i = assign2600_e2229;

        let (assign2610_e2240,) = {
    if (var_guard83 == 0.0) {
        let assign2610_e2234: f64 = (p.p227 * var_psce1_i);
        let assign2610_e2236: f64 = (assign2610_e2234 * var_tox2_i);
        let assign2610_e2238: f64 = (assign2610_e2236 / var_tox1_i);
        (assign2610_e2238,)
    } else {
        (var_psce2_i,)
    }
};
        var_psce2_i = assign2610_e2240;

        let (assign2620_e2247,) = {
    if (var_guard83 == 0.0) {
        let assign2620_e2245: f64 = (p.p228 * 1000000.0);
        (assign2620_e2245,)
    } else {
        (var_nsddc_i,)
    }
};
        var_nsddc_i = assign2620_e2247;

        let (assign2630_e2252,) = {
    if (var_guard83 == 0.0) {
        (p.p229,)
    } else {
        (var_pscedlb_i,)
    }
};
        var_pscedlb_i = assign2630_e2252;

        let (assign2640_e2259,) = {
    if (var_guard83 == 0.0) {
        let assign2640_e2257: f64 = (p.p230 * var_iwe);
        (assign2640_e2257,)
    } else {
        (var_pnce_p,)
    }
};
        var_pnce_p = assign2640_e2259;

        let (assign2650_e2269,) = {
    if (var_guard83 == 0.0) {
        let assign2650_e2264: f64 = (-1.0);
        let assign2650_e2265: f64 = (var_pnce_p).max(assign2650_e2264);
        let assign2650_e2267: f64 = (assign2650_e2265).min(1.0);
        (assign2650_e2267,)
    } else {
        (var_pnce_i,)
    }
};
        var_pnce_i = assign2650_e2269;

        let (assign2660_e2282, assign2660_e2282_d_n0, assign2660_e2282_d_n1, assign2660_e2282_d_n2, assign2660_e2282_d_n3, assign2660_e2282_d_n4, assign2660_e2282_d_n5, assign2660_e2282_d_n6, assign2660_e2282_d_n7, assign2660_e2282_d_n8, assign2660_e2282_d_n9, assign2660_e2282_d_b0, assign2660_e2282_d_b1, assign2660_e2282_d_b2, assign2660_e2282_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2660_e2274: f64 = (var_lambda_le).powf(p.p232);
        let assign2660_e2278: f64 = (p.p233 * var_iwe);
        let assign2660_e2279: f64 = (1.0 + assign2660_e2278);
        let assign2660_e2280: f64 = (assign2660_e2274 * assign2660_e2279);
        (assign2660_e2280, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign2660_e2282;
        var_temp_dn0 = assign2660_e2282_d_n0;
        var_temp_dn1 = assign2660_e2282_d_n1;
        var_temp_dn2 = assign2660_e2282_d_n2;
        var_temp_dn3 = assign2660_e2282_d_n3;
        var_temp_dn4 = assign2660_e2282_d_n4;
        var_temp_dn5 = assign2660_e2282_d_n5;
        var_temp_dn6 = assign2660_e2282_d_n6;
        var_temp_dn7 = assign2660_e2282_d_n7;
        var_temp_dn8 = assign2660_e2282_d_n8;
        var_temp_dn9 = assign2660_e2282_d_n9;
        var_temp_db0 = assign2660_e2282_d_b0;
        var_temp_db1 = assign2660_e2282_d_b1;
        var_temp_db2 = assign2660_e2282_d_b2;
        var_temp_db3 = assign2660_e2282_d_b3;

        let (assign2670_e2289, assign2670_e2289_d_n0, assign2670_e2289_d_n1, assign2670_e2289_d_n2, assign2670_e2289_d_n3, assign2670_e2289_d_n4, assign2670_e2289_d_n5, assign2670_e2289_d_n6, assign2670_e2289_d_n7, assign2670_e2289_d_n8, assign2670_e2289_d_n9, assign2670_e2289_d_b0, assign2670_e2289_d_b1, assign2670_e2289_d_b2, assign2670_e2289_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2670_e2287: f64 = (p.p231 * var_temp);
        (assign2670_e2287, (p.p231 * var_temp_dn0), (p.p231 * var_temp_dn1), (p.p231 * var_temp_dn2), (p.p231 * var_temp_dn3), (p.p231 * var_temp_dn4), (p.p231 * var_temp_dn5), (p.p231 * var_temp_dn6), (p.p231 * var_temp_dn7), (p.p231 * var_temp_dn8), (p.p231 * var_temp_dn9), (p.p231 * var_temp_db0), (p.p231 * var_temp_db1), (p.p231 * var_temp_db2), (p.p231 * var_temp_db3),)
    } else {
        (var_cf_p, var_cf_p_dn0, var_cf_p_dn1, var_cf_p_dn2, var_cf_p_dn3, var_cf_p_dn4, var_cf_p_dn5, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9, var_cf_p_db0, var_cf_p_db1, var_cf_p_db2, var_cf_p_db3,)
    }
};
        var_cf_p = assign2670_e2289;
        var_cf_p_dn0 = assign2670_e2289_d_n0;
        var_cf_p_dn1 = assign2670_e2289_d_n1;
        var_cf_p_dn2 = assign2670_e2289_d_n2;
        var_cf_p_dn3 = assign2670_e2289_d_n3;
        var_cf_p_dn4 = assign2670_e2289_d_n4;
        var_cf_p_dn5 = assign2670_e2289_d_n5;
        var_cf_p_dn6 = assign2670_e2289_d_n6;
        var_cf_p_dn7 = assign2670_e2289_d_n7;
        var_cf_p_dn8 = assign2670_e2289_d_n8;
        var_cf_p_dn9 = assign2670_e2289_d_n9;
        var_cf_p_db0 = assign2670_e2289_d_b0;
        var_cf_p_db1 = assign2670_e2289_d_b1;
        var_cf_p_db2 = assign2670_e2289_d_b2;
        var_cf_p_db3 = assign2670_e2289_d_b3;

        let (assign2680_e2296, assign2680_e2296_d_n0, assign2680_e2296_d_n1, assign2680_e2296_d_n2, assign2680_e2296_d_n3, assign2680_e2296_d_n4, assign2680_e2296_d_n5, assign2680_e2296_d_n6, assign2680_e2296_d_n7, assign2680_e2296_d_n8, assign2680_e2296_d_n9, assign2680_e2296_d_b0, assign2680_e2296_d_b1, assign2680_e2296_d_b2, assign2680_e2296_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2680_e2294: f64 = (var_cf_p).max(0.0);
        (assign2680_e2294, if var_cf_p >= 0.0 { var_cf_p_dn0 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn1 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn2 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn3 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn5 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db0 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db1 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db2 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db3 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn0, var_cf1_t_dn1, var_cf1_t_dn2, var_cf1_t_dn3, var_cf1_t_dn4, var_cf1_t_dn5, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9, var_cf1_t_db0, var_cf1_t_db1, var_cf1_t_db2, var_cf1_t_db3,)
    }
};
        var_cf1_t = assign2680_e2296;
        var_cf1_t_dn0 = assign2680_e2296_d_n0;
        var_cf1_t_dn1 = assign2680_e2296_d_n1;
        var_cf1_t_dn2 = assign2680_e2296_d_n2;
        var_cf1_t_dn3 = assign2680_e2296_d_n3;
        var_cf1_t_dn4 = assign2680_e2296_d_n4;
        var_cf1_t_dn5 = assign2680_e2296_d_n5;
        var_cf1_t_dn6 = assign2680_e2296_d_n6;
        var_cf1_t_dn7 = assign2680_e2296_d_n7;
        var_cf1_t_dn8 = assign2680_e2296_d_n8;
        var_cf1_t_dn9 = assign2680_e2296_d_n9;
        var_cf1_t_db0 = assign2680_e2296_d_b0;
        var_cf1_t_db1 = assign2680_e2296_d_b1;
        var_cf1_t_db2 = assign2680_e2296_d_b2;
        var_cf1_t_db3 = assign2680_e2296_d_b3;

        let (assign2690_e2307, assign2690_e2307_d_n0, assign2690_e2307_d_n1, assign2690_e2307_d_n2, assign2690_e2307_d_n3, assign2690_e2307_d_n4, assign2690_e2307_d_n5, assign2690_e2307_d_n6, assign2690_e2307_d_n7, assign2690_e2307_d_n8, assign2690_e2307_d_n9, assign2690_e2307_d_b0, assign2690_e2307_d_b1, assign2690_e2307_d_b2, assign2690_e2307_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2690_e2301: f64 = (p.p234 * var_cf1_t);
        let assign2690_e2303: f64 = (assign2690_e2301 * var_tox2_i);
        let assign2690_e2305: f64 = (assign2690_e2303 / var_tox1_i);
        (assign2690_e2305, (((p.p234 * var_cf1_t_dn0) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn1) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn2) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn3) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn5) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_dn9) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_db0) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_db1) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_db2) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cf1_t_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2_t, var_cf2_t_dn0, var_cf2_t_dn1, var_cf2_t_dn2, var_cf2_t_dn3, var_cf2_t_dn4, var_cf2_t_dn5, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9, var_cf2_t_db0, var_cf2_t_db1, var_cf2_t_db2, var_cf2_t_db3,)
    }
};
        var_cf2_t = assign2690_e2307;
        var_cf2_t_dn0 = assign2690_e2307_d_n0;
        var_cf2_t_dn1 = assign2690_e2307_d_n1;
        var_cf2_t_dn2 = assign2690_e2307_d_n2;
        var_cf2_t_dn3 = assign2690_e2307_d_n3;
        var_cf2_t_dn4 = assign2690_e2307_d_n4;
        var_cf2_t_dn5 = assign2690_e2307_d_n5;
        var_cf2_t_dn6 = assign2690_e2307_d_n6;
        var_cf2_t_dn7 = assign2690_e2307_d_n7;
        var_cf2_t_dn8 = assign2690_e2307_d_n8;
        var_cf2_t_dn9 = assign2690_e2307_d_n9;
        var_cf2_t_db0 = assign2690_e2307_d_b0;
        var_cf2_t_db1 = assign2690_e2307_d_b1;
        var_cf2_t_db2 = assign2690_e2307_d_b2;
        var_cf2_t_db3 = assign2690_e2307_d_b3;

        let (assign2700_e2314, assign2700_e2314_d_n0, assign2700_e2314_d_n1, assign2700_e2314_d_n2, assign2700_e2314_d_n3, assign2700_e2314_d_n4, assign2700_e2314_d_n5, assign2700_e2314_d_n6, assign2700_e2314_d_n7, assign2700_e2314_d_n8, assign2700_e2314_d_n9, assign2700_e2314_d_b0, assign2700_e2314_d_b1, assign2700_e2314_d_b2, assign2700_e2314_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2700_e2312: f64 = (p.p235 * var_temp);
        (assign2700_e2312, (p.p235 * var_temp_dn0), (p.p235 * var_temp_dn1), (p.p235 * var_temp_dn2), (p.p235 * var_temp_dn3), (p.p235 * var_temp_dn4), (p.p235 * var_temp_dn5), (p.p235 * var_temp_dn6), (p.p235 * var_temp_dn7), (p.p235 * var_temp_dn8), (p.p235 * var_temp_dn9), (p.p235 * var_temp_db0), (p.p235 * var_temp_db1), (p.p235 * var_temp_db2), (p.p235 * var_temp_db3),)
    } else {
        (var_stcf_i, var_stcf_i_dn0, var_stcf_i_dn1, var_stcf_i_dn2, var_stcf_i_dn3, var_stcf_i_dn4, var_stcf_i_dn5, var_stcf_i_dn6, var_stcf_i_dn7, var_stcf_i_dn8, var_stcf_i_dn9, var_stcf_i_db0, var_stcf_i_db1, var_stcf_i_db2, var_stcf_i_db3,)
    }
};
        var_stcf_i = assign2700_e2314;
        var_stcf_i_dn0 = assign2700_e2314_d_n0;
        var_stcf_i_dn1 = assign2700_e2314_d_n1;
        var_stcf_i_dn2 = assign2700_e2314_d_n2;
        var_stcf_i_dn3 = assign2700_e2314_d_n3;
        var_stcf_i_dn4 = assign2700_e2314_d_n4;
        var_stcf_i_dn5 = assign2700_e2314_d_n5;
        var_stcf_i_dn6 = assign2700_e2314_d_n6;
        var_stcf_i_dn7 = assign2700_e2314_d_n7;
        var_stcf_i_dn8 = assign2700_e2314_d_n8;
        var_stcf_i_dn9 = assign2700_e2314_d_n9;
        var_stcf_i_db0 = assign2700_e2314_d_b0;
        var_stcf_i_db1 = assign2700_e2314_d_b1;
        var_stcf_i_db2 = assign2700_e2314_d_b2;
        var_stcf_i_db3 = assign2700_e2314_d_b3;

        let (assign2710_e2319,) = {
    if (var_guard83 == 0.0) {
        (p.p236,)
    } else {
        (var_cfd_i,)
    }
};
        var_cfd_i = assign2710_e2319;

        let (assign2720_e2334,) = {
    if (var_guard83 == 0.0) {
        let assign2720_e2324: f64 = (p.p237 * var_ile);
        let assign2720_e2328: f64 = (p.p238 * var_iwe);
        let assign2720_e2329: f64 = (1.0 + assign2720_e2328);
        let assign2720_e2331: f64 = (assign2720_e2329).max(0.001);
        let assign2720_e2332: f64 = (assign2720_e2324 / assign2720_e2331);
        (assign2720_e2332,)
    } else {
        (var_cfdl_i,)
    }
};
        var_cfdl_i = assign2720_e2334;

        let (assign2730_e2339,) = {
    if (var_guard83 == 0.0) {
        (p.p239,)
    } else {
        (var_cfdlb_i,)
    }
};
        var_cfdlb_i = assign2730_e2339;

        let (assign2740_e2355, assign2740_e2355_d_n0, assign2740_e2355_d_n1, assign2740_e2355_d_n2, assign2740_e2355_d_n3, assign2740_e2355_d_n4, assign2740_e2355_d_n5, assign2740_e2355_d_n6, assign2740_e2355_d_n7, assign2740_e2355_d_n8, assign2740_e2355_d_n9, assign2740_e2355_d_b0, assign2740_e2355_d_b1, assign2740_e2355_d_b2, assign2740_e2355_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2740_e2343: f64 = (-var_le);
        let assign2740_e2348: f64 = (p.p244 * var_iwe);
        let assign2740_e2349: f64 = (1.0 + assign2740_e2348);
        let assign2740_e2351: f64 = (assign2740_e2349).max(0.001);
        let assign2740_e2352: f64 = (p.p243 * assign2740_e2351);
        let assign2740_e2353: f64 = (assign2740_e2343 / assign2740_e2352);
        (assign2740_e2353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3,)
    }
};
        var_temp1 = assign2740_e2355;
        var_temp1_dn0 = assign2740_e2355_d_n0;
        var_temp1_dn1 = assign2740_e2355_d_n1;
        var_temp1_dn2 = assign2740_e2355_d_n2;
        var_temp1_dn3 = assign2740_e2355_d_n3;
        var_temp1_dn4 = assign2740_e2355_d_n4;
        var_temp1_dn5 = assign2740_e2355_d_n5;
        var_temp1_dn6 = assign2740_e2355_d_n6;
        var_temp1_dn7 = assign2740_e2355_d_n7;
        var_temp1_dn8 = assign2740_e2355_d_n8;
        var_temp1_dn9 = assign2740_e2355_d_n9;
        var_temp1_db0 = assign2740_e2355_d_b0;
        var_temp1_db1 = assign2740_e2355_d_b1;
        var_temp1_db2 = assign2740_e2355_d_b2;
        var_temp1_db3 = assign2740_e2355_d_b3;

        let assign2750_e2358: f64 = (-80.0);
        let assign2750_e2359: f64 = if var_temp1 > assign2750_e2358 { 1.0 } else { 0.0 };
        var_guard96 = assign2750_e2359;

        let (assign2760_e2367, assign2760_e2367_d_n0, assign2760_e2367_d_n1, assign2760_e2367_d_n2, assign2760_e2367_d_n3, assign2760_e2367_d_n4, assign2760_e2367_d_n5, assign2760_e2367_d_n6, assign2760_e2367_d_n7, assign2760_e2367_d_n8, assign2760_e2367_d_n9, assign2760_e2367_d_b0, assign2760_e2367_d_b1, assign2760_e2367_d_b2, assign2760_e2367_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard96 != 0.0)) {
        let assign2760_e2365: f64 = (var_temp1).exp();
        (assign2760_e2365, (assign2760_e2365 * var_temp1_dn0), (assign2760_e2365 * var_temp1_dn1), (assign2760_e2365 * var_temp1_dn2), (assign2760_e2365 * var_temp1_dn3), (assign2760_e2365 * var_temp1_dn4), (assign2760_e2365 * var_temp1_dn5), (assign2760_e2365 * var_temp1_dn6), (assign2760_e2365 * var_temp1_dn7), (assign2760_e2365 * var_temp1_dn8), (assign2760_e2365 * var_temp1_dn9), (assign2760_e2365 * var_temp1_db0), (assign2760_e2365 * var_temp1_db1), (assign2760_e2365 * var_temp1_db2), (assign2760_e2365 * var_temp1_db3),)
    } else {
        (var_temp2, var_temp2_dn0, var_temp2_dn1, var_temp2_dn2, var_temp2_dn3, var_temp2_dn4, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9, var_temp2_db0, var_temp2_db1, var_temp2_db2, var_temp2_db3,)
    }
};
        var_temp2 = assign2760_e2367;
        var_temp2_dn0 = assign2760_e2367_d_n0;
        var_temp2_dn1 = assign2760_e2367_d_n1;
        var_temp2_dn2 = assign2760_e2367_d_n2;
        var_temp2_dn3 = assign2760_e2367_d_n3;
        var_temp2_dn4 = assign2760_e2367_d_n4;
        var_temp2_dn5 = assign2760_e2367_d_n5;
        var_temp2_dn6 = assign2760_e2367_d_n6;
        var_temp2_dn7 = assign2760_e2367_d_n7;
        var_temp2_dn8 = assign2760_e2367_d_n8;
        var_temp2_dn9 = assign2760_e2367_d_n9;
        var_temp2_db0 = assign2760_e2367_d_b0;
        var_temp2_db1 = assign2760_e2367_d_b1;
        var_temp2_db2 = assign2760_e2367_d_b2;
        var_temp2_db3 = assign2760_e2367_d_b3;

        let (assign2770_e2400, assign2770_e2400_d_n0, assign2770_e2400_d_n1, assign2770_e2400_d_n2, assign2770_e2400_d_n3, assign2770_e2400_d_n4, assign2770_e2400_d_n5, assign2770_e2400_d_n6, assign2770_e2400_d_n7, assign2770_e2400_d_n8, assign2770_e2400_d_n9, assign2770_e2400_d_b0, assign2770_e2400_d_b1, assign2770_e2400_d_b2, assign2770_e2400_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard96 == 0.0)) {
        let assign2770_e2376: f64 = (-var_temp1);
        let assign2770_e2378: f64 = (assign2770_e2376 - 80.0);
        let assign2770_e2382: f64 = (-var_temp1);
        let assign2770_e2384: f64 = (assign2770_e2382 - 80.0);
        let assign2770_e2385: f64 = (0.5 * assign2770_e2384);
        let assign2770_e2388: f64 = (-var_temp1);
        let assign2770_e2390: f64 = (assign2770_e2388 - 80.0);
        let assign2770_e2392: f64 = (assign2770_e2390 * 0.3333333333333);
        let assign2770_e2393: f64 = (1.0 + assign2770_e2392);
        let assign2770_e2394: f64 = (assign2770_e2385 * assign2770_e2393);
        let assign2770_e2395: f64 = (1.0 + assign2770_e2394);
        let assign2770_e2396: f64 = (assign2770_e2378 * assign2770_e2395);
        let assign2770_e2397: f64 = (1.0 + assign2770_e2396);
        let assign2770_e2398: f64 = (1.80485e-35 / assign2770_e2397);
        (assign2770_e2398, (-((1.80485e-35 * (((-var_temp1_dn0) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn0)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn0) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn1) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn1)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn1) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn2) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn2)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn2) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn3) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn3)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn3) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn4) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn4)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn4) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn5) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn5)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn5) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn6) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn6)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn6) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn7) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn7)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn7) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn8) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn8)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn8) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_dn9) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_dn9)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_dn9) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_db0) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_db0)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_db0) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_db1) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_db1)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_db1) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_db2) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_db2)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_db2) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))), (-((1.80485e-35 * (((-var_temp1_db3) * assign2770_e2395) + (assign2770_e2378 * (((0.5 * (-var_temp1_db3)) * assign2770_e2393) + (assign2770_e2385 * ((-var_temp1_db3) * 0.3333333333333)))))) / (assign2770_e2397 * assign2770_e2397))),)
    } else {
        (var_temp2, var_temp2_dn0, var_temp2_dn1, var_temp2_dn2, var_temp2_dn3, var_temp2_dn4, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9, var_temp2_db0, var_temp2_db1, var_temp2_db2, var_temp2_db3,)
    }
};
        var_temp2 = assign2770_e2400;
        var_temp2_dn0 = assign2770_e2400_d_n0;
        var_temp2_dn1 = assign2770_e2400_d_n1;
        var_temp2_dn2 = assign2770_e2400_d_n2;
        var_temp2_dn3 = assign2770_e2400_d_n3;
        var_temp2_dn4 = assign2770_e2400_d_n4;
        var_temp2_dn5 = assign2770_e2400_d_n5;
        var_temp2_dn6 = assign2770_e2400_d_n6;
        var_temp2_dn7 = assign2770_e2400_d_n7;
        var_temp2_dn8 = assign2770_e2400_d_n8;
        var_temp2_dn9 = assign2770_e2400_d_n9;
        var_temp2_db0 = assign2770_e2400_d_b0;
        var_temp2_db1 = assign2770_e2400_d_b1;
        var_temp2_db2 = assign2770_e2400_d_b2;
        var_temp2_db3 = assign2770_e2400_d_b3;

        let (assign2780_e2408, assign2780_e2408_d_n0, assign2780_e2408_d_n1, assign2780_e2408_d_n2, assign2780_e2408_d_n3, assign2780_e2408_d_n4, assign2780_e2408_d_n5, assign2780_e2408_d_n6, assign2780_e2408_d_n7, assign2780_e2408_d_n8, assign2780_e2408_d_n9, assign2780_e2408_d_b0, assign2780_e2408_d_b1, assign2780_e2408_d_b2, assign2780_e2408_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2780_e2404: f64 = (-var_le);
        let assign2780_e2406: f64 = (assign2780_e2404 / p.p246);
        (assign2780_e2406, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp3, var_temp3_dn0, var_temp3_dn1, var_temp3_dn2, var_temp3_dn3, var_temp3_dn4, var_temp3_dn5, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9, var_temp3_db0, var_temp3_db1, var_temp3_db2, var_temp3_db3,)
    }
};
        var_temp3 = assign2780_e2408;
        var_temp3_dn0 = assign2780_e2408_d_n0;
        var_temp3_dn1 = assign2780_e2408_d_n1;
        var_temp3_dn2 = assign2780_e2408_d_n2;
        var_temp3_dn3 = assign2780_e2408_d_n3;
        var_temp3_dn4 = assign2780_e2408_d_n4;
        var_temp3_dn5 = assign2780_e2408_d_n5;
        var_temp3_dn6 = assign2780_e2408_d_n6;
        var_temp3_dn7 = assign2780_e2408_d_n7;
        var_temp3_dn8 = assign2780_e2408_d_n8;
        var_temp3_dn9 = assign2780_e2408_d_n9;
        var_temp3_db0 = assign2780_e2408_d_b0;
        var_temp3_db1 = assign2780_e2408_d_b1;
        var_temp3_db2 = assign2780_e2408_d_b2;
        var_temp3_db3 = assign2780_e2408_d_b3;

        let assign2790_e2411: f64 = (-80.0);
        let assign2790_e2412: f64 = if var_temp3 > assign2790_e2411 { 1.0 } else { 0.0 };
        var_guard97 = assign2790_e2412;

        let (assign2800_e2420, assign2800_e2420_d_n0, assign2800_e2420_d_n1, assign2800_e2420_d_n2, assign2800_e2420_d_n3, assign2800_e2420_d_n4, assign2800_e2420_d_n5, assign2800_e2420_d_n6, assign2800_e2420_d_n7, assign2800_e2420_d_n8, assign2800_e2420_d_n9, assign2800_e2420_d_b0, assign2800_e2420_d_b1, assign2800_e2420_d_b2, assign2800_e2420_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard97 != 0.0)) {
        let assign2800_e2418: f64 = (var_temp3).exp();
        (assign2800_e2418, (assign2800_e2418 * var_temp3_dn0), (assign2800_e2418 * var_temp3_dn1), (assign2800_e2418 * var_temp3_dn2), (assign2800_e2418 * var_temp3_dn3), (assign2800_e2418 * var_temp3_dn4), (assign2800_e2418 * var_temp3_dn5), (assign2800_e2418 * var_temp3_dn6), (assign2800_e2418 * var_temp3_dn7), (assign2800_e2418 * var_temp3_dn8), (assign2800_e2418 * var_temp3_dn9), (assign2800_e2418 * var_temp3_db0), (assign2800_e2418 * var_temp3_db1), (assign2800_e2418 * var_temp3_db2), (assign2800_e2418 * var_temp3_db3),)
    } else {
        (var_temp4, var_temp4_dn0, var_temp4_dn1, var_temp4_dn2, var_temp4_dn3, var_temp4_dn4, var_temp4_dn5, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9, var_temp4_db0, var_temp4_db1, var_temp4_db2, var_temp4_db3,)
    }
};
        var_temp4 = assign2800_e2420;
        var_temp4_dn0 = assign2800_e2420_d_n0;
        var_temp4_dn1 = assign2800_e2420_d_n1;
        var_temp4_dn2 = assign2800_e2420_d_n2;
        var_temp4_dn3 = assign2800_e2420_d_n3;
        var_temp4_dn4 = assign2800_e2420_d_n4;
        var_temp4_dn5 = assign2800_e2420_d_n5;
        var_temp4_dn6 = assign2800_e2420_d_n6;
        var_temp4_dn7 = assign2800_e2420_d_n7;
        var_temp4_dn8 = assign2800_e2420_d_n8;
        var_temp4_dn9 = assign2800_e2420_d_n9;
        var_temp4_db0 = assign2800_e2420_d_b0;
        var_temp4_db1 = assign2800_e2420_d_b1;
        var_temp4_db2 = assign2800_e2420_d_b2;
        var_temp4_db3 = assign2800_e2420_d_b3;

        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_db0_slot = var_cf1_t_db0;
        *var_cf1_t_db1_slot = var_cf1_t_db1;
        *var_cf1_t_db2_slot = var_cf1_t_db2;
        *var_cf1_t_db3_slot = var_cf1_t_db3;
        *var_cf1_t_dn0_slot = var_cf1_t_dn0;
        *var_cf1_t_dn1_slot = var_cf1_t_dn1;
        *var_cf1_t_dn2_slot = var_cf1_t_dn2;
        *var_cf1_t_dn3_slot = var_cf1_t_dn3;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn5_slot = var_cf1_t_dn5;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_db0_slot = var_cf2_t_db0;
        *var_cf2_t_db1_slot = var_cf2_t_db1;
        *var_cf2_t_db2_slot = var_cf2_t_db2;
        *var_cf2_t_db3_slot = var_cf2_t_db3;
        *var_cf2_t_dn0_slot = var_cf2_t_dn0;
        *var_cf2_t_dn1_slot = var_cf2_t_dn1;
        *var_cf2_t_dn2_slot = var_cf2_t_dn2;
        *var_cf2_t_dn3_slot = var_cf2_t_dn3;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn5_slot = var_cf2_t_dn5;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_db0_slot = var_cf_p_db0;
        *var_cf_p_db1_slot = var_cf_p_db1;
        *var_cf_p_db2_slot = var_cf_p_db2;
        *var_cf_p_db3_slot = var_cf_p_db3;
        *var_cf_p_dn0_slot = var_cf_p_dn0;
        *var_cf_p_dn1_slot = var_cf_p_dn1;
        *var_cf_p_dn2_slot = var_cf_p_dn2;
        *var_cf_p_dn3_slot = var_cf_p_dn3;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn5_slot = var_cf_p_dn5;
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
        *var_guard96_slot = var_guard96;
        *var_guard97_slot = var_guard97;
        *var_lambda_le_slot = var_lambda_le;
        *var_np_i_slot = var_np_i;
        *var_np_i_db0_slot = var_np_i_db0;
        *var_np_i_db1_slot = var_np_i_db1;
        *var_np_i_db2_slot = var_np_i_db2;
        *var_np_i_db3_slot = var_np_i_db3;
        *var_np_i_dn0_slot = var_np_i_dn0;
        *var_np_i_dn1_slot = var_np_i_dn1;
        *var_np_i_dn2_slot = var_np_i_dn2;
        *var_np_i_dn3_slot = var_np_i_dn3;
        *var_np_i_dn4_slot = var_np_i_dn4;
        *var_np_i_dn5_slot = var_np_i_dn5;
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
        *var_stcf_i_db0_slot = var_stcf_i_db0;
        *var_stcf_i_db1_slot = var_stcf_i_db1;
        *var_stcf_i_db2_slot = var_stcf_i_db2;
        *var_stcf_i_db3_slot = var_stcf_i_db3;
        *var_stcf_i_dn0_slot = var_stcf_i_dn0;
        *var_stcf_i_dn1_slot = var_stcf_i_dn1;
        *var_stcf_i_dn2_slot = var_stcf_i_dn2;
        *var_stcf_i_dn3_slot = var_stcf_i_dn3;
        *var_stcf_i_dn4_slot = var_stcf_i_dn4;
        *var_stcf_i_dn5_slot = var_stcf_i_dn5;
        *var_stcf_i_dn6_slot = var_stcf_i_dn6;
        *var_stcf_i_dn7_slot = var_stcf_i_dn7;
        *var_stcf_i_dn8_slot = var_stcf_i_dn8;
        *var_stcf_i_dn9_slot = var_stcf_i_dn9;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_db0_slot = var_temp0__blk79_db0;
        *var_temp0__blk79_db1_slot = var_temp0__blk79_db1;
        *var_temp0__blk79_db2_slot = var_temp0__blk79_db2;
        *var_temp0__blk79_db3_slot = var_temp0__blk79_db3;
        *var_temp0__blk79_dn0_slot = var_temp0__blk79_dn0;
        *var_temp0__blk79_dn1_slot = var_temp0__blk79_dn1;
        *var_temp0__blk79_dn2_slot = var_temp0__blk79_dn2;
        *var_temp0__blk79_dn3_slot = var_temp0__blk79_dn3;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn5_slot = var_temp0__blk79_dn5;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp1_slot = var_temp1;
        *var_temp1_db0_slot = var_temp1_db0;
        *var_temp1_db1_slot = var_temp1_db1;
        *var_temp1_db2_slot = var_temp1_db2;
        *var_temp1_db3_slot = var_temp1_db3;
        *var_temp1_dn0_slot = var_temp1_dn0;
        *var_temp1_dn1_slot = var_temp1_dn1;
        *var_temp1_dn2_slot = var_temp1_dn2;
        *var_temp1_dn3_slot = var_temp1_dn3;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_db0_slot = var_temp2_db0;
        *var_temp2_db1_slot = var_temp2_db1;
        *var_temp2_db2_slot = var_temp2_db2;
        *var_temp2_db3_slot = var_temp2_db3;
        *var_temp2_dn0_slot = var_temp2_dn0;
        *var_temp2_dn1_slot = var_temp2_dn1;
        *var_temp2_dn2_slot = var_temp2_dn2;
        *var_temp2_dn3_slot = var_temp2_dn3;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_db0_slot = var_temp3_db0;
        *var_temp3_db1_slot = var_temp3_db1;
        *var_temp3_db2_slot = var_temp3_db2;
        *var_temp3_db3_slot = var_temp3_db3;
        *var_temp3_dn0_slot = var_temp3_dn0;
        *var_temp3_dn1_slot = var_temp3_dn1;
        *var_temp3_dn2_slot = var_temp3_dn2;
        *var_temp3_dn3_slot = var_temp3_dn3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn5_slot = var_temp3_dn5;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_db0_slot = var_temp4_db0;
        *var_temp4_db1_slot = var_temp4_db1;
        *var_temp4_db2_slot = var_temp4_db2;
        *var_temp4_db3_slot = var_temp4_db3;
        *var_temp4_dn0_slot = var_temp4_dn0;
        *var_temp4_dn1_slot = var_temp4_dn1;
        *var_temp4_dn2_slot = var_temp4_dn2;
        *var_temp4_dn3_slot = var_temp4_dn3;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn5_slot = var_temp4_dn5;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_7(
        p: &Parameters,
        var_guard83: f64,
        var_guard97: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_temp1: f64,
        var_temp1_db0: f64,
        var_temp1_db1: f64,
        var_temp1_db2: f64,
        var_temp1_db3: f64,
        var_temp1_dn0: f64,
        var_temp1_dn1: f64,
        var_temp1_dn2: f64,
        var_temp1_dn3: f64,
        var_temp1_dn4: f64,
        var_temp1_dn5: f64,
        var_temp1_dn6: f64,
        var_temp1_dn7: f64,
        var_temp1_dn8: f64,
        var_temp1_dn9: f64,
        var_temp2: f64,
        var_temp2_db0: f64,
        var_temp2_db1: f64,
        var_temp2_db2: f64,
        var_temp2_db3: f64,
        var_temp2_dn0: f64,
        var_temp2_dn1: f64,
        var_temp2_dn2: f64,
        var_temp2_dn3: f64,
        var_temp2_dn4: f64,
        var_temp2_dn5: f64,
        var_temp2_dn6: f64,
        var_temp2_dn7: f64,
        var_temp2_dn8: f64,
        var_temp2_dn9: f64,
        var_temp3: f64,
        var_temp3_db0: f64,
        var_temp3_db1: f64,
        var_temp3_db2: f64,
        var_temp3_db3: f64,
        var_temp3_dn0: f64,
        var_temp3_dn1: f64,
        var_temp3_dn2: f64,
        var_temp3_dn3: f64,
        var_temp3_dn4: f64,
        var_temp3_dn5: f64,
        var_temp3_dn6: f64,
        var_temp3_dn7: f64,
        var_temp3_dn8: f64,
        var_temp3_dn9: f64,
        var_we: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_db0_slot: &mut f64,
        var_betn1_t_db1_slot: &mut f64,
        var_betn1_t_db2_slot: &mut f64,
        var_betn1_t_db3_slot: &mut f64,
        var_betn1_t_dn0_slot: &mut f64,
        var_betn1_t_dn1_slot: &mut f64,
        var_betn1_t_dn2_slot: &mut f64,
        var_betn1_t_dn3_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn5_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_db0_slot: &mut f64,
        var_betn2_t_db1_slot: &mut f64,
        var_betn2_t_db2_slot: &mut f64,
        var_betn2_t_db3_slot: &mut f64,
        var_betn2_t_dn0_slot: &mut f64,
        var_betn2_t_dn1_slot: &mut f64,
        var_betn2_t_dn2_slot: &mut f64,
        var_betn2_t_dn3_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn5_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_db0_slot: &mut f64,
        var_betn_p_db1_slot: &mut f64,
        var_betn_p_db2_slot: &mut f64,
        var_betn_p_db3_slot: &mut f64,
        var_betn_p_dn0_slot: &mut f64,
        var_betn_p_dn1_slot: &mut f64,
        var_betn_p_dn2_slot: &mut f64,
        var_betn_p_dn3_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn5_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_csbi_i_slot: &mut f64,
        var_csfi_i_slot: &mut f64,
        var_csthr_i_slot: &mut f64,
        var_csthrb_i_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_ge_slot: &mut f64,
        var_ge_db0_slot: &mut f64,
        var_ge_db1_slot: &mut f64,
        var_ge_db2_slot: &mut f64,
        var_ge_db3_slot: &mut f64,
        var_ge_dn0_slot: &mut f64,
        var_ge_dn1_slot: &mut f64,
        var_ge_dn2_slot: &mut f64,
        var_ge_dn3_slot: &mut f64,
        var_ge_dn4_slot: &mut f64,
        var_ge_dn5_slot: &mut f64,
        var_ge_dn6_slot: &mut f64,
        var_ge_dn7_slot: &mut f64,
        var_ge_dn8_slot: &mut f64,
        var_ge_dn9_slot: &mut f64,
        var_gpe_slot: &mut f64,
        var_gpe_db0_slot: &mut f64,
        var_gpe_db1_slot: &mut f64,
        var_gpe_db2_slot: &mut f64,
        var_gpe_db3_slot: &mut f64,
        var_gpe_dn0_slot: &mut f64,
        var_gpe_dn1_slot: &mut f64,
        var_gpe_dn2_slot: &mut f64,
        var_gpe_dn3_slot: &mut f64,
        var_gpe_dn4_slot: &mut f64,
        var_gpe_dn5_slot: &mut f64,
        var_gpe_dn6_slot: &mut f64,
        var_gpe_dn7_slot: &mut f64,
        var_gpe_dn8_slot: &mut f64,
        var_gpe_dn9_slot: &mut f64,
        var_gwe_slot: &mut f64,
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
        var_stxcor_i_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_db0_slot: &mut f64,
        var_temp4_db1_slot: &mut f64,
        var_temp4_db2_slot: &mut f64,
        var_temp4_db3_slot: &mut f64,
        var_temp4_dn0_slot: &mut f64,
        var_temp4_dn1_slot: &mut f64,
        var_temp4_dn2_slot: &mut f64,
        var_temp4_dn3_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn5_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_thersg_i_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_db0_slot: &mut f64,
        var_thesat_p_db1_slot: &mut f64,
        var_thesat_p_db2_slot: &mut f64,
        var_thesat_p_db3_slot: &mut f64,
        var_thesat_p_dn0_slot: &mut f64,
        var_thesat_p_dn1_slot: &mut f64,
        var_thesat_p_dn2_slot: &mut f64,
        var_thesat_p_dn3_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn5_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_db0_slot: &mut f64,
        var_thesat_t_db1_slot: &mut f64,
        var_thesat_t_db2_slot: &mut f64,
        var_thesat_t_db3_slot: &mut f64,
        var_thesat_t_dn0_slot: &mut f64,
        var_thesat_t_dn1_slot: &mut f64,
        var_thesat_t_dn2_slot: &mut f64,
        var_thesat_t_dn3_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn5_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcorb_i_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_db0: f64 = *var_betn1_t_db0_slot;
        let mut var_betn1_t_db1: f64 = *var_betn1_t_db1_slot;
        let mut var_betn1_t_db2: f64 = *var_betn1_t_db2_slot;
        let mut var_betn1_t_db3: f64 = *var_betn1_t_db3_slot;
        let mut var_betn1_t_dn0: f64 = *var_betn1_t_dn0_slot;
        let mut var_betn1_t_dn1: f64 = *var_betn1_t_dn1_slot;
        let mut var_betn1_t_dn2: f64 = *var_betn1_t_dn2_slot;
        let mut var_betn1_t_dn3: f64 = *var_betn1_t_dn3_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn5: f64 = *var_betn1_t_dn5_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_db0: f64 = *var_betn2_t_db0_slot;
        let mut var_betn2_t_db1: f64 = *var_betn2_t_db1_slot;
        let mut var_betn2_t_db2: f64 = *var_betn2_t_db2_slot;
        let mut var_betn2_t_db3: f64 = *var_betn2_t_db3_slot;
        let mut var_betn2_t_dn0: f64 = *var_betn2_t_dn0_slot;
        let mut var_betn2_t_dn1: f64 = *var_betn2_t_dn1_slot;
        let mut var_betn2_t_dn2: f64 = *var_betn2_t_dn2_slot;
        let mut var_betn2_t_dn3: f64 = *var_betn2_t_dn3_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn5: f64 = *var_betn2_t_dn5_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_db0: f64 = *var_betn_p_db0_slot;
        let mut var_betn_p_db1: f64 = *var_betn_p_db1_slot;
        let mut var_betn_p_db2: f64 = *var_betn_p_db2_slot;
        let mut var_betn_p_db3: f64 = *var_betn_p_db3_slot;
        let mut var_betn_p_dn0: f64 = *var_betn_p_dn0_slot;
        let mut var_betn_p_dn1: f64 = *var_betn_p_dn1_slot;
        let mut var_betn_p_dn2: f64 = *var_betn_p_dn2_slot;
        let mut var_betn_p_dn3: f64 = *var_betn_p_dn3_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn5: f64 = *var_betn_p_dn5_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_csbi_i: f64 = *var_csbi_i_slot;
        let mut var_csfi_i: f64 = *var_csfi_i_slot;
        let mut var_csthr_i: f64 = *var_csthr_i_slot;
        let mut var_csthrb_i: f64 = *var_csthrb_i_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_ge: f64 = *var_ge_slot;
        let mut var_ge_db0: f64 = *var_ge_db0_slot;
        let mut var_ge_db1: f64 = *var_ge_db1_slot;
        let mut var_ge_db2: f64 = *var_ge_db2_slot;
        let mut var_ge_db3: f64 = *var_ge_db3_slot;
        let mut var_ge_dn0: f64 = *var_ge_dn0_slot;
        let mut var_ge_dn1: f64 = *var_ge_dn1_slot;
        let mut var_ge_dn2: f64 = *var_ge_dn2_slot;
        let mut var_ge_dn3: f64 = *var_ge_dn3_slot;
        let mut var_ge_dn4: f64 = *var_ge_dn4_slot;
        let mut var_ge_dn5: f64 = *var_ge_dn5_slot;
        let mut var_ge_dn6: f64 = *var_ge_dn6_slot;
        let mut var_ge_dn7: f64 = *var_ge_dn7_slot;
        let mut var_ge_dn8: f64 = *var_ge_dn8_slot;
        let mut var_ge_dn9: f64 = *var_ge_dn9_slot;
        let mut var_gpe: f64 = *var_gpe_slot;
        let mut var_gpe_db0: f64 = *var_gpe_db0_slot;
        let mut var_gpe_db1: f64 = *var_gpe_db1_slot;
        let mut var_gpe_db2: f64 = *var_gpe_db2_slot;
        let mut var_gpe_db3: f64 = *var_gpe_db3_slot;
        let mut var_gpe_dn0: f64 = *var_gpe_dn0_slot;
        let mut var_gpe_dn1: f64 = *var_gpe_dn1_slot;
        let mut var_gpe_dn2: f64 = *var_gpe_dn2_slot;
        let mut var_gpe_dn3: f64 = *var_gpe_dn3_slot;
        let mut var_gpe_dn4: f64 = *var_gpe_dn4_slot;
        let mut var_gpe_dn5: f64 = *var_gpe_dn5_slot;
        let mut var_gpe_dn6: f64 = *var_gpe_dn6_slot;
        let mut var_gpe_dn7: f64 = *var_gpe_dn7_slot;
        let mut var_gpe_dn8: f64 = *var_gpe_dn8_slot;
        let mut var_gpe_dn9: f64 = *var_gpe_dn9_slot;
        let mut var_gwe: f64 = *var_gwe_slot;
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
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_db0: f64 = *var_temp4_db0_slot;
        let mut var_temp4_db1: f64 = *var_temp4_db1_slot;
        let mut var_temp4_db2: f64 = *var_temp4_db2_slot;
        let mut var_temp4_db3: f64 = *var_temp4_db3_slot;
        let mut var_temp4_dn0: f64 = *var_temp4_dn0_slot;
        let mut var_temp4_dn1: f64 = *var_temp4_dn1_slot;
        let mut var_temp4_dn2: f64 = *var_temp4_dn2_slot;
        let mut var_temp4_dn3: f64 = *var_temp4_dn3_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn5: f64 = *var_temp4_dn5_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_thersg_i: f64 = *var_thersg_i_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_db0: f64 = *var_thesat_p_db0_slot;
        let mut var_thesat_p_db1: f64 = *var_thesat_p_db1_slot;
        let mut var_thesat_p_db2: f64 = *var_thesat_p_db2_slot;
        let mut var_thesat_p_db3: f64 = *var_thesat_p_db3_slot;
        let mut var_thesat_p_dn0: f64 = *var_thesat_p_dn0_slot;
        let mut var_thesat_p_dn1: f64 = *var_thesat_p_dn1_slot;
        let mut var_thesat_p_dn2: f64 = *var_thesat_p_dn2_slot;
        let mut var_thesat_p_dn3: f64 = *var_thesat_p_dn3_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn5: f64 = *var_thesat_p_dn5_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_db0: f64 = *var_thesat_t_db0_slot;
        let mut var_thesat_t_db1: f64 = *var_thesat_t_db1_slot;
        let mut var_thesat_t_db2: f64 = *var_thesat_t_db2_slot;
        let mut var_thesat_t_db3: f64 = *var_thesat_t_db3_slot;
        let mut var_thesat_t_dn0: f64 = *var_thesat_t_dn0_slot;
        let mut var_thesat_t_dn1: f64 = *var_thesat_t_dn1_slot;
        let mut var_thesat_t_dn2: f64 = *var_thesat_t_dn2_slot;
        let mut var_thesat_t_dn3: f64 = *var_thesat_t_dn3_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn5: f64 = *var_thesat_t_dn5_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcorb_i: f64 = *var_xcorb_i_slot;

        let (assign2810_e2453, assign2810_e2453_d_n0, assign2810_e2453_d_n1, assign2810_e2453_d_n2, assign2810_e2453_d_n3, assign2810_e2453_d_n4, assign2810_e2453_d_n5, assign2810_e2453_d_n6, assign2810_e2453_d_n7, assign2810_e2453_d_n8, assign2810_e2453_d_n9, assign2810_e2453_d_b0, assign2810_e2453_d_b1, assign2810_e2453_d_b2, assign2810_e2453_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard97 == 0.0)) {
        let assign2810_e2429: f64 = (-var_temp3);
        let assign2810_e2431: f64 = (assign2810_e2429 - 80.0);
        let assign2810_e2435: f64 = (-var_temp3);
        let assign2810_e2437: f64 = (assign2810_e2435 - 80.0);
        let assign2810_e2438: f64 = (0.5 * assign2810_e2437);
        let assign2810_e2441: f64 = (-var_temp3);
        let assign2810_e2443: f64 = (assign2810_e2441 - 80.0);
        let assign2810_e2445: f64 = (assign2810_e2443 * 0.3333333333333);
        let assign2810_e2446: f64 = (1.0 + assign2810_e2445);
        let assign2810_e2447: f64 = (assign2810_e2438 * assign2810_e2446);
        let assign2810_e2448: f64 = (1.0 + assign2810_e2447);
        let assign2810_e2449: f64 = (assign2810_e2431 * assign2810_e2448);
        let assign2810_e2450: f64 = (1.0 + assign2810_e2449);
        let assign2810_e2451: f64 = (1.80485e-35 / assign2810_e2450);
        (assign2810_e2451, (-((1.80485e-35 * (((-var_temp3_dn0) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn0)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn0) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn1) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn1)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn1) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn2) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn2)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn2) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn3) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn3)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn3) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn4) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn4)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn4) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn5) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn5)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn5) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn6) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn6)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn6) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn7) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn7)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn7) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn8) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn8)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn8) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_dn9) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_dn9)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_dn9) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_db0) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_db0)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_db0) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_db1) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_db1)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_db1) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_db2) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_db2)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_db2) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))), (-((1.80485e-35 * (((-var_temp3_db3) * assign2810_e2448) + (assign2810_e2431 * (((0.5 * (-var_temp3_db3)) * assign2810_e2446) + (assign2810_e2438 * ((-var_temp3_db3) * 0.3333333333333)))))) / (assign2810_e2450 * assign2810_e2450))),)
    } else {
        (var_temp4, var_temp4_dn0, var_temp4_dn1, var_temp4_dn2, var_temp4_dn3, var_temp4_dn4, var_temp4_dn5, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9, var_temp4_db0, var_temp4_db1, var_temp4_db2, var_temp4_db3,)
    }
};
        var_temp4 = assign2810_e2453;
        var_temp4_dn0 = assign2810_e2453_d_n0;
        var_temp4_dn1 = assign2810_e2453_d_n1;
        var_temp4_dn2 = assign2810_e2453_d_n2;
        var_temp4_dn3 = assign2810_e2453_d_n3;
        var_temp4_dn4 = assign2810_e2453_d_n4;
        var_temp4_dn5 = assign2810_e2453_d_n5;
        var_temp4_dn6 = assign2810_e2453_d_n6;
        var_temp4_dn7 = assign2810_e2453_d_n7;
        var_temp4_dn8 = assign2810_e2453_d_n8;
        var_temp4_dn9 = assign2810_e2453_d_n9;
        var_temp4_db0 = assign2810_e2453_d_b0;
        var_temp4_db1 = assign2810_e2453_d_b1;
        var_temp4_db2 = assign2810_e2453_d_b2;
        var_temp4_db3 = assign2810_e2453_d_b3;

        let (assign2820_e2482, assign2820_e2482_d_n0, assign2820_e2482_d_n1, assign2820_e2482_d_n2, assign2820_e2482_d_n3, assign2820_e2482_d_n4, assign2820_e2482_d_n5, assign2820_e2482_d_n6, assign2820_e2482_d_n7, assign2820_e2482_d_n8, assign2820_e2482_d_n9, assign2820_e2482_d_b0, assign2820_e2482_d_b1, assign2820_e2482_d_b2, assign2820_e2482_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2820_e2461: f64 = (p.p242 * var_iwe);
        let assign2820_e2462: f64 = (1.0 + assign2820_e2461);
        let assign2820_e2463: f64 = (p.p241 * assign2820_e2462);
        let assign2820_e2466: f64 = (var_temp2 - 1.0);
        let assign2820_e2467: f64 = (assign2820_e2463 * assign2820_e2466);
        let assign2820_e2469: f64 = (assign2820_e2467 / var_temp1);
        let assign2820_e2470: f64 = (1.0 + assign2820_e2469);
        let assign2820_e2474: f64 = (var_temp4 - 1.0);
        let assign2820_e2475: f64 = (p.p245 * assign2820_e2474);
        let assign2820_e2477: f64 = (assign2820_e2475 / var_temp3);
        let assign2820_e2478: f64 = (assign2820_e2470 + assign2820_e2477);
        let assign2820_e2480: f64 = (assign2820_e2478).max(1e-6);
        (assign2820_e2480, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn0) * var_temp1) - (assign2820_e2467 * var_temp1_dn0)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn0) * var_temp3) - (assign2820_e2475 * var_temp3_dn0)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn1) * var_temp1) - (assign2820_e2467 * var_temp1_dn1)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn1) * var_temp3) - (assign2820_e2475 * var_temp3_dn1)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn2) * var_temp1) - (assign2820_e2467 * var_temp1_dn2)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn2) * var_temp3) - (assign2820_e2475 * var_temp3_dn2)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn3) * var_temp1) - (assign2820_e2467 * var_temp1_dn3)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn3) * var_temp3) - (assign2820_e2475 * var_temp3_dn3)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn4) * var_temp1) - (assign2820_e2467 * var_temp1_dn4)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn4) * var_temp3) - (assign2820_e2475 * var_temp3_dn4)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn5) * var_temp1) - (assign2820_e2467 * var_temp1_dn5)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn5) * var_temp3) - (assign2820_e2475 * var_temp3_dn5)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn6) * var_temp1) - (assign2820_e2467 * var_temp1_dn6)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn6) * var_temp3) - (assign2820_e2475 * var_temp3_dn6)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn7) * var_temp1) - (assign2820_e2467 * var_temp1_dn7)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn7) * var_temp3) - (assign2820_e2475 * var_temp3_dn7)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn8) * var_temp1) - (assign2820_e2467 * var_temp1_dn8)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn8) * var_temp3) - (assign2820_e2475 * var_temp3_dn8)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_dn9) * var_temp1) - (assign2820_e2467 * var_temp1_dn9)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_dn9) * var_temp3) - (assign2820_e2475 * var_temp3_dn9)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_db0) * var_temp1) - (assign2820_e2467 * var_temp1_db0)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_db0) * var_temp3) - (assign2820_e2475 * var_temp3_db0)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_db1) * var_temp1) - (assign2820_e2467 * var_temp1_db1)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_db1) * var_temp3) - (assign2820_e2475 * var_temp3_db1)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_db2) * var_temp1) - (assign2820_e2467 * var_temp1_db2)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_db2) * var_temp3) - (assign2820_e2475 * var_temp3_db2)) / (var_temp3 * var_temp3))) } else { 0.0 }, if assign2820_e2478 >= 1e-6 { (((((assign2820_e2463 * var_temp2_db3) * var_temp1) - (assign2820_e2467 * var_temp1_db3)) / (var_temp1 * var_temp1)) + ((((p.p245 * var_temp4_db3) * var_temp3) - (assign2820_e2475 * var_temp3_db3)) / (var_temp3 * var_temp3))) } else { 0.0 },)
    } else {
        (var_gpe, var_gpe_dn0, var_gpe_dn1, var_gpe_dn2, var_gpe_dn3, var_gpe_dn4, var_gpe_dn5, var_gpe_dn6, var_gpe_dn7, var_gpe_dn8, var_gpe_dn9, var_gpe_db0, var_gpe_db1, var_gpe_db2, var_gpe_db3,)
    }
};
        var_gpe = assign2820_e2482;
        var_gpe_dn0 = assign2820_e2482_d_n0;
        var_gpe_dn1 = assign2820_e2482_d_n1;
        var_gpe_dn2 = assign2820_e2482_d_n2;
        var_gpe_dn3 = assign2820_e2482_d_n3;
        var_gpe_dn4 = assign2820_e2482_d_n4;
        var_gpe_dn5 = assign2820_e2482_d_n5;
        var_gpe_dn6 = assign2820_e2482_d_n6;
        var_gpe_dn7 = assign2820_e2482_d_n7;
        var_gpe_dn8 = assign2820_e2482_d_n8;
        var_gpe_dn9 = assign2820_e2482_d_n9;
        var_gpe_db0 = assign2820_e2482_d_b0;
        var_gpe_db1 = assign2820_e2482_d_b1;
        var_gpe_db2 = assign2820_e2482_d_b2;
        var_gpe_db3 = assign2820_e2482_d_b3;

        let (assign2830_e2504,) = {
    if (var_guard83 == 0.0) {
        let assign2830_e2488: f64 = (p.p247 * var_iwe);
        let assign2830_e2489: f64 = (1.0 + assign2830_e2488);
        let assign2830_e2492: f64 = (p.p248 * var_iwe);
        let assign2830_e2496: f64 = (var_we / p.p249);
        let assign2830_e2497: f64 = (1.0 + assign2830_e2496);
        let assign2830_e2498: f64 = (assign2830_e2497).ln();
        let assign2830_e2499: f64 = (assign2830_e2492 * assign2830_e2498);
        let assign2830_e2500: f64 = (assign2830_e2489 + assign2830_e2499);
        let assign2830_e2502: f64 = (assign2830_e2500).max(1e-6);
        (assign2830_e2502,)
    } else {
        (var_gwe,)
    }
};
        var_gwe = assign2830_e2504;

        let (assign2840_e2513, assign2840_e2513_d_n0, assign2840_e2513_d_n1, assign2840_e2513_d_n2, assign2840_e2513_d_n3, assign2840_e2513_d_n4, assign2840_e2513_d_n5, assign2840_e2513_d_n6, assign2840_e2513_d_n7, assign2840_e2513_d_n8, assign2840_e2513_d_n9, assign2840_e2513_d_b0, assign2840_e2513_d_b1, assign2840_e2513_d_b2, assign2840_e2513_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2840_e2509: f64 = (p.p240 / var_gpe);
        let assign2840_e2511: f64 = (assign2840_e2509 * var_gwe);
        (assign2840_e2511, ((-((p.p240 * var_gpe_dn0) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn1) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn2) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn3) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn4) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn5) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn6) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn7) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn8) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_dn9) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_db0) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_db1) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_db2) / (var_gpe * var_gpe))) * var_gwe), ((-((p.p240 * var_gpe_db3) / (var_gpe * var_gpe))) * var_gwe),)
    } else {
        (var_ge, var_ge_dn0, var_ge_dn1, var_ge_dn2, var_ge_dn3, var_ge_dn4, var_ge_dn5, var_ge_dn6, var_ge_dn7, var_ge_dn8, var_ge_dn9, var_ge_db0, var_ge_db1, var_ge_db2, var_ge_db3,)
    }
};
        var_ge = assign2840_e2513;
        var_ge_dn0 = assign2840_e2513_d_n0;
        var_ge_dn1 = assign2840_e2513_d_n1;
        var_ge_dn2 = assign2840_e2513_d_n2;
        var_ge_dn3 = assign2840_e2513_d_n3;
        var_ge_dn4 = assign2840_e2513_d_n4;
        var_ge_dn5 = assign2840_e2513_d_n5;
        var_ge_dn6 = assign2840_e2513_d_n6;
        var_ge_dn7 = assign2840_e2513_d_n7;
        var_ge_dn8 = assign2840_e2513_d_n8;
        var_ge_dn9 = assign2840_e2513_d_n9;
        var_ge_db0 = assign2840_e2513_d_b0;
        var_ge_db1 = assign2840_e2513_d_b1;
        var_ge_db2 = assign2840_e2513_d_b2;
        var_ge_db3 = assign2840_e2513_d_b3;

        let (assign2850_e2522, assign2850_e2522_d_n0, assign2850_e2522_d_n1, assign2850_e2522_d_n2, assign2850_e2522_d_n3, assign2850_e2522_d_n4, assign2850_e2522_d_n5, assign2850_e2522_d_n6, assign2850_e2522_d_n7, assign2850_e2522_d_n8, assign2850_e2522_d_n9, assign2850_e2522_d_b0, assign2850_e2522_d_b1, assign2850_e2522_d_b2, assign2850_e2522_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2850_e2518: f64 = (var_ge * var_we);
        let assign2850_e2520: f64 = (assign2850_e2518 / var_le);
        (assign2850_e2520, ((var_ge_dn0 * var_we) / var_le), ((var_ge_dn1 * var_we) / var_le), ((var_ge_dn2 * var_we) / var_le), ((var_ge_dn3 * var_we) / var_le), ((var_ge_dn4 * var_we) / var_le), ((var_ge_dn5 * var_we) / var_le), ((var_ge_dn6 * var_we) / var_le), ((var_ge_dn7 * var_we) / var_le), ((var_ge_dn8 * var_we) / var_le), ((var_ge_dn9 * var_we) / var_le), ((var_ge_db0 * var_we) / var_le), ((var_ge_db1 * var_we) / var_le), ((var_ge_db2 * var_we) / var_le), ((var_ge_db3 * var_we) / var_le),)
    } else {
        (var_betn_p, var_betn_p_dn0, var_betn_p_dn1, var_betn_p_dn2, var_betn_p_dn3, var_betn_p_dn4, var_betn_p_dn5, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9, var_betn_p_db0, var_betn_p_db1, var_betn_p_db2, var_betn_p_db3,)
    }
};
        var_betn_p = assign2850_e2522;
        var_betn_p_dn0 = assign2850_e2522_d_n0;
        var_betn_p_dn1 = assign2850_e2522_d_n1;
        var_betn_p_dn2 = assign2850_e2522_d_n2;
        var_betn_p_dn3 = assign2850_e2522_d_n3;
        var_betn_p_dn4 = assign2850_e2522_d_n4;
        var_betn_p_dn5 = assign2850_e2522_d_n5;
        var_betn_p_dn6 = assign2850_e2522_d_n6;
        var_betn_p_dn7 = assign2850_e2522_d_n7;
        var_betn_p_dn8 = assign2850_e2522_d_n8;
        var_betn_p_dn9 = assign2850_e2522_d_n9;
        var_betn_p_db0 = assign2850_e2522_d_b0;
        var_betn_p_db1 = assign2850_e2522_d_b1;
        var_betn_p_db2 = assign2850_e2522_d_b2;
        var_betn_p_db3 = assign2850_e2522_d_b3;

        let (assign2860_e2529, assign2860_e2529_d_n0, assign2860_e2529_d_n1, assign2860_e2529_d_n2, assign2860_e2529_d_n3, assign2860_e2529_d_n4, assign2860_e2529_d_n5, assign2860_e2529_d_n6, assign2860_e2529_d_n7, assign2860_e2529_d_n8, assign2860_e2529_d_n9, assign2860_e2529_d_b0, assign2860_e2529_d_b1, assign2860_e2529_d_b2, assign2860_e2529_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2860_e2527: f64 = (var_betn_p).max(1e-10);
        (assign2860_e2527, if var_betn_p >= 1e-10 { var_betn_p_dn0 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn1 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn2 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn3 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn5 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db0 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db1 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db2 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db3 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn0, var_betn1_t_dn1, var_betn1_t_dn2, var_betn1_t_dn3, var_betn1_t_dn4, var_betn1_t_dn5, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9, var_betn1_t_db0, var_betn1_t_db1, var_betn1_t_db2, var_betn1_t_db3,)
    }
};
        var_betn1_t = assign2860_e2529;
        var_betn1_t_dn0 = assign2860_e2529_d_n0;
        var_betn1_t_dn1 = assign2860_e2529_d_n1;
        var_betn1_t_dn2 = assign2860_e2529_d_n2;
        var_betn1_t_dn3 = assign2860_e2529_d_n3;
        var_betn1_t_dn4 = assign2860_e2529_d_n4;
        var_betn1_t_dn5 = assign2860_e2529_d_n5;
        var_betn1_t_dn6 = assign2860_e2529_d_n6;
        var_betn1_t_dn7 = assign2860_e2529_d_n7;
        var_betn1_t_dn8 = assign2860_e2529_d_n8;
        var_betn1_t_dn9 = assign2860_e2529_d_n9;
        var_betn1_t_db0 = assign2860_e2529_d_b0;
        var_betn1_t_db1 = assign2860_e2529_d_b1;
        var_betn1_t_db2 = assign2860_e2529_d_b2;
        var_betn1_t_db3 = assign2860_e2529_d_b3;

        let (assign2870_e2536, assign2870_e2536_d_n0, assign2870_e2536_d_n1, assign2870_e2536_d_n2, assign2870_e2536_d_n3, assign2870_e2536_d_n4, assign2870_e2536_d_n5, assign2870_e2536_d_n6, assign2870_e2536_d_n7, assign2870_e2536_d_n8, assign2870_e2536_d_n9, assign2870_e2536_d_b0, assign2870_e2536_d_b1, assign2870_e2536_d_b2, assign2870_e2536_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign2870_e2534: f64 = (p.p250 * var_betn1_t);
        (assign2870_e2534, (p.p250 * var_betn1_t_dn0), (p.p250 * var_betn1_t_dn1), (p.p250 * var_betn1_t_dn2), (p.p250 * var_betn1_t_dn3), (p.p250 * var_betn1_t_dn4), (p.p250 * var_betn1_t_dn5), (p.p250 * var_betn1_t_dn6), (p.p250 * var_betn1_t_dn7), (p.p250 * var_betn1_t_dn8), (p.p250 * var_betn1_t_dn9), (p.p250 * var_betn1_t_db0), (p.p250 * var_betn1_t_db1), (p.p250 * var_betn1_t_db2), (p.p250 * var_betn1_t_db3),)
    } else {
        (var_betn2_t, var_betn2_t_dn0, var_betn2_t_dn1, var_betn2_t_dn2, var_betn2_t_dn3, var_betn2_t_dn4, var_betn2_t_dn5, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9, var_betn2_t_db0, var_betn2_t_db1, var_betn2_t_db2, var_betn2_t_db3,)
    }
};
        var_betn2_t = assign2870_e2536;
        var_betn2_t_dn0 = assign2870_e2536_d_n0;
        var_betn2_t_dn1 = assign2870_e2536_d_n1;
        var_betn2_t_dn2 = assign2870_e2536_d_n2;
        var_betn2_t_dn3 = assign2870_e2536_d_n3;
        var_betn2_t_dn4 = assign2870_e2536_d_n4;
        var_betn2_t_dn5 = assign2870_e2536_d_n5;
        var_betn2_t_dn6 = assign2870_e2536_d_n6;
        var_betn2_t_dn7 = assign2870_e2536_d_n7;
        var_betn2_t_dn8 = assign2870_e2536_d_n8;
        var_betn2_t_dn9 = assign2870_e2536_d_n9;
        var_betn2_t_db0 = assign2870_e2536_d_b0;
        var_betn2_t_db1 = assign2870_e2536_d_b1;
        var_betn2_t_db2 = assign2870_e2536_d_b2;
        var_betn2_t_db3 = assign2870_e2536_d_b3;

        let (assign2880_e2559,) = {
    if (var_guard83 == 0.0) {
        let assign2880_e2543: f64 = (p.p252 * var_ile);
        let assign2880_e2544: f64 = (1.0 + assign2880_e2543);
        let assign2880_e2545: f64 = (p.p251 * assign2880_e2544);
        let assign2880_e2549: f64 = (p.p253 * var_iwe);
        let assign2880_e2550: f64 = (1.0 + assign2880_e2549);
        let assign2880_e2551: f64 = (assign2880_e2545 * assign2880_e2550);
        let assign2880_e2555: f64 = (p.p254 * var_iae);
        let assign2880_e2556: f64 = (1.0 + assign2880_e2555);
        let assign2880_e2557: f64 = (assign2880_e2551 * assign2880_e2556);
        (assign2880_e2557,)
    } else {
        (var_stbet_i,)
    }
};
        var_stbet_i = assign2880_e2559;

        let (assign2890_e2582,) = {
    if (var_guard83 == 0.0) {
        let assign2890_e2566: f64 = (var_ile).powf(p.p257);
        let assign2890_e2567: f64 = (p.p256 * assign2890_e2566);
        let assign2890_e2568: f64 = (p.p255 + assign2890_e2567);
        let assign2890_e2572: f64 = (p.p258 * var_iwe);
        let assign2890_e2573: f64 = (1.0 + assign2890_e2572);
        let assign2890_e2574: f64 = (assign2890_e2568 * assign2890_e2573);
        let assign2890_e2578: f64 = (p.p259 * var_iae);
        let assign2890_e2579: f64 = (1.0 + assign2890_e2578);
        let assign2890_e2580: f64 = (assign2890_e2574 * assign2890_e2579);
        (assign2890_e2580,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign2890_e2582;

        let (assign2900_e2589,) = {
    if (var_guard83 == 0.0) {
        let assign2900_e2587: f64 = (var_cs_p).max(0.0);
        (assign2900_e2587,)
    } else {
        (var_cs_t,)
    }
};
        var_cs_t = assign2900_e2589;

        let (assign2910_e2594,) = {
    if (var_guard83 == 0.0) {
        (p.p260,)
    } else {
        (var_csfi_i,)
    }
};
        var_csfi_i = assign2910_e2594;

        let (assign2920_e2599,) = {
    if (var_guard83 == 0.0) {
        (p.p261,)
    } else {
        (var_csbi_i,)
    }
};
        var_csbi_i = assign2920_e2599;

        let (assign2930_e2622,) = {
    if (var_guard83 == 0.0) {
        let assign2930_e2606: f64 = (p.p263 * var_ile);
        let assign2930_e2607: f64 = (1.0 + assign2930_e2606);
        let assign2930_e2608: f64 = (p.p262 * assign2930_e2607);
        let assign2930_e2612: f64 = (p.p264 * var_iwe);
        let assign2930_e2613: f64 = (1.0 + assign2930_e2612);
        let assign2930_e2614: f64 = (assign2930_e2608 * assign2930_e2613);
        let assign2930_e2618: f64 = (p.p265 * var_iae);
        let assign2930_e2619: f64 = (1.0 + assign2930_e2618);
        let assign2930_e2620: f64 = (assign2930_e2614 * assign2930_e2619);
        (assign2930_e2620,)
    } else {
        (var_stcs_i,)
    }
};
        var_stcs_i = assign2930_e2622;

        let (assign2940_e2627,) = {
    if (var_guard83 == 0.0) {
        (p.p266,)
    } else {
        (var_thecs_t,)
    }
};
        var_thecs_t = assign2940_e2627;

        let (assign2950_e2632,) = {
    if (var_guard83 == 0.0) {
        (p.p267,)
    } else {
        (var_stthecs_i,)
    }
};
        var_stthecs_i = assign2950_e2632;

        let (assign2960_e2637,) = {
    if (var_guard83 == 0.0) {
        (p.p268,)
    } else {
        (var_csthr_i,)
    }
};
        var_csthr_i = assign2960_e2637;

        let (assign2970_e2642,) = {
    if (var_guard83 == 0.0) {
        (p.p269,)
    } else {
        (var_csthrb_i,)
    }
};
        var_csthrb_i = assign2970_e2642;

        let (assign2980_e2647,) = {
    if (var_guard83 == 0.0) {
        (p.p270,)
    } else {
        (var_mue_t,)
    }
};
        var_mue_t = assign2980_e2647;

        let (assign2990_e2652,) = {
    if (var_guard83 == 0.0) {
        (p.p271,)
    } else {
        (var_stmue_i,)
    }
};
        var_stmue_i = assign2990_e2652;

        let (assign3000_e2657,) = {
    if (var_guard83 == 0.0) {
        (p.p272,)
    } else {
        (var_themu_t,)
    }
};
        var_themu_t = assign3000_e2657;

        let (assign3010_e2662,) = {
    if (var_guard83 == 0.0) {
        (p.p273,)
    } else {
        (var_stthemu_i,)
    }
};
        var_stthemu_i = assign3010_e2662;

        let (assign3020_e2685,) = {
    if (var_guard83 == 0.0) {
        let assign3020_e2669: f64 = (var_ile).powf(p.p276);
        let assign3020_e2670: f64 = (p.p275 * assign3020_e2669);
        let assign3020_e2671: f64 = (p.p274 + assign3020_e2670);
        let assign3020_e2675: f64 = (p.p277 * var_iwe);
        let assign3020_e2676: f64 = (1.0 + assign3020_e2675);
        let assign3020_e2677: f64 = (assign3020_e2671 * assign3020_e2676);
        let assign3020_e2681: f64 = (p.p278 * var_iae);
        let assign3020_e2682: f64 = (1.0 + assign3020_e2681);
        let assign3020_e2683: f64 = (assign3020_e2677 * assign3020_e2682);
        (assign3020_e2683,)
    } else {
        (var_xcor_t,)
    }
};
        var_xcor_t = assign3020_e2685;

        let (assign3030_e2690,) = {
    if (var_guard83 == 0.0) {
        (p.p279,)
    } else {
        (var_xcorb_i,)
    }
};
        var_xcorb_i = assign3030_e2690;

        let (assign3040_e2695,) = {
    if (var_guard83 == 0.0) {
        (p.p280,)
    } else {
        (var_stxcor_i,)
    }
};
        var_stxcor_i = assign3040_e2695;

        let (assign3050_e2700,) = {
    if (var_guard83 == 0.0) {
        (p.p281,)
    } else {
        (var_feta_i,)
    }
};
        var_feta_i = assign3050_e2700;

        let (assign3060_e2713,) = {
    if (var_guard83 == 0.0) {
        let assign3060_e2705: f64 = (p.p282 * var_iwe);
        let assign3060_e2709: f64 = (p.p283 * var_iwe);
        let assign3060_e2710: f64 = (1.0 + assign3060_e2709);
        let assign3060_e2711: f64 = (assign3060_e2705 * assign3060_e2710);
        (assign3060_e2711,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign3060_e2713;

        let (assign3070_e2720,) = {
    if (var_guard83 == 0.0) {
        let assign3070_e2718: f64 = (var_rs_p).max(0.0);
        (assign3070_e2718,)
    } else {
        (var_rs_t,)
    }
};
        var_rs_t = assign3070_e2720;

        let (assign3080_e2725,) = {
    if (var_guard83 == 0.0) {
        (p.p284,)
    } else {
        (var_rsig_i,)
    }
};
        var_rsig_i = assign3080_e2725;

        let (assign3090_e2730,) = {
    if (var_guard83 == 0.0) {
        (p.p285,)
    } else {
        (var_strs_i,)
    }
};
        var_strs_i = assign3090_e2730;

        let (assign3100_e2735,) = {
    if (var_guard83 == 0.0) {
        (p.p286,)
    } else {
        (var_rsg_i,)
    }
};
        var_rsg_i = assign3100_e2735;

        let (assign3110_e2740,) = {
    if (var_guard83 == 0.0) {
        (p.p287,)
    } else {
        (var_thersg_i,)
    }
};
        var_thersg_i = assign3110_e2740;

        let (assign3120_e2745,) = {
    if (var_guard83 == 0.0) {
        (p.p288,)
    } else {
        (var_rsb_i,)
    }
};
        var_rsb_i = assign3120_e2745;

        let (assign3130_e2770, assign3130_e2770_d_n0, assign3130_e2770_d_n1, assign3130_e2770_d_n2, assign3130_e2770_d_n3, assign3130_e2770_d_n4, assign3130_e2770_d_n5, assign3130_e2770_d_n6, assign3130_e2770_d_n7, assign3130_e2770_d_n8, assign3130_e2770_d_n9, assign3130_e2770_d_b0, assign3130_e2770_d_b1, assign3130_e2770_d_b2, assign3130_e2770_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3130_e2753: f64 = (var_ile).powf(p.p291);
        let assign3130_e2754: f64 = (p.p290 * assign3130_e2753);
        let assign3130_e2755: f64 = (p.p289 + assign3130_e2754);
        let assign3130_e2756: f64 = (var_ge * assign3130_e2755);
        let assign3130_e2760: f64 = (p.p292 * var_iwe);
        let assign3130_e2761: f64 = (1.0 + assign3130_e2760);
        let assign3130_e2762: f64 = (assign3130_e2756 * assign3130_e2761);
        let assign3130_e2766: f64 = (p.p293 * var_iae);
        let assign3130_e2767: f64 = (1.0 + assign3130_e2766);
        let assign3130_e2768: f64 = (assign3130_e2762 * assign3130_e2767);
        (assign3130_e2768, (((var_ge_dn0 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn1 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn2 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn3 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn4 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn5 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn6 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn7 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn8 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_dn9 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_db0 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_db1 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_db2 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767), (((var_ge_db3 * assign3130_e2755) * assign3130_e2761) * assign3130_e2767),)
    } else {
        (var_thesat_p, var_thesat_p_dn0, var_thesat_p_dn1, var_thesat_p_dn2, var_thesat_p_dn3, var_thesat_p_dn4, var_thesat_p_dn5, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9, var_thesat_p_db0, var_thesat_p_db1, var_thesat_p_db2, var_thesat_p_db3,)
    }
};
        var_thesat_p = assign3130_e2770;
        var_thesat_p_dn0 = assign3130_e2770_d_n0;
        var_thesat_p_dn1 = assign3130_e2770_d_n1;
        var_thesat_p_dn2 = assign3130_e2770_d_n2;
        var_thesat_p_dn3 = assign3130_e2770_d_n3;
        var_thesat_p_dn4 = assign3130_e2770_d_n4;
        var_thesat_p_dn5 = assign3130_e2770_d_n5;
        var_thesat_p_dn6 = assign3130_e2770_d_n6;
        var_thesat_p_dn7 = assign3130_e2770_d_n7;
        var_thesat_p_dn8 = assign3130_e2770_d_n8;
        var_thesat_p_dn9 = assign3130_e2770_d_n9;
        var_thesat_p_db0 = assign3130_e2770_d_b0;
        var_thesat_p_db1 = assign3130_e2770_d_b1;
        var_thesat_p_db2 = assign3130_e2770_d_b2;
        var_thesat_p_db3 = assign3130_e2770_d_b3;

        let (assign3140_e2777, assign3140_e2777_d_n0, assign3140_e2777_d_n1, assign3140_e2777_d_n2, assign3140_e2777_d_n3, assign3140_e2777_d_n4, assign3140_e2777_d_n5, assign3140_e2777_d_n6, assign3140_e2777_d_n7, assign3140_e2777_d_n8, assign3140_e2777_d_n9, assign3140_e2777_d_b0, assign3140_e2777_d_b1, assign3140_e2777_d_b2, assign3140_e2777_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3140_e2775: f64 = (var_thesat_p).max(0.0);
        (assign3140_e2775, if var_thesat_p >= 0.0 { var_thesat_p_dn0 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn1 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn2 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn3 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn5 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db0 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db1 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db2 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db3 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn0, var_thesat_t_dn1, var_thesat_t_dn2, var_thesat_t_dn3, var_thesat_t_dn4, var_thesat_t_dn5, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9, var_thesat_t_db0, var_thesat_t_db1, var_thesat_t_db2, var_thesat_t_db3,)
    }
};
        var_thesat_t = assign3140_e2777;
        var_thesat_t_dn0 = assign3140_e2777_d_n0;
        var_thesat_t_dn1 = assign3140_e2777_d_n1;
        var_thesat_t_dn2 = assign3140_e2777_d_n2;
        var_thesat_t_dn3 = assign3140_e2777_d_n3;
        var_thesat_t_dn4 = assign3140_e2777_d_n4;
        var_thesat_t_dn5 = assign3140_e2777_d_n5;
        var_thesat_t_dn6 = assign3140_e2777_d_n6;
        var_thesat_t_dn7 = assign3140_e2777_d_n7;
        var_thesat_t_dn8 = assign3140_e2777_d_n8;
        var_thesat_t_dn9 = assign3140_e2777_d_n9;
        var_thesat_t_db0 = assign3140_e2777_d_b0;
        var_thesat_t_db1 = assign3140_e2777_d_b1;
        var_thesat_t_db2 = assign3140_e2777_d_b2;
        var_thesat_t_db3 = assign3140_e2777_d_b3;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_db0_slot = var_betn1_t_db0;
        *var_betn1_t_db1_slot = var_betn1_t_db1;
        *var_betn1_t_db2_slot = var_betn1_t_db2;
        *var_betn1_t_db3_slot = var_betn1_t_db3;
        *var_betn1_t_dn0_slot = var_betn1_t_dn0;
        *var_betn1_t_dn1_slot = var_betn1_t_dn1;
        *var_betn1_t_dn2_slot = var_betn1_t_dn2;
        *var_betn1_t_dn3_slot = var_betn1_t_dn3;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn5_slot = var_betn1_t_dn5;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_db0_slot = var_betn2_t_db0;
        *var_betn2_t_db1_slot = var_betn2_t_db1;
        *var_betn2_t_db2_slot = var_betn2_t_db2;
        *var_betn2_t_db3_slot = var_betn2_t_db3;
        *var_betn2_t_dn0_slot = var_betn2_t_dn0;
        *var_betn2_t_dn1_slot = var_betn2_t_dn1;
        *var_betn2_t_dn2_slot = var_betn2_t_dn2;
        *var_betn2_t_dn3_slot = var_betn2_t_dn3;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn5_slot = var_betn2_t_dn5;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_db0_slot = var_betn_p_db0;
        *var_betn_p_db1_slot = var_betn_p_db1;
        *var_betn_p_db2_slot = var_betn_p_db2;
        *var_betn_p_db3_slot = var_betn_p_db3;
        *var_betn_p_dn0_slot = var_betn_p_dn0;
        *var_betn_p_dn1_slot = var_betn_p_dn1;
        *var_betn_p_dn2_slot = var_betn_p_dn2;
        *var_betn_p_dn3_slot = var_betn_p_dn3;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn5_slot = var_betn_p_dn5;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_cs_p_slot = var_cs_p;
        *var_cs_t_slot = var_cs_t;
        *var_csbi_i_slot = var_csbi_i;
        *var_csfi_i_slot = var_csfi_i;
        *var_csthr_i_slot = var_csthr_i;
        *var_csthrb_i_slot = var_csthrb_i;
        *var_feta_i_slot = var_feta_i;
        *var_ge_slot = var_ge;
        *var_ge_db0_slot = var_ge_db0;
        *var_ge_db1_slot = var_ge_db1;
        *var_ge_db2_slot = var_ge_db2;
        *var_ge_db3_slot = var_ge_db3;
        *var_ge_dn0_slot = var_ge_dn0;
        *var_ge_dn1_slot = var_ge_dn1;
        *var_ge_dn2_slot = var_ge_dn2;
        *var_ge_dn3_slot = var_ge_dn3;
        *var_ge_dn4_slot = var_ge_dn4;
        *var_ge_dn5_slot = var_ge_dn5;
        *var_ge_dn6_slot = var_ge_dn6;
        *var_ge_dn7_slot = var_ge_dn7;
        *var_ge_dn8_slot = var_ge_dn8;
        *var_ge_dn9_slot = var_ge_dn9;
        *var_gpe_slot = var_gpe;
        *var_gpe_db0_slot = var_gpe_db0;
        *var_gpe_db1_slot = var_gpe_db1;
        *var_gpe_db2_slot = var_gpe_db2;
        *var_gpe_db3_slot = var_gpe_db3;
        *var_gpe_dn0_slot = var_gpe_dn0;
        *var_gpe_dn1_slot = var_gpe_dn1;
        *var_gpe_dn2_slot = var_gpe_dn2;
        *var_gpe_dn3_slot = var_gpe_dn3;
        *var_gpe_dn4_slot = var_gpe_dn4;
        *var_gpe_dn5_slot = var_gpe_dn5;
        *var_gpe_dn6_slot = var_gpe_dn6;
        *var_gpe_dn7_slot = var_gpe_dn7;
        *var_gpe_dn8_slot = var_gpe_dn8;
        *var_gpe_dn9_slot = var_gpe_dn9;
        *var_gwe_slot = var_gwe;
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
        *var_stxcor_i_slot = var_stxcor_i;
        *var_temp4_slot = var_temp4;
        *var_temp4_db0_slot = var_temp4_db0;
        *var_temp4_db1_slot = var_temp4_db1;
        *var_temp4_db2_slot = var_temp4_db2;
        *var_temp4_db3_slot = var_temp4_db3;
        *var_temp4_dn0_slot = var_temp4_dn0;
        *var_temp4_dn1_slot = var_temp4_dn1;
        *var_temp4_dn2_slot = var_temp4_dn2;
        *var_temp4_dn3_slot = var_temp4_dn3;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn5_slot = var_temp4_dn5;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_thecs_t_slot = var_thecs_t;
        *var_themu_t_slot = var_themu_t;
        *var_thersg_i_slot = var_thersg_i;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_db0_slot = var_thesat_p_db0;
        *var_thesat_p_db1_slot = var_thesat_p_db1;
        *var_thesat_p_db2_slot = var_thesat_p_db2;
        *var_thesat_p_db3_slot = var_thesat_p_db3;
        *var_thesat_p_dn0_slot = var_thesat_p_dn0;
        *var_thesat_p_dn1_slot = var_thesat_p_dn1;
        *var_thesat_p_dn2_slot = var_thesat_p_dn2;
        *var_thesat_p_dn3_slot = var_thesat_p_dn3;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn5_slot = var_thesat_p_dn5;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_db0_slot = var_thesat_t_db0;
        *var_thesat_t_db1_slot = var_thesat_t_db1;
        *var_thesat_t_db2_slot = var_thesat_t_db2;
        *var_thesat_t_db3_slot = var_thesat_t_db3;
        *var_thesat_t_dn0_slot = var_thesat_t_dn0;
        *var_thesat_t_dn1_slot = var_thesat_t_dn1;
        *var_thesat_t_dn2_slot = var_thesat_t_dn2;
        *var_thesat_t_dn3_slot = var_thesat_t_dn3;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn5_slot = var_thesat_t_dn5;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcorb_i_slot = var_xcorb_i;
    }

    pub(super) fn stamp_transient_block_8(
        p: &Parameters,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_a1_p_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_db0_slot: &mut f64,
        var_agidl_i_db1_slot: &mut f64,
        var_agidl_i_db2_slot: &mut f64,
        var_agidl_i_db3_slot: &mut f64,
        var_agidl_i_dn0_slot: &mut f64,
        var_agidl_i_dn1_slot: &mut f64,
        var_agidl_i_dn2_slot: &mut f64,
        var_agidl_i_dn3_slot: &mut f64,
        var_agidl_i_dn4_slot: &mut f64,
        var_agidl_i_dn5_slot: &mut f64,
        var_agidl_i_dn6_slot: &mut f64,
        var_agidl_i_dn7_slot: &mut f64,
        var_agidl_i_dn8_slot: &mut f64,
        var_agidl_i_dn9_slot: &mut f64,
        var_agidl_p_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_db0_slot: &mut f64,
        var_agidld_i_db1_slot: &mut f64,
        var_agidld_i_db2_slot: &mut f64,
        var_agidld_i_db3_slot: &mut f64,
        var_agidld_i_dn0_slot: &mut f64,
        var_agidld_i_dn1_slot: &mut f64,
        var_agidld_i_dn2_slot: &mut f64,
        var_agidld_i_dn3_slot: &mut f64,
        var_agidld_i_dn4_slot: &mut f64,
        var_agidld_i_dn5_slot: &mut f64,
        var_agidld_i_dn6_slot: &mut f64,
        var_agidld_i_dn7_slot: &mut f64,
        var_agidld_i_dn8_slot: &mut f64,
        var_agidld_i_dn9_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alpb_i_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_dgidl_i_slot: &mut f64,
        var_dgidld_i_slot: &mut f64,
        var_fnovinv_t_slot: &mut f64,
        var_fnovinvd_t_slot: &mut f64,
        var_gc2ch_i_slot: &mut f64,
        var_gc2ovacc_i_slot: &mut f64,
        var_gc2ovinv_i_slot: &mut f64,
        var_gc3ch_i_slot: &mut f64,
        var_gc3ovacc_i_slot: &mut f64,
        var_gc3ovinv_i_slot: &mut f64,
        var_gcdov_i_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gcovinvfn_i_slot: &mut f64,
        var_gcvdov_i_slot: &mut f64,
        var_iginv_t_slot: &mut f64,
        var_igovacc_t_slot: &mut f64,
        var_igovaccd_t_slot: &mut f64,
        var_igovinv_t_slot: &mut f64,
        var_igovinvd_t_slot: &mut f64,
        var_niginv_i_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stigfn_i_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_thesat1_i_slot: &mut f64,
        var_thesat2_i_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vpg_i_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_db0: f64 = *var_agidl_i_db0_slot;
        let mut var_agidl_i_db1: f64 = *var_agidl_i_db1_slot;
        let mut var_agidl_i_db2: f64 = *var_agidl_i_db2_slot;
        let mut var_agidl_i_db3: f64 = *var_agidl_i_db3_slot;
        let mut var_agidl_i_dn0: f64 = *var_agidl_i_dn0_slot;
        let mut var_agidl_i_dn1: f64 = *var_agidl_i_dn1_slot;
        let mut var_agidl_i_dn2: f64 = *var_agidl_i_dn2_slot;
        let mut var_agidl_i_dn3: f64 = *var_agidl_i_dn3_slot;
        let mut var_agidl_i_dn4: f64 = *var_agidl_i_dn4_slot;
        let mut var_agidl_i_dn5: f64 = *var_agidl_i_dn5_slot;
        let mut var_agidl_i_dn6: f64 = *var_agidl_i_dn6_slot;
        let mut var_agidl_i_dn7: f64 = *var_agidl_i_dn7_slot;
        let mut var_agidl_i_dn8: f64 = *var_agidl_i_dn8_slot;
        let mut var_agidl_i_dn9: f64 = *var_agidl_i_dn9_slot;
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_db0: f64 = *var_agidld_i_db0_slot;
        let mut var_agidld_i_db1: f64 = *var_agidld_i_db1_slot;
        let mut var_agidld_i_db2: f64 = *var_agidld_i_db2_slot;
        let mut var_agidld_i_db3: f64 = *var_agidld_i_db3_slot;
        let mut var_agidld_i_dn0: f64 = *var_agidld_i_dn0_slot;
        let mut var_agidld_i_dn1: f64 = *var_agidld_i_dn1_slot;
        let mut var_agidld_i_dn2: f64 = *var_agidld_i_dn2_slot;
        let mut var_agidld_i_dn3: f64 = *var_agidld_i_dn3_slot;
        let mut var_agidld_i_dn4: f64 = *var_agidld_i_dn4_slot;
        let mut var_agidld_i_dn5: f64 = *var_agidld_i_dn5_slot;
        let mut var_agidld_i_dn6: f64 = *var_agidld_i_dn6_slot;
        let mut var_agidld_i_dn7: f64 = *var_agidld_i_dn7_slot;
        let mut var_agidld_i_dn8: f64 = *var_agidld_i_dn8_slot;
        let mut var_agidld_i_dn9: f64 = *var_agidld_i_dn9_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alpb_i: f64 = *var_alpb_i_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_dgidl_i: f64 = *var_dgidl_i_slot;
        let mut var_dgidld_i: f64 = *var_dgidld_i_slot;
        let mut var_fnovinv_t: f64 = *var_fnovinv_t_slot;
        let mut var_fnovinvd_t: f64 = *var_fnovinvd_t_slot;
        let mut var_gc2ch_i: f64 = *var_gc2ch_i_slot;
        let mut var_gc2ovacc_i: f64 = *var_gc2ovacc_i_slot;
        let mut var_gc2ovinv_i: f64 = *var_gc2ovinv_i_slot;
        let mut var_gc3ch_i: f64 = *var_gc3ch_i_slot;
        let mut var_gc3ovacc_i: f64 = *var_gc3ovacc_i_slot;
        let mut var_gc3ovinv_i: f64 = *var_gc3ovinv_i_slot;
        let mut var_gcdov_i: f64 = *var_gcdov_i_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gcovinvfn_i: f64 = *var_gcovinvfn_i_slot;
        let mut var_gcvdov_i: f64 = *var_gcvdov_i_slot;
        let mut var_iginv_t: f64 = *var_iginv_t_slot;
        let mut var_igovacc_t: f64 = *var_igovacc_t_slot;
        let mut var_igovaccd_t: f64 = *var_igovaccd_t_slot;
        let mut var_igovinv_t: f64 = *var_igovinv_t_slot;
        let mut var_igovinvd_t: f64 = *var_igovinvd_t_slot;
        let mut var_niginv_i: f64 = *var_niginv_i_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stigfn_i: f64 = *var_stigfn_i_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_thesat1_i: f64 = *var_thesat1_i_slot;
        let mut var_thesat2_i: f64 = *var_thesat2_i_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vpg_i: f64 = *var_vpg_i_slot;

        let (assign3150_e2800,) = {
    if (var_guard83 == 0.0) {
        let assign3150_e2784: f64 = (p.p295 * var_ile);
        let assign3150_e2785: f64 = (1.0 + assign3150_e2784);
        let assign3150_e2786: f64 = (p.p294 * assign3150_e2785);
        let assign3150_e2790: f64 = (p.p296 * var_iwe);
        let assign3150_e2791: f64 = (1.0 + assign3150_e2790);
        let assign3150_e2792: f64 = (assign3150_e2786 * assign3150_e2791);
        let assign3150_e2796: f64 = (p.p297 * var_iae);
        let assign3150_e2797: f64 = (1.0 + assign3150_e2796);
        let assign3150_e2798: f64 = (assign3150_e2792 * assign3150_e2797);
        (assign3150_e2798,)
    } else {
        (var_stthesat_i,)
    }
};
        var_stthesat_i = assign3150_e2800;

        let (assign3160_e2805,) = {
    if (var_guard83 == 0.0) {
        (p.p298,)
    } else {
        (var_thesat1_i,)
    }
};
        var_thesat1_i = assign3160_e2805;

        let (assign3170_e2810,) = {
    if (var_guard83 == 0.0) {
        (p.p299,)
    } else {
        (var_thesat2_i,)
    }
};
        var_thesat2_i = assign3170_e2810;

        let (assign3180_e2831,) = {
    if (var_guard83 == 0.0) {
        let assign3180_e2818: f64 = (var_ile).powf(p.p302);
        let assign3180_e2819: f64 = (p.p301 * assign3180_e2818);
        let assign3180_e2824: f64 = (var_ile).powf(p.p304);
        let assign3180_e2825: f64 = (p.p303 * assign3180_e2824);
        let assign3180_e2826: f64 = (1.0 + assign3180_e2825);
        let assign3180_e2827: f64 = (assign3180_e2819 / assign3180_e2826);
        let assign3180_e2828: f64 = (1.0 + assign3180_e2827);
        let assign3180_e2829: f64 = (p.p300 / assign3180_e2828);
        (assign3180_e2829,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign3180_e2831;

        let (assign3190_e2840,) = {
    if (var_guard83 == 0.0) {
        let assign3190_e2836: f64 = (var_ax_p).max(1.0);
        let assign3190_e2838: f64 = (assign3190_e2836).min(16.0);
        (assign3190_e2838,)
    } else {
        (var_ax_i,)
    }
};
        var_ax_i = assign3190_e2840;

        let (assign3200_e2863,) = {
    if (var_guard83 == 0.0) {
        let assign3200_e2846: f64 = (var_ile).powf(p.p306);
        let assign3200_e2847: f64 = (p.p305 * assign3200_e2846);
        let assign3200_e2851: f64 = (p.p309 * var_iwe);
        let assign3200_e2852: f64 = (1.0 + assign3200_e2851);
        let assign3200_e2853: f64 = (assign3200_e2847 * assign3200_e2852);
        let assign3200_e2858: f64 = (var_ile).powf(p.p308);
        let assign3200_e2859: f64 = (p.p307 * assign3200_e2858);
        let assign3200_e2860: f64 = (1.0 + assign3200_e2859);
        let assign3200_e2861: f64 = (assign3200_e2853 / assign3200_e2860);
        (assign3200_e2861,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign3200_e2863;

        let (assign3210_e2870,) = {
    if (var_guard83 == 0.0) {
        let assign3210_e2868: f64 = (var_alp_p).max(0.0);
        (assign3210_e2868,)
    } else {
        (var_alp_i,)
    }
};
        var_alp_i = assign3210_e2870;

        let (assign3220_e2893,) = {
    if (var_guard83 == 0.0) {
        let assign3220_e2876: f64 = (var_ile).powf(p.p311);
        let assign3220_e2877: f64 = (p.p310 * assign3220_e2876);
        let assign3220_e2881: f64 = (p.p314 * var_iwe);
        let assign3220_e2882: f64 = (1.0 + assign3220_e2881);
        let assign3220_e2883: f64 = (assign3220_e2877 * assign3220_e2882);
        let assign3220_e2888: f64 = (var_ile).powf(p.p313);
        let assign3220_e2889: f64 = (p.p312 * assign3220_e2888);
        let assign3220_e2890: f64 = (1.0 + assign3220_e2889);
        let assign3220_e2891: f64 = (assign3220_e2883 / assign3220_e2890);
        (assign3220_e2891,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign3220_e2893;

        let (assign3230_e2900,) = {
    if (var_guard83 == 0.0) {
        let assign3230_e2898: f64 = (var_alp1_p).max(0.0);
        (assign3230_e2898,)
    } else {
        (var_alp1_i,)
    }
};
        var_alp1_i = assign3230_e2900;

        let (assign3240_e2905,) = {
    if (var_guard83 == 0.0) {
        (p.p315,)
    } else {
        (var_alpb_i,)
    }
};
        var_alpb_i = assign3240_e2905;

        let (assign3250_e2910,) = {
    if (var_guard83 == 0.0) {
        (p.p316,)
    } else {
        (var_vp_i,)
    }
};
        var_vp_i = assign3250_e2910;

        let (assign3260_e2915,) = {
    if (var_guard83 == 0.0) {
        (p.p317,)
    } else {
        (var_vpg_i,)
    }
};
        var_vpg_i = assign3260_e2915;

        let (assign3270_e2920,) = {
    if (var_guard83 == 0.0) {
        (p.p318,)
    } else {
        (var_gco_i,)
    }
};
        var_gco_i = assign3270_e2920;

        let (assign3280_e2927,) = {
    if (var_guard83 == 0.0) {
        let assign3280_e2925: f64 = (p.p319 / var_iae);
        (assign3280_e2925,)
    } else {
        (var_iginv_t,)
    }
};
        var_iginv_t = assign3280_e2927;

        let (assign3290_e2934,) = {
    if (var_guard83 == 0.0) {
        let assign3290_e2932: f64 = (p.p320 / var_iwe);
        (assign3290_e2932,)
    } else {
        (var_igovinv_t,)
    }
};
        var_igovinv_t = assign3290_e2934;

        let (assign3300_e2941,) = {
    if (var_guard83 == 0.0) {
        let assign3300_e2939: f64 = (p.p321 / var_iwe);
        (assign3300_e2939,)
    } else {
        (var_igovinvd_t,)
    }
};
        var_igovinvd_t = assign3300_e2941;

        let (assign3310_e2948,) = {
    if (var_guard83 == 0.0) {
        let assign3310_e2946: f64 = (p.p335 / var_iwe);
        (assign3310_e2946,)
    } else {
        (var_fnovinv_t,)
    }
};
        var_fnovinv_t = assign3310_e2948;

        let (assign3320_e2955,) = {
    if (var_guard83 == 0.0) {
        let assign3320_e2953: f64 = (p.p336 / var_iwe);
        (assign3320_e2953,)
    } else {
        (var_fnovinvd_t,)
    }
};
        var_fnovinvd_t = assign3320_e2955;

        let (assign3330_e2962,) = {
    if (var_guard83 == 0.0) {
        let assign3330_e2960: f64 = (p.p322 / var_iwe);
        (assign3330_e2960,)
    } else {
        (var_igovacc_t,)
    }
};
        var_igovacc_t = assign3330_e2962;

        let (assign3340_e2969,) = {
    if (var_guard83 == 0.0) {
        let assign3340_e2967: f64 = (p.p323 / var_iwe);
        (assign3340_e2967,)
    } else {
        (var_igovaccd_t,)
    }
};
        var_igovaccd_t = assign3340_e2969;

        let (assign3350_e2974,) = {
    if (var_guard83 == 0.0) {
        (p.p324,)
    } else {
        (var_stig_i,)
    }
};
        var_stig_i = assign3350_e2974;

        let (assign3360_e2979,) = {
    if (var_guard83 == 0.0) {
        (p.p338,)
    } else {
        (var_stigfn_i,)
    }
};
        var_stigfn_i = assign3360_e2979;

        let (assign3370_e2984,) = {
    if (var_guard83 == 0.0) {
        (p.p325,)
    } else {
        (var_gc2ch_i,)
    }
};
        var_gc2ch_i = assign3370_e2984;

        let (assign3380_e2989,) = {
    if (var_guard83 == 0.0) {
        (p.p326,)
    } else {
        (var_gc3ch_i,)
    }
};
        var_gc3ch_i = assign3380_e2989;

        let (assign3390_e2994,) = {
    if (var_guard83 == 0.0) {
        (p.p327,)
    } else {
        (var_gc2ovinv_i,)
    }
};
        var_gc2ovinv_i = assign3390_e2994;

        let (assign3400_e2999,) = {
    if (var_guard83 == 0.0) {
        (p.p337,)
    } else {
        (var_gcovinvfn_i,)
    }
};
        var_gcovinvfn_i = assign3400_e2999;

        let (assign3410_e3004,) = {
    if (var_guard83 == 0.0) {
        (p.p328,)
    } else {
        (var_gc3ovinv_i,)
    }
};
        var_gc3ovinv_i = assign3410_e3004;

        let (assign3420_e3009,) = {
    if (var_guard83 == 0.0) {
        (p.p329,)
    } else {
        (var_gc2ovacc_i,)
    }
};
        var_gc2ovacc_i = assign3420_e3009;

        let (assign3430_e3014,) = {
    if (var_guard83 == 0.0) {
        (p.p330,)
    } else {
        (var_gc3ovacc_i,)
    }
};
        var_gc3ovacc_i = assign3430_e3014;

        let (assign3440_e3021,) = {
    if (var_guard83 == 0.0) {
        let assign3440_e3019: f64 = (p.p331 * var_ile);
        (assign3440_e3019,)
    } else {
        (var_gcdov_i,)
    }
};
        var_gcdov_i = assign3440_e3021;

        let (assign3450_e3026,) = {
    if (var_guard83 == 0.0) {
        (p.p332,)
    } else {
        (var_gcvdov_i,)
    }
};
        var_gcvdov_i = assign3450_e3026;

        let (assign3460_e3031,) = {
    if (var_guard83 == 0.0) {
        (p.p333,)
    } else {
        (var_chib_i,)
    }
};
        var_chib_i = assign3460_e3031;

        let (assign3470_e3036,) = {
    if (var_guard83 == 0.0) {
        (p.p334,)
    } else {
        (var_niginv_i,)
    }
};
        var_niginv_i = assign3470_e3036;

        let (assign3480_e3045,) = {
    if (var_guard83 == 0.0) {
        let assign3480_e3042: f64 = (p.p341 / var_iwe);
        let assign3480_e3043: f64 = (p.p339 + assign3480_e3042);
        (assign3480_e3043,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign3480_e3045;

        let (assign3490_e3052, assign3490_e3052_d_n0, assign3490_e3052_d_n1, assign3490_e3052_d_n2, assign3490_e3052_d_n3, assign3490_e3052_d_n4, assign3490_e3052_d_n5, assign3490_e3052_d_n6, assign3490_e3052_d_n7, assign3490_e3052_d_n8, assign3490_e3052_d_n9, assign3490_e3052_d_b0, assign3490_e3052_d_b1, assign3490_e3052_d_b2, assign3490_e3052_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3490_e3050: f64 = (var_agidl_p).max(0.0);
        (assign3490_e3050, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidl_i, var_agidl_i_dn0, var_agidl_i_dn1, var_agidl_i_dn2, var_agidl_i_dn3, var_agidl_i_dn4, var_agidl_i_dn5, var_agidl_i_dn6, var_agidl_i_dn7, var_agidl_i_dn8, var_agidl_i_dn9, var_agidl_i_db0, var_agidl_i_db1, var_agidl_i_db2, var_agidl_i_db3,)
    }
};
        var_agidl_i = assign3490_e3052;
        var_agidl_i_dn0 = assign3490_e3052_d_n0;
        var_agidl_i_dn1 = assign3490_e3052_d_n1;
        var_agidl_i_dn2 = assign3490_e3052_d_n2;
        var_agidl_i_dn3 = assign3490_e3052_d_n3;
        var_agidl_i_dn4 = assign3490_e3052_d_n4;
        var_agidl_i_dn5 = assign3490_e3052_d_n5;
        var_agidl_i_dn6 = assign3490_e3052_d_n6;
        var_agidl_i_dn7 = assign3490_e3052_d_n7;
        var_agidl_i_dn8 = assign3490_e3052_d_n8;
        var_agidl_i_dn9 = assign3490_e3052_d_n9;
        var_agidl_i_db0 = assign3490_e3052_d_b0;
        var_agidl_i_db1 = assign3490_e3052_d_b1;
        var_agidl_i_db2 = assign3490_e3052_d_b2;
        var_agidl_i_db3 = assign3490_e3052_d_b3;

        let (assign3500_e3061,) = {
    if (var_guard83 == 0.0) {
        let assign3500_e3058: f64 = (p.p342 / var_iwe);
        let assign3500_e3059: f64 = (p.p340 + assign3500_e3058);
        (assign3500_e3059,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign3500_e3061;

        let (assign3510_e3068, assign3510_e3068_d_n0, assign3510_e3068_d_n1, assign3510_e3068_d_n2, assign3510_e3068_d_n3, assign3510_e3068_d_n4, assign3510_e3068_d_n5, assign3510_e3068_d_n6, assign3510_e3068_d_n7, assign3510_e3068_d_n8, assign3510_e3068_d_n9, assign3510_e3068_d_b0, assign3510_e3068_d_b1, assign3510_e3068_d_b2, assign3510_e3068_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3510_e3066: f64 = (var_agidld_p).max(0.0);
        (assign3510_e3066, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_agidld_i, var_agidld_i_dn0, var_agidld_i_dn1, var_agidld_i_dn2, var_agidld_i_dn3, var_agidld_i_dn4, var_agidld_i_dn5, var_agidld_i_dn6, var_agidld_i_dn7, var_agidld_i_dn8, var_agidld_i_dn9, var_agidld_i_db0, var_agidld_i_db1, var_agidld_i_db2, var_agidld_i_db3,)
    }
};
        var_agidld_i = assign3510_e3068;
        var_agidld_i_dn0 = assign3510_e3068_d_n0;
        var_agidld_i_dn1 = assign3510_e3068_d_n1;
        var_agidld_i_dn2 = assign3510_e3068_d_n2;
        var_agidld_i_dn3 = assign3510_e3068_d_n3;
        var_agidld_i_dn4 = assign3510_e3068_d_n4;
        var_agidld_i_dn5 = assign3510_e3068_d_n5;
        var_agidld_i_dn6 = assign3510_e3068_d_n6;
        var_agidld_i_dn7 = assign3510_e3068_d_n7;
        var_agidld_i_dn8 = assign3510_e3068_d_n8;
        var_agidld_i_dn9 = assign3510_e3068_d_n9;
        var_agidld_i_db0 = assign3510_e3068_d_b0;
        var_agidld_i_db1 = assign3510_e3068_d_b1;
        var_agidld_i_db2 = assign3510_e3068_d_b2;
        var_agidld_i_db3 = assign3510_e3068_d_b3;

        let (assign3520_e3073,) = {
    if (var_guard83 == 0.0) {
        (p.p343,)
    } else {
        (var_bgidl_t,)
    }
};
        var_bgidl_t = assign3520_e3073;

        let (assign3530_e3078,) = {
    if (var_guard83 == 0.0) {
        (p.p344,)
    } else {
        (var_bgidld_t,)
    }
};
        var_bgidld_t = assign3530_e3078;

        let (assign3540_e3083,) = {
    if (var_guard83 == 0.0) {
        (p.p345,)
    } else {
        (var_stbgidl_i,)
    }
};
        var_stbgidl_i = assign3540_e3083;

        let (assign3550_e3088,) = {
    if (var_guard83 == 0.0) {
        (p.p346,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign3550_e3088;

        let (assign3560_e3093,) = {
    if (var_guard83 == 0.0) {
        (p.p347,)
    } else {
        (var_cgidl_i,)
    }
};
        var_cgidl_i = assign3560_e3093;

        let (assign3570_e3098,) = {
    if (var_guard83 == 0.0) {
        (p.p348,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign3570_e3098;

        let (assign3580_e3107,) = {
    if (var_guard83 == 0.0) {
        let assign3580_e3104: f64 = (p.p351 * var_ile);
        let assign3580_e3105: f64 = (p.p349 + assign3580_e3104);
        (assign3580_e3105,)
    } else {
        (var_dgidl_i,)
    }
};
        var_dgidl_i = assign3580_e3107;

        let (assign3590_e3116,) = {
    if (var_guard83 == 0.0) {
        let assign3590_e3113: f64 = (p.p352 * var_ile);
        let assign3590_e3114: f64 = (p.p350 + assign3590_e3113);
        (assign3590_e3114,)
    } else {
        (var_dgidld_i,)
    }
};
        var_dgidld_i = assign3590_e3116;

        let (assign3600_e3133,) = {
    if (var_guard83 == 0.0) {
        let assign3600_e3123: f64 = (p.p385 * var_ile);
        let assign3600_e3124: f64 = (1.0 + assign3600_e3123);
        let assign3600_e3125: f64 = (p.p384 * assign3600_e3124);
        let assign3600_e3129: f64 = (p.p386 * var_iwe);
        let assign3600_e3130: f64 = (1.0 + assign3600_e3129);
        let assign3600_e3131: f64 = (assign3600_e3125 * assign3600_e3130);
        (assign3600_e3131,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign3600_e3133;

        *var_a1_p_slot = var_a1_p;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_db0_slot = var_agidl_i_db0;
        *var_agidl_i_db1_slot = var_agidl_i_db1;
        *var_agidl_i_db2_slot = var_agidl_i_db2;
        *var_agidl_i_db3_slot = var_agidl_i_db3;
        *var_agidl_i_dn0_slot = var_agidl_i_dn0;
        *var_agidl_i_dn1_slot = var_agidl_i_dn1;
        *var_agidl_i_dn2_slot = var_agidl_i_dn2;
        *var_agidl_i_dn3_slot = var_agidl_i_dn3;
        *var_agidl_i_dn4_slot = var_agidl_i_dn4;
        *var_agidl_i_dn5_slot = var_agidl_i_dn5;
        *var_agidl_i_dn6_slot = var_agidl_i_dn6;
        *var_agidl_i_dn7_slot = var_agidl_i_dn7;
        *var_agidl_i_dn8_slot = var_agidl_i_dn8;
        *var_agidl_i_dn9_slot = var_agidl_i_dn9;
        *var_agidl_p_slot = var_agidl_p;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_db0_slot = var_agidld_i_db0;
        *var_agidld_i_db1_slot = var_agidld_i_db1;
        *var_agidld_i_db2_slot = var_agidld_i_db2;
        *var_agidld_i_db3_slot = var_agidld_i_db3;
        *var_agidld_i_dn0_slot = var_agidld_i_dn0;
        *var_agidld_i_dn1_slot = var_agidld_i_dn1;
        *var_agidld_i_dn2_slot = var_agidld_i_dn2;
        *var_agidld_i_dn3_slot = var_agidld_i_dn3;
        *var_agidld_i_dn4_slot = var_agidld_i_dn4;
        *var_agidld_i_dn5_slot = var_agidld_i_dn5;
        *var_agidld_i_dn6_slot = var_agidld_i_dn6;
        *var_agidld_i_dn7_slot = var_agidld_i_dn7;
        *var_agidld_i_dn8_slot = var_agidld_i_dn8;
        *var_agidld_i_dn9_slot = var_agidld_i_dn9;
        *var_agidld_p_slot = var_agidld_p;
        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp_i_slot = var_alp_i;
        *var_alp_p_slot = var_alp_p;
        *var_alpb_i_slot = var_alpb_i;
        *var_ax_i_slot = var_ax_i;
        *var_ax_p_slot = var_ax_p;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_chib_i_slot = var_chib_i;
        *var_dgidl_i_slot = var_dgidl_i;
        *var_dgidld_i_slot = var_dgidld_i;
        *var_fnovinv_t_slot = var_fnovinv_t;
        *var_fnovinvd_t_slot = var_fnovinvd_t;
        *var_gc2ch_i_slot = var_gc2ch_i;
        *var_gc2ovacc_i_slot = var_gc2ovacc_i;
        *var_gc2ovinv_i_slot = var_gc2ovinv_i;
        *var_gc3ch_i_slot = var_gc3ch_i;
        *var_gc3ovacc_i_slot = var_gc3ovacc_i;
        *var_gc3ovinv_i_slot = var_gc3ovinv_i;
        *var_gcdov_i_slot = var_gcdov_i;
        *var_gco_i_slot = var_gco_i;
        *var_gcovinvfn_i_slot = var_gcovinvfn_i;
        *var_gcvdov_i_slot = var_gcvdov_i;
        *var_iginv_t_slot = var_iginv_t;
        *var_igovacc_t_slot = var_igovacc_t;
        *var_igovaccd_t_slot = var_igovaccd_t;
        *var_igovinv_t_slot = var_igovinv_t;
        *var_igovinvd_t_slot = var_igovinvd_t;
        *var_niginv_i_slot = var_niginv_i;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stig_i_slot = var_stig_i;
        *var_stigfn_i_slot = var_stigfn_i;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_thesat1_i_slot = var_thesat1_i;
        *var_thesat2_i_slot = var_thesat2_i;
        *var_vp_i_slot = var_vp_i;
        *var_vpg_i_slot = var_vpg_i;
    }

    pub(super) fn stamp_transient_block_9(
        p: &Parameters,
        var_a1_p: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lambda_le: f64,
        var_le: f64,
        var_lecv: f64,
        var_lphy: f64,
        var_lphy_db0: f64,
        var_lphy_db1: f64,
        var_lphy_db2: f64,
        var_lphy_db3: f64,
        var_lphy_dn0: f64,
        var_lphy_dn1: f64,
        var_lphy_dn2: f64,
        var_lphy_dn3: f64,
        var_lphy_dn4: f64,
        var_lphy_dn5: f64,
        var_lphy_dn6: f64,
        var_lphy_dn7: f64,
        var_lphy_dn8: f64,
        var_lphy_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_we: f64,
        var_wecv: f64,
        var_a1_i_slot: &mut f64,
        var_a2_t_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_areaq_i_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_db0_slot: &mut f64,
        var_betnedge_t_db1_slot: &mut f64,
        var_betnedge_t_db2_slot: &mut f64,
        var_betnedge_t_db3_slot: &mut f64,
        var_betnedge_t_dn0_slot: &mut f64,
        var_betnedge_t_dn1_slot: &mut f64,
        var_betnedge_t_dn2_slot: &mut f64,
        var_betnedge_t_dn3_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_dn5_slot: &mut f64,
        var_betnedge_t_dn6_slot: &mut f64,
        var_betnedge_t_dn7_slot: &mut f64,
        var_betnedge_t_dn8_slot: &mut f64,
        var_betnedge_t_dn9_slot: &mut f64,
        var_cf1edge_i_slot: &mut f64,
        var_cf1edge_i_db0_slot: &mut f64,
        var_cf1edge_i_db1_slot: &mut f64,
        var_cf1edge_i_db2_slot: &mut f64,
        var_cf1edge_i_db3_slot: &mut f64,
        var_cf1edge_i_dn0_slot: &mut f64,
        var_cf1edge_i_dn1_slot: &mut f64,
        var_cf1edge_i_dn2_slot: &mut f64,
        var_cf1edge_i_dn3_slot: &mut f64,
        var_cf1edge_i_dn4_slot: &mut f64,
        var_cf1edge_i_dn5_slot: &mut f64,
        var_cf1edge_i_dn6_slot: &mut f64,
        var_cf1edge_i_dn7_slot: &mut f64,
        var_cf1edge_i_dn8_slot: &mut f64,
        var_cf1edge_i_dn9_slot: &mut f64,
        var_cf2edge_i_slot: &mut f64,
        var_cf2edge_i_db0_slot: &mut f64,
        var_cf2edge_i_db1_slot: &mut f64,
        var_cf2edge_i_db2_slot: &mut f64,
        var_cf2edge_i_db3_slot: &mut f64,
        var_cf2edge_i_dn0_slot: &mut f64,
        var_cf2edge_i_dn1_slot: &mut f64,
        var_cf2edge_i_dn2_slot: &mut f64,
        var_cf2edge_i_dn3_slot: &mut f64,
        var_cf2edge_i_dn4_slot: &mut f64,
        var_cf2edge_i_dn5_slot: &mut f64,
        var_cf2edge_i_dn6_slot: &mut f64,
        var_cf2edge_i_dn7_slot: &mut f64,
        var_cf2edge_i_dn8_slot: &mut f64,
        var_cf2edge_i_dn9_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_db0_slot: &mut f64,
        var_cgbov_p_db1_slot: &mut f64,
        var_cgbov_p_db2_slot: &mut f64,
        var_cgbov_p_db3_slot: &mut f64,
        var_cgbov_p_dn0_slot: &mut f64,
        var_cgbov_p_dn1_slot: &mut f64,
        var_cgbov_p_dn2_slot: &mut f64,
        var_cgbov_p_dn3_slot: &mut f64,
        var_cgbov_p_dn4_slot: &mut f64,
        var_cgbov_p_dn5_slot: &mut f64,
        var_cgbov_p_dn6_slot: &mut f64,
        var_cgbov_p_dn7_slot: &mut f64,
        var_cgbov_p_dn8_slot: &mut f64,
        var_cgbov_p_dn9_slot: &mut f64,
        var_cic1edge_i_slot: &mut f64,
        var_cic2edge_i_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_psce1edge_i_slot: &mut f64,
        var_psce1edge_i_db0_slot: &mut f64,
        var_psce1edge_i_db1_slot: &mut f64,
        var_psce1edge_i_db2_slot: &mut f64,
        var_psce1edge_i_db3_slot: &mut f64,
        var_psce1edge_i_dn0_slot: &mut f64,
        var_psce1edge_i_dn1_slot: &mut f64,
        var_psce1edge_i_dn2_slot: &mut f64,
        var_psce1edge_i_dn3_slot: &mut f64,
        var_psce1edge_i_dn4_slot: &mut f64,
        var_psce1edge_i_dn5_slot: &mut f64,
        var_psce1edge_i_dn6_slot: &mut f64,
        var_psce1edge_i_dn7_slot: &mut f64,
        var_psce1edge_i_dn8_slot: &mut f64,
        var_psce1edge_i_dn9_slot: &mut f64,
        var_psce2edge_i_slot: &mut f64,
        var_psce2edge_i_db0_slot: &mut f64,
        var_psce2edge_i_db1_slot: &mut f64,
        var_psce2edge_i_db2_slot: &mut f64,
        var_psce2edge_i_db3_slot: &mut f64,
        var_psce2edge_i_dn0_slot: &mut f64,
        var_psce2edge_i_dn1_slot: &mut f64,
        var_psce2edge_i_dn2_slot: &mut f64,
        var_psce2edge_i_dn3_slot: &mut f64,
        var_psce2edge_i_dn4_slot: &mut f64,
        var_psce2edge_i_dn5_slot: &mut f64,
        var_psce2edge_i_dn6_slot: &mut f64,
        var_psce2edge_i_dn7_slot: &mut f64,
        var_psce2edge_i_dn8_slot: &mut f64,
        var_psce2edge_i_dn9_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_vfb1edge_t_slot: &mut f64,
        var_vfb1edge_t_db0_slot: &mut f64,
        var_vfb1edge_t_db1_slot: &mut f64,
        var_vfb1edge_t_db2_slot: &mut f64,
        var_vfb1edge_t_db3_slot: &mut f64,
        var_vfb1edge_t_dn0_slot: &mut f64,
        var_vfb1edge_t_dn1_slot: &mut f64,
        var_vfb1edge_t_dn2_slot: &mut f64,
        var_vfb1edge_t_dn3_slot: &mut f64,
        var_vfb1edge_t_dn4_slot: &mut f64,
        var_vfb1edge_t_dn5_slot: &mut f64,
        var_vfb1edge_t_dn6_slot: &mut f64,
        var_vfb1edge_t_dn7_slot: &mut f64,
        var_vfb1edge_t_dn8_slot: &mut f64,
        var_vfb1edge_t_dn9_slot: &mut f64,
        var_vfb2edge_t_slot: &mut f64,
        var_we_edge_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_areaq_i: f64 = *var_areaq_i_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_db0: f64 = *var_betnedge_t_db0_slot;
        let mut var_betnedge_t_db1: f64 = *var_betnedge_t_db1_slot;
        let mut var_betnedge_t_db2: f64 = *var_betnedge_t_db2_slot;
        let mut var_betnedge_t_db3: f64 = *var_betnedge_t_db3_slot;
        let mut var_betnedge_t_dn0: f64 = *var_betnedge_t_dn0_slot;
        let mut var_betnedge_t_dn1: f64 = *var_betnedge_t_dn1_slot;
        let mut var_betnedge_t_dn2: f64 = *var_betnedge_t_dn2_slot;
        let mut var_betnedge_t_dn3: f64 = *var_betnedge_t_dn3_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_dn5: f64 = *var_betnedge_t_dn5_slot;
        let mut var_betnedge_t_dn6: f64 = *var_betnedge_t_dn6_slot;
        let mut var_betnedge_t_dn7: f64 = *var_betnedge_t_dn7_slot;
        let mut var_betnedge_t_dn8: f64 = *var_betnedge_t_dn8_slot;
        let mut var_betnedge_t_dn9: f64 = *var_betnedge_t_dn9_slot;
        let mut var_cf1edge_i: f64 = *var_cf1edge_i_slot;
        let mut var_cf1edge_i_db0: f64 = *var_cf1edge_i_db0_slot;
        let mut var_cf1edge_i_db1: f64 = *var_cf1edge_i_db1_slot;
        let mut var_cf1edge_i_db2: f64 = *var_cf1edge_i_db2_slot;
        let mut var_cf1edge_i_db3: f64 = *var_cf1edge_i_db3_slot;
        let mut var_cf1edge_i_dn0: f64 = *var_cf1edge_i_dn0_slot;
        let mut var_cf1edge_i_dn1: f64 = *var_cf1edge_i_dn1_slot;
        let mut var_cf1edge_i_dn2: f64 = *var_cf1edge_i_dn2_slot;
        let mut var_cf1edge_i_dn3: f64 = *var_cf1edge_i_dn3_slot;
        let mut var_cf1edge_i_dn4: f64 = *var_cf1edge_i_dn4_slot;
        let mut var_cf1edge_i_dn5: f64 = *var_cf1edge_i_dn5_slot;
        let mut var_cf1edge_i_dn6: f64 = *var_cf1edge_i_dn6_slot;
        let mut var_cf1edge_i_dn7: f64 = *var_cf1edge_i_dn7_slot;
        let mut var_cf1edge_i_dn8: f64 = *var_cf1edge_i_dn8_slot;
        let mut var_cf1edge_i_dn9: f64 = *var_cf1edge_i_dn9_slot;
        let mut var_cf2edge_i: f64 = *var_cf2edge_i_slot;
        let mut var_cf2edge_i_db0: f64 = *var_cf2edge_i_db0_slot;
        let mut var_cf2edge_i_db1: f64 = *var_cf2edge_i_db1_slot;
        let mut var_cf2edge_i_db2: f64 = *var_cf2edge_i_db2_slot;
        let mut var_cf2edge_i_db3: f64 = *var_cf2edge_i_db3_slot;
        let mut var_cf2edge_i_dn0: f64 = *var_cf2edge_i_dn0_slot;
        let mut var_cf2edge_i_dn1: f64 = *var_cf2edge_i_dn1_slot;
        let mut var_cf2edge_i_dn2: f64 = *var_cf2edge_i_dn2_slot;
        let mut var_cf2edge_i_dn3: f64 = *var_cf2edge_i_dn3_slot;
        let mut var_cf2edge_i_dn4: f64 = *var_cf2edge_i_dn4_slot;
        let mut var_cf2edge_i_dn5: f64 = *var_cf2edge_i_dn5_slot;
        let mut var_cf2edge_i_dn6: f64 = *var_cf2edge_i_dn6_slot;
        let mut var_cf2edge_i_dn7: f64 = *var_cf2edge_i_dn7_slot;
        let mut var_cf2edge_i_dn8: f64 = *var_cf2edge_i_dn8_slot;
        let mut var_cf2edge_i_dn9: f64 = *var_cf2edge_i_dn9_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_db0: f64 = *var_cgbov_p_db0_slot;
        let mut var_cgbov_p_db1: f64 = *var_cgbov_p_db1_slot;
        let mut var_cgbov_p_db2: f64 = *var_cgbov_p_db2_slot;
        let mut var_cgbov_p_db3: f64 = *var_cgbov_p_db3_slot;
        let mut var_cgbov_p_dn0: f64 = *var_cgbov_p_dn0_slot;
        let mut var_cgbov_p_dn1: f64 = *var_cgbov_p_dn1_slot;
        let mut var_cgbov_p_dn2: f64 = *var_cgbov_p_dn2_slot;
        let mut var_cgbov_p_dn3: f64 = *var_cgbov_p_dn3_slot;
        let mut var_cgbov_p_dn4: f64 = *var_cgbov_p_dn4_slot;
        let mut var_cgbov_p_dn5: f64 = *var_cgbov_p_dn5_slot;
        let mut var_cgbov_p_dn6: f64 = *var_cgbov_p_dn6_slot;
        let mut var_cgbov_p_dn7: f64 = *var_cgbov_p_dn7_slot;
        let mut var_cgbov_p_dn8: f64 = *var_cgbov_p_dn8_slot;
        let mut var_cgbov_p_dn9: f64 = *var_cgbov_p_dn9_slot;
        let mut var_cic1edge_i: f64 = *var_cic1edge_i_slot;
        let mut var_cic2edge_i: f64 = *var_cic2edge_i_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_psce1edge_i: f64 = *var_psce1edge_i_slot;
        let mut var_psce1edge_i_db0: f64 = *var_psce1edge_i_db0_slot;
        let mut var_psce1edge_i_db1: f64 = *var_psce1edge_i_db1_slot;
        let mut var_psce1edge_i_db2: f64 = *var_psce1edge_i_db2_slot;
        let mut var_psce1edge_i_db3: f64 = *var_psce1edge_i_db3_slot;
        let mut var_psce1edge_i_dn0: f64 = *var_psce1edge_i_dn0_slot;
        let mut var_psce1edge_i_dn1: f64 = *var_psce1edge_i_dn1_slot;
        let mut var_psce1edge_i_dn2: f64 = *var_psce1edge_i_dn2_slot;
        let mut var_psce1edge_i_dn3: f64 = *var_psce1edge_i_dn3_slot;
        let mut var_psce1edge_i_dn4: f64 = *var_psce1edge_i_dn4_slot;
        let mut var_psce1edge_i_dn5: f64 = *var_psce1edge_i_dn5_slot;
        let mut var_psce1edge_i_dn6: f64 = *var_psce1edge_i_dn6_slot;
        let mut var_psce1edge_i_dn7: f64 = *var_psce1edge_i_dn7_slot;
        let mut var_psce1edge_i_dn8: f64 = *var_psce1edge_i_dn8_slot;
        let mut var_psce1edge_i_dn9: f64 = *var_psce1edge_i_dn9_slot;
        let mut var_psce2edge_i: f64 = *var_psce2edge_i_slot;
        let mut var_psce2edge_i_db0: f64 = *var_psce2edge_i_db0_slot;
        let mut var_psce2edge_i_db1: f64 = *var_psce2edge_i_db1_slot;
        let mut var_psce2edge_i_db2: f64 = *var_psce2edge_i_db2_slot;
        let mut var_psce2edge_i_db3: f64 = *var_psce2edge_i_db3_slot;
        let mut var_psce2edge_i_dn0: f64 = *var_psce2edge_i_dn0_slot;
        let mut var_psce2edge_i_dn1: f64 = *var_psce2edge_i_dn1_slot;
        let mut var_psce2edge_i_dn2: f64 = *var_psce2edge_i_dn2_slot;
        let mut var_psce2edge_i_dn3: f64 = *var_psce2edge_i_dn3_slot;
        let mut var_psce2edge_i_dn4: f64 = *var_psce2edge_i_dn4_slot;
        let mut var_psce2edge_i_dn5: f64 = *var_psce2edge_i_dn5_slot;
        let mut var_psce2edge_i_dn6: f64 = *var_psce2edge_i_dn6_slot;
        let mut var_psce2edge_i_dn7: f64 = *var_psce2edge_i_dn7_slot;
        let mut var_psce2edge_i_dn8: f64 = *var_psce2edge_i_dn8_slot;
        let mut var_psce2edge_i_dn9: f64 = *var_psce2edge_i_dn9_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_vfb1edge_t: f64 = *var_vfb1edge_t_slot;
        let mut var_vfb1edge_t_db0: f64 = *var_vfb1edge_t_db0_slot;
        let mut var_vfb1edge_t_db1: f64 = *var_vfb1edge_t_db1_slot;
        let mut var_vfb1edge_t_db2: f64 = *var_vfb1edge_t_db2_slot;
        let mut var_vfb1edge_t_db3: f64 = *var_vfb1edge_t_db3_slot;
        let mut var_vfb1edge_t_dn0: f64 = *var_vfb1edge_t_dn0_slot;
        let mut var_vfb1edge_t_dn1: f64 = *var_vfb1edge_t_dn1_slot;
        let mut var_vfb1edge_t_dn2: f64 = *var_vfb1edge_t_dn2_slot;
        let mut var_vfb1edge_t_dn3: f64 = *var_vfb1edge_t_dn3_slot;
        let mut var_vfb1edge_t_dn4: f64 = *var_vfb1edge_t_dn4_slot;
        let mut var_vfb1edge_t_dn5: f64 = *var_vfb1edge_t_dn5_slot;
        let mut var_vfb1edge_t_dn6: f64 = *var_vfb1edge_t_dn6_slot;
        let mut var_vfb1edge_t_dn7: f64 = *var_vfb1edge_t_dn7_slot;
        let mut var_vfb1edge_t_dn8: f64 = *var_vfb1edge_t_dn8_slot;
        let mut var_vfb1edge_t_dn9: f64 = *var_vfb1edge_t_dn9_slot;
        let mut var_vfb2edge_t: f64 = *var_vfb2edge_t_slot;
        let mut var_we_edge: f64 = *var_we_edge_slot;

        let (assign3610_e3140,) = {
    if (var_guard83 == 0.0) {
        let assign3610_e3138: f64 = (var_a1_p).max(0.0);
        (assign3610_e3138,)
    } else {
        (var_a1_i,)
    }
};
        var_a1_i = assign3610_e3140;

        let (assign3620_e3145,) = {
    if (var_guard83 == 0.0) {
        (p.p387,)
    } else {
        (var_a2_t,)
    }
};
        var_a2_t = assign3620_e3145;

        let (assign3630_e3150,) = {
    if (var_guard83 == 0.0) {
        (p.p388,)
    } else {
        (var_sta2_i,)
    }
};
        var_sta2_i = assign3630_e3150;

        let (assign3640_e3167,) = {
    if (var_guard83 == 0.0) {
        let assign3640_e3157: f64 = (p.p390 * var_ile);
        let assign3640_e3158: f64 = (1.0 + assign3640_e3157);
        let assign3640_e3159: f64 = (p.p389 * assign3640_e3158);
        let assign3640_e3163: f64 = (p.p391 * var_iwe);
        let assign3640_e3164: f64 = (1.0 + assign3640_e3163);
        let assign3640_e3165: f64 = (assign3640_e3159 * assign3640_e3164);
        (assign3640_e3165,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign3640_e3167;

        let (assign3650_e3174,) = {
    if (var_guard83 == 0.0) {
        let assign3650_e3172: f64 = (var_a3_p).max(0.0);
        (assign3650_e3172,)
    } else {
        (var_a3_i,)
    }
};
        var_a3_i = assign3650_e3174;

        let (assign3660_e3185,) = {
    if (var_guard83 == 0.0) {
        let assign3660_e3179: f64 = (2.0 * p.p353);
        let assign3660_e3182: f64 = (p.p354 * var_we);
        let assign3660_e3183: f64 = (assign3660_e3179 + assign3660_e3182);
        (assign3660_e3183,)
    } else {
        (var_we_edge,)
    }
};
        var_we_edge = assign3660_e3185;

        let (assign3670_e3190,) = {
    if (var_guard83 == 0.0) {
        (p.p355,)
    } else {
        (var_ctedge_i,)
    }
};
        var_ctedge_i = assign3670_e3190;

        let (assign3680_e3199, assign3680_e3199_d_n0, assign3680_e3199_d_n1, assign3680_e3199_d_n2, assign3680_e3199_d_n3, assign3680_e3199_d_n4, assign3680_e3199_d_n5, assign3680_e3199_d_n6, assign3680_e3199_d_n7, assign3680_e3199_d_n8, assign3680_e3199_d_n9, assign3680_e3199_d_b0, assign3680_e3199_d_b1, assign3680_e3199_d_b2, assign3680_e3199_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3680_e3196: f64 = (var_ile).powf(p.p358);
        let assign3680_e3197: f64 = (p.p357 * assign3680_e3196);
        (assign3680_e3197, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3680_e3199;
        var_temp_dn0 = assign3680_e3199_d_n0;
        var_temp_dn1 = assign3680_e3199_d_n1;
        var_temp_dn2 = assign3680_e3199_d_n2;
        var_temp_dn3 = assign3680_e3199_d_n3;
        var_temp_dn4 = assign3680_e3199_d_n4;
        var_temp_dn5 = assign3680_e3199_d_n5;
        var_temp_dn6 = assign3680_e3199_d_n6;
        var_temp_dn7 = assign3680_e3199_d_n7;
        var_temp_dn8 = assign3680_e3199_d_n8;
        var_temp_dn9 = assign3680_e3199_d_n9;
        var_temp_db0 = assign3680_e3199_d_b0;
        var_temp_db1 = assign3680_e3199_d_b1;
        var_temp_db2 = assign3680_e3199_d_b2;
        var_temp_db3 = assign3680_e3199_d_b3;

        let (assign3690_e3214, assign3690_e3214_d_n0, assign3690_e3214_d_n1, assign3690_e3214_d_n2, assign3690_e3214_d_n3, assign3690_e3214_d_n4, assign3690_e3214_d_n5, assign3690_e3214_d_n6, assign3690_e3214_d_n7, assign3690_e3214_d_n8, assign3690_e3214_d_n9, assign3690_e3214_d_b0, assign3690_e3214_d_b1, assign3690_e3214_d_b2, assign3690_e3214_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3690_e3204: f64 = (p.p356 + var_temp);
        let assign3690_e3207: f64 = (p.p359 * var_iwe);
        let assign3690_e3208: f64 = (assign3690_e3204 + assign3690_e3207);
        let assign3690_e3211: f64 = (p.p360 * var_iae);
        let assign3690_e3212: f64 = (assign3690_e3208 + assign3690_e3211);
        (assign3690_e3212, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    } else {
        (var_vfb1edge_t, var_vfb1edge_t_dn0, var_vfb1edge_t_dn1, var_vfb1edge_t_dn2, var_vfb1edge_t_dn3, var_vfb1edge_t_dn4, var_vfb1edge_t_dn5, var_vfb1edge_t_dn6, var_vfb1edge_t_dn7, var_vfb1edge_t_dn8, var_vfb1edge_t_dn9, var_vfb1edge_t_db0, var_vfb1edge_t_db1, var_vfb1edge_t_db2, var_vfb1edge_t_db3,)
    }
};
        var_vfb1edge_t = assign3690_e3214;
        var_vfb1edge_t_dn0 = assign3690_e3214_d_n0;
        var_vfb1edge_t_dn1 = assign3690_e3214_d_n1;
        var_vfb1edge_t_dn2 = assign3690_e3214_d_n2;
        var_vfb1edge_t_dn3 = assign3690_e3214_d_n3;
        var_vfb1edge_t_dn4 = assign3690_e3214_d_n4;
        var_vfb1edge_t_dn5 = assign3690_e3214_d_n5;
        var_vfb1edge_t_dn6 = assign3690_e3214_d_n6;
        var_vfb1edge_t_dn7 = assign3690_e3214_d_n7;
        var_vfb1edge_t_dn8 = assign3690_e3214_d_n8;
        var_vfb1edge_t_dn9 = assign3690_e3214_d_n9;
        var_vfb1edge_t_db0 = assign3690_e3214_d_b0;
        var_vfb1edge_t_db1 = assign3690_e3214_d_b1;
        var_vfb1edge_t_db2 = assign3690_e3214_d_b2;
        var_vfb1edge_t_db3 = assign3690_e3214_d_b3;

        let (assign3700_e3219,) = {
    if (var_guard83 == 0.0) {
        (p.p361,)
    } else {
        (var_vfb2edge_t,)
    }
};
        var_vfb2edge_t = assign3700_e3219;

        let (assign3710_e3242,) = {
    if (var_guard83 == 0.0) {
        let assign3710_e3226: f64 = (p.p363 * var_ile);
        let assign3710_e3227: f64 = (1.0 + assign3710_e3226);
        let assign3710_e3228: f64 = (p.p362 * assign3710_e3227);
        let assign3710_e3232: f64 = (p.p364 * var_iwe);
        let assign3710_e3233: f64 = (1.0 + assign3710_e3232);
        let assign3710_e3234: f64 = (assign3710_e3228 * assign3710_e3233);
        let assign3710_e3238: f64 = (p.p365 * var_iae);
        let assign3710_e3239: f64 = (1.0 + assign3710_e3238);
        let assign3710_e3240: f64 = (assign3710_e3234 * assign3710_e3239);
        (assign3710_e3240,)
    } else {
        (var_stvfbedge_i,)
    }
};
        var_stvfbedge_i = assign3710_e3242;

        let (assign3720_e3247,) = {
    if (var_guard83 == 0.0) {
        (p.p366,)
    } else {
        (var_cic1edge_i,)
    }
};
        var_cic1edge_i = assign3720_e3247;

        let (assign3730_e3252,) = {
    if (var_guard83 == 0.0) {
        (p.p367,)
    } else {
        (var_cic2edge_i,)
    }
};
        var_cic2edge_i = assign3730_e3252;

        let (assign3740_e3269, assign3740_e3269_d_n0, assign3740_e3269_d_n1, assign3740_e3269_d_n2, assign3740_e3269_d_n3, assign3740_e3269_d_n4, assign3740_e3269_d_n5, assign3740_e3269_d_n6, assign3740_e3269_d_n7, assign3740_e3269_d_n8, assign3740_e3269_d_n9, assign3740_e3269_d_b0, assign3740_e3269_d_b1, assign3740_e3269_d_b2, assign3740_e3269_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3740_e3257: f64 = (p.p368 * 2.0);
        let assign3740_e3260: f64 = (var_lambda_le).powf(p.p369);
        let assign3740_e3261: f64 = (assign3740_e3257 * assign3740_e3260);
        let assign3740_e3265: f64 = (p.p370 * var_iwe);
        let assign3740_e3266: f64 = (1.0 + assign3740_e3265);
        let assign3740_e3267: f64 = (assign3740_e3261 * assign3740_e3266);
        (assign3740_e3267, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3740_e3269;
        var_temp_dn0 = assign3740_e3269_d_n0;
        var_temp_dn1 = assign3740_e3269_d_n1;
        var_temp_dn2 = assign3740_e3269_d_n2;
        var_temp_dn3 = assign3740_e3269_d_n3;
        var_temp_dn4 = assign3740_e3269_d_n4;
        var_temp_dn5 = assign3740_e3269_d_n5;
        var_temp_dn6 = assign3740_e3269_d_n6;
        var_temp_dn7 = assign3740_e3269_d_n7;
        var_temp_dn8 = assign3740_e3269_d_n8;
        var_temp_dn9 = assign3740_e3269_d_n9;
        var_temp_db0 = assign3740_e3269_d_b0;
        var_temp_db1 = assign3740_e3269_d_b1;
        var_temp_db2 = assign3740_e3269_d_b2;
        var_temp_db3 = assign3740_e3269_d_b3;

        let (assign3750_e3278, assign3750_e3278_d_n0, assign3750_e3278_d_n1, assign3750_e3278_d_n2, assign3750_e3278_d_n3, assign3750_e3278_d_n4, assign3750_e3278_d_n5, assign3750_e3278_d_n6, assign3750_e3278_d_n7, assign3750_e3278_d_n8, assign3750_e3278_d_n9, assign3750_e3278_d_b0, assign3750_e3278_d_b1, assign3750_e3278_d_b2, assign3750_e3278_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3750_e3274: f64 = (var_temp).max(0.0);
        let assign3750_e3276: f64 = (assign3750_e3274).min(5.0);
        (assign3750_e3276, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn0 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn1 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn2 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn3 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn5 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_db0 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_db1 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_db2 } else { 0.0 } } else { 0.0 }, if assign3750_e3274 <= 5.0 { if var_temp >= 0.0 { var_temp_db3 } else { 0.0 } } else { 0.0 },)
    } else {
        (var_psce1edge_i, var_psce1edge_i_dn0, var_psce1edge_i_dn1, var_psce1edge_i_dn2, var_psce1edge_i_dn3, var_psce1edge_i_dn4, var_psce1edge_i_dn5, var_psce1edge_i_dn6, var_psce1edge_i_dn7, var_psce1edge_i_dn8, var_psce1edge_i_dn9, var_psce1edge_i_db0, var_psce1edge_i_db1, var_psce1edge_i_db2, var_psce1edge_i_db3,)
    }
};
        var_psce1edge_i = assign3750_e3278;
        var_psce1edge_i_dn0 = assign3750_e3278_d_n0;
        var_psce1edge_i_dn1 = assign3750_e3278_d_n1;
        var_psce1edge_i_dn2 = assign3750_e3278_d_n2;
        var_psce1edge_i_dn3 = assign3750_e3278_d_n3;
        var_psce1edge_i_dn4 = assign3750_e3278_d_n4;
        var_psce1edge_i_dn5 = assign3750_e3278_d_n5;
        var_psce1edge_i_dn6 = assign3750_e3278_d_n6;
        var_psce1edge_i_dn7 = assign3750_e3278_d_n7;
        var_psce1edge_i_dn8 = assign3750_e3278_d_n8;
        var_psce1edge_i_dn9 = assign3750_e3278_d_n9;
        var_psce1edge_i_db0 = assign3750_e3278_d_b0;
        var_psce1edge_i_db1 = assign3750_e3278_d_b1;
        var_psce1edge_i_db2 = assign3750_e3278_d_b2;
        var_psce1edge_i_db3 = assign3750_e3278_d_b3;

        let (assign3760_e3289, assign3760_e3289_d_n0, assign3760_e3289_d_n1, assign3760_e3289_d_n2, assign3760_e3289_d_n3, assign3760_e3289_d_n4, assign3760_e3289_d_n5, assign3760_e3289_d_n6, assign3760_e3289_d_n7, assign3760_e3289_d_n8, assign3760_e3289_d_n9, assign3760_e3289_d_b0, assign3760_e3289_d_b1, assign3760_e3289_d_b2, assign3760_e3289_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3760_e3283: f64 = (p.p371 * var_psce1edge_i);
        let assign3760_e3285: f64 = (assign3760_e3283 * var_tox2_i);
        let assign3760_e3287: f64 = (assign3760_e3285 / var_tox1_i);
        (assign3760_e3287, (((p.p371 * var_psce1edge_i_dn0) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn1) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn2) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn3) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn5) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_dn9) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_db0) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_db1) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_db2) * var_tox2_i) / var_tox1_i), (((p.p371 * var_psce1edge_i_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_psce2edge_i, var_psce2edge_i_dn0, var_psce2edge_i_dn1, var_psce2edge_i_dn2, var_psce2edge_i_dn3, var_psce2edge_i_dn4, var_psce2edge_i_dn5, var_psce2edge_i_dn6, var_psce2edge_i_dn7, var_psce2edge_i_dn8, var_psce2edge_i_dn9, var_psce2edge_i_db0, var_psce2edge_i_db1, var_psce2edge_i_db2, var_psce2edge_i_db3,)
    }
};
        var_psce2edge_i = assign3760_e3289;
        var_psce2edge_i_dn0 = assign3760_e3289_d_n0;
        var_psce2edge_i_dn1 = assign3760_e3289_d_n1;
        var_psce2edge_i_dn2 = assign3760_e3289_d_n2;
        var_psce2edge_i_dn3 = assign3760_e3289_d_n3;
        var_psce2edge_i_dn4 = assign3760_e3289_d_n4;
        var_psce2edge_i_dn5 = assign3760_e3289_d_n5;
        var_psce2edge_i_dn6 = assign3760_e3289_d_n6;
        var_psce2edge_i_dn7 = assign3760_e3289_d_n7;
        var_psce2edge_i_dn8 = assign3760_e3289_d_n8;
        var_psce2edge_i_dn9 = assign3760_e3289_d_n9;
        var_psce2edge_i_db0 = assign3760_e3289_d_b0;
        var_psce2edge_i_db1 = assign3760_e3289_d_b1;
        var_psce2edge_i_db2 = assign3760_e3289_d_b2;
        var_psce2edge_i_db3 = assign3760_e3289_d_b3;

        let (assign3770_e3302, assign3770_e3302_d_n0, assign3770_e3302_d_n1, assign3770_e3302_d_n2, assign3770_e3302_d_n3, assign3770_e3302_d_n4, assign3770_e3302_d_n5, assign3770_e3302_d_n6, assign3770_e3302_d_n7, assign3770_e3302_d_n8, assign3770_e3302_d_n9, assign3770_e3302_d_b0, assign3770_e3302_d_b1, assign3770_e3302_d_b2, assign3770_e3302_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3770_e3294: f64 = (var_lambda_le).powf(p.p373);
        let assign3770_e3298: f64 = (p.p374 * var_iwe);
        let assign3770_e3299: f64 = (1.0 + assign3770_e3298);
        let assign3770_e3300: f64 = (assign3770_e3294 * assign3770_e3299);
        (assign3770_e3300, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3770_e3302;
        var_temp_dn0 = assign3770_e3302_d_n0;
        var_temp_dn1 = assign3770_e3302_d_n1;
        var_temp_dn2 = assign3770_e3302_d_n2;
        var_temp_dn3 = assign3770_e3302_d_n3;
        var_temp_dn4 = assign3770_e3302_d_n4;
        var_temp_dn5 = assign3770_e3302_d_n5;
        var_temp_dn6 = assign3770_e3302_d_n6;
        var_temp_dn7 = assign3770_e3302_d_n7;
        var_temp_dn8 = assign3770_e3302_d_n8;
        var_temp_dn9 = assign3770_e3302_d_n9;
        var_temp_db0 = assign3770_e3302_d_b0;
        var_temp_db1 = assign3770_e3302_d_b1;
        var_temp_db2 = assign3770_e3302_d_b2;
        var_temp_db3 = assign3770_e3302_d_b3;

        let (assign3780_e3309, assign3780_e3309_d_n0, assign3780_e3309_d_n1, assign3780_e3309_d_n2, assign3780_e3309_d_n3, assign3780_e3309_d_n4, assign3780_e3309_d_n5, assign3780_e3309_d_n6, assign3780_e3309_d_n7, assign3780_e3309_d_n8, assign3780_e3309_d_n9, assign3780_e3309_d_b0, assign3780_e3309_d_b1, assign3780_e3309_d_b2, assign3780_e3309_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3780_e3307: f64 = (p.p372 * var_temp);
        (assign3780_e3307, (p.p372 * var_temp_dn0), (p.p372 * var_temp_dn1), (p.p372 * var_temp_dn2), (p.p372 * var_temp_dn3), (p.p372 * var_temp_dn4), (p.p372 * var_temp_dn5), (p.p372 * var_temp_dn6), (p.p372 * var_temp_dn7), (p.p372 * var_temp_dn8), (p.p372 * var_temp_dn9), (p.p372 * var_temp_db0), (p.p372 * var_temp_db1), (p.p372 * var_temp_db2), (p.p372 * var_temp_db3),)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3780_e3309;
        var_temp_dn0 = assign3780_e3309_d_n0;
        var_temp_dn1 = assign3780_e3309_d_n1;
        var_temp_dn2 = assign3780_e3309_d_n2;
        var_temp_dn3 = assign3780_e3309_d_n3;
        var_temp_dn4 = assign3780_e3309_d_n4;
        var_temp_dn5 = assign3780_e3309_d_n5;
        var_temp_dn6 = assign3780_e3309_d_n6;
        var_temp_dn7 = assign3780_e3309_d_n7;
        var_temp_dn8 = assign3780_e3309_d_n8;
        var_temp_dn9 = assign3780_e3309_d_n9;
        var_temp_db0 = assign3780_e3309_d_b0;
        var_temp_db1 = assign3780_e3309_d_b1;
        var_temp_db2 = assign3780_e3309_d_b2;
        var_temp_db3 = assign3780_e3309_d_b3;

        let (assign3790_e3316, assign3790_e3316_d_n0, assign3790_e3316_d_n1, assign3790_e3316_d_n2, assign3790_e3316_d_n3, assign3790_e3316_d_n4, assign3790_e3316_d_n5, assign3790_e3316_d_n6, assign3790_e3316_d_n7, assign3790_e3316_d_n8, assign3790_e3316_d_n9, assign3790_e3316_d_b0, assign3790_e3316_d_b1, assign3790_e3316_d_b2, assign3790_e3316_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3790_e3314: f64 = (var_temp).max(0.0);
        (assign3790_e3314, if var_temp >= 0.0 { var_temp_dn0 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn1 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn2 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn3 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn4 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn5 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn6 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn7 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn8 } else { 0.0 }, if var_temp >= 0.0 { var_temp_dn9 } else { 0.0 }, if var_temp >= 0.0 { var_temp_db0 } else { 0.0 }, if var_temp >= 0.0 { var_temp_db1 } else { 0.0 }, if var_temp >= 0.0 { var_temp_db2 } else { 0.0 }, if var_temp >= 0.0 { var_temp_db3 } else { 0.0 },)
    } else {
        (var_cf1edge_i, var_cf1edge_i_dn0, var_cf1edge_i_dn1, var_cf1edge_i_dn2, var_cf1edge_i_dn3, var_cf1edge_i_dn4, var_cf1edge_i_dn5, var_cf1edge_i_dn6, var_cf1edge_i_dn7, var_cf1edge_i_dn8, var_cf1edge_i_dn9, var_cf1edge_i_db0, var_cf1edge_i_db1, var_cf1edge_i_db2, var_cf1edge_i_db3,)
    }
};
        var_cf1edge_i = assign3790_e3316;
        var_cf1edge_i_dn0 = assign3790_e3316_d_n0;
        var_cf1edge_i_dn1 = assign3790_e3316_d_n1;
        var_cf1edge_i_dn2 = assign3790_e3316_d_n2;
        var_cf1edge_i_dn3 = assign3790_e3316_d_n3;
        var_cf1edge_i_dn4 = assign3790_e3316_d_n4;
        var_cf1edge_i_dn5 = assign3790_e3316_d_n5;
        var_cf1edge_i_dn6 = assign3790_e3316_d_n6;
        var_cf1edge_i_dn7 = assign3790_e3316_d_n7;
        var_cf1edge_i_dn8 = assign3790_e3316_d_n8;
        var_cf1edge_i_dn9 = assign3790_e3316_d_n9;
        var_cf1edge_i_db0 = assign3790_e3316_d_b0;
        var_cf1edge_i_db1 = assign3790_e3316_d_b1;
        var_cf1edge_i_db2 = assign3790_e3316_d_b2;
        var_cf1edge_i_db3 = assign3790_e3316_d_b3;

        let (assign3800_e3327, assign3800_e3327_d_n0, assign3800_e3327_d_n1, assign3800_e3327_d_n2, assign3800_e3327_d_n3, assign3800_e3327_d_n4, assign3800_e3327_d_n5, assign3800_e3327_d_n6, assign3800_e3327_d_n7, assign3800_e3327_d_n8, assign3800_e3327_d_n9, assign3800_e3327_d_b0, assign3800_e3327_d_b1, assign3800_e3327_d_b2, assign3800_e3327_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3800_e3321: f64 = (p.p375 * var_cf1edge_i);
        let assign3800_e3323: f64 = (assign3800_e3321 * var_tox2_i);
        let assign3800_e3325: f64 = (assign3800_e3323 / var_tox1_i);
        (assign3800_e3325, (((p.p375 * var_cf1edge_i_dn0) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn1) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn2) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn3) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn4) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn5) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn6) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn7) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn8) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_dn9) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_db0) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_db1) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_db2) * var_tox2_i) / var_tox1_i), (((p.p375 * var_cf1edge_i_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cf2edge_i, var_cf2edge_i_dn0, var_cf2edge_i_dn1, var_cf2edge_i_dn2, var_cf2edge_i_dn3, var_cf2edge_i_dn4, var_cf2edge_i_dn5, var_cf2edge_i_dn6, var_cf2edge_i_dn7, var_cf2edge_i_dn8, var_cf2edge_i_dn9, var_cf2edge_i_db0, var_cf2edge_i_db1, var_cf2edge_i_db2, var_cf2edge_i_db3,)
    }
};
        var_cf2edge_i = assign3800_e3327;
        var_cf2edge_i_dn0 = assign3800_e3327_d_n0;
        var_cf2edge_i_dn1 = assign3800_e3327_d_n1;
        var_cf2edge_i_dn2 = assign3800_e3327_d_n2;
        var_cf2edge_i_dn3 = assign3800_e3327_d_n3;
        var_cf2edge_i_dn4 = assign3800_e3327_d_n4;
        var_cf2edge_i_dn5 = assign3800_e3327_d_n5;
        var_cf2edge_i_dn6 = assign3800_e3327_d_n6;
        var_cf2edge_i_dn7 = assign3800_e3327_d_n7;
        var_cf2edge_i_dn8 = assign3800_e3327_d_n8;
        var_cf2edge_i_dn9 = assign3800_e3327_d_n9;
        var_cf2edge_i_db0 = assign3800_e3327_d_b0;
        var_cf2edge_i_db1 = assign3800_e3327_d_b1;
        var_cf2edge_i_db2 = assign3800_e3327_d_b2;
        var_cf2edge_i_db3 = assign3800_e3327_d_b3;

        let (assign3810_e3332,) = {
    if (var_guard83 == 0.0) {
        (p.p376,)
    } else {
        (var_cfdedge_i,)
    }
};
        var_cfdedge_i = assign3810_e3332;

        let (assign3820_e3351, assign3820_e3351_d_n0, assign3820_e3351_d_n1, assign3820_e3351_d_n2, assign3820_e3351_d_n3, assign3820_e3351_d_n4, assign3820_e3351_d_n5, assign3820_e3351_d_n6, assign3820_e3351_d_n7, assign3820_e3351_d_n8, assign3820_e3351_d_n9, assign3820_e3351_d_b0, assign3820_e3351_d_b1, assign3820_e3351_d_b2, assign3820_e3351_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3820_e3338: f64 = (p.p377 * p.p378);
        let assign3820_e3340: f64 = (assign3820_e3338 / var_le);
        let assign3820_e3343: f64 = (-var_le);
        let assign3820_e3345: f64 = (assign3820_e3343 / p.p378);
        let assign3820_e3346: f64 = (assign3820_e3345).exp();
        let assign3820_e3347: f64 = (1.0 - assign3820_e3346);
        let assign3820_e3348: f64 = (assign3820_e3340 * assign3820_e3347);
        let assign3820_e3349: f64 = (1.0 + assign3820_e3348);
        (assign3820_e3349, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3820_e3351;
        var_temp_dn0 = assign3820_e3351_d_n0;
        var_temp_dn1 = assign3820_e3351_d_n1;
        var_temp_dn2 = assign3820_e3351_d_n2;
        var_temp_dn3 = assign3820_e3351_d_n3;
        var_temp_dn4 = assign3820_e3351_d_n4;
        var_temp_dn5 = assign3820_e3351_d_n5;
        var_temp_dn6 = assign3820_e3351_d_n6;
        var_temp_dn7 = assign3820_e3351_d_n7;
        var_temp_dn8 = assign3820_e3351_d_n8;
        var_temp_dn9 = assign3820_e3351_d_n9;
        var_temp_db0 = assign3820_e3351_d_b0;
        var_temp_db1 = assign3820_e3351_d_b1;
        var_temp_db2 = assign3820_e3351_d_b2;
        var_temp_db3 = assign3820_e3351_d_b3;

        let (assign3830_e3358, assign3830_e3358_d_n0, assign3830_e3358_d_n1, assign3830_e3358_d_n2, assign3830_e3358_d_n3, assign3830_e3358_d_n4, assign3830_e3358_d_n5, assign3830_e3358_d_n6, assign3830_e3358_d_n7, assign3830_e3358_d_n8, assign3830_e3358_d_n9, assign3830_e3358_d_b0, assign3830_e3358_d_b1, assign3830_e3358_d_b2, assign3830_e3358_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3830_e3356: f64 = (var_temp).max(1e-15);
        (assign3830_e3356, if var_temp >= 1e-15 { var_temp_dn0 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn1 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn2 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn3 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn4 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn5 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn6 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn7 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn8 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_dn9 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_db0 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_db1 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_db2 } else { 0.0 }, if var_temp >= 1e-15 { var_temp_db3 } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign3830_e3358;
        var_temp_dn0 = assign3830_e3358_d_n0;
        var_temp_dn1 = assign3830_e3358_d_n1;
        var_temp_dn2 = assign3830_e3358_d_n2;
        var_temp_dn3 = assign3830_e3358_d_n3;
        var_temp_dn4 = assign3830_e3358_d_n4;
        var_temp_dn5 = assign3830_e3358_d_n5;
        var_temp_dn6 = assign3830_e3358_d_n6;
        var_temp_dn7 = assign3830_e3358_d_n7;
        var_temp_dn8 = assign3830_e3358_d_n8;
        var_temp_dn9 = assign3830_e3358_d_n9;
        var_temp_db0 = assign3830_e3358_d_b0;
        var_temp_db1 = assign3830_e3358_d_b1;
        var_temp_db2 = assign3830_e3358_d_b2;
        var_temp_db3 = assign3830_e3358_d_b3;

        let (assign3840_e3375, assign3840_e3375_d_n0, assign3840_e3375_d_n1, assign3840_e3375_d_n2, assign3840_e3375_d_n3, assign3840_e3375_d_n4, assign3840_e3375_d_n5, assign3840_e3375_d_n6, assign3840_e3375_d_n7, assign3840_e3375_d_n8, assign3840_e3375_d_n9, assign3840_e3375_d_b0, assign3840_e3375_d_b1, assign3840_e3375_d_b2, assign3840_e3375_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3840_e3363: f64 = (p.p240 * var_we_edge);
        let assign3840_e3366: f64 = (var_temp * var_le);
        let assign3840_e3367: f64 = (assign3840_e3363 / assign3840_e3366);
        let assign3840_e3371: f64 = (p.p379 * var_iwe);
        let assign3840_e3372: f64 = (1.0 + assign3840_e3371);
        let assign3840_e3373: f64 = (assign3840_e3367 * assign3840_e3372);
        (assign3840_e3373, ((-((assign3840_e3363 * (var_temp_dn0 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn1 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn2 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn3 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn4 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn5 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn6 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn7 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn8 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_dn9 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_db0 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_db1 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_db2 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372), ((-((assign3840_e3363 * (var_temp_db3 * var_le)) / (assign3840_e3366 * assign3840_e3366))) * assign3840_e3372),)
    } else {
        (var_betnedge_t, var_betnedge_t_dn0, var_betnedge_t_dn1, var_betnedge_t_dn2, var_betnedge_t_dn3, var_betnedge_t_dn4, var_betnedge_t_dn5, var_betnedge_t_dn6, var_betnedge_t_dn7, var_betnedge_t_dn8, var_betnedge_t_dn9, var_betnedge_t_db0, var_betnedge_t_db1, var_betnedge_t_db2, var_betnedge_t_db3,)
    }
};
        var_betnedge_t = assign3840_e3375;
        var_betnedge_t_dn0 = assign3840_e3375_d_n0;
        var_betnedge_t_dn1 = assign3840_e3375_d_n1;
        var_betnedge_t_dn2 = assign3840_e3375_d_n2;
        var_betnedge_t_dn3 = assign3840_e3375_d_n3;
        var_betnedge_t_dn4 = assign3840_e3375_d_n4;
        var_betnedge_t_dn5 = assign3840_e3375_d_n5;
        var_betnedge_t_dn6 = assign3840_e3375_d_n6;
        var_betnedge_t_dn7 = assign3840_e3375_d_n7;
        var_betnedge_t_dn8 = assign3840_e3375_d_n8;
        var_betnedge_t_dn9 = assign3840_e3375_d_n9;
        var_betnedge_t_db0 = assign3840_e3375_d_b0;
        var_betnedge_t_db1 = assign3840_e3375_d_b1;
        var_betnedge_t_db2 = assign3840_e3375_d_b2;
        var_betnedge_t_db3 = assign3840_e3375_d_b3;

        let (assign3850_e3394,) = {
    if (var_guard83 == 0.0) {
        let assign3850_e3381: f64 = (p.p381 * var_ile);
        let assign3850_e3382: f64 = (p.p380 + assign3850_e3381);
        let assign3850_e3385: f64 = (p.p382 * var_iwe);
        let assign3850_e3386: f64 = (assign3850_e3382 + assign3850_e3385);
        let assign3850_e3389: f64 = (p.p383 * var_ile);
        let assign3850_e3391: f64 = (assign3850_e3389 * var_iwe);
        let assign3850_e3392: f64 = (assign3850_e3386 + assign3850_e3391);
        (assign3850_e3392,)
    } else {
        (var_stbetedge_i,)
    }
};
        var_stbetedge_i = assign3850_e3394;

        let (assign3860_e3401,) = {
    if (var_guard83 == 0.0) {
        let assign3860_e3399: f64 = (var_wecv * var_lecv);
        (assign3860_e3399,)
    } else {
        (var_areaq_i,)
    }
};
        var_areaq_i = assign3860_e3401;

        let (assign3870_e3410, assign3870_e3410_d_n0, assign3870_e3410_d_n1, assign3870_e3410_d_n2, assign3870_e3410_d_n3, assign3870_e3410_d_n4, assign3870_e3410_d_n5, assign3870_e3410_d_n6, assign3870_e3410_d_n7, assign3870_e3410_d_n8, assign3870_e3410_d_n9, assign3870_e3410_d_b0, assign3870_e3410_d_b1, assign3870_e3410_d_b2, assign3870_e3410_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3870_e3407: f64 = (p.p393 * var_lphy);
        let assign3870_e3408: f64 = (p.p392 + assign3870_e3407);
        (assign3870_e3408, (p.p393 * var_lphy_dn0), (p.p393 * var_lphy_dn1), (p.p393 * var_lphy_dn2), (p.p393 * var_lphy_dn3), (p.p393 * var_lphy_dn4), (p.p393 * var_lphy_dn5), (p.p393 * var_lphy_dn6), (p.p393 * var_lphy_dn7), (p.p393 * var_lphy_dn8), (p.p393 * var_lphy_dn9), (p.p393 * var_lphy_db0), (p.p393 * var_lphy_db1), (p.p393 * var_lphy_db2), (p.p393 * var_lphy_db3),)
    } else {
        (var_cgbov_p, var_cgbov_p_dn0, var_cgbov_p_dn1, var_cgbov_p_dn2, var_cgbov_p_dn3, var_cgbov_p_dn4, var_cgbov_p_dn5, var_cgbov_p_dn6, var_cgbov_p_dn7, var_cgbov_p_dn8, var_cgbov_p_dn9, var_cgbov_p_db0, var_cgbov_p_db1, var_cgbov_p_db2, var_cgbov_p_db3,)
    }
};
        var_cgbov_p = assign3870_e3410;
        var_cgbov_p_dn0 = assign3870_e3410_d_n0;
        var_cgbov_p_dn1 = assign3870_e3410_d_n1;
        var_cgbov_p_dn2 = assign3870_e3410_d_n2;
        var_cgbov_p_dn3 = assign3870_e3410_d_n3;
        var_cgbov_p_dn4 = assign3870_e3410_d_n4;
        var_cgbov_p_dn5 = assign3870_e3410_d_n5;
        var_cgbov_p_dn6 = assign3870_e3410_d_n6;
        var_cgbov_p_dn7 = assign3870_e3410_d_n7;
        var_cgbov_p_dn8 = assign3870_e3410_d_n8;
        var_cgbov_p_dn9 = assign3870_e3410_d_n9;
        var_cgbov_p_db0 = assign3870_e3410_d_b0;
        var_cgbov_p_db1 = assign3870_e3410_d_b1;
        var_cgbov_p_db2 = assign3870_e3410_d_b2;
        var_cgbov_p_db3 = assign3870_e3410_d_b3;

        *var_a1_i_slot = var_a1_i;
        *var_a2_t_slot = var_a2_t;
        *var_a3_i_slot = var_a3_i;
        *var_a3_p_slot = var_a3_p;
        *var_areaq_i_slot = var_areaq_i;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_db0_slot = var_betnedge_t_db0;
        *var_betnedge_t_db1_slot = var_betnedge_t_db1;
        *var_betnedge_t_db2_slot = var_betnedge_t_db2;
        *var_betnedge_t_db3_slot = var_betnedge_t_db3;
        *var_betnedge_t_dn0_slot = var_betnedge_t_dn0;
        *var_betnedge_t_dn1_slot = var_betnedge_t_dn1;
        *var_betnedge_t_dn2_slot = var_betnedge_t_dn2;
        *var_betnedge_t_dn3_slot = var_betnedge_t_dn3;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_dn5_slot = var_betnedge_t_dn5;
        *var_betnedge_t_dn6_slot = var_betnedge_t_dn6;
        *var_betnedge_t_dn7_slot = var_betnedge_t_dn7;
        *var_betnedge_t_dn8_slot = var_betnedge_t_dn8;
        *var_betnedge_t_dn9_slot = var_betnedge_t_dn9;
        *var_cf1edge_i_slot = var_cf1edge_i;
        *var_cf1edge_i_db0_slot = var_cf1edge_i_db0;
        *var_cf1edge_i_db1_slot = var_cf1edge_i_db1;
        *var_cf1edge_i_db2_slot = var_cf1edge_i_db2;
        *var_cf1edge_i_db3_slot = var_cf1edge_i_db3;
        *var_cf1edge_i_dn0_slot = var_cf1edge_i_dn0;
        *var_cf1edge_i_dn1_slot = var_cf1edge_i_dn1;
        *var_cf1edge_i_dn2_slot = var_cf1edge_i_dn2;
        *var_cf1edge_i_dn3_slot = var_cf1edge_i_dn3;
        *var_cf1edge_i_dn4_slot = var_cf1edge_i_dn4;
        *var_cf1edge_i_dn5_slot = var_cf1edge_i_dn5;
        *var_cf1edge_i_dn6_slot = var_cf1edge_i_dn6;
        *var_cf1edge_i_dn7_slot = var_cf1edge_i_dn7;
        *var_cf1edge_i_dn8_slot = var_cf1edge_i_dn8;
        *var_cf1edge_i_dn9_slot = var_cf1edge_i_dn9;
        *var_cf2edge_i_slot = var_cf2edge_i;
        *var_cf2edge_i_db0_slot = var_cf2edge_i_db0;
        *var_cf2edge_i_db1_slot = var_cf2edge_i_db1;
        *var_cf2edge_i_db2_slot = var_cf2edge_i_db2;
        *var_cf2edge_i_db3_slot = var_cf2edge_i_db3;
        *var_cf2edge_i_dn0_slot = var_cf2edge_i_dn0;
        *var_cf2edge_i_dn1_slot = var_cf2edge_i_dn1;
        *var_cf2edge_i_dn2_slot = var_cf2edge_i_dn2;
        *var_cf2edge_i_dn3_slot = var_cf2edge_i_dn3;
        *var_cf2edge_i_dn4_slot = var_cf2edge_i_dn4;
        *var_cf2edge_i_dn5_slot = var_cf2edge_i_dn5;
        *var_cf2edge_i_dn6_slot = var_cf2edge_i_dn6;
        *var_cf2edge_i_dn7_slot = var_cf2edge_i_dn7;
        *var_cf2edge_i_dn8_slot = var_cf2edge_i_dn8;
        *var_cf2edge_i_dn9_slot = var_cf2edge_i_dn9;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_db0_slot = var_cgbov_p_db0;
        *var_cgbov_p_db1_slot = var_cgbov_p_db1;
        *var_cgbov_p_db2_slot = var_cgbov_p_db2;
        *var_cgbov_p_db3_slot = var_cgbov_p_db3;
        *var_cgbov_p_dn0_slot = var_cgbov_p_dn0;
        *var_cgbov_p_dn1_slot = var_cgbov_p_dn1;
        *var_cgbov_p_dn2_slot = var_cgbov_p_dn2;
        *var_cgbov_p_dn3_slot = var_cgbov_p_dn3;
        *var_cgbov_p_dn4_slot = var_cgbov_p_dn4;
        *var_cgbov_p_dn5_slot = var_cgbov_p_dn5;
        *var_cgbov_p_dn6_slot = var_cgbov_p_dn6;
        *var_cgbov_p_dn7_slot = var_cgbov_p_dn7;
        *var_cgbov_p_dn8_slot = var_cgbov_p_dn8;
        *var_cgbov_p_dn9_slot = var_cgbov_p_dn9;
        *var_cic1edge_i_slot = var_cic1edge_i;
        *var_cic2edge_i_slot = var_cic2edge_i;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_psce1edge_i_slot = var_psce1edge_i;
        *var_psce1edge_i_db0_slot = var_psce1edge_i_db0;
        *var_psce1edge_i_db1_slot = var_psce1edge_i_db1;
        *var_psce1edge_i_db2_slot = var_psce1edge_i_db2;
        *var_psce1edge_i_db3_slot = var_psce1edge_i_db3;
        *var_psce1edge_i_dn0_slot = var_psce1edge_i_dn0;
        *var_psce1edge_i_dn1_slot = var_psce1edge_i_dn1;
        *var_psce1edge_i_dn2_slot = var_psce1edge_i_dn2;
        *var_psce1edge_i_dn3_slot = var_psce1edge_i_dn3;
        *var_psce1edge_i_dn4_slot = var_psce1edge_i_dn4;
        *var_psce1edge_i_dn5_slot = var_psce1edge_i_dn5;
        *var_psce1edge_i_dn6_slot = var_psce1edge_i_dn6;
        *var_psce1edge_i_dn7_slot = var_psce1edge_i_dn7;
        *var_psce1edge_i_dn8_slot = var_psce1edge_i_dn8;
        *var_psce1edge_i_dn9_slot = var_psce1edge_i_dn9;
        *var_psce2edge_i_slot = var_psce2edge_i;
        *var_psce2edge_i_db0_slot = var_psce2edge_i_db0;
        *var_psce2edge_i_db1_slot = var_psce2edge_i_db1;
        *var_psce2edge_i_db2_slot = var_psce2edge_i_db2;
        *var_psce2edge_i_db3_slot = var_psce2edge_i_db3;
        *var_psce2edge_i_dn0_slot = var_psce2edge_i_dn0;
        *var_psce2edge_i_dn1_slot = var_psce2edge_i_dn1;
        *var_psce2edge_i_dn2_slot = var_psce2edge_i_dn2;
        *var_psce2edge_i_dn3_slot = var_psce2edge_i_dn3;
        *var_psce2edge_i_dn4_slot = var_psce2edge_i_dn4;
        *var_psce2edge_i_dn5_slot = var_psce2edge_i_dn5;
        *var_psce2edge_i_dn6_slot = var_psce2edge_i_dn6;
        *var_psce2edge_i_dn7_slot = var_psce2edge_i_dn7;
        *var_psce2edge_i_dn8_slot = var_psce2edge_i_dn8;
        *var_psce2edge_i_dn9_slot = var_psce2edge_i_dn9;
        *var_sta2_i_slot = var_sta2_i;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_temp_slot = var_temp;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_vfb1edge_t_slot = var_vfb1edge_t;
        *var_vfb1edge_t_db0_slot = var_vfb1edge_t_db0;
        *var_vfb1edge_t_db1_slot = var_vfb1edge_t_db1;
        *var_vfb1edge_t_db2_slot = var_vfb1edge_t_db2;
        *var_vfb1edge_t_db3_slot = var_vfb1edge_t_db3;
        *var_vfb1edge_t_dn0_slot = var_vfb1edge_t_dn0;
        *var_vfb1edge_t_dn1_slot = var_vfb1edge_t_dn1;
        *var_vfb1edge_t_dn2_slot = var_vfb1edge_t_dn2;
        *var_vfb1edge_t_dn3_slot = var_vfb1edge_t_dn3;
        *var_vfb1edge_t_dn4_slot = var_vfb1edge_t_dn4;
        *var_vfb1edge_t_dn5_slot = var_vfb1edge_t_dn5;
        *var_vfb1edge_t_dn6_slot = var_vfb1edge_t_dn6;
        *var_vfb1edge_t_dn7_slot = var_vfb1edge_t_dn7;
        *var_vfb1edge_t_dn8_slot = var_vfb1edge_t_dn8;
        *var_vfb1edge_t_dn9_slot = var_vfb1edge_t_dn9;
        *var_vfb2edge_t_slot = var_vfb2edge_t;
        *var_we_edge_slot = var_we_edge;
    }

    pub(super) fn stamp_transient_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_alp_i: f64,
        var_ax_i: f64,
        var_cf1_t: f64,
        var_cf1_t_db0: f64,
        var_cf1_t_db1: f64,
        var_cf1_t_db2: f64,
        var_cf1_t_db3: f64,
        var_cf1_t_dn0: f64,
        var_cf1_t_dn1: f64,
        var_cf1_t_dn2: f64,
        var_cf1_t_dn3: f64,
        var_cf1_t_dn4: f64,
        var_cf1_t_dn5: f64,
        var_cf1_t_dn6: f64,
        var_cf1_t_dn7: f64,
        var_cf1_t_dn8: f64,
        var_cf1_t_dn9: f64,
        var_cf2_t: f64,
        var_cf2_t_db0: f64,
        var_cf2_t_db1: f64,
        var_cf2_t_db2: f64,
        var_cf2_t_db3: f64,
        var_cf2_t_dn0: f64,
        var_cf2_t_dn1: f64,
        var_cf2_t_dn2: f64,
        var_cf2_t_dn3: f64,
        var_cf2_t_dn4: f64,
        var_cf2_t_dn5: f64,
        var_cf2_t_dn6: f64,
        var_cf2_t_dn7: f64,
        var_cf2_t_dn8: f64,
        var_cf2_t_dn9: f64,
        var_cf_p: f64,
        var_cf_p_db0: f64,
        var_cf_p_db1: f64,
        var_cf_p_db2: f64,
        var_cf_p_db3: f64,
        var_cf_p_dn0: f64,
        var_cf_p_dn1: f64,
        var_cf_p_dn2: f64,
        var_cf_p_dn3: f64,
        var_cf_p_dn4: f64,
        var_cf_p_dn5: f64,
        var_cf_p_dn6: f64,
        var_cf_p_dn7: f64,
        var_cf_p_dn8: f64,
        var_cf_p_dn9: f64,
        var_cgbov_p: f64,
        var_cgbov_p_db0: f64,
        var_cgbov_p_db1: f64,
        var_cgbov_p_db2: f64,
        var_cgbov_p_db3: f64,
        var_cgbov_p_dn0: f64,
        var_cgbov_p_dn1: f64,
        var_cgbov_p_dn2: f64,
        var_cgbov_p_dn3: f64,
        var_cgbov_p_dn4: f64,
        var_cgbov_p_dn5: f64,
        var_cgbov_p_dn6: f64,
        var_cgbov_p_dn7: f64,
        var_cgbov_p_dn8: f64,
        var_cgbov_p_dn9: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_psce1_i: f64,
        var_psce2_i: f64,
        var_thesat_p: f64,
        var_thesat_p_db0: f64,
        var_thesat_p_db1: f64,
        var_thesat_p_db2: f64,
        var_thesat_p_db3: f64,
        var_thesat_p_dn0: f64,
        var_thesat_p_dn1: f64,
        var_thesat_p_dn2: f64,
        var_thesat_p_dn3: f64,
        var_thesat_p_dn4: f64,
        var_thesat_p_dn5: f64,
        var_thesat_p_dn6: f64,
        var_thesat_p_dn7: f64,
        var_thesat_p_dn8: f64,
        var_thesat_p_dn9: f64,
        var_thesat_t: f64,
        var_thesat_t_db0: f64,
        var_thesat_t_db1: f64,
        var_thesat_t_db2: f64,
        var_thesat_t_db3: f64,
        var_thesat_t_dn0: f64,
        var_thesat_t_dn1: f64,
        var_thesat_t_dn2: f64,
        var_thesat_t_dn3: f64,
        var_thesat_t_dn4: f64,
        var_thesat_t_dn5: f64,
        var_thesat_t_dn6: f64,
        var_thesat_t_dn7: f64,
        var_thesat_t_dn8: f64,
        var_thesat_t_dn9: f64,
        var_vfb1_t: f64,
        var_vfb1_t_db0: f64,
        var_vfb1_t_db1: f64,
        var_vfb1_t_db2: f64,
        var_vfb1_t_db3: f64,
        var_vfb1_t_dn0: f64,
        var_vfb1_t_dn1: f64,
        var_vfb1_t_dn2: f64,
        var_vfb1_t_dn3: f64,
        var_vfb1_t_dn4: f64,
        var_vfb1_t_dn5: f64,
        var_vfb1_t_dn6: f64,
        var_vfb1_t_dn7: f64,
        var_vfb1_t_dn8: f64,
        var_vfb1_t_dn9: f64,
        var_vfb2_t: f64,
        var_vfb2_t_db0: f64,
        var_vfb2_t_db1: f64,
        var_vfb2_t_db2: f64,
        var_vfb2_t_db3: f64,
        var_vfb2_t_dn0: f64,
        var_vfb2_t_dn1: f64,
        var_vfb2_t_dn2: f64,
        var_vfb2_t_dn3: f64,
        var_vfb2_t_dn4: f64,
        var_vfb2_t_dn5: f64,
        var_vfb2_t_dn6: f64,
        var_vfb2_t_dn7: f64,
        var_vfb2_t_dn8: f64,
        var_vfb2_t_dn9: f64,
        var_wecv: f64,
        var_wen: f64,
        var_alpac_i_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_db0_slot: &mut f64,
        var_cfac1_t_db1_slot: &mut f64,
        var_cfac1_t_db2_slot: &mut f64,
        var_cfac1_t_db3_slot: &mut f64,
        var_cfac1_t_dn0_slot: &mut f64,
        var_cfac1_t_dn1_slot: &mut f64,
        var_cfac1_t_dn2_slot: &mut f64,
        var_cfac1_t_dn3_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn5_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_db0_slot: &mut f64,
        var_cfac2_t_db1_slot: &mut f64,
        var_cfac2_t_db2_slot: &mut f64,
        var_cfac2_t_db3_slot: &mut f64,
        var_cfac2_t_dn0_slot: &mut f64,
        var_cfac2_t_dn1_slot: &mut f64,
        var_cfac2_t_dn2_slot: &mut f64,
        var_cfac2_t_dn3_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn5_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_db0_slot: &mut f64,
        var_cfac_p_db1_slot: &mut f64,
        var_cfac_p_db2_slot: &mut f64,
        var_cfac_p_db3_slot: &mut f64,
        var_cfac_p_dn0_slot: &mut f64,
        var_cfac_p_dn1_slot: &mut f64,
        var_cfac_p_dn2_slot: &mut f64,
        var_cfac_p_dn3_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn5_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_db0_slot: &mut f64,
        var_cgbov_i_db1_slot: &mut f64,
        var_cgbov_i_db2_slot: &mut f64,
        var_cgbov_i_db3_slot: &mut f64,
        var_cgbov_i_dn0_slot: &mut f64,
        var_cgbov_i_dn1_slot: &mut f64,
        var_cgbov_i_dn2_slot: &mut f64,
        var_cgbov_i_dn3_slot: &mut f64,
        var_cgbov_i_dn4_slot: &mut f64,
        var_cgbov_i_dn5_slot: &mut f64,
        var_cgbov_i_dn6_slot: &mut f64,
        var_cgbov_i_dn7_slot: &mut f64,
        var_cgbov_i_dn8_slot: &mut f64,
        var_cgbov_i_dn9_slot: &mut f64,
        var_fif_i_slot: &mut f64,
        var_fsceac_i_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_nsdac_i_slot: &mut f64,
        var_psceac1_i_slot: &mut f64,
        var_psceac2_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_db0_slot: &mut f64,
        var_thesatac_p_db1_slot: &mut f64,
        var_thesatac_p_db2_slot: &mut f64,
        var_thesatac_p_db3_slot: &mut f64,
        var_thesatac_p_dn0_slot: &mut f64,
        var_thesatac_p_dn1_slot: &mut f64,
        var_thesatac_p_dn2_slot: &mut f64,
        var_thesatac_p_dn3_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn5_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_db0_slot: &mut f64,
        var_thesatac_t_db1_slot: &mut f64,
        var_thesatac_t_db2_slot: &mut f64,
        var_thesatac_t_db3_slot: &mut f64,
        var_thesatac_t_dn0_slot: &mut f64,
        var_thesatac_t_dn1_slot: &mut f64,
        var_thesatac_t_dn2_slot: &mut f64,
        var_thesatac_t_dn3_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn5_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_db0_slot: &mut f64,
        var_vfbac1_t_db1_slot: &mut f64,
        var_vfbac1_t_db2_slot: &mut f64,
        var_vfbac1_t_db3_slot: &mut f64,
        var_vfbac1_t_dn0_slot: &mut f64,
        var_vfbac1_t_dn1_slot: &mut f64,
        var_vfbac1_t_dn2_slot: &mut f64,
        var_vfbac1_t_dn3_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn5_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_db0_slot: &mut f64,
        var_vfbac2_t_db1_slot: &mut f64,
        var_vfbac2_t_db2_slot: &mut f64,
        var_vfbac2_t_db3_slot: &mut f64,
        var_vfbac2_t_dn0_slot: &mut f64,
        var_vfbac2_t_dn1_slot: &mut f64,
        var_vfbac2_t_dn2_slot: &mut f64,
        var_vfbac2_t_dn3_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn5_slot: &mut f64,
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
        var_vfbbaco_i_slot: &mut f64,
        var_vfblbaco_i_slot: &mut f64,
    ) {
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_db0: f64 = *var_cfac1_t_db0_slot;
        let mut var_cfac1_t_db1: f64 = *var_cfac1_t_db1_slot;
        let mut var_cfac1_t_db2: f64 = *var_cfac1_t_db2_slot;
        let mut var_cfac1_t_db3: f64 = *var_cfac1_t_db3_slot;
        let mut var_cfac1_t_dn0: f64 = *var_cfac1_t_dn0_slot;
        let mut var_cfac1_t_dn1: f64 = *var_cfac1_t_dn1_slot;
        let mut var_cfac1_t_dn2: f64 = *var_cfac1_t_dn2_slot;
        let mut var_cfac1_t_dn3: f64 = *var_cfac1_t_dn3_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn5: f64 = *var_cfac1_t_dn5_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_db0: f64 = *var_cfac2_t_db0_slot;
        let mut var_cfac2_t_db1: f64 = *var_cfac2_t_db1_slot;
        let mut var_cfac2_t_db2: f64 = *var_cfac2_t_db2_slot;
        let mut var_cfac2_t_db3: f64 = *var_cfac2_t_db3_slot;
        let mut var_cfac2_t_dn0: f64 = *var_cfac2_t_dn0_slot;
        let mut var_cfac2_t_dn1: f64 = *var_cfac2_t_dn1_slot;
        let mut var_cfac2_t_dn2: f64 = *var_cfac2_t_dn2_slot;
        let mut var_cfac2_t_dn3: f64 = *var_cfac2_t_dn3_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn5: f64 = *var_cfac2_t_dn5_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_db0: f64 = *var_cfac_p_db0_slot;
        let mut var_cfac_p_db1: f64 = *var_cfac_p_db1_slot;
        let mut var_cfac_p_db2: f64 = *var_cfac_p_db2_slot;
        let mut var_cfac_p_db3: f64 = *var_cfac_p_db3_slot;
        let mut var_cfac_p_dn0: f64 = *var_cfac_p_dn0_slot;
        let mut var_cfac_p_dn1: f64 = *var_cfac_p_dn1_slot;
        let mut var_cfac_p_dn2: f64 = *var_cfac_p_dn2_slot;
        let mut var_cfac_p_dn3: f64 = *var_cfac_p_dn3_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn5: f64 = *var_cfac_p_dn5_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_db0: f64 = *var_cgbov_i_db0_slot;
        let mut var_cgbov_i_db1: f64 = *var_cgbov_i_db1_slot;
        let mut var_cgbov_i_db2: f64 = *var_cgbov_i_db2_slot;
        let mut var_cgbov_i_db3: f64 = *var_cgbov_i_db3_slot;
        let mut var_cgbov_i_dn0: f64 = *var_cgbov_i_dn0_slot;
        let mut var_cgbov_i_dn1: f64 = *var_cgbov_i_dn1_slot;
        let mut var_cgbov_i_dn2: f64 = *var_cgbov_i_dn2_slot;
        let mut var_cgbov_i_dn3: f64 = *var_cgbov_i_dn3_slot;
        let mut var_cgbov_i_dn4: f64 = *var_cgbov_i_dn4_slot;
        let mut var_cgbov_i_dn5: f64 = *var_cgbov_i_dn5_slot;
        let mut var_cgbov_i_dn6: f64 = *var_cgbov_i_dn6_slot;
        let mut var_cgbov_i_dn7: f64 = *var_cgbov_i_dn7_slot;
        let mut var_cgbov_i_dn8: f64 = *var_cgbov_i_dn8_slot;
        let mut var_cgbov_i_dn9: f64 = *var_cgbov_i_dn9_slot;
        let mut var_fif_i: f64 = *var_fif_i_slot;
        let mut var_fsceac_i: f64 = *var_fsceac_i_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_nsdac_i: f64 = *var_nsdac_i_slot;
        let mut var_psceac1_i: f64 = *var_psceac1_i_slot;
        let mut var_psceac2_i: f64 = *var_psceac2_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_db0: f64 = *var_thesatac_p_db0_slot;
        let mut var_thesatac_p_db1: f64 = *var_thesatac_p_db1_slot;
        let mut var_thesatac_p_db2: f64 = *var_thesatac_p_db2_slot;
        let mut var_thesatac_p_db3: f64 = *var_thesatac_p_db3_slot;
        let mut var_thesatac_p_dn0: f64 = *var_thesatac_p_dn0_slot;
        let mut var_thesatac_p_dn1: f64 = *var_thesatac_p_dn1_slot;
        let mut var_thesatac_p_dn2: f64 = *var_thesatac_p_dn2_slot;
        let mut var_thesatac_p_dn3: f64 = *var_thesatac_p_dn3_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn5: f64 = *var_thesatac_p_dn5_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_db0: f64 = *var_thesatac_t_db0_slot;
        let mut var_thesatac_t_db1: f64 = *var_thesatac_t_db1_slot;
        let mut var_thesatac_t_db2: f64 = *var_thesatac_t_db2_slot;
        let mut var_thesatac_t_db3: f64 = *var_thesatac_t_db3_slot;
        let mut var_thesatac_t_dn0: f64 = *var_thesatac_t_dn0_slot;
        let mut var_thesatac_t_dn1: f64 = *var_thesatac_t_dn1_slot;
        let mut var_thesatac_t_dn2: f64 = *var_thesatac_t_dn2_slot;
        let mut var_thesatac_t_dn3: f64 = *var_thesatac_t_dn3_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn5: f64 = *var_thesatac_t_dn5_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_db0: f64 = *var_vfbac1_t_db0_slot;
        let mut var_vfbac1_t_db1: f64 = *var_vfbac1_t_db1_slot;
        let mut var_vfbac1_t_db2: f64 = *var_vfbac1_t_db2_slot;
        let mut var_vfbac1_t_db3: f64 = *var_vfbac1_t_db3_slot;
        let mut var_vfbac1_t_dn0: f64 = *var_vfbac1_t_dn0_slot;
        let mut var_vfbac1_t_dn1: f64 = *var_vfbac1_t_dn1_slot;
        let mut var_vfbac1_t_dn2: f64 = *var_vfbac1_t_dn2_slot;
        let mut var_vfbac1_t_dn3: f64 = *var_vfbac1_t_dn3_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn5: f64 = *var_vfbac1_t_dn5_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_db0: f64 = *var_vfbac2_t_db0_slot;
        let mut var_vfbac2_t_db1: f64 = *var_vfbac2_t_db1_slot;
        let mut var_vfbac2_t_db2: f64 = *var_vfbac2_t_db2_slot;
        let mut var_vfbac2_t_db3: f64 = *var_vfbac2_t_db3_slot;
        let mut var_vfbac2_t_dn0: f64 = *var_vfbac2_t_dn0_slot;
        let mut var_vfbac2_t_dn1: f64 = *var_vfbac2_t_dn1_slot;
        let mut var_vfbac2_t_dn2: f64 = *var_vfbac2_t_dn2_slot;
        let mut var_vfbac2_t_dn3: f64 = *var_vfbac2_t_dn3_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn5: f64 = *var_vfbac2_t_dn5_slot;
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
        let mut var_vfbbaco_i: f64 = *var_vfbbaco_i_slot;
        let mut var_vfblbaco_i: f64 = *var_vfblbaco_i_slot;

        let (assign3880_e3417, assign3880_e3417_d_n0, assign3880_e3417_d_n1, assign3880_e3417_d_n2, assign3880_e3417_d_n3, assign3880_e3417_d_n4, assign3880_e3417_d_n5, assign3880_e3417_d_n6, assign3880_e3417_d_n7, assign3880_e3417_d_n8, assign3880_e3417_d_n9, assign3880_e3417_d_b0, assign3880_e3417_d_b1, assign3880_e3417_d_b2, assign3880_e3417_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign3880_e3415: f64 = (var_cgbov_p).max(0.0);
        (assign3880_e3415, if var_cgbov_p >= 0.0 { var_cgbov_p_dn0 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn1 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn2 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn3 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn4 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn5 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn6 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn7 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn8 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_dn9 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_db0 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_db1 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_db2 } else { 0.0 }, if var_cgbov_p >= 0.0 { var_cgbov_p_db3 } else { 0.0 },)
    } else {
        (var_cgbov_i, var_cgbov_i_dn0, var_cgbov_i_dn1, var_cgbov_i_dn2, var_cgbov_i_dn3, var_cgbov_i_dn4, var_cgbov_i_dn5, var_cgbov_i_dn6, var_cgbov_i_dn7, var_cgbov_i_dn8, var_cgbov_i_dn9, var_cgbov_i_db0, var_cgbov_i_db1, var_cgbov_i_db2, var_cgbov_i_db3,)
    }
};
        var_cgbov_i = assign3880_e3417;
        var_cgbov_i_dn0 = assign3880_e3417_d_n0;
        var_cgbov_i_dn1 = assign3880_e3417_d_n1;
        var_cgbov_i_dn2 = assign3880_e3417_d_n2;
        var_cgbov_i_dn3 = assign3880_e3417_d_n3;
        var_cgbov_i_dn4 = assign3880_e3417_d_n4;
        var_cgbov_i_dn5 = assign3880_e3417_d_n5;
        var_cgbov_i_dn6 = assign3880_e3417_d_n6;
        var_cgbov_i_dn7 = assign3880_e3417_d_n7;
        var_cgbov_i_dn8 = assign3880_e3417_d_n8;
        var_cgbov_i_dn9 = assign3880_e3417_d_n9;
        var_cgbov_i_db0 = assign3880_e3417_d_b0;
        var_cgbov_i_db1 = assign3880_e3417_d_b1;
        var_cgbov_i_db2 = assign3880_e3417_d_b2;
        var_cgbov_i_db3 = assign3880_e3417_d_b3;

        let (assign3890_e3424,) = {
    if (var_guard83 == 0.0) {
        let assign3890_e3422: f64 = (p.p394 * 1000000.0);
        (assign3890_e3422,)
    } else {
        (var_nsdac_i,)
    }
};
        var_nsdac_i = assign3890_e3424;

        let (assign3900_e3433,) = {
    if (var_guard83 == 0.0) {
        let assign3900_e3429: f64 = (p.p395 * var_wecv);
        let assign3900_e3431: f64 = (assign3900_e3429 / var_wen);
        (assign3900_e3431,)
    } else {
        (var_fif_i,)
    }
};
        var_fif_i = assign3900_e3433;

        let (assign3910_e3438,) = {
    if (var_guard83 == 0.0) {
        (p.p396,)
    } else {
        (var_fsceac_i,)
    }
};
        var_fsceac_i = assign3910_e3438;

        let (assign3920_e3443, assign3920_e3443_d_n0, assign3920_e3443_d_n1, assign3920_e3443_d_n2, assign3920_e3443_d_n3, assign3920_e3443_d_n4, assign3920_e3443_d_n5, assign3920_e3443_d_n6, assign3920_e3443_d_n7, assign3920_e3443_d_n8, assign3920_e3443_d_n9, assign3920_e3443_d_b0, assign3920_e3443_d_b1, assign3920_e3443_d_b2, assign3920_e3443_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_vfb1_t, var_vfb1_t_dn0, var_vfb1_t_dn1, var_vfb1_t_dn2, var_vfb1_t_dn3, var_vfb1_t_dn4, var_vfb1_t_dn5, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9, var_vfb1_t_db0, var_vfb1_t_db1, var_vfb1_t_db2, var_vfb1_t_db3,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign3920_e3443;
        var_vfbac1_t_dn0 = assign3920_e3443_d_n0;
        var_vfbac1_t_dn1 = assign3920_e3443_d_n1;
        var_vfbac1_t_dn2 = assign3920_e3443_d_n2;
        var_vfbac1_t_dn3 = assign3920_e3443_d_n3;
        var_vfbac1_t_dn4 = assign3920_e3443_d_n4;
        var_vfbac1_t_dn5 = assign3920_e3443_d_n5;
        var_vfbac1_t_dn6 = assign3920_e3443_d_n6;
        var_vfbac1_t_dn7 = assign3920_e3443_d_n7;
        var_vfbac1_t_dn8 = assign3920_e3443_d_n8;
        var_vfbac1_t_dn9 = assign3920_e3443_d_n9;
        var_vfbac1_t_db0 = assign3920_e3443_d_b0;
        var_vfbac1_t_db1 = assign3920_e3443_d_b1;
        var_vfbac1_t_db2 = assign3920_e3443_d_b2;
        var_vfbac1_t_db3 = assign3920_e3443_d_b3;

        let (assign3930_e3448, assign3930_e3448_d_n0, assign3930_e3448_d_n1, assign3930_e3448_d_n2, assign3930_e3448_d_n3, assign3930_e3448_d_n4, assign3930_e3448_d_n5, assign3930_e3448_d_n6, assign3930_e3448_d_n7, assign3930_e3448_d_n8, assign3930_e3448_d_n9, assign3930_e3448_d_b0, assign3930_e3448_d_b1, assign3930_e3448_d_b2, assign3930_e3448_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_vfb2_t, var_vfb2_t_dn0, var_vfb2_t_dn1, var_vfb2_t_dn2, var_vfb2_t_dn3, var_vfb2_t_dn4, var_vfb2_t_dn5, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9, var_vfb2_t_db0, var_vfb2_t_db1, var_vfb2_t_db2, var_vfb2_t_db3,)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign3930_e3448;
        var_vfbac2_t_dn0 = assign3930_e3448_d_n0;
        var_vfbac2_t_dn1 = assign3930_e3448_d_n1;
        var_vfbac2_t_dn2 = assign3930_e3448_d_n2;
        var_vfbac2_t_dn3 = assign3930_e3448_d_n3;
        var_vfbac2_t_dn4 = assign3930_e3448_d_n4;
        var_vfbac2_t_dn5 = assign3930_e3448_d_n5;
        var_vfbac2_t_dn6 = assign3930_e3448_d_n6;
        var_vfbac2_t_dn7 = assign3930_e3448_d_n7;
        var_vfbac2_t_dn8 = assign3930_e3448_d_n8;
        var_vfbac2_t_dn9 = assign3930_e3448_d_n9;
        var_vfbac2_t_db0 = assign3930_e3448_d_b0;
        var_vfbac2_t_db1 = assign3930_e3448_d_b1;
        var_vfbac2_t_db2 = assign3930_e3448_d_b2;
        var_vfbac2_t_db3 = assign3930_e3448_d_b3;

        let (assign3940_e3453,) = {
    if (var_guard83 == 0.0) {
        (var_psce1_i,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign3940_e3453;

        let (assign3950_e3458,) = {
    if (var_guard83 == 0.0) {
        (var_psce2_i,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign3950_e3458;

        let (assign3960_e3463, assign3960_e3463_d_n0, assign3960_e3463_d_n1, assign3960_e3463_d_n2, assign3960_e3463_d_n3, assign3960_e3463_d_n4, assign3960_e3463_d_n5, assign3960_e3463_d_n6, assign3960_e3463_d_n7, assign3960_e3463_d_n8, assign3960_e3463_d_n9, assign3960_e3463_d_b0, assign3960_e3463_d_b1, assign3960_e3463_d_b2, assign3960_e3463_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_cf_p, var_cf_p_dn0, var_cf_p_dn1, var_cf_p_dn2, var_cf_p_dn3, var_cf_p_dn4, var_cf_p_dn5, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9, var_cf_p_db0, var_cf_p_db1, var_cf_p_db2, var_cf_p_db3,)
    } else {
        (var_cfac_p, var_cfac_p_dn0, var_cfac_p_dn1, var_cfac_p_dn2, var_cfac_p_dn3, var_cfac_p_dn4, var_cfac_p_dn5, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9, var_cfac_p_db0, var_cfac_p_db1, var_cfac_p_db2, var_cfac_p_db3,)
    }
};
        var_cfac_p = assign3960_e3463;
        var_cfac_p_dn0 = assign3960_e3463_d_n0;
        var_cfac_p_dn1 = assign3960_e3463_d_n1;
        var_cfac_p_dn2 = assign3960_e3463_d_n2;
        var_cfac_p_dn3 = assign3960_e3463_d_n3;
        var_cfac_p_dn4 = assign3960_e3463_d_n4;
        var_cfac_p_dn5 = assign3960_e3463_d_n5;
        var_cfac_p_dn6 = assign3960_e3463_d_n6;
        var_cfac_p_dn7 = assign3960_e3463_d_n7;
        var_cfac_p_dn8 = assign3960_e3463_d_n8;
        var_cfac_p_dn9 = assign3960_e3463_d_n9;
        var_cfac_p_db0 = assign3960_e3463_d_b0;
        var_cfac_p_db1 = assign3960_e3463_d_b1;
        var_cfac_p_db2 = assign3960_e3463_d_b2;
        var_cfac_p_db3 = assign3960_e3463_d_b3;

        let (assign3970_e3468, assign3970_e3468_d_n0, assign3970_e3468_d_n1, assign3970_e3468_d_n2, assign3970_e3468_d_n3, assign3970_e3468_d_n4, assign3970_e3468_d_n5, assign3970_e3468_d_n6, assign3970_e3468_d_n7, assign3970_e3468_d_n8, assign3970_e3468_d_n9, assign3970_e3468_d_b0, assign3970_e3468_d_b1, assign3970_e3468_d_b2, assign3970_e3468_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_cf1_t, var_cf1_t_dn0, var_cf1_t_dn1, var_cf1_t_dn2, var_cf1_t_dn3, var_cf1_t_dn4, var_cf1_t_dn5, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9, var_cf1_t_db0, var_cf1_t_db1, var_cf1_t_db2, var_cf1_t_db3,)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign3970_e3468;
        var_cfac1_t_dn0 = assign3970_e3468_d_n0;
        var_cfac1_t_dn1 = assign3970_e3468_d_n1;
        var_cfac1_t_dn2 = assign3970_e3468_d_n2;
        var_cfac1_t_dn3 = assign3970_e3468_d_n3;
        var_cfac1_t_dn4 = assign3970_e3468_d_n4;
        var_cfac1_t_dn5 = assign3970_e3468_d_n5;
        var_cfac1_t_dn6 = assign3970_e3468_d_n6;
        var_cfac1_t_dn7 = assign3970_e3468_d_n7;
        var_cfac1_t_dn8 = assign3970_e3468_d_n8;
        var_cfac1_t_dn9 = assign3970_e3468_d_n9;
        var_cfac1_t_db0 = assign3970_e3468_d_b0;
        var_cfac1_t_db1 = assign3970_e3468_d_b1;
        var_cfac1_t_db2 = assign3970_e3468_d_b2;
        var_cfac1_t_db3 = assign3970_e3468_d_b3;

        let (assign3980_e3473, assign3980_e3473_d_n0, assign3980_e3473_d_n1, assign3980_e3473_d_n2, assign3980_e3473_d_n3, assign3980_e3473_d_n4, assign3980_e3473_d_n5, assign3980_e3473_d_n6, assign3980_e3473_d_n7, assign3980_e3473_d_n8, assign3980_e3473_d_n9, assign3980_e3473_d_b0, assign3980_e3473_d_b1, assign3980_e3473_d_b2, assign3980_e3473_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_cf2_t, var_cf2_t_dn0, var_cf2_t_dn1, var_cf2_t_dn2, var_cf2_t_dn3, var_cf2_t_dn4, var_cf2_t_dn5, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9, var_cf2_t_db0, var_cf2_t_db1, var_cf2_t_db2, var_cf2_t_db3,)
    } else {
        (var_cfac2_t, var_cfac2_t_dn0, var_cfac2_t_dn1, var_cfac2_t_dn2, var_cfac2_t_dn3, var_cfac2_t_dn4, var_cfac2_t_dn5, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9, var_cfac2_t_db0, var_cfac2_t_db1, var_cfac2_t_db2, var_cfac2_t_db3,)
    }
};
        var_cfac2_t = assign3980_e3473;
        var_cfac2_t_dn0 = assign3980_e3473_d_n0;
        var_cfac2_t_dn1 = assign3980_e3473_d_n1;
        var_cfac2_t_dn2 = assign3980_e3473_d_n2;
        var_cfac2_t_dn3 = assign3980_e3473_d_n3;
        var_cfac2_t_dn4 = assign3980_e3473_d_n4;
        var_cfac2_t_dn5 = assign3980_e3473_d_n5;
        var_cfac2_t_dn6 = assign3980_e3473_d_n6;
        var_cfac2_t_dn7 = assign3980_e3473_d_n7;
        var_cfac2_t_dn8 = assign3980_e3473_d_n8;
        var_cfac2_t_dn9 = assign3980_e3473_d_n9;
        var_cfac2_t_db0 = assign3980_e3473_d_b0;
        var_cfac2_t_db1 = assign3980_e3473_d_b1;
        var_cfac2_t_db2 = assign3980_e3473_d_b2;
        var_cfac2_t_db3 = assign3980_e3473_d_b3;

        let (assign3990_e3478, assign3990_e3478_d_n0, assign3990_e3478_d_n1, assign3990_e3478_d_n2, assign3990_e3478_d_n3, assign3990_e3478_d_n4, assign3990_e3478_d_n5, assign3990_e3478_d_n6, assign3990_e3478_d_n7, assign3990_e3478_d_n8, assign3990_e3478_d_n9, assign3990_e3478_d_b0, assign3990_e3478_d_b1, assign3990_e3478_d_b2, assign3990_e3478_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_p, var_thesat_p_dn0, var_thesat_p_dn1, var_thesat_p_dn2, var_thesat_p_dn3, var_thesat_p_dn4, var_thesat_p_dn5, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9, var_thesat_p_db0, var_thesat_p_db1, var_thesat_p_db2, var_thesat_p_db3,)
    } else {
        (var_thesatac_p, var_thesatac_p_dn0, var_thesatac_p_dn1, var_thesatac_p_dn2, var_thesatac_p_dn3, var_thesatac_p_dn4, var_thesatac_p_dn5, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9, var_thesatac_p_db0, var_thesatac_p_db1, var_thesatac_p_db2, var_thesatac_p_db3,)
    }
};
        var_thesatac_p = assign3990_e3478;
        var_thesatac_p_dn0 = assign3990_e3478_d_n0;
        var_thesatac_p_dn1 = assign3990_e3478_d_n1;
        var_thesatac_p_dn2 = assign3990_e3478_d_n2;
        var_thesatac_p_dn3 = assign3990_e3478_d_n3;
        var_thesatac_p_dn4 = assign3990_e3478_d_n4;
        var_thesatac_p_dn5 = assign3990_e3478_d_n5;
        var_thesatac_p_dn6 = assign3990_e3478_d_n6;
        var_thesatac_p_dn7 = assign3990_e3478_d_n7;
        var_thesatac_p_dn8 = assign3990_e3478_d_n8;
        var_thesatac_p_dn9 = assign3990_e3478_d_n9;
        var_thesatac_p_db0 = assign3990_e3478_d_b0;
        var_thesatac_p_db1 = assign3990_e3478_d_b1;
        var_thesatac_p_db2 = assign3990_e3478_d_b2;
        var_thesatac_p_db3 = assign3990_e3478_d_b3;

        let (assign4000_e3483, assign4000_e3483_d_n0, assign4000_e3483_d_n1, assign4000_e3483_d_n2, assign4000_e3483_d_n3, assign4000_e3483_d_n4, assign4000_e3483_d_n5, assign4000_e3483_d_n6, assign4000_e3483_d_n7, assign4000_e3483_d_n8, assign4000_e3483_d_n9, assign4000_e3483_d_b0, assign4000_e3483_d_b1, assign4000_e3483_d_b2, assign4000_e3483_d_b3,) = {
    if (var_guard83 == 0.0) {
        (var_thesat_t, var_thesat_t_dn0, var_thesat_t_dn1, var_thesat_t_dn2, var_thesat_t_dn3, var_thesat_t_dn4, var_thesat_t_dn5, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9, var_thesat_t_db0, var_thesat_t_db1, var_thesat_t_db2, var_thesat_t_db3,)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign4000_e3483;
        var_thesatac_t_dn0 = assign4000_e3483_d_n0;
        var_thesatac_t_dn1 = assign4000_e3483_d_n1;
        var_thesatac_t_dn2 = assign4000_e3483_d_n2;
        var_thesatac_t_dn3 = assign4000_e3483_d_n3;
        var_thesatac_t_dn4 = assign4000_e3483_d_n4;
        var_thesatac_t_dn5 = assign4000_e3483_d_n5;
        var_thesatac_t_dn6 = assign4000_e3483_d_n6;
        var_thesatac_t_dn7 = assign4000_e3483_d_n7;
        var_thesatac_t_dn8 = assign4000_e3483_d_n8;
        var_thesatac_t_dn9 = assign4000_e3483_d_n9;
        var_thesatac_t_db0 = assign4000_e3483_d_b0;
        var_thesatac_t_db1 = assign4000_e3483_d_b1;
        var_thesatac_t_db2 = assign4000_e3483_d_b2;
        var_thesatac_t_db3 = assign4000_e3483_d_b3;

        let (assign4010_e3488,) = {
    if (var_guard83 == 0.0) {
        (var_ax_i,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4010_e3488;

        let (assign4020_e3493,) = {
    if (var_guard83 == 0.0) {
        (var_alp_i,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign4020_e3493;

        let assign4030_e3496: f64 = if p.p11 > 0.0 { 1.0 } else { 0.0 };
        var_guard98 = assign4030_e3496;

        let (assign4040_e3503,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p207,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4040_e3503;

        let assign4050_e3505: f64 = if param_given[397] { 1.0 } else { 0.0 };
        let assign4050_e3507: f64 = if assign4050_e3505 == 1.0 { 1.0 } else { 0.0 };
        var_guard99 = assign4050_e3507;

        let (assign4060_e3516,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard99 != 0.0)) {
        (p.p397,)
    } else {
        (var_vfbaco_i,)
    }
};
        var_vfbaco_i = assign4060_e3516;

        let (assign4070_e3523,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p208,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4070_e3523;

        let assign4080_e3525: f64 = if param_given[398] { 1.0 } else { 0.0 };
        let assign4080_e3527: f64 = if assign4080_e3525 == 1.0 { 1.0 } else { 0.0 };
        var_guard100 = assign4080_e3527;

        let (assign4090_e3536,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard100 != 0.0)) {
        (p.p398,)
    } else {
        (var_vfbacl_i,)
    }
};
        var_vfbacl_i = assign4090_e3536;

        let (assign4100_e3543,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p209,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4100_e3543;

        let assign4110_e3545: f64 = if param_given[399] { 1.0 } else { 0.0 };
        let assign4110_e3547: f64 = if assign4110_e3545 == 1.0 { 1.0 } else { 0.0 };
        var_guard101 = assign4110_e3547;

        let (assign4120_e3556,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard101 != 0.0)) {
        (p.p399,)
    } else {
        (var_vfbaclexp_i,)
    }
};
        var_vfbaclexp_i = assign4120_e3556;

        let (assign4130_e3563,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p212,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4130_e3563;

        let assign4140_e3565: f64 = if param_given[402] { 1.0 } else { 0.0 };
        let assign4140_e3567: f64 = if assign4140_e3565 == 1.0 { 1.0 } else { 0.0 };
        var_guard102 = assign4140_e3567;

        let (assign4150_e3576,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard102 != 0.0)) {
        (p.p402,)
    } else {
        (var_vfbacw_i,)
    }
};
        var_vfbacw_i = assign4150_e3576;

        let (assign4160_e3583,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p213,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4160_e3583;

        let assign4170_e3585: f64 = if param_given[403] { 1.0 } else { 0.0 };
        let assign4170_e3587: f64 = if assign4170_e3585 == 1.0 { 1.0 } else { 0.0 };
        var_guard103 = assign4170_e3587;

        let (assign4180_e3596,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard103 != 0.0)) {
        (p.p403,)
    } else {
        (var_vfbaclw_i,)
    }
};
        var_vfbaclw_i = assign4180_e3596;

        let (assign4190_e3603,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p210,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4190_e3603;

        let assign4200_e3605: f64 = if param_given[400] { 1.0 } else { 0.0 };
        let assign4200_e3607: f64 = if assign4200_e3605 == 1.0 { 1.0 } else { 0.0 };
        var_guard104 = assign4200_e3607;

        let (assign4210_e3616,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard104 != 0.0)) {
        (p.p400,)
    } else {
        (var_vfbacl2_i,)
    }
};
        var_vfbacl2_i = assign4210_e3616;

        let (assign4220_e3623,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p211,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4220_e3623;

        let assign4230_e3625: f64 = if param_given[401] { 1.0 } else { 0.0 };
        let assign4230_e3627: f64 = if assign4230_e3625 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign4230_e3627;

        let (assign4240_e3636,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p401,)
    } else {
        (var_vfbaclexp2_i,)
    }
};
        var_vfbaclexp2_i = assign4240_e3636;

        let (assign4250_e3655, assign4250_e3655_d_n0, assign4250_e3655_d_n1, assign4250_e3655_d_n2, assign4250_e3655_d_n3, assign4250_e3655_d_n4, assign4250_e3655_d_n5, assign4250_e3655_d_n6, assign4250_e3655_d_n7, assign4250_e3655_d_n8, assign4250_e3655_d_n9, assign4250_e3655_d_b0, assign4250_e3655_d_b1, assign4250_e3655_d_b2, assign4250_e3655_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4250_e3644: f64 = (var_ile).powf(var_vfbaclexp_i);
        let assign4250_e3645: f64 = (var_vfbacl_i * assign4250_e3644);
        let assign4250_e3650: f64 = (var_ile).powf(var_vfbaclexp2_i);
        let assign4250_e3651: f64 = (var_vfbacl2_i * assign4250_e3650);
        let assign4250_e3652: f64 = (1.0 + assign4250_e3651);
        let assign4250_e3653: f64 = (assign4250_e3645 / assign4250_e3652);
        (assign4250_e3653, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign4250_e3655;
        var_temp_dn0 = assign4250_e3655_d_n0;
        var_temp_dn1 = assign4250_e3655_d_n1;
        var_temp_dn2 = assign4250_e3655_d_n2;
        var_temp_dn3 = assign4250_e3655_d_n3;
        var_temp_dn4 = assign4250_e3655_d_n4;
        var_temp_dn5 = assign4250_e3655_d_n5;
        var_temp_dn6 = assign4250_e3655_d_n6;
        var_temp_dn7 = assign4250_e3655_d_n7;
        var_temp_dn8 = assign4250_e3655_d_n8;
        var_temp_dn9 = assign4250_e3655_d_n9;
        var_temp_db0 = assign4250_e3655_d_b0;
        var_temp_db1 = assign4250_e3655_d_b1;
        var_temp_db2 = assign4250_e3655_d_b2;
        var_temp_db3 = assign4250_e3655_d_b3;

        let (assign4260_e3672, assign4260_e3672_d_n0, assign4260_e3672_d_n1, assign4260_e3672_d_n2, assign4260_e3672_d_n3, assign4260_e3672_d_n4, assign4260_e3672_d_n5, assign4260_e3672_d_n6, assign4260_e3672_d_n7, assign4260_e3672_d_n8, assign4260_e3672_d_n9, assign4260_e3672_d_b0, assign4260_e3672_d_b1, assign4260_e3672_d_b2, assign4260_e3672_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4260_e3662: f64 = (var_vfbaco_i + var_temp);
        let assign4260_e3665: f64 = (var_vfbacw_i * var_iwe);
        let assign4260_e3666: f64 = (assign4260_e3662 + assign4260_e3665);
        let assign4260_e3669: f64 = (var_vfbaclw_i * var_iae);
        let assign4260_e3670: f64 = (assign4260_e3666 + assign4260_e3669);
        (assign4260_e3670, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign4260_e3672;
        var_vfbac1_t_dn0 = assign4260_e3672_d_n0;
        var_vfbac1_t_dn1 = assign4260_e3672_d_n1;
        var_vfbac1_t_dn2 = assign4260_e3672_d_n2;
        var_vfbac1_t_dn3 = assign4260_e3672_d_n3;
        var_vfbac1_t_dn4 = assign4260_e3672_d_n4;
        var_vfbac1_t_dn5 = assign4260_e3672_d_n5;
        var_vfbac1_t_dn6 = assign4260_e3672_d_n6;
        var_vfbac1_t_dn7 = assign4260_e3672_d_n7;
        var_vfbac1_t_dn8 = assign4260_e3672_d_n8;
        var_vfbac1_t_dn9 = assign4260_e3672_d_n9;
        var_vfbac1_t_db0 = assign4260_e3672_d_b0;
        var_vfbac1_t_db1 = assign4260_e3672_d_b1;
        var_vfbac1_t_db2 = assign4260_e3672_d_b2;
        var_vfbac1_t_db3 = assign4260_e3672_d_b3;

        let (assign4270_e3679,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p214,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4270_e3679;

        let assign4280_e3681: f64 = if param_given[404] { 1.0 } else { 0.0 };
        let assign4280_e3683: f64 = if assign4280_e3681 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign4280_e3683;

        let (assign4290_e3692,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p404,)
    } else {
        (var_vfbbaco_i,)
    }
};
        var_vfbbaco_i = assign4290_e3692;

        let (assign4300_e3699,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p215,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4300_e3699;

        let assign4310_e3701: f64 = if param_given[405] { 1.0 } else { 0.0 };
        let assign4310_e3703: f64 = if assign4310_e3701 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign4310_e3703;

        let (assign4320_e3712,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p405,)
    } else {
        (var_vfblbaco_i,)
    }
};
        var_vfblbaco_i = assign4320_e3712;

        *var_alpac_i_slot = var_alpac_i;
        *var_axac_i_slot = var_axac_i;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_db0_slot = var_cfac1_t_db0;
        *var_cfac1_t_db1_slot = var_cfac1_t_db1;
        *var_cfac1_t_db2_slot = var_cfac1_t_db2;
        *var_cfac1_t_db3_slot = var_cfac1_t_db3;
        *var_cfac1_t_dn0_slot = var_cfac1_t_dn0;
        *var_cfac1_t_dn1_slot = var_cfac1_t_dn1;
        *var_cfac1_t_dn2_slot = var_cfac1_t_dn2;
        *var_cfac1_t_dn3_slot = var_cfac1_t_dn3;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn5_slot = var_cfac1_t_dn5;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_db0_slot = var_cfac2_t_db0;
        *var_cfac2_t_db1_slot = var_cfac2_t_db1;
        *var_cfac2_t_db2_slot = var_cfac2_t_db2;
        *var_cfac2_t_db3_slot = var_cfac2_t_db3;
        *var_cfac2_t_dn0_slot = var_cfac2_t_dn0;
        *var_cfac2_t_dn1_slot = var_cfac2_t_dn1;
        *var_cfac2_t_dn2_slot = var_cfac2_t_dn2;
        *var_cfac2_t_dn3_slot = var_cfac2_t_dn3;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn5_slot = var_cfac2_t_dn5;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_db0_slot = var_cfac_p_db0;
        *var_cfac_p_db1_slot = var_cfac_p_db1;
        *var_cfac_p_db2_slot = var_cfac_p_db2;
        *var_cfac_p_db3_slot = var_cfac_p_db3;
        *var_cfac_p_dn0_slot = var_cfac_p_dn0;
        *var_cfac_p_dn1_slot = var_cfac_p_dn1;
        *var_cfac_p_dn2_slot = var_cfac_p_dn2;
        *var_cfac_p_dn3_slot = var_cfac_p_dn3;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn5_slot = var_cfac_p_dn5;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_db0_slot = var_cgbov_i_db0;
        *var_cgbov_i_db1_slot = var_cgbov_i_db1;
        *var_cgbov_i_db2_slot = var_cgbov_i_db2;
        *var_cgbov_i_db3_slot = var_cgbov_i_db3;
        *var_cgbov_i_dn0_slot = var_cgbov_i_dn0;
        *var_cgbov_i_dn1_slot = var_cgbov_i_dn1;
        *var_cgbov_i_dn2_slot = var_cgbov_i_dn2;
        *var_cgbov_i_dn3_slot = var_cgbov_i_dn3;
        *var_cgbov_i_dn4_slot = var_cgbov_i_dn4;
        *var_cgbov_i_dn5_slot = var_cgbov_i_dn5;
        *var_cgbov_i_dn6_slot = var_cgbov_i_dn6;
        *var_cgbov_i_dn7_slot = var_cgbov_i_dn7;
        *var_cgbov_i_dn8_slot = var_cgbov_i_dn8;
        *var_cgbov_i_dn9_slot = var_cgbov_i_dn9;
        *var_fif_i_slot = var_fif_i;
        *var_fsceac_i_slot = var_fsceac_i;
        *var_guard100_slot = var_guard100;
        *var_guard101_slot = var_guard101;
        *var_guard102_slot = var_guard102;
        *var_guard103_slot = var_guard103;
        *var_guard104_slot = var_guard104;
        *var_guard105_slot = var_guard105;
        *var_guard106_slot = var_guard106;
        *var_guard107_slot = var_guard107;
        *var_guard98_slot = var_guard98;
        *var_guard99_slot = var_guard99;
        *var_nsdac_i_slot = var_nsdac_i;
        *var_psceac1_i_slot = var_psceac1_i;
        *var_psceac2_i_slot = var_psceac2_i;
        *var_temp_slot = var_temp;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_db0_slot = var_thesatac_p_db0;
        *var_thesatac_p_db1_slot = var_thesatac_p_db1;
        *var_thesatac_p_db2_slot = var_thesatac_p_db2;
        *var_thesatac_p_db3_slot = var_thesatac_p_db3;
        *var_thesatac_p_dn0_slot = var_thesatac_p_dn0;
        *var_thesatac_p_dn1_slot = var_thesatac_p_dn1;
        *var_thesatac_p_dn2_slot = var_thesatac_p_dn2;
        *var_thesatac_p_dn3_slot = var_thesatac_p_dn3;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn5_slot = var_thesatac_p_dn5;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_db0_slot = var_thesatac_t_db0;
        *var_thesatac_t_db1_slot = var_thesatac_t_db1;
        *var_thesatac_t_db2_slot = var_thesatac_t_db2;
        *var_thesatac_t_db3_slot = var_thesatac_t_db3;
        *var_thesatac_t_dn0_slot = var_thesatac_t_dn0;
        *var_thesatac_t_dn1_slot = var_thesatac_t_dn1;
        *var_thesatac_t_dn2_slot = var_thesatac_t_dn2;
        *var_thesatac_t_dn3_slot = var_thesatac_t_dn3;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn5_slot = var_thesatac_t_dn5;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_db0_slot = var_vfbac1_t_db0;
        *var_vfbac1_t_db1_slot = var_vfbac1_t_db1;
        *var_vfbac1_t_db2_slot = var_vfbac1_t_db2;
        *var_vfbac1_t_db3_slot = var_vfbac1_t_db3;
        *var_vfbac1_t_dn0_slot = var_vfbac1_t_dn0;
        *var_vfbac1_t_dn1_slot = var_vfbac1_t_dn1;
        *var_vfbac1_t_dn2_slot = var_vfbac1_t_dn2;
        *var_vfbac1_t_dn3_slot = var_vfbac1_t_dn3;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn5_slot = var_vfbac1_t_dn5;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_db0_slot = var_vfbac2_t_db0;
        *var_vfbac2_t_db1_slot = var_vfbac2_t_db1;
        *var_vfbac2_t_db2_slot = var_vfbac2_t_db2;
        *var_vfbac2_t_db3_slot = var_vfbac2_t_db3;
        *var_vfbac2_t_dn0_slot = var_vfbac2_t_dn0;
        *var_vfbac2_t_dn1_slot = var_vfbac2_t_dn1;
        *var_vfbac2_t_dn2_slot = var_vfbac2_t_dn2;
        *var_vfbac2_t_dn3_slot = var_vfbac2_t_dn3;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn5_slot = var_vfbac2_t_dn5;
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
        *var_vfbbaco_i_slot = var_vfbbaco_i;
        *var_vfblbaco_i_slot = var_vfblbaco_i;
    }

    pub(super) fn stamp_transient_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_ge: f64,
        var_ge_db0: f64,
        var_ge_db1: f64,
        var_ge_db2: f64,
        var_ge_db3: f64,
        var_ge_dn0: f64,
        var_ge_dn1: f64,
        var_ge_dn2: f64,
        var_ge_dn3: f64,
        var_ge_dn4: f64,
        var_ge_dn5: f64,
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
        var_vfbbaco_i: f64,
        var_vfblbaco_i: f64,
        var_axacl_i_slot: &mut f64,
        var_axaco_i_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_db0_slot: &mut f64,
        var_cfac1_t_db1_slot: &mut f64,
        var_cfac1_t_db2_slot: &mut f64,
        var_cfac1_t_db3_slot: &mut f64,
        var_cfac1_t_dn0_slot: &mut f64,
        var_cfac1_t_dn1_slot: &mut f64,
        var_cfac1_t_dn2_slot: &mut f64,
        var_cfac1_t_dn3_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn5_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_db0_slot: &mut f64,
        var_cfac2_t_db1_slot: &mut f64,
        var_cfac2_t_db2_slot: &mut f64,
        var_cfac2_t_db3_slot: &mut f64,
        var_cfac2_t_dn0_slot: &mut f64,
        var_cfac2_t_dn1_slot: &mut f64,
        var_cfac2_t_dn2_slot: &mut f64,
        var_cfac2_t_dn3_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn5_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_db0_slot: &mut f64,
        var_cfac_p_db1_slot: &mut f64,
        var_cfac_p_db2_slot: &mut f64,
        var_cfac_p_db3_slot: &mut f64,
        var_cfac_p_dn0_slot: &mut f64,
        var_cfac_p_dn1_slot: &mut f64,
        var_cfac_p_dn2_slot: &mut f64,
        var_cfac_p_dn3_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn5_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_cfacl_i_slot: &mut f64,
        var_cfaclexp_i_slot: &mut f64,
        var_cfacw_i_slot: &mut f64,
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
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_db0_slot: &mut f64,
        var_thesatac_p_db1_slot: &mut f64,
        var_thesatac_p_db2_slot: &mut f64,
        var_thesatac_p_db3_slot: &mut f64,
        var_thesatac_p_dn0_slot: &mut f64,
        var_thesatac_p_dn1_slot: &mut f64,
        var_thesatac_p_dn2_slot: &mut f64,
        var_thesatac_p_dn3_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn5_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_db0_slot: &mut f64,
        var_thesatac_t_db1_slot: &mut f64,
        var_thesatac_t_db2_slot: &mut f64,
        var_thesatac_t_db3_slot: &mut f64,
        var_thesatac_t_dn0_slot: &mut f64,
        var_thesatac_t_dn1_slot: &mut f64,
        var_thesatac_t_dn2_slot: &mut f64,
        var_thesatac_t_dn3_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn5_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_thesatacl_i_slot: &mut f64,
        var_thesataclexp_i_slot: &mut f64,
        var_thesataclw_i_slot: &mut f64,
        var_thesataco_i_slot: &mut f64,
        var_thesatacw_i_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_db0_slot: &mut f64,
        var_vfbac2_t_db1_slot: &mut f64,
        var_vfbac2_t_db2_slot: &mut f64,
        var_vfbac2_t_db3_slot: &mut f64,
        var_vfbac2_t_dn0_slot: &mut f64,
        var_vfbac2_t_dn1_slot: &mut f64,
        var_vfbac2_t_dn2_slot: &mut f64,
        var_vfbac2_t_dn3_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn5_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_axacl_i: f64 = *var_axacl_i_slot;
        let mut var_axaco_i: f64 = *var_axaco_i_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_db0: f64 = *var_cfac1_t_db0_slot;
        let mut var_cfac1_t_db1: f64 = *var_cfac1_t_db1_slot;
        let mut var_cfac1_t_db2: f64 = *var_cfac1_t_db2_slot;
        let mut var_cfac1_t_db3: f64 = *var_cfac1_t_db3_slot;
        let mut var_cfac1_t_dn0: f64 = *var_cfac1_t_dn0_slot;
        let mut var_cfac1_t_dn1: f64 = *var_cfac1_t_dn1_slot;
        let mut var_cfac1_t_dn2: f64 = *var_cfac1_t_dn2_slot;
        let mut var_cfac1_t_dn3: f64 = *var_cfac1_t_dn3_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn5: f64 = *var_cfac1_t_dn5_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_db0: f64 = *var_cfac2_t_db0_slot;
        let mut var_cfac2_t_db1: f64 = *var_cfac2_t_db1_slot;
        let mut var_cfac2_t_db2: f64 = *var_cfac2_t_db2_slot;
        let mut var_cfac2_t_db3: f64 = *var_cfac2_t_db3_slot;
        let mut var_cfac2_t_dn0: f64 = *var_cfac2_t_dn0_slot;
        let mut var_cfac2_t_dn1: f64 = *var_cfac2_t_dn1_slot;
        let mut var_cfac2_t_dn2: f64 = *var_cfac2_t_dn2_slot;
        let mut var_cfac2_t_dn3: f64 = *var_cfac2_t_dn3_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn5: f64 = *var_cfac2_t_dn5_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_db0: f64 = *var_cfac_p_db0_slot;
        let mut var_cfac_p_db1: f64 = *var_cfac_p_db1_slot;
        let mut var_cfac_p_db2: f64 = *var_cfac_p_db2_slot;
        let mut var_cfac_p_db3: f64 = *var_cfac_p_db3_slot;
        let mut var_cfac_p_dn0: f64 = *var_cfac_p_dn0_slot;
        let mut var_cfac_p_dn1: f64 = *var_cfac_p_dn1_slot;
        let mut var_cfac_p_dn2: f64 = *var_cfac_p_dn2_slot;
        let mut var_cfac_p_dn3: f64 = *var_cfac_p_dn3_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn5: f64 = *var_cfac_p_dn5_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_cfacl_i: f64 = *var_cfacl_i_slot;
        let mut var_cfaclexp_i: f64 = *var_cfaclexp_i_slot;
        let mut var_cfacw_i: f64 = *var_cfacw_i_slot;
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
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_db0: f64 = *var_thesatac_p_db0_slot;
        let mut var_thesatac_p_db1: f64 = *var_thesatac_p_db1_slot;
        let mut var_thesatac_p_db2: f64 = *var_thesatac_p_db2_slot;
        let mut var_thesatac_p_db3: f64 = *var_thesatac_p_db3_slot;
        let mut var_thesatac_p_dn0: f64 = *var_thesatac_p_dn0_slot;
        let mut var_thesatac_p_dn1: f64 = *var_thesatac_p_dn1_slot;
        let mut var_thesatac_p_dn2: f64 = *var_thesatac_p_dn2_slot;
        let mut var_thesatac_p_dn3: f64 = *var_thesatac_p_dn3_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn5: f64 = *var_thesatac_p_dn5_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_db0: f64 = *var_thesatac_t_db0_slot;
        let mut var_thesatac_t_db1: f64 = *var_thesatac_t_db1_slot;
        let mut var_thesatac_t_db2: f64 = *var_thesatac_t_db2_slot;
        let mut var_thesatac_t_db3: f64 = *var_thesatac_t_db3_slot;
        let mut var_thesatac_t_dn0: f64 = *var_thesatac_t_dn0_slot;
        let mut var_thesatac_t_dn1: f64 = *var_thesatac_t_dn1_slot;
        let mut var_thesatac_t_dn2: f64 = *var_thesatac_t_dn2_slot;
        let mut var_thesatac_t_dn3: f64 = *var_thesatac_t_dn3_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn5: f64 = *var_thesatac_t_dn5_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_thesatacl_i: f64 = *var_thesatacl_i_slot;
        let mut var_thesataclexp_i: f64 = *var_thesataclexp_i_slot;
        let mut var_thesataclw_i: f64 = *var_thesataclw_i_slot;
        let mut var_thesataco_i: f64 = *var_thesataco_i_slot;
        let mut var_thesatacw_i: f64 = *var_thesatacw_i_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_db0: f64 = *var_vfbac2_t_db0_slot;
        let mut var_vfbac2_t_db1: f64 = *var_vfbac2_t_db1_slot;
        let mut var_vfbac2_t_db2: f64 = *var_vfbac2_t_db2_slot;
        let mut var_vfbac2_t_db3: f64 = *var_vfbac2_t_db3_slot;
        let mut var_vfbac2_t_dn0: f64 = *var_vfbac2_t_dn0_slot;
        let mut var_vfbac2_t_dn1: f64 = *var_vfbac2_t_dn1_slot;
        let mut var_vfbac2_t_dn2: f64 = *var_vfbac2_t_dn2_slot;
        let mut var_vfbac2_t_dn3: f64 = *var_vfbac2_t_dn3_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn5: f64 = *var_vfbac2_t_dn5_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign4330_e3727, assign4330_e3727_d_n0, assign4330_e3727_d_n1, assign4330_e3727_d_n2, assign4330_e3727_d_n3, assign4330_e3727_d_n4, assign4330_e3727_d_n5, assign4330_e3727_d_n6, assign4330_e3727_d_n7, assign4330_e3727_d_n8, assign4330_e3727_d_n9, assign4330_e3727_d_b0, assign4330_e3727_d_b1, assign4330_e3727_d_b2, assign4330_e3727_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4330_e3720: f64 = (var_vfblbaco_i * var_tox2_i);
        let assign4330_e3722: f64 = (assign4330_e3720 / var_tox1_i);
        let assign4330_e3724: f64 = (assign4330_e3722 * var_temp);
        let assign4330_e3725: f64 = (var_vfbbaco_i + assign4330_e3724);
        (assign4330_e3725, (assign4330_e3722 * var_temp_dn0), (assign4330_e3722 * var_temp_dn1), (assign4330_e3722 * var_temp_dn2), (assign4330_e3722 * var_temp_dn3), (assign4330_e3722 * var_temp_dn4), (assign4330_e3722 * var_temp_dn5), (assign4330_e3722 * var_temp_dn6), (assign4330_e3722 * var_temp_dn7), (assign4330_e3722 * var_temp_dn8), (assign4330_e3722 * var_temp_dn9), (assign4330_e3722 * var_temp_db0), (assign4330_e3722 * var_temp_db1), (assign4330_e3722 * var_temp_db2), (assign4330_e3722 * var_temp_db3),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign4330_e3727;
        var_vfbac2_t_dn0 = assign4330_e3727_d_n0;
        var_vfbac2_t_dn1 = assign4330_e3727_d_n1;
        var_vfbac2_t_dn2 = assign4330_e3727_d_n2;
        var_vfbac2_t_dn3 = assign4330_e3727_d_n3;
        var_vfbac2_t_dn4 = assign4330_e3727_d_n4;
        var_vfbac2_t_dn5 = assign4330_e3727_d_n5;
        var_vfbac2_t_dn6 = assign4330_e3727_d_n6;
        var_vfbac2_t_dn7 = assign4330_e3727_d_n7;
        var_vfbac2_t_dn8 = assign4330_e3727_d_n8;
        var_vfbac2_t_dn9 = assign4330_e3727_d_n9;
        var_vfbac2_t_db0 = assign4330_e3727_d_b0;
        var_vfbac2_t_db1 = assign4330_e3727_d_b1;
        var_vfbac2_t_db2 = assign4330_e3727_d_b2;
        var_vfbac2_t_db3 = assign4330_e3727_d_b3;

        let (assign4340_e3734,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p224,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4340_e3734;

        let assign4350_e3736: f64 = if param_given[406] { 1.0 } else { 0.0 };
        let assign4350_e3738: f64 = if assign4350_e3736 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign4350_e3738;

        let (assign4360_e3747,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p406,)
    } else {
        (var_psceacl_i,)
    }
};
        var_psceacl_i = assign4360_e3747;

        let (assign4370_e3754,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p225,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4370_e3754;

        let assign4380_e3756: f64 = if param_given[407] { 1.0 } else { 0.0 };
        let assign4380_e3758: f64 = if assign4380_e3756 == 1.0 { 1.0 } else { 0.0 };
        var_guard109 = assign4380_e3758;

        let (assign4390_e3767,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard109 != 0.0)) {
        (p.p407,)
    } else {
        (var_psceaclexp_i,)
    }
};
        var_psceaclexp_i = assign4390_e3767;

        let (assign4400_e3774,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p226,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4400_e3774;

        let assign4410_e3776: f64 = if param_given[408] { 1.0 } else { 0.0 };
        let assign4410_e3778: f64 = if assign4410_e3776 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign4410_e3778;

        let (assign4420_e3787,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p408,)
    } else {
        (var_psceacw_i,)
    }
};
        var_psceacw_i = assign4420_e3787;

        let (assign4430_e3806,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4430_e3794: f64 = (var_psceacl_i * 2.0);
        let assign4430_e3797: f64 = (var_lambda_le).powf(var_psceaclexp_i);
        let assign4430_e3798: f64 = (assign4430_e3794 * assign4430_e3797);
        let assign4430_e3802: f64 = (var_psceacw_i * var_iwe);
        let assign4430_e3803: f64 = (1.0 + assign4430_e3802);
        let assign4430_e3804: f64 = (assign4430_e3798 * assign4430_e3803);
        (assign4430_e3804,)
    } else {
        (var_psceac_p,)
    }
};
        var_psceac_p = assign4430_e3806;

        let (assign4440_e3817,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4440_e3813: f64 = (var_psceac_p).max(0.0);
        let assign4440_e3815: f64 = (assign4440_e3813).min(5.0);
        (assign4440_e3815,)
    } else {
        (var_psceac1_i,)
    }
};
        var_psceac1_i = assign4440_e3817;

        let (assign4450_e3830,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4450_e3824: f64 = (p.p227 * var_psceac1_i);
        let assign4450_e3826: f64 = (assign4450_e3824 * var_tox2_i);
        let assign4450_e3828: f64 = (assign4450_e3826 / var_tox1_i);
        (assign4450_e3828,)
    } else {
        (var_psceac2_i,)
    }
};
        var_psceac2_i = assign4450_e3830;

        let (assign4460_e3837,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p231,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4460_e3837;

        let assign4470_e3839: f64 = if param_given[409] { 1.0 } else { 0.0 };
        let assign4470_e3841: f64 = if assign4470_e3839 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign4470_e3841;

        let (assign4480_e3850,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p409,)
    } else {
        (var_cfacl_i,)
    }
};
        var_cfacl_i = assign4480_e3850;

        let (assign4490_e3857,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p232,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4490_e3857;

        let assign4500_e3859: f64 = if param_given[410] { 1.0 } else { 0.0 };
        let assign4500_e3861: f64 = if assign4500_e3859 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign4500_e3861;

        let (assign4510_e3870,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p410,)
    } else {
        (var_cfaclexp_i,)
    }
};
        var_cfaclexp_i = assign4510_e3870;

        let (assign4520_e3877,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p233,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4520_e3877;

        let assign4530_e3879: f64 = if param_given[411] { 1.0 } else { 0.0 };
        let assign4530_e3881: f64 = if assign4530_e3879 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign4530_e3881;

        let (assign4540_e3890,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p411,)
    } else {
        (var_cfacw_i,)
    }
};
        var_cfacw_i = assign4540_e3890;

        let (assign4550_e3905, assign4550_e3905_d_n0, assign4550_e3905_d_n1, assign4550_e3905_d_n2, assign4550_e3905_d_n3, assign4550_e3905_d_n4, assign4550_e3905_d_n5, assign4550_e3905_d_n6, assign4550_e3905_d_n7, assign4550_e3905_d_n8, assign4550_e3905_d_n9, assign4550_e3905_d_b0, assign4550_e3905_d_b1, assign4550_e3905_d_b2, assign4550_e3905_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4550_e3897: f64 = (var_lambda_le).powf(var_cfaclexp_i);
        let assign4550_e3901: f64 = (var_cfacw_i * var_iwe);
        let assign4550_e3902: f64 = (1.0 + assign4550_e3901);
        let assign4550_e3903: f64 = (assign4550_e3897 * assign4550_e3902);
        (assign4550_e3903, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign4550_e3905;
        var_temp_dn0 = assign4550_e3905_d_n0;
        var_temp_dn1 = assign4550_e3905_d_n1;
        var_temp_dn2 = assign4550_e3905_d_n2;
        var_temp_dn3 = assign4550_e3905_d_n3;
        var_temp_dn4 = assign4550_e3905_d_n4;
        var_temp_dn5 = assign4550_e3905_d_n5;
        var_temp_dn6 = assign4550_e3905_d_n6;
        var_temp_dn7 = assign4550_e3905_d_n7;
        var_temp_dn8 = assign4550_e3905_d_n8;
        var_temp_dn9 = assign4550_e3905_d_n9;
        var_temp_db0 = assign4550_e3905_d_b0;
        var_temp_db1 = assign4550_e3905_d_b1;
        var_temp_db2 = assign4550_e3905_d_b2;
        var_temp_db3 = assign4550_e3905_d_b3;

        let (assign4560_e3914, assign4560_e3914_d_n0, assign4560_e3914_d_n1, assign4560_e3914_d_n2, assign4560_e3914_d_n3, assign4560_e3914_d_n4, assign4560_e3914_d_n5, assign4560_e3914_d_n6, assign4560_e3914_d_n7, assign4560_e3914_d_n8, assign4560_e3914_d_n9, assign4560_e3914_d_b0, assign4560_e3914_d_b1, assign4560_e3914_d_b2, assign4560_e3914_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4560_e3912: f64 = (var_cfacl_i * var_temp);
        (assign4560_e3912, (var_cfacl_i * var_temp_dn0), (var_cfacl_i * var_temp_dn1), (var_cfacl_i * var_temp_dn2), (var_cfacl_i * var_temp_dn3), (var_cfacl_i * var_temp_dn4), (var_cfacl_i * var_temp_dn5), (var_cfacl_i * var_temp_dn6), (var_cfacl_i * var_temp_dn7), (var_cfacl_i * var_temp_dn8), (var_cfacl_i * var_temp_dn9), (var_cfacl_i * var_temp_db0), (var_cfacl_i * var_temp_db1), (var_cfacl_i * var_temp_db2), (var_cfacl_i * var_temp_db3),)
    } else {
        (var_cfac_p, var_cfac_p_dn0, var_cfac_p_dn1, var_cfac_p_dn2, var_cfac_p_dn3, var_cfac_p_dn4, var_cfac_p_dn5, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9, var_cfac_p_db0, var_cfac_p_db1, var_cfac_p_db2, var_cfac_p_db3,)
    }
};
        var_cfac_p = assign4560_e3914;
        var_cfac_p_dn0 = assign4560_e3914_d_n0;
        var_cfac_p_dn1 = assign4560_e3914_d_n1;
        var_cfac_p_dn2 = assign4560_e3914_d_n2;
        var_cfac_p_dn3 = assign4560_e3914_d_n3;
        var_cfac_p_dn4 = assign4560_e3914_d_n4;
        var_cfac_p_dn5 = assign4560_e3914_d_n5;
        var_cfac_p_dn6 = assign4560_e3914_d_n6;
        var_cfac_p_dn7 = assign4560_e3914_d_n7;
        var_cfac_p_dn8 = assign4560_e3914_d_n8;
        var_cfac_p_dn9 = assign4560_e3914_d_n9;
        var_cfac_p_db0 = assign4560_e3914_d_b0;
        var_cfac_p_db1 = assign4560_e3914_d_b1;
        var_cfac_p_db2 = assign4560_e3914_d_b2;
        var_cfac_p_db3 = assign4560_e3914_d_b3;

        let (assign4570_e3923, assign4570_e3923_d_n0, assign4570_e3923_d_n1, assign4570_e3923_d_n2, assign4570_e3923_d_n3, assign4570_e3923_d_n4, assign4570_e3923_d_n5, assign4570_e3923_d_n6, assign4570_e3923_d_n7, assign4570_e3923_d_n8, assign4570_e3923_d_n9, assign4570_e3923_d_b0, assign4570_e3923_d_b1, assign4570_e3923_d_b2, assign4570_e3923_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4570_e3921: f64 = (var_cfac_p).max(0.0);
        (assign4570_e3921, if var_cfac_p >= 0.0 { var_cfac_p_dn0 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn1 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn2 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn3 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn5 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db0 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db1 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db2 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db3 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign4570_e3923;
        var_cfac1_t_dn0 = assign4570_e3923_d_n0;
        var_cfac1_t_dn1 = assign4570_e3923_d_n1;
        var_cfac1_t_dn2 = assign4570_e3923_d_n2;
        var_cfac1_t_dn3 = assign4570_e3923_d_n3;
        var_cfac1_t_dn4 = assign4570_e3923_d_n4;
        var_cfac1_t_dn5 = assign4570_e3923_d_n5;
        var_cfac1_t_dn6 = assign4570_e3923_d_n6;
        var_cfac1_t_dn7 = assign4570_e3923_d_n7;
        var_cfac1_t_dn8 = assign4570_e3923_d_n8;
        var_cfac1_t_dn9 = assign4570_e3923_d_n9;
        var_cfac1_t_db0 = assign4570_e3923_d_b0;
        var_cfac1_t_db1 = assign4570_e3923_d_b1;
        var_cfac1_t_db2 = assign4570_e3923_d_b2;
        var_cfac1_t_db3 = assign4570_e3923_d_b3;

        let (assign4580_e3936, assign4580_e3936_d_n0, assign4580_e3936_d_n1, assign4580_e3936_d_n2, assign4580_e3936_d_n3, assign4580_e3936_d_n4, assign4580_e3936_d_n5, assign4580_e3936_d_n6, assign4580_e3936_d_n7, assign4580_e3936_d_n8, assign4580_e3936_d_n9, assign4580_e3936_d_b0, assign4580_e3936_d_b1, assign4580_e3936_d_b2, assign4580_e3936_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4580_e3930: f64 = (p.p234 * var_cfac1_t);
        let assign4580_e3932: f64 = (assign4580_e3930 * var_tox2_i);
        let assign4580_e3934: f64 = (assign4580_e3932 / var_tox1_i);
        (assign4580_e3934, (((p.p234 * var_cfac1_t_dn0) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn1) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn2) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn3) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn4) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn5) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn6) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn7) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn8) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_dn9) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_db0) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_db1) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_db2) * var_tox2_i) / var_tox1_i), (((p.p234 * var_cfac1_t_db3) * var_tox2_i) / var_tox1_i),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn0, var_cfac2_t_dn1, var_cfac2_t_dn2, var_cfac2_t_dn3, var_cfac2_t_dn4, var_cfac2_t_dn5, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9, var_cfac2_t_db0, var_cfac2_t_db1, var_cfac2_t_db2, var_cfac2_t_db3,)
    }
};
        var_cfac2_t = assign4580_e3936;
        var_cfac2_t_dn0 = assign4580_e3936_d_n0;
        var_cfac2_t_dn1 = assign4580_e3936_d_n1;
        var_cfac2_t_dn2 = assign4580_e3936_d_n2;
        var_cfac2_t_dn3 = assign4580_e3936_d_n3;
        var_cfac2_t_dn4 = assign4580_e3936_d_n4;
        var_cfac2_t_dn5 = assign4580_e3936_d_n5;
        var_cfac2_t_dn6 = assign4580_e3936_d_n6;
        var_cfac2_t_dn7 = assign4580_e3936_d_n7;
        var_cfac2_t_dn8 = assign4580_e3936_d_n8;
        var_cfac2_t_dn9 = assign4580_e3936_d_n9;
        var_cfac2_t_db0 = assign4580_e3936_d_b0;
        var_cfac2_t_db1 = assign4580_e3936_d_b1;
        var_cfac2_t_db2 = assign4580_e3936_d_b2;
        var_cfac2_t_db3 = assign4580_e3936_d_b3;

        let (assign4590_e3943,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p289,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4590_e3943;

        let assign4600_e3945: f64 = if param_given[412] { 1.0 } else { 0.0 };
        let assign4600_e3947: f64 = if assign4600_e3945 == 1.0 { 1.0 } else { 0.0 };
        var_guard114 = assign4600_e3947;

        let (assign4610_e3956,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard114 != 0.0)) {
        (p.p412,)
    } else {
        (var_thesataco_i,)
    }
};
        var_thesataco_i = assign4610_e3956;

        let (assign4620_e3963,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p290,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4620_e3963;

        let assign4630_e3965: f64 = if param_given[413] { 1.0 } else { 0.0 };
        let assign4630_e3967: f64 = if assign4630_e3965 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign4630_e3967;

        let (assign4640_e3976,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard115 != 0.0)) {
        (p.p413,)
    } else {
        (var_thesatacl_i,)
    }
};
        var_thesatacl_i = assign4640_e3976;

        let (assign4650_e3983,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p291,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4650_e3983;

        let assign4660_e3985: f64 = if param_given[414] { 1.0 } else { 0.0 };
        let assign4660_e3987: f64 = if assign4660_e3985 == 1.0 { 1.0 } else { 0.0 };
        var_guard116 = assign4660_e3987;

        let (assign4670_e3996,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard116 != 0.0)) {
        (p.p414,)
    } else {
        (var_thesataclexp_i,)
    }
};
        var_thesataclexp_i = assign4670_e3996;

        let (assign4680_e4003,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p292,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4680_e4003;

        let assign4690_e4005: f64 = if param_given[415] { 1.0 } else { 0.0 };
        let assign4690_e4007: f64 = if assign4690_e4005 == 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign4690_e4007;

        let (assign4700_e4016,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard117 != 0.0)) {
        (p.p415,)
    } else {
        (var_thesatacw_i,)
    }
};
        var_thesatacw_i = assign4700_e4016;

        let (assign4710_e4023,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p293,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4710_e4023;

        let assign4720_e4025: f64 = if param_given[416] { 1.0 } else { 0.0 };
        let assign4720_e4027: f64 = if assign4720_e4025 == 1.0 { 1.0 } else { 0.0 };
        var_guard118 = assign4720_e4027;

        let (assign4730_e4036,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard118 != 0.0)) {
        (p.p416,)
    } else {
        (var_thesataclw_i,)
    }
};
        var_thesataclw_i = assign4730_e4036;

        let (assign4740_e4063, assign4740_e4063_d_n0, assign4740_e4063_d_n1, assign4740_e4063_d_n2, assign4740_e4063_d_n3, assign4740_e4063_d_n4, assign4740_e4063_d_n5, assign4740_e4063_d_n6, assign4740_e4063_d_n7, assign4740_e4063_d_n8, assign4740_e4063_d_n9, assign4740_e4063_d_b0, assign4740_e4063_d_b1, assign4740_e4063_d_b2, assign4740_e4063_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4740_e4046: f64 = (var_ile).powf(var_thesataclexp_i);
        let assign4740_e4047: f64 = (var_thesatacl_i * assign4740_e4046);
        let assign4740_e4048: f64 = (var_thesataco_i + assign4740_e4047);
        let assign4740_e4049: f64 = (var_ge * assign4740_e4048);
        let assign4740_e4053: f64 = (var_thesatacw_i * var_iwe);
        let assign4740_e4054: f64 = (1.0 + assign4740_e4053);
        let assign4740_e4055: f64 = (assign4740_e4049 * assign4740_e4054);
        let assign4740_e4059: f64 = (var_thesataclw_i * var_iae);
        let assign4740_e4060: f64 = (1.0 + assign4740_e4059);
        let assign4740_e4061: f64 = (assign4740_e4055 * assign4740_e4060);
        (assign4740_e4061, (((var_ge_dn0 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn1 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn2 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn3 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn4 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn5 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn6 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn7 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn8 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_dn9 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_db0 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_db1 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_db2 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060), (((var_ge_db3 * assign4740_e4048) * assign4740_e4054) * assign4740_e4060),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn0, var_thesatac_p_dn1, var_thesatac_p_dn2, var_thesatac_p_dn3, var_thesatac_p_dn4, var_thesatac_p_dn5, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9, var_thesatac_p_db0, var_thesatac_p_db1, var_thesatac_p_db2, var_thesatac_p_db3,)
    }
};
        var_thesatac_p = assign4740_e4063;
        var_thesatac_p_dn0 = assign4740_e4063_d_n0;
        var_thesatac_p_dn1 = assign4740_e4063_d_n1;
        var_thesatac_p_dn2 = assign4740_e4063_d_n2;
        var_thesatac_p_dn3 = assign4740_e4063_d_n3;
        var_thesatac_p_dn4 = assign4740_e4063_d_n4;
        var_thesatac_p_dn5 = assign4740_e4063_d_n5;
        var_thesatac_p_dn6 = assign4740_e4063_d_n6;
        var_thesatac_p_dn7 = assign4740_e4063_d_n7;
        var_thesatac_p_dn8 = assign4740_e4063_d_n8;
        var_thesatac_p_dn9 = assign4740_e4063_d_n9;
        var_thesatac_p_db0 = assign4740_e4063_d_b0;
        var_thesatac_p_db1 = assign4740_e4063_d_b1;
        var_thesatac_p_db2 = assign4740_e4063_d_b2;
        var_thesatac_p_db3 = assign4740_e4063_d_b3;

        let (assign4750_e4072, assign4750_e4072_d_n0, assign4750_e4072_d_n1, assign4750_e4072_d_n2, assign4750_e4072_d_n3, assign4750_e4072_d_n4, assign4750_e4072_d_n5, assign4750_e4072_d_n6, assign4750_e4072_d_n7, assign4750_e4072_d_n8, assign4750_e4072_d_n9, assign4750_e4072_d_b0, assign4750_e4072_d_b1, assign4750_e4072_d_b2, assign4750_e4072_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4750_e4070: f64 = (var_thesatac_p).max(0.0);
        (assign4750_e4070, if var_thesatac_p >= 0.0 { var_thesatac_p_dn0 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn1 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn2 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn3 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn5 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db0 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db1 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db2 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db3 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign4750_e4072;
        var_thesatac_t_dn0 = assign4750_e4072_d_n0;
        var_thesatac_t_dn1 = assign4750_e4072_d_n1;
        var_thesatac_t_dn2 = assign4750_e4072_d_n2;
        var_thesatac_t_dn3 = assign4750_e4072_d_n3;
        var_thesatac_t_dn4 = assign4750_e4072_d_n4;
        var_thesatac_t_dn5 = assign4750_e4072_d_n5;
        var_thesatac_t_dn6 = assign4750_e4072_d_n6;
        var_thesatac_t_dn7 = assign4750_e4072_d_n7;
        var_thesatac_t_dn8 = assign4750_e4072_d_n8;
        var_thesatac_t_dn9 = assign4750_e4072_d_n9;
        var_thesatac_t_db0 = assign4750_e4072_d_b0;
        var_thesatac_t_db1 = assign4750_e4072_d_b1;
        var_thesatac_t_db2 = assign4750_e4072_d_b2;
        var_thesatac_t_db3 = assign4750_e4072_d_b3;

        let (assign4760_e4079,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p300,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4760_e4079;

        let assign4770_e4081: f64 = if param_given[417] { 1.0 } else { 0.0 };
        let assign4770_e4083: f64 = if assign4770_e4081 == 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign4770_e4083;

        let (assign4780_e4092,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard119 != 0.0)) {
        (p.p417,)
    } else {
        (var_axaco_i,)
    }
};
        var_axaco_i = assign4780_e4092;

        let (assign4790_e4099,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p301,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4790_e4099;

        let assign4800_e4101: f64 = if param_given[418] { 1.0 } else { 0.0 };
        let assign4800_e4103: f64 = if assign4800_e4101 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign4800_e4103;

        let (assign4810_e4112,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard120 != 0.0)) {
        (p.p418,)
    } else {
        (var_axacl_i,)
    }
};
        var_axacl_i = assign4810_e4112;

        *var_axacl_i_slot = var_axacl_i;
        *var_axaco_i_slot = var_axaco_i;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_db0_slot = var_cfac1_t_db0;
        *var_cfac1_t_db1_slot = var_cfac1_t_db1;
        *var_cfac1_t_db2_slot = var_cfac1_t_db2;
        *var_cfac1_t_db3_slot = var_cfac1_t_db3;
        *var_cfac1_t_dn0_slot = var_cfac1_t_dn0;
        *var_cfac1_t_dn1_slot = var_cfac1_t_dn1;
        *var_cfac1_t_dn2_slot = var_cfac1_t_dn2;
        *var_cfac1_t_dn3_slot = var_cfac1_t_dn3;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn5_slot = var_cfac1_t_dn5;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_db0_slot = var_cfac2_t_db0;
        *var_cfac2_t_db1_slot = var_cfac2_t_db1;
        *var_cfac2_t_db2_slot = var_cfac2_t_db2;
        *var_cfac2_t_db3_slot = var_cfac2_t_db3;
        *var_cfac2_t_dn0_slot = var_cfac2_t_dn0;
        *var_cfac2_t_dn1_slot = var_cfac2_t_dn1;
        *var_cfac2_t_dn2_slot = var_cfac2_t_dn2;
        *var_cfac2_t_dn3_slot = var_cfac2_t_dn3;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn5_slot = var_cfac2_t_dn5;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_db0_slot = var_cfac_p_db0;
        *var_cfac_p_db1_slot = var_cfac_p_db1;
        *var_cfac_p_db2_slot = var_cfac_p_db2;
        *var_cfac_p_db3_slot = var_cfac_p_db3;
        *var_cfac_p_dn0_slot = var_cfac_p_dn0;
        *var_cfac_p_dn1_slot = var_cfac_p_dn1;
        *var_cfac_p_dn2_slot = var_cfac_p_dn2;
        *var_cfac_p_dn3_slot = var_cfac_p_dn3;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn5_slot = var_cfac_p_dn5;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_cfacl_i_slot = var_cfacl_i;
        *var_cfaclexp_i_slot = var_cfaclexp_i;
        *var_cfacw_i_slot = var_cfacw_i;
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
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_db0_slot = var_thesatac_p_db0;
        *var_thesatac_p_db1_slot = var_thesatac_p_db1;
        *var_thesatac_p_db2_slot = var_thesatac_p_db2;
        *var_thesatac_p_db3_slot = var_thesatac_p_db3;
        *var_thesatac_p_dn0_slot = var_thesatac_p_dn0;
        *var_thesatac_p_dn1_slot = var_thesatac_p_dn1;
        *var_thesatac_p_dn2_slot = var_thesatac_p_dn2;
        *var_thesatac_p_dn3_slot = var_thesatac_p_dn3;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn5_slot = var_thesatac_p_dn5;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_db0_slot = var_thesatac_t_db0;
        *var_thesatac_t_db1_slot = var_thesatac_t_db1;
        *var_thesatac_t_db2_slot = var_thesatac_t_db2;
        *var_thesatac_t_db3_slot = var_thesatac_t_db3;
        *var_thesatac_t_dn0_slot = var_thesatac_t_dn0;
        *var_thesatac_t_dn1_slot = var_thesatac_t_dn1;
        *var_thesatac_t_dn2_slot = var_thesatac_t_dn2;
        *var_thesatac_t_dn3_slot = var_thesatac_t_dn3;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn5_slot = var_thesatac_t_dn5;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_thesatacl_i_slot = var_thesatacl_i;
        *var_thesataclexp_i_slot = var_thesataclexp_i;
        *var_thesataclw_i_slot = var_thesataclw_i;
        *var_thesataco_i_slot = var_thesataco_i;
        *var_thesatacw_i_slot = var_thesatacw_i;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_db0_slot = var_vfbac2_t_db0;
        *var_vfbac2_t_db1_slot = var_vfbac2_t_db1;
        *var_vfbac2_t_db2_slot = var_vfbac2_t_db2;
        *var_vfbac2_t_db3_slot = var_vfbac2_t_db3;
        *var_vfbac2_t_dn0_slot = var_vfbac2_t_dn0;
        *var_vfbac2_t_dn1_slot = var_vfbac2_t_dn1;
        *var_vfbac2_t_dn2_slot = var_vfbac2_t_dn2;
        *var_vfbac2_t_dn3_slot = var_vfbac2_t_dn3;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn5_slot = var_vfbac2_t_dn5;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }

    pub(super) fn stamp_transient_block_12(
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
        var_lphy_db0: f64,
        var_lphy_db1: f64,
        var_lphy_db2: f64,
        var_lphy_db3: f64,
        var_lphy_dn0: f64,
        var_lphy_dn1: f64,
        var_lphy_dn2: f64,
        var_lphy_dn3: f64,
        var_lphy_dn4: f64,
        var_lphy_dn5: f64,
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
        var_wphy_db0: f64,
        var_wphy_db1: f64,
        var_wphy_db2: f64,
        var_wphy_db3: f64,
        var_wphy_dn0: f64,
        var_wphy_dn1: f64,
        var_wphy_dn2: f64,
        var_wphy_dn3: f64,
        var_wphy_dn4: f64,
        var_wphy_dn5: f64,
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
        var_cfr_i_db0_slot: &mut f64,
        var_cfr_i_db1_slot: &mut f64,
        var_cfr_i_db2_slot: &mut f64,
        var_cfr_i_db3_slot: &mut f64,
        var_cfr_i_dn0_slot: &mut f64,
        var_cfr_i_dn1_slot: &mut f64,
        var_cfr_i_dn2_slot: &mut f64,
        var_cfr_i_dn3_slot: &mut f64,
        var_cfr_i_dn4_slot: &mut f64,
        var_cfr_i_dn5_slot: &mut f64,
        var_cfr_i_dn6_slot: &mut f64,
        var_cfr_i_dn7_slot: &mut f64,
        var_cfr_i_dn8_slot: &mut f64,
        var_cfr_i_dn9_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_db0_slot: &mut f64,
        var_cfr_p_db1_slot: &mut f64,
        var_cfr_p_db2_slot: &mut f64,
        var_cfr_p_db3_slot: &mut f64,
        var_cfr_p_dn0_slot: &mut f64,
        var_cfr_p_dn1_slot: &mut f64,
        var_cfr_p_dn2_slot: &mut f64,
        var_cfr_p_dn3_slot: &mut f64,
        var_cfr_p_dn4_slot: &mut f64,
        var_cfr_p_dn5_slot: &mut f64,
        var_cfr_p_dn6_slot: &mut f64,
        var_cfr_p_dn7_slot: &mut f64,
        var_cfr_p_dn8_slot: &mut f64,
        var_cfr_p_dn9_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_db0_slot: &mut f64,
        var_cfrd_i_db1_slot: &mut f64,
        var_cfrd_i_db2_slot: &mut f64,
        var_cfrd_i_db3_slot: &mut f64,
        var_cfrd_i_dn0_slot: &mut f64,
        var_cfrd_i_dn1_slot: &mut f64,
        var_cfrd_i_dn2_slot: &mut f64,
        var_cfrd_i_dn3_slot: &mut f64,
        var_cfrd_i_dn4_slot: &mut f64,
        var_cfrd_i_dn5_slot: &mut f64,
        var_cfrd_i_dn6_slot: &mut f64,
        var_cfrd_i_dn7_slot: &mut f64,
        var_cfrd_i_dn8_slot: &mut f64,
        var_cfrd_i_dn9_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_db0_slot: &mut f64,
        var_cfrd_p_db1_slot: &mut f64,
        var_cfrd_p_db2_slot: &mut f64,
        var_cfrd_p_db3_slot: &mut f64,
        var_cfrd_p_dn0_slot: &mut f64,
        var_cfrd_p_dn1_slot: &mut f64,
        var_cfrd_p_dn2_slot: &mut f64,
        var_cfrd_p_dn3_slot: &mut f64,
        var_cfrd_p_dn4_slot: &mut f64,
        var_cfrd_p_dn5_slot: &mut f64,
        var_cfrd_p_dn6_slot: &mut f64,
        var_cfrd_p_dn7_slot: &mut f64,
        var_cfrd_p_dn8_slot: &mut f64,
        var_cfrd_p_dn9_slot: &mut f64,
        var_cov_i_slot: &mut f64,
        var_cov_i_db0_slot: &mut f64,
        var_cov_i_db1_slot: &mut f64,
        var_cov_i_db2_slot: &mut f64,
        var_cov_i_db3_slot: &mut f64,
        var_cov_i_dn0_slot: &mut f64,
        var_cov_i_dn1_slot: &mut f64,
        var_cov_i_dn2_slot: &mut f64,
        var_cov_i_dn3_slot: &mut f64,
        var_cov_i_dn4_slot: &mut f64,
        var_cov_i_dn5_slot: &mut f64,
        var_cov_i_dn6_slot: &mut f64,
        var_cov_i_dn7_slot: &mut f64,
        var_cov_i_dn8_slot: &mut f64,
        var_cov_i_dn9_slot: &mut f64,
        var_covd_i_slot: &mut f64,
        var_covd_i_db0_slot: &mut f64,
        var_covd_i_db1_slot: &mut f64,
        var_covd_i_db2_slot: &mut f64,
        var_covd_i_db3_slot: &mut f64,
        var_covd_i_dn0_slot: &mut f64,
        var_covd_i_dn1_slot: &mut f64,
        var_covd_i_dn2_slot: &mut f64,
        var_covd_i_dn3_slot: &mut f64,
        var_covd_i_dn4_slot: &mut f64,
        var_covd_i_dn5_slot: &mut f64,
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
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_db0_slot: &mut f64,
        var_temp1_db1_slot: &mut f64,
        var_temp1_db2_slot: &mut f64,
        var_temp1_db3_slot: &mut f64,
        var_temp1_dn0_slot: &mut f64,
        var_temp1_dn1_slot: &mut f64,
        var_temp1_dn2_slot: &mut f64,
        var_temp1_dn3_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
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
        let mut var_cfr_i_db0: f64 = *var_cfr_i_db0_slot;
        let mut var_cfr_i_db1: f64 = *var_cfr_i_db1_slot;
        let mut var_cfr_i_db2: f64 = *var_cfr_i_db2_slot;
        let mut var_cfr_i_db3: f64 = *var_cfr_i_db3_slot;
        let mut var_cfr_i_dn0: f64 = *var_cfr_i_dn0_slot;
        let mut var_cfr_i_dn1: f64 = *var_cfr_i_dn1_slot;
        let mut var_cfr_i_dn2: f64 = *var_cfr_i_dn2_slot;
        let mut var_cfr_i_dn3: f64 = *var_cfr_i_dn3_slot;
        let mut var_cfr_i_dn4: f64 = *var_cfr_i_dn4_slot;
        let mut var_cfr_i_dn5: f64 = *var_cfr_i_dn5_slot;
        let mut var_cfr_i_dn6: f64 = *var_cfr_i_dn6_slot;
        let mut var_cfr_i_dn7: f64 = *var_cfr_i_dn7_slot;
        let mut var_cfr_i_dn8: f64 = *var_cfr_i_dn8_slot;
        let mut var_cfr_i_dn9: f64 = *var_cfr_i_dn9_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_db0: f64 = *var_cfr_p_db0_slot;
        let mut var_cfr_p_db1: f64 = *var_cfr_p_db1_slot;
        let mut var_cfr_p_db2: f64 = *var_cfr_p_db2_slot;
        let mut var_cfr_p_db3: f64 = *var_cfr_p_db3_slot;
        let mut var_cfr_p_dn0: f64 = *var_cfr_p_dn0_slot;
        let mut var_cfr_p_dn1: f64 = *var_cfr_p_dn1_slot;
        let mut var_cfr_p_dn2: f64 = *var_cfr_p_dn2_slot;
        let mut var_cfr_p_dn3: f64 = *var_cfr_p_dn3_slot;
        let mut var_cfr_p_dn4: f64 = *var_cfr_p_dn4_slot;
        let mut var_cfr_p_dn5: f64 = *var_cfr_p_dn5_slot;
        let mut var_cfr_p_dn6: f64 = *var_cfr_p_dn6_slot;
        let mut var_cfr_p_dn7: f64 = *var_cfr_p_dn7_slot;
        let mut var_cfr_p_dn8: f64 = *var_cfr_p_dn8_slot;
        let mut var_cfr_p_dn9: f64 = *var_cfr_p_dn9_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_db0: f64 = *var_cfrd_i_db0_slot;
        let mut var_cfrd_i_db1: f64 = *var_cfrd_i_db1_slot;
        let mut var_cfrd_i_db2: f64 = *var_cfrd_i_db2_slot;
        let mut var_cfrd_i_db3: f64 = *var_cfrd_i_db3_slot;
        let mut var_cfrd_i_dn0: f64 = *var_cfrd_i_dn0_slot;
        let mut var_cfrd_i_dn1: f64 = *var_cfrd_i_dn1_slot;
        let mut var_cfrd_i_dn2: f64 = *var_cfrd_i_dn2_slot;
        let mut var_cfrd_i_dn3: f64 = *var_cfrd_i_dn3_slot;
        let mut var_cfrd_i_dn4: f64 = *var_cfrd_i_dn4_slot;
        let mut var_cfrd_i_dn5: f64 = *var_cfrd_i_dn5_slot;
        let mut var_cfrd_i_dn6: f64 = *var_cfrd_i_dn6_slot;
        let mut var_cfrd_i_dn7: f64 = *var_cfrd_i_dn7_slot;
        let mut var_cfrd_i_dn8: f64 = *var_cfrd_i_dn8_slot;
        let mut var_cfrd_i_dn9: f64 = *var_cfrd_i_dn9_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_db0: f64 = *var_cfrd_p_db0_slot;
        let mut var_cfrd_p_db1: f64 = *var_cfrd_p_db1_slot;
        let mut var_cfrd_p_db2: f64 = *var_cfrd_p_db2_slot;
        let mut var_cfrd_p_db3: f64 = *var_cfrd_p_db3_slot;
        let mut var_cfrd_p_dn0: f64 = *var_cfrd_p_dn0_slot;
        let mut var_cfrd_p_dn1: f64 = *var_cfrd_p_dn1_slot;
        let mut var_cfrd_p_dn2: f64 = *var_cfrd_p_dn2_slot;
        let mut var_cfrd_p_dn3: f64 = *var_cfrd_p_dn3_slot;
        let mut var_cfrd_p_dn4: f64 = *var_cfrd_p_dn4_slot;
        let mut var_cfrd_p_dn5: f64 = *var_cfrd_p_dn5_slot;
        let mut var_cfrd_p_dn6: f64 = *var_cfrd_p_dn6_slot;
        let mut var_cfrd_p_dn7: f64 = *var_cfrd_p_dn7_slot;
        let mut var_cfrd_p_dn8: f64 = *var_cfrd_p_dn8_slot;
        let mut var_cfrd_p_dn9: f64 = *var_cfrd_p_dn9_slot;
        let mut var_cov_i: f64 = *var_cov_i_slot;
        let mut var_cov_i_db0: f64 = *var_cov_i_db0_slot;
        let mut var_cov_i_db1: f64 = *var_cov_i_db1_slot;
        let mut var_cov_i_db2: f64 = *var_cov_i_db2_slot;
        let mut var_cov_i_db3: f64 = *var_cov_i_db3_slot;
        let mut var_cov_i_dn0: f64 = *var_cov_i_dn0_slot;
        let mut var_cov_i_dn1: f64 = *var_cov_i_dn1_slot;
        let mut var_cov_i_dn2: f64 = *var_cov_i_dn2_slot;
        let mut var_cov_i_dn3: f64 = *var_cov_i_dn3_slot;
        let mut var_cov_i_dn4: f64 = *var_cov_i_dn4_slot;
        let mut var_cov_i_dn5: f64 = *var_cov_i_dn5_slot;
        let mut var_cov_i_dn6: f64 = *var_cov_i_dn6_slot;
        let mut var_cov_i_dn7: f64 = *var_cov_i_dn7_slot;
        let mut var_cov_i_dn8: f64 = *var_cov_i_dn8_slot;
        let mut var_cov_i_dn9: f64 = *var_cov_i_dn9_slot;
        let mut var_covd_i: f64 = *var_covd_i_slot;
        let mut var_covd_i_db0: f64 = *var_covd_i_db0_slot;
        let mut var_covd_i_db1: f64 = *var_covd_i_db1_slot;
        let mut var_covd_i_db2: f64 = *var_covd_i_db2_slot;
        let mut var_covd_i_db3: f64 = *var_covd_i_db3_slot;
        let mut var_covd_i_dn0: f64 = *var_covd_i_dn0_slot;
        let mut var_covd_i_dn1: f64 = *var_covd_i_dn1_slot;
        let mut var_covd_i_dn2: f64 = *var_covd_i_dn2_slot;
        let mut var_covd_i_dn3: f64 = *var_covd_i_dn3_slot;
        let mut var_covd_i_dn4: f64 = *var_covd_i_dn4_slot;
        let mut var_covd_i_dn5: f64 = *var_covd_i_dn5_slot;
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
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_db0: f64 = *var_temp1_db0_slot;
        let mut var_temp1_db1: f64 = *var_temp1_db1_slot;
        let mut var_temp1_db2: f64 = *var_temp1_db2_slot;
        let mut var_temp1_db3: f64 = *var_temp1_db3_slot;
        let mut var_temp1_dn0: f64 = *var_temp1_dn0_slot;
        let mut var_temp1_dn1: f64 = *var_temp1_dn1_slot;
        let mut var_temp1_dn2: f64 = *var_temp1_dn2_slot;
        let mut var_temp1_dn3: f64 = *var_temp1_dn3_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;

        let (assign4820_e4119,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p302,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4820_e4119;

        let assign4830_e4121: f64 = if param_given[419] { 1.0 } else { 0.0 };
        let assign4830_e4123: f64 = if assign4830_e4121 == 1.0 { 1.0 } else { 0.0 };
        var_guard121 = assign4830_e4123;

        let (assign4840_e4132,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard121 != 0.0)) {
        (p.p419,)
    } else {
        (var_axaclexp_i,)
    }
};
        var_axaclexp_i = assign4840_e4132;

        let (assign4850_e4139,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p303,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4850_e4139;

        let assign4860_e4141: f64 = if param_given[420] { 1.0 } else { 0.0 };
        let assign4860_e4143: f64 = if assign4860_e4141 == 1.0 { 1.0 } else { 0.0 };
        var_guard122 = assign4860_e4143;

        let (assign4870_e4152,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard122 != 0.0)) {
        (p.p420,)
    } else {
        (var_axacl2_i,)
    }
};
        var_axacl2_i = assign4870_e4152;

        let (assign4880_e4159,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p304,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4880_e4159;

        let assign4890_e4161: f64 = if param_given[421] { 1.0 } else { 0.0 };
        let assign4890_e4163: f64 = if assign4890_e4161 == 1.0 { 1.0 } else { 0.0 };
        var_guard123 = assign4890_e4163;

        let (assign4900_e4172,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard123 != 0.0)) {
        (p.p421,)
    } else {
        (var_axaclexp2_i,)
    }
};
        var_axaclexp2_i = assign4900_e4172;

        let (assign4910_e4195,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4910_e4182: f64 = (var_ile).powf(var_axaclexp_i);
        let assign4910_e4183: f64 = (var_axacl_i * assign4910_e4182);
        let assign4910_e4188: f64 = (var_ile).powf(var_axaclexp2_i);
        let assign4910_e4189: f64 = (var_axacl2_i * assign4910_e4188);
        let assign4910_e4190: f64 = (1.0 + assign4910_e4189);
        let assign4910_e4191: f64 = (assign4910_e4183 / assign4910_e4190);
        let assign4910_e4192: f64 = (1.0 + assign4910_e4191);
        let assign4910_e4193: f64 = (var_axaco_i / assign4910_e4192);
        (assign4910_e4193,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign4910_e4195;

        let (assign4920_e4206,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign4920_e4202: f64 = (var_axac_p).max(1.0);
        let assign4920_e4204: f64 = (assign4920_e4202).min(16.0);
        (assign4920_e4204,)
    } else {
        (var_axac_i,)
    }
};
        var_axac_i = assign4920_e4206;

        let (assign4930_e4213,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p305,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4930_e4213;

        let assign4940_e4215: f64 = if param_given[422] { 1.0 } else { 0.0 };
        let assign4940_e4217: f64 = if assign4940_e4215 == 1.0 { 1.0 } else { 0.0 };
        var_guard124 = assign4940_e4217;

        let (assign4950_e4226,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard124 != 0.0)) {
        (p.p422,)
    } else {
        (var_alpacl1_i,)
    }
};
        var_alpacl1_i = assign4950_e4226;

        let (assign4960_e4233,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p306,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign4960_e4233;

        let assign4970_e4235: f64 = if param_given[423] { 1.0 } else { 0.0 };
        let assign4970_e4237: f64 = if assign4970_e4235 == 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign4970_e4237;

        let (assign4980_e4246,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard125 != 0.0)) {
        (p.p423,)
    } else {
        (var_alpaclexp_i,)
    }
};
        var_alpaclexp_i = assign4980_e4246;

        let (assign4990_e4253,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p307,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign4990_e4253;

        let assign5000_e4255: f64 = if param_given[424] { 1.0 } else { 0.0 };
        let assign5000_e4257: f64 = if assign5000_e4255 == 1.0 { 1.0 } else { 0.0 };
        var_guard126 = assign5000_e4257;

        let (assign5010_e4266,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard126 != 0.0)) {
        (p.p424,)
    } else {
        (var_alpacl2_i,)
    }
};
        var_alpacl2_i = assign5010_e4266;

        let (assign5020_e4273,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p308,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5020_e4273;

        let assign5030_e4275: f64 = if param_given[425] { 1.0 } else { 0.0 };
        let assign5030_e4277: f64 = if assign5030_e4275 == 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign5030_e4277;

        let (assign5040_e4286,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard127 != 0.0)) {
        (p.p425,)
    } else {
        (var_alpaclexp2_i,)
    }
};
        var_alpaclexp2_i = assign5040_e4286;

        let (assign5050_e4293,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        (p.p309,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5050_e4293;

        let assign5060_e4295: f64 = if param_given[426] { 1.0 } else { 0.0 };
        let assign5060_e4297: f64 = if assign5060_e4295 == 1.0 { 1.0 } else { 0.0 };
        var_guard128 = assign5060_e4297;

        let (assign5070_e4306,) = {
    if (((var_guard83 == 0.0) && (var_guard98 != 0.0)) && (var_guard128 != 0.0)) {
        (p.p426,)
    } else {
        (var_alpacw_i,)
    }
};
        var_alpacw_i = assign5070_e4306;

        let (assign5080_e4331,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5080_e4314: f64 = (var_ile).powf(var_alpaclexp_i);
        let assign5080_e4315: f64 = (var_alpacl1_i * assign5080_e4314);
        let assign5080_e4319: f64 = (var_alpacw_i * var_iwe);
        let assign5080_e4320: f64 = (1.0 + assign5080_e4319);
        let assign5080_e4321: f64 = (assign5080_e4315 * assign5080_e4320);
        let assign5080_e4326: f64 = (var_ile).powf(var_alpaclexp2_i);
        let assign5080_e4327: f64 = (var_alpacl2_i * assign5080_e4326);
        let assign5080_e4328: f64 = (1.0 + assign5080_e4327);
        let assign5080_e4329: f64 = (assign5080_e4321 / assign5080_e4328);
        (assign5080_e4329,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign5080_e4331;

        let (assign5090_e4340,) = {
    if ((var_guard83 == 0.0) && (var_guard98 != 0.0)) {
        let assign5090_e4338: f64 = (var_alpac_p).max(0.0);
        (assign5090_e4338,)
    } else {
        (var_alpac_i,)
    }
};
        var_alpac_i = assign5090_e4340;

        let (assign5100_e4349, assign5100_e4349_d_n0, assign5100_e4349_d_n1, assign5100_e4349_d_n2, assign5100_e4349_d_n3, assign5100_e4349_d_n4, assign5100_e4349_d_n5, assign5100_e4349_d_n6, assign5100_e4349_d_n7, assign5100_e4349_d_n8, assign5100_e4349_d_n9, assign5100_e4349_d_b0, assign5100_e4349_d_b1, assign5100_e4349_d_b2, assign5100_e4349_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5100_e4345: f64 = (3.45313e-11 / var_tox1_i);
        let assign5100_e4347: f64 = (assign5100_e4345 * var_wecv);
        (assign5100_e4347, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5100_e4349;
        var_temp_dn0 = assign5100_e4349_d_n0;
        var_temp_dn1 = assign5100_e4349_d_n1;
        var_temp_dn2 = assign5100_e4349_d_n2;
        var_temp_dn3 = assign5100_e4349_d_n3;
        var_temp_dn4 = assign5100_e4349_d_n4;
        var_temp_dn5 = assign5100_e4349_d_n5;
        var_temp_dn6 = assign5100_e4349_d_n6;
        var_temp_dn7 = assign5100_e4349_d_n7;
        var_temp_dn8 = assign5100_e4349_d_n8;
        var_temp_dn9 = assign5100_e4349_d_n9;
        var_temp_db0 = assign5100_e4349_d_b0;
        var_temp_db1 = assign5100_e4349_d_b1;
        var_temp_db2 = assign5100_e4349_d_b2;
        var_temp_db3 = assign5100_e4349_d_b3;

        let (assign5110_e4356, assign5110_e4356_d_n0, assign5110_e4356_d_n1, assign5110_e4356_d_n2, assign5110_e4356_d_n3, assign5110_e4356_d_n4, assign5110_e4356_d_n5, assign5110_e4356_d_n6, assign5110_e4356_d_n7, assign5110_e4356_d_n8, assign5110_e4356_d_n9, assign5110_e4356_d_b0, assign5110_e4356_d_b1, assign5110_e4356_d_b2, assign5110_e4356_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5110_e4354: f64 = (var_temp * p.p427);
        (assign5110_e4354, (var_temp_dn0 * p.p427), (var_temp_dn1 * p.p427), (var_temp_dn2 * p.p427), (var_temp_dn3 * p.p427), (var_temp_dn4 * p.p427), (var_temp_dn5 * p.p427), (var_temp_dn6 * p.p427), (var_temp_dn7 * p.p427), (var_temp_dn8 * p.p427), (var_temp_dn9 * p.p427), (var_temp_db0 * p.p427), (var_temp_db1 * p.p427), (var_temp_db2 * p.p427), (var_temp_db3 * p.p427),)
    } else {
        (var_cov_i, var_cov_i_dn0, var_cov_i_dn1, var_cov_i_dn2, var_cov_i_dn3, var_cov_i_dn4, var_cov_i_dn5, var_cov_i_dn6, var_cov_i_dn7, var_cov_i_dn8, var_cov_i_dn9, var_cov_i_db0, var_cov_i_db1, var_cov_i_db2, var_cov_i_db3,)
    }
};
        var_cov_i = assign5110_e4356;
        var_cov_i_dn0 = assign5110_e4356_d_n0;
        var_cov_i_dn1 = assign5110_e4356_d_n1;
        var_cov_i_dn2 = assign5110_e4356_d_n2;
        var_cov_i_dn3 = assign5110_e4356_d_n3;
        var_cov_i_dn4 = assign5110_e4356_d_n4;
        var_cov_i_dn5 = assign5110_e4356_d_n5;
        var_cov_i_dn6 = assign5110_e4356_d_n6;
        var_cov_i_dn7 = assign5110_e4356_d_n7;
        var_cov_i_dn8 = assign5110_e4356_d_n8;
        var_cov_i_dn9 = assign5110_e4356_d_n9;
        var_cov_i_db0 = assign5110_e4356_d_b0;
        var_cov_i_db1 = assign5110_e4356_d_b1;
        var_cov_i_db2 = assign5110_e4356_d_b2;
        var_cov_i_db3 = assign5110_e4356_d_b3;

        let (assign5120_e4363, assign5120_e4363_d_n0, assign5120_e4363_d_n1, assign5120_e4363_d_n2, assign5120_e4363_d_n3, assign5120_e4363_d_n4, assign5120_e4363_d_n5, assign5120_e4363_d_n6, assign5120_e4363_d_n7, assign5120_e4363_d_n8, assign5120_e4363_d_n9, assign5120_e4363_d_b0, assign5120_e4363_d_b1, assign5120_e4363_d_b2, assign5120_e4363_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5120_e4361: f64 = (var_temp * p.p428);
        (assign5120_e4361, (var_temp_dn0 * p.p428), (var_temp_dn1 * p.p428), (var_temp_dn2 * p.p428), (var_temp_dn3 * p.p428), (var_temp_dn4 * p.p428), (var_temp_dn5 * p.p428), (var_temp_dn6 * p.p428), (var_temp_dn7 * p.p428), (var_temp_dn8 * p.p428), (var_temp_dn9 * p.p428), (var_temp_db0 * p.p428), (var_temp_db1 * p.p428), (var_temp_db2 * p.p428), (var_temp_db3 * p.p428),)
    } else {
        (var_covd_i, var_covd_i_dn0, var_covd_i_dn1, var_covd_i_dn2, var_covd_i_dn3, var_covd_i_dn4, var_covd_i_dn5, var_covd_i_dn6, var_covd_i_dn7, var_covd_i_dn8, var_covd_i_dn9, var_covd_i_db0, var_covd_i_db1, var_covd_i_db2, var_covd_i_db3,)
    }
};
        var_covd_i = assign5120_e4363;
        var_covd_i_dn0 = assign5120_e4363_d_n0;
        var_covd_i_dn1 = assign5120_e4363_d_n1;
        var_covd_i_dn2 = assign5120_e4363_d_n2;
        var_covd_i_dn3 = assign5120_e4363_d_n3;
        var_covd_i_dn4 = assign5120_e4363_d_n4;
        var_covd_i_dn5 = assign5120_e4363_d_n5;
        var_covd_i_dn6 = assign5120_e4363_d_n6;
        var_covd_i_dn7 = assign5120_e4363_d_n7;
        var_covd_i_dn8 = assign5120_e4363_d_n8;
        var_covd_i_dn9 = assign5120_e4363_d_n9;
        var_covd_i_db0 = assign5120_e4363_d_b0;
        var_covd_i_db1 = assign5120_e4363_d_b1;
        var_covd_i_db2 = assign5120_e4363_d_b2;
        var_covd_i_db3 = assign5120_e4363_d_b3;

        let (assign5130_e4378,) = {
    if (var_guard83 == 0.0) {
        let assign5130_e4370: f64 = (p.p430 * var_wen);
        let assign5130_e4372: f64 = (assign5130_e4370 / var_wecv);
        let assign5130_e4373: f64 = (1.0 + assign5130_e4372);
        let assign5130_e4375: f64 = (assign5130_e4373).max(0.001);
        let assign5130_e4376: f64 = (p.p429 / assign5130_e4375);
        (assign5130_e4376,)
    } else {
        (var_covdl_i,)
    }
};
        var_covdl_i = assign5130_e4378;

        let (assign5140_e4383,) = {
    if (var_guard83 == 0.0) {
        (p.p431,)
    } else {
        (var_covdlb_i,)
    }
};
        var_covdlb_i = assign5140_e4383;

        let (assign5150_e4388,) = {
    if (var_guard83 == 0.0) {
        (p.p432,)
    } else {
        (var_dvfbov_i,)
    }
};
        var_dvfbov_i = assign5150_e4388;

        let (assign5160_e4397, assign5160_e4397_d_n0, assign5160_e4397_d_n1, assign5160_e4397_d_n2, assign5160_e4397_d_n3, assign5160_e4397_d_n4, assign5160_e4397_d_n5, assign5160_e4397_d_n6, assign5160_e4397_d_n7, assign5160_e4397_d_n8, assign5160_e4397_d_n9, assign5160_e4397_d_b0, assign5160_e4397_d_b1, assign5160_e4397_d_b2, assign5160_e4397_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5160_e4394: f64 = (p.p435 * var_wphy);
        let assign5160_e4395: f64 = (p.p433 + assign5160_e4394);
        (assign5160_e4395, (p.p435 * var_wphy_dn0), (p.p435 * var_wphy_dn1), (p.p435 * var_wphy_dn2), (p.p435 * var_wphy_dn3), (p.p435 * var_wphy_dn4), (p.p435 * var_wphy_dn5), (p.p435 * var_wphy_dn6), (p.p435 * var_wphy_dn7), (p.p435 * var_wphy_dn8), (p.p435 * var_wphy_dn9), (p.p435 * var_wphy_db0), (p.p435 * var_wphy_db1), (p.p435 * var_wphy_db2), (p.p435 * var_wphy_db3),)
    } else {
        (var_cfr_p, var_cfr_p_dn0, var_cfr_p_dn1, var_cfr_p_dn2, var_cfr_p_dn3, var_cfr_p_dn4, var_cfr_p_dn5, var_cfr_p_dn6, var_cfr_p_dn7, var_cfr_p_dn8, var_cfr_p_dn9, var_cfr_p_db0, var_cfr_p_db1, var_cfr_p_db2, var_cfr_p_db3,)
    }
};
        var_cfr_p = assign5160_e4397;
        var_cfr_p_dn0 = assign5160_e4397_d_n0;
        var_cfr_p_dn1 = assign5160_e4397_d_n1;
        var_cfr_p_dn2 = assign5160_e4397_d_n2;
        var_cfr_p_dn3 = assign5160_e4397_d_n3;
        var_cfr_p_dn4 = assign5160_e4397_d_n4;
        var_cfr_p_dn5 = assign5160_e4397_d_n5;
        var_cfr_p_dn6 = assign5160_e4397_d_n6;
        var_cfr_p_dn7 = assign5160_e4397_d_n7;
        var_cfr_p_dn8 = assign5160_e4397_d_n8;
        var_cfr_p_dn9 = assign5160_e4397_d_n9;
        var_cfr_p_db0 = assign5160_e4397_d_b0;
        var_cfr_p_db1 = assign5160_e4397_d_b1;
        var_cfr_p_db2 = assign5160_e4397_d_b2;
        var_cfr_p_db3 = assign5160_e4397_d_b3;

        let (assign5170_e4404, assign5170_e4404_d_n0, assign5170_e4404_d_n1, assign5170_e4404_d_n2, assign5170_e4404_d_n3, assign5170_e4404_d_n4, assign5170_e4404_d_n5, assign5170_e4404_d_n6, assign5170_e4404_d_n7, assign5170_e4404_d_n8, assign5170_e4404_d_n9, assign5170_e4404_d_b0, assign5170_e4404_d_b1, assign5170_e4404_d_b2, assign5170_e4404_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5170_e4402: f64 = (var_cfr_p).max(0.0);
        (assign5170_e4402, if var_cfr_p >= 0.0 { var_cfr_p_dn0 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn1 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn2 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn3 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn4 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn5 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn6 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn7 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn8 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_dn9 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_db0 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_db1 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_db2 } else { 0.0 }, if var_cfr_p >= 0.0 { var_cfr_p_db3 } else { 0.0 },)
    } else {
        (var_cfr_i, var_cfr_i_dn0, var_cfr_i_dn1, var_cfr_i_dn2, var_cfr_i_dn3, var_cfr_i_dn4, var_cfr_i_dn5, var_cfr_i_dn6, var_cfr_i_dn7, var_cfr_i_dn8, var_cfr_i_dn9, var_cfr_i_db0, var_cfr_i_db1, var_cfr_i_db2, var_cfr_i_db3,)
    }
};
        var_cfr_i = assign5170_e4404;
        var_cfr_i_dn0 = assign5170_e4404_d_n0;
        var_cfr_i_dn1 = assign5170_e4404_d_n1;
        var_cfr_i_dn2 = assign5170_e4404_d_n2;
        var_cfr_i_dn3 = assign5170_e4404_d_n3;
        var_cfr_i_dn4 = assign5170_e4404_d_n4;
        var_cfr_i_dn5 = assign5170_e4404_d_n5;
        var_cfr_i_dn6 = assign5170_e4404_d_n6;
        var_cfr_i_dn7 = assign5170_e4404_d_n7;
        var_cfr_i_dn8 = assign5170_e4404_d_n8;
        var_cfr_i_dn9 = assign5170_e4404_d_n9;
        var_cfr_i_db0 = assign5170_e4404_d_b0;
        var_cfr_i_db1 = assign5170_e4404_d_b1;
        var_cfr_i_db2 = assign5170_e4404_d_b2;
        var_cfr_i_db3 = assign5170_e4404_d_b3;

        let (assign5180_e4413, assign5180_e4413_d_n0, assign5180_e4413_d_n1, assign5180_e4413_d_n2, assign5180_e4413_d_n3, assign5180_e4413_d_n4, assign5180_e4413_d_n5, assign5180_e4413_d_n6, assign5180_e4413_d_n7, assign5180_e4413_d_n8, assign5180_e4413_d_n9, assign5180_e4413_d_b0, assign5180_e4413_d_b1, assign5180_e4413_d_b2, assign5180_e4413_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5180_e4410: f64 = (p.p436 * var_wphy);
        let assign5180_e4411: f64 = (p.p434 + assign5180_e4410);
        (assign5180_e4411, (p.p436 * var_wphy_dn0), (p.p436 * var_wphy_dn1), (p.p436 * var_wphy_dn2), (p.p436 * var_wphy_dn3), (p.p436 * var_wphy_dn4), (p.p436 * var_wphy_dn5), (p.p436 * var_wphy_dn6), (p.p436 * var_wphy_dn7), (p.p436 * var_wphy_dn8), (p.p436 * var_wphy_dn9), (p.p436 * var_wphy_db0), (p.p436 * var_wphy_db1), (p.p436 * var_wphy_db2), (p.p436 * var_wphy_db3),)
    } else {
        (var_cfrd_p, var_cfrd_p_dn0, var_cfrd_p_dn1, var_cfrd_p_dn2, var_cfrd_p_dn3, var_cfrd_p_dn4, var_cfrd_p_dn5, var_cfrd_p_dn6, var_cfrd_p_dn7, var_cfrd_p_dn8, var_cfrd_p_dn9, var_cfrd_p_db0, var_cfrd_p_db1, var_cfrd_p_db2, var_cfrd_p_db3,)
    }
};
        var_cfrd_p = assign5180_e4413;
        var_cfrd_p_dn0 = assign5180_e4413_d_n0;
        var_cfrd_p_dn1 = assign5180_e4413_d_n1;
        var_cfrd_p_dn2 = assign5180_e4413_d_n2;
        var_cfrd_p_dn3 = assign5180_e4413_d_n3;
        var_cfrd_p_dn4 = assign5180_e4413_d_n4;
        var_cfrd_p_dn5 = assign5180_e4413_d_n5;
        var_cfrd_p_dn6 = assign5180_e4413_d_n6;
        var_cfrd_p_dn7 = assign5180_e4413_d_n7;
        var_cfrd_p_dn8 = assign5180_e4413_d_n8;
        var_cfrd_p_dn9 = assign5180_e4413_d_n9;
        var_cfrd_p_db0 = assign5180_e4413_d_b0;
        var_cfrd_p_db1 = assign5180_e4413_d_b1;
        var_cfrd_p_db2 = assign5180_e4413_d_b2;
        var_cfrd_p_db3 = assign5180_e4413_d_b3;

        let (assign5190_e4420, assign5190_e4420_d_n0, assign5190_e4420_d_n1, assign5190_e4420_d_n2, assign5190_e4420_d_n3, assign5190_e4420_d_n4, assign5190_e4420_d_n5, assign5190_e4420_d_n6, assign5190_e4420_d_n7, assign5190_e4420_d_n8, assign5190_e4420_d_n9, assign5190_e4420_d_b0, assign5190_e4420_d_b1, assign5190_e4420_d_b2, assign5190_e4420_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5190_e4418: f64 = (var_cfrd_p).max(0.0);
        (assign5190_e4418, if var_cfrd_p >= 0.0 { var_cfrd_p_dn0 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn1 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn2 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn3 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn4 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn5 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn6 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn7 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn8 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_dn9 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_db0 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_db1 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_db2 } else { 0.0 }, if var_cfrd_p >= 0.0 { var_cfrd_p_db3 } else { 0.0 },)
    } else {
        (var_cfrd_i, var_cfrd_i_dn0, var_cfrd_i_dn1, var_cfrd_i_dn2, var_cfrd_i_dn3, var_cfrd_i_dn4, var_cfrd_i_dn5, var_cfrd_i_dn6, var_cfrd_i_dn7, var_cfrd_i_dn8, var_cfrd_i_dn9, var_cfrd_i_db0, var_cfrd_i_db1, var_cfrd_i_db2, var_cfrd_i_db3,)
    }
};
        var_cfrd_i = assign5190_e4420;
        var_cfrd_i_dn0 = assign5190_e4420_d_n0;
        var_cfrd_i_dn1 = assign5190_e4420_d_n1;
        var_cfrd_i_dn2 = assign5190_e4420_d_n2;
        var_cfrd_i_dn3 = assign5190_e4420_d_n3;
        var_cfrd_i_dn4 = assign5190_e4420_d_n4;
        var_cfrd_i_dn5 = assign5190_e4420_d_n5;
        var_cfrd_i_dn6 = assign5190_e4420_d_n6;
        var_cfrd_i_dn7 = assign5190_e4420_d_n7;
        var_cfrd_i_dn8 = assign5190_e4420_d_n8;
        var_cfrd_i_dn9 = assign5190_e4420_d_n9;
        var_cfrd_i_db0 = assign5190_e4420_d_b0;
        var_cfrd_i_db1 = assign5190_e4420_d_b1;
        var_cfrd_i_db2 = assign5190_e4420_d_b2;
        var_cfrd_i_db3 = assign5190_e4420_d_b3;

        let (assign5200_e4433,) = {
    if (var_guard83 == 0.0) {
        let assign5200_e4425: f64 = (p.p437 * var_epsch);
        let assign5200_e4427: f64 = (assign5200_e4425 * var_tsi_i);
        let assign5200_e4429: f64 = (assign5200_e4427 * var_we);
        let assign5200_e4431: f64 = (assign5200_e4429 / var_le);
        (assign5200_e4431,)
    } else {
        (var_csd_i,)
    }
};
        var_csd_i = assign5200_e4433;

        let (assign5210_e4438,) = {
    if (var_guard83 == 0.0) {
        (p.p438,)
    } else {
        (var_csdbp_i,)
    }
};
        var_csdbp_i = assign5210_e4438;

        let (assign5220_e4459, assign5220_e4459_d_n0, assign5220_e4459_d_n1, assign5220_e4459_d_n2, assign5220_e4459_d_n3, assign5220_e4459_d_n4, assign5220_e4459_d_n5, assign5220_e4459_d_n6, assign5220_e4459_d_n7, assign5220_e4459_d_n8, assign5220_e4459_d_n9, assign5220_e4459_d_b0, assign5220_e4459_d_b1, assign5220_e4459_d_b2, assign5220_e4459_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5220_e4444: f64 = (p.p440 * var_lphy);
        let assign5220_e4445: f64 = (1.0 + assign5220_e4444);
        let assign5220_e4448: f64 = (p.p441 * var_wphy);
        let assign5220_e4449: f64 = (assign5220_e4445 + assign5220_e4448);
        let assign5220_e4452: f64 = (p.p442 * var_lphy);
        let assign5220_e4454: f64 = (assign5220_e4452 * var_wphy);
        let assign5220_e4455: f64 = (assign5220_e4449 + assign5220_e4454);
        let assign5220_e4457: f64 = (assign5220_e4455).max(1e-10);
        (assign5220_e4457, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn0) + (p.p441 * var_wphy_dn0)) + (((p.p442 * var_lphy_dn0) * var_wphy) + (assign5220_e4452 * var_wphy_dn0))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn1) + (p.p441 * var_wphy_dn1)) + (((p.p442 * var_lphy_dn1) * var_wphy) + (assign5220_e4452 * var_wphy_dn1))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn2) + (p.p441 * var_wphy_dn2)) + (((p.p442 * var_lphy_dn2) * var_wphy) + (assign5220_e4452 * var_wphy_dn2))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn3) + (p.p441 * var_wphy_dn3)) + (((p.p442 * var_lphy_dn3) * var_wphy) + (assign5220_e4452 * var_wphy_dn3))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn4) + (p.p441 * var_wphy_dn4)) + (((p.p442 * var_lphy_dn4) * var_wphy) + (assign5220_e4452 * var_wphy_dn4))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn5) + (p.p441 * var_wphy_dn5)) + (((p.p442 * var_lphy_dn5) * var_wphy) + (assign5220_e4452 * var_wphy_dn5))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn6) + (p.p441 * var_wphy_dn6)) + (((p.p442 * var_lphy_dn6) * var_wphy) + (assign5220_e4452 * var_wphy_dn6))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn7) + (p.p441 * var_wphy_dn7)) + (((p.p442 * var_lphy_dn7) * var_wphy) + (assign5220_e4452 * var_wphy_dn7))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn8) + (p.p441 * var_wphy_dn8)) + (((p.p442 * var_lphy_dn8) * var_wphy) + (assign5220_e4452 * var_wphy_dn8))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_dn9) + (p.p441 * var_wphy_dn9)) + (((p.p442 * var_lphy_dn9) * var_wphy) + (assign5220_e4452 * var_wphy_dn9))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_db0) + (p.p441 * var_wphy_db0)) + (((p.p442 * var_lphy_db0) * var_wphy) + (assign5220_e4452 * var_wphy_db0))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_db1) + (p.p441 * var_wphy_db1)) + (((p.p442 * var_lphy_db1) * var_wphy) + (assign5220_e4452 * var_wphy_db1))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_db2) + (p.p441 * var_wphy_db2)) + (((p.p442 * var_lphy_db2) * var_wphy) + (assign5220_e4452 * var_wphy_db2))) } else { 0.0 }, if assign5220_e4455 >= 1e-10 { (((p.p440 * var_lphy_db3) + (p.p441 * var_wphy_db3)) + (((p.p442 * var_lphy_db3) * var_wphy) + (assign5220_e4452 * var_wphy_db3))) } else { 0.0 },)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5220_e4459;
        var_temp_dn0 = assign5220_e4459_d_n0;
        var_temp_dn1 = assign5220_e4459_d_n1;
        var_temp_dn2 = assign5220_e4459_d_n2;
        var_temp_dn3 = assign5220_e4459_d_n3;
        var_temp_dn4 = assign5220_e4459_d_n4;
        var_temp_dn5 = assign5220_e4459_d_n5;
        var_temp_dn6 = assign5220_e4459_d_n6;
        var_temp_dn7 = assign5220_e4459_d_n7;
        var_temp_dn8 = assign5220_e4459_d_n8;
        var_temp_dn9 = assign5220_e4459_d_n9;
        var_temp_db0 = assign5220_e4459_d_b0;
        var_temp_db1 = assign5220_e4459_d_b1;
        var_temp_db2 = assign5220_e4459_d_b2;
        var_temp_db3 = assign5220_e4459_d_b3;

        let (assign5230_e4464, assign5230_e4464_d_n0, assign5230_e4464_d_n1, assign5230_e4464_d_n2, assign5230_e4464_d_n3, assign5230_e4464_d_n4, assign5230_e4464_d_n5, assign5230_e4464_d_n6, assign5230_e4464_d_n7, assign5230_e4464_d_n8, assign5230_e4464_d_n9, assign5230_e4464_d_b0, assign5230_e4464_d_b1, assign5230_e4464_d_b2, assign5230_e4464_d_b3,) = {
    if (var_guard83 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3,)
    }
};
        var_temp1 = assign5230_e4464;
        var_temp1_dn0 = assign5230_e4464_d_n0;
        var_temp1_dn1 = assign5230_e4464_d_n1;
        var_temp1_dn2 = assign5230_e4464_d_n2;
        var_temp1_dn3 = assign5230_e4464_d_n3;
        var_temp1_dn4 = assign5230_e4464_d_n4;
        var_temp1_dn5 = assign5230_e4464_d_n5;
        var_temp1_dn6 = assign5230_e4464_d_n6;
        var_temp1_dn7 = assign5230_e4464_d_n7;
        var_temp1_dn8 = assign5230_e4464_d_n8;
        var_temp1_dn9 = assign5230_e4464_d_n9;
        var_temp1_db0 = assign5230_e4464_d_b0;
        var_temp1_db1 = assign5230_e4464_d_b1;
        var_temp1_db2 = assign5230_e4464_d_b2;
        var_temp1_db3 = assign5230_e4464_d_b3;

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
        *var_cfr_i_db0_slot = var_cfr_i_db0;
        *var_cfr_i_db1_slot = var_cfr_i_db1;
        *var_cfr_i_db2_slot = var_cfr_i_db2;
        *var_cfr_i_db3_slot = var_cfr_i_db3;
        *var_cfr_i_dn0_slot = var_cfr_i_dn0;
        *var_cfr_i_dn1_slot = var_cfr_i_dn1;
        *var_cfr_i_dn2_slot = var_cfr_i_dn2;
        *var_cfr_i_dn3_slot = var_cfr_i_dn3;
        *var_cfr_i_dn4_slot = var_cfr_i_dn4;
        *var_cfr_i_dn5_slot = var_cfr_i_dn5;
        *var_cfr_i_dn6_slot = var_cfr_i_dn6;
        *var_cfr_i_dn7_slot = var_cfr_i_dn7;
        *var_cfr_i_dn8_slot = var_cfr_i_dn8;
        *var_cfr_i_dn9_slot = var_cfr_i_dn9;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_db0_slot = var_cfr_p_db0;
        *var_cfr_p_db1_slot = var_cfr_p_db1;
        *var_cfr_p_db2_slot = var_cfr_p_db2;
        *var_cfr_p_db3_slot = var_cfr_p_db3;
        *var_cfr_p_dn0_slot = var_cfr_p_dn0;
        *var_cfr_p_dn1_slot = var_cfr_p_dn1;
        *var_cfr_p_dn2_slot = var_cfr_p_dn2;
        *var_cfr_p_dn3_slot = var_cfr_p_dn3;
        *var_cfr_p_dn4_slot = var_cfr_p_dn4;
        *var_cfr_p_dn5_slot = var_cfr_p_dn5;
        *var_cfr_p_dn6_slot = var_cfr_p_dn6;
        *var_cfr_p_dn7_slot = var_cfr_p_dn7;
        *var_cfr_p_dn8_slot = var_cfr_p_dn8;
        *var_cfr_p_dn9_slot = var_cfr_p_dn9;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_db0_slot = var_cfrd_i_db0;
        *var_cfrd_i_db1_slot = var_cfrd_i_db1;
        *var_cfrd_i_db2_slot = var_cfrd_i_db2;
        *var_cfrd_i_db3_slot = var_cfrd_i_db3;
        *var_cfrd_i_dn0_slot = var_cfrd_i_dn0;
        *var_cfrd_i_dn1_slot = var_cfrd_i_dn1;
        *var_cfrd_i_dn2_slot = var_cfrd_i_dn2;
        *var_cfrd_i_dn3_slot = var_cfrd_i_dn3;
        *var_cfrd_i_dn4_slot = var_cfrd_i_dn4;
        *var_cfrd_i_dn5_slot = var_cfrd_i_dn5;
        *var_cfrd_i_dn6_slot = var_cfrd_i_dn6;
        *var_cfrd_i_dn7_slot = var_cfrd_i_dn7;
        *var_cfrd_i_dn8_slot = var_cfrd_i_dn8;
        *var_cfrd_i_dn9_slot = var_cfrd_i_dn9;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_db0_slot = var_cfrd_p_db0;
        *var_cfrd_p_db1_slot = var_cfrd_p_db1;
        *var_cfrd_p_db2_slot = var_cfrd_p_db2;
        *var_cfrd_p_db3_slot = var_cfrd_p_db3;
        *var_cfrd_p_dn0_slot = var_cfrd_p_dn0;
        *var_cfrd_p_dn1_slot = var_cfrd_p_dn1;
        *var_cfrd_p_dn2_slot = var_cfrd_p_dn2;
        *var_cfrd_p_dn3_slot = var_cfrd_p_dn3;
        *var_cfrd_p_dn4_slot = var_cfrd_p_dn4;
        *var_cfrd_p_dn5_slot = var_cfrd_p_dn5;
        *var_cfrd_p_dn6_slot = var_cfrd_p_dn6;
        *var_cfrd_p_dn7_slot = var_cfrd_p_dn7;
        *var_cfrd_p_dn8_slot = var_cfrd_p_dn8;
        *var_cfrd_p_dn9_slot = var_cfrd_p_dn9;
        *var_cov_i_slot = var_cov_i;
        *var_cov_i_db0_slot = var_cov_i_db0;
        *var_cov_i_db1_slot = var_cov_i_db1;
        *var_cov_i_db2_slot = var_cov_i_db2;
        *var_cov_i_db3_slot = var_cov_i_db3;
        *var_cov_i_dn0_slot = var_cov_i_dn0;
        *var_cov_i_dn1_slot = var_cov_i_dn1;
        *var_cov_i_dn2_slot = var_cov_i_dn2;
        *var_cov_i_dn3_slot = var_cov_i_dn3;
        *var_cov_i_dn4_slot = var_cov_i_dn4;
        *var_cov_i_dn5_slot = var_cov_i_dn5;
        *var_cov_i_dn6_slot = var_cov_i_dn6;
        *var_cov_i_dn7_slot = var_cov_i_dn7;
        *var_cov_i_dn8_slot = var_cov_i_dn8;
        *var_cov_i_dn9_slot = var_cov_i_dn9;
        *var_covd_i_slot = var_covd_i;
        *var_covd_i_db0_slot = var_covd_i_db0;
        *var_covd_i_db1_slot = var_covd_i_db1;
        *var_covd_i_db2_slot = var_covd_i_db2;
        *var_covd_i_db3_slot = var_covd_i_db3;
        *var_covd_i_dn0_slot = var_covd_i_dn0;
        *var_covd_i_dn1_slot = var_covd_i_dn1;
        *var_covd_i_dn2_slot = var_covd_i_dn2;
        *var_covd_i_dn3_slot = var_covd_i_dn3;
        *var_covd_i_dn4_slot = var_covd_i_dn4;
        *var_covd_i_dn5_slot = var_covd_i_dn5;
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
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_db0_slot = var_temp1_db0;
        *var_temp1_db1_slot = var_temp1_db1;
        *var_temp1_db2_slot = var_temp1_db2;
        *var_temp1_db3_slot = var_temp1_db3;
        *var_temp1_dn0_slot = var_temp1_dn0;
        *var_temp1_dn1_slot = var_temp1_dn1;
        *var_temp1_dn2_slot = var_temp1_dn2;
        *var_temp1_dn3_slot = var_temp1_dn3;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
    }

    pub(super) fn stamp_transient_block_13(
        p: &Parameters,
        var_betn_p: f64,
        var_guard83: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_db0_slot: &mut f64,
        var_cth_i_db1_slot: &mut f64,
        var_cth_i_db2_slot: &mut f64,
        var_cth_i_db3_slot: &mut f64,
        var_cth_i_dn0_slot: &mut f64,
        var_cth_i_dn1_slot: &mut f64,
        var_cth_i_dn2_slot: &mut f64,
        var_cth_i_dn3_slot: &mut f64,
        var_cth_i_dn4_slot: &mut f64,
        var_cth_i_dn5_slot: &mut f64,
        var_cth_i_dn6_slot: &mut f64,
        var_cth_i_dn7_slot: &mut f64,
        var_cth_i_dn8_slot: &mut f64,
        var_cth_i_dn9_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_db0_slot: &mut f64,
        var_cth_p_db1_slot: &mut f64,
        var_cth_p_db2_slot: &mut f64,
        var_cth_p_db3_slot: &mut f64,
        var_cth_p_dn0_slot: &mut f64,
        var_cth_p_dn1_slot: &mut f64,
        var_cth_p_dn2_slot: &mut f64,
        var_cth_p_dn3_slot: &mut f64,
        var_cth_p_dn4_slot: &mut f64,
        var_cth_p_dn5_slot: &mut f64,
        var_cth_p_dn6_slot: &mut f64,
        var_cth_p_dn7_slot: &mut f64,
        var_cth_p_dn8_slot: &mut f64,
        var_cth_p_dn9_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fntexc_i_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_nfa_i_slot: &mut f64,
        var_nfa_p_slot: &mut f64,
        var_nfb_i_slot: &mut f64,
        var_nfc_i_slot: &mut f64,
        var_nfe_i_slot: &mut f64,
        var_nfeb_i_slot: &mut f64,
        var_rth_p_slot: &mut f64,
        var_rth_p_db0_slot: &mut f64,
        var_rth_p_db1_slot: &mut f64,
        var_rth_p_db2_slot: &mut f64,
        var_rth_p_db3_slot: &mut f64,
        var_rth_p_dn0_slot: &mut f64,
        var_rth_p_dn1_slot: &mut f64,
        var_rth_p_dn2_slot: &mut f64,
        var_rth_p_dn3_slot: &mut f64,
        var_rth_p_dn4_slot: &mut f64,
        var_rth_p_dn5_slot: &mut f64,
        var_rth_p_dn6_slot: &mut f64,
        var_rth_p_dn7_slot: &mut f64,
        var_rth_p_dn8_slot: &mut f64,
        var_rth_p_dn9_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_rth_t_db0_slot: &mut f64,
        var_rth_t_db1_slot: &mut f64,
        var_rth_t_db2_slot: &mut f64,
        var_rth_t_db3_slot: &mut f64,
        var_rth_t_dn0_slot: &mut f64,
        var_rth_t_dn1_slot: &mut f64,
        var_rth_t_dn2_slot: &mut f64,
        var_rth_t_dn3_slot: &mut f64,
        var_rth_t_dn4_slot: &mut f64,
        var_rth_t_dn5_slot: &mut f64,
        var_rth_t_dn6_slot: &mut f64,
        var_rth_t_dn7_slot: &mut f64,
        var_rth_t_dn8_slot: &mut f64,
        var_rth_t_dn9_slot: &mut f64,
        var_strth_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_db0_slot: &mut f64,
        var_temp1_db1_slot: &mut f64,
        var_temp1_db2_slot: &mut f64,
        var_temp1_db3_slot: &mut f64,
        var_temp1_dn0_slot: &mut f64,
        var_temp1_dn1_slot: &mut f64,
        var_temp1_dn2_slot: &mut f64,
        var_temp1_dn3_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn5_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_db0_slot: &mut f64,
        var_temp2_db1_slot: &mut f64,
        var_temp2_db2_slot: &mut f64,
        var_temp2_db3_slot: &mut f64,
        var_temp2_dn0_slot: &mut f64,
        var_temp2_dn1_slot: &mut f64,
        var_temp2_dn2_slot: &mut f64,
        var_temp2_dn3_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn5_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp3_slot: &mut f64,
        var_temp3_db0_slot: &mut f64,
        var_temp3_db1_slot: &mut f64,
        var_temp3_db2_slot: &mut f64,
        var_temp3_db3_slot: &mut f64,
        var_temp3_dn0_slot: &mut f64,
        var_temp3_dn1_slot: &mut f64,
        var_temp3_dn2_slot: &mut f64,
        var_temp3_dn3_slot: &mut f64,
        var_temp3_dn4_slot: &mut f64,
        var_temp3_dn5_slot: &mut f64,
        var_temp3_dn6_slot: &mut f64,
        var_temp3_dn7_slot: &mut f64,
        var_temp3_dn8_slot: &mut f64,
        var_temp3_dn9_slot: &mut f64,
        var_temp4_slot: &mut f64,
        var_temp4_db0_slot: &mut f64,
        var_temp4_db1_slot: &mut f64,
        var_temp4_db2_slot: &mut f64,
        var_temp4_db3_slot: &mut f64,
        var_temp4_dn0_slot: &mut f64,
        var_temp4_dn1_slot: &mut f64,
        var_temp4_dn2_slot: &mut f64,
        var_temp4_dn3_slot: &mut f64,
        var_temp4_dn4_slot: &mut f64,
        var_temp4_dn5_slot: &mut f64,
        var_temp4_dn6_slot: &mut f64,
        var_temp4_dn7_slot: &mut f64,
        var_temp4_dn8_slot: &mut f64,
        var_temp4_dn9_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_db0_slot: &mut f64,
        var_tmpa_db1_slot: &mut f64,
        var_tmpa_db2_slot: &mut f64,
        var_tmpa_db3_slot: &mut f64,
        var_tmpa_dn0_slot: &mut f64,
        var_tmpa_dn1_slot: &mut f64,
        var_tmpa_dn2_slot: &mut f64,
        var_tmpa_dn3_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn5_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpb_slot: &mut f64,
    ) {
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_db0: f64 = *var_cth_i_db0_slot;
        let mut var_cth_i_db1: f64 = *var_cth_i_db1_slot;
        let mut var_cth_i_db2: f64 = *var_cth_i_db2_slot;
        let mut var_cth_i_db3: f64 = *var_cth_i_db3_slot;
        let mut var_cth_i_dn0: f64 = *var_cth_i_dn0_slot;
        let mut var_cth_i_dn1: f64 = *var_cth_i_dn1_slot;
        let mut var_cth_i_dn2: f64 = *var_cth_i_dn2_slot;
        let mut var_cth_i_dn3: f64 = *var_cth_i_dn3_slot;
        let mut var_cth_i_dn4: f64 = *var_cth_i_dn4_slot;
        let mut var_cth_i_dn5: f64 = *var_cth_i_dn5_slot;
        let mut var_cth_i_dn6: f64 = *var_cth_i_dn6_slot;
        let mut var_cth_i_dn7: f64 = *var_cth_i_dn7_slot;
        let mut var_cth_i_dn8: f64 = *var_cth_i_dn8_slot;
        let mut var_cth_i_dn9: f64 = *var_cth_i_dn9_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_db0: f64 = *var_cth_p_db0_slot;
        let mut var_cth_p_db1: f64 = *var_cth_p_db1_slot;
        let mut var_cth_p_db2: f64 = *var_cth_p_db2_slot;
        let mut var_cth_p_db3: f64 = *var_cth_p_db3_slot;
        let mut var_cth_p_dn0: f64 = *var_cth_p_dn0_slot;
        let mut var_cth_p_dn1: f64 = *var_cth_p_dn1_slot;
        let mut var_cth_p_dn2: f64 = *var_cth_p_dn2_slot;
        let mut var_cth_p_dn3: f64 = *var_cth_p_dn3_slot;
        let mut var_cth_p_dn4: f64 = *var_cth_p_dn4_slot;
        let mut var_cth_p_dn5: f64 = *var_cth_p_dn5_slot;
        let mut var_cth_p_dn6: f64 = *var_cth_p_dn6_slot;
        let mut var_cth_p_dn7: f64 = *var_cth_p_dn7_slot;
        let mut var_cth_p_dn8: f64 = *var_cth_p_dn8_slot;
        let mut var_cth_p_dn9: f64 = *var_cth_p_dn9_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fntexc_i: f64 = *var_fntexc_i_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_nfa_i: f64 = *var_nfa_i_slot;
        let mut var_nfa_p: f64 = *var_nfa_p_slot;
        let mut var_nfb_i: f64 = *var_nfb_i_slot;
        let mut var_nfc_i: f64 = *var_nfc_i_slot;
        let mut var_nfe_i: f64 = *var_nfe_i_slot;
        let mut var_nfeb_i: f64 = *var_nfeb_i_slot;
        let mut var_rth_p: f64 = *var_rth_p_slot;
        let mut var_rth_p_db0: f64 = *var_rth_p_db0_slot;
        let mut var_rth_p_db1: f64 = *var_rth_p_db1_slot;
        let mut var_rth_p_db2: f64 = *var_rth_p_db2_slot;
        let mut var_rth_p_db3: f64 = *var_rth_p_db3_slot;
        let mut var_rth_p_dn0: f64 = *var_rth_p_dn0_slot;
        let mut var_rth_p_dn1: f64 = *var_rth_p_dn1_slot;
        let mut var_rth_p_dn2: f64 = *var_rth_p_dn2_slot;
        let mut var_rth_p_dn3: f64 = *var_rth_p_dn3_slot;
        let mut var_rth_p_dn4: f64 = *var_rth_p_dn4_slot;
        let mut var_rth_p_dn5: f64 = *var_rth_p_dn5_slot;
        let mut var_rth_p_dn6: f64 = *var_rth_p_dn6_slot;
        let mut var_rth_p_dn7: f64 = *var_rth_p_dn7_slot;
        let mut var_rth_p_dn8: f64 = *var_rth_p_dn8_slot;
        let mut var_rth_p_dn9: f64 = *var_rth_p_dn9_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_rth_t_db0: f64 = *var_rth_t_db0_slot;
        let mut var_rth_t_db1: f64 = *var_rth_t_db1_slot;
        let mut var_rth_t_db2: f64 = *var_rth_t_db2_slot;
        let mut var_rth_t_db3: f64 = *var_rth_t_db3_slot;
        let mut var_rth_t_dn0: f64 = *var_rth_t_dn0_slot;
        let mut var_rth_t_dn1: f64 = *var_rth_t_dn1_slot;
        let mut var_rth_t_dn2: f64 = *var_rth_t_dn2_slot;
        let mut var_rth_t_dn3: f64 = *var_rth_t_dn3_slot;
        let mut var_rth_t_dn4: f64 = *var_rth_t_dn4_slot;
        let mut var_rth_t_dn5: f64 = *var_rth_t_dn5_slot;
        let mut var_rth_t_dn6: f64 = *var_rth_t_dn6_slot;
        let mut var_rth_t_dn7: f64 = *var_rth_t_dn7_slot;
        let mut var_rth_t_dn8: f64 = *var_rth_t_dn8_slot;
        let mut var_rth_t_dn9: f64 = *var_rth_t_dn9_slot;
        let mut var_strth_i: f64 = *var_strth_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_db0: f64 = *var_temp1_db0_slot;
        let mut var_temp1_db1: f64 = *var_temp1_db1_slot;
        let mut var_temp1_db2: f64 = *var_temp1_db2_slot;
        let mut var_temp1_db3: f64 = *var_temp1_db3_slot;
        let mut var_temp1_dn0: f64 = *var_temp1_dn0_slot;
        let mut var_temp1_dn1: f64 = *var_temp1_dn1_slot;
        let mut var_temp1_dn2: f64 = *var_temp1_dn2_slot;
        let mut var_temp1_dn3: f64 = *var_temp1_dn3_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn5: f64 = *var_temp1_dn5_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_db0: f64 = *var_temp2_db0_slot;
        let mut var_temp2_db1: f64 = *var_temp2_db1_slot;
        let mut var_temp2_db2: f64 = *var_temp2_db2_slot;
        let mut var_temp2_db3: f64 = *var_temp2_db3_slot;
        let mut var_temp2_dn0: f64 = *var_temp2_dn0_slot;
        let mut var_temp2_dn1: f64 = *var_temp2_dn1_slot;
        let mut var_temp2_dn2: f64 = *var_temp2_dn2_slot;
        let mut var_temp2_dn3: f64 = *var_temp2_dn3_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn5: f64 = *var_temp2_dn5_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp3: f64 = *var_temp3_slot;
        let mut var_temp3_db0: f64 = *var_temp3_db0_slot;
        let mut var_temp3_db1: f64 = *var_temp3_db1_slot;
        let mut var_temp3_db2: f64 = *var_temp3_db2_slot;
        let mut var_temp3_db3: f64 = *var_temp3_db3_slot;
        let mut var_temp3_dn0: f64 = *var_temp3_dn0_slot;
        let mut var_temp3_dn1: f64 = *var_temp3_dn1_slot;
        let mut var_temp3_dn2: f64 = *var_temp3_dn2_slot;
        let mut var_temp3_dn3: f64 = *var_temp3_dn3_slot;
        let mut var_temp3_dn4: f64 = *var_temp3_dn4_slot;
        let mut var_temp3_dn5: f64 = *var_temp3_dn5_slot;
        let mut var_temp3_dn6: f64 = *var_temp3_dn6_slot;
        let mut var_temp3_dn7: f64 = *var_temp3_dn7_slot;
        let mut var_temp3_dn8: f64 = *var_temp3_dn8_slot;
        let mut var_temp3_dn9: f64 = *var_temp3_dn9_slot;
        let mut var_temp4: f64 = *var_temp4_slot;
        let mut var_temp4_db0: f64 = *var_temp4_db0_slot;
        let mut var_temp4_db1: f64 = *var_temp4_db1_slot;
        let mut var_temp4_db2: f64 = *var_temp4_db2_slot;
        let mut var_temp4_db3: f64 = *var_temp4_db3_slot;
        let mut var_temp4_dn0: f64 = *var_temp4_dn0_slot;
        let mut var_temp4_dn1: f64 = *var_temp4_dn1_slot;
        let mut var_temp4_dn2: f64 = *var_temp4_dn2_slot;
        let mut var_temp4_dn3: f64 = *var_temp4_dn3_slot;
        let mut var_temp4_dn4: f64 = *var_temp4_dn4_slot;
        let mut var_temp4_dn5: f64 = *var_temp4_dn5_slot;
        let mut var_temp4_dn6: f64 = *var_temp4_dn6_slot;
        let mut var_temp4_dn7: f64 = *var_temp4_dn7_slot;
        let mut var_temp4_dn8: f64 = *var_temp4_dn8_slot;
        let mut var_temp4_dn9: f64 = *var_temp4_dn9_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_db0: f64 = *var_tmpa_db0_slot;
        let mut var_tmpa_db1: f64 = *var_tmpa_db1_slot;
        let mut var_tmpa_db2: f64 = *var_tmpa_db2_slot;
        let mut var_tmpa_db3: f64 = *var_tmpa_db3_slot;
        let mut var_tmpa_dn0: f64 = *var_tmpa_dn0_slot;
        let mut var_tmpa_dn1: f64 = *var_tmpa_dn1_slot;
        let mut var_tmpa_dn2: f64 = *var_tmpa_dn2_slot;
        let mut var_tmpa_dn3: f64 = *var_tmpa_dn3_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn5: f64 = *var_tmpa_dn5_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;

        let assign5240_e4471: f64 = if ((p.p29 > 1.0) && (p.p28 > 0.0)) { 1.0 } else { 0.0 };
        var_guard129 = assign5240_e4471;

        let (assign5250_e4483, assign5250_e4483_d_n0, assign5250_e4483_d_n1, assign5250_e4483_d_n2, assign5250_e4483_d_n3, assign5250_e4483_d_n4, assign5250_e4483_d_n5, assign5250_e4483_d_n6, assign5250_e4483_d_n7, assign5250_e4483_d_n8, assign5250_e4483_d_n9, assign5250_e4483_d_b0, assign5250_e4483_d_b1, assign5250_e4483_d_b2, assign5250_e4483_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5250_e4478: f64 = (p.p28 + p.p20);
        let assign5250_e4479: f64 = (-assign5250_e4478);
        let assign5250_e4481: f64 = (assign5250_e4479 / p.p445);
        (assign5250_e4481, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn0, var_temp2_dn1, var_temp2_dn2, var_temp2_dn3, var_temp2_dn4, var_temp2_dn5, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9, var_temp2_db0, var_temp2_db1, var_temp2_db2, var_temp2_db3,)
    }
};
        var_temp2 = assign5250_e4483;
        var_temp2_dn0 = assign5250_e4483_d_n0;
        var_temp2_dn1 = assign5250_e4483_d_n1;
        var_temp2_dn2 = assign5250_e4483_d_n2;
        var_temp2_dn3 = assign5250_e4483_d_n3;
        var_temp2_dn4 = assign5250_e4483_d_n4;
        var_temp2_dn5 = assign5250_e4483_d_n5;
        var_temp2_dn6 = assign5250_e4483_d_n6;
        var_temp2_dn7 = assign5250_e4483_d_n7;
        var_temp2_dn8 = assign5250_e4483_d_n8;
        var_temp2_dn9 = assign5250_e4483_d_n9;
        var_temp2_db0 = assign5250_e4483_d_b0;
        var_temp2_db1 = assign5250_e4483_d_b1;
        var_temp2_db2 = assign5250_e4483_d_b2;
        var_temp2_db3 = assign5250_e4483_d_b3;

        let assign5260_e4485: f64 = (var_temp2).abs();
        let assign5260_e4487: f64 = if assign5260_e4485 < 80.0 { 1.0 } else { 0.0 };
        var_guard130 = assign5260_e4487;

        let (assign5270_e4497, assign5270_e4497_d_n0, assign5270_e4497_d_n1, assign5270_e4497_d_n2, assign5270_e4497_d_n3, assign5270_e4497_d_n4, assign5270_e4497_d_n5, assign5270_e4497_d_n6, assign5270_e4497_d_n7, assign5270_e4497_d_n8, assign5270_e4497_d_n9, assign5270_e4497_d_b0, assign5270_e4497_d_b1, assign5270_e4497_d_b2, assign5270_e4497_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 != 0.0)) {
        let assign5270_e4495: f64 = (var_temp2).exp();
        (assign5270_e4495, (assign5270_e4495 * var_temp2_dn0), (assign5270_e4495 * var_temp2_dn1), (assign5270_e4495 * var_temp2_dn2), (assign5270_e4495 * var_temp2_dn3), (assign5270_e4495 * var_temp2_dn4), (assign5270_e4495 * var_temp2_dn5), (assign5270_e4495 * var_temp2_dn6), (assign5270_e4495 * var_temp2_dn7), (assign5270_e4495 * var_temp2_dn8), (assign5270_e4495 * var_temp2_dn9), (assign5270_e4495 * var_temp2_db0), (assign5270_e4495 * var_temp2_db1), (assign5270_e4495 * var_temp2_db2), (assign5270_e4495 * var_temp2_db3),)
    } else {
        (var_temp3, var_temp3_dn0, var_temp3_dn1, var_temp3_dn2, var_temp3_dn3, var_temp3_dn4, var_temp3_dn5, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9, var_temp3_db0, var_temp3_db1, var_temp3_db2, var_temp3_db3,)
    }
};
        var_temp3 = assign5270_e4497;
        var_temp3_dn0 = assign5270_e4497_d_n0;
        var_temp3_dn1 = assign5270_e4497_d_n1;
        var_temp3_dn2 = assign5270_e4497_d_n2;
        var_temp3_dn3 = assign5270_e4497_d_n3;
        var_temp3_dn4 = assign5270_e4497_d_n4;
        var_temp3_dn5 = assign5270_e4497_d_n5;
        var_temp3_dn6 = assign5270_e4497_d_n6;
        var_temp3_dn7 = assign5270_e4497_d_n7;
        var_temp3_dn8 = assign5270_e4497_d_n8;
        var_temp3_dn9 = assign5270_e4497_d_n9;
        var_temp3_db0 = assign5270_e4497_d_b0;
        var_temp3_db1 = assign5270_e4497_d_b1;
        var_temp3_db2 = assign5270_e4497_d_b2;
        var_temp3_db3 = assign5270_e4497_d_b3;

        let assign5280_e4500: f64 = (-80.0);
        let assign5280_e4501: f64 = if var_temp2 < assign5280_e4500 { 1.0 } else { 0.0 };
        var_guard131 = assign5280_e4501;

        let (assign5290_e4538, assign5290_e4538_d_n0, assign5290_e4538_d_n1, assign5290_e4538_d_n2, assign5290_e4538_d_n3, assign5290_e4538_d_n4, assign5290_e4538_d_n5, assign5290_e4538_d_n6, assign5290_e4538_d_n7, assign5290_e4538_d_n8, assign5290_e4538_d_n9, assign5290_e4538_d_b0, assign5290_e4538_d_b1, assign5290_e4538_d_b2, assign5290_e4538_d_b3,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 != 0.0)) {
        let assign5290_e4514: f64 = (-var_temp2);
        let assign5290_e4516: f64 = (assign5290_e4514 - 80.0);
        let assign5290_e4520: f64 = (-var_temp2);
        let assign5290_e4522: f64 = (assign5290_e4520 - 80.0);
        let assign5290_e4523: f64 = (0.5 * assign5290_e4522);
        let assign5290_e4526: f64 = (-var_temp2);
        let assign5290_e4528: f64 = (assign5290_e4526 - 80.0);
        let assign5290_e4530: f64 = (assign5290_e4528 * 0.3333333333333);
        let assign5290_e4531: f64 = (1.0 + assign5290_e4530);
        let assign5290_e4532: f64 = (assign5290_e4523 * assign5290_e4531);
        let assign5290_e4533: f64 = (1.0 + assign5290_e4532);
        let assign5290_e4534: f64 = (assign5290_e4516 * assign5290_e4533);
        let assign5290_e4535: f64 = (1.0 + assign5290_e4534);
        let assign5290_e4536: f64 = (1.80485e-35 / assign5290_e4535);
        (assign5290_e4536, (-((1.80485e-35 * (((-var_temp2_dn0) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn0)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn0) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn1) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn1)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn1) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn2) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn2)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn2) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn3) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn3)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn3) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn4) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn4)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn4) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn5) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn5)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn5) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn6) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn6)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn6) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn7) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn7)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn7) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn8) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn8)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn8) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_dn9) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_dn9)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_dn9) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_db0) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_db0)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_db0) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_db1) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_db1)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_db1) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_db2) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_db2)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_db2) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))), (-((1.80485e-35 * (((-var_temp2_db3) * assign5290_e4533) + (assign5290_e4516 * (((0.5 * (-var_temp2_db3)) * assign5290_e4531) + (assign5290_e4523 * ((-var_temp2_db3) * 0.3333333333333)))))) / (assign5290_e4535 * assign5290_e4535))),)
    } else {
        (var_temp3, var_temp3_dn0, var_temp3_dn1, var_temp3_dn2, var_temp3_dn3, var_temp3_dn4, var_temp3_dn5, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9, var_temp3_db0, var_temp3_db1, var_temp3_db2, var_temp3_db3,)
    }
};
        var_temp3 = assign5290_e4538;
        var_temp3_dn0 = assign5290_e4538_d_n0;
        var_temp3_dn1 = assign5290_e4538_d_n1;
        var_temp3_dn2 = assign5290_e4538_d_n2;
        var_temp3_dn3 = assign5290_e4538_d_n3;
        var_temp3_dn4 = assign5290_e4538_d_n4;
        var_temp3_dn5 = assign5290_e4538_d_n5;
        var_temp3_dn6 = assign5290_e4538_d_n6;
        var_temp3_dn7 = assign5290_e4538_d_n7;
        var_temp3_dn8 = assign5290_e4538_d_n8;
        var_temp3_dn9 = assign5290_e4538_d_n9;
        var_temp3_db0 = assign5290_e4538_d_b0;
        var_temp3_db1 = assign5290_e4538_d_b1;
        var_temp3_db2 = assign5290_e4538_d_b2;
        var_temp3_db3 = assign5290_e4538_d_b3;

        let (assign5300_e4573, assign5300_e4573_d_n0, assign5300_e4573_d_n1, assign5300_e4573_d_n2, assign5300_e4573_d_n3, assign5300_e4573_d_n4, assign5300_e4573_d_n5, assign5300_e4573_d_n6, assign5300_e4573_d_n7, assign5300_e4573_d_n8, assign5300_e4573_d_n9, assign5300_e4573_d_b0, assign5300_e4573_d_b1, assign5300_e4573_d_b2, assign5300_e4573_d_b3,) = {
    if ((((var_guard83 == 0.0) && (var_guard129 != 0.0)) && (var_guard130 == 0.0)) && (var_guard131 == 0.0)) {
        let assign5300_e4553: f64 = (var_temp2 - 80.0);
        let assign5300_e4558: f64 = (var_temp2 - 80.0);
        let assign5300_e4559: f64 = (0.5 * assign5300_e4558);
        let assign5300_e4563: f64 = (var_temp2 - 80.0);
        let assign5300_e4565: f64 = (assign5300_e4563 * 0.3333333333333);
        let assign5300_e4566: f64 = (1.0 + assign5300_e4565);
        let assign5300_e4567: f64 = (assign5300_e4559 * assign5300_e4566);
        let assign5300_e4568: f64 = (1.0 + assign5300_e4567);
        let assign5300_e4569: f64 = (assign5300_e4553 * assign5300_e4568);
        let assign5300_e4570: f64 = (1.0 + assign5300_e4569);
        let assign5300_e4571: f64 = (5.54062e34 * assign5300_e4570);
        (assign5300_e4571, (5.54062e34 * ((var_temp2_dn0 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn0) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn0 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn1 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn1) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn1 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn2 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn2) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn2 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn3 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn3) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn3 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn4 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn4) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn4 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn5 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn5) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn5 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn6 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn6) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn6 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn7 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn7) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn7 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn8 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn8) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn8 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_dn9 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_dn9) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_dn9 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_db0 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_db0) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_db0 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_db1 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_db1) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_db1 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_db2 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_db2) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_db2 * 0.3333333333333)))))), (5.54062e34 * ((var_temp2_db3 * assign5300_e4568) + (assign5300_e4553 * (((0.5 * var_temp2_db3) * assign5300_e4566) + (assign5300_e4559 * (var_temp2_db3 * 0.3333333333333)))))),)
    } else {
        (var_temp3, var_temp3_dn0, var_temp3_dn1, var_temp3_dn2, var_temp3_dn3, var_temp3_dn4, var_temp3_dn5, var_temp3_dn6, var_temp3_dn7, var_temp3_dn8, var_temp3_dn9, var_temp3_db0, var_temp3_db1, var_temp3_db2, var_temp3_db3,)
    }
};
        var_temp3 = assign5300_e4573;
        var_temp3_dn0 = assign5300_e4573_d_n0;
        var_temp3_dn1 = assign5300_e4573_d_n1;
        var_temp3_dn2 = assign5300_e4573_d_n2;
        var_temp3_dn3 = assign5300_e4573_d_n3;
        var_temp3_dn4 = assign5300_e4573_d_n4;
        var_temp3_dn5 = assign5300_e4573_d_n5;
        var_temp3_dn6 = assign5300_e4573_d_n6;
        var_temp3_dn7 = assign5300_e4573_d_n7;
        var_temp3_dn8 = assign5300_e4573_d_n8;
        var_temp3_dn9 = assign5300_e4573_d_n9;
        var_temp3_db0 = assign5300_e4573_d_b0;
        var_temp3_db1 = assign5300_e4573_d_b1;
        var_temp3_db2 = assign5300_e4573_d_b2;
        var_temp3_db3 = assign5300_e4573_d_b3;

        let (assign5310_e4582, assign5310_e4582_d_n0, assign5310_e4582_d_n1, assign5310_e4582_d_n2, assign5310_e4582_d_n3, assign5310_e4582_d_n4, assign5310_e4582_d_n5, assign5310_e4582_d_n6, assign5310_e4582_d_n7, assign5310_e4582_d_n8, assign5310_e4582_d_n9, assign5310_e4582_d_b0, assign5310_e4582_d_b1, assign5310_e4582_d_b2, assign5310_e4582_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5310_e4580: f64 = (1.0 - var_temp3);
        (assign5310_e4580, (-var_temp3_dn0), (-var_temp3_dn1), (-var_temp3_dn2), (-var_temp3_dn3), (-var_temp3_dn4), (-var_temp3_dn5), (-var_temp3_dn6), (-var_temp3_dn7), (-var_temp3_dn8), (-var_temp3_dn9), (-var_temp3_db0), (-var_temp3_db1), (-var_temp3_db2), (-var_temp3_db3),)
    } else {
        (var_temp4, var_temp4_dn0, var_temp4_dn1, var_temp4_dn2, var_temp4_dn3, var_temp4_dn4, var_temp4_dn5, var_temp4_dn6, var_temp4_dn7, var_temp4_dn8, var_temp4_dn9, var_temp4_db0, var_temp4_db1, var_temp4_db2, var_temp4_db3,)
    }
};
        var_temp4 = assign5310_e4582;
        var_temp4_dn0 = assign5310_e4582_d_n0;
        var_temp4_dn1 = assign5310_e4582_d_n1;
        var_temp4_dn2 = assign5310_e4582_d_n2;
        var_temp4_dn3 = assign5310_e4582_d_n3;
        var_temp4_dn4 = assign5310_e4582_d_n4;
        var_temp4_dn5 = assign5310_e4582_d_n5;
        var_temp4_dn6 = assign5310_e4582_d_n6;
        var_temp4_dn7 = assign5310_e4582_d_n7;
        var_temp4_dn8 = assign5310_e4582_d_n8;
        var_temp4_dn9 = assign5310_e4582_d_n9;
        var_temp4_db0 = assign5310_e4582_d_b0;
        var_temp4_db1 = assign5310_e4582_d_b1;
        var_temp4_db2 = assign5310_e4582_d_b2;
        var_temp4_db3 = assign5310_e4582_d_b3;

        let (assign5320_e4607, assign5320_e4607_d_n0, assign5320_e4607_d_n1, assign5320_e4607_d_n2, assign5320_e4607_d_n3, assign5320_e4607_d_n4, assign5320_e4607_d_n5, assign5320_e4607_d_n6, assign5320_e4607_d_n7, assign5320_e4607_d_n8, assign5320_e4607_d_n9, assign5320_e4607_d_b0, assign5320_e4607_d_b1, assign5320_e4607_d_b2, assign5320_e4607_d_b3,) = {
    if ((var_guard83 == 0.0) && (var_guard129 != 0.0)) {
        let assign5320_e4589: f64 = (2.0 * p.p446);
        let assign5320_e4591: f64 = (assign5320_e4589 * var_temp3);
        let assign5320_e4596: f64 = (var_temp3).powf(p.p29);
        let assign5320_e4597: f64 = (1.0 - assign5320_e4596);
        let assign5320_e4599: f64 = (assign5320_e4597 / p.p29);
        let assign5320_e4600: f64 = (var_temp4 - assign5320_e4599);
        let assign5320_e4601: f64 = (assign5320_e4591 * assign5320_e4600);
        let assign5320_e4604: f64 = (var_temp4 * var_temp4);
        let assign5320_e4605: f64 = (assign5320_e4601 / assign5320_e4604);
        (assign5320_e4605, ((((((assign5320_e4589 * var_temp3_dn0) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn0 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn0)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn0 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn0 * var_temp4) + (var_temp4 * var_temp4_dn0)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn1) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn1 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn1)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn1 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn1 * var_temp4) + (var_temp4 * var_temp4_dn1)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn2) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn2 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn2)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn2 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn2 * var_temp4) + (var_temp4 * var_temp4_dn2)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn3) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn3 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn3)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn3 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn3 * var_temp4) + (var_temp4 * var_temp4_dn3)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn4) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn4 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn4)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn4 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn4 * var_temp4) + (var_temp4 * var_temp4_dn4)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn5) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn5 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn5)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn5 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn5 * var_temp4) + (var_temp4 * var_temp4_dn5)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn6) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn6 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn6)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn6 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn6 * var_temp4) + (var_temp4 * var_temp4_dn6)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn7) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn7 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn7)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn7 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn7 * var_temp4) + (var_temp4 * var_temp4_dn7)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn8) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn8 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn8)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn8 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn8 * var_temp4) + (var_temp4 * var_temp4_dn8)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_dn9) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_dn9 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_dn9)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_dn9 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_dn9 * var_temp4) + (var_temp4 * var_temp4_dn9)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_db0) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_db0 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_db0)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_db0 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_db0 * var_temp4) + (var_temp4 * var_temp4_db0)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_db1) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_db1 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_db1)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_db1 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_db1 * var_temp4) + (var_temp4 * var_temp4_db1)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_db2) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_db2 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_db2)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_db2 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_db2 * var_temp4) + (var_temp4 * var_temp4_db2)))) / (assign5320_e4604 * assign5320_e4604)), ((((((assign5320_e4589 * var_temp3_db3) * assign5320_e4600) + (assign5320_e4591 * (var_temp4_db3 - ((-if 0.0 == 0.0 && ((p.p29) as f64).is_finite() && ((p.p29) as f64).fract() == 0.0 { if p.p29 == 0.0 { 0.0 } else { (p.p29 * ((var_temp3).powf(p.p29 - 1.0) * var_temp3_db3)) } } else { (assign5320_e4596 * (p.p29 * (var_temp3_db3 / var_temp3))) }) / p.p29)))) * assign5320_e4604) - (assign5320_e4601 * ((var_temp4_db3 * var_temp4) + (var_temp4 * var_temp4_db3)))) / (assign5320_e4604 * assign5320_e4604)),)
    } else {
        (var_temp1, var_temp1_dn0, var_temp1_dn1, var_temp1_dn2, var_temp1_dn3, var_temp1_dn4, var_temp1_dn5, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9, var_temp1_db0, var_temp1_db1, var_temp1_db2, var_temp1_db3,)
    }
};
        var_temp1 = assign5320_e4607;
        var_temp1_dn0 = assign5320_e4607_d_n0;
        var_temp1_dn1 = assign5320_e4607_d_n1;
        var_temp1_dn2 = assign5320_e4607_d_n2;
        var_temp1_dn3 = assign5320_e4607_d_n3;
        var_temp1_dn4 = assign5320_e4607_d_n4;
        var_temp1_dn5 = assign5320_e4607_d_n5;
        var_temp1_dn6 = assign5320_e4607_d_n6;
        var_temp1_dn7 = assign5320_e4607_d_n7;
        var_temp1_dn8 = assign5320_e4607_d_n8;
        var_temp1_dn9 = assign5320_e4607_d_n9;
        var_temp1_db0 = assign5320_e4607_d_b0;
        var_temp1_db1 = assign5320_e4607_d_b1;
        var_temp1_db2 = assign5320_e4607_d_b2;
        var_temp1_db3 = assign5320_e4607_d_b3;

        let (assign5330_e4616, assign5330_e4616_d_n0, assign5330_e4616_d_n1, assign5330_e4616_d_n2, assign5330_e4616_d_n3, assign5330_e4616_d_n4, assign5330_e4616_d_n5, assign5330_e4616_d_n6, assign5330_e4616_d_n7, assign5330_e4616_d_n8, assign5330_e4616_d_n9, assign5330_e4616_d_b0, assign5330_e4616_d_b1, assign5330_e4616_d_b2, assign5330_e4616_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5330_e4613: f64 = (1.0 + var_temp1);
        let assign5330_e4614: f64 = (var_temp / assign5330_e4613);
        (assign5330_e4614, (((var_temp_dn0 * assign5330_e4613) - (var_temp * var_temp1_dn0)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn1 * assign5330_e4613) - (var_temp * var_temp1_dn1)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn2 * assign5330_e4613) - (var_temp * var_temp1_dn2)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn3 * assign5330_e4613) - (var_temp * var_temp1_dn3)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn4 * assign5330_e4613) - (var_temp * var_temp1_dn4)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn5 * assign5330_e4613) - (var_temp * var_temp1_dn5)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn6 * assign5330_e4613) - (var_temp * var_temp1_dn6)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn7 * assign5330_e4613) - (var_temp * var_temp1_dn7)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn8 * assign5330_e4613) - (var_temp * var_temp1_dn8)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_dn9 * assign5330_e4613) - (var_temp * var_temp1_dn9)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_db0 * assign5330_e4613) - (var_temp * var_temp1_db0)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_db1 * assign5330_e4613) - (var_temp * var_temp1_db1)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_db2 * assign5330_e4613) - (var_temp * var_temp1_db2)) / (assign5330_e4613 * assign5330_e4613)), (((var_temp_db3 * assign5330_e4613) - (var_temp * var_temp1_db3)) / (assign5330_e4613 * assign5330_e4613)),)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5330_e4616;
        var_temp_dn0 = assign5330_e4616_d_n0;
        var_temp_dn1 = assign5330_e4616_d_n1;
        var_temp_dn2 = assign5330_e4616_d_n2;
        var_temp_dn3 = assign5330_e4616_d_n3;
        var_temp_dn4 = assign5330_e4616_d_n4;
        var_temp_dn5 = assign5330_e4616_d_n5;
        var_temp_dn6 = assign5330_e4616_d_n6;
        var_temp_dn7 = assign5330_e4616_d_n7;
        var_temp_dn8 = assign5330_e4616_d_n8;
        var_temp_dn9 = assign5330_e4616_d_n9;
        var_temp_db0 = assign5330_e4616_d_b0;
        var_temp_db1 = assign5330_e4616_d_b1;
        var_temp_db2 = assign5330_e4616_d_b2;
        var_temp_db3 = assign5330_e4616_d_b3;

        let (assign5340_e4623, assign5340_e4623_d_n0, assign5340_e4623_d_n1, assign5340_e4623_d_n2, assign5340_e4623_d_n3, assign5340_e4623_d_n4, assign5340_e4623_d_n5, assign5340_e4623_d_n6, assign5340_e4623_d_n7, assign5340_e4623_d_n8, assign5340_e4623_d_n9, assign5340_e4623_d_b0, assign5340_e4623_d_b1, assign5340_e4623_d_b2, assign5340_e4623_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5340_e4621: f64 = (p.p439 / var_temp);
        (assign5340_e4621, (-((p.p439 * var_temp_dn0) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn1) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn2) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn3) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn4) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn5) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn6) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn7) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn8) / (var_temp * var_temp))), (-((p.p439 * var_temp_dn9) / (var_temp * var_temp))), (-((p.p439 * var_temp_db0) / (var_temp * var_temp))), (-((p.p439 * var_temp_db1) / (var_temp * var_temp))), (-((p.p439 * var_temp_db2) / (var_temp * var_temp))), (-((p.p439 * var_temp_db3) / (var_temp * var_temp))),)
    } else {
        (var_rth_p, var_rth_p_dn0, var_rth_p_dn1, var_rth_p_dn2, var_rth_p_dn3, var_rth_p_dn4, var_rth_p_dn5, var_rth_p_dn6, var_rth_p_dn7, var_rth_p_dn8, var_rth_p_dn9, var_rth_p_db0, var_rth_p_db1, var_rth_p_db2, var_rth_p_db3,)
    }
};
        var_rth_p = assign5340_e4623;
        var_rth_p_dn0 = assign5340_e4623_d_n0;
        var_rth_p_dn1 = assign5340_e4623_d_n1;
        var_rth_p_dn2 = assign5340_e4623_d_n2;
        var_rth_p_dn3 = assign5340_e4623_d_n3;
        var_rth_p_dn4 = assign5340_e4623_d_n4;
        var_rth_p_dn5 = assign5340_e4623_d_n5;
        var_rth_p_dn6 = assign5340_e4623_d_n6;
        var_rth_p_dn7 = assign5340_e4623_d_n7;
        var_rth_p_dn8 = assign5340_e4623_d_n8;
        var_rth_p_dn9 = assign5340_e4623_d_n9;
        var_rth_p_db0 = assign5340_e4623_d_b0;
        var_rth_p_db1 = assign5340_e4623_d_b1;
        var_rth_p_db2 = assign5340_e4623_d_b2;
        var_rth_p_db3 = assign5340_e4623_d_b3;

        let (assign5350_e4630, assign5350_e4630_d_n0, assign5350_e4630_d_n1, assign5350_e4630_d_n2, assign5350_e4630_d_n3, assign5350_e4630_d_n4, assign5350_e4630_d_n5, assign5350_e4630_d_n6, assign5350_e4630_d_n7, assign5350_e4630_d_n8, assign5350_e4630_d_n9, assign5350_e4630_d_b0, assign5350_e4630_d_b1, assign5350_e4630_d_b2, assign5350_e4630_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5350_e4628: f64 = (var_rth_p).max(1e-6);
        (assign5350_e4628, if var_rth_p >= 1e-6 { var_rth_p_dn0 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn1 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn2 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn3 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn4 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn5 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn6 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn7 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn8 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_dn9 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_db0 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_db1 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_db2 } else { 0.0 }, if var_rth_p >= 1e-6 { var_rth_p_db3 } else { 0.0 },)
    } else {
        (var_rth_t, var_rth_t_dn0, var_rth_t_dn1, var_rth_t_dn2, var_rth_t_dn3, var_rth_t_dn4, var_rth_t_dn5, var_rth_t_dn6, var_rth_t_dn7, var_rth_t_dn8, var_rth_t_dn9, var_rth_t_db0, var_rth_t_db1, var_rth_t_db2, var_rth_t_db3,)
    }
};
        var_rth_t = assign5350_e4630;
        var_rth_t_dn0 = assign5350_e4630_d_n0;
        var_rth_t_dn1 = assign5350_e4630_d_n1;
        var_rth_t_dn2 = assign5350_e4630_d_n2;
        var_rth_t_dn3 = assign5350_e4630_d_n3;
        var_rth_t_dn4 = assign5350_e4630_d_n4;
        var_rth_t_dn5 = assign5350_e4630_d_n5;
        var_rth_t_dn6 = assign5350_e4630_d_n6;
        var_rth_t_dn7 = assign5350_e4630_d_n7;
        var_rth_t_dn8 = assign5350_e4630_d_n8;
        var_rth_t_dn9 = assign5350_e4630_d_n9;
        var_rth_t_db0 = assign5350_e4630_d_b0;
        var_rth_t_db1 = assign5350_e4630_d_b1;
        var_rth_t_db2 = assign5350_e4630_d_b2;
        var_rth_t_db3 = assign5350_e4630_d_b3;

        let (assign5360_e4635,) = {
    if (var_guard83 == 0.0) {
        (p.p443,)
    } else {
        (var_strth_i,)
    }
};
        var_strth_i = assign5360_e4635;

        let (assign5370_e4642, assign5370_e4642_d_n0, assign5370_e4642_d_n1, assign5370_e4642_d_n2, assign5370_e4642_d_n3, assign5370_e4642_d_n4, assign5370_e4642_d_n5, assign5370_e4642_d_n6, assign5370_e4642_d_n7, assign5370_e4642_d_n8, assign5370_e4642_d_n9, assign5370_e4642_d_b0, assign5370_e4642_d_b1, assign5370_e4642_d_b2, assign5370_e4642_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5370_e4640: f64 = (p.p444 * var_temp);
        (assign5370_e4640, (p.p444 * var_temp_dn0), (p.p444 * var_temp_dn1), (p.p444 * var_temp_dn2), (p.p444 * var_temp_dn3), (p.p444 * var_temp_dn4), (p.p444 * var_temp_dn5), (p.p444 * var_temp_dn6), (p.p444 * var_temp_dn7), (p.p444 * var_temp_dn8), (p.p444 * var_temp_dn9), (p.p444 * var_temp_db0), (p.p444 * var_temp_db1), (p.p444 * var_temp_db2), (p.p444 * var_temp_db3),)
    } else {
        (var_cth_p, var_cth_p_dn0, var_cth_p_dn1, var_cth_p_dn2, var_cth_p_dn3, var_cth_p_dn4, var_cth_p_dn5, var_cth_p_dn6, var_cth_p_dn7, var_cth_p_dn8, var_cth_p_dn9, var_cth_p_db0, var_cth_p_db1, var_cth_p_db2, var_cth_p_db3,)
    }
};
        var_cth_p = assign5370_e4642;
        var_cth_p_dn0 = assign5370_e4642_d_n0;
        var_cth_p_dn1 = assign5370_e4642_d_n1;
        var_cth_p_dn2 = assign5370_e4642_d_n2;
        var_cth_p_dn3 = assign5370_e4642_d_n3;
        var_cth_p_dn4 = assign5370_e4642_d_n4;
        var_cth_p_dn5 = assign5370_e4642_d_n5;
        var_cth_p_dn6 = assign5370_e4642_d_n6;
        var_cth_p_dn7 = assign5370_e4642_d_n7;
        var_cth_p_dn8 = assign5370_e4642_d_n8;
        var_cth_p_dn9 = assign5370_e4642_d_n9;
        var_cth_p_db0 = assign5370_e4642_d_b0;
        var_cth_p_db1 = assign5370_e4642_d_b1;
        var_cth_p_db2 = assign5370_e4642_d_b2;
        var_cth_p_db3 = assign5370_e4642_d_b3;

        let (assign5380_e4649, assign5380_e4649_d_n0, assign5380_e4649_d_n1, assign5380_e4649_d_n2, assign5380_e4649_d_n3, assign5380_e4649_d_n4, assign5380_e4649_d_n5, assign5380_e4649_d_n6, assign5380_e4649_d_n7, assign5380_e4649_d_n8, assign5380_e4649_d_n9, assign5380_e4649_d_b0, assign5380_e4649_d_b1, assign5380_e4649_d_b2, assign5380_e4649_d_b3,) = {
    if (var_guard83 == 0.0) {
        let assign5380_e4647: f64 = (var_cth_p).max(0.0);
        (assign5380_e4647, if var_cth_p >= 0.0 { var_cth_p_dn0 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn1 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn2 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn3 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn4 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn5 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn6 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn7 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn8 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_dn9 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_db0 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_db1 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_db2 } else { 0.0 }, if var_cth_p >= 0.0 { var_cth_p_db3 } else { 0.0 },)
    } else {
        (var_cth_i, var_cth_i_dn0, var_cth_i_dn1, var_cth_i_dn2, var_cth_i_dn3, var_cth_i_dn4, var_cth_i_dn5, var_cth_i_dn6, var_cth_i_dn7, var_cth_i_dn8, var_cth_i_dn9, var_cth_i_db0, var_cth_i_db1, var_cth_i_db2, var_cth_i_db3,)
    }
};
        var_cth_i = assign5380_e4649;
        var_cth_i_dn0 = assign5380_e4649_d_n0;
        var_cth_i_dn1 = assign5380_e4649_d_n1;
        var_cth_i_dn2 = assign5380_e4649_d_n2;
        var_cth_i_dn3 = assign5380_e4649_d_n3;
        var_cth_i_dn4 = assign5380_e4649_d_n4;
        var_cth_i_dn5 = assign5380_e4649_d_n5;
        var_cth_i_dn6 = assign5380_e4649_d_n6;
        var_cth_i_dn7 = assign5380_e4649_d_n7;
        var_cth_i_dn8 = assign5380_e4649_d_n8;
        var_cth_i_dn9 = assign5380_e4649_d_n9;
        var_cth_i_db0 = assign5380_e4649_d_b0;
        var_cth_i_db1 = assign5380_e4649_d_b1;
        var_cth_i_db2 = assign5380_e4649_d_b2;
        var_cth_i_db3 = assign5380_e4649_d_b3;

        let (assign5390_e4654,) = {
    if (var_guard83 == 0.0) {
        (p.p447,)
    } else {
        (var_fnt_i,)
    }
};
        var_fnt_i = assign5390_e4654;

        let (assign5400_e4673,) = {
    if (var_guard83 == 0.0) {
        let assign5400_e4659: f64 = (p.p448 * var_betn_p);
        let assign5400_e4661: f64 = (assign5400_e4659 * var_betn_p);
        let assign5400_e4663: f64 = (assign5400_e4661 * var_iwe);
        let assign5400_e4665: f64 = (assign5400_e4663 * var_iwe);
        let assign5400_e4669: f64 = (p.p449 - 2.0);
        let assign5400_e4670: f64 = (var_ile).powf(assign5400_e4669);
        let assign5400_e4671: f64 = (assign5400_e4665 * assign5400_e4670);
        (assign5400_e4671,)
    } else {
        (var_fntexc_i,)
    }
};
        var_fntexc_i = assign5400_e4673;

        let (assign5410_e4684,) = {
    if (var_guard83 == 0.0) {
        let assign5410_e4678: f64 = (p.p450 * var_iae);
        let assign5410_e4681: f64 = (p.p451 * var_iwe);
        let assign5410_e4682: f64 = (assign5410_e4678 + assign5410_e4681);
        (assign5410_e4682,)
    } else {
        (var_nfa_p,)
    }
};
        var_nfa_p = assign5410_e4684;

        let (assign5420_e4691,) = {
    if (var_guard83 == 0.0) {
        let assign5420_e4689: f64 = (var_nfa_p).max(0.0);
        (assign5420_e4689,)
    } else {
        (var_nfa_i,)
    }
};
        var_nfa_i = assign5420_e4691;

        let (assign5430_e4698,) = {
    if (var_guard83 == 0.0) {
        let assign5430_e4696: f64 = (p.p452 * var_iae);
        (assign5430_e4696,)
    } else {
        (var_nfb_i,)
    }
};
        var_nfb_i = assign5430_e4698;

        let (assign5440_e4705,) = {
    if (var_guard83 == 0.0) {
        let assign5440_e4703: f64 = (p.p453 * var_iae);
        (assign5440_e4703,)
    } else {
        (var_nfc_i,)
    }
};
        var_nfc_i = assign5440_e4705;

        let (assign5450_e4710,) = {
    if (var_guard83 == 0.0) {
        (p.p454,)
    } else {
        (var_nfe_i,)
    }
};
        var_nfe_i = assign5450_e4710;

        let (assign5460_e4715,) = {
    if (var_guard83 == 0.0) {
        (p.p455,)
    } else {
        (var_nfeb_i,)
    }
};
        var_nfeb_i = assign5460_e4715;

        let assign5570_e4828: f64 = if ((((p.p457 > 0.0) && (p.p26 > 0.0)) && (p.p27 > 0.0)) && ((p.p29 == 1.0) || ((p.p29 > 1.0) && (p.p28 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard133 = assign5570_e4828;

        let assign5580_e4831: f64 = if p.p457 == 1.0 { 1.0 } else { 0.0 };
        var_guard134 = assign5580_e4831;

        let (assign5590_e4840, assign5590_e4840_d_n0, assign5590_e4840_d_n1, assign5590_e4840_d_n2, assign5590_e4840_d_n3, assign5590_e4840_d_n4, assign5590_e4840_d_n5, assign5590_e4840_d_n6, assign5590_e4840_d_n7, assign5590_e4840_d_n8, assign5590_e4840_d_n9, assign5590_e4840_d_b0, assign5590_e4840_d_b1, assign5590_e4840_d_b2, assign5590_e4840_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn0, var_tmpa_dn1, var_tmpa_dn2, var_tmpa_dn3, var_tmpa_dn4, var_tmpa_dn5, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9, var_tmpa_db0, var_tmpa_db1, var_tmpa_db2, var_tmpa_db3,)
    }
};
        var_tmpa = assign5590_e4840;
        var_tmpa_dn0 = assign5590_e4840_d_n0;
        var_tmpa_dn1 = assign5590_e4840_d_n1;
        var_tmpa_dn2 = assign5590_e4840_d_n2;
        var_tmpa_dn3 = assign5590_e4840_d_n3;
        var_tmpa_dn4 = assign5590_e4840_d_n4;
        var_tmpa_dn5 = assign5590_e4840_d_n5;
        var_tmpa_dn6 = assign5590_e4840_d_n6;
        var_tmpa_dn7 = assign5590_e4840_d_n7;
        var_tmpa_dn8 = assign5590_e4840_d_n8;
        var_tmpa_dn9 = assign5590_e4840_d_n9;
        var_tmpa_db0 = assign5590_e4840_d_b0;
        var_tmpa_db1 = assign5590_e4840_d_b1;
        var_tmpa_db2 = assign5590_e4840_d_b2;
        var_tmpa_db3 = assign5590_e4840_d_b3;

        let (assign5600_e4849,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign5600_e4849;

        let (assign5610_e4858,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign5610_e4858;

        *var_cth_i_slot = var_cth_i;
        *var_cth_i_db0_slot = var_cth_i_db0;
        *var_cth_i_db1_slot = var_cth_i_db1;
        *var_cth_i_db2_slot = var_cth_i_db2;
        *var_cth_i_db3_slot = var_cth_i_db3;
        *var_cth_i_dn0_slot = var_cth_i_dn0;
        *var_cth_i_dn1_slot = var_cth_i_dn1;
        *var_cth_i_dn2_slot = var_cth_i_dn2;
        *var_cth_i_dn3_slot = var_cth_i_dn3;
        *var_cth_i_dn4_slot = var_cth_i_dn4;
        *var_cth_i_dn5_slot = var_cth_i_dn5;
        *var_cth_i_dn6_slot = var_cth_i_dn6;
        *var_cth_i_dn7_slot = var_cth_i_dn7;
        *var_cth_i_dn8_slot = var_cth_i_dn8;
        *var_cth_i_dn9_slot = var_cth_i_dn9;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_db0_slot = var_cth_p_db0;
        *var_cth_p_db1_slot = var_cth_p_db1;
        *var_cth_p_db2_slot = var_cth_p_db2;
        *var_cth_p_db3_slot = var_cth_p_db3;
        *var_cth_p_dn0_slot = var_cth_p_dn0;
        *var_cth_p_dn1_slot = var_cth_p_dn1;
        *var_cth_p_dn2_slot = var_cth_p_dn2;
        *var_cth_p_dn3_slot = var_cth_p_dn3;
        *var_cth_p_dn4_slot = var_cth_p_dn4;
        *var_cth_p_dn5_slot = var_cth_p_dn5;
        *var_cth_p_dn6_slot = var_cth_p_dn6;
        *var_cth_p_dn7_slot = var_cth_p_dn7;
        *var_cth_p_dn8_slot = var_cth_p_dn8;
        *var_cth_p_dn9_slot = var_cth_p_dn9;
        *var_fnt_i_slot = var_fnt_i;
        *var_fntexc_i_slot = var_fntexc_i;
        *var_guard129_slot = var_guard129;
        *var_guard130_slot = var_guard130;
        *var_guard131_slot = var_guard131;
        *var_guard133_slot = var_guard133;
        *var_guard134_slot = var_guard134;
        *var_iloop_slot = var_iloop;
        *var_nfa_i_slot = var_nfa_i;
        *var_nfa_p_slot = var_nfa_p;
        *var_nfb_i_slot = var_nfb_i;
        *var_nfc_i_slot = var_nfc_i;
        *var_nfe_i_slot = var_nfe_i;
        *var_nfeb_i_slot = var_nfeb_i;
        *var_rth_p_slot = var_rth_p;
        *var_rth_p_db0_slot = var_rth_p_db0;
        *var_rth_p_db1_slot = var_rth_p_db1;
        *var_rth_p_db2_slot = var_rth_p_db2;
        *var_rth_p_db3_slot = var_rth_p_db3;
        *var_rth_p_dn0_slot = var_rth_p_dn0;
        *var_rth_p_dn1_slot = var_rth_p_dn1;
        *var_rth_p_dn2_slot = var_rth_p_dn2;
        *var_rth_p_dn3_slot = var_rth_p_dn3;
        *var_rth_p_dn4_slot = var_rth_p_dn4;
        *var_rth_p_dn5_slot = var_rth_p_dn5;
        *var_rth_p_dn6_slot = var_rth_p_dn6;
        *var_rth_p_dn7_slot = var_rth_p_dn7;
        *var_rth_p_dn8_slot = var_rth_p_dn8;
        *var_rth_p_dn9_slot = var_rth_p_dn9;
        *var_rth_t_slot = var_rth_t;
        *var_rth_t_db0_slot = var_rth_t_db0;
        *var_rth_t_db1_slot = var_rth_t_db1;
        *var_rth_t_db2_slot = var_rth_t_db2;
        *var_rth_t_db3_slot = var_rth_t_db3;
        *var_rth_t_dn0_slot = var_rth_t_dn0;
        *var_rth_t_dn1_slot = var_rth_t_dn1;
        *var_rth_t_dn2_slot = var_rth_t_dn2;
        *var_rth_t_dn3_slot = var_rth_t_dn3;
        *var_rth_t_dn4_slot = var_rth_t_dn4;
        *var_rth_t_dn5_slot = var_rth_t_dn5;
        *var_rth_t_dn6_slot = var_rth_t_dn6;
        *var_rth_t_dn7_slot = var_rth_t_dn7;
        *var_rth_t_dn8_slot = var_rth_t_dn8;
        *var_rth_t_dn9_slot = var_rth_t_dn9;
        *var_strth_i_slot = var_strth_i;
        *var_temp_slot = var_temp;
        *var_temp1_slot = var_temp1;
        *var_temp1_db0_slot = var_temp1_db0;
        *var_temp1_db1_slot = var_temp1_db1;
        *var_temp1_db2_slot = var_temp1_db2;
        *var_temp1_db3_slot = var_temp1_db3;
        *var_temp1_dn0_slot = var_temp1_dn0;
        *var_temp1_dn1_slot = var_temp1_dn1;
        *var_temp1_dn2_slot = var_temp1_dn2;
        *var_temp1_dn3_slot = var_temp1_dn3;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn5_slot = var_temp1_dn5;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp2_slot = var_temp2;
        *var_temp2_db0_slot = var_temp2_db0;
        *var_temp2_db1_slot = var_temp2_db1;
        *var_temp2_db2_slot = var_temp2_db2;
        *var_temp2_db3_slot = var_temp2_db3;
        *var_temp2_dn0_slot = var_temp2_dn0;
        *var_temp2_dn1_slot = var_temp2_dn1;
        *var_temp2_dn2_slot = var_temp2_dn2;
        *var_temp2_dn3_slot = var_temp2_dn3;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn5_slot = var_temp2_dn5;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp3_slot = var_temp3;
        *var_temp3_db0_slot = var_temp3_db0;
        *var_temp3_db1_slot = var_temp3_db1;
        *var_temp3_db2_slot = var_temp3_db2;
        *var_temp3_db3_slot = var_temp3_db3;
        *var_temp3_dn0_slot = var_temp3_dn0;
        *var_temp3_dn1_slot = var_temp3_dn1;
        *var_temp3_dn2_slot = var_temp3_dn2;
        *var_temp3_dn3_slot = var_temp3_dn3;
        *var_temp3_dn4_slot = var_temp3_dn4;
        *var_temp3_dn5_slot = var_temp3_dn5;
        *var_temp3_dn6_slot = var_temp3_dn6;
        *var_temp3_dn7_slot = var_temp3_dn7;
        *var_temp3_dn8_slot = var_temp3_dn8;
        *var_temp3_dn9_slot = var_temp3_dn9;
        *var_temp4_slot = var_temp4;
        *var_temp4_db0_slot = var_temp4_db0;
        *var_temp4_db1_slot = var_temp4_db1;
        *var_temp4_db2_slot = var_temp4_db2;
        *var_temp4_db3_slot = var_temp4_db3;
        *var_temp4_dn0_slot = var_temp4_dn0;
        *var_temp4_dn1_slot = var_temp4_dn1;
        *var_temp4_dn2_slot = var_temp4_dn2;
        *var_temp4_dn3_slot = var_temp4_dn3;
        *var_temp4_dn4_slot = var_temp4_dn4;
        *var_temp4_dn5_slot = var_temp4_dn5;
        *var_temp4_dn6_slot = var_temp4_dn6;
        *var_temp4_dn7_slot = var_temp4_dn7;
        *var_temp4_dn8_slot = var_temp4_dn8;
        *var_temp4_dn9_slot = var_temp4_dn9;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_db0_slot = var_tmpa_db0;
        *var_tmpa_db1_slot = var_tmpa_db1;
        *var_tmpa_db2_slot = var_tmpa_db2;
        *var_tmpa_db3_slot = var_tmpa_db3;
        *var_tmpa_dn0_slot = var_tmpa_dn0;
        *var_tmpa_dn1_slot = var_tmpa_dn1;
        *var_tmpa_dn2_slot = var_tmpa_dn2;
        *var_tmpa_dn3_slot = var_tmpa_dn3;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn5_slot = var_tmpa_dn5;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpb_slot = var_tmpb;
    }

    pub(super) fn stamp_transient_block_14(
        p: &Parameters,
        var_dellps: f64,
        var_delwod: f64,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_rt: f64,
        var_rt_db0: f64,
        var_rt_db1: f64,
        var_rt_db2: f64,
        var_rt_db3: f64,
        var_rt_dn0: f64,
        var_rt_dn1: f64,
        var_rt_dn2: f64,
        var_rt_dn3: f64,
        var_rt_dn4: f64,
        var_rt_dn5: f64,
        var_rt_dn6: f64,
        var_rt_dn7: f64,
        var_rt_dn8: f64,
        var_rt_dn9: f64,
        var_w_i: f64,
        var_betn1_t_slot: &mut f64,
        var_betn1_t_db0_slot: &mut f64,
        var_betn1_t_db1_slot: &mut f64,
        var_betn1_t_db2_slot: &mut f64,
        var_betn1_t_db3_slot: &mut f64,
        var_betn1_t_dn0_slot: &mut f64,
        var_betn1_t_dn1_slot: &mut f64,
        var_betn1_t_dn2_slot: &mut f64,
        var_betn1_t_dn3_slot: &mut f64,
        var_betn1_t_dn4_slot: &mut f64,
        var_betn1_t_dn5_slot: &mut f64,
        var_betn1_t_dn6_slot: &mut f64,
        var_betn1_t_dn7_slot: &mut f64,
        var_betn1_t_dn8_slot: &mut f64,
        var_betn1_t_dn9_slot: &mut f64,
        var_betn2_t_slot: &mut f64,
        var_betn2_t_db0_slot: &mut f64,
        var_betn2_t_db1_slot: &mut f64,
        var_betn2_t_db2_slot: &mut f64,
        var_betn2_t_db3_slot: &mut f64,
        var_betn2_t_dn0_slot: &mut f64,
        var_betn2_t_dn1_slot: &mut f64,
        var_betn2_t_dn2_slot: &mut f64,
        var_betn2_t_dn3_slot: &mut f64,
        var_betn2_t_dn4_slot: &mut f64,
        var_betn2_t_dn5_slot: &mut f64,
        var_betn2_t_dn6_slot: &mut f64,
        var_betn2_t_dn7_slot: &mut f64,
        var_betn2_t_dn8_slot: &mut f64,
        var_betn2_t_dn9_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_db0_slot: &mut f64,
        var_betn_p_db1_slot: &mut f64,
        var_betn_p_db2_slot: &mut f64,
        var_betn_p_db3_slot: &mut f64,
        var_betn_p_dn0_slot: &mut f64,
        var_betn_p_dn1_slot: &mut f64,
        var_betn_p_dn2_slot: &mut f64,
        var_betn_p_dn3_slot: &mut f64,
        var_betn_p_dn4_slot: &mut f64,
        var_betn_p_dn5_slot: &mut f64,
        var_betn_p_dn6_slot: &mut f64,
        var_betn_p_dn7_slot: &mut f64,
        var_betn_p_dn8_slot: &mut f64,
        var_betn_p_dn9_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_db0_slot: &mut f64,
        var_invsa_db1_slot: &mut f64,
        var_invsa_db2_slot: &mut f64,
        var_invsa_db3_slot: &mut f64,
        var_invsa_dn0_slot: &mut f64,
        var_invsa_dn1_slot: &mut f64,
        var_invsa_dn2_slot: &mut f64,
        var_invsa_dn3_slot: &mut f64,
        var_invsa_dn4_slot: &mut f64,
        var_invsa_dn5_slot: &mut f64,
        var_invsa_dn6_slot: &mut f64,
        var_invsa_dn7_slot: &mut f64,
        var_invsa_dn8_slot: &mut f64,
        var_invsa_dn9_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_db0_slot: &mut f64,
        var_kstressu0_db1_slot: &mut f64,
        var_kstressu0_db2_slot: &mut f64,
        var_kstressu0_db3_slot: &mut f64,
        var_kstressu0_dn0_slot: &mut f64,
        var_kstressu0_dn1_slot: &mut f64,
        var_kstressu0_dn2_slot: &mut f64,
        var_kstressu0_dn3_slot: &mut f64,
        var_kstressu0_dn4_slot: &mut f64,
        var_kstressu0_dn5_slot: &mut f64,
        var_kstressu0_dn6_slot: &mut f64,
        var_kstressu0_dn7_slot: &mut f64,
        var_kstressu0_dn8_slot: &mut f64,
        var_kstressu0_dn9_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_db0_slot: &mut f64,
        var_rhobeta_db1_slot: &mut f64,
        var_rhobeta_db2_slot: &mut f64,
        var_rhobeta_db3_slot: &mut f64,
        var_rhobeta_dn0_slot: &mut f64,
        var_rhobeta_dn1_slot: &mut f64,
        var_rhobeta_dn2_slot: &mut f64,
        var_rhobeta_dn3_slot: &mut f64,
        var_rhobeta_dn4_slot: &mut f64,
        var_rhobeta_dn5_slot: &mut f64,
        var_rhobeta_dn6_slot: &mut f64,
        var_rhobeta_dn7_slot: &mut f64,
        var_rhobeta_dn8_slot: &mut f64,
        var_rhobeta_dn9_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_db0_slot: &mut f64,
        var_rhobetaref_db1_slot: &mut f64,
        var_rhobetaref_db2_slot: &mut f64,
        var_rhobetaref_db3_slot: &mut f64,
        var_rhobetaref_dn0_slot: &mut f64,
        var_rhobetaref_dn1_slot: &mut f64,
        var_rhobetaref_dn2_slot: &mut f64,
        var_rhobetaref_dn3_slot: &mut f64,
        var_rhobetaref_dn4_slot: &mut f64,
        var_rhobetaref_dn5_slot: &mut f64,
        var_rhobetaref_dn6_slot: &mut f64,
        var_rhobetaref_dn7_slot: &mut f64,
        var_rhobetaref_dn8_slot: &mut f64,
        var_rhobetaref_dn9_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp0__blk79_slot: &mut f64,
        var_temp0__blk79_db0_slot: &mut f64,
        var_temp0__blk79_db1_slot: &mut f64,
        var_temp0__blk79_db2_slot: &mut f64,
        var_temp0__blk79_db3_slot: &mut f64,
        var_temp0__blk79_dn0_slot: &mut f64,
        var_temp0__blk79_dn1_slot: &mut f64,
        var_temp0__blk79_dn2_slot: &mut f64,
        var_temp0__blk79_dn3_slot: &mut f64,
        var_temp0__blk79_dn4_slot: &mut f64,
        var_temp0__blk79_dn5_slot: &mut f64,
        var_temp0__blk79_dn6_slot: &mut f64,
        var_temp0__blk79_dn7_slot: &mut f64,
        var_temp0__blk79_dn8_slot: &mut f64,
        var_temp0__blk79_dn9_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_db0_slot: &mut f64,
        var_thesat_p_db1_slot: &mut f64,
        var_thesat_p_db2_slot: &mut f64,
        var_thesat_p_db3_slot: &mut f64,
        var_thesat_p_dn0_slot: &mut f64,
        var_thesat_p_dn1_slot: &mut f64,
        var_thesat_p_dn2_slot: &mut f64,
        var_thesat_p_dn3_slot: &mut f64,
        var_thesat_p_dn4_slot: &mut f64,
        var_thesat_p_dn5_slot: &mut f64,
        var_thesat_p_dn6_slot: &mut f64,
        var_thesat_p_dn7_slot: &mut f64,
        var_thesat_p_dn8_slot: &mut f64,
        var_thesat_p_dn9_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_db0_slot: &mut f64,
        var_thesat_t_db1_slot: &mut f64,
        var_thesat_t_db2_slot: &mut f64,
        var_thesat_t_db3_slot: &mut f64,
        var_thesat_t_dn0_slot: &mut f64,
        var_thesat_t_dn1_slot: &mut f64,
        var_thesat_t_dn2_slot: &mut f64,
        var_thesat_t_dn3_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_dn5_slot: &mut f64,
        var_thesat_t_dn6_slot: &mut f64,
        var_thesat_t_dn7_slot: &mut f64,
        var_thesat_t_dn8_slot: &mut f64,
        var_thesat_t_dn9_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_db0_slot: &mut f64,
        var_thesatac_p_db1_slot: &mut f64,
        var_thesatac_p_db2_slot: &mut f64,
        var_thesatac_p_db3_slot: &mut f64,
        var_thesatac_p_dn0_slot: &mut f64,
        var_thesatac_p_dn1_slot: &mut f64,
        var_thesatac_p_dn2_slot: &mut f64,
        var_thesatac_p_dn3_slot: &mut f64,
        var_thesatac_p_dn4_slot: &mut f64,
        var_thesatac_p_dn5_slot: &mut f64,
        var_thesatac_p_dn6_slot: &mut f64,
        var_thesatac_p_dn7_slot: &mut f64,
        var_thesatac_p_dn8_slot: &mut f64,
        var_thesatac_p_dn9_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_db0_slot: &mut f64,
        var_tmpa_db1_slot: &mut f64,
        var_tmpa_db2_slot: &mut f64,
        var_tmpa_db3_slot: &mut f64,
        var_tmpa_dn0_slot: &mut f64,
        var_tmpa_dn1_slot: &mut f64,
        var_tmpa_dn2_slot: &mut f64,
        var_tmpa_dn3_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn5_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_wx_slot: &mut f64,
    ) {
        let mut var_betn1_t: f64 = *var_betn1_t_slot;
        let mut var_betn1_t_db0: f64 = *var_betn1_t_db0_slot;
        let mut var_betn1_t_db1: f64 = *var_betn1_t_db1_slot;
        let mut var_betn1_t_db2: f64 = *var_betn1_t_db2_slot;
        let mut var_betn1_t_db3: f64 = *var_betn1_t_db3_slot;
        let mut var_betn1_t_dn0: f64 = *var_betn1_t_dn0_slot;
        let mut var_betn1_t_dn1: f64 = *var_betn1_t_dn1_slot;
        let mut var_betn1_t_dn2: f64 = *var_betn1_t_dn2_slot;
        let mut var_betn1_t_dn3: f64 = *var_betn1_t_dn3_slot;
        let mut var_betn1_t_dn4: f64 = *var_betn1_t_dn4_slot;
        let mut var_betn1_t_dn5: f64 = *var_betn1_t_dn5_slot;
        let mut var_betn1_t_dn6: f64 = *var_betn1_t_dn6_slot;
        let mut var_betn1_t_dn7: f64 = *var_betn1_t_dn7_slot;
        let mut var_betn1_t_dn8: f64 = *var_betn1_t_dn8_slot;
        let mut var_betn1_t_dn9: f64 = *var_betn1_t_dn9_slot;
        let mut var_betn2_t: f64 = *var_betn2_t_slot;
        let mut var_betn2_t_db0: f64 = *var_betn2_t_db0_slot;
        let mut var_betn2_t_db1: f64 = *var_betn2_t_db1_slot;
        let mut var_betn2_t_db2: f64 = *var_betn2_t_db2_slot;
        let mut var_betn2_t_db3: f64 = *var_betn2_t_db3_slot;
        let mut var_betn2_t_dn0: f64 = *var_betn2_t_dn0_slot;
        let mut var_betn2_t_dn1: f64 = *var_betn2_t_dn1_slot;
        let mut var_betn2_t_dn2: f64 = *var_betn2_t_dn2_slot;
        let mut var_betn2_t_dn3: f64 = *var_betn2_t_dn3_slot;
        let mut var_betn2_t_dn4: f64 = *var_betn2_t_dn4_slot;
        let mut var_betn2_t_dn5: f64 = *var_betn2_t_dn5_slot;
        let mut var_betn2_t_dn6: f64 = *var_betn2_t_dn6_slot;
        let mut var_betn2_t_dn7: f64 = *var_betn2_t_dn7_slot;
        let mut var_betn2_t_dn8: f64 = *var_betn2_t_dn8_slot;
        let mut var_betn2_t_dn9: f64 = *var_betn2_t_dn9_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_db0: f64 = *var_betn_p_db0_slot;
        let mut var_betn_p_db1: f64 = *var_betn_p_db1_slot;
        let mut var_betn_p_db2: f64 = *var_betn_p_db2_slot;
        let mut var_betn_p_db3: f64 = *var_betn_p_db3_slot;
        let mut var_betn_p_dn0: f64 = *var_betn_p_dn0_slot;
        let mut var_betn_p_dn1: f64 = *var_betn_p_dn1_slot;
        let mut var_betn_p_dn2: f64 = *var_betn_p_dn2_slot;
        let mut var_betn_p_dn3: f64 = *var_betn_p_dn3_slot;
        let mut var_betn_p_dn4: f64 = *var_betn_p_dn4_slot;
        let mut var_betn_p_dn5: f64 = *var_betn_p_dn5_slot;
        let mut var_betn_p_dn6: f64 = *var_betn_p_dn6_slot;
        let mut var_betn_p_dn7: f64 = *var_betn_p_dn7_slot;
        let mut var_betn_p_dn8: f64 = *var_betn_p_dn8_slot;
        let mut var_betn_p_dn9: f64 = *var_betn_p_dn9_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_db0: f64 = *var_invsa_db0_slot;
        let mut var_invsa_db1: f64 = *var_invsa_db1_slot;
        let mut var_invsa_db2: f64 = *var_invsa_db2_slot;
        let mut var_invsa_db3: f64 = *var_invsa_db3_slot;
        let mut var_invsa_dn0: f64 = *var_invsa_dn0_slot;
        let mut var_invsa_dn1: f64 = *var_invsa_dn1_slot;
        let mut var_invsa_dn2: f64 = *var_invsa_dn2_slot;
        let mut var_invsa_dn3: f64 = *var_invsa_dn3_slot;
        let mut var_invsa_dn4: f64 = *var_invsa_dn4_slot;
        let mut var_invsa_dn5: f64 = *var_invsa_dn5_slot;
        let mut var_invsa_dn6: f64 = *var_invsa_dn6_slot;
        let mut var_invsa_dn7: f64 = *var_invsa_dn7_slot;
        let mut var_invsa_dn8: f64 = *var_invsa_dn8_slot;
        let mut var_invsa_dn9: f64 = *var_invsa_dn9_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_db0: f64 = *var_kstressu0_db0_slot;
        let mut var_kstressu0_db1: f64 = *var_kstressu0_db1_slot;
        let mut var_kstressu0_db2: f64 = *var_kstressu0_db2_slot;
        let mut var_kstressu0_db3: f64 = *var_kstressu0_db3_slot;
        let mut var_kstressu0_dn0: f64 = *var_kstressu0_dn0_slot;
        let mut var_kstressu0_dn1: f64 = *var_kstressu0_dn1_slot;
        let mut var_kstressu0_dn2: f64 = *var_kstressu0_dn2_slot;
        let mut var_kstressu0_dn3: f64 = *var_kstressu0_dn3_slot;
        let mut var_kstressu0_dn4: f64 = *var_kstressu0_dn4_slot;
        let mut var_kstressu0_dn5: f64 = *var_kstressu0_dn5_slot;
        let mut var_kstressu0_dn6: f64 = *var_kstressu0_dn6_slot;
        let mut var_kstressu0_dn7: f64 = *var_kstressu0_dn7_slot;
        let mut var_kstressu0_dn8: f64 = *var_kstressu0_dn8_slot;
        let mut var_kstressu0_dn9: f64 = *var_kstressu0_dn9_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_db0: f64 = *var_rhobeta_db0_slot;
        let mut var_rhobeta_db1: f64 = *var_rhobeta_db1_slot;
        let mut var_rhobeta_db2: f64 = *var_rhobeta_db2_slot;
        let mut var_rhobeta_db3: f64 = *var_rhobeta_db3_slot;
        let mut var_rhobeta_dn0: f64 = *var_rhobeta_dn0_slot;
        let mut var_rhobeta_dn1: f64 = *var_rhobeta_dn1_slot;
        let mut var_rhobeta_dn2: f64 = *var_rhobeta_dn2_slot;
        let mut var_rhobeta_dn3: f64 = *var_rhobeta_dn3_slot;
        let mut var_rhobeta_dn4: f64 = *var_rhobeta_dn4_slot;
        let mut var_rhobeta_dn5: f64 = *var_rhobeta_dn5_slot;
        let mut var_rhobeta_dn6: f64 = *var_rhobeta_dn6_slot;
        let mut var_rhobeta_dn7: f64 = *var_rhobeta_dn7_slot;
        let mut var_rhobeta_dn8: f64 = *var_rhobeta_dn8_slot;
        let mut var_rhobeta_dn9: f64 = *var_rhobeta_dn9_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_db0: f64 = *var_rhobetaref_db0_slot;
        let mut var_rhobetaref_db1: f64 = *var_rhobetaref_db1_slot;
        let mut var_rhobetaref_db2: f64 = *var_rhobetaref_db2_slot;
        let mut var_rhobetaref_db3: f64 = *var_rhobetaref_db3_slot;
        let mut var_rhobetaref_dn0: f64 = *var_rhobetaref_dn0_slot;
        let mut var_rhobetaref_dn1: f64 = *var_rhobetaref_dn1_slot;
        let mut var_rhobetaref_dn2: f64 = *var_rhobetaref_dn2_slot;
        let mut var_rhobetaref_dn3: f64 = *var_rhobetaref_dn3_slot;
        let mut var_rhobetaref_dn4: f64 = *var_rhobetaref_dn4_slot;
        let mut var_rhobetaref_dn5: f64 = *var_rhobetaref_dn5_slot;
        let mut var_rhobetaref_dn6: f64 = *var_rhobetaref_dn6_slot;
        let mut var_rhobetaref_dn7: f64 = *var_rhobetaref_dn7_slot;
        let mut var_rhobetaref_dn8: f64 = *var_rhobetaref_dn8_slot;
        let mut var_rhobetaref_dn9: f64 = *var_rhobetaref_dn9_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp0__blk79: f64 = *var_temp0__blk79_slot;
        let mut var_temp0__blk79_db0: f64 = *var_temp0__blk79_db0_slot;
        let mut var_temp0__blk79_db1: f64 = *var_temp0__blk79_db1_slot;
        let mut var_temp0__blk79_db2: f64 = *var_temp0__blk79_db2_slot;
        let mut var_temp0__blk79_db3: f64 = *var_temp0__blk79_db3_slot;
        let mut var_temp0__blk79_dn0: f64 = *var_temp0__blk79_dn0_slot;
        let mut var_temp0__blk79_dn1: f64 = *var_temp0__blk79_dn1_slot;
        let mut var_temp0__blk79_dn2: f64 = *var_temp0__blk79_dn2_slot;
        let mut var_temp0__blk79_dn3: f64 = *var_temp0__blk79_dn3_slot;
        let mut var_temp0__blk79_dn4: f64 = *var_temp0__blk79_dn4_slot;
        let mut var_temp0__blk79_dn5: f64 = *var_temp0__blk79_dn5_slot;
        let mut var_temp0__blk79_dn6: f64 = *var_temp0__blk79_dn6_slot;
        let mut var_temp0__blk79_dn7: f64 = *var_temp0__blk79_dn7_slot;
        let mut var_temp0__blk79_dn8: f64 = *var_temp0__blk79_dn8_slot;
        let mut var_temp0__blk79_dn9: f64 = *var_temp0__blk79_dn9_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_db0: f64 = *var_thesat_p_db0_slot;
        let mut var_thesat_p_db1: f64 = *var_thesat_p_db1_slot;
        let mut var_thesat_p_db2: f64 = *var_thesat_p_db2_slot;
        let mut var_thesat_p_db3: f64 = *var_thesat_p_db3_slot;
        let mut var_thesat_p_dn0: f64 = *var_thesat_p_dn0_slot;
        let mut var_thesat_p_dn1: f64 = *var_thesat_p_dn1_slot;
        let mut var_thesat_p_dn2: f64 = *var_thesat_p_dn2_slot;
        let mut var_thesat_p_dn3: f64 = *var_thesat_p_dn3_slot;
        let mut var_thesat_p_dn4: f64 = *var_thesat_p_dn4_slot;
        let mut var_thesat_p_dn5: f64 = *var_thesat_p_dn5_slot;
        let mut var_thesat_p_dn6: f64 = *var_thesat_p_dn6_slot;
        let mut var_thesat_p_dn7: f64 = *var_thesat_p_dn7_slot;
        let mut var_thesat_p_dn8: f64 = *var_thesat_p_dn8_slot;
        let mut var_thesat_p_dn9: f64 = *var_thesat_p_dn9_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_db0: f64 = *var_thesat_t_db0_slot;
        let mut var_thesat_t_db1: f64 = *var_thesat_t_db1_slot;
        let mut var_thesat_t_db2: f64 = *var_thesat_t_db2_slot;
        let mut var_thesat_t_db3: f64 = *var_thesat_t_db3_slot;
        let mut var_thesat_t_dn0: f64 = *var_thesat_t_dn0_slot;
        let mut var_thesat_t_dn1: f64 = *var_thesat_t_dn1_slot;
        let mut var_thesat_t_dn2: f64 = *var_thesat_t_dn2_slot;
        let mut var_thesat_t_dn3: f64 = *var_thesat_t_dn3_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_dn5: f64 = *var_thesat_t_dn5_slot;
        let mut var_thesat_t_dn6: f64 = *var_thesat_t_dn6_slot;
        let mut var_thesat_t_dn7: f64 = *var_thesat_t_dn7_slot;
        let mut var_thesat_t_dn8: f64 = *var_thesat_t_dn8_slot;
        let mut var_thesat_t_dn9: f64 = *var_thesat_t_dn9_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_db0: f64 = *var_thesatac_p_db0_slot;
        let mut var_thesatac_p_db1: f64 = *var_thesatac_p_db1_slot;
        let mut var_thesatac_p_db2: f64 = *var_thesatac_p_db2_slot;
        let mut var_thesatac_p_db3: f64 = *var_thesatac_p_db3_slot;
        let mut var_thesatac_p_dn0: f64 = *var_thesatac_p_dn0_slot;
        let mut var_thesatac_p_dn1: f64 = *var_thesatac_p_dn1_slot;
        let mut var_thesatac_p_dn2: f64 = *var_thesatac_p_dn2_slot;
        let mut var_thesatac_p_dn3: f64 = *var_thesatac_p_dn3_slot;
        let mut var_thesatac_p_dn4: f64 = *var_thesatac_p_dn4_slot;
        let mut var_thesatac_p_dn5: f64 = *var_thesatac_p_dn5_slot;
        let mut var_thesatac_p_dn6: f64 = *var_thesatac_p_dn6_slot;
        let mut var_thesatac_p_dn7: f64 = *var_thesatac_p_dn7_slot;
        let mut var_thesatac_p_dn8: f64 = *var_thesatac_p_dn8_slot;
        let mut var_thesatac_p_dn9: f64 = *var_thesatac_p_dn9_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_db0: f64 = *var_tmpa_db0_slot;
        let mut var_tmpa_db1: f64 = *var_tmpa_db1_slot;
        let mut var_tmpa_db2: f64 = *var_tmpa_db2_slot;
        let mut var_tmpa_db3: f64 = *var_tmpa_db3_slot;
        let mut var_tmpa_dn0: f64 = *var_tmpa_dn0_slot;
        let mut var_tmpa_dn1: f64 = *var_tmpa_dn1_slot;
        let mut var_tmpa_dn2: f64 = *var_tmpa_dn2_slot;
        let mut var_tmpa_dn3: f64 = *var_tmpa_dn3_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn5: f64 = *var_tmpa_dn5_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_wx: f64 = *var_wx_slot;

        let mut assign5620_loop_guard: usize = 0;
        while {
            let assign5620_cond_e4868: f64 = (p.p29 - 0.5);
            let assign5620_cond_e4870: f64 = if ((((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) && (var_iloop < assign5620_cond_e4868)) { 1.0 } else { 0.0 };
            assign5620_cond_e4870 != 0.0
        } {
            assign5620_loop_guard += 1;
            assert!(assign5620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign5620_body0_e4893, assign5620_body0_e4893_d_n0, assign5620_body0_e4893_d_n1, assign5620_body0_e4893_d_n2, assign5620_body0_e4893_d_n3, assign5620_body0_e4893_d_n4, assign5620_body0_e4893_d_n5, assign5620_body0_e4893_d_n6, assign5620_body0_e4893_d_n7, assign5620_body0_e4893_d_n8, assign5620_body0_e4893_d_n9, assign5620_body0_e4893_d_b0, assign5620_body0_e4893_d_b1, assign5620_body0_e4893_d_b2, assign5620_body0_e4893_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5620_body0_e4882: f64 = (0.5 * p.p20);
        let assign5620_body0_e4883: f64 = (p.p26 + assign5620_body0_e4882);
        let assign5620_body0_e4887: f64 = (p.p28 + p.p20);
        let assign5620_body0_e4888: f64 = (var_iloop * assign5620_body0_e4887);
        let assign5620_body0_e4889: f64 = (assign5620_body0_e4883 + assign5620_body0_e4888);
        let assign5620_body0_e4890: f64 = (1.0 / assign5620_body0_e4889);
        let assign5620_body0_e4891: f64 = (var_tmpa + assign5620_body0_e4890);
        (assign5620_body0_e4891, var_tmpa_dn0, var_tmpa_dn1, var_tmpa_dn2, var_tmpa_dn3, var_tmpa_dn4, var_tmpa_dn5, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9, var_tmpa_db0, var_tmpa_db1, var_tmpa_db2, var_tmpa_db3,)
    } else {
        (var_tmpa, var_tmpa_dn0, var_tmpa_dn1, var_tmpa_dn2, var_tmpa_dn3, var_tmpa_dn4, var_tmpa_dn5, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9, var_tmpa_db0, var_tmpa_db1, var_tmpa_db2, var_tmpa_db3,)
    }
};
            var_tmpa = assign5620_body0_e4893;
            var_tmpa_dn0 = assign5620_body0_e4893_d_n0;
            var_tmpa_dn1 = assign5620_body0_e4893_d_n1;
            var_tmpa_dn2 = assign5620_body0_e4893_d_n2;
            var_tmpa_dn3 = assign5620_body0_e4893_d_n3;
            var_tmpa_dn4 = assign5620_body0_e4893_d_n4;
            var_tmpa_dn5 = assign5620_body0_e4893_d_n5;
            var_tmpa_dn6 = assign5620_body0_e4893_d_n6;
            var_tmpa_dn7 = assign5620_body0_e4893_d_n7;
            var_tmpa_dn8 = assign5620_body0_e4893_d_n8;
            var_tmpa_dn9 = assign5620_body0_e4893_d_n9;
            var_tmpa_db0 = assign5620_body0_e4893_d_b0;
            var_tmpa_db1 = assign5620_body0_e4893_d_b1;
            var_tmpa_db2 = assign5620_body0_e4893_d_b2;
            var_tmpa_db3 = assign5620_body0_e4893_d_b3;
            let (assign5620_body1_e4916,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5620_body1_e4905: f64 = (0.5 * p.p20);
        let assign5620_body1_e4906: f64 = (p.p27 + assign5620_body1_e4905);
        let assign5620_body1_e4910: f64 = (p.p28 + p.p20);
        let assign5620_body1_e4911: f64 = (var_iloop * assign5620_body1_e4910);
        let assign5620_body1_e4912: f64 = (assign5620_body1_e4906 + assign5620_body1_e4911);
        let assign5620_body1_e4913: f64 = (1.0 / assign5620_body1_e4912);
        let assign5620_body1_e4914: f64 = (var_tmpb + assign5620_body1_e4913);
        (assign5620_body1_e4914,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign5620_body1_e4916;
            let (assign5620_body2_e4927,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5620_body2_e4925: f64 = (var_iloop + 1.0);
        (assign5620_body2_e4925,)
    } else {
        (var_iloop,)
    }
};
            var_iloop = assign5620_body2_e4927;
        }

        let (assign5630_e4938, assign5630_e4938_d_n0, assign5630_e4938_d_n1, assign5630_e4938_d_n2, assign5630_e4938_d_n3, assign5630_e4938_d_n4, assign5630_e4938_d_n5, assign5630_e4938_d_n6, assign5630_e4938_d_n7, assign5630_e4938_d_n8, assign5630_e4938_d_n9, assign5630_e4938_d_b0, assign5630_e4938_d_b1, assign5630_e4938_d_b2, assign5630_e4938_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5630_e4936: f64 = (var_tmpa / p.p29);
        (assign5630_e4936, (var_tmpa_dn0 / p.p29), (var_tmpa_dn1 / p.p29), (var_tmpa_dn2 / p.p29), (var_tmpa_dn3 / p.p29), (var_tmpa_dn4 / p.p29), (var_tmpa_dn5 / p.p29), (var_tmpa_dn6 / p.p29), (var_tmpa_dn7 / p.p29), (var_tmpa_dn8 / p.p29), (var_tmpa_dn9 / p.p29), (var_tmpa_db0 / p.p29), (var_tmpa_db1 / p.p29), (var_tmpa_db2 / p.p29), (var_tmpa_db3 / p.p29),)
    } else {
        (var_invsa, var_invsa_dn0, var_invsa_dn1, var_invsa_dn2, var_invsa_dn3, var_invsa_dn4, var_invsa_dn5, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9, var_invsa_db0, var_invsa_db1, var_invsa_db2, var_invsa_db3,)
    }
};
        var_invsa = assign5630_e4938;
        var_invsa_dn0 = assign5630_e4938_d_n0;
        var_invsa_dn1 = assign5630_e4938_d_n1;
        var_invsa_dn2 = assign5630_e4938_d_n2;
        var_invsa_dn3 = assign5630_e4938_d_n3;
        var_invsa_dn4 = assign5630_e4938_d_n4;
        var_invsa_dn5 = assign5630_e4938_d_n5;
        var_invsa_dn6 = assign5630_e4938_d_n6;
        var_invsa_dn7 = assign5630_e4938_d_n7;
        var_invsa_dn8 = assign5630_e4938_d_n8;
        var_invsa_dn9 = assign5630_e4938_d_n9;
        var_invsa_db0 = assign5630_e4938_d_b0;
        var_invsa_db1 = assign5630_e4938_d_b1;
        var_invsa_db2 = assign5630_e4938_d_b2;
        var_invsa_db3 = assign5630_e4938_d_b3;

        let (assign5640_e4949,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5640_e4947: f64 = (var_tmpb / p.p29);
        (assign5640_e4947,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign5640_e4949;

        let (assign5650_e4964,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5650_e4960: f64 = (0.5 * p.p20);
        let assign5650_e4961: f64 = (p.p458 + assign5650_e4960);
        let assign5650_e4962: f64 = (1.0 / assign5650_e4961);
        (assign5650_e4962,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign5650_e4964;

        let (assign5660_e4979,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5660_e4975: f64 = (0.5 * p.p20);
        let assign5660_e4976: f64 = (p.p459 + assign5660_e4975);
        let assign5660_e4977: f64 = (1.0 / assign5660_e4976);
        (assign5660_e4977,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign5660_e4979;

        let (assign5670_e4992,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5670_e4988: f64 = (p.p20 + var_dellps);
        let assign5670_e4990: f64 = (assign5670_e4988).max(1e-9);
        (assign5670_e4990,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign5670_e4992;

        let (assign5680_e5007,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5680_e5001: f64 = (var_w_i + var_delwod);
        let assign5680_e5003: f64 = (assign5680_e5001 + p.p460);
        let assign5680_e5005: f64 = (assign5680_e5003).max(1e-9);
        (assign5680_e5005,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign5680_e5007;

        let (assign5690_e5020,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5690_e5017: f64 = (var_lx).powf(p.p467);
        let assign5690_e5018: f64 = (1.0 / assign5690_e5017);
        (assign5690_e5018,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5690_e5020;

        let (assign5700_e5033,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5700_e5030: f64 = (var_wx).powf(p.p468);
        let assign5700_e5031: f64 = (1.0 / assign5700_e5030);
        (assign5700_e5031,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5700_e5033;

        let (assign5710_e5064, assign5710_e5064_d_n0, assign5710_e5064_d_n1, assign5710_e5064_d_n2, assign5710_e5064_d_n3, assign5710_e5064_d_n4, assign5710_e5064_d_n5, assign5710_e5064_d_n6, assign5710_e5064_d_n7, assign5710_e5064_d_n8, assign5710_e5064_d_n9, assign5710_e5064_d_b0, assign5710_e5064_d_b1, assign5710_e5064_d_b2, assign5710_e5064_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5710_e5043: f64 = (p.p464 * var_templ);
        let assign5710_e5044: f64 = (1.0 + assign5710_e5043);
        let assign5710_e5047: f64 = (p.p465 * var_tempw);
        let assign5710_e5048: f64 = (assign5710_e5044 + assign5710_e5047);
        let assign5710_e5051: f64 = (p.p466 * var_templ);
        let assign5710_e5053: f64 = (assign5710_e5051 * var_tempw);
        let assign5710_e5054: f64 = (assign5710_e5048 + assign5710_e5053);
        let assign5710_e5059: f64 = (var_rt - 1.0);
        let assign5710_e5060: f64 = (p.p463 * assign5710_e5059);
        let assign5710_e5061: f64 = (1.0 + assign5710_e5060);
        let assign5710_e5062: f64 = (assign5710_e5054 * assign5710_e5061);
        (assign5710_e5062, (assign5710_e5054 * (p.p463 * var_rt_dn0)), (assign5710_e5054 * (p.p463 * var_rt_dn1)), (assign5710_e5054 * (p.p463 * var_rt_dn2)), (assign5710_e5054 * (p.p463 * var_rt_dn3)), (assign5710_e5054 * (p.p463 * var_rt_dn4)), (assign5710_e5054 * (p.p463 * var_rt_dn5)), (assign5710_e5054 * (p.p463 * var_rt_dn6)), (assign5710_e5054 * (p.p463 * var_rt_dn7)), (assign5710_e5054 * (p.p463 * var_rt_dn8)), (assign5710_e5054 * (p.p463 * var_rt_dn9)), (assign5710_e5054 * (p.p463 * var_rt_db0)), (assign5710_e5054 * (p.p463 * var_rt_db1)), (assign5710_e5054 * (p.p463 * var_rt_db2)), (assign5710_e5054 * (p.p463 * var_rt_db3)),)
    } else {
        (var_kstressu0, var_kstressu0_dn0, var_kstressu0_dn1, var_kstressu0_dn2, var_kstressu0_dn3, var_kstressu0_dn4, var_kstressu0_dn5, var_kstressu0_dn6, var_kstressu0_dn7, var_kstressu0_dn8, var_kstressu0_dn9, var_kstressu0_db0, var_kstressu0_db1, var_kstressu0_db2, var_kstressu0_db3,)
    }
};
        var_kstressu0 = assign5710_e5064;
        var_kstressu0_dn0 = assign5710_e5064_d_n0;
        var_kstressu0_dn1 = assign5710_e5064_d_n1;
        var_kstressu0_dn2 = assign5710_e5064_d_n2;
        var_kstressu0_dn3 = assign5710_e5064_d_n3;
        var_kstressu0_dn4 = assign5710_e5064_d_n4;
        var_kstressu0_dn5 = assign5710_e5064_d_n5;
        var_kstressu0_dn6 = assign5710_e5064_d_n6;
        var_kstressu0_dn7 = assign5710_e5064_d_n7;
        var_kstressu0_dn8 = assign5710_e5064_d_n8;
        var_kstressu0_dn9 = assign5710_e5064_d_n9;
        var_kstressu0_db0 = assign5710_e5064_d_b0;
        var_kstressu0_db1 = assign5710_e5064_d_b1;
        var_kstressu0_db2 = assign5710_e5064_d_b2;
        var_kstressu0_db3 = assign5710_e5064_d_b3;

        let (assign5720_e5079, assign5720_e5079_d_n0, assign5720_e5079_d_n1, assign5720_e5079_d_n2, assign5720_e5079_d_n3, assign5720_e5079_d_n4, assign5720_e5079_d_n5, assign5720_e5079_d_n6, assign5720_e5079_d_n7, assign5720_e5079_d_n8, assign5720_e5079_d_n9, assign5720_e5079_d_b0, assign5720_e5079_d_b1, assign5720_e5079_d_b2, assign5720_e5079_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5720_e5074: f64 = (var_invsa + var_invsb);
        let assign5720_e5075: f64 = (p.p461 * assign5720_e5074);
        let assign5720_e5077: f64 = (assign5720_e5075 / var_kstressu0);
        (assign5720_e5077, ((((p.p461 * var_invsa_dn0) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn0)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn1) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn1)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn2) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn2)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn3) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn3)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn4) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn4)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn5) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn5)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn6) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn6)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn7) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn7)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn8) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn8)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_dn9) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_dn9)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_db0) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_db0)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_db1) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_db1)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_db2) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_db2)) / (var_kstressu0 * var_kstressu0)), ((((p.p461 * var_invsa_db3) * var_kstressu0) - (assign5720_e5075 * var_kstressu0_db3)) / (var_kstressu0 * var_kstressu0)),)
    } else {
        (var_rhobeta, var_rhobeta_dn0, var_rhobeta_dn1, var_rhobeta_dn2, var_rhobeta_dn3, var_rhobeta_dn4, var_rhobeta_dn5, var_rhobeta_dn6, var_rhobeta_dn7, var_rhobeta_dn8, var_rhobeta_dn9, var_rhobeta_db0, var_rhobeta_db1, var_rhobeta_db2, var_rhobeta_db3,)
    }
};
        var_rhobeta = assign5720_e5079;
        var_rhobeta_dn0 = assign5720_e5079_d_n0;
        var_rhobeta_dn1 = assign5720_e5079_d_n1;
        var_rhobeta_dn2 = assign5720_e5079_d_n2;
        var_rhobeta_dn3 = assign5720_e5079_d_n3;
        var_rhobeta_dn4 = assign5720_e5079_d_n4;
        var_rhobeta_dn5 = assign5720_e5079_d_n5;
        var_rhobeta_dn6 = assign5720_e5079_d_n6;
        var_rhobeta_dn7 = assign5720_e5079_d_n7;
        var_rhobeta_dn8 = assign5720_e5079_d_n8;
        var_rhobeta_dn9 = assign5720_e5079_d_n9;
        var_rhobeta_db0 = assign5720_e5079_d_b0;
        var_rhobeta_db1 = assign5720_e5079_d_b1;
        var_rhobeta_db2 = assign5720_e5079_d_b2;
        var_rhobeta_db3 = assign5720_e5079_d_b3;

        let (assign5730_e5094, assign5730_e5094_d_n0, assign5730_e5094_d_n1, assign5730_e5094_d_n2, assign5730_e5094_d_n3, assign5730_e5094_d_n4, assign5730_e5094_d_n5, assign5730_e5094_d_n6, assign5730_e5094_d_n7, assign5730_e5094_d_n8, assign5730_e5094_d_n9, assign5730_e5094_d_b0, assign5730_e5094_d_b1, assign5730_e5094_d_b2, assign5730_e5094_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5730_e5089: f64 = (var_invsaref + var_invsbref);
        let assign5730_e5090: f64 = (p.p461 * assign5730_e5089);
        let assign5730_e5092: f64 = (assign5730_e5090 / var_kstressu0);
        (assign5730_e5092, (-((assign5730_e5090 * var_kstressu0_dn0) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn1) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn2) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn3) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn4) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn5) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn6) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn7) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn8) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_dn9) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_db0) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_db1) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_db2) / (var_kstressu0 * var_kstressu0))), (-((assign5730_e5090 * var_kstressu0_db3) / (var_kstressu0 * var_kstressu0))),)
    } else {
        (var_rhobetaref, var_rhobetaref_dn0, var_rhobetaref_dn1, var_rhobetaref_dn2, var_rhobetaref_dn3, var_rhobetaref_dn4, var_rhobetaref_dn5, var_rhobetaref_dn6, var_rhobetaref_dn7, var_rhobetaref_dn8, var_rhobetaref_dn9, var_rhobetaref_db0, var_rhobetaref_db1, var_rhobetaref_db2, var_rhobetaref_db3,)
    }
};
        var_rhobetaref = assign5730_e5094;
        var_rhobetaref_dn0 = assign5730_e5094_d_n0;
        var_rhobetaref_dn1 = assign5730_e5094_d_n1;
        var_rhobetaref_dn2 = assign5730_e5094_d_n2;
        var_rhobetaref_dn3 = assign5730_e5094_d_n3;
        var_rhobetaref_dn4 = assign5730_e5094_d_n4;
        var_rhobetaref_dn5 = assign5730_e5094_d_n5;
        var_rhobetaref_dn6 = assign5730_e5094_d_n6;
        var_rhobetaref_dn7 = assign5730_e5094_d_n7;
        var_rhobetaref_dn8 = assign5730_e5094_d_n8;
        var_rhobetaref_dn9 = assign5730_e5094_d_n9;
        var_rhobetaref_db0 = assign5730_e5094_d_b0;
        var_rhobetaref_db1 = assign5730_e5094_d_b1;
        var_rhobetaref_db2 = assign5730_e5094_d_b2;
        var_rhobetaref_db3 = assign5730_e5094_d_b3;

        let (assign5740_e5107,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5740_e5104: f64 = (var_lx).powf(p.p473);
        let assign5740_e5105: f64 = (1.0 / assign5740_e5104);
        (assign5740_e5105,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign5740_e5107;

        let (assign5750_e5120,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5750_e5117: f64 = (var_wx).powf(p.p474);
        let assign5750_e5118: f64 = (1.0 / assign5750_e5117);
        (assign5750_e5118,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign5750_e5120;

        let (assign5760_e5145,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5760_e5130: f64 = (p.p470 * var_templ);
        let assign5760_e5131: f64 = (1.0 + assign5760_e5130);
        let assign5760_e5134: f64 = (p.p471 * var_tempw);
        let assign5760_e5135: f64 = (assign5760_e5131 + assign5760_e5134);
        let assign5760_e5138: f64 = (p.p472 * var_templ);
        let assign5760_e5140: f64 = (assign5760_e5138 * var_tempw);
        let assign5760_e5141: f64 = (assign5760_e5135 + assign5760_e5140);
        let assign5760_e5143: f64 = (assign5760_e5141).max(1e-20);
        (assign5760_e5143,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign5760_e5145;

        let (assign5770_e5160, assign5770_e5160_d_n0, assign5770_e5160_d_n1, assign5770_e5160_d_n2, assign5770_e5160_d_n3, assign5770_e5160_d_n4, assign5770_e5160_d_n5, assign5770_e5160_d_n6, assign5770_e5160_d_n7, assign5770_e5160_d_n8, assign5770_e5160_d_n9, assign5770_e5160_d_b0, assign5770_e5160_d_b1, assign5770_e5160_d_b2, assign5770_e5160_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5770_e5154: f64 = (var_invsa + var_invsb);
        let assign5770_e5156: f64 = (assign5770_e5154 - var_invsaref);
        let assign5770_e5158: f64 = (assign5770_e5156 - var_invsbref);
        (assign5770_e5158, var_invsa_dn0, var_invsa_dn1, var_invsa_dn2, var_invsa_dn3, var_invsa_dn4, var_invsa_dn5, var_invsa_dn6, var_invsa_dn7, var_invsa_dn8, var_invsa_dn9, var_invsa_db0, var_invsa_db1, var_invsa_db2, var_invsa_db3,)
    } else {
        (var_temp0__blk79, var_temp0__blk79_dn0, var_temp0__blk79_dn1, var_temp0__blk79_dn2, var_temp0__blk79_dn3, var_temp0__blk79_dn4, var_temp0__blk79_dn5, var_temp0__blk79_dn6, var_temp0__blk79_dn7, var_temp0__blk79_dn8, var_temp0__blk79_dn9, var_temp0__blk79_db0, var_temp0__blk79_db1, var_temp0__blk79_db2, var_temp0__blk79_db3,)
    }
};
        var_temp0__blk79 = assign5770_e5160;
        var_temp0__blk79_dn0 = assign5770_e5160_d_n0;
        var_temp0__blk79_dn1 = assign5770_e5160_d_n1;
        var_temp0__blk79_dn2 = assign5770_e5160_d_n2;
        var_temp0__blk79_dn3 = assign5770_e5160_d_n3;
        var_temp0__blk79_dn4 = assign5770_e5160_d_n4;
        var_temp0__blk79_dn5 = assign5770_e5160_d_n5;
        var_temp0__blk79_dn6 = assign5770_e5160_d_n6;
        var_temp0__blk79_dn7 = assign5770_e5160_d_n7;
        var_temp0__blk79_dn8 = assign5770_e5160_d_n8;
        var_temp0__blk79_dn9 = assign5770_e5160_d_n9;
        var_temp0__blk79_db0 = assign5770_e5160_d_b0;
        var_temp0__blk79_db1 = assign5770_e5160_d_b1;
        var_temp0__blk79_db2 = assign5770_e5160_d_b2;
        var_temp0__blk79_db3 = assign5770_e5160_d_b3;

        let (assign5780_e5177, assign5780_e5177_d_n0, assign5780_e5177_d_n1, assign5780_e5177_d_n2, assign5780_e5177_d_n3, assign5780_e5177_d_n4, assign5780_e5177_d_n5, assign5780_e5177_d_n6, assign5780_e5177_d_n7, assign5780_e5177_d_n8, assign5780_e5177_d_n9, assign5780_e5177_d_b0, assign5780_e5177_d_b1, assign5780_e5177_d_b2, assign5780_e5177_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5780_e5170: f64 = (1.0 + var_rhobeta);
        let assign5780_e5171: f64 = (var_betn_p * assign5780_e5170);
        let assign5780_e5174: f64 = (1.0 + var_rhobetaref);
        let assign5780_e5175: f64 = (assign5780_e5171 / assign5780_e5174);
        (assign5780_e5175, (((((var_betn_p_dn0 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn0)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn0)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn1 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn1)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn1)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn2 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn2)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn2)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn3 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn3)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn3)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn4 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn4)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn4)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn5 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn5)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn5)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn6 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn6)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn6)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn7 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn7)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn7)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn8 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn8)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn8)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_dn9 * assign5780_e5170) + (var_betn_p * var_rhobeta_dn9)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_dn9)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_db0 * assign5780_e5170) + (var_betn_p * var_rhobeta_db0)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_db0)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_db1 * assign5780_e5170) + (var_betn_p * var_rhobeta_db1)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_db1)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_db2 * assign5780_e5170) + (var_betn_p * var_rhobeta_db2)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_db2)) / (assign5780_e5174 * assign5780_e5174)), (((((var_betn_p_db3 * assign5780_e5170) + (var_betn_p * var_rhobeta_db3)) * assign5780_e5174) - (assign5780_e5171 * var_rhobetaref_db3)) / (assign5780_e5174 * assign5780_e5174)),)
    } else {
        (var_betn_p, var_betn_p_dn0, var_betn_p_dn1, var_betn_p_dn2, var_betn_p_dn3, var_betn_p_dn4, var_betn_p_dn5, var_betn_p_dn6, var_betn_p_dn7, var_betn_p_dn8, var_betn_p_dn9, var_betn_p_db0, var_betn_p_db1, var_betn_p_db2, var_betn_p_db3,)
    }
};
        var_betn_p = assign5780_e5177;
        var_betn_p_dn0 = assign5780_e5177_d_n0;
        var_betn_p_dn1 = assign5780_e5177_d_n1;
        var_betn_p_dn2 = assign5780_e5177_d_n2;
        var_betn_p_dn3 = assign5780_e5177_d_n3;
        var_betn_p_dn4 = assign5780_e5177_d_n4;
        var_betn_p_dn5 = assign5780_e5177_d_n5;
        var_betn_p_dn6 = assign5780_e5177_d_n6;
        var_betn_p_dn7 = assign5780_e5177_d_n7;
        var_betn_p_dn8 = assign5780_e5177_d_n8;
        var_betn_p_dn9 = assign5780_e5177_d_n9;
        var_betn_p_db0 = assign5780_e5177_d_b0;
        var_betn_p_db1 = assign5780_e5177_d_b1;
        var_betn_p_db2 = assign5780_e5177_d_b2;
        var_betn_p_db3 = assign5780_e5177_d_b3;

        let (assign5790_e5188, assign5790_e5188_d_n0, assign5790_e5188_d_n1, assign5790_e5188_d_n2, assign5790_e5188_d_n3, assign5790_e5188_d_n4, assign5790_e5188_d_n5, assign5790_e5188_d_n6, assign5790_e5188_d_n7, assign5790_e5188_d_n8, assign5790_e5188_d_n9, assign5790_e5188_d_b0, assign5790_e5188_d_b1, assign5790_e5188_d_b2, assign5790_e5188_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5790_e5186: f64 = (var_betn_p).max(1e-10);
        (assign5790_e5186, if var_betn_p >= 1e-10 { var_betn_p_dn0 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn1 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn2 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn3 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn4 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn5 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn6 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn7 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn8 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_dn9 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db0 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db1 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db2 } else { 0.0 }, if var_betn_p >= 1e-10 { var_betn_p_db3 } else { 0.0 },)
    } else {
        (var_betn1_t, var_betn1_t_dn0, var_betn1_t_dn1, var_betn1_t_dn2, var_betn1_t_dn3, var_betn1_t_dn4, var_betn1_t_dn5, var_betn1_t_dn6, var_betn1_t_dn7, var_betn1_t_dn8, var_betn1_t_dn9, var_betn1_t_db0, var_betn1_t_db1, var_betn1_t_db2, var_betn1_t_db3,)
    }
};
        var_betn1_t = assign5790_e5188;
        var_betn1_t_dn0 = assign5790_e5188_d_n0;
        var_betn1_t_dn1 = assign5790_e5188_d_n1;
        var_betn1_t_dn2 = assign5790_e5188_d_n2;
        var_betn1_t_dn3 = assign5790_e5188_d_n3;
        var_betn1_t_dn4 = assign5790_e5188_d_n4;
        var_betn1_t_dn5 = assign5790_e5188_d_n5;
        var_betn1_t_dn6 = assign5790_e5188_d_n6;
        var_betn1_t_dn7 = assign5790_e5188_d_n7;
        var_betn1_t_dn8 = assign5790_e5188_d_n8;
        var_betn1_t_dn9 = assign5790_e5188_d_n9;
        var_betn1_t_db0 = assign5790_e5188_d_b0;
        var_betn1_t_db1 = assign5790_e5188_d_b1;
        var_betn1_t_db2 = assign5790_e5188_d_b2;
        var_betn1_t_db3 = assign5790_e5188_d_b3;

        let (assign5800_e5199, assign5800_e5199_d_n0, assign5800_e5199_d_n1, assign5800_e5199_d_n2, assign5800_e5199_d_n3, assign5800_e5199_d_n4, assign5800_e5199_d_n5, assign5800_e5199_d_n6, assign5800_e5199_d_n7, assign5800_e5199_d_n8, assign5800_e5199_d_n9, assign5800_e5199_d_b0, assign5800_e5199_d_b1, assign5800_e5199_d_b2, assign5800_e5199_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5800_e5197: f64 = (p.p250 * var_betn1_t);
        (assign5800_e5197, (p.p250 * var_betn1_t_dn0), (p.p250 * var_betn1_t_dn1), (p.p250 * var_betn1_t_dn2), (p.p250 * var_betn1_t_dn3), (p.p250 * var_betn1_t_dn4), (p.p250 * var_betn1_t_dn5), (p.p250 * var_betn1_t_dn6), (p.p250 * var_betn1_t_dn7), (p.p250 * var_betn1_t_dn8), (p.p250 * var_betn1_t_dn9), (p.p250 * var_betn1_t_db0), (p.p250 * var_betn1_t_db1), (p.p250 * var_betn1_t_db2), (p.p250 * var_betn1_t_db3),)
    } else {
        (var_betn2_t, var_betn2_t_dn0, var_betn2_t_dn1, var_betn2_t_dn2, var_betn2_t_dn3, var_betn2_t_dn4, var_betn2_t_dn5, var_betn2_t_dn6, var_betn2_t_dn7, var_betn2_t_dn8, var_betn2_t_dn9, var_betn2_t_db0, var_betn2_t_db1, var_betn2_t_db2, var_betn2_t_db3,)
    }
};
        var_betn2_t = assign5800_e5199;
        var_betn2_t_dn0 = assign5800_e5199_d_n0;
        var_betn2_t_dn1 = assign5800_e5199_d_n1;
        var_betn2_t_dn2 = assign5800_e5199_d_n2;
        var_betn2_t_dn3 = assign5800_e5199_d_n3;
        var_betn2_t_dn4 = assign5800_e5199_d_n4;
        var_betn2_t_dn5 = assign5800_e5199_d_n5;
        var_betn2_t_dn6 = assign5800_e5199_d_n6;
        var_betn2_t_dn7 = assign5800_e5199_d_n7;
        var_betn2_t_dn8 = assign5800_e5199_d_n8;
        var_betn2_t_dn9 = assign5800_e5199_d_n9;
        var_betn2_t_db0 = assign5800_e5199_d_b0;
        var_betn2_t_db1 = assign5800_e5199_d_b1;
        var_betn2_t_db2 = assign5800_e5199_d_b2;
        var_betn2_t_db3 = assign5800_e5199_d_b3;

        let (assign5810_e5226, assign5810_e5226_d_n0, assign5810_e5226_d_n1, assign5810_e5226_d_n2, assign5810_e5226_d_n3, assign5810_e5226_d_n4, assign5810_e5226_d_n5, assign5810_e5226_d_n6, assign5810_e5226_d_n7, assign5810_e5226_d_n8, assign5810_e5226_d_n9, assign5810_e5226_d_b0, assign5810_e5226_d_b1, assign5810_e5226_d_b2, assign5810_e5226_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5810_e5208: f64 = (1.0 + var_rhobeta);
        let assign5810_e5212: f64 = (p.p462 * var_rhobetaref);
        let assign5810_e5213: f64 = (1.0 + assign5810_e5212);
        let assign5810_e5214: f64 = (assign5810_e5208 * assign5810_e5213);
        let assign5810_e5217: f64 = (1.0 + var_rhobetaref);
        let assign5810_e5221: f64 = (p.p462 * var_rhobeta);
        let assign5810_e5222: f64 = (1.0 + assign5810_e5221);
        let assign5810_e5223: f64 = (assign5810_e5217 * assign5810_e5222);
        let assign5810_e5224: f64 = (assign5810_e5214 / assign5810_e5223);
        (assign5810_e5224, (((((var_rhobeta_dn0 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn0))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn0 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn0))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn1 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn1))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn1 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn1))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn2 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn2))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn2 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn2))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn3 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn3))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn3 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn3))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn4 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn4))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn4 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn4))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn5 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn5))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn5 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn5))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn6 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn6))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn6 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn6))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn7 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn7))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn7 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn7))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn8 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn8))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn8 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn8))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_dn9 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_dn9))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_dn9 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_dn9))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_db0 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_db0))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_db0 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_db0))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_db1 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_db1))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_db1 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_db1))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_db2 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_db2))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_db2 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_db2))))) / (assign5810_e5223 * assign5810_e5223)), (((((var_rhobeta_db3 * assign5810_e5213) + (assign5810_e5208 * (p.p462 * var_rhobetaref_db3))) * assign5810_e5223) - (assign5810_e5214 * ((var_rhobetaref_db3 * assign5810_e5222) + (assign5810_e5217 * (p.p462 * var_rhobeta_db3))))) / (assign5810_e5223 * assign5810_e5223)),)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5810_e5226;
        var_temp_dn0 = assign5810_e5226_d_n0;
        var_temp_dn1 = assign5810_e5226_d_n1;
        var_temp_dn2 = assign5810_e5226_d_n2;
        var_temp_dn3 = assign5810_e5226_d_n3;
        var_temp_dn4 = assign5810_e5226_d_n4;
        var_temp_dn5 = assign5810_e5226_d_n5;
        var_temp_dn6 = assign5810_e5226_d_n6;
        var_temp_dn7 = assign5810_e5226_d_n7;
        var_temp_dn8 = assign5810_e5226_d_n8;
        var_temp_dn9 = assign5810_e5226_d_n9;
        var_temp_db0 = assign5810_e5226_d_b0;
        var_temp_db1 = assign5810_e5226_d_b1;
        var_temp_db2 = assign5810_e5226_d_b2;
        var_temp_db3 = assign5810_e5226_d_b3;

        let (assign5820_e5237, assign5820_e5237_d_n0, assign5820_e5237_d_n1, assign5820_e5237_d_n2, assign5820_e5237_d_n3, assign5820_e5237_d_n4, assign5820_e5237_d_n5, assign5820_e5237_d_n6, assign5820_e5237_d_n7, assign5820_e5237_d_n8, assign5820_e5237_d_n9, assign5820_e5237_d_b0, assign5820_e5237_d_b1, assign5820_e5237_d_b2, assign5820_e5237_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5820_e5235: f64 = (var_thesat_p * var_temp);
        (assign5820_e5235, ((var_thesat_p_dn0 * var_temp) + (var_thesat_p * var_temp_dn0)), ((var_thesat_p_dn1 * var_temp) + (var_thesat_p * var_temp_dn1)), ((var_thesat_p_dn2 * var_temp) + (var_thesat_p * var_temp_dn2)), ((var_thesat_p_dn3 * var_temp) + (var_thesat_p * var_temp_dn3)), ((var_thesat_p_dn4 * var_temp) + (var_thesat_p * var_temp_dn4)), ((var_thesat_p_dn5 * var_temp) + (var_thesat_p * var_temp_dn5)), ((var_thesat_p_dn6 * var_temp) + (var_thesat_p * var_temp_dn6)), ((var_thesat_p_dn7 * var_temp) + (var_thesat_p * var_temp_dn7)), ((var_thesat_p_dn8 * var_temp) + (var_thesat_p * var_temp_dn8)), ((var_thesat_p_dn9 * var_temp) + (var_thesat_p * var_temp_dn9)), ((var_thesat_p_db0 * var_temp) + (var_thesat_p * var_temp_db0)), ((var_thesat_p_db1 * var_temp) + (var_thesat_p * var_temp_db1)), ((var_thesat_p_db2 * var_temp) + (var_thesat_p * var_temp_db2)), ((var_thesat_p_db3 * var_temp) + (var_thesat_p * var_temp_db3)),)
    } else {
        (var_thesat_p, var_thesat_p_dn0, var_thesat_p_dn1, var_thesat_p_dn2, var_thesat_p_dn3, var_thesat_p_dn4, var_thesat_p_dn5, var_thesat_p_dn6, var_thesat_p_dn7, var_thesat_p_dn8, var_thesat_p_dn9, var_thesat_p_db0, var_thesat_p_db1, var_thesat_p_db2, var_thesat_p_db3,)
    }
};
        var_thesat_p = assign5820_e5237;
        var_thesat_p_dn0 = assign5820_e5237_d_n0;
        var_thesat_p_dn1 = assign5820_e5237_d_n1;
        var_thesat_p_dn2 = assign5820_e5237_d_n2;
        var_thesat_p_dn3 = assign5820_e5237_d_n3;
        var_thesat_p_dn4 = assign5820_e5237_d_n4;
        var_thesat_p_dn5 = assign5820_e5237_d_n5;
        var_thesat_p_dn6 = assign5820_e5237_d_n6;
        var_thesat_p_dn7 = assign5820_e5237_d_n7;
        var_thesat_p_dn8 = assign5820_e5237_d_n8;
        var_thesat_p_dn9 = assign5820_e5237_d_n9;
        var_thesat_p_db0 = assign5820_e5237_d_b0;
        var_thesat_p_db1 = assign5820_e5237_d_b1;
        var_thesat_p_db2 = assign5820_e5237_d_b2;
        var_thesat_p_db3 = assign5820_e5237_d_b3;

        let (assign5830_e5248, assign5830_e5248_d_n0, assign5830_e5248_d_n1, assign5830_e5248_d_n2, assign5830_e5248_d_n3, assign5830_e5248_d_n4, assign5830_e5248_d_n5, assign5830_e5248_d_n6, assign5830_e5248_d_n7, assign5830_e5248_d_n8, assign5830_e5248_d_n9, assign5830_e5248_d_b0, assign5830_e5248_d_b1, assign5830_e5248_d_b2, assign5830_e5248_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5830_e5246: f64 = (var_thesat_p).max(0.0);
        (assign5830_e5246, if var_thesat_p >= 0.0 { var_thesat_p_dn0 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn1 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn2 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn3 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn4 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn5 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn6 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn7 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn8 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_dn9 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db0 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db1 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db2 } else { 0.0 }, if var_thesat_p >= 0.0 { var_thesat_p_db3 } else { 0.0 },)
    } else {
        (var_thesat_t, var_thesat_t_dn0, var_thesat_t_dn1, var_thesat_t_dn2, var_thesat_t_dn3, var_thesat_t_dn4, var_thesat_t_dn5, var_thesat_t_dn6, var_thesat_t_dn7, var_thesat_t_dn8, var_thesat_t_dn9, var_thesat_t_db0, var_thesat_t_db1, var_thesat_t_db2, var_thesat_t_db3,)
    }
};
        var_thesat_t = assign5830_e5248;
        var_thesat_t_dn0 = assign5830_e5248_d_n0;
        var_thesat_t_dn1 = assign5830_e5248_d_n1;
        var_thesat_t_dn2 = assign5830_e5248_d_n2;
        var_thesat_t_dn3 = assign5830_e5248_d_n3;
        var_thesat_t_dn4 = assign5830_e5248_d_n4;
        var_thesat_t_dn5 = assign5830_e5248_d_n5;
        var_thesat_t_dn6 = assign5830_e5248_d_n6;
        var_thesat_t_dn7 = assign5830_e5248_d_n7;
        var_thesat_t_dn8 = assign5830_e5248_d_n8;
        var_thesat_t_dn9 = assign5830_e5248_d_n9;
        var_thesat_t_db0 = assign5830_e5248_d_b0;
        var_thesat_t_db1 = assign5830_e5248_d_b1;
        var_thesat_t_db2 = assign5830_e5248_d_b2;
        var_thesat_t_db3 = assign5830_e5248_d_b3;

        let (assign5840_e5259, assign5840_e5259_d_n0, assign5840_e5259_d_n1, assign5840_e5259_d_n2, assign5840_e5259_d_n3, assign5840_e5259_d_n4, assign5840_e5259_d_n5, assign5840_e5259_d_n6, assign5840_e5259_d_n7, assign5840_e5259_d_n8, assign5840_e5259_d_n9, assign5840_e5259_d_b0, assign5840_e5259_d_b1, assign5840_e5259_d_b2, assign5840_e5259_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5840_e5257: f64 = (var_thesatac_p * var_temp);
        (assign5840_e5257, ((var_thesatac_p_dn0 * var_temp) + (var_thesatac_p * var_temp_dn0)), ((var_thesatac_p_dn1 * var_temp) + (var_thesatac_p * var_temp_dn1)), ((var_thesatac_p_dn2 * var_temp) + (var_thesatac_p * var_temp_dn2)), ((var_thesatac_p_dn3 * var_temp) + (var_thesatac_p * var_temp_dn3)), ((var_thesatac_p_dn4 * var_temp) + (var_thesatac_p * var_temp_dn4)), ((var_thesatac_p_dn5 * var_temp) + (var_thesatac_p * var_temp_dn5)), ((var_thesatac_p_dn6 * var_temp) + (var_thesatac_p * var_temp_dn6)), ((var_thesatac_p_dn7 * var_temp) + (var_thesatac_p * var_temp_dn7)), ((var_thesatac_p_dn8 * var_temp) + (var_thesatac_p * var_temp_dn8)), ((var_thesatac_p_dn9 * var_temp) + (var_thesatac_p * var_temp_dn9)), ((var_thesatac_p_db0 * var_temp) + (var_thesatac_p * var_temp_db0)), ((var_thesatac_p_db1 * var_temp) + (var_thesatac_p * var_temp_db1)), ((var_thesatac_p_db2 * var_temp) + (var_thesatac_p * var_temp_db2)), ((var_thesatac_p_db3 * var_temp) + (var_thesatac_p * var_temp_db3)),)
    } else {
        (var_thesatac_p, var_thesatac_p_dn0, var_thesatac_p_dn1, var_thesatac_p_dn2, var_thesatac_p_dn3, var_thesatac_p_dn4, var_thesatac_p_dn5, var_thesatac_p_dn6, var_thesatac_p_dn7, var_thesatac_p_dn8, var_thesatac_p_dn9, var_thesatac_p_db0, var_thesatac_p_db1, var_thesatac_p_db2, var_thesatac_p_db3,)
    }
};
        var_thesatac_p = assign5840_e5259;
        var_thesatac_p_dn0 = assign5840_e5259_d_n0;
        var_thesatac_p_dn1 = assign5840_e5259_d_n1;
        var_thesatac_p_dn2 = assign5840_e5259_d_n2;
        var_thesatac_p_dn3 = assign5840_e5259_d_n3;
        var_thesatac_p_dn4 = assign5840_e5259_d_n4;
        var_thesatac_p_dn5 = assign5840_e5259_d_n5;
        var_thesatac_p_dn6 = assign5840_e5259_d_n6;
        var_thesatac_p_dn7 = assign5840_e5259_d_n7;
        var_thesatac_p_dn8 = assign5840_e5259_d_n8;
        var_thesatac_p_dn9 = assign5840_e5259_d_n9;
        var_thesatac_p_db0 = assign5840_e5259_d_b0;
        var_thesatac_p_db1 = assign5840_e5259_d_b1;
        var_thesatac_p_db2 = assign5840_e5259_d_b2;
        var_thesatac_p_db3 = assign5840_e5259_d_b3;

        *var_betn1_t_slot = var_betn1_t;
        *var_betn1_t_db0_slot = var_betn1_t_db0;
        *var_betn1_t_db1_slot = var_betn1_t_db1;
        *var_betn1_t_db2_slot = var_betn1_t_db2;
        *var_betn1_t_db3_slot = var_betn1_t_db3;
        *var_betn1_t_dn0_slot = var_betn1_t_dn0;
        *var_betn1_t_dn1_slot = var_betn1_t_dn1;
        *var_betn1_t_dn2_slot = var_betn1_t_dn2;
        *var_betn1_t_dn3_slot = var_betn1_t_dn3;
        *var_betn1_t_dn4_slot = var_betn1_t_dn4;
        *var_betn1_t_dn5_slot = var_betn1_t_dn5;
        *var_betn1_t_dn6_slot = var_betn1_t_dn6;
        *var_betn1_t_dn7_slot = var_betn1_t_dn7;
        *var_betn1_t_dn8_slot = var_betn1_t_dn8;
        *var_betn1_t_dn9_slot = var_betn1_t_dn9;
        *var_betn2_t_slot = var_betn2_t;
        *var_betn2_t_db0_slot = var_betn2_t_db0;
        *var_betn2_t_db1_slot = var_betn2_t_db1;
        *var_betn2_t_db2_slot = var_betn2_t_db2;
        *var_betn2_t_db3_slot = var_betn2_t_db3;
        *var_betn2_t_dn0_slot = var_betn2_t_dn0;
        *var_betn2_t_dn1_slot = var_betn2_t_dn1;
        *var_betn2_t_dn2_slot = var_betn2_t_dn2;
        *var_betn2_t_dn3_slot = var_betn2_t_dn3;
        *var_betn2_t_dn4_slot = var_betn2_t_dn4;
        *var_betn2_t_dn5_slot = var_betn2_t_dn5;
        *var_betn2_t_dn6_slot = var_betn2_t_dn6;
        *var_betn2_t_dn7_slot = var_betn2_t_dn7;
        *var_betn2_t_dn8_slot = var_betn2_t_dn8;
        *var_betn2_t_dn9_slot = var_betn2_t_dn9;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_db0_slot = var_betn_p_db0;
        *var_betn_p_db1_slot = var_betn_p_db1;
        *var_betn_p_db2_slot = var_betn_p_db2;
        *var_betn_p_db3_slot = var_betn_p_db3;
        *var_betn_p_dn0_slot = var_betn_p_dn0;
        *var_betn_p_dn1_slot = var_betn_p_dn1;
        *var_betn_p_dn2_slot = var_betn_p_dn2;
        *var_betn_p_dn3_slot = var_betn_p_dn3;
        *var_betn_p_dn4_slot = var_betn_p_dn4;
        *var_betn_p_dn5_slot = var_betn_p_dn5;
        *var_betn_p_dn6_slot = var_betn_p_dn6;
        *var_betn_p_dn7_slot = var_betn_p_dn7;
        *var_betn_p_dn8_slot = var_betn_p_dn8;
        *var_betn_p_dn9_slot = var_betn_p_dn9;
        *var_iloop_slot = var_iloop;
        *var_invsa_slot = var_invsa;
        *var_invsa_db0_slot = var_invsa_db0;
        *var_invsa_db1_slot = var_invsa_db1;
        *var_invsa_db2_slot = var_invsa_db2;
        *var_invsa_db3_slot = var_invsa_db3;
        *var_invsa_dn0_slot = var_invsa_dn0;
        *var_invsa_dn1_slot = var_invsa_dn1;
        *var_invsa_dn2_slot = var_invsa_dn2;
        *var_invsa_dn3_slot = var_invsa_dn3;
        *var_invsa_dn4_slot = var_invsa_dn4;
        *var_invsa_dn5_slot = var_invsa_dn5;
        *var_invsa_dn6_slot = var_invsa_dn6;
        *var_invsa_dn7_slot = var_invsa_dn7;
        *var_invsa_dn8_slot = var_invsa_dn8;
        *var_invsa_dn9_slot = var_invsa_dn9;
        *var_invsaref_slot = var_invsaref;
        *var_invsb_slot = var_invsb;
        *var_invsbref_slot = var_invsbref;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_db0_slot = var_kstressu0_db0;
        *var_kstressu0_db1_slot = var_kstressu0_db1;
        *var_kstressu0_db2_slot = var_kstressu0_db2;
        *var_kstressu0_db3_slot = var_kstressu0_db3;
        *var_kstressu0_dn0_slot = var_kstressu0_dn0;
        *var_kstressu0_dn1_slot = var_kstressu0_dn1;
        *var_kstressu0_dn2_slot = var_kstressu0_dn2;
        *var_kstressu0_dn3_slot = var_kstressu0_dn3;
        *var_kstressu0_dn4_slot = var_kstressu0_dn4;
        *var_kstressu0_dn5_slot = var_kstressu0_dn5;
        *var_kstressu0_dn6_slot = var_kstressu0_dn6;
        *var_kstressu0_dn7_slot = var_kstressu0_dn7;
        *var_kstressu0_dn8_slot = var_kstressu0_dn8;
        *var_kstressu0_dn9_slot = var_kstressu0_dn9;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_lx_slot = var_lx;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_db0_slot = var_rhobeta_db0;
        *var_rhobeta_db1_slot = var_rhobeta_db1;
        *var_rhobeta_db2_slot = var_rhobeta_db2;
        *var_rhobeta_db3_slot = var_rhobeta_db3;
        *var_rhobeta_dn0_slot = var_rhobeta_dn0;
        *var_rhobeta_dn1_slot = var_rhobeta_dn1;
        *var_rhobeta_dn2_slot = var_rhobeta_dn2;
        *var_rhobeta_dn3_slot = var_rhobeta_dn3;
        *var_rhobeta_dn4_slot = var_rhobeta_dn4;
        *var_rhobeta_dn5_slot = var_rhobeta_dn5;
        *var_rhobeta_dn6_slot = var_rhobeta_dn6;
        *var_rhobeta_dn7_slot = var_rhobeta_dn7;
        *var_rhobeta_dn8_slot = var_rhobeta_dn8;
        *var_rhobeta_dn9_slot = var_rhobeta_dn9;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_db0_slot = var_rhobetaref_db0;
        *var_rhobetaref_db1_slot = var_rhobetaref_db1;
        *var_rhobetaref_db2_slot = var_rhobetaref_db2;
        *var_rhobetaref_db3_slot = var_rhobetaref_db3;
        *var_rhobetaref_dn0_slot = var_rhobetaref_dn0;
        *var_rhobetaref_dn1_slot = var_rhobetaref_dn1;
        *var_rhobetaref_dn2_slot = var_rhobetaref_dn2;
        *var_rhobetaref_dn3_slot = var_rhobetaref_dn3;
        *var_rhobetaref_dn4_slot = var_rhobetaref_dn4;
        *var_rhobetaref_dn5_slot = var_rhobetaref_dn5;
        *var_rhobetaref_dn6_slot = var_rhobetaref_dn6;
        *var_rhobetaref_dn7_slot = var_rhobetaref_dn7;
        *var_rhobetaref_dn8_slot = var_rhobetaref_dn8;
        *var_rhobetaref_dn9_slot = var_rhobetaref_dn9;
        *var_temp_slot = var_temp;
        *var_temp0__blk79_slot = var_temp0__blk79;
        *var_temp0__blk79_db0_slot = var_temp0__blk79_db0;
        *var_temp0__blk79_db1_slot = var_temp0__blk79_db1;
        *var_temp0__blk79_db2_slot = var_temp0__blk79_db2;
        *var_temp0__blk79_db3_slot = var_temp0__blk79_db3;
        *var_temp0__blk79_dn0_slot = var_temp0__blk79_dn0;
        *var_temp0__blk79_dn1_slot = var_temp0__blk79_dn1;
        *var_temp0__blk79_dn2_slot = var_temp0__blk79_dn2;
        *var_temp0__blk79_dn3_slot = var_temp0__blk79_dn3;
        *var_temp0__blk79_dn4_slot = var_temp0__blk79_dn4;
        *var_temp0__blk79_dn5_slot = var_temp0__blk79_dn5;
        *var_temp0__blk79_dn6_slot = var_temp0__blk79_dn6;
        *var_temp0__blk79_dn7_slot = var_temp0__blk79_dn7;
        *var_temp0__blk79_dn8_slot = var_temp0__blk79_dn8;
        *var_temp0__blk79_dn9_slot = var_temp0__blk79_dn9;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_templ_slot = var_templ;
        *var_tempw_slot = var_tempw;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_db0_slot = var_thesat_p_db0;
        *var_thesat_p_db1_slot = var_thesat_p_db1;
        *var_thesat_p_db2_slot = var_thesat_p_db2;
        *var_thesat_p_db3_slot = var_thesat_p_db3;
        *var_thesat_p_dn0_slot = var_thesat_p_dn0;
        *var_thesat_p_dn1_slot = var_thesat_p_dn1;
        *var_thesat_p_dn2_slot = var_thesat_p_dn2;
        *var_thesat_p_dn3_slot = var_thesat_p_dn3;
        *var_thesat_p_dn4_slot = var_thesat_p_dn4;
        *var_thesat_p_dn5_slot = var_thesat_p_dn5;
        *var_thesat_p_dn6_slot = var_thesat_p_dn6;
        *var_thesat_p_dn7_slot = var_thesat_p_dn7;
        *var_thesat_p_dn8_slot = var_thesat_p_dn8;
        *var_thesat_p_dn9_slot = var_thesat_p_dn9;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_db0_slot = var_thesat_t_db0;
        *var_thesat_t_db1_slot = var_thesat_t_db1;
        *var_thesat_t_db2_slot = var_thesat_t_db2;
        *var_thesat_t_db3_slot = var_thesat_t_db3;
        *var_thesat_t_dn0_slot = var_thesat_t_dn0;
        *var_thesat_t_dn1_slot = var_thesat_t_dn1;
        *var_thesat_t_dn2_slot = var_thesat_t_dn2;
        *var_thesat_t_dn3_slot = var_thesat_t_dn3;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_dn5_slot = var_thesat_t_dn5;
        *var_thesat_t_dn6_slot = var_thesat_t_dn6;
        *var_thesat_t_dn7_slot = var_thesat_t_dn7;
        *var_thesat_t_dn8_slot = var_thesat_t_dn8;
        *var_thesat_t_dn9_slot = var_thesat_t_dn9;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_db0_slot = var_thesatac_p_db0;
        *var_thesatac_p_db1_slot = var_thesatac_p_db1;
        *var_thesatac_p_db2_slot = var_thesatac_p_db2;
        *var_thesatac_p_db3_slot = var_thesatac_p_db3;
        *var_thesatac_p_dn0_slot = var_thesatac_p_dn0;
        *var_thesatac_p_dn1_slot = var_thesatac_p_dn1;
        *var_thesatac_p_dn2_slot = var_thesatac_p_dn2;
        *var_thesatac_p_dn3_slot = var_thesatac_p_dn3;
        *var_thesatac_p_dn4_slot = var_thesatac_p_dn4;
        *var_thesatac_p_dn5_slot = var_thesatac_p_dn5;
        *var_thesatac_p_dn6_slot = var_thesatac_p_dn6;
        *var_thesatac_p_dn7_slot = var_thesatac_p_dn7;
        *var_thesatac_p_dn8_slot = var_thesatac_p_dn8;
        *var_thesatac_p_dn9_slot = var_thesatac_p_dn9;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_db0_slot = var_tmpa_db0;
        *var_tmpa_db1_slot = var_tmpa_db1;
        *var_tmpa_db2_slot = var_tmpa_db2;
        *var_tmpa_db3_slot = var_tmpa_db3;
        *var_tmpa_dn0_slot = var_tmpa_dn0;
        *var_tmpa_dn1_slot = var_tmpa_dn1;
        *var_tmpa_dn2_slot = var_tmpa_dn2;
        *var_tmpa_dn3_slot = var_tmpa_dn3;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn5_slot = var_tmpa_dn5;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_tmpb_slot = var_tmpb;
        *var_wx_slot = var_wx;
    }

    pub(super) fn stamp_transient_block_15(
        p: &Parameters,
        var_guard133: f64,
        var_guard134: f64,
        var_guard83: f64,
        var_kstressvth0: f64,
        var_temp0__blk79: f64,
        var_temp0__blk79_db0: f64,
        var_temp0__blk79_db1: f64,
        var_temp0__blk79_db2: f64,
        var_temp0__blk79_db3: f64,
        var_temp0__blk79_dn0: f64,
        var_temp0__blk79_dn1: f64,
        var_temp0__blk79_dn2: f64,
        var_temp0__blk79_dn3: f64,
        var_temp0__blk79_dn4: f64,
        var_temp0__blk79_dn5: f64,
        var_temp0__blk79_dn6: f64,
        var_temp0__blk79_dn7: f64,
        var_temp0__blk79_dn8: f64,
        var_temp0__blk79_dn9: f64,
        var_thesatac_p: f64,
        var_thesatac_p_db0: f64,
        var_thesatac_p_db1: f64,
        var_thesatac_p_db2: f64,
        var_thesatac_p_db3: f64,
        var_thesatac_p_dn0: f64,
        var_thesatac_p_dn1: f64,
        var_thesatac_p_dn2: f64,
        var_thesatac_p_dn3: f64,
        var_thesatac_p_dn4: f64,
        var_thesatac_p_dn5: f64,
        var_thesatac_p_dn6: f64,
        var_thesatac_p_dn7: f64,
        var_thesatac_p_dn8: f64,
        var_thesatac_p_dn9: f64,
        var_tox1_i: f64,
        var_tox2_i: f64,
        var_cf1_t_slot: &mut f64,
        var_cf1_t_db0_slot: &mut f64,
        var_cf1_t_db1_slot: &mut f64,
        var_cf1_t_db2_slot: &mut f64,
        var_cf1_t_db3_slot: &mut f64,
        var_cf1_t_dn0_slot: &mut f64,
        var_cf1_t_dn1_slot: &mut f64,
        var_cf1_t_dn2_slot: &mut f64,
        var_cf1_t_dn3_slot: &mut f64,
        var_cf1_t_dn4_slot: &mut f64,
        var_cf1_t_dn5_slot: &mut f64,
        var_cf1_t_dn6_slot: &mut f64,
        var_cf1_t_dn7_slot: &mut f64,
        var_cf1_t_dn8_slot: &mut f64,
        var_cf1_t_dn9_slot: &mut f64,
        var_cf2_t_slot: &mut f64,
        var_cf2_t_db0_slot: &mut f64,
        var_cf2_t_db1_slot: &mut f64,
        var_cf2_t_db2_slot: &mut f64,
        var_cf2_t_db3_slot: &mut f64,
        var_cf2_t_dn0_slot: &mut f64,
        var_cf2_t_dn1_slot: &mut f64,
        var_cf2_t_dn2_slot: &mut f64,
        var_cf2_t_dn3_slot: &mut f64,
        var_cf2_t_dn4_slot: &mut f64,
        var_cf2_t_dn5_slot: &mut f64,
        var_cf2_t_dn6_slot: &mut f64,
        var_cf2_t_dn7_slot: &mut f64,
        var_cf2_t_dn8_slot: &mut f64,
        var_cf2_t_dn9_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_db0_slot: &mut f64,
        var_cf_p_db1_slot: &mut f64,
        var_cf_p_db2_slot: &mut f64,
        var_cf_p_db3_slot: &mut f64,
        var_cf_p_dn0_slot: &mut f64,
        var_cf_p_dn1_slot: &mut f64,
        var_cf_p_dn2_slot: &mut f64,
        var_cf_p_dn3_slot: &mut f64,
        var_cf_p_dn4_slot: &mut f64,
        var_cf_p_dn5_slot: &mut f64,
        var_cf_p_dn6_slot: &mut f64,
        var_cf_p_dn7_slot: &mut f64,
        var_cf_p_dn8_slot: &mut f64,
        var_cf_p_dn9_slot: &mut f64,
        var_cfac1_t_slot: &mut f64,
        var_cfac1_t_db0_slot: &mut f64,
        var_cfac1_t_db1_slot: &mut f64,
        var_cfac1_t_db2_slot: &mut f64,
        var_cfac1_t_db3_slot: &mut f64,
        var_cfac1_t_dn0_slot: &mut f64,
        var_cfac1_t_dn1_slot: &mut f64,
        var_cfac1_t_dn2_slot: &mut f64,
        var_cfac1_t_dn3_slot: &mut f64,
        var_cfac1_t_dn4_slot: &mut f64,
        var_cfac1_t_dn5_slot: &mut f64,
        var_cfac1_t_dn6_slot: &mut f64,
        var_cfac1_t_dn7_slot: &mut f64,
        var_cfac1_t_dn8_slot: &mut f64,
        var_cfac1_t_dn9_slot: &mut f64,
        var_cfac2_t_slot: &mut f64,
        var_cfac2_t_db0_slot: &mut f64,
        var_cfac2_t_db1_slot: &mut f64,
        var_cfac2_t_db2_slot: &mut f64,
        var_cfac2_t_db3_slot: &mut f64,
        var_cfac2_t_dn0_slot: &mut f64,
        var_cfac2_t_dn1_slot: &mut f64,
        var_cfac2_t_dn2_slot: &mut f64,
        var_cfac2_t_dn3_slot: &mut f64,
        var_cfac2_t_dn4_slot: &mut f64,
        var_cfac2_t_dn5_slot: &mut f64,
        var_cfac2_t_dn6_slot: &mut f64,
        var_cfac2_t_dn7_slot: &mut f64,
        var_cfac2_t_dn8_slot: &mut f64,
        var_cfac2_t_dn9_slot: &mut f64,
        var_cfac_p_slot: &mut f64,
        var_cfac_p_db0_slot: &mut f64,
        var_cfac_p_db1_slot: &mut f64,
        var_cfac_p_db2_slot: &mut f64,
        var_cfac_p_db3_slot: &mut f64,
        var_cfac_p_dn0_slot: &mut f64,
        var_cfac_p_dn1_slot: &mut f64,
        var_cfac_p_dn2_slot: &mut f64,
        var_cfac_p_dn3_slot: &mut f64,
        var_cfac_p_dn4_slot: &mut f64,
        var_cfac_p_dn5_slot: &mut f64,
        var_cfac_p_dn6_slot: &mut f64,
        var_cfac_p_dn7_slot: &mut f64,
        var_cfac_p_dn8_slot: &mut f64,
        var_cfac_p_dn9_slot: &mut f64,
        var_iloop_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_db0_slot: &mut f64,
        var_temp_db1_slot: &mut f64,
        var_temp_db2_slot: &mut f64,
        var_temp_db3_slot: &mut f64,
        var_temp_dn0_slot: &mut f64,
        var_temp_dn1_slot: &mut f64,
        var_temp_dn2_slot: &mut f64,
        var_temp_dn3_slot: &mut f64,
        var_temp_dn4_slot: &mut f64,
        var_temp_dn5_slot: &mut f64,
        var_temp_dn6_slot: &mut f64,
        var_temp_dn7_slot: &mut f64,
        var_temp_dn8_slot: &mut f64,
        var_temp_dn9_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_db0_slot: &mut f64,
        var_thesatac_t_db1_slot: &mut f64,
        var_thesatac_t_db2_slot: &mut f64,
        var_thesatac_t_db3_slot: &mut f64,
        var_thesatac_t_dn0_slot: &mut f64,
        var_thesatac_t_dn1_slot: &mut f64,
        var_thesatac_t_dn2_slot: &mut f64,
        var_thesatac_t_dn3_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_dn5_slot: &mut f64,
        var_thesatac_t_dn6_slot: &mut f64,
        var_thesatac_t_dn7_slot: &mut f64,
        var_thesatac_t_dn8_slot: &mut f64,
        var_thesatac_t_dn9_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_db0_slot: &mut f64,
        var_tmpa_db1_slot: &mut f64,
        var_tmpa_db2_slot: &mut f64,
        var_tmpa_db3_slot: &mut f64,
        var_tmpa_dn0_slot: &mut f64,
        var_tmpa_dn1_slot: &mut f64,
        var_tmpa_dn2_slot: &mut f64,
        var_tmpa_dn3_slot: &mut f64,
        var_tmpa_dn4_slot: &mut f64,
        var_tmpa_dn5_slot: &mut f64,
        var_tmpa_dn6_slot: &mut f64,
        var_tmpa_dn7_slot: &mut f64,
        var_tmpa_dn8_slot: &mut f64,
        var_tmpa_dn9_slot: &mut f64,
        var_vfb1_t_slot: &mut f64,
        var_vfb1_t_db0_slot: &mut f64,
        var_vfb1_t_db1_slot: &mut f64,
        var_vfb1_t_db2_slot: &mut f64,
        var_vfb1_t_db3_slot: &mut f64,
        var_vfb1_t_dn0_slot: &mut f64,
        var_vfb1_t_dn1_slot: &mut f64,
        var_vfb1_t_dn2_slot: &mut f64,
        var_vfb1_t_dn3_slot: &mut f64,
        var_vfb1_t_dn4_slot: &mut f64,
        var_vfb1_t_dn5_slot: &mut f64,
        var_vfb1_t_dn6_slot: &mut f64,
        var_vfb1_t_dn7_slot: &mut f64,
        var_vfb1_t_dn8_slot: &mut f64,
        var_vfb1_t_dn9_slot: &mut f64,
        var_vfb2_t_slot: &mut f64,
        var_vfb2_t_db0_slot: &mut f64,
        var_vfb2_t_db1_slot: &mut f64,
        var_vfb2_t_db2_slot: &mut f64,
        var_vfb2_t_db3_slot: &mut f64,
        var_vfb2_t_dn0_slot: &mut f64,
        var_vfb2_t_dn1_slot: &mut f64,
        var_vfb2_t_dn2_slot: &mut f64,
        var_vfb2_t_dn3_slot: &mut f64,
        var_vfb2_t_dn4_slot: &mut f64,
        var_vfb2_t_dn5_slot: &mut f64,
        var_vfb2_t_dn6_slot: &mut f64,
        var_vfb2_t_dn7_slot: &mut f64,
        var_vfb2_t_dn8_slot: &mut f64,
        var_vfb2_t_dn9_slot: &mut f64,
        var_vfbac1_t_slot: &mut f64,
        var_vfbac1_t_db0_slot: &mut f64,
        var_vfbac1_t_db1_slot: &mut f64,
        var_vfbac1_t_db2_slot: &mut f64,
        var_vfbac1_t_db3_slot: &mut f64,
        var_vfbac1_t_dn0_slot: &mut f64,
        var_vfbac1_t_dn1_slot: &mut f64,
        var_vfbac1_t_dn2_slot: &mut f64,
        var_vfbac1_t_dn3_slot: &mut f64,
        var_vfbac1_t_dn4_slot: &mut f64,
        var_vfbac1_t_dn5_slot: &mut f64,
        var_vfbac1_t_dn6_slot: &mut f64,
        var_vfbac1_t_dn7_slot: &mut f64,
        var_vfbac1_t_dn8_slot: &mut f64,
        var_vfbac1_t_dn9_slot: &mut f64,
        var_vfbac2_t_slot: &mut f64,
        var_vfbac2_t_db0_slot: &mut f64,
        var_vfbac2_t_db1_slot: &mut f64,
        var_vfbac2_t_db2_slot: &mut f64,
        var_vfbac2_t_db3_slot: &mut f64,
        var_vfbac2_t_dn0_slot: &mut f64,
        var_vfbac2_t_dn1_slot: &mut f64,
        var_vfbac2_t_dn2_slot: &mut f64,
        var_vfbac2_t_dn3_slot: &mut f64,
        var_vfbac2_t_dn4_slot: &mut f64,
        var_vfbac2_t_dn5_slot: &mut f64,
        var_vfbac2_t_dn6_slot: &mut f64,
        var_vfbac2_t_dn7_slot: &mut f64,
        var_vfbac2_t_dn8_slot: &mut f64,
        var_vfbac2_t_dn9_slot: &mut f64,
    ) {
        let mut var_cf1_t: f64 = *var_cf1_t_slot;
        let mut var_cf1_t_db0: f64 = *var_cf1_t_db0_slot;
        let mut var_cf1_t_db1: f64 = *var_cf1_t_db1_slot;
        let mut var_cf1_t_db2: f64 = *var_cf1_t_db2_slot;
        let mut var_cf1_t_db3: f64 = *var_cf1_t_db3_slot;
        let mut var_cf1_t_dn0: f64 = *var_cf1_t_dn0_slot;
        let mut var_cf1_t_dn1: f64 = *var_cf1_t_dn1_slot;
        let mut var_cf1_t_dn2: f64 = *var_cf1_t_dn2_slot;
        let mut var_cf1_t_dn3: f64 = *var_cf1_t_dn3_slot;
        let mut var_cf1_t_dn4: f64 = *var_cf1_t_dn4_slot;
        let mut var_cf1_t_dn5: f64 = *var_cf1_t_dn5_slot;
        let mut var_cf1_t_dn6: f64 = *var_cf1_t_dn6_slot;
        let mut var_cf1_t_dn7: f64 = *var_cf1_t_dn7_slot;
        let mut var_cf1_t_dn8: f64 = *var_cf1_t_dn8_slot;
        let mut var_cf1_t_dn9: f64 = *var_cf1_t_dn9_slot;
        let mut var_cf2_t: f64 = *var_cf2_t_slot;
        let mut var_cf2_t_db0: f64 = *var_cf2_t_db0_slot;
        let mut var_cf2_t_db1: f64 = *var_cf2_t_db1_slot;
        let mut var_cf2_t_db2: f64 = *var_cf2_t_db2_slot;
        let mut var_cf2_t_db3: f64 = *var_cf2_t_db3_slot;
        let mut var_cf2_t_dn0: f64 = *var_cf2_t_dn0_slot;
        let mut var_cf2_t_dn1: f64 = *var_cf2_t_dn1_slot;
        let mut var_cf2_t_dn2: f64 = *var_cf2_t_dn2_slot;
        let mut var_cf2_t_dn3: f64 = *var_cf2_t_dn3_slot;
        let mut var_cf2_t_dn4: f64 = *var_cf2_t_dn4_slot;
        let mut var_cf2_t_dn5: f64 = *var_cf2_t_dn5_slot;
        let mut var_cf2_t_dn6: f64 = *var_cf2_t_dn6_slot;
        let mut var_cf2_t_dn7: f64 = *var_cf2_t_dn7_slot;
        let mut var_cf2_t_dn8: f64 = *var_cf2_t_dn8_slot;
        let mut var_cf2_t_dn9: f64 = *var_cf2_t_dn9_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_db0: f64 = *var_cf_p_db0_slot;
        let mut var_cf_p_db1: f64 = *var_cf_p_db1_slot;
        let mut var_cf_p_db2: f64 = *var_cf_p_db2_slot;
        let mut var_cf_p_db3: f64 = *var_cf_p_db3_slot;
        let mut var_cf_p_dn0: f64 = *var_cf_p_dn0_slot;
        let mut var_cf_p_dn1: f64 = *var_cf_p_dn1_slot;
        let mut var_cf_p_dn2: f64 = *var_cf_p_dn2_slot;
        let mut var_cf_p_dn3: f64 = *var_cf_p_dn3_slot;
        let mut var_cf_p_dn4: f64 = *var_cf_p_dn4_slot;
        let mut var_cf_p_dn5: f64 = *var_cf_p_dn5_slot;
        let mut var_cf_p_dn6: f64 = *var_cf_p_dn6_slot;
        let mut var_cf_p_dn7: f64 = *var_cf_p_dn7_slot;
        let mut var_cf_p_dn8: f64 = *var_cf_p_dn8_slot;
        let mut var_cf_p_dn9: f64 = *var_cf_p_dn9_slot;
        let mut var_cfac1_t: f64 = *var_cfac1_t_slot;
        let mut var_cfac1_t_db0: f64 = *var_cfac1_t_db0_slot;
        let mut var_cfac1_t_db1: f64 = *var_cfac1_t_db1_slot;
        let mut var_cfac1_t_db2: f64 = *var_cfac1_t_db2_slot;
        let mut var_cfac1_t_db3: f64 = *var_cfac1_t_db3_slot;
        let mut var_cfac1_t_dn0: f64 = *var_cfac1_t_dn0_slot;
        let mut var_cfac1_t_dn1: f64 = *var_cfac1_t_dn1_slot;
        let mut var_cfac1_t_dn2: f64 = *var_cfac1_t_dn2_slot;
        let mut var_cfac1_t_dn3: f64 = *var_cfac1_t_dn3_slot;
        let mut var_cfac1_t_dn4: f64 = *var_cfac1_t_dn4_slot;
        let mut var_cfac1_t_dn5: f64 = *var_cfac1_t_dn5_slot;
        let mut var_cfac1_t_dn6: f64 = *var_cfac1_t_dn6_slot;
        let mut var_cfac1_t_dn7: f64 = *var_cfac1_t_dn7_slot;
        let mut var_cfac1_t_dn8: f64 = *var_cfac1_t_dn8_slot;
        let mut var_cfac1_t_dn9: f64 = *var_cfac1_t_dn9_slot;
        let mut var_cfac2_t: f64 = *var_cfac2_t_slot;
        let mut var_cfac2_t_db0: f64 = *var_cfac2_t_db0_slot;
        let mut var_cfac2_t_db1: f64 = *var_cfac2_t_db1_slot;
        let mut var_cfac2_t_db2: f64 = *var_cfac2_t_db2_slot;
        let mut var_cfac2_t_db3: f64 = *var_cfac2_t_db3_slot;
        let mut var_cfac2_t_dn0: f64 = *var_cfac2_t_dn0_slot;
        let mut var_cfac2_t_dn1: f64 = *var_cfac2_t_dn1_slot;
        let mut var_cfac2_t_dn2: f64 = *var_cfac2_t_dn2_slot;
        let mut var_cfac2_t_dn3: f64 = *var_cfac2_t_dn3_slot;
        let mut var_cfac2_t_dn4: f64 = *var_cfac2_t_dn4_slot;
        let mut var_cfac2_t_dn5: f64 = *var_cfac2_t_dn5_slot;
        let mut var_cfac2_t_dn6: f64 = *var_cfac2_t_dn6_slot;
        let mut var_cfac2_t_dn7: f64 = *var_cfac2_t_dn7_slot;
        let mut var_cfac2_t_dn8: f64 = *var_cfac2_t_dn8_slot;
        let mut var_cfac2_t_dn9: f64 = *var_cfac2_t_dn9_slot;
        let mut var_cfac_p: f64 = *var_cfac_p_slot;
        let mut var_cfac_p_db0: f64 = *var_cfac_p_db0_slot;
        let mut var_cfac_p_db1: f64 = *var_cfac_p_db1_slot;
        let mut var_cfac_p_db2: f64 = *var_cfac_p_db2_slot;
        let mut var_cfac_p_db3: f64 = *var_cfac_p_db3_slot;
        let mut var_cfac_p_dn0: f64 = *var_cfac_p_dn0_slot;
        let mut var_cfac_p_dn1: f64 = *var_cfac_p_dn1_slot;
        let mut var_cfac_p_dn2: f64 = *var_cfac_p_dn2_slot;
        let mut var_cfac_p_dn3: f64 = *var_cfac_p_dn3_slot;
        let mut var_cfac_p_dn4: f64 = *var_cfac_p_dn4_slot;
        let mut var_cfac_p_dn5: f64 = *var_cfac_p_dn5_slot;
        let mut var_cfac_p_dn6: f64 = *var_cfac_p_dn6_slot;
        let mut var_cfac_p_dn7: f64 = *var_cfac_p_dn7_slot;
        let mut var_cfac_p_dn8: f64 = *var_cfac_p_dn8_slot;
        let mut var_cfac_p_dn9: f64 = *var_cfac_p_dn9_slot;
        let mut var_iloop: f64 = *var_iloop_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_db0: f64 = *var_temp_db0_slot;
        let mut var_temp_db1: f64 = *var_temp_db1_slot;
        let mut var_temp_db2: f64 = *var_temp_db2_slot;
        let mut var_temp_db3: f64 = *var_temp_db3_slot;
        let mut var_temp_dn0: f64 = *var_temp_dn0_slot;
        let mut var_temp_dn1: f64 = *var_temp_dn1_slot;
        let mut var_temp_dn2: f64 = *var_temp_dn2_slot;
        let mut var_temp_dn3: f64 = *var_temp_dn3_slot;
        let mut var_temp_dn4: f64 = *var_temp_dn4_slot;
        let mut var_temp_dn5: f64 = *var_temp_dn5_slot;
        let mut var_temp_dn6: f64 = *var_temp_dn6_slot;
        let mut var_temp_dn7: f64 = *var_temp_dn7_slot;
        let mut var_temp_dn8: f64 = *var_temp_dn8_slot;
        let mut var_temp_dn9: f64 = *var_temp_dn9_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_db0: f64 = *var_thesatac_t_db0_slot;
        let mut var_thesatac_t_db1: f64 = *var_thesatac_t_db1_slot;
        let mut var_thesatac_t_db2: f64 = *var_thesatac_t_db2_slot;
        let mut var_thesatac_t_db3: f64 = *var_thesatac_t_db3_slot;
        let mut var_thesatac_t_dn0: f64 = *var_thesatac_t_dn0_slot;
        let mut var_thesatac_t_dn1: f64 = *var_thesatac_t_dn1_slot;
        let mut var_thesatac_t_dn2: f64 = *var_thesatac_t_dn2_slot;
        let mut var_thesatac_t_dn3: f64 = *var_thesatac_t_dn3_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_dn5: f64 = *var_thesatac_t_dn5_slot;
        let mut var_thesatac_t_dn6: f64 = *var_thesatac_t_dn6_slot;
        let mut var_thesatac_t_dn7: f64 = *var_thesatac_t_dn7_slot;
        let mut var_thesatac_t_dn8: f64 = *var_thesatac_t_dn8_slot;
        let mut var_thesatac_t_dn9: f64 = *var_thesatac_t_dn9_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_db0: f64 = *var_tmpa_db0_slot;
        let mut var_tmpa_db1: f64 = *var_tmpa_db1_slot;
        let mut var_tmpa_db2: f64 = *var_tmpa_db2_slot;
        let mut var_tmpa_db3: f64 = *var_tmpa_db3_slot;
        let mut var_tmpa_dn0: f64 = *var_tmpa_dn0_slot;
        let mut var_tmpa_dn1: f64 = *var_tmpa_dn1_slot;
        let mut var_tmpa_dn2: f64 = *var_tmpa_dn2_slot;
        let mut var_tmpa_dn3: f64 = *var_tmpa_dn3_slot;
        let mut var_tmpa_dn4: f64 = *var_tmpa_dn4_slot;
        let mut var_tmpa_dn5: f64 = *var_tmpa_dn5_slot;
        let mut var_tmpa_dn6: f64 = *var_tmpa_dn6_slot;
        let mut var_tmpa_dn7: f64 = *var_tmpa_dn7_slot;
        let mut var_tmpa_dn8: f64 = *var_tmpa_dn8_slot;
        let mut var_tmpa_dn9: f64 = *var_tmpa_dn9_slot;
        let mut var_vfb1_t: f64 = *var_vfb1_t_slot;
        let mut var_vfb1_t_db0: f64 = *var_vfb1_t_db0_slot;
        let mut var_vfb1_t_db1: f64 = *var_vfb1_t_db1_slot;
        let mut var_vfb1_t_db2: f64 = *var_vfb1_t_db2_slot;
        let mut var_vfb1_t_db3: f64 = *var_vfb1_t_db3_slot;
        let mut var_vfb1_t_dn0: f64 = *var_vfb1_t_dn0_slot;
        let mut var_vfb1_t_dn1: f64 = *var_vfb1_t_dn1_slot;
        let mut var_vfb1_t_dn2: f64 = *var_vfb1_t_dn2_slot;
        let mut var_vfb1_t_dn3: f64 = *var_vfb1_t_dn3_slot;
        let mut var_vfb1_t_dn4: f64 = *var_vfb1_t_dn4_slot;
        let mut var_vfb1_t_dn5: f64 = *var_vfb1_t_dn5_slot;
        let mut var_vfb1_t_dn6: f64 = *var_vfb1_t_dn6_slot;
        let mut var_vfb1_t_dn7: f64 = *var_vfb1_t_dn7_slot;
        let mut var_vfb1_t_dn8: f64 = *var_vfb1_t_dn8_slot;
        let mut var_vfb1_t_dn9: f64 = *var_vfb1_t_dn9_slot;
        let mut var_vfb2_t: f64 = *var_vfb2_t_slot;
        let mut var_vfb2_t_db0: f64 = *var_vfb2_t_db0_slot;
        let mut var_vfb2_t_db1: f64 = *var_vfb2_t_db1_slot;
        let mut var_vfb2_t_db2: f64 = *var_vfb2_t_db2_slot;
        let mut var_vfb2_t_db3: f64 = *var_vfb2_t_db3_slot;
        let mut var_vfb2_t_dn0: f64 = *var_vfb2_t_dn0_slot;
        let mut var_vfb2_t_dn1: f64 = *var_vfb2_t_dn1_slot;
        let mut var_vfb2_t_dn2: f64 = *var_vfb2_t_dn2_slot;
        let mut var_vfb2_t_dn3: f64 = *var_vfb2_t_dn3_slot;
        let mut var_vfb2_t_dn4: f64 = *var_vfb2_t_dn4_slot;
        let mut var_vfb2_t_dn5: f64 = *var_vfb2_t_dn5_slot;
        let mut var_vfb2_t_dn6: f64 = *var_vfb2_t_dn6_slot;
        let mut var_vfb2_t_dn7: f64 = *var_vfb2_t_dn7_slot;
        let mut var_vfb2_t_dn8: f64 = *var_vfb2_t_dn8_slot;
        let mut var_vfb2_t_dn9: f64 = *var_vfb2_t_dn9_slot;
        let mut var_vfbac1_t: f64 = *var_vfbac1_t_slot;
        let mut var_vfbac1_t_db0: f64 = *var_vfbac1_t_db0_slot;
        let mut var_vfbac1_t_db1: f64 = *var_vfbac1_t_db1_slot;
        let mut var_vfbac1_t_db2: f64 = *var_vfbac1_t_db2_slot;
        let mut var_vfbac1_t_db3: f64 = *var_vfbac1_t_db3_slot;
        let mut var_vfbac1_t_dn0: f64 = *var_vfbac1_t_dn0_slot;
        let mut var_vfbac1_t_dn1: f64 = *var_vfbac1_t_dn1_slot;
        let mut var_vfbac1_t_dn2: f64 = *var_vfbac1_t_dn2_slot;
        let mut var_vfbac1_t_dn3: f64 = *var_vfbac1_t_dn3_slot;
        let mut var_vfbac1_t_dn4: f64 = *var_vfbac1_t_dn4_slot;
        let mut var_vfbac1_t_dn5: f64 = *var_vfbac1_t_dn5_slot;
        let mut var_vfbac1_t_dn6: f64 = *var_vfbac1_t_dn6_slot;
        let mut var_vfbac1_t_dn7: f64 = *var_vfbac1_t_dn7_slot;
        let mut var_vfbac1_t_dn8: f64 = *var_vfbac1_t_dn8_slot;
        let mut var_vfbac1_t_dn9: f64 = *var_vfbac1_t_dn9_slot;
        let mut var_vfbac2_t: f64 = *var_vfbac2_t_slot;
        let mut var_vfbac2_t_db0: f64 = *var_vfbac2_t_db0_slot;
        let mut var_vfbac2_t_db1: f64 = *var_vfbac2_t_db1_slot;
        let mut var_vfbac2_t_db2: f64 = *var_vfbac2_t_db2_slot;
        let mut var_vfbac2_t_db3: f64 = *var_vfbac2_t_db3_slot;
        let mut var_vfbac2_t_dn0: f64 = *var_vfbac2_t_dn0_slot;
        let mut var_vfbac2_t_dn1: f64 = *var_vfbac2_t_dn1_slot;
        let mut var_vfbac2_t_dn2: f64 = *var_vfbac2_t_dn2_slot;
        let mut var_vfbac2_t_dn3: f64 = *var_vfbac2_t_dn3_slot;
        let mut var_vfbac2_t_dn4: f64 = *var_vfbac2_t_dn4_slot;
        let mut var_vfbac2_t_dn5: f64 = *var_vfbac2_t_dn5_slot;
        let mut var_vfbac2_t_dn6: f64 = *var_vfbac2_t_dn6_slot;
        let mut var_vfbac2_t_dn7: f64 = *var_vfbac2_t_dn7_slot;
        let mut var_vfbac2_t_dn8: f64 = *var_vfbac2_t_dn8_slot;
        let mut var_vfbac2_t_dn9: f64 = *var_vfbac2_t_dn9_slot;

        let (assign5850_e5270, assign5850_e5270_d_n0, assign5850_e5270_d_n1, assign5850_e5270_d_n2, assign5850_e5270_d_n3, assign5850_e5270_d_n4, assign5850_e5270_d_n5, assign5850_e5270_d_n6, assign5850_e5270_d_n7, assign5850_e5270_d_n8, assign5850_e5270_d_n9, assign5850_e5270_d_b0, assign5850_e5270_d_b1, assign5850_e5270_d_b2, assign5850_e5270_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5850_e5268: f64 = (var_thesatac_p).max(0.0);
        (assign5850_e5268, if var_thesatac_p >= 0.0 { var_thesatac_p_dn0 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn1 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn2 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn3 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn4 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn5 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn6 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn7 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn8 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_dn9 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db0 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db1 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db2 } else { 0.0 }, if var_thesatac_p >= 0.0 { var_thesatac_p_db3 } else { 0.0 },)
    } else {
        (var_thesatac_t, var_thesatac_t_dn0, var_thesatac_t_dn1, var_thesatac_t_dn2, var_thesatac_t_dn3, var_thesatac_t_dn4, var_thesatac_t_dn5, var_thesatac_t_dn6, var_thesatac_t_dn7, var_thesatac_t_dn8, var_thesatac_t_dn9, var_thesatac_t_db0, var_thesatac_t_db1, var_thesatac_t_db2, var_thesatac_t_db3,)
    }
};
        var_thesatac_t = assign5850_e5270;
        var_thesatac_t_dn0 = assign5850_e5270_d_n0;
        var_thesatac_t_dn1 = assign5850_e5270_d_n1;
        var_thesatac_t_dn2 = assign5850_e5270_d_n2;
        var_thesatac_t_dn3 = assign5850_e5270_d_n3;
        var_thesatac_t_dn4 = assign5850_e5270_d_n4;
        var_thesatac_t_dn5 = assign5850_e5270_d_n5;
        var_thesatac_t_dn6 = assign5850_e5270_d_n6;
        var_thesatac_t_dn7 = assign5850_e5270_d_n7;
        var_thesatac_t_dn8 = assign5850_e5270_d_n8;
        var_thesatac_t_dn9 = assign5850_e5270_d_n9;
        var_thesatac_t_db0 = assign5850_e5270_d_b0;
        var_thesatac_t_db1 = assign5850_e5270_d_b1;
        var_thesatac_t_db2 = assign5850_e5270_d_b2;
        var_thesatac_t_db3 = assign5850_e5270_d_b3;

        let (assign5860_e5283, assign5860_e5283_d_n0, assign5860_e5283_d_n1, assign5860_e5283_d_n2, assign5860_e5283_d_n3, assign5860_e5283_d_n4, assign5860_e5283_d_n5, assign5860_e5283_d_n6, assign5860_e5283_d_n7, assign5860_e5283_d_n8, assign5860_e5283_d_n9, assign5860_e5283_d_b0, assign5860_e5283_d_b1, assign5860_e5283_d_b2, assign5860_e5283_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5860_e5279: f64 = (p.p469 * var_temp0__blk79);
        let assign5860_e5281: f64 = (assign5860_e5279 / var_kstressvth0);
        (assign5860_e5281, ((p.p469 * var_temp0__blk79_dn0) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn1) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn2) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn3) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn4) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn5) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn6) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn7) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn8) / var_kstressvth0), ((p.p469 * var_temp0__blk79_dn9) / var_kstressvth0), ((p.p469 * var_temp0__blk79_db0) / var_kstressvth0), ((p.p469 * var_temp0__blk79_db1) / var_kstressvth0), ((p.p469 * var_temp0__blk79_db2) / var_kstressvth0), ((p.p469 * var_temp0__blk79_db3) / var_kstressvth0),)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5860_e5283;
        var_temp_dn0 = assign5860_e5283_d_n0;
        var_temp_dn1 = assign5860_e5283_d_n1;
        var_temp_dn2 = assign5860_e5283_d_n2;
        var_temp_dn3 = assign5860_e5283_d_n3;
        var_temp_dn4 = assign5860_e5283_d_n4;
        var_temp_dn5 = assign5860_e5283_d_n5;
        var_temp_dn6 = assign5860_e5283_d_n6;
        var_temp_dn7 = assign5860_e5283_d_n7;
        var_temp_dn8 = assign5860_e5283_d_n8;
        var_temp_dn9 = assign5860_e5283_d_n9;
        var_temp_db0 = assign5860_e5283_d_b0;
        var_temp_db1 = assign5860_e5283_d_b1;
        var_temp_db2 = assign5860_e5283_d_b2;
        var_temp_db3 = assign5860_e5283_d_b3;

        let (assign5870_e5294, assign5870_e5294_d_n0, assign5870_e5294_d_n1, assign5870_e5294_d_n2, assign5870_e5294_d_n3, assign5870_e5294_d_n4, assign5870_e5294_d_n5, assign5870_e5294_d_n6, assign5870_e5294_d_n7, assign5870_e5294_d_n8, assign5870_e5294_d_n9, assign5870_e5294_d_b0, assign5870_e5294_d_b1, assign5870_e5294_d_b2, assign5870_e5294_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5870_e5292: f64 = (var_vfb1_t + var_temp);
        (assign5870_e5292, (var_vfb1_t_dn0 + var_temp_dn0), (var_vfb1_t_dn1 + var_temp_dn1), (var_vfb1_t_dn2 + var_temp_dn2), (var_vfb1_t_dn3 + var_temp_dn3), (var_vfb1_t_dn4 + var_temp_dn4), (var_vfb1_t_dn5 + var_temp_dn5), (var_vfb1_t_dn6 + var_temp_dn6), (var_vfb1_t_dn7 + var_temp_dn7), (var_vfb1_t_dn8 + var_temp_dn8), (var_vfb1_t_dn9 + var_temp_dn9), (var_vfb1_t_db0 + var_temp_db0), (var_vfb1_t_db1 + var_temp_db1), (var_vfb1_t_db2 + var_temp_db2), (var_vfb1_t_db3 + var_temp_db3),)
    } else {
        (var_vfb1_t, var_vfb1_t_dn0, var_vfb1_t_dn1, var_vfb1_t_dn2, var_vfb1_t_dn3, var_vfb1_t_dn4, var_vfb1_t_dn5, var_vfb1_t_dn6, var_vfb1_t_dn7, var_vfb1_t_dn8, var_vfb1_t_dn9, var_vfb1_t_db0, var_vfb1_t_db1, var_vfb1_t_db2, var_vfb1_t_db3,)
    }
};
        var_vfb1_t = assign5870_e5294;
        var_vfb1_t_dn0 = assign5870_e5294_d_n0;
        var_vfb1_t_dn1 = assign5870_e5294_d_n1;
        var_vfb1_t_dn2 = assign5870_e5294_d_n2;
        var_vfb1_t_dn3 = assign5870_e5294_d_n3;
        var_vfb1_t_dn4 = assign5870_e5294_d_n4;
        var_vfb1_t_dn5 = assign5870_e5294_d_n5;
        var_vfb1_t_dn6 = assign5870_e5294_d_n6;
        var_vfb1_t_dn7 = assign5870_e5294_d_n7;
        var_vfb1_t_dn8 = assign5870_e5294_d_n8;
        var_vfb1_t_dn9 = assign5870_e5294_d_n9;
        var_vfb1_t_db0 = assign5870_e5294_d_b0;
        var_vfb1_t_db1 = assign5870_e5294_d_b1;
        var_vfb1_t_db2 = assign5870_e5294_d_b2;
        var_vfb1_t_db3 = assign5870_e5294_d_b3;

        let (assign5880_e5305, assign5880_e5305_d_n0, assign5880_e5305_d_n1, assign5880_e5305_d_n2, assign5880_e5305_d_n3, assign5880_e5305_d_n4, assign5880_e5305_d_n5, assign5880_e5305_d_n6, assign5880_e5305_d_n7, assign5880_e5305_d_n8, assign5880_e5305_d_n9, assign5880_e5305_d_b0, assign5880_e5305_d_b1, assign5880_e5305_d_b2, assign5880_e5305_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5880_e5303: f64 = (var_vfb2_t + var_temp);
        (assign5880_e5303, (var_vfb2_t_dn0 + var_temp_dn0), (var_vfb2_t_dn1 + var_temp_dn1), (var_vfb2_t_dn2 + var_temp_dn2), (var_vfb2_t_dn3 + var_temp_dn3), (var_vfb2_t_dn4 + var_temp_dn4), (var_vfb2_t_dn5 + var_temp_dn5), (var_vfb2_t_dn6 + var_temp_dn6), (var_vfb2_t_dn7 + var_temp_dn7), (var_vfb2_t_dn8 + var_temp_dn8), (var_vfb2_t_dn9 + var_temp_dn9), (var_vfb2_t_db0 + var_temp_db0), (var_vfb2_t_db1 + var_temp_db1), (var_vfb2_t_db2 + var_temp_db2), (var_vfb2_t_db3 + var_temp_db3),)
    } else {
        (var_vfb2_t, var_vfb2_t_dn0, var_vfb2_t_dn1, var_vfb2_t_dn2, var_vfb2_t_dn3, var_vfb2_t_dn4, var_vfb2_t_dn5, var_vfb2_t_dn6, var_vfb2_t_dn7, var_vfb2_t_dn8, var_vfb2_t_dn9, var_vfb2_t_db0, var_vfb2_t_db1, var_vfb2_t_db2, var_vfb2_t_db3,)
    }
};
        var_vfb2_t = assign5880_e5305;
        var_vfb2_t_dn0 = assign5880_e5305_d_n0;
        var_vfb2_t_dn1 = assign5880_e5305_d_n1;
        var_vfb2_t_dn2 = assign5880_e5305_d_n2;
        var_vfb2_t_dn3 = assign5880_e5305_d_n3;
        var_vfb2_t_dn4 = assign5880_e5305_d_n4;
        var_vfb2_t_dn5 = assign5880_e5305_d_n5;
        var_vfb2_t_dn6 = assign5880_e5305_d_n6;
        var_vfb2_t_dn7 = assign5880_e5305_d_n7;
        var_vfb2_t_dn8 = assign5880_e5305_d_n8;
        var_vfb2_t_dn9 = assign5880_e5305_d_n9;
        var_vfb2_t_db0 = assign5880_e5305_d_b0;
        var_vfb2_t_db1 = assign5880_e5305_d_b1;
        var_vfb2_t_db2 = assign5880_e5305_d_b2;
        var_vfb2_t_db3 = assign5880_e5305_d_b3;

        let (assign5890_e5316, assign5890_e5316_d_n0, assign5890_e5316_d_n1, assign5890_e5316_d_n2, assign5890_e5316_d_n3, assign5890_e5316_d_n4, assign5890_e5316_d_n5, assign5890_e5316_d_n6, assign5890_e5316_d_n7, assign5890_e5316_d_n8, assign5890_e5316_d_n9, assign5890_e5316_d_b0, assign5890_e5316_d_b1, assign5890_e5316_d_b2, assign5890_e5316_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5890_e5314: f64 = (var_vfbac1_t + var_temp);
        (assign5890_e5314, (var_vfbac1_t_dn0 + var_temp_dn0), (var_vfbac1_t_dn1 + var_temp_dn1), (var_vfbac1_t_dn2 + var_temp_dn2), (var_vfbac1_t_dn3 + var_temp_dn3), (var_vfbac1_t_dn4 + var_temp_dn4), (var_vfbac1_t_dn5 + var_temp_dn5), (var_vfbac1_t_dn6 + var_temp_dn6), (var_vfbac1_t_dn7 + var_temp_dn7), (var_vfbac1_t_dn8 + var_temp_dn8), (var_vfbac1_t_dn9 + var_temp_dn9), (var_vfbac1_t_db0 + var_temp_db0), (var_vfbac1_t_db1 + var_temp_db1), (var_vfbac1_t_db2 + var_temp_db2), (var_vfbac1_t_db3 + var_temp_db3),)
    } else {
        (var_vfbac1_t, var_vfbac1_t_dn0, var_vfbac1_t_dn1, var_vfbac1_t_dn2, var_vfbac1_t_dn3, var_vfbac1_t_dn4, var_vfbac1_t_dn5, var_vfbac1_t_dn6, var_vfbac1_t_dn7, var_vfbac1_t_dn8, var_vfbac1_t_dn9, var_vfbac1_t_db0, var_vfbac1_t_db1, var_vfbac1_t_db2, var_vfbac1_t_db3,)
    }
};
        var_vfbac1_t = assign5890_e5316;
        var_vfbac1_t_dn0 = assign5890_e5316_d_n0;
        var_vfbac1_t_dn1 = assign5890_e5316_d_n1;
        var_vfbac1_t_dn2 = assign5890_e5316_d_n2;
        var_vfbac1_t_dn3 = assign5890_e5316_d_n3;
        var_vfbac1_t_dn4 = assign5890_e5316_d_n4;
        var_vfbac1_t_dn5 = assign5890_e5316_d_n5;
        var_vfbac1_t_dn6 = assign5890_e5316_d_n6;
        var_vfbac1_t_dn7 = assign5890_e5316_d_n7;
        var_vfbac1_t_dn8 = assign5890_e5316_d_n8;
        var_vfbac1_t_dn9 = assign5890_e5316_d_n9;
        var_vfbac1_t_db0 = assign5890_e5316_d_b0;
        var_vfbac1_t_db1 = assign5890_e5316_d_b1;
        var_vfbac1_t_db2 = assign5890_e5316_d_b2;
        var_vfbac1_t_db3 = assign5890_e5316_d_b3;

        let (assign5900_e5327, assign5900_e5327_d_n0, assign5900_e5327_d_n1, assign5900_e5327_d_n2, assign5900_e5327_d_n3, assign5900_e5327_d_n4, assign5900_e5327_d_n5, assign5900_e5327_d_n6, assign5900_e5327_d_n7, assign5900_e5327_d_n8, assign5900_e5327_d_n9, assign5900_e5327_d_b0, assign5900_e5327_d_b1, assign5900_e5327_d_b2, assign5900_e5327_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5900_e5325: f64 = (var_vfbac2_t + var_temp);
        (assign5900_e5325, (var_vfbac2_t_dn0 + var_temp_dn0), (var_vfbac2_t_dn1 + var_temp_dn1), (var_vfbac2_t_dn2 + var_temp_dn2), (var_vfbac2_t_dn3 + var_temp_dn3), (var_vfbac2_t_dn4 + var_temp_dn4), (var_vfbac2_t_dn5 + var_temp_dn5), (var_vfbac2_t_dn6 + var_temp_dn6), (var_vfbac2_t_dn7 + var_temp_dn7), (var_vfbac2_t_dn8 + var_temp_dn8), (var_vfbac2_t_dn9 + var_temp_dn9), (var_vfbac2_t_db0 + var_temp_db0), (var_vfbac2_t_db1 + var_temp_db1), (var_vfbac2_t_db2 + var_temp_db2), (var_vfbac2_t_db3 + var_temp_db3),)
    } else {
        (var_vfbac2_t, var_vfbac2_t_dn0, var_vfbac2_t_dn1, var_vfbac2_t_dn2, var_vfbac2_t_dn3, var_vfbac2_t_dn4, var_vfbac2_t_dn5, var_vfbac2_t_dn6, var_vfbac2_t_dn7, var_vfbac2_t_dn8, var_vfbac2_t_dn9, var_vfbac2_t_db0, var_vfbac2_t_db1, var_vfbac2_t_db2, var_vfbac2_t_db3,)
    }
};
        var_vfbac2_t = assign5900_e5327;
        var_vfbac2_t_dn0 = assign5900_e5327_d_n0;
        var_vfbac2_t_dn1 = assign5900_e5327_d_n1;
        var_vfbac2_t_dn2 = assign5900_e5327_d_n2;
        var_vfbac2_t_dn3 = assign5900_e5327_d_n3;
        var_vfbac2_t_dn4 = assign5900_e5327_d_n4;
        var_vfbac2_t_dn5 = assign5900_e5327_d_n5;
        var_vfbac2_t_dn6 = assign5900_e5327_d_n6;
        var_vfbac2_t_dn7 = assign5900_e5327_d_n7;
        var_vfbac2_t_dn8 = assign5900_e5327_d_n8;
        var_vfbac2_t_dn9 = assign5900_e5327_d_n9;
        var_vfbac2_t_db0 = assign5900_e5327_d_b0;
        var_vfbac2_t_db1 = assign5900_e5327_d_b1;
        var_vfbac2_t_db2 = assign5900_e5327_d_b2;
        var_vfbac2_t_db3 = assign5900_e5327_d_b3;

        let (assign5910_e5342, assign5910_e5342_d_n0, assign5910_e5342_d_n1, assign5910_e5342_d_n2, assign5910_e5342_d_n3, assign5910_e5342_d_n4, assign5910_e5342_d_n5, assign5910_e5342_d_n6, assign5910_e5342_d_n7, assign5910_e5342_d_n8, assign5910_e5342_d_n9, assign5910_e5342_d_b0, assign5910_e5342_d_b1, assign5910_e5342_d_b2, assign5910_e5342_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5910_e5336: f64 = (p.p475 * var_temp0__blk79);
        let assign5910_e5339: f64 = (var_kstressvth0).powf(p.p476);
        let assign5910_e5340: f64 = (assign5910_e5336 / assign5910_e5339);
        (assign5910_e5340, ((p.p475 * var_temp0__blk79_dn0) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn1) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn2) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn3) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn4) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn5) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn6) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn7) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn8) / assign5910_e5339), ((p.p475 * var_temp0__blk79_dn9) / assign5910_e5339), ((p.p475 * var_temp0__blk79_db0) / assign5910_e5339), ((p.p475 * var_temp0__blk79_db1) / assign5910_e5339), ((p.p475 * var_temp0__blk79_db2) / assign5910_e5339), ((p.p475 * var_temp0__blk79_db3) / assign5910_e5339),)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5910_e5342;
        var_temp_dn0 = assign5910_e5342_d_n0;
        var_temp_dn1 = assign5910_e5342_d_n1;
        var_temp_dn2 = assign5910_e5342_d_n2;
        var_temp_dn3 = assign5910_e5342_d_n3;
        var_temp_dn4 = assign5910_e5342_d_n4;
        var_temp_dn5 = assign5910_e5342_d_n5;
        var_temp_dn6 = assign5910_e5342_d_n6;
        var_temp_dn7 = assign5910_e5342_d_n7;
        var_temp_dn8 = assign5910_e5342_d_n8;
        var_temp_dn9 = assign5910_e5342_d_n9;
        var_temp_db0 = assign5910_e5342_d_b0;
        var_temp_db1 = assign5910_e5342_d_b1;
        var_temp_db2 = assign5910_e5342_d_b2;
        var_temp_db3 = assign5910_e5342_d_b3;

        let (assign5920_e5353, assign5920_e5353_d_n0, assign5920_e5353_d_n1, assign5920_e5353_d_n2, assign5920_e5353_d_n3, assign5920_e5353_d_n4, assign5920_e5353_d_n5, assign5920_e5353_d_n6, assign5920_e5353_d_n7, assign5920_e5353_d_n8, assign5920_e5353_d_n9, assign5920_e5353_d_b0, assign5920_e5353_d_b1, assign5920_e5353_d_b2, assign5920_e5353_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5920_e5351: f64 = (var_cf_p + var_temp);
        (assign5920_e5351, (var_cf_p_dn0 + var_temp_dn0), (var_cf_p_dn1 + var_temp_dn1), (var_cf_p_dn2 + var_temp_dn2), (var_cf_p_dn3 + var_temp_dn3), (var_cf_p_dn4 + var_temp_dn4), (var_cf_p_dn5 + var_temp_dn5), (var_cf_p_dn6 + var_temp_dn6), (var_cf_p_dn7 + var_temp_dn7), (var_cf_p_dn8 + var_temp_dn8), (var_cf_p_dn9 + var_temp_dn9), (var_cf_p_db0 + var_temp_db0), (var_cf_p_db1 + var_temp_db1), (var_cf_p_db2 + var_temp_db2), (var_cf_p_db3 + var_temp_db3),)
    } else {
        (var_cf_p, var_cf_p_dn0, var_cf_p_dn1, var_cf_p_dn2, var_cf_p_dn3, var_cf_p_dn4, var_cf_p_dn5, var_cf_p_dn6, var_cf_p_dn7, var_cf_p_dn8, var_cf_p_dn9, var_cf_p_db0, var_cf_p_db1, var_cf_p_db2, var_cf_p_db3,)
    }
};
        var_cf_p = assign5920_e5353;
        var_cf_p_dn0 = assign5920_e5353_d_n0;
        var_cf_p_dn1 = assign5920_e5353_d_n1;
        var_cf_p_dn2 = assign5920_e5353_d_n2;
        var_cf_p_dn3 = assign5920_e5353_d_n3;
        var_cf_p_dn4 = assign5920_e5353_d_n4;
        var_cf_p_dn5 = assign5920_e5353_d_n5;
        var_cf_p_dn6 = assign5920_e5353_d_n6;
        var_cf_p_dn7 = assign5920_e5353_d_n7;
        var_cf_p_dn8 = assign5920_e5353_d_n8;
        var_cf_p_dn9 = assign5920_e5353_d_n9;
        var_cf_p_db0 = assign5920_e5353_d_b0;
        var_cf_p_db1 = assign5920_e5353_d_b1;
        var_cf_p_db2 = assign5920_e5353_d_b2;
        var_cf_p_db3 = assign5920_e5353_d_b3;

        let (assign5930_e5364, assign5930_e5364_d_n0, assign5930_e5364_d_n1, assign5930_e5364_d_n2, assign5930_e5364_d_n3, assign5930_e5364_d_n4, assign5930_e5364_d_n5, assign5930_e5364_d_n6, assign5930_e5364_d_n7, assign5930_e5364_d_n8, assign5930_e5364_d_n9, assign5930_e5364_d_b0, assign5930_e5364_d_b1, assign5930_e5364_d_b2, assign5930_e5364_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5930_e5362: f64 = (var_cf_p).max(0.0);
        (assign5930_e5362, if var_cf_p >= 0.0 { var_cf_p_dn0 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn1 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn2 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn3 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn4 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn5 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn6 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn7 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn8 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_dn9 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db0 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db1 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db2 } else { 0.0 }, if var_cf_p >= 0.0 { var_cf_p_db3 } else { 0.0 },)
    } else {
        (var_cf1_t, var_cf1_t_dn0, var_cf1_t_dn1, var_cf1_t_dn2, var_cf1_t_dn3, var_cf1_t_dn4, var_cf1_t_dn5, var_cf1_t_dn6, var_cf1_t_dn7, var_cf1_t_dn8, var_cf1_t_dn9, var_cf1_t_db0, var_cf1_t_db1, var_cf1_t_db2, var_cf1_t_db3,)
    }
};
        var_cf1_t = assign5930_e5364;
        var_cf1_t_dn0 = assign5930_e5364_d_n0;
        var_cf1_t_dn1 = assign5930_e5364_d_n1;
        var_cf1_t_dn2 = assign5930_e5364_d_n2;
        var_cf1_t_dn3 = assign5930_e5364_d_n3;
        var_cf1_t_dn4 = assign5930_e5364_d_n4;
        var_cf1_t_dn5 = assign5930_e5364_d_n5;
        var_cf1_t_dn6 = assign5930_e5364_d_n6;
        var_cf1_t_dn7 = assign5930_e5364_d_n7;
        var_cf1_t_dn8 = assign5930_e5364_d_n8;
        var_cf1_t_dn9 = assign5930_e5364_d_n9;
        var_cf1_t_db0 = assign5930_e5364_d_b0;
        var_cf1_t_db1 = assign5930_e5364_d_b1;
        var_cf1_t_db2 = assign5930_e5364_d_b2;
        var_cf1_t_db3 = assign5930_e5364_d_b3;

        let (assign5940_e5375, assign5940_e5375_d_n0, assign5940_e5375_d_n1, assign5940_e5375_d_n2, assign5940_e5375_d_n3, assign5940_e5375_d_n4, assign5940_e5375_d_n5, assign5940_e5375_d_n6, assign5940_e5375_d_n7, assign5940_e5375_d_n8, assign5940_e5375_d_n9, assign5940_e5375_d_b0, assign5940_e5375_d_b1, assign5940_e5375_d_b2, assign5940_e5375_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5940_e5373: f64 = (var_cfac_p + var_temp);
        (assign5940_e5373, (var_cfac_p_dn0 + var_temp_dn0), (var_cfac_p_dn1 + var_temp_dn1), (var_cfac_p_dn2 + var_temp_dn2), (var_cfac_p_dn3 + var_temp_dn3), (var_cfac_p_dn4 + var_temp_dn4), (var_cfac_p_dn5 + var_temp_dn5), (var_cfac_p_dn6 + var_temp_dn6), (var_cfac_p_dn7 + var_temp_dn7), (var_cfac_p_dn8 + var_temp_dn8), (var_cfac_p_dn9 + var_temp_dn9), (var_cfac_p_db0 + var_temp_db0), (var_cfac_p_db1 + var_temp_db1), (var_cfac_p_db2 + var_temp_db2), (var_cfac_p_db3 + var_temp_db3),)
    } else {
        (var_cfac_p, var_cfac_p_dn0, var_cfac_p_dn1, var_cfac_p_dn2, var_cfac_p_dn3, var_cfac_p_dn4, var_cfac_p_dn5, var_cfac_p_dn6, var_cfac_p_dn7, var_cfac_p_dn8, var_cfac_p_dn9, var_cfac_p_db0, var_cfac_p_db1, var_cfac_p_db2, var_cfac_p_db3,)
    }
};
        var_cfac_p = assign5940_e5375;
        var_cfac_p_dn0 = assign5940_e5375_d_n0;
        var_cfac_p_dn1 = assign5940_e5375_d_n1;
        var_cfac_p_dn2 = assign5940_e5375_d_n2;
        var_cfac_p_dn3 = assign5940_e5375_d_n3;
        var_cfac_p_dn4 = assign5940_e5375_d_n4;
        var_cfac_p_dn5 = assign5940_e5375_d_n5;
        var_cfac_p_dn6 = assign5940_e5375_d_n6;
        var_cfac_p_dn7 = assign5940_e5375_d_n7;
        var_cfac_p_dn8 = assign5940_e5375_d_n8;
        var_cfac_p_dn9 = assign5940_e5375_d_n9;
        var_cfac_p_db0 = assign5940_e5375_d_b0;
        var_cfac_p_db1 = assign5940_e5375_d_b1;
        var_cfac_p_db2 = assign5940_e5375_d_b2;
        var_cfac_p_db3 = assign5940_e5375_d_b3;

        let (assign5950_e5386, assign5950_e5386_d_n0, assign5950_e5386_d_n1, assign5950_e5386_d_n2, assign5950_e5386_d_n3, assign5950_e5386_d_n4, assign5950_e5386_d_n5, assign5950_e5386_d_n6, assign5950_e5386_d_n7, assign5950_e5386_d_n8, assign5950_e5386_d_n9, assign5950_e5386_d_b0, assign5950_e5386_d_b1, assign5950_e5386_d_b2, assign5950_e5386_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5950_e5384: f64 = (var_cfac_p).max(0.0);
        (assign5950_e5384, if var_cfac_p >= 0.0 { var_cfac_p_dn0 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn1 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn2 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn3 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn4 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn5 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn6 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn7 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn8 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_dn9 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db0 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db1 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db2 } else { 0.0 }, if var_cfac_p >= 0.0 { var_cfac_p_db3 } else { 0.0 },)
    } else {
        (var_cfac1_t, var_cfac1_t_dn0, var_cfac1_t_dn1, var_cfac1_t_dn2, var_cfac1_t_dn3, var_cfac1_t_dn4, var_cfac1_t_dn5, var_cfac1_t_dn6, var_cfac1_t_dn7, var_cfac1_t_dn8, var_cfac1_t_dn9, var_cfac1_t_db0, var_cfac1_t_db1, var_cfac1_t_db2, var_cfac1_t_db3,)
    }
};
        var_cfac1_t = assign5950_e5386;
        var_cfac1_t_dn0 = assign5950_e5386_d_n0;
        var_cfac1_t_dn1 = assign5950_e5386_d_n1;
        var_cfac1_t_dn2 = assign5950_e5386_d_n2;
        var_cfac1_t_dn3 = assign5950_e5386_d_n3;
        var_cfac1_t_dn4 = assign5950_e5386_d_n4;
        var_cfac1_t_dn5 = assign5950_e5386_d_n5;
        var_cfac1_t_dn6 = assign5950_e5386_d_n6;
        var_cfac1_t_dn7 = assign5950_e5386_d_n7;
        var_cfac1_t_dn8 = assign5950_e5386_d_n8;
        var_cfac1_t_dn9 = assign5950_e5386_d_n9;
        var_cfac1_t_db0 = assign5950_e5386_d_b0;
        var_cfac1_t_db1 = assign5950_e5386_d_b1;
        var_cfac1_t_db2 = assign5950_e5386_d_b2;
        var_cfac1_t_db3 = assign5950_e5386_d_b3;

        let (assign5960_e5399, assign5960_e5399_d_n0, assign5960_e5399_d_n1, assign5960_e5399_d_n2, assign5960_e5399_d_n3, assign5960_e5399_d_n4, assign5960_e5399_d_n5, assign5960_e5399_d_n6, assign5960_e5399_d_n7, assign5960_e5399_d_n8, assign5960_e5399_d_n9, assign5960_e5399_d_b0, assign5960_e5399_d_b1, assign5960_e5399_d_b2, assign5960_e5399_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5960_e5395: f64 = (p.p234 * var_tox2_i);
        let assign5960_e5397: f64 = (assign5960_e5395 / var_tox1_i);
        (assign5960_e5397, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign5960_e5399;
        var_temp_dn0 = assign5960_e5399_d_n0;
        var_temp_dn1 = assign5960_e5399_d_n1;
        var_temp_dn2 = assign5960_e5399_d_n2;
        var_temp_dn3 = assign5960_e5399_d_n3;
        var_temp_dn4 = assign5960_e5399_d_n4;
        var_temp_dn5 = assign5960_e5399_d_n5;
        var_temp_dn6 = assign5960_e5399_d_n6;
        var_temp_dn7 = assign5960_e5399_d_n7;
        var_temp_dn8 = assign5960_e5399_d_n8;
        var_temp_dn9 = assign5960_e5399_d_n9;
        var_temp_db0 = assign5960_e5399_d_b0;
        var_temp_db1 = assign5960_e5399_d_b1;
        var_temp_db2 = assign5960_e5399_d_b2;
        var_temp_db3 = assign5960_e5399_d_b3;

        let (assign5970_e5410, assign5970_e5410_d_n0, assign5970_e5410_d_n1, assign5970_e5410_d_n2, assign5970_e5410_d_n3, assign5970_e5410_d_n4, assign5970_e5410_d_n5, assign5970_e5410_d_n6, assign5970_e5410_d_n7, assign5970_e5410_d_n8, assign5970_e5410_d_n9, assign5970_e5410_d_b0, assign5970_e5410_d_b1, assign5970_e5410_d_b2, assign5970_e5410_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5970_e5408: f64 = (var_cf1_t * var_temp);
        (assign5970_e5408, ((var_cf1_t_dn0 * var_temp) + (var_cf1_t * var_temp_dn0)), ((var_cf1_t_dn1 * var_temp) + (var_cf1_t * var_temp_dn1)), ((var_cf1_t_dn2 * var_temp) + (var_cf1_t * var_temp_dn2)), ((var_cf1_t_dn3 * var_temp) + (var_cf1_t * var_temp_dn3)), ((var_cf1_t_dn4 * var_temp) + (var_cf1_t * var_temp_dn4)), ((var_cf1_t_dn5 * var_temp) + (var_cf1_t * var_temp_dn5)), ((var_cf1_t_dn6 * var_temp) + (var_cf1_t * var_temp_dn6)), ((var_cf1_t_dn7 * var_temp) + (var_cf1_t * var_temp_dn7)), ((var_cf1_t_dn8 * var_temp) + (var_cf1_t * var_temp_dn8)), ((var_cf1_t_dn9 * var_temp) + (var_cf1_t * var_temp_dn9)), ((var_cf1_t_db0 * var_temp) + (var_cf1_t * var_temp_db0)), ((var_cf1_t_db1 * var_temp) + (var_cf1_t * var_temp_db1)), ((var_cf1_t_db2 * var_temp) + (var_cf1_t * var_temp_db2)), ((var_cf1_t_db3 * var_temp) + (var_cf1_t * var_temp_db3)),)
    } else {
        (var_cf2_t, var_cf2_t_dn0, var_cf2_t_dn1, var_cf2_t_dn2, var_cf2_t_dn3, var_cf2_t_dn4, var_cf2_t_dn5, var_cf2_t_dn6, var_cf2_t_dn7, var_cf2_t_dn8, var_cf2_t_dn9, var_cf2_t_db0, var_cf2_t_db1, var_cf2_t_db2, var_cf2_t_db3,)
    }
};
        var_cf2_t = assign5970_e5410;
        var_cf2_t_dn0 = assign5970_e5410_d_n0;
        var_cf2_t_dn1 = assign5970_e5410_d_n1;
        var_cf2_t_dn2 = assign5970_e5410_d_n2;
        var_cf2_t_dn3 = assign5970_e5410_d_n3;
        var_cf2_t_dn4 = assign5970_e5410_d_n4;
        var_cf2_t_dn5 = assign5970_e5410_d_n5;
        var_cf2_t_dn6 = assign5970_e5410_d_n6;
        var_cf2_t_dn7 = assign5970_e5410_d_n7;
        var_cf2_t_dn8 = assign5970_e5410_d_n8;
        var_cf2_t_dn9 = assign5970_e5410_d_n9;
        var_cf2_t_db0 = assign5970_e5410_d_b0;
        var_cf2_t_db1 = assign5970_e5410_d_b1;
        var_cf2_t_db2 = assign5970_e5410_d_b2;
        var_cf2_t_db3 = assign5970_e5410_d_b3;

        let (assign5980_e5421, assign5980_e5421_d_n0, assign5980_e5421_d_n1, assign5980_e5421_d_n2, assign5980_e5421_d_n3, assign5980_e5421_d_n4, assign5980_e5421_d_n5, assign5980_e5421_d_n6, assign5980_e5421_d_n7, assign5980_e5421_d_n8, assign5980_e5421_d_n9, assign5980_e5421_d_b0, assign5980_e5421_d_b1, assign5980_e5421_d_b2, assign5980_e5421_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 != 0.0)) {
        let assign5980_e5419: f64 = (var_cfac1_t * var_temp);
        (assign5980_e5419, ((var_cfac1_t_dn0 * var_temp) + (var_cfac1_t * var_temp_dn0)), ((var_cfac1_t_dn1 * var_temp) + (var_cfac1_t * var_temp_dn1)), ((var_cfac1_t_dn2 * var_temp) + (var_cfac1_t * var_temp_dn2)), ((var_cfac1_t_dn3 * var_temp) + (var_cfac1_t * var_temp_dn3)), ((var_cfac1_t_dn4 * var_temp) + (var_cfac1_t * var_temp_dn4)), ((var_cfac1_t_dn5 * var_temp) + (var_cfac1_t * var_temp_dn5)), ((var_cfac1_t_dn6 * var_temp) + (var_cfac1_t * var_temp_dn6)), ((var_cfac1_t_dn7 * var_temp) + (var_cfac1_t * var_temp_dn7)), ((var_cfac1_t_dn8 * var_temp) + (var_cfac1_t * var_temp_dn8)), ((var_cfac1_t_dn9 * var_temp) + (var_cfac1_t * var_temp_dn9)), ((var_cfac1_t_db0 * var_temp) + (var_cfac1_t * var_temp_db0)), ((var_cfac1_t_db1 * var_temp) + (var_cfac1_t * var_temp_db1)), ((var_cfac1_t_db2 * var_temp) + (var_cfac1_t * var_temp_db2)), ((var_cfac1_t_db3 * var_temp) + (var_cfac1_t * var_temp_db3)),)
    } else {
        (var_cfac2_t, var_cfac2_t_dn0, var_cfac2_t_dn1, var_cfac2_t_dn2, var_cfac2_t_dn3, var_cfac2_t_dn4, var_cfac2_t_dn5, var_cfac2_t_dn6, var_cfac2_t_dn7, var_cfac2_t_dn8, var_cfac2_t_dn9, var_cfac2_t_db0, var_cfac2_t_db1, var_cfac2_t_db2, var_cfac2_t_db3,)
    }
};
        var_cfac2_t = assign5980_e5421;
        var_cfac2_t_dn0 = assign5980_e5421_d_n0;
        var_cfac2_t_dn1 = assign5980_e5421_d_n1;
        var_cfac2_t_dn2 = assign5980_e5421_d_n2;
        var_cfac2_t_dn3 = assign5980_e5421_d_n3;
        var_cfac2_t_dn4 = assign5980_e5421_d_n4;
        var_cfac2_t_dn5 = assign5980_e5421_d_n5;
        var_cfac2_t_dn6 = assign5980_e5421_d_n6;
        var_cfac2_t_dn7 = assign5980_e5421_d_n7;
        var_cfac2_t_dn8 = assign5980_e5421_d_n8;
        var_cfac2_t_dn9 = assign5980_e5421_d_n9;
        var_cfac2_t_db0 = assign5980_e5421_d_b0;
        var_cfac2_t_db1 = assign5980_e5421_d_b1;
        var_cfac2_t_db2 = assign5980_e5421_d_b2;
        var_cfac2_t_db3 = assign5980_e5421_d_b3;

        let (assign5990_e5431, assign5990_e5431_d_n0, assign5990_e5431_d_n1, assign5990_e5431_d_n2, assign5990_e5431_d_n3, assign5990_e5431_d_n4, assign5990_e5431_d_n5, assign5990_e5431_d_n6, assign5990_e5431_d_n7, assign5990_e5431_d_n8, assign5990_e5431_d_n9, assign5990_e5431_d_b0, assign5990_e5431_d_b1, assign5990_e5431_d_b2, assign5990_e5431_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmpa, var_tmpa_dn0, var_tmpa_dn1, var_tmpa_dn2, var_tmpa_dn3, var_tmpa_dn4, var_tmpa_dn5, var_tmpa_dn6, var_tmpa_dn7, var_tmpa_dn8, var_tmpa_dn9, var_tmpa_db0, var_tmpa_db1, var_tmpa_db2, var_tmpa_db3,)
    }
};
        var_tmpa = assign5990_e5431;
        var_tmpa_dn0 = assign5990_e5431_d_n0;
        var_tmpa_dn1 = assign5990_e5431_d_n1;
        var_tmpa_dn2 = assign5990_e5431_d_n2;
        var_tmpa_dn3 = assign5990_e5431_d_n3;
        var_tmpa_dn4 = assign5990_e5431_d_n4;
        var_tmpa_dn5 = assign5990_e5431_d_n5;
        var_tmpa_dn6 = assign5990_e5431_d_n6;
        var_tmpa_dn7 = assign5990_e5431_d_n7;
        var_tmpa_dn8 = assign5990_e5431_d_n8;
        var_tmpa_dn9 = assign5990_e5431_d_n9;
        var_tmpa_db0 = assign5990_e5431_d_b0;
        var_tmpa_db1 = assign5990_e5431_d_b1;
        var_tmpa_db2 = assign5990_e5431_d_b2;
        var_tmpa_db3 = assign5990_e5431_d_b3;

        let (assign6000_e5441,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        (0.0,)
    } else {
        (var_iloop,)
    }
};
        var_iloop = assign6000_e5441;

        let (assign6010_e5454, assign6010_e5454_d_n0, assign6010_e5454_d_n1, assign6010_e5454_d_n2, assign6010_e5454_d_n3, assign6010_e5454_d_n4, assign6010_e5454_d_n5, assign6010_e5454_d_n6, assign6010_e5454_d_n7, assign6010_e5454_d_n8, assign6010_e5454_d_n9, assign6010_e5454_d_b0, assign6010_e5454_d_b1, assign6010_e5454_d_b2, assign6010_e5454_d_b3,) = {
    if (((var_guard83 == 0.0) && (var_guard133 != 0.0)) && (var_guard134 == 0.0)) {
        let assign6010_e5450: f64 = (-1.0);
        let assign6010_e5452: f64 = (assign6010_e5450 / p.p478);
        (assign6010_e5452, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp, var_temp_dn0, var_temp_dn1, var_temp_dn2, var_temp_dn3, var_temp_dn4, var_temp_dn5, var_temp_dn6, var_temp_dn7, var_temp_dn8, var_temp_dn9, var_temp_db0, var_temp_db1, var_temp_db2, var_temp_db3,)
    }
};
        var_temp = assign6010_e5454;
        var_temp_dn0 = assign6010_e5454_d_n0;
        var_temp_dn1 = assign6010_e5454_d_n1;
        var_temp_dn2 = assign6010_e5454_d_n2;
        var_temp_dn3 = assign6010_e5454_d_n3;
        var_temp_dn4 = assign6010_e5454_d_n4;
        var_temp_dn5 = assign6010_e5454_d_n5;
        var_temp_dn6 = assign6010_e5454_d_n6;
        var_temp_dn7 = assign6010_e5454_d_n7;
        var_temp_dn8 = assign6010_e5454_d_n8;
        var_temp_dn9 = assign6010_e5454_d_n9;
        var_temp_db0 = assign6010_e5454_d_b0;
        var_temp_db1 = assign6010_e5454_d_b1;
        var_temp_db2 = assign6010_e5454_d_b2;
        var_temp_db3 = assign6010_e5454_d_b3;

        *var_cf1_t_slot = var_cf1_t;
        *var_cf1_t_db0_slot = var_cf1_t_db0;
        *var_cf1_t_db1_slot = var_cf1_t_db1;
        *var_cf1_t_db2_slot = var_cf1_t_db2;
        *var_cf1_t_db3_slot = var_cf1_t_db3;
        *var_cf1_t_dn0_slot = var_cf1_t_dn0;
        *var_cf1_t_dn1_slot = var_cf1_t_dn1;
        *var_cf1_t_dn2_slot = var_cf1_t_dn2;
        *var_cf1_t_dn3_slot = var_cf1_t_dn3;
        *var_cf1_t_dn4_slot = var_cf1_t_dn4;
        *var_cf1_t_dn5_slot = var_cf1_t_dn5;
        *var_cf1_t_dn6_slot = var_cf1_t_dn6;
        *var_cf1_t_dn7_slot = var_cf1_t_dn7;
        *var_cf1_t_dn8_slot = var_cf1_t_dn8;
        *var_cf1_t_dn9_slot = var_cf1_t_dn9;
        *var_cf2_t_slot = var_cf2_t;
        *var_cf2_t_db0_slot = var_cf2_t_db0;
        *var_cf2_t_db1_slot = var_cf2_t_db1;
        *var_cf2_t_db2_slot = var_cf2_t_db2;
        *var_cf2_t_db3_slot = var_cf2_t_db3;
        *var_cf2_t_dn0_slot = var_cf2_t_dn0;
        *var_cf2_t_dn1_slot = var_cf2_t_dn1;
        *var_cf2_t_dn2_slot = var_cf2_t_dn2;
        *var_cf2_t_dn3_slot = var_cf2_t_dn3;
        *var_cf2_t_dn4_slot = var_cf2_t_dn4;
        *var_cf2_t_dn5_slot = var_cf2_t_dn5;
        *var_cf2_t_dn6_slot = var_cf2_t_dn6;
        *var_cf2_t_dn7_slot = var_cf2_t_dn7;
        *var_cf2_t_dn8_slot = var_cf2_t_dn8;
        *var_cf2_t_dn9_slot = var_cf2_t_dn9;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_db0_slot = var_cf_p_db0;
        *var_cf_p_db1_slot = var_cf_p_db1;
        *var_cf_p_db2_slot = var_cf_p_db2;
        *var_cf_p_db3_slot = var_cf_p_db3;
        *var_cf_p_dn0_slot = var_cf_p_dn0;
        *var_cf_p_dn1_slot = var_cf_p_dn1;
        *var_cf_p_dn2_slot = var_cf_p_dn2;
        *var_cf_p_dn3_slot = var_cf_p_dn3;
        *var_cf_p_dn4_slot = var_cf_p_dn4;
        *var_cf_p_dn5_slot = var_cf_p_dn5;
        *var_cf_p_dn6_slot = var_cf_p_dn6;
        *var_cf_p_dn7_slot = var_cf_p_dn7;
        *var_cf_p_dn8_slot = var_cf_p_dn8;
        *var_cf_p_dn9_slot = var_cf_p_dn9;
        *var_cfac1_t_slot = var_cfac1_t;
        *var_cfac1_t_db0_slot = var_cfac1_t_db0;
        *var_cfac1_t_db1_slot = var_cfac1_t_db1;
        *var_cfac1_t_db2_slot = var_cfac1_t_db2;
        *var_cfac1_t_db3_slot = var_cfac1_t_db3;
        *var_cfac1_t_dn0_slot = var_cfac1_t_dn0;
        *var_cfac1_t_dn1_slot = var_cfac1_t_dn1;
        *var_cfac1_t_dn2_slot = var_cfac1_t_dn2;
        *var_cfac1_t_dn3_slot = var_cfac1_t_dn3;
        *var_cfac1_t_dn4_slot = var_cfac1_t_dn4;
        *var_cfac1_t_dn5_slot = var_cfac1_t_dn5;
        *var_cfac1_t_dn6_slot = var_cfac1_t_dn6;
        *var_cfac1_t_dn7_slot = var_cfac1_t_dn7;
        *var_cfac1_t_dn8_slot = var_cfac1_t_dn8;
        *var_cfac1_t_dn9_slot = var_cfac1_t_dn9;
        *var_cfac2_t_slot = var_cfac2_t;
        *var_cfac2_t_db0_slot = var_cfac2_t_db0;
        *var_cfac2_t_db1_slot = var_cfac2_t_db1;
        *var_cfac2_t_db2_slot = var_cfac2_t_db2;
        *var_cfac2_t_db3_slot = var_cfac2_t_db3;
        *var_cfac2_t_dn0_slot = var_cfac2_t_dn0;
        *var_cfac2_t_dn1_slot = var_cfac2_t_dn1;
        *var_cfac2_t_dn2_slot = var_cfac2_t_dn2;
        *var_cfac2_t_dn3_slot = var_cfac2_t_dn3;
        *var_cfac2_t_dn4_slot = var_cfac2_t_dn4;
        *var_cfac2_t_dn5_slot = var_cfac2_t_dn5;
        *var_cfac2_t_dn6_slot = var_cfac2_t_dn6;
        *var_cfac2_t_dn7_slot = var_cfac2_t_dn7;
        *var_cfac2_t_dn8_slot = var_cfac2_t_dn8;
        *var_cfac2_t_dn9_slot = var_cfac2_t_dn9;
        *var_cfac_p_slot = var_cfac_p;
        *var_cfac_p_db0_slot = var_cfac_p_db0;
        *var_cfac_p_db1_slot = var_cfac_p_db1;
        *var_cfac_p_db2_slot = var_cfac_p_db2;
        *var_cfac_p_db3_slot = var_cfac_p_db3;
        *var_cfac_p_dn0_slot = var_cfac_p_dn0;
        *var_cfac_p_dn1_slot = var_cfac_p_dn1;
        *var_cfac_p_dn2_slot = var_cfac_p_dn2;
        *var_cfac_p_dn3_slot = var_cfac_p_dn3;
        *var_cfac_p_dn4_slot = var_cfac_p_dn4;
        *var_cfac_p_dn5_slot = var_cfac_p_dn5;
        *var_cfac_p_dn6_slot = var_cfac_p_dn6;
        *var_cfac_p_dn7_slot = var_cfac_p_dn7;
        *var_cfac_p_dn8_slot = var_cfac_p_dn8;
        *var_cfac_p_dn9_slot = var_cfac_p_dn9;
        *var_iloop_slot = var_iloop;
        *var_temp_slot = var_temp;
        *var_temp_db0_slot = var_temp_db0;
        *var_temp_db1_slot = var_temp_db1;
        *var_temp_db2_slot = var_temp_db2;
        *var_temp_db3_slot = var_temp_db3;
        *var_temp_dn0_slot = var_temp_dn0;
        *var_temp_dn1_slot = var_temp_dn1;
        *var_temp_dn2_slot = var_temp_dn2;
        *var_temp_dn3_slot = var_temp_dn3;
        *var_temp_dn4_slot = var_temp_dn4;
        *var_temp_dn5_slot = var_temp_dn5;
        *var_temp_dn6_slot = var_temp_dn6;
        *var_temp_dn7_slot = var_temp_dn7;
        *var_temp_dn8_slot = var_temp_dn8;
        *var_temp_dn9_slot = var_temp_dn9;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_db0_slot = var_thesatac_t_db0;
        *var_thesatac_t_db1_slot = var_thesatac_t_db1;
        *var_thesatac_t_db2_slot = var_thesatac_t_db2;
        *var_thesatac_t_db3_slot = var_thesatac_t_db3;
        *var_thesatac_t_dn0_slot = var_thesatac_t_dn0;
        *var_thesatac_t_dn1_slot = var_thesatac_t_dn1;
        *var_thesatac_t_dn2_slot = var_thesatac_t_dn2;
        *var_thesatac_t_dn3_slot = var_thesatac_t_dn3;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_dn5_slot = var_thesatac_t_dn5;
        *var_thesatac_t_dn6_slot = var_thesatac_t_dn6;
        *var_thesatac_t_dn7_slot = var_thesatac_t_dn7;
        *var_thesatac_t_dn8_slot = var_thesatac_t_dn8;
        *var_thesatac_t_dn9_slot = var_thesatac_t_dn9;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_db0_slot = var_tmpa_db0;
        *var_tmpa_db1_slot = var_tmpa_db1;
        *var_tmpa_db2_slot = var_tmpa_db2;
        *var_tmpa_db3_slot = var_tmpa_db3;
        *var_tmpa_dn0_slot = var_tmpa_dn0;
        *var_tmpa_dn1_slot = var_tmpa_dn1;
        *var_tmpa_dn2_slot = var_tmpa_dn2;
        *var_tmpa_dn3_slot = var_tmpa_dn3;
        *var_tmpa_dn4_slot = var_tmpa_dn4;
        *var_tmpa_dn5_slot = var_tmpa_dn5;
        *var_tmpa_dn6_slot = var_tmpa_dn6;
        *var_tmpa_dn7_slot = var_tmpa_dn7;
        *var_tmpa_dn8_slot = var_tmpa_dn8;
        *var_tmpa_dn9_slot = var_tmpa_dn9;
        *var_vfb1_t_slot = var_vfb1_t;
        *var_vfb1_t_db0_slot = var_vfb1_t_db0;
        *var_vfb1_t_db1_slot = var_vfb1_t_db1;
        *var_vfb1_t_db2_slot = var_vfb1_t_db2;
        *var_vfb1_t_db3_slot = var_vfb1_t_db3;
        *var_vfb1_t_dn0_slot = var_vfb1_t_dn0;
        *var_vfb1_t_dn1_slot = var_vfb1_t_dn1;
        *var_vfb1_t_dn2_slot = var_vfb1_t_dn2;
        *var_vfb1_t_dn3_slot = var_vfb1_t_dn3;
        *var_vfb1_t_dn4_slot = var_vfb1_t_dn4;
        *var_vfb1_t_dn5_slot = var_vfb1_t_dn5;
        *var_vfb1_t_dn6_slot = var_vfb1_t_dn6;
        *var_vfb1_t_dn7_slot = var_vfb1_t_dn7;
        *var_vfb1_t_dn8_slot = var_vfb1_t_dn8;
        *var_vfb1_t_dn9_slot = var_vfb1_t_dn9;
        *var_vfb2_t_slot = var_vfb2_t;
        *var_vfb2_t_db0_slot = var_vfb2_t_db0;
        *var_vfb2_t_db1_slot = var_vfb2_t_db1;
        *var_vfb2_t_db2_slot = var_vfb2_t_db2;
        *var_vfb2_t_db3_slot = var_vfb2_t_db3;
        *var_vfb2_t_dn0_slot = var_vfb2_t_dn0;
        *var_vfb2_t_dn1_slot = var_vfb2_t_dn1;
        *var_vfb2_t_dn2_slot = var_vfb2_t_dn2;
        *var_vfb2_t_dn3_slot = var_vfb2_t_dn3;
        *var_vfb2_t_dn4_slot = var_vfb2_t_dn4;
        *var_vfb2_t_dn5_slot = var_vfb2_t_dn5;
        *var_vfb2_t_dn6_slot = var_vfb2_t_dn6;
        *var_vfb2_t_dn7_slot = var_vfb2_t_dn7;
        *var_vfb2_t_dn8_slot = var_vfb2_t_dn8;
        *var_vfb2_t_dn9_slot = var_vfb2_t_dn9;
        *var_vfbac1_t_slot = var_vfbac1_t;
        *var_vfbac1_t_db0_slot = var_vfbac1_t_db0;
        *var_vfbac1_t_db1_slot = var_vfbac1_t_db1;
        *var_vfbac1_t_db2_slot = var_vfbac1_t_db2;
        *var_vfbac1_t_db3_slot = var_vfbac1_t_db3;
        *var_vfbac1_t_dn0_slot = var_vfbac1_t_dn0;
        *var_vfbac1_t_dn1_slot = var_vfbac1_t_dn1;
        *var_vfbac1_t_dn2_slot = var_vfbac1_t_dn2;
        *var_vfbac1_t_dn3_slot = var_vfbac1_t_dn3;
        *var_vfbac1_t_dn4_slot = var_vfbac1_t_dn4;
        *var_vfbac1_t_dn5_slot = var_vfbac1_t_dn5;
        *var_vfbac1_t_dn6_slot = var_vfbac1_t_dn6;
        *var_vfbac1_t_dn7_slot = var_vfbac1_t_dn7;
        *var_vfbac1_t_dn8_slot = var_vfbac1_t_dn8;
        *var_vfbac1_t_dn9_slot = var_vfbac1_t_dn9;
        *var_vfbac2_t_slot = var_vfbac2_t;
        *var_vfbac2_t_db0_slot = var_vfbac2_t_db0;
        *var_vfbac2_t_db1_slot = var_vfbac2_t_db1;
        *var_vfbac2_t_db2_slot = var_vfbac2_t_db2;
        *var_vfbac2_t_db3_slot = var_vfbac2_t_db3;
        *var_vfbac2_t_dn0_slot = var_vfbac2_t_dn0;
        *var_vfbac2_t_dn1_slot = var_vfbac2_t_dn1;
        *var_vfbac2_t_dn2_slot = var_vfbac2_t_dn2;
        *var_vfbac2_t_dn3_slot = var_vfbac2_t_dn3;
        *var_vfbac2_t_dn4_slot = var_vfbac2_t_dn4;
        *var_vfbac2_t_dn5_slot = var_vfbac2_t_dn5;
        *var_vfbac2_t_dn6_slot = var_vfbac2_t_dn6;
        *var_vfbac2_t_dn7_slot = var_vfbac2_t_dn7;
        *var_vfbac2_t_dn8_slot = var_vfbac2_t_dn8;
        *var_vfbac2_t_dn9_slot = var_vfbac2_t_dn9;
    }
}
