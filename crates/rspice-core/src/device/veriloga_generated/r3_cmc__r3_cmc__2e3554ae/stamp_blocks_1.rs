#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        var_dt: f64,
        var_dt_db0: f64,
        var_dt_db1: f64,
        var_dt_dn0: f64,
        var_dt_dn1: f64,
        var_dt_dn2: f64,
        var_dt_dn3: f64,
        var_dt_dn4: f64,
        var_dt_dn5: f64,
        var_l_um: f64,
        var_l_um_db0: f64,
        var_l_um_db1: f64,
        var_l_um_dn0: f64,
        var_l_um_dn1: f64,
        var_l_um_dn2: f64,
        var_l_um_dn3: f64,
        var_l_um_dn4: f64,
        var_l_um_dn5: f64,
        var_leff_um: f64,
        var_leff_um_db0: f64,
        var_leff_um_db1: f64,
        var_leff_um_dn0: f64,
        var_leff_um_dn1: f64,
        var_leff_um_dn2: f64,
        var_leff_um_dn3: f64,
        var_leff_um_dn4: f64,
        var_leff_um_dn5: f64,
        var_w_um: f64,
        var_w_um_db0: f64,
        var_w_um_db1: f64,
        var_w_um_dn0: f64,
        var_w_um_dn1: f64,
        var_w_um_dn2: f64,
        var_w_um_dn3: f64,
        var_w_um_dn4: f64,
        var_w_um_dn5: f64,
        var_df_slot: &mut f64,
        var_df_db0_slot: &mut f64,
        var_df_db1_slot: &mut f64,
        var_df_dn0_slot: &mut f64,
        var_df_dn1_slot: &mut f64,
        var_df_dn2_slot: &mut f64,
        var_df_dn3_slot: &mut f64,
        var_df_dn4_slot: &mut f64,
        var_df_dn5_slot: &mut f64,
        var_df_rdb0_slot: &mut f64,
        var_df_rdb1_slot: &mut f64,
        var_df_rdn0_slot: &mut f64,
        var_df_rdn1_slot: &mut f64,
        var_df_rdn2_slot: &mut f64,
        var_df_rdn3_slot: &mut f64,
        var_df_rdn4_slot: &mut f64,
        var_df_rdn5_slot: &mut f64,
        var_df_rv_slot: &mut f64,
        var_dfmin_slot: &mut f64,
        var_dfmin_db0_slot: &mut f64,
        var_dfmin_db1_slot: &mut f64,
        var_dfmin_dn0_slot: &mut f64,
        var_dfmin_dn1_slot: &mut f64,
        var_dfmin_dn2_slot: &mut f64,
        var_dfmin_dn3_slot: &mut f64,
        var_dfmin_dn4_slot: &mut f64,
        var_dfmin_dn5_slot: &mut f64,
        var_dfmin_rdb0_slot: &mut f64,
        var_dfmin_rdb1_slot: &mut f64,
        var_dfmin_rdn0_slot: &mut f64,
        var_dfmin_rdn1_slot: &mut f64,
        var_dfmin_rdn2_slot: &mut f64,
        var_dfmin_rdn3_slot: &mut f64,
        var_dfmin_rdn4_slot: &mut f64,
        var_dfmin_rdn5_slot: &mut f64,
        var_dfmin_rv_slot: &mut f64,
        var_dfsq_slot: &mut f64,
        var_dfsq_db0_slot: &mut f64,
        var_dfsq_db1_slot: &mut f64,
        var_dfsq_dn0_slot: &mut f64,
        var_dfsq_dn1_slot: &mut f64,
        var_dfsq_dn2_slot: &mut f64,
        var_dfsq_dn3_slot: &mut f64,
        var_dfsq_dn4_slot: &mut f64,
        var_dfsq_dn5_slot: &mut f64,
        var_dfsq_rdb0_slot: &mut f64,
        var_dfsq_rdb1_slot: &mut f64,
        var_dfsq_rdn0_slot: &mut f64,
        var_dfsq_rdn1_slot: &mut f64,
        var_dfsq_rdn2_slot: &mut f64,
        var_dfsq_rdn3_slot: &mut f64,
        var_dfsq_rdn4_slot: &mut f64,
        var_dfsq_rdn5_slot: &mut f64,
        var_dfsq_rv_slot: &mut f64,
        var_dp_i_slot: &mut f64,
        var_dp_i_db0_slot: &mut f64,
        var_dp_i_db1_slot: &mut f64,
        var_dp_i_dn0_slot: &mut f64,
        var_dp_i_dn1_slot: &mut f64,
        var_dp_i_dn2_slot: &mut f64,
        var_dp_i_dn3_slot: &mut f64,
        var_dp_i_dn4_slot: &mut f64,
        var_dp_i_dn5_slot: &mut f64,
        var_dp_i_rdb0_slot: &mut f64,
        var_dp_i_rdb1_slot: &mut f64,
        var_dp_i_rdn0_slot: &mut f64,
        var_dp_i_rdn1_slot: &mut f64,
        var_dp_i_rdn2_slot: &mut f64,
        var_dp_i_rdn3_slot: &mut f64,
        var_dp_i_rdn4_slot: &mut f64,
        var_dp_i_rdn5_slot: &mut f64,
        var_dp_i_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_db0_slot: &mut f64,
        var_guard116_db1_slot: &mut f64,
        var_guard116_dn0_slot: &mut f64,
        var_guard116_dn1_slot: &mut f64,
        var_guard116_dn2_slot: &mut f64,
        var_guard116_dn3_slot: &mut f64,
        var_guard116_dn4_slot: &mut f64,
        var_guard116_dn5_slot: &mut f64,
        var_guard116_rdb0_slot: &mut f64,
        var_guard116_rdb1_slot: &mut f64,
        var_guard116_rdn0_slot: &mut f64,
        var_guard116_rdn1_slot: &mut f64,
        var_guard116_rdn2_slot: &mut f64,
        var_guard116_rdn3_slot: &mut f64,
        var_guard116_rdn4_slot: &mut f64,
        var_guard116_rdn5_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_db0_slot: &mut f64,
        var_guard117_db1_slot: &mut f64,
        var_guard117_dn0_slot: &mut f64,
        var_guard117_dn1_slot: &mut f64,
        var_guard117_dn2_slot: &mut f64,
        var_guard117_dn3_slot: &mut f64,
        var_guard117_dn4_slot: &mut f64,
        var_guard117_dn5_slot: &mut f64,
        var_guard117_rdb0_slot: &mut f64,
        var_guard117_rdb1_slot: &mut f64,
        var_guard117_rdn0_slot: &mut f64,
        var_guard117_rdn1_slot: &mut f64,
        var_guard117_rdn2_slot: &mut f64,
        var_guard117_rdn3_slot: &mut f64,
        var_guard117_rdn4_slot: &mut f64,
        var_guard117_rdn5_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_db0_slot: &mut f64,
        var_guard118_db1_slot: &mut f64,
        var_guard118_dn0_slot: &mut f64,
        var_guard118_dn1_slot: &mut f64,
        var_guard118_dn2_slot: &mut f64,
        var_guard118_dn3_slot: &mut f64,
        var_guard118_dn4_slot: &mut f64,
        var_guard118_dn5_slot: &mut f64,
        var_guard118_rdb0_slot: &mut f64,
        var_guard118_rdb1_slot: &mut f64,
        var_guard118_rdn0_slot: &mut f64,
        var_guard118_rdn1_slot: &mut f64,
        var_guard118_rdn2_slot: &mut f64,
        var_guard118_rdn3_slot: &mut f64,
        var_guard118_rdn4_slot: &mut f64,
        var_guard118_rdn5_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_db0_slot: &mut f64,
        var_guard119_db1_slot: &mut f64,
        var_guard119_dn0_slot: &mut f64,
        var_guard119_dn1_slot: &mut f64,
        var_guard119_dn2_slot: &mut f64,
        var_guard119_dn3_slot: &mut f64,
        var_guard119_dn4_slot: &mut f64,
        var_guard119_dn5_slot: &mut f64,
        var_guard119_rdb0_slot: &mut f64,
        var_guard119_rdb1_slot: &mut f64,
        var_guard119_rdn0_slot: &mut f64,
        var_guard119_rdn1_slot: &mut f64,
        var_guard119_rdn2_slot: &mut f64,
        var_guard119_rdn3_slot: &mut f64,
        var_guard119_rdn4_slot: &mut f64,
        var_guard119_rdn5_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_il_dple_slot: &mut f64,
        var_il_dple_db0_slot: &mut f64,
        var_il_dple_db1_slot: &mut f64,
        var_il_dple_dn0_slot: &mut f64,
        var_il_dple_dn1_slot: &mut f64,
        var_il_dple_dn2_slot: &mut f64,
        var_il_dple_dn3_slot: &mut f64,
        var_il_dple_dn4_slot: &mut f64,
        var_il_dple_dn5_slot: &mut f64,
        var_il_dple_rdb0_slot: &mut f64,
        var_il_dple_rdb1_slot: &mut f64,
        var_il_dple_rdn0_slot: &mut f64,
        var_il_dple_rdn1_slot: &mut f64,
        var_il_dple_rdn2_slot: &mut f64,
        var_il_dple_rdn3_slot: &mut f64,
        var_il_dple_rdn4_slot: &mut f64,
        var_il_dple_rdn5_slot: &mut f64,
        var_il_dple_rv_slot: &mut f64,
        var_iw_dpwe_slot: &mut f64,
        var_iw_dpwe_db0_slot: &mut f64,
        var_iw_dpwe_db1_slot: &mut f64,
        var_iw_dpwe_dn0_slot: &mut f64,
        var_iw_dpwe_dn1_slot: &mut f64,
        var_iw_dpwe_dn2_slot: &mut f64,
        var_iw_dpwe_dn3_slot: &mut f64,
        var_iw_dpwe_dn4_slot: &mut f64,
        var_iw_dpwe_dn5_slot: &mut f64,
        var_iw_dpwe_rdb0_slot: &mut f64,
        var_iw_dpwe_rdb1_slot: &mut f64,
        var_iw_dpwe_rdn0_slot: &mut f64,
        var_iw_dpwe_rdn1_slot: &mut f64,
        var_iw_dpwe_rdn2_slot: &mut f64,
        var_iw_dpwe_rdn3_slot: &mut f64,
        var_iw_dpwe_rdn4_slot: &mut f64,
        var_iw_dpwe_rdn5_slot: &mut f64,
        var_iw_dpwe_rv_slot: &mut f64,
        var_len_slot: &mut f64,
        var_len_db0_slot: &mut f64,
        var_len_db1_slot: &mut f64,
        var_len_dn0_slot: &mut f64,
        var_len_dn1_slot: &mut f64,
        var_len_dn2_slot: &mut f64,
        var_len_dn3_slot: &mut f64,
        var_len_dn4_slot: &mut f64,
        var_len_dn5_slot: &mut f64,
        var_len_rdb0_slot: &mut f64,
        var_len_rdb1_slot: &mut f64,
        var_len_rdn0_slot: &mut f64,
        var_len_rdn1_slot: &mut f64,
        var_len_rdn2_slot: &mut f64,
        var_len_rdn3_slot: &mut f64,
        var_len_rdn4_slot: &mut f64,
        var_len_rdn5_slot: &mut f64,
        var_len_rv_slot: &mut f64,
        var_vpo_slot: &mut f64,
        var_vpo_db0_slot: &mut f64,
        var_vpo_db1_slot: &mut f64,
        var_vpo_dn0_slot: &mut f64,
        var_vpo_dn1_slot: &mut f64,
        var_vpo_dn2_slot: &mut f64,
        var_vpo_dn3_slot: &mut f64,
        var_vpo_dn4_slot: &mut f64,
        var_vpo_dn5_slot: &mut f64,
        var_vpo_rdb0_slot: &mut f64,
        var_vpo_rdb1_slot: &mut f64,
        var_vpo_rdn0_slot: &mut f64,
        var_vpo_rdn1_slot: &mut f64,
        var_vpo_rdn2_slot: &mut f64,
        var_vpo_rdn3_slot: &mut f64,
        var_vpo_rdn4_slot: &mut f64,
        var_vpo_rdn5_slot: &mut f64,
        var_vpo_rv_slot: &mut f64,
        var_vpoe_slot: &mut f64,
        var_vpoe_db0_slot: &mut f64,
        var_vpoe_db1_slot: &mut f64,
        var_vpoe_dn0_slot: &mut f64,
        var_vpoe_dn1_slot: &mut f64,
        var_vpoe_dn2_slot: &mut f64,
        var_vpoe_dn3_slot: &mut f64,
        var_vpoe_dn4_slot: &mut f64,
        var_vpoe_dn5_slot: &mut f64,
        var_vpoe_rdb0_slot: &mut f64,
        var_vpoe_rdb1_slot: &mut f64,
        var_vpoe_rdn0_slot: &mut f64,
        var_vpoe_rdn1_slot: &mut f64,
        var_vpoe_rdn2_slot: &mut f64,
        var_vpoe_rdn3_slot: &mut f64,
        var_vpoe_rdn4_slot: &mut f64,
        var_vpoe_rdn5_slot: &mut f64,
        var_vpoe_rv_slot: &mut f64,
        var_wid_slot: &mut f64,
        var_wid_db0_slot: &mut f64,
        var_wid_db1_slot: &mut f64,
        var_wid_dn0_slot: &mut f64,
        var_wid_dn1_slot: &mut f64,
        var_wid_dn2_slot: &mut f64,
        var_wid_dn3_slot: &mut f64,
        var_wid_dn4_slot: &mut f64,
        var_wid_dn5_slot: &mut f64,
        var_wid_rdb0_slot: &mut f64,
        var_wid_rdb1_slot: &mut f64,
        var_wid_rdn0_slot: &mut f64,
        var_wid_rdn1_slot: &mut f64,
        var_wid_rdn2_slot: &mut f64,
        var_wid_rdn3_slot: &mut f64,
        var_wid_rdn4_slot: &mut f64,
        var_wid_rdn5_slot: &mut f64,
        var_wid_rv_slot: &mut f64,
    ) {
        let mut var_df: f64 = *var_df_slot;
        let mut var_df_db0: f64 = *var_df_db0_slot;
        let mut var_df_db1: f64 = *var_df_db1_slot;
        let mut var_df_dn0: f64 = *var_df_dn0_slot;
        let mut var_df_dn1: f64 = *var_df_dn1_slot;
        let mut var_df_dn2: f64 = *var_df_dn2_slot;
        let mut var_df_dn3: f64 = *var_df_dn3_slot;
        let mut var_df_dn4: f64 = *var_df_dn4_slot;
        let mut var_df_dn5: f64 = *var_df_dn5_slot;
        let mut var_df_rdb0: f64 = *var_df_rdb0_slot;
        let mut var_df_rdb1: f64 = *var_df_rdb1_slot;
        let mut var_df_rdn0: f64 = *var_df_rdn0_slot;
        let mut var_df_rdn1: f64 = *var_df_rdn1_slot;
        let mut var_df_rdn2: f64 = *var_df_rdn2_slot;
        let mut var_df_rdn3: f64 = *var_df_rdn3_slot;
        let mut var_df_rdn4: f64 = *var_df_rdn4_slot;
        let mut var_df_rdn5: f64 = *var_df_rdn5_slot;
        let mut var_df_rv: f64 = *var_df_rv_slot;
        let mut var_dfmin: f64 = *var_dfmin_slot;
        let mut var_dfmin_db0: f64 = *var_dfmin_db0_slot;
        let mut var_dfmin_db1: f64 = *var_dfmin_db1_slot;
        let mut var_dfmin_dn0: f64 = *var_dfmin_dn0_slot;
        let mut var_dfmin_dn1: f64 = *var_dfmin_dn1_slot;
        let mut var_dfmin_dn2: f64 = *var_dfmin_dn2_slot;
        let mut var_dfmin_dn3: f64 = *var_dfmin_dn3_slot;
        let mut var_dfmin_dn4: f64 = *var_dfmin_dn4_slot;
        let mut var_dfmin_dn5: f64 = *var_dfmin_dn5_slot;
        let mut var_dfmin_rdb0: f64 = *var_dfmin_rdb0_slot;
        let mut var_dfmin_rdb1: f64 = *var_dfmin_rdb1_slot;
        let mut var_dfmin_rdn0: f64 = *var_dfmin_rdn0_slot;
        let mut var_dfmin_rdn1: f64 = *var_dfmin_rdn1_slot;
        let mut var_dfmin_rdn2: f64 = *var_dfmin_rdn2_slot;
        let mut var_dfmin_rdn3: f64 = *var_dfmin_rdn3_slot;
        let mut var_dfmin_rdn4: f64 = *var_dfmin_rdn4_slot;
        let mut var_dfmin_rdn5: f64 = *var_dfmin_rdn5_slot;
        let mut var_dfmin_rv: f64 = *var_dfmin_rv_slot;
        let mut var_dfsq: f64 = *var_dfsq_slot;
        let mut var_dfsq_db0: f64 = *var_dfsq_db0_slot;
        let mut var_dfsq_db1: f64 = *var_dfsq_db1_slot;
        let mut var_dfsq_dn0: f64 = *var_dfsq_dn0_slot;
        let mut var_dfsq_dn1: f64 = *var_dfsq_dn1_slot;
        let mut var_dfsq_dn2: f64 = *var_dfsq_dn2_slot;
        let mut var_dfsq_dn3: f64 = *var_dfsq_dn3_slot;
        let mut var_dfsq_dn4: f64 = *var_dfsq_dn4_slot;
        let mut var_dfsq_dn5: f64 = *var_dfsq_dn5_slot;
        let mut var_dfsq_rdb0: f64 = *var_dfsq_rdb0_slot;
        let mut var_dfsq_rdb1: f64 = *var_dfsq_rdb1_slot;
        let mut var_dfsq_rdn0: f64 = *var_dfsq_rdn0_slot;
        let mut var_dfsq_rdn1: f64 = *var_dfsq_rdn1_slot;
        let mut var_dfsq_rdn2: f64 = *var_dfsq_rdn2_slot;
        let mut var_dfsq_rdn3: f64 = *var_dfsq_rdn3_slot;
        let mut var_dfsq_rdn4: f64 = *var_dfsq_rdn4_slot;
        let mut var_dfsq_rdn5: f64 = *var_dfsq_rdn5_slot;
        let mut var_dfsq_rv: f64 = *var_dfsq_rv_slot;
        let mut var_dp_i: f64 = *var_dp_i_slot;
        let mut var_dp_i_db0: f64 = *var_dp_i_db0_slot;
        let mut var_dp_i_db1: f64 = *var_dp_i_db1_slot;
        let mut var_dp_i_dn0: f64 = *var_dp_i_dn0_slot;
        let mut var_dp_i_dn1: f64 = *var_dp_i_dn1_slot;
        let mut var_dp_i_dn2: f64 = *var_dp_i_dn2_slot;
        let mut var_dp_i_dn3: f64 = *var_dp_i_dn3_slot;
        let mut var_dp_i_dn4: f64 = *var_dp_i_dn4_slot;
        let mut var_dp_i_dn5: f64 = *var_dp_i_dn5_slot;
        let mut var_dp_i_rdb0: f64 = *var_dp_i_rdb0_slot;
        let mut var_dp_i_rdb1: f64 = *var_dp_i_rdb1_slot;
        let mut var_dp_i_rdn0: f64 = *var_dp_i_rdn0_slot;
        let mut var_dp_i_rdn1: f64 = *var_dp_i_rdn1_slot;
        let mut var_dp_i_rdn2: f64 = *var_dp_i_rdn2_slot;
        let mut var_dp_i_rdn3: f64 = *var_dp_i_rdn3_slot;
        let mut var_dp_i_rdn4: f64 = *var_dp_i_rdn4_slot;
        let mut var_dp_i_rdn5: f64 = *var_dp_i_rdn5_slot;
        let mut var_dp_i_rv: f64 = *var_dp_i_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_db0: f64 = *var_guard116_db0_slot;
        let mut var_guard116_db1: f64 = *var_guard116_db1_slot;
        let mut var_guard116_dn0: f64 = *var_guard116_dn0_slot;
        let mut var_guard116_dn1: f64 = *var_guard116_dn1_slot;
        let mut var_guard116_dn2: f64 = *var_guard116_dn2_slot;
        let mut var_guard116_dn3: f64 = *var_guard116_dn3_slot;
        let mut var_guard116_dn4: f64 = *var_guard116_dn4_slot;
        let mut var_guard116_dn5: f64 = *var_guard116_dn5_slot;
        let mut var_guard116_rdb0: f64 = *var_guard116_rdb0_slot;
        let mut var_guard116_rdb1: f64 = *var_guard116_rdb1_slot;
        let mut var_guard116_rdn0: f64 = *var_guard116_rdn0_slot;
        let mut var_guard116_rdn1: f64 = *var_guard116_rdn1_slot;
        let mut var_guard116_rdn2: f64 = *var_guard116_rdn2_slot;
        let mut var_guard116_rdn3: f64 = *var_guard116_rdn3_slot;
        let mut var_guard116_rdn4: f64 = *var_guard116_rdn4_slot;
        let mut var_guard116_rdn5: f64 = *var_guard116_rdn5_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_db0: f64 = *var_guard117_db0_slot;
        let mut var_guard117_db1: f64 = *var_guard117_db1_slot;
        let mut var_guard117_dn0: f64 = *var_guard117_dn0_slot;
        let mut var_guard117_dn1: f64 = *var_guard117_dn1_slot;
        let mut var_guard117_dn2: f64 = *var_guard117_dn2_slot;
        let mut var_guard117_dn3: f64 = *var_guard117_dn3_slot;
        let mut var_guard117_dn4: f64 = *var_guard117_dn4_slot;
        let mut var_guard117_dn5: f64 = *var_guard117_dn5_slot;
        let mut var_guard117_rdb0: f64 = *var_guard117_rdb0_slot;
        let mut var_guard117_rdb1: f64 = *var_guard117_rdb1_slot;
        let mut var_guard117_rdn0: f64 = *var_guard117_rdn0_slot;
        let mut var_guard117_rdn1: f64 = *var_guard117_rdn1_slot;
        let mut var_guard117_rdn2: f64 = *var_guard117_rdn2_slot;
        let mut var_guard117_rdn3: f64 = *var_guard117_rdn3_slot;
        let mut var_guard117_rdn4: f64 = *var_guard117_rdn4_slot;
        let mut var_guard117_rdn5: f64 = *var_guard117_rdn5_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_db0: f64 = *var_guard118_db0_slot;
        let mut var_guard118_db1: f64 = *var_guard118_db1_slot;
        let mut var_guard118_dn0: f64 = *var_guard118_dn0_slot;
        let mut var_guard118_dn1: f64 = *var_guard118_dn1_slot;
        let mut var_guard118_dn2: f64 = *var_guard118_dn2_slot;
        let mut var_guard118_dn3: f64 = *var_guard118_dn3_slot;
        let mut var_guard118_dn4: f64 = *var_guard118_dn4_slot;
        let mut var_guard118_dn5: f64 = *var_guard118_dn5_slot;
        let mut var_guard118_rdb0: f64 = *var_guard118_rdb0_slot;
        let mut var_guard118_rdb1: f64 = *var_guard118_rdb1_slot;
        let mut var_guard118_rdn0: f64 = *var_guard118_rdn0_slot;
        let mut var_guard118_rdn1: f64 = *var_guard118_rdn1_slot;
        let mut var_guard118_rdn2: f64 = *var_guard118_rdn2_slot;
        let mut var_guard118_rdn3: f64 = *var_guard118_rdn3_slot;
        let mut var_guard118_rdn4: f64 = *var_guard118_rdn4_slot;
        let mut var_guard118_rdn5: f64 = *var_guard118_rdn5_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_db0: f64 = *var_guard119_db0_slot;
        let mut var_guard119_db1: f64 = *var_guard119_db1_slot;
        let mut var_guard119_dn0: f64 = *var_guard119_dn0_slot;
        let mut var_guard119_dn1: f64 = *var_guard119_dn1_slot;
        let mut var_guard119_dn2: f64 = *var_guard119_dn2_slot;
        let mut var_guard119_dn3: f64 = *var_guard119_dn3_slot;
        let mut var_guard119_dn4: f64 = *var_guard119_dn4_slot;
        let mut var_guard119_dn5: f64 = *var_guard119_dn5_slot;
        let mut var_guard119_rdb0: f64 = *var_guard119_rdb0_slot;
        let mut var_guard119_rdb1: f64 = *var_guard119_rdb1_slot;
        let mut var_guard119_rdn0: f64 = *var_guard119_rdn0_slot;
        let mut var_guard119_rdn1: f64 = *var_guard119_rdn1_slot;
        let mut var_guard119_rdn2: f64 = *var_guard119_rdn2_slot;
        let mut var_guard119_rdn3: f64 = *var_guard119_rdn3_slot;
        let mut var_guard119_rdn4: f64 = *var_guard119_rdn4_slot;
        let mut var_guard119_rdn5: f64 = *var_guard119_rdn5_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_il_dple: f64 = *var_il_dple_slot;
        let mut var_il_dple_db0: f64 = *var_il_dple_db0_slot;
        let mut var_il_dple_db1: f64 = *var_il_dple_db1_slot;
        let mut var_il_dple_dn0: f64 = *var_il_dple_dn0_slot;
        let mut var_il_dple_dn1: f64 = *var_il_dple_dn1_slot;
        let mut var_il_dple_dn2: f64 = *var_il_dple_dn2_slot;
        let mut var_il_dple_dn3: f64 = *var_il_dple_dn3_slot;
        let mut var_il_dple_dn4: f64 = *var_il_dple_dn4_slot;
        let mut var_il_dple_dn5: f64 = *var_il_dple_dn5_slot;
        let mut var_il_dple_rdb0: f64 = *var_il_dple_rdb0_slot;
        let mut var_il_dple_rdb1: f64 = *var_il_dple_rdb1_slot;
        let mut var_il_dple_rdn0: f64 = *var_il_dple_rdn0_slot;
        let mut var_il_dple_rdn1: f64 = *var_il_dple_rdn1_slot;
        let mut var_il_dple_rdn2: f64 = *var_il_dple_rdn2_slot;
        let mut var_il_dple_rdn3: f64 = *var_il_dple_rdn3_slot;
        let mut var_il_dple_rdn4: f64 = *var_il_dple_rdn4_slot;
        let mut var_il_dple_rdn5: f64 = *var_il_dple_rdn5_slot;
        let mut var_il_dple_rv: f64 = *var_il_dple_rv_slot;
        let mut var_iw_dpwe: f64 = *var_iw_dpwe_slot;
        let mut var_iw_dpwe_db0: f64 = *var_iw_dpwe_db0_slot;
        let mut var_iw_dpwe_db1: f64 = *var_iw_dpwe_db1_slot;
        let mut var_iw_dpwe_dn0: f64 = *var_iw_dpwe_dn0_slot;
        let mut var_iw_dpwe_dn1: f64 = *var_iw_dpwe_dn1_slot;
        let mut var_iw_dpwe_dn2: f64 = *var_iw_dpwe_dn2_slot;
        let mut var_iw_dpwe_dn3: f64 = *var_iw_dpwe_dn3_slot;
        let mut var_iw_dpwe_dn4: f64 = *var_iw_dpwe_dn4_slot;
        let mut var_iw_dpwe_dn5: f64 = *var_iw_dpwe_dn5_slot;
        let mut var_iw_dpwe_rdb0: f64 = *var_iw_dpwe_rdb0_slot;
        let mut var_iw_dpwe_rdb1: f64 = *var_iw_dpwe_rdb1_slot;
        let mut var_iw_dpwe_rdn0: f64 = *var_iw_dpwe_rdn0_slot;
        let mut var_iw_dpwe_rdn1: f64 = *var_iw_dpwe_rdn1_slot;
        let mut var_iw_dpwe_rdn2: f64 = *var_iw_dpwe_rdn2_slot;
        let mut var_iw_dpwe_rdn3: f64 = *var_iw_dpwe_rdn3_slot;
        let mut var_iw_dpwe_rdn4: f64 = *var_iw_dpwe_rdn4_slot;
        let mut var_iw_dpwe_rdn5: f64 = *var_iw_dpwe_rdn5_slot;
        let mut var_iw_dpwe_rv: f64 = *var_iw_dpwe_rv_slot;
        let mut var_len: f64 = *var_len_slot;
        let mut var_len_db0: f64 = *var_len_db0_slot;
        let mut var_len_db1: f64 = *var_len_db1_slot;
        let mut var_len_dn0: f64 = *var_len_dn0_slot;
        let mut var_len_dn1: f64 = *var_len_dn1_slot;
        let mut var_len_dn2: f64 = *var_len_dn2_slot;
        let mut var_len_dn3: f64 = *var_len_dn3_slot;
        let mut var_len_dn4: f64 = *var_len_dn4_slot;
        let mut var_len_dn5: f64 = *var_len_dn5_slot;
        let mut var_len_rdb0: f64 = *var_len_rdb0_slot;
        let mut var_len_rdb1: f64 = *var_len_rdb1_slot;
        let mut var_len_rdn0: f64 = *var_len_rdn0_slot;
        let mut var_len_rdn1: f64 = *var_len_rdn1_slot;
        let mut var_len_rdn2: f64 = *var_len_rdn2_slot;
        let mut var_len_rdn3: f64 = *var_len_rdn3_slot;
        let mut var_len_rdn4: f64 = *var_len_rdn4_slot;
        let mut var_len_rdn5: f64 = *var_len_rdn5_slot;
        let mut var_len_rv: f64 = *var_len_rv_slot;
        let mut var_vpo: f64 = *var_vpo_slot;
        let mut var_vpo_db0: f64 = *var_vpo_db0_slot;
        let mut var_vpo_db1: f64 = *var_vpo_db1_slot;
        let mut var_vpo_dn0: f64 = *var_vpo_dn0_slot;
        let mut var_vpo_dn1: f64 = *var_vpo_dn1_slot;
        let mut var_vpo_dn2: f64 = *var_vpo_dn2_slot;
        let mut var_vpo_dn3: f64 = *var_vpo_dn3_slot;
        let mut var_vpo_dn4: f64 = *var_vpo_dn4_slot;
        let mut var_vpo_dn5: f64 = *var_vpo_dn5_slot;
        let mut var_vpo_rdb0: f64 = *var_vpo_rdb0_slot;
        let mut var_vpo_rdb1: f64 = *var_vpo_rdb1_slot;
        let mut var_vpo_rdn0: f64 = *var_vpo_rdn0_slot;
        let mut var_vpo_rdn1: f64 = *var_vpo_rdn1_slot;
        let mut var_vpo_rdn2: f64 = *var_vpo_rdn2_slot;
        let mut var_vpo_rdn3: f64 = *var_vpo_rdn3_slot;
        let mut var_vpo_rdn4: f64 = *var_vpo_rdn4_slot;
        let mut var_vpo_rdn5: f64 = *var_vpo_rdn5_slot;
        let mut var_vpo_rv: f64 = *var_vpo_rv_slot;
        let mut var_vpoe: f64 = *var_vpoe_slot;
        let mut var_vpoe_db0: f64 = *var_vpoe_db0_slot;
        let mut var_vpoe_db1: f64 = *var_vpoe_db1_slot;
        let mut var_vpoe_dn0: f64 = *var_vpoe_dn0_slot;
        let mut var_vpoe_dn1: f64 = *var_vpoe_dn1_slot;
        let mut var_vpoe_dn2: f64 = *var_vpoe_dn2_slot;
        let mut var_vpoe_dn3: f64 = *var_vpoe_dn3_slot;
        let mut var_vpoe_dn4: f64 = *var_vpoe_dn4_slot;
        let mut var_vpoe_dn5: f64 = *var_vpoe_dn5_slot;
        let mut var_vpoe_rdb0: f64 = *var_vpoe_rdb0_slot;
        let mut var_vpoe_rdb1: f64 = *var_vpoe_rdb1_slot;
        let mut var_vpoe_rdn0: f64 = *var_vpoe_rdn0_slot;
        let mut var_vpoe_rdn1: f64 = *var_vpoe_rdn1_slot;
        let mut var_vpoe_rdn2: f64 = *var_vpoe_rdn2_slot;
        let mut var_vpoe_rdn3: f64 = *var_vpoe_rdn3_slot;
        let mut var_vpoe_rdn4: f64 = *var_vpoe_rdn4_slot;
        let mut var_vpoe_rdn5: f64 = *var_vpoe_rdn5_slot;
        let mut var_vpoe_rv: f64 = *var_vpoe_rv_slot;
        let mut var_wid: f64 = *var_wid_slot;
        let mut var_wid_db0: f64 = *var_wid_db0_slot;
        let mut var_wid_db1: f64 = *var_wid_db1_slot;
        let mut var_wid_dn0: f64 = *var_wid_dn0_slot;
        let mut var_wid_dn1: f64 = *var_wid_dn1_slot;
        let mut var_wid_dn2: f64 = *var_wid_dn2_slot;
        let mut var_wid_dn3: f64 = *var_wid_dn3_slot;
        let mut var_wid_dn4: f64 = *var_wid_dn4_slot;
        let mut var_wid_dn5: f64 = *var_wid_dn5_slot;
        let mut var_wid_rdb0: f64 = *var_wid_rdb0_slot;
        let mut var_wid_rdb1: f64 = *var_wid_rdb1_slot;
        let mut var_wid_rdn0: f64 = *var_wid_rdn0_slot;
        let mut var_wid_rdn1: f64 = *var_wid_rdn1_slot;
        let mut var_wid_rdn2: f64 = *var_wid_rdn2_slot;
        let mut var_wid_rdn3: f64 = *var_wid_rdn3_slot;
        let mut var_wid_rdn4: f64 = *var_wid_rdn4_slot;
        let mut var_wid_rdn5: f64 = *var_wid_rdn5_slot;
        let mut var_wid_rv: f64 = *var_wid_rv_slot;

        let (assign590_e678, assign590_e678_d_n0, assign590_e678_d_n1, assign590_e678_d_n2, assign590_e678_d_n3, assign590_e678_d_n4, assign590_e678_d_n5, assign590_e678_d_b0, assign590_e678_d_b1,) = {
    if (p.p53 != 0.0) {
        (var_leff_um, var_leff_um_dn0, var_leff_um_dn1, var_leff_um_dn2, var_leff_um_dn3, var_leff_um_dn4, var_leff_um_dn5, var_leff_um_db0, var_leff_um_db1,)
    } else {
        (var_len, var_len_dn0, var_len_dn1, var_len_dn2, var_len_dn3, var_len_dn4, var_len_dn5, var_len_db0, var_len_db1,)
    }
};
        var_len = assign590_e678;
        var_len_dn0 = assign590_e678_d_n0;
        var_len_dn1 = assign590_e678_d_n1;
        var_len_dn2 = assign590_e678_d_n2;
        var_len_dn3 = assign590_e678_d_n3;
        var_len_dn4 = assign590_e678_d_n4;
        var_len_dn5 = assign590_e678_d_n5;
        var_len_db0 = assign590_e678_d_b0;
        var_len_db1 = assign590_e678_d_b1;
        var_len_rv = 0.0;
        var_len_rdn0 = 0.0;
        var_len_rdn1 = 0.0;
        var_len_rdn2 = 0.0;
        var_len_rdn3 = 0.0;
        var_len_rdn4 = 0.0;
        var_len_rdn5 = 0.0;
        var_len_rdb0 = 0.0;
        var_len_rdb1 = 0.0;

        let (assign600_e683, assign600_e683_d_n0, assign600_e683_d_n1, assign600_e683_d_n2, assign600_e683_d_n3, assign600_e683_d_n4, assign600_e683_d_n5, assign600_e683_d_b0, assign600_e683_d_b1,) = {
    if (p.p53 == 0.0) {
        (var_w_um, var_w_um_dn0, var_w_um_dn1, var_w_um_dn2, var_w_um_dn3, var_w_um_dn4, var_w_um_dn5, var_w_um_db0, var_w_um_db1,)
    } else {
        (var_wid, var_wid_dn0, var_wid_dn1, var_wid_dn2, var_wid_dn3, var_wid_dn4, var_wid_dn5, var_wid_db0, var_wid_db1,)
    }
};
        var_wid = assign600_e683;
        var_wid_dn0 = assign600_e683_d_n0;
        var_wid_dn1 = assign600_e683_d_n1;
        var_wid_dn2 = assign600_e683_d_n2;
        var_wid_dn3 = assign600_e683_d_n3;
        var_wid_dn4 = assign600_e683_d_n4;
        var_wid_dn5 = assign600_e683_d_n5;
        var_wid_db0 = assign600_e683_d_b0;
        var_wid_db1 = assign600_e683_d_b1;
        var_wid_rv = 0.0;
        var_wid_rdn0 = 0.0;
        var_wid_rdn1 = 0.0;
        var_wid_rdn2 = 0.0;
        var_wid_rdn3 = 0.0;
        var_wid_rdn4 = 0.0;
        var_wid_rdn5 = 0.0;
        var_wid_rdb0 = 0.0;
        var_wid_rdb1 = 0.0;

        let (assign610_e688, assign610_e688_d_n0, assign610_e688_d_n1, assign610_e688_d_n2, assign610_e688_d_n3, assign610_e688_d_n4, assign610_e688_d_n5, assign610_e688_d_b0, assign610_e688_d_b1,) = {
    if (p.p53 == 0.0) {
        (var_l_um, var_l_um_dn0, var_l_um_dn1, var_l_um_dn2, var_l_um_dn3, var_l_um_dn4, var_l_um_dn5, var_l_um_db0, var_l_um_db1,)
    } else {
        (var_len, var_len_dn0, var_len_dn1, var_len_dn2, var_len_dn3, var_len_dn4, var_len_dn5, var_len_db0, var_len_db1,)
    }
};
        var_len = assign610_e688;
        var_len_dn0 = assign610_e688_d_n0;
        var_len_dn1 = assign610_e688_d_n1;
        var_len_dn2 = assign610_e688_d_n2;
        var_len_dn3 = assign610_e688_d_n3;
        var_len_dn4 = assign610_e688_d_n4;
        var_len_dn5 = assign610_e688_d_n5;
        var_len_db0 = assign610_e688_d_b0;
        var_len_db1 = assign610_e688_d_b1;
        var_len_rv = 0.0;
        var_len_rdn0 = 0.0;
        var_len_rdn1 = 0.0;
        var_len_rdn2 = 0.0;
        var_len_rdn3 = 0.0;
        var_len_rdn4 = 0.0;
        var_len_rdn5 = 0.0;
        var_len_rdb0 = 0.0;
        var_len_rdb1 = 0.0;

        let assign620_e692: f64 = (var_wid).powf(p.p56);
        let assign620_e693: f64 = (1.0 / assign620_e692);
        var_iw_dpwe = assign620_e693;
        var_iw_dpwe_dn0 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn0)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn0 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_dn1 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn1)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn1 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_dn2 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn2)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn2 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_dn3 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn3)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn3 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_dn4 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn4)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn4 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_dn5 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_dn5)) } } else { (assign620_e692 * (p.p56 * (var_wid_dn5 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_db0 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_db0)) } } else { (assign620_e692 * (p.p56 * (var_wid_db0 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_db1 = (-(if 0.0 == 0.0 && ((p.p56) as f64).is_finite() && ((p.p56) as f64).fract() == 0.0 { if p.p56 == 0.0 { 0.0 } else { (p.p56 * ((var_wid).powf(p.p56 - 1.0) * var_wid_db1)) } } else { (assign620_e692 * (p.p56 * (var_wid_db1 / var_wid))) } / (assign620_e692 * assign620_e692)));
        var_iw_dpwe_rv = 0.0;
        var_iw_dpwe_rdn0 = 0.0;
        var_iw_dpwe_rdn1 = 0.0;
        var_iw_dpwe_rdn2 = 0.0;
        var_iw_dpwe_rdn3 = 0.0;
        var_iw_dpwe_rdn4 = 0.0;
        var_iw_dpwe_rdn5 = 0.0;
        var_iw_dpwe_rdb0 = 0.0;
        var_iw_dpwe_rdb1 = 0.0;

        let assign630_e697: f64 = (var_len).powf(p.p58);
        let assign630_e698: f64 = (1.0 / assign630_e697);
        var_il_dple = assign630_e698;
        var_il_dple_dn0 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn0)) } } else { (assign630_e697 * (p.p58 * (var_len_dn0 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_dn1 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn1)) } } else { (assign630_e697 * (p.p58 * (var_len_dn1 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_dn2 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn2)) } } else { (assign630_e697 * (p.p58 * (var_len_dn2 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_dn3 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn3)) } } else { (assign630_e697 * (p.p58 * (var_len_dn3 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_dn4 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn4)) } } else { (assign630_e697 * (p.p58 * (var_len_dn4 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_dn5 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_dn5)) } } else { (assign630_e697 * (p.p58 * (var_len_dn5 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_db0 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_db0)) } } else { (assign630_e697 * (p.p58 * (var_len_db0 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_db1 = (-(if 0.0 == 0.0 && ((p.p58) as f64).is_finite() && ((p.p58) as f64).fract() == 0.0 { if p.p58 == 0.0 { 0.0 } else { (p.p58 * ((var_len).powf(p.p58 - 1.0) * var_len_db1)) } } else { (assign630_e697 * (p.p58 * (var_len_db1 / var_len))) } / (assign630_e697 * assign630_e697)));
        var_il_dple_rv = 0.0;
        var_il_dple_rdn0 = 0.0;
        var_il_dple_rdn1 = 0.0;
        var_il_dple_rdn2 = 0.0;
        var_il_dple_rdn3 = 0.0;
        var_il_dple_rdn4 = 0.0;
        var_il_dple_rdn5 = 0.0;
        var_il_dple_rdb0 = 0.0;
        var_il_dple_rdb1 = 0.0;

        let assign640_e703: f64 = (p.p55 * var_iw_dpwe);
        let assign640_e704: f64 = (1.0 + assign640_e703);
        let assign640_e705: f64 = (p.p54 * assign640_e704);
        let assign640_e709: f64 = (p.p57 * var_il_dple);
        let assign640_e710: f64 = (1.0 + assign640_e709);
        let assign640_e711: f64 = (assign640_e705 * assign640_e710);
        let assign640_e715: f64 = (p.p59 * var_iw_dpwe);
        let assign640_e717: f64 = (assign640_e715 * var_il_dple);
        let assign640_e718: f64 = (1.0 + assign640_e717);
        let assign640_e719: f64 = (assign640_e711 * assign640_e718);
        let assign640_e725: f64 = (var_dt * p.p104);
        let assign640_e726: f64 = (p.p103 + assign640_e725);
        let assign640_e727: f64 = (var_dt * assign640_e726);
        let assign640_e728: f64 = (1.0 + assign640_e727);
        let assign640_e729: f64 = (assign640_e719 * assign640_e728);
        var_dp_i = assign640_e729;
        var_dp_i_dn0 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn0)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn0))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn0) * var_il_dple) + (assign640_e715 * var_il_dple_dn0)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn0 * assign640_e726) + (var_dt * (var_dt_dn0 * p.p104)))));
        var_dp_i_dn1 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn1)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn1))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn1) * var_il_dple) + (assign640_e715 * var_il_dple_dn1)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn1 * assign640_e726) + (var_dt * (var_dt_dn1 * p.p104)))));
        var_dp_i_dn2 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn2)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn2))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn2) * var_il_dple) + (assign640_e715 * var_il_dple_dn2)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn2 * assign640_e726) + (var_dt * (var_dt_dn2 * p.p104)))));
        var_dp_i_dn3 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn3)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn3))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn3) * var_il_dple) + (assign640_e715 * var_il_dple_dn3)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn3 * assign640_e726) + (var_dt * (var_dt_dn3 * p.p104)))));
        var_dp_i_dn4 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn4)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn4))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn4) * var_il_dple) + (assign640_e715 * var_il_dple_dn4)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn4 * assign640_e726) + (var_dt * (var_dt_dn4 * p.p104)))));
        var_dp_i_dn5 = (((((((p.p54 * (p.p55 * var_iw_dpwe_dn5)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_dn5))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_dn5) * var_il_dple) + (assign640_e715 * var_il_dple_dn5)))) * assign640_e728) + (assign640_e719 * ((var_dt_dn5 * assign640_e726) + (var_dt * (var_dt_dn5 * p.p104)))));
        var_dp_i_db0 = (((((((p.p54 * (p.p55 * var_iw_dpwe_db0)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_db0))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_db0) * var_il_dple) + (assign640_e715 * var_il_dple_db0)))) * assign640_e728) + (assign640_e719 * ((var_dt_db0 * assign640_e726) + (var_dt * (var_dt_db0 * p.p104)))));
        var_dp_i_db1 = (((((((p.p54 * (p.p55 * var_iw_dpwe_db1)) * assign640_e710) + (assign640_e705 * (p.p57 * var_il_dple_db1))) * assign640_e718) + (assign640_e711 * (((p.p59 * var_iw_dpwe_db1) * var_il_dple) + (assign640_e715 * var_il_dple_db1)))) * assign640_e728) + (assign640_e719 * ((var_dt_db1 * assign640_e726) + (var_dt * (var_dt_db1 * p.p104)))));
        var_dp_i_rv = 0.0;
        var_dp_i_rdn0 = 0.0;
        var_dp_i_rdn1 = 0.0;
        var_dp_i_rdn2 = 0.0;
        var_dp_i_rdn3 = 0.0;
        var_dp_i_rdn4 = 0.0;
        var_dp_i_rdn5 = 0.0;
        var_dp_i_rdb0 = 0.0;
        var_dp_i_rdb1 = 0.0;

        let (assign650_e735, assign650_e735_d_n0, assign650_e735_d_n1, assign650_e735_d_n2, assign650_e735_d_n3, assign650_e735_d_n4, assign650_e735_d_n5, assign650_e735_d_b0, assign650_e735_d_b1,) = {
    if (var_dp_i > 0.1) {
        (var_dp_i, var_dp_i_dn0, var_dp_i_dn1, var_dp_i_dn2, var_dp_i_dn3, var_dp_i_dn4, var_dp_i_dn5, var_dp_i_db0, var_dp_i_db1,)
    } else {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        var_dp_i = assign650_e735;
        var_dp_i_dn0 = assign650_e735_d_n0;
        var_dp_i_dn1 = assign650_e735_d_n1;
        var_dp_i_dn2 = assign650_e735_d_n2;
        var_dp_i_dn3 = assign650_e735_d_n3;
        var_dp_i_dn4 = assign650_e735_d_n4;
        var_dp_i_dn5 = assign650_e735_d_n5;
        var_dp_i_db0 = assign650_e735_d_b0;
        var_dp_i_db1 = assign650_e735_d_b1;
        var_dp_i_rv = 0.0;
        var_dp_i_rdn0 = 0.0;
        var_dp_i_rdn1 = 0.0;
        var_dp_i_rdn2 = 0.0;
        var_dp_i_rdn3 = 0.0;
        var_dp_i_rdn4 = 0.0;
        var_dp_i_rdn5 = 0.0;
        var_dp_i_rdb0 = 0.0;
        var_dp_i_rdb1 = 0.0;

        let assign660_e737: f64 = (var_dp_i).sqrt();
        let assign660_e740: f64 = (var_dp_i + 10000.0);
        let assign660_e741: f64 = (assign660_e737 / assign660_e740);
        var_dfmin = assign660_e741;
        var_dfmin_dn0 = ((((var_dp_i_dn0 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn0)) / (assign660_e740 * assign660_e740));
        var_dfmin_dn1 = ((((var_dp_i_dn1 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn1)) / (assign660_e740 * assign660_e740));
        var_dfmin_dn2 = ((((var_dp_i_dn2 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn2)) / (assign660_e740 * assign660_e740));
        var_dfmin_dn3 = ((((var_dp_i_dn3 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn3)) / (assign660_e740 * assign660_e740));
        var_dfmin_dn4 = ((((var_dp_i_dn4 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn4)) / (assign660_e740 * assign660_e740));
        var_dfmin_dn5 = ((((var_dp_i_dn5 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_dn5)) / (assign660_e740 * assign660_e740));
        var_dfmin_db0 = ((((var_dp_i_db0 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_db0)) / (assign660_e740 * assign660_e740));
        var_dfmin_db1 = ((((var_dp_i_db1 / (2.0 * assign660_e737)) * assign660_e740) - (assign660_e737 * var_dp_i_db1)) / (assign660_e740 * assign660_e740));
        var_dfmin_rv = 0.0;
        var_dfmin_rdn0 = 0.0;
        var_dfmin_rdn1 = 0.0;
        var_dfmin_rdn2 = 0.0;
        var_dfmin_rdn3 = 0.0;
        var_dfmin_rdn4 = 0.0;
        var_dfmin_rdn5 = 0.0;
        var_dfmin_rdb0 = 0.0;
        var_dfmin_rdb1 = 0.0;

        let (assign670_e759, assign670_e759_d_n0, assign670_e759_d_n1, assign670_e759_d_n2, assign670_e759_d_n3, assign670_e759_d_n4, assign670_e759_d_n5, assign670_e759_d_b0, assign670_e759_d_b1,) = {
    if (p.p15 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign670_e747: f64 = (p.p50 * var_len);
        let assign670_e750: f64 = (p.p51 * var_wid);
        let assign670_e751: f64 = (assign670_e747 + assign670_e750);
        let assign670_e753: f64 = (assign670_e751 + p.p52);
        let assign670_e756: f64 = (var_len * var_wid);
        let assign670_e757: f64 = (assign670_e753 / assign670_e756);
        let assign670_e758: f64 = (p.p49 + assign670_e757);
        (assign670_e758, (((((p.p50 * var_len_dn0) + (p.p51 * var_wid_dn0)) * assign670_e756) - (assign670_e753 * ((var_len_dn0 * var_wid) + (var_len * var_wid_dn0)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_dn1) + (p.p51 * var_wid_dn1)) * assign670_e756) - (assign670_e753 * ((var_len_dn1 * var_wid) + (var_len * var_wid_dn1)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_dn2) + (p.p51 * var_wid_dn2)) * assign670_e756) - (assign670_e753 * ((var_len_dn2 * var_wid) + (var_len * var_wid_dn2)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_dn3) + (p.p51 * var_wid_dn3)) * assign670_e756) - (assign670_e753 * ((var_len_dn3 * var_wid) + (var_len * var_wid_dn3)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_dn4) + (p.p51 * var_wid_dn4)) * assign670_e756) - (assign670_e753 * ((var_len_dn4 * var_wid) + (var_len * var_wid_dn4)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_dn5) + (p.p51 * var_wid_dn5)) * assign670_e756) - (assign670_e753 * ((var_len_dn5 * var_wid) + (var_len * var_wid_dn5)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_db0) + (p.p51 * var_wid_db0)) * assign670_e756) - (assign670_e753 * ((var_len_db0 * var_wid) + (var_len * var_wid_db0)))) / (assign670_e756 * assign670_e756)), (((((p.p50 * var_len_db1) + (p.p51 * var_wid_db1)) * assign670_e756) - (assign670_e753 * ((var_len_db1 * var_wid) + (var_len * var_wid_db1)))) / (assign670_e756 * assign670_e756)),)
    }
};
        var_df = assign670_e759;
        var_df_dn0 = assign670_e759_d_n0;
        var_df_dn1 = assign670_e759_d_n1;
        var_df_dn2 = assign670_e759_d_n2;
        var_df_dn3 = assign670_e759_d_n3;
        var_df_dn4 = assign670_e759_d_n4;
        var_df_dn5 = assign670_e759_d_n5;
        var_df_db0 = assign670_e759_d_b0;
        var_df_db1 = assign670_e759_d_b1;
        var_df_rv = 0.0;
        var_df_rdn0 = 0.0;
        var_df_rdn1 = 0.0;
        var_df_rdn2 = 0.0;
        var_df_rdn3 = 0.0;
        var_df_rdn4 = 0.0;
        var_df_rdn5 = 0.0;
        var_df_rdb0 = 0.0;
        var_df_rdb1 = 0.0;

        let assign680_e762: f64 = if var_df < var_dfmin { 1.0 } else { 0.0 };
        var_guard116 = assign680_e762;
        var_guard116_dn0 = 0.0;
        var_guard116_dn1 = 0.0;
        var_guard116_dn2 = 0.0;
        var_guard116_dn3 = 0.0;
        var_guard116_dn4 = 0.0;
        var_guard116_dn5 = 0.0;
        var_guard116_db0 = 0.0;
        var_guard116_db1 = 0.0;
        var_guard116_rv = 0.0;
        var_guard116_rdn0 = 0.0;
        var_guard116_rdn1 = 0.0;
        var_guard116_rdn2 = 0.0;
        var_guard116_rdn3 = 0.0;
        var_guard116_rdn4 = 0.0;
        var_guard116_rdn5 = 0.0;
        var_guard116_rdb0 = 0.0;
        var_guard116_rdb1 = 0.0;

        let (assign690_e771, assign690_e771_d_n0, assign690_e771_d_n1, assign690_e771_d_n2, assign690_e771_d_n3, assign690_e771_d_n4, assign690_e771_d_n5, assign690_e771_d_b0, assign690_e771_d_b1,) = {
    if (var_guard116 != 0.0) {
        let (assign690_e769, assign690_e769_d_n0, assign690_e769_d_n1, assign690_e769_d_n2, assign690_e769_d_n3, assign690_e769_d_n4, assign690_e769_d_n5, assign690_e769_d_b0, assign690_e769_d_b1,) = {
            if (var_df > 0.0) {
                (var_df, var_df_dn0, var_df_dn1, var_df_dn2, var_df_dn3, var_df_dn4, var_df_dn5, var_df_db0, var_df_db1,)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign690_e769, assign690_e769_d_n0, assign690_e769_d_n1, assign690_e769_d_n2, assign690_e769_d_n3, assign690_e769_d_n4, assign690_e769_d_n5, assign690_e769_d_b0, assign690_e769_d_b1,)
    } else {
        (var_df, var_df_dn0, var_df_dn1, var_df_dn2, var_df_dn3, var_df_dn4, var_df_dn5, var_df_db0, var_df_db1,)
    }
};
        var_df = assign690_e771;
        var_df_dn0 = assign690_e771_d_n0;
        var_df_dn1 = assign690_e771_d_n1;
        var_df_dn2 = assign690_e771_d_n2;
        var_df_dn3 = assign690_e771_d_n3;
        var_df_dn4 = assign690_e771_d_n4;
        var_df_dn5 = assign690_e771_d_n5;
        var_df_db0 = assign690_e771_d_b0;
        var_df_db1 = assign690_e771_d_b1;
        var_df_rv = 0.0;
        var_df_rdn0 = 0.0;
        var_df_rdn1 = 0.0;
        var_df_rdn2 = 0.0;
        var_df_rdn3 = 0.0;
        var_df_rdn4 = 0.0;
        var_df_rdn5 = 0.0;
        var_df_rdb0 = 0.0;
        var_df_rdb1 = 0.0;

        let (assign700_e777, assign700_e777_d_n0, assign700_e777_d_n1, assign700_e777_d_n2, assign700_e777_d_n3, assign700_e777_d_n4, assign700_e777_d_n5, assign700_e777_d_b0, assign700_e777_d_b1,) = {
    if (var_guard116 != 0.0) {
        let assign700_e775: f64 = (var_dfmin * var_dfmin);
        (assign700_e775, ((var_dfmin_dn0 * var_dfmin) + (var_dfmin * var_dfmin_dn0)), ((var_dfmin_dn1 * var_dfmin) + (var_dfmin * var_dfmin_dn1)), ((var_dfmin_dn2 * var_dfmin) + (var_dfmin * var_dfmin_dn2)), ((var_dfmin_dn3 * var_dfmin) + (var_dfmin * var_dfmin_dn3)), ((var_dfmin_dn4 * var_dfmin) + (var_dfmin * var_dfmin_dn4)), ((var_dfmin_dn5 * var_dfmin) + (var_dfmin * var_dfmin_dn5)), ((var_dfmin_db0 * var_dfmin) + (var_dfmin * var_dfmin_db0)), ((var_dfmin_db1 * var_dfmin) + (var_dfmin * var_dfmin_db1)),)
    } else {
        (var_dfsq, var_dfsq_dn0, var_dfsq_dn1, var_dfsq_dn2, var_dfsq_dn3, var_dfsq_dn4, var_dfsq_dn5, var_dfsq_db0, var_dfsq_db1,)
    }
};
        var_dfsq = assign700_e777;
        var_dfsq_dn0 = assign700_e777_d_n0;
        var_dfsq_dn1 = assign700_e777_d_n1;
        var_dfsq_dn2 = assign700_e777_d_n2;
        var_dfsq_dn3 = assign700_e777_d_n3;
        var_dfsq_dn4 = assign700_e777_d_n4;
        var_dfsq_dn5 = assign700_e777_d_n5;
        var_dfsq_db0 = assign700_e777_d_b0;
        var_dfsq_db1 = assign700_e777_d_b1;
        var_dfsq_rv = 0.0;
        var_dfsq_rdn0 = 0.0;
        var_dfsq_rdn1 = 0.0;
        var_dfsq_rdn2 = 0.0;
        var_dfsq_rdn3 = 0.0;
        var_dfsq_rdn4 = 0.0;
        var_dfsq_rdn5 = 0.0;
        var_dfsq_rdb0 = 0.0;
        var_dfsq_rdb1 = 0.0;

        let (assign710_e784, assign710_e784_d_n0, assign710_e784_d_n1, assign710_e784_d_n2, assign710_e784_d_n3, assign710_e784_d_n4, assign710_e784_d_n5, assign710_e784_d_b0, assign710_e784_d_b1,) = {
    if (var_guard116 == 0.0) {
        let assign710_e782: f64 = (var_df * var_df);
        (assign710_e782, ((var_df_dn0 * var_df) + (var_df * var_df_dn0)), ((var_df_dn1 * var_df) + (var_df * var_df_dn1)), ((var_df_dn2 * var_df) + (var_df * var_df_dn2)), ((var_df_dn3 * var_df) + (var_df * var_df_dn3)), ((var_df_dn4 * var_df) + (var_df * var_df_dn4)), ((var_df_dn5 * var_df) + (var_df * var_df_dn5)), ((var_df_db0 * var_df) + (var_df * var_df_db0)), ((var_df_db1 * var_df) + (var_df * var_df_db1)),)
    } else {
        (var_dfsq, var_dfsq_dn0, var_dfsq_dn1, var_dfsq_dn2, var_dfsq_dn3, var_dfsq_dn4, var_dfsq_dn5, var_dfsq_db0, var_dfsq_db1,)
    }
};
        var_dfsq = assign710_e784;
        var_dfsq_dn0 = assign710_e784_d_n0;
        var_dfsq_dn1 = assign710_e784_d_n1;
        var_dfsq_dn2 = assign710_e784_d_n2;
        var_dfsq_dn3 = assign710_e784_d_n3;
        var_dfsq_dn4 = assign710_e784_d_n4;
        var_dfsq_dn5 = assign710_e784_d_n5;
        var_dfsq_db0 = assign710_e784_d_b0;
        var_dfsq_db1 = assign710_e784_d_b1;
        var_dfsq_rv = 0.0;
        var_dfsq_rdn0 = 0.0;
        var_dfsq_rdn1 = 0.0;
        var_dfsq_rdn2 = 0.0;
        var_dfsq_rdn3 = 0.0;
        var_dfsq_rdn4 = 0.0;
        var_dfsq_rdn5 = 0.0;
        var_dfsq_rdb0 = 0.0;
        var_dfsq_rdb1 = 0.0;

        let assign720_e787: f64 = (0.5 / var_dfsq);
        let assign720_e790: f64 = (var_dp_i * 0.5);
        let assign720_e791: f64 = (assign720_e787 - assign720_e790);
        var_vpo = assign720_e791;
        var_vpo_dn0 = ((-((0.5 * var_dfsq_dn0) / (var_dfsq * var_dfsq))) - (var_dp_i_dn0 * 0.5));
        var_vpo_dn1 = ((-((0.5 * var_dfsq_dn1) / (var_dfsq * var_dfsq))) - (var_dp_i_dn1 * 0.5));
        var_vpo_dn2 = ((-((0.5 * var_dfsq_dn2) / (var_dfsq * var_dfsq))) - (var_dp_i_dn2 * 0.5));
        var_vpo_dn3 = ((-((0.5 * var_dfsq_dn3) / (var_dfsq * var_dfsq))) - (var_dp_i_dn3 * 0.5));
        var_vpo_dn4 = ((-((0.5 * var_dfsq_dn4) / (var_dfsq * var_dfsq))) - (var_dp_i_dn4 * 0.5));
        var_vpo_dn5 = ((-((0.5 * var_dfsq_dn5) / (var_dfsq * var_dfsq))) - (var_dp_i_dn5 * 0.5));
        var_vpo_db0 = ((-((0.5 * var_dfsq_db0) / (var_dfsq * var_dfsq))) - (var_dp_i_db0 * 0.5));
        var_vpo_db1 = ((-((0.5 * var_dfsq_db1) / (var_dfsq * var_dfsq))) - (var_dp_i_db1 * 0.5));
        var_vpo_rv = 0.0;
        var_vpo_rdn0 = 0.0;
        var_vpo_rdn1 = 0.0;
        var_vpo_rdn2 = 0.0;
        var_vpo_rdn3 = 0.0;
        var_vpo_rdn4 = 0.0;
        var_vpo_rdn5 = 0.0;
        var_vpo_rdb0 = 0.0;
        var_vpo_rdb1 = 0.0;

        let assign730_e794: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        var_guard117 = assign730_e794;
        var_guard117_dn0 = 0.0;
        var_guard117_dn1 = 0.0;
        var_guard117_dn2 = 0.0;
        var_guard117_dn3 = 0.0;
        var_guard117_dn4 = 0.0;
        var_guard117_dn5 = 0.0;
        var_guard117_db0 = 0.0;
        var_guard117_db1 = 0.0;
        var_guard117_rv = 0.0;
        var_guard117_rdn0 = 0.0;
        var_guard117_rdn1 = 0.0;
        var_guard117_rdn2 = 0.0;
        var_guard117_rdn3 = 0.0;
        var_guard117_rdn4 = 0.0;
        var_guard117_rdn5 = 0.0;
        var_guard117_rdb0 = 0.0;
        var_guard117_rdb1 = 0.0;

        let (assign740_e804, assign740_e804_d_n0, assign740_e804_d_n1, assign740_e804_d_n2, assign740_e804_d_n3, assign740_e804_d_n4, assign740_e804_d_n5, assign740_e804_d_b0, assign740_e804_d_b1,) = {
    if (var_guard117 != 0.0) {
        let assign740_e799: f64 = (2.0 * p.p64);
        let assign740_e801: f64 = (assign740_e799 / var_dfsq);
        let assign740_e802: f64 = (var_vpo - assign740_e801);
        (assign740_e802, (var_vpo_dn0 - (-((assign740_e799 * var_dfsq_dn0) / (var_dfsq * var_dfsq)))), (var_vpo_dn1 - (-((assign740_e799 * var_dfsq_dn1) / (var_dfsq * var_dfsq)))), (var_vpo_dn2 - (-((assign740_e799 * var_dfsq_dn2) / (var_dfsq * var_dfsq)))), (var_vpo_dn3 - (-((assign740_e799 * var_dfsq_dn3) / (var_dfsq * var_dfsq)))), (var_vpo_dn4 - (-((assign740_e799 * var_dfsq_dn4) / (var_dfsq * var_dfsq)))), (var_vpo_dn5 - (-((assign740_e799 * var_dfsq_dn5) / (var_dfsq * var_dfsq)))), (var_vpo_db0 - (-((assign740_e799 * var_dfsq_db0) / (var_dfsq * var_dfsq)))), (var_vpo_db1 - (-((assign740_e799 * var_dfsq_db1) / (var_dfsq * var_dfsq)))),)
    } else {
        (var_vpoe, var_vpoe_dn0, var_vpoe_dn1, var_vpoe_dn2, var_vpoe_dn3, var_vpoe_dn4, var_vpoe_dn5, var_vpoe_db0, var_vpoe_db1,)
    }
};
        var_vpoe = assign740_e804;
        var_vpoe_dn0 = assign740_e804_d_n0;
        var_vpoe_dn1 = assign740_e804_d_n1;
        var_vpoe_dn2 = assign740_e804_d_n2;
        var_vpoe_dn3 = assign740_e804_d_n3;
        var_vpoe_dn4 = assign740_e804_d_n4;
        var_vpoe_dn5 = assign740_e804_d_n5;
        var_vpoe_db0 = assign740_e804_d_b0;
        var_vpoe_db1 = assign740_e804_d_b1;
        var_vpoe_rv = 0.0;
        var_vpoe_rdn0 = 0.0;
        var_vpoe_rdn1 = 0.0;
        var_vpoe_rdn2 = 0.0;
        var_vpoe_rdn3 = 0.0;
        var_vpoe_rdn4 = 0.0;
        var_vpoe_rdn5 = 0.0;
        var_vpoe_rdb0 = 0.0;
        var_vpoe_rdb1 = 0.0;

        let assign760_e817: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        var_guard118 = assign760_e817;
        var_guard118_dn0 = 0.0;
        var_guard118_dn1 = 0.0;
        var_guard118_dn2 = 0.0;
        var_guard118_dn3 = 0.0;
        var_guard118_dn4 = 0.0;
        var_guard118_dn5 = 0.0;
        var_guard118_db0 = 0.0;
        var_guard118_db1 = 0.0;
        var_guard118_rv = 0.0;
        var_guard118_rdn0 = 0.0;
        var_guard118_rdn1 = 0.0;
        var_guard118_rdn2 = 0.0;
        var_guard118_rdn3 = 0.0;
        var_guard118_rdn4 = 0.0;
        var_guard118_rdn5 = 0.0;
        var_guard118_rdb0 = 0.0;
        var_guard118_rdb1 = 0.0;

        let (assign770_e831, assign770_e831_d_n0, assign770_e831_d_n1, assign770_e831_d_n2, assign770_e831_d_n3, assign770_e831_d_n4, assign770_e831_d_n5, assign770_e831_d_b0, assign770_e831_d_b1,) = {
    if ((var_guard117 == 0.0) && (var_guard118 != 0.0)) {
        let assign770_e825: f64 = (2.0 * p.p64);
        let assign770_e827: f64 = (assign770_e825 / var_dfsq);
        let assign770_e828: f64 = (assign770_e827).sqrt();
        let assign770_e829: f64 = (var_vpo - assign770_e828);
        (assign770_e829, (var_vpo_dn0 - ((-((assign770_e825 * var_dfsq_dn0) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_dn1 - ((-((assign770_e825 * var_dfsq_dn1) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_dn2 - ((-((assign770_e825 * var_dfsq_dn2) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_dn3 - ((-((assign770_e825 * var_dfsq_dn3) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_dn4 - ((-((assign770_e825 * var_dfsq_dn4) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_dn5 - ((-((assign770_e825 * var_dfsq_dn5) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_db0 - ((-((assign770_e825 * var_dfsq_db0) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))), (var_vpo_db1 - ((-((assign770_e825 * var_dfsq_db1) / (var_dfsq * var_dfsq))) / (2.0 * assign770_e828))),)
    } else {
        (var_vpoe, var_vpoe_dn0, var_vpoe_dn1, var_vpoe_dn2, var_vpoe_dn3, var_vpoe_dn4, var_vpoe_dn5, var_vpoe_db0, var_vpoe_db1,)
    }
};
        var_vpoe = assign770_e831;
        var_vpoe_dn0 = assign770_e831_d_n0;
        var_vpoe_dn1 = assign770_e831_d_n1;
        var_vpoe_dn2 = assign770_e831_d_n2;
        var_vpoe_dn3 = assign770_e831_d_n3;
        var_vpoe_dn4 = assign770_e831_d_n4;
        var_vpoe_dn5 = assign770_e831_d_n5;
        var_vpoe_db0 = assign770_e831_d_b0;
        var_vpoe_db1 = assign770_e831_d_b1;
        var_vpoe_rv = 0.0;
        var_vpoe_rdn0 = 0.0;
        var_vpoe_rdn1 = 0.0;
        var_vpoe_rdn2 = 0.0;
        var_vpoe_rdn3 = 0.0;
        var_vpoe_rdn4 = 0.0;
        var_vpoe_rdn5 = 0.0;
        var_vpoe_rdb0 = 0.0;
        var_vpoe_rdb1 = 0.0;

        let (assign790_e846, assign790_e846_d_n0, assign790_e846_d_n1, assign790_e846_d_n2, assign790_e846_d_n3, assign790_e846_d_n4, assign790_e846_d_n5, assign790_e846_d_b0, assign790_e846_d_b1,) = {
    if ((var_guard117 == 0.0) && (var_guard118 == 0.0)) {
        (var_vpo, var_vpo_dn0, var_vpo_dn1, var_vpo_dn2, var_vpo_dn3, var_vpo_dn4, var_vpo_dn5, var_vpo_db0, var_vpo_db1,)
    } else {
        (var_vpoe, var_vpoe_dn0, var_vpoe_dn1, var_vpoe_dn2, var_vpoe_dn3, var_vpoe_dn4, var_vpoe_dn5, var_vpoe_db0, var_vpoe_db1,)
    }
};
        var_vpoe = assign790_e846;
        var_vpoe_dn0 = assign790_e846_d_n0;
        var_vpoe_dn1 = assign790_e846_d_n1;
        var_vpoe_dn2 = assign790_e846_d_n2;
        var_vpoe_dn3 = assign790_e846_d_n3;
        var_vpoe_dn4 = assign790_e846_d_n4;
        var_vpoe_dn5 = assign790_e846_d_n5;
        var_vpoe_db0 = assign790_e846_d_b0;
        var_vpoe_db1 = assign790_e846_d_b1;
        var_vpoe_rv = 0.0;
        var_vpoe_rdn0 = 0.0;
        var_vpoe_rdn1 = 0.0;
        var_vpoe_rdn2 = 0.0;
        var_vpoe_rdn3 = 0.0;
        var_vpoe_rdn4 = 0.0;
        var_vpoe_rdn5 = 0.0;
        var_vpoe_rdb0 = 0.0;
        var_vpoe_rdb1 = 0.0;

        let assign820_e864: f64 = if p.p63 > 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign820_e864;
        var_guard119_dn0 = 0.0;
        var_guard119_dn1 = 0.0;
        var_guard119_dn2 = 0.0;
        var_guard119_dn3 = 0.0;
        var_guard119_dn4 = 0.0;
        var_guard119_dn5 = 0.0;
        var_guard119_db0 = 0.0;
        var_guard119_db1 = 0.0;
        var_guard119_rv = 0.0;
        var_guard119_rdn0 = 0.0;
        var_guard119_rdn1 = 0.0;
        var_guard119_rdn2 = 0.0;
        var_guard119_rdn3 = 0.0;
        var_guard119_rdn4 = 0.0;
        var_guard119_rdn5 = 0.0;
        var_guard119_rdb0 = 0.0;
        var_guard119_rdb1 = 0.0;

        *var_df_slot = var_df;
        *var_df_db0_slot = var_df_db0;
        *var_df_db1_slot = var_df_db1;
        *var_df_dn0_slot = var_df_dn0;
        *var_df_dn1_slot = var_df_dn1;
        *var_df_dn2_slot = var_df_dn2;
        *var_df_dn3_slot = var_df_dn3;
        *var_df_dn4_slot = var_df_dn4;
        *var_df_dn5_slot = var_df_dn5;
        *var_df_rdb0_slot = var_df_rdb0;
        *var_df_rdb1_slot = var_df_rdb1;
        *var_df_rdn0_slot = var_df_rdn0;
        *var_df_rdn1_slot = var_df_rdn1;
        *var_df_rdn2_slot = var_df_rdn2;
        *var_df_rdn3_slot = var_df_rdn3;
        *var_df_rdn4_slot = var_df_rdn4;
        *var_df_rdn5_slot = var_df_rdn5;
        *var_df_rv_slot = var_df_rv;
        *var_dfmin_slot = var_dfmin;
        *var_dfmin_db0_slot = var_dfmin_db0;
        *var_dfmin_db1_slot = var_dfmin_db1;
        *var_dfmin_dn0_slot = var_dfmin_dn0;
        *var_dfmin_dn1_slot = var_dfmin_dn1;
        *var_dfmin_dn2_slot = var_dfmin_dn2;
        *var_dfmin_dn3_slot = var_dfmin_dn3;
        *var_dfmin_dn4_slot = var_dfmin_dn4;
        *var_dfmin_dn5_slot = var_dfmin_dn5;
        *var_dfmin_rdb0_slot = var_dfmin_rdb0;
        *var_dfmin_rdb1_slot = var_dfmin_rdb1;
        *var_dfmin_rdn0_slot = var_dfmin_rdn0;
        *var_dfmin_rdn1_slot = var_dfmin_rdn1;
        *var_dfmin_rdn2_slot = var_dfmin_rdn2;
        *var_dfmin_rdn3_slot = var_dfmin_rdn3;
        *var_dfmin_rdn4_slot = var_dfmin_rdn4;
        *var_dfmin_rdn5_slot = var_dfmin_rdn5;
        *var_dfmin_rv_slot = var_dfmin_rv;
        *var_dfsq_slot = var_dfsq;
        *var_dfsq_db0_slot = var_dfsq_db0;
        *var_dfsq_db1_slot = var_dfsq_db1;
        *var_dfsq_dn0_slot = var_dfsq_dn0;
        *var_dfsq_dn1_slot = var_dfsq_dn1;
        *var_dfsq_dn2_slot = var_dfsq_dn2;
        *var_dfsq_dn3_slot = var_dfsq_dn3;
        *var_dfsq_dn4_slot = var_dfsq_dn4;
        *var_dfsq_dn5_slot = var_dfsq_dn5;
        *var_dfsq_rdb0_slot = var_dfsq_rdb0;
        *var_dfsq_rdb1_slot = var_dfsq_rdb1;
        *var_dfsq_rdn0_slot = var_dfsq_rdn0;
        *var_dfsq_rdn1_slot = var_dfsq_rdn1;
        *var_dfsq_rdn2_slot = var_dfsq_rdn2;
        *var_dfsq_rdn3_slot = var_dfsq_rdn3;
        *var_dfsq_rdn4_slot = var_dfsq_rdn4;
        *var_dfsq_rdn5_slot = var_dfsq_rdn5;
        *var_dfsq_rv_slot = var_dfsq_rv;
        *var_dp_i_slot = var_dp_i;
        *var_dp_i_db0_slot = var_dp_i_db0;
        *var_dp_i_db1_slot = var_dp_i_db1;
        *var_dp_i_dn0_slot = var_dp_i_dn0;
        *var_dp_i_dn1_slot = var_dp_i_dn1;
        *var_dp_i_dn2_slot = var_dp_i_dn2;
        *var_dp_i_dn3_slot = var_dp_i_dn3;
        *var_dp_i_dn4_slot = var_dp_i_dn4;
        *var_dp_i_dn5_slot = var_dp_i_dn5;
        *var_dp_i_rdb0_slot = var_dp_i_rdb0;
        *var_dp_i_rdb1_slot = var_dp_i_rdb1;
        *var_dp_i_rdn0_slot = var_dp_i_rdn0;
        *var_dp_i_rdn1_slot = var_dp_i_rdn1;
        *var_dp_i_rdn2_slot = var_dp_i_rdn2;
        *var_dp_i_rdn3_slot = var_dp_i_rdn3;
        *var_dp_i_rdn4_slot = var_dp_i_rdn4;
        *var_dp_i_rdn5_slot = var_dp_i_rdn5;
        *var_dp_i_rv_slot = var_dp_i_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_db0_slot = var_guard116_db0;
        *var_guard116_db1_slot = var_guard116_db1;
        *var_guard116_dn0_slot = var_guard116_dn0;
        *var_guard116_dn1_slot = var_guard116_dn1;
        *var_guard116_dn2_slot = var_guard116_dn2;
        *var_guard116_dn3_slot = var_guard116_dn3;
        *var_guard116_dn4_slot = var_guard116_dn4;
        *var_guard116_dn5_slot = var_guard116_dn5;
        *var_guard116_rdb0_slot = var_guard116_rdb0;
        *var_guard116_rdb1_slot = var_guard116_rdb1;
        *var_guard116_rdn0_slot = var_guard116_rdn0;
        *var_guard116_rdn1_slot = var_guard116_rdn1;
        *var_guard116_rdn2_slot = var_guard116_rdn2;
        *var_guard116_rdn3_slot = var_guard116_rdn3;
        *var_guard116_rdn4_slot = var_guard116_rdn4;
        *var_guard116_rdn5_slot = var_guard116_rdn5;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_db0_slot = var_guard117_db0;
        *var_guard117_db1_slot = var_guard117_db1;
        *var_guard117_dn0_slot = var_guard117_dn0;
        *var_guard117_dn1_slot = var_guard117_dn1;
        *var_guard117_dn2_slot = var_guard117_dn2;
        *var_guard117_dn3_slot = var_guard117_dn3;
        *var_guard117_dn4_slot = var_guard117_dn4;
        *var_guard117_dn5_slot = var_guard117_dn5;
        *var_guard117_rdb0_slot = var_guard117_rdb0;
        *var_guard117_rdb1_slot = var_guard117_rdb1;
        *var_guard117_rdn0_slot = var_guard117_rdn0;
        *var_guard117_rdn1_slot = var_guard117_rdn1;
        *var_guard117_rdn2_slot = var_guard117_rdn2;
        *var_guard117_rdn3_slot = var_guard117_rdn3;
        *var_guard117_rdn4_slot = var_guard117_rdn4;
        *var_guard117_rdn5_slot = var_guard117_rdn5;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_db0_slot = var_guard118_db0;
        *var_guard118_db1_slot = var_guard118_db1;
        *var_guard118_dn0_slot = var_guard118_dn0;
        *var_guard118_dn1_slot = var_guard118_dn1;
        *var_guard118_dn2_slot = var_guard118_dn2;
        *var_guard118_dn3_slot = var_guard118_dn3;
        *var_guard118_dn4_slot = var_guard118_dn4;
        *var_guard118_dn5_slot = var_guard118_dn5;
        *var_guard118_rdb0_slot = var_guard118_rdb0;
        *var_guard118_rdb1_slot = var_guard118_rdb1;
        *var_guard118_rdn0_slot = var_guard118_rdn0;
        *var_guard118_rdn1_slot = var_guard118_rdn1;
        *var_guard118_rdn2_slot = var_guard118_rdn2;
        *var_guard118_rdn3_slot = var_guard118_rdn3;
        *var_guard118_rdn4_slot = var_guard118_rdn4;
        *var_guard118_rdn5_slot = var_guard118_rdn5;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_db0_slot = var_guard119_db0;
        *var_guard119_db1_slot = var_guard119_db1;
        *var_guard119_dn0_slot = var_guard119_dn0;
        *var_guard119_dn1_slot = var_guard119_dn1;
        *var_guard119_dn2_slot = var_guard119_dn2;
        *var_guard119_dn3_slot = var_guard119_dn3;
        *var_guard119_dn4_slot = var_guard119_dn4;
        *var_guard119_dn5_slot = var_guard119_dn5;
        *var_guard119_rdb0_slot = var_guard119_rdb0;
        *var_guard119_rdb1_slot = var_guard119_rdb1;
        *var_guard119_rdn0_slot = var_guard119_rdn0;
        *var_guard119_rdn1_slot = var_guard119_rdn1;
        *var_guard119_rdn2_slot = var_guard119_rdn2;
        *var_guard119_rdn3_slot = var_guard119_rdn3;
        *var_guard119_rdn4_slot = var_guard119_rdn4;
        *var_guard119_rdn5_slot = var_guard119_rdn5;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_il_dple_slot = var_il_dple;
        *var_il_dple_db0_slot = var_il_dple_db0;
        *var_il_dple_db1_slot = var_il_dple_db1;
        *var_il_dple_dn0_slot = var_il_dple_dn0;
        *var_il_dple_dn1_slot = var_il_dple_dn1;
        *var_il_dple_dn2_slot = var_il_dple_dn2;
        *var_il_dple_dn3_slot = var_il_dple_dn3;
        *var_il_dple_dn4_slot = var_il_dple_dn4;
        *var_il_dple_dn5_slot = var_il_dple_dn5;
        *var_il_dple_rdb0_slot = var_il_dple_rdb0;
        *var_il_dple_rdb1_slot = var_il_dple_rdb1;
        *var_il_dple_rdn0_slot = var_il_dple_rdn0;
        *var_il_dple_rdn1_slot = var_il_dple_rdn1;
        *var_il_dple_rdn2_slot = var_il_dple_rdn2;
        *var_il_dple_rdn3_slot = var_il_dple_rdn3;
        *var_il_dple_rdn4_slot = var_il_dple_rdn4;
        *var_il_dple_rdn5_slot = var_il_dple_rdn5;
        *var_il_dple_rv_slot = var_il_dple_rv;
        *var_iw_dpwe_slot = var_iw_dpwe;
        *var_iw_dpwe_db0_slot = var_iw_dpwe_db0;
        *var_iw_dpwe_db1_slot = var_iw_dpwe_db1;
        *var_iw_dpwe_dn0_slot = var_iw_dpwe_dn0;
        *var_iw_dpwe_dn1_slot = var_iw_dpwe_dn1;
        *var_iw_dpwe_dn2_slot = var_iw_dpwe_dn2;
        *var_iw_dpwe_dn3_slot = var_iw_dpwe_dn3;
        *var_iw_dpwe_dn4_slot = var_iw_dpwe_dn4;
        *var_iw_dpwe_dn5_slot = var_iw_dpwe_dn5;
        *var_iw_dpwe_rdb0_slot = var_iw_dpwe_rdb0;
        *var_iw_dpwe_rdb1_slot = var_iw_dpwe_rdb1;
        *var_iw_dpwe_rdn0_slot = var_iw_dpwe_rdn0;
        *var_iw_dpwe_rdn1_slot = var_iw_dpwe_rdn1;
        *var_iw_dpwe_rdn2_slot = var_iw_dpwe_rdn2;
        *var_iw_dpwe_rdn3_slot = var_iw_dpwe_rdn3;
        *var_iw_dpwe_rdn4_slot = var_iw_dpwe_rdn4;
        *var_iw_dpwe_rdn5_slot = var_iw_dpwe_rdn5;
        *var_iw_dpwe_rv_slot = var_iw_dpwe_rv;
        *var_len_slot = var_len;
        *var_len_db0_slot = var_len_db0;
        *var_len_db1_slot = var_len_db1;
        *var_len_dn0_slot = var_len_dn0;
        *var_len_dn1_slot = var_len_dn1;
        *var_len_dn2_slot = var_len_dn2;
        *var_len_dn3_slot = var_len_dn3;
        *var_len_dn4_slot = var_len_dn4;
        *var_len_dn5_slot = var_len_dn5;
        *var_len_rdb0_slot = var_len_rdb0;
        *var_len_rdb1_slot = var_len_rdb1;
        *var_len_rdn0_slot = var_len_rdn0;
        *var_len_rdn1_slot = var_len_rdn1;
        *var_len_rdn2_slot = var_len_rdn2;
        *var_len_rdn3_slot = var_len_rdn3;
        *var_len_rdn4_slot = var_len_rdn4;
        *var_len_rdn5_slot = var_len_rdn5;
        *var_len_rv_slot = var_len_rv;
        *var_vpo_slot = var_vpo;
        *var_vpo_db0_slot = var_vpo_db0;
        *var_vpo_db1_slot = var_vpo_db1;
        *var_vpo_dn0_slot = var_vpo_dn0;
        *var_vpo_dn1_slot = var_vpo_dn1;
        *var_vpo_dn2_slot = var_vpo_dn2;
        *var_vpo_dn3_slot = var_vpo_dn3;
        *var_vpo_dn4_slot = var_vpo_dn4;
        *var_vpo_dn5_slot = var_vpo_dn5;
        *var_vpo_rdb0_slot = var_vpo_rdb0;
        *var_vpo_rdb1_slot = var_vpo_rdb1;
        *var_vpo_rdn0_slot = var_vpo_rdn0;
        *var_vpo_rdn1_slot = var_vpo_rdn1;
        *var_vpo_rdn2_slot = var_vpo_rdn2;
        *var_vpo_rdn3_slot = var_vpo_rdn3;
        *var_vpo_rdn4_slot = var_vpo_rdn4;
        *var_vpo_rdn5_slot = var_vpo_rdn5;
        *var_vpo_rv_slot = var_vpo_rv;
        *var_vpoe_slot = var_vpoe;
        *var_vpoe_db0_slot = var_vpoe_db0;
        *var_vpoe_db1_slot = var_vpoe_db1;
        *var_vpoe_dn0_slot = var_vpoe_dn0;
        *var_vpoe_dn1_slot = var_vpoe_dn1;
        *var_vpoe_dn2_slot = var_vpoe_dn2;
        *var_vpoe_dn3_slot = var_vpoe_dn3;
        *var_vpoe_dn4_slot = var_vpoe_dn4;
        *var_vpoe_dn5_slot = var_vpoe_dn5;
        *var_vpoe_rdb0_slot = var_vpoe_rdb0;
        *var_vpoe_rdb1_slot = var_vpoe_rdb1;
        *var_vpoe_rdn0_slot = var_vpoe_rdn0;
        *var_vpoe_rdn1_slot = var_vpoe_rdn1;
        *var_vpoe_rdn2_slot = var_vpoe_rdn2;
        *var_vpoe_rdn3_slot = var_vpoe_rdn3;
        *var_vpoe_rdn4_slot = var_vpoe_rdn4;
        *var_vpoe_rdn5_slot = var_vpoe_rdn5;
        *var_vpoe_rv_slot = var_vpoe_rv;
        *var_wid_slot = var_wid;
        *var_wid_db0_slot = var_wid_db0;
        *var_wid_db1_slot = var_wid_db1;
        *var_wid_dn0_slot = var_wid_dn0;
        *var_wid_dn1_slot = var_wid_dn1;
        *var_wid_dn2_slot = var_wid_dn2;
        *var_wid_dn3_slot = var_wid_dn3;
        *var_wid_dn4_slot = var_wid_dn4;
        *var_wid_dn5_slot = var_wid_dn5;
        *var_wid_rdb0_slot = var_wid_rdb0;
        *var_wid_rdb1_slot = var_wid_rdb1;
        *var_wid_rdn0_slot = var_wid_rdn0;
        *var_wid_rdn1_slot = var_wid_rdn1;
        *var_wid_rdn2_slot = var_wid_rdn2;
        *var_wid_rdn3_slot = var_wid_rdn3;
        *var_wid_rdn4_slot = var_wid_rdn4;
        *var_wid_rdn5_slot = var_wid_rdn5;
        *var_wid_rv_slot = var_wid_rv;
    }

    pub(super) fn stamp_reactive_block_3(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_a1_um2: f64,
        var_a1_um2_db0: f64,
        var_a1_um2_db1: f64,
        var_a1_um2_dn0: f64,
        var_a1_um2_dn1: f64,
        var_a1_um2_dn2: f64,
        var_a1_um2_dn3: f64,
        var_a1_um2_dn4: f64,
        var_a1_um2_dn5: f64,
        var_a2_um2: f64,
        var_a2_um2_db0: f64,
        var_a2_um2_db1: f64,
        var_a2_um2_dn0: f64,
        var_a2_um2_dn1: f64,
        var_a2_um2_dn2: f64,
        var_a2_um2_dn3: f64,
        var_a2_um2_dn4: f64,
        var_a2_um2_dn5: f64,
        var_a_um2: f64,
        var_a_um2_db0: f64,
        var_a_um2_db1: f64,
        var_a_um2_dn0: f64,
        var_a_um2_dn1: f64,
        var_a_um2_dn2: f64,
        var_a_um2_dn3: f64,
        var_a_um2_dn4: f64,
        var_a_um2_dn5: f64,
        var_guard119: f64,
        var_leff_um: f64,
        var_leff_um_db0: f64,
        var_leff_um_db1: f64,
        var_leff_um_dn0: f64,
        var_leff_um_dn1: f64,
        var_leff_um_dn2: f64,
        var_leff_um_dn3: f64,
        var_leff_um_dn4: f64,
        var_leff_um_dn5: f64,
        var_p1_um: f64,
        var_p1_um_db0: f64,
        var_p1_um_db1: f64,
        var_p1_um_dn0: f64,
        var_p1_um_dn1: f64,
        var_p1_um_dn2: f64,
        var_p1_um_dn3: f64,
        var_p1_um_dn4: f64,
        var_p1_um_dn5: f64,
        var_p2_um: f64,
        var_p2_um_db0: f64,
        var_p2_um_db1: f64,
        var_p2_um_dn0: f64,
        var_p2_um_dn1: f64,
        var_p2_um_dn2: f64,
        var_p2_um_dn3: f64,
        var_p2_um_dn4: f64,
        var_p2_um_dn5: f64,
        var_p_um: f64,
        var_p_um_db0: f64,
        var_p_um_db1: f64,
        var_p_um_dn0: f64,
        var_p_um_dn1: f64,
        var_p_um_dn2: f64,
        var_p_um_dn3: f64,
        var_p_um_dn4: f64,
        var_p_um_dn5: f64,
        var_phi_t0: f64,
        var_phi_t0_db0: f64,
        var_phi_t0_db1: f64,
        var_phi_t0_dn0: f64,
        var_phi_t0_dn1: f64,
        var_phi_t0_dn2: f64,
        var_phi_t0_dn3: f64,
        var_phi_t0_dn4: f64,
        var_phi_t0_dn5: f64,
        var_weff_um: f64,
        var_weff_um_db0: f64,
        var_weff_um_db1: f64,
        var_weff_um_dn0: f64,
        var_weff_um_dn1: f64,
        var_weff_um_dn2: f64,
        var_weff_um_dn3: f64,
        var_weff_um_dn4: f64,
        var_weff_um_dn5: f64,
        var_cf1_slot: &mut f64,
        var_cf1_db0_slot: &mut f64,
        var_cf1_db1_slot: &mut f64,
        var_cf1_dn0_slot: &mut f64,
        var_cf1_dn1_slot: &mut f64,
        var_cf1_dn2_slot: &mut f64,
        var_cf1_dn3_slot: &mut f64,
        var_cf1_dn4_slot: &mut f64,
        var_cf1_dn5_slot: &mut f64,
        var_cf1_rdb0_slot: &mut f64,
        var_cf1_rdb1_slot: &mut f64,
        var_cf1_rdn0_slot: &mut f64,
        var_cf1_rdn1_slot: &mut f64,
        var_cf1_rdn2_slot: &mut f64,
        var_cf1_rdn3_slot: &mut f64,
        var_cf1_rdn4_slot: &mut f64,
        var_cf1_rdn5_slot: &mut f64,
        var_cf1_rv_slot: &mut f64,
        var_cf2_slot: &mut f64,
        var_cf2_db0_slot: &mut f64,
        var_cf2_db1_slot: &mut f64,
        var_cf2_dn0_slot: &mut f64,
        var_cf2_dn1_slot: &mut f64,
        var_cf2_dn2_slot: &mut f64,
        var_cf2_dn3_slot: &mut f64,
        var_cf2_dn4_slot: &mut f64,
        var_cf2_dn5_slot: &mut f64,
        var_cf2_rdb0_slot: &mut f64,
        var_cf2_rdb1_slot: &mut f64,
        var_cf2_rdn0_slot: &mut f64,
        var_cf2_rdn1_slot: &mut f64,
        var_cf2_rdn2_slot: &mut f64,
        var_cf2_rdn3_slot: &mut f64,
        var_cf2_rdn4_slot: &mut f64,
        var_cf2_rdn5_slot: &mut f64,
        var_cf2_rv_slot: &mut f64,
        var_cj1_slot: &mut f64,
        var_cj1_db0_slot: &mut f64,
        var_cj1_db1_slot: &mut f64,
        var_cj1_dn0_slot: &mut f64,
        var_cj1_dn1_slot: &mut f64,
        var_cj1_dn2_slot: &mut f64,
        var_cj1_dn3_slot: &mut f64,
        var_cj1_dn4_slot: &mut f64,
        var_cj1_dn5_slot: &mut f64,
        var_cj1_rdb0_slot: &mut f64,
        var_cj1_rdb1_slot: &mut f64,
        var_cj1_rdn0_slot: &mut f64,
        var_cj1_rdn1_slot: &mut f64,
        var_cj1_rdn2_slot: &mut f64,
        var_cj1_rdn3_slot: &mut f64,
        var_cj1_rdn4_slot: &mut f64,
        var_cj1_rdn5_slot: &mut f64,
        var_cj1_rv_slot: &mut f64,
        var_cj2_slot: &mut f64,
        var_cj2_db0_slot: &mut f64,
        var_cj2_db1_slot: &mut f64,
        var_cj2_dn0_slot: &mut f64,
        var_cj2_dn1_slot: &mut f64,
        var_cj2_dn2_slot: &mut f64,
        var_cj2_dn3_slot: &mut f64,
        var_cj2_dn4_slot: &mut f64,
        var_cj2_dn5_slot: &mut f64,
        var_cj2_rdb0_slot: &mut f64,
        var_cj2_rdb1_slot: &mut f64,
        var_cj2_rdn0_slot: &mut f64,
        var_cj2_rdn1_slot: &mut f64,
        var_cj2_rdn2_slot: &mut f64,
        var_cj2_rdn3_slot: &mut f64,
        var_cj2_rdn4_slot: &mut f64,
        var_cj2_rdn5_slot: &mut f64,
        var_cj2_rv_slot: &mut f64,
        var_cth_slot: &mut f64,
        var_cth_db0_slot: &mut f64,
        var_cth_db1_slot: &mut f64,
        var_cth_dn0_slot: &mut f64,
        var_cth_dn1_slot: &mut f64,
        var_cth_dn2_slot: &mut f64,
        var_cth_dn3_slot: &mut f64,
        var_cth_dn4_slot: &mut f64,
        var_cth_dn5_slot: &mut f64,
        var_cth_rdb0_slot: &mut f64,
        var_cth_rdb1_slot: &mut f64,
        var_cth_rdn0_slot: &mut f64,
        var_cth_rdn1_slot: &mut f64,
        var_cth_rdn2_slot: &mut f64,
        var_cth_rdn3_slot: &mut f64,
        var_cth_rdn4_slot: &mut f64,
        var_cth_rdn5_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_dt_et_slot: &mut f64,
        var_dt_et_db0_slot: &mut f64,
        var_dt_et_db1_slot: &mut f64,
        var_dt_et_dn0_slot: &mut f64,
        var_dt_et_dn1_slot: &mut f64,
        var_dt_et_dn2_slot: &mut f64,
        var_dt_et_dn3_slot: &mut f64,
        var_dt_et_dn4_slot: &mut f64,
        var_dt_et_dn5_slot: &mut f64,
        var_dt_et_rdb0_slot: &mut f64,
        var_dt_et_rdb1_slot: &mut f64,
        var_dt_et_rdn0_slot: &mut f64,
        var_dt_et_rdn1_slot: &mut f64,
        var_dt_et_rdn2_slot: &mut f64,
        var_dt_et_rdn3_slot: &mut f64,
        var_dt_et_rdn4_slot: &mut f64,
        var_dt_et_rdn5_slot: &mut f64,
        var_dt_et_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_db0_slot: &mut f64,
        var_guard120_db1_slot: &mut f64,
        var_guard120_dn0_slot: &mut f64,
        var_guard120_dn1_slot: &mut f64,
        var_guard120_dn2_slot: &mut f64,
        var_guard120_dn3_slot: &mut f64,
        var_guard120_dn4_slot: &mut f64,
        var_guard120_dn5_slot: &mut f64,
        var_guard120_rdb0_slot: &mut f64,
        var_guard120_rdb1_slot: &mut f64,
        var_guard120_rdn0_slot: &mut f64,
        var_guard120_rdn1_slot: &mut f64,
        var_guard120_rdn2_slot: &mut f64,
        var_guard120_rdn3_slot: &mut f64,
        var_guard120_rdn4_slot: &mut f64,
        var_guard120_rdn5_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_db0_slot: &mut f64,
        var_guard124_db1_slot: &mut f64,
        var_guard124_dn0_slot: &mut f64,
        var_guard124_dn1_slot: &mut f64,
        var_guard124_dn2_slot: &mut f64,
        var_guard124_dn3_slot: &mut f64,
        var_guard124_dn4_slot: &mut f64,
        var_guard124_dn5_slot: &mut f64,
        var_guard124_rdb0_slot: &mut f64,
        var_guard124_rdb1_slot: &mut f64,
        var_guard124_rdn0_slot: &mut f64,
        var_guard124_rdn1_slot: &mut f64,
        var_guard124_rdn2_slot: &mut f64,
        var_guard124_rdn3_slot: &mut f64,
        var_guard124_rdn4_slot: &mut f64,
        var_guard124_rdn5_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_db0_slot: &mut f64,
        var_guard125_db1_slot: &mut f64,
        var_guard125_dn0_slot: &mut f64,
        var_guard125_dn1_slot: &mut f64,
        var_guard125_dn2_slot: &mut f64,
        var_guard125_dn3_slot: &mut f64,
        var_guard125_dn4_slot: &mut f64,
        var_guard125_dn5_slot: &mut f64,
        var_guard125_rdb0_slot: &mut f64,
        var_guard125_rdb1_slot: &mut f64,
        var_guard125_rdn0_slot: &mut f64,
        var_guard125_rdn1_slot: &mut f64,
        var_guard125_rdn2_slot: &mut f64,
        var_guard125_rdn3_slot: &mut f64,
        var_guard125_rdn4_slot: &mut f64,
        var_guard125_rdn5_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_nsteff_slot: &mut f64,
        var_nsteff_db0_slot: &mut f64,
        var_nsteff_db1_slot: &mut f64,
        var_nsteff_dn0_slot: &mut f64,
        var_nsteff_dn1_slot: &mut f64,
        var_nsteff_dn2_slot: &mut f64,
        var_nsteff_dn3_slot: &mut f64,
        var_nsteff_dn4_slot: &mut f64,
        var_nsteff_dn5_slot: &mut f64,
        var_nsteff_rdb0_slot: &mut f64,
        var_nsteff_rdb1_slot: &mut f64,
        var_nsteff_rdn0_slot: &mut f64,
        var_nsteff_rdn1_slot: &mut f64,
        var_nsteff_rdn2_slot: &mut f64,
        var_nsteff_rdn3_slot: &mut f64,
        var_nsteff_rdn4_slot: &mut f64,
        var_nsteff_rdn5_slot: &mut f64,
        var_nsteff_rv_slot: &mut f64,
        var_tc1e_slot: &mut f64,
        var_tc1e_db0_slot: &mut f64,
        var_tc1e_db1_slot: &mut f64,
        var_tc1e_dn0_slot: &mut f64,
        var_tc1e_dn1_slot: &mut f64,
        var_tc1e_dn2_slot: &mut f64,
        var_tc1e_dn3_slot: &mut f64,
        var_tc1e_dn4_slot: &mut f64,
        var_tc1e_dn5_slot: &mut f64,
        var_tc1e_rdb0_slot: &mut f64,
        var_tc1e_rdb1_slot: &mut f64,
        var_tc1e_rdn0_slot: &mut f64,
        var_tc1e_rdn1_slot: &mut f64,
        var_tc1e_rdn2_slot: &mut f64,
        var_tc1e_rdn3_slot: &mut f64,
        var_tc1e_rdn4_slot: &mut f64,
        var_tc1e_rdn5_slot: &mut f64,
        var_tc1e_rv_slot: &mut f64,
        var_tc2e_slot: &mut f64,
        var_tc2e_db0_slot: &mut f64,
        var_tc2e_db1_slot: &mut f64,
        var_tc2e_dn0_slot: &mut f64,
        var_tc2e_dn1_slot: &mut f64,
        var_tc2e_dn2_slot: &mut f64,
        var_tc2e_dn3_slot: &mut f64,
        var_tc2e_dn4_slot: &mut f64,
        var_tc2e_dn5_slot: &mut f64,
        var_tc2e_rdb0_slot: &mut f64,
        var_tc2e_rdb1_slot: &mut f64,
        var_tc2e_rdn0_slot: &mut f64,
        var_tc2e_rdn1_slot: &mut f64,
        var_tc2e_rdn2_slot: &mut f64,
        var_tc2e_rdn3_slot: &mut f64,
        var_tc2e_rdn4_slot: &mut f64,
        var_tc2e_rdn5_slot: &mut f64,
        var_tc2e_rv_slot: &mut f64,
        var_tdevc_slot: &mut f64,
        var_tdevc_db0_slot: &mut f64,
        var_tdevc_db1_slot: &mut f64,
        var_tdevc_dn0_slot: &mut f64,
        var_tdevc_dn1_slot: &mut f64,
        var_tdevc_dn2_slot: &mut f64,
        var_tdevc_dn3_slot: &mut f64,
        var_tdevc_dn4_slot: &mut f64,
        var_tdevc_dn5_slot: &mut f64,
        var_tdevc_rdb0_slot: &mut f64,
        var_tdevc_rdb1_slot: &mut f64,
        var_tdevc_rdn0_slot: &mut f64,
        var_tdevc_rdn1_slot: &mut f64,
        var_tdevc_rdn2_slot: &mut f64,
        var_tdevc_rdn3_slot: &mut f64,
        var_tdevc_rdn4_slot: &mut f64,
        var_tdevc_rdn5_slot: &mut f64,
        var_tdevc_rv_slot: &mut f64,
        var_vc1_slot: &mut f64,
        var_vc1_db0_slot: &mut f64,
        var_vc1_db1_slot: &mut f64,
        var_vc1_dn0_slot: &mut f64,
        var_vc1_dn1_slot: &mut f64,
        var_vc1_dn2_slot: &mut f64,
        var_vc1_dn3_slot: &mut f64,
        var_vc1_dn4_slot: &mut f64,
        var_vc1_dn5_slot: &mut f64,
        var_vc1_rdb0_slot: &mut f64,
        var_vc1_rdb1_slot: &mut f64,
        var_vc1_rdn0_slot: &mut f64,
        var_vc1_rdn1_slot: &mut f64,
        var_vc1_rdn2_slot: &mut f64,
        var_vc1_rdn3_slot: &mut f64,
        var_vc1_rdn4_slot: &mut f64,
        var_vc1_rdn5_slot: &mut f64,
        var_vc1_rv_slot: &mut f64,
        var_vc2_slot: &mut f64,
        var_vc2_db0_slot: &mut f64,
        var_vc2_db1_slot: &mut f64,
        var_vc2_dn0_slot: &mut f64,
        var_vc2_dn1_slot: &mut f64,
        var_vc2_dn2_slot: &mut f64,
        var_vc2_dn3_slot: &mut f64,
        var_vc2_dn4_slot: &mut f64,
        var_vc2_dn5_slot: &mut f64,
        var_vc2_rdb0_slot: &mut f64,
        var_vc2_rdb1_slot: &mut f64,
        var_vc2_rdn0_slot: &mut f64,
        var_vc2_rdn1_slot: &mut f64,
        var_vc2_rdn2_slot: &mut f64,
        var_vc2_rdn3_slot: &mut f64,
        var_vc2_rdn4_slot: &mut f64,
        var_vc2_rdn5_slot: &mut f64,
        var_vc2_rv_slot: &mut f64,
        var_vrb_slot: &mut f64,
        var_vrb_db0_slot: &mut f64,
        var_vrb_db1_slot: &mut f64,
        var_vrb_dn0_slot: &mut f64,
        var_vrb_dn1_slot: &mut f64,
        var_vrb_dn2_slot: &mut f64,
        var_vrb_dn3_slot: &mut f64,
        var_vrb_dn4_slot: &mut f64,
        var_vrb_dn5_slot: &mut f64,
        var_vrb_rdb0_slot: &mut f64,
        var_vrb_rdb1_slot: &mut f64,
        var_vrb_rdn0_slot: &mut f64,
        var_vrb_rdn1_slot: &mut f64,
        var_vrb_rdn2_slot: &mut f64,
        var_vrb_rdn3_slot: &mut f64,
        var_vrb_rdn4_slot: &mut f64,
        var_vrb_rdn5_slot: &mut f64,
        var_vrb_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let mut var_cf1: f64 = *var_cf1_slot;
        let mut var_cf1_db0: f64 = *var_cf1_db0_slot;
        let mut var_cf1_db1: f64 = *var_cf1_db1_slot;
        let mut var_cf1_dn0: f64 = *var_cf1_dn0_slot;
        let mut var_cf1_dn1: f64 = *var_cf1_dn1_slot;
        let mut var_cf1_dn2: f64 = *var_cf1_dn2_slot;
        let mut var_cf1_dn3: f64 = *var_cf1_dn3_slot;
        let mut var_cf1_dn4: f64 = *var_cf1_dn4_slot;
        let mut var_cf1_dn5: f64 = *var_cf1_dn5_slot;
        let mut var_cf1_rdb0: f64 = *var_cf1_rdb0_slot;
        let mut var_cf1_rdb1: f64 = *var_cf1_rdb1_slot;
        let mut var_cf1_rdn0: f64 = *var_cf1_rdn0_slot;
        let mut var_cf1_rdn1: f64 = *var_cf1_rdn1_slot;
        let mut var_cf1_rdn2: f64 = *var_cf1_rdn2_slot;
        let mut var_cf1_rdn3: f64 = *var_cf1_rdn3_slot;
        let mut var_cf1_rdn4: f64 = *var_cf1_rdn4_slot;
        let mut var_cf1_rdn5: f64 = *var_cf1_rdn5_slot;
        let mut var_cf1_rv: f64 = *var_cf1_rv_slot;
        let mut var_cf2: f64 = *var_cf2_slot;
        let mut var_cf2_db0: f64 = *var_cf2_db0_slot;
        let mut var_cf2_db1: f64 = *var_cf2_db1_slot;
        let mut var_cf2_dn0: f64 = *var_cf2_dn0_slot;
        let mut var_cf2_dn1: f64 = *var_cf2_dn1_slot;
        let mut var_cf2_dn2: f64 = *var_cf2_dn2_slot;
        let mut var_cf2_dn3: f64 = *var_cf2_dn3_slot;
        let mut var_cf2_dn4: f64 = *var_cf2_dn4_slot;
        let mut var_cf2_dn5: f64 = *var_cf2_dn5_slot;
        let mut var_cf2_rdb0: f64 = *var_cf2_rdb0_slot;
        let mut var_cf2_rdb1: f64 = *var_cf2_rdb1_slot;
        let mut var_cf2_rdn0: f64 = *var_cf2_rdn0_slot;
        let mut var_cf2_rdn1: f64 = *var_cf2_rdn1_slot;
        let mut var_cf2_rdn2: f64 = *var_cf2_rdn2_slot;
        let mut var_cf2_rdn3: f64 = *var_cf2_rdn3_slot;
        let mut var_cf2_rdn4: f64 = *var_cf2_rdn4_slot;
        let mut var_cf2_rdn5: f64 = *var_cf2_rdn5_slot;
        let mut var_cf2_rv: f64 = *var_cf2_rv_slot;
        let mut var_cj1: f64 = *var_cj1_slot;
        let mut var_cj1_db0: f64 = *var_cj1_db0_slot;
        let mut var_cj1_db1: f64 = *var_cj1_db1_slot;
        let mut var_cj1_dn0: f64 = *var_cj1_dn0_slot;
        let mut var_cj1_dn1: f64 = *var_cj1_dn1_slot;
        let mut var_cj1_dn2: f64 = *var_cj1_dn2_slot;
        let mut var_cj1_dn3: f64 = *var_cj1_dn3_slot;
        let mut var_cj1_dn4: f64 = *var_cj1_dn4_slot;
        let mut var_cj1_dn5: f64 = *var_cj1_dn5_slot;
        let mut var_cj1_rdb0: f64 = *var_cj1_rdb0_slot;
        let mut var_cj1_rdb1: f64 = *var_cj1_rdb1_slot;
        let mut var_cj1_rdn0: f64 = *var_cj1_rdn0_slot;
        let mut var_cj1_rdn1: f64 = *var_cj1_rdn1_slot;
        let mut var_cj1_rdn2: f64 = *var_cj1_rdn2_slot;
        let mut var_cj1_rdn3: f64 = *var_cj1_rdn3_slot;
        let mut var_cj1_rdn4: f64 = *var_cj1_rdn4_slot;
        let mut var_cj1_rdn5: f64 = *var_cj1_rdn5_slot;
        let mut var_cj1_rv: f64 = *var_cj1_rv_slot;
        let mut var_cj2: f64 = *var_cj2_slot;
        let mut var_cj2_db0: f64 = *var_cj2_db0_slot;
        let mut var_cj2_db1: f64 = *var_cj2_db1_slot;
        let mut var_cj2_dn0: f64 = *var_cj2_dn0_slot;
        let mut var_cj2_dn1: f64 = *var_cj2_dn1_slot;
        let mut var_cj2_dn2: f64 = *var_cj2_dn2_slot;
        let mut var_cj2_dn3: f64 = *var_cj2_dn3_slot;
        let mut var_cj2_dn4: f64 = *var_cj2_dn4_slot;
        let mut var_cj2_dn5: f64 = *var_cj2_dn5_slot;
        let mut var_cj2_rdb0: f64 = *var_cj2_rdb0_slot;
        let mut var_cj2_rdb1: f64 = *var_cj2_rdb1_slot;
        let mut var_cj2_rdn0: f64 = *var_cj2_rdn0_slot;
        let mut var_cj2_rdn1: f64 = *var_cj2_rdn1_slot;
        let mut var_cj2_rdn2: f64 = *var_cj2_rdn2_slot;
        let mut var_cj2_rdn3: f64 = *var_cj2_rdn3_slot;
        let mut var_cj2_rdn4: f64 = *var_cj2_rdn4_slot;
        let mut var_cj2_rdn5: f64 = *var_cj2_rdn5_slot;
        let mut var_cj2_rv: f64 = *var_cj2_rv_slot;
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_db0: f64 = *var_cth_db0_slot;
        let mut var_cth_db1: f64 = *var_cth_db1_slot;
        let mut var_cth_dn0: f64 = *var_cth_dn0_slot;
        let mut var_cth_dn1: f64 = *var_cth_dn1_slot;
        let mut var_cth_dn2: f64 = *var_cth_dn2_slot;
        let mut var_cth_dn3: f64 = *var_cth_dn3_slot;
        let mut var_cth_dn4: f64 = *var_cth_dn4_slot;
        let mut var_cth_dn5: f64 = *var_cth_dn5_slot;
        let mut var_cth_rdb0: f64 = *var_cth_rdb0_slot;
        let mut var_cth_rdb1: f64 = *var_cth_rdb1_slot;
        let mut var_cth_rdn0: f64 = *var_cth_rdn0_slot;
        let mut var_cth_rdn1: f64 = *var_cth_rdn1_slot;
        let mut var_cth_rdn2: f64 = *var_cth_rdn2_slot;
        let mut var_cth_rdn3: f64 = *var_cth_rdn3_slot;
        let mut var_cth_rdn4: f64 = *var_cth_rdn4_slot;
        let mut var_cth_rdn5: f64 = *var_cth_rdn5_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_dt_et: f64 = *var_dt_et_slot;
        let mut var_dt_et_db0: f64 = *var_dt_et_db0_slot;
        let mut var_dt_et_db1: f64 = *var_dt_et_db1_slot;
        let mut var_dt_et_dn0: f64 = *var_dt_et_dn0_slot;
        let mut var_dt_et_dn1: f64 = *var_dt_et_dn1_slot;
        let mut var_dt_et_dn2: f64 = *var_dt_et_dn2_slot;
        let mut var_dt_et_dn3: f64 = *var_dt_et_dn3_slot;
        let mut var_dt_et_dn4: f64 = *var_dt_et_dn4_slot;
        let mut var_dt_et_dn5: f64 = *var_dt_et_dn5_slot;
        let mut var_dt_et_rdb0: f64 = *var_dt_et_rdb0_slot;
        let mut var_dt_et_rdb1: f64 = *var_dt_et_rdb1_slot;
        let mut var_dt_et_rdn0: f64 = *var_dt_et_rdn0_slot;
        let mut var_dt_et_rdn1: f64 = *var_dt_et_rdn1_slot;
        let mut var_dt_et_rdn2: f64 = *var_dt_et_rdn2_slot;
        let mut var_dt_et_rdn3: f64 = *var_dt_et_rdn3_slot;
        let mut var_dt_et_rdn4: f64 = *var_dt_et_rdn4_slot;
        let mut var_dt_et_rdn5: f64 = *var_dt_et_rdn5_slot;
        let mut var_dt_et_rv: f64 = *var_dt_et_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_db0: f64 = *var_guard120_db0_slot;
        let mut var_guard120_db1: f64 = *var_guard120_db1_slot;
        let mut var_guard120_dn0: f64 = *var_guard120_dn0_slot;
        let mut var_guard120_dn1: f64 = *var_guard120_dn1_slot;
        let mut var_guard120_dn2: f64 = *var_guard120_dn2_slot;
        let mut var_guard120_dn3: f64 = *var_guard120_dn3_slot;
        let mut var_guard120_dn4: f64 = *var_guard120_dn4_slot;
        let mut var_guard120_dn5: f64 = *var_guard120_dn5_slot;
        let mut var_guard120_rdb0: f64 = *var_guard120_rdb0_slot;
        let mut var_guard120_rdb1: f64 = *var_guard120_rdb1_slot;
        let mut var_guard120_rdn0: f64 = *var_guard120_rdn0_slot;
        let mut var_guard120_rdn1: f64 = *var_guard120_rdn1_slot;
        let mut var_guard120_rdn2: f64 = *var_guard120_rdn2_slot;
        let mut var_guard120_rdn3: f64 = *var_guard120_rdn3_slot;
        let mut var_guard120_rdn4: f64 = *var_guard120_rdn4_slot;
        let mut var_guard120_rdn5: f64 = *var_guard120_rdn5_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_db0: f64 = *var_guard124_db0_slot;
        let mut var_guard124_db1: f64 = *var_guard124_db1_slot;
        let mut var_guard124_dn0: f64 = *var_guard124_dn0_slot;
        let mut var_guard124_dn1: f64 = *var_guard124_dn1_slot;
        let mut var_guard124_dn2: f64 = *var_guard124_dn2_slot;
        let mut var_guard124_dn3: f64 = *var_guard124_dn3_slot;
        let mut var_guard124_dn4: f64 = *var_guard124_dn4_slot;
        let mut var_guard124_dn5: f64 = *var_guard124_dn5_slot;
        let mut var_guard124_rdb0: f64 = *var_guard124_rdb0_slot;
        let mut var_guard124_rdb1: f64 = *var_guard124_rdb1_slot;
        let mut var_guard124_rdn0: f64 = *var_guard124_rdn0_slot;
        let mut var_guard124_rdn1: f64 = *var_guard124_rdn1_slot;
        let mut var_guard124_rdn2: f64 = *var_guard124_rdn2_slot;
        let mut var_guard124_rdn3: f64 = *var_guard124_rdn3_slot;
        let mut var_guard124_rdn4: f64 = *var_guard124_rdn4_slot;
        let mut var_guard124_rdn5: f64 = *var_guard124_rdn5_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_db0: f64 = *var_guard125_db0_slot;
        let mut var_guard125_db1: f64 = *var_guard125_db1_slot;
        let mut var_guard125_dn0: f64 = *var_guard125_dn0_slot;
        let mut var_guard125_dn1: f64 = *var_guard125_dn1_slot;
        let mut var_guard125_dn2: f64 = *var_guard125_dn2_slot;
        let mut var_guard125_dn3: f64 = *var_guard125_dn3_slot;
        let mut var_guard125_dn4: f64 = *var_guard125_dn4_slot;
        let mut var_guard125_dn5: f64 = *var_guard125_dn5_slot;
        let mut var_guard125_rdb0: f64 = *var_guard125_rdb0_slot;
        let mut var_guard125_rdb1: f64 = *var_guard125_rdb1_slot;
        let mut var_guard125_rdn0: f64 = *var_guard125_rdn0_slot;
        let mut var_guard125_rdn1: f64 = *var_guard125_rdn1_slot;
        let mut var_guard125_rdn2: f64 = *var_guard125_rdn2_slot;
        let mut var_guard125_rdn3: f64 = *var_guard125_rdn3_slot;
        let mut var_guard125_rdn4: f64 = *var_guard125_rdn4_slot;
        let mut var_guard125_rdn5: f64 = *var_guard125_rdn5_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_nsteff: f64 = *var_nsteff_slot;
        let mut var_nsteff_db0: f64 = *var_nsteff_db0_slot;
        let mut var_nsteff_db1: f64 = *var_nsteff_db1_slot;
        let mut var_nsteff_dn0: f64 = *var_nsteff_dn0_slot;
        let mut var_nsteff_dn1: f64 = *var_nsteff_dn1_slot;
        let mut var_nsteff_dn2: f64 = *var_nsteff_dn2_slot;
        let mut var_nsteff_dn3: f64 = *var_nsteff_dn3_slot;
        let mut var_nsteff_dn4: f64 = *var_nsteff_dn4_slot;
        let mut var_nsteff_dn5: f64 = *var_nsteff_dn5_slot;
        let mut var_nsteff_rdb0: f64 = *var_nsteff_rdb0_slot;
        let mut var_nsteff_rdb1: f64 = *var_nsteff_rdb1_slot;
        let mut var_nsteff_rdn0: f64 = *var_nsteff_rdn0_slot;
        let mut var_nsteff_rdn1: f64 = *var_nsteff_rdn1_slot;
        let mut var_nsteff_rdn2: f64 = *var_nsteff_rdn2_slot;
        let mut var_nsteff_rdn3: f64 = *var_nsteff_rdn3_slot;
        let mut var_nsteff_rdn4: f64 = *var_nsteff_rdn4_slot;
        let mut var_nsteff_rdn5: f64 = *var_nsteff_rdn5_slot;
        let mut var_nsteff_rv: f64 = *var_nsteff_rv_slot;
        let mut var_tc1e: f64 = *var_tc1e_slot;
        let mut var_tc1e_db0: f64 = *var_tc1e_db0_slot;
        let mut var_tc1e_db1: f64 = *var_tc1e_db1_slot;
        let mut var_tc1e_dn0: f64 = *var_tc1e_dn0_slot;
        let mut var_tc1e_dn1: f64 = *var_tc1e_dn1_slot;
        let mut var_tc1e_dn2: f64 = *var_tc1e_dn2_slot;
        let mut var_tc1e_dn3: f64 = *var_tc1e_dn3_slot;
        let mut var_tc1e_dn4: f64 = *var_tc1e_dn4_slot;
        let mut var_tc1e_dn5: f64 = *var_tc1e_dn5_slot;
        let mut var_tc1e_rdb0: f64 = *var_tc1e_rdb0_slot;
        let mut var_tc1e_rdb1: f64 = *var_tc1e_rdb1_slot;
        let mut var_tc1e_rdn0: f64 = *var_tc1e_rdn0_slot;
        let mut var_tc1e_rdn1: f64 = *var_tc1e_rdn1_slot;
        let mut var_tc1e_rdn2: f64 = *var_tc1e_rdn2_slot;
        let mut var_tc1e_rdn3: f64 = *var_tc1e_rdn3_slot;
        let mut var_tc1e_rdn4: f64 = *var_tc1e_rdn4_slot;
        let mut var_tc1e_rdn5: f64 = *var_tc1e_rdn5_slot;
        let mut var_tc1e_rv: f64 = *var_tc1e_rv_slot;
        let mut var_tc2e: f64 = *var_tc2e_slot;
        let mut var_tc2e_db0: f64 = *var_tc2e_db0_slot;
        let mut var_tc2e_db1: f64 = *var_tc2e_db1_slot;
        let mut var_tc2e_dn0: f64 = *var_tc2e_dn0_slot;
        let mut var_tc2e_dn1: f64 = *var_tc2e_dn1_slot;
        let mut var_tc2e_dn2: f64 = *var_tc2e_dn2_slot;
        let mut var_tc2e_dn3: f64 = *var_tc2e_dn3_slot;
        let mut var_tc2e_dn4: f64 = *var_tc2e_dn4_slot;
        let mut var_tc2e_dn5: f64 = *var_tc2e_dn5_slot;
        let mut var_tc2e_rdb0: f64 = *var_tc2e_rdb0_slot;
        let mut var_tc2e_rdb1: f64 = *var_tc2e_rdb1_slot;
        let mut var_tc2e_rdn0: f64 = *var_tc2e_rdn0_slot;
        let mut var_tc2e_rdn1: f64 = *var_tc2e_rdn1_slot;
        let mut var_tc2e_rdn2: f64 = *var_tc2e_rdn2_slot;
        let mut var_tc2e_rdn3: f64 = *var_tc2e_rdn3_slot;
        let mut var_tc2e_rdn4: f64 = *var_tc2e_rdn4_slot;
        let mut var_tc2e_rdn5: f64 = *var_tc2e_rdn5_slot;
        let mut var_tc2e_rv: f64 = *var_tc2e_rv_slot;
        let mut var_tdevc: f64 = *var_tdevc_slot;
        let mut var_tdevc_db0: f64 = *var_tdevc_db0_slot;
        let mut var_tdevc_db1: f64 = *var_tdevc_db1_slot;
        let mut var_tdevc_dn0: f64 = *var_tdevc_dn0_slot;
        let mut var_tdevc_dn1: f64 = *var_tdevc_dn1_slot;
        let mut var_tdevc_dn2: f64 = *var_tdevc_dn2_slot;
        let mut var_tdevc_dn3: f64 = *var_tdevc_dn3_slot;
        let mut var_tdevc_dn4: f64 = *var_tdevc_dn4_slot;
        let mut var_tdevc_dn5: f64 = *var_tdevc_dn5_slot;
        let mut var_tdevc_rdb0: f64 = *var_tdevc_rdb0_slot;
        let mut var_tdevc_rdb1: f64 = *var_tdevc_rdb1_slot;
        let mut var_tdevc_rdn0: f64 = *var_tdevc_rdn0_slot;
        let mut var_tdevc_rdn1: f64 = *var_tdevc_rdn1_slot;
        let mut var_tdevc_rdn2: f64 = *var_tdevc_rdn2_slot;
        let mut var_tdevc_rdn3: f64 = *var_tdevc_rdn3_slot;
        let mut var_tdevc_rdn4: f64 = *var_tdevc_rdn4_slot;
        let mut var_tdevc_rdn5: f64 = *var_tdevc_rdn5_slot;
        let mut var_tdevc_rv: f64 = *var_tdevc_rv_slot;
        let mut var_vc1: f64 = *var_vc1_slot;
        let mut var_vc1_db0: f64 = *var_vc1_db0_slot;
        let mut var_vc1_db1: f64 = *var_vc1_db1_slot;
        let mut var_vc1_dn0: f64 = *var_vc1_dn0_slot;
        let mut var_vc1_dn1: f64 = *var_vc1_dn1_slot;
        let mut var_vc1_dn2: f64 = *var_vc1_dn2_slot;
        let mut var_vc1_dn3: f64 = *var_vc1_dn3_slot;
        let mut var_vc1_dn4: f64 = *var_vc1_dn4_slot;
        let mut var_vc1_dn5: f64 = *var_vc1_dn5_slot;
        let mut var_vc1_rdb0: f64 = *var_vc1_rdb0_slot;
        let mut var_vc1_rdb1: f64 = *var_vc1_rdb1_slot;
        let mut var_vc1_rdn0: f64 = *var_vc1_rdn0_slot;
        let mut var_vc1_rdn1: f64 = *var_vc1_rdn1_slot;
        let mut var_vc1_rdn2: f64 = *var_vc1_rdn2_slot;
        let mut var_vc1_rdn3: f64 = *var_vc1_rdn3_slot;
        let mut var_vc1_rdn4: f64 = *var_vc1_rdn4_slot;
        let mut var_vc1_rdn5: f64 = *var_vc1_rdn5_slot;
        let mut var_vc1_rv: f64 = *var_vc1_rv_slot;
        let mut var_vc2: f64 = *var_vc2_slot;
        let mut var_vc2_db0: f64 = *var_vc2_db0_slot;
        let mut var_vc2_db1: f64 = *var_vc2_db1_slot;
        let mut var_vc2_dn0: f64 = *var_vc2_dn0_slot;
        let mut var_vc2_dn1: f64 = *var_vc2_dn1_slot;
        let mut var_vc2_dn2: f64 = *var_vc2_dn2_slot;
        let mut var_vc2_dn3: f64 = *var_vc2_dn3_slot;
        let mut var_vc2_dn4: f64 = *var_vc2_dn4_slot;
        let mut var_vc2_dn5: f64 = *var_vc2_dn5_slot;
        let mut var_vc2_rdb0: f64 = *var_vc2_rdb0_slot;
        let mut var_vc2_rdb1: f64 = *var_vc2_rdb1_slot;
        let mut var_vc2_rdn0: f64 = *var_vc2_rdn0_slot;
        let mut var_vc2_rdn1: f64 = *var_vc2_rdn1_slot;
        let mut var_vc2_rdn2: f64 = *var_vc2_rdn2_slot;
        let mut var_vc2_rdn3: f64 = *var_vc2_rdn3_slot;
        let mut var_vc2_rdn4: f64 = *var_vc2_rdn4_slot;
        let mut var_vc2_rdn5: f64 = *var_vc2_rdn5_slot;
        let mut var_vc2_rv: f64 = *var_vc2_rv_slot;
        let mut var_vrb: f64 = *var_vrb_slot;
        let mut var_vrb_db0: f64 = *var_vrb_db0_slot;
        let mut var_vrb_db1: f64 = *var_vrb_db1_slot;
        let mut var_vrb_dn0: f64 = *var_vrb_dn0_slot;
        let mut var_vrb_dn1: f64 = *var_vrb_dn1_slot;
        let mut var_vrb_dn2: f64 = *var_vrb_dn2_slot;
        let mut var_vrb_dn3: f64 = *var_vrb_dn3_slot;
        let mut var_vrb_dn4: f64 = *var_vrb_dn4_slot;
        let mut var_vrb_dn5: f64 = *var_vrb_dn5_slot;
        let mut var_vrb_rdb0: f64 = *var_vrb_rdb0_slot;
        let mut var_vrb_rdb1: f64 = *var_vrb_rdb1_slot;
        let mut var_vrb_rdn0: f64 = *var_vrb_rdn0_slot;
        let mut var_vrb_rdn1: f64 = *var_vrb_rdn1_slot;
        let mut var_vrb_rdn2: f64 = *var_vrb_rdn2_slot;
        let mut var_vrb_rdn3: f64 = *var_vrb_rdn3_slot;
        let mut var_vrb_rdn4: f64 = *var_vrb_rdn4_slot;
        let mut var_vrb_rdn5: f64 = *var_vrb_rdn5_slot;
        let mut var_vrb_rv: f64 = *var_vrb_rv_slot;

        let (assign830_e870, assign830_e870_d_n0, assign830_e870_d_n1, assign830_e870_d_n2, assign830_e870_d_n3, assign830_e870_d_n4, assign830_e870_d_n5, assign830_e870_d_b0, assign830_e870_d_b1,) = {
    if (var_guard119 != 0.0) {
        let assign830_e868: f64 = (p.p46 * var_phi_t0);
        (assign830_e868, (p.p46 * var_phi_t0_dn0), (p.p46 * var_phi_t0_dn1), (p.p46 * var_phi_t0_dn2), (p.p46 * var_phi_t0_dn3), (p.p46 * var_phi_t0_dn4), (p.p46 * var_phi_t0_dn5), (p.p46 * var_phi_t0_db0), (p.p46 * var_phi_t0_db1),)
    } else {
        (var_nsteff, var_nsteff_dn0, var_nsteff_dn1, var_nsteff_dn2, var_nsteff_dn3, var_nsteff_dn4, var_nsteff_dn5, var_nsteff_db0, var_nsteff_db1,)
    }
};
        var_nsteff = assign830_e870;
        var_nsteff_dn0 = assign830_e870_d_n0;
        var_nsteff_dn1 = assign830_e870_d_n1;
        var_nsteff_dn2 = assign830_e870_d_n2;
        var_nsteff_dn3 = assign830_e870_d_n3;
        var_nsteff_dn4 = assign830_e870_d_n4;
        var_nsteff_dn5 = assign830_e870_d_n5;
        var_nsteff_db0 = assign830_e870_d_b0;
        var_nsteff_db1 = assign830_e870_d_b1;
        var_nsteff_rv = 0.0;
        var_nsteff_rdn0 = 0.0;
        var_nsteff_rdn1 = 0.0;
        var_nsteff_rdn2 = 0.0;
        var_nsteff_rdn3 = 0.0;
        var_nsteff_rdn4 = 0.0;
        var_nsteff_rdn5 = 0.0;
        var_nsteff_rdb0 = 0.0;
        var_nsteff_rdb1 = 0.0;

        let assign850_e894: f64 = if p.p63 > 0.0 { 1.0 } else { 0.0 };
        var_guard120 = assign850_e894;
        var_guard120_dn0 = 0.0;
        var_guard120_dn1 = 0.0;
        var_guard120_dn2 = 0.0;
        var_guard120_dn3 = 0.0;
        var_guard120_dn4 = 0.0;
        var_guard120_dn5 = 0.0;
        var_guard120_db0 = 0.0;
        var_guard120_db1 = 0.0;
        var_guard120_rv = 0.0;
        var_guard120_rdn0 = 0.0;
        var_guard120_rdn1 = 0.0;
        var_guard120_rdn2 = 0.0;
        var_guard120_rdn3 = 0.0;
        var_guard120_rdn4 = 0.0;
        var_guard120_rdn5 = 0.0;
        var_guard120_rdb0 = 0.0;
        var_guard120_rdb1 = 0.0;

        let (assign860_e905, assign860_e905_d_n0, assign860_e905_d_n1, assign860_e905_d_n2, assign860_e905_d_n3, assign860_e905_d_n4, assign860_e905_d_n5, assign860_e905_d_b0, assign860_e905_d_b1,) = {
    if ((var_guard119 == 0.0) && (var_guard120 != 0.0)) {
        let assign860_e901: f64 = (2.0 * p.p46);
        let assign860_e903: f64 = (assign860_e901 * var_phi_t0);
        (assign860_e903, (assign860_e901 * var_phi_t0_dn0), (assign860_e901 * var_phi_t0_dn1), (assign860_e901 * var_phi_t0_dn2), (assign860_e901 * var_phi_t0_dn3), (assign860_e901 * var_phi_t0_dn4), (assign860_e901 * var_phi_t0_dn5), (assign860_e901 * var_phi_t0_db0), (assign860_e901 * var_phi_t0_db1),)
    } else {
        (var_nsteff, var_nsteff_dn0, var_nsteff_dn1, var_nsteff_dn2, var_nsteff_dn3, var_nsteff_dn4, var_nsteff_dn5, var_nsteff_db0, var_nsteff_db1,)
    }
};
        var_nsteff = assign860_e905;
        var_nsteff_dn0 = assign860_e905_d_n0;
        var_nsteff_dn1 = assign860_e905_d_n1;
        var_nsteff_dn2 = assign860_e905_d_n2;
        var_nsteff_dn3 = assign860_e905_d_n3;
        var_nsteff_dn4 = assign860_e905_d_n4;
        var_nsteff_dn5 = assign860_e905_d_n5;
        var_nsteff_db0 = assign860_e905_d_b0;
        var_nsteff_db1 = assign860_e905_d_b1;
        var_nsteff_rv = 0.0;
        var_nsteff_rdn0 = 0.0;
        var_nsteff_rdn1 = 0.0;
        var_nsteff_rdn2 = 0.0;
        var_nsteff_rdn3 = 0.0;
        var_nsteff_rdn4 = 0.0;
        var_nsteff_rdn5 = 0.0;
        var_nsteff_rdb0 = 0.0;
        var_nsteff_rdb1 = 0.0;

        let (assign880_e926, assign880_e926_d_n0, assign880_e926_d_n1, assign880_e926_d_n2, assign880_e926_d_n3, assign880_e926_d_n4, assign880_e926_d_n5, assign880_e926_d_b0, assign880_e926_d_b1,) = {
    if ((var_guard119 == 0.0) && (var_guard120 == 0.0)) {
        let assign880_e924: f64 = (p.p46 * var_phi_t0);
        (assign880_e924, (p.p46 * var_phi_t0_dn0), (p.p46 * var_phi_t0_dn1), (p.p46 * var_phi_t0_dn2), (p.p46 * var_phi_t0_dn3), (p.p46 * var_phi_t0_dn4), (p.p46 * var_phi_t0_dn5), (p.p46 * var_phi_t0_db0), (p.p46 * var_phi_t0_db1),)
    } else {
        (var_nsteff, var_nsteff_dn0, var_nsteff_dn1, var_nsteff_dn2, var_nsteff_dn3, var_nsteff_dn4, var_nsteff_dn5, var_nsteff_db0, var_nsteff_db1,)
    }
};
        var_nsteff = assign880_e926;
        var_nsteff_dn0 = assign880_e926_d_n0;
        var_nsteff_dn1 = assign880_e926_d_n1;
        var_nsteff_dn2 = assign880_e926_d_n2;
        var_nsteff_dn3 = assign880_e926_d_n3;
        var_nsteff_dn4 = assign880_e926_d_n4;
        var_nsteff_dn5 = assign880_e926_d_n5;
        var_nsteff_db0 = assign880_e926_d_b0;
        var_nsteff_db1 = assign880_e926_d_b1;
        var_nsteff_rv = 0.0;
        var_nsteff_rdn0 = 0.0;
        var_nsteff_rdn1 = 0.0;
        var_nsteff_rdn2 = 0.0;
        var_nsteff_rdn3 = 0.0;
        var_nsteff_rdn4 = 0.0;
        var_nsteff_rdn5 = 0.0;
        var_nsteff_rdb0 = 0.0;
        var_nsteff_rdb1 = 0.0;

        let (assign990_e1007, assign990_e1007_d_n0, assign990_e1007_d_n1, assign990_e1007_d_n2, assign990_e1007_d_n3, assign990_e1007_d_n4, assign990_e1007_d_n5, assign990_e1007_d_b0, assign990_e1007_d_b1,) = {
    if (p.p15 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cth, var_cth_dn0, var_cth_dn1, var_cth_dn2, var_cth_dn3, var_cth_dn4, var_cth_dn5, var_cth_db0, var_cth_db1,)
    }
};
        var_cth = assign990_e1007;
        var_cth_dn0 = assign990_e1007_d_n0;
        var_cth_dn1 = assign990_e1007_d_n1;
        var_cth_dn2 = assign990_e1007_d_n2;
        var_cth_dn3 = assign990_e1007_d_n3;
        var_cth_dn4 = assign990_e1007_d_n4;
        var_cth_dn5 = assign990_e1007_d_n5;
        var_cth_db0 = assign990_e1007_d_b0;
        var_cth_db1 = assign990_e1007_d_b1;
        var_cth_rv = 0.0;
        var_cth_rdn0 = 0.0;
        var_cth_rdn1 = 0.0;
        var_cth_rdn2 = 0.0;
        var_cth_rdn3 = 0.0;
        var_cth_rdn4 = 0.0;
        var_cth_rdn5 = 0.0;
        var_cth_rdb0 = 0.0;
        var_cth_rdb1 = 0.0;

        let (assign1010_e1049, assign1010_e1049_d_n0, assign1010_e1049_d_n1, assign1010_e1049_d_n2, assign1010_e1049_d_n3, assign1010_e1049_d_n4, assign1010_e1049_d_n5, assign1010_e1049_d_b0, assign1010_e1049_d_b1,) = {
    if (p.p15 == 0.0) {
        let assign1010_e1036: f64 = (p.p115 * var_p_um);
        let assign1010_e1037: f64 = (p.p114 + assign1010_e1036);
        let assign1010_e1040: f64 = (p.p116 * var_a_um2);
        let assign1010_e1041: f64 = (assign1010_e1037 + assign1010_e1040);
        let assign1010_e1045: f64 = (p.p5 + p.p8);
        let assign1010_e1046: f64 = (p.p117 * assign1010_e1045);
        let assign1010_e1047: f64 = (assign1010_e1041 + assign1010_e1046);
        (assign1010_e1047, ((p.p115 * var_p_um_dn0) + (p.p116 * var_a_um2_dn0)), ((p.p115 * var_p_um_dn1) + (p.p116 * var_a_um2_dn1)), ((p.p115 * var_p_um_dn2) + (p.p116 * var_a_um2_dn2)), ((p.p115 * var_p_um_dn3) + (p.p116 * var_a_um2_dn3)), ((p.p115 * var_p_um_dn4) + (p.p116 * var_a_um2_dn4)), ((p.p115 * var_p_um_dn5) + (p.p116 * var_a_um2_dn5)), ((p.p115 * var_p_um_db0) + (p.p116 * var_a_um2_db0)), ((p.p115 * var_p_um_db1) + (p.p116 * var_a_um2_db1)),)
    } else {
        (var_cth, var_cth_dn0, var_cth_dn1, var_cth_dn2, var_cth_dn3, var_cth_dn4, var_cth_dn5, var_cth_db0, var_cth_db1,)
    }
};
        var_cth = assign1010_e1049;
        var_cth_dn0 = assign1010_e1049_d_n0;
        var_cth_dn1 = assign1010_e1049_d_n1;
        var_cth_dn2 = assign1010_e1049_d_n2;
        var_cth_dn3 = assign1010_e1049_d_n3;
        var_cth_dn4 = assign1010_e1049_d_n4;
        var_cth_dn5 = assign1010_e1049_d_n5;
        var_cth_db0 = assign1010_e1049_d_b0;
        var_cth_db1 = assign1010_e1049_d_b1;
        var_cth_rv = 0.0;
        var_cth_rdn0 = 0.0;
        var_cth_rdn1 = 0.0;
        var_cth_rdn2 = 0.0;
        var_cth_rdn3 = 0.0;
        var_cth_rdn4 = 0.0;
        var_cth_rdn5 = 0.0;
        var_cth_rdb0 = 0.0;
        var_cth_rdb1 = 0.0;

        let assign1020_e1053: f64 = (p.p97 / var_weff_um);
        let assign1020_e1054: f64 = (p.p93 + assign1020_e1053);
        let assign1020_e1058: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1061: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1020_e1062: f64 = (assign1020_e1058 + assign1020_e1061);
        let assign1020_e1063: f64 = (0.5 * assign1020_e1062);
        let assign1020_e1067: f64 = (p.p99 / var_weff_um);
        let assign1020_e1068: f64 = (p.p95 + assign1020_e1067);
        let assign1020_e1069: f64 = (assign1020_e1063 * assign1020_e1068);
        let assign1020_e1071: f64 = (assign1020_e1069 / var_leff_um);
        let assign1020_e1072: f64 = (assign1020_e1054 + assign1020_e1071);
        var_tc1e = assign1020_e1072;
        var_tc1e_dn0 = ((-((p.p97 * var_weff_um_dn0) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn0) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn0)) / (var_leff_um * var_leff_um)));
        var_tc1e_dn1 = ((-((p.p97 * var_weff_um_dn1) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn1) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn1)) / (var_leff_um * var_leff_um)));
        var_tc1e_dn2 = ((-((p.p97 * var_weff_um_dn2) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn2) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn2)) / (var_leff_um * var_leff_um)));
        var_tc1e_dn3 = ((-((p.p97 * var_weff_um_dn3) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn3) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn3)) / (var_leff_um * var_leff_um)));
        var_tc1e_dn4 = ((-((p.p97 * var_weff_um_dn4) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn4) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn4)) / (var_leff_um * var_leff_um)));
        var_tc1e_dn5 = ((-((p.p97 * var_weff_um_dn5) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_dn5) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_dn5)) / (var_leff_um * var_leff_um)));
        var_tc1e_db0 = ((-((p.p97 * var_weff_um_db0) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_db0) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_db0)) / (var_leff_um * var_leff_um)));
        var_tc1e_db1 = ((-((p.p97 * var_weff_um_db1) / (var_weff_um * var_weff_um))) + ((((assign1020_e1063 * (-((p.p99 * var_weff_um_db1) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1020_e1069 * var_leff_um_db1)) / (var_leff_um * var_leff_um)));
        var_tc1e_rv = 0.0;
        var_tc1e_rdn0 = 0.0;
        var_tc1e_rdn1 = 0.0;
        var_tc1e_rdn2 = 0.0;
        var_tc1e_rdn3 = 0.0;
        var_tc1e_rdn4 = 0.0;
        var_tc1e_rdn5 = 0.0;
        var_tc1e_rdb0 = 0.0;
        var_tc1e_rdb1 = 0.0;

        let assign1030_e1076: f64 = (p.p98 / var_weff_um);
        let assign1030_e1077: f64 = (p.p94 + assign1030_e1076);
        let assign1030_e1081: f64 = if p.p5 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1084: f64 = if p.p8 > 0.0 { 1.0 } else { 0.0 };
        let assign1030_e1085: f64 = (assign1030_e1081 + assign1030_e1084);
        let assign1030_e1086: f64 = (0.5 * assign1030_e1085);
        let assign1030_e1090: f64 = (p.p100 / var_weff_um);
        let assign1030_e1091: f64 = (p.p96 + assign1030_e1090);
        let assign1030_e1092: f64 = (assign1030_e1086 * assign1030_e1091);
        let assign1030_e1094: f64 = (assign1030_e1092 / var_leff_um);
        let assign1030_e1095: f64 = (assign1030_e1077 + assign1030_e1094);
        var_tc2e = assign1030_e1095;
        var_tc2e_dn0 = ((-((p.p98 * var_weff_um_dn0) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn0) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn0)) / (var_leff_um * var_leff_um)));
        var_tc2e_dn1 = ((-((p.p98 * var_weff_um_dn1) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn1) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn1)) / (var_leff_um * var_leff_um)));
        var_tc2e_dn2 = ((-((p.p98 * var_weff_um_dn2) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn2) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn2)) / (var_leff_um * var_leff_um)));
        var_tc2e_dn3 = ((-((p.p98 * var_weff_um_dn3) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn3) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn3)) / (var_leff_um * var_leff_um)));
        var_tc2e_dn4 = ((-((p.p98 * var_weff_um_dn4) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn4) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn4)) / (var_leff_um * var_leff_um)));
        var_tc2e_dn5 = ((-((p.p98 * var_weff_um_dn5) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_dn5) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_dn5)) / (var_leff_um * var_leff_um)));
        var_tc2e_db0 = ((-((p.p98 * var_weff_um_db0) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_db0) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_db0)) / (var_leff_um * var_leff_um)));
        var_tc2e_db1 = ((-((p.p98 * var_weff_um_db1) / (var_weff_um * var_weff_um))) + ((((assign1030_e1086 * (-((p.p100 * var_weff_um_db1) / (var_weff_um * var_weff_um)))) * var_leff_um) - (assign1030_e1092 * var_leff_um_db1)) / (var_leff_um * var_leff_um)));
        var_tc2e_rv = 0.0;
        var_tc2e_rdn0 = 0.0;
        var_tc2e_rdn1 = 0.0;
        var_tc2e_rdn2 = 0.0;
        var_tc2e_rdn3 = 0.0;
        var_tc2e_rdn4 = 0.0;
        var_tc2e_rdn5 = 0.0;
        var_tc2e_rdb0 = 0.0;
        var_tc2e_rdb1 = 0.0;

        let assign1040_e1098: f64 = (p.p71 * var_a1_um2);
        let assign1040_e1101: f64 = (p.p78 * var_p1_um);
        let assign1040_e1102: f64 = (assign1040_e1098 + assign1040_e1101);
        var_cf1 = assign1040_e1102;
        var_cf1_dn0 = ((p.p71 * var_a1_um2_dn0) + (p.p78 * var_p1_um_dn0));
        var_cf1_dn1 = ((p.p71 * var_a1_um2_dn1) + (p.p78 * var_p1_um_dn1));
        var_cf1_dn2 = ((p.p71 * var_a1_um2_dn2) + (p.p78 * var_p1_um_dn2));
        var_cf1_dn3 = ((p.p71 * var_a1_um2_dn3) + (p.p78 * var_p1_um_dn3));
        var_cf1_dn4 = ((p.p71 * var_a1_um2_dn4) + (p.p78 * var_p1_um_dn4));
        var_cf1_dn5 = ((p.p71 * var_a1_um2_dn5) + (p.p78 * var_p1_um_dn5));
        var_cf1_db0 = ((p.p71 * var_a1_um2_db0) + (p.p78 * var_p1_um_db0));
        var_cf1_db1 = ((p.p71 * var_a1_um2_db1) + (p.p78 * var_p1_um_db1));
        var_cf1_rv = 0.0;
        var_cf1_rdn0 = 0.0;
        var_cf1_rdn1 = 0.0;
        var_cf1_rdn2 = 0.0;
        var_cf1_rdn3 = 0.0;
        var_cf1_rdn4 = 0.0;
        var_cf1_rdn5 = 0.0;
        var_cf1_rdb0 = 0.0;
        var_cf1_rdb1 = 0.0;

        let assign1050_e1105: f64 = (p.p71 * var_a2_um2);
        let assign1050_e1108: f64 = (p.p78 * var_p2_um);
        let assign1050_e1109: f64 = (assign1050_e1105 + assign1050_e1108);
        var_cf2 = assign1050_e1109;
        var_cf2_dn0 = ((p.p71 * var_a2_um2_dn0) + (p.p78 * var_p2_um_dn0));
        var_cf2_dn1 = ((p.p71 * var_a2_um2_dn1) + (p.p78 * var_p2_um_dn1));
        var_cf2_dn2 = ((p.p71 * var_a2_um2_dn2) + (p.p78 * var_p2_um_dn2));
        var_cf2_dn3 = ((p.p71 * var_a2_um2_dn3) + (p.p78 * var_p2_um_dn3));
        var_cf2_dn4 = ((p.p71 * var_a2_um2_dn4) + (p.p78 * var_p2_um_dn4));
        var_cf2_dn5 = ((p.p71 * var_a2_um2_dn5) + (p.p78 * var_p2_um_dn5));
        var_cf2_db0 = ((p.p71 * var_a2_um2_db0) + (p.p78 * var_p2_um_db0));
        var_cf2_db1 = ((p.p71 * var_a2_um2_db1) + (p.p78 * var_p2_um_db1));
        var_cf2_rv = 0.0;
        var_cf2_rdn0 = 0.0;
        var_cf2_rdn1 = 0.0;
        var_cf2_rdn2 = 0.0;
        var_cf2_rdn3 = 0.0;
        var_cf2_rdn4 = 0.0;
        var_cf2_rdn5 = 0.0;
        var_cf2_rdb0 = 0.0;
        var_cf2_rdb1 = 0.0;

        let assign1060_e1112: f64 = (p.p72 * var_a1_um2);
        let assign1060_e1115: f64 = (p.p79 * var_p1_um);
        let assign1060_e1116: f64 = (assign1060_e1112 + assign1060_e1115);
        var_cj1 = assign1060_e1116;
        var_cj1_dn0 = ((p.p72 * var_a1_um2_dn0) + (p.p79 * var_p1_um_dn0));
        var_cj1_dn1 = ((p.p72 * var_a1_um2_dn1) + (p.p79 * var_p1_um_dn1));
        var_cj1_dn2 = ((p.p72 * var_a1_um2_dn2) + (p.p79 * var_p1_um_dn2));
        var_cj1_dn3 = ((p.p72 * var_a1_um2_dn3) + (p.p79 * var_p1_um_dn3));
        var_cj1_dn4 = ((p.p72 * var_a1_um2_dn4) + (p.p79 * var_p1_um_dn4));
        var_cj1_dn5 = ((p.p72 * var_a1_um2_dn5) + (p.p79 * var_p1_um_dn5));
        var_cj1_db0 = ((p.p72 * var_a1_um2_db0) + (p.p79 * var_p1_um_db0));
        var_cj1_db1 = ((p.p72 * var_a1_um2_db1) + (p.p79 * var_p1_um_db1));
        var_cj1_rv = 0.0;
        var_cj1_rdn0 = 0.0;
        var_cj1_rdn1 = 0.0;
        var_cj1_rdn2 = 0.0;
        var_cj1_rdn3 = 0.0;
        var_cj1_rdn4 = 0.0;
        var_cj1_rdn5 = 0.0;
        var_cj1_rdb0 = 0.0;
        var_cj1_rdb1 = 0.0;

        let assign1070_e1119: f64 = (p.p72 * var_a2_um2);
        let assign1070_e1122: f64 = (p.p79 * var_p2_um);
        let assign1070_e1123: f64 = (assign1070_e1119 + assign1070_e1122);
        var_cj2 = assign1070_e1123;
        var_cj2_dn0 = ((p.p72 * var_a2_um2_dn0) + (p.p79 * var_p2_um_dn0));
        var_cj2_dn1 = ((p.p72 * var_a2_um2_dn1) + (p.p79 * var_p2_um_dn1));
        var_cj2_dn2 = ((p.p72 * var_a2_um2_dn2) + (p.p79 * var_p2_um_dn2));
        var_cj2_dn3 = ((p.p72 * var_a2_um2_dn3) + (p.p79 * var_p2_um_dn3));
        var_cj2_dn4 = ((p.p72 * var_a2_um2_dn4) + (p.p79 * var_p2_um_dn4));
        var_cj2_dn5 = ((p.p72 * var_a2_um2_dn5) + (p.p79 * var_p2_um_dn5));
        var_cj2_db0 = ((p.p72 * var_a2_um2_db0) + (p.p79 * var_p2_um_db0));
        var_cj2_db1 = ((p.p72 * var_a2_um2_db1) + (p.p79 * var_p2_um_db1));
        var_cj2_rv = 0.0;
        var_cj2_rdn0 = 0.0;
        var_cj2_rdn1 = 0.0;
        var_cj2_rdn2 = 0.0;
        var_cj2_rdn3 = 0.0;
        var_cj2_rdn4 = 0.0;
        var_cj2_rdn5 = 0.0;
        var_cj2_rdb0 = 0.0;
        var_cj2_rdb1 = 0.0;

        var_dt_et = (nv3 - 0.0);
        var_dt_et_dn0 = 0.0;
        var_dt_et_dn1 = 0.0;
        var_dt_et_dn2 = 0.0;
        var_dt_et_dn3 = 1.0;
        var_dt_et_dn4 = 0.0;
        var_dt_et_dn5 = 0.0;
        var_dt_et_db0 = 0.0;
        var_dt_et_db1 = 0.0;
        var_dt_et_rv = 0.0;
        var_dt_et_rdn0 = 0.0;
        var_dt_et_rdn1 = 0.0;
        var_dt_et_rdn2 = 0.0;
        var_dt_et_rdn3 = 0.0;
        var_dt_et_rdn4 = 0.0;
        var_dt_et_rdn5 = 0.0;
        var_dt_et_rdb0 = 0.0;
        var_dt_et_rdb1 = 0.0;

        let assign1090_e1126: f64 = (-p.p21);
        let assign1090_e1128: f64 = (assign1090_e1126 * (nv5 - nv4));
        var_vrb = assign1090_e1128;
        var_vrb_dn0 = 0.0;
        var_vrb_dn1 = 0.0;
        var_vrb_dn2 = 0.0;
        var_vrb_dn3 = 0.0;
        var_vrb_dn4 = (-assign1090_e1126);
        var_vrb_dn5 = assign1090_e1126;
        var_vrb_db0 = 0.0;
        var_vrb_db1 = 0.0;
        var_vrb_rv = 0.0;
        var_vrb_rdn0 = 0.0;
        var_vrb_rdn1 = 0.0;
        var_vrb_rdn2 = 0.0;
        var_vrb_rdn3 = 0.0;
        var_vrb_rdn4 = 0.0;
        var_vrb_rdn5 = 0.0;
        var_vrb_rdb0 = 0.0;
        var_vrb_rdb1 = 0.0;

        let assign1100_e1130: f64 = (-p.p21);
        let assign1100_e1132: f64 = (assign1100_e1130 * (nv1 - nv4));
        var_vc1 = assign1100_e1132;
        var_vc1_dn0 = 0.0;
        var_vc1_dn1 = assign1100_e1130;
        var_vc1_dn2 = 0.0;
        var_vc1_dn3 = 0.0;
        var_vc1_dn4 = (-assign1100_e1130);
        var_vc1_dn5 = 0.0;
        var_vc1_db0 = 0.0;
        var_vc1_db1 = 0.0;
        var_vc1_rv = 0.0;
        var_vc1_rdn0 = 0.0;
        var_vc1_rdn1 = 0.0;
        var_vc1_rdn2 = 0.0;
        var_vc1_rdn3 = 0.0;
        var_vc1_rdn4 = 0.0;
        var_vc1_rdn5 = 0.0;
        var_vc1_rdb0 = 0.0;
        var_vc1_rdb1 = 0.0;

        let assign1110_e1134: f64 = (-p.p21);
        let assign1110_e1136: f64 = (assign1110_e1134 * (nv1 - nv5));
        var_vc2 = assign1110_e1136;
        var_vc2_dn0 = 0.0;
        var_vc2_dn1 = assign1110_e1134;
        var_vc2_dn2 = 0.0;
        var_vc2_dn3 = 0.0;
        var_vc2_dn4 = 0.0;
        var_vc2_dn5 = (-assign1110_e1134);
        var_vc2_db0 = 0.0;
        var_vc2_db1 = 0.0;
        var_vc2_rv = 0.0;
        var_vc2_rdn0 = 0.0;
        var_vc2_rdn1 = 0.0;
        var_vc2_rdn2 = 0.0;
        var_vc2_rdn3 = 0.0;
        var_vc2_rdn4 = 0.0;
        var_vc2_rdn5 = 0.0;
        var_vc2_rdb0 = 0.0;
        var_vc2_rdb1 = 0.0;

        let assign1120_e1137: f64 = ctx_temp;
        let assign1120_e1139: f64 = (assign1120_e1137 + p.p9);
        let assign1120_e1141: f64 = (assign1120_e1139 + var_dt_et);
        let assign1120_e1143: f64 = (assign1120_e1141 - 273.15);
        var_tdevc = assign1120_e1143;
        var_tdevc_dn0 = var_dt_et_dn0;
        var_tdevc_dn1 = var_dt_et_dn1;
        var_tdevc_dn2 = var_dt_et_dn2;
        var_tdevc_dn3 = var_dt_et_dn3;
        var_tdevc_dn4 = var_dt_et_dn4;
        var_tdevc_dn5 = var_dt_et_dn5;
        var_tdevc_db0 = var_dt_et_db0;
        var_tdevc_db1 = var_dt_et_db1;
        var_tdevc_rv = 0.0;
        var_tdevc_rdn0 = 0.0;
        var_tdevc_rdn1 = 0.0;
        var_tdevc_rdn2 = 0.0;
        var_tdevc_rdn3 = 0.0;
        var_tdevc_rdn4 = 0.0;
        var_tdevc_rdn5 = 0.0;
        var_tdevc_rdb0 = 0.0;
        var_tdevc_rdb1 = 0.0;

        let assign1130_e1147: f64 = (p.p35 + 1.0);
        let assign1130_e1148: f64 = if var_tdevc < assign1130_e1147 { 1.0 } else { 0.0 };
        var_guard124 = assign1130_e1148;
        var_guard124_dn0 = 0.0;
        var_guard124_dn1 = 0.0;
        var_guard124_dn2 = 0.0;
        var_guard124_dn3 = 0.0;
        var_guard124_dn4 = 0.0;
        var_guard124_dn5 = 0.0;
        var_guard124_db0 = 0.0;
        var_guard124_db1 = 0.0;
        var_guard124_rv = 0.0;
        var_guard124_rdn0 = 0.0;
        var_guard124_rdn1 = 0.0;
        var_guard124_rdn2 = 0.0;
        var_guard124_rdn3 = 0.0;
        var_guard124_rdn4 = 0.0;
        var_guard124_rdn5 = 0.0;
        var_guard124_rdb0 = 0.0;
        var_guard124_rdb1 = 0.0;

        let (assign1140_e1159, assign1140_e1159_d_n0, assign1140_e1159_d_n1, assign1140_e1159_d_n2, assign1140_e1159_d_n3, assign1140_e1159_d_n4, assign1140_e1159_d_n5, assign1140_e1159_d_b0, assign1140_e1159_d_b1,) = {
    if (var_guard124 != 0.0) {
        let assign1140_e1153: f64 = (var_tdevc - p.p35);
        let assign1140_e1155: f64 = (assign1140_e1153 - 1.0);
        let assign1140_e1156: f64 = (assign1140_e1155).exp();
        let assign1140_e1157: f64 = (p.p35 + assign1140_e1156);
        (assign1140_e1157, (assign1140_e1156 * var_tdevc_dn0), (assign1140_e1156 * var_tdevc_dn1), (assign1140_e1156 * var_tdevc_dn2), (assign1140_e1156 * var_tdevc_dn3), (assign1140_e1156 * var_tdevc_dn4), (assign1140_e1156 * var_tdevc_dn5), (assign1140_e1156 * var_tdevc_db0), (assign1140_e1156 * var_tdevc_db1),)
    } else {
        (var_tdevc, var_tdevc_dn0, var_tdevc_dn1, var_tdevc_dn2, var_tdevc_dn3, var_tdevc_dn4, var_tdevc_dn5, var_tdevc_db0, var_tdevc_db1,)
    }
};
        var_tdevc = assign1140_e1159;
        var_tdevc_dn0 = assign1140_e1159_d_n0;
        var_tdevc_dn1 = assign1140_e1159_d_n1;
        var_tdevc_dn2 = assign1140_e1159_d_n2;
        var_tdevc_dn3 = assign1140_e1159_d_n3;
        var_tdevc_dn4 = assign1140_e1159_d_n4;
        var_tdevc_dn5 = assign1140_e1159_d_n5;
        var_tdevc_db0 = assign1140_e1159_d_b0;
        var_tdevc_db1 = assign1140_e1159_d_b1;
        var_tdevc_rv = 0.0;
        var_tdevc_rdn0 = 0.0;
        var_tdevc_rdn1 = 0.0;
        var_tdevc_rdn2 = 0.0;
        var_tdevc_rdn3 = 0.0;
        var_tdevc_rdn4 = 0.0;
        var_tdevc_rdn5 = 0.0;
        var_tdevc_rdb0 = 0.0;
        var_tdevc_rdb1 = 0.0;

        let assign1150_e1163: f64 = (p.p36 - 1.0);
        let assign1150_e1164: f64 = if var_tdevc > assign1150_e1163 { 1.0 } else { 0.0 };
        var_guard125 = assign1150_e1164;
        var_guard125_dn0 = 0.0;
        var_guard125_dn1 = 0.0;
        var_guard125_dn2 = 0.0;
        var_guard125_dn3 = 0.0;
        var_guard125_dn4 = 0.0;
        var_guard125_dn5 = 0.0;
        var_guard125_db0 = 0.0;
        var_guard125_db1 = 0.0;
        var_guard125_rv = 0.0;
        var_guard125_rdn0 = 0.0;
        var_guard125_rdn1 = 0.0;
        var_guard125_rdn2 = 0.0;
        var_guard125_rdn3 = 0.0;
        var_guard125_rdn4 = 0.0;
        var_guard125_rdn5 = 0.0;
        var_guard125_rdb0 = 0.0;
        var_guard125_rdb1 = 0.0;

        *var_cf1_slot = var_cf1;
        *var_cf1_db0_slot = var_cf1_db0;
        *var_cf1_db1_slot = var_cf1_db1;
        *var_cf1_dn0_slot = var_cf1_dn0;
        *var_cf1_dn1_slot = var_cf1_dn1;
        *var_cf1_dn2_slot = var_cf1_dn2;
        *var_cf1_dn3_slot = var_cf1_dn3;
        *var_cf1_dn4_slot = var_cf1_dn4;
        *var_cf1_dn5_slot = var_cf1_dn5;
        *var_cf1_rdb0_slot = var_cf1_rdb0;
        *var_cf1_rdb1_slot = var_cf1_rdb1;
        *var_cf1_rdn0_slot = var_cf1_rdn0;
        *var_cf1_rdn1_slot = var_cf1_rdn1;
        *var_cf1_rdn2_slot = var_cf1_rdn2;
        *var_cf1_rdn3_slot = var_cf1_rdn3;
        *var_cf1_rdn4_slot = var_cf1_rdn4;
        *var_cf1_rdn5_slot = var_cf1_rdn5;
        *var_cf1_rv_slot = var_cf1_rv;
        *var_cf2_slot = var_cf2;
        *var_cf2_db0_slot = var_cf2_db0;
        *var_cf2_db1_slot = var_cf2_db1;
        *var_cf2_dn0_slot = var_cf2_dn0;
        *var_cf2_dn1_slot = var_cf2_dn1;
        *var_cf2_dn2_slot = var_cf2_dn2;
        *var_cf2_dn3_slot = var_cf2_dn3;
        *var_cf2_dn4_slot = var_cf2_dn4;
        *var_cf2_dn5_slot = var_cf2_dn5;
        *var_cf2_rdb0_slot = var_cf2_rdb0;
        *var_cf2_rdb1_slot = var_cf2_rdb1;
        *var_cf2_rdn0_slot = var_cf2_rdn0;
        *var_cf2_rdn1_slot = var_cf2_rdn1;
        *var_cf2_rdn2_slot = var_cf2_rdn2;
        *var_cf2_rdn3_slot = var_cf2_rdn3;
        *var_cf2_rdn4_slot = var_cf2_rdn4;
        *var_cf2_rdn5_slot = var_cf2_rdn5;
        *var_cf2_rv_slot = var_cf2_rv;
        *var_cj1_slot = var_cj1;
        *var_cj1_db0_slot = var_cj1_db0;
        *var_cj1_db1_slot = var_cj1_db1;
        *var_cj1_dn0_slot = var_cj1_dn0;
        *var_cj1_dn1_slot = var_cj1_dn1;
        *var_cj1_dn2_slot = var_cj1_dn2;
        *var_cj1_dn3_slot = var_cj1_dn3;
        *var_cj1_dn4_slot = var_cj1_dn4;
        *var_cj1_dn5_slot = var_cj1_dn5;
        *var_cj1_rdb0_slot = var_cj1_rdb0;
        *var_cj1_rdb1_slot = var_cj1_rdb1;
        *var_cj1_rdn0_slot = var_cj1_rdn0;
        *var_cj1_rdn1_slot = var_cj1_rdn1;
        *var_cj1_rdn2_slot = var_cj1_rdn2;
        *var_cj1_rdn3_slot = var_cj1_rdn3;
        *var_cj1_rdn4_slot = var_cj1_rdn4;
        *var_cj1_rdn5_slot = var_cj1_rdn5;
        *var_cj1_rv_slot = var_cj1_rv;
        *var_cj2_slot = var_cj2;
        *var_cj2_db0_slot = var_cj2_db0;
        *var_cj2_db1_slot = var_cj2_db1;
        *var_cj2_dn0_slot = var_cj2_dn0;
        *var_cj2_dn1_slot = var_cj2_dn1;
        *var_cj2_dn2_slot = var_cj2_dn2;
        *var_cj2_dn3_slot = var_cj2_dn3;
        *var_cj2_dn4_slot = var_cj2_dn4;
        *var_cj2_dn5_slot = var_cj2_dn5;
        *var_cj2_rdb0_slot = var_cj2_rdb0;
        *var_cj2_rdb1_slot = var_cj2_rdb1;
        *var_cj2_rdn0_slot = var_cj2_rdn0;
        *var_cj2_rdn1_slot = var_cj2_rdn1;
        *var_cj2_rdn2_slot = var_cj2_rdn2;
        *var_cj2_rdn3_slot = var_cj2_rdn3;
        *var_cj2_rdn4_slot = var_cj2_rdn4;
        *var_cj2_rdn5_slot = var_cj2_rdn5;
        *var_cj2_rv_slot = var_cj2_rv;
        *var_cth_slot = var_cth;
        *var_cth_db0_slot = var_cth_db0;
        *var_cth_db1_slot = var_cth_db1;
        *var_cth_dn0_slot = var_cth_dn0;
        *var_cth_dn1_slot = var_cth_dn1;
        *var_cth_dn2_slot = var_cth_dn2;
        *var_cth_dn3_slot = var_cth_dn3;
        *var_cth_dn4_slot = var_cth_dn4;
        *var_cth_dn5_slot = var_cth_dn5;
        *var_cth_rdb0_slot = var_cth_rdb0;
        *var_cth_rdb1_slot = var_cth_rdb1;
        *var_cth_rdn0_slot = var_cth_rdn0;
        *var_cth_rdn1_slot = var_cth_rdn1;
        *var_cth_rdn2_slot = var_cth_rdn2;
        *var_cth_rdn3_slot = var_cth_rdn3;
        *var_cth_rdn4_slot = var_cth_rdn4;
        *var_cth_rdn5_slot = var_cth_rdn5;
        *var_cth_rv_slot = var_cth_rv;
        *var_dt_et_slot = var_dt_et;
        *var_dt_et_db0_slot = var_dt_et_db0;
        *var_dt_et_db1_slot = var_dt_et_db1;
        *var_dt_et_dn0_slot = var_dt_et_dn0;
        *var_dt_et_dn1_slot = var_dt_et_dn1;
        *var_dt_et_dn2_slot = var_dt_et_dn2;
        *var_dt_et_dn3_slot = var_dt_et_dn3;
        *var_dt_et_dn4_slot = var_dt_et_dn4;
        *var_dt_et_dn5_slot = var_dt_et_dn5;
        *var_dt_et_rdb0_slot = var_dt_et_rdb0;
        *var_dt_et_rdb1_slot = var_dt_et_rdb1;
        *var_dt_et_rdn0_slot = var_dt_et_rdn0;
        *var_dt_et_rdn1_slot = var_dt_et_rdn1;
        *var_dt_et_rdn2_slot = var_dt_et_rdn2;
        *var_dt_et_rdn3_slot = var_dt_et_rdn3;
        *var_dt_et_rdn4_slot = var_dt_et_rdn4;
        *var_dt_et_rdn5_slot = var_dt_et_rdn5;
        *var_dt_et_rv_slot = var_dt_et_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_db0_slot = var_guard120_db0;
        *var_guard120_db1_slot = var_guard120_db1;
        *var_guard120_dn0_slot = var_guard120_dn0;
        *var_guard120_dn1_slot = var_guard120_dn1;
        *var_guard120_dn2_slot = var_guard120_dn2;
        *var_guard120_dn3_slot = var_guard120_dn3;
        *var_guard120_dn4_slot = var_guard120_dn4;
        *var_guard120_dn5_slot = var_guard120_dn5;
        *var_guard120_rdb0_slot = var_guard120_rdb0;
        *var_guard120_rdb1_slot = var_guard120_rdb1;
        *var_guard120_rdn0_slot = var_guard120_rdn0;
        *var_guard120_rdn1_slot = var_guard120_rdn1;
        *var_guard120_rdn2_slot = var_guard120_rdn2;
        *var_guard120_rdn3_slot = var_guard120_rdn3;
        *var_guard120_rdn4_slot = var_guard120_rdn4;
        *var_guard120_rdn5_slot = var_guard120_rdn5;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_db0_slot = var_guard124_db0;
        *var_guard124_db1_slot = var_guard124_db1;
        *var_guard124_dn0_slot = var_guard124_dn0;
        *var_guard124_dn1_slot = var_guard124_dn1;
        *var_guard124_dn2_slot = var_guard124_dn2;
        *var_guard124_dn3_slot = var_guard124_dn3;
        *var_guard124_dn4_slot = var_guard124_dn4;
        *var_guard124_dn5_slot = var_guard124_dn5;
        *var_guard124_rdb0_slot = var_guard124_rdb0;
        *var_guard124_rdb1_slot = var_guard124_rdb1;
        *var_guard124_rdn0_slot = var_guard124_rdn0;
        *var_guard124_rdn1_slot = var_guard124_rdn1;
        *var_guard124_rdn2_slot = var_guard124_rdn2;
        *var_guard124_rdn3_slot = var_guard124_rdn3;
        *var_guard124_rdn4_slot = var_guard124_rdn4;
        *var_guard124_rdn5_slot = var_guard124_rdn5;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_db0_slot = var_guard125_db0;
        *var_guard125_db1_slot = var_guard125_db1;
        *var_guard125_dn0_slot = var_guard125_dn0;
        *var_guard125_dn1_slot = var_guard125_dn1;
        *var_guard125_dn2_slot = var_guard125_dn2;
        *var_guard125_dn3_slot = var_guard125_dn3;
        *var_guard125_dn4_slot = var_guard125_dn4;
        *var_guard125_dn5_slot = var_guard125_dn5;
        *var_guard125_rdb0_slot = var_guard125_rdb0;
        *var_guard125_rdb1_slot = var_guard125_rdb1;
        *var_guard125_rdn0_slot = var_guard125_rdn0;
        *var_guard125_rdn1_slot = var_guard125_rdn1;
        *var_guard125_rdn2_slot = var_guard125_rdn2;
        *var_guard125_rdn3_slot = var_guard125_rdn3;
        *var_guard125_rdn4_slot = var_guard125_rdn4;
        *var_guard125_rdn5_slot = var_guard125_rdn5;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_nsteff_slot = var_nsteff;
        *var_nsteff_db0_slot = var_nsteff_db0;
        *var_nsteff_db1_slot = var_nsteff_db1;
        *var_nsteff_dn0_slot = var_nsteff_dn0;
        *var_nsteff_dn1_slot = var_nsteff_dn1;
        *var_nsteff_dn2_slot = var_nsteff_dn2;
        *var_nsteff_dn3_slot = var_nsteff_dn3;
        *var_nsteff_dn4_slot = var_nsteff_dn4;
        *var_nsteff_dn5_slot = var_nsteff_dn5;
        *var_nsteff_rdb0_slot = var_nsteff_rdb0;
        *var_nsteff_rdb1_slot = var_nsteff_rdb1;
        *var_nsteff_rdn0_slot = var_nsteff_rdn0;
        *var_nsteff_rdn1_slot = var_nsteff_rdn1;
        *var_nsteff_rdn2_slot = var_nsteff_rdn2;
        *var_nsteff_rdn3_slot = var_nsteff_rdn3;
        *var_nsteff_rdn4_slot = var_nsteff_rdn4;
        *var_nsteff_rdn5_slot = var_nsteff_rdn5;
        *var_nsteff_rv_slot = var_nsteff_rv;
        *var_tc1e_slot = var_tc1e;
        *var_tc1e_db0_slot = var_tc1e_db0;
        *var_tc1e_db1_slot = var_tc1e_db1;
        *var_tc1e_dn0_slot = var_tc1e_dn0;
        *var_tc1e_dn1_slot = var_tc1e_dn1;
        *var_tc1e_dn2_slot = var_tc1e_dn2;
        *var_tc1e_dn3_slot = var_tc1e_dn3;
        *var_tc1e_dn4_slot = var_tc1e_dn4;
        *var_tc1e_dn5_slot = var_tc1e_dn5;
        *var_tc1e_rdb0_slot = var_tc1e_rdb0;
        *var_tc1e_rdb1_slot = var_tc1e_rdb1;
        *var_tc1e_rdn0_slot = var_tc1e_rdn0;
        *var_tc1e_rdn1_slot = var_tc1e_rdn1;
        *var_tc1e_rdn2_slot = var_tc1e_rdn2;
        *var_tc1e_rdn3_slot = var_tc1e_rdn3;
        *var_tc1e_rdn4_slot = var_tc1e_rdn4;
        *var_tc1e_rdn5_slot = var_tc1e_rdn5;
        *var_tc1e_rv_slot = var_tc1e_rv;
        *var_tc2e_slot = var_tc2e;
        *var_tc2e_db0_slot = var_tc2e_db0;
        *var_tc2e_db1_slot = var_tc2e_db1;
        *var_tc2e_dn0_slot = var_tc2e_dn0;
        *var_tc2e_dn1_slot = var_tc2e_dn1;
        *var_tc2e_dn2_slot = var_tc2e_dn2;
        *var_tc2e_dn3_slot = var_tc2e_dn3;
        *var_tc2e_dn4_slot = var_tc2e_dn4;
        *var_tc2e_dn5_slot = var_tc2e_dn5;
        *var_tc2e_rdb0_slot = var_tc2e_rdb0;
        *var_tc2e_rdb1_slot = var_tc2e_rdb1;
        *var_tc2e_rdn0_slot = var_tc2e_rdn0;
        *var_tc2e_rdn1_slot = var_tc2e_rdn1;
        *var_tc2e_rdn2_slot = var_tc2e_rdn2;
        *var_tc2e_rdn3_slot = var_tc2e_rdn3;
        *var_tc2e_rdn4_slot = var_tc2e_rdn4;
        *var_tc2e_rdn5_slot = var_tc2e_rdn5;
        *var_tc2e_rv_slot = var_tc2e_rv;
        *var_tdevc_slot = var_tdevc;
        *var_tdevc_db0_slot = var_tdevc_db0;
        *var_tdevc_db1_slot = var_tdevc_db1;
        *var_tdevc_dn0_slot = var_tdevc_dn0;
        *var_tdevc_dn1_slot = var_tdevc_dn1;
        *var_tdevc_dn2_slot = var_tdevc_dn2;
        *var_tdevc_dn3_slot = var_tdevc_dn3;
        *var_tdevc_dn4_slot = var_tdevc_dn4;
        *var_tdevc_dn5_slot = var_tdevc_dn5;
        *var_tdevc_rdb0_slot = var_tdevc_rdb0;
        *var_tdevc_rdb1_slot = var_tdevc_rdb1;
        *var_tdevc_rdn0_slot = var_tdevc_rdn0;
        *var_tdevc_rdn1_slot = var_tdevc_rdn1;
        *var_tdevc_rdn2_slot = var_tdevc_rdn2;
        *var_tdevc_rdn3_slot = var_tdevc_rdn3;
        *var_tdevc_rdn4_slot = var_tdevc_rdn4;
        *var_tdevc_rdn5_slot = var_tdevc_rdn5;
        *var_tdevc_rv_slot = var_tdevc_rv;
        *var_vc1_slot = var_vc1;
        *var_vc1_db0_slot = var_vc1_db0;
        *var_vc1_db1_slot = var_vc1_db1;
        *var_vc1_dn0_slot = var_vc1_dn0;
        *var_vc1_dn1_slot = var_vc1_dn1;
        *var_vc1_dn2_slot = var_vc1_dn2;
        *var_vc1_dn3_slot = var_vc1_dn3;
        *var_vc1_dn4_slot = var_vc1_dn4;
        *var_vc1_dn5_slot = var_vc1_dn5;
        *var_vc1_rdb0_slot = var_vc1_rdb0;
        *var_vc1_rdb1_slot = var_vc1_rdb1;
        *var_vc1_rdn0_slot = var_vc1_rdn0;
        *var_vc1_rdn1_slot = var_vc1_rdn1;
        *var_vc1_rdn2_slot = var_vc1_rdn2;
        *var_vc1_rdn3_slot = var_vc1_rdn3;
        *var_vc1_rdn4_slot = var_vc1_rdn4;
        *var_vc1_rdn5_slot = var_vc1_rdn5;
        *var_vc1_rv_slot = var_vc1_rv;
        *var_vc2_slot = var_vc2;
        *var_vc2_db0_slot = var_vc2_db0;
        *var_vc2_db1_slot = var_vc2_db1;
        *var_vc2_dn0_slot = var_vc2_dn0;
        *var_vc2_dn1_slot = var_vc2_dn1;
        *var_vc2_dn2_slot = var_vc2_dn2;
        *var_vc2_dn3_slot = var_vc2_dn3;
        *var_vc2_dn4_slot = var_vc2_dn4;
        *var_vc2_dn5_slot = var_vc2_dn5;
        *var_vc2_rdb0_slot = var_vc2_rdb0;
        *var_vc2_rdb1_slot = var_vc2_rdb1;
        *var_vc2_rdn0_slot = var_vc2_rdn0;
        *var_vc2_rdn1_slot = var_vc2_rdn1;
        *var_vc2_rdn2_slot = var_vc2_rdn2;
        *var_vc2_rdn3_slot = var_vc2_rdn3;
        *var_vc2_rdn4_slot = var_vc2_rdn4;
        *var_vc2_rdn5_slot = var_vc2_rdn5;
        *var_vc2_rv_slot = var_vc2_rv;
        *var_vrb_slot = var_vrb;
        *var_vrb_db0_slot = var_vrb_db0;
        *var_vrb_db1_slot = var_vrb_db1;
        *var_vrb_dn0_slot = var_vrb_dn0;
        *var_vrb_dn1_slot = var_vrb_dn1;
        *var_vrb_dn2_slot = var_vrb_dn2;
        *var_vrb_dn3_slot = var_vrb_dn3;
        *var_vrb_dn4_slot = var_vrb_dn4;
        *var_vrb_dn5_slot = var_vrb_dn5;
        *var_vrb_rdb0_slot = var_vrb_rdb0;
        *var_vrb_rdb1_slot = var_vrb_rdb1;
        *var_vrb_rdn0_slot = var_vrb_rdn0;
        *var_vrb_rdn1_slot = var_vrb_rdn1;
        *var_vrb_rdn2_slot = var_vrb_rdn2;
        *var_vrb_rdn3_slot = var_vrb_rdn3;
        *var_vrb_rdn4_slot = var_vrb_rdn4;
        *var_vrb_rdn5_slot = var_vrb_rdn5;
        *var_vrb_rv_slot = var_vrb_rv;
    }

    pub(super) fn stamp_reactive_block_4(
        p: &Parameters,
        var_guard124: f64,
        var_guard125: f64,
        var_tc1e: f64,
        var_tc1e_db0: f64,
        var_tc1e_db1: f64,
        var_tc1e_dn0: f64,
        var_tc1e_dn1: f64,
        var_tc1e_dn2: f64,
        var_tc1e_dn3: f64,
        var_tc1e_dn4: f64,
        var_tc1e_dn5: f64,
        var_tc2e: f64,
        var_tc2e_db0: f64,
        var_tc2e_db1: f64,
        var_tc2e_dn0: f64,
        var_tc2e_dn1: f64,
        var_tc2e_dn2: f64,
        var_tc2e_dn3: f64,
        var_tc2e_dn4: f64,
        var_tc2e_dn5: f64,
        var_tinik: f64,
        var_tinik_db0: f64,
        var_tinik_db1: f64,
        var_tinik_dn0: f64,
        var_tinik_dn1: f64,
        var_tinik_dn2: f64,
        var_tinik_dn3: f64,
        var_tinik_dn4: f64,
        var_tinik_dn5: f64,
        var_cja_t_slot: &mut f64,
        var_cja_t_db0_slot: &mut f64,
        var_cja_t_db1_slot: &mut f64,
        var_cja_t_dn0_slot: &mut f64,
        var_cja_t_dn1_slot: &mut f64,
        var_cja_t_dn2_slot: &mut f64,
        var_cja_t_dn3_slot: &mut f64,
        var_cja_t_dn4_slot: &mut f64,
        var_cja_t_dn5_slot: &mut f64,
        var_cja_t_rdb0_slot: &mut f64,
        var_cja_t_rdb1_slot: &mut f64,
        var_cja_t_rdn0_slot: &mut f64,
        var_cja_t_rdn1_slot: &mut f64,
        var_cja_t_rdn2_slot: &mut f64,
        var_cja_t_rdn3_slot: &mut f64,
        var_cja_t_rdn4_slot: &mut f64,
        var_cja_t_rdn5_slot: &mut f64,
        var_cja_t_rv_slot: &mut f64,
        var_dt_slot: &mut f64,
        var_dt_db0_slot: &mut f64,
        var_dt_db1_slot: &mut f64,
        var_dt_dn0_slot: &mut f64,
        var_dt_dn1_slot: &mut f64,
        var_dt_dn2_slot: &mut f64,
        var_dt_dn3_slot: &mut f64,
        var_dt_dn4_slot: &mut f64,
        var_dt_dn5_slot: &mut f64,
        var_dt_rdb0_slot: &mut f64,
        var_dt_rdb1_slot: &mut f64,
        var_dt_rdn0_slot: &mut f64,
        var_dt_rdn1_slot: &mut f64,
        var_dt_rdn2_slot: &mut f64,
        var_dt_rdn3_slot: &mut f64,
        var_dt_rdn4_slot: &mut f64,
        var_dt_rdn5_slot: &mut f64,
        var_dt_rv_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard126_db0_slot: &mut f64,
        var_guard126_db1_slot: &mut f64,
        var_guard126_dn0_slot: &mut f64,
        var_guard126_dn1_slot: &mut f64,
        var_guard126_dn2_slot: &mut f64,
        var_guard126_dn3_slot: &mut f64,
        var_guard126_dn4_slot: &mut f64,
        var_guard126_dn5_slot: &mut f64,
        var_guard126_rdb0_slot: &mut f64,
        var_guard126_rdb1_slot: &mut f64,
        var_guard126_rdn0_slot: &mut f64,
        var_guard126_rdn1_slot: &mut f64,
        var_guard126_rdn2_slot: &mut f64,
        var_guard126_rdn3_slot: &mut f64,
        var_guard126_rdn4_slot: &mut f64,
        var_guard126_rdn5_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_db0_slot: &mut f64,
        var_guard130_db1_slot: &mut f64,
        var_guard130_dn0_slot: &mut f64,
        var_guard130_dn1_slot: &mut f64,
        var_guard130_dn2_slot: &mut f64,
        var_guard130_dn3_slot: &mut f64,
        var_guard130_dn4_slot: &mut f64,
        var_guard130_dn5_slot: &mut f64,
        var_guard130_rdb0_slot: &mut f64,
        var_guard130_rdb1_slot: &mut f64,
        var_guard130_rdn0_slot: &mut f64,
        var_guard130_rdn1_slot: &mut f64,
        var_guard130_rdn2_slot: &mut f64,
        var_guard130_rdn3_slot: &mut f64,
        var_guard130_rdn4_slot: &mut f64,
        var_guard130_rdn5_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_db0_slot: &mut f64,
        var_guard133_db1_slot: &mut f64,
        var_guard133_dn0_slot: &mut f64,
        var_guard133_dn1_slot: &mut f64,
        var_guard133_dn2_slot: &mut f64,
        var_guard133_dn3_slot: &mut f64,
        var_guard133_dn4_slot: &mut f64,
        var_guard133_dn5_slot: &mut f64,
        var_guard133_rdb0_slot: &mut f64,
        var_guard133_rdb1_slot: &mut f64,
        var_guard133_rdn0_slot: &mut f64,
        var_guard133_rdn1_slot: &mut f64,
        var_guard133_rdn2_slot: &mut f64,
        var_guard133_rdn3_slot: &mut f64,
        var_guard133_rdn4_slot: &mut f64,
        var_guard133_rdn5_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_pa_t_slot: &mut f64,
        var_pa_t_db0_slot: &mut f64,
        var_pa_t_db1_slot: &mut f64,
        var_pa_t_dn0_slot: &mut f64,
        var_pa_t_dn1_slot: &mut f64,
        var_pa_t_dn2_slot: &mut f64,
        var_pa_t_dn3_slot: &mut f64,
        var_pa_t_dn4_slot: &mut f64,
        var_pa_t_dn5_slot: &mut f64,
        var_pa_t_rdb0_slot: &mut f64,
        var_pa_t_rdb1_slot: &mut f64,
        var_pa_t_rdn0_slot: &mut f64,
        var_pa_t_rdn1_slot: &mut f64,
        var_pa_t_rdn2_slot: &mut f64,
        var_pa_t_rdn3_slot: &mut f64,
        var_pa_t_rdn4_slot: &mut f64,
        var_pa_t_rdn5_slot: &mut f64,
        var_pa_t_rv_slot: &mut f64,
        var_phi_t_slot: &mut f64,
        var_phi_t_db0_slot: &mut f64,
        var_phi_t_db1_slot: &mut f64,
        var_phi_t_dn0_slot: &mut f64,
        var_phi_t_dn1_slot: &mut f64,
        var_phi_t_dn2_slot: &mut f64,
        var_phi_t_dn3_slot: &mut f64,
        var_phi_t_dn4_slot: &mut f64,
        var_phi_t_dn5_slot: &mut f64,
        var_phi_t_rdb0_slot: &mut f64,
        var_phi_t_rdb1_slot: &mut f64,
        var_phi_t_rdn0_slot: &mut f64,
        var_phi_t_rdn1_slot: &mut f64,
        var_phi_t_rdn2_slot: &mut f64,
        var_phi_t_rdn3_slot: &mut f64,
        var_phi_t_rdn4_slot: &mut f64,
        var_phi_t_rdn5_slot: &mut f64,
        var_phi_t_rv_slot: &mut f64,
        var_psiin_slot: &mut f64,
        var_psiin_db0_slot: &mut f64,
        var_psiin_db1_slot: &mut f64,
        var_psiin_dn0_slot: &mut f64,
        var_psiin_dn1_slot: &mut f64,
        var_psiin_dn2_slot: &mut f64,
        var_psiin_dn3_slot: &mut f64,
        var_psiin_dn4_slot: &mut f64,
        var_psiin_dn5_slot: &mut f64,
        var_psiin_rdb0_slot: &mut f64,
        var_psiin_rdb1_slot: &mut f64,
        var_psiin_rdn0_slot: &mut f64,
        var_psiin_rdn1_slot: &mut f64,
        var_psiin_rdn2_slot: &mut f64,
        var_psiin_rdn3_slot: &mut f64,
        var_psiin_rdn4_slot: &mut f64,
        var_psiin_rdn5_slot: &mut f64,
        var_psiin_rv_slot: &mut f64,
        var_psiio_slot: &mut f64,
        var_psiio_db0_slot: &mut f64,
        var_psiio_db1_slot: &mut f64,
        var_psiio_dn0_slot: &mut f64,
        var_psiio_dn1_slot: &mut f64,
        var_psiio_dn2_slot: &mut f64,
        var_psiio_dn3_slot: &mut f64,
        var_psiio_dn4_slot: &mut f64,
        var_psiio_dn5_slot: &mut f64,
        var_psiio_rdb0_slot: &mut f64,
        var_psiio_rdb1_slot: &mut f64,
        var_psiio_rdn0_slot: &mut f64,
        var_psiio_rdn1_slot: &mut f64,
        var_psiio_rdn2_slot: &mut f64,
        var_psiio_rdn3_slot: &mut f64,
        var_psiio_rdn4_slot: &mut f64,
        var_psiio_rdn5_slot: &mut f64,
        var_psiio_rv_slot: &mut f64,
        var_rt_slot: &mut f64,
        var_rt_db0_slot: &mut f64,
        var_rt_db1_slot: &mut f64,
        var_rt_dn0_slot: &mut f64,
        var_rt_dn1_slot: &mut f64,
        var_rt_dn2_slot: &mut f64,
        var_rt_dn3_slot: &mut f64,
        var_rt_dn4_slot: &mut f64,
        var_rt_dn5_slot: &mut f64,
        var_rt_rdb0_slot: &mut f64,
        var_rt_rdb1_slot: &mut f64,
        var_rt_rdn0_slot: &mut f64,
        var_rt_rdn1_slot: &mut f64,
        var_rt_rdn2_slot: &mut f64,
        var_rt_rdn3_slot: &mut f64,
        var_rt_rdn4_slot: &mut f64,
        var_rt_rdn5_slot: &mut f64,
        var_rt_rv_slot: &mut f64,
        var_tcr_slot: &mut f64,
        var_tcr_db0_slot: &mut f64,
        var_tcr_db1_slot: &mut f64,
        var_tcr_dn0_slot: &mut f64,
        var_tcr_dn1_slot: &mut f64,
        var_tcr_dn2_slot: &mut f64,
        var_tcr_dn3_slot: &mut f64,
        var_tcr_dn4_slot: &mut f64,
        var_tcr_dn5_slot: &mut f64,
        var_tcr_rdb0_slot: &mut f64,
        var_tcr_rdb1_slot: &mut f64,
        var_tcr_rdn0_slot: &mut f64,
        var_tcr_rdn1_slot: &mut f64,
        var_tcr_rdn2_slot: &mut f64,
        var_tcr_rdn3_slot: &mut f64,
        var_tcr_rdn4_slot: &mut f64,
        var_tcr_rdn5_slot: &mut f64,
        var_tcr_rv_slot: &mut f64,
        var_tcvsat_slot: &mut f64,
        var_tcvsat_db0_slot: &mut f64,
        var_tcvsat_db1_slot: &mut f64,
        var_tcvsat_dn0_slot: &mut f64,
        var_tcvsat_dn1_slot: &mut f64,
        var_tcvsat_dn2_slot: &mut f64,
        var_tcvsat_dn3_slot: &mut f64,
        var_tcvsat_dn4_slot: &mut f64,
        var_tcvsat_dn5_slot: &mut f64,
        var_tcvsat_rdb0_slot: &mut f64,
        var_tcvsat_rdb1_slot: &mut f64,
        var_tcvsat_rdn0_slot: &mut f64,
        var_tcvsat_rdn1_slot: &mut f64,
        var_tcvsat_rdn2_slot: &mut f64,
        var_tcvsat_rdn3_slot: &mut f64,
        var_tcvsat_rdn4_slot: &mut f64,
        var_tcvsat_rdn5_slot: &mut f64,
        var_tcvsat_rv_slot: &mut f64,
        var_tdevc_slot: &mut f64,
        var_tdevc_db0_slot: &mut f64,
        var_tdevc_db1_slot: &mut f64,
        var_tdevc_dn0_slot: &mut f64,
        var_tdevc_dn1_slot: &mut f64,
        var_tdevc_dn2_slot: &mut f64,
        var_tdevc_dn3_slot: &mut f64,
        var_tdevc_dn4_slot: &mut f64,
        var_tdevc_dn5_slot: &mut f64,
        var_tdevc_rdb0_slot: &mut f64,
        var_tdevc_rdb1_slot: &mut f64,
        var_tdevc_rdn0_slot: &mut f64,
        var_tdevc_rdn1_slot: &mut f64,
        var_tdevc_rdn2_slot: &mut f64,
        var_tdevc_rdn3_slot: &mut f64,
        var_tdevc_rdn4_slot: &mut f64,
        var_tdevc_rdn5_slot: &mut f64,
        var_tdevc_rv_slot: &mut f64,
        var_tdevk_slot: &mut f64,
        var_tdevk_db0_slot: &mut f64,
        var_tdevk_db1_slot: &mut f64,
        var_tdevk_dn0_slot: &mut f64,
        var_tdevk_dn1_slot: &mut f64,
        var_tdevk_dn2_slot: &mut f64,
        var_tdevk_dn3_slot: &mut f64,
        var_tdevk_dn4_slot: &mut f64,
        var_tdevk_dn5_slot: &mut f64,
        var_tdevk_rdb0_slot: &mut f64,
        var_tdevk_rdb1_slot: &mut f64,
        var_tdevk_rdn0_slot: &mut f64,
        var_tdevk_rdn1_slot: &mut f64,
        var_tdevk_rdn2_slot: &mut f64,
        var_tdevk_rdn3_slot: &mut f64,
        var_tdevk_rdn4_slot: &mut f64,
        var_tdevk_rdn5_slot: &mut f64,
        var_tdevk_rv_slot: &mut f64,
    ) {
        let mut var_cja_t: f64 = *var_cja_t_slot;
        let mut var_cja_t_db0: f64 = *var_cja_t_db0_slot;
        let mut var_cja_t_db1: f64 = *var_cja_t_db1_slot;
        let mut var_cja_t_dn0: f64 = *var_cja_t_dn0_slot;
        let mut var_cja_t_dn1: f64 = *var_cja_t_dn1_slot;
        let mut var_cja_t_dn2: f64 = *var_cja_t_dn2_slot;
        let mut var_cja_t_dn3: f64 = *var_cja_t_dn3_slot;
        let mut var_cja_t_dn4: f64 = *var_cja_t_dn4_slot;
        let mut var_cja_t_dn5: f64 = *var_cja_t_dn5_slot;
        let mut var_cja_t_rdb0: f64 = *var_cja_t_rdb0_slot;
        let mut var_cja_t_rdb1: f64 = *var_cja_t_rdb1_slot;
        let mut var_cja_t_rdn0: f64 = *var_cja_t_rdn0_slot;
        let mut var_cja_t_rdn1: f64 = *var_cja_t_rdn1_slot;
        let mut var_cja_t_rdn2: f64 = *var_cja_t_rdn2_slot;
        let mut var_cja_t_rdn3: f64 = *var_cja_t_rdn3_slot;
        let mut var_cja_t_rdn4: f64 = *var_cja_t_rdn4_slot;
        let mut var_cja_t_rdn5: f64 = *var_cja_t_rdn5_slot;
        let mut var_cja_t_rv: f64 = *var_cja_t_rv_slot;
        let mut var_dt: f64 = *var_dt_slot;
        let mut var_dt_db0: f64 = *var_dt_db0_slot;
        let mut var_dt_db1: f64 = *var_dt_db1_slot;
        let mut var_dt_dn0: f64 = *var_dt_dn0_slot;
        let mut var_dt_dn1: f64 = *var_dt_dn1_slot;
        let mut var_dt_dn2: f64 = *var_dt_dn2_slot;
        let mut var_dt_dn3: f64 = *var_dt_dn3_slot;
        let mut var_dt_dn4: f64 = *var_dt_dn4_slot;
        let mut var_dt_dn5: f64 = *var_dt_dn5_slot;
        let mut var_dt_rdb0: f64 = *var_dt_rdb0_slot;
        let mut var_dt_rdb1: f64 = *var_dt_rdb1_slot;
        let mut var_dt_rdn0: f64 = *var_dt_rdn0_slot;
        let mut var_dt_rdn1: f64 = *var_dt_rdn1_slot;
        let mut var_dt_rdn2: f64 = *var_dt_rdn2_slot;
        let mut var_dt_rdn3: f64 = *var_dt_rdn3_slot;
        let mut var_dt_rdn4: f64 = *var_dt_rdn4_slot;
        let mut var_dt_rdn5: f64 = *var_dt_rdn5_slot;
        let mut var_dt_rv: f64 = *var_dt_rv_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_db0: f64 = *var_guard126_db0_slot;
        let mut var_guard126_db1: f64 = *var_guard126_db1_slot;
        let mut var_guard126_dn0: f64 = *var_guard126_dn0_slot;
        let mut var_guard126_dn1: f64 = *var_guard126_dn1_slot;
        let mut var_guard126_dn2: f64 = *var_guard126_dn2_slot;
        let mut var_guard126_dn3: f64 = *var_guard126_dn3_slot;
        let mut var_guard126_dn4: f64 = *var_guard126_dn4_slot;
        let mut var_guard126_dn5: f64 = *var_guard126_dn5_slot;
        let mut var_guard126_rdb0: f64 = *var_guard126_rdb0_slot;
        let mut var_guard126_rdb1: f64 = *var_guard126_rdb1_slot;
        let mut var_guard126_rdn0: f64 = *var_guard126_rdn0_slot;
        let mut var_guard126_rdn1: f64 = *var_guard126_rdn1_slot;
        let mut var_guard126_rdn2: f64 = *var_guard126_rdn2_slot;
        let mut var_guard126_rdn3: f64 = *var_guard126_rdn3_slot;
        let mut var_guard126_rdn4: f64 = *var_guard126_rdn4_slot;
        let mut var_guard126_rdn5: f64 = *var_guard126_rdn5_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_db0: f64 = *var_guard130_db0_slot;
        let mut var_guard130_db1: f64 = *var_guard130_db1_slot;
        let mut var_guard130_dn0: f64 = *var_guard130_dn0_slot;
        let mut var_guard130_dn1: f64 = *var_guard130_dn1_slot;
        let mut var_guard130_dn2: f64 = *var_guard130_dn2_slot;
        let mut var_guard130_dn3: f64 = *var_guard130_dn3_slot;
        let mut var_guard130_dn4: f64 = *var_guard130_dn4_slot;
        let mut var_guard130_dn5: f64 = *var_guard130_dn5_slot;
        let mut var_guard130_rdb0: f64 = *var_guard130_rdb0_slot;
        let mut var_guard130_rdb1: f64 = *var_guard130_rdb1_slot;
        let mut var_guard130_rdn0: f64 = *var_guard130_rdn0_slot;
        let mut var_guard130_rdn1: f64 = *var_guard130_rdn1_slot;
        let mut var_guard130_rdn2: f64 = *var_guard130_rdn2_slot;
        let mut var_guard130_rdn3: f64 = *var_guard130_rdn3_slot;
        let mut var_guard130_rdn4: f64 = *var_guard130_rdn4_slot;
        let mut var_guard130_rdn5: f64 = *var_guard130_rdn5_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_db0: f64 = *var_guard133_db0_slot;
        let mut var_guard133_db1: f64 = *var_guard133_db1_slot;
        let mut var_guard133_dn0: f64 = *var_guard133_dn0_slot;
        let mut var_guard133_dn1: f64 = *var_guard133_dn1_slot;
        let mut var_guard133_dn2: f64 = *var_guard133_dn2_slot;
        let mut var_guard133_dn3: f64 = *var_guard133_dn3_slot;
        let mut var_guard133_dn4: f64 = *var_guard133_dn4_slot;
        let mut var_guard133_dn5: f64 = *var_guard133_dn5_slot;
        let mut var_guard133_rdb0: f64 = *var_guard133_rdb0_slot;
        let mut var_guard133_rdb1: f64 = *var_guard133_rdb1_slot;
        let mut var_guard133_rdn0: f64 = *var_guard133_rdn0_slot;
        let mut var_guard133_rdn1: f64 = *var_guard133_rdn1_slot;
        let mut var_guard133_rdn2: f64 = *var_guard133_rdn2_slot;
        let mut var_guard133_rdn3: f64 = *var_guard133_rdn3_slot;
        let mut var_guard133_rdn4: f64 = *var_guard133_rdn4_slot;
        let mut var_guard133_rdn5: f64 = *var_guard133_rdn5_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_pa_t: f64 = *var_pa_t_slot;
        let mut var_pa_t_db0: f64 = *var_pa_t_db0_slot;
        let mut var_pa_t_db1: f64 = *var_pa_t_db1_slot;
        let mut var_pa_t_dn0: f64 = *var_pa_t_dn0_slot;
        let mut var_pa_t_dn1: f64 = *var_pa_t_dn1_slot;
        let mut var_pa_t_dn2: f64 = *var_pa_t_dn2_slot;
        let mut var_pa_t_dn3: f64 = *var_pa_t_dn3_slot;
        let mut var_pa_t_dn4: f64 = *var_pa_t_dn4_slot;
        let mut var_pa_t_dn5: f64 = *var_pa_t_dn5_slot;
        let mut var_pa_t_rdb0: f64 = *var_pa_t_rdb0_slot;
        let mut var_pa_t_rdb1: f64 = *var_pa_t_rdb1_slot;
        let mut var_pa_t_rdn0: f64 = *var_pa_t_rdn0_slot;
        let mut var_pa_t_rdn1: f64 = *var_pa_t_rdn1_slot;
        let mut var_pa_t_rdn2: f64 = *var_pa_t_rdn2_slot;
        let mut var_pa_t_rdn3: f64 = *var_pa_t_rdn3_slot;
        let mut var_pa_t_rdn4: f64 = *var_pa_t_rdn4_slot;
        let mut var_pa_t_rdn5: f64 = *var_pa_t_rdn5_slot;
        let mut var_pa_t_rv: f64 = *var_pa_t_rv_slot;
        let mut var_phi_t: f64 = *var_phi_t_slot;
        let mut var_phi_t_db0: f64 = *var_phi_t_db0_slot;
        let mut var_phi_t_db1: f64 = *var_phi_t_db1_slot;
        let mut var_phi_t_dn0: f64 = *var_phi_t_dn0_slot;
        let mut var_phi_t_dn1: f64 = *var_phi_t_dn1_slot;
        let mut var_phi_t_dn2: f64 = *var_phi_t_dn2_slot;
        let mut var_phi_t_dn3: f64 = *var_phi_t_dn3_slot;
        let mut var_phi_t_dn4: f64 = *var_phi_t_dn4_slot;
        let mut var_phi_t_dn5: f64 = *var_phi_t_dn5_slot;
        let mut var_phi_t_rdb0: f64 = *var_phi_t_rdb0_slot;
        let mut var_phi_t_rdb1: f64 = *var_phi_t_rdb1_slot;
        let mut var_phi_t_rdn0: f64 = *var_phi_t_rdn0_slot;
        let mut var_phi_t_rdn1: f64 = *var_phi_t_rdn1_slot;
        let mut var_phi_t_rdn2: f64 = *var_phi_t_rdn2_slot;
        let mut var_phi_t_rdn3: f64 = *var_phi_t_rdn3_slot;
        let mut var_phi_t_rdn4: f64 = *var_phi_t_rdn4_slot;
        let mut var_phi_t_rdn5: f64 = *var_phi_t_rdn5_slot;
        let mut var_phi_t_rv: f64 = *var_phi_t_rv_slot;
        let mut var_psiin: f64 = *var_psiin_slot;
        let mut var_psiin_db0: f64 = *var_psiin_db0_slot;
        let mut var_psiin_db1: f64 = *var_psiin_db1_slot;
        let mut var_psiin_dn0: f64 = *var_psiin_dn0_slot;
        let mut var_psiin_dn1: f64 = *var_psiin_dn1_slot;
        let mut var_psiin_dn2: f64 = *var_psiin_dn2_slot;
        let mut var_psiin_dn3: f64 = *var_psiin_dn3_slot;
        let mut var_psiin_dn4: f64 = *var_psiin_dn4_slot;
        let mut var_psiin_dn5: f64 = *var_psiin_dn5_slot;
        let mut var_psiin_rdb0: f64 = *var_psiin_rdb0_slot;
        let mut var_psiin_rdb1: f64 = *var_psiin_rdb1_slot;
        let mut var_psiin_rdn0: f64 = *var_psiin_rdn0_slot;
        let mut var_psiin_rdn1: f64 = *var_psiin_rdn1_slot;
        let mut var_psiin_rdn2: f64 = *var_psiin_rdn2_slot;
        let mut var_psiin_rdn3: f64 = *var_psiin_rdn3_slot;
        let mut var_psiin_rdn4: f64 = *var_psiin_rdn4_slot;
        let mut var_psiin_rdn5: f64 = *var_psiin_rdn5_slot;
        let mut var_psiin_rv: f64 = *var_psiin_rv_slot;
        let mut var_psiio: f64 = *var_psiio_slot;
        let mut var_psiio_db0: f64 = *var_psiio_db0_slot;
        let mut var_psiio_db1: f64 = *var_psiio_db1_slot;
        let mut var_psiio_dn0: f64 = *var_psiio_dn0_slot;
        let mut var_psiio_dn1: f64 = *var_psiio_dn1_slot;
        let mut var_psiio_dn2: f64 = *var_psiio_dn2_slot;
        let mut var_psiio_dn3: f64 = *var_psiio_dn3_slot;
        let mut var_psiio_dn4: f64 = *var_psiio_dn4_slot;
        let mut var_psiio_dn5: f64 = *var_psiio_dn5_slot;
        let mut var_psiio_rdb0: f64 = *var_psiio_rdb0_slot;
        let mut var_psiio_rdb1: f64 = *var_psiio_rdb1_slot;
        let mut var_psiio_rdn0: f64 = *var_psiio_rdn0_slot;
        let mut var_psiio_rdn1: f64 = *var_psiio_rdn1_slot;
        let mut var_psiio_rdn2: f64 = *var_psiio_rdn2_slot;
        let mut var_psiio_rdn3: f64 = *var_psiio_rdn3_slot;
        let mut var_psiio_rdn4: f64 = *var_psiio_rdn4_slot;
        let mut var_psiio_rdn5: f64 = *var_psiio_rdn5_slot;
        let mut var_psiio_rv: f64 = *var_psiio_rv_slot;
        let mut var_rt: f64 = *var_rt_slot;
        let mut var_rt_db0: f64 = *var_rt_db0_slot;
        let mut var_rt_db1: f64 = *var_rt_db1_slot;
        let mut var_rt_dn0: f64 = *var_rt_dn0_slot;
        let mut var_rt_dn1: f64 = *var_rt_dn1_slot;
        let mut var_rt_dn2: f64 = *var_rt_dn2_slot;
        let mut var_rt_dn3: f64 = *var_rt_dn3_slot;
        let mut var_rt_dn4: f64 = *var_rt_dn4_slot;
        let mut var_rt_dn5: f64 = *var_rt_dn5_slot;
        let mut var_rt_rdb0: f64 = *var_rt_rdb0_slot;
        let mut var_rt_rdb1: f64 = *var_rt_rdb1_slot;
        let mut var_rt_rdn0: f64 = *var_rt_rdn0_slot;
        let mut var_rt_rdn1: f64 = *var_rt_rdn1_slot;
        let mut var_rt_rdn2: f64 = *var_rt_rdn2_slot;
        let mut var_rt_rdn3: f64 = *var_rt_rdn3_slot;
        let mut var_rt_rdn4: f64 = *var_rt_rdn4_slot;
        let mut var_rt_rdn5: f64 = *var_rt_rdn5_slot;
        let mut var_rt_rv: f64 = *var_rt_rv_slot;
        let mut var_tcr: f64 = *var_tcr_slot;
        let mut var_tcr_db0: f64 = *var_tcr_db0_slot;
        let mut var_tcr_db1: f64 = *var_tcr_db1_slot;
        let mut var_tcr_dn0: f64 = *var_tcr_dn0_slot;
        let mut var_tcr_dn1: f64 = *var_tcr_dn1_slot;
        let mut var_tcr_dn2: f64 = *var_tcr_dn2_slot;
        let mut var_tcr_dn3: f64 = *var_tcr_dn3_slot;
        let mut var_tcr_dn4: f64 = *var_tcr_dn4_slot;
        let mut var_tcr_dn5: f64 = *var_tcr_dn5_slot;
        let mut var_tcr_rdb0: f64 = *var_tcr_rdb0_slot;
        let mut var_tcr_rdb1: f64 = *var_tcr_rdb1_slot;
        let mut var_tcr_rdn0: f64 = *var_tcr_rdn0_slot;
        let mut var_tcr_rdn1: f64 = *var_tcr_rdn1_slot;
        let mut var_tcr_rdn2: f64 = *var_tcr_rdn2_slot;
        let mut var_tcr_rdn3: f64 = *var_tcr_rdn3_slot;
        let mut var_tcr_rdn4: f64 = *var_tcr_rdn4_slot;
        let mut var_tcr_rdn5: f64 = *var_tcr_rdn5_slot;
        let mut var_tcr_rv: f64 = *var_tcr_rv_slot;
        let mut var_tcvsat: f64 = *var_tcvsat_slot;
        let mut var_tcvsat_db0: f64 = *var_tcvsat_db0_slot;
        let mut var_tcvsat_db1: f64 = *var_tcvsat_db1_slot;
        let mut var_tcvsat_dn0: f64 = *var_tcvsat_dn0_slot;
        let mut var_tcvsat_dn1: f64 = *var_tcvsat_dn1_slot;
        let mut var_tcvsat_dn2: f64 = *var_tcvsat_dn2_slot;
        let mut var_tcvsat_dn3: f64 = *var_tcvsat_dn3_slot;
        let mut var_tcvsat_dn4: f64 = *var_tcvsat_dn4_slot;
        let mut var_tcvsat_dn5: f64 = *var_tcvsat_dn5_slot;
        let mut var_tcvsat_rdb0: f64 = *var_tcvsat_rdb0_slot;
        let mut var_tcvsat_rdb1: f64 = *var_tcvsat_rdb1_slot;
        let mut var_tcvsat_rdn0: f64 = *var_tcvsat_rdn0_slot;
        let mut var_tcvsat_rdn1: f64 = *var_tcvsat_rdn1_slot;
        let mut var_tcvsat_rdn2: f64 = *var_tcvsat_rdn2_slot;
        let mut var_tcvsat_rdn3: f64 = *var_tcvsat_rdn3_slot;
        let mut var_tcvsat_rdn4: f64 = *var_tcvsat_rdn4_slot;
        let mut var_tcvsat_rdn5: f64 = *var_tcvsat_rdn5_slot;
        let mut var_tcvsat_rv: f64 = *var_tcvsat_rv_slot;
        let mut var_tdevc: f64 = *var_tdevc_slot;
        let mut var_tdevc_db0: f64 = *var_tdevc_db0_slot;
        let mut var_tdevc_db1: f64 = *var_tdevc_db1_slot;
        let mut var_tdevc_dn0: f64 = *var_tdevc_dn0_slot;
        let mut var_tdevc_dn1: f64 = *var_tdevc_dn1_slot;
        let mut var_tdevc_dn2: f64 = *var_tdevc_dn2_slot;
        let mut var_tdevc_dn3: f64 = *var_tdevc_dn3_slot;
        let mut var_tdevc_dn4: f64 = *var_tdevc_dn4_slot;
        let mut var_tdevc_dn5: f64 = *var_tdevc_dn5_slot;
        let mut var_tdevc_rdb0: f64 = *var_tdevc_rdb0_slot;
        let mut var_tdevc_rdb1: f64 = *var_tdevc_rdb1_slot;
        let mut var_tdevc_rdn0: f64 = *var_tdevc_rdn0_slot;
        let mut var_tdevc_rdn1: f64 = *var_tdevc_rdn1_slot;
        let mut var_tdevc_rdn2: f64 = *var_tdevc_rdn2_slot;
        let mut var_tdevc_rdn3: f64 = *var_tdevc_rdn3_slot;
        let mut var_tdevc_rdn4: f64 = *var_tdevc_rdn4_slot;
        let mut var_tdevc_rdn5: f64 = *var_tdevc_rdn5_slot;
        let mut var_tdevc_rv: f64 = *var_tdevc_rv_slot;
        let mut var_tdevk: f64 = *var_tdevk_slot;
        let mut var_tdevk_db0: f64 = *var_tdevk_db0_slot;
        let mut var_tdevk_db1: f64 = *var_tdevk_db1_slot;
        let mut var_tdevk_dn0: f64 = *var_tdevk_dn0_slot;
        let mut var_tdevk_dn1: f64 = *var_tdevk_dn1_slot;
        let mut var_tdevk_dn2: f64 = *var_tdevk_dn2_slot;
        let mut var_tdevk_dn3: f64 = *var_tdevk_dn3_slot;
        let mut var_tdevk_dn4: f64 = *var_tdevk_dn4_slot;
        let mut var_tdevk_dn5: f64 = *var_tdevk_dn5_slot;
        let mut var_tdevk_rdb0: f64 = *var_tdevk_rdb0_slot;
        let mut var_tdevk_rdb1: f64 = *var_tdevk_rdb1_slot;
        let mut var_tdevk_rdn0: f64 = *var_tdevk_rdn0_slot;
        let mut var_tdevk_rdn1: f64 = *var_tdevk_rdn1_slot;
        let mut var_tdevk_rdn2: f64 = *var_tdevk_rdn2_slot;
        let mut var_tdevk_rdn3: f64 = *var_tdevk_rdn3_slot;
        let mut var_tdevk_rdn4: f64 = *var_tdevk_rdn4_slot;
        let mut var_tdevk_rdn5: f64 = *var_tdevk_rdn5_slot;
        let mut var_tdevk_rv: f64 = *var_tdevk_rv_slot;

        let (assign1160_e1178, assign1160_e1178_d_n0, assign1160_e1178_d_n1, assign1160_e1178_d_n2, assign1160_e1178_d_n3, assign1160_e1178_d_n4, assign1160_e1178_d_n5, assign1160_e1178_d_b0, assign1160_e1178_d_b1,) = {
    if ((var_guard124 == 0.0) && (var_guard125 != 0.0)) {
        let assign1160_e1172: f64 = (p.p36 - var_tdevc);
        let assign1160_e1174: f64 = (assign1160_e1172 - 1.0);
        let assign1160_e1175: f64 = (assign1160_e1174).exp();
        let assign1160_e1176: f64 = (p.p36 - assign1160_e1175);
        (assign1160_e1176, (-(assign1160_e1175 * (-var_tdevc_dn0))), (-(assign1160_e1175 * (-var_tdevc_dn1))), (-(assign1160_e1175 * (-var_tdevc_dn2))), (-(assign1160_e1175 * (-var_tdevc_dn3))), (-(assign1160_e1175 * (-var_tdevc_dn4))), (-(assign1160_e1175 * (-var_tdevc_dn5))), (-(assign1160_e1175 * (-var_tdevc_db0))), (-(assign1160_e1175 * (-var_tdevc_db1))),)
    } else {
        (var_tdevc, var_tdevc_dn0, var_tdevc_dn1, var_tdevc_dn2, var_tdevc_dn3, var_tdevc_dn4, var_tdevc_dn5, var_tdevc_db0, var_tdevc_db1,)
    }
};
        var_tdevc = assign1160_e1178;
        var_tdevc_dn0 = assign1160_e1178_d_n0;
        var_tdevc_dn1 = assign1160_e1178_d_n1;
        var_tdevc_dn2 = assign1160_e1178_d_n2;
        var_tdevc_dn3 = assign1160_e1178_d_n3;
        var_tdevc_dn4 = assign1160_e1178_d_n4;
        var_tdevc_dn5 = assign1160_e1178_d_n5;
        var_tdevc_db0 = assign1160_e1178_d_b0;
        var_tdevc_db1 = assign1160_e1178_d_b1;
        var_tdevc_rv = 0.0;
        var_tdevc_rdn0 = 0.0;
        var_tdevc_rdn1 = 0.0;
        var_tdevc_rdn2 = 0.0;
        var_tdevc_rdn3 = 0.0;
        var_tdevc_rdn4 = 0.0;
        var_tdevc_rdn5 = 0.0;
        var_tdevc_rdb0 = 0.0;
        var_tdevc_rdb1 = 0.0;

        let (assign1170_e1186, assign1170_e1186_d_n0, assign1170_e1186_d_n1, assign1170_e1186_d_n2, assign1170_e1186_d_n3, assign1170_e1186_d_n4, assign1170_e1186_d_n5, assign1170_e1186_d_b0, assign1170_e1186_d_b1,) = {
    if ((var_guard124 == 0.0) && (var_guard125 == 0.0)) {
        (var_tdevc, var_tdevc_dn0, var_tdevc_dn1, var_tdevc_dn2, var_tdevc_dn3, var_tdevc_dn4, var_tdevc_dn5, var_tdevc_db0, var_tdevc_db1,)
    } else {
        (var_tdevc, var_tdevc_dn0, var_tdevc_dn1, var_tdevc_dn2, var_tdevc_dn3, var_tdevc_dn4, var_tdevc_dn5, var_tdevc_db0, var_tdevc_db1,)
    }
};
        var_tdevc = assign1170_e1186;
        var_tdevc_dn0 = assign1170_e1186_d_n0;
        var_tdevc_dn1 = assign1170_e1186_d_n1;
        var_tdevc_dn2 = assign1170_e1186_d_n2;
        var_tdevc_dn3 = assign1170_e1186_d_n3;
        var_tdevc_dn4 = assign1170_e1186_d_n4;
        var_tdevc_dn5 = assign1170_e1186_d_n5;
        var_tdevc_db0 = assign1170_e1186_d_b0;
        var_tdevc_db1 = assign1170_e1186_d_b1;
        var_tdevc_rv = 0.0;
        var_tdevc_rdn0 = 0.0;
        var_tdevc_rdn1 = 0.0;
        var_tdevc_rdn2 = 0.0;
        var_tdevc_rdn3 = 0.0;
        var_tdevc_rdn4 = 0.0;
        var_tdevc_rdn5 = 0.0;
        var_tdevc_rdb0 = 0.0;
        var_tdevc_rdb1 = 0.0;

        let assign1180_e1189: f64 = (var_tdevc + 273.15);
        var_tdevk = assign1180_e1189;
        var_tdevk_dn0 = var_tdevc_dn0;
        var_tdevk_dn1 = var_tdevc_dn1;
        var_tdevk_dn2 = var_tdevc_dn2;
        var_tdevk_dn3 = var_tdevc_dn3;
        var_tdevk_dn4 = var_tdevc_dn4;
        var_tdevk_dn5 = var_tdevc_dn5;
        var_tdevk_db0 = var_tdevc_db0;
        var_tdevk_db1 = var_tdevc_db1;
        var_tdevk_rv = 0.0;
        var_tdevk_rdn0 = 0.0;
        var_tdevk_rdn1 = 0.0;
        var_tdevk_rdn2 = 0.0;
        var_tdevk_rdn3 = 0.0;
        var_tdevk_rdn4 = 0.0;
        var_tdevk_rdn5 = 0.0;
        var_tdevk_rdb0 = 0.0;
        var_tdevk_rdb1 = 0.0;

        let assign1190_e1192: f64 = (1.3806505e-23 * var_tdevk);
        let assign1190_e1194: f64 = (assign1190_e1192 / 1.60217653e-19);
        var_phi_t = assign1190_e1194;
        var_phi_t_dn0 = ((1.3806505e-23 * var_tdevk_dn0) / 1.60217653e-19);
        var_phi_t_dn1 = ((1.3806505e-23 * var_tdevk_dn1) / 1.60217653e-19);
        var_phi_t_dn2 = ((1.3806505e-23 * var_tdevk_dn2) / 1.60217653e-19);
        var_phi_t_dn3 = ((1.3806505e-23 * var_tdevk_dn3) / 1.60217653e-19);
        var_phi_t_dn4 = ((1.3806505e-23 * var_tdevk_dn4) / 1.60217653e-19);
        var_phi_t_dn5 = ((1.3806505e-23 * var_tdevk_dn5) / 1.60217653e-19);
        var_phi_t_db0 = ((1.3806505e-23 * var_tdevk_db0) / 1.60217653e-19);
        var_phi_t_db1 = ((1.3806505e-23 * var_tdevk_db1) / 1.60217653e-19);
        var_phi_t_rv = 0.0;
        var_phi_t_rdn0 = 0.0;
        var_phi_t_rdn1 = 0.0;
        var_phi_t_rdn2 = 0.0;
        var_phi_t_rdn3 = 0.0;
        var_phi_t_rdn4 = 0.0;
        var_phi_t_rdn5 = 0.0;
        var_phi_t_rdb0 = 0.0;
        var_phi_t_rdb1 = 0.0;

        let assign1200_e1197: f64 = (var_tdevk / var_tinik);
        var_rt = assign1200_e1197;
        var_rt_dn0 = (((var_tdevk_dn0 * var_tinik) - (var_tdevk * var_tinik_dn0)) / (var_tinik * var_tinik));
        var_rt_dn1 = (((var_tdevk_dn1 * var_tinik) - (var_tdevk * var_tinik_dn1)) / (var_tinik * var_tinik));
        var_rt_dn2 = (((var_tdevk_dn2 * var_tinik) - (var_tdevk * var_tinik_dn2)) / (var_tinik * var_tinik));
        var_rt_dn3 = (((var_tdevk_dn3 * var_tinik) - (var_tdevk * var_tinik_dn3)) / (var_tinik * var_tinik));
        var_rt_dn4 = (((var_tdevk_dn4 * var_tinik) - (var_tdevk * var_tinik_dn4)) / (var_tinik * var_tinik));
        var_rt_dn5 = (((var_tdevk_dn5 * var_tinik) - (var_tdevk * var_tinik_dn5)) / (var_tinik * var_tinik));
        var_rt_db0 = (((var_tdevk_db0 * var_tinik) - (var_tdevk * var_tinik_db0)) / (var_tinik * var_tinik));
        var_rt_db1 = (((var_tdevk_db1 * var_tinik) - (var_tdevk * var_tinik_db1)) / (var_tinik * var_tinik));
        var_rt_rv = 0.0;
        var_rt_rdn0 = 0.0;
        var_rt_rdn1 = 0.0;
        var_rt_rdn2 = 0.0;
        var_rt_rdn3 = 0.0;
        var_rt_rdn4 = 0.0;
        var_rt_rdn5 = 0.0;
        var_rt_rdb0 = 0.0;
        var_rt_rdb1 = 0.0;

        let assign1210_e1200: f64 = (var_tdevk - var_tinik);
        var_dt = assign1210_e1200;
        var_dt_dn0 = (var_tdevk_dn0 - var_tinik_dn0);
        var_dt_dn1 = (var_tdevk_dn1 - var_tinik_dn1);
        var_dt_dn2 = (var_tdevk_dn2 - var_tinik_dn2);
        var_dt_dn3 = (var_tdevk_dn3 - var_tinik_dn3);
        var_dt_dn4 = (var_tdevk_dn4 - var_tinik_dn4);
        var_dt_dn5 = (var_tdevk_dn5 - var_tinik_dn5);
        var_dt_db0 = (var_tdevk_db0 - var_tinik_db0);
        var_dt_db1 = (var_tdevk_db1 - var_tinik_db1);
        var_dt_rv = 0.0;
        var_dt_rdn0 = 0.0;
        var_dt_rdn1 = 0.0;
        var_dt_rdn2 = 0.0;
        var_dt_rdn3 = 0.0;
        var_dt_rdn4 = 0.0;
        var_dt_rdn5 = 0.0;
        var_dt_rdb0 = 0.0;
        var_dt_rdb1 = 0.0;

        let assign1220_e1206: f64 = (var_dt * var_tc2e);
        let assign1220_e1207: f64 = (var_tc1e + assign1220_e1206);
        let assign1220_e1208: f64 = (var_dt * assign1220_e1207);
        let assign1220_e1209: f64 = (1.0 + assign1220_e1208);
        var_tcr = assign1220_e1209;
        var_tcr_dn0 = ((var_dt_dn0 * assign1220_e1207) + (var_dt * (var_tc1e_dn0 + ((var_dt_dn0 * var_tc2e) + (var_dt * var_tc2e_dn0)))));
        var_tcr_dn1 = ((var_dt_dn1 * assign1220_e1207) + (var_dt * (var_tc1e_dn1 + ((var_dt_dn1 * var_tc2e) + (var_dt * var_tc2e_dn1)))));
        var_tcr_dn2 = ((var_dt_dn2 * assign1220_e1207) + (var_dt * (var_tc1e_dn2 + ((var_dt_dn2 * var_tc2e) + (var_dt * var_tc2e_dn2)))));
        var_tcr_dn3 = ((var_dt_dn3 * assign1220_e1207) + (var_dt * (var_tc1e_dn3 + ((var_dt_dn3 * var_tc2e) + (var_dt * var_tc2e_dn3)))));
        var_tcr_dn4 = ((var_dt_dn4 * assign1220_e1207) + (var_dt * (var_tc1e_dn4 + ((var_dt_dn4 * var_tc2e) + (var_dt * var_tc2e_dn4)))));
        var_tcr_dn5 = ((var_dt_dn5 * assign1220_e1207) + (var_dt * (var_tc1e_dn5 + ((var_dt_dn5 * var_tc2e) + (var_dt * var_tc2e_dn5)))));
        var_tcr_db0 = ((var_dt_db0 * assign1220_e1207) + (var_dt * (var_tc1e_db0 + ((var_dt_db0 * var_tc2e) + (var_dt * var_tc2e_db0)))));
        var_tcr_db1 = ((var_dt_db1 * assign1220_e1207) + (var_dt * (var_tc1e_db1 + ((var_dt_db1 * var_tc2e) + (var_dt * var_tc2e_db1)))));
        var_tcr_rv = 0.0;
        var_tcr_rdn0 = 0.0;
        var_tcr_rdn1 = 0.0;
        var_tcr_rdn2 = 0.0;
        var_tcr_rdn3 = 0.0;
        var_tcr_rdn4 = 0.0;
        var_tcr_rdn5 = 0.0;
        var_tcr_rdb0 = 0.0;
        var_tcr_rdb1 = 0.0;

        let assign1230_e1213: f64 = (0.01 + 0.1);
        let assign1230_e1214: f64 = if var_tcr < assign1230_e1213 { 1.0 } else { 0.0 };
        var_guard126 = assign1230_e1214;
        var_guard126_dn0 = 0.0;
        var_guard126_dn1 = 0.0;
        var_guard126_dn2 = 0.0;
        var_guard126_dn3 = 0.0;
        var_guard126_dn4 = 0.0;
        var_guard126_dn5 = 0.0;
        var_guard126_db0 = 0.0;
        var_guard126_db1 = 0.0;
        var_guard126_rv = 0.0;
        var_guard126_rdn0 = 0.0;
        var_guard126_rdn1 = 0.0;
        var_guard126_rdn2 = 0.0;
        var_guard126_rdn3 = 0.0;
        var_guard126_rdn4 = 0.0;
        var_guard126_rdn5 = 0.0;
        var_guard126_rdb0 = 0.0;
        var_guard126_rdb1 = 0.0;

        let (assign1240_e1229, assign1240_e1229_d_n0, assign1240_e1229_d_n1, assign1240_e1229_d_n2, assign1240_e1229_d_n3, assign1240_e1229_d_n4, assign1240_e1229_d_n5, assign1240_e1229_d_b0, assign1240_e1229_d_b1,) = {
    if (var_guard126 != 0.0) {
        let assign1240_e1221: f64 = (var_tcr - 0.01);
        let assign1240_e1222: f64 = (10.0 * assign1240_e1221);
        let assign1240_e1224: f64 = (assign1240_e1222 - 1.0);
        let assign1240_e1225: f64 = (assign1240_e1224).exp();
        let assign1240_e1226: f64 = (0.1 * assign1240_e1225);
        let assign1240_e1227: f64 = (0.01 + assign1240_e1226);
        (assign1240_e1227, (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn0))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn1))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn2))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn3))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn4))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_dn5))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_db0))), (0.1 * (assign1240_e1225 * (10.0 * var_tcr_db1))),)
    } else {
        (var_tcr, var_tcr_dn0, var_tcr_dn1, var_tcr_dn2, var_tcr_dn3, var_tcr_dn4, var_tcr_dn5, var_tcr_db0, var_tcr_db1,)
    }
};
        var_tcr = assign1240_e1229;
        var_tcr_dn0 = assign1240_e1229_d_n0;
        var_tcr_dn1 = assign1240_e1229_d_n1;
        var_tcr_dn2 = assign1240_e1229_d_n2;
        var_tcr_dn3 = assign1240_e1229_d_n3;
        var_tcr_dn4 = assign1240_e1229_d_n4;
        var_tcr_dn5 = assign1240_e1229_d_n5;
        var_tcr_db0 = assign1240_e1229_d_b0;
        var_tcr_db1 = assign1240_e1229_d_b1;
        var_tcr_rv = 0.0;
        var_tcr_rdn0 = 0.0;
        var_tcr_rdn1 = 0.0;
        var_tcr_rdn2 = 0.0;
        var_tcr_rdn3 = 0.0;
        var_tcr_rdn4 = 0.0;
        var_tcr_rdn5 = 0.0;
        var_tcr_rdb0 = 0.0;
        var_tcr_rdb1 = 0.0;

        let (assign1250_e1234, assign1250_e1234_d_n0, assign1250_e1234_d_n1, assign1250_e1234_d_n2, assign1250_e1234_d_n3, assign1250_e1234_d_n4, assign1250_e1234_d_n5, assign1250_e1234_d_b0, assign1250_e1234_d_b1,) = {
    if (var_guard126 == 0.0) {
        (var_tcr, var_tcr_dn0, var_tcr_dn1, var_tcr_dn2, var_tcr_dn3, var_tcr_dn4, var_tcr_dn5, var_tcr_db0, var_tcr_db1,)
    } else {
        (var_tcr, var_tcr_dn0, var_tcr_dn1, var_tcr_dn2, var_tcr_dn3, var_tcr_dn4, var_tcr_dn5, var_tcr_db0, var_tcr_db1,)
    }
};
        var_tcr = assign1250_e1234;
        var_tcr_dn0 = assign1250_e1234_d_n0;
        var_tcr_dn1 = assign1250_e1234_d_n1;
        var_tcr_dn2 = assign1250_e1234_d_n2;
        var_tcr_dn3 = assign1250_e1234_d_n3;
        var_tcr_dn4 = assign1250_e1234_d_n4;
        var_tcr_dn5 = assign1250_e1234_d_n5;
        var_tcr_db0 = assign1250_e1234_d_b0;
        var_tcr_db1 = assign1250_e1234_d_b1;
        var_tcr_rv = 0.0;
        var_tcr_rdn0 = 0.0;
        var_tcr_rdn1 = 0.0;
        var_tcr_rdn2 = 0.0;
        var_tcr_rdn3 = 0.0;
        var_tcr_rdn4 = 0.0;
        var_tcr_rdn5 = 0.0;
        var_tcr_rdb0 = 0.0;
        var_tcr_rdb1 = 0.0;

        let assign1320_e1295: f64 = (var_rt).powf(p.p92);
        var_tcvsat = assign1320_e1295;
        var_tcvsat_dn0 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn0)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn0 / var_rt))) };
        var_tcvsat_dn1 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn1)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn1 / var_rt))) };
        var_tcvsat_dn2 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn2)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn2 / var_rt))) };
        var_tcvsat_dn3 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn3)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn3 / var_rt))) };
        var_tcvsat_dn4 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn4)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn4 / var_rt))) };
        var_tcvsat_dn5 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_dn5)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_dn5 / var_rt))) };
        var_tcvsat_db0 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_db0)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_db0 / var_rt))) };
        var_tcvsat_db1 = if 0.0 == 0.0 && ((p.p92) as f64).is_finite() && ((p.p92) as f64).fract() == 0.0 { if p.p92 == 0.0 { 0.0 } else { (p.p92 * ((var_rt).powf(p.p92 - 1.0) * var_rt_db1)) } } else { (assign1320_e1295 * (p.p92 * (var_rt_db1 / var_rt))) };
        var_tcvsat_rv = 0.0;
        var_tcvsat_rdn0 = 0.0;
        var_tcvsat_rdn1 = 0.0;
        var_tcvsat_rdn2 = 0.0;
        var_tcvsat_rdn3 = 0.0;
        var_tcvsat_rdn4 = 0.0;
        var_tcvsat_rdn5 = 0.0;
        var_tcvsat_rdb0 = 0.0;
        var_tcvsat_rdb1 = 0.0;

        let assign1450_e1406: f64 = if p.p72 > 0.0 { 1.0 } else { 0.0 };
        var_guard130 = assign1450_e1406;
        var_guard130_dn0 = 0.0;
        var_guard130_dn1 = 0.0;
        var_guard130_dn2 = 0.0;
        var_guard130_dn3 = 0.0;
        var_guard130_dn4 = 0.0;
        var_guard130_dn5 = 0.0;
        var_guard130_db0 = 0.0;
        var_guard130_db1 = 0.0;
        var_guard130_rv = 0.0;
        var_guard130_rdn0 = 0.0;
        var_guard130_rdn1 = 0.0;
        var_guard130_rdn2 = 0.0;
        var_guard130_rdn3 = 0.0;
        var_guard130_rdn4 = 0.0;
        var_guard130_rdn5 = 0.0;
        var_guard130_rdb0 = 0.0;
        var_guard130_rdb1 = 0.0;

        let (assign1460_e1434, assign1460_e1434_d_n0, assign1460_e1434_d_n1, assign1460_e1434_d_n2, assign1460_e1434_d_n3, assign1460_e1434_d_n4, assign1460_e1434_d_n5, assign1460_e1434_d_b0, assign1460_e1434_d_b1,) = {
    if (var_guard130 != 0.0) {
        let assign1460_e1411: f64 = (var_phi_t / var_rt);
        let assign1460_e1412: f64 = (2.0 * assign1460_e1411);
        let assign1460_e1415: f64 = (0.5 * p.p73);
        let assign1460_e1417: f64 = (assign1460_e1415 * var_rt);
        let assign1460_e1419: f64 = (assign1460_e1417 / var_phi_t);
        let assign1460_e1420: f64 = (assign1460_e1419).exp();
        let assign1460_e1422: f64 = (-0.5);
        let assign1460_e1424: f64 = (assign1460_e1422 * p.p73);
        let assign1460_e1426: f64 = (assign1460_e1424 * var_rt);
        let assign1460_e1428: f64 = (assign1460_e1426 / var_phi_t);
        let assign1460_e1429: f64 = (assign1460_e1428).exp();
        let assign1460_e1430: f64 = (assign1460_e1420 - assign1460_e1429);
        let assign1460_e1431: f64 = (assign1460_e1430).ln();
        let assign1460_e1432: f64 = (assign1460_e1412 * assign1460_e1431);
        (assign1460_e1432, (((2.0 * (((var_phi_t_dn0 * var_rt) - (var_phi_t * var_rt_dn0)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn0) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn0)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn0) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn0)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_dn1 * var_rt) - (var_phi_t * var_rt_dn1)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn1) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn1)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn1) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn1)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_dn2 * var_rt) - (var_phi_t * var_rt_dn2)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn2) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn2)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn2) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn2)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_dn3 * var_rt) - (var_phi_t * var_rt_dn3)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn3) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn3)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn3) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn3)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_dn4 * var_rt) - (var_phi_t * var_rt_dn4)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn4) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn4)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn4) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn4)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_dn5 * var_rt) - (var_phi_t * var_rt_dn5)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_dn5) * var_phi_t) - (assign1460_e1417 * var_phi_t_dn5)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_dn5) * var_phi_t) - (assign1460_e1426 * var_phi_t_dn5)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_db0 * var_rt) - (var_phi_t * var_rt_db0)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_db0) * var_phi_t) - (assign1460_e1417 * var_phi_t_db0)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_db0) * var_phi_t) - (assign1460_e1426 * var_phi_t_db0)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))), (((2.0 * (((var_phi_t_db1 * var_rt) - (var_phi_t * var_rt_db1)) / (var_rt * var_rt))) * assign1460_e1431) + (assign1460_e1412 * (((assign1460_e1420 * ((((assign1460_e1415 * var_rt_db1) * var_phi_t) - (assign1460_e1417 * var_phi_t_db1)) / (var_phi_t * var_phi_t))) - (assign1460_e1429 * ((((assign1460_e1424 * var_rt_db1) * var_phi_t) - (assign1460_e1426 * var_phi_t_db1)) / (var_phi_t * var_phi_t)))) / assign1460_e1430))),)
    } else {
        (var_psiio, var_psiio_dn0, var_psiio_dn1, var_psiio_dn2, var_psiio_dn3, var_psiio_dn4, var_psiio_dn5, var_psiio_db0, var_psiio_db1,)
    }
};
        var_psiio = assign1460_e1434;
        var_psiio_dn0 = assign1460_e1434_d_n0;
        var_psiio_dn1 = assign1460_e1434_d_n1;
        var_psiio_dn2 = assign1460_e1434_d_n2;
        var_psiio_dn3 = assign1460_e1434_d_n3;
        var_psiio_dn4 = assign1460_e1434_d_n4;
        var_psiio_dn5 = assign1460_e1434_d_n5;
        var_psiio_db0 = assign1460_e1434_d_b0;
        var_psiio_db1 = assign1460_e1434_d_b1;
        var_psiio_rv = 0.0;
        var_psiio_rdn0 = 0.0;
        var_psiio_rdn1 = 0.0;
        var_psiio_rdn2 = 0.0;
        var_psiio_rdn3 = 0.0;
        var_psiio_rdn4 = 0.0;
        var_psiio_rdn5 = 0.0;
        var_psiio_rdb0 = 0.0;
        var_psiio_rdb1 = 0.0;

        let (assign1470_e1453, assign1470_e1453_d_n0, assign1470_e1453_d_n1, assign1470_e1453_d_n2, assign1470_e1453_d_n3, assign1470_e1453_d_n4, assign1470_e1453_d_n5, assign1470_e1453_d_b0, assign1470_e1453_d_b1,) = {
    if (var_guard130 != 0.0) {
        let assign1470_e1438: f64 = (var_psiio * var_rt);
        let assign1470_e1441: f64 = (3.0 * var_phi_t);
        let assign1470_e1443: f64 = (var_rt).ln();
        let assign1470_e1444: f64 = (assign1470_e1441 * assign1470_e1443);
        let assign1470_e1445: f64 = (assign1470_e1438 - assign1470_e1444);
        let assign1470_e1449: f64 = (var_rt - 1.0);
        let assign1470_e1450: f64 = (p.p90 * assign1470_e1449);
        let assign1470_e1451: f64 = (assign1470_e1445 - assign1470_e1450);
        (assign1470_e1451, ((((var_psiio_dn0 * var_rt) + (var_psiio * var_rt_dn0)) - (((3.0 * var_phi_t_dn0) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn0 / var_rt)))) - (p.p90 * var_rt_dn0)), ((((var_psiio_dn1 * var_rt) + (var_psiio * var_rt_dn1)) - (((3.0 * var_phi_t_dn1) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn1 / var_rt)))) - (p.p90 * var_rt_dn1)), ((((var_psiio_dn2 * var_rt) + (var_psiio * var_rt_dn2)) - (((3.0 * var_phi_t_dn2) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn2 / var_rt)))) - (p.p90 * var_rt_dn2)), ((((var_psiio_dn3 * var_rt) + (var_psiio * var_rt_dn3)) - (((3.0 * var_phi_t_dn3) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn3 / var_rt)))) - (p.p90 * var_rt_dn3)), ((((var_psiio_dn4 * var_rt) + (var_psiio * var_rt_dn4)) - (((3.0 * var_phi_t_dn4) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn4 / var_rt)))) - (p.p90 * var_rt_dn4)), ((((var_psiio_dn5 * var_rt) + (var_psiio * var_rt_dn5)) - (((3.0 * var_phi_t_dn5) * assign1470_e1443) + (assign1470_e1441 * (var_rt_dn5 / var_rt)))) - (p.p90 * var_rt_dn5)), ((((var_psiio_db0 * var_rt) + (var_psiio * var_rt_db0)) - (((3.0 * var_phi_t_db0) * assign1470_e1443) + (assign1470_e1441 * (var_rt_db0 / var_rt)))) - (p.p90 * var_rt_db0)), ((((var_psiio_db1 * var_rt) + (var_psiio * var_rt_db1)) - (((3.0 * var_phi_t_db1) * assign1470_e1443) + (assign1470_e1441 * (var_rt_db1 / var_rt)))) - (p.p90 * var_rt_db1)),)
    } else {
        (var_psiin, var_psiin_dn0, var_psiin_dn1, var_psiin_dn2, var_psiin_dn3, var_psiin_dn4, var_psiin_dn5, var_psiin_db0, var_psiin_db1,)
    }
};
        var_psiin = assign1470_e1453;
        var_psiin_dn0 = assign1470_e1453_d_n0;
        var_psiin_dn1 = assign1470_e1453_d_n1;
        var_psiin_dn2 = assign1470_e1453_d_n2;
        var_psiin_dn3 = assign1470_e1453_d_n3;
        var_psiin_dn4 = assign1470_e1453_d_n4;
        var_psiin_dn5 = assign1470_e1453_d_n5;
        var_psiin_db0 = assign1470_e1453_d_b0;
        var_psiin_db1 = assign1470_e1453_d_b1;
        var_psiin_rv = 0.0;
        var_psiin_rdn0 = 0.0;
        var_psiin_rdn1 = 0.0;
        var_psiin_rdn2 = 0.0;
        var_psiin_rdn3 = 0.0;
        var_psiin_rdn4 = 0.0;
        var_psiin_rdn5 = 0.0;
        var_psiin_rdb0 = 0.0;
        var_psiin_rdb1 = 0.0;

        let (assign1480_e1477, assign1480_e1477_d_n0, assign1480_e1477_d_n1, assign1480_e1477_d_n2, assign1480_e1477_d_n3, assign1480_e1477_d_n4, assign1480_e1477_d_n5, assign1480_e1477_d_b0, assign1480_e1477_d_b1,) = {
    if (var_guard130 != 0.0) {
        let assign1480_e1458: f64 = (2.0 * var_phi_t);
        let assign1480_e1464: f64 = (-var_psiin);
        let assign1480_e1466: f64 = (assign1480_e1464 / var_phi_t);
        let assign1480_e1467: f64 = (assign1480_e1466).exp();
        let assign1480_e1468: f64 = (4.0 * assign1480_e1467);
        let assign1480_e1469: f64 = (1.0 + assign1480_e1468);
        let assign1480_e1470: f64 = (assign1480_e1469).sqrt();
        let assign1480_e1471: f64 = (1.0 + assign1480_e1470);
        let assign1480_e1472: f64 = (0.5 * assign1480_e1471);
        let assign1480_e1473: f64 = (assign1480_e1472).ln();
        let assign1480_e1474: f64 = (assign1480_e1458 * assign1480_e1473);
        let assign1480_e1475: f64 = (var_psiin + assign1480_e1474);
        (assign1480_e1475, (var_psiin_dn0 + (((2.0 * var_phi_t_dn0) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn0) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn0)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_dn1 + (((2.0 * var_phi_t_dn1) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn1) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn1)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_dn2 + (((2.0 * var_phi_t_dn2) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn2) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn2)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_dn3 + (((2.0 * var_phi_t_dn3) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn3) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn3)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_dn4 + (((2.0 * var_phi_t_dn4) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn4) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn4)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_dn5 + (((2.0 * var_phi_t_dn5) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_dn5) * var_phi_t) - (assign1480_e1464 * var_phi_t_dn5)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_db0 + (((2.0 * var_phi_t_db0) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_db0) * var_phi_t) - (assign1480_e1464 * var_phi_t_db0)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))), (var_psiin_db1 + (((2.0 * var_phi_t_db1) * assign1480_e1473) + (assign1480_e1458 * ((0.5 * ((4.0 * (assign1480_e1467 * ((((-var_psiin_db1) * var_phi_t) - (assign1480_e1464 * var_phi_t_db1)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1480_e1470))) / assign1480_e1472)))),)
    } else {
        (var_pa_t, var_pa_t_dn0, var_pa_t_dn1, var_pa_t_dn2, var_pa_t_dn3, var_pa_t_dn4, var_pa_t_dn5, var_pa_t_db0, var_pa_t_db1,)
    }
};
        var_pa_t = assign1480_e1477;
        var_pa_t_dn0 = assign1480_e1477_d_n0;
        var_pa_t_dn1 = assign1480_e1477_d_n1;
        var_pa_t_dn2 = assign1480_e1477_d_n2;
        var_pa_t_dn3 = assign1480_e1477_d_n3;
        var_pa_t_dn4 = assign1480_e1477_d_n4;
        var_pa_t_dn5 = assign1480_e1477_d_n5;
        var_pa_t_db0 = assign1480_e1477_d_b0;
        var_pa_t_db1 = assign1480_e1477_d_b1;
        var_pa_t_rv = 0.0;
        var_pa_t_rdn0 = 0.0;
        var_pa_t_rdn1 = 0.0;
        var_pa_t_rdn2 = 0.0;
        var_pa_t_rdn3 = 0.0;
        var_pa_t_rdn4 = 0.0;
        var_pa_t_rdn5 = 0.0;
        var_pa_t_rdb0 = 0.0;
        var_pa_t_rdb1 = 0.0;

        let (assign1490_e1487, assign1490_e1487_d_n0, assign1490_e1487_d_n1, assign1490_e1487_d_n2, assign1490_e1487_d_n3, assign1490_e1487_d_n4, assign1490_e1487_d_n5, assign1490_e1487_d_b0, assign1490_e1487_d_b1,) = {
    if (var_guard130 != 0.0) {
        let assign1490_e1482: f64 = (p.p73 / var_pa_t);
        let assign1490_e1484: f64 = (assign1490_e1482).powf(p.p74);
        let assign1490_e1485: f64 = (p.p72 * assign1490_e1484);
        (assign1490_e1485, (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn0) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn0) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn1) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn1) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn2) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn2) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn3) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn3) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn4) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn4) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_dn5) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_dn5) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_db0) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_db0) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }), (p.p72 * if 0.0 == 0.0 && ((p.p74) as f64).is_finite() && ((p.p74) as f64).fract() == 0.0 { if p.p74 == 0.0 { 0.0 } else { (p.p74 * ((assign1490_e1482).powf(p.p74 - 1.0) * (-((p.p73 * var_pa_t_db1) / (var_pa_t * var_pa_t))))) } } else { (assign1490_e1484 * (p.p74 * ((-((p.p73 * var_pa_t_db1) / (var_pa_t * var_pa_t))) / assign1490_e1482))) }),)
    } else {
        (var_cja_t, var_cja_t_dn0, var_cja_t_dn1, var_cja_t_dn2, var_cja_t_dn3, var_cja_t_dn4, var_cja_t_dn5, var_cja_t_db0, var_cja_t_db1,)
    }
};
        var_cja_t = assign1490_e1487;
        var_cja_t_dn0 = assign1490_e1487_d_n0;
        var_cja_t_dn1 = assign1490_e1487_d_n1;
        var_cja_t_dn2 = assign1490_e1487_d_n2;
        var_cja_t_dn3 = assign1490_e1487_d_n3;
        var_cja_t_dn4 = assign1490_e1487_d_n4;
        var_cja_t_dn5 = assign1490_e1487_d_n5;
        var_cja_t_db0 = assign1490_e1487_d_b0;
        var_cja_t_db1 = assign1490_e1487_d_b1;
        var_cja_t_rv = 0.0;
        var_cja_t_rdn0 = 0.0;
        var_cja_t_rdn1 = 0.0;
        var_cja_t_rdn2 = 0.0;
        var_cja_t_rdn3 = 0.0;
        var_cja_t_rdn4 = 0.0;
        var_cja_t_rdn5 = 0.0;
        var_cja_t_rdb0 = 0.0;
        var_cja_t_rdb1 = 0.0;

        let (assign1500_e1492, assign1500_e1492_d_n0, assign1500_e1492_d_n1, assign1500_e1492_d_n2, assign1500_e1492_d_n3, assign1500_e1492_d_n4, assign1500_e1492_d_n5, assign1500_e1492_d_b0, assign1500_e1492_d_b1,) = {
    if (var_guard130 == 0.0) {
        (p.p73, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pa_t, var_pa_t_dn0, var_pa_t_dn1, var_pa_t_dn2, var_pa_t_dn3, var_pa_t_dn4, var_pa_t_dn5, var_pa_t_db0, var_pa_t_db1,)
    }
};
        var_pa_t = assign1500_e1492;
        var_pa_t_dn0 = assign1500_e1492_d_n0;
        var_pa_t_dn1 = assign1500_e1492_d_n1;
        var_pa_t_dn2 = assign1500_e1492_d_n2;
        var_pa_t_dn3 = assign1500_e1492_d_n3;
        var_pa_t_dn4 = assign1500_e1492_d_n4;
        var_pa_t_dn5 = assign1500_e1492_d_n5;
        var_pa_t_db0 = assign1500_e1492_d_b0;
        var_pa_t_db1 = assign1500_e1492_d_b1;
        var_pa_t_rv = 0.0;
        var_pa_t_rdn0 = 0.0;
        var_pa_t_rdn1 = 0.0;
        var_pa_t_rdn2 = 0.0;
        var_pa_t_rdn3 = 0.0;
        var_pa_t_rdn4 = 0.0;
        var_pa_t_rdn5 = 0.0;
        var_pa_t_rdb0 = 0.0;
        var_pa_t_rdb1 = 0.0;

        let (assign1510_e1497, assign1510_e1497_d_n0, assign1510_e1497_d_n1, assign1510_e1497_d_n2, assign1510_e1497_d_n3, assign1510_e1497_d_n4, assign1510_e1497_d_n5, assign1510_e1497_d_b0, assign1510_e1497_d_b1,) = {
    if (var_guard130 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cja_t, var_cja_t_dn0, var_cja_t_dn1, var_cja_t_dn2, var_cja_t_dn3, var_cja_t_dn4, var_cja_t_dn5, var_cja_t_db0, var_cja_t_db1,)
    }
};
        var_cja_t = assign1510_e1497;
        var_cja_t_dn0 = assign1510_e1497_d_n0;
        var_cja_t_dn1 = assign1510_e1497_d_n1;
        var_cja_t_dn2 = assign1510_e1497_d_n2;
        var_cja_t_dn3 = assign1510_e1497_d_n3;
        var_cja_t_dn4 = assign1510_e1497_d_n4;
        var_cja_t_dn5 = assign1510_e1497_d_n5;
        var_cja_t_db0 = assign1510_e1497_d_b0;
        var_cja_t_db1 = assign1510_e1497_d_b1;
        var_cja_t_rv = 0.0;
        var_cja_t_rdn0 = 0.0;
        var_cja_t_rdn1 = 0.0;
        var_cja_t_rdn2 = 0.0;
        var_cja_t_rdn3 = 0.0;
        var_cja_t_rdn4 = 0.0;
        var_cja_t_rdn5 = 0.0;
        var_cja_t_rdb0 = 0.0;
        var_cja_t_rdb1 = 0.0;

        let assign1520_e1500: f64 = if p.p79 > 0.0 { 1.0 } else { 0.0 };
        var_guard133 = assign1520_e1500;
        var_guard133_dn0 = 0.0;
        var_guard133_dn1 = 0.0;
        var_guard133_dn2 = 0.0;
        var_guard133_dn3 = 0.0;
        var_guard133_dn4 = 0.0;
        var_guard133_dn5 = 0.0;
        var_guard133_db0 = 0.0;
        var_guard133_db1 = 0.0;
        var_guard133_rv = 0.0;
        var_guard133_rdn0 = 0.0;
        var_guard133_rdn1 = 0.0;
        var_guard133_rdn2 = 0.0;
        var_guard133_rdn3 = 0.0;
        var_guard133_rdn4 = 0.0;
        var_guard133_rdn5 = 0.0;
        var_guard133_rdb0 = 0.0;
        var_guard133_rdb1 = 0.0;

        *var_cja_t_slot = var_cja_t;
        *var_cja_t_db0_slot = var_cja_t_db0;
        *var_cja_t_db1_slot = var_cja_t_db1;
        *var_cja_t_dn0_slot = var_cja_t_dn0;
        *var_cja_t_dn1_slot = var_cja_t_dn1;
        *var_cja_t_dn2_slot = var_cja_t_dn2;
        *var_cja_t_dn3_slot = var_cja_t_dn3;
        *var_cja_t_dn4_slot = var_cja_t_dn4;
        *var_cja_t_dn5_slot = var_cja_t_dn5;
        *var_cja_t_rdb0_slot = var_cja_t_rdb0;
        *var_cja_t_rdb1_slot = var_cja_t_rdb1;
        *var_cja_t_rdn0_slot = var_cja_t_rdn0;
        *var_cja_t_rdn1_slot = var_cja_t_rdn1;
        *var_cja_t_rdn2_slot = var_cja_t_rdn2;
        *var_cja_t_rdn3_slot = var_cja_t_rdn3;
        *var_cja_t_rdn4_slot = var_cja_t_rdn4;
        *var_cja_t_rdn5_slot = var_cja_t_rdn5;
        *var_cja_t_rv_slot = var_cja_t_rv;
        *var_dt_slot = var_dt;
        *var_dt_db0_slot = var_dt_db0;
        *var_dt_db1_slot = var_dt_db1;
        *var_dt_dn0_slot = var_dt_dn0;
        *var_dt_dn1_slot = var_dt_dn1;
        *var_dt_dn2_slot = var_dt_dn2;
        *var_dt_dn3_slot = var_dt_dn3;
        *var_dt_dn4_slot = var_dt_dn4;
        *var_dt_dn5_slot = var_dt_dn5;
        *var_dt_rdb0_slot = var_dt_rdb0;
        *var_dt_rdb1_slot = var_dt_rdb1;
        *var_dt_rdn0_slot = var_dt_rdn0;
        *var_dt_rdn1_slot = var_dt_rdn1;
        *var_dt_rdn2_slot = var_dt_rdn2;
        *var_dt_rdn3_slot = var_dt_rdn3;
        *var_dt_rdn4_slot = var_dt_rdn4;
        *var_dt_rdn5_slot = var_dt_rdn5;
        *var_dt_rv_slot = var_dt_rv;
        *var_guard126_slot = var_guard126;
        *var_guard126_db0_slot = var_guard126_db0;
        *var_guard126_db1_slot = var_guard126_db1;
        *var_guard126_dn0_slot = var_guard126_dn0;
        *var_guard126_dn1_slot = var_guard126_dn1;
        *var_guard126_dn2_slot = var_guard126_dn2;
        *var_guard126_dn3_slot = var_guard126_dn3;
        *var_guard126_dn4_slot = var_guard126_dn4;
        *var_guard126_dn5_slot = var_guard126_dn5;
        *var_guard126_rdb0_slot = var_guard126_rdb0;
        *var_guard126_rdb1_slot = var_guard126_rdb1;
        *var_guard126_rdn0_slot = var_guard126_rdn0;
        *var_guard126_rdn1_slot = var_guard126_rdn1;
        *var_guard126_rdn2_slot = var_guard126_rdn2;
        *var_guard126_rdn3_slot = var_guard126_rdn3;
        *var_guard126_rdn4_slot = var_guard126_rdn4;
        *var_guard126_rdn5_slot = var_guard126_rdn5;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_db0_slot = var_guard130_db0;
        *var_guard130_db1_slot = var_guard130_db1;
        *var_guard130_dn0_slot = var_guard130_dn0;
        *var_guard130_dn1_slot = var_guard130_dn1;
        *var_guard130_dn2_slot = var_guard130_dn2;
        *var_guard130_dn3_slot = var_guard130_dn3;
        *var_guard130_dn4_slot = var_guard130_dn4;
        *var_guard130_dn5_slot = var_guard130_dn5;
        *var_guard130_rdb0_slot = var_guard130_rdb0;
        *var_guard130_rdb1_slot = var_guard130_rdb1;
        *var_guard130_rdn0_slot = var_guard130_rdn0;
        *var_guard130_rdn1_slot = var_guard130_rdn1;
        *var_guard130_rdn2_slot = var_guard130_rdn2;
        *var_guard130_rdn3_slot = var_guard130_rdn3;
        *var_guard130_rdn4_slot = var_guard130_rdn4;
        *var_guard130_rdn5_slot = var_guard130_rdn5;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_db0_slot = var_guard133_db0;
        *var_guard133_db1_slot = var_guard133_db1;
        *var_guard133_dn0_slot = var_guard133_dn0;
        *var_guard133_dn1_slot = var_guard133_dn1;
        *var_guard133_dn2_slot = var_guard133_dn2;
        *var_guard133_dn3_slot = var_guard133_dn3;
        *var_guard133_dn4_slot = var_guard133_dn4;
        *var_guard133_dn5_slot = var_guard133_dn5;
        *var_guard133_rdb0_slot = var_guard133_rdb0;
        *var_guard133_rdb1_slot = var_guard133_rdb1;
        *var_guard133_rdn0_slot = var_guard133_rdn0;
        *var_guard133_rdn1_slot = var_guard133_rdn1;
        *var_guard133_rdn2_slot = var_guard133_rdn2;
        *var_guard133_rdn3_slot = var_guard133_rdn3;
        *var_guard133_rdn4_slot = var_guard133_rdn4;
        *var_guard133_rdn5_slot = var_guard133_rdn5;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_pa_t_slot = var_pa_t;
        *var_pa_t_db0_slot = var_pa_t_db0;
        *var_pa_t_db1_slot = var_pa_t_db1;
        *var_pa_t_dn0_slot = var_pa_t_dn0;
        *var_pa_t_dn1_slot = var_pa_t_dn1;
        *var_pa_t_dn2_slot = var_pa_t_dn2;
        *var_pa_t_dn3_slot = var_pa_t_dn3;
        *var_pa_t_dn4_slot = var_pa_t_dn4;
        *var_pa_t_dn5_slot = var_pa_t_dn5;
        *var_pa_t_rdb0_slot = var_pa_t_rdb0;
        *var_pa_t_rdb1_slot = var_pa_t_rdb1;
        *var_pa_t_rdn0_slot = var_pa_t_rdn0;
        *var_pa_t_rdn1_slot = var_pa_t_rdn1;
        *var_pa_t_rdn2_slot = var_pa_t_rdn2;
        *var_pa_t_rdn3_slot = var_pa_t_rdn3;
        *var_pa_t_rdn4_slot = var_pa_t_rdn4;
        *var_pa_t_rdn5_slot = var_pa_t_rdn5;
        *var_pa_t_rv_slot = var_pa_t_rv;
        *var_phi_t_slot = var_phi_t;
        *var_phi_t_db0_slot = var_phi_t_db0;
        *var_phi_t_db1_slot = var_phi_t_db1;
        *var_phi_t_dn0_slot = var_phi_t_dn0;
        *var_phi_t_dn1_slot = var_phi_t_dn1;
        *var_phi_t_dn2_slot = var_phi_t_dn2;
        *var_phi_t_dn3_slot = var_phi_t_dn3;
        *var_phi_t_dn4_slot = var_phi_t_dn4;
        *var_phi_t_dn5_slot = var_phi_t_dn5;
        *var_phi_t_rdb0_slot = var_phi_t_rdb0;
        *var_phi_t_rdb1_slot = var_phi_t_rdb1;
        *var_phi_t_rdn0_slot = var_phi_t_rdn0;
        *var_phi_t_rdn1_slot = var_phi_t_rdn1;
        *var_phi_t_rdn2_slot = var_phi_t_rdn2;
        *var_phi_t_rdn3_slot = var_phi_t_rdn3;
        *var_phi_t_rdn4_slot = var_phi_t_rdn4;
        *var_phi_t_rdn5_slot = var_phi_t_rdn5;
        *var_phi_t_rv_slot = var_phi_t_rv;
        *var_psiin_slot = var_psiin;
        *var_psiin_db0_slot = var_psiin_db0;
        *var_psiin_db1_slot = var_psiin_db1;
        *var_psiin_dn0_slot = var_psiin_dn0;
        *var_psiin_dn1_slot = var_psiin_dn1;
        *var_psiin_dn2_slot = var_psiin_dn2;
        *var_psiin_dn3_slot = var_psiin_dn3;
        *var_psiin_dn4_slot = var_psiin_dn4;
        *var_psiin_dn5_slot = var_psiin_dn5;
        *var_psiin_rdb0_slot = var_psiin_rdb0;
        *var_psiin_rdb1_slot = var_psiin_rdb1;
        *var_psiin_rdn0_slot = var_psiin_rdn0;
        *var_psiin_rdn1_slot = var_psiin_rdn1;
        *var_psiin_rdn2_slot = var_psiin_rdn2;
        *var_psiin_rdn3_slot = var_psiin_rdn3;
        *var_psiin_rdn4_slot = var_psiin_rdn4;
        *var_psiin_rdn5_slot = var_psiin_rdn5;
        *var_psiin_rv_slot = var_psiin_rv;
        *var_psiio_slot = var_psiio;
        *var_psiio_db0_slot = var_psiio_db0;
        *var_psiio_db1_slot = var_psiio_db1;
        *var_psiio_dn0_slot = var_psiio_dn0;
        *var_psiio_dn1_slot = var_psiio_dn1;
        *var_psiio_dn2_slot = var_psiio_dn2;
        *var_psiio_dn3_slot = var_psiio_dn3;
        *var_psiio_dn4_slot = var_psiio_dn4;
        *var_psiio_dn5_slot = var_psiio_dn5;
        *var_psiio_rdb0_slot = var_psiio_rdb0;
        *var_psiio_rdb1_slot = var_psiio_rdb1;
        *var_psiio_rdn0_slot = var_psiio_rdn0;
        *var_psiio_rdn1_slot = var_psiio_rdn1;
        *var_psiio_rdn2_slot = var_psiio_rdn2;
        *var_psiio_rdn3_slot = var_psiio_rdn3;
        *var_psiio_rdn4_slot = var_psiio_rdn4;
        *var_psiio_rdn5_slot = var_psiio_rdn5;
        *var_psiio_rv_slot = var_psiio_rv;
        *var_rt_slot = var_rt;
        *var_rt_db0_slot = var_rt_db0;
        *var_rt_db1_slot = var_rt_db1;
        *var_rt_dn0_slot = var_rt_dn0;
        *var_rt_dn1_slot = var_rt_dn1;
        *var_rt_dn2_slot = var_rt_dn2;
        *var_rt_dn3_slot = var_rt_dn3;
        *var_rt_dn4_slot = var_rt_dn4;
        *var_rt_dn5_slot = var_rt_dn5;
        *var_rt_rdb0_slot = var_rt_rdb0;
        *var_rt_rdb1_slot = var_rt_rdb1;
        *var_rt_rdn0_slot = var_rt_rdn0;
        *var_rt_rdn1_slot = var_rt_rdn1;
        *var_rt_rdn2_slot = var_rt_rdn2;
        *var_rt_rdn3_slot = var_rt_rdn3;
        *var_rt_rdn4_slot = var_rt_rdn4;
        *var_rt_rdn5_slot = var_rt_rdn5;
        *var_rt_rv_slot = var_rt_rv;
        *var_tcr_slot = var_tcr;
        *var_tcr_db0_slot = var_tcr_db0;
        *var_tcr_db1_slot = var_tcr_db1;
        *var_tcr_dn0_slot = var_tcr_dn0;
        *var_tcr_dn1_slot = var_tcr_dn1;
        *var_tcr_dn2_slot = var_tcr_dn2;
        *var_tcr_dn3_slot = var_tcr_dn3;
        *var_tcr_dn4_slot = var_tcr_dn4;
        *var_tcr_dn5_slot = var_tcr_dn5;
        *var_tcr_rdb0_slot = var_tcr_rdb0;
        *var_tcr_rdb1_slot = var_tcr_rdb1;
        *var_tcr_rdn0_slot = var_tcr_rdn0;
        *var_tcr_rdn1_slot = var_tcr_rdn1;
        *var_tcr_rdn2_slot = var_tcr_rdn2;
        *var_tcr_rdn3_slot = var_tcr_rdn3;
        *var_tcr_rdn4_slot = var_tcr_rdn4;
        *var_tcr_rdn5_slot = var_tcr_rdn5;
        *var_tcr_rv_slot = var_tcr_rv;
        *var_tcvsat_slot = var_tcvsat;
        *var_tcvsat_db0_slot = var_tcvsat_db0;
        *var_tcvsat_db1_slot = var_tcvsat_db1;
        *var_tcvsat_dn0_slot = var_tcvsat_dn0;
        *var_tcvsat_dn1_slot = var_tcvsat_dn1;
        *var_tcvsat_dn2_slot = var_tcvsat_dn2;
        *var_tcvsat_dn3_slot = var_tcvsat_dn3;
        *var_tcvsat_dn4_slot = var_tcvsat_dn4;
        *var_tcvsat_dn5_slot = var_tcvsat_dn5;
        *var_tcvsat_rdb0_slot = var_tcvsat_rdb0;
        *var_tcvsat_rdb1_slot = var_tcvsat_rdb1;
        *var_tcvsat_rdn0_slot = var_tcvsat_rdn0;
        *var_tcvsat_rdn1_slot = var_tcvsat_rdn1;
        *var_tcvsat_rdn2_slot = var_tcvsat_rdn2;
        *var_tcvsat_rdn3_slot = var_tcvsat_rdn3;
        *var_tcvsat_rdn4_slot = var_tcvsat_rdn4;
        *var_tcvsat_rdn5_slot = var_tcvsat_rdn5;
        *var_tcvsat_rv_slot = var_tcvsat_rv;
        *var_tdevc_slot = var_tdevc;
        *var_tdevc_db0_slot = var_tdevc_db0;
        *var_tdevc_db1_slot = var_tdevc_db1;
        *var_tdevc_dn0_slot = var_tdevc_dn0;
        *var_tdevc_dn1_slot = var_tdevc_dn1;
        *var_tdevc_dn2_slot = var_tdevc_dn2;
        *var_tdevc_dn3_slot = var_tdevc_dn3;
        *var_tdevc_dn4_slot = var_tdevc_dn4;
        *var_tdevc_dn5_slot = var_tdevc_dn5;
        *var_tdevc_rdb0_slot = var_tdevc_rdb0;
        *var_tdevc_rdb1_slot = var_tdevc_rdb1;
        *var_tdevc_rdn0_slot = var_tdevc_rdn0;
        *var_tdevc_rdn1_slot = var_tdevc_rdn1;
        *var_tdevc_rdn2_slot = var_tdevc_rdn2;
        *var_tdevc_rdn3_slot = var_tdevc_rdn3;
        *var_tdevc_rdn4_slot = var_tdevc_rdn4;
        *var_tdevc_rdn5_slot = var_tdevc_rdn5;
        *var_tdevc_rv_slot = var_tdevc_rv;
        *var_tdevk_slot = var_tdevk;
        *var_tdevk_db0_slot = var_tdevk_db0;
        *var_tdevk_db1_slot = var_tdevk_db1;
        *var_tdevk_dn0_slot = var_tdevk_dn0;
        *var_tdevk_dn1_slot = var_tdevk_dn1;
        *var_tdevk_dn2_slot = var_tdevk_dn2;
        *var_tdevk_dn3_slot = var_tdevk_dn3;
        *var_tdevk_dn4_slot = var_tdevk_dn4;
        *var_tdevk_dn5_slot = var_tdevk_dn5;
        *var_tdevk_rdb0_slot = var_tdevk_rdb0;
        *var_tdevk_rdb1_slot = var_tdevk_rdb1;
        *var_tdevk_rdn0_slot = var_tdevk_rdn0;
        *var_tdevk_rdn1_slot = var_tdevk_rdn1;
        *var_tdevk_rdn2_slot = var_tdevk_rdn2;
        *var_tdevk_rdn3_slot = var_tdevk_rdn3;
        *var_tdevk_rdn4_slot = var_tdevk_rdn4;
        *var_tdevk_rdn5_slot = var_tdevk_rdn5;
        *var_tdevk_rv_slot = var_tdevk_rv;
    }

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        var_guard133: f64,
        var_leffe_um: f64,
        var_leffe_um_db0: f64,
        var_leffe_um_db1: f64,
        var_leffe_um_dn0: f64,
        var_leffe_um_dn1: f64,
        var_leffe_um_dn2: f64,
        var_leffe_um_dn3: f64,
        var_leffe_um_dn4: f64,
        var_leffe_um_dn5: f64,
        var_phi_t: f64,
        var_phi_t_db0: f64,
        var_phi_t_db1: f64,
        var_phi_t_dn0: f64,
        var_phi_t_dn1: f64,
        var_phi_t_dn2: f64,
        var_phi_t_dn3: f64,
        var_phi_t_dn4: f64,
        var_phi_t_dn5: f64,
        var_rt: f64,
        var_rt_db0: f64,
        var_rt_db1: f64,
        var_rt_dn0: f64,
        var_rt_dn1: f64,
        var_rt_dn2: f64,
        var_rt_dn3: f64,
        var_rt_dn4: f64,
        var_rt_dn5: f64,
        var_tcr: f64,
        var_tcr_db0: f64,
        var_tcr_db1: f64,
        var_tcr_dn0: f64,
        var_tcr_dn1: f64,
        var_tcr_dn2: f64,
        var_tcr_dn3: f64,
        var_tcr_dn4: f64,
        var_tcr_dn5: f64,
        var_tcvsat: f64,
        var_tcvsat_db0: f64,
        var_tcvsat_db1: f64,
        var_tcvsat_dn0: f64,
        var_tcvsat_dn1: f64,
        var_tcvsat_dn2: f64,
        var_tcvsat_dn3: f64,
        var_tcvsat_dn4: f64,
        var_tcvsat_dn5: f64,
        var_cjp_t_slot: &mut f64,
        var_cjp_t_db0_slot: &mut f64,
        var_cjp_t_db1_slot: &mut f64,
        var_cjp_t_dn0_slot: &mut f64,
        var_cjp_t_dn1_slot: &mut f64,
        var_cjp_t_dn2_slot: &mut f64,
        var_cjp_t_dn3_slot: &mut f64,
        var_cjp_t_dn4_slot: &mut f64,
        var_cjp_t_dn5_slot: &mut f64,
        var_cjp_t_rdb0_slot: &mut f64,
        var_cjp_t_rdb1_slot: &mut f64,
        var_cjp_t_rdn0_slot: &mut f64,
        var_cjp_t_rdn1_slot: &mut f64,
        var_cjp_t_rdn2_slot: &mut f64,
        var_cjp_t_rdn3_slot: &mut f64,
        var_cjp_t_rdn4_slot: &mut f64,
        var_cjp_t_rdn5_slot: &mut f64,
        var_cjp_t_rv_slot: &mut f64,
        var_de_slot: &mut f64,
        var_de_db0_slot: &mut f64,
        var_de_db1_slot: &mut f64,
        var_de_dn0_slot: &mut f64,
        var_de_dn1_slot: &mut f64,
        var_de_dn2_slot: &mut f64,
        var_de_dn3_slot: &mut f64,
        var_de_dn4_slot: &mut f64,
        var_de_dn5_slot: &mut f64,
        var_de_rdb0_slot: &mut f64,
        var_de_rdb1_slot: &mut f64,
        var_de_rdn0_slot: &mut f64,
        var_de_rdn1_slot: &mut f64,
        var_de_rdn2_slot: &mut f64,
        var_de_rdn3_slot: &mut f64,
        var_de_rdn4_slot: &mut f64,
        var_de_rdn5_slot: &mut f64,
        var_de_rv_slot: &mut f64,
        var_ecorn_t_slot: &mut f64,
        var_ecorn_t_db0_slot: &mut f64,
        var_ecorn_t_db1_slot: &mut f64,
        var_ecorn_t_dn0_slot: &mut f64,
        var_ecorn_t_dn1_slot: &mut f64,
        var_ecorn_t_dn2_slot: &mut f64,
        var_ecorn_t_dn3_slot: &mut f64,
        var_ecorn_t_dn4_slot: &mut f64,
        var_ecorn_t_dn5_slot: &mut f64,
        var_ecorn_t_rdb0_slot: &mut f64,
        var_ecorn_t_rdb1_slot: &mut f64,
        var_ecorn_t_rdn0_slot: &mut f64,
        var_ecorn_t_rdn1_slot: &mut f64,
        var_ecorn_t_rdn2_slot: &mut f64,
        var_ecorn_t_rdn3_slot: &mut f64,
        var_ecorn_t_rdn4_slot: &mut f64,
        var_ecorn_t_rdn5_slot: &mut f64,
        var_ecorn_t_rv_slot: &mut f64,
        var_ecrit_t_slot: &mut f64,
        var_ecrit_t_db0_slot: &mut f64,
        var_ecrit_t_db1_slot: &mut f64,
        var_ecrit_t_dn0_slot: &mut f64,
        var_ecrit_t_dn1_slot: &mut f64,
        var_ecrit_t_dn2_slot: &mut f64,
        var_ecrit_t_dn3_slot: &mut f64,
        var_ecrit_t_dn4_slot: &mut f64,
        var_ecrit_t_dn5_slot: &mut f64,
        var_ecrit_t_rdb0_slot: &mut f64,
        var_ecrit_t_rdb1_slot: &mut f64,
        var_ecrit_t_rdn0_slot: &mut f64,
        var_ecrit_t_rdn1_slot: &mut f64,
        var_ecrit_t_rdn2_slot: &mut f64,
        var_ecrit_t_rdn3_slot: &mut f64,
        var_ecrit_t_rdn4_slot: &mut f64,
        var_ecrit_t_rdn5_slot: &mut f64,
        var_ecrit_t_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_db0_slot: &mut f64,
        var_guard137_db1_slot: &mut f64,
        var_guard137_dn0_slot: &mut f64,
        var_guard137_dn1_slot: &mut f64,
        var_guard137_dn2_slot: &mut f64,
        var_guard137_dn3_slot: &mut f64,
        var_guard137_dn4_slot: &mut f64,
        var_guard137_dn5_slot: &mut f64,
        var_guard137_rdb0_slot: &mut f64,
        var_guard137_rdb1_slot: &mut f64,
        var_guard137_rdn0_slot: &mut f64,
        var_guard137_rdn1_slot: &mut f64,
        var_guard137_rdn2_slot: &mut f64,
        var_guard137_rdn3_slot: &mut f64,
        var_guard137_rdn4_slot: &mut f64,
        var_guard137_rdn5_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_db0_slot: &mut f64,
        var_guard138_db1_slot: &mut f64,
        var_guard138_dn0_slot: &mut f64,
        var_guard138_dn1_slot: &mut f64,
        var_guard138_dn2_slot: &mut f64,
        var_guard138_dn3_slot: &mut f64,
        var_guard138_dn4_slot: &mut f64,
        var_guard138_dn5_slot: &mut f64,
        var_guard138_rdb0_slot: &mut f64,
        var_guard138_rdb1_slot: &mut f64,
        var_guard138_rdn0_slot: &mut f64,
        var_guard138_rdn1_slot: &mut f64,
        var_guard138_rdn2_slot: &mut f64,
        var_guard138_rdn3_slot: &mut f64,
        var_guard138_rdn4_slot: &mut f64,
        var_guard138_rdn5_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_iecrit_slot: &mut f64,
        var_iecrit_db0_slot: &mut f64,
        var_iecrit_db1_slot: &mut f64,
        var_iecrit_dn0_slot: &mut f64,
        var_iecrit_dn1_slot: &mut f64,
        var_iecrit_dn2_slot: &mut f64,
        var_iecrit_dn3_slot: &mut f64,
        var_iecrit_dn4_slot: &mut f64,
        var_iecrit_dn5_slot: &mut f64,
        var_iecrit_rdb0_slot: &mut f64,
        var_iecrit_rdb1_slot: &mut f64,
        var_iecrit_rdn0_slot: &mut f64,
        var_iecrit_rdn1_slot: &mut f64,
        var_iecrit_rdn2_slot: &mut f64,
        var_iecrit_rdn3_slot: &mut f64,
        var_iecrit_rdn4_slot: &mut f64,
        var_iecrit_rdn5_slot: &mut f64,
        var_iecrit_rv_slot: &mut f64,
        var_lde_slot: &mut f64,
        var_lde_db0_slot: &mut f64,
        var_lde_db1_slot: &mut f64,
        var_lde_dn0_slot: &mut f64,
        var_lde_dn1_slot: &mut f64,
        var_lde_dn2_slot: &mut f64,
        var_lde_dn3_slot: &mut f64,
        var_lde_dn4_slot: &mut f64,
        var_lde_dn5_slot: &mut f64,
        var_lde_rdb0_slot: &mut f64,
        var_lde_rdb1_slot: &mut f64,
        var_lde_rdn0_slot: &mut f64,
        var_lde_rdn1_slot: &mut f64,
        var_lde_rdn2_slot: &mut f64,
        var_lde_rdn3_slot: &mut f64,
        var_lde_rdn4_slot: &mut f64,
        var_lde_rdn5_slot: &mut f64,
        var_lde_rv_slot: &mut f64,
        var_pp_t_slot: &mut f64,
        var_pp_t_db0_slot: &mut f64,
        var_pp_t_db1_slot: &mut f64,
        var_pp_t_dn0_slot: &mut f64,
        var_pp_t_dn1_slot: &mut f64,
        var_pp_t_dn2_slot: &mut f64,
        var_pp_t_dn3_slot: &mut f64,
        var_pp_t_dn4_slot: &mut f64,
        var_pp_t_dn5_slot: &mut f64,
        var_pp_t_rdb0_slot: &mut f64,
        var_pp_t_rdb1_slot: &mut f64,
        var_pp_t_rdn0_slot: &mut f64,
        var_pp_t_rdn1_slot: &mut f64,
        var_pp_t_rdn2_slot: &mut f64,
        var_pp_t_rdn3_slot: &mut f64,
        var_pp_t_rdn4_slot: &mut f64,
        var_pp_t_rdn5_slot: &mut f64,
        var_pp_t_rv_slot: &mut f64,
        var_psiin__blk135_slot: &mut f64,
        var_psiin__blk135_db0_slot: &mut f64,
        var_psiin__blk135_db1_slot: &mut f64,
        var_psiin__blk135_dn0_slot: &mut f64,
        var_psiin__blk135_dn1_slot: &mut f64,
        var_psiin__blk135_dn2_slot: &mut f64,
        var_psiin__blk135_dn3_slot: &mut f64,
        var_psiin__blk135_dn4_slot: &mut f64,
        var_psiin__blk135_dn5_slot: &mut f64,
        var_psiin__blk135_rdb0_slot: &mut f64,
        var_psiin__blk135_rdb1_slot: &mut f64,
        var_psiin__blk135_rdn0_slot: &mut f64,
        var_psiin__blk135_rdn1_slot: &mut f64,
        var_psiin__blk135_rdn2_slot: &mut f64,
        var_psiin__blk135_rdn3_slot: &mut f64,
        var_psiin__blk135_rdn4_slot: &mut f64,
        var_psiin__blk135_rdn5_slot: &mut f64,
        var_psiin__blk135_rv_slot: &mut f64,
        var_psiio__blk134_slot: &mut f64,
        var_psiio__blk134_db0_slot: &mut f64,
        var_psiio__blk134_db1_slot: &mut f64,
        var_psiio__blk134_dn0_slot: &mut f64,
        var_psiio__blk134_dn1_slot: &mut f64,
        var_psiio__blk134_dn2_slot: &mut f64,
        var_psiio__blk134_dn3_slot: &mut f64,
        var_psiio__blk134_dn4_slot: &mut f64,
        var_psiio__blk134_dn5_slot: &mut f64,
        var_psiio__blk134_rdb0_slot: &mut f64,
        var_psiio__blk134_rdb1_slot: &mut f64,
        var_psiio__blk134_rdn0_slot: &mut f64,
        var_psiio__blk134_rdn1_slot: &mut f64,
        var_psiio__blk134_rdn2_slot: &mut f64,
        var_psiio__blk134_rdn3_slot: &mut f64,
        var_psiio__blk134_rdn4_slot: &mut f64,
        var_psiio__blk134_rdn5_slot: &mut f64,
        var_psiio__blk134_rv_slot: &mut f64,
    ) {
        let mut var_cjp_t: f64 = *var_cjp_t_slot;
        let mut var_cjp_t_db0: f64 = *var_cjp_t_db0_slot;
        let mut var_cjp_t_db1: f64 = *var_cjp_t_db1_slot;
        let mut var_cjp_t_dn0: f64 = *var_cjp_t_dn0_slot;
        let mut var_cjp_t_dn1: f64 = *var_cjp_t_dn1_slot;
        let mut var_cjp_t_dn2: f64 = *var_cjp_t_dn2_slot;
        let mut var_cjp_t_dn3: f64 = *var_cjp_t_dn3_slot;
        let mut var_cjp_t_dn4: f64 = *var_cjp_t_dn4_slot;
        let mut var_cjp_t_dn5: f64 = *var_cjp_t_dn5_slot;
        let mut var_cjp_t_rdb0: f64 = *var_cjp_t_rdb0_slot;
        let mut var_cjp_t_rdb1: f64 = *var_cjp_t_rdb1_slot;
        let mut var_cjp_t_rdn0: f64 = *var_cjp_t_rdn0_slot;
        let mut var_cjp_t_rdn1: f64 = *var_cjp_t_rdn1_slot;
        let mut var_cjp_t_rdn2: f64 = *var_cjp_t_rdn2_slot;
        let mut var_cjp_t_rdn3: f64 = *var_cjp_t_rdn3_slot;
        let mut var_cjp_t_rdn4: f64 = *var_cjp_t_rdn4_slot;
        let mut var_cjp_t_rdn5: f64 = *var_cjp_t_rdn5_slot;
        let mut var_cjp_t_rv: f64 = *var_cjp_t_rv_slot;
        let mut var_de: f64 = *var_de_slot;
        let mut var_de_db0: f64 = *var_de_db0_slot;
        let mut var_de_db1: f64 = *var_de_db1_slot;
        let mut var_de_dn0: f64 = *var_de_dn0_slot;
        let mut var_de_dn1: f64 = *var_de_dn1_slot;
        let mut var_de_dn2: f64 = *var_de_dn2_slot;
        let mut var_de_dn3: f64 = *var_de_dn3_slot;
        let mut var_de_dn4: f64 = *var_de_dn4_slot;
        let mut var_de_dn5: f64 = *var_de_dn5_slot;
        let mut var_de_rdb0: f64 = *var_de_rdb0_slot;
        let mut var_de_rdb1: f64 = *var_de_rdb1_slot;
        let mut var_de_rdn0: f64 = *var_de_rdn0_slot;
        let mut var_de_rdn1: f64 = *var_de_rdn1_slot;
        let mut var_de_rdn2: f64 = *var_de_rdn2_slot;
        let mut var_de_rdn3: f64 = *var_de_rdn3_slot;
        let mut var_de_rdn4: f64 = *var_de_rdn4_slot;
        let mut var_de_rdn5: f64 = *var_de_rdn5_slot;
        let mut var_de_rv: f64 = *var_de_rv_slot;
        let mut var_ecorn_t: f64 = *var_ecorn_t_slot;
        let mut var_ecorn_t_db0: f64 = *var_ecorn_t_db0_slot;
        let mut var_ecorn_t_db1: f64 = *var_ecorn_t_db1_slot;
        let mut var_ecorn_t_dn0: f64 = *var_ecorn_t_dn0_slot;
        let mut var_ecorn_t_dn1: f64 = *var_ecorn_t_dn1_slot;
        let mut var_ecorn_t_dn2: f64 = *var_ecorn_t_dn2_slot;
        let mut var_ecorn_t_dn3: f64 = *var_ecorn_t_dn3_slot;
        let mut var_ecorn_t_dn4: f64 = *var_ecorn_t_dn4_slot;
        let mut var_ecorn_t_dn5: f64 = *var_ecorn_t_dn5_slot;
        let mut var_ecorn_t_rdb0: f64 = *var_ecorn_t_rdb0_slot;
        let mut var_ecorn_t_rdb1: f64 = *var_ecorn_t_rdb1_slot;
        let mut var_ecorn_t_rdn0: f64 = *var_ecorn_t_rdn0_slot;
        let mut var_ecorn_t_rdn1: f64 = *var_ecorn_t_rdn1_slot;
        let mut var_ecorn_t_rdn2: f64 = *var_ecorn_t_rdn2_slot;
        let mut var_ecorn_t_rdn3: f64 = *var_ecorn_t_rdn3_slot;
        let mut var_ecorn_t_rdn4: f64 = *var_ecorn_t_rdn4_slot;
        let mut var_ecorn_t_rdn5: f64 = *var_ecorn_t_rdn5_slot;
        let mut var_ecorn_t_rv: f64 = *var_ecorn_t_rv_slot;
        let mut var_ecrit_t: f64 = *var_ecrit_t_slot;
        let mut var_ecrit_t_db0: f64 = *var_ecrit_t_db0_slot;
        let mut var_ecrit_t_db1: f64 = *var_ecrit_t_db1_slot;
        let mut var_ecrit_t_dn0: f64 = *var_ecrit_t_dn0_slot;
        let mut var_ecrit_t_dn1: f64 = *var_ecrit_t_dn1_slot;
        let mut var_ecrit_t_dn2: f64 = *var_ecrit_t_dn2_slot;
        let mut var_ecrit_t_dn3: f64 = *var_ecrit_t_dn3_slot;
        let mut var_ecrit_t_dn4: f64 = *var_ecrit_t_dn4_slot;
        let mut var_ecrit_t_dn5: f64 = *var_ecrit_t_dn5_slot;
        let mut var_ecrit_t_rdb0: f64 = *var_ecrit_t_rdb0_slot;
        let mut var_ecrit_t_rdb1: f64 = *var_ecrit_t_rdb1_slot;
        let mut var_ecrit_t_rdn0: f64 = *var_ecrit_t_rdn0_slot;
        let mut var_ecrit_t_rdn1: f64 = *var_ecrit_t_rdn1_slot;
        let mut var_ecrit_t_rdn2: f64 = *var_ecrit_t_rdn2_slot;
        let mut var_ecrit_t_rdn3: f64 = *var_ecrit_t_rdn3_slot;
        let mut var_ecrit_t_rdn4: f64 = *var_ecrit_t_rdn4_slot;
        let mut var_ecrit_t_rdn5: f64 = *var_ecrit_t_rdn5_slot;
        let mut var_ecrit_t_rv: f64 = *var_ecrit_t_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_db0: f64 = *var_guard137_db0_slot;
        let mut var_guard137_db1: f64 = *var_guard137_db1_slot;
        let mut var_guard137_dn0: f64 = *var_guard137_dn0_slot;
        let mut var_guard137_dn1: f64 = *var_guard137_dn1_slot;
        let mut var_guard137_dn2: f64 = *var_guard137_dn2_slot;
        let mut var_guard137_dn3: f64 = *var_guard137_dn3_slot;
        let mut var_guard137_dn4: f64 = *var_guard137_dn4_slot;
        let mut var_guard137_dn5: f64 = *var_guard137_dn5_slot;
        let mut var_guard137_rdb0: f64 = *var_guard137_rdb0_slot;
        let mut var_guard137_rdb1: f64 = *var_guard137_rdb1_slot;
        let mut var_guard137_rdn0: f64 = *var_guard137_rdn0_slot;
        let mut var_guard137_rdn1: f64 = *var_guard137_rdn1_slot;
        let mut var_guard137_rdn2: f64 = *var_guard137_rdn2_slot;
        let mut var_guard137_rdn3: f64 = *var_guard137_rdn3_slot;
        let mut var_guard137_rdn4: f64 = *var_guard137_rdn4_slot;
        let mut var_guard137_rdn5: f64 = *var_guard137_rdn5_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_db0: f64 = *var_guard138_db0_slot;
        let mut var_guard138_db1: f64 = *var_guard138_db1_slot;
        let mut var_guard138_dn0: f64 = *var_guard138_dn0_slot;
        let mut var_guard138_dn1: f64 = *var_guard138_dn1_slot;
        let mut var_guard138_dn2: f64 = *var_guard138_dn2_slot;
        let mut var_guard138_dn3: f64 = *var_guard138_dn3_slot;
        let mut var_guard138_dn4: f64 = *var_guard138_dn4_slot;
        let mut var_guard138_dn5: f64 = *var_guard138_dn5_slot;
        let mut var_guard138_rdb0: f64 = *var_guard138_rdb0_slot;
        let mut var_guard138_rdb1: f64 = *var_guard138_rdb1_slot;
        let mut var_guard138_rdn0: f64 = *var_guard138_rdn0_slot;
        let mut var_guard138_rdn1: f64 = *var_guard138_rdn1_slot;
        let mut var_guard138_rdn2: f64 = *var_guard138_rdn2_slot;
        let mut var_guard138_rdn3: f64 = *var_guard138_rdn3_slot;
        let mut var_guard138_rdn4: f64 = *var_guard138_rdn4_slot;
        let mut var_guard138_rdn5: f64 = *var_guard138_rdn5_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_iecrit: f64 = *var_iecrit_slot;
        let mut var_iecrit_db0: f64 = *var_iecrit_db0_slot;
        let mut var_iecrit_db1: f64 = *var_iecrit_db1_slot;
        let mut var_iecrit_dn0: f64 = *var_iecrit_dn0_slot;
        let mut var_iecrit_dn1: f64 = *var_iecrit_dn1_slot;
        let mut var_iecrit_dn2: f64 = *var_iecrit_dn2_slot;
        let mut var_iecrit_dn3: f64 = *var_iecrit_dn3_slot;
        let mut var_iecrit_dn4: f64 = *var_iecrit_dn4_slot;
        let mut var_iecrit_dn5: f64 = *var_iecrit_dn5_slot;
        let mut var_iecrit_rdb0: f64 = *var_iecrit_rdb0_slot;
        let mut var_iecrit_rdb1: f64 = *var_iecrit_rdb1_slot;
        let mut var_iecrit_rdn0: f64 = *var_iecrit_rdn0_slot;
        let mut var_iecrit_rdn1: f64 = *var_iecrit_rdn1_slot;
        let mut var_iecrit_rdn2: f64 = *var_iecrit_rdn2_slot;
        let mut var_iecrit_rdn3: f64 = *var_iecrit_rdn3_slot;
        let mut var_iecrit_rdn4: f64 = *var_iecrit_rdn4_slot;
        let mut var_iecrit_rdn5: f64 = *var_iecrit_rdn5_slot;
        let mut var_iecrit_rv: f64 = *var_iecrit_rv_slot;
        let mut var_lde: f64 = *var_lde_slot;
        let mut var_lde_db0: f64 = *var_lde_db0_slot;
        let mut var_lde_db1: f64 = *var_lde_db1_slot;
        let mut var_lde_dn0: f64 = *var_lde_dn0_slot;
        let mut var_lde_dn1: f64 = *var_lde_dn1_slot;
        let mut var_lde_dn2: f64 = *var_lde_dn2_slot;
        let mut var_lde_dn3: f64 = *var_lde_dn3_slot;
        let mut var_lde_dn4: f64 = *var_lde_dn4_slot;
        let mut var_lde_dn5: f64 = *var_lde_dn5_slot;
        let mut var_lde_rdb0: f64 = *var_lde_rdb0_slot;
        let mut var_lde_rdb1: f64 = *var_lde_rdb1_slot;
        let mut var_lde_rdn0: f64 = *var_lde_rdn0_slot;
        let mut var_lde_rdn1: f64 = *var_lde_rdn1_slot;
        let mut var_lde_rdn2: f64 = *var_lde_rdn2_slot;
        let mut var_lde_rdn3: f64 = *var_lde_rdn3_slot;
        let mut var_lde_rdn4: f64 = *var_lde_rdn4_slot;
        let mut var_lde_rdn5: f64 = *var_lde_rdn5_slot;
        let mut var_lde_rv: f64 = *var_lde_rv_slot;
        let mut var_pp_t: f64 = *var_pp_t_slot;
        let mut var_pp_t_db0: f64 = *var_pp_t_db0_slot;
        let mut var_pp_t_db1: f64 = *var_pp_t_db1_slot;
        let mut var_pp_t_dn0: f64 = *var_pp_t_dn0_slot;
        let mut var_pp_t_dn1: f64 = *var_pp_t_dn1_slot;
        let mut var_pp_t_dn2: f64 = *var_pp_t_dn2_slot;
        let mut var_pp_t_dn3: f64 = *var_pp_t_dn3_slot;
        let mut var_pp_t_dn4: f64 = *var_pp_t_dn4_slot;
        let mut var_pp_t_dn5: f64 = *var_pp_t_dn5_slot;
        let mut var_pp_t_rdb0: f64 = *var_pp_t_rdb0_slot;
        let mut var_pp_t_rdb1: f64 = *var_pp_t_rdb1_slot;
        let mut var_pp_t_rdn0: f64 = *var_pp_t_rdn0_slot;
        let mut var_pp_t_rdn1: f64 = *var_pp_t_rdn1_slot;
        let mut var_pp_t_rdn2: f64 = *var_pp_t_rdn2_slot;
        let mut var_pp_t_rdn3: f64 = *var_pp_t_rdn3_slot;
        let mut var_pp_t_rdn4: f64 = *var_pp_t_rdn4_slot;
        let mut var_pp_t_rdn5: f64 = *var_pp_t_rdn5_slot;
        let mut var_pp_t_rv: f64 = *var_pp_t_rv_slot;
        let mut var_psiin__blk135: f64 = *var_psiin__blk135_slot;
        let mut var_psiin__blk135_db0: f64 = *var_psiin__blk135_db0_slot;
        let mut var_psiin__blk135_db1: f64 = *var_psiin__blk135_db1_slot;
        let mut var_psiin__blk135_dn0: f64 = *var_psiin__blk135_dn0_slot;
        let mut var_psiin__blk135_dn1: f64 = *var_psiin__blk135_dn1_slot;
        let mut var_psiin__blk135_dn2: f64 = *var_psiin__blk135_dn2_slot;
        let mut var_psiin__blk135_dn3: f64 = *var_psiin__blk135_dn3_slot;
        let mut var_psiin__blk135_dn4: f64 = *var_psiin__blk135_dn4_slot;
        let mut var_psiin__blk135_dn5: f64 = *var_psiin__blk135_dn5_slot;
        let mut var_psiin__blk135_rdb0: f64 = *var_psiin__blk135_rdb0_slot;
        let mut var_psiin__blk135_rdb1: f64 = *var_psiin__blk135_rdb1_slot;
        let mut var_psiin__blk135_rdn0: f64 = *var_psiin__blk135_rdn0_slot;
        let mut var_psiin__blk135_rdn1: f64 = *var_psiin__blk135_rdn1_slot;
        let mut var_psiin__blk135_rdn2: f64 = *var_psiin__blk135_rdn2_slot;
        let mut var_psiin__blk135_rdn3: f64 = *var_psiin__blk135_rdn3_slot;
        let mut var_psiin__blk135_rdn4: f64 = *var_psiin__blk135_rdn4_slot;
        let mut var_psiin__blk135_rdn5: f64 = *var_psiin__blk135_rdn5_slot;
        let mut var_psiin__blk135_rv: f64 = *var_psiin__blk135_rv_slot;
        let mut var_psiio__blk134: f64 = *var_psiio__blk134_slot;
        let mut var_psiio__blk134_db0: f64 = *var_psiio__blk134_db0_slot;
        let mut var_psiio__blk134_db1: f64 = *var_psiio__blk134_db1_slot;
        let mut var_psiio__blk134_dn0: f64 = *var_psiio__blk134_dn0_slot;
        let mut var_psiio__blk134_dn1: f64 = *var_psiio__blk134_dn1_slot;
        let mut var_psiio__blk134_dn2: f64 = *var_psiio__blk134_dn2_slot;
        let mut var_psiio__blk134_dn3: f64 = *var_psiio__blk134_dn3_slot;
        let mut var_psiio__blk134_dn4: f64 = *var_psiio__blk134_dn4_slot;
        let mut var_psiio__blk134_dn5: f64 = *var_psiio__blk134_dn5_slot;
        let mut var_psiio__blk134_rdb0: f64 = *var_psiio__blk134_rdb0_slot;
        let mut var_psiio__blk134_rdb1: f64 = *var_psiio__blk134_rdb1_slot;
        let mut var_psiio__blk134_rdn0: f64 = *var_psiio__blk134_rdn0_slot;
        let mut var_psiio__blk134_rdn1: f64 = *var_psiio__blk134_rdn1_slot;
        let mut var_psiio__blk134_rdn2: f64 = *var_psiio__blk134_rdn2_slot;
        let mut var_psiio__blk134_rdn3: f64 = *var_psiio__blk134_rdn3_slot;
        let mut var_psiio__blk134_rdn4: f64 = *var_psiio__blk134_rdn4_slot;
        let mut var_psiio__blk134_rdn5: f64 = *var_psiio__blk134_rdn5_slot;
        let mut var_psiio__blk134_rv: f64 = *var_psiio__blk134_rv_slot;

        let (assign1530_e1528, assign1530_e1528_d_n0, assign1530_e1528_d_n1, assign1530_e1528_d_n2, assign1530_e1528_d_n3, assign1530_e1528_d_n4, assign1530_e1528_d_n5, assign1530_e1528_d_b0, assign1530_e1528_d_b1,) = {
    if (var_guard133 != 0.0) {
        let assign1530_e1505: f64 = (var_phi_t / var_rt);
        let assign1530_e1506: f64 = (2.0 * assign1530_e1505);
        let assign1530_e1509: f64 = (0.5 * p.p80);
        let assign1530_e1511: f64 = (assign1530_e1509 * var_rt);
        let assign1530_e1513: f64 = (assign1530_e1511 / var_phi_t);
        let assign1530_e1514: f64 = (assign1530_e1513).exp();
        let assign1530_e1516: f64 = (-0.5);
        let assign1530_e1518: f64 = (assign1530_e1516 * p.p80);
        let assign1530_e1520: f64 = (assign1530_e1518 * var_rt);
        let assign1530_e1522: f64 = (assign1530_e1520 / var_phi_t);
        let assign1530_e1523: f64 = (assign1530_e1522).exp();
        let assign1530_e1524: f64 = (assign1530_e1514 - assign1530_e1523);
        let assign1530_e1525: f64 = (assign1530_e1524).ln();
        let assign1530_e1526: f64 = (assign1530_e1506 * assign1530_e1525);
        (assign1530_e1526, (((2.0 * (((var_phi_t_dn0 * var_rt) - (var_phi_t * var_rt_dn0)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn0) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn0)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn0) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn0)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_dn1 * var_rt) - (var_phi_t * var_rt_dn1)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn1) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn1)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn1) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn1)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_dn2 * var_rt) - (var_phi_t * var_rt_dn2)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn2) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn2)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn2) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn2)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_dn3 * var_rt) - (var_phi_t * var_rt_dn3)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn3) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn3)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn3) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn3)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_dn4 * var_rt) - (var_phi_t * var_rt_dn4)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn4) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn4)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn4) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn4)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_dn5 * var_rt) - (var_phi_t * var_rt_dn5)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_dn5) * var_phi_t) - (assign1530_e1511 * var_phi_t_dn5)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_dn5) * var_phi_t) - (assign1530_e1520 * var_phi_t_dn5)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_db0 * var_rt) - (var_phi_t * var_rt_db0)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_db0) * var_phi_t) - (assign1530_e1511 * var_phi_t_db0)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_db0) * var_phi_t) - (assign1530_e1520 * var_phi_t_db0)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))), (((2.0 * (((var_phi_t_db1 * var_rt) - (var_phi_t * var_rt_db1)) / (var_rt * var_rt))) * assign1530_e1525) + (assign1530_e1506 * (((assign1530_e1514 * ((((assign1530_e1509 * var_rt_db1) * var_phi_t) - (assign1530_e1511 * var_phi_t_db1)) / (var_phi_t * var_phi_t))) - (assign1530_e1523 * ((((assign1530_e1518 * var_rt_db1) * var_phi_t) - (assign1530_e1520 * var_phi_t_db1)) / (var_phi_t * var_phi_t)))) / assign1530_e1524))),)
    } else {
        (var_psiio__blk134, var_psiio__blk134_dn0, var_psiio__blk134_dn1, var_psiio__blk134_dn2, var_psiio__blk134_dn3, var_psiio__blk134_dn4, var_psiio__blk134_dn5, var_psiio__blk134_db0, var_psiio__blk134_db1,)
    }
};
        var_psiio__blk134 = assign1530_e1528;
        var_psiio__blk134_dn0 = assign1530_e1528_d_n0;
        var_psiio__blk134_dn1 = assign1530_e1528_d_n1;
        var_psiio__blk134_dn2 = assign1530_e1528_d_n2;
        var_psiio__blk134_dn3 = assign1530_e1528_d_n3;
        var_psiio__blk134_dn4 = assign1530_e1528_d_n4;
        var_psiio__blk134_dn5 = assign1530_e1528_d_n5;
        var_psiio__blk134_db0 = assign1530_e1528_d_b0;
        var_psiio__blk134_db1 = assign1530_e1528_d_b1;
        var_psiio__blk134_rv = 0.0;
        var_psiio__blk134_rdn0 = 0.0;
        var_psiio__blk134_rdn1 = 0.0;
        var_psiio__blk134_rdn2 = 0.0;
        var_psiio__blk134_rdn3 = 0.0;
        var_psiio__blk134_rdn4 = 0.0;
        var_psiio__blk134_rdn5 = 0.0;
        var_psiio__blk134_rdb0 = 0.0;
        var_psiio__blk134_rdb1 = 0.0;

        let (assign1540_e1547, assign1540_e1547_d_n0, assign1540_e1547_d_n1, assign1540_e1547_d_n2, assign1540_e1547_d_n3, assign1540_e1547_d_n4, assign1540_e1547_d_n5, assign1540_e1547_d_b0, assign1540_e1547_d_b1,) = {
    if (var_guard133 != 0.0) {
        let assign1540_e1532: f64 = (var_psiio__blk134 * var_rt);
        let assign1540_e1535: f64 = (3.0 * var_phi_t);
        let assign1540_e1537: f64 = (var_rt).ln();
        let assign1540_e1538: f64 = (assign1540_e1535 * assign1540_e1537);
        let assign1540_e1539: f64 = (assign1540_e1532 - assign1540_e1538);
        let assign1540_e1543: f64 = (var_rt - 1.0);
        let assign1540_e1544: f64 = (p.p90 * assign1540_e1543);
        let assign1540_e1545: f64 = (assign1540_e1539 - assign1540_e1544);
        (assign1540_e1545, ((((var_psiio__blk134_dn0 * var_rt) + (var_psiio__blk134 * var_rt_dn0)) - (((3.0 * var_phi_t_dn0) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn0 / var_rt)))) - (p.p90 * var_rt_dn0)), ((((var_psiio__blk134_dn1 * var_rt) + (var_psiio__blk134 * var_rt_dn1)) - (((3.0 * var_phi_t_dn1) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn1 / var_rt)))) - (p.p90 * var_rt_dn1)), ((((var_psiio__blk134_dn2 * var_rt) + (var_psiio__blk134 * var_rt_dn2)) - (((3.0 * var_phi_t_dn2) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn2 / var_rt)))) - (p.p90 * var_rt_dn2)), ((((var_psiio__blk134_dn3 * var_rt) + (var_psiio__blk134 * var_rt_dn3)) - (((3.0 * var_phi_t_dn3) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn3 / var_rt)))) - (p.p90 * var_rt_dn3)), ((((var_psiio__blk134_dn4 * var_rt) + (var_psiio__blk134 * var_rt_dn4)) - (((3.0 * var_phi_t_dn4) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn4 / var_rt)))) - (p.p90 * var_rt_dn4)), ((((var_psiio__blk134_dn5 * var_rt) + (var_psiio__blk134 * var_rt_dn5)) - (((3.0 * var_phi_t_dn5) * assign1540_e1537) + (assign1540_e1535 * (var_rt_dn5 / var_rt)))) - (p.p90 * var_rt_dn5)), ((((var_psiio__blk134_db0 * var_rt) + (var_psiio__blk134 * var_rt_db0)) - (((3.0 * var_phi_t_db0) * assign1540_e1537) + (assign1540_e1535 * (var_rt_db0 / var_rt)))) - (p.p90 * var_rt_db0)), ((((var_psiio__blk134_db1 * var_rt) + (var_psiio__blk134 * var_rt_db1)) - (((3.0 * var_phi_t_db1) * assign1540_e1537) + (assign1540_e1535 * (var_rt_db1 / var_rt)))) - (p.p90 * var_rt_db1)),)
    } else {
        (var_psiin__blk135, var_psiin__blk135_dn0, var_psiin__blk135_dn1, var_psiin__blk135_dn2, var_psiin__blk135_dn3, var_psiin__blk135_dn4, var_psiin__blk135_dn5, var_psiin__blk135_db0, var_psiin__blk135_db1,)
    }
};
        var_psiin__blk135 = assign1540_e1547;
        var_psiin__blk135_dn0 = assign1540_e1547_d_n0;
        var_psiin__blk135_dn1 = assign1540_e1547_d_n1;
        var_psiin__blk135_dn2 = assign1540_e1547_d_n2;
        var_psiin__blk135_dn3 = assign1540_e1547_d_n3;
        var_psiin__blk135_dn4 = assign1540_e1547_d_n4;
        var_psiin__blk135_dn5 = assign1540_e1547_d_n5;
        var_psiin__blk135_db0 = assign1540_e1547_d_b0;
        var_psiin__blk135_db1 = assign1540_e1547_d_b1;
        var_psiin__blk135_rv = 0.0;
        var_psiin__blk135_rdn0 = 0.0;
        var_psiin__blk135_rdn1 = 0.0;
        var_psiin__blk135_rdn2 = 0.0;
        var_psiin__blk135_rdn3 = 0.0;
        var_psiin__blk135_rdn4 = 0.0;
        var_psiin__blk135_rdn5 = 0.0;
        var_psiin__blk135_rdb0 = 0.0;
        var_psiin__blk135_rdb1 = 0.0;

        let (assign1550_e1571, assign1550_e1571_d_n0, assign1550_e1571_d_n1, assign1550_e1571_d_n2, assign1550_e1571_d_n3, assign1550_e1571_d_n4, assign1550_e1571_d_n5, assign1550_e1571_d_b0, assign1550_e1571_d_b1,) = {
    if (var_guard133 != 0.0) {
        let assign1550_e1552: f64 = (2.0 * var_phi_t);
        let assign1550_e1558: f64 = (-var_psiin__blk135);
        let assign1550_e1560: f64 = (assign1550_e1558 / var_phi_t);
        let assign1550_e1561: f64 = (assign1550_e1560).exp();
        let assign1550_e1562: f64 = (4.0 * assign1550_e1561);
        let assign1550_e1563: f64 = (1.0 + assign1550_e1562);
        let assign1550_e1564: f64 = (assign1550_e1563).sqrt();
        let assign1550_e1565: f64 = (1.0 + assign1550_e1564);
        let assign1550_e1566: f64 = (0.5 * assign1550_e1565);
        let assign1550_e1567: f64 = (assign1550_e1566).ln();
        let assign1550_e1568: f64 = (assign1550_e1552 * assign1550_e1567);
        let assign1550_e1569: f64 = (var_psiin__blk135 + assign1550_e1568);
        (assign1550_e1569, (var_psiin__blk135_dn0 + (((2.0 * var_phi_t_dn0) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn0) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn0)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_dn1 + (((2.0 * var_phi_t_dn1) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn1) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn1)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_dn2 + (((2.0 * var_phi_t_dn2) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn2) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn2)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_dn3 + (((2.0 * var_phi_t_dn3) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn3) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn3)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_dn4 + (((2.0 * var_phi_t_dn4) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn4) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn4)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_dn5 + (((2.0 * var_phi_t_dn5) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_dn5) * var_phi_t) - (assign1550_e1558 * var_phi_t_dn5)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_db0 + (((2.0 * var_phi_t_db0) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_db0) * var_phi_t) - (assign1550_e1558 * var_phi_t_db0)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))), (var_psiin__blk135_db1 + (((2.0 * var_phi_t_db1) * assign1550_e1567) + (assign1550_e1552 * ((0.5 * ((4.0 * (assign1550_e1561 * ((((-var_psiin__blk135_db1) * var_phi_t) - (assign1550_e1558 * var_phi_t_db1)) / (var_phi_t * var_phi_t)))) / (2.0 * assign1550_e1564))) / assign1550_e1566)))),)
    } else {
        (var_pp_t, var_pp_t_dn0, var_pp_t_dn1, var_pp_t_dn2, var_pp_t_dn3, var_pp_t_dn4, var_pp_t_dn5, var_pp_t_db0, var_pp_t_db1,)
    }
};
        var_pp_t = assign1550_e1571;
        var_pp_t_dn0 = assign1550_e1571_d_n0;
        var_pp_t_dn1 = assign1550_e1571_d_n1;
        var_pp_t_dn2 = assign1550_e1571_d_n2;
        var_pp_t_dn3 = assign1550_e1571_d_n3;
        var_pp_t_dn4 = assign1550_e1571_d_n4;
        var_pp_t_dn5 = assign1550_e1571_d_n5;
        var_pp_t_db0 = assign1550_e1571_d_b0;
        var_pp_t_db1 = assign1550_e1571_d_b1;
        var_pp_t_rv = 0.0;
        var_pp_t_rdn0 = 0.0;
        var_pp_t_rdn1 = 0.0;
        var_pp_t_rdn2 = 0.0;
        var_pp_t_rdn3 = 0.0;
        var_pp_t_rdn4 = 0.0;
        var_pp_t_rdn5 = 0.0;
        var_pp_t_rdb0 = 0.0;
        var_pp_t_rdb1 = 0.0;

        let (assign1560_e1581, assign1560_e1581_d_n0, assign1560_e1581_d_n1, assign1560_e1581_d_n2, assign1560_e1581_d_n3, assign1560_e1581_d_n4, assign1560_e1581_d_n5, assign1560_e1581_d_b0, assign1560_e1581_d_b1,) = {
    if (var_guard133 != 0.0) {
        let assign1560_e1576: f64 = (p.p80 / var_pp_t);
        let assign1560_e1578: f64 = (assign1560_e1576).powf(p.p81);
        let assign1560_e1579: f64 = (p.p79 * assign1560_e1578);
        (assign1560_e1579, (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn0) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn0) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn1) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn1) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn2) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn2) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn3) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn3) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn4) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn4) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_dn5) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_dn5) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_db0) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_db0) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }), (p.p79 * if 0.0 == 0.0 && ((p.p81) as f64).is_finite() && ((p.p81) as f64).fract() == 0.0 { if p.p81 == 0.0 { 0.0 } else { (p.p81 * ((assign1560_e1576).powf(p.p81 - 1.0) * (-((p.p80 * var_pp_t_db1) / (var_pp_t * var_pp_t))))) } } else { (assign1560_e1578 * (p.p81 * ((-((p.p80 * var_pp_t_db1) / (var_pp_t * var_pp_t))) / assign1560_e1576))) }),)
    } else {
        (var_cjp_t, var_cjp_t_dn0, var_cjp_t_dn1, var_cjp_t_dn2, var_cjp_t_dn3, var_cjp_t_dn4, var_cjp_t_dn5, var_cjp_t_db0, var_cjp_t_db1,)
    }
};
        var_cjp_t = assign1560_e1581;
        var_cjp_t_dn0 = assign1560_e1581_d_n0;
        var_cjp_t_dn1 = assign1560_e1581_d_n1;
        var_cjp_t_dn2 = assign1560_e1581_d_n2;
        var_cjp_t_dn3 = assign1560_e1581_d_n3;
        var_cjp_t_dn4 = assign1560_e1581_d_n4;
        var_cjp_t_dn5 = assign1560_e1581_d_n5;
        var_cjp_t_db0 = assign1560_e1581_d_b0;
        var_cjp_t_db1 = assign1560_e1581_d_b1;
        var_cjp_t_rv = 0.0;
        var_cjp_t_rdn0 = 0.0;
        var_cjp_t_rdn1 = 0.0;
        var_cjp_t_rdn2 = 0.0;
        var_cjp_t_rdn3 = 0.0;
        var_cjp_t_rdn4 = 0.0;
        var_cjp_t_rdn5 = 0.0;
        var_cjp_t_rdb0 = 0.0;
        var_cjp_t_rdb1 = 0.0;

        let (assign1570_e1586, assign1570_e1586_d_n0, assign1570_e1586_d_n1, assign1570_e1586_d_n2, assign1570_e1586_d_n3, assign1570_e1586_d_n4, assign1570_e1586_d_n5, assign1570_e1586_d_b0, assign1570_e1586_d_b1,) = {
    if (var_guard133 == 0.0) {
        (p.p80, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pp_t, var_pp_t_dn0, var_pp_t_dn1, var_pp_t_dn2, var_pp_t_dn3, var_pp_t_dn4, var_pp_t_dn5, var_pp_t_db0, var_pp_t_db1,)
    }
};
        var_pp_t = assign1570_e1586;
        var_pp_t_dn0 = assign1570_e1586_d_n0;
        var_pp_t_dn1 = assign1570_e1586_d_n1;
        var_pp_t_dn2 = assign1570_e1586_d_n2;
        var_pp_t_dn3 = assign1570_e1586_d_n3;
        var_pp_t_dn4 = assign1570_e1586_d_n4;
        var_pp_t_dn5 = assign1570_e1586_d_n5;
        var_pp_t_db0 = assign1570_e1586_d_b0;
        var_pp_t_db1 = assign1570_e1586_d_b1;
        var_pp_t_rv = 0.0;
        var_pp_t_rdn0 = 0.0;
        var_pp_t_rdn1 = 0.0;
        var_pp_t_rdn2 = 0.0;
        var_pp_t_rdn3 = 0.0;
        var_pp_t_rdn4 = 0.0;
        var_pp_t_rdn5 = 0.0;
        var_pp_t_rdb0 = 0.0;
        var_pp_t_rdb1 = 0.0;

        let (assign1580_e1591, assign1580_e1591_d_n0, assign1580_e1591_d_n1, assign1580_e1591_d_n2, assign1580_e1591_d_n3, assign1580_e1591_d_n4, assign1580_e1591_d_n5, assign1580_e1591_d_b0, assign1580_e1591_d_b1,) = {
    if (var_guard133 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cjp_t, var_cjp_t_dn0, var_cjp_t_dn1, var_cjp_t_dn2, var_cjp_t_dn3, var_cjp_t_dn4, var_cjp_t_dn5, var_cjp_t_db0, var_cjp_t_db1,)
    }
};
        var_cjp_t = assign1580_e1591;
        var_cjp_t_dn0 = assign1580_e1591_d_n0;
        var_cjp_t_dn1 = assign1580_e1591_d_n1;
        var_cjp_t_dn2 = assign1580_e1591_d_n2;
        var_cjp_t_dn3 = assign1580_e1591_d_n3;
        var_cjp_t_dn4 = assign1580_e1591_d_n4;
        var_cjp_t_dn5 = assign1580_e1591_d_n5;
        var_cjp_t_db0 = assign1580_e1591_d_b0;
        var_cjp_t_db1 = assign1580_e1591_d_b1;
        var_cjp_t_rv = 0.0;
        var_cjp_t_rdn0 = 0.0;
        var_cjp_t_rdn1 = 0.0;
        var_cjp_t_rdn2 = 0.0;
        var_cjp_t_rdn3 = 0.0;
        var_cjp_t_rdn4 = 0.0;
        var_cjp_t_rdn5 = 0.0;
        var_cjp_t_rdb0 = 0.0;
        var_cjp_t_rdb1 = 0.0;

        let assign1690_e1680: f64 = if ((p.p60 > 0.0) && (p.p15 == 0.0)) { 1.0 } else { 0.0 };
        var_guard137 = assign1690_e1680;
        var_guard137_dn0 = 0.0;
        var_guard137_dn1 = 0.0;
        var_guard137_dn2 = 0.0;
        var_guard137_dn3 = 0.0;
        var_guard137_dn4 = 0.0;
        var_guard137_dn5 = 0.0;
        var_guard137_db0 = 0.0;
        var_guard137_db1 = 0.0;
        var_guard137_rv = 0.0;
        var_guard137_rdn0 = 0.0;
        var_guard137_rdn1 = 0.0;
        var_guard137_rdn2 = 0.0;
        var_guard137_rdn3 = 0.0;
        var_guard137_rdn4 = 0.0;
        var_guard137_rdn5 = 0.0;
        var_guard137_rdb0 = 0.0;
        var_guard137_rdb1 = 0.0;

        let (assign1700_e1690, assign1700_e1690_d_n0, assign1700_e1690_d_n1, assign1700_e1690_d_n2, assign1700_e1690_d_n3, assign1700_e1690_d_n4, assign1700_e1690_d_n5, assign1700_e1690_d_b0, assign1700_e1690_d_b1,) = {
    if ((var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1700_e1686: f64 = (p.p61 * var_tcvsat);
        let assign1700_e1688: f64 = (assign1700_e1686 * var_tcr);
        (assign1700_e1688, (((p.p61 * var_tcvsat_dn0) * var_tcr) + (assign1700_e1686 * var_tcr_dn0)), (((p.p61 * var_tcvsat_dn1) * var_tcr) + (assign1700_e1686 * var_tcr_dn1)), (((p.p61 * var_tcvsat_dn2) * var_tcr) + (assign1700_e1686 * var_tcr_dn2)), (((p.p61 * var_tcvsat_dn3) * var_tcr) + (assign1700_e1686 * var_tcr_dn3)), (((p.p61 * var_tcvsat_dn4) * var_tcr) + (assign1700_e1686 * var_tcr_dn4)), (((p.p61 * var_tcvsat_dn5) * var_tcr) + (assign1700_e1686 * var_tcr_dn5)), (((p.p61 * var_tcvsat_db0) * var_tcr) + (assign1700_e1686 * var_tcr_db0)), (((p.p61 * var_tcvsat_db1) * var_tcr) + (assign1700_e1686 * var_tcr_db1)),)
    } else {
        (var_ecorn_t, var_ecorn_t_dn0, var_ecorn_t_dn1, var_ecorn_t_dn2, var_ecorn_t_dn3, var_ecorn_t_dn4, var_ecorn_t_dn5, var_ecorn_t_db0, var_ecorn_t_db1,)
    }
};
        var_ecorn_t = assign1700_e1690;
        var_ecorn_t_dn0 = assign1700_e1690_d_n0;
        var_ecorn_t_dn1 = assign1700_e1690_d_n1;
        var_ecorn_t_dn2 = assign1700_e1690_d_n2;
        var_ecorn_t_dn3 = assign1700_e1690_d_n3;
        var_ecorn_t_dn4 = assign1700_e1690_d_n4;
        var_ecorn_t_dn5 = assign1700_e1690_d_n5;
        var_ecorn_t_db0 = assign1700_e1690_d_b0;
        var_ecorn_t_db1 = assign1700_e1690_d_b1;
        var_ecorn_t_rv = 0.0;
        var_ecorn_t_rdn0 = 0.0;
        var_ecorn_t_rdn1 = 0.0;
        var_ecorn_t_rdn2 = 0.0;
        var_ecorn_t_rdn3 = 0.0;
        var_ecorn_t_rdn4 = 0.0;
        var_ecorn_t_rdn5 = 0.0;
        var_ecorn_t_rdb0 = 0.0;
        var_ecorn_t_rdb1 = 0.0;

        let (assign1710_e1700, assign1710_e1700_d_n0, assign1710_e1700_d_n1, assign1710_e1700_d_n2, assign1710_e1700_d_n3, assign1710_e1700_d_n4, assign1710_e1700_d_n5, assign1710_e1700_d_b0, assign1710_e1700_d_b1,) = {
    if ((var_guard137 != 0.0) && (p.p62 != 0.0)) {
        let assign1710_e1696: f64 = (p.p60 * var_tcvsat);
        let assign1710_e1698: f64 = (assign1710_e1696 * var_tcr);
        (assign1710_e1698, (((p.p60 * var_tcvsat_dn0) * var_tcr) + (assign1710_e1696 * var_tcr_dn0)), (((p.p60 * var_tcvsat_dn1) * var_tcr) + (assign1710_e1696 * var_tcr_dn1)), (((p.p60 * var_tcvsat_dn2) * var_tcr) + (assign1710_e1696 * var_tcr_dn2)), (((p.p60 * var_tcvsat_dn3) * var_tcr) + (assign1710_e1696 * var_tcr_dn3)), (((p.p60 * var_tcvsat_dn4) * var_tcr) + (assign1710_e1696 * var_tcr_dn4)), (((p.p60 * var_tcvsat_dn5) * var_tcr) + (assign1710_e1696 * var_tcr_dn5)), (((p.p60 * var_tcvsat_db0) * var_tcr) + (assign1710_e1696 * var_tcr_db0)), (((p.p60 * var_tcvsat_db1) * var_tcr) + (assign1710_e1696 * var_tcr_db1)),)
    } else {
        (var_ecrit_t, var_ecrit_t_dn0, var_ecrit_t_dn1, var_ecrit_t_dn2, var_ecrit_t_dn3, var_ecrit_t_dn4, var_ecrit_t_dn5, var_ecrit_t_db0, var_ecrit_t_db1,)
    }
};
        var_ecrit_t = assign1710_e1700;
        var_ecrit_t_dn0 = assign1710_e1700_d_n0;
        var_ecrit_t_dn1 = assign1710_e1700_d_n1;
        var_ecrit_t_dn2 = assign1710_e1700_d_n2;
        var_ecrit_t_dn3 = assign1710_e1700_d_n3;
        var_ecrit_t_dn4 = assign1710_e1700_d_n4;
        var_ecrit_t_dn5 = assign1710_e1700_d_n5;
        var_ecrit_t_db0 = assign1710_e1700_d_b0;
        var_ecrit_t_db1 = assign1710_e1700_d_b1;
        var_ecrit_t_rv = 0.0;
        var_ecrit_t_rdn0 = 0.0;
        var_ecrit_t_rdn1 = 0.0;
        var_ecrit_t_rdn2 = 0.0;
        var_ecrit_t_rdn3 = 0.0;
        var_ecrit_t_rdn4 = 0.0;
        var_ecrit_t_rdn5 = 0.0;
        var_ecrit_t_rdb0 = 0.0;
        var_ecrit_t_rdb1 = 0.0;

        let (assign1720_e1707, assign1720_e1707_d_n0, assign1720_e1707_d_n1, assign1720_e1707_d_n2, assign1720_e1707_d_n3, assign1720_e1707_d_n4, assign1720_e1707_d_n5, assign1720_e1707_d_b0, assign1720_e1707_d_b1,) = {
    if ((var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p61, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ecorn_t, var_ecorn_t_dn0, var_ecorn_t_dn1, var_ecorn_t_dn2, var_ecorn_t_dn3, var_ecorn_t_dn4, var_ecorn_t_dn5, var_ecorn_t_db0, var_ecorn_t_db1,)
    }
};
        var_ecorn_t = assign1720_e1707;
        var_ecorn_t_dn0 = assign1720_e1707_d_n0;
        var_ecorn_t_dn1 = assign1720_e1707_d_n1;
        var_ecorn_t_dn2 = assign1720_e1707_d_n2;
        var_ecorn_t_dn3 = assign1720_e1707_d_n3;
        var_ecorn_t_dn4 = assign1720_e1707_d_n4;
        var_ecorn_t_dn5 = assign1720_e1707_d_n5;
        var_ecorn_t_db0 = assign1720_e1707_d_b0;
        var_ecorn_t_db1 = assign1720_e1707_d_b1;
        var_ecorn_t_rv = 0.0;
        var_ecorn_t_rdn0 = 0.0;
        var_ecorn_t_rdn1 = 0.0;
        var_ecorn_t_rdn2 = 0.0;
        var_ecorn_t_rdn3 = 0.0;
        var_ecorn_t_rdn4 = 0.0;
        var_ecorn_t_rdn5 = 0.0;
        var_ecorn_t_rdb0 = 0.0;
        var_ecorn_t_rdb1 = 0.0;

        let (assign1730_e1714, assign1730_e1714_d_n0, assign1730_e1714_d_n1, assign1730_e1714_d_n2, assign1730_e1714_d_n3, assign1730_e1714_d_n4, assign1730_e1714_d_n5, assign1730_e1714_d_b0, assign1730_e1714_d_b1,) = {
    if ((var_guard137 != 0.0) && (p.p62 == 0.0)) {
        (p.p60, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ecrit_t, var_ecrit_t_dn0, var_ecrit_t_dn1, var_ecrit_t_dn2, var_ecrit_t_dn3, var_ecrit_t_dn4, var_ecrit_t_dn5, var_ecrit_t_db0, var_ecrit_t_db1,)
    }
};
        var_ecrit_t = assign1730_e1714;
        var_ecrit_t_dn0 = assign1730_e1714_d_n0;
        var_ecrit_t_dn1 = assign1730_e1714_d_n1;
        var_ecrit_t_dn2 = assign1730_e1714_d_n2;
        var_ecrit_t_dn3 = assign1730_e1714_d_n3;
        var_ecrit_t_dn4 = assign1730_e1714_d_n4;
        var_ecrit_t_dn5 = assign1730_e1714_d_n5;
        var_ecrit_t_db0 = assign1730_e1714_d_b0;
        var_ecrit_t_db1 = assign1730_e1714_d_b1;
        var_ecrit_t_rv = 0.0;
        var_ecrit_t_rdn0 = 0.0;
        var_ecrit_t_rdn1 = 0.0;
        var_ecrit_t_rdn2 = 0.0;
        var_ecrit_t_rdn3 = 0.0;
        var_ecrit_t_rdn4 = 0.0;
        var_ecrit_t_rdn5 = 0.0;
        var_ecrit_t_rdb0 = 0.0;
        var_ecrit_t_rdb1 = 0.0;

        let (assign1770_e1766, assign1770_e1766_d_n0, assign1770_e1766_d_n1, assign1770_e1766_d_n2, assign1770_e1766_d_n3, assign1770_e1766_d_n4, assign1770_e1766_d_n5, assign1770_e1766_d_b0, assign1770_e1766_d_b1,) = {
    if (var_guard137 != 0.0) {
        let assign1770_e1764: f64 = (var_ecrit_t - var_ecorn_t);
        (assign1770_e1764, (var_ecrit_t_dn0 - var_ecorn_t_dn0), (var_ecrit_t_dn1 - var_ecorn_t_dn1), (var_ecrit_t_dn2 - var_ecorn_t_dn2), (var_ecrit_t_dn3 - var_ecorn_t_dn3), (var_ecrit_t_dn4 - var_ecorn_t_dn4), (var_ecrit_t_dn5 - var_ecorn_t_dn5), (var_ecrit_t_db0 - var_ecorn_t_db0), (var_ecrit_t_db1 - var_ecorn_t_db1),)
    } else {
        (var_de, var_de_dn0, var_de_dn1, var_de_dn2, var_de_dn3, var_de_dn4, var_de_dn5, var_de_db0, var_de_db1,)
    }
};
        var_de = assign1770_e1766;
        var_de_dn0 = assign1770_e1766_d_n0;
        var_de_dn1 = assign1770_e1766_d_n1;
        var_de_dn2 = assign1770_e1766_d_n2;
        var_de_dn3 = assign1770_e1766_d_n3;
        var_de_dn4 = assign1770_e1766_d_n4;
        var_de_dn5 = assign1770_e1766_d_n5;
        var_de_db0 = assign1770_e1766_d_b0;
        var_de_db1 = assign1770_e1766_d_b1;
        var_de_rv = 0.0;
        var_de_rdn0 = 0.0;
        var_de_rdn1 = 0.0;
        var_de_rdn2 = 0.0;
        var_de_rdn3 = 0.0;
        var_de_rdn4 = 0.0;
        var_de_rdn5 = 0.0;
        var_de_rdb0 = 0.0;
        var_de_rdb1 = 0.0;

        let (assign1780_e1772, assign1780_e1772_d_n0, assign1780_e1772_d_n1, assign1780_e1772_d_n2, assign1780_e1772_d_n3, assign1780_e1772_d_n4, assign1780_e1772_d_n5, assign1780_e1772_d_b0, assign1780_e1772_d_b1,) = {
    if (var_guard137 != 0.0) {
        let assign1780_e1770: f64 = (1.0 / var_ecrit_t);
        (assign1780_e1770, (-(var_ecrit_t_dn0 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_dn1 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_dn2 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_dn3 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_dn4 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_dn5 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_db0 / (var_ecrit_t * var_ecrit_t))), (-(var_ecrit_t_db1 / (var_ecrit_t * var_ecrit_t))),)
    } else {
        (var_iecrit, var_iecrit_dn0, var_iecrit_dn1, var_iecrit_dn2, var_iecrit_dn3, var_iecrit_dn4, var_iecrit_dn5, var_iecrit_db0, var_iecrit_db1,)
    }
};
        var_iecrit = assign1780_e1772;
        var_iecrit_dn0 = assign1780_e1772_d_n0;
        var_iecrit_dn1 = assign1780_e1772_d_n1;
        var_iecrit_dn2 = assign1780_e1772_d_n2;
        var_iecrit_dn3 = assign1780_e1772_d_n3;
        var_iecrit_dn4 = assign1780_e1772_d_n4;
        var_iecrit_dn5 = assign1780_e1772_d_n5;
        var_iecrit_db0 = assign1780_e1772_d_b0;
        var_iecrit_db1 = assign1780_e1772_d_b1;
        var_iecrit_rv = 0.0;
        var_iecrit_rdn0 = 0.0;
        var_iecrit_rdn1 = 0.0;
        var_iecrit_rdn2 = 0.0;
        var_iecrit_rdn3 = 0.0;
        var_iecrit_rdn4 = 0.0;
        var_iecrit_rdn5 = 0.0;
        var_iecrit_rdb0 = 0.0;
        var_iecrit_rdb1 = 0.0;

        let (assign1820_e1792, assign1820_e1792_d_n0, assign1820_e1792_d_n1, assign1820_e1792_d_n2, assign1820_e1792_d_n3, assign1820_e1792_d_n4, assign1820_e1792_d_n5, assign1820_e1792_d_b0, assign1820_e1792_d_b1,) = {
    if (var_guard137 == 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_de, var_de_dn0, var_de_dn1, var_de_dn2, var_de_dn3, var_de_dn4, var_de_dn5, var_de_db0, var_de_db1,)
    }
};
        var_de = assign1820_e1792;
        var_de_dn0 = assign1820_e1792_d_n0;
        var_de_dn1 = assign1820_e1792_d_n1;
        var_de_dn2 = assign1820_e1792_d_n2;
        var_de_dn3 = assign1820_e1792_d_n3;
        var_de_dn4 = assign1820_e1792_d_n4;
        var_de_dn5 = assign1820_e1792_d_n5;
        var_de_db0 = assign1820_e1792_d_b0;
        var_de_db1 = assign1820_e1792_d_b1;
        var_de_rv = 0.0;
        var_de_rdn0 = 0.0;
        var_de_rdn1 = 0.0;
        var_de_rdn2 = 0.0;
        var_de_rdn3 = 0.0;
        var_de_rdn4 = 0.0;
        var_de_rdn5 = 0.0;
        var_de_rdb0 = 0.0;
        var_de_rdb1 = 0.0;

        let (assign1830_e1797, assign1830_e1797_d_n0, assign1830_e1797_d_n1, assign1830_e1797_d_n2, assign1830_e1797_d_n3, assign1830_e1797_d_n4, assign1830_e1797_d_n5, assign1830_e1797_d_b0, assign1830_e1797_d_b1,) = {
    if (var_guard137 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_iecrit, var_iecrit_dn0, var_iecrit_dn1, var_iecrit_dn2, var_iecrit_dn3, var_iecrit_dn4, var_iecrit_dn5, var_iecrit_db0, var_iecrit_db1,)
    }
};
        var_iecrit = assign1830_e1797;
        var_iecrit_dn0 = assign1830_e1797_d_n0;
        var_iecrit_dn1 = assign1830_e1797_d_n1;
        var_iecrit_dn2 = assign1830_e1797_d_n2;
        var_iecrit_dn3 = assign1830_e1797_d_n3;
        var_iecrit_dn4 = assign1830_e1797_d_n4;
        var_iecrit_dn5 = assign1830_e1797_d_n5;
        var_iecrit_db0 = assign1830_e1797_d_b0;
        var_iecrit_db1 = assign1830_e1797_d_b1;
        var_iecrit_rv = 0.0;
        var_iecrit_rdn0 = 0.0;
        var_iecrit_rdn1 = 0.0;
        var_iecrit_rdn2 = 0.0;
        var_iecrit_rdn3 = 0.0;
        var_iecrit_rdn4 = 0.0;
        var_iecrit_rdn5 = 0.0;
        var_iecrit_rdb0 = 0.0;
        var_iecrit_rdb1 = 0.0;

        let assign1840_e1800: f64 = (var_leffe_um * var_de);
        var_lde = assign1840_e1800;
        var_lde_dn0 = ((var_leffe_um_dn0 * var_de) + (var_leffe_um * var_de_dn0));
        var_lde_dn1 = ((var_leffe_um_dn1 * var_de) + (var_leffe_um * var_de_dn1));
        var_lde_dn2 = ((var_leffe_um_dn2 * var_de) + (var_leffe_um * var_de_dn2));
        var_lde_dn3 = ((var_leffe_um_dn3 * var_de) + (var_leffe_um * var_de_dn3));
        var_lde_dn4 = ((var_leffe_um_dn4 * var_de) + (var_leffe_um * var_de_dn4));
        var_lde_dn5 = ((var_leffe_um_dn5 * var_de) + (var_leffe_um * var_de_dn5));
        var_lde_db0 = ((var_leffe_um_db0 * var_de) + (var_leffe_um * var_de_db0));
        var_lde_db1 = ((var_leffe_um_db1 * var_de) + (var_leffe_um * var_de_db1));
        var_lde_rv = 0.0;
        var_lde_rdn0 = 0.0;
        var_lde_rdn1 = 0.0;
        var_lde_rdn2 = 0.0;
        var_lde_rdn3 = 0.0;
        var_lde_rdn4 = 0.0;
        var_lde_rdn5 = 0.0;
        var_lde_rdb0 = 0.0;
        var_lde_rdb1 = 0.0;

        let assign1850_e1803: f64 = if var_lde > 100000.0 { 1.0 } else { 0.0 };
        var_guard138 = assign1850_e1803;
        var_guard138_dn0 = 0.0;
        var_guard138_dn1 = 0.0;
        var_guard138_dn2 = 0.0;
        var_guard138_dn3 = 0.0;
        var_guard138_dn4 = 0.0;
        var_guard138_dn5 = 0.0;
        var_guard138_db0 = 0.0;
        var_guard138_db1 = 0.0;
        var_guard138_rv = 0.0;
        var_guard138_rdn0 = 0.0;
        var_guard138_rdn1 = 0.0;
        var_guard138_rdn2 = 0.0;
        var_guard138_rdn3 = 0.0;
        var_guard138_rdn4 = 0.0;
        var_guard138_rdn5 = 0.0;
        var_guard138_rdb0 = 0.0;
        var_guard138_rdb1 = 0.0;

        let (assign1860_e1807, assign1860_e1807_d_n0, assign1860_e1807_d_n1, assign1860_e1807_d_n2, assign1860_e1807_d_n3, assign1860_e1807_d_n4, assign1860_e1807_d_n5, assign1860_e1807_d_b0, assign1860_e1807_d_b1,) = {
    if (var_guard138 != 0.0) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lde, var_lde_dn0, var_lde_dn1, var_lde_dn2, var_lde_dn3, var_lde_dn4, var_lde_dn5, var_lde_db0, var_lde_db1,)
    }
};
        var_lde = assign1860_e1807;
        var_lde_dn0 = assign1860_e1807_d_n0;
        var_lde_dn1 = assign1860_e1807_d_n1;
        var_lde_dn2 = assign1860_e1807_d_n2;
        var_lde_dn3 = assign1860_e1807_d_n3;
        var_lde_dn4 = assign1860_e1807_d_n4;
        var_lde_dn5 = assign1860_e1807_d_n5;
        var_lde_db0 = assign1860_e1807_d_b0;
        var_lde_db1 = assign1860_e1807_d_b1;
        var_lde_rv = 0.0;
        var_lde_rdn0 = 0.0;
        var_lde_rdn1 = 0.0;
        var_lde_rdn2 = 0.0;
        var_lde_rdn3 = 0.0;
        var_lde_rdn4 = 0.0;
        var_lde_rdn5 = 0.0;
        var_lde_rdb0 = 0.0;
        var_lde_rdb1 = 0.0;

        *var_cjp_t_slot = var_cjp_t;
        *var_cjp_t_db0_slot = var_cjp_t_db0;
        *var_cjp_t_db1_slot = var_cjp_t_db1;
        *var_cjp_t_dn0_slot = var_cjp_t_dn0;
        *var_cjp_t_dn1_slot = var_cjp_t_dn1;
        *var_cjp_t_dn2_slot = var_cjp_t_dn2;
        *var_cjp_t_dn3_slot = var_cjp_t_dn3;
        *var_cjp_t_dn4_slot = var_cjp_t_dn4;
        *var_cjp_t_dn5_slot = var_cjp_t_dn5;
        *var_cjp_t_rdb0_slot = var_cjp_t_rdb0;
        *var_cjp_t_rdb1_slot = var_cjp_t_rdb1;
        *var_cjp_t_rdn0_slot = var_cjp_t_rdn0;
        *var_cjp_t_rdn1_slot = var_cjp_t_rdn1;
        *var_cjp_t_rdn2_slot = var_cjp_t_rdn2;
        *var_cjp_t_rdn3_slot = var_cjp_t_rdn3;
        *var_cjp_t_rdn4_slot = var_cjp_t_rdn4;
        *var_cjp_t_rdn5_slot = var_cjp_t_rdn5;
        *var_cjp_t_rv_slot = var_cjp_t_rv;
        *var_de_slot = var_de;
        *var_de_db0_slot = var_de_db0;
        *var_de_db1_slot = var_de_db1;
        *var_de_dn0_slot = var_de_dn0;
        *var_de_dn1_slot = var_de_dn1;
        *var_de_dn2_slot = var_de_dn2;
        *var_de_dn3_slot = var_de_dn3;
        *var_de_dn4_slot = var_de_dn4;
        *var_de_dn5_slot = var_de_dn5;
        *var_de_rdb0_slot = var_de_rdb0;
        *var_de_rdb1_slot = var_de_rdb1;
        *var_de_rdn0_slot = var_de_rdn0;
        *var_de_rdn1_slot = var_de_rdn1;
        *var_de_rdn2_slot = var_de_rdn2;
        *var_de_rdn3_slot = var_de_rdn3;
        *var_de_rdn4_slot = var_de_rdn4;
        *var_de_rdn5_slot = var_de_rdn5;
        *var_de_rv_slot = var_de_rv;
        *var_ecorn_t_slot = var_ecorn_t;
        *var_ecorn_t_db0_slot = var_ecorn_t_db0;
        *var_ecorn_t_db1_slot = var_ecorn_t_db1;
        *var_ecorn_t_dn0_slot = var_ecorn_t_dn0;
        *var_ecorn_t_dn1_slot = var_ecorn_t_dn1;
        *var_ecorn_t_dn2_slot = var_ecorn_t_dn2;
        *var_ecorn_t_dn3_slot = var_ecorn_t_dn3;
        *var_ecorn_t_dn4_slot = var_ecorn_t_dn4;
        *var_ecorn_t_dn5_slot = var_ecorn_t_dn5;
        *var_ecorn_t_rdb0_slot = var_ecorn_t_rdb0;
        *var_ecorn_t_rdb1_slot = var_ecorn_t_rdb1;
        *var_ecorn_t_rdn0_slot = var_ecorn_t_rdn0;
        *var_ecorn_t_rdn1_slot = var_ecorn_t_rdn1;
        *var_ecorn_t_rdn2_slot = var_ecorn_t_rdn2;
        *var_ecorn_t_rdn3_slot = var_ecorn_t_rdn3;
        *var_ecorn_t_rdn4_slot = var_ecorn_t_rdn4;
        *var_ecorn_t_rdn5_slot = var_ecorn_t_rdn5;
        *var_ecorn_t_rv_slot = var_ecorn_t_rv;
        *var_ecrit_t_slot = var_ecrit_t;
        *var_ecrit_t_db0_slot = var_ecrit_t_db0;
        *var_ecrit_t_db1_slot = var_ecrit_t_db1;
        *var_ecrit_t_dn0_slot = var_ecrit_t_dn0;
        *var_ecrit_t_dn1_slot = var_ecrit_t_dn1;
        *var_ecrit_t_dn2_slot = var_ecrit_t_dn2;
        *var_ecrit_t_dn3_slot = var_ecrit_t_dn3;
        *var_ecrit_t_dn4_slot = var_ecrit_t_dn4;
        *var_ecrit_t_dn5_slot = var_ecrit_t_dn5;
        *var_ecrit_t_rdb0_slot = var_ecrit_t_rdb0;
        *var_ecrit_t_rdb1_slot = var_ecrit_t_rdb1;
        *var_ecrit_t_rdn0_slot = var_ecrit_t_rdn0;
        *var_ecrit_t_rdn1_slot = var_ecrit_t_rdn1;
        *var_ecrit_t_rdn2_slot = var_ecrit_t_rdn2;
        *var_ecrit_t_rdn3_slot = var_ecrit_t_rdn3;
        *var_ecrit_t_rdn4_slot = var_ecrit_t_rdn4;
        *var_ecrit_t_rdn5_slot = var_ecrit_t_rdn5;
        *var_ecrit_t_rv_slot = var_ecrit_t_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_db0_slot = var_guard137_db0;
        *var_guard137_db1_slot = var_guard137_db1;
        *var_guard137_dn0_slot = var_guard137_dn0;
        *var_guard137_dn1_slot = var_guard137_dn1;
        *var_guard137_dn2_slot = var_guard137_dn2;
        *var_guard137_dn3_slot = var_guard137_dn3;
        *var_guard137_dn4_slot = var_guard137_dn4;
        *var_guard137_dn5_slot = var_guard137_dn5;
        *var_guard137_rdb0_slot = var_guard137_rdb0;
        *var_guard137_rdb1_slot = var_guard137_rdb1;
        *var_guard137_rdn0_slot = var_guard137_rdn0;
        *var_guard137_rdn1_slot = var_guard137_rdn1;
        *var_guard137_rdn2_slot = var_guard137_rdn2;
        *var_guard137_rdn3_slot = var_guard137_rdn3;
        *var_guard137_rdn4_slot = var_guard137_rdn4;
        *var_guard137_rdn5_slot = var_guard137_rdn5;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_db0_slot = var_guard138_db0;
        *var_guard138_db1_slot = var_guard138_db1;
        *var_guard138_dn0_slot = var_guard138_dn0;
        *var_guard138_dn1_slot = var_guard138_dn1;
        *var_guard138_dn2_slot = var_guard138_dn2;
        *var_guard138_dn3_slot = var_guard138_dn3;
        *var_guard138_dn4_slot = var_guard138_dn4;
        *var_guard138_dn5_slot = var_guard138_dn5;
        *var_guard138_rdb0_slot = var_guard138_rdb0;
        *var_guard138_rdb1_slot = var_guard138_rdb1;
        *var_guard138_rdn0_slot = var_guard138_rdn0;
        *var_guard138_rdn1_slot = var_guard138_rdn1;
        *var_guard138_rdn2_slot = var_guard138_rdn2;
        *var_guard138_rdn3_slot = var_guard138_rdn3;
        *var_guard138_rdn4_slot = var_guard138_rdn4;
        *var_guard138_rdn5_slot = var_guard138_rdn5;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_iecrit_slot = var_iecrit;
        *var_iecrit_db0_slot = var_iecrit_db0;
        *var_iecrit_db1_slot = var_iecrit_db1;
        *var_iecrit_dn0_slot = var_iecrit_dn0;
        *var_iecrit_dn1_slot = var_iecrit_dn1;
        *var_iecrit_dn2_slot = var_iecrit_dn2;
        *var_iecrit_dn3_slot = var_iecrit_dn3;
        *var_iecrit_dn4_slot = var_iecrit_dn4;
        *var_iecrit_dn5_slot = var_iecrit_dn5;
        *var_iecrit_rdb0_slot = var_iecrit_rdb0;
        *var_iecrit_rdb1_slot = var_iecrit_rdb1;
        *var_iecrit_rdn0_slot = var_iecrit_rdn0;
        *var_iecrit_rdn1_slot = var_iecrit_rdn1;
        *var_iecrit_rdn2_slot = var_iecrit_rdn2;
        *var_iecrit_rdn3_slot = var_iecrit_rdn3;
        *var_iecrit_rdn4_slot = var_iecrit_rdn4;
        *var_iecrit_rdn5_slot = var_iecrit_rdn5;
        *var_iecrit_rv_slot = var_iecrit_rv;
        *var_lde_slot = var_lde;
        *var_lde_db0_slot = var_lde_db0;
        *var_lde_db1_slot = var_lde_db1;
        *var_lde_dn0_slot = var_lde_dn0;
        *var_lde_dn1_slot = var_lde_dn1;
        *var_lde_dn2_slot = var_lde_dn2;
        *var_lde_dn3_slot = var_lde_dn3;
        *var_lde_dn4_slot = var_lde_dn4;
        *var_lde_dn5_slot = var_lde_dn5;
        *var_lde_rdb0_slot = var_lde_rdb0;
        *var_lde_rdb1_slot = var_lde_rdb1;
        *var_lde_rdn0_slot = var_lde_rdn0;
        *var_lde_rdn1_slot = var_lde_rdn1;
        *var_lde_rdn2_slot = var_lde_rdn2;
        *var_lde_rdn3_slot = var_lde_rdn3;
        *var_lde_rdn4_slot = var_lde_rdn4;
        *var_lde_rdn5_slot = var_lde_rdn5;
        *var_lde_rv_slot = var_lde_rv;
        *var_pp_t_slot = var_pp_t;
        *var_pp_t_db0_slot = var_pp_t_db0;
        *var_pp_t_db1_slot = var_pp_t_db1;
        *var_pp_t_dn0_slot = var_pp_t_dn0;
        *var_pp_t_dn1_slot = var_pp_t_dn1;
        *var_pp_t_dn2_slot = var_pp_t_dn2;
        *var_pp_t_dn3_slot = var_pp_t_dn3;
        *var_pp_t_dn4_slot = var_pp_t_dn4;
        *var_pp_t_dn5_slot = var_pp_t_dn5;
        *var_pp_t_rdb0_slot = var_pp_t_rdb0;
        *var_pp_t_rdb1_slot = var_pp_t_rdb1;
        *var_pp_t_rdn0_slot = var_pp_t_rdn0;
        *var_pp_t_rdn1_slot = var_pp_t_rdn1;
        *var_pp_t_rdn2_slot = var_pp_t_rdn2;
        *var_pp_t_rdn3_slot = var_pp_t_rdn3;
        *var_pp_t_rdn4_slot = var_pp_t_rdn4;
        *var_pp_t_rdn5_slot = var_pp_t_rdn5;
        *var_pp_t_rv_slot = var_pp_t_rv;
        *var_psiin__blk135_slot = var_psiin__blk135;
        *var_psiin__blk135_db0_slot = var_psiin__blk135_db0;
        *var_psiin__blk135_db1_slot = var_psiin__blk135_db1;
        *var_psiin__blk135_dn0_slot = var_psiin__blk135_dn0;
        *var_psiin__blk135_dn1_slot = var_psiin__blk135_dn1;
        *var_psiin__blk135_dn2_slot = var_psiin__blk135_dn2;
        *var_psiin__blk135_dn3_slot = var_psiin__blk135_dn3;
        *var_psiin__blk135_dn4_slot = var_psiin__blk135_dn4;
        *var_psiin__blk135_dn5_slot = var_psiin__blk135_dn5;
        *var_psiin__blk135_rdb0_slot = var_psiin__blk135_rdb0;
        *var_psiin__blk135_rdb1_slot = var_psiin__blk135_rdb1;
        *var_psiin__blk135_rdn0_slot = var_psiin__blk135_rdn0;
        *var_psiin__blk135_rdn1_slot = var_psiin__blk135_rdn1;
        *var_psiin__blk135_rdn2_slot = var_psiin__blk135_rdn2;
        *var_psiin__blk135_rdn3_slot = var_psiin__blk135_rdn3;
        *var_psiin__blk135_rdn4_slot = var_psiin__blk135_rdn4;
        *var_psiin__blk135_rdn5_slot = var_psiin__blk135_rdn5;
        *var_psiin__blk135_rv_slot = var_psiin__blk135_rv;
        *var_psiio__blk134_slot = var_psiio__blk134;
        *var_psiio__blk134_db0_slot = var_psiio__blk134_db0;
        *var_psiio__blk134_db1_slot = var_psiio__blk134_db1;
        *var_psiio__blk134_dn0_slot = var_psiio__blk134_dn0;
        *var_psiio__blk134_dn1_slot = var_psiio__blk134_dn1;
        *var_psiio__blk134_dn2_slot = var_psiio__blk134_dn2;
        *var_psiio__blk134_dn3_slot = var_psiio__blk134_dn3;
        *var_psiio__blk134_dn4_slot = var_psiio__blk134_dn4;
        *var_psiio__blk134_dn5_slot = var_psiio__blk134_dn5;
        *var_psiio__blk134_rdb0_slot = var_psiio__blk134_rdb0;
        *var_psiio__blk134_rdb1_slot = var_psiio__blk134_rdb1;
        *var_psiio__blk134_rdn0_slot = var_psiio__blk134_rdn0;
        *var_psiio__blk134_rdn1_slot = var_psiio__blk134_rdn1;
        *var_psiio__blk134_rdn2_slot = var_psiio__blk134_rdn2;
        *var_psiio__blk134_rdn3_slot = var_psiio__blk134_rdn3;
        *var_psiio__blk134_rdn4_slot = var_psiio__blk134_rdn4;
        *var_psiio__blk134_rdn5_slot = var_psiio__blk134_rdn5;
        *var_psiio__blk134_rv_slot = var_psiio__blk134_rv;
    }

    pub(super) fn stamp_reactive_block_6(
        p: &Parameters,
        var_cj1: f64,
        var_dfsq: f64,
        var_dfsq_db0: f64,
        var_dfsq_db1: f64,
        var_dfsq_dn0: f64,
        var_dfsq_dn1: f64,
        var_dfsq_dn2: f64,
        var_dfsq_dn3: f64,
        var_dfsq_dn4: f64,
        var_dfsq_dn5: f64,
        var_dp_i: f64,
        var_dp_i_db0: f64,
        var_dp_i_db1: f64,
        var_dp_i_dn0: f64,
        var_dp_i_dn1: f64,
        var_dp_i_dn2: f64,
        var_dp_i_dn3: f64,
        var_dp_i_dn4: f64,
        var_dp_i_dn5: f64,
        var_iecrit: f64,
        var_lde: f64,
        var_lde_db0: f64,
        var_lde_db1: f64,
        var_lde_dn0: f64,
        var_lde_dn1: f64,
        var_lde_dn2: f64,
        var_lde_dn3: f64,
        var_lde_dn4: f64,
        var_lde_dn5: f64,
        var_nsteff: f64,
        var_nsteff_db0: f64,
        var_nsteff_db1: f64,
        var_nsteff_dn0: f64,
        var_nsteff_dn1: f64,
        var_nsteff_dn2: f64,
        var_nsteff_dn3: f64,
        var_nsteff_dn4: f64,
        var_nsteff_dn5: f64,
        var_vc1: f64,
        var_vc1_db0: f64,
        var_vc1_db1: f64,
        var_vc1_dn0: f64,
        var_vc1_dn1: f64,
        var_vc1_dn2: f64,
        var_vc1_dn3: f64,
        var_vc1_dn4: f64,
        var_vc1_dn5: f64,
        var_vc2: f64,
        var_vc2_db0: f64,
        var_vc2_db1: f64,
        var_vc2_dn0: f64,
        var_vc2_dn1: f64,
        var_vc2_dn2: f64,
        var_vc2_dn3: f64,
        var_vc2_dn4: f64,
        var_vc2_dn5: f64,
        var_vpoe: f64,
        var_vpoe_db0: f64,
        var_vpoe_db1: f64,
        var_vpoe_dn0: f64,
        var_vpoe_dn1: f64,
        var_vpoe_dn2: f64,
        var_vpoe_dn3: f64,
        var_vpoe_dn4: f64,
        var_vpoe_dn5: f64,
        var_vrb: f64,
        var_vrb_db0: f64,
        var_vrb_db1: f64,
        var_vrb_dn0: f64,
        var_vrb_dn1: f64,
        var_vrb_dn2: f64,
        var_vrb_dn3: f64,
        var_vrb_dn4: f64,
        var_vrb_dn5: f64,
        var_a1_slot: &mut f64,
        var_a1_db0_slot: &mut f64,
        var_a1_db1_slot: &mut f64,
        var_a1_dn0_slot: &mut f64,
        var_a1_dn1_slot: &mut f64,
        var_a1_dn2_slot: &mut f64,
        var_a1_dn3_slot: &mut f64,
        var_a1_dn4_slot: &mut f64,
        var_a1_dn5_slot: &mut f64,
        var_a1_rdb0_slot: &mut f64,
        var_a1_rdb1_slot: &mut f64,
        var_a1_rdn0_slot: &mut f64,
        var_a1_rdn1_slot: &mut f64,
        var_a1_rdn2_slot: &mut f64,
        var_a1_rdn3_slot: &mut f64,
        var_a1_rdn4_slot: &mut f64,
        var_a1_rdn5_slot: &mut f64,
        var_a1_rv_slot: &mut f64,
        var_a2_slot: &mut f64,
        var_a2_db0_slot: &mut f64,
        var_a2_db1_slot: &mut f64,
        var_a2_dn0_slot: &mut f64,
        var_a2_dn1_slot: &mut f64,
        var_a2_dn2_slot: &mut f64,
        var_a2_dn3_slot: &mut f64,
        var_a2_dn4_slot: &mut f64,
        var_a2_dn5_slot: &mut f64,
        var_a2_rdb0_slot: &mut f64,
        var_a2_rdb1_slot: &mut f64,
        var_a2_rdn0_slot: &mut f64,
        var_a2_rdn1_slot: &mut f64,
        var_a2_rdn2_slot: &mut f64,
        var_a2_rdn3_slot: &mut f64,
        var_a2_rdn4_slot: &mut f64,
        var_a2_rdn5_slot: &mut f64,
        var_a2_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_db0_slot: &mut f64,
        var_guard189_db1_slot: &mut f64,
        var_guard189_dn0_slot: &mut f64,
        var_guard189_dn1_slot: &mut f64,
        var_guard189_dn2_slot: &mut f64,
        var_guard189_dn3_slot: &mut f64,
        var_guard189_dn4_slot: &mut f64,
        var_guard189_dn5_slot: &mut f64,
        var_guard189_rdb0_slot: &mut f64,
        var_guard189_rdb1_slot: &mut f64,
        var_guard189_rdn0_slot: &mut f64,
        var_guard189_rdn1_slot: &mut f64,
        var_guard189_rdn2_slot: &mut f64,
        var_guard189_rdn3_slot: &mut f64,
        var_guard189_rdn4_slot: &mut f64,
        var_guard189_rdn5_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_db0_slot: &mut f64,
        var_guard190_db1_slot: &mut f64,
        var_guard190_dn0_slot: &mut f64,
        var_guard190_dn1_slot: &mut f64,
        var_guard190_dn2_slot: &mut f64,
        var_guard190_dn3_slot: &mut f64,
        var_guard190_dn4_slot: &mut f64,
        var_guard190_dn5_slot: &mut f64,
        var_guard190_rdb0_slot: &mut f64,
        var_guard190_rdb1_slot: &mut f64,
        var_guard190_rdn0_slot: &mut f64,
        var_guard190_rdn1_slot: &mut f64,
        var_guard190_rdn2_slot: &mut f64,
        var_guard190_rdn3_slot: &mut f64,
        var_guard190_rdn4_slot: &mut f64,
        var_guard190_rdn5_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_db0_slot: &mut f64,
        var_guard191_db1_slot: &mut f64,
        var_guard191_dn0_slot: &mut f64,
        var_guard191_dn1_slot: &mut f64,
        var_guard191_dn2_slot: &mut f64,
        var_guard191_dn3_slot: &mut f64,
        var_guard191_dn4_slot: &mut f64,
        var_guard191_dn5_slot: &mut f64,
        var_guard191_rdb0_slot: &mut f64,
        var_guard191_rdb1_slot: &mut f64,
        var_guard191_rdn0_slot: &mut f64,
        var_guard191_rdn1_slot: &mut f64,
        var_guard191_rdn2_slot: &mut f64,
        var_guard191_rdn3_slot: &mut f64,
        var_guard191_rdn4_slot: &mut f64,
        var_guard191_rdn5_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_db0_slot: &mut f64,
        var_guard192_db1_slot: &mut f64,
        var_guard192_dn0_slot: &mut f64,
        var_guard192_dn1_slot: &mut f64,
        var_guard192_dn2_slot: &mut f64,
        var_guard192_dn3_slot: &mut f64,
        var_guard192_dn4_slot: &mut f64,
        var_guard192_dn5_slot: &mut f64,
        var_guard192_rdb0_slot: &mut f64,
        var_guard192_rdb1_slot: &mut f64,
        var_guard192_rdn0_slot: &mut f64,
        var_guard192_rdn1_slot: &mut f64,
        var_guard192_rdn2_slot: &mut f64,
        var_guard192_rdn3_slot: &mut f64,
        var_guard192_rdn4_slot: &mut f64,
        var_guard192_rdn5_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_db0_slot: &mut f64,
        var_guard193_db1_slot: &mut f64,
        var_guard193_dn0_slot: &mut f64,
        var_guard193_dn1_slot: &mut f64,
        var_guard193_dn2_slot: &mut f64,
        var_guard193_dn3_slot: &mut f64,
        var_guard193_dn4_slot: &mut f64,
        var_guard193_dn5_slot: &mut f64,
        var_guard193_rdb0_slot: &mut f64,
        var_guard193_rdb1_slot: &mut f64,
        var_guard193_rdn0_slot: &mut f64,
        var_guard193_rdn1_slot: &mut f64,
        var_guard193_rdn2_slot: &mut f64,
        var_guard193_rdn3_slot: &mut f64,
        var_guard193_rdn4_slot: &mut f64,
        var_guard193_rdn5_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard249_db0_slot: &mut f64,
        var_guard249_db1_slot: &mut f64,
        var_guard249_dn0_slot: &mut f64,
        var_guard249_dn1_slot: &mut f64,
        var_guard249_dn2_slot: &mut f64,
        var_guard249_dn3_slot: &mut f64,
        var_guard249_dn4_slot: &mut f64,
        var_guard249_dn5_slot: &mut f64,
        var_guard249_rdb0_slot: &mut f64,
        var_guard249_rdb1_slot: &mut f64,
        var_guard249_rdn0_slot: &mut f64,
        var_guard249_rdn1_slot: &mut f64,
        var_guard249_rdn2_slot: &mut f64,
        var_guard249_rdn3_slot: &mut f64,
        var_guard249_rdn4_slot: &mut f64,
        var_guard249_rdn5_slot: &mut f64,
        var_guard249_rv_slot: &mut f64,
        var_pe_slot: &mut f64,
        var_pe_db0_slot: &mut f64,
        var_pe_db1_slot: &mut f64,
        var_pe_dn0_slot: &mut f64,
        var_pe_dn1_slot: &mut f64,
        var_pe_dn2_slot: &mut f64,
        var_pe_dn3_slot: &mut f64,
        var_pe_dn4_slot: &mut f64,
        var_pe_dn5_slot: &mut f64,
        var_pe_rdb0_slot: &mut f64,
        var_pe_rdb1_slot: &mut f64,
        var_pe_rdn0_slot: &mut f64,
        var_pe_rdn1_slot: &mut f64,
        var_pe_rdn2_slot: &mut f64,
        var_pe_rdn3_slot: &mut f64,
        var_pe_rdn4_slot: &mut f64,
        var_pe_rdn5_slot: &mut f64,
        var_pe_rv_slot: &mut f64,
        var_v1c_slot: &mut f64,
        var_v1c_db0_slot: &mut f64,
        var_v1c_db1_slot: &mut f64,
        var_v1c_dn0_slot: &mut f64,
        var_v1c_dn1_slot: &mut f64,
        var_v1c_dn2_slot: &mut f64,
        var_v1c_dn3_slot: &mut f64,
        var_v1c_dn4_slot: &mut f64,
        var_v1c_dn5_slot: &mut f64,
        var_v1c_rdb0_slot: &mut f64,
        var_v1c_rdb1_slot: &mut f64,
        var_v1c_rdn0_slot: &mut f64,
        var_v1c_rdn1_slot: &mut f64,
        var_v1c_rdn2_slot: &mut f64,
        var_v1c_rdn3_slot: &mut f64,
        var_v1c_rdn4_slot: &mut f64,
        var_v1c_rdn5_slot: &mut f64,
        var_v1c_rv_slot: &mut f64,
        var_v1ci_slot: &mut f64,
        var_v1ci_db0_slot: &mut f64,
        var_v1ci_db1_slot: &mut f64,
        var_v1ci_dn0_slot: &mut f64,
        var_v1ci_dn1_slot: &mut f64,
        var_v1ci_dn2_slot: &mut f64,
        var_v1ci_dn3_slot: &mut f64,
        var_v1ci_dn4_slot: &mut f64,
        var_v1ci_dn5_slot: &mut f64,
        var_v1ci_rdb0_slot: &mut f64,
        var_v1ci_rdb1_slot: &mut f64,
        var_v1ci_rdn0_slot: &mut f64,
        var_v1ci_rdn1_slot: &mut f64,
        var_v1ci_rdn2_slot: &mut f64,
        var_v1ci_rdn3_slot: &mut f64,
        var_v1ci_rdn4_slot: &mut f64,
        var_v1ci_rdn5_slot: &mut f64,
        var_v1ci_rv_slot: &mut f64,
        var_v1cl_slot: &mut f64,
        var_v1cl_db0_slot: &mut f64,
        var_v1cl_db1_slot: &mut f64,
        var_v1cl_dn0_slot: &mut f64,
        var_v1cl_dn1_slot: &mut f64,
        var_v1cl_dn2_slot: &mut f64,
        var_v1cl_dn3_slot: &mut f64,
        var_v1cl_dn4_slot: &mut f64,
        var_v1cl_dn5_slot: &mut f64,
        var_v1cl_rdb0_slot: &mut f64,
        var_v1cl_rdb1_slot: &mut f64,
        var_v1cl_rdn0_slot: &mut f64,
        var_v1cl_rdn1_slot: &mut f64,
        var_v1cl_rdn2_slot: &mut f64,
        var_v1cl_rdn3_slot: &mut f64,
        var_v1cl_rdn4_slot: &mut f64,
        var_v1cl_rdn5_slot: &mut f64,
        var_v1cl_rv_slot: &mut f64,
        var_vrbi_slot: &mut f64,
        var_vrbi_db0_slot: &mut f64,
        var_vrbi_db1_slot: &mut f64,
        var_vrbi_dn0_slot: &mut f64,
        var_vrbi_dn1_slot: &mut f64,
        var_vrbi_dn2_slot: &mut f64,
        var_vrbi_dn3_slot: &mut f64,
        var_vrbi_dn4_slot: &mut f64,
        var_vrbi_dn5_slot: &mut f64,
        var_vrbi_rdb0_slot: &mut f64,
        var_vrbi_rdb1_slot: &mut f64,
        var_vrbi_rdn0_slot: &mut f64,
        var_vrbi_rdn1_slot: &mut f64,
        var_vrbi_rdn2_slot: &mut f64,
        var_vrbi_rdn3_slot: &mut f64,
        var_vrbi_rdn4_slot: &mut f64,
        var_vrbi_rdn5_slot: &mut f64,
        var_vrbi_rv_slot: &mut f64,
    ) {
        let mut var_a1: f64 = *var_a1_slot;
        let mut var_a1_db0: f64 = *var_a1_db0_slot;
        let mut var_a1_db1: f64 = *var_a1_db1_slot;
        let mut var_a1_dn0: f64 = *var_a1_dn0_slot;
        let mut var_a1_dn1: f64 = *var_a1_dn1_slot;
        let mut var_a1_dn2: f64 = *var_a1_dn2_slot;
        let mut var_a1_dn3: f64 = *var_a1_dn3_slot;
        let mut var_a1_dn4: f64 = *var_a1_dn4_slot;
        let mut var_a1_dn5: f64 = *var_a1_dn5_slot;
        let mut var_a1_rdb0: f64 = *var_a1_rdb0_slot;
        let mut var_a1_rdb1: f64 = *var_a1_rdb1_slot;
        let mut var_a1_rdn0: f64 = *var_a1_rdn0_slot;
        let mut var_a1_rdn1: f64 = *var_a1_rdn1_slot;
        let mut var_a1_rdn2: f64 = *var_a1_rdn2_slot;
        let mut var_a1_rdn3: f64 = *var_a1_rdn3_slot;
        let mut var_a1_rdn4: f64 = *var_a1_rdn4_slot;
        let mut var_a1_rdn5: f64 = *var_a1_rdn5_slot;
        let mut var_a1_rv: f64 = *var_a1_rv_slot;
        let mut var_a2: f64 = *var_a2_slot;
        let mut var_a2_db0: f64 = *var_a2_db0_slot;
        let mut var_a2_db1: f64 = *var_a2_db1_slot;
        let mut var_a2_dn0: f64 = *var_a2_dn0_slot;
        let mut var_a2_dn1: f64 = *var_a2_dn1_slot;
        let mut var_a2_dn2: f64 = *var_a2_dn2_slot;
        let mut var_a2_dn3: f64 = *var_a2_dn3_slot;
        let mut var_a2_dn4: f64 = *var_a2_dn4_slot;
        let mut var_a2_dn5: f64 = *var_a2_dn5_slot;
        let mut var_a2_rdb0: f64 = *var_a2_rdb0_slot;
        let mut var_a2_rdb1: f64 = *var_a2_rdb1_slot;
        let mut var_a2_rdn0: f64 = *var_a2_rdn0_slot;
        let mut var_a2_rdn1: f64 = *var_a2_rdn1_slot;
        let mut var_a2_rdn2: f64 = *var_a2_rdn2_slot;
        let mut var_a2_rdn3: f64 = *var_a2_rdn3_slot;
        let mut var_a2_rdn4: f64 = *var_a2_rdn4_slot;
        let mut var_a2_rdn5: f64 = *var_a2_rdn5_slot;
        let mut var_a2_rv: f64 = *var_a2_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_db0: f64 = *var_guard189_db0_slot;
        let mut var_guard189_db1: f64 = *var_guard189_db1_slot;
        let mut var_guard189_dn0: f64 = *var_guard189_dn0_slot;
        let mut var_guard189_dn1: f64 = *var_guard189_dn1_slot;
        let mut var_guard189_dn2: f64 = *var_guard189_dn2_slot;
        let mut var_guard189_dn3: f64 = *var_guard189_dn3_slot;
        let mut var_guard189_dn4: f64 = *var_guard189_dn4_slot;
        let mut var_guard189_dn5: f64 = *var_guard189_dn5_slot;
        let mut var_guard189_rdb0: f64 = *var_guard189_rdb0_slot;
        let mut var_guard189_rdb1: f64 = *var_guard189_rdb1_slot;
        let mut var_guard189_rdn0: f64 = *var_guard189_rdn0_slot;
        let mut var_guard189_rdn1: f64 = *var_guard189_rdn1_slot;
        let mut var_guard189_rdn2: f64 = *var_guard189_rdn2_slot;
        let mut var_guard189_rdn3: f64 = *var_guard189_rdn3_slot;
        let mut var_guard189_rdn4: f64 = *var_guard189_rdn4_slot;
        let mut var_guard189_rdn5: f64 = *var_guard189_rdn5_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_db0: f64 = *var_guard190_db0_slot;
        let mut var_guard190_db1: f64 = *var_guard190_db1_slot;
        let mut var_guard190_dn0: f64 = *var_guard190_dn0_slot;
        let mut var_guard190_dn1: f64 = *var_guard190_dn1_slot;
        let mut var_guard190_dn2: f64 = *var_guard190_dn2_slot;
        let mut var_guard190_dn3: f64 = *var_guard190_dn3_slot;
        let mut var_guard190_dn4: f64 = *var_guard190_dn4_slot;
        let mut var_guard190_dn5: f64 = *var_guard190_dn5_slot;
        let mut var_guard190_rdb0: f64 = *var_guard190_rdb0_slot;
        let mut var_guard190_rdb1: f64 = *var_guard190_rdb1_slot;
        let mut var_guard190_rdn0: f64 = *var_guard190_rdn0_slot;
        let mut var_guard190_rdn1: f64 = *var_guard190_rdn1_slot;
        let mut var_guard190_rdn2: f64 = *var_guard190_rdn2_slot;
        let mut var_guard190_rdn3: f64 = *var_guard190_rdn3_slot;
        let mut var_guard190_rdn4: f64 = *var_guard190_rdn4_slot;
        let mut var_guard190_rdn5: f64 = *var_guard190_rdn5_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_db0: f64 = *var_guard191_db0_slot;
        let mut var_guard191_db1: f64 = *var_guard191_db1_slot;
        let mut var_guard191_dn0: f64 = *var_guard191_dn0_slot;
        let mut var_guard191_dn1: f64 = *var_guard191_dn1_slot;
        let mut var_guard191_dn2: f64 = *var_guard191_dn2_slot;
        let mut var_guard191_dn3: f64 = *var_guard191_dn3_slot;
        let mut var_guard191_dn4: f64 = *var_guard191_dn4_slot;
        let mut var_guard191_dn5: f64 = *var_guard191_dn5_slot;
        let mut var_guard191_rdb0: f64 = *var_guard191_rdb0_slot;
        let mut var_guard191_rdb1: f64 = *var_guard191_rdb1_slot;
        let mut var_guard191_rdn0: f64 = *var_guard191_rdn0_slot;
        let mut var_guard191_rdn1: f64 = *var_guard191_rdn1_slot;
        let mut var_guard191_rdn2: f64 = *var_guard191_rdn2_slot;
        let mut var_guard191_rdn3: f64 = *var_guard191_rdn3_slot;
        let mut var_guard191_rdn4: f64 = *var_guard191_rdn4_slot;
        let mut var_guard191_rdn5: f64 = *var_guard191_rdn5_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_db0: f64 = *var_guard192_db0_slot;
        let mut var_guard192_db1: f64 = *var_guard192_db1_slot;
        let mut var_guard192_dn0: f64 = *var_guard192_dn0_slot;
        let mut var_guard192_dn1: f64 = *var_guard192_dn1_slot;
        let mut var_guard192_dn2: f64 = *var_guard192_dn2_slot;
        let mut var_guard192_dn3: f64 = *var_guard192_dn3_slot;
        let mut var_guard192_dn4: f64 = *var_guard192_dn4_slot;
        let mut var_guard192_dn5: f64 = *var_guard192_dn5_slot;
        let mut var_guard192_rdb0: f64 = *var_guard192_rdb0_slot;
        let mut var_guard192_rdb1: f64 = *var_guard192_rdb1_slot;
        let mut var_guard192_rdn0: f64 = *var_guard192_rdn0_slot;
        let mut var_guard192_rdn1: f64 = *var_guard192_rdn1_slot;
        let mut var_guard192_rdn2: f64 = *var_guard192_rdn2_slot;
        let mut var_guard192_rdn3: f64 = *var_guard192_rdn3_slot;
        let mut var_guard192_rdn4: f64 = *var_guard192_rdn4_slot;
        let mut var_guard192_rdn5: f64 = *var_guard192_rdn5_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_db0: f64 = *var_guard193_db0_slot;
        let mut var_guard193_db1: f64 = *var_guard193_db1_slot;
        let mut var_guard193_dn0: f64 = *var_guard193_dn0_slot;
        let mut var_guard193_dn1: f64 = *var_guard193_dn1_slot;
        let mut var_guard193_dn2: f64 = *var_guard193_dn2_slot;
        let mut var_guard193_dn3: f64 = *var_guard193_dn3_slot;
        let mut var_guard193_dn4: f64 = *var_guard193_dn4_slot;
        let mut var_guard193_dn5: f64 = *var_guard193_dn5_slot;
        let mut var_guard193_rdb0: f64 = *var_guard193_rdb0_slot;
        let mut var_guard193_rdb1: f64 = *var_guard193_rdb1_slot;
        let mut var_guard193_rdn0: f64 = *var_guard193_rdn0_slot;
        let mut var_guard193_rdn1: f64 = *var_guard193_rdn1_slot;
        let mut var_guard193_rdn2: f64 = *var_guard193_rdn2_slot;
        let mut var_guard193_rdn3: f64 = *var_guard193_rdn3_slot;
        let mut var_guard193_rdn4: f64 = *var_guard193_rdn4_slot;
        let mut var_guard193_rdn5: f64 = *var_guard193_rdn5_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard249_db0: f64 = *var_guard249_db0_slot;
        let mut var_guard249_db1: f64 = *var_guard249_db1_slot;
        let mut var_guard249_dn0: f64 = *var_guard249_dn0_slot;
        let mut var_guard249_dn1: f64 = *var_guard249_dn1_slot;
        let mut var_guard249_dn2: f64 = *var_guard249_dn2_slot;
        let mut var_guard249_dn3: f64 = *var_guard249_dn3_slot;
        let mut var_guard249_dn4: f64 = *var_guard249_dn4_slot;
        let mut var_guard249_dn5: f64 = *var_guard249_dn5_slot;
        let mut var_guard249_rdb0: f64 = *var_guard249_rdb0_slot;
        let mut var_guard249_rdb1: f64 = *var_guard249_rdb1_slot;
        let mut var_guard249_rdn0: f64 = *var_guard249_rdn0_slot;
        let mut var_guard249_rdn1: f64 = *var_guard249_rdn1_slot;
        let mut var_guard249_rdn2: f64 = *var_guard249_rdn2_slot;
        let mut var_guard249_rdn3: f64 = *var_guard249_rdn3_slot;
        let mut var_guard249_rdn4: f64 = *var_guard249_rdn4_slot;
        let mut var_guard249_rdn5: f64 = *var_guard249_rdn5_slot;
        let mut var_guard249_rv: f64 = *var_guard249_rv_slot;
        let mut var_pe: f64 = *var_pe_slot;
        let mut var_pe_db0: f64 = *var_pe_db0_slot;
        let mut var_pe_db1: f64 = *var_pe_db1_slot;
        let mut var_pe_dn0: f64 = *var_pe_dn0_slot;
        let mut var_pe_dn1: f64 = *var_pe_dn1_slot;
        let mut var_pe_dn2: f64 = *var_pe_dn2_slot;
        let mut var_pe_dn3: f64 = *var_pe_dn3_slot;
        let mut var_pe_dn4: f64 = *var_pe_dn4_slot;
        let mut var_pe_dn5: f64 = *var_pe_dn5_slot;
        let mut var_pe_rdb0: f64 = *var_pe_rdb0_slot;
        let mut var_pe_rdb1: f64 = *var_pe_rdb1_slot;
        let mut var_pe_rdn0: f64 = *var_pe_rdn0_slot;
        let mut var_pe_rdn1: f64 = *var_pe_rdn1_slot;
        let mut var_pe_rdn2: f64 = *var_pe_rdn2_slot;
        let mut var_pe_rdn3: f64 = *var_pe_rdn3_slot;
        let mut var_pe_rdn4: f64 = *var_pe_rdn4_slot;
        let mut var_pe_rdn5: f64 = *var_pe_rdn5_slot;
        let mut var_pe_rv: f64 = *var_pe_rv_slot;
        let mut var_v1c: f64 = *var_v1c_slot;
        let mut var_v1c_db0: f64 = *var_v1c_db0_slot;
        let mut var_v1c_db1: f64 = *var_v1c_db1_slot;
        let mut var_v1c_dn0: f64 = *var_v1c_dn0_slot;
        let mut var_v1c_dn1: f64 = *var_v1c_dn1_slot;
        let mut var_v1c_dn2: f64 = *var_v1c_dn2_slot;
        let mut var_v1c_dn3: f64 = *var_v1c_dn3_slot;
        let mut var_v1c_dn4: f64 = *var_v1c_dn4_slot;
        let mut var_v1c_dn5: f64 = *var_v1c_dn5_slot;
        let mut var_v1c_rdb0: f64 = *var_v1c_rdb0_slot;
        let mut var_v1c_rdb1: f64 = *var_v1c_rdb1_slot;
        let mut var_v1c_rdn0: f64 = *var_v1c_rdn0_slot;
        let mut var_v1c_rdn1: f64 = *var_v1c_rdn1_slot;
        let mut var_v1c_rdn2: f64 = *var_v1c_rdn2_slot;
        let mut var_v1c_rdn3: f64 = *var_v1c_rdn3_slot;
        let mut var_v1c_rdn4: f64 = *var_v1c_rdn4_slot;
        let mut var_v1c_rdn5: f64 = *var_v1c_rdn5_slot;
        let mut var_v1c_rv: f64 = *var_v1c_rv_slot;
        let mut var_v1ci: f64 = *var_v1ci_slot;
        let mut var_v1ci_db0: f64 = *var_v1ci_db0_slot;
        let mut var_v1ci_db1: f64 = *var_v1ci_db1_slot;
        let mut var_v1ci_dn0: f64 = *var_v1ci_dn0_slot;
        let mut var_v1ci_dn1: f64 = *var_v1ci_dn1_slot;
        let mut var_v1ci_dn2: f64 = *var_v1ci_dn2_slot;
        let mut var_v1ci_dn3: f64 = *var_v1ci_dn3_slot;
        let mut var_v1ci_dn4: f64 = *var_v1ci_dn4_slot;
        let mut var_v1ci_dn5: f64 = *var_v1ci_dn5_slot;
        let mut var_v1ci_rdb0: f64 = *var_v1ci_rdb0_slot;
        let mut var_v1ci_rdb1: f64 = *var_v1ci_rdb1_slot;
        let mut var_v1ci_rdn0: f64 = *var_v1ci_rdn0_slot;
        let mut var_v1ci_rdn1: f64 = *var_v1ci_rdn1_slot;
        let mut var_v1ci_rdn2: f64 = *var_v1ci_rdn2_slot;
        let mut var_v1ci_rdn3: f64 = *var_v1ci_rdn3_slot;
        let mut var_v1ci_rdn4: f64 = *var_v1ci_rdn4_slot;
        let mut var_v1ci_rdn5: f64 = *var_v1ci_rdn5_slot;
        let mut var_v1ci_rv: f64 = *var_v1ci_rv_slot;
        let mut var_v1cl: f64 = *var_v1cl_slot;
        let mut var_v1cl_db0: f64 = *var_v1cl_db0_slot;
        let mut var_v1cl_db1: f64 = *var_v1cl_db1_slot;
        let mut var_v1cl_dn0: f64 = *var_v1cl_dn0_slot;
        let mut var_v1cl_dn1: f64 = *var_v1cl_dn1_slot;
        let mut var_v1cl_dn2: f64 = *var_v1cl_dn2_slot;
        let mut var_v1cl_dn3: f64 = *var_v1cl_dn3_slot;
        let mut var_v1cl_dn4: f64 = *var_v1cl_dn4_slot;
        let mut var_v1cl_dn5: f64 = *var_v1cl_dn5_slot;
        let mut var_v1cl_rdb0: f64 = *var_v1cl_rdb0_slot;
        let mut var_v1cl_rdb1: f64 = *var_v1cl_rdb1_slot;
        let mut var_v1cl_rdn0: f64 = *var_v1cl_rdn0_slot;
        let mut var_v1cl_rdn1: f64 = *var_v1cl_rdn1_slot;
        let mut var_v1cl_rdn2: f64 = *var_v1cl_rdn2_slot;
        let mut var_v1cl_rdn3: f64 = *var_v1cl_rdn3_slot;
        let mut var_v1cl_rdn4: f64 = *var_v1cl_rdn4_slot;
        let mut var_v1cl_rdn5: f64 = *var_v1cl_rdn5_slot;
        let mut var_v1cl_rv: f64 = *var_v1cl_rv_slot;
        let mut var_vrbi: f64 = *var_vrbi_slot;
        let mut var_vrbi_db0: f64 = *var_vrbi_db0_slot;
        let mut var_vrbi_db1: f64 = *var_vrbi_db1_slot;
        let mut var_vrbi_dn0: f64 = *var_vrbi_dn0_slot;
        let mut var_vrbi_dn1: f64 = *var_vrbi_dn1_slot;
        let mut var_vrbi_dn2: f64 = *var_vrbi_dn2_slot;
        let mut var_vrbi_dn3: f64 = *var_vrbi_dn3_slot;
        let mut var_vrbi_dn4: f64 = *var_vrbi_dn4_slot;
        let mut var_vrbi_dn5: f64 = *var_vrbi_dn5_slot;
        let mut var_vrbi_rdb0: f64 = *var_vrbi_rdb0_slot;
        let mut var_vrbi_rdb1: f64 = *var_vrbi_rdb1_slot;
        let mut var_vrbi_rdn0: f64 = *var_vrbi_rdn0_slot;
        let mut var_vrbi_rdn1: f64 = *var_vrbi_rdn1_slot;
        let mut var_vrbi_rdn2: f64 = *var_vrbi_rdn2_slot;
        let mut var_vrbi_rdn3: f64 = *var_vrbi_rdn3_slot;
        let mut var_vrbi_rdn4: f64 = *var_vrbi_rdn4_slot;
        let mut var_vrbi_rdn5: f64 = *var_vrbi_rdn5_slot;
        let mut var_vrbi_rv: f64 = *var_vrbi_rv_slot;

        let assign1870_e1810: f64 = if var_vrb < 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign1870_e1810;
        var_guard189_dn0 = 0.0;
        var_guard189_dn1 = 0.0;
        var_guard189_dn2 = 0.0;
        var_guard189_dn3 = 0.0;
        var_guard189_dn4 = 0.0;
        var_guard189_dn5 = 0.0;
        var_guard189_db0 = 0.0;
        var_guard189_db1 = 0.0;
        var_guard189_rv = 0.0;
        var_guard189_rdn0 = 0.0;
        var_guard189_rdn1 = 0.0;
        var_guard189_rdn2 = 0.0;
        var_guard189_rdn3 = 0.0;
        var_guard189_rdn4 = 0.0;
        var_guard189_rdn5 = 0.0;
        var_guard189_rdb0 = 0.0;
        var_guard189_rdb1 = 0.0;

        let (assign1890_e1820, assign1890_e1820_d_n0, assign1890_e1820_d_n1, assign1890_e1820_d_n2, assign1890_e1820_d_n3, assign1890_e1820_d_n4, assign1890_e1820_d_n5, assign1890_e1820_d_b0, assign1890_e1820_d_b1,) = {
    if (var_guard189 != 0.0) {
        let assign1890_e1818: f64 = (-var_vc2);
        (assign1890_e1818, (-var_vc2_dn0), (-var_vc2_dn1), (-var_vc2_dn2), (-var_vc2_dn3), (-var_vc2_dn4), (-var_vc2_dn5), (-var_vc2_db0), (-var_vc2_db1),)
    } else {
        (var_v1ci, var_v1ci_dn0, var_v1ci_dn1, var_v1ci_dn2, var_v1ci_dn3, var_v1ci_dn4, var_v1ci_dn5, var_v1ci_db0, var_v1ci_db1,)
    }
};
        var_v1ci = assign1890_e1820;
        var_v1ci_dn0 = assign1890_e1820_d_n0;
        var_v1ci_dn1 = assign1890_e1820_d_n1;
        var_v1ci_dn2 = assign1890_e1820_d_n2;
        var_v1ci_dn3 = assign1890_e1820_d_n3;
        var_v1ci_dn4 = assign1890_e1820_d_n4;
        var_v1ci_dn5 = assign1890_e1820_d_n5;
        var_v1ci_db0 = assign1890_e1820_d_b0;
        var_v1ci_db1 = assign1890_e1820_d_b1;
        var_v1ci_rv = 0.0;
        var_v1ci_rdn0 = 0.0;
        var_v1ci_rdn1 = 0.0;
        var_v1ci_rdn2 = 0.0;
        var_v1ci_rdn3 = 0.0;
        var_v1ci_rdn4 = 0.0;
        var_v1ci_rdn5 = 0.0;
        var_v1ci_rdb0 = 0.0;
        var_v1ci_rdb1 = 0.0;

        let (assign1900_e1825, assign1900_e1825_d_n0, assign1900_e1825_d_n1, assign1900_e1825_d_n2, assign1900_e1825_d_n3, assign1900_e1825_d_n4, assign1900_e1825_d_n5, assign1900_e1825_d_b0, assign1900_e1825_d_b1,) = {
    if (var_guard189 != 0.0) {
        let assign1900_e1823: f64 = (-var_vrb);
        (assign1900_e1823, (-var_vrb_dn0), (-var_vrb_dn1), (-var_vrb_dn2), (-var_vrb_dn3), (-var_vrb_dn4), (-var_vrb_dn5), (-var_vrb_db0), (-var_vrb_db1),)
    } else {
        (var_vrbi, var_vrbi_dn0, var_vrbi_dn1, var_vrbi_dn2, var_vrbi_dn3, var_vrbi_dn4, var_vrbi_dn5, var_vrbi_db0, var_vrbi_db1,)
    }
};
        var_vrbi = assign1900_e1825;
        var_vrbi_dn0 = assign1900_e1825_d_n0;
        var_vrbi_dn1 = assign1900_e1825_d_n1;
        var_vrbi_dn2 = assign1900_e1825_d_n2;
        var_vrbi_dn3 = assign1900_e1825_d_n3;
        var_vrbi_dn4 = assign1900_e1825_d_n4;
        var_vrbi_dn5 = assign1900_e1825_d_n5;
        var_vrbi_db0 = assign1900_e1825_d_b0;
        var_vrbi_db1 = assign1900_e1825_d_b1;
        var_vrbi_rv = 0.0;
        var_vrbi_rdn0 = 0.0;
        var_vrbi_rdn1 = 0.0;
        var_vrbi_rdn2 = 0.0;
        var_vrbi_rdn3 = 0.0;
        var_vrbi_rdn4 = 0.0;
        var_vrbi_rdn5 = 0.0;
        var_vrbi_rdb0 = 0.0;
        var_vrbi_rdb1 = 0.0;

        let (assign1920_e1836, assign1920_e1836_d_n0, assign1920_e1836_d_n1, assign1920_e1836_d_n2, assign1920_e1836_d_n3, assign1920_e1836_d_n4, assign1920_e1836_d_n5, assign1920_e1836_d_b0, assign1920_e1836_d_b1,) = {
    if (var_guard189 == 0.0) {
        let assign1920_e1834: f64 = (-var_vc1);
        (assign1920_e1834, (-var_vc1_dn0), (-var_vc1_dn1), (-var_vc1_dn2), (-var_vc1_dn3), (-var_vc1_dn4), (-var_vc1_dn5), (-var_vc1_db0), (-var_vc1_db1),)
    } else {
        (var_v1ci, var_v1ci_dn0, var_v1ci_dn1, var_v1ci_dn2, var_v1ci_dn3, var_v1ci_dn4, var_v1ci_dn5, var_v1ci_db0, var_v1ci_db1,)
    }
};
        var_v1ci = assign1920_e1836;
        var_v1ci_dn0 = assign1920_e1836_d_n0;
        var_v1ci_dn1 = assign1920_e1836_d_n1;
        var_v1ci_dn2 = assign1920_e1836_d_n2;
        var_v1ci_dn3 = assign1920_e1836_d_n3;
        var_v1ci_dn4 = assign1920_e1836_d_n4;
        var_v1ci_dn5 = assign1920_e1836_d_n5;
        var_v1ci_db0 = assign1920_e1836_d_b0;
        var_v1ci_db1 = assign1920_e1836_d_b1;
        var_v1ci_rv = 0.0;
        var_v1ci_rdn0 = 0.0;
        var_v1ci_rdn1 = 0.0;
        var_v1ci_rdn2 = 0.0;
        var_v1ci_rdn3 = 0.0;
        var_v1ci_rdn4 = 0.0;
        var_v1ci_rdn5 = 0.0;
        var_v1ci_rdb0 = 0.0;
        var_v1ci_rdb1 = 0.0;

        let (assign1930_e1841, assign1930_e1841_d_n0, assign1930_e1841_d_n1, assign1930_e1841_d_n2, assign1930_e1841_d_n3, assign1930_e1841_d_n4, assign1930_e1841_d_n5, assign1930_e1841_d_b0, assign1930_e1841_d_b1,) = {
    if (var_guard189 == 0.0) {
        (var_vrb, var_vrb_dn0, var_vrb_dn1, var_vrb_dn2, var_vrb_dn3, var_vrb_dn4, var_vrb_dn5, var_vrb_db0, var_vrb_db1,)
    } else {
        (var_vrbi, var_vrbi_dn0, var_vrbi_dn1, var_vrbi_dn2, var_vrbi_dn3, var_vrbi_dn4, var_vrbi_dn5, var_vrbi_db0, var_vrbi_db1,)
    }
};
        var_vrbi = assign1930_e1841;
        var_vrbi_dn0 = assign1930_e1841_d_n0;
        var_vrbi_dn1 = assign1930_e1841_d_n1;
        var_vrbi_dn2 = assign1930_e1841_d_n2;
        var_vrbi_dn3 = assign1930_e1841_d_n3;
        var_vrbi_dn4 = assign1930_e1841_d_n4;
        var_vrbi_dn5 = assign1930_e1841_d_n5;
        var_vrbi_db0 = assign1930_e1841_d_b0;
        var_vrbi_db1 = assign1930_e1841_d_b1;
        var_vrbi_rv = 0.0;
        var_vrbi_rdn0 = 0.0;
        var_vrbi_rdn1 = 0.0;
        var_vrbi_rdn2 = 0.0;
        var_vrbi_rdn3 = 0.0;
        var_vrbi_rdn4 = 0.0;
        var_vrbi_rdn5 = 0.0;
        var_vrbi_rdb0 = 0.0;
        var_vrbi_rdb1 = 0.0;

        let assign1940_e1844: f64 = if var_v1ci > var_vpoe { 1.0 } else { 0.0 };
        var_guard190 = assign1940_e1844;
        var_guard190_dn0 = 0.0;
        var_guard190_dn1 = 0.0;
        var_guard190_dn2 = 0.0;
        var_guard190_dn3 = 0.0;
        var_guard190_dn4 = 0.0;
        var_guard190_dn5 = 0.0;
        var_guard190_db0 = 0.0;
        var_guard190_db1 = 0.0;
        var_guard190_rv = 0.0;
        var_guard190_rdn0 = 0.0;
        var_guard190_rdn1 = 0.0;
        var_guard190_rdn2 = 0.0;
        var_guard190_rdn3 = 0.0;
        var_guard190_rdn4 = 0.0;
        var_guard190_rdn5 = 0.0;
        var_guard190_rdb0 = 0.0;
        var_guard190_rdb1 = 0.0;

        let (assign1950_e1860, assign1950_e1860_d_n0, assign1950_e1860_d_n1, assign1950_e1860_d_n2, assign1950_e1860_d_n3, assign1950_e1860_d_n4, assign1950_e1860_d_n5, assign1950_e1860_d_b0, assign1950_e1860_d_b1,) = {
    if (var_guard190 != 0.0) {
        let assign1950_e1851: f64 = (var_vpoe - var_v1ci);
        let assign1950_e1853: f64 = (assign1950_e1851 / var_nsteff);
        let assign1950_e1854: f64 = (assign1950_e1853).exp();
        let assign1950_e1855: f64 = (1.0 + assign1950_e1854);
        let assign1950_e1856: f64 = (assign1950_e1855).ln();
        let assign1950_e1857: f64 = (var_nsteff * assign1950_e1856);
        let assign1950_e1858: f64 = (var_vpoe - assign1950_e1857);
        (assign1950_e1858, (var_vpoe_dn0 - ((var_nsteff_dn0 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn0 - var_v1ci_dn0) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn0)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_dn1 - ((var_nsteff_dn1 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn1 - var_v1ci_dn1) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn1)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_dn2 - ((var_nsteff_dn2 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn2 - var_v1ci_dn2) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn2)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_dn3 - ((var_nsteff_dn3 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn3 - var_v1ci_dn3) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn3)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_dn4 - ((var_nsteff_dn4 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn4 - var_v1ci_dn4) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn4)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_dn5 - ((var_nsteff_dn5 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_dn5 - var_v1ci_dn5) * var_nsteff) - (assign1950_e1851 * var_nsteff_dn5)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_db0 - ((var_nsteff_db0 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_db0 - var_v1ci_db0) * var_nsteff) - (assign1950_e1851 * var_nsteff_db0)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))), (var_vpoe_db1 - ((var_nsteff_db1 * assign1950_e1856) + (var_nsteff * ((assign1950_e1854 * ((((var_vpoe_db1 - var_v1ci_db1) * var_nsteff) - (assign1950_e1851 * var_nsteff_db1)) / (var_nsteff * var_nsteff))) / assign1950_e1855)))),)
    } else {
        (var_v1cl, var_v1cl_dn0, var_v1cl_dn1, var_v1cl_dn2, var_v1cl_dn3, var_v1cl_dn4, var_v1cl_dn5, var_v1cl_db0, var_v1cl_db1,)
    }
};
        var_v1cl = assign1950_e1860;
        var_v1cl_dn0 = assign1950_e1860_d_n0;
        var_v1cl_dn1 = assign1950_e1860_d_n1;
        var_v1cl_dn2 = assign1950_e1860_d_n2;
        var_v1cl_dn3 = assign1950_e1860_d_n3;
        var_v1cl_dn4 = assign1950_e1860_d_n4;
        var_v1cl_dn5 = assign1950_e1860_d_n5;
        var_v1cl_db0 = assign1950_e1860_d_b0;
        var_v1cl_db1 = assign1950_e1860_d_b1;
        var_v1cl_rv = 0.0;
        var_v1cl_rdn0 = 0.0;
        var_v1cl_rdn1 = 0.0;
        var_v1cl_rdn2 = 0.0;
        var_v1cl_rdn3 = 0.0;
        var_v1cl_rdn4 = 0.0;
        var_v1cl_rdn5 = 0.0;
        var_v1cl_rdb0 = 0.0;
        var_v1cl_rdb1 = 0.0;

        let (assign1960_e1877, assign1960_e1877_d_n0, assign1960_e1877_d_n1, assign1960_e1877_d_n2, assign1960_e1877_d_n3, assign1960_e1877_d_n4, assign1960_e1877_d_n5, assign1960_e1877_d_b0, assign1960_e1877_d_b1,) = {
    if (var_guard190 == 0.0) {
        let assign1960_e1868: f64 = (var_v1ci - var_vpoe);
        let assign1960_e1870: f64 = (assign1960_e1868 / var_nsteff);
        let assign1960_e1871: f64 = (assign1960_e1870).exp();
        let assign1960_e1872: f64 = (1.0 + assign1960_e1871);
        let assign1960_e1873: f64 = (assign1960_e1872).ln();
        let assign1960_e1874: f64 = (var_nsteff * assign1960_e1873);
        let assign1960_e1875: f64 = (var_v1ci - assign1960_e1874);
        (assign1960_e1875, (var_v1ci_dn0 - ((var_nsteff_dn0 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn0 - var_vpoe_dn0) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn0)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_dn1 - ((var_nsteff_dn1 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn1 - var_vpoe_dn1) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn1)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_dn2 - ((var_nsteff_dn2 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn2 - var_vpoe_dn2) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn2)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_dn3 - ((var_nsteff_dn3 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn3 - var_vpoe_dn3) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn3)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_dn4 - ((var_nsteff_dn4 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn4 - var_vpoe_dn4) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn4)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_dn5 - ((var_nsteff_dn5 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_dn5 - var_vpoe_dn5) * var_nsteff) - (assign1960_e1868 * var_nsteff_dn5)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_db0 - ((var_nsteff_db0 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_db0 - var_vpoe_db0) * var_nsteff) - (assign1960_e1868 * var_nsteff_db0)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))), (var_v1ci_db1 - ((var_nsteff_db1 * assign1960_e1873) + (var_nsteff * ((assign1960_e1871 * ((((var_v1ci_db1 - var_vpoe_db1) * var_nsteff) - (assign1960_e1868 * var_nsteff_db1)) / (var_nsteff * var_nsteff))) / assign1960_e1872)))),)
    } else {
        (var_v1cl, var_v1cl_dn0, var_v1cl_dn1, var_v1cl_dn2, var_v1cl_dn3, var_v1cl_dn4, var_v1cl_dn5, var_v1cl_db0, var_v1cl_db1,)
    }
};
        var_v1cl = assign1960_e1877;
        var_v1cl_dn0 = assign1960_e1877_d_n0;
        var_v1cl_dn1 = assign1960_e1877_d_n1;
        var_v1cl_dn2 = assign1960_e1877_d_n2;
        var_v1cl_dn3 = assign1960_e1877_d_n3;
        var_v1cl_dn4 = assign1960_e1877_d_n4;
        var_v1cl_dn5 = assign1960_e1877_d_n5;
        var_v1cl_db0 = assign1960_e1877_d_b0;
        var_v1cl_db1 = assign1960_e1877_d_b1;
        var_v1cl_rv = 0.0;
        var_v1cl_rdn0 = 0.0;
        var_v1cl_rdn1 = 0.0;
        var_v1cl_rdn2 = 0.0;
        var_v1cl_rdn3 = 0.0;
        var_v1cl_rdn4 = 0.0;
        var_v1cl_rdn5 = 0.0;
        var_v1cl_rdb0 = 0.0;
        var_v1cl_rdb1 = 0.0;

        let assign1970_e1880: f64 = (-0.4);
        let assign1970_e1885: f64 = (var_vpoe - var_v1cl);
        let (assign1970_e1891, assign1970_e1891_d_n0, assign1970_e1891_d_n1, assign1970_e1891_d_n2, assign1970_e1891_d_n3, assign1970_e1891_d_n4, assign1970_e1891_d_n5, assign1970_e1891_d_b0, assign1970_e1891_d_b1,) = {
    if (var_vrbi < assign1970_e1885) {
        (var_vrbi, var_vrbi_dn0, var_vrbi_dn1, var_vrbi_dn2, var_vrbi_dn3, var_vrbi_dn4, var_vrbi_dn5, var_vrbi_db0, var_vrbi_db1,)
    } else {
        let assign1970_e1890: f64 = (var_vpoe - var_v1cl);
        (assign1970_e1890, (var_vpoe_dn0 - var_v1cl_dn0), (var_vpoe_dn1 - var_v1cl_dn1), (var_vpoe_dn2 - var_v1cl_dn2), (var_vpoe_dn3 - var_v1cl_dn3), (var_vpoe_dn4 - var_v1cl_dn4), (var_vpoe_dn5 - var_v1cl_dn5), (var_vpoe_db0 - var_v1cl_db0), (var_vpoe_db1 - var_v1cl_db1),)
    }
};
        let assign1970_e1892: f64 = (var_dp_i + assign1970_e1891);
        let assign1970_e1893: f64 = (assign1970_e1880 * assign1970_e1892);
        let assign1970_e1894: f64 = if var_v1cl < assign1970_e1893 { 1.0 } else { 0.0 };
        var_guard191 = assign1970_e1894;
        var_guard191_dn0 = 0.0;
        var_guard191_dn1 = 0.0;
        var_guard191_dn2 = 0.0;
        var_guard191_dn3 = 0.0;
        var_guard191_dn4 = 0.0;
        var_guard191_dn5 = 0.0;
        var_guard191_db0 = 0.0;
        var_guard191_db1 = 0.0;
        var_guard191_rv = 0.0;
        var_guard191_rdn0 = 0.0;
        var_guard191_rdn1 = 0.0;
        var_guard191_rdn2 = 0.0;
        var_guard191_rdn3 = 0.0;
        var_guard191_rdn4 = 0.0;
        var_guard191_rdn5 = 0.0;
        var_guard191_rdb0 = 0.0;
        var_guard191_rdb1 = 0.0;

        let (assign1980_e1914, assign1980_e1914_d_n0, assign1980_e1914_d_n1, assign1980_e1914_d_n2, assign1980_e1914_d_n3, assign1980_e1914_d_n4, assign1980_e1914_d_n5, assign1980_e1914_d_b0, assign1980_e1914_d_b1,) = {
    if ((p.p63 != 0.0) && (var_guard191 != 0.0)) {
        let assign1980_e1899: f64 = (-0.4);
        let assign1980_e1904: f64 = (var_vpoe - var_v1cl);
        let (assign1980_e1910, assign1980_e1910_d_n0, assign1980_e1910_d_n1, assign1980_e1910_d_n2, assign1980_e1910_d_n3, assign1980_e1910_d_n4, assign1980_e1910_d_n5, assign1980_e1910_d_b0, assign1980_e1910_d_b1,) = {
            if (var_vrbi < assign1980_e1904) {
                (var_vrbi, var_vrbi_dn0, var_vrbi_dn1, var_vrbi_dn2, var_vrbi_dn3, var_vrbi_dn4, var_vrbi_dn5, var_vrbi_db0, var_vrbi_db1,)
            } else {
                let assign1980_e1909: f64 = (var_vpoe - var_v1cl);
                (assign1980_e1909, (var_vpoe_dn0 - var_v1cl_dn0), (var_vpoe_dn1 - var_v1cl_dn1), (var_vpoe_dn2 - var_v1cl_dn2), (var_vpoe_dn3 - var_v1cl_dn3), (var_vpoe_dn4 - var_v1cl_dn4), (var_vpoe_dn5 - var_v1cl_dn5), (var_vpoe_db0 - var_v1cl_db0), (var_vpoe_db1 - var_v1cl_db1),)
            }
        };
        let assign1980_e1911: f64 = (var_dp_i + assign1980_e1910);
        let assign1980_e1912: f64 = (assign1980_e1899 * assign1980_e1911);
        (assign1980_e1912, (assign1980_e1899 * (var_dp_i_dn0 + assign1980_e1910_d_n0)), (assign1980_e1899 * (var_dp_i_dn1 + assign1980_e1910_d_n1)), (assign1980_e1899 * (var_dp_i_dn2 + assign1980_e1910_d_n2)), (assign1980_e1899 * (var_dp_i_dn3 + assign1980_e1910_d_n3)), (assign1980_e1899 * (var_dp_i_dn4 + assign1980_e1910_d_n4)), (assign1980_e1899 * (var_dp_i_dn5 + assign1980_e1910_d_n5)), (assign1980_e1899 * (var_dp_i_db0 + assign1980_e1910_d_b0)), (assign1980_e1899 * (var_dp_i_db1 + assign1980_e1910_d_b1)),)
    } else {
        (var_v1c, var_v1c_dn0, var_v1c_dn1, var_v1c_dn2, var_v1c_dn3, var_v1c_dn4, var_v1c_dn5, var_v1c_db0, var_v1c_db1,)
    }
};
        var_v1c = assign1980_e1914;
        var_v1c_dn0 = assign1980_e1914_d_n0;
        var_v1c_dn1 = assign1980_e1914_d_n1;
        var_v1c_dn2 = assign1980_e1914_d_n2;
        var_v1c_dn3 = assign1980_e1914_d_n3;
        var_v1c_dn4 = assign1980_e1914_d_n4;
        var_v1c_dn5 = assign1980_e1914_d_n5;
        var_v1c_db0 = assign1980_e1914_d_b0;
        var_v1c_db1 = assign1980_e1914_d_b1;
        var_v1c_rv = 0.0;
        var_v1c_rdn0 = 0.0;
        var_v1c_rdn1 = 0.0;
        var_v1c_rdn2 = 0.0;
        var_v1c_rdn3 = 0.0;
        var_v1c_rdn4 = 0.0;
        var_v1c_rdn5 = 0.0;
        var_v1c_rdb0 = 0.0;
        var_v1c_rdb1 = 0.0;

        let (assign1990_e1921, assign1990_e1921_d_n0, assign1990_e1921_d_n1, assign1990_e1921_d_n2, assign1990_e1921_d_n3, assign1990_e1921_d_n4, assign1990_e1921_d_n5, assign1990_e1921_d_b0, assign1990_e1921_d_b1,) = {
    if ((p.p63 != 0.0) && (var_guard191 == 0.0)) {
        (var_v1cl, var_v1cl_dn0, var_v1cl_dn1, var_v1cl_dn2, var_v1cl_dn3, var_v1cl_dn4, var_v1cl_dn5, var_v1cl_db0, var_v1cl_db1,)
    } else {
        (var_v1c, var_v1c_dn0, var_v1c_dn1, var_v1c_dn2, var_v1c_dn3, var_v1c_dn4, var_v1c_dn5, var_v1c_db0, var_v1c_db1,)
    }
};
        var_v1c = assign1990_e1921;
        var_v1c_dn0 = assign1990_e1921_d_n0;
        var_v1c_dn1 = assign1990_e1921_d_n1;
        var_v1c_dn2 = assign1990_e1921_d_n2;
        var_v1c_dn3 = assign1990_e1921_d_n3;
        var_v1c_dn4 = assign1990_e1921_d_n4;
        var_v1c_dn5 = assign1990_e1921_d_n5;
        var_v1c_db0 = assign1990_e1921_d_b0;
        var_v1c_db1 = assign1990_e1921_d_b1;
        var_v1c_rv = 0.0;
        var_v1c_rdn0 = 0.0;
        var_v1c_rdn1 = 0.0;
        var_v1c_rdn2 = 0.0;
        var_v1c_rdn3 = 0.0;
        var_v1c_rdn4 = 0.0;
        var_v1c_rdn5 = 0.0;
        var_v1c_rdb0 = 0.0;
        var_v1c_rdb1 = 0.0;

        let assign2000_e1924: f64 = (-0.4);
        let assign2000_e1926: f64 = (assign2000_e1924 * var_dp_i);
        let assign2000_e1927: f64 = if var_v1cl < assign2000_e1926 { 1.0 } else { 0.0 };
        var_guard192 = assign2000_e1927;
        var_guard192_dn0 = 0.0;
        var_guard192_dn1 = 0.0;
        var_guard192_dn2 = 0.0;
        var_guard192_dn3 = 0.0;
        var_guard192_dn4 = 0.0;
        var_guard192_dn5 = 0.0;
        var_guard192_db0 = 0.0;
        var_guard192_db1 = 0.0;
        var_guard192_rv = 0.0;
        var_guard192_rdn0 = 0.0;
        var_guard192_rdn1 = 0.0;
        var_guard192_rdn2 = 0.0;
        var_guard192_rdn3 = 0.0;
        var_guard192_rdn4 = 0.0;
        var_guard192_rdn5 = 0.0;
        var_guard192_rdb0 = 0.0;
        var_guard192_rdb1 = 0.0;

        let (assign2010_e1937, assign2010_e1937_d_n0, assign2010_e1937_d_n1, assign2010_e1937_d_n2, assign2010_e1937_d_n3, assign2010_e1937_d_n4, assign2010_e1937_d_n5, assign2010_e1937_d_b0, assign2010_e1937_d_b1,) = {
    if ((p.p63 == 0.0) && (var_guard192 != 0.0)) {
        let assign2010_e1933: f64 = (-0.4);
        let assign2010_e1935: f64 = (assign2010_e1933 * var_dp_i);
        (assign2010_e1935, (assign2010_e1933 * var_dp_i_dn0), (assign2010_e1933 * var_dp_i_dn1), (assign2010_e1933 * var_dp_i_dn2), (assign2010_e1933 * var_dp_i_dn3), (assign2010_e1933 * var_dp_i_dn4), (assign2010_e1933 * var_dp_i_dn5), (assign2010_e1933 * var_dp_i_db0), (assign2010_e1933 * var_dp_i_db1),)
    } else {
        (var_v1c, var_v1c_dn0, var_v1c_dn1, var_v1c_dn2, var_v1c_dn3, var_v1c_dn4, var_v1c_dn5, var_v1c_db0, var_v1c_db1,)
    }
};
        var_v1c = assign2010_e1937;
        var_v1c_dn0 = assign2010_e1937_d_n0;
        var_v1c_dn1 = assign2010_e1937_d_n1;
        var_v1c_dn2 = assign2010_e1937_d_n2;
        var_v1c_dn3 = assign2010_e1937_d_n3;
        var_v1c_dn4 = assign2010_e1937_d_n4;
        var_v1c_dn5 = assign2010_e1937_d_n5;
        var_v1c_db0 = assign2010_e1937_d_b0;
        var_v1c_db1 = assign2010_e1937_d_b1;
        var_v1c_rv = 0.0;
        var_v1c_rdn0 = 0.0;
        var_v1c_rdn1 = 0.0;
        var_v1c_rdn2 = 0.0;
        var_v1c_rdn3 = 0.0;
        var_v1c_rdn4 = 0.0;
        var_v1c_rdn5 = 0.0;
        var_v1c_rdb0 = 0.0;
        var_v1c_rdb1 = 0.0;

        let (assign2020_e1945, assign2020_e1945_d_n0, assign2020_e1945_d_n1, assign2020_e1945_d_n2, assign2020_e1945_d_n3, assign2020_e1945_d_n4, assign2020_e1945_d_n5, assign2020_e1945_d_b0, assign2020_e1945_d_b1,) = {
    if ((p.p63 == 0.0) && (var_guard192 == 0.0)) {
        (var_v1cl, var_v1cl_dn0, var_v1cl_dn1, var_v1cl_dn2, var_v1cl_dn3, var_v1cl_dn4, var_v1cl_dn5, var_v1cl_db0, var_v1cl_db1,)
    } else {
        (var_v1c, var_v1c_dn0, var_v1c_dn1, var_v1c_dn2, var_v1c_dn3, var_v1c_dn4, var_v1c_dn5, var_v1c_db0, var_v1c_db1,)
    }
};
        var_v1c = assign2020_e1945;
        var_v1c_dn0 = assign2020_e1945_d_n0;
        var_v1c_dn1 = assign2020_e1945_d_n1;
        var_v1c_dn2 = assign2020_e1945_d_n2;
        var_v1c_dn3 = assign2020_e1945_d_n3;
        var_v1c_dn4 = assign2020_e1945_d_n4;
        var_v1c_dn5 = assign2020_e1945_d_n5;
        var_v1c_db0 = assign2020_e1945_d_b0;
        var_v1c_db1 = assign2020_e1945_d_b1;
        var_v1c_rv = 0.0;
        var_v1c_rdn0 = 0.0;
        var_v1c_rdn1 = 0.0;
        var_v1c_rdn2 = 0.0;
        var_v1c_rdn3 = 0.0;
        var_v1c_rdn4 = 0.0;
        var_v1c_rdn5 = 0.0;
        var_v1c_rdb0 = 0.0;
        var_v1c_rdb1 = 0.0;

        let assign2030_e1949: f64 = (2.0 * var_v1c);
        let assign2030_e1950: f64 = (var_dp_i + assign2030_e1949);
        var_pe = assign2030_e1950;
        var_pe_dn0 = (var_dp_i_dn0 + (2.0 * var_v1c_dn0));
        var_pe_dn1 = (var_dp_i_dn1 + (2.0 * var_v1c_dn1));
        var_pe_dn2 = (var_dp_i_dn2 + (2.0 * var_v1c_dn2));
        var_pe_dn3 = (var_dp_i_dn3 + (2.0 * var_v1c_dn3));
        var_pe_dn4 = (var_dp_i_dn4 + (2.0 * var_v1c_dn4));
        var_pe_dn5 = (var_dp_i_dn5 + (2.0 * var_v1c_dn5));
        var_pe_db0 = (var_dp_i_db0 + (2.0 * var_v1c_db0));
        var_pe_db1 = (var_dp_i_db1 + (2.0 * var_v1c_db1));
        var_pe_rv = 0.0;
        var_pe_rdn0 = 0.0;
        var_pe_rdn1 = 0.0;
        var_pe_rdn2 = 0.0;
        var_pe_rdn3 = 0.0;
        var_pe_rdn4 = 0.0;
        var_pe_rdn5 = 0.0;
        var_pe_rdb0 = 0.0;
        var_pe_rdb1 = 0.0;

        let assign2040_e1953: f64 = if var_iecrit > 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign2040_e1953;
        var_guard193_dn0 = 0.0;
        var_guard193_dn1 = 0.0;
        var_guard193_dn2 = 0.0;
        var_guard193_dn3 = 0.0;
        var_guard193_dn4 = 0.0;
        var_guard193_dn5 = 0.0;
        var_guard193_db0 = 0.0;
        var_guard193_db1 = 0.0;
        var_guard193_rv = 0.0;
        var_guard193_rdn0 = 0.0;
        var_guard193_rdn1 = 0.0;
        var_guard193_rdn2 = 0.0;
        var_guard193_rdn3 = 0.0;
        var_guard193_rdn4 = 0.0;
        var_guard193_rdn5 = 0.0;
        var_guard193_rdb0 = 0.0;
        var_guard193_rdb1 = 0.0;

        let (assign2060_e1974, assign2060_e1974_d_n0, assign2060_e1974_d_n1, assign2060_e1974_d_n2, assign2060_e1974_d_n3, assign2060_e1974_d_n4, assign2060_e1974_d_n5, assign2060_e1974_d_b0, assign2060_e1974_d_b1,) = {
    if (var_guard193 != 0.0) {
        let assign2060_e1966: f64 = (-1.0);
        let assign2060_e1969: f64 = (3.0 * var_dfsq);
        let assign2060_e1971: f64 = (assign2060_e1969 * var_pe);
        let assign2060_e1972: f64 = (assign2060_e1966 + assign2060_e1971);
        (assign2060_e1972, (((3.0 * var_dfsq_dn0) * var_pe) + (assign2060_e1969 * var_pe_dn0)), (((3.0 * var_dfsq_dn1) * var_pe) + (assign2060_e1969 * var_pe_dn1)), (((3.0 * var_dfsq_dn2) * var_pe) + (assign2060_e1969 * var_pe_dn2)), (((3.0 * var_dfsq_dn3) * var_pe) + (assign2060_e1969 * var_pe_dn3)), (((3.0 * var_dfsq_dn4) * var_pe) + (assign2060_e1969 * var_pe_dn4)), (((3.0 * var_dfsq_dn5) * var_pe) + (assign2060_e1969 * var_pe_dn5)), (((3.0 * var_dfsq_db0) * var_pe) + (assign2060_e1969 * var_pe_db0)), (((3.0 * var_dfsq_db1) * var_pe) + (assign2060_e1969 * var_pe_db1)),)
    } else {
        (p.p3, var_a1_dn0, var_a1_dn1, var_a1_dn2, var_a1_dn3, var_a1_dn4, var_a1_dn5, var_a1_db0, var_a1_db1,)
    }
};
        var_a1 = assign2060_e1974;
        var_a1_dn0 = assign2060_e1974_d_n0;
        var_a1_dn1 = assign2060_e1974_d_n1;
        var_a1_dn2 = assign2060_e1974_d_n2;
        var_a1_dn3 = assign2060_e1974_d_n3;
        var_a1_dn4 = assign2060_e1974_d_n4;
        var_a1_dn5 = assign2060_e1974_d_n5;
        var_a1_db0 = assign2060_e1974_d_b0;
        var_a1_db1 = assign2060_e1974_d_b1;
        var_a1_rv = 0.0;
        var_a1_rdn0 = 0.0;
        var_a1_rdn1 = 0.0;
        var_a1_rdn2 = 0.0;
        var_a1_rdn3 = 0.0;
        var_a1_rdn4 = 0.0;
        var_a1_rdn5 = 0.0;
        var_a1_rdb0 = 0.0;
        var_a1_rdb1 = 0.0;

        let (assign2070_e1986, assign2070_e1986_d_n0, assign2070_e1986_d_n1, assign2070_e1986_d_n2, assign2070_e1986_d_n3, assign2070_e1986_d_n4, assign2070_e1986_d_n5, assign2070_e1986_d_b0, assign2070_e1986_d_b1,) = {
    if (var_guard193 != 0.0) {
        let assign2070_e1979: f64 = (9.0 / 4.0);
        let assign2070_e1982: f64 = (var_pe / var_lde);
        let assign2070_e1983: f64 = (assign2070_e1979 + assign2070_e1982);
        let assign2070_e1984: f64 = (var_dfsq * assign2070_e1983);
        (assign2070_e1984, ((var_dfsq_dn0 * assign2070_e1983) + (var_dfsq * (((var_pe_dn0 * var_lde) - (var_pe * var_lde_dn0)) / (var_lde * var_lde)))), ((var_dfsq_dn1 * assign2070_e1983) + (var_dfsq * (((var_pe_dn1 * var_lde) - (var_pe * var_lde_dn1)) / (var_lde * var_lde)))), ((var_dfsq_dn2 * assign2070_e1983) + (var_dfsq * (((var_pe_dn2 * var_lde) - (var_pe * var_lde_dn2)) / (var_lde * var_lde)))), ((var_dfsq_dn3 * assign2070_e1983) + (var_dfsq * (((var_pe_dn3 * var_lde) - (var_pe * var_lde_dn3)) / (var_lde * var_lde)))), ((var_dfsq_dn4 * assign2070_e1983) + (var_dfsq * (((var_pe_dn4 * var_lde) - (var_pe * var_lde_dn4)) / (var_lde * var_lde)))), ((var_dfsq_dn5 * assign2070_e1983) + (var_dfsq * (((var_pe_dn5 * var_lde) - (var_pe * var_lde_dn5)) / (var_lde * var_lde)))), ((var_dfsq_db0 * assign2070_e1983) + (var_dfsq * (((var_pe_db0 * var_lde) - (var_pe * var_lde_db0)) / (var_lde * var_lde)))), ((var_dfsq_db1 * assign2070_e1983) + (var_dfsq * (((var_pe_db1 * var_lde) - (var_pe * var_lde_db1)) / (var_lde * var_lde)))),)
    } else {
        (p.p6, var_a2_dn0, var_a2_dn1, var_a2_dn2, var_a2_dn3, var_a2_dn4, var_a2_dn5, var_a2_db0, var_a2_db1,)
    }
};
        var_a2 = assign2070_e1986;
        var_a2_dn0 = assign2070_e1986_d_n0;
        var_a2_dn1 = assign2070_e1986_d_n1;
        var_a2_dn2 = assign2070_e1986_d_n2;
        var_a2_dn3 = assign2070_e1986_d_n3;
        var_a2_dn4 = assign2070_e1986_d_n4;
        var_a2_dn5 = assign2070_e1986_d_n5;
        var_a2_db0 = assign2070_e1986_d_b0;
        var_a2_db1 = assign2070_e1986_d_b1;
        var_a2_rv = 0.0;
        var_a2_rdn0 = 0.0;
        var_a2_rdn1 = 0.0;
        var_a2_rdn2 = 0.0;
        var_a2_rdn3 = 0.0;
        var_a2_rdn4 = 0.0;
        var_a2_rdn5 = 0.0;
        var_a2_rdb0 = 0.0;
        var_a2_rdb1 = 0.0;

        let assign3800_e3655: f64 = if var_cj1 > 0.0 { 1.0 } else { 0.0 };
        var_guard249 = assign3800_e3655;
        var_guard249_dn0 = 0.0;
        var_guard249_dn1 = 0.0;
        var_guard249_dn2 = 0.0;
        var_guard249_dn3 = 0.0;
        var_guard249_dn4 = 0.0;
        var_guard249_dn5 = 0.0;
        var_guard249_db0 = 0.0;
        var_guard249_db1 = 0.0;
        var_guard249_rv = 0.0;
        var_guard249_rdn0 = 0.0;
        var_guard249_rdn1 = 0.0;
        var_guard249_rdn2 = 0.0;
        var_guard249_rdn3 = 0.0;
        var_guard249_rdn4 = 0.0;
        var_guard249_rdn5 = 0.0;
        var_guard249_rdb0 = 0.0;
        var_guard249_rdb1 = 0.0;

        *var_a1_slot = var_a1;
        *var_a1_db0_slot = var_a1_db0;
        *var_a1_db1_slot = var_a1_db1;
        *var_a1_dn0_slot = var_a1_dn0;
        *var_a1_dn1_slot = var_a1_dn1;
        *var_a1_dn2_slot = var_a1_dn2;
        *var_a1_dn3_slot = var_a1_dn3;
        *var_a1_dn4_slot = var_a1_dn4;
        *var_a1_dn5_slot = var_a1_dn5;
        *var_a1_rdb0_slot = var_a1_rdb0;
        *var_a1_rdb1_slot = var_a1_rdb1;
        *var_a1_rdn0_slot = var_a1_rdn0;
        *var_a1_rdn1_slot = var_a1_rdn1;
        *var_a1_rdn2_slot = var_a1_rdn2;
        *var_a1_rdn3_slot = var_a1_rdn3;
        *var_a1_rdn4_slot = var_a1_rdn4;
        *var_a1_rdn5_slot = var_a1_rdn5;
        *var_a1_rv_slot = var_a1_rv;
        *var_a2_slot = var_a2;
        *var_a2_db0_slot = var_a2_db0;
        *var_a2_db1_slot = var_a2_db1;
        *var_a2_dn0_slot = var_a2_dn0;
        *var_a2_dn1_slot = var_a2_dn1;
        *var_a2_dn2_slot = var_a2_dn2;
        *var_a2_dn3_slot = var_a2_dn3;
        *var_a2_dn4_slot = var_a2_dn4;
        *var_a2_dn5_slot = var_a2_dn5;
        *var_a2_rdb0_slot = var_a2_rdb0;
        *var_a2_rdb1_slot = var_a2_rdb1;
        *var_a2_rdn0_slot = var_a2_rdn0;
        *var_a2_rdn1_slot = var_a2_rdn1;
        *var_a2_rdn2_slot = var_a2_rdn2;
        *var_a2_rdn3_slot = var_a2_rdn3;
        *var_a2_rdn4_slot = var_a2_rdn4;
        *var_a2_rdn5_slot = var_a2_rdn5;
        *var_a2_rv_slot = var_a2_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_db0_slot = var_guard189_db0;
        *var_guard189_db1_slot = var_guard189_db1;
        *var_guard189_dn0_slot = var_guard189_dn0;
        *var_guard189_dn1_slot = var_guard189_dn1;
        *var_guard189_dn2_slot = var_guard189_dn2;
        *var_guard189_dn3_slot = var_guard189_dn3;
        *var_guard189_dn4_slot = var_guard189_dn4;
        *var_guard189_dn5_slot = var_guard189_dn5;
        *var_guard189_rdb0_slot = var_guard189_rdb0;
        *var_guard189_rdb1_slot = var_guard189_rdb1;
        *var_guard189_rdn0_slot = var_guard189_rdn0;
        *var_guard189_rdn1_slot = var_guard189_rdn1;
        *var_guard189_rdn2_slot = var_guard189_rdn2;
        *var_guard189_rdn3_slot = var_guard189_rdn3;
        *var_guard189_rdn4_slot = var_guard189_rdn4;
        *var_guard189_rdn5_slot = var_guard189_rdn5;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_db0_slot = var_guard190_db0;
        *var_guard190_db1_slot = var_guard190_db1;
        *var_guard190_dn0_slot = var_guard190_dn0;
        *var_guard190_dn1_slot = var_guard190_dn1;
        *var_guard190_dn2_slot = var_guard190_dn2;
        *var_guard190_dn3_slot = var_guard190_dn3;
        *var_guard190_dn4_slot = var_guard190_dn4;
        *var_guard190_dn5_slot = var_guard190_dn5;
        *var_guard190_rdb0_slot = var_guard190_rdb0;
        *var_guard190_rdb1_slot = var_guard190_rdb1;
        *var_guard190_rdn0_slot = var_guard190_rdn0;
        *var_guard190_rdn1_slot = var_guard190_rdn1;
        *var_guard190_rdn2_slot = var_guard190_rdn2;
        *var_guard190_rdn3_slot = var_guard190_rdn3;
        *var_guard190_rdn4_slot = var_guard190_rdn4;
        *var_guard190_rdn5_slot = var_guard190_rdn5;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_db0_slot = var_guard191_db0;
        *var_guard191_db1_slot = var_guard191_db1;
        *var_guard191_dn0_slot = var_guard191_dn0;
        *var_guard191_dn1_slot = var_guard191_dn1;
        *var_guard191_dn2_slot = var_guard191_dn2;
        *var_guard191_dn3_slot = var_guard191_dn3;
        *var_guard191_dn4_slot = var_guard191_dn4;
        *var_guard191_dn5_slot = var_guard191_dn5;
        *var_guard191_rdb0_slot = var_guard191_rdb0;
        *var_guard191_rdb1_slot = var_guard191_rdb1;
        *var_guard191_rdn0_slot = var_guard191_rdn0;
        *var_guard191_rdn1_slot = var_guard191_rdn1;
        *var_guard191_rdn2_slot = var_guard191_rdn2;
        *var_guard191_rdn3_slot = var_guard191_rdn3;
        *var_guard191_rdn4_slot = var_guard191_rdn4;
        *var_guard191_rdn5_slot = var_guard191_rdn5;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_db0_slot = var_guard192_db0;
        *var_guard192_db1_slot = var_guard192_db1;
        *var_guard192_dn0_slot = var_guard192_dn0;
        *var_guard192_dn1_slot = var_guard192_dn1;
        *var_guard192_dn2_slot = var_guard192_dn2;
        *var_guard192_dn3_slot = var_guard192_dn3;
        *var_guard192_dn4_slot = var_guard192_dn4;
        *var_guard192_dn5_slot = var_guard192_dn5;
        *var_guard192_rdb0_slot = var_guard192_rdb0;
        *var_guard192_rdb1_slot = var_guard192_rdb1;
        *var_guard192_rdn0_slot = var_guard192_rdn0;
        *var_guard192_rdn1_slot = var_guard192_rdn1;
        *var_guard192_rdn2_slot = var_guard192_rdn2;
        *var_guard192_rdn3_slot = var_guard192_rdn3;
        *var_guard192_rdn4_slot = var_guard192_rdn4;
        *var_guard192_rdn5_slot = var_guard192_rdn5;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_db0_slot = var_guard193_db0;
        *var_guard193_db1_slot = var_guard193_db1;
        *var_guard193_dn0_slot = var_guard193_dn0;
        *var_guard193_dn1_slot = var_guard193_dn1;
        *var_guard193_dn2_slot = var_guard193_dn2;
        *var_guard193_dn3_slot = var_guard193_dn3;
        *var_guard193_dn4_slot = var_guard193_dn4;
        *var_guard193_dn5_slot = var_guard193_dn5;
        *var_guard193_rdb0_slot = var_guard193_rdb0;
        *var_guard193_rdb1_slot = var_guard193_rdb1;
        *var_guard193_rdn0_slot = var_guard193_rdn0;
        *var_guard193_rdn1_slot = var_guard193_rdn1;
        *var_guard193_rdn2_slot = var_guard193_rdn2;
        *var_guard193_rdn3_slot = var_guard193_rdn3;
        *var_guard193_rdn4_slot = var_guard193_rdn4;
        *var_guard193_rdn5_slot = var_guard193_rdn5;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_guard249_slot = var_guard249;
        *var_guard249_db0_slot = var_guard249_db0;
        *var_guard249_db1_slot = var_guard249_db1;
        *var_guard249_dn0_slot = var_guard249_dn0;
        *var_guard249_dn1_slot = var_guard249_dn1;
        *var_guard249_dn2_slot = var_guard249_dn2;
        *var_guard249_dn3_slot = var_guard249_dn3;
        *var_guard249_dn4_slot = var_guard249_dn4;
        *var_guard249_dn5_slot = var_guard249_dn5;
        *var_guard249_rdb0_slot = var_guard249_rdb0;
        *var_guard249_rdb1_slot = var_guard249_rdb1;
        *var_guard249_rdn0_slot = var_guard249_rdn0;
        *var_guard249_rdn1_slot = var_guard249_rdn1;
        *var_guard249_rdn2_slot = var_guard249_rdn2;
        *var_guard249_rdn3_slot = var_guard249_rdn3;
        *var_guard249_rdn4_slot = var_guard249_rdn4;
        *var_guard249_rdn5_slot = var_guard249_rdn5;
        *var_guard249_rv_slot = var_guard249_rv;
        *var_pe_slot = var_pe;
        *var_pe_db0_slot = var_pe_db0;
        *var_pe_db1_slot = var_pe_db1;
        *var_pe_dn0_slot = var_pe_dn0;
        *var_pe_dn1_slot = var_pe_dn1;
        *var_pe_dn2_slot = var_pe_dn2;
        *var_pe_dn3_slot = var_pe_dn3;
        *var_pe_dn4_slot = var_pe_dn4;
        *var_pe_dn5_slot = var_pe_dn5;
        *var_pe_rdb0_slot = var_pe_rdb0;
        *var_pe_rdb1_slot = var_pe_rdb1;
        *var_pe_rdn0_slot = var_pe_rdn0;
        *var_pe_rdn1_slot = var_pe_rdn1;
        *var_pe_rdn2_slot = var_pe_rdn2;
        *var_pe_rdn3_slot = var_pe_rdn3;
        *var_pe_rdn4_slot = var_pe_rdn4;
        *var_pe_rdn5_slot = var_pe_rdn5;
        *var_pe_rv_slot = var_pe_rv;
        *var_v1c_slot = var_v1c;
        *var_v1c_db0_slot = var_v1c_db0;
        *var_v1c_db1_slot = var_v1c_db1;
        *var_v1c_dn0_slot = var_v1c_dn0;
        *var_v1c_dn1_slot = var_v1c_dn1;
        *var_v1c_dn2_slot = var_v1c_dn2;
        *var_v1c_dn3_slot = var_v1c_dn3;
        *var_v1c_dn4_slot = var_v1c_dn4;
        *var_v1c_dn5_slot = var_v1c_dn5;
        *var_v1c_rdb0_slot = var_v1c_rdb0;
        *var_v1c_rdb1_slot = var_v1c_rdb1;
        *var_v1c_rdn0_slot = var_v1c_rdn0;
        *var_v1c_rdn1_slot = var_v1c_rdn1;
        *var_v1c_rdn2_slot = var_v1c_rdn2;
        *var_v1c_rdn3_slot = var_v1c_rdn3;
        *var_v1c_rdn4_slot = var_v1c_rdn4;
        *var_v1c_rdn5_slot = var_v1c_rdn5;
        *var_v1c_rv_slot = var_v1c_rv;
        *var_v1ci_slot = var_v1ci;
        *var_v1ci_db0_slot = var_v1ci_db0;
        *var_v1ci_db1_slot = var_v1ci_db1;
        *var_v1ci_dn0_slot = var_v1ci_dn0;
        *var_v1ci_dn1_slot = var_v1ci_dn1;
        *var_v1ci_dn2_slot = var_v1ci_dn2;
        *var_v1ci_dn3_slot = var_v1ci_dn3;
        *var_v1ci_dn4_slot = var_v1ci_dn4;
        *var_v1ci_dn5_slot = var_v1ci_dn5;
        *var_v1ci_rdb0_slot = var_v1ci_rdb0;
        *var_v1ci_rdb1_slot = var_v1ci_rdb1;
        *var_v1ci_rdn0_slot = var_v1ci_rdn0;
        *var_v1ci_rdn1_slot = var_v1ci_rdn1;
        *var_v1ci_rdn2_slot = var_v1ci_rdn2;
        *var_v1ci_rdn3_slot = var_v1ci_rdn3;
        *var_v1ci_rdn4_slot = var_v1ci_rdn4;
        *var_v1ci_rdn5_slot = var_v1ci_rdn5;
        *var_v1ci_rv_slot = var_v1ci_rv;
        *var_v1cl_slot = var_v1cl;
        *var_v1cl_db0_slot = var_v1cl_db0;
        *var_v1cl_db1_slot = var_v1cl_db1;
        *var_v1cl_dn0_slot = var_v1cl_dn0;
        *var_v1cl_dn1_slot = var_v1cl_dn1;
        *var_v1cl_dn2_slot = var_v1cl_dn2;
        *var_v1cl_dn3_slot = var_v1cl_dn3;
        *var_v1cl_dn4_slot = var_v1cl_dn4;
        *var_v1cl_dn5_slot = var_v1cl_dn5;
        *var_v1cl_rdb0_slot = var_v1cl_rdb0;
        *var_v1cl_rdb1_slot = var_v1cl_rdb1;
        *var_v1cl_rdn0_slot = var_v1cl_rdn0;
        *var_v1cl_rdn1_slot = var_v1cl_rdn1;
        *var_v1cl_rdn2_slot = var_v1cl_rdn2;
        *var_v1cl_rdn3_slot = var_v1cl_rdn3;
        *var_v1cl_rdn4_slot = var_v1cl_rdn4;
        *var_v1cl_rdn5_slot = var_v1cl_rdn5;
        *var_v1cl_rv_slot = var_v1cl_rv;
        *var_vrbi_slot = var_vrbi;
        *var_vrbi_db0_slot = var_vrbi_db0;
        *var_vrbi_db1_slot = var_vrbi_db1;
        *var_vrbi_dn0_slot = var_vrbi_dn0;
        *var_vrbi_dn1_slot = var_vrbi_dn1;
        *var_vrbi_dn2_slot = var_vrbi_dn2;
        *var_vrbi_dn3_slot = var_vrbi_dn3;
        *var_vrbi_dn4_slot = var_vrbi_dn4;
        *var_vrbi_dn5_slot = var_vrbi_dn5;
        *var_vrbi_rdb0_slot = var_vrbi_rdb0;
        *var_vrbi_rdb1_slot = var_vrbi_rdb1;
        *var_vrbi_rdn0_slot = var_vrbi_rdn0;
        *var_vrbi_rdn1_slot = var_vrbi_rdn1;
        *var_vrbi_rdn2_slot = var_vrbi_rdn2;
        *var_vrbi_rdn3_slot = var_vrbi_rdn3;
        *var_vrbi_rdn4_slot = var_vrbi_rdn4;
        *var_vrbi_rdn5_slot = var_vrbi_rdn5;
        *var_vrbi_rv_slot = var_vrbi_rv;
    }

    pub(super) fn stamp_reactive_block_7(
        p: &Parameters,
        var_a1_um2: f64,
        var_a1_um2_db0: f64,
        var_a1_um2_db1: f64,
        var_a1_um2_dn0: f64,
        var_a1_um2_dn1: f64,
        var_a1_um2_dn2: f64,
        var_a1_um2_dn3: f64,
        var_a1_um2_dn4: f64,
        var_a1_um2_dn5: f64,
        var_cja_t: f64,
        var_cja_t_db0: f64,
        var_cja_t_db1: f64,
        var_cja_t_dn0: f64,
        var_cja_t_dn1: f64,
        var_cja_t_dn2: f64,
        var_cja_t_dn3: f64,
        var_cja_t_dn4: f64,
        var_cja_t_dn5: f64,
        var_cjp_t: f64,
        var_cjp_t_db0: f64,
        var_cjp_t_db1: f64,
        var_cjp_t_dn0: f64,
        var_cjp_t_dn1: f64,
        var_cjp_t_dn2: f64,
        var_cjp_t_dn3: f64,
        var_cjp_t_dn4: f64,
        var_cjp_t_dn5: f64,
        var_guard249: f64,
        var_p1_um: f64,
        var_p1_um_db0: f64,
        var_p1_um_db1: f64,
        var_p1_um_dn0: f64,
        var_p1_um_dn1: f64,
        var_p1_um_dn2: f64,
        var_p1_um_dn3: f64,
        var_p1_um_dn4: f64,
        var_p1_um_dn5: f64,
        var_pa_t: f64,
        var_pa_t_db0: f64,
        var_pa_t_db1: f64,
        var_pa_t_dn0: f64,
        var_pa_t_dn1: f64,
        var_pa_t_dn2: f64,
        var_pa_t_dn3: f64,
        var_pa_t_dn4: f64,
        var_pa_t_dn5: f64,
        var_vc1: f64,
        var_vc1_db0: f64,
        var_vc1_db1: f64,
        var_vc1_dn0: f64,
        var_vc1_dn1: f64,
        var_vc1_dn2: f64,
        var_vc1_dn3: f64,
        var_vc1_dn4: f64,
        var_vc1_dn5: f64,
        var_vpo: f64,
        var_vpo_db0: f64,
        var_vpo_db1: f64,
        var_vpo_dn0: f64,
        var_vpo_dn1: f64,
        var_vpo_dn2: f64,
        var_vpo_dn3: f64,
        var_vpo_dn4: f64,
        var_vpo_dn5: f64,
        var_acja_slot: &mut f64,
        var_acja_db0_slot: &mut f64,
        var_acja_db1_slot: &mut f64,
        var_acja_dn0_slot: &mut f64,
        var_acja_dn1_slot: &mut f64,
        var_acja_dn2_slot: &mut f64,
        var_acja_dn3_slot: &mut f64,
        var_acja_dn4_slot: &mut f64,
        var_acja_dn5_slot: &mut f64,
        var_acja_rdb0_slot: &mut f64,
        var_acja_rdb1_slot: &mut f64,
        var_acja_rdn0_slot: &mut f64,
        var_acja_rdn1_slot: &mut f64,
        var_acja_rdn2_slot: &mut f64,
        var_acja_rdn3_slot: &mut f64,
        var_acja_rdn4_slot: &mut f64,
        var_acja_rdn5_slot: &mut f64,
        var_acja_rv_slot: &mut f64,
        var_arga_slot: &mut f64,
        var_arga_db0_slot: &mut f64,
        var_arga_db1_slot: &mut f64,
        var_arga_dn0_slot: &mut f64,
        var_arga_dn1_slot: &mut f64,
        var_arga_dn2_slot: &mut f64,
        var_arga_dn3_slot: &mut f64,
        var_arga_dn4_slot: &mut f64,
        var_arga_dn5_slot: &mut f64,
        var_arga_rdb0_slot: &mut f64,
        var_arga_rdb1_slot: &mut f64,
        var_arga_rdn0_slot: &mut f64,
        var_arga_rdn1_slot: &mut f64,
        var_arga_rdn2_slot: &mut f64,
        var_arga_rdn3_slot: &mut f64,
        var_arga_rdn4_slot: &mut f64,
        var_arga_rdn5_slot: &mut f64,
        var_arga_rv_slot: &mut f64,
        var_dv_slot: &mut f64,
        var_dv0_slot: &mut f64,
        var_dv0_db0_slot: &mut f64,
        var_dv0_db1_slot: &mut f64,
        var_dv0_dn0_slot: &mut f64,
        var_dv0_dn1_slot: &mut f64,
        var_dv0_dn2_slot: &mut f64,
        var_dv0_dn3_slot: &mut f64,
        var_dv0_dn4_slot: &mut f64,
        var_dv0_dn5_slot: &mut f64,
        var_dv0_rdb0_slot: &mut f64,
        var_dv0_rdb1_slot: &mut f64,
        var_dv0_rdn0_slot: &mut f64,
        var_dv0_rdn1_slot: &mut f64,
        var_dv0_rdn2_slot: &mut f64,
        var_dv0_rdn3_slot: &mut f64,
        var_dv0_rdn4_slot: &mut f64,
        var_dv0_rdn5_slot: &mut f64,
        var_dv0_rv_slot: &mut f64,
        var_dv_db0_slot: &mut f64,
        var_dv_db1_slot: &mut f64,
        var_dv_dn0_slot: &mut f64,
        var_dv_dn1_slot: &mut f64,
        var_dv_dn2_slot: &mut f64,
        var_dv_dn3_slot: &mut f64,
        var_dv_dn4_slot: &mut f64,
        var_dv_dn5_slot: &mut f64,
        var_dv_rdb0_slot: &mut f64,
        var_dv_rdb1_slot: &mut f64,
        var_dv_rdn0_slot: &mut f64,
        var_dv_rdn1_slot: &mut f64,
        var_dv_rdn2_slot: &mut f64,
        var_dv_rdn3_slot: &mut f64,
        var_dv_rdn4_slot: &mut f64,
        var_dv_rdn5_slot: &mut f64,
        var_dv_rv_slot: &mut f64,
        var_dvh_slot: &mut f64,
        var_dvh_db0_slot: &mut f64,
        var_dvh_db1_slot: &mut f64,
        var_dvh_dn0_slot: &mut f64,
        var_dvh_dn1_slot: &mut f64,
        var_dvh_dn2_slot: &mut f64,
        var_dvh_dn3_slot: &mut f64,
        var_dvh_dn4_slot: &mut f64,
        var_dvh_dn5_slot: &mut f64,
        var_dvh_rdb0_slot: &mut f64,
        var_dvh_rdb1_slot: &mut f64,
        var_dvh_rdn0_slot: &mut f64,
        var_dvh_rdn1_slot: &mut f64,
        var_dvh_rdn2_slot: &mut f64,
        var_dvh_rdn3_slot: &mut f64,
        var_dvh_rdn4_slot: &mut f64,
        var_dvh_rdn5_slot: &mut f64,
        var_dvh_rv_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard254_db0_slot: &mut f64,
        var_guard254_db1_slot: &mut f64,
        var_guard254_dn0_slot: &mut f64,
        var_guard254_dn1_slot: &mut f64,
        var_guard254_dn2_slot: &mut f64,
        var_guard254_dn3_slot: &mut f64,
        var_guard254_dn4_slot: &mut f64,
        var_guard254_dn5_slot: &mut f64,
        var_guard254_rdb0_slot: &mut f64,
        var_guard254_rdb1_slot: &mut f64,
        var_guard254_rdn0_slot: &mut f64,
        var_guard254_rdn1_slot: &mut f64,
        var_guard254_rdn2_slot: &mut f64,
        var_guard254_rdn3_slot: &mut f64,
        var_guard254_rdn4_slot: &mut f64,
        var_guard254_rdn5_slot: &mut f64,
        var_guard254_rv_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard265_db0_slot: &mut f64,
        var_guard265_db1_slot: &mut f64,
        var_guard265_dn0_slot: &mut f64,
        var_guard265_dn1_slot: &mut f64,
        var_guard265_dn2_slot: &mut f64,
        var_guard265_dn3_slot: &mut f64,
        var_guard265_dn4_slot: &mut f64,
        var_guard265_dn5_slot: &mut f64,
        var_guard265_rdb0_slot: &mut f64,
        var_guard265_rdb1_slot: &mut f64,
        var_guard265_rdn0_slot: &mut f64,
        var_guard265_rdn1_slot: &mut f64,
        var_guard265_rdn2_slot: &mut f64,
        var_guard265_rdn3_slot: &mut f64,
        var_guard265_rdn4_slot: &mut f64,
        var_guard265_rdn5_slot: &mut f64,
        var_guard265_rv_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard266_db0_slot: &mut f64,
        var_guard266_db1_slot: &mut f64,
        var_guard266_dn0_slot: &mut f64,
        var_guard266_dn1_slot: &mut f64,
        var_guard266_dn2_slot: &mut f64,
        var_guard266_dn3_slot: &mut f64,
        var_guard266_dn4_slot: &mut f64,
        var_guard266_dn5_slot: &mut f64,
        var_guard266_rdb0_slot: &mut f64,
        var_guard266_rdb1_slot: &mut f64,
        var_guard266_rdn0_slot: &mut f64,
        var_guard266_rdn1_slot: &mut f64,
        var_guard266_rdn2_slot: &mut f64,
        var_guard266_rdn3_slot: &mut f64,
        var_guard266_rdn4_slot: &mut f64,
        var_guard266_rdn5_slot: &mut f64,
        var_guard266_rv_slot: &mut f64,
        var_mv0_slot: &mut f64,
        var_mv0_db0_slot: &mut f64,
        var_mv0_db1_slot: &mut f64,
        var_mv0_dn0_slot: &mut f64,
        var_mv0_dn1_slot: &mut f64,
        var_mv0_dn2_slot: &mut f64,
        var_mv0_dn3_slot: &mut f64,
        var_mv0_dn4_slot: &mut f64,
        var_mv0_dn5_slot: &mut f64,
        var_mv0_rdb0_slot: &mut f64,
        var_mv0_rdb1_slot: &mut f64,
        var_mv0_rdn0_slot: &mut f64,
        var_mv0_rdn1_slot: &mut f64,
        var_mv0_rdn2_slot: &mut f64,
        var_mv0_rdn3_slot: &mut f64,
        var_mv0_rdn4_slot: &mut f64,
        var_mv0_rdn5_slot: &mut f64,
        var_mv0_rv_slot: &mut f64,
        var_pcjp_slot: &mut f64,
        var_pcjp_db0_slot: &mut f64,
        var_pcjp_db1_slot: &mut f64,
        var_pcjp_dn0_slot: &mut f64,
        var_pcjp_dn1_slot: &mut f64,
        var_pcjp_dn2_slot: &mut f64,
        var_pcjp_dn3_slot: &mut f64,
        var_pcjp_dn4_slot: &mut f64,
        var_pcjp_dn5_slot: &mut f64,
        var_pcjp_rdb0_slot: &mut f64,
        var_pcjp_rdb1_slot: &mut f64,
        var_pcjp_rdn0_slot: &mut f64,
        var_pcjp_rdn1_slot: &mut f64,
        var_pcjp_rdn2_slot: &mut f64,
        var_pcjp_rdn3_slot: &mut f64,
        var_pcjp_rdn4_slot: &mut f64,
        var_pcjp_rdn5_slot: &mut f64,
        var_pcjp_rv_slot: &mut f64,
        var_pwq_slot: &mut f64,
        var_pwq_db0_slot: &mut f64,
        var_pwq_db1_slot: &mut f64,
        var_pwq_dn0_slot: &mut f64,
        var_pwq_dn1_slot: &mut f64,
        var_pwq_dn2_slot: &mut f64,
        var_pwq_dn3_slot: &mut f64,
        var_pwq_dn4_slot: &mut f64,
        var_pwq_dn5_slot: &mut f64,
        var_pwq_rdb0_slot: &mut f64,
        var_pwq_rdb1_slot: &mut f64,
        var_pwq_rdn0_slot: &mut f64,
        var_pwq_rdn1_slot: &mut f64,
        var_pwq_rdn2_slot: &mut f64,
        var_pwq_rdn3_slot: &mut f64,
        var_pwq_rdn4_slot: &mut f64,
        var_pwq_rdn5_slot: &mut f64,
        var_pwq_rv_slot: &mut f64,
        var_qhi_slot: &mut f64,
        var_qhi_db0_slot: &mut f64,
        var_qhi_db1_slot: &mut f64,
        var_qhi_dn0_slot: &mut f64,
        var_qhi_dn1_slot: &mut f64,
        var_qhi_dn2_slot: &mut f64,
        var_qhi_dn3_slot: &mut f64,
        var_qhi_dn4_slot: &mut f64,
        var_qhi_dn5_slot: &mut f64,
        var_qhi_rdb0_slot: &mut f64,
        var_qhi_rdb1_slot: &mut f64,
        var_qhi_rdn0_slot: &mut f64,
        var_qhi_rdn1_slot: &mut f64,
        var_qhi_rdn2_slot: &mut f64,
        var_qhi_rdn3_slot: &mut f64,
        var_qhi_rdn4_slot: &mut f64,
        var_qhi_rdn5_slot: &mut f64,
        var_qhi_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
        var_vcl_slot: &mut f64,
        var_vcl_db0_slot: &mut f64,
        var_vcl_db1_slot: &mut f64,
        var_vcl_dn0_slot: &mut f64,
        var_vcl_dn1_slot: &mut f64,
        var_vcl_dn2_slot: &mut f64,
        var_vcl_dn3_slot: &mut f64,
        var_vcl_dn4_slot: &mut f64,
        var_vcl_dn5_slot: &mut f64,
        var_vcl_rdb0_slot: &mut f64,
        var_vcl_rdb1_slot: &mut f64,
        var_vcl_rdn0_slot: &mut f64,
        var_vcl_rdn1_slot: &mut f64,
        var_vcl_rdn2_slot: &mut f64,
        var_vcl_rdn3_slot: &mut f64,
        var_vcl_rdn4_slot: &mut f64,
        var_vcl_rdn5_slot: &mut f64,
        var_vcl_rv_slot: &mut f64,
        var_vl0_slot: &mut f64,
        var_vl0_db0_slot: &mut f64,
        var_vl0_db1_slot: &mut f64,
        var_vl0_dn0_slot: &mut f64,
        var_vl0_dn1_slot: &mut f64,
        var_vl0_dn2_slot: &mut f64,
        var_vl0_dn3_slot: &mut f64,
        var_vl0_dn4_slot: &mut f64,
        var_vl0_dn5_slot: &mut f64,
        var_vl0_rdb0_slot: &mut f64,
        var_vl0_rdb1_slot: &mut f64,
        var_vl0_rdn0_slot: &mut f64,
        var_vl0_rdn1_slot: &mut f64,
        var_vl0_rdn2_slot: &mut f64,
        var_vl0_rdn3_slot: &mut f64,
        var_vl0_rdn4_slot: &mut f64,
        var_vl0_rdn5_slot: &mut f64,
        var_vl0_rv_slot: &mut f64,
    ) {
        let mut var_acja: f64 = *var_acja_slot;
        let mut var_acja_db0: f64 = *var_acja_db0_slot;
        let mut var_acja_db1: f64 = *var_acja_db1_slot;
        let mut var_acja_dn0: f64 = *var_acja_dn0_slot;
        let mut var_acja_dn1: f64 = *var_acja_dn1_slot;
        let mut var_acja_dn2: f64 = *var_acja_dn2_slot;
        let mut var_acja_dn3: f64 = *var_acja_dn3_slot;
        let mut var_acja_dn4: f64 = *var_acja_dn4_slot;
        let mut var_acja_dn5: f64 = *var_acja_dn5_slot;
        let mut var_acja_rdb0: f64 = *var_acja_rdb0_slot;
        let mut var_acja_rdb1: f64 = *var_acja_rdb1_slot;
        let mut var_acja_rdn0: f64 = *var_acja_rdn0_slot;
        let mut var_acja_rdn1: f64 = *var_acja_rdn1_slot;
        let mut var_acja_rdn2: f64 = *var_acja_rdn2_slot;
        let mut var_acja_rdn3: f64 = *var_acja_rdn3_slot;
        let mut var_acja_rdn4: f64 = *var_acja_rdn4_slot;
        let mut var_acja_rdn5: f64 = *var_acja_rdn5_slot;
        let mut var_acja_rv: f64 = *var_acja_rv_slot;
        let mut var_arga: f64 = *var_arga_slot;
        let mut var_arga_db0: f64 = *var_arga_db0_slot;
        let mut var_arga_db1: f64 = *var_arga_db1_slot;
        let mut var_arga_dn0: f64 = *var_arga_dn0_slot;
        let mut var_arga_dn1: f64 = *var_arga_dn1_slot;
        let mut var_arga_dn2: f64 = *var_arga_dn2_slot;
        let mut var_arga_dn3: f64 = *var_arga_dn3_slot;
        let mut var_arga_dn4: f64 = *var_arga_dn4_slot;
        let mut var_arga_dn5: f64 = *var_arga_dn5_slot;
        let mut var_arga_rdb0: f64 = *var_arga_rdb0_slot;
        let mut var_arga_rdb1: f64 = *var_arga_rdb1_slot;
        let mut var_arga_rdn0: f64 = *var_arga_rdn0_slot;
        let mut var_arga_rdn1: f64 = *var_arga_rdn1_slot;
        let mut var_arga_rdn2: f64 = *var_arga_rdn2_slot;
        let mut var_arga_rdn3: f64 = *var_arga_rdn3_slot;
        let mut var_arga_rdn4: f64 = *var_arga_rdn4_slot;
        let mut var_arga_rdn5: f64 = *var_arga_rdn5_slot;
        let mut var_arga_rv: f64 = *var_arga_rv_slot;
        let mut var_dv: f64 = *var_dv_slot;
        let mut var_dv0: f64 = *var_dv0_slot;
        let mut var_dv0_db0: f64 = *var_dv0_db0_slot;
        let mut var_dv0_db1: f64 = *var_dv0_db1_slot;
        let mut var_dv0_dn0: f64 = *var_dv0_dn0_slot;
        let mut var_dv0_dn1: f64 = *var_dv0_dn1_slot;
        let mut var_dv0_dn2: f64 = *var_dv0_dn2_slot;
        let mut var_dv0_dn3: f64 = *var_dv0_dn3_slot;
        let mut var_dv0_dn4: f64 = *var_dv0_dn4_slot;
        let mut var_dv0_dn5: f64 = *var_dv0_dn5_slot;
        let mut var_dv0_rdb0: f64 = *var_dv0_rdb0_slot;
        let mut var_dv0_rdb1: f64 = *var_dv0_rdb1_slot;
        let mut var_dv0_rdn0: f64 = *var_dv0_rdn0_slot;
        let mut var_dv0_rdn1: f64 = *var_dv0_rdn1_slot;
        let mut var_dv0_rdn2: f64 = *var_dv0_rdn2_slot;
        let mut var_dv0_rdn3: f64 = *var_dv0_rdn3_slot;
        let mut var_dv0_rdn4: f64 = *var_dv0_rdn4_slot;
        let mut var_dv0_rdn5: f64 = *var_dv0_rdn5_slot;
        let mut var_dv0_rv: f64 = *var_dv0_rv_slot;
        let mut var_dv_db0: f64 = *var_dv_db0_slot;
        let mut var_dv_db1: f64 = *var_dv_db1_slot;
        let mut var_dv_dn0: f64 = *var_dv_dn0_slot;
        let mut var_dv_dn1: f64 = *var_dv_dn1_slot;
        let mut var_dv_dn2: f64 = *var_dv_dn2_slot;
        let mut var_dv_dn3: f64 = *var_dv_dn3_slot;
        let mut var_dv_dn4: f64 = *var_dv_dn4_slot;
        let mut var_dv_dn5: f64 = *var_dv_dn5_slot;
        let mut var_dv_rdb0: f64 = *var_dv_rdb0_slot;
        let mut var_dv_rdb1: f64 = *var_dv_rdb1_slot;
        let mut var_dv_rdn0: f64 = *var_dv_rdn0_slot;
        let mut var_dv_rdn1: f64 = *var_dv_rdn1_slot;
        let mut var_dv_rdn2: f64 = *var_dv_rdn2_slot;
        let mut var_dv_rdn3: f64 = *var_dv_rdn3_slot;
        let mut var_dv_rdn4: f64 = *var_dv_rdn4_slot;
        let mut var_dv_rdn5: f64 = *var_dv_rdn5_slot;
        let mut var_dv_rv: f64 = *var_dv_rv_slot;
        let mut var_dvh: f64 = *var_dvh_slot;
        let mut var_dvh_db0: f64 = *var_dvh_db0_slot;
        let mut var_dvh_db1: f64 = *var_dvh_db1_slot;
        let mut var_dvh_dn0: f64 = *var_dvh_dn0_slot;
        let mut var_dvh_dn1: f64 = *var_dvh_dn1_slot;
        let mut var_dvh_dn2: f64 = *var_dvh_dn2_slot;
        let mut var_dvh_dn3: f64 = *var_dvh_dn3_slot;
        let mut var_dvh_dn4: f64 = *var_dvh_dn4_slot;
        let mut var_dvh_dn5: f64 = *var_dvh_dn5_slot;
        let mut var_dvh_rdb0: f64 = *var_dvh_rdb0_slot;
        let mut var_dvh_rdb1: f64 = *var_dvh_rdb1_slot;
        let mut var_dvh_rdn0: f64 = *var_dvh_rdn0_slot;
        let mut var_dvh_rdn1: f64 = *var_dvh_rdn1_slot;
        let mut var_dvh_rdn2: f64 = *var_dvh_rdn2_slot;
        let mut var_dvh_rdn3: f64 = *var_dvh_rdn3_slot;
        let mut var_dvh_rdn4: f64 = *var_dvh_rdn4_slot;
        let mut var_dvh_rdn5: f64 = *var_dvh_rdn5_slot;
        let mut var_dvh_rv: f64 = *var_dvh_rv_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard254_db0: f64 = *var_guard254_db0_slot;
        let mut var_guard254_db1: f64 = *var_guard254_db1_slot;
        let mut var_guard254_dn0: f64 = *var_guard254_dn0_slot;
        let mut var_guard254_dn1: f64 = *var_guard254_dn1_slot;
        let mut var_guard254_dn2: f64 = *var_guard254_dn2_slot;
        let mut var_guard254_dn3: f64 = *var_guard254_dn3_slot;
        let mut var_guard254_dn4: f64 = *var_guard254_dn4_slot;
        let mut var_guard254_dn5: f64 = *var_guard254_dn5_slot;
        let mut var_guard254_rdb0: f64 = *var_guard254_rdb0_slot;
        let mut var_guard254_rdb1: f64 = *var_guard254_rdb1_slot;
        let mut var_guard254_rdn0: f64 = *var_guard254_rdn0_slot;
        let mut var_guard254_rdn1: f64 = *var_guard254_rdn1_slot;
        let mut var_guard254_rdn2: f64 = *var_guard254_rdn2_slot;
        let mut var_guard254_rdn3: f64 = *var_guard254_rdn3_slot;
        let mut var_guard254_rdn4: f64 = *var_guard254_rdn4_slot;
        let mut var_guard254_rdn5: f64 = *var_guard254_rdn5_slot;
        let mut var_guard254_rv: f64 = *var_guard254_rv_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard265_db0: f64 = *var_guard265_db0_slot;
        let mut var_guard265_db1: f64 = *var_guard265_db1_slot;
        let mut var_guard265_dn0: f64 = *var_guard265_dn0_slot;
        let mut var_guard265_dn1: f64 = *var_guard265_dn1_slot;
        let mut var_guard265_dn2: f64 = *var_guard265_dn2_slot;
        let mut var_guard265_dn3: f64 = *var_guard265_dn3_slot;
        let mut var_guard265_dn4: f64 = *var_guard265_dn4_slot;
        let mut var_guard265_dn5: f64 = *var_guard265_dn5_slot;
        let mut var_guard265_rdb0: f64 = *var_guard265_rdb0_slot;
        let mut var_guard265_rdb1: f64 = *var_guard265_rdb1_slot;
        let mut var_guard265_rdn0: f64 = *var_guard265_rdn0_slot;
        let mut var_guard265_rdn1: f64 = *var_guard265_rdn1_slot;
        let mut var_guard265_rdn2: f64 = *var_guard265_rdn2_slot;
        let mut var_guard265_rdn3: f64 = *var_guard265_rdn3_slot;
        let mut var_guard265_rdn4: f64 = *var_guard265_rdn4_slot;
        let mut var_guard265_rdn5: f64 = *var_guard265_rdn5_slot;
        let mut var_guard265_rv: f64 = *var_guard265_rv_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard266_db0: f64 = *var_guard266_db0_slot;
        let mut var_guard266_db1: f64 = *var_guard266_db1_slot;
        let mut var_guard266_dn0: f64 = *var_guard266_dn0_slot;
        let mut var_guard266_dn1: f64 = *var_guard266_dn1_slot;
        let mut var_guard266_dn2: f64 = *var_guard266_dn2_slot;
        let mut var_guard266_dn3: f64 = *var_guard266_dn3_slot;
        let mut var_guard266_dn4: f64 = *var_guard266_dn4_slot;
        let mut var_guard266_dn5: f64 = *var_guard266_dn5_slot;
        let mut var_guard266_rdb0: f64 = *var_guard266_rdb0_slot;
        let mut var_guard266_rdb1: f64 = *var_guard266_rdb1_slot;
        let mut var_guard266_rdn0: f64 = *var_guard266_rdn0_slot;
        let mut var_guard266_rdn1: f64 = *var_guard266_rdn1_slot;
        let mut var_guard266_rdn2: f64 = *var_guard266_rdn2_slot;
        let mut var_guard266_rdn3: f64 = *var_guard266_rdn3_slot;
        let mut var_guard266_rdn4: f64 = *var_guard266_rdn4_slot;
        let mut var_guard266_rdn5: f64 = *var_guard266_rdn5_slot;
        let mut var_guard266_rv: f64 = *var_guard266_rv_slot;
        let mut var_mv0: f64 = *var_mv0_slot;
        let mut var_mv0_db0: f64 = *var_mv0_db0_slot;
        let mut var_mv0_db1: f64 = *var_mv0_db1_slot;
        let mut var_mv0_dn0: f64 = *var_mv0_dn0_slot;
        let mut var_mv0_dn1: f64 = *var_mv0_dn1_slot;
        let mut var_mv0_dn2: f64 = *var_mv0_dn2_slot;
        let mut var_mv0_dn3: f64 = *var_mv0_dn3_slot;
        let mut var_mv0_dn4: f64 = *var_mv0_dn4_slot;
        let mut var_mv0_dn5: f64 = *var_mv0_dn5_slot;
        let mut var_mv0_rdb0: f64 = *var_mv0_rdb0_slot;
        let mut var_mv0_rdb1: f64 = *var_mv0_rdb1_slot;
        let mut var_mv0_rdn0: f64 = *var_mv0_rdn0_slot;
        let mut var_mv0_rdn1: f64 = *var_mv0_rdn1_slot;
        let mut var_mv0_rdn2: f64 = *var_mv0_rdn2_slot;
        let mut var_mv0_rdn3: f64 = *var_mv0_rdn3_slot;
        let mut var_mv0_rdn4: f64 = *var_mv0_rdn4_slot;
        let mut var_mv0_rdn5: f64 = *var_mv0_rdn5_slot;
        let mut var_mv0_rv: f64 = *var_mv0_rv_slot;
        let mut var_pcjp: f64 = *var_pcjp_slot;
        let mut var_pcjp_db0: f64 = *var_pcjp_db0_slot;
        let mut var_pcjp_db1: f64 = *var_pcjp_db1_slot;
        let mut var_pcjp_dn0: f64 = *var_pcjp_dn0_slot;
        let mut var_pcjp_dn1: f64 = *var_pcjp_dn1_slot;
        let mut var_pcjp_dn2: f64 = *var_pcjp_dn2_slot;
        let mut var_pcjp_dn3: f64 = *var_pcjp_dn3_slot;
        let mut var_pcjp_dn4: f64 = *var_pcjp_dn4_slot;
        let mut var_pcjp_dn5: f64 = *var_pcjp_dn5_slot;
        let mut var_pcjp_rdb0: f64 = *var_pcjp_rdb0_slot;
        let mut var_pcjp_rdb1: f64 = *var_pcjp_rdb1_slot;
        let mut var_pcjp_rdn0: f64 = *var_pcjp_rdn0_slot;
        let mut var_pcjp_rdn1: f64 = *var_pcjp_rdn1_slot;
        let mut var_pcjp_rdn2: f64 = *var_pcjp_rdn2_slot;
        let mut var_pcjp_rdn3: f64 = *var_pcjp_rdn3_slot;
        let mut var_pcjp_rdn4: f64 = *var_pcjp_rdn4_slot;
        let mut var_pcjp_rdn5: f64 = *var_pcjp_rdn5_slot;
        let mut var_pcjp_rv: f64 = *var_pcjp_rv_slot;
        let mut var_pwq: f64 = *var_pwq_slot;
        let mut var_pwq_db0: f64 = *var_pwq_db0_slot;
        let mut var_pwq_db1: f64 = *var_pwq_db1_slot;
        let mut var_pwq_dn0: f64 = *var_pwq_dn0_slot;
        let mut var_pwq_dn1: f64 = *var_pwq_dn1_slot;
        let mut var_pwq_dn2: f64 = *var_pwq_dn2_slot;
        let mut var_pwq_dn3: f64 = *var_pwq_dn3_slot;
        let mut var_pwq_dn4: f64 = *var_pwq_dn4_slot;
        let mut var_pwq_dn5: f64 = *var_pwq_dn5_slot;
        let mut var_pwq_rdb0: f64 = *var_pwq_rdb0_slot;
        let mut var_pwq_rdb1: f64 = *var_pwq_rdb1_slot;
        let mut var_pwq_rdn0: f64 = *var_pwq_rdn0_slot;
        let mut var_pwq_rdn1: f64 = *var_pwq_rdn1_slot;
        let mut var_pwq_rdn2: f64 = *var_pwq_rdn2_slot;
        let mut var_pwq_rdn3: f64 = *var_pwq_rdn3_slot;
        let mut var_pwq_rdn4: f64 = *var_pwq_rdn4_slot;
        let mut var_pwq_rdn5: f64 = *var_pwq_rdn5_slot;
        let mut var_pwq_rv: f64 = *var_pwq_rv_slot;
        let mut var_qhi: f64 = *var_qhi_slot;
        let mut var_qhi_db0: f64 = *var_qhi_db0_slot;
        let mut var_qhi_db1: f64 = *var_qhi_db1_slot;
        let mut var_qhi_dn0: f64 = *var_qhi_dn0_slot;
        let mut var_qhi_dn1: f64 = *var_qhi_dn1_slot;
        let mut var_qhi_dn2: f64 = *var_qhi_dn2_slot;
        let mut var_qhi_dn3: f64 = *var_qhi_dn3_slot;
        let mut var_qhi_dn4: f64 = *var_qhi_dn4_slot;
        let mut var_qhi_dn5: f64 = *var_qhi_dn5_slot;
        let mut var_qhi_rdb0: f64 = *var_qhi_rdb0_slot;
        let mut var_qhi_rdb1: f64 = *var_qhi_rdb1_slot;
        let mut var_qhi_rdn0: f64 = *var_qhi_rdn0_slot;
        let mut var_qhi_rdn1: f64 = *var_qhi_rdn1_slot;
        let mut var_qhi_rdn2: f64 = *var_qhi_rdn2_slot;
        let mut var_qhi_rdn3: f64 = *var_qhi_rdn3_slot;
        let mut var_qhi_rdn4: f64 = *var_qhi_rdn4_slot;
        let mut var_qhi_rdn5: f64 = *var_qhi_rdn5_slot;
        let mut var_qhi_rv: f64 = *var_qhi_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;
        let mut var_vcl: f64 = *var_vcl_slot;
        let mut var_vcl_db0: f64 = *var_vcl_db0_slot;
        let mut var_vcl_db1: f64 = *var_vcl_db1_slot;
        let mut var_vcl_dn0: f64 = *var_vcl_dn0_slot;
        let mut var_vcl_dn1: f64 = *var_vcl_dn1_slot;
        let mut var_vcl_dn2: f64 = *var_vcl_dn2_slot;
        let mut var_vcl_dn3: f64 = *var_vcl_dn3_slot;
        let mut var_vcl_dn4: f64 = *var_vcl_dn4_slot;
        let mut var_vcl_dn5: f64 = *var_vcl_dn5_slot;
        let mut var_vcl_rdb0: f64 = *var_vcl_rdb0_slot;
        let mut var_vcl_rdb1: f64 = *var_vcl_rdb1_slot;
        let mut var_vcl_rdn0: f64 = *var_vcl_rdn0_slot;
        let mut var_vcl_rdn1: f64 = *var_vcl_rdn1_slot;
        let mut var_vcl_rdn2: f64 = *var_vcl_rdn2_slot;
        let mut var_vcl_rdn3: f64 = *var_vcl_rdn3_slot;
        let mut var_vcl_rdn4: f64 = *var_vcl_rdn4_slot;
        let mut var_vcl_rdn5: f64 = *var_vcl_rdn5_slot;
        let mut var_vcl_rv: f64 = *var_vcl_rv_slot;
        let mut var_vl0: f64 = *var_vl0_slot;
        let mut var_vl0_db0: f64 = *var_vl0_db0_slot;
        let mut var_vl0_db1: f64 = *var_vl0_db1_slot;
        let mut var_vl0_dn0: f64 = *var_vl0_dn0_slot;
        let mut var_vl0_dn1: f64 = *var_vl0_dn1_slot;
        let mut var_vl0_dn2: f64 = *var_vl0_dn2_slot;
        let mut var_vl0_dn3: f64 = *var_vl0_dn3_slot;
        let mut var_vl0_dn4: f64 = *var_vl0_dn4_slot;
        let mut var_vl0_dn5: f64 = *var_vl0_dn5_slot;
        let mut var_vl0_rdb0: f64 = *var_vl0_rdb0_slot;
        let mut var_vl0_rdb1: f64 = *var_vl0_rdb1_slot;
        let mut var_vl0_rdn0: f64 = *var_vl0_rdn0_slot;
        let mut var_vl0_rdn1: f64 = *var_vl0_rdn1_slot;
        let mut var_vl0_rdn2: f64 = *var_vl0_rdn2_slot;
        let mut var_vl0_rdn3: f64 = *var_vl0_rdn3_slot;
        let mut var_vl0_rdn4: f64 = *var_vl0_rdn4_slot;
        let mut var_vl0_rdn5: f64 = *var_vl0_rdn5_slot;
        let mut var_vl0_rv: f64 = *var_vl0_rv_slot;

        let (assign3810_e3676, assign3810_e3676_d_n0, assign3810_e3676_d_n1, assign3810_e3676_d_n2, assign3810_e3676_d_n3, assign3810_e3676_d_n4, assign3810_e3676_d_n5, assign3810_e3676_d_b0, assign3810_e3676_d_b1,) = {
    if ((var_guard249 != 0.0) && (p.p63 != 0.0)) {
        let assign3810_e3662: f64 = (var_vc1 - var_vpo);
        let assign3810_e3665: f64 = (var_vc1 + var_vpo);
        let assign3810_e3668: f64 = (var_vc1 + var_vpo);
        let assign3810_e3669: f64 = (assign3810_e3665 * assign3810_e3668);
        let assign3810_e3671: f64 = (assign3810_e3669 + 0.04);
        let assign3810_e3672: f64 = (assign3810_e3671).sqrt();
        let assign3810_e3673: f64 = (assign3810_e3662 + assign3810_e3672);
        let assign3810_e3674: f64 = (0.5 * assign3810_e3673);
        (assign3810_e3674, (0.5 * ((var_vc1_dn0 - var_vpo_dn0) + ((((var_vc1_dn0 + var_vpo_dn0) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn0 + var_vpo_dn0))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_dn1 - var_vpo_dn1) + ((((var_vc1_dn1 + var_vpo_dn1) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn1 + var_vpo_dn1))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_dn2 - var_vpo_dn2) + ((((var_vc1_dn2 + var_vpo_dn2) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn2 + var_vpo_dn2))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_dn3 - var_vpo_dn3) + ((((var_vc1_dn3 + var_vpo_dn3) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn3 + var_vpo_dn3))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_dn4 - var_vpo_dn4) + ((((var_vc1_dn4 + var_vpo_dn4) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn4 + var_vpo_dn4))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_dn5 - var_vpo_dn5) + ((((var_vc1_dn5 + var_vpo_dn5) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_dn5 + var_vpo_dn5))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_db0 - var_vpo_db0) + ((((var_vc1_db0 + var_vpo_db0) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_db0 + var_vpo_db0))) / (2.0 * assign3810_e3672)))), (0.5 * ((var_vc1_db1 - var_vpo_db1) + ((((var_vc1_db1 + var_vpo_db1) * assign3810_e3668) + (assign3810_e3665 * (var_vc1_db1 + var_vpo_db1))) / (2.0 * assign3810_e3672)))),)
    } else {
        (var_vcl, var_vcl_dn0, var_vcl_dn1, var_vcl_dn2, var_vcl_dn3, var_vcl_dn4, var_vcl_dn5, var_vcl_db0, var_vcl_db1,)
    }
};
        var_vcl = assign3810_e3676;
        var_vcl_dn0 = assign3810_e3676_d_n0;
        var_vcl_dn1 = assign3810_e3676_d_n1;
        var_vcl_dn2 = assign3810_e3676_d_n2;
        var_vcl_dn3 = assign3810_e3676_d_n3;
        var_vcl_dn4 = assign3810_e3676_d_n4;
        var_vcl_dn5 = assign3810_e3676_d_n5;
        var_vcl_db0 = assign3810_e3676_d_b0;
        var_vcl_db1 = assign3810_e3676_d_b1;
        var_vcl_rv = 0.0;
        var_vcl_rdn0 = 0.0;
        var_vcl_rdn1 = 0.0;
        var_vcl_rdn2 = 0.0;
        var_vcl_rdn3 = 0.0;
        var_vcl_rdn4 = 0.0;
        var_vcl_rdn5 = 0.0;
        var_vcl_rdb0 = 0.0;
        var_vcl_rdb1 = 0.0;

        let (assign3820_e3683, assign3820_e3683_d_n0, assign3820_e3683_d_n1, assign3820_e3683_d_n2, assign3820_e3683_d_n3, assign3820_e3683_d_n4, assign3820_e3683_d_n5, assign3820_e3683_d_b0, assign3820_e3683_d_b1,) = {
    if ((var_guard249 != 0.0) && (p.p63 == 0.0)) {
        (var_vc1, var_vc1_dn0, var_vc1_dn1, var_vc1_dn2, var_vc1_dn3, var_vc1_dn4, var_vc1_dn5, var_vc1_db0, var_vc1_db1,)
    } else {
        (var_vcl, var_vcl_dn0, var_vcl_dn1, var_vcl_dn2, var_vcl_dn3, var_vcl_dn4, var_vcl_dn5, var_vcl_db0, var_vcl_db1,)
    }
};
        var_vcl = assign3820_e3683;
        var_vcl_dn0 = assign3820_e3683_d_n0;
        var_vcl_dn1 = assign3820_e3683_d_n1;
        var_vcl_dn2 = assign3820_e3683_d_n2;
        var_vcl_dn3 = assign3820_e3683_d_n3;
        var_vcl_dn4 = assign3820_e3683_d_n4;
        var_vcl_dn5 = assign3820_e3683_d_n5;
        var_vcl_db0 = assign3820_e3683_d_b0;
        var_vcl_db1 = assign3820_e3683_d_b1;
        var_vcl_rv = 0.0;
        var_vcl_rdn0 = 0.0;
        var_vcl_rdn1 = 0.0;
        var_vcl_rdn2 = 0.0;
        var_vcl_rdn3 = 0.0;
        var_vcl_rdn4 = 0.0;
        var_vcl_rdn5 = 0.0;
        var_vcl_rdb0 = 0.0;
        var_vcl_rdb1 = 0.0;

        let (assign3830_e3689, assign3830_e3689_d_n0, assign3830_e3689_d_n1, assign3830_e3689_d_n2, assign3830_e3689_d_n3, assign3830_e3689_d_n4, assign3830_e3689_d_n5, assign3830_e3689_d_b0, assign3830_e3689_d_b1,) = {
    if (var_guard249 != 0.0) {
        let assign3830_e3687: f64 = (var_a1_um2 * var_cja_t);
        (assign3830_e3687, ((var_a1_um2_dn0 * var_cja_t) + (var_a1_um2 * var_cja_t_dn0)), ((var_a1_um2_dn1 * var_cja_t) + (var_a1_um2 * var_cja_t_dn1)), ((var_a1_um2_dn2 * var_cja_t) + (var_a1_um2 * var_cja_t_dn2)), ((var_a1_um2_dn3 * var_cja_t) + (var_a1_um2 * var_cja_t_dn3)), ((var_a1_um2_dn4 * var_cja_t) + (var_a1_um2 * var_cja_t_dn4)), ((var_a1_um2_dn5 * var_cja_t) + (var_a1_um2 * var_cja_t_dn5)), ((var_a1_um2_db0 * var_cja_t) + (var_a1_um2 * var_cja_t_db0)), ((var_a1_um2_db1 * var_cja_t) + (var_a1_um2 * var_cja_t_db1)),)
    } else {
        (var_acja, var_acja_dn0, var_acja_dn1, var_acja_dn2, var_acja_dn3, var_acja_dn4, var_acja_dn5, var_acja_db0, var_acja_db1,)
    }
};
        var_acja = assign3830_e3689;
        var_acja_dn0 = assign3830_e3689_d_n0;
        var_acja_dn1 = assign3830_e3689_d_n1;
        var_acja_dn2 = assign3830_e3689_d_n2;
        var_acja_dn3 = assign3830_e3689_d_n3;
        var_acja_dn4 = assign3830_e3689_d_n4;
        var_acja_dn5 = assign3830_e3689_d_n5;
        var_acja_db0 = assign3830_e3689_d_b0;
        var_acja_db1 = assign3830_e3689_d_b1;
        var_acja_rv = 0.0;
        var_acja_rdn0 = 0.0;
        var_acja_rdn1 = 0.0;
        var_acja_rdn2 = 0.0;
        var_acja_rdn3 = 0.0;
        var_acja_rdn4 = 0.0;
        var_acja_rdn5 = 0.0;
        var_acja_rdb0 = 0.0;
        var_acja_rdb1 = 0.0;

        let (assign3840_e3695, assign3840_e3695_d_n0, assign3840_e3695_d_n1, assign3840_e3695_d_n2, assign3840_e3695_d_n3, assign3840_e3695_d_n4, assign3840_e3695_d_n5, assign3840_e3695_d_b0, assign3840_e3695_d_b1,) = {
    if (var_guard249 != 0.0) {
        let assign3840_e3693: f64 = (var_p1_um * var_cjp_t);
        (assign3840_e3693, ((var_p1_um_dn0 * var_cjp_t) + (var_p1_um * var_cjp_t_dn0)), ((var_p1_um_dn1 * var_cjp_t) + (var_p1_um * var_cjp_t_dn1)), ((var_p1_um_dn2 * var_cjp_t) + (var_p1_um * var_cjp_t_dn2)), ((var_p1_um_dn3 * var_cjp_t) + (var_p1_um * var_cjp_t_dn3)), ((var_p1_um_dn4 * var_cjp_t) + (var_p1_um * var_cjp_t_dn4)), ((var_p1_um_dn5 * var_cjp_t) + (var_p1_um * var_cjp_t_dn5)), ((var_p1_um_db0 * var_cjp_t) + (var_p1_um * var_cjp_t_db0)), ((var_p1_um_db1 * var_cjp_t) + (var_p1_um * var_cjp_t_db1)),)
    } else {
        (var_pcjp, var_pcjp_dn0, var_pcjp_dn1, var_pcjp_dn2, var_pcjp_dn3, var_pcjp_dn4, var_pcjp_dn5, var_pcjp_db0, var_pcjp_db1,)
    }
};
        var_pcjp = assign3840_e3695;
        var_pcjp_dn0 = assign3840_e3695_d_n0;
        var_pcjp_dn1 = assign3840_e3695_d_n1;
        var_pcjp_dn2 = assign3840_e3695_d_n2;
        var_pcjp_dn3 = assign3840_e3695_d_n3;
        var_pcjp_dn4 = assign3840_e3695_d_n4;
        var_pcjp_dn5 = assign3840_e3695_d_n5;
        var_pcjp_db0 = assign3840_e3695_d_b0;
        var_pcjp_db1 = assign3840_e3695_d_b1;
        var_pcjp_rv = 0.0;
        var_pcjp_rdn0 = 0.0;
        var_pcjp_rdn1 = 0.0;
        var_pcjp_rdn2 = 0.0;
        var_pcjp_rdn3 = 0.0;
        var_pcjp_rdn4 = 0.0;
        var_pcjp_rdn5 = 0.0;
        var_pcjp_rdb0 = 0.0;
        var_pcjp_rdb1 = 0.0;

        let assign3850_e3698: f64 = if var_acja > 0.0 { 1.0 } else { 0.0 };
        var_guard254 = assign3850_e3698;
        var_guard254_dn0 = 0.0;
        var_guard254_dn1 = 0.0;
        var_guard254_dn2 = 0.0;
        var_guard254_dn3 = 0.0;
        var_guard254_dn4 = 0.0;
        var_guard254_dn5 = 0.0;
        var_guard254_db0 = 0.0;
        var_guard254_db1 = 0.0;
        var_guard254_rv = 0.0;
        var_guard254_rdn0 = 0.0;
        var_guard254_rdn1 = 0.0;
        var_guard254_rdn2 = 0.0;
        var_guard254_rdn3 = 0.0;
        var_guard254_rdn4 = 0.0;
        var_guard254_rdn5 = 0.0;
        var_guard254_rdb0 = 0.0;
        var_guard254_rdb1 = 0.0;

        let (assign3860_e3707, assign3860_e3707_d_n0, assign3860_e3707_d_n1, assign3860_e3707_d_n2, assign3860_e3707_d_n3, assign3860_e3707_d_n4, assign3860_e3707_d_n5, assign3860_e3707_d_b0, assign3860_e3707_d_b1,) = {
    if ((var_guard249 != 0.0) && (var_guard254 != 0.0)) {
        let assign3860_e3703: f64 = (-var_pa_t);
        let assign3860_e3705: f64 = (assign3860_e3703 * p.p68);
        (assign3860_e3705, ((-var_pa_t_dn0) * p.p68), ((-var_pa_t_dn1) * p.p68), ((-var_pa_t_dn2) * p.p68), ((-var_pa_t_dn3) * p.p68), ((-var_pa_t_dn4) * p.p68), ((-var_pa_t_dn5) * p.p68), ((-var_pa_t_db0) * p.p68), ((-var_pa_t_db1) * p.p68),)
    } else {
        (var_dv0, var_dv0_dn0, var_dv0_dn1, var_dv0_dn2, var_dv0_dn3, var_dv0_dn4, var_dv0_dn5, var_dv0_db0, var_dv0_db1,)
    }
};
        var_dv0 = assign3860_e3707;
        var_dv0_dn0 = assign3860_e3707_d_n0;
        var_dv0_dn1 = assign3860_e3707_d_n1;
        var_dv0_dn2 = assign3860_e3707_d_n2;
        var_dv0_dn3 = assign3860_e3707_d_n3;
        var_dv0_dn4 = assign3860_e3707_d_n4;
        var_dv0_dn5 = assign3860_e3707_d_n5;
        var_dv0_db0 = assign3860_e3707_d_b0;
        var_dv0_db1 = assign3860_e3707_d_b1;
        var_dv0_rv = 0.0;
        var_dv0_rdn0 = 0.0;
        var_dv0_rdn1 = 0.0;
        var_dv0_rdn2 = 0.0;
        var_dv0_rdn3 = 0.0;
        var_dv0_rdn4 = 0.0;
        var_dv0_rdn5 = 0.0;
        var_dv0_rdb0 = 0.0;
        var_dv0_rdb1 = 0.0;

        let assign3870_e3710: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        var_guard265 = assign3870_e3710;
        var_guard265_dn0 = 0.0;
        var_guard265_dn1 = 0.0;
        var_guard265_dn2 = 0.0;
        var_guard265_dn3 = 0.0;
        var_guard265_dn4 = 0.0;
        var_guard265_dn5 = 0.0;
        var_guard265_db0 = 0.0;
        var_guard265_db1 = 0.0;
        var_guard265_rv = 0.0;
        var_guard265_rdn0 = 0.0;
        var_guard265_rdn1 = 0.0;
        var_guard265_rdn2 = 0.0;
        var_guard265_rdn3 = 0.0;
        var_guard265_rdn4 = 0.0;
        var_guard265_rdn5 = 0.0;
        var_guard265_rdb0 = 0.0;
        var_guard265_rdb1 = 0.0;

        let (assign3880_e3720, assign3880_e3720_d_n0, assign3880_e3720_d_n1, assign3880_e3720_d_n2, assign3880_e3720_d_n3, assign3880_e3720_d_n4, assign3880_e3720_d_n5, assign3880_e3720_d_b0, assign3880_e3720_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) {
        let assign3880_e3718: f64 = (var_vcl + var_dv0);
        (assign3880_e3718, (var_vcl_dn0 + var_dv0_dn0), (var_vcl_dn1 + var_dv0_dn1), (var_vcl_dn2 + var_dv0_dn2), (var_vcl_dn3 + var_dv0_dn3), (var_vcl_dn4 + var_dv0_dn4), (var_vcl_dn5 + var_dv0_dn5), (var_vcl_db0 + var_dv0_db0), (var_vcl_db1 + var_dv0_db1),)
    } else {
        (var_dvh, var_dvh_dn0, var_dvh_dn1, var_dvh_dn2, var_dvh_dn3, var_dvh_dn4, var_dvh_dn5, var_dvh_db0, var_dvh_db1,)
    }
};
        var_dvh = assign3880_e3720;
        var_dvh_dn0 = assign3880_e3720_d_n0;
        var_dvh_dn1 = assign3880_e3720_d_n1;
        var_dvh_dn2 = assign3880_e3720_d_n2;
        var_dvh_dn3 = assign3880_e3720_d_n3;
        var_dvh_dn4 = assign3880_e3720_d_n4;
        var_dvh_dn5 = assign3880_e3720_d_n5;
        var_dvh_db0 = assign3880_e3720_d_b0;
        var_dvh_db1 = assign3880_e3720_d_b1;
        var_dvh_rv = 0.0;
        var_dvh_rdn0 = 0.0;
        var_dvh_rdn1 = 0.0;
        var_dvh_rdn2 = 0.0;
        var_dvh_rdn3 = 0.0;
        var_dvh_rdn4 = 0.0;
        var_dvh_rdn5 = 0.0;
        var_dvh_rdb0 = 0.0;
        var_dvh_rdb1 = 0.0;

        let assign3890_e3723: f64 = if var_dvh > 0.0 { 1.0 } else { 0.0 };
        var_guard266 = assign3890_e3723;
        var_guard266_dn0 = 0.0;
        var_guard266_dn1 = 0.0;
        var_guard266_dn2 = 0.0;
        var_guard266_dn3 = 0.0;
        var_guard266_dn4 = 0.0;
        var_guard266_dn5 = 0.0;
        var_guard266_db0 = 0.0;
        var_guard266_db1 = 0.0;
        var_guard266_rv = 0.0;
        var_guard266_rdn0 = 0.0;
        var_guard266_rdn1 = 0.0;
        var_guard266_rdn2 = 0.0;
        var_guard266_rdn3 = 0.0;
        var_guard266_rdn4 = 0.0;
        var_guard266_rdn5 = 0.0;
        var_guard266_rdb0 = 0.0;
        var_guard266_rdb1 = 0.0;

        let (assign3900_e3738, assign3900_e3738_d_n0, assign3900_e3738_d_n1, assign3900_e3738_d_n2, assign3900_e3738_d_n3, assign3900_e3738_d_n4, assign3900_e3738_d_n5, assign3900_e3738_d_b0, assign3900_e3738_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) {
        let assign3900_e3733: f64 = (1.0 - p.p68);
        let assign3900_e3735: f64 = (-p.p74);
        let assign3900_e3736: f64 = (assign3900_e3733).powf(assign3900_e3735);
        (assign3900_e3736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq, var_pwq_dn0, var_pwq_dn1, var_pwq_dn2, var_pwq_dn3, var_pwq_dn4, var_pwq_dn5, var_pwq_db0, var_pwq_db1,)
    }
};
        var_pwq = assign3900_e3738;
        var_pwq_dn0 = assign3900_e3738_d_n0;
        var_pwq_dn1 = assign3900_e3738_d_n1;
        var_pwq_dn2 = assign3900_e3738_d_n2;
        var_pwq_dn3 = assign3900_e3738_d_n3;
        var_pwq_dn4 = assign3900_e3738_d_n4;
        var_pwq_dn5 = assign3900_e3738_d_n5;
        var_pwq_db0 = assign3900_e3738_d_b0;
        var_pwq_db1 = assign3900_e3738_d_b1;
        var_pwq_rv = 0.0;
        var_pwq_rdn0 = 0.0;
        var_pwq_rdn1 = 0.0;
        var_pwq_rdn2 = 0.0;
        var_pwq_rdn3 = 0.0;
        var_pwq_rdn4 = 0.0;
        var_pwq_rdn5 = 0.0;
        var_pwq_rdb0 = 0.0;
        var_pwq_rdb1 = 0.0;

        let (assign3910_e3760, assign3910_e3760_d_n0, assign3910_e3760_d_n1, assign3910_e3760_d_n2, assign3910_e3760_d_n3, assign3910_e3760_d_n4, assign3910_e3760_d_n5, assign3910_e3760_d_b0, assign3910_e3760_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) {
        let assign3910_e3751: f64 = (1.0 - p.p68);
        let assign3910_e3752: f64 = (var_pwq * assign3910_e3751);
        let assign3910_e3753: f64 = (1.0 - assign3910_e3752);
        let assign3910_e3754: f64 = (var_pa_t * assign3910_e3753);
        let assign3910_e3757: f64 = (1.0 - p.p74);
        let assign3910_e3758: f64 = (assign3910_e3754 / assign3910_e3757);
        (assign3910_e3758, (((var_pa_t_dn0 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn0 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_dn1 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn1 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_dn2 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn2 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_dn3 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn3 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_dn4 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn4 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_dn5 * assign3910_e3753) + (var_pa_t * (-(var_pwq_dn5 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_db0 * assign3910_e3753) + (var_pa_t * (-(var_pwq_db0 * assign3910_e3751)))) / assign3910_e3757), (((var_pa_t_db1 * assign3910_e3753) + (var_pa_t * (-(var_pwq_db1 * assign3910_e3751)))) / assign3910_e3757),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_db0, var_qlo_db1,)
    }
};
        var_qlo = assign3910_e3760;
        var_qlo_dn0 = assign3910_e3760_d_n0;
        var_qlo_dn1 = assign3910_e3760_d_n1;
        var_qlo_dn2 = assign3910_e3760_d_n2;
        var_qlo_dn3 = assign3910_e3760_d_n3;
        var_qlo_dn4 = assign3910_e3760_d_n4;
        var_qlo_dn5 = assign3910_e3760_d_n5;
        var_qlo_db0 = assign3910_e3760_d_b0;
        var_qlo_db1 = assign3910_e3760_d_b1;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;

        let (assign3920_e3786, assign3920_e3786_d_n0, assign3920_e3786_d_n1, assign3920_e3786_d_n2, assign3920_e3786_d_n3, assign3920_e3786_d_n4, assign3920_e3786_d_n5, assign3920_e3786_d_b0, assign3920_e3786_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 != 0.0)) {
        let assign3920_e3772: f64 = (0.5 * p.p74);
        let assign3920_e3774: f64 = (assign3920_e3772 * var_dvh);
        let assign3920_e3778: f64 = (1.0 - p.p68);
        let assign3920_e3779: f64 = (var_pa_t * assign3920_e3778);
        let assign3920_e3780: f64 = (assign3920_e3774 / assign3920_e3779);
        let assign3920_e3781: f64 = (1.0 + assign3920_e3780);
        let assign3920_e3782: f64 = (var_dvh * assign3920_e3781);
        let assign3920_e3784: f64 = (assign3920_e3782 * var_pwq);
        (assign3920_e3784, ((((var_dvh_dn0 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn0) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn0 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn0)), ((((var_dvh_dn1 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn1) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn1 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn1)), ((((var_dvh_dn2 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn2) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn2 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn2)), ((((var_dvh_dn3 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn3) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn3 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn3)), ((((var_dvh_dn4 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn4) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn4 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn4)), ((((var_dvh_dn5 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_dn5) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_dn5 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_dn5)), ((((var_dvh_db0 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_db0) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_db0 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_db0)), ((((var_dvh_db1 * assign3920_e3781) + (var_dvh * ((((assign3920_e3772 * var_dvh_db1) * assign3920_e3779) - (assign3920_e3774 * (var_pa_t_db1 * assign3920_e3778))) / (assign3920_e3779 * assign3920_e3779)))) * var_pwq) + (assign3920_e3782 * var_pwq_db1)),)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_db0, var_qhi_db1,)
    }
};
        var_qhi = assign3920_e3786;
        var_qhi_dn0 = assign3920_e3786_d_n0;
        var_qhi_dn1 = assign3920_e3786_d_n1;
        var_qhi_dn2 = assign3920_e3786_d_n2;
        var_qhi_dn3 = assign3920_e3786_d_n3;
        var_qhi_dn4 = assign3920_e3786_d_n4;
        var_qhi_dn5 = assign3920_e3786_d_n5;
        var_qhi_db0 = assign3920_e3786_d_b0;
        var_qhi_db1 = assign3920_e3786_d_b1;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;

        let (assign3930_e3813, assign3930_e3813_d_n0, assign3930_e3813_d_n1, assign3930_e3813_d_n2, assign3930_e3813_d_n3, assign3930_e3813_d_n4, assign3930_e3813_d_n5, assign3930_e3813_d_b0, assign3930_e3813_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) {
        let assign3930_e3800: f64 = (var_vcl / var_pa_t);
        let assign3930_e3801: f64 = (1.0 - assign3930_e3800);
        let assign3930_e3804: f64 = (1.0 - p.p74);
        let assign3930_e3805: f64 = (assign3930_e3801).powf(assign3930_e3804);
        let assign3930_e3806: f64 = (1.0 - assign3930_e3805);
        let assign3930_e3807: f64 = (var_pa_t * assign3930_e3806);
        let assign3930_e3810: f64 = (1.0 - p.p74);
        let assign3930_e3811: f64 = (assign3930_e3807 / assign3930_e3810);
        (assign3930_e3811, (((var_pa_t_dn0 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn0 * var_pa_t) - (var_vcl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn0 * var_pa_t) - (var_vcl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_dn1 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn1 * var_pa_t) - (var_vcl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn1 * var_pa_t) - (var_vcl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_dn2 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn2 * var_pa_t) - (var_vcl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn2 * var_pa_t) - (var_vcl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_dn3 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn3 * var_pa_t) - (var_vcl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn3 * var_pa_t) - (var_vcl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_dn4 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn4 * var_pa_t) - (var_vcl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn4 * var_pa_t) - (var_vcl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_dn5 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_dn5 * var_pa_t) - (var_vcl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_dn5 * var_pa_t) - (var_vcl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_db0 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_db0 * var_pa_t) - (var_vcl * var_pa_t_db0)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_db0 * var_pa_t) - (var_vcl * var_pa_t_db0)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810), (((var_pa_t_db1 * assign3930_e3806) + (var_pa_t * (-if 0.0 == 0.0 && ((assign3930_e3804) as f64).is_finite() && ((assign3930_e3804) as f64).fract() == 0.0 { if assign3930_e3804 == 0.0 { 0.0 } else { (assign3930_e3804 * ((assign3930_e3801).powf(assign3930_e3804 - 1.0) * (-(((var_vcl_db1 * var_pa_t) - (var_vcl * var_pa_t_db1)) / (var_pa_t * var_pa_t))))) } } else { (assign3930_e3805 * (assign3930_e3804 * ((-(((var_vcl_db1 * var_pa_t) - (var_vcl * var_pa_t_db1)) / (var_pa_t * var_pa_t))) / assign3930_e3801))) }))) / assign3930_e3810),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_db0, var_qlo_db1,)
    }
};
        var_qlo = assign3930_e3813;
        var_qlo_dn0 = assign3930_e3813_d_n0;
        var_qlo_dn1 = assign3930_e3813_d_n1;
        var_qlo_dn2 = assign3930_e3813_d_n2;
        var_qlo_dn3 = assign3930_e3813_d_n3;
        var_qlo_dn4 = assign3930_e3813_d_n4;
        var_qlo_dn5 = assign3930_e3813_d_n5;
        var_qlo_db0 = assign3930_e3813_d_b0;
        var_qlo_db1 = assign3930_e3813_d_b1;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;

        let (assign3940_e3824, assign3940_e3824_d_n0, assign3940_e3824_d_n1, assign3940_e3824_d_n2, assign3940_e3824_d_n3, assign3940_e3824_d_n4, assign3940_e3824_d_n5, assign3940_e3824_d_b0, assign3940_e3824_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) && (var_guard266 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi, var_qhi_dn0, var_qhi_dn1, var_qhi_dn2, var_qhi_dn3, var_qhi_dn4, var_qhi_dn5, var_qhi_db0, var_qhi_db1,)
    }
};
        var_qhi = assign3940_e3824;
        var_qhi_dn0 = assign3940_e3824_d_n0;
        var_qhi_dn1 = assign3940_e3824_d_n1;
        var_qhi_dn2 = assign3940_e3824_d_n2;
        var_qhi_dn3 = assign3940_e3824_d_n3;
        var_qhi_dn4 = assign3940_e3824_d_n4;
        var_qhi_dn5 = assign3940_e3824_d_n5;
        var_qhi_db0 = assign3940_e3824_d_b0;
        var_qhi_db1 = assign3940_e3824_d_b1;
        var_qhi_rv = 0.0;
        var_qhi_rdn0 = 0.0;
        var_qhi_rdn1 = 0.0;
        var_qhi_rdn2 = 0.0;
        var_qhi_rdn3 = 0.0;
        var_qhi_rdn4 = 0.0;
        var_qhi_rdn5 = 0.0;
        var_qhi_rdb0 = 0.0;
        var_qhi_rdb1 = 0.0;

        let (assign3950_e3834, assign3950_e3834_d_n0, assign3950_e3834_d_n1, assign3950_e3834_d_n2, assign3950_e3834_d_n3, assign3950_e3834_d_n4, assign3950_e3834_d_n5, assign3950_e3834_d_b0, assign3950_e3834_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 != 0.0)) {
        let assign3950_e3832: f64 = (var_qlo + var_qhi);
        (assign3950_e3832, (var_qlo_dn0 + var_qhi_dn0), (var_qlo_dn1 + var_qhi_dn1), (var_qlo_dn2 + var_qhi_dn2), (var_qlo_dn3 + var_qhi_dn3), (var_qlo_dn4 + var_qhi_dn4), (var_qlo_dn5 + var_qhi_dn5), (var_qlo_db0 + var_qhi_db0), (var_qlo_db1 + var_qhi_db1),)
    } else {
        (var_arga, var_arga_dn0, var_arga_dn1, var_arga_dn2, var_arga_dn3, var_arga_dn4, var_arga_dn5, var_arga_db0, var_arga_db1,)
    }
};
        var_arga = assign3950_e3834;
        var_arga_dn0 = assign3950_e3834_d_n0;
        var_arga_dn1 = assign3950_e3834_d_n1;
        var_arga_dn2 = assign3950_e3834_d_n2;
        var_arga_dn3 = assign3950_e3834_d_n3;
        var_arga_dn4 = assign3950_e3834_d_n4;
        var_arga_dn5 = assign3950_e3834_d_n5;
        var_arga_db0 = assign3950_e3834_d_b0;
        var_arga_db1 = assign3950_e3834_d_b1;
        var_arga_rv = 0.0;
        var_arga_rdn0 = 0.0;
        var_arga_rdn1 = 0.0;
        var_arga_rdn2 = 0.0;
        var_arga_rdn3 = 0.0;
        var_arga_rdn4 = 0.0;
        var_arga_rdn5 = 0.0;
        var_arga_rdb0 = 0.0;
        var_arga_rdb1 = 0.0;

        let (assign3960_e3852, assign3960_e3852_d_n0, assign3960_e3852_d_n1, assign3960_e3852_d_n2, assign3960_e3852_d_n3, assign3960_e3852_d_n4, assign3960_e3852_d_n5, assign3960_e3852_d_b0, assign3960_e3852_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign3960_e3843: f64 = (var_dv0 * var_dv0);
        let assign3960_e3846: f64 = (4.0 * p.p75);
        let assign3960_e3848: f64 = (assign3960_e3846 * p.p75);
        let assign3960_e3849: f64 = (assign3960_e3843 + assign3960_e3848);
        let assign3960_e3850: f64 = (assign3960_e3849).sqrt();
        (assign3960_e3850, (((var_dv0_dn0 * var_dv0) + (var_dv0 * var_dv0_dn0)) / (2.0 * assign3960_e3850)), (((var_dv0_dn1 * var_dv0) + (var_dv0 * var_dv0_dn1)) / (2.0 * assign3960_e3850)), (((var_dv0_dn2 * var_dv0) + (var_dv0 * var_dv0_dn2)) / (2.0 * assign3960_e3850)), (((var_dv0_dn3 * var_dv0) + (var_dv0 * var_dv0_dn3)) / (2.0 * assign3960_e3850)), (((var_dv0_dn4 * var_dv0) + (var_dv0 * var_dv0_dn4)) / (2.0 * assign3960_e3850)), (((var_dv0_dn5 * var_dv0) + (var_dv0 * var_dv0_dn5)) / (2.0 * assign3960_e3850)), (((var_dv0_db0 * var_dv0) + (var_dv0 * var_dv0_db0)) / (2.0 * assign3960_e3850)), (((var_dv0_db1 * var_dv0) + (var_dv0 * var_dv0_db1)) / (2.0 * assign3960_e3850)),)
    } else {
        (var_mv0, var_mv0_dn0, var_mv0_dn1, var_mv0_dn2, var_mv0_dn3, var_mv0_dn4, var_mv0_dn5, var_mv0_db0, var_mv0_db1,)
    }
};
        var_mv0 = assign3960_e3852;
        var_mv0_dn0 = assign3960_e3852_d_n0;
        var_mv0_dn1 = assign3960_e3852_d_n1;
        var_mv0_dn2 = assign3960_e3852_d_n2;
        var_mv0_dn3 = assign3960_e3852_d_n3;
        var_mv0_dn4 = assign3960_e3852_d_n4;
        var_mv0_dn5 = assign3960_e3852_d_n5;
        var_mv0_db0 = assign3960_e3852_d_b0;
        var_mv0_db1 = assign3960_e3852_d_b1;
        var_mv0_rv = 0.0;
        var_mv0_rdn0 = 0.0;
        var_mv0_rdn1 = 0.0;
        var_mv0_rdn2 = 0.0;
        var_mv0_rdn3 = 0.0;
        var_mv0_rdn4 = 0.0;
        var_mv0_rdn5 = 0.0;
        var_mv0_rdb0 = 0.0;
        var_mv0_rdb1 = 0.0;

        let (assign3970_e3866, assign3970_e3866_d_n0, assign3970_e3866_d_n1, assign3970_e3866_d_n2, assign3970_e3866_d_n3, assign3970_e3866_d_n4, assign3970_e3866_d_n5, assign3970_e3866_d_b0, assign3970_e3866_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign3970_e3860: f64 = (-0.5);
        let assign3970_e3863: f64 = (var_dv0 + var_mv0);
        let assign3970_e3864: f64 = (assign3970_e3860 * assign3970_e3863);
        (assign3970_e3864, (assign3970_e3860 * (var_dv0_dn0 + var_mv0_dn0)), (assign3970_e3860 * (var_dv0_dn1 + var_mv0_dn1)), (assign3970_e3860 * (var_dv0_dn2 + var_mv0_dn2)), (assign3970_e3860 * (var_dv0_dn3 + var_mv0_dn3)), (assign3970_e3860 * (var_dv0_dn4 + var_mv0_dn4)), (assign3970_e3860 * (var_dv0_dn5 + var_mv0_dn5)), (assign3970_e3860 * (var_dv0_db0 + var_mv0_db0)), (assign3970_e3860 * (var_dv0_db1 + var_mv0_db1)),)
    } else {
        (var_vl0, var_vl0_dn0, var_vl0_dn1, var_vl0_dn2, var_vl0_dn3, var_vl0_dn4, var_vl0_dn5, var_vl0_db0, var_vl0_db1,)
    }
};
        var_vl0 = assign3970_e3866;
        var_vl0_dn0 = assign3970_e3866_d_n0;
        var_vl0_dn1 = assign3970_e3866_d_n1;
        var_vl0_dn2 = assign3970_e3866_d_n2;
        var_vl0_dn3 = assign3970_e3866_d_n3;
        var_vl0_dn4 = assign3970_e3866_d_n4;
        var_vl0_dn5 = assign3970_e3866_d_n5;
        var_vl0_db0 = assign3970_e3866_d_b0;
        var_vl0_db1 = assign3970_e3866_d_b1;
        var_vl0_rv = 0.0;
        var_vl0_rdn0 = 0.0;
        var_vl0_rdn1 = 0.0;
        var_vl0_rdn2 = 0.0;
        var_vl0_rdn3 = 0.0;
        var_vl0_rdn4 = 0.0;
        var_vl0_rdn5 = 0.0;
        var_vl0_rdb0 = 0.0;
        var_vl0_rdb1 = 0.0;

        let (assign3980_e3877, assign3980_e3877_d_n0, assign3980_e3877_d_n1, assign3980_e3877_d_n2, assign3980_e3877_d_n3, assign3980_e3877_d_n4, assign3980_e3877_d_n5, assign3980_e3877_d_b0, assign3980_e3877_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign3980_e3875: f64 = (var_vcl + var_dv0);
        (assign3980_e3875, (var_vcl_dn0 + var_dv0_dn0), (var_vcl_dn1 + var_dv0_dn1), (var_vcl_dn2 + var_dv0_dn2), (var_vcl_dn3 + var_dv0_dn3), (var_vcl_dn4 + var_dv0_dn4), (var_vcl_dn5 + var_dv0_dn5), (var_vcl_db0 + var_dv0_db0), (var_vcl_db1 + var_dv0_db1),)
    } else {
        (var_dv, var_dv_dn0, var_dv_dn1, var_dv_dn2, var_dv_dn3, var_dv_dn4, var_dv_dn5, var_dv_db0, var_dv_db1,)
    }
};
        var_dv = assign3980_e3877;
        var_dv_dn0 = assign3980_e3877_d_n0;
        var_dv_dn1 = assign3980_e3877_d_n1;
        var_dv_dn2 = assign3980_e3877_d_n2;
        var_dv_dn3 = assign3980_e3877_d_n3;
        var_dv_dn4 = assign3980_e3877_d_n4;
        var_dv_dn5 = assign3980_e3877_d_n5;
        var_dv_db0 = assign3980_e3877_d_b0;
        var_dv_db1 = assign3980_e3877_d_b1;
        var_dv_rv = 0.0;
        var_dv_rdn0 = 0.0;
        var_dv_rdn1 = 0.0;
        var_dv_rdn2 = 0.0;
        var_dv_rdn3 = 0.0;
        var_dv_rdn4 = 0.0;
        var_dv_rdn5 = 0.0;
        var_dv_rdb0 = 0.0;
        var_dv_rdb1 = 0.0;

        *var_acja_slot = var_acja;
        *var_acja_db0_slot = var_acja_db0;
        *var_acja_db1_slot = var_acja_db1;
        *var_acja_dn0_slot = var_acja_dn0;
        *var_acja_dn1_slot = var_acja_dn1;
        *var_acja_dn2_slot = var_acja_dn2;
        *var_acja_dn3_slot = var_acja_dn3;
        *var_acja_dn4_slot = var_acja_dn4;
        *var_acja_dn5_slot = var_acja_dn5;
        *var_acja_rdb0_slot = var_acja_rdb0;
        *var_acja_rdb1_slot = var_acja_rdb1;
        *var_acja_rdn0_slot = var_acja_rdn0;
        *var_acja_rdn1_slot = var_acja_rdn1;
        *var_acja_rdn2_slot = var_acja_rdn2;
        *var_acja_rdn3_slot = var_acja_rdn3;
        *var_acja_rdn4_slot = var_acja_rdn4;
        *var_acja_rdn5_slot = var_acja_rdn5;
        *var_acja_rv_slot = var_acja_rv;
        *var_arga_slot = var_arga;
        *var_arga_db0_slot = var_arga_db0;
        *var_arga_db1_slot = var_arga_db1;
        *var_arga_dn0_slot = var_arga_dn0;
        *var_arga_dn1_slot = var_arga_dn1;
        *var_arga_dn2_slot = var_arga_dn2;
        *var_arga_dn3_slot = var_arga_dn3;
        *var_arga_dn4_slot = var_arga_dn4;
        *var_arga_dn5_slot = var_arga_dn5;
        *var_arga_rdb0_slot = var_arga_rdb0;
        *var_arga_rdb1_slot = var_arga_rdb1;
        *var_arga_rdn0_slot = var_arga_rdn0;
        *var_arga_rdn1_slot = var_arga_rdn1;
        *var_arga_rdn2_slot = var_arga_rdn2;
        *var_arga_rdn3_slot = var_arga_rdn3;
        *var_arga_rdn4_slot = var_arga_rdn4;
        *var_arga_rdn5_slot = var_arga_rdn5;
        *var_arga_rv_slot = var_arga_rv;
        *var_dv_slot = var_dv;
        *var_dv0_slot = var_dv0;
        *var_dv0_db0_slot = var_dv0_db0;
        *var_dv0_db1_slot = var_dv0_db1;
        *var_dv0_dn0_slot = var_dv0_dn0;
        *var_dv0_dn1_slot = var_dv0_dn1;
        *var_dv0_dn2_slot = var_dv0_dn2;
        *var_dv0_dn3_slot = var_dv0_dn3;
        *var_dv0_dn4_slot = var_dv0_dn4;
        *var_dv0_dn5_slot = var_dv0_dn5;
        *var_dv0_rdb0_slot = var_dv0_rdb0;
        *var_dv0_rdb1_slot = var_dv0_rdb1;
        *var_dv0_rdn0_slot = var_dv0_rdn0;
        *var_dv0_rdn1_slot = var_dv0_rdn1;
        *var_dv0_rdn2_slot = var_dv0_rdn2;
        *var_dv0_rdn3_slot = var_dv0_rdn3;
        *var_dv0_rdn4_slot = var_dv0_rdn4;
        *var_dv0_rdn5_slot = var_dv0_rdn5;
        *var_dv0_rv_slot = var_dv0_rv;
        *var_dv_db0_slot = var_dv_db0;
        *var_dv_db1_slot = var_dv_db1;
        *var_dv_dn0_slot = var_dv_dn0;
        *var_dv_dn1_slot = var_dv_dn1;
        *var_dv_dn2_slot = var_dv_dn2;
        *var_dv_dn3_slot = var_dv_dn3;
        *var_dv_dn4_slot = var_dv_dn4;
        *var_dv_dn5_slot = var_dv_dn5;
        *var_dv_rdb0_slot = var_dv_rdb0;
        *var_dv_rdb1_slot = var_dv_rdb1;
        *var_dv_rdn0_slot = var_dv_rdn0;
        *var_dv_rdn1_slot = var_dv_rdn1;
        *var_dv_rdn2_slot = var_dv_rdn2;
        *var_dv_rdn3_slot = var_dv_rdn3;
        *var_dv_rdn4_slot = var_dv_rdn4;
        *var_dv_rdn5_slot = var_dv_rdn5;
        *var_dv_rv_slot = var_dv_rv;
        *var_dvh_slot = var_dvh;
        *var_dvh_db0_slot = var_dvh_db0;
        *var_dvh_db1_slot = var_dvh_db1;
        *var_dvh_dn0_slot = var_dvh_dn0;
        *var_dvh_dn1_slot = var_dvh_dn1;
        *var_dvh_dn2_slot = var_dvh_dn2;
        *var_dvh_dn3_slot = var_dvh_dn3;
        *var_dvh_dn4_slot = var_dvh_dn4;
        *var_dvh_dn5_slot = var_dvh_dn5;
        *var_dvh_rdb0_slot = var_dvh_rdb0;
        *var_dvh_rdb1_slot = var_dvh_rdb1;
        *var_dvh_rdn0_slot = var_dvh_rdn0;
        *var_dvh_rdn1_slot = var_dvh_rdn1;
        *var_dvh_rdn2_slot = var_dvh_rdn2;
        *var_dvh_rdn3_slot = var_dvh_rdn3;
        *var_dvh_rdn4_slot = var_dvh_rdn4;
        *var_dvh_rdn5_slot = var_dvh_rdn5;
        *var_dvh_rv_slot = var_dvh_rv;
        *var_guard254_slot = var_guard254;
        *var_guard254_db0_slot = var_guard254_db0;
        *var_guard254_db1_slot = var_guard254_db1;
        *var_guard254_dn0_slot = var_guard254_dn0;
        *var_guard254_dn1_slot = var_guard254_dn1;
        *var_guard254_dn2_slot = var_guard254_dn2;
        *var_guard254_dn3_slot = var_guard254_dn3;
        *var_guard254_dn4_slot = var_guard254_dn4;
        *var_guard254_dn5_slot = var_guard254_dn5;
        *var_guard254_rdb0_slot = var_guard254_rdb0;
        *var_guard254_rdb1_slot = var_guard254_rdb1;
        *var_guard254_rdn0_slot = var_guard254_rdn0;
        *var_guard254_rdn1_slot = var_guard254_rdn1;
        *var_guard254_rdn2_slot = var_guard254_rdn2;
        *var_guard254_rdn3_slot = var_guard254_rdn3;
        *var_guard254_rdn4_slot = var_guard254_rdn4;
        *var_guard254_rdn5_slot = var_guard254_rdn5;
        *var_guard254_rv_slot = var_guard254_rv;
        *var_guard265_slot = var_guard265;
        *var_guard265_db0_slot = var_guard265_db0;
        *var_guard265_db1_slot = var_guard265_db1;
        *var_guard265_dn0_slot = var_guard265_dn0;
        *var_guard265_dn1_slot = var_guard265_dn1;
        *var_guard265_dn2_slot = var_guard265_dn2;
        *var_guard265_dn3_slot = var_guard265_dn3;
        *var_guard265_dn4_slot = var_guard265_dn4;
        *var_guard265_dn5_slot = var_guard265_dn5;
        *var_guard265_rdb0_slot = var_guard265_rdb0;
        *var_guard265_rdb1_slot = var_guard265_rdb1;
        *var_guard265_rdn0_slot = var_guard265_rdn0;
        *var_guard265_rdn1_slot = var_guard265_rdn1;
        *var_guard265_rdn2_slot = var_guard265_rdn2;
        *var_guard265_rdn3_slot = var_guard265_rdn3;
        *var_guard265_rdn4_slot = var_guard265_rdn4;
        *var_guard265_rdn5_slot = var_guard265_rdn5;
        *var_guard265_rv_slot = var_guard265_rv;
        *var_guard266_slot = var_guard266;
        *var_guard266_db0_slot = var_guard266_db0;
        *var_guard266_db1_slot = var_guard266_db1;
        *var_guard266_dn0_slot = var_guard266_dn0;
        *var_guard266_dn1_slot = var_guard266_dn1;
        *var_guard266_dn2_slot = var_guard266_dn2;
        *var_guard266_dn3_slot = var_guard266_dn3;
        *var_guard266_dn4_slot = var_guard266_dn4;
        *var_guard266_dn5_slot = var_guard266_dn5;
        *var_guard266_rdb0_slot = var_guard266_rdb0;
        *var_guard266_rdb1_slot = var_guard266_rdb1;
        *var_guard266_rdn0_slot = var_guard266_rdn0;
        *var_guard266_rdn1_slot = var_guard266_rdn1;
        *var_guard266_rdn2_slot = var_guard266_rdn2;
        *var_guard266_rdn3_slot = var_guard266_rdn3;
        *var_guard266_rdn4_slot = var_guard266_rdn4;
        *var_guard266_rdn5_slot = var_guard266_rdn5;
        *var_guard266_rv_slot = var_guard266_rv;
        *var_mv0_slot = var_mv0;
        *var_mv0_db0_slot = var_mv0_db0;
        *var_mv0_db1_slot = var_mv0_db1;
        *var_mv0_dn0_slot = var_mv0_dn0;
        *var_mv0_dn1_slot = var_mv0_dn1;
        *var_mv0_dn2_slot = var_mv0_dn2;
        *var_mv0_dn3_slot = var_mv0_dn3;
        *var_mv0_dn4_slot = var_mv0_dn4;
        *var_mv0_dn5_slot = var_mv0_dn5;
        *var_mv0_rdb0_slot = var_mv0_rdb0;
        *var_mv0_rdb1_slot = var_mv0_rdb1;
        *var_mv0_rdn0_slot = var_mv0_rdn0;
        *var_mv0_rdn1_slot = var_mv0_rdn1;
        *var_mv0_rdn2_slot = var_mv0_rdn2;
        *var_mv0_rdn3_slot = var_mv0_rdn3;
        *var_mv0_rdn4_slot = var_mv0_rdn4;
        *var_mv0_rdn5_slot = var_mv0_rdn5;
        *var_mv0_rv_slot = var_mv0_rv;
        *var_pcjp_slot = var_pcjp;
        *var_pcjp_db0_slot = var_pcjp_db0;
        *var_pcjp_db1_slot = var_pcjp_db1;
        *var_pcjp_dn0_slot = var_pcjp_dn0;
        *var_pcjp_dn1_slot = var_pcjp_dn1;
        *var_pcjp_dn2_slot = var_pcjp_dn2;
        *var_pcjp_dn3_slot = var_pcjp_dn3;
        *var_pcjp_dn4_slot = var_pcjp_dn4;
        *var_pcjp_dn5_slot = var_pcjp_dn5;
        *var_pcjp_rdb0_slot = var_pcjp_rdb0;
        *var_pcjp_rdb1_slot = var_pcjp_rdb1;
        *var_pcjp_rdn0_slot = var_pcjp_rdn0;
        *var_pcjp_rdn1_slot = var_pcjp_rdn1;
        *var_pcjp_rdn2_slot = var_pcjp_rdn2;
        *var_pcjp_rdn3_slot = var_pcjp_rdn3;
        *var_pcjp_rdn4_slot = var_pcjp_rdn4;
        *var_pcjp_rdn5_slot = var_pcjp_rdn5;
        *var_pcjp_rv_slot = var_pcjp_rv;
        *var_pwq_slot = var_pwq;
        *var_pwq_db0_slot = var_pwq_db0;
        *var_pwq_db1_slot = var_pwq_db1;
        *var_pwq_dn0_slot = var_pwq_dn0;
        *var_pwq_dn1_slot = var_pwq_dn1;
        *var_pwq_dn2_slot = var_pwq_dn2;
        *var_pwq_dn3_slot = var_pwq_dn3;
        *var_pwq_dn4_slot = var_pwq_dn4;
        *var_pwq_dn5_slot = var_pwq_dn5;
        *var_pwq_rdb0_slot = var_pwq_rdb0;
        *var_pwq_rdb1_slot = var_pwq_rdb1;
        *var_pwq_rdn0_slot = var_pwq_rdn0;
        *var_pwq_rdn1_slot = var_pwq_rdn1;
        *var_pwq_rdn2_slot = var_pwq_rdn2;
        *var_pwq_rdn3_slot = var_pwq_rdn3;
        *var_pwq_rdn4_slot = var_pwq_rdn4;
        *var_pwq_rdn5_slot = var_pwq_rdn5;
        *var_pwq_rv_slot = var_pwq_rv;
        *var_qhi_slot = var_qhi;
        *var_qhi_db0_slot = var_qhi_db0;
        *var_qhi_db1_slot = var_qhi_db1;
        *var_qhi_dn0_slot = var_qhi_dn0;
        *var_qhi_dn1_slot = var_qhi_dn1;
        *var_qhi_dn2_slot = var_qhi_dn2;
        *var_qhi_dn3_slot = var_qhi_dn3;
        *var_qhi_dn4_slot = var_qhi_dn4;
        *var_qhi_dn5_slot = var_qhi_dn5;
        *var_qhi_rdb0_slot = var_qhi_rdb0;
        *var_qhi_rdb1_slot = var_qhi_rdb1;
        *var_qhi_rdn0_slot = var_qhi_rdn0;
        *var_qhi_rdn1_slot = var_qhi_rdn1;
        *var_qhi_rdn2_slot = var_qhi_rdn2;
        *var_qhi_rdn3_slot = var_qhi_rdn3;
        *var_qhi_rdn4_slot = var_qhi_rdn4;
        *var_qhi_rdn5_slot = var_qhi_rdn5;
        *var_qhi_rv_slot = var_qhi_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rv_slot = var_qlo_rv;
        *var_vcl_slot = var_vcl;
        *var_vcl_db0_slot = var_vcl_db0;
        *var_vcl_db1_slot = var_vcl_db1;
        *var_vcl_dn0_slot = var_vcl_dn0;
        *var_vcl_dn1_slot = var_vcl_dn1;
        *var_vcl_dn2_slot = var_vcl_dn2;
        *var_vcl_dn3_slot = var_vcl_dn3;
        *var_vcl_dn4_slot = var_vcl_dn4;
        *var_vcl_dn5_slot = var_vcl_dn5;
        *var_vcl_rdb0_slot = var_vcl_rdb0;
        *var_vcl_rdb1_slot = var_vcl_rdb1;
        *var_vcl_rdn0_slot = var_vcl_rdn0;
        *var_vcl_rdn1_slot = var_vcl_rdn1;
        *var_vcl_rdn2_slot = var_vcl_rdn2;
        *var_vcl_rdn3_slot = var_vcl_rdn3;
        *var_vcl_rdn4_slot = var_vcl_rdn4;
        *var_vcl_rdn5_slot = var_vcl_rdn5;
        *var_vcl_rv_slot = var_vcl_rv;
        *var_vl0_slot = var_vl0;
        *var_vl0_db0_slot = var_vl0_db0;
        *var_vl0_db1_slot = var_vl0_db1;
        *var_vl0_dn0_slot = var_vl0_dn0;
        *var_vl0_dn1_slot = var_vl0_dn1;
        *var_vl0_dn2_slot = var_vl0_dn2;
        *var_vl0_dn3_slot = var_vl0_dn3;
        *var_vl0_dn4_slot = var_vl0_dn4;
        *var_vl0_dn5_slot = var_vl0_dn5;
        *var_vl0_rdb0_slot = var_vl0_rdb0;
        *var_vl0_rdb1_slot = var_vl0_rdb1;
        *var_vl0_rdn0_slot = var_vl0_rdn0;
        *var_vl0_rdn1_slot = var_vl0_rdn1;
        *var_vl0_rdn2_slot = var_vl0_rdn2;
        *var_vl0_rdn3_slot = var_vl0_rdn3;
        *var_vl0_rdn4_slot = var_vl0_rdn4;
        *var_vl0_rdn5_slot = var_vl0_rdn5;
        *var_vl0_rv_slot = var_vl0_rv;
    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        var_dv: f64,
        var_dv0: f64,
        var_dv0_db0: f64,
        var_dv0_db1: f64,
        var_dv0_dn0: f64,
        var_dv0_dn1: f64,
        var_dv0_dn2: f64,
        var_dv0_dn3: f64,
        var_dv0_dn4: f64,
        var_dv0_dn5: f64,
        var_dv_db0: f64,
        var_dv_db1: f64,
        var_dv_dn0: f64,
        var_dv_dn1: f64,
        var_dv_dn2: f64,
        var_dv_dn3: f64,
        var_dv_dn4: f64,
        var_dv_dn5: f64,
        var_guard249: f64,
        var_guard254: f64,
        var_guard265: f64,
        var_pa_t: f64,
        var_pa_t_db0: f64,
        var_pa_t_db1: f64,
        var_pa_t_dn0: f64,
        var_pa_t_dn1: f64,
        var_pa_t_dn2: f64,
        var_pa_t_dn3: f64,
        var_pa_t_dn4: f64,
        var_pa_t_dn5: f64,
        var_pcjp: f64,
        var_pp_t: f64,
        var_pp_t_db0: f64,
        var_pp_t_db1: f64,
        var_pp_t_dn0: f64,
        var_pp_t_dn1: f64,
        var_pp_t_dn2: f64,
        var_pp_t_dn3: f64,
        var_pp_t_dn4: f64,
        var_pp_t_dn5: f64,
        var_vcl: f64,
        var_vcl_db0: f64,
        var_vcl_db1: f64,
        var_vcl_dn0: f64,
        var_vcl_dn1: f64,
        var_vcl_dn2: f64,
        var_vcl_dn3: f64,
        var_vcl_dn4: f64,
        var_vcl_dn5: f64,
        var_vl0: f64,
        var_vl0_db0: f64,
        var_vl0_db1: f64,
        var_vl0_dn0: f64,
        var_vl0_dn1: f64,
        var_vl0_dn2: f64,
        var_vl0_dn3: f64,
        var_vl0_dn4: f64,
        var_vl0_dn5: f64,
        var_arga_slot: &mut f64,
        var_arga_db0_slot: &mut f64,
        var_arga_db1_slot: &mut f64,
        var_arga_dn0_slot: &mut f64,
        var_arga_dn1_slot: &mut f64,
        var_arga_dn2_slot: &mut f64,
        var_arga_dn3_slot: &mut f64,
        var_arga_dn4_slot: &mut f64,
        var_arga_dn5_slot: &mut f64,
        var_arga_rdb0_slot: &mut f64,
        var_arga_rdb1_slot: &mut f64,
        var_arga_rdn0_slot: &mut f64,
        var_arga_rdn1_slot: &mut f64,
        var_arga_rdn2_slot: &mut f64,
        var_arga_rdn3_slot: &mut f64,
        var_arga_rdn4_slot: &mut f64,
        var_arga_rdn5_slot: &mut f64,
        var_arga_rv_slot: &mut f64,
        var_argp_slot: &mut f64,
        var_argp_db0_slot: &mut f64,
        var_argp_db1_slot: &mut f64,
        var_argp_dn0_slot: &mut f64,
        var_argp_dn1_slot: &mut f64,
        var_argp_dn2_slot: &mut f64,
        var_argp_dn3_slot: &mut f64,
        var_argp_dn4_slot: &mut f64,
        var_argp_dn5_slot: &mut f64,
        var_argp_rdb0_slot: &mut f64,
        var_argp_rdb1_slot: &mut f64,
        var_argp_rdn0_slot: &mut f64,
        var_argp_rdn1_slot: &mut f64,
        var_argp_rdn2_slot: &mut f64,
        var_argp_rdn3_slot: &mut f64,
        var_argp_rdn4_slot: &mut f64,
        var_argp_rdn5_slot: &mut f64,
        var_argp_rv_slot: &mut f64,
        var_dv0__blk268_slot: &mut f64,
        var_dv0__blk268_db0_slot: &mut f64,
        var_dv0__blk268_db1_slot: &mut f64,
        var_dv0__blk268_dn0_slot: &mut f64,
        var_dv0__blk268_dn1_slot: &mut f64,
        var_dv0__blk268_dn2_slot: &mut f64,
        var_dv0__blk268_dn3_slot: &mut f64,
        var_dv0__blk268_dn4_slot: &mut f64,
        var_dv0__blk268_dn5_slot: &mut f64,
        var_dv0__blk268_rdb0_slot: &mut f64,
        var_dv0__blk268_rdb1_slot: &mut f64,
        var_dv0__blk268_rdn0_slot: &mut f64,
        var_dv0__blk268_rdn1_slot: &mut f64,
        var_dv0__blk268_rdn2_slot: &mut f64,
        var_dv0__blk268_rdn3_slot: &mut f64,
        var_dv0__blk268_rdn4_slot: &mut f64,
        var_dv0__blk268_rdn5_slot: &mut f64,
        var_dv0__blk268_rv_slot: &mut f64,
        var_dvh__blk269_slot: &mut f64,
        var_dvh__blk269_db0_slot: &mut f64,
        var_dvh__blk269_db1_slot: &mut f64,
        var_dvh__blk269_dn0_slot: &mut f64,
        var_dvh__blk269_dn1_slot: &mut f64,
        var_dvh__blk269_dn2_slot: &mut f64,
        var_dvh__blk269_dn3_slot: &mut f64,
        var_dvh__blk269_dn4_slot: &mut f64,
        var_dvh__blk269_dn5_slot: &mut f64,
        var_dvh__blk269_rdb0_slot: &mut f64,
        var_dvh__blk269_rdb1_slot: &mut f64,
        var_dvh__blk269_rdn0_slot: &mut f64,
        var_dvh__blk269_rdn1_slot: &mut f64,
        var_dvh__blk269_rdn2_slot: &mut f64,
        var_dvh__blk269_rdn3_slot: &mut f64,
        var_dvh__blk269_rdn4_slot: &mut f64,
        var_dvh__blk269_rdn5_slot: &mut f64,
        var_dvh__blk269_rv_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard267_db0_slot: &mut f64,
        var_guard267_db1_slot: &mut f64,
        var_guard267_dn0_slot: &mut f64,
        var_guard267_dn1_slot: &mut f64,
        var_guard267_dn2_slot: &mut f64,
        var_guard267_dn3_slot: &mut f64,
        var_guard267_dn4_slot: &mut f64,
        var_guard267_dn5_slot: &mut f64,
        var_guard267_rdb0_slot: &mut f64,
        var_guard267_rdb1_slot: &mut f64,
        var_guard267_rdn0_slot: &mut f64,
        var_guard267_rdn1_slot: &mut f64,
        var_guard267_rdn2_slot: &mut f64,
        var_guard267_rdn3_slot: &mut f64,
        var_guard267_rdn4_slot: &mut f64,
        var_guard267_rdn5_slot: &mut f64,
        var_guard267_rv_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard278_db0_slot: &mut f64,
        var_guard278_db1_slot: &mut f64,
        var_guard278_dn0_slot: &mut f64,
        var_guard278_dn1_slot: &mut f64,
        var_guard278_dn2_slot: &mut f64,
        var_guard278_dn3_slot: &mut f64,
        var_guard278_dn4_slot: &mut f64,
        var_guard278_dn5_slot: &mut f64,
        var_guard278_rdb0_slot: &mut f64,
        var_guard278_rdb1_slot: &mut f64,
        var_guard278_rdn0_slot: &mut f64,
        var_guard278_rdn1_slot: &mut f64,
        var_guard278_rdn2_slot: &mut f64,
        var_guard278_rdn3_slot: &mut f64,
        var_guard278_rdn4_slot: &mut f64,
        var_guard278_rdn5_slot: &mut f64,
        var_guard278_rv_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard279_db0_slot: &mut f64,
        var_guard279_db1_slot: &mut f64,
        var_guard279_dn0_slot: &mut f64,
        var_guard279_dn1_slot: &mut f64,
        var_guard279_dn2_slot: &mut f64,
        var_guard279_dn3_slot: &mut f64,
        var_guard279_dn4_slot: &mut f64,
        var_guard279_dn5_slot: &mut f64,
        var_guard279_rdb0_slot: &mut f64,
        var_guard279_rdb1_slot: &mut f64,
        var_guard279_rdn0_slot: &mut f64,
        var_guard279_rdn1_slot: &mut f64,
        var_guard279_rdn2_slot: &mut f64,
        var_guard279_rdn3_slot: &mut f64,
        var_guard279_rdn4_slot: &mut f64,
        var_guard279_rdn5_slot: &mut f64,
        var_guard279_rv_slot: &mut f64,
        var_mv_slot: &mut f64,
        var_mv0__blk273_slot: &mut f64,
        var_mv0__blk273_db0_slot: &mut f64,
        var_mv0__blk273_db1_slot: &mut f64,
        var_mv0__blk273_dn0_slot: &mut f64,
        var_mv0__blk273_dn1_slot: &mut f64,
        var_mv0__blk273_dn2_slot: &mut f64,
        var_mv0__blk273_dn3_slot: &mut f64,
        var_mv0__blk273_dn4_slot: &mut f64,
        var_mv0__blk273_dn5_slot: &mut f64,
        var_mv0__blk273_rdb0_slot: &mut f64,
        var_mv0__blk273_rdb1_slot: &mut f64,
        var_mv0__blk273_rdn0_slot: &mut f64,
        var_mv0__blk273_rdn1_slot: &mut f64,
        var_mv0__blk273_rdn2_slot: &mut f64,
        var_mv0__blk273_rdn3_slot: &mut f64,
        var_mv0__blk273_rdn4_slot: &mut f64,
        var_mv0__blk273_rdn5_slot: &mut f64,
        var_mv0__blk273_rv_slot: &mut f64,
        var_mv_db0_slot: &mut f64,
        var_mv_db1_slot: &mut f64,
        var_mv_dn0_slot: &mut f64,
        var_mv_dn1_slot: &mut f64,
        var_mv_dn2_slot: &mut f64,
        var_mv_dn3_slot: &mut f64,
        var_mv_dn4_slot: &mut f64,
        var_mv_dn5_slot: &mut f64,
        var_mv_rdb0_slot: &mut f64,
        var_mv_rdb1_slot: &mut f64,
        var_mv_rdn0_slot: &mut f64,
        var_mv_rdn1_slot: &mut f64,
        var_mv_rdn2_slot: &mut f64,
        var_mv_rdn3_slot: &mut f64,
        var_mv_rdn4_slot: &mut f64,
        var_mv_rdn5_slot: &mut f64,
        var_mv_rv_slot: &mut f64,
        var_pwq__blk270_slot: &mut f64,
        var_pwq__blk270_db0_slot: &mut f64,
        var_pwq__blk270_db1_slot: &mut f64,
        var_pwq__blk270_dn0_slot: &mut f64,
        var_pwq__blk270_dn1_slot: &mut f64,
        var_pwq__blk270_dn2_slot: &mut f64,
        var_pwq__blk270_dn3_slot: &mut f64,
        var_pwq__blk270_dn4_slot: &mut f64,
        var_pwq__blk270_dn5_slot: &mut f64,
        var_pwq__blk270_rdb0_slot: &mut f64,
        var_pwq__blk270_rdb1_slot: &mut f64,
        var_pwq__blk270_rdn0_slot: &mut f64,
        var_pwq__blk270_rdn1_slot: &mut f64,
        var_pwq__blk270_rdn2_slot: &mut f64,
        var_pwq__blk270_rdn3_slot: &mut f64,
        var_pwq__blk270_rdn4_slot: &mut f64,
        var_pwq__blk270_rdn5_slot: &mut f64,
        var_pwq__blk270_rv_slot: &mut f64,
        var_qhi__blk272_slot: &mut f64,
        var_qhi__blk272_db0_slot: &mut f64,
        var_qhi__blk272_db1_slot: &mut f64,
        var_qhi__blk272_dn0_slot: &mut f64,
        var_qhi__blk272_dn1_slot: &mut f64,
        var_qhi__blk272_dn2_slot: &mut f64,
        var_qhi__blk272_dn3_slot: &mut f64,
        var_qhi__blk272_dn4_slot: &mut f64,
        var_qhi__blk272_dn5_slot: &mut f64,
        var_qhi__blk272_rdb0_slot: &mut f64,
        var_qhi__blk272_rdb1_slot: &mut f64,
        var_qhi__blk272_rdn0_slot: &mut f64,
        var_qhi__blk272_rdn1_slot: &mut f64,
        var_qhi__blk272_rdn2_slot: &mut f64,
        var_qhi__blk272_rdn3_slot: &mut f64,
        var_qhi__blk272_rdn4_slot: &mut f64,
        var_qhi__blk272_rdn5_slot: &mut f64,
        var_qhi__blk272_rv_slot: &mut f64,
        var_qlo_slot: &mut f64,
        var_qlo__blk271_slot: &mut f64,
        var_qlo__blk271_db0_slot: &mut f64,
        var_qlo__blk271_db1_slot: &mut f64,
        var_qlo__blk271_dn0_slot: &mut f64,
        var_qlo__blk271_dn1_slot: &mut f64,
        var_qlo__blk271_dn2_slot: &mut f64,
        var_qlo__blk271_dn3_slot: &mut f64,
        var_qlo__blk271_dn4_slot: &mut f64,
        var_qlo__blk271_dn5_slot: &mut f64,
        var_qlo__blk271_rdb0_slot: &mut f64,
        var_qlo__blk271_rdb1_slot: &mut f64,
        var_qlo__blk271_rdn0_slot: &mut f64,
        var_qlo__blk271_rdn1_slot: &mut f64,
        var_qlo__blk271_rdn2_slot: &mut f64,
        var_qlo__blk271_rdn3_slot: &mut f64,
        var_qlo__blk271_rdn4_slot: &mut f64,
        var_qlo__blk271_rdn5_slot: &mut f64,
        var_qlo__blk271_rv_slot: &mut f64,
        var_qlo_db0_slot: &mut f64,
        var_qlo_db1_slot: &mut f64,
        var_qlo_dn0_slot: &mut f64,
        var_qlo_dn1_slot: &mut f64,
        var_qlo_dn2_slot: &mut f64,
        var_qlo_dn3_slot: &mut f64,
        var_qlo_dn4_slot: &mut f64,
        var_qlo_dn5_slot: &mut f64,
        var_qlo_rdb0_slot: &mut f64,
        var_qlo_rdb1_slot: &mut f64,
        var_qlo_rdn0_slot: &mut f64,
        var_qlo_rdn1_slot: &mut f64,
        var_qlo_rdn2_slot: &mut f64,
        var_qlo_rdn3_slot: &mut f64,
        var_qlo_rdn4_slot: &mut f64,
        var_qlo_rdn5_slot: &mut f64,
        var_qlo_rv_slot: &mut f64,
        var_vl_slot: &mut f64,
        var_vl_db0_slot: &mut f64,
        var_vl_db1_slot: &mut f64,
        var_vl_dn0_slot: &mut f64,
        var_vl_dn1_slot: &mut f64,
        var_vl_dn2_slot: &mut f64,
        var_vl_dn3_slot: &mut f64,
        var_vl_dn4_slot: &mut f64,
        var_vl_dn5_slot: &mut f64,
        var_vl_rdb0_slot: &mut f64,
        var_vl_rdb1_slot: &mut f64,
        var_vl_rdn0_slot: &mut f64,
        var_vl_rdn1_slot: &mut f64,
        var_vl_rdn2_slot: &mut f64,
        var_vl_rdn3_slot: &mut f64,
        var_vl_rdn4_slot: &mut f64,
        var_vl_rdn5_slot: &mut f64,
        var_vl_rv_slot: &mut f64,
    ) {
        let mut var_arga: f64 = *var_arga_slot;
        let mut var_arga_db0: f64 = *var_arga_db0_slot;
        let mut var_arga_db1: f64 = *var_arga_db1_slot;
        let mut var_arga_dn0: f64 = *var_arga_dn0_slot;
        let mut var_arga_dn1: f64 = *var_arga_dn1_slot;
        let mut var_arga_dn2: f64 = *var_arga_dn2_slot;
        let mut var_arga_dn3: f64 = *var_arga_dn3_slot;
        let mut var_arga_dn4: f64 = *var_arga_dn4_slot;
        let mut var_arga_dn5: f64 = *var_arga_dn5_slot;
        let mut var_arga_rdb0: f64 = *var_arga_rdb0_slot;
        let mut var_arga_rdb1: f64 = *var_arga_rdb1_slot;
        let mut var_arga_rdn0: f64 = *var_arga_rdn0_slot;
        let mut var_arga_rdn1: f64 = *var_arga_rdn1_slot;
        let mut var_arga_rdn2: f64 = *var_arga_rdn2_slot;
        let mut var_arga_rdn3: f64 = *var_arga_rdn3_slot;
        let mut var_arga_rdn4: f64 = *var_arga_rdn4_slot;
        let mut var_arga_rdn5: f64 = *var_arga_rdn5_slot;
        let mut var_arga_rv: f64 = *var_arga_rv_slot;
        let mut var_argp: f64 = *var_argp_slot;
        let mut var_argp_db0: f64 = *var_argp_db0_slot;
        let mut var_argp_db1: f64 = *var_argp_db1_slot;
        let mut var_argp_dn0: f64 = *var_argp_dn0_slot;
        let mut var_argp_dn1: f64 = *var_argp_dn1_slot;
        let mut var_argp_dn2: f64 = *var_argp_dn2_slot;
        let mut var_argp_dn3: f64 = *var_argp_dn3_slot;
        let mut var_argp_dn4: f64 = *var_argp_dn4_slot;
        let mut var_argp_dn5: f64 = *var_argp_dn5_slot;
        let mut var_argp_rdb0: f64 = *var_argp_rdb0_slot;
        let mut var_argp_rdb1: f64 = *var_argp_rdb1_slot;
        let mut var_argp_rdn0: f64 = *var_argp_rdn0_slot;
        let mut var_argp_rdn1: f64 = *var_argp_rdn1_slot;
        let mut var_argp_rdn2: f64 = *var_argp_rdn2_slot;
        let mut var_argp_rdn3: f64 = *var_argp_rdn3_slot;
        let mut var_argp_rdn4: f64 = *var_argp_rdn4_slot;
        let mut var_argp_rdn5: f64 = *var_argp_rdn5_slot;
        let mut var_argp_rv: f64 = *var_argp_rv_slot;
        let mut var_dv0__blk268: f64 = *var_dv0__blk268_slot;
        let mut var_dv0__blk268_db0: f64 = *var_dv0__blk268_db0_slot;
        let mut var_dv0__blk268_db1: f64 = *var_dv0__blk268_db1_slot;
        let mut var_dv0__blk268_dn0: f64 = *var_dv0__blk268_dn0_slot;
        let mut var_dv0__blk268_dn1: f64 = *var_dv0__blk268_dn1_slot;
        let mut var_dv0__blk268_dn2: f64 = *var_dv0__blk268_dn2_slot;
        let mut var_dv0__blk268_dn3: f64 = *var_dv0__blk268_dn3_slot;
        let mut var_dv0__blk268_dn4: f64 = *var_dv0__blk268_dn4_slot;
        let mut var_dv0__blk268_dn5: f64 = *var_dv0__blk268_dn5_slot;
        let mut var_dv0__blk268_rdb0: f64 = *var_dv0__blk268_rdb0_slot;
        let mut var_dv0__blk268_rdb1: f64 = *var_dv0__blk268_rdb1_slot;
        let mut var_dv0__blk268_rdn0: f64 = *var_dv0__blk268_rdn0_slot;
        let mut var_dv0__blk268_rdn1: f64 = *var_dv0__blk268_rdn1_slot;
        let mut var_dv0__blk268_rdn2: f64 = *var_dv0__blk268_rdn2_slot;
        let mut var_dv0__blk268_rdn3: f64 = *var_dv0__blk268_rdn3_slot;
        let mut var_dv0__blk268_rdn4: f64 = *var_dv0__blk268_rdn4_slot;
        let mut var_dv0__blk268_rdn5: f64 = *var_dv0__blk268_rdn5_slot;
        let mut var_dv0__blk268_rv: f64 = *var_dv0__blk268_rv_slot;
        let mut var_dvh__blk269: f64 = *var_dvh__blk269_slot;
        let mut var_dvh__blk269_db0: f64 = *var_dvh__blk269_db0_slot;
        let mut var_dvh__blk269_db1: f64 = *var_dvh__blk269_db1_slot;
        let mut var_dvh__blk269_dn0: f64 = *var_dvh__blk269_dn0_slot;
        let mut var_dvh__blk269_dn1: f64 = *var_dvh__blk269_dn1_slot;
        let mut var_dvh__blk269_dn2: f64 = *var_dvh__blk269_dn2_slot;
        let mut var_dvh__blk269_dn3: f64 = *var_dvh__blk269_dn3_slot;
        let mut var_dvh__blk269_dn4: f64 = *var_dvh__blk269_dn4_slot;
        let mut var_dvh__blk269_dn5: f64 = *var_dvh__blk269_dn5_slot;
        let mut var_dvh__blk269_rdb0: f64 = *var_dvh__blk269_rdb0_slot;
        let mut var_dvh__blk269_rdb1: f64 = *var_dvh__blk269_rdb1_slot;
        let mut var_dvh__blk269_rdn0: f64 = *var_dvh__blk269_rdn0_slot;
        let mut var_dvh__blk269_rdn1: f64 = *var_dvh__blk269_rdn1_slot;
        let mut var_dvh__blk269_rdn2: f64 = *var_dvh__blk269_rdn2_slot;
        let mut var_dvh__blk269_rdn3: f64 = *var_dvh__blk269_rdn3_slot;
        let mut var_dvh__blk269_rdn4: f64 = *var_dvh__blk269_rdn4_slot;
        let mut var_dvh__blk269_rdn5: f64 = *var_dvh__blk269_rdn5_slot;
        let mut var_dvh__blk269_rv: f64 = *var_dvh__blk269_rv_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard267_db0: f64 = *var_guard267_db0_slot;
        let mut var_guard267_db1: f64 = *var_guard267_db1_slot;
        let mut var_guard267_dn0: f64 = *var_guard267_dn0_slot;
        let mut var_guard267_dn1: f64 = *var_guard267_dn1_slot;
        let mut var_guard267_dn2: f64 = *var_guard267_dn2_slot;
        let mut var_guard267_dn3: f64 = *var_guard267_dn3_slot;
        let mut var_guard267_dn4: f64 = *var_guard267_dn4_slot;
        let mut var_guard267_dn5: f64 = *var_guard267_dn5_slot;
        let mut var_guard267_rdb0: f64 = *var_guard267_rdb0_slot;
        let mut var_guard267_rdb1: f64 = *var_guard267_rdb1_slot;
        let mut var_guard267_rdn0: f64 = *var_guard267_rdn0_slot;
        let mut var_guard267_rdn1: f64 = *var_guard267_rdn1_slot;
        let mut var_guard267_rdn2: f64 = *var_guard267_rdn2_slot;
        let mut var_guard267_rdn3: f64 = *var_guard267_rdn3_slot;
        let mut var_guard267_rdn4: f64 = *var_guard267_rdn4_slot;
        let mut var_guard267_rdn5: f64 = *var_guard267_rdn5_slot;
        let mut var_guard267_rv: f64 = *var_guard267_rv_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard278_db0: f64 = *var_guard278_db0_slot;
        let mut var_guard278_db1: f64 = *var_guard278_db1_slot;
        let mut var_guard278_dn0: f64 = *var_guard278_dn0_slot;
        let mut var_guard278_dn1: f64 = *var_guard278_dn1_slot;
        let mut var_guard278_dn2: f64 = *var_guard278_dn2_slot;
        let mut var_guard278_dn3: f64 = *var_guard278_dn3_slot;
        let mut var_guard278_dn4: f64 = *var_guard278_dn4_slot;
        let mut var_guard278_dn5: f64 = *var_guard278_dn5_slot;
        let mut var_guard278_rdb0: f64 = *var_guard278_rdb0_slot;
        let mut var_guard278_rdb1: f64 = *var_guard278_rdb1_slot;
        let mut var_guard278_rdn0: f64 = *var_guard278_rdn0_slot;
        let mut var_guard278_rdn1: f64 = *var_guard278_rdn1_slot;
        let mut var_guard278_rdn2: f64 = *var_guard278_rdn2_slot;
        let mut var_guard278_rdn3: f64 = *var_guard278_rdn3_slot;
        let mut var_guard278_rdn4: f64 = *var_guard278_rdn4_slot;
        let mut var_guard278_rdn5: f64 = *var_guard278_rdn5_slot;
        let mut var_guard278_rv: f64 = *var_guard278_rv_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard279_db0: f64 = *var_guard279_db0_slot;
        let mut var_guard279_db1: f64 = *var_guard279_db1_slot;
        let mut var_guard279_dn0: f64 = *var_guard279_dn0_slot;
        let mut var_guard279_dn1: f64 = *var_guard279_dn1_slot;
        let mut var_guard279_dn2: f64 = *var_guard279_dn2_slot;
        let mut var_guard279_dn3: f64 = *var_guard279_dn3_slot;
        let mut var_guard279_dn4: f64 = *var_guard279_dn4_slot;
        let mut var_guard279_dn5: f64 = *var_guard279_dn5_slot;
        let mut var_guard279_rdb0: f64 = *var_guard279_rdb0_slot;
        let mut var_guard279_rdb1: f64 = *var_guard279_rdb1_slot;
        let mut var_guard279_rdn0: f64 = *var_guard279_rdn0_slot;
        let mut var_guard279_rdn1: f64 = *var_guard279_rdn1_slot;
        let mut var_guard279_rdn2: f64 = *var_guard279_rdn2_slot;
        let mut var_guard279_rdn3: f64 = *var_guard279_rdn3_slot;
        let mut var_guard279_rdn4: f64 = *var_guard279_rdn4_slot;
        let mut var_guard279_rdn5: f64 = *var_guard279_rdn5_slot;
        let mut var_guard279_rv: f64 = *var_guard279_rv_slot;
        let mut var_mv: f64 = *var_mv_slot;
        let mut var_mv0__blk273: f64 = *var_mv0__blk273_slot;
        let mut var_mv0__blk273_db0: f64 = *var_mv0__blk273_db0_slot;
        let mut var_mv0__blk273_db1: f64 = *var_mv0__blk273_db1_slot;
        let mut var_mv0__blk273_dn0: f64 = *var_mv0__blk273_dn0_slot;
        let mut var_mv0__blk273_dn1: f64 = *var_mv0__blk273_dn1_slot;
        let mut var_mv0__blk273_dn2: f64 = *var_mv0__blk273_dn2_slot;
        let mut var_mv0__blk273_dn3: f64 = *var_mv0__blk273_dn3_slot;
        let mut var_mv0__blk273_dn4: f64 = *var_mv0__blk273_dn4_slot;
        let mut var_mv0__blk273_dn5: f64 = *var_mv0__blk273_dn5_slot;
        let mut var_mv0__blk273_rdb0: f64 = *var_mv0__blk273_rdb0_slot;
        let mut var_mv0__blk273_rdb1: f64 = *var_mv0__blk273_rdb1_slot;
        let mut var_mv0__blk273_rdn0: f64 = *var_mv0__blk273_rdn0_slot;
        let mut var_mv0__blk273_rdn1: f64 = *var_mv0__blk273_rdn1_slot;
        let mut var_mv0__blk273_rdn2: f64 = *var_mv0__blk273_rdn2_slot;
        let mut var_mv0__blk273_rdn3: f64 = *var_mv0__blk273_rdn3_slot;
        let mut var_mv0__blk273_rdn4: f64 = *var_mv0__blk273_rdn4_slot;
        let mut var_mv0__blk273_rdn5: f64 = *var_mv0__blk273_rdn5_slot;
        let mut var_mv0__blk273_rv: f64 = *var_mv0__blk273_rv_slot;
        let mut var_mv_db0: f64 = *var_mv_db0_slot;
        let mut var_mv_db1: f64 = *var_mv_db1_slot;
        let mut var_mv_dn0: f64 = *var_mv_dn0_slot;
        let mut var_mv_dn1: f64 = *var_mv_dn1_slot;
        let mut var_mv_dn2: f64 = *var_mv_dn2_slot;
        let mut var_mv_dn3: f64 = *var_mv_dn3_slot;
        let mut var_mv_dn4: f64 = *var_mv_dn4_slot;
        let mut var_mv_dn5: f64 = *var_mv_dn5_slot;
        let mut var_mv_rdb0: f64 = *var_mv_rdb0_slot;
        let mut var_mv_rdb1: f64 = *var_mv_rdb1_slot;
        let mut var_mv_rdn0: f64 = *var_mv_rdn0_slot;
        let mut var_mv_rdn1: f64 = *var_mv_rdn1_slot;
        let mut var_mv_rdn2: f64 = *var_mv_rdn2_slot;
        let mut var_mv_rdn3: f64 = *var_mv_rdn3_slot;
        let mut var_mv_rdn4: f64 = *var_mv_rdn4_slot;
        let mut var_mv_rdn5: f64 = *var_mv_rdn5_slot;
        let mut var_mv_rv: f64 = *var_mv_rv_slot;
        let mut var_pwq__blk270: f64 = *var_pwq__blk270_slot;
        let mut var_pwq__blk270_db0: f64 = *var_pwq__blk270_db0_slot;
        let mut var_pwq__blk270_db1: f64 = *var_pwq__blk270_db1_slot;
        let mut var_pwq__blk270_dn0: f64 = *var_pwq__blk270_dn0_slot;
        let mut var_pwq__blk270_dn1: f64 = *var_pwq__blk270_dn1_slot;
        let mut var_pwq__blk270_dn2: f64 = *var_pwq__blk270_dn2_slot;
        let mut var_pwq__blk270_dn3: f64 = *var_pwq__blk270_dn3_slot;
        let mut var_pwq__blk270_dn4: f64 = *var_pwq__blk270_dn4_slot;
        let mut var_pwq__blk270_dn5: f64 = *var_pwq__blk270_dn5_slot;
        let mut var_pwq__blk270_rdb0: f64 = *var_pwq__blk270_rdb0_slot;
        let mut var_pwq__blk270_rdb1: f64 = *var_pwq__blk270_rdb1_slot;
        let mut var_pwq__blk270_rdn0: f64 = *var_pwq__blk270_rdn0_slot;
        let mut var_pwq__blk270_rdn1: f64 = *var_pwq__blk270_rdn1_slot;
        let mut var_pwq__blk270_rdn2: f64 = *var_pwq__blk270_rdn2_slot;
        let mut var_pwq__blk270_rdn3: f64 = *var_pwq__blk270_rdn3_slot;
        let mut var_pwq__blk270_rdn4: f64 = *var_pwq__blk270_rdn4_slot;
        let mut var_pwq__blk270_rdn5: f64 = *var_pwq__blk270_rdn5_slot;
        let mut var_pwq__blk270_rv: f64 = *var_pwq__blk270_rv_slot;
        let mut var_qhi__blk272: f64 = *var_qhi__blk272_slot;
        let mut var_qhi__blk272_db0: f64 = *var_qhi__blk272_db0_slot;
        let mut var_qhi__blk272_db1: f64 = *var_qhi__blk272_db1_slot;
        let mut var_qhi__blk272_dn0: f64 = *var_qhi__blk272_dn0_slot;
        let mut var_qhi__blk272_dn1: f64 = *var_qhi__blk272_dn1_slot;
        let mut var_qhi__blk272_dn2: f64 = *var_qhi__blk272_dn2_slot;
        let mut var_qhi__blk272_dn3: f64 = *var_qhi__blk272_dn3_slot;
        let mut var_qhi__blk272_dn4: f64 = *var_qhi__blk272_dn4_slot;
        let mut var_qhi__blk272_dn5: f64 = *var_qhi__blk272_dn5_slot;
        let mut var_qhi__blk272_rdb0: f64 = *var_qhi__blk272_rdb0_slot;
        let mut var_qhi__blk272_rdb1: f64 = *var_qhi__blk272_rdb1_slot;
        let mut var_qhi__blk272_rdn0: f64 = *var_qhi__blk272_rdn0_slot;
        let mut var_qhi__blk272_rdn1: f64 = *var_qhi__blk272_rdn1_slot;
        let mut var_qhi__blk272_rdn2: f64 = *var_qhi__blk272_rdn2_slot;
        let mut var_qhi__blk272_rdn3: f64 = *var_qhi__blk272_rdn3_slot;
        let mut var_qhi__blk272_rdn4: f64 = *var_qhi__blk272_rdn4_slot;
        let mut var_qhi__blk272_rdn5: f64 = *var_qhi__blk272_rdn5_slot;
        let mut var_qhi__blk272_rv: f64 = *var_qhi__blk272_rv_slot;
        let mut var_qlo: f64 = *var_qlo_slot;
        let mut var_qlo__blk271: f64 = *var_qlo__blk271_slot;
        let mut var_qlo__blk271_db0: f64 = *var_qlo__blk271_db0_slot;
        let mut var_qlo__blk271_db1: f64 = *var_qlo__blk271_db1_slot;
        let mut var_qlo__blk271_dn0: f64 = *var_qlo__blk271_dn0_slot;
        let mut var_qlo__blk271_dn1: f64 = *var_qlo__blk271_dn1_slot;
        let mut var_qlo__blk271_dn2: f64 = *var_qlo__blk271_dn2_slot;
        let mut var_qlo__blk271_dn3: f64 = *var_qlo__blk271_dn3_slot;
        let mut var_qlo__blk271_dn4: f64 = *var_qlo__blk271_dn4_slot;
        let mut var_qlo__blk271_dn5: f64 = *var_qlo__blk271_dn5_slot;
        let mut var_qlo__blk271_rdb0: f64 = *var_qlo__blk271_rdb0_slot;
        let mut var_qlo__blk271_rdb1: f64 = *var_qlo__blk271_rdb1_slot;
        let mut var_qlo__blk271_rdn0: f64 = *var_qlo__blk271_rdn0_slot;
        let mut var_qlo__blk271_rdn1: f64 = *var_qlo__blk271_rdn1_slot;
        let mut var_qlo__blk271_rdn2: f64 = *var_qlo__blk271_rdn2_slot;
        let mut var_qlo__blk271_rdn3: f64 = *var_qlo__blk271_rdn3_slot;
        let mut var_qlo__blk271_rdn4: f64 = *var_qlo__blk271_rdn4_slot;
        let mut var_qlo__blk271_rdn5: f64 = *var_qlo__blk271_rdn5_slot;
        let mut var_qlo__blk271_rv: f64 = *var_qlo__blk271_rv_slot;
        let mut var_qlo_db0: f64 = *var_qlo_db0_slot;
        let mut var_qlo_db1: f64 = *var_qlo_db1_slot;
        let mut var_qlo_dn0: f64 = *var_qlo_dn0_slot;
        let mut var_qlo_dn1: f64 = *var_qlo_dn1_slot;
        let mut var_qlo_dn2: f64 = *var_qlo_dn2_slot;
        let mut var_qlo_dn3: f64 = *var_qlo_dn3_slot;
        let mut var_qlo_dn4: f64 = *var_qlo_dn4_slot;
        let mut var_qlo_dn5: f64 = *var_qlo_dn5_slot;
        let mut var_qlo_rdb0: f64 = *var_qlo_rdb0_slot;
        let mut var_qlo_rdb1: f64 = *var_qlo_rdb1_slot;
        let mut var_qlo_rdn0: f64 = *var_qlo_rdn0_slot;
        let mut var_qlo_rdn1: f64 = *var_qlo_rdn1_slot;
        let mut var_qlo_rdn2: f64 = *var_qlo_rdn2_slot;
        let mut var_qlo_rdn3: f64 = *var_qlo_rdn3_slot;
        let mut var_qlo_rdn4: f64 = *var_qlo_rdn4_slot;
        let mut var_qlo_rdn5: f64 = *var_qlo_rdn5_slot;
        let mut var_qlo_rv: f64 = *var_qlo_rv_slot;
        let mut var_vl: f64 = *var_vl_slot;
        let mut var_vl_db0: f64 = *var_vl_db0_slot;
        let mut var_vl_db1: f64 = *var_vl_db1_slot;
        let mut var_vl_dn0: f64 = *var_vl_dn0_slot;
        let mut var_vl_dn1: f64 = *var_vl_dn1_slot;
        let mut var_vl_dn2: f64 = *var_vl_dn2_slot;
        let mut var_vl_dn3: f64 = *var_vl_dn3_slot;
        let mut var_vl_dn4: f64 = *var_vl_dn4_slot;
        let mut var_vl_dn5: f64 = *var_vl_dn5_slot;
        let mut var_vl_rdb0: f64 = *var_vl_rdb0_slot;
        let mut var_vl_rdb1: f64 = *var_vl_rdb1_slot;
        let mut var_vl_rdn0: f64 = *var_vl_rdn0_slot;
        let mut var_vl_rdn1: f64 = *var_vl_rdn1_slot;
        let mut var_vl_rdn2: f64 = *var_vl_rdn2_slot;
        let mut var_vl_rdn3: f64 = *var_vl_rdn3_slot;
        let mut var_vl_rdn4: f64 = *var_vl_rdn4_slot;
        let mut var_vl_rdn5: f64 = *var_vl_rdn5_slot;
        let mut var_vl_rv: f64 = *var_vl_rv_slot;

        let (assign3990_e3895, assign3990_e3895_d_n0, assign3990_e3895_d_n1, assign3990_e3895_d_n2, assign3990_e3895_d_n3, assign3990_e3895_d_n4, assign3990_e3895_d_n5, assign3990_e3895_d_b0, assign3990_e3895_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign3990_e3886: f64 = (var_dv * var_dv);
        let assign3990_e3889: f64 = (4.0 * p.p75);
        let assign3990_e3891: f64 = (assign3990_e3889 * p.p75);
        let assign3990_e3892: f64 = (assign3990_e3886 + assign3990_e3891);
        let assign3990_e3893: f64 = (assign3990_e3892).sqrt();
        (assign3990_e3893, (((var_dv_dn0 * var_dv) + (var_dv * var_dv_dn0)) / (2.0 * assign3990_e3893)), (((var_dv_dn1 * var_dv) + (var_dv * var_dv_dn1)) / (2.0 * assign3990_e3893)), (((var_dv_dn2 * var_dv) + (var_dv * var_dv_dn2)) / (2.0 * assign3990_e3893)), (((var_dv_dn3 * var_dv) + (var_dv * var_dv_dn3)) / (2.0 * assign3990_e3893)), (((var_dv_dn4 * var_dv) + (var_dv * var_dv_dn4)) / (2.0 * assign3990_e3893)), (((var_dv_dn5 * var_dv) + (var_dv * var_dv_dn5)) / (2.0 * assign3990_e3893)), (((var_dv_db0 * var_dv) + (var_dv * var_dv_db0)) / (2.0 * assign3990_e3893)), (((var_dv_db1 * var_dv) + (var_dv * var_dv_db1)) / (2.0 * assign3990_e3893)),)
    } else {
        (var_mv, var_mv_dn0, var_mv_dn1, var_mv_dn2, var_mv_dn3, var_mv_dn4, var_mv_dn5, var_mv_db0, var_mv_db1,)
    }
};
        var_mv = assign3990_e3895;
        var_mv_dn0 = assign3990_e3895_d_n0;
        var_mv_dn1 = assign3990_e3895_d_n1;
        var_mv_dn2 = assign3990_e3895_d_n2;
        var_mv_dn3 = assign3990_e3895_d_n3;
        var_mv_dn4 = assign3990_e3895_d_n4;
        var_mv_dn5 = assign3990_e3895_d_n5;
        var_mv_db0 = assign3990_e3895_d_b0;
        var_mv_db1 = assign3990_e3895_d_b1;
        var_mv_rv = 0.0;
        var_mv_rdn0 = 0.0;
        var_mv_rdn1 = 0.0;
        var_mv_rdn2 = 0.0;
        var_mv_rdn3 = 0.0;
        var_mv_rdn4 = 0.0;
        var_mv_rdn5 = 0.0;
        var_mv_rdb0 = 0.0;
        var_mv_rdb1 = 0.0;

        let (assign4000_e3910, assign4000_e3910_d_n0, assign4000_e3910_d_n1, assign4000_e3910_d_n2, assign4000_e3910_d_n3, assign4000_e3910_d_n4, assign4000_e3910_d_n5, assign4000_e3910_d_b0, assign4000_e3910_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign4000_e3905: f64 = (var_dv - var_mv);
        let assign4000_e3906: f64 = (0.5 * assign4000_e3905);
        let assign4000_e3908: f64 = (assign4000_e3906 - var_dv0);
        (assign4000_e3908, ((0.5 * (var_dv_dn0 - var_mv_dn0)) - var_dv0_dn0), ((0.5 * (var_dv_dn1 - var_mv_dn1)) - var_dv0_dn1), ((0.5 * (var_dv_dn2 - var_mv_dn2)) - var_dv0_dn2), ((0.5 * (var_dv_dn3 - var_mv_dn3)) - var_dv0_dn3), ((0.5 * (var_dv_dn4 - var_mv_dn4)) - var_dv0_dn4), ((0.5 * (var_dv_dn5 - var_mv_dn5)) - var_dv0_dn5), ((0.5 * (var_dv_db0 - var_mv_db0)) - var_dv0_db0), ((0.5 * (var_dv_db1 - var_mv_db1)) - var_dv0_db1),)
    } else {
        (var_vl, var_vl_dn0, var_vl_dn1, var_vl_dn2, var_vl_dn3, var_vl_dn4, var_vl_dn5, var_vl_db0, var_vl_db1,)
    }
};
        var_vl = assign4000_e3910;
        var_vl_dn0 = assign4000_e3910_d_n0;
        var_vl_dn1 = assign4000_e3910_d_n1;
        var_vl_dn2 = assign4000_e3910_d_n2;
        var_vl_dn3 = assign4000_e3910_d_n3;
        var_vl_dn4 = assign4000_e3910_d_n4;
        var_vl_dn5 = assign4000_e3910_d_n5;
        var_vl_db0 = assign4000_e3910_d_b0;
        var_vl_db1 = assign4000_e3910_d_b1;
        var_vl_rv = 0.0;
        var_vl_rdn0 = 0.0;
        var_vl_rdn1 = 0.0;
        var_vl_rdn2 = 0.0;
        var_vl_rdn3 = 0.0;
        var_vl_rdn4 = 0.0;
        var_vl_rdn5 = 0.0;
        var_vl_rdb0 = 0.0;
        var_vl_rdb1 = 0.0;

        let (assign4010_e3934, assign4010_e3934_d_n0, assign4010_e3934_d_n1, assign4010_e3934_d_n2, assign4010_e3934_d_n3, assign4010_e3934_d_n4, assign4010_e3934_d_n5, assign4010_e3934_d_b0, assign4010_e3934_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign4010_e3918: f64 = (-var_pa_t);
        let assign4010_e3922: f64 = (var_vl / var_pa_t);
        let assign4010_e3923: f64 = (1.0 - assign4010_e3922);
        let assign4010_e3926: f64 = (1.0 - p.p74);
        let assign4010_e3927: f64 = (assign4010_e3923).powf(assign4010_e3926);
        let assign4010_e3928: f64 = (assign4010_e3918 * assign4010_e3927);
        let assign4010_e3931: f64 = (1.0 - p.p74);
        let assign4010_e3932: f64 = (assign4010_e3928 / assign4010_e3931);
        (assign4010_e3932, ((((-var_pa_t_dn0) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn0 * var_pa_t) - (var_vl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn0 * var_pa_t) - (var_vl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_dn1) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn1 * var_pa_t) - (var_vl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn1 * var_pa_t) - (var_vl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_dn2) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn2 * var_pa_t) - (var_vl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn2 * var_pa_t) - (var_vl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_dn3) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn3 * var_pa_t) - (var_vl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn3 * var_pa_t) - (var_vl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_dn4) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn4 * var_pa_t) - (var_vl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn4 * var_pa_t) - (var_vl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_dn5) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_dn5 * var_pa_t) - (var_vl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_dn5 * var_pa_t) - (var_vl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_db0) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_db0 * var_pa_t) - (var_vl * var_pa_t_db0)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_db0 * var_pa_t) - (var_vl * var_pa_t_db0)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931), ((((-var_pa_t_db1) * assign4010_e3927) + (assign4010_e3918 * if 0.0 == 0.0 && ((assign4010_e3926) as f64).is_finite() && ((assign4010_e3926) as f64).fract() == 0.0 { if assign4010_e3926 == 0.0 { 0.0 } else { (assign4010_e3926 * ((assign4010_e3923).powf(assign4010_e3926 - 1.0) * (-(((var_vl_db1 * var_pa_t) - (var_vl * var_pa_t_db1)) / (var_pa_t * var_pa_t))))) } } else { (assign4010_e3927 * (assign4010_e3926 * ((-(((var_vl_db1 * var_pa_t) - (var_vl * var_pa_t_db1)) / (var_pa_t * var_pa_t))) / assign4010_e3923))) })) / assign4010_e3931),)
    } else {
        (var_qlo, var_qlo_dn0, var_qlo_dn1, var_qlo_dn2, var_qlo_dn3, var_qlo_dn4, var_qlo_dn5, var_qlo_db0, var_qlo_db1,)
    }
};
        var_qlo = assign4010_e3934;
        var_qlo_dn0 = assign4010_e3934_d_n0;
        var_qlo_dn1 = assign4010_e3934_d_n1;
        var_qlo_dn2 = assign4010_e3934_d_n2;
        var_qlo_dn3 = assign4010_e3934_d_n3;
        var_qlo_dn4 = assign4010_e3934_d_n4;
        var_qlo_dn5 = assign4010_e3934_d_n5;
        var_qlo_db0 = assign4010_e3934_d_b0;
        var_qlo_db1 = assign4010_e3934_d_b1;
        var_qlo_rv = 0.0;
        var_qlo_rdn0 = 0.0;
        var_qlo_rdn1 = 0.0;
        var_qlo_rdn2 = 0.0;
        var_qlo_rdn3 = 0.0;
        var_qlo_rdn4 = 0.0;
        var_qlo_rdn5 = 0.0;
        var_qlo_rdb0 = 0.0;
        var_qlo_rdb1 = 0.0;

        let (assign4020_e3974, assign4020_e3974_d_n0, assign4020_e3974_d_n1, assign4020_e3974_d_n2, assign4020_e3974_d_n3, assign4020_e3974_d_n4, assign4020_e3974_d_n5, assign4020_e3974_d_b0, assign4020_e3974_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard254 != 0.0)) && (var_guard265 == 0.0)) {
        let assign4020_e3944: f64 = (1.0 - p.p68);
        let assign4020_e3946: f64 = (-p.p74);
        let assign4020_e3947: f64 = (assign4020_e3944).powf(assign4020_e3946);
        let assign4020_e3950: f64 = (var_vcl - var_vl);
        let assign4020_e3952: f64 = (assign4020_e3950 + var_vl0);
        let assign4020_e3953: f64 = (assign4020_e3947 * assign4020_e3952);
        let assign4020_e3957: f64 = (0.5 * p.p74);
        let assign4020_e3960: f64 = (var_vcl - var_vl);
        let assign4020_e3962: f64 = (assign4020_e3960 + var_vl0);
        let assign4020_e3963: f64 = (assign4020_e3957 * assign4020_e3962);
        let assign4020_e3967: f64 = (1.0 - p.p68);
        let assign4020_e3968: f64 = (var_pa_t * assign4020_e3967);
        let assign4020_e3969: f64 = (assign4020_e3963 / assign4020_e3968);
        let assign4020_e3970: f64 = (1.0 + assign4020_e3969);
        let assign4020_e3971: f64 = (assign4020_e3953 * assign4020_e3970);
        let assign4020_e3972: f64 = (var_qlo + assign4020_e3971);
        (assign4020_e3972, (var_qlo_dn0 + (((assign4020_e3947 * ((var_vcl_dn0 - var_vl_dn0) + var_vl0_dn0)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn0 - var_vl_dn0) + var_vl0_dn0)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn0 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_dn1 + (((assign4020_e3947 * ((var_vcl_dn1 - var_vl_dn1) + var_vl0_dn1)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn1 - var_vl_dn1) + var_vl0_dn1)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn1 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_dn2 + (((assign4020_e3947 * ((var_vcl_dn2 - var_vl_dn2) + var_vl0_dn2)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn2 - var_vl_dn2) + var_vl0_dn2)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn2 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_dn3 + (((assign4020_e3947 * ((var_vcl_dn3 - var_vl_dn3) + var_vl0_dn3)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn3 - var_vl_dn3) + var_vl0_dn3)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn3 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_dn4 + (((assign4020_e3947 * ((var_vcl_dn4 - var_vl_dn4) + var_vl0_dn4)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn4 - var_vl_dn4) + var_vl0_dn4)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn4 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_dn5 + (((assign4020_e3947 * ((var_vcl_dn5 - var_vl_dn5) + var_vl0_dn5)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_dn5 - var_vl_dn5) + var_vl0_dn5)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_dn5 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_db0 + (((assign4020_e3947 * ((var_vcl_db0 - var_vl_db0) + var_vl0_db0)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_db0 - var_vl_db0) + var_vl0_db0)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_db0 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))), (var_qlo_db1 + (((assign4020_e3947 * ((var_vcl_db1 - var_vl_db1) + var_vl0_db1)) * assign4020_e3970) + (assign4020_e3953 * ((((assign4020_e3957 * ((var_vcl_db1 - var_vl_db1) + var_vl0_db1)) * assign4020_e3968) - (assign4020_e3963 * (var_pa_t_db1 * assign4020_e3967))) / (assign4020_e3968 * assign4020_e3968))))),)
    } else {
        (var_arga, var_arga_dn0, var_arga_dn1, var_arga_dn2, var_arga_dn3, var_arga_dn4, var_arga_dn5, var_arga_db0, var_arga_db1,)
    }
};
        var_arga = assign4020_e3974;
        var_arga_dn0 = assign4020_e3974_d_n0;
        var_arga_dn1 = assign4020_e3974_d_n1;
        var_arga_dn2 = assign4020_e3974_d_n2;
        var_arga_dn3 = assign4020_e3974_d_n3;
        var_arga_dn4 = assign4020_e3974_d_n4;
        var_arga_dn5 = assign4020_e3974_d_n5;
        var_arga_db0 = assign4020_e3974_d_b0;
        var_arga_db1 = assign4020_e3974_d_b1;
        var_arga_rv = 0.0;
        var_arga_rdn0 = 0.0;
        var_arga_rdn1 = 0.0;
        var_arga_rdn2 = 0.0;
        var_arga_rdn3 = 0.0;
        var_arga_rdn4 = 0.0;
        var_arga_rdn5 = 0.0;
        var_arga_rdb0 = 0.0;
        var_arga_rdb1 = 0.0;

        let (assign4030_e3981, assign4030_e3981_d_n0, assign4030_e3981_d_n1, assign4030_e3981_d_n2, assign4030_e3981_d_n3, assign4030_e3981_d_n4, assign4030_e3981_d_n5, assign4030_e3981_d_b0, assign4030_e3981_d_b1,) = {
    if ((var_guard249 != 0.0) && (var_guard254 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arga, var_arga_dn0, var_arga_dn1, var_arga_dn2, var_arga_dn3, var_arga_dn4, var_arga_dn5, var_arga_db0, var_arga_db1,)
    }
};
        var_arga = assign4030_e3981;
        var_arga_dn0 = assign4030_e3981_d_n0;
        var_arga_dn1 = assign4030_e3981_d_n1;
        var_arga_dn2 = assign4030_e3981_d_n2;
        var_arga_dn3 = assign4030_e3981_d_n3;
        var_arga_dn4 = assign4030_e3981_d_n4;
        var_arga_dn5 = assign4030_e3981_d_n5;
        var_arga_db0 = assign4030_e3981_d_b0;
        var_arga_db1 = assign4030_e3981_d_b1;
        var_arga_rv = 0.0;
        var_arga_rdn0 = 0.0;
        var_arga_rdn1 = 0.0;
        var_arga_rdn2 = 0.0;
        var_arga_rdn3 = 0.0;
        var_arga_rdn4 = 0.0;
        var_arga_rdn5 = 0.0;
        var_arga_rdb0 = 0.0;
        var_arga_rdb1 = 0.0;

        let assign4040_e3984: f64 = if var_pcjp > 0.0 { 1.0 } else { 0.0 };
        var_guard267 = assign4040_e3984;
        var_guard267_dn0 = 0.0;
        var_guard267_dn1 = 0.0;
        var_guard267_dn2 = 0.0;
        var_guard267_dn3 = 0.0;
        var_guard267_dn4 = 0.0;
        var_guard267_dn5 = 0.0;
        var_guard267_db0 = 0.0;
        var_guard267_db1 = 0.0;
        var_guard267_rv = 0.0;
        var_guard267_rdn0 = 0.0;
        var_guard267_rdn1 = 0.0;
        var_guard267_rdn2 = 0.0;
        var_guard267_rdn3 = 0.0;
        var_guard267_rdn4 = 0.0;
        var_guard267_rdn5 = 0.0;
        var_guard267_rdb0 = 0.0;
        var_guard267_rdb1 = 0.0;

        let (assign4050_e3993, assign4050_e3993_d_n0, assign4050_e3993_d_n1, assign4050_e3993_d_n2, assign4050_e3993_d_n3, assign4050_e3993_d_n4, assign4050_e3993_d_n5, assign4050_e3993_d_b0, assign4050_e3993_d_b1,) = {
    if ((var_guard249 != 0.0) && (var_guard267 != 0.0)) {
        let assign4050_e3989: f64 = (-var_pp_t);
        let assign4050_e3991: f64 = (assign4050_e3989 * p.p68);
        (assign4050_e3991, ((-var_pp_t_dn0) * p.p68), ((-var_pp_t_dn1) * p.p68), ((-var_pp_t_dn2) * p.p68), ((-var_pp_t_dn3) * p.p68), ((-var_pp_t_dn4) * p.p68), ((-var_pp_t_dn5) * p.p68), ((-var_pp_t_db0) * p.p68), ((-var_pp_t_db1) * p.p68),)
    } else {
        (var_dv0__blk268, var_dv0__blk268_dn0, var_dv0__blk268_dn1, var_dv0__blk268_dn2, var_dv0__blk268_dn3, var_dv0__blk268_dn4, var_dv0__blk268_dn5, var_dv0__blk268_db0, var_dv0__blk268_db1,)
    }
};
        var_dv0__blk268 = assign4050_e3993;
        var_dv0__blk268_dn0 = assign4050_e3993_d_n0;
        var_dv0__blk268_dn1 = assign4050_e3993_d_n1;
        var_dv0__blk268_dn2 = assign4050_e3993_d_n2;
        var_dv0__blk268_dn3 = assign4050_e3993_d_n3;
        var_dv0__blk268_dn4 = assign4050_e3993_d_n4;
        var_dv0__blk268_dn5 = assign4050_e3993_d_n5;
        var_dv0__blk268_db0 = assign4050_e3993_d_b0;
        var_dv0__blk268_db1 = assign4050_e3993_d_b1;
        var_dv0__blk268_rv = 0.0;
        var_dv0__blk268_rdn0 = 0.0;
        var_dv0__blk268_rdn1 = 0.0;
        var_dv0__blk268_rdn2 = 0.0;
        var_dv0__blk268_rdn3 = 0.0;
        var_dv0__blk268_rdn4 = 0.0;
        var_dv0__blk268_rdn5 = 0.0;
        var_dv0__blk268_rdb0 = 0.0;
        var_dv0__blk268_rdb1 = 0.0;

        let assign4060_e3996: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        var_guard278 = assign4060_e3996;
        var_guard278_dn0 = 0.0;
        var_guard278_dn1 = 0.0;
        var_guard278_dn2 = 0.0;
        var_guard278_dn3 = 0.0;
        var_guard278_dn4 = 0.0;
        var_guard278_dn5 = 0.0;
        var_guard278_db0 = 0.0;
        var_guard278_db1 = 0.0;
        var_guard278_rv = 0.0;
        var_guard278_rdn0 = 0.0;
        var_guard278_rdn1 = 0.0;
        var_guard278_rdn2 = 0.0;
        var_guard278_rdn3 = 0.0;
        var_guard278_rdn4 = 0.0;
        var_guard278_rdn5 = 0.0;
        var_guard278_rdb0 = 0.0;
        var_guard278_rdb1 = 0.0;

        let (assign4070_e4006, assign4070_e4006_d_n0, assign4070_e4006_d_n1, assign4070_e4006_d_n2, assign4070_e4006_d_n3, assign4070_e4006_d_n4, assign4070_e4006_d_n5, assign4070_e4006_d_b0, assign4070_e4006_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) {
        let assign4070_e4004: f64 = (var_vcl + var_dv0__blk268);
        (assign4070_e4004, (var_vcl_dn0 + var_dv0__blk268_dn0), (var_vcl_dn1 + var_dv0__blk268_dn1), (var_vcl_dn2 + var_dv0__blk268_dn2), (var_vcl_dn3 + var_dv0__blk268_dn3), (var_vcl_dn4 + var_dv0__blk268_dn4), (var_vcl_dn5 + var_dv0__blk268_dn5), (var_vcl_db0 + var_dv0__blk268_db0), (var_vcl_db1 + var_dv0__blk268_db1),)
    } else {
        (var_dvh__blk269, var_dvh__blk269_dn0, var_dvh__blk269_dn1, var_dvh__blk269_dn2, var_dvh__blk269_dn3, var_dvh__blk269_dn4, var_dvh__blk269_dn5, var_dvh__blk269_db0, var_dvh__blk269_db1,)
    }
};
        var_dvh__blk269 = assign4070_e4006;
        var_dvh__blk269_dn0 = assign4070_e4006_d_n0;
        var_dvh__blk269_dn1 = assign4070_e4006_d_n1;
        var_dvh__blk269_dn2 = assign4070_e4006_d_n2;
        var_dvh__blk269_dn3 = assign4070_e4006_d_n3;
        var_dvh__blk269_dn4 = assign4070_e4006_d_n4;
        var_dvh__blk269_dn5 = assign4070_e4006_d_n5;
        var_dvh__blk269_db0 = assign4070_e4006_d_b0;
        var_dvh__blk269_db1 = assign4070_e4006_d_b1;
        var_dvh__blk269_rv = 0.0;
        var_dvh__blk269_rdn0 = 0.0;
        var_dvh__blk269_rdn1 = 0.0;
        var_dvh__blk269_rdn2 = 0.0;
        var_dvh__blk269_rdn3 = 0.0;
        var_dvh__blk269_rdn4 = 0.0;
        var_dvh__blk269_rdn5 = 0.0;
        var_dvh__blk269_rdb0 = 0.0;
        var_dvh__blk269_rdb1 = 0.0;

        let assign4080_e4009: f64 = if var_dvh__blk269 > 0.0 { 1.0 } else { 0.0 };
        var_guard279 = assign4080_e4009;
        var_guard279_dn0 = 0.0;
        var_guard279_dn1 = 0.0;
        var_guard279_dn2 = 0.0;
        var_guard279_dn3 = 0.0;
        var_guard279_dn4 = 0.0;
        var_guard279_dn5 = 0.0;
        var_guard279_db0 = 0.0;
        var_guard279_db1 = 0.0;
        var_guard279_rv = 0.0;
        var_guard279_rdn0 = 0.0;
        var_guard279_rdn1 = 0.0;
        var_guard279_rdn2 = 0.0;
        var_guard279_rdn3 = 0.0;
        var_guard279_rdn4 = 0.0;
        var_guard279_rdn5 = 0.0;
        var_guard279_rdb0 = 0.0;
        var_guard279_rdb1 = 0.0;

        let (assign4090_e4024, assign4090_e4024_d_n0, assign4090_e4024_d_n1, assign4090_e4024_d_n2, assign4090_e4024_d_n3, assign4090_e4024_d_n4, assign4090_e4024_d_n5, assign4090_e4024_d_b0, assign4090_e4024_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign4090_e4019: f64 = (1.0 - p.p68);
        let assign4090_e4021: f64 = (-p.p81);
        let assign4090_e4022: f64 = (assign4090_e4019).powf(assign4090_e4021);
        (assign4090_e4022, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq__blk270, var_pwq__blk270_dn0, var_pwq__blk270_dn1, var_pwq__blk270_dn2, var_pwq__blk270_dn3, var_pwq__blk270_dn4, var_pwq__blk270_dn5, var_pwq__blk270_db0, var_pwq__blk270_db1,)
    }
};
        var_pwq__blk270 = assign4090_e4024;
        var_pwq__blk270_dn0 = assign4090_e4024_d_n0;
        var_pwq__blk270_dn1 = assign4090_e4024_d_n1;
        var_pwq__blk270_dn2 = assign4090_e4024_d_n2;
        var_pwq__blk270_dn3 = assign4090_e4024_d_n3;
        var_pwq__blk270_dn4 = assign4090_e4024_d_n4;
        var_pwq__blk270_dn5 = assign4090_e4024_d_n5;
        var_pwq__blk270_db0 = assign4090_e4024_d_b0;
        var_pwq__blk270_db1 = assign4090_e4024_d_b1;
        var_pwq__blk270_rv = 0.0;
        var_pwq__blk270_rdn0 = 0.0;
        var_pwq__blk270_rdn1 = 0.0;
        var_pwq__blk270_rdn2 = 0.0;
        var_pwq__blk270_rdn3 = 0.0;
        var_pwq__blk270_rdn4 = 0.0;
        var_pwq__blk270_rdn5 = 0.0;
        var_pwq__blk270_rdb0 = 0.0;
        var_pwq__blk270_rdb1 = 0.0;

        let (assign4100_e4046, assign4100_e4046_d_n0, assign4100_e4046_d_n1, assign4100_e4046_d_n2, assign4100_e4046_d_n3, assign4100_e4046_d_n4, assign4100_e4046_d_n5, assign4100_e4046_d_b0, assign4100_e4046_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign4100_e4037: f64 = (1.0 - p.p68);
        let assign4100_e4038: f64 = (var_pwq__blk270 * assign4100_e4037);
        let assign4100_e4039: f64 = (1.0 - assign4100_e4038);
        let assign4100_e4040: f64 = (var_pp_t * assign4100_e4039);
        let assign4100_e4043: f64 = (1.0 - p.p81);
        let assign4100_e4044: f64 = (assign4100_e4040 / assign4100_e4043);
        (assign4100_e4044, (((var_pp_t_dn0 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn0 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_dn1 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn1 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_dn2 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn2 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_dn3 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn3 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_dn4 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn4 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_dn5 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_dn5 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_db0 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_db0 * assign4100_e4037)))) / assign4100_e4043), (((var_pp_t_db1 * assign4100_e4039) + (var_pp_t * (-(var_pwq__blk270_db1 * assign4100_e4037)))) / assign4100_e4043),)
    } else {
        (var_qlo__blk271, var_qlo__blk271_dn0, var_qlo__blk271_dn1, var_qlo__blk271_dn2, var_qlo__blk271_dn3, var_qlo__blk271_dn4, var_qlo__blk271_dn5, var_qlo__blk271_db0, var_qlo__blk271_db1,)
    }
};
        var_qlo__blk271 = assign4100_e4046;
        var_qlo__blk271_dn0 = assign4100_e4046_d_n0;
        var_qlo__blk271_dn1 = assign4100_e4046_d_n1;
        var_qlo__blk271_dn2 = assign4100_e4046_d_n2;
        var_qlo__blk271_dn3 = assign4100_e4046_d_n3;
        var_qlo__blk271_dn4 = assign4100_e4046_d_n4;
        var_qlo__blk271_dn5 = assign4100_e4046_d_n5;
        var_qlo__blk271_db0 = assign4100_e4046_d_b0;
        var_qlo__blk271_db1 = assign4100_e4046_d_b1;
        var_qlo__blk271_rv = 0.0;
        var_qlo__blk271_rdn0 = 0.0;
        var_qlo__blk271_rdn1 = 0.0;
        var_qlo__blk271_rdn2 = 0.0;
        var_qlo__blk271_rdn3 = 0.0;
        var_qlo__blk271_rdn4 = 0.0;
        var_qlo__blk271_rdn5 = 0.0;
        var_qlo__blk271_rdb0 = 0.0;
        var_qlo__blk271_rdb1 = 0.0;

        let (assign4110_e4072, assign4110_e4072_d_n0, assign4110_e4072_d_n1, assign4110_e4072_d_n2, assign4110_e4072_d_n3, assign4110_e4072_d_n4, assign4110_e4072_d_n5, assign4110_e4072_d_b0, assign4110_e4072_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign4110_e4058: f64 = (0.5 * p.p81);
        let assign4110_e4060: f64 = (assign4110_e4058 * var_dvh__blk269);
        let assign4110_e4064: f64 = (1.0 - p.p68);
        let assign4110_e4065: f64 = (var_pp_t * assign4110_e4064);
        let assign4110_e4066: f64 = (assign4110_e4060 / assign4110_e4065);
        let assign4110_e4067: f64 = (1.0 + assign4110_e4066);
        let assign4110_e4068: f64 = (var_dvh__blk269 * assign4110_e4067);
        let assign4110_e4070: f64 = (assign4110_e4068 * var_pwq__blk270);
        (assign4110_e4070, ((((var_dvh__blk269_dn0 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn0) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn0 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn0)), ((((var_dvh__blk269_dn1 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn1) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn1 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn1)), ((((var_dvh__blk269_dn2 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn2) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn2 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn2)), ((((var_dvh__blk269_dn3 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn3) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn3 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn3)), ((((var_dvh__blk269_dn4 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn4) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn4 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn4)), ((((var_dvh__blk269_dn5 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_dn5) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_dn5 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_dn5)), ((((var_dvh__blk269_db0 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_db0) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_db0 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_db0)), ((((var_dvh__blk269_db1 * assign4110_e4067) + (var_dvh__blk269 * ((((assign4110_e4058 * var_dvh__blk269_db1) * assign4110_e4065) - (assign4110_e4060 * (var_pp_t_db1 * assign4110_e4064))) / (assign4110_e4065 * assign4110_e4065)))) * var_pwq__blk270) + (assign4110_e4068 * var_pwq__blk270_db1)),)
    } else {
        (var_qhi__blk272, var_qhi__blk272_dn0, var_qhi__blk272_dn1, var_qhi__blk272_dn2, var_qhi__blk272_dn3, var_qhi__blk272_dn4, var_qhi__blk272_dn5, var_qhi__blk272_db0, var_qhi__blk272_db1,)
    }
};
        var_qhi__blk272 = assign4110_e4072;
        var_qhi__blk272_dn0 = assign4110_e4072_d_n0;
        var_qhi__blk272_dn1 = assign4110_e4072_d_n1;
        var_qhi__blk272_dn2 = assign4110_e4072_d_n2;
        var_qhi__blk272_dn3 = assign4110_e4072_d_n3;
        var_qhi__blk272_dn4 = assign4110_e4072_d_n4;
        var_qhi__blk272_dn5 = assign4110_e4072_d_n5;
        var_qhi__blk272_db0 = assign4110_e4072_d_b0;
        var_qhi__blk272_db1 = assign4110_e4072_d_b1;
        var_qhi__blk272_rv = 0.0;
        var_qhi__blk272_rdn0 = 0.0;
        var_qhi__blk272_rdn1 = 0.0;
        var_qhi__blk272_rdn2 = 0.0;
        var_qhi__blk272_rdn3 = 0.0;
        var_qhi__blk272_rdn4 = 0.0;
        var_qhi__blk272_rdn5 = 0.0;
        var_qhi__blk272_rdb0 = 0.0;
        var_qhi__blk272_rdb1 = 0.0;

        let (assign4120_e4099, assign4120_e4099_d_n0, assign4120_e4099_d_n1, assign4120_e4099_d_n2, assign4120_e4099_d_n3, assign4120_e4099_d_n4, assign4120_e4099_d_n5, assign4120_e4099_d_b0, assign4120_e4099_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) && (var_guard279 == 0.0)) {
        let assign4120_e4086: f64 = (var_vcl / var_pp_t);
        let assign4120_e4087: f64 = (1.0 - assign4120_e4086);
        let assign4120_e4090: f64 = (1.0 - p.p81);
        let assign4120_e4091: f64 = (assign4120_e4087).powf(assign4120_e4090);
        let assign4120_e4092: f64 = (1.0 - assign4120_e4091);
        let assign4120_e4093: f64 = (var_pp_t * assign4120_e4092);
        let assign4120_e4096: f64 = (1.0 - p.p81);
        let assign4120_e4097: f64 = (assign4120_e4093 / assign4120_e4096);
        (assign4120_e4097, (((var_pp_t_dn0 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn0 * var_pp_t) - (var_vcl * var_pp_t_dn0)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn0 * var_pp_t) - (var_vcl * var_pp_t_dn0)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_dn1 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn1 * var_pp_t) - (var_vcl * var_pp_t_dn1)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn1 * var_pp_t) - (var_vcl * var_pp_t_dn1)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_dn2 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn2 * var_pp_t) - (var_vcl * var_pp_t_dn2)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn2 * var_pp_t) - (var_vcl * var_pp_t_dn2)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_dn3 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn3 * var_pp_t) - (var_vcl * var_pp_t_dn3)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn3 * var_pp_t) - (var_vcl * var_pp_t_dn3)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_dn4 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn4 * var_pp_t) - (var_vcl * var_pp_t_dn4)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn4 * var_pp_t) - (var_vcl * var_pp_t_dn4)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_dn5 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_dn5 * var_pp_t) - (var_vcl * var_pp_t_dn5)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_dn5 * var_pp_t) - (var_vcl * var_pp_t_dn5)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_db0 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_db0 * var_pp_t) - (var_vcl * var_pp_t_db0)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_db0 * var_pp_t) - (var_vcl * var_pp_t_db0)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096), (((var_pp_t_db1 * assign4120_e4092) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4120_e4090) as f64).is_finite() && ((assign4120_e4090) as f64).fract() == 0.0 { if assign4120_e4090 == 0.0 { 0.0 } else { (assign4120_e4090 * ((assign4120_e4087).powf(assign4120_e4090 - 1.0) * (-(((var_vcl_db1 * var_pp_t) - (var_vcl * var_pp_t_db1)) / (var_pp_t * var_pp_t))))) } } else { (assign4120_e4091 * (assign4120_e4090 * ((-(((var_vcl_db1 * var_pp_t) - (var_vcl * var_pp_t_db1)) / (var_pp_t * var_pp_t))) / assign4120_e4087))) }))) / assign4120_e4096),)
    } else {
        (var_qlo__blk271, var_qlo__blk271_dn0, var_qlo__blk271_dn1, var_qlo__blk271_dn2, var_qlo__blk271_dn3, var_qlo__blk271_dn4, var_qlo__blk271_dn5, var_qlo__blk271_db0, var_qlo__blk271_db1,)
    }
};
        var_qlo__blk271 = assign4120_e4099;
        var_qlo__blk271_dn0 = assign4120_e4099_d_n0;
        var_qlo__blk271_dn1 = assign4120_e4099_d_n1;
        var_qlo__blk271_dn2 = assign4120_e4099_d_n2;
        var_qlo__blk271_dn3 = assign4120_e4099_d_n3;
        var_qlo__blk271_dn4 = assign4120_e4099_d_n4;
        var_qlo__blk271_dn5 = assign4120_e4099_d_n5;
        var_qlo__blk271_db0 = assign4120_e4099_d_b0;
        var_qlo__blk271_db1 = assign4120_e4099_d_b1;
        var_qlo__blk271_rv = 0.0;
        var_qlo__blk271_rdn0 = 0.0;
        var_qlo__blk271_rdn1 = 0.0;
        var_qlo__blk271_rdn2 = 0.0;
        var_qlo__blk271_rdn3 = 0.0;
        var_qlo__blk271_rdn4 = 0.0;
        var_qlo__blk271_rdn5 = 0.0;
        var_qlo__blk271_rdb0 = 0.0;
        var_qlo__blk271_rdb1 = 0.0;

        let (assign4130_e4110, assign4130_e4110_d_n0, assign4130_e4110_d_n1, assign4130_e4110_d_n2, assign4130_e4110_d_n3, assign4130_e4110_d_n4, assign4130_e4110_d_n5, assign4130_e4110_d_b0, assign4130_e4110_d_b1,) = {
    if ((((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) && (var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk272, var_qhi__blk272_dn0, var_qhi__blk272_dn1, var_qhi__blk272_dn2, var_qhi__blk272_dn3, var_qhi__blk272_dn4, var_qhi__blk272_dn5, var_qhi__blk272_db0, var_qhi__blk272_db1,)
    }
};
        var_qhi__blk272 = assign4130_e4110;
        var_qhi__blk272_dn0 = assign4130_e4110_d_n0;
        var_qhi__blk272_dn1 = assign4130_e4110_d_n1;
        var_qhi__blk272_dn2 = assign4130_e4110_d_n2;
        var_qhi__blk272_dn3 = assign4130_e4110_d_n3;
        var_qhi__blk272_dn4 = assign4130_e4110_d_n4;
        var_qhi__blk272_dn5 = assign4130_e4110_d_n5;
        var_qhi__blk272_db0 = assign4130_e4110_d_b0;
        var_qhi__blk272_db1 = assign4130_e4110_d_b1;
        var_qhi__blk272_rv = 0.0;
        var_qhi__blk272_rdn0 = 0.0;
        var_qhi__blk272_rdn1 = 0.0;
        var_qhi__blk272_rdn2 = 0.0;
        var_qhi__blk272_rdn3 = 0.0;
        var_qhi__blk272_rdn4 = 0.0;
        var_qhi__blk272_rdn5 = 0.0;
        var_qhi__blk272_rdb0 = 0.0;
        var_qhi__blk272_rdb1 = 0.0;

        let (assign4140_e4120, assign4140_e4120_d_n0, assign4140_e4120_d_n1, assign4140_e4120_d_n2, assign4140_e4120_d_n3, assign4140_e4120_d_n4, assign4140_e4120_d_n5, assign4140_e4120_d_b0, assign4140_e4120_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 != 0.0)) {
        let assign4140_e4118: f64 = (var_qlo__blk271 + var_qhi__blk272);
        (assign4140_e4118, (var_qlo__blk271_dn0 + var_qhi__blk272_dn0), (var_qlo__blk271_dn1 + var_qhi__blk272_dn1), (var_qlo__blk271_dn2 + var_qhi__blk272_dn2), (var_qlo__blk271_dn3 + var_qhi__blk272_dn3), (var_qlo__blk271_dn4 + var_qhi__blk272_dn4), (var_qlo__blk271_dn5 + var_qhi__blk272_dn5), (var_qlo__blk271_db0 + var_qhi__blk272_db0), (var_qlo__blk271_db1 + var_qhi__blk272_db1),)
    } else {
        (var_argp, var_argp_dn0, var_argp_dn1, var_argp_dn2, var_argp_dn3, var_argp_dn4, var_argp_dn5, var_argp_db0, var_argp_db1,)
    }
};
        var_argp = assign4140_e4120;
        var_argp_dn0 = assign4140_e4120_d_n0;
        var_argp_dn1 = assign4140_e4120_d_n1;
        var_argp_dn2 = assign4140_e4120_d_n2;
        var_argp_dn3 = assign4140_e4120_d_n3;
        var_argp_dn4 = assign4140_e4120_d_n4;
        var_argp_dn5 = assign4140_e4120_d_n5;
        var_argp_db0 = assign4140_e4120_d_b0;
        var_argp_db1 = assign4140_e4120_d_b1;
        var_argp_rv = 0.0;
        var_argp_rdn0 = 0.0;
        var_argp_rdn1 = 0.0;
        var_argp_rdn2 = 0.0;
        var_argp_rdn3 = 0.0;
        var_argp_rdn4 = 0.0;
        var_argp_rdn5 = 0.0;
        var_argp_rdb0 = 0.0;
        var_argp_rdb1 = 0.0;

        let (assign4150_e4138, assign4150_e4138_d_n0, assign4150_e4138_d_n1, assign4150_e4138_d_n2, assign4150_e4138_d_n3, assign4150_e4138_d_n4, assign4150_e4138_d_n5, assign4150_e4138_d_b0, assign4150_e4138_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4150_e4129: f64 = (var_dv0__blk268 * var_dv0__blk268);
        let assign4150_e4132: f64 = (4.0 * p.p82);
        let assign4150_e4134: f64 = (assign4150_e4132 * p.p82);
        let assign4150_e4135: f64 = (assign4150_e4129 + assign4150_e4134);
        let assign4150_e4136: f64 = (assign4150_e4135).sqrt();
        (assign4150_e4136, (((var_dv0__blk268_dn0 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn0)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_dn1 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn1)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_dn2 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn2)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_dn3 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn3)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_dn4 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn4)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_dn5 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_dn5)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_db0 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_db0)) / (2.0 * assign4150_e4136)), (((var_dv0__blk268_db1 * var_dv0__blk268) + (var_dv0__blk268 * var_dv0__blk268_db1)) / (2.0 * assign4150_e4136)),)
    } else {
        (var_mv0__blk273, var_mv0__blk273_dn0, var_mv0__blk273_dn1, var_mv0__blk273_dn2, var_mv0__blk273_dn3, var_mv0__blk273_dn4, var_mv0__blk273_dn5, var_mv0__blk273_db0, var_mv0__blk273_db1,)
    }
};
        var_mv0__blk273 = assign4150_e4138;
        var_mv0__blk273_dn0 = assign4150_e4138_d_n0;
        var_mv0__blk273_dn1 = assign4150_e4138_d_n1;
        var_mv0__blk273_dn2 = assign4150_e4138_d_n2;
        var_mv0__blk273_dn3 = assign4150_e4138_d_n3;
        var_mv0__blk273_dn4 = assign4150_e4138_d_n4;
        var_mv0__blk273_dn5 = assign4150_e4138_d_n5;
        var_mv0__blk273_db0 = assign4150_e4138_d_b0;
        var_mv0__blk273_db1 = assign4150_e4138_d_b1;
        var_mv0__blk273_rv = 0.0;
        var_mv0__blk273_rdn0 = 0.0;
        var_mv0__blk273_rdn1 = 0.0;
        var_mv0__blk273_rdn2 = 0.0;
        var_mv0__blk273_rdn3 = 0.0;
        var_mv0__blk273_rdn4 = 0.0;
        var_mv0__blk273_rdn5 = 0.0;
        var_mv0__blk273_rdb0 = 0.0;
        var_mv0__blk273_rdb1 = 0.0;

        *var_arga_slot = var_arga;
        *var_arga_db0_slot = var_arga_db0;
        *var_arga_db1_slot = var_arga_db1;
        *var_arga_dn0_slot = var_arga_dn0;
        *var_arga_dn1_slot = var_arga_dn1;
        *var_arga_dn2_slot = var_arga_dn2;
        *var_arga_dn3_slot = var_arga_dn3;
        *var_arga_dn4_slot = var_arga_dn4;
        *var_arga_dn5_slot = var_arga_dn5;
        *var_arga_rdb0_slot = var_arga_rdb0;
        *var_arga_rdb1_slot = var_arga_rdb1;
        *var_arga_rdn0_slot = var_arga_rdn0;
        *var_arga_rdn1_slot = var_arga_rdn1;
        *var_arga_rdn2_slot = var_arga_rdn2;
        *var_arga_rdn3_slot = var_arga_rdn3;
        *var_arga_rdn4_slot = var_arga_rdn4;
        *var_arga_rdn5_slot = var_arga_rdn5;
        *var_arga_rv_slot = var_arga_rv;
        *var_argp_slot = var_argp;
        *var_argp_db0_slot = var_argp_db0;
        *var_argp_db1_slot = var_argp_db1;
        *var_argp_dn0_slot = var_argp_dn0;
        *var_argp_dn1_slot = var_argp_dn1;
        *var_argp_dn2_slot = var_argp_dn2;
        *var_argp_dn3_slot = var_argp_dn3;
        *var_argp_dn4_slot = var_argp_dn4;
        *var_argp_dn5_slot = var_argp_dn5;
        *var_argp_rdb0_slot = var_argp_rdb0;
        *var_argp_rdb1_slot = var_argp_rdb1;
        *var_argp_rdn0_slot = var_argp_rdn0;
        *var_argp_rdn1_slot = var_argp_rdn1;
        *var_argp_rdn2_slot = var_argp_rdn2;
        *var_argp_rdn3_slot = var_argp_rdn3;
        *var_argp_rdn4_slot = var_argp_rdn4;
        *var_argp_rdn5_slot = var_argp_rdn5;
        *var_argp_rv_slot = var_argp_rv;
        *var_dv0__blk268_slot = var_dv0__blk268;
        *var_dv0__blk268_db0_slot = var_dv0__blk268_db0;
        *var_dv0__blk268_db1_slot = var_dv0__blk268_db1;
        *var_dv0__blk268_dn0_slot = var_dv0__blk268_dn0;
        *var_dv0__blk268_dn1_slot = var_dv0__blk268_dn1;
        *var_dv0__blk268_dn2_slot = var_dv0__blk268_dn2;
        *var_dv0__blk268_dn3_slot = var_dv0__blk268_dn3;
        *var_dv0__blk268_dn4_slot = var_dv0__blk268_dn4;
        *var_dv0__blk268_dn5_slot = var_dv0__blk268_dn5;
        *var_dv0__blk268_rdb0_slot = var_dv0__blk268_rdb0;
        *var_dv0__blk268_rdb1_slot = var_dv0__blk268_rdb1;
        *var_dv0__blk268_rdn0_slot = var_dv0__blk268_rdn0;
        *var_dv0__blk268_rdn1_slot = var_dv0__blk268_rdn1;
        *var_dv0__blk268_rdn2_slot = var_dv0__blk268_rdn2;
        *var_dv0__blk268_rdn3_slot = var_dv0__blk268_rdn3;
        *var_dv0__blk268_rdn4_slot = var_dv0__blk268_rdn4;
        *var_dv0__blk268_rdn5_slot = var_dv0__blk268_rdn5;
        *var_dv0__blk268_rv_slot = var_dv0__blk268_rv;
        *var_dvh__blk269_slot = var_dvh__blk269;
        *var_dvh__blk269_db0_slot = var_dvh__blk269_db0;
        *var_dvh__blk269_db1_slot = var_dvh__blk269_db1;
        *var_dvh__blk269_dn0_slot = var_dvh__blk269_dn0;
        *var_dvh__blk269_dn1_slot = var_dvh__blk269_dn1;
        *var_dvh__blk269_dn2_slot = var_dvh__blk269_dn2;
        *var_dvh__blk269_dn3_slot = var_dvh__blk269_dn3;
        *var_dvh__blk269_dn4_slot = var_dvh__blk269_dn4;
        *var_dvh__blk269_dn5_slot = var_dvh__blk269_dn5;
        *var_dvh__blk269_rdb0_slot = var_dvh__blk269_rdb0;
        *var_dvh__blk269_rdb1_slot = var_dvh__blk269_rdb1;
        *var_dvh__blk269_rdn0_slot = var_dvh__blk269_rdn0;
        *var_dvh__blk269_rdn1_slot = var_dvh__blk269_rdn1;
        *var_dvh__blk269_rdn2_slot = var_dvh__blk269_rdn2;
        *var_dvh__blk269_rdn3_slot = var_dvh__blk269_rdn3;
        *var_dvh__blk269_rdn4_slot = var_dvh__blk269_rdn4;
        *var_dvh__blk269_rdn5_slot = var_dvh__blk269_rdn5;
        *var_dvh__blk269_rv_slot = var_dvh__blk269_rv;
        *var_guard267_slot = var_guard267;
        *var_guard267_db0_slot = var_guard267_db0;
        *var_guard267_db1_slot = var_guard267_db1;
        *var_guard267_dn0_slot = var_guard267_dn0;
        *var_guard267_dn1_slot = var_guard267_dn1;
        *var_guard267_dn2_slot = var_guard267_dn2;
        *var_guard267_dn3_slot = var_guard267_dn3;
        *var_guard267_dn4_slot = var_guard267_dn4;
        *var_guard267_dn5_slot = var_guard267_dn5;
        *var_guard267_rdb0_slot = var_guard267_rdb0;
        *var_guard267_rdb1_slot = var_guard267_rdb1;
        *var_guard267_rdn0_slot = var_guard267_rdn0;
        *var_guard267_rdn1_slot = var_guard267_rdn1;
        *var_guard267_rdn2_slot = var_guard267_rdn2;
        *var_guard267_rdn3_slot = var_guard267_rdn3;
        *var_guard267_rdn4_slot = var_guard267_rdn4;
        *var_guard267_rdn5_slot = var_guard267_rdn5;
        *var_guard267_rv_slot = var_guard267_rv;
        *var_guard278_slot = var_guard278;
        *var_guard278_db0_slot = var_guard278_db0;
        *var_guard278_db1_slot = var_guard278_db1;
        *var_guard278_dn0_slot = var_guard278_dn0;
        *var_guard278_dn1_slot = var_guard278_dn1;
        *var_guard278_dn2_slot = var_guard278_dn2;
        *var_guard278_dn3_slot = var_guard278_dn3;
        *var_guard278_dn4_slot = var_guard278_dn4;
        *var_guard278_dn5_slot = var_guard278_dn5;
        *var_guard278_rdb0_slot = var_guard278_rdb0;
        *var_guard278_rdb1_slot = var_guard278_rdb1;
        *var_guard278_rdn0_slot = var_guard278_rdn0;
        *var_guard278_rdn1_slot = var_guard278_rdn1;
        *var_guard278_rdn2_slot = var_guard278_rdn2;
        *var_guard278_rdn3_slot = var_guard278_rdn3;
        *var_guard278_rdn4_slot = var_guard278_rdn4;
        *var_guard278_rdn5_slot = var_guard278_rdn5;
        *var_guard278_rv_slot = var_guard278_rv;
        *var_guard279_slot = var_guard279;
        *var_guard279_db0_slot = var_guard279_db0;
        *var_guard279_db1_slot = var_guard279_db1;
        *var_guard279_dn0_slot = var_guard279_dn0;
        *var_guard279_dn1_slot = var_guard279_dn1;
        *var_guard279_dn2_slot = var_guard279_dn2;
        *var_guard279_dn3_slot = var_guard279_dn3;
        *var_guard279_dn4_slot = var_guard279_dn4;
        *var_guard279_dn5_slot = var_guard279_dn5;
        *var_guard279_rdb0_slot = var_guard279_rdb0;
        *var_guard279_rdb1_slot = var_guard279_rdb1;
        *var_guard279_rdn0_slot = var_guard279_rdn0;
        *var_guard279_rdn1_slot = var_guard279_rdn1;
        *var_guard279_rdn2_slot = var_guard279_rdn2;
        *var_guard279_rdn3_slot = var_guard279_rdn3;
        *var_guard279_rdn4_slot = var_guard279_rdn4;
        *var_guard279_rdn5_slot = var_guard279_rdn5;
        *var_guard279_rv_slot = var_guard279_rv;
        *var_mv_slot = var_mv;
        *var_mv0__blk273_slot = var_mv0__blk273;
        *var_mv0__blk273_db0_slot = var_mv0__blk273_db0;
        *var_mv0__blk273_db1_slot = var_mv0__blk273_db1;
        *var_mv0__blk273_dn0_slot = var_mv0__blk273_dn0;
        *var_mv0__blk273_dn1_slot = var_mv0__blk273_dn1;
        *var_mv0__blk273_dn2_slot = var_mv0__blk273_dn2;
        *var_mv0__blk273_dn3_slot = var_mv0__blk273_dn3;
        *var_mv0__blk273_dn4_slot = var_mv0__blk273_dn4;
        *var_mv0__blk273_dn5_slot = var_mv0__blk273_dn5;
        *var_mv0__blk273_rdb0_slot = var_mv0__blk273_rdb0;
        *var_mv0__blk273_rdb1_slot = var_mv0__blk273_rdb1;
        *var_mv0__blk273_rdn0_slot = var_mv0__blk273_rdn0;
        *var_mv0__blk273_rdn1_slot = var_mv0__blk273_rdn1;
        *var_mv0__blk273_rdn2_slot = var_mv0__blk273_rdn2;
        *var_mv0__blk273_rdn3_slot = var_mv0__blk273_rdn3;
        *var_mv0__blk273_rdn4_slot = var_mv0__blk273_rdn4;
        *var_mv0__blk273_rdn5_slot = var_mv0__blk273_rdn5;
        *var_mv0__blk273_rv_slot = var_mv0__blk273_rv;
        *var_mv_db0_slot = var_mv_db0;
        *var_mv_db1_slot = var_mv_db1;
        *var_mv_dn0_slot = var_mv_dn0;
        *var_mv_dn1_slot = var_mv_dn1;
        *var_mv_dn2_slot = var_mv_dn2;
        *var_mv_dn3_slot = var_mv_dn3;
        *var_mv_dn4_slot = var_mv_dn4;
        *var_mv_dn5_slot = var_mv_dn5;
        *var_mv_rdb0_slot = var_mv_rdb0;
        *var_mv_rdb1_slot = var_mv_rdb1;
        *var_mv_rdn0_slot = var_mv_rdn0;
        *var_mv_rdn1_slot = var_mv_rdn1;
        *var_mv_rdn2_slot = var_mv_rdn2;
        *var_mv_rdn3_slot = var_mv_rdn3;
        *var_mv_rdn4_slot = var_mv_rdn4;
        *var_mv_rdn5_slot = var_mv_rdn5;
        *var_mv_rv_slot = var_mv_rv;
        *var_pwq__blk270_slot = var_pwq__blk270;
        *var_pwq__blk270_db0_slot = var_pwq__blk270_db0;
        *var_pwq__blk270_db1_slot = var_pwq__blk270_db1;
        *var_pwq__blk270_dn0_slot = var_pwq__blk270_dn0;
        *var_pwq__blk270_dn1_slot = var_pwq__blk270_dn1;
        *var_pwq__blk270_dn2_slot = var_pwq__blk270_dn2;
        *var_pwq__blk270_dn3_slot = var_pwq__blk270_dn3;
        *var_pwq__blk270_dn4_slot = var_pwq__blk270_dn4;
        *var_pwq__blk270_dn5_slot = var_pwq__blk270_dn5;
        *var_pwq__blk270_rdb0_slot = var_pwq__blk270_rdb0;
        *var_pwq__blk270_rdb1_slot = var_pwq__blk270_rdb1;
        *var_pwq__blk270_rdn0_slot = var_pwq__blk270_rdn0;
        *var_pwq__blk270_rdn1_slot = var_pwq__blk270_rdn1;
        *var_pwq__blk270_rdn2_slot = var_pwq__blk270_rdn2;
        *var_pwq__blk270_rdn3_slot = var_pwq__blk270_rdn3;
        *var_pwq__blk270_rdn4_slot = var_pwq__blk270_rdn4;
        *var_pwq__blk270_rdn5_slot = var_pwq__blk270_rdn5;
        *var_pwq__blk270_rv_slot = var_pwq__blk270_rv;
        *var_qhi__blk272_slot = var_qhi__blk272;
        *var_qhi__blk272_db0_slot = var_qhi__blk272_db0;
        *var_qhi__blk272_db1_slot = var_qhi__blk272_db1;
        *var_qhi__blk272_dn0_slot = var_qhi__blk272_dn0;
        *var_qhi__blk272_dn1_slot = var_qhi__blk272_dn1;
        *var_qhi__blk272_dn2_slot = var_qhi__blk272_dn2;
        *var_qhi__blk272_dn3_slot = var_qhi__blk272_dn3;
        *var_qhi__blk272_dn4_slot = var_qhi__blk272_dn4;
        *var_qhi__blk272_dn5_slot = var_qhi__blk272_dn5;
        *var_qhi__blk272_rdb0_slot = var_qhi__blk272_rdb0;
        *var_qhi__blk272_rdb1_slot = var_qhi__blk272_rdb1;
        *var_qhi__blk272_rdn0_slot = var_qhi__blk272_rdn0;
        *var_qhi__blk272_rdn1_slot = var_qhi__blk272_rdn1;
        *var_qhi__blk272_rdn2_slot = var_qhi__blk272_rdn2;
        *var_qhi__blk272_rdn3_slot = var_qhi__blk272_rdn3;
        *var_qhi__blk272_rdn4_slot = var_qhi__blk272_rdn4;
        *var_qhi__blk272_rdn5_slot = var_qhi__blk272_rdn5;
        *var_qhi__blk272_rv_slot = var_qhi__blk272_rv;
        *var_qlo_slot = var_qlo;
        *var_qlo__blk271_slot = var_qlo__blk271;
        *var_qlo__blk271_db0_slot = var_qlo__blk271_db0;
        *var_qlo__blk271_db1_slot = var_qlo__blk271_db1;
        *var_qlo__blk271_dn0_slot = var_qlo__blk271_dn0;
        *var_qlo__blk271_dn1_slot = var_qlo__blk271_dn1;
        *var_qlo__blk271_dn2_slot = var_qlo__blk271_dn2;
        *var_qlo__blk271_dn3_slot = var_qlo__blk271_dn3;
        *var_qlo__blk271_dn4_slot = var_qlo__blk271_dn4;
        *var_qlo__blk271_dn5_slot = var_qlo__blk271_dn5;
        *var_qlo__blk271_rdb0_slot = var_qlo__blk271_rdb0;
        *var_qlo__blk271_rdb1_slot = var_qlo__blk271_rdb1;
        *var_qlo__blk271_rdn0_slot = var_qlo__blk271_rdn0;
        *var_qlo__blk271_rdn1_slot = var_qlo__blk271_rdn1;
        *var_qlo__blk271_rdn2_slot = var_qlo__blk271_rdn2;
        *var_qlo__blk271_rdn3_slot = var_qlo__blk271_rdn3;
        *var_qlo__blk271_rdn4_slot = var_qlo__blk271_rdn4;
        *var_qlo__blk271_rdn5_slot = var_qlo__blk271_rdn5;
        *var_qlo__blk271_rv_slot = var_qlo__blk271_rv;
        *var_qlo_db0_slot = var_qlo_db0;
        *var_qlo_db1_slot = var_qlo_db1;
        *var_qlo_dn0_slot = var_qlo_dn0;
        *var_qlo_dn1_slot = var_qlo_dn1;
        *var_qlo_dn2_slot = var_qlo_dn2;
        *var_qlo_dn3_slot = var_qlo_dn3;
        *var_qlo_dn4_slot = var_qlo_dn4;
        *var_qlo_dn5_slot = var_qlo_dn5;
        *var_qlo_rdb0_slot = var_qlo_rdb0;
        *var_qlo_rdb1_slot = var_qlo_rdb1;
        *var_qlo_rdn0_slot = var_qlo_rdn0;
        *var_qlo_rdn1_slot = var_qlo_rdn1;
        *var_qlo_rdn2_slot = var_qlo_rdn2;
        *var_qlo_rdn3_slot = var_qlo_rdn3;
        *var_qlo_rdn4_slot = var_qlo_rdn4;
        *var_qlo_rdn5_slot = var_qlo_rdn5;
        *var_qlo_rv_slot = var_qlo_rv;
        *var_vl_slot = var_vl;
        *var_vl_db0_slot = var_vl_db0;
        *var_vl_db1_slot = var_vl_db1;
        *var_vl_dn0_slot = var_vl_dn0;
        *var_vl_dn1_slot = var_vl_dn1;
        *var_vl_dn2_slot = var_vl_dn2;
        *var_vl_dn3_slot = var_vl_dn3;
        *var_vl_dn4_slot = var_vl_dn4;
        *var_vl_dn5_slot = var_vl_dn5;
        *var_vl_rdb0_slot = var_vl_rdb0;
        *var_vl_rdb1_slot = var_vl_rdb1;
        *var_vl_rdn0_slot = var_vl_rdn0;
        *var_vl_rdn1_slot = var_vl_rdn1;
        *var_vl_rdn2_slot = var_vl_rdn2;
        *var_vl_rdn3_slot = var_vl_rdn3;
        *var_vl_rdn4_slot = var_vl_rdn4;
        *var_vl_rdn5_slot = var_vl_rdn5;
        *var_vl_rv_slot = var_vl_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_a2_um2: f64,
        var_a2_um2_db0: f64,
        var_a2_um2_db1: f64,
        var_a2_um2_dn0: f64,
        var_a2_um2_dn1: f64,
        var_a2_um2_dn2: f64,
        var_a2_um2_dn3: f64,
        var_a2_um2_dn4: f64,
        var_a2_um2_dn5: f64,
        var_acja: f64,
        var_acja_db0: f64,
        var_acja_db1: f64,
        var_acja_dn0: f64,
        var_acja_dn1: f64,
        var_acja_dn2: f64,
        var_acja_dn3: f64,
        var_acja_dn4: f64,
        var_acja_dn5: f64,
        var_arga: f64,
        var_arga_db0: f64,
        var_arga_db1: f64,
        var_arga_dn0: f64,
        var_arga_dn1: f64,
        var_arga_dn2: f64,
        var_arga_dn3: f64,
        var_arga_dn4: f64,
        var_arga_dn5: f64,
        var_cj2: f64,
        var_cja_t: f64,
        var_cja_t_db0: f64,
        var_cja_t_db1: f64,
        var_cja_t_dn0: f64,
        var_cja_t_dn1: f64,
        var_cja_t_dn2: f64,
        var_cja_t_dn3: f64,
        var_cja_t_dn4: f64,
        var_cja_t_dn5: f64,
        var_cjp_t: f64,
        var_cjp_t_db0: f64,
        var_cjp_t_db1: f64,
        var_cjp_t_dn0: f64,
        var_cjp_t_dn1: f64,
        var_cjp_t_dn2: f64,
        var_cjp_t_dn3: f64,
        var_cjp_t_dn4: f64,
        var_cjp_t_dn5: f64,
        var_dv0__blk268: f64,
        var_dv0__blk268_db0: f64,
        var_dv0__blk268_db1: f64,
        var_dv0__blk268_dn0: f64,
        var_dv0__blk268_dn1: f64,
        var_dv0__blk268_dn2: f64,
        var_dv0__blk268_dn3: f64,
        var_dv0__blk268_dn4: f64,
        var_dv0__blk268_dn5: f64,
        var_guard249: f64,
        var_guard267: f64,
        var_guard278: f64,
        var_mv0__blk273: f64,
        var_mv0__blk273_db0: f64,
        var_mv0__blk273_db1: f64,
        var_mv0__blk273_dn0: f64,
        var_mv0__blk273_dn1: f64,
        var_mv0__blk273_dn2: f64,
        var_mv0__blk273_dn3: f64,
        var_mv0__blk273_dn4: f64,
        var_mv0__blk273_dn5: f64,
        var_p2_um: f64,
        var_p2_um_db0: f64,
        var_p2_um_db1: f64,
        var_p2_um_dn0: f64,
        var_p2_um_dn1: f64,
        var_p2_um_dn2: f64,
        var_p2_um_dn3: f64,
        var_p2_um_dn4: f64,
        var_p2_um_dn5: f64,
        var_pa_t: f64,
        var_pa_t_db0: f64,
        var_pa_t_db1: f64,
        var_pa_t_dn0: f64,
        var_pa_t_dn1: f64,
        var_pa_t_dn2: f64,
        var_pa_t_dn3: f64,
        var_pa_t_dn4: f64,
        var_pa_t_dn5: f64,
        var_pcjp: f64,
        var_pcjp_db0: f64,
        var_pcjp_db1: f64,
        var_pcjp_dn0: f64,
        var_pcjp_dn1: f64,
        var_pcjp_dn2: f64,
        var_pcjp_dn3: f64,
        var_pcjp_dn4: f64,
        var_pcjp_dn5: f64,
        var_pp_t: f64,
        var_pp_t_db0: f64,
        var_pp_t_db1: f64,
        var_pp_t_dn0: f64,
        var_pp_t_dn1: f64,
        var_pp_t_dn2: f64,
        var_pp_t_dn3: f64,
        var_pp_t_dn4: f64,
        var_pp_t_dn5: f64,
        var_vc2: f64,
        var_vc2_db0: f64,
        var_vc2_db1: f64,
        var_vc2_dn0: f64,
        var_vc2_dn1: f64,
        var_vc2_dn2: f64,
        var_vc2_dn3: f64,
        var_vc2_dn4: f64,
        var_vc2_dn5: f64,
        var_vpo: f64,
        var_vpo_db0: f64,
        var_vpo_db1: f64,
        var_vpo_dn0: f64,
        var_vpo_dn1: f64,
        var_vpo_dn2: f64,
        var_vpo_dn3: f64,
        var_vpo_dn4: f64,
        var_vpo_dn5: f64,
        var_acja__blk281_slot: &mut f64,
        var_acja__blk281_db0_slot: &mut f64,
        var_acja__blk281_db1_slot: &mut f64,
        var_acja__blk281_dn0_slot: &mut f64,
        var_acja__blk281_dn1_slot: &mut f64,
        var_acja__blk281_dn2_slot: &mut f64,
        var_acja__blk281_dn3_slot: &mut f64,
        var_acja__blk281_dn4_slot: &mut f64,
        var_acja__blk281_dn5_slot: &mut f64,
        var_acja__blk281_rdb0_slot: &mut f64,
        var_acja__blk281_rdb1_slot: &mut f64,
        var_acja__blk281_rdn0_slot: &mut f64,
        var_acja__blk281_rdn1_slot: &mut f64,
        var_acja__blk281_rdn2_slot: &mut f64,
        var_acja__blk281_rdn3_slot: &mut f64,
        var_acja__blk281_rdn4_slot: &mut f64,
        var_acja__blk281_rdn5_slot: &mut f64,
        var_acja__blk281_rv_slot: &mut f64,
        var_argp_slot: &mut f64,
        var_argp_db0_slot: &mut f64,
        var_argp_db1_slot: &mut f64,
        var_argp_dn0_slot: &mut f64,
        var_argp_dn1_slot: &mut f64,
        var_argp_dn2_slot: &mut f64,
        var_argp_dn3_slot: &mut f64,
        var_argp_dn4_slot: &mut f64,
        var_argp_dn5_slot: &mut f64,
        var_argp_rdb0_slot: &mut f64,
        var_argp_rdb1_slot: &mut f64,
        var_argp_rdn0_slot: &mut f64,
        var_argp_rdn1_slot: &mut f64,
        var_argp_rdn2_slot: &mut f64,
        var_argp_rdn3_slot: &mut f64,
        var_argp_rdn4_slot: &mut f64,
        var_argp_rdn5_slot: &mut f64,
        var_argp_rv_slot: &mut f64,
        var_dv0__blk286_slot: &mut f64,
        var_dv0__blk286_db0_slot: &mut f64,
        var_dv0__blk286_db1_slot: &mut f64,
        var_dv0__blk286_dn0_slot: &mut f64,
        var_dv0__blk286_dn1_slot: &mut f64,
        var_dv0__blk286_dn2_slot: &mut f64,
        var_dv0__blk286_dn3_slot: &mut f64,
        var_dv0__blk286_dn4_slot: &mut f64,
        var_dv0__blk286_dn5_slot: &mut f64,
        var_dv0__blk286_rdb0_slot: &mut f64,
        var_dv0__blk286_rdb1_slot: &mut f64,
        var_dv0__blk286_rdn0_slot: &mut f64,
        var_dv0__blk286_rdn1_slot: &mut f64,
        var_dv0__blk286_rdn2_slot: &mut f64,
        var_dv0__blk286_rdn3_slot: &mut f64,
        var_dv0__blk286_rdn4_slot: &mut f64,
        var_dv0__blk286_rdn5_slot: &mut f64,
        var_dv0__blk286_rv_slot: &mut f64,
        var_dv__blk275_slot: &mut f64,
        var_dv__blk275_db0_slot: &mut f64,
        var_dv__blk275_db1_slot: &mut f64,
        var_dv__blk275_dn0_slot: &mut f64,
        var_dv__blk275_dn1_slot: &mut f64,
        var_dv__blk275_dn2_slot: &mut f64,
        var_dv__blk275_dn3_slot: &mut f64,
        var_dv__blk275_dn4_slot: &mut f64,
        var_dv__blk275_dn5_slot: &mut f64,
        var_dv__blk275_rdb0_slot: &mut f64,
        var_dv__blk275_rdb1_slot: &mut f64,
        var_dv__blk275_rdn0_slot: &mut f64,
        var_dv__blk275_rdn1_slot: &mut f64,
        var_dv__blk275_rdn2_slot: &mut f64,
        var_dv__blk275_rdn3_slot: &mut f64,
        var_dv__blk275_rdn4_slot: &mut f64,
        var_dv__blk275_rdn5_slot: &mut f64,
        var_dv__blk275_rv_slot: &mut f64,
        var_dvh__blk287_slot: &mut f64,
        var_dvh__blk287_db0_slot: &mut f64,
        var_dvh__blk287_db1_slot: &mut f64,
        var_dvh__blk287_dn0_slot: &mut f64,
        var_dvh__blk287_dn1_slot: &mut f64,
        var_dvh__blk287_dn2_slot: &mut f64,
        var_dvh__blk287_dn3_slot: &mut f64,
        var_dvh__blk287_dn4_slot: &mut f64,
        var_dvh__blk287_dn5_slot: &mut f64,
        var_dvh__blk287_rdb0_slot: &mut f64,
        var_dvh__blk287_rdb1_slot: &mut f64,
        var_dvh__blk287_rdn0_slot: &mut f64,
        var_dvh__blk287_rdn1_slot: &mut f64,
        var_dvh__blk287_rdn2_slot: &mut f64,
        var_dvh__blk287_rdn3_slot: &mut f64,
        var_dvh__blk287_rdn4_slot: &mut f64,
        var_dvh__blk287_rdn5_slot: &mut f64,
        var_dvh__blk287_rv_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard280_db0_slot: &mut f64,
        var_guard280_db1_slot: &mut f64,
        var_guard280_dn0_slot: &mut f64,
        var_guard280_dn1_slot: &mut f64,
        var_guard280_dn2_slot: &mut f64,
        var_guard280_dn3_slot: &mut f64,
        var_guard280_dn4_slot: &mut f64,
        var_guard280_dn5_slot: &mut f64,
        var_guard280_rdb0_slot: &mut f64,
        var_guard280_rdb1_slot: &mut f64,
        var_guard280_rdn0_slot: &mut f64,
        var_guard280_rdn1_slot: &mut f64,
        var_guard280_rdn2_slot: &mut f64,
        var_guard280_rdn3_slot: &mut f64,
        var_guard280_rdn4_slot: &mut f64,
        var_guard280_rdn5_slot: &mut f64,
        var_guard280_rv_slot: &mut f64,
        var_guard285_slot: &mut f64,
        var_guard285_db0_slot: &mut f64,
        var_guard285_db1_slot: &mut f64,
        var_guard285_dn0_slot: &mut f64,
        var_guard285_dn1_slot: &mut f64,
        var_guard285_dn2_slot: &mut f64,
        var_guard285_dn3_slot: &mut f64,
        var_guard285_dn4_slot: &mut f64,
        var_guard285_dn5_slot: &mut f64,
        var_guard285_rdb0_slot: &mut f64,
        var_guard285_rdb1_slot: &mut f64,
        var_guard285_rdn0_slot: &mut f64,
        var_guard285_rdn1_slot: &mut f64,
        var_guard285_rdn2_slot: &mut f64,
        var_guard285_rdn3_slot: &mut f64,
        var_guard285_rdn4_slot: &mut f64,
        var_guard285_rdn5_slot: &mut f64,
        var_guard285_rv_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_guard296_db0_slot: &mut f64,
        var_guard296_db1_slot: &mut f64,
        var_guard296_dn0_slot: &mut f64,
        var_guard296_dn1_slot: &mut f64,
        var_guard296_dn2_slot: &mut f64,
        var_guard296_dn3_slot: &mut f64,
        var_guard296_dn4_slot: &mut f64,
        var_guard296_dn5_slot: &mut f64,
        var_guard296_rdb0_slot: &mut f64,
        var_guard296_rdb1_slot: &mut f64,
        var_guard296_rdn0_slot: &mut f64,
        var_guard296_rdn1_slot: &mut f64,
        var_guard296_rdn2_slot: &mut f64,
        var_guard296_rdn3_slot: &mut f64,
        var_guard296_rdn4_slot: &mut f64,
        var_guard296_rdn5_slot: &mut f64,
        var_guard296_rv_slot: &mut f64,
        var_mv__blk276_slot: &mut f64,
        var_mv__blk276_db0_slot: &mut f64,
        var_mv__blk276_db1_slot: &mut f64,
        var_mv__blk276_dn0_slot: &mut f64,
        var_mv__blk276_dn1_slot: &mut f64,
        var_mv__blk276_dn2_slot: &mut f64,
        var_mv__blk276_dn3_slot: &mut f64,
        var_mv__blk276_dn4_slot: &mut f64,
        var_mv__blk276_dn5_slot: &mut f64,
        var_mv__blk276_rdb0_slot: &mut f64,
        var_mv__blk276_rdb1_slot: &mut f64,
        var_mv__blk276_rdn0_slot: &mut f64,
        var_mv__blk276_rdn1_slot: &mut f64,
        var_mv__blk276_rdn2_slot: &mut f64,
        var_mv__blk276_rdn3_slot: &mut f64,
        var_mv__blk276_rdn4_slot: &mut f64,
        var_mv__blk276_rdn5_slot: &mut f64,
        var_mv__blk276_rv_slot: &mut f64,
        var_pcjp__blk282_slot: &mut f64,
        var_pcjp__blk282_db0_slot: &mut f64,
        var_pcjp__blk282_db1_slot: &mut f64,
        var_pcjp__blk282_dn0_slot: &mut f64,
        var_pcjp__blk282_dn1_slot: &mut f64,
        var_pcjp__blk282_dn2_slot: &mut f64,
        var_pcjp__blk282_dn3_slot: &mut f64,
        var_pcjp__blk282_dn4_slot: &mut f64,
        var_pcjp__blk282_dn5_slot: &mut f64,
        var_pcjp__blk282_rdb0_slot: &mut f64,
        var_pcjp__blk282_rdb1_slot: &mut f64,
        var_pcjp__blk282_rdn0_slot: &mut f64,
        var_pcjp__blk282_rdn1_slot: &mut f64,
        var_pcjp__blk282_rdn2_slot: &mut f64,
        var_pcjp__blk282_rdn3_slot: &mut f64,
        var_pcjp__blk282_rdn4_slot: &mut f64,
        var_pcjp__blk282_rdn5_slot: &mut f64,
        var_pcjp__blk282_rv_slot: &mut f64,
        var_qcp1_slot: &mut f64,
        var_qcp1_db0_slot: &mut f64,
        var_qcp1_db1_slot: &mut f64,
        var_qcp1_dn0_slot: &mut f64,
        var_qcp1_dn1_slot: &mut f64,
        var_qcp1_dn2_slot: &mut f64,
        var_qcp1_dn3_slot: &mut f64,
        var_qcp1_dn4_slot: &mut f64,
        var_qcp1_dn5_slot: &mut f64,
        var_qcp1_rdb0_slot: &mut f64,
        var_qcp1_rdb1_slot: &mut f64,
        var_qcp1_rdn0_slot: &mut f64,
        var_qcp1_rdn1_slot: &mut f64,
        var_qcp1_rdn2_slot: &mut f64,
        var_qcp1_rdn3_slot: &mut f64,
        var_qcp1_rdn4_slot: &mut f64,
        var_qcp1_rdn5_slot: &mut f64,
        var_qcp1_rv_slot: &mut f64,
        var_qlo__blk271_slot: &mut f64,
        var_qlo__blk271_db0_slot: &mut f64,
        var_qlo__blk271_db1_slot: &mut f64,
        var_qlo__blk271_dn0_slot: &mut f64,
        var_qlo__blk271_dn1_slot: &mut f64,
        var_qlo__blk271_dn2_slot: &mut f64,
        var_qlo__blk271_dn3_slot: &mut f64,
        var_qlo__blk271_dn4_slot: &mut f64,
        var_qlo__blk271_dn5_slot: &mut f64,
        var_qlo__blk271_rdb0_slot: &mut f64,
        var_qlo__blk271_rdb1_slot: &mut f64,
        var_qlo__blk271_rdn0_slot: &mut f64,
        var_qlo__blk271_rdn1_slot: &mut f64,
        var_qlo__blk271_rdn2_slot: &mut f64,
        var_qlo__blk271_rdn3_slot: &mut f64,
        var_qlo__blk271_rdn4_slot: &mut f64,
        var_qlo__blk271_rdn5_slot: &mut f64,
        var_qlo__blk271_rv_slot: &mut f64,
        var_vcl_slot: &mut f64,
        var_vcl_db0_slot: &mut f64,
        var_vcl_db1_slot: &mut f64,
        var_vcl_dn0_slot: &mut f64,
        var_vcl_dn1_slot: &mut f64,
        var_vcl_dn2_slot: &mut f64,
        var_vcl_dn3_slot: &mut f64,
        var_vcl_dn4_slot: &mut f64,
        var_vcl_dn5_slot: &mut f64,
        var_vcl_rdb0_slot: &mut f64,
        var_vcl_rdb1_slot: &mut f64,
        var_vcl_rdn0_slot: &mut f64,
        var_vcl_rdn1_slot: &mut f64,
        var_vcl_rdn2_slot: &mut f64,
        var_vcl_rdn3_slot: &mut f64,
        var_vcl_rdn4_slot: &mut f64,
        var_vcl_rdn5_slot: &mut f64,
        var_vcl_rv_slot: &mut f64,
        var_vl0__blk274_slot: &mut f64,
        var_vl0__blk274_db0_slot: &mut f64,
        var_vl0__blk274_db1_slot: &mut f64,
        var_vl0__blk274_dn0_slot: &mut f64,
        var_vl0__blk274_dn1_slot: &mut f64,
        var_vl0__blk274_dn2_slot: &mut f64,
        var_vl0__blk274_dn3_slot: &mut f64,
        var_vl0__blk274_dn4_slot: &mut f64,
        var_vl0__blk274_dn5_slot: &mut f64,
        var_vl0__blk274_rdb0_slot: &mut f64,
        var_vl0__blk274_rdb1_slot: &mut f64,
        var_vl0__blk274_rdn0_slot: &mut f64,
        var_vl0__blk274_rdn1_slot: &mut f64,
        var_vl0__blk274_rdn2_slot: &mut f64,
        var_vl0__blk274_rdn3_slot: &mut f64,
        var_vl0__blk274_rdn4_slot: &mut f64,
        var_vl0__blk274_rdn5_slot: &mut f64,
        var_vl0__blk274_rv_slot: &mut f64,
        var_vl__blk277_slot: &mut f64,
        var_vl__blk277_db0_slot: &mut f64,
        var_vl__blk277_db1_slot: &mut f64,
        var_vl__blk277_dn0_slot: &mut f64,
        var_vl__blk277_dn1_slot: &mut f64,
        var_vl__blk277_dn2_slot: &mut f64,
        var_vl__blk277_dn3_slot: &mut f64,
        var_vl__blk277_dn4_slot: &mut f64,
        var_vl__blk277_dn5_slot: &mut f64,
        var_vl__blk277_rdb0_slot: &mut f64,
        var_vl__blk277_rdb1_slot: &mut f64,
        var_vl__blk277_rdn0_slot: &mut f64,
        var_vl__blk277_rdn1_slot: &mut f64,
        var_vl__blk277_rdn2_slot: &mut f64,
        var_vl__blk277_rdn3_slot: &mut f64,
        var_vl__blk277_rdn4_slot: &mut f64,
        var_vl__blk277_rdn5_slot: &mut f64,
        var_vl__blk277_rv_slot: &mut f64,
    ) {
        let mut var_acja__blk281: f64 = *var_acja__blk281_slot;
        let mut var_acja__blk281_db0: f64 = *var_acja__blk281_db0_slot;
        let mut var_acja__blk281_db1: f64 = *var_acja__blk281_db1_slot;
        let mut var_acja__blk281_dn0: f64 = *var_acja__blk281_dn0_slot;
        let mut var_acja__blk281_dn1: f64 = *var_acja__blk281_dn1_slot;
        let mut var_acja__blk281_dn2: f64 = *var_acja__blk281_dn2_slot;
        let mut var_acja__blk281_dn3: f64 = *var_acja__blk281_dn3_slot;
        let mut var_acja__blk281_dn4: f64 = *var_acja__blk281_dn4_slot;
        let mut var_acja__blk281_dn5: f64 = *var_acja__blk281_dn5_slot;
        let mut var_acja__blk281_rdb0: f64 = *var_acja__blk281_rdb0_slot;
        let mut var_acja__blk281_rdb1: f64 = *var_acja__blk281_rdb1_slot;
        let mut var_acja__blk281_rdn0: f64 = *var_acja__blk281_rdn0_slot;
        let mut var_acja__blk281_rdn1: f64 = *var_acja__blk281_rdn1_slot;
        let mut var_acja__blk281_rdn2: f64 = *var_acja__blk281_rdn2_slot;
        let mut var_acja__blk281_rdn3: f64 = *var_acja__blk281_rdn3_slot;
        let mut var_acja__blk281_rdn4: f64 = *var_acja__blk281_rdn4_slot;
        let mut var_acja__blk281_rdn5: f64 = *var_acja__blk281_rdn5_slot;
        let mut var_acja__blk281_rv: f64 = *var_acja__blk281_rv_slot;
        let mut var_argp: f64 = *var_argp_slot;
        let mut var_argp_db0: f64 = *var_argp_db0_slot;
        let mut var_argp_db1: f64 = *var_argp_db1_slot;
        let mut var_argp_dn0: f64 = *var_argp_dn0_slot;
        let mut var_argp_dn1: f64 = *var_argp_dn1_slot;
        let mut var_argp_dn2: f64 = *var_argp_dn2_slot;
        let mut var_argp_dn3: f64 = *var_argp_dn3_slot;
        let mut var_argp_dn4: f64 = *var_argp_dn4_slot;
        let mut var_argp_dn5: f64 = *var_argp_dn5_slot;
        let mut var_argp_rdb0: f64 = *var_argp_rdb0_slot;
        let mut var_argp_rdb1: f64 = *var_argp_rdb1_slot;
        let mut var_argp_rdn0: f64 = *var_argp_rdn0_slot;
        let mut var_argp_rdn1: f64 = *var_argp_rdn1_slot;
        let mut var_argp_rdn2: f64 = *var_argp_rdn2_slot;
        let mut var_argp_rdn3: f64 = *var_argp_rdn3_slot;
        let mut var_argp_rdn4: f64 = *var_argp_rdn4_slot;
        let mut var_argp_rdn5: f64 = *var_argp_rdn5_slot;
        let mut var_argp_rv: f64 = *var_argp_rv_slot;
        let mut var_dv0__blk286: f64 = *var_dv0__blk286_slot;
        let mut var_dv0__blk286_db0: f64 = *var_dv0__blk286_db0_slot;
        let mut var_dv0__blk286_db1: f64 = *var_dv0__blk286_db1_slot;
        let mut var_dv0__blk286_dn0: f64 = *var_dv0__blk286_dn0_slot;
        let mut var_dv0__blk286_dn1: f64 = *var_dv0__blk286_dn1_slot;
        let mut var_dv0__blk286_dn2: f64 = *var_dv0__blk286_dn2_slot;
        let mut var_dv0__blk286_dn3: f64 = *var_dv0__blk286_dn3_slot;
        let mut var_dv0__blk286_dn4: f64 = *var_dv0__blk286_dn4_slot;
        let mut var_dv0__blk286_dn5: f64 = *var_dv0__blk286_dn5_slot;
        let mut var_dv0__blk286_rdb0: f64 = *var_dv0__blk286_rdb0_slot;
        let mut var_dv0__blk286_rdb1: f64 = *var_dv0__blk286_rdb1_slot;
        let mut var_dv0__blk286_rdn0: f64 = *var_dv0__blk286_rdn0_slot;
        let mut var_dv0__blk286_rdn1: f64 = *var_dv0__blk286_rdn1_slot;
        let mut var_dv0__blk286_rdn2: f64 = *var_dv0__blk286_rdn2_slot;
        let mut var_dv0__blk286_rdn3: f64 = *var_dv0__blk286_rdn3_slot;
        let mut var_dv0__blk286_rdn4: f64 = *var_dv0__blk286_rdn4_slot;
        let mut var_dv0__blk286_rdn5: f64 = *var_dv0__blk286_rdn5_slot;
        let mut var_dv0__blk286_rv: f64 = *var_dv0__blk286_rv_slot;
        let mut var_dv__blk275: f64 = *var_dv__blk275_slot;
        let mut var_dv__blk275_db0: f64 = *var_dv__blk275_db0_slot;
        let mut var_dv__blk275_db1: f64 = *var_dv__blk275_db1_slot;
        let mut var_dv__blk275_dn0: f64 = *var_dv__blk275_dn0_slot;
        let mut var_dv__blk275_dn1: f64 = *var_dv__blk275_dn1_slot;
        let mut var_dv__blk275_dn2: f64 = *var_dv__blk275_dn2_slot;
        let mut var_dv__blk275_dn3: f64 = *var_dv__blk275_dn3_slot;
        let mut var_dv__blk275_dn4: f64 = *var_dv__blk275_dn4_slot;
        let mut var_dv__blk275_dn5: f64 = *var_dv__blk275_dn5_slot;
        let mut var_dv__blk275_rdb0: f64 = *var_dv__blk275_rdb0_slot;
        let mut var_dv__blk275_rdb1: f64 = *var_dv__blk275_rdb1_slot;
        let mut var_dv__blk275_rdn0: f64 = *var_dv__blk275_rdn0_slot;
        let mut var_dv__blk275_rdn1: f64 = *var_dv__blk275_rdn1_slot;
        let mut var_dv__blk275_rdn2: f64 = *var_dv__blk275_rdn2_slot;
        let mut var_dv__blk275_rdn3: f64 = *var_dv__blk275_rdn3_slot;
        let mut var_dv__blk275_rdn4: f64 = *var_dv__blk275_rdn4_slot;
        let mut var_dv__blk275_rdn5: f64 = *var_dv__blk275_rdn5_slot;
        let mut var_dv__blk275_rv: f64 = *var_dv__blk275_rv_slot;
        let mut var_dvh__blk287: f64 = *var_dvh__blk287_slot;
        let mut var_dvh__blk287_db0: f64 = *var_dvh__blk287_db0_slot;
        let mut var_dvh__blk287_db1: f64 = *var_dvh__blk287_db1_slot;
        let mut var_dvh__blk287_dn0: f64 = *var_dvh__blk287_dn0_slot;
        let mut var_dvh__blk287_dn1: f64 = *var_dvh__blk287_dn1_slot;
        let mut var_dvh__blk287_dn2: f64 = *var_dvh__blk287_dn2_slot;
        let mut var_dvh__blk287_dn3: f64 = *var_dvh__blk287_dn3_slot;
        let mut var_dvh__blk287_dn4: f64 = *var_dvh__blk287_dn4_slot;
        let mut var_dvh__blk287_dn5: f64 = *var_dvh__blk287_dn5_slot;
        let mut var_dvh__blk287_rdb0: f64 = *var_dvh__blk287_rdb0_slot;
        let mut var_dvh__blk287_rdb1: f64 = *var_dvh__blk287_rdb1_slot;
        let mut var_dvh__blk287_rdn0: f64 = *var_dvh__blk287_rdn0_slot;
        let mut var_dvh__blk287_rdn1: f64 = *var_dvh__blk287_rdn1_slot;
        let mut var_dvh__blk287_rdn2: f64 = *var_dvh__blk287_rdn2_slot;
        let mut var_dvh__blk287_rdn3: f64 = *var_dvh__blk287_rdn3_slot;
        let mut var_dvh__blk287_rdn4: f64 = *var_dvh__blk287_rdn4_slot;
        let mut var_dvh__blk287_rdn5: f64 = *var_dvh__blk287_rdn5_slot;
        let mut var_dvh__blk287_rv: f64 = *var_dvh__blk287_rv_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard280_db0: f64 = *var_guard280_db0_slot;
        let mut var_guard280_db1: f64 = *var_guard280_db1_slot;
        let mut var_guard280_dn0: f64 = *var_guard280_dn0_slot;
        let mut var_guard280_dn1: f64 = *var_guard280_dn1_slot;
        let mut var_guard280_dn2: f64 = *var_guard280_dn2_slot;
        let mut var_guard280_dn3: f64 = *var_guard280_dn3_slot;
        let mut var_guard280_dn4: f64 = *var_guard280_dn4_slot;
        let mut var_guard280_dn5: f64 = *var_guard280_dn5_slot;
        let mut var_guard280_rdb0: f64 = *var_guard280_rdb0_slot;
        let mut var_guard280_rdb1: f64 = *var_guard280_rdb1_slot;
        let mut var_guard280_rdn0: f64 = *var_guard280_rdn0_slot;
        let mut var_guard280_rdn1: f64 = *var_guard280_rdn1_slot;
        let mut var_guard280_rdn2: f64 = *var_guard280_rdn2_slot;
        let mut var_guard280_rdn3: f64 = *var_guard280_rdn3_slot;
        let mut var_guard280_rdn4: f64 = *var_guard280_rdn4_slot;
        let mut var_guard280_rdn5: f64 = *var_guard280_rdn5_slot;
        let mut var_guard280_rv: f64 = *var_guard280_rv_slot;
        let mut var_guard285: f64 = *var_guard285_slot;
        let mut var_guard285_db0: f64 = *var_guard285_db0_slot;
        let mut var_guard285_db1: f64 = *var_guard285_db1_slot;
        let mut var_guard285_dn0: f64 = *var_guard285_dn0_slot;
        let mut var_guard285_dn1: f64 = *var_guard285_dn1_slot;
        let mut var_guard285_dn2: f64 = *var_guard285_dn2_slot;
        let mut var_guard285_dn3: f64 = *var_guard285_dn3_slot;
        let mut var_guard285_dn4: f64 = *var_guard285_dn4_slot;
        let mut var_guard285_dn5: f64 = *var_guard285_dn5_slot;
        let mut var_guard285_rdb0: f64 = *var_guard285_rdb0_slot;
        let mut var_guard285_rdb1: f64 = *var_guard285_rdb1_slot;
        let mut var_guard285_rdn0: f64 = *var_guard285_rdn0_slot;
        let mut var_guard285_rdn1: f64 = *var_guard285_rdn1_slot;
        let mut var_guard285_rdn2: f64 = *var_guard285_rdn2_slot;
        let mut var_guard285_rdn3: f64 = *var_guard285_rdn3_slot;
        let mut var_guard285_rdn4: f64 = *var_guard285_rdn4_slot;
        let mut var_guard285_rdn5: f64 = *var_guard285_rdn5_slot;
        let mut var_guard285_rv: f64 = *var_guard285_rv_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_guard296_db0: f64 = *var_guard296_db0_slot;
        let mut var_guard296_db1: f64 = *var_guard296_db1_slot;
        let mut var_guard296_dn0: f64 = *var_guard296_dn0_slot;
        let mut var_guard296_dn1: f64 = *var_guard296_dn1_slot;
        let mut var_guard296_dn2: f64 = *var_guard296_dn2_slot;
        let mut var_guard296_dn3: f64 = *var_guard296_dn3_slot;
        let mut var_guard296_dn4: f64 = *var_guard296_dn4_slot;
        let mut var_guard296_dn5: f64 = *var_guard296_dn5_slot;
        let mut var_guard296_rdb0: f64 = *var_guard296_rdb0_slot;
        let mut var_guard296_rdb1: f64 = *var_guard296_rdb1_slot;
        let mut var_guard296_rdn0: f64 = *var_guard296_rdn0_slot;
        let mut var_guard296_rdn1: f64 = *var_guard296_rdn1_slot;
        let mut var_guard296_rdn2: f64 = *var_guard296_rdn2_slot;
        let mut var_guard296_rdn3: f64 = *var_guard296_rdn3_slot;
        let mut var_guard296_rdn4: f64 = *var_guard296_rdn4_slot;
        let mut var_guard296_rdn5: f64 = *var_guard296_rdn5_slot;
        let mut var_guard296_rv: f64 = *var_guard296_rv_slot;
        let mut var_mv__blk276: f64 = *var_mv__blk276_slot;
        let mut var_mv__blk276_db0: f64 = *var_mv__blk276_db0_slot;
        let mut var_mv__blk276_db1: f64 = *var_mv__blk276_db1_slot;
        let mut var_mv__blk276_dn0: f64 = *var_mv__blk276_dn0_slot;
        let mut var_mv__blk276_dn1: f64 = *var_mv__blk276_dn1_slot;
        let mut var_mv__blk276_dn2: f64 = *var_mv__blk276_dn2_slot;
        let mut var_mv__blk276_dn3: f64 = *var_mv__blk276_dn3_slot;
        let mut var_mv__blk276_dn4: f64 = *var_mv__blk276_dn4_slot;
        let mut var_mv__blk276_dn5: f64 = *var_mv__blk276_dn5_slot;
        let mut var_mv__blk276_rdb0: f64 = *var_mv__blk276_rdb0_slot;
        let mut var_mv__blk276_rdb1: f64 = *var_mv__blk276_rdb1_slot;
        let mut var_mv__blk276_rdn0: f64 = *var_mv__blk276_rdn0_slot;
        let mut var_mv__blk276_rdn1: f64 = *var_mv__blk276_rdn1_slot;
        let mut var_mv__blk276_rdn2: f64 = *var_mv__blk276_rdn2_slot;
        let mut var_mv__blk276_rdn3: f64 = *var_mv__blk276_rdn3_slot;
        let mut var_mv__blk276_rdn4: f64 = *var_mv__blk276_rdn4_slot;
        let mut var_mv__blk276_rdn5: f64 = *var_mv__blk276_rdn5_slot;
        let mut var_mv__blk276_rv: f64 = *var_mv__blk276_rv_slot;
        let mut var_pcjp__blk282: f64 = *var_pcjp__blk282_slot;
        let mut var_pcjp__blk282_db0: f64 = *var_pcjp__blk282_db0_slot;
        let mut var_pcjp__blk282_db1: f64 = *var_pcjp__blk282_db1_slot;
        let mut var_pcjp__blk282_dn0: f64 = *var_pcjp__blk282_dn0_slot;
        let mut var_pcjp__blk282_dn1: f64 = *var_pcjp__blk282_dn1_slot;
        let mut var_pcjp__blk282_dn2: f64 = *var_pcjp__blk282_dn2_slot;
        let mut var_pcjp__blk282_dn3: f64 = *var_pcjp__blk282_dn3_slot;
        let mut var_pcjp__blk282_dn4: f64 = *var_pcjp__blk282_dn4_slot;
        let mut var_pcjp__blk282_dn5: f64 = *var_pcjp__blk282_dn5_slot;
        let mut var_pcjp__blk282_rdb0: f64 = *var_pcjp__blk282_rdb0_slot;
        let mut var_pcjp__blk282_rdb1: f64 = *var_pcjp__blk282_rdb1_slot;
        let mut var_pcjp__blk282_rdn0: f64 = *var_pcjp__blk282_rdn0_slot;
        let mut var_pcjp__blk282_rdn1: f64 = *var_pcjp__blk282_rdn1_slot;
        let mut var_pcjp__blk282_rdn2: f64 = *var_pcjp__blk282_rdn2_slot;
        let mut var_pcjp__blk282_rdn3: f64 = *var_pcjp__blk282_rdn3_slot;
        let mut var_pcjp__blk282_rdn4: f64 = *var_pcjp__blk282_rdn4_slot;
        let mut var_pcjp__blk282_rdn5: f64 = *var_pcjp__blk282_rdn5_slot;
        let mut var_pcjp__blk282_rv: f64 = *var_pcjp__blk282_rv_slot;
        let mut var_qcp1: f64 = *var_qcp1_slot;
        let mut var_qcp1_db0: f64 = *var_qcp1_db0_slot;
        let mut var_qcp1_db1: f64 = *var_qcp1_db1_slot;
        let mut var_qcp1_dn0: f64 = *var_qcp1_dn0_slot;
        let mut var_qcp1_dn1: f64 = *var_qcp1_dn1_slot;
        let mut var_qcp1_dn2: f64 = *var_qcp1_dn2_slot;
        let mut var_qcp1_dn3: f64 = *var_qcp1_dn3_slot;
        let mut var_qcp1_dn4: f64 = *var_qcp1_dn4_slot;
        let mut var_qcp1_dn5: f64 = *var_qcp1_dn5_slot;
        let mut var_qcp1_rdb0: f64 = *var_qcp1_rdb0_slot;
        let mut var_qcp1_rdb1: f64 = *var_qcp1_rdb1_slot;
        let mut var_qcp1_rdn0: f64 = *var_qcp1_rdn0_slot;
        let mut var_qcp1_rdn1: f64 = *var_qcp1_rdn1_slot;
        let mut var_qcp1_rdn2: f64 = *var_qcp1_rdn2_slot;
        let mut var_qcp1_rdn3: f64 = *var_qcp1_rdn3_slot;
        let mut var_qcp1_rdn4: f64 = *var_qcp1_rdn4_slot;
        let mut var_qcp1_rdn5: f64 = *var_qcp1_rdn5_slot;
        let mut var_qcp1_rv: f64 = *var_qcp1_rv_slot;
        let mut var_qlo__blk271: f64 = *var_qlo__blk271_slot;
        let mut var_qlo__blk271_db0: f64 = *var_qlo__blk271_db0_slot;
        let mut var_qlo__blk271_db1: f64 = *var_qlo__blk271_db1_slot;
        let mut var_qlo__blk271_dn0: f64 = *var_qlo__blk271_dn0_slot;
        let mut var_qlo__blk271_dn1: f64 = *var_qlo__blk271_dn1_slot;
        let mut var_qlo__blk271_dn2: f64 = *var_qlo__blk271_dn2_slot;
        let mut var_qlo__blk271_dn3: f64 = *var_qlo__blk271_dn3_slot;
        let mut var_qlo__blk271_dn4: f64 = *var_qlo__blk271_dn4_slot;
        let mut var_qlo__blk271_dn5: f64 = *var_qlo__blk271_dn5_slot;
        let mut var_qlo__blk271_rdb0: f64 = *var_qlo__blk271_rdb0_slot;
        let mut var_qlo__blk271_rdb1: f64 = *var_qlo__blk271_rdb1_slot;
        let mut var_qlo__blk271_rdn0: f64 = *var_qlo__blk271_rdn0_slot;
        let mut var_qlo__blk271_rdn1: f64 = *var_qlo__blk271_rdn1_slot;
        let mut var_qlo__blk271_rdn2: f64 = *var_qlo__blk271_rdn2_slot;
        let mut var_qlo__blk271_rdn3: f64 = *var_qlo__blk271_rdn3_slot;
        let mut var_qlo__blk271_rdn4: f64 = *var_qlo__blk271_rdn4_slot;
        let mut var_qlo__blk271_rdn5: f64 = *var_qlo__blk271_rdn5_slot;
        let mut var_qlo__blk271_rv: f64 = *var_qlo__blk271_rv_slot;
        let mut var_vcl: f64 = *var_vcl_slot;
        let mut var_vcl_db0: f64 = *var_vcl_db0_slot;
        let mut var_vcl_db1: f64 = *var_vcl_db1_slot;
        let mut var_vcl_dn0: f64 = *var_vcl_dn0_slot;
        let mut var_vcl_dn1: f64 = *var_vcl_dn1_slot;
        let mut var_vcl_dn2: f64 = *var_vcl_dn2_slot;
        let mut var_vcl_dn3: f64 = *var_vcl_dn3_slot;
        let mut var_vcl_dn4: f64 = *var_vcl_dn4_slot;
        let mut var_vcl_dn5: f64 = *var_vcl_dn5_slot;
        let mut var_vcl_rdb0: f64 = *var_vcl_rdb0_slot;
        let mut var_vcl_rdb1: f64 = *var_vcl_rdb1_slot;
        let mut var_vcl_rdn0: f64 = *var_vcl_rdn0_slot;
        let mut var_vcl_rdn1: f64 = *var_vcl_rdn1_slot;
        let mut var_vcl_rdn2: f64 = *var_vcl_rdn2_slot;
        let mut var_vcl_rdn3: f64 = *var_vcl_rdn3_slot;
        let mut var_vcl_rdn4: f64 = *var_vcl_rdn4_slot;
        let mut var_vcl_rdn5: f64 = *var_vcl_rdn5_slot;
        let mut var_vcl_rv: f64 = *var_vcl_rv_slot;
        let mut var_vl0__blk274: f64 = *var_vl0__blk274_slot;
        let mut var_vl0__blk274_db0: f64 = *var_vl0__blk274_db0_slot;
        let mut var_vl0__blk274_db1: f64 = *var_vl0__blk274_db1_slot;
        let mut var_vl0__blk274_dn0: f64 = *var_vl0__blk274_dn0_slot;
        let mut var_vl0__blk274_dn1: f64 = *var_vl0__blk274_dn1_slot;
        let mut var_vl0__blk274_dn2: f64 = *var_vl0__blk274_dn2_slot;
        let mut var_vl0__blk274_dn3: f64 = *var_vl0__blk274_dn3_slot;
        let mut var_vl0__blk274_dn4: f64 = *var_vl0__blk274_dn4_slot;
        let mut var_vl0__blk274_dn5: f64 = *var_vl0__blk274_dn5_slot;
        let mut var_vl0__blk274_rdb0: f64 = *var_vl0__blk274_rdb0_slot;
        let mut var_vl0__blk274_rdb1: f64 = *var_vl0__blk274_rdb1_slot;
        let mut var_vl0__blk274_rdn0: f64 = *var_vl0__blk274_rdn0_slot;
        let mut var_vl0__blk274_rdn1: f64 = *var_vl0__blk274_rdn1_slot;
        let mut var_vl0__blk274_rdn2: f64 = *var_vl0__blk274_rdn2_slot;
        let mut var_vl0__blk274_rdn3: f64 = *var_vl0__blk274_rdn3_slot;
        let mut var_vl0__blk274_rdn4: f64 = *var_vl0__blk274_rdn4_slot;
        let mut var_vl0__blk274_rdn5: f64 = *var_vl0__blk274_rdn5_slot;
        let mut var_vl0__blk274_rv: f64 = *var_vl0__blk274_rv_slot;
        let mut var_vl__blk277: f64 = *var_vl__blk277_slot;
        let mut var_vl__blk277_db0: f64 = *var_vl__blk277_db0_slot;
        let mut var_vl__blk277_db1: f64 = *var_vl__blk277_db1_slot;
        let mut var_vl__blk277_dn0: f64 = *var_vl__blk277_dn0_slot;
        let mut var_vl__blk277_dn1: f64 = *var_vl__blk277_dn1_slot;
        let mut var_vl__blk277_dn2: f64 = *var_vl__blk277_dn2_slot;
        let mut var_vl__blk277_dn3: f64 = *var_vl__blk277_dn3_slot;
        let mut var_vl__blk277_dn4: f64 = *var_vl__blk277_dn4_slot;
        let mut var_vl__blk277_dn5: f64 = *var_vl__blk277_dn5_slot;
        let mut var_vl__blk277_rdb0: f64 = *var_vl__blk277_rdb0_slot;
        let mut var_vl__blk277_rdb1: f64 = *var_vl__blk277_rdb1_slot;
        let mut var_vl__blk277_rdn0: f64 = *var_vl__blk277_rdn0_slot;
        let mut var_vl__blk277_rdn1: f64 = *var_vl__blk277_rdn1_slot;
        let mut var_vl__blk277_rdn2: f64 = *var_vl__blk277_rdn2_slot;
        let mut var_vl__blk277_rdn3: f64 = *var_vl__blk277_rdn3_slot;
        let mut var_vl__blk277_rdn4: f64 = *var_vl__blk277_rdn4_slot;
        let mut var_vl__blk277_rdn5: f64 = *var_vl__blk277_rdn5_slot;
        let mut var_vl__blk277_rv: f64 = *var_vl__blk277_rv_slot;

        let (assign4160_e4152, assign4160_e4152_d_n0, assign4160_e4152_d_n1, assign4160_e4152_d_n2, assign4160_e4152_d_n3, assign4160_e4152_d_n4, assign4160_e4152_d_n5, assign4160_e4152_d_b0, assign4160_e4152_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4160_e4146: f64 = (-0.5);
        let assign4160_e4149: f64 = (var_dv0__blk268 + var_mv0__blk273);
        let assign4160_e4150: f64 = (assign4160_e4146 * assign4160_e4149);
        (assign4160_e4150, (assign4160_e4146 * (var_dv0__blk268_dn0 + var_mv0__blk273_dn0)), (assign4160_e4146 * (var_dv0__blk268_dn1 + var_mv0__blk273_dn1)), (assign4160_e4146 * (var_dv0__blk268_dn2 + var_mv0__blk273_dn2)), (assign4160_e4146 * (var_dv0__blk268_dn3 + var_mv0__blk273_dn3)), (assign4160_e4146 * (var_dv0__blk268_dn4 + var_mv0__blk273_dn4)), (assign4160_e4146 * (var_dv0__blk268_dn5 + var_mv0__blk273_dn5)), (assign4160_e4146 * (var_dv0__blk268_db0 + var_mv0__blk273_db0)), (assign4160_e4146 * (var_dv0__blk268_db1 + var_mv0__blk273_db1)),)
    } else {
        (var_vl0__blk274, var_vl0__blk274_dn0, var_vl0__blk274_dn1, var_vl0__blk274_dn2, var_vl0__blk274_dn3, var_vl0__blk274_dn4, var_vl0__blk274_dn5, var_vl0__blk274_db0, var_vl0__blk274_db1,)
    }
};
        var_vl0__blk274 = assign4160_e4152;
        var_vl0__blk274_dn0 = assign4160_e4152_d_n0;
        var_vl0__blk274_dn1 = assign4160_e4152_d_n1;
        var_vl0__blk274_dn2 = assign4160_e4152_d_n2;
        var_vl0__blk274_dn3 = assign4160_e4152_d_n3;
        var_vl0__blk274_dn4 = assign4160_e4152_d_n4;
        var_vl0__blk274_dn5 = assign4160_e4152_d_n5;
        var_vl0__blk274_db0 = assign4160_e4152_d_b0;
        var_vl0__blk274_db1 = assign4160_e4152_d_b1;
        var_vl0__blk274_rv = 0.0;
        var_vl0__blk274_rdn0 = 0.0;
        var_vl0__blk274_rdn1 = 0.0;
        var_vl0__blk274_rdn2 = 0.0;
        var_vl0__blk274_rdn3 = 0.0;
        var_vl0__blk274_rdn4 = 0.0;
        var_vl0__blk274_rdn5 = 0.0;
        var_vl0__blk274_rdb0 = 0.0;
        var_vl0__blk274_rdb1 = 0.0;

        let (assign4170_e4163, assign4170_e4163_d_n0, assign4170_e4163_d_n1, assign4170_e4163_d_n2, assign4170_e4163_d_n3, assign4170_e4163_d_n4, assign4170_e4163_d_n5, assign4170_e4163_d_b0, assign4170_e4163_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4170_e4161: f64 = (var_vcl + var_dv0__blk268);
        (assign4170_e4161, (var_vcl_dn0 + var_dv0__blk268_dn0), (var_vcl_dn1 + var_dv0__blk268_dn1), (var_vcl_dn2 + var_dv0__blk268_dn2), (var_vcl_dn3 + var_dv0__blk268_dn3), (var_vcl_dn4 + var_dv0__blk268_dn4), (var_vcl_dn5 + var_dv0__blk268_dn5), (var_vcl_db0 + var_dv0__blk268_db0), (var_vcl_db1 + var_dv0__blk268_db1),)
    } else {
        (var_dv__blk275, var_dv__blk275_dn0, var_dv__blk275_dn1, var_dv__blk275_dn2, var_dv__blk275_dn3, var_dv__blk275_dn4, var_dv__blk275_dn5, var_dv__blk275_db0, var_dv__blk275_db1,)
    }
};
        var_dv__blk275 = assign4170_e4163;
        var_dv__blk275_dn0 = assign4170_e4163_d_n0;
        var_dv__blk275_dn1 = assign4170_e4163_d_n1;
        var_dv__blk275_dn2 = assign4170_e4163_d_n2;
        var_dv__blk275_dn3 = assign4170_e4163_d_n3;
        var_dv__blk275_dn4 = assign4170_e4163_d_n4;
        var_dv__blk275_dn5 = assign4170_e4163_d_n5;
        var_dv__blk275_db0 = assign4170_e4163_d_b0;
        var_dv__blk275_db1 = assign4170_e4163_d_b1;
        var_dv__blk275_rv = 0.0;
        var_dv__blk275_rdn0 = 0.0;
        var_dv__blk275_rdn1 = 0.0;
        var_dv__blk275_rdn2 = 0.0;
        var_dv__blk275_rdn3 = 0.0;
        var_dv__blk275_rdn4 = 0.0;
        var_dv__blk275_rdn5 = 0.0;
        var_dv__blk275_rdb0 = 0.0;
        var_dv__blk275_rdb1 = 0.0;

        let (assign4180_e4181, assign4180_e4181_d_n0, assign4180_e4181_d_n1, assign4180_e4181_d_n2, assign4180_e4181_d_n3, assign4180_e4181_d_n4, assign4180_e4181_d_n5, assign4180_e4181_d_b0, assign4180_e4181_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4180_e4172: f64 = (var_dv__blk275 * var_dv__blk275);
        let assign4180_e4175: f64 = (4.0 * p.p82);
        let assign4180_e4177: f64 = (assign4180_e4175 * p.p82);
        let assign4180_e4178: f64 = (assign4180_e4172 + assign4180_e4177);
        let assign4180_e4179: f64 = (assign4180_e4178).sqrt();
        (assign4180_e4179, (((var_dv__blk275_dn0 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn0)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_dn1 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn1)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_dn2 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn2)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_dn3 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn3)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_dn4 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn4)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_dn5 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_dn5)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_db0 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_db0)) / (2.0 * assign4180_e4179)), (((var_dv__blk275_db1 * var_dv__blk275) + (var_dv__blk275 * var_dv__blk275_db1)) / (2.0 * assign4180_e4179)),)
    } else {
        (var_mv__blk276, var_mv__blk276_dn0, var_mv__blk276_dn1, var_mv__blk276_dn2, var_mv__blk276_dn3, var_mv__blk276_dn4, var_mv__blk276_dn5, var_mv__blk276_db0, var_mv__blk276_db1,)
    }
};
        var_mv__blk276 = assign4180_e4181;
        var_mv__blk276_dn0 = assign4180_e4181_d_n0;
        var_mv__blk276_dn1 = assign4180_e4181_d_n1;
        var_mv__blk276_dn2 = assign4180_e4181_d_n2;
        var_mv__blk276_dn3 = assign4180_e4181_d_n3;
        var_mv__blk276_dn4 = assign4180_e4181_d_n4;
        var_mv__blk276_dn5 = assign4180_e4181_d_n5;
        var_mv__blk276_db0 = assign4180_e4181_d_b0;
        var_mv__blk276_db1 = assign4180_e4181_d_b1;
        var_mv__blk276_rv = 0.0;
        var_mv__blk276_rdn0 = 0.0;
        var_mv__blk276_rdn1 = 0.0;
        var_mv__blk276_rdn2 = 0.0;
        var_mv__blk276_rdn3 = 0.0;
        var_mv__blk276_rdn4 = 0.0;
        var_mv__blk276_rdn5 = 0.0;
        var_mv__blk276_rdb0 = 0.0;
        var_mv__blk276_rdb1 = 0.0;

        let (assign4190_e4196, assign4190_e4196_d_n0, assign4190_e4196_d_n1, assign4190_e4196_d_n2, assign4190_e4196_d_n3, assign4190_e4196_d_n4, assign4190_e4196_d_n5, assign4190_e4196_d_b0, assign4190_e4196_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4190_e4191: f64 = (var_dv__blk275 - var_mv__blk276);
        let assign4190_e4192: f64 = (0.5 * assign4190_e4191);
        let assign4190_e4194: f64 = (assign4190_e4192 - var_dv0__blk268);
        (assign4190_e4194, ((0.5 * (var_dv__blk275_dn0 - var_mv__blk276_dn0)) - var_dv0__blk268_dn0), ((0.5 * (var_dv__blk275_dn1 - var_mv__blk276_dn1)) - var_dv0__blk268_dn1), ((0.5 * (var_dv__blk275_dn2 - var_mv__blk276_dn2)) - var_dv0__blk268_dn2), ((0.5 * (var_dv__blk275_dn3 - var_mv__blk276_dn3)) - var_dv0__blk268_dn3), ((0.5 * (var_dv__blk275_dn4 - var_mv__blk276_dn4)) - var_dv0__blk268_dn4), ((0.5 * (var_dv__blk275_dn5 - var_mv__blk276_dn5)) - var_dv0__blk268_dn5), ((0.5 * (var_dv__blk275_db0 - var_mv__blk276_db0)) - var_dv0__blk268_db0), ((0.5 * (var_dv__blk275_db1 - var_mv__blk276_db1)) - var_dv0__blk268_db1),)
    } else {
        (var_vl__blk277, var_vl__blk277_dn0, var_vl__blk277_dn1, var_vl__blk277_dn2, var_vl__blk277_dn3, var_vl__blk277_dn4, var_vl__blk277_dn5, var_vl__blk277_db0, var_vl__blk277_db1,)
    }
};
        var_vl__blk277 = assign4190_e4196;
        var_vl__blk277_dn0 = assign4190_e4196_d_n0;
        var_vl__blk277_dn1 = assign4190_e4196_d_n1;
        var_vl__blk277_dn2 = assign4190_e4196_d_n2;
        var_vl__blk277_dn3 = assign4190_e4196_d_n3;
        var_vl__blk277_dn4 = assign4190_e4196_d_n4;
        var_vl__blk277_dn5 = assign4190_e4196_d_n5;
        var_vl__blk277_db0 = assign4190_e4196_d_b0;
        var_vl__blk277_db1 = assign4190_e4196_d_b1;
        var_vl__blk277_rv = 0.0;
        var_vl__blk277_rdn0 = 0.0;
        var_vl__blk277_rdn1 = 0.0;
        var_vl__blk277_rdn2 = 0.0;
        var_vl__blk277_rdn3 = 0.0;
        var_vl__blk277_rdn4 = 0.0;
        var_vl__blk277_rdn5 = 0.0;
        var_vl__blk277_rdb0 = 0.0;
        var_vl__blk277_rdb1 = 0.0;

        let (assign4200_e4220, assign4200_e4220_d_n0, assign4200_e4220_d_n1, assign4200_e4220_d_n2, assign4200_e4220_d_n3, assign4200_e4220_d_n4, assign4200_e4220_d_n5, assign4200_e4220_d_b0, assign4200_e4220_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4200_e4204: f64 = (-var_pp_t);
        let assign4200_e4208: f64 = (var_vl__blk277 / var_pp_t);
        let assign4200_e4209: f64 = (1.0 - assign4200_e4208);
        let assign4200_e4212: f64 = (1.0 - p.p81);
        let assign4200_e4213: f64 = (assign4200_e4209).powf(assign4200_e4212);
        let assign4200_e4214: f64 = (assign4200_e4204 * assign4200_e4213);
        let assign4200_e4217: f64 = (1.0 - p.p81);
        let assign4200_e4218: f64 = (assign4200_e4214 / assign4200_e4217);
        (assign4200_e4218, ((((-var_pp_t_dn0) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn0 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn0)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn0 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn0)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_dn1) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn1 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn1)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn1 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn1)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_dn2) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn2 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn2)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn2 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn2)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_dn3) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn3 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn3)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn3 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn3)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_dn4) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn4 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn4)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn4 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn4)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_dn5) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_dn5 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn5)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_dn5 * var_pp_t) - (var_vl__blk277 * var_pp_t_dn5)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_db0) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_db0 * var_pp_t) - (var_vl__blk277 * var_pp_t_db0)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_db0 * var_pp_t) - (var_vl__blk277 * var_pp_t_db0)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217), ((((-var_pp_t_db1) * assign4200_e4213) + (assign4200_e4204 * if 0.0 == 0.0 && ((assign4200_e4212) as f64).is_finite() && ((assign4200_e4212) as f64).fract() == 0.0 { if assign4200_e4212 == 0.0 { 0.0 } else { (assign4200_e4212 * ((assign4200_e4209).powf(assign4200_e4212 - 1.0) * (-(((var_vl__blk277_db1 * var_pp_t) - (var_vl__blk277 * var_pp_t_db1)) / (var_pp_t * var_pp_t))))) } } else { (assign4200_e4213 * (assign4200_e4212 * ((-(((var_vl__blk277_db1 * var_pp_t) - (var_vl__blk277 * var_pp_t_db1)) / (var_pp_t * var_pp_t))) / assign4200_e4209))) })) / assign4200_e4217),)
    } else {
        (var_qlo__blk271, var_qlo__blk271_dn0, var_qlo__blk271_dn1, var_qlo__blk271_dn2, var_qlo__blk271_dn3, var_qlo__blk271_dn4, var_qlo__blk271_dn5, var_qlo__blk271_db0, var_qlo__blk271_db1,)
    }
};
        var_qlo__blk271 = assign4200_e4220;
        var_qlo__blk271_dn0 = assign4200_e4220_d_n0;
        var_qlo__blk271_dn1 = assign4200_e4220_d_n1;
        var_qlo__blk271_dn2 = assign4200_e4220_d_n2;
        var_qlo__blk271_dn3 = assign4200_e4220_d_n3;
        var_qlo__blk271_dn4 = assign4200_e4220_d_n4;
        var_qlo__blk271_dn5 = assign4200_e4220_d_n5;
        var_qlo__blk271_db0 = assign4200_e4220_d_b0;
        var_qlo__blk271_db1 = assign4200_e4220_d_b1;
        var_qlo__blk271_rv = 0.0;
        var_qlo__blk271_rdn0 = 0.0;
        var_qlo__blk271_rdn1 = 0.0;
        var_qlo__blk271_rdn2 = 0.0;
        var_qlo__blk271_rdn3 = 0.0;
        var_qlo__blk271_rdn4 = 0.0;
        var_qlo__blk271_rdn5 = 0.0;
        var_qlo__blk271_rdb0 = 0.0;
        var_qlo__blk271_rdb1 = 0.0;

        let (assign4210_e4260, assign4210_e4260_d_n0, assign4210_e4260_d_n1, assign4210_e4260_d_n2, assign4210_e4260_d_n3, assign4210_e4260_d_n4, assign4210_e4260_d_n5, assign4210_e4260_d_b0, assign4210_e4260_d_b1,) = {
    if (((var_guard249 != 0.0) && (var_guard267 != 0.0)) && (var_guard278 == 0.0)) {
        let assign4210_e4230: f64 = (1.0 - p.p68);
        let assign4210_e4232: f64 = (-p.p81);
        let assign4210_e4233: f64 = (assign4210_e4230).powf(assign4210_e4232);
        let assign4210_e4236: f64 = (var_vcl - var_vl__blk277);
        let assign4210_e4238: f64 = (assign4210_e4236 + var_vl0__blk274);
        let assign4210_e4239: f64 = (assign4210_e4233 * assign4210_e4238);
        let assign4210_e4243: f64 = (0.5 * p.p81);
        let assign4210_e4246: f64 = (var_vcl - var_vl__blk277);
        let assign4210_e4248: f64 = (assign4210_e4246 + var_vl0__blk274);
        let assign4210_e4249: f64 = (assign4210_e4243 * assign4210_e4248);
        let assign4210_e4253: f64 = (1.0 - p.p68);
        let assign4210_e4254: f64 = (var_pp_t * assign4210_e4253);
        let assign4210_e4255: f64 = (assign4210_e4249 / assign4210_e4254);
        let assign4210_e4256: f64 = (1.0 + assign4210_e4255);
        let assign4210_e4257: f64 = (assign4210_e4239 * assign4210_e4256);
        let assign4210_e4258: f64 = (var_qlo__blk271 + assign4210_e4257);
        (assign4210_e4258, (var_qlo__blk271_dn0 + (((assign4210_e4233 * ((var_vcl_dn0 - var_vl__blk277_dn0) + var_vl0__blk274_dn0)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn0 - var_vl__blk277_dn0) + var_vl0__blk274_dn0)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn0 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_dn1 + (((assign4210_e4233 * ((var_vcl_dn1 - var_vl__blk277_dn1) + var_vl0__blk274_dn1)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn1 - var_vl__blk277_dn1) + var_vl0__blk274_dn1)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn1 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_dn2 + (((assign4210_e4233 * ((var_vcl_dn2 - var_vl__blk277_dn2) + var_vl0__blk274_dn2)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn2 - var_vl__blk277_dn2) + var_vl0__blk274_dn2)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn2 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_dn3 + (((assign4210_e4233 * ((var_vcl_dn3 - var_vl__blk277_dn3) + var_vl0__blk274_dn3)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn3 - var_vl__blk277_dn3) + var_vl0__blk274_dn3)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn3 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_dn4 + (((assign4210_e4233 * ((var_vcl_dn4 - var_vl__blk277_dn4) + var_vl0__blk274_dn4)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn4 - var_vl__blk277_dn4) + var_vl0__blk274_dn4)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn4 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_dn5 + (((assign4210_e4233 * ((var_vcl_dn5 - var_vl__blk277_dn5) + var_vl0__blk274_dn5)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_dn5 - var_vl__blk277_dn5) + var_vl0__blk274_dn5)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_dn5 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_db0 + (((assign4210_e4233 * ((var_vcl_db0 - var_vl__blk277_db0) + var_vl0__blk274_db0)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_db0 - var_vl__blk277_db0) + var_vl0__blk274_db0)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_db0 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))), (var_qlo__blk271_db1 + (((assign4210_e4233 * ((var_vcl_db1 - var_vl__blk277_db1) + var_vl0__blk274_db1)) * assign4210_e4256) + (assign4210_e4239 * ((((assign4210_e4243 * ((var_vcl_db1 - var_vl__blk277_db1) + var_vl0__blk274_db1)) * assign4210_e4254) - (assign4210_e4249 * (var_pp_t_db1 * assign4210_e4253))) / (assign4210_e4254 * assign4210_e4254))))),)
    } else {
        (var_argp, var_argp_dn0, var_argp_dn1, var_argp_dn2, var_argp_dn3, var_argp_dn4, var_argp_dn5, var_argp_db0, var_argp_db1,)
    }
};
        var_argp = assign4210_e4260;
        var_argp_dn0 = assign4210_e4260_d_n0;
        var_argp_dn1 = assign4210_e4260_d_n1;
        var_argp_dn2 = assign4210_e4260_d_n2;
        var_argp_dn3 = assign4210_e4260_d_n3;
        var_argp_dn4 = assign4210_e4260_d_n4;
        var_argp_dn5 = assign4210_e4260_d_n5;
        var_argp_db0 = assign4210_e4260_d_b0;
        var_argp_db1 = assign4210_e4260_d_b1;
        var_argp_rv = 0.0;
        var_argp_rdn0 = 0.0;
        var_argp_rdn1 = 0.0;
        var_argp_rdn2 = 0.0;
        var_argp_rdn3 = 0.0;
        var_argp_rdn4 = 0.0;
        var_argp_rdn5 = 0.0;
        var_argp_rdb0 = 0.0;
        var_argp_rdb1 = 0.0;

        let (assign4220_e4267, assign4220_e4267_d_n0, assign4220_e4267_d_n1, assign4220_e4267_d_n2, assign4220_e4267_d_n3, assign4220_e4267_d_n4, assign4220_e4267_d_n5, assign4220_e4267_d_b0, assign4220_e4267_d_b1,) = {
    if ((var_guard249 != 0.0) && (var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_argp, var_argp_dn0, var_argp_dn1, var_argp_dn2, var_argp_dn3, var_argp_dn4, var_argp_dn5, var_argp_db0, var_argp_db1,)
    }
};
        var_argp = assign4220_e4267;
        var_argp_dn0 = assign4220_e4267_d_n0;
        var_argp_dn1 = assign4220_e4267_d_n1;
        var_argp_dn2 = assign4220_e4267_d_n2;
        var_argp_dn3 = assign4220_e4267_d_n3;
        var_argp_dn4 = assign4220_e4267_d_n4;
        var_argp_dn5 = assign4220_e4267_d_n5;
        var_argp_db0 = assign4220_e4267_d_b0;
        var_argp_db1 = assign4220_e4267_d_b1;
        var_argp_rv = 0.0;
        var_argp_rdn0 = 0.0;
        var_argp_rdn1 = 0.0;
        var_argp_rdn2 = 0.0;
        var_argp_rdn3 = 0.0;
        var_argp_rdn4 = 0.0;
        var_argp_rdn5 = 0.0;
        var_argp_rdb0 = 0.0;
        var_argp_rdb1 = 0.0;

        let (assign4230_e4277, assign4230_e4277_d_n0, assign4230_e4277_d_n1, assign4230_e4277_d_n2, assign4230_e4277_d_n3, assign4230_e4277_d_n4, assign4230_e4277_d_n5, assign4230_e4277_d_b0, assign4230_e4277_d_b1,) = {
    if (var_guard249 != 0.0) {
        let assign4230_e4271: f64 = (var_acja * var_arga);
        let assign4230_e4274: f64 = (var_pcjp * var_argp);
        let assign4230_e4275: f64 = (assign4230_e4271 + assign4230_e4274);
        (assign4230_e4275, (((var_acja_dn0 * var_arga) + (var_acja * var_arga_dn0)) + ((var_pcjp_dn0 * var_argp) + (var_pcjp * var_argp_dn0))), (((var_acja_dn1 * var_arga) + (var_acja * var_arga_dn1)) + ((var_pcjp_dn1 * var_argp) + (var_pcjp * var_argp_dn1))), (((var_acja_dn2 * var_arga) + (var_acja * var_arga_dn2)) + ((var_pcjp_dn2 * var_argp) + (var_pcjp * var_argp_dn2))), (((var_acja_dn3 * var_arga) + (var_acja * var_arga_dn3)) + ((var_pcjp_dn3 * var_argp) + (var_pcjp * var_argp_dn3))), (((var_acja_dn4 * var_arga) + (var_acja * var_arga_dn4)) + ((var_pcjp_dn4 * var_argp) + (var_pcjp * var_argp_dn4))), (((var_acja_dn5 * var_arga) + (var_acja * var_arga_dn5)) + ((var_pcjp_dn5 * var_argp) + (var_pcjp * var_argp_dn5))), (((var_acja_db0 * var_arga) + (var_acja * var_arga_db0)) + ((var_pcjp_db0 * var_argp) + (var_pcjp * var_argp_db0))), (((var_acja_db1 * var_arga) + (var_acja * var_arga_db1)) + ((var_pcjp_db1 * var_argp) + (var_pcjp * var_argp_db1))),)
    } else {
        (var_qcp1, var_qcp1_dn0, var_qcp1_dn1, var_qcp1_dn2, var_qcp1_dn3, var_qcp1_dn4, var_qcp1_dn5, var_qcp1_db0, var_qcp1_db1,)
    }
};
        var_qcp1 = assign4230_e4277;
        var_qcp1_dn0 = assign4230_e4277_d_n0;
        var_qcp1_dn1 = assign4230_e4277_d_n1;
        var_qcp1_dn2 = assign4230_e4277_d_n2;
        var_qcp1_dn3 = assign4230_e4277_d_n3;
        var_qcp1_dn4 = assign4230_e4277_d_n4;
        var_qcp1_dn5 = assign4230_e4277_d_n5;
        var_qcp1_db0 = assign4230_e4277_d_b0;
        var_qcp1_db1 = assign4230_e4277_d_b1;
        var_qcp1_rv = 0.0;
        var_qcp1_rdn0 = 0.0;
        var_qcp1_rdn1 = 0.0;
        var_qcp1_rdn2 = 0.0;
        var_qcp1_rdn3 = 0.0;
        var_qcp1_rdn4 = 0.0;
        var_qcp1_rdn5 = 0.0;
        var_qcp1_rdb0 = 0.0;
        var_qcp1_rdb1 = 0.0;

        let (assign4240_e4282, assign4240_e4282_d_n0, assign4240_e4282_d_n1, assign4240_e4282_d_n2, assign4240_e4282_d_n3, assign4240_e4282_d_n4, assign4240_e4282_d_n5, assign4240_e4282_d_b0, assign4240_e4282_d_b1,) = {
    if (var_guard249 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qcp1, var_qcp1_dn0, var_qcp1_dn1, var_qcp1_dn2, var_qcp1_dn3, var_qcp1_dn4, var_qcp1_dn5, var_qcp1_db0, var_qcp1_db1,)
    }
};
        var_qcp1 = assign4240_e4282;
        var_qcp1_dn0 = assign4240_e4282_d_n0;
        var_qcp1_dn1 = assign4240_e4282_d_n1;
        var_qcp1_dn2 = assign4240_e4282_d_n2;
        var_qcp1_dn3 = assign4240_e4282_d_n3;
        var_qcp1_dn4 = assign4240_e4282_d_n4;
        var_qcp1_dn5 = assign4240_e4282_d_n5;
        var_qcp1_db0 = assign4240_e4282_d_b0;
        var_qcp1_db1 = assign4240_e4282_d_b1;
        var_qcp1_rv = 0.0;
        var_qcp1_rdn0 = 0.0;
        var_qcp1_rdn1 = 0.0;
        var_qcp1_rdn2 = 0.0;
        var_qcp1_rdn3 = 0.0;
        var_qcp1_rdn4 = 0.0;
        var_qcp1_rdn5 = 0.0;
        var_qcp1_rdb0 = 0.0;
        var_qcp1_rdb1 = 0.0;

        let assign4250_e4285: f64 = if var_cj2 > 0.0 { 1.0 } else { 0.0 };
        var_guard280 = assign4250_e4285;
        var_guard280_dn0 = 0.0;
        var_guard280_dn1 = 0.0;
        var_guard280_dn2 = 0.0;
        var_guard280_dn3 = 0.0;
        var_guard280_dn4 = 0.0;
        var_guard280_dn5 = 0.0;
        var_guard280_db0 = 0.0;
        var_guard280_db1 = 0.0;
        var_guard280_rv = 0.0;
        var_guard280_rdn0 = 0.0;
        var_guard280_rdn1 = 0.0;
        var_guard280_rdn2 = 0.0;
        var_guard280_rdn3 = 0.0;
        var_guard280_rdn4 = 0.0;
        var_guard280_rdn5 = 0.0;
        var_guard280_rdb0 = 0.0;
        var_guard280_rdb1 = 0.0;

        let (assign4260_e4306, assign4260_e4306_d_n0, assign4260_e4306_d_n1, assign4260_e4306_d_n2, assign4260_e4306_d_n3, assign4260_e4306_d_n4, assign4260_e4306_d_n5, assign4260_e4306_d_b0, assign4260_e4306_d_b1,) = {
    if ((var_guard280 != 0.0) && (p.p63 != 0.0)) {
        let assign4260_e4292: f64 = (var_vc2 - var_vpo);
        let assign4260_e4295: f64 = (var_vc2 + var_vpo);
        let assign4260_e4298: f64 = (var_vc2 + var_vpo);
        let assign4260_e4299: f64 = (assign4260_e4295 * assign4260_e4298);
        let assign4260_e4301: f64 = (assign4260_e4299 + 0.04);
        let assign4260_e4302: f64 = (assign4260_e4301).sqrt();
        let assign4260_e4303: f64 = (assign4260_e4292 + assign4260_e4302);
        let assign4260_e4304: f64 = (0.5 * assign4260_e4303);
        (assign4260_e4304, (0.5 * ((var_vc2_dn0 - var_vpo_dn0) + ((((var_vc2_dn0 + var_vpo_dn0) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn0 + var_vpo_dn0))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_dn1 - var_vpo_dn1) + ((((var_vc2_dn1 + var_vpo_dn1) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn1 + var_vpo_dn1))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_dn2 - var_vpo_dn2) + ((((var_vc2_dn2 + var_vpo_dn2) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn2 + var_vpo_dn2))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_dn3 - var_vpo_dn3) + ((((var_vc2_dn3 + var_vpo_dn3) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn3 + var_vpo_dn3))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_dn4 - var_vpo_dn4) + ((((var_vc2_dn4 + var_vpo_dn4) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn4 + var_vpo_dn4))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_dn5 - var_vpo_dn5) + ((((var_vc2_dn5 + var_vpo_dn5) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_dn5 + var_vpo_dn5))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_db0 - var_vpo_db0) + ((((var_vc2_db0 + var_vpo_db0) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_db0 + var_vpo_db0))) / (2.0 * assign4260_e4302)))), (0.5 * ((var_vc2_db1 - var_vpo_db1) + ((((var_vc2_db1 + var_vpo_db1) * assign4260_e4298) + (assign4260_e4295 * (var_vc2_db1 + var_vpo_db1))) / (2.0 * assign4260_e4302)))),)
    } else {
        (var_vcl, var_vcl_dn0, var_vcl_dn1, var_vcl_dn2, var_vcl_dn3, var_vcl_dn4, var_vcl_dn5, var_vcl_db0, var_vcl_db1,)
    }
};
        var_vcl = assign4260_e4306;
        var_vcl_dn0 = assign4260_e4306_d_n0;
        var_vcl_dn1 = assign4260_e4306_d_n1;
        var_vcl_dn2 = assign4260_e4306_d_n2;
        var_vcl_dn3 = assign4260_e4306_d_n3;
        var_vcl_dn4 = assign4260_e4306_d_n4;
        var_vcl_dn5 = assign4260_e4306_d_n5;
        var_vcl_db0 = assign4260_e4306_d_b0;
        var_vcl_db1 = assign4260_e4306_d_b1;
        var_vcl_rv = 0.0;
        var_vcl_rdn0 = 0.0;
        var_vcl_rdn1 = 0.0;
        var_vcl_rdn2 = 0.0;
        var_vcl_rdn3 = 0.0;
        var_vcl_rdn4 = 0.0;
        var_vcl_rdn5 = 0.0;
        var_vcl_rdb0 = 0.0;
        var_vcl_rdb1 = 0.0;

        let (assign4270_e4313, assign4270_e4313_d_n0, assign4270_e4313_d_n1, assign4270_e4313_d_n2, assign4270_e4313_d_n3, assign4270_e4313_d_n4, assign4270_e4313_d_n5, assign4270_e4313_d_b0, assign4270_e4313_d_b1,) = {
    if ((var_guard280 != 0.0) && (p.p63 == 0.0)) {
        (var_vc2, var_vc2_dn0, var_vc2_dn1, var_vc2_dn2, var_vc2_dn3, var_vc2_dn4, var_vc2_dn5, var_vc2_db0, var_vc2_db1,)
    } else {
        (var_vcl, var_vcl_dn0, var_vcl_dn1, var_vcl_dn2, var_vcl_dn3, var_vcl_dn4, var_vcl_dn5, var_vcl_db0, var_vcl_db1,)
    }
};
        var_vcl = assign4270_e4313;
        var_vcl_dn0 = assign4270_e4313_d_n0;
        var_vcl_dn1 = assign4270_e4313_d_n1;
        var_vcl_dn2 = assign4270_e4313_d_n2;
        var_vcl_dn3 = assign4270_e4313_d_n3;
        var_vcl_dn4 = assign4270_e4313_d_n4;
        var_vcl_dn5 = assign4270_e4313_d_n5;
        var_vcl_db0 = assign4270_e4313_d_b0;
        var_vcl_db1 = assign4270_e4313_d_b1;
        var_vcl_rv = 0.0;
        var_vcl_rdn0 = 0.0;
        var_vcl_rdn1 = 0.0;
        var_vcl_rdn2 = 0.0;
        var_vcl_rdn3 = 0.0;
        var_vcl_rdn4 = 0.0;
        var_vcl_rdn5 = 0.0;
        var_vcl_rdb0 = 0.0;
        var_vcl_rdb1 = 0.0;

        let (assign4280_e4319, assign4280_e4319_d_n0, assign4280_e4319_d_n1, assign4280_e4319_d_n2, assign4280_e4319_d_n3, assign4280_e4319_d_n4, assign4280_e4319_d_n5, assign4280_e4319_d_b0, assign4280_e4319_d_b1,) = {
    if (var_guard280 != 0.0) {
        let assign4280_e4317: f64 = (var_a2_um2 * var_cja_t);
        (assign4280_e4317, ((var_a2_um2_dn0 * var_cja_t) + (var_a2_um2 * var_cja_t_dn0)), ((var_a2_um2_dn1 * var_cja_t) + (var_a2_um2 * var_cja_t_dn1)), ((var_a2_um2_dn2 * var_cja_t) + (var_a2_um2 * var_cja_t_dn2)), ((var_a2_um2_dn3 * var_cja_t) + (var_a2_um2 * var_cja_t_dn3)), ((var_a2_um2_dn4 * var_cja_t) + (var_a2_um2 * var_cja_t_dn4)), ((var_a2_um2_dn5 * var_cja_t) + (var_a2_um2 * var_cja_t_dn5)), ((var_a2_um2_db0 * var_cja_t) + (var_a2_um2 * var_cja_t_db0)), ((var_a2_um2_db1 * var_cja_t) + (var_a2_um2 * var_cja_t_db1)),)
    } else {
        (var_acja__blk281, var_acja__blk281_dn0, var_acja__blk281_dn1, var_acja__blk281_dn2, var_acja__blk281_dn3, var_acja__blk281_dn4, var_acja__blk281_dn5, var_acja__blk281_db0, var_acja__blk281_db1,)
    }
};
        var_acja__blk281 = assign4280_e4319;
        var_acja__blk281_dn0 = assign4280_e4319_d_n0;
        var_acja__blk281_dn1 = assign4280_e4319_d_n1;
        var_acja__blk281_dn2 = assign4280_e4319_d_n2;
        var_acja__blk281_dn3 = assign4280_e4319_d_n3;
        var_acja__blk281_dn4 = assign4280_e4319_d_n4;
        var_acja__blk281_dn5 = assign4280_e4319_d_n5;
        var_acja__blk281_db0 = assign4280_e4319_d_b0;
        var_acja__blk281_db1 = assign4280_e4319_d_b1;
        var_acja__blk281_rv = 0.0;
        var_acja__blk281_rdn0 = 0.0;
        var_acja__blk281_rdn1 = 0.0;
        var_acja__blk281_rdn2 = 0.0;
        var_acja__blk281_rdn3 = 0.0;
        var_acja__blk281_rdn4 = 0.0;
        var_acja__blk281_rdn5 = 0.0;
        var_acja__blk281_rdb0 = 0.0;
        var_acja__blk281_rdb1 = 0.0;

        let (assign4290_e4325, assign4290_e4325_d_n0, assign4290_e4325_d_n1, assign4290_e4325_d_n2, assign4290_e4325_d_n3, assign4290_e4325_d_n4, assign4290_e4325_d_n5, assign4290_e4325_d_b0, assign4290_e4325_d_b1,) = {
    if (var_guard280 != 0.0) {
        let assign4290_e4323: f64 = (var_p2_um * var_cjp_t);
        (assign4290_e4323, ((var_p2_um_dn0 * var_cjp_t) + (var_p2_um * var_cjp_t_dn0)), ((var_p2_um_dn1 * var_cjp_t) + (var_p2_um * var_cjp_t_dn1)), ((var_p2_um_dn2 * var_cjp_t) + (var_p2_um * var_cjp_t_dn2)), ((var_p2_um_dn3 * var_cjp_t) + (var_p2_um * var_cjp_t_dn3)), ((var_p2_um_dn4 * var_cjp_t) + (var_p2_um * var_cjp_t_dn4)), ((var_p2_um_dn5 * var_cjp_t) + (var_p2_um * var_cjp_t_dn5)), ((var_p2_um_db0 * var_cjp_t) + (var_p2_um * var_cjp_t_db0)), ((var_p2_um_db1 * var_cjp_t) + (var_p2_um * var_cjp_t_db1)),)
    } else {
        (var_pcjp__blk282, var_pcjp__blk282_dn0, var_pcjp__blk282_dn1, var_pcjp__blk282_dn2, var_pcjp__blk282_dn3, var_pcjp__blk282_dn4, var_pcjp__blk282_dn5, var_pcjp__blk282_db0, var_pcjp__blk282_db1,)
    }
};
        var_pcjp__blk282 = assign4290_e4325;
        var_pcjp__blk282_dn0 = assign4290_e4325_d_n0;
        var_pcjp__blk282_dn1 = assign4290_e4325_d_n1;
        var_pcjp__blk282_dn2 = assign4290_e4325_d_n2;
        var_pcjp__blk282_dn3 = assign4290_e4325_d_n3;
        var_pcjp__blk282_dn4 = assign4290_e4325_d_n4;
        var_pcjp__blk282_dn5 = assign4290_e4325_d_n5;
        var_pcjp__blk282_db0 = assign4290_e4325_d_b0;
        var_pcjp__blk282_db1 = assign4290_e4325_d_b1;
        var_pcjp__blk282_rv = 0.0;
        var_pcjp__blk282_rdn0 = 0.0;
        var_pcjp__blk282_rdn1 = 0.0;
        var_pcjp__blk282_rdn2 = 0.0;
        var_pcjp__blk282_rdn3 = 0.0;
        var_pcjp__blk282_rdn4 = 0.0;
        var_pcjp__blk282_rdn5 = 0.0;
        var_pcjp__blk282_rdb0 = 0.0;
        var_pcjp__blk282_rdb1 = 0.0;

        let assign4300_e4328: f64 = if var_acja__blk281 > 0.0 { 1.0 } else { 0.0 };
        var_guard285 = assign4300_e4328;
        var_guard285_dn0 = 0.0;
        var_guard285_dn1 = 0.0;
        var_guard285_dn2 = 0.0;
        var_guard285_dn3 = 0.0;
        var_guard285_dn4 = 0.0;
        var_guard285_dn5 = 0.0;
        var_guard285_db0 = 0.0;
        var_guard285_db1 = 0.0;
        var_guard285_rv = 0.0;
        var_guard285_rdn0 = 0.0;
        var_guard285_rdn1 = 0.0;
        var_guard285_rdn2 = 0.0;
        var_guard285_rdn3 = 0.0;
        var_guard285_rdn4 = 0.0;
        var_guard285_rdn5 = 0.0;
        var_guard285_rdb0 = 0.0;
        var_guard285_rdb1 = 0.0;

        let (assign4310_e4337, assign4310_e4337_d_n0, assign4310_e4337_d_n1, assign4310_e4337_d_n2, assign4310_e4337_d_n3, assign4310_e4337_d_n4, assign4310_e4337_d_n5, assign4310_e4337_d_b0, assign4310_e4337_d_b1,) = {
    if ((var_guard280 != 0.0) && (var_guard285 != 0.0)) {
        let assign4310_e4333: f64 = (-var_pa_t);
        let assign4310_e4335: f64 = (assign4310_e4333 * p.p68);
        (assign4310_e4335, ((-var_pa_t_dn0) * p.p68), ((-var_pa_t_dn1) * p.p68), ((-var_pa_t_dn2) * p.p68), ((-var_pa_t_dn3) * p.p68), ((-var_pa_t_dn4) * p.p68), ((-var_pa_t_dn5) * p.p68), ((-var_pa_t_db0) * p.p68), ((-var_pa_t_db1) * p.p68),)
    } else {
        (var_dv0__blk286, var_dv0__blk286_dn0, var_dv0__blk286_dn1, var_dv0__blk286_dn2, var_dv0__blk286_dn3, var_dv0__blk286_dn4, var_dv0__blk286_dn5, var_dv0__blk286_db0, var_dv0__blk286_db1,)
    }
};
        var_dv0__blk286 = assign4310_e4337;
        var_dv0__blk286_dn0 = assign4310_e4337_d_n0;
        var_dv0__blk286_dn1 = assign4310_e4337_d_n1;
        var_dv0__blk286_dn2 = assign4310_e4337_d_n2;
        var_dv0__blk286_dn3 = assign4310_e4337_d_n3;
        var_dv0__blk286_dn4 = assign4310_e4337_d_n4;
        var_dv0__blk286_dn5 = assign4310_e4337_d_n5;
        var_dv0__blk286_db0 = assign4310_e4337_d_b0;
        var_dv0__blk286_db1 = assign4310_e4337_d_b1;
        var_dv0__blk286_rv = 0.0;
        var_dv0__blk286_rdn0 = 0.0;
        var_dv0__blk286_rdn1 = 0.0;
        var_dv0__blk286_rdn2 = 0.0;
        var_dv0__blk286_rdn3 = 0.0;
        var_dv0__blk286_rdn4 = 0.0;
        var_dv0__blk286_rdn5 = 0.0;
        var_dv0__blk286_rdb0 = 0.0;
        var_dv0__blk286_rdb1 = 0.0;

        let assign4320_e4340: f64 = if p.p75 <= 0.0 { 1.0 } else { 0.0 };
        var_guard296 = assign4320_e4340;
        var_guard296_dn0 = 0.0;
        var_guard296_dn1 = 0.0;
        var_guard296_dn2 = 0.0;
        var_guard296_dn3 = 0.0;
        var_guard296_dn4 = 0.0;
        var_guard296_dn5 = 0.0;
        var_guard296_db0 = 0.0;
        var_guard296_db1 = 0.0;
        var_guard296_rv = 0.0;
        var_guard296_rdn0 = 0.0;
        var_guard296_rdn1 = 0.0;
        var_guard296_rdn2 = 0.0;
        var_guard296_rdn3 = 0.0;
        var_guard296_rdn4 = 0.0;
        var_guard296_rdn5 = 0.0;
        var_guard296_rdb0 = 0.0;
        var_guard296_rdb1 = 0.0;

        let (assign4330_e4350, assign4330_e4350_d_n0, assign4330_e4350_d_n1, assign4330_e4350_d_n2, assign4330_e4350_d_n3, assign4330_e4350_d_n4, assign4330_e4350_d_n5, assign4330_e4350_d_b0, assign4330_e4350_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) {
        let assign4330_e4348: f64 = (var_vcl + var_dv0__blk286);
        (assign4330_e4348, (var_vcl_dn0 + var_dv0__blk286_dn0), (var_vcl_dn1 + var_dv0__blk286_dn1), (var_vcl_dn2 + var_dv0__blk286_dn2), (var_vcl_dn3 + var_dv0__blk286_dn3), (var_vcl_dn4 + var_dv0__blk286_dn4), (var_vcl_dn5 + var_dv0__blk286_dn5), (var_vcl_db0 + var_dv0__blk286_db0), (var_vcl_db1 + var_dv0__blk286_db1),)
    } else {
        (var_dvh__blk287, var_dvh__blk287_dn0, var_dvh__blk287_dn1, var_dvh__blk287_dn2, var_dvh__blk287_dn3, var_dvh__blk287_dn4, var_dvh__blk287_dn5, var_dvh__blk287_db0, var_dvh__blk287_db1,)
    }
};
        var_dvh__blk287 = assign4330_e4350;
        var_dvh__blk287_dn0 = assign4330_e4350_d_n0;
        var_dvh__blk287_dn1 = assign4330_e4350_d_n1;
        var_dvh__blk287_dn2 = assign4330_e4350_d_n2;
        var_dvh__blk287_dn3 = assign4330_e4350_d_n3;
        var_dvh__blk287_dn4 = assign4330_e4350_d_n4;
        var_dvh__blk287_dn5 = assign4330_e4350_d_n5;
        var_dvh__blk287_db0 = assign4330_e4350_d_b0;
        var_dvh__blk287_db1 = assign4330_e4350_d_b1;
        var_dvh__blk287_rv = 0.0;
        var_dvh__blk287_rdn0 = 0.0;
        var_dvh__blk287_rdn1 = 0.0;
        var_dvh__blk287_rdn2 = 0.0;
        var_dvh__blk287_rdn3 = 0.0;
        var_dvh__blk287_rdn4 = 0.0;
        var_dvh__blk287_rdn5 = 0.0;
        var_dvh__blk287_rdb0 = 0.0;
        var_dvh__blk287_rdb1 = 0.0;

        *var_acja__blk281_slot = var_acja__blk281;
        *var_acja__blk281_db0_slot = var_acja__blk281_db0;
        *var_acja__blk281_db1_slot = var_acja__blk281_db1;
        *var_acja__blk281_dn0_slot = var_acja__blk281_dn0;
        *var_acja__blk281_dn1_slot = var_acja__blk281_dn1;
        *var_acja__blk281_dn2_slot = var_acja__blk281_dn2;
        *var_acja__blk281_dn3_slot = var_acja__blk281_dn3;
        *var_acja__blk281_dn4_slot = var_acja__blk281_dn4;
        *var_acja__blk281_dn5_slot = var_acja__blk281_dn5;
        *var_acja__blk281_rdb0_slot = var_acja__blk281_rdb0;
        *var_acja__blk281_rdb1_slot = var_acja__blk281_rdb1;
        *var_acja__blk281_rdn0_slot = var_acja__blk281_rdn0;
        *var_acja__blk281_rdn1_slot = var_acja__blk281_rdn1;
        *var_acja__blk281_rdn2_slot = var_acja__blk281_rdn2;
        *var_acja__blk281_rdn3_slot = var_acja__blk281_rdn3;
        *var_acja__blk281_rdn4_slot = var_acja__blk281_rdn4;
        *var_acja__blk281_rdn5_slot = var_acja__blk281_rdn5;
        *var_acja__blk281_rv_slot = var_acja__blk281_rv;
        *var_argp_slot = var_argp;
        *var_argp_db0_slot = var_argp_db0;
        *var_argp_db1_slot = var_argp_db1;
        *var_argp_dn0_slot = var_argp_dn0;
        *var_argp_dn1_slot = var_argp_dn1;
        *var_argp_dn2_slot = var_argp_dn2;
        *var_argp_dn3_slot = var_argp_dn3;
        *var_argp_dn4_slot = var_argp_dn4;
        *var_argp_dn5_slot = var_argp_dn5;
        *var_argp_rdb0_slot = var_argp_rdb0;
        *var_argp_rdb1_slot = var_argp_rdb1;
        *var_argp_rdn0_slot = var_argp_rdn0;
        *var_argp_rdn1_slot = var_argp_rdn1;
        *var_argp_rdn2_slot = var_argp_rdn2;
        *var_argp_rdn3_slot = var_argp_rdn3;
        *var_argp_rdn4_slot = var_argp_rdn4;
        *var_argp_rdn5_slot = var_argp_rdn5;
        *var_argp_rv_slot = var_argp_rv;
        *var_dv0__blk286_slot = var_dv0__blk286;
        *var_dv0__blk286_db0_slot = var_dv0__blk286_db0;
        *var_dv0__blk286_db1_slot = var_dv0__blk286_db1;
        *var_dv0__blk286_dn0_slot = var_dv0__blk286_dn0;
        *var_dv0__blk286_dn1_slot = var_dv0__blk286_dn1;
        *var_dv0__blk286_dn2_slot = var_dv0__blk286_dn2;
        *var_dv0__blk286_dn3_slot = var_dv0__blk286_dn3;
        *var_dv0__blk286_dn4_slot = var_dv0__blk286_dn4;
        *var_dv0__blk286_dn5_slot = var_dv0__blk286_dn5;
        *var_dv0__blk286_rdb0_slot = var_dv0__blk286_rdb0;
        *var_dv0__blk286_rdb1_slot = var_dv0__blk286_rdb1;
        *var_dv0__blk286_rdn0_slot = var_dv0__blk286_rdn0;
        *var_dv0__blk286_rdn1_slot = var_dv0__blk286_rdn1;
        *var_dv0__blk286_rdn2_slot = var_dv0__blk286_rdn2;
        *var_dv0__blk286_rdn3_slot = var_dv0__blk286_rdn3;
        *var_dv0__blk286_rdn4_slot = var_dv0__blk286_rdn4;
        *var_dv0__blk286_rdn5_slot = var_dv0__blk286_rdn5;
        *var_dv0__blk286_rv_slot = var_dv0__blk286_rv;
        *var_dv__blk275_slot = var_dv__blk275;
        *var_dv__blk275_db0_slot = var_dv__blk275_db0;
        *var_dv__blk275_db1_slot = var_dv__blk275_db1;
        *var_dv__blk275_dn0_slot = var_dv__blk275_dn0;
        *var_dv__blk275_dn1_slot = var_dv__blk275_dn1;
        *var_dv__blk275_dn2_slot = var_dv__blk275_dn2;
        *var_dv__blk275_dn3_slot = var_dv__blk275_dn3;
        *var_dv__blk275_dn4_slot = var_dv__blk275_dn4;
        *var_dv__blk275_dn5_slot = var_dv__blk275_dn5;
        *var_dv__blk275_rdb0_slot = var_dv__blk275_rdb0;
        *var_dv__blk275_rdb1_slot = var_dv__blk275_rdb1;
        *var_dv__blk275_rdn0_slot = var_dv__blk275_rdn0;
        *var_dv__blk275_rdn1_slot = var_dv__blk275_rdn1;
        *var_dv__blk275_rdn2_slot = var_dv__blk275_rdn2;
        *var_dv__blk275_rdn3_slot = var_dv__blk275_rdn3;
        *var_dv__blk275_rdn4_slot = var_dv__blk275_rdn4;
        *var_dv__blk275_rdn5_slot = var_dv__blk275_rdn5;
        *var_dv__blk275_rv_slot = var_dv__blk275_rv;
        *var_dvh__blk287_slot = var_dvh__blk287;
        *var_dvh__blk287_db0_slot = var_dvh__blk287_db0;
        *var_dvh__blk287_db1_slot = var_dvh__blk287_db1;
        *var_dvh__blk287_dn0_slot = var_dvh__blk287_dn0;
        *var_dvh__blk287_dn1_slot = var_dvh__blk287_dn1;
        *var_dvh__blk287_dn2_slot = var_dvh__blk287_dn2;
        *var_dvh__blk287_dn3_slot = var_dvh__blk287_dn3;
        *var_dvh__blk287_dn4_slot = var_dvh__blk287_dn4;
        *var_dvh__blk287_dn5_slot = var_dvh__blk287_dn5;
        *var_dvh__blk287_rdb0_slot = var_dvh__blk287_rdb0;
        *var_dvh__blk287_rdb1_slot = var_dvh__blk287_rdb1;
        *var_dvh__blk287_rdn0_slot = var_dvh__blk287_rdn0;
        *var_dvh__blk287_rdn1_slot = var_dvh__blk287_rdn1;
        *var_dvh__blk287_rdn2_slot = var_dvh__blk287_rdn2;
        *var_dvh__blk287_rdn3_slot = var_dvh__blk287_rdn3;
        *var_dvh__blk287_rdn4_slot = var_dvh__blk287_rdn4;
        *var_dvh__blk287_rdn5_slot = var_dvh__blk287_rdn5;
        *var_dvh__blk287_rv_slot = var_dvh__blk287_rv;
        *var_guard280_slot = var_guard280;
        *var_guard280_db0_slot = var_guard280_db0;
        *var_guard280_db1_slot = var_guard280_db1;
        *var_guard280_dn0_slot = var_guard280_dn0;
        *var_guard280_dn1_slot = var_guard280_dn1;
        *var_guard280_dn2_slot = var_guard280_dn2;
        *var_guard280_dn3_slot = var_guard280_dn3;
        *var_guard280_dn4_slot = var_guard280_dn4;
        *var_guard280_dn5_slot = var_guard280_dn5;
        *var_guard280_rdb0_slot = var_guard280_rdb0;
        *var_guard280_rdb1_slot = var_guard280_rdb1;
        *var_guard280_rdn0_slot = var_guard280_rdn0;
        *var_guard280_rdn1_slot = var_guard280_rdn1;
        *var_guard280_rdn2_slot = var_guard280_rdn2;
        *var_guard280_rdn3_slot = var_guard280_rdn3;
        *var_guard280_rdn4_slot = var_guard280_rdn4;
        *var_guard280_rdn5_slot = var_guard280_rdn5;
        *var_guard280_rv_slot = var_guard280_rv;
        *var_guard285_slot = var_guard285;
        *var_guard285_db0_slot = var_guard285_db0;
        *var_guard285_db1_slot = var_guard285_db1;
        *var_guard285_dn0_slot = var_guard285_dn0;
        *var_guard285_dn1_slot = var_guard285_dn1;
        *var_guard285_dn2_slot = var_guard285_dn2;
        *var_guard285_dn3_slot = var_guard285_dn3;
        *var_guard285_dn4_slot = var_guard285_dn4;
        *var_guard285_dn5_slot = var_guard285_dn5;
        *var_guard285_rdb0_slot = var_guard285_rdb0;
        *var_guard285_rdb1_slot = var_guard285_rdb1;
        *var_guard285_rdn0_slot = var_guard285_rdn0;
        *var_guard285_rdn1_slot = var_guard285_rdn1;
        *var_guard285_rdn2_slot = var_guard285_rdn2;
        *var_guard285_rdn3_slot = var_guard285_rdn3;
        *var_guard285_rdn4_slot = var_guard285_rdn4;
        *var_guard285_rdn5_slot = var_guard285_rdn5;
        *var_guard285_rv_slot = var_guard285_rv;
        *var_guard296_slot = var_guard296;
        *var_guard296_db0_slot = var_guard296_db0;
        *var_guard296_db1_slot = var_guard296_db1;
        *var_guard296_dn0_slot = var_guard296_dn0;
        *var_guard296_dn1_slot = var_guard296_dn1;
        *var_guard296_dn2_slot = var_guard296_dn2;
        *var_guard296_dn3_slot = var_guard296_dn3;
        *var_guard296_dn4_slot = var_guard296_dn4;
        *var_guard296_dn5_slot = var_guard296_dn5;
        *var_guard296_rdb0_slot = var_guard296_rdb0;
        *var_guard296_rdb1_slot = var_guard296_rdb1;
        *var_guard296_rdn0_slot = var_guard296_rdn0;
        *var_guard296_rdn1_slot = var_guard296_rdn1;
        *var_guard296_rdn2_slot = var_guard296_rdn2;
        *var_guard296_rdn3_slot = var_guard296_rdn3;
        *var_guard296_rdn4_slot = var_guard296_rdn4;
        *var_guard296_rdn5_slot = var_guard296_rdn5;
        *var_guard296_rv_slot = var_guard296_rv;
        *var_mv__blk276_slot = var_mv__blk276;
        *var_mv__blk276_db0_slot = var_mv__blk276_db0;
        *var_mv__blk276_db1_slot = var_mv__blk276_db1;
        *var_mv__blk276_dn0_slot = var_mv__blk276_dn0;
        *var_mv__blk276_dn1_slot = var_mv__blk276_dn1;
        *var_mv__blk276_dn2_slot = var_mv__blk276_dn2;
        *var_mv__blk276_dn3_slot = var_mv__blk276_dn3;
        *var_mv__blk276_dn4_slot = var_mv__blk276_dn4;
        *var_mv__blk276_dn5_slot = var_mv__blk276_dn5;
        *var_mv__blk276_rdb0_slot = var_mv__blk276_rdb0;
        *var_mv__blk276_rdb1_slot = var_mv__blk276_rdb1;
        *var_mv__blk276_rdn0_slot = var_mv__blk276_rdn0;
        *var_mv__blk276_rdn1_slot = var_mv__blk276_rdn1;
        *var_mv__blk276_rdn2_slot = var_mv__blk276_rdn2;
        *var_mv__blk276_rdn3_slot = var_mv__blk276_rdn3;
        *var_mv__blk276_rdn4_slot = var_mv__blk276_rdn4;
        *var_mv__blk276_rdn5_slot = var_mv__blk276_rdn5;
        *var_mv__blk276_rv_slot = var_mv__blk276_rv;
        *var_pcjp__blk282_slot = var_pcjp__blk282;
        *var_pcjp__blk282_db0_slot = var_pcjp__blk282_db0;
        *var_pcjp__blk282_db1_slot = var_pcjp__blk282_db1;
        *var_pcjp__blk282_dn0_slot = var_pcjp__blk282_dn0;
        *var_pcjp__blk282_dn1_slot = var_pcjp__blk282_dn1;
        *var_pcjp__blk282_dn2_slot = var_pcjp__blk282_dn2;
        *var_pcjp__blk282_dn3_slot = var_pcjp__blk282_dn3;
        *var_pcjp__blk282_dn4_slot = var_pcjp__blk282_dn4;
        *var_pcjp__blk282_dn5_slot = var_pcjp__blk282_dn5;
        *var_pcjp__blk282_rdb0_slot = var_pcjp__blk282_rdb0;
        *var_pcjp__blk282_rdb1_slot = var_pcjp__blk282_rdb1;
        *var_pcjp__blk282_rdn0_slot = var_pcjp__blk282_rdn0;
        *var_pcjp__blk282_rdn1_slot = var_pcjp__blk282_rdn1;
        *var_pcjp__blk282_rdn2_slot = var_pcjp__blk282_rdn2;
        *var_pcjp__blk282_rdn3_slot = var_pcjp__blk282_rdn3;
        *var_pcjp__blk282_rdn4_slot = var_pcjp__blk282_rdn4;
        *var_pcjp__blk282_rdn5_slot = var_pcjp__blk282_rdn5;
        *var_pcjp__blk282_rv_slot = var_pcjp__blk282_rv;
        *var_qcp1_slot = var_qcp1;
        *var_qcp1_db0_slot = var_qcp1_db0;
        *var_qcp1_db1_slot = var_qcp1_db1;
        *var_qcp1_dn0_slot = var_qcp1_dn0;
        *var_qcp1_dn1_slot = var_qcp1_dn1;
        *var_qcp1_dn2_slot = var_qcp1_dn2;
        *var_qcp1_dn3_slot = var_qcp1_dn3;
        *var_qcp1_dn4_slot = var_qcp1_dn4;
        *var_qcp1_dn5_slot = var_qcp1_dn5;
        *var_qcp1_rdb0_slot = var_qcp1_rdb0;
        *var_qcp1_rdb1_slot = var_qcp1_rdb1;
        *var_qcp1_rdn0_slot = var_qcp1_rdn0;
        *var_qcp1_rdn1_slot = var_qcp1_rdn1;
        *var_qcp1_rdn2_slot = var_qcp1_rdn2;
        *var_qcp1_rdn3_slot = var_qcp1_rdn3;
        *var_qcp1_rdn4_slot = var_qcp1_rdn4;
        *var_qcp1_rdn5_slot = var_qcp1_rdn5;
        *var_qcp1_rv_slot = var_qcp1_rv;
        *var_qlo__blk271_slot = var_qlo__blk271;
        *var_qlo__blk271_db0_slot = var_qlo__blk271_db0;
        *var_qlo__blk271_db1_slot = var_qlo__blk271_db1;
        *var_qlo__blk271_dn0_slot = var_qlo__blk271_dn0;
        *var_qlo__blk271_dn1_slot = var_qlo__blk271_dn1;
        *var_qlo__blk271_dn2_slot = var_qlo__blk271_dn2;
        *var_qlo__blk271_dn3_slot = var_qlo__blk271_dn3;
        *var_qlo__blk271_dn4_slot = var_qlo__blk271_dn4;
        *var_qlo__blk271_dn5_slot = var_qlo__blk271_dn5;
        *var_qlo__blk271_rdb0_slot = var_qlo__blk271_rdb0;
        *var_qlo__blk271_rdb1_slot = var_qlo__blk271_rdb1;
        *var_qlo__blk271_rdn0_slot = var_qlo__blk271_rdn0;
        *var_qlo__blk271_rdn1_slot = var_qlo__blk271_rdn1;
        *var_qlo__blk271_rdn2_slot = var_qlo__blk271_rdn2;
        *var_qlo__blk271_rdn3_slot = var_qlo__blk271_rdn3;
        *var_qlo__blk271_rdn4_slot = var_qlo__blk271_rdn4;
        *var_qlo__blk271_rdn5_slot = var_qlo__blk271_rdn5;
        *var_qlo__blk271_rv_slot = var_qlo__blk271_rv;
        *var_vcl_slot = var_vcl;
        *var_vcl_db0_slot = var_vcl_db0;
        *var_vcl_db1_slot = var_vcl_db1;
        *var_vcl_dn0_slot = var_vcl_dn0;
        *var_vcl_dn1_slot = var_vcl_dn1;
        *var_vcl_dn2_slot = var_vcl_dn2;
        *var_vcl_dn3_slot = var_vcl_dn3;
        *var_vcl_dn4_slot = var_vcl_dn4;
        *var_vcl_dn5_slot = var_vcl_dn5;
        *var_vcl_rdb0_slot = var_vcl_rdb0;
        *var_vcl_rdb1_slot = var_vcl_rdb1;
        *var_vcl_rdn0_slot = var_vcl_rdn0;
        *var_vcl_rdn1_slot = var_vcl_rdn1;
        *var_vcl_rdn2_slot = var_vcl_rdn2;
        *var_vcl_rdn3_slot = var_vcl_rdn3;
        *var_vcl_rdn4_slot = var_vcl_rdn4;
        *var_vcl_rdn5_slot = var_vcl_rdn5;
        *var_vcl_rv_slot = var_vcl_rv;
        *var_vl0__blk274_slot = var_vl0__blk274;
        *var_vl0__blk274_db0_slot = var_vl0__blk274_db0;
        *var_vl0__blk274_db1_slot = var_vl0__blk274_db1;
        *var_vl0__blk274_dn0_slot = var_vl0__blk274_dn0;
        *var_vl0__blk274_dn1_slot = var_vl0__blk274_dn1;
        *var_vl0__blk274_dn2_slot = var_vl0__blk274_dn2;
        *var_vl0__blk274_dn3_slot = var_vl0__blk274_dn3;
        *var_vl0__blk274_dn4_slot = var_vl0__blk274_dn4;
        *var_vl0__blk274_dn5_slot = var_vl0__blk274_dn5;
        *var_vl0__blk274_rdb0_slot = var_vl0__blk274_rdb0;
        *var_vl0__blk274_rdb1_slot = var_vl0__blk274_rdb1;
        *var_vl0__blk274_rdn0_slot = var_vl0__blk274_rdn0;
        *var_vl0__blk274_rdn1_slot = var_vl0__blk274_rdn1;
        *var_vl0__blk274_rdn2_slot = var_vl0__blk274_rdn2;
        *var_vl0__blk274_rdn3_slot = var_vl0__blk274_rdn3;
        *var_vl0__blk274_rdn4_slot = var_vl0__blk274_rdn4;
        *var_vl0__blk274_rdn5_slot = var_vl0__blk274_rdn5;
        *var_vl0__blk274_rv_slot = var_vl0__blk274_rv;
        *var_vl__blk277_slot = var_vl__blk277;
        *var_vl__blk277_db0_slot = var_vl__blk277_db0;
        *var_vl__blk277_db1_slot = var_vl__blk277_db1;
        *var_vl__blk277_dn0_slot = var_vl__blk277_dn0;
        *var_vl__blk277_dn1_slot = var_vl__blk277_dn1;
        *var_vl__blk277_dn2_slot = var_vl__blk277_dn2;
        *var_vl__blk277_dn3_slot = var_vl__blk277_dn3;
        *var_vl__blk277_dn4_slot = var_vl__blk277_dn4;
        *var_vl__blk277_dn5_slot = var_vl__blk277_dn5;
        *var_vl__blk277_rdb0_slot = var_vl__blk277_rdb0;
        *var_vl__blk277_rdb1_slot = var_vl__blk277_rdb1;
        *var_vl__blk277_rdn0_slot = var_vl__blk277_rdn0;
        *var_vl__blk277_rdn1_slot = var_vl__blk277_rdn1;
        *var_vl__blk277_rdn2_slot = var_vl__blk277_rdn2;
        *var_vl__blk277_rdn3_slot = var_vl__blk277_rdn3;
        *var_vl__blk277_rdn4_slot = var_vl__blk277_rdn4;
        *var_vl__blk277_rdn5_slot = var_vl__blk277_rdn5;
        *var_vl__blk277_rv_slot = var_vl__blk277_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        var_dv0__blk286: f64,
        var_dv0__blk286_db0: f64,
        var_dv0__blk286_db1: f64,
        var_dv0__blk286_dn0: f64,
        var_dv0__blk286_dn1: f64,
        var_dv0__blk286_dn2: f64,
        var_dv0__blk286_dn3: f64,
        var_dv0__blk286_dn4: f64,
        var_dv0__blk286_dn5: f64,
        var_dvh__blk287: f64,
        var_dvh__blk287_db0: f64,
        var_dvh__blk287_db1: f64,
        var_dvh__blk287_dn0: f64,
        var_dvh__blk287_dn1: f64,
        var_dvh__blk287_dn2: f64,
        var_dvh__blk287_dn3: f64,
        var_dvh__blk287_dn4: f64,
        var_dvh__blk287_dn5: f64,
        var_guard280: f64,
        var_guard285: f64,
        var_guard296: f64,
        var_pa_t: f64,
        var_pa_t_db0: f64,
        var_pa_t_db1: f64,
        var_pa_t_dn0: f64,
        var_pa_t_dn1: f64,
        var_pa_t_dn2: f64,
        var_pa_t_dn3: f64,
        var_pa_t_dn4: f64,
        var_pa_t_dn5: f64,
        var_pcjp__blk282: f64,
        var_pp_t: f64,
        var_pp_t_db0: f64,
        var_pp_t_db1: f64,
        var_pp_t_dn0: f64,
        var_pp_t_dn1: f64,
        var_pp_t_dn2: f64,
        var_pp_t_dn3: f64,
        var_pp_t_dn4: f64,
        var_pp_t_dn5: f64,
        var_vcl: f64,
        var_vcl_db0: f64,
        var_vcl_db1: f64,
        var_vcl_dn0: f64,
        var_vcl_dn1: f64,
        var_vcl_dn2: f64,
        var_vcl_dn3: f64,
        var_vcl_dn4: f64,
        var_vcl_dn5: f64,
        var_arga__blk283_slot: &mut f64,
        var_arga__blk283_db0_slot: &mut f64,
        var_arga__blk283_db1_slot: &mut f64,
        var_arga__blk283_dn0_slot: &mut f64,
        var_arga__blk283_dn1_slot: &mut f64,
        var_arga__blk283_dn2_slot: &mut f64,
        var_arga__blk283_dn3_slot: &mut f64,
        var_arga__blk283_dn4_slot: &mut f64,
        var_arga__blk283_dn5_slot: &mut f64,
        var_arga__blk283_rdb0_slot: &mut f64,
        var_arga__blk283_rdb1_slot: &mut f64,
        var_arga__blk283_rdn0_slot: &mut f64,
        var_arga__blk283_rdn1_slot: &mut f64,
        var_arga__blk283_rdn2_slot: &mut f64,
        var_arga__blk283_rdn3_slot: &mut f64,
        var_arga__blk283_rdn4_slot: &mut f64,
        var_arga__blk283_rdn5_slot: &mut f64,
        var_arga__blk283_rv_slot: &mut f64,
        var_dv0__blk299_slot: &mut f64,
        var_dv0__blk299_db0_slot: &mut f64,
        var_dv0__blk299_db1_slot: &mut f64,
        var_dv0__blk299_dn0_slot: &mut f64,
        var_dv0__blk299_dn1_slot: &mut f64,
        var_dv0__blk299_dn2_slot: &mut f64,
        var_dv0__blk299_dn3_slot: &mut f64,
        var_dv0__blk299_dn4_slot: &mut f64,
        var_dv0__blk299_dn5_slot: &mut f64,
        var_dv0__blk299_rdb0_slot: &mut f64,
        var_dv0__blk299_rdb1_slot: &mut f64,
        var_dv0__blk299_rdn0_slot: &mut f64,
        var_dv0__blk299_rdn1_slot: &mut f64,
        var_dv0__blk299_rdn2_slot: &mut f64,
        var_dv0__blk299_rdn3_slot: &mut f64,
        var_dv0__blk299_rdn4_slot: &mut f64,
        var_dv0__blk299_rdn5_slot: &mut f64,
        var_dv0__blk299_rv_slot: &mut f64,
        var_dv__blk293_slot: &mut f64,
        var_dv__blk293_db0_slot: &mut f64,
        var_dv__blk293_db1_slot: &mut f64,
        var_dv__blk293_dn0_slot: &mut f64,
        var_dv__blk293_dn1_slot: &mut f64,
        var_dv__blk293_dn2_slot: &mut f64,
        var_dv__blk293_dn3_slot: &mut f64,
        var_dv__blk293_dn4_slot: &mut f64,
        var_dv__blk293_dn5_slot: &mut f64,
        var_dv__blk293_rdb0_slot: &mut f64,
        var_dv__blk293_rdb1_slot: &mut f64,
        var_dv__blk293_rdn0_slot: &mut f64,
        var_dv__blk293_rdn1_slot: &mut f64,
        var_dv__blk293_rdn2_slot: &mut f64,
        var_dv__blk293_rdn3_slot: &mut f64,
        var_dv__blk293_rdn4_slot: &mut f64,
        var_dv__blk293_rdn5_slot: &mut f64,
        var_dv__blk293_rv_slot: &mut f64,
        var_guard297_slot: &mut f64,
        var_guard297_db0_slot: &mut f64,
        var_guard297_db1_slot: &mut f64,
        var_guard297_dn0_slot: &mut f64,
        var_guard297_dn1_slot: &mut f64,
        var_guard297_dn2_slot: &mut f64,
        var_guard297_dn3_slot: &mut f64,
        var_guard297_dn4_slot: &mut f64,
        var_guard297_dn5_slot: &mut f64,
        var_guard297_rdb0_slot: &mut f64,
        var_guard297_rdb1_slot: &mut f64,
        var_guard297_rdn0_slot: &mut f64,
        var_guard297_rdn1_slot: &mut f64,
        var_guard297_rdn2_slot: &mut f64,
        var_guard297_rdn3_slot: &mut f64,
        var_guard297_rdn4_slot: &mut f64,
        var_guard297_rdn5_slot: &mut f64,
        var_guard297_rv_slot: &mut f64,
        var_guard298_slot: &mut f64,
        var_guard298_db0_slot: &mut f64,
        var_guard298_db1_slot: &mut f64,
        var_guard298_dn0_slot: &mut f64,
        var_guard298_dn1_slot: &mut f64,
        var_guard298_dn2_slot: &mut f64,
        var_guard298_dn3_slot: &mut f64,
        var_guard298_dn4_slot: &mut f64,
        var_guard298_dn5_slot: &mut f64,
        var_guard298_rdb0_slot: &mut f64,
        var_guard298_rdb1_slot: &mut f64,
        var_guard298_rdn0_slot: &mut f64,
        var_guard298_rdn1_slot: &mut f64,
        var_guard298_rdn2_slot: &mut f64,
        var_guard298_rdn3_slot: &mut f64,
        var_guard298_rdn4_slot: &mut f64,
        var_guard298_rdn5_slot: &mut f64,
        var_guard298_rv_slot: &mut f64,
        var_mv0__blk291_slot: &mut f64,
        var_mv0__blk291_db0_slot: &mut f64,
        var_mv0__blk291_db1_slot: &mut f64,
        var_mv0__blk291_dn0_slot: &mut f64,
        var_mv0__blk291_dn1_slot: &mut f64,
        var_mv0__blk291_dn2_slot: &mut f64,
        var_mv0__blk291_dn3_slot: &mut f64,
        var_mv0__blk291_dn4_slot: &mut f64,
        var_mv0__blk291_dn5_slot: &mut f64,
        var_mv0__blk291_rdb0_slot: &mut f64,
        var_mv0__blk291_rdb1_slot: &mut f64,
        var_mv0__blk291_rdn0_slot: &mut f64,
        var_mv0__blk291_rdn1_slot: &mut f64,
        var_mv0__blk291_rdn2_slot: &mut f64,
        var_mv0__blk291_rdn3_slot: &mut f64,
        var_mv0__blk291_rdn4_slot: &mut f64,
        var_mv0__blk291_rdn5_slot: &mut f64,
        var_mv0__blk291_rv_slot: &mut f64,
        var_mv__blk294_slot: &mut f64,
        var_mv__blk294_db0_slot: &mut f64,
        var_mv__blk294_db1_slot: &mut f64,
        var_mv__blk294_dn0_slot: &mut f64,
        var_mv__blk294_dn1_slot: &mut f64,
        var_mv__blk294_dn2_slot: &mut f64,
        var_mv__blk294_dn3_slot: &mut f64,
        var_mv__blk294_dn4_slot: &mut f64,
        var_mv__blk294_dn5_slot: &mut f64,
        var_mv__blk294_rdb0_slot: &mut f64,
        var_mv__blk294_rdb1_slot: &mut f64,
        var_mv__blk294_rdn0_slot: &mut f64,
        var_mv__blk294_rdn1_slot: &mut f64,
        var_mv__blk294_rdn2_slot: &mut f64,
        var_mv__blk294_rdn3_slot: &mut f64,
        var_mv__blk294_rdn4_slot: &mut f64,
        var_mv__blk294_rdn5_slot: &mut f64,
        var_mv__blk294_rv_slot: &mut f64,
        var_pwq__blk288_slot: &mut f64,
        var_pwq__blk288_db0_slot: &mut f64,
        var_pwq__blk288_db1_slot: &mut f64,
        var_pwq__blk288_dn0_slot: &mut f64,
        var_pwq__blk288_dn1_slot: &mut f64,
        var_pwq__blk288_dn2_slot: &mut f64,
        var_pwq__blk288_dn3_slot: &mut f64,
        var_pwq__blk288_dn4_slot: &mut f64,
        var_pwq__blk288_dn5_slot: &mut f64,
        var_pwq__blk288_rdb0_slot: &mut f64,
        var_pwq__blk288_rdb1_slot: &mut f64,
        var_pwq__blk288_rdn0_slot: &mut f64,
        var_pwq__blk288_rdn1_slot: &mut f64,
        var_pwq__blk288_rdn2_slot: &mut f64,
        var_pwq__blk288_rdn3_slot: &mut f64,
        var_pwq__blk288_rdn4_slot: &mut f64,
        var_pwq__blk288_rdn5_slot: &mut f64,
        var_pwq__blk288_rv_slot: &mut f64,
        var_qhi__blk290_slot: &mut f64,
        var_qhi__blk290_db0_slot: &mut f64,
        var_qhi__blk290_db1_slot: &mut f64,
        var_qhi__blk290_dn0_slot: &mut f64,
        var_qhi__blk290_dn1_slot: &mut f64,
        var_qhi__blk290_dn2_slot: &mut f64,
        var_qhi__blk290_dn3_slot: &mut f64,
        var_qhi__blk290_dn4_slot: &mut f64,
        var_qhi__blk290_dn5_slot: &mut f64,
        var_qhi__blk290_rdb0_slot: &mut f64,
        var_qhi__blk290_rdb1_slot: &mut f64,
        var_qhi__blk290_rdn0_slot: &mut f64,
        var_qhi__blk290_rdn1_slot: &mut f64,
        var_qhi__blk290_rdn2_slot: &mut f64,
        var_qhi__blk290_rdn3_slot: &mut f64,
        var_qhi__blk290_rdn4_slot: &mut f64,
        var_qhi__blk290_rdn5_slot: &mut f64,
        var_qhi__blk290_rv_slot: &mut f64,
        var_qlo__blk289_slot: &mut f64,
        var_qlo__blk289_db0_slot: &mut f64,
        var_qlo__blk289_db1_slot: &mut f64,
        var_qlo__blk289_dn0_slot: &mut f64,
        var_qlo__blk289_dn1_slot: &mut f64,
        var_qlo__blk289_dn2_slot: &mut f64,
        var_qlo__blk289_dn3_slot: &mut f64,
        var_qlo__blk289_dn4_slot: &mut f64,
        var_qlo__blk289_dn5_slot: &mut f64,
        var_qlo__blk289_rdb0_slot: &mut f64,
        var_qlo__blk289_rdb1_slot: &mut f64,
        var_qlo__blk289_rdn0_slot: &mut f64,
        var_qlo__blk289_rdn1_slot: &mut f64,
        var_qlo__blk289_rdn2_slot: &mut f64,
        var_qlo__blk289_rdn3_slot: &mut f64,
        var_qlo__blk289_rdn4_slot: &mut f64,
        var_qlo__blk289_rdn5_slot: &mut f64,
        var_qlo__blk289_rv_slot: &mut f64,
        var_vl0__blk292_slot: &mut f64,
        var_vl0__blk292_db0_slot: &mut f64,
        var_vl0__blk292_db1_slot: &mut f64,
        var_vl0__blk292_dn0_slot: &mut f64,
        var_vl0__blk292_dn1_slot: &mut f64,
        var_vl0__blk292_dn2_slot: &mut f64,
        var_vl0__blk292_dn3_slot: &mut f64,
        var_vl0__blk292_dn4_slot: &mut f64,
        var_vl0__blk292_dn5_slot: &mut f64,
        var_vl0__blk292_rdb0_slot: &mut f64,
        var_vl0__blk292_rdb1_slot: &mut f64,
        var_vl0__blk292_rdn0_slot: &mut f64,
        var_vl0__blk292_rdn1_slot: &mut f64,
        var_vl0__blk292_rdn2_slot: &mut f64,
        var_vl0__blk292_rdn3_slot: &mut f64,
        var_vl0__blk292_rdn4_slot: &mut f64,
        var_vl0__blk292_rdn5_slot: &mut f64,
        var_vl0__blk292_rv_slot: &mut f64,
        var_vl__blk295_slot: &mut f64,
        var_vl__blk295_db0_slot: &mut f64,
        var_vl__blk295_db1_slot: &mut f64,
        var_vl__blk295_dn0_slot: &mut f64,
        var_vl__blk295_dn1_slot: &mut f64,
        var_vl__blk295_dn2_slot: &mut f64,
        var_vl__blk295_dn3_slot: &mut f64,
        var_vl__blk295_dn4_slot: &mut f64,
        var_vl__blk295_dn5_slot: &mut f64,
        var_vl__blk295_rdb0_slot: &mut f64,
        var_vl__blk295_rdb1_slot: &mut f64,
        var_vl__blk295_rdn0_slot: &mut f64,
        var_vl__blk295_rdn1_slot: &mut f64,
        var_vl__blk295_rdn2_slot: &mut f64,
        var_vl__blk295_rdn3_slot: &mut f64,
        var_vl__blk295_rdn4_slot: &mut f64,
        var_vl__blk295_rdn5_slot: &mut f64,
        var_vl__blk295_rv_slot: &mut f64,
    ) {
        let mut var_arga__blk283: f64 = *var_arga__blk283_slot;
        let mut var_arga__blk283_db0: f64 = *var_arga__blk283_db0_slot;
        let mut var_arga__blk283_db1: f64 = *var_arga__blk283_db1_slot;
        let mut var_arga__blk283_dn0: f64 = *var_arga__blk283_dn0_slot;
        let mut var_arga__blk283_dn1: f64 = *var_arga__blk283_dn1_slot;
        let mut var_arga__blk283_dn2: f64 = *var_arga__blk283_dn2_slot;
        let mut var_arga__blk283_dn3: f64 = *var_arga__blk283_dn3_slot;
        let mut var_arga__blk283_dn4: f64 = *var_arga__blk283_dn4_slot;
        let mut var_arga__blk283_dn5: f64 = *var_arga__blk283_dn5_slot;
        let mut var_arga__blk283_rdb0: f64 = *var_arga__blk283_rdb0_slot;
        let mut var_arga__blk283_rdb1: f64 = *var_arga__blk283_rdb1_slot;
        let mut var_arga__blk283_rdn0: f64 = *var_arga__blk283_rdn0_slot;
        let mut var_arga__blk283_rdn1: f64 = *var_arga__blk283_rdn1_slot;
        let mut var_arga__blk283_rdn2: f64 = *var_arga__blk283_rdn2_slot;
        let mut var_arga__blk283_rdn3: f64 = *var_arga__blk283_rdn3_slot;
        let mut var_arga__blk283_rdn4: f64 = *var_arga__blk283_rdn4_slot;
        let mut var_arga__blk283_rdn5: f64 = *var_arga__blk283_rdn5_slot;
        let mut var_arga__blk283_rv: f64 = *var_arga__blk283_rv_slot;
        let mut var_dv0__blk299: f64 = *var_dv0__blk299_slot;
        let mut var_dv0__blk299_db0: f64 = *var_dv0__blk299_db0_slot;
        let mut var_dv0__blk299_db1: f64 = *var_dv0__blk299_db1_slot;
        let mut var_dv0__blk299_dn0: f64 = *var_dv0__blk299_dn0_slot;
        let mut var_dv0__blk299_dn1: f64 = *var_dv0__blk299_dn1_slot;
        let mut var_dv0__blk299_dn2: f64 = *var_dv0__blk299_dn2_slot;
        let mut var_dv0__blk299_dn3: f64 = *var_dv0__blk299_dn3_slot;
        let mut var_dv0__blk299_dn4: f64 = *var_dv0__blk299_dn4_slot;
        let mut var_dv0__blk299_dn5: f64 = *var_dv0__blk299_dn5_slot;
        let mut var_dv0__blk299_rdb0: f64 = *var_dv0__blk299_rdb0_slot;
        let mut var_dv0__blk299_rdb1: f64 = *var_dv0__blk299_rdb1_slot;
        let mut var_dv0__blk299_rdn0: f64 = *var_dv0__blk299_rdn0_slot;
        let mut var_dv0__blk299_rdn1: f64 = *var_dv0__blk299_rdn1_slot;
        let mut var_dv0__blk299_rdn2: f64 = *var_dv0__blk299_rdn2_slot;
        let mut var_dv0__blk299_rdn3: f64 = *var_dv0__blk299_rdn3_slot;
        let mut var_dv0__blk299_rdn4: f64 = *var_dv0__blk299_rdn4_slot;
        let mut var_dv0__blk299_rdn5: f64 = *var_dv0__blk299_rdn5_slot;
        let mut var_dv0__blk299_rv: f64 = *var_dv0__blk299_rv_slot;
        let mut var_dv__blk293: f64 = *var_dv__blk293_slot;
        let mut var_dv__blk293_db0: f64 = *var_dv__blk293_db0_slot;
        let mut var_dv__blk293_db1: f64 = *var_dv__blk293_db1_slot;
        let mut var_dv__blk293_dn0: f64 = *var_dv__blk293_dn0_slot;
        let mut var_dv__blk293_dn1: f64 = *var_dv__blk293_dn1_slot;
        let mut var_dv__blk293_dn2: f64 = *var_dv__blk293_dn2_slot;
        let mut var_dv__blk293_dn3: f64 = *var_dv__blk293_dn3_slot;
        let mut var_dv__blk293_dn4: f64 = *var_dv__blk293_dn4_slot;
        let mut var_dv__blk293_dn5: f64 = *var_dv__blk293_dn5_slot;
        let mut var_dv__blk293_rdb0: f64 = *var_dv__blk293_rdb0_slot;
        let mut var_dv__blk293_rdb1: f64 = *var_dv__blk293_rdb1_slot;
        let mut var_dv__blk293_rdn0: f64 = *var_dv__blk293_rdn0_slot;
        let mut var_dv__blk293_rdn1: f64 = *var_dv__blk293_rdn1_slot;
        let mut var_dv__blk293_rdn2: f64 = *var_dv__blk293_rdn2_slot;
        let mut var_dv__blk293_rdn3: f64 = *var_dv__blk293_rdn3_slot;
        let mut var_dv__blk293_rdn4: f64 = *var_dv__blk293_rdn4_slot;
        let mut var_dv__blk293_rdn5: f64 = *var_dv__blk293_rdn5_slot;
        let mut var_dv__blk293_rv: f64 = *var_dv__blk293_rv_slot;
        let mut var_guard297: f64 = *var_guard297_slot;
        let mut var_guard297_db0: f64 = *var_guard297_db0_slot;
        let mut var_guard297_db1: f64 = *var_guard297_db1_slot;
        let mut var_guard297_dn0: f64 = *var_guard297_dn0_slot;
        let mut var_guard297_dn1: f64 = *var_guard297_dn1_slot;
        let mut var_guard297_dn2: f64 = *var_guard297_dn2_slot;
        let mut var_guard297_dn3: f64 = *var_guard297_dn3_slot;
        let mut var_guard297_dn4: f64 = *var_guard297_dn4_slot;
        let mut var_guard297_dn5: f64 = *var_guard297_dn5_slot;
        let mut var_guard297_rdb0: f64 = *var_guard297_rdb0_slot;
        let mut var_guard297_rdb1: f64 = *var_guard297_rdb1_slot;
        let mut var_guard297_rdn0: f64 = *var_guard297_rdn0_slot;
        let mut var_guard297_rdn1: f64 = *var_guard297_rdn1_slot;
        let mut var_guard297_rdn2: f64 = *var_guard297_rdn2_slot;
        let mut var_guard297_rdn3: f64 = *var_guard297_rdn3_slot;
        let mut var_guard297_rdn4: f64 = *var_guard297_rdn4_slot;
        let mut var_guard297_rdn5: f64 = *var_guard297_rdn5_slot;
        let mut var_guard297_rv: f64 = *var_guard297_rv_slot;
        let mut var_guard298: f64 = *var_guard298_slot;
        let mut var_guard298_db0: f64 = *var_guard298_db0_slot;
        let mut var_guard298_db1: f64 = *var_guard298_db1_slot;
        let mut var_guard298_dn0: f64 = *var_guard298_dn0_slot;
        let mut var_guard298_dn1: f64 = *var_guard298_dn1_slot;
        let mut var_guard298_dn2: f64 = *var_guard298_dn2_slot;
        let mut var_guard298_dn3: f64 = *var_guard298_dn3_slot;
        let mut var_guard298_dn4: f64 = *var_guard298_dn4_slot;
        let mut var_guard298_dn5: f64 = *var_guard298_dn5_slot;
        let mut var_guard298_rdb0: f64 = *var_guard298_rdb0_slot;
        let mut var_guard298_rdb1: f64 = *var_guard298_rdb1_slot;
        let mut var_guard298_rdn0: f64 = *var_guard298_rdn0_slot;
        let mut var_guard298_rdn1: f64 = *var_guard298_rdn1_slot;
        let mut var_guard298_rdn2: f64 = *var_guard298_rdn2_slot;
        let mut var_guard298_rdn3: f64 = *var_guard298_rdn3_slot;
        let mut var_guard298_rdn4: f64 = *var_guard298_rdn4_slot;
        let mut var_guard298_rdn5: f64 = *var_guard298_rdn5_slot;
        let mut var_guard298_rv: f64 = *var_guard298_rv_slot;
        let mut var_mv0__blk291: f64 = *var_mv0__blk291_slot;
        let mut var_mv0__blk291_db0: f64 = *var_mv0__blk291_db0_slot;
        let mut var_mv0__blk291_db1: f64 = *var_mv0__blk291_db1_slot;
        let mut var_mv0__blk291_dn0: f64 = *var_mv0__blk291_dn0_slot;
        let mut var_mv0__blk291_dn1: f64 = *var_mv0__blk291_dn1_slot;
        let mut var_mv0__blk291_dn2: f64 = *var_mv0__blk291_dn2_slot;
        let mut var_mv0__blk291_dn3: f64 = *var_mv0__blk291_dn3_slot;
        let mut var_mv0__blk291_dn4: f64 = *var_mv0__blk291_dn4_slot;
        let mut var_mv0__blk291_dn5: f64 = *var_mv0__blk291_dn5_slot;
        let mut var_mv0__blk291_rdb0: f64 = *var_mv0__blk291_rdb0_slot;
        let mut var_mv0__blk291_rdb1: f64 = *var_mv0__blk291_rdb1_slot;
        let mut var_mv0__blk291_rdn0: f64 = *var_mv0__blk291_rdn0_slot;
        let mut var_mv0__blk291_rdn1: f64 = *var_mv0__blk291_rdn1_slot;
        let mut var_mv0__blk291_rdn2: f64 = *var_mv0__blk291_rdn2_slot;
        let mut var_mv0__blk291_rdn3: f64 = *var_mv0__blk291_rdn3_slot;
        let mut var_mv0__blk291_rdn4: f64 = *var_mv0__blk291_rdn4_slot;
        let mut var_mv0__blk291_rdn5: f64 = *var_mv0__blk291_rdn5_slot;
        let mut var_mv0__blk291_rv: f64 = *var_mv0__blk291_rv_slot;
        let mut var_mv__blk294: f64 = *var_mv__blk294_slot;
        let mut var_mv__blk294_db0: f64 = *var_mv__blk294_db0_slot;
        let mut var_mv__blk294_db1: f64 = *var_mv__blk294_db1_slot;
        let mut var_mv__blk294_dn0: f64 = *var_mv__blk294_dn0_slot;
        let mut var_mv__blk294_dn1: f64 = *var_mv__blk294_dn1_slot;
        let mut var_mv__blk294_dn2: f64 = *var_mv__blk294_dn2_slot;
        let mut var_mv__blk294_dn3: f64 = *var_mv__blk294_dn3_slot;
        let mut var_mv__blk294_dn4: f64 = *var_mv__blk294_dn4_slot;
        let mut var_mv__blk294_dn5: f64 = *var_mv__blk294_dn5_slot;
        let mut var_mv__blk294_rdb0: f64 = *var_mv__blk294_rdb0_slot;
        let mut var_mv__blk294_rdb1: f64 = *var_mv__blk294_rdb1_slot;
        let mut var_mv__blk294_rdn0: f64 = *var_mv__blk294_rdn0_slot;
        let mut var_mv__blk294_rdn1: f64 = *var_mv__blk294_rdn1_slot;
        let mut var_mv__blk294_rdn2: f64 = *var_mv__blk294_rdn2_slot;
        let mut var_mv__blk294_rdn3: f64 = *var_mv__blk294_rdn3_slot;
        let mut var_mv__blk294_rdn4: f64 = *var_mv__blk294_rdn4_slot;
        let mut var_mv__blk294_rdn5: f64 = *var_mv__blk294_rdn5_slot;
        let mut var_mv__blk294_rv: f64 = *var_mv__blk294_rv_slot;
        let mut var_pwq__blk288: f64 = *var_pwq__blk288_slot;
        let mut var_pwq__blk288_db0: f64 = *var_pwq__blk288_db0_slot;
        let mut var_pwq__blk288_db1: f64 = *var_pwq__blk288_db1_slot;
        let mut var_pwq__blk288_dn0: f64 = *var_pwq__blk288_dn0_slot;
        let mut var_pwq__blk288_dn1: f64 = *var_pwq__blk288_dn1_slot;
        let mut var_pwq__blk288_dn2: f64 = *var_pwq__blk288_dn2_slot;
        let mut var_pwq__blk288_dn3: f64 = *var_pwq__blk288_dn3_slot;
        let mut var_pwq__blk288_dn4: f64 = *var_pwq__blk288_dn4_slot;
        let mut var_pwq__blk288_dn5: f64 = *var_pwq__blk288_dn5_slot;
        let mut var_pwq__blk288_rdb0: f64 = *var_pwq__blk288_rdb0_slot;
        let mut var_pwq__blk288_rdb1: f64 = *var_pwq__blk288_rdb1_slot;
        let mut var_pwq__blk288_rdn0: f64 = *var_pwq__blk288_rdn0_slot;
        let mut var_pwq__blk288_rdn1: f64 = *var_pwq__blk288_rdn1_slot;
        let mut var_pwq__blk288_rdn2: f64 = *var_pwq__blk288_rdn2_slot;
        let mut var_pwq__blk288_rdn3: f64 = *var_pwq__blk288_rdn3_slot;
        let mut var_pwq__blk288_rdn4: f64 = *var_pwq__blk288_rdn4_slot;
        let mut var_pwq__blk288_rdn5: f64 = *var_pwq__blk288_rdn5_slot;
        let mut var_pwq__blk288_rv: f64 = *var_pwq__blk288_rv_slot;
        let mut var_qhi__blk290: f64 = *var_qhi__blk290_slot;
        let mut var_qhi__blk290_db0: f64 = *var_qhi__blk290_db0_slot;
        let mut var_qhi__blk290_db1: f64 = *var_qhi__blk290_db1_slot;
        let mut var_qhi__blk290_dn0: f64 = *var_qhi__blk290_dn0_slot;
        let mut var_qhi__blk290_dn1: f64 = *var_qhi__blk290_dn1_slot;
        let mut var_qhi__blk290_dn2: f64 = *var_qhi__blk290_dn2_slot;
        let mut var_qhi__blk290_dn3: f64 = *var_qhi__blk290_dn3_slot;
        let mut var_qhi__blk290_dn4: f64 = *var_qhi__blk290_dn4_slot;
        let mut var_qhi__blk290_dn5: f64 = *var_qhi__blk290_dn5_slot;
        let mut var_qhi__blk290_rdb0: f64 = *var_qhi__blk290_rdb0_slot;
        let mut var_qhi__blk290_rdb1: f64 = *var_qhi__blk290_rdb1_slot;
        let mut var_qhi__blk290_rdn0: f64 = *var_qhi__blk290_rdn0_slot;
        let mut var_qhi__blk290_rdn1: f64 = *var_qhi__blk290_rdn1_slot;
        let mut var_qhi__blk290_rdn2: f64 = *var_qhi__blk290_rdn2_slot;
        let mut var_qhi__blk290_rdn3: f64 = *var_qhi__blk290_rdn3_slot;
        let mut var_qhi__blk290_rdn4: f64 = *var_qhi__blk290_rdn4_slot;
        let mut var_qhi__blk290_rdn5: f64 = *var_qhi__blk290_rdn5_slot;
        let mut var_qhi__blk290_rv: f64 = *var_qhi__blk290_rv_slot;
        let mut var_qlo__blk289: f64 = *var_qlo__blk289_slot;
        let mut var_qlo__blk289_db0: f64 = *var_qlo__blk289_db0_slot;
        let mut var_qlo__blk289_db1: f64 = *var_qlo__blk289_db1_slot;
        let mut var_qlo__blk289_dn0: f64 = *var_qlo__blk289_dn0_slot;
        let mut var_qlo__blk289_dn1: f64 = *var_qlo__blk289_dn1_slot;
        let mut var_qlo__blk289_dn2: f64 = *var_qlo__blk289_dn2_slot;
        let mut var_qlo__blk289_dn3: f64 = *var_qlo__blk289_dn3_slot;
        let mut var_qlo__blk289_dn4: f64 = *var_qlo__blk289_dn4_slot;
        let mut var_qlo__blk289_dn5: f64 = *var_qlo__blk289_dn5_slot;
        let mut var_qlo__blk289_rdb0: f64 = *var_qlo__blk289_rdb0_slot;
        let mut var_qlo__blk289_rdb1: f64 = *var_qlo__blk289_rdb1_slot;
        let mut var_qlo__blk289_rdn0: f64 = *var_qlo__blk289_rdn0_slot;
        let mut var_qlo__blk289_rdn1: f64 = *var_qlo__blk289_rdn1_slot;
        let mut var_qlo__blk289_rdn2: f64 = *var_qlo__blk289_rdn2_slot;
        let mut var_qlo__blk289_rdn3: f64 = *var_qlo__blk289_rdn3_slot;
        let mut var_qlo__blk289_rdn4: f64 = *var_qlo__blk289_rdn4_slot;
        let mut var_qlo__blk289_rdn5: f64 = *var_qlo__blk289_rdn5_slot;
        let mut var_qlo__blk289_rv: f64 = *var_qlo__blk289_rv_slot;
        let mut var_vl0__blk292: f64 = *var_vl0__blk292_slot;
        let mut var_vl0__blk292_db0: f64 = *var_vl0__blk292_db0_slot;
        let mut var_vl0__blk292_db1: f64 = *var_vl0__blk292_db1_slot;
        let mut var_vl0__blk292_dn0: f64 = *var_vl0__blk292_dn0_slot;
        let mut var_vl0__blk292_dn1: f64 = *var_vl0__blk292_dn1_slot;
        let mut var_vl0__blk292_dn2: f64 = *var_vl0__blk292_dn2_slot;
        let mut var_vl0__blk292_dn3: f64 = *var_vl0__blk292_dn3_slot;
        let mut var_vl0__blk292_dn4: f64 = *var_vl0__blk292_dn4_slot;
        let mut var_vl0__blk292_dn5: f64 = *var_vl0__blk292_dn5_slot;
        let mut var_vl0__blk292_rdb0: f64 = *var_vl0__blk292_rdb0_slot;
        let mut var_vl0__blk292_rdb1: f64 = *var_vl0__blk292_rdb1_slot;
        let mut var_vl0__blk292_rdn0: f64 = *var_vl0__blk292_rdn0_slot;
        let mut var_vl0__blk292_rdn1: f64 = *var_vl0__blk292_rdn1_slot;
        let mut var_vl0__blk292_rdn2: f64 = *var_vl0__blk292_rdn2_slot;
        let mut var_vl0__blk292_rdn3: f64 = *var_vl0__blk292_rdn3_slot;
        let mut var_vl0__blk292_rdn4: f64 = *var_vl0__blk292_rdn4_slot;
        let mut var_vl0__blk292_rdn5: f64 = *var_vl0__blk292_rdn5_slot;
        let mut var_vl0__blk292_rv: f64 = *var_vl0__blk292_rv_slot;
        let mut var_vl__blk295: f64 = *var_vl__blk295_slot;
        let mut var_vl__blk295_db0: f64 = *var_vl__blk295_db0_slot;
        let mut var_vl__blk295_db1: f64 = *var_vl__blk295_db1_slot;
        let mut var_vl__blk295_dn0: f64 = *var_vl__blk295_dn0_slot;
        let mut var_vl__blk295_dn1: f64 = *var_vl__blk295_dn1_slot;
        let mut var_vl__blk295_dn2: f64 = *var_vl__blk295_dn2_slot;
        let mut var_vl__blk295_dn3: f64 = *var_vl__blk295_dn3_slot;
        let mut var_vl__blk295_dn4: f64 = *var_vl__blk295_dn4_slot;
        let mut var_vl__blk295_dn5: f64 = *var_vl__blk295_dn5_slot;
        let mut var_vl__blk295_rdb0: f64 = *var_vl__blk295_rdb0_slot;
        let mut var_vl__blk295_rdb1: f64 = *var_vl__blk295_rdb1_slot;
        let mut var_vl__blk295_rdn0: f64 = *var_vl__blk295_rdn0_slot;
        let mut var_vl__blk295_rdn1: f64 = *var_vl__blk295_rdn1_slot;
        let mut var_vl__blk295_rdn2: f64 = *var_vl__blk295_rdn2_slot;
        let mut var_vl__blk295_rdn3: f64 = *var_vl__blk295_rdn3_slot;
        let mut var_vl__blk295_rdn4: f64 = *var_vl__blk295_rdn4_slot;
        let mut var_vl__blk295_rdn5: f64 = *var_vl__blk295_rdn5_slot;
        let mut var_vl__blk295_rv: f64 = *var_vl__blk295_rv_slot;

        let assign4340_e4353: f64 = if var_dvh__blk287 > 0.0 { 1.0 } else { 0.0 };
        var_guard297 = assign4340_e4353;
        var_guard297_dn0 = 0.0;
        var_guard297_dn1 = 0.0;
        var_guard297_dn2 = 0.0;
        var_guard297_dn3 = 0.0;
        var_guard297_dn4 = 0.0;
        var_guard297_dn5 = 0.0;
        var_guard297_db0 = 0.0;
        var_guard297_db1 = 0.0;
        var_guard297_rv = 0.0;
        var_guard297_rdn0 = 0.0;
        var_guard297_rdn1 = 0.0;
        var_guard297_rdn2 = 0.0;
        var_guard297_rdn3 = 0.0;
        var_guard297_rdn4 = 0.0;
        var_guard297_rdn5 = 0.0;
        var_guard297_rdb0 = 0.0;
        var_guard297_rdb1 = 0.0;

        let (assign4350_e4368, assign4350_e4368_d_n0, assign4350_e4368_d_n1, assign4350_e4368_d_n2, assign4350_e4368_d_n3, assign4350_e4368_d_n4, assign4350_e4368_d_n5, assign4350_e4368_d_b0, assign4350_e4368_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) && (var_guard297 != 0.0)) {
        let assign4350_e4363: f64 = (1.0 - p.p68);
        let assign4350_e4365: f64 = (-p.p74);
        let assign4350_e4366: f64 = (assign4350_e4363).powf(assign4350_e4365);
        (assign4350_e4366, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq__blk288, var_pwq__blk288_dn0, var_pwq__blk288_dn1, var_pwq__blk288_dn2, var_pwq__blk288_dn3, var_pwq__blk288_dn4, var_pwq__blk288_dn5, var_pwq__blk288_db0, var_pwq__blk288_db1,)
    }
};
        var_pwq__blk288 = assign4350_e4368;
        var_pwq__blk288_dn0 = assign4350_e4368_d_n0;
        var_pwq__blk288_dn1 = assign4350_e4368_d_n1;
        var_pwq__blk288_dn2 = assign4350_e4368_d_n2;
        var_pwq__blk288_dn3 = assign4350_e4368_d_n3;
        var_pwq__blk288_dn4 = assign4350_e4368_d_n4;
        var_pwq__blk288_dn5 = assign4350_e4368_d_n5;
        var_pwq__blk288_db0 = assign4350_e4368_d_b0;
        var_pwq__blk288_db1 = assign4350_e4368_d_b1;
        var_pwq__blk288_rv = 0.0;
        var_pwq__blk288_rdn0 = 0.0;
        var_pwq__blk288_rdn1 = 0.0;
        var_pwq__blk288_rdn2 = 0.0;
        var_pwq__blk288_rdn3 = 0.0;
        var_pwq__blk288_rdn4 = 0.0;
        var_pwq__blk288_rdn5 = 0.0;
        var_pwq__blk288_rdb0 = 0.0;
        var_pwq__blk288_rdb1 = 0.0;

        let (assign4360_e4390, assign4360_e4390_d_n0, assign4360_e4390_d_n1, assign4360_e4390_d_n2, assign4360_e4390_d_n3, assign4360_e4390_d_n4, assign4360_e4390_d_n5, assign4360_e4390_d_b0, assign4360_e4390_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) && (var_guard297 != 0.0)) {
        let assign4360_e4381: f64 = (1.0 - p.p68);
        let assign4360_e4382: f64 = (var_pwq__blk288 * assign4360_e4381);
        let assign4360_e4383: f64 = (1.0 - assign4360_e4382);
        let assign4360_e4384: f64 = (var_pa_t * assign4360_e4383);
        let assign4360_e4387: f64 = (1.0 - p.p74);
        let assign4360_e4388: f64 = (assign4360_e4384 / assign4360_e4387);
        (assign4360_e4388, (((var_pa_t_dn0 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn0 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_dn1 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn1 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_dn2 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn2 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_dn3 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn3 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_dn4 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn4 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_dn5 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_dn5 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_db0 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_db0 * assign4360_e4381)))) / assign4360_e4387), (((var_pa_t_db1 * assign4360_e4383) + (var_pa_t * (-(var_pwq__blk288_db1 * assign4360_e4381)))) / assign4360_e4387),)
    } else {
        (var_qlo__blk289, var_qlo__blk289_dn0, var_qlo__blk289_dn1, var_qlo__blk289_dn2, var_qlo__blk289_dn3, var_qlo__blk289_dn4, var_qlo__blk289_dn5, var_qlo__blk289_db0, var_qlo__blk289_db1,)
    }
};
        var_qlo__blk289 = assign4360_e4390;
        var_qlo__blk289_dn0 = assign4360_e4390_d_n0;
        var_qlo__blk289_dn1 = assign4360_e4390_d_n1;
        var_qlo__blk289_dn2 = assign4360_e4390_d_n2;
        var_qlo__blk289_dn3 = assign4360_e4390_d_n3;
        var_qlo__blk289_dn4 = assign4360_e4390_d_n4;
        var_qlo__blk289_dn5 = assign4360_e4390_d_n5;
        var_qlo__blk289_db0 = assign4360_e4390_d_b0;
        var_qlo__blk289_db1 = assign4360_e4390_d_b1;
        var_qlo__blk289_rv = 0.0;
        var_qlo__blk289_rdn0 = 0.0;
        var_qlo__blk289_rdn1 = 0.0;
        var_qlo__blk289_rdn2 = 0.0;
        var_qlo__blk289_rdn3 = 0.0;
        var_qlo__blk289_rdn4 = 0.0;
        var_qlo__blk289_rdn5 = 0.0;
        var_qlo__blk289_rdb0 = 0.0;
        var_qlo__blk289_rdb1 = 0.0;

        let (assign4370_e4416, assign4370_e4416_d_n0, assign4370_e4416_d_n1, assign4370_e4416_d_n2, assign4370_e4416_d_n3, assign4370_e4416_d_n4, assign4370_e4416_d_n5, assign4370_e4416_d_b0, assign4370_e4416_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) && (var_guard297 != 0.0)) {
        let assign4370_e4402: f64 = (0.5 * p.p74);
        let assign4370_e4404: f64 = (assign4370_e4402 * var_dvh__blk287);
        let assign4370_e4408: f64 = (1.0 - p.p68);
        let assign4370_e4409: f64 = (var_pa_t * assign4370_e4408);
        let assign4370_e4410: f64 = (assign4370_e4404 / assign4370_e4409);
        let assign4370_e4411: f64 = (1.0 + assign4370_e4410);
        let assign4370_e4412: f64 = (var_dvh__blk287 * assign4370_e4411);
        let assign4370_e4414: f64 = (assign4370_e4412 * var_pwq__blk288);
        (assign4370_e4414, ((((var_dvh__blk287_dn0 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn0) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn0 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn0)), ((((var_dvh__blk287_dn1 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn1) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn1 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn1)), ((((var_dvh__blk287_dn2 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn2) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn2 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn2)), ((((var_dvh__blk287_dn3 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn3) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn3 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn3)), ((((var_dvh__blk287_dn4 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn4) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn4 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn4)), ((((var_dvh__blk287_dn5 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_dn5) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_dn5 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_dn5)), ((((var_dvh__blk287_db0 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_db0) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_db0 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_db0)), ((((var_dvh__blk287_db1 * assign4370_e4411) + (var_dvh__blk287 * ((((assign4370_e4402 * var_dvh__blk287_db1) * assign4370_e4409) - (assign4370_e4404 * (var_pa_t_db1 * assign4370_e4408))) / (assign4370_e4409 * assign4370_e4409)))) * var_pwq__blk288) + (assign4370_e4412 * var_pwq__blk288_db1)),)
    } else {
        (var_qhi__blk290, var_qhi__blk290_dn0, var_qhi__blk290_dn1, var_qhi__blk290_dn2, var_qhi__blk290_dn3, var_qhi__blk290_dn4, var_qhi__blk290_dn5, var_qhi__blk290_db0, var_qhi__blk290_db1,)
    }
};
        var_qhi__blk290 = assign4370_e4416;
        var_qhi__blk290_dn0 = assign4370_e4416_d_n0;
        var_qhi__blk290_dn1 = assign4370_e4416_d_n1;
        var_qhi__blk290_dn2 = assign4370_e4416_d_n2;
        var_qhi__blk290_dn3 = assign4370_e4416_d_n3;
        var_qhi__blk290_dn4 = assign4370_e4416_d_n4;
        var_qhi__blk290_dn5 = assign4370_e4416_d_n5;
        var_qhi__blk290_db0 = assign4370_e4416_d_b0;
        var_qhi__blk290_db1 = assign4370_e4416_d_b1;
        var_qhi__blk290_rv = 0.0;
        var_qhi__blk290_rdn0 = 0.0;
        var_qhi__blk290_rdn1 = 0.0;
        var_qhi__blk290_rdn2 = 0.0;
        var_qhi__blk290_rdn3 = 0.0;
        var_qhi__blk290_rdn4 = 0.0;
        var_qhi__blk290_rdn5 = 0.0;
        var_qhi__blk290_rdb0 = 0.0;
        var_qhi__blk290_rdb1 = 0.0;

        let (assign4380_e4443, assign4380_e4443_d_n0, assign4380_e4443_d_n1, assign4380_e4443_d_n2, assign4380_e4443_d_n3, assign4380_e4443_d_n4, assign4380_e4443_d_n5, assign4380_e4443_d_b0, assign4380_e4443_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) && (var_guard297 == 0.0)) {
        let assign4380_e4430: f64 = (var_vcl / var_pa_t);
        let assign4380_e4431: f64 = (1.0 - assign4380_e4430);
        let assign4380_e4434: f64 = (1.0 - p.p74);
        let assign4380_e4435: f64 = (assign4380_e4431).powf(assign4380_e4434);
        let assign4380_e4436: f64 = (1.0 - assign4380_e4435);
        let assign4380_e4437: f64 = (var_pa_t * assign4380_e4436);
        let assign4380_e4440: f64 = (1.0 - p.p74);
        let assign4380_e4441: f64 = (assign4380_e4437 / assign4380_e4440);
        (assign4380_e4441, (((var_pa_t_dn0 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn0 * var_pa_t) - (var_vcl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn0 * var_pa_t) - (var_vcl * var_pa_t_dn0)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_dn1 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn1 * var_pa_t) - (var_vcl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn1 * var_pa_t) - (var_vcl * var_pa_t_dn1)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_dn2 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn2 * var_pa_t) - (var_vcl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn2 * var_pa_t) - (var_vcl * var_pa_t_dn2)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_dn3 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn3 * var_pa_t) - (var_vcl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn3 * var_pa_t) - (var_vcl * var_pa_t_dn3)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_dn4 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn4 * var_pa_t) - (var_vcl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn4 * var_pa_t) - (var_vcl * var_pa_t_dn4)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_dn5 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_dn5 * var_pa_t) - (var_vcl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_dn5 * var_pa_t) - (var_vcl * var_pa_t_dn5)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_db0 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_db0 * var_pa_t) - (var_vcl * var_pa_t_db0)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_db0 * var_pa_t) - (var_vcl * var_pa_t_db0)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440), (((var_pa_t_db1 * assign4380_e4436) + (var_pa_t * (-if 0.0 == 0.0 && ((assign4380_e4434) as f64).is_finite() && ((assign4380_e4434) as f64).fract() == 0.0 { if assign4380_e4434 == 0.0 { 0.0 } else { (assign4380_e4434 * ((assign4380_e4431).powf(assign4380_e4434 - 1.0) * (-(((var_vcl_db1 * var_pa_t) - (var_vcl * var_pa_t_db1)) / (var_pa_t * var_pa_t))))) } } else { (assign4380_e4435 * (assign4380_e4434 * ((-(((var_vcl_db1 * var_pa_t) - (var_vcl * var_pa_t_db1)) / (var_pa_t * var_pa_t))) / assign4380_e4431))) }))) / assign4380_e4440),)
    } else {
        (var_qlo__blk289, var_qlo__blk289_dn0, var_qlo__blk289_dn1, var_qlo__blk289_dn2, var_qlo__blk289_dn3, var_qlo__blk289_dn4, var_qlo__blk289_dn5, var_qlo__blk289_db0, var_qlo__blk289_db1,)
    }
};
        var_qlo__blk289 = assign4380_e4443;
        var_qlo__blk289_dn0 = assign4380_e4443_d_n0;
        var_qlo__blk289_dn1 = assign4380_e4443_d_n1;
        var_qlo__blk289_dn2 = assign4380_e4443_d_n2;
        var_qlo__blk289_dn3 = assign4380_e4443_d_n3;
        var_qlo__blk289_dn4 = assign4380_e4443_d_n4;
        var_qlo__blk289_dn5 = assign4380_e4443_d_n5;
        var_qlo__blk289_db0 = assign4380_e4443_d_b0;
        var_qlo__blk289_db1 = assign4380_e4443_d_b1;
        var_qlo__blk289_rv = 0.0;
        var_qlo__blk289_rdn0 = 0.0;
        var_qlo__blk289_rdn1 = 0.0;
        var_qlo__blk289_rdn2 = 0.0;
        var_qlo__blk289_rdn3 = 0.0;
        var_qlo__blk289_rdn4 = 0.0;
        var_qlo__blk289_rdn5 = 0.0;
        var_qlo__blk289_rdb0 = 0.0;
        var_qlo__blk289_rdb1 = 0.0;

        let (assign4390_e4454, assign4390_e4454_d_n0, assign4390_e4454_d_n1, assign4390_e4454_d_n2, assign4390_e4454_d_n3, assign4390_e4454_d_n4, assign4390_e4454_d_n5, assign4390_e4454_d_b0, assign4390_e4454_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) && (var_guard297 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk290, var_qhi__blk290_dn0, var_qhi__blk290_dn1, var_qhi__blk290_dn2, var_qhi__blk290_dn3, var_qhi__blk290_dn4, var_qhi__blk290_dn5, var_qhi__blk290_db0, var_qhi__blk290_db1,)
    }
};
        var_qhi__blk290 = assign4390_e4454;
        var_qhi__blk290_dn0 = assign4390_e4454_d_n0;
        var_qhi__blk290_dn1 = assign4390_e4454_d_n1;
        var_qhi__blk290_dn2 = assign4390_e4454_d_n2;
        var_qhi__blk290_dn3 = assign4390_e4454_d_n3;
        var_qhi__blk290_dn4 = assign4390_e4454_d_n4;
        var_qhi__blk290_dn5 = assign4390_e4454_d_n5;
        var_qhi__blk290_db0 = assign4390_e4454_d_b0;
        var_qhi__blk290_db1 = assign4390_e4454_d_b1;
        var_qhi__blk290_rv = 0.0;
        var_qhi__blk290_rdn0 = 0.0;
        var_qhi__blk290_rdn1 = 0.0;
        var_qhi__blk290_rdn2 = 0.0;
        var_qhi__blk290_rdn3 = 0.0;
        var_qhi__blk290_rdn4 = 0.0;
        var_qhi__blk290_rdn5 = 0.0;
        var_qhi__blk290_rdb0 = 0.0;
        var_qhi__blk290_rdb1 = 0.0;

        let (assign4400_e4464, assign4400_e4464_d_n0, assign4400_e4464_d_n1, assign4400_e4464_d_n2, assign4400_e4464_d_n3, assign4400_e4464_d_n4, assign4400_e4464_d_n5, assign4400_e4464_d_b0, assign4400_e4464_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 != 0.0)) {
        let assign4400_e4462: f64 = (var_qlo__blk289 + var_qhi__blk290);
        (assign4400_e4462, (var_qlo__blk289_dn0 + var_qhi__blk290_dn0), (var_qlo__blk289_dn1 + var_qhi__blk290_dn1), (var_qlo__blk289_dn2 + var_qhi__blk290_dn2), (var_qlo__blk289_dn3 + var_qhi__blk290_dn3), (var_qlo__blk289_dn4 + var_qhi__blk290_dn4), (var_qlo__blk289_dn5 + var_qhi__blk290_dn5), (var_qlo__blk289_db0 + var_qhi__blk290_db0), (var_qlo__blk289_db1 + var_qhi__blk290_db1),)
    } else {
        (var_arga__blk283, var_arga__blk283_dn0, var_arga__blk283_dn1, var_arga__blk283_dn2, var_arga__blk283_dn3, var_arga__blk283_dn4, var_arga__blk283_dn5, var_arga__blk283_db0, var_arga__blk283_db1,)
    }
};
        var_arga__blk283 = assign4400_e4464;
        var_arga__blk283_dn0 = assign4400_e4464_d_n0;
        var_arga__blk283_dn1 = assign4400_e4464_d_n1;
        var_arga__blk283_dn2 = assign4400_e4464_d_n2;
        var_arga__blk283_dn3 = assign4400_e4464_d_n3;
        var_arga__blk283_dn4 = assign4400_e4464_d_n4;
        var_arga__blk283_dn5 = assign4400_e4464_d_n5;
        var_arga__blk283_db0 = assign4400_e4464_d_b0;
        var_arga__blk283_db1 = assign4400_e4464_d_b1;
        var_arga__blk283_rv = 0.0;
        var_arga__blk283_rdn0 = 0.0;
        var_arga__blk283_rdn1 = 0.0;
        var_arga__blk283_rdn2 = 0.0;
        var_arga__blk283_rdn3 = 0.0;
        var_arga__blk283_rdn4 = 0.0;
        var_arga__blk283_rdn5 = 0.0;
        var_arga__blk283_rdb0 = 0.0;
        var_arga__blk283_rdb1 = 0.0;

        let (assign4410_e4482, assign4410_e4482_d_n0, assign4410_e4482_d_n1, assign4410_e4482_d_n2, assign4410_e4482_d_n3, assign4410_e4482_d_n4, assign4410_e4482_d_n5, assign4410_e4482_d_b0, assign4410_e4482_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4410_e4473: f64 = (var_dv0__blk286 * var_dv0__blk286);
        let assign4410_e4476: f64 = (4.0 * p.p75);
        let assign4410_e4478: f64 = (assign4410_e4476 * p.p75);
        let assign4410_e4479: f64 = (assign4410_e4473 + assign4410_e4478);
        let assign4410_e4480: f64 = (assign4410_e4479).sqrt();
        (assign4410_e4480, (((var_dv0__blk286_dn0 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn0)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_dn1 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn1)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_dn2 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn2)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_dn3 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn3)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_dn4 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn4)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_dn5 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_dn5)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_db0 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_db0)) / (2.0 * assign4410_e4480)), (((var_dv0__blk286_db1 * var_dv0__blk286) + (var_dv0__blk286 * var_dv0__blk286_db1)) / (2.0 * assign4410_e4480)),)
    } else {
        (var_mv0__blk291, var_mv0__blk291_dn0, var_mv0__blk291_dn1, var_mv0__blk291_dn2, var_mv0__blk291_dn3, var_mv0__blk291_dn4, var_mv0__blk291_dn5, var_mv0__blk291_db0, var_mv0__blk291_db1,)
    }
};
        var_mv0__blk291 = assign4410_e4482;
        var_mv0__blk291_dn0 = assign4410_e4482_d_n0;
        var_mv0__blk291_dn1 = assign4410_e4482_d_n1;
        var_mv0__blk291_dn2 = assign4410_e4482_d_n2;
        var_mv0__blk291_dn3 = assign4410_e4482_d_n3;
        var_mv0__blk291_dn4 = assign4410_e4482_d_n4;
        var_mv0__blk291_dn5 = assign4410_e4482_d_n5;
        var_mv0__blk291_db0 = assign4410_e4482_d_b0;
        var_mv0__blk291_db1 = assign4410_e4482_d_b1;
        var_mv0__blk291_rv = 0.0;
        var_mv0__blk291_rdn0 = 0.0;
        var_mv0__blk291_rdn1 = 0.0;
        var_mv0__blk291_rdn2 = 0.0;
        var_mv0__blk291_rdn3 = 0.0;
        var_mv0__blk291_rdn4 = 0.0;
        var_mv0__blk291_rdn5 = 0.0;
        var_mv0__blk291_rdb0 = 0.0;
        var_mv0__blk291_rdb1 = 0.0;

        let (assign4420_e4496, assign4420_e4496_d_n0, assign4420_e4496_d_n1, assign4420_e4496_d_n2, assign4420_e4496_d_n3, assign4420_e4496_d_n4, assign4420_e4496_d_n5, assign4420_e4496_d_b0, assign4420_e4496_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4420_e4490: f64 = (-0.5);
        let assign4420_e4493: f64 = (var_dv0__blk286 + var_mv0__blk291);
        let assign4420_e4494: f64 = (assign4420_e4490 * assign4420_e4493);
        (assign4420_e4494, (assign4420_e4490 * (var_dv0__blk286_dn0 + var_mv0__blk291_dn0)), (assign4420_e4490 * (var_dv0__blk286_dn1 + var_mv0__blk291_dn1)), (assign4420_e4490 * (var_dv0__blk286_dn2 + var_mv0__blk291_dn2)), (assign4420_e4490 * (var_dv0__blk286_dn3 + var_mv0__blk291_dn3)), (assign4420_e4490 * (var_dv0__blk286_dn4 + var_mv0__blk291_dn4)), (assign4420_e4490 * (var_dv0__blk286_dn5 + var_mv0__blk291_dn5)), (assign4420_e4490 * (var_dv0__blk286_db0 + var_mv0__blk291_db0)), (assign4420_e4490 * (var_dv0__blk286_db1 + var_mv0__blk291_db1)),)
    } else {
        (var_vl0__blk292, var_vl0__blk292_dn0, var_vl0__blk292_dn1, var_vl0__blk292_dn2, var_vl0__blk292_dn3, var_vl0__blk292_dn4, var_vl0__blk292_dn5, var_vl0__blk292_db0, var_vl0__blk292_db1,)
    }
};
        var_vl0__blk292 = assign4420_e4496;
        var_vl0__blk292_dn0 = assign4420_e4496_d_n0;
        var_vl0__blk292_dn1 = assign4420_e4496_d_n1;
        var_vl0__blk292_dn2 = assign4420_e4496_d_n2;
        var_vl0__blk292_dn3 = assign4420_e4496_d_n3;
        var_vl0__blk292_dn4 = assign4420_e4496_d_n4;
        var_vl0__blk292_dn5 = assign4420_e4496_d_n5;
        var_vl0__blk292_db0 = assign4420_e4496_d_b0;
        var_vl0__blk292_db1 = assign4420_e4496_d_b1;
        var_vl0__blk292_rv = 0.0;
        var_vl0__blk292_rdn0 = 0.0;
        var_vl0__blk292_rdn1 = 0.0;
        var_vl0__blk292_rdn2 = 0.0;
        var_vl0__blk292_rdn3 = 0.0;
        var_vl0__blk292_rdn4 = 0.0;
        var_vl0__blk292_rdn5 = 0.0;
        var_vl0__blk292_rdb0 = 0.0;
        var_vl0__blk292_rdb1 = 0.0;

        let (assign4430_e4507, assign4430_e4507_d_n0, assign4430_e4507_d_n1, assign4430_e4507_d_n2, assign4430_e4507_d_n3, assign4430_e4507_d_n4, assign4430_e4507_d_n5, assign4430_e4507_d_b0, assign4430_e4507_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4430_e4505: f64 = (var_vcl + var_dv0__blk286);
        (assign4430_e4505, (var_vcl_dn0 + var_dv0__blk286_dn0), (var_vcl_dn1 + var_dv0__blk286_dn1), (var_vcl_dn2 + var_dv0__blk286_dn2), (var_vcl_dn3 + var_dv0__blk286_dn3), (var_vcl_dn4 + var_dv0__blk286_dn4), (var_vcl_dn5 + var_dv0__blk286_dn5), (var_vcl_db0 + var_dv0__blk286_db0), (var_vcl_db1 + var_dv0__blk286_db1),)
    } else {
        (var_dv__blk293, var_dv__blk293_dn0, var_dv__blk293_dn1, var_dv__blk293_dn2, var_dv__blk293_dn3, var_dv__blk293_dn4, var_dv__blk293_dn5, var_dv__blk293_db0, var_dv__blk293_db1,)
    }
};
        var_dv__blk293 = assign4430_e4507;
        var_dv__blk293_dn0 = assign4430_e4507_d_n0;
        var_dv__blk293_dn1 = assign4430_e4507_d_n1;
        var_dv__blk293_dn2 = assign4430_e4507_d_n2;
        var_dv__blk293_dn3 = assign4430_e4507_d_n3;
        var_dv__blk293_dn4 = assign4430_e4507_d_n4;
        var_dv__blk293_dn5 = assign4430_e4507_d_n5;
        var_dv__blk293_db0 = assign4430_e4507_d_b0;
        var_dv__blk293_db1 = assign4430_e4507_d_b1;
        var_dv__blk293_rv = 0.0;
        var_dv__blk293_rdn0 = 0.0;
        var_dv__blk293_rdn1 = 0.0;
        var_dv__blk293_rdn2 = 0.0;
        var_dv__blk293_rdn3 = 0.0;
        var_dv__blk293_rdn4 = 0.0;
        var_dv__blk293_rdn5 = 0.0;
        var_dv__blk293_rdb0 = 0.0;
        var_dv__blk293_rdb1 = 0.0;

        let (assign4440_e4525, assign4440_e4525_d_n0, assign4440_e4525_d_n1, assign4440_e4525_d_n2, assign4440_e4525_d_n3, assign4440_e4525_d_n4, assign4440_e4525_d_n5, assign4440_e4525_d_b0, assign4440_e4525_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4440_e4516: f64 = (var_dv__blk293 * var_dv__blk293);
        let assign4440_e4519: f64 = (4.0 * p.p75);
        let assign4440_e4521: f64 = (assign4440_e4519 * p.p75);
        let assign4440_e4522: f64 = (assign4440_e4516 + assign4440_e4521);
        let assign4440_e4523: f64 = (assign4440_e4522).sqrt();
        (assign4440_e4523, (((var_dv__blk293_dn0 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn0)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_dn1 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn1)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_dn2 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn2)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_dn3 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn3)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_dn4 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn4)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_dn5 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_dn5)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_db0 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_db0)) / (2.0 * assign4440_e4523)), (((var_dv__blk293_db1 * var_dv__blk293) + (var_dv__blk293 * var_dv__blk293_db1)) / (2.0 * assign4440_e4523)),)
    } else {
        (var_mv__blk294, var_mv__blk294_dn0, var_mv__blk294_dn1, var_mv__blk294_dn2, var_mv__blk294_dn3, var_mv__blk294_dn4, var_mv__blk294_dn5, var_mv__blk294_db0, var_mv__blk294_db1,)
    }
};
        var_mv__blk294 = assign4440_e4525;
        var_mv__blk294_dn0 = assign4440_e4525_d_n0;
        var_mv__blk294_dn1 = assign4440_e4525_d_n1;
        var_mv__blk294_dn2 = assign4440_e4525_d_n2;
        var_mv__blk294_dn3 = assign4440_e4525_d_n3;
        var_mv__blk294_dn4 = assign4440_e4525_d_n4;
        var_mv__blk294_dn5 = assign4440_e4525_d_n5;
        var_mv__blk294_db0 = assign4440_e4525_d_b0;
        var_mv__blk294_db1 = assign4440_e4525_d_b1;
        var_mv__blk294_rv = 0.0;
        var_mv__blk294_rdn0 = 0.0;
        var_mv__blk294_rdn1 = 0.0;
        var_mv__blk294_rdn2 = 0.0;
        var_mv__blk294_rdn3 = 0.0;
        var_mv__blk294_rdn4 = 0.0;
        var_mv__blk294_rdn5 = 0.0;
        var_mv__blk294_rdb0 = 0.0;
        var_mv__blk294_rdb1 = 0.0;

        let (assign4450_e4540, assign4450_e4540_d_n0, assign4450_e4540_d_n1, assign4450_e4540_d_n2, assign4450_e4540_d_n3, assign4450_e4540_d_n4, assign4450_e4540_d_n5, assign4450_e4540_d_b0, assign4450_e4540_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4450_e4535: f64 = (var_dv__blk293 - var_mv__blk294);
        let assign4450_e4536: f64 = (0.5 * assign4450_e4535);
        let assign4450_e4538: f64 = (assign4450_e4536 - var_dv0__blk286);
        (assign4450_e4538, ((0.5 * (var_dv__blk293_dn0 - var_mv__blk294_dn0)) - var_dv0__blk286_dn0), ((0.5 * (var_dv__blk293_dn1 - var_mv__blk294_dn1)) - var_dv0__blk286_dn1), ((0.5 * (var_dv__blk293_dn2 - var_mv__blk294_dn2)) - var_dv0__blk286_dn2), ((0.5 * (var_dv__blk293_dn3 - var_mv__blk294_dn3)) - var_dv0__blk286_dn3), ((0.5 * (var_dv__blk293_dn4 - var_mv__blk294_dn4)) - var_dv0__blk286_dn4), ((0.5 * (var_dv__blk293_dn5 - var_mv__blk294_dn5)) - var_dv0__blk286_dn5), ((0.5 * (var_dv__blk293_db0 - var_mv__blk294_db0)) - var_dv0__blk286_db0), ((0.5 * (var_dv__blk293_db1 - var_mv__blk294_db1)) - var_dv0__blk286_db1),)
    } else {
        (var_vl__blk295, var_vl__blk295_dn0, var_vl__blk295_dn1, var_vl__blk295_dn2, var_vl__blk295_dn3, var_vl__blk295_dn4, var_vl__blk295_dn5, var_vl__blk295_db0, var_vl__blk295_db1,)
    }
};
        var_vl__blk295 = assign4450_e4540;
        var_vl__blk295_dn0 = assign4450_e4540_d_n0;
        var_vl__blk295_dn1 = assign4450_e4540_d_n1;
        var_vl__blk295_dn2 = assign4450_e4540_d_n2;
        var_vl__blk295_dn3 = assign4450_e4540_d_n3;
        var_vl__blk295_dn4 = assign4450_e4540_d_n4;
        var_vl__blk295_dn5 = assign4450_e4540_d_n5;
        var_vl__blk295_db0 = assign4450_e4540_d_b0;
        var_vl__blk295_db1 = assign4450_e4540_d_b1;
        var_vl__blk295_rv = 0.0;
        var_vl__blk295_rdn0 = 0.0;
        var_vl__blk295_rdn1 = 0.0;
        var_vl__blk295_rdn2 = 0.0;
        var_vl__blk295_rdn3 = 0.0;
        var_vl__blk295_rdn4 = 0.0;
        var_vl__blk295_rdn5 = 0.0;
        var_vl__blk295_rdb0 = 0.0;
        var_vl__blk295_rdb1 = 0.0;

        let (assign4460_e4564, assign4460_e4564_d_n0, assign4460_e4564_d_n1, assign4460_e4564_d_n2, assign4460_e4564_d_n3, assign4460_e4564_d_n4, assign4460_e4564_d_n5, assign4460_e4564_d_b0, assign4460_e4564_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4460_e4548: f64 = (-var_pa_t);
        let assign4460_e4552: f64 = (var_vl__blk295 / var_pa_t);
        let assign4460_e4553: f64 = (1.0 - assign4460_e4552);
        let assign4460_e4556: f64 = (1.0 - p.p74);
        let assign4460_e4557: f64 = (assign4460_e4553).powf(assign4460_e4556);
        let assign4460_e4558: f64 = (assign4460_e4548 * assign4460_e4557);
        let assign4460_e4561: f64 = (1.0 - p.p74);
        let assign4460_e4562: f64 = (assign4460_e4558 / assign4460_e4561);
        (assign4460_e4562, ((((-var_pa_t_dn0) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn0 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn0)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn0 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn0)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_dn1) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn1 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn1)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn1 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn1)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_dn2) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn2 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn2)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn2 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn2)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_dn3) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn3 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn3)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn3 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn3)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_dn4) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn4 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn4)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn4 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn4)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_dn5) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_dn5 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn5)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_dn5 * var_pa_t) - (var_vl__blk295 * var_pa_t_dn5)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_db0) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_db0 * var_pa_t) - (var_vl__blk295 * var_pa_t_db0)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_db0 * var_pa_t) - (var_vl__blk295 * var_pa_t_db0)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561), ((((-var_pa_t_db1) * assign4460_e4557) + (assign4460_e4548 * if 0.0 == 0.0 && ((assign4460_e4556) as f64).is_finite() && ((assign4460_e4556) as f64).fract() == 0.0 { if assign4460_e4556 == 0.0 { 0.0 } else { (assign4460_e4556 * ((assign4460_e4553).powf(assign4460_e4556 - 1.0) * (-(((var_vl__blk295_db1 * var_pa_t) - (var_vl__blk295 * var_pa_t_db1)) / (var_pa_t * var_pa_t))))) } } else { (assign4460_e4557 * (assign4460_e4556 * ((-(((var_vl__blk295_db1 * var_pa_t) - (var_vl__blk295 * var_pa_t_db1)) / (var_pa_t * var_pa_t))) / assign4460_e4553))) })) / assign4460_e4561),)
    } else {
        (var_qlo__blk289, var_qlo__blk289_dn0, var_qlo__blk289_dn1, var_qlo__blk289_dn2, var_qlo__blk289_dn3, var_qlo__blk289_dn4, var_qlo__blk289_dn5, var_qlo__blk289_db0, var_qlo__blk289_db1,)
    }
};
        var_qlo__blk289 = assign4460_e4564;
        var_qlo__blk289_dn0 = assign4460_e4564_d_n0;
        var_qlo__blk289_dn1 = assign4460_e4564_d_n1;
        var_qlo__blk289_dn2 = assign4460_e4564_d_n2;
        var_qlo__blk289_dn3 = assign4460_e4564_d_n3;
        var_qlo__blk289_dn4 = assign4460_e4564_d_n4;
        var_qlo__blk289_dn5 = assign4460_e4564_d_n5;
        var_qlo__blk289_db0 = assign4460_e4564_d_b0;
        var_qlo__blk289_db1 = assign4460_e4564_d_b1;
        var_qlo__blk289_rv = 0.0;
        var_qlo__blk289_rdn0 = 0.0;
        var_qlo__blk289_rdn1 = 0.0;
        var_qlo__blk289_rdn2 = 0.0;
        var_qlo__blk289_rdn3 = 0.0;
        var_qlo__blk289_rdn4 = 0.0;
        var_qlo__blk289_rdn5 = 0.0;
        var_qlo__blk289_rdb0 = 0.0;
        var_qlo__blk289_rdb1 = 0.0;

        let (assign4470_e4604, assign4470_e4604_d_n0, assign4470_e4604_d_n1, assign4470_e4604_d_n2, assign4470_e4604_d_n3, assign4470_e4604_d_n4, assign4470_e4604_d_n5, assign4470_e4604_d_b0, assign4470_e4604_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard285 != 0.0)) && (var_guard296 == 0.0)) {
        let assign4470_e4574: f64 = (1.0 - p.p68);
        let assign4470_e4576: f64 = (-p.p74);
        let assign4470_e4577: f64 = (assign4470_e4574).powf(assign4470_e4576);
        let assign4470_e4580: f64 = (var_vcl - var_vl__blk295);
        let assign4470_e4582: f64 = (assign4470_e4580 + var_vl0__blk292);
        let assign4470_e4583: f64 = (assign4470_e4577 * assign4470_e4582);
        let assign4470_e4587: f64 = (0.5 * p.p74);
        let assign4470_e4590: f64 = (var_vcl - var_vl__blk295);
        let assign4470_e4592: f64 = (assign4470_e4590 + var_vl0__blk292);
        let assign4470_e4593: f64 = (assign4470_e4587 * assign4470_e4592);
        let assign4470_e4597: f64 = (1.0 - p.p68);
        let assign4470_e4598: f64 = (var_pa_t * assign4470_e4597);
        let assign4470_e4599: f64 = (assign4470_e4593 / assign4470_e4598);
        let assign4470_e4600: f64 = (1.0 + assign4470_e4599);
        let assign4470_e4601: f64 = (assign4470_e4583 * assign4470_e4600);
        let assign4470_e4602: f64 = (var_qlo__blk289 + assign4470_e4601);
        (assign4470_e4602, (var_qlo__blk289_dn0 + (((assign4470_e4577 * ((var_vcl_dn0 - var_vl__blk295_dn0) + var_vl0__blk292_dn0)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn0 - var_vl__blk295_dn0) + var_vl0__blk292_dn0)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn0 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_dn1 + (((assign4470_e4577 * ((var_vcl_dn1 - var_vl__blk295_dn1) + var_vl0__blk292_dn1)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn1 - var_vl__blk295_dn1) + var_vl0__blk292_dn1)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn1 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_dn2 + (((assign4470_e4577 * ((var_vcl_dn2 - var_vl__blk295_dn2) + var_vl0__blk292_dn2)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn2 - var_vl__blk295_dn2) + var_vl0__blk292_dn2)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn2 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_dn3 + (((assign4470_e4577 * ((var_vcl_dn3 - var_vl__blk295_dn3) + var_vl0__blk292_dn3)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn3 - var_vl__blk295_dn3) + var_vl0__blk292_dn3)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn3 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_dn4 + (((assign4470_e4577 * ((var_vcl_dn4 - var_vl__blk295_dn4) + var_vl0__blk292_dn4)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn4 - var_vl__blk295_dn4) + var_vl0__blk292_dn4)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn4 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_dn5 + (((assign4470_e4577 * ((var_vcl_dn5 - var_vl__blk295_dn5) + var_vl0__blk292_dn5)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_dn5 - var_vl__blk295_dn5) + var_vl0__blk292_dn5)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_dn5 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_db0 + (((assign4470_e4577 * ((var_vcl_db0 - var_vl__blk295_db0) + var_vl0__blk292_db0)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_db0 - var_vl__blk295_db0) + var_vl0__blk292_db0)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_db0 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))), (var_qlo__blk289_db1 + (((assign4470_e4577 * ((var_vcl_db1 - var_vl__blk295_db1) + var_vl0__blk292_db1)) * assign4470_e4600) + (assign4470_e4583 * ((((assign4470_e4587 * ((var_vcl_db1 - var_vl__blk295_db1) + var_vl0__blk292_db1)) * assign4470_e4598) - (assign4470_e4593 * (var_pa_t_db1 * assign4470_e4597))) / (assign4470_e4598 * assign4470_e4598))))),)
    } else {
        (var_arga__blk283, var_arga__blk283_dn0, var_arga__blk283_dn1, var_arga__blk283_dn2, var_arga__blk283_dn3, var_arga__blk283_dn4, var_arga__blk283_dn5, var_arga__blk283_db0, var_arga__blk283_db1,)
    }
};
        var_arga__blk283 = assign4470_e4604;
        var_arga__blk283_dn0 = assign4470_e4604_d_n0;
        var_arga__blk283_dn1 = assign4470_e4604_d_n1;
        var_arga__blk283_dn2 = assign4470_e4604_d_n2;
        var_arga__blk283_dn3 = assign4470_e4604_d_n3;
        var_arga__blk283_dn4 = assign4470_e4604_d_n4;
        var_arga__blk283_dn5 = assign4470_e4604_d_n5;
        var_arga__blk283_db0 = assign4470_e4604_d_b0;
        var_arga__blk283_db1 = assign4470_e4604_d_b1;
        var_arga__blk283_rv = 0.0;
        var_arga__blk283_rdn0 = 0.0;
        var_arga__blk283_rdn1 = 0.0;
        var_arga__blk283_rdn2 = 0.0;
        var_arga__blk283_rdn3 = 0.0;
        var_arga__blk283_rdn4 = 0.0;
        var_arga__blk283_rdn5 = 0.0;
        var_arga__blk283_rdb0 = 0.0;
        var_arga__blk283_rdb1 = 0.0;

        let (assign4480_e4611, assign4480_e4611_d_n0, assign4480_e4611_d_n1, assign4480_e4611_d_n2, assign4480_e4611_d_n3, assign4480_e4611_d_n4, assign4480_e4611_d_n5, assign4480_e4611_d_b0, assign4480_e4611_d_b1,) = {
    if ((var_guard280 != 0.0) && (var_guard285 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arga__blk283, var_arga__blk283_dn0, var_arga__blk283_dn1, var_arga__blk283_dn2, var_arga__blk283_dn3, var_arga__blk283_dn4, var_arga__blk283_dn5, var_arga__blk283_db0, var_arga__blk283_db1,)
    }
};
        var_arga__blk283 = assign4480_e4611;
        var_arga__blk283_dn0 = assign4480_e4611_d_n0;
        var_arga__blk283_dn1 = assign4480_e4611_d_n1;
        var_arga__blk283_dn2 = assign4480_e4611_d_n2;
        var_arga__blk283_dn3 = assign4480_e4611_d_n3;
        var_arga__blk283_dn4 = assign4480_e4611_d_n4;
        var_arga__blk283_dn5 = assign4480_e4611_d_n5;
        var_arga__blk283_db0 = assign4480_e4611_d_b0;
        var_arga__blk283_db1 = assign4480_e4611_d_b1;
        var_arga__blk283_rv = 0.0;
        var_arga__blk283_rdn0 = 0.0;
        var_arga__blk283_rdn1 = 0.0;
        var_arga__blk283_rdn2 = 0.0;
        var_arga__blk283_rdn3 = 0.0;
        var_arga__blk283_rdn4 = 0.0;
        var_arga__blk283_rdn5 = 0.0;
        var_arga__blk283_rdb0 = 0.0;
        var_arga__blk283_rdb1 = 0.0;

        let assign4490_e4614: f64 = if var_pcjp__blk282 > 0.0 { 1.0 } else { 0.0 };
        var_guard298 = assign4490_e4614;
        var_guard298_dn0 = 0.0;
        var_guard298_dn1 = 0.0;
        var_guard298_dn2 = 0.0;
        var_guard298_dn3 = 0.0;
        var_guard298_dn4 = 0.0;
        var_guard298_dn5 = 0.0;
        var_guard298_db0 = 0.0;
        var_guard298_db1 = 0.0;
        var_guard298_rv = 0.0;
        var_guard298_rdn0 = 0.0;
        var_guard298_rdn1 = 0.0;
        var_guard298_rdn2 = 0.0;
        var_guard298_rdn3 = 0.0;
        var_guard298_rdn4 = 0.0;
        var_guard298_rdn5 = 0.0;
        var_guard298_rdb0 = 0.0;
        var_guard298_rdb1 = 0.0;

        let (assign4500_e4623, assign4500_e4623_d_n0, assign4500_e4623_d_n1, assign4500_e4623_d_n2, assign4500_e4623_d_n3, assign4500_e4623_d_n4, assign4500_e4623_d_n5, assign4500_e4623_d_b0, assign4500_e4623_d_b1,) = {
    if ((var_guard280 != 0.0) && (var_guard298 != 0.0)) {
        let assign4500_e4619: f64 = (-var_pp_t);
        let assign4500_e4621: f64 = (assign4500_e4619 * p.p68);
        (assign4500_e4621, ((-var_pp_t_dn0) * p.p68), ((-var_pp_t_dn1) * p.p68), ((-var_pp_t_dn2) * p.p68), ((-var_pp_t_dn3) * p.p68), ((-var_pp_t_dn4) * p.p68), ((-var_pp_t_dn5) * p.p68), ((-var_pp_t_db0) * p.p68), ((-var_pp_t_db1) * p.p68),)
    } else {
        (var_dv0__blk299, var_dv0__blk299_dn0, var_dv0__blk299_dn1, var_dv0__blk299_dn2, var_dv0__blk299_dn3, var_dv0__blk299_dn4, var_dv0__blk299_dn5, var_dv0__blk299_db0, var_dv0__blk299_db1,)
    }
};
        var_dv0__blk299 = assign4500_e4623;
        var_dv0__blk299_dn0 = assign4500_e4623_d_n0;
        var_dv0__blk299_dn1 = assign4500_e4623_d_n1;
        var_dv0__blk299_dn2 = assign4500_e4623_d_n2;
        var_dv0__blk299_dn3 = assign4500_e4623_d_n3;
        var_dv0__blk299_dn4 = assign4500_e4623_d_n4;
        var_dv0__blk299_dn5 = assign4500_e4623_d_n5;
        var_dv0__blk299_db0 = assign4500_e4623_d_b0;
        var_dv0__blk299_db1 = assign4500_e4623_d_b1;
        var_dv0__blk299_rv = 0.0;
        var_dv0__blk299_rdn0 = 0.0;
        var_dv0__blk299_rdn1 = 0.0;
        var_dv0__blk299_rdn2 = 0.0;
        var_dv0__blk299_rdn3 = 0.0;
        var_dv0__blk299_rdn4 = 0.0;
        var_dv0__blk299_rdn5 = 0.0;
        var_dv0__blk299_rdb0 = 0.0;
        var_dv0__blk299_rdb1 = 0.0;

        *var_arga__blk283_slot = var_arga__blk283;
        *var_arga__blk283_db0_slot = var_arga__blk283_db0;
        *var_arga__blk283_db1_slot = var_arga__blk283_db1;
        *var_arga__blk283_dn0_slot = var_arga__blk283_dn0;
        *var_arga__blk283_dn1_slot = var_arga__blk283_dn1;
        *var_arga__blk283_dn2_slot = var_arga__blk283_dn2;
        *var_arga__blk283_dn3_slot = var_arga__blk283_dn3;
        *var_arga__blk283_dn4_slot = var_arga__blk283_dn4;
        *var_arga__blk283_dn5_slot = var_arga__blk283_dn5;
        *var_arga__blk283_rdb0_slot = var_arga__blk283_rdb0;
        *var_arga__blk283_rdb1_slot = var_arga__blk283_rdb1;
        *var_arga__blk283_rdn0_slot = var_arga__blk283_rdn0;
        *var_arga__blk283_rdn1_slot = var_arga__blk283_rdn1;
        *var_arga__blk283_rdn2_slot = var_arga__blk283_rdn2;
        *var_arga__blk283_rdn3_slot = var_arga__blk283_rdn3;
        *var_arga__blk283_rdn4_slot = var_arga__blk283_rdn4;
        *var_arga__blk283_rdn5_slot = var_arga__blk283_rdn5;
        *var_arga__blk283_rv_slot = var_arga__blk283_rv;
        *var_dv0__blk299_slot = var_dv0__blk299;
        *var_dv0__blk299_db0_slot = var_dv0__blk299_db0;
        *var_dv0__blk299_db1_slot = var_dv0__blk299_db1;
        *var_dv0__blk299_dn0_slot = var_dv0__blk299_dn0;
        *var_dv0__blk299_dn1_slot = var_dv0__blk299_dn1;
        *var_dv0__blk299_dn2_slot = var_dv0__blk299_dn2;
        *var_dv0__blk299_dn3_slot = var_dv0__blk299_dn3;
        *var_dv0__blk299_dn4_slot = var_dv0__blk299_dn4;
        *var_dv0__blk299_dn5_slot = var_dv0__blk299_dn5;
        *var_dv0__blk299_rdb0_slot = var_dv0__blk299_rdb0;
        *var_dv0__blk299_rdb1_slot = var_dv0__blk299_rdb1;
        *var_dv0__blk299_rdn0_slot = var_dv0__blk299_rdn0;
        *var_dv0__blk299_rdn1_slot = var_dv0__blk299_rdn1;
        *var_dv0__blk299_rdn2_slot = var_dv0__blk299_rdn2;
        *var_dv0__blk299_rdn3_slot = var_dv0__blk299_rdn3;
        *var_dv0__blk299_rdn4_slot = var_dv0__blk299_rdn4;
        *var_dv0__blk299_rdn5_slot = var_dv0__blk299_rdn5;
        *var_dv0__blk299_rv_slot = var_dv0__blk299_rv;
        *var_dv__blk293_slot = var_dv__blk293;
        *var_dv__blk293_db0_slot = var_dv__blk293_db0;
        *var_dv__blk293_db1_slot = var_dv__blk293_db1;
        *var_dv__blk293_dn0_slot = var_dv__blk293_dn0;
        *var_dv__blk293_dn1_slot = var_dv__blk293_dn1;
        *var_dv__blk293_dn2_slot = var_dv__blk293_dn2;
        *var_dv__blk293_dn3_slot = var_dv__blk293_dn3;
        *var_dv__blk293_dn4_slot = var_dv__blk293_dn4;
        *var_dv__blk293_dn5_slot = var_dv__blk293_dn5;
        *var_dv__blk293_rdb0_slot = var_dv__blk293_rdb0;
        *var_dv__blk293_rdb1_slot = var_dv__blk293_rdb1;
        *var_dv__blk293_rdn0_slot = var_dv__blk293_rdn0;
        *var_dv__blk293_rdn1_slot = var_dv__blk293_rdn1;
        *var_dv__blk293_rdn2_slot = var_dv__blk293_rdn2;
        *var_dv__blk293_rdn3_slot = var_dv__blk293_rdn3;
        *var_dv__blk293_rdn4_slot = var_dv__blk293_rdn4;
        *var_dv__blk293_rdn5_slot = var_dv__blk293_rdn5;
        *var_dv__blk293_rv_slot = var_dv__blk293_rv;
        *var_guard297_slot = var_guard297;
        *var_guard297_db0_slot = var_guard297_db0;
        *var_guard297_db1_slot = var_guard297_db1;
        *var_guard297_dn0_slot = var_guard297_dn0;
        *var_guard297_dn1_slot = var_guard297_dn1;
        *var_guard297_dn2_slot = var_guard297_dn2;
        *var_guard297_dn3_slot = var_guard297_dn3;
        *var_guard297_dn4_slot = var_guard297_dn4;
        *var_guard297_dn5_slot = var_guard297_dn5;
        *var_guard297_rdb0_slot = var_guard297_rdb0;
        *var_guard297_rdb1_slot = var_guard297_rdb1;
        *var_guard297_rdn0_slot = var_guard297_rdn0;
        *var_guard297_rdn1_slot = var_guard297_rdn1;
        *var_guard297_rdn2_slot = var_guard297_rdn2;
        *var_guard297_rdn3_slot = var_guard297_rdn3;
        *var_guard297_rdn4_slot = var_guard297_rdn4;
        *var_guard297_rdn5_slot = var_guard297_rdn5;
        *var_guard297_rv_slot = var_guard297_rv;
        *var_guard298_slot = var_guard298;
        *var_guard298_db0_slot = var_guard298_db0;
        *var_guard298_db1_slot = var_guard298_db1;
        *var_guard298_dn0_slot = var_guard298_dn0;
        *var_guard298_dn1_slot = var_guard298_dn1;
        *var_guard298_dn2_slot = var_guard298_dn2;
        *var_guard298_dn3_slot = var_guard298_dn3;
        *var_guard298_dn4_slot = var_guard298_dn4;
        *var_guard298_dn5_slot = var_guard298_dn5;
        *var_guard298_rdb0_slot = var_guard298_rdb0;
        *var_guard298_rdb1_slot = var_guard298_rdb1;
        *var_guard298_rdn0_slot = var_guard298_rdn0;
        *var_guard298_rdn1_slot = var_guard298_rdn1;
        *var_guard298_rdn2_slot = var_guard298_rdn2;
        *var_guard298_rdn3_slot = var_guard298_rdn3;
        *var_guard298_rdn4_slot = var_guard298_rdn4;
        *var_guard298_rdn5_slot = var_guard298_rdn5;
        *var_guard298_rv_slot = var_guard298_rv;
        *var_mv0__blk291_slot = var_mv0__blk291;
        *var_mv0__blk291_db0_slot = var_mv0__blk291_db0;
        *var_mv0__blk291_db1_slot = var_mv0__blk291_db1;
        *var_mv0__blk291_dn0_slot = var_mv0__blk291_dn0;
        *var_mv0__blk291_dn1_slot = var_mv0__blk291_dn1;
        *var_mv0__blk291_dn2_slot = var_mv0__blk291_dn2;
        *var_mv0__blk291_dn3_slot = var_mv0__blk291_dn3;
        *var_mv0__blk291_dn4_slot = var_mv0__blk291_dn4;
        *var_mv0__blk291_dn5_slot = var_mv0__blk291_dn5;
        *var_mv0__blk291_rdb0_slot = var_mv0__blk291_rdb0;
        *var_mv0__blk291_rdb1_slot = var_mv0__blk291_rdb1;
        *var_mv0__blk291_rdn0_slot = var_mv0__blk291_rdn0;
        *var_mv0__blk291_rdn1_slot = var_mv0__blk291_rdn1;
        *var_mv0__blk291_rdn2_slot = var_mv0__blk291_rdn2;
        *var_mv0__blk291_rdn3_slot = var_mv0__blk291_rdn3;
        *var_mv0__blk291_rdn4_slot = var_mv0__blk291_rdn4;
        *var_mv0__blk291_rdn5_slot = var_mv0__blk291_rdn5;
        *var_mv0__blk291_rv_slot = var_mv0__blk291_rv;
        *var_mv__blk294_slot = var_mv__blk294;
        *var_mv__blk294_db0_slot = var_mv__blk294_db0;
        *var_mv__blk294_db1_slot = var_mv__blk294_db1;
        *var_mv__blk294_dn0_slot = var_mv__blk294_dn0;
        *var_mv__blk294_dn1_slot = var_mv__blk294_dn1;
        *var_mv__blk294_dn2_slot = var_mv__blk294_dn2;
        *var_mv__blk294_dn3_slot = var_mv__blk294_dn3;
        *var_mv__blk294_dn4_slot = var_mv__blk294_dn4;
        *var_mv__blk294_dn5_slot = var_mv__blk294_dn5;
        *var_mv__blk294_rdb0_slot = var_mv__blk294_rdb0;
        *var_mv__blk294_rdb1_slot = var_mv__blk294_rdb1;
        *var_mv__blk294_rdn0_slot = var_mv__blk294_rdn0;
        *var_mv__blk294_rdn1_slot = var_mv__blk294_rdn1;
        *var_mv__blk294_rdn2_slot = var_mv__blk294_rdn2;
        *var_mv__blk294_rdn3_slot = var_mv__blk294_rdn3;
        *var_mv__blk294_rdn4_slot = var_mv__blk294_rdn4;
        *var_mv__blk294_rdn5_slot = var_mv__blk294_rdn5;
        *var_mv__blk294_rv_slot = var_mv__blk294_rv;
        *var_pwq__blk288_slot = var_pwq__blk288;
        *var_pwq__blk288_db0_slot = var_pwq__blk288_db0;
        *var_pwq__blk288_db1_slot = var_pwq__blk288_db1;
        *var_pwq__blk288_dn0_slot = var_pwq__blk288_dn0;
        *var_pwq__blk288_dn1_slot = var_pwq__blk288_dn1;
        *var_pwq__blk288_dn2_slot = var_pwq__blk288_dn2;
        *var_pwq__blk288_dn3_slot = var_pwq__blk288_dn3;
        *var_pwq__blk288_dn4_slot = var_pwq__blk288_dn4;
        *var_pwq__blk288_dn5_slot = var_pwq__blk288_dn5;
        *var_pwq__blk288_rdb0_slot = var_pwq__blk288_rdb0;
        *var_pwq__blk288_rdb1_slot = var_pwq__blk288_rdb1;
        *var_pwq__blk288_rdn0_slot = var_pwq__blk288_rdn0;
        *var_pwq__blk288_rdn1_slot = var_pwq__blk288_rdn1;
        *var_pwq__blk288_rdn2_slot = var_pwq__blk288_rdn2;
        *var_pwq__blk288_rdn3_slot = var_pwq__blk288_rdn3;
        *var_pwq__blk288_rdn4_slot = var_pwq__blk288_rdn4;
        *var_pwq__blk288_rdn5_slot = var_pwq__blk288_rdn5;
        *var_pwq__blk288_rv_slot = var_pwq__blk288_rv;
        *var_qhi__blk290_slot = var_qhi__blk290;
        *var_qhi__blk290_db0_slot = var_qhi__blk290_db0;
        *var_qhi__blk290_db1_slot = var_qhi__blk290_db1;
        *var_qhi__blk290_dn0_slot = var_qhi__blk290_dn0;
        *var_qhi__blk290_dn1_slot = var_qhi__blk290_dn1;
        *var_qhi__blk290_dn2_slot = var_qhi__blk290_dn2;
        *var_qhi__blk290_dn3_slot = var_qhi__blk290_dn3;
        *var_qhi__blk290_dn4_slot = var_qhi__blk290_dn4;
        *var_qhi__blk290_dn5_slot = var_qhi__blk290_dn5;
        *var_qhi__blk290_rdb0_slot = var_qhi__blk290_rdb0;
        *var_qhi__blk290_rdb1_slot = var_qhi__blk290_rdb1;
        *var_qhi__blk290_rdn0_slot = var_qhi__blk290_rdn0;
        *var_qhi__blk290_rdn1_slot = var_qhi__blk290_rdn1;
        *var_qhi__blk290_rdn2_slot = var_qhi__blk290_rdn2;
        *var_qhi__blk290_rdn3_slot = var_qhi__blk290_rdn3;
        *var_qhi__blk290_rdn4_slot = var_qhi__blk290_rdn4;
        *var_qhi__blk290_rdn5_slot = var_qhi__blk290_rdn5;
        *var_qhi__blk290_rv_slot = var_qhi__blk290_rv;
        *var_qlo__blk289_slot = var_qlo__blk289;
        *var_qlo__blk289_db0_slot = var_qlo__blk289_db0;
        *var_qlo__blk289_db1_slot = var_qlo__blk289_db1;
        *var_qlo__blk289_dn0_slot = var_qlo__blk289_dn0;
        *var_qlo__blk289_dn1_slot = var_qlo__blk289_dn1;
        *var_qlo__blk289_dn2_slot = var_qlo__blk289_dn2;
        *var_qlo__blk289_dn3_slot = var_qlo__blk289_dn3;
        *var_qlo__blk289_dn4_slot = var_qlo__blk289_dn4;
        *var_qlo__blk289_dn5_slot = var_qlo__blk289_dn5;
        *var_qlo__blk289_rdb0_slot = var_qlo__blk289_rdb0;
        *var_qlo__blk289_rdb1_slot = var_qlo__blk289_rdb1;
        *var_qlo__blk289_rdn0_slot = var_qlo__blk289_rdn0;
        *var_qlo__blk289_rdn1_slot = var_qlo__blk289_rdn1;
        *var_qlo__blk289_rdn2_slot = var_qlo__blk289_rdn2;
        *var_qlo__blk289_rdn3_slot = var_qlo__blk289_rdn3;
        *var_qlo__blk289_rdn4_slot = var_qlo__blk289_rdn4;
        *var_qlo__blk289_rdn5_slot = var_qlo__blk289_rdn5;
        *var_qlo__blk289_rv_slot = var_qlo__blk289_rv;
        *var_vl0__blk292_slot = var_vl0__blk292;
        *var_vl0__blk292_db0_slot = var_vl0__blk292_db0;
        *var_vl0__blk292_db1_slot = var_vl0__blk292_db1;
        *var_vl0__blk292_dn0_slot = var_vl0__blk292_dn0;
        *var_vl0__blk292_dn1_slot = var_vl0__blk292_dn1;
        *var_vl0__blk292_dn2_slot = var_vl0__blk292_dn2;
        *var_vl0__blk292_dn3_slot = var_vl0__blk292_dn3;
        *var_vl0__blk292_dn4_slot = var_vl0__blk292_dn4;
        *var_vl0__blk292_dn5_slot = var_vl0__blk292_dn5;
        *var_vl0__blk292_rdb0_slot = var_vl0__blk292_rdb0;
        *var_vl0__blk292_rdb1_slot = var_vl0__blk292_rdb1;
        *var_vl0__blk292_rdn0_slot = var_vl0__blk292_rdn0;
        *var_vl0__blk292_rdn1_slot = var_vl0__blk292_rdn1;
        *var_vl0__blk292_rdn2_slot = var_vl0__blk292_rdn2;
        *var_vl0__blk292_rdn3_slot = var_vl0__blk292_rdn3;
        *var_vl0__blk292_rdn4_slot = var_vl0__blk292_rdn4;
        *var_vl0__blk292_rdn5_slot = var_vl0__blk292_rdn5;
        *var_vl0__blk292_rv_slot = var_vl0__blk292_rv;
        *var_vl__blk295_slot = var_vl__blk295;
        *var_vl__blk295_db0_slot = var_vl__blk295_db0;
        *var_vl__blk295_db1_slot = var_vl__blk295_db1;
        *var_vl__blk295_dn0_slot = var_vl__blk295_dn0;
        *var_vl__blk295_dn1_slot = var_vl__blk295_dn1;
        *var_vl__blk295_dn2_slot = var_vl__blk295_dn2;
        *var_vl__blk295_dn3_slot = var_vl__blk295_dn3;
        *var_vl__blk295_dn4_slot = var_vl__blk295_dn4;
        *var_vl__blk295_dn5_slot = var_vl__blk295_dn5;
        *var_vl__blk295_rdb0_slot = var_vl__blk295_rdb0;
        *var_vl__blk295_rdb1_slot = var_vl__blk295_rdb1;
        *var_vl__blk295_rdn0_slot = var_vl__blk295_rdn0;
        *var_vl__blk295_rdn1_slot = var_vl__blk295_rdn1;
        *var_vl__blk295_rdn2_slot = var_vl__blk295_rdn2;
        *var_vl__blk295_rdn3_slot = var_vl__blk295_rdn3;
        *var_vl__blk295_rdn4_slot = var_vl__blk295_rdn4;
        *var_vl__blk295_rdn5_slot = var_vl__blk295_rdn5;
        *var_vl__blk295_rv_slot = var_vl__blk295_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_dv0__blk299: f64,
        var_dv0__blk299_db0: f64,
        var_dv0__blk299_db1: f64,
        var_dv0__blk299_dn0: f64,
        var_dv0__blk299_dn1: f64,
        var_dv0__blk299_dn2: f64,
        var_dv0__blk299_dn3: f64,
        var_dv0__blk299_dn4: f64,
        var_dv0__blk299_dn5: f64,
        var_guard280: f64,
        var_guard298: f64,
        var_pp_t: f64,
        var_pp_t_db0: f64,
        var_pp_t_db1: f64,
        var_pp_t_dn0: f64,
        var_pp_t_dn1: f64,
        var_pp_t_dn2: f64,
        var_pp_t_dn3: f64,
        var_pp_t_dn4: f64,
        var_pp_t_dn5: f64,
        var_vcl: f64,
        var_vcl_db0: f64,
        var_vcl_db1: f64,
        var_vcl_dn0: f64,
        var_vcl_dn1: f64,
        var_vcl_dn2: f64,
        var_vcl_dn3: f64,
        var_vcl_dn4: f64,
        var_vcl_dn5: f64,
        var_argp__blk284_slot: &mut f64,
        var_argp__blk284_db0_slot: &mut f64,
        var_argp__blk284_db1_slot: &mut f64,
        var_argp__blk284_dn0_slot: &mut f64,
        var_argp__blk284_dn1_slot: &mut f64,
        var_argp__blk284_dn2_slot: &mut f64,
        var_argp__blk284_dn3_slot: &mut f64,
        var_argp__blk284_dn4_slot: &mut f64,
        var_argp__blk284_dn5_slot: &mut f64,
        var_argp__blk284_rdb0_slot: &mut f64,
        var_argp__blk284_rdb1_slot: &mut f64,
        var_argp__blk284_rdn0_slot: &mut f64,
        var_argp__blk284_rdn1_slot: &mut f64,
        var_argp__blk284_rdn2_slot: &mut f64,
        var_argp__blk284_rdn3_slot: &mut f64,
        var_argp__blk284_rdn4_slot: &mut f64,
        var_argp__blk284_rdn5_slot: &mut f64,
        var_argp__blk284_rv_slot: &mut f64,
        var_dv__blk306_slot: &mut f64,
        var_dv__blk306_db0_slot: &mut f64,
        var_dv__blk306_db1_slot: &mut f64,
        var_dv__blk306_dn0_slot: &mut f64,
        var_dv__blk306_dn1_slot: &mut f64,
        var_dv__blk306_dn2_slot: &mut f64,
        var_dv__blk306_dn3_slot: &mut f64,
        var_dv__blk306_dn4_slot: &mut f64,
        var_dv__blk306_dn5_slot: &mut f64,
        var_dv__blk306_rdb0_slot: &mut f64,
        var_dv__blk306_rdb1_slot: &mut f64,
        var_dv__blk306_rdn0_slot: &mut f64,
        var_dv__blk306_rdn1_slot: &mut f64,
        var_dv__blk306_rdn2_slot: &mut f64,
        var_dv__blk306_rdn3_slot: &mut f64,
        var_dv__blk306_rdn4_slot: &mut f64,
        var_dv__blk306_rdn5_slot: &mut f64,
        var_dv__blk306_rv_slot: &mut f64,
        var_dvh__blk300_slot: &mut f64,
        var_dvh__blk300_db0_slot: &mut f64,
        var_dvh__blk300_db1_slot: &mut f64,
        var_dvh__blk300_dn0_slot: &mut f64,
        var_dvh__blk300_dn1_slot: &mut f64,
        var_dvh__blk300_dn2_slot: &mut f64,
        var_dvh__blk300_dn3_slot: &mut f64,
        var_dvh__blk300_dn4_slot: &mut f64,
        var_dvh__blk300_dn5_slot: &mut f64,
        var_dvh__blk300_rdb0_slot: &mut f64,
        var_dvh__blk300_rdb1_slot: &mut f64,
        var_dvh__blk300_rdn0_slot: &mut f64,
        var_dvh__blk300_rdn1_slot: &mut f64,
        var_dvh__blk300_rdn2_slot: &mut f64,
        var_dvh__blk300_rdn3_slot: &mut f64,
        var_dvh__blk300_rdn4_slot: &mut f64,
        var_dvh__blk300_rdn5_slot: &mut f64,
        var_dvh__blk300_rv_slot: &mut f64,
        var_guard309_slot: &mut f64,
        var_guard309_db0_slot: &mut f64,
        var_guard309_db1_slot: &mut f64,
        var_guard309_dn0_slot: &mut f64,
        var_guard309_dn1_slot: &mut f64,
        var_guard309_dn2_slot: &mut f64,
        var_guard309_dn3_slot: &mut f64,
        var_guard309_dn4_slot: &mut f64,
        var_guard309_dn5_slot: &mut f64,
        var_guard309_rdb0_slot: &mut f64,
        var_guard309_rdb1_slot: &mut f64,
        var_guard309_rdn0_slot: &mut f64,
        var_guard309_rdn1_slot: &mut f64,
        var_guard309_rdn2_slot: &mut f64,
        var_guard309_rdn3_slot: &mut f64,
        var_guard309_rdn4_slot: &mut f64,
        var_guard309_rdn5_slot: &mut f64,
        var_guard309_rv_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard310_db0_slot: &mut f64,
        var_guard310_db1_slot: &mut f64,
        var_guard310_dn0_slot: &mut f64,
        var_guard310_dn1_slot: &mut f64,
        var_guard310_dn2_slot: &mut f64,
        var_guard310_dn3_slot: &mut f64,
        var_guard310_dn4_slot: &mut f64,
        var_guard310_dn5_slot: &mut f64,
        var_guard310_rdb0_slot: &mut f64,
        var_guard310_rdb1_slot: &mut f64,
        var_guard310_rdn0_slot: &mut f64,
        var_guard310_rdn1_slot: &mut f64,
        var_guard310_rdn2_slot: &mut f64,
        var_guard310_rdn3_slot: &mut f64,
        var_guard310_rdn4_slot: &mut f64,
        var_guard310_rdn5_slot: &mut f64,
        var_guard310_rv_slot: &mut f64,
        var_mv0__blk304_slot: &mut f64,
        var_mv0__blk304_db0_slot: &mut f64,
        var_mv0__blk304_db1_slot: &mut f64,
        var_mv0__blk304_dn0_slot: &mut f64,
        var_mv0__blk304_dn1_slot: &mut f64,
        var_mv0__blk304_dn2_slot: &mut f64,
        var_mv0__blk304_dn3_slot: &mut f64,
        var_mv0__blk304_dn4_slot: &mut f64,
        var_mv0__blk304_dn5_slot: &mut f64,
        var_mv0__blk304_rdb0_slot: &mut f64,
        var_mv0__blk304_rdb1_slot: &mut f64,
        var_mv0__blk304_rdn0_slot: &mut f64,
        var_mv0__blk304_rdn1_slot: &mut f64,
        var_mv0__blk304_rdn2_slot: &mut f64,
        var_mv0__blk304_rdn3_slot: &mut f64,
        var_mv0__blk304_rdn4_slot: &mut f64,
        var_mv0__blk304_rdn5_slot: &mut f64,
        var_mv0__blk304_rv_slot: &mut f64,
        var_mv__blk307_slot: &mut f64,
        var_mv__blk307_db0_slot: &mut f64,
        var_mv__blk307_db1_slot: &mut f64,
        var_mv__blk307_dn0_slot: &mut f64,
        var_mv__blk307_dn1_slot: &mut f64,
        var_mv__blk307_dn2_slot: &mut f64,
        var_mv__blk307_dn3_slot: &mut f64,
        var_mv__blk307_dn4_slot: &mut f64,
        var_mv__blk307_dn5_slot: &mut f64,
        var_mv__blk307_rdb0_slot: &mut f64,
        var_mv__blk307_rdb1_slot: &mut f64,
        var_mv__blk307_rdn0_slot: &mut f64,
        var_mv__blk307_rdn1_slot: &mut f64,
        var_mv__blk307_rdn2_slot: &mut f64,
        var_mv__blk307_rdn3_slot: &mut f64,
        var_mv__blk307_rdn4_slot: &mut f64,
        var_mv__blk307_rdn5_slot: &mut f64,
        var_mv__blk307_rv_slot: &mut f64,
        var_pwq__blk301_slot: &mut f64,
        var_pwq__blk301_db0_slot: &mut f64,
        var_pwq__blk301_db1_slot: &mut f64,
        var_pwq__blk301_dn0_slot: &mut f64,
        var_pwq__blk301_dn1_slot: &mut f64,
        var_pwq__blk301_dn2_slot: &mut f64,
        var_pwq__blk301_dn3_slot: &mut f64,
        var_pwq__blk301_dn4_slot: &mut f64,
        var_pwq__blk301_dn5_slot: &mut f64,
        var_pwq__blk301_rdb0_slot: &mut f64,
        var_pwq__blk301_rdb1_slot: &mut f64,
        var_pwq__blk301_rdn0_slot: &mut f64,
        var_pwq__blk301_rdn1_slot: &mut f64,
        var_pwq__blk301_rdn2_slot: &mut f64,
        var_pwq__blk301_rdn3_slot: &mut f64,
        var_pwq__blk301_rdn4_slot: &mut f64,
        var_pwq__blk301_rdn5_slot: &mut f64,
        var_pwq__blk301_rv_slot: &mut f64,
        var_qhi__blk303_slot: &mut f64,
        var_qhi__blk303_db0_slot: &mut f64,
        var_qhi__blk303_db1_slot: &mut f64,
        var_qhi__blk303_dn0_slot: &mut f64,
        var_qhi__blk303_dn1_slot: &mut f64,
        var_qhi__blk303_dn2_slot: &mut f64,
        var_qhi__blk303_dn3_slot: &mut f64,
        var_qhi__blk303_dn4_slot: &mut f64,
        var_qhi__blk303_dn5_slot: &mut f64,
        var_qhi__blk303_rdb0_slot: &mut f64,
        var_qhi__blk303_rdb1_slot: &mut f64,
        var_qhi__blk303_rdn0_slot: &mut f64,
        var_qhi__blk303_rdn1_slot: &mut f64,
        var_qhi__blk303_rdn2_slot: &mut f64,
        var_qhi__blk303_rdn3_slot: &mut f64,
        var_qhi__blk303_rdn4_slot: &mut f64,
        var_qhi__blk303_rdn5_slot: &mut f64,
        var_qhi__blk303_rv_slot: &mut f64,
        var_qlo__blk302_slot: &mut f64,
        var_qlo__blk302_db0_slot: &mut f64,
        var_qlo__blk302_db1_slot: &mut f64,
        var_qlo__blk302_dn0_slot: &mut f64,
        var_qlo__blk302_dn1_slot: &mut f64,
        var_qlo__blk302_dn2_slot: &mut f64,
        var_qlo__blk302_dn3_slot: &mut f64,
        var_qlo__blk302_dn4_slot: &mut f64,
        var_qlo__blk302_dn5_slot: &mut f64,
        var_qlo__blk302_rdb0_slot: &mut f64,
        var_qlo__blk302_rdb1_slot: &mut f64,
        var_qlo__blk302_rdn0_slot: &mut f64,
        var_qlo__blk302_rdn1_slot: &mut f64,
        var_qlo__blk302_rdn2_slot: &mut f64,
        var_qlo__blk302_rdn3_slot: &mut f64,
        var_qlo__blk302_rdn4_slot: &mut f64,
        var_qlo__blk302_rdn5_slot: &mut f64,
        var_qlo__blk302_rv_slot: &mut f64,
        var_vl0__blk305_slot: &mut f64,
        var_vl0__blk305_db0_slot: &mut f64,
        var_vl0__blk305_db1_slot: &mut f64,
        var_vl0__blk305_dn0_slot: &mut f64,
        var_vl0__blk305_dn1_slot: &mut f64,
        var_vl0__blk305_dn2_slot: &mut f64,
        var_vl0__blk305_dn3_slot: &mut f64,
        var_vl0__blk305_dn4_slot: &mut f64,
        var_vl0__blk305_dn5_slot: &mut f64,
        var_vl0__blk305_rdb0_slot: &mut f64,
        var_vl0__blk305_rdb1_slot: &mut f64,
        var_vl0__blk305_rdn0_slot: &mut f64,
        var_vl0__blk305_rdn1_slot: &mut f64,
        var_vl0__blk305_rdn2_slot: &mut f64,
        var_vl0__blk305_rdn3_slot: &mut f64,
        var_vl0__blk305_rdn4_slot: &mut f64,
        var_vl0__blk305_rdn5_slot: &mut f64,
        var_vl0__blk305_rv_slot: &mut f64,
        var_vl__blk308_slot: &mut f64,
        var_vl__blk308_db0_slot: &mut f64,
        var_vl__blk308_db1_slot: &mut f64,
        var_vl__blk308_dn0_slot: &mut f64,
        var_vl__blk308_dn1_slot: &mut f64,
        var_vl__blk308_dn2_slot: &mut f64,
        var_vl__blk308_dn3_slot: &mut f64,
        var_vl__blk308_dn4_slot: &mut f64,
        var_vl__blk308_dn5_slot: &mut f64,
        var_vl__blk308_rdb0_slot: &mut f64,
        var_vl__blk308_rdb1_slot: &mut f64,
        var_vl__blk308_rdn0_slot: &mut f64,
        var_vl__blk308_rdn1_slot: &mut f64,
        var_vl__blk308_rdn2_slot: &mut f64,
        var_vl__blk308_rdn3_slot: &mut f64,
        var_vl__blk308_rdn4_slot: &mut f64,
        var_vl__blk308_rdn5_slot: &mut f64,
        var_vl__blk308_rv_slot: &mut f64,
    ) {
        let mut var_argp__blk284: f64 = *var_argp__blk284_slot;
        let mut var_argp__blk284_db0: f64 = *var_argp__blk284_db0_slot;
        let mut var_argp__blk284_db1: f64 = *var_argp__blk284_db1_slot;
        let mut var_argp__blk284_dn0: f64 = *var_argp__blk284_dn0_slot;
        let mut var_argp__blk284_dn1: f64 = *var_argp__blk284_dn1_slot;
        let mut var_argp__blk284_dn2: f64 = *var_argp__blk284_dn2_slot;
        let mut var_argp__blk284_dn3: f64 = *var_argp__blk284_dn3_slot;
        let mut var_argp__blk284_dn4: f64 = *var_argp__blk284_dn4_slot;
        let mut var_argp__blk284_dn5: f64 = *var_argp__blk284_dn5_slot;
        let mut var_argp__blk284_rdb0: f64 = *var_argp__blk284_rdb0_slot;
        let mut var_argp__blk284_rdb1: f64 = *var_argp__blk284_rdb1_slot;
        let mut var_argp__blk284_rdn0: f64 = *var_argp__blk284_rdn0_slot;
        let mut var_argp__blk284_rdn1: f64 = *var_argp__blk284_rdn1_slot;
        let mut var_argp__blk284_rdn2: f64 = *var_argp__blk284_rdn2_slot;
        let mut var_argp__blk284_rdn3: f64 = *var_argp__blk284_rdn3_slot;
        let mut var_argp__blk284_rdn4: f64 = *var_argp__blk284_rdn4_slot;
        let mut var_argp__blk284_rdn5: f64 = *var_argp__blk284_rdn5_slot;
        let mut var_argp__blk284_rv: f64 = *var_argp__blk284_rv_slot;
        let mut var_dv__blk306: f64 = *var_dv__blk306_slot;
        let mut var_dv__blk306_db0: f64 = *var_dv__blk306_db0_slot;
        let mut var_dv__blk306_db1: f64 = *var_dv__blk306_db1_slot;
        let mut var_dv__blk306_dn0: f64 = *var_dv__blk306_dn0_slot;
        let mut var_dv__blk306_dn1: f64 = *var_dv__blk306_dn1_slot;
        let mut var_dv__blk306_dn2: f64 = *var_dv__blk306_dn2_slot;
        let mut var_dv__blk306_dn3: f64 = *var_dv__blk306_dn3_slot;
        let mut var_dv__blk306_dn4: f64 = *var_dv__blk306_dn4_slot;
        let mut var_dv__blk306_dn5: f64 = *var_dv__blk306_dn5_slot;
        let mut var_dv__blk306_rdb0: f64 = *var_dv__blk306_rdb0_slot;
        let mut var_dv__blk306_rdb1: f64 = *var_dv__blk306_rdb1_slot;
        let mut var_dv__blk306_rdn0: f64 = *var_dv__blk306_rdn0_slot;
        let mut var_dv__blk306_rdn1: f64 = *var_dv__blk306_rdn1_slot;
        let mut var_dv__blk306_rdn2: f64 = *var_dv__blk306_rdn2_slot;
        let mut var_dv__blk306_rdn3: f64 = *var_dv__blk306_rdn3_slot;
        let mut var_dv__blk306_rdn4: f64 = *var_dv__blk306_rdn4_slot;
        let mut var_dv__blk306_rdn5: f64 = *var_dv__blk306_rdn5_slot;
        let mut var_dv__blk306_rv: f64 = *var_dv__blk306_rv_slot;
        let mut var_dvh__blk300: f64 = *var_dvh__blk300_slot;
        let mut var_dvh__blk300_db0: f64 = *var_dvh__blk300_db0_slot;
        let mut var_dvh__blk300_db1: f64 = *var_dvh__blk300_db1_slot;
        let mut var_dvh__blk300_dn0: f64 = *var_dvh__blk300_dn0_slot;
        let mut var_dvh__blk300_dn1: f64 = *var_dvh__blk300_dn1_slot;
        let mut var_dvh__blk300_dn2: f64 = *var_dvh__blk300_dn2_slot;
        let mut var_dvh__blk300_dn3: f64 = *var_dvh__blk300_dn3_slot;
        let mut var_dvh__blk300_dn4: f64 = *var_dvh__blk300_dn4_slot;
        let mut var_dvh__blk300_dn5: f64 = *var_dvh__blk300_dn5_slot;
        let mut var_dvh__blk300_rdb0: f64 = *var_dvh__blk300_rdb0_slot;
        let mut var_dvh__blk300_rdb1: f64 = *var_dvh__blk300_rdb1_slot;
        let mut var_dvh__blk300_rdn0: f64 = *var_dvh__blk300_rdn0_slot;
        let mut var_dvh__blk300_rdn1: f64 = *var_dvh__blk300_rdn1_slot;
        let mut var_dvh__blk300_rdn2: f64 = *var_dvh__blk300_rdn2_slot;
        let mut var_dvh__blk300_rdn3: f64 = *var_dvh__blk300_rdn3_slot;
        let mut var_dvh__blk300_rdn4: f64 = *var_dvh__blk300_rdn4_slot;
        let mut var_dvh__blk300_rdn5: f64 = *var_dvh__blk300_rdn5_slot;
        let mut var_dvh__blk300_rv: f64 = *var_dvh__blk300_rv_slot;
        let mut var_guard309: f64 = *var_guard309_slot;
        let mut var_guard309_db0: f64 = *var_guard309_db0_slot;
        let mut var_guard309_db1: f64 = *var_guard309_db1_slot;
        let mut var_guard309_dn0: f64 = *var_guard309_dn0_slot;
        let mut var_guard309_dn1: f64 = *var_guard309_dn1_slot;
        let mut var_guard309_dn2: f64 = *var_guard309_dn2_slot;
        let mut var_guard309_dn3: f64 = *var_guard309_dn3_slot;
        let mut var_guard309_dn4: f64 = *var_guard309_dn4_slot;
        let mut var_guard309_dn5: f64 = *var_guard309_dn5_slot;
        let mut var_guard309_rdb0: f64 = *var_guard309_rdb0_slot;
        let mut var_guard309_rdb1: f64 = *var_guard309_rdb1_slot;
        let mut var_guard309_rdn0: f64 = *var_guard309_rdn0_slot;
        let mut var_guard309_rdn1: f64 = *var_guard309_rdn1_slot;
        let mut var_guard309_rdn2: f64 = *var_guard309_rdn2_slot;
        let mut var_guard309_rdn3: f64 = *var_guard309_rdn3_slot;
        let mut var_guard309_rdn4: f64 = *var_guard309_rdn4_slot;
        let mut var_guard309_rdn5: f64 = *var_guard309_rdn5_slot;
        let mut var_guard309_rv: f64 = *var_guard309_rv_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard310_db0: f64 = *var_guard310_db0_slot;
        let mut var_guard310_db1: f64 = *var_guard310_db1_slot;
        let mut var_guard310_dn0: f64 = *var_guard310_dn0_slot;
        let mut var_guard310_dn1: f64 = *var_guard310_dn1_slot;
        let mut var_guard310_dn2: f64 = *var_guard310_dn2_slot;
        let mut var_guard310_dn3: f64 = *var_guard310_dn3_slot;
        let mut var_guard310_dn4: f64 = *var_guard310_dn4_slot;
        let mut var_guard310_dn5: f64 = *var_guard310_dn5_slot;
        let mut var_guard310_rdb0: f64 = *var_guard310_rdb0_slot;
        let mut var_guard310_rdb1: f64 = *var_guard310_rdb1_slot;
        let mut var_guard310_rdn0: f64 = *var_guard310_rdn0_slot;
        let mut var_guard310_rdn1: f64 = *var_guard310_rdn1_slot;
        let mut var_guard310_rdn2: f64 = *var_guard310_rdn2_slot;
        let mut var_guard310_rdn3: f64 = *var_guard310_rdn3_slot;
        let mut var_guard310_rdn4: f64 = *var_guard310_rdn4_slot;
        let mut var_guard310_rdn5: f64 = *var_guard310_rdn5_slot;
        let mut var_guard310_rv: f64 = *var_guard310_rv_slot;
        let mut var_mv0__blk304: f64 = *var_mv0__blk304_slot;
        let mut var_mv0__blk304_db0: f64 = *var_mv0__blk304_db0_slot;
        let mut var_mv0__blk304_db1: f64 = *var_mv0__blk304_db1_slot;
        let mut var_mv0__blk304_dn0: f64 = *var_mv0__blk304_dn0_slot;
        let mut var_mv0__blk304_dn1: f64 = *var_mv0__blk304_dn1_slot;
        let mut var_mv0__blk304_dn2: f64 = *var_mv0__blk304_dn2_slot;
        let mut var_mv0__blk304_dn3: f64 = *var_mv0__blk304_dn3_slot;
        let mut var_mv0__blk304_dn4: f64 = *var_mv0__blk304_dn4_slot;
        let mut var_mv0__blk304_dn5: f64 = *var_mv0__blk304_dn5_slot;
        let mut var_mv0__blk304_rdb0: f64 = *var_mv0__blk304_rdb0_slot;
        let mut var_mv0__blk304_rdb1: f64 = *var_mv0__blk304_rdb1_slot;
        let mut var_mv0__blk304_rdn0: f64 = *var_mv0__blk304_rdn0_slot;
        let mut var_mv0__blk304_rdn1: f64 = *var_mv0__blk304_rdn1_slot;
        let mut var_mv0__blk304_rdn2: f64 = *var_mv0__blk304_rdn2_slot;
        let mut var_mv0__blk304_rdn3: f64 = *var_mv0__blk304_rdn3_slot;
        let mut var_mv0__blk304_rdn4: f64 = *var_mv0__blk304_rdn4_slot;
        let mut var_mv0__blk304_rdn5: f64 = *var_mv0__blk304_rdn5_slot;
        let mut var_mv0__blk304_rv: f64 = *var_mv0__blk304_rv_slot;
        let mut var_mv__blk307: f64 = *var_mv__blk307_slot;
        let mut var_mv__blk307_db0: f64 = *var_mv__blk307_db0_slot;
        let mut var_mv__blk307_db1: f64 = *var_mv__blk307_db1_slot;
        let mut var_mv__blk307_dn0: f64 = *var_mv__blk307_dn0_slot;
        let mut var_mv__blk307_dn1: f64 = *var_mv__blk307_dn1_slot;
        let mut var_mv__blk307_dn2: f64 = *var_mv__blk307_dn2_slot;
        let mut var_mv__blk307_dn3: f64 = *var_mv__blk307_dn3_slot;
        let mut var_mv__blk307_dn4: f64 = *var_mv__blk307_dn4_slot;
        let mut var_mv__blk307_dn5: f64 = *var_mv__blk307_dn5_slot;
        let mut var_mv__blk307_rdb0: f64 = *var_mv__blk307_rdb0_slot;
        let mut var_mv__blk307_rdb1: f64 = *var_mv__blk307_rdb1_slot;
        let mut var_mv__blk307_rdn0: f64 = *var_mv__blk307_rdn0_slot;
        let mut var_mv__blk307_rdn1: f64 = *var_mv__blk307_rdn1_slot;
        let mut var_mv__blk307_rdn2: f64 = *var_mv__blk307_rdn2_slot;
        let mut var_mv__blk307_rdn3: f64 = *var_mv__blk307_rdn3_slot;
        let mut var_mv__blk307_rdn4: f64 = *var_mv__blk307_rdn4_slot;
        let mut var_mv__blk307_rdn5: f64 = *var_mv__blk307_rdn5_slot;
        let mut var_mv__blk307_rv: f64 = *var_mv__blk307_rv_slot;
        let mut var_pwq__blk301: f64 = *var_pwq__blk301_slot;
        let mut var_pwq__blk301_db0: f64 = *var_pwq__blk301_db0_slot;
        let mut var_pwq__blk301_db1: f64 = *var_pwq__blk301_db1_slot;
        let mut var_pwq__blk301_dn0: f64 = *var_pwq__blk301_dn0_slot;
        let mut var_pwq__blk301_dn1: f64 = *var_pwq__blk301_dn1_slot;
        let mut var_pwq__blk301_dn2: f64 = *var_pwq__blk301_dn2_slot;
        let mut var_pwq__blk301_dn3: f64 = *var_pwq__blk301_dn3_slot;
        let mut var_pwq__blk301_dn4: f64 = *var_pwq__blk301_dn4_slot;
        let mut var_pwq__blk301_dn5: f64 = *var_pwq__blk301_dn5_slot;
        let mut var_pwq__blk301_rdb0: f64 = *var_pwq__blk301_rdb0_slot;
        let mut var_pwq__blk301_rdb1: f64 = *var_pwq__blk301_rdb1_slot;
        let mut var_pwq__blk301_rdn0: f64 = *var_pwq__blk301_rdn0_slot;
        let mut var_pwq__blk301_rdn1: f64 = *var_pwq__blk301_rdn1_slot;
        let mut var_pwq__blk301_rdn2: f64 = *var_pwq__blk301_rdn2_slot;
        let mut var_pwq__blk301_rdn3: f64 = *var_pwq__blk301_rdn3_slot;
        let mut var_pwq__blk301_rdn4: f64 = *var_pwq__blk301_rdn4_slot;
        let mut var_pwq__blk301_rdn5: f64 = *var_pwq__blk301_rdn5_slot;
        let mut var_pwq__blk301_rv: f64 = *var_pwq__blk301_rv_slot;
        let mut var_qhi__blk303: f64 = *var_qhi__blk303_slot;
        let mut var_qhi__blk303_db0: f64 = *var_qhi__blk303_db0_slot;
        let mut var_qhi__blk303_db1: f64 = *var_qhi__blk303_db1_slot;
        let mut var_qhi__blk303_dn0: f64 = *var_qhi__blk303_dn0_slot;
        let mut var_qhi__blk303_dn1: f64 = *var_qhi__blk303_dn1_slot;
        let mut var_qhi__blk303_dn2: f64 = *var_qhi__blk303_dn2_slot;
        let mut var_qhi__blk303_dn3: f64 = *var_qhi__blk303_dn3_slot;
        let mut var_qhi__blk303_dn4: f64 = *var_qhi__blk303_dn4_slot;
        let mut var_qhi__blk303_dn5: f64 = *var_qhi__blk303_dn5_slot;
        let mut var_qhi__blk303_rdb0: f64 = *var_qhi__blk303_rdb0_slot;
        let mut var_qhi__blk303_rdb1: f64 = *var_qhi__blk303_rdb1_slot;
        let mut var_qhi__blk303_rdn0: f64 = *var_qhi__blk303_rdn0_slot;
        let mut var_qhi__blk303_rdn1: f64 = *var_qhi__blk303_rdn1_slot;
        let mut var_qhi__blk303_rdn2: f64 = *var_qhi__blk303_rdn2_slot;
        let mut var_qhi__blk303_rdn3: f64 = *var_qhi__blk303_rdn3_slot;
        let mut var_qhi__blk303_rdn4: f64 = *var_qhi__blk303_rdn4_slot;
        let mut var_qhi__blk303_rdn5: f64 = *var_qhi__blk303_rdn5_slot;
        let mut var_qhi__blk303_rv: f64 = *var_qhi__blk303_rv_slot;
        let mut var_qlo__blk302: f64 = *var_qlo__blk302_slot;
        let mut var_qlo__blk302_db0: f64 = *var_qlo__blk302_db0_slot;
        let mut var_qlo__blk302_db1: f64 = *var_qlo__blk302_db1_slot;
        let mut var_qlo__blk302_dn0: f64 = *var_qlo__blk302_dn0_slot;
        let mut var_qlo__blk302_dn1: f64 = *var_qlo__blk302_dn1_slot;
        let mut var_qlo__blk302_dn2: f64 = *var_qlo__blk302_dn2_slot;
        let mut var_qlo__blk302_dn3: f64 = *var_qlo__blk302_dn3_slot;
        let mut var_qlo__blk302_dn4: f64 = *var_qlo__blk302_dn4_slot;
        let mut var_qlo__blk302_dn5: f64 = *var_qlo__blk302_dn5_slot;
        let mut var_qlo__blk302_rdb0: f64 = *var_qlo__blk302_rdb0_slot;
        let mut var_qlo__blk302_rdb1: f64 = *var_qlo__blk302_rdb1_slot;
        let mut var_qlo__blk302_rdn0: f64 = *var_qlo__blk302_rdn0_slot;
        let mut var_qlo__blk302_rdn1: f64 = *var_qlo__blk302_rdn1_slot;
        let mut var_qlo__blk302_rdn2: f64 = *var_qlo__blk302_rdn2_slot;
        let mut var_qlo__blk302_rdn3: f64 = *var_qlo__blk302_rdn3_slot;
        let mut var_qlo__blk302_rdn4: f64 = *var_qlo__blk302_rdn4_slot;
        let mut var_qlo__blk302_rdn5: f64 = *var_qlo__blk302_rdn5_slot;
        let mut var_qlo__blk302_rv: f64 = *var_qlo__blk302_rv_slot;
        let mut var_vl0__blk305: f64 = *var_vl0__blk305_slot;
        let mut var_vl0__blk305_db0: f64 = *var_vl0__blk305_db0_slot;
        let mut var_vl0__blk305_db1: f64 = *var_vl0__blk305_db1_slot;
        let mut var_vl0__blk305_dn0: f64 = *var_vl0__blk305_dn0_slot;
        let mut var_vl0__blk305_dn1: f64 = *var_vl0__blk305_dn1_slot;
        let mut var_vl0__blk305_dn2: f64 = *var_vl0__blk305_dn2_slot;
        let mut var_vl0__blk305_dn3: f64 = *var_vl0__blk305_dn3_slot;
        let mut var_vl0__blk305_dn4: f64 = *var_vl0__blk305_dn4_slot;
        let mut var_vl0__blk305_dn5: f64 = *var_vl0__blk305_dn5_slot;
        let mut var_vl0__blk305_rdb0: f64 = *var_vl0__blk305_rdb0_slot;
        let mut var_vl0__blk305_rdb1: f64 = *var_vl0__blk305_rdb1_slot;
        let mut var_vl0__blk305_rdn0: f64 = *var_vl0__blk305_rdn0_slot;
        let mut var_vl0__blk305_rdn1: f64 = *var_vl0__blk305_rdn1_slot;
        let mut var_vl0__blk305_rdn2: f64 = *var_vl0__blk305_rdn2_slot;
        let mut var_vl0__blk305_rdn3: f64 = *var_vl0__blk305_rdn3_slot;
        let mut var_vl0__blk305_rdn4: f64 = *var_vl0__blk305_rdn4_slot;
        let mut var_vl0__blk305_rdn5: f64 = *var_vl0__blk305_rdn5_slot;
        let mut var_vl0__blk305_rv: f64 = *var_vl0__blk305_rv_slot;
        let mut var_vl__blk308: f64 = *var_vl__blk308_slot;
        let mut var_vl__blk308_db0: f64 = *var_vl__blk308_db0_slot;
        let mut var_vl__blk308_db1: f64 = *var_vl__blk308_db1_slot;
        let mut var_vl__blk308_dn0: f64 = *var_vl__blk308_dn0_slot;
        let mut var_vl__blk308_dn1: f64 = *var_vl__blk308_dn1_slot;
        let mut var_vl__blk308_dn2: f64 = *var_vl__blk308_dn2_slot;
        let mut var_vl__blk308_dn3: f64 = *var_vl__blk308_dn3_slot;
        let mut var_vl__blk308_dn4: f64 = *var_vl__blk308_dn4_slot;
        let mut var_vl__blk308_dn5: f64 = *var_vl__blk308_dn5_slot;
        let mut var_vl__blk308_rdb0: f64 = *var_vl__blk308_rdb0_slot;
        let mut var_vl__blk308_rdb1: f64 = *var_vl__blk308_rdb1_slot;
        let mut var_vl__blk308_rdn0: f64 = *var_vl__blk308_rdn0_slot;
        let mut var_vl__blk308_rdn1: f64 = *var_vl__blk308_rdn1_slot;
        let mut var_vl__blk308_rdn2: f64 = *var_vl__blk308_rdn2_slot;
        let mut var_vl__blk308_rdn3: f64 = *var_vl__blk308_rdn3_slot;
        let mut var_vl__blk308_rdn4: f64 = *var_vl__blk308_rdn4_slot;
        let mut var_vl__blk308_rdn5: f64 = *var_vl__blk308_rdn5_slot;
        let mut var_vl__blk308_rv: f64 = *var_vl__blk308_rv_slot;

        let assign4510_e4626: f64 = if p.p82 <= 0.0 { 1.0 } else { 0.0 };
        var_guard309 = assign4510_e4626;
        var_guard309_dn0 = 0.0;
        var_guard309_dn1 = 0.0;
        var_guard309_dn2 = 0.0;
        var_guard309_dn3 = 0.0;
        var_guard309_dn4 = 0.0;
        var_guard309_dn5 = 0.0;
        var_guard309_db0 = 0.0;
        var_guard309_db1 = 0.0;
        var_guard309_rv = 0.0;
        var_guard309_rdn0 = 0.0;
        var_guard309_rdn1 = 0.0;
        var_guard309_rdn2 = 0.0;
        var_guard309_rdn3 = 0.0;
        var_guard309_rdn4 = 0.0;
        var_guard309_rdn5 = 0.0;
        var_guard309_rdb0 = 0.0;
        var_guard309_rdb1 = 0.0;

        let (assign4520_e4636, assign4520_e4636_d_n0, assign4520_e4636_d_n1, assign4520_e4636_d_n2, assign4520_e4636_d_n3, assign4520_e4636_d_n4, assign4520_e4636_d_n5, assign4520_e4636_d_b0, assign4520_e4636_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) {
        let assign4520_e4634: f64 = (var_vcl + var_dv0__blk299);
        (assign4520_e4634, (var_vcl_dn0 + var_dv0__blk299_dn0), (var_vcl_dn1 + var_dv0__blk299_dn1), (var_vcl_dn2 + var_dv0__blk299_dn2), (var_vcl_dn3 + var_dv0__blk299_dn3), (var_vcl_dn4 + var_dv0__blk299_dn4), (var_vcl_dn5 + var_dv0__blk299_dn5), (var_vcl_db0 + var_dv0__blk299_db0), (var_vcl_db1 + var_dv0__blk299_db1),)
    } else {
        (var_dvh__blk300, var_dvh__blk300_dn0, var_dvh__blk300_dn1, var_dvh__blk300_dn2, var_dvh__blk300_dn3, var_dvh__blk300_dn4, var_dvh__blk300_dn5, var_dvh__blk300_db0, var_dvh__blk300_db1,)
    }
};
        var_dvh__blk300 = assign4520_e4636;
        var_dvh__blk300_dn0 = assign4520_e4636_d_n0;
        var_dvh__blk300_dn1 = assign4520_e4636_d_n1;
        var_dvh__blk300_dn2 = assign4520_e4636_d_n2;
        var_dvh__blk300_dn3 = assign4520_e4636_d_n3;
        var_dvh__blk300_dn4 = assign4520_e4636_d_n4;
        var_dvh__blk300_dn5 = assign4520_e4636_d_n5;
        var_dvh__blk300_db0 = assign4520_e4636_d_b0;
        var_dvh__blk300_db1 = assign4520_e4636_d_b1;
        var_dvh__blk300_rv = 0.0;
        var_dvh__blk300_rdn0 = 0.0;
        var_dvh__blk300_rdn1 = 0.0;
        var_dvh__blk300_rdn2 = 0.0;
        var_dvh__blk300_rdn3 = 0.0;
        var_dvh__blk300_rdn4 = 0.0;
        var_dvh__blk300_rdn5 = 0.0;
        var_dvh__blk300_rdb0 = 0.0;
        var_dvh__blk300_rdb1 = 0.0;

        let assign4530_e4639: f64 = if var_dvh__blk300 > 0.0 { 1.0 } else { 0.0 };
        var_guard310 = assign4530_e4639;
        var_guard310_dn0 = 0.0;
        var_guard310_dn1 = 0.0;
        var_guard310_dn2 = 0.0;
        var_guard310_dn3 = 0.0;
        var_guard310_dn4 = 0.0;
        var_guard310_dn5 = 0.0;
        var_guard310_db0 = 0.0;
        var_guard310_db1 = 0.0;
        var_guard310_rv = 0.0;
        var_guard310_rdn0 = 0.0;
        var_guard310_rdn1 = 0.0;
        var_guard310_rdn2 = 0.0;
        var_guard310_rdn3 = 0.0;
        var_guard310_rdn4 = 0.0;
        var_guard310_rdn5 = 0.0;
        var_guard310_rdb0 = 0.0;
        var_guard310_rdb1 = 0.0;

        let (assign4540_e4654, assign4540_e4654_d_n0, assign4540_e4654_d_n1, assign4540_e4654_d_n2, assign4540_e4654_d_n3, assign4540_e4654_d_n4, assign4540_e4654_d_n5, assign4540_e4654_d_b0, assign4540_e4654_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) && (var_guard310 != 0.0)) {
        let assign4540_e4649: f64 = (1.0 - p.p68);
        let assign4540_e4651: f64 = (-p.p81);
        let assign4540_e4652: f64 = (assign4540_e4649).powf(assign4540_e4651);
        (assign4540_e4652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_pwq__blk301, var_pwq__blk301_dn0, var_pwq__blk301_dn1, var_pwq__blk301_dn2, var_pwq__blk301_dn3, var_pwq__blk301_dn4, var_pwq__blk301_dn5, var_pwq__blk301_db0, var_pwq__blk301_db1,)
    }
};
        var_pwq__blk301 = assign4540_e4654;
        var_pwq__blk301_dn0 = assign4540_e4654_d_n0;
        var_pwq__blk301_dn1 = assign4540_e4654_d_n1;
        var_pwq__blk301_dn2 = assign4540_e4654_d_n2;
        var_pwq__blk301_dn3 = assign4540_e4654_d_n3;
        var_pwq__blk301_dn4 = assign4540_e4654_d_n4;
        var_pwq__blk301_dn5 = assign4540_e4654_d_n5;
        var_pwq__blk301_db0 = assign4540_e4654_d_b0;
        var_pwq__blk301_db1 = assign4540_e4654_d_b1;
        var_pwq__blk301_rv = 0.0;
        var_pwq__blk301_rdn0 = 0.0;
        var_pwq__blk301_rdn1 = 0.0;
        var_pwq__blk301_rdn2 = 0.0;
        var_pwq__blk301_rdn3 = 0.0;
        var_pwq__blk301_rdn4 = 0.0;
        var_pwq__blk301_rdn5 = 0.0;
        var_pwq__blk301_rdb0 = 0.0;
        var_pwq__blk301_rdb1 = 0.0;

        let (assign4550_e4676, assign4550_e4676_d_n0, assign4550_e4676_d_n1, assign4550_e4676_d_n2, assign4550_e4676_d_n3, assign4550_e4676_d_n4, assign4550_e4676_d_n5, assign4550_e4676_d_b0, assign4550_e4676_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) && (var_guard310 != 0.0)) {
        let assign4550_e4667: f64 = (1.0 - p.p68);
        let assign4550_e4668: f64 = (var_pwq__blk301 * assign4550_e4667);
        let assign4550_e4669: f64 = (1.0 - assign4550_e4668);
        let assign4550_e4670: f64 = (var_pp_t * assign4550_e4669);
        let assign4550_e4673: f64 = (1.0 - p.p81);
        let assign4550_e4674: f64 = (assign4550_e4670 / assign4550_e4673);
        (assign4550_e4674, (((var_pp_t_dn0 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn0 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_dn1 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn1 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_dn2 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn2 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_dn3 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn3 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_dn4 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn4 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_dn5 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_dn5 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_db0 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_db0 * assign4550_e4667)))) / assign4550_e4673), (((var_pp_t_db1 * assign4550_e4669) + (var_pp_t * (-(var_pwq__blk301_db1 * assign4550_e4667)))) / assign4550_e4673),)
    } else {
        (var_qlo__blk302, var_qlo__blk302_dn0, var_qlo__blk302_dn1, var_qlo__blk302_dn2, var_qlo__blk302_dn3, var_qlo__blk302_dn4, var_qlo__blk302_dn5, var_qlo__blk302_db0, var_qlo__blk302_db1,)
    }
};
        var_qlo__blk302 = assign4550_e4676;
        var_qlo__blk302_dn0 = assign4550_e4676_d_n0;
        var_qlo__blk302_dn1 = assign4550_e4676_d_n1;
        var_qlo__blk302_dn2 = assign4550_e4676_d_n2;
        var_qlo__blk302_dn3 = assign4550_e4676_d_n3;
        var_qlo__blk302_dn4 = assign4550_e4676_d_n4;
        var_qlo__blk302_dn5 = assign4550_e4676_d_n5;
        var_qlo__blk302_db0 = assign4550_e4676_d_b0;
        var_qlo__blk302_db1 = assign4550_e4676_d_b1;
        var_qlo__blk302_rv = 0.0;
        var_qlo__blk302_rdn0 = 0.0;
        var_qlo__blk302_rdn1 = 0.0;
        var_qlo__blk302_rdn2 = 0.0;
        var_qlo__blk302_rdn3 = 0.0;
        var_qlo__blk302_rdn4 = 0.0;
        var_qlo__blk302_rdn5 = 0.0;
        var_qlo__blk302_rdb0 = 0.0;
        var_qlo__blk302_rdb1 = 0.0;

        let (assign4560_e4702, assign4560_e4702_d_n0, assign4560_e4702_d_n1, assign4560_e4702_d_n2, assign4560_e4702_d_n3, assign4560_e4702_d_n4, assign4560_e4702_d_n5, assign4560_e4702_d_b0, assign4560_e4702_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) && (var_guard310 != 0.0)) {
        let assign4560_e4688: f64 = (0.5 * p.p81);
        let assign4560_e4690: f64 = (assign4560_e4688 * var_dvh__blk300);
        let assign4560_e4694: f64 = (1.0 - p.p68);
        let assign4560_e4695: f64 = (var_pp_t * assign4560_e4694);
        let assign4560_e4696: f64 = (assign4560_e4690 / assign4560_e4695);
        let assign4560_e4697: f64 = (1.0 + assign4560_e4696);
        let assign4560_e4698: f64 = (var_dvh__blk300 * assign4560_e4697);
        let assign4560_e4700: f64 = (assign4560_e4698 * var_pwq__blk301);
        (assign4560_e4700, ((((var_dvh__blk300_dn0 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn0) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn0 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn0)), ((((var_dvh__blk300_dn1 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn1) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn1 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn1)), ((((var_dvh__blk300_dn2 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn2) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn2 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn2)), ((((var_dvh__blk300_dn3 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn3) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn3 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn3)), ((((var_dvh__blk300_dn4 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn4) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn4 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn4)), ((((var_dvh__blk300_dn5 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_dn5) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_dn5 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_dn5)), ((((var_dvh__blk300_db0 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_db0) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_db0 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_db0)), ((((var_dvh__blk300_db1 * assign4560_e4697) + (var_dvh__blk300 * ((((assign4560_e4688 * var_dvh__blk300_db1) * assign4560_e4695) - (assign4560_e4690 * (var_pp_t_db1 * assign4560_e4694))) / (assign4560_e4695 * assign4560_e4695)))) * var_pwq__blk301) + (assign4560_e4698 * var_pwq__blk301_db1)),)
    } else {
        (var_qhi__blk303, var_qhi__blk303_dn0, var_qhi__blk303_dn1, var_qhi__blk303_dn2, var_qhi__blk303_dn3, var_qhi__blk303_dn4, var_qhi__blk303_dn5, var_qhi__blk303_db0, var_qhi__blk303_db1,)
    }
};
        var_qhi__blk303 = assign4560_e4702;
        var_qhi__blk303_dn0 = assign4560_e4702_d_n0;
        var_qhi__blk303_dn1 = assign4560_e4702_d_n1;
        var_qhi__blk303_dn2 = assign4560_e4702_d_n2;
        var_qhi__blk303_dn3 = assign4560_e4702_d_n3;
        var_qhi__blk303_dn4 = assign4560_e4702_d_n4;
        var_qhi__blk303_dn5 = assign4560_e4702_d_n5;
        var_qhi__blk303_db0 = assign4560_e4702_d_b0;
        var_qhi__blk303_db1 = assign4560_e4702_d_b1;
        var_qhi__blk303_rv = 0.0;
        var_qhi__blk303_rdn0 = 0.0;
        var_qhi__blk303_rdn1 = 0.0;
        var_qhi__blk303_rdn2 = 0.0;
        var_qhi__blk303_rdn3 = 0.0;
        var_qhi__blk303_rdn4 = 0.0;
        var_qhi__blk303_rdn5 = 0.0;
        var_qhi__blk303_rdb0 = 0.0;
        var_qhi__blk303_rdb1 = 0.0;

        let (assign4570_e4729, assign4570_e4729_d_n0, assign4570_e4729_d_n1, assign4570_e4729_d_n2, assign4570_e4729_d_n3, assign4570_e4729_d_n4, assign4570_e4729_d_n5, assign4570_e4729_d_b0, assign4570_e4729_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) && (var_guard310 == 0.0)) {
        let assign4570_e4716: f64 = (var_vcl / var_pp_t);
        let assign4570_e4717: f64 = (1.0 - assign4570_e4716);
        let assign4570_e4720: f64 = (1.0 - p.p81);
        let assign4570_e4721: f64 = (assign4570_e4717).powf(assign4570_e4720);
        let assign4570_e4722: f64 = (1.0 - assign4570_e4721);
        let assign4570_e4723: f64 = (var_pp_t * assign4570_e4722);
        let assign4570_e4726: f64 = (1.0 - p.p81);
        let assign4570_e4727: f64 = (assign4570_e4723 / assign4570_e4726);
        (assign4570_e4727, (((var_pp_t_dn0 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn0 * var_pp_t) - (var_vcl * var_pp_t_dn0)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn0 * var_pp_t) - (var_vcl * var_pp_t_dn0)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_dn1 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn1 * var_pp_t) - (var_vcl * var_pp_t_dn1)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn1 * var_pp_t) - (var_vcl * var_pp_t_dn1)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_dn2 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn2 * var_pp_t) - (var_vcl * var_pp_t_dn2)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn2 * var_pp_t) - (var_vcl * var_pp_t_dn2)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_dn3 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn3 * var_pp_t) - (var_vcl * var_pp_t_dn3)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn3 * var_pp_t) - (var_vcl * var_pp_t_dn3)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_dn4 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn4 * var_pp_t) - (var_vcl * var_pp_t_dn4)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn4 * var_pp_t) - (var_vcl * var_pp_t_dn4)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_dn5 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_dn5 * var_pp_t) - (var_vcl * var_pp_t_dn5)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_dn5 * var_pp_t) - (var_vcl * var_pp_t_dn5)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_db0 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_db0 * var_pp_t) - (var_vcl * var_pp_t_db0)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_db0 * var_pp_t) - (var_vcl * var_pp_t_db0)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726), (((var_pp_t_db1 * assign4570_e4722) + (var_pp_t * (-if 0.0 == 0.0 && ((assign4570_e4720) as f64).is_finite() && ((assign4570_e4720) as f64).fract() == 0.0 { if assign4570_e4720 == 0.0 { 0.0 } else { (assign4570_e4720 * ((assign4570_e4717).powf(assign4570_e4720 - 1.0) * (-(((var_vcl_db1 * var_pp_t) - (var_vcl * var_pp_t_db1)) / (var_pp_t * var_pp_t))))) } } else { (assign4570_e4721 * (assign4570_e4720 * ((-(((var_vcl_db1 * var_pp_t) - (var_vcl * var_pp_t_db1)) / (var_pp_t * var_pp_t))) / assign4570_e4717))) }))) / assign4570_e4726),)
    } else {
        (var_qlo__blk302, var_qlo__blk302_dn0, var_qlo__blk302_dn1, var_qlo__blk302_dn2, var_qlo__blk302_dn3, var_qlo__blk302_dn4, var_qlo__blk302_dn5, var_qlo__blk302_db0, var_qlo__blk302_db1,)
    }
};
        var_qlo__blk302 = assign4570_e4729;
        var_qlo__blk302_dn0 = assign4570_e4729_d_n0;
        var_qlo__blk302_dn1 = assign4570_e4729_d_n1;
        var_qlo__blk302_dn2 = assign4570_e4729_d_n2;
        var_qlo__blk302_dn3 = assign4570_e4729_d_n3;
        var_qlo__blk302_dn4 = assign4570_e4729_d_n4;
        var_qlo__blk302_dn5 = assign4570_e4729_d_n5;
        var_qlo__blk302_db0 = assign4570_e4729_d_b0;
        var_qlo__blk302_db1 = assign4570_e4729_d_b1;
        var_qlo__blk302_rv = 0.0;
        var_qlo__blk302_rdn0 = 0.0;
        var_qlo__blk302_rdn1 = 0.0;
        var_qlo__blk302_rdn2 = 0.0;
        var_qlo__blk302_rdn3 = 0.0;
        var_qlo__blk302_rdn4 = 0.0;
        var_qlo__blk302_rdn5 = 0.0;
        var_qlo__blk302_rdb0 = 0.0;
        var_qlo__blk302_rdb1 = 0.0;

        let (assign4580_e4740, assign4580_e4740_d_n0, assign4580_e4740_d_n1, assign4580_e4740_d_n2, assign4580_e4740_d_n3, assign4580_e4740_d_n4, assign4580_e4740_d_n5, assign4580_e4740_d_b0, assign4580_e4740_d_b1,) = {
    if ((((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) && (var_guard310 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qhi__blk303, var_qhi__blk303_dn0, var_qhi__blk303_dn1, var_qhi__blk303_dn2, var_qhi__blk303_dn3, var_qhi__blk303_dn4, var_qhi__blk303_dn5, var_qhi__blk303_db0, var_qhi__blk303_db1,)
    }
};
        var_qhi__blk303 = assign4580_e4740;
        var_qhi__blk303_dn0 = assign4580_e4740_d_n0;
        var_qhi__blk303_dn1 = assign4580_e4740_d_n1;
        var_qhi__blk303_dn2 = assign4580_e4740_d_n2;
        var_qhi__blk303_dn3 = assign4580_e4740_d_n3;
        var_qhi__blk303_dn4 = assign4580_e4740_d_n4;
        var_qhi__blk303_dn5 = assign4580_e4740_d_n5;
        var_qhi__blk303_db0 = assign4580_e4740_d_b0;
        var_qhi__blk303_db1 = assign4580_e4740_d_b1;
        var_qhi__blk303_rv = 0.0;
        var_qhi__blk303_rdn0 = 0.0;
        var_qhi__blk303_rdn1 = 0.0;
        var_qhi__blk303_rdn2 = 0.0;
        var_qhi__blk303_rdn3 = 0.0;
        var_qhi__blk303_rdn4 = 0.0;
        var_qhi__blk303_rdn5 = 0.0;
        var_qhi__blk303_rdb0 = 0.0;
        var_qhi__blk303_rdb1 = 0.0;

        let (assign4590_e4750, assign4590_e4750_d_n0, assign4590_e4750_d_n1, assign4590_e4750_d_n2, assign4590_e4750_d_n3, assign4590_e4750_d_n4, assign4590_e4750_d_n5, assign4590_e4750_d_b0, assign4590_e4750_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 != 0.0)) {
        let assign4590_e4748: f64 = (var_qlo__blk302 + var_qhi__blk303);
        (assign4590_e4748, (var_qlo__blk302_dn0 + var_qhi__blk303_dn0), (var_qlo__blk302_dn1 + var_qhi__blk303_dn1), (var_qlo__blk302_dn2 + var_qhi__blk303_dn2), (var_qlo__blk302_dn3 + var_qhi__blk303_dn3), (var_qlo__blk302_dn4 + var_qhi__blk303_dn4), (var_qlo__blk302_dn5 + var_qhi__blk303_dn5), (var_qlo__blk302_db0 + var_qhi__blk303_db0), (var_qlo__blk302_db1 + var_qhi__blk303_db1),)
    } else {
        (var_argp__blk284, var_argp__blk284_dn0, var_argp__blk284_dn1, var_argp__blk284_dn2, var_argp__blk284_dn3, var_argp__blk284_dn4, var_argp__blk284_dn5, var_argp__blk284_db0, var_argp__blk284_db1,)
    }
};
        var_argp__blk284 = assign4590_e4750;
        var_argp__blk284_dn0 = assign4590_e4750_d_n0;
        var_argp__blk284_dn1 = assign4590_e4750_d_n1;
        var_argp__blk284_dn2 = assign4590_e4750_d_n2;
        var_argp__blk284_dn3 = assign4590_e4750_d_n3;
        var_argp__blk284_dn4 = assign4590_e4750_d_n4;
        var_argp__blk284_dn5 = assign4590_e4750_d_n5;
        var_argp__blk284_db0 = assign4590_e4750_d_b0;
        var_argp__blk284_db1 = assign4590_e4750_d_b1;
        var_argp__blk284_rv = 0.0;
        var_argp__blk284_rdn0 = 0.0;
        var_argp__blk284_rdn1 = 0.0;
        var_argp__blk284_rdn2 = 0.0;
        var_argp__blk284_rdn3 = 0.0;
        var_argp__blk284_rdn4 = 0.0;
        var_argp__blk284_rdn5 = 0.0;
        var_argp__blk284_rdb0 = 0.0;
        var_argp__blk284_rdb1 = 0.0;

        let (assign4600_e4768, assign4600_e4768_d_n0, assign4600_e4768_d_n1, assign4600_e4768_d_n2, assign4600_e4768_d_n3, assign4600_e4768_d_n4, assign4600_e4768_d_n5, assign4600_e4768_d_b0, assign4600_e4768_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4600_e4759: f64 = (var_dv0__blk299 * var_dv0__blk299);
        let assign4600_e4762: f64 = (4.0 * p.p82);
        let assign4600_e4764: f64 = (assign4600_e4762 * p.p82);
        let assign4600_e4765: f64 = (assign4600_e4759 + assign4600_e4764);
        let assign4600_e4766: f64 = (assign4600_e4765).sqrt();
        (assign4600_e4766, (((var_dv0__blk299_dn0 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn0)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_dn1 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn1)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_dn2 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn2)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_dn3 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn3)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_dn4 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn4)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_dn5 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_dn5)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_db0 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_db0)) / (2.0 * assign4600_e4766)), (((var_dv0__blk299_db1 * var_dv0__blk299) + (var_dv0__blk299 * var_dv0__blk299_db1)) / (2.0 * assign4600_e4766)),)
    } else {
        (var_mv0__blk304, var_mv0__blk304_dn0, var_mv0__blk304_dn1, var_mv0__blk304_dn2, var_mv0__blk304_dn3, var_mv0__blk304_dn4, var_mv0__blk304_dn5, var_mv0__blk304_db0, var_mv0__blk304_db1,)
    }
};
        var_mv0__blk304 = assign4600_e4768;
        var_mv0__blk304_dn0 = assign4600_e4768_d_n0;
        var_mv0__blk304_dn1 = assign4600_e4768_d_n1;
        var_mv0__blk304_dn2 = assign4600_e4768_d_n2;
        var_mv0__blk304_dn3 = assign4600_e4768_d_n3;
        var_mv0__blk304_dn4 = assign4600_e4768_d_n4;
        var_mv0__blk304_dn5 = assign4600_e4768_d_n5;
        var_mv0__blk304_db0 = assign4600_e4768_d_b0;
        var_mv0__blk304_db1 = assign4600_e4768_d_b1;
        var_mv0__blk304_rv = 0.0;
        var_mv0__blk304_rdn0 = 0.0;
        var_mv0__blk304_rdn1 = 0.0;
        var_mv0__blk304_rdn2 = 0.0;
        var_mv0__blk304_rdn3 = 0.0;
        var_mv0__blk304_rdn4 = 0.0;
        var_mv0__blk304_rdn5 = 0.0;
        var_mv0__blk304_rdb0 = 0.0;
        var_mv0__blk304_rdb1 = 0.0;

        let (assign4610_e4782, assign4610_e4782_d_n0, assign4610_e4782_d_n1, assign4610_e4782_d_n2, assign4610_e4782_d_n3, assign4610_e4782_d_n4, assign4610_e4782_d_n5, assign4610_e4782_d_b0, assign4610_e4782_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4610_e4776: f64 = (-0.5);
        let assign4610_e4779: f64 = (var_dv0__blk299 + var_mv0__blk304);
        let assign4610_e4780: f64 = (assign4610_e4776 * assign4610_e4779);
        (assign4610_e4780, (assign4610_e4776 * (var_dv0__blk299_dn0 + var_mv0__blk304_dn0)), (assign4610_e4776 * (var_dv0__blk299_dn1 + var_mv0__blk304_dn1)), (assign4610_e4776 * (var_dv0__blk299_dn2 + var_mv0__blk304_dn2)), (assign4610_e4776 * (var_dv0__blk299_dn3 + var_mv0__blk304_dn3)), (assign4610_e4776 * (var_dv0__blk299_dn4 + var_mv0__blk304_dn4)), (assign4610_e4776 * (var_dv0__blk299_dn5 + var_mv0__blk304_dn5)), (assign4610_e4776 * (var_dv0__blk299_db0 + var_mv0__blk304_db0)), (assign4610_e4776 * (var_dv0__blk299_db1 + var_mv0__blk304_db1)),)
    } else {
        (var_vl0__blk305, var_vl0__blk305_dn0, var_vl0__blk305_dn1, var_vl0__blk305_dn2, var_vl0__blk305_dn3, var_vl0__blk305_dn4, var_vl0__blk305_dn5, var_vl0__blk305_db0, var_vl0__blk305_db1,)
    }
};
        var_vl0__blk305 = assign4610_e4782;
        var_vl0__blk305_dn0 = assign4610_e4782_d_n0;
        var_vl0__blk305_dn1 = assign4610_e4782_d_n1;
        var_vl0__blk305_dn2 = assign4610_e4782_d_n2;
        var_vl0__blk305_dn3 = assign4610_e4782_d_n3;
        var_vl0__blk305_dn4 = assign4610_e4782_d_n4;
        var_vl0__blk305_dn5 = assign4610_e4782_d_n5;
        var_vl0__blk305_db0 = assign4610_e4782_d_b0;
        var_vl0__blk305_db1 = assign4610_e4782_d_b1;
        var_vl0__blk305_rv = 0.0;
        var_vl0__blk305_rdn0 = 0.0;
        var_vl0__blk305_rdn1 = 0.0;
        var_vl0__blk305_rdn2 = 0.0;
        var_vl0__blk305_rdn3 = 0.0;
        var_vl0__blk305_rdn4 = 0.0;
        var_vl0__blk305_rdn5 = 0.0;
        var_vl0__blk305_rdb0 = 0.0;
        var_vl0__blk305_rdb1 = 0.0;

        let (assign4620_e4793, assign4620_e4793_d_n0, assign4620_e4793_d_n1, assign4620_e4793_d_n2, assign4620_e4793_d_n3, assign4620_e4793_d_n4, assign4620_e4793_d_n5, assign4620_e4793_d_b0, assign4620_e4793_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4620_e4791: f64 = (var_vcl + var_dv0__blk299);
        (assign4620_e4791, (var_vcl_dn0 + var_dv0__blk299_dn0), (var_vcl_dn1 + var_dv0__blk299_dn1), (var_vcl_dn2 + var_dv0__blk299_dn2), (var_vcl_dn3 + var_dv0__blk299_dn3), (var_vcl_dn4 + var_dv0__blk299_dn4), (var_vcl_dn5 + var_dv0__blk299_dn5), (var_vcl_db0 + var_dv0__blk299_db0), (var_vcl_db1 + var_dv0__blk299_db1),)
    } else {
        (var_dv__blk306, var_dv__blk306_dn0, var_dv__blk306_dn1, var_dv__blk306_dn2, var_dv__blk306_dn3, var_dv__blk306_dn4, var_dv__blk306_dn5, var_dv__blk306_db0, var_dv__blk306_db1,)
    }
};
        var_dv__blk306 = assign4620_e4793;
        var_dv__blk306_dn0 = assign4620_e4793_d_n0;
        var_dv__blk306_dn1 = assign4620_e4793_d_n1;
        var_dv__blk306_dn2 = assign4620_e4793_d_n2;
        var_dv__blk306_dn3 = assign4620_e4793_d_n3;
        var_dv__blk306_dn4 = assign4620_e4793_d_n4;
        var_dv__blk306_dn5 = assign4620_e4793_d_n5;
        var_dv__blk306_db0 = assign4620_e4793_d_b0;
        var_dv__blk306_db1 = assign4620_e4793_d_b1;
        var_dv__blk306_rv = 0.0;
        var_dv__blk306_rdn0 = 0.0;
        var_dv__blk306_rdn1 = 0.0;
        var_dv__blk306_rdn2 = 0.0;
        var_dv__blk306_rdn3 = 0.0;
        var_dv__blk306_rdn4 = 0.0;
        var_dv__blk306_rdn5 = 0.0;
        var_dv__blk306_rdb0 = 0.0;
        var_dv__blk306_rdb1 = 0.0;

        let (assign4630_e4811, assign4630_e4811_d_n0, assign4630_e4811_d_n1, assign4630_e4811_d_n2, assign4630_e4811_d_n3, assign4630_e4811_d_n4, assign4630_e4811_d_n5, assign4630_e4811_d_b0, assign4630_e4811_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4630_e4802: f64 = (var_dv__blk306 * var_dv__blk306);
        let assign4630_e4805: f64 = (4.0 * p.p82);
        let assign4630_e4807: f64 = (assign4630_e4805 * p.p82);
        let assign4630_e4808: f64 = (assign4630_e4802 + assign4630_e4807);
        let assign4630_e4809: f64 = (assign4630_e4808).sqrt();
        (assign4630_e4809, (((var_dv__blk306_dn0 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn0)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_dn1 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn1)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_dn2 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn2)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_dn3 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn3)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_dn4 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn4)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_dn5 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_dn5)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_db0 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_db0)) / (2.0 * assign4630_e4809)), (((var_dv__blk306_db1 * var_dv__blk306) + (var_dv__blk306 * var_dv__blk306_db1)) / (2.0 * assign4630_e4809)),)
    } else {
        (var_mv__blk307, var_mv__blk307_dn0, var_mv__blk307_dn1, var_mv__blk307_dn2, var_mv__blk307_dn3, var_mv__blk307_dn4, var_mv__blk307_dn5, var_mv__blk307_db0, var_mv__blk307_db1,)
    }
};
        var_mv__blk307 = assign4630_e4811;
        var_mv__blk307_dn0 = assign4630_e4811_d_n0;
        var_mv__blk307_dn1 = assign4630_e4811_d_n1;
        var_mv__blk307_dn2 = assign4630_e4811_d_n2;
        var_mv__blk307_dn3 = assign4630_e4811_d_n3;
        var_mv__blk307_dn4 = assign4630_e4811_d_n4;
        var_mv__blk307_dn5 = assign4630_e4811_d_n5;
        var_mv__blk307_db0 = assign4630_e4811_d_b0;
        var_mv__blk307_db1 = assign4630_e4811_d_b1;
        var_mv__blk307_rv = 0.0;
        var_mv__blk307_rdn0 = 0.0;
        var_mv__blk307_rdn1 = 0.0;
        var_mv__blk307_rdn2 = 0.0;
        var_mv__blk307_rdn3 = 0.0;
        var_mv__blk307_rdn4 = 0.0;
        var_mv__blk307_rdn5 = 0.0;
        var_mv__blk307_rdb0 = 0.0;
        var_mv__blk307_rdb1 = 0.0;

        let (assign4640_e4826, assign4640_e4826_d_n0, assign4640_e4826_d_n1, assign4640_e4826_d_n2, assign4640_e4826_d_n3, assign4640_e4826_d_n4, assign4640_e4826_d_n5, assign4640_e4826_d_b0, assign4640_e4826_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4640_e4821: f64 = (var_dv__blk306 - var_mv__blk307);
        let assign4640_e4822: f64 = (0.5 * assign4640_e4821);
        let assign4640_e4824: f64 = (assign4640_e4822 - var_dv0__blk299);
        (assign4640_e4824, ((0.5 * (var_dv__blk306_dn0 - var_mv__blk307_dn0)) - var_dv0__blk299_dn0), ((0.5 * (var_dv__blk306_dn1 - var_mv__blk307_dn1)) - var_dv0__blk299_dn1), ((0.5 * (var_dv__blk306_dn2 - var_mv__blk307_dn2)) - var_dv0__blk299_dn2), ((0.5 * (var_dv__blk306_dn3 - var_mv__blk307_dn3)) - var_dv0__blk299_dn3), ((0.5 * (var_dv__blk306_dn4 - var_mv__blk307_dn4)) - var_dv0__blk299_dn4), ((0.5 * (var_dv__blk306_dn5 - var_mv__blk307_dn5)) - var_dv0__blk299_dn5), ((0.5 * (var_dv__blk306_db0 - var_mv__blk307_db0)) - var_dv0__blk299_db0), ((0.5 * (var_dv__blk306_db1 - var_mv__blk307_db1)) - var_dv0__blk299_db1),)
    } else {
        (var_vl__blk308, var_vl__blk308_dn0, var_vl__blk308_dn1, var_vl__blk308_dn2, var_vl__blk308_dn3, var_vl__blk308_dn4, var_vl__blk308_dn5, var_vl__blk308_db0, var_vl__blk308_db1,)
    }
};
        var_vl__blk308 = assign4640_e4826;
        var_vl__blk308_dn0 = assign4640_e4826_d_n0;
        var_vl__blk308_dn1 = assign4640_e4826_d_n1;
        var_vl__blk308_dn2 = assign4640_e4826_d_n2;
        var_vl__blk308_dn3 = assign4640_e4826_d_n3;
        var_vl__blk308_dn4 = assign4640_e4826_d_n4;
        var_vl__blk308_dn5 = assign4640_e4826_d_n5;
        var_vl__blk308_db0 = assign4640_e4826_d_b0;
        var_vl__blk308_db1 = assign4640_e4826_d_b1;
        var_vl__blk308_rv = 0.0;
        var_vl__blk308_rdn0 = 0.0;
        var_vl__blk308_rdn1 = 0.0;
        var_vl__blk308_rdn2 = 0.0;
        var_vl__blk308_rdn3 = 0.0;
        var_vl__blk308_rdn4 = 0.0;
        var_vl__blk308_rdn5 = 0.0;
        var_vl__blk308_rdb0 = 0.0;
        var_vl__blk308_rdb1 = 0.0;

        let (assign4650_e4850, assign4650_e4850_d_n0, assign4650_e4850_d_n1, assign4650_e4850_d_n2, assign4650_e4850_d_n3, assign4650_e4850_d_n4, assign4650_e4850_d_n5, assign4650_e4850_d_b0, assign4650_e4850_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4650_e4834: f64 = (-var_pp_t);
        let assign4650_e4838: f64 = (var_vl__blk308 / var_pp_t);
        let assign4650_e4839: f64 = (1.0 - assign4650_e4838);
        let assign4650_e4842: f64 = (1.0 - p.p81);
        let assign4650_e4843: f64 = (assign4650_e4839).powf(assign4650_e4842);
        let assign4650_e4844: f64 = (assign4650_e4834 * assign4650_e4843);
        let assign4650_e4847: f64 = (1.0 - p.p81);
        let assign4650_e4848: f64 = (assign4650_e4844 / assign4650_e4847);
        (assign4650_e4848, ((((-var_pp_t_dn0) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn0 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn0)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn0 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn0)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_dn1) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn1 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn1)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn1 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn1)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_dn2) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn2 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn2)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn2 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn2)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_dn3) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn3 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn3)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn3 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn3)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_dn4) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn4 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn4)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn4 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn4)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_dn5) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_dn5 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn5)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_dn5 * var_pp_t) - (var_vl__blk308 * var_pp_t_dn5)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_db0) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_db0 * var_pp_t) - (var_vl__blk308 * var_pp_t_db0)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_db0 * var_pp_t) - (var_vl__blk308 * var_pp_t_db0)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847), ((((-var_pp_t_db1) * assign4650_e4843) + (assign4650_e4834 * if 0.0 == 0.0 && ((assign4650_e4842) as f64).is_finite() && ((assign4650_e4842) as f64).fract() == 0.0 { if assign4650_e4842 == 0.0 { 0.0 } else { (assign4650_e4842 * ((assign4650_e4839).powf(assign4650_e4842 - 1.0) * (-(((var_vl__blk308_db1 * var_pp_t) - (var_vl__blk308 * var_pp_t_db1)) / (var_pp_t * var_pp_t))))) } } else { (assign4650_e4843 * (assign4650_e4842 * ((-(((var_vl__blk308_db1 * var_pp_t) - (var_vl__blk308 * var_pp_t_db1)) / (var_pp_t * var_pp_t))) / assign4650_e4839))) })) / assign4650_e4847),)
    } else {
        (var_qlo__blk302, var_qlo__blk302_dn0, var_qlo__blk302_dn1, var_qlo__blk302_dn2, var_qlo__blk302_dn3, var_qlo__blk302_dn4, var_qlo__blk302_dn5, var_qlo__blk302_db0, var_qlo__blk302_db1,)
    }
};
        var_qlo__blk302 = assign4650_e4850;
        var_qlo__blk302_dn0 = assign4650_e4850_d_n0;
        var_qlo__blk302_dn1 = assign4650_e4850_d_n1;
        var_qlo__blk302_dn2 = assign4650_e4850_d_n2;
        var_qlo__blk302_dn3 = assign4650_e4850_d_n3;
        var_qlo__blk302_dn4 = assign4650_e4850_d_n4;
        var_qlo__blk302_dn5 = assign4650_e4850_d_n5;
        var_qlo__blk302_db0 = assign4650_e4850_d_b0;
        var_qlo__blk302_db1 = assign4650_e4850_d_b1;
        var_qlo__blk302_rv = 0.0;
        var_qlo__blk302_rdn0 = 0.0;
        var_qlo__blk302_rdn1 = 0.0;
        var_qlo__blk302_rdn2 = 0.0;
        var_qlo__blk302_rdn3 = 0.0;
        var_qlo__blk302_rdn4 = 0.0;
        var_qlo__blk302_rdn5 = 0.0;
        var_qlo__blk302_rdb0 = 0.0;
        var_qlo__blk302_rdb1 = 0.0;

        let (assign4660_e4890, assign4660_e4890_d_n0, assign4660_e4890_d_n1, assign4660_e4890_d_n2, assign4660_e4890_d_n3, assign4660_e4890_d_n4, assign4660_e4890_d_n5, assign4660_e4890_d_b0, assign4660_e4890_d_b1,) = {
    if (((var_guard280 != 0.0) && (var_guard298 != 0.0)) && (var_guard309 == 0.0)) {
        let assign4660_e4860: f64 = (1.0 - p.p68);
        let assign4660_e4862: f64 = (-p.p81);
        let assign4660_e4863: f64 = (assign4660_e4860).powf(assign4660_e4862);
        let assign4660_e4866: f64 = (var_vcl - var_vl__blk308);
        let assign4660_e4868: f64 = (assign4660_e4866 + var_vl0__blk305);
        let assign4660_e4869: f64 = (assign4660_e4863 * assign4660_e4868);
        let assign4660_e4873: f64 = (0.5 * p.p81);
        let assign4660_e4876: f64 = (var_vcl - var_vl__blk308);
        let assign4660_e4878: f64 = (assign4660_e4876 + var_vl0__blk305);
        let assign4660_e4879: f64 = (assign4660_e4873 * assign4660_e4878);
        let assign4660_e4883: f64 = (1.0 - p.p68);
        let assign4660_e4884: f64 = (var_pp_t * assign4660_e4883);
        let assign4660_e4885: f64 = (assign4660_e4879 / assign4660_e4884);
        let assign4660_e4886: f64 = (1.0 + assign4660_e4885);
        let assign4660_e4887: f64 = (assign4660_e4869 * assign4660_e4886);
        let assign4660_e4888: f64 = (var_qlo__blk302 + assign4660_e4887);
        (assign4660_e4888, (var_qlo__blk302_dn0 + (((assign4660_e4863 * ((var_vcl_dn0 - var_vl__blk308_dn0) + var_vl0__blk305_dn0)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn0 - var_vl__blk308_dn0) + var_vl0__blk305_dn0)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn0 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_dn1 + (((assign4660_e4863 * ((var_vcl_dn1 - var_vl__blk308_dn1) + var_vl0__blk305_dn1)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn1 - var_vl__blk308_dn1) + var_vl0__blk305_dn1)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn1 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_dn2 + (((assign4660_e4863 * ((var_vcl_dn2 - var_vl__blk308_dn2) + var_vl0__blk305_dn2)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn2 - var_vl__blk308_dn2) + var_vl0__blk305_dn2)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn2 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_dn3 + (((assign4660_e4863 * ((var_vcl_dn3 - var_vl__blk308_dn3) + var_vl0__blk305_dn3)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn3 - var_vl__blk308_dn3) + var_vl0__blk305_dn3)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn3 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_dn4 + (((assign4660_e4863 * ((var_vcl_dn4 - var_vl__blk308_dn4) + var_vl0__blk305_dn4)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn4 - var_vl__blk308_dn4) + var_vl0__blk305_dn4)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn4 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_dn5 + (((assign4660_e4863 * ((var_vcl_dn5 - var_vl__blk308_dn5) + var_vl0__blk305_dn5)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_dn5 - var_vl__blk308_dn5) + var_vl0__blk305_dn5)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_dn5 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_db0 + (((assign4660_e4863 * ((var_vcl_db0 - var_vl__blk308_db0) + var_vl0__blk305_db0)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_db0 - var_vl__blk308_db0) + var_vl0__blk305_db0)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_db0 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))), (var_qlo__blk302_db1 + (((assign4660_e4863 * ((var_vcl_db1 - var_vl__blk308_db1) + var_vl0__blk305_db1)) * assign4660_e4886) + (assign4660_e4869 * ((((assign4660_e4873 * ((var_vcl_db1 - var_vl__blk308_db1) + var_vl0__blk305_db1)) * assign4660_e4884) - (assign4660_e4879 * (var_pp_t_db1 * assign4660_e4883))) / (assign4660_e4884 * assign4660_e4884))))),)
    } else {
        (var_argp__blk284, var_argp__blk284_dn0, var_argp__blk284_dn1, var_argp__blk284_dn2, var_argp__blk284_dn3, var_argp__blk284_dn4, var_argp__blk284_dn5, var_argp__blk284_db0, var_argp__blk284_db1,)
    }
};
        var_argp__blk284 = assign4660_e4890;
        var_argp__blk284_dn0 = assign4660_e4890_d_n0;
        var_argp__blk284_dn1 = assign4660_e4890_d_n1;
        var_argp__blk284_dn2 = assign4660_e4890_d_n2;
        var_argp__blk284_dn3 = assign4660_e4890_d_n3;
        var_argp__blk284_dn4 = assign4660_e4890_d_n4;
        var_argp__blk284_dn5 = assign4660_e4890_d_n5;
        var_argp__blk284_db0 = assign4660_e4890_d_b0;
        var_argp__blk284_db1 = assign4660_e4890_d_b1;
        var_argp__blk284_rv = 0.0;
        var_argp__blk284_rdn0 = 0.0;
        var_argp__blk284_rdn1 = 0.0;
        var_argp__blk284_rdn2 = 0.0;
        var_argp__blk284_rdn3 = 0.0;
        var_argp__blk284_rdn4 = 0.0;
        var_argp__blk284_rdn5 = 0.0;
        var_argp__blk284_rdb0 = 0.0;
        var_argp__blk284_rdb1 = 0.0;

        let (assign4670_e4897, assign4670_e4897_d_n0, assign4670_e4897_d_n1, assign4670_e4897_d_n2, assign4670_e4897_d_n3, assign4670_e4897_d_n4, assign4670_e4897_d_n5, assign4670_e4897_d_b0, assign4670_e4897_d_b1,) = {
    if ((var_guard280 != 0.0) && (var_guard298 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_argp__blk284, var_argp__blk284_dn0, var_argp__blk284_dn1, var_argp__blk284_dn2, var_argp__blk284_dn3, var_argp__blk284_dn4, var_argp__blk284_dn5, var_argp__blk284_db0, var_argp__blk284_db1,)
    }
};
        var_argp__blk284 = assign4670_e4897;
        var_argp__blk284_dn0 = assign4670_e4897_d_n0;
        var_argp__blk284_dn1 = assign4670_e4897_d_n1;
        var_argp__blk284_dn2 = assign4670_e4897_d_n2;
        var_argp__blk284_dn3 = assign4670_e4897_d_n3;
        var_argp__blk284_dn4 = assign4670_e4897_d_n4;
        var_argp__blk284_dn5 = assign4670_e4897_d_n5;
        var_argp__blk284_db0 = assign4670_e4897_d_b0;
        var_argp__blk284_db1 = assign4670_e4897_d_b1;
        var_argp__blk284_rv = 0.0;
        var_argp__blk284_rdn0 = 0.0;
        var_argp__blk284_rdn1 = 0.0;
        var_argp__blk284_rdn2 = 0.0;
        var_argp__blk284_rdn3 = 0.0;
        var_argp__blk284_rdn4 = 0.0;
        var_argp__blk284_rdn5 = 0.0;
        var_argp__blk284_rdb0 = 0.0;
        var_argp__blk284_rdb1 = 0.0;

        *var_argp__blk284_slot = var_argp__blk284;
        *var_argp__blk284_db0_slot = var_argp__blk284_db0;
        *var_argp__blk284_db1_slot = var_argp__blk284_db1;
        *var_argp__blk284_dn0_slot = var_argp__blk284_dn0;
        *var_argp__blk284_dn1_slot = var_argp__blk284_dn1;
        *var_argp__blk284_dn2_slot = var_argp__blk284_dn2;
        *var_argp__blk284_dn3_slot = var_argp__blk284_dn3;
        *var_argp__blk284_dn4_slot = var_argp__blk284_dn4;
        *var_argp__blk284_dn5_slot = var_argp__blk284_dn5;
        *var_argp__blk284_rdb0_slot = var_argp__blk284_rdb0;
        *var_argp__blk284_rdb1_slot = var_argp__blk284_rdb1;
        *var_argp__blk284_rdn0_slot = var_argp__blk284_rdn0;
        *var_argp__blk284_rdn1_slot = var_argp__blk284_rdn1;
        *var_argp__blk284_rdn2_slot = var_argp__blk284_rdn2;
        *var_argp__blk284_rdn3_slot = var_argp__blk284_rdn3;
        *var_argp__blk284_rdn4_slot = var_argp__blk284_rdn4;
        *var_argp__blk284_rdn5_slot = var_argp__blk284_rdn5;
        *var_argp__blk284_rv_slot = var_argp__blk284_rv;
        *var_dv__blk306_slot = var_dv__blk306;
        *var_dv__blk306_db0_slot = var_dv__blk306_db0;
        *var_dv__blk306_db1_slot = var_dv__blk306_db1;
        *var_dv__blk306_dn0_slot = var_dv__blk306_dn0;
        *var_dv__blk306_dn1_slot = var_dv__blk306_dn1;
        *var_dv__blk306_dn2_slot = var_dv__blk306_dn2;
        *var_dv__blk306_dn3_slot = var_dv__blk306_dn3;
        *var_dv__blk306_dn4_slot = var_dv__blk306_dn4;
        *var_dv__blk306_dn5_slot = var_dv__blk306_dn5;
        *var_dv__blk306_rdb0_slot = var_dv__blk306_rdb0;
        *var_dv__blk306_rdb1_slot = var_dv__blk306_rdb1;
        *var_dv__blk306_rdn0_slot = var_dv__blk306_rdn0;
        *var_dv__blk306_rdn1_slot = var_dv__blk306_rdn1;
        *var_dv__blk306_rdn2_slot = var_dv__blk306_rdn2;
        *var_dv__blk306_rdn3_slot = var_dv__blk306_rdn3;
        *var_dv__blk306_rdn4_slot = var_dv__blk306_rdn4;
        *var_dv__blk306_rdn5_slot = var_dv__blk306_rdn5;
        *var_dv__blk306_rv_slot = var_dv__blk306_rv;
        *var_dvh__blk300_slot = var_dvh__blk300;
        *var_dvh__blk300_db0_slot = var_dvh__blk300_db0;
        *var_dvh__blk300_db1_slot = var_dvh__blk300_db1;
        *var_dvh__blk300_dn0_slot = var_dvh__blk300_dn0;
        *var_dvh__blk300_dn1_slot = var_dvh__blk300_dn1;
        *var_dvh__blk300_dn2_slot = var_dvh__blk300_dn2;
        *var_dvh__blk300_dn3_slot = var_dvh__blk300_dn3;
        *var_dvh__blk300_dn4_slot = var_dvh__blk300_dn4;
        *var_dvh__blk300_dn5_slot = var_dvh__blk300_dn5;
        *var_dvh__blk300_rdb0_slot = var_dvh__blk300_rdb0;
        *var_dvh__blk300_rdb1_slot = var_dvh__blk300_rdb1;
        *var_dvh__blk300_rdn0_slot = var_dvh__blk300_rdn0;
        *var_dvh__blk300_rdn1_slot = var_dvh__blk300_rdn1;
        *var_dvh__blk300_rdn2_slot = var_dvh__blk300_rdn2;
        *var_dvh__blk300_rdn3_slot = var_dvh__blk300_rdn3;
        *var_dvh__blk300_rdn4_slot = var_dvh__blk300_rdn4;
        *var_dvh__blk300_rdn5_slot = var_dvh__blk300_rdn5;
        *var_dvh__blk300_rv_slot = var_dvh__blk300_rv;
        *var_guard309_slot = var_guard309;
        *var_guard309_db0_slot = var_guard309_db0;
        *var_guard309_db1_slot = var_guard309_db1;
        *var_guard309_dn0_slot = var_guard309_dn0;
        *var_guard309_dn1_slot = var_guard309_dn1;
        *var_guard309_dn2_slot = var_guard309_dn2;
        *var_guard309_dn3_slot = var_guard309_dn3;
        *var_guard309_dn4_slot = var_guard309_dn4;
        *var_guard309_dn5_slot = var_guard309_dn5;
        *var_guard309_rdb0_slot = var_guard309_rdb0;
        *var_guard309_rdb1_slot = var_guard309_rdb1;
        *var_guard309_rdn0_slot = var_guard309_rdn0;
        *var_guard309_rdn1_slot = var_guard309_rdn1;
        *var_guard309_rdn2_slot = var_guard309_rdn2;
        *var_guard309_rdn3_slot = var_guard309_rdn3;
        *var_guard309_rdn4_slot = var_guard309_rdn4;
        *var_guard309_rdn5_slot = var_guard309_rdn5;
        *var_guard309_rv_slot = var_guard309_rv;
        *var_guard310_slot = var_guard310;
        *var_guard310_db0_slot = var_guard310_db0;
        *var_guard310_db1_slot = var_guard310_db1;
        *var_guard310_dn0_slot = var_guard310_dn0;
        *var_guard310_dn1_slot = var_guard310_dn1;
        *var_guard310_dn2_slot = var_guard310_dn2;
        *var_guard310_dn3_slot = var_guard310_dn3;
        *var_guard310_dn4_slot = var_guard310_dn4;
        *var_guard310_dn5_slot = var_guard310_dn5;
        *var_guard310_rdb0_slot = var_guard310_rdb0;
        *var_guard310_rdb1_slot = var_guard310_rdb1;
        *var_guard310_rdn0_slot = var_guard310_rdn0;
        *var_guard310_rdn1_slot = var_guard310_rdn1;
        *var_guard310_rdn2_slot = var_guard310_rdn2;
        *var_guard310_rdn3_slot = var_guard310_rdn3;
        *var_guard310_rdn4_slot = var_guard310_rdn4;
        *var_guard310_rdn5_slot = var_guard310_rdn5;
        *var_guard310_rv_slot = var_guard310_rv;
        *var_mv0__blk304_slot = var_mv0__blk304;
        *var_mv0__blk304_db0_slot = var_mv0__blk304_db0;
        *var_mv0__blk304_db1_slot = var_mv0__blk304_db1;
        *var_mv0__blk304_dn0_slot = var_mv0__blk304_dn0;
        *var_mv0__blk304_dn1_slot = var_mv0__blk304_dn1;
        *var_mv0__blk304_dn2_slot = var_mv0__blk304_dn2;
        *var_mv0__blk304_dn3_slot = var_mv0__blk304_dn3;
        *var_mv0__blk304_dn4_slot = var_mv0__blk304_dn4;
        *var_mv0__blk304_dn5_slot = var_mv0__blk304_dn5;
        *var_mv0__blk304_rdb0_slot = var_mv0__blk304_rdb0;
        *var_mv0__blk304_rdb1_slot = var_mv0__blk304_rdb1;
        *var_mv0__blk304_rdn0_slot = var_mv0__blk304_rdn0;
        *var_mv0__blk304_rdn1_slot = var_mv0__blk304_rdn1;
        *var_mv0__blk304_rdn2_slot = var_mv0__blk304_rdn2;
        *var_mv0__blk304_rdn3_slot = var_mv0__blk304_rdn3;
        *var_mv0__blk304_rdn4_slot = var_mv0__blk304_rdn4;
        *var_mv0__blk304_rdn5_slot = var_mv0__blk304_rdn5;
        *var_mv0__blk304_rv_slot = var_mv0__blk304_rv;
        *var_mv__blk307_slot = var_mv__blk307;
        *var_mv__blk307_db0_slot = var_mv__blk307_db0;
        *var_mv__blk307_db1_slot = var_mv__blk307_db1;
        *var_mv__blk307_dn0_slot = var_mv__blk307_dn0;
        *var_mv__blk307_dn1_slot = var_mv__blk307_dn1;
        *var_mv__blk307_dn2_slot = var_mv__blk307_dn2;
        *var_mv__blk307_dn3_slot = var_mv__blk307_dn3;
        *var_mv__blk307_dn4_slot = var_mv__blk307_dn4;
        *var_mv__blk307_dn5_slot = var_mv__blk307_dn5;
        *var_mv__blk307_rdb0_slot = var_mv__blk307_rdb0;
        *var_mv__blk307_rdb1_slot = var_mv__blk307_rdb1;
        *var_mv__blk307_rdn0_slot = var_mv__blk307_rdn0;
        *var_mv__blk307_rdn1_slot = var_mv__blk307_rdn1;
        *var_mv__blk307_rdn2_slot = var_mv__blk307_rdn2;
        *var_mv__blk307_rdn3_slot = var_mv__blk307_rdn3;
        *var_mv__blk307_rdn4_slot = var_mv__blk307_rdn4;
        *var_mv__blk307_rdn5_slot = var_mv__blk307_rdn5;
        *var_mv__blk307_rv_slot = var_mv__blk307_rv;
        *var_pwq__blk301_slot = var_pwq__blk301;
        *var_pwq__blk301_db0_slot = var_pwq__blk301_db0;
        *var_pwq__blk301_db1_slot = var_pwq__blk301_db1;
        *var_pwq__blk301_dn0_slot = var_pwq__blk301_dn0;
        *var_pwq__blk301_dn1_slot = var_pwq__blk301_dn1;
        *var_pwq__blk301_dn2_slot = var_pwq__blk301_dn2;
        *var_pwq__blk301_dn3_slot = var_pwq__blk301_dn3;
        *var_pwq__blk301_dn4_slot = var_pwq__blk301_dn4;
        *var_pwq__blk301_dn5_slot = var_pwq__blk301_dn5;
        *var_pwq__blk301_rdb0_slot = var_pwq__blk301_rdb0;
        *var_pwq__blk301_rdb1_slot = var_pwq__blk301_rdb1;
        *var_pwq__blk301_rdn0_slot = var_pwq__blk301_rdn0;
        *var_pwq__blk301_rdn1_slot = var_pwq__blk301_rdn1;
        *var_pwq__blk301_rdn2_slot = var_pwq__blk301_rdn2;
        *var_pwq__blk301_rdn3_slot = var_pwq__blk301_rdn3;
        *var_pwq__blk301_rdn4_slot = var_pwq__blk301_rdn4;
        *var_pwq__blk301_rdn5_slot = var_pwq__blk301_rdn5;
        *var_pwq__blk301_rv_slot = var_pwq__blk301_rv;
        *var_qhi__blk303_slot = var_qhi__blk303;
        *var_qhi__blk303_db0_slot = var_qhi__blk303_db0;
        *var_qhi__blk303_db1_slot = var_qhi__blk303_db1;
        *var_qhi__blk303_dn0_slot = var_qhi__blk303_dn0;
        *var_qhi__blk303_dn1_slot = var_qhi__blk303_dn1;
        *var_qhi__blk303_dn2_slot = var_qhi__blk303_dn2;
        *var_qhi__blk303_dn3_slot = var_qhi__blk303_dn3;
        *var_qhi__blk303_dn4_slot = var_qhi__blk303_dn4;
        *var_qhi__blk303_dn5_slot = var_qhi__blk303_dn5;
        *var_qhi__blk303_rdb0_slot = var_qhi__blk303_rdb0;
        *var_qhi__blk303_rdb1_slot = var_qhi__blk303_rdb1;
        *var_qhi__blk303_rdn0_slot = var_qhi__blk303_rdn0;
        *var_qhi__blk303_rdn1_slot = var_qhi__blk303_rdn1;
        *var_qhi__blk303_rdn2_slot = var_qhi__blk303_rdn2;
        *var_qhi__blk303_rdn3_slot = var_qhi__blk303_rdn3;
        *var_qhi__blk303_rdn4_slot = var_qhi__blk303_rdn4;
        *var_qhi__blk303_rdn5_slot = var_qhi__blk303_rdn5;
        *var_qhi__blk303_rv_slot = var_qhi__blk303_rv;
        *var_qlo__blk302_slot = var_qlo__blk302;
        *var_qlo__blk302_db0_slot = var_qlo__blk302_db0;
        *var_qlo__blk302_db1_slot = var_qlo__blk302_db1;
        *var_qlo__blk302_dn0_slot = var_qlo__blk302_dn0;
        *var_qlo__blk302_dn1_slot = var_qlo__blk302_dn1;
        *var_qlo__blk302_dn2_slot = var_qlo__blk302_dn2;
        *var_qlo__blk302_dn3_slot = var_qlo__blk302_dn3;
        *var_qlo__blk302_dn4_slot = var_qlo__blk302_dn4;
        *var_qlo__blk302_dn5_slot = var_qlo__blk302_dn5;
        *var_qlo__blk302_rdb0_slot = var_qlo__blk302_rdb0;
        *var_qlo__blk302_rdb1_slot = var_qlo__blk302_rdb1;
        *var_qlo__blk302_rdn0_slot = var_qlo__blk302_rdn0;
        *var_qlo__blk302_rdn1_slot = var_qlo__blk302_rdn1;
        *var_qlo__blk302_rdn2_slot = var_qlo__blk302_rdn2;
        *var_qlo__blk302_rdn3_slot = var_qlo__blk302_rdn3;
        *var_qlo__blk302_rdn4_slot = var_qlo__blk302_rdn4;
        *var_qlo__blk302_rdn5_slot = var_qlo__blk302_rdn5;
        *var_qlo__blk302_rv_slot = var_qlo__blk302_rv;
        *var_vl0__blk305_slot = var_vl0__blk305;
        *var_vl0__blk305_db0_slot = var_vl0__blk305_db0;
        *var_vl0__blk305_db1_slot = var_vl0__blk305_db1;
        *var_vl0__blk305_dn0_slot = var_vl0__blk305_dn0;
        *var_vl0__blk305_dn1_slot = var_vl0__blk305_dn1;
        *var_vl0__blk305_dn2_slot = var_vl0__blk305_dn2;
        *var_vl0__blk305_dn3_slot = var_vl0__blk305_dn3;
        *var_vl0__blk305_dn4_slot = var_vl0__blk305_dn4;
        *var_vl0__blk305_dn5_slot = var_vl0__blk305_dn5;
        *var_vl0__blk305_rdb0_slot = var_vl0__blk305_rdb0;
        *var_vl0__blk305_rdb1_slot = var_vl0__blk305_rdb1;
        *var_vl0__blk305_rdn0_slot = var_vl0__blk305_rdn0;
        *var_vl0__blk305_rdn1_slot = var_vl0__blk305_rdn1;
        *var_vl0__blk305_rdn2_slot = var_vl0__blk305_rdn2;
        *var_vl0__blk305_rdn3_slot = var_vl0__blk305_rdn3;
        *var_vl0__blk305_rdn4_slot = var_vl0__blk305_rdn4;
        *var_vl0__blk305_rdn5_slot = var_vl0__blk305_rdn5;
        *var_vl0__blk305_rv_slot = var_vl0__blk305_rv;
        *var_vl__blk308_slot = var_vl__blk308;
        *var_vl__blk308_db0_slot = var_vl__blk308_db0;
        *var_vl__blk308_db1_slot = var_vl__blk308_db1;
        *var_vl__blk308_dn0_slot = var_vl__blk308_dn0;
        *var_vl__blk308_dn1_slot = var_vl__blk308_dn1;
        *var_vl__blk308_dn2_slot = var_vl__blk308_dn2;
        *var_vl__blk308_dn3_slot = var_vl__blk308_dn3;
        *var_vl__blk308_dn4_slot = var_vl__blk308_dn4;
        *var_vl__blk308_dn5_slot = var_vl__blk308_dn5;
        *var_vl__blk308_rdb0_slot = var_vl__blk308_rdb0;
        *var_vl__blk308_rdb1_slot = var_vl__blk308_rdb1;
        *var_vl__blk308_rdn0_slot = var_vl__blk308_rdn0;
        *var_vl__blk308_rdn1_slot = var_vl__blk308_rdn1;
        *var_vl__blk308_rdn2_slot = var_vl__blk308_rdn2;
        *var_vl__blk308_rdn3_slot = var_vl__blk308_rdn3;
        *var_vl__blk308_rdn4_slot = var_vl__blk308_rdn4;
        *var_vl__blk308_rdn5_slot = var_vl__blk308_rdn5;
        *var_vl__blk308_rv_slot = var_vl__blk308_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_acja__blk281: f64,
        var_acja__blk281_db0: f64,
        var_acja__blk281_db1: f64,
        var_acja__blk281_dn0: f64,
        var_acja__blk281_dn1: f64,
        var_acja__blk281_dn2: f64,
        var_acja__blk281_dn3: f64,
        var_acja__blk281_dn4: f64,
        var_acja__blk281_dn5: f64,
        var_arga__blk283: f64,
        var_arga__blk283_db0: f64,
        var_arga__blk283_db1: f64,
        var_arga__blk283_dn0: f64,
        var_arga__blk283_dn1: f64,
        var_arga__blk283_dn2: f64,
        var_arga__blk283_dn3: f64,
        var_arga__blk283_dn4: f64,
        var_arga__blk283_dn5: f64,
        var_argp__blk284: f64,
        var_argp__blk284_db0: f64,
        var_argp__blk284_db1: f64,
        var_argp__blk284_dn0: f64,
        var_argp__blk284_dn1: f64,
        var_argp__blk284_dn2: f64,
        var_argp__blk284_dn3: f64,
        var_argp__blk284_dn4: f64,
        var_argp__blk284_dn5: f64,
        var_cf1: f64,
        var_cf1_db0: f64,
        var_cf1_db1: f64,
        var_cf1_dn0: f64,
        var_cf1_dn1: f64,
        var_cf1_dn2: f64,
        var_cf1_dn3: f64,
        var_cf1_dn4: f64,
        var_cf1_dn5: f64,
        var_cf2: f64,
        var_cf2_db0: f64,
        var_cf2_db1: f64,
        var_cf2_dn0: f64,
        var_cf2_dn1: f64,
        var_cf2_dn2: f64,
        var_cf2_dn3: f64,
        var_cf2_dn4: f64,
        var_cf2_dn5: f64,
        var_dt_et: f64,
        var_dt_et_db0: f64,
        var_dt_et_db1: f64,
        var_dt_et_dn0: f64,
        var_dt_et_dn1: f64,
        var_dt_et_dn2: f64,
        var_dt_et_dn3: f64,
        var_dt_et_dn4: f64,
        var_dt_et_dn5: f64,
        var_guard280: f64,
        var_l_um: f64,
        var_l_um_db0: f64,
        var_l_um_db1: f64,
        var_l_um_dn0: f64,
        var_l_um_dn1: f64,
        var_l_um_dn2: f64,
        var_l_um_dn3: f64,
        var_l_um_dn4: f64,
        var_l_um_dn5: f64,
        var_leff_um: f64,
        var_leff_um_db0: f64,
        var_leff_um_db1: f64,
        var_leff_um_dn0: f64,
        var_leff_um_dn1: f64,
        var_leff_um_dn2: f64,
        var_leff_um_dn3: f64,
        var_leff_um_dn4: f64,
        var_leff_um_dn5: f64,
        var_pcjp__blk282: f64,
        var_pcjp__blk282_db0: f64,
        var_pcjp__blk282_db1: f64,
        var_pcjp__blk282_dn0: f64,
        var_pcjp__blk282_dn1: f64,
        var_pcjp__blk282_dn2: f64,
        var_pcjp__blk282_dn3: f64,
        var_pcjp__blk282_dn4: f64,
        var_pcjp__blk282_dn5: f64,
        var_vc1: f64,
        var_vc1_db0: f64,
        var_vc1_db1: f64,
        var_vc1_dn0: f64,
        var_vc1_dn1: f64,
        var_vc1_dn2: f64,
        var_vc1_dn3: f64,
        var_vc1_dn4: f64,
        var_vc1_dn5: f64,
        var_vc2: f64,
        var_vc2_db0: f64,
        var_vc2_db1: f64,
        var_vc2_dn0: f64,
        var_vc2_dn1: f64,
        var_vc2_dn2: f64,
        var_vc2_dn3: f64,
        var_vc2_dn4: f64,
        var_vc2_dn5: f64,
        var_w_um: f64,
        var_w_um_db0: f64,
        var_w_um_db1: f64,
        var_w_um_dn0: f64,
        var_w_um_dn1: f64,
        var_w_um_dn2: f64,
        var_w_um_dn3: f64,
        var_w_um_dn4: f64,
        var_w_um_dn5: f64,
        var_weff_um: f64,
        var_weff_um_db0: f64,
        var_weff_um_db1: f64,
        var_weff_um_dn0: f64,
        var_weff_um_dn1: f64,
        var_weff_um_dn2: f64,
        var_weff_um_dn3: f64,
        var_weff_um_dn4: f64,
        var_weff_um_dn5: f64,
        var_cth_slot: &mut f64,
        var_cth_db0_slot: &mut f64,
        var_cth_db1_slot: &mut f64,
        var_cth_dn0_slot: &mut f64,
        var_cth_dn1_slot: &mut f64,
        var_cth_dn2_slot: &mut f64,
        var_cth_dn3_slot: &mut f64,
        var_cth_dn4_slot: &mut f64,
        var_cth_dn5_slot: &mut f64,
        var_cth_rdb0_slot: &mut f64,
        var_cth_rdb1_slot: &mut f64,
        var_cth_rdn0_slot: &mut f64,
        var_cth_rdn1_slot: &mut f64,
        var_cth_rdn2_slot: &mut f64,
        var_cth_rdn3_slot: &mut f64,
        var_cth_rdn4_slot: &mut f64,
        var_cth_rdn5_slot: &mut f64,
        var_cth_rv_slot: &mut f64,
        var_len_slot: &mut f64,
        var_len_db0_slot: &mut f64,
        var_len_db1_slot: &mut f64,
        var_len_dn0_slot: &mut f64,
        var_len_dn1_slot: &mut f64,
        var_len_dn2_slot: &mut f64,
        var_len_dn3_slot: &mut f64,
        var_len_dn4_slot: &mut f64,
        var_len_dn5_slot: &mut f64,
        var_len_rdb0_slot: &mut f64,
        var_len_rdb1_slot: &mut f64,
        var_len_rdn0_slot: &mut f64,
        var_len_rdn1_slot: &mut f64,
        var_len_rdn2_slot: &mut f64,
        var_len_rdn3_slot: &mut f64,
        var_len_rdn4_slot: &mut f64,
        var_len_rdn5_slot: &mut f64,
        var_len_rv_slot: &mut f64,
        var_qcp1_slot: &mut f64,
        var_qcp1_db0_slot: &mut f64,
        var_qcp1_db1_slot: &mut f64,
        var_qcp1_dn0_slot: &mut f64,
        var_qcp1_dn1_slot: &mut f64,
        var_qcp1_dn2_slot: &mut f64,
        var_qcp1_dn3_slot: &mut f64,
        var_qcp1_dn4_slot: &mut f64,
        var_qcp1_dn5_slot: &mut f64,
        var_qcp1_rdb0_slot: &mut f64,
        var_qcp1_rdb1_slot: &mut f64,
        var_qcp1_rdn0_slot: &mut f64,
        var_qcp1_rdn1_slot: &mut f64,
        var_qcp1_rdn2_slot: &mut f64,
        var_qcp1_rdn3_slot: &mut f64,
        var_qcp1_rdn4_slot: &mut f64,
        var_qcp1_rdn5_slot: &mut f64,
        var_qcp1_rv_slot: &mut f64,
        var_qcp2_slot: &mut f64,
        var_qcp2_db0_slot: &mut f64,
        var_qcp2_db1_slot: &mut f64,
        var_qcp2_dn0_slot: &mut f64,
        var_qcp2_dn1_slot: &mut f64,
        var_qcp2_dn2_slot: &mut f64,
        var_qcp2_dn3_slot: &mut f64,
        var_qcp2_dn4_slot: &mut f64,
        var_qcp2_dn5_slot: &mut f64,
        var_qcp2_rdb0_slot: &mut f64,
        var_qcp2_rdb1_slot: &mut f64,
        var_qcp2_rdn0_slot: &mut f64,
        var_qcp2_rdn1_slot: &mut f64,
        var_qcp2_rdn2_slot: &mut f64,
        var_qcp2_rdn3_slot: &mut f64,
        var_qcp2_rdn4_slot: &mut f64,
        var_qcp2_rdn5_slot: &mut f64,
        var_qcp2_rv_slot: &mut f64,
        var_qcth_slot: &mut f64,
        var_qcth_db0_slot: &mut f64,
        var_qcth_db1_slot: &mut f64,
        var_qcth_dn0_slot: &mut f64,
        var_qcth_dn1_slot: &mut f64,
        var_qcth_dn2_slot: &mut f64,
        var_qcth_dn3_slot: &mut f64,
        var_qcth_dn4_slot: &mut f64,
        var_qcth_dn5_slot: &mut f64,
        var_qcth_rdb0_slot: &mut f64,
        var_qcth_rdb1_slot: &mut f64,
        var_qcth_rdn0_slot: &mut f64,
        var_qcth_rdn1_slot: &mut f64,
        var_qcth_rdn2_slot: &mut f64,
        var_qcth_rdn3_slot: &mut f64,
        var_qcth_rdn4_slot: &mut f64,
        var_qcth_rdn5_slot: &mut f64,
        var_qcth_rv_slot: &mut f64,
        var_wid_slot: &mut f64,
        var_wid_db0_slot: &mut f64,
        var_wid_db1_slot: &mut f64,
        var_wid_dn0_slot: &mut f64,
        var_wid_dn1_slot: &mut f64,
        var_wid_dn2_slot: &mut f64,
        var_wid_dn3_slot: &mut f64,
        var_wid_dn4_slot: &mut f64,
        var_wid_dn5_slot: &mut f64,
        var_wid_rdb0_slot: &mut f64,
        var_wid_rdb1_slot: &mut f64,
        var_wid_rdn0_slot: &mut f64,
        var_wid_rdn1_slot: &mut f64,
        var_wid_rdn2_slot: &mut f64,
        var_wid_rdn3_slot: &mut f64,
        var_wid_rdn4_slot: &mut f64,
        var_wid_rdn5_slot: &mut f64,
        var_wid_rv_slot: &mut f64,
    ) {
        let mut var_cth: f64 = *var_cth_slot;
        let mut var_cth_db0: f64 = *var_cth_db0_slot;
        let mut var_cth_db1: f64 = *var_cth_db1_slot;
        let mut var_cth_dn0: f64 = *var_cth_dn0_slot;
        let mut var_cth_dn1: f64 = *var_cth_dn1_slot;
        let mut var_cth_dn2: f64 = *var_cth_dn2_slot;
        let mut var_cth_dn3: f64 = *var_cth_dn3_slot;
        let mut var_cth_dn4: f64 = *var_cth_dn4_slot;
        let mut var_cth_dn5: f64 = *var_cth_dn5_slot;
        let mut var_cth_rdb0: f64 = *var_cth_rdb0_slot;
        let mut var_cth_rdb1: f64 = *var_cth_rdb1_slot;
        let mut var_cth_rdn0: f64 = *var_cth_rdn0_slot;
        let mut var_cth_rdn1: f64 = *var_cth_rdn1_slot;
        let mut var_cth_rdn2: f64 = *var_cth_rdn2_slot;
        let mut var_cth_rdn3: f64 = *var_cth_rdn3_slot;
        let mut var_cth_rdn4: f64 = *var_cth_rdn4_slot;
        let mut var_cth_rdn5: f64 = *var_cth_rdn5_slot;
        let mut var_cth_rv: f64 = *var_cth_rv_slot;
        let mut var_len: f64 = *var_len_slot;
        let mut var_len_db0: f64 = *var_len_db0_slot;
        let mut var_len_db1: f64 = *var_len_db1_slot;
        let mut var_len_dn0: f64 = *var_len_dn0_slot;
        let mut var_len_dn1: f64 = *var_len_dn1_slot;
        let mut var_len_dn2: f64 = *var_len_dn2_slot;
        let mut var_len_dn3: f64 = *var_len_dn3_slot;
        let mut var_len_dn4: f64 = *var_len_dn4_slot;
        let mut var_len_dn5: f64 = *var_len_dn5_slot;
        let mut var_len_rdb0: f64 = *var_len_rdb0_slot;
        let mut var_len_rdb1: f64 = *var_len_rdb1_slot;
        let mut var_len_rdn0: f64 = *var_len_rdn0_slot;
        let mut var_len_rdn1: f64 = *var_len_rdn1_slot;
        let mut var_len_rdn2: f64 = *var_len_rdn2_slot;
        let mut var_len_rdn3: f64 = *var_len_rdn3_slot;
        let mut var_len_rdn4: f64 = *var_len_rdn4_slot;
        let mut var_len_rdn5: f64 = *var_len_rdn5_slot;
        let mut var_len_rv: f64 = *var_len_rv_slot;
        let mut var_qcp1: f64 = *var_qcp1_slot;
        let mut var_qcp1_db0: f64 = *var_qcp1_db0_slot;
        let mut var_qcp1_db1: f64 = *var_qcp1_db1_slot;
        let mut var_qcp1_dn0: f64 = *var_qcp1_dn0_slot;
        let mut var_qcp1_dn1: f64 = *var_qcp1_dn1_slot;
        let mut var_qcp1_dn2: f64 = *var_qcp1_dn2_slot;
        let mut var_qcp1_dn3: f64 = *var_qcp1_dn3_slot;
        let mut var_qcp1_dn4: f64 = *var_qcp1_dn4_slot;
        let mut var_qcp1_dn5: f64 = *var_qcp1_dn5_slot;
        let mut var_qcp1_rdb0: f64 = *var_qcp1_rdb0_slot;
        let mut var_qcp1_rdb1: f64 = *var_qcp1_rdb1_slot;
        let mut var_qcp1_rdn0: f64 = *var_qcp1_rdn0_slot;
        let mut var_qcp1_rdn1: f64 = *var_qcp1_rdn1_slot;
        let mut var_qcp1_rdn2: f64 = *var_qcp1_rdn2_slot;
        let mut var_qcp1_rdn3: f64 = *var_qcp1_rdn3_slot;
        let mut var_qcp1_rdn4: f64 = *var_qcp1_rdn4_slot;
        let mut var_qcp1_rdn5: f64 = *var_qcp1_rdn5_slot;
        let mut var_qcp1_rv: f64 = *var_qcp1_rv_slot;
        let mut var_qcp2: f64 = *var_qcp2_slot;
        let mut var_qcp2_db0: f64 = *var_qcp2_db0_slot;
        let mut var_qcp2_db1: f64 = *var_qcp2_db1_slot;
        let mut var_qcp2_dn0: f64 = *var_qcp2_dn0_slot;
        let mut var_qcp2_dn1: f64 = *var_qcp2_dn1_slot;
        let mut var_qcp2_dn2: f64 = *var_qcp2_dn2_slot;
        let mut var_qcp2_dn3: f64 = *var_qcp2_dn3_slot;
        let mut var_qcp2_dn4: f64 = *var_qcp2_dn4_slot;
        let mut var_qcp2_dn5: f64 = *var_qcp2_dn5_slot;
        let mut var_qcp2_rdb0: f64 = *var_qcp2_rdb0_slot;
        let mut var_qcp2_rdb1: f64 = *var_qcp2_rdb1_slot;
        let mut var_qcp2_rdn0: f64 = *var_qcp2_rdn0_slot;
        let mut var_qcp2_rdn1: f64 = *var_qcp2_rdn1_slot;
        let mut var_qcp2_rdn2: f64 = *var_qcp2_rdn2_slot;
        let mut var_qcp2_rdn3: f64 = *var_qcp2_rdn3_slot;
        let mut var_qcp2_rdn4: f64 = *var_qcp2_rdn4_slot;
        let mut var_qcp2_rdn5: f64 = *var_qcp2_rdn5_slot;
        let mut var_qcp2_rv: f64 = *var_qcp2_rv_slot;
        let mut var_qcth: f64 = *var_qcth_slot;
        let mut var_qcth_db0: f64 = *var_qcth_db0_slot;
        let mut var_qcth_db1: f64 = *var_qcth_db1_slot;
        let mut var_qcth_dn0: f64 = *var_qcth_dn0_slot;
        let mut var_qcth_dn1: f64 = *var_qcth_dn1_slot;
        let mut var_qcth_dn2: f64 = *var_qcth_dn2_slot;
        let mut var_qcth_dn3: f64 = *var_qcth_dn3_slot;
        let mut var_qcth_dn4: f64 = *var_qcth_dn4_slot;
        let mut var_qcth_dn5: f64 = *var_qcth_dn5_slot;
        let mut var_qcth_rdb0: f64 = *var_qcth_rdb0_slot;
        let mut var_qcth_rdb1: f64 = *var_qcth_rdb1_slot;
        let mut var_qcth_rdn0: f64 = *var_qcth_rdn0_slot;
        let mut var_qcth_rdn1: f64 = *var_qcth_rdn1_slot;
        let mut var_qcth_rdn2: f64 = *var_qcth_rdn2_slot;
        let mut var_qcth_rdn3: f64 = *var_qcth_rdn3_slot;
        let mut var_qcth_rdn4: f64 = *var_qcth_rdn4_slot;
        let mut var_qcth_rdn5: f64 = *var_qcth_rdn5_slot;
        let mut var_qcth_rv: f64 = *var_qcth_rv_slot;
        let mut var_wid: f64 = *var_wid_slot;
        let mut var_wid_db0: f64 = *var_wid_db0_slot;
        let mut var_wid_db1: f64 = *var_wid_db1_slot;
        let mut var_wid_dn0: f64 = *var_wid_dn0_slot;
        let mut var_wid_dn1: f64 = *var_wid_dn1_slot;
        let mut var_wid_dn2: f64 = *var_wid_dn2_slot;
        let mut var_wid_dn3: f64 = *var_wid_dn3_slot;
        let mut var_wid_dn4: f64 = *var_wid_dn4_slot;
        let mut var_wid_dn5: f64 = *var_wid_dn5_slot;
        let mut var_wid_rdb0: f64 = *var_wid_rdb0_slot;
        let mut var_wid_rdb1: f64 = *var_wid_rdb1_slot;
        let mut var_wid_rdn0: f64 = *var_wid_rdn0_slot;
        let mut var_wid_rdn1: f64 = *var_wid_rdn1_slot;
        let mut var_wid_rdn2: f64 = *var_wid_rdn2_slot;
        let mut var_wid_rdn3: f64 = *var_wid_rdn3_slot;
        let mut var_wid_rdn4: f64 = *var_wid_rdn4_slot;
        let mut var_wid_rdn5: f64 = *var_wid_rdn5_slot;
        let mut var_wid_rv: f64 = *var_wid_rv_slot;

        let (assign4680_e4907, assign4680_e4907_d_n0, assign4680_e4907_d_n1, assign4680_e4907_d_n2, assign4680_e4907_d_n3, assign4680_e4907_d_n4, assign4680_e4907_d_n5, assign4680_e4907_d_b0, assign4680_e4907_d_b1,) = {
    if (var_guard280 != 0.0) {
        let assign4680_e4901: f64 = (var_acja__blk281 * var_arga__blk283);
        let assign4680_e4904: f64 = (var_pcjp__blk282 * var_argp__blk284);
        let assign4680_e4905: f64 = (assign4680_e4901 + assign4680_e4904);
        (assign4680_e4905, (((var_acja__blk281_dn0 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn0)) + ((var_pcjp__blk282_dn0 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn0))), (((var_acja__blk281_dn1 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn1)) + ((var_pcjp__blk282_dn1 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn1))), (((var_acja__blk281_dn2 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn2)) + ((var_pcjp__blk282_dn2 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn2))), (((var_acja__blk281_dn3 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn3)) + ((var_pcjp__blk282_dn3 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn3))), (((var_acja__blk281_dn4 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn4)) + ((var_pcjp__blk282_dn4 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn4))), (((var_acja__blk281_dn5 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_dn5)) + ((var_pcjp__blk282_dn5 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_dn5))), (((var_acja__blk281_db0 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_db0)) + ((var_pcjp__blk282_db0 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_db0))), (((var_acja__blk281_db1 * var_arga__blk283) + (var_acja__blk281 * var_arga__blk283_db1)) + ((var_pcjp__blk282_db1 * var_argp__blk284) + (var_pcjp__blk282 * var_argp__blk284_db1))),)
    } else {
        (var_qcp2, var_qcp2_dn0, var_qcp2_dn1, var_qcp2_dn2, var_qcp2_dn3, var_qcp2_dn4, var_qcp2_dn5, var_qcp2_db0, var_qcp2_db1,)
    }
};
        var_qcp2 = assign4680_e4907;
        var_qcp2_dn0 = assign4680_e4907_d_n0;
        var_qcp2_dn1 = assign4680_e4907_d_n1;
        var_qcp2_dn2 = assign4680_e4907_d_n2;
        var_qcp2_dn3 = assign4680_e4907_d_n3;
        var_qcp2_dn4 = assign4680_e4907_d_n4;
        var_qcp2_dn5 = assign4680_e4907_d_n5;
        var_qcp2_db0 = assign4680_e4907_d_b0;
        var_qcp2_db1 = assign4680_e4907_d_b1;
        var_qcp2_rv = 0.0;
        var_qcp2_rdn0 = 0.0;
        var_qcp2_rdn1 = 0.0;
        var_qcp2_rdn2 = 0.0;
        var_qcp2_rdn3 = 0.0;
        var_qcp2_rdn4 = 0.0;
        var_qcp2_rdn5 = 0.0;
        var_qcp2_rdb0 = 0.0;
        var_qcp2_rdb1 = 0.0;

        let (assign4690_e4912, assign4690_e4912_d_n0, assign4690_e4912_d_n1, assign4690_e4912_d_n2, assign4690_e4912_d_n3, assign4690_e4912_d_n4, assign4690_e4912_d_n5, assign4690_e4912_d_b0, assign4690_e4912_d_b1,) = {
    if (var_guard280 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qcp2, var_qcp2_dn0, var_qcp2_dn1, var_qcp2_dn2, var_qcp2_dn3, var_qcp2_dn4, var_qcp2_dn5, var_qcp2_db0, var_qcp2_db1,)
    }
};
        var_qcp2 = assign4690_e4912;
        var_qcp2_dn0 = assign4690_e4912_d_n0;
        var_qcp2_dn1 = assign4690_e4912_d_n1;
        var_qcp2_dn2 = assign4690_e4912_d_n2;
        var_qcp2_dn3 = assign4690_e4912_d_n3;
        var_qcp2_dn4 = assign4690_e4912_d_n4;
        var_qcp2_dn5 = assign4690_e4912_d_n5;
        var_qcp2_db0 = assign4690_e4912_d_b0;
        var_qcp2_db1 = assign4690_e4912_d_b1;
        var_qcp2_rv = 0.0;
        var_qcp2_rdn0 = 0.0;
        var_qcp2_rdn1 = 0.0;
        var_qcp2_rdn2 = 0.0;
        var_qcp2_rdn3 = 0.0;
        var_qcp2_rdn4 = 0.0;
        var_qcp2_rdn5 = 0.0;
        var_qcp2_rdb0 = 0.0;
        var_qcp2_rdb1 = 0.0;

        let assign4700_e4916: f64 = (var_cf1 * var_vc1);
        let assign4700_e4917: f64 = (var_qcp1 + assign4700_e4916);
        var_qcp1 = assign4700_e4917;
        var_qcp1_dn0 = (var_qcp1_dn0 + ((var_cf1_dn0 * var_vc1) + (var_cf1 * var_vc1_dn0)));
        var_qcp1_dn1 = (var_qcp1_dn1 + ((var_cf1_dn1 * var_vc1) + (var_cf1 * var_vc1_dn1)));
        var_qcp1_dn2 = (var_qcp1_dn2 + ((var_cf1_dn2 * var_vc1) + (var_cf1 * var_vc1_dn2)));
        var_qcp1_dn3 = (var_qcp1_dn3 + ((var_cf1_dn3 * var_vc1) + (var_cf1 * var_vc1_dn3)));
        var_qcp1_dn4 = (var_qcp1_dn4 + ((var_cf1_dn4 * var_vc1) + (var_cf1 * var_vc1_dn4)));
        var_qcp1_dn5 = (var_qcp1_dn5 + ((var_cf1_dn5 * var_vc1) + (var_cf1 * var_vc1_dn5)));
        var_qcp1_db0 = (var_qcp1_db0 + ((var_cf1_db0 * var_vc1) + (var_cf1 * var_vc1_db0)));
        var_qcp1_db1 = (var_qcp1_db1 + ((var_cf1_db1 * var_vc1) + (var_cf1 * var_vc1_db1)));
        var_qcp1_rv = 0.0;
        var_qcp1_rdn0 = 0.0;
        var_qcp1_rdn1 = 0.0;
        var_qcp1_rdn2 = 0.0;
        var_qcp1_rdn3 = 0.0;
        var_qcp1_rdn4 = 0.0;
        var_qcp1_rdn5 = 0.0;
        var_qcp1_rdb0 = 0.0;
        var_qcp1_rdb1 = 0.0;

        let assign4710_e4921: f64 = (var_cf2 * var_vc2);
        let assign4710_e4922: f64 = (var_qcp2 + assign4710_e4921);
        var_qcp2 = assign4710_e4922;
        var_qcp2_dn0 = (var_qcp2_dn0 + ((var_cf2_dn0 * var_vc2) + (var_cf2 * var_vc2_dn0)));
        var_qcp2_dn1 = (var_qcp2_dn1 + ((var_cf2_dn1 * var_vc2) + (var_cf2 * var_vc2_dn1)));
        var_qcp2_dn2 = (var_qcp2_dn2 + ((var_cf2_dn2 * var_vc2) + (var_cf2 * var_vc2_dn2)));
        var_qcp2_dn3 = (var_qcp2_dn3 + ((var_cf2_dn3 * var_vc2) + (var_cf2 * var_vc2_dn3)));
        var_qcp2_dn4 = (var_qcp2_dn4 + ((var_cf2_dn4 * var_vc2) + (var_cf2 * var_vc2_dn4)));
        var_qcp2_dn5 = (var_qcp2_dn5 + ((var_cf2_dn5 * var_vc2) + (var_cf2 * var_vc2_dn5)));
        var_qcp2_db0 = (var_qcp2_db0 + ((var_cf2_db0 * var_vc2) + (var_cf2 * var_vc2_db0)));
        var_qcp2_db1 = (var_qcp2_db1 + ((var_cf2_db1 * var_vc2) + (var_cf2 * var_vc2_db1)));
        var_qcp2_rv = 0.0;
        var_qcp2_rdn0 = 0.0;
        var_qcp2_rdn1 = 0.0;
        var_qcp2_rdn2 = 0.0;
        var_qcp2_rdn3 = 0.0;
        var_qcp2_rdn4 = 0.0;
        var_qcp2_rdn5 = 0.0;
        var_qcp2_rdb0 = 0.0;
        var_qcp2_rdb1 = 0.0;

        let assign4720_e4924: f64 = (-p.p21);
        let assign4720_e4926: f64 = (assign4720_e4924 * var_qcp1);
        var_qcp1 = assign4720_e4926;
        var_qcp1_dn0 = (assign4720_e4924 * var_qcp1_dn0);
        var_qcp1_dn1 = (assign4720_e4924 * var_qcp1_dn1);
        var_qcp1_dn2 = (assign4720_e4924 * var_qcp1_dn2);
        var_qcp1_dn3 = (assign4720_e4924 * var_qcp1_dn3);
        var_qcp1_dn4 = (assign4720_e4924 * var_qcp1_dn4);
        var_qcp1_dn5 = (assign4720_e4924 * var_qcp1_dn5);
        var_qcp1_db0 = (assign4720_e4924 * var_qcp1_db0);
        var_qcp1_db1 = (assign4720_e4924 * var_qcp1_db1);
        var_qcp1_rv = 0.0;
        var_qcp1_rdn0 = 0.0;
        var_qcp1_rdn1 = 0.0;
        var_qcp1_rdn2 = 0.0;
        var_qcp1_rdn3 = 0.0;
        var_qcp1_rdn4 = 0.0;
        var_qcp1_rdn5 = 0.0;
        var_qcp1_rdb0 = 0.0;
        var_qcp1_rdb1 = 0.0;

        let assign4730_e4928: f64 = (-p.p21);
        let assign4730_e4930: f64 = (assign4730_e4928 * var_qcp2);
        var_qcp2 = assign4730_e4930;
        var_qcp2_dn0 = (assign4730_e4928 * var_qcp2_dn0);
        var_qcp2_dn1 = (assign4730_e4928 * var_qcp2_dn1);
        var_qcp2_dn2 = (assign4730_e4928 * var_qcp2_dn2);
        var_qcp2_dn3 = (assign4730_e4928 * var_qcp2_dn3);
        var_qcp2_dn4 = (assign4730_e4928 * var_qcp2_dn4);
        var_qcp2_dn5 = (assign4730_e4928 * var_qcp2_dn5);
        var_qcp2_db0 = (assign4730_e4928 * var_qcp2_db0);
        var_qcp2_db1 = (assign4730_e4928 * var_qcp2_db1);
        var_qcp2_rv = 0.0;
        var_qcp2_rdn0 = 0.0;
        var_qcp2_rdn1 = 0.0;
        var_qcp2_rdn2 = 0.0;
        var_qcp2_rdn3 = 0.0;
        var_qcp2_rdn4 = 0.0;
        var_qcp2_rdn5 = 0.0;
        var_qcp2_rdb0 = 0.0;
        var_qcp2_rdb1 = 0.0;

        let assign4740_e4933: f64 = (var_dt_et * var_cth);
        var_qcth = assign4740_e4933;
        var_qcth_dn0 = ((var_dt_et_dn0 * var_cth) + (var_dt_et * var_cth_dn0));
        var_qcth_dn1 = ((var_dt_et_dn1 * var_cth) + (var_dt_et * var_cth_dn1));
        var_qcth_dn2 = ((var_dt_et_dn2 * var_cth) + (var_dt_et * var_cth_dn2));
        var_qcth_dn3 = ((var_dt_et_dn3 * var_cth) + (var_dt_et * var_cth_dn3));
        var_qcth_dn4 = ((var_dt_et_dn4 * var_cth) + (var_dt_et * var_cth_dn4));
        var_qcth_dn5 = ((var_dt_et_dn5 * var_cth) + (var_dt_et * var_cth_dn5));
        var_qcth_db0 = ((var_dt_et_db0 * var_cth) + (var_dt_et * var_cth_db0));
        var_qcth_db1 = ((var_dt_et_db1 * var_cth) + (var_dt_et * var_cth_db1));
        var_qcth_rv = 0.0;
        var_qcth_rdn0 = 0.0;
        var_qcth_rdn1 = 0.0;
        var_qcth_rdn2 = 0.0;
        var_qcth_rdn3 = 0.0;
        var_qcth_rdn4 = 0.0;
        var_qcth_rdn5 = 0.0;
        var_qcth_rdb0 = 0.0;
        var_qcth_rdb1 = 0.0;

        let (assign4770_e4949, assign4770_e4949_d_n0, assign4770_e4949_d_n1, assign4770_e4949_d_n2, assign4770_e4949_d_n3, assign4770_e4949_d_n4, assign4770_e4949_d_n5, assign4770_e4949_d_b0, assign4770_e4949_d_b1,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (var_leff_um, var_leff_um_dn0, var_leff_um_dn1, var_leff_um_dn2, var_leff_um_dn3, var_leff_um_dn4, var_leff_um_dn5, var_leff_um_db0, var_leff_um_db1,)
    } else {
        (var_len, var_len_dn0, var_len_dn1, var_len_dn2, var_len_dn3, var_len_dn4, var_len_dn5, var_len_db0, var_len_db1,)
    }
};
        var_len = assign4770_e4949;
        var_len_dn0 = assign4770_e4949_d_n0;
        var_len_dn1 = assign4770_e4949_d_n1;
        var_len_dn2 = assign4770_e4949_d_n2;
        var_len_dn3 = assign4770_e4949_d_n3;
        var_len_dn4 = assign4770_e4949_d_n4;
        var_len_dn5 = assign4770_e4949_d_n5;
        var_len_db0 = assign4770_e4949_d_b0;
        var_len_db1 = assign4770_e4949_d_b1;
        var_len_rv = 0.0;
        var_len_rdn0 = 0.0;
        var_len_rdn1 = 0.0;
        var_len_rdn2 = 0.0;
        var_len_rdn3 = 0.0;
        var_len_rdn4 = 0.0;
        var_len_rdn5 = 0.0;
        var_len_rdb0 = 0.0;
        var_len_rdb1 = 0.0;

        let (assign4780_e4955, assign4780_e4955_d_n0, assign4780_e4955_d_n1, assign4780_e4955_d_n2, assign4780_e4955_d_n3, assign4780_e4955_d_n4, assign4780_e4955_d_n5, assign4780_e4955_d_b0, assign4780_e4955_d_b1,) = {
    if ((p.p13 != 0.0) && (p.p89 != 0.0)) {
        (var_weff_um, var_weff_um_dn0, var_weff_um_dn1, var_weff_um_dn2, var_weff_um_dn3, var_weff_um_dn4, var_weff_um_dn5, var_weff_um_db0, var_weff_um_db1,)
    } else {
        (var_wid, var_wid_dn0, var_wid_dn1, var_wid_dn2, var_wid_dn3, var_wid_dn4, var_wid_dn5, var_wid_db0, var_wid_db1,)
    }
};
        var_wid = assign4780_e4955;
        var_wid_dn0 = assign4780_e4955_d_n0;
        var_wid_dn1 = assign4780_e4955_d_n1;
        var_wid_dn2 = assign4780_e4955_d_n2;
        var_wid_dn3 = assign4780_e4955_d_n3;
        var_wid_dn4 = assign4780_e4955_d_n4;
        var_wid_dn5 = assign4780_e4955_d_n5;
        var_wid_db0 = assign4780_e4955_d_b0;
        var_wid_db1 = assign4780_e4955_d_b1;
        var_wid_rv = 0.0;
        var_wid_rdn0 = 0.0;
        var_wid_rdn1 = 0.0;
        var_wid_rdn2 = 0.0;
        var_wid_rdn3 = 0.0;
        var_wid_rdn4 = 0.0;
        var_wid_rdn5 = 0.0;
        var_wid_rdb0 = 0.0;
        var_wid_rdb1 = 0.0;

        let (assign4790_e4962, assign4790_e4962_d_n0, assign4790_e4962_d_n1, assign4790_e4962_d_n2, assign4790_e4962_d_n3, assign4790_e4962_d_n4, assign4790_e4962_d_n5, assign4790_e4962_d_b0, assign4790_e4962_d_b1,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (var_l_um, var_l_um_dn0, var_l_um_dn1, var_l_um_dn2, var_l_um_dn3, var_l_um_dn4, var_l_um_dn5, var_l_um_db0, var_l_um_db1,)
    } else {
        (var_len, var_len_dn0, var_len_dn1, var_len_dn2, var_len_dn3, var_len_dn4, var_len_dn5, var_len_db0, var_len_db1,)
    }
};
        var_len = assign4790_e4962;
        var_len_dn0 = assign4790_e4962_d_n0;
        var_len_dn1 = assign4790_e4962_d_n1;
        var_len_dn2 = assign4790_e4962_d_n2;
        var_len_dn3 = assign4790_e4962_d_n3;
        var_len_dn4 = assign4790_e4962_d_n4;
        var_len_dn5 = assign4790_e4962_d_n5;
        var_len_db0 = assign4790_e4962_d_b0;
        var_len_db1 = assign4790_e4962_d_b1;
        var_len_rv = 0.0;
        var_len_rdn0 = 0.0;
        var_len_rdn1 = 0.0;
        var_len_rdn2 = 0.0;
        var_len_rdn3 = 0.0;
        var_len_rdn4 = 0.0;
        var_len_rdn5 = 0.0;
        var_len_rdb0 = 0.0;
        var_len_rdb1 = 0.0;

        let (assign4800_e4969, assign4800_e4969_d_n0, assign4800_e4969_d_n1, assign4800_e4969_d_n2, assign4800_e4969_d_n3, assign4800_e4969_d_n4, assign4800_e4969_d_n5, assign4800_e4969_d_b0, assign4800_e4969_d_b1,) = {
    if ((p.p13 != 0.0) && (p.p89 == 0.0)) {
        (var_w_um, var_w_um_dn0, var_w_um_dn1, var_w_um_dn2, var_w_um_dn3, var_w_um_dn4, var_w_um_dn5, var_w_um_db0, var_w_um_db1,)
    } else {
        (var_wid, var_wid_dn0, var_wid_dn1, var_wid_dn2, var_wid_dn3, var_wid_dn4, var_wid_dn5, var_wid_db0, var_wid_db1,)
    }
};
        var_wid = assign4800_e4969;
        var_wid_dn0 = assign4800_e4969_d_n0;
        var_wid_dn1 = assign4800_e4969_d_n1;
        var_wid_dn2 = assign4800_e4969_d_n2;
        var_wid_dn3 = assign4800_e4969_d_n3;
        var_wid_dn4 = assign4800_e4969_d_n4;
        var_wid_dn5 = assign4800_e4969_d_n5;
        var_wid_db0 = assign4800_e4969_d_b0;
        var_wid_db1 = assign4800_e4969_d_b1;
        var_wid_rv = 0.0;
        var_wid_rdn0 = 0.0;
        var_wid_rdn1 = 0.0;
        var_wid_rdn2 = 0.0;
        var_wid_rdn3 = 0.0;
        var_wid_rdn4 = 0.0;
        var_wid_rdn5 = 0.0;
        var_wid_rdb0 = 0.0;
        var_wid_rdb1 = 0.0;

        var_cth = var_cth;
        var_cth_dn0 = var_cth_dn0;
        var_cth_dn1 = var_cth_dn1;
        var_cth_dn2 = var_cth_dn2;
        var_cth_dn3 = var_cth_dn3;
        var_cth_dn4 = var_cth_dn4;
        var_cth_dn5 = var_cth_dn5;
        var_cth_db0 = var_cth_db0;
        var_cth_db1 = var_cth_db1;
        var_cth_rv = 0.0;
        var_cth_rdn0 = 0.0;
        var_cth_rdn1 = 0.0;
        var_cth_rdn2 = 0.0;
        var_cth_rdn3 = 0.0;
        var_cth_rdn4 = 0.0;
        var_cth_rdn5 = 0.0;
        var_cth_rdb0 = 0.0;
        var_cth_rdb1 = 0.0;

        *var_cth_slot = var_cth;
        *var_cth_db0_slot = var_cth_db0;
        *var_cth_db1_slot = var_cth_db1;
        *var_cth_dn0_slot = var_cth_dn0;
        *var_cth_dn1_slot = var_cth_dn1;
        *var_cth_dn2_slot = var_cth_dn2;
        *var_cth_dn3_slot = var_cth_dn3;
        *var_cth_dn4_slot = var_cth_dn4;
        *var_cth_dn5_slot = var_cth_dn5;
        *var_cth_rdb0_slot = var_cth_rdb0;
        *var_cth_rdb1_slot = var_cth_rdb1;
        *var_cth_rdn0_slot = var_cth_rdn0;
        *var_cth_rdn1_slot = var_cth_rdn1;
        *var_cth_rdn2_slot = var_cth_rdn2;
        *var_cth_rdn3_slot = var_cth_rdn3;
        *var_cth_rdn4_slot = var_cth_rdn4;
        *var_cth_rdn5_slot = var_cth_rdn5;
        *var_cth_rv_slot = var_cth_rv;
        *var_len_slot = var_len;
        *var_len_db0_slot = var_len_db0;
        *var_len_db1_slot = var_len_db1;
        *var_len_dn0_slot = var_len_dn0;
        *var_len_dn1_slot = var_len_dn1;
        *var_len_dn2_slot = var_len_dn2;
        *var_len_dn3_slot = var_len_dn3;
        *var_len_dn4_slot = var_len_dn4;
        *var_len_dn5_slot = var_len_dn5;
        *var_len_rdb0_slot = var_len_rdb0;
        *var_len_rdb1_slot = var_len_rdb1;
        *var_len_rdn0_slot = var_len_rdn0;
        *var_len_rdn1_slot = var_len_rdn1;
        *var_len_rdn2_slot = var_len_rdn2;
        *var_len_rdn3_slot = var_len_rdn3;
        *var_len_rdn4_slot = var_len_rdn4;
        *var_len_rdn5_slot = var_len_rdn5;
        *var_len_rv_slot = var_len_rv;
        *var_qcp1_slot = var_qcp1;
        *var_qcp1_db0_slot = var_qcp1_db0;
        *var_qcp1_db1_slot = var_qcp1_db1;
        *var_qcp1_dn0_slot = var_qcp1_dn0;
        *var_qcp1_dn1_slot = var_qcp1_dn1;
        *var_qcp1_dn2_slot = var_qcp1_dn2;
        *var_qcp1_dn3_slot = var_qcp1_dn3;
        *var_qcp1_dn4_slot = var_qcp1_dn4;
        *var_qcp1_dn5_slot = var_qcp1_dn5;
        *var_qcp1_rdb0_slot = var_qcp1_rdb0;
        *var_qcp1_rdb1_slot = var_qcp1_rdb1;
        *var_qcp1_rdn0_slot = var_qcp1_rdn0;
        *var_qcp1_rdn1_slot = var_qcp1_rdn1;
        *var_qcp1_rdn2_slot = var_qcp1_rdn2;
        *var_qcp1_rdn3_slot = var_qcp1_rdn3;
        *var_qcp1_rdn4_slot = var_qcp1_rdn4;
        *var_qcp1_rdn5_slot = var_qcp1_rdn5;
        *var_qcp1_rv_slot = var_qcp1_rv;
        *var_qcp2_slot = var_qcp2;
        *var_qcp2_db0_slot = var_qcp2_db0;
        *var_qcp2_db1_slot = var_qcp2_db1;
        *var_qcp2_dn0_slot = var_qcp2_dn0;
        *var_qcp2_dn1_slot = var_qcp2_dn1;
        *var_qcp2_dn2_slot = var_qcp2_dn2;
        *var_qcp2_dn3_slot = var_qcp2_dn3;
        *var_qcp2_dn4_slot = var_qcp2_dn4;
        *var_qcp2_dn5_slot = var_qcp2_dn5;
        *var_qcp2_rdb0_slot = var_qcp2_rdb0;
        *var_qcp2_rdb1_slot = var_qcp2_rdb1;
        *var_qcp2_rdn0_slot = var_qcp2_rdn0;
        *var_qcp2_rdn1_slot = var_qcp2_rdn1;
        *var_qcp2_rdn2_slot = var_qcp2_rdn2;
        *var_qcp2_rdn3_slot = var_qcp2_rdn3;
        *var_qcp2_rdn4_slot = var_qcp2_rdn4;
        *var_qcp2_rdn5_slot = var_qcp2_rdn5;
        *var_qcp2_rv_slot = var_qcp2_rv;
        *var_qcth_slot = var_qcth;
        *var_qcth_db0_slot = var_qcth_db0;
        *var_qcth_db1_slot = var_qcth_db1;
        *var_qcth_dn0_slot = var_qcth_dn0;
        *var_qcth_dn1_slot = var_qcth_dn1;
        *var_qcth_dn2_slot = var_qcth_dn2;
        *var_qcth_dn3_slot = var_qcth_dn3;
        *var_qcth_dn4_slot = var_qcth_dn4;
        *var_qcth_dn5_slot = var_qcth_dn5;
        *var_qcth_rdb0_slot = var_qcth_rdb0;
        *var_qcth_rdb1_slot = var_qcth_rdb1;
        *var_qcth_rdn0_slot = var_qcth_rdn0;
        *var_qcth_rdn1_slot = var_qcth_rdn1;
        *var_qcth_rdn2_slot = var_qcth_rdn2;
        *var_qcth_rdn3_slot = var_qcth_rdn3;
        *var_qcth_rdn4_slot = var_qcth_rdn4;
        *var_qcth_rdn5_slot = var_qcth_rdn5;
        *var_qcth_rv_slot = var_qcth_rv;
        *var_wid_slot = var_wid;
        *var_wid_db0_slot = var_wid_db0;
        *var_wid_db1_slot = var_wid_db1;
        *var_wid_dn0_slot = var_wid_dn0;
        *var_wid_dn1_slot = var_wid_dn1;
        *var_wid_dn2_slot = var_wid_dn2;
        *var_wid_dn3_slot = var_wid_dn3;
        *var_wid_dn4_slot = var_wid_dn4;
        *var_wid_dn5_slot = var_wid_dn5;
        *var_wid_rdb0_slot = var_wid_rdb0;
        *var_wid_rdb1_slot = var_wid_rdb1;
        *var_wid_rdn0_slot = var_wid_rdn0;
        *var_wid_rdn1_slot = var_wid_rdn1;
        *var_wid_rdn2_slot = var_wid_rdn2;
        *var_wid_rdn3_slot = var_wid_rdn3;
        *var_wid_rdn4_slot = var_wid_rdn4;
        *var_wid_rdn5_slot = var_wid_rdn5;
        *var_wid_rv_slot = var_wid_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
        var_ip1: f64,
        var_ip1_db0: f64,
        var_ip1_db1: f64,
        var_ip1_dn0: f64,
        var_ip1_dn1: f64,
        var_ip1_dn2: f64,
        var_ip1_dn3: f64,
        var_ip1_dn4: f64,
        var_ip1_dn5: f64,
        var_ip2: f64,
        var_ip2_db0: f64,
        var_ip2_db1: f64,
        var_ip2_dn0: f64,
        var_ip2_dn1: f64,
        var_ip2_dn2: f64,
        var_ip2_dn3: f64,
        var_ip2_dn4: f64,
        var_ip2_dn5: f64,
        var_irb: f64,
        var_irb_db0: f64,
        var_irb_db1: f64,
        var_irb_dn0: f64,
        var_irb_dn1: f64,
        var_irb_dn2: f64,
        var_irb_dn3: f64,
        var_irb_dn4: f64,
        var_irb_dn5: f64,
        var_irth: f64,
        var_irth_db0: f64,
        var_irth_db1: f64,
        var_irth_dn0: f64,
        var_irth_dn1: f64,
        var_irth_dn2: f64,
        var_irth_dn3: f64,
        var_irth_dn4: f64,
        var_irth_dn5: f64,
        var_ith: f64,
        var_ith_db0: f64,
        var_ith_db1: f64,
        var_ith_dn0: f64,
        var_ith_dn1: f64,
        var_ith_dn2: f64,
        var_ith_dn3: f64,
        var_ith_dn4: f64,
        var_ith_dn5: f64,
    ) {
        let eq0_value: f64 = var_irb;
        let eq0_node_derivatives: [f64; 6] = [var_irb_dn0, var_irb_dn1, var_irb_dn2, var_irb_dn3, var_irb_dn4, var_irb_dn5];
        let eq0_branch_derivatives: [f64; 2] = [var_irb_db0, var_irb_db1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_value: f64 = var_ip1;
        let eq1_node_derivatives: [f64; 6] = [var_ip1_dn0, var_ip1_dn1, var_ip1_dn2, var_ip1_dn3, var_ip1_dn4, var_ip1_dn5];
        let eq1_branch_derivatives: [f64; 2] = [var_ip1_db0, var_ip1_db1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_value: f64 = var_ip2;
        let eq2_node_derivatives: [f64; 6] = [var_ip2_dn0, var_ip2_dn1, var_ip2_dn2, var_ip2_dn3, var_ip2_dn4, var_ip2_dn5];
        let eq2_branch_derivatives: [f64; 2] = [var_ip2_db0, var_ip2_db1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_value: f64 = var_irth;
        let eq3_node_derivatives: [f64; 6] = [var_irth_dn0, var_irth_dn1, var_irth_dn2, var_irth_dn3, var_irth_dn4, var_irth_dn5];
        let eq3_branch_derivatives: [f64; 2] = [var_irth_db0, var_irth_db1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let eq4_value: f64 = var_ith;
        let eq4_node_derivatives: [f64; 6] = [var_ith_dn0, var_ith_dn1, var_ith_dn2, var_ith_dn3, var_ith_dn4, var_ith_dn5];
        let eq4_branch_derivatives: [f64; 2] = [var_ith_db0, var_ith_db1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_guard311: f64,
        var_rc1_tnom: f64,
        var_tcrc: f64,
        var_tcrc_db0: f64,
        var_tcrc_db1: f64,
        var_tcrc_dn0: f64,
        var_tcrc_dn1: f64,
        var_tcrc_dn2: f64,
        var_tcrc_dn3: f64,
        var_tcrc_dn4: f64,
        var_tcrc_dn5: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let (eq6_e162, eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5, eq6_e162_d_b0, eq6_e162_d_b1,) = {
    if (var_guard311 == 0.0) {
        let eq6_e159: f64 = (var_rc1_tnom * var_tcrc);
        let eq6_e159_d_n0: f64 = (var_rc1_tnom * var_tcrc_dn0);
        let eq6_e159_d_n1: f64 = (var_rc1_tnom * var_tcrc_dn1);
        let eq6_e159_d_n2: f64 = (var_rc1_tnom * var_tcrc_dn2);
        let eq6_e159_d_n3: f64 = (var_rc1_tnom * var_tcrc_dn3);
        let eq6_e159_d_n4: f64 = (var_rc1_tnom * var_tcrc_dn4);
        let eq6_e159_d_n5: f64 = (var_rc1_tnom * var_tcrc_dn5);
        let eq6_e159_d_b0: f64 = (var_rc1_tnom * var_tcrc_db0);
        let eq6_e159_d_b1: f64 = (var_rc1_tnom * var_tcrc_db1);
        let eq6_e160: f64 = ((nv0 - nv4) / eq6_e159);
        let eq6_e160_d_n0: f64 = ((eq6_e159 - ((nv0 - nv4) * eq6_e159_d_n0)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n1: f64 = (-(((nv0 - nv4) * eq6_e159_d_n1) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n2: f64 = (-(((nv0 - nv4) * eq6_e159_d_n2) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n3: f64 = (-(((nv0 - nv4) * eq6_e159_d_n3) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_n4: f64 = (((-eq6_e159) - ((nv0 - nv4) * eq6_e159_d_n4)) / (eq6_e159 * eq6_e159));
        let eq6_e160_d_n5: f64 = (-(((nv0 - nv4) * eq6_e159_d_n5) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b0: f64 = (-(((nv0 - nv4) * eq6_e159_d_b0) / (eq6_e159 * eq6_e159)));
        let eq6_e160_d_b1: f64 = (-(((nv0 - nv4) * eq6_e159_d_b1) / (eq6_e159 * eq6_e159)));
        (eq6_e160, eq6_e160_d_n0, eq6_e160_d_n1, eq6_e160_d_n2, eq6_e160_d_n3, eq6_e160_d_n4, eq6_e160_d_n5, eq6_e160_d_b0, eq6_e160_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e162;
        let eq6_node_derivatives: [f64; 6] = [eq6_e162_d_n0, eq6_e162_d_n1, eq6_e162_d_n2, eq6_e162_d_n3, eq6_e162_d_n4, eq6_e162_d_n5];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e162_d_b0, eq6_e162_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(4),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
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
        var_guard312: f64,
        var_qcp1: f64,
        var_qcp1_db0: f64,
        var_qcp1_db1: f64,
        var_qcp1_dn0: f64,
        var_qcp1_dn1: f64,
        var_qcp1_dn2: f64,
        var_qcp1_dn3: f64,
        var_qcp1_dn4: f64,
        var_qcp1_dn5: f64,
        var_qcp2: f64,
        var_qcp2_db0: f64,
        var_qcp2_db1: f64,
        var_qcp2_dn0: f64,
        var_qcp2_dn1: f64,
        var_qcp2_dn2: f64,
        var_qcp2_dn3: f64,
        var_qcp2_dn4: f64,
        var_qcp2_dn5: f64,
        var_qcth: f64,
        var_qcth_db0: f64,
        var_qcth_db1: f64,
        var_qcth_dn0: f64,
        var_qcth_dn1: f64,
        var_qcth_dn2: f64,
        var_qcth_dn3: f64,
        var_qcth_dn4: f64,
        var_qcth_dn5: f64,
        var_rc2_tnom: f64,
        var_tcrc: f64,
        var_tcrc_db0: f64,
        var_tcrc_db1: f64,
        var_tcrc_dn0: f64,
        var_tcrc_dn1: f64,
        var_tcrc_dn2: f64,
        var_tcrc_dn3: f64,
        var_tcrc_dn4: f64,
        var_tcrc_dn5: f64,
    ) {
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let (eq8_e179, eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5, eq8_e179_d_b0, eq8_e179_d_b1,) = {
    if (var_guard312 == 0.0) {
        let eq8_e176: f64 = (var_rc2_tnom * var_tcrc);
        let eq8_e176_d_n0: f64 = (var_rc2_tnom * var_tcrc_dn0);
        let eq8_e176_d_n1: f64 = (var_rc2_tnom * var_tcrc_dn1);
        let eq8_e176_d_n2: f64 = (var_rc2_tnom * var_tcrc_dn2);
        let eq8_e176_d_n3: f64 = (var_rc2_tnom * var_tcrc_dn3);
        let eq8_e176_d_n4: f64 = (var_rc2_tnom * var_tcrc_dn4);
        let eq8_e176_d_n5: f64 = (var_rc2_tnom * var_tcrc_dn5);
        let eq8_e176_d_b0: f64 = (var_rc2_tnom * var_tcrc_db0);
        let eq8_e176_d_b1: f64 = (var_rc2_tnom * var_tcrc_db1);
        let eq8_e177: f64 = ((nv2 - nv5) / eq8_e176);
        let eq8_e177_d_n0: f64 = (-(((nv2 - nv5) * eq8_e176_d_n0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n1: f64 = (-(((nv2 - nv5) * eq8_e176_d_n1) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n2: f64 = ((eq8_e176 - ((nv2 - nv5) * eq8_e176_d_n2)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_n3: f64 = (-(((nv2 - nv5) * eq8_e176_d_n3) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n4: f64 = (-(((nv2 - nv5) * eq8_e176_d_n4) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_n5: f64 = (((-eq8_e176) - ((nv2 - nv5) * eq8_e176_d_n5)) / (eq8_e176 * eq8_e176));
        let eq8_e177_d_b0: f64 = (-(((nv2 - nv5) * eq8_e176_d_b0) / (eq8_e176 * eq8_e176)));
        let eq8_e177_d_b1: f64 = (-(((nv2 - nv5) * eq8_e176_d_b1) / (eq8_e176 * eq8_e176)));
        (eq8_e177, eq8_e177_d_n0, eq8_e177_d_n1, eq8_e177_d_n2, eq8_e177_d_n3, eq8_e177_d_n4, eq8_e177_d_n5, eq8_e177_d_b0, eq8_e177_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e179;
        let eq8_node_derivatives: [f64; 6] = [eq8_e179_d_n0, eq8_e179_d_n1, eq8_e179_d_n2, eq8_e179_d_n3, eq8_e179_d_n4, eq8_e179_d_n5];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e179_d_b0, eq8_e179_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(5),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e181: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, var_qcp1);
        let eq9_value: f64 = eq9_e181;
        let eq9_node_derivatives: [f64; 6] = [(var_qcp1_dn0 * ddt_scale), (var_qcp1_dn1 * ddt_scale), (var_qcp1_dn2 * ddt_scale), (var_qcp1_dn3 * ddt_scale), (var_qcp1_dn4 * ddt_scale), (var_qcp1_dn5 * ddt_scale)];
        let eq9_branch_derivatives: [f64; 2] = [(var_qcp1_db0 * ddt_scale), (var_qcp1_db1 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(4),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e183: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, var_qcp2);
        let eq10_value: f64 = eq10_e183;
        let eq10_node_derivatives: [f64; 6] = [(var_qcp2_dn0 * ddt_scale), (var_qcp2_dn1 * ddt_scale), (var_qcp2_dn2 * ddt_scale), (var_qcp2_dn3 * ddt_scale), (var_qcp2_dn4 * ddt_scale), (var_qcp2_dn5 * ddt_scale)];
        let eq10_branch_derivatives: [f64; 2] = [(var_qcp2_db0 * ddt_scale), (var_qcp2_db1 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e185: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, var_qcth);
        let eq11_value: f64 = eq11_e185;
        let eq11_node_derivatives: [f64; 6] = [(var_qcth_dn0 * ddt_scale), (var_qcth_dn1 * ddt_scale), (var_qcth_dn2 * ddt_scale), (var_qcth_dn3 * ddt_scale), (var_qcth_dn4 * ddt_scale), (var_qcth_dn5 * ddt_scale)];
        let eq11_branch_derivatives: [f64; 2] = [(var_qcth_db0 * ddt_scale), (var_qcth_db1 * ddt_scale)];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_qcp1: f64,
        var_qcp1_db0: f64,
        var_qcp1_db1: f64,
        var_qcp1_dn0: f64,
        var_qcp1_dn1: f64,
        var_qcp1_dn2: f64,
        var_qcp1_dn3: f64,
        var_qcp1_dn4: f64,
        var_qcp1_dn5: f64,
        var_qcp2: f64,
        var_qcp2_db0: f64,
        var_qcp2_db1: f64,
        var_qcp2_dn0: f64,
        var_qcp2_dn1: f64,
        var_qcp2_dn2: f64,
        var_qcp2_dn3: f64,
        var_qcp2_dn4: f64,
        var_qcp2_dn5: f64,
        var_qcth: f64,
        var_qcth_db0: f64,
        var_qcth_db1: f64,
        var_qcth_dn0: f64,
        var_qcth_dn1: f64,
        var_qcth_dn2: f64,
        var_qcth_dn3: f64,
        var_qcth_dn4: f64,
        var_qcth_dn5: f64,
    ) {
        let eq9_e181_q: f64 = var_qcp1;
        let eq9_reactive_node_derivatives: [f64; 6] = [var_qcp1_dn0, var_qcp1_dn1, var_qcp1_dn2, var_qcp1_dn3, var_qcp1_dn4, var_qcp1_dn5];
        let eq9_reactive_branch_derivatives: [f64; 2] = [var_qcp1_db0, var_qcp1_db1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[4]),
            nodes,
            &eq9_reactive_node_derivatives,
            branches,
            &eq9_reactive_branch_derivatives,
            multiplicity,
        );
        let eq10_e183_q: f64 = var_qcp2;
        let eq10_reactive_node_derivatives: [f64; 6] = [var_qcp2_dn0, var_qcp2_dn1, var_qcp2_dn2, var_qcp2_dn3, var_qcp2_dn4, var_qcp2_dn5];
        let eq10_reactive_branch_derivatives: [f64; 2] = [var_qcp2_db0, var_qcp2_db1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[5]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e185_q: f64 = var_qcth;
        let eq11_reactive_node_derivatives: [f64; 6] = [var_qcth_dn0, var_qcth_dn1, var_qcth_dn2, var_qcth_dn3, var_qcth_dn4, var_qcth_dn5];
        let eq11_reactive_branch_derivatives: [f64; 2] = [var_qcth_db0, var_qcth_db1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
